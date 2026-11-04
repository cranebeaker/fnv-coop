//! `fallout/interface/menus/startmenu.cpp` (Xbox PDB source unit), part 2: its functions from `007d3aa0` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::startmenu`]; anything public there may be used here.
//!
//! This part holds the first 60 functions of its range (`007d3aa0` ..
//! `007d74c0`): the save/load screen (list preparation, the confirm dialog,
//! the info panel, the idle handling), the options list and its key
//! remapping, `StartMenu::Fade`, `StartMenu::UserOption`, and the
//! constructor, destructor, `AddItem`, `RemoveAll` and `Sort` of
//! `ListBox<StartMenu::Option*>`. The second batch (`007d74f0` .. `007da040`,
//! 49 functions) holds the rest of the unit: the state, selection, highlight
//! and removal functions of `ListBox<StartMenu::Option*>` and its twin
//! `ListBox<BGSSaveLoadFileEntry*>`, `ListBox<int>::SetEnabled`, and a few
//! small array helpers.
//!
//! Callees are called by address. Most tile functions (`BSMenu/tile.cpp`)
//! have no name in the engine map, so the constants below describe each by
//! the arguments the code passes and, where the decompiler showed it, by the
//! body; the trait ids passed to them are written `TRAIT_<id>` and their
//! names are unconfirmed. The unit's own helper layouts (`StartMenu`,
//! `StartMenuOption`, `StartMenuUserOption`, `ListBoxLayout`) are declared
//! here because the part-1 file does not provide them.
//!
//! Exception-unwinding frames (`FS:[0]`) and `/GS` cookie checks (`00ec408c`)
//! are not translated; every function that has one carries a
//! `Translation incomplete` marker.

// The x87 comparisons below must fail on NaN exactly as the exe's flag tests
// do, so negated comparisons are kept on purpose.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[allow(unused_imports)]
use super::startmenu::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

// ---- globals ---------------------------------------------------------------

/// `StartMenu::pMe` (Xbox PDB static): the game's start menu.
const START_MENU_INSTANCE: u32 = 0x011d_aac0;
/// The `BSSimpleArray<StartMenu::UserOption*>` of user options
/// (`StartMenu::pUserOptionsA` in the Xbox PDB; the elements have a
/// `UserOption`'s layout: type at +0x1C, change function at +0x2C).
const USER_OPTIONS_ARRAY: u32 = 0x011d_ab50;
/// The `BSSimpleArray<StartMenu::Option*>` of options
/// (`StartMenu::pOptionsA` in the Xbox PDB; `Option`'s layout).
const OPTIONS_ARRAY: u32 = 0x011d_ab88;
/// The save/load manager singleton.
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// The loading menu singleton (null while it does not exist).
const LOADING_MENU: u32 = 0x011d_a0c0;
/// The object whose `00877720` returns the controls object.
const CONTROLS_OWNER: u32 = 0x011d_ea0c;
/// A pointer to an object with a flags byte at +0x680 (`005a03f0` tests it).
const OBJECT_POINTER_011DEA3C: u32 = 0x011d_ea3c;
/// A `TESGlobal*` (read with the global value getter `00526ac0`).
const TES_GLOBAL_POINTER: u32 = 0x011c_3f3c;
/// The object `00463070`, `00465040`, `004614e0`, `0046fdf0` and `0046fe10`
/// act on (`fallout shared/tesdatahandler.cpp` in the engine map).
const DATA_HANDLER_OBJECT: u32 = 0x011c_3f2c;
/// The object `00970d50` is called with (`fallout/ai/processlists.cpp` in the
/// engine map).
const PROCESS_LISTS_OBJECT: u32 = 0x011e_0e80;
/// The object `006003c0` is called with (`fallout shared/tesidlemanager.cpp`
/// in the engine map).
const IDLE_MANAGER_OBJECT: u32 = 0x011c_b6a0;
/// A form object with a flags word at +0x244, passed to `007d6bd0`.
const FLAGGED_FORM_OBJECT: u32 = 0x011d_df38;
/// Tick count at which the save wait ends (`007d5af0`).
const SAVE_WAIT_DEADLINE: u32 = 0x011d_ac68;
/// Tick count before which more key-remap input is ignored (`007d52a0`).
const REMAP_INPUT_DEADLINE: u32 = 0x011d_ac64;
/// A byte set while the main list is shown (`007d6550`, `007d6c90`).
const FLAG_BYTE_MAIN_LIST: u32 = 0x011d_aacd;
/// A byte set together with `FLAG_DLC_WAIT_MESSAGE` in `007d6550`.
const FLAG_BYTE_PENDING: u32 = 0x011d_aace;
/// A byte `007d5830` clears after it rewrites the seed cell.
const SEED_DIRTY_BYTE: u32 = 0x011a_0294;
/// When this byte is nonzero `RemoveAll` does not destroy the item tiles.
const SKIP_TILE_DESTROY_BYTE: u32 = 0x011d_8909;
/// The `GetTickCount` import slot (called through `[00fdf060]`).
const GET_TICK_COUNT_IMPORT: u32 = 0x00fd_f060;
/// The object `00454af0` reads a byte of.
const FLAG_SETTING_OBJECT: u32 = 0x011d_abb0;
/// The object whose `0043d4d0` returns a pointer to the cell `007d5830` reads
/// and rewrites.
const SEED_CELL_OBJECT: u32 = 0x011d_0c60;

// ---- constants read from the exe -----------------------------------------------

/// `float` every `Fade` call in this unit passes as its rate (`01012054`).
const FADE_TARGET_VALUE: u32 = 0x0101_2054;
/// `double` compared with in the fade and tile code (`01012060`).
const COMPARE_ZERO: u32 = 0x0101_2060;
/// `double` compared with in `007d5830` and `007d61e0` (`0101e568`).
const COMPARE_ONE: u32 = 0x0101_e568;
/// `float` `Fade` substitutes when the rate passed is below zero.
const FADE_DEFAULT_VALUE: u32 = 0x0101_622c;
/// `float` `Fade` passes to `AddFadeControl` as the far end (`01023cd8`).
const FADE_CONTROL_RATE: u32 = 0x0102_3cd8;
/// `float` the confirm-dialog fade uses as its rate (`0105559c`).
const FADE_CONFIRM_VALUE: u32 = 0x0105_559c;

// ---- vtables and function pointers ----------------------------------------------

/// Vtable stored by `StartMenu::UserOption`'s constructor (RTTI:
/// `StartMenu::UserOption`).
const USER_OPTION_VTABLE: u32 = 0x0107_704c;
/// Vtable stored by `ListBox<StartMenu::Option*>`'s constructor (RTTI:
/// `ListBox<StartMenu::Option*>`).
const OPTION_LIST_BOX_VTABLE: u32 = 0x0107_706c;
/// Template name of the toggle options (`lb_toggle_template`).
const USER_OPTION_TOGGLE_TEMPLATE: u32 = 0x0107_7034;
/// Template name of the meter options and the plain options
/// (`lb_meter_template`).
const USER_OPTION_METER_TEMPLATE: u32 = 0x0107_6ff8;
/// Change function of the brightness option (`pBrightnessOption`).
const BRIGHTNESS_CHANGE_FN: u32 = 0x007d_19c0;
/// The change functions `007d42b0` guards with the controller query.
const CONTROLLER_OPTION_CHANGE_FN_A: u32 = 0x007d_2fc0;
const CONTROLLER_OPTION_CHANGE_FN_B: u32 = 0x007d_30e0;
/// The change function `007d42b0` greys out while the XUI is up.
const GAMEPAD_OPTION_CHANGE_FN: u32 = 0x007d_13a0;
/// The save/load list's filter (`007d4040`).
const SAVE_LOAD_FILTER_FN: u32 = 0x007d_4040;
/// The per-item initializer (`007d67b0`) the save/load list runs over a range.
const SAVE_LOAD_ITEM_INIT_FN: u32 = 0x007d_67b0;
/// Item refresh callback run over the options list (`007cfe40`).
const OPTION_LIST_REFRESH_FN: u32 = 0x007c_fe40;
/// Item callback run over the options list when action mapping ends
/// (`007ce970`).
const ACTION_MAPPING_ITEM_FN: u32 = 0x007c_e970;
/// The message callback `007d5f50` (`FailedSaveCallbackFunction`).
const FAILED_SAVE_CALLBACK: u32 = 0x007d_5f50;

// ---- strings -----------------------------------------------------------------

/// `"lb_newsave_template"`.
const NEW_SAVE_TEMPLATE: u32 = 0x0107_6fbc;
/// `"%s %u/%i %s"`.
const FORMAT_SAVE_COUNT: u32 = 0x0107_6fb0;
/// `"_alpha"`.
const TRAIT_NAME_ALPHA: u32 = 0x0107_702c;
/// `"_current_value"`.
const TRAIT_NAME_CURRENT_VALUE: u32 = 0x0107_02d8;
/// `"_SetInCode "`.
const TRAIT_NAME_SET_IN_CODE: u32 = 0x0107_6fd0;
/// `"_text_offset"`.
const TRAIT_NAME_TEXT_OFFSET: u32 = 0x0107_6fdc;
/// `"_center_x"`.
const TRAIT_NAME_CENTER_X: u32 = 0x0107_6fec;
/// `"_target"`.
const TRAIT_NAME_TARGET: u32 = 0x0107_700c;
/// `"_initialized"`.
const TRAIT_NAME_INITIALIZED: u32 = 0x0107_7058;
/// `"#%04d"`.
const FORMAT_SAVE_NUMBER: u32 = 0x0107_7050;
/// `"%s %d%s: %s"`.
const FORMAT_LEVEL: u32 = 0x0107_7020;
/// `"%s%s: %s"`.
const FORMAT_TWO_STRINGS: u32 = 0x0107_7014;
/// `"FRENCH"`.
const LANGUAGE_FRENCH: u32 = 0x0101_fd00;
/// `"Data\\Music\\Special\\MainTitle.mp3"`.
const FORMAT_CREDITS_MEDIA: u32 = 0x0107_6f3c;
/// `"_number_of_visible_items"`.
const LIST_HEIGHT_TRAIT_NAME: u32 = 0x0106_f860;
/// `"_num_filtered"`.
const LIST_RESET_TRAIT_NAME: u32 = 0x0106_f8d0;
/// An empty string, passed as the last word of the DLC message (`01011584`).
const MESSAGE_FOOTER: u32 = 0x0101_1584;

// ---- trait ids (names unconfirmed) ----------------------------------------------

/// "use the `_alpha` trait" marker `Fade` takes as its trait.
const TRAIT_DEFAULT: u32 = 0x8000_0000;
const TRAIT_0FA1: u32 = 0x0fa1;
const TRAIT_0FA2: u32 = 0x0fa2;
const TRAIT_0FA3: u32 = 0x0fa3;
const TRAIT_0FA9: u32 = 0x0fa9;
const TRAIT_0FAA: u32 = 0x0faa;
const TRAIT_0FAC: u32 = 0x0fac;
const TRAIT_0FAF: u32 = 0x0faf;
const TRAIT_0FB0: u32 = 0x0fb0;
const TRAIT_0FB1: u32 = 0x0fb1;
const TRAIT_0FC3: u32 = 0x0fc3;
const TRAIT_0FC4: u32 = 0x0fc4;
const TRAIT_1004: u32 = 0x1004;
const TRAIT_1005: u32 = 0x1005;
const TRAIT_1006: u32 = 0x1006;
const TRAIT_1007: u32 = 0x1007;
const TRAIT_1008: u32 = 0x1008;
const TRAIT_1009: u32 = 0x1009;

// ---- game settings (strings read through 00403df0) -------------------------------
// The setting objects are filled when the game starts, so their text is not
// in the exe; each is named by its address and the doc says where it is
// used.

/// `007d3c70`/`007d3b60`: the text of the first item when the list is empty,
/// of `007d3b60`'s new-save item.
const SETTING_011D39C0: u32 = 0x011d_39c0;
/// Defaults-button text of the save/load screen.
const SETTING_011D29E8: u32 = 0x011d_29e8;
/// Text for the list's single tile when `007d3c70` is not in save mode.
const SETTING_011D39F0: u32 = 0x011d_39f0;
/// Parts of the `"%s %u/%i %s"` save-count line (`007d3c70`).
const SETTING_011D25D4: u32 = 0x011d_25d4;
const SETTING_011D2C4C: u32 = 0x011d_2c4c;
/// Confirm-dialog question in delete mode (`007d40a0`).
const SETTING_011D4FB4: u32 = 0x011d_4fb4;
/// Confirm-dialog question in save mode (`007d40a0`).
const SETTING_011D3294: u32 = 0x011d_3294;
/// Confirm-dialog question otherwise (`007d40a0`).
const SETTING_011D3F28: u32 = 0x011d_3f28;
/// Defaults-button text of the options screen (`007d42b0`).
const SETTING_011D42F4: u32 = 0x011d_42f4;
/// The downloads list's `none` item text.
const SETTING_011D3AD4: u32 = 0x011d_3ad4;
/// Text of a corrupt save in the info panel.
const SETTING_011D1F14: u32 = 0x011d_1f14;
/// The setting `007d4df0` compares with `"FRENCH"`.
const SETTING_011C3CB4: u32 = 0x011c_3cb4;
/// Parts of the info panel lines (`007d4df0`).
const SETTING_011D3B2C: u32 = 0x011d_3b2c;
const SETTING_011D3954: u32 = 0x011d_3954;
const SETTING_011D21B4: u32 = 0x011d_21b4;
/// Autosave, and the two other special save kinds, list texts (`007d67b0`).
const SETTING_011D3F10: u32 = 0x011d_3f10;
const SETTING_011D4E1C: u32 = 0x011d_4e1c;
const SETTING_011D4E04: u32 = 0x011d_4e04;
/// Location text of a corrupt save (`007d67b0`).
const SETTING_011D46E4: u32 = 0x011d_46e4;
/// The text the message boxes pass (as their ninth word) next to the icon.
const MESSAGE_ICON_SETTING: u32 = 0x011d_38b8;
/// Remap failure message (`007d52a0`).
const SETTING_011D4750: u32 = 0x011d_4750;
/// Stick-moved message (`007d52a0`).
const SETTING_011D3108: u32 = 0x011d_3108;
/// DLC message while the DLC menu is missing (`007d5830`).
const SETTING_011D3E2C: u32 = 0x011d_3e2c;
/// DLC-corrupt message (`007d5830`).
const SETTING_011D2F7C: u32 = 0x011d_2f7c;
/// Save-failed message (`007d5af0`).
const SETTING_011D4420: u32 = 0x011d_4420;
/// Messages of `007d6d70`.
const SETTING_011D493C: u32 = 0x011d_493c;
const SETTING_011D3138: u32 = 0x011d_3138;
/// Table of four game-setting pointers indexed by `field_1c8`.
const REMAP_PROMPT_TABLE: u32 = 0x011d_5730;

// ---- the unit's own functions called by address ----------------------------------

/// `StartMenu::ChooseBack` (name from the engine map).
const START_MENU_CHOOSE_BACK: u32 = 0x007d_0e40;
/// `StartMenu::Close(flag)` (cdecl, name from the engine map).
const START_MENU_CLOSE: u32 = 0x007c_e7a0;
/// `StartMenu::WaitingForControlRemapButtonPress` (name from the engine map).
const WAITING_FOR_REMAP_PRESS: u32 = 0x007c_e8f0;
/// `StartMenu::UpdateCanLoadStatus` (name from the engine map).
const UPDATE_CAN_LOAD_STATUS: u32 = 0x007c_fd20;
/// Unnamed startmenu.cpp function called by `007d52a0` (no arguments but
/// `this`).
const FN_007D01B0: u32 = 0x007d_01b0;
/// Unnamed startmenu.cpp function (cdecl, one word) called by `007d6c30`.
const START_MENU_FN_7D30B0: u32 = 0x007d_30b0;
/// `StartMenu::IdleTimer` singleton accessor (`007cb710`).
const IDLE_TIMER_GET: u32 = 0x007c_b710;
/// Resets a `StartMenu::IdleTimer` (`007cf5c0`).
const IDLE_TIMER_RESET: u32 = 0x007c_f5c0;
/// `StartMenu::Option::Option(this, text, choose function, modes)`
/// (`007ce620`).
const OPTION_CONSTRUCT: u32 = 0x007c_e620;
/// `StartMenu::Option`'s destructor body (`007ce690`).
const OPTION_DESTRUCT: u32 = 0x007c_e690;
/// `(option, modes) -> bool`: `option.iModes & modes != 0` (`007cc5d0`).
const OPTION_MATCHES_MODES: u32 = 0x007c_c5d0;
/// `(option, flags)`: `Option`'s scalar deleting destructor, as called on the
/// list's items (`004459e0`).
const OPTION_SCALAR_DELETING_DESTRUCTOR: u32 = 0x0044_59e0;

// ListBox<StartMenu::Option*> and ListBox<BGSSaveLoadFileEntry*> functions.
const OPTION_LIST_SET_ENABLED: u32 = 0x007d_74f0;
const OPTION_LIST_IS_ENABLED: u32 = 0x007d_7580;
const OPTION_LIST_DESELECT_ALL: u32 = 0x007d_7b50;
const OPTION_LIST_GET_HIGHLIGHT_TARGET: u32 = 0x007d_76e0;
const OPTION_LIST_SYNCHRONIZE: u32 = 0x007d_8f10;
const OPTION_LIST_SET_TOTAL_LIST_HEIGHT: u32 = 0x007d_9460;
const OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS: u32 = 0x007d_9140;
/// `ListBox<StartMenu::Option*>::GetTileFor(&data)` (name from the engine
/// map).
const OPTION_LIST_GET_TILE_FOR: u32 = 0x007a_22d0;
const SAVE_LOAD_LIST_ADD_ITEM: u32 = 0x007d_7da0;
const SAVE_LOAD_LIST_SET_ENABLED: u32 = 0x007d_8810;
const SAVE_LOAD_LIST_IS_ENABLED: u32 = 0x007d_88a0;
const SAVE_LOAD_LIST_DESELECT_ALL: u32 = 0x007d_8da0;
const SAVE_LOAD_LIST_SET_SELECTION: u32 = 0x007d_8d60;
const SAVE_LOAD_LIST_SELECT_HIGHLIGHT: u32 = 0x007d_8d80;
const SAVE_LOAD_LIST_FILTER: u32 = 0x007d_8450;
/// `(list, index, 0) -> data` (`007d86c0`).
const LIST_DATA_AT: u32 = 0x007d_86c0;
/// `(list, index, 0)` (`007d80f0`).
const LIST_REMOVE_AT: u32 = 0x007d_80f0;
/// `(list) -> item count` (`007d87b0`).
const LIST_ITEM_COUNT: u32 = 0x007d_87b0;
/// `(list)`: `GetTileAt(0, 0)` through the vtable (`007d6e90`).
const LIST_FIRST_TILE: u32 = 0x007d_6e90;
/// `(list) -> bool`: the embedded list is empty (`007d6e70`).
const LIST_IS_EMPTY: u32 = 0x007d_6e70;
/// `(list) -> count` of the embedded list (`00726c40`).
const LIST_COUNT: u32 = 0x0072_6c40;
/// `(list) -> bool`: the list has a container tile that passes `00a040a0`
/// (`007d7630`).
const LIST_CONTAINER_VALID: u32 = 0x007d_7630;
/// `(list) -> pScrollBar` (`00825c00`).
const LIST_SCROLL_BAR: u32 = 0x0082_5c00;
/// `(list) -> data of the highlighted item` (`007a1910`).
const LIST_SELECTED_DATA: u32 = 0x007a_1910;
/// `(list, function, start, end)`: runs `function` over the items from
/// `start` to `end` (`007314c0`).
const LIST_FOR_RANGE: u32 = 0x0073_14c0;
/// `ListBox<int>::AddItem(list, data, text, 0, 0)` (name from the engine
/// map).
const DOWNLOADS_LIST_ADD_ITEM: u32 = 0x0076_4430;
/// `ListBox<int>::SetEnabled` (name from the engine map).
const DOWNLOADS_LIST_SET_ENABLED: u32 = 0x007d_7bd0;
/// `(list, bit) -> bool`: `usFlags >> bit & 1` (`007242c0`).
const LIST_FLAG_TEST: u32 = 0x0072_42c0;
/// `(list, tile)` (`007269d0`, `InitTilePosition` in the engine map).
const LIST_INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// `(embedded list, item)`: appends (`00905820`).
const LIST_ADD_TAIL: u32 = 0x0090_5820;
/// `(embedded list, item, position)`: inserts (`007a7eb0`).
const LIST_INSERT: u32 = 0x007a_7eb0;
/// `(embedded list, argument)`: sorts (`0083fd60`).
const LIST_SORT: u32 = 0x0083_fd60;
/// `ListBoxItem(this, tile, &data)` constructor (`007a62e0`).
const LIST_ITEM_CONSTRUCT: u32 = 0x007a_62e0;
/// Returns the node's own address: the address of its item (`006815c0`).
const LIST_NODE_SELF: u32 = 0x0068_15c0;
/// `(node, next)`: stores the next pointer at +4 (`006ecd40`).
const LIST_NODE_SET_NEXT: u32 = 0x006e_cd40;
/// Scalar deleting destructor of a list node (`004702f0`).
const LIST_NODE_SCALAR_DELETING_DESTRUCTOR: u32 = 0x0047_02f0;
/// `(list, node) -> node ? [node] : 0` (`007b52d0`).
const LIST_NEXT_NODE: u32 = 0x007b_52d0;
/// `(list, node) -> &item` (`006317a0`, node + 8).
const LIST_NODE_ITEM_SLOT: u32 = 0x0063_17a0;
/// `(list) -> [list] + 8` (`0073ac90`).
const LIST_HEAD_ITEM_SLOT: u32 = 0x0073_ac90;
/// `(object) -> [object + 4]` (`00726070`).
const WORD_AT_4: u32 = 0x0072_6070;
/// `(object) -> object + 0xC` (`0048d150`).
const LIST_FIRST_NODE: u32 = 0x0048_d150;
/// `BSSimpleList::IsEmpty` on the list at the object (`008256d0`).
const LIST_IS_EMPTY_NODE: u32 = 0x0082_56d0;
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const EMBEDDED_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const EMBEDDED_LIST_DESTRUCT: u32 = 0x0046_ffb0;
const LIST_BOX_FN_7A1950: u32 = 0x007a_1950;
const LIST_BOX_FN_721130: u32 = 0x0072_1130;
const LIST_BOX_FN_7211C0: u32 = 0x0072_11c0;

// Tiles.
/// `Tile::GetChildByID(tile, id)` (name from the engine map).
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
/// `Tile::Value::SetFloat(value, float, notify)` (name from the engine map).
const TILE_VALUE_SET_FLOAT: u32 = 0x00a0_a270;
/// `TileImage` accessor: the word at +0x64 of the object (`009c8660`).
const IMAGE_ALPHA_MAP: u32 = 0x009c_8660;
/// `(image, texture)` (`007700b0`).
const IMAGE_SET_ALPHA_MAP: u32 = 0x0077_00b0;

// Save games.
/// `BGSSaveLoadFileEntry::LoadData` (name from the engine map).
const FILE_ENTRY_LOAD_DATA: u32 = 0x0084_6900;
/// `BGSSaveLoadFileEntry::IsAutosave` (name from the engine map).
const FILE_ENTRY_IS_AUTOSAVE: u32 = 0x0084_6af0;
/// `(entry) -> bool`: asks the save/load manager about the entry's file name
/// (`00851a60`); unnamed.
const FILE_ENTRY_NAME_TEST_AD0: u32 = 0x0084_6ad0;
/// `(entry) -> bool`: asks the save/load manager about the entry's file name
/// (`00851b00`); unnamed.
const FILE_ENTRY_NAME_TEST_B10: u32 = 0x0084_6b10;
/// `BGSSaveLoadFileEntry::IsCorrupt` (name from the engine map).
const FILE_ENTRY_IS_CORRUPT: u32 = 0x0084_6b30;
/// `BGSSaveLoadFileEntry::GetScreenshot` (name from the engine map).
const FILE_ENTRY_GET_SCREENSHOT: u32 = 0x0084_6820;
/// `BGSSaveLoadFileEntry::SaveGame` (name from the engine map).
const FILE_ENTRY_SAVE_GAME: u32 = 0x0084_6710;
/// Loads the entry's data, returns `pPlayerName` (+0x0C, `0075f8c0`).
const FILE_ENTRY_PLAYER_NAME: u32 = 0x0075_f8c0;
/// Returns the address `entry + 0x3C` (`pcLanguage`, `005a8080`).
const FILE_ENTRY_LANGUAGE: u32 = 0x005a_8080;
/// `BGSSaveLoadManager::SaveGame` (name from the engine map).
const SAVE_LOAD_MANAGER_SAVE_GAME: u32 = 0x0085_03b0;
/// Refreshes the save/load manager's file list (`00851330`).
const SAVE_LOAD_MANAGER_REFRESH: u32 = 0x0085_1330;

// Controls (roles read from the decompiled bodies; unconfirmed).
/// Keyboard key state query `(controls, key, mode)` (`00a24180`).
const KEY_STATE: u32 = 0x00a2_4180;
/// Binds an input to an action `(controls, action, device, input) -> bool`
/// (`00a24fc0`).
const BIND_INPUT: u32 = 0x00a2_4fc0;
/// Clears an action's binding `(controls, action, device, 0xFF)`
/// (`00a25180`).
const CLEAR_BINDING: u32 = 0x00a2_5180;
/// "Cancel" query `(controls, 0x1C, 1)` (`00a24660`).
const BINDING_CANCEL: u32 = 0x00a2_4660;
/// Mouse button count (`00a23a30`).
const MOUSE_BUTTON_COUNT: u32 = 0x00a2_3a30;
/// Mouse button state `(controls, button, mode)` (`00a23a50`).
const MOUSE_BUTTON_STATE: u32 = 0x00a2_3a50;
/// Gamepad button count `(controls, pad)` (`00a236b0`).
const GAMEPAD_BUTTON_COUNT: u32 = 0x00a2_36b0;
/// Gamepad button state `(controls, pad, button, mode)` (`00a234c0`).
const GAMEPAD_BUTTON_STATE: u32 = 0x00a2_34c0;
/// Gamepad axis count `(controls, pad)` (`00a236d0`).
const GAMEPAD_AXIS_COUNT: u32 = 0x00a2_36d0;
/// Gamepad axis source `(controls, pad, axis)` (`00a235f0`).
const GAMEPAD_AXIS_SOURCE: u32 = 0x00a2_35f0;
/// Gamepad axis value `(controls, pad, axis)` (`00a23390`).
const GAMEPAD_AXIS_VALUE: u32 = 0x00a2_3390;
/// `(controls)` (`00a22e10`).
const CONTROLS_FN_A22E10: u32 = 0x00a2_2e10;
/// `abs`-like wrapper (`0072fa60`, cdecl): calls `00ec7d40`.
const ABSOLUTE_VALUE: u32 = 0x0072_fa60;
/// Windows import thunk `009f996e` (jumps through the import slot
/// `00fdf394`); called with `(0, &16-byte buffer)` and 0 means success. The
/// shape matches `XInputGetState`; unconfirmed.
const CONTROLLER_STATE_IMPORT: u32 = 0x009f_996e;
/// `(object, mask) -> bool`: `byte [object + 0x680] & mask` (`005a03f0`).
const OBJECT_FLAG_BYTE_TEST: u32 = 0x005a_03f0;

// Utilities and singletons.
/// `BSSystemUtility::QInstance` (name from the engine map).
const SYSTEM_UTILITY_INSTANCE: u32 = 0x00af_2640;
/// Called on the system utility, returns the DLC utility (`005dfa10` ->
/// `00af26c0`).
const DLC_UTILITY_FROM_SYSTEM: u32 = 0x005d_fa10;
const DLC_UTILITY_FN_AF2940: u32 = 0x00af_2940;
const DLC_UTILITY_FN_AF2B10: u32 = 0x00af_2b10;
/// Returns the result of `BSNetworkSystemUtility::QInstance` (`00af2c90`).
const NETWORK_SYSTEM_UTILITY_INSTANCE: u32 = 0x00af_2c90;
/// `(object) -> byte [object]` (`009373f0`).
const FIRST_BYTE: u32 = 0x0093_73f0;
/// `(tile menu) -> state` (`0059bb30`).
const DLC_ENUMERATION_STATE: u32 = 0x0059_bb30;
const DLC_FINISH: u32 = 0x00af_59f0;
const DATA_HANDLER_IS_LOADED: u32 = 0x0046_fdf0;
const DATA_HANDLER_COUNT: u32 = 0x0046_fe10;
const DATA_HANDLER_FN_463070: u32 = 0x0046_3070;
const DATA_HANDLER_FN_465040: u32 = 0x0046_5040;
const DATA_HANDLER_FN_4614E0: u32 = 0x0046_14e0;
const PROCESS_LISTS_FN_970D50: u32 = 0x0097_0d50;
const IDLE_MANAGER_FN_6003C0: u32 = 0x0060_03c0;
const FORM_FN_570F40: u32 = 0x0057_0f40;
/// Returns a pointer to the int cell of the object it is called on
/// (`0043d4d0`).
const SEED_CELL: u32 = 0x0043_d4d0;
/// `max(a, b)` of two ints (`00647b70`, cdecl).
const MAX_INT: u32 = 0x0064_7b70;
/// `min(a, b)` of two ints (`004a8f20`, cdecl).
const MIN_INT: u32 = 0x004a_8f20;
/// `max(a, b)` of two floats, result in `ST0` (`00404010`, cdecl).
const FLOAT_MAX: u32 = 0x0040_4010;
/// `(object) -> bool`: a flags byte test (`00454af0`).
const FLAG_SETTING_BYTE: u32 = 0x0045_4af0;
/// Interface singleton accessors (`0045d180`, `004de490`).
const SINGLETON_0045D180: u32 = 0x0045_d180;
const SINGLETON_004DE490: u32 = 0x004d_e490;
/// `(object) -> object + 4` (`00717e50`).
const OBJECT_PLUS_4: u32 = 0x0071_7e50;
/// `(object, value) -> bool` (`005e01b0`).
const OBJECT_TEST_005E01B0: u32 = 0x005e_01b0;
/// `(loading menu)` (`0078aa90`).
const LOADING_MENU_FN_078AA90: u32 = 0x0078_aa90;
/// `(flag)` (cdecl, `0078d2d0`).
const FN_0078D2D0: u32 = 0x0078_d2d0;
/// `LoadingMenu::ResumeBackgroundThread` (name from the engine map).
const LOADING_MENU_RESUME_BACKGROUND_THREAD: u32 = 0x0078_d020;
/// `LoadingMenu::SuspendBackgroundThread` (name from the engine map).
const LOADING_MENU_SUSPEND_BACKGROUND_THREAD: u32 = 0x0078_cfc0;
/// The message menu close helper called next to `Interface::CloseMessageMenu`
/// (`007aa480`).
const MESSAGE_MENU_CLOSE_007AA480: u32 = 0x007a_a480;
/// `FalloutAudioMedia::MediaIsPlaying(8, 1)` (name from the engine map).
const MEDIA_IS_PLAYING: u32 = 0x0083_0750;
/// `(8, path, 0x3e8, 1, 1, 0.0, 0)` (`008300c0`, `falloutaudiomedia.cpp`).
const MEDIA_PLAY: u32 = 0x0083_00c0;
const MEDIA_FN_761460: u32 = 0x0076_1460;
const MEDIA_FN_8304C0: u32 = 0x0083_04c0;
/// `Controls`-style string accessor: the first word of a string object
/// (`00b008d0`).
const STRING_CHARACTERS: u32 = 0x00b0_08d0;
/// `XUserInterface::XUIIsUp` (name from the engine map).
const XUI_IS_UP: u32 = 0x0070_edf0;

// ---- the list boxes' own traits, statics and callees (second half) ---------------

/// `"_enabled"`.
const TRAIT_NAME_ENABLED: u32 = 0x0107_0244;
/// `"_selected"`.
const TRAIT_NAME_SELECTED: u32 = 0x0107_1b64;
/// `"_scroll_delta"`.
const TRAIT_NAME_SCROLL_DELTA: u32 = 0x0106_f90c;
/// `"_number_of_items"`.
const TRAIT_NAME_NUMBER_OF_ITEMS: u32 = 0x0106_f8f8;
/// `"_highlight_y"`.
const TRAIT_NAME_HIGHLIGHT_Y: u32 = 0x0106_f8c0;
/// `"_selected_height"`.
const TRAIT_NAME_SELECTED_HEIGHT: u32 = 0x0106_f8ac;
/// `"lb_highlight_box"`.
const NAME_HIGHLIGHT_BOX: u32 = 0x0106_f898;
/// `"top"`.
const NAME_TOP: u32 = 0x0106_f894;
/// `"bottom"`.
const NAME_BOTTOM: u32 = 0x0106_f88c;
/// `"left"`.
const NAME_LEFT: u32 = 0x0106_f884;
/// `"right"`.
const NAME_RIGHT: u32 = 0x0106_f87c;
/// `double` 1.0 (`01012070`), subtracted from the container's `_num_filtered`
/// value when a filtered-out item is removed.
const ONE_AS_DOUBLE: u32 = 0x0101_2070;
/// Vtable stored by `ListBox<BGSSaveLoadFileEntry*>`'s constructor (RTTI:
/// `ListBox<BGSSaveLoadFileEntry*>`).
const FILE_ENTRY_LIST_BOX_VTABLE: u32 = 0x0107_7094;
/// Vtable stored by the constructor `007d9f90` and the destructor `007d9ff0`
/// (the one `BSSimpleArray<StartMenu::Option*>`'s scalar deleting destructor
/// runs).
const OPTION_ARRAY_VTABLE: u32 = 0x0107_71d8;
/// Vtable stored by the constructor `007da010`.
const USER_OPTION_ARRAY_VTABLE: u32 = 0x0107_71ec;

/// `Tile::IsVisible(tile) -> bool` (name from the engine map).
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
/// `(list, tile) -> bool` (`007242f0`, `fallout/interface/menumanager.cpp`
/// in the engine map): walks the list's items and answers whether one of them
/// has this tile; tested before a tile is highlighted or selected.
const LIST_CONTAINS_TILE: u32 = 0x0072_42f0;
/// `Tile::GetChildByName(tile, name)` (name from the engine map).
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `(tile, name) -> tile` (`00a08b20`, cdecl, `BSMenu/tile.cpp` in the engine
/// map): the tile the name designates relative to the first argument; the
/// list boxes ask for `lb_highlight_box` below their container.
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
/// `(tile, byte)` (`0071aa60`): stores the byte at `tile + 0x35`.
const TILE_STORE_BYTE_35: u32 = 0x0071_aa60;
/// `(tile) -> byte` (`0071aa80`): the byte at `tile + 0x35`.
const TILE_LOAD_BYTE_35: u32 = 0x0071_aa80;
/// `(value, multiple) -> float` (`004bd510`, cdecl, result in `ST0`): rounds
/// `value` up to a multiple of `multiple` (translated in
/// `fallout shared/inventorychanges.cpp`).
const ROUND_UP_TO_MULTIPLE: u32 = 0x004b_d510;
/// `(float) -> float` (`004e4430`, cdecl, result in `ST0`): rounds the value
/// up (translated in `fallout shared/teswater.cpp`); the total-height
/// computation applies it before truncating.
const ROUND_FLOAT: u32 = 0x004e_4430;
/// `(element, flags)` (`005aa3f0`): a scalar deleting destructor whose body is
/// `007d9ec0`; `007d9e80` runs it over elements 12 bytes apart.
const ARRAY_ELEMENT_DESTROY: u32 = 0x005a_a3f0;
/// `(array)` frees the buffer through the array's vtable (`006a8500`).
const ARRAY_BUFFER_RELEASE: u32 = 0x006a_8500;
/// `(array, 0, 0)` zeroes the array's fields (`006b3eb0`).
const ARRAY_INIT: u32 = 0x006b_3eb0;
/// `(array, 1)` the array destructor body (`008454f0`).
const ARRAY_DESTRUCT_BODY: u32 = 0x0084_54f0;
/// The destructor body of the second array (`007da070`, outside this part).
const USER_OPTION_ARRAY_DESTRUCT: u32 = 0x007d_a070;

layout! {
    /// `StartMenu` (Xbox PDB), 0x1CC bytes on the Xbox; the PC build has two
    /// more words before `ENABLED_TRAIT`. Only the fields used here are
    /// declared; the tiles (`pTiles`, +0x28, one pointer per `TileIndex`)
    /// and the list boxes are read through the offsets below.
    pub struct StartMenu: 0x1d4 {
        /// `uiFlags` (Xbox PDB): the `StartMenu::Flag` bits.
        0x1A8 uiFlags: u32,
        /// `fSaveLoadThreshold` (Xbox PDB).
        0x1AC fSaveLoadThreshold: f32,
        /// `pSaveGame` (Xbox PDB): `BGSSaveLoadFileEntry*`.
        0x1B0 pSaveGame: Ptr,
        /// `pBrightnessOption` (Xbox PDB): `UserOption*`.
        0x1B4 pBrightnessOption: Ptr,
        /// `spAlphaMap` (Xbox PDB): `NiPointer<NiSourceTexture>`.
        0x1B8 spAlphaMap: Ptr,
        /// `pDownloadsOption` (Xbox PDB): `Tile*`.
        0x1BC pDownloadsOption: Ptr,
        /// PC only (not in the Xbox PDB): a selector the key-remap code
        /// compares with 0, 1, 2, 3 and 0xFF.
        0x1C8 field_1c8: i32,
        /// PC only (not in the Xbox PDB): the tile `007d42b0` adds for the
        /// option whose change function is `007d30e0`.
        0x1CC field_1cc: Ptr,
        /// `ENABLED_TRAIT` (Xbox PDB, `const u32`; +0x1C8 there).
        0x1D0 ENABLED_TRAIT: u32,
    }

    /// `StartMenu::Option` (Xbox PDB), 0x10 bytes.
    pub struct StartMenuOption: 0x10 {
        /// `pDisplayText` (Xbox PDB): `const char*`.
        0x04 pDisplayText: u32,
        /// `pfChooseFn` (Xbox PDB).
        0x08 pfChooseFn: u32,
        /// `iModes` (Xbox PDB).
        0x0C iModes: u32,
    }

    /// `StartMenu::UserOption` (Xbox PDB), 0x30 bytes; its base `Option`
    /// fields are those of [`StartMenuOption`].
    pub struct StartMenuUserOption: 0x30 {
        /// `pDisplayText` (Xbox PDB).
        0x04 pDisplayText: u32,
        /// `pfChooseFn` (Xbox PDB).
        0x08 pfChooseFn: u32,
        /// `iModes` (Xbox PDB).
        0x0C iModes: u32,
        /// `pTemplate` (Xbox PDB): `const char*`.
        0x10 pTemplate: u32,
        /// `iVal` (Xbox PDB).
        0x14 iVal: i32,
        /// `iDefaultVal` (Xbox PDB).
        0x18 iDefaultVal: i32,
        /// `eType` (Xbox PDB): 0 and 1 toggles, 2 meter.
        0x1C eType: i32,
        /// `iNumSettings` (Xbox PDB).
        0x20 iNumSettings: u32,
        /// `iArrowSpacing` (Xbox PDB).
        0x24 iArrowSpacing: u32,
        /// `pValueStrings` (Xbox PDB): `const char**`.
        0x28 pValueStrings: u32,
        /// `pfChangeFn` (Xbox PDB).
        0x2C pfChangeFn: u32,
    }

    /// `ListBox<T>` / `StartMenu::OptionListBox` (Xbox PDB), 0x30 bytes. The
    /// `BSSimpleList` of items is embedded at +4 (head item +4, next +8).
    pub struct ListBoxLayout: 0x30 {
        /// The embedded list's head item (`m_item`).
        0x04 m_item: Ptr,
        /// The embedded list's next node (`m_pkNext`).
        0x08 m_pkNext: Ptr,
        /// `pContainerRect` (Xbox PDB): `Tile*`.
        0x0C pContainerRect: Ptr,
        /// `pHighlightedItem` (Xbox PDB): `Tile*`.
        0x10 pHighlightedItem: Ptr,
        /// `pScrollBar` (Xbox PDB): `Tile*`.
        0x14 pScrollBar: Ptr,
        /// `pTemplateName` (Xbox PDB): `const char*`.
        0x18 pTemplateName: u32,
        /// `usNextIndex` (Xbox PDB).
        0x1C usNextIndex: u16,
        /// `fTotalListHeight` (Xbox PDB).
        0x20 fTotalListHeight: f32,
        /// `usFlags` (Xbox PDB).
        0x2C usFlags: u16,
    }
}

/// The save/load list box inside the start menu (`lbSaveLoad`).
const LIST_SAVE_LOAD: u32 = 0x174;
/// `lbMain` (Xbox PDB).
const LIST_MAIN: u32 = 0x84;
/// `lbConfirm` (Xbox PDB).
const LIST_CONFIRM: u32 = 0xE4;
/// `lbOptions` (Xbox PDB).
const LIST_OPTIONS: u32 = 0x114;
/// `lbDownloads` (Xbox PDB), a `ListBox<int>`.
const LIST_DOWNLOADS: u32 = 0x144;

/// `StartMenu::pTiles` (Xbox PDB) starts at +0x28; the tile for a
/// `TileIndex` is at `0x28 + 4 * index`.
const TILES: u32 = 0x28;
// `StartMenu::TileIndex` (Xbox PDB) values used here.
const TILE_MAIN_CONTAINER: u32 = 0;
const TILE_OPTIONS_CONTAINER: u32 = 2;
const TILE_MAIN_TITLE: u32 = 3;
const TILE_DEFAULTS_BUTTON: u32 = 4;
const TILE_BACK_BUTTON: u32 = 5;
const TILE_SAVE_LOAD_CONTAINER: u32 = 6;
const TILE_CONFIRM_CONTAINER: u32 = 7;
const TILE_CONFIRM_QUESTION: u32 = 8;
const TILE_VERSION_TEXT: u32 = 10;
const TILE_SCREENSHOT: u32 = 11;
const TILE_PLAYER_NAME: u32 = 12;
const TILE_DLC_CONTAINER: u32 = 14;
const TILE_DEVICE_BUTTON: u32 = 15;
const TILE_CONTROLS_HEADER: u32 = 16;
const TILE_PLAYER_LEVEL: u32 = 18;
const TILE_PLAY_TIME: u32 = 19;
const TILE_CONFIRM_WARNING: u32 = 20;
const TILE_SAVE_VERSION: u32 = 22;

// `StartMenu::Flag` (Xbox PDB) values used here.
const FLAG_INGAME: u32 = 0x1;
const FLAG_OPTIONS_DIRTY: u32 = 0x2;
const FLAG_SAVE_MODE: u32 = 0x8;
const FLAG_FADE_IN: u32 = 0x10;
const FLAG_SAVE_OPTIONS: u32 = 0x20;
const FLAG_DLC_NEEDS_UPDATE: u32 = 0x80;
const FLAG_NEW_DLC_INSTALLED: u32 = 0x100;
const FLAG_DLC_WAIT_MESSAGE: u32 = 0x400;
const FLAG_SYSUTILSAVECOMPLETE: u32 = 0x800;
const FLAG_ACTION_MAPPING: u32 = 0x1000;
const FLAG_DELETE_MODE: u32 = 0x2000;
const FLAG_SAVE_WAIT_MESSAGE: u32 = 0x4000;
const FLAG_SAVE_WAIT_SAVING: u32 = 0x8000;
const FLAG_SHOWING_CREDITS: u32 = 0x1_0000;
const FLAG_DONT_FORCE_BACK: u32 = 0x2_0000;
const FLAG_SAVELOAD_BACKOUT: u32 = 0x4_0000;
const FLAG_IIS_PREFADE: u32 = 0x8_0000;
const FLAG_LOADING_SAVEGAMES: u32 = 0x10_0000;
const FLAG_RESTART_REQUIRED: u32 = 0x40_0000;
const FLAG_DLC_CORRUPT_MESSAGE: u32 = 0x100_0000;
const FLAG_MEM_CARD_REMOVED: u32 = 0x800_0000;
const FLAG_SAVE_SAVING: u32 = 0x1000_0000;

// ---- callees ------------------------------------------------------------------

/// `(menu, mask) -> bool`: `uiFlags & mask != 0` (`[+0x1A8]`, decompiled).
const MENU_GET_FLAG: u32 = 0x004a_4080;
/// `(menu, mask, on)`: sets or clears the `mask` bits of `uiFlags`.
const MENU_SET_FLAG: u32 = 0x0075_f6f0;
/// `(tile, trait, text, notify)`: sets a string value under the tile lock
/// (calls `Tile::Value::SetString`, decompiled).
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `(tile, trait, value, notify)`: sets a float value under the tile lock
/// (calls `Tile::Value::SetFloat`, decompiled).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `(tile, trait, int)`: the same with `(float) int` and notify set.
const TILE_SET_INT: u32 = 0x0070_0320;
/// `(tile, trait) -> float` in `ST0`: the tile's value for the trait, 0
/// when it has none (`Tile::GetValue` then `[value + 8]`, decompiled).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::Poke(tile, trait, value)` (name from the engine map).
const TILE_POKE: u32 = 0x00a0_1290;
/// `Tile::AddUserTrait(name, id) -> trait id` (name from the engine map).
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::IsTrue(tile, trait) -> bool` (name from the engine map).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetValue(tile, trait)` (name from the engine map): the value
/// record or null.
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::GetValueQ(tile, trait)` (name from the engine map).
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// `Tile::Value::ClearActions(value)` (name from the engine map).
const TILE_VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
/// `Tile::Value::AddAction_ov2(value, a, b, c)` (name from the engine map).
const TILE_VALUE_ADD_ACTION: u32 = 0x00a0_9130;
/// `Tile::Value::AddAction(value, a, float)` (name from the engine map).
const TILE_VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
/// `Tile::Value::CalculateValue(value, flag)` (name from the engine map).
const TILE_VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `Tile::TextToTrait(name) -> trait id` (cdecl, name from the engine map).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::GetMenu(tile)` (name from the engine map).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::AddNeedsUpdate(tile, flags)` (name from the engine map).
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Tile::UpdateTile(tile, flag)` (name from the engine map).
const TILE_UPDATE_TILE: u32 = 0x00a0_4640;
/// `Tile::AddFadeControl(tile, trait, from, to, rate, flag)` (name from the
/// engine map).
const TILE_ADD_FADE_CONTROL: u32 = 0x00a0_7c60;
/// `Tile::HasFadeControl(tile, trait) -> bool` (name from the engine map).
const TILE_HAS_FADE_CONTROL: u32 = 0x00a0_7ed0;
/// `Tile::GetFadeFinished(tile, trait) -> bool` (name from the engine map).
const TILE_GET_FADE_FINISHED: u32 = 0x00a0_7fc0;
/// `Tile::GetMenuByClass(class) -> Menu*` (cdecl, name from the engine map).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Menu::RenderTemplate(menu, parent, template name, flag) -> Tile*`
/// (name from the engine map).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// `TileImage::SetSourceTexture(tile, texture)` (name from the engine map).
const TILE_IMAGE_SET_SOURCE_TEXTURE: u32 = 0x00a2_0610;
/// `(tile, trait, int)` (`fallout/interface/interfacemanager.cpp`): sets a
/// whole-number value.
const TILE_SET_INTEGER_VALUE: u32 = 0x0071_5c60;
/// `(tile, string)` (`fallout/interface/interface.cpp`).
const TILE_SET_TEXT_SOURCE: u32 = 0x0070_7e10;
/// `(tile)`: the sum of trait 0xFA1 over the tile and the chain of next
/// siblings that have trait 0xFA8 set (`00a013d0`, decompiled).
const TILE_CHAIN_SUM_FA1: u32 = 0x00a0_13d0;
/// Same for trait 0xFA2 (`00a01440`, decompiled).
const TILE_CHAIN_SUM_FA2: u32 = 0x00a0_1440;
/// Returns `[ecx + 0x28]`, the tile's next sibling (`0045cd60`).
const TILE_NEXT_SIBLING: u32 = 0x0045_cd60;
/// `InterfaceManager::SetCurrentFocusTarget(manager, tile, trait, flag)`
/// (name from the engine map).
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
/// Returns the interface manager singleton (`[011d8a80]`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
/// `bool` (`004b71d0`): a byte flag is set and `004b9930` holds on the
/// object at `011d8c4c`.
const INTERFACE_ACTIVE: u32 = 0x004b_71d0;
/// The interface's message box (`fallout/interface/interface.cpp`): cdecl,
/// the first word is the text, the number of words varies with the call.
const SHOW_MESSAGE: u32 = 0x0070_3e80;
/// `Interface::CloseMessageMenu` (name from the engine map).
const CLOSE_MESSAGE_MENU: u32 = 0x0070_4000;
/// `Interface::IsConsoleVisible` (name from the engine map).
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
/// `InterfaceManager::PlayMenuSound(id)` (cdecl, name from the engine map).
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
/// Static, no arguments: returns a float in `ST0` (`007177c0`).
const INTERFACE_UNIT_SIZE: u32 = 0x0071_77c0;
/// `(setting) -> text`: 0 for null, else `[setting + 4]` (`00403df0`,
/// decompiled). The settings the menu passes are game-setting objects.
const SETTING_STRING: u32 = 0x0040_3df0;
/// Same body as [`SETTING_STRING`] (`004c69f0`).
const SETTING_STRING_ALT: u32 = 0x004c_69f0;
/// `(object) -> [object]` (`00559450`): the first word, e.g. a string's
/// character pointer.
const FIRST_WORD: u32 = 0x0055_9450;
/// `(array) -> count` of a `BSSimpleArray` (`0044ddc0`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `(array, index) -> pointer to the element` (`00877a30`).
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// String constructor on an 8-byte object (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// String destructor (`004037d0`).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `(string object, format, ...)` (`00406f60`, cdecl): formats into the
/// string object.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `sprintf_s(buffer, size, format, ...)` (`00406d00`, cdecl).
const SPRINTF_S: u32 = 0x0040_6d00;
/// `memset(buffer, value, size)` (`00403d30`, cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// `_wcstombs(dest, source, count)` (`00ecacb2`, name from the engine map).
const WCSTOMBS: u32 = 0x00ec_acb2;
/// `(a, b) -> int` string comparison (`00408b20`, cdecl).
const STRING_COMPARE: u32 = 0x0040_8b20;
/// `operator new(size)` (`00401000`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `_ftol2_sse` (`00ec62c0`): the `f64` is the `ST0` value.
const FTOL: u32 = 0x00ec_62c0;
/// `TESGlobal` value getter (`00526ac0`, returns the `float` in `ST0`).
const TES_GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;
/// `TESGlobal` value setter (`0046dce0`, `float` on the stack).
const TES_GLOBAL_SET_VALUE: u32 = 0x0046_dce0;
/// `GetTickCount` wrapper (`00457fe0`).
const TICK_COUNT: u32 = 0x0045_7fe0;
/// Called on `[011dea0c]` it returns the controls object (`00877720`, named
/// `Controls::GetInstance` by the decompiler).
const CONTROLS_GET: u32 = 0x0087_7720;

// ---- small helpers over the game's calls ---------------------------------------

/// `StartMenu::pMe` as a typed pointer (null before the menu exists).
fn instance(e: &Engine) -> Ptr<StartMenu> {
    Ptr::new(e.global::<u32>(START_MENU_INSTANCE))
}

fn get_flag(e: &mut Engine, menu: Ptr<StartMenu>, mask: u32) -> bool {
    e.call(MENU_GET_FLAG, &args![menu, mask]).bool()
}

fn set_flag(e: &mut Engine, menu: Ptr<StartMenu>, mask: u32, on: bool) {
    e.call(MENU_SET_FLAG, &args![menu, mask, on]);
}

fn set_int(e: &mut Engine, tile: Ptr, trait_id: u32, value: i32) {
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

fn set_float(e: &mut Engine, tile: Ptr, trait_id: u32, value: f32, notify: bool) {
    e.call(TILE_SET_FLOAT, &args![tile, trait_id, value, notify]);
}

fn set_string(e: &mut Engine, tile: Ptr, trait_id: u32, text: u32, notify: bool) {
    e.call(TILE_SET_STRING, &args![tile, trait_id, text, notify]);
}

/// The tile's value for a trait as the `ST0` value the getter returns.
fn tile_value(e: &mut Engine, tile: Ptr, trait_id: u32) -> f64 {
    e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64()
}

/// The text of the game setting object at `setting`.
fn setting_text(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_STRING, &args![setting]).u32()
}

/// `Tile::AddUserTrait(name, -1)`: the trait id for a trait name.
fn user_trait(e: &mut Engine, name: u32) -> u32 {
    e.call(TILE_ADD_USER_TRAIT, &args![name, -1i32]).u32()
}

/// The menu's tile for a `TileIndex` (`pTiles[index]`).
fn tile_of(e: &Engine, menu: Ptr<StartMenu>, index: u32) -> Ptr {
    Ptr::new(e.mem.u32(menu.addr() + TILES + 4 * index))
}

/// A list box embedded in the menu.
fn list_at(menu: Ptr<StartMenu>, offset: u32) -> Ptr<ListBoxLayout> {
    menu.byte_add(offset).cast()
}

/// `_ftol2_sse`: the integer the game truncates a value to.
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

fn array_count(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_COUNT, &args![array]).u32()
}

/// `array[index]` of a `BSSimpleArray` of pointers: `00877a30` returns the
/// element's address, the code then reads the pointer.
fn array_item(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(slot)
}

/// The controls object (`[011dea0c]` through `00877720`).
fn controls(e: &mut Engine) -> u32 {
    let owner = e.global::<u32>(CONTROLS_OWNER);
    e.call(CONTROLS_GET, &args![owner]).u32()
}

/// A list box's virtual `GetTileAt(index, 0)` (slot +0x14).
fn list_tile_at(e: &mut Engine, list: Ptr<ListBoxLayout>, index: i32) -> u32 {
    e.vcall(list.addr(), 0x14, &args![index, 0u32]).u32()
}

/// A list box's virtual `GetHighlightedItem()` (slot +4).
fn list_highlighted(e: &mut Engine, list: Ptr<ListBoxLayout>) -> u32 {
    e.vcall(list.addr(), 0x04, &[]).u32()
}

// Translated from 007d3aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the change function (`pfChangeFn`, +0x2C) of every user option, with
/// `FLAG_SAVE_OPTIONS` cleared meanwhile and set again afterwards, and
/// remembers in `pBrightnessOption` the option whose change function is
/// `007d19c0`.
///
/// The change function is called with the option as its only argument; the
/// array element is fetched again for every use, as the code does.
pub fn fn_007d3aa0(e: &mut Engine, this: Ptr<StartMenu>) {
    set_flag(e, this, FLAG_SAVE_OPTIONS, false);
    let mut index = 0;
    while index < array_count(e, USER_OPTIONS_ARRAY) {
        let option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        if e.get(option, StartMenuUserOption::pfChangeFn) != 0 {
            let argument = array_item(e, USER_OPTIONS_ARRAY, index);
            let callee = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
            let change_fn = e.get(callee, StartMenuUserOption::pfChangeFn);
            e.call(change_fn, &args![argument]);
        }
        let option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        if e.get(option, StartMenuUserOption::pfChangeFn) == BRIGHTNESS_CHANGE_FN {
            let brightness = array_item(e, USER_OPTIONS_ARRAY, index);
            e.set(this, StartMenu::pBrightnessOption, Ptr::new(brightness));
        }
        index += 1;
    }
    set_flag(e, this, FLAG_SAVE_OPTIONS, true);
}

// Translated from 007d3b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::PrepSaveLoad` (Xbox PDB): when the save list is empty, adds
/// the "new save" item (template `lb_newsave_template`), enables the list and
/// fades the save/load container and the back button in; otherwise filters
/// the list and finishes the preparation.
pub fn start_menu_prep_save_load(e: &mut Engine, this: Ptr<StartMenu>) {
    let list = list_at(this, LIST_SAVE_LOAD);
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let item = e
            .call(
                SAVE_LOAD_LIST_ADD_ITEM,
                &args![list, 0u32, 0u32, 0u32, NEW_SAVE_TEMPLATE],
            )
            .ptr::<()>();
        let text = setting_text(e, SETTING_011D39C0);
        set_string(e, item, TRAIT_0FC4, text, true);
        let enabled_trait = e.get(this, StartMenu::ENABLED_TRAIT);
        set_int(e, item, enabled_trait, 0);
        e.call(SAVE_LOAD_LIST_SET_ENABLED, &args![list, true]);
        let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
        start_menu_fade(
            e,
            this,
            TILE_SAVE_LOAD_CONTAINER,
            true,
            TRAIT_DEFAULT,
            fade_value,
        );
        start_menu_fade(e, this, TILE_BACK_BUTTON, true, TRAIT_DEFAULT, fade_value);
        let text = setting_text(e, SETTING_011D29E8);
        let defaults_button = tile_of(e, this, TILE_DEFAULTS_BUTTON);
        set_string(e, defaults_button, TRAIT_0FC4, text, true);
        let back_button = tile_of(e, this, TILE_BACK_BUTTON);
        set_int(e, back_button, TRAIT_0FAF, 0);
        set_flag(e, this, FLAG_LOADING_SAVEGAMES, true);
    } else {
        e.call(SAVE_LOAD_LIST_FILTER, &args![list, 0u32]);
        start_menu_finish_prep_save_load(e, this);
    }
}

// Translated from 007d3c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::FinishPrepSaveLoad` (Xbox PDB): prepares the save/load list
/// once its entries are loaded. Drops the autosave entries (and, in save mode,
/// the ones the user cannot overwrite) from the list, shows the entry count
/// as `"%s %u/%i %s"` in the title tile, re-filters the list, sets the
/// "initial focus" traits, and fades the container in (or marks the back
/// button when the list is still being loaded).
///
/// Translation incomplete: 007d3c70 - the /GS stack-cookie check at the end
/// (for the 64-byte count buffer) is not translated.
pub fn start_menu_finish_prep_save_load(e: &mut Engine, this: Ptr<StartMenu>) {
    let list = list_at(this, LIST_SAVE_LOAD);
    let enabled_trait = e.get(this, StartMenu::ENABLED_TRAIT);
    let mut count: u32;
    if get_flag(e, this, FLAG_SAVE_MODE) {
        count = e.call(LIST_COUNT, &args![list]).u32();
        if count > 0 {
            let tile = e.call(LIST_FIRST_TILE, &args![list]).ptr::<()>();
            set_int(e, tile, enabled_trait, 1);
        }
        let mut index = e.call(LIST_COUNT, &args![list]).i32().wrapping_sub(1);
        while index >= 0 {
            let tile = list_tile_at(e, list, index);
            let entry = e.call(LIST_DATA_AT, &args![list, index, 0u32]).u32();
            if tile != 0 && entry != 0 {
                // The exe tests the name only when the entry is not an autosave.
                if e.call(FILE_ENTRY_IS_AUTOSAVE, &args![entry]).bool()
                    || e.call(FILE_ENTRY_NAME_TEST_B10, &args![entry]).bool()
                {
                    e.call(LIST_REMOVE_AT, &args![list, index, 0u32]);
                    count = count.wrapping_sub(1);
                }
            }
            index -= 1;
        }
        if count >= 1000 {
            let tile = e.call(LIST_FIRST_TILE, &args![list]).ptr::<()>();
            e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
            let second = list_tile_at(e, list, 1);
            e.vcall(list.addr(), 0x00, &args![second]);
            e.vcall(list.addr(), 0x10, &[]);
        }
        if count != 0 {
            let suffix = setting_text(e, SETTING_011D25D4);
            let prefix = setting_text(e, SETTING_011D2C4C);
            e.with_stack(0x40, |e, buffer| {
                e.call(
                    SPRINTF_S,
                    &args![
                        buffer,
                        0x40u32,
                        FORMAT_SAVE_COUNT,
                        prefix,
                        count.wrapping_sub(1),
                        1000u32,
                        suffix
                    ],
                );
                let tile = e.call(LIST_FIRST_TILE, &args![list]).ptr::<()>();
                set_string(e, tile, TRAIT_0FC4, buffer.addr(), true);
            });
        }
    } else if e.call(LIST_FIRST_TILE, &args![list]).u32() != 0 {
        let text = setting_text(e, SETTING_011D39F0);
        let tile = e.call(LIST_FIRST_TILE, &args![list]).ptr::<()>();
        set_string(e, tile, TRAIT_0FC4, text, true);
        let tile = e.call(LIST_FIRST_TILE, &args![list]).ptr::<()>();
        set_int(e, tile, enabled_trait, 0);
    }
    e.call(SAVE_LOAD_LIST_SET_ENABLED, &args![list, true]);
    e.call(SAVE_LOAD_LIST_DESELECT_ALL, &args![list, true]);
    e.call(SAVE_LOAD_LIST_FILTER, &args![list, SAVE_LOAD_FILTER_FN]);
    if e.call(INTERFACE_ACTIVE, &[]).bool() {
        let tile = e.call(LIST_FIRST_TILE, &args![list]).ptr::<()>();
        let manager = e.call(INTERFACE_MANAGER, &[]).u32();
        e.call(
            SET_CURRENT_FOCUS_TARGET,
            &args![manager, tile, TRAIT_0FC3, true],
        );
        let current_value = user_trait(e, TRAIT_NAME_CURRENT_VALUE);
        let scroll_bar = e.call(LIST_SCROLL_BAR, &args![list]).ptr::<()>();
        set_float(e, scroll_bar, current_value, 0.0, false);
        let set_in_code = user_trait(e, TRAIT_NAME_SET_IN_CODE);
        let scroll_bar = e.call(LIST_SCROLL_BAR, &args![list]).ptr::<()>();
        e.call(TILE_POKE, &args![scroll_bar, set_in_code, 1.0f32]);
    }
    e.set(this, StartMenu::fSaveLoadThreshold, 0.0);
    let item_count = e.call(LIST_ITEM_COUNT, &args![list]).u32();
    e.call(
        LIST_FOR_RANGE,
        &args![list, SAVE_LOAD_ITEM_INIT_FN, 0u32, item_count],
    );
    let container = tile_of(e, this, TILE_SAVE_LOAD_CONTAINER);
    set_int(e, container, TRAIT_1004, 1);
    if get_flag(e, this, FLAG_LOADING_SAVEGAMES) {
        let back_button = tile_of(e, this, TILE_BACK_BUTTON);
        set_int(e, back_button, TRAIT_0FAF, 1);
    } else {
        let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
        start_menu_fade(
            e,
            this,
            TILE_SAVE_LOAD_CONTAINER,
            true,
            TRAIT_DEFAULT,
            fade_value,
        );
        start_menu_fade(e, this, TILE_BACK_BUTTON, true, TRAIT_DEFAULT, fade_value);
        let text = setting_text(e, SETTING_011D29E8);
        let defaults_button = tile_of(e, this, TILE_DEFAULTS_BUTTON);
        set_string(e, defaults_button, TRAIT_0FC4, text, true);
    }
}

// Translated from 007d4040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The filter the save/load list uses: true when the start menu does not
/// exist, or when it is not in save mode, `entry` is null and the list holds
/// more than one item.
pub fn fn_007d4040(e: &mut Engine, entry: u32) -> bool {
    let menu = instance(e);
    if menu.is_null() {
        return true;
    }
    if !get_flag(e, menu, FLAG_SAVE_MODE) && entry == 0 {
        let count = e
            .call(LIST_COUNT, &args![list_at(menu, LIST_SAVE_LOAD)])
            .u32();
        if count > 1 {
            return true;
        }
    }
    false
}

// Translated from 007d40a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens the confirm dialog for the highlighted save/load entry: picks the
/// question text (delete mode, save mode, otherwise load, with the
/// "in game" button layout when `FLAG_INGAME` is set), selects the entry,
/// disables the list, shows the question, and opens the dialog with
/// `007d4950`; its two offsets are built from the highlighted tile's 0xFB0
/// value and 0xFA2 sum, the container and question tiles' 0xFA1 and 0xFB1
/// values, the scroll bar's 0xFB1 value and the container's `_text_offset`.
pub fn fn_007d40a0(e: &mut Engine, this: Ptr<StartMenu>) {
    let mut buttons = 0u8;
    let question_mode;
    let question;
    if get_flag(e, this, FLAG_DELETE_MODE) {
        question_mode = 0x800u32;
        question = setting_text(e, SETTING_011D4FB4);
    } else if get_flag(e, this, FLAG_SAVE_MODE) {
        question_mode = 0x400;
        question = setting_text(e, SETTING_011D3294);
    } else {
        question_mode = 0x200;
        question = setting_text(e, SETTING_011D3F28);
        if get_flag(e, this, FLAG_INGAME) {
            buttons = 2;
        }
    }
    let list = list_at(this, LIST_SAVE_LOAD);
    if list_highlighted(e, list) == 0 {
        return;
    }
    let highlighted = list_highlighted(e, list);
    e.call(SAVE_LOAD_LIST_SET_SELECTION, &args![list, highlighted]);
    e.call(SAVE_LOAD_LIST_SET_ENABLED, &args![list, false]);
    let container = tile_of(e, this, TILE_SAVE_LOAD_CONTAINER);
    set_int(e, container, TRAIT_1004, 0);
    let confirm_question = tile_of(e, this, TILE_CONFIRM_QUESTION);
    set_string(e, confirm_question, TRAIT_0FC4, question, true);
    e.call(TILE_UPDATE_TILE, &args![confirm_question, false]);
    let tile = list_highlighted(e, list);
    let sum_fa2 = e.call(TILE_CHAIN_SUM_FA2, &args![tile]).f64();
    let scale = if buttons & 2 != 0 { 2.0 } else { 1.0 };
    let tile = list_highlighted(e, list);
    let tile_fb0 = tile_value(e, Ptr::new(tile), TRAIT_0FB0);
    let first = float_to_int(e, tile_fb0 * scale + sum_fa2);
    let container = tile_of(e, this, TILE_SAVE_LOAD_CONTAINER);
    let container_sum = e.call(TILE_CHAIN_SUM_FA1, &args![container]).f64();
    let confirm_question = tile_of(e, this, TILE_CONFIRM_QUESTION);
    let question_fa1 = tile_value(e, confirm_question, TRAIT_0FA1);
    let difference = container_sum - question_fa1;
    let question_fb1 = tile_value(e, confirm_question, TRAIT_0FB1);
    let sum = question_fb1 + difference;
    let scroll_bar = e.call(LIST_SCROLL_BAR, &args![list]).ptr::<()>();
    let scroll_fb1 = tile_value(e, scroll_bar, TRAIT_0FB1);
    let total = scroll_fb1 + sum;
    let trait_id = user_trait(e, TRAIT_NAME_TEXT_OFFSET);
    let container = tile_of(e, this, TILE_SAVE_LOAD_CONTAINER);
    let offset = tile_value(e, container, trait_id);
    let second = float_to_int(e, offset + total);
    fn_007d4950(e, this, question_mode, second as u32, first as u32, buttons);
}

// Translated from 007d42b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::PopulateOptionsListBox` (Xbox PDB): empties and enables the
/// options list, shows its title, and adds one list item for every user
/// option and every plain option whose modes match `modes`.
///
/// The user options whose change function is `007d2fc0` or `007d30e0` are
/// skipped (and `field_1cc` cleared) when the import behind `009f996e`
/// fails for user 0; the one whose change function is `007d13a0` is greyed
/// out while the XUI is up or `[011dea3c]` has bit 4 set. Meter options get
/// their last step as the `0x1007` value, their arrow spacing as `0x1008`.
/// The menu tile's `_center_x` trait is set to the largest of its 0x1005 ..
/// 0x1008 values selected by `modes` (bits 0x08, 0x10, 0x20, 0x40).
pub fn start_menu_populate_options_list_box(e: &mut Engine, this: Ptr<StartMenu>, modes: u32) {
    let options = list_at(this, LIST_OPTIONS);
    e.vcall(options.addr(), 0x1c, &[]);
    e.call(OPTION_LIST_SET_ENABLED, &args![options, true]);
    let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
    start_menu_fade(
        e,
        this,
        TILE_OPTIONS_CONTAINER,
        true,
        TRAIT_DEFAULT,
        fade_value,
    );
    let text = setting_text(e, SETTING_011D42F4);
    let defaults_button = tile_of(e, this, TILE_DEFAULTS_BUTTON);
    set_string(e, defaults_button, TRAIT_0FC4, text, true);

    let mut index = 0;
    while index < array_count(e, USER_OPTIONS_ARRAY) {
        let option = array_item(e, USER_OPTIONS_ARRAY, index);
        if !e.call(OPTION_MATCHES_MODES, &args![option, modes]).bool() {
            index += 1;
            continue;
        }
        let option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        let mut change_fn = e.get(option, StartMenuUserOption::pfChangeFn);
        if change_fn != CONTROLLER_OPTION_CHANGE_FN_A {
            let option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
            change_fn = e.get(option, StartMenuUserOption::pfChangeFn);
        }
        if change_fn == CONTROLLER_OPTION_CHANGE_FN_A || change_fn == CONTROLLER_OPTION_CHANGE_FN_B
        {
            // The import's result 0 lets the option be added; any other
            // result clears `field_1cc` and skips it.
            let result = e.with_stack(0x10, |e, state| {
                e.call(CONTROLLER_STATE_IMPORT, &args![0u32, state]).u32()
            });
            if result != 0 {
                e.set(this, StartMenu::field_1cc, Ptr::new(0));
                index += 1;
                continue;
            }
        }
        let template_option =
            Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        let template = e.get(template_option, StartMenuUserOption::pTemplate);
        let text_option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        let text = e.get(text_option, StartMenuUserOption::pDisplayText);
        let data = array_item(e, USER_OPTIONS_ARRAY, index);
        let item = list_box_option_add_item(e, options, data, text, 0, template);
        let user_option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        if e.get(user_option, StartMenuUserOption::pfChangeFn) == CONTROLLER_OPTION_CHANGE_FN_B {
            e.set(this, StartMenu::field_1cc, item);
            let active = e.call(INTERFACE_ACTIVE, &[]).bool();
            let menu = instance(e);
            let enabled_trait = e.get(menu, StartMenu::ENABLED_TRAIT);
            set_int(e, item, enabled_trait, active as i32);
        }
        let user_option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        if e.get(user_option, StartMenuUserOption::pfChangeFn) == GAMEPAD_OPTION_CHANGE_FN {
            let xui_up = e.call(XUI_IS_UP, &[]).bool();
            let setting_pointer = e.global::<u32>(OBJECT_POINTER_011DEA3C);
            let usable = !xui_up
                && setting_pointer != 0
                && !e
                    .call(OBJECT_FLAG_BYTE_TEST, &args![setting_pointer, 4u32])
                    .bool();
            set_int(e, item, TRAIT_0FAF, usable as i32);
            let menu = instance(e);
            let enabled_trait = e.get(menu, StartMenu::ENABLED_TRAIT);
            set_int(e, item, enabled_trait, usable as i32);
            let children_list = item.byte_add(4);
            let mut node = e.call(FIRST_WORD, &args![children_list]).u32();
            while node != 0 && !children_list.is_null() {
                let head_slot = e.call(LIST_HEAD_ITEM_SLOT, &args![children_list]).u32();
                if e.mem.u32(head_slot) == 0 {
                    break;
                }
                let menu = instance(e);
                let enabled_trait = e.get(menu, StartMenu::ENABLED_TRAIT);
                let slot = e
                    .call(LIST_NODE_ITEM_SLOT, &args![children_list, node])
                    .u32();
                let child = Ptr::<()>::new(e.mem.u32(slot));
                set_int(e, child, enabled_trait, usable as i32);
                let slot = e
                    .call(LIST_NODE_ITEM_SLOT, &args![children_list, node])
                    .u32();
                let child = Ptr::<()>::new(e.mem.u32(slot));
                set_int(e, child, TRAIT_0FAF, usable as i32);
                node = e.call(LIST_NEXT_NODE, &args![children_list, node]).u32();
            }
        }
        let user_option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        if e.get(user_option, StartMenuUserOption::eType) == 2 {
            let steps = e.get(user_option, StartMenuUserOption::iNumSettings);
            e.call(
                TILE_SET_INTEGER_VALUE,
                &args![item, TRAIT_1007, steps.wrapping_sub(1)],
            );
        }
        let user_option = Ptr::<StartMenuUserOption>::new(array_item(e, USER_OPTIONS_ARRAY, index));
        let spacing = e.get(user_option, StartMenuUserOption::iArrowSpacing);
        if spacing != 0 {
            e.call(TILE_SET_INTEGER_VALUE, &args![item, TRAIT_1008, spacing]);
        }
        let option = array_item(e, USER_OPTIONS_ARRAY, index);
        start_menu_display_option(e, this, option, item.addr());
        index += 1;
    }

    let mut index = 0;
    while index < array_count(e, OPTIONS_ARRAY) {
        let option = array_item(e, OPTIONS_ARRAY, index);
        if e.call(OPTION_MATCHES_MODES, &args![option, modes]).bool() {
            let text_option = Ptr::<StartMenuOption>::new(array_item(e, OPTIONS_ARRAY, index));
            let text = e.get(text_option, StartMenuOption::pDisplayText);
            let data = array_item(e, OPTIONS_ARRAY, index);
            let item =
                list_box_option_add_item(e, options, data, text, 0, USER_OPTION_METER_TEMPLATE);
            let text_option = Ptr::<StartMenuOption>::new(array_item(e, OPTIONS_ARRAY, index));
            let text = e.get(text_option, StartMenuOption::pDisplayText);
            e.call(TILE_SET_TEXT_SOURCE, &args![item, text]);
            set_int(e, item, TRAIT_1007, 0x10);
            for (child_id, visible) in [(0x64u32, 0.0f32), (0x66, 0.0), (0x65, 1.0)] {
                let child = e
                    .call(TILE_GET_CHILD_BY_ID, &args![item, child_id])
                    .ptr::<()>();
                let value = e.call(TILE_GET_VALUE_Q, &args![child, TRAIT_0FA3]).u32();
                e.call(TILE_VALUE_CLEAR_ACTIONS, &args![value]);
                e.call(TILE_VALUE_SET_FLOAT, &args![value, visible, true]);
                let value = e.call(TILE_GET_VALUE, &args![child, TRAIT_0FAF]).u32();
                e.call(TILE_VALUE_CLEAR_ACTIONS, &args![value]);
                e.call(TILE_VALUE_SET_FLOAT, &args![value, 0.0f32, true]);
            }
        }
        index += 1;
    }

    let menu_tile = Ptr::<()>::new(e.mem.u32(this.addr() + 4));
    let mut widest = 0.0f32;
    for (mask, trait_id) in [
        (0x08u32, TRAIT_1005),
        (0x10, TRAIT_1006),
        (0x20, TRAIT_1007),
        (0x40, TRAIT_1008),
    ] {
        if modes & mask != 0 {
            let value = tile_value(e, menu_tile, trait_id) as f32;
            widest = e.call(FLOAT_MAX, &args![widest, value]).f32();
        }
    }
    let center_trait = user_trait(e, TRAIT_NAME_CENTER_X);
    set_float(e, menu_tile, center_trait, widest, true);
}

// Translated from 007d48a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens the confirm dialog (`007d49a0`) for the main list. `top` is `0xA8`
/// when `layout & 1`, else `0x50`. The first offset is the main container's
/// 0xFA1 chain sum, truncated, less `top`, taken as an unsigned value and
/// less the interface unit size (`007177c0`), truncated again; the second is
/// the highlighted main-list tile's 0xFA2 chain sum, truncated, less `0x3c`.
/// `top` becomes the dialog's spacing and `layout & 2` selects the warning.
pub fn fn_007d48a0(e: &mut Engine, this: Ptr<StartMenu>, dialog_mode: u32, layout: u8) {
    let top = if layout & 1 != 0 { 0xA8 } else { 0x50 };
    let container = tile_of(e, this, TILE_MAIN_CONTAINER);
    let sum = e.call(TILE_CHAIN_SUM_FA1, &args![container]).f64();
    let sum = float_to_int(e, sum);
    let low = sum.wrapping_sub(top);
    let unit = e.call(INTERFACE_UNIT_SIZE, &[]).f64();
    // The difference is converted as a 64-bit integer whose high word is 0,
    // i.e. as an unsigned 32-bit value.
    let first = float_to_int(e, f64::from(low as u32) - unit);
    let main_list = list_at(this, LIST_MAIN);
    let tile = list_highlighted(e, main_list);
    let sum = e.call(TILE_CHAIN_SUM_FA2, &args![tile]).f64();
    let second = float_to_int(e, sum).wrapping_sub(0x3c);
    fn_007d49a0(
        e,
        this,
        dialog_mode,
        first as u32,
        second as u32,
        top as u32,
        layout & 2 != 0,
    );
}

// Translated from 007d4950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `007d49a0` with the fourth argument `0x55` or, when
/// `layout & 1`, `0xA8`, and the fifth `layout & 2 != 0`.
pub fn fn_007d4950(
    e: &mut Engine,
    this: Ptr<StartMenu>,
    dialog_mode: u32,
    x_value: u32,
    y_value: u32,
    layout: u8,
) {
    let spacing = if layout & 1 != 0 { 0xA8 } else { 0x55 };
    fn_007d49a0(
        e,
        this,
        dialog_mode,
        x_value,
        y_value,
        spacing,
        layout & 2 != 0,
    );
}

// Translated from 007d49a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the confirm dialog (`lbConfirm`): empties and enables the list,
/// fades the confirm container in, sets the container tile's 0xFA1 and
/// 0xFA2 values to `x_value` and `y_value` less the next sibling's 0xFA1 and
/// 0xFA2 chain sums, its 0xFB1 value to `spacing` and the warning tile's
/// visibility to `warning`, and adds an item (text and text source set) for
/// every option whose modes match `dialog_mode`.
pub fn fn_007d49a0(
    e: &mut Engine,
    this: Ptr<StartMenu>,
    dialog_mode: u32,
    x_value: u32,
    y_value: u32,
    spacing: u32,
    warning: bool,
) {
    let confirm = list_at(this, LIST_CONFIRM);
    e.vcall(confirm.addr(), 0x1c, &[]);
    e.call(OPTION_LIST_SET_ENABLED, &args![confirm, true]);
    let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
    start_menu_fade(
        e,
        this,
        TILE_CONFIRM_CONTAINER,
        true,
        TRAIT_DEFAULT,
        fade_value,
    );

    let container = tile_of(e, this, TILE_CONFIRM_CONTAINER);
    let sibling = e.call(TILE_NEXT_SIBLING, &args![container]).ptr::<()>();
    let sum = e.call(TILE_CHAIN_SUM_FA1, &args![sibling]).f64();
    let container = tile_of(e, this, TILE_CONFIRM_CONTAINER);
    set_float(
        e,
        container,
        TRAIT_0FA1,
        (f64::from(x_value as i32) - sum) as f32,
        true,
    );
    let container = tile_of(e, this, TILE_CONFIRM_CONTAINER);
    let sibling = e.call(TILE_NEXT_SIBLING, &args![container]).ptr::<()>();
    let sum = e.call(TILE_CHAIN_SUM_FA2, &args![sibling]).f64();
    let container = tile_of(e, this, TILE_CONFIRM_CONTAINER);
    set_float(
        e,
        container,
        TRAIT_0FA2,
        (f64::from(y_value as i32) - sum) as f32,
        true,
    );
    let container = tile_of(e, this, TILE_CONFIRM_CONTAINER);
    e.call(
        TILE_SET_INTEGER_VALUE,
        &args![container, TRAIT_0FB1, spacing],
    );
    let warning_tile = tile_of(e, this, TILE_CONFIRM_WARNING);
    set_int(e, warning_tile, TRAIT_0FA3, warning as i32);

    let mut index = 0;
    while index < array_count(e, OPTIONS_ARRAY) {
        let option = array_item(e, OPTIONS_ARRAY, index);
        if e.call(OPTION_MATCHES_MODES, &args![option, dialog_mode])
            .bool()
        {
            let data = array_item(e, OPTIONS_ARRAY, index);
            let item = list_box_option_add_item(e, confirm, data, 0, 0, 0);
            let text_option = Ptr::<StartMenuOption>::new(array_item(e, OPTIONS_ARRAY, index));
            let text = e.get(text_option, StartMenuOption::pDisplayText);
            set_string(e, item, TRAIT_0FC4, text, true);
            let text_option = Ptr::<StartMenuOption>::new(array_item(e, OPTIONS_ARRAY, index));
            let text = e.get(text_option, StartMenuOption::pDisplayText);
            e.call(TILE_SET_TEXT_SOURCE, &args![item, text]);
        }
        index += 1;
    }
}

// Translated from 007d4b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::PopulateSaveLoadListBox` (Xbox PDB): resumes the loading
/// menu's background thread when there is one, asks the save/load manager to
/// refresh its list (`00851330`), and adds one save/load list item for each
/// entry of the manager's list, stopping at the first node without an entry.
pub fn start_menu_populate_save_load_list_box(e: &mut Engine, this: Ptr<StartMenu>) {
    if e.global::<u32>(LOADING_MENU) != 0 {
        e.call(LOADING_MENU_RESUME_BACKGROUND_THREAD, &[]);
    }
    let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
    e.call(SAVE_LOAD_MANAGER_REFRESH, &args![manager]);
    let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
    let mut node = e.call(FIRST_WORD, &args![manager]).u32();
    while node != 0 {
        let slot = e.call(LIST_NODE_SELF, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SELF, &args![node]).u32();
        let entry = e.mem.u32(slot);
        e.call(
            SAVE_LOAD_LIST_ADD_ITEM,
            &args![list_at(this, LIST_SAVE_LOAD), entry, 0u32, 0u32, 0u32],
        );
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
}

// Translated from 007d4bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the downloads (DLC) list: adds the `none` item to `lbDownloads` when
/// it is empty, enables it, and fades the DLC container and the back button
/// in.
pub fn fn_007d4bc0(e: &mut Engine, this: Ptr<StartMenu>) {
    let downloads = list_at(this, LIST_DOWNLOADS);
    if e.call(LIST_COUNT, &args![downloads]).u32() == 0 {
        let text = setting_text(e, SETTING_011D3AD4);
        e.call(
            DOWNLOADS_LIST_ADD_ITEM,
            &args![downloads, -1i32, text, 0u32, 0u32],
        );
    }
    e.call(DOWNLOADS_LIST_SET_ENABLED, &args![downloads, true]);
    let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
    start_menu_fade(e, this, TILE_DLC_CONTAINER, true, TRAIT_DEFAULT, fade_value);
    start_menu_fade(e, this, TILE_BACK_BUTTON, true, TRAIT_DEFAULT, fade_value);
}

// Translated from 007d4c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback the DLC enumeration (`00af2b10`) calls once per installed
/// content item `(index, count - 1, 1, name)` and once with `(-1, -1, 0, 0)`
/// at the end. At the end it closes the message menu and shows the
/// downloads list; for an item it converts the item's name from wide to
/// narrow characters (at most 0x80) and adds it to `lbDownloads`.
///
/// Translation incomplete: 007d4c50 - the /GS stack-cookie check at the end
/// (for the 0x80-byte name buffer) is not translated.
pub fn fn_007d4c50(e: &mut Engine, index: i32, count: i32, _unused_3: u32, name: u32) {
    if index == -1 && count == -1 {
        e.call(CLOSE_MESSAGE_MENU, &[]);
        let menu = instance(e);
        fn_007d4bc0(e, menu);
    } else {
        let wide_name = e.call(STRING_CHARACTERS, &args![name]).u32();
        e.with_stack(0x80, |e, narrow| {
            e.call(WCSTOMBS, &args![narrow, wide_name, 0x80u32]);
            let menu = instance(e);
            let downloads = list_at(menu, LIST_DOWNLOADS);
            e.call(
                DOWNLOADS_LIST_ADD_ITEM,
                &args![downloads, 0u32, narrow, 0u32, 0u32],
            );
        });
    }
}

// Translated from 007d4ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::DisplayOption` (Xbox PDB): shows a user option's current
/// value on its list tile. Looks the tile up (`ListBox::GetTileFor` on
/// `lbOptions`) when it is not given. A meter option (`eType` 2) sets the
/// tile's 0x1004 value to `iVal`; another option whose value is in range
/// sets 0x1004 to the value's string. The 0x1006 value then says which arrows
/// show: 1 both, 2 only the right one at the first value, 0 only the left one
/// at the last value (a type-0 option always 1).
pub fn start_menu_display_option(e: &mut Engine, this: Ptr<StartMenu>, option: u32, tile: u32) {
    let mut tile = tile;
    if option != 0 && tile == 0 {
        let found = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), option);
            e.call(
                OPTION_LIST_GET_TILE_FOR,
                &args![list_at(this, LIST_OPTIONS), slot],
            )
            .u32()
        });
        tile = found;
    }
    if option == 0 || tile == 0 {
        return;
    }
    let tile = Ptr::<()>::new(tile);
    let option = Ptr::<StartMenuUserOption>::new(option);
    let value = e.get(option, StartMenuUserOption::iVal);
    let kind = e.get(option, StartMenuUserOption::eType);
    let count = e.get(option, StartMenuUserOption::iNumSettings);
    if kind == 2 {
        set_int(e, tile, TRAIT_1004, value);
    } else {
        let strings = e.get(option, StartMenuUserOption::pValueStrings);
        if strings != 0 && value >= 0 && (value as u32) <= count.wrapping_sub(1) {
            let text = e.mem.u32(strings + 4 * value as u32);
            set_string(e, tile, TRAIT_1004, text, true);
        }
    }
    if kind == 0 {
        set_int(e, tile, TRAIT_1006, 1);
    } else if value == 0 {
        set_int(e, tile, TRAIT_1006, 2);
    } else if value == count.wrapping_sub(1) as i32 {
        set_int(e, tile, TRAIT_1006, 0);
    } else {
        set_int(e, tile, TRAIT_1006, 1);
    }
}

// Translated from 007d4df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::UpdateHighlightedSaveGameInfo` (Xbox PDB): refreshes the
/// panel that shows the highlighted save. A pending `FLAG_SAVELOAD_BACKOUT`
/// is just cleared. Otherwise, for the highlighted entry: the screenshot
/// (or, while the idle timer is armed for it, restarts that timer and hides
/// the screenshot), then for a corrupt entry only the name tile with the
/// corrupt text, else the player name, `"%s %d%s: %s"` level line, play time
/// and `"%s%s: %s"` language line. The tiles' visibility (0xFA3) follows what
/// was filled; the defaults button's `_target` trait follows whether an entry
/// is highlighted.
///
/// The language line passes a blank for the French language setting
/// (`FRENCH` from `0101fd00`) and the empty string otherwise.
///
/// Translation incomplete: 007d4df0 - the exception-unwinding frame (the
/// formatted string temporary's destructor on unwind) is not translated.
pub fn start_menu_update_highlighted_save_game_info(e: &mut Engine, this: Ptr<StartMenu>) {
    if get_flag(e, this, FLAG_SAVELOAD_BACKOUT) {
        set_flag(e, this, FLAG_SAVELOAD_BACKOUT, false);
        return;
    }
    let mut screenshot_shown = false;
    let mut name_shown = false;
    let mut level_shown = false;
    let mut time_shown = false;
    let mut language_shown = false;
    let list = list_at(this, LIST_SAVE_LOAD);
    let entry = e.call(LIST_SELECTED_DATA, &args![list]).u32();
    if entry != 0 {
        let screenshot_tile = tile_of(e, this, TILE_SCREENSHOT);
        let name_tile = tile_of(e, this, TILE_PLAYER_NAME);
        if e.call(INTERFACE_ACTIVE, &[]).bool()
            && e.call(FLAG_SETTING_BYTE, &args![FLAG_SETTING_OBJECT])
                .bool()
        {
            let timer = e.call(IDLE_TIMER_GET, &[]).u32();
            fn_007d5120(e, Ptr::new(timer));
            screenshot_shown = true;
            set_float(e, screenshot_tile, TRAIT_0FA9, 0.0, true);
        } else {
            screenshot_shown = start_menu_set_save_game_screenshot(e, this, entry);
        }
        if e.call(FILE_ENTRY_IS_CORRUPT, &args![entry]).bool() {
            let text = setting_text(e, SETTING_011D1F14);
            set_string(e, name_tile, TRAIT_0FC4, text, true);
            name_shown = true;
        } else {
            let player_name = e.call(FILE_ENTRY_PLAYER_NAME, &args![entry]).u32();
            let mut blank = [0u8, 0u8];
            let language = setting_text(e, SETTING_011C3CB4);
            if e.call(STRING_COMPARE, &args![language, LANGUAGE_FRENCH])
                .i32()
                == 0
            {
                blank[0] = 0x20;
            }
            set_string(e, name_tile, TRAIT_0FC4, player_name, true);
            name_shown = player_name != 0;
            // The exe's frame (`SUB ESP,0x20` under the exception record, then the record
            // link): the text at `EBP-0x20` (+0x10), the blank at `EBP-0x28` (+8).
            e.with_stack(0x30, |e, frame| {
                let text_object = frame.addr() + 0x10;
                let blank_text = frame.addr() + 8;
                e.mem.write(blank_text, &blank);
                e.call(STRING_CONSTRUCT, &args![text_object]);
                let title = fn_007d5140(e, Ptr::new(entry));
                let level = fn_007d5160(e, Ptr::new(entry));
                let level_text = setting_text(e, SETTING_011D3B2C);
                e.call(
                    STRING_FORMAT,
                    &args![
                        text_object,
                        FORMAT_LEVEL,
                        level_text,
                        level,
                        blank_text,
                        title
                    ],
                );
                let level_tile = tile_of(e, this, TILE_PLAYER_LEVEL);
                let characters = e.call(FIRST_WORD, &args![text_object]).u32();
                set_string(e, level_tile, TRAIT_0FC4, characters, true);
                level_shown = true;
                let play_time = fn_007d5180(e, Ptr::new(entry));
                let play_time_text = setting_text(e, SETTING_011D3954);
                e.call(
                    STRING_FORMAT,
                    &args![
                        text_object,
                        FORMAT_TWO_STRINGS,
                        play_time_text,
                        blank_text,
                        play_time
                    ],
                );
                let time_tile = tile_of(e, this, TILE_PLAY_TIME);
                let characters = e.call(FIRST_WORD, &args![text_object]).u32();
                set_string(e, time_tile, TRAIT_0FC4, characters, true);
                time_shown = true;
                let language_name = e.call(FILE_ENTRY_LANGUAGE, &args![entry]).u32();
                let language_text = setting_text(e, SETTING_011D21B4);
                e.call(
                    STRING_FORMAT,
                    &args![
                        text_object,
                        FORMAT_TWO_STRINGS,
                        language_text,
                        blank_text,
                        language_name
                    ],
                );
                let version_tile = tile_of(e, this, TILE_SAVE_VERSION);
                let characters = e.call(FIRST_WORD, &args![text_object]).u32();
                set_string(e, version_tile, TRAIT_0FC4, characters, true);
                language_shown = true;
                e.call(STRING_DESTRUCT, &args![text_object]);
            });
        }
    } else if e.call(INTERFACE_ACTIVE, &[]).bool() {
        let timer = e.call(IDLE_TIMER_GET, &[]).u32();
        e.call(IDLE_TIMER_RESET, &args![timer]);
    }
    let target_trait = user_trait(e, TRAIT_NAME_TARGET);
    let defaults_button = tile_of(e, this, TILE_DEFAULTS_BUTTON);
    set_int(e, defaults_button, target_trait, (entry != 0) as i32);
    let screenshot_tile = tile_of(e, this, TILE_SCREENSHOT);
    set_int(e, screenshot_tile, TRAIT_0FA3, screenshot_shown as i32);
    let name_tile = tile_of(e, this, TILE_PLAYER_NAME);
    set_int(e, name_tile, TRAIT_0FA3, name_shown as i32);
    let level_tile = tile_of(e, this, TILE_PLAYER_LEVEL);
    set_int(e, level_tile, TRAIT_0FA3, level_shown as i32);
    let time_tile = tile_of(e, this, TILE_PLAY_TIME);
    set_int(e, time_tile, TRAIT_0FA3, time_shown as i32);
    let version_tile = tile_of(e, this, TILE_SAVE_VERSION);
    set_int(e, version_tile, TRAIT_0FA3, language_shown as i32);
}

// Translated from 007d5120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a `StartMenu::IdleTimer` (Xbox PDB, 8 bytes): `[+0]` is the tick
/// count now, `[+4]` (the end tick) is cleared.
pub fn fn_007d5120(e: &mut Engine, this: Ptr) {
    let now = e.call(TICK_COUNT, &[]).u32();
    e.mem.set_u32(this.addr(), now);
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 007d5140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry`: loads the entry's data, then returns `pPlayerTitle`
/// (+0x10, Xbox PDB).
pub fn fn_007d5140(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x10)
}

// Translated from 007d5160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry`: loads the entry's data, then returns
/// `iPlayerLevel` (+0x1C, Xbox PDB).
pub fn fn_007d5160(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x1c)
}

// Translated from 007d5180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry`: loads the entry's data, then returns `pPlayTime`
/// (+0x18, Xbox PDB).
pub fn fn_007d5180(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x18)
}

// Translated from 007d51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::SetSaveGameScreenshot` (Xbox PDB): shows the screenshot of
/// `entry` (the highlighted entry when `entry` is null) on the screenshot
/// tile: sets the texture, its size (the image's virtual slots +0x98 and
/// +0x94 into traits 0xFB0 and 0xFB1), and gives the tile's image the alpha
/// map when it has none yet. Returns false when there is no entry or it has
/// no screenshot.
pub fn start_menu_set_save_game_screenshot(
    e: &mut Engine,
    this: Ptr<StartMenu>,
    entry: u32,
) -> bool {
    let mut entry = entry;
    if entry == 0 {
        entry = e
            .call(LIST_SELECTED_DATA, &args![list_at(this, LIST_SAVE_LOAD)])
            .u32();
    }
    let screenshot = if entry != 0 {
        e.call(FILE_ENTRY_GET_SCREENSHOT, &args![entry]).u32()
    } else {
        0
    };
    if screenshot == 0 {
        return false;
    }
    let tile = tile_of(e, this, TILE_SCREENSHOT);
    e.call(TILE_IMAGE_SET_SOURCE_TEXTURE, &args![tile, screenshot]);
    let width = e.vcall(screenshot, 0x98, &[]).u32();
    e.call(TILE_SET_INTEGER_VALUE, &args![tile, TRAIT_0FB0, width]);
    let height = e.vcall(screenshot, 0x94, &[]).u32();
    e.call(TILE_SET_INTEGER_VALUE, &args![tile, TRAIT_0FB1, height]);
    let image_slot = fn_007d5280(e, tile);
    if e.call(IMAGE_ALPHA_MAP, &args![image_slot]).u32() == 0 {
        let alpha_map = e.call(FIRST_WORD, &args![this.byte_add(0x1b8)]).u32();
        let image_slot = fn_007d5280(e, tile);
        e.call(IMAGE_SET_ALPHA_MAP, &args![image_slot, alpha_map]);
    }
    true
}

// Translated from 007d5280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x40` of a tile image (read through `00559450` on the
/// address `this + 0x40`).
pub fn fn_007d5280(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FIRST_WORD, &args![this.byte_add(0x40)]).u32()
}

// Translated from 007d52a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame handling of the action-mapping (key remapping) state
/// (`FLAG_ACTION_MAPPING`): returns false when the flag is not set, true
/// otherwise.
///
/// Once the options container's fade has finished: when its alpha reads
/// zero the container is faded back in (and the `0x1004` value of the DLC
/// container follows whether the interface is active); otherwise, while the
/// menu waits for a remap press (`007ce8f0`) and the console is hidden, the
/// selector `field_1c8` picks the device to scan: 0 the keyboard (keys 0 ..
/// 0xFF, each queried with mode 1), 1 the mouse buttons, 2 the gamepad
/// (buttons, then its two analogue sources). A press is bound with
/// `00a24fc0`; a refused binding shows the message box. `field_1c8` index
/// into the prompt table gives the text for the header tile. Independent of
/// that, a gamepad stick pushed past 0x21F1 on either axis shows another
/// message.
pub fn fn_007d52a0(e: &mut Engine, this: Ptr<StartMenu>) -> bool {
    if !get_flag(e, this, FLAG_ACTION_MAPPING) {
        return false;
    }
    let alpha = user_trait(e, TRAIT_NAME_ALPHA);
    let options_container = tile_of(e, this, TILE_OPTIONS_CONTAINER);
    if !e
        .call(TILE_GET_FADE_FINISHED, &args![options_container, alpha])
        .bool()
    {
        return true;
    }
    let alpha = user_trait(e, TRAIT_NAME_ALPHA);
    let options_container = tile_of(e, this, TILE_OPTIONS_CONTAINER);
    let alpha_value = tile_value(e, options_container, alpha);
    let zero: f64 = e.global(COMPARE_ZERO);
    if alpha_value == zero {
        e.call(FN_007D01B0, &args![this]);
        let alpha = user_trait(e, TRAIT_NAME_ALPHA);
        let confirm_value = e.global::<f32>(FADE_CONFIRM_VALUE);
        start_menu_fade(e, this, TILE_OPTIONS_CONTAINER, true, alpha, confirm_value);
        let active = e.call(INTERFACE_ACTIVE, &[]).bool();
        let device_button = tile_of(e, this, TILE_DEVICE_BUTTON);
        set_int(e, device_button, TRAIT_1004, !active as i32);
    } else if e.call(WAITING_FOR_REMAP_PRESS, &args![this]).bool()
        && !e.call(IS_CONSOLE_VISIBLE, &[]).bool()
    {
        let mut bound = false;
        let selected = e
            .call(LIST_SELECTED_DATA, &args![list_at(this, LIST_OPTIONS)])
            .u32();
        let action = e.mem.u32(selected + 0xc);
        match e.get(this, StartMenu::field_1c8) {
            0 => {
                let mut key = 0u32;
                while key & 0x100 == 0 {
                    let controls_object = controls(e);
                    if e.call(KEY_STATE, &args![controls_object, key, 1u32]).bool() {
                        let controls_object = controls(e);
                        let accepted = e
                            .call(BIND_INPUT, &args![controls_object, action, 0u32, key as u8])
                            .bool();
                        bound = accepted || key == 1;
                        e.call(PLAY_MENU_SOUND, &args![0xau32]);
                        if !bound {
                            show_message_with_icon(e, SETTING_011D4750);
                        }
                        break;
                    }
                    key += 1;
                }
            }
            1 => {
                let mut button = 0i32;
                while !bound {
                    let controls_object = controls(e);
                    let count = e.call(MOUSE_BUTTON_COUNT, &args![controls_object]).i32();
                    if button >= count {
                        break;
                    }
                    let controls_object = controls(e);
                    if e.call(MOUSE_BUTTON_STATE, &args![controls_object, button, 1u32])
                        .bool()
                    {
                        e.call(PLAY_MENU_SOUND, &args![0xau32]);
                        let controls_object = controls(e);
                        bound = e
                            .call(
                                BIND_INPUT,
                                &args![controls_object, action, 1u32, button as u8],
                            )
                            .bool();
                    }
                    button += 1;
                }
                let controls_object = controls(e);
                if e.call(BINDING_CANCEL, &args![controls_object, 0x1cu32, 1u32])
                    .bool()
                {
                    bound = true;
                    let controls_object = controls(e);
                    e.call(
                        CLEAR_BINDING,
                        &args![controls_object, action, 1u32, 0xffu32],
                    );
                    e.call(PLAY_MENU_SOUND, &args![0xau32]);
                }
            }
            2 => {
                let mut button = 0i32;
                while !bound {
                    let controls_object = controls(e);
                    let count = e
                        .call(GAMEPAD_BUTTON_COUNT, &args![controls_object, 0u32])
                        .i32();
                    if button >= count {
                        break;
                    }
                    let controls_object = controls(e);
                    if e.call(
                        GAMEPAD_BUTTON_STATE,
                        &args![controls_object, 0u32, button, 1u32],
                    )
                    .bool()
                    {
                        let controls_object = controls(e);
                        bound = e
                            .call(
                                BIND_INPUT,
                                &args![controls_object, action, 2u32, button as u8],
                            )
                            .bool();
                        e.call(PLAY_MENU_SOUND, &args![0xau32]);
                    }
                    button += 1;
                }
                let mut axis = 0i32;
                while !bound {
                    let controls_object = controls(e);
                    let count = e
                        .call(GAMEPAD_AXIS_COUNT, &args![controls_object, 0u32])
                        .i32();
                    if axis >= count {
                        break;
                    }
                    let controls_object = controls(e);
                    let source = e
                        .call(GAMEPAD_AXIS_SOURCE, &args![controls_object, 0u32, axis])
                        .i32();
                    let mut accepted = false;
                    if source != -1 {
                        let now = e.call(TICK_COUNT, &[]).u32();
                        if now >= e.global::<u32>(REMAP_INPUT_DEADLINE) {
                            let controls_object = controls(e);
                            accepted = e
                                .call(
                                    BIND_INPUT,
                                    &args![controls_object, action, 2u32, source as u8],
                                )
                                .bool();
                        }
                    }
                    bound = accepted;
                    e.call(PLAY_MENU_SOUND, &args![0xau32]);
                    if bound {
                        let now = e.call(GET_TICK_COUNT_IMPORT, &[]).u32();
                        e.set_global(REMAP_INPUT_DEADLINE, now.wrapping_add(0x64));
                    }
                    axis += 1;
                }
                let controls_object = controls(e);
                if e.call(BINDING_CANCEL, &args![controls_object, 0x1cu32, 1u32])
                    .bool()
                {
                    bound = true;
                    let controls_object = controls(e);
                    e.call(
                        CLEAR_BINDING,
                        &args![controls_object, action, 2u32, 0xffu32],
                    );
                    e.call(PLAY_MENU_SOUND, &args![0xau32]);
                }
            }
            _ => {}
        }
        if bound {
            let options = list_at(this, LIST_OPTIONS);
            e.call(OPTION_LIST_SET_ENABLED, &args![options, true]);
            e.call(OPTION_LIST_DESELECT_ALL, &args![options, true]);
            e.call(
                LIST_FOR_RANGE,
                &args![options, OPTION_LIST_REFRESH_FN, 0u32, 0x7fff_ffffu32],
            );
            let selector = e.get(this, StartMenu::field_1c8);
            let prompt_setting = e.mem.u32(REMAP_PROMPT_TABLE + 4 * selector as u32);
            let text = setting_text(e, prompt_setting);
            let header = tile_of(e, this, TILE_CONTROLS_HEADER);
            set_string(e, header, TRAIT_0FC4, text, true);
            set_flag(e, this, FLAG_OPTIONS_DIRTY, true);
        }
    }
    if e.call(INTERFACE_ACTIVE, &[]).bool() && e.call(WAITING_FOR_REMAP_PRESS, &args![this]).bool()
    {
        let controls_object = controls(e);
        let axis_a = e
            .call(GAMEPAD_AXIS_VALUE, &args![controls_object, 0u32, 9u32])
            .i32();
        let controls_object = controls(e);
        let axis_b = e
            .call(GAMEPAD_AXIS_VALUE, &args![controls_object, 0u32, 0xau32])
            .i32();
        let over_a = e.call(ABSOLUTE_VALUE, &args![axis_a]).i32() > 0x21f1;
        if over_a || e.call(ABSOLUTE_VALUE, &args![axis_b]).i32() > 0x21f1 {
            e.call(PLAY_MENU_SOUND, &args![0xau32]);
            show_message_with_icon(e, SETTING_011D3108);
        }
    }
    true
}

/// The message box `007d52a0` shows: the text of `text_setting`, the icon
/// setting's text (`004c69f0` on `011d38b8`) as the ninth word, style 0x12,
/// no callback.
fn show_message_with_icon(e: &mut Engine, text_setting: u32) {
    let icon = e
        .call(SETTING_STRING_ALT, &args![MESSAGE_ICON_SETTING])
        .u32();
    let text = setting_text(e, text_setting);
    e.call(
        SHOW_MESSAGE,
        &args![text, 0u32, 0u32, 0u32, 1u32, 0x12u32, 0.0f32, 0.0f32, icon, 0u32],
    );
}

// Translated from 007d5830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame work of the start menu outside the save/load screen: returns
/// false at once when `FLAG_INGAME` is set, true otherwise.
///
/// * `FLAG_IIS_PREFADE` with a loading menu present: when the mouse query
///   `00a23a50(controls, 0, 2)` holds, calls `0078aa90` on the loading menu.
/// * `FLAG_FADE_IN`, while the main title tile has no fade control and the
///   global at `011c3f3c` is missing or not above zero: a title `0xFA9` value
///   of 0 starts the fade (`Fade(3, …, 1.0)` and `0078d2d0(1)`); a value equal
///   to the constant at `0101e568` rebuilds the menu (`007d6550`), rewrites
///   the seed cell of `0043d4d0` (`min(max(cell + 5 * count, 30), 50)`, the
///   count from `0046fe10`) and clears `FLAG_FADE_IN`.
/// * `FLAG_DLC_WAIT_MESSAGE`: shows a message while the menu of class `0x3e9`
///   is missing, otherwise ends the wait once `0059bb30` reports 1.
/// * `FLAG_NEW_DLC_INSTALLED`: moves to `FLAG_DLC_WAIT_MESSAGE` on the global
///   start menu.
/// * `FLAG_DLC_CORRUPT_MESSAGE`: shows a message and clears itself when the
///   menu of class `0x3e9` is gone.
pub fn fn_007d5830(e: &mut Engine, this: Ptr<StartMenu>) -> bool {
    if get_flag(e, this, FLAG_INGAME) {
        return false;
    }
    let loading_menu = e.global::<u32>(LOADING_MENU);
    if get_flag(e, this, FLAG_IIS_PREFADE) && loading_menu != 0 {
        let controls_object = controls(e);
        if e.call(MOUSE_BUTTON_STATE, &args![controls_object, 0u32, 2u32])
            .bool()
        {
            let loading_menu = e.global::<u32>(LOADING_MENU);
            e.call(LOADING_MENU_FN_078AA90, &args![loading_menu]);
        }
    }
    if get_flag(e, this, FLAG_FADE_IN) {
        let title = tile_of(e, this, TILE_MAIN_TITLE);
        let fading = e
            .call(TILE_HAS_FADE_CONTROL, &args![title, TRAIT_0FA9])
            .bool();
        let zero: f64 = e.global(COMPARE_ZERO);
        let global = e.global::<u32>(TES_GLOBAL_POINTER);
        let global_positive =
            global != 0 && e.call(TES_GLOBAL_GET_VALUE, &args![global]).f64() > zero;
        if !fading && !global_positive {
            let title = tile_of(e, this, TILE_MAIN_TITLE);
            let alpha = tile_value(e, title, TRAIT_0FA9) as f32;
            let one: f64 = e.global(COMPARE_ONE);
            if f64::from(alpha) == zero {
                start_menu_fade(e, this, TILE_MAIN_TITLE, true, TRAIT_0FA9, 1.0);
                e.call(FN_0078D2D0, &args![1u32]);
            } else if f64::from(alpha) == one {
                fn_007d6550(e, this);
                let cell = e.call(SEED_CELL, &args![SEED_CELL_OBJECT]).u32();
                let seed = e.mem.u32(cell);
                let count = e
                    .call(
                        DATA_HANDLER_COUNT,
                        &args![e.global::<u32>(DATA_HANDLER_OBJECT)],
                    )
                    .u32();
                let seed = (seed as i32).wrapping_add((count as i32).wrapping_mul(5));
                let larger = e.call(MAX_INT, &args![seed, 0x1eu32]).i32();
                let clamped = e.call(MIN_INT, &args![larger, 0x32u32]).i32();
                let cell = e.call(SEED_CELL, &args![SEED_CELL_OBJECT]).u32();
                e.mem.set_u32(cell, clamped as u32);
                e.mem.set_u8(SEED_DIRTY_BYTE, 0);
                set_flag(e, this, FLAG_FADE_IN, false);
            }
        }
    }
    if get_flag(e, this, FLAG_DLC_WAIT_MESSAGE) {
        let menu = e.call(TILE_GET_MENU_BY_CLASS, &args![0x3e9u32]).u32();
        if menu == 0 {
            let text = e.call(SETTING_STRING_ALT, &args![SETTING_011D3E2C]).u32();
            e.call(
                SHOW_MESSAGE,
                &args![
                    text,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0x12u32,
                    0.0f32,
                    0.0f32,
                    MESSAGE_FOOTER
                ],
            );
        } else {
            let tile_menu = e.call(TILE_GET_MENU, &args![menu]).u32();
            if e.call(DLC_ENUMERATION_STATE, &args![tile_menu]).u32() == 1 {
                set_flag(e, this, FLAG_DLC_WAIT_MESSAGE, false);
                e.call(DLC_FINISH, &[]);
                fn_007d69f0(e);
            }
        }
    }
    if get_flag(e, this, FLAG_NEW_DLC_INSTALLED) {
        let menu = instance(e);
        set_flag(e, menu, FLAG_DLC_WAIT_MESSAGE, true);
        let menu = instance(e);
        set_flag(e, menu, FLAG_NEW_DLC_INSTALLED, false);
    }
    if get_flag(e, this, FLAG_DLC_CORRUPT_MESSAGE)
        && e.call(TILE_GET_MENU_BY_CLASS, &args![0x3e9u32]).u32() == 0
    {
        set_flag(e, this, FLAG_DLC_CORRUPT_MESSAGE, false);
        let icon = setting_text(e, MESSAGE_ICON_SETTING);
        let text = setting_text(e, SETTING_011D2F7C);
        e.call(
            SHOW_MESSAGE,
            &args![text, 0u32, 0u32, 0u32, 0u32, 0x12u32, 0.0f32, 0.0f32, icon, 0u32],
        );
    }
    true
}

// Translated from 007d5af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::SaveLoadIdle` (Xbox PDB): the per-frame work of the
/// save/load screen. Returns false when the save/load list has no valid
/// container, true otherwise.
///
/// * `FLAG_SAVE_WAIT_MESSAGE`: arms the 3 second (`0xBB8`) wait and switches
///   to `FLAG_SAVE_WAIT_SAVING`.
/// * `FLAG_SAVE_WAIT_SAVING`: starts the save (`FLAG_SAVE_SAVING`) when the
///   chosen entry accepts it (`00846710`) or, without an entry, when the
///   manager's save call (`008503b0`) does; if neither works and nothing has
///   started, shows the "saving" message and leaves the wait. After 1 second
///   of waiting past the deadline the deadline is renewed; once it passes the
///   menu closes (`StartMenu::Close(1)`).
/// * `FLAG_LOADING_SAVEGAMES`: once the list tile has no fade control,
///   populates and finishes the list.
/// * `FLAG_SAVELOAD_BACKOUT`: goes back (`007d62e0`) when the confirm list is
///   not showing.
/// * Otherwise, with the list enabled: re-initializes the items the scroll
///   position newly exposes and clicks (`DoClick(4, 0)`) on the mouse press.
///   Last, with the interface active, a highlight idle of 500 ms or more
///   refreshes the screenshot.
pub fn start_menu_save_load_idle(e: &mut Engine, this: Ptr<StartMenu>) -> bool {
    let list = list_at(this, LIST_SAVE_LOAD);
    if !e.call(LIST_CONTAINER_VALID, &args![list]).bool() {
        return false;
    }
    if get_flag(e, this, FLAG_SAVE_WAIT_MESSAGE) {
        let now = e.call(TICK_COUNT, &[]).u32();
        e.set_global(SAVE_WAIT_DEADLINE, now.wrapping_add(0xbb8));
        set_flag(e, this, FLAG_SAVE_WAIT_MESSAGE, false);
        set_flag(e, this, FLAG_SAVE_WAIT_SAVING, true);
        return true;
    }
    if get_flag(e, this, FLAG_SAVE_WAIT_SAVING) {
        if !get_flag(e, this, FLAG_SAVE_SAVING) {
            let save_game = e.get(this, StartMenu::pSaveGame);
            let mut started = false;
            if !save_game.is_null() {
                started = e.call(FILE_ENTRY_SAVE_GAME, &args![save_game]).bool();
            }
            if !started && e.get(this, StartMenu::pSaveGame).is_null() {
                let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
                started = e
                    .call(
                        SAVE_LOAD_MANAGER_SAVE_GAME,
                        &args![manager, 0u32, -1i32, 0u32],
                    )
                    .bool();
            }
            if started {
                set_flag(e, this, FLAG_SAVE_SAVING, true);
            } else {
                let icon = setting_text(e, MESSAGE_ICON_SETTING);
                let text = setting_text(e, SETTING_011D4420);
                e.call(
                    SHOW_MESSAGE,
                    &args![
                        text,
                        0u32,
                        0u32,
                        FAILED_SAVE_CALLBACK,
                        1u32,
                        0u32,
                        0.0f32,
                        0.0f32,
                        icon,
                        0u32
                    ],
                );
                set_flag(e, this, FLAG_MEM_CARD_REMOVED, true);
                set_flag(e, this, FLAG_SAVE_WAIT_SAVING, false);
                set_flag(e, this, FLAG_SAVE_SAVING, false);
            }
        }
        let now = e.call(TICK_COUNT, &[]).u32();
        let deadline = e.global::<u32>(SAVE_WAIT_DEADLINE);
        if now >= deadline && now.wrapping_sub(deadline) >= 0x3e8 {
            let renewed = e.call(TICK_COUNT, &[]).u32();
            e.set_global(SAVE_WAIT_DEADLINE, renewed.wrapping_add(0x3e8));
        }
        if now >= e.global::<u32>(SAVE_WAIT_DEADLINE) {
            e.call(START_MENU_CLOSE, &args![1u32]);
            set_flag(e, this, FLAG_SAVE_WAIT_SAVING, false);
            set_flag(e, this, FLAG_SAVE_SAVING, false);
        }
        return true;
    }
    if get_flag(e, this, FLAG_LOADING_SAVEGAMES) {
        let alpha = user_trait(e, TRAIT_NAME_ALPHA);
        let container = tile_of(e, this, TILE_SAVE_LOAD_CONTAINER);
        if !e
            .call(TILE_HAS_FADE_CONTROL, &args![container, alpha])
            .bool()
        {
            start_menu_populate_save_load_list_box(e, this);
            start_menu_finish_prep_save_load(e, this);
            set_flag(e, this, FLAG_LOADING_SAVEGAMES, false);
            return true;
        }
    }
    if get_flag(e, this, FLAG_SAVELOAD_BACKOUT)
        && !e
            .call(LIST_CONTAINER_VALID, &args![list_at(this, LIST_CONFIRM)])
            .bool()
    {
        fn_007d62e0(e, this);
        return true;
    }
    if e.call(SAVE_LOAD_LIST_IS_ENABLED, &args![list]).bool() {
        let current_value = user_trait(e, TRAIT_NAME_CURRENT_VALUE);
        let scroll_bar = e.call(LIST_SCROLL_BAR, &args![list]).ptr::<()>();
        let scroll = tile_value(e, scroll_bar, current_value) as f32;
        let threshold = e.get(this, StartMenu::fSaveLoadThreshold);
        if threshold < scroll {
            let count = e.call(LIST_ITEM_COUNT, &args![list]).u32();
            let start = float_to_int(e, f64::from(count) + f64::from(threshold));
            let end = float_to_int(e, f64::from(scroll) - f64::from(threshold)).wrapping_add(start);
            e.call(
                LIST_FOR_RANGE,
                &args![list, SAVE_LOAD_ITEM_INIT_FN, start, end],
            );
            e.set(this, StartMenu::fSaveLoadThreshold, scroll);
        }
        let controls_object = controls(e);
        if e.call(MOUSE_BUTTON_STATE, &args![controls_object, 1u32, 2u32])
            .bool()
            && e.call(LIST_SELECTED_DATA, &args![list]).u32() != 0
        {
            e.vcall(this.addr(), 0x0c, &args![4u32, 0u32]);
        }
    }
    if e.call(INTERFACE_ACTIVE, &[]).bool() {
        let timer = e.call(IDLE_TIMER_GET, &[]).u32();
        if fn_007d5eb0(e, Ptr::new(timer)) >= 0x1f4 {
            if start_menu_set_save_game_screenshot(e, this, 0) {
                let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
                start_menu_fade(e, this, TILE_SCREENSHOT, true, TRAIT_0FA9, fade_value);
            }
            let timer = e.call(IDLE_TIMER_GET, &[]).u32();
            e.call(IDLE_TIMER_RESET, &args![timer]);
        }
    }
    true
}

// Translated from 007d5eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The time a `StartMenu::IdleTimer` has run: 0 when it was never started,
/// otherwise the end tick (`[+4]`) when it has one, or the ticks since the
/// start tick (`[+0]`), minus the start tick.
pub fn fn_007d5eb0(e: &mut Engine, this: Ptr) -> u32 {
    if !fn_007d5f10(e, this) {
        return 0;
    }
    if fn_007d5f30(e, this) {
        e.mem
            .u32(this.addr() + 4)
            .wrapping_sub(e.mem.u32(this.addr()))
    } else {
        let now = e.call(TICK_COUNT, &[]).u32();
        now.wrapping_sub(e.mem.u32(this.addr()))
    }
}

// Translated from 007d5f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `StartMenu::IdleTimer` has been started (its start tick `[+0]` is not 0).
pub fn fn_007d5f10(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) != 0
}

// Translated from 007d5f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `StartMenu::IdleTimer` has an end tick (`[+4]` is not 0).
pub fn fn_007d5f30(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 4) != 0
}

// Translated from 007d5f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::FailedSaveCallbackFunction` (Xbox PDB): clears
/// `FLAG_MEM_CARD_REMOVED` on the start menu, closes the message menu
/// (`007aa480`, `00704000`) and goes back (`007d0e40`).
pub fn start_menu_failed_save_callback_function(e: &mut Engine) {
    let menu = instance(e);
    set_flag(e, menu, FLAG_MEM_CARD_REMOVED, false);
    e.call(MESSAGE_MENU_CLOSE_007AA480, &[]);
    e.call(CLOSE_MESSAGE_MENU, &[]);
    e.call(START_MENU_CHOOSE_BACK, &[]);
}

// Translated from 007d5f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::HideOptionsContainer` (Xbox PDB): hides the version text tile,
/// disables `lbOptions` and fades the options container out. In
/// action-mapping mode it also resets the remap UI: the list's items are
/// refreshed by `007ce970`, the mode flag is cleared, the device button's
/// 0x1004 value is cleared and the controls header is hidden.
pub fn start_menu_hide_options_container(e: &mut Engine, this: Ptr<StartMenu>) {
    let version = tile_of(e, this, TILE_VERSION_TEXT);
    set_int(e, version, TRAIT_0FA3, 0);
    e.call(
        OPTION_LIST_SET_ENABLED,
        &args![list_at(this, LIST_OPTIONS), false],
    );
    let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
    start_menu_fade(
        e,
        this,
        TILE_OPTIONS_CONTAINER,
        false,
        TRAIT_DEFAULT,
        fade_value,
    );
    if get_flag(e, this, FLAG_ACTION_MAPPING) {
        e.call(
            LIST_FOR_RANGE,
            &args![
                list_at(this, LIST_OPTIONS),
                ACTION_MAPPING_ITEM_FN,
                0u32,
                0x7fff_ffffu32
            ],
        );
        set_flag(e, this, FLAG_ACTION_MAPPING, false);
        let device_button = tile_of(e, this, TILE_DEVICE_BUTTON);
        set_int(e, device_button, TRAIT_1004, 0);
        let header = tile_of(e, this, TILE_CONTROLS_HEADER);
        set_int(e, header, TRAIT_0FA3, 0);
    }
}

// Translated from 007d6030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the device selector `field_1c8` names a usable device: false for
/// 0xFF; with the interface active only selector 3 qualifies; otherwise a
/// selector other than 3 qualifies when `007d60b0` says the controls object
/// has that device.
pub fn fn_007d6030(e: &mut Engine, this: Ptr<StartMenu>) -> bool {
    let selector = e.get(this, StartMenu::field_1c8);
    if selector == 0xff {
        return false;
    }
    if e.call(INTERFACE_ACTIVE, &[]).bool() {
        return selector == 3;
    }
    if selector != 3 {
        let controls_object = controls(e);
        let selector = e.get(this, StartMenu::field_1c8);
        if fn_007d60b0(e, Ptr::new(controls_object), selector) != 0 {
            return true;
        }
    }
    false
}

// Translated from 007d60b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-device test on the controls object: device 0 `007d6120`, 1
/// `007d6140`, 2 `007d6100`; any other device gives 0.
pub fn fn_007d60b0(e: &mut Engine, this: Ptr, device: i32) -> u32 {
    match device {
        0 => fn_007d6120(e, this),
        1 => fn_007d6140(e, this),
        2 => fn_007d6100(e, this),
        _ => 0,
    }
}

// Translated from 007d6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x18F4` of the controls object.
pub fn fn_007d6100(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x18f4)
}

// Translated from 007d6120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the controls object's word at `+0x2C` is not zero.
pub fn fn_007d6120(e: &mut Engine, this: Ptr) -> u32 {
    (e.mem.u32(this.addr() + 0x2c) != 0) as u32
}

// Translated from 007d6140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the controls object's word at `+0x30` is not zero.
pub fn fn_007d6140(e: &mut Engine, this: Ptr) -> u32 {
    (e.mem.u32(this.addr() + 0x30) != 0) as u32
}

// Translated from 007d6160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances the device selector `field_1c8` (`007d61b0`) up to four times,
/// stopping at the first selector `007d6030` accepts.
pub fn fn_007d6160(e: &mut Engine, this: Ptr<StartMenu>) {
    let mut remaining = 4u8;
    while remaining != 0 {
        fn_007d61b0(e, this.byte_add(0x1c8));
        if fn_007d6030(e, this) {
            break;
        }
        remaining -= 1;
    }
}

// Translated from 007d61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the `int` at `selector` and wraps it to 0 once it reaches 4;
/// returns the pointer.
pub fn fn_007d61b0(e: &mut Engine, selector: Ptr) -> Ptr {
    let next = e.mem.i32(selector.addr()).wrapping_add(1);
    e.mem
        .set_i32(selector.addr(), if next >= 4 { 0 } else { next });
    selector
}

// Translated from 007d61e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::Fade` (Xbox PDB): starts a fade of tile `index` on `trait_id`
/// (the `_alpha` trait when it is `0x80000000`). A negative `rate` becomes
/// the default at `0101622c`. Fading in does nothing when the trait is
/// already at or above the constant at `0101e568`, else adds a fade control
/// from 0 up to the constant at `01023cd8`; fading out does nothing when the
/// trait is not above zero, else adds one from the constant down to 0.
pub fn start_menu_fade(
    e: &mut Engine,
    this: Ptr<StartMenu>,
    index: u32,
    fade_in: bool,
    trait_id: u32,
    rate: f32,
) {
    let mut trait_id = trait_id;
    let mut rate = rate;
    if trait_id == TRAIT_DEFAULT {
        trait_id = user_trait(e, TRAIT_NAME_ALPHA);
    }
    let zero: f64 = e.global(COMPARE_ZERO);
    if f64::from(rate) < zero {
        rate = e.global(FADE_DEFAULT_VALUE);
    }
    let tile = tile_of(e, this, index);
    if tile.is_null() {
        return;
    }
    if fade_in {
        let value = tile_value(e, tile, trait_id);
        let one: f64 = e.global(COMPARE_ONE);
        if value < one {
            let target: f32 = e.global(FADE_CONTROL_RATE);
            e.call(
                TILE_ADD_FADE_CONTROL,
                &args![tile, trait_id, 0.0f32, target, rate, 0u32],
            );
        }
    } else {
        let value = tile_value(e, tile, trait_id);
        if value > zero {
            let target: f32 = e.global(FADE_CONTROL_RATE);
            e.call(
                TILE_ADD_FADE_CONTROL,
                &args![tile, trait_id, target, 0.0f32, rate, 0u32],
            );
        }
    }
}

// Translated from 007d62e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-enables the save/load list after a confirm dialog: enables it,
/// selects the highlighted item (`007d8d80`), shows the list container
/// again (0x1004 set), and clears `FLAG_SAVELOAD_BACKOUT` unless the
/// interface is active.
pub fn fn_007d62e0(e: &mut Engine, this: Ptr<StartMenu>) {
    let list = list_at(this, LIST_SAVE_LOAD);
    e.call(SAVE_LOAD_LIST_SET_ENABLED, &args![list, true]);
    let highlighted = list_highlighted(e, list);
    e.call(SAVE_LOAD_LIST_SELECT_HIGHLIGHT, &args![list, highlighted]);
    let container = tile_of(e, this, TILE_SAVE_LOAD_CONTAINER);
    set_int(e, container, TRAIT_1004, 1);
    if !e.call(INTERFACE_ACTIVE, &[]).bool() {
        set_flag(e, this, FLAG_SAVELOAD_BACKOUT, false);
    }
}

// Translated from 007d6350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::UserOption::UserOption` (Xbox PDB), cdecl with `this` as the
/// first stack word: builds the `Option` base (`007ce620` with the display
/// text, no choose function, `modes`), installs the `UserOption` vtable,
/// zeroes the value and default, and stores the type, setting count, arrow
/// spacing and change function. For the toggle types (0 and 1) it stores the
/// toggle template name and copies `num_settings` value-string pointers from
/// the trailing arguments into a new array of that many words (a size that
/// overflows 32 bits becomes `0xFFFFFFFF`); for the meter type (2) it stores
/// the meter template name and no array. Returns `this`.
///
/// The trailing arguments are the value strings; a call with fewer than
/// `num_settings` of them panics here (the game reads whatever is on the
/// stack).
///
/// Translation incomplete: 007d6350 - the exception-unwinding frame (the
/// base class destructor if the array allocation throws) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn start_menu_user_option_user_option(
    e: &mut Engine,
    this: Ptr<StartMenuUserOption>,
    display_text: u32,
    change_fn: u32,
    modes: u32,
    kind: i32,
    num_settings: u32,
    arrow_spacing: u32,
    value_strings: &[u32],
) -> Ptr<StartMenuUserOption> {
    e.call(OPTION_CONSTRUCT, &args![this, display_text, 0u32, modes]);
    e.mem.set_u32(this.addr(), USER_OPTION_VTABLE);
    e.set(this, StartMenuUserOption::iVal, 0);
    e.set(this, StartMenuUserOption::iDefaultVal, 0);
    e.set(this, StartMenuUserOption::eType, kind);
    e.set(this, StartMenuUserOption::iNumSettings, num_settings);
    e.set(this, StartMenuUserOption::iArrowSpacing, arrow_spacing);
    e.set(this, StartMenuUserOption::pfChangeFn, change_fn);
    match kind {
        0 | 1 => {
            e.set(
                this,
                StartMenuUserOption::pTemplate,
                USER_OPTION_TOGGLE_TEMPLATE,
            );
            let size = num_settings.saturating_mul(4); // operator new[] overflow passes 0xffffffff
            let array = e.call(OPERATOR_NEW, &args![size]).u32();
            e.set(this, StartMenuUserOption::pValueStrings, array);
            for index in 0..num_settings {
                e.mem
                    .set_u32(array + 4 * index, value_strings[index as usize]);
            }
        }
        2 => {
            e.set(
                this,
                StartMenuUserOption::pTemplate,
                USER_OPTION_METER_TEMPLATE,
            );
            e.set(this, StartMenuUserOption::pValueStrings, 0);
        }
        _ => {}
    }
    this
}

/// The variable-argument entry of `007d6350`: the words are `this`, display
/// text, change function, modes, type, setting count, arrow spacing and the
/// value strings.
fn start_menu_user_option_user_option_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let this = start_menu_user_option_user_option(
        e,
        Ptr::new(words[0]),
        words[1],
        words[2],
        words[3],
        words[4] as i32,
        words[5],
        words[6],
        &words[7..],
    );
    Ret {
        eax: this.addr(),
        ..Ret::default()
    }
}

// Translated from 007d64a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::UserOption::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body (`007d64d0`) and frees the object when bit 0 of `flags`
/// is set. Returns `this`.
pub fn start_menu_user_option_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<StartMenuUserOption>,
    flags: u32,
) -> Ptr<StartMenuUserOption> {
    fn_007d64d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007d64d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `StartMenu::UserOption`: installs the vtable,
/// frees the value-string array and runs the `Option` base destructor
/// (`007ce690`).
pub fn fn_007d64d0(e: &mut Engine, this: Ptr<StartMenuUserOption>) {
    e.mem.set_u32(this.addr(), USER_OPTION_VTABLE);
    let strings = e.get(this, StartMenuUserOption::pValueStrings);
    e.call(OPERATOR_DELETE, &args![strings]);
    e.call(OPTION_DESTRUCT, &args![this]);
}

// Translated from 007d6510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::OptionListBox::GetHighlightTarget` (Xbox PDB): navigation
/// directions 0xF and 0x10 have no target (0); any other goes to
/// `ListBox<StartMenu::Option*>::GetHighlightTarget` (`007d76e0`).
pub fn start_menu_option_list_box_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    direction: u32,
) -> u32 {
    if direction == 0xf || direction == 0x10 {
        0
    } else {
        e.call(OPTION_LIST_GET_HIGHLIGHT_TARGET, &args![this, direction])
            .u32()
    }
}

// Translated from 007d6550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the start menu's lists after the menu was (re)opened, unless the
/// credits are showing (`FLAG_SHOWING_CREDITS`): empties the save/load list,
/// runs `007d6730` unless `FLAG_IIS_PREFADE` is set, and, when the menu is not
/// in game, marks the DLC state pending (`FLAG_DLC_NEEDS_UPDATE`, and
/// `FLAG_DLC_WAIT_MESSAGE` on the global menu when the first byte of the
/// object `00af2c90` returns is set) and refreshes the can-load status
/// (`007cfd20`). Then it runs the user options' change functions
/// (`007d3aa0`) and clears `FLAG_OPTIONS_DIRTY`.
///
/// Afterwards `FLAG_DONT_FORCE_BACK` is simply cleared when set; otherwise,
/// while the global at `011c3f3c` is missing or not above zero, the main list
/// is faded in and enabled, and a pending `FLAG_SHOWING_CREDITS` deselects it
/// and clears the flag.
pub fn fn_007d6550(e: &mut Engine, this: Ptr<StartMenu>) {
    if !get_flag(e, this, FLAG_SHOWING_CREDITS) {
        e.vcall(list_at(this, LIST_SAVE_LOAD).addr(), 0x1c, &[]);
        if !get_flag(e, this, FLAG_IIS_PREFADE) {
            fn_007d6730(e, this);
        }
        if !get_flag(e, this, FLAG_INGAME) {
            let utility = e.call(SYSTEM_UTILITY_INSTANCE, &[]).u32();
            let content_source = fn_007d6720(e, Ptr::new(utility));
            let has_content = e.call(FIRST_BYTE, &args![content_source]).bool();
            let menu = instance(e);
            if has_content && !menu.is_null() && !get_flag(e, menu, FLAG_DLC_WAIT_MESSAGE) {
                let menu = instance(e);
                set_flag(e, menu, FLAG_DLC_WAIT_MESSAGE, true);
                e.mem.set_u8(FLAG_BYTE_PENDING, 1);
            }
            set_flag(e, this, FLAG_DLC_NEEDS_UPDATE, true);
            e.call(UPDATE_CAN_LOAD_STATUS, &args![this]);
        }
        fn_007d3aa0(e, this);
        set_flag(e, this, FLAG_OPTIONS_DIRTY, false);
    }
    if get_flag(e, this, FLAG_DONT_FORCE_BACK) {
        set_flag(e, this, FLAG_DONT_FORCE_BACK, false);
        return;
    }
    let zero: f64 = e.global(COMPARE_ZERO);
    let global = e.global::<u32>(TES_GLOBAL_POINTER);
    if global == 0 || !(e.call(TES_GLOBAL_GET_VALUE, &args![global]).f64() > zero) {
        e.mem.set_u8(FLAG_BYTE_MAIN_LIST, 0);
        let fade_value = e.global::<f32>(FADE_TARGET_VALUE);
        start_menu_fade(
            e,
            this,
            TILE_MAIN_CONTAINER,
            true,
            TRAIT_DEFAULT,
            fade_value,
        );
        e.call(
            OPTION_LIST_SET_ENABLED,
            &args![list_at(this, LIST_MAIN), true],
        );
    }
    if get_flag(e, this, FLAG_SHOWING_CREDITS) {
        let global = e.global::<u32>(TES_GLOBAL_POINTER);
        if global == 0 || !(e.call(TES_GLOBAL_GET_VALUE, &args![global]).f64() > zero) {
            e.call(
                OPTION_LIST_DESELECT_ALL,
                &args![list_at(this, LIST_MAIN), true],
            );
            set_flag(e, this, FLAG_SHOWING_CREDITS, false);
        }
    }
}

// Translated from 007d6720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the result of `00af2c90` (the engine map names it
/// `BSNetworkSystemUtility::QInstance`); the `this` it is called with is not
/// used.
pub fn fn_007d6720(e: &mut Engine, _unused_this: Ptr) -> u32 {
    e.call(NETWORK_SYSTEM_UTILITY_INSTANCE, &[]).u32()
}

// Translated from 007d6730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Backs out of menu levels until the main list is usable again: when the main
/// list has a container and `FLAG_DONT_FORCE_BACK` is clear, repeats
/// `StartMenu::ChooseBack` (`007d0e40`) (and the save/load back-out,
/// `007d62e0`, when `FLAG_SAVELOAD_BACKOUT` is set) until the main list is
/// enabled.
pub fn fn_007d6730(e: &mut Engine, this: Ptr<StartMenu>) {
    let main_list = list_at(this, LIST_MAIN);
    if !e.call(LIST_CONTAINER_VALID, &args![main_list]).bool() {
        return;
    }
    if get_flag(e, this, FLAG_DONT_FORCE_BACK) {
        return;
    }
    while !e.call(OPTION_LIST_IS_ENABLED, &args![main_list]).bool() {
        e.call(START_MENU_CHOOSE_BACK, &[]);
        if get_flag(e, this, FLAG_SAVELOAD_BACKOUT) {
            fn_007d62e0(e, this);
            set_flag(e, this, FLAG_SAVELOAD_BACKOUT, false);
        }
    }
}

// Translated from 007d67b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::InitializeSaveLoadListItem` (Xbox PDB), cdecl: fills a
/// save/load list tile from its entry, once (the `_initialized` trait marks
/// it). The 0x1004 text is the autosave text, the next two predicates'
/// texts, or the save number formatted as `"#%04d"`; the 0x1005 text is the
/// corrupt text or the entry's location.
///
/// Translation incomplete: 007d67b0 - the exception-unwinding frame (the
/// formatted string temporary's destructor on unwind) is not translated.
pub fn start_menu_initialize_save_load_list_item(e: &mut Engine, tile: Ptr, entry: Ptr) {
    if tile.is_null() || entry.is_null() {
        return;
    }
    let initialized = user_trait(e, TRAIT_NAME_INITIALIZED);
    if e.call(TILE_IS_TRUE, &args![tile, initialized]).bool() {
        return;
    }
    if e.call(FILE_ENTRY_IS_AUTOSAVE, &args![entry]).bool() {
        let text = setting_text(e, SETTING_011D3F10);
        set_string(e, tile, TRAIT_1004, text, true);
    } else if e.call(FILE_ENTRY_NAME_TEST_AD0, &args![entry]).bool() {
        let text = setting_text(e, SETTING_011D4E1C);
        set_string(e, tile, TRAIT_1004, text, true);
    } else if e.call(FILE_ENTRY_NAME_TEST_B10, &args![entry]).bool() {
        let text = setting_text(e, SETTING_011D4E04);
        set_string(e, tile, TRAIT_1004, text, true);
    } else {
        // The exe's frame (`SUB ESP,0xc` under the exception record, then the record link):
        // the text at `EBP-0x14` (+8).
        e.with_stack(0x1c, |e, frame| {
            let text_object = frame.addr() + 8;
            e.call(STRING_CONSTRUCT, &args![text_object]);
            let number = fn_007d6950(e, entry);
            e.call(
                STRING_FORMAT,
                &args![text_object, FORMAT_SAVE_NUMBER, number],
            );
            let characters = e.call(FIRST_WORD, &args![text_object]).u32();
            set_string(e, tile, TRAIT_1004, characters, true);
            e.call(STRING_DESTRUCT, &args![text_object]);
        });
    }
    let location = if e.call(FILE_ENTRY_IS_CORRUPT, &args![entry]).bool() {
        setting_text(e, SETTING_011D46E4)
    } else {
        fn_007d6970(e, entry)
    };
    set_string(e, tile, TRAIT_1005, location, true);
    let initialized = user_trait(e, TRAIT_NAME_INITIALIZED);
    set_int(e, tile, initialized, 1);
}

// Translated from 007d6950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry`: loads the entry's data, then returns
/// `iSaveGameNumber` (+0x08, Xbox PDB).
pub fn fn_007d6950(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x08)
}

// Translated from 007d6970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry`: loads the entry's data, then returns `pLocation`
/// (+0x14, Xbox PDB).
pub fn fn_007d6970(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x14)
}

// Translated from 007d6990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl: when `argument` is 0 and the start menu exists and has a
/// downloads option, queries its flag 1 (the result is not used).
pub fn fn_007d6990(e: &mut Engine, argument: u32) {
    if argument != 0 {
        return;
    }
    let menu = instance(e);
    if !menu.is_null() && !e.get(menu, StartMenu::pDownloadsOption).is_null() {
        get_flag(e, menu, FLAG_INGAME);
    }
}

// Translated from 007d69d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `FLAG_SYSUTILSAVECOMPLETE` on the start menu when it exists.
pub fn fn_007d69d0(e: &mut Engine) {
    let menu = instance(e);
    if !menu.is_null() {
        set_flag(e, menu, FLAG_SYSUTILSAVECOMPLETE, true);
    }
}

// Translated from 007d69f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the downloads list (`lbDownloads`): calls `00af2940(utility, 0,
/// -1)` and `00af2b10(utility, 0, 0)` on the utility object `005dfa10`
/// returns for `BSSystemUtility::QInstance`, and, when `0046fdf0` (which
/// returns 0 in this build) says so, reloads through the data handler with
/// the form's flag bit saved around it (`007d6bd0`). It then refreshes the idle manager,
/// closes the message menu, marks `FLAG_DLC_CORRUPT_MESSAGE` when the DLC list
/// is not empty, empties `lbDownloads`, adds its `none` item and adds an item
/// with the narrowed name of every installed content entry.
///
/// Translation incomplete: 007d69f0 - the /GS stack-cookie check at the end
/// (for the 0x104-byte name buffer) is not translated.
pub fn fn_007d69f0(e: &mut Engine) {
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &[]).u32();
    let dlc = e.call(DLC_UTILITY_FROM_SYSTEM, &args![utility]).u32();
    e.call(DLC_UTILITY_FN_AF2940, &args![dlc, 0u32, -1i32]);
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &[]).u32();
    let dlc = e.call(DLC_UTILITY_FROM_SYSTEM, &args![utility]).u32();
    e.call(DLC_UTILITY_FN_AF2B10, &args![dlc, 0u32, 0u32]);
    let handler = e.global::<u32>(DATA_HANDLER_OBJECT);
    if e.call(DATA_HANDLER_IS_LOADED, &args![handler]).u32() != 0 {
        let form = Ptr::new(e.global::<u32>(FLAGGED_FORM_OBJECT));
        let saved = fn_007d6bd0(e, form, false);
        let handler = e.global::<u32>(DATA_HANDLER_OBJECT);
        e.call(DATA_HANDLER_FN_463070, &args![handler, 1u32]);
        let handler = e.global::<u32>(DATA_HANDLER_OBJECT);
        e.call(DATA_HANDLER_FN_465040, &args![handler]);
        let form = Ptr::new(e.global::<u32>(FLAGGED_FORM_OBJECT));
        fn_007d6bd0(e, form, saved);
        e.call(PROCESS_LISTS_FN_970D50, &args![PROCESS_LISTS_OBJECT]);
        let handler = e.global::<u32>(DATA_HANDLER_OBJECT);
        e.call(DATA_HANDLER_FN_4614E0, &args![handler]);
    }
    let idle_manager = e.global::<u32>(IDLE_MANAGER_OBJECT);
    e.call(IDLE_MANAGER_FN_6003C0, &args![idle_manager]);
    e.call(MESSAGE_MENU_CLOSE_007AA480, &[]);
    e.call(CLOSE_MESSAGE_MENU, &[]);
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &[]).u32();
    let dlc = e.call(DLC_UTILITY_FROM_SYSTEM, &args![utility]).u32();
    let content_list = fn_007d6bb0(e, Ptr::new(dlc));
    if !e.call(LIST_IS_EMPTY_NODE, &args![content_list]).bool() {
        let menu = instance(e);
        set_flag(e, menu, FLAG_DLC_CORRUPT_MESSAGE, true);
    }
    let menu = instance(e);
    let downloads = list_at(menu, LIST_DOWNLOADS);
    e.vcall(downloads.addr(), 0x1c, &[]);
    let text = setting_text(e, SETTING_011D3AD4);
    let menu = instance(e);
    e.call(
        DOWNLOADS_LIST_ADD_ITEM,
        &args![list_at(menu, LIST_DOWNLOADS), -1i32, text, 0u32, 0u32],
    );
    let mut node = e.call(LIST_FIRST_NODE, &args![dlc]).u32();
    while node != 0 {
        let slot = e.call(LIST_NODE_SELF, &args![node]).u32();
        let name = e.mem.u32(slot);
        if name != 0 {
            let wide_name = e.call(STRING_CHARACTERS, &args![name]).u32();
            e.with_stack(0x104, |e, narrow| {
                e.call(WCSTOMBS, &args![narrow, wide_name, 0x104u32]);
                let menu = instance(e);
                e.call(
                    DOWNLOADS_LIST_ADD_ITEM,
                    &args![list_at(menu, LIST_DOWNLOADS), 0u32, narrow, 0u32, 0u32],
                );
            });
        }
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
}

// Translated from 007d6bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address 0x14 bytes into the object (the DLC utility's content list).
pub fn fn_007d6bb0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x14)
}

// Translated from 007d6bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00570f40` on the form and then sets bit 0 of the word at `+0x244`
/// when `clear` is false, clears it otherwise; returns what `00570f40`
/// returned.
pub fn fn_007d6bd0(e: &mut Engine, this: Ptr, clear: bool) -> bool {
    let result = e.call(FORM_FN_570F40, &args![this]).bool();
    let flags = e.mem.u32(this.addr() + 0x244);
    let flags = if !clear { flags | 1 } else { flags & !1 };
    e.mem.set_u32(this.addr() + 0x244, flags);
    result
}

// Translated from 007d6c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the start menu exists: sets its menu tile's 0x1009 value to whether
/// the interface is active, calls `007d30b0` with the same flag, and, in
/// action-mapping mode, goes back (`007d0e40`).
pub fn fn_007d6c30(e: &mut Engine) {
    let menu = instance(e);
    if menu.is_null() {
        return;
    }
    let active = e.call(INTERFACE_ACTIVE, &[]).bool();
    let menu_tile = e.call(WORD_AT_4, &args![menu]).ptr::<()>();
    set_int(e, menu_tile, TRAIT_1009, active as i32);
    let active = e.call(INTERFACE_ACTIVE, &[]).bool();
    e.call(START_MENU_FN_7D30B0, &args![active]);
    if get_flag(e, menu, FLAG_ACTION_MAPPING) {
        e.call(START_MENU_CHOOSE_BACK, &[]);
    }
}

// Translated from 007d6c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::_EndCredits` (Xbox PDB): unless the menu is in game, restarts
/// the main title music when the media object 8 is still playing (the path
/// from `01076f3c`, volume ticks 0x3e8), then sets `FLAG_FADE_IN`, clears
/// `FLAG_SHOWING_CREDITS`, sets a main-list byte, zeroes the global at
/// `011c3f3c` and calls `0078cfc0`.
///
/// Translation incomplete: 007d6c90 - the /GS stack-cookie check at the end
/// (for the 0x40-byte path buffer) is not translated.
pub fn start_menu_end_credits(e: &mut Engine, this: Ptr<StartMenu>) {
    if get_flag(e, this, FLAG_INGAME) {
        return;
    }
    if e.call(MEDIA_IS_PLAYING, &args![8u32, 1u32]).bool() {
        e.with_stack(0x40, |e, path| {
            e.call(MEMSET, &args![path, 0u32, 0x40u32]);
            e.call(SPRINTF_S, &args![path, 0x40u32, FORMAT_CREDITS_MEDIA]);
            e.call(MEDIA_FN_761460, &args![1u32]);
            e.call(
                MEDIA_PLAY,
                &args![8u32, path, 0x3e8u32, 1u32, 1u32, 0.0f32, 0u32],
            );
            e.call(MEDIA_FN_8304C0, &[]);
        });
    }
    set_flag(e, this, FLAG_FADE_IN, true);
    set_flag(e, this, FLAG_SHOWING_CREDITS, false);
    e.mem.set_u8(FLAG_BYTE_MAIN_LIST, 1);
    let global = e.global::<u32>(TES_GLOBAL_POINTER);
    e.call(TES_GLOBAL_SET_VALUE, &args![global, 0.0f32]);
    e.call(LOADING_MENU_SUSPEND_BACKGROUND_THREAD, &[]);
}

// Translated from 007d6d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the singleton from `0045d180` and the one from `004de490` to
/// accept their own `+4` value (`005e01b0`; the first answer is not used).
/// When the second accepts, the controls are updated (`00a22e10`) and, if
/// the start menu has `FLAG_RESTART_REQUIRED`, one message is shown and the
/// flag cleared; when it does not accept, another message is shown.
pub fn fn_007d6d70(e: &mut Engine) {
    let first = e.call(SINGLETON_0045D180, &[]).u32();
    let first_value = e.call(OBJECT_PLUS_4, &args![first]).u32();
    let first = e.call(SINGLETON_0045D180, &[]).u32();
    e.call(OBJECT_TEST_005E01B0, &args![first, first_value]);
    let second = e.call(SINGLETON_004DE490, &[]).u32();
    let second_value = e.call(OBJECT_PLUS_4, &args![second]).u32();
    let second = e.call(SINGLETON_004DE490, &[]).u32();
    if e.call(OBJECT_TEST_005E01B0, &args![second, second_value])
        .bool()
    {
        let controls_object = controls(e);
        e.call(CONTROLS_FN_A22E10, &args![controls_object]);
        let menu = instance(e);
        if !menu.is_null() && get_flag(e, menu, FLAG_RESTART_REQUIRED) {
            let icon = e
                .call(SETTING_STRING_ALT, &args![MESSAGE_ICON_SETTING])
                .u32();
            let text = e.call(SETTING_STRING_ALT, &args![SETTING_011D493C]).u32();
            e.call(
                SHOW_MESSAGE,
                &args![text, 0u32, 0u32, 0u32, 0u32, 0x12u32, 0.0f32, 0.0f32, icon, 0u32],
            );
            let menu = instance(e);
            set_flag(e, menu, FLAG_RESTART_REQUIRED, false);
        }
    } else {
        let icon = e
            .call(SETTING_STRING_ALT, &args![MESSAGE_ICON_SETTING])
            .u32();
        let text = e.call(SETTING_STRING_ALT, &args![SETTING_011D3138]).u32();
        e.call(
            SHOW_MESSAGE,
            &args![text, 0u32, 0u32, 0u32, 0u32, 0x12u32, 0.0f32, 0.0f32, icon, 0u32],
        );
    }
}

// Translated from 007d6eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::ListBox` (constructor): builds the base
/// (`007240c0`), the embedded list (`0096a2d0` on `this + 4`), installs the
/// `ListBox<StartMenu::Option*>` vtable, zeroes the container, highlight,
/// scroll bar, template name, flags, next index and the three float fields,
/// runs `007a1950(this, 0, 1)` and `00721130(this)`. Returns `this`.
///
/// Translation incomplete: 007d6eb0 - the exception-unwinding frame (the base
/// destructors if a later step throws) is not translated.
pub fn list_box_option_ptr_construct(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
) -> Ptr<ListBoxLayout> {
    construct_list_box(e, this, OPTION_LIST_BOX_VTABLE)
}

/// The body of the constructor shared by both list box instances: `vtable`
/// is the instance's vtable.
fn construct_list_box(e: &mut Engine, this: Ptr<ListBoxLayout>, vtable: u32) -> Ptr<ListBoxLayout> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(EMBEDDED_LIST_CONSTRUCT, &args![this.byte_add(4)]);
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, ListBoxLayout::pScrollBar, Ptr::new(0));
    e.set(this, ListBoxLayout::pHighlightedItem, Ptr::new(0));
    e.set(this, ListBoxLayout::pContainerRect, Ptr::new(0));
    e.set(this, ListBoxLayout::pTemplateName, 0);
    e.set(this, ListBoxLayout::usFlags, 0);
    e.set(this, ListBoxLayout::usNextIndex, 0);
    e.mem.set_f32(this.addr() + 0x28, 0.0);
    e.mem.set_f32(this.addr() + 0x24, 0.0);
    e.set(this, ListBoxLayout::fTotalListHeight, 0.0);
    e.call(LIST_BOX_FN_7A1950, &args![this, 0u32, 1u32]);
    e.call(LIST_BOX_FN_721130, &args![this]);
    this
}

// Translated from 007d6f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `ListBox<StartMenu::Option*>`: installs the vtable,
/// empties the list (`RemoveAll`, `007d7290`), runs `007211c0` and the
/// embedded list's destructor (`0046ffb0` on `this + 4`).
///
/// Translation incomplete: 007d6f80 - the exception-unwinding frame is not
/// translated.
pub fn list_box_option_ptr_destruct(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    e.mem.set_u32(this.addr(), OPTION_LIST_BOX_VTABLE);
    list_box_option_remove_all(e, this);
    e.call(LIST_BOX_FN_7211C0, &args![this]);
    e.call(EMBEDDED_LIST_DESTRUCT, &args![this.byte_add(4)]);
}

// Translated from 007d6ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::AddItem` (Xbox PDB): renders the list item
/// template (`template`, else the list's own) under the container, makes sure
/// the item has an id value (-1 when missing), sets its text, and links a new
/// `ListBoxItem` for `data`: after `position` when given, else at the end
/// (when auto update is on it then places the tile and recomputes the total
/// height) with the next index stored on the tile. The first item of a list
/// whose container has a positive `_number_of_visible_items` value also gets
/// the actions that keep that value in sync. Returns the item's tile, or null
/// when there is no container or no template.
///
/// Translation incomplete: 007d6ff0 - the exception-unwinding frame (freeing
/// the new `ListBoxItem` if its constructor throws) is not translated.
pub fn list_box_option_add_item(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    data: u32,
    text: u32,
    position: u32,
    template: u32,
) -> Ptr {
    add_item(
        e,
        this,
        (data, text, position, template),
        (OPTION_LIST_SYNCHRONIZE, OPTION_LIST_SET_TOTAL_LIST_HEIGHT),
    )
}

/// The body of `AddItem` shared by both list box instances; the last argument
/// holds the addresses of the instance's `Synchronize` and
/// `SetTotalListHeight`.
fn add_item(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    (data, text, position, template): (u32, u32, u32, u32),
    (synchronize, set_total_list_height): (u32, u32),
) -> Ptr {
    let container = e.get(this, ListBoxLayout::pContainerRect);
    if container.is_null() {
        return Ptr::NULL;
    }
    let list_template = e.get(this, ListBoxLayout::pTemplateName);
    if list_template == 0 && template == 0 {
        return Ptr::NULL;
    }
    let template_name = if template != 0 {
        template
    } else {
        list_template
    };
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let item = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu, container, template_name, 0u32],
        )
        .ptr::<()>();
    if e.call(TILE_GET_VALUE, &args![item, TRAIT_0FAA]).u32() == 0 {
        set_int(e, item, TRAIT_0FAA, -1);
    }
    if text != 0 {
        set_string(e, item, TRAIT_0FC4, text, true);
    }
    let make_list_item = |e: &mut Engine| -> u32 {
        let node = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        if node == 0 {
            return 0;
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), data);
            e.call(LIST_ITEM_CONSTRUCT, &args![node, item, slot]).u32()
        })
    };
    if position != 0 {
        let list_item = make_list_item(e);
        e.call(LIST_INSERT, &args![this.byte_add(4), list_item, position]);
        if e.call(LIST_FLAG_TEST, &args![this, 0u32]).bool() {
            e.call(synchronize, &args![this]);
        }
    } else {
        let list_item = make_list_item(e);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), list_item);
            e.call(LIST_ADD_TAIL, &args![this.byte_add(4), slot]);
        });
        if e.call(LIST_FLAG_TEST, &args![this, 0u32]).bool() {
            e.call(LIST_INIT_TILE_POSITION, &args![this, item]);
            e.call(set_total_list_height, &args![this]);
        }
        let index = e.get(this, ListBoxLayout::usNextIndex);
        e.set(this, ListBoxLayout::usNextIndex, index.wrapping_add(1));
        set_int(e, item, TRAIT_0FAC, i32::from(index));
    }
    if e.call(LIST_COUNT, &args![this]).u32() == 1 {
        let height_trait = user_trait(e, LIST_HEIGHT_TRAIT_NAME);
        let height = tile_value(e, container, height_trait);
        let zero: f64 = e.global(COMPARE_ZERO);
        if height > zero {
            let value = e
                .call(TILE_GET_VALUE_Q, &args![container, TRAIT_0FB0])
                .u32();
            e.call(TILE_VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                TILE_VALUE_ADD_ACTION,
                &args![value, 0x7d0u32, item, TRAIT_0FB0],
            );
            let height_trait = user_trait(e, LIST_HEIGHT_TRAIT_NAME);
            let height = tile_value(e, container, height_trait) as f32;
            e.call(TILE_VALUE_ADD_ACTION_FLOAT, &args![value, 0x7d3u32, height]);
            e.call(TILE_VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    item
}

// Translated from 007d7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::RemoveAll` (Xbox PDB): deletes every list
/// item. For each (the nodes after the head first, then the head): destroys
/// the item's tile (unless the byte at `011d8909` is set), deletes the item's
/// data when the list owns it (flag 1), frees the `ListBoxItem` and the node.
/// Then it clears the head, the next index and the total height, resets the
/// container's `_num_filtered` value, recomputes the height
/// (`007d9460`), clears the highlight and updates the highlight box
/// (`007d9140`).
pub fn list_box_option_remove_all(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    remove_all(
        e,
        this,
        (
            OPTION_LIST_SET_TOTAL_LIST_HEIGHT,
            OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS,
        ),
    );
}

/// The body of `RemoveAll` shared by both list box instances; the last
/// argument holds the addresses of the instance's `SetTotalListHeight` and
/// `UpdateHighlightBoxPos`.
fn remove_all(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    (set_total_list_height, update_highlight_box_pos): (u32, u32),
) {
    loop {
        let node = e.get(this, ListBoxLayout::m_pkNext);
        if node.is_null() {
            break;
        }
        let first_slot = e.call(LIST_NODE_SELF, &args![node]).u32();
        if e.mem.u32(first_slot) == 0 {
            break;
        }
        let following = e.call(WORD_AT_4, &args![node]).u32();
        if e.mem.u8(SKIP_TILE_DESTROY_BYTE) == 0 {
            let node = e.get(this, ListBoxLayout::m_pkNext);
            let slot = e.call(LIST_NODE_SELF, &args![node]).u32();
            let list_item = e.mem.u32(slot);
            let tile = e.mem.u32(list_item);
            if tile != 0 {
                e.vcall(tile, 0x00, &args![1u32]);
            }
        }
        if e.call(LIST_FLAG_TEST, &args![this, 1u32]).bool() {
            let node = e.get(this, ListBoxLayout::m_pkNext);
            let slot = e.call(LIST_NODE_SELF, &args![node]).u32();
            let list_item = e.mem.u32(slot);
            let option = e.mem.u32(list_item + 4);
            if option != 0 {
                e.call(OPTION_SCALAR_DELETING_DESTRUCTOR, &args![option, 1u32]);
            }
        }
        let node = e.get(this, ListBoxLayout::m_pkNext);
        let slot = e.call(LIST_NODE_SELF, &args![node]).u32();
        let list_item = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![list_item]);
        let node = e.get(this, ListBoxLayout::m_pkNext);
        e.call(LIST_NODE_SET_NEXT, &args![node, 0u32]);
        let node = e.get(this, ListBoxLayout::m_pkNext);
        if !node.is_null() {
            e.call(LIST_NODE_SCALAR_DELETING_DESTRUCTOR, &args![node, 1u32]);
        }
        e.set(this, ListBoxLayout::m_pkNext, Ptr::new(following));
    }
    let head = e.get(this, ListBoxLayout::m_item);
    if !head.is_null() {
        if e.mem.u8(SKIP_TILE_DESTROY_BYTE) == 0 {
            let list_item = e.get(this, ListBoxLayout::m_item).addr();
            let tile = e.mem.u32(list_item);
            if tile != 0 {
                e.vcall(tile, 0x00, &args![1u32]);
            }
        }
        if e.call(LIST_FLAG_TEST, &args![this, 1u32]).bool() {
            let list_item = e.get(this, ListBoxLayout::m_item).addr();
            let option = e.mem.u32(list_item + 4);
            if option != 0 {
                e.call(OPTION_SCALAR_DELETING_DESTRUCTOR, &args![option, 1u32]);
            }
        }
        let list_item = e.get(this, ListBoxLayout::m_item);
        e.call(OPERATOR_DELETE, &args![list_item]);
    }
    e.set(this, ListBoxLayout::m_item, Ptr::new(0));
    e.set(this, ListBoxLayout::usNextIndex, 0);
    e.set(this, ListBoxLayout::fTotalListHeight, 0.0);
    let container = e.get(this, ListBoxLayout::pContainerRect);
    if !container.is_null() {
        let reset_trait = e
            .call(TILE_TEXT_TO_TRAIT, &args![LIST_RESET_TRAIT_NAME])
            .u32();
        set_float(e, container, reset_trait, 0.0, true);
    }
    e.call(set_total_list_height, &args![this]);
    e.set(this, ListBoxLayout::pHighlightedItem, Ptr::new(0));
    e.call(update_highlight_box_pos, &args![this]);
}

// Translated from 007d74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::Sort` (Xbox PDB): sorts the embedded list
/// with `argument` (`0083fd60` on `this + 4`) and synchronizes the tiles
/// (`007d8f10`).
pub fn list_box_option_sort(e: &mut Engine, this: Ptr<ListBoxLayout>, argument: u32) {
    e.call(LIST_SORT, &args![this.byte_add(4), argument]);
    e.call(OPTION_LIST_SYNCHRONIZE, &args![this]);
}

// ---- the second half: list box state, selection, removal and the arrays ---------

/// `ListBoxItem` (a node's item, built by `007a62e0`): the tile at +0.
const LIST_ITEM_TILE: u32 = 0;
/// `ListBoxItem`: the data pointer at +4.
const LIST_ITEM_DATA: u32 = 4;
/// `ListBoxItem`: a byte at +8, set by `Filter` to 1 when the filter drops the
/// item.
const LIST_ITEM_FILTERED: u32 = 8;

/// `ListBox<BGSSaveLoadFileEntry*>::Synchronize` (`007d9590`).
const FILE_ENTRY_LIST_SYNCHRONIZE: u32 = 0x007d_9590;
/// `ListBox<BGSSaveLoadFileEntry*>::SetTotalListHeight` (`007d9d50`).
const FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT: u32 = 0x007d_9d50;
/// `ListBox<BGSSaveLoadFileEntry*>::UpdateHighlightBoxPos` (`007d97a0`).
const FILE_ENTRY_LIST_UPDATE_HIGHLIGHT_BOX_POS: u32 = 0x007d_97a0;
/// `ListBox<BGSSaveLoadFileEntry*>::GetAt(list, index, flag) -> item`
/// (name from the engine map).
const FILE_ENTRY_LIST_GET_AT: u32 = 0x007a_6fa0;

/// A trait id the game looks up once, in a function-local static: `guard`
/// holds the "initialized" bits (`bit` is this static's), `cache` the id,
/// `name` the trait's name.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

const fn static_trait_of(guard: u32, bit: u32, cache: u32, name: u32) -> StaticTrait {
    StaticTrait {
        guard,
        bit,
        cache,
        name,
    }
}

const OPTION_SET_ENABLED_TRAIT: StaticTrait =
    static_trait_of(0x011d_ac70, 1, 0x011d_ac6c, TRAIT_NAME_ENABLED);
const OPTION_IS_ENABLED_TRAIT: StaticTrait =
    static_trait_of(0x011d_ac78, 1, 0x011d_ac74, TRAIT_NAME_ENABLED);
const OPTION_IS_SELECTED_TRAIT: StaticTrait =
    static_trait_of(0x011d_ac80, 1, 0x011d_ac7c, TRAIT_NAME_SELECTED);
const DOWNLOADS_SET_ENABLED_TRAIT: StaticTrait =
    static_trait_of(0x011d_ac88, 1, 0x011d_ac84, TRAIT_NAME_ENABLED);
const FILE_ENTRY_FILTER_TRAIT: StaticTrait =
    static_trait_of(0x011d_ac90, 1, 0x011d_ac8c, LIST_RESET_TRAIT_NAME);
const FILE_ENTRY_NUM_UNFILTERED_TRAIT: StaticTrait =
    static_trait_of(0x011d_ac98, 1, 0x011d_ac94, LIST_RESET_TRAIT_NAME);
const FILE_ENTRY_SET_ENABLED_TRAIT: StaticTrait =
    static_trait_of(0x011d_aca0, 1, 0x011d_ac9c, TRAIT_NAME_ENABLED);
const FILE_ENTRY_IS_ENABLED_TRAIT: StaticTrait =
    static_trait_of(0x011d_aca8, 1, 0x011d_aca4, TRAIT_NAME_ENABLED);
const OPTION_SELECT_TRAIT: StaticTrait =
    static_trait_of(0x011d_acb0, 1, 0x011d_acac, TRAIT_NAME_SELECTED);
const OPTION_HIGHLIGHT_Y_TRAIT: StaticTrait =
    static_trait_of(0x011d_acbc, 1, 0x011d_acb8, TRAIT_NAME_HIGHLIGHT_Y);
const OPTION_SELECTED_HEIGHT_TRAIT: StaticTrait =
    static_trait_of(0x011d_acbc, 2, 0x011d_acb4, TRAIT_NAME_SELECTED_HEIGHT);
const FILE_ENTRY_SELECT_TRAIT: StaticTrait =
    static_trait_of(0x011d_acc4, 1, 0x011d_acc0, TRAIT_NAME_SELECTED);
const FILE_ENTRY_HIGHLIGHT_Y_TRAIT: StaticTrait =
    static_trait_of(0x011d_acd0, 1, 0x011d_accc, TRAIT_NAME_HIGHLIGHT_Y);
const FILE_ENTRY_SELECTED_HEIGHT_TRAIT: StaticTrait =
    static_trait_of(0x011d_acd0, 2, 0x011d_acc8, TRAIT_NAME_SELECTED_HEIGHT);
const OPTION_NUM_UNFILTERED_TRAIT: StaticTrait =
    static_trait_of(0x011d_acd8, 1, 0x011d_acd4, LIST_RESET_TRAIT_NAME);

/// A list box's `IsEnabled`.
type ListBoxQuery = fn(&mut Engine, Ptr<ListBoxLayout>) -> bool;
/// A list box's `UpdateHighlightBoxPos`, `SetTotalListHeight`, ...
type ListBoxAction = fn(&mut Engine, Ptr<ListBoxLayout>);
/// A list box's `NumUnfiltered`.
type ListBoxCount = fn(&mut Engine, Ptr<ListBoxLayout>) -> i32;
/// A list box's selection function `(list, tile) -> bool`.
type ListBoxTileAction = fn(&mut Engine, Ptr<ListBoxLayout>, Ptr) -> bool;

/// The trait id of a function-local static: `Tile::AddUserTrait(name, -1)`
/// the first time (the guard bit is set before the call), the cached id
/// afterwards.
fn static_trait(e: &mut Engine, which: StaticTrait) -> u32 {
    let guard = e.mem.u32(which.guard);
    if guard & which.bit == 0 {
        e.mem.set_u32(which.guard, guard | which.bit);
        let id = user_trait(e, which.name);
        e.mem.set_u32(which.cache, id);
    }
    e.mem.u32(which.cache)
}

/// `Tile::IsTrue(tile, trait)`.
fn tile_is_true(e: &mut Engine, tile: Ptr, trait_id: u32) -> bool {
    e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool()
}

/// x87 `FISTP` to 64 bits with truncation, low 32 bits (what the code reads
/// back): the "integer indefinite" value (low half 0) for NaN and for values
/// that do not fit.
fn fistp_truncate(value: f64) -> i32 {
    let wide = if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        i64::MIN
    } else {
        value.trunc() as i64
    };
    wide as i32
}

/// The first node of the list: the embedded list sits at `this + 4` (the code
/// guards a null `this`).
fn first_node(this: Ptr<ListBoxLayout>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// The `ListBoxItem*` in a node's item slot (`006815c0` returns the node's own
/// address, which is the slot).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_NODE_SELF, &args![node]).u32();
    e.mem.u32(slot)
}

/// The tile of a node's item.
fn node_tile(e: &mut Engine, node: u32) -> u32 {
    let item = node_item(e, node);
    e.mem.u32(item + LIST_ITEM_TILE)
}

/// The node after `node` (`00726070`, the word at +4).
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_4, &args![node]).u32()
}

/// `SetEnabled`: sets the container tile's `_enabled` trait.
fn set_enabled(e: &mut Engine, this: Ptr<ListBoxLayout>, which: StaticTrait, enabled: u8) {
    let id = static_trait(e, which);
    let container = e.get(this, ListBoxLayout::pContainerRect);
    set_int(e, container, id, i32::from(enabled));
}

/// `IsEnabled`: the container exists and its `_enabled` trait is true.
fn is_enabled(e: &mut Engine, this: Ptr<ListBoxLayout>, which: StaticTrait) -> bool {
    let id = static_trait(e, which);
    let container = e.get(this, ListBoxLayout::pContainerRect);
    !container.is_null() && tile_is_true(e, container, id)
}

/// `Highlight`: makes `tile` (or nothing, for null) the highlighted item when
/// the list is enabled, the list holds the tile and the tile is enabled.
fn highlight(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    tile: Ptr,
    enabled: ListBoxQuery,
    update_highlight_box_pos: ListBoxAction,
) -> bool {
    if !enabled(e, this) {
        return false;
    }
    if !tile.is_null() {
        if !e.call(LIST_CONTAINS_TILE, &args![this, tile]).bool() {
            return false;
        }
        let id = user_trait(e, TRAIT_NAME_ENABLED);
        if !tile_is_true(e, tile, id) {
            return false;
        }
    }
    e.set(this, ListBoxLayout::pHighlightedItem, tile);
    update_highlight_box_pos(e, this);
    true
}

/// `GetHighlightTarget`: the tile a move in `direction` lands on, or null.
/// Directions 1 and 2 step by -1 and +1; 0xF and 0x10 step by minus and plus
/// one page (the container's `0xFB0` value over its `_scroll_delta` value,
/// rounded up and truncated); the others do not move. The step is added to
/// the highlighted item's index (`0xFAC` trait; the first or last index when
/// nothing is highlighted), clamped, and the first enabled tile at or beyond
/// that index in the stepping direction (then back the other way) is chosen.
/// The result is null when it is the highlighted tile itself.
fn highlight_target(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    direction: u32,
    enabled: ListBoxQuery,
) -> u32 {
    let container = e.get(this, ListBoxLayout::pContainerRect);
    let step: i32 = match direction {
        1 => -1,
        2 => 1,
        0xf | 0x10 => {
            if container.is_null() {
                0
            } else {
                let visible = tile_value(e, container, TRAIT_0FB0);
                let visible = if direction == 0xf { -visible } else { visible };
                let delta_trait = e
                    .call(TILE_TEXT_TO_TRAIT, &args![TRAIT_NAME_SCROLL_DELTA])
                    .u32();
                let delta = tile_value(e, container, delta_trait);
                let pages = (visible / delta) as f32;
                let rounded = e.call(ROUND_UP_TO_MULTIPLE, &args![pages, 1.0f32]).f32();
                float_to_int(e, f64::from(rounded))
            }
        }
        _ => 0,
    };
    let mut found = 0u32;
    let count = i32::from(e.get(this, ListBoxLayout::usNextIndex));
    if enabled(e, this) && step != 0 && count != 0 {
        let last = count - 1;
        let highlighted = e.get(this, ListBoxLayout::pHighlightedItem);
        let mut index = if !highlighted.is_null() {
            let current_value = tile_value(e, highlighted, TRAIT_0FAC);
            let current = float_to_int(e, current_value);
            let moved = current.wrapping_add(step);
            if moved < 0 {
                0
            } else if moved > last {
                last
            } else {
                moved
            }
        } else if step > 0 {
            0
        } else {
            last
        };
        loop {
            // the item whose index trait equals `index`
            let mut node = first_node(this);
            while found == 0 && node != 0 {
                if node_item(e, node) == 0 {
                    break;
                }
                let tile = node_tile(e, node);
                let value = tile_value(e, Ptr::new(tile), TRAIT_0FAC);
                if value == f64::from(index) {
                    found = node_tile(e, node);
                }
                node = node_next(e, node);
            }
            if found == 0 {
                break;
            }
            let id = user_trait(e, TRAIT_NAME_ENABLED);
            if tile_is_true(e, Ptr::new(found), id) {
                break;
            }
            found = 0;
            if step > 0 {
                index += 1;
                if index > last {
                    // past the end: walk back from the one before the last
                    index = last - 1;
                    loop {
                        if found != 0 {
                            let id = user_trait(e, TRAIT_NAME_ENABLED);
                            if tile_is_true(e, Ptr::new(found), id) {
                                break;
                            }
                        }
                        if index < 0 {
                            break;
                        }
                        found = list_tile_at(e, this, index);
                        index -= 1;
                    }
                    break;
                }
            } else {
                index -= 1;
                if index < 0 {
                    // before the start: walk forward from index 1
                    index = 1;
                    loop {
                        if found != 0 {
                            let id = user_trait(e, TRAIT_NAME_ENABLED);
                            if tile_is_true(e, Ptr::new(found), id) {
                                break;
                            }
                        }
                        if index > last {
                            break;
                        }
                        found = list_tile_at(e, this, index);
                        index += 1;
                    }
                    break;
                }
            }
        }
    }
    if found != e.get(this, ListBoxLayout::pHighlightedItem).addr() {
        found
    } else {
        0
    }
}

/// The selection function behind `SetSelection` and its sibling: when the
/// list is enabled and holds the tile, sets the tile's `_selected` trait for
/// `mode` 0 (selects when it is not), 1 (deselects when it is) or 2 (toggles).
/// True when the trait was written.
fn select(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    tile: Ptr,
    mode: u32,
    (enabled, which): (ListBoxQuery, StaticTrait),
) -> bool {
    if !enabled(e, this) || tile.is_null() {
        return false;
    }
    if !e.call(LIST_CONTAINS_TILE, &args![this, tile]).bool() {
        return false;
    }
    let id = static_trait(e, which);
    let selected = tile_is_true(e, tile, id);
    match mode {
        0 if !selected => {
            set_int(e, tile, id, 1);
            true
        }
        1 if selected => {
            set_int(e, tile, id, 0);
            true
        }
        2 => {
            set_int(e, tile, id, i32::from(!selected));
            true
        }
        _ => false,
    }
}

/// `DeselectAll`: runs `deselect` on the tile of every item; with
/// `skip_filtered` set the items the filter dropped are left alone.
fn deselect_all(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    skip_filtered: u8,
    deselect: ListBoxTileAction,
) {
    let mut node = first_node(this);
    while node != 0 {
        if node_item(e, node) == 0 {
            break;
        }
        let skip = skip_filtered != 0 && {
            let item = node_item(e, node);
            e.mem.u8(item + LIST_ITEM_FILTERED) != 0
        };
        if !skip {
            let tile = node_tile(e, node);
            deselect(e, this, Ptr::new(tile));
        }
        node = node_next(e, node);
    }
}

/// `Synchronize`: numbers the unfiltered items again (the `0xFAC` trait and
/// the position of each), then recomputes the total height and the highlight
/// box.
fn synchronize(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    (set_total_list_height, update_highlight_box_pos): (ListBoxAction, ListBoxAction),
) {
    e.set(this, ListBoxLayout::usNextIndex, 0);
    e.set(this, ListBoxLayout::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 {
        if node_item(e, node) == 0 {
            break;
        }
        let item = node_item(e, node);
        if e.mem.u8(item + LIST_ITEM_FILTERED) == 0 {
            let index = e.get(this, ListBoxLayout::usNextIndex);
            e.set(this, ListBoxLayout::usNextIndex, index.wrapping_add(1));
            let tile = node_tile(e, node);
            set_int(e, Ptr::new(tile), TRAIT_0FAC, i32::from(index));
            let tile = node_tile(e, node);
            e.call(LIST_INIT_TILE_POSITION, &args![this, tile]);
        }
        node = node_next(e, node);
    }
    set_total_list_height(e, this);
    update_highlight_box_pos(e, this);
}

/// `NumUnfiltered`: the number of items minus the container's
/// `_num_filtered` value (truncated).
fn num_unfiltered(e: &mut Engine, this: Ptr<ListBoxLayout>, which: StaticTrait) -> i32 {
    let id = static_trait(e, which);
    let items = e.call(LIST_COUNT, &args![this]).i32();
    let container = e.get(this, ListBoxLayout::pContainerRect);
    let filtered = fistp_truncate(tile_value(e, container, id));
    items.wrapping_sub(filtered)
}

/// A tile's flag byte (`tile + 0x35`) set to `flag`, and the tile queued for
/// an update (flags 8).
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_STORE_BYTE_35, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

/// `UpdateHighlightBoxPos`: without a highlighted item the container's
/// `_highlight_y` is -1 and `_selected_height` 0; otherwise both follow the
/// highlighted tile (the `0xFA2` and `0xFB0` values) through actions, and the
/// `lb_highlight_box` tile and its four edges get their flag byte set to the
/// highlighted tile's own flag byte.
fn update_highlight_box_pos(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    (y_static, height_static): (StaticTrait, StaticTrait),
) {
    let y_trait = static_trait(e, y_static);
    let height_trait = static_trait(e, height_static);
    let container = e.get(this, ListBoxLayout::pContainerRect);
    if container.is_null() {
        return;
    }
    let highlighted = e.get(this, ListBoxLayout::pHighlightedItem);
    if highlighted.is_null() {
        set_int(e, container, y_trait, -1);
        set_int(e, container, height_trait, 0);
        return;
    }
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_value = e
        .call(TILE_GET_VALUE, &args![container, height_trait])
        .u32();
    e.call(
        TILE_VALUE_ADD_ACTION,
        &args![y_value, 0x7d0u32, highlighted, TRAIT_0FA2],
    );
    e.call(TILE_VALUE_CALCULATE, &args![y_value, 0u32]);
    e.call(
        TILE_VALUE_ADD_ACTION,
        &args![height_value, 0x7d0u32, highlighted, TRAIT_0FB0],
    );
    e.call(TILE_VALUE_CALCULATE, &args![height_value, 0u32]);
    let flag = u32::from(e.call(TILE_LOAD_BYTE_35, &args![highlighted]).bool());
    let box_tile = e
        .call(TILE_FIND_BY_NAME, &args![container, NAME_HIGHLIGHT_BOX])
        .u32();
    if box_tile == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![box_tile, NAME_TOP])
        .u32();
    if top != 0 {
        mark_tile(e, top, flag);
        // the other three edges are not checked for null
        for name in [NAME_BOTTOM, NAME_LEFT, NAME_RIGHT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![box_tile, name]).u32();
            mark_tile(e, edge, flag);
        }
    }
    mark_tile(e, box_tile, flag);
}

/// `SetTotalListHeight`: when the container's visible height (`0xFB0`) is
/// smaller than the total list height, the list needs
/// `max(1 + trunc(round_up((total - visible) * unfiltered / total)), 2)`
/// scroll steps; the container's `_scroll_delta` becomes
/// `(total - visible) / (steps - 1)` when there are more than one, and the
/// scroll bar's `_number_of_items` the number of steps.
fn set_total_list_height(e: &mut Engine, this: Ptr<ListBoxLayout>, unfiltered: ListBoxCount) {
    let container = e.get(this, ListBoxLayout::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = tile_value(e, container, TRAIT_0FB0) as f32;
    let mut steps = 1i32;
    let total = e.get(this, ListBoxLayout::fTotalListHeight);
    let zero: f64 = e.global(COMPARE_ZERO);
    if f64::from(total) > zero && f64::from(visible) > zero {
        if f64::from(visible) < f64::from(total) {
            let count = unfiltered(e, this) as u32;
            let ratio = (f64::from(count) * (f64::from(total) - f64::from(visible))
                / f64::from(total)) as f32;
            let rounded = e.call(ROUND_FLOAT, &args![ratio]).f32();
            let whole = float_to_int(e, f64::from(rounded));
            steps = e.call(MAX_INT, &args![whole.wrapping_add(1), 2i32]).i32();
        }
        if steps > 1 {
            let delta = ((f64::from(total) - f64::from(visible)) / f64::from(steps - 1)) as f32;
            let delta_trait = e
                .call(TILE_TEXT_TO_TRAIT, &args![TRAIT_NAME_SCROLL_DELTA])
                .u32();
            set_float(e, container, delta_trait, delta, true);
        }
    }
    let scroll_bar = e.get(this, ListBoxLayout::pScrollBar);
    if !scroll_bar.is_null() {
        let items_trait = e
            .call(TILE_TEXT_TO_TRAIT, &args![TRAIT_NAME_NUMBER_OF_ITEMS])
            .u32();
        set_int(e, scroll_bar, items_trait, steps);
    }
}

// Translated from 007d74f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::SetEnabled` (Xbox PDB): sets the container
/// tile's `_enabled` trait to `enabled` (zero-extended byte). The trait id is
/// looked up once, in a function-local static.
// Translation incomplete: 007d74f0 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_option_set_enabled(e: &mut Engine, this: Ptr<ListBoxLayout>, enabled: u8) {
    set_enabled(e, this, OPTION_SET_ENABLED_TRAIT, enabled);
}

// Translated from 007d7580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::IsEnabled` (Xbox PDB): the container exists
/// and its `_enabled` trait is true.
// Translation incomplete: 007d7580 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_option_is_enabled(e: &mut Engine, this: Ptr<ListBoxLayout>) -> bool {
    is_enabled(e, this, OPTION_IS_ENABLED_TRAIT)
}

// Translated from 007d7630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container tile exists and `Tile::IsVisible` (`00a040a0`) holds for it.
pub fn fn_007d7630(e: &mut Engine, this: Ptr<ListBoxLayout>) -> bool {
    let container = e.get(this, ListBoxLayout::pContainerRect);
    !container.is_null() && e.call(TILE_IS_VISIBLE, &args![container]).bool()
}

// Translated from 007d7670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::Highlight` (Xbox PDB): see [`highlight`].
pub fn list_box_option_highlight(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: Ptr) -> bool {
    highlight(
        e,
        this,
        tile,
        list_box_option_is_enabled,
        list_box_option_update_highlight_box_pos,
    )
}

// Translated from 007d76e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::GetHighlightTarget` (Xbox PDB): see
/// [`highlight_target`].
pub fn list_box_option_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    direction: u32,
) -> u32 {
    highlight_target(e, this, direction, list_box_option_is_enabled)
}

// Translated from 007d7a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Selects the tile (`007d9000` with mode 0). Its twin `007d8d60` is named
/// `ListBox<BGSSaveLoadFileEntry*>::SetSelection` in the Xbox PDB.
pub fn fn_007d7a80(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: Ptr) -> bool {
    fn_007d9000(e, this, tile, 0)
}

// Translated from 007d7aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::IsSelected` (Xbox PDB): the tile's
/// `_selected` trait is true (false for a null tile). `this` is not used.
// Translation incomplete: 007d7aa0 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_option_is_selected(e: &mut Engine, _unused_this: Ptr, tile: Ptr) -> bool {
    let id = static_trait(e, OPTION_IS_SELECTED_TRAIT);
    id != 0 && !tile.is_null() && tile_is_true(e, tile, id)
}

// Translated from 007d7b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::DeselectAll` (Xbox PDB): deselects (`007d8fe0`)
/// the tile of every item; with `skip_filtered` nonzero the items the filter
/// dropped are left alone.
pub fn list_box_option_deselect_all(e: &mut Engine, this: Ptr<ListBoxLayout>, skip_filtered: u8) {
    deselect_all(e, this, skip_filtered, fn_007d8fe0);
}

// Translated from 007d7bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>::SetEnabled` (Xbox PDB): sets the container tile's
/// `_enabled` trait to `enabled` (zero-extended byte).
// Translation incomplete: 007d7bd0 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_int_set_enabled(e: &mut Engine, this: Ptr<ListBoxLayout>, enabled: u8) {
    set_enabled(e, this, DOWNLOADS_SET_ENABLED_TRAIT, enabled);
}

// Translated from 007d7c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `ListBox<BGSSaveLoadFileEntry*>`: the same steps as
/// [`list_box_option_ptr_construct`] with the file entry list's vtable.
/// Returns `this`.
// Translation incomplete: 007d7c60 - the exception-unwinding frame (the base destructors if a later step throws) is not translated.
pub fn list_box_file_entry_construct(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
) -> Ptr<ListBoxLayout> {
    construct_list_box(e, this, FILE_ENTRY_LIST_BOX_VTABLE)
}

// Translated from 007d7d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `ListBox<BGSSaveLoadFileEntry*>`: installs the
/// vtable, empties the list (`RemoveAll`, `007d81f0`), runs `007211c0` and the
/// embedded list's destructor (`0046ffb0` on `this + 4`).
// Translation incomplete: 007d7d30 - the exception-unwinding frame is not translated.
pub fn list_box_file_entry_destruct(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    e.mem.set_u32(this.addr(), FILE_ENTRY_LIST_BOX_VTABLE);
    list_box_file_entry_remove_all(e, this);
    e.call(LIST_BOX_FN_7211C0, &args![this]);
    e.call(EMBEDDED_LIST_DESTRUCT, &args![this.byte_add(4)]);
}

// Translated from 007d7da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::AddItem` (Xbox PDB): the same steps as
/// [`list_box_option_add_item`] with this list's `Synchronize` (`007d9590`) and
/// `SetTotalListHeight` (`007d9d50`).
// Translation incomplete: 007d7da0 - the exception-unwinding frame (freeing the new ListBoxItem if its constructor throws) is not translated.
pub fn list_box_file_entry_add_item(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    data: u32,
    text: u32,
    position: u32,
    template: u32,
) -> Ptr {
    add_item(
        e,
        this,
        (data, text, position, template),
        (
            FILE_ENTRY_LIST_SYNCHRONIZE,
            FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT,
        ),
    )
}

// Translated from 007d8040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the item whose tile is `tile` (`007d9ac0` on its node and the node
/// before it); false for a null tile or an empty list. When the tile is not in
/// the list the walk ends at the null node and `007d9ac0` is called with it.
pub fn fn_007d8040(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: u32) -> bool {
    if tile == 0 || e.call(LIST_IS_EMPTY_NODE, &args![this.byte_add(4)]).bool() {
        return false;
    }
    let mut node = first_node(this);
    let mut previous = first_node(this);
    while node != 0 {
        let item = node_item(e, node);
        if e.mem.u32(item) == tile {
            break;
        }
        previous = node;
        node = node_next(e, node);
    }
    fn_007d9ac0(e, this, node, previous)
}

// Translated from 007d80f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes an item (`007d9ac0` on its node and the node before it): with
/// `by_position` nonzero the item at position `key` of the list, else the one
/// whose index trait (`0xFAC`) equals `key`. False for an empty list; when
/// nothing matches the walk ends at the null node and `007d9ac0` is called
/// with it.
pub fn fn_007d80f0(e: &mut Engine, this: Ptr<ListBoxLayout>, key: u32, by_position: u8) -> bool {
    if e.call(LIST_IS_EMPTY_NODE, &args![this.byte_add(4)]).bool() {
        return false;
    }
    let mut node = first_node(this);
    let mut previous = first_node(this);
    if by_position != 0 {
        let mut position = 0u32;
        while node != 0 {
            let current = position;
            position = position.wrapping_add(1);
            if current == key {
                break;
            }
            previous = node;
            node = node_next(e, node);
        }
    } else {
        while node != 0 {
            let tile = node_tile(e, node);
            let value = tile_value(e, Ptr::new(tile), TRAIT_0FAC);
            if value == f64::from(key) {
                break;
            }
            previous = node;
            node = node_next(e, node);
        }
    }
    fn_007d9ac0(e, this, node, previous)
}

// Translated from 007d81f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::RemoveAll` (Xbox PDB): the same steps as
/// [`list_box_option_remove_all`] with this list's `SetTotalListHeight`
/// (`007d9d50`) and `UpdateHighlightBoxPos` (`007d97a0`).
pub fn list_box_file_entry_remove_all(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    remove_all(
        e,
        this,
        (
            FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT,
            FILE_ENTRY_LIST_UPDATE_HIGHLIGHT_BOX_POS,
        ),
    );
}

// Translated from 007d8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::Sort` (Xbox PDB): sorts the embedded list
/// with `argument` (`0083fd60` on `this + 4`) and synchronizes the tiles
/// (`007d9590`).
pub fn list_box_file_entry_sort(e: &mut Engine, this: Ptr<ListBoxLayout>, argument: u32) {
    e.call(LIST_SORT, &args![this.byte_add(4), argument]);
    list_box_file_entry_synchronize(e, this);
}

// Translated from 007d8450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::Filter` (Xbox PDB): runs `filter` (cdecl,
/// `(data) -> bool`) over every item's data. An item for which it returns
/// true is marked filtered out (its byte at +8 is 1, its index trait becomes
/// -1, the container's `_num_filtered` counts it); the others are numbered
/// again from 0 and positioned. The highlight moves on: a filtered-out
/// highlighted item is dropped and its old index remembered, and the item that
/// takes that index (else the one `007d8ee0` returns) is highlighted. Then the
/// container's `_num_filtered` is stored, the total height recomputed and the
/// highlight box updated.
///
/// With a null `filter` nothing is filtered out.
// Translation incomplete: 007d8450 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_file_entry_filter(e: &mut Engine, this: Ptr<ListBoxLayout>, filter: u32) {
    let num_filtered_trait = static_trait(e, FILE_ENTRY_FILTER_TRAIT);
    let mut filtered_count = 0i32;
    let mut remembered_index = -1i32;
    e.set(this, ListBoxLayout::usNextIndex, 0);
    e.set(this, ListBoxLayout::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 {
        if node_item(e, node) == 0 {
            break;
        }
        let filtered = filter != 0 && {
            let item = node_item(e, node);
            let data = e.mem.u32(item + LIST_ITEM_DATA);
            e.call(filter, &args![data]).bool()
        };
        let item = node_item(e, node);
        e.mem.set_u8(item + LIST_ITEM_FILTERED, u8::from(filtered));
        let item = node_item(e, node);
        if e.mem.u8(item + LIST_ITEM_FILTERED) == 0 {
            let position = e.get(this, ListBoxLayout::usNextIndex);
            if remembered_index >= 0 && i32::from(position) == remembered_index {
                let tile = node_tile(e, node);
                e.set(this, ListBoxLayout::pHighlightedItem, Ptr::new(tile));
            }
            let index = e.get(this, ListBoxLayout::usNextIndex);
            e.set(this, ListBoxLayout::usNextIndex, index.wrapping_add(1));
            let tile = node_tile(e, node);
            set_int(e, Ptr::new(tile), TRAIT_0FAC, i32::from(index));
            let tile = node_tile(e, node);
            e.call(LIST_INIT_TILE_POSITION, &args![this, tile]);
        } else {
            let tile = node_tile(e, node);
            if tile == e.get(this, ListBoxLayout::pHighlightedItem).addr() {
                let tile = node_tile(e, node);
                let value = tile_value(e, Ptr::new(tile), TRAIT_0FAC);
                remembered_index = float_to_int(e, value);
                e.set(this, ListBoxLayout::pHighlightedItem, Ptr::new(0));
            }
            let tile = node_tile(e, node);
            set_int(e, Ptr::new(tile), TRAIT_0FAC, -1);
            filtered_count += 1;
        }
        node = node_next(e, node);
    }
    let container = e.get(this, ListBoxLayout::pContainerRect);
    e.call(
        TILE_SET_INTEGER_VALUE,
        &args![container, num_filtered_trait, filtered_count],
    );
    list_box_file_entry_set_total_list_height(e, this);
    if e.get(this, ListBoxLayout::pHighlightedItem).is_null() && remembered_index >= 0 {
        let tile = list_tile_at(e, this, remembered_index);
        e.set(this, ListBoxLayout::pHighlightedItem, Ptr::new(tile));
    }
    if e.get(this, ListBoxLayout::pHighlightedItem).is_null() && remembered_index >= 0 {
        let tile = fn_007d8ee0(e, this);
        e.set(this, ListBoxLayout::pHighlightedItem, Ptr::new(tile));
    }
    list_box_file_entry_update_highlight_box_pos(e, this);
}

// Translated from 007d86c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The data pointer of the item `GetAt(index, flag)` (`007a6fa0`) returns, or
/// 0 when there is none.
pub fn fn_007d86c0(e: &mut Engine, this: Ptr<ListBoxLayout>, index: u32, flag: u8) -> u32 {
    let item = e
        .call(FILE_ENTRY_LIST_GET_AT, &args![this, index, u32::from(flag)])
        .u32();
    if item == 0 {
        0
    } else {
        e.mem.u32(item + LIST_ITEM_DATA)
    }
}

// Translated from 007d8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of items minus the container's `_num_filtered` value
/// (truncated). Same body as `007d9ee0`, which the Xbox PDB names
/// `ListBox<StartMenu::Option*>::NumUnfiltered`.
// Translation incomplete: 007d8700 - the exception-unwinding frame around the static initialization is not translated.
pub fn fn_007d8700(e: &mut Engine, this: Ptr<ListBoxLayout>) -> i32 {
    num_unfiltered(e, this, FILE_ENTRY_NUM_UNFILTERED_TRAIT)
}

// Translated from 007d87b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's `_number_of_visible_items` value truncated to an integer,
/// 0 without a container.
pub fn fn_007d87b0(e: &mut Engine, this: Ptr<ListBoxLayout>) -> i32 {
    let container = e.get(this, ListBoxLayout::pContainerRect);
    if container.is_null() {
        return 0;
    }
    let height_trait = user_trait(e, LIST_HEIGHT_TRAIT_NAME);
    fistp_truncate(tile_value(e, container, height_trait))
}

// Translated from 007d8810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::SetEnabled` (Xbox PDB): sets the
/// container tile's `_enabled` trait to `enabled` (zero-extended byte).
// Translation incomplete: 007d8810 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_file_entry_set_enabled(e: &mut Engine, this: Ptr<ListBoxLayout>, enabled: u8) {
    set_enabled(e, this, FILE_ENTRY_SET_ENABLED_TRAIT, enabled);
}

// Translated from 007d88a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::IsEnabled` (Xbox PDB): the container
/// exists and its `_enabled` trait is true.
// Translation incomplete: 007d88a0 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_file_entry_is_enabled(e: &mut Engine, this: Ptr<ListBoxLayout>) -> bool {
    is_enabled(e, this, FILE_ENTRY_IS_ENABLED_TRAIT)
}

// Translated from 007d8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::Highlight` (Xbox PDB): see [`highlight`].
pub fn list_box_file_entry_highlight(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: Ptr) -> bool {
    highlight(
        e,
        this,
        tile,
        list_box_file_entry_is_enabled,
        list_box_file_entry_update_highlight_box_pos,
    )
}

// Translated from 007d89c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::GetHighlightTarget` (Xbox PDB): see
/// [`highlight_target`].
pub fn list_box_file_entry_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    direction: u32,
) -> u32 {
    highlight_target(e, this, direction, list_box_file_entry_is_enabled)
}

// Translated from 007d8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::SetSelection` (Xbox PDB): selects the tile
/// (`007d9660` with mode 0).
pub fn list_box_file_entry_set_selection(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    tile: Ptr,
) -> bool {
    fn_007d9660(e, this, tile, 0)
}

// Translated from 007d8d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deselects the tile (`007d9660` with mode 1).
pub fn fn_007d8d80(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: Ptr) -> bool {
    fn_007d9660(e, this, tile, 1)
}

// Translated from 007d8da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::DeselectAll` (Xbox PDB): deselects
/// (`007d8d80`) the tile of every item; with `skip_filtered` nonzero the items
/// the filter dropped are left alone.
pub fn list_box_file_entry_deselect_all(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    skip_filtered: u8,
) {
    deselect_all(e, this, skip_filtered, fn_007d8d80);
}

// Translated from 007d8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties an array shaped like a `BSSimpleArray` (buffer +4, size +8,
/// reserved +0xC) of 12-byte elements: destroys the elements (`007d9e80`);
/// with `free_buffer` nonzero it also releases the buffer (`006a8500`) and
/// zeroes the buffer and reserved size. The size becomes 0. Nothing happens
/// without a buffer.
pub fn fn_007d8e20(e: &mut Engine, this: Ptr<BSSimpleArray>, free_buffer: u8) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    fn_007d9e80(e, this.cast(), buffer, size);
    if free_buffer != 0 {
        e.call(ARRAY_BUFFER_RELEASE, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 007d8e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `ListBox<StartMenu::Option*>` (it runs
/// the destructor body `007d6f80`): frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_007d8e80(e: &mut Engine, this: Ptr<ListBoxLayout>, flags: u32) -> Ptr<ListBoxLayout> {
    list_box_option_ptr_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007d8eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor body `007d7d30` and frees the object when bit 0 of
/// `flags` is set. Returns `this`.
pub fn list_box_file_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBoxLayout>,
    flags: u32,
) -> Ptr<ListBoxLayout> {
    list_box_file_entry_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007d8ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile the list's virtual `GetTileAt` (slot +0x14) returns for the index
/// of the last unfiltered item (`007d8700` minus one) with flag 0. (The Xbox
/// PDB name the map gives this address, `NumUnfiltered`, does not describe the
/// body.)
pub fn fn_007d8ee0(e: &mut Engine, this: Ptr<ListBoxLayout>) -> u32 {
    let count = fn_007d8700(e, this);
    list_tile_at(e, this, count.wrapping_sub(1))
}

// Translated from 007d8f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::Synchronize` (Xbox PDB): see [`synchronize`].
pub fn list_box_option_synchronize(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    synchronize(
        e,
        this,
        (
            list_box_option_set_total_list_height,
            list_box_option_update_highlight_box_pos,
        ),
    );
}

// Translated from 007d8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deselects the tile (`007d9000` with mode 1).
pub fn fn_007d8fe0(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: Ptr) -> bool {
    fn_007d9000(e, this, tile, 1)
}

// Translated from 007d9000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The selection function of the options list: see [`select`] (the list's
/// `IsEnabled` is `007d7580`).
// Translation incomplete: 007d9000 - the exception-unwinding frame around the static initialization is not translated.
pub fn fn_007d9000(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: Ptr, mode: u32) -> bool {
    select(
        e,
        this,
        tile,
        mode,
        (list_box_option_is_enabled, OPTION_SELECT_TRAIT),
    )
}

// Translated from 007d9140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::UpdateHighlightBoxPos` (Xbox PDB): see
/// [`update_highlight_box_pos`].
// Translation incomplete: 007d9140 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_option_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    update_highlight_box_pos(
        e,
        this,
        (OPTION_HIGHLIGHT_Y_TRAIT, OPTION_SELECTED_HEIGHT_TRAIT),
    );
}

// Translated from 007d9460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::SetTotalListHeight` (Xbox PDB): see
/// [`set_total_list_height`].
pub fn list_box_option_set_total_list_height(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    set_total_list_height(e, this, list_box_option_num_unfiltered);
}

// Translated from 007d9590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::Synchronize` (Xbox PDB): see
/// [`synchronize`].
pub fn list_box_file_entry_synchronize(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    synchronize(
        e,
        this,
        (
            list_box_file_entry_set_total_list_height,
            list_box_file_entry_update_highlight_box_pos,
        ),
    );
}

// Translated from 007d9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The selection function of the file entry list: see [`select`] (the list's
/// `IsEnabled` is `007d88a0`).
// Translation incomplete: 007d9660 - the exception-unwinding frame around the static initialization is not translated.
pub fn fn_007d9660(e: &mut Engine, this: Ptr<ListBoxLayout>, tile: Ptr, mode: u32) -> bool {
    select(
        e,
        this,
        tile,
        mode,
        (list_box_file_entry_is_enabled, FILE_ENTRY_SELECT_TRAIT),
    )
}

// Translated from 007d97a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::UpdateHighlightBoxPos` (Xbox PDB): see
/// [`update_highlight_box_pos`].
// Translation incomplete: 007d97a0 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_file_entry_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    update_highlight_box_pos(
        e,
        this,
        (
            FILE_ENTRY_HIGHLIGHT_Y_TRAIT,
            FILE_ENTRY_SELECTED_HEIGHT_TRAIT,
        ),
    );
}

// Translated from 007d9ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the item of `node` (`previous` is the node before it, 0 or the node
/// itself for the head) from the file entry list. If its tile is the
/// highlighted one, the highlight moves to the next node's tile, else the
/// previous node's (else nothing) and the interface manager focuses it
/// (`SetCurrentFocusTarget`, trait `0xFC3`). A filtered-out item lowers the
/// container's `_num_filtered` value by one. The item's tile is destroyed
/// through its vtable, its data deleted when the list owns it (flag 1), the
/// `ListBoxItem` freed and the node unlinked and deleted (the head's slot is
/// refilled from the second node, which is deleted instead). With auto update
/// (flag 0) the list synchronizes. False for a null node.
pub fn fn_007d9ac0(e: &mut Engine, this: Ptr<ListBoxLayout>, node: u32, previous: u32) -> bool {
    if node == 0 {
        return false;
    }
    let tile = node_tile(e, node);
    if tile == e.get(this, ListBoxLayout::pHighlightedItem).addr() {
        let replacement = if node_next(e, node) != 0 {
            let next = node_next(e, node);
            node_tile(e, next)
        } else if previous != 0 && previous != node {
            node_tile(e, previous)
        } else {
            0
        };
        e.set(this, ListBoxLayout::pHighlightedItem, Ptr::new(replacement));
        if replacement != 0 {
            let manager = e.call(INTERFACE_MANAGER, &[]).u32();
            e.call(
                SET_CURRENT_FOCUS_TARGET,
                &args![manager, replacement, TRAIT_0FC3, 1u32],
            );
        }
    }
    if !e.get(this, ListBoxLayout::pContainerRect).is_null() {
        let item = node_item(e, node);
        if e.mem.u8(item + LIST_ITEM_FILTERED) != 0 {
            let container = e.get(this, ListBoxLayout::pContainerRect);
            let filtered_trait = e
                .call(TILE_TEXT_TO_TRAIT, &args![LIST_RESET_TRAIT_NAME])
                .u32();
            let filtered = tile_value(e, container, filtered_trait);
            let one: f64 = e.global(ONE_AS_DOUBLE);
            let lowered = (filtered - one) as f32;
            let filtered_trait = e
                .call(TILE_TEXT_TO_TRAIT, &args![LIST_RESET_TRAIT_NAME])
                .u32();
            set_float(e, container, filtered_trait, lowered, true);
        }
    }
    let tile = node_tile(e, node);
    if tile != 0 {
        e.vcall(tile, 0x00, &args![1u32]);
    }
    if e.call(LIST_FLAG_TEST, &args![this, 1u32]).bool() {
        let item = node_item(e, node);
        let data = e.mem.u32(item + LIST_ITEM_DATA);
        if data != 0 {
            e.call(OPTION_SCALAR_DELETING_DESTRUCTOR, &args![data, 1u32]);
        }
    }
    let item = node_item(e, node);
    e.call(OPERATOR_DELETE, &args![item]);
    if node == first_node(this) {
        let second = e.get(this, ListBoxLayout::m_pkNext).addr();
        if second != 0 {
            let after_second = node_next(e, second);
            e.set(this, ListBoxLayout::m_pkNext, Ptr::new(after_second));
            let second_item = node_item(e, second);
            e.set(this, ListBoxLayout::m_item, Ptr::new(second_item));
            e.call(LIST_NODE_SET_NEXT, &args![second, 0u32]);
            e.call(LIST_NODE_SCALAR_DELETING_DESTRUCTOR, &args![second, 1u32]);
        } else {
            e.set(this, ListBoxLayout::m_item, Ptr::new(0));
        }
    } else {
        let following = node_next(e, node);
        e.call(LIST_NODE_SET_NEXT, &args![previous, following]);
        e.call(LIST_NODE_SET_NEXT, &args![node, 0u32]);
        e.call(LIST_NODE_SCALAR_DELETING_DESTRUCTOR, &args![node, 1u32]);
    }
    if e.call(LIST_FLAG_TEST, &args![this, 0u32]).bool() {
        list_box_file_entry_synchronize(e, this);
    }
    true
}

// Translated from 007d9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSSaveLoadFileEntry*>::SetTotalListHeight` (Xbox PDB): see
/// [`set_total_list_height`].
pub fn list_box_file_entry_set_total_list_height(e: &mut Engine, this: Ptr<ListBoxLayout>) {
    set_total_list_height(e, this, fn_007d8700);
}

// Translated from 007d9e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the 12-byte element destroyer `005aa3f0` (with flags 0) over `count`
/// elements starting at `elements`. `this` is not used.
pub fn fn_007d9e80(e: &mut Engine, _unused_this: Ptr, elements: u32, count: u32) {
    for index in 0..count {
        e.call(
            ARRAY_ELEMENT_DESTROY,
            &args![index.wrapping_mul(12).wrapping_add(elements), 0u32],
        );
    }
}

// Translated from 007d9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the string destructor (`004037d0`) on the object.
pub fn fn_007d9ec0(e: &mut Engine, this: Ptr) {
    e.call(STRING_DESTRUCT, &args![this]);
}

// Translated from 007d9ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StartMenu::Option*>::NumUnfiltered` (Xbox PDB): the number of
/// items minus the container's `_num_filtered` value (truncated).
// Translation incomplete: 007d9ee0 - the exception-unwinding frame around the static initialization is not translated.
pub fn list_box_option_num_unfiltered(e: &mut Engine, this: Ptr<ListBoxLayout>) -> i32 {
    num_unfiltered(e, this, OPTION_NUM_UNFILTERED_TRAIT)
}

// Translated from 007d9f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the array vtable `010771d8` (the one the destructor `007d9ff0`
/// stores) and zeroes the array (`006b3eb0(this, 0, 0)`). Returns `this`.
pub fn fn_007d9f90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), OPTION_ARRAY_VTABLE);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 007d9fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<StartMenu::Option_P_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor `007d9ff0` and frees the object when bit 0
/// of `flags` is set. Returns `this`.
pub fn bs_simple_array_option_ptr_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_007d9ff0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007d9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the array vtable `010771d8` and runs the array destructor body
/// (`008454f0(this, 1)`).
pub fn fn_007d9ff0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), OPTION_ARRAY_VTABLE);
    e.call(ARRAY_DESTRUCT_BODY, &args![this, 1u32]);
}

// Translated from 007da010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the array vtable `010771ec` (the one `007da070` stores) and zeroes
/// the array (`006b3eb0(this, 0, 0)`). Returns `this`.
pub fn fn_007da010(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), USER_OPTION_ARRAY_VTABLE);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 007da040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<StartMenu::UserOption_P_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor body `007da070` and frees the object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_user_option_ptr_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(USER_OPTION_ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007d3aa0, fn_007d3aa0(Ptr<StartMenu>)),
        entry!(0x007d3b60, start_menu_prep_save_load(Ptr<StartMenu>)),
        entry!(0x007d3c70, start_menu_finish_prep_save_load(Ptr<StartMenu>)),
        entry!(0x007d4040, fn_007d4040(u32) -> bool),
        entry!(0x007d40a0, fn_007d40a0(Ptr<StartMenu>)),
        entry!(
            0x007d42b0,
            start_menu_populate_options_list_box(Ptr<StartMenu>, u32)
        ),
        entry!(0x007d48a0, fn_007d48a0(Ptr<StartMenu>, u32, u8)),
        entry!(0x007d4950, fn_007d4950(Ptr<StartMenu>, u32, u32, u32, u8)),
        entry!(
            0x007d49a0,
            fn_007d49a0(Ptr<StartMenu>, u32, u32, u32, u32, bool)
        ),
        entry!(
            0x007d4b40,
            start_menu_populate_save_load_list_box(Ptr<StartMenu>)
        ),
        entry!(0x007d4bc0, fn_007d4bc0(Ptr<StartMenu>)),
        entry!(0x007d4c50, fn_007d4c50(i32, i32, u32, u32)),
        entry!(
            0x007d4ce0,
            start_menu_display_option(Ptr<StartMenu>, u32, u32)
        ),
        entry!(
            0x007d4df0,
            start_menu_update_highlighted_save_game_info(Ptr<StartMenu>)
        ),
        entry!(0x007d5120, fn_007d5120(Ptr)),
        entry!(0x007d5140, fn_007d5140(Ptr) -> u32),
        entry!(0x007d5160, fn_007d5160(Ptr) -> u32),
        entry!(0x007d5180, fn_007d5180(Ptr) -> u32),
        entry!(
            0x007d51a0,
            start_menu_set_save_game_screenshot(Ptr<StartMenu>, u32) -> bool
        ),
        entry!(0x007d5280, fn_007d5280(Ptr) -> u32),
        entry!(0x007d52a0, fn_007d52a0(Ptr<StartMenu>) -> bool),
        entry!(0x007d5830, fn_007d5830(Ptr<StartMenu>) -> bool),
        entry!(
            0x007d5af0,
            start_menu_save_load_idle(Ptr<StartMenu>) -> bool
        ),
        entry!(0x007d5eb0, fn_007d5eb0(Ptr) -> u32),
        entry!(0x007d5f10, fn_007d5f10(Ptr) -> bool),
        entry!(0x007d5f30, fn_007d5f30(Ptr) -> bool),
        entry!(0x007d5f50, start_menu_failed_save_callback_function()),
        entry!(
            0x007d5f80,
            start_menu_hide_options_container(Ptr<StartMenu>)
        ),
        entry!(0x007d6030, fn_007d6030(Ptr<StartMenu>) -> bool),
        entry!(0x007d60b0, fn_007d60b0(Ptr, i32) -> u32),
        entry!(0x007d6100, fn_007d6100(Ptr) -> u32),
        entry!(0x007d6120, fn_007d6120(Ptr) -> u32),
        entry!(0x007d6140, fn_007d6140(Ptr) -> u32),
        entry!(0x007d6160, fn_007d6160(Ptr<StartMenu>)),
        entry!(0x007d61b0, fn_007d61b0(Ptr) -> Ptr),
        entry!(
            0x007d61e0,
            start_menu_fade(Ptr<StartMenu>, u32, bool, u32, f32)
        ),
        entry!(0x007d62e0, fn_007d62e0(Ptr<StartMenu>)),
        (
            0x007d6350,
            start_menu_user_option_user_option_entry as AbiFn,
        ),
        entry!(
            0x007d64a0,
            start_menu_user_option_scalar_deleting_destructor(
                Ptr<StartMenuUserOption>,
                u32,
            )
                -> Ptr<StartMenuUserOption>
        ),
        entry!(0x007d64d0, fn_007d64d0(Ptr<StartMenuUserOption>)),
        entry!(
            0x007d6510,
            start_menu_option_list_box_get_highlight_target(Ptr<ListBoxLayout>, u32) -> u32
        ),
        entry!(0x007d6550, fn_007d6550(Ptr<StartMenu>)),
        entry!(0x007d6720, fn_007d6720(Ptr) -> u32),
        entry!(0x007d6730, fn_007d6730(Ptr<StartMenu>)),
        entry!(
            0x007d67b0,
            start_menu_initialize_save_load_list_item(Ptr, Ptr)
        ),
        entry!(0x007d6950, fn_007d6950(Ptr) -> u32),
        entry!(0x007d6970, fn_007d6970(Ptr) -> u32),
        entry!(0x007d6990, fn_007d6990(u32)),
        entry!(0x007d69d0, fn_007d69d0()),
        entry!(0x007d69f0, fn_007d69f0()),
        entry!(0x007d6bb0, fn_007d6bb0(Ptr) -> Ptr),
        entry!(0x007d6bd0, fn_007d6bd0(Ptr, bool) -> bool),
        entry!(0x007d6c30, fn_007d6c30()),
        entry!(0x007d6c90, start_menu_end_credits(Ptr<StartMenu>)),
        entry!(0x007d6d70, fn_007d6d70()),
        entry!(
            0x007d6eb0,
            list_box_option_ptr_construct(Ptr<ListBoxLayout>) -> Ptr<ListBoxLayout>
        ),
        entry!(0x007d6f80, list_box_option_ptr_destruct(Ptr<ListBoxLayout>)),
        entry!(
            0x007d6ff0,
            list_box_option_add_item(Ptr<ListBoxLayout>, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x007d7290, list_box_option_remove_all(Ptr<ListBoxLayout>)),
        entry!(0x007d74c0, list_box_option_sort(Ptr<ListBoxLayout>, u32)),
        entry!(
            0x007d74f0,
            list_box_option_set_enabled(Ptr<ListBoxLayout>, u8)
        ),
        entry!(
            0x007d7580,
            list_box_option_is_enabled(Ptr<ListBoxLayout>) -> bool
        ),
        entry!(0x007d7630, fn_007d7630(Ptr<ListBoxLayout>) -> bool),
        entry!(
            0x007d7670,
            list_box_option_highlight(Ptr<ListBoxLayout>, Ptr) -> bool
        ),
        entry!(
            0x007d76e0,
            list_box_option_get_highlight_target(Ptr<ListBoxLayout>, u32) -> u32
        ),
        entry!(0x007d7a80, fn_007d7a80(Ptr<ListBoxLayout>, Ptr) -> bool),
        entry!(0x007d7aa0, list_box_option_is_selected(Ptr, Ptr) -> bool),
        entry!(
            0x007d7b50,
            list_box_option_deselect_all(Ptr<ListBoxLayout>, u8)
        ),
        entry!(0x007d7bd0, list_box_int_set_enabled(Ptr<ListBoxLayout>, u8)),
        entry!(
            0x007d7c60,
            list_box_file_entry_construct(Ptr<ListBoxLayout>) -> Ptr<ListBoxLayout>
        ),
        entry!(0x007d7d30, list_box_file_entry_destruct(Ptr<ListBoxLayout>)),
        entry!(
            0x007d7da0,
            list_box_file_entry_add_item(Ptr<ListBoxLayout>, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x007d8040, fn_007d8040(Ptr<ListBoxLayout>, u32) -> bool),
        entry!(0x007d80f0, fn_007d80f0(Ptr<ListBoxLayout>, u32, u8) -> bool),
        entry!(
            0x007d81f0,
            list_box_file_entry_remove_all(Ptr<ListBoxLayout>)
        ),
        entry!(
            0x007d8420,
            list_box_file_entry_sort(Ptr<ListBoxLayout>, u32)
        ),
        entry!(
            0x007d8450,
            list_box_file_entry_filter(Ptr<ListBoxLayout>, u32)
        ),
        entry!(0x007d86c0, fn_007d86c0(Ptr<ListBoxLayout>, u32, u8) -> u32),
        entry!(0x007d8700, fn_007d8700(Ptr<ListBoxLayout>) -> i32),
        entry!(0x007d87b0, fn_007d87b0(Ptr<ListBoxLayout>) -> i32),
        entry!(
            0x007d8810,
            list_box_file_entry_set_enabled(Ptr<ListBoxLayout>, u8)
        ),
        entry!(
            0x007d88a0,
            list_box_file_entry_is_enabled(Ptr<ListBoxLayout>) -> bool
        ),
        entry!(
            0x007d8950,
            list_box_file_entry_highlight(Ptr<ListBoxLayout>, Ptr) -> bool
        ),
        entry!(
            0x007d89c0,
            list_box_file_entry_get_highlight_target(Ptr<ListBoxLayout>, u32) -> u32
        ),
        entry!(
            0x007d8d60,
            list_box_file_entry_set_selection(Ptr<ListBoxLayout>, Ptr) -> bool
        ),
        entry!(0x007d8d80, fn_007d8d80(Ptr<ListBoxLayout>, Ptr) -> bool),
        entry!(
            0x007d8da0,
            list_box_file_entry_deselect_all(Ptr<ListBoxLayout>, u8)
        ),
        entry!(0x007d8e20, fn_007d8e20(Ptr<BSSimpleArray>, u8)),
        entry!(
            0x007d8e80,
            fn_007d8e80(Ptr<ListBoxLayout>, u32) -> Ptr<ListBoxLayout>
        ),
        entry!(
            0x007d8eb0,
            list_box_file_entry_scalar_deleting_destructor(
                Ptr<ListBoxLayout>,
                u32,
            ) -> Ptr<ListBoxLayout>
        ),
        entry!(0x007d8ee0, fn_007d8ee0(Ptr<ListBoxLayout>) -> u32),
        entry!(0x007d8f10, list_box_option_synchronize(Ptr<ListBoxLayout>)),
        entry!(0x007d8fe0, fn_007d8fe0(Ptr<ListBoxLayout>, Ptr) -> bool),
        entry!(
            0x007d9000,
            fn_007d9000(Ptr<ListBoxLayout>, Ptr, u32) -> bool
        ),
        entry!(
            0x007d9140,
            list_box_option_update_highlight_box_pos(Ptr<ListBoxLayout>)
        ),
        entry!(
            0x007d9460,
            list_box_option_set_total_list_height(Ptr<ListBoxLayout>)
        ),
        entry!(
            0x007d9590,
            list_box_file_entry_synchronize(Ptr<ListBoxLayout>)
        ),
        entry!(
            0x007d9660,
            fn_007d9660(Ptr<ListBoxLayout>, Ptr, u32) -> bool
        ),
        entry!(
            0x007d97a0,
            list_box_file_entry_update_highlight_box_pos(Ptr<ListBoxLayout>)
        ),
        entry!(
            0x007d9ac0,
            fn_007d9ac0(Ptr<ListBoxLayout>, u32, u32) -> bool
        ),
        entry!(
            0x007d9d50,
            list_box_file_entry_set_total_list_height(Ptr<ListBoxLayout>)
        ),
        entry!(0x007d9e80, fn_007d9e80(Ptr, u32, u32)),
        entry!(0x007d9ec0, fn_007d9ec0(Ptr)),
        entry!(
            0x007d9ee0,
            list_box_option_num_unfiltered(Ptr<ListBoxLayout>) -> i32
        ),
        entry!(0x007d9f90, fn_007d9f90(Ptr) -> Ptr),
        entry!(
            0x007d9fc0,
            bs_simple_array_option_ptr_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x007d9ff0, fn_007d9ff0(Ptr)),
        entry!(0x007da010, fn_007da010(Ptr) -> Ptr),
        entry!(
            0x007da040,
            bs_simple_array_user_option_ptr_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Every function this file calls by address (and the function
    /// pointers it hands to others): tests replace them with doubles that
    /// return zero, then give the ones a case needs their own behaviour.
    const CALLEES: &[u32] = &[
        ABSOLUTE_VALUE,
        ARRAY_BUFFER_RELEASE,
        ARRAY_DESTRUCT_BODY,
        ARRAY_ELEMENT_DESTROY,
        ARRAY_INIT,
        FILE_ENTRY_LIST_GET_AT,
        FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT,
        FILE_ENTRY_LIST_SYNCHRONIZE,
        FILE_ENTRY_LIST_UPDATE_HIGHLIGHT_BOX_POS,
        LIST_CONTAINS_TILE,
        ROUND_FLOAT,
        ROUND_UP_TO_MULTIPLE,
        TILE_FIND_BY_NAME,
        TILE_GET_CHILD_BY_NAME,
        TILE_IS_VISIBLE,
        TILE_LOAD_BYTE_35,
        TILE_STORE_BYTE_35,
        USER_OPTION_ARRAY_DESTRUCT,
        ACTION_MAPPING_ITEM_FN,
        ARRAY_COUNT,
        ARRAY_ELEMENT,
        BINDING_CANCEL,
        BIND_INPUT,
        BRIGHTNESS_CHANGE_FN,
        CLEAR_BINDING,
        CLOSE_MESSAGE_MENU,
        CONTROLLER_OPTION_CHANGE_FN_A,
        CONTROLLER_OPTION_CHANGE_FN_B,
        CONTROLLER_STATE_IMPORT,
        CONTROLS_FN_A22E10,
        CONTROLS_GET,
        DATA_HANDLER_COUNT,
        DATA_HANDLER_FN_4614E0,
        DATA_HANDLER_FN_463070,
        DATA_HANDLER_FN_465040,
        DATA_HANDLER_IS_LOADED,
        DLC_ENUMERATION_STATE,
        DLC_FINISH,
        DLC_UTILITY_FN_AF2940,
        DLC_UTILITY_FN_AF2B10,
        DLC_UTILITY_FROM_SYSTEM,
        DOWNLOADS_LIST_ADD_ITEM,
        DOWNLOADS_LIST_SET_ENABLED,
        EMBEDDED_LIST_CONSTRUCT,
        EMBEDDED_LIST_DESTRUCT,
        FAILED_SAVE_CALLBACK,
        FILE_ENTRY_GET_SCREENSHOT,
        FILE_ENTRY_IS_AUTOSAVE,
        FILE_ENTRY_IS_CORRUPT,
        FILE_ENTRY_LANGUAGE,
        FILE_ENTRY_LOAD_DATA,
        FILE_ENTRY_NAME_TEST_AD0,
        FILE_ENTRY_NAME_TEST_B10,
        FILE_ENTRY_PLAYER_NAME,
        FILE_ENTRY_SAVE_GAME,
        FIRST_BYTE,
        FIRST_WORD,
        FLAG_SETTING_BYTE,
        FLOAT_MAX,
        FN_0078D2D0,
        FN_007D01B0,
        FORM_FN_570F40,
        FTOL,
        GAMEPAD_AXIS_COUNT,
        GAMEPAD_AXIS_SOURCE,
        GAMEPAD_AXIS_VALUE,
        GAMEPAD_BUTTON_COUNT,
        GAMEPAD_BUTTON_STATE,
        GAMEPAD_OPTION_CHANGE_FN,
        IDLE_MANAGER_FN_6003C0,
        IDLE_TIMER_GET,
        IDLE_TIMER_RESET,
        IMAGE_ALPHA_MAP,
        IMAGE_SET_ALPHA_MAP,
        INTERFACE_ACTIVE,
        INTERFACE_MANAGER,
        INTERFACE_UNIT_SIZE,
        IS_CONSOLE_VISIBLE,
        KEY_STATE,
        LIST_ADD_TAIL,
        LIST_BOX_BASE_CONSTRUCT,
        LIST_BOX_FN_721130,
        LIST_BOX_FN_7211C0,
        LIST_BOX_FN_7A1950,
        LIST_CONTAINER_VALID,
        LIST_COUNT,
        LIST_DATA_AT,
        LIST_FIRST_NODE,
        LIST_FIRST_TILE,
        LIST_FLAG_TEST,
        LIST_FOR_RANGE,
        LIST_HEAD_ITEM_SLOT,
        LIST_INIT_TILE_POSITION,
        LIST_INSERT,
        LIST_IS_EMPTY,
        LIST_IS_EMPTY_NODE,
        LIST_ITEM_CONSTRUCT,
        LIST_ITEM_COUNT,
        LIST_NEXT_NODE,
        LIST_NODE_ITEM_SLOT,
        LIST_NODE_SCALAR_DELETING_DESTRUCTOR,
        LIST_NODE_SELF,
        LIST_NODE_SET_NEXT,
        LIST_REMOVE_AT,
        LIST_SCROLL_BAR,
        LIST_SELECTED_DATA,
        LIST_SORT,
        LOADING_MENU_FN_078AA90,
        LOADING_MENU_RESUME_BACKGROUND_THREAD,
        LOADING_MENU_SUSPEND_BACKGROUND_THREAD,
        MAX_INT,
        MEDIA_FN_761460,
        MEDIA_FN_8304C0,
        MEDIA_IS_PLAYING,
        MEDIA_PLAY,
        MEMSET,
        MENU_GET_FLAG,
        MENU_RENDER_TEMPLATE,
        MENU_SET_FLAG,
        MESSAGE_MENU_CLOSE_007AA480,
        MIN_INT,
        MOUSE_BUTTON_COUNT,
        MOUSE_BUTTON_STATE,
        NETWORK_SYSTEM_UTILITY_INSTANCE,
        OBJECT_FLAG_BYTE_TEST,
        OBJECT_PLUS_4,
        OBJECT_TEST_005E01B0,
        OPERATOR_DELETE,
        OPERATOR_NEW,
        OPTION_CONSTRUCT,
        OPTION_DESTRUCT,
        OPTION_LIST_DESELECT_ALL,
        OPTION_LIST_GET_HIGHLIGHT_TARGET,
        OPTION_LIST_GET_TILE_FOR,
        OPTION_LIST_IS_ENABLED,
        OPTION_LIST_REFRESH_FN,
        OPTION_LIST_SET_ENABLED,
        OPTION_LIST_SET_TOTAL_LIST_HEIGHT,
        OPTION_LIST_SYNCHRONIZE,
        OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS,
        OPTION_MATCHES_MODES,
        OPTION_SCALAR_DELETING_DESTRUCTOR,
        PLAY_MENU_SOUND,
        PROCESS_LISTS_FN_970D50,
        SAVE_LOAD_FILTER_FN,
        SAVE_LOAD_ITEM_INIT_FN,
        SAVE_LOAD_LIST_ADD_ITEM,
        SAVE_LOAD_LIST_DESELECT_ALL,
        SAVE_LOAD_LIST_FILTER,
        SAVE_LOAD_LIST_IS_ENABLED,
        SAVE_LOAD_LIST_SELECT_HIGHLIGHT,
        SAVE_LOAD_LIST_SET_ENABLED,
        SAVE_LOAD_LIST_SET_SELECTION,
        SAVE_LOAD_MANAGER_REFRESH,
        SAVE_LOAD_MANAGER_SAVE_GAME,
        SEED_CELL,
        SETTING_STRING,
        SETTING_STRING_ALT,
        SET_CURRENT_FOCUS_TARGET,
        SHOW_MESSAGE,
        SINGLETON_0045D180,
        SINGLETON_004DE490,
        SPRINTF_S,
        START_MENU_CHOOSE_BACK,
        START_MENU_CLOSE,
        START_MENU_FN_7D30B0,
        STRING_CHARACTERS,
        STRING_COMPARE,
        STRING_CONSTRUCT,
        STRING_DESTRUCT,
        STRING_FORMAT,
        SYSTEM_UTILITY_INSTANCE,
        TES_GLOBAL_GET_VALUE,
        TES_GLOBAL_SET_VALUE,
        TICK_COUNT,
        TILE_ADD_FADE_CONTROL,
        TILE_ADD_NEEDS_UPDATE,
        TILE_ADD_USER_TRAIT,
        TILE_CHAIN_SUM_FA1,
        TILE_CHAIN_SUM_FA2,
        TILE_GET_CHILD_BY_ID,
        TILE_GET_FADE_FINISHED,
        TILE_GET_FLOAT,
        TILE_GET_MENU,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_VALUE,
        TILE_GET_VALUE_Q,
        TILE_HAS_FADE_CONTROL,
        TILE_IMAGE_SET_SOURCE_TEXTURE,
        TILE_IS_TRUE,
        TILE_NEXT_SIBLING,
        TILE_POKE,
        TILE_SET_FLOAT,
        TILE_SET_INT,
        TILE_SET_INTEGER_VALUE,
        TILE_SET_STRING,
        TILE_SET_TEXT_SOURCE,
        TILE_TEXT_TO_TRAIT,
        TILE_UPDATE_TILE,
        TILE_VALUE_ADD_ACTION,
        TILE_VALUE_ADD_ACTION_FLOAT,
        TILE_VALUE_CALCULATE,
        TILE_VALUE_CLEAR_ACTIONS,
        TILE_VALUE_SET_FLOAT,
        UPDATE_CAN_LOAD_STATUS,
        WAITING_FOR_REMAP_PRESS,
        WCSTOMBS,
        WORD_AT_4,
        XUI_IS_UP,
    ];

    const TILE_BASE: u32 = 0x7000_0000;
    /// Address of the double standing for the virtual function at byte offset
    /// `slot` of a list box's vtable (see [`list_vtable`]).
    const SLOT_BASE: u32 = 0x6000_0100;

    fn tile(index: u32) -> u32 {
        TILE_BASE + 0x100 * index
    }

    fn slot_fn(slot: u32) -> u32 {
        SLOT_BASE + slot
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn float_ret(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    /// An engine with the data pages this file reads mapped, the game's
    /// constants set to distinct values, every callee replaced by a double
    /// that returns zero, working menu flag accessors, a truncating
    /// `_ftol2_sse`, and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x00fd_f000, 0x1000);
        e.map(0x0100_0000, 0x0006_0000);
        e.map(0x011a_0000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011c_b000, 0x1000);
        e.map(0x011d_0000, 0x0012_0000);
        for &addr in CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MENU_GET_FLAG, |e, a| {
            ret((e.mem.u32(a[0] + 0x1a8) & a[1] != 0) as u32)
        });
        e.register(MENU_SET_FLAG, |e, a| {
            let flags = e.mem.u32(a[0] + 0x1a8);
            let flags = if a[2] & 0xff != 0 {
                flags | a[1]
            } else {
                flags & !a[1]
            };
            e.mem.set_u32(a[0] + 0x1a8, flags);
            Ret::default()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ret(value as i32 as u32)
        });
        // The setting text is its address plus 4, so a test can tell which
        // setting a text came from.
        e.register(SETTING_STRING, |_, a| ret(a[0] + 4));
        e.register(SETTING_STRING_ALT, |_, a| ret(a[0] + 4));
        e.set_global(COMPARE_ZERO, 0.0f64);
        e.set_global(COMPARE_ONE, 1.0f64);
        e.set_global(FADE_TARGET_VALUE, 3.5f32);
        e.set_global(FADE_DEFAULT_VALUE, 2.5f32);
        e.set_global(FADE_CONTROL_RATE, 1.5f32);
        e.set_global(FADE_CONFIRM_VALUE, 4.5f32);
        e.call_log = Some(Vec::new());
        e
    }

    fn new_menu(e: &mut Engine) -> Ptr<StartMenu> {
        let menu = e.new_object::<StartMenu>();
        for index in 0..23 {
            e.mem.set_u32(menu.addr() + TILES + 4 * index, tile(index));
        }
        e.set(menu, StartMenu::ENABLED_TRAIT, 0x2000);
        e.set_global(START_MENU_INSTANCE, menu.addr());
        menu
    }

    fn flags(e: &Engine, menu: Ptr<StartMenu>) -> u32 {
        e.get(menu, StartMenu::uiFlags)
    }

    fn set_flags(e: &mut Engine, menu: Ptr<StartMenu>, mask: u32) {
        let value = flags(e, menu) | mask;
        e.set(menu, StartMenu::uiFlags, value);
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn stub_float(e: &mut Engine, addr: u32, st0: f64) {
        e.register_double(addr, move |_, _| float_ret(st0));
    }

    /// The local the next `with_stack(size)` on the running thread takes (the called function's
    /// own frame, taken at the stack's current top).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The addresses of the logged calls, in order, among `wanted`.
    fn order(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| wanted.contains(a))
            .collect()
    }

    /// Makes `list` a list box with a vtable whose slot `n` is the double
    /// at [`slot_fn`]`(n)`.
    fn list_vtable(e: &mut Engine, list: Ptr<ListBoxLayout>) {
        let slots: Vec<u32> = (0..9).map(|i| slot_fn(4 * i)).collect();
        e.put_vtable(0x6000_0000, &slots);
        e.mem.set_u32(list.addr(), 0x6000_0000);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
    }

    fn list_of(menu: Ptr<StartMenu>, offset: u32) -> u32 {
        menu.addr() + offset
    }

    /// A `BSSimpleArray` whose elements are `items`: `ARRAY_COUNT` and
    /// `ARRAY_ELEMENT` answer for the array at `array`.
    fn install_array(e: &mut Engine, array: u32, items: &[u32]) {
        let slots = e.mem.alloc(4 * items.len().max(1) as u32);
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(slots + 4 * index as u32, *item);
        }
        let count = items.len() as u32;
        e.register_double(ARRAY_COUNT, move |_, a| {
            assert_eq!(a[0], array, "count of an unexpected array");
            ret(count)
        });
        e.register_double(ARRAY_ELEMENT, move |_, a| {
            assert_eq!(a[0], array, "element of an unexpected array");
            ret(slots + 4 * a[1])
        });
    }

    #[test]
    fn registrations_cover_the_first_sixty_functions() {
        let first: Vec<u32> = funcs()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a <= 0x007d_74c0)
            .collect();
        assert_eq!(first.len(), 60);
        let mut addresses = first.clone();
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 60);
        assert!(addresses.iter().all(|a| *a >= 0x007d_3aa0));
    }

    #[test]
    fn change_functions_run_for_every_user_option() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let first = e.mem.alloc(0x30);
        let second = e.mem.alloc(0x30);
        e.mem.set_u32(first + 0x2c, BRIGHTNESS_CHANGE_FN);
        install_array(&mut e, USER_OPTIONS_ARRAY, &[first, second]);
        fn_007d3aa0(&mut e, menu);
        assert_eq!(calls(&e, BRIGHTNESS_CHANGE_FN), vec![vec![first]]);
        assert_eq!(e.get(menu, StartMenu::pBrightnessOption).addr(), first);
        let menu_addr = menu.addr();
        assert_eq!(
            calls(&e, MENU_SET_FLAG),
            vec![vec![menu_addr, 0x20, 0], vec![menu_addr, 0x20, 1]]
        );
        assert_eq!(flags(&e, menu), 0x20);
    }

    #[test]
    fn prep_save_load_adds_the_new_save_item_to_an_empty_list() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, LIST_IS_EMPTY, 1);
        stub(&mut e, SAVE_LOAD_LIST_ADD_ITEM, 0x5000);
        start_menu_prep_save_load(&mut e, menu);
        let list = list_of(menu, 0x174);
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_ADD_ITEM),
            vec![vec![list, 0, 0, 0, NEW_SAVE_TEMPLATE]]
        );
        assert!(calls(&e, TILE_SET_STRING).contains(&vec![
            0x5000,
            0x0fc4,
            SETTING_011D39C0 + 4,
            1
        ]));
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x5000, 0x2000, 0]));
        assert_eq!(calls(&e, SAVE_LOAD_LIST_SET_ENABLED), vec![vec![list, 1]]);
        assert_eq!(flags(&e, menu), FLAG_LOADING_SAVEGAMES);
        // the container (6) and the back button (5) fade in
        let fades = calls(&e, TILE_ADD_FADE_CONTROL);
        assert_eq!(fades.len(), 2);
        assert_eq!(fades[0][0], tile(6));
        assert_eq!(fades[1][0], tile(5));
        assert!(calls(&e, SAVE_LOAD_LIST_FILTER).is_empty());
    }

    #[test]
    fn prep_save_load_filters_a_list_that_has_entries() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, LIST_IS_EMPTY, 0);
        start_menu_prep_save_load(&mut e, menu);
        let list = list_of(menu, 0x174);
        // first the unfiltered pass, then FinishPrepSaveLoad's filter
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_FILTER),
            vec![vec![list, 0], vec![list, SAVE_LOAD_FILTER_FN]]
        );
        assert!(calls(&e, SAVE_LOAD_LIST_ADD_ITEM).is_empty());
    }

    #[test]
    fn finish_prep_save_load_in_save_mode_drops_special_saves() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_SAVE_MODE);
        let list = list_at(menu, LIST_SAVE_LOAD);
        list_vtable(&mut e, list);
        stub(&mut e, LIST_COUNT, 3);
        stub(&mut e, slot_fn(0x14), 0x5000);
        stub(&mut e, LIST_FIRST_TILE, 0x5100);
        // entries 0xe0 + index: 0 is an autosave, 2 the other special kind
        e.register(LIST_DATA_AT, |_, a| ret(0xe0 + a[1]));
        e.register(FILE_ENTRY_IS_AUTOSAVE, |_, a| ret((a[0] == 0xe0) as u32));
        e.register(FILE_ENTRY_NAME_TEST_B10, |_, a| ret((a[0] == 0xe2) as u32));
        start_menu_finish_prep_save_load(&mut e, menu);
        let list = list.addr();
        assert_eq!(
            calls(&e, LIST_REMOVE_AT),
            vec![vec![list, 2, 0], vec![list, 0, 0]]
        );
        // the first tile is enabled, the title shows one entry
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x5100, 0x2000, 1]));
        let counts = calls(&e, SPRINTF_S);
        assert_eq!(counts.len(), 1);
        assert_eq!(counts[0][1..3], [0x40, FORMAT_SAVE_COUNT]);
        assert_eq!(counts[0][3], SETTING_011D2C4C + 4);
        assert_eq!(counts[0][4..6], [0, 1000]);
        assert_eq!(counts[0][6], SETTING_011D25D4 + 4);
        assert!(calls(&e, TILE_SET_STRING)
            .iter()
            .any(|c| c[..2] == [0x5100, 0x0fc4] && c[2] == counts[0][0]));
        assert_eq!(calls(&e, SAVE_LOAD_LIST_SET_ENABLED), vec![vec![list, 1]]);
        assert_eq!(calls(&e, SAVE_LOAD_LIST_DESELECT_ALL), vec![vec![list, 1]]);
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_FILTER),
            vec![vec![list, SAVE_LOAD_FILTER_FN]]
        );
        // no loading in progress: the container and the back button fade in
        assert_eq!(calls(&e, TILE_ADD_FADE_CONTROL).len(), 2);
    }

    #[test]
    fn finish_prep_save_load_with_a_thousand_saves_scrolls_to_the_second() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_SAVE_MODE | FLAG_LOADING_SAVEGAMES);
        let list = list_at(menu, LIST_SAVE_LOAD);
        list_vtable(&mut e, list);
        stub(&mut e, LIST_COUNT, 1000);
        stub(&mut e, LIST_FIRST_TILE, 0x5100);
        stub(&mut e, slot_fn(0x14), 0x5000);
        start_menu_finish_prep_save_load(&mut e, menu);
        assert_eq!(calls(&e, TILE_ADD_NEEDS_UPDATE), vec![vec![0x5100, 8]]);
        // Highlight(second tile) then ScrollToHighlight
        assert_eq!(calls(&e, slot_fn(0x00)), vec![vec![list.addr(), 0x5000]]);
        assert_eq!(calls(&e, slot_fn(0x10)), vec![vec![list.addr()]]);
        // the list is still being loaded: the back button is marked instead
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(5), 0x0faf, 1]));
        assert!(calls(&e, TILE_ADD_FADE_CONTROL).is_empty());
    }

    #[test]
    fn finish_prep_save_load_outside_save_mode_disables_the_only_tile() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let list = list_at(menu, LIST_SAVE_LOAD);
        list_vtable(&mut e, list);
        stub(&mut e, LIST_FIRST_TILE, 0x5100);
        stub(&mut e, INTERFACE_ACTIVE, 1);
        stub(&mut e, INTERFACE_MANAGER, 0x5200);
        stub(&mut e, LIST_SCROLL_BAR, 0x5300);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        stub(&mut e, LIST_ITEM_COUNT, 9);
        start_menu_finish_prep_save_load(&mut e, menu);
        assert!(calls(&e, TILE_SET_STRING).contains(&vec![
            0x5100,
            0x0fc4,
            SETTING_011D39F0 + 4,
            1
        ]));
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x5100, 0x2000, 0]));
        // the interface is active: focus goes to the first tile, the scroll
        // bar's value is reset and its "set in code" trait poked
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x5200, 0x5100, 0x0fc3, 1]]
        );
        assert_eq!(calls(&e, TILE_SET_FLOAT), vec![vec![0x5300, 0xabc, 0, 0]]);
        assert_eq!(
            calls(&e, TILE_POKE),
            vec![vec![0x5300, 0xabc, 1.0f32.to_bits()]]
        );
        // the items are initialised over [0, count)
        assert_eq!(
            calls(&e, LIST_FOR_RANGE),
            vec![vec![list.addr(), SAVE_LOAD_ITEM_INIT_FN, 0, 9]]
        );
        assert_eq!(e.get(menu, StartMenu::fSaveLoadThreshold), 0.0);
    }

    #[test]
    fn the_save_load_filter_hides_the_list_unless_it_has_several_entries() {
        let mut e = engine();
        // no start menu yet
        assert!(fn_007d4040(&mut e, 0));
        let menu = new_menu(&mut e);
        stub(&mut e, LIST_COUNT, 2);
        assert!(fn_007d4040(&mut e, 0));
        assert!(!fn_007d4040(&mut e, 5));
        stub(&mut e, LIST_COUNT, 1);
        assert!(!fn_007d4040(&mut e, 0));
        stub(&mut e, LIST_COUNT, 2);
        set_flags(&mut e, menu, FLAG_SAVE_MODE);
        assert!(!fn_007d4040(&mut e, 0));
    }

    #[test]
    fn opening_the_confirm_dialog_places_it_from_the_highlighted_tile() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_DELETE_MODE);
        let list = list_at(menu, LIST_SAVE_LOAD);
        list_vtable(&mut e, list);
        list_vtable(&mut e, list_at(menu, LIST_CONFIRM));
        stub(&mut e, slot_fn(0x04), 0x5200);
        stub(&mut e, LIST_SCROLL_BAR, 0x5300);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        stub_float(&mut e, TILE_CHAIN_SUM_FA2, 10.0);
        stub_float(&mut e, TILE_CHAIN_SUM_FA1, 100.0);
        let (container, question) = (tile(6), tile(8));
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            float_ret(match (a[0], a[1]) {
                (0x5200, 0x0fb0) => 20.0,
                (t, 0x0fa1) if t == question => 3.0,
                (t, 0x0fb1) if t == question => 5.0,
                (0x5300, 0x0fb1) => 7.0,
                (t, 0xabc) if t == container => 11.0,
                _ => 0.0,
            })
        });
        fn_007d40a0(&mut e, menu);
        let list = list.addr();
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_SET_SELECTION),
            vec![vec![list, 0x5200]]
        );
        assert_eq!(calls(&e, SAVE_LOAD_LIST_SET_ENABLED), vec![vec![list, 0]]);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(6), 0x1004, 0]));
        assert!(calls(&e, TILE_SET_STRING).contains(&vec![
            tile(8),
            0x0fc4,
            SETTING_011D4FB4 + 4,
            1
        ]));
        assert_eq!(calls(&e, TILE_UPDATE_TILE), vec![vec![tile(8), 0]]);
        // first = 20 * 1 + 10, second = 11 + (7 + (5 + (100 - 3))) go to the
        // confirm dialog as its x and y (less the sibling sums)
        assert_eq!(calls(&e, MENU_SET_FLAG).len(), 0);
        let floats = calls(&e, TILE_SET_FLOAT);
        assert!(floats.contains(&vec![tile(7), 0x0fa1, 20.0f32.to_bits(), 1]));
        assert!(floats.contains(&vec![tile(7), 0x0fa2, 20.0f32.to_bits(), 1]));
        assert!(calls(&e, TILE_SET_INTEGER_VALUE).contains(&vec![tile(7), 0x0fb1, 0x55]));
    }

    #[test]
    fn opening_the_confirm_dialog_in_save_mode_uses_the_in_game_layout() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_INGAME);
        let list = list_at(menu, LIST_SAVE_LOAD);
        list_vtable(&mut e, list);
        list_vtable(&mut e, list_at(menu, LIST_CONFIRM));
        stub(&mut e, slot_fn(0x04), 0);
        // nothing highlighted: nothing happens beyond the question text choice
        fn_007d40a0(&mut e, menu);
        assert!(calls(&e, SAVE_LOAD_LIST_SET_SELECTION).is_empty());
        assert!(calls(&e, TILE_SET_STRING).is_empty());
        assert_eq!(calls(&e, MENU_GET_FLAG).len(), 3);
        // a highlighted tile: the load question and, in game, the two-button
        // layout (the height value is doubled)
        stub(&mut e, slot_fn(0x04), 0x5200);
        e.register(TILE_GET_FLOAT, |_, a| {
            float_ret(if a[1] == 0x0fb0 { 8.0 } else { 0.0 })
        });
        fn_007d40a0(&mut e, menu);
        assert!(calls(&e, TILE_SET_STRING).contains(&vec![
            tile(8),
            0x0fc4,
            SETTING_011D3F28 + 4,
            1
        ]));
        // layout bit 2 reaches the warning tile (0xfa3 of tile 20)
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(20), 0x0fa3, 1]));
    }

    #[test]
    fn options_list_populates_user_options_and_options() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let options = list_at(menu, LIST_OPTIONS);
        list_vtable(&mut e, options);
        // one user option (a plain toggle) and one plain option
        let user = e.new_object::<StartMenuUserOption>();
        e.set(user, StartMenuUserOption::pDisplayText, 0x9100);
        e.set(user, StartMenuUserOption::pTemplate, 0x9200);
        e.set(user, StartMenuUserOption::eType, 2);
        e.set(user, StartMenuUserOption::iNumSettings, 6);
        e.set(user, StartMenuUserOption::iArrowSpacing, 3);
        let plain = e.new_object::<StartMenuOption>();
        e.set(plain, StartMenuOption::pDisplayText, 0x9300);
        // user options: the meter option; options: the plain one
        let (user_addr, plain_addr) = (user.addr(), plain.addr());
        stub(&mut e, ARRAY_COUNT, 1);
        let user_slot = e.mem.alloc(4);
        e.mem.set_u32(user_slot, user_addr);
        let plain_slot = e.mem.alloc(4);
        e.mem.set_u32(plain_slot, plain_addr);
        e.register_double(ARRAY_ELEMENT, move |_, a| {
            ret(if a[0] == USER_OPTIONS_ARRAY {
                user_slot
            } else {
                plain_slot
            })
        });
        stub(&mut e, OPTION_MATCHES_MODES, 1);
        // a container exists, so AddItem renders one tile per option
        let container = e.mem.alloc(16);
        e.set(options, ListBoxLayout::pContainerRect, Ptr::new(container));
        e.set(options, ListBoxLayout::pTemplateName, 0x9400);
        stub(&mut e, MENU_RENDER_TEMPLATE, 0x5500);
        stub(&mut e, LIST_FLAG_TEST, 0);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        stub(&mut e, TILE_GET_VALUE, 1);
        start_menu_populate_options_list_box(&mut e, menu, 0x8 | 0x40);
        // the list is emptied (slot 0x1c), enabled and its title text set
        assert_eq!(calls(&e, slot_fn(0x1c)).len(), 1);
        assert_eq!(
            calls(&e, OPTION_LIST_SET_ENABLED),
            vec![vec![options.addr(), 1]]
        );
        assert!(calls(&e, TILE_SET_STRING).contains(&vec![
            tile(4),
            0x0fc4,
            SETTING_011D42F4 + 4,
            1
        ]));
        assert_eq!(
            calls(&e, OPTION_MATCHES_MODES),
            vec![vec![user_addr, 0x48], vec![plain_addr, 0x48]]
        );
        // the meter's last step and arrow spacing
        let integer = calls(&e, TILE_SET_INTEGER_VALUE);
        assert!(integer.contains(&vec![0x5500, 0x1007, 5]));
        assert!(integer.contains(&vec![0x5500, 0x1008, 3]));
        // the plain option's tile gets the meter template's three children
        let children = calls(&e, TILE_GET_CHILD_BY_ID);
        assert_eq!(
            children,
            vec![vec![0x5500, 0x64], vec![0x5500, 0x66], vec![0x5500, 0x65]]
        );
        assert_eq!(calls(&e, TILE_SET_TEXT_SOURCE), vec![vec![0x5500, 0x9300]]);
        // width trait from the menu tile
        assert!(calls(&e, TILE_SET_FLOAT)
            .iter()
            .any(|c| c[0] == 0 && c[1] == 0xabc));
    }

    #[test]
    fn options_list_skips_controller_options_without_a_controller() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let options = list_at(menu, LIST_OPTIONS);
        list_vtable(&mut e, options);
        let container = e.mem.alloc(16);
        e.set(options, ListBoxLayout::pContainerRect, Ptr::new(container));
        e.set(options, ListBoxLayout::pTemplateName, 0x9400);
        stub(&mut e, MENU_RENDER_TEMPLATE, 0x5500);
        stub(&mut e, TILE_GET_VALUE, 1);
        let controller = e.new_object::<StartMenuUserOption>();
        e.set(
            controller,
            StartMenuUserOption::pfChangeFn,
            CONTROLLER_OPTION_CHANGE_FN_B,
        );
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, controller.addr());
        e.register_double(ARRAY_COUNT, |_, a| ret((a[0] == USER_OPTIONS_ARRAY) as u32));
        e.register_double(ARRAY_ELEMENT, move |_, _| ret(slot));
        stub(&mut e, OPTION_MATCHES_MODES, 1);
        e.set(menu, StartMenu::field_1cc, Ptr::new(0x1234));
        // the import reports an error: the option is skipped and 1cc cleared
        stub(&mut e, CONTROLLER_STATE_IMPORT, 1167);
        start_menu_populate_options_list_box(&mut e, menu, 1);
        assert_eq!(e.get(menu, StartMenu::field_1cc).addr(), 0);
        let state = calls(&e, CONTROLLER_STATE_IMPORT);
        assert_eq!(state.len(), 1);
        assert_eq!(state[0][0], 0);
        assert!(calls(&e, MENU_RENDER_TEMPLATE).is_empty());
        // the import succeeds: the option is added and remembered in 1cc, and
        // its enabled trait follows whether the interface is active
        stub(&mut e, CONTROLLER_STATE_IMPORT, 0);
        stub(&mut e, INTERFACE_ACTIVE, 1);
        e.call_log = Some(Vec::new());
        start_menu_populate_options_list_box(&mut e, menu, 1);
        assert_eq!(e.get(menu, StartMenu::field_1cc).addr(), 0x5500);
        assert_eq!(calls(&e, MENU_RENDER_TEMPLATE).len(), 1);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x5500, 0x2000, 1]));
    }

    #[test]
    fn confirm_dialog_position_comes_from_the_main_container() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let main_list = list_at(menu, LIST_MAIN);
        list_vtable(&mut e, main_list);
        list_vtable(&mut e, list_at(menu, LIST_CONFIRM));
        stub(&mut e, slot_fn(0x04), 0x5400);
        stub_float(&mut e, TILE_CHAIN_SUM_FA1, 100.0);
        stub_float(&mut e, INTERFACE_UNIT_SIZE, 10.0);
        e.register_double(TILE_CHAIN_SUM_FA2, |_, a| {
            float_ret(if a[0] == 0x5400 { 200.0 } else { 40.0 })
        });
        // layout 2: top 0x50, warning shown
        fn_007d48a0(&mut e, menu, 0x11, 2);
        // x = (100 - 0x50) - 10 = 10, y = 200 - 0x3c = 140; the confirm
        // container's x and y are those less the sibling's sums (100, 40)
        let floats = calls(&e, TILE_SET_FLOAT);
        assert!(floats.contains(&vec![tile(7), 0x0fa1, (-90.0f32).to_bits(), 1]));
        assert!(floats.contains(&vec![tile(7), 0x0fa2, 100.0f32.to_bits(), 1]));
        assert!(calls(&e, TILE_SET_INTEGER_VALUE).contains(&vec![tile(7), 0x0fb1, 0x50]));
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(20), 0x0fa3, 1]));
    }

    #[test]
    fn confirm_dialog_forwarders_choose_spacing_and_warning_from_the_layout() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        list_vtable(&mut e, list_at(menu, LIST_CONFIRM));
        fn_007d4950(&mut e, menu, 7, 111, 222, 3);
        let integer = calls(&e, TILE_SET_INTEGER_VALUE);
        assert!(integer.contains(&vec![tile(7), 0x0fb1, 0xa8]));
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(20), 0x0fa3, 1]));
        // fade, empty list
        let confirm = list_of(menu, 0xe4);
        assert_eq!(calls(&e, OPTION_LIST_SET_ENABLED), vec![vec![confirm, 1]]);
        e.call_log = Some(Vec::new());
        fn_007d4950(&mut e, menu, 7, 111, 222, 0);
        assert!(calls(&e, TILE_SET_INTEGER_VALUE).contains(&vec![tile(7), 0x0fb1, 0x55]));
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(20), 0x0fa3, 0]));
    }

    #[test]
    fn confirm_dialog_lists_the_options_whose_modes_match() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let confirm = list_at(menu, LIST_CONFIRM);
        list_vtable(&mut e, confirm);
        let yes = e.new_object::<StartMenuOption>();
        e.set(yes, StartMenuOption::pDisplayText, 0x9100);
        let no = e.new_object::<StartMenuOption>();
        e.set(no, StartMenuOption::pDisplayText, 0x9200);
        install_array(&mut e, OPTIONS_ARRAY, &[yes.addr(), no.addr()]);
        let (yes_addr, no_addr) = (yes.addr(), no.addr());
        e.register(OPTION_MATCHES_MODES, |_, a| ret((a[1] == 0x200) as u32));
        stub(&mut e, MENU_RENDER_TEMPLATE, 0x5500);
        // the list has no container: AddItem returns null for each match
        fn_007d49a0(&mut e, menu, 0x200, 40, 50, 9, true);
        assert_eq!(
            calls(&e, OPTION_MATCHES_MODES),
            vec![vec![yes_addr, 0x200], vec![no_addr, 0x200]]
        );
        assert_eq!(calls(&e, slot_fn(0x1c)), vec![vec![confirm.addr()]]);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(20), 0x0fa3, 1]));
        assert!(calls(&e, TILE_SET_INTEGER_VALUE).contains(&vec![tile(7), 0x0fb1, 9]));
        // x and y: 40 - 0 and 50 - 0 (the sibling sums are zero)
        let floats = calls(&e, TILE_SET_FLOAT);
        assert!(floats.contains(&vec![tile(7), 0x0fa1, 40.0f32.to_bits(), 1]));
        assert!(floats.contains(&vec![tile(7), 0x0fa2, 50.0f32.to_bits(), 1]));
        // each matching option's text goes to the (null) item tile
        assert_eq!(
            calls(&e, TILE_SET_TEXT_SOURCE),
            vec![vec![0, 0x9100], vec![0, 0x9200]]
        );
    }

    #[test]
    fn save_load_list_is_filled_from_the_managers_list() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        e.set_global(LOADING_MENU, 0x4321);
        let manager = e.mem.alloc(8);
        e.set_global(SAVE_LOAD_MANAGER, manager);
        // nodes {item, next}: two entries, then a node without an item
        let (node3, node2, node1) = (e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(node1, 0xe1);
        e.mem.set_u32(node1 + 4, node2);
        e.mem.set_u32(node2, 0xe2);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(manager, node1);
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_NODE_SELF, |_, a| ret(a[0]));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        start_menu_populate_save_load_list_box(&mut e, menu);
        let list = list_of(menu, 0x174);
        assert_eq!(calls(&e, LOADING_MENU_RESUME_BACKGROUND_THREAD).len(), 1);
        assert_eq!(calls(&e, SAVE_LOAD_MANAGER_REFRESH), vec![vec![manager]]);
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_ADD_ITEM),
            vec![vec![list, 0xe1, 0, 0, 0], vec![list, 0xe2, 0, 0, 0]]
        );
    }

    #[test]
    fn downloads_list_gets_a_none_item_when_empty() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let downloads = list_of(menu, 0x144);
        fn_007d4bc0(&mut e, menu);
        assert_eq!(
            calls(&e, DOWNLOADS_LIST_ADD_ITEM),
            vec![vec![downloads, u32::MAX, SETTING_011D3AD4 + 4, 0, 0]]
        );
        assert_eq!(
            calls(&e, DOWNLOADS_LIST_SET_ENABLED),
            vec![vec![downloads, 1]]
        );
        // the DLC container (14) and the back button (5) fade in
        let fades = calls(&e, TILE_ADD_FADE_CONTROL);
        assert_eq!((fades[0][0], fades[1][0]), (tile(14), tile(5)));
        e.call_log = Some(Vec::new());
        stub(&mut e, LIST_COUNT, 2);
        fn_007d4bc0(&mut e, menu);
        assert!(calls(&e, DOWNLOADS_LIST_ADD_ITEM).is_empty());
    }

    #[test]
    fn the_dlc_callback_adds_names_and_finishes_with_the_list() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let downloads = list_of(menu, 0x144);
        // the wide-to-narrow conversion writes "dlc" and the add records it
        let seen = Rc::new(RefCell::new(Vec::new()));
        e.register(STRING_CHARACTERS, |_, a| ret(a[0] + 0x10));
        e.register(WCSTOMBS, |e, a| {
            e.mem.set_cstr(a[0], b"dlc");
            ret(3)
        });
        let record = Rc::clone(&seen);
        e.register_double(DOWNLOADS_LIST_ADD_ITEM, move |e, a| {
            record.borrow_mut().push((a[0], a[1], e.mem.cstr(a[2])));
            Ret::default()
        });
        fn_007d4c50(&mut e, 0, 1, 1, 0x7777);
        assert_eq!(*seen.borrow(), vec![(downloads, 0, b"dlc".to_vec())]);
        assert_eq!(calls(&e, WCSTOMBS)[0][1..], [0x7787, 0x80]);
        // the terminating call closes the message menu and shows the list
        fn_007d4c50(&mut e, -1, -1, 0, 0);
        assert_eq!(calls(&e, CLOSE_MESSAGE_MENU).len(), 1);
        assert_eq!(
            calls(&e, DOWNLOADS_LIST_SET_ENABLED),
            vec![vec![downloads, 1]]
        );
        // a call with only one of the two indices -1 is an ordinary item
        fn_007d4c50(&mut e, -1, 0, 0, 0x7777);
        let dlc_items = seen.borrow().iter().filter(|item| item.2 == b"dlc").count();
        assert_eq!(dlc_items, 2);
    }

    #[test]
    fn display_option_shows_meters_and_toggles() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let option = e.new_object::<StartMenuUserOption>();
        e.set(option, StartMenuUserOption::eType, 2);
        e.set(option, StartMenuUserOption::iVal, 7);
        e.set(option, StartMenuUserOption::iNumSettings, 10);
        // a meter: the value itself, both arrows
        start_menu_display_option(&mut e, menu, option.addr(), 0x5600);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x5600, 0x1004, 7], vec![0x5600, 0x1006, 1]]
        );
        // a toggle at its first value: the text, only the right arrow
        let strings = e.mem.alloc(8);
        e.mem.set_u32(strings, 0xa100);
        e.mem.set_u32(strings + 4, 0xa200);
        e.set(option, StartMenuUserOption::eType, 1);
        e.set(option, StartMenuUserOption::iNumSettings, 2);
        e.set(option, StartMenuUserOption::pValueStrings, strings);
        e.set(option, StartMenuUserOption::iVal, 0);
        e.call_log = Some(Vec::new());
        start_menu_display_option(&mut e, menu, option.addr(), 0x5600);
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x5600, 0x1004, 0xa100, 1]]
        );
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x5600, 0x1006, 2]]);
        // the last value: only the left arrow
        e.set(option, StartMenuUserOption::iVal, 1);
        e.call_log = Some(Vec::new());
        start_menu_display_option(&mut e, menu, option.addr(), 0x5600);
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x5600, 0x1004, 0xa200, 1]]
        );
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x5600, 0x1006, 0]]);
        // a value out of range shows no text; a plain toggle (type 0) always 1
        e.set(option, StartMenuUserOption::eType, 0);
        e.set(option, StartMenuUserOption::iVal, 5);
        e.call_log = Some(Vec::new());
        start_menu_display_option(&mut e, menu, option.addr(), 0x5600);
        assert!(calls(&e, TILE_SET_STRING).is_empty());
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x5600, 0x1006, 1]]);
        // no option or no tile at all: nothing
        e.call_log = Some(Vec::new());
        start_menu_display_option(&mut e, menu, 0, 0x5600);
        assert!(calls(&e, TILE_SET_INT).is_empty());
    }

    #[test]
    fn display_option_looks_the_tile_up_in_the_options_list() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let option = e.new_object::<StartMenuUserOption>();
        e.set(option, StartMenuUserOption::eType, 2);
        e.set(option, StartMenuUserOption::iVal, 3);
        let found = Rc::new(RefCell::new(0));
        let record = Rc::clone(&found);
        e.register_double(OPTION_LIST_GET_TILE_FOR, move |e, a| {
            // the argument is the address of a cell holding the option
            *record.borrow_mut() = e.mem.u32(a[1]);
            ret(0x5700)
        });
        start_menu_display_option(&mut e, menu, option.addr(), 0);
        assert_eq!(*found.borrow(), option.addr());
        assert_eq!(
            calls(&e, OPTION_LIST_GET_TILE_FOR)[0][0],
            list_of(menu, 0x114)
        );
        assert_eq!(calls(&e, TILE_SET_INT)[0], vec![0x5700, 0x1004, 3]);
        // a lookup that fails ends the function
        stub(&mut e, OPTION_LIST_GET_TILE_FOR, 0);
        e.call_log = Some(Vec::new());
        start_menu_display_option(&mut e, menu, option.addr(), 0);
        assert!(calls(&e, TILE_SET_INT).is_empty());
    }

    /// Builds a `BGSSaveLoadFileEntry`-like block with the fields the info
    /// panel reads.
    fn save_entry(e: &mut Engine) -> u32 {
        let entry = e.mem.alloc(0x80);
        e.mem.set_u32(entry + 0x08, 42); // iSaveGameNumber
        e.mem.set_u32(entry + 0x0c, 0x9001); // pPlayerName
        e.mem.set_u32(entry + 0x10, 0x9002); // pPlayerTitle
        e.mem.set_u32(entry + 0x14, 0x9004); // pLocation
        e.mem.set_u32(entry + 0x18, 0x9003); // pPlayTime
        e.mem.set_u32(entry + 0x1c, 12); // iPlayerLevel
        entry
    }

    #[test]
    fn the_info_panel_clears_a_pending_backout_and_does_nothing_else() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_SAVELOAD_BACKOUT);
        start_menu_update_highlighted_save_game_info(&mut e, menu);
        assert_eq!(flags(&e, menu), 0);
        assert!(calls(&e, TILE_SET_INT).is_empty());
        assert!(calls(&e, LIST_SELECTED_DATA).is_empty());
    }

    #[test]
    fn the_info_panel_shows_the_highlighted_entrys_details() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let entry = save_entry(&mut e);
        stub(&mut e, LIST_SELECTED_DATA, entry);
        stub(&mut e, FILE_ENTRY_PLAYER_NAME, 0x9001);
        stub(&mut e, FILE_ENTRY_LANGUAGE, entry + 0x3c);
        stub(&mut e, FIRST_WORD, 0x9100);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        let formats = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&formats);
        e.register_double(STRING_FORMAT, move |e, a| {
            // the blank text is the fifth word of the level line, the fourth
            // of the other two
            let blank = e.mem.cstr(a[if a.len() == 6 { 4 } else { 3 }]);
            record.borrow_mut().push((a.to_vec(), blank));
            Ret::default()
        });
        start_menu_update_highlighted_save_game_info(&mut e, menu);
        // the language setting is compared with "FRENCH": equal (the
        // double's 0) gives a blank " ", otherwise ""
        assert_eq!(
            calls(&e, STRING_COMPARE),
            vec![vec![SETTING_011C3CB4 + 4, LANGUAGE_FRENCH]]
        );
        let formats = formats.borrow();
        assert_eq!(formats.len(), 3);
        let object = formats[0].0[0];
        assert_eq!(
            formats[0].0,
            vec![
                object,
                FORMAT_LEVEL,
                SETTING_011D3B2C + 4,
                12,
                formats[0].0[4],
                0x9002
            ]
        );
        assert_eq!(
            formats[1].0[1..3],
            [FORMAT_TWO_STRINGS, SETTING_011D3954 + 4]
        );
        assert_eq!(formats[1].0[4], 0x9003);
        assert_eq!(
            formats[2].0[1..3],
            [FORMAT_TWO_STRINGS, SETTING_011D21B4 + 4]
        );
        assert_eq!(formats[2].0[4], entry + 0x3c);
        assert!(formats.iter().all(|f| f.0[0] == object && f.1 == b" "));
        assert_eq!(calls(&e, STRING_CONSTRUCT), vec![vec![object]]);
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![object]]);
        let strings = calls(&e, TILE_SET_STRING);
        assert!(strings.contains(&vec![tile(12), 0x0fc4, 0x9001, 1]));
        for panel_tile in [18, 19, 22] {
            assert!(strings.contains(&vec![tile(panel_tile), 0x0fc4, 0x9100, 1]));
        }
        // visibility: no screenshot, every text shown; "_target" follows the
        // entry
        let ints = calls(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![tile(4), 0xabc, 1]));
        assert!(ints.contains(&vec![tile(11), 0x0fa3, 0]));
        for shown in [12, 18, 19, 22] {
            assert!(ints.contains(&vec![tile(shown), 0x0fa3, 1]));
        }
    }

    #[test]
    fn the_info_panel_uses_no_blank_for_other_languages() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let entry = save_entry(&mut e);
        stub(&mut e, LIST_SELECTED_DATA, entry);
        stub(&mut e, FILE_ENTRY_PLAYER_NAME, 0);
        stub(&mut e, STRING_COMPARE, 1);
        let blanks = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&blanks);
        e.register_double(STRING_FORMAT, move |e, a| {
            record
                .borrow_mut()
                .push(e.mem.cstr(a[if a.len() == 6 { 4 } else { 3 }]));
            Ret::default()
        });
        let frame = next_frame(&mut e, 0x30);
        start_menu_update_highlighted_save_game_info(&mut e, menu);
        // The 0x30-byte frame: the text at +0x10, the blank at +8.
        assert_eq!(calls(&e, STRING_CONSTRUCT)[0][0], frame + 0x10);
        assert_eq!(calls(&e, STRING_FORMAT)[0][4], frame + 8);
        assert_eq!(*blanks.borrow(), vec![Vec::<u8>::new(); 3]);
        // no player name: its tile is not shown
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(12), 0x0fa3, 0]));
    }

    #[test]
    fn the_info_panel_shows_only_the_corrupt_text_for_a_corrupt_entry() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let entry = save_entry(&mut e);
        stub(&mut e, LIST_SELECTED_DATA, entry);
        stub(&mut e, FILE_ENTRY_IS_CORRUPT, 1);
        start_menu_update_highlighted_save_game_info(&mut e, menu);
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![tile(12), 0x0fc4, SETTING_011D1F14 + 4, 1]]
        );
        assert!(calls(&e, STRING_FORMAT).is_empty());
        let ints = calls(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![tile(12), 0x0fa3, 1]));
        for hidden in [11, 18, 19, 22] {
            assert!(ints.contains(&vec![tile(hidden), 0x0fa3, 0]));
        }
    }

    #[test]
    fn the_info_panel_arms_the_idle_timer_when_the_interface_is_active() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let entry = save_entry(&mut e);
        let timer = e.mem.alloc(8);
        e.mem.set_u32(timer + 4, 9);
        stub(&mut e, LIST_SELECTED_DATA, entry);
        stub(&mut e, FILE_ENTRY_IS_CORRUPT, 1);
        stub(&mut e, INTERFACE_ACTIVE, 1);
        stub(&mut e, FLAG_SETTING_BYTE, 1);
        stub(&mut e, IDLE_TIMER_GET, timer);
        stub(&mut e, TICK_COUNT, 777);
        start_menu_update_highlighted_save_game_info(&mut e, menu);
        // the timer starts now and the screenshot is hidden and shown
        assert_eq!(e.mem.u32(timer), 777);
        assert_eq!(e.mem.u32(timer + 4), 0);
        assert_eq!(
            calls(&e, FLAG_SETTING_BYTE),
            vec![vec![FLAG_SETTING_OBJECT]]
        );
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![tile(11), 0x0fa9, 0, 1]]
        );
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(11), 0x0fa3, 1]));
        // no entry: the timer is reset instead
        stub(&mut e, LIST_SELECTED_DATA, 0);
        e.call_log = Some(Vec::new());
        start_menu_update_highlighted_save_game_info(&mut e, menu);
        assert_eq!(calls(&e, IDLE_TIMER_RESET), vec![vec![timer]]);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(4), 0, 0]));
    }

    #[test]
    fn idle_timer_helpers_and_entry_accessors() {
        let mut e = engine();
        let timer = Ptr::<()>::new(e.mem.alloc(8));
        stub(&mut e, TICK_COUNT, 777);
        e.mem.set_u32(timer.addr() + 4, 5);
        fn_007d5120(&mut e, timer);
        assert_eq!(e.mem.u32(timer.addr()), 777);
        assert_eq!(e.mem.u32(timer.addr() + 4), 0);
        let entry = Ptr::<()>::new(save_entry(&mut e));
        assert_eq!(fn_007d5140(&mut e, entry), 0x9002);
        assert_eq!(fn_007d5160(&mut e, entry), 12);
        assert_eq!(fn_007d5180(&mut e, entry), 0x9003);
        assert_eq!(fn_007d6950(&mut e, entry), 42);
        assert_eq!(fn_007d6970(&mut e, entry), 0x9004);
        // each one loads the entry's data first
        assert_eq!(calls(&e, FILE_ENTRY_LOAD_DATA), vec![vec![entry.addr()]; 5]);
        stub(&mut e, FIRST_WORD, 0x4242);
        assert_eq!(fn_007d5280(&mut e, timer), 0x4242);
        assert_eq!(calls(&e, FIRST_WORD), vec![vec![timer.addr() + 0x40]]);
    }

    #[test]
    fn the_save_game_screenshot_is_sized_from_the_image() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let image = e.mem.alloc(8);
        let slots: Vec<u32> = (0..40).map(|i| 0x6100_1000 + 4 * i).collect();
        e.put_vtable(0x6100_0000, &slots);
        e.mem.set_u32(image, 0x6100_0000);
        stub(&mut e, 0x6100_1000 + 0x98, 640);
        stub(&mut e, 0x6100_1000 + 0x94, 480);
        e.register(FIRST_WORD, |_, a| ret(a[0] + 1));
        // without a given entry the list's selected one is used
        stub(&mut e, LIST_SELECTED_DATA, 0xe1);
        stub(&mut e, FILE_ENTRY_GET_SCREENSHOT, image);
        assert!(start_menu_set_save_game_screenshot(&mut e, menu, 0));
        let screen = tile(11);
        assert_eq!(calls(&e, FILE_ENTRY_GET_SCREENSHOT), vec![vec![0xe1]]);
        assert_eq!(
            calls(&e, TILE_IMAGE_SET_SOURCE_TEXTURE),
            vec![vec![screen, image]]
        );
        assert_eq!(
            calls(&e, TILE_SET_INTEGER_VALUE),
            vec![vec![screen, 0x0fb0, 640], vec![screen, 0x0fb1, 480]]
        );
        // no alpha map yet: the menu's is given to the tile image
        let image_slot = screen + 0x40 + 1;
        assert_eq!(calls(&e, IMAGE_ALPHA_MAP), vec![vec![image_slot]]);
        assert_eq!(
            calls(&e, IMAGE_SET_ALPHA_MAP),
            vec![vec![image_slot, menu.addr() + 0x1b8 + 1]]
        );
        // an existing alpha map is kept
        stub(&mut e, IMAGE_ALPHA_MAP, 7);
        e.call_log = Some(Vec::new());
        assert!(start_menu_set_save_game_screenshot(&mut e, menu, 0xe2));
        assert!(calls(&e, IMAGE_SET_ALPHA_MAP).is_empty());
        assert!(calls(&e, LIST_SELECTED_DATA).is_empty());
        // no screenshot, or no entry at all
        stub(&mut e, FILE_ENTRY_GET_SCREENSHOT, 0);
        assert!(!start_menu_set_save_game_screenshot(&mut e, menu, 0xe2));
        stub(&mut e, LIST_SELECTED_DATA, 0);
        assert!(!start_menu_set_save_game_screenshot(&mut e, menu, 0));
    }

    /// A menu in action-mapping mode whose options container finished
    /// fading with a non-zero alpha, waiting for a remap press with the
    /// console hidden; the highlighted option's action is 77.
    fn remapping_menu(e: &mut Engine, selector: i32) -> Ptr<StartMenu> {
        let menu = new_menu(e);
        set_flags(e, menu, FLAG_ACTION_MAPPING);
        e.set(menu, StartMenu::field_1c8, selector);
        stub(e, TILE_GET_FADE_FINISHED, 1);
        stub(e, TILE_ADD_USER_TRAIT, 0xabc);
        stub_float(e, TILE_GET_FLOAT, 1.0);
        stub(e, WAITING_FOR_REMAP_PRESS, 1);
        e.set_global(CONTROLS_OWNER, 0x1111);
        stub(e, CONTROLS_GET, 0x2222);
        let option = e.mem.alloc(0x10);
        e.mem.set_u32(option + 0xc, 77);
        stub(e, LIST_SELECTED_DATA, option);
        e.mem.set_u32(REMAP_PROMPT_TABLE, 0x011d_6000);
        e.mem.set_u32(REMAP_PROMPT_TABLE + 4, 0x011d_6010);
        e.mem.set_u32(REMAP_PROMPT_TABLE + 8, 0x011d_6020);
        menu
    }

    #[test]
    fn remapping_does_nothing_without_the_flag_or_before_the_fade_ends() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        assert!(!fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, MENU_GET_FLAG).len(), 1);
        assert!(calls(&e, TILE_GET_FADE_FINISHED).is_empty());
        set_flags(&mut e, menu, FLAG_ACTION_MAPPING);
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, TILE_GET_FADE_FINISHED).len(), 1);
        assert!(calls(&e, TILE_GET_FLOAT).is_empty());
    }

    #[test]
    fn remapping_fades_the_options_back_in_when_their_alpha_is_zero() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_ACTION_MAPPING);
        stub(&mut e, TILE_GET_FADE_FINISHED, 1);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, FN_007D01B0), vec![vec![menu.addr()]]);
        let fades = calls(&e, TILE_ADD_FADE_CONTROL);
        assert_eq!(fades.len(), 1);
        assert_eq!(fades[0][..2], [tile(2), 0xabc]);
        assert_eq!(fades[0][4], 4.5f32.to_bits());
        // the device button's value is 1 while the interface is not active
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(15), 0x1004, 1]));
        assert!(calls(&e, KEY_STATE).is_empty());
    }

    /// Binds nothing: the keyboard scan sees one key down.
    fn keyboard_with_key_down(e: &mut Engine, key: u32, accepted: bool) {
        e.register_double(KEY_STATE, move |_, a| ret((a[1] == key) as u32));
        stub(e, BIND_INPUT, accepted as u32);
    }

    #[test]
    fn remapping_binds_the_pressed_key() {
        let mut e = engine();
        let menu = remapping_menu(&mut e, 0);
        keyboard_with_key_down(&mut e, 30, true);
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, KEY_STATE).len(), 31);
        assert_eq!(calls(&e, KEY_STATE)[30], vec![0x2222, 30, 1]);
        assert_eq!(calls(&e, BIND_INPUT), vec![vec![0x2222, 77, 0, 30]]);
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![0xa]]);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        // the options list is refreshed and the header shows the prompt
        let options = list_of(menu, 0x114);
        assert_eq!(calls(&e, OPTION_LIST_SET_ENABLED), vec![vec![options, 1]]);
        assert_eq!(calls(&e, OPTION_LIST_DESELECT_ALL), vec![vec![options, 1]]);
        assert_eq!(
            calls(&e, LIST_FOR_RANGE),
            vec![vec![options, OPTION_LIST_REFRESH_FN, 0, 0x7fff_ffff]]
        );
        assert!(calls(&e, TILE_SET_STRING).contains(&vec![tile(16), 0x0fc4, 0x011d_6004, 1]));
        assert_eq!(flags(&e, menu), FLAG_ACTION_MAPPING | FLAG_OPTIONS_DIRTY);
    }

    #[test]
    fn remapping_refuses_a_key_the_controls_reject() {
        let mut e = engine();
        let menu = remapping_menu(&mut e, 0);
        keyboard_with_key_down(&mut e, 30, false);
        assert!(fn_007d52a0(&mut e, menu));
        let messages = calls(&e, SHOW_MESSAGE);
        assert_eq!(
            messages,
            vec![vec![
                SETTING_011D4750 + 4,
                0,
                0,
                0,
                1,
                0x12,
                0,
                0,
                MESSAGE_ICON_SETTING + 4,
                0
            ]]
        );
        assert!(calls(&e, OPTION_LIST_SET_ENABLED).is_empty());
        // key 1 counts as bound whatever the controls say, with no message
        keyboard_with_key_down(&mut e, 1, false);
        e.call_log = Some(Vec::new());
        assert!(fn_007d52a0(&mut e, menu));
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        assert_eq!(calls(&e, OPTION_LIST_SET_ENABLED).len(), 1);
    }

    #[test]
    fn remapping_binds_mouse_buttons_and_the_cancel_binding() {
        let mut e = engine();
        let menu = remapping_menu(&mut e, 1);
        stub(&mut e, MOUSE_BUTTON_COUNT, 3);
        e.register_double(MOUSE_BUTTON_STATE, |_, a| ret((a[1] == 2) as u32));
        stub(&mut e, BIND_INPUT, 1);
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, MOUSE_BUTTON_STATE)[2], vec![0x2222, 2, 1]);
        assert_eq!(calls(&e, BIND_INPUT), vec![vec![0x2222, 77, 1, 2]]);
        // the sound comes before the bind, and the cancel binding is queried
        assert_eq!(
            order(&e, &[PLAY_MENU_SOUND, BIND_INPUT, BINDING_CANCEL]),
            vec![PLAY_MENU_SOUND, BIND_INPUT, BINDING_CANCEL]
        );
        assert_eq!(calls(&e, BINDING_CANCEL), vec![vec![0x2222, 0x1c, 1]]);
        assert!(calls(&e, CLEAR_BINDING).is_empty());
        // nothing pressed, but cancel: the action's binding is cleared
        let menu = remapping_menu(&mut e, 1);
        e.register_double(MOUSE_BUTTON_STATE, |_, _| ret(0));
        stub(&mut e, BINDING_CANCEL, 1);
        e.call_log = Some(Vec::new());
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, CLEAR_BINDING), vec![vec![0x2222, 77, 1, 0xff]]);
        assert!(calls(&e, BIND_INPUT).is_empty());
        assert_eq!(calls(&e, OPTION_LIST_SET_ENABLED).len(), 1);
    }

    #[test]
    fn remapping_binds_gamepad_buttons_and_axes() {
        let mut e = engine();
        let menu = remapping_menu(&mut e, 2);
        stub(&mut e, GAMEPAD_BUTTON_COUNT, 2);
        e.register_double(GAMEPAD_BUTTON_STATE, |_, a| ret((a[2] == 1) as u32));
        stub(&mut e, BIND_INPUT, 1);
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, GAMEPAD_BUTTON_STATE)[1], vec![0x2222, 0, 1, 1]);
        assert_eq!(calls(&e, BIND_INPUT), vec![vec![0x2222, 77, 2, 1]]);
        assert!(calls(&e, GAMEPAD_AXIS_COUNT).is_empty());
        // no button: the axes are scanned, axis 0 has no source, axis 1
        // source 5, and a bind postpones the next one by 100 ms
        let menu = remapping_menu(&mut e, 2);
        e.register_double(GAMEPAD_BUTTON_STATE, |_, _| ret(0));
        stub(&mut e, GAMEPAD_AXIS_COUNT, 2);
        e.register_double(GAMEPAD_AXIS_SOURCE, |_, a| {
            ret(if a[2] == 1 { 5 } else { u32::MAX })
        });
        stub(&mut e, TICK_COUNT, 1000);
        stub(&mut e, GET_TICK_COUNT_IMPORT, 2000);
        e.set_global(REMAP_INPUT_DEADLINE, 500u32);
        e.call_log = Some(Vec::new());
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, BIND_INPUT), vec![vec![0x2222, 77, 2, 5]]);
        assert_eq!(e.global::<u32>(REMAP_INPUT_DEADLINE), 2100);
        assert_eq!(calls(&e, PLAY_MENU_SOUND).len(), 2);
        // before the deadline nothing is bound
        let menu = remapping_menu(&mut e, 2);
        e.set_global(REMAP_INPUT_DEADLINE, 5000u32);
        e.call_log = Some(Vec::new());
        assert!(fn_007d52a0(&mut e, menu));
        assert!(calls(&e, BIND_INPUT).is_empty());
        assert_eq!(e.global::<u32>(REMAP_INPUT_DEADLINE), 5000);
    }

    #[test]
    fn a_pushed_stick_shows_a_message_while_waiting_for_a_remap_press() {
        let mut e = engine();
        let menu = remapping_menu(&mut e, 0xff);
        stub(&mut e, INTERFACE_ACTIVE, 1);
        e.register(ABSOLUTE_VALUE, |_, a| ret((a[0] as i32).unsigned_abs()));
        e.register_double(GAMEPAD_AXIS_VALUE, |_, a| {
            ret(if a[2] == 9 { 0x3000 } else { 0 })
        });
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, GAMEPAD_AXIS_VALUE).len(), 2);
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![0xa]]);
        let messages = calls(&e, SHOW_MESSAGE);
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0][0], SETTING_011D3108 + 4);
        // the second axis alone is enough too; small values are not
        e.register_double(GAMEPAD_AXIS_VALUE, |_, a| {
            ret(if a[2] == 0xa {
                (-0x2200i32) as u32
            } else {
                0x2000
            })
        });
        e.call_log = Some(Vec::new());
        assert!(fn_007d52a0(&mut e, menu));
        assert_eq!(calls(&e, SHOW_MESSAGE).len(), 1);
        e.register_double(GAMEPAD_AXIS_VALUE, |_, _| ret(0x21f1));
        e.call_log = Some(Vec::new());
        assert!(fn_007d52a0(&mut e, menu));
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn housekeeping_stops_at_once_in_game() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_INGAME | FLAG_FADE_IN);
        assert!(!fn_007d5830(&mut e, menu));
        assert!(calls(&e, TILE_HAS_FADE_CONTROL).is_empty());
    }

    #[test]
    fn housekeeping_fades_the_title_in_and_updates_the_seed() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_FADE_IN);
        // the title alpha is 0: the fade starts
        assert!(fn_007d5830(&mut e, menu));
        let fades = calls(&e, TILE_ADD_FADE_CONTROL);
        assert_eq!(fades.len(), 1);
        assert_eq!(fades[0][..2], [tile(3), 0x0fa9]);
        assert_eq!(fades[0][4], 1.0f32.to_bits());
        assert_eq!(calls(&e, FN_0078D2D0), vec![vec![1]]);
        assert_eq!(flags(&e, menu), FLAG_FADE_IN);
        // the alpha is 1.0: the menu is rebuilt and the seed rewritten
        let list = list_at(menu, LIST_SAVE_LOAD);
        list_vtable(&mut e, list);
        stub_float(&mut e, TILE_GET_FLOAT, 1.0);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 40);
        stub(&mut e, SEED_CELL, cell);
        stub(&mut e, DATA_HANDLER_COUNT, 4);
        e.set_global(DATA_HANDLER_OBJECT, 0x4444u32);
        e.register(MAX_INT, |_, a| ret((a[0] as i32).max(a[1] as i32) as u32));
        e.register(MIN_INT, |_, a| ret((a[0] as i32).min(a[1] as i32) as u32));
        e.mem.set_u8(SEED_DIRTY_BYTE, 1);
        e.call_log = Some(Vec::new());
        assert!(fn_007d5830(&mut e, menu));
        // 40 + 5 * 4 = 60, clamped to 50
        assert_eq!(e.mem.u32(cell), 50);
        assert_eq!(calls(&e, DATA_HANDLER_COUNT), vec![vec![0x4444]]);
        assert_eq!(calls(&e, MAX_INT), vec![vec![60, 0x1e]]);
        assert_eq!(calls(&e, MIN_INT), vec![vec![60, 0x32]]);
        assert_eq!(e.mem.u8(SEED_DIRTY_BYTE), 0);
        // the rebuild marks the DLC state pending
        assert_eq!(flags(&e, menu), FLAG_DLC_NEEDS_UPDATE | FLAG_SAVE_OPTIONS);
        assert_eq!(calls(&e, slot_fn(0x1c)).len(), 1);
        // a positive global changes nothing
        set_flags(&mut e, menu, FLAG_FADE_IN);
        e.set_global(TES_GLOBAL_POINTER, 0x5000u32);
        stub_float(&mut e, TES_GLOBAL_GET_VALUE, 2.0);
        e.call_log = Some(Vec::new());
        assert!(fn_007d5830(&mut e, menu));
        assert_eq!(
            flags(&e, menu),
            FLAG_FADE_IN | FLAG_DLC_NEEDS_UPDATE | FLAG_SAVE_OPTIONS
        );
        assert!(calls(&e, TILE_GET_FLOAT).is_empty());
    }

    #[test]
    fn housekeeping_polls_the_loading_menu_and_the_dlc_flags() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        e.set_global(LOADING_MENU, 0x4321u32);
        set_flags(&mut e, menu, FLAG_IIS_PREFADE);
        e.register_double(MOUSE_BUTTON_STATE, |_, a| ret((a[1..] == [0, 2]) as u32));
        stub(&mut e, CONTROLS_GET, 0x2222);
        assert!(fn_007d5830(&mut e, menu));
        assert_eq!(calls(&e, MOUSE_BUTTON_STATE), vec![vec![0x2222, 0, 2]]);
        assert_eq!(calls(&e, LOADING_MENU_FN_078AA90), vec![vec![0x4321]]);
        // a new DLC makes the global menu wait for it
        e.call_log = Some(Vec::new());
        e.set(menu, StartMenu::uiFlags, FLAG_NEW_DLC_INSTALLED);
        assert!(fn_007d5830(&mut e, menu));
        assert_eq!(flags(&e, menu), FLAG_DLC_WAIT_MESSAGE);
        // waiting without the DLC menu shows the message
        e.call_log = Some(Vec::new());
        assert!(fn_007d5830(&mut e, menu));
        let messages = calls(&e, SHOW_MESSAGE);
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0][0], SETTING_011D3E2C + 4);
        assert_eq!(messages[0][8], MESSAGE_FOOTER);
        assert_eq!(messages[0].len(), 9);
        // with the menu present and its state 1 the wait ends
        stub(&mut e, TILE_GET_MENU_BY_CLASS, 0x5800);
        stub(&mut e, TILE_GET_MENU, 0x5801);
        stub(&mut e, DLC_ENUMERATION_STATE, 1);
        let downloads = list_at(menu, LIST_DOWNLOADS);
        list_vtable(&mut e, downloads);
        e.call_log = Some(Vec::new());
        assert!(fn_007d5830(&mut e, menu));
        assert_eq!(calls(&e, DLC_ENUMERATION_STATE), vec![vec![0x5801]]);
        assert_eq!(calls(&e, DLC_FINISH).len(), 1);
        // rebuilding the downloads list found DLC content to report
        assert_eq!(flags(&e, menu), FLAG_DLC_CORRUPT_MESSAGE);
    }

    #[test]
    fn housekeeping_reports_corrupt_dlc_once_the_menu_is_gone() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_DLC_CORRUPT_MESSAGE);
        stub(&mut e, TILE_GET_MENU_BY_CLASS, 0x5800);
        assert!(fn_007d5830(&mut e, menu));
        assert_eq!(flags(&e, menu), FLAG_DLC_CORRUPT_MESSAGE);
        stub(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        assert!(fn_007d5830(&mut e, menu));
        assert_eq!(flags(&e, menu), 0);
        let messages = calls(&e, SHOW_MESSAGE);
        assert_eq!(
            messages,
            vec![vec![
                SETTING_011D2F7C + 4,
                0,
                0,
                0,
                0,
                0x12,
                0,
                0,
                MESSAGE_ICON_SETTING + 4,
                0
            ]]
        );
    }

    #[test]
    fn the_save_load_idle_needs_a_valid_container() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_SAVE_WAIT_MESSAGE);
        assert!(!start_menu_save_load_idle(&mut e, menu));
        assert_eq!(flags(&e, menu), FLAG_SAVE_WAIT_MESSAGE);
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        stub(&mut e, TICK_COUNT, 1000);
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(e.global::<u32>(SAVE_WAIT_DEADLINE), 1000 + 0xbb8);
        assert_eq!(flags(&e, menu), FLAG_SAVE_WAIT_SAVING);
    }

    #[test]
    fn the_save_load_idle_starts_the_save_and_closes_after_the_deadline() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        stub(&mut e, TICK_COUNT, 1000);
        e.set_global(SAVE_WAIT_DEADLINE, 4000u32);
        set_flags(&mut e, menu, FLAG_SAVE_WAIT_SAVING);
        // the chosen entry accepts: saving starts
        e.set(menu, StartMenu::pSaveGame, Ptr::new(0xe5));
        stub(&mut e, FILE_ENTRY_SAVE_GAME, 1);
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(calls(&e, FILE_ENTRY_SAVE_GAME), vec![vec![0xe5]]);
        assert!(calls(&e, SAVE_LOAD_MANAGER_SAVE_GAME).is_empty());
        assert_eq!(flags(&e, menu), FLAG_SAVE_WAIT_SAVING | FLAG_SAVE_SAVING);
        // without an entry the manager is asked: (0, -1, 0)
        let manager = e.mem.alloc(4);
        e.set_global(SAVE_LOAD_MANAGER, manager);
        e.set(menu, StartMenu::uiFlags, FLAG_SAVE_WAIT_SAVING);
        e.set(menu, StartMenu::pSaveGame, Ptr::new(0));
        stub(&mut e, SAVE_LOAD_MANAGER_SAVE_GAME, 1);
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(
            calls(&e, SAVE_LOAD_MANAGER_SAVE_GAME),
            vec![vec![manager, 0, u32::MAX, 0]]
        );
        assert_eq!(flags(&e, menu), FLAG_SAVE_WAIT_SAVING | FLAG_SAVE_SAVING);
        // the deadline passes: past a second the deadline is renewed
        stub(&mut e, TICK_COUNT, 5500);
        e.call_log = Some(Vec::new());
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(e.global::<u32>(SAVE_WAIT_DEADLINE), 5500 + 0x3e8);
        assert!(calls(&e, START_MENU_CLOSE).is_empty());
        // shortly after the deadline the menu closes and the flags clear
        e.set_global(SAVE_WAIT_DEADLINE, 5000u32);
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(calls(&e, START_MENU_CLOSE), vec![vec![1]]);
        assert_eq!(flags(&e, menu), 0);
    }

    #[test]
    fn the_save_load_idle_reports_a_save_that_cannot_start() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        stub(&mut e, TICK_COUNT, 1000);
        e.set_global(SAVE_WAIT_DEADLINE, 4000u32);
        set_flags(&mut e, menu, FLAG_SAVE_WAIT_SAVING);
        e.set(menu, StartMenu::pSaveGame, Ptr::new(0xe5));
        assert!(start_menu_save_load_idle(&mut e, menu));
        // an entry that refuses is not retried through the manager
        assert!(calls(&e, SAVE_LOAD_MANAGER_SAVE_GAME).is_empty());
        let messages = calls(&e, SHOW_MESSAGE);
        assert_eq!(
            messages,
            vec![vec![
                SETTING_011D4420 + 4,
                0,
                0,
                FAILED_SAVE_CALLBACK,
                1,
                0,
                0,
                0,
                MESSAGE_ICON_SETTING + 4,
                0
            ]]
        );
        assert_eq!(flags(&e, menu), FLAG_MEM_CARD_REMOVED);
    }

    #[test]
    fn the_save_load_idle_populates_the_list_once_its_fade_ends() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        set_flags(&mut e, menu, FLAG_LOADING_SAVEGAMES);
        stub(&mut e, TILE_HAS_FADE_CONTROL, 1);
        // still fading: falls through to the enabled-list checks
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert!(calls(&e, SAVE_LOAD_MANAGER_REFRESH).is_empty());
        stub(&mut e, TILE_HAS_FADE_CONTROL, 0);
        let manager = e.mem.alloc(4);
        e.set_global(SAVE_LOAD_MANAGER, manager);
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(calls(&e, SAVE_LOAD_MANAGER_REFRESH), vec![vec![manager]]);
        // FinishPrepSaveLoad ran and the flag is cleared
        assert_eq!(calls(&e, SAVE_LOAD_LIST_FILTER).len(), 1);
        assert_eq!(flags(&e, menu), 0);
    }

    #[test]
    fn the_save_load_idle_backs_out_and_scrolls() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let list = list_at(menu, LIST_SAVE_LOAD);
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        set_flags(&mut e, menu, FLAG_SAVELOAD_BACKOUT);
        // the confirm list has no container: back out of the dialog
        e.register_double(LIST_CONTAINER_VALID, move |_, a| {
            ret((a[0] == list.addr()) as u32)
        });
        list_vtable(&mut e, list);
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_SET_ENABLED),
            vec![vec![list.addr(), 1]]
        );
        assert_eq!(flags(&e, menu), 0);
        // an enabled list scrolled past the threshold initialises the newly
        // exposed items and a mouse press clicks the highlighted item
        let menu_vtable = e.mem.alloc(16);
        e.mem.set_u32(menu_vtable + 12, 0x6200_0000);
        e.mem.set_u32(menu.addr(), menu_vtable);
        e.register(0x6200_0000, |_, _| Ret::default());
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        stub(&mut e, SAVE_LOAD_LIST_IS_ENABLED, 1);
        stub(&mut e, LIST_SCROLL_BAR, 0x5300);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        e.register_double(TILE_GET_FLOAT, |_, a| {
            float_ret(if a[0] == 0x5300 { 5.0 } else { 0.0 })
        });
        e.set(menu, StartMenu::fSaveLoadThreshold, 2.0);
        stub(&mut e, CONTROLS_GET, 0x2222);
        stub(&mut e, LIST_ITEM_COUNT, 10);
        e.register_double(MOUSE_BUTTON_STATE, |_, a| ret((a[1] == 1) as u32));
        stub(&mut e, LIST_SELECTED_DATA, 0xe1);
        e.call_log = Some(Vec::new());
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(
            calls(&e, LIST_FOR_RANGE),
            vec![vec![list.addr(), SAVE_LOAD_ITEM_INIT_FN, 12, 15]]
        );
        assert_eq!(e.get(menu, StartMenu::fSaveLoadThreshold), 5.0);
        assert_eq!(calls(&e, MOUSE_BUTTON_STATE), vec![vec![0x2222, 1, 2]]);
        assert_eq!(calls(&e, 0x6200_0000), vec![vec![menu.addr(), 4, 0]]);
    }

    #[test]
    fn the_save_load_idle_refreshes_the_screenshot_after_half_a_second() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        stub(&mut e, INTERFACE_ACTIVE, 1);
        let timer = e.mem.alloc(8);
        e.mem.set_u32(timer, 1000);
        e.mem.set_u32(timer + 4, 1499);
        stub(&mut e, IDLE_TIMER_GET, timer);
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert!(calls(&e, IDLE_TIMER_RESET).is_empty());
        e.mem.set_u32(timer + 4, 1500);
        // no screenshot: only the timer is reset
        assert!(start_menu_save_load_idle(&mut e, menu));
        assert_eq!(calls(&e, IDLE_TIMER_RESET), vec![vec![timer]]);
        assert!(calls(&e, TILE_ADD_FADE_CONTROL).is_empty());
        // a screenshot fades its tile in with the 0xfa9 trait
        stub(&mut e, LIST_SELECTED_DATA, 0xe1);
        let image = e.mem.alloc(8);
        let slots: Vec<u32> = (0..40).map(|i| 0x6100_1000 + 4 * i).collect();
        e.put_vtable(0x6100_0000, &slots);
        e.mem.set_u32(image, 0x6100_0000);
        stub(&mut e, FILE_ENTRY_GET_SCREENSHOT, image);
        for slot in &slots {
            e.register(*slot, |_, _| Ret::default());
        }
        e.call_log = Some(Vec::new());
        assert!(start_menu_save_load_idle(&mut e, menu));
        let fades = calls(&e, TILE_ADD_FADE_CONTROL);
        assert_eq!(fades.len(), 1);
        assert_eq!(fades[0][..2], [tile(11), 0x0fa9]);
    }

    #[test]
    fn idle_timer_elapsed_time() {
        let mut e = engine();
        let timer = Ptr::<()>::new(e.mem.alloc(8));
        stub(&mut e, TICK_COUNT, 5000);
        // never started
        assert!(!fn_007d5f10(&mut e, timer));
        assert!(!fn_007d5f30(&mut e, timer));
        assert_eq!(fn_007d5eb0(&mut e, timer), 0);
        // running: the time since the start
        e.mem.set_u32(timer.addr(), 1200);
        assert!(fn_007d5f10(&mut e, timer));
        assert_eq!(fn_007d5eb0(&mut e, timer), 3800);
        // stopped: the span between the two ticks
        e.mem.set_u32(timer.addr() + 4, 1700);
        assert!(fn_007d5f30(&mut e, timer));
        assert_eq!(fn_007d5eb0(&mut e, timer), 500);
    }

    #[test]
    fn the_failed_save_callback_closes_the_message_and_goes_back() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_MEM_CARD_REMOVED);
        start_menu_failed_save_callback_function(&mut e);
        assert_eq!(flags(&e, menu), 0);
        assert_eq!(
            order(
                &e,
                &[
                    MESSAGE_MENU_CLOSE_007AA480,
                    CLOSE_MESSAGE_MENU,
                    START_MENU_CHOOSE_BACK
                ]
            ),
            vec![
                MESSAGE_MENU_CLOSE_007AA480,
                CLOSE_MESSAGE_MENU,
                START_MENU_CHOOSE_BACK
            ]
        );
    }

    #[test]
    fn hiding_the_options_container_resets_the_remap_ui_in_action_mapping() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let options = list_of(menu, 0x114);
        start_menu_hide_options_container(&mut e, menu);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(10), 0x0fa3, 0]));
        assert_eq!(calls(&e, OPTION_LIST_SET_ENABLED), vec![vec![options, 0]]);
        // the options container fades out: its alpha is 0, so nothing starts
        assert!(calls(&e, TILE_ADD_FADE_CONTROL).is_empty());
        assert!(calls(&e, LIST_FOR_RANGE).is_empty());
        set_flags(&mut e, menu, FLAG_ACTION_MAPPING);
        stub_float(&mut e, TILE_GET_FLOAT, 1.0);
        e.call_log = Some(Vec::new());
        start_menu_hide_options_container(&mut e, menu);
        let fades = calls(&e, TILE_ADD_FADE_CONTROL);
        assert_eq!(fades.len(), 1);
        assert_eq!(fades[0][0], tile(2));
        assert_eq!(
            calls(&e, LIST_FOR_RANGE),
            vec![vec![options, ACTION_MAPPING_ITEM_FN, 0, 0x7fff_ffff]]
        );
        assert_eq!(flags(&e, menu), 0);
        let ints = calls(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![tile(15), 0x1004, 0]));
        assert!(ints.contains(&vec![tile(16), 0x0fa3, 0]));
    }

    #[test]
    fn the_device_selector_must_name_a_usable_device() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        e.set_global(CONTROLS_OWNER, 0x1111u32);
        let controls_object = e.mem.alloc(0x1900);
        e.mem.set_u32(controls_object + 0x18f4, 1);
        stub(&mut e, CONTROLS_GET, controls_object);
        e.set(menu, StartMenu::field_1c8, 0xff);
        assert!(!fn_007d6030(&mut e, menu));
        // with the interface active only selector 3 qualifies
        stub(&mut e, INTERFACE_ACTIVE, 1);
        e.set(menu, StartMenu::field_1c8, 3);
        assert!(fn_007d6030(&mut e, menu));
        e.set(menu, StartMenu::field_1c8, 1);
        assert!(!fn_007d6030(&mut e, menu));
        stub(&mut e, INTERFACE_ACTIVE, 0);
        // otherwise selector 3 never qualifies and the others are asked
        e.set(menu, StartMenu::field_1c8, 3);
        assert!(!fn_007d6030(&mut e, menu));
        e.set(menu, StartMenu::field_1c8, 1);
        assert!(!fn_007d6030(&mut e, menu));
        e.set(menu, StartMenu::field_1c8, 2);
        assert!(fn_007d6030(&mut e, menu));
        e.set(menu, StartMenu::field_1c8, 0);
        assert!(!fn_007d6030(&mut e, menu));
        e.mem.set_u32(controls_object + 0x2c, 9);
        assert!(fn_007d6030(&mut e, menu));
    }

    #[test]
    fn the_per_device_tests_read_the_controls_object() {
        let mut e = engine();
        let controls_object = Ptr::<()>::new(e.mem.alloc(0x1900));
        assert_eq!(fn_007d60b0(&mut e, controls_object, 0), 0);
        assert_eq!(fn_007d60b0(&mut e, controls_object, 1), 0);
        assert_eq!(fn_007d60b0(&mut e, controls_object, 2), 0);
        e.mem.set_u32(controls_object.addr() + 0x2c, 5);
        e.mem.set_u32(controls_object.addr() + 0x30, 6);
        e.mem.set_u32(controls_object.addr() + 0x18f4, 7);
        assert_eq!(fn_007d6120(&mut e, controls_object), 1);
        assert_eq!(fn_007d6140(&mut e, controls_object), 1);
        assert_eq!(fn_007d6100(&mut e, controls_object), 7);
        assert_eq!(fn_007d60b0(&mut e, controls_object, 0), 1);
        assert_eq!(fn_007d60b0(&mut e, controls_object, 1), 1);
        assert_eq!(fn_007d60b0(&mut e, controls_object, 2), 7);
        assert_eq!(fn_007d60b0(&mut e, controls_object, 3), 0);
        assert_eq!(fn_007d60b0(&mut e, controls_object, -1), 0);
    }

    #[test]
    fn the_selector_advances_and_wraps_at_four() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let selector = menu.byte_add(0x1c8);
        e.set(menu, StartMenu::field_1c8, 2);
        assert_eq!(fn_007d61b0(&mut e, selector), selector);
        assert_eq!(e.get(menu, StartMenu::field_1c8), 3);
        fn_007d61b0(&mut e, selector);
        assert_eq!(e.get(menu, StartMenu::field_1c8), 0);
        // 007d6160 steps until a usable device is found: only selector 2 is
        let controls_object = e.mem.alloc(0x1900);
        e.set_global(CONTROLS_OWNER, 0x1111u32);
        stub(&mut e, CONTROLS_GET, controls_object);
        e.mem.set_u32(controls_object + 0x18f4, 1);
        fn_007d6160(&mut e, menu);
        assert_eq!(e.get(menu, StartMenu::field_1c8), 2);
        // no usable device: four steps bring the selector back
        e.mem.set_u32(controls_object + 0x18f4, 0);
        e.set(menu, StartMenu::field_1c8, 1);
        fn_007d6160(&mut e, menu);
        assert_eq!(e.get(menu, StartMenu::field_1c8), 1);
    }

    #[test]
    fn fading_in_adds_a_fade_control_only_below_the_full_value() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        // the default trait is _alpha, the rate 3.5 as given
        start_menu_fade(&mut e, menu, 6, true, TRAIT_DEFAULT, 3.5);
        assert_eq!(
            calls(&e, TILE_ADD_USER_TRAIT),
            vec![vec![TRAIT_NAME_ALPHA, u32::MAX]]
        );
        assert_eq!(
            calls(&e, TILE_ADD_FADE_CONTROL),
            vec![vec![
                tile(6),
                0xabc,
                0,
                1.5f32.to_bits(),
                3.5f32.to_bits(),
                0
            ]]
        );
        // already at the full value: nothing
        e.call_log = Some(Vec::new());
        stub_float(&mut e, TILE_GET_FLOAT, 1.0);
        start_menu_fade(&mut e, menu, 6, true, 0x0fa9, 3.5);
        assert!(calls(&e, TILE_ADD_FADE_CONTROL).is_empty());
        // a negative rate becomes the default one
        stub_float(&mut e, TILE_GET_FLOAT, 0.25);
        start_menu_fade(&mut e, menu, 6, true, 0x0fa9, -1.0);
        assert_eq!(
            calls(&e, TILE_ADD_FADE_CONTROL),
            vec![vec![
                tile(6),
                0x0fa9,
                0,
                1.5f32.to_bits(),
                2.5f32.to_bits(),
                0
            ]]
        );
    }

    #[test]
    fn fading_out_adds_a_fade_control_only_above_zero() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub_float(&mut e, TILE_GET_FLOAT, 0.5);
        start_menu_fade(&mut e, menu, 2, false, 0x0fa9, 3.5);
        assert_eq!(
            calls(&e, TILE_ADD_FADE_CONTROL),
            vec![vec![
                tile(2),
                0x0fa9,
                1.5f32.to_bits(),
                0,
                3.5f32.to_bits(),
                0
            ]]
        );
        e.call_log = Some(Vec::new());
        stub_float(&mut e, TILE_GET_FLOAT, 0.0);
        start_menu_fade(&mut e, menu, 2, false, 0x0fa9, 3.5);
        assert!(calls(&e, TILE_ADD_FADE_CONTROL).is_empty());
        // NaN is neither above zero nor below one
        stub_float(&mut e, TILE_GET_FLOAT, f64::NAN);
        start_menu_fade(&mut e, menu, 2, false, 0x0fa9, 3.5);
        start_menu_fade(&mut e, menu, 2, true, 0x0fa9, 3.5);
        assert!(calls(&e, TILE_ADD_FADE_CONTROL).is_empty());
        // a tile that does not exist
        e.mem.set_u32(menu.addr() + TILES + 8, 0);
        stub_float(&mut e, TILE_GET_FLOAT, 0.5);
        start_menu_fade(&mut e, menu, 2, false, 0x0fa9, 3.5);
        assert!(calls(&e, TILE_ADD_FADE_CONTROL).is_empty());
    }

    #[test]
    fn leaving_the_confirm_dialog_re_enables_the_list() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let list = list_at(menu, LIST_SAVE_LOAD);
        list_vtable(&mut e, list);
        stub(&mut e, slot_fn(0x04), 0x5200);
        set_flags(&mut e, menu, FLAG_SAVELOAD_BACKOUT);
        fn_007d62e0(&mut e, menu);
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_SET_ENABLED),
            vec![vec![list.addr(), 1]]
        );
        assert_eq!(
            calls(&e, SAVE_LOAD_LIST_SELECT_HIGHLIGHT),
            vec![vec![list.addr(), 0x5200]]
        );
        assert!(calls(&e, TILE_SET_INT).contains(&vec![tile(6), 0x1004, 1]));
        assert_eq!(flags(&e, menu), 0);
        // with the interface active the back-out flag stays
        set_flags(&mut e, menu, FLAG_SAVELOAD_BACKOUT);
        stub(&mut e, INTERFACE_ACTIVE, 1);
        fn_007d62e0(&mut e, menu);
        assert_eq!(flags(&e, menu), FLAG_SAVELOAD_BACKOUT);
    }

    #[test]
    fn a_toggle_user_option_copies_its_value_strings() {
        let mut e = engine();
        let option = e.new_object::<StartMenuUserOption>();
        e.register_double(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let result = start_menu_user_option_user_option(
            &mut e,
            option,
            0x9100,
            0xc0de,
            0x8,
            1,
            2,
            5,
            &[0xa1, 0xa2],
        );
        assert_eq!(result, option);
        assert_eq!(
            calls(&e, OPTION_CONSTRUCT),
            vec![vec![option.addr(), 0x9100, 0, 8]]
        );
        assert_eq!(e.mem.u32(option.addr()), USER_OPTION_VTABLE);
        assert_eq!(e.get(option, StartMenuUserOption::iVal), 0);
        assert_eq!(e.get(option, StartMenuUserOption::iDefaultVal), 0);
        assert_eq!(e.get(option, StartMenuUserOption::eType), 1);
        assert_eq!(e.get(option, StartMenuUserOption::iNumSettings), 2);
        assert_eq!(e.get(option, StartMenuUserOption::iArrowSpacing), 5);
        assert_eq!(e.get(option, StartMenuUserOption::pfChangeFn), 0xc0de);
        assert_eq!(
            e.get(option, StartMenuUserOption::pTemplate),
            USER_OPTION_TOGGLE_TEMPLATE
        );
        // a new array of two words holds the strings
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        let strings = e.get(option, StartMenuUserOption::pValueStrings);
        assert_eq!((e.mem.u32(strings), e.mem.u32(strings + 4)), (0xa1, 0xa2));
    }

    #[test]
    fn a_meter_user_option_has_no_value_strings() {
        let mut e = engine();
        let option = e.new_object::<StartMenuUserOption>();
        e.set(option, StartMenuUserOption::pValueStrings, 0x1234);
        start_menu_user_option_user_option(&mut e, option, 0x9100, 0, 1, 2, 10, 0, &[]);
        assert_eq!(
            e.get(option, StartMenuUserOption::pTemplate),
            USER_OPTION_METER_TEMPLATE
        );
        assert_eq!(e.get(option, StartMenuUserOption::pValueStrings), 0);
        assert_eq!(e.get(option, StartMenuUserOption::iNumSettings), 10);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn the_user_option_constructor_is_registered_with_trailing_strings() {
        let mut e = engine();
        e.register_double(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let option = e.new_object::<StartMenuUserOption>();
        let words =
            args![option, 0x9100u32, 0xc0deu32, 8u32, 0i32, 3u32, 0u32, 0xb1u32, 0xb2u32, 0xb3u32];
        let result = e.call(0x007d_6350, &words);
        assert_eq!(result.u32(), option.addr());
        assert_eq!(e.get(option, StartMenuUserOption::eType), 0);
        let strings = e.get(option, StartMenuUserOption::pValueStrings);
        assert_eq!(
            [
                e.mem.u32(strings),
                e.mem.u32(strings + 4),
                e.mem.u32(strings + 8)
            ],
            [0xb1, 0xb2, 0xb3]
        );
    }

    #[test]
    fn the_user_option_destructor_frees_its_strings_and_optionally_itself() {
        let mut e = engine();
        let option = e.new_object::<StartMenuUserOption>();
        e.set(option, StartMenuUserOption::pValueStrings, 0x7700);
        let returned = start_menu_user_option_scalar_deleting_destructor(&mut e, option, 0);
        assert_eq!(returned, option);
        assert_eq!(e.mem.u32(option.addr()), USER_OPTION_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x7700]]);
        assert_eq!(calls(&e, OPTION_DESTRUCT), vec![vec![option.addr()]]);
        e.call_log = Some(Vec::new());
        start_menu_user_option_scalar_deleting_destructor(&mut e, option, 1);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![0x7700], vec![option.addr()]]
        );
        assert_eq!(
            order(&e, &[OPERATOR_DELETE, OPTION_DESTRUCT]),
            vec![OPERATOR_DELETE, OPTION_DESTRUCT, OPERATOR_DELETE]
        );
    }

    #[test]
    fn the_option_list_has_no_target_for_two_directions() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        stub(&mut e, OPTION_LIST_GET_HIGHLIGHT_TARGET, 0x77);
        assert_eq!(
            start_menu_option_list_box_get_highlight_target(&mut e, list, 0xf),
            0
        );
        assert_eq!(
            start_menu_option_list_box_get_highlight_target(&mut e, list, 0x10),
            0
        );
        assert!(calls(&e, OPTION_LIST_GET_HIGHLIGHT_TARGET).is_empty());
        assert_eq!(
            start_menu_option_list_box_get_highlight_target(&mut e, list, 5),
            0x77
        );
        assert_eq!(
            calls(&e, OPTION_LIST_GET_HIGHLIGHT_TARGET),
            vec![vec![list.addr(), 5]]
        );
    }

    #[test]
    fn rebuilding_the_menu_marks_the_dlc_state_and_shows_the_main_list() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_OPTIONS_DIRTY);
        list_vtable(&mut e, list_at(menu, LIST_SAVE_LOAD));
        stub(&mut e, NETWORK_SYSTEM_UTILITY_INSTANCE, 0x4000);
        stub(&mut e, FIRST_BYTE, 1);
        fn_007d6550(&mut e, menu);
        assert_eq!(calls(&e, slot_fn(0x1c)).len(), 1);
        assert_eq!(calls(&e, FIRST_BYTE), vec![vec![0x4000]]);
        assert_eq!(e.mem.u8(FLAG_BYTE_PENDING), 1);
        assert_eq!(calls(&e, UPDATE_CAN_LOAD_STATUS), vec![vec![menu.addr()]]);
        assert_eq!(
            flags(&e, menu),
            FLAG_DLC_WAIT_MESSAGE | FLAG_DLC_NEEDS_UPDATE | FLAG_SAVE_OPTIONS
        );
        // the main list fades in and is enabled
        let main_list = list_of(menu, 0x84);
        assert_eq!(calls(&e, OPTION_LIST_SET_ENABLED), vec![vec![main_list, 1]]);
        assert_eq!(calls(&e, TILE_ADD_FADE_CONTROL)[0][0], tile(0));
        assert_eq!(e.mem.u8(FLAG_BYTE_MAIN_LIST), 0);
    }

    #[test]
    fn rebuilding_the_menu_leaves_credits_and_force_back_alone() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        // credits showing: the rebuild is skipped, the credits are dismissed
        set_flags(&mut e, menu, FLAG_SHOWING_CREDITS);
        fn_007d6550(&mut e, menu);
        assert!(calls(&e, UPDATE_CAN_LOAD_STATUS).is_empty());
        let main_list = list_of(menu, 0x84);
        assert_eq!(
            calls(&e, OPTION_LIST_DESELECT_ALL),
            vec![vec![main_list, 1]]
        );
        assert_eq!(flags(&e, menu), 0);
        // "don't force back" is just cleared
        set_flags(&mut e, menu, FLAG_DONT_FORCE_BACK | FLAG_SHOWING_CREDITS);
        e.call_log = Some(Vec::new());
        fn_007d6550(&mut e, menu);
        assert_eq!(flags(&e, menu), FLAG_SHOWING_CREDITS);
        assert!(calls(&e, OPTION_LIST_SET_ENABLED).is_empty());
        // a positive global keeps the main list as it is
        e.set(menu, StartMenu::uiFlags, FLAG_SHOWING_CREDITS);
        e.set_global(TES_GLOBAL_POINTER, 0x5000u32);
        stub_float(&mut e, TES_GLOBAL_GET_VALUE, 3.0);
        e.call_log = Some(Vec::new());
        fn_007d6550(&mut e, menu);
        assert!(calls(&e, OPTION_LIST_SET_ENABLED).is_empty());
        assert!(calls(&e, OPTION_LIST_DESELECT_ALL).is_empty());
        assert_eq!(flags(&e, menu), FLAG_SHOWING_CREDITS);
    }

    #[test]
    fn the_network_utility_accessor_ignores_its_this() {
        let mut e = engine();
        stub(&mut e, NETWORK_SYSTEM_UTILITY_INSTANCE, 0x4abc);
        assert_eq!(fn_007d6720(&mut e, Ptr::new(0x1234)), 0x4abc);
    }

    #[test]
    fn backing_out_runs_until_the_main_list_is_enabled() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        list_vtable(&mut e, list_at(menu, LIST_SAVE_LOAD));
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        let enabled_checks = Rc::new(std::cell::Cell::new(0));
        let counter = Rc::clone(&enabled_checks);
        e.register_double(OPTION_LIST_IS_ENABLED, move |_, _| {
            counter.set(counter.get() + 1);
            ret((counter.get() > 2) as u32)
        });
        let menu_addr = menu.addr();
        let mut backs = 0;
        e.register_double(START_MENU_CHOOSE_BACK, move |e, _| {
            backs += 1;
            if backs == 1 {
                e.mem.set_u32(menu_addr + 0x1a8, FLAG_SAVELOAD_BACKOUT);
            }
            Ret::default()
        });
        fn_007d6730(&mut e, menu);
        assert_eq!(calls(&e, START_MENU_CHOOSE_BACK).len(), 2);
        // the first back set the back-out flag: the dialog was left and the
        // flag cleared
        assert_eq!(calls(&e, SAVE_LOAD_LIST_SET_ENABLED).len(), 1);
        assert_eq!(flags(&e, menu), 0);
        // nothing happens without a container, or with "don't force back"
        e.call_log = Some(Vec::new());
        stub(&mut e, LIST_CONTAINER_VALID, 0);
        fn_007d6730(&mut e, menu);
        stub(&mut e, LIST_CONTAINER_VALID, 1);
        set_flags(&mut e, menu, FLAG_DONT_FORCE_BACK);
        fn_007d6730(&mut e, menu);
        assert!(calls(&e, START_MENU_CHOOSE_BACK).is_empty());
    }

    #[test]
    fn list_items_get_their_texts_from_the_entry() {
        let mut e = engine();
        let entry = save_entry(&mut e);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        stub(&mut e, FIRST_WORD, 0x9100);
        let tile = Ptr::<()>::new(0x5000);
        let entry_ptr = Ptr::<()>::new(entry);
        // an autosave
        stub(&mut e, FILE_ENTRY_IS_AUTOSAVE, 1);
        start_menu_initialize_save_load_list_item(&mut e, tile, entry_ptr);
        assert_eq!(calls(&e, TILE_IS_TRUE), vec![vec![0x5000, 0xabc]]);
        let strings = calls(&e, TILE_SET_STRING);
        assert_eq!(
            strings,
            vec![
                vec![0x5000, 0x1004, SETTING_011D3F10 + 4, 1],
                vec![0x5000, 0x1005, 0x9004, 1]
            ]
        );
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x5000, 0xabc, 1]]);
        // the two other special kinds
        stub(&mut e, FILE_ENTRY_IS_AUTOSAVE, 0);
        stub(&mut e, FILE_ENTRY_NAME_TEST_AD0, 1);
        e.call_log = Some(Vec::new());
        start_menu_initialize_save_load_list_item(&mut e, tile, entry_ptr);
        assert_eq!(
            calls(&e, TILE_SET_STRING)[0],
            vec![0x5000, 0x1004, SETTING_011D4E1C + 4, 1]
        );
        stub(&mut e, FILE_ENTRY_NAME_TEST_AD0, 0);
        stub(&mut e, FILE_ENTRY_NAME_TEST_B10, 1);
        e.call_log = Some(Vec::new());
        start_menu_initialize_save_load_list_item(&mut e, tile, entry_ptr);
        assert_eq!(
            calls(&e, TILE_SET_STRING)[0],
            vec![0x5000, 0x1004, SETTING_011D4E04 + 4, 1]
        );
        // a numbered save: "#%04d" with the save number
        stub(&mut e, FILE_ENTRY_NAME_TEST_B10, 0);
        e.call_log = Some(Vec::new());
        let frame = next_frame(&mut e, 0x1c);
        start_menu_initialize_save_load_list_item(&mut e, tile, entry_ptr);
        // The 0x1c-byte frame: the save number text at +8.
        assert_eq!(calls(&e, STRING_CONSTRUCT)[0][0], frame + 8);
        let formats = calls(&e, STRING_FORMAT);
        assert_eq!(formats.len(), 1);
        assert_eq!(formats[0][1..], [FORMAT_SAVE_NUMBER, 42]);
        assert_eq!(
            calls(&e, TILE_SET_STRING)[0],
            vec![0x5000, 0x1004, 0x9100, 1]
        );
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![formats[0][0]]]);
        // a corrupt entry shows the corrupt location text
        stub(&mut e, FILE_ENTRY_IS_CORRUPT, 1);
        e.call_log = Some(Vec::new());
        start_menu_initialize_save_load_list_item(&mut e, tile, entry_ptr);
        assert_eq!(
            calls(&e, TILE_SET_STRING)[1],
            vec![0x5000, 0x1005, SETTING_011D46E4 + 4, 1]
        );
    }

    #[test]
    fn list_items_are_initialised_only_once() {
        let mut e = engine();
        let entry = Ptr::<()>::new(save_entry(&mut e));
        stub(&mut e, TILE_IS_TRUE, 1);
        start_menu_initialize_save_load_list_item(&mut e, Ptr::new(0x5000), entry);
        assert!(calls(&e, TILE_SET_STRING).is_empty());
        assert!(calls(&e, TILE_SET_INT).is_empty());
        stub(&mut e, TILE_IS_TRUE, 0);
        start_menu_initialize_save_load_list_item(&mut e, Ptr::NULL, entry);
        start_menu_initialize_save_load_list_item(&mut e, Ptr::new(0x5000), Ptr::NULL);
        assert!(calls(&e, TILE_SET_STRING).is_empty());
        assert_eq!(calls(&e, TILE_IS_TRUE).len(), 1);
    }

    #[test]
    fn small_start_menu_helpers_set_and_query_flags() {
        let mut e = engine();
        // no start menu: nothing happens
        fn_007d69d0(&mut e);
        fn_007d6990(&mut e, 0);
        fn_007d6c30(&mut e);
        assert!(calls(&e, MENU_SET_FLAG).is_empty());
        assert!(calls(&e, MENU_GET_FLAG).is_empty());
        let menu = new_menu(&mut e);
        fn_007d69d0(&mut e);
        assert_eq!(flags(&e, menu), FLAG_SYSUTILSAVECOMPLETE);
        // 007d6990 queries flag 1 when the downloads option exists
        fn_007d6990(&mut e, 0);
        assert!(calls(&e, MENU_GET_FLAG).is_empty());
        e.set(menu, StartMenu::pDownloadsOption, Ptr::new(0x5000));
        fn_007d6990(&mut e, 5);
        assert!(calls(&e, MENU_GET_FLAG).is_empty());
        fn_007d6990(&mut e, 0);
        assert_eq!(calls(&e, MENU_GET_FLAG), vec![vec![menu.addr(), 1]]);
        // the address 0x14 into an object
        assert_eq!(fn_007d6bb0(&mut e, Ptr::new(0x1000)), Ptr::new(0x1014));
    }

    #[test]
    fn the_form_flag_helper_sets_or_clears_bit_zero() {
        let mut e = engine();
        let form = Ptr::<()>::new(e.mem.alloc(0x250));
        stub(&mut e, FORM_FN_570F40, 1);
        assert!(fn_007d6bd0(&mut e, form, false));
        assert_eq!(e.mem.u32(form.addr() + 0x244), 1);
        e.mem.set_u32(form.addr() + 0x244, 0x81);
        assert!(fn_007d6bd0(&mut e, form, true));
        assert_eq!(e.mem.u32(form.addr() + 0x244), 0x80);
        stub(&mut e, FORM_FN_570F40, 0);
        assert!(!fn_007d6bd0(&mut e, form, true));
        assert_eq!(calls(&e, FORM_FN_570F40).len(), 3);
    }

    #[test]
    fn the_menu_tile_follows_the_interface_state() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        stub(&mut e, WORD_AT_4, 0x5900);
        stub(&mut e, INTERFACE_ACTIVE, 1);
        fn_007d6c30(&mut e);
        assert_eq!(calls(&e, WORD_AT_4), vec![vec![menu.addr()]]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x5900, 0x1009, 1]]);
        assert_eq!(calls(&e, START_MENU_FN_7D30B0), vec![vec![1]]);
        assert!(calls(&e, START_MENU_CHOOSE_BACK).is_empty());
        // in action-mapping mode it also goes back
        set_flags(&mut e, menu, FLAG_ACTION_MAPPING);
        fn_007d6c30(&mut e);
        assert_eq!(calls(&e, START_MENU_CHOOSE_BACK).len(), 1);
    }

    #[test]
    fn rebuilding_the_downloads_list_reads_the_dlc_utility() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let downloads = list_at(menu, LIST_DOWNLOADS);
        list_vtable(&mut e, downloads);
        stub(&mut e, SYSTEM_UTILITY_INSTANCE, 0x4100);
        stub(&mut e, DLC_UTILITY_FROM_SYSTEM, 0x4200);
        stub(&mut e, LIST_FIRST_NODE, 0);
        fn_007d69f0(&mut e);
        assert_eq!(
            calls(&e, DLC_UTILITY_FN_AF2940),
            vec![vec![0x4200, 0, u32::MAX]]
        );
        assert_eq!(calls(&e, DLC_UTILITY_FN_AF2B10), vec![vec![0x4200, 0, 0]]);
        // the content list at +0x14 is not empty, so the corrupt flag is set
        assert_eq!(calls(&e, LIST_IS_EMPTY_NODE), vec![vec![0x4214]]);
        assert_eq!(flags(&e, menu), FLAG_DLC_CORRUPT_MESSAGE);
        assert_eq!(calls(&e, slot_fn(0x1c)).len(), 1);
        assert_eq!(
            calls(&e, DOWNLOADS_LIST_ADD_ITEM),
            vec![vec![downloads.addr(), u32::MAX, SETTING_011D3AD4 + 4, 0, 0]]
        );
        // an empty list leaves the flag alone
        stub(&mut e, LIST_IS_EMPTY_NODE, 1);
        e.set(menu, StartMenu::uiFlags, 0);
        fn_007d69f0(&mut e);
        assert_eq!(flags(&e, menu), 0);
    }

    #[test]
    fn rebuilding_the_downloads_list_adds_every_named_content_item() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        let downloads = list_at(menu, LIST_DOWNLOADS);
        list_vtable(&mut e, downloads);
        stub(&mut e, DLC_UTILITY_FROM_SYSTEM, 0x4200);
        stub(&mut e, LIST_IS_EMPTY_NODE, 1);
        // nodes {name, next}: one named, one without a name
        let (node2, node1) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(node1, 0x7001);
        e.mem.set_u32(node1 + 4, node2);
        stub(&mut e, LIST_FIRST_NODE, node1);
        e.register(LIST_NODE_SELF, |_, a| ret(a[0]));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(STRING_CHARACTERS, |_, a| ret(a[0] + 0x10));
        e.register(WCSTOMBS, |e, a| {
            e.mem.set_cstr(a[0], b"pack");
            ret(4)
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen);
        e.register_double(DOWNLOADS_LIST_ADD_ITEM, move |e, a| {
            record.borrow_mut().push((a[1], e.mem.cstr(a[2])));
            Ret::default()
        });
        fn_007d69f0(&mut e);
        assert_eq!(
            *seen.borrow(),
            vec![
                (u32::MAX, e.mem.cstr(SETTING_011D3AD4 + 4)),
                (0, b"pack".to_vec())
            ]
        );
        assert_eq!(calls(&e, WCSTOMBS)[0][1..], [0x7011, 0x104]);
    }

    #[test]
    fn rebuilding_the_downloads_list_reloads_a_loaded_game_data() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        list_vtable(&mut e, list_at(menu, LIST_DOWNLOADS));
        let form = e.mem.alloc(0x250);
        e.set_global(FLAGGED_FORM_OBJECT, form);
        e.set_global(DATA_HANDLER_OBJECT, 0x4400u32);
        stub(&mut e, DATA_HANDLER_IS_LOADED, 1);
        stub(&mut e, FORM_FN_570F40, 1);
        fn_007d69f0(&mut e);
        assert_eq!(
            order(
                &e,
                &[
                    FORM_FN_570F40,
                    DATA_HANDLER_FN_463070,
                    DATA_HANDLER_FN_465040,
                    PROCESS_LISTS_FN_970D50,
                    DATA_HANDLER_FN_4614E0
                ]
            ),
            vec![
                FORM_FN_570F40,
                DATA_HANDLER_FN_463070,
                DATA_HANDLER_FN_465040,
                FORM_FN_570F40,
                PROCESS_LISTS_FN_970D50,
                DATA_HANDLER_FN_4614E0
            ]
        );
        assert_eq!(calls(&e, DATA_HANDLER_FN_463070), vec![vec![0x4400, 1]]);
        assert_eq!(
            calls(&e, PROCESS_LISTS_FN_970D50),
            vec![vec![PROCESS_LISTS_OBJECT]]
        );
        // the form's flag bit is back where it was
        assert_eq!(e.mem.u32(form + 0x244), 0);
    }

    #[test]
    fn ending_the_credits_restarts_the_title_music() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        e.set_global(TES_GLOBAL_POINTER, 0x5000u32);
        set_flags(&mut e, menu, FLAG_SHOWING_CREDITS | FLAG_INGAME);
        start_menu_end_credits(&mut e, menu);
        assert!(calls(&e, MEDIA_IS_PLAYING).is_empty());
        e.set(menu, StartMenu::uiFlags, FLAG_SHOWING_CREDITS);
        stub(&mut e, MEDIA_IS_PLAYING, 1);
        start_menu_end_credits(&mut e, menu);
        assert_eq!(calls(&e, MEDIA_IS_PLAYING), vec![vec![8, 1]]);
        let path = calls(&e, MEMSET)[0][0];
        assert_eq!(calls(&e, MEMSET), vec![vec![path, 0, 0x40]]);
        assert_eq!(
            calls(&e, SPRINTF_S),
            vec![vec![path, 0x40, FORMAT_CREDITS_MEDIA]]
        );
        assert_eq!(calls(&e, MEDIA_FN_761460), vec![vec![1]]);
        assert_eq!(
            calls(&e, MEDIA_PLAY),
            vec![vec![8, path, 0x3e8, 1, 1, 0, 0]]
        );
        assert_eq!(calls(&e, MEDIA_FN_8304C0).len(), 1);
        assert_eq!(flags(&e, menu), FLAG_FADE_IN);
        assert_eq!(e.mem.u8(FLAG_BYTE_MAIN_LIST), 1);
        assert_eq!(calls(&e, TES_GLOBAL_SET_VALUE), vec![vec![0x5000, 0]]);
        assert_eq!(calls(&e, LOADING_MENU_SUSPEND_BACKGROUND_THREAD).len(), 1);
        // music already stopped: only the state changes
        stub(&mut e, MEDIA_IS_PLAYING, 0);
        e.call_log = Some(Vec::new());
        start_menu_end_credits(&mut e, menu);
        assert!(calls(&e, MEDIA_PLAY).is_empty());
        assert_eq!(calls(&e, TES_GLOBAL_SET_VALUE).len(), 1);
    }

    #[test]
    fn the_dlc_mount_result_message_depends_on_the_second_check() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        set_flags(&mut e, menu, FLAG_RESTART_REQUIRED);
        stub(&mut e, SINGLETON_0045D180, 0x4001);
        stub(&mut e, SINGLETON_004DE490, 0x4002);
        e.register(OBJECT_PLUS_4, |_, a| ret(a[0] + 4));
        e.register(OBJECT_TEST_005E01B0, |_, a| ret((a[0] == 0x4002) as u32));
        e.set_global(CONTROLS_OWNER, 0x1111u32);
        stub(&mut e, CONTROLS_GET, 0x2222);
        fn_007d6d70(&mut e);
        assert_eq!(
            calls(&e, OBJECT_TEST_005E01B0),
            vec![vec![0x4001, 0x4005], vec![0x4002, 0x4006]]
        );
        assert_eq!(calls(&e, CONTROLS_FN_A22E10), vec![vec![0x2222]]);
        assert_eq!(
            calls(&e, SHOW_MESSAGE),
            vec![vec![
                SETTING_011D493C + 4,
                0,
                0,
                0,
                0,
                0x12,
                0,
                0,
                MESSAGE_ICON_SETTING + 4,
                0
            ]]
        );
        assert_eq!(flags(&e, menu), 0);
        // without the restart flag nothing is shown
        e.call_log = Some(Vec::new());
        fn_007d6d70(&mut e);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        // a failed second check shows the other message
        e.register(OBJECT_TEST_005E01B0, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        fn_007d6d70(&mut e);
        let messages = calls(&e, SHOW_MESSAGE);
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0][0], SETTING_011D3138 + 4);
        assert!(calls(&e, CONTROLS_FN_A22E10).is_empty());
    }

    #[test]
    fn the_option_list_constructor_zeroes_its_fields() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        for offset in (4..0x30).step_by(4) {
            e.mem.set_u32(list.addr() + offset, 0xffff_ffff);
        }
        let result = list_box_option_ptr_construct(&mut e, list);
        assert_eq!(result, list);
        assert_eq!(calls(&e, LIST_BOX_BASE_CONSTRUCT), vec![vec![list.addr()]]);
        assert_eq!(
            calls(&e, EMBEDDED_LIST_CONSTRUCT),
            vec![vec![list.addr() + 4]]
        );
        assert_eq!(e.mem.u32(list.addr()), OPTION_LIST_BOX_VTABLE);
        for field in [0x0c, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(list.addr() + field), 0);
        }
        assert_eq!(e.mem.u16(list.addr() + 0x1c), 0);
        assert_eq!(e.mem.u16(list.addr() + 0x2c), 0);
        for field in [0x20, 0x24, 0x28] {
            assert_eq!(e.mem.f32(list.addr() + field), 0.0);
        }
        // the embedded list's head and next are not touched here
        assert_eq!(e.mem.u32(list.addr() + 4), 0xffff_ffff);
        assert_eq!(calls(&e, LIST_BOX_FN_7A1950), vec![vec![list.addr(), 0, 1]]);
        assert_eq!(calls(&e, LIST_BOX_FN_721130), vec![vec![list.addr()]]);
    }

    #[test]
    fn the_option_list_destructor_empties_the_list_first() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        list_box_option_ptr_destruct(&mut e, list);
        assert_eq!(e.mem.u32(list.addr()), OPTION_LIST_BOX_VTABLE);
        assert_eq!(
            order(
                &e,
                &[
                    OPTION_LIST_SET_TOTAL_LIST_HEIGHT,
                    OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS,
                    LIST_BOX_FN_7211C0,
                    EMBEDDED_LIST_DESTRUCT
                ]
            ),
            vec![
                OPTION_LIST_SET_TOTAL_LIST_HEIGHT,
                OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS,
                LIST_BOX_FN_7211C0,
                EMBEDDED_LIST_DESTRUCT
            ]
        );
        assert_eq!(
            calls(&e, EMBEDDED_LIST_DESTRUCT),
            vec![vec![list.addr() + 4]]
        );
    }

    /// A list box with a container and a template name, whose item
    /// rendering and list-node construction are doubled.
    fn adding_list(e: &mut Engine) -> (Ptr<ListBoxLayout>, u32) {
        let list = e.new_object::<ListBoxLayout>();
        let container = e.mem.alloc(16);
        e.set(list, ListBoxLayout::pContainerRect, Ptr::new(container));
        e.set(list, ListBoxLayout::pTemplateName, 0x9400);
        e.set(list, ListBoxLayout::usNextIndex, 5);
        stub(e, TILE_GET_MENU, 0x5800);
        stub(e, MENU_RENDER_TEMPLATE, 0x5500);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        // ListBoxItem(this, tile, &data): {tile, data}
        e.register(LIST_ITEM_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            let data = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 4, data);
            ret(a[0])
        });
        (list, container)
    }

    #[test]
    fn adding_an_item_appends_a_list_item_and_numbers_the_tile() {
        let mut e = engine();
        let (list, container) = adding_list(&mut e);
        stub(&mut e, LIST_FLAG_TEST, 1);
        let appended = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&appended);
        e.register_double(LIST_ADD_TAIL, move |e, a| {
            let node = e.mem.u32(a[1]);
            record
                .borrow_mut()
                .push((a[0], e.mem.u32(node), e.mem.u32(node + 4)));
            Ret::default()
        });
        let item = list_box_option_add_item(&mut e, list, 0xd1, 0x9500, 0, 0);
        assert_eq!(item, Ptr::new(0x5500));
        assert_eq!(calls(&e, TILE_GET_MENU), vec![vec![container]]);
        assert_eq!(
            calls(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x5800, container, 0x9400, 0]]
        );
        // an item without an id value gets -1
        assert_eq!(calls(&e, TILE_GET_VALUE), vec![vec![0x5500, 0x0faa]]);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x5500, 0x0faa, u32::MAX]));
        assert!(calls(&e, TILE_SET_STRING).contains(&vec![0x5500, 0x0fc4, 0x9500, 1]));
        assert_eq!(*appended.borrow(), vec![(list.addr() + 4, 0x5500, 0xd1)]);
        // auto update places the tile and recomputes the height
        assert_eq!(
            calls(&e, LIST_INIT_TILE_POSITION),
            vec![vec![list.addr(), 0x5500]]
        );
        assert_eq!(
            calls(&e, OPTION_LIST_SET_TOTAL_LIST_HEIGHT),
            vec![vec![list.addr()]]
        );
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 6);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x5500, 0x0fac, 5]));
        assert!(calls(&e, LIST_INSERT).is_empty());
    }

    #[test]
    fn adding_an_item_after_a_position_inserts_it_and_synchronises() {
        let mut e = engine();
        let (list, container) = adding_list(&mut e);
        stub(&mut e, LIST_FLAG_TEST, 1);
        stub(&mut e, TILE_GET_VALUE, 1);
        let item = list_box_option_add_item(&mut e, list, 0xd2, 0, 0x7700, 0x9600);
        assert_eq!(item, Ptr::new(0x5500));
        // the given template wins over the list's own
        assert_eq!(
            calls(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x5800, container, 0x9600, 0]]
        );
        assert!(calls(&e, TILE_SET_STRING).is_empty());
        assert!(calls(&e, TILE_SET_INT).is_empty());
        let inserts = calls(&e, LIST_INSERT);
        assert_eq!(inserts.len(), 1);
        assert_eq!(inserts[0][0], list.addr() + 4);
        assert_eq!(inserts[0][2], 0x7700);
        assert_eq!(calls(&e, OPTION_LIST_SYNCHRONIZE), vec![vec![list.addr()]]);
        assert!(calls(&e, LIST_ADD_TAIL).is_empty());
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 5);
        // without auto update there is no synchronisation
        stub(&mut e, LIST_FLAG_TEST, 0);
        e.call_log = Some(Vec::new());
        list_box_option_add_item(&mut e, list, 0xd2, 0, 0x7700, 0);
        assert!(calls(&e, OPTION_LIST_SYNCHRONIZE).is_empty());
    }

    #[test]
    fn adding_an_item_needs_a_container_and_a_template() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        assert_eq!(
            list_box_option_add_item(&mut e, list, 1, 2, 0, 3),
            Ptr::NULL
        );
        let container = e.mem.alloc(16);
        e.set(list, ListBoxLayout::pContainerRect, Ptr::new(container));
        assert_eq!(
            list_box_option_add_item(&mut e, list, 1, 2, 0, 0),
            Ptr::NULL
        );
        assert!(calls(&e, MENU_RENDER_TEMPLATE).is_empty());
    }

    #[test]
    fn the_first_item_keeps_the_visible_item_count_in_sync() {
        let mut e = engine();
        let (list, container) = adding_list(&mut e);
        stub(&mut e, TILE_GET_VALUE, 1);
        stub(&mut e, LIST_COUNT, 1);
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        stub(&mut e, TILE_GET_VALUE_Q, 0x5a00);
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            float_ret(if a[0] == container && a[1] == 0xabc {
                5.0
            } else {
                0.0
            })
        });
        let item = list_box_option_add_item(&mut e, list, 0xd3, 0, 0, 0);
        assert_eq!(item, Ptr::new(0x5500));
        assert_eq!(calls(&e, TILE_VALUE_CLEAR_ACTIONS), vec![vec![0x5a00]]);
        assert_eq!(
            calls(&e, TILE_VALUE_ADD_ACTION),
            vec![vec![0x5a00, 0x7d0, 0x5500, 0x0fb0]]
        );
        assert_eq!(
            calls(&e, TILE_VALUE_ADD_ACTION_FLOAT),
            vec![vec![0x5a00, 0x7d3, 5.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, TILE_VALUE_CALCULATE), vec![vec![0x5a00, 0]]);
        assert_eq!(
            calls(&e, TILE_ADD_USER_TRAIT)[0],
            vec![LIST_HEIGHT_TRAIT_NAME, u32::MAX]
        );
        // a zero value leaves the actions alone
        stub_float(&mut e, TILE_GET_FLOAT, 0.0);
        e.call_log = Some(Vec::new());
        list_box_option_add_item(&mut e, list, 0xd4, 0, 0, 0);
        assert!(calls(&e, TILE_VALUE_CLEAR_ACTIONS).is_empty());
    }

    #[test]
    fn a_failed_list_item_allocation_appends_a_null_item() {
        let mut e = engine();
        let (list, _) = adding_list(&mut e);
        stub(&mut e, TILE_GET_VALUE, 1);
        stub(&mut e, OPERATOR_NEW, 0);
        let appended = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&appended);
        e.register_double(LIST_ADD_TAIL, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        list_box_option_add_item(&mut e, list, 0xd5, 0, 0, 0);
        assert_eq!(*appended.borrow(), vec![0]);
        assert!(calls(&e, LIST_ITEM_CONSTRUCT).is_empty());
    }

    /// A tile whose vtable's first slot is the double at `0x6300_0000`.
    fn tile_object(e: &mut Engine) -> u32 {
        let tile = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable, 0x6300_0000);
        e.mem.set_u32(tile, vtable);
        e.register(0x6300_0000, |_, _| Ret::default());
        tile
    }

    #[test]
    fn removing_all_items_destroys_nodes_then_the_head() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        let container = e.mem.alloc(16);
        e.set(list, ListBoxLayout::pContainerRect, Ptr::new(container));
        e.set(list, ListBoxLayout::usNextIndex, 7);
        e.set(list, ListBoxLayout::fTotalListHeight, 9.0);
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(0x5000));
        // head item H {tile, data}; node N1 -> item I1 {tile, data}
        let (head_tile, tile_1) = (tile_object(&mut e), tile_object(&mut e));
        let head = e.mem.alloc(12);
        e.mem.set_u32(head, head_tile);
        e.mem.set_u32(head + 4, 0xd0);
        let item_1 = e.mem.alloc(12);
        e.mem.set_u32(item_1, tile_1);
        e.mem.set_u32(item_1 + 4, 0xd1);
        let node_1 = e.mem.alloc(8);
        e.mem.set_u32(node_1, item_1);
        e.set(list, ListBoxLayout::m_item, Ptr::new(head));
        e.set(list, ListBoxLayout::m_pkNext, Ptr::new(node_1));
        e.register(LIST_NODE_SELF, |_, a| ret(a[0]));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_NODE_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        stub(&mut e, LIST_FLAG_TEST, 1);
        stub(&mut e, TILE_TEXT_TO_TRAIT, 0xabc2);
        list_box_option_remove_all(&mut e, list);
        let (head_tile_dtor, tile_1_dtor) = (vec![head_tile, 1], vec![tile_1, 1]);
        assert_eq!(calls(&e, 0x6300_0000), vec![tile_1_dtor, head_tile_dtor]);
        assert_eq!(
            calls(&e, OPTION_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![0xd1, 1], vec![0xd0, 1]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![item_1], vec![head]]);
        assert_eq!(calls(&e, LIST_NODE_SET_NEXT), vec![vec![node_1, 0]]);
        assert_eq!(
            calls(&e, LIST_NODE_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![node_1, 1]]
        );
        assert_eq!(
            order(
                &e,
                &[
                    0x6300_0000,
                    LIST_NODE_SCALAR_DELETING_DESTRUCTOR,
                    OPERATOR_DELETE
                ]
            ),
            vec![
                0x6300_0000,
                OPERATOR_DELETE,
                LIST_NODE_SCALAR_DELETING_DESTRUCTOR,
                0x6300_0000,
                OPERATOR_DELETE
            ]
        );
        // the list is back to its empty state
        assert_eq!(e.get(list, ListBoxLayout::m_item), Ptr::NULL);
        assert_eq!(e.get(list, ListBoxLayout::m_pkNext), Ptr::NULL);
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 0);
        assert_eq!(e.get(list, ListBoxLayout::fTotalListHeight), 0.0);
        assert_eq!(e.get(list, ListBoxLayout::pHighlightedItem), Ptr::NULL);
        assert_eq!(
            calls(&e, TILE_TEXT_TO_TRAIT),
            vec![vec![LIST_RESET_TRAIT_NAME]]
        );
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![container, 0xabc2, 0, 1]]
        );
        assert_eq!(
            order(
                &e,
                &[
                    OPTION_LIST_SET_TOTAL_LIST_HEIGHT,
                    OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS
                ]
            ),
            vec![
                OPTION_LIST_SET_TOTAL_LIST_HEIGHT,
                OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS
            ]
        );
    }

    #[test]
    fn removing_all_items_can_keep_tiles_and_data() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        let head_tile = tile_object(&mut e);
        let head = e.mem.alloc(12);
        e.mem.set_u32(head, head_tile);
        e.mem.set_u32(head + 4, 0xd0);
        e.set(list, ListBoxLayout::m_item, Ptr::new(head));
        e.mem.set_u8(SKIP_TILE_DESTROY_BYTE, 1);
        stub(&mut e, LIST_FLAG_TEST, 0);
        list_box_option_remove_all(&mut e, list);
        assert!(calls(&e, 0x6300_0000).is_empty());
        assert!(calls(&e, OPTION_SCALAR_DELETING_DESTRUCTOR).is_empty());
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![head]]);
        // no container: no value reset
        assert!(calls(&e, TILE_TEXT_TO_TRAIT).is_empty());
        assert_eq!(e.get(list, ListBoxLayout::m_item), Ptr::NULL);
        // an empty list only resets the state
        e.call_log = Some(Vec::new());
        list_box_option_remove_all(&mut e, list);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls(&e, OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS).len(), 1);
    }

    #[test]
    fn sorting_the_embedded_list_synchronises_the_tiles() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        list_box_option_sort(&mut e, list, 0x7a7a);
        assert_eq!(calls(&e, LIST_SORT), vec![vec![list.addr() + 4, 0x7a7a]]);
        assert_eq!(calls(&e, OPTION_LIST_SYNCHRONIZE), vec![vec![list.addr()]]);
        assert_eq!(
            order(&e, &[LIST_SORT, OPTION_LIST_SYNCHRONIZE]),
            vec![LIST_SORT, OPTION_LIST_SYNCHRONIZE]
        );
    }

    // ---- the second half: list box state, selection, removal and the arrays ----

    const ENABLED_ID: u32 = TRAIT_NAME_ENABLED & 0xffff;
    const SELECTED_ID: u32 = TRAIT_NAME_SELECTED & 0xffff;
    /// The id the `_num_filtered` name gets from [`install_traits`].
    const FILTERED_ID: u32 = LIST_RESET_TRAIT_NAME & 0xffff;

    /// A list box with a container tile.
    fn list_with_container(e: &mut Engine) -> (Ptr<ListBoxLayout>, u32) {
        let list = e.new_object::<ListBoxLayout>();
        let container = tile(30);
        e.set(list, ListBoxLayout::pContainerRect, Ptr::new(container));
        (list, container)
    }

    /// Gives each trait name an id (the low half of the name's address) and
    /// answers `Tile::IsTrue`: `_enabled` is true except for the `disabled`
    /// tiles, `_selected` only for the `selected` ones.
    fn install_traits(e: &mut Engine, selected: &[u32], disabled: &[u32]) {
        e.register(TILE_ADD_USER_TRAIT, |_, a| ret(a[0] & 0xffff));
        let selected = selected.to_vec();
        let disabled = disabled.to_vec();
        e.register_double(TILE_IS_TRUE, move |_, a| {
            let truth = match a[1] {
                ENABLED_ID => !disabled.contains(&a[0]),
                SELECTED_ID => selected.contains(&a[0]),
                _ => false,
            };
            ret(u32::from(truth))
        });
    }

    /// Items on `list`, one per tile: node 0 is the embedded head (at
    /// `list + 4`), the others are allocated; an item is {tile, data, filtered
    /// byte}. Returns each item's node and item address.
    fn fill_list(e: &mut Engine, list: Ptr<ListBoxLayout>, tiles: &[u32]) -> Vec<(u32, u32)> {
        e.register(LIST_NODE_SELF, |_, a| ret(a[0]));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        let mut placed: Vec<(u32, u32)> = Vec::new();
        for (index, tile) in tiles.iter().enumerate() {
            let node = if index == 0 {
                list.addr() + 4
            } else {
                e.mem.alloc(8)
            };
            let item = e.mem.alloc(12);
            e.mem.set_u32(item, *tile);
            e.mem.set_u32(item + 4, 0xd0 + index as u32);
            e.mem.set_u32(node, item);
            if let Some((previous, _)) = placed.last() {
                e.mem.set_u32(previous + 4, node);
            }
            placed.push((node, item));
        }
        e.set(list, ListBoxLayout::usNextIndex, tiles.len() as u16);
        placed
    }

    /// The `0xFAC` value of each tile is its position in `tiles`.
    fn install_indexes(e: &mut Engine, tiles: &[u32]) {
        let tiles = tiles.to_vec();
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            float_ret(if a[1] == TRAIT_0FAC {
                tiles
                    .iter()
                    .position(|t| *t == a[0])
                    .map_or(-1.0, |p| p as f64)
            } else {
                0.0
            })
        });
    }

    /// Four items (tiles 40 to 43, indexes 0 to 3) on a list with a vtable
    /// whose `GetTileAt` (slot +0x14) answers with the tile of that index.
    fn four_item_list(e: &mut Engine) -> (Ptr<ListBoxLayout>, u32, Vec<u32>) {
        let (list, container) = list_with_container(e);
        let tiles: Vec<u32> = (40..44).map(tile).collect();
        fill_list(e, list, &tiles);
        install_indexes(e, &tiles);
        list_vtable(e, list);
        let at = tiles.clone();
        e.register_double(slot_fn(0x14), move |_, a| ret(at[a[1] as usize]));
        (list, container, tiles)
    }

    /// The guard bit of `which` is set and its cache holds `id`.
    fn assert_static(e: &Engine, which: StaticTrait, id: u32) {
        assert_eq!(e.mem.u32(which.guard) & which.bit, which.bit);
        assert_eq!(e.mem.u32(which.cache), id);
    }

    fn check_set_enabled(set: fn(&mut Engine, Ptr<ListBoxLayout>, u8), which: StaticTrait) {
        let mut e = engine();
        install_traits(&mut e, &[], &[]);
        let (list, container) = list_with_container(&mut e);
        set(&mut e, list, 1);
        set(&mut e, list, 0);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![container, ENABLED_ID, 1],
                vec![container, ENABLED_ID, 0]
            ]
        );
        // the trait id is looked up once
        assert_eq!(
            calls(&e, TILE_ADD_USER_TRAIT),
            vec![vec![TRAIT_NAME_ENABLED, u32::MAX]]
        );
        assert_static(&e, which, ENABLED_ID);
    }

    fn check_is_enabled(is: fn(&mut Engine, Ptr<ListBoxLayout>) -> bool, which: StaticTrait) {
        let mut e = engine();
        install_traits(&mut e, &[], &[]);
        let list = e.new_object::<ListBoxLayout>();
        assert!(!is(&mut e, list));
        assert!(calls(&e, TILE_IS_TRUE).is_empty());
        let container = tile(30);
        e.set(list, ListBoxLayout::pContainerRect, Ptr::new(container));
        assert!(is(&mut e, list));
        assert_eq!(calls(&e, TILE_IS_TRUE), vec![vec![container, ENABLED_ID]]);
        install_traits(&mut e, &[], &[container]);
        assert!(!is(&mut e, list));
        assert_static(&e, which, ENABLED_ID);
    }

    fn check_highlight(
        highlight: fn(&mut Engine, Ptr<ListBoxLayout>, Ptr) -> bool,
        is_enabled_guard: u32,
    ) {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        install_traits(&mut e, &[], &[tile(41)]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        assert!(highlight(&mut e, list, Ptr::new(tile(40))));
        assert_eq!(
            e.get(list, ListBoxLayout::pHighlightedItem),
            Ptr::new(tile(40))
        );
        // the highlight box followed it, and the list's `IsEnabled` ran
        assert!(!calls(&e, TILE_GET_VALUE).is_empty());
        assert_eq!(e.mem.u32(is_enabled_guard) & 1, 1);
        // a disabled tile is refused
        assert!(!highlight(&mut e, list, Ptr::new(tile(41))));
        assert_eq!(
            e.get(list, ListBoxLayout::pHighlightedItem),
            Ptr::new(tile(40))
        );
        // so is a tile the list does not hold
        stub(&mut e, LIST_CONTAINS_TILE, 0);
        assert!(!highlight(&mut e, list, Ptr::new(tile(42))));
        // nothing clears the highlight
        assert!(highlight(&mut e, list, Ptr::NULL));
        assert_eq!(e.get(list, ListBoxLayout::pHighlightedItem), Ptr::NULL);
        // a disabled list refuses everything
        install_traits(&mut e, &[], &[tile(30)]);
        assert!(!highlight(&mut e, list, Ptr::NULL));
    }

    fn check_highlight_target(target: fn(&mut Engine, Ptr<ListBoxLayout>, u32) -> u32) {
        let mut e = engine();
        let (list, container, tiles) = four_item_list(&mut e);
        install_traits(&mut e, &[], &[tiles[2]]);
        // no highlight: +1 starts at the first tile
        assert_eq!(target(&mut e, list, 2), tiles[0]);
        // -1 starts at the last one
        assert_eq!(target(&mut e, list, 1), tiles[3]);
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(tiles[1]));
        // +1 lands on the disabled tile 2, so tile 3 is chosen
        assert_eq!(target(&mut e, list, 2), tiles[3]);
        // -1 moves back to tile 0
        assert_eq!(target(&mut e, list, 1), tiles[0]);
        // directions that do not move
        assert_eq!(target(&mut e, list, 7), 0);
        // at the first tile -1 stays on it, which gives no target
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(tiles[0]));
        assert_eq!(target(&mut e, list, 1), 0);
        // paging: a page is the visible height over the scroll delta
        e.register(ROUND_UP_TO_MULTIPLE, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0])))
        });
        stub(&mut e, TILE_TEXT_TO_TRAIT, 0xd1d1);
        let at = tiles.clone();
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            float_ret(match (a[0] == container, a[1]) {
                (true, TRAIT_0FB0) => 2.0,
                (true, 0xd1d1) => 1.0,
                (false, TRAIT_0FAC) => at
                    .iter()
                    .position(|t| *t == a[0])
                    .map_or(-1.0, |p| p as f64),
                _ => 0.0,
            })
        });
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(tiles[0]));
        assert_eq!(target(&mut e, list, 0x10), tiles[3]);
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(tiles[3]));
        assert_eq!(target(&mut e, list, 0xf), tiles[1]);
        assert_eq!(
            calls(&e, TILE_TEXT_TO_TRAIT)[0],
            vec![TRAIT_NAME_SCROLL_DELTA]
        );
        // a disabled list has no target
        install_traits(&mut e, &[], &[container]);
        assert_eq!(target(&mut e, list, 2), 0);
    }

    fn check_select(
        select: fn(&mut Engine, Ptr<ListBoxLayout>, Ptr, u32) -> bool,
        which: StaticTrait,
    ) {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        let (chosen, other) = (tile(40), tile(41));
        install_traits(&mut e, &[other], &[]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        // mode 0 selects what is not selected, and only that
        assert!(select(&mut e, list, Ptr::new(chosen), 0));
        assert!(!select(&mut e, list, Ptr::new(other), 0));
        // mode 1 deselects what is selected
        assert!(select(&mut e, list, Ptr::new(other), 1));
        assert!(!select(&mut e, list, Ptr::new(chosen), 1));
        // mode 2 toggles
        assert!(select(&mut e, list, Ptr::new(chosen), 2));
        assert!(select(&mut e, list, Ptr::new(other), 2));
        // other modes, a null tile and a tile the list lacks do nothing
        assert!(!select(&mut e, list, Ptr::new(chosen), 3));
        assert!(!select(&mut e, list, Ptr::NULL, 0));
        stub(&mut e, LIST_CONTAINS_TILE, 0);
        assert!(!select(&mut e, list, Ptr::new(chosen), 0));
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![chosen, SELECTED_ID, 1],
                vec![other, SELECTED_ID, 0],
                vec![chosen, SELECTED_ID, 1],
                vec![other, SELECTED_ID, 0]
            ]
        );
        assert_static(&e, which, SELECTED_ID);
        // a disabled list selects nothing
        install_traits(&mut e, &[], &[tile(30)]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        assert!(!select(&mut e, list, Ptr::new(chosen), 0));
    }

    fn check_deselect_all(deselect_all: fn(&mut Engine, Ptr<ListBoxLayout>, u8)) {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        let tiles: Vec<u32> = (40..43).map(tile).collect();
        let placed = fill_list(&mut e, list, &tiles);
        // the second item was filtered out; the first and third are selected
        e.mem.set_u8(placed[1].1 + LIST_ITEM_FILTERED, 1);
        install_traits(&mut e, &[tiles[0], tiles[1], tiles[2]], &[]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        deselect_all(&mut e, list, 1);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![tiles[0], SELECTED_ID, 0],
                vec![tiles[2], SELECTED_ID, 0]
            ]
        );
        e.call_log = Some(Vec::new());
        deselect_all(&mut e, list, 0);
        assert_eq!(calls(&e, TILE_SET_INT).len(), 3);
    }

    fn check_synchronize(synchronize: fn(&mut Engine, Ptr<ListBoxLayout>)) {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        let tiles: Vec<u32> = (40..43).map(tile).collect();
        let placed = fill_list(&mut e, list, &tiles);
        e.mem.set_u8(placed[1].1 + LIST_ITEM_FILTERED, 1);
        e.set(list, ListBoxLayout::fTotalListHeight, 9.0);
        install_traits(&mut e, &[], &[]);
        synchronize(&mut e, list);
        // the filtered item is skipped; the others are numbered 0 and 1
        assert_eq!(
            calls(&e, TILE_SET_INT)
                .into_iter()
                .filter(|c| c[1] == TRAIT_0FAC)
                .collect::<Vec<_>>(),
            vec![vec![tiles[0], TRAIT_0FAC, 0], vec![tiles[2], TRAIT_0FAC, 1]]
        );
        assert_eq!(
            calls(&e, LIST_INIT_TILE_POSITION),
            vec![vec![list.addr(), tiles[0]], vec![list.addr(), tiles[2]]]
        );
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 2);
        assert_eq!(e.get(list, ListBoxLayout::fTotalListHeight), 0.0);
    }

    /// A list with a highlighted tile whose flag byte `TILE_LOAD_BYTE_35`
    /// answers with `flag`, and the tiles `lb_highlight_box` and its edges.
    fn check_update_highlight_box_pos(
        update: fn(&mut Engine, Ptr<ListBoxLayout>),
        y: StaticTrait,
        height: StaticTrait,
    ) {
        let mut e = engine();
        e.register(TILE_ADD_USER_TRAIT, |_, a| ret(a[0] & 0xffff));
        // no container: only the static traits are looked up
        let bare = e.new_object::<ListBoxLayout>();
        update(&mut e, bare);
        assert!(calls(&e, TILE_SET_INT).is_empty());
        assert_static(&e, y, TRAIT_NAME_HIGHLIGHT_Y & 0xffff);
        assert_static(&e, height, TRAIT_NAME_SELECTED_HEIGHT & 0xffff);
        // no highlight: y is -1 and the height 0
        let (list, container) = list_with_container(&mut e);
        update(&mut e, list);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![container, TRAIT_NAME_HIGHLIGHT_Y & 0xffff, u32::MAX],
                vec![container, TRAIT_NAME_SELECTED_HEIGHT & 0xffff, 0]
            ]
        );
        // a highlight
        e.call_log = Some(Vec::new());
        let highlighted = tile(40);
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(highlighted));
        e.register(TILE_GET_VALUE, |_, a| ret(0x9000 + a[1]));
        stub(&mut e, TILE_LOAD_BYTE_35, 1);
        e.register(TILE_FIND_BY_NAME, |_, a| {
            ret(if a[1] == NAME_HIGHLIGHT_BOX {
                0x8000
            } else {
                0
            })
        });
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| ret(0x8000 + (a[1] & 0xff)));
        update(&mut e, list);
        let y_value = 0x9000 + (TRAIT_NAME_HIGHLIGHT_Y & 0xffff);
        let height_value = 0x9000 + (TRAIT_NAME_SELECTED_HEIGHT & 0xffff);
        assert_eq!(
            calls(&e, TILE_VALUE_ADD_ACTION),
            vec![
                vec![y_value, 0x7d0, highlighted, TRAIT_0FA2],
                vec![height_value, 0x7d0, highlighted, TRAIT_0FB0]
            ]
        );
        assert_eq!(
            calls(&e, TILE_VALUE_CALCULATE),
            vec![vec![y_value, 0], vec![height_value, 0]]
        );
        // the four edges (top, bottom, left, right), then the box itself
        let marked = calls(&e, TILE_STORE_BYTE_35);
        let names = [NAME_TOP, NAME_BOTTOM, NAME_LEFT, NAME_RIGHT];
        let mut expected: Vec<Vec<u32>> =
            names.iter().map(|n| vec![0x8000 + (n & 0xff), 1]).collect();
        expected.push(vec![0x8000, 1]);
        assert_eq!(marked, expected);
        assert_eq!(
            calls(&e, TILE_FIND_BY_NAME),
            vec![vec![container, NAME_HIGHLIGHT_BOX]]
        );
        assert_eq!(calls(&e, TILE_ADD_NEEDS_UPDATE).len(), 5);
        assert!(calls(&e, TILE_ADD_NEEDS_UPDATE).iter().all(|c| c[1] == 8));
        // a box without a top edge only marks itself
        e.call_log = Some(Vec::new());
        e.register(TILE_GET_CHILD_BY_NAME, |_, _| Ret::default());
        stub(&mut e, TILE_LOAD_BYTE_35, 0);
        update(&mut e, list);
        assert_eq!(calls(&e, TILE_STORE_BYTE_35), vec![vec![0x8000, 0]]);
        // no box at all
        e.call_log = Some(Vec::new());
        stub(&mut e, TILE_FIND_BY_NAME, 0);
        update(&mut e, list);
        assert!(calls(&e, TILE_STORE_BYTE_35).is_empty());
    }

    fn check_set_total_list_height(set: fn(&mut Engine, Ptr<ListBoxLayout>)) {
        let mut e = engine();
        let (list, container) = list_with_container(&mut e);
        let scroll_bar = tile(31);
        e.set(list, ListBoxLayout::pScrollBar, Ptr::new(scroll_bar));
        e.set(list, ListBoxLayout::fTotalListHeight, 10.0);
        install_traits(&mut e, &[], &[]);
        stub(&mut e, TILE_TEXT_TO_TRAIT, 0xabc3);
        stub(&mut e, LIST_COUNT, 5);
        e.register(ROUND_FLOAT, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).ceil()))
        });
        e.register(MAX_INT, |_, a| ret((a[0] as i32).max(a[1] as i32) as u32));
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            float_ret(match (a[0] == container, a[1]) {
                (true, TRAIT_0FB0) => 2.0,
                (true, FILTERED_ID) => 1.0,
                _ => 0.0,
            })
        });
        set(&mut e, list);
        // 5 items - 1 filtered = 4; 4 * (10 - 2) / 10 = 3.2 -> 4; 4 + 1 = 5 steps
        assert_eq!(calls(&e, MAX_INT), vec![vec![5, 2]]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![container, 0xabc3, 2.0f32.to_bits(), 1]]
        );
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![scroll_bar, 0xabc3, 5]]);
        assert_eq!(
            calls(&e, TILE_TEXT_TO_TRAIT),
            vec![
                vec![TRAIT_NAME_SCROLL_DELTA],
                vec![TRAIT_NAME_NUMBER_OF_ITEMS]
            ]
        );
        // a list that fits needs one step and no scroll delta
        e.set(list, ListBoxLayout::fTotalListHeight, 2.0);
        e.call_log = Some(Vec::new());
        set(&mut e, list);
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![scroll_bar, 0xabc3, 1]]);
        // without a container nothing happens
        let bare = e.new_object::<ListBoxLayout>();
        e.call_log = Some(Vec::new());
        set(&mut e, bare);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    fn check_num_unfiltered(count: fn(&mut Engine, Ptr<ListBoxLayout>) -> i32, which: StaticTrait) {
        let mut e = engine();
        let (list, container) = list_with_container(&mut e);
        install_traits(&mut e, &[], &[]);
        stub(&mut e, LIST_COUNT, 7);
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            float_ret(if a[0] == container && a[1] == FILTERED_ID {
                2.75
            } else {
                0.0
            })
        });
        assert_eq!(count(&mut e, list), 5);
        assert_static(&e, which, FILTERED_ID);
    }

    #[test]
    fn the_second_half_registers_forty_nine_functions() {
        let table = funcs();
        assert_eq!(table.len(), 109);
        let mut addresses: Vec<u32> = table.iter().map(|(a, _)| *a).collect();
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 109);
        assert!(addresses
            .iter()
            .all(|a| (0x007d_3aa0..=0x007d_a040).contains(a)));
        assert_eq!(addresses.iter().filter(|a| **a > 0x007d_74c0).count(), 49);
    }

    #[test]
    fn the_float_to_integer_helper_is_fistp_truncation() {
        assert_eq!(fistp_truncate(2.75), 2);
        assert_eq!(fistp_truncate(-2.75), -2);
        assert_eq!(fistp_truncate(f64::NAN), 0);
        assert_eq!(fistp_truncate(1.0e30), 0);
        assert_eq!(fistp_truncate(4_294_967_296.0 + 7.0), 7);
    }

    #[test]
    fn option_list_set_enabled_writes_the_trait() {
        check_set_enabled(list_box_option_set_enabled, OPTION_SET_ENABLED_TRAIT);
    }

    #[test]
    fn option_list_is_enabled_needs_a_container_with_the_trait() {
        check_is_enabled(list_box_option_is_enabled, OPTION_IS_ENABLED_TRAIT);
    }

    #[test]
    fn the_container_visibility_test_needs_a_container() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        assert!(!fn_007d7630(&mut e, list));
        let container = tile(30);
        e.set(list, ListBoxLayout::pContainerRect, Ptr::new(container));
        assert!(!fn_007d7630(&mut e, list));
        stub(&mut e, TILE_IS_VISIBLE, 1);
        assert!(fn_007d7630(&mut e, list));
        assert_eq!(calls(&e, TILE_IS_VISIBLE).last(), Some(&vec![container]));
    }

    #[test]
    fn option_list_highlight_checks_the_list_and_the_tile() {
        check_highlight(list_box_option_highlight, 0x011d_ac78);
    }

    #[test]
    fn option_list_highlight_target_steps_over_disabled_tiles() {
        check_highlight_target(list_box_option_get_highlight_target);
    }

    #[test]
    fn selecting_in_the_options_list_is_mode_zero() {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        install_traits(&mut e, &[], &[]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        assert!(fn_007d7a80(&mut e, list, Ptr::new(tile(40))));
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![tile(40), SELECTED_ID, 1]]
        );
    }

    #[test]
    fn option_list_is_selected_reads_the_selected_trait() {
        let mut e = engine();
        install_traits(&mut e, &[tile(40)], &[]);
        let list = Ptr::new(0x1234);
        assert!(list_box_option_is_selected(
            &mut e,
            list,
            Ptr::new(tile(40))
        ));
        assert!(!list_box_option_is_selected(
            &mut e,
            list,
            Ptr::new(tile(41))
        ));
        assert!(!list_box_option_is_selected(&mut e, list, Ptr::NULL));
        assert_eq!(calls(&e, TILE_IS_TRUE).len(), 2);
        assert_static(&e, OPTION_IS_SELECTED_TRAIT, SELECTED_ID);
        // an unusable trait id means nothing is selected
        e.mem.set_u32(OPTION_IS_SELECTED_TRAIT.cache, 0);
        assert!(!list_box_option_is_selected(
            &mut e,
            list,
            Ptr::new(tile(40))
        ));
    }

    #[test]
    fn option_list_deselect_all_can_skip_filtered_items() {
        check_deselect_all(list_box_option_deselect_all);
    }

    #[test]
    fn the_downloads_list_set_enabled_writes_the_trait() {
        check_set_enabled(list_box_int_set_enabled, DOWNLOADS_SET_ENABLED_TRAIT);
    }

    #[test]
    fn the_file_entry_list_constructor_stores_its_vtable() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        for offset in (4..0x30).step_by(4) {
            e.mem.set_u32(list.addr() + offset, 0xffff_ffff);
        }
        assert_eq!(list_box_file_entry_construct(&mut e, list), list);
        assert_eq!(e.mem.u32(list.addr()), FILE_ENTRY_LIST_BOX_VTABLE);
        assert_eq!(calls(&e, LIST_BOX_BASE_CONSTRUCT), vec![vec![list.addr()]]);
        assert_eq!(
            calls(&e, EMBEDDED_LIST_CONSTRUCT),
            vec![vec![list.addr() + 4]]
        );
        for field in [0x0c, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(list.addr() + field), 0);
        }
        assert_eq!(e.mem.u16(list.addr() + 0x1c), 0);
        assert_eq!(e.mem.u16(list.addr() + 0x2c), 0);
        for field in [0x20, 0x24, 0x28] {
            assert_eq!(e.mem.f32(list.addr() + field), 0.0);
        }
        assert_eq!(calls(&e, LIST_BOX_FN_7A1950), vec![vec![list.addr(), 0, 1]]);
        assert_eq!(calls(&e, LIST_BOX_FN_721130), vec![vec![list.addr()]]);
    }

    #[test]
    fn the_file_entry_list_destructor_empties_the_list_first() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        list_box_file_entry_destruct(&mut e, list);
        assert_eq!(e.mem.u32(list.addr()), FILE_ENTRY_LIST_BOX_VTABLE);
        assert_eq!(
            order(
                &e,
                &[
                    FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT,
                    FILE_ENTRY_LIST_UPDATE_HIGHLIGHT_BOX_POS,
                    LIST_BOX_FN_7211C0,
                    EMBEDDED_LIST_DESTRUCT
                ]
            ),
            vec![
                FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT,
                FILE_ENTRY_LIST_UPDATE_HIGHLIGHT_BOX_POS,
                LIST_BOX_FN_7211C0,
                EMBEDDED_LIST_DESTRUCT
            ]
        );
        assert_eq!(
            calls(&e, EMBEDDED_LIST_DESTRUCT),
            vec![vec![list.addr() + 4]]
        );
    }

    #[test]
    fn adding_a_file_entry_uses_the_file_entry_list_functions() {
        let mut e = engine();
        let (list, _) = adding_list(&mut e);
        stub(&mut e, LIST_FLAG_TEST, 1);
        stub(&mut e, TILE_GET_VALUE, 1);
        let item = list_box_file_entry_add_item(&mut e, list, 0xd1, 0, 0, 0);
        assert_eq!(item, Ptr::new(0x5500));
        assert_eq!(
            calls(&e, FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT),
            vec![vec![list.addr()]]
        );
        assert!(calls(&e, OPTION_LIST_SET_TOTAL_LIST_HEIGHT).is_empty());
        // inserted after a position: the file entry list synchronizes
        list_box_file_entry_add_item(&mut e, list, 0xd2, 0, 0x7700, 0);
        assert_eq!(
            calls(&e, FILE_ENTRY_LIST_SYNCHRONIZE),
            vec![vec![list.addr()]]
        );
        assert!(calls(&e, OPTION_LIST_SYNCHRONIZE).is_empty());
    }

    /// Three items whose tiles have a destructor double; the highlight is on
    /// the tile of `highlighted`.
    fn removable_list(
        e: &mut Engine,
        highlighted: usize,
    ) -> (Ptr<ListBoxLayout>, Vec<u32>, Vec<(u32, u32)>) {
        let (list, _) = list_with_container(e);
        let tiles = vec![tile_object(e), tile_object(e), tile_object(e)];
        let placed = fill_list(e, list, &tiles);
        e.set(
            list,
            ListBoxLayout::pHighlightedItem,
            Ptr::new(tiles[highlighted]),
        );
        e.register(LIST_NODE_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(LIST_FLAG_TEST, |_, a| ret(a[1]));
        stub(e, INTERFACE_MANAGER, 0x4000);
        (list, tiles, placed)
    }

    #[test]
    fn removing_a_middle_node_moves_the_highlight_to_the_next_tile() {
        let mut e = engine();
        let (list, tiles, placed) = removable_list(&mut e, 1);
        let (head, middle, last) = (placed[0].0, placed[1].0, placed[2].0);
        assert!(fn_007d9ac0(&mut e, list, middle, head));
        assert_eq!(
            e.get(list, ListBoxLayout::pHighlightedItem),
            Ptr::new(tiles[2])
        );
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x4000, tiles[2], TRAIT_0FC3, 1]]
        );
        // the tile is destroyed, the data deleted (the list owns it), the
        // item freed, the node unlinked and deleted
        assert_eq!(
            calls(&e, tile_dtor_slot(&e, tiles[1])),
            vec![vec![tiles[1], 1]]
        );
        assert_eq!(
            calls(&e, OPTION_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![0xd1, 1]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![placed[1].1]]);
        assert_eq!(
            calls(&e, LIST_NODE_SET_NEXT),
            vec![vec![head, last], vec![middle, 0]]
        );
        assert_eq!(
            calls(&e, LIST_NODE_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![middle, 1]]
        );
        assert_eq!(e.mem.u32(head + 4), last);
        // a null node is refused
        assert!(!fn_007d9ac0(&mut e, list, 0, head));
    }

    /// The address of the destructor double of a tile made by `tile_object`.
    fn tile_dtor_slot(e: &Engine, tile: u32) -> u32 {
        let vtable = e.mem.u32(tile);
        e.mem.u32(vtable)
    }

    #[test]
    fn removing_the_head_pulls_the_second_node_up() {
        let mut e = engine();
        let (list, tiles, placed) = removable_list(&mut e, 2);
        let (head, second) = (placed[0].0, placed[1].0);
        assert_eq!(head, list.addr() + 4);
        assert!(fn_007d9ac0(&mut e, list, head, head));
        // the second node's item is now the head's, its node is deleted
        assert_eq!(e.get(list, ListBoxLayout::m_item), Ptr::new(placed[1].1));
        assert_eq!(e.mem.u32(head + 4), placed[2].0);
        assert_eq!(calls(&e, LIST_NODE_SET_NEXT), vec![vec![second, 0]]);
        assert_eq!(
            calls(&e, LIST_NODE_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![second, 1]]
        );
        // the highlighted tile was not removed
        assert_eq!(
            e.get(list, ListBoxLayout::pHighlightedItem),
            Ptr::new(tiles[2])
        );
        assert!(calls(&e, SET_CURRENT_FOCUS_TARGET).is_empty());
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![placed[0].1]]);
    }

    #[test]
    fn removing_the_only_node_empties_the_list_and_may_synchronize() {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        let tiles = vec![tile_object(&mut e)];
        let placed = fill_list(&mut e, list, &tiles);
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(tiles[0]));
        e.register(LIST_FLAG_TEST, |_, a| ret(1 - a[1]));
        assert!(fn_007d9ac0(&mut e, list, placed[0].0, placed[0].0));
        assert_eq!(e.get(list, ListBoxLayout::m_item), Ptr::NULL);
        // no next node and the node is the previous one: nothing to highlight
        assert_eq!(e.get(list, ListBoxLayout::pHighlightedItem), Ptr::NULL);
        assert!(calls(&e, SET_CURRENT_FOCUS_TARGET).is_empty());
        // the data is not owned, but auto update (flag 0) is on
        assert!(calls(&e, OPTION_SCALAR_DELETING_DESTRUCTOR).is_empty());
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 0);
    }

    #[test]
    fn removing_a_filtered_item_lowers_the_filtered_count() {
        let mut e = engine();
        let (list, tiles, placed) = removable_list(&mut e, 0);
        let container = e.get(list, ListBoxLayout::pContainerRect).addr();
        e.mem.set_u8(placed[1].1 + LIST_ITEM_FILTERED, 1);
        e.set_global(ONE_AS_DOUBLE, 1.0f64);
        stub(&mut e, TILE_TEXT_TO_TRAIT, 0xabc2);
        stub_float(&mut e, TILE_GET_FLOAT, 5.0);
        assert!(fn_007d9ac0(&mut e, list, placed[1].0, placed[0].0));
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![container, 0xabc2, 4.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls(&e, TILE_TEXT_TO_TRAIT),
            vec![vec![LIST_RESET_TRAIT_NAME], vec![LIST_RESET_TRAIT_NAME]]
        );
        // the highlight stayed on the first tile
        assert_eq!(
            e.get(list, ListBoxLayout::pHighlightedItem),
            Ptr::new(tiles[0])
        );
    }

    #[test]
    fn removing_by_tile_finds_the_node_and_the_one_before() {
        let mut e = engine();
        let (list, tiles, placed) = removable_list(&mut e, 0);
        assert!(!fn_007d8040(&mut e, list, 0));
        assert!(fn_007d8040(&mut e, list, tiles[2]));
        assert_eq!(
            calls(&e, LIST_NODE_SET_NEXT),
            vec![vec![placed[1].0, 0], vec![placed[2].0, 0]]
        );
        // an empty list is refused
        stub(&mut e, LIST_IS_EMPTY_NODE, 1);
        assert!(!fn_007d8040(&mut e, list, tiles[1]));
        assert_eq!(
            calls(&e, LIST_IS_EMPTY_NODE).last(),
            Some(&vec![list.addr() + 4])
        );
    }

    #[test]
    fn removing_by_position_or_index_trait() {
        let mut e = engine();
        let (list, tiles, placed) = removable_list(&mut e, 0);
        install_indexes(&mut e, &tiles);
        // position 1: the second node, whose previous is the head
        assert!(fn_007d80f0(&mut e, list, 1, 1));
        assert_eq!(
            calls(&e, LIST_NODE_SET_NEXT)[0],
            vec![placed[0].0, placed[2].0]
        );
        // the item whose index trait is 2 (the last node; the previous node
        // is now the head)
        e.call_log = Some(Vec::new());
        let (list, tiles, placed) = removable_list(&mut e, 0);
        install_indexes(&mut e, &tiles);
        assert!(fn_007d80f0(&mut e, list, 2, 0));
        assert_eq!(calls(&e, LIST_NODE_SET_NEXT)[0], vec![placed[1].0, 0]);
        // an empty list is refused
        stub(&mut e, LIST_IS_EMPTY_NODE, 1);
        assert!(!fn_007d80f0(&mut e, list, 0, 1));
    }

    #[test]
    fn removing_everything_from_the_file_entry_list_uses_its_own_functions() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        e.set(list, ListBoxLayout::usNextIndex, 3);
        list_box_file_entry_remove_all(&mut e, list);
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 0);
        assert_eq!(
            order(
                &e,
                &[
                    FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT,
                    FILE_ENTRY_LIST_UPDATE_HIGHLIGHT_BOX_POS,
                    OPTION_LIST_SET_TOTAL_LIST_HEIGHT,
                    OPTION_LIST_UPDATE_HIGHLIGHT_BOX_POS
                ]
            ),
            vec![
                FILE_ENTRY_LIST_SET_TOTAL_LIST_HEIGHT,
                FILE_ENTRY_LIST_UPDATE_HIGHLIGHT_BOX_POS
            ]
        );
    }

    #[test]
    fn sorting_the_file_entry_list_synchronizes_its_tiles() {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        fill_list(&mut e, list, &[]);
        install_traits(&mut e, &[], &[]);
        e.set(list, ListBoxLayout::fTotalListHeight, 4.0);
        list_box_file_entry_sort(&mut e, list, 0x7a7a);
        assert_eq!(calls(&e, LIST_SORT), vec![vec![list.addr() + 4, 0x7a7a]]);
        assert_eq!(e.get(list, ListBoxLayout::fTotalListHeight), 0.0);
    }

    #[test]
    fn filtering_the_file_entry_list_renumbers_and_moves_the_highlight() {
        let mut e = engine();
        let (list, container) = list_with_container(&mut e);
        let tiles: Vec<u32> = (40..43).map(tile).collect();
        let placed = fill_list(&mut e, list, &tiles);
        install_indexes(&mut e, &tiles);
        install_traits(&mut e, &[], &[]);
        list_vtable(&mut e, list);
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(tiles[1]));
        // the filter drops the item whose data is 0xd1 (the highlighted one)
        e.register_double(0x6400_0000, |_, a| ret((a[0] == 0xd1) as u32));
        list_box_file_entry_filter(&mut e, list, 0x6400_0000);
        assert_eq!(
            calls(&e, 0x6400_0000),
            vec![vec![0xd0], vec![0xd1], vec![0xd2]]
        );
        let flags: Vec<u8> = placed
            .iter()
            .map(|(_, item)| e.mem.u8(item + LIST_ITEM_FILTERED))
            .collect();
        assert_eq!(flags, vec![0, 1, 0]);
        let numbering: Vec<Vec<u32>> = calls(&e, TILE_SET_INT)
            .into_iter()
            .filter(|c| c[1] == TRAIT_0FAC)
            .collect();
        assert_eq!(
            numbering,
            vec![
                vec![tiles[0], TRAIT_0FAC, 0],
                vec![tiles[1], TRAIT_0FAC, u32::MAX],
                vec![tiles[2], TRAIT_0FAC, 1]
            ]
        );
        assert_eq!(
            calls(&e, LIST_INIT_TILE_POSITION),
            vec![vec![list.addr(), tiles[0]], vec![list.addr(), tiles[2]]]
        );
        // the container counts one filtered item
        assert_eq!(
            calls(&e, TILE_SET_INTEGER_VALUE),
            vec![vec![container, FILTERED_ID, 1]]
        );
        // the dropped highlight went to the item that took its index (1)
        assert_eq!(
            e.get(list, ListBoxLayout::pHighlightedItem),
            Ptr::new(tiles[2])
        );
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 2);
        assert_static(&e, FILE_ENTRY_FILTER_TRAIT, FILTERED_ID);
    }

    #[test]
    fn filtering_without_a_filter_keeps_everything_and_falls_back_to_the_last_tile() {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        let tiles: Vec<u32> = (40..43).map(tile).collect();
        fill_list(&mut e, list, &tiles);
        install_indexes(&mut e, &tiles);
        install_traits(&mut e, &[], &[]);
        list_box_file_entry_filter(&mut e, list, 0);
        assert_eq!(e.get(list, ListBoxLayout::usNextIndex), 3);
        assert_eq!(e.get(list, ListBoxLayout::pHighlightedItem), Ptr::NULL);
        // a remembered index with no tile at that position (the filter drops
        // the last item and it was highlighted) falls back to `007d8ee0`
        list_vtable(&mut e, list);
        e.register_double(slot_fn(0x14), |_, a| {
            ret(if a[1] == 2 { 0 } else { 0x7777 })
        });
        stub(&mut e, LIST_COUNT, 2);
        e.set(list, ListBoxLayout::pHighlightedItem, Ptr::new(tiles[2]));
        e.register_double(0x6400_0000, |_, a| ret((a[0] == 0xd2) as u32));
        list_box_file_entry_filter(&mut e, list, 0x6400_0000);
        assert_eq!(
            e.get(list, ListBoxLayout::pHighlightedItem),
            Ptr::new(0x7777)
        );
    }

    #[test]
    fn the_file_entry_data_lookup_reads_the_item_data() {
        let mut e = engine();
        let list = Ptr::new(0x6100_0000);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + LIST_ITEM_DATA, 0xd9);
        stub(&mut e, FILE_ENTRY_LIST_GET_AT, item);
        assert_eq!(fn_007d86c0(&mut e, list, 3, 1), 0xd9);
        assert_eq!(
            calls(&e, FILE_ENTRY_LIST_GET_AT),
            vec![vec![list.addr(), 3, 1]]
        );
        stub(&mut e, FILE_ENTRY_LIST_GET_AT, 0);
        assert_eq!(fn_007d86c0(&mut e, list, 4, 0), 0);
    }

    #[test]
    fn the_file_entry_unfiltered_count_subtracts_the_filtered_value() {
        check_num_unfiltered(fn_007d8700, FILE_ENTRY_NUM_UNFILTERED_TRAIT);
    }

    #[test]
    fn the_visible_item_count_is_the_truncated_container_value() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        assert_eq!(fn_007d87b0(&mut e, list), 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        let container = tile(30);
        e.set(list, ListBoxLayout::pContainerRect, Ptr::new(container));
        stub(&mut e, TILE_ADD_USER_TRAIT, 0xabc);
        stub_float(&mut e, TILE_GET_FLOAT, 6.9);
        assert_eq!(fn_007d87b0(&mut e, list), 6);
        assert_eq!(
            calls(&e, TILE_ADD_USER_TRAIT),
            vec![vec![LIST_HEIGHT_TRAIT_NAME, u32::MAX]]
        );
        assert_eq!(calls(&e, TILE_GET_FLOAT), vec![vec![container, 0xabc]]);
    }

    #[test]
    fn file_entry_list_set_enabled_writes_the_trait() {
        check_set_enabled(
            list_box_file_entry_set_enabled,
            FILE_ENTRY_SET_ENABLED_TRAIT,
        );
    }

    #[test]
    fn file_entry_list_is_enabled_needs_a_container_with_the_trait() {
        check_is_enabled(list_box_file_entry_is_enabled, FILE_ENTRY_IS_ENABLED_TRAIT);
    }

    #[test]
    fn file_entry_list_highlight_checks_the_list_and_the_tile() {
        check_highlight(list_box_file_entry_highlight, 0x011d_aca8);
    }

    #[test]
    fn file_entry_list_highlight_target_steps_over_disabled_tiles() {
        check_highlight_target(list_box_file_entry_get_highlight_target);
    }

    #[test]
    fn file_entry_list_set_selection_is_mode_zero() {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        install_traits(&mut e, &[tile(41)], &[]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        assert!(list_box_file_entry_set_selection(
            &mut e,
            list,
            Ptr::new(tile(40))
        ));
        assert!(!list_box_file_entry_set_selection(
            &mut e,
            list,
            Ptr::new(tile(41))
        ));
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![tile(40), SELECTED_ID, 1]]
        );
    }

    #[test]
    fn deselecting_in_the_file_entry_list_is_mode_one() {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        install_traits(&mut e, &[tile(41)], &[]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        assert!(fn_007d8d80(&mut e, list, Ptr::new(tile(41))));
        assert!(!fn_007d8d80(&mut e, list, Ptr::new(tile(40))));
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![tile(41), SELECTED_ID, 0]]
        );
    }

    #[test]
    fn file_entry_list_deselect_all_can_skip_filtered_items() {
        check_deselect_all(list_box_file_entry_deselect_all);
    }

    #[test]
    fn emptying_the_twelve_byte_array_destroys_elements_and_may_free_the_buffer() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        fn_007d8e20(&mut e, array, 1);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.set(array, BSSimpleArray::pBuffer, 0x6200_0000);
        e.set(array, BSSimpleArray::iSize, 2);
        e.set(array, BSSimpleArray::iReservedSize, 8);
        fn_007d8e20(&mut e, array, 0);
        assert_eq!(
            calls(&e, ARRAY_ELEMENT_DESTROY),
            vec![vec![0x6200_0000, 0], vec![0x6200_000c, 0]]
        );
        assert!(calls(&e, ARRAY_BUFFER_RELEASE).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x6200_0000);
        e.set(array, BSSimpleArray::iSize, 1);
        fn_007d8e20(&mut e, array, 1);
        assert_eq!(calls(&e, ARRAY_BUFFER_RELEASE), vec![vec![array.addr()]]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
    }

    #[test]
    fn the_option_list_scalar_deleting_destructor_frees_on_request() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        assert_eq!(fn_007d8e80(&mut e, list, 0), list);
        assert_eq!(e.mem.u32(list.addr()), OPTION_LIST_BOX_VTABLE);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(fn_007d8e80(&mut e, list, 3), list);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![list.addr()]]);
    }

    #[test]
    fn the_file_entry_list_scalar_deleting_destructor_frees_on_request() {
        let mut e = engine();
        let list = e.new_object::<ListBoxLayout>();
        assert_eq!(
            list_box_file_entry_scalar_deleting_destructor(&mut e, list, 0),
            list
        );
        assert_eq!(e.mem.u32(list.addr()), FILE_ENTRY_LIST_BOX_VTABLE);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        list_box_file_entry_scalar_deleting_destructor(&mut e, list, 1);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![list.addr()]]);
    }

    #[test]
    fn the_last_unfiltered_tile_comes_from_the_virtual_tile_lookup() {
        let mut e = engine();
        let (list, container) = list_with_container(&mut e);
        install_traits(&mut e, &[], &[]);
        stub(&mut e, LIST_COUNT, 6);
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            float_ret(if a[0] == container && a[1] == FILTERED_ID {
                2.0
            } else {
                0.0
            })
        });
        list_vtable(&mut e, list);
        e.register_double(slot_fn(0x14), |_, a| ret(0x7000 + a[1]));
        // 6 items - 2 filtered = 4, the tile of index 3
        assert_eq!(fn_007d8ee0(&mut e, list), 0x7003);
        assert_eq!(calls(&e, slot_fn(0x14)), vec![vec![list.addr(), 3, 0]]);
    }

    #[test]
    fn option_list_synchronize_renumbers_the_unfiltered_items() {
        check_synchronize(list_box_option_synchronize);
    }

    #[test]
    fn deselecting_in_the_options_list_is_mode_one() {
        let mut e = engine();
        let (list, _) = list_with_container(&mut e);
        install_traits(&mut e, &[tile(41)], &[]);
        stub(&mut e, LIST_CONTAINS_TILE, 1);
        assert!(fn_007d8fe0(&mut e, list, Ptr::new(tile(41))));
        assert!(!fn_007d8fe0(&mut e, list, Ptr::new(tile(40))));
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![tile(41), SELECTED_ID, 0]]
        );
    }

    #[test]
    fn the_options_list_selection_function_handles_the_three_modes() {
        check_select(fn_007d9000, OPTION_SELECT_TRAIT);
    }

    #[test]
    fn option_list_update_highlight_box_pos_follows_the_highlight() {
        check_update_highlight_box_pos(
            list_box_option_update_highlight_box_pos,
            OPTION_HIGHLIGHT_Y_TRAIT,
            OPTION_SELECTED_HEIGHT_TRAIT,
        );
    }

    #[test]
    fn option_list_set_total_list_height_computes_the_scroll_steps() {
        check_set_total_list_height(list_box_option_set_total_list_height);
    }

    #[test]
    fn file_entry_list_synchronize_renumbers_the_unfiltered_items() {
        check_synchronize(list_box_file_entry_synchronize);
    }

    #[test]
    fn the_file_entry_list_selection_function_handles_the_three_modes() {
        check_select(fn_007d9660, FILE_ENTRY_SELECT_TRAIT);
    }

    #[test]
    fn file_entry_list_update_highlight_box_pos_follows_the_highlight() {
        check_update_highlight_box_pos(
            list_box_file_entry_update_highlight_box_pos,
            FILE_ENTRY_HIGHLIGHT_Y_TRAIT,
            FILE_ENTRY_SELECTED_HEIGHT_TRAIT,
        );
    }

    #[test]
    fn file_entry_list_set_total_list_height_computes_the_scroll_steps() {
        check_set_total_list_height(list_box_file_entry_set_total_list_height);
    }

    #[test]
    fn the_element_destroyer_runs_over_twelve_byte_elements() {
        let mut e = engine();
        fn_007d9e80(&mut e, Ptr::NULL, 0x6200_0000, 3);
        assert_eq!(
            calls(&e, ARRAY_ELEMENT_DESTROY),
            vec![
                vec![0x6200_0000, 0],
                vec![0x6200_000c, 0],
                vec![0x6200_0018, 0]
            ]
        );
        e.call_log = Some(Vec::new());
        fn_007d9e80(&mut e, Ptr::NULL, 0x6200_0000, 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn the_string_destructor_wrapper_forwards_this() {
        let mut e = engine();
        fn_007d9ec0(&mut e, Ptr::new(0x6200_0040));
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![0x6200_0040]]);
    }

    #[test]
    fn option_list_num_unfiltered_subtracts_the_filtered_value() {
        check_num_unfiltered(list_box_option_num_unfiltered, OPTION_NUM_UNFILTERED_TRAIT);
    }

    #[test]
    fn the_first_array_constructor_stores_its_vtable_and_clears_it() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(fn_007d9f90(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), OPTION_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_INIT), vec![vec![array.addr(), 0, 0]]);
    }

    #[test]
    fn the_first_array_scalar_deleting_destructor_frees_on_request() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(
            bs_simple_array_option_ptr_scalar_deleting_destructor(&mut e, array, 0),
            array
        );
        assert_eq!(calls(&e, ARRAY_DESTRUCT_BODY), vec![vec![array.addr(), 1]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        bs_simple_array_option_ptr_scalar_deleting_destructor(&mut e, array, 1);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![array.addr()]]);
    }

    #[test]
    fn the_first_array_destructor_stores_its_vtable_and_runs_the_body() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        fn_007d9ff0(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), OPTION_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_DESTRUCT_BODY), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn the_second_array_constructor_stores_its_vtable_and_clears_it() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(fn_007da010(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), USER_OPTION_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_INIT), vec![vec![array.addr(), 0, 0]]);
    }

    #[test]
    fn the_second_array_scalar_deleting_destructor_frees_on_request() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(
            bs_simple_array_user_option_ptr_scalar_deleting_destructor(&mut e, array, 0),
            array
        );
        assert_eq!(
            calls(&e, USER_OPTION_ARRAY_DESTRUCT),
            vec![vec![array.addr()]]
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        bs_simple_array_user_option_ptr_scalar_deleting_destructor(&mut e, array, 1);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![array.addr()]]);
    }
}
