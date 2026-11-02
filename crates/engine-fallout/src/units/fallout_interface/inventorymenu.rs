//! `fallout/interface/menus/inventorymenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Pip-Boy inventory menu (`InventoryMenu`, class `0x3ea`): a `Menu`
//! with 23 tiles from +0x28, the current tab, six page states, the
//! inventory list, the hot-key wheel and the list of items to update. The
//! singleton `InventoryMenu::pMe` is `011d9ea4` and the selected item
//! (`pCurrItem`, an `ItemChange *`) `011d9ea8`. The constructor and the
//! destructor keep a C++ exception-unwinding frame (`FS:[0]`) that is not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `InventoryMenu::pMe` (Xbox PDB static): the menu singleton.
const INVENTORY_MENU: u32 = 0x011d_9ea4;
/// `InventoryMenu::pCurrItem` (Xbox PDB static): the selected item.
const CURRENT_ITEM: u32 = 0x011d_9ea8;
/// A byte static `DoIdle` sets while the hot-key wheel has a highlight
/// (its name is not confirmed).
const WHEEL_HIGHLIGHTED: u32 = 0x011d_9f38;
/// The menu class number.
const INVENTORY_MENU_CLASS: u32 = 0x3ea;
/// `InventoryMenu`'s vtable.
const INVENTORY_MENU_VTABLE: u32 = 0x0107_39b4;
/// The five trait ids the constructor looks up (the Xbox PDB lists
/// `iUT_Value`, `iUT_ItemType`, `iUT_CurrentTab`, `iUT_KeyringOpen` and
/// `iUT_EquippableItem`; which address is which is not confirmed, so they
/// are named by the address of the trait name each is looked up with).
const TRAIT_FROM_NAME_0106ED18: u32 = 0x011d_9eb4;
const TRAIT_FROM_NAME_010739A0: u32 = 0x011d_9eb0;
const TRAIT_FROM_NAME_01073994: u32 = 0x011d_9eac;
const TRAIT_FROM_NAME_01073984: u32 = 0x011d_9eb8;
const TRAIT_FROM_NAME_01073974: u32 = 0x011d_9ebc;
/// The input singleton (the object `00877720` answers the controls of).
const INPUT_GLOBALS: u32 = 0x011d_ea0c;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;

/// `Menu::Menu`.
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The `Menu` destructor body.
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `ListBox<ItemChange *>::ListBox<ItemChange *>`.
const LIST_BOX_CONSTRUCTOR: u32 = 0x0073_0810;
/// The list box's destructor body.
const LIST_BOX_DESTRUCTOR: u32 = 0x0073_08e0;
/// The hot-key wheel's constructor (the shared body that answers `this`).
const WHEEL_CONSTRUCTOR: u32 = 0x0068_15c0;
/// The hot-key wheel's destructor body.
const WHEEL_DESTRUCTOR: u32 = 0x0070_1520;
/// The item update list's constructor and destructor bodies.
const UPDATE_LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const UPDATE_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// `Tile::TextToTrait` (cdecl, one word).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The interface manager singleton getter.
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// `Tile::GetMenuByClass` (cdecl, one word).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// The tile reader of the interface manager (`this` only).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`this` + file name).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu`.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Menu::RemoveFromXMLArchive` (cdecl: file name, menu).
const MENU_REMOVE_FROM_XML_ARCHIVE: u32 = 0x00a1_ca20;
/// `Menu::SetMenuTile` (`this`, tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// Sets the byte at +0x1c of `this` to 1 (`bRenderedMenu` on a menu).
const SET_BYTE_AT_0X1C: u32 = 0x0086_7a20;
/// The print function the menu reports a missing tile with (cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// Reads a float trait of a tile (`this`, trait; result in ST0).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// Sets a float trait of a tile (`this`, trait, value, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// Sets a string trait of a tile (`this`, trait, text, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// Sets an integer trait of a tile (`this`, trait, value).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// `Tile::IsTrue` (`this`, trait).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetMaximumDepth` (no arguments; result in ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// The tutorial message shower (`this` = interface + 0x4d4; id, class, time).
const SHOW_TUTORIAL: u32 = 0x0071_8630;
/// `ListBox<unsigned int>::Initialize` (`this`, container tile, template).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// A list box call with one word (`007a1950(0, word)` behind it).
const LIST_BOX_WITH_FLAG: u32 = 0x007a_1930;
/// `Interface::InitializeTabline` (cdecl, eight words).
const INITIALIZE_TABLINE: u32 = 0x0070_7be0;
/// `HotKeysWheel::Initialize` (`this`, tile).
const WHEEL_INITIALIZE: u32 = 0x0070_1610;
/// A setting's string value (`ECX` = the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The word at +4 of an object (a menu's `pMenu` tile).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// `Interface::IsTopMenuID` (cdecl, one word).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `Interface::GetPipboy` (no arguments).
const GET_PIPBOY: u32 = 0x0070_5990;
/// A state test of the Pip-Boy object (`this` only; a byte).
const PIPBOY_READY: u32 = 0x0097_4d90;
/// Whether a gamepad is in use (no arguments).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
/// `HotKeysWheel::GetVisible`, `SetVisible` and `MoveHighlight`.
const WHEEL_GET_VISIBLE: u32 = 0x0070_1740;
const WHEEL_SET_VISIBLE: u32 = 0x0070_1760;
const WHEEL_MOVE_HIGHLIGHT: u32 = 0x0070_1bd0;
/// The hot-key wheel's item assignment (`this`, item).
const WHEEL_ASSIGN: u32 = 0x0070_19e0;
/// The hot-key wheel's slot count (`this` only).
const WHEEL_COUNT: u32 = 0x006e_4ba0;
/// `ItemChange::GetItemHealth` (`this`, flag; ST0).
const ITEM_GET_HEALTH: u32 = 0x004b_cdb0;
/// The item's object (`this` only).
const ITEM_OBJECT: u32 = 0x0044_ddc0;
/// A form's type byte (`this` only).
const FORM_TYPE: u32 = 0x0040_1170;
/// Two float getters on the item's object (`this` only; ST0).
const OBJECT_VALUE_004BE180: u32 = 0x004b_e180;
const OBJECT_VALUE_004BE060: u32 = 0x004b_e060;
/// The message box shower (cdecl, six words).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// Adds the object to a list (`this`, address of the object).
const LIST_ADD_OBJECT: u32 = 0x005a_e3d0;
/// `InventoryMenu::UpdateInventoryMenu` (no arguments).
const UPDATE_INVENTORY_MENU: u32 = 0x0078_2a90;
/// The controls object of the input singleton (`this` only).
const INPUT_CONTROLS: u32 = 0x0087_7720;
/// Two key-state queries of the controls (`this`, key, flag).
const CONTROL_STATE: u32 = 0x00a2_4660;
const CONTROL_PRESSED: u32 = 0x00a2_3a50;
/// `Interface::IsInMenuMode` (no arguments).
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// Fills the item card tiles (cdecl: three tiles, item, flag).
const SET_ITEM_CARD: u32 = 0x0071_9790;

/// The menu file name, the missing-tile message and the list template.
const MENU_FILE: u32 = 0x0107_3a70;
const MISSING_TILE_MESSAGE: u32 = 0x0107_3a18;
const ITEM_TEMPLATE_NAME: u32 = 0x0107_39fc;
/// The two strings and the float the message boxes are shown with.
const MESSAGE_TEXT_ONE: u32 = 0x0102_08a0;
const MESSAGE_TEXT_TWO: u32 = 0x0107_3a94;
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// The two message settings.
const BROKEN_ITEM_SETTING: u32 = 0x011d_3810;
const NO_HOTKEY_SETTING: u32 = 0x011d_1e18;
/// The `double` stacking types the depth is set for, and 0.0.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The trait name the tab tiles' labels are set with.
const TAB_LABEL_TRAIT_NAME: u32 = 0x0106_ed20;
/// Tile traits.
const TRAIT_VISIBLE: i32 = 0xfa3;
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_STRING: i32 = 0xfc4;
const TRAIT_0XFA9: i32 = 0xfa9;

layout! {
    /// `InventoryMenu` (Xbox PDB), 0x124 bytes; the PC code uses the PDB's
    /// offsets.
    pub struct InventoryMenu: 0x124 {
        /// `iCurrTab` (Xbox PDB).
        0x84 iCurrTab: i32,
    }
}

/// Offsets of `pPageStatesA` (six `sPageState`s of two words),
/// `lbInventoryList`, `HKWheel` and `ItemUpdateList` (Xbox PDB).
const PAGE_STATES: u32 = 0x88;
const INVENTORY_LIST: u32 = 0xb8;
const HOT_KEY_WHEEL: u32 = 0xe8;
const ITEM_UPDATE_LIST: u32 = 0x11c;

fn me(e: &Engine) -> u32 {
    e.global::<u32>(INVENTORY_MENU)
}

fn current_item(e: &Engine) -> u32 {
    e.global::<u32>(CURRENT_ITEM)
}

fn tile_at(e: &Engine, menu: u32, offset: u32) -> u32 {
    e.mem.u32(menu.wrapping_add(offset))
}

fn set_trait_from_int(e: &mut Engine, tile: u32, trait_id: i32, value: i32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
}

/// The type byte of the selected item's object.
fn current_item_type(e: &mut Engine) -> u8 {
    let item = current_item(e);
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    e.call(FORM_TYPE, &args![object]).u8()
}

// Translated from 0077fa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::InventoryMenu` (Xbox PDB): constructs the `Menu` base,
/// stores the class's vtable, constructs the inventory list, the hot-key
/// wheel and the item update list, looks up the five trait ids the menu
/// keeps in statics, sets the six page states to (-1, 0) and the current
/// tab to 0. Returns `this`.
// Translation incomplete: 0077fa00 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn inventory_menu_inventory_menu(
    e: &mut Engine,
    this: Ptr<InventoryMenu>,
) -> Ptr<InventoryMenu> {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), INVENTORY_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.byte_add(INVENTORY_LIST)]);
    e.call(WHEEL_CONSTRUCTOR, &args![this.byte_add(HOT_KEY_WHEEL)]);
    e.call(
        UPDATE_LIST_CONSTRUCTOR,
        &args![this.byte_add(ITEM_UPDATE_LIST)],
    );
    for (name, slot) in [
        (0x0106_ed18u32, TRAIT_FROM_NAME_0106ED18),
        (0x0107_39a0, TRAIT_FROM_NAME_010739A0),
        (0x0107_3994, TRAIT_FROM_NAME_01073994),
        (0x0107_3984, TRAIT_FROM_NAME_01073984),
        (0x0107_3974, TRAIT_FROM_NAME_01073974),
    ] {
        let id = e.call(TILE_TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(slot, id);
    }
    for i in 0..6 {
        let state = this.addr() + PAGE_STATES + 8 * i;
        e.mem.set_u32(state, u32::MAX);
        e.mem.set_u32(state + 4, 0);
    }
    e.set(this, InventoryMenu::iCurrTab, 0);
    this
}

// Translated from 0077fb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::GetClass` (Xbox PDB): the class number `0x3ea`.
pub fn inventory_menu_get_class(_e: &mut Engine, _this: Ptr<InventoryMenu>) -> u32 {
    INVENTORY_MENU_CLASS
}

// Translated from 0077fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn inventory_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<InventoryMenu>,
    flags: u32,
) -> Ptr<InventoryMenu> {
    fn_0077fb80(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0077fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `InventoryMenu` destructor: stores the class's vtable, clears
/// `pMe`, destroys the item update list, the hot-key wheel and the
/// inventory list (in that order) and runs the `Menu` destructor.
// Translation incomplete: 0077fb80 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_0077fb80(e: &mut Engine, this: Ptr<InventoryMenu>) {
    e.mem.set_u32(this.addr(), INVENTORY_MENU_VTABLE);
    e.set_global(INVENTORY_MENU, 0u32);
    e.call(
        UPDATE_LIST_DESTRUCTOR,
        &args![this.byte_add(ITEM_UPDATE_LIST)],
    );
    e.call(WHEEL_DESTRUCTOR, &args![this.byte_add(HOT_KEY_WHEEL)]);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(INVENTORY_LIST)]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 0077fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::Create` (Xbox PDB, cdecl, no arguments): asks for the
/// maximum depth (the value is not used), deletes an open menu of the
/// class, reads the menu file (`01073a70`), takes its menu as `pMe` and
/// removes the file from the XML archive. For a menu of this class:
/// attaches the tile, marks it rendered, checks all 23 tiles (else prints
/// a message and answers 0), sets the depth for the two stacking types,
/// shows tutorial message `0x22` (time `0x200`), sets up the inventory list
/// on the tile at +0x38, the tab line on the tile at +0x5c (five setting
/// strings), the hot-key wheel on the tile at +0x3c, the text of the tile
/// at +0x50 and the labels of the five tab tiles at +0x60..+0x70 that
/// exist, and prepares the menu for showing (flag 1). Returns the root
/// tile. A menu of another class is deleted (when it has a tile) and the
/// result is 0.
pub fn inventory_menu_create(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(TILE_GET_MAXIMUM_DEPTH, &args![]);
    let class = fn_0077ff80(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(INVENTORY_MENU, menu);
    let menu = me(e);
    e.call(MENU_REMOVE_FROM_XML_ARCHIVE, &args![MENU_FILE, menu]);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_0077ff80(e) {
            return finish_create(e, root);
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let menu = me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`inventory_menu_create`] after the menu of the class was
/// read.
fn finish_create(e: &mut Engine, root: u32) -> u32 {
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    let menu = me(e);
    e.call(SET_BYTE_AT_0X1C, &args![menu]);
    if !fn_0077ff90(e, Ptr::new(me(e))) {
        e.call(PRINT_MESSAGE, &args![MISSING_TILE_MESSAGE]);
        return 0;
    }
    let kind = e
        .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
        .f64();
    let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
        let again = e
            .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
            .f64();
        again == e.global::<f64>(STACKING_TYPE_TWO)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            SET_FLOAT_TRAIT,
            &args![root, TRAIT_DEPTH, x87_store32(depth), true],
        );
    }
    let class = fn_0077ff80(e);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(
        SHOW_TUTORIAL,
        &args![interface.wrapping_add(0x4d4), 0x22u32, class, 0x200u32],
    );
    let container = tile_at(e, me(e), 0x38);
    let list = me(e).wrapping_add(INVENTORY_LIST);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![list, container, ITEM_TEMPLATE_NAME],
    );
    let list = me(e).wrapping_add(INVENTORY_LIST);
    e.call(LIST_BOX_WITH_FLAG, &args![list, 0u32]);
    // The tab labels, read in the order the code pushes them.
    let mut labels = [0u32; 5];
    for (label, setting) in labels.iter_mut().zip([
        0x011d_3258u32,
        0x011d_2acc,
        0x011d_40b4,
        0x011d_4d50,
        0x011d_29b8,
    ]) {
        *label = e.call(SETTING_STRING, &args![setting]).u32();
    }
    let tab_line = tile_at(e, me(e), 0x5c);
    e.call(
        INITIALIZE_TABLINE,
        &args![tab_line, 0x18u32, labels[4], labels[3], labels[2], labels[1], labels[0], 0u32],
    );
    let wheel_tile = tile_at(e, me(e), 0x3c);
    let wheel = me(e).wrapping_add(HOT_KEY_WHEEL);
    e.call(WHEEL_INITIALIZE, &args![wheel, wheel_tile]);
    let text = e.call(SETTING_STRING, &args![0x011d_29f4u32]).u32();
    let tile = tile_at(e, me(e), 0x50);
    e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, text, true]);
    let label_trait = e
        .call(TILE_TEXT_TO_TRAIT, &args![TAB_LABEL_TRAIT_NAME])
        .u32();
    for (offset, setting) in [
        (0x60u32, 0x011d_2c7cu32),
        (0x64, 0x011d_412c),
        (0x68, 0x011d_330c),
        (0x6c, 0x011d_20c4),
        (0x70, 0x011d_396c),
    ] {
        if tile_at(e, me(e), offset) != 0 {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            let tile = tile_at(e, me(e), offset);
            e.call(SET_STRING_TRAIT, &args![tile, label_trait, text, true]);
        }
    }
    let menu = me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 1u32]);
    root
}

// Translated from 0077ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x3ea`, without an object (no `this`).
pub fn fn_0077ff80(_e: &mut Engine) -> u32 {
    INVENTORY_MENU_CLASS
}

// Translated from 0077ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all 23 tiles of `pTiles` are set.
pub fn fn_0077ff90(e: &mut Engine, this: Ptr<InventoryMenu>) -> bool {
    (0..0x17).all(|i| tile_at(e, this.addr(), 0x28 + 4 * i) != 0)
}

// Translated from 00781ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DoIdle` (Xbox PDB). Only while the menu is the top menu
/// and the Pip-Boy's state test (`00974d90`) passes:
/// - with a gamepad and the hot-key wheel shown: hides the tiles at +0x78
///   and +0x80 and moves the wheel's highlight by -1; a highlight sets the
///   byte static `011d9f38`; without one, when the static is set, the
///   selected item is assigned to the wheel (or a message is shown: for a
///   broken item, health 0.0; for an item of type `0x19` or when the
///   keyring trait is false), and the static is cleared;
/// - with a gamepad and the wheel hidden: for a selected item of type
///   `0x28` shows those tiles and fills them with its card;
/// - with the mouse: for an item of type `0x28` shows those tiles, then the
///   number keys (controls `0x11 + i`, slot 1 skipped) open the wheel on
///   their slot; with none pressed an open wheel is closed (and the tile at
///   +0x54 shown when the list has a highlighted item).
///
/// Then, in menu mode, control 1 with a highlighted item and control `0xe`
/// up clicks button 7 (`this`'s slot `+0xc`); the item card is filled for
/// the current tab (`InventoryMenu::pMe`'s `iCurrTab`: 0 the three tiles at
/// +0x60.., 1 the tiles at +0x6c and +0x70 when the object's two values are
/// not 0.0), the tile at +0x30's trait `0xfa9` is set to 0.0 and the last
/// card is filled with the tile at +0x30 when the player's actor value
/// `0x12` is not 0.0.
pub fn inventory_menu_do_idle(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    if !e.call(IS_TOP_MENU_ID, &args![INVENTORY_MENU_CLASS]).bool() {
        return;
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_READY, &args![pipboy]).bool() {
        return;
    }
    let wheel = me_addr + HOT_KEY_WHEEL;
    if e.call(GAMEPAD_IN_USE, &args![]).bool() {
        if e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool() {
            gamepad_wheel_shown(e, this);
        } else if current_item(e) != 0 && current_item_type(e) == 0x28 {
            show_card_tiles(e, me_addr, 1);
            for offset in [0x78u32, 0x80] {
                let tile = tile_at(e, me_addr, offset);
                let item = current_item(e);
                e.call(SET_ITEM_CARD, &args![0u32, 0u32, tile, item, 1u32]);
            }
        }
    } else {
        mouse_hot_keys(e, this);
    }
    finish_idle(e, this);
}

/// Shows or hides the tiles at +0x78 and +0x80.
fn show_card_tiles(e: &mut Engine, menu: u32, visible: i32) {
    for offset in [0x78u32, 0x80] {
        let tile = tile_at(e, menu, offset);
        set_trait_from_int(e, tile, TRAIT_VISIBLE, visible);
    }
}

/// Shows a message box with the setting's text.
fn show_message(e: &mut Engine, setting: u32) {
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![text, 0u32, MESSAGE_TEXT_ONE, MESSAGE_TEXT_TWO, time, 0u32],
    );
}

/// [`inventory_menu_do_idle`] with a gamepad and the hot-key wheel shown.
fn gamepad_wheel_shown(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    let wheel = me_addr + HOT_KEY_WHEEL;
    show_card_tiles(e, me_addr, 0);
    let highlight = e.call(WHEEL_MOVE_HIGHLIGHT, &args![wheel, -1i32]).i32();
    if highlight >= 0 {
        e.set_global(WHEEL_HIGHLIGHTED, 1u8);
        return;
    }
    if e.global::<u8>(WHEEL_HIGHLIGHTED) == 0 {
        return;
    }
    let broken = current_item(e) != 0 && {
        let item = current_item(e);
        let health = e.call(ITEM_GET_HEALTH, &args![item, true]).f64();
        health == e.global::<f64>(ZERO_DOUBLE)
    };
    if broken {
        show_message(e, BROKEN_ITEM_SETTING);
    } else {
        let keyring_trait = e.global::<u32>(TRAIT_FROM_NAME_01073974);
        let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        let refused = !e.call(TILE_IS_TRUE, &args![tile, keyring_trait]).bool()
            || (current_item(e) != 0 && current_item_type(e) == 0x19);
        if refused {
            show_message(e, NO_HOTKEY_SETTING);
        } else {
            let item = current_item(e);
            e.call(WHEEL_ASSIGN, &args![wheel, item]);
            let item = current_item(e);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), object);
                e.call(LIST_ADD_OBJECT, &args![me_addr + ITEM_UPDATE_LIST, slot]);
            });
            e.call(UPDATE_INVENTORY_MENU, &args![]);
            e.call(WHEEL_SET_VISIBLE, &args![wheel, 1u32]);
            let tile = tile_at(e, me_addr, 0x54);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        }
    }
    e.set_global(WHEEL_HIGHLIGHTED, 0u8);
}

/// [`inventory_menu_do_idle`] without a gamepad: the number keys.
fn mouse_hot_keys(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    let wheel = me_addr + HOT_KEY_WHEEL;
    let mut done = false;
    if current_item(e) != 0 && current_item_type(e) == 0x28 {
        show_card_tiles(e, me_addr, 1);
    }
    let mut slot = 0i32;
    loop {
        if done {
            break;
        }
        let keyring_open = e.global::<u32>(TRAIT_FROM_NAME_01073984);
        let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        if e.call(TILE_IS_TRUE, &args![tile, keyring_open]).bool() {
            break;
        }
        if slot >= e.call(WHEEL_COUNT, &args![wheel]).i32() {
            break;
        }
        let controls = e
            .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
            .u32();
        let key = slot.wrapping_add(0x11);
        let pressed = if e.call(CONTROL_STATE, &args![controls, key, 1u32]).i32() != 0 {
            true
        } else {
            let controls = e
                .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
                .u32();
            e.call(CONTROL_STATE, &args![controls, key, 0u32]).i32() != 0 && slot != 1
        };
        if pressed && slot != 1 {
            if !e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool() {
                e.call(WHEEL_SET_VISIBLE, &args![wheel, 1u32]);
                let tile = tile_at(e, me_addr, 0x54);
                set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
            } else {
                show_card_tiles(e, me_addr, 0);
            }
            e.call(WHEEL_MOVE_HIGHLIGHT, &args![wheel, slot]);
            done = true;
        }
        slot = slot.wrapping_add(1);
    }
    if !done && e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool() {
        e.call(WHEEL_SET_VISIBLE, &args![wheel, 0u32]);
        let list = me_addr + INVENTORY_LIST;
        if e.vcall(list, 4, &args![]).u32() != 0 {
            let tile = tile_at(e, me_addr, 0x54);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 1);
        }
    }
}

/// The tail of [`inventory_menu_do_idle`] that every mode runs.
fn finish_idle(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let me_addr = this.addr();
    let list = me_addr + INVENTORY_LIST;
    if e.call(IS_IN_MENU_MODE, &args![]).bool() {
        let controls = e
            .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
            .u32();
        if e.call(CONTROL_PRESSED, &args![controls, 1i32, 1u32]).bool()
            && e.vcall(list, 4, &args![]).u32() != 0
        {
            let controls = e
                .call(INPUT_CONTROLS, &args![e.global::<u32>(INPUT_GLOBALS)])
                .u32();
            if e.call(CONTROL_STATE, &args![controls, 0xei32, 1u32]).i32() == 0 {
                e.vcall(me_addr, 0xc, &args![7u32, 0u32]);
            }
        }
    }
    let tab = e
        .get(Ptr::<InventoryMenu>::new(me(e)), InventoryMenu::iCurrTab)
        .wrapping_add(0x18);
    if tab == 0x18 {
        let first = tile_at(e, me_addr, 0x60);
        let second = tile_at(e, me_addr, 0x64);
        let third = tile_at(e, me_addr, 0x68);
        let item = current_item(e);
        e.call(SET_ITEM_CARD, &args![first, second, third, item, 0u32]);
    } else if tab == 0x19 {
        let second = if current_item(e) != 0 && {
            let item = current_item(e);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            let value = e.call(OBJECT_VALUE_004BE180, &args![object]).f64();
            value != e.global::<f64>(ZERO_DOUBLE)
        } {
            tile_at(e, me_addr, 0x70)
        } else {
            0
        };
        let first = if current_item(e) != 0 && {
            let item = current_item(e);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            let value = e.call(OBJECT_VALUE_004BE060, &args![object]).f64();
            value != e.global::<f64>(ZERO_DOUBLE)
        } {
            tile_at(e, me_addr, 0x6c)
        } else {
            0
        };
        let item = current_item(e);
        e.call(SET_ITEM_CARD, &args![first, second, 0u32, item, 0u32]);
    }
    let tile = tile_at(e, me_addr, 0x30);
    e.call(SET_FLOAT_TRAIT, &args![tile, TRAIT_0XFA9, 0.0f32, true]);
    let player = e.global::<u32>(PLAYER);
    // The player's object at +0xa4, slot +0xc, asked for actor value 0x12.
    let value = e
        .vcall(player.wrapping_add(0xa4), 0xc, &args![0x12u32])
        .f64();
    let first = if value == e.global::<f64>(ZERO_DOUBLE) {
        0
    } else {
        tile_at(e, me_addr, 0x30)
    };
    let last = tile_at(e, me_addr, 0x7c);
    let item = current_item(e);
    e.call(SET_ITEM_CARD, &args![first, last, 0u32, item, 0u32]);
}

// Translated from 007824b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low byte of `InventoryMenu::pMe`'s `iCurrTab`, or 0 without a menu.
pub fn fn_007824b0(e: &mut Engine) -> u8 {
    if me(e) == 0 {
        0
    } else {
        e.mem.u8(me(e) + 0x84)
    }
}

/// `_ftol2_sse` (the value in ST0, as a leading `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// The highlighted tile of the inventory list (`00825c00`, `thiscall`: the
/// word at +0x14 of the list).
const LIST_WORD_AT_0X14: u32 = 0x0082_5c00;
/// The trait name the list's scroll position is read with.
const SCROLL_TRAIT_NAME: u32 = 0x0107_02d8;
/// Restores the list's scroll state (`007832e0` in this unit calls it):
/// `ListBox<ItemChange *>::RestoreListState` (Xbox PDB; `this`, flag).
const LIST_RESTORE_STATE: u32 = 0x0073_1360;
/// `FOPipboyManager::SetTabKnobPosition` and `FOPipboyManager::MoveKnob`
/// (Xbox PDB) and the Pip-Boy manager of the interface manager (`00704370`).
const PIPBOY_MANAGER: u32 = 0x0070_4370;
const SET_TAB_KNOB_POSITION: u32 = 0x007f_a0f0;
const MOVE_KNOB: u32 = 0x007f_8610;
/// The word at +0 of an object (`00559450`) and the identity `006815c0`.
const WORD_AT_ZERO: u32 = 0x0055_9450;
const IDENTITY: u32 = 0x0068_15c0;
/// `ExtraDataList::GetCount` (Xbox PDB; a 16-bit count).
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
/// The drop call of the player (`009614b0`, `playercharacter.cpp`;
/// `thiscall`: object, address of the position, flag, flag).
const PLAYER_DROP_PREPARE: u32 = 0x0096_14b0;
/// The position the dropped item starts from (three floats).
const DROP_POSITION: u32 = 0x011f_426c;
/// `Interface::GetTopMenuID` (Xbox PDB).
const GET_TOP_MENU_ID: u32 = 0x0070_23c0;
/// Sorts the list (`007314c0`, `bartermenu.cpp`; `thiscall`: compare
/// function, two words).
const LIST_SORT: u32 = 0x0073_14c0;
/// `BGSDefaultObjectManager::GetDefaultObject` (Xbox PDB, cdecl) and the
/// form lookup by id (`004839c0`, cdecl).
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `ItemChange::GetWorn`, `ItemChange::GetFullName`,
/// `ItemChange::GetIconOne`, `ItemChange::GetModSlots` (Xbox PDB).
const ITEM_GET_WORN: u32 = 0x004b_ddd0;
const ITEM_GET_FULL_NAME: u32 = 0x004b_e2d0;
const ITEM_GET_ICON_ONE: u32 = 0x004b_e200;
const ITEM_GET_MOD_SLOTS: u32 = 0x004b_d820;
/// `Actor::QueueEquipObject` and `Actor::QueueUnEquipObject` (Xbox PDB).
const QUEUE_EQUIP: u32 = 0x0088_c650;
const QUEUE_UNEQUIP: u32 = 0x0088_c790;
/// A second setting string getter body (`004c69f0`, `ECX` = the setting).
const SETTING_STRING_004C69F0: u32 = 0x004c_69f0;
/// The setting of the message `SelectCurrentItem` shows.
const CANNOT_USE_SETTING: u32 = 0x011d_2af0;
/// The form id `SelectCurrentItem` compares with.
const FORM_ID_000CCEF2: u32 = 0x000c_cef2;
/// The setting pointers the buttons are labelled with (the code reads the
/// pointer at these addresses) and the plain setting addresses.
const DROP_LABEL_SETTING_POINTER: u32 = 0x011d_5178;
const UNEQUIP_LABEL_SETTING: u32 = 0x011d_460c;
const APPAREL_LABEL_SETTING: u32 = 0x011d_478c;
const EQUIP_LABEL_SETTING: u32 = 0x011d_3cdc;
const ITEM_BUTTONS: [u32; 5] = [0x40, 0x44, 0x4c, 0x74, 0x48];
/// The word at +0 of an object, used on a tile + 4 (`0073ac90`) and the
/// word at +4 (`007b5370`).
const WORD_AT_ZERO_0073AC90: u32 = 0x0073_ac90;
const WORD_AT_FOUR_007B5370: u32 = 0x007b_5370;
/// The repairability test's helpers: a health rounding (`00476b20`, cdecl
/// float; ST0), the simple list's constructor (`0096a2d0`), its head
/// removal and destructor, `InventoryChanges::GetInventoryChanges` (cdecl),
/// `InventoryChanges::GetRepairItemGroup` (`this`, object, list),
/// `ExtraDataList::HasWeaponMods` and the deleting destructor of a group
/// entry (`004459e0`, flag).
const HEALTH_ROUND: u32 = 0x0047_6b20;
const SIMPLE_LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const SIMPLE_LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const SIMPLE_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const GET_REPAIR_ITEM_GROUP: u32 = 0x004d_4bd0;
const HAS_WEAPON_MODS: u32 = 0x0041_8ba0;
const ITEM_DELETE: u32 = 0x0044_59e0;
/// `100.0` (`double`).
const FULL_HEALTH: u32 = 0x0101_7a40;
/// `Interface::IsInMenuMode`-style test (`00709d40`, no arguments) and
/// `HotKeysWheel::IsDPadPressed` (Xbox PDB, no arguments).
const MENU_TEST_00709D40: u32 = 0x0070_9d40;
const WHEEL_IS_DPAD_PRESSED: u32 = 0x0070_2140;
/// `InterfaceManager::PlayMenuSound`, `Interface::SetStatsMenuVisible` and
/// `Interface::SetMapMenuVisible` (Xbox PDB, cdecl).
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
const SET_STATS_MENU_VISIBLE: u32 = 0x0070_4c10;
const SET_MAP_MENU_VISIBLE: u32 = 0x0070_4170;
/// The string comparison (cdecl: two strings).
const STRING_COMPARE: u32 = 0x0040_8b20;
/// `TESBipedModelForm::GetFormAsBipedModel` (cdecl),
/// `TESBipedModelForm::GetPlayable`, `TESContainer::IsGold` (cdecl),
/// `TESForm::GetFormByEditorID` (cdecl), `BGSListForm::GetItemIndex`
/// (Xbox PDB) and the player test `004be580`.
const GET_FORM_AS_BIPED_MODEL: u32 = 0x0048_0db0;
const BIPED_GET_PLAYABLE: u32 = 0x0048_0d90;
const IS_GOLD: u32 = 0x0048_1f10;
const GET_FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
const LIST_FORM_ITEM_INDEX: u32 = 0x0058_ff60;
const PLAYER_TEST_004BE580: u32 = 0x004b_e580;
/// The editor id of the form list the last tab filters with.
const FILTER_LIST_EDITOR_ID: u32 = 0x0107_22f4;
/// `sprintf_s` (cdecl: buffer, size, format, words) and the two count
/// formats.
const SPRINTF_S: u32 = 0x0040_6d00;
const MOD_COUNT_FORMAT: u32 = 0x0107_3af4;
const COUNT_FORMAT: u32 = 0x0106_eb00;
/// The player's test whether an item can be used (`00967b70`; `this`,
/// item).
const PLAYER_CAN_USE: u32 = 0x0096_7b70;
/// `Actor::GetCurrentWeapon` and `TESObjectWEAP::GetCurrentAmmo` (Xbox PDB).
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
const GET_CURRENT_AMMO: u32 = 0x0052_5980;
/// A tile list lookup (`006317a0`, `thiscall`: word; answers the address
/// of the element).
const TILE_LIST_LOOKUP: u32 = 0x0063_17a0;

/// `Tile::GetString` (Xbox PDB; `this`, trait).
const TILE_GET_STRING: u32 = 0x00a0_11f0;

/// The element of a list node: the word at the address `006815c0` answers.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(slot)
}

/// `_ftol2_sse` on the value in ST0.
fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// The page state slot for the current tab: 5 while the keyring is open.
fn page_slot(e: &mut Engine) -> u32 {
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073984);
    let tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    if e.call(TILE_IS_TRUE, &args![tile, keyring]).bool() {
        5
    } else {
        e.mem.u32(me(e) + 0x84)
    }
}

// Translated from 0077ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::SavePageState` (Xbox PDB, no arguments): stores the
/// highlighted row (trait `0xfac` of the list's highlighted tile, -1 without
/// one) and the list's scroll position (the user trait `010702d8` of the
/// list's tile at +0x14) in the page state of the current tab (5 while the
/// keyring is open).
pub fn inventory_menu_save_page_state(e: &mut Engine) {
    let slot = page_slot(e);
    let list = me(e) + INVENTORY_LIST;
    let row = if e.vcall(list, 4, &args![]).u32() != 0 {
        let list = me(e) + INVENTORY_LIST;
        let tile = e.vcall(list, 4, &args![]).u32();
        let value = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfacu32]).f64();
        ftol(e, value)
    } else {
        -1
    };
    e.mem
        .set_i32(me(e).wrapping_add(PAGE_STATES + 8 * slot), row);
    let trait_id = e.call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME]).u32();
    let list = me(e) + INVENTORY_LIST;
    let tile = e.call(LIST_WORD_AT_0X14, &args![list]).u32();
    let value = e.call(GET_FLOAT_TRAIT, &args![tile, trait_id]).f64();
    let scroll = ftol(e, value);
    e.mem
        .set_i32(me(e).wrapping_add(PAGE_STATES + 8 * slot + 4), scroll);
}

// Translated from 007800c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::LoadPageState` (Xbox PDB, no arguments): restores the list
/// (`007832e0`) from the current tab's page state, both words as floats.
pub fn inventory_menu_load_page_state(e: &mut Engine) {
    let slot = page_slot(e);
    let state = me(e).wrapping_add(PAGE_STATES + 8 * slot);
    let scroll = x87_store32(f64::from(e.mem.i32(state + 4)));
    let row = x87_store32(f64::from(e.mem.i32(state)));
    fn_007832e0(e, Ptr::new(me(e) + INVENTORY_LIST), row, scroll);
}

// Translated from 00780be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores `value` in the byte at +0x13 of `this`.
pub fn fn_00780be0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x13, value);
}

// Translated from 00780c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): shows `tile` when its trait `0xfac` is
/// at least 0.0 and hides it otherwise (`FCOMP; TEST AH,1`: NaN hides).
pub fn fn_00780c00(e: &mut Engine, tile: u32) {
    let value = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfacu32]).f64();
    let visible = !(value < e.global::<f64>(ZERO_DOUBLE) || value.is_nan());
    e.call(
        SET_TRAIT_FROM_INT,
        &args![tile, TRAIT_VISIBLE, u32::from(visible)],
    );
}

/// The first extra data list of an item (`*006815c0(item's list)`), or 0.
fn first_extra_list(e: &mut Engine, item: u32) -> u32 {
    if e.call(WORD_AT_ZERO, &args![item]).u32() == 0 {
        return 0;
    }
    let list = e.call(WORD_AT_ZERO, &args![item]).u32();
    let node = e.call(IDENTITY, &args![list]).u32();
    e.mem.u32(node)
}

// Translated from 00780c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DropCurrentItem` (Xbox PDB, cdecl: count): drops `count`
/// of the selected item at the position `011f426c` (with its first extra
/// list when that holds at least `count`): the player's preparation
/// (`009614b0`) and its drop (vtable `+0x3cc`); with a menu, marks the
/// object for update and updates the menu. Nothing for a count of 0.
pub fn inventory_menu_drop_current_item(e: &mut Engine, count: i32) {
    e.with_stack(0xc, |e, position| {
        for i in 0..3 {
            let word = e.mem.u32(DROP_POSITION + 4 * i);
            e.mem.set_u32(position.addr() + 4 * i, word);
        }
        let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        let mut extra = first_extra_list(e, current_item(e));
        if count == 0 {
            return;
        }
        if extra != 0 && i32::from(e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16) < count {
            extra = 0;
        }
        let player = e.global::<u32>(PLAYER);
        e.call(
            PLAYER_DROP_PREPARE,
            &args![player, object, position, 1u32, 1u32],
        );
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, 0x3cc, &args![object, extra, count, position, 0u32]);
        if me(e) == 0 {
            return;
        }
        if current_item(e) != 0 {
            let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), object);
                e.call(LIST_ADD_OBJECT, &args![me(e) + ITEM_UPDATE_LIST, slot]);
            });
        }
        inventory_menu_update_inventory_menu(e);
    });
}

/// Shows the "cannot use" message of `SelectCurrentItem`.
fn show_cannot_use(e: &mut Engine) {
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
    let text = e
        .call(SETTING_STRING_004C69F0, &args![CANNOT_USE_SETTING])
        .u32();
    e.call(SHOW_MESSAGE, &args![text, 0u32, 0u32, 0u32, time, 0u32]);
}

/// A player's actor value through `this + 0xa4`'s vtable slot (`+0x8`
/// current, `+0x1c` base).
fn player_value(e: &mut Engine, slot: u32, id: u32) -> i32 {
    let player = e.global::<u32>(PLAYER);
    e.vcall(player.wrapping_add(0xa4), slot, &args![id]).i32()
}

// Translated from 00780d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::SelectCurrentItem` (Xbox PDB): sorts the list (keyring
/// open), marks the object for update, then for the default object 0 or
/// form `000ccef2` with actor value `0x10` at its base, or for default
/// object `0x15` when `00780f70` finds nothing below its base, shows the
/// "cannot use" message; otherwise equips (or, when worn, unequips) the
/// item. Updates the menu.
pub fn inventory_menu_select_current_item(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let base = this.addr();
    let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    let extra = first_extra_list(e, current_item(e));
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073974);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    if e.call(TILE_IS_TRUE, &args![tile, keyring]).bool() {
        e.call(
            LIST_SORT,
            &args![base + INVENTORY_LIST, 0x0078_32a0u32, 0u32, 0x7fff_ffffu32],
        );
    }
    let updated = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), updated);
        e.call(LIST_ADD_OBJECT, &args![base + ITEM_UPDATE_LIST, slot]);
    });
    let current = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    let special = current == e.call(GET_DEFAULT_OBJECT, &args![0u32]).u32() || {
        let current = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        current == e.call(LOOKUP_FORM_BY_ID, &args![FORM_ID_000CCEF2]).u32()
    };
    if special && player_value(e, 8, 0x10) == player_value(e, 0x1c, 0x10) {
        show_cannot_use(e);
    } else {
        let current = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        if current == e.call(GET_DEFAULT_OBJECT, &args![0x15u32]).u32() && !fn_00780f70(e) {
            show_cannot_use(e);
        } else {
            let worn = e.call(ITEM_GET_WORN, &args![current_item(e), 0u32]).bool();
            let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).u32();
            let player = e.global::<u32>(PLAYER);
            let queue = if worn { QUEUE_UNEQUIP } else { QUEUE_EQUIP };
            e.call(
                queue,
                &args![player, object, count, extra, 0u32, 0u32, 1u32],
            );
        }
    }
    inventory_menu_update_inventory_menu(e);
}

// Translated from 00780f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): whether any of the player's
/// actor values `0x19`..`0x1e` is below its base (vtable `+0x8` against
/// `+0x1c` of the player's `+0xa4`).
pub fn fn_00780f70(e: &mut Engine) -> bool {
    for id in 0x19..0x1f {
        let current = player_value(e, 8, id);
        let base = player_value(e, 0x1c, id);
        if current < base {
            return true;
        }
    }
    false
}

/// `Tile::AddNeedsUpdate` (Xbox PDB; `this`, flag).
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Interface::PopulateItemStatsDisplay` (Xbox PDB, cdecl: item, tile).
const POPULATE_ITEM_STATS: u32 = 0x0070_7e30;
/// `MiddleHighProcess::GetLastBoundWeapon` and
/// `MiddleHighProcess::GetDesiredTarget` (the engine map's names for the
/// shared getter bodies `008d85e0` and `008d8160`, used on the item's
/// object), and `ActorValue::GetItemWeaponTypeBadge` (cdecl).
const OBJECT_GETTER_008D85E0: u32 = 0x008d_85e0;
const OBJECT_GETTER_008D8160: u32 = 0x008d_8160;
const WEAPON_TYPE_BADGE: u32 = 0x0066_ea20;
/// `BGSEquipType::GetEquipType` and `BGSEquipType::GetEquipTypeIconFilename`
/// (Xbox PDB, cdecl).
const GET_EQUIP_TYPE: u32 = 0x0047_9430;
const GET_EQUIP_TYPE_ICON: u32 = 0x0047_9540;
/// `strcpy_s` and `strcat_s` (cdecl: buffer, size, text) and `strlen`.
const STRCPY_S: u32 = 0x0040_6d30;
const STRCAT_S: u32 = 0x0040_6d50;
const STRLEN: u32 = 0x0044_a670;
/// The list's highlighted item (`007a1910`, `thiscall`).
const LIST_SELECTED_ITEM: u32 = 0x007a_1910;
/// A setting's float address (`00403e20`, `ECX` = the setting).
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// `DoEnter`'s static: the last highlighted row.
const LAST_ROW: u32 = 0x011d_9f34;
/// The settings of the knob movement.
const KNOB_STEP_SETTING: u32 = 0x011d_b2f0;
const KNOB_TIME_SETTING: u32 = 0x011d_b358;
/// The health thresholds (`float`) of the repair label: weapons and others.
const HEALTH_THRESHOLD: u32 = 0x0102_f0f8;
const HEALTH_THRESHOLD_ARMOR: u32 = 0x0101_b268;
const REPAIR_LABEL_SETTING: u32 = 0x011d_3dcc;
const REPAIR_LABEL_SETTING_LOW: u32 = 0x011d_1f2c;
/// The badge format, the icon compared and the image placement floats.
const BADGE_FORMAT: u32 = 0x0102_0764;
const DEFAULT_ICON_SETTING: u32 = 0x011d_33f0;
const IMAGE_X: u32 = 0x0105_0280;
const IMAGE_SIZE: u32 = 0x0107_2e3c;
const AMMO_IMAGE_X_OFFSET: u32 = 0x0107_3af0;
const AMMO_IMAGE_SIZE_OFFSET: u32 = 0x0104_f754;
const AMMO_IMAGE_X: u32 = 0x0107_3ae8;
const AMMO_IMAGE_SIZE: u32 = 0x0107_3ae0;
/// The icon folder the equip-type icon is appended to, and the empty icon.
const ICON_FOLDER: u32 = 0x0106_eb24;
const EMPTY_ICON: u32 = 0x0107_3aac;
/// The setting pointer of the button label without an item.
const NO_ITEM_LABEL_SETTING_POINTER: u32 = 0x011d_5168;

/// Sets three float traits of the image tile (+0x54).
fn place_image(e: &mut Engine, menu: u32, x: u32, width: u32, height: u32) {
    for (trait_id, value) in [(0xfa1u32, x), (0xfb1, width), (0xfb0, height)] {
        let tile = tile_at(e, menu, 0x54);
        e.call(SET_FLOAT_TRAIT, &args![tile, trait_id, value, 1u32]);
    }
}

// Translated from 00780ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DoEnter` (Xbox PDB). Entering the list (`0x1d`) selects
/// its highlighted item: moves the Pip-Boy knob by the step setting (up or
/// down) when the row changed, labels the repair button by the item's
/// health against the threshold (armour has its own), fills the stats, the
/// can-use trait, the icon, the weapon badge and name, places the image
/// (ammunition differently; the default icon its own way), sets the
/// equip-type icon (shown when it adds to the folder) and shows the image
/// unless the wheel is open, then updates the buttons. Leaving it (`0x1e`)
/// clears the item, the icon and the label.
// Translation incomplete: 00780ff0 - the /GS cookie check (00ec408c) is not translated
pub fn inventory_menu_do_enter(e: &mut Engine, this: Ptr<InventoryMenu>, id: i32, _unused_2: u32) {
    let base = this.addr();
    if id == 0x1d {
        let item = e
            .call(LIST_SELECTED_ITEM, &args![base + INVENTORY_LIST])
            .u32();
        e.set_global(CURRENT_ITEM, item);
        if current_item(e) == 0 {
            return;
        }
        enter_item(e, this);
    } else if id == 0x1e {
        inventory_menu_clear_current_item(e, this);
        let image = tile_at(e, base, 0x54);
        e.call(SET_STRING_TRAIT, &args![image, 0xfccu32, EMPTY_ICON, 1u32]);
        if !e
            .call(WHEEL_GET_VISIBLE, &args![base + HOT_KEY_WHEEL])
            .bool()
        {
            let image = tile_at(e, base, 0x54);
            set_trait_from_int(e, image, TRAIT_VISIBLE, 1);
        }
        let setting = e.global::<u32>(NO_ITEM_LABEL_SETTING_POINTER);
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let button = tile_at(e, base, 0x40);
        e.call(SET_STRING_TRAIT, &args![button, TRAIT_STRING, text, 1u32]);
        let button = tile_at(e, base, 0x40);
        set_trait_from_int(e, button, 0xfaf, 1);
    }
}

/// [`inventory_menu_do_enter`] for a selected item.
fn enter_item(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let base = this.addr();
    let list = base + INVENTORY_LIST;
    let tile = e.vcall(list, 4, &args![]).u32();
    let value = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfacu32]).f64();
    let row = ftol(e, value);
    if e.global::<i32>(LAST_ROW) != row {
        let step = e.call(SETTING_FLOAT, &args![KNOB_STEP_SETTING]).u32();
        let step = if e.global::<i32>(LAST_ROW) < row {
            x87_quiet_bits(e.mem.u32(step))
        } else {
            x87_store32(-x87_load32(e.mem.u32(step)))
        };
        let time = e.call(SETTING_FLOAT, &args![KNOB_TIME_SETTING]).u32();
        let time = x87_quiet_bits(e.mem.u32(time));
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let pipboy = e.call(PIPBOY_MANAGER, &args![manager]).u32();
        e.call(MOVE_KNOB, &args![pipboy, 0u32, step, time, 1u32]);
        e.set_global(LAST_ROW, row);
    }
    let mut threshold = x87_quiet_bits(e.global::<u32>(HEALTH_THRESHOLD));
    if current_item_type(e) == 0x18 {
        threshold = x87_quiet_bits(e.global::<u32>(HEALTH_THRESHOLD_ARMOR));
    }
    let health = e.call(ITEM_GET_HEALTH, &args![current_item(e), 1u32]).f64();
    // FCOMPP; TEST AH,0x41; JP: the first label when the threshold is at
    // most the health (ordered).
    let threshold = x87_load32(threshold);
    let label = if threshold <= health {
        REPAIR_LABEL_SETTING
    } else {
        REPAIR_LABEL_SETTING_LOW
    };
    let text = e.call(SETTING_STRING, &args![label]).u32();
    let button = tile_at(e, base, 0x48);
    e.call(SET_STRING_TRAIT, &args![button, TRAIT_STRING, text, 1u32]);
    let stats = tile_at(e, base, 0x58);
    e.call(POPULATE_ITEM_STATS, &args![current_item(e), stats]);
    let stats = tile_at(e, base, 0x58);
    set_trait_from_int(e, stats, TRAIT_VISIBLE, 1);
    let player = e.global::<u32>(PLAYER);
    let usable = e
        .call(PLAYER_CAN_USE, &args![player, current_item(e)])
        .bool();
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073974);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, keyring, u32::from(usable)]);
    let icon = e
        .call(ITEM_GET_ICON_ONE, &args![current_item(e), 0u32])
        .u32();
    let image = tile_at(e, base, 0x54);
    e.call(SET_STRING_TRAIT, &args![image, 0xfccu32, icon, 1u32]);
    if current_item_type(e) == 0x28 {
        let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        let weapon = e.call(OBJECT_GETTER_008D85E0, &args![object]).u32();
        let badge = e.call(WEAPON_TYPE_BADGE, &args![weapon]).u32();
        let card = tile_at(e, base, 0x78);
        e.call(SET_STRING_TRAIT, &args![card, 0xfccu32, badge, 1u32]);
        let name = e.call(OBJECT_GETTER_008D8160, &args![object]).u32();
        e.with_stack(0x80, |e, buffer| {
            e.call(SPRINTF_S, &args![buffer, 0x80u32, BADGE_FORMAT, name]);
            if tile_at(e, base, 0x80) != 0 {
                let tile = tile_at(e, base, 0x80);
                e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, buffer, 1u32]);
            }
        });
    }
    let default_icon = e.call(SETTING_STRING, &args![DEFAULT_ICON_SETTING]).u32();
    let image = tile_at(e, base, 0x54);
    let shown = e.call(TILE_GET_STRING, &args![image, 0xfccu32]).u32();
    let same = e.call(STRING_COMPARE, &args![shown, default_icon]).i32() == 0;
    let image_x = x87_quiet_bits(e.global::<u32>(IMAGE_X));
    let image_size = x87_quiet_bits(e.global::<u32>(IMAGE_SIZE));
    if same {
        place_image(e, base, image_x, image_size, image_size);
    } else if current_item_type(e) == 0x29 {
        let x_offset = x87_quiet_bits(e.global::<u32>(AMMO_IMAGE_X_OFFSET));
        let size_offset = x87_quiet_bits(e.global::<u32>(AMMO_IMAGE_SIZE_OFFSET));
        let x = x87_store32(x87_add(x87_load32(x_offset), e.global::<f64>(AMMO_IMAGE_X)));
        let size = x87_store32(x87_add(
            x87_load32(size_offset),
            e.global::<f64>(AMMO_IMAGE_SIZE),
        ));
        place_image(e, base, x, size, size);
    } else {
        place_image(e, base, image_x, image_size, image_size);
    }
    let image = tile_at(e, base, 0x54);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![image, 0x11u32]);
    e.with_stack(0x104, |e, path| {
        e.call(STRCPY_S, &args![path, 0x104u32, ICON_FOLDER]);
        let folder_length = e.call(STRLEN, &args![path]).u32();
        let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        let equip_type = e.call(GET_EQUIP_TYPE, &args![object]).u32();
        let icon = e.call(GET_EQUIP_TYPE_ICON, &args![equip_type]).u32();
        e.call(STRCAT_S, &args![path, 0x104u32, icon]);
        let equip_tile = |e: &mut Engine| {
            let tile = tile_at(e, base, 0x3c);
            let slot = e
                .call(WORD_AT_ZERO_0073AC90, &args![tile.wrapping_add(4)])
                .u32();
            e.mem.u32(slot)
        };
        if e.call(STRLEN, &args![path]).u32() > folder_length {
            let tile = equip_tile(e);
            e.call(SET_STRING_TRAIT, &args![tile, 0xfccu32, path, 1u32]);
            let tile = equip_tile(e);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 1);
        } else {
            let tile = equip_tile(e);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        }
    });
    if !e
        .call(WHEEL_GET_VISIBLE, &args![base + HOT_KEY_WHEEL])
        .bool()
    {
        let image = tile_at(e, base, 0x54);
        set_trait_from_int(e, image, TRAIT_VISIBLE, 1);
    }
    inventory_menu_update_buttons(e, this);
}

// Translated from 00781620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DoLeave` (Xbox PDB): leaving the list (`0x1d` or `0x1e`)
/// clears the selected item (unless a gamepad is in use with menu `0x3f8` on
/// top) and clears the menu tile's can-use trait.
pub fn inventory_menu_do_leave(e: &mut Engine, this: Ptr<InventoryMenu>, id: i32, _unused_2: u32) {
    if id != 0x1d && id != 0x1e {
        return;
    }
    let keep =
        e.call(GAMEPAD_IN_USE, &args![]).bool() && e.call(GET_TOP_MENU_ID, &args![]).u32() == 0x3f8;
    if !keep {
        inventory_menu_clear_current_item(e, this);
    }
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073974);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, keyring, 0u32]);
}

// Translated from 00781680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::UpdateButtons` (Xbox PDB): with a selected item, labels
/// the first button (drop label for type `0x19`, unequip label when worn),
/// enables it like the menu's can-use trait, enables the second and fourth,
/// the third unless the item is of type `0x29`, and the repair button when
/// the item is repairable. Without one, labels the first button by the tab
/// (apparel tab `0x1a` or equip) and disables all five.
pub fn inventory_menu_update_buttons(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let base = this.addr();
    if current_item(e) != 0 {
        let label = if current_item_type(e) == 0x19 {
            Some(e.global::<u32>(DROP_LABEL_SETTING_POINTER))
        } else if e.call(ITEM_GET_WORN, &args![current_item(e), 0u32]).bool() {
            Some(UNEQUIP_LABEL_SETTING)
        } else {
            None
        };
        if let Some(setting) = label {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            let button = tile_at(e, base, 0x40);
            e.call(SET_STRING_TRAIT, &args![button, TRAIT_STRING, text, 1u32]);
        }
        let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073974);
        let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        let usable = e.call(GET_FLOAT_TRAIT, &args![tile, keyring]).f64();
        let button = tile_at(e, base, 0x40);
        e.call(
            SET_FLOAT_TRAIT,
            &args![button, 0xfafu32, x87_store32(usable), 1u32],
        );
        let button = tile_at(e, base, 0x44);
        set_trait_from_int(e, button, 0xfaf, 1);
        let not_ammo = current_item_type(e) != 0x29;
        let button = tile_at(e, base, 0x4c);
        set_trait_from_int(e, button, 0xfaf, i32::from(not_ammo));
        let button = tile_at(e, base, 0x74);
        set_trait_from_int(e, button, 0xfaf, 1);
        let repairable = inventory_menu_is_item_repairable(e, this, current_item(e));
        let button = tile_at(e, base, 0x48);
        set_trait_from_int(e, button, 0xfaf, i32::from(repairable));
    } else {
        let tab = e.mem.u32(me(e) + 0x84).wrapping_add(0x18);
        let setting = if tab == 0x1a {
            APPAREL_LABEL_SETTING
        } else {
            EQUIP_LABEL_SETTING
        };
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let button = tile_at(e, base, 0x40);
        e.call(SET_STRING_TRAIT, &args![button, TRAIT_STRING, text, 1u32]);
        for offset in ITEM_BUTTONS {
            let button = tile_at(e, base, offset);
            set_trait_from_int(e, button, 0xfaf, 0);
        }
    }
}

/// `00476b20` on an item's health (stored as a `float` first; ST0).
fn rounded_health(e: &mut Engine, item: u32) -> f64 {
    let health = e.call(ITEM_GET_HEALTH, &args![item, 1u32]).f64();
    e.call(HEALTH_ROUND, &args![x87_store32(health)]).f64()
}

// Translated from 00781860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::IsItemRepairable` (Xbox PDB): an item (weapon `0x28` or
/// armour `0x18`) below full health is repairable when it is a stack of
/// more than one, or when the player holds another unworn item of its
/// repair group (`GetRepairItemGroup`): a different object, or the same
/// object below full health with another extra list that has no weapon
/// mods. The group entries are deleted afterwards. False at full health
/// (rounded) or without an item or object.
// Translation incomplete: 00781860 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn inventory_menu_is_item_repairable(
    e: &mut Engine,
    _this: Ptr<InventoryMenu>,
    item: u32,
) -> bool {
    let full = e.global::<f64>(FULL_HEALTH);
    let object = if item != 0 {
        e.call(ITEM_OBJECT, &args![item]).u32()
    } else {
        0
    };
    if item != 0 && rounded_health(e, item) == full {
        return false;
    }
    if item == 0 || object == 0 {
        return false;
    }
    let kind = e.call(FORM_TYPE, &args![object]).u32();
    if kind != 0x28 && e.call(FORM_TYPE, &args![object]).u32() != 0x18 {
        return false;
    }
    let health = e.call(ITEM_GET_HEALTH, &args![item, 1u32]).f64();
    // FCOMP; TEST AH,5; JP: on only when below (ordered).
    if !(health < full) {
        return false;
    }
    if e.call(WORD_AT_FOUR, &args![item]).i32() > 1 {
        return true;
    }
    e.with_stack(8, |e, group| {
        e.call(SIMPLE_LIST_CONSTRUCTOR, &args![group]);
        let player = e.global::<u32>(PLAYER);
        let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
        let own_extra = first_extra_list(e, item);
        e.call(GET_REPAIR_ITEM_GROUP, &args![changes, object, group]);
        let mut repairable = false;
        let mut node = group.addr();
        while node != 0 {
            let entry_slot = e.call(IDENTITY, &args![node]).u32();
            if e.mem.u32(entry_slot) == 0 {
                break;
            }
            let entry = node_item(e, node);
            let other = e.call(ITEM_OBJECT, &args![entry]).u32();
            let entry = node_item(e, node);
            let other_extra = first_extra_list(e, entry);
            let entry = node_item(e, node);
            if !e.call(ITEM_GET_WORN, &args![entry, 0u32]).bool() {
                if other != object {
                    repairable = true;
                } else {
                    let entry = node_item(e, node);
                    if rounded_health(e, entry) < full
                        && other_extra != own_extra
                        && other_extra != 0
                        && !e.call(HAS_WEAPON_MODS, &args![other_extra]).bool()
                    {
                        repairable = true;
                    }
                }
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        let head = group.addr();
        loop {
            let slot = e.call(IDENTITY, &args![head]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let entry = node_item(e, head);
            if entry != 0 {
                e.call(ITEM_DELETE, &args![entry, 1u32]);
            }
            e.call(SIMPLE_LIST_REMOVE_HEAD, &args![head]);
        }
        e.call(SIMPLE_LIST_DESTRUCTOR, &args![group]);
        repairable
    })
}

// Translated from 00781b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::ClearCurrentItem` (Xbox PDB): clears `pCurrItem`, hides
/// the stats tile, the image, the badge, the name and the equip-type icon,
/// and updates the buttons.
pub fn inventory_menu_clear_current_item(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let base = this.addr();
    e.set_global(CURRENT_ITEM, 0u32);
    let stats = tile_at(e, base, 0x58);
    set_trait_from_int(e, stats, TRAIT_VISIBLE, 0);
    for offset in [0x54u32, 0x78] {
        let tile = tile_at(e, base, offset);
        e.call(
            SET_FLOAT_TRAIT,
            &args![tile, TRAIT_VISIBLE, 0.0f32.to_bits(), 0u32],
        );
    }
    let name = tile_at(e, base, 0x80);
    set_trait_from_int(e, name, TRAIT_VISIBLE, 0);
    let tile = tile_at(e, base, 0x3c);
    let slot = e
        .call(WORD_AT_ZERO_0073AC90, &args![tile.wrapping_add(4)])
        .u32();
    let icon = e.mem.u32(slot);
    set_trait_from_int(e, icon, TRAIT_VISIBLE, 0);
    inventory_menu_update_buttons(e, this);
}

// Translated from 00782190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DoGamepad` (Xbox PDB): only as the top menu with the
/// Pip-Boy ready. A d-pad press (buttons 1..4) with the wheel open (and
/// `00709d40`) is the wheel's. Otherwise: buttons 3 and 4 switch the tab
/// right or left (wrapping 0..4: `DoLeave(0x1d)`, `DoClick(tab + 0x18)`,
/// the wheel kept open), 13 and 14 open the stats and map menus (sound
/// `0x24`), 15 clicks `0x13`, 16 opens the wheel when the third button is
/// enabled. Answers whether it handled the button.
pub fn inventory_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<InventoryMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    let base = this.addr();
    if !e.call(IS_TOP_MENU_ID, &args![INVENTORY_MENU_CLASS]).bool() {
        return false;
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_READY, &args![pipboy]).bool() {
        return false;
    }
    let wheel = base + HOT_KEY_WHEEL;
    let mut handled = e.call(MENU_TEST_00709D40, &args![]).bool()
        && e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool()
        && button > 0
        && button <= 4
        && e.call(WHEEL_IS_DPAD_PRESSED, &args![]).bool();
    if handled || !e.call(IS_TOP_MENU_ID, &args![INVENTORY_MENU_CLASS]).bool() {
        return handled;
    }
    match button {
        3 | 4 => {
            let mut tab = e.get(this, InventoryMenu::iCurrTab);
            let wheel_open = e.call(WHEEL_GET_VISIBLE, &args![wheel]).bool();
            e.vcall(base, 0x14, &args![0x1du32, 0u32]);
            tab = tab.wrapping_add(if button != 4 { 1 } else { -1 });
            if tab > 4 {
                tab = 0;
            }
            if tab < 0 {
                tab = 4;
            }
            e.vcall(base, 0xc, &args![tab.wrapping_add(0x18), 0u32]);
            if wheel_open {
                e.call(WHEEL_SET_VISIBLE, &args![wheel, 1u32]);
            }
            handled = true;
        }
        0xd => {
            e.call(PLAY_MENU_SOUND, &args![0x24u32]);
            e.call(SET_STATS_MENU_VISIBLE, &args![1u32, 0u32]);
            handled = true;
        }
        0xe => {
            e.call(PLAY_MENU_SOUND, &args![0x24u32]);
            e.call(SET_MAP_MENU_VISIBLE, &args![1u32, 0u32]);
            handled = true;
        }
        0xf => {
            e.vcall(base, 0xc, &args![0x13u32, 0u32]);
            handled = true;
        }
        0x10 => {
            let button_tile = tile_at(e, base, 0x4c);
            let enabled = e.call(GET_FLOAT_TRAIT, &args![button_tile, 0xfafu32]).f64();
            // FCOMP; TEST AH,0x44; JNP: skipped only when equal to 0.0.
            if enabled != e.global::<f64>(ZERO_DOUBLE) {
                e.call(WHEEL_SET_VISIBLE, &args![wheel, 1u32]);
                let image = tile_at(e, base, 0x54);
                set_trait_from_int(e, image, TRAIT_VISIBLE, 0);
            }
            handled = true;
        }
        _ => {}
    }
    handled
}

// Translated from 007823d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DoGamepadUpEvent` (Xbox PDB): as the top menu, button
/// `0xa` clicks `0xa`; button `0x10` closes the wheel and shows the image
/// when the list has a highlighted tile. Answers whether it handled it.
pub fn inventory_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr<InventoryMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    let base = this.addr();
    if !e.call(IS_TOP_MENU_ID, &args![INVENTORY_MENU_CLASS]).bool() {
        return false;
    }
    match button {
        0xa => {
            e.vcall(base, 0xc, &args![0xau32, 0u32]);
            true
        }
        0x10 => {
            e.call(WHEEL_SET_VISIBLE, &args![base + HOT_KEY_WHEEL, 0u32]);
            if e.vcall(base + INVENTORY_LIST, 4, &args![]).u32() != 0 {
                let image = tile_at(e, base, 0x54);
                set_trait_from_int(e, image, TRAIT_VISIBLE, 1);
            }
            true
        }
        _ => false,
    }
}

// Translated from 00782470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: flag): with a menu, sets the Pip-Boy
/// tab knob to the current tab (`FOPipboyManager::SetTabKnobPosition`).
pub fn fn_00782470(e: &mut Engine, flag: u8) {
    if me(e) == 0 {
        return;
    }
    let tab = e.mem.u32(me(e) + 0x84);
    let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let pipboy = e.call(PIPBOY_MANAGER, &args![manager]).u32();
    e.call(SET_TAB_KNOB_POSITION, &args![pipboy, tab, u32::from(flag)]);
}

// Translated from 007824e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::SortFunc` (Xbox PDB, cdecl: two list entries, the item at
/// +4): entries without an item last; by full name; equal names by health
/// (higher first) and then worn first.
pub fn inventory_menu_sort_func(e: &mut Engine, a: u32, b: u32) -> i32 {
    if e.mem.u32(a + 4) == 0 {
        return 1;
    }
    if e.mem.u32(b + 4) == 0 {
        return -1;
    }
    let compare_names = |e: &mut Engine| {
        let second = e.call(ITEM_GET_FULL_NAME, &args![e.mem.u32(b + 4)]).u32();
        let first = e.call(ITEM_GET_FULL_NAME, &args![e.mem.u32(a + 4)]).u32();
        e.call(STRING_COMPARE, &args![first, second]).i32()
    };
    if compare_names(e) != 0 {
        return compare_names(e);
    }
    let health_a = e
        .call(ITEM_GET_HEALTH, &args![e.mem.u32(a + 4), 0u32])
        .f64();
    let health_b = e
        .call(ITEM_GET_HEALTH, &args![e.mem.u32(b + 4), 0u32])
        .f64();
    // FCOMP (b against a); TEST AH,5; JP: -1 when b is below a.
    let mut result = if health_b < health_a {
        -1
    } else {
        let health_a = e
            .call(ITEM_GET_HEALTH, &args![e.mem.u32(a + 4), 0u32])
            .f64();
        let health_b = e
            .call(ITEM_GET_HEALTH, &args![e.mem.u32(b + 4), 0u32])
            .f64();
        // TEST AH,0x41; JNZ: 1 only when b is above a.
        i32::from(health_b > health_a)
    };
    if result == 0 {
        if e.call(ITEM_GET_WORN, &args![e.mem.u32(a + 4), 0u32]).bool() {
            result = -1;
        } else if e.call(ITEM_GET_WORN, &args![e.mem.u32(b + 4), 0u32]).bool() {
            result = 1;
        }
    }
    result
}

// Translated from 00782620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::FilterFunc` (Xbox PDB, cdecl: item): whether the item is
/// left out of the current tab (always without a menu). An empty entry is
/// left out except on tab 3. Unplayable biped models, gold, items whose
/// vtable `+0x94` holds unless the player test `004be580` passes, and type
/// `0x2e` are left out. Then by tab: 0 weapons (`0x28`), 1 apparel (`0x18`,
/// `0x1a`), 2 aid (`0x1d`, `0x2f`, `0x19`), 3 everything else, 4 ammunition
/// (`0x29`) in the form list `010722f4`; another tab leaves it out.
pub fn inventory_menu_filter_func(e: &mut Engine, item: u32) -> bool {
    if me(e) == 0 {
        return true;
    }
    let object = if item != 0 {
        e.call(ITEM_OBJECT, &args![item]).u32()
    } else {
        0
    };
    let tab = e.mem.u32(me(e) + 0x84).wrapping_add(0x18);
    if object == 0 {
        return tab != 0x1b;
    }
    let kind = e.call(FORM_TYPE, &args![object]).u32();
    let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![object]).u32();
    if biped != 0 && !e.call(BIPED_GET_PLAYABLE, &args![biped]).bool() {
        return true;
    }
    if e.call(IS_GOLD, &args![object]).bool() {
        return true;
    }
    if e.vcall(object, 0x94, &args![]).bool() {
        let player = e.global::<u32>(PLAYER);
        if !e.call(PLAYER_TEST_004BE580, &args![player]).bool() {
            return true;
        }
    }
    if kind == 0x2e {
        return true;
    }
    match tab.wrapping_sub(0x18) {
        0 => kind != 0x28,
        1 => !(kind == 0x18 || kind == 0x1a),
        2 => !(kind == 0x1d || kind == 0x2f || kind == 0x19),
        3 => [0x28, 0x18, 0x1a, 0x1d, 0x2f, 0x29, 0x19].contains(&kind),
        4 => {
            if kind != 0x29 {
                return true;
            }
            let list = e
                .call(GET_FORM_BY_EDITOR_ID, &args![FILTER_LIST_EDITOR_ID])
                .u32();
            list != 0 && e.call(LIST_FORM_ITEM_INDEX, &args![list, object]).i32() >= 0
        }
        _ => true,
    }
}

// Translated from 00782810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: item): true without an item, else
/// whether its object's type is not `0x2e`.
pub fn fn_00782810(e: &mut Engine, item: u32) -> bool {
    if item == 0 {
        return true;
    }
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    e.call(FORM_TYPE, &args![object]).u32() != 0x2e
}

/// Sets the visibility (`0xfa3`) of the list entry's tile through
/// `006317a0` (the tile's word at +4).
fn set_entry_flag(e: &mut Engine, tile: u32, flag: u32) {
    let word = e.call(WORD_AT_ZERO, &args![tile.wrapping_add(4)]).u32();
    let slot = e
        .call(TILE_LIST_LOOKUP, &args![tile.wrapping_add(4), word])
        .u32();
    let target = e.mem.u32(slot);
    e.call(SET_TRAIT_FROM_INT, &args![target, TRAIT_VISIBLE, flag]);
}

// Translated from 00782850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::InitFunc` (Xbox PDB, cdecl: tile, item): names a stack of
/// more than one with its count (`01073af4` with mod slots, else
/// `0106eb00`), sets the trait `0xfaa` to `0x1d`, dims weapons, armour and
/// ammunition the player cannot use (`0xfa9` 0x7f, else 0xff), and marks
/// the equipped ammunition (or a worn item) on the entry.
// Translation incomplete: 00782850 - the /GS cookie check (00ec408c) is not translated
pub fn inventory_menu_init_func(e: &mut Engine, tile: u32, item: u32) {
    if item == 0 {
        return;
    }
    if e.call(WORD_AT_FOUR, &args![item]).i32() > 1 {
        e.with_stack(0x80, |e, buffer| {
            let format = if e.call(ITEM_GET_MOD_SLOTS, &args![item]).bool() {
                MOD_COUNT_FORMAT
            } else {
                COUNT_FORMAT
            };
            let count = e.call(WORD_AT_FOUR, &args![item]).u32();
            let name = e.call(ITEM_GET_FULL_NAME, &args![item]).u32();
            e.call(SPRINTF_S, &args![buffer, 0x80u32, format, name, count]);
            e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, buffer, 1u32]);
        });
    }
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfaau32, 0x1du32]);
    let item_type = |e: &mut Engine| {
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        e.call(FORM_TYPE, &args![object]).u32()
    };
    if item_type(e) == 0x29 || item_type(e) == 0x18 || item_type(e) == 0x28 {
        let player = e.global::<u32>(PLAYER);
        let usable = e.call(PLAYER_CAN_USE, &args![player, item]).bool();
        let slot = e
            .call(WORD_AT_FOUR_007B5370, &args![tile.wrapping_add(4)])
            .u32();
        let target = e.mem.u32(slot);
        let alpha = if usable { 0xffu32 } else { 0x7f };
        e.call(SET_TRAIT_FROM_INT, &args![target, 0xfa9u32, alpha]);
    }
    let ammo = item_type(e) == 0x29 && {
        let player = e.global::<u32>(PLAYER);
        e.call(GET_CURRENT_WEAPON, &args![player]).u32() != 0
    };
    let flag = if ammo {
        item != 0 && {
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            let player = e.global::<u32>(PLAYER);
            let weapon = e.call(GET_CURRENT_WEAPON, &args![player]).u32();
            object == e.call(GET_CURRENT_AMMO, &args![weapon, player]).u32()
        }
    } else {
        item != 0 && e.call(ITEM_GET_WORN, &args![item, 0u32]).bool()
    };
    set_entry_flag(e, tile, u32::from(flag));
}

// Translated from 007832e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` = the inventory list): stores the two
/// floats at +0x24 and +0x28 and restores the list state
/// (`ListBox<ItemChange *>::RestoreListState(1)`).
pub fn fn_007832e0(e: &mut Engine, this: Ptr, row: u32, scroll: u32) {
    e.mem.set_u32(this.addr() + 0x24, x87_quiet_bits(row));
    e.mem.set_u32(this.addr() + 0x28, x87_quiet_bits(scroll));
    e.call(LIST_RESTORE_STATE, &args![this, 1u32]);
}

/// The rounding of a float to an integer (`00406d90`, cdecl float).
const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// The settings and formats of the status line.
const HEALTH_CAP_SETTING: u32 = 0x011d_1020;
const WEIGHT_FORMAT: u32 = 0x0107_3b20;
const CAPS_LIMIT_SETTING: u32 = 0x011d_3a08;
const CAPS_LIMIT_FORMAT: u32 = 0x0107_3b1c;
const PAIR_FORMAT: u32 = 0x0107_3b14;
/// `MiddleHighProcess::GetSavedAcquireObject` (the engine map's name for
/// the getter body `008d8520`, on the player).
const PLAYER_GETTER_008D8520: u32 = 0x008d_8520;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB, cdecl).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `Actor::GetGoldAmount` (Xbox PDB) and the player's float `008a0c20`.
const GET_GOLD_AMOUNT: u32 = 0x0089_1d70;
const PLAYER_VALUE_008A0C20: u32 = 0x008a_0c20;
/// The language setting and the string it is compared with; the labels set
/// when they differ.
const LANGUAGE_SETTING: u32 = 0x011c_3cb4;
const LANGUAGE_DEFAULT: u32 = 0x0101_8348;
const LABEL_TRAIT_NAME: u32 = 0x0106_ed20;
const LABEL_CHILD_NAME: u32 = 0x0107_3b00;
/// `Tile::GetChildByName` (Xbox PDB; `this`, name).
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// Whether the update list is empty (`008256d0`), its clear (`00470470`).
const UPDATE_LIST_IS_EMPTY: u32 = 0x0082_56d0;
const UPDATE_LIST_CLEAR: u32 = 0x0047_0470;
/// `InventoryChanges::GetItemGroup` (Xbox PDB; `this`, object, list), the
/// lookup of an object type (`004c6ba0`; `this`, type, address).
const GET_ITEM_GROUP: u32 = 0x004d_4b00;
const CHANGES_FIND_TYPE: u32 = 0x004c_6ba0;
/// `ListBox<ItemChange *>`'s `RemoveItemGroup`, `AddItem`, `Synchronize`,
/// `Filter` (Xbox PDB) and its fill from an inventory (`00719ef0`).
const LIST_REMOVE_ITEM_GROUP: u32 = 0x0071_a200;
const LIST_ADD_ITEM: u32 = 0x0071_a320;
const LIST_SYNCHRONIZE: u32 = 0x0071_a670;
const LIST_FILTER: u32 = 0x0073_0bb0;
const LIST_FILL: u32 = 0x0071_9ef0;
/// `__RTDynamicCast` (cdecl) with its two type descriptors, and
/// `MapMarkerData::GetLocationName` (the engine map's name for the shared
/// name getter `00408da0`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const NAME_CAST_SOURCE: u32 = 0x0118_3108;
const NAME_CAST_TARGET: u32 = 0x0118_3158;
const GET_NAME: u32 = 0x0040_8da0;
/// The suffix of a modded item's name.
const MODDED_SUFFIX: u32 = 0x0103_45e8;
/// The setting of the extra entry for type `0x2e` items.
const EXTRA_ENTRY_SETTING: u32 = 0x011d_3b20;

/// `sprintf_s(buffer, 0x40, format, words)` then the string trait
/// `011d9eb4` of the tile at `offset` of the menu.
fn status_line(e: &mut Engine, buffer: Ptr, offset: u32, format: u32, words: &[u32]) {
    let mut call = vec![buffer.addr(), 0x40, format];
    call.extend_from_slice(words);
    e.call(SPRINTF_S, &call);
    let trait_id = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
    let tile = tile_at(e, me(e), offset);
    e.call(SET_STRING_TRAIT, &args![tile, trait_id, buffer, 1u32]);
}

// Translated from 00782a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::UpdateInventoryMenu` (Xbox PDB, no arguments): with a
/// menu and the Pip-Boy ready, writes the status line (health capped by the
/// setting, carried weight with entry point `0x38`, caps up to the limit,
/// two actor-value pairs), relabels the buttons for another language,
/// hides the tile at +0x3c, clears the keyring trait and leaves the list.
/// With objects to update, saves the page, rebuilds their item groups in
/// the list (named by location name, modded ones with a suffix) and sorts;
/// otherwise refills the list from the player (an extra entry for type
/// `0x2e` items), filters and sorts it. Clears the item and loads the page.
// Translation incomplete: 00782a90 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
pub fn inventory_menu_update_inventory_menu(e: &mut Engine) {
    if me(e) == 0 {
        return;
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_READY, &args![pipboy]).bool() {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    e.with_stack(0x40, |e, buffer| {
        write_status_line(e, buffer);
    });
    let language = e.call(SETTING_STRING, &args![LANGUAGE_SETTING]).u32();
    if e.call(STRING_COMPARE, &args![language, LANGUAGE_DEFAULT])
        .i32()
        != 0
    {
        relabel(e);
    }
    let tile = tile_at(e, me(e), 0x3c);
    set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073984);
    let tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, keyring, 0u32]);
    e.vcall(me(e), 0x14, &args![0x1du32, 0u32]);
    let updates = me(e) + ITEM_UPDATE_LIST;
    if !e.call(UPDATE_LIST_IS_EMPTY, &args![updates]).bool() && changes != 0 {
        inventory_menu_save_page_state(e);
        e.vcall(me(e) + INVENTORY_LIST, 0, &args![0u32]);
        let mut node = me(e) + ITEM_UPDATE_LIST;
        while node != 0 {
            let slot = e.call(IDENTITY, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            rebuild_group(e, changes, node);
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        let list = me(e) + INVENTORY_LIST;
        e.call(
            LIST_SORT,
            &args![list, 0x0078_2850u32, 0u32, 0x7fff_ffffu32],
        );
        e.call(LIST_SYNCHRONIZE, &args![me(e) + INVENTORY_LIST]);
        let list = me(e) + INVENTORY_LIST;
        e.call(
            LIST_SORT,
            &args![list, 0x0078_0c00u32, -1i32, 0x7fff_ffffu32],
        );
        e.call(UPDATE_LIST_CLEAR, &args![me(e) + ITEM_UPDATE_LIST]);
    } else {
        let list = me(e) + INVENTORY_LIST;
        let player = e.global::<u32>(PLAYER);
        e.call(
            LIST_FILL,
            &args![list, player, 0u32, 0u32, 0x0078_2850u32, 0u32, 0u32],
        );
        let found = changes != 0
            && e.with_stack(4, |e, slot| {
                e.call(CHANGES_FIND_TYPE, &args![changes, 0x2eu32, slot])
                    .u32()
                    != 0
            });
        if found {
            let text = e.call(SETTING_STRING, &args![EXTRA_ENTRY_SETTING]).u32();
            let list = me(e) + INVENTORY_LIST;
            let entry = e
                .call(LIST_ADD_ITEM, &args![list, 0u32, text, 0u32, 0u32])
                .u32();
            e.call(SET_TRAIT_FROM_INT, &args![entry, 0xfaau32, 0x1eu32]);
        }
        e.call(LIST_FILTER, &args![me(e) + INVENTORY_LIST, 0x0078_2620u32]);
        let list = me(e) + INVENTORY_LIST;
        e.call(
            LIST_SORT,
            &args![list, 0x0078_0c00u32, -1i32, 0x7fff_ffffu32],
        );
        e.vcall(me(e) + INVENTORY_LIST, 0x20, &args![0x0078_24e0u32]);
    }
    inventory_menu_clear_current_item(e, Ptr::new(me(e)));
    inventory_menu_load_page_state(e);
}

/// The status line of [`inventory_menu_update_inventory_menu`].
fn write_status_line(e: &mut Engine, buffer: Ptr) {
    let health = player_float_value(e, 0x12);
    let health = e.call(FLOAT_TO_INT, &args![x87_store32(health)]).i32();
    let cap = e.call(SETTING_FLOAT, &args![HEALTH_CAP_SETTING]).u32();
    let cap = ftol(e, x87_load32(e.mem.u32(cap)));
    let health = if health > cap { cap } else { health };
    status_line(e, buffer, 0x30, BADGE_FORMAT, &[health as u32]);
    let weight = player_float_value(e, 0x4c);
    let weight = e.call(FLOAT_TO_INT, &args![x87_store32(weight)]).i32();
    let weight = x87_store32(f64::from(weight));
    let player = e.global::<u32>(PLAYER);
    let process = e.call(PLAYER_GETTER_008D8520, &args![player]).u32();
    let held = e.vcall(process, 0x148, &args![]).u32();
    let form = if held != 0 {
        e.call(ITEM_OBJECT, &args![held]).u32()
    } else {
        e.call(LOOKUP_FORM_BY_ID, &args![0x1f4u32]).u32()
    };
    let bonus = e.with_stack(4, |e, bonus| {
        e.mem.set_u32(bonus.addr(), 0.0f32.to_bits());
        let player = e.global::<u32>(PLAYER);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![0x38u32, player, player, form, bonus],
        );
        e.mem.u32(bonus.addr())
    });
    let total = x87_add(x87_load32(weight), x87_m32(bonus));
    status_line(e, buffer, 0x7c, WEIGHT_FORMAT, &args![total]);
    let player = e.global::<u32>(PLAYER);
    let gold = e.call(GET_GOLD_AMOUNT, &args![player]).i32();
    let limit = e.call(SETTING_VALUE_WORD, &args![CAPS_LIMIT_SETTING]).u32();
    if gold > e.mem.i32(limit) {
        let limit = e.call(SETTING_VALUE_WORD, &args![CAPS_LIMIT_SETTING]).u32();
        let limit = e.mem.u32(limit);
        status_line(e, buffer, 0x28, CAPS_LIMIT_FORMAT, &[limit]);
    } else {
        let player = e.global::<u32>(PLAYER);
        let gold = e.call(GET_GOLD_AMOUNT, &args![player]).u32();
        status_line(e, buffer, 0x28, BADGE_FORMAT, &[gold]);
    }
    let player = e.global::<u32>(PLAYER);
    let value = e.call(PLAYER_VALUE_008A0C20, &args![player]).f64();
    let value = ftol(e, value);
    let current = player_value(e, 8, 0x2e);
    status_line(
        e,
        buffer,
        0x34,
        PAIR_FORMAT,
        &[current as u32, value as u32],
    );
    let base = player_value(e, 0x1c, 0x10);
    let current = player_value(e, 8, 0x10);
    status_line(e, buffer, 0x2c, PAIR_FORMAT, &[current as u32, base as u32]);
}

/// A player's actor value as a float (vtable `+0xc` of the player's
/// `+0xa4`; ST0).
fn player_float_value(e: &mut Engine, id: u32) -> f64 {
    let player = e.global::<u32>(PLAYER);
    e.vcall(player.wrapping_add(0xa4), 0xc, &args![id]).f64()
}

/// The labels [`inventory_menu_update_inventory_menu`] sets for another
/// language.
fn relabel(e: &mut Engine) {
    let label = e.call(TILE_TEXT_TO_TRAIT, &args![LABEL_TRAIT_NAME]).u32();
    for (offset, setting) in [
        (0x44u32, 0x011d_2a18u32),
        (0x48, 0x011d_4f60),
        (0x4c, 0x011d_35d0),
        (0x74, 0x011d_2eb0),
    ] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = tile_at(e, me(e), offset);
        e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, text, 1u32]);
    }
    let text = e.call(SETTING_STRING, &args![0x011d_4378u32]).u32();
    let menu_tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    let child = e
        .call(TILE_GET_CHILD_BY_NAME, &args![menu_tile, LABEL_CHILD_NAME])
        .u32();
    e.call(SET_STRING_TRAIT, &args![child, TRAIT_STRING, text, 1u32]);
    for (offset, setting) in [
        (0x30u32, 0x011d_20c4u32),
        (0x7c, 0x011d_396c),
        (0x28, 0x011d_25b0),
        (0x34, 0x011d_1e60),
        (0x2c, 0x011d_300c),
    ] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = tile_at(e, me(e), offset);
        e.call(SET_STRING_TRAIT, &args![tile, label, text, 1u32]);
    }
}

/// Rebuilds the list entries of the object at `node` of the update list.
fn rebuild_group(e: &mut Engine, changes: u32, node: u32) {
    e.with_stack(8, |e, group| {
        e.call(SIMPLE_LIST_CONSTRUCTOR, &args![group]);
        let object = node_item(e, node);
        e.call(GET_ITEM_GROUP, &args![changes, object, group]);
        let object = node_item(e, node);
        e.call(
            LIST_REMOVE_ITEM_GROUP,
            &args![me(e) + INVENTORY_LIST, object],
        );
        let mut entry_node = group.addr();
        while entry_node != 0 {
            let slot = e.call(IDENTITY, &args![entry_node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let item = node_item(e, entry_node);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            let named = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![object, 0u32, NAME_CAST_SOURCE, NAME_CAST_TARGET, 0u32],
                )
                .u32();
            let name = e.call(GET_NAME, &args![named]).u32();
            e.call(STRLEN, &args![name]);
            e.with_stack(0x80, |e, text| {
                let name = e.call(GET_NAME, &args![named]).u32();
                e.call(STRCPY_S, &args![text, 0x80u32, name]);
                let item = node_item(e, entry_node);
                if e.call(ITEM_GET_MOD_SLOTS, &args![item]).bool() {
                    e.call(STRCAT_S, &args![text, 0x80u32, MODDED_SUFFIX]);
                }
                let item = node_item(e, entry_node);
                e.call(
                    LIST_ADD_ITEM,
                    &args![me(e) + INVENTORY_LIST, item, text, 0x0078_24e0u32, 0u32],
                );
            });
            entry_node = e.call(WORD_AT_FOUR, &args![entry_node]).u32();
        }
        e.call(SIMPLE_LIST_DESTRUCTOR, &args![group]);
    });
}

/// A setting's value word (`0043d4d0`, `ECX` = the setting; answers the
/// address of the value).
const SETTING_VALUE_WORD: u32 = 0x0043_d4d0;

/// The tab trait `011d9eac` and the burst of the tab switch:
/// `FORenderedMenu::TriggerBurst` (Xbox PDB) with two settings and a float.
const TAB_TRAIT: u32 = 0x011d_9eac;
const TRIGGER_BURST: u32 = 0x007f_b150;
const BURST_SETTING_ONE: u32 = 0x011d_b458;
const BURST_SETTING_TWO: u32 = 0x011d_b37c;
const BURST_FLOAT: u32 = 0x0101_6248;
/// `RepairMenu::InitRepairMenu` and `ItemModMenu::InitItemModMenu` (Xbox
/// PDB, cdecl), and the inventory call before the mod menu (`004c1520`).
const INIT_REPAIR_MENU: u32 = 0x007b_7020;
const INIT_ITEM_MOD_MENU: u32 = 0x0078_4710;
const CHANGES_PREPARE_MOD: u32 = 0x004c_1520;
/// `HotKeysWheel`'s item assignment (`this`, item).
const WHEEL_ASSIGN_ITEM: u32 = 0x0070_19e0;
/// `PlayerCharacter::UpdatePlayer3d` (Xbox PDB).
const UPDATE_PLAYER_3D: u32 = 0x0095_0030;
/// `ItemChange::ItemChange` (Xbox PDB; `this`, object, count).
const ITEM_CHANGE_CONSTRUCTOR: u32 = 0x004b_c550;
/// The player calls of the ammunition swap: `006044c0` (`tesnpc.cpp`), the
/// weapon's swap `008aded0` (object, flag, flag), the test `004938c0`, and
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB; `this`, effect).
const PLAYER_GETTER_006044C0: u32 = 0x0060_44c0;
const PLAYER_SWAP_AMMO: u32 = 0x008a_ded0;
const TEST_004938C0: u32 = 0x0049_38c0;
const HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
/// Clears a link (`006ecd40`, `thiscall`: flag).
const CLEAR_LINK: u32 = 0x006e_cd40;
/// `BSAudio::QInstance` and `BSAudio::GetSoundHandleByName` (Xbox PDB),
/// `BSSoundHandle::Play` and its destructor body.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND: u32 = 0x00ad_7550;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// `Actor::GetAnimAction` (Xbox PDB), the player's state test `009337b0`,
/// `ExtraDataList::GetCanNotWear` and `ExtraDataList::GetWorn` (Xbox PDB).
const GET_ANIM_ACTION: u32 = 0x008a_7570;
const PLAYER_TEST_009337B0: u32 = 0x0093_37b0;
const EXTRA_CAN_NOT_WEAR: u32 = 0x0041_8b10;
const EXTRA_GET_WORN: u32 = 0x0041_8ab0;
/// `QuantityMenu::Create` (Xbox PDB, cdecl: count, callback, maximum) and
/// the drop count setting.
const QUANTITY_MENU_CREATE: u32 = 0x007a_ba00;
const QUANTITY_SETTING: u32 = 0x011d_2418;

/// A message box with a setting's text (`007052f0`): the three words after
/// the text, the time float `010162c0` and 0.
fn message_box(e: &mut Engine, setting: u32, words: [u32; 3]) {
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![text, words[0], words[1], words[2], time, 0u32],
    );
}

/// The player's update after an equip or a drop: `UpdatePlayer3d`, the
/// process's vtable `+0x580(1, 0, 0)` and the player's `+0x3c4`.
fn refresh_player(e: &mut Engine) {
    let player = e.global::<u32>(PLAYER);
    e.call(UPDATE_PLAYER_3D, &args![player]);
    let player = e.global::<u32>(PLAYER);
    let process = e.call(PLAYER_GETTER_008D8520, &args![player]).u32();
    e.vcall(process, 0x580, &args![1u32, 0u32, 0u32]);
    let player = e.global::<u32>(PLAYER);
    e.vcall(player, 0x3c4, &args![]);
}

/// The keyring trait of the menu tile.
fn keyring_open(e: &mut Engine, this: Ptr<InventoryMenu>) -> bool {
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073984);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    e.call(TILE_IS_TRUE, &args![tile, keyring]).bool()
}

/// Sets the keyring trait of the menu tile.
fn set_keyring(e: &mut Engine, this: Ptr<InventoryMenu>, value: u32) {
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073984);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, keyring, value]);
}

/// Filters the list with `filter` and sorts it (`00780c00`, -1).
fn filter_and_sort(e: &mut Engine, list: u32, filter: u32) {
    e.call(LIST_FILTER, &args![list, filter]);
    e.call(
        LIST_SORT,
        &args![list, 0x0078_0c00u32, -1i32, 0x7fff_ffffu32],
    );
}

/// Marks the selected item's object for update and updates the menu.
fn update_current_object(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), object);
        e.call(
            LIST_ADD_OBJECT,
            &args![this.addr() + ITEM_UPDATE_LIST, slot],
        );
    });
    inventory_menu_update_inventory_menu(e);
}

// Translated from 00780140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::DoClick` (Xbox PDB). By button: the tabs (`0x18`..`0x1c`)
/// switch the tab (closing the keyring, saving and loading the page, the
/// burst and the knob) and show their tutorials and tiles; 8 repairs, `0x13`
/// modifies a weapon, `0x1d` uses the item (hot-keys it while the wheel is
/// open, with the broken and refused messages; ammunition with the keyring
/// open swaps the weapon's ammunition), `0x1e` opens the keyring (refused
/// while the wheel is open), 10 closes it, 7 drops the item (refusals by
/// message; a large stack asks for the quantity).
// Translation incomplete: 00780140 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn inventory_menu_do_click(e: &mut Engine, this: Ptr<InventoryMenu>, id: i32, tile: u32) {
    let base = this.addr();
    // The clicked tile is looked up as the code does; no case reads it.
    let _clicked = if tile == 0 && (0..0x17).contains(&id) {
        tile_at(e, base, 0x28 + 4 * id as u32)
    } else {
        tile
    };
    match id {
        0x18..=0x1c => click_tab(e, this, id),
        8 => {
            if current_item(e) != 0 {
                e.call(PLAY_MENU_SOUND, &args![0x24u32]);
                inventory_menu_save_page_state(e);
                e.call(INIT_REPAIR_MENU, &args![current_item(e)]);
            }
        }
        0x13 => {
            if current_item(e) != 0 && current_item_type(e) == 0x28 {
                e.call(PLAY_MENU_SOUND, &args![0x24u32]);
                inventory_menu_save_page_state(e);
                if e.call(WORD_AT_ZERO, &args![current_item(e)]).u32() != 0 {
                    let list = e.call(WORD_AT_ZERO, &args![current_item(e)]).u32();
                    if node_item(e, list) == 0 {
                        let player = e.global::<u32>(PLAYER);
                        let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
                        let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).u32();
                        let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
                        let player = e.global::<u32>(PLAYER);
                        e.call(
                            CHANGES_PREPARE_MOD,
                            &args![changes, object, count, player, 0u32, current_item(e)],
                        );
                    }
                }
                e.call(INIT_ITEM_MOD_MENU, &args![current_item(e), 1u32]);
            }
        }
        0x1d => click_use(e, this),
        0x1e => {
            if !e
                .call(WHEEL_GET_VISIBLE, &args![base + HOT_KEY_WHEEL])
                .bool()
            {
                inventory_menu_save_page_state(e);
                set_keyring(e, this, 1);
                filter_and_sort(e, base + INVENTORY_LIST, 0x0078_2810);
                inventory_menu_clear_current_item(e, this);
                inventory_menu_load_page_state(e);
            } else {
                message_box(
                    e,
                    NO_HOTKEY_SETTING,
                    [0, MESSAGE_TEXT_ONE, MESSAGE_TEXT_TWO],
                );
            }
        }
        0xa => {
            inventory_menu_save_page_state(e);
            set_keyring(e, this, 0);
            filter_and_sort(e, base + INVENTORY_LIST, 0x0078_2620);
            inventory_menu_load_page_state(e);
        }
        7 => click_drop(e, this),
        _ => {}
    }
}

/// The tab buttons of [`inventory_menu_do_click`].
fn click_tab(e: &mut Engine, this: Ptr<InventoryMenu>, id: i32) {
    let base = this.addr();
    if e.get(this, InventoryMenu::iCurrTab) != id - 0x18 {
        if keyring_open(e, this) {
            set_keyring(e, this, 0);
        }
        inventory_menu_clear_current_item(e, this);
        inventory_menu_save_page_state(e);
        e.set(this, InventoryMenu::iCurrTab, id - 0x18);
        let tab = e.get(this, InventoryMenu::iCurrTab);
        let tab_trait = e.global::<u32>(TAB_TRAIT);
        let tile = tile_at(e, base, 0x5c);
        e.call(SET_TRAIT_FROM_INT, &args![tile, tab_trait, tab]);
        let constant = x87_quiet_bits(e.global::<u32>(BURST_FLOAT));
        let second = e.call(SETTING_FLOAT, &args![BURST_SETTING_TWO]).u32();
        let second = x87_quiet_bits(e.mem.u32(second));
        let first = e.call(SETTING_FLOAT, &args![BURST_SETTING_ONE]).u32();
        let first = x87_quiet_bits(e.mem.u32(first));
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let pipboy = e.call(PIPBOY_MANAGER, &args![manager]).u32();
        e.call(TRIGGER_BURST, &args![pipboy, first, second, constant]);
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        fn_00780be0(e, Ptr::new(manager), id as u8);
        filter_and_sort(e, base + INVENTORY_LIST, 0x0078_2620);
        inventory_menu_load_page_state(e);
        fn_00782470(e, 1);
    }
    if id == 0x18 {
        let tile = tile_at(e, base, 0x74);
        set_trait_from_int(e, tile, TRAIT_VISIBLE, 1);
    } else if id == 0x1c {
        let class = fn_0077ff80(e);
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(
            SHOW_TUTORIAL,
            &args![manager.wrapping_add(0x4d4), 0x24u32, class, 0x1f4u32],
        );
        let list = me(e) + INVENTORY_LIST;
        e.call(
            LIST_SORT,
            &args![list, 0x0078_2850u32, 0u32, 0x7fff_ffffu32],
        );
    } else {
        if id == 0x19 {
            let class = fn_0077ff80(e);
            let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
            e.call(
                SHOW_TUTORIAL,
                &args![manager.wrapping_add(0x4d4), 0x23u32, class, 0x200u32],
            );
        }
        for offset in [0x74u32, 0x78, 0x80] {
            let tile = tile_at(e, base, offset);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        }
    }
}

/// The use button (`0x1d`) of [`inventory_menu_do_click`].
fn click_use(e: &mut Engine, this: Ptr<InventoryMenu>) {
    let base = this.addr();
    let usable = current_item(e) != 0 && !keyring_open(e, this) && current_item_type(e) != 0x29;
    if usable {
        let mut selected = false;
        if e.call(WHEEL_GET_VISIBLE, &args![base + HOT_KEY_WHEEL])
            .bool()
        {
            if !e.call(GAMEPAD_IN_USE, &args![]).bool() {
                let broken = current_item(e) != 0 && {
                    let health = e.call(ITEM_GET_HEALTH, &args![current_item(e), 1u32]).f64();
                    health == e.global::<f64>(ZERO_DOUBLE)
                };
                if broken {
                    message_box(e, BROKEN_ITEM_SETTING, [2, 0, MESSAGE_TEXT_TWO]);
                } else {
                    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073974);
                    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
                    let refused = !e.call(TILE_IS_TRUE, &args![tile, keyring]).bool()
                        || current_item_type(e) == 0x19;
                    if refused {
                        message_box(e, NO_HOTKEY_SETTING, [2, 0, MESSAGE_TEXT_TWO]);
                    } else {
                        e.call(
                            WHEEL_ASSIGN_ITEM,
                            &args![base + HOT_KEY_WHEEL, current_item(e)],
                        );
                        update_current_object(e, this);
                    }
                }
            }
        } else {
            inventory_menu_select_current_item(e, this);
            selected = true;
        }
        if selected {
            refresh_player(e);
        }
        return;
    }
    if current_item_type(e) != 0x29 {
        return;
    }
    let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    if object == 0 {
        return;
    }
    let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073974);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    if !e.call(TILE_IS_TRUE, &args![tile, keyring]).bool() {
        return;
    }
    let block = e.call(OPERATOR_NEW_WORD, &args![0xcu32]).u32();
    let item = if block != 0 {
        let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).u32();
        let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        e.call(ITEM_CHANGE_CONSTRUCTOR, &args![block, object, count])
            .u32()
    } else {
        0
    };
    let player = e.global::<u32>(PLAYER);
    let process = e.call(PLAYER_GETTER_008D8520, &args![player]).u32();
    let player = e.global::<u32>(PLAYER);
    let npc = e.call(PLAYER_GETTER_006044C0, &args![player]).u32();
    let player = e.global::<u32>(PLAYER);
    let weapon = e.call(GET_CURRENT_WEAPON, &args![player]).u32();
    e.vcall(process, 0x168, &args![item]);
    let link = e.vcall(process, 0x14c, &args![]).u32();
    e.call(CLEAR_LINK, &args![link, 0u32]);
    let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    let player = e.global::<u32>(PLAYER);
    e.call(PLAYER_SWAP_AMMO, &args![player, object, 1u32, 0u32]);
    let flag = if npc != 0 && !e.call(TEST_004938C0, &args![npc]).bool() {
        0u32
    } else {
        2
    };
    let held = e.vcall(process, 0x148, &args![]).u32();
    let active = e.call(HAS_MOD_EFFECT_ACTIVE, &args![held, 2u32]).bool();
    let player = e.global::<u32>(PLAYER);
    e.vcall(player, 0x3ec, &args![weapon, flag, u32::from(active), 0u32]);
    update_current_object(e, this);
}

/// The drop button (7) of [`inventory_menu_do_click`].
fn click_drop(e: &mut Engine, this: Ptr<InventoryMenu>) {
    if current_item(e) == 0 {
        return;
    }
    e.with_stack(0xc, |e, position| {
        for i in 0..3 {
            let word = e.mem.u32(DROP_POSITION + 4 * i);
            e.mem.set_u32(position.addr() + 4 * i, word);
        }
        let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        let extra = first_extra_list(e, current_item(e));
        if keyring_open(e, this) {
            e.with_stack(0xc, |e, handle| {
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                let sound = e
                    .call(
                        AUDIO_GET_SOUND,
                        &args![audio, handle, MESSAGE_TEXT_TWO, 0x121u32],
                    )
                    .u32();
                e.call(SOUND_PLAY, &args![sound, 0u32]);
                e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
            });
            return;
        }
        if e.vcall(object, 0x94, &args![]).bool() {
            message_box(e, 0x011d_47a4, [2, 0, 0]);
            return;
        }
        let player = e.global::<u32>(PLAYER);
        if e.call(GET_ANIM_ACTION, &args![player]).i32() != -1
            && e.call(ITEM_GET_WORN, &args![current_item(e), 0u32]).bool()
        {
            message_box(e, 0x011d_22c8, [2, 0, 0]);
            return;
        }
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_TEST_009337B0, &args![player]).bool() {
            message_box(e, 0x011d_43e4, [2, 0, 0]);
            return;
        }
        if extra != 0
            && e.call(EXTRA_CAN_NOT_WEAR, &args![extra]).bool()
            && e.call(EXTRA_GET_WORN, &args![extra, 0u32]).bool()
        {
            message_box(e, 0x011d_31f8, [2, 0, 0]);
            return;
        }
        let player = e.global::<u32>(PLAYER);
        if !e
            .call(
                PLAYER_DROP_PREPARE,
                &args![player, object, position, 1u32, 1u32],
            )
            .bool()
        {
            message_box(e, 0x011d_3d30, [2, 0, 0]);
            return;
        }
        let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).i32();
        let limit = e.call(SETTING_VALUE_WORD, &args![QUANTITY_SETTING]).u32();
        if count > e.mem.i32(limit) {
            let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).u32();
            e.call(
                QUANTITY_MENU_CREATE,
                &args![count, 0x0078_0c50u32, 0x7fff_ffffu32],
            );
        } else {
            inventory_menu_drop_current_item(e, 1);
        }
        refresh_player(e);
    });
}

/// `operator new` (cdecl, one word).
const OPERATOR_NEW_WORD: u32 = 0x0040_1000;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0077_fa00,
            inventory_menu_inventory_menu(Ptr<InventoryMenu>) -> Ptr<InventoryMenu>
        ),
        entry!(
            0x0077_fb40,
            inventory_menu_get_class(Ptr<InventoryMenu>) -> u32
        ),
        entry!(
            0x0077_fb50,
            inventory_menu_scalar_deleting_destructor(
                Ptr<InventoryMenu>,
                u32,
            ) -> Ptr<InventoryMenu>
        ),
        entry!(0x0077_fb80, fn_0077fb80(Ptr<InventoryMenu>)),
        entry!(0x0077_fc10, inventory_menu_create() -> u32),
        entry!(0x0077_ff80, fn_0077ff80() -> u32),
        entry!(0x0077_ff90, fn_0077ff90(Ptr<InventoryMenu>) -> bool),
        entry!(0x0078_1ba0, inventory_menu_do_idle(Ptr<InventoryMenu>)),
        entry!(0x0078_24b0, fn_007824b0() -> u8),
        entry!(0x0077_ffd0, inventory_menu_save_page_state()),
        entry!(0x0078_00c0, inventory_menu_load_page_state()),
        entry!(0x0078_0be0, fn_00780be0(Ptr, u8)),
        entry!(0x0078_0c00, fn_00780c00(u32)),
        entry!(0x0078_0c50, inventory_menu_drop_current_item(i32)),
        entry!(
            0x0078_0d60,
            inventory_menu_select_current_item(Ptr<InventoryMenu>)
        ),
        entry!(0x0078_0f70, fn_00780f70() -> bool),
        entry!(
            0x0078_0ff0,
            inventory_menu_do_enter(Ptr<InventoryMenu>, i32, u32)
        ),
        entry!(
            0x0078_1620,
            inventory_menu_do_leave(Ptr<InventoryMenu>, i32, u32)
        ),
        entry!(
            0x0078_1680,
            inventory_menu_update_buttons(Ptr<InventoryMenu>)
        ),
        entry!(
            0x0078_1860,
            inventory_menu_is_item_repairable(Ptr<InventoryMenu>, u32) -> bool
        ),
        entry!(
            0x0078_1b10,
            inventory_menu_clear_current_item(Ptr<InventoryMenu>)
        ),
        entry!(
            0x0078_2190,
            inventory_menu_do_gamepad(Ptr<InventoryMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0078_23d0,
            inventory_menu_do_gamepad_up_event(Ptr<InventoryMenu>, i32, u32) -> bool
        ),
        entry!(0x0078_2470, fn_00782470(u8)),
        entry!(0x0078_24e0, inventory_menu_sort_func(u32, u32) -> i32),
        entry!(0x0078_2620, inventory_menu_filter_func(u32) -> bool),
        entry!(0x0078_2810, fn_00782810(u32) -> bool),
        entry!(0x0078_2850, inventory_menu_init_func(u32, u32)),
        entry!(0x0078_2a90, inventory_menu_update_inventory_menu()),
        entry!(0x0078_32e0, fn_007832e0(Ptr, u32, u32)),
        entry!(
            0x0078_0140,
            inventory_menu_do_click(Ptr<InventoryMenu>, i32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn answer_st0(e: &mut Engine, address: u32, st0: f64) {
        e.register_double(address, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    fn globals(e: &mut Engine) {
        e.map(INVENTORY_MENU & !0xfff, 0x1000);
        e.map(INPUT_GLOBALS & !0xfff, 0x1000);
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.map(MESSAGE_TIME & !0xfff, 0x1000);
        e.map(0x0107_0000, 0x1000);
    }

    #[test]
    fn constructor_builds_the_parts_and_resets_the_pages() {
        let mut e = Engine::new();
        globals(&mut e);
        stub(
            &mut e,
            &[
                MENU_CONSTRUCTOR,
                LIST_BOX_CONSTRUCTOR,
                WHEEL_CONSTRUCTOR,
                UPDATE_LIST_CONSTRUCTOR,
            ],
        );
        e.register(TILE_TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        let this = e.new_object::<InventoryMenu>();
        e.set(this, InventoryMenu::iCurrTab, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0077_fa00, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), INVENTORY_MENU_VTABLE);
        assert_eq!(
            calls_to(&e, WHEEL_CONSTRUCTOR),
            vec![vec![this.addr() + 0xe8]]
        );
        assert_eq!(
            calls_to(&e, UPDATE_LIST_CONSTRUCTOR),
            vec![vec![this.addr() + 0x11c]]
        );
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01073974), 0x3974);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED18), 0xed18);
        for i in 0..6 {
            assert_eq!(e.mem.u32(this.addr() + 0x88 + 8 * i), u32::MAX);
        }
        assert_eq!(e.get(this, InventoryMenu::iCurrTab), 0);
    }

    #[test]
    fn destructor_order_and_delete() {
        let mut e = Engine::new();
        globals(&mut e);
        let this = e.new_object::<InventoryMenu>();
        e.set_global(INVENTORY_MENU, this.addr());
        stub(
            &mut e,
            &[
                UPDATE_LIST_DESTRUCTOR,
                WHEEL_DESTRUCTOR,
                LIST_BOX_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0077_fb50, &args![this, 1u32]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x0077_fb50,
                UPDATE_LIST_DESTRUCTOR,
                WHEEL_DESTRUCTOR,
                LIST_BOX_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE
            ]
        );
        assert_eq!(e.global::<u32>(INVENTORY_MENU), 0);
        e.call_log = Some(vec![]);
        e.call(0x0077_fb50, &args![this, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn class_tiles_and_current_tab_byte() {
        let mut e = Engine::new();
        globals(&mut e);
        assert_eq!(e.call(0x0077_fb40, &args![0u32]).u32(), 0x3ea);
        assert_eq!(e.call(0x0077_ff80, &args![]).u32(), 0x3ea);
        let this = e.new_object::<InventoryMenu>();
        assert!(!e.call(0x0077_ff90, &args![this]).bool());
        for i in 0..23 {
            e.mem.set_u32(this.addr() + 0x28 + 4 * i, 1);
        }
        assert!(e.call(0x0077_ff90, &args![this]).bool());
        assert_eq!(e.call(0x0078_24b0, &args![]).u32() & 0xff, 0);
        e.set_global(INVENTORY_MENU, this.addr());
        e.set(this, InventoryMenu::iCurrTab, 0x102);
        assert_eq!(e.call(0x0078_24b0, &args![]).u32() & 0xff, 2);
    }

    #[test]
    fn create_sets_up_the_menu() {
        let mut e = Engine::new();
        globals(&mut e);
        e.set_global(STACKING_TYPE_TWO, 103.0f64);
        let this = e.new_object::<InventoryMenu>();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0077_fb40);
        for i in 0..23 {
            e.mem.set_u32(this.addr() + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this.addr());
        answer_st0(&mut e, GET_FLOAT_TRAIT, 103.0);
        answer_st0(&mut e, TILE_GET_MAXIMUM_DEPTH, 8.0);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x99);
        e.register(SETTING_STRING, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_READER,
                MENU_REMOVE_FROM_XML_ARCHIVE,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                SET_FLOAT_TRAIT,
                SHOW_TUTORIAL,
                LIST_BOX_INITIALIZE,
                LIST_BOX_WITH_FLAG,
                INITIALIZE_TABLINE,
                WHEEL_INITIALIZE,
                SET_STRING_TRAIT,
                MENU_PREP_FOR_VISIBILITY,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0077_fc10, &args![]).u32(), 0x4444);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, MENU_REMOVE_FROM_XML_ARCHIVE),
            vec![vec![MENU_FILE, me_addr]]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x4444, 0xfad, 8.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, SHOW_TUTORIAL),
            vec![vec![0x4d4, 0x22, 0x3ea, 0x200]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![me_addr + 0xb8, 0x110, ITEM_TEMPLATE_NAME]]
        );
        assert_eq!(
            calls_to(&e, INITIALIZE_TABLINE),
            vec![vec![0x134, 0x18, 0x29b8, 0x4d50, 0x40b4, 0x2acc, 0x3258, 0]]
        );
        assert_eq!(
            calls_to(&e, WHEEL_INITIALIZE),
            vec![vec![me_addr + 0xe8, 0x114]]
        );
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![
                vec![0x128, 0xfc4, 0x29f4, 1],
                vec![0x138, 0x99, 0x2c7c, 1],
                vec![0x13c, 0x99, 0x412c, 1],
                vec![0x140, 0x99, 0x330c, 1],
                vec![0x144, 0x99, 0x20c4, 1],
                vec![0x148, 0x99, 0x396c, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![me_addr, 1]]
        );
    }

    #[test]
    fn create_reports_missing_tiles() {
        let mut e = Engine::new();
        globals(&mut e);
        let this = e.new_object::<InventoryMenu>();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0077_fb40);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_GET_MENU, this.addr());
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_GET_MAXIMUM_DEPTH,
                TILE_READER,
                TILE_READ_FILE,
                MENU_REMOVE_FROM_XML_ARCHIVE,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                PRINT_MESSAGE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0077_fc10, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    /// A menu for `DoIdle` with its tiles, the list and the player set up.
    fn idle_menu(e: &mut Engine) -> Ptr<InventoryMenu> {
        globals(e);
        let this = e.new_object::<InventoryMenu>();
        e.set_global(INVENTORY_MENU, this.addr());
        for i in 0..23 {
            e.mem.set_u32(this.addr() + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0xc, 0x00c0_000c);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0xb8, list_vtable);
        e.mem.set_u32(list_vtable + 4, 0x00c0_0004);
        e.map(PLAYER & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0xc, 0x00c0_00a4);
        answer(e, IS_TOP_MENU_ID, 1);
        answer(e, PIPBOY_READY, 1);
        stub(
            e,
            &[
                GET_PIPBOY,
                SET_TRAIT_FROM_INT,
                SET_FLOAT_TRAIT,
                SET_ITEM_CARD,
                IS_IN_MENU_MODE,
                0x00c0_00a4,
                0x00c0_000c,
            ],
        );
        this
    }

    #[test]
    fn do_idle_does_nothing_unless_on_top() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        answer(&mut e, IS_TOP_MENU_ID, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        assert_eq!(calls_to(&e, IS_TOP_MENU_ID), vec![vec![0x3ea]]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn do_idle_fills_the_card_for_the_first_tab() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, WHEEL_GET_VISIBLE, 0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x28);
        answer_st0(&mut e, 0x00c0_00a4, 5.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        let cards = calls_to(&e, SET_ITEM_CARD);
        assert_eq!(
            cards,
            vec![
                vec![0, 0, 0x150, 0x7000, 1],
                vec![0, 0, 0x158, 0x7000, 1],
                vec![0x138, 0x13c, 0x140, 0x7000, 0],
                vec![0x108, 0x154, 0, 0x7000, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x108, 0xfa9, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, 0x00c0_00a4),
            vec![vec![e.global::<u32>(PLAYER) + 0xa4, 0x12]]
        );
        let _ = me_addr;
    }

    #[test]
    fn do_idle_assigns_the_item_when_the_wheel_highlight_ends() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        e.set_global(WHEEL_HIGHLIGHTED, 1u8);
        e.set(this, InventoryMenu::iCurrTab, 5);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, WHEEL_GET_VISIBLE, 1);
        answer(&mut e, WHEEL_MOVE_HIGHLIGHT, u32::MAX);
        answer_st0(&mut e, ITEM_GET_HEALTH, 0.5);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_IS_TRUE, 1);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x1f);
        e.register(LIST_ADD_OBJECT, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x7100);
            Ret::default()
        });
        stub(
            &mut e,
            &[WHEEL_ASSIGN, UPDATE_INVENTORY_MENU, WHEEL_SET_VISIBLE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, WHEEL_ASSIGN),
            vec![vec![me_addr + 0xe8, 0x7000]]
        );
        assert_eq!(calls_to(&e, LIST_ADD_OBJECT).len(), 1);
        assert_eq!(
            calls_to(&e, WHEEL_SET_VISIBLE),
            vec![vec![me_addr + 0xe8, 1]]
        );
        assert_eq!(e.global::<u8>(WHEEL_HIGHLIGHTED), 0);
        assert!(calls_to(&e, SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn do_idle_refuses_a_broken_item() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        e.set_global(WHEEL_HIGHLIGHTED, 1u8);
        e.set_global(MESSAGE_TIME, 2.0f32.to_bits());
        e.set(this, InventoryMenu::iCurrTab, 5);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, WHEEL_GET_VISIBLE, 1);
        answer(&mut e, WHEEL_MOVE_HIGHLIGHT, u32::MAX);
        answer_st0(&mut e, ITEM_GET_HEALTH, 0.0);
        answer(&mut e, SETTING_STRING, 0x5100);
        stub(&mut e, &[SHOW_MESSAGE]);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        assert_eq!(
            calls_to(&e, SETTING_STRING),
            vec![vec![BROKEN_ITEM_SETTING]]
        );
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE),
            vec![vec![
                0x5100,
                0,
                MESSAGE_TEXT_ONE,
                MESSAGE_TEXT_TWO,
                2.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(e.global::<u8>(WHEEL_HIGHLIGHTED), 0);
    }

    #[test]
    fn do_idle_number_key_opens_the_wheel_on_its_slot() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set(this, InventoryMenu::iCurrTab, 1);
        answer(&mut e, GAMEPAD_IN_USE, 0);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, WHEEL_COUNT, 8);
        answer(&mut e, INPUT_CONTROLS, 0x6000);
        // Key 0x11 + 2 is held.
        e.register(CONTROL_STATE, |_, a| Ret {
            eax: u32::from(a[1] == 0x13 && a[2] == 0),
            ..Ret::default()
        });
        answer(&mut e, WHEEL_GET_VISIBLE, 0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer_st0(&mut e, OBJECT_VALUE_004BE180, 1.0);
        answer_st0(&mut e, OBJECT_VALUE_004BE060, 0.0);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, FORM_TYPE, 0x1f);
        stub(&mut e, &[WHEEL_SET_VISIBLE, WHEEL_MOVE_HIGHLIGHT]);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, WHEEL_SET_VISIBLE),
            vec![vec![me_addr + 0xe8, 1]]
        );
        assert_eq!(
            calls_to(&e, WHEEL_MOVE_HIGHLIGHT),
            vec![vec![me_addr + 0xe8, 2]]
        );
        // Tab 1: the tile at +0x70 for the first value, none for the second.
        let cards = calls_to(&e, SET_ITEM_CARD);
        assert_eq!(cards[0], vec![0, 0x148, 0, 0x7000, 0]);
    }

    #[test]
    fn do_idle_closes_the_wheel_without_a_key_and_clicks_on_control_1() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e);
        e.set(this, InventoryMenu::iCurrTab, 4);
        answer(&mut e, GAMEPAD_IN_USE, 0);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, WHEEL_COUNT, 3);
        answer(&mut e, INPUT_CONTROLS, 0x6000);
        stub(&mut e, &[CONTROL_STATE, WHEEL_SET_VISIBLE]);
        answer(&mut e, WHEEL_GET_VISIBLE, 1);
        answer(&mut e, 0x00c0_0004, 0x55);
        answer(&mut e, IS_IN_MENU_MODE, 1);
        answer(&mut e, CONTROL_PRESSED, 1);
        e.call_log = Some(vec![]);
        e.call(0x0078_1ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, CONTROL_STATE).len(),
            7,
            "two queries per slot, one more"
        );
        assert_eq!(
            calls_to(&e, WHEEL_SET_VISIBLE),
            vec![vec![me_addr + 0xe8, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x154 - 0x28, 0xfa3, 1]]
        );
        assert_eq!(calls_to(&e, 0x00c0_000c), vec![vec![me_addr, 7, 0]]);
    }
}

#[cfg(test)]
mod n054_tests {
    use super::*;

    type Log = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, callee: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn answer_st0(e: &mut Engine, address: u32, st0: f64) {
        e.register_double(address, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    fn real_ftol(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32).into_ret()
        });
    }

    fn pages(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            if !e.mem.is_mapped(address) {
                e.map(address & !0xfff, 0x1000);
            }
        }
    }

    /// A menu with its 23 tiles (`0x100 + 4 * i`) as `pMe`, a vtable with
    /// slots `0x00c0_0000 + offset` and the list's vtable likewise
    /// (`0x00c1_0000 + offset`).
    fn menu(e: &mut Engine) -> Ptr<InventoryMenu> {
        pages(
            e,
            &[
                INVENTORY_MENU,
                PLAYER,
                ZERO_DOUBLE,
                MESSAGE_TIME,
                DROP_POSITION,
            ],
        );
        let this = e.new_object::<InventoryMenu>();
        for i in 0..23 {
            e.mem.set_u32(this.addr() + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        for slot in 0..0x10 {
            e.mem.set_u32(vtable + 4 * slot, 0x00c0_0000 + 4 * slot);
        }
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0xb8, list_vtable);
        for slot in 0..0x10 {
            e.mem
                .set_u32(list_vtable + 4 * slot, 0x00c1_0000 + 4 * slot);
        }
        e.set_global(INVENTORY_MENU, this.addr());
        this
    }

    #[test]
    fn page_state_round_trip() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set(this, InventoryMenu::iCurrTab, 2);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, 0x00c1_0004, 0x900);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x2000);
        answer(&mut e, LIST_WORD_AT_0X14, 0x910);
        e.register(GET_FLOAT_TRAIT, |_, a| Ret {
            st0: if a[0] == 0x900 { 7.0 } else { 3.0 },
            ..Ret::default()
        });
        real_ftol(&mut e);
        e.call(0x0077_ffd0, &args![]);
        assert_eq!(e.mem.i32(this.addr() + 0x88 + 16), 7);
        assert_eq!(e.mem.i32(this.addr() + 0x8c + 16), 3);
        // The keyring page (5) without a highlighted tile.
        answer(&mut e, TILE_IS_TRUE, 1);
        answer(&mut e, 0x00c1_0004, 0);
        e.call(0x0077_ffd0, &args![]);
        assert_eq!(e.mem.i32(this.addr() + 0x88 + 40), -1);
        stub(&mut e, &[LIST_RESTORE_STATE]);
        e.call_log = Some(vec![]);
        e.call(0x0078_00c0, &args![]);
        assert_eq!(e.mem.f32(this.addr() + 0xb8 + 0x24), -1.0);
        assert_eq!(e.mem.f32(this.addr() + 0xb8 + 0x28), 3.0);
        assert_eq!(
            calls_to(&e, LIST_RESTORE_STATE),
            vec![vec![this.addr() + 0xb8, 1]]
        );
    }

    #[test]
    fn small_helpers() {
        let mut e = Engine::new();
        let menu_pointer = menu(&mut e);
        let block = e.mem.alloc(0x20);
        e.call(0x0078_0be0, &args![block, 0x1234u32]);
        assert_eq!(e.mem.u8(block + 0x13), 0x34);
        stub(&mut e, &[SET_TRAIT_FROM_INT]);
        for (row, visible) in [(0.0, 1u32), (-1.0, 0), (f64::NAN, 0)] {
            answer_st0(&mut e, GET_FLOAT_TRAIT, row);
            e.call_log = Some(vec![]);
            e.call(0x0078_0c00, &args![0x700u32]);
            assert_eq!(
                calls_to(&e, SET_TRAIT_FROM_INT),
                vec![vec![0x700, 0xfa3, visible]]
            );
        }
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x2e);
        assert!(e.call(0x0078_2810, &args![0u32]).bool());
        assert!(!e.call(0x0078_2810, &args![0x7000u32]).bool());
        // Actor values: 0x1b below its base.
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        let values = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, values);
        e.mem.set_u32(values + 8, 0x00c2_0008);
        e.mem.set_u32(values + 0x1c, 0x00c2_001c);
        e.register(0x00c2_0008, |_, a| {
            (if a[1] == 0x1b { 5u32 } else { 9 }).into_ret()
        });
        answer(&mut e, 0x00c2_001c, 9);
        assert!(e.call(0x0078_0f70, &args![]).bool());
        answer(&mut e, 0x00c2_0008, 9);
        assert!(!e.call(0x0078_0f70, &args![]).bool());
        // The tab knob.
        e.set(menu_pointer, InventoryMenu::iCurrTab, 3);
        answer(&mut e, INTERFACE_SINGLETON, 0x6000);
        answer(&mut e, PIPBOY_MANAGER, 0x6100);
        stub(&mut e, &[SET_TAB_KNOB_POSITION]);
        e.call_log = Some(vec![]);
        e.call(0x0078_2470, &args![1u32]);
        assert_eq!(
            calls_to(&e, SET_TAB_KNOB_POSITION),
            vec![vec![0x6100, 3, 1]]
        );
        e.set_global(INVENTORY_MENU, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0078_2470, &args![1u32]);
        assert!(calls_to(&e, SET_TAB_KNOB_POSITION).is_empty());
    }

    #[test]
    fn drop_uses_the_extra_list_only_when_it_holds_enough() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        e.register(WORD_AT_ZERO, |_, _| 0x7200u32.into_ret());
        e.register(IDENTITY, |_, a| a[0].into_ret());
        e.mem.map(0x7000, 0x1000);
        e.mem.set_u32(0x7200, 0x7300);
        answer(&mut e, EXTRA_GET_COUNT, 1);
        let player = e.mem.alloc(0x400);
        e.set_global(PLAYER, player);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(player, vtable);
        e.mem.set_u32(vtable + 0x3cc, 0x00c3_03cc);
        stub(
            &mut e,
            &[
                PLAYER_DROP_PREPARE,
                0x00c3_03cc,
                LIST_ADD_OBJECT,
                GET_PIPBOY,
            ],
        );
        answer(&mut e, PIPBOY_READY, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_0c50, &args![2i32]);
        let drop = calls_to(&e, 0x00c3_03cc);
        assert_eq!(drop[0][..4], [player, 0x7100, 0, 2]);
        assert_eq!(calls_to(&e, LIST_ADD_OBJECT)[0][0], this.addr() + 0x11c);
        e.call_log = Some(vec![]);
        e.call(0x0078_0c50, &args![1i32]);
        assert_eq!(calls_to(&e, 0x00c3_03cc)[0][2], 0x7300);
        e.call_log = Some(vec![]);
        e.call(0x0078_0c50, &args![0i32]);
        assert!(calls_to(&e, 0x00c3_03cc).is_empty());
    }

    #[test]
    fn select_equips_or_unequips() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, WORD_AT_ZERO, 0);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, GET_DEFAULT_OBJECT, 0x9900);
        answer(&mut e, LOOKUP_FORM_BY_ID, 0x9901);
        answer(&mut e, ITEM_GET_WORN, 0);
        stub(
            &mut e,
            &[LIST_ADD_OBJECT, QUEUE_EQUIP, QUEUE_UNEQUIP, GET_PIPBOY],
        );
        answer(&mut e, PIPBOY_READY, 0);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        e.call_log = Some(vec![]);
        e.call(0x0078_0d60, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUE_EQUIP),
            vec![vec![player, 0x7100, 0x4000, 0, 0, 0, 1]]
        );
        answer(&mut e, ITEM_GET_WORN, 1);
        e.call_log = Some(vec![]);
        e.call(0x0078_0d60, &args![this]);
        assert_eq!(calls_to(&e, QUEUE_UNEQUIP).len(), 1);
    }

    #[test]
    fn leaving_the_list_clears_the_item() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, GAMEPAD_IN_USE, 0);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        stub(
            &mut e,
            &[
                SET_TRAIT_FROM_INT,
                SET_FLOAT_TRAIT,
                SETTING_STRING,
                SET_STRING_TRAIT,
            ],
        );
        answer(&mut e, WORD_AT_ZERO_0073AC90, 0x5000);
        e.mem.map(0x5000, 0x1000);
        e.mem.set_u32(0x5000, 0x5100);
        e.call_log = Some(vec![]);
        e.call(0x0078_1620, &args![this, 0x1ei32, 0u32]);
        assert_eq!(e.global::<u32>(CURRENT_ITEM), 0);
        let sets = calls_to(&e, SET_TRAIT_FROM_INT);
        assert!(sets.contains(&vec![0x5100, 0xfa3, 0]));
        // ClearCurrentItem's floats and the buttons disabled.
        assert_eq!(calls_to(&e, SET_FLOAT_TRAIT)[0], vec![0x12c, 0xfa3, 0, 0]);
        assert!(sets.contains(&vec![0x120, 0xfaf, 0]));
        let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073974);
        assert_eq!(sets.last().unwrap(), &vec![0x4000, keyring, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0078_1620, &args![this, 3i32, 0u32]);
        assert!(calls_to(&e, SET_TRAIT_FROM_INT).is_empty());
    }

    #[test]
    fn buttons_for_a_selected_weapon() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x29);
        answer(&mut e, ITEM_GET_WORN, 1);
        answer(&mut e, SETTING_STRING, 0x5100);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 1.0);
        // Not repairable: full health.
        pages(&mut e, &[FULL_HEALTH]);
        e.mem.set_f64(FULL_HEALTH, 100.0);
        answer_st0(&mut e, ITEM_GET_HEALTH, 100.0);
        e.register(HEALTH_ROUND, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        stub(
            &mut e,
            &[SET_STRING_TRAIT, SET_FLOAT_TRAIT, SET_TRAIT_FROM_INT],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_1680, &args![this]);
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![0x118, 0xfc4, 0x5100, 1]]
        );
        assert_eq!(
            calls_to(&e, SETTING_STRING),
            vec![vec![UNEQUIP_LABEL_SETTING]]
        );
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![
                vec![0x11c, 0xfaf, 1],
                vec![0x124, 0xfaf, 0],
                vec![0x14c, 0xfaf, 1],
                vec![0x120, 0xfaf, 0]
            ]
        );
    }

    #[test]
    fn repairable_stack_and_group() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[FULL_HEALTH]);
        e.mem.set_f64(FULL_HEALTH, 100.0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x28);
        answer_st0(&mut e, ITEM_GET_HEALTH, 50.0);
        e.register(HEALTH_ROUND, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        answer(&mut e, WORD_AT_FOUR, 2);
        assert!(e.call(0x0078_1860, &args![this, 0x7000u32]).bool());
        assert!(!e.call(0x0078_1860, &args![this, 0u32]).bool());
        // A single item: the group holds another object.
        e.register(WORD_AT_FOUR, |_, a| u32::from(a[0] == 0x7000).into_ret());
        e.register(IDENTITY, |_, a| a[0].into_ret());
        answer(&mut e, WORD_AT_ZERO, 0);
        stub(
            &mut e,
            &[
                SIMPLE_LIST_CONSTRUCTOR,
                GET_INVENTORY_CHANGES,
                SIMPLE_LIST_DESTRUCTOR,
                ITEM_DELETE,
            ],
        );
        e.register(GET_REPAIR_ITEM_GROUP, |e, a| {
            e.mem.set_u32(a[2], 0x7400);
            Ret::default()
        });
        e.register(SIMPLE_LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(ITEM_OBJECT, |_, a| {
            (if a[0] == 0x7400 { 0x7500u32 } else { 0x7100 }).into_ret()
        });
        answer(&mut e, ITEM_GET_WORN, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_1860, &args![this, 0x7000u32]).bool());
        assert_eq!(calls_to(&e, ITEM_DELETE), vec![vec![0x7400, 1]]);
        // Full health (rounded) is not repairable.
        answer_st0(&mut e, ITEM_GET_HEALTH, 100.0);
        assert!(!e.call(0x0078_1860, &args![this, 0x7000u32]).bool());
    }

    #[test]
    fn gamepad_buttons() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        answer(&mut e, IS_TOP_MENU_ID, 1);
        stub(
            &mut e,
            &[GET_PIPBOY, PLAY_MENU_SOUND, SET_STATS_MENU_VISIBLE],
        );
        answer(&mut e, PIPBOY_READY, 1);
        answer(&mut e, MENU_TEST_00709D40, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_2190, &args![this, 0xdi32, 0u32]).bool());
        assert_eq!(calls_to(&e, SET_STATS_MENU_VISIBLE), vec![vec![1, 0]]);
        // Tab left from 0 wraps to 4.
        answer(&mut e, WHEEL_GET_VISIBLE, 0);
        stub(&mut e, &[0x00c0_0014, 0x00c0_000c]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_2190, &args![this, 4i32, 0u32]).bool());
        assert_eq!(calls_to(&e, 0x00c0_0014), vec![vec![this.addr(), 0x1d, 0]]);
        assert_eq!(calls_to(&e, 0x00c0_000c), vec![vec![this.addr(), 0x1c, 0]]);
        // Button 5 is not handled; not on top neither.
        assert!(!e.call(0x0078_2190, &args![this, 5i32, 0u32]).bool());
        answer(&mut e, IS_TOP_MENU_ID, 0);
        assert!(!e.call(0x0078_2190, &args![this, 0xdi32, 0u32]).bool());
    }

    #[test]
    fn gamepad_up_events() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        answer(&mut e, IS_TOP_MENU_ID, 1);
        stub(
            &mut e,
            &[0x00c0_000c, WHEEL_SET_VISIBLE, SET_TRAIT_FROM_INT],
        );
        answer(&mut e, 0x00c1_0004, 0x900);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_23d0, &args![this, 0xai32, 0u32]).bool());
        assert_eq!(calls_to(&e, 0x00c0_000c), vec![vec![this.addr(), 0xa, 0]]);
        assert!(e.call(0x0078_23d0, &args![this, 0x10i32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x12c, 0xfa3, 1]]
        );
        assert!(!e.call(0x0078_23d0, &args![this, 3i32, 0u32]).bool());
    }

    #[test]
    fn sort_by_name_then_health_then_worn() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u32(a + 4, 0xa000);
        e.mem.set_u32(b + 4, 0xb000);
        e.register(ITEM_GET_FULL_NAME, |_, a| a[0].into_ret());
        e.register(STRING_COMPARE, |_, a| {
            (a[0] as i32 - a[1] as i32).signum().into_ret()
        });
        assert_eq!(e.call(0x0078_24e0, &args![a, b]).i32(), -1);
        e.register(STRING_COMPARE, |_, _| 0u32.into_ret());
        e.register(ITEM_GET_HEALTH, |_, a| Ret {
            st0: if a[0] == 0xa000 { 80.0 } else { 20.0 },
            ..Ret::default()
        });
        assert_eq!(e.call(0x0078_24e0, &args![a, b]).i32(), -1);
        answer_st0(&mut e, ITEM_GET_HEALTH, 50.0);
        e.register(ITEM_GET_WORN, |_, a| u32::from(a[0] == 0xb000).into_ret());
        assert_eq!(e.call(0x0078_24e0, &args![a, b]).i32(), 1);
        e.mem.set_u32(a + 4, 0);
        assert_eq!(e.call(0x0078_24e0, &args![a, b]).i32(), 1);
    }

    #[test]
    fn filter_by_tab() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, GET_FORM_AS_BIPED_MODEL, 0);
        answer(&mut e, IS_GOLD, 0);
        e.mem.map(0x7000, 0x1000);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(0x7100, vtable);
        e.mem.set_u32(vtable + 0x94, 0x00c4_0094);
        answer(&mut e, 0x00c4_0094, 0);
        answer(&mut e, FORM_TYPE, 0x28);
        assert!(!e.call(0x0078_2620, &args![0x7000u32]).bool());
        e.set(this, InventoryMenu::iCurrTab, 1);
        assert!(e.call(0x0078_2620, &args![0x7000u32]).bool());
        e.set(this, InventoryMenu::iCurrTab, 3);
        assert!(e.call(0x0078_2620, &args![0x7000u32]).bool());
        assert!(!e.call(0x0078_2620, &args![0u32]).bool());
        answer(&mut e, IS_GOLD, 1);
        e.set(this, InventoryMenu::iCurrTab, 0);
        assert!(e.call(0x0078_2620, &args![0x7000u32]).bool());
    }

    #[test]
    fn init_names_a_stack_and_marks_a_worn_item() {
        let mut e = Engine::new();
        menu(&mut e);
        answer(&mut e, WORD_AT_FOUR, 3);
        answer(&mut e, ITEM_GET_MOD_SLOTS, 0);
        answer(&mut e, ITEM_GET_FULL_NAME, 0x5500);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x1f);
        answer(&mut e, ITEM_GET_WORN, 1);
        answer(&mut e, WORD_AT_ZERO, 0x66);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x5600);
        answer(&mut e, TILE_LIST_LOOKUP, slot);
        stub(&mut e, &[SPRINTF_S, SET_STRING_TRAIT, SET_TRAIT_FROM_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0078_2850, &args![0x700u32, 0x7000u32]);
        assert_eq!(
            calls_to(&e, SPRINTF_S)[0][1..],
            [0x80, COUNT_FORMAT, 0x5500, 3]
        );
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x700, 0xfaa, 0x1d], vec![0x5600, 0xfa3, 1]]
        );
        assert_eq!(calls_to(&e, TILE_LIST_LOOKUP), vec![vec![0x704, 0x66]]);
    }

    #[test]
    fn update_does_nothing_without_a_ready_pipboy() {
        let mut e = Engine::new();
        menu(&mut e);
        stub(&mut e, &[GET_PIPBOY]);
        answer(&mut e, PIPBOY_READY, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_2a90, &args![]);
        assert_eq!(calls_to(&e, PIPBOY_READY).len(), 1);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
        e.set_global(INVENTORY_MENU, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0078_2a90, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn click_closes_the_keyring() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, 0x00c1_0004, 0);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x2000);
        answer(&mut e, LIST_WORD_AT_0X14, 0x910);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 0.0);
        real_ftol(&mut e);
        stub(
            &mut e,
            &[
                SET_TRAIT_FROM_INT,
                LIST_FILTER,
                LIST_SORT,
                LIST_RESTORE_STATE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_0140, &args![this, 0xai32, 0u32]);
        let keyring = e.global::<u32>(TRAIT_FROM_NAME_01073984);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x4000, keyring, 0]]
        );
        assert_eq!(
            calls_to(&e, LIST_FILTER),
            vec![vec![this.addr() + 0xb8, 0x0078_2620]]
        );
    }

    #[test]
    fn click_drop_with_the_keyring_open_plays_the_refusal() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, WORD_AT_ZERO, 0);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        answer(&mut e, TILE_IS_TRUE, 1);
        answer(&mut e, AUDIO_INSTANCE, 0x8000);
        answer(&mut e, AUDIO_GET_SOUND, 0x8100);
        stub(&mut e, &[SOUND_PLAY, SOUND_HANDLE_DESTRUCTOR]);
        e.call_log = Some(vec![]);
        e.call(0x0078_0140, &args![this, 7i32, 0u32]);
        assert_eq!(
            calls_to(&e, AUDIO_GET_SOUND)[0][2..],
            [MESSAGE_TEXT_TWO, 0x121]
        );
        assert_eq!(calls_to(&e, SOUND_PLAY), vec![vec![0x8100, 0]]);
        // Without an item nothing happens.
        e.set_global(CURRENT_ITEM, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0078_0140, &args![this, 7i32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn enter_and_leave_the_list() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        answer(&mut e, LIST_SELECTED_ITEM, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_0ff0, &args![this, 0x1di32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        // Leaving: the item and the icon are cleared, the label set.
        e.set_global(CURRENT_ITEM, 0x7000u32);
        stub(
            &mut e,
            &[SET_TRAIT_FROM_INT, SET_FLOAT_TRAIT, SET_STRING_TRAIT],
        );
        answer(&mut e, SETTING_STRING, 0x5100);
        answer(&mut e, WORD_AT_ZERO_0073AC90, 0x5000);
        e.mem.map(0x5000, 0x1000);
        e.mem.set_u32(0x5000, 0x5200);
        pages(&mut e, &[NO_ITEM_LABEL_SETTING_POINTER]);
        e.set_global(NO_ITEM_LABEL_SETTING_POINTER, 0x0aaau32);
        answer(&mut e, WHEEL_GET_VISIBLE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0078_0ff0, &args![this, 0x1ei32, 0u32]);
        assert_eq!(e.global::<u32>(CURRENT_ITEM), 0);
        let strings = calls_to(&e, SET_STRING_TRAIT);
        assert!(strings.contains(&vec![0x12c, 0xfcc, EMPTY_ICON, 1]));
        assert_eq!(strings.last().unwrap(), &vec![0x118, 0xfc4, 0x5100, 1]);
        assert!(calls_to(&e, SETTING_STRING).contains(&vec![0x0aaa]));
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT).last().unwrap(),
            &vec![0x118, 0xfaf, 1]
        );
    }
}
