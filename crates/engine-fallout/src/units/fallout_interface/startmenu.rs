//! `fallout/interface/menus/startmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The start menu (`StartMenu`, class 0x3F5) and the `ListBox` instances it
//! uses (the engine map lists them under this unit as
//! `ListBox<StartMenu::Option_P>`). Part files hold the functions from
//! `007d3aa0` on; this file holds the ones before it.
//!
//! x87 note: the game computes floats in extended precision and stores
//! `float` results. The translations compute in `f64` and round to `f32` at
//! each `float` store. Comparisons keep the exe's flag tests, NaN included
//! (`FCOMP; FNSTSW; TEST AH,mask; JP/JNZ`): the helpers below spell the
//! tests the exe uses.
//!
//! Functions whose disassembly has a C++ exception frame (`FS:[0]`) or the
//! `/GS` cookie check carry an incomplete-translation marker: the unwinding
//! frames and the cookie are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `Tile::GetValue(id)` (unnamed in the engine map, Ghidra labels it with the
/// `Tile` methods around it): the value of trait `id`, `float` in ST0.
/// `RET 4`.
const TILE_GET_VALUE: u32 = 0x00a0_11b0;
/// `Tile::SetValue(id, float, flag)` (`RET 0xC`).
const TILE_SET_VALUE: u32 = 0x00a0_12d0;
/// `Tile::SetString(id, text, flag)` (`RET 0xC`; has an exception frame).
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::GetString(id)` (Xbox PDB name per Ghidra), `RET 4`.
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// `Tile::IsTrue(id)` (Ghidra name), `RET 4`.
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::Poke(id, value)` (Ghidra name), `RET 8`.
const TILE_POKE: u32 = 0x00a0_1290;
/// `Tile::AddUserTrait(name, -1)` (Ghidra name): cdecl, returns the trait id.
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::TextToTrait(name)` (Ghidra name): cdecl, returns the trait id.
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::GetMenu` (Ghidra name): the menu a tile belongs to.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMenuByClass(class)` (Ghidra name).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::GetMaximumDepth` (Ghidra name), `float` in ST0.
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Tile::Lock` and `Tile::Unlock` (Ghidra names).
const TILE_LOCK: u32 = 0x00a0_44f0;
const TILE_UNLOCK: u32 = 0x00a0_4500;
/// `Tile::ReadFile(path)` (Ghidra name).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// Sets a tile's trait from an integer, `SetValue(id, (float)value, 1)`; the
/// tile is `this`, `RET 8`.
const TILE_SET_INT: u32 = 0x0070_0320;
/// `BSSimpleList` node accessors: the address of a node's item word
/// (`006815c0`, no arguments) and the next node (`00726070`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `BSSimpleList::IsEmpty` (`008256d0`), the list is `this`.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `NiTPointerList` iterator step: `(list, &position)` returns the address
/// of the current element and advances `position` (`00683520`, `RET 4`).
const POINTER_LIST_STEP: u32 = 0x0068_3520;
/// A list iterator step for another list shape: `(list, &cursor)` returns the
/// address of the current item and advances the cursor (`0057cbe0`, `RET 4`).
const POINTER_LIST_STEP_GLOBAL: u32 = 0x0057_cbe0;
/// `FontManager` instance getter (`005bd5b0`) and
/// `FontManager::CalculateStringDimensions` (Ghidra name), `RET 0x14`.
const FONT_MANAGER_GET: u32 = 0x005b_d5b0;
const CALCULATE_STRING_DIMENSIONS: u32 = 0x00a1_b020;
/// `_ftol2_sse` (`00ec62c0`): the `f64` in the first two words, truncated.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// Debug log line (`005b5e40`, one string argument).
const LOG_MESSAGE: u32 = 0x005b_5e40;

/// `Tile::enumTrait` ids (Xbox PDB) this file uses.
const TRAIT_Y: u32 = 4002;
const TRAIT_VISIBLE: u32 = 4003;
const TRAIT_STACKING_TYPE: u32 = 4007;
const TRAIT_ALPHA: u32 = 4009;
const TRAIT_DEPTH: u32 = 4013;
const TRAIT_LIST_INDEX: u32 = 4012;
const TRAIT_TARGET: u32 = 4015;
const TRAIT_HEIGHT: u32 = 4016;
const TRAIT_WIDTH: u32 = 4017;
const TRAIT_FONT: u32 = 4025;
const TRAIT_WRAP_WIDTH: u32 = 4026;
const TRAIT_STRING: u32 = 4036;
const TRAIT_BRIGHTNESS: u32 = 4085;
const TRAIT_USER_0: u32 = 4100;
const TRAIT_USER_1: u32 = 4101;
const TRAIT_USER_3: u32 = 4103;
const TRAIT_DELETE_ON_FADE: u32 = 6002;

/// Strings of the exe's data (addresses only; the text is read from the
/// executable at run time): `"_y"`, `"_VerticalSpacing"`,
/// `"_current_value"`, `"_SetInCode"` and the scrollbar warning
/// (`"MENUS: The scrollbar update in ScrollToHighlight() has hit an infinite
/// loop ..."`).
const TEXT_USER_TRAIT_Y: u32 = 0x0106_f8f4;
const TEXT_VERTICAL_SPACING: u32 = 0x0106_f8e0;
const TEXT_CURRENT_VALUE: u32 = 0x0107_02d8;
const TEXT_SET_IN_CODE: u32 = 0x0107_0250;
const TEXT_SCROLLBAR_LOOP: u32 = 0x0107_0260;
/// The largest `float` (`3.4028235e38`) in the exe's data.
const FLOAT_MAX: u32 = 0x0101_6970;

layout! {
    /// `ListBox<T>` (Xbox PDB `ListBox<StartMenu::Option *>`), 0x30 bytes: the
    /// vtable, an embedded `BSSimpleList` (first node at +4: item, next at
    /// +8) and the list's tiles and counters. The exe's instances for
    /// `StartMenu::Option`, `int` and `BGSSaveLoadFileEntry` share it.
    pub struct ListBox: 0x30 {
        /// `pContainerRect` (Xbox PDB): the tile the items are laid out in.
        0x0C pContainerRect: Ptr,
        /// `pHighlightedItem` (Xbox PDB).
        0x10 pHighlightedItem: Ptr,
        /// `pScrollBar` (Xbox PDB).
        0x14 pScrollBar: Ptr,
        /// `pTemplateName` (Xbox PDB): `const char *`.
        0x18 pTemplateName: Ptr,
        /// `usNextIndex` (Xbox PDB).
        0x1C usNextIndex: u16,
        /// `fTotalListHeight` (Xbox PDB).
        0x20 fTotalListHeight: f32,
        /// `fSavedHighlightIndex` (Xbox PDB).
        0x24 fSavedHighlightIndex: f32,
        /// `fSavedScrollbarValue` (Xbox PDB).
        0x28 fSavedScrollbarValue: f32,
        /// `usFlags` (Xbox PDB).
        0x2C usFlags: u16,
    }
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: taken when the compared value is
/// not strictly above the other (less, equal or unordered).
fn not_above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) != Some(std::cmp::Ordering::Greater)
}

/// `FCOMP; FNSTSW AX; TEST AH,1; JNZ`: taken when the first is less than the
/// second or they are unordered (NaN).
fn less_or_unordered(a: f64, b: f64) -> bool {
    matches!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less) | None)
}

fn tile_value(e: &mut Engine, tile: Ptr, id: u32) -> f32 {
    e.call(TILE_GET_VALUE, &args![tile, id]).f32()
}

fn set_tile_value(e: &mut Engine, tile: Ptr, id: u32, value: f32, flag: u32) {
    e.call(TILE_SET_VALUE, &args![tile, id, value, flag]);
}

fn text_to_trait(e: &mut Engine, name: u32) -> u32 {
    e.call(TILE_TEXT_TO_TRAIT, &args![name]).u32()
}

fn node_item(e: &mut Engine, node: u32) -> Ptr {
    e.call(NODE_ITEM, &args![node]).ptr()
}

fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

// Translated from 00723030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option *>::BelongsTo` (Xbox PDB): whether the list's
/// container tile (+0xC) belongs to `menu` (`Tile::GetMenu` returns it).
pub fn list_box_belongs_to(e: &mut Engine, this: Ptr<ListBox>, menu: u32) -> bool {
    let container = e.get(this, ListBox::pContainerRect);
    !container.is_null() && e.call(TILE_GET_MENU, &args![container]).u32() == menu
}

// Translated from 007269d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option *>::InitTilePosition` (Xbox PDB): gives the
/// item tile a `_y` user trait equal to the list's total height so far,
/// works out the tile's height (its own height trait; when that is zero,
/// the height of the first child of class 0x387 text-wrapped to that child's
/// wrap width, plus the tile's `_VerticalSpacing`) and adds it to
/// `fTotalListHeight`.
pub fn list_box_init_tile_position(e: &mut Engine, this: Ptr<ListBox>, tile: Ptr) {
    let total = e.get(this, ListBox::fTotalListHeight);
    let user_trait = e
        .call(
            TILE_ADD_USER_TRAIT,
            &args![TEXT_USER_TRAIT_Y, 0xffff_ffffu32],
        )
        .u32();
    e.call(TILE_SET_VALUE, &args![tile, user_trait, total, 1u32]);
    let mut height = tile_value(e, tile, TRAIT_HEIGHT);
    if height == 0.0 {
        let children = tile.byte_add(4);
        let first = e.call(NODE_NEXT, &args![children]).u32();
        let found = e.with_stack(4, |e, position| {
            e.mem.set_u32(position.addr(), first);
            let mut found = Ptr::NULL;
            while e.mem.u32(position.addr()) != 0 && found.is_null() {
                let slot = e.call(POINTER_LIST_STEP, &args![children, position]).u32();
                let child = e.mem.u32(slot);
                // Slot +0xC of the child's vtable returns the tile class.
                if e.vcall(child, 0xc, &args![]).u32() == 0x387 {
                    found = Ptr::new(child);
                }
            }
            found
        });
        if !found.is_null() {
            let wrap = tile_value(e, found, TRAIT_WRAP_WIDTH);
            let limit = if not_above(f64::from(wrap), 0.0) {
                e.global::<f32>(FLOAT_MAX)
            } else {
                wrap
            };
            let font = tile_value(e, found, TRAIT_FONT);
            let font = e.call(FLOAT_TO_INT, &args![f64::from(font)]).u32();
            let text = e.call(TILE_GET_STRING, &args![found, TRAIT_STRING]).u32();
            let font_manager = e.call(FONT_MANAGER_GET, &args![]).u32();
            let text_height = e.with_stack(12, |e, out| {
                let size = e
                    .call(
                        CALCULATE_STRING_DIMENSIONS,
                        &args![font_manager, out, text, font, limit, 0u32],
                    )
                    .ptr::<()>();
                e.mem.f32(size.addr() + 4)
            });
            let spacing_trait = text_to_trait(e, TEXT_VERTICAL_SPACING);
            let spacing = tile_value(e, tile, spacing_trait);
            height = (f64::from(spacing) + f64::from(text_height)) as f32;
        }
    }
    set_tile_value(e, tile, TRAIT_HEIGHT, height, 0);
    let total = e.get(this, ListBox::fTotalListHeight);
    let total = (f64::from(total) + f64::from(height)) as f32;
    e.set(this, ListBox::fTotalListHeight, total);
}

// Translated from 007a22d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option *>::GetTileFor` (Xbox PDB): the tile of the
/// list item whose `Option` (the item's word at +4) is the one `key` points
/// to, 0 when none.
pub fn list_box_get_tile_for(e: &mut Engine, this: Ptr<ListBox>, key: Ptr) -> Ptr {
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 {
        let slot = node_item(e, node);
        let item = e.mem.u32(slot.addr());
        if item == 0 {
            break;
        }
        if e.mem.u32(item + 4) == e.mem.u32(key.addr()) {
            let slot = node_item(e, node);
            let item = e.mem.u32(slot.addr());
            return Ptr::new(e.mem.u32(item));
        }
        node = node_next(e, node);
    }
    Ptr::NULL
}

// Translated from 007a30a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option *>::GetTileAt` (Xbox PDB): the tile (first
/// word) of the item `GetAt` finds, 0 when there is none.
pub fn list_box_get_tile_at(
    e: &mut Engine,
    this: Ptr<ListBox>,
    index: u32,
    by_position: u8,
) -> Ptr {
    let item = list_box_get_at(e, this, index, by_position);
    if item.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(e.mem.u32(item.addr()))
    }
}

// Translated from 007a5e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option *>::ScrollToHighlight` (Xbox PDB): when the
/// highlighted item lies above (`_y` < 0) or below (container height <
/// item height + `_y`) the visible area, steps the scrollbar's
/// `_current_value` by one in that direction until the item is inside or the
/// value stops changing (then logs a warning), and pokes `_SetInCode` on the
/// scrollbar if it had to scroll at all.
pub fn list_box_scroll_to_highlight(e: &mut Engine, this: Ptr<ListBox>) {
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    let container = e.get(this, ListBox::pContainerRect);
    if highlighted.is_null() || scroll_bar.is_null() || container.is_null() {
        return;
    }
    let container_height = tile_value(e, container, TRAIT_HEIGHT);
    if not_above(f64::from(container_height), 0.0) {
        return;
    }
    let (mut above, mut below) = highlight_outside(e, container, highlighted);
    let outside = above || below;
    loop {
        // The loop ends on a negative (or NaN) list index of the item.
        let index = tile_value(e, highlighted, TRAIT_LIST_INDEX);
        if less_or_unordered(f64::from(index), 0.0) || !(above || below) {
            break;
        }
        let step: i32 = if above { -1 } else { 1 };
        let value_trait = text_to_trait(e, TEXT_CURRENT_VALUE);
        let value = tile_value(e, scroll_bar, value_trait);
        let moved = (f64::from(step) + f64::from(value)) as f32;
        let value_trait = text_to_trait(e, TEXT_CURRENT_VALUE);
        set_tile_value(e, scroll_bar, value_trait, moved, 0);
        let value_trait = text_to_trait(e, TEXT_CURRENT_VALUE);
        let after = tile_value(e, scroll_bar, value_trait);
        if value == after {
            e.call(LOG_MESSAGE, &args![TEXT_SCROLLBAR_LOOP]);
            break;
        }
        (above, below) = highlight_outside(e, container, highlighted);
    }
    if outside {
        let set_in_code = text_to_trait(e, TEXT_SET_IN_CODE);
        e.call(TILE_POKE, &args![scroll_bar, set_in_code, 1.0f32]);
    }
}

/// Whether the highlighted item lies above the visible area (`_y` < 0) and
/// below it (container height < item height + `_y`), as `ScrollToHighlight`
/// tests it (the sum is stored as a `double`).
fn highlight_outside(e: &mut Engine, container: Ptr, highlighted: Ptr) -> (bool, bool) {
    let y = tile_value(e, highlighted, TRAIT_Y);
    let above = f64::from(y) < 0.0;
    let y = tile_value(e, highlighted, TRAIT_Y);
    let height = tile_value(e, highlighted, TRAIT_HEIGHT);
    let bottom = f64::from(height) + f64::from(y);
    let container_height = tile_value(e, container, TRAIT_HEIGHT);
    (above, f64::from(container_height) < bottom)
}

// Translated from 007a6fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry *>::GetAt` (Xbox PDB; the same code serves
/// every instance): the item (the node's item word) at
/// position `index` when `by_position` is set, else the first item whose
/// tile's `eListIndex` trait equals `index`; 0 when the list is empty or no
/// item matches.
pub fn list_box_get_at(e: &mut Engine, this: Ptr<ListBox>, index: u32, by_position: u8) -> Ptr {
    if e.call(LIST_IS_EMPTY, &args![this.byte_add(4)]).u8() != 0 {
        return Ptr::NULL;
    }
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    if by_position != 0 {
        let mut count = 0u32;
        while node != 0 {
            let at = count;
            count = count.wrapping_add(1);
            if at == index {
                break;
            }
            node = node_next(e, node);
        }
    } else {
        while node != 0 {
            let slot = node_item(e, node);
            let item = e.mem.u32(slot.addr());
            let tile = e.mem.u32(item);
            let value = tile_value(e, Ptr::new(tile), TRAIT_LIST_INDEX);
            if f64::from(index) == f64::from(value) {
                break;
            }
            node = node_next(e, node);
        }
    }
    if node == 0 {
        Ptr::NULL
    } else {
        let slot = node_item(e, node);
        Ptr::new(e.mem.u32(slot.addr()))
    }
}

/// The start menu object (`StartMenu::pMe`, Xbox PDB), the global at
/// `011daac0`; 0 while the menu does not exist.
const START_MENU: u32 = 0x011d_aac0;
/// A lazily created 8-byte object (`007cb710`), the global at `011daac4`.
const LAZY_OBJECT: u32 = 0x011d_aac4;
/// Tick counts taken when the menu is created (`011daab4`, `011daab8`).
const TICK_CREATED: u32 = 0x011d_aab4;
const TICK_CREATED_NEW_GAME: u32 = 0x011d_aab8;
/// The loading-menu byte tested before `SetupForMainMenu` (`011a0294`).
const LOADING_BYTE: u32 = 0x011a_0294;
/// Object that gets the menu's callbacks (`011de134`) and the object whose
/// byte at `+8` says that no menu elements exist yet (`011dab88`).
const CALLBACK_OWNER: u32 = 0x011d_e134;
const ELEMENTS_OBJECT: u32 = 0x011d_ab88;
/// A `BSSimpleList` the destructor empties (`011dabd8`).
const SCRIPT_LIST: u32 = 0x011d_abd8;
/// The object `450410` reads a `float` from when the menu is destroyed
/// with flag `0x40` set (`011d9774`).
const VOLUME_OBJECT: u32 = 0x011d_9774;
/// The class number of the start menu.
const START_MENU_CLASS: u32 = 0x3f5;

/// `StartMenu::uiFlags` test (`004a4080`, `RET 4`): bits of the word at
/// `+0x1a8` of the object in `this`.
const MENU_FLAGS_TEST: u32 = 0x004a_4080;
/// Sets or clears bits of that word (`0075f6f0(bits, set)`, `RET 8`).
const MENU_FLAGS_SET: u32 = 0x0075_f6f0;
/// The tick count (`00457fe0`, `GetTickCount`).
const TICK_COUNT: u32 = 0x0045_7fe0;
/// `operator new(size)` (`00401000`) and `operator delete` (`00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The interface manager object getter (`004b7210`, the word at
/// `011d8a80`, per `interface.rs`) and its `pMenusRoot` getter (`00586150`:
/// the word at `+0x9c`).
const INTERFACE_MANAGER_GETTER: u32 = 0x004b_7210;
const MENUS_ROOT_GETTER: u32 = 0x0058_6150;
/// The first word of an object (`00559450`): a list head.
const FIRST_WORD: u32 = 0x0055_9450;
/// `Menu::xFadeState` getter (`0059bb30`, the word at `+0x24`; the engine
/// map calls it `D3DTexture_LockRect` because the linker folded the code).
const MENU_FADE_STATE: u32 = 0x0059_bb30;
/// `Menu::SetMenuTile(tile, 0)` (Ghidra name), `RET 8`.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// The menu class getter returning `0x3f5` (`0070ede0`).
const START_MENU_CLASS_GETTER: u32 = 0x0070_ede0;
/// `XUserInterface::XUIIsUp` (Ghidra name).
const XUI_IS_UP: u32 = 0x0070_edf0;
/// A scope guard (`00404eb0` constructs it from `(id, flag, file, line)`,
/// `00404ee0` destroys it); 4 bytes on the stack.
const SCOPE_GUARD_NEW: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTROY: u32 = 0x0040_4ee0;
/// The source file name and line the guard is given (the path of `StartMenu.cpp` in the Xbox build tree).
const SOURCE_FILE_NAME: u32 = 0x0107_6db0;
/// `LoadingMenu::SetupForMainMenu(bool)` (Ghidra name), cdecl.
const SETUP_FOR_MAIN_MENU: u32 = 0x0078_d280;
/// Setters of the callback owner (`0061cc40` and `0086cf00`, one argument
/// each): the function `007d69d0` is passed to both.
const OWNER_SET_CALLBACK_A: u32 = 0x0061_cc40;
const OWNER_SET_CALLBACK_B: u32 = 0x0086_cf00;
const MENU_CALLBACK: u32 = 0x007d_69d0;
/// `0076b610`: true when the word at `+8` of the object is 0.
const ELEMENTS_MISSING: u32 = 0x0076_b610;
/// The path `"Data\Menus\Options\start_menu.xml"` and its source line.
const START_MENU_XML: u32 = 0x0107_6d8c;
/// `BSAudio::QInstance` (Ghidra name) and the audio call made on it by the
/// destructor (`0070bc00`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_ON_MENU_DESTROYED: u32 = 0x0070_bc00;
/// Destructor helpers: the `float` read of `450410`, and the HUD call that
/// takes it (`0077e430`), the dialog toggle (`00761460`), the fade-out
/// (`00830680`) and the saves-manager call (`00851540`).
const FLOAT_AT_OBJECT: u32 = 0x0045_0410;
const HUD_SET_VALUE: u32 = 0x0077_e430;
const SET_DIALOG_FLAG: u32 = 0x0076_1460;
const FADE_MUSIC: u32 = 0x0083_0680;
const SAVE_MANAGER_RELEASE: u32 = 0x0085_1540;
/// `NiTPointerListBase::RemoveAll` (Ghidra name) of the script list.
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// Destructors of the embedded parts: `NiPointer` (`0045cec0`), the
/// save-load list (`007d7d30`), the downloads list (`007643c0`)
/// and the base `Menu` (`00a1c520`).
const NI_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const SAVE_LOAD_LIST_DESTRUCTOR: u32 = 0x007d_7d30;
const DOWNLOADS_LIST_DESTRUCTOR: u32 = 0x0076_43c0;
const MENU_BASE_DESTRUCTOR: u32 = 0x00a1_c520;
/// Constructor of the lazily created object (`004143c0`, clears two words).
const LAZY_OBJECT_NEW: u32 = 0x0041_43c0;
/// The base constructor of the option list (`007d6eb0`) and the list
/// destructor it calls (`007d6f80`).
const LIST_BOX_BASE_NEW: u32 = 0x007d_6eb0;
const LIST_BOX_BASE_DESTROY: u32 = 0x007d_6f80;
/// The `float` constant (`double` in the exe's data) that `Create`
/// compares the stacking type with: 6008, 6006 and 102.
const STACKING_DOES_NOT_STACK: u32 = 0x0107_0210;
const STACKING_NO_CLICK_PAST_DONE: u32 = 0x0107_04f0;
const STACKING_102: u32 = 0x0107_04e8;
/// Vtables set by the constructors/destructors here: `StartMenu`
/// (`01076d1c`), the option list (`01076d68`) and `StartMenu::Option`
/// (`01076ee0`).
const VTABLE_START_MENU: u32 = 0x0107_6d1c;
const VTABLE_OPTION_LIST: u32 = 0x0107_6d68;
const VTABLE_OPTION: u32 = 0x0107_6ee0;

layout! {
    /// `StartMenu` (Xbox PDB), 0x1CC bytes on the Xbox; the PC build has an extra
    /// word at +0x1C8 and keeps the other fields up to +0x1C4 at the PDB offsets.
    pub struct StartMenu: 0x1d8 {
        /// `iClass` (Xbox PDB, `Menu`).
        0x20 iClass: i32,
        /// `lbMain` (Xbox PDB): `StartMenu::OptionListBox`.
        0x84 lbMain: Inline<ListBox>,
        /// `lbSettings` (Xbox PDB).
        0xB4 lbSettings: Inline<ListBox>,
        /// `lbConfirm` (Xbox PDB).
        0xE4 lbConfirm: Inline<ListBox>,
        /// `lbOptions` (Xbox PDB).
        0x114 lbOptions: Inline<ListBox>,
        /// `lbDownloads` (Xbox PDB): `ListBox<int>`.
        0x144 lbDownloads: Inline<ListBox>,
        /// `lbSaveLoad` (Xbox PDB): `ListBox<BGSSaveLoadFileEntry *>`.
        0x174 lbSaveLoad: Inline<ListBox>,
        /// `pDirtyOption` (Xbox PDB).
        0x1A4 pDirtyOption: Ptr,
        /// `uiFlags` (Xbox PDB).
        0x1A8 uiFlags: u32,
        /// `fSaveLoadThreshold` (Xbox PDB).
        0x1AC fSaveLoadThreshold: f32,
        /// `pSaveGame` (Xbox PDB).
        0x1B0 pSaveGame: Ptr,
        /// `pBrightnessOption` (Xbox PDB).
        0x1B4 pBrightnessOption: Ptr,
        /// `spAlphaMap` (Xbox PDB): `NiPointer<NiSourceTexture>`.
        0x1B8 spAlphaMap: Ptr,
        /// `pDownloadsOption` (Xbox PDB).
        0x1BC pDownloadsOption: Ptr,
        /// `pLoadOption` (Xbox PDB).
        0x1C0 pLoadOption: Ptr,
        /// `pContinueOption` (Xbox PDB).
        0x1C4 pContinueOption: Ptr,
        /// PC only (not in the Xbox PDB): the controls-list device, 0 to 3
        /// (keyboard, mouse, third device, gamepad by the way the name tables
        /// are indexed).
        0x1C8 iRemapDevice: u32,
        /// `ENABLED_TRAIT` (Xbox PDB `+0x1c8`; `+0x1d0` on PC).
        0x1D0 ENABLED_TRAIT: u32,
    }
}

fn menu_flag(e: &mut Engine, menu: Ptr, flag: u32) -> bool {
    e.call(MENU_FLAGS_TEST, &args![menu, flag]).bool()
}

fn start_menu_global(e: &Engine) -> Ptr {
    Ptr::new(e.global::<u32>(START_MENU))
}

// Translated from 007cb4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::GetClass` (Xbox PDB): the class number `0x3f5`.
pub fn start_menu_get_class(_e: &mut Engine, _this: Ptr<StartMenu>) -> u32 {
    START_MENU_CLASS
}

// Translated from 007cb4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]`
/// when `0 <= id < 23`.
pub fn start_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<StartMenu>, id: i32, tile: Ptr) {
    if (0..0x17).contains(&id) {
        e.mem
            .set_u32(this.addr() + 0x28 + 4 * id as u32, tile.addr());
    }
}

// Translated from 007cb510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// and, when bit 0 of `flags` is set, frees the object.
pub fn start_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<StartMenu>,
    flags: u32,
) -> Ptr<StartMenu> {
    fn_007cb5b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007cb540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the start menu's option list (`StartMenu::OptionListBox`
/// per its vtable name `Highlight`, unconfirmed): runs the base list
/// constructor `007d6eb0` and sets the vtable at `01076d68`. Returns `this`.
pub fn fn_007cb540(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_NEW, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_OPTION_LIST);
    this
}

// Translated from 007cb560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The option list's destructor body: runs the base list destructor
/// `007d6f80`.
pub fn fn_007cb560(e: &mut Engine, this: Ptr<ListBox>) {
    e.call(LIST_BOX_BASE_DESTROY, &args![this]);
}

// Translated from 007cb580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The option list's scalar deleting destructor: destructor `007cb560`, then
/// frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_007cb580(e: &mut Engine, this: Ptr<ListBox>, flags: u32) -> Ptr<ListBox> {
    fn_007cb560(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007cb5b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cb5b0 - the C++ exception frame (FS:[0], unwind states 7..0) and the /GS cookie are not translated; if a destructor of a part threw, the remaining parts would not be destroyed
/// The body `StartMenu`'s scalar deleting destructor calls (unlabelled in
/// Ghidra, so the name is by role): sets the vtable, restores the volume
/// when flag `0x40` is set, tells the audio object, closes the dialog when
/// the XUI is up, releases the save manager, clears `StartMenu::pMe`, empties
/// the script list, drops the lazy object, then destroys the embedded parts
/// in reverse order (alpha map, save-load list, downloads list, the four
/// option lists) and the base `Menu`.
pub fn fn_007cb5b0(e: &mut Engine, this: Ptr<StartMenu>) {
    e.mem.set_u32(this.addr(), VTABLE_START_MENU);
    if menu_flag(e, this.cast(), 0x40) {
        let volume = e.call(FLOAT_AT_OBJECT, &args![VOLUME_OBJECT]).f32();
        e.call(HUD_SET_VALUE, &args![volume]);
    }
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_ON_MENU_DESTROYED, &args![audio]);
    if e.call(XUI_IS_UP, &args![]).bool() {
        e.call(SET_DIALOG_FLAG, &args![0u32]);
        e.call(FADE_MUSIC, &args![0x7d0u32, 0u32]);
    }
    let owner = e.global::<u32>(CALLBACK_OWNER);
    e.call(SAVE_MANAGER_RELEASE, &args![owner]);
    e.set_global(START_MENU, 0u32);
    e.call(LIST_REMOVE_ALL, &args![SCRIPT_LIST]);
    fn_007cb710(e);
    fn_007cb7a0(e);
    e.call(NI_POINTER_DESTRUCTOR, &args![this.byte_add(0x1b8)]);
    e.call(SAVE_LOAD_LIST_DESTRUCTOR, &args![this.byte_add(0x174)]);
    e.call(DOWNLOADS_LIST_DESTRUCTOR, &args![this.byte_add(0x144)]);
    for offset in [0x114, 0xe4, 0xb4, 0x84] {
        fn_007cb560(e, this.byte_add(offset).cast());
    }
    e.call(MENU_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 007cb710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cb710 - the C++ exception frame (FS:[0]) and the /GS cookie are not translated; the unwind funclet that frees the 8-byte block when the constructor throws is missing
/// The lazily created 8-byte object (constructor `004143c0` clears two
/// words), kept in the global at `011daac4`; returns it.
pub fn fn_007cb710(e: &mut Engine) -> Ptr {
    if e.global::<u32>(LAZY_OBJECT) == 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let object = if block == 0 {
            0
        } else {
            e.call(LAZY_OBJECT_NEW, &args![block]).u32()
        };
        e.set_global(LAZY_OBJECT, object);
    }
    Ptr::new(e.global::<u32>(LAZY_OBJECT))
}

// Translated from 007cb7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the lazily created object and clears the global.
pub fn fn_007cb7a0(e: &mut Engine) {
    let object = e.global::<u32>(LAZY_OBJECT);
    if object != 0 {
        e.call(OPERATOR_DELETE, &args![object]);
    }
    e.set_global(LAZY_OBJECT, 0u32);
}

// Translated from 007cb7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cb7d0 - the C++ exception frame (FS:[0]) and the /GS cookie are not translated; the scope guard is not destroyed if a callee throws
/// `StartMenu::Create` (Ghidra name; cdecl, two byte arguments): takes the
/// creation tick counts, sets up the loading menu for the main menu when
/// `flag_a` is 0, builds the menu elements when none exist, refuses (returns
/// 0) when another menu that is fading in or out and does not carry
/// `eDoesNotStack` is in the menus list, otherwise destroys any old start
/// menu, reads `Data\Menus\Options\start_menu.xml` as the menu, installs it as
/// `StartMenu::pMe` and runs `_Create`. Returns whether the menu was made.
pub fn start_menu_create(e: &mut Engine, flag_a: u8, flag_b: u8) -> bool {
    // The exe's frame (`SUB ESP,0x50` under the exception record, then `ESI` and the record
    // link): the scope at `EBP-0x24` (+0x40), the menu list cursor at `EBP-0x28` (+0x3c).
    e.with_stack(0x64, |e, frame| {
        let guard = frame.addr() + 0x40;
        e.call(
            SCOPE_GUARD_NEW,
            &args![guard, 0xdu32, 1u32, SOURCE_FILE_NAME, 0x151u32],
        );
        let result = start_menu_create_body(e, frame.addr(), flag_a, flag_b);
        e.call(SCOPE_GUARD_DESTROY, &args![guard]);
        result
    })
}

fn start_menu_create_body(e: &mut Engine, frame: u32, flag_a: u8, flag_b: u8) -> bool {
    let now = e.call(TICK_COUNT, &args![]).u32();
    e.set_global(TICK_CREATED, now);
    if flag_a != 0 {
        let now = e.call(TICK_COUNT, &args![]).u32();
        e.set_global(TICK_CREATED_NEW_GAME, now);
    }
    if flag_a == 0 && e.global::<u8>(LOADING_BYTE) == 0 {
        e.call(SETUP_FOR_MAIN_MENU, &args![1u32]);
    }
    let owner = e.global::<u32>(CALLBACK_OWNER);
    if owner != 0 {
        e.call(OWNER_SET_CALLBACK_A, &args![owner, MENU_CALLBACK]);
        e.call(OWNER_SET_CALLBACK_B, &args![owner, MENU_CALLBACK]);
    }
    if e.call(ELEMENTS_MISSING, &args![ELEMENTS_OBJECT]).bool() {
        start_menu_create_menu_elements(e);
    }
    let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
    let root = e.call(MENUS_ROOT_GETTER, &args![manager]).u32();
    let list = root + 4;
    let first = e.call(FIRST_WORD, &args![list]).u32();
    let blocked = 'scan: {
        let position = Ptr::<()>::new(frame + 0x3c);
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            let slot = e
                .call(POINTER_LIST_STEP_GLOBAL, &args![list, position])
                .u32();
            let tile = e.mem.u32(slot);
            let menu = if tile == 0 {
                0
            } else {
                e.call(TILE_GET_MENU, &args![tile]).u32()
            };
            if menu != 0 && e.call(NODE_NEXT, &args![menu]).u32() != 0 {
                let state = e.call(MENU_FADE_STATE, &args![menu]).u32();
                if state == 8 || state == 2 {
                    let stacking = tile_value(e, Ptr::new(tile), TRAIT_STACKING_TYPE);
                    if f64::from(stacking) != e.global::<f64>(STACKING_DOES_NOT_STACK) {
                        break 'scan true;
                    }
                }
            }
        }
        false
    };
    if blocked {
        return false;
    }
    let old = e
        .call(TILE_GET_MENU_BY_CLASS, &args![START_MENU_CLASS])
        .u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f32();
    e.call(TILE_LOCK, &args![]);
    let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
    let root = e.call(MENUS_ROOT_GETTER, &args![manager]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, START_MENU_XML]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    if menu != 0 {
        let class = e.vcall(menu, 0x34, &args![]).u32();
        if class == e.call(START_MENU_CLASS_GETTER, &args![]).u32() {
            e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
            e.set_global(START_MENU, menu);
            let stacking = tile_value(e, Ptr::new(tile), TRAIT_STACKING_TYPE);
            let stacking = f64::from(stacking);
            if stacking == e.global::<f64>(STACKING_NO_CLICK_PAST_DONE) || {
                let again = tile_value(e, Ptr::new(tile), TRAIT_STACKING_TYPE);
                f64::from(again) == e.global::<f64>(STACKING_102)
            } {
                set_tile_value(e, Ptr::new(tile), TRAIT_DEPTH, depth, 1);
            }
            let this = e.global::<u32>(START_MENU);
            start_menu_create_inner(e, Ptr::new(this), flag_a, flag_b);
            e.call(TILE_UNLOCK, &args![]);
            return true;
        }
    }
    if menu != 0 && e.call(NODE_NEXT, &args![menu]).u32() != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    e.call(TILE_UNLOCK, &args![]);
    false
}

/// `TES::CreateTextureImage(path, &texture, 0, 0)` (Ghidra name), on the
/// object in the global `011dea10`; `RET 0x10`.
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const TEXTURE_OWNER: u32 = 0x011d_ea10;
/// `"Textures\Interface\Main\screenshot_alpha_frame.dds"`.
const TEXT_ALPHA_FRAME: u32 = 0x0107_6ea8;
/// `ListBox<unsigned int>::Initialize(tile, template name)` (Ghidra name),
/// `RET 8`, with the option lists' template names (`"lb_item_template"`,
/// `"lb_saveload_template"`).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
const TEXT_ITEM_TEMPLATE: u32 = 0x0107_6e94;
const TEXT_SAVELOAD_TEMPLATE: u32 = 0x0107_6e7c;
/// The save manager object (`011de134`) writes the saves directory path
/// into a buffer (`0084ff30`, `RET 4`); without it the base path
/// (`004dc110` returns `01202fa0`) and the saves folder name from the
/// setting at `011c3f74` (`00403df0`, then `00464f30`, which returns its
/// argument) are joined with `strcpy_s`/`strcat_s` (`00406d30`, `00406d50`).
const SAVE_MANAGER_PATH: u32 = 0x0084_ff30;
const BASE_PATH_GETTER: u32 = 0x004d_c110;
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
const IDENTITY: u32 = 0x0046_4f30;
const SAVES_FOLDER_SETTING: u32 = 0x011c_3f74;
/// `GameSetting` string getter (`00403df0`, `this` = the setting's
/// address): the string at `+4`, 0 for a null `this`.
const SETTING_STRING: u32 = 0x0040_3df0;
/// The two literals appended to the saves path (`0101ffbc`, `01076e74`).
const TEXT_PATH_SEPARATOR: u32 = 0x0101_ffbc;
const TEXT_SAVE_PATTERN: u32 = 0x0107_6e74;
/// Import slots of `FindFirstFileA` and `FindClose`.
const FIND_FIRST_FILE: u32 = 0x00fd_f070;
const FIND_CLOSE: u32 = 0x00fd_f064;
/// The options array (`StartMenu::pOptionsA`, `BSSimpleArray<Option *>`
/// at `011dab88`): size getter (`0044ddc0`, the word at `+8`) and element
/// address (`00877a30`, `RET 4`).
const OPTIONS_ARRAY: u32 = 0x011d_ab88;
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `ListBox<StartMenu::Option_P>::AddItem(option, 0, 0, 0)` (Ghidra name),
/// `RET 0x10`.
const LIST_BOX_ADD_ITEM: u32 = 0x007d_6ff0;
/// The choose functions the options are recognised by.
const CHOOSE_007D06C0: u32 = 0x007d_06c0;
const CHOOSE_007D0440: u32 = 0x007d_0440;
const CHOOSE_007D0680: u32 = 0x007d_0680;
const CHOOSE_HELP: u32 = 0x007d_0770;
const CHOOSE_007D0550: u32 = 0x007d_0550;
/// `00850fe0` (on the save manager, `RET 4`): whether saving works.
const SAVE_MANAGER_CAN_SAVE: u32 = 0x0085_0fe0;
/// The menu class `0x423` (`007cc6d0`) looked up before the help option
/// is greyed.
const HELP_MENU_CLASS: u32 = 0x423;
/// List helpers: element count of a list (`00726c40`, on the list box),
/// and `Tile::SetValue(id, (float)value, 1)` through the interface helper
/// (`00715c60`, `RET 8`).
const LIST_BOX_COUNT: u32 = 0x0072_6c40;
const TILE_SET_COUNT: u32 = 0x0071_5c60;
/// The user trait `"_number_of_visible_items"`.
const TEXT_NUMBER_OF_VISIBLE_ITEMS: u32 = 0x0106_f860;
/// The version string `"1.4.0.525"` shown in the menu.
const TEXT_VERSION: u32 = 0x0107_6e68;
/// Settings whose strings fill the labels (`GameSetting` objects).
const SETTING_LABEL_A: u32 = 0x011d_2eec;
const SETTING_LABEL_B: u32 = 0x011d_48a0;
const SETTING_LABEL_C: u32 = 0x011d_2070;
const SETTING_LABEL_D: u32 = 0x011d_1fec;
/// Screen metrics: `float` getters (`00706e50`: the width tested against
/// 640 and `00706e70`: `Interface::GetScreenAspectRatio`, Ghidra name), and
/// the `double` constants they are compared with / added to.
const SCREEN_WIDTH: u32 = 0x0070_6e50;
const SCREEN_ASPECT_RATIO: u32 = 0x0070_6e70;
const WIDTH_LIMIT: u32 = 0x0107_4b88;
const ASPECT_LIMIT: u32 = 0x0107_6e60;
const Y_OFFSET_SMALL_SCREEN: u32 = 0x0104_f2f0;
/// `StartMenu::Fade(kind, flag, mask, float)` (Ghidra name), `RET 0x10`.
const FADE: u32 = 0x007d_61e0;
/// `float` constants: -1.0 (`01012054`), the background model scale
/// (`01073f20`) and 255.0 (`01023cd8`).
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
const FLOAT_MODEL_SCALE: u32 = 0x0107_3f20;
const FLOAT_255: u32 = 0x0102_3cd8;
/// `ListBox<StartMenu::Option_P>::SetEnabled(bool)` (Ghidra name),
/// `RET 4`.
const LIST_BOX_SET_ENABLED: u32 = 0x007d_74f0;
/// Tile model getter (`0056c7f0`) and the model update taking a `float`
/// (`00440490`, `RET 4`).
const TILE_MODEL: u32 = 0x0056_c7f0;
const MODEL_SET_SCALE: u32 = 0x0044_0490;
const TEXT_BACKGROUND_MODEL_NULL: u32 = 0x0107_6e2c;
/// `InterfaceManager::GetEnterStack(index)` (Ghidra name), `RET 4`.
const GET_ENTER_STACK: u32 = 0x0071_4f70;
/// `NiTPointerList::AddTail`-like append of a pointer, given by address
/// (`004ed8c0`, `RET 4`).
const POINTER_LIST_ADD: u32 = 0x004e_d8c0;
/// `Interface::IsInRenderedMenu` (Ghidra name) and
/// `Main::RenderMenuBackground` (Ghidra name; `this` is the global at
/// `011dea0c`).
const IS_IN_RENDERED_MENU: u32 = 0x0070_79b0;
const RENDER_MENU_BACKGROUND: u32 = 0x0087_1dc0;
const MAIN_OBJECT: u32 = 0x011d_ea0c;
/// `VATS::SetMode(0, 0)` (Ghidra name) on the VATS object at `011f2250`,
/// `RET 8`.
const VATS_SET_MODE: u32 = 0x009c_6c30;
const VATS_OBJECT: u32 = 0x011f_2250;
/// `Interface::IsTopMenuID(class)` (Ghidra name), cdecl.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// Dialog/pause helpers: `00703fe0(0)`, `MessageMenu::SetPauseMenuMode`
/// (Ghidra name), `Interface::InDialog`, `Interface::SetDialogPause`.
const CLOSE_PAUSE_HELPER: u32 = 0x0070_3fe0;
const SET_PAUSE_MENU_MODE: u32 = 0x007a_af20;
const IN_DIALOG: u32 = 0x0070_50d0;
const SET_DIALOG_PAUSE: u32 = 0x0070_5110;
/// The credits music: an object with a `float` value (`011c3f3c`, getter
/// `00526ac0`), `memset`, `wcscpy`-like copy (`00406d00`), the media start
/// (`008300c0`, seven arguments) and the media finish (`008304c0`).
const CREDITS_SETTING: u32 = 0x011c_3f3c;
const SETTING_FLOAT: u32 = 0x0052_6ac0;
const MEMSET: u32 = 0x0040_3d30;
const STRING_COPY_NARROW: u32 = 0x0040_6d00;
const MEDIA_PLAY: u32 = 0x0083_00c0;
const MEDIA_FINISH: u32 = 0x0083_04c0;
const TEXT_END_CREDITS_MUSIC: u32 = 0x0107_6e08;
/// `"_xbox_extra"` user trait.
const TEXT_XBOX_EXTRA: u32 = 0x0107_6dfc;
/// `Menu::PrepForVisibility(0)` (Ghidra name), `RET 4`.
const PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// `BSNetworkSystemUtility::QInstance` (Ghidra name) and the method
/// (`00af2d70`, `RET 4`) that takes the callback `007d6990`.
const NETWORK_UTILITY_INSTANCE: u32 = 0x00af_2c90;
const NETWORK_UTILITY_SET_CALLBACK: u32 = 0x00af_2d70;
const NETWORK_CALLBACK: u32 = 0x007d_6990;
/// The object whose flag `007cc680(false)` clears (`011ddf38`).
const SIGN_IN_OBJECT: u32 = 0x011d_df38;
/// The function at `007d3aa0` (other half of this unit), called on the menu.
const FN_007D3AA0: u32 = 0x007d_3aa0;

// Translated from 007cbaf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cbaf0 - the C++ exception frame (FS:[0]) and the /GS cookie check are not translated; the scope guard is not destroyed if a callee throws
/// `StartMenu::_Create` (Ghidra name): builds the menu's contents once the
/// XML is read. Stores the alpha-frame texture, sets the creation flags,
/// initialises the six lists, fills the main/settings lists from the options
/// whose modes fit (greying the ones that cannot run), sets the visible
/// item counts and the label strings, then either sets up the in-game
/// (`flag_a` set) or main-menu look (VATS off, background alpha) and plays
/// the end-credits music when the credits setting is non-zero.
pub fn start_menu_create_inner(e: &mut Engine, this: Ptr<StartMenu>, flag_a: u8, flag_b: u8) {
    // The exe's frame (`SUB ESP,0x2d0` under the exception record, then the record link): the
    // scope at `EBP-0x120` (+0x1c0), the save path at `EBP-0x11c` (+0x1c4), the find data at
    // `EBP-0x264` (+0x7c), the tile at `EBP-0x280` (+0x60) and the music name at `EBP-0x2c4`
    // (+0x1c).
    e.with_stack(0x2e0, |e, frame| {
        let guard = frame.addr() + 0x1c0;
        e.call(
            SCOPE_GUARD_NEW,
            &args![guard, 0xdu32, 1u32, SOURCE_FILE_NAME, 0x1aau32],
        );
        start_menu_create_inner_body(e, frame.addr(), this, flag_a, flag_b);
        e.call(SCOPE_GUARD_DESTROY, &args![guard]);
    });
}

fn start_menu_create_inner_body(
    e: &mut Engine,
    frame: u32,
    this: Ptr<StartMenu>,
    flag_a: u8,
    flag_b: u8,
) {
    let texture_owner = e.global::<u32>(TEXTURE_OWNER);
    e.call(
        CREATE_TEXTURE_IMAGE,
        &args![
            texture_owner,
            TEXT_ALPHA_FRAME,
            this.byte_add(0x1b8),
            0u32,
            0u32
        ],
    );
    e.call(MENU_FLAGS_SET, &args![this, 1u32, u32::from(flag_a)]);
    e.call(MENU_FLAGS_SET, &args![this, 4u32, u32::from(flag_b)]);
    let mask: u32 = if e.global::<u8>(LOADING_BYTE) != 0 && flag_a == 0 {
        0x80000
    } else {
        0x10
    };
    e.call(MENU_FLAGS_SET, &args![this, mask, 1u32]);
    let lists = [
        (0x84u32, 0x28u32, TEXT_ITEM_TEMPLATE),
        (0xb4, 0x2c, TEXT_ITEM_TEMPLATE),
        (0xe4, 0x44, TEXT_ITEM_TEMPLATE),
        (0x114, 0x30, 0),
        (0x174, 0x40, TEXT_SAVELOAD_TEMPLATE),
        (0x144, 0x60, TEXT_ITEM_TEMPLATE),
    ];
    for (list, tile, template) in lists {
        let tile = e.mem.u32(this.addr() + tile);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![this.byte_add(list), tile, template],
        );
    }
    e.call(
        MENU_FLAGS_SET,
        &args![this, 0x80u32, u32::from(flag_a == 0)],
    );

    let found = {
        let path = frame + 0x1c4;
        let owner = e.global::<u32>(CALLBACK_OWNER);
        if owner != 0 {
            e.call(SAVE_MANAGER_PATH, &args![owner, path]);
        } else {
            let base = e.call(BASE_PATH_GETTER, &args![]).u32();
            e.call(STRING_COPY, &args![path, 0x104u32, base]);
            let folder = e.call(SETTING_STRING, &args![SAVES_FOLDER_SETTING]).u32();
            let folder = e.call(IDENTITY, &args![folder, 0u32]).u32();
            e.call(STRING_APPEND, &args![path, 0x104u32, folder]);
        }
        e.call(STRING_APPEND, &args![path, 0x104u32, TEXT_PATH_SEPARATOR]);
        e.call(STRING_APPEND, &args![path, 0x104u32, TEXT_SAVE_PATTERN]);
        let data = frame + 0x7c;
        let handle = e.call(FIND_FIRST_FILE, &args![path, data]).u32();
        e.call(FIND_CLOSE, &args![handle]);
        handle != 0xffff_ffff
    };

    let count = e.call(ARRAY_SIZE, &args![OPTIONS_ARRAY]).u32();
    let mut index = 0;
    while index < count {
        let mut item = 0u32;
        let option = option_at(e, index);
        let choose = e.mem.u32(option + 8);
        let owner = e.global::<u32>(CALLBACK_OWNER);
        let help_class = fn_007cc6d0(e);
        let grayed = (choose == CHOOSE_007D06C0
            && !e.call(SAVE_MANAGER_CAN_SAVE, &args![owner, 0u32]).bool())
            || (choose == CHOOSE_007D0440
                && ((flag_a != 0 && flag_b != 0) || (flag_a == 0 && !found)))
            || (choose == CHOOSE_007D0680 && !found)
            || (choose == CHOOSE_HELP
                && e.call(TILE_GET_MENU_BY_CLASS, &args![help_class]).u32() != 0);
        let in_main_list = if flag_a != 0 {
            fn_007cc5d0(e, Ptr::new(option), 2)
        } else {
            fn_007cc5d0(e, Ptr::new(option), 1)
        };
        if in_main_list {
            item = e
                .call(
                    LIST_BOX_ADD_ITEM,
                    &args![this.byte_add(0x84), option, 0u32, 0u32, 0u32],
                )
                .u32();
        } else if fn_007cc5d0(e, Ptr::new(option), 4) {
            item = e
                .call(
                    LIST_BOX_ADD_ITEM,
                    &args![this.byte_add(0xb4), option, 0u32, 0u32, 0u32],
                )
                .u32();
        }
        let choose = e.mem.u32(option + 8);
        if choose == CHOOSE_007D0550 {
            e.mem.set_u32(this.addr() + 0x1bc, item);
        } else if choose == CHOOSE_007D0440 {
            e.mem.set_u32(this.addr() + 0x1c4, item);
        } else if choose == CHOOSE_007D0680 {
            e.mem.set_u32(this.addr() + 0x1c0, item);
        }
        if item != 0 {
            let text = e.mem.u32(option + 4);
            e.call(TILE_SET_STRING, &args![item, TRAIT_STRING, text, 1u32]);
            if grayed {
                let enabled_trait = e.get(this, StartMenu::ENABLED_TRAIT);
                e.call(TILE_SET_INT, &args![item, enabled_trait, 0u32]);
            }
        }
        index += 1;
    }

    for (list, tile) in [(0x84u32, 0x28u32), (0xb4, 0x2c)] {
        let visible = e.call(LIST_BOX_COUNT, &args![this.byte_add(list)]).u32();
        let id = e
            .call(
                TILE_ADD_USER_TRAIT,
                &args![TEXT_NUMBER_OF_VISIBLE_ITEMS, 0xffff_ffffu32],
            )
            .u32();
        let tile = e.mem.u32(this.addr() + tile);
        e.call(TILE_SET_COUNT, &args![tile, id, visible]);
    }
    let version = e.mem.u32(this.addr() + 0x50);
    e.call(
        TILE_SET_STRING,
        &args![version, TRAIT_STRING, TEXT_VERSION, 1u32],
    );
    for (offset, setting) in [
        (0x4cu32, SETTING_LABEL_A),
        (0x78, SETTING_LABEL_B),
        (0x64, SETTING_LABEL_C),
        (0x3c, SETTING_LABEL_D),
    ] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = e.mem.u32(this.addr() + offset);
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
    }
    let tile = e.mem.u32(this.addr() + 0x7c);
    e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 0u32]);
    let width = e.call(SCREEN_WIDTH, &args![]).f64();
    let narrow = width <= e.global::<f64>(WIDTH_LIMIT);
    let tile = e.mem.u32(this.addr() + 0x34);
    if narrow && tile != 0 {
        let y = tile_value(e, Ptr::new(tile), TRAIT_Y);
        let offset = e.global::<f64>(Y_OFFSET_SMALL_SCREEN);
        set_tile_value(
            e,
            Ptr::new(tile),
            TRAIT_Y,
            (f64::from(y) + offset) as f32,
            1,
        );
    }
    let aspect = e.call(SCREEN_ASPECT_RATIO, &args![]).f64();
    let wide = aspect > e.global::<f64>(ASPECT_LIMIT);
    let menu_tile = e.mem.u32(this.addr() + 4);
    e.call(
        TILE_SET_INT,
        &args![menu_tile, TRAIT_USER_0, u32::from(wide)],
    );

    if flag_a != 0 {
        let tile = e.mem.u32(this.addr() + 0x34);
        e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 0u32]);
        e.call(TILE_SET_INT, &args![menu_tile, TRAIT_ALPHA, 0x40u32]);
        let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
        e.call(FADE, &args![this, 0u32, 1u32, 0x8000_0000u32, minus_one]);
        e.call(LIST_BOX_SET_ENABLED, &args![this.byte_add(0x84), 1u32]);
        e.call(FN_007D3AA0, &args![this]);
        let background = e.mem.u32(this.addr() + 0x5c);
        let model = e.call(TILE_MODEL, &args![background]).u32();
        if model != 0 {
            let scale = e.global::<f32>(FLOAT_MODEL_SCALE);
            let model = e.call(TILE_MODEL, &args![background]).u32();
            e.call(MODEL_SET_SCALE, &args![model, scale]);
        } else {
            e.call(LOG_MESSAGE, &args![0x12u32, TEXT_BACKGROUND_MODEL_NULL]);
        }
        for depth in 0..10u32 {
            let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
            let entry = e.call(GET_ENTER_STACK, &args![manager, depth]).u32();
            if e.call(START_MENU_CLASS_GETTER, &args![]).u32() == entry {
                break;
            }
            if fn_007cc5f0(e, entry) {
                let tile = e.call(TILE_GET_MENU_BY_CLASS, &args![entry]).u32();
                if tile != 0 && e.call(TILE_GET_MENU, &args![tile]).u32() != 0 {
                    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
                    if e.call(MENU_FADE_STATE, &args![menu]).u32() == 1 {
                        e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 0u32]);
                        let cell = frame + 0x60;
                        e.mem.set_u32(cell, tile);
                        e.call(POINTER_LIST_ADD, &args![SCRIPT_LIST, cell]);
                    }
                }
            }
        }
        if e.call(IS_IN_RENDERED_MENU, &args![]).bool() {
            let main = e.global::<u32>(MAIN_OBJECT);
            e.call(RENDER_MENU_BACKGROUND, &args![main]);
        }
    } else {
        e.call(VATS_SET_MODE, &args![VATS_OBJECT, 0u32, 0u32]);
        let tile = e.mem.u32(this.addr() + 0x34);
        e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 1u32]);
        let alpha = if e.call(IS_TOP_MENU_ID, &args![START_MENU_CLASS]).bool() {
            0.0f32
        } else {
            e.global::<f32>(FLOAT_255)
        };
        set_tile_value(e, Ptr::new(tile), TRAIT_ALPHA, alpha, 1);
        e.call(MENU_FLAGS_SET, &args![this, 0x20u32, 1u32]);
    }

    if menu_flag(e, this.cast(), 4) {
        fn_007d0680(e);
    }
    if e.call(XUI_IS_UP, &args![]).bool() {
        e.call(CLOSE_PAUSE_HELPER, &args![0u32]);
    } else {
        e.call(SET_PAUSE_MENU_MODE, &args![1u32]);
    }
    if e.call(IN_DIALOG, &args![]).bool() {
        e.call(SET_DIALOG_PAUSE, &args![1u32]);
    }
    let credits = e.global::<u32>(CREDITS_SETTING);
    if credits != 0 && e.call(SETTING_FLOAT, &args![credits]).f64() != 0.0 {
        let menu = e.global::<u32>(START_MENU);
        e.call(MENU_FLAGS_SET, &args![menu, 0x10000u32, 1u32]);
        e.call(MENU_FLAGS_SET, &args![menu, 0x4000_0000u32, 1u32]);
        {
            let name = frame + 0x1c;
            e.call(MEMSET, &args![name, 0u32, 0x40u32]);
            e.call(
                STRING_COPY_NARROW,
                &args![name, 0x40u32, TEXT_END_CREDITS_MUSIC],
            );
            e.call(
                MEDIA_PLAY,
                &args![8u32, name, 0x7d0u32, 1u32, 1u32, 0.0f32, 0u32],
            );
            e.call(MEDIA_FINISH, &args![]);
        }
    }
    let menu = start_menu_global(e);
    if !menu_flag(e, menu, 1) {
        let id = e
            .call(TILE_ADD_USER_TRAIT, &args![TEXT_XBOX_EXTRA, 0xffff_ffffu32])
            .u32();
        let tile = e.mem.u32(this.addr() + 0x28);
        e.call(TILE_SET_INT, &args![tile, id, 0x19u32]);
    }
    e.call(PREP_FOR_VISIBILITY, &args![this, 0u32]);
    let network = e.call(NETWORK_UTILITY_INSTANCE, &args![]).u32();
    e.call(
        NETWORK_UTILITY_SET_CALLBACK,
        &args![network, NETWORK_CALLBACK],
    );
    let sign_in = e.global::<u32>(SIGN_IN_OBJECT);
    if sign_in != 0 {
        fn_007cc680(e, Ptr::new(sign_in), 0);
    }
}

fn option_at(e: &mut Engine, index: u32) -> u32 {
    option_in(e, OPTIONS_ARRAY, index)
}

/// `BSSimpleArray<StartMenu::UserOption *, 1024>` of the user options
/// (`StartMenu::pUserOptionsA`, Xbox PDB) at `011dab50`.
const USER_OPTIONS_ARRAY: u32 = 0x011d_ab50;
/// `BSSimpleArray::Add(&item)` (`007cb2e0`, `RET 4`).
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// The option text getter (`004c69f0`, `this` = the setting's address; no
/// arguments) and `StartMenu::UserOption::UserOption` (`007d6350`, cdecl:
/// `this`, text, choose, modes, type, number of settings, arrow spacing,
/// then the value strings in reverse order). The plain option constructor
/// is `fn_007ce620` below.
const OPTION_TEXT: u32 = 0x004c_69f0;
const USER_OPTION_NEW: u32 = 0x007d_6350;
/// Sets a string from a setting's text (`004037f0(text, 0)`, `RET 8`) and
/// the call made on the string afterwards (`007b52a0`).
const STRING_SET: u32 = 0x0040_37f0;
const STRING_UPDATE: u32 = 0x007b_52a0;
/// The two strings `CreateMenuElements` fills while the first is empty
/// (`011dabcc`, `011daad0`) and the settings that provide their text.
const STRING_A: u32 = 0x011d_abcc;
const STRING_B: u32 = 0x011d_aad0;
const SETTING_STRING_A: u32 = 0x011d_4630;
const SETTING_STRING_B: u32 = 0x011d_4354;
/// `BSSimpleArray::Clear(true)` (`008454f0`, `RET 4`) and the call made on
/// the object at `011daae0` at the end of `007ce6b0` (`007d8e20`, `RET 4`).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
const SAVE_LOAD_ENTRIES: u32 = 0x011d_aae0;
const SAVE_LOAD_ENTRIES_CLEAR: u32 = 0x007d_8e20;

/// The options `CreateMenuElements` creates, in order: (text setting,
/// choose function, modes). The choose functions are in this file and in
/// the second half of the unit.
const OPTION_ROWS: [(u32, u32, u32); 23] = [
    (0x011d_3564, 0x007d_0440, 3),
    (0x011d_216c, 0x007d_0490, 1),
    (0x011d_3b8c, 0x007d_06c0, 2),
    (0x011d_4888, 0x007d_0680, 3),
    (0x011d_1fe0, 0x007d_0700, 3),
    (0x011d_2580, 0x007d_0770, 2),
    (0x011d_46b4, 0x007d_04d0, 1),
    (0x011d_433c, 0x007d_0550, 1),
    (0x011d_42e8, 0x007d_0a10, 3),
    (0x011d_3d90, 0x007d_0d40, 4),
    (0x011d_375c, 0x007d_0d70, 4),
    (0x011d_219c, 0x007d_0dc0, 4),
    (0x011d_1f50, 0x007d_0df0, 4),
    (0x011d_34f8, 0x007d_32b0, 128),
    (0x011d_34f8, 0x007d_3190, 256),
    (0x011d_34f8, 0x007d_3470, 512),
    (0x011d_34f8, 0x007d_35c0, 1024),
    (0x011d_34f8, 0x007d_3620, 2048),
    (0x011d_3684, 0x007d_0e40, 3968),
    (0x011d_2730, 0x007d_0a70, 8192),
    (0x011d_1e84, 0x007d_0bf0, 12288),
    (0x011d_29f4, 0x007d_0e40, 12288),
    (0x011d_3828, 0x007d_0c50, 64),
];

/// The user options `CreateMenuElements` creates, in order: (text setting,
/// choose function, the words after the choose function in the order the
/// constructor takes them (modes, type, number of settings, arrow spacing),
/// and the settings of the value strings in the order the code reads them
/// (the constructor gets them in reverse)).
#[allow(clippy::type_complexity)]
const USER_OPTION_ROWS: [(u32, u32, [u32; 4], &[u32]); 34] = [
    (
        0x011d_366c,
        0x007d_1260,
        [8, 0, 3, 85],
        &[0x011d_4fc0, 0x011d_466c, 0x011d_4ed0],
    ),
    (
        0x011d_1d78,
        0x007d_13a0,
        [8, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (
        0x011d_1e30,
        0x007d_11d0,
        [8, 0, 5, 85],
        &[
            0x011c_39a8,
            0x011c_3a94,
            0x011d_2fc4,
            0x011c_3a70,
            0x011c_39e8,
        ],
    ),
    (
        0x011d_4a20,
        0x007d_1510,
        [8, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (
        0x011d_3bec,
        0x007d_1570,
        [8, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (
        0x011d_3a50,
        0x007d_15d0,
        [8, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (
        0x011d_3ef8,
        0x007d_16f0,
        [8, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (0x011d_2520, 0x007d_19c0, [16, 2, 16, 0], &[]),
    (0x011d_2f34, 0x007d_1750, [16, 2, 16, 0], &[]),
    (
        0x011d_2940,
        0x007d_1840,
        [16, 0, 4, 55],
        &[0x011d_2124, 0x011d_5074, STRING_A, STRING_B],
    ),
    (
        0x011d_23c4,
        0x007d_1900,
        [16, 0, 4, 55],
        &[0x011d_2124, 0x011d_5074, STRING_A, STRING_B],
    ),
    (
        0x011d_21f0,
        0x007d_1b40,
        [16, 0, 3, 55],
        &[0x011d_2430, 0x011d_2cdc, 0x011d_4270],
    ),
    (0x011d_4e40, 0x007d_1bb0, [16, 2, 16, 0], &[]),
    (0x011d_22bc, 0x007d_1d10, [16, 2, 16, 0], &[]),
    (0x011d_31b0, 0x007d_1e60, [16, 2, 16, 0], &[]),
    (0x011d_3b98, 0x007d_1fb0, [16, 2, 16, 0], &[]),
    (0x011d_2610, 0x007d_20f0, [16, 2, 16, 0], &[]),
    (0x011d_34d4, 0x007d_2250, [16, 2, 16, 0], &[]),
    (0x011d_2484, 0x007d_23b0, [16, 2, 16, 0], &[]),
    (0x011d_2bec, 0x007d_2510, [16, 2, 16, 0], &[]),
    (0x011d_2afc, 0x007d_2650, [16, 2, 16, 0], &[]),
    (
        0x011d_28f8,
        0x007d_1440,
        [16, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (
        0x011d_2e5c,
        0x007d_1630,
        [16, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (
        0x011d_3c28,
        0x007d_1690,
        [16, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (0x011d_2e8c, 0x007d_2790, [32, 2, 26, 0], &[]),
    (0x011d_3054, 0x007d_29d0, [32, 2, 26, 0], &[]),
    (0x011d_4abc, 0x007d_28b0, [32, 2, 26, 0], &[]),
    (0x011d_24e4, 0x007d_2af0, [32, 2, 26, 0], &[]),
    (0x011d_399c, 0x007d_2c10, [32, 2, 26, 0], &[]),
    (0x011d_1dc0, 0x007d_2d30, [32, 2, 26, 0], &[]),
    (
        0x011d_4648,
        0x007d_2e50,
        [64, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (0x011d_25c8, 0x007d_2eb0, [64, 2, 16, 0], &[]),
    (
        0x011d_49fc,
        0x007d_2fc0,
        [64, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
    (
        0x011d_3750,
        0x007d_30e0,
        [64, 0, 2, 0],
        &[0x011d_4dec, 0x011d_258c],
    ),
];

// Translated from 007cc5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::Option` mode test: whether the option's `iModes` (+0xC) has
/// any bit of `mask`.
pub fn fn_007cc5d0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0xc) & mask != 0
}

// Translated from 007cc5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether menu class `class` is one of the 20 classes the start menu hides
/// when it opens over them (the compiler's byte-table switch over
/// `0x3f0..=0x43c`).
pub fn fn_007cc5f0(_e: &mut Engine, class: u32) -> bool {
    matches!(
        class,
        0x3f0
            | 0x3f1
            | 0x3f4
            | 0x3f6
            | 0x3f8
            | 0x403
            | 0x417
            | 0x418
            | 0x41b
            | 0x41d
            | 0x420
            | 0x423
            | 0x433
            | 0x434
            | 0x435
            | 0x438
            | 0x439
            | 0x43a
            | 0x43b
            | 0x43c
    )
}

// Translated from 007cc680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (nonzero `on`) or clears bit `0x80` of the word at `+0x244` of the
/// object.
pub fn fn_007cc680(e: &mut Engine, this: Ptr, on: u8) {
    let flags = e.mem.u32(this.addr() + 0x244);
    let flags = if on != 0 { flags | 0x80 } else { flags & !0x80 };
    e.mem.set_u32(this.addr() + 0x244, flags);
}

// Translated from 007cc6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class `0x423`.
pub fn fn_007cc6d0(_e: &mut Engine) -> u32 {
    HELP_MENU_CLASS
}

// Translated from 007cc6e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cc6e0 - the C++ exception frame (FS:[0], one unwind state per option) and the /GS cookie are not translated; the 0x10/0x30-byte blocks are not freed if a constructor throws
/// `StartMenu::CreateMenuElements` (Ghidra name): when the first string
/// (`011dabcc`) is empty, fills it and the second (`011daad0`) from their
/// settings; clears the old options (`007ce6b0`) and creates the 23 plain
/// options and 34 user options in order, each a freshly allocated object
/// (0x10 / 0x30 bytes, null when the allocation fails) appended to
/// `pOptionsA` / `pUserOptionsA`.
pub fn start_menu_create_menu_elements(e: &mut Engine) {
    if e.call(FIRST_WORD, &args![STRING_A]).u32() == 0 {
        let text = e.call(SETTING_STRING, &args![SETTING_STRING_A]).u32();
        e.call(STRING_SET, &args![STRING_A, text, 0u32]);
        e.call(STRING_UPDATE, &args![STRING_A]);
        let text = e.call(SETTING_STRING, &args![SETTING_STRING_B]).u32();
        e.call(STRING_SET, &args![STRING_B, text, 0u32]);
        e.call(STRING_UPDATE, &args![STRING_B]);
    }
    fn_007ce6b0(e);
    for (text_setting, choose, modes) in OPTION_ROWS {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let option = if block == 0 {
            0
        } else {
            let text = e.call(OPTION_TEXT, &args![text_setting]).u32();
            fn_007ce620(e, Ptr::new(block), text, choose, modes).addr()
        };
        push_array(e, OPTIONS_ARRAY, option);
    }
    for (text_setting, choose, words, strings) in USER_OPTION_ROWS {
        let block = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
        let option = if block == 0 {
            0
        } else {
            // In push order: settings' strings (00403df0), and for the two 4-value rows
            // (007cd7cf, 007cd884) the texts of STRING_A and STRING_B (00559450) last.
            let values: Vec<u32> = strings
                .iter()
                .map(|&s| {
                    let getter = if s == STRING_A || s == STRING_B {
                        FIRST_WORD
                    } else {
                        SETTING_STRING
                    };
                    e.call(getter, &args![s]).u32()
                })
                .collect();
            let text = e.call(OPTION_TEXT, &args![text_setting]).u32();
            let mut call = args![block, text, choose, words[0], words[1], words[2], words[3]];
            call.extend(values.iter().rev());
            e.call(USER_OPTION_NEW, &call).u32()
        };
        push_array(e, USER_OPTIONS_ARRAY, option);
    }
}

/// `BSSimpleArray::Add` of the word `item`, passed by address as the game
/// passes it (a local).
fn push_array(e: &mut Engine, array: u32, item: u32) {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), item);
        e.call(ARRAY_ADD, &args![array, cell]);
    });
}

// Translated from 007ce620 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `StartMenu::Option` (the engine map and Ghidra label
/// it `Gdiplus::GdiplusStartupInput::GdiplusStartupInput` because the code
/// is identical; the vtable at `01076ee0` is `StartMenu::Option`'s): stores
/// the vtable, `pDisplayText` (+4), `pfChooseFn` (+8) and `iModes` (+0xC).
pub fn fn_007ce620(e: &mut Engine, this: Ptr, text: u32, choose: u32, modes: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_OPTION);
    e.mem.set_u32(this.addr() + 4, text);
    e.mem.set_u32(this.addr() + 8, choose);
    e.mem.set_u32(this.addr() + 0xc, modes);
    this
}

// Translated from 007ce660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::Option::_scalar_deleting_destructor_` (Xbox PDB): destructor
/// (`007ce690`), then frees the object when bit 0 of `flags` is set.
pub fn start_menu_option_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007ce690(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007ce690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StartMenu::Option` destructor body: resets the vtable.
pub fn fn_007ce690(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_OPTION);
}

// Translated from 007ce6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every option and user option (virtual destructor with flag 1),
/// empties both arrays and the save-load entry list at `011daae0`.
pub fn fn_007ce6b0(e: &mut Engine) {
    for array in [OPTIONS_ARRAY, USER_OPTIONS_ARRAY] {
        let mut index = 0;
        while index < e.call(ARRAY_SIZE, &args![array]).u32() {
            let option = option_in(e, array, index);
            if option != 0 {
                e.vcall(option, 0, &args![1u32]);
            }
            index += 1;
        }
        e.call(ARRAY_CLEAR, &args![array, 1u32]);
    }
    e.call(SAVE_LOAD_ENTRIES_CLEAR, &args![SAVE_LOAD_ENTRIES, 1u32]);
}

fn option_in(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(slot)
}

/// `ListBox<StartMenu::Option_P>::IsSelected`-style helpers and the
/// functions of the second half of this unit that the handlers below call,
/// by address. `007d88a0` is `ListBox<BGSSaveLoadFileEntry_P>::IsEnabled`,
/// `007d8810` its `SetEnabled`, `007d8d60` its `SetSelection` (Ghidra names);
/// `007d7aa0` is `ListBox<StartMenu::Option_P>::IsSelected`, `007d7b50` its
/// `DeselectAll`, `007d7bd0` is `ListBox<int>::SetEnabled`, `00764e40`
/// `ListBox<int>::IsEnabled`.
const SAVE_LOAD_LIST_IS_ENABLED: u32 = 0x007d_88a0;
const SAVE_LOAD_LIST_SET_ENABLED: u32 = 0x007d_8810;
const SAVE_LOAD_LIST_SET_SELECTION: u32 = 0x007d_8d60;
const LIST_BOX_IS_SELECTED: u32 = 0x007d_7aa0;
const LIST_BOX_DESELECT_ALL: u32 = 0x007d_7b50;
const INT_LIST_SET_ENABLED: u32 = 0x007d_7bd0;
const INT_LIST_IS_ENABLED: u32 = 0x0076_4e40;
/// `StartMenu::ConfirmSaveLoad` (Ghidra name), `StartMenu::UpdateHighlightedSaveGameInfo`
/// (Ghidra name) and `StartMenu::HideOptionsContainer` (Ghidra name), all
/// on the menu.
const CONFIRM_SAVE_LOAD: u32 = 0x007d_40a0;
const UPDATE_HIGHLIGHTED_SAVE_GAME_INFO: u32 = 0x007d_4df0;
const HIDE_OPTIONS_CONTAINER: u32 = 0x007d_5f80;
/// `ListBox::ForEach(function, 0, 0x7fffffff)` (`007314c0`, `RET 0xC`, the
/// list is `this`).
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
/// `ListBox<StartMenu::Option_P>::GetItemFor(tile)` (`007e1110`, `RET 4`):
/// the item the list holds for a tile (an `Option`, a `UserOption`, a save
/// entry, or -1 for the downloads placeholder).
const LIST_BOX_GET_ITEM: u32 = 0x007e_1110;
/// `ListBox<..>::BelongsTo(tile)` as called here (`007242f0`, `RET 4`) and
/// `007d7a80(tile)`: marks a tile as the list's selection.
const LIST_BOX_HAS_TILE: u32 = 0x0072_42f0;
const LIST_BOX_SELECT: u32 = 0x007d_7a80;
/// `007d7630` (on the options list): whether anything is selected there,
/// and `007d6160`/`007d6d70` (on the menu): follow-up refreshes.
const OPTIONS_LIST_HAS_SELECTION: u32 = 0x007d_7630;
const MENU_REFRESH_A: u32 = 0x007d_6160;
/// Takes no `this`: none of its 4 exe callers (007cec93, 007cfbab ...) sets ECX.
const MENU_REFRESH_B: u32 = 0x007d_6d70;
/// `Interface::ShowMessageBox`-like call made when defaults are reset in the
/// controls list (`00703e80`, cdecl, 11 words): text, then `0, 0,
/// 007d3a30, 0, 0x12, 0.0, 0.0` and two button texts.
const SHOW_MESSAGE: u32 = 0x0070_3e80;
const MESSAGE_CALLBACK: u32 = 0x007d_3a30;
/// `BGSSaveLoadFileEntry::IsCorrupt` (Ghidra name).
const SAVE_ENTRY_IS_CORRUPT: u32 = 0x0084_6b30;
/// Count of a list (`00726c40`, `this` = the list box).
/// The marketplace calls made for a downloads placeholder item:
/// `00a25920` (an object getter), `008d8520`, `00af2640` (Ghidra:
/// `BSSystemUtility::QInstance`), `005dfa10` and `00af28f0`.
const DLC_GETTER: u32 = 0x00a2_5920;
const DLC_STEP_A: u32 = 0x008d_8520;
const DLC_INSTANCE: u32 = 0x00af_2640;
const DLC_STEP_B: u32 = 0x005d_fa10;
const DLC_OPEN: u32 = 0x00af_28f0;
/// `__RTDynamicCast(ptr, 0, from, to, 0)` (LIBCMT) and the two type
/// descriptors the code casts between (`Option` to `UserOption`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_DESCRIPTOR_OPTION: u32 = 0x011a_0960;
const TYPE_DESCRIPTOR_USER_OPTION: u32 = 0x011a_0980;
/// Strings for the reset-to-defaults dialog and the downloads list item.
const SETTING_RESET_BUTTON_A: u32 = 0x011d_34f8;
const SETTING_RESET_BUTTON_B: u32 = 0x011d_3684;
const SETTING_RESET_TEXT: u32 = 0x011d_29dc;
const SETTING_SAVE_SELECTED: u32 = 0x011d_4930;

fn fade(e: &mut Engine, menu: Ptr, kind: u32, flag: u32) {
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    e.call(FADE, &args![menu, kind, flag, 0x8000_0000u32, minus_one]);
}

// Translated from 007ce7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::Close` (Ghidra name; cdecl, one byte argument): resets the
/// Pipboy colours when the start menu was opened over the Pipboy, then (when
/// a start menu exists and is not waiting for a control remap) closes the
/// dialog pause, and, unless flag 4 is set, deletes the list items of the
/// options list when remapping, restores the pause-menu mode, shows the
/// hidden tiles again, tells the menu tile to delete on fade (`delete_on_fade`)
/// and starts the fade-out; finally redraws the menu background when a
/// rendered menu is open.
pub fn start_menu_close(e: &mut Engine, delete_on_fade: u8) {
    if e.call(START_MENU_FLAG_1, &args![]).bool() {
        let rendered = e.call(CURRENT_RENDERED_MENU, &args![]).u32();
        if rendered != 0 {
            let rendered = e.call(CURRENT_RENDERED_MENU, &args![]).u32();
            let pipboy = e.call(GET_PIPBOY, &args![]).u32();
            if rendered == pipboy {
                let pipboy = e.call(GET_PIPBOY, &args![]).u32();
                e.call(PIPBOY_RESET_COLORS, &args![pipboy]);
            }
        }
    }
    let menu = start_menu_global(e);
    if menu.is_null() || start_menu_waiting_for_control_remap_button_press(e, menu) {
        return;
    }
    if e.call(IN_DIALOG, &args![]).bool() {
        e.call(SET_DIALOG_PAUSE, &args![0u32]);
    }
    let menu = start_menu_global(e);
    if !menu_flag(e, menu, 4) {
        let menu = start_menu_global(e);
        if menu_flag(e, menu, 0x1000) {
            e.call(
                LIST_BOX_FOR_EACH,
                &args![menu.byte_add(0x114), FN_007CE970, 0u32, 0x7fff_ffffu32],
            );
        }
        if e.call(START_MENU_FLAG_1, &args![]).bool() {
            e.call(SET_PAUSE_MENU_MODE, &args![0u32]);
        }
        let hidden = e.call(FIRST_WORD, &args![SCRIPT_LIST]).u32();
        e.with_stack(4, |e, position| {
            e.mem.set_u32(position.addr(), hidden);
            while e.mem.u32(position.addr()) != 0 {
                let slot = e
                    .call(POINTER_LIST_STEP_GLOBAL, &args![SCRIPT_LIST, position])
                    .u32();
                let tile = e.mem.u32(slot);
                e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 1u32]);
            }
        });
        let menu = start_menu_global(e);
        let menu_tile = e.mem.u32(menu.addr() + 4);
        e.call(
            TILE_SET_INT,
            &args![menu_tile, TRAIT_DELETE_ON_FADE, u32::from(delete_on_fade)],
        );
        e.call(MENU_START_FADE_OUT, &args![menu]);
        e.set_global(START_MENU, 0u32);
    }
    if e.call(IS_IN_RENDERED_MENU, &args![]).bool() {
        let main = e.global::<u32>(MAIN_OBJECT);
        e.call(RENDER_MENU_BACKGROUND, &args![main]);
    }
}

// Translated from 007ce8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::WaitingForControlRemapButtonPress` (Ghidra name): whether the
/// start menu is the top menu, in the controls-remap mode (flag `0x1000`) and
/// its options list has a selection (the list's `GetHighlightedItem`, vtable
/// slot +4, is selected).
pub fn start_menu_waiting_for_control_remap_button_press(e: &mut Engine, this: Ptr) -> bool {
    if !e.call(IS_TOP_MENU_ID, &args![START_MENU_CLASS]).bool() {
        return false;
    }
    if !menu_flag(e, this, 0x1000) {
        return false;
    }
    let list = this.byte_add(0x114);
    let highlighted = e.vcall(list.addr(), 4, &args![]).u32();
    e.call(LIST_BOX_IS_SELECTED, &args![list, highlighted])
        .bool()
}

// Translated from 007ce970 (decompiled, FalloutNV.exe 1.4.0.525)
/// A list-item delete callback (Ghidra labels it `_InternalDeleteHelper<>`
/// because several identical functions were folded): calls the virtual
/// destructor (slot 0, argument 1) of its second argument when that is not
/// null. Returns whatever the destructor returned.
pub fn fn_007ce970(e: &mut Engine, _unused: u32, item: u32) -> u32 {
    if item == 0 {
        return 0;
    }
    e.vcall(item, 0, &args![1u32]).u32()
}

// Translated from 007ce9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoClick` (Xbox PDB): `code` is the clicked tile's click id,
/// `tile` the tile. Code 5 chooses Back first. With flag `0x400` set
/// nothing else happens. Code 4 confirms the save/load list, resets the
/// controls defaults (in remap mode a confirmation message, else the
/// `ResetToDefaults` callback on every item of the options list) ; code
/// `0xf` refreshes the downloads page; code 1 opens settings; code 0
/// (unless flag `0x100000`) switches from the save-load view back to the
/// main list. A click on a tile at least 500 ms after the menu was created
/// then runs the choose function of the option the tile belongs to, or
/// selects/confirms a save entry or a downloads item, or steps a user
/// option (codes 100/101 left/right).
pub fn start_menu_do_click(e: &mut Engine, this: Ptr<StartMenu>, code: i32, tile: Ptr) {
    if code == 5 {
        e.call(CHOOSE_BACK, &args![]);
    }
    if menu_flag(e, this.cast(), 0x400) {
        return;
    }
    if code == 4 {
        let save_load = this.byte_add(0x174);
        if e.call(SAVE_LOAD_LIST_IS_ENABLED, &args![save_load]).bool() {
            e.call(MENU_FLAGS_SET, &args![this, 0x2000u32, 1u32]);
            e.call(CONFIRM_SAVE_LOAD, &args![this]);
            e.call(MENU_FLAGS_SET, &args![this, 0x2000u32, 0u32]);
        } else if menu_flag(e, this.cast(), 0x1000) {
            let button_a = e.call(SETTING_STRING, &args![SETTING_RESET_BUTTON_A]).u32();
            let button_b = e.call(SETTING_STRING, &args![SETTING_RESET_BUTTON_B]).u32();
            let text = e.call(OPTION_TEXT, &args![SETTING_RESET_TEXT]).u32();
            e.call(
                SHOW_MESSAGE,
                &args![
                    text,
                    0u32,
                    0u32,
                    MESSAGE_CALLBACK,
                    0u32,
                    0x12u32,
                    0.0f32,
                    0.0f32,
                    button_b,
                    button_a,
                    0u32
                ],
            );
        } else {
            e.call(
                LIST_BOX_FOR_EACH,
                &args![this.byte_add(0x114), FN_007CFDC0, 0u32, 0x7fff_ffffu32],
            );
        }
    } else if code == 0xf {
        let options = this.byte_add(0x114);
        if e.call(OPTIONS_LIST_HAS_SELECTION, &args![options]).bool() {
            e.call(
                LIST_BOX_FOR_EACH,
                &args![options, FN_007CE970, 0u32, 0x7fff_ffffu32],
            );
            e.call(MENU_REFRESH_A, &args![this]);
            fn_007d01b0(e, this);
        }
    } else if code == 1 {
        e.call(HIDE_OPTIONS_CONTAINER, &args![this]);
        start_menu_choose_settings(e);
    } else if code == 0 && !menu_flag(e, this.cast(), 0x10_0000) {
        e.call(HIDE_OPTIONS_CONTAINER, &args![this]);
        let pending = start_menu_global(e);
        if e.call(INT_LIST_IS_ENABLED, &args![pending.byte_add(0x144)])
            .bool()
        {
            fade(e, pending.cast(), 0xe, 0);
            e.call(INT_LIST_SET_ENABLED, &args![pending.byte_add(0x144), 0u32]);
        }
        fade(e, this.cast(), 1, 0);
        e.call(LIST_BOX_SET_ENABLED, &args![this.byte_add(0xb4), 0u32]);
        e.call(LIST_BOX_DESELECT_ALL, &args![this.byte_add(0xb4), 1u32]);
        e.call(
            SAVE_LOAD_LIST_SET_ENABLED,
            &args![this.byte_add(0x174), 1u32],
        );
        e.vcall(this.addr() + 0x174, 0, &args![0u32]);
        e.call(MENU_FLAGS_SET, &args![this, 0x4_0000u32, 0u32]);
        e.call(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO, &args![this]);
        e.call(
            SAVE_LOAD_LIST_SET_ENABLED,
            &args![this.byte_add(0x174), 0u32],
        );
        fade(e, this.cast(), 6, 0);
        e.call(LIST_BOX_SET_ENABLED, &args![this.byte_add(0xe4), 0u32]);
        fade(e, this.cast(), 7, 0);
        e.call(LIST_BOX_SET_ENABLED, &args![this.byte_add(0x84), 1u32]);
        e.call(LIST_BOX_DESELECT_ALL, &args![this.byte_add(0x84), 1u32]);
        fade(e, this.cast(), 5, 0);
        if menu_flag(e, this.cast(), 2) {
            e.call(MENU_REFRESH_B, &args![]);
            e.call(MENU_FLAGS_SET, &args![this, 2u32, 0u32]);
        }
    }
    if tile.is_null() {
        return;
    }
    let now = e.call(TICK_COUNT, &args![]).u32();
    if now.wrapping_sub(e.global::<u32>(TICK_CREATED)) <= 500 {
        return;
    }
    let lists = [
        this.addr() + 0x84,
        this.addr() + 0xb4,
        this.addr() + 0xe4,
        this.addr() + 0x114,
    ];
    for list in lists {
        let item = e.call(LIST_BOX_GET_ITEM, &args![list, tile]).u32();
        if item != 0 && e.mem.u32(item + 8) != 0 {
            let choose = e.mem.u32(item + 8);
            if choose != CHOOSE_007D0C50 {
                e.call(LIST_BOX_SELECT, &args![list, tile]);
            }
            if list != this.addr() + 0xe4 && choose != CHOOSE_BACK {
                e.call(LIST_BOX_SET_ENABLED, &args![list, 0u32]);
            }
            e.call(choose, &args![]);
            let now = e.call(TICK_COUNT, &args![]).u32();
            e.set_global(TICK_CREATED, now);
            return;
        }
    }
    let save = e
        .call(LIST_BOX_GET_ITEM, &args![this.byte_add(0x174), tile])
        .u32();
    e.set(this, StartMenu::pSaveGame, Ptr::new(save));
    let confirm = save != 0
        && (menu_flag(e, this.cast(), 8) || !e.call(SAVE_ENTRY_IS_CORRUPT, &args![save]).bool());
    if confirm {
        e.call(CONFIRM_SAVE_LOAD, &args![this]);
    } else if e
        .call(LIST_BOX_HAS_TILE, &args![this.byte_add(0x174), tile])
        .bool()
        && menu_flag(e, this.cast(), 8)
    {
        let count = e.call(LIST_BOX_COUNT, &args![this.byte_add(0x174)]).u32();
        if count > 1000 && e.call(LIST_BOX_COUNT, &args![this.byte_add(0x174)]).u32() != 0 {
            return;
        }
        let text = e.call(SETTING_STRING, &args![SETTING_SAVE_SELECTED]).u32();
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
        e.call(
            SAVE_LOAD_LIST_SET_SELECTION,
            &args![this.byte_add(0x174), tile],
        );
        let container = e.mem.u32(this.addr() + 0x40);
        e.call(TILE_SET_INT, &args![container, TRAIT_USER_0, 0u32]);
        let menu = start_menu_global(e);
        e.call(MENU_FLAGS_SET, &args![menu, 0x4000u32, 1u32]);
    } else if e
        .call(LIST_BOX_HAS_TILE, &args![this.byte_add(0x144), tile])
        .bool()
        && e.call(LIST_BOX_GET_ITEM, &args![this.byte_add(0x144), tile])
            .i32()
            == -1
    {
        let getter = e.call(DLC_GETTER, &args![]).u32();
        let step = e.call(DLC_STEP_A, &args![getter]).u32();
        // 007ceedd pushes `step` before the no-argument getter 00af2640; the
        // word is popped by 00af28f0 (RET 4, which never reads it).
        let instance = e.call(DLC_INSTANCE, &args![]).u32();
        let next = e.call(DLC_STEP_B, &args![instance]).u32();
        e.call(DLC_OPEN, &args![next, step]);
    }
    let parent = e.call(TILE_PARENT, &args![tile]).u32();
    let item = e
        .call(LIST_BOX_GET_ITEM, &args![this.byte_add(0x114), parent])
        .u32();
    let user_option = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                item,
                0u32,
                TYPE_DESCRIPTOR_OPTION,
                TYPE_DESCRIPTOR_USER_OPTION,
                0u32
            ],
        )
        .u32();
    if user_option == 0 {
        return;
    }
    let kind = e.mem.u32(user_option + 0x1c);
    let value = e.mem.u32(user_option + 0x14);
    let settings = e.mem.u32(user_option + 0x20);
    if code == 100 || code == 0x65 || kind == 0 || (kind == 1 && value < settings.wrapping_sub(1)) {
        let step: u32 = if code == 100 { u32::MAX } else { 1 };
        let moved = value.wrapping_add(settings).wrapping_add(step) % settings;
        e.mem.set_u32(user_option + 0x14, moved);
        e.set(this, StartMenu::pDirtyOption, Ptr::new(user_option));
        let menu_tile = e.mem.u32(this.addr() + 4);
        let brightness = tile_value(e, Ptr::new(menu_tile), TRAIT_BRIGHTNESS);
        set_tile_value(e, tile, TRAIT_BRIGHTNESS, brightness, 1);
    }
}

/// `Tile::GetChildByID(id)` (Ghidra name), `RET 4`; `Tile::IsVisible`
/// (Ghidra name); `Tile::PlayTileSound(id)` (Ghidra name), `RET 4`.
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
const TILE_PLAY_TILE_SOUND: u32 = 0x00a0_b110;
/// Sound id played for a gamepad click (`eClickSound` default, 0xFCB).
const SOUND_ID_CLICK: u32 = 0xfcb;
/// `ListBox<StartMenu::Option_P>::IsEnabled` (Ghidra name).
const LIST_BOX_IS_ENABLED: u32 = 0x007d_7580;
/// `007a1910` (on a list box): the item the list highlights (the
/// `Option` object, 0 when none).
const LIST_BOX_HIGHLIGHTED_ITEM: u32 = 0x007a_1910;
/// `007189c0(code)`: maps a gamepad button to the key id the remap list
/// stores; `00877720` (on the object at `011dea0c`) gives the input object;
/// `00a24fc0(input, key, device, code)` (`RET 0xC`) tries to bind it.
const GAMEPAD_BUTTON_TO_KEY: u32 = 0x0071_89c0;
const INPUT_OBJECT_GETTER: u32 = 0x0087_7720;
const BIND_CONTROL: u32 = 0x00a2_4fc0;
/// `Interface::IsGamepad`-like test (`004b71d0`): the interface manager's
/// input flag (the exe reads the byte at `011d8a84`).
const GAMEPAD_ACTIVE: u32 = 0x004b_71d0;
/// `InterfaceManager::PlayMenuSound(sound)` (Ghidra name), cdecl.
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
/// The remap-mode label table: `GameSetting *` per control mode, at
/// `011d5730`.
const CONTROL_MODE_LABELS: u32 = 0x011d_5730;
/// Settings for the "already bound" message and its title.
const SETTING_BINDING_TEXT: u32 = 0x011d_38b8;
const SETTING_BINDING_TITLE: u32 = 0x011d_3108;
/// The loading menu object (`011da0c0`) and its key handler (`0078aa90`).
const LOADING_MENU: u32 = 0x011d_a0c0;
const LOADING_MENU_HANDLER: u32 = 0x0078_aa90;
/// `StartMenu::FN_004BD510`: `(float, float)` returning a `float` in ST0
/// (`004bd510`), used to scale the slider step.
const SLIDER_STEP: u32 = 0x004b_d510;
/// The menu class `0x3e9` looked up before the binding message.
const BINDING_MENU_CLASS: u32 = 0x3e9;

// Translated from 007cefc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoGamepad` (Xbox PDB): handles gamepad button `button`
/// while the start menu is the top menu; returns whether it handled it.
/// Nothing is handled unless the start menu is the top menu; flag `0x400`,
/// or `0x8000` with `0x8`, swallow every button. A positive button first
/// goes to the loading menu (flag `0x80000`) or, in the remap mode (control
/// device 3 while waiting for a button), is bound as the control.
/// Otherwise: buttons 3 and 4 click the highlighted option's left/right
/// arrow (codes 101/100), 5 and 9 back out, 0xB and 0xC click the
/// downloads/load tiles, 0xF and 0x10 step a slider option.
pub fn start_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<StartMenu>,
    button: i32,
    _unused_1: u32,
) -> bool {
    if !e.call(IS_TOP_MENU_ID, &args![START_MENU_CLASS]).bool() {
        return false;
    }
    if menu_flag(e, this.cast(), 0x400) {
        return true;
    }
    if menu_flag(e, this.cast(), 0x8000) && menu_flag(e, this.cast(), 8) {
        return true;
    }
    let options = this.byte_add(0x114);
    if button > 0 {
        let loading = e.global::<u32>(LOADING_MENU);
        if menu_flag(e, this.cast(), 0x8_0000) && loading != 0 {
            return e.call(LOADING_MENU_HANDLER, &args![loading]).bool();
        }
        let mode = e.mem.u32(start_menu_global(e).addr() + 0x1c8);
        if mode == 3 && start_menu_waiting_for_control_remap_button_press(e, this.cast()) {
            let item = e.call(LIST_BOX_HIGHLIGHTED_ITEM, &args![options]).u32();
            let value = e.mem.u32(item + 0xc);
            let key = e.call(GAMEPAD_BUTTON_TO_KEY, &args![button]).u32();
            let key = key & 0xff;
            let main = e.global::<u32>(MAIN_OBJECT);
            let input = e.call(INPUT_OBJECT_GETTER, &args![main]).u32();
            if e.call(BIND_CONTROL, &args![input, value, 3u32, key]).bool() {
                if e.call(GAMEPAD_ACTIVE, &args![]).bool() {
                    let label_tile = e.mem.u32(this.addr() + 0x68);
                    e.call(TILE_SET_INT, &args![label_tile, TRAIT_VISIBLE, 0u32]);
                } else {
                    let setting = e
                        .mem
                        .u32(CONTROL_MODE_LABELS + 4 * control_mode(e, this.cast()));
                    let text = e.call(SETTING_STRING, &args![setting]).u32();
                    let label_tile = e.mem.u32(this.addr() + 0x68);
                    e.call(
                        TILE_SET_STRING,
                        &args![label_tile, TRAIT_STRING, text, 1u32],
                    );
                }
                e.call(LIST_BOX_SET_ENABLED, &args![options, 1u32]);
                e.call(LIST_BOX_DESELECT_ALL, &args![options, 1u32]);
                e.call(
                    LIST_BOX_FOR_EACH,
                    &args![options, FN_007CFE40, 0u32, 0x7fff_ffffu32],
                );
                e.call(MENU_FLAGS_SET, &args![this, 2u32, 1u32]);
                e.call(MENU_FLAGS_SET, &args![this, 0x400_0000u32, 1u32]);
                let tile = e.mem.u32(this.addr() + 0x3c);
                e.call(TILE_SET_INT, &args![tile, TRAIT_TARGET, 1u32]);
            } else if e
                .call(TILE_GET_MENU_BY_CLASS, &args![BINDING_MENU_CLASS])
                .u32()
                == 0
            {
                e.call(PLAY_MENU_SOUND, &args![0xau32]);
                let text = e.call(OPTION_TEXT, &args![SETTING_BINDING_TEXT]).u32();
                let title = e.call(SETTING_STRING, &args![SETTING_BINDING_TITLE]).u32();
                e.call(
                    SHOW_MESSAGE,
                    &args![title, 0u32, 0u32, 0u32, 1u32, 0x12u32, 0.0f32, 0.0f32, text, 0u32],
                );
            }
            return true;
        }
    }
    match button {
        3 | 4 => {
            if e.call(LIST_BOX_IS_ENABLED, &args![options]).bool()
                && e.vcall(options.addr(), 4, &args![]).u32() != 0
            {
                let id: u32 = if button != 4 { 101 } else { 100 };
                let highlighted = e.vcall(options.addr(), 4, &args![]).u32();
                let tile = e.call(TILE_GET_CHILD_BY_ID, &args![highlighted, id]).u32();
                if tile != 0
                    && e.call(TILE_IS_VISIBLE, &args![tile]).bool()
                    && e.call(TILE_IS_TRUE, &args![tile, TRAIT_TARGET]).bool()
                {
                    e.vcall(this.addr(), 0xc, &args![id, tile]);
                    e.call(TILE_PLAY_TILE_SOUND, &args![tile, SOUND_ID_CLICK]);
                    return true;
                }
                if id == 0x65 {
                    let item = e.call(LIST_BOX_HIGHLIGHTED_ITEM, &args![options]).u32();
                    if item != 0 {
                        let item = e.call(LIST_BOX_HIGHLIGHTED_ITEM, &args![options]).u32();
                        if e.mem.u32(item + 8) == CHOOSE_007D0C50 {
                            let highlighted = e.vcall(options.addr(), 4, &args![]).u32();
                            e.vcall(this.addr(), 0xc, &args![0xffff_ffffu32, highlighted]);
                            e.call(TILE_PLAY_TILE_SOUND, &args![tile, SOUND_ID_CLICK]);
                            return true;
                        }
                    }
                }
            }
            false
        }
        5 | 9 => {
            let tile = e.mem.u32(this.addr() + 0x4c);
            if !menu_flag(e, this.cast(), 1)
                && e.call(TILE_IS_TRUE, &args![tile, TRAIT_VISIBLE]).bool()
                && tile_value(e, Ptr::new(tile), TRAIT_ALPHA) != 0.0
            {
                e.call(MENU_REFRESH_C, &args![this]);
                let lazy = fn_007cb710(e);
                fn_007cf5c0(e, lazy);
                return true;
            }
            false
        }
        0xb => {
            let tile = e.mem.u32(this.addr() + 0x64);
            if e.call(TILE_IS_VISIBLE, &args![tile]).bool()
                && e.call(TILE_IS_TRUE, &args![tile, TRAIT_TARGET]).bool()
            {
                e.call(TILE_PLAY_TILE_SOUND, &args![tile, SOUND_ID_CLICK]);
                e.vcall(this.addr(), 0xc, &args![0xfu32, tile]);
                return true;
            }
            false
        }
        0xc => {
            let tile = e.mem.u32(this.addr() + 0x38);
            if e.call(TILE_IS_VISIBLE, &args![tile]).bool()
                && e.call(TILE_IS_TRUE, &args![tile, TRAIT_TARGET]).bool()
            {
                e.call(TILE_PLAY_TILE_SOUND, &args![tile, SOUND_ID_CLICK]);
                e.vcall(this.addr(), 0xc, &args![4u32, tile]);
                return true;
            }
            false
        }
        0xf | 0x10 => {
            if !e.call(LIST_BOX_IS_ENABLED, &args![options]).bool() {
                return false;
            }
            let item = e.call(LIST_BOX_HIGHLIGHTED_ITEM, &args![options]).u32();
            let user_option = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        item,
                        0u32,
                        TYPE_DESCRIPTOR_OPTION,
                        TYPE_DESCRIPTOR_USER_OPTION,
                        0u32
                    ],
                )
                .u32();
            if user_option == 0 || e.mem.u32(user_option + 0x1c) != 2 {
                return false;
            }
            let highlighted = e.vcall(options.addr(), 4, &args![]).u32();
            let slider = tile_value(e, Ptr::new(highlighted), TRAIT_USER_1);
            let scaled = e.call(SLIDER_STEP, &args![slider, 1.0f32]).f64();
            let mut delta = e.call(FLOAT_TO_INT, &args![scaled]).i32();
            let mut id: u32 = 0x65;
            if button == 0xf {
                delta = delta.wrapping_neg();
                id = 100;
            }
            let value = (e.mem.u32(user_option + 0x14) as i32).wrapping_add(delta);
            let count = e.mem.u32(user_option + 0x20);
            let value = if value < 0 {
                0
            } else if (value as u32) >= count {
                count.wrapping_sub(1)
            } else {
                value as u32
            };
            e.mem.set_u32(user_option + 0x14, value);
            e.set(this, StartMenu::pDirtyOption, Ptr::new(user_option));
            let highlighted = e.vcall(options.addr(), 4, &args![]).u32();
            let tile = e.call(TILE_GET_CHILD_BY_ID, &args![highlighted, id]).u32();
            e.call(TILE_PLAY_TILE_SOUND, &args![tile, SOUND_ID_CLICK]);
            true
        }
        _ => false,
    }
}

/// `Tile::GetFadeFinished(id)` (Ghidra name), `RET 4`.
const TILE_GET_FADE_FINISHED: u32 = 0x00a0_7fc0;
/// `Tile::GetX`-like position getters (`00a013d0`, `float` in ST0) and
/// `00a01440` (`float` in ST0), as called on a tile.
const TILE_POSITION_A: u32 = 0x00a0_13d0;
const TILE_POSITION_B: u32 = 0x00a0_1440;
/// The interface manager's width (`00747d00`, on the manager, returns an
/// `int`) and `004e4430(float)` (cdecl, returns a `float` in ST0).
const MANAGER_WIDTH: u32 = 0x0074_7d00;
const ROUND_FLOAT: u32 = 0x004e_4430;
/// `InterfaceManager::ClearMenuButton(button)` (Ghidra name), cdecl.
const CLEAR_MENU_BUTTON: u32 = 0x0071_7de0;
/// `ChooseBack` (`007d0e40`, other half of this unit).
const CHOOSE_BACK: u32 = 0x007d_0e40;
const CHOOSE_007D0C50: u32 = 0x007d_0c50;
/// Callbacks given by address to `ListBox::ForEach`.
const FN_007CE970: u32 = 0x007c_e970;
const FN_007CFDC0: u32 = 0x007c_fdc0;
const FN_007CFE40: u32 = 0x007c_fe40;
/// Tile accessor shared by several handlers: the word at `+0x28` of a tile
/// (`Tile::pParent`'s slot in the exe, `0045cd60`).
const TILE_PARENT: u32 = 0x0045_cd60;
/// `NiTPointerList`/`BSSimpleList` head pointer getter used on a form
/// (`00500940`) and the node iteration of `006815c0`/`00726070` (above).
const FORM_LIST_HEAD: u32 = 0x0050_0940;
/// `Tile::GetMenu`'s fade state check: the menu fade state getter above.
/// `TESForm` lookup by form id (`004839c0`, cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// Type descriptors the help option casts forms between (`01183028`,
/// `01186130`, `011863d4`).
const TYPE_DESCRIPTOR_FORM: u32 = 0x0118_3028;
const TYPE_DESCRIPTOR_FORM_DERIVED_A: u32 = 0x0118_6130;
const TYPE_DESCRIPTOR_FORM_DERIVED_B: u32 = 0x0118_63d4;
/// `Interface::IsMenuIDVisible(id, flag)` (Ghidra name), cdecl.
const IS_MENU_ID_VISIBLE: u32 = 0x0070_2680;
/// `TutorialMenu::Create(form, flag)` (Ghidra name), cdecl.
const TUTORIAL_MENU_CREATE: u32 = 0x007e_8890;
/// The interface manager's table of help entries (`manager + 0x4d4`).
const HELP_TABLE_OFFSET: u32 = 0x4d4;
/// `StartMenu::Fade`'s caller `007d48a0(bits, value)`, the menu's
/// "request" setter, and `PrepSaveLoad` (`007d3b60`).
const MENU_REQUEST: u32 = 0x007d_48a0;
const PREP_SAVE_LOAD: u32 = 0x007d_3b60;
/// `DisplayOption(option, flag)` (`007d4ce0`, `RET 8`, on the menu).
const DISPLAY_OPTION: u32 = 0x007d_4ce0;
/// Settings used by the choose functions.
const SETTING_CONTINUE: u32 = 0x011d_2be0;
const SETTING_LOAD: u32 = 0x011d_3b50;
const SETTING_BACK: u32 = 0x011d_3984;
const SETTING_DOWNLOADS: u32 = 0x011d_3ad4;
const DOWNLOAD_MODS_OBJECT: u32 = 0x011c_3f2c;
/// `ListBox<int>::AddItem(id, text, 0, 0)` (Ghidra name), `RET 0x10`.
const INT_LIST_ADD_ITEM: u32 = 0x0076_4430;
/// The data handler's list of mods (`0045dfc0`, returns the first node),
/// `TESFile::IsActive`-like test (`00464db0`).
const MOD_LIST_HEAD: u32 = 0x0045_dfc0;
const MOD_IS_ACTIVE: u32 = 0x0046_4db0;
/// Objects the idle handler polls: `011da0c4`, `011daac8`, `011dac60`.
const IDLE_GATE_OBJECT: u32 = 0x011d_a0c4;
const CHILD_MENU: u32 = 0x011d_aac8;
const FADE_DEADLINE: u32 = 0x011d_ac60;
/// Idle helpers: `007e6760` (done test of the child menu), `LoadingMenu`
/// calls (`00789820` update, `00458110` check, `0078a5a0` make list),
/// `007d5830`, `007d52a0`, `007d5af0` (`SaveLoadIdle`, Ghidra name),
/// `007a92a0`, `Interface::CloseMessageMenu` (`00704000`), `007d6d70`,
/// `LoadingMenu::ResumeBackgroundThread` (`0078d020`),
/// `CreditsMenu::Create` (`0075f2a0`), `00af2b10` (marketplace call).
const CHILD_MENU_DONE: u32 = 0x007e_6760;
const LOADING_MENU_UPDATE: u32 = 0x0078_9820;
const LOADING_MENU_CHECK: u32 = 0x0045_8110;
const MAKE_LOAD_SCREEN_LIST: u32 = 0x0078_a5a0;
const IDLE_STEP_A: u32 = 0x007d_5830;
const IDLE_STEP_B: u32 = 0x007d_52a0;
const SAVE_LOAD_IDLE: u32 = 0x007d_5af0;
const MESSAGE_MENU_ACTIVE: u32 = 0x007a_92a0;
const CLOSE_MESSAGE_MENU: u32 = 0x0070_4000;
const MENU_REFRESH_D: u32 = 0x007d_6d70;
const RESUME_BACKGROUND_THREAD: u32 = 0x0078_d020;
const CREDITS_MENU_CREATE: u32 = 0x0075_f2a0;
const DLC_CALL_WITH_CALLBACK: u32 = 0x00af_2b10;
const CHOOSE_007D4C50: u32 = 0x007d_4c50;
/// `Menu::GetClass` of the credits menu looked for (`0x417`) and the
/// binding message menu (`0x3e9`).
const CREDITS_MENU_CLASS: u32 = 0x417;
/// The 28 key labels tables of the controls list: `011d51d0` (names by
/// index), the four remap name tables (`011d52f0`, `011d5240`,
/// `011d5130`, and the labels `011d5730`).
const OPTION_LABELS: u32 = 0x011d_51d0;
const KEY_NAMES_KEYBOARD: u32 = 0x011d_52f0;
const KEY_NAMES_MOUSE: u32 = 0x011d_5240;
const KEY_NAMES_PAD: u32 = 0x011d_5130;
const SETTING_KEY_NUMBER: u32 = 0x011d_2e14;
/// Format strings `"%s"`, `"%s %d"`, `"%d"` and the empty string, and the
/// glow-button path format.
const FORMAT_STRING: u32 = 0x0101_9f08;
const FORMAT_STRING_NUMBER: u32 = 0x0104_700c;
const FORMAT_NUMBER: u32 = 0x0102_0764;
const TEXT_EMPTY: u32 = 0x0104_7008;
const FORMAT_GLOW_BUTTON: u32 = 0x0107_6ee4;
/// The glow-button names of the gamepad (`01076f38` ...), by key id 7..=17.
const GLOW_BUTTON_NAMES: [(u32, u32); 11] = [
    (7, 0x0107_6f18),
    (8, 0x0107_6f2c),
    (9, 0x0107_6f28),
    (10, 0x0102_1568),
    (11, 0x0107_6f24),
    (12, 0x0103_45e4),
    (13, 0x0107_6f20),
    (14, 0x0106_e79c),
    (15, 0x0107_6f30),
    (16, 0x0107_6f38),
    (17, 0x0107_6f34),
];
/// `BSStringT` helpers: construct (`004037b0`), destroy (`004037d0`) and
/// `Format(buffer, format, ...)` (`00406f60`, cdecl). The key lookup
/// (`00703be0`, `RET 8`) and `ListBox`-less count of control slots
/// (`00a236b0`, `RET 4`).
const STRING_NEW: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const KEY_LOOKUP: u32 = 0x0070_3be0;
const CONTROL_COUNT: u32 = 0x00a2_36b0;
/// Increment of the loop counter held in game memory (`007dc0a0`, cdecl,
/// the address of the integer).
const INCREMENT: u32 = 0x007d_c0a0;
/// `StartMenu::Option` button callbacks used by the controls list
/// (`007d0cb0`) and `Tile` string setter used with it (`00707e10`).
const CONTROL_CHOOSE: u32 = 0x007d_0cb0;
const SET_TILE_LABEL: u32 = 0x0070_7e10;

// Translated from 007cf5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the two words at the start of the object (the lazily created
/// 8-byte object of `007cb710` is reset this way).
pub fn fn_007cf5c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 007cf5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoGamepadUpEvent` (Xbox PDB): for button 10 (back): clears
/// flag `0x4000000` if set and handles the button; otherwise, when the
/// target tile's `eTarget` trait is true and 500 ms have passed since the
/// menu was created, clears the menu button, chooses Back and restarts the
/// timer. Returns whether it handled the button.
pub fn start_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr<StartMenu>,
    button: i32,
    _unused_1: u32,
) -> bool {
    if button != 10 {
        return false;
    }
    if menu_flag(e, this.cast(), 0x400_0000) {
        e.call(MENU_FLAGS_SET, &args![this, 0x400_0000u32, 0u32]);
        return true;
    }
    let target = e.mem.u32(this.addr() + 0x3c);
    if e.call(TILE_IS_TRUE, &args![target, TRAIT_TARGET]).bool() {
        let now = e.call(TICK_COUNT, &args![]).u32();
        if now.wrapping_sub(e.global::<u32>(TICK_CREATED)) > 500 {
            e.call(INTERFACE_MANAGER_GETTER, &args![]);
            e.call(CLEAR_MENU_BUTTON, &args![10u32]);
            e.call(CHOOSE_BACK, &args![]);
            let now = e.call(TICK_COUNT, &args![]).u32();
            e.set_global(TICK_CREATED, now);
            return true;
        }
    }
    false
}

// Translated from 007cf670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoDownClick` (Xbox PDB): pressing an arrow tile (codes 100
/// and 101) sets the tile's `eBrightness` trait to -1.
pub fn start_menu_do_down_click(e: &mut Engine, _this: Ptr<StartMenu>, code: i32, tile: Ptr) {
    if !tile.is_null() && (code == 100 || code == 0x65) {
        e.call(TILE_SET_INT, &args![tile, TRAIT_BRIGHTNESS, 0xffff_ffffu32]);
    }
}

// Translated from 007cf6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoStillDown` (Xbox PDB): while the slider tile (code 0x66)
/// of the highlighted user option is held, maps the cursor position along
/// the slider to the option's value (`(manager width - tile x) * eUser3 /
/// eWidth`, rounded and clamped to `0..iNumSettings`) and marks the option
/// dirty when the value changed.
pub fn start_menu_do_still_down(e: &mut Engine, this: Ptr<StartMenu>, code: i32, tile: Ptr) {
    if code != 0x66 {
        return;
    }
    let options = this.byte_add(0x114);
    let item = e.call(LIST_BOX_HIGHLIGHTED_ITEM, &args![options]).u32();
    let user_option = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                item,
                0u32,
                TYPE_DESCRIPTOR_OPTION,
                TYPE_DESCRIPTOR_USER_OPTION,
                0u32
            ],
        )
        .u32();
    if user_option == 0 {
        return;
    }
    let parent = e.call(TILE_PARENT, &args![tile]).u32();
    let highlighted = e.vcall(options.addr(), 4, &args![]).u32();
    if parent != highlighted {
        return;
    }
    let old = e.mem.u32(user_option + 0x14);
    let parent = e.call(TILE_PARENT, &args![tile]).u32();
    let scale = f64::from(tile_value(e, Ptr::new(parent), TRAIT_USER_3));
    let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
    let width = e.call(MANAGER_WIDTH, &args![manager]).i32();
    let position = e.call(TILE_POSITION_A, &args![tile]).f32();
    let scaled = (f64::from(width) - f64::from(position)) * scale;
    let tile_width = tile_value(e, tile, TRAIT_WIDTH);
    let ratio = (scaled / f64::from(tile_width)) as f32;
    let rounded = e.call(ROUND_FLOAT, &args![ratio]).f64();
    let value = e.call(FLOAT_TO_INT, &args![rounded]).i32();
    let count = e.mem.u32(user_option + 0x20);
    let value = if value < 0 {
        0
    } else if (value as u32) > count.wrapping_sub(1) {
        count.wrapping_sub(1)
    } else {
        value as u32
    };
    e.mem.set_u32(user_option + 0x14, value);
    if old != value {
        e.set(this, StartMenu::pDirtyOption, Ptr::new(user_option));
    }
}

// Translated from 007cf7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoEnter` (Xbox PDB): the mouse entered `tile`. Highlights the
/// options-list entry of the tile's parent, refreshes the save-game info
/// when the tile is in the save-load list (always with the gamepad,
/// otherwise when the cursor row is at or above the manager's row count),
/// and shows the brightness option's tile when it is the list's highlight.
pub fn start_menu_do_enter(e: &mut Engine, this: Ptr<StartMenu>, _unused_1: u32, tile: Ptr) {
    let options = this.byte_add(0x114);
    if !tile.is_null() {
        let parent = e.call(TILE_PARENT, &args![tile]).u32();
        if parent != 0 {
            let parent = e.call(TILE_PARENT, &args![tile]).u32();
            if e.call(LIST_BOX_HAS_TILE, &args![options, parent]).bool() {
                let parent = e.call(TILE_PARENT, &args![tile]).u32();
                e.vcall(options.addr(), 0, &args![parent]);
            }
        }
    }
    if e.call(LIST_BOX_HAS_TILE, &args![this.byte_add(0x174), tile])
        .bool()
    {
        let update = if e.call(GAMEPAD_ACTIVE, &args![]).bool() {
            true
        } else {
            let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
            let rows = e.call(MANAGER_ROW_COUNT, &args![manager]).i32();
            let list_tile = e.mem.u32(this.addr() + 0x40);
            let position = e.call(TILE_POSITION_B, &args![list_tile]).f64();
            position <= f64::from(rows)
        };
        if update {
            e.call(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO, &args![this]);
        }
    }
    if e.call(LIST_BOX_IS_ENABLED, &args![options]).bool() {
        let highlighted = e.call(LIST_BOX_HIGHLIGHTED_ITEM, &args![options]).u32();
        if highlighted == e.get(this, StartMenu::pBrightnessOption).addr() {
            let tile = e.mem.u32(this.addr() + 0x6c);
            e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 1u32]);
        }
    }
}

// Translated from 007cf8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoLeave` (Xbox PDB): the mouse left `tile`. For the arrow
/// tiles (codes 100/101) copies the menu tile's `eBrightness` to the tile;
/// refreshes the save-game info for a save-load tile (with the keyboard);
/// hides the brightness tile unless it was an arrow.
pub fn start_menu_do_leave(e: &mut Engine, this: Ptr<StartMenu>, code: i32, tile: Ptr) {
    let arrow = code == 100 || code == 0x65;
    if !tile.is_null() && arrow {
        let menu_tile = e.mem.u32(this.addr() + 4);
        let brightness = tile_value(e, Ptr::new(menu_tile), TRAIT_BRIGHTNESS);
        set_tile_value(e, tile, TRAIT_BRIGHTNESS, brightness, 1);
    }
    if !e.call(GAMEPAD_ACTIVE, &args![]).bool()
        && e.call(LIST_BOX_HAS_TILE, &args![this.byte_add(0x174), tile])
            .bool()
    {
        e.call(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO, &args![this]);
    }
    let brightness_tile = e.mem.u32(this.addr() + 0x6c);
    if !arrow && brightness_tile != 0 {
        e.call(TILE_SET_INT, &args![brightness_tile, TRAIT_VISIBLE, 0u32]);
    }
}

// Translated from 007cf9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoIdle` (Xbox PDB): the per-frame update. Does nothing while
/// the byte at `+0x49` of the object at `011da0c4` is 0. Otherwise ticks and
/// frees the child menu, updates the loading menu, runs the save-load idle
/// steps, starts the credits when the menu is fully faded (flag `0x10000`),
/// closes the confirmation message after its deadline, applies a pending
/// option change and opens the marketplace when the message menu has faded
/// in (flag `0x800000`).
pub fn start_menu_do_idle(e: &mut Engine, this: Ptr<StartMenu>) {
    let gate = e.global::<u32>(IDLE_GATE_OBJECT);
    if fn_007cfd00(e, Ptr::new(gate)) == 0 {
        return;
    }
    let child = e.global::<u32>(CHILD_MENU);
    if child != 0 {
        e.vcall(child, 0x2c, &args![]);
        let child = e.global::<u32>(CHILD_MENU);
        if e.call(CHILD_MENU_DONE, &args![child]).bool() {
            let child = e.global::<u32>(CHILD_MENU);
            if child != 0 {
                e.vcall(child, 0, &args![1u32]);
            }
            e.set_global(CHILD_MENU, 0u32);
        }
    }
    let loading = e.global::<u32>(LOADING_MENU);
    if loading != 0 {
        e.call(LOADING_MENU_UPDATE, &args![loading]);
        if menu_flag(e, this.cast(), 0x8_0000)
            && e.call(LOADING_MENU_CHECK, &args![loading, 0u32]).bool()
        {
            e.call(MAKE_LOAD_SCREEN_LIST, &args![loading]);
            e.call(MENU_FLAGS_SET, &args![this, 0x8_0000u32, 0u32]);
            e.call(MENU_FLAGS_SET, &args![this, 0x10u32, 1u32]);
        }
    }
    e.call(IDLE_STEP_A, &args![this]);
    if e.call(IDLE_STEP_B, &args![this]).bool() {
        return;
    }
    if menu_flag(e, this.cast(), 0x1_0000) {
        let tile = e.mem.u32(this.addr() + 0x34);
        if e.call(TILE_GET_FADE_FINISHED, &args![tile, TRAIT_ALPHA])
            .bool()
        {
            let alpha = tile_value(e, Ptr::new(tile), TRAIT_ALPHA);
            if alpha == 0.0
                && e.call(TILE_GET_MENU_BY_CLASS, &args![CREDITS_MENU_CLASS])
                    .u32()
                    == 0
            {
                e.call(RESUME_BACKGROUND_THREAD, &args![]);
                e.call(CREDITS_MENU_CREATE, &args![]);
            }
        }
        return;
    }
    let idle_handled = e.call(SAVE_LOAD_IDLE, &args![this]).bool();
    if !idle_handled && menu_flag(e, this.cast(), 0xc000) {
        if menu_flag(e, this.cast(), 0x4000) {
            let message = e
                .call(TILE_GET_MENU_BY_CLASS, &args![BINDING_MENU_CLASS])
                .u32();
            let mut shown = false;
            if message != 0 {
                let menu = e.call(TILE_GET_MENU, &args![message]).u32();
                if e.call(MENU_FADE_STATE, &args![menu]).u32() == 1 {
                    e.call(MENU_FLAGS_SET, &args![this, 0x4000u32, 0u32]);
                    e.call(MENU_FLAGS_SET, &args![this, 0x8000u32, 1u32]);
                    let now = e.call(TICK_COUNT, &args![]).u32();
                    e.set_global(FADE_DEADLINE, now.wrapping_add(3000));
                    e.call(MENU_REFRESH_D, &args![]);
                    shown = true;
                }
            }
            if !shown && e.call(MESSAGE_MENU_ACTIVE, &args![]).u32() == 0 {
                e.call(MENU_REFRESH_D, &args![]);
                e.call(MENU_FLAGS_SET, &args![this, 0x4000u32, 0u32]);
                e.call(MENU_FLAGS_SET, &args![this, 2u32, 0u32]);
            }
        } else {
            let now = e.call(TICK_COUNT, &args![]).u32();
            if now >= e.global::<u32>(FADE_DEADLINE) {
                e.call(CLOSE_MESSAGE_MENU, &args![]);
                e.call(MENU_FLAGS_SET, &args![this, 0x8000u32, 0u32]);
                e.call(MENU_FLAGS_SET, &args![this, 2u32, 0u32]);
            }
        }
    }
    let dirty = e.get(this, StartMenu::pDirtyOption);
    if !dirty.is_null() {
        e.call(DISPLAY_OPTION, &args![this, dirty, 0u32]);
        let change = e.mem.u32(dirty.addr() + 0x2c);
        if change != 0 {
            e.call(change, &args![dirty]);
            e.call(MENU_FLAGS_SET, &args![this, 2u32, 1u32]);
        }
        e.set(this, StartMenu::pDirtyOption, Ptr::NULL);
    }
    if menu_flag(e, this.cast(), 0x80_0000) {
        let message = e
            .call(TILE_GET_MENU_BY_CLASS, &args![BINDING_MENU_CLASS])
            .u32();
        let menu = if message == 0 {
            0
        } else {
            e.call(TILE_GET_MENU, &args![message]).u32()
        };
        if menu != 0 && e.call(MENU_FADE_STATE, &args![menu]).u32() == 1 {
            let owner = start_menu_global(e);
            e.call(MENU_FLAGS_SET, &args![owner, 0x80_0000u32, 0u32]);
            let instance = e.call(DLC_INSTANCE, &args![]).u32();
            let next = e.call(DLC_STEP_B, &args![instance]).u32();
            e.call(DLC_CALL_WITH_CALLBACK, &args![next, 0u32, CHOOSE_007D4C50]);
        }
    }
}

// Translated from 007cfd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x49` of the object.
pub fn fn_007cfd00(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x49)
}

// Translated from 007cfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::UpdateCanLoadStatus` (Xbox PDB): sets the menu's enabled
/// trait (`+0x1d0`) to 1 on the continue (`+0x1c4`) and load (`+0x1c0`)
/// option tiles.
pub fn start_menu_update_can_load_status(e: &mut Engine, this: Ptr<StartMenu>) {
    let enabled_trait = e.get(this, StartMenu::ENABLED_TRAIT);
    for offset in [0x1c4u32, 0x1c0] {
        let tile = e.mem.u32(this.addr() + offset);
        e.call(TILE_SET_INT, &args![tile, enabled_trait, 1u32]);
    }
}

// Translated from 007cfd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DoKey` (Xbox PDB): with flag `0x80000` and a loading menu
/// present, whether the loading menu's handler (`0078aa90`) accepts the key.
pub fn start_menu_do_key(e: &mut Engine, this: Ptr<StartMenu>, _unused_1: u32) -> bool {
    let loading = e.global::<u32>(LOADING_MENU);
    menu_flag(e, this.cast(), 0x8_0000)
        && loading != 0
        && e.call(LOADING_MENU_HANDLER, &args![loading]).bool()
}

// Translated from 007cfdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::ResetToDefaults` (Ghidra name; cdecl, called by
/// `ListBox::ForEach` with the caller's argument and the item): when the
/// item is a `UserOption` (`__RTDynamicCast`) whose value differs from its
/// default (+0x18), sets the value to the default, redisplays it and runs the
/// change function (+0x2C) if there is one.
pub fn start_menu_reset_to_defaults(e: &mut Engine, argument: u32, item: Ptr) {
    let user_option = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                item,
                0u32,
                TYPE_DESCRIPTOR_OPTION,
                TYPE_DESCRIPTOR_USER_OPTION,
                0u32
            ],
        )
        .u32();
    if user_option == 0 {
        return;
    }
    let value = e.mem.u32(user_option + 0x14);
    let default = e.mem.u32(user_option + 0x18);
    if value == default {
        return;
    }
    e.mem.set_u32(user_option + 0x14, default);
    let menu = e.global::<u32>(START_MENU);
    e.call(DISPLAY_OPTION, &args![menu, user_option, argument]);
    let change = e.mem.u32(user_option + 0x2c);
    if change != 0 {
        e.call(change, &args![user_option]);
        let menu = e.global::<u32>(START_MENU);
        e.call(MENU_FLAGS_SET, &args![menu, 2u32, 1u32]);
    }
}

/// `StartMenu::pMe + 0x1c8`: the controls-list device (PC only, the Xbox
/// PDB has `ENABLED_TRAIT` here): 0 keyboard, 1 mouse, 2 (third device),
/// 3 gamepad, from the way the code indexes the name tables.
fn control_mode(e: &Engine, menu: Ptr) -> u32 {
    e.mem.u32(menu.addr() + 0x1c8)
}

fn format_key_text(e: &mut Engine, text: Ptr, format: u32, argument: u32) {
    e.call(STRING_FORMAT, &args![text, format, argument]);
}

// Translated from 007cfe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cfe40 - the C++ exception frame (FS:[0]) is not translated; the temporary string is not destroyed if a callee throws
/// The controls-list item callback (given to `ListBox::ForEach`, cdecl with
/// the item's tile and the item): shows the name of the key bound to the
/// control. Looks the bound key up (`00703be0`) for the item's id (+0xC)
/// and the device (`+0x1c8` of the start menu); builds the label from the
/// keyboard, mouse, third-device or gamepad name tables (the gamepad
/// glyph path `Interface\Shared\Button\glow_general_button_%s.dds` sets
/// `eUser0` to 2), `"%d"` of the key id when there is no name, an empty
/// string for the unbound key 0xFF, and stores it in the tile's `eUser1`
/// string.
pub fn fn_007cfe40(e: &mut Engine, tile: Ptr, item: Ptr) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    // The exe's frame (`SUB ESP,0x14` under the exception record, then `ESI` and the record
    // link): the text at `EBP-0x14` (+0x14).
    e.with_stack(0x28, |e, frame| {
        let text = Ptr::new(frame.addr() + 0x14);
        e.call(STRING_NEW, &args![text]);
        e.call(TILE_SET_INT, &args![tile, TRAIT_USER_0, 1u32]);
        let mode = control_mode(e, menu);
        let value = e.mem.u32(item.addr() + 0xc);
        let main = e.global::<u32>(MAIN_OBJECT);
        let input = e.call(INPUT_OBJECT_GETTER, &args![main]).u32();
        let key = e.call(KEY_LOOKUP, &args![input, value, mode]).u8();
        let key_word = u32::from(key);
        let menu = start_menu_global(e);
        let table_name = |e: &mut Engine, table: u32| -> Option<u32> {
            let setting = e.mem.u32(table + 4 * key_word);
            if setting == 0 {
                None
            } else {
                Some(e.call(SETTING_STRING, &args![setting]).u32())
            }
        };
        if key == 0xff {
            e.call(STRING_SET, &args![text, TEXT_EMPTY, 0u32]);
        } else if mode == 0 && key_word < 0xee && e.mem.u32(KEY_NAMES_KEYBOARD + 4 * key_word) != 0
        {
            let name = table_name(e, KEY_NAMES_KEYBOARD).unwrap_or(0);
            format_key_text(e, text, FORMAT_STRING, name);
        } else if control_mode(e, menu) == 1
            && key_word < 9
            && e.mem.u32(KEY_NAMES_MOUSE + 4 * key_word) != 0
        {
            let name = table_name(e, KEY_NAMES_MOUSE).unwrap_or(0);
            format_key_text(e, text, FORMAT_STRING, name);
        } else if control_mode(e, menu) == 2 {
            let input = e.call(INPUT_OBJECT_GETTER, &args![main]).u32();
            let count = e.call(CONTROL_COUNT, &args![input, 0u32]).i32();
            if (key_word as i32) < count {
                let name = e.call(SETTING_STRING, &args![SETTING_KEY_NUMBER]).u32();
                e.call(
                    STRING_FORMAT,
                    &args![text, FORMAT_STRING_NUMBER, name, key_word + 1],
                );
            } else if (0x20..0x28).contains(&key_word)
                && e.mem.u32(KEY_NAMES_PAD + 4 * key_word) != 0
            {
                let name = table_name(e, KEY_NAMES_PAD).unwrap_or(0);
                format_key_text(e, text, FORMAT_STRING, name);
            } else {
                e.call(STRING_SET, &args![text, TEXT_EMPTY, 0u32]);
            }
        } else if control_mode(e, menu) == 3 {
            match GLOW_BUTTON_NAMES.iter().find(|(id, _)| *id == key_word) {
                Some((_, name)) => {
                    format_key_text(e, text, FORMAT_GLOW_BUTTON, *name);
                    e.call(TILE_SET_INT, &args![tile, TRAIT_USER_0, 2u32]);
                }
                None => format_key_text(e, text, FORMAT_NUMBER, key_word),
            }
        } else {
            format_key_text(e, text, FORMAT_NUMBER, key_word);
        }
        let characters = e.call(FIRST_WORD, &args![text]).u32();
        e.call(
            TILE_SET_STRING,
            &args![tile, TRAIT_USER_1, characters, 1u32],
        );
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 007d01b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007d01b0 - the C++ exception frame (FS:[0]) and the /GS cookie are not translated; the 0x10-byte option block is not freed if its constructor throws
/// Rebuilds the controls list: empties the options list (virtual slot
/// +0x1C), enables it, adds an `Option` (choose function `007d0cb0`, modes
/// = the control index) for each of the 28 controls that the current device
/// offers (all of them for the gamepad's own list; otherwise those
/// `007d03a0` accepts), runs the key-label callback `007cfe40` on every
/// item and, without the gamepad, shows the device label on the
/// device tile (`+0x68`).
pub fn fn_007d01b0(e: &mut Engine, this: Ptr<StartMenu>) {
    let options = this.byte_add(0x114);
    e.vcall(options.addr(), 0x1c, &args![]);
    e.call(LIST_BOX_SET_ENABLED, &args![options, 1u32]);
    e.with_stack(4, |e, counter| {
        e.mem.set_u32(counter.addr(), 0);
        while (e.mem.u32(counter.addr()) as i32) < 0x1c {
            let index = e.mem.u32(counter.addr());
            let mode = control_mode(e, this.cast());
            let wanted = if mode == 3 {
                matches!(
                    index,
                    0xe | 5 | 4 | 7 | 8 | 0xc | 0xd | 0xf | 6 | 0x10 | 0x1b
                )
            } else {
                fn_007d03a0(e, index)
            };
            if wanted {
                let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
                let option = if block == 0 {
                    0
                } else {
                    let setting = e.mem.u32(OPTION_LABELS + 4 * index);
                    let text = e.call(SETTING_STRING, &args![setting]).u32();
                    fn_007ce620(e, Ptr::new(block), text, CONTROL_CHOOSE, index).addr()
                };
                let label = e.mem.u32(option + 4);
                let item = e
                    .call(
                        LIST_BOX_ADD_ITEM,
                        &args![options, option, label, 0u32, TEXT_ITEM_TEMPLATE],
                    )
                    .u32();
                let setting = e.mem.u32(OPTION_LABELS + 4 * index);
                let text = e.call(SETTING_STRING, &args![setting]).u32();
                e.call(SET_TILE_LABEL, &args![item, text]);
            }
            e.call(INCREMENT, &args![counter]);
        }
    });
    e.call(
        LIST_BOX_FOR_EACH,
        &args![options, FN_007CFE40, 0u32, 0x7fff_ffffu32],
    );
    if !e.call(GAMEPAD_ACTIVE, &args![]).bool() {
        let mode = control_mode(e, this.cast());
        let setting = e.mem.u32(CONTROL_MODE_LABELS + 4 * mode);
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = e.mem.u32(this.addr() + 0x68);
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
        e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 1u32]);
    }
}

// Translated from 007d03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether control index `index` is one of the 24 the keyboard, mouse and
/// third-device lists show: `0..=0xf`, `0x10`, `0x12`, `0x19`, `0x1a` and
/// `0x1b`.
pub fn fn_007d03a0(_e: &mut Engine, index: u32) -> bool {
    matches!(index, 0..=0x10 | 0x12 | 0x19 | 0x1a | 0x1b)
}

fn request(e: &mut Engine, menu: Ptr, bits: u32, value: u32) {
    e.call(MENU_REQUEST, &args![menu, bits, value]);
}

fn set_label(e: &mut Engine, menu: Ptr, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let tile = e.mem.u32(menu.addr() + 0x48);
    e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
}

// Translated from 007d0440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function (the first option, `iModes` 3): with the XUI up, sets the
/// label tile (`+0x48`) text and requests `0x100`; otherwise closes the
/// start menu.
pub fn fn_007d0440(e: &mut Engine) {
    if e.call(XUI_IS_UP, &args![]).bool() {
        let menu = start_menu_global(e);
        set_label(e, menu, SETTING_CONTINUE);
        let menu = start_menu_global(e);
        request(e, menu, 0x100, 0);
    } else {
        start_menu_close(e, 1);
    }
}

// Translated from 007d0490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: sets the label tile text and requests `0x80`.
pub fn fn_007d0490(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    set_label(e, menu, SETTING_LOAD);
    let menu = start_menu_global(e);
    request(e, menu, 0x80, 0);
}

// Translated from 007d04d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function (credits): sets flag `0x10000`, then fades with the
/// three `Fade` kinds 3 (alpha), 0 and 9.
pub fn fn_007d04d0(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    e.call(MENU_FLAGS_SET, &args![menu, 0x1_0000u32, 1u32]);
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    let menu = start_menu_global(e);
    e.call(FADE, &args![menu, 3u32, 0u32, 0xfa9u32, minus_one]);
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    let menu = start_menu_global(e);
    e.call(FADE, &args![menu, 0u32, 0u32, 0x8000_0000u32, minus_one]);
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    let menu = start_menu_global(e);
    e.call(FADE, &args![menu, 9u32, 0u32, 0xfa9u32, minus_one]);
}

// Translated from 007d0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function (downloads): rebuilds the downloads list (empties it, adds
/// a placeholder item with id -1 and one item per active mod of the data
/// handler, labelled with the mod's name at `+0x418`), enables it and fades
/// the downloads and list pages in.
pub fn fn_007d0550(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    let handler = e.global::<u32>(DOWNLOAD_MODS_OBJECT);
    let mut node = e.call(MOD_LIST_HEAD, &args![handler]).u32();
    let downloads = menu.byte_add(0x144);
    e.vcall(downloads.addr(), 0x1c, &args![]);
    let text = e.call(SETTING_STRING, &args![SETTING_DOWNLOADS]).u32();
    e.call(
        INT_LIST_ADD_ITEM,
        &args![downloads, 0xffff_ffffu32, text, 0u32, 0u32],
    );
    while node != 0 {
        let slot = node_item(e, node);
        let module = e.mem.u32(slot.addr());
        if module != 0 && e.call(MOD_IS_ACTIVE, &args![module]).bool() {
            let name_slot = fn_007d0660(e, Ptr::new(module));
            let name = e.call(FIRST_WORD, &args![name_slot]).u32();
            e.call(INT_LIST_ADD_ITEM, &args![downloads, 0u32, name, 0u32, 0u32]);
        }
        node = node_next(e, node);
    }
    e.call(INT_LIST_SET_ENABLED, &args![downloads, 1u32]);
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    e.call(FADE, &args![menu, 0xeu32, 1u32, 0x8000_0000u32, minus_one]);
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    e.call(FADE, &args![menu, 5u32, 1u32, 0x8000_0000u32, minus_one]);
}

// Translated from 007d0660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the field at `+0x418` of the object (a mod's name).
pub fn fn_007d0660(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x418).cast()
}

// Translated from 007d0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function (save): hides the tile at `+0x64`, clears flag 8 (load
/// mode) and prepares the save/load page (`PrepSaveLoad`).
pub fn fn_007d0680(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    let tile = e.mem.u32(menu.addr() + 0x64);
    e.call(TILE_SET_INT, &args![tile, TRAIT_USER_0, 0u32]);
    e.call(MENU_FLAGS_SET, &args![menu, 8u32, 0u32]);
    e.call(PREP_SAVE_LOAD, &args![menu]);
}

// Translated from 007d06c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function (load): like `007d0680` but sets flag 8 (load mode).
pub fn fn_007d06c0(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    let tile = e.mem.u32(menu.addr() + 0x64);
    e.call(TILE_SET_INT, &args![tile, TRAIT_USER_0, 0u32]);
    e.call(MENU_FLAGS_SET, &args![menu, 8u32, 1u32]);
    e.call(PREP_SAVE_LOAD, &args![menu]);
}

// Translated from 007d0700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::ChooseSettings` (Ghidra name): enables the settings list,
/// fades kind 1 in, deselects all in it and fades kind 5 in.
pub fn start_menu_choose_settings(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    e.call(LIST_BOX_SET_ENABLED, &args![menu.byte_add(0xb4), 1u32]);
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    let menu = start_menu_global(e);
    e.call(FADE, &args![menu, 1u32, 1u32, 0x8000_0000u32, minus_one]);
    let menu = start_menu_global(e);
    e.call(LIST_BOX_DESELECT_ALL, &args![menu.byte_add(0xb4), 1u32]);
    let minus_one = e.global::<f32>(FLOAT_MINUS_ONE);
    let menu = start_menu_global(e);
    e.call(FADE, &args![menu, 5u32, 1u32, 0x8000_0000u32, minus_one]);
}

// Translated from 007d0770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::ChooseHelp` (Ghidra name): shows the help for the menu under
/// the start menu. Looks through the interface manager's enter stack (up to
/// ten entries, stopping at the start menu itself) for a menu with a tutorial
/// form (`TESForm` id `0x168 + index` cast to the first type, whose companion
/// `0x163`/`0x165` form lists it) and opens the tutorial menu for it
/// (null when none was found).
pub fn start_menu_choose_help(e: &mut Engine) {
    let menu = start_menu_global(e);
    let tile = e.mem.u32(menu.addr() + 0x50);
    e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 1u32]);
    let mut found = 0u32;
    let mut depth = 0u32;
    while found == 0 && depth < 10 {
        let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
        let entry = e.call(GET_ENTER_STACK, &args![manager, depth]).u32();
        if entry == e.call(START_MENU_CLASS_GETTER, &args![]).u32() {
            break;
        }
        let mut index = 0x29;
        if entry == 1 {
            for class in [0x3ebu32, 0x40b, 0x3ea, 0x3ff] {
                if e.call(IS_MENU_ID_VISIBLE, &args![class, 0u32]).bool() {
                    let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
                    index = fn_007d09c0(e, Ptr::new(manager + HELP_TABLE_OFFSET), class);
                    break;
                }
            }
        } else {
            let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
            index = fn_007d09c0(e, Ptr::new(manager + HELP_TABLE_OFFSET), entry);
        }
        if index < 0x29 {
            let form = e.call(LOOKUP_FORM, &args![index + 0x168]).u32();
            let candidate = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TYPE_DESCRIPTOR_FORM,
                        TYPE_DESCRIPTOR_FORM_DERIVED_A,
                        0u32
                    ],
                )
                .u32();
            if candidate != 0 {
                let gamepad = u32::from(e.call(GAMEPAD_ACTIVE, &args![]).bool());
                let companion_id = 0x163 + gamepad * 2;
                let companion_form = e.call(LOOKUP_FORM, &args![companion_id]).u32();
                let companion = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![
                            companion_form,
                            0u32,
                            TYPE_DESCRIPTOR_FORM,
                            TYPE_DESCRIPTOR_FORM_DERIVED_B,
                            0u32
                        ],
                    )
                    .u32();
                if companion != 0 {
                    let mut listed = false;
                    let mut node = e.call(FORM_LIST_HEAD, &args![companion]).u32();
                    while node != 0 {
                        let slot = node_item(e, node);
                        if e.mem.u32(slot.addr()) == 0 {
                            break;
                        }
                        let slot = node_item(e, node);
                        if e.mem.u32(slot.addr()) == candidate {
                            listed = true;
                            break;
                        }
                        node = node_next(e, node);
                    }
                    if !listed {
                        depth += 1;
                        continue;
                    }
                }
            }
            found = candidate;
        }
        depth += 1;
    }
    e.call(TUTORIAL_MENU_CREATE, &args![found, 1u32]);
}

// Translated from 007d09c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index (0..=0x29) of the help table entry (`table[i] >> 2 & 0x3f` plus
/// `0x3e9` is the menu class) for menu class `class`; 0x29 when none.
pub fn fn_007d09c0(e: &mut Engine, table: Ptr, class: u32) -> u32 {
    let mut index = 0u32;
    while (index as i32) < 0x29
        && ((e.mem.u32(table.addr() + 4 * index) >> 2) & 0x3f) + 0x3e9 != class
    {
        index += 1;
    }
    index
}

// Translated from 007d0a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: sets the label tile text and requests `0x2000` (mode 3)
/// when the start menu has flag 1, else `0x1000` (mode 1).
pub fn fn_007d0a10(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    set_label(e, menu, SETTING_BACK);
    let menu = start_menu_global(e);
    if e.call(START_MENU_FLAG_1, &args![]).bool() {
        request(e, menu, 0x2000, 3);
    } else {
        request(e, menu, 0x1000, 1);
    }
}

// Translated from 007d0a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007d0a70 - the /GS stack cookie check is not translated
/// `StartMenu::ChooseMainMenu` (Ghidra name): deletes the start menu if it
/// exists, hides the menus and support menus, kills every sound, clears the
/// media, resets the save-load state (the object `00a25920` returns gets its
/// `+0x88` byte cleared and `+0x87` set), marks `011daacd`, sets the
/// byte at `+2` of the main object, starts the main-title music
/// (`Data\Music\Special\MainTitle.mp3`) and requests the main menu flag.
pub fn start_menu_choose_main_menu(e: &mut Engine) {
    let menu = start_menu_global(e);
    if !menu.is_null() {
        e.vcall(menu.addr(), 0, &args![1u32]);
    }
    e.call(HIDE_MENUS, &args![]);
    e.call(CLOSE_ALL_SUPPORT_MENUS, &args![0u32]);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(KILL_ALL_OF_TYPE, &args![audio, 0xffff_ffffu32, 0u32]);
    e.call(MEDIA_CLEAR, &args![]);
    let object = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
    e.call(OBJECT_CALL_009698A0, &args![object]);
    if e.call(DLC_GETTER, &args![]).u32() != 0 {
        let object = e.call(DLC_GETTER, &args![]).u32();
        fn_007d0bd0(e, Ptr::new(object), 0);
        let object = e.call(DLC_GETTER, &args![]).u32();
        fn_007d0bb0(e, Ptr::new(object));
    }
    e.mem.set_u8(MAIN_MENU_BYTE, 1);
    let main = e.global::<u32>(MAIN_OBJECT);
    fn_007d0b90(e, Ptr::new(main));
    let texture_owner = e.global::<u32>(TEXTURE_OWNER);
    e.call(TEXTURE_OWNER_CALL, &args![texture_owner, 1u32, 0u32, 0u32]);
    e.with_stack(0x40, |e, name| {
        e.call(MEMSET, &args![name, 0u32, 0x40u32]);
        e.call(
            STRING_COPY_NARROW,
            &args![name, 0x40u32, TEXT_MAIN_TITLE_MUSIC],
        );
        e.call(SET_DIALOG_FLAG, &args![1u32]);
        e.call(
            MEDIA_PLAY,
            &args![8u32, name, 0x1388u32, 1u32, 1u32, 0.0f32, 0u32],
        );
        e.call(MEDIA_FINISH, &args![]);
    });
}

// Translated from 007d0b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+2` of the object.
pub fn fn_007d0b90(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 2, 1);
}

// Translated from 007d0bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x87` of the object.
pub fn fn_007d0bb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x87, 1);
}

// Translated from 007d0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x88` of the object.
pub fn fn_007d0bd0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x88, value);
}

/// `Interface::HideMenus` (Ghidra name; cdecl, no arguments),
/// `Interface::CloseAllSupportMenus(flag)` (Ghidra name; cdecl),
/// `BSAudio::KillAllOfType(audio, -1, 0)` (Ghidra name, `RET 8`) and
/// `FalloutAudioMedia::MediaClear` (Ghidra name).
const HIDE_MENUS: u32 = 0x0070_3610;
const CLOSE_ALL_SUPPORT_MENUS: u32 = 0x0070_6320;
const KILL_ALL_OF_TYPE: u32 = 0x00ad_84e0;
const MEDIA_CLEAR: u32 = 0x0082_fa30;
/// The global at `011dea3c`: `009698a0` (the engine map files it under
/// `ai/playercharacter.cpp`) is called on it and its result is unused.
const GLOBAL_OBJECT_011DEA3C: u32 = 0x011d_ea3c;
const OBJECT_CALL_009698A0: u32 = 0x0096_98a0;
/// The byte at `011daacd`, set when the main menu is chosen.
const MAIN_MENU_BYTE: u32 = 0x011d_aacd;
/// `Main` object call `00457d70(1, 0, 0)` on the object at `011dea10`
/// (`RET 0xC`).
const TEXTURE_OWNER_CALL: u32 = 0x0045_7d70;
/// `"Data\Music\Special\MainTitle.mp3"`.
const TEXT_MAIN_TITLE_MUSIC: u32 = 0x0107_6f3c;
/// `007cb710`-adjacent object: `004a4040`, true when the start menu exists
/// and its flag 1 is set; `Interface::GetCurrentRenderedMenu` and
/// `Interface::GetPipboy` (Ghidra names); `FOPipboyManager::ResetColors`
/// (Ghidra name); `Menu::StartFadeOut` (Ghidra name).
const START_MENU_FLAG_1: u32 = 0x004a_4040;
const CURRENT_RENDERED_MENU: u32 = 0x0070_7ad0;
const GET_PIPBOY: u32 = 0x0070_5990;
const PIPBOY_RESET_COLORS: u32 = 0x007f_7f40;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `007d6550` (on the menu) and the interface manager's row count
/// (`0073b020`, on the manager, returns an `int`).
const MENU_REFRESH_C: u32 = 0x007d_6550;
const MANAGER_ROW_COUNT: u32 = 0x0073_b020;

/// Callees and data of the functions from `007d0bf0` on that no earlier
/// function of this file needed (names by address and role; the ones the
/// engine map or the Xbox PDB names say so).
///
/// `007d0bf0`: a setter `(byte)` on a model object (`00450f90`, `RET 4`), the
/// interface manager's word at `+0x28` (`0045cd60`), the manager call
/// `00713fb0(0, 0)` and the main object call `005b6cb0`.
const FN_00450F90: u32 = 0x0045_0f90;
const FN_00713FB0: u32 = 0x0071_3fb0;
const FN_005B6CB0: u32 = 0x005b_6cb0;
/// `007d6030` (the menu is `this`, returns a byte) and
/// `StartMenu::PopulateOptionsListBox` (Ghidra name, `RET 4`; it belongs to a
/// later range of this unit).
const FN_007D6030: u32 = 0x007d_6030;
const POPULATE_OPTIONS_LIST_BOX: u32 = 0x007d_42b0;
/// `ShowMessage` call arguments: the setting strings `ChooseBack`,
/// `ConfirmContinue` and `ConfirmNewGame` show (`Setting` objects whose
/// `403df0` returns the text).
const SETTING_011D2B14: u32 = 0x011d_2b14;
const SETTING_011D1F14: u32 = 0x011d_1f14;
const SETTING_011D48AC: u32 = 0x011d_48ac;
const SETTING_011DAC00: u32 = 0x011d_ac00;
const SETTING_011D4F78: u32 = 0x011d_4f78;
const SETTING_011D25D4: u32 = 0x011d_25d4;
const SETTING_011D2C4C: u32 = 0x011d_2c4c;
/// The value slot of a setting object (`0043d4d0`: `this + 4`, or a static
/// slot when `this` is null) and the two settings `007d11d0` and `007d1260`
/// copy their option's value into.
const SETTING_VALUE_SLOT: u32 = 0x0043_d4d0;
const SETTING_011E0940: u32 = 0x011e_0940;
const SETTING_011E087C: u32 = 0x011e_087c;
/// The object at `011dea3c` (`GLOBAL_OBJECT_011DEA3C`): its word `+0x7b8`
/// (`005be4d0` reads it, `007d1240` writes it), `+0x7c0` (`007d1340`,
/// `007d1320`) and the float at `+0xe18` (`007d1360`), and the calls
/// `0093e770(old, 1)`, `005d1530(0)`, `005d14d0(1)` and `007adcc0(0)`.
const FN_005BE4D0: u32 = 0x005b_e4d0;
const FN_0093E770: u32 = 0x0093_e770;
const FN_005D1530: u32 = 0x005d_1530;
const FN_005D14D0: u32 = 0x005d_14d0;
const FN_007ADCC0: u32 = 0x007a_dcc0;
/// `PlayerCharacter::SetHardcore` (Xbox PDB name, engine map): `(this, value,
/// 0)`, `RET 8`; `004d1360` is the getter (`+0x7bc == 1`).
const SET_HARDCORE: u32 = 0x0096_9e90;
const FN_004D1360: u32 = 0x004d_1360;
/// Boolean settings: setter `(this, byte)` `RET 4` (`004de2d0`, returns
/// `this`) and getter (`00454af0`, returns the byte); `004503f0` returns the
/// value word.
const SETTING_SET_BOOL: u32 = 0x004d_e2d0;
const SETTING_GET_BOOL: u32 = 0x0045_4af0;
const SETTING_GET_VALUE: u32 = 0x0045_03f0;
/// The object at `011d96c0` whose flag word `+0x1c0` `007d14c0` edits.
const FLAGS_OBJECT_011D96C0: u32 = 0x011d_96c0;
/// The boolean settings of `007d1440` .. `007d16f0`, `007d2e50` and
/// `007d30e0`.
const SETTING_011D8A38: u32 = 0x011d_8a38;
const SETTING_011D89F0: u32 = 0x011d_89f0;
const SETTING_011D8974: u32 = 0x011d_8974;
const SETTING_011D8980: u32 = 0x011d_8980;
const SETTING_011D8934: u32 = 0x011d_8934;
const SETTING_011D8928: u32 = 0x011d_8928;
const SETTING_011D898C: u32 = 0x011d_898c;
const SETTING_011E0A5C: u32 = 0x011e_0a5c;
const SETTING_011E0850: u32 = 0x011e_0850;
const SETTING_011D8C4C: u32 = 0x011d_8c4c;
/// Slider helpers: `Setting` float slot (`00403e20`: the address of the float
/// at `this + 4`, or of a static zero), the float setter `(this, value)`
/// (`004ed780`, `RET 4`, returns `this`), the float getter (`00450410`, ST0),
/// the table store `(index, value)` (`004504f0`, cdecl) and its table
/// (`011ad7b8`), and the float sink `004dc2d0(value)` (cdecl).
const FLOAT_SLOT: u32 = 0x0040_3e20;
const FLOAT_SET: u32 = 0x004e_d780;
const FLOAT_TABLE_SET: u32 = 0x0045_04f0;
const FLOAT_TABLE: u32 = 0x011a_d7b8;
const FLOAT_SINK: u32 = 0x004d_c2d0;
/// `SystemColorManager::GetInstance` (Xbox PDB, no arguments), its setter
/// `(this, 1 or 4, colour)` (`00719120`, `RET 8`), its getter
/// `(this, 1 or 4)` (`007190a0`, `RET 4`), the colour table (`011dac50`) and
/// `00a0b350(this, 1 or 4, 0.0)` (`RET 8`) on the interface manager's menus
/// root.
const SYSTEM_COLOR_INSTANCE: u32 = 0x0071_8b60;
const SYSTEM_COLOR_SET: u32 = 0x0071_9120;
const SYSTEM_COLOR_GET: u32 = 0x0071_90a0;
const COLOR_TABLE: u32 = 0x011d_ac50;
const FN_00A0B350: u32 = 0x00a0_b350;
/// The float object `007d1b30` reads, the integer at `0126efc4` and the
/// setting `007d1b40` writes with `0045ce80`.
const FLOAT_OBJECT_011C71AC: u32 = 0x011c_71ac;
const GLOBAL_0126EFC4: u32 = 0x0126_efc4;
const SETTING_SET_INT: u32 = 0x0045_ce80;
const SETTING_011C779C: u32 = 0x011c_779c;
/// Constants of `007d2eb0`: two doubles and a float in the exe's data.
const DOUBLE_SLIDER_SCALE: u32 = 0x0107_6f70;
const DOUBLE_SLIDER_OFFSET: u32 = 0x0107_6f68;
const FLOAT_SLIDER_FULL: u32 = 0x0107_6f60;
const SETTING_011E0A6C: u32 = 0x011e_0a6c;
/// `0.0` and `1.0` as doubles in the exe's data (`007d1360`, `007d3620`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_ONE: u32 = 0x0101_2070;
/// The doubles the audio volume sliders multiply `count - 1` by for `+0x18`
/// (`FMUL double ptr [...]`): `0.5` at `01011588` (`007d29a9`, `007d2e29`)
/// and `0x3FE3333340000000` (0.6 as a float, widened) at `010290b8` (`007d2ac9`,
/// `007d2be9`).
const DOUBLE_HALF: u32 = 0x0101_1588;
const DOUBLE_SIX_TENTHS: u32 = 0x0102_90b8;
/// `007d2fc0` and `007d30e0`: the interface manager's call
/// `00719740(byte)` (`RET 4`), the object `007d30b0` writes (`011f35cc`, byte
/// at `+0x1b88`), the main object's `00877720` result byte setter `005407b0`.
const FN_00719740: u32 = 0x0071_9740;
const OBJECT_011F35CC: u32 = 0x011f_35cc;
const FN_005407B0: u32 = 0x0054_07b0;
/// `00a24b70(this, index)` (`RET 4`) on the main object's `00877720` result.
const FN_00A24B70: u32 = 0x00a2_4b70;
/// `ConfirmContinue` and `007d3470`: save entry calls. `00851330(this)` on
/// the save manager, `008467b0(entry)` (returns a byte),
/// `ArchiveManager::CleanupArchiveStringsAfterStartup` (Xbox PDB name),
/// `0075f8c0(entry)`, `00408da0` and `00408b20(a, b)` (cdecl).
const FN_00851330: u32 = 0x0085_1330;
const FN_008467B0: u32 = 0x0084_67b0;
const ARCHIVE_CLEANUP: u32 = 0x00af_59e0;
const FN_0075F8C0: u32 = 0x0075_f8c0;
const FN_00408DA0: u32 = 0x0040_8da0;
const FN_00408B20: u32 = 0x0040_8b20;
/// `ConfirmNewGame`: the C `strtol` (`00ec8d08`, cdecl), the form type byte
/// getter (`00401170`), the type descriptors it casts between (`TESForm`
/// `01183028` to `TESQuest` `01186500`; `TESBoundObject` `01183108` to
/// `TESNPC` `01183a1c`, names read from the descriptors), the form getter
/// `BGSSaveFormBuffer::GetForm` (Xbox PDB name, `007af430`: the word at
/// `+0x20`), and the other calls it makes.
const STRTOL: u32 = 0x00ec_8d08;
const FN_00401170: u32 = 0x0040_1170;
const TYPE_DESCRIPTOR_QUEST: u32 = 0x0118_6500;
const TYPE_DESCRIPTOR_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_DESCRIPTOR_NPC: u32 = 0x0118_3a1c;
const GET_FORM: u32 = 0x007a_f430;
const FN_0060C9C0: u32 = 0x0060_c9c0;
const FN_004581C0: u32 = 0x0045_81c0;
const ERROR_CALL: u32 = 0x0040_fbe0;
const MEDIA_CLOSE: u32 = 0x0083_04a0;
const FN_00788A90: u32 = 0x0078_8a90;
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;
const FLAG_011D8907: u32 = 0x011d_8907;
const TEXT_QUEST_NOT_FOUND: u32 = 0x0107_6f78;
/// `007d3620` and its friends: `BGSSaveLoadManager::RemoveSaveGame` (Xbox PDB
/// name, `008467e0`), `BGSSaveLoadFileEntry::IsAutosave` (Xbox PDB name,
/// `00846af0`) and its sibling `00846b10`, the list calls `007d8040(list,
/// item)`, `007d8700(list)`, `007d87b0(list)`, `007d62e0(menu)`,
/// `007d86c0(list, index, 0)`, `007d80f0(list, index, 0)`, `00825c00(list)`
/// (the word at `+0x14`), `007d6e90(list)` (the list's item `(0, 0)`),
/// `Tile::AddNeedsUpdate` (Xbox PDB name) and the callback
/// `StartMenu::InitializeSaveLoadListItem` (Ghidra name, `007d67b0`).
const REMOVE_SAVE_GAME: u32 = 0x0084_67e0;
const IS_AUTOSAVE: u32 = 0x0084_6af0;
const FN_00846B10: u32 = 0x0084_6b10;
const FN_007D8040: u32 = 0x007d_8040;
const FN_007D8700: u32 = 0x007d_8700;
const FN_007D87B0: u32 = 0x007d_87b0;
const FN_007D62E0: u32 = 0x007d_62e0;
const FN_007D86C0: u32 = 0x007d_86c0;
const FN_007D80F0: u32 = 0x007d_80f0;
const FN_00825C00: u32 = 0x0082_5c00;
const FN_007D6E90: u32 = 0x007d_6e90;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
const INITIALIZE_SAVE_LOAD_LIST_ITEM: u32 = 0x007d_67b0;
/// `"%s %u/%i %s"` (the save count line) in the exe's data.
const FORMAT_SAVE_COUNT: u32 = 0x0107_6fb0;

/// `if (pMe != 0 && pMe->TestFlags(bits))` as the exe tests it: the global
/// is read again for the flag call.
fn menu_exists_with_flag(e: &mut Engine, bits: u32) -> bool {
    let menu = start_menu_global(e);
    !menu.is_null() && menu_flag(e, menu, bits)
}

/// Stores an option's value (`UserOption::iVal`, Xbox PDB, `+0x14`) and then
/// its second word (`iDefaultVal`, `+0x18`).
fn set_option_words(e: &mut Engine, option: Ptr, value: u32, second: u32) {
    e.mem.set_u32(option.addr() + 0x14, value);
    e.mem.set_u32(option.addr() + 0x18, second);
}

/// `Setting` float value as the slider functions read it: `*00403e20(setting)`
/// copied through the x87 stack into a `float`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let slot = e.call(FLOAT_SLOT, &args![setting]).u32();
    e.mem.f32(slot)
}

/// `float(count - 1)` for `FILD qword` of the unsigned `count - 1`
/// (`UserOption::iNumSettings`, `+0x20`, minus one).
fn last_step(e: &Engine, option: Ptr) -> f64 {
    f64::from(e.mem.u32(option.addr() + 0x20).wrapping_sub(1))
}

/// `FILD dword [option + 0x14] / FILD qword (count - 1); FSTP float`: the
/// slider position as a `float`.
fn slider_ratio(e: &Engine, option: Ptr) -> f32 {
    let value = f64::from(e.mem.u32(option.addr() + 0x14) as i32);
    (value / last_step(e, option)) as f32
}

/// `004bd510(scaled, 1.0)` then `_ftol2`: the whole step the slider rounds
/// to.
fn slider_step_index(e: &mut Engine, scaled: f32) -> u32 {
    let stepped = e.call(SLIDER_STEP, &args![scaled, 1.0f32]).f64();
    e.call(FLOAT_TO_INT, &args![stepped]).u32()
}

/// Stores one of the slider's words (`offset` `0x14` or `0x18`) from a ratio:
/// `step(last * ratio)` with `last = count - 1`.
fn store_slider_step(e: &mut Engine, option: Ptr, offset: u32, ratio: f32) {
    store_slider_step_f64(e, option, offset, f64::from(ratio));
}

/// `store_slider_step` for a ratio the exe multiplies by as a `double`
/// (`FMUL double ptr [...]` or `FLD1; FMULP`), not narrowed to a `float`
/// first: `step(float(last * ratio))`.
fn store_slider_step_f64(e: &mut Engine, option: Ptr, offset: u32, ratio: f64) {
    let last = last_step(e, option);
    let step = slider_step_index(e, (last * ratio) as f32);
    e.mem.set_u32(option.addr() + offset, step);
}

/// Both words of the slider from two ratios that need no call between them.
fn store_slider_steps(e: &mut Engine, option: Ptr, first: f32, second: f32) {
    store_slider_step(e, option, 0x14, first);
    store_slider_step(e, option, 0x18, second);
}

/// `(value - min) / (max - min)` computed in extended precision and stored
/// as a `float`.
fn range_ratio(value: f64, min: f32, max: f32) -> f32 {
    ((value - f64::from(min)) / (f64::from(max) - f64::from(min))) as f32
}

/// `(max - min) * ratio + min` stored as a `float`.
fn range_value(min: f32, max: f32, ratio: f32) -> f32 {
    ((f64::from(max) - f64::from(min)) * f64::from(ratio) + f64::from(min)) as f32
}

// Translated from 007d0bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unlabelled in Ghidra (the name is by role): sets the byte `1` on the
/// model of the interface manager's menus root and on the model of the word
/// at `+0x28` of the manager, calls `00713fb0(0, 0)` on the manager and
/// `005b6cb0` on the main object.
pub fn fn_007d0bf0(e: &mut Engine) {
    let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
    let root = e.call(MENUS_ROOT_GETTER, &args![manager]).u32();
    let model = e.call(TILE_MODEL, &args![root]).u32();
    e.call(FN_00450F90, &args![model, 1u32]);
    let word = e.call(TILE_PARENT, &args![manager]).u32();
    let model = e.call(TILE_MODEL, &args![word]).u32();
    e.call(FN_00450F90, &args![model, 1u32]);
    e.call(FN_00713FB0, &args![manager, 0u32, 0u32]);
    let main = e.global::<u32>(MAIN_OBJECT);
    e.call(FN_005B6CB0, &args![main]);
}

// Translated from 007d0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: with a start menu, fades kind 2 in, sets menu flag
/// `0x1000` and, unless `007d6030` says otherwise, refreshes the menu.
pub fn fn_007d0c50(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    fade(e, menu, 2, 0);
    let menu = start_menu_global(e);
    e.call(MENU_FLAGS_SET, &args![menu, 0x1000u32, 1u32]);
    let menu = start_menu_global(e);
    if !e.call(FN_007D6030, &args![menu]).bool() {
        let menu = start_menu_global(e);
        e.call(MENU_REFRESH_A, &args![menu]);
    }
}

// Translated from 007d0cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: when menu flag `0x1000` is set, sets the text of the
/// tile at `+0x68` (`011d4b1c`'s setting string), makes it visible, and when
/// the gamepad is active and the remap device is 3 hides the tile at `+0x3c`
/// (trait `0xfaf`).
pub fn fn_007d0cb0(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() || !menu_flag(e, menu, 0x1000) {
        return;
    }
    let text = e.call(SETTING_STRING, &args![SETTING_REMAP_TEXT]).u32();
    let menu = start_menu_global(e);
    let tile = e.mem.u32(menu.addr() + 0x68);
    e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
    let menu = start_menu_global(e);
    let tile = e.mem.u32(menu.addr() + 0x68);
    e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 1u32]);
    if e.call(GAMEPAD_ACTIVE, &args![]).bool() {
        let menu = start_menu_global(e);
        if e.get(menu.cast::<StartMenu>(), StartMenu::iRemapDevice) == 3 {
            let menu = start_menu_global(e);
            let tile = e.mem.u32(menu.addr() + 0x3c);
            e.call(TILE_SET_INT, &args![tile, TRAIT_TARGET, 0u32]);
        }
    }
}
/// The setting string `007d0cb0` puts on the remap tile.
const SETTING_REMAP_TEXT: u32 = 0x011d_4b1c;

/// A tile trait a choose function sets: the menu's tile at `offset`, trait
/// `id`, integer `value`.
type TileTrait = (u32, u32, u32);

/// The four choose functions that show a list of options: with a start menu,
/// set `before` (when given), disable the settings list (`+0xb4`), set `after`
/// (when given) and populate the options list for `mode`.
fn choose_options_list(
    e: &mut Engine,
    mode: u32,
    before: Option<TileTrait>,
    after: Option<TileTrait>,
) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    if let Some((offset, id, value)) = before {
        let tile = e.mem.u32(menu.addr() + offset);
        e.call(TILE_SET_INT, &args![tile, id, value]);
    }
    let menu = start_menu_global(e);
    e.call(LIST_BOX_SET_ENABLED, &args![menu.byte_add(0xb4), 0u32]);
    if let Some((offset, id, value)) = after {
        let menu = start_menu_global(e);
        let tile = e.mem.u32(menu.addr() + offset);
        e.call(TILE_SET_INT, &args![tile, id, value]);
    }
    let menu = start_menu_global(e);
    e.call(POPULATE_OPTIONS_LIST_BOX, &args![menu, mode]);
}

// Translated from 007d0d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: disables the settings list and populates the options
/// list for mode 8.
pub fn fn_007d0d40(e: &mut Engine) {
    choose_options_list(e, 8, None, None);
}

// Translated from 007d0d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: makes the tile at `+0x50` visible, disables the settings
/// list and populates the options list for mode `0x10`.
pub fn fn_007d0d70(e: &mut Engine) {
    choose_options_list(e, 0x10, Some((0x50, TRAIT_VISIBLE, 1)), None);
}

// Translated from 007d0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: disables the settings list and populates the options
/// list for mode `0x20`.
pub fn fn_007d0dc0(e: &mut Engine) {
    choose_options_list(e, 0x20, None, None);
}

// Translated from 007d0df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: disables the settings list, sets trait `0x1004` (user
/// trait 0) of the tile at `+0x64` to 0 and populates the options list for
/// mode `0x40`.
pub fn fn_007d0df0(e: &mut Engine) {
    choose_options_list(e, 0x40, None, Some((0x64, TRAIT_USER_0, 0)));
}

/// The list `ChooseBack` steps out of: the first enabled one in the order the
/// exe tests them (confirm, settings, downloads, save-load, main).
enum BackStage {
    Confirm,
    Settings,
    Downloads,
    SaveLoad,
    Main,
    Nothing,
}

fn back_stage(e: &mut Engine) -> BackStage {
    let menu = start_menu_global(e);
    if e.call(LIST_BOX_IS_ENABLED, &args![menu.byte_add(0xe4)])
        .bool()
    {
        return BackStage::Confirm;
    }
    let menu = start_menu_global(e);
    if e.call(LIST_BOX_IS_ENABLED, &args![menu.byte_add(0xb4)])
        .bool()
    {
        return BackStage::Settings;
    }
    let menu = start_menu_global(e);
    if e.call(INT_LIST_IS_ENABLED, &args![menu.byte_add(0x144)])
        .bool()
    {
        return BackStage::Downloads;
    }
    let menu = start_menu_global(e);
    if e.call(SAVE_LOAD_LIST_IS_ENABLED, &args![menu.byte_add(0x174)])
        .bool()
    {
        return BackStage::SaveLoad;
    }
    let menu = start_menu_global(e);
    if e.call(LIST_BOX_IS_ENABLED, &args![menu.byte_add(0x84)])
        .bool()
    {
        return BackStage::Main;
    }
    BackStage::Nothing
}

// Translated from 007d0e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::ChooseBack` (Xbox PDB): steps back one level of the start
/// menu. With the main list selectable: closes a pending message
/// (`0x20000` shows the setting-string message), clears flags `0x20000` and
/// `0x200000` and hides the tile at `+0x50`; then, for the first of the
/// options, confirm, settings, downloads, save-load or main list that is
/// enabled (or flag `0x1000`), hides it and fades to the previous one
/// (kinds 7, 1, 0xe, 6, 5); it returns early where the exe does (main list
/// with no flag `1`, flag `4`, or flag `0x40000`).
pub fn start_menu_choose_back(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    if !e
        .call(OPTIONS_LIST_HAS_SELECTION, &args![menu.byte_add(0x84)])
        .bool()
    {
        return;
    }
    let menu = start_menu_global(e);
    if menu_flag(e, menu, 0x20000) {
        let text = e.call(SETTING_STRING, &args![SETTING_BINDING_TEXT]).u32();
        let title = e.call(SETTING_STRING, &args![SETTING_011D2B14]).u32();
        e.call(
            SHOW_MESSAGE,
            &args![title, 0u32, 0u32, 0u32, 0u32, 0x12u32, 0.0f32, 0.0f32, text, 0u32],
        );
    }
    let menu = start_menu_global(e);
    e.call(MENU_FLAGS_SET, &args![menu, 0x20000u32, 0u32]);
    let menu = start_menu_global(e);
    e.call(MENU_FLAGS_SET, &args![menu, 0x20_0000u32, 0u32]);
    let menu = start_menu_global(e);
    let tile = e.mem.u32(menu.addr() + 0x50);
    e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 0u32]);
    let menu = start_menu_global(e);
    let mut hide_options = e
        .call(LIST_BOX_IS_ENABLED, &args![menu.byte_add(0x114)])
        .bool();
    if !hide_options {
        let menu = start_menu_global(e);
        hide_options = menu_flag(e, menu, 0x1000);
    }
    if hide_options {
        let menu = start_menu_global(e);
        e.call(HIDE_OPTIONS_CONTAINER, &args![menu]);
        start_menu_choose_settings(e);
        return;
    }
    match back_stage(e) {
        BackStage::Confirm => {
            let menu = start_menu_global(e);
            e.call(LIST_BOX_SET_ENABLED, &args![menu.byte_add(0xe4), 0u32]);
            let menu = start_menu_global(e);
            fade(e, menu, 7, 0);
            let menu = start_menu_global(e);
            if e.call(OPTIONS_LIST_HAS_SELECTION, &args![menu.byte_add(0x174)])
                .bool()
            {
                let menu = start_menu_global(e);
                e.call(MENU_FLAGS_SET, &args![menu, 0x40000u32, 1u32]);
                return;
            }
        }
        BackStage::Settings => {
            let menu = start_menu_global(e);
            fade(e, menu, 1, 0);
            let menu = start_menu_global(e);
            e.call(LIST_BOX_SET_ENABLED, &args![menu.byte_add(0xb4), 0u32]);
        }
        BackStage::Downloads => {
            let menu = start_menu_global(e);
            fade(e, menu, 0xe, 0);
            let menu = start_menu_global(e);
            e.call(INT_LIST_SET_ENABLED, &args![menu.byte_add(0x144), 0u32]);
        }
        BackStage::SaveLoad => {
            let menu = start_menu_global(e);
            start_menu_update_can_load_status(e, menu.cast());
            let menu = start_menu_global(e);
            e.vcall(menu.addr() + 0x174, 0, &args![0u32]);
            let menu = start_menu_global(e);
            e.call(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO, &args![menu]);
            let menu = start_menu_global(e);
            e.call(
                SAVE_LOAD_LIST_SET_ENABLED,
                &args![menu.byte_add(0x174), 0u32],
            );
            let menu = start_menu_global(e);
            fade(e, menu, 6, 0);
            let menu = start_menu_global(e);
            e.vcall(menu.addr() + 0x174, 0x1c, &args![]);
        }
        BackStage::Main => {
            if !e.call(START_MENU_FLAG_1, &args![]).bool() {
                return;
            }
            let menu = start_menu_global(e);
            if menu_flag(e, menu, 4) {
                return;
            }
            start_menu_close(e, 1);
            return;
        }
        BackStage::Nothing => {
            let menu = start_menu_global(e);
            if menu_flag(e, menu, 0x40000) {
                return;
            }
        }
    }
    let menu = start_menu_global(e);
    e.call(LIST_BOX_SET_ENABLED, &args![menu.byte_add(0x84), 1u32]);
    let menu = start_menu_global(e);
    e.call(LIST_BOX_DESELECT_ALL, &args![menu.byte_add(0x84), 1u32]);
    let menu = start_menu_global(e);
    fade(e, menu, 5, 0);
    let menu = start_menu_global(e);
    if menu_flag(e, menu, 2) {
        e.call(MENU_REFRESH_D, &args![]);
        let menu = start_menu_global(e);
        e.call(MENU_FLAGS_SET, &args![menu, 2u32, 0u32]);
    }
}

// Translated from 007d11d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback (`UserOption::pfChangeFn`, Xbox PDB): with the
/// menu running and flag `0x20` set, copies the option's value (`+0x14`)
/// into the value slot of the setting at `011e0940` and passes it to
/// `007d1240` on the object at `011dea3c`; otherwise reads that object's
/// `+0x7b8` (`005be4d0`) into the option and marks it kind 2.
pub fn fn_007d11d0(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let slot = e.call(SETTING_VALUE_SLOT, &args![SETTING_011E0940]).u32();
        let value = e.mem.u32(option.addr() + 0x14);
        e.mem.set_u32(slot, value);
        let value = e.mem.u32(slot);
        let object = Ptr::new(e.global::<u32>(GLOBAL_OBJECT_011DEA3C));
        fn_007d1240(e, object, value);
    } else {
        let object = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
        let value = e.call(FN_005BE4D0, &args![object]).u32();
        set_option_words(e, option, value, 2);
    }
}

// Translated from 007d1240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+0x7b8` of the object.
pub fn fn_007d1240(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x7b8, value);
}

// Translated from 007d1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback like `007d11d0`, for the setting at `011e087c`
/// and the word at `+0x7c0` (`007d12d0` writes it, `007d1340` reads it).
pub fn fn_007d1260(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let slot = e.call(SETTING_VALUE_SLOT, &args![SETTING_011E087C]).u32();
        let value = e.mem.u32(option.addr() + 0x14);
        e.mem.set_u32(slot, value);
        let value = e.mem.u32(slot);
        let object = Ptr::new(e.global::<u32>(GLOBAL_OBJECT_011DEA3C));
        fn_007d12d0(e, object, value);
    } else {
        let object = Ptr::new(e.global::<u32>(GLOBAL_OBJECT_011DEA3C));
        let value = fn_007d1340(e, object);
        set_option_words(e, option, value, 2);
    }
}

// Translated from 007d12d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0x7c0` of the object (`007d1320`); when that changed
/// the word and `007d1360` says the float at `+0xe18` is above zero, calls
/// `0093e770(old, 1)`.
pub fn fn_007d12d0(e: &mut Engine, this: Ptr, value: u32) {
    let old = fn_007d1340(e, this);
    fn_007d1320(e, this, value);
    if old != fn_007d1340(e, this) && fn_007d1360(e, this) {
        e.call(FN_0093E770, &args![this, old, 1u32]);
    }
}

// Translated from 007d1320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+0x7c0` of the object.
pub fn fn_007d1320(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x7c0, value);
}

// Translated from 007d1340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x7c0` of the object.
pub fn fn_007d1340(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x7c0)
}

// Translated from 007d1360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the float at `+0xe18` of the object is above zero (`FCOMP` against
/// the double zero at `01012060`, flag test `0x41`: unordered is false).
pub fn fn_007d1360(e: &mut Engine, this: Ptr) -> bool {
    let value = f64::from(e.mem.f32(this.addr() + 0xe18));
    !not_above(value, e.global::<f64>(DOUBLE_ZERO))
}

// Translated from 007d13a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback: with the menu running and flag `0x20` set, calls
/// `007d1400` on the object at `011dea3c` with the option's value; otherwise
/// stores whether `004d1360` (the object's `+0x7bc == 1`) holds as the value
/// and kind 0.
pub fn fn_007d13a0(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let value = e.mem.u32(option.addr() + 0x14);
        let object = Ptr::new(e.global::<u32>(GLOBAL_OBJECT_011DEA3C));
        fn_007d1400(e, object, value);
    } else {
        let object = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
        let hardcore = e.call(FN_004D1360, &args![object]).bool();
        set_option_words(e, option, u32::from(hardcore), 0);
    }
}

// Translated from 007d1400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `PlayerCharacter::SetHardcore(this, value, 0)` (Xbox PDB name) when
/// `value` is 0 or 1.
pub fn fn_007d1400(e: &mut Engine, this: Ptr, value: u32) {
    if value == 1 || value == 0 {
        e.call(SET_HARDCORE, &args![this, value, 0u32]);
    }
}

// Translated from 007d1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback for the boolean setting at `011d8a38`: running,
/// it sets the setting from the option (`value == 1`) and then edits bit
/// `0x40` of the flag word of `011d96c0` (`007d14c0`) with the setting's
/// value (`004503f0`); otherwise reads the setting into the option as kind 1.
pub fn fn_007d1440(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let on = e.mem.u32(option.addr() + 0x14) == 1;
        e.call(SETTING_SET_BOOL, &args![SETTING_011D8A38, u32::from(on)]);
        let value = e.call(SETTING_GET_VALUE, &args![SETTING_011D8A38]).u32();
        fn_007d14c0(e, 0x40, value);
    } else {
        let on = e.call(SETTING_GET_BOOL, &args![SETTING_011D8A38]).bool();
        set_option_words(e, option, u32::from(on), 1);
    }
}

// Translated from 007d14c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the object at `011d96c0`, sets the bits `flags` of its word `+0x1c0`
/// when `value` is not 0 and clears them when it is 0.
pub fn fn_007d14c0(e: &mut Engine, flags: u32, value: u32) {
    let object = e.global::<u32>(FLAGS_OBJECT_011D96C0);
    if object == 0 {
        return;
    }
    let old = e.mem.u32(object + 0x1c0);
    let object = e.global::<u32>(FLAGS_OBJECT_011D96C0);
    let new = if value != 0 {
        old | flags
    } else {
        !flags & old
    };
    e.mem.set_u32(object + 0x1c0, new);
}

/// The shared body of the boolean option callbacks `007d1510` .. `007d16f0`
/// and `007d2e50`: with the menu running and flag `0x20` set, writes the
/// option's `value == 1` to `setting`; otherwise reads the setting into the
/// option with kind `second`.
fn boolean_option_callback(e: &mut Engine, option: Ptr, setting: u32, second: u32) {
    if menu_exists_with_flag(e, 0x20) {
        let on = e.mem.u32(option.addr() + 0x14) == 1;
        e.call(SETTING_SET_BOOL, &args![setting, u32::from(on)]);
    } else {
        let on = e.call(SETTING_GET_BOOL, &args![setting]).bool();
        set_option_words(e, option, u32::from(on), second);
    }
}

// Translated from 007d1510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Boolean option callback for the setting at `011d89f0` (kind 1).
pub fn fn_007d1510(e: &mut Engine, option: Ptr) {
    boolean_option_callback(e, option, SETTING_011D89F0, 1);
}

// Translated from 007d1570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Boolean option callback for the setting at `011d8974` (kind 1).
pub fn fn_007d1570(e: &mut Engine, option: Ptr) {
    boolean_option_callback(e, option, SETTING_011D8974, 1);
}

// Translated from 007d15d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Boolean option callback for the setting at `011d8980` (kind 1).
pub fn fn_007d15d0(e: &mut Engine, option: Ptr) {
    boolean_option_callback(e, option, SETTING_011D8980, 1);
}

// Translated from 007d1630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Boolean option callback for the setting at `011d8934` (kind 1).
pub fn fn_007d1630(e: &mut Engine, option: Ptr) {
    boolean_option_callback(e, option, SETTING_011D8934, 1);
}

// Translated from 007d1690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Boolean option callback for the setting at `011d8928` (kind 0).
pub fn fn_007d1690(e: &mut Engine, option: Ptr) {
    boolean_option_callback(e, option, SETTING_011D8928, 0);
}

// Translated from 007d16f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Boolean option callback for the setting at `011d898c` (kind 1).
pub fn fn_007d16f0(e: &mut Engine, option: Ptr) {
    boolean_option_callback(e, option, SETTING_011D898C, 1);
}

// Translated from 007d1750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback for the slider on the setting at `011d9774`
/// (`VOLUME_OBJECT`): running, sets the setting to `iVal / (count - 1)` and
/// sets menu flag `0x40`; otherwise works the option's two words out from
/// the setting's value (kind words `step(last * value)` and
/// `step(last * 1.0)`).
pub fn fn_007d1750(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let ratio = slider_ratio(e, option);
        e.call(FLOAT_SET, &args![VOLUME_OBJECT, ratio]);
        let menu = start_menu_global(e);
        e.call(MENU_FLAGS_SET, &args![menu, 0x40u32, 1u32]);
    } else {
        let value = setting_float(e, VOLUME_OBJECT);
        store_slider_steps(e, option, value, 1.0);
    }
}

/// `SystemColorManager` option callback: running, sets colour `id` of the
/// instance to the table entry for the option's value and calls
/// `00a0b350(1 or 4, 0.0)` on the menus root; otherwise looks for the
/// table entry equal to the instance's colour `id` (default word 2, value
/// 0, falling back to the default word when none matches).
fn color_option_callback(e: &mut Engine, option: Ptr, id: u32) {
    if menu_exists_with_flag(e, 0x20) {
        let index = e.mem.u32(option.addr() + 0x14);
        let color = e.mem.u32(COLOR_TABLE.wrapping_add(index.wrapping_mul(4)));
        let instance = e.call(SYSTEM_COLOR_INSTANCE, &args![]).u32();
        e.call(SYSTEM_COLOR_SET, &args![instance, id, color]);
        let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
        let root = e.call(MENUS_ROOT_GETTER, &args![manager]).u32();
        e.call(FN_00A0B350, &args![root, id, 0.0f32]);
    } else {
        e.mem.set_u32(option.addr() + 0x18, 2);
        e.mem.set_u32(option.addr() + 0x14, 0);
        loop {
            let index = e.mem.u32(option.addr() + 0x14);
            let instance = e.call(SYSTEM_COLOR_INSTANCE, &args![]).u32();
            let current = e.call(SYSTEM_COLOR_GET, &args![instance, id]).u32();
            if e.mem.u32(COLOR_TABLE.wrapping_add(index.wrapping_mul(4))) == current {
                return;
            }
            let next = e.mem.u32(option.addr() + 0x14).wrapping_add(1);
            e.mem.set_u32(option.addr() + 0x14, next);
            if next >= e.mem.u32(option.addr() + 0x20) {
                let default = e.mem.u32(option.addr() + 0x18);
                e.mem.set_u32(option.addr() + 0x14, default);
                return;
            }
        }
    }
}

// Translated from 007d1840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Colour option callback for system colour 1 (see `color_option_callback`).
pub fn fn_007d1840(e: &mut Engine, option: Ptr) {
    color_option_callback(e, option, 1);
}

// Translated from 007d1900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Colour option callback for system colour 4.
pub fn fn_007d1900(e: &mut Engine, option: Ptr) {
    color_option_callback(e, option, 4);
}

// Translated from 007d19c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback for a slider between the floats of the settings at
/// `011dab98` (minimum) and `011dab60` (maximum): running, passes
/// `min + (max - min) * iVal / (count - 1)` to `004dc2d0`; otherwise works
/// the option's two words out from `007d1b30` (the float of `011c71ac`) and
/// the value 1.0, both as fractions of the range.
pub fn fn_007d19c0(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let min = setting_float(e, SETTING_MINIMUM_011DAB98);
        let max = setting_float(e, SETTING_MAXIMUM_011DAB60);
        let ratio = slider_ratio(e, option);
        let value = range_value(min, max, ratio);
        e.call(FLOAT_SINK, &args![value]);
    } else {
        let current = fn_007d1b30(e);
        // The difference is stored as a double (FSTP qword), the divisor is
        // formed on the x87 stack.
        let diff = f64::from(current) - f64::from(setting_float(e, SETTING_MINIMUM_011DAB98));
        let max = setting_float(e, SETTING_MAXIMUM_011DAB60);
        let min = setting_float(e, SETTING_MINIMUM_011DAB98);
        let first = (diff / (f64::from(max) - f64::from(min))) as f32;
        let last = last_step(e, option);
        let value = slider_step_index(e, (last * f64::from(first)) as f32);
        e.mem.set_u32(option.addr() + 0x14, value);
        let min = setting_float(e, SETTING_MINIMUM_011DAB98);
        let diff = 1.0 - f64::from(min);
        let max = setting_float(e, SETTING_MAXIMUM_011DAB60);
        let min = setting_float(e, SETTING_MINIMUM_011DAB98);
        let second = (diff / (f64::from(max) - f64::from(min))) as f32;
        let last = last_step(e, option);
        let default = slider_step_index(e, (last * f64::from(second)) as f32);
        e.mem.set_u32(option.addr() + 0x18, default);
    }
}
const SETTING_MINIMUM_011DAB98: u32 = 0x011d_ab98;
const SETTING_MAXIMUM_011DAB60: u32 = 0x011d_ab60;

// Translated from 007d1b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float `00450410` returns for the object at `011c71ac` (ST0).
pub fn fn_007d1b30(e: &mut Engine) -> f32 {
    e.call(FLOAT_AT_OBJECT, &args![FLOAT_OBJECT_011C71AC]).f32()
}

// Translated from 007d1b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback for the integer at `0126efc4`: running, sets the
/// setting at `011c779c` to the option's value (`0045ce80`) and sets menu
/// flag `0x400000` to whether the integer differs from it; otherwise reads
/// the integer into the option as kind 0.
pub fn fn_007d1b40(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let value = e.mem.u32(option.addr() + 0x14);
        e.call(SETTING_SET_INT, &args![SETTING_011C779C, value]);
        let changed = e.global::<u32>(GLOBAL_0126EFC4) != e.mem.u32(option.addr() + 0x14);
        let menu = start_menu_global(e);
        e.call(
            MENU_FLAGS_SET,
            &args![menu, 0x40_0000u32, u32::from(changed)],
        );
    } else {
        let value = e.global::<u32>(GLOBAL_0126EFC4);
        set_option_words(e, option, value, 0);
    }
}

/// The audio-level sliders `007d1bb0`, `007d1d10` and `007d1e60`: between the
/// floats of the settings `min` and `max` (read first, as `float`s).
/// Running: stores `min + (max - min) * iVal / (count - 1)` in the float
/// table at index `index` (`004504f0`) and in the setting `target`
/// (`004ed780`). Otherwise works the option's two words out from
/// `table[index]` (`007d1d00`) and from the setting `default` as fractions
/// of the range.
fn audio_level_slider(
    e: &mut Engine,
    option: Ptr,
    min_setting: u32,
    max_setting: u32,
    index: u32,
    target: u32,
    default_setting: u32,
) {
    let min = setting_float(e, min_setting);
    let max = setting_float(e, max_setting);
    if menu_exists_with_flag(e, 0x20) {
        let ratio = slider_ratio(e, option);
        let value = range_value(min, max, ratio);
        e.call(FLOAT_TABLE_SET, &args![index, value]);
        e.call(FLOAT_SET, &args![target, value]);
    } else {
        let current = fn_007d1d00(e, index as i32);
        let first = range_ratio(f64::from(current), min, max);
        store_slider_step(e, option, 0x14, first);
        let default = setting_float(e, default_setting);
        let second = range_ratio(f64::from(default), min, max);
        store_slider_step(e, option, 0x18, second);
    }
}

// Translated from 007d1bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio-level slider for float table index 3 (settings `011dab1c`,
/// `011daaf4`, target `011c3ec0`, default `011dac38`).
pub fn fn_007d1bb0(e: &mut Engine, option: Ptr) {
    audio_level_slider(
        e,
        option,
        0x011d_ab1c,
        0x011d_aaf4,
        3,
        0x011c_3ec0,
        0x011d_ac38,
    );
}

// Translated from 007d1d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the float table at `011ad7b8` (ST0).
pub fn fn_007d1d00(e: &mut Engine, index: i32) -> f32 {
    e.mem
        .f32(FLOAT_TABLE.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 007d1d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio-level slider for float table index 2 (settings `011dabf0`,
/// `011dac24`, target `011c3d0c`, default `011dabc0`).
pub fn fn_007d1d10(e: &mut Engine, option: Ptr) {
    audio_level_slider(
        e,
        option,
        0x011d_abf0,
        0x011d_ac24,
        2,
        0x011c_3d0c,
        0x011d_abc0,
    );
}

// Translated from 007d1e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio-level slider for float table index 1 (settings `011dab30`,
/// `011dab78`, target `011c3d78`, default `011dab6c`).
pub fn fn_007d1e60(e: &mut Engine, option: Ptr) {
    audio_level_slider(
        e,
        option,
        0x011d_ab30,
        0x011d_ab78,
        1,
        0x011c_3d78,
        0x011d_ab6c,
    );
}

/// The sliders between two settings that store into a third (`007d1fb0`,
/// `007d2510`, `007d2650`): running, `*setting = min + (max - min) * iVal /
/// (count - 1)`; otherwise the option's two words come from the settings
/// `setting` and `default` as fractions of the range.
fn setting_range_slider(
    e: &mut Engine,
    option: Ptr,
    min_setting: u32,
    max_setting: u32,
    setting: u32,
    default_setting: u32,
) {
    let min = setting_float(e, min_setting);
    let max = setting_float(e, max_setting);
    if menu_exists_with_flag(e, 0x20) {
        let ratio = slider_ratio(e, option);
        let value = range_value(min, max, ratio);
        let slot = e.call(FLOAT_SLOT, &args![setting]).u32();
        e.mem.set_f32(slot, value);
    } else {
        let current = setting_float(e, setting);
        let first = range_ratio(f64::from(current), min, max);
        store_slider_step(e, option, 0x14, first);
        let default = setting_float(e, default_setting);
        let second = range_ratio(f64::from(default), min, max);
        store_slider_step(e, option, 0x18, second);
    }
}

// Translated from 007d1fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Range slider (settings `011ca4ac`, `011ca4d8`, stored in `011ca450`,
/// default `011ca4cc`).
pub fn fn_007d1fb0(e: &mut Engine, option: Ptr) {
    setting_range_slider(
        e,
        option,
        0x011c_a4ac,
        0x011c_a4d8,
        0x011c_a450,
        0x011c_a4cc,
    );
}

/// The sliders `007d20f0`, `007d2250` and `007d23b0`: like
/// `setting_range_slider` for `setting`, and running they also store the value
/// in the float at `copy` and its sum with the float of the setting `extra`
/// in the float at `sum`.
fn setting_range_slider_with_copies(
    e: &mut Engine,
    option: Ptr,
    settings: (u32, u32, u32, u32),
    copy: u32,
    extra: u32,
    sum: u32,
) {
    let (min_setting, max_setting, setting, default_setting) = settings;
    let min = setting_float(e, min_setting);
    let max = setting_float(e, max_setting);
    if menu_exists_with_flag(e, 0x20) {
        let ratio = slider_ratio(e, option);
        let value = range_value(min, max, ratio);
        let slot = e.call(FLOAT_SLOT, &args![setting]).u32();
        e.mem.set_f32(slot, value);
        e.mem.set_f32(copy, value);
        let extra_slot = e.call(FLOAT_SLOT, &args![extra]).u32();
        let added = f64::from(value) + f64::from(e.mem.f32(extra_slot));
        e.mem.set_f32(sum, added as f32);
    } else {
        let current = setting_float(e, setting);
        let first = range_ratio(f64::from(current), min, max);
        store_slider_step(e, option, 0x14, first);
        let default = setting_float(e, default_setting);
        let second = range_ratio(f64::from(default), min, max);
        store_slider_step(e, option, 0x18, second);
    }
}

// Translated from 007d20f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Range slider (settings `011c7294`, `011c71b8`, stored in `011c7568`,
/// default `011c7278`; copies in `011f944c` and, plus `011c7400`, `011f9450`).
pub fn fn_007d20f0(e: &mut Engine, option: Ptr) {
    setting_range_slider_with_copies(
        e,
        option,
        (0x011c_7294, 0x011c_71b8, 0x011c_7568, 0x011c_7278),
        0x011f_944c,
        0x011c_7400,
        0x011f_9450,
    );
}

// Translated from 007d2250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Range slider (settings `011c72ac`, `011c74ec`, stored in `011c7740`,
/// default `011c744c`; copies in `011f9444` and, plus `011c76b4`,
/// `011f9448`).
pub fn fn_007d2250(e: &mut Engine, option: Ptr) {
    setting_range_slider_with_copies(
        e,
        option,
        (0x011c_72ac, 0x011c_74ec, 0x011c_7740, 0x011c_744c),
        0x011f_9444,
        0x011c_76b4,
        0x011f_9448,
    );
}

// Translated from 007d23b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Range slider (settings `011c7440`, `011c71d0`, stored in `011c7598`,
/// default `011c7494`; copies in `011f9454` and, plus `011c7634`,
/// `011f9458`).
pub fn fn_007d23b0(e: &mut Engine, option: Ptr) {
    setting_range_slider_with_copies(
        e,
        option,
        (0x011c_7440, 0x011c_71d0, 0x011c_7598, 0x011c_7494),
        0x011f_9454,
        0x011c_7634,
        0x011f_9458,
    );
}

// Translated from 007d2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Range slider (settings `011d86c4`, `011d86ec`, stored in `011d8724`,
/// default `011d870c`).
pub fn fn_007d2510(e: &mut Engine, option: Ptr) {
    setting_range_slider(
        e,
        option,
        0x011d_86c4,
        0x011d_86ec,
        0x011d_8724,
        0x011d_870c,
    );
}

// Translated from 007d2650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Range slider (settings `011d8734`, `011d8718`, stored in `011d8788`,
/// default `011d8770`).
pub fn fn_007d2650(e: &mut Engine, option: Ptr) {
    setting_range_slider(
        e,
        option,
        0x011d_8734,
        0x011d_8718,
        0x011d_8788,
        0x011d_8770,
    );
}

/// The audio volume sliders `007d2790` .. `007d2d30`. `option` may be null:
/// then the audio object's setter is called with the setting's value.
/// Running: sets the setting to `iVal / (count - 1)` and passes the
/// setting's value to the audio object's setter. Otherwise works the
/// option's two words out from the audio object's getter (`+0x14`) and the
/// constant each slider multiplies `count - 1` by for `+0x18`: `second` is
/// `None` for `FLD1` (the instruction's 1.0), or the address of the
/// `double` in the exe's data that `FMUL double ptr [second]` reads
/// (`01011588` or `010290b8`), read from memory after the `+0x14` step.
fn audio_volume_slider(
    e: &mut Engine,
    option: Ptr,
    setting: u32,
    setter: u32,
    getter: u32,
    second: Option<u32>,
) {
    if option.is_null() {
        let volume = e.call(FLOAT_AT_OBJECT, &args![setting]).f32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(setter, &args![audio, volume]);
    } else if menu_exists_with_flag(e, 0x20) {
        let ratio = slider_ratio(e, option);
        let object = e.call(FLOAT_SET, &args![setting, ratio]).u32();
        let volume = e.call(FLOAT_AT_OBJECT, &args![object]).f32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(setter, &args![audio, volume]);
    } else {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let volume = e.call(getter, &args![audio]).f32();
        store_slider_step(e, option, 0x14, volume);
        let ratio = match second {
            None => 1.0,
            Some(address) => e.global::<f64>(address),
        };
        store_slider_step_f64(e, option, 0x18, ratio);
    }
}

// Translated from 007d2790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio volume slider on the setting at `011f6e14` (audio setter
/// `00ad7b10`, getter `00ad79f0`); `+0x18` from `count - 1` times 1.0.
pub fn fn_007d2790(e: &mut Engine, option: Ptr) {
    // 007d2889: FLD1; FMULP on (count - 1).
    audio_volume_slider(e, option, 0x011f_6e14, 0x00ad_7b10, 0x00ad_79f0, None);
}

// Translated from 007d28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio volume slider on the setting at `011f6e20` (setter `00ad7b40`,
/// getter `00ad7a20`); `+0x18` from `count - 1` times the double at
/// `01011588`.
pub fn fn_007d28b0(e: &mut Engine, option: Ptr) {
    // 007d29a9: FMUL double ptr [01011588] on (count - 1).
    audio_volume_slider(
        e,
        option,
        0x011f_6e20,
        0x00ad_7b40,
        0x00ad_7a20,
        Some(DOUBLE_HALF),
    );
}

// Translated from 007d29d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio volume slider on the setting at `011f6e44` (setter `00ad7bd0`,
/// getter `00ad7ab0`); `+0x18` from `count - 1` times the double at
/// `010290b8`.
pub fn fn_007d29d0(e: &mut Engine, option: Ptr) {
    // 007d2ac9: FMUL double ptr [010290b8] on (count - 1).
    audio_volume_slider(
        e,
        option,
        0x011f_6e44,
        0x00ad_7bd0,
        0x00ad_7ab0,
        Some(DOUBLE_SIX_TENTHS),
    );
}

// Translated from 007d2af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio volume slider on the setting at `011f6e2c` (setter `00ad7b70`,
/// getter `00ad7a50`); `+0x18` from `count - 1` times the double at
/// `010290b8`.
pub fn fn_007d2af0(e: &mut Engine, option: Ptr) {
    // 007d2be9: FMUL double ptr [010290b8] on (count - 1).
    audio_volume_slider(
        e,
        option,
        0x011f_6e2c,
        0x00ad_7b70,
        0x00ad_7a50,
        Some(DOUBLE_SIX_TENTHS),
    );
}

// Translated from 007d2c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio volume slider on the setting at `011f6e38` (setter `00ad7ba0`,
/// getter `00ad7a80`); `+0x18` from `count - 1` times 1.0.
pub fn fn_007d2c10(e: &mut Engine, option: Ptr) {
    // 007d2d09: FLD1; FMULP on (count - 1).
    audio_volume_slider(e, option, 0x011f_6e38, 0x00ad_7ba0, 0x00ad_7a80, None);
}

// Translated from 007d2d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::SetRadioVol` (Xbox PDB): the audio volume slider on the
/// setting at `011f6e50` (setter `00ad7c00`, getter `00ad7ae0`); `+0x18`
/// from `count - 1` times the double at `01011588`.
pub fn start_menu_set_radio_vol(e: &mut Engine, option: Ptr) {
    // 007d2e29: FMUL double ptr [01011588] on (count - 1).
    audio_volume_slider(
        e,
        option,
        0x011f_6e50,
        0x00ad_7c00,
        0x00ad_7ae0,
        Some(DOUBLE_HALF),
    );
}

// Translated from 007d2e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Boolean option callback for the setting at `011e0a5c` (kind 0).
pub fn fn_007d2e50(e: &mut Engine, option: Ptr) {
    boolean_option_callback(e, option, SETTING_011E0A5C, 0);
}

// Translated from 007d2eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback for the slider on the setting at `011e0a6c`
/// between the doubles `0.0005` and `0.0005 + 0.0095` (exe data): running,
/// stores `iVal / (count - 1) * 0.0095 + 0.0005` (rounded to a double, then
/// a float) in the setting; otherwise works the option's two words out from
/// the setting's value and from the float at `01076f60`.
pub fn fn_007d2eb0(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let ratio = slider_ratio(e, option);
        let scale = e.global::<f64>(DOUBLE_SLIDER_SCALE);
        let offset = e.global::<f64>(DOUBLE_SLIDER_OFFSET);
        let value = f64::from(ratio) * scale + offset;
        let slot = e.call(FLOAT_SLOT, &args![SETTING_011E0A6C]).u32();
        e.mem.set_f32(slot, value as f32);
    } else {
        let slot = e.call(FLOAT_SLOT, &args![SETTING_011E0A6C]).u32();
        let current = f64::from(e.mem.f32(slot));
        let scale = e.global::<f64>(DOUBLE_SLIDER_SCALE);
        let offset = e.global::<f64>(DOUBLE_SLIDER_OFFSET);
        let first = ((current - offset) / scale) as f32;
        let full = e.global::<f32>(FLOAT_SLIDER_FULL);
        store_slider_steps(e, option, first, full);
    }
}

// Translated from 007d2fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback for the "boolean via the interface manager"
/// option. Running: passes `value != 1` to `00719740` on the interface
/// manager and, when the tile at `+0x1cc` of the menu exists, sets its
/// enabled trait (`+0x1d0`) to `value == 1`. Otherwise reads the setting at
/// `011d8c4c` (`007d30d0`) into the option (inverted), copies it into the
/// main object's `00877720` result (`005407b0`), and sets the byte of
/// `007d30b0` to its inverse; kind 1.
pub fn fn_007d2fc0(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let on = e.mem.u32(option.addr() + 0x14) == 1;
        let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
        e.call(FN_00719740, &args![manager, u32::from(!on)]);
        let menu = start_menu_global(e);
        let tile = e.mem.u32(menu.addr() + 0x1cc);
        if tile != 0 {
            let menu = start_menu_global(e);
            let enabled_trait = e.mem.u32(menu.addr() + 0x1d0);
            e.call(TILE_SET_INT, &args![tile, enabled_trait, u32::from(on)]);
        }
    } else {
        e.call(INTERFACE_MANAGER_GETTER, &args![]);
        let value = fn_007d30d0(e);
        e.mem.set_u32(option.addr() + 0x14, u32::from(value == 0));
        e.call(INTERFACE_MANAGER_GETTER, &args![]);
        let value = fn_007d30d0(e);
        let main = e.global::<u32>(MAIN_OBJECT);
        let target = e.call(INPUT_OBJECT_GETTER, &args![main]).u32();
        e.call(FN_005407B0, &args![target, u32::from(value)]);
        let main = e.global::<u32>(MAIN_OBJECT);
        e.call(INPUT_OBJECT_GETTER, &args![main]);
        e.call(INTERFACE_MANAGER_GETTER, &args![]);
        let value = fn_007d30d0(e);
        fn_007d30b0(e, u8::from(value == 0));
        e.mem.set_u32(option.addr() + 0x18, 1);
    }
}

// Translated from 007d30b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x1b88` of the object at `011f35cc`, if
/// there is one.
pub fn fn_007d30b0(e: &mut Engine, value: u8) {
    if e.global::<u32>(OBJECT_011F35CC) != 0 {
        let object = e.global::<u32>(OBJECT_011F35CC);
        e.mem.set_u8(object + 0x1b88, value);
    }
}

// Translated from 007d30d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte `00454af0` returns for the setting at `011d8c4c`.
pub fn fn_007d30d0(e: &mut Engine) -> u8 {
    e.call(SETTING_GET_BOOL, &args![SETTING_011D8C4C]).u8()
}

// Translated from 007d30e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option change callback for the setting at `011e0850`: running, sets the
/// setting from the option and passes its byte to `007d30b0`; otherwise
/// reads the setting into the option and its second word from the byte at
/// `+0x85` of the object `00a25920` returns (`007d3170`).
pub fn fn_007d30e0(e: &mut Engine, option: Ptr) {
    if menu_exists_with_flag(e, 0x20) {
        let on = e.mem.u32(option.addr() + 0x14) == 1;
        let setting = e
            .call(SETTING_SET_BOOL, &args![SETTING_011E0850, u32::from(on)])
            .u32();
        let value = e.call(SETTING_GET_BOOL, &args![setting]).u8();
        fn_007d30b0(e, value);
    } else {
        let on = e.call(SETTING_GET_BOOL, &args![SETTING_011E0850]).bool();
        e.mem.set_u32(option.addr() + 0x14, u32::from(on));
        let object = e.call(DLC_GETTER, &args![]).u32();
        let flag = fn_007d3170(e, Ptr::new(object));
        e.mem.set_u32(option.addr() + 0x18, u32::from(flag != 0));
    }
}

// Translated from 007d3170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x85` of the object.
pub fn fn_007d3170(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x85)
}

/// The two-line message `ConfirmContinue` and `ConfirmNewGame` show:
/// `ShowMessage(title, 0, 0, 0, 0, 5, 0.0, 0.0, text, 0)` with the setting
/// strings of `SETTING_BINDING_TEXT` (text) and `title_setting`.
fn show_save_message(e: &mut Engine, title_setting: u32) {
    let text = e.call(SETTING_STRING, &args![SETTING_BINDING_TEXT]).u32();
    let title = e.call(SETTING_STRING, &args![title_setting]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![title, 0u32, 0u32, 0u32, 0u32, 5u32, 0.0f32, 0.0f32, text, 0u32],
    );
}

/// The first list node's item word of the save manager at `011de134`:
/// `FIRST_WORD(manager)` is the node, `NODE_ITEM(node)` the address of its
/// item. Returns the node (0 when the list is empty).
fn save_manager_first_node(e: &mut Engine) -> u32 {
    let manager = e.global::<u32>(CALLBACK_OWNER);
    e.call(FIRST_WORD, &args![manager]).u32()
}

/// The item of the manager's first node.
fn save_manager_first_item(e: &mut Engine) -> u32 {
    let node = save_manager_first_node(e);
    let item = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(item)
}

// Translated from 007d3190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::ConfirmContinue` (Xbox PDB): when the save manager (`011de134`)
/// has no first node or that node's item is 0, calls `00851330` on it; then,
/// with a first item that is not corrupt (`00846b30`), runs the archive
/// clean-up and `008467b0` on it; with a corrupt one or none shows the
/// message; always ends with `ChooseBack`.
pub fn start_menu_confirm_continue(e: &mut Engine) {
    let mut empty = save_manager_first_node(e) == 0;
    if !empty {
        empty = save_manager_first_item(e) == 0;
    }
    if empty {
        let manager = e.global::<u32>(CALLBACK_OWNER);
        e.call(FN_00851330, &args![manager]);
    }
    let mut shown = false;
    if save_manager_first_node(e) != 0 {
        let item = save_manager_first_item(e);
        if item != 0 {
            let entry = save_manager_first_item(e);
            if e.call(SAVE_ENTRY_IS_CORRUPT, &args![entry]).bool() {
                show_save_message(e, SETTING_011D1F14);
            } else {
                e.call(ARCHIVE_CLEANUP, &args![]);
                let entry = save_manager_first_item(e);
                e.call(FN_008467B0, &args![entry]);
            }
            shown = true;
        }
    }
    if !shown {
        show_save_message(e, SETTING_011D48AC);
    }
    start_menu_choose_back(e);
}

// Translated from 007d32b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::ConfirmNewGame` (Xbox PDB): looks up the form whose id is the
/// hexadecimal text of the setting at `011dac00` (`strtol(text, &end, 16)`
/// then `004839c0`); the object at `011dea3c` gets `005d1530(0)` and
/// `005d14d0(1)`. A form of type byte `0x47` is cast from `TESForm` to
/// `TESQuest`; with no quest it logs `QUESTS: Unable to find character
/// generation quest` and calls `ChooseBack`. With a quest: closes the menu,
/// runs the archive clean-up, sets `011d8907`, calls `007adcc0(0)` on the
/// object at `011dea3c`, casts its `GetForm` result from `TESBoundObject`
/// to `TESNPC`, stores the setting `011d4f78`'s text through `00489100` at
/// `+0xd0`, starts the quest (`0060c9c0(1)`), shows the loading tile,
/// resets the audio and the dialog flag and finishes the DLC flags and the
/// two tutorial messages.
pub fn start_menu_confirm_new_game(e: &mut Engine) {
    let text = e.call(SETTING_STRING, &args![SETTING_011DAC00]).u32();
    let value = e.with_stack(4, |e, end| e.call(STRTOL, &args![text, end, 0x10u32]).u32());
    let form = e.call(LOOKUP_FORM, &args![value]).u32();
    let mut quest = 0;
    let player = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
    if player != 0 {
        e.call(FN_005D1530, &args![player, 0u32]);
        let player = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
        e.call(FN_005D14D0, &args![player, 1u32]);
    }
    if form != 0 && e.call(FN_00401170, &args![form]).u32() == 0x47 {
        quest = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    form,
                    0u32,
                    TYPE_DESCRIPTOR_FORM,
                    TYPE_DESCRIPTOR_QUEST,
                    0u32
                ],
            )
            .u32();
    }
    if quest == 0 {
        let text = e.call(SETTING_STRING, &args![SETTING_011DAC00]).u32();
        e.call(LOG_MESSAGE, &args![TEXT_QUEST_NOT_FOUND, text]);
        start_menu_choose_back(e);
        return;
    }
    start_menu_close(e, 0);
    e.call(ARCHIVE_CLEANUP, &args![]);
    e.mem.set_u8(FLAG_011D8907, 1);
    let player = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
    e.call(FN_007ADCC0, &args![player, 0u32]);
    let player = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
    let base = e.call(GET_FORM, &args![player]).u32();
    let npc = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                base,
                0u32,
                TYPE_DESCRIPTOR_BOUND_OBJECT,
                TYPE_DESCRIPTOR_NPC,
                0u32
            ],
        )
        .u32();
    let sound = e.call(OPTION_TEXT, &args![SETTING_011D4F78]).u32();
    e.call(SET_STRING_FIELD, &args![npc.wrapping_add(0xd0), sound]);
    e.call(FN_0060C9C0, &args![quest, 1u32]);
    let loading = e.global::<u32>(LOADING_MENU);
    let tile = e.mem.u32(loading + 0x54);
    e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, 1u32]);
    e.call(FN_004581C0, &args![1u32]);
    e.call(ERROR_CALL, &args![]);
    e.call(SET_DIALOG_FLAG, &args![0u32]);
    e.call(MEDIA_CLOSE, &args![]);
    if e.call(DLC_GETTER, &args![]).u32() != 0 {
        let object = e.call(DLC_GETTER, &args![]).u32();
        fn_007d0bd0(e, Ptr::new(object), 1);
        let object = e.call(DLC_GETTER, &args![]).u32();
        fn_007d0bb0(e, Ptr::new(object));
    }
    e.call(FN_00788A90, &args![1u32]);
    let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
    e.call(
        TUTORIAL_SHOW_MESSAGE,
        &args![
            manager.wrapping_add(HELP_TABLE_OFFSET),
            0x19u32,
            0x3ebu32,
            0x200u32
        ],
    );
    let manager = e.call(INTERFACE_MANAGER_GETTER, &args![]).u32();
    e.call(
        TUTORIAL_SHOW_MESSAGE,
        &args![
            manager.wrapping_add(HELP_TABLE_OFFSET),
            0x1au32,
            0x3ffu32,
            0x200u32
        ],
    );
}
/// `00489100(this, text)` (`RET 4`), called on the NPC's field at `+0xd0`
/// (Ghidra name `TESSoundFile::SetSoundFile`, unconfirmed).
const SET_STRING_FIELD: u32 = 0x0048_9100;

// Translated from 007d3470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Continues with the save game the menu holds (`+0x1b0`): runs the archive
/// clean-up and, with the player object running and flag 1 of the start
/// menu, tells the texture owner (`011dea10`, `00457d70(1, 0, 0)`) when the
/// location name of the NPC's `+0xd0` (`00408da0`) is found by `00408b20`
/// in the save entry's `0075f8c0` word; then loads the entry (`008467b0`)
/// and either closes the start menu (flag 1 set and loaded) or backs out.
pub fn fn_007d3470(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    if e.mem.u32(menu.addr() + 0x1b0) == 0 {
        return;
    }
    e.call(ARCHIVE_CLEANUP, &args![]);
    let menu = start_menu_global(e);
    let save = e.mem.u32(menu.addr() + 0x1b0);
    let player = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
    if player != 0 && e.call(START_MENU_FLAG_1, &args![]).bool() {
        let player = e.global::<u32>(GLOBAL_OBJECT_011DEA3C);
        let base = e.call(GET_FORM, &args![player]).u32();
        let npc = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    base,
                    0u32,
                    TYPE_DESCRIPTOR_BOUND_OBJECT,
                    TYPE_DESCRIPTOR_NPC,
                    0u32
                ],
            )
            .u32();
        if npc != 0 && e.call(FN_00408DA0, &args![npc.wrapping_add(0xd0)]).u32() != 0 {
            let place = e.call(FN_0075F8C0, &args![save]).u32();
            if place != 0 {
                let place = e.call(FN_0075F8C0, &args![save]).u32();
                let name = e.call(FN_00408DA0, &args![npc.wrapping_add(0xd0)]).u32();
                if e.call(FN_00408B20, &args![name, place]).u32() != 0 {
                    let owner = e.global::<u32>(TEXTURE_OWNER);
                    e.call(TEXTURE_OWNER_CALL, &args![owner, 1u32, 0u32, 0u32]);
                }
            }
        }
    }
    let loaded = e.call(FN_008467B0, &args![save]).u8();
    let menu = start_menu_global(e);
    if !menu.is_null() && menu_flag(e, menu, 1) && loaded != 0 {
        start_menu_close(e, 1);
        return;
    }
    if loaded == 0 {
        let menu = start_menu_global(e);
        if !menu.is_null() {
            // Both outcomes of this test lead to ChooseBack.
            menu_flag(e, menu, 0x2000_0000);
        }
    }
    start_menu_choose_back(e);
}

// Translated from 007d35c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Choose function: with a start menu, calls slot `0x1c` of the confirm
/// list (`+0xe4`), sets the text of the tile at `+0x48` to the setting string
/// of `011d4930` and sets menu flag `0x4000`.
pub fn fn_007d35c0(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    e.vcall(menu.addr() + 0xe4, 0x1c, &args![]);
    let text = e.call(SETTING_STRING, &args![SETTING_SAVE_SELECTED]).u32();
    let menu = start_menu_global(e);
    let tile = e.mem.u32(menu.addr() + 0x48);
    e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
    let menu = start_menu_global(e);
    e.call(MENU_FLAGS_SET, &args![menu, 0x4000u32, 1u32]);
}

/// The save entry and the entry object the save-load list holds at `index`
/// (`vcall` slot `0x14` and `007d86c0`); both non-null means the index is
/// usable.
fn save_entry_pair(e: &mut Engine, index: u32) -> (u32, u32) {
    let menu = start_menu_global(e);
    let item = e
        .vcall(menu.addr() + 0x174, 0x14, &args![index, 0u32])
        .u32();
    let menu = start_menu_global(e);
    let entry = e
        .call(FN_007D86C0, &args![menu.byte_add(0x174), index, 0u32])
        .u32();
    (item, entry)
}

// Translated from 007d3620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007d3620 - the /GS stack cookie (security cookie xor and check call 00ec408c) is not translated
/// Deletes the highlighted save game of the save-load list. With the list
/// holding a highlighted item: removes the save (`008467e0` on the
/// highlighted entry), removes the item from the list, backs out, and works
/// out which entry the list shows next (`007d8700` / `007d87b0`): with
/// none, refreshes the menu and clears flag `0x40000`; with the list at its
/// limit and the saved threshold (`+0x1ac`) equal to the scrollbar's
/// current value, redraws the item after the last one (the loop that drops
/// autosaves from the count when flag 8 is set); otherwise lowers the
/// threshold by one. Finally, with flag 8 set and entries left, writes the
/// "n/1000" count line into the list's tile (`sprintf` of the settings
/// `011d2c4c` and `011d25d4`).
pub fn fn_007d3620(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    if e.vcall(menu.addr() + 0x174, 4, &args![]).u32() == 0 {
        return;
    }
    let menu = start_menu_global(e);
    let list = menu.addr() + 0x174;
    let entry = e.call(LIST_BOX_HIGHLIGHTED_ITEM, &args![list]).u32();
    e.call(REMOVE_SAVE_GAME, &args![entry]);
    let item = e.vcall(list, 4, &args![]).u32();
    e.call(FN_007D8040, &args![list, item]);
    start_menu_choose_back(e);
    let mut remaining: i32 = -1;
    let count = e.call(FN_007D8700, &args![list]).u32();
    let limit = e.call(FN_007D87B0, &args![list]).u32();
    if count == 0 {
        let menu = start_menu_global(e);
        e.call(FN_007D62E0, &args![menu]);
        let menu = start_menu_global(e);
        e.call(MENU_FLAGS_SET, &args![menu, 0x40000u32, 0u32]);
        start_menu_choose_back(e);
    } else {
        let mut redrawn = false;
        if count >= limit {
            let trait_id = e
                .call(
                    TILE_ADD_USER_TRAIT,
                    &args![TEXT_CURRENT_VALUE, 0xffff_ffffu32],
                )
                .u32();
            let scrollbar = e.call(FN_00825C00, &args![list]).u32();
            let current = e.call(TILE_GET_VALUE, &args![scrollbar, trait_id]).f64();
            let menu = start_menu_global(e);
            let threshold = e.get(menu.cast::<StartMenu>(), StartMenu::fSaveLoadThreshold);
            // FUCOMPP then `TEST AH,0x44; JP`: only an ordered equal result
            // continues.
            if f64::from(threshold) == current {
                let menu = start_menu_global(e);
                let threshold = e.get(menu.cast::<StartMenu>(), StartMenu::fSaveLoadThreshold);
                let position =
                    f64::from(limit) + f64::from(threshold) - e.global::<f64>(DOUBLE_ONE);
                let index = e.call(FLOAT_TO_INT, &args![position]).i32();
                e.call(
                    LIST_BOX_FOR_EACH,
                    &args![
                        list,
                        INITIALIZE_SAVE_LOAD_LIST_ITEM,
                        index as u32,
                        index.wrapping_add(1) as u32
                    ],
                );
                let menu = start_menu_global(e);
                remaining = e.call(LIST_BOX_COUNT, &args![menu.byte_add(0x174)]).u32() as i32;
                if menu_exists_with_flag(e, 8) {
                    let mut i = 0u32;
                    loop {
                        let menu = start_menu_global(e);
                        if i >= e.call(LIST_BOX_COUNT, &args![menu.byte_add(0x174)]).u32() {
                            break;
                        }
                        let (item, entry) = save_entry_pair(e, i);
                        if item != 0 && entry != 0 {
                            let drop_it = e.call(IS_AUTOSAVE, &args![entry]).bool()
                                || e.call(FN_00846B10, &args![entry]).bool();
                            if drop_it {
                                let menu = start_menu_global(e);
                                e.call(FN_007D80F0, &args![menu.byte_add(0x174), i, 0u32]);
                                remaining = remaining.wrapping_sub(1);
                            }
                        }
                        i += 1;
                    }
                }
                redrawn = true;
            }
        }
        if !redrawn {
            let menu = start_menu_global(e);
            let threshold = e.get(menu.cast::<StartMenu>(), StartMenu::fSaveLoadThreshold);
            // FCOMP against zero, `TEST AH,0x44; JNP`: only an ordered
            // equal result skips the decrement.
            if f64::from(threshold) != e.global::<f64>(DOUBLE_ZERO) {
                let menu = start_menu_global(e);
                let threshold = e.get(menu.cast::<StartMenu>(), StartMenu::fSaveLoadThreshold);
                let lowered = f64::from(threshold) - e.global::<f64>(DOUBLE_ONE);
                let menu = start_menu_global(e);
                e.set(
                    menu.cast::<StartMenu>(),
                    StartMenu::fSaveLoadThreshold,
                    lowered as f32,
                );
            }
        }
    }
    if !menu_exists_with_flag(e, 8) {
        return;
    }
    let menu = start_menu_global(e);
    if e.call(LIST_BOX_COUNT, &args![menu.byte_add(0x174)]).u32() == 0 {
        return;
    }
    if remaining == -1 {
        let menu = start_menu_global(e);
        remaining = e.call(LIST_BOX_COUNT, &args![menu.byte_add(0x174)]).u32() as i32;
        let mut i = 0u32;
        loop {
            let menu = start_menu_global(e);
            if i >= e.call(LIST_BOX_COUNT, &args![menu.byte_add(0x174)]).u32() {
                break;
            }
            let (item, entry) = save_entry_pair(e, i);
            if item != 0 && entry != 0 {
                let drop_it = e.call(IS_AUTOSAVE, &args![entry]).bool()
                    || e.call(FN_00846B10, &args![entry]).bool();
                if drop_it {
                    remaining = remaining.wrapping_sub(1);
                }
            }
            i += 1;
        }
    }
    remaining = remaining.wrapping_sub(1);
    let capacity_text = e.call(SETTING_STRING, &args![SETTING_011D25D4]).u32();
    let label = e.call(SETTING_STRING, &args![SETTING_011D2C4C]).u32();
    e.with_stack(0x40, |e, buffer| {
        e.call(
            STRING_COPY_NARROW,
            &args![
                buffer,
                0x40u32,
                FORMAT_SAVE_COUNT,
                label,
                remaining as u32,
                1000u32,
                capacity_text
            ],
        );
        let menu = start_menu_global(e);
        let tile = e.call(FN_007D6E90, &args![menu.byte_add(0x174)]).u32();
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, buffer, 1u32]);
    });
    let menu = start_menu_global(e);
    let tile = e.call(FN_007D6E90, &args![menu.byte_add(0x174)]).u32();
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

// Translated from 007d3a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Message callback (`MESSAGE_CALLBACK`): when a start menu exists and the
/// message menu's result byte is 1, clears the controller binding of the
/// remap device (`00a24b70` on the main object's `00877720` result), sets
/// menu flag 2 and applies `007cfe40` to every item of the options list.
pub fn fn_007d3a30(e: &mut Engine) {
    let menu = start_menu_global(e);
    if menu.is_null() {
        return;
    }
    let result = e.call(MESSAGE_MENU_RESULT, &args![]).u8() as i8;
    if result != 1 {
        return;
    }
    let menu = start_menu_global(e);
    let device = e.get(menu.cast::<StartMenu>(), StartMenu::iRemapDevice);
    let main = e.global::<u32>(MAIN_OBJECT);
    let target = e.call(INPUT_OBJECT_GETTER, &args![main]).u32();
    e.call(FN_00A24B70, &args![target, device]);
    let menu = start_menu_global(e);
    e.call(MENU_FLAGS_SET, &args![menu, 2u32, 1u32]);
    let menu = start_menu_global(e);
    e.call(
        LIST_BOX_FOR_EACH,
        &args![menu.byte_add(0x114), FN_007CFE40, 0u32, 0x7fff_ffffu32],
    );
}
/// `Interface::GetMessageMenuResult` (Ghidra name, `00703fa0`, returns a
/// signed byte).
const MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00723030, list_box_belongs_to(Ptr<ListBox>, u32) -> bool),
        entry!(0x007269d0, list_box_init_tile_position(Ptr<ListBox>, Ptr)),
        entry!(0x007a22d0, list_box_get_tile_for(Ptr<ListBox>, Ptr) -> Ptr),
        entry!(
            0x007a30a0,
            list_box_get_tile_at(Ptr<ListBox>, u32, u8) -> Ptr
        ),
        entry!(0x007a5e70, list_box_scroll_to_highlight(Ptr<ListBox>)),
        entry!(0x007a6fa0, list_box_get_at(Ptr<ListBox>, u32, u8) -> Ptr),
        entry!(0x007cb4d0, start_menu_get_class(Ptr<StartMenu>) -> u32),
        entry!(
            0x007cb4e0,
            start_menu_attach_tile_by_id(Ptr<StartMenu>, i32, Ptr)
        ),
        entry!(
            0x007cb510,
            start_menu_scalar_deleting_destructor(Ptr<StartMenu>, u32) -> Ptr<StartMenu>
        ),
        entry!(0x007cb540, fn_007cb540(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007cb560, fn_007cb560(Ptr<ListBox>)),
        entry!(0x007cb580, fn_007cb580(Ptr<ListBox>, u32) -> Ptr<ListBox>),
        entry!(0x007cb5b0, fn_007cb5b0(Ptr<StartMenu>)),
        entry!(0x007cb710, fn_007cb710() -> Ptr),
        entry!(0x007cb7a0, fn_007cb7a0()),
        entry!(0x007cb7d0, start_menu_create(u8, u8) -> bool),
        entry!(0x007cbaf0, start_menu_create_inner(Ptr<StartMenu>, u8, u8)),
        entry!(0x007cc5d0, fn_007cc5d0(Ptr, u32) -> bool),
        entry!(0x007cc5f0, fn_007cc5f0(u32) -> bool),
        entry!(0x007cc680, fn_007cc680(Ptr, u8)),
        entry!(0x007cc6d0, fn_007cc6d0() -> u32),
        entry!(0x007cc6e0, start_menu_create_menu_elements()),
        entry!(0x007ce620, fn_007ce620(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x007ce660,
            start_menu_option_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x007ce690, fn_007ce690(Ptr)),
        entry!(0x007ce6b0, fn_007ce6b0()),
        entry!(0x007ce7a0, start_menu_close(u8)),
        entry!(
            0x007ce8f0,
            start_menu_waiting_for_control_remap_button_press(Ptr) -> bool
        ),
        entry!(0x007ce970, fn_007ce970(u32, u32) -> u32),
        entry!(0x007ce9b0, start_menu_do_click(Ptr<StartMenu>, i32, Ptr)),
        entry!(
            0x007cefc0,
            start_menu_do_gamepad(Ptr<StartMenu>, i32, u32) -> bool
        ),
        entry!(0x007cf5c0, fn_007cf5c0(Ptr)),
        entry!(
            0x007cf5e0,
            start_menu_do_gamepad_up_event(Ptr<StartMenu>, i32, u32) -> bool
        ),
        entry!(
            0x007cf670,
            start_menu_do_down_click(Ptr<StartMenu>, i32, Ptr)
        ),
        entry!(
            0x007cf6a0,
            start_menu_do_still_down(Ptr<StartMenu>, i32, Ptr)
        ),
        entry!(0x007cf7e0, start_menu_do_enter(Ptr<StartMenu>, u32, Ptr)),
        entry!(0x007cf8e0, start_menu_do_leave(Ptr<StartMenu>, i32, Ptr)),
        entry!(0x007cf9a0, start_menu_do_idle(Ptr<StartMenu>)),
        entry!(0x007cfd00, fn_007cfd00(Ptr) -> u8),
        entry!(
            0x007cfd20,
            start_menu_update_can_load_status(Ptr<StartMenu>)
        ),
        entry!(0x007cfd60, start_menu_do_key(Ptr<StartMenu>, u32) -> bool),
        entry!(0x007cfdc0, start_menu_reset_to_defaults(u32, Ptr)),
        entry!(0x007cfe40, fn_007cfe40(Ptr, Ptr)),
        entry!(0x007d01b0, fn_007d01b0(Ptr<StartMenu>)),
        entry!(0x007d03a0, fn_007d03a0(u32) -> bool),
        entry!(0x007d0440, fn_007d0440()),
        entry!(0x007d0490, fn_007d0490()),
        entry!(0x007d04d0, fn_007d04d0()),
        entry!(0x007d0550, fn_007d0550()),
        entry!(0x007d0660, fn_007d0660(Ptr) -> Ptr),
        entry!(0x007d0680, fn_007d0680()),
        entry!(0x007d06c0, fn_007d06c0()),
        entry!(0x007d0700, start_menu_choose_settings()),
        entry!(0x007d0770, start_menu_choose_help()),
        entry!(0x007d09c0, fn_007d09c0(Ptr, u32) -> u32),
        entry!(0x007d0a10, fn_007d0a10()),
        entry!(0x007d0a70, start_menu_choose_main_menu()),
        entry!(0x007d0b90, fn_007d0b90(Ptr)),
        entry!(0x007d0bb0, fn_007d0bb0(Ptr)),
        entry!(0x007d0bd0, fn_007d0bd0(Ptr, u8)),
        entry!(0x007d0bf0, fn_007d0bf0()),
        entry!(0x007d0c50, fn_007d0c50()),
        entry!(0x007d0cb0, fn_007d0cb0()),
        entry!(0x007d0d40, fn_007d0d40()),
        entry!(0x007d0d70, fn_007d0d70()),
        entry!(0x007d0dc0, fn_007d0dc0()),
        entry!(0x007d0df0, fn_007d0df0()),
        entry!(0x007d0e40, start_menu_choose_back()),
        entry!(0x007d11d0, fn_007d11d0(Ptr)),
        entry!(0x007d1240, fn_007d1240(Ptr, u32)),
        entry!(0x007d1260, fn_007d1260(Ptr)),
        entry!(0x007d12d0, fn_007d12d0(Ptr, u32)),
        entry!(0x007d1320, fn_007d1320(Ptr, u32)),
        entry!(0x007d1340, fn_007d1340(Ptr) -> u32),
        entry!(0x007d1360, fn_007d1360(Ptr) -> bool),
        entry!(0x007d13a0, fn_007d13a0(Ptr)),
        entry!(0x007d1400, fn_007d1400(Ptr, u32)),
        entry!(0x007d1440, fn_007d1440(Ptr)),
        entry!(0x007d14c0, fn_007d14c0(u32, u32)),
        entry!(0x007d1510, fn_007d1510(Ptr)),
        entry!(0x007d1570, fn_007d1570(Ptr)),
        entry!(0x007d15d0, fn_007d15d0(Ptr)),
        entry!(0x007d1630, fn_007d1630(Ptr)),
        entry!(0x007d1690, fn_007d1690(Ptr)),
        entry!(0x007d16f0, fn_007d16f0(Ptr)),
        entry!(0x007d1750, fn_007d1750(Ptr)),
        entry!(0x007d1840, fn_007d1840(Ptr)),
        entry!(0x007d1900, fn_007d1900(Ptr)),
        entry!(0x007d19c0, fn_007d19c0(Ptr)),
        entry!(0x007d1b30, fn_007d1b30() -> f32),
        entry!(0x007d1b40, fn_007d1b40(Ptr)),
        entry!(0x007d1bb0, fn_007d1bb0(Ptr)),
        entry!(0x007d1d00, fn_007d1d00(i32) -> f32),
        entry!(0x007d1d10, fn_007d1d10(Ptr)),
        entry!(0x007d1e60, fn_007d1e60(Ptr)),
        entry!(0x007d1fb0, fn_007d1fb0(Ptr)),
        entry!(0x007d20f0, fn_007d20f0(Ptr)),
        entry!(0x007d2250, fn_007d2250(Ptr)),
        entry!(0x007d23b0, fn_007d23b0(Ptr)),
        entry!(0x007d2510, fn_007d2510(Ptr)),
        entry!(0x007d2650, fn_007d2650(Ptr)),
        entry!(0x007d2790, fn_007d2790(Ptr)),
        entry!(0x007d28b0, fn_007d28b0(Ptr)),
        entry!(0x007d29d0, fn_007d29d0(Ptr)),
        entry!(0x007d2af0, fn_007d2af0(Ptr)),
        entry!(0x007d2c10, fn_007d2c10(Ptr)),
        entry!(0x007d2d30, start_menu_set_radio_vol(Ptr)),
        entry!(0x007d2e50, fn_007d2e50(Ptr)),
        entry!(0x007d2eb0, fn_007d2eb0(Ptr)),
        entry!(0x007d2fc0, fn_007d2fc0(Ptr)),
        entry!(0x007d30b0, fn_007d30b0(u8)),
        entry!(0x007d30d0, fn_007d30d0() -> u8),
        entry!(0x007d30e0, fn_007d30e0(Ptr)),
        entry!(0x007d3170, fn_007d3170(Ptr) -> u8),
        entry!(0x007d3190, start_menu_confirm_continue()),
        entry!(0x007d32b0, start_menu_confirm_new_game()),
        entry!(0x007d3470, fn_007d3470()),
        entry!(0x007d35c0, fn_007d35c0()),
        entry!(0x007d3620, fn_007d3620()),
        entry!(0x007d3a30, fn_007d3a30()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Values = Rc<RefCell<HashMap<(u32, u32), f32>>>;

    /// Every callee outside this file (functions of the exe and import
    /// slots): the rig doubles all of them.
    const EXTERNAL: &[u32] = &[
        FIND_FIRST_FILE,
        FIND_CLOSE,
        TILE_GET_VALUE,
        TILE_SET_VALUE,
        TILE_SET_STRING,
        TILE_GET_STRING,
        TILE_IS_TRUE,
        TILE_POKE,
        TILE_ADD_USER_TRAIT,
        TILE_TEXT_TO_TRAIT,
        TILE_GET_MENU,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_MAXIMUM_DEPTH,
        TILE_LOCK,
        TILE_UNLOCK,
        TILE_READ_FILE,
        TILE_SET_INT,
        NODE_ITEM,
        NODE_NEXT,
        LIST_IS_EMPTY,
        POINTER_LIST_STEP,
        POINTER_LIST_STEP_GLOBAL,
        FONT_MANAGER_GET,
        CALCULATE_STRING_DIMENSIONS,
        FLOAT_TO_INT,
        LOG_MESSAGE,
        MENU_FLAGS_TEST,
        MENU_FLAGS_SET,
        TICK_COUNT,
        OPERATOR_NEW,
        OPERATOR_DELETE,
        INTERFACE_MANAGER_GETTER,
        MENUS_ROOT_GETTER,
        FIRST_WORD,
        MENU_FADE_STATE,
        MENU_SET_MENU_TILE,
        START_MENU_CLASS_GETTER,
        XUI_IS_UP,
        SCOPE_GUARD_NEW,
        SCOPE_GUARD_DESTROY,
        SETUP_FOR_MAIN_MENU,
        OWNER_SET_CALLBACK_A,
        OWNER_SET_CALLBACK_B,
        MENU_CALLBACK,
        ELEMENTS_MISSING,
        AUDIO_INSTANCE,
        AUDIO_ON_MENU_DESTROYED,
        FLOAT_AT_OBJECT,
        HUD_SET_VALUE,
        SET_DIALOG_FLAG,
        FADE_MUSIC,
        SAVE_MANAGER_RELEASE,
        LIST_REMOVE_ALL,
        NI_POINTER_DESTRUCTOR,
        SAVE_LOAD_LIST_DESTRUCTOR,
        DOWNLOADS_LIST_DESTRUCTOR,
        MENU_BASE_DESTRUCTOR,
        LAZY_OBJECT_NEW,
        LIST_BOX_BASE_NEW,
        LIST_BOX_BASE_DESTROY,
        CREATE_TEXTURE_IMAGE,
        LIST_BOX_INITIALIZE,
        SAVE_MANAGER_PATH,
        BASE_PATH_GETTER,
        STRING_COPY,
        STRING_APPEND,
        IDENTITY,
        SETTING_STRING,
        ARRAY_SIZE,
        ARRAY_ELEMENT,
        LIST_BOX_ADD_ITEM,
        SAVE_MANAGER_CAN_SAVE,
        LIST_BOX_COUNT,
        TILE_SET_COUNT,
        SCREEN_WIDTH,
        SCREEN_ASPECT_RATIO,
        FADE,
        LIST_BOX_SET_ENABLED,
        TILE_MODEL,
        MODEL_SET_SCALE,
        GET_ENTER_STACK,
        POINTER_LIST_ADD,
        IS_IN_RENDERED_MENU,
        RENDER_MENU_BACKGROUND,
        VATS_SET_MODE,
        IS_TOP_MENU_ID,
        CLOSE_PAUSE_HELPER,
        SET_PAUSE_MENU_MODE,
        IN_DIALOG,
        SET_DIALOG_PAUSE,
        SETTING_FLOAT,
        MEMSET,
        STRING_COPY_NARROW,
        MEDIA_PLAY,
        MEDIA_FINISH,
        PREP_FOR_VISIBILITY,
        NETWORK_UTILITY_INSTANCE,
        NETWORK_UTILITY_SET_CALLBACK,
        NETWORK_CALLBACK,
        FN_007D3AA0,
        ARRAY_ADD,
        OPTION_TEXT,
        USER_OPTION_NEW,
        STRING_SET,
        STRING_UPDATE,
        ARRAY_CLEAR,
        SAVE_LOAD_ENTRIES_CLEAR,
        SAVE_LOAD_LIST_IS_ENABLED,
        SAVE_LOAD_LIST_SET_ENABLED,
        SAVE_LOAD_LIST_SET_SELECTION,
        LIST_BOX_IS_SELECTED,
        LIST_BOX_DESELECT_ALL,
        INT_LIST_SET_ENABLED,
        INT_LIST_IS_ENABLED,
        CONFIRM_SAVE_LOAD,
        UPDATE_HIGHLIGHTED_SAVE_GAME_INFO,
        HIDE_OPTIONS_CONTAINER,
        LIST_BOX_FOR_EACH,
        LIST_BOX_GET_ITEM,
        LIST_BOX_HAS_TILE,
        LIST_BOX_SELECT,
        OPTIONS_LIST_HAS_SELECTION,
        MENU_REFRESH_A,
        MENU_REFRESH_B,
        SHOW_MESSAGE,
        MESSAGE_CALLBACK,
        SAVE_ENTRY_IS_CORRUPT,
        DLC_GETTER,
        DLC_STEP_A,
        DLC_INSTANCE,
        DLC_STEP_B,
        DLC_OPEN,
        RT_DYNAMIC_CAST,
        TILE_GET_CHILD_BY_ID,
        TILE_IS_VISIBLE,
        TILE_PLAY_TILE_SOUND,
        LIST_BOX_IS_ENABLED,
        LIST_BOX_HIGHLIGHTED_ITEM,
        GAMEPAD_BUTTON_TO_KEY,
        INPUT_OBJECT_GETTER,
        BIND_CONTROL,
        GAMEPAD_ACTIVE,
        PLAY_MENU_SOUND,
        LOADING_MENU_HANDLER,
        SLIDER_STEP,
        TILE_GET_FADE_FINISHED,
        TILE_POSITION_A,
        TILE_POSITION_B,
        MANAGER_WIDTH,
        ROUND_FLOAT,
        CLEAR_MENU_BUTTON,
        CHOOSE_BACK,
        CHOOSE_007D0C50,
        TILE_PARENT,
        FORM_LIST_HEAD,
        LOOKUP_FORM,
        IS_MENU_ID_VISIBLE,
        TUTORIAL_MENU_CREATE,
        MENU_REQUEST,
        PREP_SAVE_LOAD,
        DISPLAY_OPTION,
        INT_LIST_ADD_ITEM,
        MOD_LIST_HEAD,
        MOD_IS_ACTIVE,
        CHILD_MENU_DONE,
        LOADING_MENU_UPDATE,
        LOADING_MENU_CHECK,
        MAKE_LOAD_SCREEN_LIST,
        IDLE_STEP_A,
        IDLE_STEP_B,
        SAVE_LOAD_IDLE,
        MESSAGE_MENU_ACTIVE,
        CLOSE_MESSAGE_MENU,
        MENU_REFRESH_D,
        RESUME_BACKGROUND_THREAD,
        CREDITS_MENU_CREATE,
        DLC_CALL_WITH_CALLBACK,
        CHOOSE_007D4C50,
        STRING_NEW,
        STRING_DESTROY,
        STRING_FORMAT,
        KEY_LOOKUP,
        CONTROL_COUNT,
        INCREMENT,
        CONTROL_CHOOSE,
        SET_TILE_LABEL,
        HIDE_MENUS,
        CLOSE_ALL_SUPPORT_MENUS,
        KILL_ALL_OF_TYPE,
        MEDIA_CLEAR,
        OBJECT_CALL_009698A0,
        TEXTURE_OWNER_CALL,
        START_MENU_FLAG_1,
        CURRENT_RENDERED_MENU,
        GET_PIPBOY,
        PIPBOY_RESET_COLORS,
        MENU_START_FADE_OUT,
        MENU_REFRESH_C,
        MANAGER_ROW_COUNT,
        FN_00450F90,
        FN_00713FB0,
        FN_005B6CB0,
        FN_007D6030,
        POPULATE_OPTIONS_LIST_BOX,
        SETTING_VALUE_SLOT,
        FN_005BE4D0,
        FN_0093E770,
        FN_005D1530,
        FN_005D14D0,
        FN_007ADCC0,
        SET_HARDCORE,
        FN_004D1360,
        SETTING_SET_BOOL,
        SETTING_GET_BOOL,
        SETTING_GET_VALUE,
        FLOAT_SLOT,
        FLOAT_SET,
        FLOAT_TABLE_SET,
        FLOAT_SINK,
        SYSTEM_COLOR_INSTANCE,
        SYSTEM_COLOR_SET,
        SYSTEM_COLOR_GET,
        FN_00A0B350,
        SETTING_SET_INT,
        FN_00719740,
        FN_005407B0,
        FN_00A24B70,
        FN_00851330,
        FN_008467B0,
        ARCHIVE_CLEANUP,
        FN_0075F8C0,
        FN_00408DA0,
        FN_00408B20,
        STRTOL,
        FN_00401170,
        GET_FORM,
        FN_0060C9C0,
        FN_004581C0,
        ERROR_CALL,
        MEDIA_CLOSE,
        FN_00788A90,
        TUTORIAL_SHOW_MESSAGE,
        SET_STRING_FIELD,
        REMOVE_SAVE_GAME,
        IS_AUTOSAVE,
        FN_00846B10,
        FN_007D8040,
        FN_007D8700,
        FN_007D87B0,
        FN_007D62E0,
        FN_007D86C0,
        FN_007D80F0,
        FN_00825C00,
        FN_007D6E90,
        TILE_ADD_NEEDS_UPDATE,
        MESSAGE_MENU_RESULT,
    ];

    /// Pages of the exe's data the code reads or writes.
    const DATA_PAGES: &[u32] = &[
        0x011a_0000,
        0x011c_3000,
        0x011d_1000,
        0x011d_2000,
        0x011d_3000,
        0x011d_4000,
        0x011d_5000,
        0x011d_9000,
        0x011d_a000,
        0x011d_b000,
        0x011d_c000,
        0x011d_d000,
        0x011d_e000,
        0x0101_1000,
        0x0101_2000,
        0x0101_6000,
        0x0102_3000,
        0x0102_9000,
        0x0104_f000,
        0x0107_0000,
        0x0107_3000,
        0x0107_4000,
        0x0107_6000,
        0x0106_f000,
    ];

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: f64::from(value),
            ..Ret::default()
        }
    }

    struct Rig {
        e: Engine,
        calls: Calls,
        values: Values,
    }

    /// The local the next `with_stack(size)` on the running thread takes (the called function's
    /// own frame, taken at the stack's current top).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    /// An engine whose every external callee is a recording double
    /// returning 0, with the tile trait getters and setters backed by a
    /// table and `Tile::TextToTrait` returning the address of its name.
    fn rig() -> Rig {
        let mut r = Rig {
            e: Engine::new(),
            calls: Calls::default(),
            values: Values::default(),
        };
        for page in DATA_PAGES {
            r.e.map(*page, 0x1000);
        }
        for addr in EXTERNAL {
            r.stub(*addr, 0);
        }
        let values = r.values.clone();
        r.with(TILE_GET_VALUE, move |_, a| {
            st0(values.borrow().get(&(a[0], a[1])).copied().unwrap_or(0.0))
        });
        for setter in [TILE_SET_VALUE, TILE_SET_INT, TILE_POKE] {
            let values = r.values.clone();
            r.with(setter, move |_, a| {
                let value = if setter == TILE_SET_INT {
                    a[2] as i32 as f32
                } else {
                    f32::from_bits(a[2])
                };
                values.borrow_mut().insert((a[0], a[1]), value);
                eax(0)
            });
        }
        r.with(TILE_TEXT_TO_TRAIT, |_, a| eax(a[0]));
        r.with(INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value.wrapping_add(1));
            eax(a[0])
        });
        r.with(NODE_ITEM, |_, a| eax(a[0]));
        r.with(NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        r.e.mem.set_f32(FLOAT_MINUS_ONE, -1.0);
        r
    }

    impl Rig {
        /// A double at `addr` that records its call and returns `f(args)`.
        fn with(&mut self, addr: u32, mut f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static) {
            let calls = self.calls.clone();
            self.e.register_double(addr, move |e, a| {
                calls.borrow_mut().push((addr, a.to_vec()));
                f(e, a)
            });
        }
        /// A double returning `value` in EAX.
        fn stub(&mut self, addr: u32, value: u32) {
            self.with(addr, move |_, _| eax(value));
        }
        /// A double returning the `float` `value` in ST0.
        fn stub_float(&mut self, addr: u32, value: f32) {
            self.with(addr, move |_, _| st0(value));
        }
        /// The argument words of every recorded call to `addr`.
        fn calls_to(&self, addr: u32) -> Vec<Vec<u32>> {
            self.calls
                .borrow()
                .iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, args)| args.clone())
                .collect()
        }
        /// The addresses called, in order.
        fn order(&self) -> Vec<u32> {
            self.calls.borrow().iter().map(|(a, _)| *a).collect()
        }
        fn clear_calls(&self) {
            self.calls.borrow_mut().clear();
        }
        fn alloc(&mut self, size: u32) -> u32 {
            self.e.mem.alloc(size)
        }
        /// A zeroed start menu, installed as `StartMenu::pMe`.
        fn menu(&mut self) -> Ptr<StartMenu> {
            let menu = self.e.new_object::<StartMenu>();
            self.e.set_global(START_MENU, menu.addr());
            menu
        }
        fn set_tile(&self, tile: u32, id: u32, value: f32) {
            self.values.borrow_mut().insert((tile, id), value);
        }
        fn tile(&self, tile: u32, id: u32) -> f32 {
            self.values
                .borrow()
                .get(&(tile, id))
                .copied()
                .unwrap_or(f32::NAN)
        }
        /// A chain of `BSSimpleList` nodes holding `items`; returns the
        /// first node.
        fn nodes(&mut self, items: &[u32]) -> u32 {
            let mut next = 0;
            for item in items.iter().rev() {
                let node = self.alloc(8);
                self.e.mem.set_u32(node, *item);
                self.e.mem.set_u32(node + 4, next);
                next = node;
            }
            next
        }
        /// A list box whose embedded first node (+4) holds `items`.
        fn list_box(&mut self, items: &[u32]) -> Ptr<ListBox> {
            let list = self.e.new_object::<ListBox>();
            if let Some((first, rest)) = items.split_first() {
                let rest = self.nodes(rest);
                self.e.mem.set_u32(list.addr() + 4, *first);
                self.e.mem.set_u32(list.addr() + 8, rest);
            }
            list
        }
        /// A list item (a tile and a key), as the list stores it.
        fn item(&mut self, tile: u32, key: u32) -> u32 {
            let item = self.alloc(0x10);
            self.e.mem.set_u32(item, tile);
            self.e.mem.set_u32(item + 4, key);
            item
        }
    }

    #[test]
    fn every_translation_is_registered() {
        let e = Engine::new();
        for (addr, _) in funcs() {
            assert!(e.is_translated(addr), "{addr:08x}");
        }
        assert_eq!(funcs().len(), 120);
    }

    // ----- 00723030 -----

    #[test]
    fn belongs_to_compares_the_containers_menu() {
        let mut r = rig();
        let list = r.e.new_object::<ListBox>();
        // No container tile: false without asking for the menu.
        assert!(!r.e.call(0x0072_3030, &args![list, 0x77u32]).bool());
        assert!(r.calls_to(TILE_GET_MENU).is_empty());
        r.e.set(list, ListBox::pContainerRect, Ptr::new(0x500));
        r.stub(TILE_GET_MENU, 0x77);
        assert!(r.e.call(0x0072_3030, &args![list, 0x77u32]).bool());
        assert!(!r.e.call(0x0072_3030, &args![list, 0x78u32]).bool());
        assert_eq!(r.calls_to(TILE_GET_MENU)[0], vec![0x500]);
    }

    // ----- 007269d0 -----

    #[test]
    fn init_tile_position_uses_the_tiles_own_height() {
        let mut r = rig();
        let list = r.e.new_object::<ListBox>();
        r.e.set(list, ListBox::fTotalListHeight, 30.0);
        r.stub(TILE_ADD_USER_TRAIT, 0x4242);
        r.set_tile(0x600, TRAIT_HEIGHT, 18.0);
        r.e.call(0x0072_69d0, &args![list, 0x600u32]);
        // The `_y` trait gets the height so far, with flag 1.
        let add = &r.calls_to(TILE_ADD_USER_TRAIT)[0];
        assert_eq!(add, &vec![TEXT_USER_TRAIT_Y, 0xffff_ffff]);
        assert_eq!(
            r.calls_to(TILE_SET_VALUE)[0],
            vec![0x600, 0x4242, 30.0f32.to_bits(), 1]
        );
        // The height is stored back (flag 0) and added to the total.
        assert_eq!(r.tile(0x600, TRAIT_HEIGHT), 18.0);
        assert_eq!(r.calls_to(TILE_SET_VALUE)[1][3], 0);
        assert_eq!(r.e.get(list, ListBox::fTotalListHeight), 48.0);
        assert!(r.calls_to(FONT_MANAGER_GET).is_empty());
    }

    #[test]
    fn init_tile_position_measures_the_text_child_when_the_height_is_zero() {
        let mut r = rig();
        let list = r.e.new_object::<ListBox>();
        let tile = r.alloc(0x40);
        // The child list: head at +4 of the tile's list object (tile + 4),
        // iterated by a cursor word; one child whose class is 0x387.
        let child = r.alloc(8);
        let vtable = r.alloc(0x20);
        r.e.mem.set_u32(child, vtable);
        r.e.mem.set_u32(vtable + 0xc, 0x0ddd_0001);
        r.stub(0x0ddd_0001, 0x387);
        r.stub(NODE_NEXT, 1);
        let slot = r.alloc(4);
        r.e.mem.set_u32(slot, child);
        r.with(POINTER_LIST_STEP, move |e, a| {
            e.mem.set_u32(a[1], 0);
            eax(slot)
        });
        r.set_tile(child, TRAIT_WRAP_WIDTH, 120.0);
        r.set_tile(child, TRAIT_FONT, 3.0);
        r.stub(TILE_GET_STRING, 0x9000);
        r.stub(FONT_MANAGER_GET, 0x7777);
        r.with(CALCULATE_STRING_DIMENSIONS, |e, a| {
            e.mem.set_f32(a[1] + 4, 30.0);
            eax(a[1])
        });
        r.stub(FLOAT_TO_INT, 3);
        r.stub(TILE_TEXT_TO_TRAIT, 0x5555);
        r.set_tile(tile, 0x5555, 2.5);
        r.e.call(0x0072_69d0, &args![list, tile]);
        let measure = r.calls_to(CALCULATE_STRING_DIMENSIONS)[0].clone();
        assert_eq!(measure.len(), 6);
        assert_eq!(
            [measure[0], measure[2], measure[3], measure[4], measure[5]],
            [0x7777, 0x9000, 3, 120.0f32.to_bits(), 0]
        );
        // The font index is the truncated font trait (passed as a double).
        let font = 3.0f64.to_bits();
        assert_eq!(
            r.calls_to(FLOAT_TO_INT)[0],
            vec![font as u32, (font >> 32) as u32]
        );
        // Text height 30 + the tile's vertical spacing 2.5.
        assert_eq!(r.tile(tile, TRAIT_HEIGHT), 32.5);
        assert_eq!(r.e.get(list, ListBox::fTotalListHeight), 32.5);
    }

    #[test]
    fn init_tile_position_without_a_text_child_leaves_the_height_zero() {
        let mut r = rig();
        let list = r.e.new_object::<ListBox>();
        let tile = r.alloc(0x40);
        // An empty child list: the first node is 0.
        r.stub(NODE_NEXT, 0);
        r.e.call(0x0072_69d0, &args![list, tile]);
        assert_eq!(r.tile(tile, TRAIT_HEIGHT), 0.0);
        assert_eq!(r.e.get(list, ListBox::fTotalListHeight), 0.0);
        assert!(r.calls_to(CALCULATE_STRING_DIMENSIONS).is_empty());
    }

    #[test]
    fn init_tile_position_uses_the_largest_float_for_a_nonpositive_wrap_width() {
        let mut r = rig();
        let list = r.e.new_object::<ListBox>();
        let tile = r.alloc(0x40);
        let child = r.alloc(8);
        let vtable = r.alloc(0x20);
        r.e.mem.set_u32(child, vtable);
        r.e.mem.set_u32(vtable + 0xc, 0x0ddd_0001);
        r.stub(0x0ddd_0001, 0x387);
        r.stub(NODE_NEXT, 1);
        let slot = r.alloc(4);
        r.e.mem.set_u32(slot, child);
        r.with(POINTER_LIST_STEP, move |e, a| {
            e.mem.set_u32(a[1], 0);
            eax(slot)
        });
        r.set_tile(child, TRAIT_WRAP_WIDTH, -4.0);
        r.e.mem.set_f32(FLOAT_MAX, f32::MAX);
        r.with(CALCULATE_STRING_DIMENSIONS, |_, a| eax(a[1]));
        r.e.call(0x0072_69d0, &args![list, tile]);
        assert_eq!(
            r.calls_to(CALCULATE_STRING_DIMENSIONS)[0][4],
            f32::MAX.to_bits()
        );
    }

    // ----- 007a22d0, 007a30a0, 007a6fa0 -----

    #[test]
    fn get_tile_for_finds_the_item_with_the_same_option() {
        let mut r = rig();
        let first = r.item(0xa1, 0x111);
        let second = r.item(0xa2, 0x222);
        let list = r.list_box(&[first, second]);
        let key = r.alloc(4);
        r.e.mem.set_u32(key, 0x222);
        assert_eq!(r.e.call(0x007a_22d0, &args![list, key]).u32(), 0xa2);
        r.e.mem.set_u32(key, 0x333);
        assert_eq!(r.e.call(0x007a_22d0, &args![list, key]).u32(), 0);
        // A null list.
        assert_eq!(
            r.e.call(0x007a_22d0, &args![Ptr::<ListBox>::NULL, key])
                .u32(),
            0
        );
    }

    #[test]
    fn get_tile_for_stops_at_a_null_item() {
        let mut r = rig();
        let second = r.item(0xa2, 0x222);
        let list = r.list_box(&[0, second]);
        let key = r.alloc(4);
        r.e.mem.set_u32(key, 0x222);
        assert_eq!(r.e.call(0x007a_22d0, &args![list, key]).u32(), 0);
    }

    #[test]
    fn get_at_by_position_walks_the_nodes() {
        let mut r = rig();
        let items: Vec<u32> = (0..3).map(|i| r.item(0xb0 + i, 0)).collect();
        let list = r.list_box(&items);
        r.stub(LIST_IS_EMPTY, 0);
        assert_eq!(
            r.e.call(0x007a_6fa0, &args![list, 2u32, 1u32]).u32(),
            items[2]
        );
        assert_eq!(
            r.e.call(0x007a_6fa0, &args![list, 0u32, 1u32]).u32(),
            items[0]
        );
        // Past the end.
        assert_eq!(r.e.call(0x007a_6fa0, &args![list, 3u32, 1u32]).u32(), 0);
    }

    #[test]
    fn get_at_by_list_index_compares_the_tiles_trait() {
        let mut r = rig();
        let a = r.item(0xc1, 0);
        let b = r.item(0xc2, 0);
        let list = r.list_box(&[a, b]);
        r.stub(LIST_IS_EMPTY, 0);
        r.set_tile(0xc1, TRAIT_LIST_INDEX, 5.0);
        r.set_tile(0xc2, TRAIT_LIST_INDEX, 9.0);
        assert_eq!(r.e.call(0x007a_6fa0, &args![list, 9u32, 0u32]).u32(), b);
        assert_eq!(r.e.call(0x007a_6fa0, &args![list, 5u32, 0u32]).u32(), a);
        assert_eq!(r.e.call(0x007a_6fa0, &args![list, 6u32, 0u32]).u32(), 0);
        // An empty list answers 0 without walking.
        r.stub(LIST_IS_EMPTY, 1);
        assert_eq!(r.e.call(0x007a_6fa0, &args![list, 5u32, 0u32]).u32(), 0);
        assert_eq!(r.calls_to(LIST_IS_EMPTY)[0], vec![list.addr() + 4]);
    }

    #[test]
    fn get_tile_at_returns_the_first_word_of_the_item() {
        let mut r = rig();
        let a = r.item(0xd1, 0);
        let list = r.list_box(&[a]);
        r.stub(LIST_IS_EMPTY, 0);
        assert_eq!(r.e.call(0x007a_30a0, &args![list, 0u32, 1u32]).u32(), 0xd1);
        // No item: 0.
        assert_eq!(r.e.call(0x007a_30a0, &args![list, 4u32, 1u32]).u32(), 0);
    }

    // ----- 007a5e70 -----

    /// Scrollbar scene: container 0x100 (height `container`), highlighted
    /// item 0x200 (`y`, `height`), scrollbar 0x300. Setting the scrollbar's
    /// `_current_value` moves the item by `-step` per unit when `moves`.
    fn scroll_scene(r: &mut Rig, container: f32, y: f32, height: f32) -> Ptr<ListBox> {
        let list = r.e.new_object::<ListBox>();
        r.e.set(list, ListBox::pContainerRect, Ptr::new(0x100));
        r.e.set(list, ListBox::pHighlightedItem, Ptr::new(0x200));
        r.e.set(list, ListBox::pScrollBar, Ptr::new(0x300));
        r.set_tile(0x100, TRAIT_HEIGHT, container);
        r.set_tile(0x200, TRAIT_Y, y);
        r.set_tile(0x200, TRAIT_HEIGHT, height);
        r.set_tile(0x200, TRAIT_LIST_INDEX, 0.0);
        r.set_tile(0x300, TEXT_CURRENT_VALUE, 0.0);
        list
    }

    #[test]
    fn scroll_to_highlight_does_nothing_without_the_three_tiles() {
        let mut r = rig();
        let list = r.e.new_object::<ListBox>();
        r.e.call(0x007a_5e70, &args![list]);
        assert!(r.calls_to(TILE_GET_VALUE).is_empty());
        // A container of height zero (or NaN) ends it too.
        let list = scroll_scene(&mut r, 0.0, 100.0, 20.0);
        r.e.call(0x007a_5e70, &args![list]);
        assert!(r.calls_to(TILE_POKE).is_empty());
        let list = scroll_scene(&mut r, f32::NAN, 100.0, 20.0);
        r.e.call(0x007a_5e70, &args![list]);
        assert!(r.calls_to(TILE_POKE).is_empty());
    }

    #[test]
    fn scroll_to_highlight_leaves_a_visible_item_alone() {
        let mut r = rig();
        let list = scroll_scene(&mut r, 100.0, 10.0, 20.0);
        r.e.call(0x007a_5e70, &args![list]);
        assert!(r.calls_to(TILE_SET_VALUE).is_empty());
        assert!(r.calls_to(TILE_POKE).is_empty());
    }

    #[test]
    fn scroll_to_highlight_steps_down_until_the_item_is_visible() {
        let mut r = rig();
        // Below the visible area: y 100 + height 20 > container 50. Every
        // scrollbar step moves the item up by 30.
        let list = scroll_scene(&mut r, 50.0, 100.0, 20.0);
        let values = r.values.clone();
        r.with(TILE_SET_VALUE, move |_, a| {
            let value = f32::from_bits(a[2]);
            let mut values = values.borrow_mut();
            if a[0] == 0x300 {
                let y = values[&(0x200, TRAIT_Y)];
                values.insert((0x200, TRAIT_Y), y - 30.0);
            }
            values.insert((a[0], a[1]), value);
            eax(0)
        });
        r.e.call(0x007a_5e70, &args![list]);
        let sets = r.calls_to(TILE_SET_VALUE);
        assert_eq!(sets.len(), 3);
        assert_eq!(f32::from_bits(sets[2][2]), 3.0);
        assert_eq!(sets[0][0], 0x300);
        assert_eq!(sets[0][1], TEXT_CURRENT_VALUE);
        assert_eq!(sets[0][3], 0);
        assert_eq!(r.tile(0x200, TRAIT_Y), 10.0);
        let poke = &r.calls_to(TILE_POKE)[0];
        assert_eq!(poke, &vec![0x300, TEXT_SET_IN_CODE, 1.0f32.to_bits()]);
        assert!(r.calls_to(LOG_MESSAGE).is_empty());
    }

    #[test]
    fn scroll_to_highlight_steps_up_for_an_item_above_the_area() {
        let mut r = rig();
        let list = scroll_scene(&mut r, 100.0, -5.0, 20.0);
        r.set_tile(0x300, TEXT_CURRENT_VALUE, 4.0);
        let values = r.values.clone();
        r.with(TILE_SET_VALUE, move |_, a| {
            let mut values = values.borrow_mut();
            if a[0] == 0x300 {
                values.insert((0x200, TRAIT_Y), 0.0);
            }
            values.insert((a[0], a[1]), f32::from_bits(a[2]));
            eax(0)
        });
        r.e.call(0x007a_5e70, &args![list]);
        // One step of -1 from 4.0.
        let sets = r.calls_to(TILE_SET_VALUE);
        assert_eq!(sets.len(), 1);
        assert_eq!(f32::from_bits(sets[0][2]), 3.0);
        assert_eq!(r.calls_to(TILE_POKE).len(), 1);
    }

    #[test]
    fn scroll_to_highlight_reports_a_scrollbar_that_does_not_move() {
        let mut r = rig();
        let list = scroll_scene(&mut r, 50.0, 100.0, 20.0);
        // The setter changes nothing: the value read back is the old one.
        r.with(TILE_SET_VALUE, |_, _| eax(0));
        r.e.call(0x007a_5e70, &args![list]);
        assert_eq!(r.calls_to(LOG_MESSAGE), vec![vec![TEXT_SCROLLBAR_LOOP]]);
        assert_eq!(r.calls_to(TILE_SET_VALUE).len(), 1);
        // The poke still happens.
        assert_eq!(r.calls_to(TILE_POKE).len(), 1);
    }

    #[test]
    fn scroll_to_highlight_stops_for_a_negative_list_index() {
        let mut r = rig();
        let list = scroll_scene(&mut r, 50.0, 100.0, 20.0);
        r.set_tile(0x200, TRAIT_LIST_INDEX, -1.0);
        r.e.call(0x007a_5e70, &args![list]);
        assert!(r.calls_to(TILE_SET_VALUE).is_empty());
        // It was outside, so the scrollbar is still poked.
        assert_eq!(r.calls_to(TILE_POKE).len(), 1);
    }

    impl Rig {
        /// The flag word of the menu (`+0x1a8`), tested and changed by the
        /// two flag functions like the game does.
        fn real_flags(&mut self) {
            self.with(MENU_FLAGS_TEST, |e, a| {
                eax(u32::from(e.mem.u32(a[0] + 0x1a8) & a[1] != 0))
            });
            self.with(MENU_FLAGS_SET, |e, a| {
                let flags = e.mem.u32(a[0] + 0x1a8);
                let flags = if a[2] as u8 != 0 {
                    flags | a[1]
                } else {
                    flags & !a[1]
                };
                e.mem.set_u32(a[0] + 0x1a8, flags);
                eax(0)
            });
        }
        /// A vtable at a fresh address whose slot `slot` (byte offset)
        /// points at a double returning `value`; returns the vtable.
        fn vtable_with(&mut self, slot: u32, value: u32) -> u32 {
            let vtable = self.alloc(0x100);
            let target = 0x0eee_0000 + vtable + slot;
            self.e.mem.set_u32(vtable + slot, target);
            self.stub(target, value);
            vtable
        }
        /// An object with vtable `vtable`.
        fn object(&mut self, vtable: u32) -> u32 {
            let object = self.alloc(0x40);
            self.e.mem.set_u32(object, vtable);
            object
        }
    }

    // ----- 007cb4d0 .. 007cb580 -----

    #[test]
    fn get_class_is_0x3f5() {
        let mut r = rig();
        let menu = r.menu();
        assert_eq!(r.e.call(0x007c_b4d0, &args![menu]).u32(), 0x3f5);
        assert_eq!(r.e.call(0x007c_c6d0, &args![]).u32(), 0x423);
    }

    #[test]
    fn attach_tile_by_id_stores_in_range_only() {
        let mut r = rig();
        let menu = r.menu();
        r.e.call(0x007c_b4e0, &args![menu, 3i32, 0x1234u32]);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x28 + 12), 0x1234);
        r.e.call(0x007c_b4e0, &args![menu, 0x16i32, 0x5678u32]);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x28 + 0x16 * 4), 0x5678);
        // Out of range: nothing is written (the word after the table, the
        // list box at +0x84, stays 0; so does the vtable word).
        r.e.call(0x007c_b4e0, &args![menu, 0x17i32, 0x9999u32]);
        r.e.call(0x007c_b4e0, &args![menu, -1i32, 0x9999u32]);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x28 + 0x17 * 4), 0);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x24), 0);
    }

    #[test]
    fn option_list_constructor_and_destructors() {
        let mut r = rig();
        let list = r.e.new_object::<ListBox>();
        assert_eq!(r.e.call(0x007c_b540, &args![list]).u32(), list.addr());
        assert_eq!(r.e.mem.u32(list.addr()), VTABLE_OPTION_LIST);
        assert_eq!(r.calls_to(LIST_BOX_BASE_NEW), vec![vec![list.addr()]]);
        r.e.call(0x007c_b560, &args![list]);
        assert_eq!(r.calls_to(LIST_BOX_BASE_DESTROY), vec![vec![list.addr()]]);
        // The scalar deleting form frees only when bit 0 is set.
        assert_eq!(r.e.call(0x007c_b580, &args![list, 0u32]).u32(), list.addr());
        assert!(r.calls_to(OPERATOR_DELETE).is_empty());
        r.e.call(0x007c_b580, &args![list, 1u32]);
        assert_eq!(r.calls_to(OPERATOR_DELETE), vec![vec![list.addr()]]);
        assert_eq!(r.calls_to(LIST_BOX_BASE_DESTROY).len(), 3);
    }

    // ----- 007cb5b0, 007cb510 -----

    #[test]
    fn destructor_releases_everything_in_order() {
        let mut r = rig();
        let menu = r.menu();
        r.real_flags();
        r.e.set(menu, StartMenu::uiFlags, 0x40);
        r.stub_float(FLOAT_AT_OBJECT, 0.75);
        r.stub(AUDIO_INSTANCE, 0xa0d10);
        r.stub(XUI_IS_UP, 1);
        r.e.set_global(CALLBACK_OWNER, 0x5150);
        r.e.set_global(LAZY_OBJECT, 0xbeef);
        r.e.call(0x007c_b5b0, &args![menu]);
        assert_eq!(r.e.mem.u32(menu.addr()), VTABLE_START_MENU);
        // Flag 0x40: the float read from the object goes to the HUD call.
        assert_eq!(r.calls_to(FLOAT_AT_OBJECT), vec![vec![VOLUME_OBJECT]]);
        assert_eq!(r.calls_to(HUD_SET_VALUE), vec![vec![0.75f32.to_bits()]]);
        assert_eq!(r.calls_to(AUDIO_ON_MENU_DESTROYED), vec![vec![0xa0d10]]);
        assert_eq!(r.calls_to(SET_DIALOG_FLAG), vec![vec![0]]);
        assert_eq!(r.calls_to(FADE_MUSIC), vec![vec![0x7d0, 0]]);
        assert_eq!(r.calls_to(SAVE_MANAGER_RELEASE), vec![vec![0x5150]]);
        assert_eq!(r.calls_to(LIST_REMOVE_ALL), vec![vec![SCRIPT_LIST]]);
        // The lazy object is kept by the first step (already there) and
        // then freed.
        assert_eq!(r.calls_to(OPERATOR_DELETE), vec![vec![0xbeef]]);
        assert_eq!(r.e.global::<u32>(LAZY_OBJECT), 0);
        assert_eq!(r.e.global::<u32>(START_MENU), 0);
        // The parts, last to first, then the base menu.
        let parts: Vec<u32> = r
            .order()
            .into_iter()
            .filter(|a| {
                [
                    NI_POINTER_DESTRUCTOR,
                    SAVE_LOAD_LIST_DESTRUCTOR,
                    DOWNLOADS_LIST_DESTRUCTOR,
                    LIST_BOX_BASE_DESTROY,
                    MENU_BASE_DESTRUCTOR,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            parts,
            vec![
                NI_POINTER_DESTRUCTOR,
                SAVE_LOAD_LIST_DESTRUCTOR,
                DOWNLOADS_LIST_DESTRUCTOR,
                LIST_BOX_BASE_DESTROY,
                LIST_BOX_BASE_DESTROY,
                LIST_BOX_BASE_DESTROY,
                LIST_BOX_BASE_DESTROY,
                MENU_BASE_DESTRUCTOR,
            ]
        );
        let lists: Vec<u32> = r
            .calls_to(LIST_BOX_BASE_DESTROY)
            .into_iter()
            .map(|a| a[0] - menu.addr())
            .collect();
        assert_eq!(lists, vec![0x114, 0xe4, 0xb4, 0x84]);
    }

    #[test]
    fn destructor_skips_the_volume_and_dialog_steps_without_their_conditions() {
        let mut r = rig();
        let menu = r.menu();
        r.real_flags();
        r.e.call(0x007c_b5b0, &args![menu]);
        assert!(r.calls_to(HUD_SET_VALUE).is_empty());
        assert!(r.calls_to(SET_DIALOG_FLAG).is_empty());
        assert!(r.calls_to(FADE_MUSIC).is_empty());
    }

    #[test]
    fn scalar_deleting_destructor_frees_when_bit_zero_is_set() {
        let mut r = rig();
        let menu = r.menu();
        r.real_flags();
        assert_eq!(r.e.call(0x007c_b510, &args![menu, 0u32]).u32(), menu.addr());
        assert!(r.calls_to(OPERATOR_DELETE).is_empty());
        assert_eq!(r.e.mem.u32(menu.addr()), VTABLE_START_MENU);
        r.e.call(0x007c_b510, &args![menu, 1u32]);
        assert_eq!(r.calls_to(OPERATOR_DELETE), vec![vec![menu.addr()]]);
    }

    // ----- 007cb710, 007cb7a0 -----

    #[test]
    fn lazy_object_is_created_once() {
        let mut r = rig();
        r.stub(OPERATOR_NEW, 0x9000);
        r.with(LAZY_OBJECT_NEW, |_, a| eax(a[0] + 0x10));
        assert_eq!(r.e.call(0x007c_b710, &args![]).u32(), 0x9010);
        assert_eq!(r.e.global::<u32>(LAZY_OBJECT), 0x9010);
        assert_eq!(r.calls_to(OPERATOR_NEW), vec![vec![8]]);
        // The second call returns the same object without allocating.
        assert_eq!(r.e.call(0x007c_b710, &args![]).u32(), 0x9010);
        assert_eq!(r.calls_to(OPERATOR_NEW).len(), 1);
    }

    #[test]
    fn lazy_object_is_null_when_the_allocation_fails() {
        let mut r = rig();
        r.stub(OPERATOR_NEW, 0);
        assert_eq!(r.e.call(0x007c_b710, &args![]).u32(), 0);
        assert!(r.calls_to(LAZY_OBJECT_NEW).is_empty());
    }

    #[test]
    fn free_lazy_object_deletes_and_clears() {
        let mut r = rig();
        r.e.call(0x007c_b7a0, &args![]);
        assert!(r.calls_to(OPERATOR_DELETE).is_empty());
        r.e.set_global(LAZY_OBJECT, 0x1234u32);
        r.e.call(0x007c_b7a0, &args![]);
        assert_eq!(r.calls_to(OPERATOR_DELETE), vec![vec![0x1234]]);
        assert_eq!(r.e.global::<u32>(LAZY_OBJECT), 0);
    }

    // ----- 007cb7d0 -----

    /// The pieces `Create` needs to reach the XML: no menu elements to
    /// build, an empty menu list, the new menu returned for the XML tile.
    fn create_scene(r: &mut Rig) -> (u32, u32) {
        r.real_flags();
        let class_vtable = r.vtable_with(0x34, 0x3f5);
        let menu = r.alloc(0x1d8);
        r.e.mem.set_u32(menu, class_vtable);
        let tile = 0x3333;
        r.stub(ELEMENTS_MISSING, 0);
        r.stub(INTERFACE_MANAGER_GETTER, 0x1111);
        r.stub(MENUS_ROOT_GETTER, 0x2222);
        r.stub(FIRST_WORD, 0);
        r.stub(TILE_GET_MENU_BY_CLASS, 0);
        r.stub_float(TILE_GET_MAXIMUM_DEPTH, 5.0);
        r.stub(TILE_READ_FILE, tile);
        r.with(TILE_GET_MENU, move |_, a| {
            eax(if a[0] == tile { menu } else { 0 })
        });
        r.stub(START_MENU_CLASS_GETTER, 0x3f5);
        r.stub(TICK_COUNT, 777);
        (menu, tile)
    }

    #[test]
    fn create_installs_the_menu_read_from_the_xml() {
        let mut r = rig();
        let (menu, tile) = create_scene(&mut r);
        r.set_tile(tile, TRAIT_STACKING_TYPE, 6006.0);
        r.e.mem.set_f64(STACKING_NO_CLICK_PAST_DONE, 6006.0);
        let frame = next_frame(&mut r.e, 0x64);
        assert!(r.e.call(0x007c_b7d0, &args![0u32, 0u32]).bool());
        // The 0x64-byte frame: the scope at +0x40.
        assert_eq!(r.calls_to(SCOPE_GUARD_NEW)[0][0], frame + 0x40);
        assert_eq!(r.e.global::<u32>(START_MENU), menu);
        assert_eq!(r.e.global::<u32>(TICK_CREATED), 777);
        // Flag a clear and the loading byte clear: the loading menu is set up.
        assert_eq!(r.calls_to(SETUP_FOR_MAIN_MENU), vec![vec![1]]);
        assert_eq!(r.calls_to(MENU_SET_MENU_TILE), vec![vec![menu, tile, 0]]);
        // Stacking type 6006: the depth trait gets the maximum depth.
        let depth = r
            .calls_to(TILE_SET_VALUE)
            .into_iter()
            .find(|c| c[0] == tile && c[1] == TRAIT_DEPTH)
            .expect("depth set");
        assert_eq!(depth[2], 5.0f32.to_bits());
        assert_eq!(
            r.calls_to(TILE_READ_FILE),
            vec![vec![0x2222, START_MENU_XML]]
        );
        let order = r.order();
        let lock = order.iter().position(|a| *a == TILE_LOCK).unwrap();
        let unlock = order.iter().rposition(|a| *a == TILE_UNLOCK).unwrap();
        assert!(lock < unlock);
        // The scope guard is created and destroyed around it.
        assert_eq!(
            r.calls_to(SCOPE_GUARD_NEW)[0][1..],
            [0xd, 1, SOURCE_FILE_NAME, 0x151]
        );
        // `_Create` has a guard of its own.
        assert_eq!(r.calls_to(SCOPE_GUARD_DESTROY).len(), 2);
    }

    #[test]
    fn create_with_the_new_game_flag_keeps_the_loading_menu_and_builds_elements() {
        let mut r = rig();
        let (menu, tile) = create_scene(&mut r);
        r.set_tile(tile, TRAIT_STACKING_TYPE, 1.0);
        r.stub(ELEMENTS_MISSING, 1);
        r.e.set_global(CALLBACK_OWNER, 0x6000u32);
        assert!(r.e.call(0x007c_b7d0, &args![1u32, 1u32]).bool());
        assert!(r.calls_to(SETUP_FOR_MAIN_MENU).is_empty());
        assert_eq!(r.calls_to(TICK_COUNT).len(), 2);
        assert_eq!(
            r.calls_to(OWNER_SET_CALLBACK_A),
            vec![vec![0x6000, MENU_CALLBACK]]
        );
        assert_eq!(
            r.calls_to(OWNER_SET_CALLBACK_B),
            vec![vec![0x6000, MENU_CALLBACK]]
        );
        // The menu elements were built (the first array add happened).
        assert!(!r.calls_to(ARRAY_ADD).is_empty());
        // No depth for an ordinary stacking type.
        assert!(r
            .calls_to(TILE_SET_VALUE)
            .iter()
            .all(|c| !(c[0] == tile && c[1] == TRAIT_DEPTH)));
        assert_eq!(r.e.global::<u32>(START_MENU), menu);
    }

    #[test]
    fn create_deletes_an_old_start_menu() {
        let mut r = rig();
        let (_, tile) = create_scene(&mut r);
        r.set_tile(tile, TRAIT_STACKING_TYPE, 1.0);
        let vtable = r.vtable_with(0, 0);
        let old = r.object(vtable);
        r.stub(TILE_GET_MENU_BY_CLASS, old);
        assert!(r.e.call(0x007c_b7d0, &args![0u32, 0u32]).bool());
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS)[0], vec![0x3f5]);
        let delete = 0x0eee_0000 + vtable;
        assert_eq!(r.calls_to(delete), vec![vec![old, 1]]);
    }

    #[test]
    fn create_refuses_while_another_menu_is_fading() {
        let mut r = rig();
        let (_, _) = create_scene(&mut r);
        // One menu in the list: its tile 0x7100, menu 0x7200 (the +4 word
        // set), fade state 2, stacking type not 6008.
        r.stub(FIRST_WORD, 1);
        let slot = r.alloc(4);
        r.e.mem.set_u32(slot, 0x7100);
        r.with(POINTER_LIST_STEP_GLOBAL, move |e, a| {
            e.mem.set_u32(a[1], 0);
            eax(slot)
        });
        let other = r.alloc(8);
        r.e.mem.set_u32(other + 4, 1);
        r.with(TILE_GET_MENU, move |_, _| eax(other));
        r.stub(MENU_FADE_STATE, 2);
        r.set_tile(0x7100, TRAIT_STACKING_TYPE, 6000.0);
        r.e.mem.set_f64(STACKING_DOES_NOT_STACK, 6008.0);
        assert!(!r.e.call(0x007c_b7d0, &args![0u32, 0u32]).bool());
        assert!(r.calls_to(TILE_LOCK).is_empty());
        assert_eq!(r.e.global::<u32>(START_MENU), 0);
        // The guard is still destroyed.
        assert_eq!(r.calls_to(SCOPE_GUARD_DESTROY).len(), 1);
    }

    #[test]
    fn create_goes_on_past_a_fading_menu_that_does_not_stack() {
        let mut r = rig();
        let (menu, tile) = create_scene(&mut r);
        r.set_tile(tile, TRAIT_STACKING_TYPE, 1.0);
        r.stub(FIRST_WORD, 1);
        let slot = r.alloc(4);
        r.e.mem.set_u32(slot, 0x7100);
        r.with(POINTER_LIST_STEP_GLOBAL, move |e, a| {
            e.mem.set_u32(a[1], 0);
            eax(slot)
        });
        let other = r.alloc(8);
        r.e.mem.set_u32(other + 4, 1);
        r.with(TILE_GET_MENU, move |_, a| {
            eax(if a[0] == 0x7100 {
                other
            } else if a[0] == tile {
                menu
            } else {
                0
            })
        });
        r.stub(MENU_FADE_STATE, 8);
        r.set_tile(0x7100, TRAIT_STACKING_TYPE, 6008.0);
        r.e.mem.set_f64(STACKING_DOES_NOT_STACK, 6008.0);
        assert!(r.e.call(0x007c_b7d0, &args![0u32, 0u32]).bool());
    }

    #[test]
    fn create_fails_when_the_xml_menu_is_another_class() {
        let mut r = rig();
        let (_, _) = create_scene(&mut r);
        r.stub(START_MENU_CLASS_GETTER, 0x400);
        // The built menu has a next word: it is deleted.
        let built = r.alloc(0x40);
        r.e.mem.set_u32(built + 4, 1);
        let class_vtable = r.vtable_with(0x34, 0x3f5);
        r.e.mem.set_u32(built, class_vtable);
        r.e.mem.set_u32(class_vtable, 0x0eee_9999);
        r.stub(0x0eee_9999, 0);
        r.with(TILE_GET_MENU, move |_, _| eax(built));
        assert!(!r.e.call(0x007c_b7d0, &args![0u32, 0u32]).bool());
        assert_eq!(r.calls_to(0x0eee_9999), vec![vec![built, 1]]);
        assert_eq!(r.calls_to(TILE_UNLOCK).len(), 1);
        assert_eq!(r.e.global::<u32>(START_MENU), 0);
    }

    // ----- 007cbaf0 -----

    /// A start menu for `_Create`: real flags, `pTiles` filled with the
    /// words `0x8000 + offset`, ENABLED_TRAIT set.
    fn inner_scene(r: &mut Rig) -> Ptr<StartMenu> {
        r.real_flags();
        let menu = r.menu();
        for index in 0..0x17u32 {
            r.e.mem
                .set_u32(menu.addr() + 0x28 + 4 * index, 0x8000 + 0x28 + 4 * index);
        }
        r.e.set(menu, StartMenu::ENABLED_TRAIT, 0x5a5a);
        r.e.mem.set_f64(WIDTH_LIMIT, 640.0);
        r.e.mem.set_f64(ASPECT_LIMIT, 1.3333);
        r.e.mem.set_f64(Y_OFFSET_SMALL_SCREEN, 25.0);
        r.e.mem.set_f32(FLOAT_255, 255.0);
        r.stub(TILE_ADD_USER_TRAIT, 0x6006);
        menu
    }

    /// Options for the options array: `(text, choose, modes)`. The array
    /// double returns the slot holding each option.
    fn options_array(r: &mut Rig, options: &[(u32, u32, u32)]) -> Vec<u32> {
        let mut objects = Vec::new();
        let mut slots = Vec::new();
        for (text, choose, modes) in options {
            let object = r.alloc(0x10);
            r.e.mem.set_u32(object + 4, *text);
            r.e.mem.set_u32(object + 8, *choose);
            r.e.mem.set_u32(object + 0xc, *modes);
            let slot = r.alloc(4);
            r.e.mem.set_u32(slot, object);
            objects.push(object);
            slots.push(slot);
        }
        r.stub(ARRAY_SIZE, options.len() as u32);
        r.with(ARRAY_ELEMENT, move |_, a| eax(slots[a[1] as usize]));
        r.with(LIST_BOX_ADD_ITEM, |_, a| eax(0x7000 + a[1] % 0x1000));
        objects
    }

    #[test]
    fn create_inner_sets_the_flags_and_initialises_the_six_lists() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        let frame = next_frame(&mut r.e, 0x2e0);
        r.e.call(0x007c_baf0, &args![menu, 1u32, 0u32]);
        // The 0x2e0-byte frame: the scope at +0x1c0, the save path at +0x1c4, the find data at +0x7c.
        assert_eq!(
            r.calls_to(SCOPE_GUARD_NEW).last().unwrap()[0],
            frame + 0x1c0
        );
        assert_eq!(
            r.calls_to(FIND_FIRST_FILE),
            vec![vec![frame + 0x1c4, frame + 0x7c]]
        );
        let flags = r.e.get(menu, StartMenu::uiFlags);
        // Flag a: bit 1 set; flag b: bit 4 clear; loading byte clear: 0x10.
        assert_eq!(flags & 0x15, 0x11);
        assert_eq!(flags & 0x80, 0);
        // The alpha frame texture goes into the menu's NiPointer slot.
        assert_eq!(
            r.calls_to(CREATE_TEXTURE_IMAGE),
            vec![vec![0, TEXT_ALPHA_FRAME, menu.addr() + 0x1b8, 0, 0]]
        );
        let inits = r.calls_to(LIST_BOX_INITIALIZE);
        let expect: Vec<Vec<u32>> = [
            (0x84u32, 0x28u32, TEXT_ITEM_TEMPLATE),
            (0xb4, 0x2c, TEXT_ITEM_TEMPLATE),
            (0xe4, 0x44, TEXT_ITEM_TEMPLATE),
            (0x114, 0x30, 0),
            (0x174, 0x40, TEXT_SAVELOAD_TEMPLATE),
            (0x144, 0x60, TEXT_ITEM_TEMPLATE),
        ]
        .iter()
        .map(|(list, tile, template)| vec![menu.addr() + list, 0x8000 + tile, *template])
        .collect();
        assert_eq!(inits, expect);
        assert_eq!(
            r.calls_to(SCOPE_GUARD_NEW)[0][1..],
            [0xd, 1, SOURCE_FILE_NAME, 0x1aa]
        );
        assert_eq!(r.calls_to(SCOPE_GUARD_DESTROY).len(), 1);
    }

    #[test]
    fn create_inner_main_menu_look_sets_vats_off_and_the_alpha() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.set_global(LOADING_BYTE, 1u8);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 1u32]);
        let flags = r.e.get(menu, StartMenu::uiFlags);
        // Flag b: bit 4; loading byte set and flag a clear: 0x80000; the
        // main-menu bits 0x20 and 0x80.
        assert_eq!(flags & 0x8_00b5, 0x8_00a4);
        assert_eq!(r.calls_to(VATS_SET_MODE), vec![vec![VATS_OBJECT, 0, 0]]);
        assert_eq!(r.tile(0x8000 + 0x34, TRAIT_VISIBLE), 1.0);
        // Not the top menu: full alpha, with flag 1.
        let alpha = r
            .calls_to(TILE_SET_VALUE)
            .into_iter()
            .find(|c| c[1] == TRAIT_ALPHA)
            .unwrap();
        assert_eq!(
            alpha,
            vec![0x8000 + 0x34, TRAIT_ALPHA, 255.0f32.to_bits(), 1]
        );
        assert!(r.calls_to(FADE).is_empty());
        // Flag 4 ran the save choice: the tile at +0x64 is hidden, flag 8
        // cleared, PrepSaveLoad called.
        assert_eq!(r.tile(0x8000 + 0x64, TRAIT_USER_0), 0.0);
        assert_eq!(r.calls_to(PREP_SAVE_LOAD), vec![vec![menu.addr()]]);
    }

    #[test]
    fn create_inner_main_menu_look_hides_the_alpha_over_the_top_menu() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.stub(IS_TOP_MENU_ID, 1);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        let alpha = r
            .calls_to(TILE_SET_VALUE)
            .into_iter()
            .find(|c| c[1] == TRAIT_ALPHA)
            .unwrap();
        assert_eq!(alpha[2], 0.0f32.to_bits());
        assert_eq!(r.calls_to(IS_TOP_MENU_ID), vec![vec![0x3f5]]);
    }

    #[test]
    fn create_inner_in_game_look_fades_and_hides_the_menus_underneath() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.mem.set_f32(FLOAT_MODEL_SCALE, 1.5);
        r.stub(TILE_MODEL, 0x9900);
        // Enter stack: a hidden class that is fading in, then the start menu.
        r.with(GET_ENTER_STACK, |_, a| {
            eax(if a[1] == 0 { 0x3f0 } else { 0x3f5 })
        });
        r.stub(TILE_GET_MENU_BY_CLASS, 0x7a00);
        r.stub(TILE_GET_MENU, 0x7b00);
        r.stub(MENU_FADE_STATE, 1);
        r.stub(IS_IN_RENDERED_MENU, 1);
        r.stub(START_MENU_CLASS_GETTER, 0x3f5);
        r.e.set_global(MAIN_OBJECT, 0x4d41u32);
        let added = Rc::new(RefCell::new(Vec::new()));
        let log = added.clone();
        r.with(POINTER_LIST_ADD, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            eax(0)
        });
        r.e.call(0x007c_baf0, &args![menu, 1u32, 0u32]);
        // The menu tile word (+4) is 0 here, so the alpha lands on tile 0.
        assert_eq!(r.tile(0x8000 + 0x34, TRAIT_VISIBLE), 0.0);
        assert_eq!(r.tile(0, TRAIT_ALPHA), 64.0);
        assert_eq!(
            r.calls_to(FADE),
            vec![vec![menu.addr(), 0, 1, 0x8000_0000, (-1.0f32).to_bits()]]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED)[0],
            vec![menu.addr() + 0x84, 1]
        );
        assert_eq!(r.calls_to(FN_007D3AA0), vec![vec![menu.addr()]]);
        // The model scale goes to the model of the tile at +0x5c.
        assert_eq!(
            r.calls_to(MODEL_SET_SCALE),
            vec![vec![0x9900, 1.5f32.to_bits()]]
        );
        // The hidden menu's tile is hidden and remembered in the script list.
        assert_eq!(r.tile(0x7a00, TRAIT_VISIBLE), 0.0);
        assert_eq!(*added.borrow(), vec![(SCRIPT_LIST, 0x7a00)]);
        assert_eq!(r.calls_to(RENDER_MENU_BACKGROUND), vec![vec![0x4d41]]);
        assert!(r.calls_to(VATS_SET_MODE).is_empty());
    }

    #[test]
    fn create_inner_logs_a_missing_background_model() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.call(0x007c_baf0, &args![menu, 1u32, 0u32]);
        assert_eq!(
            r.calls_to(LOG_MESSAGE),
            vec![vec![0x12, TEXT_BACKGROUND_MODEL_NULL]]
        );
        assert!(r.calls_to(MODEL_SET_SCALE).is_empty());
    }

    #[test]
    fn create_inner_adds_the_options_its_modes_allow() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.stub(SAVE_MANAGER_CAN_SAVE, 1);
        let objects = options_array(
            &mut r,
            &[
                (0xaa01, CHOOSE_007D0440, 2),
                (0xaa02, 0x1234, 4),
                (0xaa03, CHOOSE_007D0550, 1),
                (0xaa04, CHOOSE_007D0680, 3),
            ],
        );
        r.e.call(0x007c_baf0, &args![menu, 1u32, 1u32]);
        let adds = r.calls_to(LIST_BOX_ADD_ITEM);
        // With the in-game flag: mode bit 2 goes to the main list, else
        // mode bit 4 to the settings list; mode 1 alone is not added.
        assert_eq!(
            adds,
            vec![
                vec![menu.addr() + 0x84, objects[0], 0, 0, 0],
                vec![menu.addr() + 0xb4, objects[1], 0, 0, 0],
                vec![menu.addr() + 0x84, objects[3], 0, 0, 0],
            ]
        );
        let item0 = 0x7000 + objects[0] % 0x1000;
        let item3 = 0x7000 + objects[3] % 0x1000;
        // Each added item gets the option's text.
        let strings = r.calls_to(TILE_SET_STRING);
        assert!(strings.contains(&vec![item0, TRAIT_STRING, 0xaa01, 1]));
        assert!(strings.contains(&vec![item3, TRAIT_STRING, 0xaa04, 1]));
        // The remembered tiles: continue (+0x1c4) and the save/load pair.
        assert_eq!(r.e.mem.u32(menu.addr() + 0x1c4), item0);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x1c0), item3);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x1bc), 0);
        // Continue with both flags set is greyed out: enabled trait 0.
        assert_eq!(r.tile(item0, 0x5a5a), 0.0);
        // The save/load option is greyed only without a save directory
        // (the file search finds one here): its enabled trait is untouched.
        assert!(r.tile(item3, 0x5a5a).is_nan());
    }

    #[test]
    fn create_inner_main_menu_modes_use_bit_one() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.stub(SAVE_MANAGER_CAN_SAVE, 1);
        let objects = options_array(
            &mut r,
            &[
                (0xbb01, 0x1111, 1),
                (0xbb02, 0x2222, 2),
                (0xbb03, CHOOSE_007D0550, 4),
            ],
        );
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        let adds = r.calls_to(LIST_BOX_ADD_ITEM);
        assert_eq!(
            adds,
            vec![
                vec![menu.addr() + 0x84, objects[0], 0, 0, 0],
                vec![menu.addr() + 0xb4, objects[2], 0, 0, 0],
            ]
        );
        // The downloads option's tile is remembered at +0x1bc.
        assert_eq!(
            r.e.mem.u32(menu.addr() + 0x1bc),
            0x7000 + objects[2] % 0x1000
        );
        // Nothing was greyed: no enabled-trait writes beyond the setup.
        assert!(r.calls_to(TILE_SET_INT).iter().all(|c| c[1] != 0x5a5a));
    }

    #[test]
    fn create_inner_greys_options_that_cannot_run() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        // Saving not possible, no save files, help menu already open.
        r.stub(SAVE_MANAGER_CAN_SAVE, 0);
        r.stub(FIND_FIRST_FILE, 0xffff_ffff);
        r.stub(TILE_GET_MENU_BY_CLASS, 0x7a00);
        let objects = options_array(
            &mut r,
            &[
                (0xcc01, CHOOSE_007D06C0, 1),
                (0xcc02, CHOOSE_007D0440, 1),
                (0xcc03, CHOOSE_007D0680, 1),
                (0xcc04, CHOOSE_HELP, 1),
                (0xcc05, 0x3333, 1),
            ],
        );
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        let grayed: Vec<bool> = objects
            .iter()
            .map(|o| r.tile(0x7000 + o % 0x1000, 0x5a5a) == 0.0)
            .collect();
        assert_eq!(grayed, vec![true, true, true, true, false]);
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS), vec![vec![0x423]]);
    }

    #[test]
    fn create_inner_new_game_greys_the_continue_option_only_with_both_flags() {
        for (a, b, greyed) in [(1u32, 1u32, true), (1, 0, false), (0, 0, true)] {
            let mut r = rig();
            let menu = inner_scene(&mut r);
            // A found save file: the flag-a-clear case is greyed only when
            // nothing is found.
            r.stub(FIND_FIRST_FILE, if a == 0 { 0xffff_ffff } else { 5 });
            let objects = options_array(&mut r, &[(0xdd01, CHOOSE_007D0440, 1 | 2)]);
            r.e.call(0x007c_baf0, &args![menu, a, b]);
            let item = 0x7000 + objects[0] % 0x1000;
            assert_eq!(r.tile(item, 0x5a5a) == 0.0, greyed, "flags {a} {b}");
        }
    }

    #[test]
    fn create_inner_builds_the_save_path_without_a_save_manager() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.stub(BASE_PATH_GETTER, 0x1202fa0);
        r.stub(SETTING_STRING, 0xf01d);
        r.with(IDENTITY, |_, a| eax(a[0] + 1));
        r.with(FIND_FIRST_FILE, |_, _| eax(7));
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        let copy = &r.calls_to(STRING_COPY)[0];
        assert_eq!(copy[1..], [0x104, 0x1202fa0]);
        let path = copy[0];
        assert_eq!(
            r.calls_to(STRING_APPEND),
            vec![
                vec![path, 0x104, 0xf01e],
                vec![path, 0x104, TEXT_PATH_SEPARATOR],
                vec![path, 0x104, TEXT_SAVE_PATTERN],
            ]
        );
        assert_eq!(r.calls_to(IDENTITY), vec![vec![0xf01d, 0]]);
        assert_eq!(r.calls_to(FIND_FIRST_FILE)[0][0], path);
        assert_eq!(r.calls_to(FIND_CLOSE), vec![vec![7]]);
        assert!(r.calls_to(SAVE_MANAGER_PATH).is_empty());
    }

    #[test]
    fn create_inner_asks_the_save_manager_for_the_path_when_there_is_one() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.set_global(CALLBACK_OWNER, 0x6100u32);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        let ask = &r.calls_to(SAVE_MANAGER_PATH)[0];
        assert_eq!(ask[0], 0x6100);
        assert!(r.calls_to(BASE_PATH_GETTER).is_empty());
        let appended: Vec<u32> = r.calls_to(STRING_APPEND).iter().map(|c| c[2]).collect();
        assert_eq!(appended, vec![TEXT_PATH_SEPARATOR, TEXT_SAVE_PATTERN]);
    }

    #[test]
    fn create_inner_labels_and_counts() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.stub(LIST_BOX_COUNT, 4);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 1));
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        // Visible-item counts for the main and settings lists.
        assert_eq!(
            r.calls_to(TILE_SET_COUNT),
            vec![
                vec![0x8000 + 0x28, 0x6006, 4],
                vec![0x8000 + 0x2c, 0x6006, 4]
            ]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_COUNT),
            vec![vec![menu.addr() + 0x84], vec![menu.addr() + 0xb4]]
        );
        let strings = r.calls_to(TILE_SET_STRING);
        assert!(strings.contains(&vec![0x8000 + 0x50, TRAIT_STRING, TEXT_VERSION, 1]));
        for (tile, setting) in [
            (0x4c, SETTING_LABEL_A),
            (0x78, SETTING_LABEL_B),
            (0x64, SETTING_LABEL_C),
            (0x3c, SETTING_LABEL_D),
        ] {
            assert!(strings.contains(&vec![0x8000 + tile, TRAIT_STRING, setting + 1, 1]));
        }
        assert_eq!(r.tile(0x8000 + 0x7c, TRAIT_VISIBLE), 0.0);
    }

    #[test]
    fn create_inner_adapts_to_the_screen() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        let menu_tile = 0x8000 + 4;
        r.e.mem.set_u32(menu.addr() + 4, menu_tile);
        // A narrow screen moves the tile at +0x34 down by 25; a wide aspect
        // ratio sets the menu tile's eUser0 to 1.
        r.stub_float(SCREEN_WIDTH, 600.0);
        r.stub_float(SCREEN_ASPECT_RATIO, 1.7);
        r.set_tile(0x8000 + 0x34, TRAIT_Y, 10.0);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        assert_eq!(r.tile(0x8000 + 0x34, TRAIT_Y), 35.0);
        assert_eq!(r.tile(menu_tile, TRAIT_USER_0), 1.0);
        // A wide screen with a square aspect ratio changes neither.
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.mem.set_u32(menu.addr() + 4, menu_tile);
        r.stub_float(SCREEN_WIDTH, 1920.0);
        r.stub_float(SCREEN_ASPECT_RATIO, 1.0);
        r.set_tile(0x8000 + 0x34, TRAIT_Y, 10.0);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        assert_eq!(r.tile(0x8000 + 0x34, TRAIT_Y), 10.0);
        assert_eq!(r.tile(menu_tile, TRAIT_USER_0), 0.0);
    }

    #[test]
    fn create_inner_plays_the_credits_music_when_the_setting_is_nonzero() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.set_global(CREDITS_SETTING, 0x9a00u32);
        r.stub_float(SETTING_FLOAT, 1.0);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        let flags = r.e.get(menu, StartMenu::uiFlags);
        assert_eq!(flags & 0x4001_0000, 0x4001_0000);
        let copy = &r.calls_to(STRING_COPY_NARROW)[0];
        assert_eq!(copy[1..], [0x40, TEXT_END_CREDITS_MUSIC]);
        let play = &r.calls_to(MEDIA_PLAY)[0];
        assert_eq!(play[0], 8);
        assert_eq!(play[2..], [0x7d0, 1, 1, 0.0f32.to_bits(), 0]);
        assert_eq!(r.calls_to(MEMSET)[0][1..], [0, 0x40]);
        assert_eq!(r.calls_to(MEDIA_FINISH).len(), 1);
        // A zero credits value does not play it.
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.set_global(CREDITS_SETTING, 0x9a00u32);
        r.stub_float(SETTING_FLOAT, 0.0);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        assert!(r.calls_to(MEDIA_PLAY).is_empty());
    }

    #[test]
    fn create_inner_closes_with_the_pause_mode_the_dialog_pause_and_the_hooks() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.stub(XUI_IS_UP, 1);
        r.stub(IN_DIALOG, 1);
        r.stub(NETWORK_UTILITY_INSTANCE, 0xee00);
        let sign_in = r.alloc(0x300);
        r.e.set_global(SIGN_IN_OBJECT, sign_in);
        r.e.mem.set_u32(sign_in + 0x244, 0xff);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        assert_eq!(r.calls_to(CLOSE_PAUSE_HELPER), vec![vec![0]]);
        assert!(r.calls_to(SET_PAUSE_MENU_MODE).is_empty());
        assert_eq!(r.calls_to(SET_DIALOG_PAUSE), vec![vec![1]]);
        assert_eq!(r.calls_to(PREP_FOR_VISIBILITY), vec![vec![menu.addr(), 0]]);
        assert_eq!(
            r.calls_to(NETWORK_UTILITY_SET_CALLBACK),
            vec![vec![0xee00, NETWORK_CALLBACK]]
        );
        // The sign-in object has bit 0x80 of its word at +0x244 cleared.
        assert_eq!(r.e.mem.u32(sign_in + 0x244), 0x7f);
        // Flag 1 clear: the "_xbox_extra" trait gets 0x19.
        assert_eq!(
            r.calls_to(TILE_ADD_USER_TRAIT).last().unwrap()[0],
            TEXT_XBOX_EXTRA
        );
        assert_eq!(r.tile(0x8000 + 0x28, 0x6006), 25.0);
    }

    #[test]
    fn create_inner_uses_the_pause_menu_mode_without_the_xui() {
        let mut r = rig();
        let menu = inner_scene(&mut r);
        r.e.call(0x007c_baf0, &args![menu, 0u32, 0u32]);
        assert_eq!(r.calls_to(SET_PAUSE_MENU_MODE), vec![vec![1]]);
        assert!(r.calls_to(CLOSE_PAUSE_HELPER).is_empty());
        assert!(r.calls_to(SET_DIALOG_PAUSE).is_empty());
    }

    // ----- 007cc5d0 .. 007cc680 -----

    #[test]
    fn option_mode_test_masks_the_modes_word() {
        let mut r = rig();
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option + 0xc, 0b0110);
        assert!(r
            .e
            .call(0x007c_c5d0, &args![Ptr::<()>::new(option), 2u32])
            .bool());
        assert!(r
            .e
            .call(0x007c_c5d0, &args![Ptr::<()>::new(option), 3u32])
            .bool());
        assert!(!r
            .e
            .call(0x007c_c5d0, &args![Ptr::<()>::new(option), 1u32])
            .bool());
        assert!(!r
            .e
            .call(0x007c_c5d0, &args![Ptr::<()>::new(option), 0u32])
            .bool());
    }

    #[test]
    fn hidden_menu_classes_are_the_twenty_of_the_table() {
        let mut r = rig();
        let hidden = [
            0x3f0, 0x3f1, 0x3f4, 0x3f6, 0x3f8, 0x403, 0x417, 0x418, 0x41b, 0x41d, 0x420, 0x423,
            0x433, 0x434, 0x435, 0x438, 0x439, 0x43a, 0x43b, 0x43c,
        ];
        for class in 0x3e0u32..0x450 {
            assert_eq!(
                r.e.call(0x007c_c5f0, &args![class]).bool(),
                hidden.contains(&class),
                "class {class:#x}"
            );
        }
    }

    #[test]
    fn flag_080_of_the_word_at_0x244_is_set_or_cleared() {
        let mut r = rig();
        let object = r.alloc(0x300);
        r.e.mem.set_u32(object + 0x244, 0x1234_0001);
        r.e.call(0x007c_c680, &args![Ptr::<()>::new(object), 1u32]);
        assert_eq!(r.e.mem.u32(object + 0x244), 0x1234_0081);
        r.e.call(0x007c_c680, &args![Ptr::<()>::new(object), 0u32]);
        assert_eq!(r.e.mem.u32(object + 0x244), 0x1234_0001);
    }

    // ----- 007ce620, 007ce660, 007ce690 -----

    #[test]
    fn option_constructor_fills_the_fields() {
        let mut r = rig();
        let option = r.alloc(0x10);
        let back =
            r.e.call(
                0x007c_e620,
                &args![Ptr::<()>::new(option), 0xaau32, 0xbbu32, 0xccu32],
            )
            .u32();
        assert_eq!(back, option);
        assert_eq!(r.e.mem.u32(option), VTABLE_OPTION);
        assert_eq!(r.e.mem.u32(option + 4), 0xaa);
        assert_eq!(r.e.mem.u32(option + 8), 0xbb);
        assert_eq!(r.e.mem.u32(option + 0xc), 0xcc);
    }

    #[test]
    fn option_destructor_resets_the_vtable_and_frees_on_request() {
        let mut r = rig();
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option, 0xdead);
        r.e.call(0x007c_e690, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option), VTABLE_OPTION);
        r.e.mem.set_u32(option, 0xdead);
        assert_eq!(
            r.e.call(0x007c_e660, &args![Ptr::<()>::new(option), 0u32])
                .u32(),
            option
        );
        assert!(r.calls_to(OPERATOR_DELETE).is_empty());
        assert_eq!(r.e.mem.u32(option), VTABLE_OPTION);
        r.e.call(0x007c_e660, &args![Ptr::<()>::new(option), 1u32]);
        assert_eq!(r.calls_to(OPERATOR_DELETE), vec![vec![option]]);
    }

    // ----- 007ce6b0 -----

    #[test]
    fn clearing_the_options_deletes_every_object_and_empties_both_arrays() {
        let mut r = rig();
        let vtable = r.vtable_with(0, 0);
        let first = r.object(vtable);
        let second = r.object(vtable);
        let slots: Vec<u32> = [first, 0, second]
            .iter()
            .map(|object| {
                let slot = r.alloc(4);
                r.e.mem.set_u32(slot, *object);
                slot
            })
            .collect();
        let user_slot = r.alloc(4);
        r.e.mem.set_u32(user_slot, first);
        r.with(ARRAY_SIZE, |_, a| {
            eax(if a[0] == OPTIONS_ARRAY { 3 } else { 1 })
        });
        r.with(ARRAY_ELEMENT, move |_, a| {
            eax(if a[0] == OPTIONS_ARRAY {
                slots[a[1] as usize]
            } else {
                user_slot
            })
        });
        r.e.call(0x007c_e6b0, &args![]);
        // The null entry is skipped; the user option array has one entry.
        let delete = 0x0eee_0000 + vtable;
        assert_eq!(
            r.calls_to(delete),
            vec![vec![first, 1], vec![second, 1], vec![first, 1]]
        );
        assert_eq!(
            r.calls_to(ARRAY_CLEAR),
            vec![vec![OPTIONS_ARRAY, 1], vec![USER_OPTIONS_ARRAY, 1]]
        );
        assert_eq!(
            r.calls_to(SAVE_LOAD_ENTRIES_CLEAR),
            vec![vec![SAVE_LOAD_ENTRIES, 1]]
        );
    }

    // ----- 007cc6e0 -----

    /// Records `(array, item)` for every `BSSimpleArray::Add`, makes
    /// `operator new` hand out real blocks and the option text/setting
    /// getters return their `this`.
    fn elements_scene(r: &mut Rig) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let added = Rc::new(RefCell::new(Vec::new()));
        let log = added.clone();
        r.with(ARRAY_ADD, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            eax(0)
        });
        r.with(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        r.with(OPTION_TEXT, |_, a| eax(a[0] + 1));
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        r.with(USER_OPTION_NEW, |_, a| eax(a[0]));
        r.stub(FIRST_WORD, 0);
        added
    }

    #[test]
    fn create_menu_elements_builds_the_23_options_and_34_user_options() {
        let mut r = rig();
        let added = elements_scene(&mut r);
        r.e.call(0x007c_c6e0, &args![]);
        let added = added.borrow();
        assert_eq!(added.len(), 57);
        assert!(added[..23].iter().all(|(array, _)| *array == OPTIONS_ARRAY));
        assert!(added[23..]
            .iter()
            .all(|(array, _)| *array == USER_OPTIONS_ARRAY));
        // The first option: text of its setting, choose function, modes.
        let first = added[0].1;
        assert_eq!(r.e.mem.u32(first), VTABLE_OPTION);
        assert_eq!(r.e.mem.u32(first + 4), 0x011d_3564 + 1);
        assert_eq!(r.e.mem.u32(first + 8), 0x007d_0440);
        assert_eq!(r.e.mem.u32(first + 0xc), 3);
        // The last: the Help option (0x7d0770) has modes 2.
        let help = added[5].1;
        assert_eq!(r.e.mem.u32(help + 8), 0x007d_0770);
        assert_eq!(r.e.mem.u32(help + 0xc), 2);
        // 23 option allocations of 0x10 and 34 of 0x30.
        let sizes: Vec<u32> = r.calls_to(OPERATOR_NEW).iter().map(|c| c[0]).collect();
        assert_eq!(sizes.iter().filter(|s| **s == 0x10).count(), 23);
        assert_eq!(sizes.iter().filter(|s| **s == 0x30).count(), 34);
        assert_eq!(r.calls_to(USER_OPTION_NEW).len(), 34);
    }

    #[test]
    fn create_menu_elements_passes_the_value_strings_in_reverse() {
        let mut r = rig();
        let _ = elements_scene(&mut r);
        r.e.call(0x007c_c6e0, &args![]);
        let user = r.calls_to(USER_OPTION_NEW);
        // First user option: text setting 011d366c, choose 007d1260, modes 8,
        // type 0, three settings, arrow spacing 0x55, then the three value
        // strings last-read first (each is its setting + 2 here).
        assert_eq!(
            user[0][1..],
            [
                0x011d_366c + 1,
                0x007d_1260,
                8,
                0,
                3,
                0x55,
                0x011d_4ed0 + 2,
                0x011d_466c + 2,
                0x011d_4fc0 + 2,
            ]
        );
        // A user option without value strings has seven words.
        assert_eq!(user[7].len(), 7);
        // The last one has two value strings.
        assert_eq!(user[33].len(), 9);
        assert_eq!(user[33][2], 0x007d_30e0);
        // The strings are read before the text: for the first user option
        // three setting reads precede its text read.
        let order = r.order();
        let new = order.iter().position(|a| *a == USER_OPTION_NEW).unwrap();
        assert_eq!(order[new - 1], OPTION_TEXT);
        assert_eq!(&order[new - 4..new - 1], [SETTING_STRING; 3]);
    }

    #[test]
    fn create_menu_elements_stores_a_null_option_when_the_allocation_fails() {
        let mut r = rig();
        let added = elements_scene(&mut r);
        r.stub(OPERATOR_NEW, 0);
        r.e.call(0x007c_c6e0, &args![]);
        let added = added.borrow();
        assert_eq!(added.len(), 57);
        assert!(added.iter().all(|(_, item)| *item == 0));
        assert!(r.calls_to(OPTION_TEXT).is_empty());
        assert!(r.calls_to(USER_OPTION_NEW).is_empty());
    }

    #[test]
    fn create_menu_elements_fills_the_two_strings_only_when_the_first_is_empty() {
        let mut r = rig();
        let _ = elements_scene(&mut r);
        r.e.call(0x007c_c6e0, &args![]);
        assert_eq!(
            r.calls_to(STRING_SET),
            vec![
                vec![STRING_A, SETTING_STRING_A + 2, 0],
                vec![STRING_B, SETTING_STRING_B + 2, 0]
            ]
        );
        assert_eq!(
            r.calls_to(STRING_UPDATE),
            vec![vec![STRING_A], vec![STRING_B]]
        );
        let mut r = rig();
        let _ = elements_scene(&mut r);
        r.stub(FIRST_WORD, 0x1234);
        r.e.call(0x007c_c6e0, &args![]);
        assert!(r.calls_to(STRING_SET).is_empty());
        assert!(r.calls_to(STRING_UPDATE).is_empty());
    }

    #[test]
    fn create_menu_elements_clears_the_old_options_first() {
        let mut r = rig();
        let _ = elements_scene(&mut r);
        r.e.call(0x007c_c6e0, &args![]);
        let order = r.order();
        let clear = order.iter().position(|a| *a == ARRAY_CLEAR).unwrap();
        let first_add = order.iter().position(|a| *a == ARRAY_ADD).unwrap();
        assert!(clear < first_add);
    }

    impl Rig {
        /// Gives the list box at `menu + offset` a vtable whose slots 0
        /// (`Highlight`/delete), 4 (highlighted item, returns `highlight`)
        /// and 0x1c (`RemoveAll`) are doubles; returns their addresses.
        fn list_vtable(&mut self, menu: Ptr<StartMenu>, offset: u32, highlight: u32) -> [u32; 3] {
            let vtable = self.alloc(0x40);
            let base = 0x0eee_2000 + offset * 0x10;
            let targets = [base, base + 4, base + 0x1c];
            self.e.mem.set_u32(vtable, targets[0]);
            self.e.mem.set_u32(vtable + 4, targets[1]);
            self.e.mem.set_u32(vtable + 0x1c, targets[2]);
            self.e.mem.set_u32(menu.addr() + offset, vtable);
            self.stub(targets[0], 0);
            self.stub(targets[1], highlight);
            self.stub(targets[2], 0);
            targets
        }
    }

    // ----- 007ce7a0 -----

    #[test]
    fn close_resets_the_pipboy_colours_when_the_pipboy_is_rendered() {
        let mut r = rig();
        r.stub(START_MENU_FLAG_1, 1);
        r.stub(CURRENT_RENDERED_MENU, 0x55);
        r.stub(GET_PIPBOY, 0x55);
        r.e.call(0x007c_e7a0, &args![1u32]);
        assert_eq!(r.calls_to(PIPBOY_RESET_COLORS), vec![vec![0x55]]);
        // No start menu: nothing else happens.
        assert!(r.calls_to(IN_DIALOG).is_empty());
        // Another rendered menu: no reset.
        let mut r = rig();
        r.stub(START_MENU_FLAG_1, 1);
        r.stub(CURRENT_RENDERED_MENU, 0x56);
        r.stub(GET_PIPBOY, 0x55);
        r.e.call(0x007c_e7a0, &args![1u32]);
        assert!(r.calls_to(PIPBOY_RESET_COLORS).is_empty());
    }

    #[test]
    fn close_waits_while_a_control_is_being_remapped() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.set(menu, StartMenu::uiFlags, 0x1000);
        r.list_vtable(menu, 0x114, 0x33);
        r.stub(IS_TOP_MENU_ID, 1);
        r.stub(LIST_BOX_IS_SELECTED, 1);
        r.e.call(0x007c_e7a0, &args![1u32]);
        assert_eq!(
            r.calls_to(LIST_BOX_IS_SELECTED),
            vec![vec![menu.addr() + 0x114, 0x33]]
        );
        assert!(r.calls_to(IN_DIALOG).is_empty());
        assert!(r.calls_to(MENU_START_FADE_OUT).is_empty());
        assert_eq!(r.e.global::<u32>(START_MENU), menu.addr());
    }

    #[test]
    fn close_fades_the_menu_out_and_restores_the_hidden_tiles() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 4, 0x4400);
        r.stub(IN_DIALOG, 1);
        r.stub(START_MENU_FLAG_1, 1);
        r.stub(FIRST_WORD, 1);
        let slot = r.alloc(4);
        r.e.mem.set_u32(slot, 0x77);
        r.with(POINTER_LIST_STEP_GLOBAL, move |e, a| {
            e.mem.set_u32(a[1], 0);
            eax(slot)
        });
        r.stub(IS_IN_RENDERED_MENU, 1);
        r.e.set_global(MAIN_OBJECT, 0x4d41u32);
        r.e.call(0x007c_e7a0, &args![1u32]);
        assert_eq!(r.calls_to(SET_DIALOG_PAUSE), vec![vec![0]]);
        assert_eq!(r.calls_to(SET_PAUSE_MENU_MODE), vec![vec![0]]);
        assert_eq!(r.calls_to(POINTER_LIST_STEP_GLOBAL)[0][0], SCRIPT_LIST);
        assert_eq!(r.tile(0x77, TRAIT_VISIBLE), 1.0);
        assert_eq!(r.tile(0x4400, TRAIT_DELETE_ON_FADE), 1.0);
        assert_eq!(r.calls_to(MENU_START_FADE_OUT), vec![vec![menu.addr()]]);
        assert_eq!(r.e.global::<u32>(START_MENU), 0);
        assert_eq!(r.calls_to(RENDER_MENU_BACKGROUND), vec![vec![0x4d41]]);
        // No remap flag: the options list is not walked.
        assert!(r.calls_to(LIST_BOX_FOR_EACH).is_empty());
    }

    #[test]
    fn close_with_the_remap_flag_deletes_the_list_items_and_passes_the_argument() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 4, 0x4400);
        r.e.set(menu, StartMenu::uiFlags, 0x1000);
        r.e.call(0x007c_e7a0, &args![0u32]);
        assert_eq!(
            r.calls_to(LIST_BOX_FOR_EACH),
            vec![vec![menu.addr() + 0x114, FN_007CE970, 0, 0x7fff_ffff]]
        );
        assert_eq!(r.tile(0x4400, TRAIT_DELETE_ON_FADE), 0.0);
    }

    #[test]
    fn close_leaves_the_menu_alone_with_flag_4_but_still_redraws() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.set(menu, StartMenu::uiFlags, 4);
        r.stub(IS_IN_RENDERED_MENU, 1);
        r.e.call(0x007c_e7a0, &args![1u32]);
        assert!(r.calls_to(MENU_START_FADE_OUT).is_empty());
        assert_eq!(r.e.global::<u32>(START_MENU), menu.addr());
        assert_eq!(r.calls_to(RENDER_MENU_BACKGROUND).len(), 1);
    }

    // ----- 007ce8f0, 007ce970 -----

    #[test]
    fn waiting_for_a_remap_needs_the_top_menu_the_flag_and_a_selection() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.list_vtable(menu, 0x114, 0x33);
        assert!(!r
            .e
            .call(0x007c_e8f0, &args![Ptr::<()>::new(menu.addr())])
            .bool());
        r.stub(IS_TOP_MENU_ID, 1);
        assert!(!r
            .e
            .call(0x007c_e8f0, &args![Ptr::<()>::new(menu.addr())])
            .bool());
        r.e.set(menu, StartMenu::uiFlags, 0x1000);
        assert!(!r
            .e
            .call(0x007c_e8f0, &args![Ptr::<()>::new(menu.addr())])
            .bool());
        r.stub(LIST_BOX_IS_SELECTED, 1);
        assert!(r
            .e
            .call(0x007c_e8f0, &args![Ptr::<()>::new(menu.addr())])
            .bool());
        assert_eq!(r.calls_to(IS_TOP_MENU_ID)[0], vec![0x3f5]);
    }

    #[test]
    fn the_delete_callback_calls_the_virtual_destructor() {
        let mut r = rig();
        let vtable = r.vtable_with(0, 0x42);
        let item = r.object(vtable);
        assert_eq!(r.e.call(0x007c_e970, &args![7u32, item]).u32(), 0x42);
        assert_eq!(r.calls_to(0x0eee_0000 + vtable), vec![vec![item, 1]]);
        assert_eq!(r.e.call(0x007c_e970, &args![7u32, 0u32]).u32(), 0);
        assert_eq!(r.calls_to(0x0eee_0000 + vtable).len(), 1);
    }

    // ----- 007ce9b0 -----

    /// A menu with real flags, the option and save-load lists given
    /// vtables, and the interface values of a click after the menu was
    /// created 1000 ticks ago.
    fn click_scene(r: &mut Rig) -> Ptr<StartMenu> {
        r.real_flags();
        let menu = r.menu();
        r.list_vtable(menu, 0x114, 0x4040);
        r.list_vtable(menu, 0x174, 0x4041);
        r.e.mem.set_u32(menu.addr() + 4, 0x4400);
        r.e.mem.set_u32(menu.addr() + 0x40, 0x4500);
        r.stub(TICK_COUNT, 2000);
        r.e.set_global(TICK_CREATED, 1000u32);
        r.stub(0x0eee_7000, 0);
        menu
    }

    /// `ListBox::GetItem` double: the item for `(list, tile)`.
    fn items(r: &mut Rig, table: &[(u32, u32, u32)]) {
        let table = table.to_vec();
        r.with(LIST_BOX_GET_ITEM, move |_, a| {
            eax(table
                .iter()
                .find(|(list, tile, _)| *list == a[0] && *tile == a[1])
                .map_or(0, |(_, _, item)| *item))
        });
    }

    #[test]
    fn click_5_chooses_back_and_flag_400_stops_the_rest() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x400);
        r.e.call(0x007c_e9b0, &args![menu, 5i32, 0x9000u32]);
        assert_eq!(r.calls_to(CHOOSE_BACK), vec![vec![]]);
        // Flag 400: nothing after, not even the tick count.
        assert!(r.calls_to(TICK_COUNT).is_empty());
    }

    #[test]
    fn click_4_confirms_the_selected_save_game() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.stub(SAVE_LOAD_LIST_IS_ENABLED, 1);
        r.e.call(0x007c_e9b0, &args![menu, 4i32, 0u32]);
        assert_eq!(
            r.order(),
            vec![
                MENU_FLAGS_TEST,
                SAVE_LOAD_LIST_IS_ENABLED,
                MENU_FLAGS_SET,
                CONFIRM_SAVE_LOAD,
                MENU_FLAGS_SET
            ]
        );
        assert_eq!(
            r.calls_to(MENU_FLAGS_SET),
            vec![vec![menu.addr(), 0x2000, 1], vec![menu.addr(), 0x2000, 0]]
        );
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0);
    }

    #[test]
    fn click_4_in_remap_mode_asks_for_confirmation() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x1000);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        r.with(OPTION_TEXT, |_, a| eax(a[0] + 1));
        r.e.call(0x007c_e9b0, &args![menu, 4i32, 0u32]);
        assert_eq!(
            r.calls_to(SHOW_MESSAGE),
            vec![vec![
                SETTING_RESET_TEXT + 1,
                0,
                0,
                MESSAGE_CALLBACK,
                0,
                0x12,
                0,
                0,
                SETTING_RESET_BUTTON_B + 2,
                SETTING_RESET_BUTTON_A + 2,
                0
            ]]
        );
        assert!(r.calls_to(LIST_BOX_FOR_EACH).is_empty());
    }

    #[test]
    fn click_4_otherwise_resets_every_option_to_its_default() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.call(0x007c_e9b0, &args![menu, 4i32, 0u32]);
        assert_eq!(
            r.calls_to(LIST_BOX_FOR_EACH),
            vec![vec![menu.addr() + 0x114, FN_007CFDC0, 0, 0x7fff_ffff]]
        );
    }

    #[test]
    fn click_0xf_refreshes_the_controls_list_when_something_is_selected() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.with(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        r.e.call(0x007c_e9b0, &args![menu, 0xfi32, 0u32]);
        assert!(r.calls_to(LIST_BOX_FOR_EACH).is_empty());
        r.stub(OPTIONS_LIST_HAS_SELECTION, 1);
        r.e.call(0x007c_e9b0, &args![menu, 0xfi32, 0u32]);
        let each = r.calls_to(LIST_BOX_FOR_EACH);
        assert_eq!(
            each[0],
            vec![menu.addr() + 0x114, FN_007CE970, 0, 0x7fff_ffff]
        );
        assert_eq!(r.calls_to(MENU_REFRESH_A), vec![vec![menu.addr()]]);
        // Then the controls list is rebuilt (`007d01b0`): its RemoveAll.
        assert_eq!(r.calls_to(0x0eee_2000 + 0x114 * 0x10 + 0x1c).len(), 1);
        assert_eq!(each[1][1], FN_007CFE40);
    }

    #[test]
    fn click_1_opens_the_settings() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.call(0x007c_e9b0, &args![menu, 1i32, 0u32]);
        let relevant: Vec<u32> = r
            .order()
            .into_iter()
            .filter(|a| {
                [
                    HIDE_OPTIONS_CONTAINER,
                    LIST_BOX_SET_ENABLED,
                    FADE,
                    LIST_BOX_DESELECT_ALL,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            relevant,
            vec![
                HIDE_OPTIONS_CONTAINER,
                LIST_BOX_SET_ENABLED,
                FADE,
                LIST_BOX_DESELECT_ALL,
                FADE
            ]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED),
            vec![vec![menu.addr() + 0xb4, 1]]
        );
    }

    #[test]
    fn click_0_goes_back_from_the_save_load_page() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.stub(INT_LIST_IS_ENABLED, 1);
        r.e.set(menu, StartMenu::uiFlags, 2);
        r.e.call(0x007c_e9b0, &args![menu, 0i32, 0u32]);
        let watched = [
            HIDE_OPTIONS_CONTAINER,
            INT_LIST_IS_ENABLED,
            FADE,
            INT_LIST_SET_ENABLED,
            LIST_BOX_SET_ENABLED,
            LIST_BOX_DESELECT_ALL,
            SAVE_LOAD_LIST_SET_ENABLED,
            0x0eee_2000 + 0x174 * 0x10,
            UPDATE_HIGHLIGHTED_SAVE_GAME_INFO,
            MENU_REFRESH_B,
        ];
        let relevant: Vec<u32> = r
            .order()
            .into_iter()
            .filter(|a| watched.contains(a))
            .collect();
        assert_eq!(
            relevant,
            vec![
                HIDE_OPTIONS_CONTAINER,
                INT_LIST_IS_ENABLED,
                FADE,
                INT_LIST_SET_ENABLED,
                FADE,
                LIST_BOX_SET_ENABLED,
                LIST_BOX_DESELECT_ALL,
                SAVE_LOAD_LIST_SET_ENABLED,
                0x0eee_2000 + 0x174 * 0x10,
                UPDATE_HIGHLIGHTED_SAVE_GAME_INFO,
                SAVE_LOAD_LIST_SET_ENABLED,
                FADE,
                LIST_BOX_SET_ENABLED,
                FADE,
                LIST_BOX_SET_ENABLED,
                LIST_BOX_DESELECT_ALL,
                FADE,
                MENU_REFRESH_B,
            ]
        );
        let kinds: Vec<u32> = r.calls_to(FADE).iter().map(|c| c[1]).collect();
        assert_eq!(kinds, vec![0xe, 1, 6, 7, 5]);
        let enables: Vec<(u32, u32)> = r
            .calls_to(LIST_BOX_SET_ENABLED)
            .iter()
            .map(|c| (c[0] - menu.addr(), c[1]))
            .collect();
        assert_eq!(enables, vec![(0xb4, 0), (0xe4, 0), (0x84, 1)]);
        // Flag 2 is consumed; flag 0x40000 cleared.
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0);
        // Flag 0x100000 turns the whole page switch off.
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x10_0000);
        r.e.call(0x007c_e9b0, &args![menu, 0i32, 0u32]);
        assert!(r.calls_to(HIDE_OPTIONS_CONTAINER).is_empty());
    }

    #[test]
    fn a_click_runs_the_choose_function_of_its_option() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option + 8, 0x0eee_7000);
        items(&mut r, &[(menu.addr() + 0xb4, 0x9000, option)]);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert_eq!(
            r.calls_to(LIST_BOX_SELECT),
            vec![vec![menu.addr() + 0xb4, 0x9000]]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED),
            vec![vec![menu.addr() + 0xb4, 0]]
        );
        assert_eq!(r.calls_to(0x0eee_7000), vec![vec![]]);
        assert_eq!(r.e.global::<u32>(TICK_CREATED), 2000);
        // The choose function ended the click: no save lookups.
        assert!(r.calls_to(SAVE_ENTRY_IS_CORRUPT).is_empty());
    }

    #[test]
    fn a_click_on_the_confirm_list_or_back_option_keeps_the_list_enabled() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option + 8, CHOOSE_BACK);
        r.stub(CHOOSE_BACK, 0);
        items(&mut r, &[(menu.addr() + 0x84, 0x9000, option)]);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        // Back: selected, but not disabled.
        assert_eq!(r.calls_to(LIST_BOX_SELECT).len(), 1);
        assert!(r.calls_to(LIST_BOX_SET_ENABLED).is_empty());
        // The confirm list (+0xe4) is never disabled.
        let mut r = rig();
        let menu = click_scene(&mut r);
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option + 8, 0x0eee_7000);
        items(&mut r, &[(menu.addr() + 0xe4, 0x9000, option)]);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert!(r.calls_to(LIST_BOX_SET_ENABLED).is_empty());
        // The 007d0c50 option is not selected either.
        let mut r = rig();
        let menu = click_scene(&mut r);
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option + 8, CHOOSE_007D0C50);
        r.stub(CHOOSE_007D0C50, 0);
        items(&mut r, &[(menu.addr() + 0x84, 0x9000, option)]);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert!(r.calls_to(LIST_BOX_SELECT).is_empty());
    }

    #[test]
    fn a_click_within_500_ticks_of_the_menu_opening_is_ignored() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.stub(TICK_COUNT, 1500);
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option + 8, 0x0eee_7000);
        items(&mut r, &[(menu.addr() + 0xb4, 0x9000, option)]);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert!(r.calls_to(0x0eee_7000).is_empty());
        // Without a tile too.
        r.stub(TICK_COUNT, 5000);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0u32]);
        assert!(r.calls_to(0x0eee_7000).is_empty());
    }

    #[test]
    fn a_click_on_a_save_game_confirms_it_unless_corrupt() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        items(&mut r, &[(menu.addr() + 0x174, 0x9000, 0x5500)]);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert_eq!(r.e.get(menu, StartMenu::pSaveGame), Ptr::new(0x5500));
        assert_eq!(r.calls_to(CONFIRM_SAVE_LOAD), vec![vec![menu.addr()]]);
        // Corrupt and not in load mode: not confirmed.
        let mut r = rig();
        let menu = click_scene(&mut r);
        items(&mut r, &[(menu.addr() + 0x174, 0x9000, 0x5500)]);
        r.stub(SAVE_ENTRY_IS_CORRUPT, 1);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert!(r.calls_to(CONFIRM_SAVE_LOAD).is_empty());
        // Corrupt but flag 8 set: confirmed anyway.
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 8);
        items(&mut r, &[(menu.addr() + 0x174, 0x9000, 0x5500)]);
        r.stub(SAVE_ENTRY_IS_CORRUPT, 1);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert_eq!(r.calls_to(CONFIRM_SAVE_LOAD).len(), 1);
    }

    #[test]
    fn a_click_on_an_empty_save_slot_in_load_mode_selects_it() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 8);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        r.with(LIST_BOX_HAS_TILE, |_, a| eax(u32::from(a[1] == 0x9000)));
        r.stub(LIST_BOX_COUNT, 10);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x9000, TRAIT_STRING, SETTING_SAVE_SELECTED + 2, 1]]
        );
        assert_eq!(
            r.calls_to(SAVE_LOAD_LIST_SET_SELECTION),
            vec![vec![menu.addr() + 0x174, 0x9000]]
        );
        assert_eq!(r.tile(0x4500, TRAIT_USER_0), 0.0);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags) & 0x4000, 0x4000);
        // A list over 1000 entries refuses.
        let mut r = rig();
        let menu = click_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 8);
        r.with(LIST_BOX_HAS_TILE, |_, a| eax(u32::from(a[1] == 0x9000)));
        r.stub(LIST_BOX_COUNT, 1001);
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert!(r.calls_to(SAVE_LOAD_LIST_SET_SELECTION).is_empty());
        assert!(r.calls_to(TILE_SET_STRING).is_empty());
    }

    #[test]
    fn a_click_on_the_downloads_placeholder_opens_the_marketplace() {
        let mut r = rig();
        let menu = click_scene(&mut r);
        let downloads = menu.addr() + 0x144;
        r.with(LIST_BOX_HAS_TILE, move |_, a| {
            eax(u32::from(a[0] == downloads))
        });
        items(&mut r, &[(downloads, 0x9000, 0xffff_ffff)]);
        r.stub(DLC_GETTER, 1);
        r.with(DLC_STEP_A, |_, a| eax(a[0] + 1));
        r.with(DLC_INSTANCE, |_, a| {
            assert!(a.is_empty());
            eax(3)
        });
        r.with(DLC_STEP_B, |_, a| eax(a[0] + 1));
        r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
        assert_eq!(r.calls_to(DLC_OPEN), vec![vec![4, 2]]);
    }

    /// A user option: value, default, count, type, change function.
    fn user_option(r: &mut Rig, value: u32, count: u32, kind: u32) -> u32 {
        let object = r.alloc(0x30);
        r.e.mem.set_u32(object + 0x14, value);
        r.e.mem.set_u32(object + 0x1c, kind);
        r.e.mem.set_u32(object + 0x20, count);
        object
    }

    #[test]
    fn a_click_on_an_arrow_steps_the_user_option() {
        for (code, value, expected) in [(100, 0, 2), (101, 2, 0), (101, 0, 1)] {
            let mut r = rig();
            let menu = click_scene(&mut r);
            let option = user_option(&mut r, value, 3, 2);
            r.with(TILE_PARENT, |_, _| eax(0x9100));
            items(&mut r, &[(menu.addr() + 0x114, 0x9100, 0x6600)]);
            r.stub(RT_DYNAMIC_CAST, option);
            r.set_tile(0x4400, TRAIT_BRIGHTNESS, 0.5);
            r.e.call(0x007c_e9b0, &args![menu, code, 0x9000u32]);
            assert_eq!(
                r.e.mem.u32(option + 0x14),
                expected,
                "code {code} value {value}"
            );
            assert_eq!(r.e.get(menu, StartMenu::pDirtyOption), Ptr::new(option));
            // The menu tile's brightness is copied to the clicked tile.
            assert_eq!(r.tile(0x9000, TRAIT_BRIGHTNESS), 0.5);
            let cast = r.calls_to(RT_DYNAMIC_CAST)[0].clone();
            assert_eq!(
                cast,
                vec![
                    0x6600,
                    0,
                    TYPE_DESCRIPTOR_OPTION,
                    TYPE_DESCRIPTOR_USER_OPTION,
                    0
                ]
            );
        }
    }

    #[test]
    fn a_click_elsewhere_steps_only_the_plain_and_not_last_enumerations() {
        // Type 0 (an on/off option) steps on any click; type 1 only below
        // the last setting; type 2 only for the arrows.
        for (kind, value, stepped) in [(0, 0, true), (1, 0, true), (1, 2, false), (2, 0, false)] {
            let mut r = rig();
            let menu = click_scene(&mut r);
            let option = user_option(&mut r, value, 3, kind);
            r.with(TILE_PARENT, |_, _| eax(0x9100));
            items(&mut r, &[(menu.addr() + 0x114, 0x9100, 0x6600)]);
            r.stub(RT_DYNAMIC_CAST, option);
            r.e.call(0x007c_e9b0, &args![menu, 9i32, 0x9000u32]);
            let now = r.e.mem.u32(option + 0x14);
            assert_eq!(now != value, stepped, "type {kind} value {value}");
            if stepped {
                assert_eq!(now, (value + 1) % 3);
            }
        }
    }

    // ----- 007cefc0 -----

    /// A menu whose own vtable has a click handler (slot 0xc) double, the
    /// highlighted option tile 0x4040 and real flags.
    fn gamepad_scene(r: &mut Rig) -> Ptr<StartMenu> {
        r.real_flags();
        let menu = r.menu();
        let vtable = r.vtable_with(0xc, 0);
        r.e.mem.set_u32(menu.addr(), vtable);
        r.list_vtable(menu, 0x114, 0x4040);
        r.stub(IS_TOP_MENU_ID, 1);
        menu
    }

    fn click_handler(vtable_of: Ptr<StartMenu>, r: &Rig) -> u32 {
        0x0eee_0000 + r.e.mem.u32(vtable_of.addr()) + 0xc
    }

    #[test]
    fn gamepad_ignores_everything_unless_the_start_menu_is_on_top() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        r.stub(IS_TOP_MENU_ID, 0);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 3i32, 0u32]).bool());
        assert_eq!(r.calls_to(IS_TOP_MENU_ID), vec![vec![0x3f5]]);
        assert_eq!(r.order().len(), 1);
    }

    #[test]
    fn gamepad_swallows_buttons_while_busy() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x400);
        assert!(r.e.call(0x007c_efc0, &args![menu, 3i32, 0u32]).bool());
        r.e.set(menu, StartMenu::uiFlags, 0x8000 | 8);
        assert!(r.e.call(0x007c_efc0, &args![menu, 3i32, 0u32]).bool());
        // 0x8000 alone does not.
        r.e.set(menu, StartMenu::uiFlags, 0x8000);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 99i32, 0u32]).bool());
    }

    #[test]
    fn gamepad_buttons_go_to_the_loading_menu_first() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x8_0000);
        r.e.set_global(LOADING_MENU, 0x9300u32);
        r.stub(LOADING_MENU_HANDLER, 1);
        assert!(r.e.call(0x007c_efc0, &args![menu, 1i32, 0u32]).bool());
        r.stub(LOADING_MENU_HANDLER, 0);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 1i32, 0u32]).bool());
        assert_eq!(r.calls_to(LOADING_MENU_HANDLER)[0], vec![0x9300]);
        // A button of 0 or less skips it.
        r.stub(LOADING_MENU_HANDLER, 1);
        r.clear_calls();
        assert!(!r.e.call(0x007c_efc0, &args![menu, 0i32, 0u32]).bool());
        assert!(r.calls_to(LOADING_MENU_HANDLER).is_empty());
    }

    /// The remap mode: device 3, waiting for a button, the highlighted
    /// option's id 0x66.
    fn remap_scene(r: &mut Rig) -> Ptr<StartMenu> {
        let menu = gamepad_scene(r);
        r.e.mem.set_u32(menu.addr() + 0x1c8, 3);
        r.e.set(menu, StartMenu::uiFlags, 0x1000);
        r.stub(LIST_BOX_IS_SELECTED, 1);
        let item = r.alloc(0x10);
        r.e.mem.set_u32(item + 0xc, 0x66);
        r.stub(LIST_BOX_HIGHLIGHTED_ITEM, item);
        r.stub(GAMEPAD_BUTTON_TO_KEY, 0x1ff);
        r.stub(INPUT_OBJECT_GETTER, 0x7e7e);
        r.e.mem.set_u32(menu.addr() + 0x68, 0x4800);
        r.e.mem.set_u32(menu.addr() + 0x3c, 0x4900);
        r.e.mem.set_u32(CONTROL_MODE_LABELS + 12, 0x5100);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        menu
    }

    #[test]
    fn gamepad_binds_the_pressed_button_in_remap_mode() {
        let mut r = rig();
        let menu = remap_scene(&mut r);
        r.stub(BIND_CONTROL, 1);
        assert!(r.e.call(0x007c_efc0, &args![menu, 7i32, 0u32]).bool());
        assert_eq!(r.calls_to(GAMEPAD_BUTTON_TO_KEY), vec![vec![7]]);
        assert_eq!(r.calls_to(BIND_CONTROL), vec![vec![0x7e7e, 0x66, 3, 0xff]]);
        // Without the gamepad the device label is shown.
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x4800, TRAIT_STRING, 0x5100 + 2, 1]]
        );
        let options = menu.addr() + 0x114;
        assert_eq!(r.calls_to(LIST_BOX_SET_ENABLED), vec![vec![options, 1]]);
        assert_eq!(r.calls_to(LIST_BOX_DESELECT_ALL), vec![vec![options, 1]]);
        assert_eq!(
            r.calls_to(LIST_BOX_FOR_EACH),
            vec![vec![options, FN_007CFE40, 0, 0x7fff_ffff]]
        );
        let flags = r.e.get(menu, StartMenu::uiFlags);
        assert_eq!(flags & 0x400_0002, 0x400_0002);
        assert_eq!(r.tile(0x4900, TRAIT_TARGET), 1.0);
    }

    #[test]
    fn gamepad_hides_the_device_label_when_the_pad_is_active() {
        let mut r = rig();
        let menu = remap_scene(&mut r);
        r.stub(BIND_CONTROL, 1);
        r.stub(GAMEPAD_ACTIVE, 1);
        assert!(r.e.call(0x007c_efc0, &args![menu, 7i32, 0u32]).bool());
        assert!(r.calls_to(TILE_SET_STRING).is_empty());
        assert_eq!(r.tile(0x4800, TRAIT_VISIBLE), 0.0);
    }

    #[test]
    fn gamepad_warns_when_the_button_cannot_be_bound() {
        let mut r = rig();
        let menu = remap_scene(&mut r);
        r.stub(BIND_CONTROL, 0);
        r.with(OPTION_TEXT, |_, a| eax(a[0] + 1));
        assert!(r.e.call(0x007c_efc0, &args![menu, 7i32, 0u32]).bool());
        assert_eq!(r.calls_to(PLAY_MENU_SOUND), vec![vec![0xa]]);
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS), vec![vec![0x3e9]]);
        assert_eq!(
            r.calls_to(SHOW_MESSAGE),
            vec![vec![
                SETTING_BINDING_TITLE + 2,
                0,
                0,
                0,
                1,
                0x12,
                0,
                0,
                SETTING_BINDING_TEXT + 1,
                0
            ]]
        );
        assert!(r.calls_to(LIST_BOX_FOR_EACH).is_empty());
        // The message is not repeated while one is open.
        r.clear_calls();
        r.stub(TILE_GET_MENU_BY_CLASS, 0x6000);
        assert!(r.e.call(0x007c_efc0, &args![menu, 7i32, 0u32]).bool());
        assert!(r.calls_to(SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn gamepad_left_and_right_click_the_arrow_tiles() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        let handler = click_handler(menu, &r);
        r.stub(LIST_BOX_IS_ENABLED, 1);
        r.with(TILE_GET_CHILD_BY_ID, |_, a| eax(0x4100 + a[1]));
        r.stub(TILE_IS_VISIBLE, 1);
        r.stub(TILE_IS_TRUE, 1);
        // Button 4 clicks id 100, button 3 id 101.
        assert!(r.e.call(0x007c_efc0, &args![menu, 4i32, 0u32]).bool());
        assert_eq!(
            r.calls_to(handler),
            vec![vec![menu.addr(), 100, 0x4100 + 100]]
        );
        assert_eq!(
            r.calls_to(TILE_PLAY_TILE_SOUND),
            vec![vec![0x4100 + 100, SOUND_ID_CLICK]]
        );
        assert_eq!(r.calls_to(TILE_GET_CHILD_BY_ID)[0], vec![0x4040, 100]);
        assert!(r.e.call(0x007c_efc0, &args![menu, 3i32, 0u32]).bool());
        assert_eq!(r.calls_to(handler)[1][1], 101);
        // The click comes before the sound.
        let order = r.order();
        let click = order.iter().position(|a| *a == handler).unwrap();
        let sound = order
            .iter()
            .position(|a| *a == TILE_PLAY_TILE_SOUND)
            .unwrap();
        assert!(click < sound);
    }

    #[test]
    fn gamepad_right_on_the_back_option_clicks_it_with_minus_one() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        let handler = click_handler(menu, &r);
        r.stub(LIST_BOX_IS_ENABLED, 1);
        // No right-arrow tile, but the highlighted option is `007d0c50`.
        let option = r.alloc(0x10);
        r.e.mem.set_u32(option + 8, CHOOSE_007D0C50);
        r.stub(LIST_BOX_HIGHLIGHTED_ITEM, option);
        assert!(r.e.call(0x007c_efc0, &args![menu, 3i32, 0u32]).bool());
        assert_eq!(
            r.calls_to(handler),
            vec![vec![menu.addr(), 0xffff_ffff, 0x4040]]
        );
        // Left (id 100) does not.
        r.clear_calls();
        assert!(!r.e.call(0x007c_efc0, &args![menu, 4i32, 0u32]).bool());
        assert!(r.calls_to(handler).is_empty());
        // A disabled list or no highlight does nothing.
        r.stub(LIST_BOX_IS_ENABLED, 0);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 3i32, 0u32]).bool());
    }

    #[test]
    fn gamepad_buttons_5_and_9_back_out_of_a_visible_menu() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        r.e.mem.set_u32(menu.addr() + 0x4c, 0x4c00);
        r.stub(TILE_IS_TRUE, 1);
        r.set_tile(0x4c00, TRAIT_ALPHA, 255.0);
        let lazy = r.alloc(8);
        r.e.mem.set_u32(lazy, 0x11);
        r.e.mem.set_u32(lazy + 4, 0x22);
        r.e.set_global(LAZY_OBJECT, lazy);
        assert!(r.e.call(0x007c_efc0, &args![menu, 5i32, 0u32]).bool());
        assert_eq!(r.calls_to(MENU_REFRESH_C), vec![vec![menu.addr()]]);
        assert_eq!(r.e.mem.u32(lazy), 0);
        assert_eq!(r.e.mem.u32(lazy + 4), 0);
        assert_eq!(r.calls_to(TILE_IS_TRUE), vec![vec![0x4c00, TRAIT_VISIBLE]]);
        // Fully transparent (alpha 0): not handled.
        r.set_tile(0x4c00, TRAIT_ALPHA, 0.0);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 9i32, 0u32]).bool());
        // Flag 1 set: not handled.
        r.set_tile(0x4c00, TRAIT_ALPHA, 255.0);
        r.e.set(menu, StartMenu::uiFlags, 1);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 9i32, 0u32]).bool());
    }

    #[test]
    fn gamepad_shoulder_buttons_click_the_load_and_downloads_tiles() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        let handler = click_handler(menu, &r);
        r.e.mem.set_u32(menu.addr() + 0x64, 0x6400);
        r.e.mem.set_u32(menu.addr() + 0x38, 0x3800);
        r.stub(TILE_IS_VISIBLE, 1);
        r.stub(TILE_IS_TRUE, 1);
        assert!(r.e.call(0x007c_efc0, &args![menu, 0xbi32, 0u32]).bool());
        assert_eq!(r.calls_to(handler), vec![vec![menu.addr(), 0xf, 0x6400]]);
        // Sound first, then the click.
        let order = r.order();
        let sound = order
            .iter()
            .position(|a| *a == TILE_PLAY_TILE_SOUND)
            .unwrap();
        let click = order.iter().position(|a| *a == handler).unwrap();
        assert!(sound < click);
        assert!(r.e.call(0x007c_efc0, &args![menu, 0xci32, 0u32]).bool());
        assert_eq!(r.calls_to(handler)[1], vec![menu.addr(), 4, 0x3800]);
        // A hidden tile is not clicked.
        r.stub(TILE_IS_VISIBLE, 0);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 0xbi32, 0u32]).bool());
    }

    #[test]
    fn gamepad_triggers_move_a_slider_option() {
        for (button, start, step, expected, id) in [
            (0x10, 3u32, 2, 5u32, 0x65u32),
            (0xf, 3, 2, 1, 100),
            (0xf, 1, 2, 0, 100),
            (0x10, 9, 2, 9, 0x65),
        ] {
            let mut r = rig();
            let menu = gamepad_scene(&mut r);
            r.stub(LIST_BOX_IS_ENABLED, 1);
            let option = user_option(&mut r, start, 10, 2);
            r.stub(RT_DYNAMIC_CAST, option);
            r.set_tile(0x4040, TRAIT_USER_1, 4.0);
            r.stub_float(SLIDER_STEP, 2.0);
            r.stub(FLOAT_TO_INT, step);
            r.with(TILE_GET_CHILD_BY_ID, |_, a| eax(0x4100 + a[1]));
            assert!(r.e.call(0x007c_efc0, &args![menu, button, 0u32]).bool());
            assert_eq!(
                r.e.mem.u32(option + 0x14),
                expected,
                "button {button:#x} from {start}"
            );
            assert_eq!(r.e.get(menu, StartMenu::pDirtyOption), Ptr::new(option));
            assert_eq!(
                r.calls_to(TILE_PLAY_TILE_SOUND),
                vec![vec![0x4100 + id, SOUND_ID_CLICK]]
            );
            // The step is the slider value scaled by 1.0.
            assert_eq!(
                r.calls_to(SLIDER_STEP),
                vec![vec![4.0f32.to_bits(), 1.0f32.to_bits()]]
            );
        }
    }

    #[test]
    fn gamepad_triggers_ignore_other_option_types() {
        let mut r = rig();
        let menu = gamepad_scene(&mut r);
        r.stub(LIST_BOX_IS_ENABLED, 1);
        let option = user_option(&mut r, 3, 10, 1);
        r.stub(RT_DYNAMIC_CAST, option);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 0x10i32, 0u32]).bool());
        assert_eq!(r.e.mem.u32(option + 0x14), 3);
        r.stub(RT_DYNAMIC_CAST, 0);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 0x10i32, 0u32]).bool());
        r.stub(LIST_BOX_IS_ENABLED, 0);
        assert!(!r.e.call(0x007c_efc0, &args![menu, 0x10i32, 0u32]).bool());
        // Unknown buttons and non-positive ones are not handled.
        assert!(!r.e.call(0x007c_efc0, &args![menu, 6i32, 0u32]).bool());
        assert!(!r.e.call(0x007c_efc0, &args![menu, -3i32, 0u32]).bool());
    }

    // ----- 007cf5c0, 007cfd00, 007cfd20, 007cfd60, 007cfdc0 -----

    #[test]
    fn the_two_word_clear_and_the_byte_getter() {
        let mut r = rig();
        let object = r.alloc(0x60);
        r.e.mem.set_u32(object, 0x11);
        r.e.mem.set_u32(object + 4, 0x22);
        r.e.mem.set_u8(object + 0x49, 0x7f);
        r.e.call(0x007c_f5c0, &args![Ptr::<()>::new(object)]);
        assert_eq!(r.e.mem.u32(object), 0);
        assert_eq!(r.e.mem.u32(object + 4), 0);
        assert_eq!(
            r.e.call(0x007c_fd00, &args![Ptr::<()>::new(object)]).u8(),
            0x7f
        );
    }

    #[test]
    fn update_can_load_status_enables_the_continue_and_load_tiles() {
        let mut r = rig();
        let menu = r.menu();
        r.e.set(menu, StartMenu::ENABLED_TRAIT, 0x5a5a);
        r.e.mem.set_u32(menu.addr() + 0x1c4, 0x9400);
        r.e.mem.set_u32(menu.addr() + 0x1c0, 0x9500);
        r.e.call(0x007c_fd20, &args![menu]);
        assert_eq!(
            r.calls_to(TILE_SET_INT),
            vec![vec![0x9400, 0x5a5a, 1], vec![0x9500, 0x5a5a, 1]]
        );
    }

    #[test]
    fn do_key_asks_the_loading_menu_only_in_its_mode() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.stub(LOADING_MENU_HANDLER, 1);
        // No flag: false without asking.
        assert!(!r.e.call(0x007c_fd60, &args![menu, 13u32]).bool());
        assert!(r.calls_to(LOADING_MENU_HANDLER).is_empty());
        r.e.set(menu, StartMenu::uiFlags, 0x8_0000);
        // Flag but no loading menu.
        assert!(!r.e.call(0x007c_fd60, &args![menu, 13u32]).bool());
        r.e.set_global(LOADING_MENU, 0x9300u32);
        assert!(r.e.call(0x007c_fd60, &args![menu, 13u32]).bool());
        r.stub(LOADING_MENU_HANDLER, 0);
        assert!(!r.e.call(0x007c_fd60, &args![menu, 13u32]).bool());
        assert_eq!(r.calls_to(LOADING_MENU_HANDLER)[0], vec![0x9300]);
    }

    #[test]
    fn reset_to_defaults_restores_a_changed_user_option() {
        let mut r = rig();
        let menu = r.menu();
        r.real_flags();
        let option = user_option(&mut r, 2, 5, 1);
        r.e.mem.set_u32(option + 0x18, 4);
        r.e.mem.set_u32(option + 0x2c, 0x0eee_7000);
        r.stub(0x0eee_7000, 0);
        r.stub(RT_DYNAMIC_CAST, option);
        r.e.call(0x007c_fdc0, &args![0x1234u32, 0x6600u32]);
        assert_eq!(r.e.mem.u32(option + 0x14), 4);
        assert_eq!(
            r.calls_to(DISPLAY_OPTION),
            vec![vec![menu.addr(), option, 0x1234]]
        );
        assert_eq!(r.calls_to(0x0eee_7000), vec![vec![option]]);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags) & 2, 2);
        let cast = r.calls_to(RT_DYNAMIC_CAST)[0].clone();
        assert_eq!(
            cast,
            vec![
                0x6600,
                0,
                TYPE_DESCRIPTOR_OPTION,
                TYPE_DESCRIPTOR_USER_OPTION,
                0
            ]
        );
    }

    #[test]
    fn reset_to_defaults_leaves_options_that_are_not_changed() {
        let mut r = rig();
        let _ = r.menu();
        r.real_flags();
        // Not a user option.
        r.stub(RT_DYNAMIC_CAST, 0);
        r.e.call(0x007c_fdc0, &args![0u32, 0x6600u32]);
        assert!(r.calls_to(DISPLAY_OPTION).is_empty());
        // Already at its default.
        let option = user_option(&mut r, 3, 5, 1);
        r.e.mem.set_u32(option + 0x18, 3);
        r.stub(RT_DYNAMIC_CAST, option);
        r.e.call(0x007c_fdc0, &args![0u32, 0x6600u32]);
        assert!(r.calls_to(DISPLAY_OPTION).is_empty());
        // Changed, but with no change function: displayed, flag 2 not set.
        r.e.mem.set_u32(option + 0x18, 1);
        r.e.call(0x007c_fdc0, &args![0u32, 0x6600u32]);
        assert_eq!(r.calls_to(DISPLAY_OPTION).len(), 1);
        assert!(r.calls_to(MENU_FLAGS_SET).is_empty());
    }

    // ----- 007cf5e0, 007cf670, 007cf6a0 -----

    #[test]
    fn gamepad_up_event_for_button_10_goes_back_after_the_grace_period() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 0x3c, 0x3c00);
        r.stub(TICK_COUNT, 2000);
        r.e.set_global(TICK_CREATED, 1000u32);
        // Other buttons are ignored.
        assert!(!r.e.call(0x007c_f5e0, &args![menu, 9i32, 0u32]).bool());
        assert!(r.order().is_empty());
        // Target tile not true: not handled.
        assert!(!r.e.call(0x007c_f5e0, &args![menu, 10i32, 0u32]).bool());
        r.stub(TILE_IS_TRUE, 1);
        assert!(r.e.call(0x007c_f5e0, &args![menu, 10i32, 0u32]).bool());
        assert_eq!(r.calls_to(CLEAR_MENU_BUTTON), vec![vec![10]]);
        assert_eq!(r.calls_to(CHOOSE_BACK).len(), 1);
        assert_eq!(r.calls_to(TILE_IS_TRUE)[0], vec![0x3c00, TRAIT_TARGET]);
        // Within 500 ticks of the last: no.
        r.e.set_global(TICK_CREATED, 1700u32);
        assert!(!r.e.call(0x007c_f5e0, &args![menu, 10i32, 0u32]).bool());
        assert_eq!(r.calls_to(CHOOSE_BACK).len(), 1);
    }

    #[test]
    fn gamepad_up_event_clears_the_block_flag_first() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.set(menu, StartMenu::uiFlags, 0x400_0000);
        assert!(r.e.call(0x007c_f5e0, &args![menu, 10i32, 0u32]).bool());
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0);
        assert!(r.calls_to(TILE_IS_TRUE).is_empty());
    }

    #[test]
    fn down_click_darkens_the_arrow_tiles_only() {
        let mut r = rig();
        let menu = r.menu();
        r.e.call(0x007c_f670, &args![menu, 100i32, 0x9000u32]);
        assert_eq!(r.tile(0x9000, TRAIT_BRIGHTNESS), -1.0);
        r.e.call(0x007c_f670, &args![menu, 101i32, 0x9100u32]);
        assert_eq!(r.tile(0x9100, TRAIT_BRIGHTNESS), -1.0);
        r.e.call(0x007c_f670, &args![menu, 99i32, 0x9200u32]);
        r.e.call(0x007c_f670, &args![menu, 100i32, 0u32]);
        assert_eq!(r.calls_to(TILE_SET_INT).len(), 2);
    }

    /// The slider drag: option value 1 of 5, slider scale 2, manager width
    /// 100, cursor x 20, tile width 40 -> position (100 - 20) * 2 / 40 = 4.
    fn drag_scene(r: &mut Rig, rounded_to: u32) -> (Ptr<StartMenu>, u32) {
        let menu = r.menu();
        r.list_vtable(menu, 0x114, 0x4040);
        let option = user_option(r, 1, 5, 2);
        r.stub(LIST_BOX_HIGHLIGHTED_ITEM, 0x6600);
        r.stub(RT_DYNAMIC_CAST, option);
        r.with(TILE_PARENT, |_, _| eax(0x4040));
        r.set_tile(0x4040, TRAIT_USER_3, 2.0);
        r.stub(INTERFACE_MANAGER_GETTER, 0xaa00);
        r.stub(MANAGER_WIDTH, 100);
        r.stub_float(TILE_POSITION_A, 20.0);
        r.set_tile(0x9000, TRAIT_WIDTH, 40.0);
        r.stub_float(ROUND_FLOAT, 4.0);
        r.stub(FLOAT_TO_INT, rounded_to);
        (menu, option)
    }

    #[test]
    fn still_down_on_the_slider_sets_the_value_from_the_cursor() {
        let mut r = rig();
        let (menu, option) = drag_scene(&mut r, 4);
        r.e.call(0x007c_f6a0, &args![menu, 0x66i32, 0x9000u32]);
        assert_eq!(r.e.mem.u32(option + 0x14), 4);
        assert_eq!(r.e.get(menu, StartMenu::pDirtyOption), Ptr::new(option));
        assert_eq!(r.calls_to(ROUND_FLOAT), vec![vec![4.0f32.to_bits()]]);
        assert_eq!(r.calls_to(MANAGER_WIDTH), vec![vec![0xaa00]]);
        assert_eq!(r.calls_to(TILE_GET_VALUE).len(), 2);
    }

    #[test]
    fn still_down_clamps_to_the_option_range_and_skips_unchanged_values() {
        for (rounded, expected) in [(9u32, 4u32), (0xffff_fffe, 0), (1, 1)] {
            let mut r = rig();
            let (menu, option) = drag_scene(&mut r, rounded);
            r.e.call(0x007c_f6a0, &args![menu, 0x66i32, 0x9000u32]);
            assert_eq!(r.e.mem.u32(option + 0x14), expected, "rounded {rounded:#x}");
            // The value 1 was already there: the option is not marked.
            let dirty = r.e.get(menu, StartMenu::pDirtyOption);
            assert_eq!(dirty.is_null(), expected == 1);
        }
    }

    #[test]
    fn still_down_ignores_other_tiles_and_codes() {
        let mut r = rig();
        let (menu, option) = drag_scene(&mut r, 4);
        r.e.call(0x007c_f6a0, &args![menu, 0x65i32, 0x9000u32]);
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
        // A slider of another option: its parent is not the highlight.
        r.with(TILE_PARENT, |_, _| eax(0x1111));
        r.e.call(0x007c_f6a0, &args![menu, 0x66i32, 0x9000u32]);
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
        // Not a user option.
        r.with(TILE_PARENT, |_, _| eax(0x4040));
        r.stub(RT_DYNAMIC_CAST, 0);
        r.e.call(0x007c_f6a0, &args![menu, 0x66i32, 0x9000u32]);
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
    }

    // ----- 007cf7e0, 007cf8e0 -----

    #[test]
    fn do_enter_highlights_the_list_item_of_the_tiles_parent() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        let targets = r.list_vtable(menu, 0x114, 0x4040);
        r.with(TILE_PARENT, |_, _| eax(0x9100));
        let options = menu.addr() + 0x114;
        r.with(LIST_BOX_HAS_TILE, move |_, a| {
            eax(u32::from(a[0] == options))
        });
        r.e.call(0x007c_f7e0, &args![menu, 0u32, 0x9000u32]);
        assert_eq!(r.calls_to(targets[0]), vec![vec![options, 0x9100]]);
        // The tile is not in the save-load list: no info refresh.
        assert!(r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO).is_empty());
    }

    #[test]
    fn do_enter_refreshes_the_save_info_for_a_save_tile() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.list_vtable(menu, 0x114, 0x4040);
        let saves = menu.addr() + 0x174;
        r.with(LIST_BOX_HAS_TILE, move |_, a| eax(u32::from(a[0] == saves)));
        r.e.mem.set_u32(menu.addr() + 0x40, 0x4500);
        // Keyboard: the refresh happens when the tile's row is within the
        // manager's row count.
        r.stub(MANAGER_ROW_COUNT, 10);
        r.stub_float(TILE_POSITION_B, 10.0);
        r.e.call(0x007c_f7e0, &args![menu, 0u32, 0x9000u32]);
        assert_eq!(r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO).len(), 1);
        assert_eq!(r.calls_to(TILE_POSITION_B), vec![vec![0x4500]]);
        r.stub_float(TILE_POSITION_B, 10.5);
        r.e.call(0x007c_f7e0, &args![menu, 0u32, 0x9000u32]);
        assert_eq!(r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO).len(), 1);
        r.stub_float(TILE_POSITION_B, f32::NAN);
        r.e.call(0x007c_f7e0, &args![menu, 0u32, 0x9000u32]);
        assert_eq!(r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO).len(), 1);
        // The gamepad always refreshes.
        r.stub(GAMEPAD_ACTIVE, 1);
        r.e.call(0x007c_f7e0, &args![menu, 0u32, 0x9000u32]);
        assert_eq!(r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO).len(), 2);
    }

    #[test]
    fn do_enter_shows_the_brightness_tile_for_the_brightness_option() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.list_vtable(menu, 0x114, 0x4040);
        r.stub(LIST_BOX_IS_ENABLED, 1);
        r.stub(LIST_BOX_HIGHLIGHTED_ITEM, 0x7700);
        r.e.set(menu, StartMenu::pBrightnessOption, Ptr::new(0x7700));
        r.e.mem.set_u32(menu.addr() + 0x6c, 0x6c00);
        r.e.call(0x007c_f7e0, &args![menu, 0u32, 0u32]);
        assert_eq!(r.tile(0x6c00, TRAIT_VISIBLE), 1.0);
        // A null tile skips the parent lookup entirely.
        assert!(r.calls_to(TILE_PARENT).is_empty());
        // Another highlighted option: nothing.
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.stub(LIST_BOX_IS_ENABLED, 1);
        r.stub(LIST_BOX_HIGHLIGHTED_ITEM, 0x7701);
        r.e.set(menu, StartMenu::pBrightnessOption, Ptr::new(0x7700));
        r.e.call(0x007c_f7e0, &args![menu, 0u32, 0u32]);
        assert!(r.calls_to(TILE_SET_INT).is_empty());
    }

    #[test]
    fn do_leave_copies_the_brightness_and_hides_the_brightness_tile() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 4, 0x4400);
        r.e.mem.set_u32(menu.addr() + 0x6c, 0x6c00);
        r.set_tile(0x4400, TRAIT_BRIGHTNESS, 0.25);
        let saves = menu.addr() + 0x174;
        r.with(LIST_BOX_HAS_TILE, move |_, a| eax(u32::from(a[0] == saves)));
        // An arrow tile: brightness copied, save info refreshed (keyboard),
        // the brightness tile kept.
        r.e.call(0x007c_f8e0, &args![menu, 100i32, 0x9000u32]);
        assert_eq!(r.tile(0x9000, TRAIT_BRIGHTNESS), 0.25);
        assert_eq!(r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO).len(), 1);
        assert!(r.calls_to(TILE_SET_INT).is_empty());
        // Any other tile: hide the brightness tile.
        r.e.call(0x007c_f8e0, &args![menu, 7i32, 0x9100u32]);
        assert_eq!(r.tile(0x6c00, TRAIT_VISIBLE), 0.0);
        assert!(r.tile(0x9100, TRAIT_BRIGHTNESS).is_nan());
        // With the gamepad the save info is not refreshed.
        r.stub(GAMEPAD_ACTIVE, 1);
        r.e.call(0x007c_f8e0, &args![menu, 7i32, 0x9100u32]);
        assert_eq!(r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO).len(), 2);
    }

    #[test]
    fn do_leave_without_a_tile_or_a_brightness_tile_is_quiet() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.call(0x007c_f8e0, &args![menu, 101i32, 0u32]);
        assert!(r.calls_to(TILE_SET_VALUE).is_empty());
        assert!(r.calls_to(TILE_SET_INT).is_empty());
    }

    // ----- 007cf9a0 -----

    /// An idle scene: the gate object's byte set, all flags real.
    fn idle_scene(r: &mut Rig) -> Ptr<StartMenu> {
        r.real_flags();
        let menu = r.menu();
        let gate = r.alloc(0x60);
        r.e.mem.set_u8(gate + 0x49, 1);
        r.e.set_global(IDLE_GATE_OBJECT, gate);
        r.e.mem.set_u32(menu.addr() + 0x34, 0x3400);
        menu
    }

    #[test]
    fn idle_does_nothing_while_the_gate_byte_is_clear() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        let gate = r.e.global::<u32>(IDLE_GATE_OBJECT);
        r.e.mem.set_u8(gate + 0x49, 0);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert!(r.order().is_empty());
    }

    #[test]
    fn idle_ticks_and_frees_a_finished_child_menu() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        let vtable = r.vtable_with(0x2c, 0);
        let free = r.alloc(0x10);
        r.e.mem.set_u32(vtable, free);
        r.stub(free, 0);
        let child = r.object(vtable);
        r.e.set_global(CHILD_MENU, child);
        r.e.call(0x007c_f9a0, &args![menu]);
        // Not finished: ticked only.
        assert_eq!(r.calls_to(0x0eee_0000 + vtable + 0x2c), vec![vec![child]]);
        assert_eq!(r.e.global::<u32>(CHILD_MENU), child);
        r.stub(CHILD_MENU_DONE, 1);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.calls_to(free), vec![vec![child, 1]]);
        assert_eq!(r.e.global::<u32>(CHILD_MENU), 0);
    }

    #[test]
    fn idle_updates_the_loading_menu_and_starts_the_load_screen_list() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set_global(LOADING_MENU, 0x9300u32);
        r.e.set(menu, StartMenu::uiFlags, 0x8_0000);
        r.stub(LOADING_MENU_CHECK, 1);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.calls_to(LOADING_MENU_UPDATE), vec![vec![0x9300]]);
        assert_eq!(r.calls_to(LOADING_MENU_CHECK), vec![vec![0x9300, 0]]);
        assert_eq!(r.calls_to(MAKE_LOAD_SCREEN_LIST), vec![vec![0x9300]]);
        // 0x80000 replaced by 0x10.
        assert_eq!(r.e.get(menu, StartMenu::uiFlags) & 0x8_0010, 0x10);
    }

    #[test]
    fn idle_stops_when_the_step_functions_say_so() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.stub(IDLE_STEP_B, 1);
        r.e.set(menu, StartMenu::uiFlags, 0x1_0000);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.calls_to(IDLE_STEP_A), vec![vec![menu.addr()]]);
        assert!(r.calls_to(TILE_GET_FADE_FINISHED).is_empty());
    }

    #[test]
    fn idle_with_the_credits_flag_starts_the_credits_once_faded() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x1_0000);
        r.stub(TILE_GET_FADE_FINISHED, 1);
        r.set_tile(0x3400, TRAIT_ALPHA, 0.0);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(
            r.calls_to(TILE_GET_FADE_FINISHED),
            vec![vec![0x3400, TRAIT_ALPHA]]
        );
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS), vec![vec![0x417]]);
        assert_eq!(r.calls_to(RESUME_BACKGROUND_THREAD).len(), 1);
        assert_eq!(r.calls_to(CREDITS_MENU_CREATE).len(), 1);
        // The save-load idle does not run in this mode.
        assert!(r.calls_to(SAVE_LOAD_IDLE).is_empty());
        // Not yet faded, still visible, or already open: no credits.
        for (finished, alpha, open) in [(0, 0.0, 0), (1, 0.5, 0), (1, 0.0, 0x77)] {
            let mut r = rig();
            let menu = idle_scene(&mut r);
            r.e.set(menu, StartMenu::uiFlags, 0x1_0000);
            r.stub(TILE_GET_FADE_FINISHED, finished);
            r.set_tile(0x3400, TRAIT_ALPHA, alpha);
            r.stub(TILE_GET_MENU_BY_CLASS, open);
            r.e.call(0x007c_f9a0, &args![menu]);
            assert!(r.calls_to(CREDITS_MENU_CREATE).is_empty());
        }
    }

    #[test]
    fn idle_closes_the_message_after_its_deadline() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x8000 | 2);
        r.e.set_global(FADE_DEADLINE, 5000u32);
        r.stub(TICK_COUNT, 4999);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert!(r.calls_to(CLOSE_MESSAGE_MENU).is_empty());
        r.stub(TICK_COUNT, 5000);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.calls_to(CLOSE_MESSAGE_MENU).len(), 1);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0);
    }

    #[test]
    fn idle_moves_from_the_binding_message_to_its_deadline() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x4000 | 2);
        r.stub(TILE_GET_MENU_BY_CLASS, 0x3e90);
        r.stub(TILE_GET_MENU, 0x3e91);
        r.stub(MENU_FADE_STATE, 1);
        r.stub(TICK_COUNT, 1000);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS)[0], vec![0x3e9]);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0x8000 | 2);
        assert_eq!(r.e.global::<u32>(FADE_DEADLINE), 4000);
        assert_eq!(r.calls_to(MENU_REFRESH_D).len(), 1);
    }

    #[test]
    fn idle_abandons_the_binding_message_when_no_message_menu_exists() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x4000 | 2);
        r.stub(MESSAGE_MENU_ACTIVE, 0);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.calls_to(MENU_REFRESH_D).len(), 1);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0);
        // A message menu is still up: left alone.
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x4000 | 2);
        r.stub(MESSAGE_MENU_ACTIVE, 1);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert!(r.calls_to(MENU_REFRESH_D).is_empty());
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0x4000 | 2);
    }

    #[test]
    fn idle_skips_the_message_handling_after_the_save_load_idle_handled_it() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x4000);
        r.stub(SAVE_LOAD_IDLE, 1);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert!(r.calls_to(TILE_GET_MENU_BY_CLASS).is_empty());
        assert!(r.calls_to(TICK_COUNT).is_empty());
    }

    #[test]
    fn idle_applies_a_dirty_option() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        let option = user_option(&mut r, 1, 3, 1);
        r.e.mem.set_u32(option + 0x2c, 0x0eee_7000);
        r.stub(0x0eee_7000, 0);
        r.e.set(menu, StartMenu::pDirtyOption, Ptr::new(option));
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(
            r.calls_to(DISPLAY_OPTION),
            vec![vec![menu.addr(), option, 0]]
        );
        assert_eq!(r.calls_to(0x0eee_7000), vec![vec![option]]);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags) & 2, 2);
        assert!(r.e.get(menu, StartMenu::pDirtyOption).is_null());
        // Without a change function the flag stays clear.
        let mut r = rig();
        let menu = idle_scene(&mut r);
        let option = user_option(&mut r, 1, 3, 1);
        r.e.set(menu, StartMenu::pDirtyOption, Ptr::new(option));
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0);
        assert!(r.e.get(menu, StartMenu::pDirtyOption).is_null());
    }

    #[test]
    fn idle_opens_the_marketplace_once_the_message_menu_has_faded_in() {
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x80_0000);
        r.stub(TILE_GET_MENU_BY_CLASS, 0x3e90);
        r.stub(TILE_GET_MENU, 0x3e91);
        r.stub(MENU_FADE_STATE, 1);
        r.stub(DLC_INSTANCE, 0xd1);
        r.with(DLC_STEP_B, |_, a| eax(a[0] + 1));
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0);
        assert_eq!(
            r.calls_to(DLC_CALL_WITH_CALLBACK),
            vec![vec![0xd2, 0, CHOOSE_007D4C50]]
        );
        // No message menu: nothing happens and the flag stays.
        let mut r = rig();
        let menu = idle_scene(&mut r);
        r.e.set(menu, StartMenu::uiFlags, 0x80_0000);
        r.e.call(0x007c_f9a0, &args![menu]);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0x80_0000);
        assert!(r.calls_to(DLC_CALL_WITH_CALLBACK).is_empty());
    }

    // ----- 007cfe40 -----

    /// The key-label callback with the device `mode` and the looked-up key.
    /// Returns the menu, the label tile and the item; `STRING_FORMAT` and
    /// `FIRST_WORD` are doubles (the label's characters are 0xc4c4).
    fn label_scene(r: &mut Rig, mode: u32, key: u32) -> (Ptr<StartMenu>, u32, u32) {
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 0x1c8, mode);
        let item = r.alloc(0x10);
        r.e.mem.set_u32(item + 0xc, 0x66);
        r.stub(INPUT_OBJECT_GETTER, 0x7e7e);
        r.stub(KEY_LOOKUP, key);
        r.stub(FIRST_WORD, 0xc4c4);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        (menu, 0x9000, item)
    }

    #[test]
    fn key_label_without_a_start_menu_does_nothing() {
        let mut r = rig();
        r.e.call(0x007c_fe40, &args![0x9000u32, 0x1000u32]);
        assert!(r.order().is_empty());
    }

    #[test]
    fn key_label_of_an_unbound_control_is_empty() {
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 0, 0xff);
        let frame = next_frame(&mut r.e, 0x28);
        r.e.call(0x007c_fe40, &args![tile, item]);
        // The 0x28-byte frame: the text at +0x14.
        assert_eq!(r.calls_to(STRING_SET)[0][0], frame + 0x14);
        // The tile is flagged and receives the (empty) string.
        assert_eq!(r.tile(tile, TRAIT_USER_0), 1.0);
        assert_eq!(r.calls_to(KEY_LOOKUP), vec![vec![0x7e7e, 0x66, 0]]);
        let set = r.calls_to(STRING_SET);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][1..], [TEXT_EMPTY, 0]);
        assert!(r.calls_to(STRING_FORMAT).is_empty());
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![tile, TRAIT_USER_1, 0xc4c4, 1]]
        );
        // The temporary string is created first and destroyed last.
        let text = r.calls_to(STRING_NEW)[0][0];
        assert_eq!(r.calls_to(STRING_DESTROY), vec![vec![text]]);
        assert_eq!(r.order().first(), Some(&STRING_NEW));
        assert_eq!(r.order().last(), Some(&STRING_DESTROY));
    }

    #[test]
    fn key_label_of_a_keyboard_key_is_its_name() {
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 0, 5);
        r.e.mem.set_u32(KEY_NAMES_KEYBOARD + 20, 0x6101);
        r.e.call(0x007c_fe40, &args![tile, item]);
        let text = r.calls_to(STRING_NEW)[0][0];
        assert_eq!(
            r.calls_to(STRING_FORMAT),
            vec![vec![text, FORMAT_STRING, 0x6103]]
        );
        // A key without a name is shown as its number.
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 0, 5);
        r.e.call(0x007c_fe40, &args![tile, item]);
        let text = r.calls_to(STRING_NEW)[0][0];
        assert_eq!(
            r.calls_to(STRING_FORMAT),
            vec![vec![text, FORMAT_NUMBER, 5]]
        );
        // And a key id of 0xee or more has no table entry either.
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 0, 0xee);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert_eq!(r.calls_to(STRING_FORMAT)[0][1..], [FORMAT_NUMBER, 0xee]);
    }

    #[test]
    fn key_label_of_a_mouse_button_is_its_name() {
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 1, 3);
        r.e.mem.set_u32(KEY_NAMES_MOUSE + 12, 0x6201);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert_eq!(r.calls_to(STRING_FORMAT)[0][1..], [FORMAT_STRING, 0x6203]);
        // A button number past the 9 mouse buttons falls back to the number.
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 1, 9);
        r.e.mem.set_u32(KEY_NAMES_MOUSE + 36, 0x6201);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert_eq!(r.calls_to(STRING_FORMAT)[0][1..], [FORMAT_NUMBER, 9]);
    }

    #[test]
    fn key_label_of_the_third_device_is_numbered_or_named() {
        // Key below the control count: "<name> <number>".
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 2, 4);
        r.stub(CONTROL_COUNT, 10);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert_eq!(r.calls_to(CONTROL_COUNT), vec![vec![0x7e7e, 0]]);
        assert_eq!(
            r.calls_to(STRING_FORMAT)[0][1..],
            [FORMAT_STRING_NUMBER, SETTING_KEY_NUMBER + 2, 5]
        );
        // Key 0x21 past the count with a name entry.
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 2, 0x21);
        r.stub(CONTROL_COUNT, 10);
        r.e.mem.set_u32(KEY_NAMES_PAD + 4 * 0x21, 0x6301);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert_eq!(r.calls_to(STRING_FORMAT)[0][1..], [FORMAT_STRING, 0x6303]);
        // Key 12: no name, so the string is cleared.
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 2, 12);
        r.stub(CONTROL_COUNT, 10);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert!(r.calls_to(STRING_FORMAT).is_empty());
        assert_eq!(r.calls_to(STRING_SET).len(), 1);
    }

    #[test]
    fn key_label_of_a_gamepad_button_is_a_glow_button_picture() {
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 3, 8);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert_eq!(
            r.calls_to(STRING_FORMAT)[0][1..],
            [FORMAT_GLOW_BUTTON, 0x0107_6f2c]
        );
        // eUser0 is set to 1 first and 2 for a picture.
        assert_eq!(r.tile(tile, TRAIT_USER_0), 2.0);
        for (key, name) in [
            (7, 0x0107_6f18),
            (14, 0x0106_e79c),
            (16, 0x0107_6f38),
            (17, 0x0107_6f34),
            (10, 0x0102_1568),
        ] {
            let mut r = rig();
            let (_, tile, item) = label_scene(&mut r, 3, key);
            r.e.call(0x007c_fe40, &args![tile, item]);
            assert_eq!(
                r.calls_to(STRING_FORMAT)[0][1..],
                [FORMAT_GLOW_BUTTON, name]
            );
        }
        // Other buttons are shown as numbers and leave eUser0 at 1.
        for key in [6, 18, 40] {
            let mut r = rig();
            let (_, tile, item) = label_scene(&mut r, 3, key);
            r.e.call(0x007c_fe40, &args![tile, item]);
            assert_eq!(r.calls_to(STRING_FORMAT)[0][1..], [FORMAT_NUMBER, key]);
            assert_eq!(r.tile(tile, TRAIT_USER_0), 1.0);
        }
    }

    #[test]
    fn key_label_for_an_unknown_device_is_the_number() {
        let mut r = rig();
        let (_, tile, item) = label_scene(&mut r, 9, 44);
        r.e.call(0x007c_fe40, &args![tile, item]);
        assert_eq!(r.calls_to(STRING_FORMAT)[0][1..], [FORMAT_NUMBER, 44]);
    }

    // ----- 007d01b0, 007d03a0 -----

    #[test]
    fn the_keyboard_control_filter() {
        let mut r = rig();
        let accepted: Vec<u32> = (0..0x28u32)
            .filter(|i| r.e.call(0x007d_03a0, &args![*i]).bool())
            .collect();
        let mut expected: Vec<u32> = (0..=0x10).collect();
        expected.extend([0x12, 0x19, 0x1a, 0x1b]);
        assert_eq!(accepted, expected);
    }

    /// Label calls recorded by the controls-list doubles.
    type LabelLog = Rc<RefCell<Vec<(u32, u32)>>>;

    /// The controls list scene: a real `operator new`, the options list
    /// with a vtable, and the label tables.
    fn controls_scene(r: &mut Rig, mode: u32) -> (Ptr<StartMenu>, LabelLog) {
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 0x1c8, mode);
        r.e.mem.set_u32(menu.addr() + 0x68, 0x6800);
        r.list_vtable(menu, 0x114, 0);
        r.with(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        for index in 0..0x1c {
            r.e.mem.set_u32(OPTION_LABELS + 4 * index, 0x6000 + index);
        }
        r.e.mem.set_u32(CONTROL_MODE_LABELS + 4 * mode, 0x5200);
        let added = Rc::new(RefCell::new(Vec::new()));
        let log = added.clone();
        r.with(LIST_BOX_ADD_ITEM, move |e, a| {
            log.borrow_mut().push((a[1], e.mem.u32(a[1] + 0xc)));
            eax(0x7000 + a[1] % 0x1000)
        });
        (menu, added)
    }

    #[test]
    fn controls_list_for_the_keyboard_has_the_accepted_controls() {
        let mut r = rig();
        let (menu, added) = controls_scene(&mut r, 0);
        r.e.call(0x007d_01b0, &args![menu]);
        let added = added.borrow();
        let indexes: Vec<u32> = added.iter().map(|(_, modes)| *modes).collect();
        let mut expected: Vec<u32> = (0..=0x10).collect();
        expected.extend([0x12, 0x19, 0x1a, 0x1b]);
        assert_eq!(indexes, expected);
        // The list is emptied (RemoveAll, slot +0x1c) and enabled first.
        assert_eq!(
            r.calls_to(0x0eee_2000 + 0x114 * 0x10 + 0x1c),
            vec![vec![menu.addr() + 0x114]]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED),
            vec![vec![menu.addr() + 0x114, 1]]
        );
        // The first option: vtable, text of the control's setting + 2, the
        // choose function and the index.
        let option = added[0].0;
        assert_eq!(r.e.mem.u32(option), VTABLE_OPTION);
        assert_eq!(r.e.mem.u32(option + 4), 0x6000 + 2);
        assert_eq!(r.e.mem.u32(option + 8), CONTROL_CHOOSE);
        assert_eq!(r.e.mem.u32(option + 0xc), 0);
        // AddItem(option, text, 0, template) and the item's label.
        let first = &r.calls_to(LIST_BOX_ADD_ITEM)[0];
        assert_eq!(
            first,
            &vec![menu.addr() + 0x114, option, 0x6002, 0, TEXT_ITEM_TEMPLATE]
        );
        assert_eq!(
            r.calls_to(SET_TILE_LABEL)[0],
            vec![0x7000 + option % 0x1000, 0x6002]
        );
        // The key labels are filled, and the device label shown.
        assert_eq!(
            r.calls_to(LIST_BOX_FOR_EACH),
            vec![vec![menu.addr() + 0x114, FN_007CFE40, 0, 0x7fff_ffff]]
        );
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x6800, TRAIT_STRING, 0x5202, 1]]
        );
        assert_eq!(r.tile(0x6800, TRAIT_VISIBLE), 1.0);
    }

    #[test]
    fn controls_list_for_the_gamepad_has_its_own_eleven_and_no_device_label() {
        let mut r = rig();
        let (menu, added) = controls_scene(&mut r, 3);
        r.stub(GAMEPAD_ACTIVE, 1);
        r.e.call(0x007d_01b0, &args![menu]);
        let mut indexes: Vec<u32> = added.borrow().iter().map(|(_, modes)| *modes).collect();
        indexes.sort_unstable();
        assert_eq!(indexes, vec![4, 5, 6, 7, 8, 0xc, 0xd, 0xe, 0xf, 0x10, 0x1b]);
        assert!(r.calls_to(TILE_SET_STRING).is_empty());
        assert_eq!(r.calls_to(LIST_BOX_FOR_EACH).len(), 1);
    }

    // ----- 007d0440 .. 007d0a10 -----

    #[test]
    fn choose_continue_requests_with_the_xui_and_closes_without_it() {
        let mut r = rig();
        r.real_flags();
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 0x48, 0x4800);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        r.stub(XUI_IS_UP, 1);
        r.e.call(0x007d_0440, &args![]);
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x4800, TRAIT_STRING, SETTING_CONTINUE + 2, 1]]
        );
        assert_eq!(r.calls_to(MENU_REQUEST), vec![vec![menu.addr(), 0x100, 0]]);
        assert!(r.calls_to(MENU_START_FADE_OUT).is_empty());
        // Without the XUI the menu closes (the delete-on-fade argument 1).
        r.e.mem.set_u32(menu.addr() + 4, 0x4400);
        r.stub(XUI_IS_UP, 0);
        r.e.call(0x007d_0440, &args![]);
        assert_eq!(r.tile(0x4400, TRAIT_DELETE_ON_FADE), 1.0);
        assert_eq!(r.calls_to(MENU_START_FADE_OUT), vec![vec![menu.addr()]]);
    }

    #[test]
    fn choose_load_and_back_set_their_label_and_request() {
        let mut r = rig();
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        // No menu: nothing.
        r.e.call(0x007d_0490, &args![]);
        r.e.call(0x007d_0a10, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 0x48, 0x4800);
        r.e.call(0x007d_0490, &args![]);
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x4800, TRAIT_STRING, SETTING_LOAD + 2, 1]]
        );
        assert_eq!(r.calls_to(MENU_REQUEST), vec![vec![menu.addr(), 0x80, 0]]);
        r.clear_calls();
        r.stub(START_MENU_FLAG_1, 1);
        r.e.call(0x007d_0a10, &args![]);
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x4800, TRAIT_STRING, SETTING_BACK + 2, 1]]
        );
        assert_eq!(r.calls_to(MENU_REQUEST), vec![vec![menu.addr(), 0x2000, 3]]);
        r.clear_calls();
        r.stub(START_MENU_FLAG_1, 0);
        r.e.call(0x007d_0a10, &args![]);
        assert_eq!(r.calls_to(MENU_REQUEST), vec![vec![menu.addr(), 0x1000, 1]]);
    }

    #[test]
    fn choose_credits_sets_the_flag_and_fades() {
        let mut r = rig();
        r.real_flags();
        r.e.call(0x007d_04d0, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu();
        r.e.call(0x007d_04d0, &args![]);
        assert_eq!(r.e.get(menu, StartMenu::uiFlags), 0x1_0000);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            r.calls_to(FADE),
            vec![
                vec![menu.addr(), 3, 0, 0xfa9, minus_one],
                vec![menu.addr(), 0, 0, 0x8000_0000, minus_one],
                vec![menu.addr(), 9, 0, 0xfa9, minus_one],
            ]
        );
    }

    #[test]
    fn choose_downloads_lists_the_active_mods() {
        let mut r = rig();
        let menu = r.menu();
        let targets = r.list_vtable(menu, 0x144, 0);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 2));
        r.e.set_global(DOWNLOAD_MODS_OBJECT, 0x6200u32);
        let active = r.alloc(0x500);
        let inactive = r.alloc(0x500);
        let last = r.alloc(0x500);
        let nodes = r.nodes(&[active, 0, inactive, last]);
        r.stub(MOD_LIST_HEAD, nodes);
        r.with(MOD_IS_ACTIVE, move |_, a| eax(u32::from(a[0] != inactive)));
        r.with(FIRST_WORD, |_, a| eax(a[0]));
        r.e.call(0x007d_0550, &args![]);
        let list = menu.addr() + 0x144;
        assert_eq!(r.calls_to(MOD_LIST_HEAD), vec![vec![0x6200]]);
        assert_eq!(r.calls_to(targets[2]), vec![vec![list]]);
        assert_eq!(
            r.calls_to(INT_LIST_ADD_ITEM),
            vec![
                vec![list, 0xffff_ffff, SETTING_DOWNLOADS + 2, 0, 0],
                vec![list, 0, active + 0x418, 0, 0],
                vec![list, 0, last + 0x418, 0, 0],
            ]
        );
        assert_eq!(r.calls_to(INT_LIST_SET_ENABLED), vec![vec![list, 1]]);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            r.calls_to(FADE),
            vec![
                vec![menu.addr(), 0xe, 1, 0x8000_0000, minus_one],
                vec![menu.addr(), 5, 1, 0x8000_0000, minus_one],
            ]
        );
        // No menu: nothing at all.
        let mut r = rig();
        r.e.call(0x007d_0550, &args![]);
        assert!(r.order().is_empty());
    }

    #[test]
    fn the_mod_name_field_is_at_0x418() {
        let mut r = rig();
        assert_eq!(
            r.e.call(0x007d_0660, &args![Ptr::<()>::new(0x1000)]).u32(),
            0x1418
        );
    }

    #[test]
    fn choose_save_and_load_prepare_the_page() {
        for (address, load) in [(0x007d_0680u32, 0u32), (0x007d_06c0, 1)] {
            let mut r = rig();
            r.real_flags();
            r.e.call(address, &args![]);
            assert!(r.order().is_empty());
            let menu = r.menu();
            r.e.mem.set_u32(menu.addr() + 0x64, 0x6400);
            r.e.set(menu, StartMenu::uiFlags, 8 * (1 - load));
            r.e.call(address, &args![]);
            assert_eq!(r.tile(0x6400, TRAIT_USER_0), 0.0);
            assert_eq!(r.e.get(menu, StartMenu::uiFlags), 8 * load);
            assert_eq!(r.calls_to(PREP_SAVE_LOAD), vec![vec![menu.addr()]]);
        }
    }

    #[test]
    fn choose_settings_opens_the_settings_list() {
        let mut r = rig();
        r.e.call(0x007d_0700, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu();
        r.e.call(0x007d_0700, &args![]);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            r.order(),
            vec![LIST_BOX_SET_ENABLED, FADE, LIST_BOX_DESELECT_ALL, FADE]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED),
            vec![vec![menu.addr() + 0xb4, 1]]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_DESELECT_ALL),
            vec![vec![menu.addr() + 0xb4, 1]]
        );
        assert_eq!(
            r.calls_to(FADE),
            vec![
                vec![menu.addr(), 1, 1, 0x8000_0000, minus_one],
                vec![menu.addr(), 5, 1, 0x8000_0000, minus_one],
            ]
        );
    }

    // ----- 007d0770, 007d09c0 -----

    #[test]
    fn the_help_table_search_finds_the_menu_class_or_29() {
        let mut r = rig();
        let table = r.alloc(0x100);
        for i in 0..0x29u32 {
            // Class 0x3e9 + i in bits 2..8, other bits set to show the mask.
            r.e.mem.set_u32(table + 4 * i, (i << 2) | 0xffff_ff03);
        }
        assert_eq!(
            r.e.call(0x007d_09c0, &args![Ptr::<()>::new(table), 0x3e9u32])
                .u32(),
            0
        );
        assert_eq!(
            r.e.call(0x007d_09c0, &args![Ptr::<()>::new(table), 0x3eeu32])
                .u32(),
            5
        );
        assert_eq!(
            r.e.call(0x007d_09c0, &args![Ptr::<()>::new(table), 0x411u32])
                .u32(),
            0x28
        );
        assert_eq!(
            r.e.call(0x007d_09c0, &args![Ptr::<()>::new(table), 0x500u32])
                .u32(),
            0x29
        );
    }

    /// The help scene: manager 0xaa00 with a help table in which class
    /// `0x3e9 + i` has index `i`; form `0x168 + i` is 0xf000 + i, cast to
    /// 0xc000 + i; the companion form (0x163 / 0x165) is 0xf1d, cast to
    /// 0xbeef, whose list is `companion_list`.
    fn help_scene(r: &mut Rig, enter_stack: Vec<u32>, companion_list: &[u32]) -> Ptr<StartMenu> {
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 0x50, 0x5000);
        let manager = r.alloc(0x600);
        for i in 0..0x29u32 {
            r.e.mem.set_u32(manager + 0x4d4 + 4 * i, i << 2);
        }
        r.stub(INTERFACE_MANAGER_GETTER, manager);
        r.stub(START_MENU_CLASS_GETTER, 0x3f5);
        r.with(GET_ENTER_STACK, move |_, a| {
            eax(enter_stack.get(a[1] as usize).copied().unwrap_or(0x3f5))
        });
        r.with(LOOKUP_FORM, |_, a| {
            eax(match a[0] {
                0x163 | 0x165 => 0xf1d,
                id if (0x168..0x168 + 0x29).contains(&id) => 0xf000 + (id - 0x168),
                _ => 0,
            })
        });
        r.with(RT_DYNAMIC_CAST, |_, a| {
            let derived_a = a[3] == TYPE_DESCRIPTOR_FORM_DERIVED_A;
            eax(if derived_a && (0xf000..0xf029).contains(&a[0]) {
                0xc000 + (a[0] - 0xf000)
            } else if !derived_a && a[0] == 0xf1d {
                0xbeef
            } else {
                0
            })
        });
        let list = r.nodes(companion_list);
        r.stub(FORM_LIST_HEAD, list);
        menu
    }

    #[test]
    fn help_opens_the_tutorial_of_the_menu_below_the_start_menu() {
        let mut r = rig();
        let menu = help_scene(&mut r, vec![0x3ee], &[0x1111, 0xc005]);
        r.e.call(0x007d_0770, &args![]);
        assert_eq!(r.tile(0x5000, TRAIT_VISIBLE), 1.0);
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0xc005, 1]]);
        assert_eq!(r.calls_to(LOOKUP_FORM)[0], vec![0x168 + 5]);
        assert_eq!(r.calls_to(LOOKUP_FORM)[1], vec![0x163]);
        assert_eq!(r.calls_to(FORM_LIST_HEAD), vec![vec![0xbeef]]);
        let _ = menu;
    }

    #[test]
    fn help_uses_the_gamepad_companion_form() {
        let mut r = rig();
        let _ = help_scene(&mut r, vec![0x3ee], &[0xc005]);
        r.stub(GAMEPAD_ACTIVE, 1);
        r.e.call(0x007d_0770, &args![]);
        assert_eq!(r.calls_to(LOOKUP_FORM)[1], vec![0x165]);
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0xc005, 1]]);
    }

    #[test]
    fn help_skips_a_tutorial_the_companion_does_not_list() {
        let mut r = rig();
        let _ = help_scene(&mut r, vec![0x3ee, 0x3ef], &[0xc006]);
        r.e.call(0x007d_0770, &args![]);
        // The first entry (index 5) is not listed; the second (index 6) is.
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0xc006, 1]]);
        // None listed at all: the tutorial is created for nothing.
        let mut r = rig();
        let _ = help_scene(&mut r, vec![0x3ee, 0x3ef], &[0x1111]);
        r.e.call(0x007d_0770, &args![]);
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0, 1]]);
    }

    #[test]
    fn help_stops_at_the_start_menu_itself_and_ignores_unknown_menus() {
        let mut r = rig();
        let _ = help_scene(&mut r, vec![0x3f5, 0x3ee], &[0xc005]);
        r.e.call(0x007d_0770, &args![]);
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0, 1]]);
        // A menu class with no table entry (index 0x29): skipped.
        let mut r = rig();
        let _ = help_scene(&mut r, vec![0x500, 0x3ee], &[0xc005]);
        r.e.call(0x007d_0770, &args![]);
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0xc005, 1]]);
    }

    #[test]
    fn help_entry_one_picks_the_first_visible_of_four_menus() {
        let mut r = rig();
        let _ = help_scene(&mut r, vec![1], &[0xc001]);
        // Only 0x3ea is visible: table index 1.
        r.with(IS_MENU_ID_VISIBLE, |_, a| eax(u32::from(a[0] == 0x3ea)));
        r.e.call(0x007d_0770, &args![]);
        assert_eq!(r.calls_to(LOOKUP_FORM)[0], vec![0x168 + 1]);
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0xc001, 1]]);
        // Asked in the order 0x3eb, 0x40b, 0x3ea (stops there).
        let asked: Vec<u32> = r
            .calls_to(IS_MENU_ID_VISIBLE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(asked, vec![0x3eb, 0x40b, 0x3ea]);
        // None visible: index 0x29, nothing found.
        let mut r = rig();
        let _ = help_scene(&mut r, vec![1], &[0xc001]);
        r.e.call(0x007d_0770, &args![]);
        assert!(r.calls_to(LOOKUP_FORM).is_empty());
        assert_eq!(r.calls_to(TUTORIAL_MENU_CREATE), vec![vec![0, 1]]);
    }

    // ----- 007d0a70, 007d0b90, 007d0bb0, 007d0bd0 -----

    #[test]
    fn choose_main_menu_tears_down_the_game_state() {
        let mut r = rig();
        let delete_vtable = r.vtable_with(0, 0);
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr(), delete_vtable);
        r.stub(AUDIO_INSTANCE, 0xa0d10);
        let dlc = r.alloc(0x100);
        r.stub(DLC_GETTER, dlc);
        let main = r.alloc(0x10);
        r.e.set_global(MAIN_OBJECT, main);
        r.e.set_global(TEXTURE_OWNER, 0x7777u32);
        r.e.set_global(GLOBAL_OBJECT_011DEA3C, 0x1234u32);
        r.e.mem.set_u8(dlc + 0x88, 0x55);
        r.e.call(0x007d_0a70, &args![]);
        assert_eq!(
            r.calls_to(0x0eee_0000 + delete_vtable),
            vec![vec![menu.addr(), 1]]
        );
        let order: Vec<u32> = r
            .order()
            .into_iter()
            .filter(|a| {
                [
                    HIDE_MENUS,
                    CLOSE_ALL_SUPPORT_MENUS,
                    KILL_ALL_OF_TYPE,
                    MEDIA_CLEAR,
                    OBJECT_CALL_009698A0,
                    TEXTURE_OWNER_CALL,
                    SET_DIALOG_FLAG,
                    MEDIA_PLAY,
                    MEDIA_FINISH,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                HIDE_MENUS,
                CLOSE_ALL_SUPPORT_MENUS,
                KILL_ALL_OF_TYPE,
                MEDIA_CLEAR,
                OBJECT_CALL_009698A0,
                TEXTURE_OWNER_CALL,
                SET_DIALOG_FLAG,
                MEDIA_PLAY,
                MEDIA_FINISH
            ]
        );
        assert_eq!(r.calls_to(CLOSE_ALL_SUPPORT_MENUS), vec![vec![0]]);
        assert_eq!(
            r.calls_to(KILL_ALL_OF_TYPE),
            vec![vec![0xa0d10, 0xffff_ffff, 0]]
        );
        assert_eq!(r.calls_to(OBJECT_CALL_009698A0), vec![vec![0x1234]]);
        assert_eq!(r.calls_to(TEXTURE_OWNER_CALL), vec![vec![0x7777, 1, 0, 0]]);
        // The object the getter returns: byte +0x88 cleared, +0x87 set.
        assert_eq!(r.e.mem.u8(dlc + 0x88), 0);
        assert_eq!(r.e.mem.u8(dlc + 0x87), 1);
        assert_eq!(r.e.mem.u8(MAIN_MENU_BYTE), 1);
        assert_eq!(r.e.mem.u8(main + 2), 1);
        // The main title music.
        assert_eq!(
            r.calls_to(STRING_COPY_NARROW)[0][1..],
            [0x40, TEXT_MAIN_TITLE_MUSIC]
        );
        assert_eq!(r.calls_to(SET_DIALOG_FLAG), vec![vec![1]]);
        let play = &r.calls_to(MEDIA_PLAY)[0];
        assert_eq!(play[0], 8);
        assert_eq!(play[2..], [0x1388, 1, 1, 0.0f32.to_bits(), 0]);
    }

    #[test]
    fn choose_main_menu_without_menu_or_dlc_object_skips_those_steps() {
        let mut r = rig();
        let main = r.alloc(0x10);
        r.e.set_global(MAIN_OBJECT, main);
        r.e.call(0x007d_0a70, &args![]);
        assert_eq!(r.e.mem.u8(main + 2), 1);
        assert_eq!(r.e.mem.u8(MAIN_MENU_BYTE), 1);
        assert_eq!(r.calls_to(DLC_GETTER).len(), 1);
    }

    #[test]
    fn the_three_flag_setters() {
        let mut r = rig();
        let object = r.alloc(0x100);
        r.e.call(0x007d_0b90, &args![Ptr::<()>::new(object)]);
        r.e.call(0x007d_0bb0, &args![Ptr::<()>::new(object)]);
        assert_eq!(r.e.mem.u8(object + 2), 1);
        assert_eq!(r.e.mem.u8(object + 0x87), 1);
        assert_eq!(r.e.mem.u8(object + 0x88), 0);
        r.e.call(0x007d_0bd0, &args![Ptr::<()>::new(object), 1u32]);
        assert_eq!(r.e.mem.u8(object + 0x88), 1);
        r.e.call(0x007d_0bd0, &args![Ptr::<()>::new(object), 0x100u32]);
        assert_eq!(r.e.mem.u8(object + 0x88), 0);
    }

    // ----- 007d0bf0 .. 007d3a30 -----

    impl Rig {
        /// Puts the real translation of `addr` back over its double.
        fn real(&mut self, addr: u32) {
            let (_, f) = funcs()
                .into_iter()
                .find(|(a, _)| *a == addr)
                .expect("registered");
            self.e.register(addr, f);
        }
        /// A zeroed user option with its value, second word and count.
        fn option(&mut self, value: u32, second: u32, count: u32) -> u32 {
            let option = self.alloc(0x30);
            self.e.mem.set_u32(option + 0x14, value);
            self.e.mem.set_u32(option + 0x18, second);
            self.e.mem.set_u32(option + 0x20, count);
            option
        }
        /// Menu flag tests answer true for the bits in `set`.
        fn flags(&mut self, set: u32) {
            self.with(MENU_FLAGS_TEST, move |_, a| eax(u32::from(a[1] & set != 0)));
        }
        /// A vtable whose slots (byte offsets) point at doubles returning
        /// the given values; the double for slot `s` of vtable `v` sits at
        /// `0x0eee_0000 + v + s`.
        fn vtable_slots(&mut self, slots: &[(u32, u32)]) -> u32 {
            let vtable = self.alloc(0x100);
            for (slot, value) in slots {
                let target = 0x0eee_0000 + vtable + slot;
                self.e.mem.set_u32(vtable + slot, target);
                self.stub(target, *value);
            }
            vtable
        }
        /// Backs the `Setting` float slots: each setting gets a slot holding
        /// its value; returns the slot addresses.
        fn float_settings(&mut self, values: &[(u32, f32)]) -> HashMap<u32, u32> {
            let mut slots = HashMap::new();
            for (setting, value) in values {
                let slot = self.alloc(4);
                self.e.mem.set_f32(slot, *value);
                slots.insert(*setting, slot);
            }
            let table = slots.clone();
            self.with(FLOAT_SLOT, move |_, a| {
                eax(table.get(&a[0]).copied().unwrap_or(0))
            });
            slots
        }
        /// `004bd510` returns its first argument and `_ftol2` truncates.
        fn slider_math(&mut self) {
            self.with(SLIDER_STEP, |_, a| st0(f32::from_bits(a[0])));
            self.with(FLOAT_TO_INT, |_, a| {
                let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
                eax(value as i32 as u32)
            });
        }
        /// The start menu is running with flag `0x20`.
        fn running_menu(&mut self) -> u32 {
            let menu = self.menu().addr();
            self.flags(0x20);
            menu
        }
    }

    #[test]
    fn hide_models_sets_the_byte_on_both_models_and_calls_the_manager() {
        let mut r = rig();
        r.stub(INTERFACE_MANAGER_GETTER, 0x100);
        r.stub(MENUS_ROOT_GETTER, 0x200);
        r.stub(TILE_PARENT, 0x300);
        r.with(TILE_MODEL, |_, a| eax(a[0] + 1));
        r.e.set_global(MAIN_OBJECT, 0x5000u32);
        r.e.call(0x007d_0bf0, &args![]);
        assert_eq!(
            r.order(),
            vec![
                INTERFACE_MANAGER_GETTER,
                MENUS_ROOT_GETTER,
                TILE_MODEL,
                FN_00450F90,
                TILE_PARENT,
                TILE_MODEL,
                FN_00450F90,
                FN_00713FB0,
                FN_005B6CB0
            ]
        );
        assert_eq!(r.calls_to(MENUS_ROOT_GETTER), vec![vec![0x100]]);
        assert_eq!(
            r.calls_to(FN_00450F90),
            vec![vec![0x201, 1], vec![0x301, 1]]
        );
        assert_eq!(r.calls_to(FN_00713FB0), vec![vec![0x100, 0, 0]]);
        assert_eq!(r.calls_to(FN_005B6CB0), vec![vec![0x5000]]);
    }

    #[test]
    fn choose_fade_kind_two_sets_the_flag_and_refreshes_unless_told_not_to() {
        let mut r = rig();
        r.real(0x007d_0c50);
        r.e.call(0x007d_0c50, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu().addr();
        r.e.call(0x007d_0c50, &args![]);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            r.calls_to(FADE),
            vec![vec![menu, 2, 0, 0x8000_0000, minus_one]]
        );
        assert_eq!(r.calls_to(MENU_FLAGS_SET), vec![vec![menu, 0x1000, 1]]);
        assert_eq!(r.calls_to(FN_007D6030), vec![vec![menu]]);
        assert_eq!(r.calls_to(MENU_REFRESH_A), vec![vec![menu]]);
        r.clear_calls();
        r.stub(FN_007D6030, 1);
        r.e.call(0x007d_0c50, &args![]);
        assert!(r.calls_to(MENU_REFRESH_A).is_empty());
    }

    #[test]
    fn remap_choose_labels_the_tile_and_hides_another_for_the_third_device() {
        let mut r = rig();
        r.real(0x007d_0cb0);
        r.e.call(0x007d_0cb0, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu();
        r.e.mem.set_u32(menu.addr() + 0x68, 0x6800);
        r.e.mem.set_u32(menu.addr() + 0x3c, 0x3c00);
        r.stub(SETTING_STRING, 0x9000);
        r.e.call(0x007d_0cb0, &args![]);
        assert!(r.calls_to(TILE_SET_STRING).is_empty());
        r.flags(0x1000);
        r.e.call(0x007d_0cb0, &args![]);
        assert_eq!(r.calls_to(SETTING_STRING), vec![vec![SETTING_REMAP_TEXT]]);
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x6800, TRAIT_STRING, 0x9000, 1]]
        );
        assert_eq!(
            r.calls_to(TILE_SET_INT),
            vec![vec![0x6800, TRAIT_VISIBLE, 1]]
        );
        r.stub(GAMEPAD_ACTIVE, 1);
        r.clear_calls();
        r.e.set(menu, StartMenu::iRemapDevice, 3);
        r.e.call(0x007d_0cb0, &args![]);
        assert_eq!(
            r.calls_to(TILE_SET_INT),
            vec![
                vec![0x6800, TRAIT_VISIBLE, 1],
                vec![0x3c00, TRAIT_TARGET, 0]
            ]
        );
        r.clear_calls();
        r.e.set(menu, StartMenu::iRemapDevice, 2);
        r.e.call(0x007d_0cb0, &args![]);
        assert_eq!(r.calls_to(TILE_SET_INT).len(), 1);
    }

    #[test]
    fn the_four_options_choosers_populate_their_mode() {
        // (address, mode, tile word offset, trait, value, set before the
        // list is disabled)
        type Case = (u32, u32, Option<(u32, u32, u32, bool)>);
        let cases: [Case; 4] = [
            (0x007d_0d40, 8, None),
            (0x007d_0d70, 0x10, Some((0x50, TRAIT_VISIBLE, 1, true))),
            (0x007d_0dc0, 0x20, None),
            (0x007d_0df0, 0x40, Some((0x64, TRAIT_USER_0, 0, false))),
        ];
        for (addr, mode, tile_trait) in cases {
            let mut r = rig();
            r.e.call(addr, &args![]);
            assert!(r.order().is_empty(), "{addr:08x} without a menu");
            let menu = r.menu().addr();
            r.e.mem.set_u32(menu + 0x50, 0x5000);
            r.e.mem.set_u32(menu + 0x64, 0x6400);
            r.e.call(addr, &args![]);
            let expected = match tile_trait {
                Some((_, _, _, true)) => {
                    vec![
                        TILE_SET_INT,
                        LIST_BOX_SET_ENABLED,
                        POPULATE_OPTIONS_LIST_BOX,
                    ]
                }
                Some(_) => vec![
                    LIST_BOX_SET_ENABLED,
                    TILE_SET_INT,
                    POPULATE_OPTIONS_LIST_BOX,
                ],
                None => vec![LIST_BOX_SET_ENABLED, POPULATE_OPTIONS_LIST_BOX],
            };
            assert_eq!(r.order(), expected, "{addr:08x}");
            assert_eq!(r.calls_to(LIST_BOX_SET_ENABLED), vec![vec![menu + 0xb4, 0]]);
            assert_eq!(
                r.calls_to(POPULATE_OPTIONS_LIST_BOX),
                vec![vec![menu, mode]]
            );
            if let Some((offset, id, value, _)) = tile_trait {
                let tile = r.e.mem.u32(menu + offset);
                assert_eq!(r.calls_to(TILE_SET_INT), vec![vec![tile, id, value]]);
            }
        }
    }

    /// A start menu for `ChooseBack`: the lists at the `enabled` offsets
    /// answer "enabled", the main list has a selection (and the save-load
    /// list one when `save_selected`), the flags `flags` are set.
    fn back_scene(r: &mut Rig, enabled: &[u32], flags: u32, save_selected: bool) -> u32 {
        let menu = r.menu().addr();
        r.real(0x007d_0e40);
        r.flags(flags);
        let enabled: Vec<u32> = enabled.iter().map(|o| menu + o).collect();
        for addr in [
            LIST_BOX_IS_ENABLED,
            INT_LIST_IS_ENABLED,
            SAVE_LOAD_LIST_IS_ENABLED,
        ] {
            let enabled = enabled.clone();
            r.with(addr, move |_, a| eax(u32::from(enabled.contains(&a[0]))));
        }
        r.with(OPTIONS_LIST_HAS_SELECTION, move |_, a| {
            eax(u32::from(
                a[0] == menu + 0x84 || (save_selected && a[0] == menu + 0x174),
            ))
        });
        r.e.mem.set_u32(menu + 0x50, 0x5000);
        menu
    }

    fn fade_kinds(r: &Rig) -> Vec<u32> {
        r.calls_to(FADE).iter().map(|c| c[1]).collect()
    }

    #[test]
    fn choose_back_does_nothing_without_a_menu_or_a_main_list_selection() {
        let mut r = rig();
        r.real(0x007d_0e40);
        r.e.call(0x007d_0e40, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu().addr();
        r.stub(OPTIONS_LIST_HAS_SELECTION, 0);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(r.order(), vec![OPTIONS_LIST_HAS_SELECTION]);
        assert_eq!(
            r.calls_to(OPTIONS_LIST_HAS_SELECTION),
            vec![vec![menu + 0x84]]
        );
    }

    #[test]
    fn choose_back_from_the_options_list_hides_it_and_chooses_settings() {
        let mut r = rig();
        let menu = back_scene(&mut r, &[0x114], 0x20000, false);
        r.stub(SETTING_STRING, 0x9000);
        r.e.call(0x007d_0e40, &args![]);
        // The pending message is shown, the flags and the tile cleared.
        assert_eq!(
            r.calls_to(SHOW_MESSAGE),
            vec![vec![0x9000, 0, 0, 0, 0, 0x12, 0, 0, 0x9000, 0]]
        );
        assert_eq!(
            r.calls_to(SETTING_STRING),
            vec![vec![SETTING_BINDING_TEXT], vec![SETTING_011D2B14]]
        );
        let flag_calls = r.calls_to(MENU_FLAGS_SET);
        assert_eq!(
            flag_calls[..2],
            [vec![menu, 0x20000, 0], vec![menu, 0x20_0000, 0]]
        );
        assert_eq!(r.calls_to(TILE_SET_INT)[0], vec![0x5000, TRAIT_VISIBLE, 0]);
        // Hidden, then ChooseSettings.
        assert_eq!(r.calls_to(HIDE_OPTIONS_CONTAINER), vec![vec![menu]]);
        assert_eq!(r.calls_to(LIST_BOX_SET_ENABLED), vec![vec![menu + 0xb4, 1]]);
        assert_eq!(fade_kinds(&r), vec![1, 5]);
        // Flag 0x1000 alone takes the same way out.
        let mut r = rig();
        let menu = back_scene(&mut r, &[], 0x1000, false);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(r.calls_to(HIDE_OPTIONS_CONTAINER), vec![vec![menu]]);
        assert!(r.calls_to(SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn choose_back_from_the_confirm_list() {
        let mut r = rig();
        let menu = back_scene(&mut r, &[0xe4], 0, false);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED),
            vec![vec![menu + 0xe4, 0], vec![menu + 0x84, 1]]
        );
        assert_eq!(fade_kinds(&r), vec![7, 5]);
        assert_eq!(
            r.calls_to(LIST_BOX_DESELECT_ALL),
            vec![vec![menu + 0x84, 1]]
        );
        // With a selection in the save-load list the flag is set and that is
        // all.
        let mut r = rig();
        let menu = back_scene(&mut r, &[0xe4], 0, true);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(fade_kinds(&r), vec![7]);
        assert_eq!(
            r.calls_to(MENU_FLAGS_SET).last().unwrap(),
            &vec![menu, 0x40000, 1]
        );
        assert!(r.calls_to(LIST_BOX_DESELECT_ALL).is_empty());
    }

    #[test]
    fn choose_back_from_the_settings_downloads_and_save_load_lists() {
        // Settings.
        let mut r = rig();
        let menu = back_scene(&mut r, &[0xb4], 0, false);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(fade_kinds(&r), vec![1, 5]);
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED),
            vec![vec![menu + 0xb4, 0], vec![menu + 0x84, 1]]
        );
        let order = r.order();
        let fade_at = order.iter().position(|a| *a == FADE).unwrap();
        let enabled_at = order
            .iter()
            .position(|a| *a == LIST_BOX_SET_ENABLED)
            .unwrap();
        assert!(fade_at < enabled_at, "the fade comes first for settings");
        // Downloads.
        let mut r = rig();
        let menu = back_scene(&mut r, &[0x144], 0, false);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(fade_kinds(&r), vec![0xe, 5]);
        assert_eq!(
            r.calls_to(INT_LIST_SET_ENABLED),
            vec![vec![menu + 0x144, 0]]
        );
        // Save-load: virtual calls 0 (argument 0) and 0x1c on the list.
        let mut r = rig();
        let menu = back_scene(&mut r, &[0x174], 0, false);
        let vtable = r.vtable_slots(&[(0, 0), (0x1c, 0)]);
        r.e.mem.set_u32(menu + 0x174, vtable);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(fade_kinds(&r), vec![6, 5]);
        assert_eq!(
            r.calls_to(0x0eee_0000 + vtable),
            vec![vec![menu + 0x174, 0]]
        );
        assert_eq!(
            r.calls_to(0x0eee_0000 + vtable + 0x1c),
            vec![vec![menu + 0x174]]
        );
        assert_eq!(
            r.calls_to(UPDATE_HIGHLIGHTED_SAVE_GAME_INFO),
            vec![vec![menu]]
        );
        assert_eq!(
            r.calls_to(SAVE_LOAD_LIST_SET_ENABLED),
            vec![vec![menu + 0x174, 0]]
        );
    }

    #[test]
    fn choose_back_from_the_main_list_closes_the_menu_unless_flag_one_is_missing() {
        // Flag 1 of the start menu missing: nothing happens.
        let mut r = rig();
        let _ = back_scene(&mut r, &[0x84], 0, false);
        r.e.call(0x007d_0e40, &args![]);
        assert!(r.calls_to(MENU_START_FADE_OUT).is_empty());
        assert!(r.calls_to(LIST_BOX_SET_ENABLED).is_empty());
        // Flag 4 set: nothing happens either.
        let mut r = rig();
        let _ = back_scene(&mut r, &[0x84], 4, false);
        r.stub(START_MENU_FLAG_1, 1);
        r.e.call(0x007d_0e40, &args![]);
        assert!(r.calls_to(MENU_START_FADE_OUT).is_empty());
        // Otherwise `Close(1)` fades the menu out.
        let mut r = rig();
        let menu = back_scene(&mut r, &[0x84], 0, false);
        r.stub(START_MENU_FLAG_1, 1);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(r.calls_to(MENU_START_FADE_OUT), vec![vec![menu]]);
        assert!(r.calls_to(LIST_BOX_SET_ENABLED).is_empty());
    }

    #[test]
    fn choose_back_with_nothing_enabled_reselects_the_main_list_unless_flag_40000() {
        let mut r = rig();
        let _ = back_scene(&mut r, &[], 0x40000, false);
        r.e.call(0x007d_0e40, &args![]);
        assert!(r.calls_to(LIST_BOX_SET_ENABLED).is_empty());
        assert!(r.calls_to(FADE).is_empty());
        // Flag 2: the menu is refreshed (no arguments) and the flag cleared.
        let mut r = rig();
        let menu2 = back_scene(&mut r, &[], 2, false);
        r.e.call(0x007d_0e40, &args![]);
        assert_eq!(
            r.calls_to(LIST_BOX_SET_ENABLED),
            vec![vec![menu2 + 0x84, 1]]
        );
        assert_eq!(
            r.calls_to(LIST_BOX_DESELECT_ALL),
            vec![vec![menu2 + 0x84, 1]]
        );
        assert_eq!(fade_kinds(&r), vec![5]);
        assert_eq!(r.calls_to(MENU_REFRESH_D), vec![Vec::<u32>::new()]);
        assert_eq!(
            r.calls_to(MENU_FLAGS_SET).last().unwrap(),
            &vec![menu2, 2, 0]
        );
    }

    // ----- option change callbacks -----

    #[test]
    fn option_callback_for_the_setting_at_011e0940() {
        let mut r = rig();
        r.stub(FN_005BE4D0, 7);
        let player = r.alloc(0x1000);
        r.e.set_global(GLOBAL_OBJECT_011DEA3C, player);
        r.e.mem.set_u32(player + 0x7b8, 7);
        let option = r.option(0, 0, 0);
        r.e.call(0x007d_11d0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 7);
        assert_eq!(r.e.mem.u32(option + 0x18), 2);
        // Running: the value goes to the setting's slot and the object.
        let _ = r.running_menu();
        let slot = r.alloc(4);
        r.stub(SETTING_VALUE_SLOT, slot);
        r.e.mem.set_u32(option + 0x14, 5);
        r.e.call(0x007d_11d0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.calls_to(SETTING_VALUE_SLOT), vec![vec![SETTING_011E0940]]);
        assert_eq!(r.e.mem.u32(slot), 5);
        assert_eq!(r.e.mem.u32(player + 0x7b8), 5);
    }

    #[test]
    fn option_callback_for_the_setting_at_011e087c_and_its_helpers() {
        let mut r = rig();
        let player = r.alloc(0x1000);
        r.e.set_global(GLOBAL_OBJECT_011DEA3C, player);
        r.e.mem.set_u32(player + 0x7c0, 9);
        let option = r.option(0, 0, 0);
        r.e.call(0x007d_1260, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 9);
        assert_eq!(r.e.mem.u32(option + 0x18), 2);
        // Running: stored in the slot and in the object; the float at
        // +0xe18 above zero and a changed value call 0093e770(old, 1).
        let _ = r.running_menu();
        let slot = r.alloc(4);
        r.stub(SETTING_VALUE_SLOT, slot);
        r.e.mem.set_u32(option + 0x14, 4);
        r.e.mem.set_f32(player + 0xe18, 1.0);
        r.e.call(0x007d_1260, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(slot), 4);
        assert_eq!(r.e.mem.u32(player + 0x7c0), 4);
        assert_eq!(r.calls_to(FN_0093E770), vec![vec![player, 9, 1]]);
        // Unchanged value, zero float, NaN: no call.
        r.clear_calls();
        r.e.call(0x007d_1260, &args![Ptr::<()>::new(option)]);
        assert!(r.calls_to(FN_0093E770).is_empty());
        r.e.mem.set_u32(option + 0x14, 6);
        r.e.mem.set_f32(player + 0xe18, 0.0);
        r.e.call(0x007d_1260, &args![Ptr::<()>::new(option)]);
        assert!(r.calls_to(FN_0093E770).is_empty());
        r.e.mem.set_u32(option + 0x14, 8);
        r.e.mem.set_f32(player + 0xe18, f32::NAN);
        r.e.call(0x007d_1260, &args![Ptr::<()>::new(option)]);
        assert!(r.calls_to(FN_0093E770).is_empty());
        assert_eq!(r.e.mem.u32(player + 0x7c0), 8);
    }

    #[test]
    fn the_object_word_accessors() {
        let mut r = rig();
        let object = r.alloc(0x1000);
        r.e.call(0x007d_1240, &args![Ptr::<()>::new(object), 0x1234u32]);
        assert_eq!(r.e.mem.u32(object + 0x7b8), 0x1234);
        r.e.call(0x007d_1320, &args![Ptr::<()>::new(object), 0x4321u32]);
        assert_eq!(r.e.mem.u32(object + 0x7c0), 0x4321);
        assert_eq!(
            r.e.call(0x007d_1340, &args![Ptr::<()>::new(object)]).u32(),
            0x4321
        );
        // The float test is ordered greater-than-zero.
        for (value, expected) in [
            (1.0f32, true),
            (0.0, false),
            (-1.0, false),
            (f32::NAN, false),
            (f32::INFINITY, true),
        ] {
            r.e.mem.set_f32(object + 0xe18, value);
            assert_eq!(
                r.e.call(0x007d_1360, &args![Ptr::<()>::new(object)]).bool(),
                expected,
                "{value}"
            );
        }
    }

    #[test]
    fn hardcore_option_reads_and_sets_the_mode() {
        let mut r = rig();
        let player = r.alloc(0x1000);
        r.e.set_global(GLOBAL_OBJECT_011DEA3C, player);
        let option = r.option(0, 0, 0);
        r.stub(FN_004D1360, 1);
        r.e.call(0x007d_13a0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
        assert_eq!(r.e.mem.u32(option + 0x18), 0);
        assert_eq!(r.calls_to(FN_004D1360), vec![vec![player]]);
        r.stub(FN_004D1360, 0x100);
        r.e.call(0x007d_13a0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 0, "only the low byte counts");
        // Running: the value goes to 007d1400, which only passes 0 and 1.
        let _ = r.running_menu();
        for (value, calls) in [(1u32, 1usize), (0, 1), (2, 0)] {
            r.clear_calls();
            r.e.mem.set_u32(option + 0x14, value);
            r.e.call(0x007d_13a0, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.calls_to(SET_HARDCORE).len(), calls, "{value}");
            if calls == 1 {
                assert_eq!(r.calls_to(SET_HARDCORE), vec![vec![player, value, 0]]);
            }
        }
    }

    #[test]
    fn flag_word_editor_sets_and_clears_bits_of_the_object() {
        let mut r = rig();
        r.e.call(0x007d_14c0, &args![0x40u32, 1u32]);
        let object = r.alloc(0x400);
        r.e.set_global(FLAGS_OBJECT_011D96C0, object);
        r.e.mem.set_u32(object + 0x1c0, 0x101);
        r.e.call(0x007d_14c0, &args![0x40u32, 7u32]);
        assert_eq!(r.e.mem.u32(object + 0x1c0), 0x141);
        r.e.call(0x007d_14c0, &args![0x101u32, 0u32]);
        assert_eq!(r.e.mem.u32(object + 0x1c0), 0x40);
    }

    #[test]
    fn option_callback_for_the_setting_at_011d8a38_edits_the_flag_word() {
        let mut r = rig();
        let option = r.option(0, 0, 0);
        r.stub(SETTING_GET_BOOL, 1);
        r.e.call(0x007d_1440, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.calls_to(SETTING_GET_BOOL), vec![vec![SETTING_011D8A38]]);
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
        assert_eq!(r.e.mem.u32(option + 0x18), 1);
        // Running: set, then the flag word of 011d96c0 follows the value.
        let _ = r.running_menu();
        let object = r.alloc(0x400);
        r.e.set_global(FLAGS_OBJECT_011D96C0, object);
        r.stub(SETTING_GET_VALUE, 1);
        r.e.mem.set_u32(option + 0x14, 1);
        r.e.call(0x007d_1440, &args![Ptr::<()>::new(option)]);
        assert_eq!(
            r.calls_to(SETTING_SET_BOOL),
            vec![vec![SETTING_011D8A38, 1]]
        );
        assert_eq!(r.e.mem.u32(object + 0x1c0), 0x40);
        r.stub(SETTING_GET_VALUE, 0);
        r.e.mem.set_u32(option + 0x14, 0);
        r.e.call(0x007d_1440, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.calls_to(SETTING_SET_BOOL)[1], vec![SETTING_011D8A38, 0]);
        assert_eq!(r.e.mem.u32(object + 0x1c0), 0);
    }

    #[test]
    fn the_boolean_option_callbacks() {
        let table = [
            (0x007d_1510u32, SETTING_011D89F0, 1u32),
            (0x007d_1570, SETTING_011D8974, 1),
            (0x007d_15d0, SETTING_011D8980, 1),
            (0x007d_1630, SETTING_011D8934, 1),
            (0x007d_1690, SETTING_011D8928, 0),
            (0x007d_16f0, SETTING_011D898C, 1),
            (0x007d_2e50, SETTING_011E0A5C, 0),
        ];
        for (addr, setting, kind) in table {
            let mut r = rig();
            let option = r.option(0x55, 0x66, 0);
            r.stub(SETTING_GET_BOOL, 1);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.calls_to(SETTING_GET_BOOL), vec![vec![setting]]);
            assert_eq!(r.e.mem.u32(option + 0x14), 1, "{addr:08x}");
            assert_eq!(r.e.mem.u32(option + 0x18), kind, "{addr:08x}");
            r.stub(SETTING_GET_BOOL, 0);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.u32(option + 0x14), 0);
            // Running: the setting follows `value == 1`.
            let _ = r.running_menu();
            r.e.mem.set_u32(option + 0x14, 1);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            r.e.mem.set_u32(option + 0x14, 2);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(
                r.calls_to(SETTING_SET_BOOL),
                vec![vec![setting, 1], vec![setting, 0]],
                "{addr:08x}"
            );
        }
    }

    #[test]
    fn volume_slider_on_the_setting_at_011d9774() {
        let mut r = rig();
        r.slider_math();
        let option = r.option(3, 0, 5);
        let menu = r.running_menu();
        r.e.call(0x007d_1750, &args![Ptr::<()>::new(option)]);
        assert_eq!(
            r.calls_to(FLOAT_SET),
            vec![vec![VOLUME_OBJECT, 0.75f32.to_bits()]]
        );
        assert_eq!(r.calls_to(MENU_FLAGS_SET), vec![vec![menu, 0x40, 1]]);
        // Not running: the value 0.5 maps to step 2, 1.0 to step 4.
        let mut r = rig();
        r.slider_math();
        let _ = r.float_settings(&[(VOLUME_OBJECT, 0.5)]);
        let option = r.option(0, 0, 5);
        r.e.call(0x007d_1750, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 2);
        assert_eq!(r.e.mem.u32(option + 0x18), 4);
        assert_eq!(
            r.calls_to(SLIDER_STEP),
            vec![
                vec![2.0f32.to_bits(), 1.0f32.to_bits()],
                vec![4.0f32.to_bits(), 1.0f32.to_bits()]
            ]
        );
    }

    #[test]
    fn colour_option_callbacks() {
        for (addr, id) in [(0x007d_1840u32, 1u32), (0x007d_1900, 4)] {
            let mut r = rig();
            for (i, color) in [0xa0u32, 0xb0, 0xc0].iter().enumerate() {
                r.e.mem.set_u32(COLOR_TABLE + 4 * i as u32, *color);
            }
            r.stub(SYSTEM_COLOR_INSTANCE, 0x7000);
            r.stub(INTERFACE_MANAGER_GETTER, 0x100);
            r.stub(MENUS_ROOT_GETTER, 0x200);
            // Not running: the matching table entry is the value.
            r.stub(SYSTEM_COLOR_GET, 0xb0);
            let option = r.option(9, 9, 3);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.u32(option + 0x14), 1, "{addr:08x}");
            assert_eq!(r.e.mem.u32(option + 0x18), 2);
            assert_eq!(r.calls_to(SYSTEM_COLOR_GET)[0], vec![0x7000, id]);
            // No match: the default word (2) becomes the value.
            r.stub(SYSTEM_COLOR_GET, 0xff);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.u32(option + 0x14), 2);
            // A match on the first entry stops at 0.
            r.stub(SYSTEM_COLOR_GET, 0xa0);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.u32(option + 0x14), 0);
            // Running: the colour is set and the root told.
            let _ = r.running_menu();
            r.e.mem.set_u32(option + 0x14, 2);
            r.clear_calls();
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.calls_to(SYSTEM_COLOR_SET), vec![vec![0x7000, id, 0xc0]]);
            assert_eq!(
                r.calls_to(FN_00A0B350),
                vec![vec![0x200, id, 0.0f32.to_bits()]]
            );
        }
    }

    #[test]
    fn range_slider_between_two_settings_and_a_float_sink() {
        // Running: min 0, max 2, step 1 of 4 -> 0.5.
        let mut r = rig();
        let _ = r.float_settings(&[
            (SETTING_MINIMUM_011DAB98, 0.0),
            (SETTING_MAXIMUM_011DAB60, 2.0),
        ]);
        let option = r.option(1, 0, 5);
        let _ = r.running_menu();
        r.e.call(0x007d_19c0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.calls_to(FLOAT_SINK), vec![vec![0.5f32.to_bits()]]);
        // Not running: 007d1b30 is 0.5 (a quarter of the range: step 1) and
        // 1.0 is half the range (step 2).
        let mut r = rig();
        r.slider_math();
        let _ = r.float_settings(&[
            (SETTING_MINIMUM_011DAB98, 0.0),
            (SETTING_MAXIMUM_011DAB60, 2.0),
        ]);
        r.stub_float(FLOAT_AT_OBJECT, 0.5);
        let option = r.option(0, 0, 5);
        r.e.call(0x007d_19c0, &args![Ptr::<()>::new(option)]);
        assert_eq!(
            r.calls_to(FLOAT_AT_OBJECT),
            vec![vec![FLOAT_OBJECT_011C71AC]]
        );
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
        assert_eq!(r.e.mem.u32(option + 0x18), 2);
    }

    #[test]
    fn float_getters() {
        let mut r = rig();
        r.stub_float(FLOAT_AT_OBJECT, 3.5);
        assert_eq!(r.e.call(0x007d_1b30, &args![]).f32(), 3.5);
        assert_eq!(
            r.calls_to(FLOAT_AT_OBJECT),
            vec![vec![FLOAT_OBJECT_011C71AC]]
        );
        r.e.map(0x011a_d000, 0x1000);
        r.e.mem.set_f32(FLOAT_TABLE + 8, -2.25);
        assert_eq!(r.e.call(0x007d_1d00, &args![2i32]).f32(), -2.25);
    }

    #[test]
    fn integer_setting_option_callback() {
        let mut r = rig();
        r.e.map(0x0126_e000, 0x1000);
        r.e.set_global(GLOBAL_0126EFC4, 5u32);
        let option = r.option(0, 0x77, 0);
        r.e.call(0x007d_1b40, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 5);
        assert_eq!(r.e.mem.u32(option + 0x18), 0);
        let menu = r.running_menu();
        r.e.mem.set_u32(option + 0x14, 5);
        r.e.call(0x007d_1b40, &args![Ptr::<()>::new(option)]);
        r.e.mem.set_u32(option + 0x14, 6);
        r.e.call(0x007d_1b40, &args![Ptr::<()>::new(option)]);
        assert_eq!(
            r.calls_to(SETTING_SET_INT),
            vec![vec![SETTING_011C779C, 5], vec![SETTING_011C779C, 6]]
        );
        assert_eq!(
            r.calls_to(MENU_FLAGS_SET),
            vec![vec![menu, 0x40_0000, 0], vec![menu, 0x40_0000, 1]]
        );
    }

    #[test]
    fn audio_level_sliders() {
        // (address, minimum, maximum, table index, target, default)
        let table = [
            (
                0x007d_1bb0u32,
                0x011d_ab1cu32,
                0x011d_aaf4u32,
                3u32,
                0x011c_3ec0u32,
                0x011d_ac38u32,
            ),
            (
                0x007d_1d10,
                0x011d_abf0,
                0x011d_ac24,
                2,
                0x011c_3d0c,
                0x011d_abc0,
            ),
            (
                0x007d_1e60,
                0x011d_ab30,
                0x011d_ab78,
                1,
                0x011c_3d78,
                0x011d_ab6c,
            ),
        ];
        for (addr, min, max, index, target, default) in table {
            // Running: value 1 of 3 steps is halfway between 1.0 and 5.0.
            let mut r = rig();
            let _ = r.float_settings(&[(min, 1.0), (max, 5.0)]);
            let option = r.option(1, 0, 3);
            let _ = r.running_menu();
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(
                r.calls_to(FLOAT_TABLE_SET),
                vec![vec![index, 3.0f32.to_bits()]],
                "{addr:08x}"
            );
            assert_eq!(r.calls_to(FLOAT_SET), vec![vec![target, 3.0f32.to_bits()]]);
            // Not running: table value 3.0 is step 1, the default 5.0 step 2.
            let mut r = rig();
            r.slider_math();
            r.e.map(0x011a_d000, 0x1000);
            r.e.mem.set_f32(FLOAT_TABLE + 4 * index, 3.0);
            let _ = r.float_settings(&[(min, 1.0), (max, 5.0), (default, 5.0)]);
            let option = r.option(0, 0, 3);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.u32(option + 0x14), 1, "{addr:08x}");
            assert_eq!(r.e.mem.u32(option + 0x18), 2);
        }
    }

    #[test]
    fn range_sliders_store_into_their_setting() {
        // (address, minimum, maximum, setting, default)
        let table = [
            (
                0x007d_1fb0u32,
                0x011c_a4acu32,
                0x011c_a4d8u32,
                0x011c_a450u32,
                0x011c_a4ccu32,
            ),
            (
                0x007d_2510,
                0x011d_86c4,
                0x011d_86ec,
                0x011d_8724,
                0x011d_870c,
            ),
            (
                0x007d_2650,
                0x011d_8734,
                0x011d_8718,
                0x011d_8788,
                0x011d_8770,
            ),
        ];
        for (addr, min, max, setting, default) in table {
            let mut r = rig();
            let slots = r.float_settings(&[(min, 1.0), (max, 5.0), (setting, 0.0)]);
            let option = r.option(1, 0, 3);
            let _ = r.running_menu();
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.f32(slots[&setting]), 3.0, "{addr:08x}");
            // Not running: setting 3.0 is step 1, default 5.0 step 2.
            let mut r = rig();
            r.slider_math();
            let _ = r.float_settings(&[(min, 1.0), (max, 5.0), (setting, 3.0), (default, 5.0)]);
            let option = r.option(0, 0, 3);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.u32(option + 0x14), 1);
            assert_eq!(r.e.mem.u32(option + 0x18), 2);
        }
    }

    #[test]
    fn range_sliders_with_copies() {
        // (address, minimum, maximum, setting, default, copy, extra, sum)
        type Case = (u32, u32, u32, u32, u32, u32, u32, u32);
        let table: [Case; 3] = [
            (
                0x007d_20f0,
                0x011c_7294,
                0x011c_71b8,
                0x011c_7568,
                0x011c_7278,
                0x011f_944c,
                0x011c_7400,
                0x011f_9450,
            ),
            (
                0x007d_2250,
                0x011c_72ac,
                0x011c_74ec,
                0x011c_7740,
                0x011c_744c,
                0x011f_9444,
                0x011c_76b4,
                0x011f_9448,
            ),
            (
                0x007d_23b0,
                0x011c_7440,
                0x011c_71d0,
                0x011c_7598,
                0x011c_7494,
                0x011f_9454,
                0x011c_7634,
                0x011f_9458,
            ),
        ];
        for (addr, min, max, setting, default, copy, extra, sum) in table {
            let mut r = rig();
            r.e.map(0x011f_9000, 0x1000);
            let slots = r.float_settings(&[(min, 1.0), (max, 5.0), (setting, 0.0), (extra, 1.5)]);
            let option = r.option(1, 0, 3);
            let _ = r.running_menu();
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.f32(slots[&setting]), 3.0, "{addr:08x}");
            assert_eq!(r.e.mem.f32(copy), 3.0);
            assert_eq!(r.e.mem.f32(sum), 4.5);
            let mut r = rig();
            r.slider_math();
            let _ = r.float_settings(&[(min, 1.0), (max, 5.0), (setting, 3.0), (default, 5.0)]);
            let option = r.option(0, 0, 3);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.e.mem.u32(option + 0x14), 1);
            assert_eq!(r.e.mem.u32(option + 0x18), 2);
        }
    }

    #[test]
    fn audio_volume_sliders() {
        // (address, setting, audio setter, audio getter, the float 004bd510
        // gets for +0x18 with count 5, the step stored there): 4 times 1.0
        // (FLD1), the double 0.5 at 01011588, or the double
        // 0x3FE3333340000000 at 010290b8 (4 times it is exactly 2.4f32).
        // The doubles are read from memory, so the rig holds the exe's
        // values at their addresses.
        let table = [
            (
                0x007d_2790u32,
                0x011f_6e14u32,
                0x00ad_7b10u32,
                0x00ad_79f0u32,
                4.0f32,
                4u32,
            ),
            (0x007d_28b0, 0x011f_6e20, 0x00ad_7b40, 0x00ad_7a20, 2.0, 2),
            (0x007d_29d0, 0x011f_6e44, 0x00ad_7bd0, 0x00ad_7ab0, 2.4, 2),
            (0x007d_2af0, 0x011f_6e2c, 0x00ad_7b70, 0x00ad_7a50, 2.4, 2),
            (0x007d_2c10, 0x011f_6e38, 0x00ad_7ba0, 0x00ad_7a80, 4.0, 4),
            (0x007d_2d30, 0x011f_6e50, 0x00ad_7c00, 0x00ad_7ae0, 2.0, 2),
        ];
        for (addr, setting, setter, getter, scaled, step) in table {
            // No option: the audio object gets the setting's value.
            let mut r = rig();
            r.stub(setter, 0);
            r.stub(getter, 0);
            r.stub(AUDIO_INSTANCE, 0xa0d10);
            r.stub_float(FLOAT_AT_OBJECT, 0.75);
            r.e.call(addr, &args![Ptr::<()>::new(0)]);
            assert_eq!(r.calls_to(setter), vec![vec![0xa0d10, 0.75f32.to_bits()]]);
            assert_eq!(r.calls_to(FLOAT_AT_OBJECT), vec![vec![setting]]);
            // Running: the slider position is stored, then applied.
            let mut r = rig();
            r.stub(setter, 0);
            r.stub(AUDIO_INSTANCE, 0xa0d10);
            r.stub_float(FLOAT_AT_OBJECT, 0.75);
            r.with(FLOAT_SET, |_, a| eax(a[0]));
            let option = r.option(3, 0, 5);
            let _ = r.running_menu();
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(
                r.calls_to(FLOAT_SET),
                vec![vec![setting, 0.75f32.to_bits()]]
            );
            assert_eq!(r.calls_to(FLOAT_AT_OBJECT), vec![vec![setting]]);
            assert_eq!(r.calls_to(setter), vec![vec![0xa0d10, 0.75f32.to_bits()]]);
            // Not running: the getter's 0.5 is step 2; +0x18 is 4 times the
            // slider's own constant.
            let mut r = rig();
            r.slider_math();
            r.e.mem.set_f64(DOUBLE_HALF, 0.5);
            r.e.mem
                .set_f64(DOUBLE_SIX_TENTHS, f64::from_bits(0x3FE3_3333_4000_0000));
            r.stub(AUDIO_INSTANCE, 0xa0d10);
            r.with(getter, |_, _| st0(0.5));
            let option = r.option(0, 0, 5);
            r.e.call(addr, &args![Ptr::<()>::new(option)]);
            assert_eq!(r.calls_to(getter), vec![vec![0xa0d10]]);
            assert_eq!(
                r.calls_to(SLIDER_STEP),
                vec![
                    vec![2.0f32.to_bits(), 1.0f32.to_bits()],
                    vec![scaled.to_bits(), 1.0f32.to_bits()]
                ],
                "{addr:08x}"
            );
            assert_eq!(r.e.mem.u32(option + 0x14), 2, "{addr:08x}");
            assert_eq!(r.e.mem.u32(option + 0x18), step, "{addr:08x}");
        }
    }

    #[test]
    fn slider_with_double_constants() {
        let constants = |r: &mut Rig| {
            r.e.mem.set_f64(DOUBLE_SLIDER_SCALE, 0.0095);
            r.e.mem.set_f64(DOUBLE_SLIDER_OFFSET, 0.0005);
            r.e.mem.set_f32(FLOAT_SLIDER_FULL, 0.157_894_75);
        };
        let mut r = rig();
        constants(&mut r);
        let slots = r.float_settings(&[(SETTING_011E0A6C, 0.0)]);
        let option = r.option(2, 0, 5);
        let _ = r.running_menu();
        r.e.call(0x007d_2eb0, &args![Ptr::<()>::new(option)]);
        let expected = (0.5f64 * 0.0095 + 0.0005) as f32;
        assert_eq!(r.e.mem.f32(slots[&SETTING_011E0A6C]), expected);
        // Not running: that value maps back to step 2 of 4; the constant
        // 0.157894 * 4 truncates to 0.
        let mut r = rig();
        constants(&mut r);
        r.slider_math();
        let _ = r.float_settings(&[(SETTING_011E0A6C, expected)]);
        let option = r.option(0, 7, 5);
        r.e.call(0x007d_2eb0, &args![Ptr::<()>::new(option)]);
        let first = ((f64::from(expected) - 0.0005) / 0.0095) as f32;
        let step = ((4.0 * f64::from(first)) as f32) as i32;
        assert_eq!(r.e.mem.u32(option + 0x14), step as u32);
        assert_eq!(r.e.mem.u32(option + 0x18), 0);
    }

    #[test]
    fn interface_manager_option_callback() {
        let mut r = rig();
        r.stub(INTERFACE_MANAGER_GETTER, 0x100);
        // Running, value 1, with the tile at +0x1cc.
        let menu = r.running_menu();
        r.e.mem.set_u32(menu + 0x1cc, 0x7000);
        r.e.mem.set_u32(menu + 0x1d0, 0x55);
        let option = r.option(1, 0, 0);
        r.e.call(0x007d_2fc0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.calls_to(FN_00719740), vec![vec![0x100, 0]]);
        assert_eq!(r.calls_to(TILE_SET_INT), vec![vec![0x7000, 0x55, 1]]);
        // Value 0: the call gets 1 and the trait 0; no tile: no trait.
        r.clear_calls();
        r.e.mem.set_u32(option + 0x14, 0);
        r.e.call(0x007d_2fc0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.calls_to(FN_00719740), vec![vec![0x100, 1]]);
        assert_eq!(r.calls_to(TILE_SET_INT), vec![vec![0x7000, 0x55, 0]]);
        r.clear_calls();
        r.e.mem.set_u32(menu + 0x1cc, 0);
        r.e.call(0x007d_2fc0, &args![Ptr::<()>::new(option)]);
        assert!(r.calls_to(TILE_SET_INT).is_empty());
        // Not running: the setting at 011d8c4c (byte 0) becomes value 1, the
        // main object's target gets the byte and 007d30b0 its inverse.
        let mut r = rig();
        r.e.map(0x011f_3000, 0x1000);
        let object = r.alloc(0x2000);
        r.e.set_global(OBJECT_011F35CC, object);
        r.e.set_global(MAIN_OBJECT, 0x4000u32);
        r.stub(INPUT_OBJECT_GETTER, 0x8000);
        let option = r.option(9, 9, 0);
        r.stub(SETTING_GET_BOOL, 0);
        r.e.call(0x007d_2fc0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
        assert_eq!(r.e.mem.u32(option + 0x18), 1);
        assert_eq!(r.calls_to(FN_005407B0), vec![vec![0x8000, 0]]);
        assert_eq!(r.calls_to(INPUT_OBJECT_GETTER).len(), 2);
        assert_eq!(r.e.mem.u8(object + 0x1b88), 1);
        r.stub(SETTING_GET_BOOL, 1);
        r.e.call(0x007d_2fc0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 0);
        assert_eq!(r.e.mem.u8(object + 0x1b88), 0);
    }

    #[test]
    fn small_byte_accessors() {
        let mut r = rig();
        r.e.map(0x011f_3000, 0x1000);
        // 007d30b0 needs the object.
        r.e.call(0x007d_30b0, &args![5u32]);
        let object = r.alloc(0x2000);
        r.e.set_global(OBJECT_011F35CC, object);
        r.e.call(0x007d_30b0, &args![0x105u32]);
        assert_eq!(r.e.mem.u8(object + 0x1b88), 5);
        // 007d30d0 returns the byte.
        r.stub(SETTING_GET_BOOL, 0x1234);
        assert_eq!(r.e.call(0x007d_30d0, &args![]).u8(), 0x34);
        assert_eq!(r.calls_to(SETTING_GET_BOOL), vec![vec![SETTING_011D8C4C]]);
        // 007d3170 reads +0x85.
        r.e.mem.set_u8(object + 0x85, 9);
        assert_eq!(
            r.e.call(0x007d_3170, &args![Ptr::<()>::new(object)]).u8(),
            9
        );
    }

    #[test]
    fn option_callback_for_the_setting_at_011e0850() {
        let mut r = rig();
        r.e.map(0x011f_3000, 0x1000);
        let dlc = r.alloc(0x100);
        r.stub(DLC_GETTER, dlc);
        r.stub(SETTING_GET_BOOL, 1);
        let option = r.option(0, 0, 0);
        r.e.mem.set_u8(dlc + 0x85, 1);
        r.e.call(0x007d_30e0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.calls_to(SETTING_GET_BOOL), vec![vec![SETTING_011E0850]]);
        assert_eq!(r.e.mem.u32(option + 0x14), 1);
        assert_eq!(r.e.mem.u32(option + 0x18), 1);
        r.e.mem.set_u8(dlc + 0x85, 0);
        r.stub(SETTING_GET_BOOL, 0);
        r.e.call(0x007d_30e0, &args![Ptr::<()>::new(option)]);
        assert_eq!(r.e.mem.u32(option + 0x14), 0);
        assert_eq!(r.e.mem.u32(option + 0x18), 0);
        // Running: the setting is set from the option and read back into
        // 007d30b0.
        let object = r.alloc(0x2000);
        r.e.set_global(OBJECT_011F35CC, object);
        let _ = r.running_menu();
        r.stub(SETTING_SET_BOOL, 0x6000);
        r.with(SETTING_GET_BOOL, |_, a| eax(u32::from(a[0] == 0x6000)));
        r.e.mem.set_u32(option + 0x14, 1);
        r.clear_calls();
        r.e.call(0x007d_30e0, &args![Ptr::<()>::new(option)]);
        assert_eq!(
            r.calls_to(SETTING_SET_BOOL),
            vec![vec![SETTING_011E0850, 1]]
        );
        assert_eq!(r.e.mem.u8(object + 0x1b88), 1);
    }

    // ----- ConfirmContinue / ConfirmNewGame -----

    #[test]
    fn confirm_continue_loads_the_first_save_entry() {
        let mut r = rig();
        let _ = r.menu();
        let manager = r.alloc(0x10);
        r.e.set_global(CALLBACK_OWNER, manager);
        let node = r.alloc(8);
        r.e.mem.set_u32(node, 0xe0e0);
        r.with(FIRST_WORD, move |_, _| eax(node));
        r.with(SETTING_STRING, |_, a| eax(a[0]));
        r.e.call(0x007d_3190, &args![]);
        assert!(r.calls_to(FN_00851330).is_empty());
        assert_eq!(r.calls_to(SAVE_ENTRY_IS_CORRUPT), vec![vec![0xe0e0]]);
        assert_eq!(r.calls_to(ARCHIVE_CLEANUP).len(), 1);
        assert_eq!(r.calls_to(FN_008467B0), vec![vec![0xe0e0]]);
        assert!(r.calls_to(SHOW_MESSAGE).is_empty());
        // ChooseBack ran (the main list has no selection, so it stops there).
        assert_eq!(r.calls_to(OPTIONS_LIST_HAS_SELECTION).len(), 1);
    }

    #[test]
    fn confirm_continue_shows_a_message_for_a_corrupt_or_missing_entry() {
        // Corrupt first entry.
        let mut r = rig();
        let manager = r.alloc(0x10);
        r.e.set_global(CALLBACK_OWNER, manager);
        let node = r.alloc(8);
        r.e.mem.set_u32(node, 0xe0e0);
        r.with(FIRST_WORD, move |_, _| eax(node));
        r.with(SETTING_STRING, |_, a| eax(a[0]));
        r.stub(SAVE_ENTRY_IS_CORRUPT, 1);
        r.e.call(0x007d_3190, &args![]);
        assert_eq!(
            r.calls_to(SHOW_MESSAGE),
            vec![vec![
                SETTING_011D1F14,
                0,
                0,
                0,
                0,
                5,
                0,
                0,
                SETTING_BINDING_TEXT,
                0
            ]]
        );
        assert!(r.calls_to(FN_008467B0).is_empty());
        assert!(r.calls_to(ARCHIVE_CLEANUP).is_empty());
        // No first node: the manager is asked to fill the list and the
        // other message shows.
        let mut r = rig();
        let manager = r.alloc(0x10);
        r.e.set_global(CALLBACK_OWNER, manager);
        r.with(SETTING_STRING, |_, a| eax(a[0]));
        r.e.call(0x007d_3190, &args![]);
        assert_eq!(r.calls_to(FN_00851330), vec![vec![manager]]);
        assert_eq!(
            r.calls_to(SHOW_MESSAGE),
            vec![vec![
                SETTING_011D48AC,
                0,
                0,
                0,
                0,
                5,
                0,
                0,
                SETTING_BINDING_TEXT,
                0
            ]]
        );
        // A node whose item is 0 does the same.
        let mut r = rig();
        let manager = r.alloc(0x10);
        r.e.set_global(CALLBACK_OWNER, manager);
        let node = r.alloc(8);
        r.with(FIRST_WORD, move |_, _| eax(node));
        r.with(SETTING_STRING, |_, a| eax(a[0]));
        r.e.call(0x007d_3190, &args![]);
        assert_eq!(r.calls_to(FN_00851330), vec![vec![manager]]);
        assert_eq!(r.calls_to(SHOW_MESSAGE).len(), 1);
    }

    #[test]
    fn confirm_new_game_without_the_quest_logs_and_backs_out() {
        for form_type in [None, Some(0x46u32)] {
            let mut r = rig();
            let _ = r.menu();
            let text = r.alloc(0x10);
            r.with(SETTING_STRING, move |_, _| eax(text));
            r.stub(STRTOL, 0x1a2b);
            match form_type {
                None => r.stub(LOOKUP_FORM, 0),
                Some(kind) => {
                    r.stub(LOOKUP_FORM, 0x6000);
                    r.stub(FN_00401170, kind);
                }
            }
            r.e.call(0x007d_32b0, &args![]);
            assert_eq!(r.calls_to(LOOKUP_FORM), vec![vec![0x1a2b]]);
            let strtol = r.calls_to(STRTOL);
            assert_eq!(strtol[0][0], text);
            assert_eq!(strtol[0][2], 16);
            assert_eq!(
                r.calls_to(LOG_MESSAGE),
                vec![vec![TEXT_QUEST_NOT_FOUND, text]]
            );
            assert!(r.calls_to(RT_DYNAMIC_CAST).is_empty());
            assert!(r.calls_to(ARCHIVE_CLEANUP).is_empty());
            // ChooseBack ran.
            assert_eq!(r.calls_to(OPTIONS_LIST_HAS_SELECTION).len(), 1);
        }
    }

    #[test]
    fn confirm_new_game_starts_the_quest() {
        let mut r = rig();
        let text = r.alloc(0x10);
        r.with(SETTING_STRING, move |_, _| eax(text));
        r.stub(STRTOL, 0x1a2b);
        r.stub(LOOKUP_FORM, 0x6000);
        r.stub(FN_00401170, 0x47);
        r.with(RT_DYNAMIC_CAST, |_, a| eax(a[0]));
        r.stub(GET_FORM, 0x9000);
        r.stub(OPTION_TEXT, 0xa000);
        r.stub(INTERFACE_MANAGER_GETTER, 0x100);
        let player = r.alloc(0x100);
        r.e.set_global(GLOBAL_OBJECT_011DEA3C, player);
        let loading = r.alloc(0x100);
        r.e.mem.set_u32(loading + 0x54, 0x5400);
        r.e.set_global(LOADING_MENU, loading);
        let dlc = r.alloc(0x100);
        r.stub(DLC_GETTER, dlc);
        r.e.map(0x011d_8000, 0x1000);
        r.e.call(0x007d_32b0, &args![]);
        assert_eq!(
            r.calls_to(RT_DYNAMIC_CAST),
            vec![
                vec![0x6000, 0, TYPE_DESCRIPTOR_FORM, TYPE_DESCRIPTOR_QUEST, 0],
                vec![
                    0x9000,
                    0,
                    TYPE_DESCRIPTOR_BOUND_OBJECT,
                    TYPE_DESCRIPTOR_NPC,
                    0
                ]
            ]
        );
        assert_eq!(r.calls_to(FN_005D1530), vec![vec![player, 0]]);
        assert_eq!(r.calls_to(FN_005D14D0), vec![vec![player, 1]]);
        assert_eq!(r.calls_to(ARCHIVE_CLEANUP).len(), 1);
        assert_eq!(r.e.mem.u8(FLAG_011D8907), 1);
        assert_eq!(r.calls_to(FN_007ADCC0), vec![vec![player, 0]]);
        assert_eq!(r.calls_to(GET_FORM), vec![vec![player]]);
        assert_eq!(r.calls_to(OPTION_TEXT), vec![vec![SETTING_011D4F78]]);
        assert_eq!(
            r.calls_to(SET_STRING_FIELD),
            vec![vec![0x9000 + 0xd0, 0xa000]]
        );
        assert_eq!(r.calls_to(FN_0060C9C0), vec![vec![0x6000, 1]]);
        assert_eq!(
            r.calls_to(TILE_SET_INT),
            vec![vec![0x5400, TRAIT_VISIBLE, 1]]
        );
        assert_eq!(r.calls_to(FN_004581C0), vec![vec![1]]);
        assert_eq!(r.calls_to(ERROR_CALL).len(), 1);
        assert_eq!(r.calls_to(SET_DIALOG_FLAG), vec![vec![0]]);
        assert_eq!(r.calls_to(MEDIA_CLOSE).len(), 1);
        assert_eq!(r.e.mem.u8(dlc + 0x88), 1);
        assert_eq!(r.e.mem.u8(dlc + 0x87), 1);
        assert_eq!(r.calls_to(FN_00788A90), vec![vec![1]]);
        assert_eq!(
            r.calls_to(TUTORIAL_SHOW_MESSAGE),
            vec![
                vec![0x100 + 0x4d4, 0x19, 0x3eb, 0x200],
                vec![0x100 + 0x4d4, 0x1a, 0x3ff, 0x200]
            ]
        );
        assert!(r.calls_to(LOG_MESSAGE).is_empty());
    }

    // ----- 007d3470, 007d35c0, 007d3a30 -----

    fn load_scene(r: &mut Rig) -> u32 {
        let menu = r.menu().addr();
        r.e.mem.set_u32(menu + 0x1b0, 0x5a5a);
        r.stub(OPTIONS_LIST_HAS_SELECTION, 0);
        menu
    }

    #[test]
    fn load_selected_save_needs_a_menu_with_a_save_game() {
        let mut r = rig();
        r.e.call(0x007d_3470, &args![]);
        assert!(r.order().is_empty());
        let _ = r.menu();
        r.e.call(0x007d_3470, &args![]);
        assert!(r.order().is_empty());
    }

    #[test]
    fn load_selected_save_closes_the_menu_or_backs_out() {
        // Loaded with flag 1: closes the start menu.
        let mut r = rig();
        let menu = load_scene(&mut r);
        r.flags(1);
        r.stub(FN_008467B0, 1);
        r.stub(START_MENU_FLAG_1, 1);
        r.e.call(0x007d_3470, &args![]);
        assert_eq!(r.calls_to(ARCHIVE_CLEANUP).len(), 1);
        assert_eq!(r.calls_to(FN_008467B0), vec![vec![0x5a5a]]);
        assert_eq!(r.calls_to(MENU_START_FADE_OUT), vec![vec![menu]]);
        assert!(r.calls_to(OPTIONS_LIST_HAS_SELECTION).is_empty());
        // Loaded without the flag: ChooseBack.
        let mut r = rig();
        let _ = load_scene(&mut r);
        r.stub(FN_008467B0, 1);
        r.e.call(0x007d_3470, &args![]);
        assert!(r.calls_to(MENU_START_FADE_OUT).is_empty());
        assert_eq!(r.calls_to(OPTIONS_LIST_HAS_SELECTION).len(), 1);
        // Not loaded: the flag 0x20000000 is looked at, then ChooseBack.
        let mut r = rig();
        let menu = load_scene(&mut r);
        r.e.call(0x007d_3470, &args![]);
        assert!(r
            .calls_to(MENU_FLAGS_TEST)
            .contains(&vec![menu, 0x2000_0000]));
        assert_eq!(r.calls_to(OPTIONS_LIST_HAS_SELECTION).len(), 1);
    }

    #[test]
    fn load_selected_save_tells_the_texture_owner_when_the_names_match() {
        for (matched, calls) in [(1u32, 1usize), (0, 0)] {
            let mut r = rig();
            let _ = load_scene(&mut r);
            r.e.set_global(GLOBAL_OBJECT_011DEA3C, 0x1234u32);
            r.e.set_global(TEXTURE_OWNER, 0x7777u32);
            r.stub(START_MENU_FLAG_1, 1);
            r.stub(GET_FORM, 0x9000);
            r.with(RT_DYNAMIC_CAST, |_, a| eax(a[0]));
            r.stub(FN_00408DA0, 0x40);
            r.stub(FN_0075F8C0, 0x50);
            r.stub(FN_00408B20, matched);
            r.e.call(0x007d_3470, &args![]);
            assert_eq!(
                r.calls_to(RT_DYNAMIC_CAST),
                vec![vec![
                    0x9000,
                    0,
                    TYPE_DESCRIPTOR_BOUND_OBJECT,
                    TYPE_DESCRIPTOR_NPC,
                    0
                ]]
            );
            assert_eq!(r.calls_to(FN_00408B20), vec![vec![0x40, 0x50]]);
            assert_eq!(r.calls_to(FN_00408DA0)[0], vec![0x9000 + 0xd0]);
            assert_eq!(r.calls_to(FN_0075F8C0), vec![vec![0x5a5a], vec![0x5a5a]]);
            assert_eq!(r.calls_to(TEXTURE_OWNER_CALL).len(), calls);
            if calls == 1 {
                assert_eq!(r.calls_to(TEXTURE_OWNER_CALL), vec![vec![0x7777, 1, 0, 0]]);
            }
        }
        // No location name: nothing further is asked.
        let mut r = rig();
        let _ = load_scene(&mut r);
        r.e.set_global(GLOBAL_OBJECT_011DEA3C, 0x1234u32);
        r.stub(START_MENU_FLAG_1, 1);
        r.stub(GET_FORM, 0x9000);
        r.with(RT_DYNAMIC_CAST, |_, a| eax(a[0]));
        r.e.call(0x007d_3470, &args![]);
        assert!(r.calls_to(FN_0075F8C0).is_empty());
    }

    #[test]
    fn save_selected_choose_function() {
        let mut r = rig();
        r.e.call(0x007d_35c0, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu().addr();
        let vtable = r.vtable_slots(&[(0x1c, 0)]);
        r.e.mem.set_u32(menu + 0xe4, vtable);
        r.e.mem.set_u32(menu + 0x48, 0x4800);
        r.stub(SETTING_STRING, 0x9000);
        r.e.call(0x007d_35c0, &args![]);
        assert_eq!(
            r.calls_to(0x0eee_0000 + vtable + 0x1c),
            vec![vec![menu + 0xe4]]
        );
        assert_eq!(
            r.calls_to(SETTING_STRING),
            vec![vec![SETTING_SAVE_SELECTED]]
        );
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![vec![0x4800, TRAIT_STRING, 0x9000, 1]]
        );
        assert_eq!(r.calls_to(MENU_FLAGS_SET), vec![vec![menu, 0x4000, 1]]);
    }

    #[test]
    fn message_callback_clears_the_binding_when_the_answer_is_yes() {
        let mut r = rig();
        r.real(0x007d_3a30);
        r.e.call(0x007d_3a30, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu();
        r.e.set(menu, StartMenu::iRemapDevice, 2);
        r.e.set_global(MAIN_OBJECT, 0x4000u32);
        r.stub(INPUT_OBJECT_GETTER, 0x8000);
        for answer in [0u32, 0xff] {
            r.stub(MESSAGE_MENU_RESULT, answer);
            r.e.call(0x007d_3a30, &args![]);
            assert!(r.calls_to(FN_00A24B70).is_empty(), "{answer}");
        }
        r.stub(MESSAGE_MENU_RESULT, 1);
        r.e.call(0x007d_3a30, &args![]);
        assert_eq!(r.calls_to(INPUT_OBJECT_GETTER), vec![vec![0x4000]]);
        assert_eq!(r.calls_to(FN_00A24B70), vec![vec![0x8000, 2]]);
        assert_eq!(r.calls_to(MENU_FLAGS_SET), vec![vec![menu.addr(), 2, 1]]);
        assert_eq!(
            r.calls_to(LIST_BOX_FOR_EACH),
            vec![vec![menu.addr() + 0x114, FN_007CFE40, 0, 0x7fff_ffff]]
        );
    }

    // ----- 007d3620 -----

    /// A start menu whose save-load list answers the virtual calls
    /// `DeleteSave` needs: slot 4 returns the highlighted item, slot 0x14
    /// returns a non-null item.
    fn delete_scene(r: &mut Rig) -> (u32, u32) {
        let menu = r.menu().addr();
        let vtable = r.vtable_slots(&[(4, 0x777), (0x14, 0x888)]);
        r.e.mem.set_u32(menu + 0x174, vtable);
        r.stub(LIST_BOX_HIGHLIGHTED_ITEM, 0x800);
        r.stub(OPTIONS_LIST_HAS_SELECTION, 0);
        r.with(SETTING_STRING, |_, a| eax(a[0]));
        r.with(FN_007D86C0, |_, a| eax(0x900 + a[1]));
        r.with(FN_007D6E90, |_, _| eax(0x6e90));
        (menu, vtable)
    }

    #[test]
    fn delete_save_needs_a_highlighted_item() {
        let mut r = rig();
        r.e.call(0x007d_3620, &args![]);
        assert!(r.order().is_empty());
        let menu = r.menu().addr();
        let vtable = r.vtable_slots(&[(4, 0)]);
        r.e.mem.set_u32(menu + 0x174, vtable);
        r.e.call(0x007d_3620, &args![]);
        assert!(r.calls_to(REMOVE_SAVE_GAME).is_empty());
    }

    #[test]
    fn delete_save_with_nothing_left_refreshes_and_clears_the_flag() {
        let mut r = rig();
        let (menu, vtable) = delete_scene(&mut r);
        r.stub(FN_007D8700, 0);
        r.stub(FN_007D87B0, 4);
        r.e.call(0x007d_3620, &args![]);
        let list = menu + 0x174;
        assert_eq!(r.calls_to(LIST_BOX_HIGHLIGHTED_ITEM), vec![vec![list]]);
        assert_eq!(r.calls_to(REMOVE_SAVE_GAME), vec![vec![0x800]]);
        assert_eq!(r.calls_to(0x0eee_0000 + vtable + 4).len(), 2);
        assert_eq!(r.calls_to(FN_007D8040), vec![vec![list, 0x777]]);
        assert_eq!(r.calls_to(FN_007D62E0), vec![vec![menu]]);
        assert_eq!(r.calls_to(MENU_FLAGS_SET), vec![vec![menu, 0x40000, 0]]);
        // ChooseBack twice (the main list has no selection here).
        assert_eq!(r.calls_to(OPTIONS_LIST_HAS_SELECTION).len(), 2);
    }

    #[test]
    fn delete_save_at_the_threshold_redraws_the_item_after_the_last() {
        let mut r = rig();
        let (menu, vtable) = delete_scene(&mut r);
        r.e.mem.set_f64(DOUBLE_ONE, 1.0);
        r.stub(FN_007D8700, 5);
        r.stub(FN_007D87B0, 3);
        r.stub(TILE_ADD_USER_TRAIT, 0x4242);
        r.stub(FN_00825C00, 0x5000);
        r.set_tile(0x5000, 0x4242, 2.0);
        r.e.set(
            Ptr::<StartMenu>::new(menu),
            StartMenu::fSaveLoadThreshold,
            2.0,
        );
        r.with(FLOAT_TO_INT, |_, a| {
            eax(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        r.stub(LIST_BOX_COUNT, 2);
        // Flag 8: entries 0x900 (autosave) and 0x901 (other) are looked at.
        r.flags(8);
        r.with(IS_AUTOSAVE, |_, a| eax(u32::from(a[0] == 0x900)));
        r.with(FN_00846B10, |_, a| eax(u32::from(a[0] == 0x901)));
        r.e.call(0x007d_3620, &args![]);
        let list = menu + 0x174;
        assert_eq!(
            r.calls_to(TILE_ADD_USER_TRAIT),
            vec![vec![TEXT_CURRENT_VALUE, 0xffff_ffff]]
        );
        // 3 + 2.0 - 1.0 = 4: the item (4, 5) is initialised.
        assert_eq!(
            r.calls_to(LIST_BOX_FOR_EACH),
            vec![vec![list, INITIALIZE_SAVE_LOAD_LIST_ITEM, 4, 5]]
        );
        // Both entries are dropped: 2 -> 0, then one more is subtracted
        // for the count line: -1 is printed as the unsigned word.
        assert_eq!(
            r.calls_to(FN_007D80F0),
            vec![vec![list, 0, 0], vec![list, 1, 0]]
        );
        let text = &r.calls_to(STRING_COPY_NARROW)[0];
        assert_eq!(text[1], 0x40);
        assert_eq!(
            text[2..],
            [
                FORMAT_SAVE_COUNT,
                SETTING_011D2C4C,
                (-1i32) as u32,
                1000,
                SETTING_011D25D4
            ]
        );
        assert_eq!(
            r.calls_to(TILE_SET_STRING).last().unwrap(),
            &vec![0x6e90, TRAIT_STRING, text[0], 1]
        );
        assert_eq!(r.calls_to(TILE_ADD_NEEDS_UPDATE), vec![vec![0x6e90, 8]]);
        assert_eq!(r.calls_to(0x0eee_0000 + vtable + 0x14).len(), 2);
        // The threshold is untouched.
        assert_eq!(
            r.e.get(Ptr::<StartMenu>::new(menu), StartMenu::fSaveLoadThreshold),
            2.0
        );
    }

    #[test]
    fn delete_save_away_from_the_threshold_lowers_it() {
        // A threshold that differs from the scrollbar's value is lowered by
        // 1.0; 0.0 stays; with flag 8 and no count yet the entries are
        // recounted.
        for (start, expected) in [(3.0f32, 2.0f32), (0.0, 0.0), (f32::NAN, f32::NAN)] {
            let mut r = rig();
            let (menu, _) = delete_scene(&mut r);
            r.e.mem.set_f64(DOUBLE_ZERO, 0.0);
            r.e.mem.set_f64(DOUBLE_ONE, 1.0);
            r.stub(FN_007D8700, 5);
            r.stub(FN_007D87B0, 3);
            r.stub(TILE_ADD_USER_TRAIT, 0x4242);
            r.stub(FN_00825C00, 0x5000);
            r.set_tile(0x5000, 0x4242, 2.0);
            r.e.set(
                Ptr::<StartMenu>::new(menu),
                StartMenu::fSaveLoadThreshold,
                start,
            );
            r.e.call(0x007d_3620, &args![]);
            let now =
                r.e.get(Ptr::<StartMenu>::new(menu), StartMenu::fSaveLoadThreshold);
            if expected.is_nan() {
                assert!(now.is_nan());
            } else {
                assert_eq!(now, expected, "{start}");
            }
            assert!(r.calls_to(LIST_BOX_FOR_EACH).is_empty());
        }
        // Count smaller than the limit: the same way out; with flag 8 the
        // entries are counted afresh (3), the last is not an autosave.
        let mut r = rig();
        let (menu, _) = delete_scene(&mut r);
        r.e.mem.set_f64(DOUBLE_ZERO, 0.0);
        r.e.mem.set_f64(DOUBLE_ONE, 1.0);
        r.stub(FN_007D8700, 2);
        r.stub(FN_007D87B0, 3);
        r.stub(LIST_BOX_COUNT, 3);
        r.flags(8);
        r.with(IS_AUTOSAVE, |_, a| eax(u32::from(a[0] == 0x901)));
        r.e.set(
            Ptr::<StartMenu>::new(menu),
            StartMenu::fSaveLoadThreshold,
            1.0,
        );
        r.e.call(0x007d_3620, &args![]);
        assert_eq!(
            r.e.get(Ptr::<StartMenu>::new(menu), StartMenu::fSaveLoadThreshold),
            0.0
        );
        // 3 entries, one autosave: 2, minus one for the count line.
        assert_eq!(r.calls_to(STRING_COPY_NARROW)[0][4], 1);
        assert!(r.calls_to(FN_007D80F0).is_empty());
    }
}
