//! `fallout/interface/menus/messagemenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The message menu shows the queued message boxes of the game. The menu
//! object is the singleton at `011da4f0` (the Xbox PDB's static `pMe`); the
//! queue is the static `BSSimpleArray<MessageMenuEntry *>` at `011da53c`
//! and the perk/activation choices are the static array at `011da524`. The
//! menu holds its tiles at +0x28 (six pointers) and a `ListBox<int>` of
//! buttons at +0x40.
//!
//! The compiler's exception-unwinding frames (`FS:[0]`) and the stack
//! cookie check (`00ec408c`) are not translated: every function that has
//! one carries a `Translation incomplete` marker.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` at each
//! `float` store.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;

/// `MessageMenu::pMe` (Xbox PDB static): the menu singleton.
const MESSAGE_MENU: u32 = 0x011d_a4f0;
/// `MessageMenu::xEntryList` (Xbox PDB static): the queued messages.
const ENTRY_LIST: u32 = 0x011d_a53c;
/// `MessageMenu::xPerkActivations` (Xbox PDB static).
const PERK_ACTIVATIONS: u32 = 0x011d_a524;
/// `MessageMenu::bDisableMessageMenu` (Xbox PDB static order, byte).
const DISABLE_MESSAGE_MENU: u32 = 0x011d_a4ec;
/// `MessageMenu::bCheckForDeletion` (byte).
const CHECK_FOR_DELETION: u32 = 0x011d_a4ed;
/// `MessageMenu::bDeletedDuringCheck` (byte).
const DELETED_DURING_CHECK: u32 = 0x011d_a4ee;
/// `MessageMenu::bDeferredCreation` (byte; named by the Xbox PDB's static
/// order, the address is the next one after the two flags above).
const DEFERRED_CREATION: u32 = 0x011d_a4ef;
/// `MessageMenu::iStartIndex` (Xbox PDB static): index of the current entry.
const START_INDEX: u32 = 0x011d_a4f4;
/// The disable flag saved by `SetPauseMenuMode` (byte).
const SAVED_DISABLE_FLAG: u32 = 0x011d_a594;
/// `DoIdle`'s function-local static: guard word (bit 0) and the time of the
/// previous check.
const IDLE_GUARD: u32 = 0x011d_a590;
const IDLE_LAST_TIME: u32 = 0x011d_a58c;
/// A byte the destructor sets and `LoadNextMessage` sets to whether the
/// entry's warning context is `0x17` (its reader is not in this unit).
const WARNING_STATE_FLAG_0119F348: u32 = 0x0119_f348;
/// A byte set by `fn_007aa6a0` (reader not in this unit).
const STATE_FLAG_01202D54: u32 = 0x0120_2d54;
/// The object `fn_007aa5c0` calls `00aa4c40` on (a pointer to it is stored
/// here).
const OBJECT_POINTER_01202D74: u32 = 0x0120_2d74;
/// `MessageMenu`'s vtable.
const MESSAGE_MENU_VTABLE: u32 = 0x0107_566c;
/// The vtables of the two `BSSimpleArray` instances.
const ENTRY_ARRAY_VTABLE: u32 = 0x0107_5794;
const PERK_ARRAY_VTABLE: u32 = 0x0107_57a8;

// Callees outside this unit, by address.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// Returns the word at +0xa0 of the interface singleton (a tile).
const INTERFACE_TILE_ROOT: u32 = 0x004f_d400;
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (engine map): reader, path.
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu` (engine map).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMenuByClass` (engine map), cdecl.
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::GetChildByName` (engine map).
const GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::TextToTrait` (engine map), cdecl.
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// `Tile::GetString` (engine map): tile, trait id.
const GET_STRING_TRAIT: u32 = 0x00a0_11f0;
/// `Tile::IsTrue` (engine map): tile, trait id.
const IS_TRUE_TRAIT: u32 = 0x00a0_1230;
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// Sets a float trait from an integer: tile, trait id, integer.
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// `Tile::GetMaximumDepth` (engine map).
const GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// Returns the word at +4 of its object (`Menu::pMenu`, a list node's next).
const WORD_AT_FOUR: u32 = 0x0072_6070;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const MENU_INSTANT_FADE_OUT: u32 = 0x00a1_d9e0;
const MENU_START_FADE_IN: u32 = 0x00a1_db20;
/// The base class's destructor body, called by the menu's destructor.
const MENU_BASE_DESTRUCTOR: u32 = 0x00a1_c520;
/// The destructor of the `ListBox<int>` member.
const LIST_BOX_DESTRUCTOR: u32 = 0x0076_43c0;
/// `ListBox<..>::Initialize` (engine map): list, container tile, template.
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// `ListBox<int>::AddItem` (engine map): list, value, text, 0, 0; returns
/// the item's tile.
const LIST_BOX_ADD_ITEM: u32 = 0x0076_4430;
/// `ListBox<int>::Synchronize` (engine map).
const LIST_BOX_SYNCHRONIZE: u32 = 0x0076_4930;
/// The number of items of a `ListBox` (calls the embedded list's counter).
const LIST_BOX_COUNT: u32 = 0x0072_6c40;
/// Calls the list box's virtual slot 0x14 with (0, 0).
const LIST_BOX_FIRST_ITEM: u32 = 0x007d_6e90;
/// The result of the list item selected (reader `007e1110`): list, tile.
const LIST_BOX_VALUE_OF_TILE: u32 = 0x007e_1110;
/// `BSTimer` update at the menu's +0x70.
const TIMER_UPDATE: u32 = 0x0077_1520;
/// Returns the word at +0x14 of its object (the timer's last time, a
/// string's size).
const WORD_AT_0X14: u32 = 0x0082_5c00;
/// Returns the word at +0x24 of its object (the menu's fade state).
const WORD_AT_0X24: u32 = 0x0059_bb30;
/// `BSSimpleArray` size (word at +8).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `BSSimpleArray` element address (list, index), `this + 4 * index`.
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// The same body under another name (a second template instance).
const ARRAY_ELEMENT_ALIAS: u32 = 0x006a_7ad0;
/// `BSSimpleArray` last element address.
const ARRAY_LAST_ELEMENT: u32 = 0x006f_2270;
/// `BSSimpleArray::Remove` (engine map): list, index, flag.
const ARRAY_REMOVE: u32 = 0x006b_f8f0;
/// `BSSimpleArray` insert (list, index, address of the item).
const ARRAY_INSERT: u32 = 0x006f_23d0;
/// `BSSimpleArray::AddUninitialized` (engine map name): list, address of
/// the item to append.
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// `BSSimpleArray` clear/free (list, flag).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// The base constructor of a `BSSimpleArray` (list, 0, 0).
const ARRAY_CONSTRUCT: u32 = 0x006b_3eb0;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
/// Constructs a `BSStringT` from a C string.
const STRING_FROM_C_STRING: u32 = 0x0040_c0e0;
/// `BSStringT` assignment of a C string.
const STRING_ASSIGN: u32 = 0x0043_8390;
/// `BSStringT<char>::operator+=` of a C string.
const STRING_APPEND: u32 = 0x0040_4820;
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_C_STR: u32 = 0x0055_9450;
/// Copies a C string: destination, size, source (cdecl).
const COPY_C_STRING: u32 = 0x0040_6d30;
/// Compares two C strings (cdecl, zero when equal).
const COMPARE_C_STRINGS: u32 = 0x0040_8b20;
/// Length of a C string (cdecl).
const C_STRING_LENGTH: u32 = 0x0044_a670;
/// `toupper` (engine map, cdecl).
const TO_UPPER: u32 = 0x00ec_a7f4;
const STD_STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
const STD_STRING_DESTROY: u32 = 0x0044_a540;
const STD_STRING_C_STR: u32 = 0x0044_a560;
const STD_STRING_DATA: u32 = 0x0044_a970;
const STD_STRING_INIT: u32 = 0x0044_a7b0;
const STD_STRING_RESET: u32 = 0x0044_a730;
const STD_STRING_ASSIGN_SUBSTR: u32 = 0x0044_a580;
const STD_STRING_FIND_C_STRING: u32 = 0x0051_03a0;
const STD_STRING_FIND_CHARS: u32 = 0x0051_03d0;
const STD_STRING_INSIDE: u32 = 0x0044_ab90;
const STD_STRING_GROW: u32 = 0x0044_a8d0;
const STD_STRING_SET_SIZE: u32 = 0x0044_a880;
/// Copies the (empty) allocator object: its address, the source.
const ALLOCATOR_COPY: u32 = 0x0065_de30;
/// Move and copy with a destination size (cdecl: dest, size, src, count).
const MEMORY_MOVE: u32 = 0x0044_d580;
const MEMORY_COPY: u32 = 0x0044_ccd0;
/// Throws `std::out_of_range` / `std::length_error` (never returns).
const THROW_OUT_OF_RANGE: u32 = 0x00ec_30e8;
const THROW_LENGTH_ERROR: u32 = 0x00ec_30b0;
/// Debug-iterator assertion of `operator[]`.
const STRING_INDEX_ASSERT: u32 = 0x00ec_7c56;
/// The value of a game setting object (word at +4, null object gives 0).
const SETTING_VALUE: u32 = 0x0040_3df0;
/// A pointer to the `int` value of an ini-style setting (word at +4).
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// A pointer to the `float` value of a setting (word at +4).
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// `Interface::GetTopMenuID` (engine map).
const GET_TOP_MENU_ID: u32 = 0x0070_23c0;
/// `Interface::GetMessageMenuResult` (engine map).
const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// `Interface::InDialog` (engine map).
const IN_DIALOG: u32 = 0x0070_50d0;
/// `Interface::SetDialogPause` (engine map), cdecl.
const SET_DIALOG_PAUSE: u32 = 0x0070_5110;
/// The check whether a menu class is on the enter stack.
const ENTER_STACK_CONTAINS: u32 = 0x0071_4f90;
/// `Interface::AddToEnterStack` (engine map), cdecl.
const ADD_TO_ENTER_STACK: u32 = 0x0070_6fb0;
/// `Interface::PopFromEnterStack` (engine map), cdecl.
const POP_FROM_ENTER_STACK: u32 = 0x0070_6fd0;
/// `Interface::GetCurrentRenderedMenu` (engine map).
const GET_CURRENT_RENDERED_MENU: u32 = 0x0070_7ad0;
/// `Interface::QueueMenuCreate` (engine map), cdecl.
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// `Interface::SetGunScopeVisible` (engine map), cdecl.
const SET_GUN_SCOPE_VISIBLE: u32 = 0x0070_9c40;
/// Returns a byte of the object (checked by `Close`).
const CLOSE_CHECK: u32 = 0x004a_4040;
/// The check `Create` makes on the new menu object.
const MENU_INITIALIZED_CHECK: u32 = 0x007a_bdb0;
/// Prints an error message (engine map `Error`, cdecl).
const ERROR_PRINT: u32 = 0x0040_fbe0;
/// The sound system instance getter.
const SOUND_SYSTEM: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByName` (engine map): handle, name, flags.
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
/// `BSSoundHandle::Play` (engine map): handle, flag.
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// Whether the handle's first word differs from the argument.
const SOUND_HANDLE_DIFFERS: u32 = 0x0076_a3b0;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// The font manager instance getter.
const FONT_MANAGER: u32 = 0x005b_d5b0;
/// `FontManager::CalculateStringDimensions` (engine map): out (3 floats),
/// text, font, maximum width, flag.
const CALCULATE_STRING_DIMENSIONS: u32 = 0x00a1_b020;
/// Float helpers (cdecl, two `float`s, result in `ST0`).
const FLOAT_MAX_OF: u32 = 0x0040_4010;
const FLOAT_MIN_OF: u32 = 0x0040_ebd0;
/// Signed integer maximum (cdecl).
const INT_MAX_OF: u32 = 0x0064_7b70;
/// `_ftol2_sse`: the value is a leading `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `InterfaceManager` screen sizes: width/height as floats and integers.
const SCREEN_WIDTH_FLOAT: u32 = 0x0071_5d40;
const SCREEN_HEIGHT_FLOAT: u32 = 0x0071_5da0;
const SCREEN_WIDTH_INT: u32 = 0x0070_6e50;
const SCREEN_HEIGHT_INT: u32 = 0x0070_6e20;
const MARGIN_WIDTH: u32 = 0x0071_77c0;
const MARGIN_HEIGHT: u32 = 0x0071_7820;
/// Script and reference functions used by the activation choices.
const SCRIPT_RUN: u32 = 0x005a_c1e0;
const REFR_ACTIVATE: u32 = 0x0057_3170;
const REFR_COUNT: u32 = 0x0077_8930;
const REFR_GET_TARGET_TYPE: u32 = 0x0057_9280;
const REFR_BASE_FORM: u32 = 0x007a_f430;
const FORM_TYPE: u32 = 0x0040_1170;
const REFR_GET_LOCK: u32 = 0x0056_9160;
const REFR_CHECK_0057_9690: u32 = 0x0057_9690;
const LOCK_CHECK: u32 = 0x0050_21a0;
const REFR_IS_AN_OWNER: u32 = 0x0057_85e0;
const DOOR_CAN_IGNORE_LOCK: u32 = 0x0051_8f00;
const OPEN_CLOSE_STATE: u32 = 0x0047_b250;
const SCRIPT_OF_ITEM: u32 = 0x0041_3f40;
/// Whether a list node is the end of the list.
const NODE_IS_END: u32 = 0x0082_56d0;
/// Returns its `this` (the address of a list node's item slot).
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// The player reference pointer.
const PLAYER: u32 = 0x011d_ea3c;

// Data.
const NPOS: u32 = 0x0101_73f0;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const TWO_DOUBLE: u32 = 0x0101_1590;
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
const TWO_FIFTY_FIVE_DOUBLE: u32 = 0x0101_e568;
const ONE_TWENTY_DOUBLE: u32 = 0x0107_18b8;
const EIGHTY_DOUBLE: u32 = 0x0107_2310;
const KIND_ONE_DOUBLE: u32 = 0x0107_04f0;
const KIND_TWO_DOUBLE: u32 = 0x0107_04e8;
const FLOAT_MAX: u32 = 0x0101_6970;
const HUNDRED_FLOAT: u32 = 0x0101_6410;
/// `"_background_fill_alpha"`.
const BACKGROUND_FILL_ALPHA_NAME: u32 = 0x0106_efe0;
/// `"Data\Menus\message_menu.xml"`.
const MENU_FILE: u32 = 0x0107_5724;
/// `"Message Menu Creation Failed... Are your menu and art resources up to
/// date?"`.
const CREATION_FAILED: u32 = 0x0107_56d8;
/// `"MM_ButtonTemplate"`, `"MM_Background"`.
const BUTTON_TEMPLATE: u32 = 0x0107_56c4;
const BACKGROUND_TILE_NAME: u32 = 0x0107_56b4;
/// `"_horbuf"`, `"UIPopUpMessageWindow"`, `"ListItemText"`,
/// `"_MaxMenuWidth"`, `"_MinMenuWidth"`.
const HORIZONTAL_BUFFER_NAME: u32 = 0x0107_5788;
const POPUP_SOUND_NAME: u32 = 0x0107_5770;
const LIST_ITEM_TEXT_NAME: u32 = 0x0107_5760;
const MAX_MENU_WIDTH_NAME: u32 = 0x0107_5750;
const MIN_MENU_WIDTH_NAME: u32 = 0x0107_5740;
/// `""`, `" "`, `";"`.
const EMPTY_STRING: u32 = 0x0101_1584;
const SPACE_STRING: u32 = 0x0102_0770;
const SEMICOLON_STRING: u32 = 0x0106_3d7c;
/// Game setting objects used by the activation choices.
const SETTING_ACTIVATE_TITLE_011D2280: u32 = 0x011d_2280;
const SETTING_011D3708: u32 = 0x011d_3708;
const SETTING_011D4F48: u32 = 0x011d_4f48;
const SETTING_011D38DC: u32 = 0x011d_38dc;
const SETTING_011D45A0: u32 = 0x011d_45a0;
const SETTING_011D22B0: u32 = 0x011d_22b0;
const SETTING_011D29F4: u32 = 0x011d_29f4;
/// Table of setting object pointers indexed by target type.
const SETTING_TABLE: u32 = 0x011d_5160;
/// The setting objects `Create`/`DisableMessageMenu` read.
const SETTING_011C3EA4: u32 = 0x011c_3ea4;
const SETTING_011D3BC8: u32 = 0x011d_3bc8;

// Trait ids (the `Tile::enumTrait` numbers the PC build uses).
const TRAIT_STACKING_TYPE: u32 = 0xfa7;
const TRAIT_DEPTH: u32 = 0xfad;
const TRAIT_VISIBLE: u32 = 0xfa3;
const TRAIT_ALPHA: u32 = 0xfa9;
const TRAIT_STRING: u32 = 0xfc4;
const TRAIT_ICON_STRING: u32 = 0xfcc;
const TRAIT_WIDTH: u32 = 0xfba;
const TRAIT_HEIGHT: u32 = 0xfb0;
const TRAIT_0FB1: u32 = 0xfb1;
const TRAIT_0FB7: u32 = 0xfb7;
const TRAIT_0FAA: u32 = 0xfaa;
const TRAIT_X: u32 = 0xfa1;
const TRAIT_Y: u32 = 0xfa2;
const TRAIT_FADE: u32 = 0x1772;

layout! {
    /// `MessageMenu` (Xbox PDB), 0x94 bytes: a `Menu` with its tiles at
    /// +0x28 (six pointers), the button `ListBox<int>` at +0x40 and a timer
    /// at +0x70.
    pub struct MessageMenu: 0x94 {
        /// `Menu::pMenu` (Xbox PDB): the `TileMenu *`.
        0x04 pMenu: Ptr,
        /// `Menu::xFadeState` (Xbox PDB).
        0x24 xFadeState: u32,
        /// `uiMessageStartTime` (Xbox PDB).
        0x90 uiMessageStartTime: u32,
    }

    /// `MessageMenu::MessageMenuEntry` (Xbox PDB), 0x50 bytes.
    pub struct MessageMenuEntry: 0x50 {
        /// `strIconFilename` (Xbox PDB).
        0x00 strIconFilename: Inline<BSStringT>,
        /// `strTitle` (Xbox PDB): `char *`.
        0x08 strTitle: Ptr,
        /// `strMessageText` (Xbox PDB).
        0x0C strMessageText: Inline<BSStringT>,
        /// `strButtonsText` (Xbox PDB).
        0x14 strButtonsText: Inline<BSStringT>,
        /// `cFirstButtonResult` (Xbox PDB).
        0x1C cFirstButtonResult: u8,
        /// `strHotkeys` (Xbox PDB).
        0x20 strHotkeys: Inline<BSStringT>,
        /// `fpResultCallback` (Xbox PDB).
        0x28 fpResultCallback: u32,
        /// `iWarningContext` (Xbox PDB).
        0x2C iWarningContext: u32,
        /// `strAudioOverride` (Xbox PDB).
        0x30 strAudioOverride: Inline<BSStringT>,
        /// `bIsModal` (Xbox PDB).
        0x38 bIsModal: u8,
        /// `iXPosOverride` (Xbox PDB).
        0x3C iXPosOverride: i32,
        /// `iYPosOverride` (Xbox PDB).
        0x40 iYPosOverride: i32,
        /// `fMillisecondsVisible` (Xbox PDB).
        0x44 fMillisecondsVisible: f32,
        /// `fBackgroundTextureOpacity` (Xbox PDB).
        0x48 fBackgroundTextureOpacity: f32,
        /// `uiWidthOverride` (Xbox PDB).
        0x4C uiWidthOverride: u32,
    }

    /// `MessageMenu::PerkActivation` (Xbox PDB), 0xC bytes.
    pub struct PerkActivation: 0xc {
        /// `DisplayText` (Xbox PDB): `const char *`.
        0x00 DisplayText: u32,
        /// `pScript` (Xbox PDB).
        0x04 pScript: u32,
        /// `pActivatedObject` (Xbox PDB).
        0x08 pActivatedObject: u32,
    }
}

/// `FLD float; FSTP float`: a copy through the FPU quiets a signalling NaN.
fn fld_fstp(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// `FISTP qword` with truncation: NaN and out-of-range values give the
/// "integer indefinite" `i64::MIN`.
fn fistp_truncate(value: f32) -> i64 {
    if value.is_nan() || value.abs() >= 9.223_372e18 {
        i64::MIN
    } else {
        value.trunc() as i64
    }
}

fn menu_object(e: &Engine) -> u32 {
    e.global(MESSAGE_MENU)
}

/// `pTiles[index]`: the tile pointer at `this + 0x28 + 4 * index`.
fn tile_of(e: &Engine, this: u32, index: u32) -> u32 {
    e.mem.u32(this + 0x28 + 4 * index)
}

fn get_float(e: &mut Engine, tile: u32, id: u32) -> f32 {
    e.call(GET_FLOAT_TRAIT, &args![tile, id]).f32()
}

fn set_float(e: &mut Engine, tile: u32, id: u32, value: f32, flag: bool) {
    e.call(SET_FLOAT_TRAIT, &args![tile, id, value, flag]);
}

fn set_int(e: &mut Engine, tile: u32, id: u32, value: u32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, id, value]);
}

fn set_string(e: &mut Engine, tile: u32, id: u32, text: u32, flag: bool) {
    e.call(SET_STRING_TRAIT, &args![tile, id, text, flag]);
}

fn c_str(e: &mut Engine, string: Ptr<BSStringT>) -> u32 {
    e.call(STRING_C_STR, &args![string]).u32()
}

/// A new entry: `operator new(0x50)` and the constructor.
fn new_entry(e: &mut Engine) -> Ptr<MessageMenuEntry> {
    let object = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if object != 0 {
        fn_007a8850(e, Ptr::new(object))
    } else {
        Ptr::NULL
    }
}

// Translated from 007a8850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `MessageMenu::MessageMenuEntry` (by its body; not named
/// in the engine map): builds the five strings and sets the defaults
/// (modal, positions -1, background opacity from the interface's
/// `_background_fill_alpha` trait).
// Translation incomplete: 007a8850 - the C++ exception-unwinding frame (FS:[0]) that destroys the built strings is not translated
pub fn fn_007a8850(e: &mut Engine, this: Ptr<MessageMenuEntry>) -> Ptr<MessageMenuEntry> {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.call(
        STRING_CONSTRUCT,
        &args![this.at(MessageMenuEntry::strMessageText)],
    );
    e.call(
        STRING_CONSTRUCT,
        &args![this.at(MessageMenuEntry::strButtonsText)],
    );
    e.call(
        STRING_CONSTRUCT,
        &args![this.at(MessageMenuEntry::strHotkeys)],
    );
    e.call(
        STRING_CONSTRUCT,
        &args![this.at(MessageMenuEntry::strAudioOverride)],
    );
    e.set(this, MessageMenuEntry::strTitle, Ptr::NULL);
    e.set(this, MessageMenuEntry::cFirstButtonResult, 0);
    e.set(this, MessageMenuEntry::fpResultCallback, 0);
    e.set(this, MessageMenuEntry::iWarningContext, 0);
    e.set(this, MessageMenuEntry::bIsModal, 1);
    e.set(this, MessageMenuEntry::iXPosOverride, -1);
    e.set(this, MessageMenuEntry::iYPosOverride, -1);
    e.set(this, MessageMenuEntry::fMillisecondsVisible, 0.0);
    let id = e
        .call(TEXT_TO_TRAIT, &args![BACKGROUND_FILL_ALPHA_NAME])
        .u32();
    let singleton = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let root = e.call(INTERFACE_TILE_ROOT, &args![singleton]).u32();
    let opacity = get_float(e, root, id);
    e.set(this, MessageMenuEntry::fBackgroundTextureOpacity, opacity);
    e.set(this, MessageMenuEntry::uiWidthOverride, 0);
    this
}

// Translated from 007a8960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `MessageMenu::MessageMenuEntry` (by its body): frees
/// the title and destroys the five strings, last to first.
// Translation incomplete: 007a8960 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_007a8960(e: &mut Engine, this: Ptr<MessageMenuEntry>) {
    let title = e.get(this, MessageMenuEntry::strTitle);
    e.call(OPERATOR_DELETE, &args![title]);
    e.set(this, MessageMenuEntry::strTitle, Ptr::NULL);
    e.call(
        STRING_DESTROY,
        &args![this.at(MessageMenuEntry::strAudioOverride)],
    );
    e.call(
        STRING_DESTROY,
        &args![this.at(MessageMenuEntry::strHotkeys)],
    );
    e.call(
        STRING_DESTROY,
        &args![this.at(MessageMenuEntry::strButtonsText)],
    );
    e.call(
        STRING_DESTROY,
        &args![this.at(MessageMenuEntry::strMessageText)],
    );
    e.call(STRING_DESTROY, &args![this]);
}

// Translated from 007a8aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::GetClass` (Xbox PDB): the menu class id, `0x3e9`.
pub fn message_menu_get_class(_e: &mut Engine, _this: Ptr<MessageMenu>) -> u32 {
    0x3e9
}

// Translated from 007a8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body and frees the object when bit 0 of `flags` is set.
pub fn message_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<MessageMenu>,
    flags: u32,
) -> Ptr<MessageMenu> {
    fn_007a8b10(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007a8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `MessageMenu` (by its body): restores the vtable,
/// clears the singleton, remembers a deletion during an idle check and
/// destroys the button list and the `Menu` base.
// Translation incomplete: 007a8b10 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_007a8b10(e: &mut Engine, this: Ptr<MessageMenu>) {
    e.mem.set_u32(this.addr(), MESSAGE_MENU_VTABLE);
    e.set_global(WARNING_STATE_FLAG_0119F348, 1u8);
    e.set_global(MESSAGE_MENU, 0u32);
    if e.global::<u8>(CHECK_FOR_DELETION) != 0 {
        e.set_global(DELETED_DURING_CHECK, 1u8);
    }
    e.call(LIST_BOX_DESTRUCTOR, &args![this.addr() + 0x40]);
    e.call(MENU_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 007a8ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::Create` (Xbox PDB): builds the menu from
/// `Data\Menus\message_menu.xml`, shows the first queued message and
/// returns whether the menu was created.
///
/// An existing menu of the class is deleted first. When the file does not
/// give a message menu the new object is deleted (if it exists) and the
/// result is false; a menu failing its check prints an error and returns
/// false.
pub fn message_menu_create(e: &mut Engine) -> bool {
    let singleton = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let mut top_menu_set = false;
    let class = fn_007a8de0(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    if e.call(GET_TOP_MENU_ID, &args![]).u32() != 0 {
        top_menu_set = true;
    }
    let reader = e.call(TILE_READER, &args![singleton]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let found = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(MESSAGE_MENU, found);
    if menu_object(e) != 0 {
        let me = menu_object(e);
        let class_of = e.vcall(me, 0x34, &args![]).u32();
        if class_of == fn_007a8de0(e) {
            return create_finish(e, root, top_menu_set);
        }
    }
    if menu_object(e) != 0 {
        let me = menu_object(e);
        if e.call(WORD_AT_FOUR, &args![me]).u32() != 0 {
            let me = menu_object(e);
            e.vcall(me, 0, &args![1u32]);
        }
    }
    false
}

/// The part of `Create` after the menu of the right class was read.
fn create_finish(e: &mut Engine, root: u32, top_menu_set: bool) -> bool {
    let me = menu_object(e);
    e.call(MENU_SET_MENU_TILE, &args![me, root, 0u32]);
    let me = menu_object(e);
    if !e.call(MENU_INITIALIZED_CHECK, &args![me]).bool() {
        e.call(ERROR_PRINT, &args![CREATION_FAILED]);
        return false;
    }
    let kind = get_float(e, root, TRAIT_STACKING_TYPE) as f64;
    let matches = kind == e.global::<f64>(KIND_ONE_DOUBLE) || {
        let again = get_float(e, root, TRAIT_STACKING_TYPE) as f64;
        again == e.global::<f64>(KIND_TWO_DOUBLE)
    };
    if matches {
        let depth = e.call(GET_MAXIMUM_DEPTH, &args![]).f32();
        set_float(e, root, TRAIT_DEPTH, depth, true);
    }
    let me = menu_object(e);
    let container = e.mem.u32(me + 0x38);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![me + 0x40, container, BUTTON_TEMPLATE],
    );
    if !fn_007a92a0(e).is_null() {
        let me = menu_object(e);
        message_menu_load_next_message(e, Ptr::new(me));
    }
    if top_menu_set {
        let setting = e
            .call(SETTING_FLOAT_POINTER, &args![SETTING_011D3BC8])
            .u32();
        let value = e.mem.f32(setting);
        let opacity = (fld_fstp(value) as f64 * e.global::<f64>(TWO_FIFTY_FIVE_DOUBLE)) as f32;
        let me = menu_object(e);
        let tile = e.call(WORD_AT_FOUR, &args![me]).u32();
        let background = e
            .call(GET_CHILD_BY_NAME, &args![tile, BACKGROUND_TILE_NAME])
            .u32();
        set_float(e, background, TRAIT_ALPHA, opacity, true);
    }
    if e.call(IN_DIALOG, &args![]).bool() {
        e.call(SET_DIALOG_PAUSE, &args![1u32]);
    }
    let me = menu_object(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![me, 0u32]);
    e.call(SET_GUN_SCOPE_VISIBLE, &args![0u32]);
    true
}

// Translated from 007a8de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class id as a plain function (`0x3e9`), what `Create` compares
/// the menu's `GetClass` slot with.
pub fn fn_007a8de0(_e: &mut Engine) -> u32 {
    0x3e9
}

// Translated from 007a8df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::Close` (Xbox PDB): hides the menu's fade trait and fades
/// it out (instantly when a menu is currently rendered), then ends the
/// dialog pause when in a dialog and the dialog check allows it.
pub fn message_menu_close(e: &mut Engine) {
    if menu_object(e) == 0 {
        return;
    }
    let me = menu_object(e);
    let tile = e.call(WORD_AT_FOUR, &args![me]).u32();
    set_int(e, tile, TRAIT_FADE, 1);
    if e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32() != 0 {
        let me = menu_object(e);
        e.call(MENU_INSTANT_FADE_OUT, &args![me]);
    } else {
        let me = menu_object(e);
        e.call(MENU_START_FADE_OUT, &args![me]);
    }
    if e.call(IN_DIALOG, &args![]).bool() && !e.call(CLOSE_CHECK, &args![]).bool() {
        e.call(SET_DIALOG_PAUSE, &args![0u32]);
    }
}

// Translated from 007a8e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a message (not named in the engine map; by its body the message
/// box adder). A cdecl function of 15 words: message text, icon file, title
/// (copied), audio override, result callback, first button's result,
/// warning context, first button text, the address of a cursor into the
/// array of further button texts (advanced by 4 for each text read), modal
/// flag, x and y override, seconds visible, background opacity and width
/// override.
///
/// Button texts are `;`-terminated entries with `&` marking a hotkey
/// (moved into the hotkey string). A modal entry goes in front of the first
/// non-modal one, else at the end. Without an open menu (and when not
/// disabled) the menu is created now or queued; otherwise the deferred
/// creation flag is set. Returns true.
// Translation incomplete: 007a8e60 - the C++ exception-unwinding frame (FS:[0]) and the stack cookie check (00ec408c) are not translated
#[allow(clippy::too_many_arguments)]
pub fn fn_007a8e60(
    e: &mut Engine,
    message_text: u32,
    icon_filename: u32,
    title: u32,
    audio_override: u32,
    result_callback: u32,
    first_button_result: u8,
    warning_context: u32,
    first_button_text: u32,
    button_cursor: u32,
    is_modal: bool,
    x_position: i32,
    y_position: i32,
    seconds_visible: f32,
    background_opacity: f32,
    width_override: f32,
) -> bool {
    // The exe's frame (`SUB ESP,0x84` under the exception record, then `ESI` and the record
    // link): the buttons string at `EBP-0x18` (+0x80), the button text at `EBP-0x3c` (+0x5c),
    // the hotkey at `EBP-0x6c` (+0x2c) and the queued entry at `EBP-0x10` (+0x88).
    e.with_stack(0x98, |e, frame| {
        let entry = new_entry(e);
        let buttons = frame.addr() + 0x80;
        e.call(STRING_FROM_C_STRING, &args![buttons, first_button_text]);
        e.set(entry, MessageMenuEntry::fpResultCallback, result_callback);
        e.set(
            entry,
            MessageMenuEntry::cFirstButtonResult,
            first_button_result,
        );
        e.set(entry, MessageMenuEntry::iWarningContext, warning_context);
        e.call(STRING_ASSIGN, &args![entry, icon_filename]);
        e.call(
            STRING_ASSIGN,
            &args![entry.at(MessageMenuEntry::strHotkeys), EMPTY_STRING],
        );
        e.call(
            STRING_ASSIGN,
            &args![entry.at(MessageMenuEntry::strMessageText), message_text],
        );
        e.call(
            STRING_ASSIGN,
            &args![entry.at(MessageMenuEntry::strAudioOverride), audio_override],
        );
        e.set(entry, MessageMenuEntry::bIsModal, is_modal as u8);
        e.set(entry, MessageMenuEntry::iXPosOverride, x_position);
        e.set(entry, MessageMenuEntry::iYPosOverride, y_position);
        let milliseconds = (seconds_visible as f64 * e.global::<f64>(THOUSAND_DOUBLE)) as f32;
        e.set(entry, MessageMenuEntry::fMillisecondsVisible, milliseconds);
        if background_opacity as f64 > e.global::<f64>(ZERO_DOUBLE) {
            let scaled =
                (background_opacity as f64 * e.global::<f64>(TWO_FIFTY_FIVE_DOUBLE)) as f32;
            e.set(entry, MessageMenuEntry::fBackgroundTextureOpacity, scaled);
        }
        let width = fistp_truncate(width_override) as u32;
        e.set(entry, MessageMenuEntry::uiWidthOverride, width);

        let title_length = if title != 0 {
            e.call(C_STRING_LENGTH, &args![title]).i32()
        } else {
            0
        };
        if title_length > 0 {
            let copy = e
                .call(OPERATOR_NEW, &args![(title_length + 1) as u32])
                .u32();
            e.set(entry, MessageMenuEntry::strTitle, Ptr::new(copy));
            let destination = e.get(entry, MessageMenuEntry::strTitle);
            e.call(
                COPY_C_STRING,
                &args![destination, (title_length + 1) as u32, title],
            );
        } else {
            e.set(entry, MessageMenuEntry::strTitle, Ptr::NULL);
        }

        while e.call(STRING_LENGTH, &args![buttons]).u32() > 0 {
            let text = c_str(e, Ptr::new(buttons));
            let work = frame.addr() + 0x5c;
            e.call(STD_STRING_FROM_C_STRING, &args![work, text]);
            parse_button_text(e, entry, work, frame.addr() + 0x2c);
            e.call(
                STRING_APPEND,
                &args![entry.at(MessageMenuEntry::strButtonsText), SEMICOLON_STRING],
            );
            let cursor = e.mem.u32(button_cursor);
            e.mem.set_u32(button_cursor, cursor.wrapping_add(4));
            let current = e.mem.u32(button_cursor);
            let next = e.mem.u32(current.wrapping_sub(4));
            e.call(STRING_ASSIGN, &args![buttons, next]);
            e.call(STD_STRING_DESTROY, &args![work]);
        }

        queue_entry(e, entry, frame.addr() + 0x88);

        if menu_object(e) == 0 && e.global::<u8>(DISABLE_MESSAGE_MENU) == 0 {
            if !e.call(IN_DIALOG, &args![]).bool() || is_modal {
                let setting = e.call(SETTING_INT_POINTER, &args![SETTING_011C3EA4]).u32();
                if e.mem.i32(setting) > 1 {
                    let not_modal = fn_007a9280(e, entry) == 0;
                    e.call(
                        QUEUE_MENU_CREATE,
                        &args![7u32, 0u32, 0u32, 0u32, 1u32, not_modal as u32],
                    );
                } else {
                    message_menu_create(e);
                }
            } else {
                e.set_global(DEFERRED_CREATION, 1u8);
            }
        } else {
            e.set_global(DEFERRED_CREATION, 1u8);
        }
        e.call(STRING_DESTROY, &args![buttons]);
        true
    })
}

/// The `&` hotkey handling shared by the two button text parsers: the
/// character after `&` goes to the hotkey string and the `&` is erased;
/// the text (unless it is a single space) is appended to the buttons text;
/// the caller appends the `;`. `key` is the caller's local for the hotkey.
fn parse_button_text(e: &mut Engine, entry: Ptr<MessageMenuEntry>, work: u32, key: u32) {
    let amp = fn_007aafe0(e, work, 0x26, 0);
    if amp != e.global::<u32>(NPOS) {
        let sub = fn_007ab000(e, work, key, amp.wrapping_add(1), 1);
        let key_text = e.call(STD_STRING_C_STR, &args![sub]).u32();
        e.call(
            STRING_APPEND,
            &args![entry.at(MessageMenuEntry::strHotkeys), key_text],
        );
        e.call(STD_STRING_DESTROY, &args![key]);
        fn_007aaf80(e, work, amp, 1, EMPTY_STRING);
    }
    let current = e.call(STD_STRING_C_STR, &args![work]).u32();
    if e.call(COMPARE_C_STRINGS, &args![current, SPACE_STRING])
        .i32()
        != 0
    {
        let again = e.call(STD_STRING_C_STR, &args![work]).u32();
        e.call(
            STRING_APPEND,
            &args![entry.at(MessageMenuEntry::strButtonsText), again],
        );
    }
}

/// Puts the entry into the queue: in front of the first non-modal entry
/// when it is modal, else at the end (`007a8e60`). `slot` is the caller's local for the entry.
fn queue_entry(e: &mut Engine, entry: Ptr<MessageMenuEntry>, slot: u32) {
    e.mem.set_u32(slot, entry.addr());
    if e.call(ARRAY_SIZE, &args![ENTRY_LIST]).u32() != 0 {
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![ENTRY_LIST]).u32() {
            let element = e.call(ARRAY_ELEMENT, &args![ENTRY_LIST, index]).u32();
            let existing = e.mem.u32(element);
            if fn_007a9280(e, Ptr::new(existing)) == 0 && fn_007a9280(e, entry) != 0 {
                e.call(ARRAY_INSERT, &args![ENTRY_LIST, index, slot]);
                break;
            }
            let element = e.call(ARRAY_ELEMENT, &args![ENTRY_LIST, index]).u32();
            let last = e.call(ARRAY_LAST_ELEMENT, &args![ENTRY_LIST]).u32();
            if e.mem.u32(element) == e.mem.u32(last) {
                e.call(ARRAY_ADD, &args![ENTRY_LIST, slot]);
                break;
            }
            index += 1;
        }
    } else {
        e.call(ARRAY_ADD, &args![ENTRY_LIST, slot]);
    }
}

// Translated from 007a9280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads `bIsModal` (Xbox PDB name of the field at +0x38) of an entry.
pub fn fn_007a9280(e: &mut Engine, this: Ptr<MessageMenuEntry>) -> u8 {
    e.get(this, MessageMenuEntry::bIsModal)
}

// Translated from 007a92a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The current entry of the queue: the element at `iStartIndex`, or null
/// when the index is past the end.
pub fn fn_007a92a0(e: &mut Engine) -> Ptr<MessageMenuEntry> {
    let mut entry = 0;
    let count = e.call(ARRAY_SIZE, &args![ENTRY_LIST]).u32();
    if e.global::<u32>(START_INDEX) < count {
        let index = e.global::<u32>(START_INDEX);
        let element = e.call(ARRAY_ELEMENT, &args![ENTRY_LIST, index]).u32();
        entry = e.mem.u32(element);
    }
    Ptr::new(entry)
}

// Translated from 007a92e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::LoadNextMessage` (Xbox PDB): lays out the current entry:
/// plays its sound, sets the title, message, icon and button texts, builds
/// the button list, measures the texts to size the tiles, places the menu
/// (at the entry's override or centered) and sets the background opacity.
///
/// Measured widths come from `FontManager::CalculateStringDimensions`
/// (first float of its 3-float result); the modal entries are put on the
/// enter stack, the others taken off it.
// Translation incomplete: 007a92e0 - the C++ exception-unwinding frame (FS:[0]) and the stack cookie check (00ec408c) are not translated
pub fn message_menu_load_next_message(e: &mut Engine, this: Ptr<MessageMenu>) {
    let cur = fn_007a92a0(e);
    if cur.is_null() {
        return;
    }
    let this = this.addr();
    // The exe's frame (`SUB ESP,0x1d8` under the exception record, then the record link): the
    // sound handles at `EBP-0xc0` (+0x128, the custom check), `EBP-0xcc` (+0x11c, the custom
    // sound) and `EBP-0xd8` (+0x110, the default one), the measured sizes at `EBP-0xe4`
    // (+0x104, the title), `EBP-0x10c` (+0xdc, a button) and `EBP-0x38` (+0x1b0, the text), the
    // button list at `EBP-0x6c` (+0x17c), the piece at `EBP-0x88` (+0x160) and its copy at
    // `EBP-0x100` (+0xe8).
    e.with_stack(0x1e8, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let mut title_width = 0.0f32;
        let mut height_sum = 0.0f32;
        let id = e.call(TEXT_TO_TRAIT, &args![HORIZONTAL_BUFFER_NAME]).u32();
        let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        let margin = get_float(e, menu_tile, id);
        let margin_twice = margin as f64 * 2.0;
        let width_override = e.get(cur, MessageMenuEntry::uiWidthOverride);
        let override_float = width_override as f32;
        let audio = cur.at(MessageMenuEntry::strAudioOverride);

        // The sound.
        let mut custom_valid = false;
        if e.call(STRING_LENGTH, &args![audio]).u32() > 0 {
            let handle = local(0x128);
            let name = c_str(e, audio);
            let system = e.call(SOUND_SYSTEM, &args![]).u32();
            let made = e
                .call(
                    GET_SOUND_HANDLE_BY_NAME,
                    &args![system, handle, name, 0x121u32],
                )
                .u32();
            custom_valid = e.call(SOUND_HANDLE_DIFFERS, &args![made, u32::MAX]).bool();
            e.call(SOUND_HANDLE_DESTROY, &args![handle]);
        }
        let handle = if custom_valid {
            local(0x11c)
        } else {
            local(0x110)
        };
        let name = if custom_valid {
            c_str(e, audio)
        } else {
            POPUP_SOUND_NAME
        };
        let system = e.call(SOUND_SYSTEM, &args![]).u32();
        let made = e
            .call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![system, handle, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![made, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);

        // The message text.
        let text = c_str(e, cur.at(MessageMenuEntry::strMessageText));
        let me = menu_object(e);
        let message_tile = e.mem.u32(me + 0x34);
        set_string(e, message_tile, TRAIT_STRING, text, true);

        // The title.
        let out = local(0x104);
        let title = e.get(cur, MessageMenuEntry::strTitle);
        if !title.is_null() {
            let title_tile = tile_of(e, menu_object(e), 1);
            set_string(e, title_tile, TRAIT_STRING, title.addr(), true);
            let title_tile = tile_of(e, menu_object(e), 1);
            set_int(e, title_tile, TRAIT_VISIBLE, 1);
            let measured = if width_override > 0 {
                override_float
            } else {
                let title_tile = tile_of(e, menu_object(e), 1);
                get_float(e, title_tile, TRAIT_WIDTH)
            };
            let maximum = if measured as f64 == 0.0 {
                e.global::<f32>(FLOAT_MAX)
            } else {
                measured
            };
            let font = e.call(FONT_MANAGER, &args![]).u32();
            e.call(
                CALCULATE_STRING_DIMENSIONS,
                &args![font, out, title.addr(), 6u32, maximum, 0u32],
            );
            title_width = e.mem.f32(out);
            let extra = if width_override > 0 {
                0.0
            } else {
                e.global::<f32>(HUNDRED_FLOAT)
            };
            title_width = (title_width as f64 + extra as f64) as f32;
        } else {
            let title_tile = tile_of(e, menu_object(e), 1);
            set_int(e, title_tile, TRAIT_VISIBLE, 0);
        }

        // The buttons.
        let mut widest = 0i32;
        let mut button_heights = 0.0f32;
        let buttons_text = c_str(e, cur.at(MessageMenuEntry::strButtonsText));
        if buttons_text != 0 {
            let list = local(0x17c);
            e.call(STD_STRING_FROM_C_STRING, &args![list, buttons_text]);
            let mut position = 0u32;
            let mut index = 0u32;
            loop {
                let size = e.call(WORD_AT_0X14, &args![list]).u32();
                if position >= size.wrapping_sub(1) {
                    break;
                }
                let semicolon = e
                    .call(
                        STD_STRING_FIND_C_STRING,
                        &args![list, SEMICOLON_STRING, position],
                    )
                    .u32();
                let piece = local(0x160);
                fn_007ab000(e, list, piece, position, semicolon.wrapping_sub(position));
                if e.call(WORD_AT_0X14, &args![piece]).u32() > 0 {
                    let semicolon = e
                        .call(
                            STD_STRING_FIND_C_STRING,
                            &args![list, SEMICOLON_STRING, position],
                        )
                        .u32();
                    let copy = local(0xe8);
                    fn_007ab000(e, list, copy, position, semicolon.wrapping_sub(position));
                    let item_text = e.call(STD_STRING_C_STR, &args![copy]).u32();
                    let value = (e.get(cur, MessageMenuEntry::cFirstButtonResult) as i8 as i32)
                        .wrapping_add(index as i32);
                    let me = menu_object(e);
                    let line_tile = e
                        .call(
                            LIST_BOX_ADD_ITEM,
                            &args![me + 0x40, value, item_text, 0u32, 0u32],
                        )
                        .u32();
                    e.call(STD_STRING_DESTROY, &args![copy]);
                    let text_tile = e
                        .call(GET_CHILD_BY_NAME, &args![line_tile, LIST_ITEM_TEXT_NAME])
                        .u32();
                    set_int(e, line_tile, TRAIT_0FAA, 7);
                    if text_tile != 0 {
                        let measured = if width_override > 0 {
                            override_float
                        } else {
                            get_float(e, text_tile, TRAIT_WIDTH)
                        };
                        let measured = if measured as f64 == 0.0 {
                            e.global::<f32>(FLOAT_MAX)
                        } else {
                            measured
                        };
                        let string = e
                            .call(GET_STRING_TRAIT, &args![text_tile, TRAIT_STRING])
                            .u32();
                        let font = e.call(FONT_MANAGER, &args![]).u32();
                        e.call(
                            CALCULATE_STRING_DIMENSIONS,
                            &args![font, local(0xdc), string, 2u32, measured, 0u32],
                        );
                        let width = e.mem.f32(local(0xdc));
                        let truncated = e.call(FTOL, &args![width as f64]).i32();
                        widest = e.call(INT_MAX_OF, &args![widest, truncated]).i32();
                    }
                    let height = get_float(e, line_tile, TRAIT_HEIGHT);
                    button_heights = (height as f64 + button_heights as f64) as f32;
                }
                position = e
                    .call(
                        STD_STRING_FIND_C_STRING,
                        &args![list, SEMICOLON_STRING, position],
                    )
                    .u32();
                let size = e.call(WORD_AT_0X14, &args![list]).u32();
                if position < size.wrapping_sub(1) {
                    position += 1;
                }
                e.call(STD_STRING_DESTROY, &args![piece]);
                index += 1;
            }
            e.call(STD_STRING_DESTROY, &args![list]);
        }

        // The icon.
        if e.call(STRING_LENGTH, &args![cur]).u32() > 0 {
            let icon = c_str(e, cur.at(MessageMenuEntry::strIconFilename));
            let icon_tile = tile_of(e, menu_object(e), 2);
            set_string(e, icon_tile, TRAIT_ICON_STRING, icon, true);
            let icon_tile = tile_of(e, menu_object(e), 2);
            set_int(e, icon_tile, TRAIT_VISIBLE, 1);
            let plus = (height_sum as f64 + e.global::<f64>(ONE_TWENTY_DOUBLE)) as f32;
            height_sum = if width_override > 0 {
                e.call(FLOAT_MIN_OF, &args![plus, override_float]).f32()
            } else {
                plus
            };
        } else {
            let icon_tile = tile_of(e, menu_object(e), 2);
            set_int(e, icon_tile, TRAIT_VISIBLE, 0);
        }

        // The width of the button list.
        let wide = (widest as f64 + e.global::<f64>(EIGHTY_DOUBLE)) as f32;
        let list_width = if width_override > 0 {
            e.call(FLOAT_MIN_OF, &args![wide, override_float]).f32()
        } else {
            wide
        };
        let button_tile = tile_of(e, menu_object(e), 4);
        set_float(e, button_tile, TRAIT_HEIGHT, button_heights, false);
        let button_tile = tile_of(e, menu_object(e), 4);
        set_float(e, button_tile, TRAIT_0FB1, list_width, false);
        let me = menu_object(e);
        e.call(LIST_BOX_SYNCHRONIZE, &args![me + 0x40]);
        let sum = (margin_twice + list_width as f64) as f32;
        let list_width = if width_override > 0 {
            e.call(FLOAT_MIN_OF, &args![sum, override_float]).f32()
        } else {
            sum
        };

        // The room for the message text.
        let room = if width_override > 0 {
            let screen = e.call(SCREEN_WIDTH_FLOAT, &args![]).f64();
            let divisor = e.call(SCREEN_WIDTH_INT, &args![]).f64();
            ((width_override as f64 / divisor) * screen) as f32
        } else {
            let id = e.call(TEXT_TO_TRAIT, &args![MAX_MENU_WIDTH_NAME]).u32();
            let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
            get_float(e, menu_tile, id)
        };
        let mut room = (room as f64 - margin_twice) as f32;
        let icon_tile = tile_of(e, menu_object(e), 2);
        if e.call(IS_TRUE_TRAIT, &args![icon_tile, TRAIT_VISIBLE])
            .bool()
        {
            room = (room as f64 - e.global::<f64>(ONE_TWENTY_DOUBLE)) as f32;
        }
        let message_tile = tile_of(e, menu_object(e), 3);
        set_float(e, message_tile, TRAIT_WIDTH, room, true);
        let message_tile = tile_of(e, menu_object(e), 3);
        let room_now = get_float(e, message_tile, TRAIT_WIDTH);
        let text = c_str(e, cur.at(MessageMenuEntry::strMessageText));
        let font = e.call(FONT_MANAGER, &args![]).u32();
        e.call(
            CALCULATE_STRING_DIMENSIONS,
            &args![font, local(0x1b0), text, 2u32, room_now, 0u32],
        );
        let measured_width = e.mem.f32(local(0x1b0));
        let lines = e.mem.f32(local(0x1b0) + 8);
        height_sum = ((margin_twice + measured_width as f64) + height_sum as f64) as f32;
        let style = if lines as f64 > e.global::<f64>(ONE_DOUBLE) {
            1
        } else {
            2
        };
        let title_tile = tile_of(e, menu_object(e), 1);
        set_int(e, title_tile, TRAIT_0FB7, style);
        let message_tile = tile_of(e, menu_object(e), 3);
        set_int(e, message_tile, TRAIT_0FB7, style);

        // The menu's width.
        let menu_width = if e.get(cur, MessageMenuEntry::uiWidthOverride) == 0 {
            let wider = e.call(FLOAT_MAX_OF, &args![title_width, height_sum]).f32();
            let menu_width = e.call(FLOAT_MAX_OF, &args![wider, list_width]).f32();
            let id = e.call(TEXT_TO_TRAIT, &args![MAX_MENU_WIDTH_NAME]).u32();
            let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
            let maximum = get_float(e, menu_tile, id);
            let menu_width = e.call(FLOAT_MIN_OF, &args![menu_width, maximum]).f32();
            let id = e.call(TEXT_TO_TRAIT, &args![MIN_MENU_WIDTH_NAME]).u32();
            let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
            let minimum = get_float(e, menu_tile, id);
            e.call(FLOAT_MAX_OF, &args![menu_width, minimum]).f32()
        } else {
            let screen = e.call(SCREEN_WIDTH_FLOAT, &args![]).f64();
            let divisor = e.call(SCREEN_WIDTH_INT, &args![]).f64();
            let width = e.get(cur, MessageMenuEntry::uiWidthOverride) as f64;
            let menu_width = ((width / divisor) * screen) as f32;
            let screen = e.call(SCREEN_WIDTH_FLOAT, &args![]).f64();
            let border = e.call(MARGIN_WIDTH, &args![]).f64();
            let limit = (screen - border * 2.0) as f32;
            e.call(FLOAT_MIN_OF, &args![menu_width, limit]).f32()
        };
        let outer = tile_of(e, menu_object(e), 0);
        set_float(e, outer, TRAIT_0FB1, menu_width, true);
        e.set_global(
            WARNING_STATE_FLAG_0119F348,
            (e.get(cur, MessageMenuEntry::iWarningContext) == 0x17) as u8,
        );
        let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        set_int(e, menu_tile, TRAIT_FADE, 0);
        let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        set_int(e, menu_tile, TRAIT_VISIBLE, 1);

        if fn_007a9280(e, cur) == 0 {
            place_message(e, this, cur);
        } else {
            let outer = tile_of(e, menu_object(e), 0);
            set_float(e, outer, TRAIT_ALPHA, 1.0, true);
            let class = e.vcall(this, 0x34, &args![]).u32();
            let singleton = e.call(INTERFACE_SINGLETON, &args![]).u32();
            if !e
                .call(ENTER_STACK_CONTAINS, &args![singleton, class])
                .bool()
            {
                let class = e.vcall(this, 0x34, &args![]).u32();
                e.call(ADD_TO_ENTER_STACK, &args![class, 0u32]);
            }
        }
        let background = tile_of(e, menu_object(e), 5);
        let opacity = fld_fstp(e.get(cur, MessageMenuEntry::fBackgroundTextureOpacity));
        set_float(e, background, TRAIT_ALPHA, opacity, true);
    });
}

/// The non-modal placement of `LoadNextMessage`: hides the menu's alpha,
/// takes the class off the enter stack and centers the outer tile, or puts
/// it at the entry's x/y override (a fraction of the screen, kept on screen).
fn place_message(e: &mut Engine, this: u32, cur: Ptr<MessageMenuEntry>) {
    let outer = tile_of(e, menu_object(e), 0);
    set_float(e, outer, TRAIT_ALPHA, 0.0, true);
    let class = e.vcall(this, 0x34, &args![]).u32();
    let singleton = e.call(INTERFACE_SINGLETON, &args![]).u32();
    if e.call(ENTER_STACK_CONTAINS, &args![singleton, class])
        .bool()
    {
        let class = e.vcall(this, 0x34, &args![]).u32();
        e.call(POP_FROM_ENTER_STACK, &args![class, 1u32]);
    }
    let screen_x = e.call(SCREEN_WIDTH_FLOAT, &args![]).f32();
    let outer = tile_of(e, menu_object(e), 0);
    let width = get_float(e, outer, TRAIT_0FB1);
    let screen_y = e.call(SCREEN_HEIGHT_FLOAT, &args![]).f32();
    let outer = tile_of(e, menu_object(e), 0);
    let height = get_float(e, outer, TRAIT_HEIGHT);
    let two = e.global::<f64>(TWO_DOUBLE);
    let mut x = ((screen_x as f64 - width as f64) / two) as f32;
    let mut y = ((screen_y as f64 - height as f64) / two) as f32;
    let x_override = e.get(cur, MessageMenuEntry::iXPosOverride);
    if x_override > -1 {
        let divisor = e.call(SCREEN_WIDTH_INT, &args![]).f64();
        let wanted = ((x_override as f64 / divisor) * screen_x as f64) as f32;
        let border = e.call(MARGIN_WIDTH, &args![]).f32();
        x = e.call(FLOAT_MAX_OF, &args![wanted, border]).f32();
        let border = e.call(MARGIN_WIDTH, &args![]).f64();
        let limit = ((screen_x as f64 - width as f64) - border) as f32;
        x = e.call(FLOAT_MIN_OF, &args![x, limit]).f32();
    }
    let y_override = e.get(cur, MessageMenuEntry::iYPosOverride);
    if y_override > -1 {
        let divisor = e.call(SCREEN_HEIGHT_INT, &args![]).f64();
        let wanted = ((y_override as f64 / divisor) * screen_y as f64) as f32;
        let border = e.call(MARGIN_HEIGHT, &args![]).f32();
        y = e.call(FLOAT_MAX_OF, &args![wanted, border]).f32();
        let border = e.call(MARGIN_HEIGHT, &args![]).f64();
        let limit = ((screen_y as f64 - height as f64) - border) as f32;
        y = e.call(FLOAT_MIN_OF, &args![y, limit]).f32();
    }
    let outer = tile_of(e, menu_object(e), 0);
    set_float(e, outer, TRAIT_X, x, true);
    let outer = tile_of(e, menu_object(e), 0);
    set_float(e, outer, TRAIT_Y, y, true);
}

// Translated from 007aa070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::DoClick` (Xbox PDB): a click on button `id` (a tile pointer
/// is taken from `pTiles[id]` for ids 0..6 when none is given). Id 7 is a
/// button of the list: its value goes to the interface's message menu
/// result, the current entry's callback runs and the queue head is cleared
/// (unless the callback deleted the menu).
pub fn message_menu_do_click(e: &mut Engine, this: Ptr<MessageMenu>, id: i32, tile: u32) {
    let mut tile = tile;
    if tile == 0 && (0..6).contains(&id) {
        tile = e.mem.u32(this.addr() + 0x28 + 4 * id as u32);
    }
    if id == 7 && tile != 0 {
        let value = e
            .call(LIST_BOX_VALUE_OF_TILE, &args![this.addr() + 0x40, tile])
            .u32();
        let singleton = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.mem.set_u8(singleton + 0xe4, value as u8);
        e.set_global(CHECK_FOR_DELETION, 1u8);
        if !fn_007a92a0(e).is_null() {
            let current = fn_007a92a0(e);
            if e.get(current, MessageMenuEntry::fpResultCallback) != 0 {
                let current = fn_007a92a0(e);
                let callback = e.get(current, MessageMenuEntry::fpResultCallback);
                e.call(callback, &args![]);
            }
        }
        e.set_global(CHECK_FOR_DELETION, 0u8);
        if e.global::<u8>(DELETED_DURING_CHECK) != 0 {
            e.set_global(DELETED_DURING_CHECK, 0u8);
        } else {
            message_menu_clear_message_queue_head(e);
        }
    }
}

// Translated from 007aa110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::DoKey` (Xbox PDB): Escape (key 1) clicks the last button
/// unless the entry's warning context is `0x17`; Space (key `0x20`) clicks
/// the list's first item; any other key is compared (upper-cased) with the
/// entry's hotkey characters and clicks the matching button. Returns
/// whether a button was clicked.
// Translation incomplete: 007aa110 - the C++ exception-unwinding frame (FS:[0]) and the stack cookie check (00ec408c) are not translated
pub fn message_menu_do_key(e: &mut Engine, this: Ptr<MessageMenu>, key: u32) -> bool {
    let mut handled = false;
    let list = this.addr() + 0x40;
    if fn_007a92a0(e).is_null() {
        return handled;
    }
    let current = fn_007a92a0(e);
    if key == 1 && e.get(current, MessageMenuEntry::iWarningContext) != 0x17 {
        let count = e.call(LIST_BOX_COUNT, &args![list]).u32();
        let item = e
            .vcall(list, 0x14, &args![count.wrapping_sub(1), 0u32])
            .u32();
        e.vcall(this.addr(), 0xc, &args![7u32, item]);
        return true;
    }
    if key == 0x20 {
        let item = e.call(LIST_BOX_FIRST_ITEM, &args![list]).u32();
        e.vcall(this.addr(), 0xc, &args![u32::MAX, item]);
        return true;
    }
    let hotkeys = fn_007a92a0(e).at(MessageMenuEntry::strHotkeys);
    if e.call(STRING_LENGTH, &args![hotkeys]).u32() > 0 {
        let hotkeys = fn_007a92a0(e).at(MessageMenuEntry::strHotkeys);
        let text = c_str(e, hotkeys);
        // The exe's frame (`SUB ESP,0x2c` under the exception record, then `ESI` and the
        // record link): the hotkeys at `EBP-0x30` (+0x10).
        e.with_stack(0x40, |e, frame| {
            let keys = frame.addr() + 0x10;
            e.call(STD_STRING_FROM_C_STRING, &args![keys, text]);
            let mut index = 0u32;
            while index < e.call(WORD_AT_0X14, &args![keys]).u32() {
                let at = fn_007aafb0(e, keys, index);
                let character = e.mem.i8(at) as i32 as u32;
                let upper = e.call(TO_UPPER, &args![character]).u32();
                let upper_key = e.call(TO_UPPER, &args![key]).u32();
                if upper == upper_key {
                    let item = e.vcall(list, 0x14, &args![index, 0u32]).u32();
                    e.vcall(this.addr(), 0xc, &args![7u32, item]);
                    handled = true;
                    break;
                }
                index += 1;
            }
            e.call(STD_STRING_DESTROY, &args![keys]);
        });
    }
    handled
}

// Translated from 007aa2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::DoIdle` (Xbox PDB): every 200 ms of the menu timer, when
/// the current entry is not modal and the outer tile is visible: fully
/// faded-out menus move on to the next message (or fade in when no start
/// time is set); fully faded-in ones record the start time and, once the
/// entry's visible time has passed, clear the queue head (last entry) or
/// start fading out.
// Translation incomplete: 007aa2a0 - the exception-unwinding frame (FS:[0]) of the static initializer is not translated
pub fn message_menu_do_idle(e: &mut Engine, this: Ptr<MessageMenu>) {
    let timer = this.addr() + 0x70;
    e.call(TIMER_UPDATE, &args![timer]);
    if e.global::<u32>(IDLE_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(IDLE_GUARD);
        e.set_global(IDLE_GUARD, guard | 1);
        let now = e.call(WORD_AT_0X14, &args![timer]).u32();
        e.set_global(IDLE_LAST_TIME, now);
    }
    let now = e.call(WORD_AT_0X14, &args![timer]).u32();
    if now.wrapping_sub(e.global::<u32>(IDLE_LAST_TIME)) < 200 {
        return;
    }
    let now = e.call(WORD_AT_0X14, &args![timer]).u32();
    e.set_global(IDLE_LAST_TIME, now);
    let current = fn_007a92a0(e);
    if current.is_null() {
        return;
    }
    let current = fn_007a92a0(e);
    if fn_007a9280(e, current) != 0 {
        return;
    }
    let outer = e.mem.u32(this.addr() + 0x28);
    if !e.call(IS_TRUE_TRAIT, &args![outer, TRAIT_VISIBLE]).bool() {
        return;
    }
    if e.call(WORD_AT_0X24, &args![this]).u32() == 4 {
        if e.get(this, MessageMenu::uiMessageStartTime) == 0 {
            e.call(MENU_START_FADE_IN, &args![this]);
        } else {
            message_menu_clear_message_queue_head(e);
            e.set(this, MessageMenu::uiMessageStartTime, 0);
        }
    } else if e.call(WORD_AT_0X24, &args![this]).u32() == 1 {
        let entry = fn_007a92a0(e);
        if e.get(this, MessageMenu::uiMessageStartTime) == 0 {
            let now = e.call(WORD_AT_0X14, &args![timer]).u32();
            e.set(this, MessageMenu::uiMessageStartTime, now);
        }
        let visible = e.get(entry, MessageMenuEntry::fMillisecondsVisible);
        if visible as f64 > e.global::<f64>(ZERO_DOUBLE) {
            let now = e.call(WORD_AT_0X14, &args![timer]).u32();
            let elapsed = now.wrapping_sub(e.get(this, MessageMenu::uiMessageStartTime));
            if visible as f64 <= elapsed as f64 {
                let last = e.call(ARRAY_LAST_ELEMENT, &args![ENTRY_LIST]).u32();
                if entry.addr() == e.mem.u32(last) {
                    let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
                    set_int(e, menu_tile, TRAIT_FADE, 1);
                    message_menu_clear_message_queue_head(e);
                } else {
                    e.call(MENU_START_FADE_OUT, &args![this]);
                }
            }
        }
    }
}

// Translated from 007aa480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every queued entry, empties the queue and resets
/// `iStartIndex` (not named in the engine map).
pub fn fn_007aa480(e: &mut Engine) {
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![ENTRY_LIST]).u32() {
        let element = e.call(ARRAY_ELEMENT_ALIAS, &args![ENTRY_LIST, index]).u32();
        let entry = e.mem.u32(element);
        if entry != 0 {
            fn_007aa500(e, Ptr::new(entry), 1);
        }
        index += 1;
    }
    e.call(ARRAY_CLEAR, &args![ENTRY_LIST, 1u32]);
    e.set_global(START_INDEX, 0u32);
}

// Translated from 007aa500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `MessageMenuEntry` (by its body): runs the
/// entry destructor and frees the object when bit 0 of `flags` is set.
pub fn fn_007aa500(
    e: &mut Engine,
    this: Ptr<MessageMenuEntry>,
    flags: u32,
) -> Ptr<MessageMenuEntry> {
    fn_007a8960(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007aa530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::ClearMessageQueueHead` (Xbox PDB): deletes the current
/// entry and removes it from the queue; then closes the menu when the queue
/// is empty, else loads the next message.
pub fn message_menu_clear_message_queue_head(e: &mut Engine) {
    if !fn_007a92a0(e).is_null() {
        let index = e.global::<u32>(START_INDEX);
        let element = e.call(ARRAY_ELEMENT_ALIAS, &args![ENTRY_LIST, index]).u32();
        let entry = e.mem.u32(element);
        if entry != 0 {
            fn_007aa500(e, Ptr::new(entry), 1);
        }
        let index = e.global::<u32>(START_INDEX);
        e.call(ARRAY_REMOVE, &args![ENTRY_LIST, index, 1u32]);
    }
    if fn_007a92a0(e).is_null() {
        message_menu_close(e);
    } else if menu_object(e) != 0 {
        let me = menu_object(e);
        message_menu_load_next_message(e, Ptr::new(me));
    }
}

// Translated from 007aa5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the queued entries from the current one on that have the same
/// warning context as the current entry (not named in the engine map; the
/// object behind `01202d74` is told the context first and `fn_007aa6a0`
/// sets its flag).
pub fn fn_007aa5c0(e: &mut Engine) {
    if fn_007a92a0(e).is_null() {
        return;
    }
    let current = fn_007a92a0(e);
    let context = e.get(current, MessageMenuEntry::iWarningContext);
    let object: u32 = e.global(OBJECT_POINTER_01202D74);
    e.call(0x00aa_4c40, &args![object, context]);
    fn_007aa6a0(e, 1);
    let mut index = e.global::<u32>(START_INDEX);
    while index < e.call(ARRAY_SIZE, &args![ENTRY_LIST]).u32() {
        let element = e.call(ARRAY_ELEMENT, &args![ENTRY_LIST, index]).u32();
        let entry = e.mem.u32(element);
        if e.mem.u32(entry + 0x2c) == context {
            let element = e.call(ARRAY_ELEMENT_ALIAS, &args![ENTRY_LIST, index]).u32();
            let entry = e.mem.u32(element);
            if entry != 0 {
                fn_007aa500(e, Ptr::new(entry), 1);
            }
            e.call(ARRAY_REMOVE, &args![ENTRY_LIST, index, 1u32]);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 007aa6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte flag at `01202d54` (cdecl; its readers are elsewhere).
pub fn fn_007aa6a0(e: &mut Engine, value: u8) {
    e.set_global(STATE_FLAG_01202D54, value);
}

// Translated from 007aa6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::DisableMessageMenu` (Xbox PDB): sets the disable flag;
/// disabling closes an open menu, enabling creates (or queues the creation
/// of) the menu when a message is waiting.
pub fn message_menu_disable_message_menu(e: &mut Engine, disable: u8) {
    e.set_global(DISABLE_MESSAGE_MENU, disable);
    if menu_object(e) != 0 && e.global::<u8>(DISABLE_MESSAGE_MENU) != 0 {
        message_menu_close(e);
    }
    if menu_object(e) == 0 && e.global::<u8>(DISABLE_MESSAGE_MENU) == 0 && !fn_007a92a0(e).is_null()
    {
        let setting = e.call(SETTING_INT_POINTER, &args![SETTING_011C3EA4]).u32();
        if e.mem.i32(setting) > 1 {
            e.call(
                QUEUE_MENU_CREATE,
                &args![7u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
        } else {
            message_menu_create(e);
        }
    }
}

// Translated from 007aa720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the activation choices of a reference (not named in the engine
/// map): `choices` is a list of items (item kind 4 gives a text and a
/// script, each kept as a `PerkActivation` with the activated object) and
/// `object` the reference being activated. An extra choice with the
/// reference's own activation text (chosen by its target type) is added
/// last. The entry's callback is `fn_007aae70`. Returns true.
///
/// The last choice's display text points at a buffer of the original's
/// stack frame; here it is a block freed on return, the same lifetime.
// Translation incomplete: 007aa720 - the C++ exception-unwinding frame (FS:[0]) and the stack cookie check (00ec408c) are not translated
pub fn fn_007aa720(e: &mut Engine, choices: u32, object: u32) -> bool {
    // The exe's frame (`SUB ESP,0x1dc` under the exception record, then the record link): the
    // text buffer at `EBP-0x124` (+0xc8), the queued words at `EBP-0x138` (+0xb4, a choice),
    // `EBP-0x12c` (+0xc0, the reference's choice) and `EBP-0x10` (+0x1dc, the entry), the
    // choice texts at `EBP-0x154` (+0x98) and `EBP-0x178` (+0x74) with their hotkeys at
    // `EBP-0x1a4` (+0x48) and `EBP-0x1c0` (+0x2c).
    e.with_stack(0x1ec, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let entry = new_entry(e);
        e.set(entry, MessageMenuEntry::fpResultCallback, 0x007a_ae70);
        e.set(entry, MessageMenuEntry::cFirstButtonResult, 0);
        e.set(entry, MessageMenuEntry::iWarningContext, 0);
        e.call(STRING_ASSIGN, &args![entry, 0u32]);
        e.call(
            STRING_ASSIGN,
            &args![entry.at(MessageMenuEntry::strHotkeys), EMPTY_STRING],
        );
        let title = e
            .call(SETTING_VALUE, &args![SETTING_ACTIVATE_TITLE_011D2280])
            .u32();
        e.call(
            STRING_ASSIGN,
            &args![entry.at(MessageMenuEntry::strMessageText), title],
        );
        let buffer = local(0xc8);
        let mut node = choices;
        while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
            let item = item_of(e, node);
            let text = e.call(WORD_AT_FOUR, &args![item]).u32();
            e.call(COPY_C_STRING, &args![buffer, 0x104u32, text]);
            let item = item_of(e, node);
            if e.vcall(item, 4, &args![]).u32() == 4 {
                let item = item_of(e, node);
                let mut usable = e.call(WORD_AT_FOUR, &args![item]).u32() != 0;
                if usable {
                    let item = item_of(e, node);
                    usable = e.call(SCRIPT_OF_ITEM, &args![item]).u32() != 0;
                }
                if usable {
                    let perk = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
                    let item = item_of(e, node);
                    let text = e.call(WORD_AT_FOUR, &args![item]).u32();
                    e.mem.set_u32(perk, text);
                    let item = item_of(e, node);
                    let script = e.call(SCRIPT_OF_ITEM, &args![item]).u32();
                    e.mem.set_u32(perk + 4, script);
                    e.mem.set_u32(perk + 8, object);
                    e.mem.set_u32(local(0xb4), perk);
                    e.call(ARRAY_ADD, &args![PERK_ACTIVATIONS, local(0xb4)]);
                    if e.call(C_STRING_LENGTH, &args![buffer]).u32() > 0 {
                        let work = local(0x98);
                        e.call(STD_STRING_FROM_C_STRING, &args![work, buffer]);
                        parse_activation_text(e, entry, work, local(0x48));
                        e.call(STD_STRING_DESTROY, &args![work]);
                    }
                }
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }

        // The reference's own choice.
        let target_type = e.call(REFR_GET_TARGET_TYPE, &args![object]).i32();
        let base = e.call(REFR_BASE_FORM, &args![object]).u32();
        let form_type = e.call(FORM_TYPE, &args![base]).u32();
        let lock = e.call(REFR_GET_LOCK, &args![object]).u32();
        let base_form = e.call(REFR_BASE_FORM, &args![object]).u32();
        let player: u32 = e.global(PLAYER);
        if target_type == 1 && e.call(REFR_CHECK_0057_9690, &args![object]).bool() {
            let setting = if form_type == 0x2a {
                SETTING_011D3708
            } else {
                SETTING_011D4F48
            };
            copy_setting(e, buffer, setting);
        } else if target_type == 2 && (form_type == 0x2a || form_type == 0x2b) {
            copy_setting(e, buffer, SETTING_011D38DC);
        } else if target_type == 8 && form_type == 0x1c {
            let state = e.call(OPEN_CLOSE_STATE, &args![object]).i32();
            let open_or_closed = state == 1 || e.call(OPEN_CLOSE_STATE, &args![object]).i32() == 2;
            let locked = lock != 0
                && e.call(LOCK_CHECK, &args![lock]).bool()
                && !e
                    .call(REFR_IS_AN_OWNER, &args![object, player, 1u32])
                    .bool()
                && (base_form == 0
                    || !e
                        .call(DOOR_CAN_IGNORE_LOCK, &args![object, player, 0u32, 1u32])
                        .bool());
            if locked {
                copy_setting(e, buffer, SETTING_011D45A0);
            } else if open_or_closed {
                copy_setting(e, buffer, SETTING_011D22B0);
            } else {
                copy_table_setting(e, buffer, target_type);
            }
        } else if target_type != 0 && target_type < 0x14 {
            copy_table_setting(e, buffer, target_type);
        } else if target_type == 0 {
            copy_setting(e, buffer, SETTING_011D29F4);
        }
        if e.call(C_STRING_LENGTH, &args![buffer]).u32() > 0 {
            let work = local(0x74);
            e.call(STD_STRING_FROM_C_STRING, &args![work, buffer]);
            parse_activation_text(e, entry, work, local(0x2c));
            e.call(STD_STRING_DESTROY, &args![work]);
        }
        let perk = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        e.mem.set_u32(perk, buffer);
        e.mem.set_u32(perk + 4, 0);
        e.mem.set_u32(perk + 8, object);
        e.mem.set_u32(local(0xc0), perk);
        e.call(ARRAY_ADD, &args![PERK_ACTIVATIONS, local(0xc0)]);
        e.mem.set_u32(local(0x1dc), entry.addr());
        e.call(ARRAY_ADD, &args![ENTRY_LIST, local(0x1dc)]);
        if menu_object(e) == 0 && e.global::<u8>(DISABLE_MESSAGE_MENU) == 0 {
            let setting = e.call(SETTING_INT_POINTER, &args![SETTING_011C3EA4]).u32();
            if e.mem.i32(setting) > 1 {
                e.call(
                    QUEUE_MENU_CREATE,
                    &args![7u32, 0u32, 0u32, 0u32, 1u32, 0u32],
                );
            } else {
                message_menu_create(e);
            }
        }
        true
    })
}

/// The item of a list node: the word at the address `006815c0` returns.
fn item_of(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

/// Copies the value of a setting object into the 0x104-byte buffer.
fn copy_setting(e: &mut Engine, buffer: u32, setting: u32) {
    let text = e.call(SETTING_VALUE, &args![setting]).u32();
    e.call(COPY_C_STRING, &args![buffer, 0x104u32, text]);
}

/// Copies the value of the setting object `SETTING_TABLE[target_type]`.
fn copy_table_setting(e: &mut Engine, buffer: u32, target_type: i32) {
    let setting = e
        .mem
        .u32(SETTING_TABLE.wrapping_add((target_type as u32).wrapping_mul(4)));
    copy_setting(e, buffer, setting);
}

/// The `&` handling of `fn_007aa720`: like [`parse_button_text`], but the
/// `;` follows the text only when the text is appended. `key` is the caller's local for the
/// hotkey.
fn parse_activation_text(e: &mut Engine, entry: Ptr<MessageMenuEntry>, work: u32, key: u32) {
    let amp = fn_007aafe0(e, work, 0x26, 0);
    if amp != e.global::<u32>(NPOS) {
        let sub = fn_007ab000(e, work, key, amp.wrapping_add(1), 1);
        let key_text = e.call(STD_STRING_C_STR, &args![sub]).u32();
        e.call(
            STRING_APPEND,
            &args![entry.at(MessageMenuEntry::strHotkeys), key_text],
        );
        e.call(STD_STRING_DESTROY, &args![key]);
        fn_007aaf80(e, work, amp, 1, EMPTY_STRING);
    }
    let current = e.call(STD_STRING_C_STR, &args![work]).u32();
    if e.call(COMPARE_C_STRINGS, &args![current, SPACE_STRING])
        .i32()
        != 0
    {
        let again = e.call(STD_STRING_C_STR, &args![work]).u32();
        e.call(
            STRING_APPEND,
            &args![entry.at(MessageMenuEntry::strButtonsText), again],
        );
        e.call(
            STRING_APPEND,
            &args![entry.at(MessageMenuEntry::strButtonsText), SEMICOLON_STRING],
        );
    }
}

// Translated from 007aae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result callback of `fn_007aa720`'s entry: the chosen index is the
/// interface's message menu result. Every choice but the last runs its
/// script on the activated object; the last activates the object with the
/// player. The activation array is then cleared.
pub fn fn_007aae70(e: &mut Engine) {
    let chosen = e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8() as i8 as i32;
    let count = e.call(ARRAY_SIZE, &args![PERK_ACTIVATIONS]).u32();
    if chosen != count.wrapping_sub(1) as i32 {
        let element = e
            .call(ARRAY_ELEMENT_ALIAS, &args![PERK_ACTIVATIONS, chosen as u32])
            .u32();
        let perk = Ptr::<PerkActivation>::new(e.mem.u32(element));
        let object = e.get(perk, PerkActivation::pActivatedObject);
        let element = e
            .call(ARRAY_ELEMENT_ALIAS, &args![PERK_ACTIVATIONS, chosen as u32])
            .u32();
        let perk = Ptr::<PerkActivation>::new(e.mem.u32(element));
        let script = e.get(perk, PerkActivation::pScript);
        e.call(SCRIPT_RUN, &args![script, object, 0u32, 0u32, 0u32]);
    } else {
        let element = e
            .call(ARRAY_ELEMENT_ALIAS, &args![PERK_ACTIVATIONS, chosen as u32])
            .u32();
        let perk = Ptr::<PerkActivation>::new(e.mem.u32(element));
        let object = e.get(perk, PerkActivation::pActivatedObject);
        let count = e.call(REFR_COUNT, &args![object]).i32();
        let player: u32 = e.global(PLAYER);
        let element = e
            .call(ARRAY_ELEMENT_ALIAS, &args![PERK_ACTIVATIONS, chosen as u32])
            .u32();
        let perk = Ptr::<PerkActivation>::new(e.mem.u32(element));
        let object = e.get(perk, PerkActivation::pActivatedObject);
        e.call(REFR_ACTIVATE, &args![object, player, 0u32, 0u32, count]);
    }
    e.call(ARRAY_CLEAR, &args![PERK_ACTIVATIONS, 1u32]);
}

// Translated from 007aaf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::SetPauseMenuMode` (Xbox PDB): entering the pause menu
/// skips the messages already queued (`iStartIndex` = queue size), closes
/// the menu and keeps the disable flag to restore later; leaving restores
/// both.
pub fn message_menu_set_pause_menu_mode(e: &mut Engine, pause: u8) {
    if pause != 0 {
        let size = e.call(ARRAY_SIZE, &args![ENTRY_LIST]).u32();
        e.set_global(START_INDEX, size);
        message_menu_close(e);
        let flag = e.global::<u8>(DISABLE_MESSAGE_MENU);
        e.set_global(SAVED_DISABLE_FLAG, flag);
        message_menu_disable_message_menu(e, 0);
    } else {
        e.set_global(START_INDEX, 0u32);
        let saved = e.global::<u8>(SAVED_DISABLE_FLAG);
        message_menu_disable_message_menu(e, saved);
    }
}

// Translated from 007aaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::replace(pos, n, const char *)`-style wrapper (by its
/// body): replaces `n` characters at `pos` with the C string, passing its
/// length to `fn_007ab0a0`.
pub fn fn_007aaf80(e: &mut Engine, this: u32, pos: u32, count: u32, text: u32) -> u32 {
    let length = e.call(C_STRING_LENGTH, &args![text]).u32();
    fn_007ab0a0(e, this, pos, count, text, length)
}

// Translated from 007aafb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::operator[]`-style accessor (by its body): the address of
/// character `index`; an index past the size trips the debug assertion
/// (`00ec7c56`).
pub fn fn_007aafb0(e: &mut Engine, this: u32, index: u32) -> u32 {
    if index > e.mem.u32(this + 0x14) {
        e.call(STRING_INDEX_ASSERT, &args![]);
    }
    let data = e.call(STD_STRING_DATA, &args![this]).u32();
    data.wrapping_add(index)
}

// Translated from 007aafe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::find(char, pos)` (by its body): finds one character from
/// `pos` through the string's `find(const char *, pos, 1)`.
pub fn fn_007aafe0(e: &mut Engine, this: u32, character: u32, pos: u32) -> u32 {
    // The exe passes its own argument slot (`EBP+8`): one `PUSH ECX` word under `EBP`, so the
    // slot is +0xc of a frame that reaches it.
    e.with_stack(0x10, |e, frame| {
        let slot = frame.addr() + 0xc;
        e.mem.set_u32(slot, character);
        e.call(STD_STRING_FIND_CHARS, &args![this, slot, pos, 1u32])
            .u32()
    })
}

// Translated from 007ab000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::substr(pos, n)` (by its body): constructs the substring in
/// `out` (through the empty allocator copy) and returns `out`.
pub fn fn_007ab000(e: &mut Engine, this: u32, out: u32, pos: u32, count: u32) -> u32 {
    // The exe's frame (`SUB ESP,0xc` under `EBP`): the allocator byte at `EBP-0x1` (+0xb).
    e.with_stack(0xc, |e, frame| {
        let copy = fn_007ab230(e, this, frame.addr() + 0xb);
        fn_007ab050(e, out, this, pos, count, copy);
    });
    out
}

// Translated from 007ab050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::string(const string &, pos, n, allocator)` (by its body):
/// copies the allocator, initializes the string and assigns the substring.
pub fn fn_007ab050(
    e: &mut Engine,
    this: u32,
    source: u32,
    pos: u32,
    count: u32,
    allocator: u32,
) -> u32 {
    // The `PUSH ECX` slot the allocator copy is made in holds the incoming
    // `this`, and `0044a7b0` receives that word.
    // The slot is the word pushed under the exe's frame (`SUB ESP,0x8` under `EBP`): +0 of a
    // frame that reaches it.
    let kept = e.with_stack(0xc, |e, frame| {
        let slot = frame.addr();
        e.mem.set_u32(slot, this);
        e.call(ALLOCATOR_COPY, &args![slot, allocator]);
        e.mem.u32(slot)
    });
    e.call(STD_STRING_INIT, &args![this, kept]);
    e.call(STD_STRING_RESET, &args![this, 0u32, 0u32]);
    e.call(STD_STRING_ASSIGN_SUBSTR, &args![this, source, pos, count]);
    this
}

fn data_of(e: &mut Engine, string: u32) -> u32 {
    e.call(STD_STRING_DATA, &args![string]).u32()
}

// Translated from 007ab0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::replace(pos, n, const char *, len)` (by its body). When
/// the text lies inside the string itself it is replaced through
/// `fn_007ab250`; otherwise the tail is moved, the string grown and the
/// text copied in. Throws (`00ec30e8`, `00ec30b0`) for a position past the
/// end or a result that is too long.
pub fn fn_007ab0a0(e: &mut Engine, this: u32, pos: u32, count: u32, text: u32, length: u32) -> u32 {
    if e.call(STD_STRING_INSIDE, &args![this, text]).bool() {
        let base = data_of(e, this);
        return fn_007ab250(e, this, pos, count, this, text.wrapping_sub(base), length);
    }
    let mut count = count;
    if e.mem.u32(this + 0x14) < pos {
        e.call(THROW_OUT_OF_RANGE, &args![]);
    }
    if e.mem.u32(this + 0x14).wrapping_sub(pos) < count {
        count = e.mem.u32(this + 0x14).wrapping_sub(pos);
    }
    let room = e.global::<u32>(NPOS).wrapping_sub(length);
    if room <= e.mem.u32(this + 0x14).wrapping_sub(count) {
        e.call(THROW_LENGTH_ERROR, &args![]);
    }
    let tail = e.mem.u32(this + 0x14).wrapping_sub(count).wrapping_sub(pos);
    if length < count {
        let capacity = e.mem.u32(this + 0x18);
        let base = data_of(e, this);
        let source = base.wrapping_add(pos).wrapping_add(count);
        let base = data_of(e, this);
        let destination = base.wrapping_add(pos).wrapping_add(length);
        let size = capacity.wrapping_sub(pos).wrapping_sub(length);
        // 007ab172: `0044d580` reads the ECX it is entered with (its first parameter);
        // `0044a970`, called last at 007ab166, leaves ECX = this.
        e.call(MEMORY_MOVE, &args![this, destination, size, source, tail]);
    }
    if length > 0 || count > 0 {
        let new_size = e
            .mem
            .u32(this + 0x14)
            .wrapping_add(length)
            .wrapping_sub(count);
        if e.call(STD_STRING_GROW, &args![this, new_size, 0u32]).bool() {
            if count < length {
                let base = data_of(e, this);
                let source = base.wrapping_add(pos).wrapping_add(count);
                let capacity = e.mem.u32(this + 0x18);
                let size = capacity.wrapping_sub(pos).wrapping_sub(length);
                let base = data_of(e, this);
                let destination = base.wrapping_add(pos).wrapping_add(length);
                // 007ab1e5: as above, ECX = this (0044a970 at 007ab1d9).
                e.call(MEMORY_MOVE, &args![this, destination, size, source, tail]);
            }
            let capacity = e.mem.u32(this + 0x18);
            let base = data_of(e, this);
            let destination = base.wrapping_add(pos);
            e.call(
                MEMORY_COPY,
                &args![destination, capacity.wrapping_sub(pos), text, length],
            );
            e.call(STD_STRING_SET_SIZE, &args![this, new_size]);
        }
    }
    this
}

// Translated from 007ab230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the (empty) allocator of the string `this` into `target` and
/// returns `target` (by its body).
pub fn fn_007ab230(e: &mut Engine, this: u32, target: u32) -> u32 {
    e.call(ALLOCATOR_COPY, &args![target, this]);
    target
}

/// `memmove_s` with the offsets of `fn_007ab250`.
fn move_chars(e: &mut Engine, this: u32, to: u32, from_string: u32, from: u32, count: u32) {
    transfer_chars(e, MEMORY_MOVE, this, to, from_string, from, count);
}

/// `memcpy_s` with the offsets of `fn_007ab250`.
fn copy_chars(e: &mut Engine, this: u32, to: u32, from_string: u32, from: u32, count: u32) {
    transfer_chars(e, MEMORY_COPY, this, to, from_string, from, count);
}

fn transfer_chars(
    e: &mut Engine,
    function: u32,
    this: u32,
    to: u32,
    from_string: u32,
    from: u32,
    count: u32,
) {
    // The destination size is the capacity minus the destination offset.
    let capacity = e.mem.u32(this + 0x18);
    let source_base = data_of(e, from_string);
    let source = source_base.wrapping_add(from);
    let base = data_of(e, this);
    let destination = base.wrapping_add(to);
    if function == MEMORY_MOVE {
        // 007ab335 and the other move sites: `0044d580` reads the ECX it is entered with;
        // `0044a970`, called last before each, leaves ECX = this.
        e.call(
            function,
            &args![this, destination, capacity.wrapping_sub(to), source, count],
        );
    } else {
        e.call(
            function,
            &args![destination, capacity.wrapping_sub(to), source, count],
        );
    }
}

// Translated from 007ab250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::replace(pos, n, const string &, pos2, n2)` (by its body):
/// makes room for the new characters (growing when the result is longer)
/// and moves the tail and the inserted characters in the order that is safe
/// when the source is the string itself. Throws for a position past the end
/// or a result that is too long.
pub fn fn_007ab250(
    e: &mut Engine,
    this: u32,
    pos: u32,
    count: u32,
    source: u32,
    source_pos: u32,
    source_len: u32,
) -> u32 {
    let mut count = count;
    let mut source_len = source_len;
    if e.mem.u32(this + 0x14) < pos || e.call(WORD_AT_0X14, &args![source]).u32() < source_pos {
        e.call(THROW_OUT_OF_RANGE, &args![]);
    }
    if e.mem.u32(this + 0x14).wrapping_sub(pos) < count {
        count = e.mem.u32(this + 0x14).wrapping_sub(pos);
    }
    let available = e
        .call(WORD_AT_0X14, &args![source])
        .u32()
        .wrapping_sub(source_pos);
    if available < source_len {
        source_len = available;
    }
    let room = e.global::<u32>(NPOS).wrapping_sub(source_len);
    if room <= e.mem.u32(this + 0x14).wrapping_sub(count) {
        e.call(THROW_LENGTH_ERROR, &args![]);
    }
    let tail = e.mem.u32(this + 0x14).wrapping_sub(count).wrapping_sub(pos);
    let new_size = e
        .mem
        .u32(this + 0x14)
        .wrapping_add(source_len)
        .wrapping_sub(count);
    if e.mem.u32(this + 0x14) < new_size {
        e.call(STD_STRING_GROW, &args![this, new_size, 0u32]);
    }
    let after_new = pos.wrapping_add(source_len);
    let after_old = pos.wrapping_add(count);
    if this != source {
        move_chars(e, this, after_new, this, after_old, tail);
        copy_chars(e, this, pos, source, source_pos, source_len);
        e.call(STD_STRING_SET_SIZE, &args![this, new_size]);
        return this;
    }
    if source_len <= count {
        move_chars(e, this, pos, this, source_pos, source_len);
        move_chars(e, this, after_new, this, after_old, tail);
    } else if source_pos <= pos {
        move_chars(e, this, after_new, this, after_old, tail);
        move_chars(e, this, pos, this, source_pos, source_len);
    } else if after_old <= source_pos {
        move_chars(e, this, after_new, this, after_old, tail);
        let from = source_pos.wrapping_add(source_len).wrapping_sub(count);
        move_chars(e, this, pos, this, from, source_len);
    } else {
        move_chars(e, this, pos, this, source_pos, count);
        move_chars(e, this, after_new, this, after_old, tail);
        let from = source_pos.wrapping_add(source_len);
        move_chars(
            e,
            this,
            after_old,
            this,
            from,
            source_len.wrapping_sub(count),
        );
    }
    e.call(STD_STRING_SET_SIZE, &args![this, new_size]);
    this
}

// Translated from 007ab590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<MessageMenuEntry *>` instance (by its
/// vtable, whose RTTI names it): sets the vtable and runs the base
/// constructor.
pub fn fn_007ab590(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, ENTRY_ARRAY_VTABLE);
    e.call(ARRAY_CONSTRUCT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 007ab5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<MessageMenu::MessageMenuEntry *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs `fn_007ab5f0` and frees the array when bit 0 of `flags`
/// is set.
pub fn bs_simple_array_message_menu_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    fn_007ab5f0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007ab5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the entry array: restores the vtable and clears it.
pub fn fn_007ab5f0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, ENTRY_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 007ab610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<PerkActivation *>` instance (by its
/// vtable's RTTI).
pub fn fn_007ab610(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, PERK_ARRAY_VTABLE);
    e.call(ARRAY_CONSTRUCT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 007ab640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<MessageMenu::PerkActivation *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor body `007ab670` (another unit) and
/// frees the array when bit 0 of `flags` is set.
pub fn bs_simple_array_perk_activation_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(0x007a_b670, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007a8850,
            fn_007a8850(Ptr<MessageMenuEntry>) -> Ptr<MessageMenuEntry>
        ),
        entry!(0x007a8960, fn_007a8960(Ptr<MessageMenuEntry>)),
        entry!(0x007a8aa0, message_menu_get_class(Ptr<MessageMenu>) -> u32),
        entry!(
            0x007a8ae0,
            message_menu_scalar_deleting_destructor(Ptr<MessageMenu>, u32) -> Ptr<MessageMenu>
        ),
        entry!(0x007a8b10, fn_007a8b10(Ptr<MessageMenu>)),
        entry!(0x007a8ba0, message_menu_create() -> bool),
        entry!(0x007a8de0, fn_007a8de0() -> u32),
        entry!(0x007a8df0, message_menu_close()),
        entry!(
            0x007a8e60,
            fn_007a8e60(u32, u32, u32, u32, u32, u8, u32, u32, u32, bool, i32, i32, f32, f32, f32) -> bool
        ),
        entry!(0x007a9280, fn_007a9280(Ptr<MessageMenuEntry>) -> u8),
        entry!(0x007a92a0, fn_007a92a0() -> Ptr<MessageMenuEntry>),
        entry!(0x007a92e0, message_menu_load_next_message(Ptr<MessageMenu>)),
        entry!(
            0x007aa070,
            message_menu_do_click(Ptr<MessageMenu>, i32, u32)
        ),
        entry!(
            0x007aa110,
            message_menu_do_key(Ptr<MessageMenu>, u32) -> bool
        ),
        entry!(0x007aa2a0, message_menu_do_idle(Ptr<MessageMenu>)),
        entry!(0x007aa480, fn_007aa480()),
        entry!(
            0x007aa500,
            fn_007aa500(Ptr<MessageMenuEntry>, u32) -> Ptr<MessageMenuEntry>
        ),
        entry!(0x007aa530, message_menu_clear_message_queue_head()),
        entry!(0x007aa5c0, fn_007aa5c0()),
        entry!(0x007aa6a0, fn_007aa6a0(u8)),
        entry!(0x007aa6b0, message_menu_disable_message_menu(u8)),
        entry!(0x007aa720, fn_007aa720(u32, u32) -> bool),
        entry!(0x007aae70, fn_007aae70()),
        entry!(0x007aaf20, message_menu_set_pause_menu_mode(u8)),
        entry!(0x007aaf80, fn_007aaf80(u32, u32, u32, u32) -> u32),
        entry!(0x007aafb0, fn_007aafb0(u32, u32) -> u32),
        entry!(0x007aafe0, fn_007aafe0(u32, u32, u32) -> u32),
        entry!(0x007ab000, fn_007ab000(u32, u32, u32, u32) -> u32),
        entry!(0x007ab050, fn_007ab050(u32, u32, u32, u32, u32) -> u32),
        entry!(0x007ab0a0, fn_007ab0a0(u32, u32, u32, u32, u32) -> u32),
        entry!(0x007ab230, fn_007ab230(u32, u32) -> u32),
        entry!(0x007ab250, fn_007ab250(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x007ab590, fn_007ab590(u32) -> u32),
        entry!(
            0x007ab5c0,
            bs_simple_array_message_menu_entry_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x007ab5f0, fn_007ab5f0(u32)),
        entry!(0x007ab610, fn_007ab610(u32) -> u32),
        entry!(
            0x007ab640,
            bs_simple_array_perk_activation_scalar_deleting_destructor(u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;

    type Appends = Rc<RefCell<Vec<(u32, String)>>>;
    type Lists = Rc<RefCell<HashMap<u32, (u32, u32)>>>;

    /// Pages holding the globals and constants the code reads.
    const PAGES: [u32; 15] = [
        0x0101_1000,
        0x0101_2000,
        0x0101_6000,
        0x0101_7000,
        0x0101_e000,
        0x0102_0000,
        0x0106_3000,
        0x0107_0000,
        0x0107_1000,
        0x0107_2000,
        0x0119_f000,
        0x011d_5000,
        0x011d_a000,
        0x011d_e000,
        0x0120_2000,
    ];

    /// Every callee outside this file; each gets a blank double first.
    const EXTERNALS: &[u32] = &[
        OPERATOR_NEW,
        OPERATOR_DELETE,
        INTERFACE_SINGLETON,
        INTERFACE_TILE_ROOT,
        TILE_READER,
        TILE_READ_FILE,
        TILE_GET_MENU,
        TILE_GET_MENU_BY_CLASS,
        GET_CHILD_BY_NAME,
        TEXT_TO_TRAIT,
        GET_FLOAT_TRAIT,
        GET_STRING_TRAIT,
        IS_TRUE_TRAIT,
        SET_FLOAT_TRAIT,
        SET_STRING_TRAIT,
        SET_TRAIT_FROM_INT,
        GET_MAXIMUM_DEPTH,
        WORD_AT_FOUR,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        MENU_INSTANT_FADE_OUT,
        MENU_START_FADE_IN,
        MENU_BASE_DESTRUCTOR,
        LIST_BOX_DESTRUCTOR,
        LIST_BOX_INITIALIZE,
        LIST_BOX_ADD_ITEM,
        LIST_BOX_SYNCHRONIZE,
        LIST_BOX_COUNT,
        LIST_BOX_FIRST_ITEM,
        LIST_BOX_VALUE_OF_TILE,
        TIMER_UPDATE,
        WORD_AT_0X14,
        WORD_AT_0X24,
        ARRAY_SIZE,
        ARRAY_ELEMENT,
        ARRAY_ELEMENT_ALIAS,
        ARRAY_LAST_ELEMENT,
        ARRAY_REMOVE,
        ARRAY_INSERT,
        ARRAY_ADD,
        ARRAY_CLEAR,
        ARRAY_CONSTRUCT,
        STRING_CONSTRUCT,
        STRING_DESTROY,
        STRING_FROM_C_STRING,
        STRING_ASSIGN,
        STRING_APPEND,
        STRING_LENGTH,
        STRING_C_STR,
        COPY_C_STRING,
        COMPARE_C_STRINGS,
        C_STRING_LENGTH,
        TO_UPPER,
        STD_STRING_FROM_C_STRING,
        STD_STRING_DESTROY,
        STD_STRING_C_STR,
        STD_STRING_DATA,
        STD_STRING_INIT,
        STD_STRING_RESET,
        STD_STRING_ASSIGN_SUBSTR,
        STD_STRING_FIND_C_STRING,
        STD_STRING_FIND_CHARS,
        STD_STRING_INSIDE,
        STD_STRING_GROW,
        STD_STRING_SET_SIZE,
        ALLOCATOR_COPY,
        MEMORY_MOVE,
        MEMORY_COPY,
        THROW_OUT_OF_RANGE,
        THROW_LENGTH_ERROR,
        STRING_INDEX_ASSERT,
        SETTING_VALUE,
        SETTING_INT_POINTER,
        SETTING_FLOAT_POINTER,
        GET_TOP_MENU_ID,
        GET_MESSAGE_MENU_RESULT,
        IN_DIALOG,
        SET_DIALOG_PAUSE,
        ENTER_STACK_CONTAINS,
        ADD_TO_ENTER_STACK,
        POP_FROM_ENTER_STACK,
        GET_CURRENT_RENDERED_MENU,
        QUEUE_MENU_CREATE,
        SET_GUN_SCOPE_VISIBLE,
        CLOSE_CHECK,
        MENU_INITIALIZED_CHECK,
        ERROR_PRINT,
        SOUND_SYSTEM,
        GET_SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_DIFFERS,
        SOUND_HANDLE_DESTROY,
        FONT_MANAGER,
        CALCULATE_STRING_DIMENSIONS,
        FLOAT_MAX_OF,
        FLOAT_MIN_OF,
        INT_MAX_OF,
        FTOL,
        SCREEN_WIDTH_FLOAT,
        SCREEN_HEIGHT_FLOAT,
        SCREEN_WIDTH_INT,
        SCREEN_HEIGHT_INT,
        MARGIN_WIDTH,
        MARGIN_HEIGHT,
        SCRIPT_RUN,
        REFR_ACTIVATE,
        REFR_COUNT,
        REFR_GET_TARGET_TYPE,
        REFR_BASE_FORM,
        FORM_TYPE,
        REFR_GET_LOCK,
        REFR_CHECK_0057_9690,
        LOCK_CHECK,
        REFR_IS_AN_OWNER,
        DOOR_CAN_IGNORE_LOCK,
        OPEN_CLOSE_STATE,
        SCRIPT_OF_ITEM,
        NODE_IS_END,
        NODE_ITEM_SLOT,
    ];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn fret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
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

    fn put_cstr(e: &mut Engine, text: &str) -> u32 {
        let p = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(p, text.as_bytes());
        p
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in PAGES {
            e.map(page, 0x1000);
        }
        e.set_global(NPOS, u32::MAX);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(HUNDRED_FLOAT, 100.0f32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(TWO_FIFTY_FIVE_DOUBLE, 255.0f64);
        e.set_global(ONE_TWENTY_DOUBLE, 120.0f64);
        e.set_global(EIGHTY_DOUBLE, 80.0f64);
        e.set_global(KIND_ONE_DOUBLE, 6006.0f64);
        e.set_global(KIND_TWO_DOUBLE, 102.0f64);
        e.mem.set_cstr(SPACE_STRING, b" ");
        e.mem.set_cstr(SEMICOLON_STRING, b";");
        for addr in EXTERNALS {
            stub(&mut e, *addr, 0);
        }
        stub(&mut e, 0x00aa_4c40, 0);
        stub(&mut e, 0x007a_b670, 0);
        e.register(FLOAT_MAX_OF, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            fret((if y < x { x } else { y }) as f64)
        });
        e.register(FLOAT_MIN_OF, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            fret((if y > x { x } else { y }) as f64)
        });
        e.register(INT_MAX_OF, |_, a| {
            ret(if (a[0] as i32) > (a[1] as i32) {
                a[0]
            } else {
                a[1]
            })
        });
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.call_log = Some(vec![]);
        e
    }

    /// Vtables of the menu (0x0b000000) and of its list box (0x0b100000);
    /// every slot is a blank double except `GetClass` (0x3e9) and the list's
    /// slot 0x14 (the item 0x9999).
    fn vtables(e: &mut Engine) {
        let menu: Vec<u32> = (0..20).map(|i| 0x0ac0_0000 + i).collect();
        let list: Vec<u32> = (0..20).map(|i| 0x0ac1_0000 + i).collect();
        e.put_vtable(0x0b00_0000, &menu);
        e.put_vtable(0x0b10_0000, &list);
        for address in menu.iter().chain(list.iter()) {
            stub(e, *address, 0);
        }
        stub(e, 0x0ac0_000d, 0x3e9);
        stub(e, 0x0ac1_0005, 0x9999);
    }

    /// A menu object with six tiles `0x6100 + 0x100 * i` and a list vtable.
    fn menu(e: &mut Engine) -> Ptr<MessageMenu> {
        let m: Ptr<MessageMenu> = e.new_object();
        e.mem.set_u32(m.addr(), 0x0b00_0000);
        for i in 0..6 {
            e.mem.set_u32(m.addr() + 0x28 + 4 * i, 0x6100 + 0x100 * i);
        }
        e.mem.set_u32(m.addr() + 0x40, 0x0b10_0000);
        m
    }

    fn entry(e: &mut Engine) -> Ptr<MessageMenuEntry> {
        e.new_object()
    }

    fn set_text(e: &mut Engine, object: u32, offset: u32, text: &str) {
        let p = put_cstr(e, text);
        e.mem.set_u32(object + offset, p);
    }

    /// Doubles that keep the queue and the activation list in memory.
    fn arrays(e: &mut Engine) -> Lists {
        let lists: Lists = Rc::default();
        let l = lists.clone();
        e.register_double(ARRAY_SIZE, move |_, a| {
            ret(l.borrow().get(&a[0]).map_or(0, |v| v.1))
        });
        for address in [ARRAY_ELEMENT, ARRAY_ELEMENT_ALIAS] {
            let l = lists.clone();
            e.register_double(address, move |_, a| ret(l.borrow()[&a[0]].0 + 4 * a[1]));
        }
        let l = lists.clone();
        e.register_double(ARRAY_LAST_ELEMENT, move |_, a| {
            let (block, len) = l.borrow()[&a[0]];
            ret(block + 4 * (len - 1))
        });
        let l = lists.clone();
        e.register_double(ARRAY_REMOVE, move |e, a| {
            let (block, len) = l.borrow()[&a[0]];
            for i in a[1]..len - 1 {
                let value = e.mem.u32(block + 4 * (i + 1));
                e.mem.set_u32(block + 4 * i, value);
            }
            l.borrow_mut().get_mut(&a[0]).unwrap().1 = len - 1;
            Ret::default()
        });
        let l = lists.clone();
        e.register_double(ARRAY_INSERT, move |e, a| {
            let (block, len) = l.borrow()[&a[0]];
            for i in (a[1]..len).rev() {
                let value = e.mem.u32(block + 4 * i);
                e.mem.set_u32(block + 4 * (i + 1), value);
            }
            let item = e.mem.u32(a[2]);
            e.mem.set_u32(block + 4 * a[1], item);
            l.borrow_mut().get_mut(&a[0]).unwrap().1 = len + 1;
            Ret::default()
        });
        let l = lists.clone();
        e.register_double(ARRAY_ADD, move |e, a| {
            let (block, len) = l.borrow()[&a[0]];
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(block + 4 * len, item);
            l.borrow_mut().get_mut(&a[0]).unwrap().1 = len + 1;
            Ret::default()
        });
        let l = lists.clone();
        e.register_double(ARRAY_CLEAR, move |_, a| {
            l.borrow_mut().get_mut(&a[0]).unwrap().1 = 0;
            Ret::default()
        });
        lists
    }

    fn set_list(lists: &Lists, e: &mut Engine, list: u32, items: &[u32]) {
        let block = e.mem.alloc(0x100);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *item);
        }
        lists.borrow_mut().insert(list, (block, items.len() as u32));
    }

    fn list_items(lists: &Lists, e: &Engine, list: u32) -> Vec<u32> {
        let (block, len) = lists.borrow()[&list];
        (0..len).map(|i| e.mem.u32(block + 4 * i)).collect()
    }

    /// Doubles for the C strings, `BSStringT` (a pointer to the text at +0)
    /// and a `std::string` (text pointer at +4, size at +0x14, capacity at
    /// +0x18). Returns the log of `BSStringT` appends.
    fn strings(e: &mut Engine) -> Appends {
        let appends: Appends = Rc::default();
        let log = appends.clone();
        e.register_double(STRING_APPEND, move |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            log.borrow_mut().push((a[0], text));
            Ret::default()
        });
        for address in [STRING_FROM_C_STRING, STRING_ASSIGN] {
            e.register(address, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
        }
        e.register(STRING_C_STR, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRING_LENGTH, |e, a| {
            let p = e.mem.u32(a[0]);
            ret(if p == 0 {
                0
            } else {
                e.mem.cstr(p).len() as u32
            })
        });
        e.register(C_STRING_LENGTH, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(COPY_C_STRING, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(COMPARE_C_STRINGS, |e, a| {
            ret(match e.mem.cstr(a[0]).cmp(&e.mem.cstr(a[1])) {
                std::cmp::Ordering::Less => u32::MAX,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            })
        });
        e.register(TO_UPPER, |_, a| {
            ret((a[0] as u8).to_ascii_uppercase() as u32)
        });
        e.register(STD_STRING_FROM_C_STRING, |e, a| {
            let text = e.mem.cstr(a[1]);
            fill_std_string(e, a[0], &text);
            Ret::default()
        });
        for address in [STD_STRING_C_STR, STD_STRING_DATA] {
            e.register(address, |e, a| ret(e.mem.u32(a[0] + 4)));
        }
        e.register(WORD_AT_0X14, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        for address in [STD_STRING_FIND_CHARS, STD_STRING_FIND_C_STRING] {
            e.register(address, |e, a| {
                let wanted = e.mem.u8(a[1]);
                let text = e.mem.cstr(e.mem.u32(a[0] + 4));
                let size = e.mem.u32(a[0] + 0x14) as usize;
                let found = text
                    .iter()
                    .take(size)
                    .enumerate()
                    .skip(a[2] as usize)
                    .find(|(_, c)| **c == wanted);
                ret(found.map_or(u32::MAX, |(i, _)| i as u32))
            });
        }
        e.register(STD_STRING_ASSIGN_SUBSTR, |e, a| {
            let text = e.mem.cstr(e.mem.u32(a[1] + 4));
            let start = (a[2] as usize).min(text.len());
            let end = start.saturating_add(a[3] as usize).min(text.len());
            fill_std_string(e, a[0], &text[start..end]);
            Ret::default()
        });
        e.register(STD_STRING_INSIDE, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            ret((a[1] >= base && a[1] <= base + e.mem.u32(a[0] + 0x14)) as u32)
        });
        stub(e, STD_STRING_GROW, 1);
        e.register(STD_STRING_SET_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 0x14, a[1]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u8(base + a[1], 0);
            Ret::default()
        });
        e.register(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        // `0044d580` takes the ECX it is entered with first.
        e.register(MEMORY_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[3], a[4]);
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        appends
    }

    fn fill_std_string(e: &mut Engine, object: u32, text: &[u8]) {
        let buffer = e.mem.alloc(0x40);
        e.mem.set_cstr(buffer, text);
        e.mem.set_u32(object + 4, buffer);
        e.mem.set_u32(object + 0x14, text.len() as u32);
        e.mem.set_u32(object + 0x18, 0x3f);
    }

    fn std_string(e: &mut Engine, text: &str) -> u32 {
        let object = e.mem.alloc(0x20);
        fill_std_string(e, object, text.as_bytes());
        object
    }

    fn std_text(e: &Engine, object: u32) -> String {
        String::from_utf8(e.mem.cstr(e.mem.u32(object + 4))).unwrap()
    }

    // ----- 007a8850 -----

    #[test]
    fn entry_constructor_sets_defaults_and_reads_the_opacity() {
        let mut e = engine();
        stub(&mut e, TEXT_TO_TRAIT, 0x55);
        stub(&mut e, INTERFACE_SINGLETON, 0x9000);
        stub(&mut e, INTERFACE_TILE_ROOT, 0x9100);
        e.register(GET_FLOAT_TRAIT, |_, _| fret(0.5));
        let object = entry(&mut e);
        assert_eq!(e.call(0x007a_8850, &args![object]).u32(), object.addr());
        assert_eq!(e.get(object, MessageMenuEntry::bIsModal), 1);
        assert_eq!(e.get(object, MessageMenuEntry::iXPosOverride), -1);
        assert_eq!(e.get(object, MessageMenuEntry::iYPosOverride), -1);
        assert_eq!(
            e.get(object, MessageMenuEntry::fBackgroundTextureOpacity),
            0.5
        );
        assert_eq!(calls(&e, STRING_CONSTRUCT).len(), 5);
        assert_eq!(calls(&e, GET_FLOAT_TRAIT), vec![vec![0x9100, 0x55]]);
    }

    // ----- 007a8960 -----

    #[test]
    fn entry_destructor_frees_the_title_and_destroys_the_strings_in_reverse() {
        let mut e = engine();
        let object = entry(&mut e);
        e.set(object, MessageMenuEntry::strTitle, Ptr::new(0x4444));
        e.call(0x007a_8960, &args![object]);
        let base = object.addr();
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x4444]]);
        assert_eq!(e.get(object, MessageMenuEntry::strTitle), Ptr::NULL);
        let order: Vec<u32> = calls(&e, STRING_DESTROY).iter().map(|w| w[0]).collect();
        assert_eq!(
            order,
            vec![base + 0x30, base + 0x20, base + 0x14, base + 0xc, base]
        );
    }

    // ----- 007a8aa0 / 007a8de0 -----

    #[test]
    fn the_class_id_is_0x3e9() {
        let mut e = engine();
        assert_eq!(e.call(0x007a_8aa0, &args![0u32]).u32(), 0x3e9);
        assert_eq!(e.call(0x007a_8de0, &args![]).u32(), 0x3e9);
    }

    // ----- 007a8ae0 / 007a8b10 -----

    #[test]
    fn destructor_clears_the_singleton_and_frees_on_flag() {
        let mut e = engine();
        let m = menu(&mut e);
        e.set_global(MESSAGE_MENU, m.addr());
        e.set_global(CHECK_FOR_DELETION, 1u8);
        e.call(0x007a_8ae0, &args![m, 0u32]);
        assert_eq!(e.global::<u32>(MESSAGE_MENU), 0);
        assert_eq!(e.mem.u32(m.addr()), MESSAGE_MENU_VTABLE);
        assert_eq!(e.global::<u8>(WARNING_STATE_FLAG_0119F348), 1);
        assert_eq!(e.global::<u8>(DELETED_DURING_CHECK), 1);
        assert_eq!(calls(&e, LIST_BOX_DESTRUCTOR), vec![vec![m.addr() + 0x40]]);
        assert_eq!(calls(&e, MENU_BASE_DESTRUCTOR), vec![vec![m.addr()]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x007a_8ae0, &args![m, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![m.addr()]]);
    }

    // ----- 007a8ba0 -----

    fn create_world(initialized: bool, class: u32) -> (Engine, Ptr<MessageMenu>) {
        let mut e = engine();
        vtables(&mut e);
        stub(&mut e, 0x0ac0_000d, class);
        let m = menu(&mut e);
        let float_cell = e.mem.alloc(4);
        e.mem.set_f32(float_cell, 0.5);
        let found = m.addr();
        stub(&mut e, INTERFACE_SINGLETON, 1);
        stub(&mut e, TILE_READ_FILE, 0x5000);
        stub(&mut e, TILE_GET_MENU, found);
        stub(&mut e, MENU_INITIALIZED_CHECK, initialized as u32);
        stub(&mut e, GET_TOP_MENU_ID, 1);
        e.register(GET_FLOAT_TRAIT, |_, _| fret(6006.0));
        e.register(GET_MAXIMUM_DEPTH, |_, _| fret(7.5));
        stub(&mut e, SETTING_FLOAT_POINTER, float_cell);
        stub(&mut e, WORD_AT_FOUR, 0x6000);
        stub(&mut e, GET_CHILD_BY_NAME, 0x6100);
        stub(&mut e, IN_DIALOG, 1);
        (e, m)
    }

    #[test]
    fn create_builds_the_menu_and_applies_the_depth_and_opacity() {
        let (mut e, m) = create_world(true, 0x3e9);
        assert!(e.call(0x007a_8ba0, &args![]).bool());
        assert_eq!(e.global::<u32>(MESSAGE_MENU), m.addr());
        let set = calls(&e, SET_FLOAT_TRAIT);
        assert!(set.contains(&vec![0x5000, 0xfad, 7.5f32.to_bits(), 1]));
        assert!(set.contains(&vec![0x6100, 0xfa9, 127.5f32.to_bits(), 1]));
        assert_eq!(
            calls(&e, LIST_BOX_INITIALIZE),
            vec![vec![m.addr() + 0x40, 0x6500, BUTTON_TEMPLATE]]
        );
        assert_eq!(calls(&e, SET_DIALOG_PAUSE), vec![vec![1]]);
        assert_eq!(calls(&e, SET_GUN_SCOPE_VISIBLE), vec![vec![0]]);
    }

    #[test]
    fn create_reports_a_menu_that_fails_its_check() {
        let (mut e, _) = create_world(false, 0x3e9);
        assert!(!e.call(0x007a_8ba0, &args![]).bool());
        assert_eq!(calls(&e, ERROR_PRINT), vec![vec![CREATION_FAILED]]);
    }

    #[test]
    fn create_deletes_an_object_of_another_class() {
        let (mut e, m) = create_world(true, 5);
        assert!(!e.call(0x007a_8ba0, &args![]).bool());
        assert_eq!(calls(&e, 0x0ac0_0000), vec![vec![m.addr(), 1]]);
    }

    // ----- 007a8df0 -----

    #[test]
    fn close_fades_out_and_ends_the_dialog_pause() {
        let mut e = engine();
        let m = menu(&mut e);
        e.set_global(MESSAGE_MENU, m.addr());
        stub(&mut e, WORD_AT_FOUR, 0x6000);
        stub(&mut e, IN_DIALOG, 1);
        e.call(0x007a_8df0, &args![]);
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT), vec![vec![0x6000, 0x1772, 1]]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![m.addr()]]);
        assert_eq!(calls(&e, SET_DIALOG_PAUSE), vec![vec![0]]);
        stub(&mut e, GET_CURRENT_RENDERED_MENU, 1);
        e.call(0x007a_8df0, &args![]);
        assert_eq!(calls(&e, MENU_INSTANT_FADE_OUT), vec![vec![m.addr()]]);
        e.set_global(MESSAGE_MENU, 0u32);
        e.call(0x007a_8df0, &args![]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT).len(), 1);
    }

    // ----- 007a8e60 -----

    struct Message {
        first_button: &'static str,
        buttons: Vec<&'static str>,
        modal: bool,
    }

    fn add_message(e: &mut Engine, message: &Message) -> bool {
        let text = put_cstr(e, "Hello");
        let icon = put_cstr(e, "icon.dds");
        let title = put_cstr(e, "Title");
        let first = put_cstr(e, message.first_button);
        let array = e.mem.alloc(0x20);
        for (i, button) in message.buttons.iter().enumerate() {
            let p = put_cstr(e, button);
            e.mem.set_u32(array + 4 * i as u32, p);
        }
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, array);
        e.call(
            0x007a_8e60,
            &args![
                text,
                icon,
                title,
                0u32,
                0x1234u32,
                2u8,
                3u32,
                first,
                cursor,
                message.modal,
                10i32,
                20i32,
                2.5f32,
                0.5f32,
                300.7f32
            ],
        )
        .bool()
    }

    fn message_world() -> (Engine, Lists, Appends) {
        let mut e = engine();
        let appends = strings(&mut e);
        let lists = arrays(&mut e);
        set_list(&lists, &mut e, ENTRY_LIST, &[]);
        let two = e.mem.alloc(4);
        e.mem.set_i32(two, 2);
        stub(&mut e, SETTING_INT_POINTER, two);
        (e, lists, appends)
    }

    #[test]
    fn add_message_fills_the_entry_and_queues_the_menu_creation() {
        let (mut e, lists, _) = message_world();
        let plain = Message {
            first_button: "",
            buttons: vec![],
            modal: false,
        };
        assert!(add_message(&mut e, &plain));
        let items = list_items(&lists, &e, ENTRY_LIST);
        assert_eq!(items.len(), 1);
        let object = Ptr::<MessageMenuEntry>::new(items[0]);
        assert_eq!(e.get(object, MessageMenuEntry::fpResultCallback), 0x1234);
        assert_eq!(e.get(object, MessageMenuEntry::cFirstButtonResult), 2);
        assert_eq!(e.get(object, MessageMenuEntry::iWarningContext), 3);
        assert_eq!(e.get(object, MessageMenuEntry::bIsModal), 0);
        assert_eq!(e.get(object, MessageMenuEntry::iXPosOverride), 10);
        assert_eq!(e.get(object, MessageMenuEntry::iYPosOverride), 20);
        assert_eq!(
            e.get(object, MessageMenuEntry::fMillisecondsVisible),
            2500.0
        );
        assert_eq!(
            e.get(object, MessageMenuEntry::fBackgroundTextureOpacity),
            127.5
        );
        assert_eq!(e.get(object, MessageMenuEntry::uiWidthOverride), 300);
        let title = e.get(object, MessageMenuEntry::strTitle);
        assert_eq!(e.mem.cstr(title.addr()), b"Title");
        assert_eq!(calls(&e, QUEUE_MENU_CREATE), vec![vec![7, 0, 0, 0, 1, 1]]);
    }

    #[test]
    fn add_message_moves_hotkeys_out_of_the_button_texts() {
        let (mut e, _, appends) = message_world();
        let buttons = Message {
            first_button: "&OK",
            buttons: vec!["Cancel"],
            modal: false,
        };
        assert!(add_message(&mut e, &buttons));
        let log = appends.borrow().clone();
        let object = log[0].0 - 0x20;
        let expected: Vec<(u32, String)> = vec![
            (object + 0x20, "O".to_string()),
            (object + 0x14, "OK".to_string()),
            (object + 0x14, ";".to_string()),
            (object + 0x14, "Cancel".to_string()),
            (object + 0x14, ";".to_string()),
        ];
        assert_eq!(log, expected);
    }

    #[test]
    fn add_message_puts_a_modal_entry_before_the_first_modeless_one() {
        let (mut e, lists, _) = message_world();
        let first = entry(&mut e);
        let second = entry(&mut e);
        set_list(&lists, &mut e, ENTRY_LIST, &[first.addr(), second.addr()]);
        let modal = Message {
            first_button: "",
            buttons: vec![],
            modal: true,
        };
        add_message(&mut e, &modal);
        let items = list_items(&lists, &e, ENTRY_LIST);
        assert_eq!(items.len(), 3);
        assert_eq!(&items[1..], &[first.addr(), second.addr()]);
        // A modeless entry behind a modal one goes to the end.
        let (mut e, lists, _) = message_world();
        let existing = entry(&mut e);
        e.set(existing, MessageMenuEntry::bIsModal, 1);
        set_list(&lists, &mut e, ENTRY_LIST, &[existing.addr()]);
        let plain = Message {
            first_button: "",
            buttons: vec![],
            modal: false,
        };
        add_message(&mut e, &plain);
        let items = list_items(&lists, &e, ENTRY_LIST);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], existing.addr());
    }

    // ----- 007a9280 / 007a92a0 -----

    #[test]
    fn current_entry_is_the_element_at_the_start_index() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let first = entry(&mut e);
        let second = entry(&mut e);
        e.set(second, MessageMenuEntry::bIsModal, 1);
        set_list(&lists, &mut e, ENTRY_LIST, &[first.addr(), second.addr()]);
        e.set_global(START_INDEX, 1u32);
        let current = e.call(0x007a_92a0, &args![]).u32();
        assert_eq!(current, second.addr());
        assert_eq!(e.call(0x007a_9280, &args![current]).u8(), 1);
        e.set_global(START_INDEX, 2u32);
        assert_eq!(e.call(0x007a_92a0, &args![]).u32(), 0);
    }

    // ----- 007a92e0 -----

    fn load_world(modal: bool) -> (Engine, Ptr<MessageMenu>, Ptr<MessageMenuEntry>) {
        let mut e = engine();
        vtables(&mut e);
        strings(&mut e);
        let lists = arrays(&mut e);
        let m = menu(&mut e);
        e.set_global(MESSAGE_MENU, m.addr());
        let object = entry(&mut e);
        set_text(&mut e, object.addr(), 0xc, "msg");
        e.set(object, MessageMenuEntry::bIsModal, modal as u8);
        e.set(object, MessageMenuEntry::iWarningContext, 0x17);
        e.set(object, MessageMenuEntry::fBackgroundTextureOpacity, 0.25);
        e.set(object, MessageMenuEntry::iXPosOverride, -1);
        e.set(object, MessageMenuEntry::iYPosOverride, -1);
        set_list(&lists, &mut e, ENTRY_LIST, &[object.addr()]);
        e.register(TEXT_TO_TRAIT, |_, a| ret(a[0]));
        stub(&mut e, INTERFACE_SINGLETON, 5);
        stub(&mut e, WORD_AT_FOUR, 0x6000);
        e.register(GET_FLOAT_TRAIT, |_, a| {
            fret(match a[1] {
                HORIZONTAL_BUFFER_NAME => 10.0,
                MAX_MENU_WIDTH_NAME => 500.0,
                MIN_MENU_WIDTH_NAME => 100.0,
                0xfb1 => 220.0,
                0xfb0 => 100.0,
                _ => 0.0,
            })
        });
        e.register(CALCULATE_STRING_DIMENSIONS, |e, a| {
            e.mem.set_f32(a[1], 200.0);
            e.mem.set_f32(a[1] + 4, 20.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            ret(a[1])
        });
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| ret(a[1]));
        e.register(SCREEN_WIDTH_FLOAT, |_, _| fret(1000.0));
        e.register(SCREEN_HEIGHT_FLOAT, |_, _| fret(800.0));
        e.register(SCREEN_WIDTH_INT, |_, _| fret(800.0));
        e.register(SCREEN_HEIGHT_INT, |_, _| fret(600.0));
        e.register(MARGIN_WIDTH, |_, _| fret(10.0));
        e.register(MARGIN_HEIGHT, |_, _| fret(10.0));
        (e, m, object)
    }

    #[test]
    fn load_next_message_sizes_and_centers_a_modeless_message() {
        let (mut e, m, object) = load_world(false);
        e.call(0x007a_92e0, &args![m]);
        let sound = calls(&e, GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(sound[0][2], POPUP_SOUND_NAME);
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY).len(), 1);
        let message = e.mem.u32(object.addr() + 0xc);
        let texts = calls(&e, SET_STRING_TRAIT);
        assert_eq!(texts[0], vec![0x6400, 0xfc4, message, 1]);
        let set = calls(&e, SET_FLOAT_TRAIT);
        assert!(set.contains(&vec![0x6100, 0xfb1, 220.0f32.to_bits(), 1]));
        assert!(set.contains(&vec![0x6100, 0xfa1, 390.0f32.to_bits(), 1]));
        assert!(set.contains(&vec![0x6100, 0xfa2, 350.0f32.to_bits(), 1]));
        assert!(set.contains(&vec![0x6600, 0xfa9, 0.25f32.to_bits(), 1]));
        let ints = calls(&e, SET_TRAIT_FROM_INT);
        assert!(ints.contains(&vec![0x6200, 0xfb7, 1]));
        assert_eq!(e.global::<u8>(WARNING_STATE_FLAG_0119F348), 1);
    }

    #[test]
    fn load_next_message_keeps_a_position_override_on_screen() {
        let (mut e, m, object) = load_world(false);
        e.set(object, MessageMenuEntry::iXPosOverride, 500);
        e.call(0x007a_92e0, &args![m]);
        let set = calls(&e, SET_FLOAT_TRAIT);
        assert!(set.contains(&vec![0x6100, 0xfa1, 625.0f32.to_bits(), 1]));
    }

    #[test]
    fn load_next_message_puts_a_modal_message_on_the_enter_stack() {
        let (mut e, m, _) = load_world(true);
        e.call(0x007a_92e0, &args![m]);
        let set = calls(&e, SET_FLOAT_TRAIT);
        assert!(set.contains(&vec![0x6100, 0xfa9, 1.0f32.to_bits(), 1]));
        assert_eq!(calls(&e, ADD_TO_ENTER_STACK), vec![vec![0x3e9, 0]]);
        assert!(!set.iter().any(|w| w[1] == 0xfa1));
    }

    // ----- 007aa070 -----

    #[test]
    fn do_click_on_a_list_button_runs_the_callback_and_clears_the_head() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let m = menu(&mut e);
        let object = entry(&mut e);
        e.set(object, MessageMenuEntry::fpResultCallback, 0x0aaa_0000);
        set_list(&lists, &mut e, ENTRY_LIST, &[object.addr()]);
        let singleton = e.mem.alloc(0x100);
        stub(&mut e, INTERFACE_SINGLETON, singleton);
        stub(&mut e, LIST_BOX_VALUE_OF_TILE, 3);
        let during = Rc::new(Cell::new(0u8));
        let seen = during.clone();
        e.register_double(0x0aaa_0000, move |e, _| {
            seen.set(e.global::<u8>(CHECK_FOR_DELETION));
            Ret::default()
        });
        e.call(0x007a_a070, &args![m, 7i32, 0x6500u32]);
        assert_eq!(during.get(), 1);
        assert_eq!(e.mem.u8(singleton + 0xe4), 3);
        assert_eq!(e.global::<u8>(CHECK_FOR_DELETION), 0);
        assert_eq!(calls(&e, ARRAY_REMOVE), vec![vec![ENTRY_LIST, 0, 1]]);
        // A callback that deleted the menu leaves the queue alone.
        let object = entry(&mut e);
        e.set(object, MessageMenuEntry::fpResultCallback, 0x0aaa_0001);
        set_list(&lists, &mut e, ENTRY_LIST, &[object.addr()]);
        e.register(0x0aaa_0001, |e, _| {
            e.set_global(DELETED_DURING_CHECK, 1u8);
            Ret::default()
        });
        e.call(0x007a_a070, &args![m, 7i32, 0x6500u32]);
        assert_eq!(calls(&e, ARRAY_REMOVE).len(), 1);
        assert_eq!(e.global::<u8>(DELETED_DURING_CHECK), 0);
    }

    // ----- 007aa110 -----

    #[test]
    fn do_key_clicks_buttons_for_escape_space_and_hotkeys() {
        let mut e = engine();
        vtables(&mut e);
        strings(&mut e);
        let lists = arrays(&mut e);
        let m = menu(&mut e);
        let object = entry(&mut e);
        set_text(&mut e, object.addr(), 0x20, "ab");
        set_list(&lists, &mut e, ENTRY_LIST, &[object.addr()]);
        stub(&mut e, LIST_BOX_COUNT, 3);
        stub(&mut e, LIST_BOX_FIRST_ITEM, 0x8888);
        let list = m.addr() + 0x40;
        // Escape: the last button.
        assert!(e.call(0x007a_a110, &args![m, 1u32]).bool());
        assert_eq!(calls(&e, 0x0ac1_0005), vec![vec![list, 2, 0]]);
        assert_eq!(calls(&e, 0x0ac0_0003), vec![vec![m.addr(), 7, 0x9999]]);
        // Space: the first item.
        assert!(e.call(0x007a_a110, &args![m, 0x20u32]).bool());
        assert_eq!(calls(&e, 0x0ac0_0003)[1], vec![m.addr(), u32::MAX, 0x8888]);
        // A hotkey letter, whatever its case.
        assert!(e.call(0x007a_a110, &args![m, 0x42u32]).bool());
        assert_eq!(calls(&e, 0x0ac1_0005)[1], vec![list, 1, 0]);
        // A key that is no hotkey does nothing.
        assert!(!e.call(0x007a_a110, &args![m, 0x5au32]).bool());
        // Escape is ignored for warning context 0x17.
        e.set(object, MessageMenuEntry::iWarningContext, 0x17);
        assert!(!e.call(0x007a_a110, &args![m, 1u32]).bool());
    }

    // ----- 007aa2a0 -----

    struct Idle {
        e: Engine,
        m: Ptr<MessageMenu>,
        time: Rc<Cell<u32>>,
        state: Rc<Cell<u32>>,
    }

    fn idle_world(entries: usize) -> Idle {
        let mut e = engine();
        let lists = arrays(&mut e);
        let m = menu(&mut e);
        let mut items = vec![];
        for _ in 0..entries {
            let object = entry(&mut e);
            e.set(object, MessageMenuEntry::fMillisecondsVisible, 100.0);
            items.push(object.addr());
        }
        set_list(&lists, &mut e, ENTRY_LIST, &items);
        let time = Rc::new(Cell::new(0u32));
        let state = Rc::new(Cell::new(0u32));
        let now = time.clone();
        e.register_double(WORD_AT_0X14, move |_, _| ret(now.get()));
        let fade = state.clone();
        e.register_double(WORD_AT_0X24, move |_, _| ret(fade.get()));
        stub(&mut e, IS_TRUE_TRAIT, 1);
        stub(&mut e, WORD_AT_FOUR, 0x6000);
        e.set_global(IDLE_GUARD, 1u32);
        Idle { e, m, time, state }
    }

    #[test]
    fn do_idle_waits_200_milliseconds_between_checks() {
        let mut w = idle_world(1);
        w.time.set(100);
        w.e.call(0x007a_a2a0, &args![w.m]);
        assert!(calls(&w.e, IS_TRUE_TRAIT).is_empty());
        w.time.set(200);
        w.e.call(0x007a_a2a0, &args![w.m]);
        assert_eq!(calls(&w.e, IS_TRUE_TRAIT).len(), 1);
        assert_eq!(w.e.global::<u32>(IDLE_LAST_TIME), 200);
    }

    #[test]
    fn do_idle_clears_the_last_message_when_its_time_is_up() {
        let mut w = idle_world(1);
        w.state.set(1);
        w.time.set(500);
        w.e.call(0x007a_a2a0, &args![w.m]);
        assert_eq!(w.e.get(w.m, MessageMenu::uiMessageStartTime), 500);
        assert!(calls(&w.e, ARRAY_REMOVE).is_empty());
        w.time.set(700);
        w.e.call(0x007a_a2a0, &args![w.m]);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![0x6000, 0x1772, 1]]
        );
        assert_eq!(calls(&w.e, ARRAY_REMOVE), vec![vec![ENTRY_LIST, 0, 1]]);
    }

    #[test]
    fn do_idle_fades_out_when_more_messages_follow_and_fades_in_at_the_start() {
        let mut w = idle_world(2);
        w.state.set(1);
        w.time.set(500);
        w.e.call(0x007a_a2a0, &args![w.m]);
        w.time.set(700);
        w.e.call(0x007a_a2a0, &args![w.m]);
        assert_eq!(calls(&w.e, MENU_START_FADE_OUT), vec![vec![w.m.addr()]]);
        let mut w = idle_world(1);
        w.state.set(4);
        w.time.set(500);
        w.e.call(0x007a_a2a0, &args![w.m]);
        assert_eq!(calls(&w.e, MENU_START_FADE_IN), vec![vec![w.m.addr()]]);
    }

    // ----- 007aa480 / 007aa500 / 007aa530 / 007aa5c0 -----

    #[test]
    fn clearing_the_queue_deletes_every_entry() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let first = entry(&mut e);
        let second = entry(&mut e);
        set_list(&lists, &mut e, ENTRY_LIST, &[first.addr(), second.addr()]);
        e.set_global(START_INDEX, 1u32);
        e.call(0x007a_a480, &args![]);
        let deleted: Vec<u32> = calls(&e, OPERATOR_DELETE).iter().map(|w| w[0]).collect();
        assert_eq!(deleted, vec![0, first.addr(), 0, second.addr()]);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![ENTRY_LIST, 1]]);
        assert_eq!(e.global::<u32>(START_INDEX), 0);
    }

    #[test]
    fn entry_scalar_deleting_destructor_frees_only_on_flag() {
        let mut e = engine();
        let object = entry(&mut e);
        e.call(0x007a_a500, &args![object, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0]]);
        e.call(0x007a_a500, &args![object, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE)[2], vec![object.addr()]);
    }

    #[test]
    fn clear_message_queue_head_removes_the_current_entry() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let object = entry(&mut e);
        set_list(&lists, &mut e, ENTRY_LIST, &[object.addr()]);
        e.call(0x007a_a530, &args![]);
        assert_eq!(calls(&e, ARRAY_REMOVE), vec![vec![ENTRY_LIST, 0, 1]]);
        assert_eq!(calls(&e, OPERATOR_DELETE)[1], vec![object.addr()]);
        assert!(list_items(&lists, &e, ENTRY_LIST).is_empty());
    }

    #[test]
    fn fn_007aa5c0_removes_the_entries_with_the_current_warning_context() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let first = entry(&mut e);
        let other = entry(&mut e);
        let third = entry(&mut e);
        for (object, context) in [(first, 5), (other, 7), (third, 5)] {
            e.set(object, MessageMenuEntry::iWarningContext, context);
        }
        set_list(
            &lists,
            &mut e,
            ENTRY_LIST,
            &[first.addr(), other.addr(), third.addr()],
        );
        e.set_global(OBJECT_POINTER_01202D74, 0x7777u32);
        e.call(0x007a_a5c0, &args![]);
        assert_eq!(calls(&e, 0x00aa_4c40), vec![vec![0x7777, 5]]);
        assert_eq!(e.global::<u8>(STATE_FLAG_01202D54), 1);
        assert_eq!(list_items(&lists, &e, ENTRY_LIST), vec![other.addr()]);
    }

    // ----- 007aa6a0 / 007aa6b0 / 007aaf20 -----

    #[test]
    fn fn_007aa6a0_stores_the_flag_byte() {
        let mut e = engine();
        e.call(0x007a_a6a0, &args![1u8]);
        assert_eq!(e.global::<u8>(STATE_FLAG_01202D54), 1);
    }

    #[test]
    fn disable_message_menu_closes_or_creates() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let m = menu(&mut e);
        e.set_global(MESSAGE_MENU, m.addr());
        stub(&mut e, WORD_AT_FOUR, 0x6000);
        e.call(0x007a_a6b0, &args![1u8]);
        assert_eq!(e.global::<u8>(DISABLE_MESSAGE_MENU), 1);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![m.addr()]]);
        // Enabling with a message waiting queues the creation.
        e.set_global(MESSAGE_MENU, 0u32);
        let object = entry(&mut e);
        set_list(&lists, &mut e, ENTRY_LIST, &[object.addr()]);
        let two = e.mem.alloc(4);
        e.mem.set_i32(two, 2);
        stub(&mut e, SETTING_INT_POINTER, two);
        e.call(0x007a_a6b0, &args![0u8]);
        assert_eq!(calls(&e, QUEUE_MENU_CREATE), vec![vec![7, 0, 0, 0, 1, 0]]);
    }

    #[test]
    fn set_pause_menu_mode_skips_the_queued_messages_and_restores_the_flag() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let first = entry(&mut e).addr();
        let second = entry(&mut e).addr();
        let third = entry(&mut e).addr();
        set_list(&lists, &mut e, ENTRY_LIST, &[first, second, third]);
        e.set_global(DISABLE_MESSAGE_MENU, 1u8);
        e.call(0x007a_af20, &args![1u8]);
        assert_eq!(e.global::<u32>(START_INDEX), 3);
        assert_eq!(e.global::<u8>(SAVED_DISABLE_FLAG), 1);
        assert_eq!(e.global::<u8>(DISABLE_MESSAGE_MENU), 0);
        e.call(0x007a_af20, &args![0u8]);
        assert_eq!(e.global::<u32>(START_INDEX), 0);
        assert_eq!(e.global::<u8>(DISABLE_MESSAGE_MENU), 1);
    }

    // ----- 007aa720 -----

    struct Activation {
        e: Engine,
        lists: Lists,
        appends: Appends,
    }

    fn activation_world(target_type: i32, locked: bool) -> Activation {
        let mut e = engine();
        let appends = strings(&mut e);
        let lists = arrays(&mut e);
        set_list(&lists, &mut e, ENTRY_LIST, &[]);
        set_list(&lists, &mut e, PERK_ACTIVATIONS, &[]);
        let two = e.mem.alloc(4);
        e.mem.set_i32(two, 2);
        stub(&mut e, SETTING_INT_POINTER, two);
        let item = e.mem.alloc(0x10);
        e.put_vtable(0x0b20_0000, &[0x0ac2_0000, 0x0ac2_0001]);
        e.mem.set_u32(item, 0x0b20_0000);
        stub(&mut e, 0x0ac2_0000, 0);
        stub(&mut e, 0x0ac2_0001, 4);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, item);
        let pick = put_cstr(&mut e, "Pick");
        let take = put_cstr(&mut e, "Take");
        let locked_text = put_cstr(&mut e, "Locked");
        let activate = put_cstr(&mut e, "Activate");
        e.register_double(WORD_AT_FOUR, move |_, a| {
            ret(if a[0] == item { pick } else { 0 })
        });
        stub(&mut e, NODE_ITEM_SLOT, slot);
        stub(&mut e, SCRIPT_OF_ITEM, 0x5151);
        e.register_double(SETTING_VALUE, move |_, a| {
            ret(match a[0] {
                SETTING_011D29F4 => take,
                SETTING_011D45A0 => locked_text,
                SETTING_ACTIVATE_TITLE_011D2280 => activate,
                _ => 0,
            })
        });
        stub(&mut e, REFR_GET_TARGET_TYPE, target_type as u32);
        stub(&mut e, FORM_TYPE, 0x1c);
        stub(&mut e, REFR_GET_LOCK, locked as u32);
        stub(&mut e, LOCK_CHECK, 1);
        Activation { e, lists, appends }
    }

    #[test]
    fn activation_choices_list_the_items_and_the_references_own_choice() {
        let mut w = activation_world(0, false);
        // A one-item list: the node's next pointer is null.
        let node = w.e.mem.alloc(8);
        assert!(w.e.call(0x007a_a720, &args![node, 0x9000u32]).bool());
        let perks = list_items(&w.lists, &w.e, PERK_ACTIVATIONS);
        assert_eq!(perks.len(), 2);
        let first = Ptr::<PerkActivation>::new(perks[0]);
        assert_eq!(w.e.get(first, PerkActivation::pScript), 0x5151);
        assert_eq!(w.e.get(first, PerkActivation::pActivatedObject), 0x9000);
        let second = Ptr::<PerkActivation>::new(perks[1]);
        assert_eq!(w.e.get(second, PerkActivation::pScript), 0);
        let queue = list_items(&w.lists, &w.e, ENTRY_LIST);
        assert_eq!(queue.len(), 1);
        let object = Ptr::<MessageMenuEntry>::new(queue[0]);
        assert_eq!(
            w.e.get(object, MessageMenuEntry::fpResultCallback),
            0x007a_ae70
        );
        let log = w.appends.borrow().clone();
        let texts: Vec<&str> = log.iter().map(|(_, t)| t.as_str()).collect();
        assert_eq!(texts, vec!["Pick", ";", "Take", ";"]);
        assert_eq!(calls(&w.e, QUEUE_MENU_CREATE), vec![vec![7, 0, 0, 0, 1, 0]]);
    }

    #[test]
    fn a_locked_door_offers_the_locked_text() {
        let mut w = activation_world(8, true);
        stub(&mut w.e, REFR_BASE_FORM, 0);
        assert!(w.e.call(0x007a_a720, &args![0u32, 0x9000u32]).bool());
        let log = w.appends.borrow().clone();
        let texts: Vec<&str> = log.iter().map(|(_, t)| t.as_str()).collect();
        assert_eq!(texts, vec!["Locked", ";"]);
        assert_eq!(list_items(&w.lists, &w.e, PERK_ACTIVATIONS).len(), 1);
    }

    // ----- 007aae70 -----

    #[test]
    fn the_activation_callback_runs_a_script_or_activates_the_object() {
        let mut e = engine();
        let lists = arrays(&mut e);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first + 4, 0x5151);
        e.mem.set_u32(first + 8, 0x9001);
        e.mem.set_u32(second + 8, 0x9002);
        set_list(&lists, &mut e, PERK_ACTIVATIONS, &[first, second]);
        stub(&mut e, GET_MESSAGE_MENU_RESULT, 0);
        e.call(0x007a_ae70, &args![]);
        assert_eq!(calls(&e, SCRIPT_RUN), vec![vec![0x5151, 0x9001, 0, 0, 0]]);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![PERK_ACTIVATIONS, 1]]);
        set_list(&lists, &mut e, PERK_ACTIVATIONS, &[first, second]);
        e.set_global(PLAYER, 0x4242u32);
        stub(&mut e, GET_MESSAGE_MENU_RESULT, 1);
        stub(&mut e, REFR_COUNT, 9);
        e.call(0x007a_ae70, &args![]);
        assert_eq!(
            calls(&e, REFR_ACTIVATE),
            vec![vec![0x9002, 0x4242, 0, 0, 9]]
        );
    }

    // ----- string functions -----

    #[test]
    fn index_accessor_checks_the_bound() {
        let mut e = engine();
        strings(&mut e);
        let s = std_string(&mut e, "abc");
        let base = e.mem.u32(s + 4);
        assert_eq!(e.call(0x007a_afb0, &args![s, 1u32]).u32(), base + 1);
        assert_eq!(e.call(0x007a_afb0, &args![s, 3u32]).u32(), base + 3);
        assert!(calls(&e, STRING_INDEX_ASSERT).is_empty());
        e.call(0x007a_afb0, &args![s, 4u32]);
        assert_eq!(calls(&e, STRING_INDEX_ASSERT).len(), 1);
    }

    #[test]
    fn find_char_searches_from_the_position() {
        let mut e = engine();
        strings(&mut e);
        let s = std_string(&mut e, "a&b&");
        assert_eq!(e.call(0x007a_afe0, &args![s, 0x26u32, 0u32]).u32(), 1);
        assert_eq!(e.call(0x007a_afe0, &args![s, 0x26u32, 2u32]).u32(), 3);
        assert_eq!(
            e.call(0x007a_afe0, &args![s, 0x78u32, 0u32]).u32(),
            u32::MAX
        );
    }

    #[test]
    fn substr_builds_the_substring_in_the_output() {
        let mut e = engine();
        strings(&mut e);
        let s = std_string(&mut e, "hello world");
        let out = e.mem.alloc(0x20);
        assert_eq!(e.call(0x007a_b000, &args![s, out, 6u32, 5u32]).u32(), out);
        assert_eq!(std_text(&e, out), "world");
        // The construction hands the string itself to the initializer.
        assert_eq!(calls(&e, STD_STRING_INIT), vec![vec![out, out]]);
    }

    #[test]
    fn substring_constructor_copies_the_allocator_and_assigns() {
        let mut e = engine();
        strings(&mut e);
        let s = std_string(&mut e, "hello");
        let out = e.mem.alloc(0x20);
        e.call(0x007a_b050, &args![out, s, 1u32, 3u32, 0x1111u32]);
        assert_eq!(std_text(&e, out), "ell");
        assert_eq!(calls(&e, STD_STRING_RESET), vec![vec![out, 0, 0]]);
        assert_eq!(calls(&e, ALLOCATOR_COPY)[0][1], 0x1111);
    }

    #[test]
    fn allocator_copy_wrapper_returns_the_target() {
        let mut e = engine();
        assert_eq!(e.call(0x007a_b230, &args![0x10u32, 0x20u32]).u32(), 0x20);
        assert_eq!(calls(&e, ALLOCATOR_COPY), vec![vec![0x20, 0x10]]);
    }

    #[test]
    fn replace_with_a_c_string_erases_inserts_and_shrinks() {
        let mut e = engine();
        strings(&mut e);
        let s = std_string(&mut e, "&OK");
        let empty = put_cstr(&mut e, "");
        assert_eq!(e.call(0x007a_af80, &args![s, 0u32, 1u32, empty]).u32(), s);
        assert_eq!(std_text(&e, s), "OK");
        // Replacing with longer text moves the tail up.
        let s = std_string(&mut e, "hello world");
        let text = put_cstr(&mut e, "HI");
        e.call(0x007a_b0a0, &args![s, 0u32, 5u32, text, 2u32]);
        assert_eq!(std_text(&e, s), "HI world");
        let text = put_cstr(&mut e, "XYZ");
        e.call(0x007a_b0a0, &args![s, 2u32, 0u32, text, 3u32]);
        assert_eq!(std_text(&e, s), "HIXYZ world");
        // Text taken from the string itself goes through the string replace.
        let base = e.mem.u32(s + 4);
        e.call(0x007a_b0a0, &args![s, 0u32, 2u32, base + 6, 5u32]);
        assert_eq!(std_text(&e, s), "worldXYZ world");
    }

    #[test]
    fn replace_with_a_string_handles_overlapping_sources() {
        let mut e = engine();
        strings(&mut e);
        let s = std_string(&mut e, "hello world");
        let other = std_string(&mut e, "bye");
        e.call(0x007a_b250, &args![s, 0u32, 5u32, other, 0u32, 3u32]);
        assert_eq!(std_text(&e, s), "bye world");
        // The source after the replaced part.
        let s = std_string(&mut e, "hello world");
        e.call(0x007a_b250, &args![s, 0u32, 1u32, s, 6u32, 5u32]);
        assert_eq!(std_text(&e, s), "worldello world");
        // The source before the replaced part.
        let s = std_string(&mut e, "abcdef");
        e.call(0x007a_b250, &args![s, 4u32, 2u32, s, 0u32, 3u32]);
        assert_eq!(std_text(&e, s), "abcdabc");
        // The source overlapping the replaced part.
        let s = std_string(&mut e, "abcdef");
        e.call(0x007a_b250, &args![s, 1u32, 2u32, s, 2u32, 4u32]);
        assert_eq!(std_text(&e, s), "acdefdef");
        // A source no longer than the replaced part.
        let s = std_string(&mut e, "abcdef");
        e.call(0x007a_b250, &args![s, 1u32, 3u32, s, 4u32, 2u32]);
        assert_eq!(std_text(&e, s), "aefef");
    }

    // ----- array instances -----

    #[test]
    fn array_instances_set_their_vtables_and_clear() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        assert_eq!(e.call(0x007a_b590, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), ENTRY_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_CONSTRUCT), vec![vec![object, 0, 0]]);
        e.call(0x007a_b610, &args![object]);
        assert_eq!(e.mem.u32(object), PERK_ARRAY_VTABLE);
        e.call(0x007a_b5f0, &args![object]);
        assert_eq!(e.mem.u32(object), ENTRY_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![object, 1]]);
        e.call(0x007a_b5c0, &args![object, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![object]]);
        e.call(0x007a_b640, &args![object, 0u32]);
        assert_eq!(calls(&e, 0x007a_b670), vec![vec![object]]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on the running thread takes (the called function's
    /// own frame, taken at the stack's current top), with the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.next_stack_local(size)
    }

    #[test]
    fn add_message_keeps_its_locals_at_the_exe_offsets() {
        let (mut e, _, _) = message_world();
        let buttons = Message {
            first_button: "&OK",
            buttons: vec!["Cancel"],
            modal: false,
        };
        // 007a8e60 (0x98 bytes): the buttons at +0x80, the button text at +0x5c, the hotkey at
        // +0x2c, the queued entry at +0x88.
        let frame = next_frame(&mut e, 0x98);
        assert!(add_message(&mut e, &buttons));
        assert_eq!(calls(&e, STRING_FROM_C_STRING)[0][0], frame + 0x80);
        let texts: Vec<u32> = calls(&e, STD_STRING_FROM_C_STRING)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(texts, vec![frame + 0x5c; 2]);
        assert_eq!(calls(&e, STD_STRING_INIT)[0][0], frame + 0x2c);
        assert_eq!(calls(&e, ARRAY_ADD), vec![vec![ENTRY_LIST, frame + 0x88]]);
    }

    #[test]
    fn load_next_message_keeps_its_locals_at_the_exe_offsets() {
        // 007a92e0 (0x1e8 bytes): the default sound handle at +0x110, the text size at +0x1b0.
        let (mut e, m, _) = load_world(false);
        let frame = next_frame(&mut e, 0x1e8);
        e.call(0x007a_92e0, &args![m]);
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_NAME)[0][1], frame + 0x110);
        let sizes = calls(&e, CALCULATE_STRING_DIMENSIONS);
        assert_eq!(sizes[sizes.len() - 1][1], frame + 0x1b0);
    }

    #[test]
    fn activation_choices_keep_their_buffer_in_the_frame() {
        // 007aa720 (0x1ec bytes): the text buffer at +0xc8, which the last choice keeps.
        let mut w = activation_world(0, false);
        let node = w.e.mem.alloc(8);
        let frame = next_frame(&mut w.e, 0x1ec);
        assert!(w.e.call(0x007a_a720, &args![node, 0x9000u32]).bool());
        assert!(calls(&w.e, COPY_C_STRING)
            .iter()
            .all(|c| c[0] == frame + 0xc8));
        let perks = list_items(&w.lists, &w.e, PERK_ACTIVATIONS);
        assert_eq!(w.e.mem.u32(perks[1]), frame + 0xc8);
        let adds: Vec<u32> = calls(&w.e, ARRAY_ADD).iter().map(|c| c[1]).collect();
        assert_eq!(adds, vec![frame + 0xb4, frame + 0xc0, frame + 0x1dc]);
    }

    #[test]
    fn the_string_helpers_keep_their_words_at_the_exe_offsets() {
        let mut e = engine();
        strings(&mut e);
        let s = std_string(&mut e, "a&b&");
        // 007aafe0: its own argument slot, +0xc of a 0x10-byte frame.
        let frame = next_frame(&mut e, 0x10);
        e.call(0x007a_afe0, &args![s, 0x26u32, 0u32]);
        assert_eq!(calls(&e, STD_STRING_FIND_CHARS)[0][1], frame + 0xc);
        // 007ab050: the pushed word under its 8-byte frame, +0 of 0xc.
        let out = e.mem.alloc(0x20);
        let frame = next_frame(&mut e, 0xc);
        e.call(0x007a_b050, &args![out, s, 1u32, 1u32, 0x1111u32]);
        assert_eq!(calls(&e, ALLOCATOR_COPY)[0], vec![frame, 0x1111]);
        // 007ab000: the allocator byte at +0xb of its 0xc-byte frame, handed to `007ab230`.
        let frame = next_frame(&mut e, 0xc);
        e.call(0x007a_b000, &args![s, out, 1u32, 1u32]);
        assert_eq!(calls(&e, ALLOCATOR_COPY)[0][0], frame + 0xb);
        // 007aa110: the hotkeys at +0x10 of its 0x40-byte frame.
        vtables(&mut e);
        let lists = arrays(&mut e);
        let m = menu(&mut e);
        let object = entry(&mut e);
        set_text(&mut e, object.addr(), 0x20, "ab");
        set_list(&lists, &mut e, ENTRY_LIST, &[object.addr()]);
        stub(&mut e, LIST_BOX_COUNT, 3);
        let frame = next_frame(&mut e, 0x40);
        e.call(0x007a_a110, &args![m, 0x5au32]);
        assert_eq!(calls(&e, STD_STRING_FROM_C_STRING)[0][0], frame + 0x10);
    }
}
