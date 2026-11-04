//! `fallout/interface/menus/computersmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::startmenu::ListBox;

/// `00470470` (`thiscall`): frees the nodes of the list it is given and clears its head;
/// `0046ffb0` (`thiscall`) forwards to it.
const LIST_REMOVE_ALL_00470470: u32 = 0x0047_0470;
const LIST_DESTROY_0046FFB0: u32 = 0x0046_ffb0;
/// `004037d0` (`thiscall`): the string destructor (`BSStringT`).
const STRING_DESTROY_004037D0: u32 = 0x0040_37d0;

// Translated from 007593e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Font::TextData::~TextData` (Xbox PDB name from the engine map; `thiscall`, plain
/// `RET`): empties the list at +0x20 (`00470470`), destroys it (`0046ffb0`), then
/// destroys the string at +0 (`004037d0`).
// Translation incomplete: 007593e0 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the members) is not translated.
pub fn font_text_data_destructor(e: &mut Engine, this: Ptr) {
    let list = this.addr().wrapping_add(0x20);
    e.call(LIST_REMOVE_ALL_00470470, &args![list]);
    e.call(LIST_DESTROY_0046FFB0, &args![list]);
    e.call(STRING_DESTROY_004037D0, &args![this]);
}

// Translated from 00759450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): the float at +0x2c, loaded with `FLD` (which quiets a
/// signalling NaN) and returned in `ST0`.
pub fn fn_00759450(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x2c)))
}

// Translated from 00759d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): the byte at +0x29 in `AL` (callers test `AL` only).
pub fn fn_00759d80(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x29))
}

/// `ComputersMenu::pMe` (Xbox PDB static): the open computers menu.
const MENU_SINGLETON: u32 = 0x011d_9334;
/// The `AnimatingTextManager` singleton `007596d0` makes on first use.
const TEXT_MANAGER_SINGLETON: u32 = 0x011d_9338;
/// `ComputersMenu::SaveColor` (Xbox PDB static, an `NiColorA`): the colour the menu saves
/// and restores.
const SAVE_COLOR: u32 = 0x011d_934c;
/// A byte static: set by `DoClick` after a menu item that leaves the menu showing a note.
const NOTE_SHOWN_011D93C0: u32 = 0x011d_93c0;
/// The time (`00457fe0`) at which the intro ends.
const INTRO_END_TIME_011D93C4: u32 = 0x011d_93c4;
/// The cursor blink: next toggle time, interval (a function-local static, float) and the
/// guard of that static.
const BLINK_TIME_011D93C8: u32 = 0x011d_93c8;
const BLINK_INTERVAL_011D93CC: u32 = 0x011d_93cc;
const BLINK_GUARD_011D93D0: u32 = 0x011d_93d0;
/// The time before which the text manager does not move to the next item.
const NEXT_ITEM_TIME_011D93D4: u32 = 0x011d_93d4;
/// A word the text code reads (`1000 / setting`, a per-character time).
const TEXT_SPEED_0119F278: u32 = 0x0119_f278;
/// `IsEnabled`'s function-local static (`_enabled` trait id) and its guard.
const ENABLED_GUARD_011D93DC: u32 = 0x011d_93dc;
const ENABLED_TRAIT_011D93D8: u32 = 0x011d_93d8;
/// `UpdateHighlightBoxPos`'s statics: guard bits, `_highlight_y` and `_selected_height` ids.
const HIGHLIGHT_GUARD_011D93E8: u32 = 0x011d_93e8;
const HIGHLIGHT_Y_TRAIT_011D93E4: u32 = 0x011d_93e4;
const SELECTED_HEIGHT_TRAIT_011D93E0: u32 = 0x011d_93e0;
/// `NumUnfiltered`'s static (`_num_filtered` trait id) and its guard.
const NUM_FILTERED_GUARD_011D93F0: u32 = 0x011d_93f0;
const NUM_FILTERED_TRAIT_011D93EC: u32 = 0x011d_93ec;
/// A byte flag: while it is set, `RemoveAll` leaves the items' tiles alone.
const KEEP_ITEM_TILES_011D8909: u32 = 0x011d_8909;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;

/// `ComputersMenu` vtable and the `ListBox<TERMINAL_MENU_ITEM *>` vtable.
const COMPUTERS_MENU_VTABLE: u32 = 0x0107_2004;
const LIST_BOX_VTABLE: u32 = 0x0107_2148;
/// The menu class (`ComputersMenu::GetClass`).
const COMPUTERS_MENU_CLASS: u32 = 0x421;

/// `Menu::Menu` and `Menu::~Menu` (thiscall).
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `BSSimpleList` constructor (`0096a2d0`, thiscall).
const SIMPLE_LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// `BSStringT`: constructor (`004037b0`), from text (`0040c0e0`, text), assignment of text
/// (`00404820`, text), `Set` (`004037f0`, text, 0) and its length (`004048e0`). Its text is
/// the word at +0 (`00559450`, a generic first-word accessor).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_FROM_TEXT: u32 = 0x0040_c0e0;
const STRING_ASSIGN: u32 = 0x0040_4820;
const STRING_SET: u32 = 0x0040_37f0;
const FIRST_WORD: u32 = 0x0055_9450;
const STRING_LENGTH: u32 = 0x0040_48e0;
/// `sprintf` into a `BSStringT` (`00406f60`, cdecl: string, format, ...).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `` `eh vector constructor iterator' `` (`00ec782f`) and destructor iterator (`00ec5fce`).
const EH_VECTOR_CONSTRUCTOR: u32 = 0x00ec_782f;
const EH_VECTOR_DESTRUCTOR: u32 = 0x00ec_5fce;
/// `BSSoundHandle`: constructor, destructor, assignment (handle), `Play` (0), `PlayAfter`
/// (delay, 0), `Stop`, `Release`.
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_PLAY_AFTER: u32 = 0x00ad_8870;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
/// `BSAudioManager::QInstance` and `GetSoundHandleByName` (handle, name, flags).
const AUDIO_MANAGER: u32 = 0x00ad_9060;
const AUDIO_SOUND_BY_NAME: u32 = 0x00ae_5680;
/// `memset` (`00403d30`, cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// The game's allocator entry (`00401000`, cdecl: size) and `operator delete` (`00401030`).
const ALLOCATE: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `SystemColorManager::GetInstance`, `GetColor` (index, colour) and `SetColor`.
const COLOR_MANAGER: u32 = 0x0071_8b60;
const COLOR_GET: u32 = 0x0071_9060;
const COLOR_SET: u32 = 0x0071_90e0;
/// The `NiColorA` constructor (`00414430`: r, g, b, a).
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// `Tile::GetMenuByClass` (cdecl), `Tile::GetMaximumDepth`, `Tile::ReadFile`, `Tile::GetMenu`.
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// The function returning the interface manager (`004b7210`) and its tile reader (`00586150`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const TILE_READER: u32 = 0x0058_6150;
/// `Menu::SetMenuTile`, `PrepForVisibility`, `StartFadeOut`.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `Interface::IsTopMenuID` (cdecl: class).
const IS_TOP_MENU: u32 = 0x0070_2450;
/// The console print (`005b5e40`, cdecl: format, ...).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// Tile traits: float getter/setter, integer setter, string setter, value objects,
/// visibility, user traits, truth, text to trait, updates, search.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_UPDATE: u32 = 0x00a0_4640;
const TILE_UPDATE_CHILDREN: u32 = 0x00a0_4620;
const TILE_FIND: u32 = 0x00a0_8b20;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// The word at +0x28 (`0045cd60`: a tile's parent) and the word at +4 (`00726070`: a menu's
/// tile, a list node's next).
const WORD_AT_0X28: u32 = 0x0045_cd60;
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// The text of a `Tile::Value` (`0084e3a0`).
const VALUE_TEXT: u32 = 0x0084_e3a0;
/// `Tile::Value`: its float (`00488d50`, `ST0`), set float (float, 1), clear actions, add
/// (operator, float), add with a tile (operator, tile, trait), calculate (0).
const VALUE_CURRENT_00488D50: u32 = 0x0048_8d50;
const VALUE_SET_FLOAT: u32 = 0x00a0_a270;
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
const VALUE_ADD_ACTION: u32 = 0x00a0_9080;
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `InterfaceManager::TutorialManager::ShowMessage` and the tutorial state test (`00718840`).
const TUTORIAL_SHOW: u32 = 0x0071_8630;
const TUTORIAL_READY_00718840: u32 = 0x0071_8840;
/// `ListBox<unsigned int>::Initialize` (tile, template name).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// `BGSTerminal::IsUnlocked` (reference), `GetHackMiniGameValue` (reference),
/// `TERMINAL_MENU_ITEM::RunResultScript` (reference).
const TERMINAL_IS_UNLOCKED: u32 = 0x0050_1ae0;
const TERMINAL_HACK_VALUE: u32 = 0x0050_1270;
const RUN_RESULT_SCRIPT: u32 = 0x0050_1830;
/// The base form of a reference (`007af430`, the engine map's `BGSSaveFormBuffer::GetForm`).
const FORM_OF: u32 = 0x007a_f430;
/// `FORenderedTerminal::PlayShutdownSound` and `FadeOut`.
const PLAY_SHUTDOWN_SOUND: u32 = 0x007f_fe40;
const RENDERED_TERMINAL_FADE_OUT: u32 = 0x007f_faf0;
/// The interface manager's rendered terminal (`00707af0`), the reference it shows
/// (`0063d040`) and a tile's reference (`0056c7f0`).
const INTERFACE_TERMINAL_00707AF0: u32 = 0x0070_7af0;
const TERMINAL_REFERENCE_0063D040: u32 = 0x0063_d040;
const TILE_REFERENCE_0056C7F0: u32 = 0x0056_c7f0;
/// `__RTDynamicCast` (cdecl, five words) and the two type descriptors `Close` casts between.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_0119F43C: u32 = 0x0119_f43c;
const TYPE_0119F418: u32 = 0x0119_f418;
/// The current time in milliseconds (`00457fe0`).
const CURRENT_TIME: u32 = 0x0045_7fe0;
/// The address of a setting's value (`0043d4d0`), its string (`00403df0`) and its other
/// string accessor (`004c69f0`).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_STRING_004C69F0: u32 = 0x004c_69f0;
/// Text length (`0044a670`, cdecl).
const TEXT_LENGTH: u32 = 0x0044_a670;
/// `ListBox::ForEach` (callback, first index, last index).
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
/// Whether a tile is one of the list's (`007242f0`) and the item of a tile (`007e1110`).
const LIST_BOX_HAS_TILE: u32 = 0x0072_42f0;
const LIST_BOX_ITEM_OF: u32 = 0x007e_1110;
/// The list box's flag test (`007242c0`, flag number) and item count (`00726c40`).
const LIST_BOX_FLAG: u32 = 0x0072_42c0;
const LIST_BOX_COUNT: u32 = 0x0072_6c40;
/// Shared list box code: base constructor (`007240c0`), set-up (`007a1950`: 0, 1),
/// registration (`00721130`, cdecl) and unregistration (`007211c0`, cdecl).
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const LIST_BOX_SET_UP_007A1950: u32 = 0x007a_1950;
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// `ListBox<StartMenu::Option *>::InitTilePosition` (tile).
const LIST_BOX_INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// The flag byte of a highlighted tile (`0071aa80`) and the highlight marker (`0071aa60`).
const TILE_FLAG_0071AA80: u32 = 0x0071_aa80;
const TILE_MARK_0071AA60: u32 = 0x0071_aa60;
/// The scroll steps of a share (`004e4430`, cdecl: float), the limit (`00647b70`, cdecl) and
/// the rounding helper (`004bd510`, cdecl: float, float).
const SCROLL_STEPS: u32 = 0x004e_4430;
const STEP_LIMIT: u32 = 0x0064_7b70;
const SCROLL_ROUND: u32 = 0x004b_d510;
/// `_ftol2` (`ST0` passed as an `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// `ListBoxItem` constructor (tile, address of the object), the list's insert after a node
/// (item, node) and append by address.
const LIST_BOX_ITEM_CONSTRUCT: u32 = 0x007a_62e0;
const LIST_INSERT_AFTER: u32 = 0x007a_7eb0;
const LIST_APPEND: u32 = 0x0090_5820;
/// `Menu::RenderTemplate` (parent, template, 0).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// List node accessors: the address of the node's item (`006815c0`), its next setter
/// (`006ecd40`), its deleting destructor (`004702f0`); the method run on removed objects
/// (`004459e0`, 1).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_SET_NEXT: u32 = 0x006e_cd40;
const NODE_DELETE: u32 = 0x0047_02f0;
const OBJECT_RELEASE: u32 = 0x0044_59e0;
/// The list's sort (`0083fd60`, compare function).
const LIST_SORT: u32 = 0x0083_fd60;
/// The terminal stack's pop of its first item (`0063f7b0`) and append of the value at an
/// address (`005ae3d0`).
const LIST_POP_FRONT: u32 = 0x0063_f7b0;
const LIST_APPEND_ADDRESS: u32 = 0x005a_e3d0;
/// `TESCondition::IsTrue` (reference, 0).
const CONDITION_IS_TRUE: u32 = 0x0068_0c30;
/// The terminal's menu item list (`00461150`).
const TERMINAL_ITEMS: u32 = 0x0046_1150;
/// String compare (`00408b20`, cdecl).
const COMPARE_TEXT: u32 = 0x0040_8b20;
/// The player's note test (`00966c10`, note) and `PlayerCharacter::AddNote` (note, 0).
const PLAYER_HAS_NOTE: u32 = 0x0096_6c10;
const PLAYER_ADD_NOTE: u32 = 0x0096_6a70;
/// A note's type (`005e8d40`), its description object (`005e91d0`) and its name object
/// (`005e9160`).
const NOTE_TYPE: u32 = 0x005e_8d40;
const NOTE_DESCRIPTION: u32 = 0x005e_91d0;
const NOTE_NAME_OBJECT: u32 = 0x005e_9160;
/// The full name of a name component (`00408da0`).
const NAME_OF: u32 = 0x0040_8da0;
/// The HUD message (`007052f0`, cdecl: text, 0, icon, 0, time, 0).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `Font` access: the font manager (`005bd5b0`), a font by index (`005bd5c0`) and
/// `Font::PrepText` (text, text data).
const FONT_MANAGER: u32 = 0x005b_d5b0;
const FONT_MANAGER_FONT: u32 = 0x005b_d5c0;
const FONT_PREP_TEXT: u32 = 0x00a1_2fb0;
/// `Font::TextData` constructor (`00759330`: width, 0, 0, lines, 10); the object is 0x28
/// bytes on the caller's stack.
const TEXT_DATA_CONSTRUCT: u32 = 0x0075_9330;
/// The cdecl two-word function `007590c0` advances the note text by (`0042f5a0`).
const ADVANCE_0042F5A0: u32 = 0x0042_f5a0;
/// `AnimatingText`: constructor (text), set-up (value, value), the setter `00726db0` (0),
/// `Update`, `ForceFinish`, its tile (`007001f0`), its text (`00458b50`), the shown text
/// (`0059bb30`) and its sound delay (`0044edb0`).
const ANIMATING_TEXT_CONSTRUCT: u32 = 0x0075_9eb0;
const ANIMATING_TEXT_SET_UP: u32 = 0x006f_fe80;
const ANIMATING_TEXT_SET_00726DB0: u32 = 0x0072_6db0;
const ANIMATING_TEXT_UPDATE: u32 = 0x006f_fea0;
const ANIMATING_TEXT_FORCE_FINISH: u32 = 0x0070_0110;
const ANIMATING_TEXT_TILE: u32 = 0x0070_01f0;
const ANIMATING_TEXT_TEXT: u32 = 0x0045_8b50;
const ANIMATING_TEXT_SHOWN: u32 = 0x0059_bb30;
const ANIMATING_TEXT_DELAY: u32 = 0x0044_edb0;
/// The text manager's list: empty test (`0076b610`), first item address (`0073ac90`),
/// remove first (`007b5390`), count (`00620b80`), append (`004ed8c0`), next node
/// (`007b52d0`), insert before (`007b5330`), iterator step (`0057cbe0`), base set-up
/// (`0048f200`), base destructor (`004a1a30`), item destructor (`00726cf0`).
const TEXT_LIST_IS_EMPTY: u32 = 0x0076_b610;
const TEXT_LIST_HEAD: u32 = 0x0073_ac90;
const TEXT_LIST_REMOVE_HEAD: u32 = 0x007b_5390;
const TEXT_LIST_COUNT: u32 = 0x0062_0b80;
const TEXT_LIST_APPEND: u32 = 0x004e_d8c0;
const TEXT_LIST_NEXT: u32 = 0x007b_52d0;
const TEXT_LIST_INSERT_BEFORE: u32 = 0x007b_5330;
const TEXT_LIST_STEP: u32 = 0x0057_cbe0;
const TEXT_LIST_SET_UP: u32 = 0x0048_f200;
const TEXT_LIST_DESTRUCT: u32 = 0x004a_1a30;
const TEXT_ITEM_DESTRUCT: u32 = 0x0072_6cf0;

/// Strings in the exe.
const SOUND_FAN_HUM: u32 = 0x0107_1fec;
const SOUND_CHAR_SCROLL: u32 = 0x0107_1fd8;
const SOUND_CHAR_ENTER: u32 = 0x0107_1fc4;
const SOUND_CHAR_SINGLE: u32 = 0x0107_2130;
const MENU_FILE: u32 = 0x0107_20e8;
const MISSING_INDEX_MESSAGE: u32 = 0x0107_20a8;
const MISSING_TERMINAL_MESSAGE: u32 = 0x0107_2068;
const FILE_TEMPLATE: u32 = 0x0107_204c;
const BLINK_INTERVAL_TEXT: u32 = 0x0107_2108;
/// `"> "`: the prompt.
const PROMPT_TEXT: u32 = 0x0107_2118;
/// `"%s%s"`.
const TWO_STRINGS_FORMAT: u32 = 0x0101_996c;
const ENABLED_TEXT: u32 = 0x0107_0244;
const TEXT_WIDTH_TEXT: u32 = 0x0107_2124;
const TEXT_Y_TEXT: u32 = 0x0107_211c;
const NOTE_ICON: u32 = 0x0101_c140;
const TEXT_NUMBER_OF_VISIBLE_ITEMS: u32 = 0x0106_f860;
const TEXT_NUM_FILTERED: u32 = 0x0106_f8d0;
const TEXT_HIGHLIGHT_Y: u32 = 0x0106_f8c0;
const TEXT_SELECTED_HEIGHT: u32 = 0x0106_f8ac;
const TEXT_SCROLL_DELTA: u32 = 0x0106_f90c;
const TEXT_NUMBER_OF_ITEMS: u32 = 0x0106_f8f8;
const TEXT_LB_HIGHLIGHT_BOX: u32 = 0x0106_f898;
const TEXT_TOP: u32 = 0x0106_f894;
const TEXT_BOTTOM: u32 = 0x0106_f88c;
const TEXT_LEFT: u32 = 0x0106_f884;
const TEXT_RIGHT: u32 = 0x0106_f87c;
/// Settings (`Setting` objects) the menu reads; their names are not confirmed here.
const SETTING_CHAR_RATE_A: u32 = 0x011d_3de4;
const SETTING_CHAR_RATE_B: u32 = 0x011d_4e4c;
const SETTING_NOTE_RATE: u32 = 0x011d_4bb8;
const SETTING_RESULT_TIME: u32 = 0x011d_3e50;
const SETTING_NOTE_ADDED: u32 = 0x011d_3ca0;
const SETTING_BACK: u32 = 0x011d_2cb8;
const SETTING_INTRO_A: u32 = 0x011d_3a38;
const SETTING_INTRO_B: u32 = 0x011d_2ee0;
const SETTING_INTRO_C: u32 = 0x011d_430c;
const SETTING_WELCOME_A: u32 = 0x011d_3d78;
const SETTING_WELCOME_B: u32 = 0x011d_4978;
/// The table of setting pointers `00758fe0` indexes with the terminal's byte at +0xa6.
const SETTING_TABLE_01189E94: u32 = 0x0118_9e94;
/// Float and double constants in the exe's data.
const FADE_ALPHA_01016264: u32 = 0x0101_6264;
const MESSAGE_TIME: u32 = 0x0101_62c0;
const DEPTH_STACKING_A: u32 = 0x0107_04f0;
const DEPTH_STACKING_B: u32 = 0x0107_04e8;
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
const TWO: u32 = 0x0101_1590;

/// Tile traits by number.
const TRAIT_VISIBLE: u32 = 0xfa3;
const TRAIT_X: u32 = 0xfa1;
const TRAIT_Y: u32 = 0xfa2;
const TRAIT_STACKING_TYPE: u32 = 0xfa7;
const TRAIT_0XFA9: u32 = 0xfa9;
const TRAIT_ID: u32 = 0xfaa;
const TRAIT_LIST_INDEX: u32 = 0xfac;
const TRAIT_DEPTH: u32 = 0xfad;
const TRAIT_0XFAF: u32 = 0xfaf;
const TRAIT_HEIGHT: u32 = 0xfb0;
const TRAIT_WIDTH: u32 = 0xfb1;
const TRAIT_0XFB9: u32 = 0xfb9;
const TRAIT_0XFBA: u32 = 0xfba;
const TRAIT_0XFBC: u32 = 0xfbc;
const TRAIT_0XFC1: u32 = 0xfc1;
const TRAIT_STRING: u32 = 0xfc4;
const TRAIT_IMAGE: u32 = 0xfcc;
const TRAIT_0XFD6: u32 = 0xfd6;
const TRAIT_0X1004: u32 = 0x1004;
const TRAIT_0X1772: u32 = 0x1772;
/// Action operators: copy (2000) and add (2003).
const ACTION_COPY: u32 = 2000;
const ACTION_ADD: u32 = 0x7d3;

fn word(e: &Engine, object: u32, offset: u32) -> u32 {
    e.mem.u32(object.wrapping_add(offset))
}

fn set_word(e: &mut Engine, object: u32, offset: u32, value: u32) {
    e.mem.set_u32(object.wrapping_add(offset), value);
}

fn set_int(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

fn get_value(e: &mut Engine, tile: u32, trait_id: u32) -> u32 {
    e.call(TILE_GET_VALUE, &args![tile, trait_id]).u32()
}

fn get_value_q(e: &mut Engine, tile: u32, trait_id: u32) -> u32 {
    e.call(TILE_GET_VALUE_Q, &args![tile, trait_id]).u32()
}

fn get_float(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64()
}

fn is_visible(e: &mut Engine, tile: u32) -> bool {
    e.call(TILE_IS_VISIBLE, &args![tile]).bool()
}

fn add_user_trait(e: &mut Engine, name: u32) -> u32 {
    e.call(TILE_ADD_USER_TRAIT, &args![name, 0xffff_ffffu32])
        .u32()
}

fn first_word(e: &mut Engine, object: u32) -> u32 {
    e.call(FIRST_WORD, &args![object]).u32()
}

fn node_item(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_ITEM, &args![node]).u32()
}

fn word_at_four(e: &mut Engine, object: u32) -> u32 {
    e.call(WORD_AT_FOUR, &args![object]).u32()
}

fn now(e: &mut Engine) -> u32 {
    e.call(CURRENT_TIME, &args![]).u32()
}

/// `1000 / setting` (the setting's value through `0043d4d0`, read again for the division as
/// the code does), or 1000 when the setting is 0.
fn rate_of(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    if e.mem.u32(value) == 0 {
        1000
    } else {
        let value = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
        1000 / e.mem.u32(value)
    }
}

/// `pATManager` (+0xe8) of the menu.
fn text_manager(e: &Engine, this: u32) -> Ptr {
    Ptr::new(word(e, this, 0xe8))
}

/// `AnimatingTextManager::AddItem` on the menu's text manager: a text, the tile values for
/// the text and the visibility, and the index.
fn add_text(e: &mut Engine, this: u32, text: u32, value: u32, visible: u32, index: i32) {
    let manager = text_manager(e, this);
    computers_menu_animating_text_manager_add_item(e, manager, text, value, visible, index);
}

// Translated from 007577f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::ComputersMenu` (Xbox PDB; `thiscall`, plain `RET`): the `Menu` base, the
/// vtable, the list box (+0x68, `00759f90`), the terminal stack (+0x98), the note string
/// (+0xb4), three sound handles (+0xc4, through the vector constructor), the 16 tile slots
/// (+0x28) cleared, the text manager (+0xe8), the four note activation functions (+0xec:
/// `00759030`, `007590c0`, `00759470`, 0) and the three sounds (`UIHackingFanHumLP` 0x31,
/// `UIHackingCharScroll` 0x21, `UIHackingCharEnter` 0x21) assigned from temporary handles;
/// then clears +0xa0..+0xb0, +0xbc and the byte +0xc0. Returns `this`.
// Translation incomplete: 007577f0 - the C++ exception frame (FS:[0], unwind states 0 to 7 for the members and the temporary handles) is not translated.
pub fn computers_menu_computers_menu(e: &mut Engine, this: Ptr) -> Ptr {
    let this = this.addr();
    e.call(MENU_CONSTRUCTOR, &args![this]);
    set_word(e, this, 0, COMPUTERS_MENU_VTABLE);
    fn_00759f90(e, Ptr::new(this.wrapping_add(0x68)));
    e.call(SIMPLE_LIST_CONSTRUCTOR, &args![this.wrapping_add(0x98)]);
    e.call(STRING_CONSTRUCT, &args![this.wrapping_add(0xb4)]);
    e.call(
        EH_VECTOR_CONSTRUCTOR,
        &args![
            this.wrapping_add(0xc4),
            0xcu32,
            3u32,
            SOUND_HANDLE_CONSTRUCTOR,
            SOUND_HANDLE_DESTRUCTOR
        ],
    );
    e.call(MEMSET, &args![this.wrapping_add(0x28), 0u32, 0x40u32]);
    let manager = computers_menu_animating_text_manager_get_instance(e);
    set_word(e, this, 0xe8, manager);
    set_word(e, this, 0xec, 0x0075_9030);
    set_word(e, this, 0xf0, 0x0075_90c0);
    set_word(e, this, 0xf4, 0x0075_9470);
    set_word(e, this, 0xf8, 0);
    let audio = e.call(AUDIO_MANAGER, &args![]).u32();
    for (name, flags, offset) in [
        (SOUND_FAN_HUM, 0x31u32, 0xc4u32),
        (SOUND_CHAR_SCROLL, 0x21, 0xd0),
        (SOUND_CHAR_ENTER, 0x21, 0xdc),
    ] {
        e.with_stack(0xc, |e, handle| {
            let sound = e
                .call(AUDIO_SOUND_BY_NAME, &args![audio, handle, name, flags])
                .u32();
            e.call(
                SOUND_HANDLE_ASSIGN,
                &args![this.wrapping_add(offset), sound],
            );
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
        });
    }
    for offset in [0xa0u32, 0xa4, 0xa8, 0xac, 0xb0, 0xbc] {
        set_word(e, this, offset, 0);
    }
    e.mem.set_u8(this.wrapping_add(0xc0), 0);
    Ptr::new(this)
}

// Translated from 00757a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::GetClass` (Xbox PDB; `thiscall`): 0x421.
pub fn computers_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    COMPUTERS_MENU_CLASS
}

// Translated from 00757a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::`scalar deleting destructor'` (Xbox PDB; `thiscall`, `RET 4`): the
/// destructor (`00757a50`), then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn computers_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00757a50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00757a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ComputersMenu` destructor (unnamed in the engine map; `thiscall`): the vtable,
/// `pMe` cleared, the text manager freed (`00759780`), the terminal stack emptied, the three
/// sounds stopped and released, the saved colour restored (`SetColor(3, SaveColor)`), then the
/// members destroyed in reverse order and the `Menu` base.
// Translation incomplete: 00757a50 - the C++ exception frame (FS:[0], unwind states 4 to 0) is not translated.
pub fn fn_00757a50(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    set_word(e, this, 0, COMPUTERS_MENU_VTABLE);
    e.set_global(MENU_SINGLETON, 0u32);
    fn_00759780(e);
    e.call(LIST_REMOVE_ALL_00470470, &args![this.wrapping_add(0x98)]);
    for i in 0..3u32 {
        let handle = this.wrapping_add(i * 0xc).wrapping_add(0xc4);
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        let handle = this.wrapping_add(i * 0xc).wrapping_add(0xc4);
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    let colors = e.call(COLOR_MANAGER, &args![]).u32();
    e.call(COLOR_SET, &args![colors, 3u32, SAVE_COLOR]);
    e.call(
        EH_VECTOR_DESTRUCTOR,
        &args![
            this.wrapping_add(0xc4),
            0xcu32,
            3u32,
            SOUND_HANDLE_DESTRUCTOR
        ],
    );
    e.call(STRING_DESTROY_004037D0, &args![this.wrapping_add(0xb4)]);
    e.call(LIST_DESTROY_0046FFB0, &args![this.wrapping_add(0x98)]);
    fn_0075a060(e, Ptr::new(this.wrapping_add(0x68)));
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 00757b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::Create` (Xbox PDB; `cdecl`, one word: the terminal reference): closes an
/// open computers menu (its deleting destructor through slot 0), saves the colour
/// (`GetColor(3, SaveColor)`), sets it to opaque white for a terminal whose byte +0xa5 has bit
/// 2, reads `Data\Menus\computers_menu.xml` and, when the file's menu is of class 0x421,
/// attaches it, makes it `pMe` and checks the 16 tile slots (`00757e30`; prints the missing
/// index and returns 0 when one is empty). Without a reference it prints the missing-terminal
/// message and returns 0. Otherwise it sets the depth (trait 0xfad, when the stacking type is
/// 6006 or 102), the tutorial flag (+0xc0), the reference (+0xac) and terminal (+0xa8),
/// initializes the list box, plays the fan hum, runs the intro (`DisplayIntro`) or skips it
/// (`FinishIntro`) for an unlocked terminal, prepares the menu and returns the file's tile.
/// A file whose menu is of another class is deleted when its menu has a tile; 0 is returned.
pub fn computers_menu_create(e: &mut Engine, reference: u32) -> u32 {
    let class = fn_00757e20(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let colors = e.call(COLOR_MANAGER, &args![]).u32();
    e.call(COLOR_GET, &args![colors, 3u32, SAVE_COLOR]);
    let terminal = e.call(FORM_OF, &args![reference]).u32();
    if fn_00757e80(e, Ptr::new(terminal)) {
        e.with_stack(0x10, |e, color| {
            let one = 1.0f32.to_bits();
            let made = e
                .call(
                    COLOR_CONSTRUCT,
                    &args![color, one, one, one, 0.0f32.to_bits()],
                )
                .u32();
            let colors = e.call(COLOR_MANAGER, &args![]).u32();
            e.call(COLOR_SET, &args![colors, 3u32, made]);
        });
    }
    let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
    let depth = x87_store32(depth);
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    if menu != 0 && e.vcall(menu, 0x34, &args![]).u32() == fn_00757e20(e) {
        e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
        e.set_global(MENU_SINGLETON, menu);
        let missing = e.with_stack(4, |e, index| {
            let me = e.global::<u32>(MENU_SINGLETON);
            if fn_00757e30(e, Ptr::new(me), index) {
                None
            } else {
                Some(e.mem.u32(index.addr()))
            }
        });
        if let Some(index) = missing {
            e.call(PRINT_MESSAGE, &args![MISSING_INDEX_MESSAGE, index]);
            return 0;
        }
        if reference == 0 {
            e.call(PRINT_MESSAGE, &args![MISSING_TERMINAL_MESSAGE]);
            return 0;
        }
        // `FCOMP; TEST AH,0x44; JNP` (equal) on 6006, then `JP` (not equal) on 102.
        let stacking = get_float(e, tile, TRAIT_STACKING_TYPE);
        let set_depth = stacking == e.global::<f64>(DEPTH_STACKING_A)
            || get_float(e, tile, TRAIT_STACKING_TYPE) == e.global::<f64>(DEPTH_STACKING_B);
        if set_depth {
            e.call(TILE_SET_FLOAT, &args![tile, TRAIT_DEPTH, depth, 1u32]);
        }
        let class = fn_00757e20(e);
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let shown = e
            .call(
                TUTORIAL_SHOW,
                &args![interface.wrapping_add(0x4d4), 0x18u32, class, 0x200u32],
            )
            .u8();
        let me = e.global::<u32>(MENU_SINGLETON);
        e.mem.set_u8(me.wrapping_add(0xc0), shown);
        set_word(e, me, 0xac, reference);
        set_word(e, me, 0xa8, terminal);
        let list_tile = word(e, me, 0x2c);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![me.wrapping_add(0x68), list_tile, FILE_TEMPLATE],
        );
        let me = e.global::<u32>(MENU_SINGLETON);
        e.call(SOUND_HANDLE_PLAY, &args![me.wrapping_add(0xc4), 0u32]);
        let me = e.global::<u32>(MENU_SINGLETON);
        let reference = word(e, me, 0xac);
        let terminal = word(e, me, 0xa8);
        let unlocked = e
            .call(TERMINAL_IS_UNLOCKED, &args![terminal, reference])
            .bool();
        let me = Ptr::new(e.global::<u32>(MENU_SINGLETON));
        if unlocked {
            computers_menu_finish_intro(e, me);
        } else {
            computers_menu_display_intro(e, me);
        }
        e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
        return tile;
    }
    if menu != 0 && word_at_four(e, menu) != 0 && menu != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

// Translated from 00757e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no `this`, plain `RET`): the menu class 0x421 (the static `GetClass`).
pub fn fn_00757e20(_e: &mut Engine) -> u32 {
    COMPUTERS_MENU_CLASS
}

// Translated from 00757e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): checks the 16 tile slots (+0x28): counts `*index` up from 0
/// and returns false at the first empty slot (leaving its index), true when all are set.
pub fn fn_00757e30(e: &mut Engine, this: Ptr, index: Ptr) -> bool {
    e.mem.set_u32(index.addr(), 0);
    loop {
        let i = e.mem.u32(index.addr());
        if i as i32 >= 0x10 {
            return true;
        }
        let slot = this
            .addr()
            .wrapping_add(i.wrapping_mul(4))
            .wrapping_add(0x28);
        if e.mem.u32(slot) == 0 {
            return false;
        }
        let next = e.mem.u32(index.addr()).wrapping_add(1);
        e.mem.set_u32(index.addr(), next);
    }
}

// Translated from 00757e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a terminal): whether bit 2 of the byte at +0xa5 is set.
pub fn fn_00757e80(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0xa5)) & 4 != 0
}

// Translated from 00757ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::Close` (Xbox PDB; no arguments): with a menu open, plays the shutdown
/// sound, sets the menu tile's trait 0xfc1 to the float at `01016264`, fades out the
/// interface's rendered terminal when it shows this menu's reference (`__RTDynamicCast` of it,
/// then `FORenderedTerminal::FadeOut`), sets trait 0x1772 to 1 and starts the fade out.
pub fn computers_menu_close(e: &mut Engine) {
    if e.global::<u32>(MENU_SINGLETON) == 0 {
        return;
    }
    e.call(PLAY_SHUTDOWN_SOUND, &args![]);
    let alpha = x87_quiet_bits(e.global::<u32>(FADE_ALPHA_01016264));
    let me = e.global::<u32>(MENU_SINGLETON);
    let tile = word_at_four(e, me);
    e.call(TILE_SET_FLOAT, &args![tile, TRAIT_0XFC1, alpha, 1u32]);
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    if e.call(INTERFACE_TERMINAL_00707AF0, &args![interface]).u32() != 0 {
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let terminal = e.call(INTERFACE_TERMINAL_00707AF0, &args![interface]).u32();
        let shown = e.call(TERMINAL_REFERENCE_0063D040, &args![terminal]).u32();
        let me = e.global::<u32>(MENU_SINGLETON);
        let tile = word_at_four(e, me);
        let reference = e.call(TILE_REFERENCE_0056C7F0, &args![tile]).u32();
        if shown == reference {
            let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
            let terminal = e.call(INTERFACE_TERMINAL_00707AF0, &args![interface]).u32();
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![terminal, 0u32, TYPE_0119F43C, TYPE_0119F418, 0u32],
                )
                .u32();
            e.call(RENDERED_TERMINAL_FADE_OUT, &args![cast]);
        }
    }
    let me = e.global::<u32>(MENU_SINGLETON);
    let tile = word_at_four(e, me);
    set_int(e, tile, TRAIT_0X1772, 1);
    let me = e.global::<u32>(MENU_SINGLETON);
    e.call(MENU_START_FADE_OUT, &args![me]);
}

// Translated from 00757f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::DoClick` (Xbox PDB; `thiscall`, `RET 8`: id, tile). A tile of 0 with an
/// id in 0..16 becomes that slot's tile. Id 2 (the screen) finishes the text, then shows the
/// rest of a note (`007590c0`) or, with no note pending, hides tile +0x30, shows tile +0x2c
/// and either redraws the terminal (`007586e0`, after a note) or re-adds the prompt text and
/// redisplays the items (`ForEach(Redisplay)`, flag +0xc1); it plays the enter sound. Id 13
/// before the intro ends finishes it. Otherwise a tile of the list box selects its item:
/// without one the previous terminal is shown; a sub-menu item runs its script and opens its
/// terminal; other items run their script, show their result text (and set the result time
/// +0xa0), add their note (with a HUD message the first time) and run the note's activation
/// function, or redraw the terminal when the item asks. The note flag `011d93c0` is cleared
/// unless a note was shown.
// Translation incomplete: 00757f70 - the C++ exception frame (FS:[0], unwind state for the message string) is not translated.
pub fn computers_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let this = this.addr();
    let mut tile = tile;
    if tile == 0 && (0..0x10).contains(&id) {
        tile = word(e, this, (id as u32).wrapping_mul(4).wrapping_add(0x28));
    }
    let mut note_shown = false;
    if id == 2 {
        computers_menu_animating_text_manager_force_finish(e, text_manager(e, this));
        if word(e, this, 0xbc) != 0 {
            fn_007590c0(e, Ptr::new(this), 0);
            note_shown = true;
        } else {
            let hidden = word(e, this, 0x30);
            set_int(e, hidden, TRAIT_VISIBLE, 0);
            let shown = word(e, this, 0x2c);
            set_int(e, shown, TRAIT_VISIBLE, 1);
            if e.global::<u8>(NOTE_SHOWN_011D93C0) != 0 {
                fn_007586e0(e, Ptr::new(this), 0);
            } else {
                let prompt = word(e, this, 0x44);
                if !is_visible(e, prompt) {
                    let cursor = word(e, this, 0x64);
                    set_int(e, cursor, TRAIT_VISIBLE, 0);
                    let prompt = word(e, this, 0x44);
                    let value = get_value(e, prompt, TRAIT_STRING);
                    let prompt = word(e, this, 0x44);
                    let visible = get_value_q(e, prompt, TRAIT_VISIBLE);
                    let text = e.call(VALUE_TEXT, &args![value]).u32();
                    add_text(e, this, text, value, visible, -1);
                }
                e.call(
                    LIST_BOX_FOR_EACH,
                    &args![
                        this.wrapping_add(0x68),
                        0x0075_9560u32,
                        0u32,
                        0x7fff_ffffu32
                    ],
                );
                e.mem.set_u8(this.wrapping_add(0xc1), 1);
            }
        }
        e.call(SOUND_HANDLE_PLAY, &args![this.wrapping_add(0xdc), 0u32]);
    } else if id == 0xd && (word(e, this, 0xb0) as i32) < 2 {
        computers_menu_finish_intro(e, Ptr::new(this));
    } else if e
        .call(LIST_BOX_HAS_TILE, &args![this.wrapping_add(0x68), tile])
        .bool()
    {
        e.call(SOUND_HANDLE_PLAY, &args![this.wrapping_add(0xdc), 0u32]);
        let item = e
            .call(LIST_BOX_ITEM_OF, &args![this.wrapping_add(0x68), tile])
            .u32();
        if item == 0 {
            computers_menu_display_previous_terminal(e, Ptr::new(this));
        } else if word(e, item, 0x70) != 0 {
            fn_007583b0(e, Ptr::new(item), 1);
            let reference = word(e, this, 0xac);
            e.call(RUN_RESULT_SCRIPT, &args![item, reference]);
            let sub_menu = word(e, item, 0x70);
            set_word(e, this, 0xa8, sub_menu);
            fn_007586e0(e, Ptr::new(this), 1);
        } else {
            fn_007583b0(e, Ptr::new(item), 1);
            let reference = word(e, this, 0xac);
            e.call(RUN_RESULT_SCRIPT, &args![item, reference]);
            if first_word(e, item.wrapping_add(8)) != 0 {
                let result = word(e, this, 0x3c);
                let visible = get_value_q(e, result, TRAIT_VISIBLE);
                let result = word(e, this, 0x3c);
                let value = get_value(e, result, TRAIT_STRING);
                let text = first_word(e, item.wrapping_add(8));
                add_text(e, this, text, value, visible, 0);
                let time = now(e);
                let seconds = e
                    .call(SETTING_VALUE_ADDRESS, &args![SETTING_RESULT_TIME])
                    .u32();
                let seconds = e.mem.u32(seconds);
                set_word(e, this, 0xa0, time.wrapping_add(seconds.wrapping_mul(1000)));
            }
            if word(e, item, 0x6c) != 0 {
                if fn_00758350(e, Ptr::new(item)) {
                    let note = word(e, item, 0x6c);
                    let player = e.global::<u32>(PLAYER);
                    if !e.call(PLAYER_HAS_NOTE, &args![player, note]).bool() {
                        let note = word(e, item, 0x6c);
                        let player = e.global::<u32>(PLAYER);
                        e.call(PLAYER_ADD_NOTE, &args![player, note, 0u32]);
                        e.with_stack(8, |e, message| {
                            e.call(STRING_CONSTRUCT, &args![message]);
                            let note = word(e, item, 0x6c);
                            let name = e.call(NAME_OF, &args![note.wrapping_add(0x48)]).u32();
                            let format = e.call(SETTING_STRING, &args![SETTING_NOTE_ADDED]).u32();
                            e.call(STRING_FORMAT, &args![message, format, name]);
                            let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
                            let text = first_word(e, message.addr());
                            e.call(
                                SHOW_MESSAGE,
                                &args![text, 0u32, NOTE_ICON, 0u32, time, 0u32],
                            );
                            e.call(STRING_DESTROY_004037D0, &args![message]);
                        });
                    }
                }
                let note = word(e, item, 0x6c);
                let kind = e.call(NOTE_TYPE, &args![note]).u32();
                if word(e, this, kind.wrapping_mul(4).wrapping_add(0xec)) != 0 {
                    let note = word(e, item, 0x6c);
                    let note_again = word(e, item, 0x6c);
                    let kind = e.call(NOTE_TYPE, &args![note_again]).u32();
                    let function = word(e, this, kind.wrapping_mul(4).wrapping_add(0xec));
                    e.call(function, &args![this, note]);
                }
                let shows = fn_00758390(e, Ptr::new(item));
                e.set_global(NOTE_SHOWN_011D93C0, u8::from(shows));
                note_shown = true;
            } else if fn_00758390(e, Ptr::new(item)) {
                fn_007586e0(e, Ptr::new(this), 0);
            }
        }
    }
    if !note_shown {
        e.set_global(NOTE_SHOWN_011D93C0, 0u8);
    }
}

// Translated from 00758350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `TERMINAL_MENU_ITEM`): whether it has a note (+0x6c) and bit 0 of
/// its flags byte (+0x74) is set.
pub fn fn_00758350(e: &mut Engine, this: Ptr) -> bool {
    word(e, this.addr(), 0x6c) != 0 && e.mem.u8(this.addr().wrapping_add(0x74)) & 1 != 0
}

// Translated from 00758390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `TERMINAL_MENU_ITEM`): whether bit 1 of its flags byte (+0x74) is
/// set (`NEG; SBB; NEG`: 0 or 1 in `EAX`).
pub fn fn_00758390(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x74)) & 2 != 0
}

// Translated from 007583b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): sets or clears bit 2 of the flags byte (+0x74).
pub fn fn_007583b0(e: &mut Engine, this: Ptr, set: u8) {
    let field = this.addr().wrapping_add(0x74);
    let flags = e.mem.u8(field);
    let flags = if set != 0 { flags | 4 } else { flags & 0xfb };
    e.mem.set_u8(field, flags);
}

// Translated from 007583f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::DoGamepadUpEvent` (Xbox PDB; `thiscall`, `RET 8`: button, an unread word):
/// with this menu on top, button 10 either clicks the screen (id 2, after clearing the pending
/// note +0xbc) while tile +0x30 is visible, or shows the previous terminal; true then.
pub fn computers_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr,
    button: i32,
    _unused_2: u32,
) -> bool {
    if !e.call(IS_TOP_MENU, &args![COMPUTERS_MENU_CLASS]).bool() || button != 10 {
        return false;
    }
    let tile = word(e, this.addr(), 0x30);
    if is_visible(e, tile) {
        set_word(e, this.addr(), 0xbc, 0);
        e.vcall(this.addr(), 0xc, &args![2u32, 0u32]);
    } else {
        computers_menu_display_previous_terminal(e, this);
    }
    true
}

// Translated from 00758470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::DoIdle` (Xbox PDB; `thiscall`): waits while the tutorial (+0xc0) is not
/// ready and while fading out (+0x24 == 2); otherwise updates the text and the cursor
/// (`007585a0`) and, with no text left: starts the intro timer (2000 ms, mode 0 -> 1), ends
/// the intro when it runs out (mode 1), or enables the items after a redisplay (+0xc1). A
/// result text whose time (+0xa0) has passed is hidden.
pub fn computers_menu_do_idle(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    if e.mem.u8(this.wrapping_add(0xc0)) != 0 {
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        if !e
            .call(
                TUTORIAL_READY_00718840,
                &args![interface.wrapping_add(0x4d4), 0x18u32],
            )
            .bool()
        {
            return;
        }
    }
    if word(e, this, 0x24) == 2 {
        return;
    }
    computers_menu_animating_text_manager_update(e, text_manager(e, this));
    fn_007585a0(e, Ptr::new(this));
    let manager = text_manager(e, this);
    if e.call(TEXT_LIST_COUNT, &args![manager]).u32() == 0 {
        let mode = word(e, this, 0xb0) as i32;
        if mode < 2 {
            if mode == 0 {
                let time = now(e).wrapping_add(2000);
                e.set_global(INTRO_END_TIME_011D93C4, time);
                set_word(e, this, 0xb0, 1);
            } else if mode == 1 && now(e) > e.global::<u32>(INTRO_END_TIME_011D93C4) {
                computers_menu_finish_intro(e, Ptr::new(this));
            }
        } else if e.mem.u8(this.wrapping_add(0xc1)) != 0 {
            e.call(
                LIST_BOX_FOR_EACH,
                &args![
                    this.wrapping_add(0x68),
                    0x0075_96a0u32,
                    0u32,
                    0x7fff_ffffu32
                ],
            );
            e.mem.set_u8(this.wrapping_add(0xc1), 0);
        }
    }
    if word(e, this, 0xa0) != 0 && now(e) > word(e, this, 0xa0) {
        set_word(e, this, 0xa0, 0);
        let result = word(e, this, 0x3c);
        set_int(e, result, TRAIT_VISIBLE, 0);
    }
}

// Translated from 007585a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the cursor blink. On first use the interval is read from tile +0x40's
/// `_blink_interval` (a function-local static). Once the time passes the next toggle and the
/// cursor value (+0xa4) is known, the value flips between 0 and 1 (1 unless it is exactly 1)
/// and the next toggle is `time + interval` (`FISTP qword` with chop, the low word); without a
/// value it is fetched from tile +0x40's visibility.
// Translation incomplete: 007585a0 - the C++ exception frame (FS:[0]) around the static's initialisation is not translated.
pub fn fn_007585a0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    if e.global::<u32>(BLINK_GUARD_011D93D0) & 1 == 0 {
        let guard = e.global::<u32>(BLINK_GUARD_011D93D0) | 1;
        e.set_global(BLINK_GUARD_011D93D0, guard);
        let id = add_user_trait(e, BLINK_INTERVAL_TEXT);
        let cursor = word(e, this, 0x40);
        let interval = get_float(e, cursor, id);
        e.set_global(BLINK_INTERVAL_011D93CC, x87_store32(interval));
    }
    let time = now(e);
    if time > e.global::<u32>(BLINK_TIME_011D93C8) && word(e, this, 0xa4) != 0 {
        let value = word(e, this, 0xa4);
        let current = e.call(VALUE_CURRENT_00488D50, &args![value]).f64();
        // `FCOMP; TEST AH,0x44; JP`: 0.0 only when exactly equal to 1.0.
        let next = if current == e.global::<f64>(ONE) {
            0.0f32
        } else {
            1.0f32
        };
        let value = word(e, this, 0xa4);
        e.call(VALUE_SET_FLOAT, &args![value, next.to_bits(), 1u32]);
        let time = now(e);
        let sum = x87_add(
            f64::from(time),
            x87_m32(e.global::<u32>(BLINK_INTERVAL_011D93CC)),
        );
        let next_time = x87_fistp64_chop(X87Ext::load64(sum)) as u32;
        e.set_global(BLINK_TIME_011D93C8, next_time);
    } else if word(e, this, 0xa4) == 0 {
        let cursor = word(e, this, 0x40);
        let value = get_value_q(e, cursor, TRAIT_VISIBLE);
        set_word(e, this, 0xa4, value);
    }
}

// Translated from 007586e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: whether to push the terminal): shows the current terminal
/// (+0xa8). Finishes the text, empties the list box (its slot +0x1c), pushes the terminal on
/// the stack (+0x98) when asked, sets the text speed (`1000 / setting`), shows the terminal's
/// description (`DESC`, through the description's slot +0x10) in tile +0x44 unless it already
/// shows it, then adds an item and a text line for each menu item whose conditions hold for the
/// reference (+0xac; the line's `0xfd6` is 2 for an item already visited, else 1) and clears
/// each item's visited bit; with a previous terminal on the stack, a "back" line follows. Sets
/// +0xc1.
// Translation incomplete: 007586e0 - the C++ exception frame (FS:[0], unwind state for the line string) is not translated.
pub fn fn_007586e0(e: &mut Engine, this: Ptr, push: u8) {
    let this = this.addr();
    if word(e, this, 0xa8) == 0 {
        return;
    }
    e.with_stack(8, |e, line| {
        e.call(STRING_CONSTRUCT, &args![line]);
        computers_menu_animating_text_manager_force_finish(e, text_manager(e, this));
        e.vcall(this.wrapping_add(0x68), 0x1c, &args![]);
        if push != 0 {
            e.call(
                LIST_APPEND_ADDRESS,
                &args![this.wrapping_add(0x98), this.wrapping_add(0xa8)],
            );
        }
        let speed = rate_of(e, SETTING_CHAR_RATE_A);
        e.set_global(TEXT_SPEED_0119F278, speed);
        let terminal = word(e, this, 0xa8);
        let description = e
            .vcall(
                terminal.wrapping_add(0x90),
                0x10,
                &args![terminal, 0x4353_4544u32],
            )
            .u32();
        let screen = word(e, this, 0x44);
        let value = get_value(e, screen, TRAIT_STRING);
        if description != 0 && e.mem.u8(description) != 0 {
            let screen = word(e, this, 0x44);
            let same = if is_visible(e, screen) {
                let shown = e.call(VALUE_TEXT, &args![value]).u32();
                e.call(COMPARE_TEXT, &args![description, shown]).u32() == 0
            } else {
                false
            };
            if !same {
                let cursor = word(e, this, 0x64);
                set_int(e, cursor, TRAIT_VISIBLE, 0);
                let screen = word(e, this, 0x44);
                let visible = get_value_q(e, screen, TRAIT_VISIBLE);
                add_text(e, this, description, value, visible, -1);
            }
        }
        let terminal = word(e, this, 0xa8);
        let mut node = e.call(TERMINAL_ITEMS, &args![terminal]).u32();
        while node != 0 {
            let slot = node_item(e, node);
            if e.mem.u32(slot) == 0 {
                break;
            }
            let reference = word(e, this, 0xac);
            let slot = node_item(e, node);
            let item = e.mem.u32(slot);
            if e.call(
                CONDITION_IS_TRUE,
                &args![item.wrapping_add(0x64), reference, 0u32],
            )
            .bool()
            {
                let slot = node_item(e, node);
                let item = e.mem.u32(slot);
                let tile = fn_0075a110(e, Ptr::new(this.wrapping_add(0x68)), item, 0, 0, 0);
                let enabled = add_user_trait(e, ENABLED_TEXT);
                set_int(e, tile, enabled, 0);
                let slot = node_item(e, node);
                let item = e.mem.u32(slot);
                let visited = fn_00758a60(e, Ptr::new(item));
                set_int(e, tile, TRAIT_0XFD6, u32::from(visited) + 1);
                let slot = node_item(e, node);
                let item = e.mem.u32(slot);
                let name = first_word(e, item);
                e.call(
                    STRING_FORMAT,
                    &args![line, TWO_STRINGS_FORMAT, PROMPT_TEXT, name],
                );
                let visible = get_value(e, tile, TRAIT_VISIBLE);
                let value = get_value(e, tile, TRAIT_0X1004);
                let text = first_word(e, line.addr());
                add_text(e, this, text, value, visible, -1);
            }
            let slot = node_item(e, node);
            let item = e.mem.u32(slot);
            fn_007583b0(e, Ptr::new(item), 0);
            node = word_at_four(e, node);
        }
        if word_at_four(e, this.wrapping_add(0x98)) != 0 {
            let tile = fn_0075a110(e, Ptr::new(this.wrapping_add(0x68)), 0, 0, 0, 0);
            let enabled = add_user_trait(e, ENABLED_TEXT);
            set_int(e, tile, enabled, 0);
            let back = e.call(SETTING_STRING, &args![SETTING_BACK]).u32();
            e.call(
                STRING_FORMAT,
                &args![line, TWO_STRINGS_FORMAT, PROMPT_TEXT, back],
            );
            let visible = get_value(e, tile, TRAIT_VISIBLE);
            let value = get_value(e, tile, TRAIT_0X1004);
            let text = first_word(e, line.addr());
            add_text(e, this, text, value, visible, -1);
        }
        e.mem.set_u8(this.wrapping_add(0xc1), 1);
        e.call(STRING_DESTROY_004037D0, &args![line]);
    });
}

// Translated from 00758a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `TERMINAL_MENU_ITEM`): whether bit 2 of its flags byte (+0x74)
/// is set.
pub fn fn_00758a60(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x74)) & 4 != 0
}

// Translated from 00758a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::DisplayPreviousTerminal` (Xbox PDB; `thiscall`): pops the terminal stack
/// (+0x98) and shows its new first terminal, or closes the menu when there is none.
pub fn computers_menu_display_previous_terminal(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.call(LIST_POP_FRONT, &args![this.wrapping_add(0x98)]);
    let slot = node_item(e, this.wrapping_add(0x98));
    let terminal = e.mem.u32(slot);
    set_word(e, this, 0xa8, terminal);
    if word(e, this, 0xa8) != 0 {
        fn_007586e0(e, Ptr::new(this), 0);
    } else {
        computers_menu_close(e);
    }
}

// Translated from 00758ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::DisplayIntro` (Xbox PDB; `thiscall`): the four intro lines of tiles +0x4c,
/// +0x50, +0x54 and +0x58, alternating the two text speeds: two setting strings, the prompt
/// followed by a setting string, and the prompt followed by one `*` per point of the
/// terminal's hacking value (`memset` into a 16-byte buffer).
// Translation incomplete: 00758ad0 - the C++ exception frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
pub fn computers_menu_display_intro(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(0x18, |e, locals| {
        // The line string at +0, the 16-byte buffer at +8.
        let line = locals.addr();
        let stars = line.wrapping_add(8);
        e.call(STRING_CONSTRUCT, &args![line]);
        let fast = rate_of(e, SETTING_CHAR_RATE_A);
        let slow = rate_of(e, SETTING_CHAR_RATE_B);
        e.set_global(TEXT_SPEED_0119F278, fast);
        let tile = word(e, this, 0x4c);
        let visible = get_value_q(e, tile, TRAIT_VISIBLE);
        let value = get_value(e, tile, TRAIT_STRING);
        let text = e
            .call(SETTING_STRING_004C69F0, &args![SETTING_INTRO_A])
            .u32();
        add_text(e, this, text, value, visible, -1);
        e.set_global(TEXT_SPEED_0119F278, slow);
        let setting = e.call(SETTING_STRING, &args![SETTING_INTRO_B]).u32();
        e.call(
            STRING_FORMAT,
            &args![line, TWO_STRINGS_FORMAT, PROMPT_TEXT, setting],
        );
        let tile = word(e, this, 0x50);
        let visible = get_value_q(e, tile, TRAIT_VISIBLE);
        let value = get_value(e, tile, TRAIT_STRING);
        let text = first_word(e, line);
        add_text(e, this, text, value, visible, -1);
        e.set_global(TEXT_SPEED_0119F278, fast);
        let tile = word(e, this, 0x54);
        let visible = get_value_q(e, tile, TRAIT_VISIBLE);
        let value = get_value(e, tile, TRAIT_STRING);
        let text = e
            .call(SETTING_STRING_004C69F0, &args![SETTING_INTRO_C])
            .u32();
        add_text(e, this, text, value, visible, -1);
        e.set_global(TEXT_SPEED_0119F278, slow);
        e.call(MEMSET, &args![stars, 0u32, 0x10u32]);
        let reference = word(e, this, 0xac);
        let terminal = word(e, this, 0xa8);
        let count = e
            .call(TERMINAL_HACK_VALUE, &args![terminal, reference])
            .u32();
        e.call(MEMSET, &args![stars, 0x2au32, count]);
        e.call(
            STRING_FORMAT,
            &args![line, TWO_STRINGS_FORMAT, PROMPT_TEXT, stars],
        );
        let tile = word(e, this, 0x58);
        let visible = get_value_q(e, tile, TRAIT_VISIBLE);
        let value = get_value(e, tile, TRAIT_STRING);
        let text = first_word(e, line);
        add_text(e, this, text, value, visible, -1);
        e.call(STRING_DESTROY_004037D0, &args![line]);
    });
}

// Translated from 00758d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::FinishIntro` (Xbox PDB; `thiscall`): hides tile +0x5c's 0xfaf and the four
/// intro tiles, finishes the text, shows the terminal (pushing it), adds the three centred
/// welcome lines (two settings and the terminal's name of `00758fe0`) at indices 0, 1 and 2,
/// the prompt in tile +0x48, and sets the mode (+0xb0) to 2.
pub fn computers_menu_finish_intro(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let tile = word(e, this, 0x5c);
    set_int(e, tile, TRAIT_0XFAF, 0);
    computers_menu_animating_text_manager_force_finish(e, text_manager(e, this));
    for offset in [0x4cu32, 0x50, 0x54, 0x58] {
        let tile = word(e, this, offset);
        set_int(e, tile, TRAIT_VISIBLE, 0);
    }
    fn_007586e0(e, Ptr::new(this), 1);
    let tile = word(e, this, 0x34);
    let visible = get_value_q(e, tile, TRAIT_VISIBLE);
    let tile = word(e, this, 0x34);
    let value = get_value(e, tile, TRAIT_STRING);
    let text = e
        .call(SETTING_STRING_004C69F0, &args![SETTING_WELCOME_A])
        .u32();
    add_text(e, this, text, value, visible, 0);
    centre_tile(e, this, 0x34, |e| {
        e.call(SETTING_STRING, &args![SETTING_WELCOME_A]).u32()
    });
    let tile = word(e, this, 0x38);
    let visible = get_value_q(e, tile, TRAIT_VISIBLE);
    let tile = word(e, this, 0x38);
    let value = get_value(e, tile, TRAIT_STRING);
    let text = e
        .call(SETTING_STRING_004C69F0, &args![SETTING_WELCOME_B])
        .u32();
    add_text(e, this, text, value, visible, 1);
    centre_tile(e, this, 0x38, |e| {
        e.call(SETTING_STRING, &args![SETTING_WELCOME_B]).u32()
    });
    let tile = word(e, this, 0x28);
    let visible = get_value_q(e, tile, TRAIT_VISIBLE);
    let tile = word(e, this, 0x28);
    let value = get_value(e, tile, TRAIT_STRING);
    let terminal = word(e, this, 0xa8);
    let text = fn_00758fe0(e, Ptr::new(terminal));
    add_text(e, this, text, value, visible, 2);
    centre_tile(e, this, 0x28, |e| {
        let terminal = word(e, this, 0xa8);
        fn_00758fe0(e, Ptr::new(terminal))
    });
    let tile = word(e, this, 0x48);
    let visible = get_value_q(e, tile, TRAIT_VISIBLE);
    let tile = word(e, this, 0x48);
    let value = get_value(e, tile, TRAIT_STRING);
    add_text(e, this, PROMPT_TEXT, value, visible, -1);
    set_word(e, this, 0xb0, 2);
}

/// `FinishIntro`'s centring of one tile: the parent's width (0xfb1) is read, then the text
/// whose length counts (`measured`, called after the width as the code does), and the tile's x
/// (0xfa1) becomes `(width - 17 * length) / 2` (`FILD qword`, `FSUBR`, `FDIV` by the double
/// 2.0, stored as a float).
fn centre_tile(e: &mut Engine, this: u32, offset: u32, measured: impl FnOnce(&mut Engine) -> u32) {
    let tile = word(e, this, offset);
    let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
    let width = get_float(e, parent, TRAIT_WIDTH);
    let text = measured(e);
    let length = e.call(TEXT_LENGTH, &args![text]).u32();
    let advance = f64::from(length.wrapping_mul(0x11));
    let x = x87_store32(x87_div(x87_sub(width, advance), e.global::<f64>(TWO)));
    let tile = word(e, this, offset);
    e.call(TILE_SET_FLOAT, &args![tile, TRAIT_X, x, 1u32]);
}

// Translated from 00758fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a terminal): the string of the setting the table at `01189e94`
/// holds for the terminal's byte +0xa6 (below 10), else 0.
pub fn fn_00758fe0(e: &mut Engine, this: Ptr) -> u32 {
    let index = u32::from(e.mem.u8(this.addr().wrapping_add(0xa6)));
    if index < 10 {
        let setting = e
            .mem
            .u32(index.wrapping_mul(4).wrapping_add(SETTING_TABLE_01189E94));
        e.call(SETTING_STRING, &args![setting]).u32()
    } else {
        0
    }
}

// Translated from 007590c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a note): the note activation function of text notes, which
/// shows the note a page at a time. Finishes the text, sets the text speed, hides tile +0x2c
/// and shows tile +0x30 (0xfa9 cleared); on the first page copies the note's text (`TNAM`,
/// through its description's slot +0x10) into the menu's string (+0xb4) and points +0xbc at
/// it. The page holds `trunc(height - width / line height) + 1` lines of the font
/// `trunc(0xfb9) - 1` (x87 as the code: `FDIVR`, `FSUBR`, `_ftol2`); the text is laid out with
/// `Font::PrepText` into a `Font::TextData` on the stack, added to the text manager, and
/// +0xbc moves on by `0042f5a0(laid-out length, string length)`, becoming 0 at the end.
// Translation incomplete: 007590c0 - the C++ exception frame (FS:[0], unwind state for the text data) is not translated.
pub fn fn_007590c0(e: &mut Engine, this: Ptr, note: u32) {
    let this = this.addr();
    computers_menu_animating_text_manager_force_finish(e, text_manager(e, this));
    let speed = rate_of(e, SETTING_NOTE_RATE);
    e.set_global(TEXT_SPEED_0119F278, speed);
    let tile = word(e, this, 0x2c);
    set_int(e, tile, TRAIT_VISIBLE, 0);
    let tile = word(e, this, 0x30);
    set_int(e, tile, TRAIT_0XFA9, 0);
    let tile = word(e, this, 0x30);
    set_int(e, tile, TRAIT_VISIBLE, 1);
    if word(e, this, 0xbc) == 0 {
        let description = e.call(NOTE_DESCRIPTION, &args![note]).u32();
        let text = e
            .vcall(description, 0x10, &args![note, 0x4d41_4e54u32])
            .u32();
        e.call(STRING_SET, &args![this.wrapping_add(0xb4), text, 0u32]);
        let text = first_word(e, this.wrapping_add(0xb4));
        set_word(e, this, 0xbc, text);
    }
    let page = word(e, this, 0x60);
    let available = get_float(e, page, TRAIT_0XFBC);
    let screen = word(e, this, 0x44);
    let extent = get_float(e, screen, TRAIT_HEIGHT);
    let screen = word(e, this, 0x44);
    let font_number = get_float(e, screen, TRAIT_0XFB9);
    let font_index = e.call(FTOL, &args![font_number]).u32().wrapping_sub(1);
    let fonts = e.call(FONT_MANAGER, &args![]).u32();
    let font = e.call(FONT_MANAGER_FONT, &args![fonts, font_index]).u32();
    let line_height = f64::from(fn_00759450(e, Ptr::new(font)));
    // `FDIVR` then `FSUBR`, both on doubles the code stored: `available - extent / height`.
    let lines = x87_sub(available, x87_div(extent, line_height));
    let lines = e.call(FTOL, &args![lines]).u32().wrapping_add(1);
    let page = word(e, this, 0x60);
    let page_width = get_float(e, page, TRAIT_0XFBA);
    let page_width = e.call(FTOL, &args![page_width]).u32();
    e.with_stack(0x28, |e, text_data| {
        e.call(
            TEXT_DATA_CONSTRUCT,
            &args![text_data, page_width, 0u32, 0u32, lines, 10u32],
        );
        let text = word(e, this, 0xbc);
        let page = word(e, this, 0x60);
        let font_number = get_float(e, page, TRAIT_0XFB9);
        let font_index = e.call(FTOL, &args![font_number]).u32().wrapping_sub(1);
        let fonts = e.call(FONT_MANAGER, &args![]).u32();
        let font = e.call(FONT_MANAGER_FONT, &args![fonts, font_index]).u32();
        e.call(FONT_PREP_TEXT, &args![font, text, text_data]);
        let page = word(e, this, 0x60);
        let visible = get_value_q(e, page, TRAIT_VISIBLE);
        let page = word(e, this, 0x60);
        let value = get_value(e, page, TRAIT_STRING);
        let laid_out = first_word(e, text_data.addr());
        add_text(e, this, laid_out, value, visible, -1);
        let length = e.call(STRING_LENGTH, &args![this.wrapping_add(0xb4)]).u32();
        let used = e.mem.u32(text_data.addr().wrapping_add(0x18));
        let step = e.call(ADVANCE_0042F5A0, &args![used, length]).u32();
        let next = step.wrapping_add(word(e, this, 0xbc));
        set_word(e, this, 0xbc, next);
        let next = word(e, this, 0xbc);
        if e.mem.u8(next) == 0 {
            set_word(e, this, 0xbc, 0);
        }
        font_text_data_destructor(e, text_data);
    });
}

// Translated from 00759470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a note): the note activation function of image notes:
/// finishes the text, hides tile +0x2c, shows tile +0x30 (0xfa9 = 0xff), hides tile +0x60 and,
/// for a terminal whose byte +0xa5 has bit 3, tiles +0x44 and +0x64; then sets tile +0x30's
/// image (0xfcc) to the name of the note's object (`005e9160`).
pub fn fn_00759470(e: &mut Engine, this: Ptr, note: u32) {
    let this = this.addr();
    computers_menu_animating_text_manager_force_finish(e, text_manager(e, this));
    let tile = word(e, this, 0x2c);
    set_int(e, tile, TRAIT_VISIBLE, 0);
    let tile = word(e, this, 0x30);
    set_int(e, tile, TRAIT_0XFA9, 0xff);
    let tile = word(e, this, 0x30);
    set_int(e, tile, TRAIT_VISIBLE, 1);
    let tile = word(e, this, 0x60);
    set_int(e, tile, TRAIT_VISIBLE, 0);
    let terminal = word(e, this, 0xa8);
    if fn_00759540(e, Ptr::new(terminal)) {
        let tile = word(e, this, 0x44);
        set_int(e, tile, TRAIT_VISIBLE, 0);
        let tile = word(e, this, 0x64);
        set_int(e, tile, TRAIT_VISIBLE, 0);
    }
    let object = e.call(NOTE_NAME_OBJECT, &args![note]).u32();
    let name = e.call(NAME_OF, &args![object]).u32();
    let tile = word(e, this, 0x30);
    e.call(TILE_SET_STRING, &args![tile, TRAIT_IMAGE, name, 1u32]);
}

// Translated from 00759540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a terminal): whether bit 3 of the byte at +0xa5 is set.
pub fn fn_00759540(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0xa5)) & 8 != 0
}

// Translated from 00759560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::Redisplay` (Xbox PDB; `cdecl`, the list box callback: tile, item): for a
/// tile, disables it and re-adds its line: an item whose conditions hold shows its text
/// (`0xfd6` 2 and the visited bit cleared when it was visited, else 1); the "back" line (no
/// item) shows the setting string; an item whose conditions fail adds nothing.
// Translation incomplete: 00759560 - the C++ exception frame (FS:[0], unwind state for the line string) is not translated.
pub fn computers_menu_redisplay(e: &mut Engine, tile: u32, item: u32) {
    if tile == 0 {
        return;
    }
    e.with_stack(8, |e, line| {
        e.call(STRING_FROM_TEXT, &args![line, PROMPT_TEXT]);
        let mut state = 1u32;
        let mut show = true;
        let enabled = add_user_trait(e, ENABLED_TEXT);
        set_int(e, tile, enabled, 0);
        if item != 0
            && e.call(
                CONDITION_IS_TRUE,
                &args![item.wrapping_add(0x64), 0u32, 0u32],
            )
            .bool()
        {
            if fn_00758a60(e, Ptr::new(item)) {
                state = 2;
                fn_007583b0(e, Ptr::new(item), 0);
            }
            let text = first_word(e, item);
            e.call(STRING_ASSIGN, &args![line, text]);
        } else if item == 0 {
            let back = e.call(SETTING_STRING, &args![SETTING_BACK]).u32();
            e.call(STRING_ASSIGN, &args![line, back]);
        } else {
            show = false;
        }
        set_int(e, tile, TRAIT_0XFD6, state);
        if show {
            let visible = get_value(e, tile, TRAIT_VISIBLE);
            let value = get_value(e, tile, TRAIT_0X1004);
            let text = first_word(e, line.addr());
            let manager = computers_menu_animating_text_manager_get_instance(e);
            computers_menu_animating_text_manager_add_item(
                e,
                Ptr::new(manager),
                text,
                value,
                visible,
                -1,
            );
        }
        e.call(STRING_DESTROY_004037D0, &args![line]);
    });
}

// Translated from 007596a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, the list box callback: tile, item): enables the tile (`_enabled` 1).
pub fn fn_007596a0(e: &mut Engine, tile: u32, _unused_1: u32) {
    if tile != 0 {
        let enabled = add_user_trait(e, ENABLED_TEXT);
        set_int(e, tile, enabled, 1);
    }
}

// Translated from 007596d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::AnimatingTextManager::GetInstance` (Xbox PDB; no arguments): the
/// singleton at `011d9338`, made on first use (0xc bytes, `00759760`; 0 when the allocation
/// fails).
// Translation incomplete: 007596d0 - the C++ exception frame (FS:[0]) that frees the block if the constructor throws is not translated.
pub fn computers_menu_animating_text_manager_get_instance(e: &mut Engine) -> u32 {
    if e.global::<u32>(TEXT_MANAGER_SINGLETON) == 0 {
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        let manager = if block != 0 {
            fn_00759760(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set_global(TEXT_MANAGER_SINGLETON, manager);
    }
    e.global::<u32>(TEXT_MANAGER_SINGLETON)
}

// Translated from 00759760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AnimatingTextManager` constructor (unnamed in the engine map; `thiscall`): its list
/// (`0048f200`). Returns `this`.
pub fn fn_00759760(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TEXT_LIST_SET_UP, &args![this]);
    this
}

// Translated from 00759780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): deletes the text manager singleton (`007597c0` with 1) and clears
/// it.
pub fn fn_00759780(e: &mut Engine) {
    let manager = e.global::<u32>(TEXT_MANAGER_SINGLETON);
    if manager != 0 {
        fn_007597c0(e, Ptr::new(manager), 1);
    }
    e.set_global(TEXT_MANAGER_SINGLETON, 0u32);
}

// Translated from 007597c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AnimatingTextManager` deleting destructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): the destructor (`007597f0`), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_007597c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007597f0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007597f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AnimatingTextManager` destructor (the engine map's library match is wrong for this
/// body; `thiscall`): deletes its items (`00759850`), then destroys its list (`004a1a30`).
// Translation incomplete: 007597f0 - the C++ exception frame (FS:[0]) is not translated.
pub fn fn_007597f0(e: &mut Engine, this: Ptr) {
    fn_00759850(e, this);
    e.call(TEXT_LIST_DESTRUCT, &args![this]);
}

// Translated from 00759850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the text manager): removes the items one by one (`007b5390`) until
/// the list is empty, deleting each one (`007598a0` with 1).
pub fn fn_00759850(e: &mut Engine, this: Ptr) {
    while !e.call(TEXT_LIST_IS_EMPTY, &args![this]).bool() {
        let item = e.call(TEXT_LIST_REMOVE_HEAD, &args![this]).u32();
        if item != 0 {
            fn_007598a0(e, Ptr::new(item), 1);
        }
    }
}

// Translated from 007598a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AnimatingText` deleting destructor (unnamed in the engine map; `thiscall`, `RET 4`):
/// the destructor (`00726cf0`), then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_007598a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(TEXT_ITEM_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The menu singleton's word at `offset`.
fn menu_word(e: &Engine, offset: u32) -> u32 {
    let me = e.global::<u32>(MENU_SINGLETON);
    word(e, me, offset)
}

/// The first item of the text manager's list (`0073ac90`, then the word there).
fn head_item(e: &mut Engine, this: Ptr) -> u32 {
    let head = e.call(TEXT_LIST_HEAD, &args![this]).u32();
    e.mem.u32(head)
}

// Translated from 007598d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::AnimatingTextManager::Update` (Xbox PDB; `thiscall`). While the list is not
/// empty and the time has passed `011d93d4`, finished items (byte +0x29) are removed: during
/// the intro (mode 0) each one sets the next time 500 ms on and plays the enter sound for a
/// prompt line (text starting with `>`); later a removed item on the screen tile (+0x44)
/// shows the cursor (+0x64). The first unfinished item is then updated: a prompt line during
/// the intro plays `UIHackingCharSingle` once per new character, spread over its delay; any
/// other line plays the scroll sound. Its tile's offset from the screen's parent is summed
/// (traits 0xfa1/0xfa2 through `_ftol2`); for a list box tile the cursor (+0x40) moves to the
/// tile's `_text_width`/`_text_y` plus that offset, otherwise (unless the tile's 0xfba is set)
/// to its width (0xfb1) plus the offset and the y offset. With the list empty, or a tile whose
/// 0xfba is set, the cursor follows the result tile (+0x3c): its x plus its width when visible,
/// and its y.
// Translation incomplete: 007598d0 - the C++ exception frame (FS:[0], unwind state for the temporary sound handle) is not translated.
pub fn computers_menu_animating_text_manager_update(e: &mut Engine, this: Ptr) {
    let mut stretched = false;
    let mut reached_item = false;
    while !e.call(TEXT_LIST_IS_EMPTY, &args![this]).bool()
        && now(e) > e.global::<u32>(NEXT_ITEM_TIME_011D93D4)
    {
        let item = head_item(e, this);
        if fn_00759d80(e, Ptr::new(item)) == 0 {
            reached_item = true;
            break;
        }
        let removed = e.call(TEXT_LIST_REMOVE_HEAD, &args![this]).u32();
        if menu_word(e, 0xb0) == 0 {
            if removed != 0 {
                let text = e.call(ANIMATING_TEXT_TEXT, &args![removed]).u32();
                if e.mem.u8(text) == e.mem.u8(PROMPT_TEXT) {
                    let me = e.global::<u32>(MENU_SINGLETON);
                    e.call(SOUND_HANDLE_PLAY, &args![me.wrapping_add(0xdc), 0u32]);
                }
            }
            let next = now(e).wrapping_add(500);
            e.set_global(NEXT_ITEM_TIME_011D93D4, next);
        } else if removed != 0 {
            let tile = e.call(ANIMATING_TEXT_TILE, &args![removed]).u32();
            if tile == menu_word(e, 0x44) {
                let cursor = menu_word(e, 0x64);
                set_int(e, cursor, TRAIT_VISIBLE, 1);
            }
        }
        if removed != 0 {
            fn_007598a0(e, Ptr::new(removed), 1);
        }
    }
    if reached_item {
        let prompt = if menu_word(e, 0xb0) == 0 {
            let item = head_item(e, this);
            let text = e.call(ANIMATING_TEXT_TEXT, &args![item]).u32();
            e.mem.u8(text) == e.mem.u8(PROMPT_TEXT)
        } else {
            false
        };
        if prompt {
            let item = head_item(e, this);
            let shown = e.call(ANIMATING_TEXT_SHOWN, &args![item]).u32();
            let before = e.call(TEXT_LENGTH, &args![shown]).u32();
            let item = head_item(e, this);
            e.call(ANIMATING_TEXT_UPDATE, &args![item]);
            let item = head_item(e, this);
            let shown = e.call(ANIMATING_TEXT_SHOWN, &args![item]).u32();
            let after = e.call(TEXT_LENGTH, &args![shown]).u32();
            let mut i = 0u32;
            while i < after.wrapping_sub(before) {
                let item = head_item(e, this);
                let delay = e.call(ANIMATING_TEXT_DELAY, &args![item]).u32();
                let delay = delay.wrapping_mul(i) / after.wrapping_sub(before);
                e.with_stack(0xc, |e, handle| {
                    let audio = e.call(AUDIO_MANAGER, &args![]).u32();
                    let sound = e
                        .call(
                            AUDIO_SOUND_BY_NAME,
                            &args![audio, handle, SOUND_CHAR_SINGLE, 0x121u32],
                        )
                        .u32();
                    e.call(SOUND_HANDLE_PLAY_AFTER, &args![sound, delay, 0u32]);
                    e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                });
                i = i.wrapping_add(1);
            }
        } else {
            let item = head_item(e, this);
            e.call(ANIMATING_TEXT_UPDATE, &args![item]);
            let me = e.global::<u32>(MENU_SINGLETON);
            e.call(SOUND_HANDLE_PLAY, &args![me.wrapping_add(0xd0), 0u32]);
        }
        let mut x = 0i32;
        let mut y = 0i32;
        let item = head_item(e, this);
        let tile = e.call(ANIMATING_TEXT_TILE, &args![item]).u32();
        stretched = get_value(e, tile, TRAIT_0XFBA) != 0;
        let mut ancestor = tile;
        while ancestor != 0 {
            let cursor = menu_word(e, 0x40);
            let stop = e.call(WORD_AT_0X28, &args![cursor]).u32();
            if ancestor == stop {
                break;
            }
            let value = get_float(e, ancestor, TRAIT_X);
            x = e.call(FTOL, &args![value]).i32().wrapping_add(x);
            let value = get_float(e, ancestor, TRAIT_Y);
            y = e.call(FTOL, &args![value]).i32().wrapping_add(y);
            ancestor = e.call(WORD_AT_0X28, &args![ancestor]).u32();
        }
        e.call(TILE_UPDATE, &args![tile, 0u32]);
        let me = e.global::<u32>(MENU_SINGLETON);
        if e.call(LIST_BOX_HAS_TILE, &args![me.wrapping_add(0x68), tile])
            .bool()
        {
            e.call(TILE_UPDATE_CHILDREN, &args![tile, 0u32]);
            let id = add_user_trait(e, TEXT_WIDTH_TEXT);
            let width = get_float(e, tile, id);
            let value = x87_store32(x87_add(width, f64::from(x)));
            let cursor = menu_word(e, 0x40);
            e.call(TILE_SET_FLOAT, &args![cursor, TRAIT_X, value, 1u32]);
            let id = add_user_trait(e, TEXT_Y_TEXT);
            let height = get_float(e, tile, id);
            let value = x87_store32(x87_add(height, f64::from(y)));
            let cursor = menu_word(e, 0x40);
            e.call(TILE_SET_FLOAT, &args![cursor, TRAIT_Y, value, 1u32]);
        } else if !stretched {
            let width = get_float(e, tile, TRAIT_WIDTH);
            let value = x87_store32(x87_add(width, f64::from(x)));
            let cursor = menu_word(e, 0x40);
            e.call(TILE_SET_FLOAT, &args![cursor, TRAIT_X, value, 1u32]);
            let cursor = menu_word(e, 0x40);
            set_int(e, cursor, TRAIT_Y, y as u32);
        }
    }
    if e.call(TEXT_LIST_IS_EMPTY, &args![this]).bool() || stretched {
        let result = menu_word(e, 0x3c);
        let width = if is_visible(e, result) {
            let result = menu_word(e, 0x3c);
            x87_store32(get_float(e, result, TRAIT_WIDTH))
        } else {
            0.0f32.to_bits()
        };
        let result = menu_word(e, 0x3c);
        let x = get_float(e, result, TRAIT_X);
        let x = x87_store32(x87_add(x, x87_m32(width)));
        let cursor = menu_word(e, 0x40);
        e.call(TILE_SET_FLOAT, &args![cursor, TRAIT_X, x, 1u32]);
        let result = menu_word(e, 0x3c);
        let y = x87_store32(get_float(e, result, TRAIT_Y));
        let cursor = menu_word(e, 0x40);
        e.call(TILE_SET_FLOAT, &args![cursor, TRAIT_Y, y, 1u32]);
    }
}

// Translated from 00759da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::AnimatingTextManager::AddItem` (Xbox PDB; `thiscall`, `RET 0x10`: text,
/// two tile values, index): a new `AnimatingText` (0x2c bytes, `00759eb0`) set up with the
/// two values, appended (index -1, or past the end) or inserted before the item at `index`,
/// then `00726db0(0)` on it.
// Translation incomplete: 00759da0 - the C++ exception frame (FS:[0]) that frees the block if the constructor throws is not translated.
pub fn computers_menu_animating_text_manager_add_item(
    e: &mut Engine,
    this: Ptr,
    text: u32,
    value: u32,
    visible: u32,
    index: i32,
) {
    let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
    let item = if block != 0 {
        e.call(ANIMATING_TEXT_CONSTRUCT, &args![block, text]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(ANIMATING_TEXT_SET_UP, &args![item, value, visible]);
        if index == -1 {
            e.call(TEXT_LIST_APPEND, &args![this, slot]);
        } else {
            let mut node = first_word(e, this.addr());
            let mut i = 0i32;
            while node != 0 && i < index {
                node = e.call(TEXT_LIST_NEXT, &args![this, node]).u32();
                i = i.wrapping_add(1);
            }
            if node != 0 {
                e.call(TEXT_LIST_INSERT_BEFORE, &args![this, node, slot]);
            } else {
                e.call(TEXT_LIST_APPEND, &args![this, slot]);
            }
        }
        let item = e.mem.u32(slot.addr());
        e.call(ANIMATING_TEXT_SET_00726DB0, &args![item, 0u32]);
    });
}

// Translated from 00759f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ComputersMenu::AnimatingTextManager::ForceFinish` (Xbox PDB; `thiscall`): walks the list
/// with an iterator on the stack (`0057cbe0`) and finishes every item.
pub fn computers_menu_animating_text_manager_force_finish(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, iterator| {
        let first = first_word(e, this.addr());
        e.mem.set_u32(iterator.addr(), first);
        while e.mem.u32(iterator.addr()) != 0 {
            let slot = e.call(TEXT_LIST_STEP, &args![this, iterator]).u32();
            let item = e.mem.u32(slot);
            e.call(ANIMATING_TEXT_FORCE_FINISH, &args![item]);
        }
    });
}

// Translated from 00759f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<TERMINAL_MENU_ITEM *>` constructor (unnamed in the engine map; `thiscall`):
/// the base (`007240c0`), the list (+4), the vtable, the tiles, counters and floats cleared,
/// `007a1950(0, 1)` and the registration `00721130(this)`. Returns `this`.
// Translation incomplete: 00759f90 - the C++ exception frame (FS:[0]) is not translated.
pub fn fn_00759f90(e: &mut Engine, this: Ptr) -> Ptr {
    let list = Ptr::<ListBox>::new(this.addr());
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(SIMPLE_LIST_CONSTRUCTOR, &args![this.addr().wrapping_add(4)]);
    set_word(e, this.addr(), 0, LIST_BOX_VTABLE);
    e.set(list, ListBox::pScrollBar, Ptr::NULL);
    e.set(list, ListBox::pHighlightedItem, Ptr::NULL);
    e.set(list, ListBox::pContainerRect, Ptr::NULL);
    e.set(list, ListBox::pTemplateName, Ptr::NULL);
    e.set(list, ListBox::usFlags, 0);
    e.set(list, ListBox::usNextIndex, 0);
    e.set(list, ListBox::fSavedScrollbarValue, 0.0);
    e.set(list, ListBox::fSavedHighlightIndex, 0.0);
    e.set(list, ListBox::fTotalListHeight, 0.0);
    e.call(LIST_BOX_SET_UP_007A1950, &args![this, 0u32, 1u32]);
    e.call(LIST_BOX_REGISTER, &args![this]);
    this
}

// Translated from 0075a060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<TERMINAL_MENU_ITEM *>` destructor (unnamed in the engine map; `thiscall`): the
/// vtable, `RemoveAll`, the unregistration `007211c0(this)` and the list (+4).
// Translation incomplete: 0075a060 - the C++ exception frame (FS:[0]) is not translated.
pub fn fn_0075a060(e: &mut Engine, this: Ptr) {
    set_word(e, this.addr(), 0, LIST_BOX_VTABLE);
    list_box_terminal_menu_item_remove_all(e, Ptr::new(this.addr()));
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTROY_0046FFB0, &args![this.addr().wrapping_add(4)]);
}

// Translated from 0075a110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::AddItem` (unnamed in the engine map; `thiscall`,
/// `RET 0x10`: object, text, node to insert after, template): renders the template (or
/// `pTemplateName`) into the container, gives the tile an id (-1) when it has none and the
/// text when given, wraps `object` in a new `ListBoxItem` (0xc bytes, given the address of the
/// `object` argument) and inserts it after `after` (synchronizing when flag 0 is set) or
/// appends it (positioning the tile and recomputing the height when flag 0 is set, and
/// numbering it with the next index). With exactly one item and a positive
/// `_number_of_visible_items`, the container's height becomes the tile's height times that
/// number. Returns the tile, or 0 without a container or template.
// Translation incomplete: 0075a110 - the C++ exception frame (FS:[0]) that frees the ListBoxItem if its constructor throws is not translated.
pub fn fn_0075a110(
    e: &mut Engine,
    this: Ptr<ListBox>,
    object: u32,
    text: u32,
    after: u32,
    template: u32,
) -> u32 {
    if e.get(this, ListBox::pContainerRect).is_null()
        || (e.get(this, ListBox::pTemplateName).is_null() && template == 0)
    {
        return 0;
    }
    let template = if template != 0 {
        template
    } else {
        e.get(this, ListBox::pTemplateName).addr()
    };
    let container = e.get(this, ListBox::pContainerRect).addr();
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let tile = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu, container, template, 0u32],
        )
        .u32();
    if get_value(e, tile, TRAIT_ID) == 0 {
        set_int(e, tile, TRAIT_ID, 0xffff_ffff);
    }
    if text != 0 {
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
    }
    // The constructor gets the address of the `object` stack argument.
    e.with_stack(4, |e, object_slot| {
        e.mem.set_u32(object_slot.addr(), object);
        let list = this.addr().wrapping_add(4);
        if after != 0 {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let item = if block != 0 {
                e.call(LIST_BOX_ITEM_CONSTRUCT, &args![block, tile, object_slot])
                    .u32()
            } else {
                0
            };
            e.call(LIST_INSERT_AFTER, &args![list, item, after]);
            if e.call(LIST_BOX_FLAG, &args![this, 0u32]).bool() {
                list_box_terminal_menu_item_synchronize(e, this);
            }
        } else {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let item = if block != 0 {
                e.call(LIST_BOX_ITEM_CONSTRUCT, &args![block, tile, object_slot])
                    .u32()
            } else {
                0
            };
            e.with_stack(4, |e, item_slot| {
                e.mem.set_u32(item_slot.addr(), item);
                e.call(LIST_APPEND, &args![list, item_slot]);
            });
            if e.call(LIST_BOX_FLAG, &args![this, 0u32]).bool() {
                e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
                list_box_terminal_menu_item_set_total_list_height(e, this);
            }
            let index = u32::from(e.get(this, ListBox::usNextIndex));
            let next = e.get(this, ListBox::usNextIndex).wrapping_add(1);
            e.set(this, ListBox::usNextIndex, next);
            set_int(e, tile, TRAIT_LIST_INDEX, index);
        }
    });
    if e.call(LIST_BOX_COUNT, &args![this]).u32() == 1 {
        let id = add_user_trait(e, TEXT_NUMBER_OF_VISIBLE_ITEMS);
        let container = e.get(this, ListBox::pContainerRect).addr();
        let visible = get_float(e, container, id);
        // `TEST AH,0x41; JNZ`: on only when strictly above 0.0.
        if visible > e.global::<f64>(ZERO) {
            let container = e.get(this, ListBox::pContainerRect).addr();
            let height = get_value_q(e, container, TRAIT_HEIGHT);
            e.call(VALUE_CLEAR_ACTIONS, &args![height]);
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![height, ACTION_COPY, tile, TRAIT_HEIGHT],
            );
            let id = add_user_trait(e, TEXT_NUMBER_OF_VISIBLE_ITEMS);
            let container = e.get(this, ListBox::pContainerRect).addr();
            let visible = x87_store32(get_float(e, container, id));
            e.call(VALUE_ADD_ACTION, &args![height, ACTION_ADD, visible]);
            e.call(VALUE_CALCULATE, &args![height, 0u32]);
        }
    }
    tile
}

/// Deletes one `ListBoxItem` as `RemoveAll` does: its tile through slot 0 (unless
/// `011d8909` is set), its object's `004459e0(1)` when flag 1 is set, then the item.
fn release_list_item(e: &mut Engine, this: Ptr<ListBox>, item_of: impl Fn(&mut Engine) -> u32) {
    if e.global::<u8>(KEEP_ITEM_TILES_011D8909) == 0 {
        let item = item_of(e);
        let tile = e.mem.u32(item);
        if tile != 0 {
            e.vcall(tile, 0, &args![1u32]);
        }
    }
    if e.call(LIST_BOX_FLAG, &args![this, 1u32]).bool() {
        let item = item_of(e);
        let object = e.mem.u32(item.wrapping_add(4));
        if object != 0 {
            e.call(OBJECT_RELEASE, &args![object, 1u32]);
        }
    }
    let item = item_of(e);
    e.call(OPERATOR_DELETE, &args![item]);
}

// Translated from 0075a3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::RemoveAll` (engine map; `thiscall`): deletes every item
/// after the embedded first node (and that node), then the first item; clears the list, the
/// next index and the total height, the container's `_num_filtered` (0.0), recomputes the
/// height, clears the highlighted tile and updates the highlight box.
pub fn list_box_terminal_menu_item_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    let base = this.addr();
    loop {
        let node = word(e, base, 8);
        if node == 0 {
            break;
        }
        let slot = node_item(e, node);
        if e.mem.u32(slot) == 0 {
            break;
        }
        let node = word(e, base, 8);
        let next = word_at_four(e, node);
        release_list_item(e, this, |e| {
            let node = word(e, base, 8);
            let slot = node_item(e, node);
            e.mem.u32(slot)
        });
        let node = word(e, base, 8);
        e.call(NODE_SET_NEXT, &args![node, 0u32]);
        let node = word(e, base, 8);
        if node != 0 {
            e.call(NODE_DELETE, &args![node, 1u32]);
        }
        set_word(e, base, 8, next);
    }
    if word(e, base, 4) != 0 {
        release_list_item(e, this, |e| word(e, base, 4));
    }
    set_word(e, base, 4, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    if !e.get(this, ListBox::pContainerRect).is_null() {
        let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_NUM_FILTERED]).u32();
        let container = e.get(this, ListBox::pContainerRect).addr();
        e.call(
            TILE_SET_FLOAT,
            &args![container, id, 0.0f32.to_bits(), 1u32],
        );
    }
    list_box_terminal_menu_item_set_total_list_height(e, this);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    list_box_terminal_menu_item_update_highlight_box_pos(e, this);
}

// Translated from 0075a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::Sort` (engine map; `thiscall`, `RET 4`: compare function):
/// sorts the list (`0083fd60`) and synchronizes.
pub fn list_box_terminal_menu_item_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    e.call(LIST_SORT, &args![this.addr().wrapping_add(4), compare]);
    list_box_terminal_menu_item_synchronize(e, this);
}

/// Whether a tile is enabled (`_enabled` through `Tile::IsTrue`).
fn tile_enabled(e: &mut Engine, tile: u32) -> bool {
    let id = add_user_trait(e, ENABLED_TEXT);
    e.call(TILE_IS_TRUE, &args![tile, id]).bool()
}

// Translated from 0075a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::Highlight` (engine map; `thiscall`, `RET 4`: tile): with
/// the list enabled, highlights the tile (or nothing for 0) when it is one of the list's and
/// enabled, updating the highlight box; true then.
pub fn list_box_terminal_menu_item_highlight(
    e: &mut Engine,
    this: Ptr<ListBox>,
    tile: u32,
) -> bool {
    if !list_box_terminal_menu_item_is_enabled(e, this) {
        return false;
    }
    if tile != 0 && !(e.call(LIST_BOX_HAS_TILE, &args![this, tile]).bool() && tile_enabled(e, tile))
    {
        return false;
    }
    e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    list_box_terminal_menu_item_update_highlight_box_pos(e, this);
    true
}

/// The step of a page scroll in `GetHighlightTarget`: 0 without a container, otherwise the
/// container's 0xfb0 (negated with `FCHS` for a page up) divided by its `_scroll_delta`
/// (`FDIVR`), stored as a float, given with 1.0 to `004bd510` and truncated by `_ftol2`.
fn page_step(e: &mut Engine, this: Ptr<ListBox>, negate: bool) -> i32 {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return 0;
    }
    let container = e.get(this, ListBox::pContainerRect).addr();
    let extent = get_float(e, container, TRAIT_HEIGHT);
    let numerator = if negate { -extent } else { extent };
    let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_SCROLL_DELTA]).u32();
    let container = e.get(this, ListBox::pContainerRect).addr();
    let delta = get_float(e, container, id);
    let step = x87_store32(x87_div(numerator, delta));
    let rounded = e.call(SCROLL_ROUND, &args![step, 1.0f32.to_bits()]).f64();
    e.call(FTOL, &args![rounded]).i32()
}

// Translated from 0075a680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::GetHighlightTarget` (engine map; `thiscall`, `RET 4`:
/// direction 1 up, 2 down, 0xf page up, 0x10 page down): the enabled tile a move reaches, or 0
/// when there is none or it is the one already highlighted. From the highlighted tile's index
/// (0xfac, clamped) or an end of the list, the item tile with that index is looked for; a
/// disabled one moves on in the direction, and past an end the search turns back from the
/// other side through the list's slot +0x14 (tile at an index).
pub fn list_box_terminal_menu_item_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    let step = match direction {
        1 => -1,
        2 => 1,
        0xf => page_step(e, this, true),
        0x10 => page_step(e, this, false),
        _ => 0,
    };
    let mut target = 0u32;
    if list_box_terminal_menu_item_is_enabled(e, this)
        && step != 0
        && e.get(this, ListBox::usNextIndex) != 0
    {
        let last = i32::from(e.get(this, ListBox::usNextIndex)) - 1;
        let mut index = if !e.get(this, ListBox::pHighlightedItem).is_null() {
            let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
            let value = get_float(e, highlighted, TRAIT_LIST_INDEX);
            let moved = e.call(FTOL, &args![value]).i32().wrapping_add(step);
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
        'search: loop {
            let mut node = if this.is_null() {
                0
            } else {
                this.addr().wrapping_add(4)
            };
            while target == 0 && node != 0 {
                let slot = node_item(e, node);
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = node_item(e, node);
                let tile = e.mem.u32(e.mem.u32(slot));
                let value = get_float(e, tile, TRAIT_LIST_INDEX);
                // `FILD; FUCOMPP; TEST AH,0x44; JP`: taken only when equal.
                if f64::from(index) == value {
                    let slot = node_item(e, node);
                    target = e.mem.u32(e.mem.u32(slot));
                }
                node = word_at_four(e, node);
            }
            if target == 0 || tile_enabled(e, target) {
                break;
            }
            target = 0;
            if step > 0 {
                index = index.wrapping_add(1);
                if index > last {
                    index = last - 1;
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'search;
                        }
                        if index < 0 {
                            break 'search;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index = index.wrapping_sub(1);
                    }
                }
            } else if step < 0 {
                index = index.wrapping_sub(1);
                if index < 0 {
                    index = 1;
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'search;
                        }
                        if index > last {
                            break 'search;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index = index.wrapping_add(1);
                    }
                }
            }
        }
    }
    if target != e.get(this, ListBox::pHighlightedItem).addr() {
        target
    } else {
        0
    }
}

// Translated from 0075aa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::`scalar deleting destructor'` (engine map; `thiscall`,
/// `RET 4`): the destructor (`0075a060`), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn list_box_terminal_menu_item_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0075a060(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0075aa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::Synchronize` (engine map; `thiscall`): clears the next
/// index and the total height, then numbers (0xfac) and positions each item tile whose
/// filtered byte (+8) is clear; then recomputes the height and the highlight box.
pub fn list_box_terminal_menu_item_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = if this.is_null() {
        0
    } else {
        this.addr().wrapping_add(4)
    };
    while node != 0 {
        let slot = node_item(e, node);
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = node_item(e, node);
        if e.mem.u8(e.mem.u32(slot).wrapping_add(8)) == 0 {
            let index = u32::from(e.get(this, ListBox::usNextIndex));
            let next = e.get(this, ListBox::usNextIndex).wrapping_add(1);
            e.set(this, ListBox::usNextIndex, next);
            let slot = node_item(e, node);
            let tile = e.mem.u32(e.mem.u32(slot));
            set_int(e, tile, TRAIT_LIST_INDEX, index);
            let slot = node_item(e, node);
            let tile = e.mem.u32(e.mem.u32(slot));
            e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
        }
        node = word_at_four(e, node);
    }
    list_box_terminal_menu_item_set_total_list_height(e, this);
    list_box_terminal_menu_item_update_highlight_box_pos(e, this);
}

// Translated from 0075ab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::IsEnabled` (engine map; `thiscall`): whether there is a
/// container and its `_enabled` (a function-local static made on first use) is true.
// Translation incomplete: 0075ab20 - the C++ exception frame (FS:[0]) around the static's initialisation is not translated.
pub fn list_box_terminal_menu_item_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    if e.global::<u32>(ENABLED_GUARD_011D93DC) & 1 == 0 {
        let guard = e.global::<u32>(ENABLED_GUARD_011D93DC) | 1;
        e.set_global(ENABLED_GUARD_011D93DC, guard);
        let id = add_user_trait(e, ENABLED_TEXT);
        e.set_global(ENABLED_TRAIT_011D93D8, id);
    }
    if e.get(this, ListBox::pContainerRect).is_null() {
        return false;
    }
    let id = e.global::<u32>(ENABLED_TRAIT_011D93D8);
    let container = e.get(this, ListBox::pContainerRect).addr();
    e.call(TILE_IS_TRUE, &args![container, id]).bool()
}

/// Marks the highlight box found under the container and its four edges (`0071aa60`, then
/// `Tile::AddNeedsUpdate(8)`).
fn mark_highlight_box(e: &mut Engine, this: Ptr<ListBox>, mark: u32) {
    let container = e.get(this, ListBox::pContainerRect).addr();
    let boxed = e
        .call(TILE_FIND, &args![container, TEXT_LB_HIGHLIGHT_BOX])
        .u32();
    if boxed == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![boxed, TEXT_TOP])
        .u32();
    if top != 0 {
        e.call(TILE_MARK_0071AA60, &args![top, mark]);
        e.call(TILE_ADD_NEEDS_UPDATE, &args![top, 8u32]);
        for name in [TEXT_BOTTOM, TEXT_LEFT, TEXT_RIGHT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![boxed, name]).u32();
            e.call(TILE_MARK_0071AA60, &args![edge, mark]);
            e.call(TILE_ADD_NEEDS_UPDATE, &args![edge, 8u32]);
        }
    }
    e.call(TILE_MARK_0071AA60, &args![boxed, mark]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![boxed, 8u32]);
}

// Translated from 0075abd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::UpdateHighlightBoxPos` (engine map; `thiscall`): with a
/// container, either makes its `_highlight_y` and `_selected_height` copy the highlighted
/// tile's y (0xfa2) and height (0xfb0) and marks the highlight box with the highlighted tile's
/// flag (`0071aa80`, as 1 or 0), or, with nothing highlighted, sets them to -1 and 0. The two
/// trait ids are function-local statics made on first use.
// Translation incomplete: 0075abd0 - the C++ exception frame (FS:[0]) around the statics' initialisation is not translated.
pub fn list_box_terminal_menu_item_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    if e.global::<u32>(HIGHLIGHT_GUARD_011D93E8) & 1 == 0 {
        let guard = e.global::<u32>(HIGHLIGHT_GUARD_011D93E8) | 1;
        e.set_global(HIGHLIGHT_GUARD_011D93E8, guard);
        let id = add_user_trait(e, TEXT_HIGHLIGHT_Y);
        e.set_global(HIGHLIGHT_Y_TRAIT_011D93E4, id);
    }
    if e.global::<u32>(HIGHLIGHT_GUARD_011D93E8) & 2 == 0 {
        let guard = e.global::<u32>(HIGHLIGHT_GUARD_011D93E8) | 2;
        e.set_global(HIGHLIGHT_GUARD_011D93E8, guard);
        let id = add_user_trait(e, TEXT_SELECTED_HEIGHT);
        e.set_global(SELECTED_HEIGHT_TRAIT_011D93E0, id);
    }
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let container = e.get(this, ListBox::pContainerRect).addr();
        let id = e.global::<u32>(HIGHLIGHT_Y_TRAIT_011D93E4);
        set_int(e, container, id, 0xffff_ffff);
        let container = e.get(this, ListBox::pContainerRect).addr();
        let id = e.global::<u32>(SELECTED_HEIGHT_TRAIT_011D93E0);
        set_int(e, container, id, 0);
        return;
    }
    let container = e.get(this, ListBox::pContainerRect).addr();
    let id = e.global::<u32>(HIGHLIGHT_Y_TRAIT_011D93E4);
    let y = get_value(e, container, id);
    let container = e.get(this, ListBox::pContainerRect).addr();
    let id = e.global::<u32>(SELECTED_HEIGHT_TRAIT_011D93E0);
    let height = get_value(e, container, id);
    let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![y, ACTION_COPY, highlighted, TRAIT_Y],
    );
    e.call(VALUE_CALCULATE, &args![y, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![height, ACTION_COPY, highlighted, TRAIT_HEIGHT],
    );
    e.call(VALUE_CALCULATE, &args![height, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
    if e.call(TILE_FLAG_0071AA80, &args![highlighted]).u8() != 0 {
        mark_highlight_box(e, this, 1);
    } else {
        mark_highlight_box(e, this, 0);
    }
}

// Translated from 0075aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::SetTotalListHeight` (engine map; `thiscall`): with a
/// container whose height (0xfb0) is positive and below a positive total height, the scroll
/// steps are `00647b70(trunc(004e4430(unfiltered * (total - height) / total)) + 1, 2)` (else
/// 1); with more than one step the container's `_scroll_delta` becomes `(total - height) /
/// (steps - 1)`. The scroll bar's `_number_of_items` gets the step count. (The 2 is the word
/// pushed before `NumUnfiltered`, which leaves it on the stack; `00647b70`'s `ADD ESP,8` takes
/// it as its second argument.)
pub fn list_box_terminal_menu_item_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    let container = e.get(this, ListBox::pContainerRect).addr();
    let height = x87_store32(get_float(e, container, TRAIT_HEIGHT));
    let mut steps = 1i32;
    let zero = e.global::<f64>(ZERO);
    let total = e.get(this, ListBox::fTotalListHeight).to_bits();
    // `TEST AH,0x41; JNZ`: on only when strictly above 0.0.
    if x87_m32(total) > zero && x87_m32(height) > zero {
        let total = e.get(this, ListBox::fTotalListHeight).to_bits();
        // `FCOMPP; TEST AH,5; JP`: on only when `height < total`.
        if x87_m32(height) < x87_m32(total) {
            let unfiltered = list_box_terminal_menu_item_num_unfiltered(e, this);
            let count = f64::from(unfiltered as u32);
            let total = e.get(this, ListBox::fTotalListHeight).to_bits();
            let difference = x87_sub(x87_m32(total), x87_m32(height));
            let product = x87_mul(count, difference);
            let total = e.get(this, ListBox::fTotalListHeight).to_bits();
            let share = x87_store32(x87_div(product, x87_m32(total)));
            let rounded = e.call(SCROLL_STEPS, &args![share]).f64();
            let whole = e.call(FTOL, &args![rounded]).u32();
            steps = e
                .call(STEP_LIMIT, &args![whole.wrapping_add(1), 2u32])
                .i32();
        }
        if steps > 1 {
            let total = e.get(this, ListBox::fTotalListHeight).to_bits();
            let difference = x87_sub(x87_m32(total), x87_m32(height));
            let delta = x87_store32(x87_div(difference, f64::from(steps.wrapping_sub(1))));
            let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_SCROLL_DELTA]).u32();
            let container = e.get(this, ListBox::pContainerRect).addr();
            e.call(TILE_SET_FLOAT, &args![container, id, delta, 1u32]);
        }
    }
    if !e.get(this, ListBox::pScrollBar).is_null() {
        let id = e
            .call(TILE_TEXT_TO_TRAIT, &args![TEXT_NUMBER_OF_ITEMS])
            .u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar).addr();
        set_int(e, scroll_bar, id, steps as u32);
    }
}

// Translated from 0075b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TERMINAL_MENU_ITEM *>::NumUnfiltered` (engine map; `thiscall`, plain `RET`): the
/// item count minus the container's `_num_filtered` (`FISTP qword` with chop, the low word).
/// The trait id is a function-local static made on first use.
// Translation incomplete: 0075b020 - the C++ exception frame (FS:[0]) around the static's initialisation is not translated.
pub fn list_box_terminal_menu_item_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    if e.global::<u32>(NUM_FILTERED_GUARD_011D93F0) & 1 == 0 {
        let guard = e.global::<u32>(NUM_FILTERED_GUARD_011D93F0) | 1;
        e.set_global(NUM_FILTERED_GUARD_011D93F0, guard);
        let id = add_user_trait(e, TEXT_NUM_FILTERED);
        e.set_global(NUM_FILTERED_TRAIT_011D93EC, id);
    }
    let count = e.call(LIST_BOX_COUNT, &args![this]).u32();
    let id = e.global::<u32>(NUM_FILTERED_TRAIT_011D93EC);
    let container = e.get(this, ListBox::pContainerRect).addr();
    let filtered = get_float(e, container, id);
    let filtered = x87_fistp64_chop(X87Ext::load64(filtered)) as u32;
    count.wrapping_sub(filtered) as i32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0075_93e0, font_text_data_destructor(Ptr)),
        entry!(0x0075_9450, fn_00759450(Ptr) -> f32),
        entry!(0x0075_9d80, fn_00759d80(Ptr) -> u8),
        entry!(0x0075_77f0, computers_menu_computers_menu(Ptr) -> Ptr),
        entry!(0x0075_7a10, computers_menu_get_class(Ptr) -> u32),
        entry!(
            0x0075_7a20,
            computers_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0075_7a50, fn_00757a50(Ptr)),
        entry!(0x0075_7b70, computers_menu_create(u32) -> u32),
        entry!(0x0075_7e20, fn_00757e20() -> u32),
        entry!(0x0075_7e30, fn_00757e30(Ptr, Ptr) -> bool),
        entry!(0x0075_7e80, fn_00757e80(Ptr) -> bool),
        entry!(0x0075_7ea0, computers_menu_close()),
        entry!(0x0075_7f70, computers_menu_do_click(Ptr, i32, u32)),
        entry!(0x0075_8350, fn_00758350(Ptr) -> bool),
        entry!(0x0075_8390, fn_00758390(Ptr) -> bool),
        entry!(0x0075_83b0, fn_007583b0(Ptr, u8)),
        entry!(
            0x0075_83f0,
            computers_menu_do_gamepad_up_event(Ptr, i32, u32) -> bool
        ),
        entry!(0x0075_8470, computers_menu_do_idle(Ptr)),
        entry!(0x0075_85a0, fn_007585a0(Ptr)),
        entry!(0x0075_86e0, fn_007586e0(Ptr, u8)),
        entry!(0x0075_8a60, fn_00758a60(Ptr) -> bool),
        entry!(0x0075_8a80, computers_menu_display_previous_terminal(Ptr)),
        entry!(0x0075_8ad0, computers_menu_display_intro(Ptr)),
        entry!(0x0075_8d30, computers_menu_finish_intro(Ptr)),
        entry!(0x0075_8fe0, fn_00758fe0(Ptr) -> u32),
        entry!(0x0075_90c0, fn_007590c0(Ptr, u32)),
        entry!(0x0075_9470, fn_00759470(Ptr, u32)),
        entry!(0x0075_9540, fn_00759540(Ptr) -> bool),
        entry!(0x0075_9560, computers_menu_redisplay(u32, u32)),
        entry!(0x0075_96a0, fn_007596a0(u32, u32)),
        entry!(
            0x0075_96d0,
            computers_menu_animating_text_manager_get_instance() -> u32
        ),
        entry!(0x0075_9760, fn_00759760(Ptr) -> Ptr),
        entry!(0x0075_9780, fn_00759780()),
        entry!(0x0075_97c0, fn_007597c0(Ptr, u32) -> Ptr),
        entry!(0x0075_97f0, fn_007597f0(Ptr)),
        entry!(0x0075_9850, fn_00759850(Ptr)),
        entry!(0x0075_98a0, fn_007598a0(Ptr, u32) -> Ptr),
        entry!(
            0x0075_98d0,
            computers_menu_animating_text_manager_update(Ptr)
        ),
        entry!(
            0x0075_9da0,
            computers_menu_animating_text_manager_add_item(Ptr, u32, u32, u32, i32)
        ),
        entry!(
            0x0075_9f50,
            computers_menu_animating_text_manager_force_finish(Ptr)
        ),
        entry!(0x0075_9f90, fn_00759f90(Ptr) -> Ptr),
        entry!(0x0075_a060, fn_0075a060(Ptr)),
        entry!(
            0x0075_a110,
            fn_0075a110(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x0075_a3b0,
            list_box_terminal_menu_item_remove_all(Ptr<ListBox>)
        ),
        entry!(
            0x0075_a5e0,
            list_box_terminal_menu_item_sort(Ptr<ListBox>, u32)
        ),
        entry!(
            0x0075_a610,
            list_box_terminal_menu_item_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x0075_a680,
            list_box_terminal_menu_item_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x0075_aa20,
            list_box_terminal_menu_item_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0075_aa50,
            list_box_terminal_menu_item_synchronize(Ptr<ListBox>)
        ),
        entry!(
            0x0075_ab20,
            list_box_terminal_menu_item_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x0075_abd0,
            list_box_terminal_menu_item_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x0075_aef0,
            list_box_terminal_menu_item_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x0075_b020,
            list_box_terminal_menu_item_num_unfiltered(Ptr<ListBox>) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_text_data_destructor_frees_the_list_then_the_string() {
        let mut e = Engine::new();
        for callee in [
            LIST_REMOVE_ALL_00470470,
            LIST_DESTROY_0046FFB0,
            STRING_DESTROY_004037D0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0075_93e0, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (LIST_REMOVE_ALL_00470470, vec![0x4020]),
                (LIST_DESTROY_0046FFB0, vec![0x4020]),
                (STRING_DESTROY_004037D0, vec![0x4000]),
            ]
        );
    }

    #[test]
    fn fn_00759450_returns_the_float_quieted() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x2c, 2.5f32.to_bits());
        assert_eq!(e.call(0x0075_9450, &args![this]).f32(), 2.5);
        e.mem.set_u32(this + 0x2c, 0xff80_0001);
        assert_eq!(
            e.call(0x0075_9450, &args![this]).f32().to_bits(),
            0xffc0_0001
        );
    }

    #[test]
    fn fn_00759d80_returns_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x0075_9d80, &args![this]).u8(), 0);
        e.mem.set_u8(this + 0x29, 2);
        assert_eq!(e.call(0x0075_9d80, &args![this]).u8(), 2);
    }
}

#[cfg(test)]
mod n053_tests {
    use super::*;

    /// Every callee outside this file, stubbed to return 0.
    const FOREIGN: &[u32] = &[
        LIST_REMOVE_ALL_00470470,
        LIST_DESTROY_0046FFB0,
        STRING_DESTROY_004037D0,
        MENU_CONSTRUCTOR,
        MENU_DESTRUCTOR,
        SIMPLE_LIST_CONSTRUCTOR,
        STRING_CONSTRUCT,
        STRING_FROM_TEXT,
        STRING_ASSIGN,
        STRING_SET,
        FIRST_WORD,
        STRING_LENGTH,
        STRING_FORMAT,
        EH_VECTOR_CONSTRUCTOR,
        EH_VECTOR_DESTRUCTOR,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_DESTRUCTOR,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_PLAY_AFTER,
        SOUND_HANDLE_STOP,
        SOUND_HANDLE_RELEASE,
        AUDIO_MANAGER,
        AUDIO_SOUND_BY_NAME,
        MEMSET,
        ALLOCATE,
        OPERATOR_DELETE,
        COLOR_MANAGER,
        COLOR_GET,
        COLOR_SET,
        COLOR_CONSTRUCT,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_MAXIMUM_DEPTH,
        TILE_READ_FILE,
        TILE_GET_MENU,
        INTERFACE_MANAGER,
        TILE_READER,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        IS_TOP_MENU,
        PRINT_MESSAGE,
        TILE_GET_FLOAT,
        TILE_SET_FLOAT,
        TILE_SET_INT,
        TILE_SET_STRING,
        TILE_GET_VALUE,
        TILE_GET_VALUE_Q,
        TILE_IS_VISIBLE,
        TILE_ADD_USER_TRAIT,
        TILE_IS_TRUE,
        TILE_TEXT_TO_TRAIT,
        TILE_UPDATE,
        TILE_UPDATE_CHILDREN,
        TILE_FIND,
        TILE_GET_CHILD_BY_NAME,
        TILE_ADD_NEEDS_UPDATE,
        WORD_AT_0X28,
        WORD_AT_FOUR,
        VALUE_TEXT,
        VALUE_CURRENT_00488D50,
        VALUE_SET_FLOAT,
        VALUE_CLEAR_ACTIONS,
        VALUE_ADD_ACTION,
        VALUE_ADD_ACTION_TILE,
        VALUE_CALCULATE,
        TUTORIAL_SHOW,
        TUTORIAL_READY_00718840,
        LIST_BOX_INITIALIZE,
        TERMINAL_IS_UNLOCKED,
        TERMINAL_HACK_VALUE,
        RUN_RESULT_SCRIPT,
        FORM_OF,
        PLAY_SHUTDOWN_SOUND,
        RENDERED_TERMINAL_FADE_OUT,
        INTERFACE_TERMINAL_00707AF0,
        TERMINAL_REFERENCE_0063D040,
        TILE_REFERENCE_0056C7F0,
        RT_DYNAMIC_CAST,
        CURRENT_TIME,
        SETTING_VALUE_ADDRESS,
        SETTING_STRING,
        SETTING_STRING_004C69F0,
        TEXT_LENGTH,
        LIST_BOX_FOR_EACH,
        LIST_BOX_HAS_TILE,
        LIST_BOX_ITEM_OF,
        LIST_BOX_FLAG,
        LIST_BOX_COUNT,
        LIST_BOX_BASE_CONSTRUCT,
        LIST_BOX_SET_UP_007A1950,
        LIST_BOX_REGISTER,
        LIST_BOX_UNREGISTER,
        LIST_BOX_INIT_TILE_POSITION,
        TILE_FLAG_0071AA80,
        TILE_MARK_0071AA60,
        SCROLL_STEPS,
        STEP_LIMIT,
        SCROLL_ROUND,
        LIST_BOX_ITEM_CONSTRUCT,
        LIST_INSERT_AFTER,
        LIST_APPEND,
        MENU_RENDER_TEMPLATE,
        NODE_ITEM,
        NODE_SET_NEXT,
        NODE_DELETE,
        OBJECT_RELEASE,
        LIST_SORT,
        LIST_POP_FRONT,
        LIST_APPEND_ADDRESS,
        CONDITION_IS_TRUE,
        TERMINAL_ITEMS,
        COMPARE_TEXT,
        PLAYER_HAS_NOTE,
        PLAYER_ADD_NOTE,
        NOTE_TYPE,
        NOTE_DESCRIPTION,
        NOTE_NAME_OBJECT,
        NAME_OF,
        SHOW_MESSAGE,
        FONT_MANAGER,
        FONT_MANAGER_FONT,
        FONT_PREP_TEXT,
        TEXT_DATA_CONSTRUCT,
        ADVANCE_0042F5A0,
        ANIMATING_TEXT_CONSTRUCT,
        ANIMATING_TEXT_SET_UP,
        ANIMATING_TEXT_SET_00726DB0,
        ANIMATING_TEXT_UPDATE,
        ANIMATING_TEXT_FORCE_FINISH,
        ANIMATING_TEXT_TILE,
        ANIMATING_TEXT_TEXT,
        ANIMATING_TEXT_SHOWN,
        ANIMATING_TEXT_DELAY,
        TEXT_LIST_IS_EMPTY,
        TEXT_LIST_HEAD,
        TEXT_LIST_REMOVE_HEAD,
        TEXT_LIST_COUNT,
        TEXT_LIST_APPEND,
        TEXT_LIST_NEXT,
        TEXT_LIST_INSERT_BEFORE,
        TEXT_LIST_STEP,
        TEXT_LIST_SET_UP,
        TEXT_LIST_DESTRUCT,
        TEXT_ITEM_DESTRUCT,
    ];

    /// Pages of the exe's data the code reads or writes.
    const PAGES: &[u32] = &[
        0x0101_1000,
        0x0101_2000,
        0x0101_6000,
        0x0107_0000,
        0x0107_2000,
        0x0118_9000,
        0x0119_f000,
        0x011d_8000,
        0x011d_9000,
        0x011d_e000,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &callee in FOREIGN {
            e.register(callee, |_, _| Ret::default());
        }
        // `_ftol2`: truncation of the `f64` argument.
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        for &page in PAGES {
            e.map(page, 0x1000);
        }
        e.set_global(ONE, 1.0f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(ZERO, 0.0f64);
        e.call_log = Some(vec![]);
        e
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

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A menu object with its 16 tile slots set to `0x6000 + 0x10 * slot`.
    fn new_menu(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x100);
        for slot in 0..16u32 {
            e.mem.set_u32(this + 0x28 + 4 * slot, 0x6000 + 0x10 * slot);
        }
        let manager = e.mem.alloc(0xc);
        e.mem.set_u32(this + 0xe8, manager);
        this
    }

    /// A terminal (+0xa8) whose description (slot +0x10 of its +0x90 object) is empty, and the
    /// menu list box's vtable (slot +0x1c), so `007586e0` can run.
    fn with_terminal(e: &mut Engine, this: u32) -> u32 {
        let terminal = e.mem.alloc(0xb0);
        e.mem.set_u8(terminal + 0xa6, 0xff);
        e.put_vtable(0x0f00_3000, &[0, 0, 0, 0, 0x0f00_3100]);
        e.register(0x0f00_3100, |_, _| Ret::default());
        e.mem.set_u32(terminal + 0x90, 0x0f00_3000);
        e.put_vtable(0x0f00_3200, &[0, 0, 0, 0, 0, 0, 0, 0x0f00_3300]);
        e.register(0x0f00_3300, |_, _| Ret::default());
        e.mem.set_u32(this + 0x68, 0x0f00_3200);
        e.mem.set_u32(this + 0xa8, terminal);
        terminal
    }

    /// `SETTING_VALUE_ADDRESS` returning the address of a word holding `value`.
    fn setting_value(e: &mut Engine, value: u32) {
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, value);
        e.register_double(SETTING_VALUE_ADDRESS, move |_, _| word.into_ret());
    }

    #[test]
    fn the_constructor_builds_the_members_and_the_sounds() {
        let mut e = engine();
        e.set_global(TEXT_MANAGER_SINGLETON, 0x7777u32);
        e.register(AUDIO_MANAGER, |_, _| 0x5000u32.into_ret());
        e.register(AUDIO_SOUND_BY_NAME, |_, a| a[1].into_ret());
        let this = e.mem.alloc(0x100);
        for offset in (0..0x100).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0075_77f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), COMPUTERS_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x68), LIST_BOX_VTABLE);
        assert_eq!(e.mem.u32(this + 0xe8), 0x7777);
        assert_eq!(e.mem.u32(this + 0xec), 0x0075_9030);
        assert_eq!(e.mem.u32(this + 0xf0), 0x0075_90c0);
        assert_eq!(e.mem.u32(this + 0xf4), 0x0075_9470);
        assert_eq!(e.mem.u32(this + 0xf8), 0);
        for offset in [0xa0, 0xa4, 0xa8, 0xac, 0xb0, 0xbc] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0xc0), 0);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x28, 0, 0x40]]);
        let sounds = calls(&e, AUDIO_SOUND_BY_NAME);
        let names: Vec<_> = sounds.iter().map(|w| (w[2], w[3])).collect();
        assert_eq!(
            names,
            vec![
                (SOUND_FAN_HUM, 0x31),
                (SOUND_CHAR_SCROLL, 0x21),
                (SOUND_CHAR_ENTER, 0x21)
            ]
        );
        let assigned: Vec<_> = calls(&e, SOUND_HANDLE_ASSIGN)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(assigned, vec![this + 0xc4, this + 0xd0, this + 0xdc]);
        assert_eq!(calls(&e, SOUND_HANDLE_DESTRUCTOR).len(), 3);
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCTOR),
            vec![vec![
                this + 0xc4,
                0xc,
                3,
                SOUND_HANDLE_CONSTRUCTOR,
                SOUND_HANDLE_DESTRUCTOR
            ]]
        );
    }

    #[test]
    fn get_class_is_0x421() {
        let mut e = engine();
        assert_eq!(e.call(0x0075_7a10, &args![0x1000u32]).u32(), 0x421);
        assert_eq!(e.call(0x0075_7e20, &args![]).u32(), 0x421);
    }

    #[test]
    fn the_deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x0075_7a20, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls(&e, MENU_DESTRUCTOR), vec![vec![this]]);
        e.call(0x0075_7a20, &args![this, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn the_destructor_stops_the_sounds_and_restores_the_colour() {
        let mut e = engine();
        e.set_global(MENU_SINGLETON, 0x1234u32);
        e.register(COLOR_MANAGER, |_, _| 0x4400u32.into_ret());
        let this = e.mem.alloc(0x100);
        e.call(0x0075_7a50, &args![this]);
        assert_eq!(e.global::<u32>(MENU_SINGLETON), 0);
        let stopped: Vec<_> = calls(&e, SOUND_HANDLE_STOP).iter().map(|w| w[0]).collect();
        assert_eq!(stopped, vec![this + 0xc4, this + 0xd0, this + 0xdc]);
        assert_eq!(calls(&e, SOUND_HANDLE_RELEASE).len(), 3);
        assert_eq!(calls(&e, COLOR_SET), vec![vec![0x4400, 3, SAVE_COLOR]]);
        assert_eq!(calls(&e, LIST_BOX_UNREGISTER), vec![vec![this + 0x68]]);
        assert_eq!(calls(&e, STRING_DESTROY_004037D0), vec![vec![this + 0xb4]]);
        assert_eq!(calls(&e, MENU_DESTRUCTOR), vec![vec![this]]);
    }

    /// A menu `Create` reads: its class (slot 0x34) is `class`; slot 0 is recorded.
    fn file_menu(e: &mut Engine, class: u32) -> u32 {
        let menu = new_menu(e);
        e.put_vtable(
            0x0f00_0000,
            &[0x0f00_0100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f00_0200],
        );
        e.register(0x0f00_0100, |_, _| Ret::default());
        e.register_double(0x0f00_0200, move |_, _| class.into_ret());
        e.mem.set_u32(menu, 0x0f00_0000);
        e.register_double(TILE_GET_MENU, move |_, _| menu.into_ret());
        e.register(TILE_READ_FILE, |_, _| 0x6600u32.into_ret());
        let terminal = e.mem.alloc(0xb0);
        e.register_double(FORM_OF, move |_, _| terminal.into_ret());
        menu
    }

    #[test]
    fn create_runs_the_intro_of_a_locked_terminal() {
        let mut e = engine();
        let menu = file_menu(&mut e, 0x421);
        let terminal = e.mem.alloc(0xb0);
        e.register_double(FORM_OF, move |_, _| terminal.into_ret());
        e.set_global(DEPTH_STACKING_A, 6006.0f64);
        e.set_global(DEPTH_STACKING_B, 102.0f64);
        e.register(TILE_GET_FLOAT, |_, _| float_ret(6006.0));
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| float_ret(3.5));
        e.register(TUTORIAL_SHOW, |_, _| 1u32.into_ret());
        setting_value(&mut e, 0);
        assert_eq!(e.call(0x0075_7b70, &args![0x8000u32]).u32(), 0x6600);
        assert_eq!(e.global::<u32>(MENU_SINGLETON), menu);
        assert_eq!(e.mem.u32(menu + 0xac), 0x8000);
        assert_eq!(e.mem.u32(menu + 0xa8), terminal);
        assert_eq!(e.mem.u8(menu + 0xc0), 1);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x6600, TRAIT_DEPTH, 3.5f32.to_bits(), 1]]
        );
        assert_eq!(calls(&e, MENU_SET_MENU_TILE), vec![vec![menu, 0x6600, 0]]);
        assert_eq!(
            calls(&e, LIST_BOX_INITIALIZE),
            vec![vec![menu + 0x68, 0x6010, FILE_TEMPLATE]]
        );
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY), vec![vec![menu + 0xc4, 0]]);
        // The intro ran (locked terminal): the hack value line.
        assert_eq!(calls(&e, TERMINAL_HACK_VALUE), vec![vec![terminal, 0x8000]]);
        assert_eq!(calls(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
        assert!(calls(&e, COLOR_CONSTRUCT).is_empty());
    }

    #[test]
    fn create_reports_a_missing_slot_and_a_missing_reference() {
        let mut e = engine();
        let menu = file_menu(&mut e, 0x421);
        e.mem.set_u32(menu + 0x28 + 4 * 5, 0);
        assert_eq!(e.call(0x0075_7b70, &args![0x8000u32]).u32(), 0);
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![MISSING_INDEX_MESSAGE, 5]]
        );
        e.mem.set_u32(menu + 0x28 + 4 * 5, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_7b70, &args![0u32]).u32(), 0);
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TERMINAL_MESSAGE]]
        );
    }

    #[test]
    fn create_deletes_a_menu_of_another_class() {
        let mut e = engine();
        let menu = file_menu(&mut e, 0x3f0);
        e.register(WORD_AT_FOUR, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0075_7b70, &args![0x8000u32]).u32(), 0);
        assert_eq!(calls(&e, 0x0f00_0100), vec![vec![menu, 1]]);
        assert!(calls(&e, MENU_SET_MENU_TILE).is_empty());
    }

    #[test]
    fn fn_00757e30_finds_the_first_empty_slot() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let index = e.mem.alloc(4);
        assert!(e.call(0x0075_7e30, &args![this, index]).bool());
        assert_eq!(e.mem.u32(index), 0x10);
        e.mem.set_u32(this + 0x28 + 4 * 7, 0);
        assert!(!e.call(0x0075_7e30, &args![this, index]).bool());
        assert_eq!(e.mem.u32(index), 7);
    }

    #[test]
    fn terminal_flag_bits() {
        let mut e = engine();
        let terminal = e.mem.alloc(0xb0);
        assert!(!e.call(0x0075_7e80, &args![terminal]).bool());
        assert!(!e.call(0x0075_9540, &args![terminal]).bool());
        e.mem.set_u8(terminal + 0xa5, 4);
        assert!(e.call(0x0075_7e80, &args![terminal]).bool());
        assert!(!e.call(0x0075_9540, &args![terminal]).bool());
        e.mem.set_u8(terminal + 0xa5, 8);
        assert!(!e.call(0x0075_7e80, &args![terminal]).bool());
        assert!(e.call(0x0075_9540, &args![terminal]).bool());
    }

    #[test]
    fn close_fades_the_rendered_terminal_of_this_reference() {
        let mut e = engine();
        e.call(0x0075_7ea0, &args![]);
        assert!(calls(&e, PLAY_SHUTDOWN_SOUND).is_empty());
        e.set_global(MENU_SINGLETON, 0x4000u32);
        e.set_global(FADE_ALPHA_01016264, 0.75f32.to_bits());
        e.register(WORD_AT_FOUR, |_, _| 0x6600u32.into_ret());
        e.register(INTERFACE_TERMINAL_00707AF0, |_, _| 0x9000u32.into_ret());
        e.register(TERMINAL_REFERENCE_0063D040, |_, _| 0x55u32.into_ret());
        e.register(TILE_REFERENCE_0056C7F0, |_, _| 0x55u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, _| 0x9100u32.into_ret());
        e.call(0x0075_7ea0, &args![]);
        assert_eq!(calls(&e, PLAY_SHUTDOWN_SOUND).len(), 1);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x6600, TRAIT_0XFC1, 0.75f32.to_bits(), 1]]
        );
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![0x9000, 0, TYPE_0119F43C, TYPE_0119F418, 0]]
        );
        assert_eq!(calls(&e, RENDERED_TERMINAL_FADE_OUT), vec![vec![0x9100]]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x6600, TRAIT_0X1772, 1]]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![0x4000]]);
    }

    #[test]
    fn clicking_the_screen_redisplays_the_items() {
        let mut e = engine();
        let this = new_menu(&mut e);
        with_terminal(&mut e, this);
        setting_value(&mut e, 0);
        e.register(TILE_IS_VISIBLE, |_, _| 1u32.into_ret());
        e.call(0x0075_7f70, &args![this, 2u32, 0u32]);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![0x6020, TRAIT_VISIBLE, 0],
                vec![0x6010, TRAIT_VISIBLE, 1]
            ]
        );
        assert_eq!(
            calls(&e, LIST_BOX_FOR_EACH),
            vec![vec![this + 0x68, 0x0075_9560, 0, 0x7fff_ffff]]
        );
        assert_eq!(e.mem.u8(this + 0xc1), 1);
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY), vec![vec![this + 0xdc, 0]]);
        // Id 13 before the intro ends finishes it.
        e.call(0x0075_7f70, &args![this, 0xdu32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xb0), 2);
    }

    #[test]
    fn clicking_an_item_with_a_note_adds_it_and_runs_its_activation() {
        let mut e = engine();
        let this = new_menu(&mut e);
        e.set_global(PLAYER, 0x7100u32);
        e.set_global(MESSAGE_TIME, 2.0f32.to_bits());
        let item = e.mem.alloc(0x80);
        e.mem.set_u32(item + 0x6c, 0x7300);
        e.mem.set_u8(item + 0x74, 3);
        e.register(LIST_BOX_HAS_TILE, |_, _| 1u32.into_ret());
        e.register_double(LIST_BOX_ITEM_OF, move |_, _| item.into_ret());
        e.register(NOTE_TYPE, |_, _| 1u32.into_ret());
        e.register(FIRST_WORD, |_, a| (a[0] + 0x10_0000).into_ret());
        e.mem.set_u32(this + 0xec + 4, 0x0f00_1000);
        e.register(0x0f00_1000, |_, _| Ret::default());
        setting_value(&mut e, 3);
        e.register(CURRENT_TIME, |_, _| 500u32.into_ret());
        e.call(0x0075_7f70, &args![this, 4u32, 0u32]);
        assert_eq!(calls(&e, RUN_RESULT_SCRIPT), vec![vec![item, 0]]);
        assert_eq!(e.mem.u8(item + 0x74), 7);
        assert_eq!(e.mem.u32(this + 0xa0), 3500);
        assert_eq!(calls(&e, PLAYER_ADD_NOTE), vec![vec![0x7100, 0x7300, 0]]);
        let message = calls(&e, SHOW_MESSAGE);
        assert_eq!(message.len(), 1);
        assert_eq!(message[0][1..], [0, NOTE_ICON, 0, 2.0f32.to_bits(), 0]);
        assert_eq!(calls(&e, 0x0f00_1000), vec![vec![this, 0x7300]]);
        assert_eq!(e.global::<u8>(NOTE_SHOWN_011D93C0), 1);
    }

    #[test]
    fn menu_item_flag_bits() {
        let mut e = engine();
        let item = e.mem.alloc(0x80);
        assert!(!e.call(0x0075_8350, &args![item]).bool());
        e.mem.set_u8(item + 0x74, 1);
        assert!(!e.call(0x0075_8350, &args![item]).bool());
        e.mem.set_u32(item + 0x6c, 5);
        assert!(e.call(0x0075_8350, &args![item]).bool());
        assert!(!e.call(0x0075_8390, &args![item]).bool());
        assert!(!e.call(0x0075_8a60, &args![item]).bool());
        e.mem.set_u8(item + 0x74, 6);
        assert!(e.call(0x0075_8390, &args![item]).bool());
        assert!(e.call(0x0075_8a60, &args![item]).bool());
    }

    #[test]
    fn fn_007583b0_sets_and_clears_bit_2() {
        let mut e = engine();
        let item = e.mem.alloc(0x80);
        e.mem.set_u8(item + 0x74, 0x81);
        e.call(0x0075_83b0, &args![item, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(item + 0x74), 0x85);
        e.call(0x0075_83b0, &args![item, 0x1234_5600u32]);
        assert_eq!(e.mem.u8(item + 0x74), 0x81);
    }

    #[test]
    fn the_gamepad_up_event_clicks_the_screen_or_goes_back() {
        let mut e = engine();
        let this = new_menu(&mut e);
        assert!(!e.call(0x0075_83f0, &args![this, 10u32, 0u32]).bool());
        e.register(IS_TOP_MENU, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0075_83f0, &args![this, 11u32, 0u32]).bool());
        e.put_vtable(0x0f00_2000, &[0, 0, 0, 0x0f00_2100]);
        e.register(0x0f00_2100, |_, _| Ret::default());
        e.mem.set_u32(this, 0x0f00_2000);
        e.mem.set_u32(this + 0xbc, 9);
        e.register(TILE_IS_VISIBLE, |_, _| 1u32.into_ret());
        assert!(e.call(0x0075_83f0, &args![this, 10u32, 0u32]).bool());
        assert_eq!(calls(&e, 0x0f00_2100), vec![vec![this, 2, 0]]);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
        e.register(TILE_IS_VISIBLE, |_, _| Ret::default());
        let empty = e.mem.alloc(4);
        e.register_double(NODE_ITEM, move |_, _| empty.into_ret());
        assert!(e.call(0x0075_83f0, &args![this, 10u32, 0u32]).bool());
        assert_eq!(calls(&e, LIST_POP_FRONT), vec![vec![this + 0x98]]);
    }

    #[test]
    fn do_idle_waits_then_starts_and_ends_the_intro() {
        let mut e = engine();
        let this = new_menu(&mut e);
        with_terminal(&mut e, this);
        setting_value(&mut e, 0);
        e.set_global(MENU_SINGLETON, this);
        e.mem.set_u8(this + 0xc0, 1);
        e.call(0x0075_8470, &args![this]);
        assert!(calls(&e, TEXT_LIST_IS_EMPTY).is_empty());
        e.mem.set_u8(this + 0xc0, 0);
        e.register(TEXT_LIST_IS_EMPTY, |_, _| 1u32.into_ret());
        e.register(CURRENT_TIME, |_, _| 1000u32.into_ret());
        e.set_global(BLINK_GUARD_011D93D0, 1u32);
        e.set_global(BLINK_TIME_011D93C8, 0xffff_ffffu32);
        e.mem.set_u32(this + 0xa4, 1);
        e.call(0x0075_8470, &args![this]);
        assert_eq!(e.global::<u32>(INTRO_END_TIME_011D93C4), 3000);
        assert_eq!(e.mem.u32(this + 0xb0), 1);
        e.register(CURRENT_TIME, |_, _| 3001u32.into_ret());
        e.mem.set_u32(this + 0xa0, 3000);
        e.call(0x0075_8470, &args![this]);
        assert_eq!(e.mem.u32(this + 0xb0), 2);
        assert_eq!(e.mem.u32(this + 0xa0), 0);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x6050, TRAIT_VISIBLE, 0]));
        // While fading out (+0x24 == 2) nothing runs.
        e.mem.set_u32(this + 0x24, 2);
        e.call_log = Some(vec![]);
        e.call(0x0075_8470, &args![this]);
        assert!(calls(&e, TEXT_LIST_IS_EMPTY).is_empty());
    }

    #[test]
    fn the_cursor_blinks_on_its_interval() {
        let mut e = engine();
        let this = new_menu(&mut e);
        e.register(TILE_GET_FLOAT, |_, _| float_ret(250.75));
        e.register(CURRENT_TIME, |_, _| 1000u32.into_ret());
        e.register(TILE_GET_VALUE_Q, |_, _| 0x7a00u32.into_ret());
        e.call(0x0075_85a0, &args![this]);
        assert_eq!(e.global::<u32>(BLINK_GUARD_011D93D0), 1);
        assert_eq!(
            e.global::<u32>(BLINK_INTERVAL_011D93CC),
            250.75f32.to_bits()
        );
        assert_eq!(e.mem.u32(this + 0xa4), 0x7a00);
        e.call(0x0075_85a0, &args![this]);
        assert_eq!(
            calls(&e, VALUE_SET_FLOAT),
            vec![vec![0x7a00, 1.0f32.to_bits(), 1]]
        );
        assert_eq!(e.global::<u32>(BLINK_TIME_011D93C8), 1250);
        assert_eq!(calls(&e, TILE_GET_FLOAT).len(), 1);
        e.register(VALUE_CURRENT_00488D50, |_, _| float_ret(1.0));
        e.set_global(BLINK_TIME_011D93C8, 0u32);
        e.call(0x0075_85a0, &args![this]);
        assert_eq!(calls(&e, VALUE_SET_FLOAT)[1], vec![0x7a00, 0, 1]);
    }

    #[test]
    fn fn_007586e0_lists_the_items_whose_conditions_hold() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let terminal = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xa8, terminal);
        e.mem.set_u32(this + 0xac, 0x8000);
        e.put_vtable(0x0f00_3000, &[0, 0, 0, 0, 0x0f00_3100]);
        e.register(0x0f00_3100, |_, _| Ret::default());
        e.mem.set_u32(terminal + 0x90, 0x0f00_3000);
        e.put_vtable(0x0f00_3200, &[0, 0, 0, 0, 0, 0, 0, 0x0f00_3300]);
        e.register(0x0f00_3300, |_, _| Ret::default());
        e.mem.set_u32(this + 0x68, 0x0f00_3200);
        let item = e.mem.alloc(0x80);
        e.mem.set_u8(item + 0x74, 4);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.register_double(TERMINAL_ITEMS, move |_, _| node.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CONDITION_IS_TRUE, |_, _| 1u32.into_ret());
        let manager = e.mem.u32(this + 0xe8);
        e.register_double(FIRST_WORD, move |_, a| {
            (if a[0] == manager { 0 } else { a[0] + 0x10_0000 }).into_ret()
        });
        setting_value(&mut e, 40);
        e.call(0x0075_86e0, &args![this, 1u32]);
        assert_eq!(calls(&e, 0x0f00_3300), vec![vec![this + 0x68]]);
        assert_eq!(
            calls(&e, LIST_APPEND_ADDRESS),
            vec![vec![this + 0x98, this + 0xa8]]
        );
        assert_eq!(e.global::<u32>(TEXT_SPEED_0119F278), 25);
        assert_eq!(
            calls(&e, 0x0f00_3100),
            vec![vec![terminal + 0x90, terminal, 0x4353_4544]]
        );
        assert_eq!(
            calls(&e, CONDITION_IS_TRUE),
            vec![vec![item + 0x64, 0x8000, 0]]
        );
        let lines = calls(&e, STRING_FORMAT);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0][1..],
            [TWO_STRINGS_FORMAT, PROMPT_TEXT, item + 0x10_0000]
        );
        // The visited bit became the line's 0xfd6 (2) and was cleared.
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0, TRAIT_0XFD6, 2]));
        assert_eq!(e.mem.u8(item + 0x74), 0);
        assert_eq!(e.mem.u8(this + 0xc1), 1);
        assert_eq!(calls(&e, TEXT_LIST_APPEND).len(), 1);
    }

    #[test]
    fn display_previous_terminal_closes_without_one() {
        let mut e = engine();
        let this = new_menu(&mut e);
        e.mem.set_u32(this + 0xa8, 0x1234);
        let empty = e.mem.alloc(4);
        e.register_double(NODE_ITEM, move |_, _| empty.into_ret());
        e.set_global(MENU_SINGLETON, 0u32);
        e.call(0x0075_8a80, &args![this]);
        assert_eq!(calls(&e, LIST_POP_FRONT), vec![vec![this + 0x98]]);
        assert_eq!(e.mem.u32(this + 0xa8), 0);
        assert!(calls(&e, STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn display_intro_writes_one_star_per_hack_point() {
        let mut e = engine();
        let this = new_menu(&mut e);
        e.mem.set_u32(this + 0xa8, 0x7400);
        e.mem.set_u32(this + 0xac, 0x8000);
        e.register(TERMINAL_HACK_VALUE, |_, _| 6u32.into_ret());
        setting_value(&mut e, 0);
        e.call(0x0075_8ad0, &args![this]);
        let fills = calls(&e, MEMSET);
        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0][1..], [0, 0x10]);
        assert_eq!(fills[1][1..], [0x2a, 6]);
        assert_eq!(fills[0][0], fills[1][0]);
        assert_eq!(calls(&e, TERMINAL_HACK_VALUE), vec![vec![0x7400, 0x8000]]);
        let settings: Vec<_> = calls(&e, SETTING_STRING_004C69F0)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(settings, vec![SETTING_INTRO_A, SETTING_INTRO_C]);
        assert_eq!(calls(&e, STRING_FORMAT).len(), 2);
        assert_eq!(calls(&e, TEXT_LIST_APPEND).len(), 4);
        assert_eq!(e.global::<u32>(TEXT_SPEED_0119F278), 1000);
    }

    #[test]
    fn finish_intro_centres_the_welcome_lines() {
        let mut e = engine();
        let this = new_menu(&mut e);
        with_terminal(&mut e, this);
        setting_value(&mut e, 0);
        e.register(TILE_GET_FLOAT, |_, _| float_ret(100.0));
        e.register(TEXT_LENGTH, |_, _| 2u32.into_ret());
        e.call(0x0075_8d30, &args![this]);
        assert_eq!(e.mem.u32(this + 0xb0), 2);
        let x: Vec<_> = calls(&e, TILE_SET_FLOAT)
            .iter()
            .map(|w| (w[0], w[1], w[2]))
            .collect();
        assert_eq!(
            x,
            vec![
                (0x6030, TRAIT_X, 33.0f32.to_bits()),
                (0x6040, TRAIT_X, 33.0f32.to_bits()),
                (0x6000, TRAIT_X, 33.0f32.to_bits())
            ]
        );
        assert_eq!(calls(&e, TEXT_LIST_APPEND).len(), 4);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x60d0, TRAIT_0XFAF, 0]));
    }

    #[test]
    fn fn_00758fe0_reads_the_setting_table() {
        let mut e = engine();
        let terminal = e.mem.alloc(0xb0);
        e.set_global(SETTING_TABLE_01189E94 + 4 * 3, 0x5550u32);
        e.register(SETTING_STRING, |_, a| (a[0] + 1).into_ret());
        e.mem.set_u8(terminal + 0xa6, 3);
        assert_eq!(e.call(0x0075_8fe0, &args![terminal]).u32(), 0x5551);
        e.mem.set_u8(terminal + 0xa6, 10);
        assert_eq!(e.call(0x0075_8fe0, &args![terminal]).u32(), 0);
    }

    #[test]
    fn a_text_note_shows_a_page_and_moves_on() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"abcd\0");
        e.put_vtable(0x0f00_4000, &[0, 0, 0, 0, 0x0f00_4100]);
        e.register_double(0x0f00_4100, move |_, _| text.into_ret());
        let description = e.mem.alloc(4);
        e.mem.set_u32(description, 0x0f00_4000);
        e.register_double(NOTE_DESCRIPTION, move |_, _| description.into_ret());
        e.register(STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(TILE_GET_FLOAT, |_, a| {
            float_ret(match a[1] {
                TRAIT_0XFBC => 20.0,
                TRAIT_HEIGHT => 30.0,
                TRAIT_0XFB9 => 2.0,
                _ => 300.0,
            })
        });
        let font = e.mem.alloc(0x30);
        e.mem.set_u32(font + 0x2c, 10.0f32.to_bits());
        e.register_double(FONT_MANAGER_FONT, move |_, _| font.into_ret());
        e.register(ADVANCE_0042F5A0, |_, _| 3u32.into_ret());
        setting_value(&mut e, 0);
        e.call(0x0075_90c0, &args![this, 0x7500u32]);
        assert_eq!(
            calls(&e, 0x0f00_4100),
            vec![vec![description, 0x7500, 0x4d41_4e54]]
        );
        let data = calls(&e, TEXT_DATA_CONSTRUCT);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0][1..], [300, 0, 0, 18, 10]);
        assert_eq!(calls(&e, FONT_PREP_TEXT)[0][..2], [font, text]);
        assert_eq!(calls(&e, FONT_MANAGER_FONT)[0][1], 1);
        assert_eq!(e.mem.u32(this + 0xbc), text + 3);
        assert_eq!(calls(&e, LIST_REMOVE_ALL_00470470).len(), 1);
        // The last page: the position reaches the terminating 0.
        e.register(ADVANCE_0042F5A0, |_, _| 1u32.into_ret());
        e.call(0x0075_90c0, &args![this, 0x7500u32]);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
    }

    #[test]
    fn an_image_note_shows_its_image() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let terminal = e.mem.alloc(0xb0);
        e.mem.set_u8(terminal + 0xa5, 8);
        e.mem.set_u32(this + 0xa8, terminal);
        e.register(NOTE_NAME_OBJECT, |_, _| 0x7600u32.into_ret());
        e.register(NAME_OF, |_, _| 0x7700u32.into_ret());
        e.call(0x0075_9470, &args![this, 0x7500u32]);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![0x6010, TRAIT_VISIBLE, 0],
                vec![0x6020, TRAIT_0XFA9, 0xff],
                vec![0x6020, TRAIT_VISIBLE, 1],
                vec![0x60e0, TRAIT_VISIBLE, 0],
                vec![0x6070, TRAIT_VISIBLE, 0],
                vec![0x60f0, TRAIT_VISIBLE, 0]
            ]
        );
        assert_eq!(calls(&e, NOTE_NAME_OBJECT), vec![vec![0x7500]]);
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x6020, TRAIT_IMAGE, 0x7700, 1]]
        );
    }

    #[test]
    fn redisplay_re_adds_the_lines_it_can() {
        let mut e = engine();
        e.set_global(TEXT_MANAGER_SINGLETON, 0x7800u32);
        e.call(0x0075_9560, &args![0u32, 0u32]);
        assert!(calls(&e, STRING_FROM_TEXT).is_empty());
        e.register(SETTING_STRING, |_, _| 0x7900u32.into_ret());
        e.call(0x0075_9560, &args![0x6100u32, 0u32]);
        assert_eq!(calls(&e, STRING_ASSIGN)[0][1], 0x7900);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x6100, TRAIT_0XFD6, 1]));
        assert_eq!(calls(&e, TEXT_LIST_APPEND).len(), 1);
        assert_eq!(calls(&e, TEXT_LIST_APPEND)[0][0], 0x7800);
        // An item whose conditions fail adds nothing.
        let item = e.mem.alloc(0x80);
        e.call(0x0075_9560, &args![0x6100u32, item]);
        assert_eq!(calls(&e, TEXT_LIST_APPEND).len(), 1);
        // A visited item whose conditions hold: 0xfd6 is 2 and the bit is cleared.
        e.register(CONDITION_IS_TRUE, |_, _| 1u32.into_ret());
        e.mem.set_u8(item + 0x74, 4);
        e.call(0x0075_9560, &args![0x6100u32, item]);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x6100, TRAIT_0XFD6, 2]));
        assert_eq!(e.mem.u8(item + 0x74), 0);
        assert_eq!(calls(&e, TEXT_LIST_APPEND).len(), 2);
    }

    #[test]
    fn fn_007596a0_enables_a_tile() {
        let mut e = engine();
        e.register(TILE_ADD_USER_TRAIT, |_, _| 0x2222u32.into_ret());
        e.call(0x0075_96a0, &args![0u32, 0u32]);
        assert!(calls(&e, TILE_SET_INT).is_empty());
        e.call(0x0075_96a0, &args![0x6100u32, 0u32]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x6100, 0x2222, 1]]);
    }

    #[test]
    fn the_text_manager_singleton_is_made_once_and_freed() {
        let mut e = engine();
        let block = e.mem.alloc(0xc);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        assert_eq!(e.call(0x0075_96d0, &args![]).u32(), block);
        assert_eq!(e.call(0x0075_96d0, &args![]).u32(), block);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0xc]]);
        assert_eq!(calls(&e, TEXT_LIST_SET_UP), vec![vec![block]]);
        e.register(TEXT_LIST_IS_EMPTY, |_, _| 1u32.into_ret());
        e.call(0x0075_9780, &args![]);
        assert_eq!(e.global::<u32>(TEXT_MANAGER_SINGLETON), 0);
        assert_eq!(calls(&e, TEXT_LIST_DESTRUCT), vec![vec![block]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![block]]);
    }

    #[test]
    fn the_text_manager_constructor_and_destructors() {
        let mut e = engine();
        assert_eq!(e.call(0x0075_9760, &args![0x5000u32]).u32(), 0x5000);
        e.register(TEXT_LIST_IS_EMPTY, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0075_97c0, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0075_97f0, &args![0x5000u32]);
        assert_eq!(calls(&e, TEXT_LIST_DESTRUCT).len(), 2);
        assert_eq!(e.call(0x0075_98a0, &args![0x5100u32, 1u32]).u32(), 0x5100);
        assert_eq!(calls(&e, TEXT_ITEM_DESTRUCT), vec![vec![0x5100]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x5100]]);
    }

    #[test]
    fn fn_00759850_deletes_every_item() {
        let mut e = engine();
        let mut empty = [false, false, true].into_iter();
        e.register_double(TEXT_LIST_IS_EMPTY, move |_, _| {
            u32::from(empty.next().unwrap()).into_ret()
        });
        let mut items = [0x5100u32, 0].into_iter();
        e.register_double(TEXT_LIST_REMOVE_HEAD, move |_, _| {
            items.next().unwrap().into_ret()
        });
        e.call(0x0075_9850, &args![0x5000u32]);
        assert_eq!(calls(&e, TEXT_LIST_REMOVE_HEAD).len(), 2);
        assert_eq!(calls(&e, TEXT_ITEM_DESTRUCT), vec![vec![0x5100]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x5100]]);
    }

    #[test]
    fn update_moves_the_cursor_after_the_current_line() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        e.mem.set_u32(menu + 0xb0, 2);
        e.set_global(MENU_SINGLETON, menu);
        let item = e.mem.alloc(0x2c);
        let head = e.mem.alloc(4);
        e.mem.set_u32(head, item);
        e.register_double(TEXT_LIST_HEAD, move |_, _| head.into_ret());
        e.register(CURRENT_TIME, |_, _| 10u32.into_ret());
        e.register(ANIMATING_TEXT_TILE, |_, _| 0x6800u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, a| {
            float_ret(match a[1] {
                TRAIT_X => 7.9,
                TRAIT_Y => -3.5,
                _ => 40.0,
            })
        });
        e.call(0x0075_98d0, &args![0x5000u32]);
        assert_eq!(calls(&e, ANIMATING_TEXT_UPDATE), vec![vec![item]]);
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY), vec![vec![menu + 0xd0, 0]]);
        assert_eq!(calls(&e, TILE_UPDATE), vec![vec![0x6800, 0]]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x6060, TRAIT_X, 47.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x6060, TRAIT_Y, (-3i32) as u32]]
        );
    }

    #[test]
    fn update_spreads_the_typing_sounds_of_a_prompt_line() {
        let mut e = engine();
        let menu = new_menu(&mut e);
        e.set_global(MENU_SINGLETON, menu);
        e.set_global(PROMPT_TEXT, u32::from(b'>'));
        let item = e.mem.alloc(0x2c);
        let head = e.mem.alloc(4);
        e.mem.set_u32(head, item);
        e.register_double(TEXT_LIST_HEAD, move |_, _| head.into_ret());
        e.register(CURRENT_TIME, |_, _| 10u32.into_ret());
        let text = e.mem.alloc(4);
        e.mem.set_u8(text, b'>');
        e.register_double(ANIMATING_TEXT_TEXT, move |_, _| text.into_ret());
        let mut lengths = [1u32, 3].into_iter();
        e.register_double(TEXT_LENGTH, move |_, _| lengths.next().unwrap().into_ret());
        e.register(ANIMATING_TEXT_DELAY, |_, _| 100u32.into_ret());
        e.register(AUDIO_SOUND_BY_NAME, |_, _| 0x5500u32.into_ret());
        e.call(0x0075_98d0, &args![0x5000u32]);
        let sounds: Vec<_> = calls(&e, AUDIO_SOUND_BY_NAME)
            .iter()
            .map(|w| (w[2], w[3]))
            .collect();
        assert_eq!(sounds, vec![(SOUND_CHAR_SINGLE, 0x121); 2]);
        assert_eq!(
            calls(&e, SOUND_HANDLE_PLAY_AFTER),
            vec![vec![0x5500, 0, 0], vec![0x5500, 50, 0]]
        );
        assert!(calls(&e, SOUND_HANDLE_PLAY).is_empty());
    }

    #[test]
    fn add_item_appends_or_inserts_before_the_index() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0x5400u32.into_ret());
        e.register(ANIMATING_TEXT_CONSTRUCT, |_, a| a[0].into_ret());
        e.call(0x0075_9da0, &args![0x5000u32, 0x7000u32, 1u32, 2u32, -1i32]);
        assert_eq!(
            calls(&e, ANIMATING_TEXT_CONSTRUCT),
            vec![vec![0x5400, 0x7000]]
        );
        assert_eq!(calls(&e, ANIMATING_TEXT_SET_UP), vec![vec![0x5400, 1, 2]]);
        assert_eq!(calls(&e, TEXT_LIST_APPEND).len(), 1);
        assert_eq!(
            calls(&e, ANIMATING_TEXT_SET_00726DB0),
            vec![vec![0x5400, 0]]
        );
        e.register(FIRST_WORD, |_, _| 0x5600u32.into_ret());
        e.register(TEXT_LIST_NEXT, |_, a| (a[1] + 0x10).into_ret());
        e.call(0x0075_9da0, &args![0x5000u32, 0x7000u32, 1u32, 2u32, 2i32]);
        let inserted = calls(&e, TEXT_LIST_INSERT_BEFORE);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][..2], [0x5000, 0x5620]);
    }

    #[test]
    fn force_finish_finishes_every_item() {
        let mut e = engine();
        e.register(FIRST_WORD, |_, _| 0x5600u32.into_ret());
        let item_slot = e.mem.alloc(4);
        e.mem.set_u32(item_slot, 0x5700);
        e.register_double(TEXT_LIST_STEP, move |e, a| {
            e.mem.set_u32(a[1], 0);
            item_slot.into_ret()
        });
        e.call(0x0075_9f50, &args![0x5000u32]);
        assert_eq!(calls(&e, ANIMATING_TEXT_FORCE_FINISH), vec![vec![0x5700]]);
    }

    #[test]
    fn the_list_box_constructor_and_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0075_9f90, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LIST_BOX_VTABLE);
        for offset in [0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u16(this + 0x1c), 0);
        assert_eq!(e.mem.u16(this + 0x2c), 0);
        assert_eq!(calls(&e, LIST_BOX_SET_UP_007A1950), vec![vec![this, 0, 1]]);
        assert_eq!(calls(&e, LIST_BOX_REGISTER), vec![vec![this]]);
        e.mem.set_u32(this + 4, 0);
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(this, 0);
        e.call(0x0075_a060, &args![this]);
        assert_eq!(e.mem.u32(this), LIST_BOX_VTABLE);
        assert_eq!(calls(&e, LIST_BOX_UNREGISTER), vec![vec![this]]);
        assert_eq!(calls(&e, LIST_DESTROY_0046FFB0), vec![vec![this + 4]]);
        assert_eq!(e.call(0x0075_aa20, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn list_box_add_item_appends_and_numbers_the_tile() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        assert_eq!(
            e.call(0x0075_a110, &args![this, 0x7100u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        e.mem.set_u32(this + 0xc, 0x6100);
        e.mem.set_u32(this + 0x18, 0x7200);
        e.mem.set_u16(this + 0x1c, 4);
        e.register(TILE_GET_MENU, |_, _| 0x4000u32.into_ret());
        e.register(MENU_RENDER_TEMPLATE, |_, _| 0x6200u32.into_ret());
        e.register(ALLOCATE, |_, _| 0x5800u32.into_ret());
        e.register(LIST_BOX_ITEM_CONSTRUCT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7100);
            a[0].into_ret()
        });
        e.register(LIST_BOX_COUNT, |_, _| 1u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| float_ret(3.0));
        e.register(TILE_GET_VALUE_Q, |_, _| 0x7a00u32.into_ret());
        let tile = e.call(0x0075_a110, &args![this, 0x7100u32, 0x7300u32, 0u32, 0u32]);
        assert_eq!(tile.u32(), 0x6200);
        assert_eq!(
            calls(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x4000, 0x6100, 0x7200, 0]]
        );
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![0x6200, TRAIT_ID, 0xffff_ffff],
                vec![0x6200, TRAIT_LIST_INDEX, 4]
            ]
        );
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x6200, TRAIT_STRING, 0x7300, 1]]
        );
        assert_eq!(calls(&e, LIST_APPEND)[0][0], this + 4);
        assert_eq!(e.mem.u16(this + 0x1c), 5);
        assert_eq!(
            calls(&e, VALUE_ADD_ACTION),
            vec![vec![0x7a00, ACTION_ADD, 3.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, VALUE_ADD_ACTION_TILE),
            vec![vec![0x7a00, ACTION_COPY, 0x6200, TRAIT_HEIGHT]]
        );
    }

    #[test]
    fn remove_all_deletes_the_items_and_their_tiles() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.put_vtable(0x0f00_5000, &[0x0f00_5100]);
        e.register(0x0f00_5100, |_, _| Ret::default());
        let tile = e.mem.alloc(4);
        e.mem.set_u32(tile, 0x0f00_5000);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, 0x7100);
        let first = e.mem.alloc(0xc);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(this + 4, first);
        e.mem.set_u32(this + 8, node);
        e.mem.set_u32(this + 0x10, 0x6300);
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_BOX_FLAG, |_, _| 1u32.into_ret());
        e.call(0x0075_a3b0, &args![this]);
        assert_eq!(calls(&e, 0x0f00_5100), vec![vec![tile, 1]]);
        assert_eq!(calls(&e, OBJECT_RELEASE), vec![vec![0x7100, 1]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![item], vec![first]]);
        assert_eq!(calls(&e, NODE_SET_NEXT), vec![vec![node, 0]]);
        assert_eq!(calls(&e, NODE_DELETE), vec![vec![node, 1]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        // With the keep flag, the tiles stay.
        e.set_global(KEEP_ITEM_TILES_011D8909, 1u8);
        e.mem.set_u32(this + 4, first);
        e.call(0x0075_a3b0, &args![this]);
        assert_eq!(calls(&e, 0x0f00_5100).len(), 1);
    }

    #[test]
    fn sort_sorts_then_synchronizes() {
        let mut e = engine();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x30);
        e.mem.set_u16(this + 0x1c, 3);
        e.call(0x0075_a5e0, &args![this, 0x0075_1234u32]);
        assert_eq!(calls(&e, LIST_SORT), vec![vec![this + 4, 0x0075_1234]]);
        assert_eq!(e.mem.u16(this + 0x1c), 0);
    }

    #[test]
    fn highlight_needs_an_enabled_list_and_tile() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        assert!(!e.call(0x0075_a610, &args![this, 0u32]).bool());
        e.mem.set_u32(this + 0xc, 0x6100);
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0075_a610, &args![this, 0x6200u32]).bool());
        e.register(LIST_BOX_HAS_TILE, |_, _| 1u32.into_ret());
        assert!(e.call(0x0075_a610, &args![this, 0x6200u32]).bool());
        assert_eq!(e.mem.u32(this + 0x10), 0x6200);
        assert!(e.call(0x0075_a610, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn the_highlight_target_moves_to_the_next_enabled_tile() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, 0x6100);
        e.mem.set_u16(this + 0x1c, 2);
        let item_a = e.mem.alloc(0xc);
        e.mem.set_u32(item_a, 0x6a00);
        let item_b = e.mem.alloc(0xc);
        e.mem.set_u32(item_b, 0x6b00);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, item_b);
        e.mem.set_u32(this + 4, item_a);
        e.mem.set_u32(this + 8, second);
        e.mem.set_u32(this + 0x10, 0x6a00);
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, a| {
            float_ret(if a[0] == 0x6b00 { 1.0 } else { 0.0 })
        });
        assert_eq!(e.call(0x0075_a680, &args![this, 2u32]).u32(), 0x6b00);
        // Up from the first stays on it, which is the highlighted one: 0.
        assert_eq!(e.call(0x0075_a680, &args![this, 1u32]).u32(), 0);
        // Another direction does not move.
        assert_eq!(e.call(0x0075_a680, &args![this, 5u32]).u32(), 0);
    }

    #[test]
    fn synchronize_numbers_the_unfiltered_tiles() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        let item_a = e.mem.alloc(0xc);
        e.mem.set_u32(item_a, 0x6a00);
        e.mem.set_u8(item_a + 8, 1);
        let item_b = e.mem.alloc(0xc);
        e.mem.set_u32(item_b, 0x6b00);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, item_b);
        e.mem.set_u32(this + 4, item_a);
        e.mem.set_u32(this + 8, second);
        e.mem.set_u16(this + 0x1c, 9);
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.call(0x0075_aa50, &args![this]);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x6b00, TRAIT_LIST_INDEX, 0]]
        );
        assert_eq!(
            calls(&e, LIST_BOX_INIT_TILE_POSITION),
            vec![vec![this, 0x6b00]]
        );
        assert_eq!(e.mem.u16(this + 0x1c), 1);
    }

    #[test]
    fn is_enabled_reads_the_container_through_a_static_trait() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.register(TILE_ADD_USER_TRAIT, |_, _| 0x2222u32.into_ret());
        assert!(!e.call(0x0075_ab20, &args![this]).bool());
        assert_eq!(e.global::<u32>(ENABLED_TRAIT_011D93D8), 0x2222);
        e.mem.set_u32(this + 0xc, 0x6100);
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        assert!(e.call(0x0075_ab20, &args![this]).bool());
        assert_eq!(calls(&e, TILE_IS_TRUE), vec![vec![0x6100, 0x2222]]);
        assert_eq!(calls(&e, TILE_ADD_USER_TRAIT).len(), 1);
    }

    #[test]
    fn the_highlight_box_follows_the_highlighted_tile() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        let mut ids = [0x31u32, 0x32].into_iter();
        e.register_double(TILE_ADD_USER_TRAIT, move |_, _| {
            ids.next().unwrap().into_ret()
        });
        e.mem.set_u32(this + 0xc, 0x6100);
        e.call(0x0075_abd0, &args![this]);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x6100, 0x31, 0xffff_ffff], vec![0x6100, 0x32, 0]]
        );
        e.mem.set_u32(this + 0x10, 0x6200);
        e.register(TILE_GET_VALUE, |_, a| (a[1] + 0x7000).into_ret());
        e.register(TILE_FLAG_0071AA80, |_, _| 1u32.into_ret());
        e.register(TILE_FIND, |_, _| 0x6300u32.into_ret());
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| (a[1] & 0xfff).into_ret());
        e.call(0x0075_abd0, &args![this]);
        assert_eq!(
            calls(&e, VALUE_ADD_ACTION_TILE),
            vec![
                vec![0x7031, ACTION_COPY, 0x6200, TRAIT_Y],
                vec![0x7032, ACTION_COPY, 0x6200, TRAIT_HEIGHT]
            ]
        );
        let marked: Vec<_> = calls(&e, TILE_MARK_0071AA60)
            .iter()
            .map(|w| (w[0], w[1]))
            .collect();
        assert_eq!(
            marked,
            vec![
                (TEXT_TOP & 0xfff, 1),
                (TEXT_BOTTOM & 0xfff, 1),
                (TEXT_LEFT & 0xfff, 1),
                (TEXT_RIGHT & 0xfff, 1),
                (0x6300, 1)
            ]
        );
    }

    #[test]
    fn the_total_height_gives_the_scroll_steps() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, 0x6100);
        e.mem.set_u32(this + 0x14, 0x6400);
        e.mem.set_u32(this + 0x20, 100.0f32.to_bits());
        e.register(TILE_GET_FLOAT, |_, a| {
            float_ret(if a[1] == TRAIT_HEIGHT { 40.0 } else { 0.0 })
        });
        e.register(LIST_BOX_COUNT, |_, _| 10u32.into_ret());
        e.register(SCROLL_STEPS, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0])))
        });
        e.register(STEP_LIMIT, |_, a| a[0].into_ret());
        e.register(TILE_TEXT_TO_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
        e.call(0x0075_aef0, &args![this]);
        // 10 * 60 / 100 = 6, plus 1: 7 steps; the delta is 60 / 6 = 10.
        assert_eq!(calls(&e, SCROLL_STEPS), vec![vec![6.0f32.to_bits()]]);
        assert_eq!(calls(&e, STEP_LIMIT), vec![vec![7, 2]]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![
                0x6100,
                TEXT_SCROLL_DELTA & 0xfff,
                10.0f32.to_bits(),
                1
            ]]
        );
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x6400, TEXT_NUMBER_OF_ITEMS & 0xfff, 7]]
        );
    }

    #[test]
    fn num_unfiltered_subtracts_the_truncated_filtered_count() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, 0x6100);
        e.register(LIST_BOX_COUNT, |_, _| 7u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| float_ret(2.9));
        e.register(TILE_ADD_USER_TRAIT, |_, _| 0x44u32.into_ret());
        assert_eq!(e.call(0x0075_b020, &args![this]).i32(), 5);
        assert_eq!(e.global::<u32>(NUM_FILTERED_TRAIT_011D93EC), 0x44);
        assert_eq!(calls(&e, TILE_GET_FLOAT), vec![vec![0x6100, 0x44]]);
    }
}
