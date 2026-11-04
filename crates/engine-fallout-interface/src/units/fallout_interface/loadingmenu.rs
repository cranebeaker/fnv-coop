//! `fallout/interface/menus/loadingmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `LoadingMenu` is the menu shown while a save or a cell loads (menu class
//! `0x3ef`). It keeps a list of `TESLoadScreen` forms (`LoadScreens`), cross
//! fades two background tiles and a text tile between them, and owns a
//! background thread object (`LoadingMenuThread`, global `0x011da0c4`).
//!
//! Layout and helpers are at the top of this file. The unit has 80 functions:
//! the first 40 (`00788730` to `0078b3f0`) were translated first, the second
//! 40 (`0078b650` to `0078d8d0`, the tile group routines, the placement of
//! the load screen text and tiles, the fade, the background thread and the
//! save/load of the level progress) follow the first ones. The unit is
//! complete; only `_dynamic_initializer_for_LoadingMenu::pWelcomeScreensA_`
//! (`00f90480`) is `library`.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`.
//!
//! Not translated: the compiler's exception-unwinding frames and the stack
//! cookie checks.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, BSStringT, NiColorA, NiTPointerList};

// ----- layout -----

/// Vtable of `LoadingMenu` (the destructor stores it as its first act).
pub(crate) const LOADING_MENU_VTABLE: u32 = 0x0107_3ebc;
/// The menu class number `LoadingMenu::GetClass` returns.
pub(crate) const LOADING_MENU_CLASS: u32 = 0x3ef;
/// Class of the menu `_Create` closes first (`0x3f4`).
pub(crate) const REPLACED_MENU_CLASS: u32 = 0x3f4;

/// `pTiles` (Xbox PDB): the tile pointers by byte offset from the menu, 38
/// words from `+0x28`; `AttachTileByID` fills them. Slots used by the
/// translations (the roles are read from how the code uses them):
pub(crate) const TILES: u32 = 0x28;
/// Number of tile slots `AttachTileByID` accepts.
pub(crate) const TILE_COUNT: i32 = 0x26;
/// The tile with the 3D model that holds the two slide nodes.
pub(crate) const SLOT_MODEL: u32 = 0x28;
/// Text tile fed by `78aec0` from a setting.
pub(crate) const SLOT_STATUS_TEXT: u32 = 0x34;
/// The loading wheel (the code warns when it has no model).
pub(crate) const SLOT_WHEEL: u32 = 0x54;
/// Description text tile.
pub(crate) const SLOT_DESCRIPTION: u32 = 0x5c;
/// Text tiles fed by `78aec0` from settings.
pub(crate) const SLOT_TEXT_60: u32 = 0x60;
pub(crate) const SLOT_TEXT_64: u32 = 0x64;
/// Tile that `78aec0` sets the 0x1004/0x1005 values of.
pub(crate) const SLOT_SCALED: u32 = 0x50;
/// The two background tiles that fade in and out alternately.
pub(crate) const SLOT_FADE_A: u32 = 0xa8;
pub(crate) const SLOT_FADE_B: u32 = 0xac;
/// The two tip text tiles and the matte tiles `Update` copies them to.
pub(crate) const SLOT_TIP_A: u32 = 0xb0;
pub(crate) const SLOT_TIP_B: u32 = 0xb4;
pub(crate) const SLOT_MATTE_A: u32 = 0xb8;
pub(crate) const SLOT_MATTE_B: u32 = 0xbc;
/// `m_pHoldTextures` (Xbox PDB): 16 `NiPointer<NiTexture>` on the Xbox; the
/// PC code walks and wraps at 4 (one word each).
pub(crate) const HOLD_TEXTURES: u32 = 0x20c;
pub(crate) const HOLD_TEXTURE_COUNT: u32 = 4;
/// `InitialSlideSound` and `SlideSound` (Xbox PDB): `BSSoundHandle`, 12 bytes.
pub(crate) const INITIAL_SLIDE_SOUND: u32 = 0x1f4;
pub(crate) const SLIDE_SOUND: u32 = 0x200;

layout! {
    /// `LoadingMenu` (Xbox PDB), 0x5c0 bytes on the Xbox (the PC size was
    /// not confirmed; the last field used ends at 0x5c0). Offsets checked
    /// against the PC code.
    pub struct LoadingMenu: 0x5c0 {
        /// Vtable (slot `0x00` `~Menu`, `0x04` `AttachTileByID`, `0x34`
        /// `GetClass`).
        0x00 vtable: u32,
        /// `pMenu` (Xbox PDB): `TileMenu*`.
        0x04 pMenu: Ptr,
        /// `rLoadScreenTypeTextWindowFaces` (Xbox PDB), a `tagRECT`: `left`.
        0xC0 faceLeft: i32,
        /// `top` of the same rectangle.
        0xC4 faceTop: i32,
        /// `right` of the same rectangle.
        0xC8 faceRight: i32,
        /// `bottom` of the same rectangle.
        0xCC faceBottom: i32,
        /// `StatsTilesList` (Xbox PDB): `NiTPointerList<Tile*>`.
        0xD0 StatsTilesList: Inline<NiTPointerList>,
        /// `ObjectiveTilesList` (Xbox PDB).
        0xDC ObjectiveTilesList: Inline<NiTPointerList>,
        /// `XPProgressTilesList` (Xbox PDB).
        0xE8 XPProgressTilesList: Inline<NiTPointerList>,
        /// `TipTilesList` (Xbox PDB).
        0xF4 TipTilesList: Inline<NiTPointerList>,
        /// `TipTilesList02` (Xbox PDB).
        0x100 TipTilesList02: Inline<NiTPointerList>,
        /// `strBuffer` (Xbox PDB): `BSStringT<char>`.
        0x1B8 strBuffer: Inline<BSStringT>,
        /// `SaveBackgroundColor` (Xbox PDB): `NiColorA`.
        0x1C0 SaveBackgroundColor: Inline<NiColorA>,
        /// `LoadScreens` (Xbox PDB): `BSSimpleList<TESLoadScreen*>`.
        0x1D0 LoadScreens: Inline<BSSimpleList>,
        /// `pNextLoadScreen` (Xbox PDB): the list node after the current one.
        0x1D8 pNextLoadScreen: Ptr,
        /// `pCurrentLoadScreen` (Xbox PDB): `TESLoadScreen*`.
        0x1DC pCurrentLoadScreen: Ptr,
        /// `uiNextBackgroundSwap` (Xbox PDB): tick count of the next swap.
        0x1E0 uiNextBackgroundSwap: u32,
        /// `uiFadeInComplete` (Xbox PDB).
        0x1E4 uiFadeInComplete: u32,
        /// `pLocation` (Xbox PDB): `TESForm*` the screen list was built for.
        0x1E8 pLocation: Ptr,
        /// `pSlide1TSP` (Xbox PDB): `TileShaderProperty*`.
        0x1EC pSlide1TSP: Ptr,
        /// `pSlide2TSP` (Xbox PDB).
        0x1F0 pSlide2TSP: Ptr,
        /// `m_nHoldTextureIndex` (Xbox PDB).
        0x21C m_nHoldTextureIndex: u32,
        /// `usFlags` (Xbox PDB): bits are tested and set through `00458110`
        /// and `00458080` (bit number as argument). Bits seen: 0 the list has
        /// one screen only, 1 slide toggle, 2 audio made multi-threaded,
        /// 3 re-show, 4 fade state, 5 overlay mode, 6 tips enabled,
        /// 7 first screen pending, 8 single screen, 9 interior location.
        0x222 usFlags: u16,
        /// `m_TileCrossFadeStartTime` (Xbox PDB).
        0x5B4 m_TileCrossFadeStartTime: u32,
        /// `m_SlideCount` (Xbox PDB).
        0x5B8 m_SlideCount: u32,
        /// `m_DelayedTipMatteUpdate` (Xbox PDB): bits 0 and 1 ask `Update`
        /// to copy a tip tile's values to its matte tile.
        0x5BC m_DelayedTipMatteUpdate: u32,
    }
}

// ----- globals -----

/// `LoadingMenu::pMe` (Xbox PDB): the one loading menu.
pub(crate) const MENU_INSTANCE: u32 = 0x011d_a0c0;
/// `LoadingMenu::pLoadingThread` (Xbox PDB): the background thread object.
pub(crate) const LOADING_THREAD: u32 = 0x011d_a0c4;
/// Byte set while `Create` reads the menu XML.
const READING_MENU_FILE: u32 = 0x011d_a0c8;
/// Byte set once `Update` has started the loading music.
pub(crate) const MUSIC_STARTED: u32 = 0x011d_a0c9;
/// `LoadingMenu::bInitialLoad` (Xbox PDB): the game's first load.
pub(crate) const INITIAL_LOAD: u32 = 0x011a_0294;
/// `LoadingMenu::bDoTileUpdate` (Xbox PDB).
pub(crate) const DO_TILE_UPDATE: u32 = 0x011a_0295;
/// Byte cleared by the destructor and tested by `00788ab0` and `_Create`.
pub(crate) const MENU_REPLACED: u32 = 0x011d_8907;
/// The byte `00788a90` stores and `00788aa0` returns.
const SWITCH_BYTE: u32 = 0x011d_cfb2;
/// Byte returned by `0078a930`.
const LIST_LOCKED: u32 = 0x011c_abb8;
/// Word: pointer to the object whose `+0xe4c` holds the loading movie.
const MOVIE_OWNER: u32 = 0x011d_ea3c;
/// Word: pointer to the movie player.
const MOVIE_PLAYER: u32 = 0x0126_fac4;
/// Word: object tested with `00678ce0` (byte at `+2`) for overlay mode.
const OVERLAY_OWNER: u32 = 0x011d_ea0c;
/// Word: object tested with `0042ce10` (flag `0x2` at `+0x244`).
const SKIP_OWNER: u32 = 0x011d_df38;
/// Word: the object `004568c0` (texture creation) is called on.
const TEXTURE_OWNER: u32 = 0x011d_ea10;
/// Word: object whose `QMultiBoundRadius`-named getter (`00526ac0`, float in
/// ST0) `_Create` and `Update` compare with 0.
const VALUE_OWNER: u32 = 0x011c_3f3c;
/// Word: pointer to the object that holds the list of all load screens at
/// `+0x128`.
const LOAD_SCREEN_OWNER: u32 = 0x011c_3f2c;
/// Table of pointers to words holding setting pointers (`0078a260`).
const SETTING_TABLE: u32 = 0x011c_6d50;
/// Globals the tip tiles are set from (`0078aec0`).
const TIP_SCALE: u32 = 0x011d_a16c;
const TIP_VALUE: u32 = 0x011d_a170;
const TIP_TEXT_OWNER: u32 = 0x011d_a168;
/// Floats `Update` stores when it starts the music.
const MUSIC_LEVEL_A: u32 = 0x011d_d300;
const MUSIC_LEVEL_B: u32 = 0x011d_d338;

// ----- settings (the INI setting objects; `00403df0` reads a string) -----

const SETTING_SCALE: u32 = 0x011d_2ad8;
const SETTING_MUSIC_PATH: u32 = 0x011d_e740;
const SETTING_TITLE_MUSIC: u32 = 0x011d_a0f0;
const SETTING_FIRST_SCREEN: u32 = 0x011d_a1b0;
/// Three words per `SettingT` in the screen setting array at `0x011da1b0`.
const SETTING_STRIDE: u32 = 12;
const SETTING_MAX_SCREENS: u32 = 0x011d_a138;
const SETTING_MAX_SCREENS_OVERLAY: u32 = 0x011d_a124;
const SETTING_LOCATION_SCREENS: u32 = 0x011d_a108;
const SETTING_INTERVAL_XUI: u32 = 0x011d_a190;
const SETTING_INTERVAL_INITIAL: u32 = 0x011d_a158;
const SETTING_INTERVAL_DEFAULT: u32 = 0x011d_a1a0;
const SETTING_DESCRIPTION: u32 = 0x011d_3000;
const SETTING_TIP_FALLBACK: u32 = 0x011d_1f8c;
const SETTING_TEXT_60: u32 = 0x011d_40f0;
const SETTING_TEXT_64: u32 = 0x011d_2358;
const SETTING_MUSIC_VOLUME: u32 = 0x011d_ed5c;

// ----- constants in .rdata -----

/// Two `double`s the `0xfa7` value of the menu tile is compared with.
const MENU_MODE_A: u32 = 0x0107_04f0;
const MENU_MODE_B: u32 = 0x0107_04e8;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `pi / 2` (`float`).
const HALF_PI: u32 = 0x0101_ff38;
/// `float` the slide model is scaled by.
const SLIDE_SCALE: u32 = 0x0107_3f20;
/// `double`s the tip matte tile values are moved by (15.0, 30.0).
const MATTE_OFFSET_DOWN: u32 = 0x0101_5a38;
const MATTE_OFFSET_UP: u32 = 0x0101_db88;
/// `double`: cross fade progress per millisecond.
const FADE_PER_MS: u32 = 0x0107_3fb8;
/// `double` 255.0.
const BYTE_SCALE: u32 = 0x0101_e568;
/// `double` 1000.0.
const MS_PER_SECOND: u32 = 0x0101_7b70;
/// `float` both music levels are set to.
const MUSIC_LEVEL: u32 = 0x0101_8180;
/// `float` 255.0: the alpha the second fade tile is set to.
const ALPHA_FULL: u32 = 0x0102_3cd8;

// Strings in .rdata.
const MENU_FILE_PATH: u32 = 0x0107_3f04;
const SLIDE_ONE: u32 = 0x0107_3f2c;
const SLIDE_TWO: u32 = 0x0107_3f24;
const WHEEL_WARNING: u32 = 0x0107_3f34;
const SEQUENCE_WARNING: u32 = 0x0107_3f68;
const MUSIC_PATH_FORMAT: u32 = 0x0107_3f9c;
const TEXTURE_PATH_FORMAT: u32 = 0x0101_dcc4;
const TEXTURES_DIRECTORY: u32 = 0x0101_7f88;
const SCREEN_PATH_FORMAT: u32 = 0x0107_4028;
const LOADING_SCREEN_PATH_FORMAT: u32 = 0x0107_400c;
const MAIN_DIRECTORY: u32 = 0x0105_c2b0;
const LOADING_DIRECTORY: u32 = 0x0107_403c;
/// Source file name the profiler guard in `0078a3e0` is given (with line
/// `0x423`).
const SOURCE_FILE_NAME: u32 = 0x0107_3fc0;
/// Sequence names `Update` plays, by flag bit 1, and the intro one.
const SEQUENCE_ODD: u32 = 0x0104_8458;
const SEQUENCE_EVEN: u32 = 0x0104_844c;
const SEQUENCE_INTRO: u32 = 0x0104_8444;
/// Address the destructor and `_Create` pass to the decal manager's vtable
/// slot `0xac`.
const DECAL_TARGET: u32 = 0x011a_9bd0;

// ----- callees outside this file, by exe address -----

/// Tests / sets bit `bit` of the 16-bit word at `this + 0x222`.
const FLAG_TEST: u32 = 0x0045_8110;
const FLAG_SET: u32 = 0x0045_8080;
/// Milliseconds (`timeGetTime` wrapper).
const TICK_COUNT: u32 = 0x0045_7fe0;
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__ehvec_dtor` (`00ec5fce`): array, element size, count, destructor.
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// `NiPointer` getter (the first word of `this`), assignment, destructor and
/// constructor taking a pointer.
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_SET: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const NI_POINTER_NEW: u32 = 0x0063_3c90;
/// `BSTexturePalette::RemoveTexture` (Xbox PDB).
const REMOVE_TEXTURE: u32 = 0x00a6_1f30;
/// Getters of the globals at `0x011d8a80`, `0x011f4748` and `0x011f6d98`.
const PLAYER_GETTER: u32 = 0x004b_7210;
const DECAL_MANAGER_GETTER: u32 = 0x0043_c4b0;
const AUDIO_GETTER: u32 = 0x0045_3a70;
/// Decal manager calls that bracket the tile changes.
const DECAL_BEGIN: u32 = 0x004a_0370;
const DECAL_END: u32 = 0x004a_03c0;
/// `BSSoundHandle` calls.
const SOUND_STOP: u32 = 0x00ad_88f0;
const SOUND_RELEASE: u32 = 0x00ad_8d10;
const SOUND_DESTRUCTOR: u32 = 0x0048_3710;
/// Destructors: `BSSimpleList`, `NiTPointerList`, `BSStringT`, and the
/// `Menu` base body.
const SIMPLE_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
const POINTER_LIST_DESTRUCTOR: u32 = 0x004a_1a30;
const STRING_DESTRUCTOR: u32 = 0x0040_37d0;
const MENU_BASE_DESTRUCTOR: u32 = 0x00a1_c520;
/// `NiTPointerList`: remove all, and add (the item is passed by address).
const POINTER_LIST_CLEAR: u32 = 0x004e_d900;
const POINTER_LIST_ADD: u32 = 0x004e_d8c0;
/// List iteration: the address of the item word of a `BSSimpleList` node,
/// the next node, and the `NiTPointerList` step that returns the address of
/// the element and advances the iterator whose address it is given.
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const POINTER_LIST_STEP: u32 = 0x0057_cbe0;
/// `BSSimpleList`: empty test, count, push at the head, append, free nodes.
const LIST_EMPTY: u32 = 0x0082_56d0;
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_PUSH: u32 = 0x005a_e3d0;
const LIST_ADD: u32 = 0x0090_5820;
const LIST_FREE_NODES: u32 = 0x0047_0470;
/// Tile calls: value getter (`float` in ST0), value setter, string setter,
/// `IsTrue`, `IsVisible`, `UpdateAll`, `GetMaximumDepth`, `GetMenuByClass`,
/// `GetMenu`, `ReadFile`, `Menu::SetMenuTile`.
const TILE_GET_VALUE: u32 = 0x00a0_11b0;
const TILE_SET_VALUE: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
const TILE_UPDATE_ALL: u32 = 0x00a0_4200;
const TILE_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// Sets a tile value from an integer (`float(value)`, flag 1).
const TILE_SET_INT: u32 = 0x0070_0320;
/// `Tile3D` calls: play a sequence by name, show frame, and the update of a
/// model node; the tile's model (the word at `+0x2c`).
const TILE_PLAY_SEQUENCE: u32 = 0x00a2_0cc0;
const TILE_SHOW_FRAME: u32 = 0x00a2_0ed0;
const NODE_UPDATE: u32 = 0x00a5_9c60;
const TILE_MODEL: u32 = 0x0056_c7f0;
/// `NiUpdateData` constructor (time, two bytes), 12 bytes.
const UPDATE_DATA_NEW: u32 = 0x0043_d410;
/// Sets tile value `id` (second argument) from a word (interface manager
/// helper that forwards to `TILE_SET_VALUE`).
const TILE_SET_ID_VALUE: u32 = 0x0071_5c60;
/// Setting value getters: address of the value (`00403e20`), string
/// (`00403df0`), `float` in ST0 (`00450410`), address of an unsigned value
/// (`0043d4d0`).
const SETTING_VALUE_POINTER: u32 = 0x0040_3e20;
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_FLOAT: u32 = 0x0045_0410;
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// String length, `memset`, `snprintf`, and the `BSStringT` formatter.
const STRING_LENGTH: u32 = 0x0044_a670;
const MEMSET: u32 = 0x0040_3d30;
const SNPRINTF: u32 = 0x0040_6d00;
const STRING_FORMAT: u32 = 0x0040_6f60;
/// CRT character conversion used by the name hash.
const CHAR_CONVERT: u32 = 0x00ec_67aa;
/// `_ftol2_sse`: truncates the `f64` in the first two words.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// Works on the `float` at the pointer; called with the bounds 0.0 and 1.0.
const CLAMP_FLOAT: u32 = 0x0053_30e0;
/// Form calls: type byte, name of the object at `+0x18`, name comparison,
/// set path, set temporary, flags word, "is temporary" test.
const FORM_TYPE: u32 = 0x0040_1170;
const FORM_NAME: u32 = 0x0040_8da0;
const NAME_COMPARE: u32 = 0x0040_8b20;
const SET_PATH: u32 = 0x0048_9100;
const SET_TEMPORARY: u32 = 0x0048_4490;
const WORD_AT_8: u32 = 0x0044_ddc0;
const IS_TEMPORARY: u32 = 0x0040_77c0;
/// Load screen calls: constructor, type, match for a location, generic
/// test, random index, minimum.
/// Bytes of a `TESLoadScreen` (`operator new` argument).
const LOAD_SCREEN_SIZE: u32 = 0x40;
const LOAD_SCREEN_NEW: u32 = 0x005a_66e0;
const LOAD_SCREEN_TYPE: u32 = 0x005a_7130;
const LOAD_SCREEN_FOR_LOCATION: u32 = 0x005a_6ec0;
/// True when the location list of the screen (`BSSimpleList` at `+0x2c`) is
/// empty (`005a7110` calls `008256d0` on it).
const LOAD_SCREEN_GENERIC: u32 = 0x005a_7110;
/// The word at `+0x34` of a load screen again (`005f36f0`: identical code
/// folded with `005a7130`).
const LOAD_SCREEN_TYPE_FOLDED: u32 = 0x005f_36f0;
const RANDOM_INDEX: u32 = 0x005a_00a0;
const MINIMUM: u32 = 0x0042_f5a0;
/// Candidate array (`BSSimpleArray`) calls: reset (`008454f0`, argument 0
/// keeps the buffer), element address, add, remove, and the head of the
/// global list of all load screens.
const CANDIDATES_RESET: u32 = 0x0084_54f0;
const CANDIDATE_AT: u32 = 0x0087_7a30;
const CANDIDATE_ADD: u32 = 0x007c_b2e0;
const CANDIDATE_REMOVE: u32 = 0x009a_4320;
const ALL_SCREENS_LIST: u32 = 0x0046_1070;
/// Temporary candidate `BSSimpleArray` (0x10 bytes, vtable `0107410c`):
/// constructor and destructor (in this unit, after
/// `0078b3f0`).
const CANDIDATE_LIST_NEW: u32 = 0x0078_d880;
const CANDIDATE_LIST_DESTRUCTOR: u32 = 0x0078_d8b0;
/// Misc state queries and actions.
const XUI_IS_UP: u32 = 0x0070_edf0;
const OVERLAY_TEST: u32 = 0x0067_8ce0;
const SKIP_TEST: u32 = 0x0042_ce10;
const MENU_CLASS_GETTER: u32 = 0x0070_ede0;
const FLAG_1A8_TEST: u32 = 0x004a_4080;
const SET_CURSOR_ALPHA: u32 = 0x0071_7740;
const SET_MENU_MODE: u32 = 0x0077_1700;
const SET_INFO_FOR_REF: u32 = 0x0077_5a00;
const SET_CROSSHAIR_TARGET_TYPE: u32 = 0x0070_3860;
const SET_CURRENT_SPELL: u32 = 0x0041_fd00;
const PLAYER_BYTE: u32 = 0x0042_4940;
const PLAYER_TILE: u32 = 0x0045_cd60;
const ROOT_TILE_GETTER: u32 = 0x0058_6150;
const PLAYING_SEQUENCE: u32 = 0x00ec_17c0;
const STOP_MOVIE: u32 = 0x00ec_25e0;
const START_SEQUENCE: u32 = 0x00ec_16d0;
const MOVIE_RELEASE: u32 = 0x0060_c9c0;
const MENU_SHOW_CHANGES: u32 = 0x0078_d080;
const LOCATION_FLAG: u32 = 0x0042_5fd0;
const REFR_GET_INTERIOR: u32 = 0x0057_5d10;
const TEXTURE_LOAD: u32 = 0x0045_68c0;
const TEXTURE_APPLY: u32 = 0x00bb_7a10;
const PROFILE_GUARD_NEW: u32 = 0x0040_4eb0;
const PROFILE_GUARD_DESTRUCTOR: u32 = 0x0040_4ee0;
/// Vtable slots: `GetObjectByName` and the node cast.
const OBJECT_BY_NAME_SLOT: u32 = 0x9c;
const AS_NODE_SLOT: u32 = 0x0c;
const NODE_CHILD: u32 = 0x0045_bc00;
const NODE_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_ID: u32 = 0x0043_8220;
const FIXED_STRING_NEW: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCTOR: u32 = 0x0043_81b0;
const MODEL_SET_SCALE: u32 = 0x0044_0490;
const MODEL_SET_ROTATION: u32 = 0x0064_04b0;
const MODEL_SHOW: u32 = 0x0045_0f90;
const LOG_LINE: u32 = 0x005b_5e40;
const REAL_SCREEN_WIDTH: u32 = 0x0070_6e40;
const REAL_SCREEN_HEIGHT: u32 = 0x0070_6e10;
const AUDIO_IS_ACTIVE: u32 = 0x004f_1540;
const AUDIO_IS_MULTI_THREADED: u32 = 0x005b_b4d0;
const AUDIO_SET_MULTI_THREADED: u32 = 0x00ad_7230;
const AUDIO_FLUSH: u32 = 0x00ad_7740;
const AUDIO_MUTE_TYPE: u32 = 0x00ad_8480;
const AUDIO_FADE: u32 = 0x005d_1740;
const MUSIC_FADE_OUT: u32 = 0x0083_0680;
const MUSIC_PLAY: u32 = 0x0083_00c0;
const MUSIC_UPDATE: u32 = 0x0083_0660;
const MUSIC_REFRESH: u32 = 0x0082_fb70;
const CREDITS_SET_FLAG: u32 = 0x0076_1460;
const SLEEP_TICK: u32 = 0x0040_fca0;
/// `QMultiBoundRadius`-named getter (`00526ac0`, `float` in ST0).
const VALUE_GETTER: u32 = 0x0052_6ac0;
/// Name of the object at `0x011da168` (`005ec330`).
const TEXT_OWNER_NAME: u32 = 0x005e_c330;
/// Getters on the model tile: the word at `+0x3c`, the word at `+0x38`, and
/// the call that takes them.
const TILE_SEQUENCE: u32 = 0x0063_9b40;
const TILE_TARGET: u32 = 0x009e_32d0;
const TILE_SEQUENCE_CALL: u32 = 0x0047_b220;
const SEQUENCE_STATE: u32 = 0x0080_41a0;

// Calls into this unit's later functions (translated by a later session).
const HIDE_ALL_BUT: u32 = 0x0078_cbd0;
const FADE_STEP: u32 = 0x0078_cc50;
const SET_TILE_FADE: u32 = 0x0078_ce10;
const SUSPEND_BACKGROUND_THREAD: u32 = 0x0078_cfc0;
const RESUME_BACKGROUND_THREAD: u32 = 0x0078_d020;
const THREAD_NEW: u32 = 0x0078_d2f0;
const REFRESH_STATS: u32 = 0x0078_d670;
const INIT_ENTER: u32 = 0x0078_b810;
const GROUP_VISIBLE_0078B6A0: u32 = 0x0078_b6a0;
const GROUP_VISIBLE_0078B650: u32 = 0x0078_b650;
const GROUP_VISIBLE_0078B6F0: u32 = 0x0078_b6f0;
const GROUP_VISIBLE_0078B7C0: u32 = 0x0078_b7c0;
const CALC_FACES: u32 = 0x0078_bb40;
const ANNOUNCE_SCREEN: u32 = 0x0078_bb60;

// ----- the second half of the unit: tile slots, thread layout, callees -----

/// Tile slots (byte offsets from the menu) the placement routines tell apart.
/// The roles are read from how the code lays them out.
/// The tile whose children are the left column of the stats rows.
pub(crate) const SLOT_STATS_COLUMN_A: u32 = 0x6c;
/// The tile whose children are the right column of the stats rows.
pub(crate) const SLOT_STATS_COLUMN_B: u32 = 0x70;
/// The tile whose value `0xfb0` is the height the stats rows share.
pub(crate) const SLOT_STATS_AREA: u32 = 0xa4;
/// The tile the objective layout positions (`0078bf80`).
pub(crate) const SLOT_OBJECTIVE: u32 = 0x94;
/// The experience bar tile (`0078c210`).
pub(crate) const SLOT_XP_BAR: u32 = 0x68;
/// The experience text tile (`0078c210`).
pub(crate) const SLOT_XP_TEXT: u32 = 0x64;
/// The tile `0078ce10` fades inverted, and the target of the fade-in image.
pub(crate) const SLOT_FADE_IMAGE: u32 = 0x58;
/// `DefaultTileSettings` (Xbox PDB): `LoadScreenDefaultProperties[38]`, 0x18
/// bytes each (`nOriginX`, `nOriginY`, `nHeight`, `nWidth`, `eFontType`,
/// `eJustification`), one per tile slot, read by the layout routines.
pub(crate) const DEFAULT_TILE_SETTINGS: u32 = 0x224;
pub(crate) const DEFAULT_TILE_SETTINGS_STRIDE: u32 = 0x18;

layout! {
    /// `LoadingMenuThread` (Xbox PDB), `BSThread` derived. The PC object is
    /// 0x4c bytes (`Create` allocates that); the semaphores sit at `0x30` and
    /// `0x3c` (the Xbox PDB has them 0x10 higher).
    pub struct LoadingMenuThread: 0x4c {
        /// Vtable (`0x010740ac`: slot `0` the scalar deleting destructor).
        0x00 vtable: u32,
        /// `bFlagSuspend` (Xbox PDB): the thread was asked to suspend itself.
        0x48 bFlagSuspend: u8,
        /// `bFlagSuspended` (Xbox PDB): the thread is waiting on its semaphore.
        0x49 bFlagSuspended: u8,
        /// `bExit` (Xbox PDB): the thread was asked to end.
        0x4A bExit: u8,
    }
}

/// `BackgroundThreadSema` (Xbox PDB), a `BSSemaphore` (3 words: count, handle,
/// maximum) at this offset of the thread; the thread waits on it.
pub(crate) const BACKGROUND_SEMAPHORE: u32 = 0x30;
/// `MainThreadSema` (Xbox PDB): the thread releases it when it suspends, the
/// caller of `SuspendBackgroundThread` waits on it.
pub(crate) const MAIN_SEMAPHORE: u32 = 0x3c;
/// Vtable of `LoadingMenuThread` and its name string (`"LoadingMenu"`).
const THREAD_VTABLE: u32 = 0x0107_40ac;
const THREAD_NAME: u32 = 0x0107_409c;
const THREAD_STACK_SIZE: u32 = 0x4000;
/// Vtable of the temporary candidate `BSSimpleArray` (`0078d880`).
const CANDIDATE_ARRAY_VTABLE: u32 = 0x0107_410c;
/// Its initializer (`BSSimpleArray` body: two zero arguments).
const CANDIDATE_ARRAY_INIT: u32 = 0x006b_3eb0;

// Globals the second half reads and writes.
/// Byte `0078bb60` clears once the screen's tiles are placed (the byte after
/// `bDoTileUpdate`).
const LAYOUT_FLAG_011A0296: u32 = 0x011a_0296;
/// Float `FadeStep` computes once (a function-local `static`: the value of
/// tile value `0xfc1` of the menu tile times 1000) and its guard word.
const FADE_DURATION: u32 = 0x011d_a1f0;
const FADE_DURATION_GUARD: u32 = 0x011d_a1f4;
/// Byte set while `ShowChanges` redraws.
const REDRAWING: u32 = 0x011c_70f8;
/// The critical section `ShowChanges` takes first, and the recursive lock
/// object `0078d1f0` takes (owner thread id, count).
const REDRAW_SECTION: u32 = 0x011f_4380;
const REDRAW_LOCK: u32 = 0x011f_4480;
/// The `NiPointer` holding the rendered texture (`0078cdf0`).
const RENDERED_TEXTURE: u32 = 0x011d_ed3c;
/// Word: pointer to the object whose `0087f9f0` gives the player's level and
/// whose embedded object at `+0xa4` answers value queries (slot `0xc`).
const PLAYER_OWNER: u32 = 0x011d_ea3c;
/// INI setting (`0043d4d0` returns the address of its unsigned value): levels
/// at or above it have no progress.
const SETTING_LEVEL_CAP: u32 = 0x011d_0c60;
/// Format string `"%d"` of the XP layout.
const INT_FORMAT: u32 = 0x0102_0774;
/// `"WARNING: LoadingMenu::CalcLoadScreenTypeTextWindowVerts() received a
/// NULL apData."`
const NULL_DATA_WARNING: u32 = 0x0107_4048;
/// `"Loading Menu:  Invalid Level Progress Values..."` (log format).
const LEVEL_PROGRESS_WARNING: u32 = 0x0107_40b8;
/// `double`s: 1280.0 and 960.0 (the design resolution) and 2.0.
const DESIGN_WIDTH: u32 = 0x0106_e960;
const DESIGN_HEIGHT: u32 = 0x0106_e7f8;
const TWO: u32 = 0x0101_1590;
/// Type number of the tile kind `0078ce10` fades through its model.
const MODEL_TILE_TYPE: u32 = 0x388;
/// Run-time type descriptors `0078d770` casts the loaded form between.
const FORM_TYPE_DESCRIPTOR: u32 = 0x0118_3028;
const OWNER_TYPE_DESCRIPTOR: u32 = 0x0118_6500;

// Callees outside this file used by the second half.
/// Step of the children list of a tile (the list object is at `tile + 4`; its
/// first node is the word `00726070` returns): returns the address of the
/// item and advances the iterator at the pointer (item at `+8`, next at `+4`).
const CHILD_STEP: u32 = 0x0068_3520;
/// `Tile::AddNeedsUpdate` (Xbox PDB) and `Tile::RecursiveRotationUpdate`.
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
const TILE_ROTATION_UPDATE: u32 = 0x00a0_b520;
/// `float` in ST0: screen width (`InterfaceManager::GetScreenWidth`, Xbox
/// PDB) and height, and the two insets subtracted from them (each reads one
/// of two INI integers by a mode test).
const SCREEN_WIDTH: u32 = 0x0071_5d40;
const SCREEN_HEIGHT: u32 = 0x0071_5da0;
const INSET_X: u32 = 0x0071_77c0;
const INSET_Y: u32 = 0x0071_7820;
/// `float` max and min of two `float` stack arguments (cdecl).
const FLOAT_MAX: u32 = 0x0040_4010;
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// Logs an error (cdecl, the format string address as argument).
const ERROR_LOG: u32 = 0x0040_fbe0;
/// Font manager: the global getter (`0x011f33f8`), the `float` at `+0x2c` of
/// a font, and `FontManager::CalculateStringDimensions` (Xbox PDB; writes a
/// size of three floats into its first argument and returns it).
const FONT_MANAGER: u32 = 0x005b_d5b0;
const FONT_VALUE: u32 = 0x0075_9450;
const STRING_DIMENSIONS: u32 = 0x00a1_b020;
/// `BSRenderedTexture::GetTexture` (Xbox PDB) and
/// `TileImage::SetSourceTexture` (Xbox PDB).
const RENDERED_GET_TEXTURE: u32 = 0x004b_c320;
const IMAGE_SET_SOURCE_TEXTURE: u32 = 0x00a2_0610;
/// Passes a fade `float` to the shader property of a model object.
const MODEL_FADE: u32 = 0x0082_1600;
/// The number of children of a node and the child at an index.
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// Slot of the type number in a tile's vtable, and the slot of the cast that
/// answers non-null for objects that carry the shader property.
const TILE_TYPE_SLOT: u32 = 0x0c;
const HAS_PROPERTY_SLOT: u32 = 0x18;
/// `BSThread` calls: constructor body, destructor body, `SetName`,
/// `Initialize` (stack size, name, 0), `SetThreadProcessor`,
/// `SetThreadPriority`, and the test `0078cfc0` makes (`bThreadIsActive`).
const BS_THREAD_NEW: u32 = 0x00aa_6360;
const BS_THREAD_DESTRUCTOR: u32 = 0x00aa_63c0;
const BS_THREAD_SET_NAME: u32 = 0x00aa_6540;
const BS_THREAD_INITIALIZE: u32 = 0x00aa_6430;
const BS_THREAD_SET_PROCESSOR: u32 = 0x00aa_6550;
const BS_THREAD_SET_PRIORITY: u32 = 0x00aa_6560;
const BS_THREAD_IS_ACTIVE: u32 = 0x0089_05f0;
/// `BSSemaphore` calls: release (`ReleaseSemaphore`), wait
/// (`WaitForSingleObject`) and the destructor body.
const SEMAPHORE_RELEASE: u32 = 0x0044_2550;
const SEMAPHORE_WAIT: u32 = 0x0044_24e0;
const SEMAPHORE_DESTRUCTOR: u32 = 0x0055_a2d0;
/// `CreateSemaphoreA` import slot (attributes, initial count, maximum, name).
const CREATE_SEMAPHORE: u32 = 0x00fd_f0b4;
/// `TryEnterCriticalSection` import slot.
const TRY_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f1b0;
/// `GetCurrentThreadId` wrapper, and the interlocked compare exchange wrapper
/// (destination, exchange, comparand: returns the previous value).
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
const COMPARE_EXCHANGE: u32 = 0x0043_b460;
/// `ShowChanges` callees: the getter of the section `0044b130` (try to
/// lock) and `0082f1f0` (unlock) work on, the redraw flush, the render
/// system getter and its calls.
const REDRAW_SECTION_GETTER: u32 = 0x0071_3d70;
const SECTION_TRY_LOCK: u32 = 0x0044_b130;
const SECTION_UNLOCK: u32 = 0x0082_f1f0;
const REDRAW_FLUSH: u32 = 0x0045_2530;
const RENDER_SYSTEM: u32 = 0x004e_a970;
const RENDER_BEGIN: u32 = 0x00ba_2f30;
const RENDER_END: u32 = 0x00ba_2fa0;
const RENDER_STAGE_A: u32 = 0x00ba_30f0;
const RENDER_STAGE_B: u32 = 0x00ba_3130;
const RENDER_SCENE: u32 = 0x0071_3fb0;
/// `0087f9f0` (the player's level in `AX`), `00952b30`, `00648b50`
/// (`GamePlayFormulas::GetRequiredExperiencePoints`, Xbox PDB, cdecl) and
/// the getter `0044edb0` (the engine map's `BaseProcess::GetCurrentProcedureIndex`).
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
const PLAYER_PROCESS: u32 = 0x0095_2b30;
const REQUIRED_EXPERIENCE: u32 = 0x0064_8b50;
const PROCESS_VALUE: u32 = 0x0044_edb0;
/// Save/load buffer calls: save a form id (id, 0), save bytes (pointer,
/// size, 0), load a form id, load bytes (pointer, size), form lookup (cdecl),
/// `__RTDynamicCast` (cdecl) and the lookup of the owner by handle.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_FORM_ID: u32 = 0x0086_48a0;
const LOAD_BYTES: u32 = 0x0086_4980;
const FORM_LOOKUP: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const OWNER_BY_HANDLE: u32 = 0x0060_c8e0;
/// Cdecl calls `0078d770` ends with (a byte, and the constant 1).
const APPLY_SAVED_BYTE: u32 = 0x005c_25e0;
const AFTER_LOAD: u32 = 0x0045_81c0;

// ----- small helpers -----

fn now(e: &mut Engine) -> u32 {
    e.call(TICK_COUNT, &args![]).u32()
}

/// Whether bit `bit` of the state word is set (`00458110`).
fn flag(e: &mut Engine, this: Ptr<LoadingMenu>, bit: u32) -> bool {
    e.call(FLAG_TEST, &args![this, bit]).bool()
}

/// Sets or clears bit `bit` of the state word (`00458080`).
fn set_flag(e: &mut Engine, this: Ptr<LoadingMenu>, bit: u32, value: u8) {
    e.call(FLAG_SET, &args![this, bit, value]);
}

/// The tile pointer stored at byte offset `offset` of the menu.
fn tile(e: &Engine, this: Ptr<LoadingMenu>, offset: u32) -> u32 {
    e.mem.u32(this.addr() + offset)
}

fn load_screens(this: Ptr<LoadingMenu>) -> u32 {
    this.at(LoadingMenu::LoadScreens).addr()
}

fn string_buffer(this: Ptr<LoadingMenu>) -> u32 {
    this.at(LoadingMenu::strBuffer).addr()
}

fn tile_value(e: &mut Engine, tile: u32, id: u32) -> f32 {
    e.call(TILE_GET_VALUE, &args![tile, id]).f32()
}

fn set_tile_value(e: &mut Engine, tile: u32, id: u32, value: f32, flag: u32) {
    e.call(TILE_SET_VALUE, &args![tile, id, value, flag]);
}

fn set_tile_int(e: &mut Engine, tile: u32, id: u32, value: u32) {
    e.call(TILE_SET_INT, &args![tile, id, value]);
}

fn set_tile_string(e: &mut Engine, tile: u32, id: u32, text: u32, flag: u32) {
    e.call(TILE_SET_STRING, &args![tile, id, text, flag]);
}

/// The `TESLoadScreen` held by the list node `node` (`006815c0` returns the
/// address of the item word).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

fn next_node(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    e.call(SETTING_FLOAT, &args![setting]).f32()
}

fn setting_string(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_STRING, &args![setting]).u32()
}

/// An unsigned INI setting's value (`0043d4d0` returns its address).
fn setting_int(e: &mut Engine, setting: u32) -> u32 {
    let pointer = e.call(SETTING_INT_POINTER, &args![setting]).u32();
    e.mem.u32(pointer)
}

fn model_of(e: &mut Engine, tile: u32) -> u32 {
    e.call(TILE_MODEL, &args![tile]).u32()
}

fn xui_is_up(e: &mut Engine) -> bool {
    e.call(XUI_IS_UP, &args![]).bool()
}

fn overlay_active(e: &mut Engine) -> bool {
    let owner = e.mem.u32(OVERLAY_OWNER);
    e.call(OVERLAY_TEST, &args![owner]).bool()
}

fn audio_instance(e: &mut Engine) -> u32 {
    e.call(AUDIO_GETTER, &args![]).u32()
}

/// Updates the decal manager around a tile change: the pair of calls
/// `004a0370` / `004a03c0` on the manager the global getter returns.
fn decals_begin(e: &mut Engine) {
    let manager = e.call(DECAL_MANAGER_GETTER, &args![]).u32();
    e.call(DECAL_BEGIN, &args![manager]);
}

fn decals_end(e: &mut Engine) {
    let manager = e.call(DECAL_MANAGER_GETTER, &args![]).u32();
    e.call(DECAL_END, &args![manager]);
}

/// The rotation call both model setups make: `pi/2`, axis `(1, 0, 0)`.
fn rotate_half_pi(e: &mut Engine, model: u32) {
    let angle: f32 = e.global(HALF_PI);
    e.call(
        MODEL_SET_ROTATION,
        &args![model, angle, 1.0f32, 0.0f32, 0.0f32],
    );
}

/// A `float` clamped through `005330e0`, which works on the float at the
/// pointer it gets (called with the bounds 0.0 and 1.0).
fn clamp_unit(e: &mut Engine, value: f32) -> f32 {
    e.with_stack(4, |e, cell| {
        e.mem.set_f32(cell.addr(), value);
        e.call(CLAMP_FLOAT, &args![cell, 0.0f32, 1.0f32]);
        e.mem.f32(cell.addr())
    })
}

/// `_ftol2_sse` on a value the game holds in ST0.
fn float_to_int(e: &mut Engine, value: f64) -> u32 {
    e.call(FLOAT_TO_INT, &args![value]).u32()
}

// Translated from 00788730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::GetClass` (Xbox PDB), vtable slot `0x34`: the menu class.
pub fn loading_menu_get_class(_e: &mut Engine, _this: Ptr<LoadingMenu>) -> u32 {
    LOADING_MENU_CLASS
}

// Translated from 00788740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::AttachTileByID` (Xbox PDB), vtable slot `0x04`: stores a
/// non-null tile into `pTiles[id]` when `0 <= id < 0x26`.
pub fn loading_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<LoadingMenu>, id: i32, tile: u32) {
    if tile != 0 && (0..TILE_COUNT).contains(&id) {
        e.mem.set_u32(this.addr() + TILES + id as u32 * 4, tile);
    }
}

// Translated from 00788770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body and, when bit 0 of `flags` is set, `operator delete`.
pub fn loading_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<LoadingMenu>,
    flags: u32,
) -> Ptr<LoadingMenu> {
    fn_007887a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007887a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `LoadingMenu` (no Xbox PDB name on the map): stops
/// the background thread, releases the hold textures, stops the loading
/// music and sounds, clears `pMe` and the loading movie, destroys the
/// members and runs the `Menu` base destructor. The exception-unwinding
/// states are not translated.
pub fn fn_007887a0(e: &mut Engine, this: Ptr<LoadingMenu>) {
    e.set(this, LoadingMenu::vtable, LOADING_MENU_VTABLE);
    e.call(SUSPEND_BACKGROUND_THREAD, &args![]);
    for index in 0..HOLD_TEXTURE_COUNT {
        let pointer = this.addr() + HOLD_TEXTURES + index * 4;
        if e.call(NI_POINTER_GET, &args![pointer]).u32() != 0 {
            let texture = e.call(NI_POINTER_GET, &args![pointer]).u32();
            e.call(REMOVE_TEXTURE, &args![texture]);
            e.call(NI_POINTER_SET, &args![pointer, 0u32]);
        }
    }
    e.mem.set_u8(MENU_REPLACED, 0);
    let manager = e.call(DECAL_MANAGER_GETTER, &args![]).u32();
    e.vcall(
        manager,
        0xac,
        &args![this.at(LoadingMenu::SaveBackgroundColor)],
    );
    let mut stop_music = xui_is_up(e);
    if !stop_music && e.mem.u8(INITIAL_LOAD) != 0 {
        let volume = setting_string(e, SETTING_MUSIC_VOLUME);
        stop_music = e.mem.i8(volume) != 0;
    }
    if stop_music {
        let audio = audio_instance(e);
        if e.call(AUDIO_IS_ACTIVE, &args![audio]).bool() {
            e.call(MUSIC_FADE_OUT, &args![4000u32, 0u32]);
        }
    }
    if fn_00788aa0(e) != 0 {
        fn_00788a90(e, 0);
        e.call(CREDITS_SET_FLAG, &args![0u32]);
        e.call(MUSIC_FADE_OUT, &args![4000u32, 0u32]);
    }
    if flag(e, this, 2) {
        let audio = audio_instance(e);
        if e.call(AUDIO_IS_MULTI_THREADED, &args![audio]).bool() {
            let audio = audio_instance(e);
            e.call(AUDIO_SET_MULTI_THREADED, &args![audio, 0u32]);
        }
    }
    let audio = audio_instance(e);
    e.call(AUDIO_FADE, &args![audio, 1000u32]);
    e.mem.set_u8(INITIAL_LOAD, 0);
    fn_0078a940(e, this);
    let slide_sound = this.addr() + SLIDE_SOUND;
    let initial_sound = this.addr() + INITIAL_SLIDE_SOUND;
    e.call(SOUND_STOP, &args![slide_sound]);
    e.call(SOUND_RELEASE, &args![slide_sound]);
    e.call(SOUND_RELEASE, &args![initial_sound]);
    e.set(this, LoadingMenu::pLocation, Ptr::NULL);
    e.mem.set_u32(MENU_INSTANCE, 0);
    let owner = e.mem.u32(MOVIE_OWNER);
    if owner != 0 && e.mem.u32(owner + 0xe4c) != 0 {
        let movie = e.mem.u32(owner + 0xe4c);
        e.call(MOVIE_RELEASE, &args![movie, 1u32]);
        e.mem.set_u32(owner + 0xe4c, 0);
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![
            this.addr() + HOLD_TEXTURES,
            4u32,
            HOLD_TEXTURE_COUNT,
            NI_POINTER_DESTRUCTOR
        ],
    );
    e.call(SOUND_DESTRUCTOR, &args![slide_sound]);
    e.call(SOUND_DESTRUCTOR, &args![initial_sound]);
    e.call(SIMPLE_LIST_DESTRUCTOR, &args![load_screens(this)]);
    e.call(STRING_DESTRUCTOR, &args![string_buffer(this)]);
    for list in [
        this.at(LoadingMenu::TipTilesList02),
        this.at(LoadingMenu::TipTilesList),
        this.at(LoadingMenu::XPProgressTilesList),
        this.at(LoadingMenu::ObjectiveTilesList),
        this.at(LoadingMenu::StatsTilesList),
    ] {
        e.call(POINTER_LIST_DESTRUCTOR, &args![list]);
    }
    e.call(MENU_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00788a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in the global at `0x011dcfb2` (the destructor clears it
/// after stopping the music it guards).
pub fn fn_00788a90(e: &mut Engine, value: u8) {
    e.mem.set_u8(SWITCH_BYTE, value);
}

// Translated from 00788aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte stored by `00788a90`.
pub fn fn_00788aa0(e: &mut Engine) -> u8 {
    e.mem.u8(SWITCH_BYTE)
}

// Translated from 00788ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-shows an existing loading menu for `location` (called by `Create`
/// after it set state bit 3; does nothing when the bit is clear): suspends
/// the thread, restarts the cross fade clock 4 s in the past, closes the menu
/// of class `0x3f5` when `0x011d8907` or the skip test says so, rebuilds the
/// load-screen list and steps two slides, then shows the wheel and resumes
/// the thread.
pub fn fn_00788ab0(e: &mut Engine, this: Ptr<LoadingMenu>, location: u32) {
    if !flag(e, this, 3) {
        return;
    }
    e.call(SUSPEND_BACKGROUND_THREAD, &args![]);
    let start = now(e).wrapping_sub(4000);
    e.set(this, LoadingMenu::m_TileCrossFadeStartTime, start);
    let skip_owner = e.mem.u32(SKIP_OWNER);
    if e.mem.u8(MENU_REPLACED) != 0 || e.call(SKIP_TEST, &args![skip_owner]).bool() {
        let class = e.call(MENU_CLASS_GETTER, &args![]).u32();
        let found = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
        let menu = e.call(TILE_GET_MENU, &args![found]).u32();
        if menu != 0 && !e.call(FLAG_1A8_TEST, &args![menu, 1u32]).bool() {
            e.vcall(menu, 0, &args![1u32]);
        }
    }
    if overlay_active(e) {
        fn_0078afa0(e, this, 0);
    } else {
        fn_0078afa0(e, this, 1);
        fn_0078aec0(e, this);
    }
    e.set(this, LoadingMenu::pLocation, Ptr::new(location));
    loading_menu_make_load_screen_list(e, this);
    e.set(this, LoadingMenu::pLocation, Ptr::new(location));
    let swap = now(e);
    e.set(this, LoadingMenu::uiNextBackgroundSwap, swap);
    for _ in 0..2 {
        let count = e.get(this, LoadingMenu::m_SlideCount);
        e.set(this, LoadingMenu::m_SlideCount, count.wrapping_add(1));
        fn_0078a280(e, this, 0);
        fn_0078a9f0(e, this);
        fn_0078a3e0(e, this, 0);
        fn_0078abd0(e, this);
    }
    set_flag(e, this, 3, 0);
    decals_begin(e);
    let wheel = tile(e, this, SLOT_WHEEL);
    if !e.call(TILE_IS_VISIBLE, &args![wheel]).bool() {
        set_tile_int(e, wheel, 0xfa3, 1);
        fn_0078a230(e, this);
    }
    let player = e.call(PLAYER_GETTER, &args![]).u32();
    let player_tile = e.call(PLAYER_TILE, &args![player]).u32();
    set_tile_int(e, player_tile, 0xfa3, 0);
    e.call(TILE_UPDATE_ALL, &args![1u32]);
    decals_end(e);
    e.mem.set_u8(DO_TILE_UPDATE, 1);
    e.call(RESUME_BACKGROUND_THREAD, &args![]);
}

// Translated from 00788cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::Create` (Xbox PDB, `cdecl`, three words): when a menu of
/// the loading menu's class exists it sets bit 3, re-shows it
/// (`00788ab0`) and stores `hidden_flag` in bit 10; otherwise it reads
/// `Data\Menus\loading_menu.xml`, and when the file's menu is a loading menu
/// builds it (tile lists, visibility, `_Create`, the background thread) and
/// returns the root tile. Returns 0 when the file holds no loading menu
/// (that menu is deleted if it has a tile).
pub fn loading_menu_create(
    e: &mut Engine,
    location: u32,
    resume_thread: u8,
    hidden_flag: u8,
) -> u32 {
    let class = fn_00788f20(e);
    let existing = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if existing != 0 {
        let menu = Ptr::<LoadingMenu>::new(e.mem.u32(MENU_INSTANCE));
        set_flag(e, menu, 3, 1);
        fn_00788ab0(e, menu, location);
        let menu = Ptr::<LoadingMenu>::new(e.mem.u32(MENU_INSTANCE));
        set_flag(e, menu, 10, hidden_flag);
        return existing;
    }
    e.mem.set_u8(READING_MENU_FILE, 1);
    let player = e.call(PLAYER_GETTER, &args![]).u32();
    let root = e.call(ROOT_TILE_GETTER, &args![player]).u32();
    let tile_root = e.call(TILE_READ_FILE, &args![root, MENU_FILE_PATH]).u32();
    e.mem.set_u8(READING_MENU_FILE, 0);
    let menu = e.call(TILE_GET_MENU, &args![tile_root]).u32();
    if menu != 0 {
        let class = e.vcall(menu, 0x34, &args![]).u32();
        if class == fn_00788f20(e) {
            e.call(SET_CURSOR_ALPHA, &args![0u32]);
            e.call(MENU_SET_MENU_TILE, &args![menu, tile_root, 0u32]);
            e.mem.set_u32(MENU_INSTANCE, menu);
            let me = Ptr::<LoadingMenu>::new(menu);
            e.call(INIT_ENTER, &args![me]);
            fn_00788f30(e, me);
            fn_00788f80(e, me);
            fn_00789080(e, me);
            fn_00788fe0(e, me);
            e.call(GROUP_VISIBLE_0078B6A0, &args![me, 0u32]);
            e.call(GROUP_VISIBLE_0078B6F0, &args![me, 0u32]);
            e.call(GROUP_VISIBLE_0078B650, &args![me, 0u32]);
            e.call(GROUP_VISIBLE_0078B7C0, &args![me, 0u32]);
            loading_menu_create_impl(e, me, location);
            set_flag(e, me, 10, hidden_flag);
            if e.mem.u32(LOADING_THREAD) == 0 {
                let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
                let thread = if block != 0 {
                    e.call(THREAD_NEW, &args![block]).u32()
                } else {
                    0
                };
                e.mem.set_u32(LOADING_THREAD, thread);
            } else if resume_thread != 0 {
                e.call(RESUME_BACKGROUND_THREAD, &args![]);
            }
            let fade_a = tile(e, me, SLOT_FADE_A);
            set_tile_int(e, fade_a, 0xfa9, 0);
            let fade_b = tile(e, me, SLOT_FADE_B);
            set_tile_int(e, fade_b, 0xfa9, 0);
            return tile_root;
        }
    }
    if menu != 0 && next_node(e, menu) != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

// Translated from 00788f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A static twin of `GetClass`: the loading menu's class, `0x3ef`.
pub fn fn_00788f20(_e: &mut Engine) -> u32 {
    LOADING_MENU_CLASS
}

/// Clears the tile list at `list` (offset in the menu) and adds the tile
/// slots at `slots` (offsets from the menu), each passed by address as
/// `NiTPointerList::AddItem` takes it.
fn fill_tile_list(e: &mut Engine, this: Ptr<LoadingMenu>, list: u32, slots: &[u32]) {
    e.call(POINTER_LIST_CLEAR, &args![this.addr() + list]);
    for slot in slots {
        e.call(
            POINTER_LIST_ADD,
            &args![this.addr() + list, this.addr() + slot],
        );
    }
}

// Translated from 00788f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `StatsTilesList` with the tile slots `+0x6c` and `+0x70`.
pub fn fn_00788f30(e: &mut Engine, this: Ptr<LoadingMenu>) {
    fill_tile_list(e, this, LoadingMenu::StatsTilesList.off, &[0x6c, 0x70]);
}

// Translated from 00788f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `ObjectiveTilesList` with the slots `+0x94`, `+0x34` and `+0x60`.
pub fn fn_00788f80(e: &mut Engine, this: Ptr<LoadingMenu>) {
    fill_tile_list(
        e,
        this,
        LoadingMenu::ObjectiveTilesList.off,
        &[0x94, 0x34, 0x60],
    );
}

// Translated from 00788fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `XPProgressTilesList` with the slots `+0x68`, `+0x98`, `+0x9c`,
/// `+0x64`, `+0x50` and `+0xa0`.
pub fn fn_00788fe0(e: &mut Engine, this: Ptr<LoadingMenu>) {
    fill_tile_list(
        e,
        this,
        LoadingMenu::XPProgressTilesList.off,
        &[0x68, 0x98, 0x9c, 0x64, 0x50, 0xa0],
    );
}

// Translated from 00789080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `TipTilesList` with the slot `+0xb0` and `TipTilesList02` with
/// `+0xb4`.
pub fn fn_00789080(e: &mut Engine, this: Ptr<LoadingMenu>) {
    fill_tile_list(e, this, LoadingMenu::TipTilesList.off, &[SLOT_TIP_A]);
    fill_tile_list(e, this, LoadingMenu::TipTilesList02.off, &[SLOT_TIP_B]);
}

/// The object the virtual `GetObjectByName` (slot `0x9c`) of `model` finds
/// for `name`, which the game passes as a temporary `NiFixedString` on its
/// stack.
fn find_object(e: &mut Engine, model: u32, name: u32) -> u32 {
    e.with_stack(4, |e, fixed| {
        let fixed_string = e.call(FIXED_STRING_NEW, &args![fixed, name]).u32();
        let object = e
            .vcall(model, OBJECT_BY_NAME_SLOT, &args![fixed_string])
            .u32();
        e.call(FIXED_STRING_DESTRUCTOR, &args![fixed]);
        object
    })
}

/// The shader property (`NiAVObject::GetProperty`) of a slide object: of its
/// first child when the object is a node, else of the object itself.
fn slide_property(e: &mut Engine, object: u32, is_node: bool) -> u32 {
    let property = e.call(PROPERTY_ID, &args![]).u32();
    if is_node {
        let child = e.call(NODE_CHILD, &args![object, 0u32]).u32();
        e.call(NODE_PROPERTY, &args![child, property]).u32()
    } else {
        e.call(NODE_PROPERTY, &args![object, property]).u32()
    }
}

/// Switches the audio system to multi-threaded mode when it is not yet and
/// records that in state bit 2.
fn make_audio_multi_threaded(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let instance = audio_instance(e);
    if !e.call(AUDIO_IS_MULTI_THREADED, &args![instance]).bool() {
        let instance = audio_instance(e);
        e.call(AUDIO_SET_MULTI_THREADED, &args![instance, 1u32]);
        set_flag(e, this, 2, 1);
    }
}

// Translated from 007890e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::_Create` (Xbox PDB): second stage of building the menu.
/// Fixes the depth value of the menu tile, makes the player's current spell
/// `2` unless its byte is `3`, sets the HUD mode, closes the `0x3f4` menu,
/// then either hides everything (initial load or a nonzero value) or lays
/// out for an interior/exterior chosen from the location's form type, sets up
/// the wheel and the two slide models, builds the load-screen list and
/// starts the audio.
pub fn loading_menu_create_impl(e: &mut Engine, this: Ptr<LoadingMenu>, location: u32) {
    e.set(this, LoadingMenu::uiNextBackgroundSwap, 0);
    let menu_tile = e.get(this, LoadingMenu::pMenu).addr();
    let mode = tile_value(e, menu_tile, 0xfa7) as f64;
    if mode == e.global::<f64>(MENU_MODE_A) || mode == e.global::<f64>(MENU_MODE_B) {
        let depth = e.call(TILE_MAXIMUM_DEPTH, &args![]).f32();
        set_tile_value(e, menu_tile, 0xfad, depth, 1);
    }
    let player = e.call(PLAYER_GETTER, &args![]).u32();
    if e.call(PLAYER_BYTE, &args![player]).u8() as i8 != 3 {
        let player = e.call(PLAYER_GETTER, &args![]).u32();
        e.call(SET_CURRENT_SPELL, &args![player, 2u32]);
    }
    e.call(SET_MENU_MODE, &args![5u32]);
    e.call(SET_INFO_FOR_REF, &args![0u32, 0u32, 0u32]);
    e.call(SET_CROSSHAIR_TARGET_TYPE, &args![0u32]);
    if e.mem.u32(MOVIE_OWNER) != 0 {
        let player = e.mem.u32(MOVIE_PLAYER);
        if !e.call(PLAYING_SEQUENCE, &args![player]).bool() {
            let player = e.mem.u32(MOVIE_PLAYER);
            e.call(STOP_MOVIE, &args![player]);
        }
    }
    let replaced = e
        .call(TILE_GET_MENU_BY_CLASS, &args![REPLACED_MENU_CLASS])
        .u32();
    if replaced != 0 {
        e.vcall(replaced, 0, &args![1u32]);
    }
    e.set(this, LoadingMenu::pLocation, Ptr::new(location));
    // 00789222: the value is read only when the initial-load byte is clear (JNZ past it).
    let hide_all = e.mem.u8(INITIAL_LOAD) != 0 || {
        let value_owner = e.mem.u32(VALUE_OWNER);
        let value = e.call(VALUE_GETTER, &args![value_owner]).f32() as f64;
        value != e.global::<f64>(ZERO)
    };
    if hide_all {
        e.call(HIDE_ALL_BUT, &args![this, 0u32, 0u32]);
    } else if e.mem.u8(MENU_REPLACED) != 0 || overlay_active(e) {
        fn_0078afa0(e, this, 0);
        decals_begin(e);
        fn_0078a230(e, this);
        decals_end(e);
    } else {
        let location = e.get(this, LoadingMenu::pLocation).addr();
        let skip_owner = e.mem.u32(SKIP_OWNER);
        if location != 0 && !e.call(SKIP_TEST, &args![skip_owner]).bool() {
            let form_type = e.call(FORM_TYPE, &args![location]).u32();
            match form_type.wrapping_sub(0x39) {
                0 => {
                    let interior = e.call(LOCATION_FLAG, &args![location]).u8();
                    set_flag(e, this, 9, interior);
                }
                1..=7 | 0x30 => {
                    let interior = e.call(REFR_GET_INTERIOR, &args![location]).u8();
                    set_flag(e, this, 9, interior);
                }
                _ => {}
            }
        }
        if flag(e, this, 9) {
            e.call(HIDE_ALL_BUT, &args![this, 0xbu32, 0u32]);
            set_flag(e, this, 4, 0);
            let fade_in = now(e);
            e.set(this, LoadingMenu::uiFadeInComplete, fade_in);
            e.mem.set_u8(DO_TILE_UPDATE, 1);
        } else {
            fn_0078afa0(e, this, 1);
            fn_0078aec0(e, this);
        }
        decals_begin(e);
        fn_0078a230(e, this);
        decals_end(e);
    }
    let manager = e.call(DECAL_MANAGER_GETTER, &args![]).u32();
    e.vcall(manager, 0xac, &args![DECAL_TARGET]);
    let wheel_tile = tile(e, this, SLOT_WHEEL);
    if model_of(e, wheel_tile) != 0 {
        let scale = setting_float(e, SETTING_SCALE);
        let node = model_of(e, wheel_tile);
        e.call(MODEL_SET_SCALE, &args![node, scale]);
        let value_pointer = e.call(SETTING_VALUE_POINTER, &args![SETTING_SCALE]).u32();
        let value = e.mem.f32(value_pointer);
        set_tile_value(e, wheel_tile, 0x1004, value, 1);
        let node = model_of(e, wheel_tile);
        rotate_half_pi(e, node);
    } else {
        e.call(LOG_LINE, &args![0x12u32, WHEEL_WARNING]);
    }
    if !flag(e, this, 9) {
        let model_tile = tile(e, this, SLOT_MODEL);
        let model = model_of(e, model_tile);
        let slide_one = find_object(e, model, SLIDE_ONE);
        let as_node = e.vcall(slide_one, AS_NODE_SLOT, &args![]).u32();
        if as_node != 0 {
            let property = slide_property(e, as_node, true);
            e.set(this, LoadingMenu::pSlide1TSP, Ptr::new(property));
            let model = model_of(e, model_tile);
            let slide_two = find_object(e, model, SLIDE_TWO);
            let property = slide_property(e, slide_two, true);
            e.set(this, LoadingMenu::pSlide2TSP, Ptr::new(property));
        } else {
            let property = slide_property(e, slide_one, false);
            e.set(this, LoadingMenu::pSlide1TSP, Ptr::new(property));
            let model = model_of(e, model_tile);
            let slide_two = find_object(e, model, SLIDE_TWO);
            let property = slide_property(e, slide_two, false);
            e.set(this, LoadingMenu::pSlide2TSP, Ptr::new(property));
        }
        loading_menu_make_load_screen_list(e, this);
        if e.mem.u8(INITIAL_LOAD) == 0 {
            fn_0078a280(e, this, 0);
            fn_0078a9f0(e, this);
            fn_0078a3e0(e, this, 0);
            fn_0078abd0(e, this);
        }
        let slide_scale: f32 = e.global(SLIDE_SCALE);
        let node = model_of(e, model_tile);
        e.call(MODEL_SET_SCALE, &args![node, slide_scale]);
        let node = model_of(e, model_tile);
        rotate_half_pi(e, node);
        let width = e.call(REAL_SCREEN_WIDTH, &args![]).f32() as f64;
        let height = e.call(REAL_SCREEN_HEIGHT, &args![]).f32() as f64;
        set_tile_value(e, model_tile, 0x1004, (width / height) as f32, 1);
        e.call(TILE_PLAY_SEQUENCE, &args![model_tile, SEQUENCE_INTRO]);
        e.with_stack(12, |e, update_data| {
            e.call(UPDATE_DATA_NEW, &args![update_data, 0.0f32, 1u32, 0u32]);
            let node = model_of(e, model_tile);
            e.call(NODE_UPDATE, &args![node, update_data]);
        });
        e.call(FADE_STEP, &args![this]);
        e.set(this, LoadingMenu::uiFadeInComplete, 0);
    }
    e.call(MENU_SHOW_CHANGES, &args![]);
    let mut handled = false;
    if e.mem.u8(INITIAL_LOAD) != 0 || location == 0 {
        let instance = audio_instance(e);
        if e.call(AUDIO_IS_ACTIVE, &args![instance]).bool() {
            let volume = setting_string(e, SETTING_MUSIC_VOLUME);
            if e.mem.i8(volume) == 0 {
                make_audio_multi_threaded(e, this);
                handled = true;
            }
        }
    }
    if !handled && e.mem.u8(INITIAL_LOAD) == 0 {
        let instance = audio_instance(e);
        if e.call(AUDIO_IS_ACTIVE, &args![instance]).bool() {
            make_audio_multi_threaded(e, this);
            let instance = audio_instance(e);
            e.call(AUDIO_MUTE_TYPE, &args![instance, 2u32, 1000u32, 6000u32]);
        }
    }
    e.set(this, LoadingMenu::m_TileCrossFadeStartTime, 0);
    let model_tile = tile(e, this, SLOT_MODEL);
    let model = model_of(e, model_tile);
    e.call(MODEL_SHOW, &args![model, 1u32]);
}

/// Copies the tip tile `source` values `0xfa2` (minus 15.0) and `0xfb0`
/// (plus 30.0) to the matte tile `target`.
fn copy_tip_matte(e: &mut Engine, this: Ptr<LoadingMenu>, source: u32, target: u32) {
    let source = tile(e, this, source);
    let target = tile(e, this, target);
    let first = (tile_value(e, source, 0xfa2) as f64 - e.global::<f64>(MATTE_OFFSET_DOWN)) as f32;
    set_tile_value(e, target, 0xfa2, first, 1);
    let second = (tile_value(e, source, 0xfb0) as f64 + e.global::<f64>(MATTE_OFFSET_UP)) as f32;
    set_tile_value(e, target, 0xfb0, second, 1);
}

/// Starts the title music: formats `Data\Music\Special\%s.mp3` with the
/// title music setting and hands it to the music player.
fn start_title_music(e: &mut Engine) {
    e.with_stack(0x40, |e, buffer| {
        e.call(MEMSET, &args![buffer, 0u32, 0x40u32]);
        let name = setting_string(e, SETTING_TITLE_MUSIC);
        e.call(SNPRINTF, &args![buffer, 0x40u32, MUSIC_PATH_FORMAT, name]);
        e.call(CREDITS_SET_FLAG, &args![1u32]);
        e.call(
            MUSIC_PLAY,
            &args![8u32, buffer, 0u32, 1u32, 1u32, 0.0f32, 0u32],
        );
        e.call(MUSIC_UPDATE, &args![]);
        let level: f32 = e.global(MUSIC_LEVEL);
        e.set_global(MUSIC_LEVEL_A, level);
        e.set_global(MUSIC_LEVEL_B, level);
    });
}

// Translated from 00789820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::Update` (Xbox PDB), no arguments: once per frame. Applies
/// the delayed tip matte copies, cross fades the two background tiles by the
/// time since `m_TileCrossFadeStartTime`, during the first 3 s starts the
/// title music on the initial load, afterwards swaps to the next load screen
/// when its time has come (or plays the next slide sequence), and finally
/// refreshes the wheel and the audio.
///
/// The wait for `0x011dea10` (while the initial load is shown and no swap
/// time is set) spins on `Sleep(5)`; it ends when the loading thread sets
/// the word, which a one-thread engine never does by itself.
pub fn loading_menu_update(e: &mut Engine, this: Ptr<LoadingMenu>) {
    e.mem.set_u8(DO_TILE_UPDATE, 1);
    let pending = e.get(this, LoadingMenu::m_DelayedTipMatteUpdate);
    if pending & 1 != 0 {
        e.set(this, LoadingMenu::m_DelayedTipMatteUpdate, pending & !1);
        copy_tip_matte(e, this, SLOT_TIP_A, SLOT_MATTE_A);
    }
    let pending = e.get(this, LoadingMenu::m_DelayedTipMatteUpdate);
    if pending & 2 != 0 {
        e.set(this, LoadingMenu::m_DelayedTipMatteUpdate, pending & !2);
        copy_tip_matte(e, this, SLOT_TIP_B, SLOT_MATTE_B);
    }
    let time = now(e);
    if !flag(e, this, 4) {
        let start = e.get(this, LoadingMenu::m_TileCrossFadeStartTime);
        let delta = time.wrapping_sub(start);
        let mut fade = (delta as f64 * e.global::<f64>(FADE_PER_MS)) as f32;
        if start == 0 {
            fade = 1.0;
        }
        fade = clamp_unit(e, fade);
        if e.get(this, LoadingMenu::m_SlideCount) % 2 == 0 {
            fade = (1.0 - fade as f64) as f32;
        }
        let incoming = clamp_unit(e, fade);
        let fade_a = tile(e, this, SLOT_FADE_A);
        if fade_a != 0 {
            let alpha = float_to_int(e, incoming as f64 * e.global::<f64>(BYTE_SCALE));
            set_tile_int(e, fade_a, 0xfa9, alpha);
        }
        let outgoing = clamp_unit(e, (1.0 - fade as f64) as f32);
        let fade_b = tile(e, this, SLOT_FADE_B);
        if fade_b != 0 {
            let alpha = float_to_int(e, outgoing as f64 * e.global::<f64>(BYTE_SCALE));
            set_tile_int(e, fade_b, 0xfa9, alpha);
        }
        let current = e.get(this, LoadingMenu::pCurrentLoadScreen).addr();
        fn_0078afc0(e, this, current, incoming, 1);
        let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
        let next_screen = node_item(e, next);
        fn_0078afc0(e, this, next_screen, outgoing, 2);
        e.mem.set_u8(DO_TILE_UPDATE, 1);
    }
    let elapsed = time.wrapping_sub(e.get(this, LoadingMenu::m_TileCrossFadeStartTime));
    if elapsed < 3000 {
        let wheel = tile(e, this, SLOT_WHEEL);
        // The title music starts once: with a music path set (first branch)
        // the movie player also starts it; with none (second branch) only the
        // music player does.
        let music_path_set = |e: &mut Engine| {
            let path = setting_string(e, SETTING_MUSIC_PATH);
            e.call(STRING_LENGTH, &args![path]).u32() != 0
        };
        if e.mem.u8(INITIAL_LOAD) != 0
            && !flag(e, this, 0)
            && !playing_sequence(e)
            && music_path_set(e)
            && e.mem.u8(MUSIC_STARTED) == 0
        {
            start_title_music(e);
            let model_tile = tile(e, this, SLOT_MODEL);
            let model = model_of(e, model_tile);
            e.call(MODEL_SHOW, &args![model, 1u32]);
            let fade_a = tile(e, this, SLOT_FADE_A);
            set_tile_value(e, fade_a, 0xfa9, 0.0, 1);
            let full: f32 = e.global(ALPHA_FULL);
            let fade_b = tile(e, this, SLOT_FADE_B);
            set_tile_value(e, fade_b, 0xfa9, full, 1);
            let path = setting_string(e, SETTING_MUSIC_PATH);
            let player = e.mem.u32(MOVIE_PLAYER);
            e.call(
                START_SEQUENCE,
                &args![player, path, 0u32, 0xffff_ffffu32, 0u32, 0u32, 0.0f32],
            );
            e.mem.set_u8(MUSIC_STARTED, 1);
            set_tile_int(e, wheel, 0xfa3, 1);
            e.mem.set_u8(DO_TILE_UPDATE, 1);
        } else if e.mem.u8(INITIAL_LOAD) != 0
            && !flag(e, this, 0)
            && !playing_sequence(e)
            && e.mem.u8(MUSIC_STARTED) == 0
            && !music_path_set(e)
        {
            start_title_music(e);
            e.mem.set_u8(MUSIC_STARTED, 1);
        }
    } else if e.get(this, LoadingMenu::uiNextBackgroundSwap) == 0 {
        if e.mem.u8(INITIAL_LOAD) != 0 {
            while e.mem.u32(TEXTURE_OWNER) == 0 {
                e.call(SLEEP_TICK, &args![5u32]);
            }
            let slide = e.get(this, LoadingMenu::pSlide1TSP).addr();
            fn_0078a3e0(e, this, slide);
        }
        if e.get(this, LoadingMenu::uiFadeInComplete) == 0 && flag(e, this, 4) {
            e.call(FADE_STEP, &args![this]);
        } else {
            let fade_in = now(e);
            e.set(this, LoadingMenu::uiFadeInComplete, fade_in);
        }
        let fade_in = e.get(this, LoadingMenu::uiFadeInComplete);
        e.set(this, LoadingMenu::uiNextBackgroundSwap, fade_in);
    } else if flag(e, this, 4) {
        e.call(FADE_STEP, &args![this]);
    } else if !flag(e, this, 9) && now(e) >= e.get(this, LoadingMenu::uiNextBackgroundSwap) {
        swap_load_screen(e, this);
    }
    let wheel = tile(e, this, SLOT_WHEEL);
    if e.call(TILE_IS_VISIBLE, &args![wheel]).bool() {
        fn_0078a230(e, this);
    }
    let instance = audio_instance(e);
    e.call(AUDIO_FLUSH, &args![instance, 1u32]);
    e.call(MUSIC_REFRESH, &args![]);
}

fn playing_sequence(e: &mut Engine) -> bool {
    let player = e.mem.u32(MOVIE_PLAYER);
    e.call(PLAYING_SEQUENCE, &args![player]).bool()
}

/// The part of `Update` that runs when the swap time has come: with the
/// model tile's sequence finished it moves on to the next screen, otherwise
/// it plays the next slide sequence.
fn swap_load_screen(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let model_tile = tile(e, this, SLOT_MODEL);
    if fn_0078a1f0(e, model_tile) {
        if e.mem.u8(INITIAL_LOAD) == 0 {
            let value_owner = e.mem.u32(VALUE_OWNER);
            let value = e.call(VALUE_GETTER, &args![value_owner]).f32() as f64;
            if value == e.global::<f64>(ZERO) && !xui_is_up(e) {
                let tip = tile(e, this, SLOT_TIP_A);
                if !flag(e, this, 3) || e.call(TILE_IS_TRUE, &args![tip, 0xfa3u32]).bool() {
                    fn_0078a280(e, this, 0);
                }
            }
        }
        fn_0078a9f0(e, this);
        fn_0078a3e0(e, this, 0);
        let sequence = e.call(TILE_SEQUENCE, &args![model_tile]).u32();
        let target = e.call(TILE_TARGET, &args![model_tile]).u32();
        e.call(TILE_SEQUENCE_CALL, &args![target, sequence, 0.0f32]);
        fn_0078abd0(e, this);
        let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
        if e.mem.u8(INITIAL_LOAD) != 0 && next == load_screens(this) {
            set_flag(e, this, 0, 1);
        }
        return;
    }
    if e.mem.u8(INITIAL_LOAD) != 0 && flag(e, this, 0) {
        return;
    }
    if flag(e, this, 3) || flag(e, this, 8) {
        return;
    }
    let time = now(e);
    e.set(this, LoadingMenu::uiNextBackgroundSwap, time);
    fn_0078a1c0(e, this, 1);
    let sequence = if flag(e, this, 1) {
        SEQUENCE_ODD
    } else {
        SEQUENCE_EVEN
    };
    if e.call(TILE_PLAY_SEQUENCE, &args![model_tile, sequence])
        .bool()
    {
        let time = now(e);
        e.set(this, LoadingMenu::m_TileCrossFadeStartTime, time);
        let count = e.get(this, LoadingMenu::m_SlideCount);
        e.set(this, LoadingMenu::m_SlideCount, count.wrapping_add(1));
        if flag(e, this, 7) {
            let screens = e.call(LIST_COUNT, &args![load_screens(this)]).u32();
            set_flag(e, this, 8, (screens == 1) as u8);
            set_flag(e, this, 7, 0);
        }
        e.with_stack(12, |e, update_data| {
            e.call(UPDATE_DATA_NEW, &args![update_data, 0.0f32, 1u32, 0u32]);
            let node = model_of(e, model_tile);
            e.call(NODE_UPDATE, &args![node, update_data]);
        });
    } else {
        let sequence = if flag(e, this, 1) {
            SEQUENCE_ODD
        } else {
            SEQUENCE_EVEN
        };
        e.call(LOG_LINE, &args![SEQUENCE_WARNING, sequence]);
    }
}

// Translated from 0078a1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Toggles bit `bit` of the state word (`usFlags`).
pub fn fn_0078a1c0(e: &mut Engine, this: Ptr<LoadingMenu>, bit: u32) {
    let flags = e.get(this, LoadingMenu::usFlags);
    let toggled = flags ^ (1u32 << (bit & 31)) as u16;
    e.set(this, LoadingMenu::usFlags, toggled);
}

// Translated from 0078a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the tile `model_tile` (the word at `+0x3c` is its current
/// sequence) has a sequence whose state word (`+0x44`, `008041a0`) is 1.
pub fn fn_0078a1f0(e: &mut Engine, model_tile: u32) -> bool {
    let sequence = e.mem.u32(model_tile + 0x3c);
    sequence != 0 && e.call(SEQUENCE_STATE, &args![sequence]).u32() == 1
}

// Translated from 0078a230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows frame `-1.0` of the wheel tile (`Tile3D` `a20ed0`).
pub fn fn_0078a230(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let wheel = tile(e, this, SLOT_WHEEL);
    let frame: f32 = e.global(MINUS_ONE);
    e.call(TILE_SHOW_FRAME, &args![wheel, frame, 0u32]);
}

// Translated from 0078a260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string of the setting that the table at `0x011c6d50` lists at
/// `index` (`cdecl`, one word).
pub fn fn_0078a260(e: &mut Engine, index: u32) -> u32 {
    let entry = e.mem.u32(SETTING_TABLE + index * 4);
    let setting = e.mem.u32(entry);
    setting_string(e, setting)
}

// Translated from 0078a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the description text of the next load screen on one of the two tip
/// tiles, alternating by `m_SlideCount`, and asks `Update` (bit 0 or 1 of
/// `m_DelayedTipMatteUpdate`) to copy it to the matte; also sets the
/// description tile from a setting. The word argument is not used. The name
/// hash it computes from the screen's name (times 33 plus the converted
/// character, modulo 100000) is not used either, but the conversion calls
/// are made.
pub fn fn_0078a280(e: &mut Engine, this: Ptr<LoadingMenu>, _unused_0: u32) {
    let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
    let screen = node_item(e, next);
    let mut name = e.call(FORM_NAME, &args![screen + 0x18]).u32();
    let mut hash: u32 = 0;
    while e.mem.i8(name) != 0 {
        let character = e.mem.i8(name) as i32;
        let converted = e.call(CHAR_CONVERT, &args![character]).u32();
        hash = (hash << 5).wrapping_add(hash).wrapping_add(converted);
        name += 1;
    }
    let _name_hash = hash % 100_000;
    let (tip, bit) = if e.get(this, LoadingMenu::m_SlideCount) % 2 == 0 {
        (SLOT_TIP_A, 1)
    } else {
        (SLOT_TIP_B, 2)
    };
    let pending = e.get(this, LoadingMenu::m_DelayedTipMatteUpdate);
    e.set(this, LoadingMenu::m_DelayedTipMatteUpdate, pending | bit);
    let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
    let screen = node_item(e, next);
    // `TESDescription::GetText`-like: vtable slot 0x10 of the sub-object at
    // +0x24 with (0, 'DESC').
    let text = e
        .vcall(screen + 0x24, 0x10, &args![0u32, 0x4353_4544u32])
        .u32();
    let tip_tile = tile(e, this, tip);
    set_tile_string(e, tip_tile, 0xfc4, text, 1);
    let description = setting_string(e, SETTING_DESCRIPTION);
    let description_tile = tile(e, this, SLOT_DESCRIPTION);
    set_tile_string(e, description_tile, 0xfc4, description, 1);
    e.mem.set_u8(DO_TILE_UPDATE, 1);
}

// Translated from 0078a3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the texture `Textures\<name of the next load screen>` into the next
/// ring-buffer slot of `m_pHoldTextures` (releasing the texture that slot
/// held) and applies it to the fading tile that is next (`+0xa8` for even
/// `m_SlideCount`, else `+0xac`; virtual slot `0x20` of that tile gives the
/// shader property). The word argument is overwritten before it is read. The
/// profiler guard (`00404eb0` with line `0x423`) is created and destroyed
/// around it.
pub fn fn_0078a3e0(e: &mut Engine, this: Ptr<LoadingMenu>, _unused_0: u32) {
    e.with_stack(4, |e, guard| {
        e.call(
            PROFILE_GUARD_NEW,
            &args![guard, 0xdu32, 1u32, SOURCE_FILE_NAME, 0x423u32],
        );
        let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
        let screen = node_item(e, next);
        let name = e.call(FORM_NAME, &args![screen + 0x18]).u32();
        e.call(
            STRING_FORMAT,
            &args![
                string_buffer(this),
                TEXTURE_PATH_FORMAT,
                TEXTURES_DIRECTORY,
                name
            ],
        );
        e.with_stack(4, |e, texture| {
            e.call(NI_POINTER_NEW, &args![texture, 0u32]);
            let path = e.call(NI_POINTER_GET, &args![string_buffer(this)]).u32();
            let owner = e.mem.u32(TEXTURE_OWNER);
            e.call(TEXTURE_LOAD, &args![owner, path, texture, 0u32, 0u32]);
            let index = e.get(this, LoadingMenu::m_nHoldTextureIndex);
            let slot = this.addr() + HOLD_TEXTURES + index * 4;
            let next_index = index.wrapping_add(1);
            e.set(this, LoadingMenu::m_nHoldTextureIndex, next_index);
            if next_index >= HOLD_TEXTURE_COUNT {
                e.set(this, LoadingMenu::m_nHoldTextureIndex, 0);
            }
            if e.call(NI_POINTER_GET, &args![slot]).u32() != 0 {
                let held = e.call(NI_POINTER_GET, &args![slot]).u32();
                e.call(REMOVE_TEXTURE, &args![held]);
            }
            let loaded = e.call(NI_POINTER_GET, &args![texture]).u32();
            e.call(NI_POINTER_SET, &args![slot, loaded]);
            let fading = if e.get(this, LoadingMenu::m_SlideCount) % 2 == 0 {
                tile(e, this, SLOT_FADE_A)
            } else {
                tile(e, this, SLOT_FADE_B)
            };
            let property = e.vcall(fading, 0x20, &args![]).u32();
            if property != 0 {
                let loaded = e.call(NI_POINTER_GET, &args![texture]).u32();
                e.call(TEXTURE_APPLY, &args![property, loaded]);
            }
            e.call(NI_POINTER_DESTRUCTOR, &args![texture]);
        });
        e.call(PROFILE_GUARD_DESTRUCTOR, &args![guard]);
    });
}

/// A new temporary `TESLoadScreen` whose path is the text in `strBuffer`.
fn new_load_screen(e: &mut Engine, this: Ptr<LoadingMenu>) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![LOAD_SCREEN_SIZE]).u32();
    let screen = if block != 0 {
        e.call(LOAD_SCREEN_NEW, &args![block]).u32()
    } else {
        0
    };
    let path = e.call(NI_POINTER_GET, &args![string_buffer(this)]).u32();
    e.call(SET_PATH, &args![screen + 0x18, path]);
    e.call(SET_TEMPORARY, &args![screen]);
    screen
}

/// Appends `screen` to the menu's screen list with `add` (`LIST_ADD` or
/// `LIST_PUSH`, which take the address of a word holding the item).
fn add_load_screen(e: &mut Engine, this: Ptr<LoadingMenu>, add: u32, screen: u32) {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), screen);
        e.call(add, &args![load_screens(this), cell]);
    });
}

// Translated from 0078a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::MakeLoadScreenList` (Xbox PDB): rebuilds `LoadScreens`.
/// A non-empty list is cleared first. On the initial load (and no XUI) it
/// builds one screen per setting from `Interface\Loading\<setting>.dds`
/// (or `Main` for the last), starting at the third when a music path is
/// set; otherwise it collects the candidates for the location and picks
/// from them (`BuildCandidateList`, `0078adc0`, once more without a
/// location when too few were found). An empty list gets one default
/// screen. Then the first screen becomes current and is shown (`0078b3f0`).
pub fn loading_menu_make_load_screen_list(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let mut cleared = false;
    if !e.call(LIST_EMPTY, &args![load_screens(this)]).bool() {
        fn_0078a940(e, this);
        cleared = true;
    }
    let overlay = xui_is_up(e) || overlay_active(e);
    set_flag(e, this, 5, overlay as u8);
    if e.mem.u8(INITIAL_LOAD) != 0 && !xui_is_up(e) {
        let music = setting_string(e, SETTING_MUSIC_PATH);
        let start = if e.call(STRING_LENGTH, &args![music]).u32() != 0 {
            3
        } else {
            0
        };
        e.with_stack(4, |e, index| {
            e.mem.set_i32(index.addr(), start);
            while e.mem.i32(index.addr()) < 5 {
                let position = e.mem.i32(index.addr());
                let directory = if position == 4 {
                    MAIN_DIRECTORY
                } else {
                    LOADING_DIRECTORY
                };
                let name =
                    setting_string(e, SETTING_FIRST_SCREEN + position as u32 * SETTING_STRIDE);
                e.call(
                    STRING_FORMAT,
                    &args![string_buffer(this), SCREEN_PATH_FORMAT, directory, name],
                );
                let screen = new_load_screen(e, this);
                add_load_screen(e, this, LIST_ADD, screen);
                fn_0078a910(e, index.addr(), 0);
            }
        });
    } else {
        e.with_stack(0x10, |e, candidates| {
            e.call(CANDIDATE_LIST_NEW, &args![candidates]);
            loading_menu_build_candidate_list(e, this, candidates.addr());
            fn_0078adc0(e, this, candidates.addr());
            if e.get(this, LoadingMenu::pLocation).addr() != 0 {
                let count = e.call(LIST_COUNT, &args![load_screens(this)]).u32();
                let limit = setting_int(e, SETTING_MAX_SCREENS);
                if count < limit && !fn_0078a930(e) {
                    e.set(this, LoadingMenu::pLocation, Ptr::NULL);
                    loading_menu_build_candidate_list(e, this, candidates.addr());
                    fn_0078adc0(e, this, candidates.addr());
                }
            }
            e.call(CANDIDATE_LIST_DESTRUCTOR, &args![candidates]);
        });
    }
    if e.call(LIST_EMPTY, &args![load_screens(this)]).bool() {
        let name = setting_string(e, SETTING_FIRST_SCREEN);
        e.call(
            STRING_FORMAT,
            &args![string_buffer(this), LOADING_SCREEN_PATH_FORMAT, name],
        );
        let screen = new_load_screen(e, this);
        add_load_screen(e, this, LIST_PUSH, screen);
    }
    if cleared {
        let model_tile = tile(e, this, SLOT_MODEL);
        if !fn_0078a1f0(e, model_tile) {
            fn_0078a3e0(e, this, 0);
            fn_0078abd0(e, this);
        }
    }
    set_flag(e, this, 7, 1);
    let first = node_item(e, load_screens(this));
    e.set(this, LoadingMenu::pCurrentLoadScreen, Ptr::new(first));
    e.call(CALC_FACES, &args![this]);
    let current = e.get(this, LoadingMenu::pCurrentLoadScreen).addr();
    fn_0078b3f0(e, this, current, 1);
}

// Translated from 0078a910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Post-increment of the word at `counter` (`cdecl`, two words, the second
/// not used): returns the old value.
pub fn fn_0078a910(e: &mut Engine, counter: u32, _unused_1: u32) -> u32 {
    let old = e.mem.u32(counter);
    e.mem.set_u32(counter, old.wrapping_add(1));
    old
}

// Translated from 0078a930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `0x011cabb8`.
pub fn fn_0078a930(e: &mut Engine) -> bool {
    e.mem.u8(LIST_LOCKED) != 0
}

// Translated from 0078a940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the temporary load screens (flag `0x4000`, `004077c0`) of
/// `LoadScreens` through their virtual slot `0x10` (argument 1), frees the
/// list nodes and resets `pNextLoadScreen` to the list head.
pub fn fn_0078a940(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let mut node = load_screens(this);
    while node != 0 {
        let screen = node_item(e, node);
        if screen == 0 {
            break;
        }
        if e.call(IS_TEMPORARY, &args![screen]).bool() {
            let screen = node_item(e, node);
            if screen != 0 {
                e.vcall(screen, 0x10, &args![1u32]);
            }
        }
        node = next_node(e, node);
    }
    e.call(LIST_FREE_NODES, &args![load_screens(this)]);
    e.set(
        this,
        LoadingMenu::pNextLoadScreen,
        Ptr::new(load_screens(this)),
    );
}

// Translated from 0078a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances to the next load screen: the current one becomes the item of
/// `pNextLoadScreen`, which moves on (wrapping to the list head) and, when
/// its item is not null, `0078bb60` gets the former current screen.
pub fn fn_0078a9f0(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
    let screen = node_item(e, next);
    e.set(this, LoadingMenu::pCurrentLoadScreen, Ptr::new(screen));
    let following = next_node(e, next);
    e.set(this, LoadingMenu::pNextLoadScreen, Ptr::new(following));
    if following == 0 {
        e.set(
            this,
            LoadingMenu::pNextLoadScreen,
            Ptr::new(load_screens(this)),
        );
    }
    let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
    if next != 0 {
        let item = node_item(e, next);
        if item != 0 {
            let current = e.get(this, LoadingMenu::pCurrentLoadScreen).addr();
            e.call(ANNOUNCE_SCREEN, &args![this, current]);
        }
    }
}

// Translated from 0078aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the initial load with state bit 0 clear and the model tile's sequence
/// not running: moves `pNextLoadScreen` to the last node, shows the screen
/// (`0078a3e0`), sets the swap time to now and returns true; false
/// otherwise.
pub fn fn_0078aa90(e: &mut Engine, this: Ptr<LoadingMenu>) -> bool {
    let model_tile = tile(e, this, SLOT_MODEL);
    if e.mem.u8(INITIAL_LOAD) != 0 && !flag(e, this, 0) && !fn_0078a1f0(e, model_tile) {
        loop {
            let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
            if next_node(e, next) == 0 {
                break;
            }
            let following = next_node(e, next);
            e.set(this, LoadingMenu::pNextLoadScreen, Ptr::new(following));
        }
        fn_0078a3e0(e, this, 0);
        let time = now(e);
        e.set(this, LoadingMenu::uiNextBackgroundSwap, time);
        return true;
    }
    false
}

// Translated from 0078ab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `screen` is a form of type `0x4b` whose name differs from the
/// name of every screen already in `LoadScreens` (the walk stops at the
/// first equal name; `00408b20` is the name comparison, nonzero meaning
/// different).
pub fn fn_0078ab20(e: &mut Engine, this: Ptr<LoadingMenu>, screen: u32) -> bool {
    let mut result = screen != 0 && e.call(FORM_TYPE, &args![screen]).u32() == 0x4b;
    let mut node = load_screens(this);
    while result && node != 0 {
        let item = node_item(e, node);
        if item == 0 {
            break;
        }
        let listed = e.call(FORM_NAME, &args![item + 0x18]).u32();
        let wanted = e.call(FORM_NAME, &args![screen + 0x18]).u32();
        result = e.call(NAME_COMPARE, &args![wanted, listed]).u32() != 0;
        node = next_node(e, node);
    }
    result
}

// Translated from 0078abd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the next swap time to now plus an interval from the settings
/// (`0x011da190` while an XUI is up, `0x011da158` on the initial load, else
/// `0x011da1a0`), in seconds times 1000 truncated.
pub fn fn_0078abd0(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let interval = if xui_is_up(e) {
        setting_float(e, SETTING_INTERVAL_XUI)
    } else if e.mem.u8(INITIAL_LOAD) != 0 {
        setting_float(e, SETTING_INTERVAL_INITIAL)
    } else {
        setting_float(e, SETTING_INTERVAL_DEFAULT)
    };
    let time = now(e);
    let milliseconds = (interval as f64 * e.global::<f64>(MS_PER_SECOND)) as i64;
    e.set(
        this,
        LoadingMenu::uiNextBackgroundSwap,
        time.wrapping_add(milliseconds as u32),
    );
}

// Translated from 0078ac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::BuildCandidateList` (Xbox PDB): resets `list` and adds
/// every load screen of the global list that suits the location (overlay
/// mode: screens with flag `0x400`; else those matching `pLocation`, or the
/// generic ones without a location). While an overlay or `0x011d8907` is
/// active, an added screen whose type is neither 0 nor 3 is removed again.
pub fn loading_menu_build_candidate_list(e: &mut Engine, this: Ptr<LoadingMenu>, list: u32) {
    e.call(CANDIDATES_RESET, &args![list, 0u32]);
    let owner = e.mem.u32(LOAD_SCREEN_OWNER);
    let mut node = e.call(ALL_SCREENS_LIST, &args![owner]).u32();
    while node != 0 {
        let screen = node_item(e, node);
        if screen == 0 {
            break;
        }
        let location = e.get(this, LoadingMenu::pLocation).addr();
        let suits = if flag(e, this, 5) {
            fn_0078ada0(e, screen)
        } else if location != 0 {
            e.call(LOAD_SCREEN_FOR_LOCATION, &args![screen, location])
                .bool()
        } else {
            e.call(LOAD_SCREEN_GENERIC, &args![screen]).bool()
        };
        if suits {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let index = e.call(CANDIDATE_ADD, &args![list, slot]).u32();
            let kind = e.call(LOAD_SCREEN_TYPE, &args![screen]).u32();
            if (e.mem.u8(MENU_REPLACED) != 0 || overlay_active(e)) && kind != 3 && kind != 0 {
                e.call(CANDIDATE_REMOVE, &args![list, index, 1u32]);
            }
        }
        node = next_node(e, node);
    }
}

// Translated from 0078ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x400` of the load screen's flags word (`+8`) is set.
pub fn fn_0078ada0(e: &mut Engine, screen: u32) -> bool {
    e.call(WORD_AT_8, &args![screen]).u32() & 0x400 != 0
}

// Translated from 0078adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves random candidates from `list` (the size is its word at `+8`) into
/// `LoadScreens` while there are fewer than the limit (the screen count
/// setting; the overlay setting in overlay mode; with a location and no list
/// lock, the smaller of it and the per-location setting). Each candidate is
/// removed from `list` whether it was taken or not; `0078ab20` decides.
pub fn fn_0078adc0(e: &mut Engine, this: Ptr<LoadingMenu>, list: u32) {
    let mut limit = setting_int(e, SETTING_MAX_SCREENS);
    if flag(e, this, 5) {
        limit = setting_int(e, SETTING_MAX_SCREENS_OVERLAY);
    } else if e.get(this, LoadingMenu::pLocation).addr() != 0 && !fn_0078a930(e) {
        let per_location = setting_int(e, SETTING_LOCATION_SCREENS);
        limit = e.call(MINIMUM, &args![limit, per_location]).u32();
    }
    let mut count = e.call(LIST_COUNT, &args![load_screens(this)]).u32();
    while count < limit {
        if e.call(WORD_AT_8, &args![list]).u32() == 0 {
            break;
        }
        let size = e.call(WORD_AT_8, &args![list]).u32();
        let index = e.call(RANDOM_INDEX, &args![size]).u32();
        let slot = e.call(CANDIDATE_AT, &args![list, index]).u32();
        let screen = e.mem.u32(slot);
        if fn_0078ab20(e, this, screen) {
            let slot = e.call(CANDIDATE_AT, &args![list, index]).u32();
            e.call(LIST_ADD, &args![load_screens(this), slot]);
            count += 1;
        }
        e.call(CANDIDATE_REMOVE, &args![list, index, 1u32]);
    }
}

// Translated from 0078aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the status tiles: refreshes the stats (`0078d670`) unless the skip
/// test passes for `0x011ddf38`, then sets the scale/value of the `+0x50`
/// tile from `0x011da16c`/`0x011da170`, the `+0x34` tile text from the
/// `0x011da168` object's name (or a setting) and the `+0x60`/`+0x64` texts
/// from settings.
pub fn fn_0078aec0(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let owner = e.mem.u32(SKIP_OWNER);
    if owner == 0 || !e.call(SKIP_TEST, &args![owner]).bool() {
        e.call(REFRESH_STATS, &args![]);
    }
    let scaled = tile(e, this, SLOT_SCALED);
    let scale: f32 = e.global(TIP_SCALE);
    set_tile_value(e, scaled, 0x1004, scale, 1);
    let value = e.mem.u32(TIP_VALUE);
    e.call(TILE_SET_ID_VALUE, &args![scaled, 0x1005u32, value]);
    let text_owner = e.mem.u32(TIP_TEXT_OWNER);
    let text = if text_owner != 0 {
        e.call(TEXT_OWNER_NAME, &args![text_owner]).u32()
    } else {
        setting_string(e, SETTING_TIP_FALLBACK)
    };
    let status = tile(e, this, SLOT_STATUS_TEXT);
    set_tile_string(e, status, 0xfc4, text, 1);
    let text = setting_string(e, SETTING_TEXT_60);
    let label = tile(e, this, SLOT_TEXT_60);
    set_tile_string(e, label, 0xfc4, text, 1);
    let text = setting_string(e, SETTING_TEXT_64);
    let label = tile(e, this, SLOT_TEXT_64);
    set_tile_string(e, label, 0xfc4, text, 1);
}

// Translated from 0078afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears state bit 6 (tips enabled) from `value`.
pub fn fn_0078afa0(e: &mut Engine, this: Ptr<LoadingMenu>, value: u8) {
    set_flag(e, this, 6, value);
}

/// The load screen `item`, or when null the current next screen's (else the
/// first of the list); null when there is none.
fn screen_or_default(e: &mut Engine, this: Ptr<LoadingMenu>, item: u32) -> u32 {
    if item != 0 {
        return item;
    }
    let next = e.get(this, LoadingMenu::pNextLoadScreen).addr();
    if next != 0 {
        node_item(e, next)
    } else {
        node_item(e, load_screens(this))
    }
}

// Translated from 0078afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With tips enabled (bit 6), takes `item` (default: the next screen, else
/// the first) and, when `005f36f0` says it is present, passes `value` and
/// `mode` to the tile-list routine for the screen's type: 1 `0078b360`,
/// 2 `0078b190`, 3 `0078b220`, 4 `0078b100`.
pub fn fn_0078afc0(e: &mut Engine, this: Ptr<LoadingMenu>, item: u32, value: f32, mode: i32) {
    if !flag(e, this, 6) {
        return;
    }
    let item = screen_or_default(e, this, item);
    if item == 0 {
        return;
    }
    if e.call(LOAD_SCREEN_TYPE_FOLDED, &args![item]).u32() == 0 {
        return;
    }
    match e.call(LOAD_SCREEN_TYPE, &args![item]).u32() {
        1 => fn_0078b360(e, this, value, mode),
        2 => fn_0078b190(e, this, value, mode),
        3 => fn_0078b220(e, this, value, mode),
        4 => fn_0078b100(e, this, value, mode),
        _ => {}
    }
}

/// Walks the `NiTPointerList` at offset `list` of the menu; for each
/// non-null tile sets its `0xfa3` value to 1 when it is 0, then calls
/// `0078ce10(this, tile, value)`.
fn fade_tile_list(e: &mut Engine, this: Ptr<LoadingMenu>, list: u32, value: f32) {
    let list = this.addr() + list;
    e.with_stack(4, |e, iterator| {
        let head = e.call(NI_POINTER_GET, &args![list]).u32();
        e.mem.set_u32(iterator.addr(), head);
        while e.mem.u32(iterator.addr()) != 0 {
            let element = e.call(POINTER_LIST_STEP, &args![list, iterator]).u32();
            let tile = e.mem.u32(element);
            if tile != 0 {
                if tile_value(e, tile, 0xfa3) as f64 == e.global::<f64>(ZERO) {
                    set_tile_int(e, tile, 0xfa3, 1);
                }
                e.call(SET_TILE_FADE, &args![this, tile, value]);
            }
        }
    });
}

// Translated from 0078b100 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `mode` 1: fades the tiles of `StatsTilesList` (`fade_tile_list`).
pub fn fn_0078b100(e: &mut Engine, this: Ptr<LoadingMenu>, value: f32, mode: i32) {
    if mode == 1 {
        fade_tile_list(e, this, LoadingMenu::StatsTilesList.off, value);
    }
}

// Translated from 0078b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `mode` 1: fades the tiles of `ObjectiveTilesList`.
pub fn fn_0078b190(e: &mut Engine, this: Ptr<LoadingMenu>, value: f32, mode: i32) {
    if mode == 1 {
        fade_tile_list(e, this, LoadingMenu::ObjectiveTilesList.off, value);
    }
}

// Translated from 0078b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `mode` 1: fades the tiles of `TipTilesList` and then the tile at
/// `+0xb8`; with `mode` 2 the tiles of `TipTilesList02` and then `+0xbc`.
pub fn fn_0078b220(e: &mut Engine, this: Ptr<LoadingMenu>, value: f32, mode: i32) {
    if mode == 1 {
        fade_tile_list(e, this, LoadingMenu::TipTilesList.off, value);
        let matte = tile(e, this, SLOT_MATTE_A);
        e.call(SET_TILE_FADE, &args![this, matte, value]);
    }
    if mode == 2 {
        fade_tile_list(e, this, LoadingMenu::TipTilesList02.off, value);
        let matte = tile(e, this, SLOT_MATTE_B);
        e.call(SET_TILE_FADE, &args![this, matte, value]);
    }
}

// Translated from 0078b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `mode` 1: fades the tiles of `XPProgressTilesList`.
pub fn fn_0078b360(e: &mut Engine, this: Ptr<LoadingMenu>, value: f32, mode: i32) {
    if mode == 1 {
        fade_tile_list(e, this, LoadingMenu::XPProgressTilesList.off, value);
    }
}

// Translated from 0078b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With tips enabled (bit 6): shows (`show` nonzero) the tile groups of
/// the load screen `item` (default as in `0078afc0`) by calling the four
/// group routines `0078b6a0`, `0078b650`, `0078b6f0` and `0078b7c0`: the
/// group of the screen's type gets `show`, the others the opposite (screen
/// type 2: `b6a0`, 4: `b650`, 3: `b6f0`, 1: `b7c0`; any other type hides
/// all). With `show` 0 all four get `show`.
pub fn fn_0078b3f0(e: &mut Engine, this: Ptr<LoadingMenu>, item: u32, show: u8) {
    if !flag(e, this, 6) {
        return;
    }
    let item = screen_or_default(e, this, item);
    if item == 0 {
        return;
    }
    let other = (show == 0) as u8;
    let groups = if show == 0 {
        [show; 4]
    } else {
        match e.call(LOAD_SCREEN_TYPE, &args![item]).u32() {
            1 => [other, other, other, show],
            2 => [show, other, other, other],
            3 => [other, other, show, other],
            4 => [other, show, other, other],
            _ => [0; 4],
        }
    };
    e.call(GROUP_VISIBLE_0078B6A0, &args![this, groups[0]]);
    e.call(GROUP_VISIBLE_0078B650, &args![this, groups[1]]);
    e.call(GROUP_VISIBLE_0078B6F0, &args![this, groups[2]]);
    e.call(GROUP_VISIBLE_0078B7C0, &args![this, groups[3]]);
}

// ----- second half: helpers -----

/// `FISTP` with the truncating rounding mode the compiler sets (control word
/// `| 0xc00`) into 32 bits: the integer indefinite `0x80000000` when the
/// value does not fit.
fn truncate_to_i32(value: f64) -> i32 {
    if value.is_nan() || value >= 2147483648.0 || value < -2147483648.0 {
        i32::MIN
    } else {
        value.trunc() as i32
    }
}

/// The same into 64 bits (`FISTP qword`), returning the low word the code
/// keeps: `0` (the low word of the indefinite `0x8000000000000000`) when the
/// value does not fit.
fn truncate_to_i64_low(value: f64) -> u32 {
    if value.is_nan() || value >= 9.223_372_036_854_776e18 || value < -9.223_372_036_854_776e18 {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// Walks the `NiTPointerList` at `list` the way the game's loops do: the
/// head from `NiPointer` getter `00559450`, then the step `0057cbe0`, which
/// returns the address of the element and advances the iterator; `visit` gets
/// the element's word.
fn for_each_in_list(e: &mut Engine, list: u32, mut visit: impl FnMut(&mut Engine, u32)) {
    e.with_stack(4, |e, iterator| {
        let head = e.call(NI_POINTER_GET, &args![list]).u32();
        e.mem.set_u32(iterator.addr(), head);
        while e.mem.u32(iterator.addr()) != 0 {
            let element = e.call(POINTER_LIST_STEP, &args![list, iterator]).u32();
            let item = e.mem.u32(element);
            visit(e, item);
        }
    });
}

/// Sets value `0xfa3` of every tile of the list at offset `list` of the menu
/// to `value` (the walkers behind `0078b650`, `0078b6a0`, `0078b7c0`). The
/// game does not test the tiles for null here.
fn set_list_value(e: &mut Engine, this: Ptr<LoadingMenu>, list: u32, value: u8) {
    for_each_in_list(e, this.addr() + list, |e, item| {
        set_tile_int(e, item, 0xfa3, value as u32);
    });
}

/// `Tile::AddNeedsUpdate(8)` on `tile`.
fn add_needs_update(e: &mut Engine, tile: u32) {
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

/// The data block word at `offset` (`LoadScreenDefaultProperties`-like block
/// of a load screen type, `0078bc60`).
fn data_word(e: &Engine, data: u32, offset: u32) -> u32 {
    e.mem.u32(data + offset)
}

/// The `float` of the data block at `offset`.
fn data_float(e: &Engine, data: u32, offset: u32) -> f32 {
    e.mem.f32(data + offset)
}

/// Sets the three `float` values `0xfb2`, `0xfb3`, `0xfb4` of `tile` from the
/// data block words at `+0x1c`, `+0x20`, `+0x24`.
fn set_block_values(e: &mut Engine, tile: u32, data: u32) {
    for (id, offset) in [(0xfb2u32, 0x1cu32), (0xfb3, 0x20), (0xfb4, 0x24)] {
        let value = data_float(e, data, offset);
        set_tile_value(e, tile, id, value, 1);
    }
}

/// `left`, `top`, `right`, `bottom` of the text window rectangle.
fn face_rect(e: &mut Engine, this: Ptr<LoadingMenu>) -> (i32, i32, i32, i32) {
    (
        e.get(this, LoadingMenu::faceLeft),
        e.get(this, LoadingMenu::faceTop),
        e.get(this, LoadingMenu::faceRight),
        e.get(this, LoadingMenu::faceBottom),
    )
}

/// The size `FontManager::CalculateStringDimensions` (`00a1b020`) writes into
/// a 12-byte block for `text` in the font `font`, wrapped at `width`; the
/// font manager is the global one (`005bd5b0`). Returns the three `float`s.
fn string_dimensions(e: &mut Engine, text: u32, font: u32, width: f32) -> [f32; 3] {
    e.with_stack(12, |e, out| {
        let manager = e.call(FONT_MANAGER, &args![]).u32();
        e.call(
            STRING_DIMENSIONS,
            &args![manager, out, text, font, width, 0u32],
        );
        [
            e.mem.f32(out.addr()),
            e.mem.f32(out.addr() + 4),
            e.mem.f32(out.addr() + 8),
        ]
    })
}

// Translated from 0078b650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets value `0xfa3` of every tile of `StatsTilesList` to `value`.
pub fn fn_0078b650(e: &mut Engine, this: Ptr<LoadingMenu>, value: u8) {
    set_list_value(e, this, LoadingMenu::StatsTilesList.off, value);
}

// Translated from 0078b6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets value `0xfa3` of every tile of `ObjectiveTilesList` to `value`.
pub fn fn_0078b6a0(e: &mut Engine, this: Ptr<LoadingMenu>, value: u8) {
    set_list_value(e, this, LoadingMenu::ObjectiveTilesList.off, value);
}

// Translated from 0078b6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets value `0xfa3` of every non-null tile of `TipTilesList` and
/// `TipTilesList02` to `value`, then of the two matte tiles (`+0xb8`,
/// `+0xbc`, not tested for null).
pub fn fn_0078b6f0(e: &mut Engine, this: Ptr<LoadingMenu>, value: u8) {
    for list in [
        LoadingMenu::TipTilesList.off,
        LoadingMenu::TipTilesList02.off,
    ] {
        for_each_in_list(e, this.addr() + list, |e, item| {
            if item != 0 {
                set_tile_int(e, item, 0xfa3, value as u32);
            }
        });
    }
    let matte_a = tile(e, this, SLOT_MATTE_A);
    set_tile_int(e, matte_a, 0xfa3, value as u32);
    let matte_b = tile(e, this, SLOT_MATTE_B);
    set_tile_int(e, matte_b, 0xfa3, value as u32);
}

// Translated from 0078b7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets value `0xfa3` of every tile of `XPProgressTilesList` to `value`.
pub fn fn_0078b7c0(e: &mut Engine, this: Ptr<LoadingMenu>, value: u8) {
    set_list_value(e, this, LoadingMenu::XPProgressTilesList.off, value);
}

// Translated from 0078b810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records the placement every attached tile has in the menu file into
/// `DefaultTileSettings` (one 0x18-byte record per slot): the truncated
/// values `0xfa1`, `0xfa2`, `0xfb0`, `0xfb1`, `0xfb9` (the last of them as
/// the word at `+0x10`), and the low byte of the truncated value `0xfb7`.
/// An empty slot gets `0, 0, 0, 0, 1` and the byte `4`.
pub fn fn_0078b810(e: &mut Engine, this: Ptr<LoadingMenu>) {
    for index in 0..TILE_COUNT as u32 {
        let (mut origin_x, mut origin_y, mut height, mut width) = (0u32, 0u32, 0u32, 0u32);
        let (mut font, mut justification) = (1u32, 4u8);
        let slot = tile(e, this, TILES + index * 4);
        if slot != 0 {
            let value = tile_value(e, slot, 0xfa1);
            origin_x = float_to_int(e, value as f64);
            let value = tile_value(e, slot, 0xfa2);
            origin_y = float_to_int(e, value as f64);
            let value = tile_value(e, slot, 0xfb0);
            height = float_to_int(e, value as f64);
            let value = tile_value(e, slot, 0xfb1);
            width = float_to_int(e, value as f64);
            let value = tile_value(e, slot, 0xfb9);
            font = float_to_int(e, value as f64);
            let value = tile_value(e, slot, 0xfb7);
            justification = truncate_to_i32(value as f64) as u8;
        }
        let record = this.addr() + DEFAULT_TILE_SETTINGS + index * DEFAULT_TILE_SETTINGS_STRIDE;
        e.mem.set_u32(record, origin_x);
        e.mem.set_u32(record + 4, origin_y);
        e.mem.set_u32(record + 8, height);
        e.mem.set_u32(record + 12, width);
        e.mem.set_u32(record + 16, font);
        e.mem.set_u8(record + 20, justification);
    }
}

// Translated from 0078b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::CalcLoadScreenTypeTextWindowFaces` (Xbox PDB): converts the
/// text window of a load screen type (`data`: a flag word, then left, top,
/// width and height in the 1280 x 960 design space) into the screen
/// rectangle `rLoadScreenTypeTextWindowFaces`, clamped to the screen less
/// its insets. Logs a warning for null `data`. Nothing happens when the
/// flag word is 0. (The height is divided by the 1280 constant too, as the
/// code does.)
pub fn loading_menu_calc_load_screen_type_text_window_faces(
    e: &mut Engine,
    this: Ptr<LoadingMenu>,
    data: u32,
) {
    if data == 0 {
        e.call(ERROR_LOG, &args![NULL_DATA_WARNING]);
        return;
    }
    let screen_width = e.call(SCREEN_WIDTH, &args![]).f32();
    let screen_height = e.call(SCREEN_HEIGHT, &args![]).f32();
    let inset = e.call(INSET_X, &args![]).f32();
    let max_right = (screen_width as f64 - inset as f64) as f32;
    let inset = e.call(INSET_Y, &args![]).f32();
    let max_bottom = (screen_height as f64 - inset as f64) as f32;
    let design_width: f64 = e.global(DESIGN_WIDTH);
    let design_height: f64 = e.global(DESIGN_HEIGHT);
    let left = (data_word(e, data, 4) as f64 / design_width * screen_width as f64) as f32;
    let top = (data_word(e, data, 8) as f64 / design_height * screen_height as f64) as f32;
    let width = (data_word(e, data, 12) as f64 / design_width * screen_width as f64) as f32;
    let height = (data_word(e, data, 16) as f64 / design_width * screen_height as f64) as f32;
    if data_word(e, data, 0) == 0 {
        return;
    }
    let inset = e.call(INSET_X, &args![]).f32();
    let value = e.call(FLOAT_MAX, &args![left, inset]).f32();
    let result = float_to_int(e, value as f64);
    e.set(this, LoadingMenu::faceLeft, result as i32);
    let inset = e.call(INSET_Y, &args![]).f32();
    let value = e.call(FLOAT_MAX, &args![top, inset]).f32();
    let result = float_to_int(e, value as f64);
    e.set(this, LoadingMenu::faceTop, result as i32);
    let sum = (e.get(this, LoadingMenu::faceLeft) as f64 + width as f64) as f32;
    let value = e.call(FLOAT_MIN, &args![sum, max_right]).f32();
    let result = float_to_int(e, value as f64);
    e.set(this, LoadingMenu::faceRight, result as i32);
    let sum = (e.get(this, LoadingMenu::faceTop) as f64 + height as f64) as f32;
    let value = e.call(FLOAT_MIN, &args![sum, max_bottom]).f32();
    let result = float_to_int(e, value as f64);
    e.set(this, LoadingMenu::faceBottom, result as i32);
}

// Translated from 0078bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Places the tiles of the current load screen (`pCurrentLoadScreen`):
/// `0078bb60` with it.
pub fn fn_0078bb40(e: &mut Engine, this: Ptr<LoadingMenu>) {
    let current = e.get(this, LoadingMenu::pCurrentLoadScreen).addr();
    fn_0078bb60(e, this, current);
}

// Translated from 0078bb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// With tips enabled (bit 6): takes `item` (default as in `0078afc0`: the
/// next screen, else the first; nothing when there is none) and, when
/// `005f36f0` says the screen has a type block, computes the text window
/// rectangle from the block (`0078bc60`, `CalcLoadScreenTypeTextWindowFaces`)
/// and lays out the tiles of the screen's type: 1 `0078c210`, 2 `0078bf80`,
/// 3 `0078c770`, 4 `InitStatsTilesSettings`. Clears the byte after
/// `bDoTileUpdate` last.
pub fn fn_0078bb60(e: &mut Engine, this: Ptr<LoadingMenu>, item: u32) {
    if !flag(e, this, 6) {
        return;
    }
    let item = screen_or_default(e, this, item);
    if item == 0 {
        return;
    }
    if e.call(LOAD_SCREEN_TYPE_FOLDED, &args![item]).u32() != 0 {
        let data = fn_0078bc60(e, item);
        loading_menu_calc_load_screen_type_text_window_faces(e, this, data);
        match e.call(LOAD_SCREEN_TYPE, &args![item]).u32() {
            1 => fn_0078c210(e, this, data),
            2 => fn_0078bf80(e, this, data),
            3 => fn_0078c770(e, this, data),
            4 => loading_menu_init_stats_tiles_settings(e, this, data),
            _ => {}
        }
    }
    e.mem.set_u8(LAYOUT_FLAG_011A0296, 0);
}

// Translated from 0078bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The block of tile settings of a load screen: the word at `+0x34` of the
/// load screen plus `0x18`.
pub fn fn_0078bc60(e: &mut Engine, screen: u32) -> u32 {
    e.mem.u32(screen + 0x34).wrapping_add(0x18)
}

// Translated from 0078bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::InitStatsTilesSettings` (Xbox PDB): lays out the stats
/// screen from the settings block `data`. Places the stats area tile
/// (`+0xa4`) at the text window rectangle, sets the font, values and colour
/// of the two column tiles (`+0x6c`, `+0x70`; the second block of values goes
/// to `+0x6c` again, as the code does), then walks the children of both
/// columns together: while the row number is at most the count in `data+0x54`
/// the two children are put at the running y position (the area height
/// divided by the count per row) and shown, the others are hidden. Ends by
/// setting `0xffa` of the area tile and updating its rotation.
pub fn loading_menu_init_stats_tiles_settings(e: &mut Engine, this: Ptr<LoadingMenu>, data: u32) {
    let (left, top, right, bottom) = face_rect(e, this);
    let area = tile(e, this, SLOT_STATS_AREA);
    set_tile_value(e, area, 0xfa1, left as f32, 1);
    set_tile_value(e, area, 0xfb1, right.wrapping_sub(left) as f32, 1);
    set_tile_value(e, area, 0xfa2, top as f32, 1);
    set_tile_value(e, area, 0xfb0, bottom.wrapping_sub(top) as f32, 1);
    let column_a = tile(e, this, SLOT_STATS_COLUMN_A);
    let column_b = tile(e, this, SLOT_STATS_COLUMN_B);
    set_tile_int(e, column_a, 0xfb9, data_word(e, data, 0x18));
    set_block_values(e, column_a, data);
    set_tile_int(e, column_b, 0xfb9, data_word(e, data, 0x40));
    for (id, offset) in [(0xfb2u32, 0x44u32), (0xfb3, 0x48), (0xfb4, 0x4c)] {
        let value = data_float(e, data, offset);
        set_tile_value(e, column_a, id, value, 1);
    }
    let area_height = tile_value(e, area, 0xfb0);
    let count = data_word(e, data, 0x54);
    let step = (area_height as f64 / count as f64) as f32;
    let mut y = 0.0f32;
    let mut row = 1u32;
    e.with_stack(8, |e, iterators| {
        let first_a = e.call(NODE_NEXT, &args![column_a + 4]).u32();
        let first_b = e.call(NODE_NEXT, &args![column_b + 4]).u32();
        e.mem.set_u32(iterators.addr(), first_a);
        e.mem.set_u32(iterators.addr() + 4, first_b);
        while e.mem.u32(iterators.addr()) != 0 && e.mem.u32(iterators.addr() + 4) != 0 {
            let item = e
                .call(CHILD_STEP, &args![column_a + 4, iterators.addr()])
                .u32();
            let child_a = e.mem.u32(item);
            let item = e
                .call(CHILD_STEP, &args![column_b + 4, iterators.addr() + 4])
                .u32();
            let child_b = e.mem.u32(item);
            if child_a != 0 && child_b != 0 {
                let shown = row <= count;
                if shown {
                    set_tile_value(e, child_a, 0xfa2, y, 1);
                    set_tile_value(e, child_b, 0xfa2, y, 1);
                    y = (y as f64 + step as f64) as f32;
                    add_needs_update(e, child_a);
                    add_needs_update(e, child_b);
                    row += 1;
                }
                set_tile_int(e, child_a, 0xfa3, shown as u32);
                set_tile_int(e, child_b, 0xfa3, shown as u32);
            }
        }
    });
    let rotation = data_float(e, data, 0x14);
    set_tile_value(e, area, 0xffa, rotation, 1);
    e.call(TILE_ROTATION_UPDATE, &args![area]);
}

// Translated from 0078bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lays out the objective screen from the settings block `data`: for each
/// tile of `ObjectiveTilesList`, the tile at `+0x94` gets the text window's
/// x (by justification `data+0x28`: 1 left, 2 centre, 4 right), width,
/// `0xfba` (its own width), y, font `data+0x18`, justification, the block
/// values and `0xff5 = -1`, and an update; the tile at `+0x34` gets only its
/// y from the height `CalculateStringDimensions` measures for the text of
/// the setting `011d40f0` in that font and the window width. The rotation
/// (`data+0x14`) of the `+0x94` tile is set last.
pub fn fn_0078bf80(e: &mut Engine, this: Ptr<LoadingMenu>, data: u32) {
    // The font object of the type (`005bd5b0`'s table indexed by `data+0x18`)
    // is read for its value `00759450` (`float` at +0x2c); the result is not
    // used afterwards.
    let manager = e.call(FONT_MANAGER, &args![]).u32();
    let font = e
        .mem
        .u32(manager.wrapping_add(data_word(e, data, 0x18).wrapping_mul(4)));
    e.call(FONT_VALUE, &args![font]);
    let objective = tile(e, this, SLOT_OBJECTIVE);
    let status = tile(e, this, SLOT_STATUS_TEXT);
    for_each_in_list(
        e,
        this.addr() + LoadingMenu::ObjectiveTilesList.off,
        |e, item| {
            if item == 0 {
                return;
            }
            let (left, top, right, _) = face_rect(e, this);
            let width = right.wrapping_sub(left);
            if item == objective {
                let mut x = 0.0f32;
                match data_word(e, data, 0x28) {
                    1 => x = left as f32,
                    2 => {
                        let half: f64 = e.global(TWO);
                        x = (left as f64 + width as f64 / half) as f32;
                    }
                    4 => x = right as f32,
                    _ => {}
                }
                set_tile_value(e, item, 0xfa1, x, 1);
                set_tile_value(e, item, 0xfb1, width as f32, 1);
                let own_width = tile_value(e, item, 0xfb1);
                set_tile_value(e, item, 0xfba, own_width, 1);
                set_tile_value(e, item, 0xfa2, top as f32, 1);
                set_tile_int(e, item, 0xfb9, data_word(e, data, 0x18));
                let justification = data_word(e, data, 0x28);
                e.call(TILE_SET_ID_VALUE, &args![item, 0xfb7u32, justification]);
                set_block_values(e, item, data);
                set_tile_int(e, item, 0xff5, 0xffff_ffff);
                add_needs_update(e, item);
            } else if item == status {
                let text = setting_string(e, SETTING_TEXT_60);
                let font = data_word(e, data, 0x18);
                let size = string_dimensions(e, text, font, width as f32);
                set_tile_value(e, item, 0xfa2, size[1], 1);
            }
        },
    );
    let objective = tile(e, this, SLOT_OBJECTIVE);
    let rotation = data_float(e, data, 0x14);
    set_tile_value(e, objective, 0xffa, rotation, 1);
    e.call(TILE_ROTATION_UPDATE, &args![objective]);
}

// Translated from 0078c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lays out the experience screen from the settings block `data`, for each
/// tile of `XPProgressTilesList`: the bar tile (`+0x68`) gets the window's
/// x, width, `0xfba`, y and height from the rectangle (less its own x and y);
/// the text tile (`+0x64`) gets x (by justification `data+0x28`: 1 at 0,
/// 2 half the width, 4 the width), the justification value, the font, the
/// width, `0xfba` and the height of the text of setting `011d2358`; the
/// tile at `+0x50` gets the x of the first number (`0x1005` printed with
/// `%d`) and a width that leaves room for the second (`0x1006`). Every tile
/// but the bar then gets the block values, `0xff5 = -1` and an update. The
/// rotation (`data+0x14`) of the bar tile is set last.
pub fn fn_0078c210(e: &mut Engine, this: Ptr<LoadingMenu>, data: u32) {
    let bar = tile(e, this, SLOT_XP_BAR);
    let text_tile = tile(e, this, SLOT_XP_TEXT);
    let numbers = tile(e, this, SLOT_SCALED);
    for_each_in_list(
        e,
        this.addr() + LoadingMenu::XPProgressTilesList.off,
        |e, item| {
            if item == 0 {
                return;
            }
            let (left, top, right, bottom) = face_rect(e, this);
            let width = right.wrapping_sub(left);
            if item == bar {
                set_tile_value(e, item, 0xfa1, left as f32, 1);
                let x = tile_value(e, item, 0xfa1);
                set_tile_value(e, item, 0xfb1, (right as f64 - x as f64) as f32, 1);
                let own_width = tile_value(e, item, 0xfb1);
                set_tile_value(e, item, 0xfba, own_width, 1);
                set_tile_value(e, item, 0xfa2, top as f32, 1);
                let y = tile_value(e, item, 0xfa2);
                set_tile_value(e, item, 0xfb0, (bottom as f64 - y as f64) as f32, 1);
                return;
            }
            if item == text_tile {
                let mut x = 0.0f32;
                match data_word(e, data, 0x28) {
                    1 => x = 0.0,
                    2 => {
                        let half: f64 = e.global(TWO);
                        x = (width as f64 / half) as f32;
                    }
                    4 => x = width as f32,
                    _ => {}
                }
                set_tile_value(e, item, 0xfa1, x, 1);
                let justification = data_word(e, data, 0x28);
                e.call(TILE_SET_ID_VALUE, &args![item, 0xfb7u32, justification]);
                set_tile_int(e, item, 0xfb9, data_word(e, data, 0x18));
                set_tile_value(e, item, 0xfb1, width as f32, 1);
                let own_width = tile_value(e, item, 0xfb1);
                set_tile_value(e, item, 0xfba, own_width, 1);
                let wrap = tile_value(e, item, 0xfba);
                let text = setting_string(e, SETTING_TEXT_64);
                let font = data_word(e, data, 0x18);
                let size = string_dimensions(e, text, font, wrap);
                set_tile_value(e, item, 0xfb0, size[1], 1);
            } else if item == numbers {
                let first = tile_value(e, item, 0x1005);
                let first_number = truncate_to_i64_low(first as f64);
                let first_width = e.with_stack(0x104, |e, buffer| {
                    e.call(SNPRINTF, &args![buffer, 0x104u32, INT_FORMAT, first_number]);
                    let wrap = tile_value(e, item, 0xfba);
                    let font = data_word(e, data, 0x18);
                    string_dimensions(e, buffer.addr(), font, wrap)[0]
                });
                set_tile_value(e, item, 0xfa1, first_width, 1);
                let second = tile_value(e, item, 0x1006);
                let second_number = truncate_to_i64_low(second as f64);
                let second_width = e.with_stack(0x104, |e, buffer| {
                    e.call(
                        SNPRINTF,
                        &args![buffer, 0x104u32, INT_FORMAT, second_number],
                    );
                    let half: f64 = e.global(TWO);
                    let wrap = ((width as f64 - first_width as f64) / half) as f32;
                    let font = data_word(e, data, 0x18);
                    string_dimensions(e, buffer.addr(), font, wrap)[0]
                });
                let remaining = ((width as f64 - (first_width as f64 + first_width as f64))
                    - second_width as f64) as f32;
                set_tile_value(e, item, 0xfb1, remaining, 1);
            }
            set_block_values(e, item, data);
            set_tile_int(e, item, 0xff5, 0xffff_ffff);
            add_needs_update(e, item);
        },
    );
    let bar = tile(e, this, SLOT_XP_BAR);
    let rotation = data_float(e, data, 0x14);
    set_tile_value(e, bar, 0xffa, rotation, 1);
    e.call(TILE_ROTATION_UPDATE, &args![bar]);
}

/// Puts one tip tile at the text window: x by justification `data+0x28`
/// (1 left, 2 centre, 4 right), the window width and height, y, the rotation
/// (`data+0x14`), the font, the justification value, the block values,
/// `0xff5 = -1` and an update.
fn place_tip_tile(e: &mut Engine, this: Ptr<LoadingMenu>, data: u32, item: u32) {
    let (left, top, right, bottom) = face_rect(e, this);
    let mut x = 0.0f32;
    match data_word(e, data, 0x28) {
        1 => x = left as f32,
        2 => {
            let half: f64 = e.global(TWO);
            x = (left as f64 + right.wrapping_sub(left) as f64 / half) as f32;
        }
        4 => x = right as f32,
        _ => {}
    }
    set_tile_value(e, item, 0xfa1, x, 1);
    set_tile_value(e, item, 0xfb1, right.wrapping_sub(left) as f32, 1);
    let width = tile_value(e, item, 0xfb1);
    set_tile_value(e, item, 0xfba, width, 1);
    set_tile_value(e, item, 0xfa2, top as f32, 1);
    let y = tile_value(e, item, 0xfa2);
    set_tile_value(e, item, 0xfb0, (bottom as f64 - y as f64) as f32, 1);
    let height = tile_value(e, item, 0xfb0);
    set_tile_value(e, item, 0xfbb, height, 1);
    let rotation = data_float(e, data, 0x14);
    set_tile_value(e, item, 0xffa, rotation, 1);
    set_tile_int(e, item, 0xfb9, data_word(e, data, 0x18));
    let justification = data_word(e, data, 0x28);
    e.call(TILE_SET_ID_VALUE, &args![item, 0xfb7u32, justification]);
    set_block_values(e, item, data);
    set_tile_int(e, item, 0xff5, 0xffff_ffff);
    add_needs_update(e, item);
}

// Translated from 0078c770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lays out the tip screen from the settings block `data`: every non-null
/// tile of `TipTilesList` and then of `TipTilesList02` is placed at the text
/// window (`place_tip_tile`).
pub fn fn_0078c770(e: &mut Engine, this: Ptr<LoadingMenu>, data: u32) {
    for list in [
        LoadingMenu::TipTilesList.off,
        LoadingMenu::TipTilesList02.off,
    ] {
        for_each_in_list(e, this.addr() + list, |e, item| {
            if item != 0 {
                place_tip_tile(e, this, data, item);
            }
        });
    }
}

// Translated from 0078cbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::HideAllBut` (Xbox PDB): sets value `0xfa3` of every attached
/// tile slot to 1 for the slot `selected` (and, for `selected` 0, also
/// slots `0x20` and `0x21`) and to 0 for the others (without the update
/// flag, and without testing the slots for null), then sets state bit 6 to
/// `value`.
pub fn loading_menu_hide_all_but(e: &mut Engine, this: Ptr<LoadingMenu>, selected: i32, value: u8) {
    for index in 0..TILE_COUNT {
        let shown = index == selected || (selected == 0 && (index == 0x20 || index == 0x21));
        let slot = tile(e, this, TILES + index as u32 * 4);
        set_tile_value(e, slot, 0xfa3, shown as u32 as f32, 0);
    }
    set_flag(e, this, 6, value);
}

// Translated from 0078cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// One step of the fade-in of the menu tile. The first call computes the fade
/// duration (value `0xfc1` of the menu tile times 1000) into a function-local
/// `static float`. While `uiFadeInComplete` is 0 it is set: when the rendered
/// texture exists or this is the initial load, the texture is given to the
/// fade image tile and the end is now plus the duration; else the end is now.
/// Before the end the menu tile is faded (`0078ce10`) by 1 - remaining /
/// duration; at the end it is set to 1, state bit 4 is cleared and
/// `m_SlideCount` is incremented.
pub fn fn_0078cc50(e: &mut Engine, this: Ptr<LoadingMenu>) {
    if e.global::<u32>(FADE_DURATION_GUARD) & 1 == 0 {
        let guard: u32 = e.global(FADE_DURATION_GUARD);
        e.set_global(FADE_DURATION_GUARD, guard | 1);
        let menu_tile = e.get(this, LoadingMenu::pMenu).addr();
        let seconds = tile_value(e, menu_tile, 0xfc1);
        let thousand: f64 = e.global(MS_PER_SECOND);
        e.set_global(FADE_DURATION, (seconds as f64 * thousand) as f32);
    }
    if e.get(this, LoadingMenu::uiFadeInComplete) == 0 {
        let owner = e.mem.u32(OVERLAY_OWNER);
        let texture = fn_0078cdf0(e, owner);
        if texture != 0 || e.mem.u8(INITIAL_LOAD) != 0 {
            let owner = e.mem.u32(OVERLAY_OWNER);
            let texture = fn_0078cdf0(e, owner);
            let image = tile(e, this, SLOT_FADE_IMAGE);
            e.call(IMAGE_SET_SOURCE_TEXTURE, &args![image, texture]);
            let ticks = now(e);
            let duration: f32 = e.global(FADE_DURATION);
            let end = truncate_to_i64_low(ticks as f64 + duration as f64);
            e.set(this, LoadingMenu::uiFadeInComplete, end);
        } else {
            let ticks = now(e);
            e.set(this, LoadingMenu::uiFadeInComplete, ticks);
        }
    }
    let ticks = now(e);
    let end = e.get(this, LoadingMenu::uiFadeInComplete);
    let menu_tile = e.get(this, LoadingMenu::pMenu).addr();
    if ticks < end {
        let ticks = now(e);
        let remaining = end.wrapping_sub(ticks);
        let duration: f32 = e.global(FADE_DURATION);
        let value = (1.0 - remaining as f64 / duration as f64) as f32;
        fn_0078ce10(e, this, menu_tile, value);
    } else {
        fn_0078ce10(e, this, menu_tile, 1.0);
        set_flag(e, this, 4, 0);
        let count = e.get(this, LoadingMenu::m_SlideCount);
        e.set(this, LoadingMenu::m_SlideCount, count.wrapping_add(1));
    }
}

// Translated from 0078cdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The rendered texture: `BSRenderedTexture::GetTexture` (flag 0) of the
/// `NiPointer` at `011ded3c`. The register argument is not used.
pub fn fn_0078cdf0(e: &mut Engine, _unused_0: u32) -> u32 {
    let rendered = e.call(NI_POINTER_GET, &args![RENDERED_TEXTURE]).u32();
    e.call(RENDERED_GET_TEXTURE, &args![rendered, 0u32]).u32()
}

// Translated from 0078ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fades `tile` and its children to `value` (clamped to 0..1, through the
/// min/max helpers). Nothing for a null tile or the tile at `+0xac`. A tile
/// of type `0x388` (virtual slot `0xc`) is faded through its model
/// (`0078cf10`); any other gets value `0xfa9` = `value * 255` (inverted
/// first when it is the tile at `+0x58`), `bDoTileUpdate` is set and the
/// children are faded the same way.
pub fn fn_0078ce10(e: &mut Engine, this: Ptr<LoadingMenu>, item: u32, value: f32) {
    if item == 0 || item == tile(e, this, SLOT_FADE_B) {
        return;
    }
    let value = e.call(FLOAT_MIN, &args![value, 1.0f32]).f32();
    let mut value = e.call(FLOAT_MAX, &args![value, 0.0f32]).f32();
    if e.vcall(item, TILE_TYPE_SLOT, &args![]).u32() == MODEL_TILE_TYPE {
        let model = model_of(e, item);
        fn_0078cf10(e, this, model, value);
        return;
    }
    if item == tile(e, this, SLOT_FADE_IMAGE) {
        value = (1.0 - value as f64) as f32;
    }
    let scale: f64 = e.global(BYTE_SCALE);
    set_tile_value(e, item, 0xfa9, (value as f64 * scale) as f32, 1);
    e.mem.set_u8(DO_TILE_UPDATE, 1);
    let list = item + 4;
    e.with_stack(4, |e, iterator| {
        let head = e.call(NI_POINTER_GET, &args![list]).u32();
        e.mem.set_u32(iterator.addr(), head);
        while e.mem.u32(iterator.addr()) != 0 {
            let element = e.call(POINTER_LIST_STEP, &args![list, iterator]).u32();
            let child = e.mem.u32(element);
            fn_0078ce10(e, this, child, value);
        }
    });
}

// Translated from 0078cf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `value` to the shader property of `node` and, first, of every child
/// when the node's cast (virtual slot `0xc`) gives a node. The property is
/// looked up (`NiAVObject::GetProperty`) with the id `00438220` when the
/// object's slot `0x18` answers non-null.
pub fn fn_0078cf10(e: &mut Engine, _this: Ptr<LoadingMenu>, node: u32, value: f32) {
    if node == 0 {
        return;
    }
    let children = e.vcall(node, AS_NODE_SLOT, &args![]).u32();
    let mut index = 0u32;
    while children != 0 && index < e.call(NODE_CHILD_COUNT, &args![children]).u32() {
        let child = e.call(NODE_CHILD_AT, &args![children, index]).u32();
        fn_0078cf10(e, _this, child, value);
        index += 1;
    }
    if e.vcall(node, HAS_PROPERTY_SLOT, &args![]).u32() != 0 {
        let property_id = e.call(PROPERTY_ID, &args![]).u32();
        let property = e.call(NODE_PROPERTY, &args![node, property_id]).u32();
        if property != 0 {
            e.call(MODEL_FADE, &args![property, value]);
        }
    }
}

/// The loading thread object (`LoadingMenu::pLoadingThread`, global
/// `011da0c4`).
fn loading_thread(e: &Engine) -> u32 {
    e.mem.u32(LOADING_THREAD)
}

// Translated from 0078cfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::SuspendBackgroundThread` (Xbox PDB): with the menu and its
/// thread existing, the thread active (`008905f0`) and not already asked to
/// suspend: sets its priority to 1, sets `bFlagSuspend` and waits on the
/// main thread semaphore.
pub fn loading_menu_suspend_background_thread(e: &mut Engine) {
    if e.mem.u32(MENU_INSTANCE) == 0 || loading_thread(e) == 0 {
        return;
    }
    let thread = loading_thread(e);
    if !e.call(BS_THREAD_IS_ACTIVE, &args![thread]).bool() {
        return;
    }
    let thread = Ptr::<LoadingMenuThread>::new(loading_thread(e));
    if e.get(thread, LoadingMenuThread::bFlagSuspend) != 0 {
        return;
    }
    e.call(BS_THREAD_SET_PRIORITY, &args![thread, 1u32]);
    let thread = Ptr::<LoadingMenuThread>::new(loading_thread(e));
    e.set(thread, LoadingMenuThread::bFlagSuspend, 1u8);
    let thread = loading_thread(e);
    e.call(SEMAPHORE_WAIT, &args![thread + MAIN_SEMAPHORE]);
}

// Translated from 0078d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::ResumeBackgroundThread` (Xbox PDB): with the menu and its
/// thread existing, wakes the thread (`0078d570`).
pub fn loading_menu_resume_background_thread(e: &mut Engine) {
    if e.mem.u32(MENU_INSTANCE) != 0 && loading_thread(e) != 0 {
        let thread = Ptr::<LoadingMenuThread>::new(loading_thread(e));
        fn_0078d570(e, thread);
    }
}

// Translated from 0078d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the thread existing: sets its `bExit` and wakes it (`0078d570`).
pub fn fn_0078d050(e: &mut Engine) {
    if loading_thread(e) != 0 {
        let thread = Ptr::<LoadingMenuThread>::new(loading_thread(e));
        e.set(thread, LoadingMenuThread::bExit, 1u8);
        let thread = Ptr::<LoadingMenuThread>::new(loading_thread(e));
        fn_0078d570(e, thread);
    }
}

// Translated from 0078d080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::ShowChanges` (Xbox PDB): redraws the loading menu from the
/// background thread. Between the decal manager's begin and end calls it
/// takes the render section (`0044b130` on `00713d70`'s object) and the
/// redraw critical section `011f4380`; when it cannot, it releases what it
/// has and stops. When tiles changed (`bDoTileUpdate`) it needs the redraw
/// lock `011f4480` too (`0078d1f0`). It then updates all tiles (when they
/// changed), flushes, steps both render stages 1 to 22 of the render
/// system, renders the scene with the redraw flag set, and releases the
/// locks.
pub fn loading_menu_show_changes(e: &mut Engine) {
    decals_begin(e);
    let section = e.call(REDRAW_SECTION_GETTER, &args![]).u32();
    if !e.call(SECTION_TRY_LOCK, &args![section]).bool() {
        decals_end(e);
        return;
    }
    if fn_0078d1d0(e, Ptr::new(REDRAW_SECTION)) == 0 {
        e.call(SECTION_UNLOCK, &args![section]);
        decals_end(e);
        return;
    }
    if e.mem.u8(DO_TILE_UPDATE) != 0 && !fn_0078d1f0(e) {
        e.call(SECTION_UNLOCK, &args![REDRAW_SECTION]);
        e.call(SECTION_UNLOCK, &args![section]);
        decals_end(e);
        return;
    }
    if e.mem.u8(DO_TILE_UPDATE) != 0 {
        e.call(TILE_UPDATE_ALL, &args![1u32]);
        e.call(REDRAW_FLUSH, &args![]);
    }
    let system = e.call(RENDER_SYSTEM, &args![]).u32();
    e.call(RENDER_BEGIN, &args![system]);
    for stage in 1..0x17u32 {
        let system = e.call(RENDER_SYSTEM, &args![]).u32();
        e.call(RENDER_STAGE_A, &args![system, 0u32, stage]);
        let system = e.call(RENDER_SYSTEM, &args![]).u32();
        e.call(RENDER_STAGE_B, &args![system, 1u32, stage]);
    }
    e.mem.set_u8(REDRAWING, 1);
    let player = e.call(PLAYER_GETTER, &args![]).u32();
    e.call(RENDER_SCENE, &args![player, 0u32, 0u32]);
    e.mem.set_u8(REDRAWING, 0);
    let system = e.call(RENDER_SYSTEM, &args![]).u32();
    e.call(RENDER_END, &args![system]);
    decals_end(e);
    e.call(SECTION_UNLOCK, &args![section]);
    e.call(SECTION_UNLOCK, &args![REDRAW_SECTION]);
}

// Translated from 0078d1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TryEnterCriticalSection(section)` through the import (slot `00fdf1b0`).
pub fn fn_0078d1d0(e: &mut Engine, section: Ptr) -> i32 {
    e.call(TRY_ENTER_CRITICAL_SECTION, &args![section]).i32()
}

// Translated from 0078d1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0078d200` on the redraw lock `011f4480`.
pub fn fn_0078d1f0(e: &mut Engine) -> bool {
    fn_0078d200(e, Ptr::new(REDRAW_LOCK))
}

// Translated from 0078d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the recursive lock at `this` (owner thread id, count) for the
/// current thread: when it already owns it the count is incremented;
/// otherwise the owner word is set with a compare exchange from 0 to the
/// thread id (`0043b460`) and, when that works (the previous value was 0),
/// `0040fbe0` is called and the count is set to 1. Returns whether the lock
/// is held.
pub fn fn_0078d200(e: &mut Engine, this: Ptr) -> bool {
    let thread_id = e.call(CURRENT_THREAD_ID, &args![]).u32();
    if e.mem.u32(this.addr()) == thread_id {
        let count = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(this.addr() + 4, count.wrapping_add(1));
        return true;
    }
    let previous = e
        .call(COMPARE_EXCHANGE, &args![this, 0u32, thread_id])
        .u32();
    let held = previous == 0;
    if held {
        e.call(ERROR_LOG, &args![]);
        e.mem.set_u32(this.addr() + 4, 1);
    }
    held
}

// Translated from 0078d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::SetupForMainMenu` (Xbox PDB): without a menu, creates it
/// (`Create(0, 0, 0)`) when `create` is nonzero; with one, suspends the
/// background thread. Then, if there is a menu, `HideAllBut(0, 0)`.
pub fn loading_menu_setup_for_main_menu(e: &mut Engine, create: u8) {
    if e.mem.u32(MENU_INSTANCE) == 0 {
        if create != 0 {
            loading_menu_create(e, 0, 0, 0);
        }
    } else {
        loading_menu_suspend_background_thread(e);
    }
    let menu = e.mem.u32(MENU_INSTANCE);
    if menu != 0 {
        loading_menu_hide_all_but(e, Ptr::new(menu), 0, 0);
    }
}

// Translated from 0078d2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a menu: `HideAllBut(0, 0)`.
pub fn fn_0078d2d0(e: &mut Engine) {
    let menu = e.mem.u32(MENU_INSTANCE);
    if menu != 0 {
        loading_menu_hide_all_but(e, Ptr::new(menu), 0, 0);
    }
}

// Translated from 0078d2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenuThread::LoadingMenuThread` (Xbox PDB): the `BSThread` base,
/// the vtable, the two semaphores (`0078d3b0` at `+0x30` and `+0x3c`), the
/// three flag bytes cleared, then the thread named `"LoadingMenu"`
/// (`Initialize(0x4000, name, 0)`), pinned to processor 5 and given priority
/// -1. Returns `this`. (The exception-unwinding frame is not translated.)
pub fn loading_menu_thread_loading_menu_thread(
    e: &mut Engine,
    this: Ptr<LoadingMenuThread>,
) -> Ptr<LoadingMenuThread> {
    e.call(BS_THREAD_NEW, &args![this]);
    e.set(this, LoadingMenuThread::vtable, THREAD_VTABLE);
    fn_0078d3b0(e, Ptr::new(this.addr() + BACKGROUND_SEMAPHORE));
    fn_0078d3b0(e, Ptr::new(this.addr() + MAIN_SEMAPHORE));
    e.set(this, LoadingMenuThread::bFlagSuspend, 0u8);
    e.set(this, LoadingMenuThread::bFlagSuspended, 0u8);
    e.set(this, LoadingMenuThread::bExit, 0u8);
    e.call(BS_THREAD_SET_NAME, &args![this, THREAD_NAME]);
    e.call(
        BS_THREAD_INITIALIZE,
        &args![this, THREAD_STACK_SIZE, THREAD_NAME, 0u32],
    );
    e.call(BS_THREAD_SET_PROCESSOR, &args![this, 5u32]);
    e.call(BS_THREAD_SET_PRIORITY, &args![this, 0xffff_ffffu32]);
    this
}

// Translated from 0078d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSemaphore` constructor: count 0, maximum 1, and a new semaphore
/// handle (`CreateSemaphoreA(0, 0, 1, 0)`). Returns `this`.
pub fn fn_0078d3b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 8, 1);
    let handle = e
        .call(CREATE_SEMAPHORE, &args![0u32, 0u32, 1u32, 0u32])
        .u32();
    e.mem.set_u32(this.addr() + 4, handle);
    this
}

// Translated from 0078d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenuThread::scalar deleting destructor` (Xbox PDB): the body
/// (`0078d420`), then `operator delete` when bit 0 of `flags` is set.
pub fn loading_menu_thread_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<LoadingMenuThread>,
    flags: u32,
) -> Ptr<LoadingMenuThread> {
    fn_0078d420(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0078d420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `LoadingMenuThread` (the decompiler's library
/// match `CMFCRibbonInfo::XQAT::~XQAT` is wrong): stores the vtable, destroys
/// the two semaphores (`0055a2d0` at `+0x3c`, then `+0x30`) and the
/// `BSThread` base (`00aa63c0`). (The exception-unwinding frame is not
/// translated.)
pub fn fn_0078d420(e: &mut Engine, this: Ptr<LoadingMenuThread>) {
    e.set(this, LoadingMenuThread::vtable, THREAD_VTABLE);
    e.call(SEMAPHORE_DESTRUCTOR, &args![this.addr() + MAIN_SEMAPHORE]);
    e.call(
        SEMAPHORE_DESTRUCTOR,
        &args![this.addr() + BACKGROUND_SEMAPHORE],
    );
    e.call(BS_THREAD_DESTRUCTOR, &args![this]);
}

// Translated from 0078d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenuThread::ThreadProc` (Xbox PDB): the loop of the background
/// thread; returns 0 once `bExit` is set. Each round: sleeps until the
/// scheduled tick when it is ahead; then, unless asked to suspend, schedules
/// the next round 0x32 ms ahead. When asked to suspend (`bFlagSuspend`), and
/// not (during the initial load, with the string setting `011ded5c` empty,
/// and `0078a1f0` true for the model tile of the menu), it releases the main
/// semaphore, sets `bFlagSuspended`, waits on its own semaphore, restores
/// priority -1 and clears `bFlagSuspended`. Every round ends with
/// `Update` on the menu and `ShowChanges`.
pub fn loading_menu_thread_thread_proc(e: &mut Engine, this: Ptr<LoadingMenuThread>) -> u32 {
    let mut next_round = now(e);
    loop {
        if e.get(this, LoadingMenuThread::bExit) == 1 {
            return 0;
        }
        if now(e) < next_round {
            let ticks = now(e);
            e.call(SLEEP_TICK, &args![next_round.wrapping_sub(ticks)]);
        }
        let mut suspend = e.get(this, LoadingMenuThread::bFlagSuspend) != 0;
        if suspend && e.mem.u8(INITIAL_LOAD) != 0 {
            let text = setting_string(e, SETTING_MUSIC_VOLUME);
            if e.mem.u8(text) as i8 == 0 {
                let menu = e.mem.u32(MENU_INSTANCE);
                let model_tile = e.mem.u32(menu + SLOT_MODEL);
                if fn_0078a1f0(e, model_tile) {
                    suspend = false;
                }
            }
        }
        if suspend {
            e.call(SEMAPHORE_RELEASE, &args![this.addr() + MAIN_SEMAPHORE]);
            e.set(this, LoadingMenuThread::bFlagSuspended, 1u8);
            e.call(SEMAPHORE_WAIT, &args![this.addr() + BACKGROUND_SEMAPHORE]);
            e.call(BS_THREAD_SET_PRIORITY, &args![this, 0xffff_ffffu32]);
            e.set(this, LoadingMenuThread::bFlagSuspended, 0u8);
        } else {
            next_round = now(e).wrapping_add(0x32);
        }
        let menu = Ptr::<LoadingMenu>::new(e.mem.u32(MENU_INSTANCE));
        loading_menu_update(e, menu);
        loading_menu_show_changes(e);
    }
}

// Translated from 0078d570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wakes the thread when it is suspended (`bFlagSuspended`): clears
/// `bFlagSuspend` and releases the semaphore the thread waits on
/// (`BackgroundThreadSema`, `+0x30`).
pub fn fn_0078d570(e: &mut Engine, this: Ptr<LoadingMenuThread>) {
    if e.get(this, LoadingMenuThread::bFlagSuspended) != 0 {
        e.set(this, LoadingMenuThread::bFlagSuspend, 0u8);
        // 0078d589..0078d58f: `ADD ECX,0x30`, the background semaphore.
        e.call(
            SEMAPHORE_RELEASE,
            &args![this.addr() + BACKGROUND_SEMAPHORE],
        );
    }
}

// Translated from 0078d5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadingMenu::GetPlayerLevelProgress` (Xbox PDB): the fraction of the way
/// from the experience needed for the current level to the next one, from
/// the player's experience (value `0x18` of the object embedded at
/// `owner + 0xa4`, virtual slot `0xc`). 0 when the level (the low word of
/// `0087f9f0`) is not below the level cap setting. Logs a warning when the
/// experience is below the current level's requirement.
pub fn loading_menu_get_player_level_progress(e: &mut Engine) -> f32 {
    let owner = e.mem.u32(PLAYER_OWNER);
    let level = e.call(PLAYER_LEVEL, &args![owner]).u32() & 0xffff;
    let cap = setting_int(e, SETTING_LEVEL_CAP);
    if (level as i32) >= cap as i32 {
        return 0.0;
    }
    let owner = e.mem.u32(PLAYER_OWNER);
    let level = e.call(PLAYER_LEVEL, &args![owner]).u32() & 0xffff;
    let low = e.call(REQUIRED_EXPERIENCE, &args![level]).u32();
    let high = e.call(REQUIRED_EXPERIENCE, &args![level + 1]).u32();
    let owner = e.mem.u32(PLAYER_OWNER);
    let current = e.vcall(owner + 0xa4, 0x0c, &args![0x18u32]).f32();
    if (current as f64) < (low as i32) as f64 {
        e.call(
            LOG_LINE,
            &args![0x12u32, LEVEL_PROGRESS_WARNING, low, high, current as f64],
        );
    }
    let span = high.wrapping_sub(low) as i32;
    ((current as f64 - (low as i32) as f64) / span as f64) as f32
}

// Translated from 0078d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Caches the player state shown on the loading screen: the word
/// `00952b30` returns for the player object (`011da168`), the level progress
/// (`011da16c`) and the level (`011da170`).
pub fn fn_0078d670(e: &mut Engine) {
    let owner = e.mem.u32(PLAYER_OWNER);
    let process = e.call(PLAYER_PROCESS, &args![owner]).u32();
    e.mem.set_u32(TIP_TEXT_OWNER, process);
    let progress = loading_menu_get_player_level_progress(e);
    e.mem.set_f32(TIP_SCALE, progress);
    let owner = e.mem.u32(PLAYER_OWNER);
    let level = e.call(PLAYER_LEVEL, &args![owner]).u32() & 0xffff;
    e.mem.set_u32(TIP_VALUE, level);
}

// Translated from 0078d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the cached player state into the save game buffer `buffer`: first
/// refreshes it (`0078d670`), then writes the form id of the `011da168`
/// object (0 without one), its handle word (`00726070`), the progress, the
/// level and the byte `0078a930` returns.
pub fn fn_0078d6b0(e: &mut Engine, buffer: u32) {
    fn_0078d670(e);
    let owner = e.mem.u32(TIP_TEXT_OWNER);
    let form_id = if owner != 0 {
        e.call(PROCESS_VALUE, &args![owner]).u32()
    } else {
        0
    };
    let handle = if owner != 0 {
        e.call(NODE_NEXT, &args![owner]).u32()
    } else {
        0
    };
    e.call(SAVE_FORM_ID, &args![buffer, form_id, 0u32]);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), handle);
        e.call(SAVE_BYTES, &args![buffer, cell, 4u32, 0u32]);
    });
    e.call(SAVE_BYTES, &args![buffer, TIP_SCALE, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, TIP_VALUE, 4u32, 0u32]);
    let locked = fn_0078a930(e);
    e.with_stack(1, |e, cell| {
        e.mem.set_u8(cell.addr(), locked as u8);
        e.call(SAVE_BYTES, &args![buffer, cell, 1u32, 0u32]);
    });
}

// Translated from 0078d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads what `0078d6b0` saved from the load game buffer `buffer`: the form
/// (looked up and cast to the owner type), the handle, the progress and the
/// level (progress reset to 0 when the level is not below the level cap), the
/// byte (passed to `005c25e0`); with a menu it then shows the wheel tile
/// (`0xfa3` = 1) and clears state bit 8; finally `004581c0(1)`.
pub fn fn_0078d770(e: &mut Engine, buffer: u32) {
    let form_id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(FORM_LOOKUP, &args![form_id]).u32();
    let owner = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                FORM_TYPE_DESCRIPTOR,
                OWNER_TYPE_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    let handle = e.with_stack(4, |e, cell| {
        e.call(LOAD_BYTES, &args![buffer, cell, 4u32]);
        e.mem.u32(cell.addr())
    });
    let process = if owner != 0 {
        e.call(OWNER_BY_HANDLE, &args![owner, handle]).u32()
    } else {
        0
    };
    e.mem.set_u32(TIP_TEXT_OWNER, process);
    e.call(LOAD_BYTES, &args![buffer, TIP_SCALE, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, TIP_VALUE, 4u32]);
    let cap = setting_int(e, SETTING_LEVEL_CAP);
    if e.mem.u32(TIP_VALUE) >= cap {
        e.mem.set_f32(TIP_SCALE, 0.0);
    }
    let byte = e.with_stack(1, |e, cell| {
        e.call(LOAD_BYTES, &args![buffer, cell, 1u32]);
        e.mem.u8(cell.addr())
    });
    e.call(APPLY_SAVED_BYTE, &args![byte as u32]);
    let menu = e.mem.u32(MENU_INSTANCE);
    if menu != 0 {
        let menu = Ptr::<LoadingMenu>::new(menu);
        let wheel = tile(e, menu, SLOT_WHEEL);
        set_tile_int(e, wheel, 0xfa3, 1);
        set_flag(e, menu, 8, 0);
    }
    e.call(AFTER_LOAD, &args![1u32]);
}

// Translated from 0078d880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the temporary candidate `BSSimpleArray<TESLoadScreen*>`
/// (1024 per growth): stores the vtable `0107410c` and initializes it
/// (`006b3eb0(0, 0)`). Returns `this`.
pub fn fn_0078d880(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), CANDIDATE_ARRAY_VTABLE);
    e.call(CANDIDATE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0078d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the candidate array: stores the vtable and frees the
/// buffer (`008454f0` with 1).
pub fn fn_0078d8b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CANDIDATE_ARRAY_VTABLE);
    e.call(CANDIDATES_RESET, &args![this, 1u32]);
}

// Translated from 0078d8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESLoadScreen*, 1024>::scalar deleting destructor` (Xbox
/// PDB): the body (`0078d8b0`), then `operator delete` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn bs_simple_array_tes_load_screen_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0078d8b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00788730, loading_menu_get_class(Ptr<LoadingMenu>) -> u32),
        entry!(
            0x00788740,
            loading_menu_attach_tile_by_id(Ptr<LoadingMenu>, i32, u32)
        ),
        entry!(
            0x00788770,
            loading_menu_scalar_deleting_destructor(Ptr<LoadingMenu>, u32) -> Ptr<LoadingMenu>
        ),
        entry!(0x007887a0, fn_007887a0(Ptr<LoadingMenu>)),
        entry!(0x00788a90, fn_00788a90(u8)),
        entry!(0x00788aa0, fn_00788aa0() -> u8),
        entry!(0x00788ab0, fn_00788ab0(Ptr<LoadingMenu>, u32)),
        entry!(0x00788cc0, loading_menu_create(u32, u8, u8) -> u32),
        entry!(0x00788f20, fn_00788f20() -> u32),
        entry!(0x00788f30, fn_00788f30(Ptr<LoadingMenu>)),
        entry!(0x00788f80, fn_00788f80(Ptr<LoadingMenu>)),
        entry!(0x00788fe0, fn_00788fe0(Ptr<LoadingMenu>)),
        entry!(0x00789080, fn_00789080(Ptr<LoadingMenu>)),
        entry!(0x007890e0, loading_menu_create_impl(Ptr<LoadingMenu>, u32)),
        entry!(0x00789820, loading_menu_update(Ptr<LoadingMenu>)),
        entry!(0x0078a1c0, fn_0078a1c0(Ptr<LoadingMenu>, u32)),
        entry!(0x0078a1f0, fn_0078a1f0(u32) -> bool),
        entry!(0x0078a230, fn_0078a230(Ptr<LoadingMenu>)),
        entry!(0x0078a260, fn_0078a260(u32) -> u32),
        entry!(0x0078a280, fn_0078a280(Ptr<LoadingMenu>, u32)),
        entry!(0x0078a3e0, fn_0078a3e0(Ptr<LoadingMenu>, u32)),
        entry!(
            0x0078a5a0,
            loading_menu_make_load_screen_list(Ptr<LoadingMenu>)
        ),
        entry!(0x0078a910, fn_0078a910(u32, u32) -> u32),
        entry!(0x0078a930, fn_0078a930() -> bool),
        entry!(0x0078a940, fn_0078a940(Ptr<LoadingMenu>)),
        entry!(0x0078a9f0, fn_0078a9f0(Ptr<LoadingMenu>)),
        entry!(0x0078aa90, fn_0078aa90(Ptr<LoadingMenu>) -> bool),
        entry!(0x0078ab20, fn_0078ab20(Ptr<LoadingMenu>, u32) -> bool),
        entry!(0x0078abd0, fn_0078abd0(Ptr<LoadingMenu>)),
        entry!(
            0x0078ac60,
            loading_menu_build_candidate_list(Ptr<LoadingMenu>, u32)
        ),
        entry!(0x0078ada0, fn_0078ada0(u32) -> bool),
        entry!(0x0078adc0, fn_0078adc0(Ptr<LoadingMenu>, u32)),
        entry!(0x0078aec0, fn_0078aec0(Ptr<LoadingMenu>)),
        entry!(0x0078afa0, fn_0078afa0(Ptr<LoadingMenu>, u8)),
        entry!(0x0078afc0, fn_0078afc0(Ptr<LoadingMenu>, u32, f32, i32)),
        entry!(0x0078b100, fn_0078b100(Ptr<LoadingMenu>, f32, i32)),
        entry!(0x0078b190, fn_0078b190(Ptr<LoadingMenu>, f32, i32)),
        entry!(0x0078b220, fn_0078b220(Ptr<LoadingMenu>, f32, i32)),
        entry!(0x0078b360, fn_0078b360(Ptr<LoadingMenu>, f32, i32)),
        entry!(0x0078b3f0, fn_0078b3f0(Ptr<LoadingMenu>, u32, u8)),
        entry!(0x0078b650, fn_0078b650(Ptr<LoadingMenu>, u8)),
        entry!(0x0078b6a0, fn_0078b6a0(Ptr<LoadingMenu>, u8)),
        entry!(0x0078b6f0, fn_0078b6f0(Ptr<LoadingMenu>, u8)),
        entry!(0x0078b7c0, fn_0078b7c0(Ptr<LoadingMenu>, u8)),
        entry!(0x0078b810, fn_0078b810(Ptr<LoadingMenu>)),
        entry!(
            0x0078b9b0,
            loading_menu_calc_load_screen_type_text_window_faces(Ptr<LoadingMenu>, u32)
        ),
        entry!(0x0078bb40, fn_0078bb40(Ptr<LoadingMenu>)),
        entry!(0x0078bb60, fn_0078bb60(Ptr<LoadingMenu>, u32)),
        entry!(0x0078bc60, fn_0078bc60(u32) -> u32),
        entry!(
            0x0078bc80,
            loading_menu_init_stats_tiles_settings(Ptr<LoadingMenu>, u32)
        ),
        entry!(0x0078bf80, fn_0078bf80(Ptr<LoadingMenu>, u32)),
        entry!(0x0078c210, fn_0078c210(Ptr<LoadingMenu>, u32)),
        entry!(0x0078c770, fn_0078c770(Ptr<LoadingMenu>, u32)),
        entry!(
            0x0078cbd0,
            loading_menu_hide_all_but(Ptr<LoadingMenu>, i32, u8)
        ),
        entry!(0x0078cc50, fn_0078cc50(Ptr<LoadingMenu>)),
        entry!(0x0078cdf0, fn_0078cdf0(u32) -> u32),
        entry!(0x0078ce10, fn_0078ce10(Ptr<LoadingMenu>, u32, f32)),
        entry!(0x0078cf10, fn_0078cf10(Ptr<LoadingMenu>, u32, f32)),
        entry!(0x0078cfc0, loading_menu_suspend_background_thread()),
        entry!(0x0078d020, loading_menu_resume_background_thread()),
        entry!(0x0078d050, fn_0078d050()),
        entry!(0x0078d080, loading_menu_show_changes()),
        entry!(0x0078d1d0, fn_0078d1d0(Ptr) -> i32),
        entry!(0x0078d1f0, fn_0078d1f0() -> bool),
        entry!(0x0078d200, fn_0078d200(Ptr) -> bool),
        entry!(0x0078d280, loading_menu_setup_for_main_menu(u8)),
        entry!(0x0078d2d0, fn_0078d2d0()),
        entry!(
            0x0078d2f0,
            loading_menu_thread_loading_menu_thread(
                Ptr<LoadingMenuThread>,
            ) -> Ptr<LoadingMenuThread>
        ),
        entry!(0x0078d3b0, fn_0078d3b0(Ptr) -> Ptr),
        entry!(
            0x0078d3f0,
            loading_menu_thread_scalar_deleting_destructor(
                Ptr<LoadingMenuThread>,
                u32,
            ) -> Ptr<LoadingMenuThread>
        ),
        entry!(0x0078d420, fn_0078d420(Ptr<LoadingMenuThread>)),
        entry!(
            0x0078d490,
            loading_menu_thread_thread_proc(Ptr<LoadingMenuThread>) -> u32
        ),
        entry!(0x0078d570, fn_0078d570(Ptr<LoadingMenuThread>)),
        entry!(0x0078d5a0, loading_menu_get_player_level_progress() -> f32),
        entry!(0x0078d670, fn_0078d670()),
        entry!(0x0078d6b0, fn_0078d6b0(u32)),
        entry!(0x0078d770, fn_0078d770(u32)),
        entry!(0x0078d880, fn_0078d880(Ptr) -> Ptr),
        entry!(0x0078d8b0, fn_0078d8b0(Ptr)),
        entry!(
            0x0078d8d0,
            bs_simple_array_tes_load_screen_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A vtable whose slot at byte offset `s` points to a double at
    /// `VTABLE_TARGETS + s` (every rig has them).
    const VTABLE: u32 = 0x6000;
    const VTABLE_TARGETS: u32 = 0x6800;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Every callee outside this file: all are doubled in a rig.
    const EXTERNAL: &[u32] = &[
        FLAG_TEST,
        FLAG_SET,
        TICK_COUNT,
        OPERATOR_NEW,
        OPERATOR_DELETE,
        VECTOR_DESTRUCT,
        NI_POINTER_GET,
        NI_POINTER_SET,
        NI_POINTER_DESTRUCTOR,
        NI_POINTER_NEW,
        REMOVE_TEXTURE,
        PLAYER_GETTER,
        DECAL_MANAGER_GETTER,
        AUDIO_GETTER,
        DECAL_BEGIN,
        DECAL_END,
        SOUND_STOP,
        SOUND_RELEASE,
        SOUND_DESTRUCTOR,
        SIMPLE_LIST_DESTRUCTOR,
        POINTER_LIST_DESTRUCTOR,
        STRING_DESTRUCTOR,
        MENU_BASE_DESTRUCTOR,
        POINTER_LIST_CLEAR,
        POINTER_LIST_ADD,
        NODE_ITEM,
        NODE_NEXT,
        POINTER_LIST_STEP,
        LIST_EMPTY,
        LIST_COUNT,
        LIST_PUSH,
        LIST_ADD,
        LIST_FREE_NODES,
        TILE_GET_VALUE,
        TILE_SET_VALUE,
        TILE_SET_STRING,
        TILE_IS_TRUE,
        TILE_IS_VISIBLE,
        TILE_UPDATE_ALL,
        TILE_MAXIMUM_DEPTH,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_MENU,
        TILE_READ_FILE,
        MENU_SET_MENU_TILE,
        TILE_SET_INT,
        TILE_PLAY_SEQUENCE,
        TILE_SHOW_FRAME,
        NODE_UPDATE,
        TILE_MODEL,
        UPDATE_DATA_NEW,
        TILE_SET_ID_VALUE,
        SETTING_VALUE_POINTER,
        SETTING_STRING,
        SETTING_FLOAT,
        SETTING_INT_POINTER,
        STRING_LENGTH,
        MEMSET,
        SNPRINTF,
        STRING_FORMAT,
        CHAR_CONVERT,
        FLOAT_TO_INT,
        CLAMP_FLOAT,
        FORM_TYPE,
        FORM_NAME,
        NAME_COMPARE,
        SET_PATH,
        SET_TEMPORARY,
        WORD_AT_8,
        IS_TEMPORARY,
        LOAD_SCREEN_NEW,
        LOAD_SCREEN_TYPE,
        LOAD_SCREEN_FOR_LOCATION,
        LOAD_SCREEN_GENERIC,
        LOAD_SCREEN_TYPE_FOLDED,
        RANDOM_INDEX,
        MINIMUM,
        CANDIDATES_RESET,
        CANDIDATE_AT,
        CANDIDATE_ADD,
        CANDIDATE_REMOVE,
        ALL_SCREENS_LIST,
        CANDIDATE_LIST_NEW,
        CANDIDATE_LIST_DESTRUCTOR,
        XUI_IS_UP,
        OVERLAY_TEST,
        SKIP_TEST,
        MENU_CLASS_GETTER,
        FLAG_1A8_TEST,
        SET_CURSOR_ALPHA,
        SET_MENU_MODE,
        SET_INFO_FOR_REF,
        SET_CROSSHAIR_TARGET_TYPE,
        SET_CURRENT_SPELL,
        PLAYER_BYTE,
        PLAYER_TILE,
        ROOT_TILE_GETTER,
        PLAYING_SEQUENCE,
        STOP_MOVIE,
        START_SEQUENCE,
        MOVIE_RELEASE,
        MENU_SHOW_CHANGES,
        LOCATION_FLAG,
        REFR_GET_INTERIOR,
        TEXTURE_LOAD,
        TEXTURE_APPLY,
        PROFILE_GUARD_NEW,
        PROFILE_GUARD_DESTRUCTOR,
        NODE_CHILD,
        NODE_PROPERTY,
        PROPERTY_ID,
        FIXED_STRING_NEW,
        FIXED_STRING_DESTRUCTOR,
        MODEL_SET_SCALE,
        MODEL_SET_ROTATION,
        MODEL_SHOW,
        LOG_LINE,
        REAL_SCREEN_WIDTH,
        REAL_SCREEN_HEIGHT,
        AUDIO_IS_ACTIVE,
        AUDIO_IS_MULTI_THREADED,
        AUDIO_SET_MULTI_THREADED,
        AUDIO_FLUSH,
        AUDIO_MUTE_TYPE,
        AUDIO_FADE,
        MUSIC_FADE_OUT,
        MUSIC_PLAY,
        MUSIC_UPDATE,
        MUSIC_REFRESH,
        CREDITS_SET_FLAG,
        SLEEP_TICK,
        VALUE_GETTER,
        TEXT_OWNER_NAME,
        TILE_SEQUENCE,
        TILE_TARGET,
        TILE_SEQUENCE_CALL,
        SEQUENCE_STATE,
        HIDE_ALL_BUT,
        FADE_STEP,
        SET_TILE_FADE,
        SUSPEND_BACKGROUND_THREAD,
        RESUME_BACKGROUND_THREAD,
        THREAD_NEW,
        REFRESH_STATS,
        INIT_ENTER,
        GROUP_VISIBLE_0078B6A0,
        GROUP_VISIBLE_0078B650,
        GROUP_VISIBLE_0078B6F0,
        GROUP_VISIBLE_0078B7C0,
        CALC_FACES,
        ANNOUNCE_SCREEN,
        CHILD_STEP,
        TILE_ADD_NEEDS_UPDATE,
        TILE_ROTATION_UPDATE,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        INSET_X,
        INSET_Y,
        FLOAT_MAX,
        FLOAT_MIN,
        ERROR_LOG,
        FONT_MANAGER,
        FONT_VALUE,
        STRING_DIMENSIONS,
        RENDERED_GET_TEXTURE,
        IMAGE_SET_SOURCE_TEXTURE,
        MODEL_FADE,
        NODE_CHILD_COUNT,
        NODE_CHILD_AT,
        BS_THREAD_NEW,
        BS_THREAD_DESTRUCTOR,
        BS_THREAD_SET_NAME,
        BS_THREAD_INITIALIZE,
        BS_THREAD_SET_PROCESSOR,
        BS_THREAD_SET_PRIORITY,
        BS_THREAD_IS_ACTIVE,
        SEMAPHORE_RELEASE,
        SEMAPHORE_WAIT,
        SEMAPHORE_DESTRUCTOR,
        CREATE_SEMAPHORE,
        TRY_ENTER_CRITICAL_SECTION,
        CURRENT_THREAD_ID,
        COMPARE_EXCHANGE,
        REDRAW_SECTION_GETTER,
        SECTION_TRY_LOCK,
        SECTION_UNLOCK,
        REDRAW_FLUSH,
        RENDER_SYSTEM,
        RENDER_BEGIN,
        RENDER_END,
        RENDER_STAGE_A,
        RENDER_STAGE_B,
        RENDER_SCENE,
        PLAYER_LEVEL,
        PLAYER_PROCESS,
        REQUIRED_EXPERIENCE,
        PROCESS_VALUE,
        SAVE_FORM_ID,
        SAVE_BYTES,
        LOAD_FORM_ID,
        LOAD_BYTES,
        FORM_LOOKUP,
        RT_DYNAMIC_CAST,
        OWNER_BY_HANDLE,
        APPLY_SAVED_BYTE,
        AFTER_LOAD,
        CANDIDATE_ARRAY_INIT,
    ];

    /// An engine with the data the unit reads mapped, and doubles that record
    /// every call they receive, in order.
    struct Rig {
        e: Engine,
        calls: Calls,
        /// The object `DECAL_MANAGER_GETTER` returns (it has the default vtable).
        manager: u32,
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0010_0000);
        e.map(0x011a_0000, 0x0008_0000);
        e.map(0x0126_f000, 0x1000);
        let mut rig = Rig {
            e,
            calls: Rc::new(RefCell::new(Vec::new())),
            manager: 0,
        };
        rig.stubs(EXTERNAL);
        rig.e.map(0x7000, 0x1000);
        rig.stub(FORM_NAME, 0x7000);
        rig.stub(SETTING_STRING, 0x7000);
        rig.stub(SETTING_INT_POINTER, 0x7010);
        rig.stub(SETTING_VALUE_POINTER, 0x7020);
        let slots: Vec<u32> = (0..64).map(|i| VTABLE_TARGETS + 4 * i).collect();
        rig.e.put_vtable(VTABLE, &slots);
        for slot in &slots {
            rig.stub(*slot, 0);
        }
        let model = rig.object(0x100);
        rig.stub(TILE_MODEL, model);
        rig.stub(VTABLE_TARGETS + 0x9c, model);
        rig.manager = rig.object(0x100);
        rig.stub(DECAL_MANAGER_GETTER, rig.manager);
        rig.e.mem.set_f32(MINUS_ONE, -1.0);
        rig.e.mem.set_f32(HALF_PI, std::f32::consts::FRAC_PI_2);
        rig
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
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
        /// Doubles returning 0 at every address in `addrs`.
        fn stubs(&mut self, addrs: &[u32]) {
            for addr in addrs {
                self.stub(*addr, 0);
            }
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
        /// A zeroed menu.
        fn menu(&mut self) -> Ptr<LoadingMenu> {
            self.e.new_object::<LoadingMenu>()
        }
        /// The state word's bit test and set, kept in the menu's `usFlags`
        /// like the game does.
        fn real_flags(&mut self) {
            self.with(FLAG_TEST, |e, a| {
                let flags = e.mem.u16(a[0] + 0x222) as u32;
                eax(flags >> (a[1] & 31) & 1)
            });
            self.with(FLAG_SET, |e, a| {
                let flags = e.mem.u16(a[0] + 0x222) as u32;
                let bit = 1u32 << (a[1] & 31);
                let new = if a[2] as u8 != 0 {
                    flags | bit
                } else {
                    flags & !bit
                };
                e.mem.set_u16(a[0] + 0x222, new as u16);
                eax(0)
            });
        }
        /// Tick count source.
        fn clock(&mut self, ms: u32) {
            self.stub(TICK_COUNT, ms);
        }
        /// List node accessors: the item word address is the node itself, the
        /// next node is the word at `+4` (the `BSSimpleList` layout).
        fn real_nodes(&mut self) {
            self.with(NODE_ITEM, |_, a| eax(a[0]));
            self.with(NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        }
        /// A `BSSimpleList` node holding `item`, chained to `next`.
        fn node(&mut self, item: u32, next: u32) -> u32 {
            let node = self.e.mem.alloc(8);
            self.e.mem.set_u32(node, item);
            self.e.mem.set_u32(node + 4, next);
            node
        }
        /// A `float` setting at `addr`'s `+4`.
        fn put_f64(&mut self, addr: u32, value: f64) {
            self.e.mem.set_f64(addr, value);
        }
    }

    #[test]
    fn every_translation_is_registered() {
        let e = Engine::new();
        for (addr, _) in funcs() {
            assert!(e.is_translated(addr), "{addr:08x}");
        }
        assert_eq!(funcs().len(), 80);
    }

    // ----- 00788730, 00788f20, 00788740 -----

    #[test]
    fn the_class_is_0x3ef() {
        let mut r = rig();
        let menu = r.menu();
        assert_eq!(r.e.call(0x0078_8730, &args![menu]).u32(), 0x3ef);
        assert_eq!(r.e.call(0x0078_8f20, &args![]).u32(), 0x3ef);
    }

    #[test]
    fn attaching_a_tile_stores_it_in_range() {
        let mut r = rig();
        let menu = r.menu();
        r.e.call(0x0078_8740, &args![menu, 3i32, 0x1234u32]);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x28 + 12), 0x1234);
        r.e.call(0x0078_8740, &args![menu, 0x25i32, 0x77u32]);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x28 + 0x25 * 4), 0x77);
    }

    #[test]
    fn attaching_ignores_null_tiles_and_ids_out_of_range() {
        let mut r = rig();
        let menu = r.menu();
        r.e.call(0x0078_8740, &args![menu, 1i32, 0u32]);
        r.e.call(0x0078_8740, &args![menu, 0x26i32, 5u32]);
        r.e.call(0x0078_8740, &args![menu, -1i32, 5u32]);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x2c), 0);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x28 + 0x26 * 4), 0);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x24), 0);
    }

    // ----- 00788a90, 00788aa0 -----

    #[test]
    fn the_switch_byte_round_trips() {
        let mut r = rig();
        r.e.call(0x0078_8a90, &args![1u8]);
        assert_eq!(r.e.mem.u8(0x011d_cfb2), 1);
        assert_eq!(r.e.call(0x0078_8aa0, &args![]).u8(), 1);
        r.e.call(0x0078_8a90, &args![0u8]);
        assert_eq!(r.e.call(0x0078_8aa0, &args![]).u8(), 0);
    }

    // ----- 007887a0, 00788770 -----

    /// Doubles for everything the destructor calls outside the unit.
    fn destructor_rig() -> (Rig, Ptr<LoadingMenu>) {
        let mut r = rig();
        let menu = r.menu();
        r.real_flags();
        r.stubs(&[
            SUSPEND_BACKGROUND_THREAD,
            REMOVE_TEXTURE,
            NI_POINTER_SET,
            XUI_IS_UP,
            AUDIO_IS_ACTIVE,
            MUSIC_FADE_OUT,
            CREDITS_SET_FLAG,
            AUDIO_IS_MULTI_THREADED,
            AUDIO_SET_MULTI_THREADED,
            AUDIO_FADE,
            SOUND_STOP,
            SOUND_RELEASE,
            MOVIE_RELEASE,
            VECTOR_DESTRUCT,
            SOUND_DESTRUCTOR,
            SIMPLE_LIST_DESTRUCTOR,
            STRING_DESTRUCTOR,
            POINTER_LIST_DESTRUCTOR,
            MENU_BASE_DESTRUCTOR,
            LIST_FREE_NODES,
            OPERATOR_DELETE,
        ]);
        r.with(NI_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        r.real_nodes();
        r.stub(AUDIO_GETTER, 0x5000);
        let manager = r.e.mem.alloc(0x100);
        r.e.mem.set_u32(manager, VTABLE);
        r.stub(DECAL_MANAGER_GETTER, manager);
        (r, menu)
    }

    #[test]
    fn the_destructor_releases_everything_and_clears_the_instance() {
        let (mut r, menu) = destructor_rig();
        // Two hold textures; the loading movie of the movie owner; the global
        // instance and initial-load flags.
        r.e.mem.set_u32(menu.addr() + 0x20c, 0x111);
        r.e.mem.set_u32(menu.addr() + 0x20c + 8, 0x222);
        r.e.mem.set_u32(MENU_INSTANCE, menu.addr());
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.mem.set_u8(MENU_REPLACED, 1);
        r.e.mem.set_u32(menu.addr() + 0x1e8, 0x99);
        let owner = r.e.mem.alloc(0xe50);
        r.e.mem.set_u32(owner + 0xe4c, 0x4242);
        r.e.mem.set_u32(MOVIE_OWNER, owner);
        r.e.mem.set_u32(menu.addr() + 0x1d0, 0);
        fn_007887a0(&mut r.e, menu);
        assert_eq!(r.e.mem.u32(menu.addr()), LOADING_MENU_VTABLE);
        assert_eq!(r.calls_to(REMOVE_TEXTURE), vec![vec![0x111], vec![0x222]]);
        assert_eq!(r.calls_to(NI_POINTER_SET).len(), 2);
        assert_eq!(r.e.mem.u8(MENU_REPLACED), 0);
        assert_eq!(r.e.mem.u8(INITIAL_LOAD), 0);
        assert_eq!(r.e.mem.u32(MENU_INSTANCE), 0);
        assert_eq!(r.e.mem.u32(menu.addr() + 0x1e8), 0);
        // The movie is released with 1 and cleared.
        assert_eq!(r.calls_to(MOVIE_RELEASE), vec![vec![0x4242, 1]]);
        assert_eq!(r.e.mem.u32(owner + 0xe4c), 0);
        // Members are destroyed before the base, which comes last.
        let order = r.order();
        assert_eq!(order[0], SUSPEND_BACKGROUND_THREAD);
        assert_eq!(*order.last().unwrap(), MENU_BASE_DESTRUCTOR);
        assert_eq!(
            r.calls_to(POINTER_LIST_DESTRUCTOR)
                .iter()
                .map(|a| a[0] - menu.addr())
                .collect::<Vec<_>>(),
            vec![0x100, 0xf4, 0xe8, 0xdc, 0xd0]
        );
        assert_eq!(r.calls_to(SOUND_RELEASE).len(), 2);
        assert_eq!(
            r.calls_to(VECTOR_DESTRUCT),
            vec![vec![menu.addr() + 0x20c, 4, 4, NI_POINTER_DESTRUCTOR]]
        );
    }

    #[test]
    fn the_destructor_stops_the_music_only_when_it_is_playing() {
        // Nothing up and not the initial load: no music stop at 4000.
        let (mut r, menu) = destructor_rig();
        fn_007887a0(&mut r.e, menu);
        assert!(r.calls_to(MUSIC_FADE_OUT).is_empty());
        // XUI up and the audio active: it stops the music.
        let (mut r, menu) = destructor_rig();
        r.stub(XUI_IS_UP, 1);
        r.stub(AUDIO_IS_ACTIVE, 1);
        fn_007887a0(&mut r.e, menu);
        assert_eq!(r.calls_to(MUSIC_FADE_OUT), vec![vec![4000, 0]]);
        // Initial load with a nonzero volume character behaves the same.
        let (mut r, menu) = destructor_rig();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.mem.set_u8(0x7000, b'1');
        r.stub(AUDIO_IS_ACTIVE, 1);
        fn_007887a0(&mut r.e, menu);
        assert_eq!(r.calls_to(MUSIC_FADE_OUT).len(), 1);
        // The switch byte stops it too and is cleared.
        let (mut r, menu) = destructor_rig();
        r.e.mem.set_u8(SWITCH_BYTE, 1);
        fn_007887a0(&mut r.e, menu);
        assert_eq!(r.calls_to(MUSIC_FADE_OUT), vec![vec![4000, 0]]);
        assert_eq!(r.calls_to(CREDITS_SET_FLAG), vec![vec![0]]);
        assert_eq!(r.e.mem.u8(SWITCH_BYTE), 0);
    }

    #[test]
    fn the_destructor_undoes_the_multi_threaded_audio_it_started() {
        let (mut r, menu) = destructor_rig();
        r.e.mem.set_u16(menu.addr() + 0x222, 1 << 2);
        r.stub(AUDIO_IS_MULTI_THREADED, 1);
        fn_007887a0(&mut r.e, menu);
        assert_eq!(r.calls_to(AUDIO_SET_MULTI_THREADED), vec![vec![0x5000, 0]]);
        let (mut r, menu) = destructor_rig();
        fn_007887a0(&mut r.e, menu);
        assert!(r.calls_to(AUDIO_SET_MULTI_THREADED).is_empty());
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_only_when_asked() {
        let (mut r, menu) = destructor_rig();
        let result = r.e.call(0x0078_8770, &args![menu, 0u32]);
        assert_eq!(result.u32(), menu.addr());
        assert!(r.calls_to(OPERATOR_DELETE).is_empty());
        let result = r.e.call(0x0078_8770, &args![menu, 1u32]);
        assert_eq!(result.u32(), menu.addr());
        assert_eq!(r.calls_to(OPERATOR_DELETE), vec![vec![menu.addr()]]);
    }

    impl Rig {
        /// A zeroed block with the default vtable.
        fn object(&mut self, size: u32) -> u32 {
            let object = self.e.mem.alloc(size);
            self.e.mem.set_u32(object, VTABLE);
            object
        }
        /// A load screen: its sub-object at `+0x24` has the default vtable.
        fn screen(&mut self) -> u32 {
            let screen = self.e.mem.alloc(0x40);
            self.e.mem.set_u32(screen, VTABLE);
            self.e.mem.set_u32(screen + 0x24, VTABLE);
            screen
        }
        /// A menu with every tile slot filled by a tile object, a `LoadScreens`
        /// list of two screens (the head node is embedded at `+0x1d0`) and
        /// `pNextLoadScreen`/`pCurrentLoadScreen` on the first. Returns the
        /// menu; the screens are at the items of the list.
        fn full_menu(&mut self) -> Ptr<LoadingMenu> {
            let menu = self.menu();
            for offset in (0x28..0xc0).step_by(4) {
                let tile = self.object(0x100);
                self.e.mem.set_u32(menu.addr() + offset, tile);
            }
            let first = self.screen();
            let second = self.screen();
            let tail = self.node(second, 0);
            self.e.mem.set_u32(menu.addr() + 0x1d0, first);
            self.e.mem.set_u32(menu.addr() + 0x1d4, tail);
            self.e.mem.set_u32(menu.addr() + 0x1d8, menu.addr() + 0x1d0);
            self.e.mem.set_u32(menu.addr() + 0x1dc, first);
            self.real_nodes();
            menu
        }
        /// The tile object in slot `offset`.
        fn tile_at(&self, menu: Ptr<LoadingMenu>, offset: u32) -> u32 {
            self.e.mem.u32(menu.addr() + offset)
        }
    }

    fn world() -> (Rig, Ptr<LoadingMenu>) {
        let mut r = rig();
        r.real_flags();
        r.clock(10_000);
        let menu = r.full_menu();
        (r, menu)
    }

    fn bit(n: u32) -> u16 {
        1 << n
    }

    // ----- 00788ab0 -----

    #[test]
    fn re_showing_does_nothing_when_bit_3_is_clear() {
        let (mut r, menu) = world();
        fn_00788ab0(&mut r.e, menu, 0x55);
        assert!(r.calls_to(SUSPEND_BACKGROUND_THREAD).is_empty());
        assert!(r.calls_to(RESUME_BACKGROUND_THREAD).is_empty());
        assert_eq!(r.e.get(menu, LoadingMenu::pLocation).addr(), 0);
    }

    #[test]
    fn re_showing_restarts_the_fade_and_steps_two_slides() {
        let (mut r, menu) = world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(3));
        fn_00788ab0(&mut r.e, menu, 0x55);
        assert_eq!(r.e.get(menu, LoadingMenu::m_TileCrossFadeStartTime), 6000);
        assert_eq!(r.e.get(menu, LoadingMenu::pLocation).addr(), 0x55);
        assert_eq!(r.e.get(menu, LoadingMenu::m_SlideCount), 2);
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(3), 0);
        assert_eq!(r.e.mem.u8(DO_TILE_UPDATE), 1);
        let order = r.order();
        assert_eq!(order[0], FLAG_TEST);
        assert_eq!(order[1], SUSPEND_BACKGROUND_THREAD);
        assert_eq!(*order.last().unwrap(), RESUME_BACKGROUND_THREAD);
        assert_eq!(r.calls_to(TILE_UPDATE_ALL), vec![vec![1]]);
        // The hidden wheel is made visible (value 0xfa3 = 1); the player's
        // tile gets 0.
        let wheel = r.tile_at(menu, SLOT_WHEEL);
        let sets = r.calls_to(TILE_SET_INT);
        assert_eq!(sets[0], vec![wheel, 0xfa3, 1]);
        assert_eq!(sets.last().unwrap()[1..], [0xfa3, 0]);
        // Not in overlay mode: the tips are enabled and the status tiles set.
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
        assert_eq!(r.calls_to(REFRESH_STATS).len(), 1);
    }

    #[test]
    fn re_showing_in_overlay_mode_disables_tips() {
        let (mut r, menu) = world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(3) | bit(6));
        r.stub(OVERLAY_TEST, 1);
        fn_00788ab0(&mut r.e, menu, 0);
        assert!(r.calls_to(REFRESH_STATS).is_empty());
        // Bit 6 was cleared by 00788afa0(0) and is not set again, but the
        // list rebuild sets bit 5 (overlay).
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(5), 0);
    }

    #[test]
    fn re_showing_deletes_the_stats_menu_when_it_is_replaced() {
        let (mut r, menu) = world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(3));
        r.e.mem.set_u8(MENU_REPLACED, 1);
        r.stub(MENU_CLASS_GETTER, 0x3f5);
        r.stub(TILE_GET_MENU_BY_CLASS, 0x300);
        let stats = r.object(0x40);
        r.stub(TILE_GET_MENU, stats);
        fn_00788ab0(&mut r.e, menu, 0);
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS), vec![vec![0x3f5]]);
        assert_eq!(r.calls_to(TILE_GET_MENU), vec![vec![0x300]]);
        // Virtual slot 0 (the destructor) with 1.
        assert_eq!(r.calls_to(VTABLE_TARGETS), vec![vec![stats, 1]]);
        // A menu with flag 1 set in +0x1a8 is left alone.
        let (mut r, menu) = world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(3));
        r.e.mem.set_u8(MENU_REPLACED, 1);
        let stats = r.object(0x40);
        r.stub(TILE_GET_MENU, stats);
        r.stub(FLAG_1A8_TEST, 1);
        fn_00788ab0(&mut r.e, menu, 0);
        assert!(r.calls_to(VTABLE_TARGETS).is_empty());
    }

    // ----- 00788cc0 -----

    #[test]
    fn creating_when_a_menu_exists_re_shows_it() {
        let (mut r, menu) = world();
        r.e.mem.set_u32(MENU_INSTANCE, menu.addr());
        r.stub(TILE_GET_MENU_BY_CLASS, 0x321);
        let result = loading_menu_create(&mut r.e, 0x77, 1, 1);
        assert_eq!(result, 0x321);
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS)[0], vec![0x3ef]);
        // Bit 3 was set for the re-show (which clears it), bit 10 takes the
        // last argument; the location reached the menu.
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(10), 0);
        assert_eq!(r.e.get(menu, LoadingMenu::pLocation).addr(), 0x77);
        assert_eq!(r.calls_to(SUSPEND_BACKGROUND_THREAD).len(), 1);
        assert!(r.calls_to(TILE_READ_FILE).is_empty());
    }

    /// A rig for building the menu from the XML: the file read returns a root
    /// tile whose menu is a loading menu.
    fn fresh_world() -> (Rig, Ptr<LoadingMenu>, u32) {
        let (mut r, menu) = world();
        let root = r.object(0x40);
        r.stub(TILE_GET_MENU_BY_CLASS, 0);
        r.stub(PLAYER_GETTER, 0x9100);
        r.stub(ROOT_TILE_GETTER, 0x9200);
        r.stub(TILE_READ_FILE, root);
        r.stub(TILE_GET_MENU, menu.addr());
        r.e.mem.set_u32(menu.addr(), VTABLE);
        r.stub(VTABLE_TARGETS + 0x34, 0x3ef);
        (r, menu, root)
    }

    #[test]
    fn creating_reads_the_menu_file_and_builds_the_menu() {
        let (mut r, menu, root) = fresh_world();
        r.stub(OPERATOR_NEW, 0x9300);
        r.stub(THREAD_NEW, 0x9400);
        let result = loading_menu_create(&mut r.e, 0x77, 0, 1);
        assert_eq!(result, root);
        assert_eq!(
            r.calls_to(TILE_READ_FILE),
            vec![vec![0x9200, MENU_FILE_PATH]]
        );
        assert_eq!(r.e.mem.u8(READING_MENU_FILE), 0);
        assert_eq!(r.e.mem.u32(MENU_INSTANCE), menu.addr());
        assert_eq!(
            r.calls_to(MENU_SET_MENU_TILE),
            vec![vec![menu.addr(), root, 0]]
        );
        assert_eq!(r.calls_to(SET_CURSOR_ALPHA), vec![vec![0]]);
        // The four tile lists were filled, the four groups hidden, _Create
        // ran (the location is stored), and the thread was created.
        assert_eq!(r.calls_to(POINTER_LIST_CLEAR).len(), 5);
        assert_eq!(r.calls_to(GROUP_VISIBLE_0078B6A0)[0], vec![menu.addr(), 0]);
        assert_eq!(r.calls_to(GROUP_VISIBLE_0078B650)[0], vec![menu.addr(), 0]);
        assert_eq!(r.calls_to(GROUP_VISIBLE_0078B6F0)[0], vec![menu.addr(), 0]);
        assert_eq!(r.calls_to(GROUP_VISIBLE_0078B7C0)[0], vec![menu.addr(), 0]);
        assert_eq!(r.calls_to(OPERATOR_NEW), vec![vec![0x4c]]);
        assert_eq!(r.calls_to(THREAD_NEW), vec![vec![0x9300]]);
        assert_eq!(r.e.mem.u32(LOADING_THREAD), 0x9400);
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(10), 0);
        // Both fade tiles start at 0.
        let fade_a = r.tile_at(menu, SLOT_FADE_A);
        let fade_b = r.tile_at(menu, SLOT_FADE_B);
        let sets = r.calls_to(TILE_SET_INT);
        assert!(sets.contains(&vec![fade_a, 0xfa9, 0]));
        assert!(sets.contains(&vec![fade_b, 0xfa9, 0]));
        assert!(r.calls_to(RESUME_BACKGROUND_THREAD).is_empty());
    }

    #[test]
    fn creating_resumes_an_existing_thread_only_when_asked() {
        let (mut r, _menu, _root) = fresh_world();
        r.e.mem.set_u32(LOADING_THREAD, 0x9400);
        loading_menu_create(&mut r.e, 0, 0, 0);
        assert!(r.calls_to(RESUME_BACKGROUND_THREAD).is_empty());
        assert!(r.calls_to(THREAD_NEW).is_empty());
        let (mut r, _menu, _root) = fresh_world();
        r.e.mem.set_u32(LOADING_THREAD, 0x9400);
        loading_menu_create(&mut r.e, 0, 1, 0);
        assert_eq!(r.calls_to(RESUME_BACKGROUND_THREAD).len(), 1);
        assert_eq!(r.e.mem.u32(LOADING_THREAD), 0x9400);
    }

    #[test]
    fn a_failed_allocation_leaves_the_thread_null() {
        let (mut r, _menu, _root) = fresh_world();
        r.stub(OPERATOR_NEW, 0);
        loading_menu_create(&mut r.e, 0, 0, 0);
        assert!(r.calls_to(THREAD_NEW).is_empty());
        assert_eq!(r.e.mem.u32(LOADING_THREAD), 0);
    }

    #[test]
    fn a_file_with_another_menu_fails_and_deletes_it() {
        let (mut r, menu, _root) = fresh_world();
        r.stub(VTABLE_TARGETS + 0x34, 0x3f0);
        r.with(NODE_NEXT, |_, _| eax(0x77));
        let result = loading_menu_create(&mut r.e, 0, 0, 0);
        assert_eq!(result, 0);
        assert!(r.calls_to(MENU_SET_MENU_TILE).is_empty());
        // Slot 0 (destructor) with 1, because the menu has a tile (nonzero
        // `pMenu`-style word from 00726070).
        assert_eq!(r.calls_to(VTABLE_TARGETS), vec![vec![menu.addr(), 1]]);
        // Without a tile it is not deleted.
        let (mut r, _menu, _root) = fresh_world();
        r.stub(VTABLE_TARGETS + 0x34, 0x3f0);
        r.stub(NODE_NEXT, 0);
        assert_eq!(loading_menu_create(&mut r.e, 0, 0, 0), 0);
        assert!(r.calls_to(VTABLE_TARGETS).is_empty());
    }

    #[test]
    fn a_file_without_a_menu_fails() {
        let (mut r, _menu, _root) = fresh_world();
        r.stub(TILE_GET_MENU, 0);
        assert_eq!(loading_menu_create(&mut r.e, 0, 0, 0), 0);
        assert!(r.calls_to(MENU_SET_MENU_TILE).is_empty());
    }

    // ----- 00788f30, 00788f80, 00788fe0, 00789080 -----

    fn list_adds(r: &Rig, list: u32) -> Vec<u32> {
        r.calls_to(POINTER_LIST_ADD)
            .iter()
            .filter(|a| a[0] == list)
            .map(|a| a[1])
            .collect()
    }

    #[test]
    fn the_tile_lists_hold_the_slots_by_address() {
        let (mut r, menu) = world();
        let base = menu.addr();
        fn_00788f30(&mut r.e, menu);
        assert_eq!(list_adds(&r, base + 0xd0), vec![base + 0x6c, base + 0x70]);
        fn_00788f80(&mut r.e, menu);
        assert_eq!(
            list_adds(&r, base + 0xdc),
            vec![base + 0x94, base + 0x34, base + 0x60]
        );
        fn_00788fe0(&mut r.e, menu);
        assert_eq!(
            list_adds(&r, base + 0xe8),
            vec![
                base + 0x68,
                base + 0x98,
                base + 0x9c,
                base + 0x64,
                base + 0x50,
                base + 0xa0
            ]
        );
        fn_00789080(&mut r.e, menu);
        assert_eq!(list_adds(&r, base + 0xf4), vec![base + 0xb0]);
        assert_eq!(list_adds(&r, base + 0x100), vec![base + 0xb4]);
        // Each list is cleared first.
        let cleared: Vec<u32> = r
            .calls_to(POINTER_LIST_CLEAR)
            .iter()
            .map(|a| a[0] - base)
            .collect();
        assert_eq!(cleared, vec![0xd0, 0xdc, 0xe8, 0xf4, 0x100]);
        assert_eq!(r.order()[0], POINTER_LIST_CLEAR);
    }

    // ----- 007890e0 -----

    /// A world for `_Create`: the flag and node accessors are real, the game
    /// time is 10000 and two floats are set the way the settings give them.
    fn impl_world() -> (Rig, Ptr<LoadingMenu>) {
        let (mut r, menu) = world();
        r.put_f64(MENU_MODE_A, 6006.0);
        r.put_f64(MENU_MODE_B, 102.0);
        r.stub_float(SETTING_FLOAT, 2.5);
        r.stub_float(TILE_MAXIMUM_DEPTH, 7.5);
        r.e.mem.set_f32(0x7020, 3.25);
        (r, menu)
    }

    fn set_values(r: &mut Rig, values: Vec<(u32, f32)>) {
        r.with(TILE_GET_VALUE, move |_, a| {
            st0(values
                .iter()
                .find(|(id, _)| *id == a[1])
                .map_or(0.0, |v| v.1))
        });
    }

    #[test]
    fn create_fixes_the_depth_for_the_two_menu_modes() {
        for mode in [6006.0, 102.0] {
            let (mut r, menu) = impl_world();
            set_values(&mut r, vec![(0xfa7, mode)]);
            loading_menu_create_impl(&mut r.e, menu, 0);
            let pm = r.e.get(menu, LoadingMenu::pMenu).addr();
            assert!(r
                .calls_to(TILE_SET_VALUE)
                .contains(&vec![pm, 0xfad, 7.5f32.to_bits(), 1]));
        }
        let (mut r, menu) = impl_world();
        set_values(&mut r, vec![(0xfa7, 5.0)]);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert!(r.calls_to(TILE_MAXIMUM_DEPTH).is_empty());
    }

    #[test]
    fn create_sets_the_spell_unless_the_player_byte_is_3() {
        let (mut r, menu) = impl_world();
        r.stub(PLAYER_GETTER, 0x9100);
        r.stub(PLAYER_BYTE, 3);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert!(r.calls_to(SET_CURRENT_SPELL).is_empty());
        let (mut r, menu) = impl_world();
        r.stub(PLAYER_GETTER, 0x9100);
        r.stub(PLAYER_BYTE, 4);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert_eq!(r.calls_to(SET_CURRENT_SPELL), vec![vec![0x9100, 2]]);
        assert_eq!(r.calls_to(SET_MENU_MODE), vec![vec![5]]);
        assert_eq!(r.calls_to(SET_INFO_FOR_REF), vec![vec![0, 0, 0]]);
        assert_eq!(r.calls_to(SET_CROSSHAIR_TARGET_TYPE), vec![vec![0]]);
    }

    #[test]
    fn create_stops_the_movie_unless_a_sequence_plays() {
        let (mut r, menu) = impl_world();
        r.e.mem.set_u32(MOVIE_OWNER, 0x9300);
        r.e.mem.set_u32(MOVIE_PLAYER, 0x9400);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert_eq!(r.calls_to(STOP_MOVIE), vec![vec![0x9400]]);
        let (mut r, menu) = impl_world();
        r.e.mem.set_u32(MOVIE_OWNER, 0x9300);
        r.e.mem.set_u32(MOVIE_PLAYER, 0x9400);
        r.stub(PLAYING_SEQUENCE, 1);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert!(r.calls_to(STOP_MOVIE).is_empty());
        // No movie owner: no query at all.
        let (mut r, menu) = impl_world();
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert!(r.calls_to(PLAYING_SEQUENCE).is_empty());
    }

    #[test]
    fn create_closes_the_replaced_menu() {
        let (mut r, menu) = impl_world();
        let other = r.object(0x40);
        r.with(TILE_GET_MENU_BY_CLASS, move |_, a| {
            eax(if a[0] == 0x3f4 { other } else { 0 })
        });
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert_eq!(r.calls_to(VTABLE_TARGETS), vec![vec![other, 1]]);
    }

    #[test]
    fn create_hides_everything_on_the_initial_load() {
        let (mut r, menu) = impl_world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert_eq!(r.calls_to(HIDE_ALL_BUT), vec![vec![menu.addr(), 0, 0]]);
        assert_eq!(r.e.get(menu, LoadingMenu::pLocation).addr(), 0x44);
        // No slide setup: the list was not rebuilt.
        assert!(r.calls_to(CANDIDATES_RESET).is_empty());
    }

    #[test]
    fn create_on_the_initial_load_does_not_read_the_value() {
        let (mut r, menu) = impl_world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert!(
            r.calls_to(VALUE_GETTER).is_empty(),
            "00789222 jumps past it"
        );
    }

    #[test]
    fn create_hides_everything_when_the_value_is_nonzero() {
        let (mut r, menu) = impl_world();
        r.stub_float(VALUE_GETTER, 1.0);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert_eq!(r.calls_to(HIDE_ALL_BUT), vec![vec![menu.addr(), 0, 0]]);
    }

    #[test]
    fn create_with_a_replaced_menu_disables_tips_and_updates_decals() {
        let (mut r, menu) = impl_world();
        r.e.mem.set_u8(MENU_REPLACED, 1);
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert!(r.calls_to(HIDE_ALL_BUT).is_empty());
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
        assert_eq!(r.calls_to(DECAL_BEGIN), vec![vec![r.manager]]);
        assert_eq!(r.calls_to(DECAL_END), vec![vec![r.manager]]);
        // The wheel tile shows frame -1.
        let wheel = r.tile_at(menu, SLOT_WHEEL);
        assert!(r
            .calls_to(TILE_SHOW_FRAME)
            .contains(&vec![wheel, (-1.0f32).to_bits(), 0]));
    }

    fn location_world(form_type: u32, interior: u32) -> (Rig, Ptr<LoadingMenu>) {
        let (mut r, menu) = impl_world();
        r.stub(FORM_TYPE, form_type);
        r.stub(LOCATION_FLAG, interior);
        r.stub(REFR_GET_INTERIOR, interior);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        (r, menu)
    }

    #[test]
    fn an_interior_cell_location_hides_all_but_the_interior_tiles() {
        let (r, menu) = location_world(0x39, 1);
        assert_eq!(r.calls_to(LOCATION_FLAG), vec![vec![0x44]]);
        assert!(r.calls_to(REFR_GET_INTERIOR).is_empty());
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(9), 0);
        assert_eq!(r.calls_to(HIDE_ALL_BUT), vec![vec![menu.addr(), 0xb, 0]]);
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(4), 0);
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 10_000);
        // The slide setup is skipped for interiors.
        assert!(r.calls_to(CANDIDATES_RESET).is_empty());
    }

    #[test]
    fn reference_form_types_ask_the_reference_for_its_interior() {
        for form_type in [0x3a, 0x3b, 0x40, 0x69] {
            let (r, menu) = location_world(form_type, 1);
            assert_eq!(
                r.calls_to(REFR_GET_INTERIOR),
                vec![vec![0x44]],
                "{form_type:x}"
            );
            assert!(r.calls_to(LOCATION_FLAG).is_empty());
            assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(9), 0);
        }
    }

    #[test]
    fn other_form_types_leave_the_layout_alone() {
        for form_type in [0x38, 0x41, 0x68, 0x6a, 0] {
            let (r, menu) = location_world(form_type, 1);
            assert!(r.calls_to(REFR_GET_INTERIOR).is_empty(), "{form_type:x}");
            assert!(r.calls_to(LOCATION_FLAG).is_empty());
            assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(9), 0);
            // Exterior layout: tips on, slides set up.
            assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
            assert_eq!(r.calls_to(REFRESH_STATS).len(), 1);
            assert!(!r.calls_to(CANDIDATES_RESET).is_empty());
        }
    }

    #[test]
    fn the_skip_test_ignores_the_location_form_type() {
        let (mut r, menu) = impl_world();
        r.stub(SKIP_TEST, 1);
        r.e.mem.set_u32(SKIP_OWNER, 0x9600);
        r.stub(FORM_TYPE, 0x39);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert!(r.calls_to(FORM_TYPE).is_empty());
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(9), 0);
    }

    #[test]
    fn the_wheel_gets_its_scale_and_rotation() {
        let (mut r, menu) = impl_world();
        r.e.mem.set_f32(HALF_PI, 1.5);
        let model = r.object(0x100);
        r.stub(TILE_MODEL, model);
        r.stub(SETTING_VALUE_POINTER, 0x7020);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        let wheel = r.tile_at(menu, SLOT_WHEEL);
        assert!(r
            .calls_to(MODEL_SET_SCALE)
            .contains(&vec![model, 2.5f32.to_bits()]));
        assert!(r
            .calls_to(TILE_SET_VALUE)
            .contains(&vec![wheel, 0x1004, 3.25f32.to_bits(), 1]));
        assert!(r.calls_to(MODEL_SET_ROTATION).contains(&vec![
            model,
            1.5f32.to_bits(),
            1.0f32.to_bits(),
            0,
            0
        ]));
        assert!(r.calls_to(LOG_LINE).is_empty());
    }

    #[test]
    fn a_wheel_without_a_model_is_reported() {
        let (mut r, menu) = impl_world();
        let model = r.object(0x100);
        let wheel = r.tile_at(menu, SLOT_WHEEL);
        r.with(TILE_MODEL, move |_, a| {
            eax(if a[0] == wheel { 0 } else { model })
        });
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert_eq!(r.calls_to(LOG_LINE), vec![vec![0x12, WHEEL_WARNING]]);
        // The wheel was not scaled.
        assert!(!r
            .calls_to(MODEL_SET_SCALE)
            .iter()
            .any(|a| a[1] == 2.5f32.to_bits()));
    }

    #[test]
    fn slides_that_are_nodes_take_the_property_of_their_first_child() {
        let (mut r, menu) = impl_world();
        let names: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let seen = names.clone();
        r.with(FIXED_STRING_NEW, move |_, a| {
            seen.borrow_mut().push(a[1]);
            eax(a[0])
        });
        let slide = r.object(0x40);
        r.stub(VTABLE_TARGETS + 0x9c, slide);
        // The slide casts to a node.
        r.stub(VTABLE_TARGETS + 0x0c, 0x9500);
        r.stub(PROPERTY_ID, 3);
        r.stub(NODE_CHILD, 0x9600);
        r.with(NODE_PROPERTY, |_, a| eax(a[0] + a[1]));
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert_eq!(*names.borrow(), vec![SLIDE_ONE, SLIDE_TWO]);
        assert_eq!(
            r.calls_to(NODE_CHILD),
            vec![vec![0x9500, 0], vec![slide, 0]]
        );
        assert_eq!(r.e.get(menu, LoadingMenu::pSlide1TSP).addr(), 0x9603);
        assert_eq!(r.e.get(menu, LoadingMenu::pSlide2TSP).addr(), 0x9603);
        assert_eq!(r.calls_to(FIXED_STRING_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn slides_that_are_not_nodes_take_their_own_property() {
        let (mut r, menu) = impl_world();
        let first = r.object(0x40);
        let second = r.object(0x40);
        let answers = Rc::new(RefCell::new(vec![second, first]));
        r.with(VTABLE_TARGETS + 0x9c, move |_, _| {
            eax(answers.borrow_mut().pop().unwrap())
        });
        r.stub(VTABLE_TARGETS + 0x0c, 0);
        r.stub(PROPERTY_ID, 3);
        r.with(NODE_PROPERTY, |_, a| eax(a[0] + a[1]));
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert!(r.calls_to(NODE_CHILD).is_empty());
        assert_eq!(r.e.get(menu, LoadingMenu::pSlide1TSP).addr(), first + 3);
        assert_eq!(r.e.get(menu, LoadingMenu::pSlide2TSP).addr(), second + 3);
    }

    #[test]
    fn create_finishes_the_slide_setup() {
        let (mut r, menu) = impl_world();
        r.stub_float(REAL_SCREEN_WIDTH, 1280.0);
        r.stub_float(REAL_SCREEN_HEIGHT, 720.0);
        r.e.mem.set_f32(SLIDE_SCALE, 0.5);
        let model_tile = r.tile_at(menu, SLOT_MODEL);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        let ratio = (1280.0f64 / 720.0) as f32;
        assert!(r
            .calls_to(TILE_SET_VALUE)
            .contains(&vec![model_tile, 0x1004, ratio.to_bits(), 1]));
        assert_eq!(
            r.calls_to(TILE_PLAY_SEQUENCE),
            vec![vec![model_tile, SEQUENCE_INTRO]]
        );
        assert_eq!(r.calls_to(UPDATE_DATA_NEW).len(), 1);
        assert_eq!(r.calls_to(UPDATE_DATA_NEW)[0][1..], [0, 1, 0]);
        assert_eq!(r.calls_to(NODE_UPDATE).len(), 1);
        assert_eq!(r.calls_to(FADE_STEP), vec![vec![menu.addr()]]);
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 0);
        assert_eq!(r.calls_to(MENU_SHOW_CHANGES).len(), 1);
        assert_eq!(r.e.get(menu, LoadingMenu::m_TileCrossFadeStartTime), 0);
        // The model is shown last.
        assert_eq!(*r.order().last().unwrap(), MODEL_SHOW);
        // Not the initial load: the list is built, then the first slide.
        assert!(!r.calls_to(CANDIDATES_RESET).is_empty());
        assert_eq!(r.e.get(menu, LoadingMenu::m_SlideCount), 0);
    }

    #[test]
    fn create_starts_multi_threaded_audio_on_the_initial_load() {
        let (mut r, menu) = impl_world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.stub(AUDIO_GETTER, 0x5000);
        r.stub(AUDIO_IS_ACTIVE, 1);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert_eq!(r.calls_to(AUDIO_SET_MULTI_THREADED), vec![vec![0x5000, 1]]);
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(2), 0);
        // The initial-load path never mutes.
        assert!(r.calls_to(AUDIO_MUTE_TYPE).is_empty());
    }

    #[test]
    fn create_leaves_the_audio_alone_when_the_music_setting_is_set() {
        let (mut r, menu) = impl_world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.mem.set_u8(0x7000, b'x');
        r.stub(AUDIO_GETTER, 0x5000);
        r.stub(AUDIO_IS_ACTIVE, 1);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert!(r.calls_to(AUDIO_SET_MULTI_THREADED).is_empty());
        assert!(r.calls_to(AUDIO_MUTE_TYPE).is_empty());
    }

    #[test]
    fn create_mutes_audio_type_2_for_a_location_load() {
        let (mut r, menu) = impl_world();
        r.stub(AUDIO_GETTER, 0x5000);
        r.stub(AUDIO_IS_ACTIVE, 1);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert_eq!(r.calls_to(AUDIO_SET_MULTI_THREADED), vec![vec![0x5000, 1]]);
        assert_eq!(
            r.calls_to(AUDIO_MUTE_TYPE),
            vec![vec![0x5000, 2, 1000, 6000]]
        );
        // Already multi-threaded: it is not switched again, still muted.
        let (mut r, menu) = impl_world();
        r.stub(AUDIO_GETTER, 0x5000);
        r.stub(AUDIO_IS_ACTIVE, 1);
        r.stub(AUDIO_IS_MULTI_THREADED, 1);
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert!(r.calls_to(AUDIO_SET_MULTI_THREADED).is_empty());
        assert_eq!(r.calls_to(AUDIO_MUTE_TYPE).len(), 1);
        // Inactive audio: nothing.
        let (mut r, menu) = impl_world();
        loading_menu_create_impl(&mut r.e, menu, 0x44);
        assert!(r.calls_to(AUDIO_MUTE_TYPE).is_empty());
    }

    #[test]
    fn create_with_no_location_takes_the_first_audio_branch() {
        let (mut r, menu) = impl_world();
        r.stub(AUDIO_GETTER, 0x5000);
        r.stub(AUDIO_IS_ACTIVE, 1);
        loading_menu_create_impl(&mut r.e, menu, 0);
        assert_eq!(r.calls_to(AUDIO_SET_MULTI_THREADED), vec![vec![0x5000, 1]]);
        assert!(r.calls_to(AUDIO_MUTE_TYPE).is_empty());
    }

    // ----- 00789820 -----

    /// A world for `Update`: time 10000, real clamp and float truncation,
    /// and the fade constants (0.0005 per ms, 255, 15, 30, 1000).
    fn update_world() -> (Rig, Ptr<LoadingMenu>) {
        let (mut r, menu) = world();
        r.with(CLAMP_FLOAT, |e, a| {
            let value = e.mem.f32(a[0]);
            let clamped = value.clamp(f32::from_bits(a[1]), f32::from_bits(a[2]));
            e.mem.set_f32(a[0], clamped);
            eax(0)
        });
        r.with(FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            eax(value as i32 as u32)
        });
        r.put_f64(FADE_PER_MS, 0.0005);
        r.put_f64(BYTE_SCALE, 255.0);
        r.put_f64(MATTE_OFFSET_DOWN, 15.0);
        r.put_f64(MATTE_OFFSET_UP, 30.0);
        r.put_f64(MS_PER_SECOND, 1000.0);
        r.stub(AUDIO_GETTER, 0x5000);
        (r, menu)
    }

    /// Flags that keep `Update` out of the fade and swap code: bit 4 set, a
    /// swap time in the future.
    fn quiet(r: &mut Rig, menu: Ptr<LoadingMenu>) {
        r.e.mem.set_u16(menu.addr() + 0x222, bit(4));
        r.e.set(menu, LoadingMenu::uiNextBackgroundSwap, 50_000);
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 1);
    }

    #[test]
    fn update_copies_the_tip_values_to_the_matte_tiles() {
        let (mut r, menu) = update_world();
        quiet(&mut r, menu);
        r.with(TILE_GET_VALUE, |_, a| {
            st0(if a[1] == 0xfa2 { 100.0 } else { 10.0 })
        });
        r.e.set(menu, LoadingMenu::m_DelayedTipMatteUpdate, 3);
        loading_menu_update(&mut r.e, menu);
        let (tip_a, tip_b) = (r.tile_at(menu, 0xb0), r.tile_at(menu, 0xb4));
        let (matte_a, matte_b) = (r.tile_at(menu, 0xb8), r.tile_at(menu, 0xbc));
        let sets = r.calls_to(TILE_SET_VALUE);
        let bits = |v: f32| v.to_bits();
        assert_eq!(
            sets,
            vec![
                vec![matte_a, 0xfa2, bits(85.0), 1],
                vec![matte_a, 0xfb0, bits(40.0), 1],
                vec![matte_b, 0xfa2, bits(85.0), 1],
                vec![matte_b, 0xfb0, bits(40.0), 1],
            ]
        );
        assert_eq!(r.calls_to(TILE_GET_VALUE)[0][0], tip_a);
        assert_eq!(r.calls_to(TILE_GET_VALUE)[2][0], tip_b);
        assert_eq!(r.e.get(menu, LoadingMenu::m_DelayedTipMatteUpdate), 0);
    }

    #[test]
    fn update_copies_only_the_requested_matte() {
        let (mut r, menu) = update_world();
        quiet(&mut r, menu);
        r.e.set(menu, LoadingMenu::m_DelayedTipMatteUpdate, 2);
        loading_menu_update(&mut r.e, menu);
        let matte_b = r.tile_at(menu, 0xbc);
        let sets = r.calls_to(TILE_SET_VALUE);
        assert_eq!(sets.len(), 2);
        assert!(sets.iter().all(|a| a[0] == matte_b));
        assert_eq!(r.e.get(menu, LoadingMenu::m_DelayedTipMatteUpdate), 0);
    }

    #[test]
    fn update_cross_fades_by_the_elapsed_time() {
        let (mut r, menu) = update_world();
        // 500 ms at 0.0005 per ms: 0.25; an odd slide count does not flip.
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 9_500);
        r.e.set(menu, LoadingMenu::m_SlideCount, 1);
        r.e.mem.set_u8(DO_TILE_UPDATE, 0);
        loading_menu_update(&mut r.e, menu);
        let (fade_a, fade_b) = (r.tile_at(menu, SLOT_FADE_A), r.tile_at(menu, SLOT_FADE_B));
        assert_eq!(
            r.calls_to(TILE_SET_INT),
            vec![vec![fade_a, 0xfa9, 63], vec![fade_b, 0xfa9, 191]]
        );
        assert_eq!(r.e.mem.u8(DO_TILE_UPDATE), 1);
    }

    #[test]
    fn update_flips_the_fade_for_an_even_slide_count() {
        let (mut r, menu) = update_world();
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 9_500);
        r.e.set(menu, LoadingMenu::m_SlideCount, 2);
        loading_menu_update(&mut r.e, menu);
        let (fade_a, fade_b) = (r.tile_at(menu, SLOT_FADE_A), r.tile_at(menu, SLOT_FADE_B));
        assert_eq!(
            r.calls_to(TILE_SET_INT),
            vec![vec![fade_a, 0xfa9, 191], vec![fade_b, 0xfa9, 63]]
        );
    }

    #[test]
    fn update_clamps_the_fade_and_treats_a_zero_start_as_complete() {
        let (mut r, menu) = update_world();
        r.e.set(menu, LoadingMenu::m_SlideCount, 1);
        // Start 0: the fade is 1.0 whatever the clock says.
        loading_menu_update(&mut r.e, menu);
        let (fade_a, fade_b) = (r.tile_at(menu, SLOT_FADE_A), r.tile_at(menu, SLOT_FADE_B));
        assert_eq!(
            r.calls_to(TILE_SET_INT),
            vec![vec![fade_a, 0xfa9, 255], vec![fade_b, 0xfa9, 0]]
        );
        // Far in the past: clamped to 1.0 too.
        let (mut r, menu) = update_world();
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 1);
        r.e.set(menu, LoadingMenu::m_SlideCount, 1);
        loading_menu_update(&mut r.e, menu);
        let fade_a = r.tile_at(menu, SLOT_FADE_A);
        assert_eq!(r.calls_to(TILE_SET_INT)[0], vec![fade_a, 0xfa9, 255]);
    }

    #[test]
    fn update_skips_null_fade_tiles() {
        let (mut r, menu) = update_world();
        r.e.mem.set_u32(menu.addr() + SLOT_FADE_A, 0);
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 9_500);
        loading_menu_update(&mut r.e, menu);
        let fade_b = r.tile_at(menu, SLOT_FADE_B);
        let sets = r.calls_to(TILE_SET_INT);
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0][0], fade_b);
    }

    #[test]
    fn update_gives_the_current_and_next_screens_to_the_tip_routine() {
        let (mut r, menu) = update_world();
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 9_500);
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.e.set(menu, LoadingMenu::m_SlideCount, 1);
        // The next node is the list head here, so its item is the first
        // screen; the current screen is the first as well.
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        loading_menu_update(&mut r.e, menu);
        assert_eq!(
            r.calls_to(LOAD_SCREEN_TYPE_FOLDED),
            vec![vec![first], vec![first]]
        );
    }

    fn started_world() -> (Rig, Ptr<LoadingMenu>) {
        let (mut r, menu) = update_world();
        // 1 s into the fade, on the initial load.
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 9_000);
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.mem.set_u32(MOVIE_PLAYER, 0x9400);
        r.e.mem.set_f32(MUSIC_LEVEL, 0.5);
        r.e.mem.set_f32(ALPHA_FULL, 255.0);
        (r, menu)
    }

    #[test]
    fn update_starts_the_title_music_and_movie_on_the_initial_load() {
        let (mut r, menu) = started_world();
        r.stub(STRING_LENGTH, 5);
        loading_menu_update(&mut r.e, menu);
        assert_eq!(r.calls_to(MEMSET).len(), 1);
        let buffer = r.calls_to(MEMSET)[0][0];
        assert_eq!(r.calls_to(MEMSET)[0][1..], [0, 0x40]);
        assert_eq!(
            r.calls_to(SNPRINTF),
            vec![vec![buffer, 0x40, MUSIC_PATH_FORMAT, 0x7000]]
        );
        assert_eq!(r.calls_to(CREDITS_SET_FLAG), vec![vec![1]]);
        assert_eq!(r.calls_to(MUSIC_PLAY), vec![vec![8, buffer, 0, 1, 1, 0, 0]]);
        assert_eq!(r.calls_to(MUSIC_UPDATE).len(), 1);
        assert_eq!(r.e.mem.f32(MUSIC_LEVEL_A), 0.5);
        assert_eq!(r.e.mem.f32(MUSIC_LEVEL_B), 0.5);
        let (fade_a, fade_b) = (r.tile_at(menu, SLOT_FADE_A), r.tile_at(menu, SLOT_FADE_B));
        let sets = r.calls_to(TILE_SET_VALUE);
        assert!(sets.contains(&vec![fade_a, 0xfa9, 0, 1]));
        assert!(sets.contains(&vec![fade_b, 0xfa9, 255.0f32.to_bits(), 1]));
        assert_eq!(
            r.calls_to(START_SEQUENCE),
            vec![vec![0x9400, 0x7000, 0, 0xffff_ffff, 0, 0, 0]]
        );
        assert_eq!(r.e.mem.u8(MUSIC_STARTED), 1);
        let wheel = r.tile_at(menu, SLOT_WHEEL);
        assert!(r.calls_to(TILE_SET_INT).contains(&vec![wheel, 0xfa3, 1]));
        assert_eq!(r.calls_to(MODEL_SHOW).len(), 1);
    }

    #[test]
    fn update_starts_only_the_music_without_a_movie_path() {
        let (mut r, menu) = started_world();
        r.stub(STRING_LENGTH, 0);
        loading_menu_update(&mut r.e, menu);
        assert_eq!(r.calls_to(MUSIC_PLAY).len(), 1);
        assert!(r.calls_to(START_SEQUENCE).is_empty());
        assert_eq!(r.e.mem.u8(MUSIC_STARTED), 1);
        assert!(r.calls_to(MODEL_SHOW).is_empty());
    }

    #[test]
    fn update_starts_the_music_once_and_only_when_nothing_blocks_it() {
        // Already started.
        let (mut r, menu) = started_world();
        r.e.mem.set_u8(MUSIC_STARTED, 1);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(MUSIC_PLAY).is_empty());
        // A sequence is playing.
        let (mut r, menu) = started_world();
        r.stub(PLAYING_SEQUENCE, 1);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(MUSIC_PLAY).is_empty());
        // State bit 0 (single screen).
        let (mut r, menu) = started_world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(0));
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(MUSIC_PLAY).is_empty());
        // Not the initial load.
        let (mut r, menu) = started_world();
        r.e.mem.set_u8(INITIAL_LOAD, 0);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(MUSIC_PLAY).is_empty());
    }

    #[test]
    fn update_waits_for_the_loading_thread_and_shows_the_first_slide() {
        let (mut r, menu) = update_world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 1);
        r.e.set(menu, LoadingMenu::pSlide1TSP, Ptr::new(0x8888));
        let turns = Rc::new(RefCell::new(0));
        let counter = turns.clone();
        r.with(SLEEP_TICK, move |e, _| {
            *counter.borrow_mut() += 1;
            if *counter.borrow() == 3 {
                e.mem.set_u32(TEXTURE_OWNER, 0x9900);
            }
            eax(0)
        });
        loading_menu_update(&mut r.e, menu);
        assert_eq!(*turns.borrow(), 3);
        assert_eq!(r.calls_to(SLEEP_TICK), vec![vec![5], vec![5], vec![5]]);
        assert_eq!(r.calls_to(PROFILE_GUARD_NEW).len(), 1);
        // The fade-in time and the swap time both become now.
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 10_000);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 10_000);
    }

    #[test]
    fn update_sets_the_swap_time_without_waiting_when_not_the_initial_load() {
        let (mut r, menu) = update_world();
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 1);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(PROFILE_GUARD_NEW).is_empty());
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 10_000);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 10_000);
    }

    #[test]
    fn update_steps_the_fade_in_when_state_bit_4_is_set() {
        let (mut r, menu) = update_world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(4));
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 1);
        loading_menu_update(&mut r.e, menu);
        assert_eq!(r.calls_to(FADE_STEP), vec![vec![menu.addr()]]);
        // The fade-in time stays 0, and so does the swap time.
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 0);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 0);
        // With a swap time set, bit 4 steps the fade-in as well.
        let (mut r, menu) = update_world();
        quiet(&mut r, menu);
        loading_menu_update(&mut r.e, menu);
        assert_eq!(r.calls_to(FADE_STEP).len(), 1);
    }

    fn swap_world(extra_flags: u16) -> (Rig, Ptr<LoadingMenu>) {
        let (mut r, menu) = update_world();
        r.e.set(menu, LoadingMenu::m_TileCrossFadeStartTime, 1);
        r.e.set(menu, LoadingMenu::uiNextBackgroundSwap, 9_000);
        r.e.mem.set_u16(menu.addr() + 0x222, extra_flags);
        (r, menu)
    }

    /// Makes the model tile's sequence report state 1 (finished).
    fn sequence_done(r: &mut Rig, menu: Ptr<LoadingMenu>) {
        let tile = r.tile_at(menu, SLOT_MODEL);
        let sequence = r.object(0x60);
        r.e.mem.set_u32(tile + 0x3c, sequence);
        r.stub(SEQUENCE_STATE, 1);
    }

    #[test]
    fn update_does_not_swap_before_the_swap_time_or_for_an_interior() {
        let (mut r, menu) = swap_world(0);
        r.e.set(menu, LoadingMenu::uiNextBackgroundSwap, 10_001);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(TILE_PLAY_SEQUENCE).is_empty());
        let (mut r, menu) = swap_world(bit(9));
        sequence_done(&mut r, menu);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(PROFILE_GUARD_NEW).is_empty());
    }

    #[test]
    fn update_moves_to_the_next_screen_when_the_sequence_has_finished() {
        let (mut r, menu) = swap_world(0);
        sequence_done(&mut r, menu);
        let model_tile = r.tile_at(menu, SLOT_MODEL);
        r.stub(TILE_SEQUENCE, 0x11);
        r.stub(TILE_TARGET, 0x22);
        loading_menu_update(&mut r.e, menu);
        // The description is set (0078a280), the screen advanced (the
        // announcement), the texture loaded, the sequence call made, and the
        // next swap time computed (now + interval).
        assert!(!r.calls_to(TILE_SET_STRING).is_empty());
        assert_eq!(r.calls_to(ANNOUNCE_SCREEN).len(), 1);
        assert_eq!(r.calls_to(PROFILE_GUARD_NEW).len(), 1);
        assert_eq!(r.calls_to(TILE_SEQUENCE_CALL), vec![vec![0x22, 0x11, 0]]);
        assert_eq!(r.calls_to(TILE_SEQUENCE), vec![vec![model_tile]]);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 10_000);
    }

    #[test]
    fn update_skips_the_description_with_a_locked_value_or_xui() {
        for (value, xui) in [(1.0, 0), (0.0, 1)] {
            let (mut r, menu) = swap_world(0);
            sequence_done(&mut r, menu);
            r.stub_float(VALUE_GETTER, value);
            r.stub(XUI_IS_UP, xui);
            loading_menu_update(&mut r.e, menu);
            assert!(r.calls_to(TILE_SET_STRING).is_empty());
            assert_eq!(r.calls_to(PROFILE_GUARD_NEW).len(), 1);
        }
        // On the initial load it is skipped as well.
        let (mut r, menu) = swap_world(0);
        sequence_done(&mut r, menu);
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.set(
            menu,
            LoadingMenu::pNextLoadScreen,
            Ptr::new(menu.addr() + 0x1d0),
        );
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(TILE_SET_STRING).is_empty());
    }

    #[test]
    fn update_with_state_bit_3_sets_the_description_only_when_the_tip_tile_is_true() {
        for (tip_true, called) in [(0, false), (1, true)] {
            let (mut r, menu) = swap_world(bit(3));
            sequence_done(&mut r, menu);
            r.stub(TILE_IS_TRUE, tip_true);
            loading_menu_update(&mut r.e, menu);
            assert_eq!(!r.calls_to(TILE_SET_STRING).is_empty(), called);
            let tip = r.tile_at(menu, SLOT_TIP_A);
            assert_eq!(r.calls_to(TILE_IS_TRUE), vec![vec![tip, 0xfa3]]);
        }
    }

    #[test]
    fn update_sets_state_bit_0_on_the_last_screen_of_the_initial_load() {
        let (mut r, menu) = swap_world(0);
        sequence_done(&mut r, menu);
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        // Make the list a single node so that advancing wraps to the head.
        r.e.mem.set_u32(menu.addr() + 0x1d4, 0);
        loading_menu_update(&mut r.e, menu);
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(0), 0);
    }

    #[test]
    fn update_plays_the_next_slide_sequence() {
        let (mut r, menu) = swap_world(0);
        r.stub(TILE_PLAY_SEQUENCE, 1);
        let model_tile = r.tile_at(menu, SLOT_MODEL);
        r.e.mem.set_u16(menu.addr() + 0x222, bit(7));
        r.stub(LIST_COUNT, 1);
        loading_menu_update(&mut r.e, menu);
        // Bit 1 was toggled on, so the odd sequence plays.
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(1), 0);
        assert_eq!(
            r.calls_to(TILE_PLAY_SEQUENCE),
            vec![vec![model_tile, SEQUENCE_ODD]]
        );
        assert_eq!(r.e.get(menu, LoadingMenu::m_TileCrossFadeStartTime), 10_000);
        assert_eq!(r.e.get(menu, LoadingMenu::m_SlideCount), 1);
        // The first screen was pending (bit 7): with one screen, bit 8 is
        // set and bit 7 cleared.
        let flags = r.e.get(menu, LoadingMenu::usFlags);
        assert_ne!(flags & bit(8), 0);
        assert_eq!(flags & bit(7), 0);
        assert_eq!(r.calls_to(UPDATE_DATA_NEW).len(), 1);
        assert_eq!(r.calls_to(NODE_UPDATE).len(), 1);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 10_000);
    }

    #[test]
    fn update_keeps_bit_8_clear_when_there_are_several_screens() {
        let (mut r, menu) = swap_world(bit(7));
        r.stub(TILE_PLAY_SEQUENCE, 1);
        r.stub(LIST_COUNT, 2);
        loading_menu_update(&mut r.e, menu);
        let flags = r.e.get(menu, LoadingMenu::usFlags);
        assert_eq!(flags & bit(8), 0);
        assert_eq!(flags & bit(7), 0);
    }

    #[test]
    fn update_reports_a_slide_sequence_that_does_not_play() {
        let (mut r, menu) = swap_world(bit(1));
        loading_menu_update(&mut r.e, menu);
        // Bit 1 toggles off, so the even sequence is tried and reported.
        assert_eq!(r.calls_to(TILE_PLAY_SEQUENCE)[0][1], SEQUENCE_EVEN);
        assert_eq!(
            r.calls_to(LOG_LINE),
            vec![vec![SEQUENCE_WARNING, SEQUENCE_EVEN]]
        );
        assert_eq!(r.e.get(menu, LoadingMenu::m_SlideCount), 0);
    }

    #[test]
    fn update_does_not_start_a_slide_in_the_blocked_states() {
        // Initial load with bit 0, bit 3 and bit 8 each stop it.
        let (mut r, menu) = swap_world(bit(0));
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(TILE_PLAY_SEQUENCE).is_empty());
        for blocking in [bit(3), bit(8)] {
            let (mut r, menu) = swap_world(blocking);
            loading_menu_update(&mut r.e, menu);
            assert!(r.calls_to(TILE_PLAY_SEQUENCE).is_empty());
            assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 9_000);
        }
    }

    #[test]
    fn update_ends_by_refreshing_the_wheel_and_the_audio() {
        let (mut r, menu) = update_world();
        quiet(&mut r, menu);
        r.stub(TILE_IS_VISIBLE, 1);
        loading_menu_update(&mut r.e, menu);
        let wheel = r.tile_at(menu, SLOT_WHEEL);
        assert_eq!(
            r.calls_to(TILE_SHOW_FRAME),
            vec![vec![wheel, (-1.0f32).to_bits(), 0]]
        );
        assert_eq!(r.calls_to(AUDIO_FLUSH), vec![vec![0x5000, 1]]);
        let order = r.order();
        assert_eq!(*order.last().unwrap(), MUSIC_REFRESH);
        // A hidden wheel is left alone.
        let (mut r, menu) = update_world();
        quiet(&mut r, menu);
        loading_menu_update(&mut r.e, menu);
        assert!(r.calls_to(TILE_SHOW_FRAME).is_empty());
    }

    // ----- 0078a1c0, 0078a1f0, 0078a230, 0078a260 -----

    #[test]
    fn toggling_a_state_bit_flips_only_that_bit() {
        let mut r = rig();
        let menu = r.menu();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(1));
        fn_0078a1c0(&mut r.e, menu, 3);
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags), bit(1) | bit(3));
        r.e.call(0x0078_a1c0, &args![menu, 3u32]);
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags), bit(1));
    }

    #[test]
    fn a_tile_is_playing_when_its_sequence_state_is_1() {
        let mut r = rig();
        let tile = r.e.mem.alloc(0x100);
        assert!(!fn_0078a1f0(&mut r.e, tile));
        assert!(r.calls_to(SEQUENCE_STATE).is_empty());
        r.e.mem.set_u32(tile + 0x3c, 0x4000);
        r.stub(SEQUENCE_STATE, 2);
        assert!(!fn_0078a1f0(&mut r.e, tile));
        r.stub(SEQUENCE_STATE, 1);
        assert!(r.e.call(0x0078_a1f0, &args![tile]).bool());
        assert_eq!(r.calls_to(SEQUENCE_STATE).last().unwrap(), &vec![0x4000]);
    }

    #[test]
    fn the_wheel_shows_frame_minus_one() {
        let (mut r, menu) = world();
        fn_0078a230(&mut r.e, menu);
        let wheel = r.tile_at(menu, SLOT_WHEEL);
        assert_eq!(
            r.calls_to(TILE_SHOW_FRAME),
            vec![vec![wheel, (-1.0f32).to_bits(), 0]]
        );
    }

    #[test]
    fn a_setting_string_is_found_through_the_table() {
        let mut r = rig();
        r.e.mem.set_u32(SETTING_TABLE + 8, 0x7100);
        r.e.mem.set_u32(0x7100, 0x7200);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 1));
        assert_eq!(r.e.call(0x0078_a260, &args![2u32]).u32(), 0x7201);
        assert_eq!(r.calls_to(SETTING_STRING), vec![vec![0x7200]]);
    }

    // ----- 0078a280 -----

    #[test]
    fn the_description_goes_to_the_first_tip_tile_for_even_slides() {
        let (mut r, menu) = world();
        r.e.mem.set_u8(0x7100, b'a');
        r.e.mem.set_u8(0x7101, b'b');
        r.stub(FORM_NAME, 0x7100);
        r.stub(VTABLE_TARGETS + 0x10, 0x7300);
        fn_0078a280(&mut r.e, menu, 0);
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        assert_eq!(
            r.calls_to(VTABLE_TARGETS + 0x10),
            vec![vec![first + 0x24, 0, 0x4353_4544]]
        );
        let (tip, description) = (
            r.tile_at(menu, SLOT_TIP_A),
            r.tile_at(menu, SLOT_DESCRIPTION),
        );
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![
                vec![tip, 0xfc4, 0x7300, 1],
                vec![description, 0xfc4, 0x7000, 1]
            ]
        );
        assert_eq!(r.e.get(menu, LoadingMenu::m_DelayedTipMatteUpdate), 1);
        assert_eq!(r.e.mem.u8(DO_TILE_UPDATE), 1);
        // The name's characters are converted one by one.
        assert_eq!(
            r.calls_to(CHAR_CONVERT),
            vec![vec![b'a' as u32], vec![b'b' as u32]]
        );
        assert_eq!(r.calls_to(FORM_NAME), vec![vec![first + 0x18]]);
    }

    #[test]
    fn the_description_goes_to_the_second_tip_tile_for_odd_slides() {
        let (mut r, menu) = world();
        r.e.set(menu, LoadingMenu::m_SlideCount, 3);
        r.e.set(menu, LoadingMenu::m_DelayedTipMatteUpdate, 1);
        r.stub(VTABLE_TARGETS + 0x10, 0x7300);
        fn_0078a280(&mut r.e, menu, 0);
        let tip = r.tile_at(menu, SLOT_TIP_B);
        assert_eq!(r.calls_to(TILE_SET_STRING)[0], vec![tip, 0xfc4, 0x7300, 1]);
        assert_eq!(r.e.get(menu, LoadingMenu::m_DelayedTipMatteUpdate), 3);
        // An empty name converts nothing.
        assert!(r.calls_to(CHAR_CONVERT).is_empty());
    }

    // ----- 0078a3e0 -----

    #[test]
    fn a_texture_goes_into_the_ring_and_onto_the_fading_tile() {
        let (mut r, menu) = world();
        r.with(NI_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        r.with(TEXTURE_LOAD, |e, a| {
            e.mem.set_u32(a[2], 0x5555);
            eax(0)
        });
        r.stub(VTABLE_TARGETS + 0x20, 0x6666);
        r.e.set(menu, LoadingMenu::m_SlideCount, 1);
        r.e.set(menu, LoadingMenu::m_nHoldTextureIndex, 3);
        r.e.mem.set_u32(menu.addr() + 0x20c + 12, 0x4444);
        r.e.mem.set_u32(menu.addr() + 0x1b8, 0x7777);
        r.e.mem.set_u32(TEXTURE_OWNER, 0x9900);
        fn_0078a3e0(&mut r.e, menu, 0);
        let slot = menu.addr() + 0x20c + 12;
        assert_eq!(r.calls_to(REMOVE_TEXTURE), vec![vec![0x4444]]);
        assert_eq!(r.calls_to(NI_POINTER_SET), vec![vec![slot, 0x5555]]);
        assert_eq!(r.e.get(menu, LoadingMenu::m_nHoldTextureIndex), 0);
        // Odd slide count: the second fading tile gets the property.
        assert_eq!(
            r.calls_to(VTABLE_TARGETS + 0x20),
            vec![vec![r.tile_at(menu, SLOT_FADE_B)]]
        );
        assert_eq!(r.calls_to(TEXTURE_APPLY), vec![vec![0x6666, 0x5555]]);
        let load = &r.calls_to(TEXTURE_LOAD)[0];
        assert_eq!((load[0], load[1], load[3], load[4]), (0x9900, 0x7777, 0, 0));
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        assert_eq!(
            r.calls_to(STRING_FORMAT),
            vec![vec![
                menu.addr() + 0x1b8,
                TEXTURE_PATH_FORMAT,
                TEXTURES_DIRECTORY,
                0x7000
            ]]
        );
        assert_eq!(r.calls_to(FORM_NAME), vec![vec![first + 0x18]]);
        // The guard brackets the work; the temporary is destroyed.
        let order = r.order();
        assert_eq!(order[0], PROFILE_GUARD_NEW);
        assert_eq!(*order.last().unwrap(), PROFILE_GUARD_DESTRUCTOR);
        let guard = r.calls_to(PROFILE_GUARD_NEW)[0].clone();
        assert_eq!(guard[1..], [0xd, 1, SOURCE_FILE_NAME, 0x423]);
        assert_eq!(r.calls_to(NI_POINTER_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn the_ring_starts_over_and_empty_slots_are_not_released() {
        let (mut r, menu) = world();
        r.with(NI_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        r.e.set(menu, LoadingMenu::m_nHoldTextureIndex, 1);
        fn_0078a3e0(&mut r.e, menu, 0);
        assert_eq!(r.e.get(menu, LoadingMenu::m_nHoldTextureIndex), 2);
        assert!(r.calls_to(REMOVE_TEXTURE).is_empty());
        // Even slide count: the first fading tile is asked; a null property
        // is not applied.
        assert_eq!(
            r.calls_to(VTABLE_TARGETS + 0x20),
            vec![vec![r.tile_at(menu, SLOT_FADE_A)]]
        );
        assert!(r.calls_to(TEXTURE_APPLY).is_empty());
        assert_eq!(r.calls_to(NI_POINTER_SET)[0][0], menu.addr() + 0x20c + 4);
    }

    // ----- 0078a910, 0078a930 -----

    #[test]
    fn the_counter_helper_post_increments() {
        let mut r = rig();
        r.e.mem.set_u32(0x7100, 41);
        assert_eq!(r.e.call(0x0078_a910, &args![0x7100u32, 0u32]).u32(), 41);
        assert_eq!(r.e.mem.u32(0x7100), 42);
    }

    #[test]
    fn the_list_lock_byte_is_read() {
        let mut r = rig();
        assert!(!r.e.call(0x0078_a930, &args![]).bool());
        r.e.mem.set_u8(LIST_LOCKED, 1);
        assert!(r.e.call(0x0078_a930, &args![]).bool());
    }

    // ----- 0078a940, 0078a9f0, 0078aa90 -----

    #[test]
    fn clearing_the_list_deletes_only_temporary_screens() {
        let (mut r, menu) = world();
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        r.with(IS_TEMPORARY, move |_, a| eax((a[0] == first) as u32));
        r.e.mem.set_u32(menu.addr() + 0x1d8, 0x1234);
        fn_0078a940(&mut r.e, menu);
        // Virtual slot 0x10 with 1 on the first screen only.
        assert_eq!(r.calls_to(VTABLE_TARGETS + 0x10), vec![vec![first, 1]]);
        assert_eq!(r.calls_to(LIST_FREE_NODES), vec![vec![menu.addr() + 0x1d0]]);
        assert_eq!(
            r.e.get(menu, LoadingMenu::pNextLoadScreen).addr(),
            menu.addr() + 0x1d0
        );
        assert_eq!(r.calls_to(IS_TEMPORARY).len(), 2);
    }

    #[test]
    fn clearing_the_list_stops_at_an_empty_node() {
        let (mut r, menu) = world();
        r.e.mem.set_u32(menu.addr() + 0x1d0, 0);
        r.stub(IS_TEMPORARY, 1);
        fn_0078a940(&mut r.e, menu);
        assert!(r.calls_to(IS_TEMPORARY).is_empty());
        assert_eq!(r.calls_to(LIST_FREE_NODES).len(), 1);
    }

    #[test]
    fn advancing_walks_the_list_and_wraps_to_the_head() {
        let (mut r, menu) = world();
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        let tail = r.e.mem.u32(menu.addr() + 0x1d4);
        let second = r.e.mem.u32(tail);
        fn_0078a9f0(&mut r.e, menu);
        assert_eq!(r.e.get(menu, LoadingMenu::pCurrentLoadScreen).addr(), first);
        assert_eq!(r.e.get(menu, LoadingMenu::pNextLoadScreen).addr(), tail);
        assert_eq!(r.calls_to(ANNOUNCE_SCREEN), vec![vec![menu.addr(), first]]);
        fn_0078a9f0(&mut r.e, menu);
        assert_eq!(
            r.e.get(menu, LoadingMenu::pCurrentLoadScreen).addr(),
            second
        );
        assert_eq!(
            r.e.get(menu, LoadingMenu::pNextLoadScreen).addr(),
            menu.addr() + 0x1d0
        );
        assert_eq!(r.calls_to(ANNOUNCE_SCREEN).len(), 2);
        assert_eq!(r.calls_to(ANNOUNCE_SCREEN)[1], vec![menu.addr(), second]);
    }

    #[test]
    fn advancing_onto_an_empty_node_announces_nothing() {
        let (mut r, menu) = world();
        let tail = r.e.mem.u32(menu.addr() + 0x1d4);
        r.e.mem.set_u32(tail, 0);
        fn_0078a9f0(&mut r.e, menu);
        assert!(r.calls_to(ANNOUNCE_SCREEN).is_empty());
    }

    #[test]
    fn jumping_to_the_last_screen_needs_the_initial_load_and_a_stopped_sequence() {
        // Not the initial load.
        let (mut r, menu) = world();
        assert!(!fn_0078aa90(&mut r.e, menu));
        // Initial load but bit 0 set.
        let (mut r, menu) = world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.mem.set_u16(menu.addr() + 0x222, bit(0));
        assert!(!fn_0078aa90(&mut r.e, menu));
        // Initial load but the sequence is running.
        let (mut r, menu) = world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        let tile = r.tile_at(menu, SLOT_MODEL);
        r.e.mem.set_u32(tile + 0x3c, 0x4000);
        r.stub(SEQUENCE_STATE, 1);
        assert!(!fn_0078aa90(&mut r.e, menu));
        assert!(r.calls_to(PROFILE_GUARD_NEW).is_empty());
    }

    #[test]
    fn jumping_to_the_last_screen_shows_it() {
        let (mut r, menu) = world();
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        assert!(r.e.call(0x0078_aa90, &args![menu]).bool());
        let tail = r.e.mem.u32(menu.addr() + 0x1d4);
        assert_eq!(r.e.get(menu, LoadingMenu::pNextLoadScreen).addr(), tail);
        assert_eq!(r.calls_to(PROFILE_GUARD_NEW).len(), 1);
        // The interval is 0 here: the swap time is now.
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 10_000);
    }

    // ----- 0078ab20 -----

    /// Name pointers by screen: the screens in `LoadScreens` are named "one"
    /// and "two"; the double compares C strings like `strcmp`.
    fn naming_world() -> (Rig, Ptr<LoadingMenu>, u32, u32) {
        let (mut r, menu) = world();
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        let tail = r.e.mem.u32(menu.addr() + 0x1d4);
        let second = r.e.mem.u32(tail);
        for (screen, text, at) in [(first, b"one\0", 0x7400u32), (second, b"two\0", 0x7410)] {
            r.e.mem.write(at, text);
            r.e.mem.set_u32(screen + 0x18, at);
        }
        r.with(FORM_NAME, |e, a| eax(e.mem.u32(a[0])));
        r.with(NAME_COMPARE, |e, a| {
            eax((e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32)
        });
        r.stub(FORM_TYPE, 0x4b);
        (r, menu, first, second)
    }

    fn candidate(r: &mut Rig, name: &[u8], at: u32) -> u32 {
        let screen = r.screen();
        r.e.mem.write(at, name);
        r.e.mem.set_u32(screen + 0x18, at);
        screen
    }

    #[test]
    fn a_new_screen_is_accepted_when_no_listed_screen_has_its_name() {
        let (mut r, menu, _, _) = naming_world();
        let screen = candidate(&mut r, b"three\0", 0x7420);
        assert!(fn_0078ab20(&mut r.e, menu, screen));
        assert_eq!(r.calls_to(NAME_COMPARE).len(), 2);
    }

    #[test]
    fn a_screen_with_a_listed_name_is_refused_at_the_first_match() {
        let (mut r, menu, _, _) = naming_world();
        let screen = candidate(&mut r, b"one\0", 0x7420);
        assert!(!fn_0078ab20(&mut r.e, menu, screen));
        assert_eq!(r.calls_to(NAME_COMPARE).len(), 1);
        // A match on the second one stops there.
        let screen = candidate(&mut r, b"two\0", 0x7430);
        r.clear_calls();
        assert!(!fn_0078ab20(&mut r.e, menu, screen));
        assert_eq!(r.calls_to(NAME_COMPARE).len(), 2);
    }

    #[test]
    fn only_forms_of_type_0x4b_are_accepted() {
        let (mut r, menu, _, _) = naming_world();
        let screen = candidate(&mut r, b"three\0", 0x7420);
        r.stub(FORM_TYPE, 0x4a);
        assert!(!fn_0078ab20(&mut r.e, menu, screen));
        assert!(r.calls_to(NAME_COMPARE).is_empty());
        assert!(!fn_0078ab20(&mut r.e, menu, 0));
        assert_eq!(r.calls_to(FORM_TYPE).len(), 1);
    }

    // ----- 0078abd0 -----

    fn interval_world() -> (Rig, Ptr<LoadingMenu>) {
        let (mut r, menu) = world();
        r.put_f64(MS_PER_SECOND, 1000.0);
        r.with(SETTING_FLOAT, |_, a| {
            st0(match a[0] {
                SETTING_INTERVAL_XUI => 1.5,
                SETTING_INTERVAL_INITIAL => 2.5,
                SETTING_INTERVAL_DEFAULT => 4.0,
                other => panic!("unexpected setting {other:x}"),
            })
        });
        (r, menu)
    }

    #[test]
    fn the_swap_interval_depends_on_the_situation() {
        let (mut r, menu) = interval_world();
        fn_0078abd0(&mut r.e, menu);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 14_000);
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        fn_0078abd0(&mut r.e, menu);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 12_500);
        r.stub(XUI_IS_UP, 1);
        r.e.call(0x0078_abd0, &args![menu]);
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 11_500);
    }

    #[test]
    fn a_negative_interval_moves_the_swap_time_back() {
        let (mut r, menu) = world();
        r.put_f64(MS_PER_SECOND, 1000.0);
        r.stub_float(SETTING_FLOAT, -1.25);
        fn_0078abd0(&mut r.e, menu);
        // The product truncates toward zero.
        assert_eq!(r.e.get(menu, LoadingMenu::uiNextBackgroundSwap), 8_750);
    }

    // ----- 0078a5a0 -----

    struct ListRig {
        r: Rig,
        menu: Ptr<LoadingMenu>,
        /// The `TESLoadScreen`s `operator new` handed out.
        made: Rc<RefCell<Vec<u32>>>,
        /// The items added through `LIST_ADD` / `LIST_PUSH` (read from the
        /// word whose address is passed).
        added: Rc<RefCell<Vec<u32>>>,
    }

    /// A rig for `MakeLoadScreenList`: `LIST_EMPTY` answers from `empty` (in
    /// order, then 0), `operator new` allocates, the constructor returns its
    /// argument and the string buffer holds the path 0x7777.
    fn list_rig(empty: Vec<u32>) -> ListRig {
        let (mut r, menu) = world();
        let made = Rc::new(RefCell::new(Vec::new()));
        let added = Rc::new(RefCell::new(Vec::new()));
        let answers = Rc::new(RefCell::new(empty));
        r.with(LIST_EMPTY, move |_, _| {
            let mut answers = answers.borrow_mut();
            eax(if answers.is_empty() {
                0
            } else {
                answers.remove(0)
            })
        });
        let record = made.clone();
        r.with(OPERATOR_NEW, move |e, a| {
            let block = e.mem.alloc(a[0]);
            record.borrow_mut().push(block);
            eax(block)
        });
        r.with(LOAD_SCREEN_NEW, |_, a| eax(a[0]));
        for add in [LIST_ADD, LIST_PUSH] {
            let record = added.clone();
            r.with(add, move |e, a| {
                record.borrow_mut().push(e.mem.u32(a[1]));
                eax(0)
            });
        }
        r.with(NI_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        r.with(SETTING_STRING, |_, a| eax(a[0]));
        r.e.mem.set_u32(menu.addr() + 0x1b8, 0x7777);
        ListRig {
            r,
            menu,
            made,
            added,
        }
    }

    #[test]
    fn the_initial_load_builds_one_screen_per_setting() {
        let mut l = list_rig(vec![1, 0]);
        l.r.e.mem.set_u8(INITIAL_LOAD, 1);
        l.r.e.mem.set_u16(l.menu.addr() + 0x222, bit(6));
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        let menu = l.menu.addr();
        let formats = l.r.calls_to(STRING_FORMAT);
        assert_eq!(formats.len(), 5);
        for (i, call) in formats.iter().enumerate() {
            let directory = if i == 4 {
                MAIN_DIRECTORY
            } else {
                LOADING_DIRECTORY
            };
            assert_eq!(
                call,
                &vec![
                    menu + 0x1b8,
                    SCREEN_PATH_FORMAT,
                    directory,
                    SETTING_FIRST_SCREEN + i as u32 * 12
                ]
            );
        }
        let made = l.made.borrow().clone();
        assert_eq!(made.len(), 5);
        assert_eq!(*l.added.borrow(), made);
        assert_eq!(l.r.calls_to(LIST_ADD).len(), 5);
        assert_eq!(l.r.calls_to(LOAD_SCREEN_NEW).len(), 5);
        for screen in &made {
            assert!(l
                .r
                .calls_to(SET_PATH)
                .contains(&vec![screen + 0x18, 0x7777]));
            assert!(l.r.calls_to(SET_TEMPORARY).contains(&vec![*screen]));
        }
        assert!(l.r.calls_to(CANDIDATES_RESET).is_empty());
        // Empty list: nothing was cleared and no default screen was added.
        assert!(l.r.calls_to(LIST_FREE_NODES).is_empty());
        assert!(l.r.calls_to(LIST_PUSH).is_empty());
        // Bookkeeping at the end: bit 7, the first screen is current, the
        // face calculation, and the tips shown for that screen.
        let flags = l.r.e.get(l.menu, LoadingMenu::usFlags);
        assert_ne!(flags & bit(7), 0);
        assert_eq!(flags & bit(5), 0);
        let first = l.r.e.mem.u32(menu + 0x1d0);
        assert_eq!(
            l.r.e.get(l.menu, LoadingMenu::pCurrentLoadScreen).addr(),
            first
        );
        assert_eq!(l.r.calls_to(CALC_FACES), vec![vec![menu]]);
        assert_eq!(l.r.calls_to(GROUP_VISIBLE_0078B6A0), vec![vec![menu, 0]]);
        assert_eq!(l.r.calls_to(LOAD_SCREEN_TYPE), vec![vec![first]]);
    }

    #[test]
    fn a_music_path_makes_the_initial_load_start_at_the_third_setting() {
        let mut l = list_rig(vec![1, 0]);
        l.r.e.mem.set_u8(INITIAL_LOAD, 1);
        l.r.stub(STRING_LENGTH, 3);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        let formats = l.r.calls_to(STRING_FORMAT);
        assert_eq!(formats.len(), 2);
        assert_eq!(
            formats[0][2..],
            [LOADING_DIRECTORY, SETTING_FIRST_SCREEN + 36]
        );
        assert_eq!(formats[1][2..], [MAIN_DIRECTORY, SETTING_FIRST_SCREEN + 48]);
    }

    #[test]
    fn a_non_empty_list_is_cleared_and_the_slide_reloaded() {
        let mut l = list_rig(vec![0, 0]);
        l.r.e.mem.set_u8(INITIAL_LOAD, 1);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        assert_eq!(l.r.calls_to(LIST_FREE_NODES).len(), 1);
        assert_eq!(l.r.calls_to(PROFILE_GUARD_NEW).len(), 1);
        // A running model sequence is not interrupted.
        let mut l = list_rig(vec![0, 0]);
        l.r.e.mem.set_u8(INITIAL_LOAD, 1);
        let tile = l.r.tile_at(l.menu, SLOT_MODEL);
        l.r.e.mem.set_u32(tile + 0x3c, 0x4000);
        l.r.stub(SEQUENCE_STATE, 1);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        assert_eq!(l.r.calls_to(LIST_FREE_NODES).len(), 1);
        assert!(l.r.calls_to(PROFILE_GUARD_NEW).is_empty());
    }

    #[test]
    fn overlay_mode_sets_bit_5_and_collects_candidates() {
        let mut l = list_rig(vec![1, 0]);
        l.r.e.mem.set_u8(INITIAL_LOAD, 1);
        l.r.stub(XUI_IS_UP, 1);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        assert_ne!(l.r.e.get(l.menu, LoadingMenu::usFlags) & bit(5), 0);
        assert_eq!(l.r.calls_to(CANDIDATES_RESET).len(), 1);
        assert!(l.r.calls_to(STRING_FORMAT).is_empty());
    }

    #[test]
    fn the_overlay_owner_also_sets_bit_5() {
        let mut l = list_rig(vec![1, 0]);
        l.r.stub(OVERLAY_TEST, 1);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        assert_ne!(l.r.e.get(l.menu, LoadingMenu::usFlags) & bit(5), 0);
    }

    #[test]
    fn candidates_are_collected_into_a_temporary_list() {
        let mut l = list_rig(vec![1, 0]);
        l.r.e.set(l.menu, LoadingMenu::pLocation, Ptr::new(0x55));
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        let created = l.r.calls_to(CANDIDATE_LIST_NEW);
        let destroyed = l.r.calls_to(CANDIDATE_LIST_DESTRUCTOR);
        assert_eq!(created.len(), 1);
        assert_eq!(created, destroyed);
        let list = created[0][0];
        assert_eq!(l.r.calls_to(CANDIDATES_RESET), vec![vec![list, 0]]);
        // The limit setting is 0, so there is no second pass.
        assert_eq!(l.r.e.get(l.menu, LoadingMenu::pLocation).addr(), 0x55);
    }

    #[test]
    fn too_few_screens_for_a_location_trigger_a_second_pass_without_it() {
        let mut l = list_rig(vec![1, 0]);
        l.r.e.set(l.menu, LoadingMenu::pLocation, Ptr::new(0x55));
        l.r.e.mem.set_u32(0x7010, 3);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        assert_eq!(l.r.calls_to(CANDIDATES_RESET).len(), 2);
        assert_eq!(l.r.e.get(l.menu, LoadingMenu::pLocation).addr(), 0);
        // A locked list is not rebuilt.
        let mut l = list_rig(vec![1, 0]);
        l.r.e.set(l.menu, LoadingMenu::pLocation, Ptr::new(0x55));
        l.r.e.mem.set_u32(0x7010, 3);
        l.r.e.mem.set_u8(LIST_LOCKED, 1);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        assert_eq!(l.r.calls_to(CANDIDATES_RESET).len(), 1);
        assert_eq!(l.r.e.get(l.menu, LoadingMenu::pLocation).addr(), 0x55);
        // Without a location there is nothing to drop.
        let mut l = list_rig(vec![1, 0]);
        l.r.e.mem.set_u32(0x7010, 3);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        assert_eq!(l.r.calls_to(CANDIDATES_RESET).len(), 1);
    }

    #[test]
    fn an_empty_list_gets_one_default_screen() {
        let mut l = list_rig(vec![1, 1]);
        loading_menu_make_load_screen_list(&mut l.r.e, l.menu);
        let menu = l.menu.addr();
        assert_eq!(
            l.r.calls_to(STRING_FORMAT),
            vec![vec![
                menu + 0x1b8,
                LOADING_SCREEN_PATH_FORMAT,
                SETTING_FIRST_SCREEN
            ]]
        );
        let made = l.made.borrow().clone();
        assert_eq!(made.len(), 1);
        assert_eq!(*l.added.borrow(), made);
        assert_eq!(l.r.calls_to(LIST_PUSH).len(), 1);
        assert_eq!(l.r.calls_to(LIST_PUSH)[0][0], menu + 0x1d0);
        assert!(l.r.calls_to(LIST_ADD).is_empty());
    }

    // ----- 0078ac60, 0078ada0 -----

    struct Candidates {
        r: Rig,
        menu: Ptr<LoadingMenu>,
        list: u32,
        screens: Vec<u32>,
        nodes: Vec<u32>,
    }

    /// The global list holds three screens; the candidate list is a block.
    fn candidates() -> Candidates {
        let (mut r, menu) = world();
        let screens: Vec<u32> = (0..3).map(|_| r.screen()).collect();
        let c = r.node(screens[2], 0);
        let b = r.node(screens[1], c);
        let a = r.node(screens[0], b);
        r.stub(ALL_SCREENS_LIST, a);
        r.e.mem.set_u32(LOAD_SCREEN_OWNER, 0x9800);
        let list = r.e.mem.alloc(0x10);
        Candidates {
            r,
            menu,
            list,
            screens,
            nodes: vec![a, b, c],
        }
    }

    fn adds(c: &Candidates) -> Vec<u32> {
        c.r.calls_to(CANDIDATE_ADD).iter().map(|a| a[1]).collect()
    }

    #[test]
    fn candidates_without_a_location_are_the_generic_screens() {
        let mut c = candidates();
        let wanted = [c.screens[0], c.screens[2]];
        c.r.with(LOAD_SCREEN_GENERIC, move |_, a| {
            eax(wanted.contains(&a[0]) as u32)
        });
        loading_menu_build_candidate_list(&mut c.r.e, c.menu, c.list);
        assert_eq!(c.r.calls_to(CANDIDATES_RESET), vec![vec![c.list, 0]]);
        assert_eq!(adds(&c), vec![c.nodes[0], c.nodes[2]]);
        assert_eq!(c.r.calls_to(ALL_SCREENS_LIST), vec![vec![0x9800]]);
        assert!(c.r.calls_to(LOAD_SCREEN_FOR_LOCATION).is_empty());
        assert!(c.r.calls_to(CANDIDATE_REMOVE).is_empty());
    }

    #[test]
    fn candidates_for_a_location_are_asked_about_it() {
        let mut c = candidates();
        c.r.e.set(c.menu, LoadingMenu::pLocation, Ptr::new(0x55));
        let wanted = c.screens[1];
        c.r.with(LOAD_SCREEN_FOR_LOCATION, move |_, a| {
            eax((a[0] == wanted) as u32)
        });
        loading_menu_build_candidate_list(&mut c.r.e, c.menu, c.list);
        assert_eq!(adds(&c), vec![c.nodes[1]]);
        assert_eq!(c.r.calls_to(LOAD_SCREEN_FOR_LOCATION)[0][1], 0x55);
        assert!(c.r.calls_to(LOAD_SCREEN_GENERIC).is_empty());
    }

    #[test]
    fn overlay_candidates_are_the_screens_with_flag_0x400() {
        let mut c = candidates();
        c.r.e.mem.set_u16(c.menu.addr() + 0x222, bit(5));
        let flagged = c.screens[1];
        c.r.with(WORD_AT_8, move |_, a| {
            eax(if a[0] == flagged { 0x400 } else { 0x3ff })
        });
        loading_menu_build_candidate_list(&mut c.r.e, c.menu, c.list);
        assert_eq!(adds(&c), vec![c.nodes[1]]);
        assert!(c.r.calls_to(LOAD_SCREEN_GENERIC).is_empty());
    }

    #[test]
    fn candidates_of_other_types_are_dropped_again_while_a_menu_is_replaced() {
        for replaced in [0, 1] {
            let mut c = candidates();
            if replaced == 1 {
                c.r.e.mem.set_u8(MENU_REPLACED, 1);
            } else {
                c.r.stub(OVERLAY_TEST, 1);
            }
            c.r.stub(LOAD_SCREEN_GENERIC, 1);
            c.r.stub(CANDIDATE_ADD, 7);
            let types = [(c.screens[0], 2), (c.screens[1], 3), (c.screens[2], 0)];
            c.r.with(LOAD_SCREEN_TYPE, move |_, a| {
                eax(types.iter().find(|t| t.0 == a[0]).unwrap().1)
            });
            loading_menu_build_candidate_list(&mut c.r.e, c.menu, c.list);
            assert_eq!(adds(&c).len(), 3);
            assert_eq!(c.r.calls_to(CANDIDATE_REMOVE), vec![vec![c.list, 7, 1]]);
        }
        // Without a replaced menu or overlay nothing is removed.
        let mut c = candidates();
        c.r.stub(LOAD_SCREEN_GENERIC, 1);
        c.r.stub(LOAD_SCREEN_TYPE, 2);
        loading_menu_build_candidate_list(&mut c.r.e, c.menu, c.list);
        assert!(c.r.calls_to(CANDIDATE_REMOVE).is_empty());
    }

    #[test]
    fn the_candidate_walk_stops_at_an_empty_node() {
        let mut c = candidates();
        c.r.stub(LOAD_SCREEN_GENERIC, 1);
        c.r.e.mem.set_u32(c.nodes[1], 0);
        loading_menu_build_candidate_list(&mut c.r.e, c.menu, c.list);
        assert_eq!(adds(&c), vec![c.nodes[0]]);
    }

    #[test]
    fn bit_0x400_of_the_flags_word_marks_an_overlay_screen() {
        let mut r = rig();
        r.stub(WORD_AT_8, 0x400);
        assert!(r.e.call(0x0078_ada0, &args![0x77u32]).bool());
        r.stub(WORD_AT_8, 0xbff);
        assert!(!r.e.call(0x0078_ada0, &args![0x77u32]).bool());
        assert_eq!(r.calls_to(WORD_AT_8)[0], vec![0x77]);
    }

    // ----- 0078adc0 -----

    struct Picking {
        r: Rig,
        menu: Ptr<LoadingMenu>,
        list: u32,
        /// Screens added to `LoadScreens`.
        added: Rc<RefCell<Vec<u32>>>,
    }

    /// A candidate list of `n` screens (the size at `+8`, the array in a
    /// block). Random choice always picks index 0; removal shifts the array
    /// and shrinks the size. Settings: 5 screens, 2 in overlay mode, 1 per
    /// location. Every screen has type `0x4b` and a name of its own.
    fn picking(n: u32) -> Picking {
        let (mut r, menu) = world();
        let list = r.e.mem.alloc(0x10);
        let array = r.e.mem.alloc(4 * n.max(1));
        r.e.mem.set_u32(list + 8, n);
        for i in 0..n {
            let screen = r.screen();
            r.e.mem.write(0x7500 + i * 8, b"s\0");
            r.e.mem.set_u8(0x7501 + i * 8, b'a' + i as u8);
            r.e.mem.set_u32(screen + 0x18, 0x7500 + i * 8);
            r.e.mem.set_u32(array + 4 * i, screen);
        }
        // The screens already listed need names too.
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        let tail = r.e.mem.u32(menu.addr() + 0x1d4);
        let second = r.e.mem.u32(tail);
        for (screen, at) in [(first, 0x7600u32), (second, 0x7610)] {
            r.e.mem.write(at, b"x\0");
            r.e.mem.set_u32(screen + 0x18, at);
        }
        r.with(WORD_AT_8, |e, a| eax(e.mem.u32(a[0] + 8)));
        r.stub(RANDOM_INDEX, 0);
        r.with(CANDIDATE_AT, move |_, a| eax(array + 4 * a[1]));
        r.with(CANDIDATE_REMOVE, move |e, a| {
            let size = e.mem.u32(a[0] + 8);
            for i in a[1]..size - 1 {
                let next = e.mem.u32(array + 4 * (i + 1));
                e.mem.set_u32(array + 4 * i, next);
            }
            e.mem.set_u32(a[0] + 8, size - 1);
            eax(0)
        });
        r.with(FORM_NAME, |e, a| eax(e.mem.u32(a[0])));
        r.with(NAME_COMPARE, |e, a| {
            eax((e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32)
        });
        r.stub(FORM_TYPE, 0x4b);
        r.with(MINIMUM, |_, a| eax(a[0].min(a[1])));
        r.with(SETTING_INT_POINTER, |e, a| {
            let value = match a[0] {
                SETTING_MAX_SCREENS => 5,
                SETTING_MAX_SCREENS_OVERLAY => 2,
                SETTING_LOCATION_SCREENS => 1,
                other => panic!("unexpected setting {other:x}"),
            };
            e.mem.set_u32(0x7010, value);
            eax(0x7010)
        });
        let added = Rc::new(RefCell::new(Vec::new()));
        let record = added.clone();
        r.with(LIST_ADD, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[1]));
            eax(0)
        });
        let record = added.clone();
        r.with(LIST_COUNT, move |_, _| eax(record.borrow().len() as u32));
        Picking {
            r,
            menu,
            list,
            added,
        }
    }

    #[test]
    fn picking_moves_candidates_until_the_screen_limit() {
        let mut p = picking(8);
        fn_0078adc0(&mut p.r.e, p.menu, p.list);
        assert_eq!(p.added.borrow().len(), 5);
        // Every pick removed its candidate; the rest stay.
        assert_eq!(p.r.calls_to(CANDIDATE_REMOVE).len(), 5);
        assert_eq!(p.r.e.mem.u32(p.list + 8), 3);
    }

    #[test]
    fn picking_stops_when_the_candidates_run_out() {
        let mut p = picking(2);
        fn_0078adc0(&mut p.r.e, p.menu, p.list);
        assert_eq!(p.added.borrow().len(), 2);
        assert_eq!(p.r.e.mem.u32(p.list + 8), 0);
    }

    #[test]
    fn picking_uses_the_overlay_limit_in_overlay_mode() {
        let mut p = picking(8);
        p.r.e.mem.set_u16(p.menu.addr() + 0x222, bit(5));
        fn_0078adc0(&mut p.r.e, p.menu, p.list);
        assert_eq!(p.added.borrow().len(), 2);
    }

    #[test]
    fn picking_for_a_location_uses_the_smaller_limit_unless_locked() {
        let mut p = picking(8);
        p.r.e.set(p.menu, LoadingMenu::pLocation, Ptr::new(0x55));
        fn_0078adc0(&mut p.r.e, p.menu, p.list);
        assert_eq!(p.added.borrow().len(), 1);
        assert_eq!(p.r.calls_to(MINIMUM), vec![vec![5, 1]]);
        let mut p = picking(8);
        p.r.e.set(p.menu, LoadingMenu::pLocation, Ptr::new(0x55));
        p.r.e.mem.set_u8(LIST_LOCKED, 1);
        fn_0078adc0(&mut p.r.e, p.menu, p.list);
        assert_eq!(p.added.borrow().len(), 5);
        assert!(p.r.calls_to(MINIMUM).is_empty());
    }

    #[test]
    fn picking_drops_candidates_the_list_already_has() {
        let mut p = picking(3);
        // Candidates of another type are removed from the list but not added.
        p.r.stub(FORM_TYPE, 0x4a);
        fn_0078adc0(&mut p.r.e, p.menu, p.list);
        assert!(p.added.borrow().is_empty());
        assert_eq!(p.r.calls_to(CANDIDATE_REMOVE).len(), 3);
        assert_eq!(p.r.e.mem.u32(p.list + 8), 0);
    }

    #[test]
    fn picking_does_nothing_when_the_list_is_already_full() {
        let mut p = picking(3);
        p.r.stub(LIST_COUNT, 5);
        fn_0078adc0(&mut p.r.e, p.menu, p.list);
        assert!(p.r.calls_to(CANDIDATE_REMOVE).is_empty());
        assert!(p.added.borrow().is_empty());
    }

    // ----- 0078aec0 -----

    #[test]
    fn the_status_tiles_are_set_from_the_globals_and_settings() {
        let (mut r, menu) = world();
        r.e.mem.set_f32(TIP_SCALE, 0.75);
        r.e.mem.set_u32(TIP_VALUE, 0x1234);
        r.e.mem.set_u32(TIP_TEXT_OWNER, 0x9a00);
        r.stub(TEXT_OWNER_NAME, 0x7500);
        r.with(SETTING_STRING, |_, a| eax(a[0] + 1));
        fn_0078aec0(&mut r.e, menu);
        assert_eq!(r.calls_to(REFRESH_STATS).len(), 1);
        let scaled = r.tile_at(menu, SLOT_SCALED);
        assert_eq!(
            r.calls_to(TILE_SET_VALUE),
            vec![vec![scaled, 0x1004, 0.75f32.to_bits(), 1]]
        );
        assert_eq!(
            r.calls_to(TILE_SET_ID_VALUE),
            vec![vec![scaled, 0x1005, 0x1234]]
        );
        assert_eq!(r.calls_to(TEXT_OWNER_NAME), vec![vec![0x9a00]]);
        let (status, label60, label64) = (
            r.tile_at(menu, SLOT_STATUS_TEXT),
            r.tile_at(menu, SLOT_TEXT_60),
            r.tile_at(menu, SLOT_TEXT_64),
        );
        assert_eq!(
            r.calls_to(TILE_SET_STRING),
            vec![
                vec![status, 0xfc4, 0x7500, 1],
                vec![label60, 0xfc4, SETTING_TEXT_60 + 1, 1],
                vec![label64, 0xfc4, SETTING_TEXT_64 + 1, 1],
            ]
        );
    }

    #[test]
    fn the_status_text_falls_back_to_a_setting_without_an_owner() {
        let (mut r, menu) = world();
        r.with(SETTING_STRING, |_, a| eax(a[0] + 1));
        fn_0078aec0(&mut r.e, menu);
        let status = r.tile_at(menu, SLOT_STATUS_TEXT);
        assert_eq!(
            r.calls_to(TILE_SET_STRING)[0],
            vec![status, 0xfc4, SETTING_TIP_FALLBACK + 1, 1]
        );
        assert!(r.calls_to(TEXT_OWNER_NAME).is_empty());
    }

    #[test]
    fn the_stats_are_not_refreshed_when_the_skip_test_passes() {
        let (mut r, menu) = world();
        r.e.mem.set_u32(SKIP_OWNER, 0x9b00);
        r.stub(SKIP_TEST, 1);
        fn_0078aec0(&mut r.e, menu);
        assert!(r.calls_to(REFRESH_STATS).is_empty());
        assert_eq!(r.calls_to(SKIP_TEST), vec![vec![0x9b00]]);
        // A failing test refreshes.
        r.stub(SKIP_TEST, 0);
        fn_0078aec0(&mut r.e, menu);
        assert_eq!(r.calls_to(REFRESH_STATS).len(), 1);
    }

    // ----- 0078afa0 -----

    #[test]
    fn tips_are_switched_with_state_bit_6() {
        let (mut r, menu) = world();
        fn_0078afa0(&mut r.e, menu, 1);
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
        r.e.call(0x0078_afa0, &args![menu, 0u8]);
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
    }

    // ----- 0078afc0 and the four tile-list routines -----

    /// A menu whose five tile lists each hold one tile, with the `NiPointer`
    /// getter and the list step the way the game defines them.
    fn lists_world() -> (Rig, Ptr<LoadingMenu>, Vec<(u32, u32)>) {
        let (mut r, menu) = world();
        r.with(NI_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        r.with(POINTER_LIST_STEP, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            eax(node + 8)
        });
        let mut tiles = Vec::new();
        for list in [0xd0u32, 0xdc, 0xe8, 0xf4, 0x100] {
            let tile = r.object(0x40);
            let node = r.e.mem.alloc(12);
            r.e.mem.set_u32(node + 8, tile);
            r.e.mem.set_u32(menu.addr() + list, node);
            tiles.push((list, tile));
        }
        (r, menu, tiles)
    }

    /// The tiles `SET_TILE_FADE` was called for, with the value bits.
    fn faded(r: &Rig) -> Vec<(u32, u32)> {
        r.calls_to(SET_TILE_FADE)
            .iter()
            .map(|a| (a[1], a[2]))
            .collect()
    }

    fn tile_of(tiles: &[(u32, u32)], list: u32) -> u32 {
        tiles.iter().find(|t| t.0 == list).unwrap().1
    }

    #[test]
    fn each_screen_type_fades_its_own_tile_list() {
        let value = 0.5f32.to_bits();
        for (kind, list) in [(1u32, 0xe8u32), (2, 0xdc), (4, 0xd0)] {
            let (mut r, menu, tiles) = lists_world();
            r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
            r.stub(LOAD_SCREEN_TYPE_FOLDED, 1);
            r.stub(FONT_MANAGER, 0x7000);
            r.stub(LOAD_SCREEN_TYPE, kind);
            fn_0078afc0(&mut r.e, menu, 0x8100, 0.5, 1);
            assert_eq!(
                faded(&r),
                vec![(tile_of(&tiles, list), value)],
                "type {kind}"
            );
            assert_eq!(r.calls_to(SET_TILE_FADE)[0][0], menu.addr());
            // The tile's 0xfa3 value was 0, so it was made 1 first.
            assert_eq!(
                r.calls_to(TILE_SET_INT),
                vec![vec![tile_of(&tiles, list), 0xfa3, 1]]
            );
        }
    }

    #[test]
    fn the_tip_type_fades_a_list_and_its_matte_by_mode() {
        let value = 0.25f32.to_bits();
        let (mut r, menu, tiles) = lists_world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.stub(LOAD_SCREEN_TYPE_FOLDED, 1);
        r.stub(LOAD_SCREEN_TYPE, 3);
        fn_0078afc0(&mut r.e, menu, 0x8100, 0.25, 1);
        assert_eq!(
            faded(&r),
            vec![
                (tile_of(&tiles, 0xf4), value),
                (r.tile_at(menu, SLOT_MATTE_A), value)
            ]
        );
        r.clear_calls();
        fn_0078afc0(&mut r.e, menu, 0x8100, 0.25, 2);
        assert_eq!(
            faded(&r),
            vec![
                (tile_of(&tiles, 0x100), value),
                (r.tile_at(menu, SLOT_MATTE_B), value)
            ]
        );
        // Any other mode does nothing for it.
        r.clear_calls();
        fn_0078afc0(&mut r.e, menu, 0x8100, 0.25, 0);
        assert!(faded(&r).is_empty());
    }

    #[test]
    fn the_other_types_ignore_mode_2() {
        for kind in [1, 2, 4] {
            let (mut r, menu, _) = lists_world();
            r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
            r.stub(LOAD_SCREEN_TYPE_FOLDED, 1);
            r.stub(LOAD_SCREEN_TYPE, kind);
            fn_0078afc0(&mut r.e, menu, 0x8100, 0.5, 2);
            assert!(faded(&r).is_empty(), "type {kind}");
        }
    }

    #[test]
    fn unknown_screen_types_and_untyped_screens_fade_nothing() {
        let (mut r, menu, _) = lists_world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.stub(LOAD_SCREEN_TYPE_FOLDED, 1);
        r.stub(LOAD_SCREEN_TYPE, 5);
        fn_0078afc0(&mut r.e, menu, 0x8100, 0.5, 1);
        r.stub(LOAD_SCREEN_TYPE_FOLDED, 0);
        r.stub(LOAD_SCREEN_TYPE, 1);
        fn_0078afc0(&mut r.e, menu, 0x8100, 0.5, 1);
        assert!(faded(&r).is_empty());
        assert_eq!(r.calls_to(LOAD_SCREEN_TYPE).len(), 1);
    }

    #[test]
    fn without_tips_nothing_is_looked_at() {
        let (mut r, menu, _) = lists_world();
        fn_0078afc0(&mut r.e, menu, 0x8100, 0.5, 1);
        assert!(r.calls_to(LOAD_SCREEN_TYPE_FOLDED).is_empty());
    }

    #[test]
    fn a_missing_screen_defaults_to_the_next_then_the_first() {
        let (mut r, menu, _) = lists_world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        let tail = r.e.mem.u32(menu.addr() + 0x1d4);
        let second = r.e.mem.u32(tail);
        r.e.set(menu, LoadingMenu::pNextLoadScreen, Ptr::new(tail));
        fn_0078afc0(&mut r.e, menu, 0, 0.5, 1);
        assert_eq!(r.calls_to(LOAD_SCREEN_TYPE_FOLDED), vec![vec![second]]);
        r.e.set(menu, LoadingMenu::pNextLoadScreen, Ptr::NULL);
        fn_0078afc0(&mut r.e, menu, 0, 0.5, 1);
        assert_eq!(r.calls_to(LOAD_SCREEN_TYPE_FOLDED)[1], vec![first]);
        // No screen anywhere: stop.
        r.e.mem.set_u32(menu.addr() + 0x1d0, 0);
        fn_0078afc0(&mut r.e, menu, 0, 0.5, 1);
        assert_eq!(r.calls_to(LOAD_SCREEN_TYPE_FOLDED).len(), 2);
    }

    #[test]
    fn a_tile_whose_value_is_set_is_not_forced_on() {
        let (mut r, menu, tiles) = lists_world();
        r.stub_float(TILE_GET_VALUE, 1.0);
        fn_0078b100(&mut r.e, menu, 0.5, 1);
        assert!(r.calls_to(TILE_SET_INT).is_empty());
        assert_eq!(faded(&r), vec![(tile_of(&tiles, 0xd0), 0.5f32.to_bits())]);
        assert_eq!(
            r.calls_to(TILE_GET_VALUE),
            vec![vec![tile_of(&tiles, 0xd0), 0xfa3]]
        );
    }

    #[test]
    fn empty_list_entries_are_skipped_and_every_tile_is_visited() {
        let (mut r, menu, tiles) = lists_world();
        // Chain a second node with a null element and a third with a tile.
        let head = r.e.mem.u32(menu.addr() + 0xdc);
        let extra = r.object(0x40);
        let third = r.e.mem.alloc(12);
        r.e.mem.set_u32(third + 8, extra);
        let second = r.e.mem.alloc(12);
        r.e.mem.set_u32(second, third);
        r.e.mem.set_u32(head, second);
        fn_0078b190(&mut r.e, menu, 1.0, 1);
        assert_eq!(
            faded(&r),
            vec![
                (tile_of(&tiles, 0xdc), 1.0f32.to_bits()),
                (extra, 1.0f32.to_bits())
            ]
        );
    }

    #[test]
    fn the_list_routines_need_mode_1_and_an_empty_list_is_fine() {
        let (mut r, menu, _) = lists_world();
        fn_0078b100(&mut r.e, menu, 0.5, 2);
        fn_0078b190(&mut r.e, menu, 0.5, 0);
        fn_0078b360(&mut r.e, menu, 0.5, 3);
        assert!(faded(&r).is_empty());
        // A list with no head visits nothing.
        r.e.mem.set_u32(menu.addr() + 0xe8, 0);
        fn_0078b360(&mut r.e, menu, 0.5, 1);
        assert!(faded(&r).is_empty());
        // The tip routine with an empty list still fades the matte.
        r.e.mem.set_u32(menu.addr() + 0xf4, 0);
        fn_0078b220(&mut r.e, menu, 0.5, 1);
        assert_eq!(
            faded(&r),
            vec![(r.tile_at(menu, SLOT_MATTE_A), 0.5f32.to_bits())]
        );
    }

    // ----- 0078b3f0 -----

    fn groups(r: &Rig) -> Vec<(u32, u32)> {
        r.calls
            .borrow()
            .iter()
            .filter(|(a, _)| {
                [
                    GROUP_VISIBLE_0078B6A0,
                    GROUP_VISIBLE_0078B650,
                    GROUP_VISIBLE_0078B6F0,
                    GROUP_VISIBLE_0078B7C0,
                ]
                .contains(a)
            })
            .map(|(a, args)| (*a, args[1]))
            .collect()
    }

    fn visibility(kind: u32, show: u8) -> Vec<(u32, u32)> {
        let (mut r, menu) = world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.stub(LOAD_SCREEN_TYPE, kind);
        fn_0078b3f0(&mut r.e, menu, 0x8100, show);
        groups(&r)
    }

    #[test]
    fn the_group_of_the_screen_type_is_shown_and_the_others_hidden() {
        let (a, b, c, d) = (
            GROUP_VISIBLE_0078B6A0,
            GROUP_VISIBLE_0078B650,
            GROUP_VISIBLE_0078B6F0,
            GROUP_VISIBLE_0078B7C0,
        );
        assert_eq!(visibility(1, 1), vec![(a, 0), (b, 0), (c, 0), (d, 1)]);
        assert_eq!(visibility(2, 1), vec![(a, 1), (b, 0), (c, 0), (d, 0)]);
        assert_eq!(visibility(3, 1), vec![(a, 0), (b, 0), (c, 1), (d, 0)]);
        assert_eq!(visibility(4, 1), vec![(a, 0), (b, 1), (c, 0), (d, 0)]);
        // The byte is passed on as it is.
        assert_eq!(visibility(2, 5), vec![(a, 5), (b, 0), (c, 0), (d, 0)]);
    }

    #[test]
    fn unknown_types_hide_all_groups_and_hiding_passes_the_byte_to_all() {
        let (a, b, c, d) = (
            GROUP_VISIBLE_0078B6A0,
            GROUP_VISIBLE_0078B650,
            GROUP_VISIBLE_0078B6F0,
            GROUP_VISIBLE_0078B7C0,
        );
        assert_eq!(visibility(7, 1), vec![(a, 0), (b, 0), (c, 0), (d, 0)]);
        assert_eq!(visibility(3, 0), vec![(a, 0), (b, 0), (c, 0), (d, 0)]);
    }

    #[test]
    fn the_groups_are_left_alone_without_tips_or_a_screen() {
        let (mut r, menu) = world();
        fn_0078b3f0(&mut r.e, menu, 0x8100, 1);
        assert!(groups(&r).is_empty());
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.e.set(menu, LoadingMenu::pNextLoadScreen, Ptr::NULL);
        r.e.mem.set_u32(menu.addr() + 0x1d0, 0);
        fn_0078b3f0(&mut r.e, menu, 0, 1);
        assert!(groups(&r).is_empty());
    }

    #[test]
    fn a_missing_screen_is_taken_from_the_list() {
        let (mut r, menu) = world();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.stub(LOAD_SCREEN_TYPE, 4);
        let first = r.e.mem.u32(menu.addr() + 0x1d0);
        fn_0078b3f0(&mut r.e, menu, 0, 1);
        assert_eq!(r.calls_to(LOAD_SCREEN_TYPE), vec![vec![first]]);
        assert_eq!(groups(&r).len(), 4);
    }

    // ===== the second half: 0078b650 to 0078d8d0 =====

    use std::cell::Cell;
    use std::collections::HashMap;

    /// Tile values by (tile, id), as the doubles of the tile accessors keep
    /// them.
    type Values = Rc<RefCell<HashMap<(u32, u32), f32>>>;

    fn value(values: &Values, tile: u32, id: u32) -> Option<f32> {
        values.borrow().get(&(tile, id)).copied()
    }

    /// Doubles for the tile value getter and the setters that keep values
    /// (an integer is stored as the `float` the game converts it to).
    fn tile_values(r: &mut Rig) -> Values {
        let values: Values = Rc::default();
        let store = values.clone();
        r.with(TILE_GET_VALUE, move |_, a| {
            st0(store.borrow().get(&(a[0], a[1])).copied().unwrap_or(0.0))
        });
        let store = values.clone();
        r.with(TILE_SET_VALUE, move |_, a| {
            store
                .borrow_mut()
                .insert((a[0], a[1]), f32::from_bits(a[2]));
            eax(0)
        });
        for addr in [TILE_SET_INT, TILE_SET_ID_VALUE] {
            let store = values.clone();
            r.with(addr, move |_, a| {
                store.borrow_mut().insert((a[0], a[1]), a[2] as i32 as f32);
                eax(0)
            });
        }
        values
    }

    /// The `float` helpers the layouts call, the way the game defines them,
    /// the truncation of `_ftol2_sse` and `snprintf` of a `%d`.
    fn math(r: &mut Rig) {
        r.with(FLOAT_MAX, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])))
        });
        r.with(FLOAT_MIN, |_, a| {
            st0(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        r.with(FLOAT_TO_INT, |_, a| {
            eax(f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32)
        });
        r.with(SNPRINTF, |e, a| {
            let text = format!("{}\0", a[3] as i32);
            for (i, byte) in text.bytes().enumerate() {
                e.mem.set_u8(a[0] + i as u32, byte);
            }
            eax(text.len() as u32 - 1)
        });
    }

    /// Doubles for the `NiTPointerList` head getter and step.
    fn list_doubles(r: &mut Rig) {
        r.with(NI_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        r.with(POINTER_LIST_STEP, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            eax(node + 8)
        });
    }

    /// Puts `items` into the list whose head word is at `head` (nodes: next,
    /// unused, item).
    fn put_list(r: &mut Rig, head: u32, items: &[u32]) {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = r.e.mem.alloc(12);
            r.e.mem.set_u32(node, next);
            r.e.mem.set_u32(node + 8, *item);
            next = node;
        }
        r.e.mem.set_u32(head, next);
    }

    /// The children list of a tile in the other layout (`00726070` gives the
    /// word at `+4` of the list object at `tile + 4`; the step `00683520`
    /// has next at `+4` and the item at `+8`).
    fn put_children(r: &mut Rig, tile: u32, items: &[u32]) {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = r.e.mem.alloc(12);
            r.e.mem.set_u32(node + 4, next);
            r.e.mem.set_u32(node + 8, *item);
            next = node;
        }
        r.e.mem.set_u32(tile + 8, next);
    }

    fn child_step(r: &mut Rig) {
        r.with(CHILD_STEP, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node + 4);
            e.mem.set_u32(a[1], next);
            eax(node + 8)
        });
    }

    /// An object whose vtable slots answer `answers` (slot offset, value).
    fn object_answering(r: &mut Rig, answers: &[(u32, u32)]) -> u32 {
        let vtable = r.e.mem.alloc(0x100);
        let object = r.e.mem.alloc(0x100);
        r.e.mem.set_u32(object, vtable);
        for (slot, answer) in answers {
            let target = vtable + 0x80 + slot;
            r.e.mem.set_u32(vtable + slot, target);
            r.stub(target, *answer);
        }
        object
    }

    /// The world of the second half: a full menu, real flags, a clock, and
    /// real tile value storage and helpers.
    fn second() -> (Rig, Ptr<LoadingMenu>, Values) {
        let (mut r, menu) = world();
        // The real functions of the second half answer at their addresses (the
        // older tests double them).
        for (addr, function) in funcs() {
            if addr >= 0x0078_b650 {
                r.e.register(addr, function);
            }
        }
        let values = tile_values(&mut r);
        math(&mut r);
        list_doubles(&mut r);
        child_step(&mut r);
        (r, menu, values)
    }

    /// A settings block: rotation 0.5 (`+0x14`), font 3 (`+0x18`), values
    /// 1.5, 2.5, 3.5 (`+0x1c`..), justification (`+0x28`), font 4 and values
    /// 4.5, 5.5, 6.5 (`+0x40`..), and `count` (`+0x54`).
    fn block(r: &mut Rig, justification: u32, count: u32) -> u32 {
        let data = r.e.mem.alloc(0x60);
        r.e.mem.set_f32(data + 0x14, 0.5);
        r.e.mem.set_u32(data + 0x18, 3);
        for (offset, v) in [(0x1c, 1.5), (0x20, 2.5), (0x24, 3.5)] {
            r.e.mem.set_f32(data + offset, v);
        }
        r.e.mem.set_u32(data + 0x28, justification);
        r.e.mem.set_u32(data + 0x40, 4);
        for (offset, v) in [(0x44, 4.5), (0x48, 5.5), (0x4c, 6.5)] {
            r.e.mem.set_f32(data + offset, v);
        }
        r.e.mem.set_u32(data + 0x54, count);
        data
    }

    fn set_faces(r: &mut Rig, menu: Ptr<LoadingMenu>, rect: [i32; 4]) {
        r.e.set(menu, LoadingMenu::faceLeft, rect[0]);
        r.e.set(menu, LoadingMenu::faceTop, rect[1]);
        r.e.set(menu, LoadingMenu::faceRight, rect[2]);
        r.e.set(menu, LoadingMenu::faceBottom, rect[3]);
    }

    // ----- 0078b650, 0078b6a0, 0078b7c0, 0078b6f0 -----

    #[test]
    fn the_group_walkers_set_value_0xfa3_of_their_own_list() {
        let (mut r, menu, values) = second();
        let (a, b, c) = (r.object(0x40), r.object(0x40), r.object(0x40));
        put_list(&mut r, menu.addr() + 0xd0, &[a]);
        put_list(&mut r, menu.addr() + 0xdc, &[b]);
        put_list(&mut r, menu.addr() + 0xe8, &[c]);
        fn_0078b650(&mut r.e, menu, 1);
        assert_eq!(value(&values, a, 0xfa3), Some(1.0));
        assert_eq!(value(&values, b, 0xfa3), None);
        fn_0078b6a0(&mut r.e, menu, 1);
        assert_eq!(value(&values, b, 0xfa3), Some(1.0));
        assert_eq!(value(&values, c, 0xfa3), None);
        fn_0078b7c0(&mut r.e, menu, 0);
        assert_eq!(value(&values, c, 0xfa3), Some(0.0));
        // They go through the registered entries too.
        r.e.call(0x0078_b650, &args![menu, 0u8]);
        assert_eq!(value(&values, a, 0xfa3), Some(0.0));
    }

    #[test]
    fn the_tip_walker_skips_null_tiles_and_sets_both_mattes() {
        let (mut r, menu, values) = second();
        let (a, b, c) = (r.object(0x40), r.object(0x40), r.object(0x40));
        put_list(&mut r, menu.addr() + 0xf4, &[a, 0, b]);
        put_list(&mut r, menu.addr() + 0x100, &[c]);
        fn_0078b6f0(&mut r.e, menu, 1);
        for tile in [a, b, c, r.tile_at(menu, 0xb8), r.tile_at(menu, 0xbc)] {
            assert_eq!(value(&values, tile, 0xfa3), Some(1.0));
        }
        assert_eq!(value(&values, 0, 0xfa3), None);
    }

    // ----- 0078b810 -----

    #[test]
    fn the_default_settings_record_each_tile_and_default_the_empty_slots() {
        let (mut r, menu, values) = second();
        for slot in 0..0x26u32 {
            r.e.mem.set_u32(menu.addr() + 0x28 + slot * 4, 0);
        }
        let tile = r.object(0x40);
        r.e.mem.set_u32(menu.addr() + 0x28 + 2 * 4, tile);
        for (id, v) in [
            (0xfa1, 10.9),
            (0xfa2, 20.5),
            (0xfb0, 30.0),
            (0xfb1, 40.25),
            (0xfb9, 2.0),
            (0xfb7, 2.9),
        ] {
            values.borrow_mut().insert((tile, id), v);
        }
        fn_0078b810(&mut r.e, menu);
        let record = menu.addr() + 0x224 + 2 * 0x18;
        let words: Vec<u32> = (0..5).map(|i| r.e.mem.u32(record + 4 * i)).collect();
        assert_eq!(words, vec![10, 20, 30, 40, 2]);
        assert_eq!(r.e.mem.u8(record + 20), 2);
        let empty = menu.addr() + 0x224;
        let words: Vec<u32> = (0..5).map(|i| r.e.mem.u32(empty + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0, 0, 1]);
        assert_eq!(r.e.mem.u8(empty + 20), 4);
        // The last slot is covered, and nothing is written past it.
        let last = menu.addr() + 0x224 + 0x25 * 0x18;
        assert_eq!(r.e.mem.u8(last + 20), 4);
        assert_eq!(r.e.get(menu, LoadingMenu::m_TileCrossFadeStartTime), 0);
    }

    #[test]
    fn truncation_gives_the_indefinite_value_out_of_range() {
        assert_eq!(truncate_to_i32(2.9), 2);
        assert_eq!(truncate_to_i32(-2.9), -2);
        assert_eq!(truncate_to_i32(3e10), i32::MIN);
        assert_eq!(truncate_to_i32(f64::NAN), i32::MIN);
        assert_eq!(truncate_to_i64_low(4294967297.5), 1);
        assert_eq!(truncate_to_i64_low(1e30), 0);
    }

    // ----- 0078b9b0 -----

    fn faces_world() -> (Rig, Ptr<LoadingMenu>, u32) {
        let (mut r, menu, _values) = second();
        r.stub_float(SCREEN_WIDTH, 1920.0);
        r.stub_float(SCREEN_HEIGHT, 1080.0);
        r.stub_float(INSET_X, 10.0);
        r.stub_float(INSET_Y, 20.0);
        r.put_f64(DESIGN_WIDTH, 1280.0);
        r.put_f64(DESIGN_HEIGHT, 960.0);
        let data = r.e.mem.alloc(0x20);
        for (i, v) in [1u32, 128, 96, 640, 480].iter().enumerate() {
            r.e.mem.set_u32(data + 4 * i as u32, *v);
        }
        (r, menu, data)
    }

    fn rect(r: &mut Rig, menu: Ptr<LoadingMenu>) -> (i32, i32, i32, i32) {
        face_rect(&mut r.e, menu)
    }

    #[test]
    fn the_text_window_is_scaled_to_the_screen() {
        let (mut r, menu, data) = faces_world();
        loading_menu_calc_load_screen_type_text_window_faces(&mut r.e, menu, data);
        // 128 / 1280 * 1920, 96 / 960 * 1080, + 640 / 1280 * 1920 and
        // + 480 / 1280 * 1080 (the height uses the 1280 divisor too).
        assert_eq!(rect(&mut r, menu), (192, 108, 1152, 513));
        assert_eq!(
            r.order()[..4],
            [SCREEN_WIDTH, SCREEN_HEIGHT, INSET_X, INSET_Y]
        );
    }

    #[test]
    fn the_text_window_is_clamped_to_the_insets() {
        let (mut r, menu, data) = faces_world();
        r.e.mem.set_u32(data + 4, 1000);
        r.e.mem.set_u32(data + 8, 1);
        loading_menu_calc_load_screen_type_text_window_faces(&mut r.e, menu, data);
        // left 1500, top 1.125 raised to the inset 20; right limited to
        // 1920 - 10, bottom 20 + 405.
        assert_eq!(rect(&mut r, menu), (1500, 20, 1910, 425));
    }

    #[test]
    fn an_unused_block_leaves_the_window_and_a_null_block_warns() {
        let (mut r, menu, data) = faces_world();
        r.e.mem.set_u32(data, 0);
        loading_menu_calc_load_screen_type_text_window_faces(&mut r.e, menu, data);
        assert_eq!(rect(&mut r, menu), (0, 0, 0, 0));
        assert!(r.calls_to(FLOAT_MAX).is_empty());
        r.clear_calls();
        loading_menu_calc_load_screen_type_text_window_faces(&mut r.e, menu, 0);
        assert_eq!(r.order(), vec![ERROR_LOG]);
        assert_eq!(r.calls_to(ERROR_LOG), vec![vec![NULL_DATA_WARNING]]);
    }

    // ----- 0078bc60, 0078bb40, 0078bb60 -----

    #[test]
    fn the_type_block_is_the_word_at_0x34_plus_0x18() {
        let (mut r, _menu, _values) = second();
        let screen = r.screen();
        r.e.mem.set_u32(screen + 0x34, 0x4000);
        assert_eq!(fn_0078bc60(&mut r.e, screen), 0x4018);
        assert_eq!(r.e.call(0x0078_bc60, &args![screen]).u32(), 0x4018);
    }

    /// A menu with tips enabled whose first screen has type `ty` and a block.
    fn layout_world(ty: u32) -> (Rig, Ptr<LoadingMenu>, Values, u32) {
        let (mut r, menu, values) = faces_world_menu();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.stub(LOAD_SCREEN_TYPE, ty);
        r.stub(LOAD_SCREEN_TYPE_FOLDED, 1);
        r.stub(FONT_MANAGER, 0x7000);
        let screen = r.e.mem.u32(menu.addr() + 0x1dc);
        let data = block(&mut r, 1, 2);
        r.e.mem.set_u32(screen + 0x34, data - 0x18);
        r.e.mem.set_u8(LAYOUT_FLAG_011A0296, 1);
        (r, menu, values, data)
    }

    fn faces_world_menu() -> (Rig, Ptr<LoadingMenu>, Values) {
        let (mut r, menu, values) = second();
        r.stub_float(SCREEN_WIDTH, 1280.0);
        r.stub_float(SCREEN_HEIGHT, 960.0);
        r.stub_float(INSET_X, 0.0);
        r.stub_float(INSET_Y, 0.0);
        r.put_f64(DESIGN_WIDTH, 1280.0);
        r.put_f64(DESIGN_HEIGHT, 960.0);
        r.put_f64(TWO, 2.0);
        r.put_f64(MS_PER_SECOND, 1000.0);
        (r, menu, values)
    }

    #[test]
    fn the_screen_layout_follows_the_type_of_the_screen() {
        // Type 4: stats (rotation of the area tile), 2: objective, 1: XP bar,
        // 3: tips (no rotation call, the tip tiles get placed).
        for (ty, slot) in [(4, 0xa4), (2, 0x94), (1, 0x68)] {
            let (mut r, menu, values, _) = layout_world(ty);
            let target = r.tile_at(menu, slot);
            // Give the type the tiles it walks.
            for list in [0xdc, 0xe8] {
                put_list(&mut r, menu.addr() + list, &[target]);
            }
            fn_0078bb60(&mut r.e, menu, 0);
            assert_eq!(
                r.calls_to(TILE_ROTATION_UPDATE),
                vec![vec![target]],
                "type {ty}"
            );
            assert_eq!(value(&values, target, 0xffa), Some(0.5), "type {ty}");
            assert_eq!(r.e.mem.u8(LAYOUT_FLAG_011A0296), 0);
        }
        let (mut r, menu, values, _) = layout_world(3);
        let tip = r.object(0x40);
        put_list(&mut r, menu.addr() + 0xf4, &[tip]);
        fn_0078bb60(&mut r.e, menu, 0);
        assert!(r.calls_to(TILE_ROTATION_UPDATE).is_empty());
        assert_eq!(value(&values, tip, 0xffa), Some(0.5));
    }

    #[test]
    fn the_layout_needs_tips_a_screen_and_a_type_block() {
        let (mut r, menu, _values, _) = layout_world(4);
        r.e.mem.set_u16(menu.addr() + 0x222, 0);
        fn_0078bb60(&mut r.e, menu, 0);
        assert_eq!(r.e.mem.u8(LAYOUT_FLAG_011A0296), 1);
        assert!(r.calls_to(LOAD_SCREEN_TYPE_FOLDED).is_empty());
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.stub(LOAD_SCREEN_TYPE_FOLDED, 0);
        fn_0078bb60(&mut r.e, menu, 0);
        // No block: only the byte is cleared.
        assert_eq!(r.e.mem.u8(LAYOUT_FLAG_011A0296), 0);
        assert!(r.calls_to(SCREEN_WIDTH).is_empty());
        // No screen at all: nothing, the byte stays.
        r.e.mem.set_u8(LAYOUT_FLAG_011A0296, 1);
        r.e.set(menu, LoadingMenu::pNextLoadScreen, Ptr::NULL);
        r.e.mem.set_u32(menu.addr() + 0x1d0, 0);
        fn_0078bb60(&mut r.e, menu, 0);
        assert_eq!(r.e.mem.u8(LAYOUT_FLAG_011A0296), 1);
    }

    #[test]
    fn the_current_screen_is_laid_out_by_0078bb40() {
        let (mut r, menu, _values, _) = layout_world(4);
        let current = r.e.mem.u32(menu.addr() + 0x1dc);
        r.e.call(0x0078_bb40, &args![menu]);
        assert_eq!(r.calls_to(LOAD_SCREEN_TYPE_FOLDED), vec![vec![current]]);
        assert_eq!(r.calls_to(TILE_ROTATION_UPDATE).len(), 1);
    }

    // ----- 0078bc80 -----

    #[test]
    fn the_stats_screen_places_rows_until_the_count_is_reached() {
        let (mut r, menu, values) = second();
        let data = block(&mut r, 1, 1);
        set_faces(&mut r, menu, [10, 20, 110, 220]);
        let (area, column_a, column_b) = (
            r.tile_at(menu, 0xa4),
            r.tile_at(menu, 0x6c),
            r.tile_at(menu, 0x70),
        );
        let rows: Vec<(u32, u32)> = (0..2).map(|_| (r.object(0x40), r.object(0x40))).collect();
        put_children(&mut r, column_a, &[rows[0].0, 0, rows[1].0]);
        put_children(&mut r, column_b, &[rows[0].1, 0, rows[1].1]);
        loading_menu_init_stats_tiles_settings(&mut r.e, menu, data);
        for (id, expected) in [(0xfa1, 10.0), (0xfb1, 100.0), (0xfa2, 20.0), (0xfb0, 200.0)] {
            assert_eq!(value(&values, area, id), Some(expected), "{id:x}");
        }
        assert_eq!(value(&values, column_a, 0xfb9), Some(3.0));
        assert_eq!(value(&values, column_b, 0xfb9), Some(4.0));
        // The second block of values goes to the first column again.
        assert_eq!(value(&values, column_a, 0xfb2), Some(4.5));
        assert_eq!(value(&values, column_a, 0xfb4), Some(6.5));
        assert_eq!(value(&values, column_b, 0xfb2), None);
        // Row 1 is shown at y 0; the null pair is skipped; row 2 is hidden.
        for child in [rows[0].0, rows[0].1] {
            assert_eq!(value(&values, child, 0xfa2), Some(0.0));
            assert_eq!(value(&values, child, 0xfa3), Some(1.0));
        }
        for child in [rows[1].0, rows[1].1] {
            assert_eq!(value(&values, child, 0xfa2), None);
            assert_eq!(value(&values, child, 0xfa3), Some(0.0));
        }
        let updated: Vec<u32> = r
            .calls_to(TILE_ADD_NEEDS_UPDATE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(updated, vec![rows[0].0, rows[0].1]);
        assert_eq!(value(&values, area, 0xffa), Some(0.5));
        assert_eq!(r.calls_to(TILE_ROTATION_UPDATE), vec![vec![area]]);
    }

    #[test]
    fn the_rows_are_spaced_by_the_area_height_over_the_count() {
        let (mut r, menu, values) = second();
        let data = block(&mut r, 1, 2);
        set_faces(&mut r, menu, [0, 0, 100, 200]);
        let (column_a, column_b) = (r.tile_at(menu, 0x6c), r.tile_at(menu, 0x70));
        let rows: Vec<(u32, u32)> = (0..2).map(|_| (r.object(0x40), r.object(0x40))).collect();
        put_children(&mut r, column_a, &[rows[0].0, rows[1].0]);
        put_children(&mut r, column_b, &[rows[0].1, rows[1].1]);
        loading_menu_init_stats_tiles_settings(&mut r.e, menu, data);
        assert_eq!(value(&values, rows[0].0, 0xfa2), Some(0.0));
        assert_eq!(value(&values, rows[1].0, 0xfa2), Some(100.0));
        assert_eq!(value(&values, rows[1].1, 0xfa3), Some(1.0));
    }

    // ----- 0078bf80 -----

    #[test]
    fn the_objective_tile_is_placed_by_justification() {
        for (justification, x) in [(1, 100.0), (2, 300.0), (4, 500.0), (3, 0.0)] {
            let (mut r, menu, values) = second();
            let data = block(&mut r, justification, 0);
            set_faces(&mut r, menu, [100, 50, 500, 450]);
            let objective = r.tile_at(menu, 0x94);
            let other = r.object(0x40);
            r.put_f64(TWO, 2.0);
            put_list(&mut r, menu.addr() + 0xdc, &[0, other, objective]);
            r.stub(FONT_MANAGER, 0x7000);
            fn_0078bf80(&mut r.e, menu, data);
            assert_eq!(value(&values, objective, 0xfa1), Some(x), "{justification}");
            assert_eq!(value(&values, objective, 0xfb1), Some(400.0));
            assert_eq!(value(&values, objective, 0xfba), Some(400.0));
            assert_eq!(value(&values, objective, 0xfa2), Some(50.0));
            assert_eq!(value(&values, objective, 0xfb9), Some(3.0));
            assert_eq!(value(&values, objective, 0xfb7), Some(justification as f32));
            assert_eq!(value(&values, objective, 0xfb2), Some(1.5));
            assert_eq!(value(&values, objective, 0xff5), Some(-1.0));
            assert_eq!(value(&values, objective, 0xffa), Some(0.5));
            // Another tile of the list is left alone.
            assert_eq!(value(&values, other, 0xfa1), None);
            assert_eq!(r.calls_to(TILE_ADD_NEEDS_UPDATE), vec![vec![objective, 8]]);
            assert_eq!(r.calls_to(TILE_ROTATION_UPDATE), vec![vec![objective]]);
        }
    }

    #[test]
    fn the_status_tile_gets_the_height_of_its_text() {
        let (mut r, menu, values) = second();
        let data = block(&mut r, 1, 0);
        set_faces(&mut r, menu, [100, 50, 500, 450]);
        let status = r.tile_at(menu, 0x34);
        put_list(&mut r, menu.addr() + 0xdc, &[status]);
        r.stub(FONT_MANAGER, 0x7000);
        r.stub(SETTING_STRING, 0x7100);
        r.with(STRING_DIMENSIONS, |e, a| {
            e.mem.set_f32(a[1], 11.0);
            e.mem.set_f32(a[1] + 4, 33.0);
            eax(a[1])
        });
        fn_0078bf80(&mut r.e, menu, data);
        assert_eq!(value(&values, status, 0xfa2), Some(33.0));
        assert_eq!(value(&values, status, 0xfa1), None);
        // this = the font manager, then out, text, font, width, 0.
        let call = &r.calls_to(STRING_DIMENSIONS)[0];
        assert_eq!(call[0], 0x7000);
        assert_eq!(call[2..], [0x7100, 3, 400.0f32.to_bits(), 0]);
        assert_eq!(r.calls_to(SETTING_STRING), vec![vec![SETTING_TEXT_60]]);
        // The font value getter was called on the font of the type.
        assert_eq!(r.calls_to(FONT_VALUE).len(), 1);
    }

    // ----- 0078c210 -----

    #[test]
    fn the_xp_screen_places_the_bar_the_text_and_the_numbers() {
        let (mut r, menu, values) = second();
        let data = block(&mut r, 2, 0);
        set_faces(&mut r, menu, [100, 50, 700, 450]);
        let (bar, text, numbers) = (
            r.tile_at(menu, 0x68),
            r.tile_at(menu, 0x64),
            r.tile_at(menu, 0x50),
        );
        let other = r.object(0x40);
        put_list(&mut r, menu.addr() + 0xe8, &[0, bar, text, numbers, other]);
        values.borrow_mut().insert((numbers, 0x1005), 12.9);
        values.borrow_mut().insert((numbers, 0x1006), 99.5);
        r.put_f64(TWO, 2.0);
        r.stub(FONT_MANAGER, 0x7000);
        r.stub(SETTING_STRING, 0x7100);
        // The size of a text: ten units per character wide, 7 high.
        r.with(STRING_DIMENSIONS, |e, a| {
            let mut length = 0;
            while e.mem.u8(a[2] + length) != 0 {
                length += 1;
            }
            e.mem.set_f32(a[1], 10.0 * length as f32);
            e.mem.set_f32(a[1] + 4, 7.0);
            eax(a[1])
        });
        fn_0078c210(&mut r.e, menu, data);
        // The bar.
        assert_eq!(value(&values, bar, 0xfa1), Some(100.0));
        assert_eq!(value(&values, bar, 0xfb1), Some(600.0));
        assert_eq!(value(&values, bar, 0xfba), Some(600.0));
        assert_eq!(value(&values, bar, 0xfa2), Some(50.0));
        assert_eq!(value(&values, bar, 0xfb0), Some(400.0));
        // The bar gets none of the common values.
        assert_eq!(value(&values, bar, 0xff5), None);
        // The text: centred, justification, font, width, text height.
        assert_eq!(value(&values, text, 0xfa1), Some(300.0));
        assert_eq!(value(&values, text, 0xfb7), Some(2.0));
        assert_eq!(value(&values, text, 0xfb9), Some(3.0));
        assert_eq!(value(&values, text, 0xfb1), Some(600.0));
        assert_eq!(value(&values, text, 0xfba), Some(600.0));
        assert_eq!(value(&values, text, 0xfb0), Some(7.0));
        assert_eq!(value(&values, text, 0xff5), Some(-1.0));
        // The numbers: "12" is 20 wide, "99" is 20 wide, so the width is
        // (600 - 2 * 20) - 20.
        assert_eq!(value(&values, numbers, 0xfa1), Some(20.0));
        assert_eq!(value(&values, numbers, 0xfb1), Some(540.0));
        assert_eq!(value(&values, numbers, 0xfb4), Some(3.5));
        // The second text is wrapped at (600 - 20) / 2.
        let widths: Vec<u32> = r.calls_to(STRING_DIMENSIONS).iter().map(|c| c[4]).collect();
        assert_eq!(widths[1], 0.0f32.to_bits());
        assert_eq!(widths[2], 290.0f32.to_bits());
        // The other tile only gets the common values.
        assert_eq!(value(&values, other, 0xfb2), Some(1.5));
        assert_eq!(value(&values, other, 0xfa1), None);
        assert_eq!(value(&values, bar, 0xffa), Some(0.5));
        assert_eq!(r.calls_to(TILE_ROTATION_UPDATE), vec![vec![bar]]);
    }

    #[test]
    fn the_xp_text_is_placed_by_justification() {
        for (justification, x) in [(1, 0.0), (2, 300.0), (4, 600.0)] {
            let (mut r, menu, values) = second();
            let data = block(&mut r, justification, 0);
            set_faces(&mut r, menu, [100, 50, 700, 450]);
            let text = r.tile_at(menu, 0x64);
            put_list(&mut r, menu.addr() + 0xe8, &[text]);
            r.put_f64(TWO, 2.0);
            r.stub(FONT_MANAGER, 0x7000);
            fn_0078c210(&mut r.e, menu, data);
            assert_eq!(value(&values, text, 0xfa1), Some(x), "{justification}");
        }
    }

    // ----- 0078c770 -----

    #[test]
    fn every_tip_tile_is_placed_at_the_text_window() {
        for (justification, x) in [(1, 100.0), (2, 300.0), (4, 500.0)] {
            let (mut r, menu, values) = second();
            let data = block(&mut r, justification, 0);
            set_faces(&mut r, menu, [100, 50, 500, 450]);
            r.put_f64(TWO, 2.0);
            let (a, b) = (r.object(0x40), r.object(0x40));
            put_list(&mut r, menu.addr() + 0xf4, &[0, a]);
            put_list(&mut r, menu.addr() + 0x100, &[b]);
            fn_0078c770(&mut r.e, menu, data);
            for tile in [a, b] {
                assert_eq!(value(&values, tile, 0xfa1), Some(x), "{justification}");
                assert_eq!(value(&values, tile, 0xfb1), Some(400.0));
                assert_eq!(value(&values, tile, 0xfba), Some(400.0));
                assert_eq!(value(&values, tile, 0xfa2), Some(50.0));
                assert_eq!(value(&values, tile, 0xfb0), Some(400.0));
                assert_eq!(value(&values, tile, 0xfbb), Some(400.0));
                assert_eq!(value(&values, tile, 0xffa), Some(0.5));
                assert_eq!(value(&values, tile, 0xfb9), Some(3.0));
                assert_eq!(value(&values, tile, 0xff5), Some(-1.0));
            }
            assert_eq!(value(&values, 0, 0xfa1), None);
            assert_eq!(r.calls_to(TILE_ADD_NEEDS_UPDATE).len(), 2);
        }
    }

    // ----- 0078cbd0 -----

    #[test]
    fn hide_all_but_shows_the_selected_slot_only() {
        let (mut r, menu, _values) = second();
        loading_menu_hide_all_but(&mut r.e, menu, 3, 1);
        let calls = r.calls_to(TILE_SET_VALUE);
        assert_eq!(calls.len(), 0x26);
        for (index, call) in calls.iter().enumerate() {
            let shown = if index == 3 { 1.0f32 } else { 0.0 };
            assert_eq!(
                call,
                &vec![
                    r.tile_at(menu, 0x28 + 4 * index as u32),
                    0xfa3,
                    shown.to_bits(),
                    0
                ]
            );
        }
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
    }

    #[test]
    fn hide_all_but_zero_also_shows_slots_0x20_and_0x21() {
        let (mut r, menu, _values) = second();
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        r.e.call(0x0078_cbd0, &args![menu, 0i32, 0u8]);
        let shown: Vec<usize> = r
            .calls_to(TILE_SET_VALUE)
            .iter()
            .enumerate()
            .filter(|(_, c)| f32::from_bits(c[2]) == 1.0)
            .map(|(i, _)| i)
            .collect();
        assert_eq!(shown, vec![0, 0x20, 0x21]);
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
    }

    // ----- 0078cc50, 0078cdf0 -----

    fn fade_world() -> (Rig, Ptr<LoadingMenu>, Values, u32) {
        let (mut r, menu, values) = second();
        let menu_tile = r.object(0x100);
        r.e.set(menu, LoadingMenu::pMenu, Ptr::new(menu_tile));
        values.borrow_mut().insert((menu_tile, 0xfc1), 2.0);
        r.put_f64(MS_PER_SECOND, 1000.0);
        r.put_f64(BYTE_SCALE, 255.0);
        r.e.mem.set_u8(INITIAL_LOAD, 0);
        (r, menu, values, menu_tile)
    }

    #[test]
    fn the_fade_in_runs_from_now_for_the_duration() {
        let (mut r, menu, values, menu_tile) = fade_world();
        r.stub(RENDERED_GET_TEXTURE, 0x1357);
        r.e.mem.set_u16(menu.addr() + 0x222, bit(4));
        fn_0078cc50(&mut r.e, menu);
        // The duration is computed once (the guard bit), 2 * 1000.
        assert_eq!(r.e.mem.f32(FADE_DURATION), 2000.0);
        assert_eq!(r.e.mem.u32(FADE_DURATION_GUARD) & 1, 1);
        let image = r.tile_at(menu, 0x58);
        assert_eq!(
            r.calls_to(IMAGE_SET_SOURCE_TEXTURE),
            vec![vec![image, 0x1357]]
        );
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 12_000);
        assert_eq!(value(&values, menu_tile, 0xfa9), Some(0.0));
        // Half way.
        values.borrow_mut().insert((menu_tile, 0xfc1), 9.0);
        r.clock(11_000);
        fn_0078cc50(&mut r.e, menu);
        assert_eq!(r.e.mem.f32(FADE_DURATION), 2000.0);
        assert_eq!(value(&values, menu_tile, 0xfa9), Some(127.5));
        assert_eq!(r.calls_to(IMAGE_SET_SOURCE_TEXTURE).len(), 1);
        assert_ne!(r.e.get(menu, LoadingMenu::usFlags) & bit(4), 0);
        // At the end: full, bit 4 cleared, one more slide.
        r.clock(12_000);
        fn_0078cc50(&mut r.e, menu);
        assert_eq!(value(&values, menu_tile, 0xfa9), Some(255.0));
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(4), 0);
        assert_eq!(r.e.get(menu, LoadingMenu::m_SlideCount), 1);
    }

    #[test]
    fn without_a_texture_the_fade_ends_at_once() {
        let (mut r, menu, values, menu_tile) = fade_world();
        r.stub(RENDERED_GET_TEXTURE, 0);
        fn_0078cc50(&mut r.e, menu);
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 10_000);
        assert!(r.calls_to(IMAGE_SET_SOURCE_TEXTURE).is_empty());
        assert_eq!(value(&values, menu_tile, 0xfa9), Some(255.0));
        assert_eq!(r.e.get(menu, LoadingMenu::m_SlideCount), 1);
        // The initial load takes the texture path even without a texture.
        let (mut r, menu, _values, _) = fade_world();
        r.stub(RENDERED_GET_TEXTURE, 0);
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        fn_0078cc50(&mut r.e, menu);
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 12_000);
        assert_eq!(r.calls_to(IMAGE_SET_SOURCE_TEXTURE).len(), 1);
    }

    #[test]
    fn a_fade_in_that_is_set_keeps_its_end() {
        let (mut r, menu, _values, _) = fade_world();
        r.e.set(menu, LoadingMenu::uiFadeInComplete, 10_500);
        r.stub(RENDERED_GET_TEXTURE, 0x1357);
        fn_0078cc50(&mut r.e, menu);
        assert_eq!(r.e.get(menu, LoadingMenu::uiFadeInComplete), 10_500);
        assert!(r.calls_to(RENDERED_GET_TEXTURE).is_empty());
    }

    #[test]
    fn the_rendered_texture_is_got_from_the_ni_pointer() {
        let (mut r, _menu, _values) = second();
        r.e.mem.set_u32(RENDERED_TEXTURE, 0x8888);
        r.stub(RENDERED_GET_TEXTURE, 0x1357);
        assert_eq!(r.e.call(0x0078_cdf0, &args![0x55u32]).u32(), 0x1357);
        assert_eq!(r.calls_to(NI_POINTER_GET), vec![vec![RENDERED_TEXTURE]]);
        assert_eq!(r.calls_to(RENDERED_GET_TEXTURE), vec![vec![0x8888, 0]]);
    }

    // ----- 0078ce10, 0078cf10 -----

    #[test]
    fn fading_clamps_scales_and_reaches_the_children() {
        let (mut r, menu, values) = second();
        r.put_f64(BYTE_SCALE, 255.0);
        let (parent, child) = (r.object(0x100), r.object(0x100));
        put_list(&mut r, parent + 4, &[child]);
        for (input, expected) in [(2.0, 255.0), (-3.0, 0.0), (0.25, 63.75)] {
            fn_0078ce10(&mut r.e, menu, parent, input);
            for tile in [parent, child] {
                assert_eq!(value(&values, tile, 0xfa9), Some(expected), "{input}");
            }
        }
        assert_eq!(r.e.mem.u8(DO_TILE_UPDATE), 1);
        assert_eq!(
            r.calls_to(TILE_SET_VALUE)[0],
            vec![parent, 0xfa9, 255.0f32.to_bits(), 1]
        );
    }

    #[test]
    fn the_fade_image_tile_is_inverted() {
        let (mut r, menu, values) = second();
        r.put_f64(BYTE_SCALE, 255.0);
        let image = r.tile_at(menu, 0x58);
        r.e.call(0x0078_ce10, &args![menu, image, 0.25f32]);
        assert_eq!(value(&values, image, 0xfa9), Some(191.25));
    }

    #[test]
    fn fading_ignores_null_and_the_second_fade_tile() {
        let (mut r, menu, values) = second();
        fn_0078ce10(&mut r.e, menu, 0, 1.0);
        let skipped = r.tile_at(menu, 0xac);
        fn_0078ce10(&mut r.e, menu, skipped, 1.0);
        assert!(values.borrow().is_empty());
        assert_eq!(r.e.mem.u8(DO_TILE_UPDATE), 0);
        assert!(r.calls_to(FLOAT_MIN).is_empty());
    }

    #[test]
    fn a_model_tile_is_faded_through_its_model() {
        let (mut r, menu, values) = second();
        let tile = object_answering(&mut r, &[(0xc, 0x388)]);
        let model = object_answering(&mut r, &[(0xc, 0), (0x18, 1)]);
        r.stub(TILE_MODEL, model);
        r.stub(PROPERTY_ID, 77);
        r.stub(NODE_PROPERTY, 0x99);
        fn_0078ce10(&mut r.e, menu, tile, 1.5);
        assert_eq!(r.calls_to(NODE_PROPERTY), vec![vec![model, 77]]);
        assert_eq!(r.calls_to(MODEL_FADE), vec![vec![0x99, 1.0f32.to_bits()]]);
        assert!(values.borrow().is_empty());
        assert_eq!(r.e.mem.u8(DO_TILE_UPDATE), 0);
    }

    #[test]
    fn the_model_fade_goes_to_the_children_first_and_skips_missing_properties() {
        let (mut r, menu, _values) = second();
        let leaf_a = object_answering(&mut r, &[(0xc, 0), (0x18, 1)]);
        let leaf_b = object_answering(&mut r, &[(0xc, 0), (0x18, 1)]);
        let plain = object_answering(&mut r, &[(0xc, 0), (0x18, 0)]);
        let children = 0x7a00;
        let parent = object_answering(&mut r, &[(0xc, children), (0x18, 1)]);
        r.stub(NODE_CHILD_COUNT, 3);
        let leaves = [leaf_a, plain, leaf_b];
        r.with(NODE_CHILD_AT, move |_, a| eax(leaves[a[1] as usize]));
        r.stub(PROPERTY_ID, 5);
        r.with(NODE_PROPERTY, move |_, a| {
            eax(if a[0] == leaf_b { 0 } else { a[0] + 1 })
        });
        fn_0078cf10(&mut r.e, menu, parent, 0.5);
        assert_eq!(
            r.calls_to(MODEL_FADE),
            vec![
                vec![leaf_a + 1, 0.5f32.to_bits()],
                vec![parent + 1, 0.5f32.to_bits()]
            ]
        );
        assert_eq!(r.calls_to(NODE_CHILD_AT).len(), 3);
        // A null node is ignored.
        r.clear_calls();
        fn_0078cf10(&mut r.e, menu, 0, 0.5);
        assert!(r.order().is_empty());
    }

    // ----- 0078cfc0, 0078d020, 0078d050, 0078d570 -----

    fn thread_rig() -> (Rig, Ptr<LoadingMenuThread>) {
        let (mut r, menu, _values) = second();
        r.e.mem.set_u32(MENU_INSTANCE, menu.addr());
        let thread = r.e.new_object::<LoadingMenuThread>();
        r.e.mem.set_u32(LOADING_THREAD, thread.addr());
        (r, thread)
    }

    #[test]
    fn suspending_asks_an_active_thread_and_waits() {
        let (mut r, thread) = thread_rig();
        r.stub(BS_THREAD_IS_ACTIVE, 1);
        loading_menu_suspend_background_thread(&mut r.e);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bFlagSuspend), 1);
        assert_eq!(
            r.order(),
            vec![BS_THREAD_IS_ACTIVE, BS_THREAD_SET_PRIORITY, SEMAPHORE_WAIT]
        );
        assert_eq!(
            r.calls_to(BS_THREAD_SET_PRIORITY),
            vec![vec![thread.addr(), 1]]
        );
        assert_eq!(r.calls_to(SEMAPHORE_WAIT), vec![vec![thread.addr() + 0x3c]]);
        // Already asked: nothing more.
        r.clear_calls();
        r.e.call(0x0078_cfc0, &args![]);
        assert_eq!(r.order(), vec![BS_THREAD_IS_ACTIVE]);
    }

    #[test]
    fn suspending_needs_the_menu_the_thread_and_an_active_thread() {
        let (mut r, thread) = thread_rig();
        loading_menu_suspend_background_thread(&mut r.e);
        assert_eq!(r.order(), vec![BS_THREAD_IS_ACTIVE]);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bFlagSuspend), 0);
        r.clear_calls();
        r.e.mem.set_u32(LOADING_THREAD, 0);
        loading_menu_suspend_background_thread(&mut r.e);
        r.e.mem.set_u32(MENU_INSTANCE, 0);
        loading_menu_suspend_background_thread(&mut r.e);
        assert!(r.order().is_empty());
    }

    #[test]
    fn resuming_wakes_a_suspended_thread() {
        let (mut r, thread) = thread_rig();
        loading_menu_resume_background_thread(&mut r.e);
        assert!(r.calls_to(SEMAPHORE_RELEASE).is_empty());
        r.e.set(thread, LoadingMenuThread::bFlagSuspended, 1u8);
        r.e.set(thread, LoadingMenuThread::bFlagSuspend, 1u8);
        loading_menu_resume_background_thread(&mut r.e);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bFlagSuspend), 0);
        assert_eq!(
            r.calls_to(SEMAPHORE_RELEASE),
            vec![vec![thread.addr() + 0x30]]
        );
        // Without a menu, nothing.
        r.clear_calls();
        r.e.set(thread, LoadingMenuThread::bFlagSuspend, 1u8);
        r.e.mem.set_u32(MENU_INSTANCE, 0);
        loading_menu_resume_background_thread(&mut r.e);
        assert!(r.order().is_empty());
    }

    #[test]
    fn asking_the_thread_to_end_sets_the_flag_and_wakes_it() {
        let (mut r, thread) = thread_rig();
        r.e.set(thread, LoadingMenuThread::bFlagSuspended, 1u8);
        fn_0078d050(&mut r.e);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bExit), 1);
        assert_eq!(r.calls_to(SEMAPHORE_RELEASE).len(), 1);
        r.e.mem.set_u32(LOADING_THREAD, 0);
        r.clear_calls();
        r.e.call(0x0078_d050, &args![]);
        assert!(r.order().is_empty());
    }

    // ----- 0078d1d0, 0078d1f0, 0078d200 -----

    #[test]
    fn the_recursive_lock_counts_for_its_owner() {
        let (mut r, _menu, _values) = second();
        r.stub(CURRENT_THREAD_ID, 5);
        let lock = r.e.mem.alloc(8);
        r.e.mem.set_u32(lock, 5);
        r.e.mem.set_u32(lock + 4, 2);
        assert!(fn_0078d200(&mut r.e, Ptr::new(lock)));
        assert_eq!(r.e.mem.u32(lock + 4), 3);
        assert!(r.calls_to(COMPARE_EXCHANGE).is_empty());
    }

    #[test]
    fn the_recursive_lock_is_taken_when_free_and_refused_when_not() {
        let (mut r, _menu, _values) = second();
        r.stub(CURRENT_THREAD_ID, 5);
        let lock = r.e.mem.alloc(8);
        r.e.mem.set_u32(lock + 4, 9);
        r.stub(COMPARE_EXCHANGE, 0);
        assert!(r.e.call(0x0078_d200, &args![Ptr::<()>::new(lock)]).bool());
        assert_eq!(r.calls_to(COMPARE_EXCHANGE), vec![vec![lock, 0, 5]]);
        assert_eq!(r.calls_to(ERROR_LOG), vec![Vec::<u32>::new()]);
        assert_eq!(r.e.mem.u32(lock + 4), 1);
        // Held by another thread: previous value 7.
        r.clear_calls();
        r.e.mem.set_u32(lock + 4, 9);
        r.stub(COMPARE_EXCHANGE, 7);
        assert!(!fn_0078d200(&mut r.e, Ptr::new(lock)));
        assert!(r.calls_to(ERROR_LOG).is_empty());
        assert_eq!(r.e.mem.u32(lock + 4), 9);
    }

    #[test]
    fn the_redraw_lock_and_the_section_wrapper_use_their_objects() {
        let (mut r, _menu, _values) = second();
        r.stub(CURRENT_THREAD_ID, 5);
        r.e.mem.set_u32(REDRAW_LOCK, 5);
        assert!(fn_0078d1f0(&mut r.e));
        assert_eq!(r.e.mem.u32(REDRAW_LOCK + 4), 1);
        r.stub(TRY_ENTER_CRITICAL_SECTION, 1);
        assert_eq!(fn_0078d1d0(&mut r.e, Ptr::new(REDRAW_SECTION)), 1);
        assert_eq!(
            r.calls_to(TRY_ENTER_CRITICAL_SECTION),
            vec![vec![REDRAW_SECTION]]
        );
        assert!(r.e.call(0x0078_d1f0, &args![]).bool());
    }

    // ----- 0078d080 -----

    /// A world where every lock can be taken and the render calls are
    /// recorded; `0044b130` answers `lock`.
    fn redraw_world() -> Rig {
        let (mut r, _menu, _values) = second();
        r.stub(REDRAW_SECTION_GETTER, 0x0aaa);
        r.stub(SECTION_TRY_LOCK, 1);
        r.stub(TRY_ENTER_CRITICAL_SECTION, 1);
        r.stub(RENDER_SYSTEM, 0x0bbb);
        r.stub(PLAYER_GETTER, 0x0ccc);
        r.stub(CURRENT_THREAD_ID, 5);
        r.stub(COMPARE_EXCHANGE, 0);
        r
    }

    #[test]
    fn showing_changes_stops_when_the_render_section_is_busy() {
        let mut r = redraw_world();
        r.stub(SECTION_TRY_LOCK, 0);
        loading_menu_show_changes(&mut r.e);
        assert_eq!(
            r.order(),
            vec![
                DECAL_MANAGER_GETTER,
                DECAL_BEGIN,
                REDRAW_SECTION_GETTER,
                SECTION_TRY_LOCK,
                DECAL_MANAGER_GETTER,
                DECAL_END,
            ]
        );
    }

    #[test]
    fn showing_changes_stops_when_the_redraw_section_is_busy() {
        let mut r = redraw_world();
        r.stub(TRY_ENTER_CRITICAL_SECTION, 0);
        loading_menu_show_changes(&mut r.e);
        assert_eq!(r.calls_to(SECTION_UNLOCK), vec![vec![0x0aaa]]);
        assert_eq!(*r.order().last().unwrap(), DECAL_END);
        assert!(r.calls_to(RENDER_SYSTEM).is_empty());
    }

    #[test]
    fn changed_tiles_need_the_redraw_lock_too() {
        let mut r = redraw_world();
        r.e.mem.set_u8(DO_TILE_UPDATE, 1);
        r.e.mem.set_u32(REDRAW_LOCK, 99);
        r.stub(COMPARE_EXCHANGE, 99);
        loading_menu_show_changes(&mut r.e);
        assert_eq!(
            r.calls_to(SECTION_UNLOCK),
            vec![vec![REDRAW_SECTION], vec![0x0aaa]]
        );
        assert!(r.calls_to(TILE_UPDATE_ALL).is_empty());
        assert!(r.calls_to(RENDER_SYSTEM).is_empty());
    }

    #[test]
    fn a_full_redraw_steps_both_render_stages_and_renders_the_scene() {
        let mut r = redraw_world();
        r.e.mem.set_u8(DO_TILE_UPDATE, 1);
        let seen = Rc::new(Cell::new(9u8));
        let flag = seen.clone();
        r.with(RENDER_SCENE, move |e, _| {
            flag.set(e.mem.u8(REDRAWING));
            eax(0)
        });
        loading_menu_show_changes(&mut r.e);
        assert_eq!(r.calls_to(TILE_UPDATE_ALL), vec![vec![1]]);
        assert_eq!(r.calls_to(REDRAW_FLUSH).len(), 1);
        assert_eq!(r.calls_to(RENDER_BEGIN), vec![vec![0x0bbb]]);
        let a = r.calls_to(RENDER_STAGE_A);
        let b = r.calls_to(RENDER_STAGE_B);
        assert_eq!(a.len(), 22);
        assert_eq!(a[0], vec![0x0bbb, 0, 1]);
        assert_eq!(a[21], vec![0x0bbb, 0, 22]);
        assert_eq!(b[0], vec![0x0bbb, 1, 1]);
        assert_eq!(b[21], vec![0x0bbb, 1, 22]);
        assert_eq!(r.calls_to(RENDER_SCENE), vec![vec![0x0ccc, 0, 0]]);
        assert_eq!(seen.get(), 1);
        assert_eq!(r.e.mem.u8(REDRAWING), 0);
        // The locks are released last.
        let order = r.order();
        assert_eq!(
            order[order.len() - 3..],
            [DECAL_END, SECTION_UNLOCK, SECTION_UNLOCK]
        );
        assert_eq!(
            r.calls_to(SECTION_UNLOCK),
            vec![vec![0x0aaa], vec![REDRAW_SECTION]]
        );
    }

    #[test]
    fn unchanged_tiles_are_not_updated() {
        let mut r = redraw_world();
        loading_menu_show_changes(&mut r.e);
        assert!(r.calls_to(TILE_UPDATE_ALL).is_empty());
        assert!(r.calls_to(REDRAW_FLUSH).is_empty());
        assert_eq!(r.calls_to(RENDER_STAGE_A).len(), 22);
    }

    // ----- 0078d280, 0078d2d0 -----

    #[test]
    fn setting_up_for_the_main_menu_creates_or_suspends_and_hides() {
        // No menu, no create: nothing.
        let (mut r, _menu, _values) = second();
        r.e.mem.set_u32(MENU_INSTANCE, 0);
        loading_menu_setup_for_main_menu(&mut r.e, 0);
        assert!(r.order().is_empty());
        // No menu, create: Create(0, 0, 0) runs (it finds no menu file tile).
        r.e.call(0x0078_d280, &args![1u8]);
        assert_eq!(r.calls_to(TILE_GET_MENU_BY_CLASS).len(), 1);
        // A menu: suspends the thread (none here) and hides all but slot 0.
        let (mut r, menu, values) = second();
        r.e.mem.set_u32(MENU_INSTANCE, menu.addr());
        r.e.mem.set_u16(menu.addr() + 0x222, bit(6));
        loading_menu_setup_for_main_menu(&mut r.e, 0);
        assert_eq!(r.calls_to(TILE_SET_VALUE).len(), 0x26);
        assert_eq!(value(&values, r.tile_at(menu, 0x28), 0xfa3), Some(1.0));
        assert_eq!(value(&values, r.tile_at(menu, 0x2c), 0xfa3), Some(0.0));
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(6), 0);
    }

    #[test]
    fn setting_up_suspends_an_existing_thread() {
        let (mut r, thread) = thread_rig();
        r.stub(BS_THREAD_IS_ACTIVE, 1);
        loading_menu_setup_for_main_menu(&mut r.e, 1);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bFlagSuspend), 1);
        assert!(r.calls_to(TILE_GET_MENU_BY_CLASS).is_empty());
    }

    #[test]
    fn hiding_for_the_main_menu_needs_a_menu() {
        let (mut r, menu, values) = second();
        r.e.mem.set_u32(MENU_INSTANCE, 0);
        fn_0078d2d0(&mut r.e);
        assert!(values.borrow().is_empty());
        r.e.mem.set_u32(MENU_INSTANCE, menu.addr());
        r.e.call(0x0078_d2d0, &args![]);
        assert_eq!(r.calls_to(TILE_SET_VALUE).len(), 0x26);
    }

    // ----- 0078d2f0, 0078d3b0, 0078d3f0, 0078d420 -----

    #[test]
    fn the_semaphore_gets_a_handle() {
        let (mut r, _menu, _values) = second();
        r.stub(CREATE_SEMAPHORE, 0x1234);
        let semaphore = r.e.mem.alloc(12);
        r.e.mem.set_u32(semaphore, 7);
        assert_eq!(fn_0078d3b0(&mut r.e, Ptr::new(semaphore)).addr(), semaphore);
        assert_eq!(r.e.mem.u32(semaphore), 0);
        assert_eq!(r.e.mem.u32(semaphore + 4), 0x1234);
        assert_eq!(r.e.mem.u32(semaphore + 8), 1);
        assert_eq!(r.calls_to(CREATE_SEMAPHORE), vec![vec![0, 0, 1, 0]]);
    }

    #[test]
    fn the_thread_is_set_up_and_named() {
        let (mut r, _menu, _values) = second();
        r.stub(CREATE_SEMAPHORE, 0x1234);
        let thread = r.e.new_object::<LoadingMenuThread>();
        r.e.mem.set_u8(thread.addr() + 0x48, 1);
        r.e.mem.set_u8(thread.addr() + 0x4a, 1);
        let result = r.e.call(0x0078_d2f0, &args![thread]).u32();
        assert_eq!(result, thread.addr());
        assert_eq!(r.e.get(thread, LoadingMenuThread::vtable), 0x0107_40ac);
        assert_eq!(r.e.mem.u32(thread.addr() + 0x34), 0x1234);
        assert_eq!(r.e.mem.u32(thread.addr() + 0x40), 0x1234);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bFlagSuspend), 0);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bExit), 0);
        let t = thread.addr();
        assert_eq!(
            r.order(),
            vec![
                BS_THREAD_NEW,
                CREATE_SEMAPHORE,
                CREATE_SEMAPHORE,
                BS_THREAD_SET_NAME,
                BS_THREAD_INITIALIZE,
                BS_THREAD_SET_PROCESSOR,
                BS_THREAD_SET_PRIORITY,
            ]
        );
        assert_eq!(r.calls_to(BS_THREAD_SET_NAME), vec![vec![t, THREAD_NAME]]);
        assert_eq!(
            r.calls_to(BS_THREAD_INITIALIZE),
            vec![vec![t, 0x4000, THREAD_NAME, 0]]
        );
        assert_eq!(r.calls_to(BS_THREAD_SET_PROCESSOR), vec![vec![t, 5]]);
        assert_eq!(
            r.calls_to(BS_THREAD_SET_PRIORITY),
            vec![vec![t, 0xffff_ffff]]
        );
    }

    #[test]
    fn the_thread_destructor_destroys_the_semaphores_and_the_base() {
        let (mut r, _menu, _values) = second();
        r.stub(OPERATOR_DELETE, 0);
        let thread = r.e.new_object::<LoadingMenuThread>();
        let t = thread.addr();
        let result = r.e.call(0x0078_d3f0, &args![thread, 1u32]).u32();
        assert_eq!(result, t);
        assert_eq!(
            r.order(),
            vec![
                SEMAPHORE_DESTRUCTOR,
                SEMAPHORE_DESTRUCTOR,
                BS_THREAD_DESTRUCTOR,
                OPERATOR_DELETE,
            ]
        );
        assert_eq!(
            r.calls_to(SEMAPHORE_DESTRUCTOR),
            vec![vec![t + 0x3c], vec![t + 0x30]]
        );
        assert_eq!(r.e.get(thread, LoadingMenuThread::vtable), 0x0107_40ac);
        r.clear_calls();
        loading_menu_thread_scalar_deleting_destructor(&mut r.e, thread, 0);
        assert!(r.calls_to(OPERATOR_DELETE).is_empty());
        r.e.call(0x0078_d420, &args![thread]);
        assert_eq!(r.calls_to(BS_THREAD_DESTRUCTOR).len(), 2);
    }

    // ----- 0078d490 -----

    /// A world for the thread: `Update` kept quiet, `ShowChanges` cut short
    /// at its first lock, which also counts the rounds and ends the thread
    /// after `rounds` of them.
    fn proc_world(rounds: u32) -> (Rig, Ptr<LoadingMenu>, Ptr<LoadingMenuThread>, Rc<Cell<u32>>) {
        let (mut r, menu) = update_world();
        quiet(&mut r, menu);
        let thread = r.e.new_object::<LoadingMenuThread>();
        r.e.mem.set_u32(MENU_INSTANCE, menu.addr());
        r.e.mem.set_u32(LOADING_THREAD, thread.addr());
        let count = Rc::new(Cell::new(0));
        let counter = count.clone();
        let t = thread.addr();
        r.with(SECTION_TRY_LOCK, move |e, _| {
            counter.set(counter.get() + 1);
            if counter.get() >= rounds {
                e.mem.set_u8(t + 0x4a, 1);
            }
            eax(0)
        });
        (r, menu, thread, count)
    }

    #[test]
    fn the_thread_runs_rounds_until_asked_to_end() {
        let (mut r, _menu, thread, count) = proc_world(2);
        assert_eq!(r.e.call(0x0078_d490, &args![thread]).u32(), 0);
        assert_eq!(count.get(), 2);
        // The second round is ahead of the first one's schedule (now + 0x32).
        assert_eq!(r.calls_to(SLEEP_TICK), vec![vec![0x32]]);
        assert!(r.calls_to(SEMAPHORE_RELEASE).is_empty());
    }

    #[test]
    fn an_exit_flag_set_at_the_start_ends_it_at_once() {
        let (mut r, _menu, thread, count) = proc_world(2);
        r.e.set(thread, LoadingMenuThread::bExit, 1u8);
        assert_eq!(loading_menu_thread_thread_proc(&mut r.e, thread), 0);
        assert_eq!(count.get(), 0);
    }

    #[test]
    fn the_thread_suspends_itself_on_request() {
        let (mut r, _menu, thread, _count) = proc_world(1);
        r.e.set(thread, LoadingMenuThread::bFlagSuspend, 1u8);
        let t = thread.addr();
        let during = Rc::new(Cell::new(0u8));
        let seen = during.clone();
        r.with(SEMAPHORE_WAIT, move |e, _| {
            seen.set(e.mem.u8(t + 0x49));
            eax(0)
        });
        loading_menu_thread_thread_proc(&mut r.e, thread);
        let order = r.order();
        let release = order.iter().position(|a| *a == SEMAPHORE_RELEASE).unwrap();
        assert_eq!(
            order[release..release + 3],
            [SEMAPHORE_RELEASE, SEMAPHORE_WAIT, BS_THREAD_SET_PRIORITY]
        );
        assert_eq!(r.calls_to(SEMAPHORE_RELEASE), vec![vec![t + 0x3c]]);
        assert_eq!(r.calls_to(SEMAPHORE_WAIT), vec![vec![t + 0x30]]);
        assert_eq!(
            r.calls_to(BS_THREAD_SET_PRIORITY),
            vec![vec![t, 0xffff_ffff]]
        );
        assert_eq!(during.get(), 1);
        assert_eq!(r.e.get(thread, LoadingMenuThread::bFlagSuspended), 0);
    }

    #[test]
    fn during_the_initial_load_a_finished_wheel_sequence_keeps_it_running() {
        let (mut r, menu, thread, _count) = proc_world(1);
        r.e.set(thread, LoadingMenuThread::bFlagSuspend, 1u8);
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        // The setting string is empty (the rig's string buffer is zero); the
        // model tile has a sequence whose state is 1.
        let model_tile = r.tile_at(menu, 0x28);
        r.e.mem.set_u32(model_tile + 0x3c, 0x4444);
        r.stub(SEQUENCE_STATE, 1);
        loading_menu_thread_thread_proc(&mut r.e, thread);
        assert!(r.calls_to(SEMAPHORE_RELEASE).is_empty());
        assert!(r
            .calls_to(SETTING_STRING)
            .contains(&vec![SETTING_MUSIC_VOLUME]));
        // A non-empty string suspends it after all.
        let (mut r, _menu, thread, _count) = proc_world(1);
        r.e.set(thread, LoadingMenuThread::bFlagSuspend, 1u8);
        r.e.mem.set_u8(INITIAL_LOAD, 1);
        r.e.mem.set_u8(0x7000, b'x');
        loading_menu_thread_thread_proc(&mut r.e, thread);
        assert_eq!(r.calls_to(SEMAPHORE_RELEASE).len(), 1);
    }

    // ----- 0078d5a0, 0078d670, 0078d6b0, 0078d770 -----

    /// A world for the level progress: owner `0x011dea3c`, level 5 with the
    /// cap 50, experience 100 and 300 for levels 5 and 6, and the actor value
    /// `current`.
    fn progress_world(current: f32) -> (Rig, u32) {
        let (mut r, _menu, _values) = second();
        let owner = r.e.mem.alloc(0x200);
        r.e.mem.set_u32(owner + 0xa4, owner + 0x100);
        r.e.mem.set_u32(owner + 0x100 + 0xc, 0x6c00);
        r.e.mem.set_u32(PLAYER_OWNER, owner);
        r.with(PLAYER_LEVEL, |_, _| eax(0xabcd_0005));
        r.e.mem.set_u32(0x7010, 50);
        r.with(REQUIRED_EXPERIENCE, |_, a| {
            eax(if a[0] == 5 { 100 } else { 300 })
        });
        r.with(0x6c00, move |_, _| st0(current));
        (r, owner)
    }

    #[test]
    fn the_level_progress_is_the_fraction_between_two_levels() {
        let (mut r, owner) = progress_world(200.0);
        assert_eq!(loading_menu_get_player_level_progress(&mut r.e), 0.5);
        assert_eq!(r.calls_to(PLAYER_LEVEL)[0], vec![owner]);
        assert_eq!(r.calls_to(REQUIRED_EXPERIENCE), vec![vec![5], vec![6]]);
        // The actor value query: this is the object at owner + 0xa4, with 0x18.
        assert_eq!(r.calls_to(0x6c00), vec![vec![owner + 0xa4, 0x18]]);
        assert!(r.calls_to(LOG_LINE).is_empty());
        assert_eq!(r.e.call(0x0078_d5a0, &args![]).f32(), 0.5);
    }

    #[test]
    fn experience_below_the_level_logs_and_gives_a_negative_fraction() {
        let (mut r, _owner) = progress_world(50.0);
        assert_eq!(loading_menu_get_player_level_progress(&mut r.e), -0.25);
        let call = &r.calls_to(LOG_LINE)[0];
        assert_eq!(call[..4], [0x12, LEVEL_PROGRESS_WARNING, 100, 300]);
        let current = f64::from_bits(call[4] as u64 | (call[5] as u64) << 32);
        assert_eq!(current, 50.0);
    }

    #[test]
    fn the_levels_at_the_cap_have_no_progress() {
        let (mut r, _owner) = progress_world(200.0);
        r.e.mem.set_u32(0x7010, 5);
        r.clear_calls();
        assert_eq!(loading_menu_get_player_level_progress(&mut r.e), 0.0);
        assert!(r.calls_to(REQUIRED_EXPERIENCE).is_empty());
        // The cap is compared as a signed number: a negative cap stops it too.
        r.e.mem.set_u32(0x7010, 0xffff_ffff);
        assert_eq!(loading_menu_get_player_level_progress(&mut r.e), 0.0);
    }

    #[test]
    fn the_player_state_is_cached_in_three_globals() {
        let (mut r, owner) = progress_world(200.0);
        r.stub(PLAYER_PROCESS, 0x5151);
        r.e.call(0x0078_d670, &args![]);
        assert_eq!(r.e.mem.u32(TIP_TEXT_OWNER), 0x5151);
        assert_eq!(r.e.mem.f32(TIP_SCALE), 0.5);
        assert_eq!(r.e.mem.u32(TIP_VALUE), 5);
        assert_eq!(r.calls_to(PLAYER_PROCESS), vec![vec![owner]]);
    }

    /// The chunks the save-buffer double was given, in order.
    fn saving(r: &mut Rig) -> Rc<RefCell<Vec<Vec<u8>>>> {
        let chunks: Rc<RefCell<Vec<Vec<u8>>>> = Rc::default();
        let store = chunks.clone();
        r.with(SAVE_BYTES, move |e, a| {
            let bytes = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            store.borrow_mut().push(bytes);
            eax(0)
        });
        chunks
    }

    #[test]
    fn saving_writes_the_form_the_handle_the_progress_the_level_and_the_lock() {
        let (mut r, _owner) = progress_world(200.0);
        let chunks = saving(&mut r);
        let process = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(process + 4, 0x2468);
        r.stub(PLAYER_PROCESS, process);
        r.stub(PROCESS_VALUE, 0x1357);
        r.e.mem.set_u8(LIST_LOCKED, 1);
        fn_0078d6b0(&mut r.e, 0xb0f0);
        assert_eq!(r.calls_to(SAVE_FORM_ID), vec![vec![0xb0f0, 0x1357, 0]]);
        assert_eq!(r.calls_to(PROCESS_VALUE), vec![vec![process]]);
        let chunks = chunks.borrow();
        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0], 0x2468u32.to_le_bytes());
        assert_eq!(chunks[1], 0.5f32.to_le_bytes());
        assert_eq!(chunks[2], 5u32.to_le_bytes());
        assert_eq!(chunks[3], vec![1]);
        for call in r.calls_to(SAVE_BYTES) {
            assert_eq!(call[0], 0xb0f0);
            assert_eq!(call[3], 0);
        }
        // The order: refresh (the process getter), the form, then the bytes.
        let order = r.order();
        let process_getter = order.iter().position(|a| *a == PLAYER_PROCESS).unwrap();
        let form = order.iter().position(|a| *a == SAVE_FORM_ID).unwrap();
        assert!(process_getter < form);
    }

    #[test]
    fn saving_without_an_object_writes_zero_ids() {
        let (mut r, _owner) = progress_world(200.0);
        let chunks = saving(&mut r);
        r.stub(PLAYER_PROCESS, 0);
        r.e.call(0x0078_d6b0, &args![0xb0f0u32]);
        assert_eq!(r.calls_to(SAVE_FORM_ID), vec![vec![0xb0f0, 0, 0]]);
        assert!(r.calls_to(PROCESS_VALUE).is_empty());
        assert_eq!(chunks.borrow()[0], 0u32.to_le_bytes());
        assert_eq!(chunks.borrow()[3], vec![0]);
    }

    /// A load buffer double that serves `chunks` in order.
    fn loading(r: &mut Rig, chunks: Vec<Vec<u8>>) {
        let mut queue = chunks.into_iter();
        r.with(LOAD_BYTES, move |e, a| {
            let bytes = queue.next().unwrap();
            assert_eq!(bytes.len() as u32, a[2]);
            for (i, byte) in bytes.iter().enumerate() {
                e.mem.set_u8(a[1] + i as u32, *byte);
            }
            eax(0)
        });
    }

    #[test]
    fn loading_restores_the_state_and_shows_the_wheel() {
        let (mut r, _owner) = progress_world(200.0);
        let menu = r.menu();
        r.e.mem.set_u32(MENU_INSTANCE, menu.addr());
        let wheel = r.object(0x40);
        r.e.mem.set_u32(menu.addr() + 0x54, wheel);
        r.e.mem.set_u16(menu.addr() + 0x222, bit(8));
        r.real_flags();
        loading(
            &mut r,
            vec![
                0x77u32.to_le_bytes().to_vec(),
                0.75f32.to_le_bytes().to_vec(),
                7u32.to_le_bytes().to_vec(),
                vec![3],
            ],
        );
        r.stub(LOAD_FORM_ID, 0x1111);
        r.stub(FORM_LOOKUP, 0x2222);
        r.stub(RT_DYNAMIC_CAST, 0x3333);
        r.stub(OWNER_BY_HANDLE, 0x4444);
        fn_0078d770(&mut r.e, 0xb0f0);
        assert_eq!(r.calls_to(LOAD_FORM_ID), vec![vec![0xb0f0]]);
        assert_eq!(r.calls_to(FORM_LOOKUP), vec![vec![0x1111]]);
        assert_eq!(
            r.calls_to(RT_DYNAMIC_CAST),
            vec![vec![0x2222, 0, 0x0118_3028, 0x0118_6500, 0]]
        );
        assert_eq!(r.calls_to(OWNER_BY_HANDLE), vec![vec![0x3333, 0x77]]);
        assert_eq!(r.e.mem.u32(TIP_TEXT_OWNER), 0x4444);
        assert_eq!(r.e.mem.f32(TIP_SCALE), 0.75);
        assert_eq!(r.e.mem.u32(TIP_VALUE), 7);
        assert_eq!(r.calls_to(APPLY_SAVED_BYTE), vec![vec![3]]);
        assert_eq!(r.calls_to(AFTER_LOAD), vec![vec![1]]);
        // The wheel is shown and state bit 8 cleared.
        assert_eq!(
            r.calls_to(TILE_SET_INT).last().unwrap()[..3],
            [wheel, 0xfa3, 1]
        );
        assert_eq!(r.e.get(menu, LoadingMenu::usFlags) & bit(8), 0);
    }

    #[test]
    fn loading_resets_the_progress_at_the_cap_and_copes_without_owner_or_menu() {
        let (mut r, _owner) = progress_world(200.0);
        r.e.mem.set_u32(MENU_INSTANCE, 0);
        loading(
            &mut r,
            vec![
                0x77u32.to_le_bytes().to_vec(),
                0.75f32.to_le_bytes().to_vec(),
                50u32.to_le_bytes().to_vec(),
                vec![0],
            ],
        );
        r.stub(RT_DYNAMIC_CAST, 0);
        r.e.call(0x0078_d770, &args![0xb0f0u32]);
        assert!(r.calls_to(OWNER_BY_HANDLE).is_empty());
        assert_eq!(r.e.mem.u32(TIP_TEXT_OWNER), 0);
        assert_eq!(r.e.mem.f32(TIP_SCALE), 0.0);
        assert!(r.calls_to(TILE_SET_INT).is_empty());
        assert_eq!(r.calls_to(AFTER_LOAD).len(), 1);
    }

    // ----- 0078d880, 0078d8b0, 0078d8d0 -----

    #[test]
    fn the_candidate_array_is_initialized_and_released() {
        let (mut r, _menu, _values) = second();
        r.stub(CANDIDATE_ARRAY_INIT, 0);
        let array = r.e.mem.alloc(0x10);
        assert_eq!(
            r.e.call(0x0078_d880, &args![Ptr::<()>::new(array)]).u32(),
            array
        );
        assert_eq!(r.e.mem.u32(array), 0x0107_410c);
        assert_eq!(r.calls_to(CANDIDATE_ARRAY_INIT), vec![vec![array, 0, 0]]);
        r.clear_calls();
        r.e.mem.set_u32(array, 0);
        r.e.call(0x0078_d8b0, &args![Ptr::<()>::new(array)]);
        assert_eq!(r.e.mem.u32(array), 0x0107_410c);
        assert_eq!(r.calls_to(CANDIDATES_RESET), vec![vec![array, 1]]);
    }

    #[test]
    fn the_candidate_array_scalar_deleting_destructor_frees_on_flag_1() {
        let (mut r, _menu, _values) = second();
        let array = r.e.mem.alloc(0x10);
        let result =
            r.e.call(0x0078_d8d0, &args![Ptr::<()>::new(array), 1u32])
                .u32();
        assert_eq!(result, array);
        assert_eq!(r.order(), vec![CANDIDATES_RESET, OPERATOR_DELETE]);
        assert_eq!(r.calls_to(OPERATOR_DELETE), vec![vec![array]]);
        r.clear_calls();
        bs_simple_array_tes_load_screen_scalar_deleting_destructor(&mut r.e, Ptr::new(array), 0);
        assert!(r.calls_to(OPERATOR_DELETE).is_empty());
    }
}
