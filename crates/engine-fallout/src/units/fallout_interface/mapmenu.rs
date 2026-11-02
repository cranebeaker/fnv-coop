//! `fallout/interface/menus/mapmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSStringT@D@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01075530: u32 = 0x0107_5530;
/// `007a1c30` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_007A1C30: u32 = 0x007a_1c30;

// Translated from 007a1aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSStringT@D@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `007a1c30(this, 1)`.
pub fn fn_007a1aa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01075530);
    e.call(ARRAY_RELEASE_007A1C30, &args![this, 1u32]);
}

use crate::types::BSSimpleArray;
use crate::units::fallout_interface::startmenu::ListBox;

layout! {
    /// `MapMenu` (Xbox PDB), 0x230 bytes; the PC constructor and destructor
    /// use the Xbox offsets. `pTiles` (22 tile pointers) starts at +0x28 and
    /// is read through [`map_tile`].
    pub struct MapMenu: 0x230 {
        /// `iCurrentPage` (Xbox PDB): the tab (0x20 to 0x24).
        0x80 iCurrentPage: i32,
        /// `pMapTile` (Xbox PDB).
        0x84 pMapTile: u32,
        /// `fWaveformAnimationStartTime` (Xbox PDB).
        0x88 fWaveformAnimationStartTime: f32,
        /// `bShowingAllNotes` (Xbox PDB).
        0x8C bShowingAllNotes: u8,
        /// `pSelectedNote` (Xbox PDB).
        0x90 pSelectedNote: u32,
        /// `uiNoteEnterStartTime` (Xbox PDB).
        0x94 uiNoteEnterStartTime: u32,
        /// `pCurrentSound` (Xbox PDB): a `BSSimpleList<BSSoundHandle>` node.
        0xB8 pCurrentSound: u32,
        /// `bSoundsLoading` (Xbox PDB).
        0xBC bSoundsLoading: u8,
        /// `iSoundPlayStartTime` (Xbox PDB).
        0xC4 iSoundPlayStartTime: u32,
        /// `fOldPipboyRadioVolume` (Xbox PDB).
        0xC8 fOldPipboyRadioVolume: f32,
        /// `pDisplayedQuest` (Xbox PDB).
        0xCC pDisplayedQuest: u32,
        /// `pDisplayedQuestTarget` (Xbox PDB).
        0xD0 pDisplayedQuestTarget: u32,
        /// `pHighlightedMapMarker` (Xbox PDB).
        0xD4 pHighlightedMapMarker: u32,
        /// `PlayerMarkerSpace` (Xbox PDB).
        0xF8 PlayerMarkerSpace: u32,
        /// `PlayerMarkerPos` (Xbox PDB): `NiPoint3`, x (y at +0x100, z at
        /// +0x104).
        0xFC PlayerMarkerPos: f32,
        /// `pMapSpace` (Xbox PDB).
        0x108 pMapSpace: u32,
        /// `pRootMapSpace` (Xbox PDB).
        0x10C pRootMapSpace: u32,
        /// `bRootSpaceChanged` (Xbox PDB).
        0x110 bRootSpaceChanged: u8,
        /// `pMapPlayerRef` (Xbox PDB).
        0x114 pMapPlayerRef: u32,
        /// `pFastTravelMarker` (Xbox PDB).
        0x118 pFastTravelMarker: u32,
        /// `pAdjustNorthRotationCell` (Xbox PDB).
        0x11C pAdjustNorthRotationCell: u32,
        /// `WorldMapDragStart` (Xbox PDB): `NiPoint2`, x (y at +0x124).
        0x120 WorldMapDragStart: f32,
        /// `uiNextLocalMapGrid` (Xbox PDB).
        0x128 uiNextLocalMapGrid: u32,
        /// `bFogOfWarEnabled` (Xbox PDB).
        0x12C bFogOfWarEnabled: u8,
    }
}

/// `MapMenu::pMe` (Xbox PDB static): the open map menu, or 0.
const MAP_MENU_SINGLETON: u32 = 0x011d_a368;
/// `MapMenu`'s vtable (RTTI `.?AVMapMenu@@`).
const MAP_MENU_VTABLE: u32 = 0x0107_4d44;
/// The byte `ClearWorldMapTexture` sets once it has cleared the texture.
const TEXTURE_CLEARED_011DA36C: u32 = 0x011d_a36c;
/// A byte that selects the challenge list over the notes list (the Xbox PDB
/// has a static `bNotesChallengeMode` in `MapMenu`).
const CHALLENGE_MODE_011DA36D: u32 = 0x011d_a36d;
/// A signed byte the challenge filter steps through 0, 1, 2 (the Xbox PDB has
/// a static `cChallengeTypeDisplayed`).
const CHALLENGE_TYPE_011DA36E: u32 = 0x011d_a36e;
/// The byte `DoIdle` sets when it pauses the note sound.
const SOUND_PAUSED_011DA36F: u32 = 0x011d_a36f;
/// The time (`00825c00` on `011f6394`) `DoIdle` keeps when it pauses the note
/// sound.
const PAUSE_TIME_011DA40C: u32 = 0x011d_a40c;
/// The byte `DoIdle` sets after giving the pip-boy tile trait `0xfa9` 255.0.
const FADED_011DA408: u32 = 0x011d_a408;
/// The table of 403df0 objects (one per challenge type) at `011a03e4`.
const CHALLENGE_TYPE_NAMES_011A03E4: u32 = 0x011a_03e4;
/// The word `007a1480` returns.
const VALUE_011DEA14: u32 = 0x011d_ea14;
/// The player (`PlayerCharacter *` the code passes to `SetActiveQuest`).
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
/// The object `00877720` is called on in `DoIdle`.
const OBJECT_011DEA0C: u32 = 0x011d_ea0c;
/// The calendar (`Calendar` at `011de7b8`).
const CALENDAR_011DE7B8: u32 = 0x011d_e7b8;
/// The timer object `00825c00` is called on.
const TIMER_011F6394: u32 = 0x011f_6394;
/// The three floats the constructor copies into `PlayerMarkerPos`.
const POINT_011F426C: u32 = 0x011f_426c;
/// The objects whose word +4 (`00403df0`) `Create` passes to the tab line,
/// in the order the code reads them.
const TABLINE_ENTRIES: [u32; 5] = [
    0x011d_4aec,
    0x011d_28a4,
    0x011d_5020,
    0x011d_204c,
    0x011d_38ac,
];

/// The trait ids the constructor looks up (`MapMenu` statics `iUT_Title`,
/// `iUT_Value`, `iUT_CurrentTab`, `iUT_Selected`, `iUT_TextAlpha`, `iUT_X`,
/// `iUT_Y`, `iUT_Magnification` in the Xbox PDB).
const TRAIT_TITLE: u32 = 0x011d_a354;
const TRAIT_VALUE: u32 = 0x011d_a358;
const TRAIT_CURRENT_TAB: u32 = 0x011d_a360;
const TRAIT_SELECTED: u32 = 0x011d_a348;
const TRAIT_TEXT_ALPHA: u32 = 0x011d_a344;
const TRAIT_X: u32 = 0x011d_a350;
const TRAIT_Y: u32 = 0x011d_a34c;
const TRAIT_MAGNIFICATION: u32 = 0x011d_a35c;

/// `"_Title"`, `"_Value"`, `"_CurrentTab"`, `"_selected"`, `"_TextAlpha"`,
/// `"_x"`, `"_y"`, `"_Magnification"`.
const TITLE_TEXT: u32 = 0x0106_ed20;
const VALUE_TEXT: u32 = 0x0106_ed18;
const CURRENT_TAB_TEXT: u32 = 0x0107_3994;
const SELECTED_TEXT: u32 = 0x0107_1b64;
const TEXT_ALPHA_TEXT: u32 = 0x0107_3cb4;
const X_COORD_TEXT: u32 = 0x0106_ec90;
const Y_COORD_TEXT: u32 = 0x0106_f8f4;
const MAGNIFICATION_TEXT: u32 = 0x0107_4d2c;
/// `"Data\\Menus\\Main\\map_menu.xml"`.
const MAP_MENU_XML: u32 = 0x0107_4df8;
/// `"MENUS: Map Menu Creation Failed... Are your menu and art resources up
/// to date?"`.
const CREATION_FAILED_TEXT: u32 = 0x0107_4da8;
/// `"MM_ListMarkerTemplate"`.
const LIST_MARKER_TEMPLATE: u32 = 0x0107_4d8c;
/// `""`.
const EMPTY_TEXT: u32 = 0x0101_1584;
/// `"%s, %d:%02d"`.
const DATE_TIME_FORMAT: u32 = 0x0107_4fdc;
/// `"ENGLISH"`.
const ENGLISH_TEXT: u32 = 0x0101_8348;
/// `"_Title_1"` to `"_Title_5"`.
const TITLE_N_TEXT: [u32; 5] = [
    0x0107_3d30,
    0x0107_4fd0,
    0x0107_4fc4,
    0x0107_4fb8,
    0x0107_4fac,
];
/// `"MM_Headline_Title"`.
const HEADLINE_TITLE_TEXT: u32 = 0x0107_4f98;
/// `"UIPipBoyHolotapeStart"`.
const HOLOTAPE_START_SOUND: u32 = 0x0107_4e78;
/// `"UIPopUpMapMarkerAdded "`.
const MARKER_ADDED_SOUND: u32 = 0x0107_4e4c;
/// `"_ShowingAllNotes"`.
const SHOWING_ALL_NOTES_TEXT: u32 = 0x0107_4e64;
/// `"_MarkerIndex"`.
const MARKER_INDEX_TEXT: u32 = 0x0107_4e3c;
/// `"%s %s?"`.
const FAST_TRAVEL_FORMAT: u32 = 0x0107_4e34;
/// `"Companion"`.
const COMPANION_TEXT: u32 = 0x0107_4e28;
/// `"_LocationName"`.
const LOCATION_NAME_TEXT: u32 = 0x0107_4e18;
/// `"_current_value"` (`ListBox`).
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;

/// `6006.0` and `102.0` (`double`): the two `0xfa7` values `Create` checks.
const CREATE_VALUE_A: u32 = 0x0107_04f0;
const CREATE_VALUE_B: u32 = 0x0107_04e8;
/// `-1.0` (`float`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `2.0` (`double`).
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `60.0` (`double`).
const SIXTY_DOUBLE: u32 = 0x0101_2638;
/// `850.0` and `500.0` (`double`): the map area `DoIdle` checks.
const MAP_WIDTH_DOUBLE: u32 = 0x0107_4f68;
const MAP_HEIGHT_DOUBLE: u32 = 0x0103_01a8;
/// `255.0` (`float`).
const FULL_ALPHA_FLOAT: u32 = 0x0102_3cd8;
/// `255.0` (`double`).
const FULL_ALPHA_DOUBLE: u32 = 0x0101_e568;
/// `1000.0` (`double`).
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
/// `128.0` (`float`).
const HALF_ALPHA_FLOAT: u32 = 0x0101_e704;
/// `0.9` (`float`): the note sound volume.
const NOTE_VOLUME_FLOAT: u32 = 0x0101_77e8;
/// `2.0` (`float`).
const TWO_FLOAT: u32 = 0x0101_62c0;
/// `0.5` (`float`).
const HALF_FLOAT: u32 = 0x0101_6248;
/// The two floats `DoClick` reads through `00403e20` for the tab burst.
const BURST_SETTING_A: u32 = 0x011d_b37c;
const BURST_SETTING_B: u32 = 0x011d_b458;

/// `Interface::IsTopMenuID` (engine map), cdecl (menu id): `AL`.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `Interface::GetPipboy` (engine map), cdecl: an object.
const GET_PIPBOY: u32 = 0x0070_5990;
/// Thiscall on the pip-boy object: `AL`.
const PIPBOY_FLAG_00974D90: u32 = 0x0097_4d90;
/// Thiscall on the pip-boy object: `AL`.
const PIPBOY_FLAG_007FAEF0: u32 = 0x007f_aef0;
/// Cdecl, no arguments: the interface manager (`011d8a80`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
/// Cdecl, no arguments: `AL`.
const FLAG_004B71D0: u32 = 0x004b_71d0;
/// Thiscall: the word at +0x28.
const WORD_AT_28: u32 = 0x0045_cd60;
/// Thiscall: the word at +0x174.
const WORD_AT_174: u32 = 0x0070_4370;
/// Thiscall on a tile (trait id, integer), `RET 8`.
const TILE_SET_INT: u32 = 0x0070_0320;
/// Thiscall on a tile (trait id), `RET 4`: a float in `ST0`.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// Thiscall on a tile (trait id, float, word), `RET 0xC`.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// Thiscall on a tile (trait id, text, word), `RET 0xC`.
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::GetString` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// `Tile::IsTrue` (engine map), thiscall (trait id), `RET 4`: `AL`.
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetValue` (engine map), thiscall (trait id), `RET 4`.
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::Value::CalculateValue` (engine map), thiscall (word), `RET 4`.
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `Tile::TextToTrait` (engine map), cdecl (name).
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::AddUserTrait` (engine map), cdecl (name, word).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::GetMenu` (engine map), thiscall.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetChildByName` (engine map), thiscall (name), `RET 4`.
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::GetChildByID` (engine map), thiscall (id), `RET 4`.
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
/// Cdecl (tile, name): a tile found by name below it.
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
/// `Tile::DeleteChildren` (engine map), thiscall.
const TILE_DELETE_CHILDREN: u32 = 0x00a0_4150;
/// `Tile::RemoveFadeControl` (engine map), thiscall (trait id), `RET 4`.
const TILE_REMOVE_FADE_CONTROL: u32 = 0x00a0_7dc0;
/// `Tile::GetMenuByClass` (engine map), cdecl (class).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::ReadFile` (engine map), thiscall (path), `RET 4`.
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMaximumDepth` (engine map), cdecl: a float in `ST0`.
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// Thiscall on a tile (`tile.cpp`): a float in `ST0` (with `00a01440`, the
/// two the map position code subtracts from x and y).
const TILE_GETTER_00A013D0: u32 = 0x00a0_13d0;
const TILE_GETTER_00A01440: u32 = 0x00a0_1440;
/// Thiscall on a tile (byte), `RET 4`.
const TILE_FLAG_SETTER_0071AA60: u32 = 0x0071_aa60;
/// Thiscall on a tile: `AL`.
const TILE_FLAG_GETTER_0071AA80: u32 = 0x0071_aa80;
/// `Tile::AddNeedsUpdate` (engine map), thiscall (flags), `RET 4`.
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Tile::Value::AddAction_ov2` (engine map), thiscall (action, tile, trait
/// id), `RET 0xC`.
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
/// `Menu::RemoveFromXMLArchive` (engine map), cdecl (path, menu).
const MENU_REMOVE_FROM_XML_ARCHIVE: u32 = 0x00a1_ca20;
/// `Menu::SetMenuTile` (engine map), thiscall (tile, word), `RET 8`.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (engine map), thiscall (word), `RET 4`.
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The `Menu` constructor and destructor bodies (thiscall).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
/// Thiscall: the word at +4 (on a list node: the next node; on the menu: the
/// tile the code reads children of).
const WORD_AT_4: u32 = 0x0072_6070;
/// Thiscall: returns `this` (on a list node: the address of its item).
const SELF: u32 = 0x0068_15c0;
/// Thiscall: the word at +4 of a non-null object, 0 for null.
const OBJECT_FIELD_00403DF0: u32 = 0x0040_3df0;
/// Thiscall: the address of the float the object holds (`effectitem.cpp`).
const OBJECT_VALUE_ADDRESS_00403E20: u32 = 0x0040_3e20;
/// Thiscall on a string (`BSStringT`): its characters.
const STRING_TEXT: u32 = 0x0055_9450;
/// String constructor and destructor (8-byte `BSStringT`), thiscall.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// Cdecl (string, format, ...): formats into the string.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// Cdecl (buffer, size, format, ...): formats into a character buffer.
const BUFFER_FORMAT: u32 = 0x0040_6d00;
/// Cdecl (text, text): `EAX` (the language check of `UpdateMenu`).
const COMPARE_00408B20: u32 = 0x0040_8b20;
/// `__stricmp`, cdecl.
const STRICMP: u32 = 0x00ec_68e4;
/// The compiler's float to integer conversion (`_ftol2`): the value in
/// `ST0` (a leading `f64`), the integer in `EAX`.
const FTOL2: u32 = 0x00ec_62c0;
/// The allocator and its free, cdecl.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// Cdecl (text): the warning print `CreationFailed` passes its message to.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// Thiscall on the timer: the current time (`EAX`).
const TIMER_NOW: u32 = 0x0082_5c00;
/// `BSAudio::QInstance` (engine map), cdecl.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetRadioVolume` (engine map), thiscall: a float in `ST0`.
const AUDIO_GET_RADIO_VOLUME: u32 = 0x00ad_7ae0;
/// `BSAudio::GetSoundHandleByName` (engine map), thiscall (result, name,
/// flags), `RET 0xC`: the result.
const AUDIO_SOUND_BY_NAME: u32 = 0x00ad_7550;
/// `BSAudio::GetSoundHandleByNumericID` (engine map), thiscall (result, id,
/// flags), `RET 0xC`.
const AUDIO_SOUND_BY_ID: u32 = 0x00ad_73b0;
/// `BSAudio::GetSoundHandleByFilename` (engine map), thiscall (result, name,
/// flags, word), `RET 0x10`.
const AUDIO_SOUND_BY_FILENAME: u32 = 0x00ad_7480;
/// `BSAudio::EnterDialogue` (engine map), thiscall.
const AUDIO_ENTER_DIALOGUE: u32 = 0x00ad_85a0;
/// `BSSoundHandle::Play` (engine map), thiscall (word), `RET 4`.
const SOUND_PLAY: u32 = 0x00ad_8830;
/// `BSSoundHandle::Pause` (engine map), thiscall.
const SOUND_PAUSE: u32 = 0x00ad_88b0;
/// `BSSoundHandle::IsPlaying` (engine map), thiscall: `AL`.
const SOUND_IS_PLAYING: u32 = 0x00ad_8930;
/// `BSSoundHandle::SetVolume` (engine map), thiscall (float), `RET 4`.
const SOUND_SET_VOLUME: u32 = 0x00ad_89e0;
/// The sound handle constructor and destructor (12-byte handle), thiscall.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// Thiscall on a sound handle (handle), `RET 4`: the assignment.
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSimpleList<BSSoundHandle>::AddTail` (engine map), thiscall (handle),
/// `RET 4`.
const SOUND_LIST_ADD_TAIL: u32 = 0x007a_19a0;
/// The `BSSimpleList<BSSoundHandle>` constructor and destructor bodies.
const SOUND_LIST_CONSTRUCT: u32 = 0x0076_b930;
const SOUND_LIST_DESTRUCT: u32 = 0x0076_b8a0;
/// List constructor (`BSSimpleList`), thiscall.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// List destructor, thiscall.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Thiscall on a list: removes its items.
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// The constructor and destructor of the array at +0x220 (`debugWaypoints`).
const WAYPOINTS_CONSTRUCT: u32 = 0x0071_9b80;
const WAYPOINTS_DESTRUCT: u32 = 0x0071_9be0;
/// Thiscall on the array at +0x220 (word), `RET 4`.
const WAYPOINTS_CLEAR: u32 = 0x0084_54f0;
/// `Calendar::GetHour` (`00867da0`) and `Calendar::GetDateString`
/// (`00867970`), thiscall.
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR_GET_DATE_STRING: u32 = 0x0086_7970;
/// Thiscall on the calendar: marks the days passed for a rewrite.
const CALENDAR_MARK_STALE: u32 = 0x0086_7a20;
/// Thiscall on a tile: an object with a vtable (`tesobjectrefr.cpp`).
const TILE_OBJECT_0056C7F0: u32 = 0x0056_c7f0;
/// Thiscall on that object: a count, and (index) an item, `RET 4`.
const OBJECT_COUNT_0043B480: u32 = 0x0043_b480;
const OBJECT_ITEM_0043B4A0: u32 = 0x0043_b4a0;
/// Thiscall (word), `RET 4` (`tes.cpp`).
const OBJECT_NODE_0045BC00: u32 = 0x0045_bc00;
/// Cdecl, no arguments: 3 (the property type `GetProperty` gets).
const PROPERTY_TYPE_00438220: u32 = 0x0043_8220;
/// `NiAVObject::GetProperty` (engine map), thiscall (type), `RET 4`.
const AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// Thiscall on the property: `EAX`.
const PROPERTY_CHECK_009C8640: u32 = 0x009c_8640;
/// `TileShaderProperty::SetTileTexture` (engine map), thiscall (texture),
/// `RET 4`.
const SET_TILE_TEXTURE: u32 = 0x00bb_7a10;
/// Thiscall on the property (word), `RET 4` (`hudmainmenu.cpp`).
const PROPERTY_SET_007700D0: u32 = 0x0077_00d0;
/// `TESWorldSpace::GetParentWorldSimple` (engine map), thiscall.
const WORLD_GET_PARENT: u32 = 0x004f_d380;
/// Thiscall on a list box (word), `RET 4` (`statsmenu.cpp`): the item of a
/// tile.
const LIST_BOX_ITEM_OF_007E1110: u32 = 0x007e_1110;
/// Thiscall (buffer, size), `RET 8`: constructs `size` array elements.
const ARRAY_CONSTRUCT_ELEMENTS_007A6310: u32 = 0x007a_6310;
/// Thiscall (buffer, size), `RET 8`: destroys array elements.
const ARRAY_DESTROY_ELEMENTS_007A63B0: u32 = 0x007a_63b0;
/// Thiscall on the array: frees its buffer.
const ARRAY_FREE_006A8500: u32 = 0x006a_8500;
/// Thiscall on a `Conversation`: its destructor body.
const CONVERSATION_DESTRUCT: u32 = 0x0083_b8d0;
/// `ListBox<unsigned int>::Initialize` (engine map), thiscall (tile, template
/// name), `RET 8`.
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// `Interface::InitializeTabline` (engine map), cdecl (tile, count, ...).
const INITIALIZE_TABLINE: u32 = 0x0070_7be0;
/// `InterfaceManager::TutorialManager::ShowMessage` (engine map), thiscall
/// (message, menu id, flags), `RET 0xC`.
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;
/// Cdecl: `0x3ff` (the map menu's id).
const MAP_MENU_ID_00720DF0: u32 = 0x0072_0df0;
/// `MapMenu::UpdateCurrentPage` (engine map), thiscall.
const UPDATE_CURRENT_PAGE: u32 = 0x0079_af30;
/// `MapMenu::UpdatePlayingSounds` (engine map), thiscall.
const UPDATE_PLAYING_SOUNDS: u32 = 0x0079_a660;
/// `MapMenu::LoadNote` (engine map), thiscall (note), `RET 4`.
const LOAD_NOTE: u32 = 0x0079_93d0;
/// `MapMenu::StopNoteSound` (engine map), thiscall.
const STOP_NOTE_SOUND: u32 = 0x0079_8ad0;
/// `MapMenu::ToggleNotesChallengeMode` (engine map), thiscall (word), `RET 4`.
const TOGGLE_NOTES_CHALLENGE_MODE: u32 = 0x0079_b460;
/// `MapMenu::MapToWorldCoord` (engine map), thiscall (result, x, y, z, world
/// space), `RET 0x14`.
const MAP_TO_WORLD_COORD: u32 = 0x0079_c450;
/// `MapMenu::WorldToMapCoord` (engine map), thiscall (result, x, y, z, world
/// space), `RET 0x14`.
const WORLD_TO_MAP_COORD: u32 = 0x0079_c380;
/// `MapMenu::CalculatePlayerMarkerSpace` (engine map), thiscall (x, y, z),
/// `RET 0xC`.
const CALCULATE_PLAYER_MARKER_SPACE: u32 = 0x0079_84d0;
/// `MapMenu::GetWorldMapTrackingRef` (engine map), thiscall (reference,
/// word), `RET 8`.
const GET_WORLD_MAP_TRACKING_REF: u32 = 0x0079_f7e0;
/// Thiscall on the menu: shows the map position (`DoIdle`).
const SHOW_POSITION_00799DC0: u32 = 0x0079_9dc0;
/// Thiscall on the hacking-menu object: a float in `ST0` (x), and on the map
/// menu object (`007983c0`): y.
const CURSOR_X_007697F0: u32 = 0x0076_97f0;
const CURSOR_Y_007983C0: u32 = 0x0079_83c0;
/// Cdecl (word) (`mapmenu.cpp`): the `DoIdle` step it runs 25 times.
const MAP_STEP_0079FFB0: u32 = 0x0079_ffb0;
/// Cdecl: `AL` (`mapmenu.cpp`).
const FLAG_00798440: u32 = 0x0079_8440;
/// Thiscall on the menu (`mapmenu.cpp`).
const PAGE_0079C0C0: u32 = 0x0079_c0c0;
/// Thiscall on a note: `AL` (`mapmenu.cpp`).
const NOTE_FLAG_00798420: u32 = 0x0079_8420;
/// Thiscall on the player: `EAX` (`mapmenu.cpp`).
const PLAYER_FLAG_00798400: u32 = 0x0079_8400;
/// Cdecl (word) (`mapmenu.cpp`).
const TAB_KNOB_0079C340: u32 = 0x0079_c340;
/// Thiscall on `011dea0c` (`main.cpp`): an object.
const OBJECT_00877720: u32 = 0x0087_7720;
/// Thiscall on that object (word, word), `RET 8`: `EAX`.
const OBJECT_CHECK_00A23A50: u32 = 0x00a2_3a50;
const OBJECT_CHECK_00A24660: u32 = 0x00a2_4660;
/// Thiscall: the byte `DoIdle` reads (`011da3b8`).
const BYTE_OF_00408D60: u32 = 0x0040_8d60;
/// The object `00408d60` is called on.
const OBJECT_011DA3B8: u32 = 0x011d_a3b8;
/// `ListBox<StartMenu::Option_P>::GetTileFor` (engine map; folded code),
/// thiscall (address of the item), `RET 4`.
const LIST_BOX_GET_TILE_FOR: u32 = 0x007a_22d0;
/// Thiscall on a note (word), `RET 4` (`bgsnote.cpp`).
const NOTE_SET_READ_005E9300: u32 = 0x005e_9300;
/// Thiscall on a note: its type (`bgsnote.cpp`).
const NOTE_TYPE: u32 = 0x005e_8d40;
/// Thiscall on a note: its sound (`bgsnote.cpp`).
const NOTE_SOUND_005E9100: u32 = 0x005e_9100;
/// Thiscall on a note: its topic (`bgsnote.cpp`).
const NOTE_TOPIC_005E9240: u32 = 0x005e_9240;
/// `BGSNote::GetNoteSpeaker` (engine map), thiscall.
const NOTE_GET_SPEAKER: u32 = 0x005e_92a0;
/// Thiscall: the word at +0xC.
const WORD_AT_C: u32 = 0x0084_e3a0;
/// Thiscall: `this + 0x10`.
const PLUS_10: u32 = 0x0046_0140;
/// `Character::Character_ov2` (engine map), thiscall (word), `RET 4`.
const CHARACTER_CONSTRUCT: u32 = 0x008d_1f40;
/// `TESForm::SetTemporary` (engine map), thiscall.
const FORM_SET_TEMPORARY: u32 = 0x0048_4490;
/// `TESObjectREFR::SetObjectReference` (engine map), thiscall (form),
/// `RET 4`.
const REFR_SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
/// `Conversation::Conversation_ov2` (engine map), thiscall (speaker, player,
/// topic), `RET 0xC`.
const CONVERSATION_CONSTRUCT: u32 = 0x0083_b850;
/// `Conversation::FirstItem` (engine map), thiscall.
const CONVERSATION_FIRST_ITEM: u32 = 0x0083_b9a0;
/// `DialogueItem::GetCurrentResponse` (engine map), thiscall.
const GET_CURRENT_RESPONSE: u32 = 0x0083_c820;
/// `DialogueItem::FirstResponse` / `NextResponse` (engine map), thiscall:
/// `AL`.
const FIRST_RESPONSE: u32 = 0x0083_c7b0;
const NEXT_RESPONSE: u32 = 0x0083_c7e0;
/// Thiscall on the subtitle array at +0xA8 (address of the text), `RET 4`.
const SUBTITLES_ADD_007A1AC0: u32 = 0x007a_1ac0;
/// `TESTopicInfo::RunResult` (engine map), thiscall (word, speaker), `RET 8`.
const TOPIC_INFO_RUN_RESULT: u32 = 0x0061_f170;
/// Cdecl (word) (`dialogmenu.cpp`).
const DIALOG_FLAG_00762150: u32 = 0x0076_2150;
/// `PlayerCharacter::SetActiveQuest` (engine map), thiscall (quest), `RET 4`.
const SET_ACTIVE_QUEST: u32 = 0x0095_29d0;
/// Thiscall on a quest: `AL` (`tesconditionfunctions.cpp`).
const QUEST_FLAG_0059E400: u32 = 0x0059_e400;
/// `Interface::PlayMenuSound` (engine map), cdecl (sound).
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
/// Thiscall on the radio list box (callback, word, word), `RET 0xC`
/// (`bartermenu.cpp`).
const LIST_BOX_FILTER_007314C0: u32 = 0x0073_14c0;
/// The radio list filter callback the code passes (`0079be80`).
const RADIO_FILTER_CALLBACK: u32 = 0x0079_be80;
/// `FalloutRadio::PipboyRadioEnable` (engine map), cdecl (word).
const RADIO_ENABLE: u32 = 0x0083_24e0;
/// Cdecl (station, word) (`falloutradio.cpp`).
const RADIO_SET_STATION_00832240: u32 = 0x0083_2240;
/// `ListBox<BGSNote_P>::Filter` (`007a2e30`) and
/// `ListBox<TESChallenge_P>::Filter` (`007a5730`) (engine map), thiscall
/// (callback), `RET 4`, with the callbacks the code passes.
const NOTE_LIST_FILTER: u32 = 0x007a_2e30;
const NOTE_FILTER_CALLBACK: u32 = 0x0079_b390;
const CHALLENGE_LIST_FILTER: u32 = 0x007a_5730;
const CHALLENGE_FILTER_CALLBACK: u32 = 0x0079_b720;
/// Thiscall on a 12-byte point (x, y, z), `RET 0xC`: the point.
const POINT_SET_00416870: u32 = 0x0041_6870;
/// Thiscall on a point (result, float), `RET 8`: the result (`tesland.cpp`).
const POINT_SCALE_0053D280: u32 = 0x0053_d280;
/// Thiscall on a form: its type byte (+4).
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESWorldSpace::AdjustMapMarkerCoord` (engine map), thiscall (point,
/// word), `RET 8`.
const ADJUST_MAP_MARKER_COORD: u32 = 0x0058_8e40;
/// `TESObjectCELL::AdjustCoordForNorthRotation` (engine map), thiscall
/// (point, point, word), `RET 0xC`.
const ADJUST_FOR_NORTH_ROTATION: u32 = 0x0055_5b10;
/// Cdecl (text, word, word, callback, word, word, float, float, ...): the
/// message box the map shows (`interface.cpp`).
const SHOW_MESSAGE_BOX: u32 = 0x0070_3e80;
/// The callbacks the message boxes get (`SetPlayerMarkerCallback` and the
/// fast travel one).
const SET_PLAYER_MARKER_CALLBACK: u32 = 0x0079_8840;
const FAST_TRAVEL_CALLBACK: u32 = 0x0079_8710;
/// `TESQuestTarget::GetReference` (engine map), thiscall (word), `RET 4`.
const QUEST_TARGET_GET_REFERENCE: u32 = 0x0061_01b0;
/// `TESObjectREFR::GetWorldSpace` (engine map), thiscall.
const REFR_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// Thiscall on the player: `AL` (`playercharacter.cpp`).
const PLAYER_FLAG_0093D660: u32 = 0x0093_d660;
/// `TESObjectREFR::GetMapMarkerData` (engine map), thiscall.
const REFR_GET_MAP_MARKER_DATA: u32 = 0x0056_9060;
/// `MapMarkerData::GetTravelLoc` (engine map), thiscall: `AL`.
const MARKER_GET_TRAVEL_LOC: u32 = 0x0043_8ef0;
/// Thiscall on map marker data: its name (`mapmarkerdata.cpp`).
const MARKER_NAME_00438EB0: u32 = 0x0043_8eb0;
/// Cdecl (text, word, word, word, float, word) (`interface.cpp`).
const SHOW_MESSAGE_007052F0: u32 = 0x0070_52f0;
/// `FORenderedMenu::TriggerBurst` (engine map), thiscall (float, float,
/// float), `RET 0xC`.
const TRIGGER_BURST: u32 = 0x007f_b150;
/// Cdecl: `AL` (`interface.cpp`).
const FLAG_00709D40: u32 = 0x0070_9d40;
/// The `ListBox` base helpers (`menumanager.rs`): the `ListBoxBase`
/// constructor, the registration with the menu manager and its removal.
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// Thiscall on a list box (flag bit), `RET 4`: `AL` (`menumanager.rs`).
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
/// Thiscall on a list box (tile), `RET 4`: `AL`, whether the tile is one of
/// its items (`menumanager.rs`).
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Thiscall on a list node (word), `RET 4`: `RemoveAll` passes 0 before
/// deleting the node.
const NODE_SET_NEXT: u32 = 0x006e_cd40;
/// Thiscall on a list node (flags), `RET 4`: its deleting destructor.
const NODE_DELETE: u32 = 0x0047_02f0;
/// Thiscall on an item's data (flags), `RET 4`.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that, when set, keeps the item tiles when a list box empties.
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// Cdecl (float): a float in `ST0` (`teswater.cpp`).
const ROUND_FLOAT: u32 = 0x004e_4430;
/// Cdecl (integer, integer): `EAX`.
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
/// Thiscall on the interface manager (`tesworldspace.cpp`): the object
/// `Tile::ReadFile` runs on.
const READER_00586150: u32 = 0x0058_6150;
/// Thiscall on the player (address of a string), `RET 4`
/// (`tesobjectrefr.cpp`): `UpdateMenu` hands it the date text.
const PLAYER_MESSAGE_00578870: u32 = 0x0057_8870;

/// `"_enabled"`, `"_highlight_y"`, `"_selected_height"`, `"lb_highlight_box"`,
/// `"top"`, `"bottom"`, `"left"`, `"right"`, `"_num_filtered"`,
/// `"_scroll_delta"`, `"_number_of_items"` (the `ListBox` methods).
const ENABLED_TEXT: u32 = 0x0107_0244;
const HIGHLIGHT_Y_TEXT: u32 = 0x0106_f8c0;
const SELECTED_HEIGHT_TEXT: u32 = 0x0106_f8ac;
const HIGHLIGHT_BOX_TEXT: u32 = 0x0106_f898;
const TOP_TEXT: u32 = 0x0106_f894;
const BOTTOM_TEXT: u32 = 0x0106_f88c;
const LEFT_TEXT: u32 = 0x0106_f884;
const RIGHT_TEXT: u32 = 0x0106_f87c;
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
const NUMBER_OF_ITEMS_TEXT: u32 = 0x0106_f8f8;

/// The tile pointer `index` of the menu at `menu` (`pTiles`, +0x28).
fn map_tile(e: &Engine, menu: u32, index: u32) -> u32 {
    e.mem.u32(menu + 0x28 + 4 * index)
}

/// `MapMenu::pMe`.
fn map_menu(e: &Engine) -> u32 {
    e.mem.u32(MAP_MENU_SINGLETON)
}

/// The `float` argument a `PUSH ECX; FLD float ptr [..]; FSTP float ptr
/// [ESP]` pair passes: the constant's bits, a signalling NaN quieted.
fn float_constant(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: on only when `a` is strictly above
/// `b` (not less, equal or unordered).
fn above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

/// `FCOMP; FNSTSW AX; TEST AH,5; JNP` (jump taken): `a` below `b`, ordered.
fn below(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Less)
}

/// `TEST AH,0x44; JP` not taken: `a` equal to `b`, ordered.
fn equal(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Equal)
}

/// The tile object chain `ClearMapMemory`, `ClearWorldMapTexture` and
/// `Create` walk from a tile to its tile shader property: `0056c7f0` on the
/// tile, `0045bc00(.., 0)` on that, then `NiAVObject::GetProperty(3)`; 0
/// where a step gives 0.
fn tile_shader_property(e: &mut Engine, tile: u32) -> u32 {
    let node = if e.call(TILE_OBJECT_0056C7F0, &args![tile]).u32() == 0 {
        0
    } else {
        let object = e.call(TILE_OBJECT_0056C7F0, &args![tile]).u32();
        e.call(OBJECT_NODE_0045BC00, &args![object, 0u32]).u32()
    };
    if node == 0 {
        return 0;
    }
    let kind = e.call(PROPERTY_TYPE_00438220, &args![]).u32();
    e.call(AV_OBJECT_GET_PROPERTY, &args![node, kind]).u32()
}

/// A sound by name played once: `BSAudio::GetSoundHandleByName(name, 0x121)`
/// into a stack handle, `Play(0)`, then the handle destroyed.
fn play_named_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let sound = e
            .call(AUDIO_SOUND_BY_NAME, &args![audio, handle, name, 0x121u32])
            .u32();
        e.call(SOUND_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// `a011b0(tile, trait)` as the `float` an `FSTP float` stores.
fn tile_float_bits(e: &mut Engine, tile: u32, id: u32) -> u32 {
    x87_store32(e.call(TILE_GET_FLOAT, &args![tile, id]).f64())
}

// Translated from 0072d040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]` for
/// an id in 0..0x16.
pub fn map_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    if (0..0x16).contains(&id) {
        e.mem.set_u32(this.addr() + 0x28 + 4 * id as u32, tile);
    }
}

// Translated from 007312e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::SaveListState` (engine map): saves the highlighted
/// item's index (trait `0xfac`, -1.0 without one) and the scroll bar's
/// `"_current_value"` (-1.0 without a scroll bar).
pub fn list_box_bgs_note_save_list_state(e: &mut Engine, this: Ptr<ListBox>) {
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let index = if highlighted.is_null() {
        float_constant(e, MINUS_ONE_FLOAT)
    } else {
        tile_float_bits(e, highlighted.addr(), 0xfac)
    };
    e.mem.set_u32(this.addr() + 0x24, x87_quiet_bits(index));
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    let value = if scroll_bar.is_null() {
        float_constant(e, MINUS_ONE_FLOAT)
    } else {
        let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        tile_float_bits(e, scroll_bar.addr(), current_value)
    };
    e.mem.set_u32(this.addr() + 0x28, x87_quiet_bits(value));
}

// Translated from 00796740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00796740 - the C++ exception frame (FS:[0] chain, unwind states 0 to 10 around the members' construction) is not translated.
/// `MapMenu::MapMenu` (Xbox PDB): the `Menu` base, the vtable, the members
/// (sound list, subtitle array, the two marker lists, the four points, the
/// five list boxes and the waypoint array), the eight trait ids of the
/// statics (`Tile::TextToTrait`), then the fields' start values: page 0x24,
/// showing all notes, fog of war on, `PlayerMarkerPos` from `011f426c` and
/// the radio volume saved from `BSAudio::GetRadioVolume`.
pub fn map_menu_map_menu(e: &mut Engine, this: Ptr<MapMenu>) -> Ptr<MapMenu> {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, MAP_MENU_VTABLE);
    e.call(SOUND_LIST_CONSTRUCT, &args![base + 0x98]);
    fn_007a1a70(e, Ptr::new(base + 0xa8));
    e.call(LIST_CONSTRUCT, &args![base + 0xd8]);
    e.call(LIST_CONSTRUCT, &args![base + 0xe0]);
    for offset in [0xe8u32, 0xf0, 0xfc, 0x120] {
        e.call(SELF, &args![base + offset]);
    }
    fn_007a1c90(e, Ptr::new(base + 0x130));
    fn_007a27f0(e, Ptr::new(base + 0x160));
    fn_007a36e0(e, Ptr::new(base + 0x190));
    fn_007a43a0(e, Ptr::new(base + 0x1c0));
    fn_007a50f0(e, Ptr::new(base + 0x1f0));
    e.call(WAYPOINTS_CONSTRUCT, &args![base + 0x220]);
    for (cache, name) in [
        (TRAIT_TITLE, TITLE_TEXT),
        (TRAIT_VALUE, VALUE_TEXT),
        (TRAIT_CURRENT_TAB, CURRENT_TAB_TEXT),
        (TRAIT_SELECTED, SELECTED_TEXT),
        (TRAIT_TEXT_ALPHA, TEXT_ALPHA_TEXT),
        (TRAIT_X, X_COORD_TEXT),
        (TRAIT_Y, Y_COORD_TEXT),
        (TRAIT_MAGNIFICATION, MAGNIFICATION_TEXT),
    ] {
        let id = e.call(TEXT_TO_TRAIT, &args![name]).u32();
        e.mem.set_u32(cache, id);
    }
    e.set(this, MapMenu::fWaveformAnimationStartTime, 0.0);
    e.set(this, MapMenu::bShowingAllNotes, 1);
    e.set(this, MapMenu::pCurrentSound, 0);
    e.set(this, MapMenu::bSoundsLoading, 0);
    e.set(this, MapMenu::pFastTravelMarker, 0);
    e.set(this, MapMenu::iCurrentPage, 0x24);
    for word in 0..3 {
        let value = e.mem.u32(POINT_011F426C + 4 * word);
        e.mem.set_u32(base + 0xfc + 4 * word, value);
    }
    e.set(this, MapMenu::PlayerMarkerSpace, 0);
    e.set(this, MapMenu::pMapSpace, 0);
    e.set(this, MapMenu::pMapTile, 0);
    e.set(this, MapMenu::pDisplayedQuest, 0);
    e.set(this, MapMenu::pSelectedNote, 0);
    e.set(this, MapMenu::pAdjustNorthRotationCell, 0);
    e.set(this, MapMenu::pHighlightedMapMarker, 0);
    e.set(this, MapMenu::uiNoteEnterStartTime, 0);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let volume = e.call(AUDIO_GET_RADIO_VOLUME, &args![audio]).f64();
    e.mem.set_u32(base + 0xc8, x87_store32(volume));
    e.set(this, MapMenu::pDisplayedQuestTarget, 0);
    e.set(this, MapMenu::uiNextLocalMapGrid, 0);
    e.set(this, MapMenu::pMapPlayerRef, 0);
    e.set(this, MapMenu::bFogOfWarEnabled, 1);
    e.set(this, MapMenu::pRootMapSpace, 0);
    e.set(this, MapMenu::bRootSpaceChanged, 0);
    this
}

// Translated from 00796a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::GetClass` (Xbox PDB): `0x3ff`.
pub fn map_menu_get_class(e: &mut Engine, this: Ptr<MapMenu>) -> u32 {
    let _ = (e, this);
    0x3ff
}

// Translated from 00796a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu`'s scalar deleting destructor (Xbox PDB `__vecDelDtor` slot):
/// the destructor body `00796a80`, then the object freed when bit 0 of
/// `flags` is set. Returns `this`.
pub fn map_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    flags: u32,
) -> Ptr<MapMenu> {
    fn_00796a80(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00796a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00796a80 - the C++ exception frame (FS:[0] chain, unwind states 9 down to 0 around the members' destruction) is not translated.
/// The `MapMenu` destructor body: the vtable, `MapMenu::pMe` cleared, then
/// the members destroyed in reverse order (waypoint array, the five list
/// boxes, the two marker lists, the subtitle array, the sound list) and the
/// `Menu` base.
pub fn fn_00796a80(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    e.mem.set_u32(base, MAP_MENU_VTABLE);
    e.mem.set_u32(MAP_MENU_SINGLETON, 0);
    e.call(WAYPOINTS_DESTRUCT, &args![base + 0x220]);
    fn_007a51c0(e, Ptr::new(base + 0x1f0));
    fn_007a4470(e, Ptr::new(base + 0x1c0));
    fn_007a37b0(e, Ptr::new(base + 0x190));
    fn_007a28c0(e, Ptr::new(base + 0x160));
    fn_007a1d60(e, Ptr::new(base + 0x130));
    e.call(LIST_DESTRUCT, &args![base + 0xe0]);
    e.call(LIST_DESTRUCT, &args![base + 0xd8]);
    fn_007a1aa0(e, Ptr::new(base + 0xa8));
    e.call(SOUND_LIST_DESTRUCT, &args![base + 0x98]);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 00796b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::Create` (Xbox PDB): deletes an open map menu (`GetMenuByClass`
/// 0x3ff, its deleting destructor with 1), reads `map_menu.xml`, takes its
/// menu as `MapMenu::pMe` and removes the file from the XML archive. When
/// that is not a map menu (class 0x3ff) the menu is deleted again (when its
/// tile, `00726070`, is set) and 0 returned. Otherwise: the calendar's days
/// are marked (`00867a20`), the tile is set (`SetMenuTile(tile, 0)`), and
/// unless a tile is missing (`00796f90`, with a warning and 0 returned) the
/// tile's depth trait `0xfad` is set to the maximum depth when trait `0xfa7`
/// is 6006 or 102, the tutorial message 0x1a is shown, the tab line and the
/// five list boxes are set up, the map's property is updated (`007700d0`),
/// the title emptied, tiles 5 and 3 get flag 0x8000 (`00796f40`), the map
/// memory is cleared and the menu prepared for visibility. Returns the tile.
pub fn map_menu_create(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![0x3ffu32]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(READER_00586150, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![reader, MAP_MENU_XML]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.mem.set_u32(MAP_MENU_SINGLETON, menu);
    let menu = map_menu(e);
    e.call(MENU_REMOVE_FROM_XML_ARCHIVE, &args![MAP_MENU_XML, menu]);
    let menu = map_menu(e);
    if menu == 0 || e.vcall(menu, 0x34, &args![]).u32() != 0x3ff {
        let menu = map_menu(e);
        if menu != 0 && e.call(WORD_AT_4, &args![menu]).u32() != 0 {
            let menu = map_menu(e);
            if menu != 0 {
                e.vcall(menu, 0, &args![1u32]);
            }
        }
        return 0;
    }
    let menu = map_menu(e);
    e.call(CALENDAR_MARK_STALE, &args![menu]);
    let menu = map_menu(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
    // The code stores `pMe` back into itself here.
    let menu = map_menu(e);
    e.mem.set_u32(MAP_MENU_SINGLETON, menu);
    if !fn_00796f90(e, Ptr::new(map_menu(e))) {
        e.call(PRINT_WARNING, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
    let first: f64 = e.global(CREATE_VALUE_A);
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
    let matches = equal(value, first) || {
        let second: f64 = e.global(CREATE_VALUE_B);
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
        equal(value, second)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfadu32, x87_store32(depth), 1u32],
        );
    }
    let menu_id = e.call(MAP_MENU_ID_00720DF0, &args![]).u32();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.call(
        TUTORIAL_SHOW_MESSAGE,
        &args![interface.wrapping_add(0x4d4), 0x1au32, menu_id, 0x200u32],
    );
    let mut names = [0u32; 5];
    for (i, entry) in TABLINE_ENTRIES.iter().enumerate() {
        names[i] = e.call(OBJECT_FIELD_00403DF0, &args![*entry]).u32();
    }
    let menu = map_menu(e);
    let tabline = map_tile(e, menu, 17);
    e.call(
        INITIALIZE_TABLINE,
        &args![tabline, 0x20u32, names[4], names[3], names[2], names[1], names[0], 0u32],
    );
    for (list, index, flag) in [
        (0x130u32, 7u32, true),
        (0x1c0, 15, true),
        (0x160, 8, true),
        (0x1f0, 19, false),
        (0x190, 10, false),
    ] {
        let menu = map_menu(e);
        let list_tile = map_tile(e, menu, index);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![menu + list, list_tile, LIST_MARKER_TEMPLATE],
        );
        if flag {
            let menu = map_menu(e);
            fn_007a1930(e, Ptr::new(menu + list), 0);
        }
    }
    let menu = map_menu(e);
    let map = map_tile(e, menu, 16);
    let property = tile_shader_property(e, map);
    if property != 0 {
        e.call(PROPERTY_SET_007700D0, &args![property, 1u32]);
    }
    let title = e
        .call(ADD_USER_TRAIT, &args![TITLE_TEXT, 0xffff_ffffu32])
        .u32();
    let menu = map_menu(e);
    let title_tile = map_tile(e, menu, 6);
    e.call(TILE_SET_STRING, &args![title_tile, title, EMPTY_TEXT, 1u32]);
    let menu = map_menu(e);
    let tile_5 = map_tile(e, menu, 5);
    fn_00796f40(e, Ptr::new(tile_5), 1);
    let menu = map_menu(e);
    let tile_3 = map_tile(e, menu, 3);
    fn_00796f40(e, Ptr::new(tile_3), 1);
    map_menu_clear_map_memory(e);
    let menu = map_menu(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 1u32]);
    tile
}

// Translated from 00796f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`flag` non-zero) or clears bit 0x8000 of
/// the word at +0x30 (the code calls it on tiles).
pub fn fn_00796f40(e: &mut Engine, this: Ptr, flag: u8) {
    let word = e.mem.u32(this.addr() + 0x30);
    if flag != 0 {
        e.mem.set_u32(this.addr() + 0x30, word | 0x8000);
    } else {
        e.mem.set_u32(this.addr() + 0x30, word & 0xffff_7fff);
    }
}

// Translated from 00796f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether all 22 `pTiles` are set.
pub fn fn_00796f90(e: &mut Engine, this: Ptr<MapMenu>) -> bool {
    (0..0x16).all(|index| map_tile(e, this.addr(), index) != 0)
}

// Translated from 00796fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00796fd0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 6 around the sound handles, the speaker and the conversation) and the /GS security cookie check (00ec408c) are not translated.
/// `MapMenu::DoClick` (Xbox PDB): with no tile, a click on id 0..0x16 uses
/// `pTiles[id]`. Then by id: 2 and 4 click the highlighted map marker (id
/// 0x1a through the vtable); 0xc acts on the current page (sets the player
/// marker on the world or local map, centres the map on the displayed quest
/// target, or switches the notes filter or the challenge type); 0x12 toggles
/// the notes/challenge mode on the notes page; 0x17 makes the highlighted
/// quest active; 0x18 opens the clicked note (and plays its sound or its
/// conversation); 0x19 switches the highlighted radio station; 0x1a asks
/// for a fast travel to the clicked marker (or shows a companion message);
/// 0x20 to 0x24 switch the page. See the body for the exact order.
pub fn map_menu_do_click(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    if tile == 0 && (0..0x16).contains(&id) {
        tile = map_tile(e, base, id as u32);
    }
    // The compiler's switch: a byte table (`00798388`) maps `id - 2` (0 to
    // 0x22, compared unsigned) to the cases of `00798364`.
    match id.wrapping_sub(2) as u32 {
        0 | 2 => {
            let marker = e.get(this, MapMenu::pHighlightedMapMarker);
            if marker != 0 {
                let marker = e.get(this, MapMenu::pHighlightedMapMarker);
                e.vcall(base, 0xc, &args![0x1au32, marker]);
            }
        }
        0xa => do_click_current_page(e, this),
        0x10 => {
            if e.get(this, MapMenu::iCurrentPage) == 0x23 {
                e.call(TOGGLE_NOTES_CHALLENGE_MODE, &args![this, 1u32]);
            }
        }
        0x15 => {
            let quests = Ptr::new(base + 0x130);
            if fn_007a1910(e, quests) != 0 {
                let quest = fn_007a1910(e, quests);
                if !e.call(QUEST_FLAG_0059E400, &args![quest]).bool() {
                    let quest = fn_007a1910(e, quests);
                    let player = e.mem.u32(PLAYER_011DEA3C);
                    e.call(SET_ACTIVE_QUEST, &args![player, quest]);
                    e.call(PLAY_MENU_SOUND, &args![3u32]);
                    e.call(UPDATE_CURRENT_PAGE, &args![this]);
                }
            }
        }
        0x16 => {
            if e.mem.u8(CHALLENGE_MODE_011DA36D) == 0 {
                do_click_note(e, this, tile);
            }
        }
        0x17 => {
            let radio = base + 0x190;
            let highlighted = e.vcall(radio, 4, &args![]).u32();
            let alpha_trait = e.mem.u32(TRAIT_TEXT_ALPHA);
            let alpha = e
                .call(TILE_GET_FLOAT, &args![highlighted, alpha_trait])
                .f64();
            let full: f64 = e.global(FULL_ALPHA_DOUBLE);
            if equal(alpha, full) {
                let selected_trait = e.mem.u32(TRAIT_SELECTED);
                let selected = e
                    .call(TILE_IS_TRUE, &args![highlighted, selected_trait])
                    .bool();
                let turn_on = !selected;
                fn_0079c090(e, this);
                e.call(
                    LIST_BOX_FILTER_007314C0,
                    &args![radio, RADIO_FILTER_CALLBACK, 0u32, 0x7fff_ffffu32],
                );
                if e.call(FLAG_00798440, &args![]).bool() {
                    e.call(RADIO_ENABLE, &args![0u32]);
                }
                if turn_on {
                    e.call(RADIO_ENABLE, &args![1u32]);
                    let station = fn_007a1910(e, Ptr::new(radio));
                    e.call(RADIO_SET_STATION_00832240, &args![station, 0u32]);
                    let selected_trait = e.mem.u32(TRAIT_SELECTED);
                    e.call(TILE_SET_INT, &args![highlighted, selected_trait, 1u32]);
                }
                e.call(UPDATE_CURRENT_PAGE, &args![this]);
            }
        }
        0x18 => do_click_marker(e, this, id, tile),
        0x1e..=0x22 => {
            let tabline = map_tile(e, base, 17);
            let tab_trait = e.mem.u32(TRAIT_CURRENT_TAB);
            e.call(
                TILE_SET_INT,
                &args![tabline, tab_trait, id.wrapping_sub(0x20)],
            );
            e.set(this, MapMenu::iCurrentPage, id);
            let third = float_constant(e, HALF_FLOAT);
            let second_ptr = e
                .call(OBJECT_VALUE_ADDRESS_00403E20, &args![BURST_SETTING_A])
                .u32();
            let second = x87_quiet_bits(e.mem.u32(second_ptr));
            let first_ptr = e
                .call(OBJECT_VALUE_ADDRESS_00403E20, &args![BURST_SETTING_B])
                .u32();
            let first = x87_quiet_bits(e.mem.u32(first_ptr));
            let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
            let rendered = e.call(WORD_AT_174, &args![interface]).u32();
            e.call(TRIGGER_BURST, &args![rendered, first, second, third]);
            if !e.call(FLAG_00709D40, &args![]).bool() && id != 0x20 && id != 0x21 {
                let alpha = float_constant(e, FULL_ALPHA_FLOAT);
                let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
                let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
                e.call(TILE_SET_FLOAT, &args![pipboy_tile, 0xfa9u32, alpha, 1u32]);
            }
            let tile_13 = map_tile(e, base, 13);
            e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 0u32]);
            let tile_12 = map_tile(e, base, 12);
            e.call(TILE_SET_INT, &args![tile_12, 0xfafu32, 1u32]);
            map_menu_update_menu(e);
            let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
            let rendered = e.call(WORD_AT_174, &args![interface]).u32();
            if e.call(PIPBOY_FLAG_00974D90, &args![rendered]).bool() {
                e.call(TAB_KNOB_0079C340, &args![1u32]);
            }
        }
        _ => {}
    }
}

/// `DoClick` id 0xC: by `iCurrentPage` (a switch on `page - 0x20`, 0 to 3).
fn do_click_current_page(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    match e.get(this, MapMenu::iCurrentPage).wrapping_sub(0x20) as u32 {
        0 | 1 => {
            if e.get(this, MapMenu::pMapTile) == 0 {
                return;
            }
            set_player_marker(e, this);
        }
        2 => {
            if e.get(this, MapMenu::pDisplayedQuestTarget) == 0 {
                return;
            }
            e.vcall(base, 0xc, &args![0x21u32, 0u32]);
            let target = e.get(this, MapMenu::pDisplayedQuestTarget);
            let quest_part = e.call(PLUS_10, &args![target]).u32();
            let target = e.get(this, MapMenu::pDisplayedQuestTarget);
            let reference = e
                .call(QUEST_TARGET_GET_REFERENCE, &args![target, 1u32])
                .u32();
            let tracked = e
                .call(
                    GET_WORLD_MAP_TRACKING_REF,
                    &args![this, reference, quest_part],
                )
                .u32();
            if tracked == 0 || e.get(this, MapMenu::pMapTile) == 0 {
                return;
            }
            let world = e.call(REFR_GET_WORLD_SPACE, &args![tracked]).u32();
            let position = e.vcall(tracked, 0x1f4, &args![]).u32();
            let x = e.mem.u32(position);
            let y = e.mem.u32(position + 4);
            let z = e.mem.u32(position + 8);
            // 0079c380 writes a whole 12-byte point (x, y, z) at EBP-0x4c in the exe.
            e.with_stack(0xc, |e, map_point| {
                e.call(WORLD_TO_MAP_COORD, &args![this, map_point, x, y, z, world]);
                for (size_trait, position_trait, offset) in
                    [(0xfb1u32, 0xfa1u32, 0u32), (0xfb0, 0xfa2, 4)]
                {
                    let map = e.get(this, MapMenu::pMapTile);
                    let owner = e.call(WORD_AT_28, &args![map]).u32();
                    let size = e.call(TILE_GET_FLOAT, &args![owner, size_trait]).f64();
                    let two: f64 = e.global(TWO_DOUBLE);
                    let half = x87_div(size, two);
                    let map = e.get(this, MapMenu::pMapTile);
                    let scale = e.call(TILE_GET_FLOAT, &args![map, size_trait]).f64();
                    let point = e.mem.u32(map_point.addr() + offset);
                    let scaled = x87_mul(scale, x87_m32(point));
                    let value = x87_store32(x87_sub(half, scaled));
                    let map = e.get(this, MapMenu::pMapTile);
                    e.call(
                        TILE_SET_FLOAT,
                        &args![map, position_trait, x87_quiet_bits(value), 0u32],
                    );
                }
            });
        }
        3 => {
            if e.mem.u8(CHALLENGE_MODE_011DA36D) == 0 {
                let showing = e.get(this, MapMenu::bShowingAllNotes);
                e.set(this, MapMenu::bShowingAllNotes, u8::from(showing == 0));
                e.call(NOTE_LIST_FILTER, &args![base + 0x160, NOTE_FILTER_CALLBACK]);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 0u32]);
                let showing = e.get(this, MapMenu::bShowingAllNotes);
                let showing_trait = e.call(TEXT_TO_TRAIT, &args![SHOWING_ALL_NOTES_TEXT]).u32();
                let tile_12 = map_tile(e, base, 12);
                e.call(
                    TILE_SET_INT,
                    &args![tile_12, showing_trait, u32::from(showing)],
                );
            } else {
                let kind = e.mem.u8(CHALLENGE_TYPE_011DA36E).wrapping_add(1);
                e.mem.set_u8(CHALLENGE_TYPE_011DA36E, kind);
                if (e.mem.u8(CHALLENGE_TYPE_011DA36E) as i8) > 2 {
                    e.mem.set_u8(CHALLENGE_TYPE_011DA36E, 0);
                }
                e.call(
                    CHALLENGE_LIST_FILTER,
                    &args![base + 0x1f0, CHALLENGE_FILTER_CALLBACK],
                );
                let kind = e.mem.u8(CHALLENGE_TYPE_011DA36E) as i8 as i32;
                let entry = e
                    .mem
                    .u32(CHALLENGE_TYPE_NAMES_011A03E4.wrapping_add((kind as u32).wrapping_mul(4)));
                let name = e.call(OBJECT_FIELD_00403DF0, &args![entry]).u32();
                let tile_21 = map_tile(e, base, 21);
                e.call(TILE_SET_STRING, &args![tile_21, 0xfc4u32, name, 1u32]);
                let tile_20 = map_tile(e, base, 20);
                e.call(TILE_SET_INT, &args![tile_20, 0xfa3u32, 0u32]);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 0u32]);
            }
        }
        _ => {}
    }
}

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// `DoClick` id 0xC on the world map (0x20) and the local map (0x21): the
/// marker position from the map tile's centre (`004b71d0` set) or from the
/// cursor, the world position (`MapToWorldCoord`), its world space, the
/// north-rotation adjustment, the sound and the confirmation message box.
fn set_player_marker(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let position = base + 0xfc;
    if e.call(FLAG_004B71D0, &args![]).bool() {
        let z = 0.0f32.to_bits();
        let mut coords = [0u32; 2];
        for (i, (size_trait, offset_trait)) in
            [(0xfb0u32, 0xfa2u32), (0xfb1, 0xfa1)].iter().enumerate()
        {
            let map = e.get(this, MapMenu::pMapTile);
            let owner = e.call(WORD_AT_28, &args![map]).u32();
            let size = e.call(TILE_GET_FLOAT, &args![owner, *size_trait]).f64();
            let two: f64 = e.global(TWO_DOUBLE);
            let half = x87_div(size, two);
            let map = e.get(this, MapMenu::pMapTile);
            let offset = e.call(TILE_GET_FLOAT, &args![map, *offset_trait]).f64();
            coords[i] = x87_quiet_bits(x87_store32(x87_sub(half, offset)));
        }
        e.with_stack(0xc, |e, point| {
            let result = e
                .call(POINT_SET_00416870, &args![point, coords[1], coords[0], z])
                .u32();
            copy_point(e, position, result);
        });
    } else {
        let z = 0.0f32.to_bits();
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let y = x87_store32(e.call(CURSOR_Y_007983C0, &args![interface]).f64());
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let x = x87_store32(e.call(CURSOR_X_007697F0, &args![interface]).f64());
        e.with_stack(0xc, |e, point| {
            let result = e.call(POINT_SET_00416870, &args![point, x, y, z]).u32();
            copy_point(e, position, result);
        });
        let map = e.get(this, MapMenu::pMapTile);
        let left = e.call(TILE_GETTER_00A013D0, &args![map]).f64();
        let x = e.mem.u32(position);
        e.mem
            .set_u32(position, x87_store32(x87_sub(x87_m32(x), left)));
        let map = e.get(this, MapMenu::pMapTile);
        let top = e.call(TILE_GETTER_00A01440, &args![map]).f64();
        let y = e.mem.u32(position + 4);
        e.mem
            .set_u32(position + 4, x87_store32(x87_sub(x87_m32(y), top)));
    }
    if e.get(this, MapMenu::iCurrentPage) == 0x20 {
        let space = e.get(this, MapMenu::pMapSpace);
        e.set(this, MapMenu::PlayerMarkerSpace, space);
        let space = e.get(this, MapMenu::PlayerMarkerSpace);
        let world = if e.call(FORM_TYPE, &args![space]).u32() == 0x41 {
            e.get(this, MapMenu::PlayerMarkerSpace)
        } else {
            0
        };
        let map = e.get(this, MapMenu::pMapTile);
        let scale = tile_float_bits(e, map, 0xfb1);
        e.with_stack(0x18, |e, temps| {
            let scaled = e
                .call(POINT_SCALE_0053D280, &args![position, temps, scale])
                .u32();
            let x = e.mem.u32(scaled);
            let y = e.mem.u32(scaled + 4);
            let z = e.mem.u32(scaled + 8);
            let result = e
                .call(
                    MAP_TO_WORLD_COORD,
                    &args![this, temps.addr() + 0xc, x, y, z, world],
                )
                .u32();
            copy_point(e, position, result);
            copy_point(e, position, position);
        });
    } else {
        let map = e.get(this, MapMenu::pMapTile);
        let scale = tile_float_bits(e, map, 0xfb1);
        e.with_stack(0x18, |e, temps| {
            let scaled = e
                .call(POINT_SCALE_0053D280, &args![position, temps, scale])
                .u32();
            let x = e.mem.u32(scaled);
            let y = e.mem.u32(scaled + 4);
            let z = e.mem.u32(scaled + 8);
            let result = e
                .call(
                    MAP_TO_WORLD_COORD,
                    &args![this, temps.addr() + 0xc, x, y, z, 0u32],
                )
                .u32();
            copy_point(e, position, result);
        });
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.u32(position + 8);
        let space = e
            .call(CALCULATE_PLAYER_MARKER_SPACE, &args![this, x, y, z])
            .u32();
        e.set(this, MapMenu::PlayerMarkerSpace, space);
        if e.get(this, MapMenu::PlayerMarkerSpace) != 0 {
            let space = e.get(this, MapMenu::PlayerMarkerSpace);
            e.call(ADJUST_MAP_MARKER_COORD, &args![space, position, 1u32]);
        }
    }
    if e.get(this, MapMenu::pAdjustNorthRotationCell) != 0 {
        let cell = e.get(this, MapMenu::pAdjustNorthRotationCell);
        e.call(
            ADJUST_FOR_NORTH_ROTATION,
            &args![cell, position, position, 0u32],
        );
    }
    let tile_6 = map_tile(e, base, 6);
    e.call(TILE_SET_INT, &args![tile_6, 0xfa3u32, 0u32]);
    play_named_sound(e, MARKER_ADDED_SOUND);
    let player = e.mem.u32(PLAYER_011DEA3C);
    let zero = 0.0f32.to_bits();
    if e.call(PLAYER_FLAG_00798400, &args![player]).u32() != 0 {
        let fourth = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4f54u32]).u32();
        let third = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_33a8u32]).u32();
        let second = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4960u32]).u32();
        let text = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4438u32]).u32();
        e.call(
            SHOW_MESSAGE_BOX,
            &args![
                text,
                0u32,
                0u32,
                SET_PLAYER_MARKER_CALLBACK,
                0u32,
                0x17u32,
                zero,
                zero,
                second,
                third,
                fourth,
                0u32
            ],
        );
    } else {
        let third = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_3684u32]).u32();
        let second = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_34f8u32]).u32();
        let text = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_3a2cu32]).u32();
        e.call(
            SHOW_MESSAGE_BOX,
            &args![
                text,
                0u32,
                0u32,
                SET_PLAYER_MARKER_CALLBACK,
                3u32,
                0x17u32,
                zero,
                zero,
                second,
                third,
                0u32
            ],
        );
    }
}

/// `DoClick` id 0x18 (notes page, not in challenge mode): the clicked note
/// is opened or closed; an audio note (type 0) or a conversation note (type
/// 3) queues its sound (`BSSimpleList<BSSoundHandle>::AddTail`) or runs its
/// conversation through a temporary `Character`, then the holotape sound
/// plays and the audio enters dialogue.
fn do_click_note(e: &mut Engine, this: Ptr<MapMenu>, tile: u32) {
    let base = this.addr();
    e.with_stack(0xc, |e, handle| {
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        let note = e
            .call(LIST_BOX_ITEM_OF_007E1110, &args![base + 0x160, tile])
            .u32();
        let audio_note = e.call(NOTE_TYPE, &args![note]).u32() == 0
            || e.call(NOTE_TYPE, &args![note]).u32() == 3;
        if note != 0 && (!e.call(FLAG_004B71D0, &args![]).bool() || audio_note) {
            let same = note == e.get(this, MapMenu::pSelectedNote);
            let mut stopped = false;
            let previous = e
                .call(LIST_BOX_GET_TILE_FOR, &args![base + 0x160, base + 0x90])
                .u32();
            if previous != 0 {
                let selected = e.mem.u32(TRAIT_SELECTED);
                e.call(TILE_SET_INT, &args![previous, selected, 0u32]);
            }
            if !same || (audio_note && e.get(this, MapMenu::pCurrentSound) == 0) {
                e.call(LOAD_NOTE, &args![this, note]);
                let selected = e.mem.u32(TRAIT_SELECTED);
                e.call(TILE_SET_INT, &args![tile, selected, 1u32]);
                e.set(this, MapMenu::pSelectedNote, note);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0x1004u32, 1u32]);
            } else {
                e.set(this, MapMenu::pSelectedNote, 0);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0x1004u32, 0u32]);
            }
            if e.get(this, MapMenu::pCurrentSound) != 0 {
                e.call(STOP_NOTE_SOUND, &args![this]);
                stopped = true;
            }
            if audio_note && !e.call(NOTE_FLAG_00798420, &args![note]).bool() {
                let alpha = float_constant(e, HALF_ALPHA_FLOAT);
                let alpha_trait = e.mem.u32(TRAIT_TEXT_ALPHA);
                e.call(TILE_SET_FLOAT, &args![tile, alpha_trait, alpha, 1u32]);
                e.call(NOTE_SET_READ_005E9300, &args![note, 1u32]);
            }
            if audio_note && (!same || !stopped) {
                play_note(e, this, note, handle.addr());
            }
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// The sound part of `DoClick` id 0x18 (`handle` is its sound handle).
fn play_note(e: &mut Engine, this: Ptr<MapMenu>, note: u32, handle: u32) {
    let base = this.addr();
    if e.call(NOTE_TYPE, &args![note]).u32() == 0
        && e.call(NOTE_SOUND_005E9100, &args![note]).u32() != 0
    {
        let sound = e.call(NOTE_SOUND_005E9100, &args![note]).u32();
        let sound_id = e.call(WORD_AT_C, &args![sound]).u32();
        e.with_stack(0xc, |e, temp| {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let found = e
                .call(AUDIO_SOUND_BY_ID, &args![audio, temp, sound_id, 0x101u32])
                .u32();
            e.call(SOUND_LIST_ADD_TAIL, &args![base + 0x98, found]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![temp]);
        });
        e.set(this, MapMenu::bSoundsLoading, 1);
    } else if e.call(NOTE_TYPE, &args![note]).u32() == 3
        && e.call(NOTE_TOPIC_005E9240, &args![note]).u32() != 0
        && e.call(NOTE_GET_SPEAKER, &args![note]).u32() != 0
    {
        run_note_conversation(e, this, note, handle);
    }
    if e.get(this, MapMenu::bSoundsLoading) != 0 {
        play_named_sound(e, HOLOTAPE_START_SOUND);
        e.call(DIALOG_FLAG_00762150, &args![]);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(AUDIO_ENTER_DIALOGUE, &args![audio]);
    }
}

/// A conversation note: a temporary `Character` (0x1c8 bytes) of the note's
/// speaker, a `Conversation` (0xc bytes) with the player and the note's
/// topic, and for each response its subtitle, its result script (before and
/// after), its sound at volume 0.9 queued; then both are deleted.
fn run_note_conversation(e: &mut Engine, this: Ptr<MapMenu>, note: u32, handle: u32) {
    let base = this.addr();
    let block = e.call(ALLOCATE, &args![0x1c8u32]).u32();
    let speaker = if block != 0 {
        e.call(CHARACTER_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(FORM_SET_TEMPORARY, &args![speaker]);
    let base_form = e.call(NOTE_GET_SPEAKER, &args![note]).u32();
    e.call(REFR_SET_OBJECT_REFERENCE, &args![speaker, base_form]);
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    let conversation = if block != 0 {
        let topic = e.call(NOTE_TOPIC_005E9240, &args![note]).u32();
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(
            CONVERSATION_CONSTRUCT,
            &args![block, speaker, player, topic],
        )
        .u32()
    } else {
        0
    };
    e.call(CONVERSATION_FIRST_ITEM, &args![conversation]);
    let has_item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32() != 0 && {
        let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
        e.call(FIRST_RESPONSE, &args![item]).bool()
    };
    if has_item {
        loop {
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let response = e.call(GET_CURRENT_RESPONSE, &args![item]).u32();
            let text = e.call(SELF, &args![response]).u32();
            e.call(SUBTITLES_ADD_007A1AC0, &args![base + 0xa8, text]);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let info = e.call(WORD_AT_C, &args![item]).u32();
            e.call(TOPIC_INFO_RUN_RESULT, &args![info, 0u32, speaker]);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let response = e.call(GET_CURRENT_RESPONSE, &args![item]).u32();
            let file = e.call(PLUS_10, &args![response]).u32();
            let file_name = e.call(STRING_TEXT, &args![file]).u32();
            e.with_stack(0xc, |e, temp| {
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                let found = e
                    .call(
                        AUDIO_SOUND_BY_FILENAME,
                        &args![audio, temp, file_name, 0x105u32, 0u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, found]);
                e.call(SOUND_HANDLE_DESTRUCT, &args![temp]);
            });
            let volume = float_constant(e, NOTE_VOLUME_FLOAT);
            e.call(SOUND_SET_VOLUME, &args![handle, volume]);
            e.call(SOUND_LIST_ADD_TAIL, &args![base + 0x98, handle]);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            let info = e.call(WORD_AT_C, &args![item]).u32();
            e.call(TOPIC_INFO_RUN_RESULT, &args![info, 1u32, speaker]);
            e.set(this, MapMenu::bSoundsLoading, 1);
            let item = e.call(GET_CURRENT_RESPONSE, &args![conversation]).u32();
            if !e.call(NEXT_RESPONSE, &args![item]).bool() {
                break;
            }
        }
    }
    if conversation != 0 {
        fn_00798450(e, Ptr::new(conversation), 1);
    }
    if speaker != 0 {
        e.vcall(speaker, 0x10, &args![1u32]);
    }
}

/// `DoClick` id 0x1a: on the local map (0x21), when the map was not dragged
/// (the map tile still at `WorldMapDragStart`, or `004b71d0` set), fast
/// travel is offered (trait `0xfaf` of tile 11 and `0093d660` on the player)
/// to the marker the clicked tile's `"_MarkerIndex"` picks from the world map
/// marker list (+0xd8); otherwise a tile whose `"_LocationName"` is not
/// `"Companion"` shows the message `007052f0`.
fn do_click_marker(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let not_dragged = if e.call(FLAG_004B71D0, &args![]).bool() {
            true
        } else if e.get(this, MapMenu::pMapTile) == 0 {
            false
        } else {
            let map = e.get(this, MapMenu::pMapTile);
            let x = e.call(TILE_GET_FLOAT, &args![map, 0xfa1u32]).f64();
            let start_x = x87_load32(e.mem.u32(base + 0x120));
            equal(start_x, x) && {
                let map = e.get(this, MapMenu::pMapTile);
                let y = e.call(TILE_GET_FLOAT, &args![map, 0xfa2u32]).f64();
                let start_y = x87_load32(e.mem.u32(base + 0x124));
                equal(start_y, y)
            }
        };
        if not_dragged {
            let tile_11 = map_tile(e, base, 11);
            if e.call(TILE_IS_TRUE, &args![tile_11, 0xfafu32]).bool() {
                let player = e.mem.u32(PLAYER_011DEA3C);
                if e.call(PLAYER_FLAG_0093D660, &args![player]).bool() {
                    offer_fast_travel(e, this, id, tile);
                    return;
                }
            }
        }
    }
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let tile_11 = map_tile(e, base, 11);
        if !e.call(TILE_IS_TRUE, &args![tile_11, 0xfafu32]).bool() {
            let name_trait = e
                .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
                .u32();
            let name = e.call(TILE_GET_STRING, &args![tile, name_trait]).u32();
            if e.call(STRICMP, &args![name, COMPANION_TEXT]).u32() != 0 {
                let text = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_48e8u32]).u32();
                let two = float_constant(e, TWO_FLOAT);
                e.call(
                    SHOW_MESSAGE_007052F0,
                    &args![text, 2u32, 0u32, 0u32, two, 0u32],
                );
            }
        }
    }
}

/// The fast travel offer of `DoClick` id 0x1a.
fn offer_fast_travel(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut index = 0i32;
    let mut node = base + 0xd8;
    while node != 0 {
        let item = e.call(SELF, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let wanted = f64::from(index);
        let marker_trait = e
            .call(ADD_USER_TRAIT, &args![MARKER_INDEX_TEXT, 0xffff_ffffu32])
            .u32();
        let value = e.call(TILE_GET_FLOAT, &args![tile, marker_trait]).f64();
        // `FCOMP; TEST AH,1; JNZ`: stops when the value is below the index or
        // unordered.
        if value.partial_cmp(&wanted).map_or(true, |o| o.is_lt()) {
            break;
        }
        let item = e.call(SELF, &args![node]).u32();
        let marker = e.mem.u32(item);
        e.set(this, MapMenu::pFastTravelMarker, marker);
        index = index.wrapping_add(1);
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    let marker = e.get(this, MapMenu::pFastTravelMarker);
    let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
    if e.call(MARKER_GET_TRAVEL_LOC, &args![data]).bool() {
        e.vcall(base, 0x14, &args![id, tile]);
        let marker = e.get(this, MapMenu::pFastTravelMarker);
        let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
        let name = e.call(MARKER_NAME_00438EB0, &args![data]).u32();
        let question = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4a38u32]).u32();
        e.with_stack(0x100, |e, text| {
            e.call(
                BUFFER_FORMAT,
                &args![text, 0x100u32, FAST_TRAVEL_FORMAT, question, name],
            );
            let tile_6 = map_tile(e, base, 6);
            e.call(TILE_SET_INT, &args![tile_6, 0xfa3u32, 0u32]);
            let third = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_3684u32]).u32();
            let second = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_34f8u32]).u32();
            let zero = 0.0f32.to_bits();
            e.call(
                SHOW_MESSAGE_BOX,
                &args![
                    text,
                    0u32,
                    0u32,
                    FAST_TRAVEL_CALLBACK,
                    0u32,
                    0x17u32,
                    zero,
                    zero,
                    second,
                    third,
                    0u32
                ],
            );
        });
    } else {
        e.set(this, MapMenu::pFastTravelMarker, 0);
    }
}

// Translated from 00798450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `Conversation` `DoClick` makes: the
/// destructor body `0083b8d0`, then the object freed when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_00798450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CONVERSATION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0079a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoIdle` (Xbox PDB): pauses the note sound while menu 0x3f5 is on
/// top (keeping the time) and resumes it once neither 0x3f5 nor 0x3e9 is
/// (adding the paused time to `iSoundPlayStartTime`). While the pip-boy is
/// up (`00974d90`): with another menu on top, the pip-boy tile fades in once
/// (trait `0xfa9` 255.0) on the first three pages; with the map menu on top,
/// by page: the world map steps its local map 25 times when asked, and the
/// world and local maps show the cursor position (with the cursor off the
/// 850x500 map the pip-boy tile's alpha is 255, otherwise 0 and a click
/// 0xc when `00a23a50` but not `00a24660`); the notes page marks the
/// selected note read (`"_TextAlpha"` 128) once its delay has passed; the
/// radio page updates (`0079c0c0` or `0079c090`). Then the playing sounds
/// are updated when any are loading or playing.
pub fn map_menu_do_idle(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let paused = e.call(IS_TOP_MENU_ID, &args![0x3f5u32]).bool()
        && e.get(this, MapMenu::pCurrentSound) != 0
        && {
            let sound = e.get(this, MapMenu::pCurrentSound);
            let handle = e.call(SELF, &args![sound]).u32();
            e.call(SOUND_IS_PLAYING, &args![handle]).bool()
        };
    if paused {
        let sound = e.get(this, MapMenu::pCurrentSound);
        let handle = e.call(SELF, &args![sound]).u32();
        e.call(SOUND_PAUSE, &args![handle]);
        e.mem.set_u8(SOUND_PAUSED_011DA36F, 1);
        let now = e.call(TIMER_NOW, &args![TIMER_011F6394]).u32();
        e.mem.set_u32(PAUSE_TIME_011DA40C, now);
    } else if !e.call(IS_TOP_MENU_ID, &args![0x3f5u32]).bool()
        && !e.call(IS_TOP_MENU_ID, &args![0x3e9u32]).bool()
        && e.get(this, MapMenu::pCurrentSound) != 0
        && e.mem.u8(SOUND_PAUSED_011DA36F) != 0
    {
        let sound = e.get(this, MapMenu::pCurrentSound);
        let handle = e.call(SELF, &args![sound]).u32();
        e.call(SOUND_PLAY, &args![handle, 0u32]);
        e.mem.set_u8(SOUND_PAUSED_011DA36F, 0);
        let now = e.call(TIMER_NOW, &args![TIMER_011F6394]).u32();
        let start = now
            .wrapping_sub(e.mem.u32(PAUSE_TIME_011DA40C))
            .wrapping_add(e.get(this, MapMenu::iSoundPlayStartTime));
        e.set(this, MapMenu::iSoundPlayStartTime, start);
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_FLAG_00974D90, &args![pipboy]).bool() {
        return;
    }
    if e.call(IS_TOP_MENU_ID, &args![0x3ffu32]).bool() {
        let page = e.mem.u32(map_menu(e) + 0x80);
        // The compiler's switch on `page - 0x20` (0 to 4; jump table
        // `0079a5f4`).
        match page.wrapping_sub(0x20) {
            0 => {
                let wanted = e.call(BYTE_OF_00408D60, &args![OBJECT_011DA3B8]).u32();
                if e.mem.u8(wanted) != 0 {
                    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
                    if e.call(PIPBOY_FLAG_007FAEF0, &args![pipboy]).bool() {
                        let menu = map_menu(e);
                        e.mem.set_u32(menu + 0x128, 0);
                        for _ in 0..0x19 {
                            e.call(MAP_STEP_0079FFB0, &args![0u32]);
                        }
                    }
                }
                idle_show_position(e, this);
            }
            1 => idle_show_position(e, this),
            3 => {
                if e.get(this, MapMenu::uiNoteEnterStartTime) != 0 {
                    let now = e.call(TIMER_NOW, &args![TIMER_011F6394]).u32();
                    let elapsed =
                        f64::from(now.wrapping_sub(e.get(this, MapMenu::uiNoteEnterStartTime)));
                    let delay = e
                        .call(OBJECT_VALUE_ADDRESS_00403E20, &args![0x011d_47bcu32])
                        .u32();
                    let thousand: f64 = e.global(THOUSAND_DOUBLE);
                    let wait = x87_mul(x87_load32(e.mem.u32(delay)), thousand);
                    // `FCOMP; TEST AH,0x41; JP`: on when the wait is not
                    // above the elapsed time (ordered).
                    if matches!(
                        wait.partial_cmp(&elapsed),
                        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
                    ) {
                        e.with_stack(4, |e, note_slot| {
                            let selected = e.get(this, MapMenu::pSelectedNote);
                            e.mem.set_u32(note_slot.addr(), selected);
                            if e.call(FLAG_004B71D0, &args![]).bool()
                                || e.get(this, MapMenu::pSelectedNote) == 0
                            {
                                let note = fn_007a1910(e, Ptr::new(base + 0x160));
                                e.mem.set_u32(note_slot.addr(), note);
                            }
                            let note_tile = e
                                .call(LIST_BOX_GET_TILE_FOR, &args![base + 0x160, note_slot])
                                .u32();
                            let note = e.mem.u32(note_slot.addr());
                            if note != 0 && note_tile != 0 {
                                let alpha = float_constant(e, HALF_ALPHA_FLOAT);
                                let alpha_trait = e.mem.u32(TRAIT_TEXT_ALPHA);
                                e.call(TILE_SET_FLOAT, &args![note_tile, alpha_trait, alpha, 1u32]);
                                let note = e.mem.u32(note_slot.addr());
                                e.call(NOTE_SET_READ_005E9300, &args![note, 1u32]);
                                e.set(this, MapMenu::uiNoteEnterStartTime, 0);
                            }
                        });
                    }
                }
            }
            4 => {
                if e.call(FLAG_00798440, &args![]).bool() {
                    e.call(PAGE_0079C0C0, &args![this]);
                } else {
                    fn_0079c090(e, this);
                }
            }
            _ => {}
        }
        e.mem.set_u8(FADED_011DA408, 0);
    } else if e.mem.u8(FADED_011DA408) == 0
        && !e.call(FLAG_004B71D0, &args![]).bool()
        && e.get(this, MapMenu::iCurrentPage) <= 0x21
    {
        let alpha = float_constant(e, FULL_ALPHA_FLOAT);
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
        e.call(TILE_SET_FLOAT, &args![pipboy_tile, 0xfa9u32, alpha, 1u32]);
        e.mem.set_u8(FADED_011DA408, 1);
    }
    if e.get(this, MapMenu::bSoundsLoading) != 0 || e.get(this, MapMenu::pCurrentSound) != 0 {
        e.call(UPDATE_PLAYING_SOUNDS, &args![this]);
    }
}

/// The world and local map cases of `DoIdle` (pages 0x20 and 0x21), unless
/// `004b71d0` is set.
fn idle_show_position(e: &mut Engine, this: Ptr<MapMenu>) {
    if e.call(FLAG_004B71D0, &args![]).bool() {
        return;
    }
    let base = this.addr();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let cursor_x = e.call(CURSOR_X_007697F0, &args![interface]).f64();
    let tile_6 = map_tile(e, base, 6);
    let owner = e.call(WORD_AT_28, &args![tile_6]).u32();
    let left = e.call(TILE_GETTER_00A013D0, &args![owner]).f64();
    let mut x = x87_store32(x87_sub(cursor_x, left));
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let cursor_y = e.call(CURSOR_Y_007983C0, &args![interface]).f64();
    let tile_6 = map_tile(e, base, 6);
    let owner = e.call(WORD_AT_28, &args![tile_6]).u32();
    let top = e.call(TILE_GETTER_00A01440, &args![owner]).f64();
    let mut y = x87_store32(x87_sub(cursor_y, top));
    let zero: f64 = e.global(ZERO_DOUBLE);
    let width: f64 = e.global(MAP_WIDTH_DOUBLE);
    let height: f64 = e.global(MAP_HEIGHT_DOUBLE);
    let outside = below(x87_load32(x), zero)
        || below(x87_load32(y), zero)
        || above(x87_load32(x), width)
        || above(x87_load32(y), height);
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    if outside {
        let alpha = float_constant(e, FULL_ALPHA_FLOAT);
        let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
        e.call(TILE_SET_FLOAT, &args![pipboy_tile, 0xfa9u32, alpha, 1u32]);
    } else {
        let pipboy_tile = e.call(WORD_AT_28, &args![interface]).u32();
        e.call(
            TILE_SET_FLOAT,
            &args![pipboy_tile, 0xfa9u32, 0.0f32.to_bits(), 1u32],
        );
        let holder = e.mem.u32(OBJECT_011DEA0C);
        let object = e.call(OBJECT_00877720, &args![holder]).u32();
        if e.call(OBJECT_CHECK_00A23A50, &args![object, 1u32, 1u32])
            .u32()
            != 0
        {
            let holder = e.mem.u32(OBJECT_011DEA0C);
            let object = e.call(OBJECT_00877720, &args![holder]).u32();
            if e.call(OBJECT_CHECK_00A24660, &args![object, 0xeu32, 1u32])
                .u32()
                == 0
            {
                e.vcall(base, 0xc, &args![0xcu32, 0u32]);
            }
        }
    }
    let two: f64 = e.global(TWO_DOUBLE);
    let tile_6 = map_tile(e, base, 6);
    let width = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfb1u32]).f64();
    x = x87_store32(x87_sub(x87_m32(x), x87_div(width, two)));
    let tile_6 = map_tile(e, base, 6);
    let height = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfb0u32]).f64();
    y = x87_store32(x87_sub(x87_m32(y), x87_div(height, two)));
    let tile_6 = map_tile(e, base, 6);
    e.call(
        TILE_SET_FLOAT,
        &args![tile_6, 0xfa1u32, x87_quiet_bits(x), 1u32],
    );
    let tile_6 = map_tile(e, base, 6);
    e.call(
        TILE_SET_FLOAT,
        &args![tile_6, 0xfa2u32, x87_quiet_bits(y), 1u32],
    );
    e.call(SHOW_POSITION_00799DC0, &args![this]);
}

// Translated from 0079aba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079aba0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the two strings) is not translated.
/// `MapMenu::UpdateMenu` (Xbox PDB): with a map menu and the pip-boy up,
/// formats the date and time (`"%s, %d:%02d"`: the calendar's date string,
/// the hour and the minutes) into tile 1's text and, through `00578870` on
/// the player, tile 0's; when `00408b20(language, "ENGLISH")` is non-zero
/// sets the five tab titles of tiles 11 and 12 (recalculating their `iUT_Title`
/// value), the text of tile 17's child 0x24 and of `"MM_Headline_Title"`.
/// Then `UpdateCurrentPage`.
pub fn map_menu_update_menu(e: &mut Engine) {
    if map_menu(e) == 0 {
        return;
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_FLAG_00974D90, &args![pipboy]).bool() {
        return;
    }
    e.with_stack(0x10, |e, strings| {
        let text = strings.addr();
        let date = strings.addr() + 8;
        e.call(STRING_CONSTRUCT, &args![text]);
        let hour_value = e.call(CALENDAR_GET_HOUR, &args![CALENDAR_011DE7B8]).f64();
        let hour_bits = x87_store32(hour_value);
        let whole = e.call(FTOL2, &args![x87_load32(hour_bits)]).i32();
        let sixty: f64 = e.global(SIXTY_DOUBLE);
        let fraction = x87_sub(x87_m32(hour_bits), f64::from(whole));
        let minutes = e.call(FTOL2, &args![x87_mul(fraction, sixty)]).u32();
        let hour = e.call(FTOL2, &args![x87_load32(hour_bits)]).u32();
        let date_string = e
            .call(CALENDAR_GET_DATE_STRING, &args![CALENDAR_011DE7B8, date])
            .u32();
        let date_text = e.call(STRING_TEXT, &args![date_string]).u32();
        e.call(
            STRING_FORMAT,
            &args![text, DATE_TIME_FORMAT, date_text, hour, minutes],
        );
        e.call(STRING_DESTRUCT, &args![date]);
        let characters = e.call(STRING_TEXT, &args![text]).u32();
        let tile_1 = map_tile(e, map_menu(e), 1);
        e.call(TILE_SET_STRING, &args![tile_1, 0xfc4u32, characters, 1u32]);
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(PLAYER_MESSAGE_00578870, &args![player, text]);
        let characters = e.call(STRING_TEXT, &args![text]).u32();
        let tile_0 = map_tile(e, map_menu(e), 0);
        e.call(TILE_SET_STRING, &args![tile_0, 0xfc4u32, characters, 1u32]);
        let language = e.call(OBJECT_FIELD_00403DF0, &args![0x011c_3cb4u32]).u32();
        if e.call(COMPARE_00408B20, &args![language, ENGLISH_TEXT])
            .u32()
            != 0
        {
            for (tile_index, entries) in [
                (
                    11u32,
                    [
                        0x011d_1da8u32,
                        0x011d_3f64,
                        0x011d_45f4,
                        0x011d_3414,
                        0x011d_2f10,
                    ],
                ),
                (
                    12,
                    [
                        0x011d_3180,
                        0x011d_4cfc,
                        0x011d_27a8,
                        0x011d_3798,
                        0x011d_3210,
                    ],
                ),
            ] {
                for (entry, name) in entries.iter().zip(TITLE_N_TEXT) {
                    let title = e.call(OBJECT_FIELD_00403DF0, &args![*entry]).u32();
                    let title_trait = e.call(TEXT_TO_TRAIT, &args![name]).u32();
                    let tile = map_tile(e, map_menu(e), tile_index);
                    e.call(TILE_SET_STRING, &args![tile, title_trait, title, 1u32]);
                }
                let title_trait = e.mem.u32(TRAIT_TITLE);
                let tile = map_tile(e, map_menu(e), tile_index);
                let value = e.call(TILE_GET_VALUE, &args![tile, title_trait]).u32();
                e.call(VALUE_CALCULATE, &args![value, 1u32]);
            }
            let name = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_4aecu32]).u32();
            let tabline = map_tile(e, map_menu(e), 17);
            let child = e.call(TILE_GET_CHILD_BY_ID, &args![tabline, 0x24u32]).u32();
            e.call(TILE_SET_STRING, &args![child, 0xfc4u32, name, 1u32]);
            let headline = e.call(OBJECT_FIELD_00403DF0, &args![0x011d_21e4u32]).u32();
            let menu = map_menu(e);
            let menu_tile = e.call(WORD_AT_4, &args![menu]).u32();
            let child = e
                .call(
                    TILE_GET_CHILD_BY_NAME,
                    &args![menu_tile, HEADLINE_TITLE_TEXT],
                )
                .u32();
            e.call(TILE_SET_STRING, &args![child, 0xfc4u32, headline, 1u32]);
        }
        let menu = map_menu(e);
        e.call(UPDATE_CURRENT_PAGE, &args![menu]);
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 0079bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this + 0x6bc` (the class is unconfirmed).
pub fn fn_0079bc30(e: &mut Engine, this: Ptr) -> u32 {
    let _ = e;
    this.addr().wrapping_add(0x6bc)
}

// Translated from 0079c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears `fWaveformAnimationStartTime` and
/// tile 16's trait `0xfa3`.
pub fn fn_0079c090(e: &mut Engine, this: Ptr<MapMenu>) {
    e.set(this, MapMenu::fWaveformAnimationStartTime, 0.0);
    let tile = map_tile(e, this.addr(), 16);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
}

// Translated from 007a1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `011dea14`.
pub fn fn_007a1480(e: &mut Engine) -> u32 {
    e.mem.u32(VALUE_011DEA14)
}

// Translated from 007a1490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::ClearMapMemory` (Xbox PDB): with a map menu whose tile 2 has an
/// object (`0056c7f0`), hands every item of it to its vtable +0xe8, deletes
/// tile 2's and 4's children, clears trait `0xfa3` of tiles 2 and 3, the
/// fade controls `0xff5` of tiles 3 and 5 and both marker lists, the
/// waypoints, `uiNextLocalMapGrid` and `pMapTile`; clears the map tile's
/// texture (tile 16's shader property, its text trait `0xfcc` emptied) and
/// `0079c090`.
pub fn map_menu_clear_map_memory(e: &mut Engine) {
    if map_menu(e) == 0 {
        return;
    }
    let tile_2 = map_tile(e, map_menu(e), 2);
    let object = e.call(TILE_OBJECT_0056C7F0, &args![tile_2]).u32();
    if object == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.call(OBJECT_COUNT_0043B480, &args![object]).u32() {
        let item = e.call(OBJECT_ITEM_0043B4A0, &args![object, index]).u32();
        if item != 0 {
            e.vcall(object, 0xe8, &args![item]);
        }
        index = index.wrapping_add(1);
    }
    let tile_2 = map_tile(e, map_menu(e), 2);
    e.call(TILE_DELETE_CHILDREN, &args![tile_2]);
    let tile_2 = map_tile(e, map_menu(e), 2);
    e.call(TILE_SET_INT, &args![tile_2, 0xfa3u32, 0u32]);
    let tile_3 = map_tile(e, map_menu(e), 3);
    e.call(TILE_SET_INT, &args![tile_3, 0xfa3u32, 0u32]);
    let tile_3 = map_tile(e, map_menu(e), 3);
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile_3, 0xff5u32]);
    let menu = map_menu(e);
    e.call(LIST_REMOVE_ALL, &args![menu + 0xe0]);
    let tile_4 = map_tile(e, map_menu(e), 4);
    e.call(TILE_DELETE_CHILDREN, &args![tile_4]);
    let tile_5 = map_tile(e, map_menu(e), 5);
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile_5, 0xff5u32]);
    let menu = map_menu(e);
    e.call(LIST_REMOVE_ALL, &args![menu + 0xd8]);
    let menu = map_menu(e);
    e.call(WAYPOINTS_CLEAR, &args![menu + 0x220, 1u32]);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x128, 0);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x84, 0);
    let tile_16 = map_tile(e, map_menu(e), 16);
    let property = tile_shader_property(e, tile_16);
    if property != 0 && e.call(PROPERTY_CHECK_009C8640, &args![property]).u32() != 0 {
        e.call(SET_TILE_TEXTURE, &args![property, 0u32]);
        let tile_16 = map_tile(e, map_menu(e), 16);
        e.call(TILE_SET_STRING, &args![tile_16, 0xfccu32, EMPTY_TEXT, 1u32]);
    }
    let menu = map_menu(e);
    fn_0079c090(e, Ptr::new(menu));
}

// Translated from 007a1670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::ClearWorldMapTexture` (Xbox PDB): unless the map menu is on top
/// on the local map (page 0x21; `pMe` is read without a null check there),
/// clears tile 4's shader property texture (when `009c8640` says it has
/// one), empties its text trait `0xfcc` and sets the byte `011da36c`.
pub fn map_menu_clear_world_map_texture(e: &mut Engine) {
    if e.call(IS_TOP_MENU_ID, &args![0x3ffu32]).bool() && e.mem.u32(map_menu(e) + 0x80) == 0x21 {
        return;
    }
    if map_menu(e) == 0 {
        return;
    }
    let tile_4 = map_tile(e, map_menu(e), 4);
    let property = tile_shader_property(e, tile_4);
    if property != 0 && e.call(PROPERTY_CHECK_009C8640, &args![property]).u32() != 0 {
        e.call(SET_TILE_TEXTURE, &args![property, 0u32]);
        let tile_4 = map_tile(e, map_menu(e), 4);
        e.call(TILE_SET_STRING, &args![tile_4, 0xfccu32, EMPTY_TEXT, 1u32]);
        e.mem.set_u8(TEXTURE_CLEARED_011DA36C, 1);
    }
}

/// `007a17a0` and `007a17e0`: the magnification trait of the map menu's tile
/// `index`, 1.0 without a map menu.
fn magnification(e: &mut Engine, index: u32) -> f32 {
    let bits = if map_menu(e) == 0 {
        1.0f32.to_bits()
    } else {
        let id = e.mem.u32(TRAIT_MAGNIFICATION);
        let tile = map_tile(e, map_menu(e), index);
        tile_float_bits(e, tile, id)
    };
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 007a17a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the magnification (`iUT_Magnification`) of
/// the map menu's tile 2, 1.0 without a map menu.
pub fn fn_007a17a0(e: &mut Engine) -> f32 {
    magnification(e, 2)
}

// Translated from 007a17e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the magnification (`iUT_Magnification`) of
/// the map menu's tile 4, 1.0 without a map menu.
pub fn fn_007a17e0(e: &mut Engine) -> f32 {
    magnification(e, 4)
}

// Translated from 007a1880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::SetRootMapSpace` (Xbox PDB): with a map menu, sets
/// `pRootMapSpace` to `space` and walks it up to its top parent world
/// (`TESWorldSpace::GetParentWorldSimple`); `bRootSpaceChanged` says whether
/// it differs from before.
pub fn map_menu_set_root_map_space(e: &mut Engine, space: u32) {
    if map_menu(e) == 0 {
        return;
    }
    let old = e.mem.u32(map_menu(e) + 0x10c);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x10c, space);
    loop {
        let current = e.mem.u32(map_menu(e) + 0x10c);
        if current == 0 || e.call(WORLD_GET_PARENT, &args![current]).u32() == 0 {
            break;
        }
        let current = e.mem.u32(map_menu(e) + 0x10c);
        let parent = e.call(WORLD_GET_PARENT, &args![current]).u32();
        let menu = map_menu(e);
        e.mem.set_u32(menu + 0x10c, parent);
    }
    let changed = old != e.mem.u32(map_menu(e) + 0x10c);
    let menu = map_menu(e);
    e.mem.set_u8(menu + 0x110, u8::from(changed));
}

// Translated from 007a1910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the item of the list box's highlighted tile
/// (`007e1110(this, pHighlightedItem)`).
pub fn fn_007a1910(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(LIST_BOX_ITEM_OF_007E1110, &args![this, highlighted])
        .u32()
}

// Translated from 007a1930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `007a1950(this, 0, value)` (flag bit 0 of
/// `usFlags`).
pub fn fn_007a1930(e: &mut Engine, this: Ptr<ListBox>, value: u8) {
    fn_007a1950(e, this, 0, value);
}

// Translated from 007a1950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`value` non-zero) or clears bit `bit`
/// (the shift count masked to 5 bits, as `SHL` does) of the list box's
/// `usFlags`.
pub fn fn_007a1950(e: &mut Engine, this: Ptr<ListBox>, bit: u8, value: u8) {
    let mask = 1u32 << (u32::from(bit) & 0x1f);
    let flags = u32::from(e.get(this, ListBox::usFlags));
    let flags = if value != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.set(this, ListBox::usFlags, flags as u16);
}

// Translated from 007a1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<BSStringT<char>,1024>` constructor (RTTI of the vtable
/// `01075530`): the vtable, then `007a63f0(this, 0, 0)`. Returns `this`.
pub fn fn_007a1a70(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01075530);
    fn_007a63f0(e, this, 0, 0);
    this
}

// Translated from 007a1c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`): with a
/// buffer, destroys its elements (`007a63b0(buffer, iSize)`); with `release`
/// set also frees the buffer (`006a8500`) and clears the buffer and the
/// reserved size; then clears the size.
pub fn fn_007a1c30(e: &mut Engine, this: Ptr<BSSimpleArray>, release: u8) {
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.call(ARRAY_DESTROY_ELEMENTS_007A63B0, &args![this, buffer, size]);
    if release != 0 {
        e.call(ARRAY_FREE_006A8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 007a62e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a list box item (tile, the word at `data`,
/// the filtered byte cleared). Returns `this`.
pub fn fn_007a62e0(e: &mut Engine, this: Ptr, tile: u32, data: u32) -> Ptr {
    e.mem.set_u32(this.addr(), tile);
    let word = e.mem.u32(data);
    e.mem.set_u32(this.addr() + 4, word);
    e.mem.set_u8(this.addr() + 8, 0);
    this
}

// Translated from 007a63f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`'s
/// constructor body): clears the buffer, size and reserved size; reserves
/// `max(reserve, size)` elements through vtable +4 when that is non-zero,
/// then constructs `size` elements (`007a6310`) and sets the size.
pub fn fn_007a63f0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserve = if size > reserve { size } else { reserve };
    if reserve > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_CONSTRUCT_ELEMENTS_007A6310,
            &args![this, buffer, size],
        );
        e.set(this, BSSimpleArray::iSize, size);
    }
}

/// The function-local statics the list box methods keep trait ids in: the
/// guard word (bit `bit` says the id has been looked up) and the id.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

/// Looks the static trait id up the first time: the guard bit is set before
/// `Tile::AddUserTrait(name, -1)` runs, as the code does.
fn static_trait(e: &mut Engine, which: StaticTrait) {
    if e.mem.u32(which.guard) & which.bit == 0 {
        let guard = e.mem.u32(which.guard);
        e.mem.set_u32(which.guard, guard | which.bit);
        let id = e
            .call(ADD_USER_TRAIT, &args![which.name, 0xffff_ffffu32])
            .u32();
        e.mem.set_u32(which.cache, id);
    }
}

/// One instance of the `ListBox<T *>` template whose methods this unit
/// holds: the vtable, the statics of `IsEnabled` and `UpdateHighlightBoxPos`
/// and the instance's `NumUnfiltered` (not in this unit). The instances'
/// code is the same apart from these.
#[derive(Clone, Copy)]
struct ListBoxKind {
    vtable: u32,
    enabled: StaticTrait,
    highlight_y: StaticTrait,
    selected_height: StaticTrait,
    num_unfiltered: u32,
}

const fn list_box_kind(
    vtable: u32,
    enabled_guard: u32,
    highlight_guard: u32,
    num_unfiltered: u32,
) -> ListBoxKind {
    ListBoxKind {
        vtable,
        enabled: StaticTrait {
            guard: enabled_guard,
            bit: 1,
            cache: enabled_guard - 4,
            name: ENABLED_TEXT,
        },
        highlight_y: StaticTrait {
            guard: highlight_guard,
            bit: 1,
            cache: highlight_guard - 4,
            name: HIGHLIGHT_Y_TEXT,
        },
        selected_height: StaticTrait {
            guard: highlight_guard,
            bit: 2,
            cache: highlight_guard - 8,
            name: SELECTED_HEIGHT_TEXT,
        },
        num_unfiltered,
    }
}

/// `ListBox<TESQuest *>` (vtable RTTI `.?AV?$ListBox@PAVTESQuest@@@@`).
const QUEST_LIST: ListBoxKind = list_box_kind(0x0107_5544, 0x011d_a460, 0x011d_a46c, 0x007a_86e0);
/// `ListBox<BGSNote *>` (RTTI `.?AV?$ListBox@PAVBGSNote@@@@`).
const NOTE_LIST: ListBoxKind = list_box_kind(0x0107_556c, 0x011d_a47c, 0x011d_a488, 0x007a_6a50);
/// `ListBox<TESObjectREFR *>` (RTTI `.?AV?$ListBox@PAVTESObjectREFR@@@@`).
const REFERENCE_LIST: ListBoxKind =
    list_box_kind(0x0107_5594, 0x011d_a498, 0x011d_a4a4, 0x007a_71b0);
/// `ListBox<BGSQuestObjective *>` (RTTI
/// `.?AV?$ListBox@PAVBGSQuestObjective@@@@`).
const OBJECTIVE_LIST: ListBoxKind =
    list_box_kind(0x0107_55bc, 0x011d_a4b4, 0x011d_a4c0, 0x007a_7830);
/// `ListBox<TESChallenge *>` (RTTI `.?AV?$ListBox@PAVTESChallenge@@@@`).
const CHALLENGE_LIST: ListBoxKind =
    list_box_kind(0x0107_55e4, 0x011d_a4d0, 0x011d_a4dc, 0x007a_8030);

/// The constructor of a `ListBox<T *>` instance: the `ListBoxBase` base
/// (`007240c0`), the embedded list (+4, `0096a2d0`), the vtable, the fields
/// cleared, `007a1950(this, 0, 1)` and the list box handed to `00721130`.
fn list_box_construct(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), kind.vtable);
    e.set(this, ListBox::pScrollBar, Ptr::NULL);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.set(this, ListBox::pContainerRect, Ptr::NULL);
    e.set(this, ListBox::pTemplateName, Ptr::NULL);
    e.set(this, ListBox::usFlags, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fSavedScrollbarValue, 0.0);
    e.set(this, ListBox::fSavedHighlightIndex, 0.0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    fn_007a1950(e, this, 0, 1);
    e.call(LIST_BOX_REGISTER, &args![this]);
    this
}

/// The destructor of a `ListBox<T *>` instance: the vtable, `RemoveAll`, the
/// list box handed to `007211c0`, then the embedded list destroyed.
fn list_box_destruct(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    e.mem.set_u32(this.addr(), kind.vtable);
    list_box_remove_all(e, this, kind);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

/// The item word of a list node, reached through `006815c0` as the code
/// does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each
/// time: the tile through its deleting destructor (vtable slot 0, flag 1)
/// unless `KEEP_ITEM_TILES` is set, the data (`004459e0` with 1) when flag
/// bit 1 is set, then the item freed.
fn delete_item(e: &mut Engine, this: Ptr<ListBox>, item: impl Fn(&mut Engine) -> u32) {
    if e.mem.u8(KEEP_ITEM_TILES) == 0 {
        let entry = item(e);
        let tile = e.mem.u32(entry);
        if tile != 0 {
            e.vcall(tile, 0, &args![1u32]);
        }
    }
    if e.call(LIST_BOX_HAS_FLAG, &args![this, 1u32]).bool() {
        let entry = item(e);
        let data = e.mem.u32(entry + 4);
        if data != 0 {
            e.call(ITEM_DATA_DELETE, &args![data, 1u32]);
        }
    }
    let entry = item(e);
    e.call(FREE, &args![entry]);
}

/// `ListBox<T *>::RemoveAll`: while the second node (+8) and its item are
/// non-null, deletes the item, clears the node's next pointer (`006ecd40`
/// with 0), deletes the node (`004702f0` with 1) and makes the following node
/// the second; then deletes the first item (+4). Clears the first item,
/// `usNextIndex` and `fTotalListHeight`, sets the container's
/// `"_num_filtered"` to 0.0, then `SetTotalListHeight`, clears
/// `pHighlightedItem` and `UpdateHighlightBoxPos`.
fn list_box_remove_all(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    let base = this.addr();
    loop {
        let second = e.mem.u32(base + 8);
        if second == 0 || node_item(e, second) == 0 {
            break;
        }
        let second = e.mem.u32(base + 8);
        let next = e.call(WORD_AT_4, &args![second]).u32();
        delete_item(e, this, |e| {
            let node = e.mem.u32(base + 8);
            node_item(e, node)
        });
        let second = e.mem.u32(base + 8);
        e.call(NODE_SET_NEXT, &args![second, 0u32]);
        let second = e.mem.u32(base + 8);
        if second != 0 {
            e.call(NODE_DELETE, &args![second, 1u32]);
        }
        e.mem.set_u32(base + 8, next);
    }
    if e.mem.u32(base + 4) != 0 {
        delete_item(e, this, |e| e.mem.u32(base + 4));
    }
    e.mem.set_u32(base + 4, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    if !e.get(this, ListBox::pContainerRect).is_null() {
        let num_filtered = e.call(TEXT_TO_TRAIT, &args![NUM_FILTERED_TEXT]).u32();
        let container = e.get(this, ListBox::pContainerRect);
        e.call(
            TILE_SET_FLOAT,
            &args![container, num_filtered, 0.0f32.to_bits(), 1u32],
        );
    }
    list_box_set_total_list_height(e, this, kind);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    list_box_update_highlight_box_pos(e, this, kind);
}

/// `ListBox<T *>::Highlight`: when the list is enabled and `tile` is null,
/// or is one of its items (`007242f0`) and `"_enabled"`, makes it
/// `pHighlightedItem`, moves the highlight box and returns true; otherwise
/// false.
fn list_box_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32, kind: ListBoxKind) -> bool {
    if !list_box_is_enabled(e, this, kind) {
        return false;
    }
    if tile != 0 {
        if !e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() {
            return false;
        }
        let enabled = e
            .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
            .u32();
        if !e.call(TILE_IS_TRUE, &args![tile, enabled]).bool() {
            return false;
        }
    }
    e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    list_box_update_highlight_box_pos(e, this, kind);
    true
}

/// `ListBox<T *>::IsEnabled`: the container exists and its `"_enabled"`
/// trait (a function-local static id) is true.
fn list_box_is_enabled(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) -> bool {
    static_trait(e, kind.enabled);
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return false;
    }
    let enabled = e.mem.u32(kind.enabled.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

/// The tile's byte (`0071aa60`) set to `flag` and the tile queued for an
/// update with flags 8.
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_FLAG_SETTER_0071AA60, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

/// `ListBox<T *>::UpdateHighlightBoxPos`: looks the `"_highlight_y"` and
/// `"_selected_height"` trait ids up (statics); with a container and no
/// highlighted item, sets them to -1 and 0 on the container; with one, makes
/// both values follow the highlighted tile's traits `0xfa2` and `0xfb0`
/// (action `0x7d0`, then calculated) and gives the container's
/// `"lb_highlight_box"` tile, and its `"top"`, `"bottom"`, `"left"` and
/// `"right"` children when `"top"` exists (the other three are not checked
/// for null), the highlighted tile's byte (`0071aa80`, as 1 or 0) and an
/// update. The code's two branches differ only in that byte.
fn list_box_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    static_trait(e, kind.highlight_y);
    static_trait(e, kind.selected_height);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let y_trait = e.mem.u32(kind.highlight_y.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, y_trait, 0xffff_ffffu32]);
        let height_trait = e.mem.u32(kind.selected_height.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, height_trait, 0u32]);
        return;
    }
    let y_trait = e.mem.u32(kind.highlight_y.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_trait = e.mem.u32(kind.selected_height.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let height_value = e
        .call(TILE_GET_VALUE, &args![container, height_trait])
        .u32();
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![y_value, 0x7d0u32, highlighted, 0xfa2u32],
    );
    e.call(VALUE_CALCULATE, &args![y_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![height_value, 0x7d0u32, highlighted, 0xfb0u32],
    );
    e.call(VALUE_CALCULATE, &args![height_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let flag = u32::from(
        e.call(TILE_FLAG_GETTER_0071AA80, &args![highlighted])
            .bool(),
    );
    let container = e.get(this, ListBox::pContainerRect);
    let box_tile = e
        .call(TILE_FIND_BY_NAME, &args![container, HIGHLIGHT_BOX_TEXT])
        .u32();
    if box_tile == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![box_tile, TOP_TEXT])
        .u32();
    if top != 0 {
        mark_tile(e, top, flag);
        for name in [BOTTOM_TEXT, LEFT_TEXT, RIGHT_TEXT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![box_tile, name]).u32();
            mark_tile(e, edge, flag);
        }
    }
    mark_tile(e, box_tile, flag);
}

/// `ListBox<T *>::SetTotalListHeight`: with a container, its visible height
/// (trait `0xfb0`, stored as a `float`) and the total list height decide the
/// number of scroll steps: 1, unless both are above 0.0 and the visible
/// height is below the total, then `00647b70(_ftol2(004e4430(unfiltered *
/// (total - visible) / total)) + 1, 2)` (the ratio stored as a `float`).
/// With both above 0.0 and more than one step, the container's
/// `"_scroll_delta"` becomes `(total - visible) / (steps - 1)`; with a scroll
/// bar its `"_number_of_items"` becomes the number of steps.
fn list_box_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = x87_store32(e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64());
    let mut steps = 1i32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let total_bits = e.mem.u32(this.addr() + 0x20);
    if above(x87_load32(total_bits), zero) && above(x87_load32(visible), zero) {
        // `FLD total; FLD visible; FCOMPP; TEST AH,5; JP`: on only when the
        // visible height is below the total (ordered).
        let total_bits = e.mem.u32(this.addr() + 0x20);
        if x87_load32(visible) < x87_load32(total_bits) {
            let count = e.call(kind.num_unfiltered, &args![this]).u32();
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let product = x87_mul(f64::from(count), difference);
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let ratio = x87_store32(x87_div(product, x87_m32(total_bits)));
            let rounded = e.call(ROUND_FLOAT, &args![x87_quiet_bits(ratio)]).f64();
            let whole = e.call(FTOL2, &args![rounded]).i32();
            steps = e
                .call(INT_PAIR_00647B70, &args![whole.wrapping_add(1), 2i32])
                .i32();
        }
        if steps > 1 {
            let total_bits = e.mem.u32(this.addr() + 0x20);
            let difference = x87_sub(x87_load32(total_bits), x87_m32(visible));
            let delta = x87_store32(x87_div(difference, f64::from(steps - 1)));
            let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            e.call(
                TILE_SET_FLOAT,
                &args![container, scroll_delta, x87_quiet_bits(delta), 1u32],
            );
        }
    }
    if !e.get(this, ListBox::pScrollBar).is_null() {
        let number_of_items = e.call(TEXT_TO_TRAIT, &args![NUMBER_OF_ITEMS_TEXT]).u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        e.call(TILE_SET_INT, &args![scroll_bar, number_of_items, steps]);
    }
}

// Translated from 007a1c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a1c90 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESQuest *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a1c90(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, QUEST_LIST)
}

// Translated from 007a1d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a1d60 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESQuest *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a1d60(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, QUEST_LIST);
}

// Translated from 007a2070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_tes_quest_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, QUEST_LIST);
}

// Translated from 007a23e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_tes_quest_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, QUEST_LIST)
}

// Translated from 007a6550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6550 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESQuest_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_tes_quest_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, QUEST_LIST)
}

// Translated from 007a6600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6600 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESQuest_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_tes_quest_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, QUEST_LIST);
}

// Translated from 007a6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_tes_quest_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, QUEST_LIST);
}

// Translated from 007a27f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a27f0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<BGSNote *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a27f0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, NOTE_LIST)
}

// Translated from 007a28c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a28c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<BGSNote *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a28c0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, NOTE_LIST);
}

// Translated from 007a2bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_bgs_note_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, NOTE_LIST);
}

// Translated from 007a3170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_bgs_note_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, NOTE_LIST)
}

// Translated from 007a6bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6bd0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSNote_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_bgs_note_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, NOTE_LIST)
}

// Translated from 007a6c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6c80 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<BGSNote_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_bgs_note_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, NOTE_LIST);
}

// Translated from 007a7080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_bgs_note_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, NOTE_LIST);
}

// Translated from 007a36e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a36e0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESObjectREFR *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a36e0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, REFERENCE_LIST)
}

// Translated from 007a37b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a37b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESObjectREFR *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a37b0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, REFERENCE_LIST);
}

// Translated from 007a3ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_tes_object_refr_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, REFERENCE_LIST);
}

// Translated from 007a3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_tes_object_refr_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, REFERENCE_LIST)
}

// Translated from 007a7330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a7330 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESObjectREFR_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_tes_object_refr_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, REFERENCE_LIST)
}

// Translated from 007a73e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a73e0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESObjectREFR_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_tes_object_refr_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, REFERENCE_LIST);
}

// Translated from 007a7700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_tes_object_refr_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, REFERENCE_LIST);
}

// Translated from 007a43a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a43a0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<BGSQuestObjective *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a43a0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, OBJECTIVE_LIST)
}

// Translated from 007a4470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a4470 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<BGSQuestObjective *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a4470(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, OBJECTIVE_LIST);
}

// Translated from 007a4780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_bgs_quest_objective_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, OBJECTIVE_LIST);
}

// Translated from 007a4ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_bgs_quest_objective_highlight(
    e: &mut Engine,
    this: Ptr<ListBox>,
    tile: u32,
) -> bool {
    list_box_highlight(e, this, tile, OBJECTIVE_LIST)
}

// Translated from 007a79b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a79b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSQuestObjective_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_bgs_quest_objective_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, OBJECTIVE_LIST)
}

// Translated from 007a7a60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a7a60 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<BGSQuestObjective_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_bgs_quest_objective_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, OBJECTIVE_LIST);
}

// Translated from 007a7d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_bgs_quest_objective_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, OBJECTIVE_LIST);
}

// Translated from 007a50f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a50f0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<TESChallenge *>` constructor (unnamed in the engine map; RTTI of its vtable):
/// see [`list_box_construct`]. Returns `this`.
pub fn fn_007a50f0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, CHALLENGE_LIST)
}

// Translated from 007a51c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a51c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<TESChallenge *>` destructor (unnamed in the engine map): see
/// [`list_box_destruct`].
pub fn fn_007a51c0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, CHALLENGE_LIST);
}

// Translated from 007a54d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::RemoveAll` (engine map): see [`list_box_remove_all`].
pub fn list_box_tes_challenge_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, CHALLENGE_LIST);
}

// Translated from 007a5a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::Highlight` (engine map): see [`list_box_highlight`].
pub fn list_box_tes_challenge_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, tile, CHALLENGE_LIST)
}

// Translated from 007a81b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a81b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESChallenge_P>::IsEnabled` (engine map): see [`list_box_is_enabled`].
pub fn list_box_tes_challenge_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, CHALLENGE_LIST)
}

// Translated from 007a8260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a8260 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<TESChallenge_P>::UpdateHighlightBoxPos` (engine map): see
/// [`list_box_update_highlight_box_pos`].
pub fn list_box_tes_challenge_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, CHALLENGE_LIST);
}

// Translated from 007a8580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::SetTotalListHeight` (engine map): see
/// [`list_box_set_total_list_height`].
pub fn list_box_tes_challenge_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, CHALLENGE_LIST);
}

// ---------------------------------------------------------------------------
// Batch n056: the rest of the unit's `MapMenu` methods and `ListBox<T *>`
// instances.
// ---------------------------------------------------------------------------

/// Cdecl, no arguments: the word at `01189480` (`fallout shared/renderer.cpp`;
/// `007983c0` divides by it).
const RENDERER_VALUE_004DC200: u32 = 0x004d_c200;
/// The `double` 960.0 `007983c0` multiplies by (`0106e7f8`).
const SCALE_0106E7F8: u32 = 0x0106_e7f8;
/// The `double` 1.0 at `01012070`.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The two `float`s `DoControllerSwitch` gives tile 6's traits `0xfa1` and
/// `0xfa2` (348.0 and 170.0 in the shipped exe).
const CONTROLLER_X_FLOAT: u32 = 0x0107_4f74;
const CONTROLLER_Y_FLOAT: u32 = 0x0107_4f70;
/// A byte `00798440` returns.
const FLAG_011DD434: u32 = 0x011d_d434;
/// The word `0079c070` returns, and the address `0079c080` returns.
const VALUE_011DD42C: u32 = 0x011d_d42c;
const OBJECT_011DD59C: u32 = 0x011d_d59c;
/// The object `0043d4d0` is called on in `007a1750` (its result is read as a
/// word pointer).
const OBJECT_011C63CC: u32 = 0x011c_63cc;
const WORD_POINTER_0043D4D0: u32 = 0x0043_d4d0;
/// `BaseProcess::GetCurrentProcedureIndex` (engine map; the body is read as a
/// thiscall returning a word).
const PROCESS_VALUE_0044EDB0: u32 = 0x0044_edb0;
/// Thiscall, `AL` (`0079be30`).
const FORM_FLAG_00440DA0: u32 = 0x0044_0da0;
/// `BGSSaveFormBuffer::GetForm` (engine map), thiscall without stack words.
const SAVE_FORM_BUFFER_GET_FORM: u32 = 0x007a_f430;
/// Cdecl (form, word), `AL`.
const FORM_FLAG_004FEE60: u32 = 0x004f_ee60;
/// `FOPipboyManager::SetTabKnobPosition` (engine map), thiscall (page, word),
/// `RET 8`.
const SET_TAB_KNOB_POSITION: u32 = 0x007f_a0f0;
/// Thiscall on the subtitle array (`BSSimpleArray<BSStringT<char>,1024>`):
/// the index of a new element at the end.
const ARRAY_ADD_SLOT_00845460: u32 = 0x0084_5460;
/// Thiscall on a `BSStringT<char>` (text), `RET 4`: the assignment.
const STRING_ASSIGN_00501C10: u32 = 0x0050_1c10;
/// Cdecl (size, address): the placement `operator new` (returns the address).
const PLACEMENT_NEW_006E6DA0: u32 = 0x006e_6da0;
/// Thiscall on the embedded list (+4) (compare function), `RET 4`: sorts it.
const LIST_SORT_0083FD60: u32 = 0x0083_fd60;
/// `BSSimpleList<ActiveEffect_P>::Insert` (engine map; folded), thiscall on
/// the embedded list (item, compare function), `RET 8`.
const LIST_INSERT_SORTED_007A7EB0: u32 = 0x007a_7eb0;
/// Thiscall on the embedded list (address of the item), `RET 4`: appends it.
const LIST_ADD_TAIL_00905820: u32 = 0x0090_5820;
/// `ListBox<StartMenu::Option_P>::InitTilePosition` (engine map; folded),
/// thiscall on the list box (tile), `RET 4`.
const LIST_BOX_INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// Thiscall on the list box: the number of its items.
const LIST_BOX_COUNT_00726C40: u32 = 0x0072_6c40;
/// `Menu::RenderTemplate` (engine map), thiscall (parent, template, word),
/// `RET 0xC`.
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// `Tile::GetValueQ` (engine map), thiscall (trait), `RET 4`.
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// `Tile::Value::ClearActions` (engine map), thiscall.
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
/// `Tile::Value::AddAction` (engine map), thiscall (action, float), `RET 8`.
const VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
/// `Tile::Poke` (engine map), thiscall (trait, float), `RET 8`.
const TILE_POKE: u32 = 0x00a0_1290;
/// `Tile::GetMenuTile` (engine map), thiscall.
const TILE_GET_MENU_TILE: u32 = 0x00a0_3d40;
/// Thiscall on a tile (trait, count), `RET 8`.
const TILE_SET_COUNT_00715C60: u32 = 0x0071_5c60;
/// `InterfaceManager::SetCurrentFocusTarget` (engine map), thiscall (tile,
/// trait, word), `RET 0xC`.
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
/// Cdecl (float, float): a value in `ST0` (`007a3580`, `GetHighlightTarget`).
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
const FLOAT_PAIR_004BD510: u32 = 0x004b_d510;
/// `"_number_of_visible_items"`, `"_SetInCode"`.
const NUMBER_OF_VISIBLE_ITEMS_TEXT: u32 = 0x0106_f860;
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;

// Translated from 007983c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the map menu code calls it on the interface
/// manager): the `float` at +0x4B0 times 960.0 (`0106e7f8`), divided by the
/// renderer's word (`004dc200`), stored as a `float`.
pub fn fn_007983c0(e: &mut Engine, this: Ptr) -> f32 {
    let value = e.mem.u32(this.addr() + 0x4b0);
    let scale: f64 = e.global(SCALE_0106E7F8);
    // `FMUL double; FSTP double`: the product kept as a double.
    let product = x87_mul(x87_load32(value), scale);
    let divisor = e.call(RENDERER_VALUE_004DC200, &args![]).i32();
    let bits = x87_store32(x87_div(product, f64::from(divisor)));
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 00798400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at +0x6F4 (the map menu code calls it
/// on the player).
pub fn fn_00798400(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x6f4)
}

// Translated from 00798420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at +0x7D (the map menu code calls it
/// on a note).
pub fn fn_00798420(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x7d)
}

// Translated from 00798440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at `011dd434`.
pub fn fn_00798440(e: &mut Engine) -> u8 {
    e.mem.u8(FLAG_011DD434)
}

// Translated from 00798480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoDownClick` (Xbox PDB): a press on id 4 (the world map) keeps
/// tile 4's traits `0xfa1` and `0xfa2` as `WorldMapDragStart`. `tile` is not
/// read.
pub fn map_menu_do_down_click(e: &mut Engine, this: Ptr<MapMenu>, id: i32, _unused_2: u32) {
    if id != 4 {
        return;
    }
    let tile = map_tile(e, this.addr(), 4);
    let x = tile_float_bits(e, tile, 0xfa1);
    e.mem.set_u32(this.addr() + 0x120, x);
    let tile = map_tile(e, this.addr(), 4);
    let y = tile_float_bits(e, tile, 0xfa2);
    e.mem.set_u32(this.addr() + 0x124, y);
}

// Translated from 0079a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoControllerSwitch` (Xbox PDB): when `on` is set, tile 6 is
/// moved to the floats at `01074f74` (trait `0xfa1`) and `01074f70` (`0xfa2`).
pub fn map_menu_do_controller_switch(e: &mut Engine, this: Ptr<MapMenu>, on: u8) {
    if on == 0 {
        return;
    }
    let x = float_constant(e, CONTROLLER_X_FLOAT);
    let tile = map_tile(e, this.addr(), 6);
    e.call(TILE_SET_FLOAT, &args![tile, 0xfa1u32, x, 1u32]);
    let y = float_constant(e, CONTROLLER_Y_FLOAT);
    let tile = map_tile(e, this.addr(), 6);
    e.call(TILE_SET_FLOAT, &args![tile, 0xfa2u32, y, 1u32]);
}

// Translated from 0079b430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): clears the open map menu's
/// `pSelectedNote` when it is `note`.
pub fn fn_0079b430(e: &mut Engine, note: u32) {
    if map_menu(e) == 0 {
        return;
    }
    if e.mem.u32(map_menu(e) + 0x90) == note {
        let menu = map_menu(e);
        e.mem.set_u32(menu + 0x90, 0);
    }
}

// Translated from 0079b800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 2 of the 16-bit word at +0x5C is
/// set (the class is unconfirmed).
pub fn fn_0079b800(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u16(this.addr() + 0x5c) & 4 != 0
}

// Translated from 0079bc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): 1 when `object` is set and its
/// `0044edb0` word differs from the map menu's `pDisplayedQuest` (`pMe` read
/// without a null check), else 0.
pub fn fn_0079bc50(e: &mut Engine, object: u32) -> u8 {
    if object == 0 {
        return 0;
    }
    let word = e.call(PROCESS_VALUE_0044EDB0, &args![object]).u32();
    u8::from(e.mem.u32(map_menu(e) + 0xcc) != word)
}

// Translated from 0079be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; a radio list filter callback): 1 when
/// `object` is null, when `00440da0` says so, or when `004fee60` says so of
/// its `007af430` form (with 0); else 0.
pub fn fn_0079be30(e: &mut Engine, object: u32) -> u8 {
    if object == 0 || e.call(FORM_FLAG_00440DA0, &args![object]).bool() {
        return 1;
    }
    let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![object]).u32();
    u8::from(e.call(FORM_FLAG_004FEE60, &args![form, 0u32]).bool())
}

// Translated from 0079be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): sets the tile's selected trait (the
/// static `011da348`) to 0.
pub fn fn_0079be80(e: &mut Engine, tile: u32) {
    let selected = e.mem.u32(TRAIT_SELECTED);
    e.call(TILE_SET_INT, &args![tile, selected, 0u32]);
}

// Translated from 0079c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `011dd42c`.
pub fn fn_0079c070(e: &mut Engine) -> u32 {
    e.mem.u32(VALUE_011DD42C)
}

// Translated from 0079c080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the address `011dd59c`.
pub fn fn_0079c080(e: &mut Engine) -> u32 {
    let _ = e;
    OBJECT_011DD59C
}

// Translated from 0079c340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): with an open map menu, moves the
/// pip-boy's tab knob (`FOPipboyManager::SetTabKnobPosition` on the
/// interface manager's `00704370` object) to the current page - 0x20 with
/// `flag`.
pub fn fn_0079c340(e: &mut Engine, flag: u8) {
    if map_menu(e) == 0 {
        return;
    }
    let page = e.mem.u32(map_menu(e) + 0x80).wrapping_sub(0x20);
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let pipboy = e.call(WORD_AT_174, &args![interface]).u32();
    e.call(SET_TAB_KNOB_POSITION, &args![pipboy, page, u32::from(flag)]);
}

// Translated from 007a1400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the bits of `mask` from the word at
/// +0x30 when any of them is set (`XOR` with the mask, as the code does).
pub fn fn_007a1400(e: &mut Engine, this: Ptr, mask: u32) {
    let word = e.mem.u32(this.addr() + 0x30);
    if word & mask != 0 {
        e.mem.set_u32(this.addr() + 0x30, word ^ mask);
    }
}

// Translated from 007a1430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores `value` in the byte at +0x92.
pub fn fn_007a1430(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x92, value);
}

// Translated from 007a1450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the four words at `from` to +0x68.
pub fn fn_007a1450(e: &mut Engine, this: Ptr, from: u32) {
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(this.addr() + 0x68 + 4 * word, value);
    }
}

// Translated from 007a1750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: with an open map menu, whether
/// `uiNextLocalMapGrid` is below the square of the word `0043d4d0` points at
/// (on `011c63cc`, read twice; unsigned, the product wrapping).
pub fn fn_007a1750(e: &mut Engine) -> bool {
    if map_menu(e) == 0 {
        return false;
    }
    let first = e.call(WORD_POINTER_0043D4D0, &args![OBJECT_011C63CC]).u32();
    let second = e.call(WORD_POINTER_0043D4D0, &args![OBJECT_011C63CC]).u32();
    let limit = e.mem.u32(first).wrapping_mul(e.mem.u32(second));
    e.mem.u32(map_menu(e) + 0x128) < limit
}

/// `007a1820` and `007a1850`: with an open map menu, the magnification
/// (`iUT_Magnification`, the static `011da35c`) of tile `index` set to
/// `value`.
fn set_magnification(e: &mut Engine, index: u32, value: f32) {
    if map_menu(e) == 0 {
        return;
    }
    let bits = x87_quiet_bits(value.to_bits());
    let id = e.mem.u32(TRAIT_MAGNIFICATION);
    let tile = map_tile(e, map_menu(e), index);
    e.call(TILE_SET_FLOAT, &args![tile, id, bits, 1u32]);
}

// Translated from 007a1820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): sets the magnification of the map
/// menu's tile 2 (see [`set_magnification`]).
pub fn fn_007a1820(e: &mut Engine, value: f32) {
    set_magnification(e, 2, value);
}

// Translated from 007a1850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): sets the magnification of the map
/// menu's tile 4 (see [`set_magnification`]).
pub fn fn_007a1850(e: &mut Engine, value: f32) {
    set_magnification(e, 4, value);
}

// Translated from 007a1ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`): adds a
/// slot at the end (`00845460`), constructs the string there (`007a6310`
/// with 1) and assigns `text` (`00501c10`). Returns the slot's index.
pub fn fn_007a1ac0(e: &mut Engine, this: Ptr<BSSimpleArray>, text: u32) -> u32 {
    let index = e.call(ARRAY_ADD_SLOT_00845460, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    fn_007a6310(e, this, buffer.wrapping_add(index.wrapping_mul(8)), 1);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = buffer.wrapping_add(index.wrapping_mul(8));
    e.call(STRING_ASSIGN_00501C10, &args![slot, text]);
    index
}

// Translated from 007a6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSStringT<char>,1024>`'s scalar deleting destructor (engine
/// map): the destructor `007a1aa0`, then the object freed when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_007a6100(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007a1aa0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// A list box's scalar deleting destructor: `destructor`, then the object
/// freed when bit 0 of `flags` is set. Returns `this`.
fn list_box_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
    destructor: fn(&mut Engine, Ptr<ListBox>),
) -> Ptr<ListBox> {
    destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007a6130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>`'s scalar deleting destructor (engine map):
/// `007a1d60`, then freed on bit 0 of `flags`.
pub fn fn_007a6130(e: &mut Engine, this: Ptr<ListBox>, flags: u32) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_007a1d60)
}

// Translated from 007a6160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>`'s scalar deleting destructor (engine map):
/// `007a28c0`, then freed on bit 0 of `flags`.
pub fn fn_007a6160(e: &mut Engine, this: Ptr<ListBox>, flags: u32) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_007a28c0)
}

// Translated from 007a6190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>`'s scalar deleting destructor (engine map):
/// `007a37b0`, then freed on bit 0 of `flags`.
pub fn fn_007a6190(e: &mut Engine, this: Ptr<ListBox>, flags: u32) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_007a37b0)
}

// Translated from 007a61c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>`'s scalar deleting destructor (engine map):
/// `007a4470`, then freed on bit 0 of `flags`.
pub fn fn_007a61c0(e: &mut Engine, this: Ptr<ListBox>, flags: u32) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_007a4470)
}

// Translated from 007a61f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>`'s scalar deleting destructor (engine map):
/// `007a51c0`, then freed on bit 0 of `flags`.
pub fn fn_007a61f0(e: &mut Engine, this: Ptr<ListBox>, flags: u32) -> Ptr<ListBox> {
    list_box_deleting_destructor(e, this, flags, fn_007a51c0)
}

// Translated from 007a6310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6310 - the C++ exception frame (FS:[0] chain, unwind state 0 around each element's construction) is not translated.
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`):
/// constructs `count` strings from `buffer` on (8 bytes each): the placement
/// `operator new` (`006e6da0`), then the string constructor when it gives an
/// address.
pub fn fn_007a6310(e: &mut Engine, this: Ptr<BSSimpleArray>, buffer: u32, count: u32) {
    let _ = this;
    let mut index = 0u32;
    while index < count {
        let slot = buffer.wrapping_add(index.wrapping_mul(8));
        let at = e.call(PLACEMENT_NEW_006E6DA0, &args![8u32, slot]).u32();
        if at != 0 {
            e.call(STRING_CONSTRUCT, &args![at]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 007a63b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`):
/// destroys `count` strings from `buffer` on (`007a86b0` with 0).
pub fn fn_007a63b0(e: &mut Engine, this: Ptr<BSSimpleArray>, buffer: u32, count: u32) {
    let _ = this;
    let mut index = 0u32;
    while index < count {
        let slot = buffer.wrapping_add(index.wrapping_mul(8));
        fn_007a86b0(e, Ptr::new(slot), 0);
        index = index.wrapping_add(1);
    }
}

// Translated from 007a86b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStringT<char>`'s scalar deleting destructor (unnamed in the engine
/// map): the string destructor (`004037d0`), then the object freed when bit
/// 0 of `flags` is set. Returns `this`.
pub fn fn_007a86b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(STRING_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007a5a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the list box container's trait `0xfa3`
/// to `value`.
pub fn fn_007a5a30(e: &mut Engine, this: Ptr<ListBox>, value: u8) {
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_SET_INT, &args![container, 0xfa3u32, u32::from(value)]);
}

/// The function-local statics of the `ListBox<T *>` methods this batch
/// translates (guard word, bit 1, id at the guard - 4).
const fn list_box_static(guard: u32, name: u32) -> StaticTrait {
    StaticTrait {
        guard,
        bit: 1,
        cache: guard - 4,
        name,
    }
}

/// `SetEnabled`'s `"_enabled"` statics, per instance.
const QUEST_SET_ENABLED: StaticTrait = list_box_static(0x011d_a420, ENABLED_TEXT);
const NOTE_SET_ENABLED: StaticTrait = list_box_static(0x011d_a430, ENABLED_TEXT);
const OBJECTIVE_SET_ENABLED: StaticTrait = list_box_static(0x011d_a448, ENABLED_TEXT);
const CHALLENGE_SET_ENABLED: StaticTrait = list_box_static(0x011d_a458, ENABLED_TEXT);
/// `Filter`'s `"_num_filtered"` statics, per instance.
const NOTE_FILTER: StaticTrait = list_box_static(0x011d_a428, NUM_FILTERED_TEXT);
const REFERENCE_FILTER: StaticTrait = list_box_static(0x011d_a438, NUM_FILTERED_TEXT);
const OBJECTIVE_FILTER: StaticTrait = list_box_static(0x011d_a440, NUM_FILTERED_TEXT);
const CHALLENGE_FILTER: StaticTrait = list_box_static(0x011d_a450, NUM_FILTERED_TEXT);
/// `NumUnfiltered`'s `"_num_filtered"` statics, per instance.
const NOTE_NUM_FILTERED: StaticTrait = list_box_static(0x011d_a474, NUM_FILTERED_TEXT);
const REFERENCE_NUM_FILTERED: StaticTrait = list_box_static(0x011d_a490, NUM_FILTERED_TEXT);
const OBJECTIVE_NUM_FILTERED: StaticTrait = list_box_static(0x011d_a4ac, NUM_FILTERED_TEXT);
const CHALLENGE_NUM_FILTERED: StaticTrait = list_box_static(0x011d_a4c8, NUM_FILTERED_TEXT);
const QUEST_NUM_FILTERED: StaticTrait = list_box_static(0x011d_a4e4, NUM_FILTERED_TEXT);

/// `ListBox<T *>::Sort`: sorts the embedded list (+4) with `compare`
/// (`0083fd60`), then `Synchronize`.
fn list_box_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32, kind: ListBoxKind) {
    e.call(LIST_SORT_0083FD60, &args![this.addr() + 4, compare]);
    list_box_synchronize(e, this, kind);
}

/// `ListBox<T *>::SetEnabled`: the container's `"_enabled"` trait (a
/// function-local static id) set to `value` (the container is not checked).
fn list_box_set_enabled(e: &mut Engine, this: Ptr<ListBox>, value: u8, which: StaticTrait) {
    static_trait(e, which);
    let enabled = e.mem.u32(which.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_SET_INT, &args![container, enabled, u32::from(value)]);
}

/// `ListBox<T *>::Synchronize`: clears `usNextIndex` and `fTotalListHeight`,
/// gives every item that is not filtered out (byte +8 of the item clear) the
/// next index as trait `0xfac` and its tile position (`007269d0`), then
/// `SetTotalListHeight` and `UpdateHighlightBoxPos`.
fn list_box_synchronize(e: &mut Engine, this: Ptr<ListBox>, kind: ListBoxKind) {
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u8(item + 8) == 0 {
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let tile = item_tile(e, node);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let tile = item_tile(e, node);
            e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
        }
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    list_box_set_total_list_height(e, this, kind);
    list_box_update_highlight_box_pos(e, this, kind);
}

/// `ListBox<T *>::NumUnfiltered`: the number of items (`00726c40`) minus the
/// container's `"_num_filtered"` (a function-local static id; the float
/// truncated to a 64-bit integer, its low word used).
fn list_box_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>, which: StaticTrait) -> u32 {
    static_trait(e, which);
    let count = e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32();
    let id = e.mem.u32(which.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let filtered = e.call(TILE_GET_FLOAT, &args![container, id]).f64();
    let filtered = x87_fistp64_chop(X87Ext::from(filtered)) as u32;
    count.wrapping_sub(filtered)
}

/// `007a6220`, `007a6250`, `007a6280`, `007a62b0`: the tile of the last
/// unfiltered item, through the list box's vtable +0x14 (index
/// `NumUnfiltered - 1`, 0).
fn list_box_last_tile(e: &mut Engine, this: Ptr<ListBox>, which: StaticTrait) -> u32 {
    let count = list_box_num_unfiltered(e, this, which);
    e.vcall(this.addr(), 0x14, &args![count.wrapping_sub(1), 0u32])
        .u32()
}

/// The scroll steps of a page for `GetHighlightTarget` (directions 0xf and
/// 0x10): 0 without a container; otherwise the container's height (trait
/// `0xfb0`, negated for `up`) over its `"_scroll_delta"`, stored as a
/// `float`, through `004bd510(.., 1.0)` and `_ftol2`.
fn list_box_page_steps(e: &mut Engine, this: Ptr<ListBox>, up: bool) -> i32 {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return 0;
    }
    let container = e.get(this, ListBox::pContainerRect);
    let height = e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64();
    let height = if up { -height } else { height };
    let delta_id = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let delta = e.call(TILE_GET_FLOAT, &args![container, delta_id]).f64();
    let ratio = x87_quiet_bits(x87_store32(x87_div(height, delta)));
    let rounded = e
        .call(FLOAT_PAIR_004BD510, &args![ratio, 1.0f32.to_bits()])
        .f64();
    e.call(FTOL2, &args![rounded]).i32()
}

/// Whether the tile's `"_enabled"` trait (looked up each time) is true.
fn tile_enabled(e: &mut Engine, tile: u32) -> bool {
    let enabled = e
        .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(TILE_IS_TRUE, &args![tile, enabled]).bool()
}

/// `ListBox<T *>::GetHighlightTarget`: the tile a move in `direction` (1 up,
/// 2 down, 0xf a page up, 0x10 a page down; others none) leads to. With the
/// list enabled, a step and items: the index starts at the highlighted
/// tile's (trait `0xfac`) plus the step, kept in 0..=last (without a
/// highlighted tile: the first for a step down, the last for one up), and
/// the item with that index is looked for; a disabled one makes it step on
/// by one. Past the end it walks back from last - 1 (through vtable +0x14)
/// and before the start forward from 1, until an enabled tile. Returns the
/// tile, or 0 when it is the highlighted one or none was found.
fn list_box_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
    kind: ListBoxKind,
) -> u32 {
    let mut target = 0u32;
    // The compiler's switch: a byte table maps `direction - 1` (0 to 0xf,
    // unsigned) to the four cases and the default.
    let step = match direction.wrapping_sub(1) {
        0 => -1,
        1 => 1,
        0xe => list_box_page_steps(e, this, true),
        0xf => list_box_page_steps(e, this, false),
        _ => 0,
    };
    'end: {
        if !list_box_is_enabled(e, this, kind) || step == 0 {
            break 'end;
        }
        if e.get(this, ListBox::usNextIndex) == 0 {
            break 'end;
        }
        let last = i32::from(e.get(this, ListBox::usNextIndex)) - 1;
        let mut index = if e.get(this, ListBox::pHighlightedItem).is_null() {
            if step > 0 {
                0
            } else {
                last
            }
        } else {
            let highlighted = e.get(this, ListBox::pHighlightedItem);
            let value = e.call(TILE_GET_FLOAT, &args![highlighted, 0xfacu32]).f64();
            let index = e.call(FTOL2, &args![value]).i32().wrapping_add(step);
            if index < 0 {
                0
            } else if index > last {
                last
            } else {
                index
            }
        };
        loop {
            let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
            while target == 0 && node != 0 && node_item(e, node) != 0 {
                let tile = item_tile(e, node);
                let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                if equal(f64::from(index), value) {
                    target = item_tile(e, node);
                }
                node = e.call(WORD_AT_4, &args![node]).u32();
            }
            if target == 0 || tile_enabled(e, target) {
                break 'end;
            }
            target = 0;
            if step > 0 {
                index = index.wrapping_add(1);
                if index > last {
                    index = last.wrapping_sub(1);
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'end;
                        }
                        if index < 0 {
                            break 'end;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index = index.wrapping_sub(1);
                    }
                }
            }
            if step < 0 {
                index = index.wrapping_sub(1);
                if index < 0 {
                    index = 1;
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'end;
                        }
                        if index > last {
                            break 'end;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index = index.wrapping_add(1);
                    }
                }
            }
        }
    }
    let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
    if target == highlighted {
        0
    } else {
        target
    }
}

/// A `ListBoxItem` (tile, the word at `data`, filtered byte clear) made with
/// `operator new` (12 bytes) and `007a62e0`; 0 when the allocation gives 0.
fn new_list_box_item(e: &mut Engine, tile: u32, data: u32) -> u32 {
    let item = e.call(ALLOCATE, &args![0xcu32]).u32();
    if item == 0 {
        return 0;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), data);
        fn_007a62e0(e, Ptr::new(item), tile, slot.addr()).addr()
    })
}

/// `ListBox<T *>::Insert` (unnamed in the engine map): with a container and
/// a template (`template`, else `pTemplateName`), renders the template into
/// the container (`Menu::RenderTemplate` on the container's menu, word 0),
/// gives the tile trait `0xfaa` -1 when it has none and `text` as its string
/// `0xfc4`; makes the item (`data`) and adds it sorted with `compare`
/// (`007a7eb0`, then `Synchronize` when `007242c0(0)`) or at the end
/// (`00905820`; when `007242c0(0)`: the tile's position and
/// `SetTotalListHeight`; then the next index as trait `0xfac`). When the list
/// then has one item and the container's `"_number_of_visible_items"` is
/// above 0.0, the container's height (trait `0xfb0`) becomes the tile's
/// height (action `0x7d0`) times that number (action `0x7d3`). Returns the
/// tile, or 0 without a container or template.
fn list_box_insert(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
    kind: ListBoxKind,
) -> u32 {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return 0;
    }
    if e.get(this, ListBox::pTemplateName).is_null() && template == 0 {
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
    if e.call(TILE_GET_VALUE, &args![tile, 0xfaau32]).u32() == 0 {
        e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0xffff_ffffu32]);
    }
    if text != 0 {
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    }
    if compare != 0 {
        let item = new_list_box_item(e, tile, data);
        e.call(
            LIST_INSERT_SORTED_007A7EB0,
            &args![this.addr() + 4, item, compare],
        );
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            list_box_synchronize(e, this, kind);
        }
    } else {
        let item = new_list_box_item(e, tile, data);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_ADD_TAIL_00905820, &args![this.addr() + 4, slot]);
        });
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
            list_box_set_total_list_height(e, this, kind);
        }
        let index = e.get(this, ListBox::usNextIndex);
        e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
        e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
    }
    if e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32() == 1 {
        let visible_id = e
            .call(
                ADD_USER_TRAIT,
                &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
            )
            .u32();
        let container = e.get(this, ListBox::pContainerRect);
        let visible = e.call(TILE_GET_FLOAT, &args![container, visible_id]).f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if above(visible, zero) {
            let container = e.get(this, ListBox::pContainerRect);
            let value = e.call(TILE_GET_VALUE_Q, &args![container, 0xfb0u32]).u32();
            e.call(VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![value, 0x7d0u32, tile, 0xfb0u32],
            );
            let visible_id = e
                .call(
                    ADD_USER_TRAIT,
                    &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
                )
                .u32();
            let container = e.get(this, ListBox::pContainerRect);
            let visible = e.call(TILE_GET_FLOAT, &args![container, visible_id]).f64();
            e.call(
                VALUE_ADD_ACTION_FLOAT,
                &args![value, 0x7d3u32, x87_store32(visible)],
            );
            e.call(VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    tile
}

/// `ListBox<T *>::Filter`: clears `usNextIndex` and `fTotalListHeight`; for
/// every item, marks it filtered (byte +8) when `callback` (cdecl, on the
/// item's data) says so. A filtered item gets index -1 (trait `0xfac`) and
/// is counted; when it was highlighted, its index is kept and the highlight
/// cleared. An unfiltered one takes the kept highlight when its new index
/// is the kept one, gets the next index and its tile position. Then the
/// container's `"_num_filtered"` (a function-local static id) is the count
/// (`00715c60`), `SetTotalListHeight`; a highlight lost with a kept index
/// goes to vtable +0x14 (kept index, 0), or else to the last unfiltered
/// item (`last`); then `UpdateHighlightBoxPos`.
fn list_box_filter(
    e: &mut Engine,
    this: Ptr<ListBox>,
    callback: u32,
    which: StaticTrait,
    kind: ListBoxKind,
    last: fn(&mut Engine, Ptr<ListBox>) -> u32,
) {
    let mut filtered = 0u32;
    let mut kept = -1i32;
    static_trait(e, which);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 && node_item(e, node) != 0 {
        let out = callback != 0 && {
            let item = node_item(e, node);
            let data = e.mem.u32(item + 4);
            e.call(callback, &args![data]).bool()
        };
        let item = node_item(e, node);
        e.mem.set_u8(item + 8, u8::from(out));
        let item = node_item(e, node);
        if e.mem.u8(item + 8) != 0 {
            let item = node_item(e, node);
            if e.mem.u32(item) == e.get(this, ListBox::pHighlightedItem).addr() {
                let tile = item_tile(e, node);
                let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                kept = e.call(FTOL2, &args![value]).i32();
                e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
            }
            let tile = item_tile(e, node);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, 0xffff_ffffu32]);
            filtered = filtered.wrapping_add(1);
        } else {
            if kept >= 0 && i32::from(e.get(this, ListBox::usNextIndex)) == kept {
                let tile = item_tile(e, node);
                e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
            }
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let tile = item_tile(e, node);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let tile = item_tile(e, node);
            e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
        }
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    let id = e.mem.u32(which.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_SET_COUNT_00715C60, &args![container, id, filtered]);
    list_box_set_total_list_height(e, this, kind);
    if e.get(this, ListBox::pHighlightedItem).is_null() && kept >= 0 {
        let tile = e.vcall(this.addr(), 0x14, &args![kept, 0u32]).u32();
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() && kept >= 0 {
        let tile = last(e, this);
        e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    }
    list_box_update_highlight_box_pos(e, this, kind);
}

// Translated from 007a22a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::Sort` (Xbox PDB): see [`list_box_sort`].
pub fn list_box_tes_quest_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, compare, QUEST_LIST);
}

// Translated from 007a2e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::Sort` (Xbox PDB): see [`list_box_sort`].
pub fn list_box_bgs_note_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, compare, NOTE_LIST);
}

// Translated from 007a3cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::Sort` (Xbox PDB): see [`list_box_sort`].
pub fn list_box_tes_object_refr_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, compare, REFERENCE_LIST);
}

// Translated from 007a49b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::Sort` (Xbox PDB): see [`list_box_sort`].
pub fn list_box_bgs_quest_objective_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, compare, OBJECTIVE_LIST);
}

// Translated from 007a5700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::Sort` (Xbox PDB): see [`list_box_sort`].
pub fn list_box_tes_challenge_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, compare, CHALLENGE_LIST);
}

// Translated from 007a2350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a2350 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESQuest_P>::SetEnabled` (Xbox PDB): see
/// [`list_box_set_enabled`].
pub fn list_box_tes_quest_set_enabled(e: &mut Engine, this: Ptr<ListBox>, value: u8) {
    list_box_set_enabled(e, this, value, QUEST_SET_ENABLED);
}

// Translated from 007a30e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a30e0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSNote_P>::SetEnabled` (Xbox PDB): see
/// [`list_box_set_enabled`].
pub fn list_box_bgs_note_set_enabled(e: &mut Engine, this: Ptr<ListBox>, value: u8) {
    list_box_set_enabled(e, this, value, NOTE_SET_ENABLED);
}

// Translated from 007a4c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a4c50 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSQuestObjective_P>::SetEnabled` (Xbox PDB): see
/// [`list_box_set_enabled`].
pub fn list_box_bgs_quest_objective_set_enabled(e: &mut Engine, this: Ptr<ListBox>, value: u8) {
    list_box_set_enabled(e, this, value, OBJECTIVE_SET_ENABLED);
}

// Translated from 007a59a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a59a0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESChallenge_P>::SetEnabled` (Xbox PDB): see
/// [`list_box_set_enabled`].
pub fn list_box_tes_challenge_set_enabled(e: &mut Engine, this: Ptr<ListBox>, value: u8) {
    list_box_set_enabled(e, this, value, CHALLENGE_SET_ENABLED);
}

// Translated from 007a2450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::GetHighlightTarget` (Xbox PDB): see
/// [`list_box_get_highlight_target`].
pub fn list_box_tes_quest_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, direction, QUEST_LIST)
}

// Translated from 007a31e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::GetHighlightTarget` (Xbox PDB): see
/// [`list_box_get_highlight_target`].
pub fn list_box_bgs_note_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, direction, NOTE_LIST)
}

// Translated from 007a4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::GetHighlightTarget` (Xbox PDB): see
/// [`list_box_get_highlight_target`].
pub fn list_box_tes_object_refr_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, direction, REFERENCE_LIST)
}

// Translated from 007a4d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::GetHighlightTarget` (Xbox PDB): see
/// [`list_box_get_highlight_target`].
pub fn list_box_bgs_quest_objective_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, direction, OBJECTIVE_LIST)
}

// Translated from 007a5ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::GetHighlightTarget` (Xbox PDB): see
/// [`list_box_get_highlight_target`].
pub fn list_box_tes_challenge_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, direction, CHALLENGE_LIST)
}

// Translated from 007a1dd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a1dd0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the items' allocation) is not translated.
/// `ListBox<TESQuest *>::Insert` (unnamed in the engine map): see
/// [`list_box_insert`].
pub fn fn_007a1dd0(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_insert(e, this, data, text, compare, template, QUEST_LIST)
}

// Translated from 007a2930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a2930 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the items' allocation) is not translated.
/// `ListBox<BGSNote *>::Insert` (unnamed in the engine map): see
/// [`list_box_insert`].
pub fn fn_007a2930(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_insert(e, this, data, text, compare, template, NOTE_LIST)
}

// Translated from 007a3820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a3820 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the items' allocation) is not translated.
/// `ListBox<TESObjectREFR *>::Insert` (unnamed in the engine map): see
/// [`list_box_insert`].
pub fn fn_007a3820(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_insert(e, this, data, text, compare, template, REFERENCE_LIST)
}

// Translated from 007a44e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a44e0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the items' allocation) is not translated.
/// `ListBox<BGSQuestObjective *>::Insert` (unnamed in the engine map): see
/// [`list_box_insert`].
pub fn fn_007a44e0(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_insert(e, this, data, text, compare, template, OBJECTIVE_LIST)
}

// Translated from 007a5230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a5230 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the items' allocation) is not translated.
/// `ListBox<TESChallenge *>::Insert` (unnamed in the engine map): see
/// [`list_box_insert`].
pub fn fn_007a5230(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_insert(e, this, data, text, compare, template, CHALLENGE_LIST)
}

// Translated from 007a2e30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a2e30 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSNote_P>::Filter` (Xbox PDB): see [`list_box_filter`].
pub fn list_box_bgs_note_filter(e: &mut Engine, this: Ptr<ListBox>, callback: u32) {
    list_box_filter(e, this, callback, NOTE_FILTER, NOTE_LIST, fn_007a6220);
}

// Translated from 007a3d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a3d20 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESObjectREFR_P>::Filter` (Xbox PDB): see [`list_box_filter`].
pub fn list_box_tes_object_refr_filter(e: &mut Engine, this: Ptr<ListBox>, callback: u32) {
    list_box_filter(
        e,
        this,
        callback,
        REFERENCE_FILTER,
        REFERENCE_LIST,
        fn_007a6250,
    );
}

// Translated from 007a49e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a49e0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSQuestObjective_P>::Filter` (Xbox PDB): see
/// [`list_box_filter`].
pub fn list_box_bgs_quest_objective_filter(e: &mut Engine, this: Ptr<ListBox>, callback: u32) {
    list_box_filter(
        e,
        this,
        callback,
        OBJECTIVE_FILTER,
        OBJECTIVE_LIST,
        fn_007a6280,
    );
}

// Translated from 007a5730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a5730 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESChallenge_P>::Filter` (Xbox PDB): see [`list_box_filter`].
pub fn list_box_tes_challenge_filter(e: &mut Engine, this: Ptr<ListBox>, callback: u32) {
    list_box_filter(
        e,
        this,
        callback,
        CHALLENGE_FILTER,
        CHALLENGE_LIST,
        fn_007a62b0,
    );
}

// Translated from 007a6220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the engine map's `ListBox<BGSNote_P>::NumUnfiltered` name does
/// not fit the body): the `ListBox<BGSNote *>`'s last unfiltered tile (see
/// [`list_box_last_tile`]).
pub fn fn_007a6220(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_last_tile(e, this, NOTE_NUM_FILTERED)
}

// Translated from 007a6250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `ListBox<TESObjectREFR *>`'s last
/// unfiltered tile (see [`list_box_last_tile`]).
pub fn fn_007a6250(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_last_tile(e, this, REFERENCE_NUM_FILTERED)
}

// Translated from 007a6280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the engine map's `ListBox<BGSQuestObjective_P>::NumUnfiltered`
/// name does not fit the body): the `ListBox<BGSQuestObjective *>`'s last
/// unfiltered tile (see [`list_box_last_tile`]).
pub fn fn_007a6280(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_last_tile(e, this, OBJECTIVE_NUM_FILTERED)
}

// Translated from 007a62b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `ListBox<TESChallenge *>`'s last
/// unfiltered tile (see [`list_box_last_tile`]).
pub fn fn_007a62b0(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_last_tile(e, this, CHALLENGE_NUM_FILTERED)
}

// Translated from 007a6480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESQuest_P>::Synchronize` (Xbox PDB): see
/// [`list_box_synchronize`].
pub fn list_box_tes_quest_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, QUEST_LIST);
}

// Translated from 007a6b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSNote_P>::Synchronize` (Xbox PDB): see
/// [`list_box_synchronize`].
pub fn list_box_bgs_note_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, NOTE_LIST);
}

// Translated from 007a7260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESObjectREFR_P>::Synchronize` (Xbox PDB): see
/// [`list_box_synchronize`].
pub fn list_box_tes_object_refr_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, REFERENCE_LIST);
}

// Translated from 007a78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<BGSQuestObjective_P>::Synchronize` (Xbox PDB): see
/// [`list_box_synchronize`].
pub fn list_box_bgs_quest_objective_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, OBJECTIVE_LIST);
}

// Translated from 007a80e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESChallenge_P>::Synchronize` (Xbox PDB): see
/// [`list_box_synchronize`].
pub fn list_box_tes_challenge_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, CHALLENGE_LIST);
}

// Translated from 007a6a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a6a50 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSNote *>::NumUnfiltered` (by the body; unnamed in the engine
/// map): see [`list_box_num_unfiltered`].
pub fn fn_007a6a50(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_num_unfiltered(e, this, NOTE_NUM_FILTERED)
}

// Translated from 007a71b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a71b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESObjectREFR_P>::NumUnfiltered` (Xbox PDB): see
/// [`list_box_num_unfiltered`].
pub fn list_box_tes_object_refr_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_num_unfiltered(e, this, REFERENCE_NUM_FILTERED)
}

// Translated from 007a7830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a7830 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<BGSQuestObjective *>::NumUnfiltered` (by the body; unnamed in
/// the engine map): see [`list_box_num_unfiltered`].
pub fn fn_007a7830(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_num_unfiltered(e, this, OBJECTIVE_NUM_FILTERED)
}

// Translated from 007a8030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a8030 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESChallenge_P>::NumUnfiltered` (Xbox PDB): see
/// [`list_box_num_unfiltered`].
pub fn list_box_tes_challenge_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_num_unfiltered(e, this, CHALLENGE_NUM_FILTERED)
}

// Translated from 007a86e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a86e0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<TESQuest_P>::NumUnfiltered` (Xbox PDB): see
/// [`list_box_num_unfiltered`].
pub fn list_box_tes_quest_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    list_box_num_unfiltered(e, this, QUEST_NUM_FILTERED)
}

// Translated from 007a3580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a `ListBox<BGSNote *>` method restoring the
/// saved state): with a container and a scroll bar, puts
/// `fSavedScrollbarValue` back as the scroll bar's `"_current_value"` (flag
/// 0) and pokes its `"_SetInCode"` with 1.0; the tile at index
/// `0040ebd0(fSavedHighlightIndex, NumUnfiltered - 1.0)` (truncated)
/// through vtable +0x14 gets the focus (`SetCurrentFocusTarget(tile, 0xfc3,
/// focus)`) and vtable +0 (highlight) with it; unless `004b71d0`, the
/// tile's menu gets vtable +0x14 (the tile's trait `0xfaa`, the tile) when
/// there is a tile, and vtable +0 is called again with 0.
pub fn fn_007a3580(e: &mut Engine, this: Ptr<ListBox>, focus: u8) {
    if e.get(this, ListBox::pContainerRect).is_null() || e.get(this, ListBox::pScrollBar).is_null()
    {
        return;
    }
    let saved = x87_quiet_bits(e.mem.u32(this.addr() + 0x28));
    let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_bar, current_value, saved, 0u32],
    );
    let set_in_code = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    let scroll_bar = e.get(this, ListBox::pScrollBar);
    e.call(TILE_POKE, &args![scroll_bar, set_in_code, 1.0f32.to_bits()]);
    let count = fn_007a6a50(e, this);
    // `FILD qword` of the count with a zero high word, minus 1.0, stored as a
    // `float`.
    let one: f64 = e.global(ONE_DOUBLE);
    let last = x87_quiet_bits(x87_store32(x87_sub(f64::from(count), one)));
    let index = x87_quiet_bits(e.mem.u32(this.addr() + 0x24));
    let chosen = e.call(FLOAT_PAIR_0040EBD0, &args![index, last]).f64();
    let chosen = x87_fistp64_chop(X87Ext::from(chosen)) as u32;
    let tile = e.vcall(this.addr(), 0x14, &args![chosen, 0u32]).u32();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.call(
        SET_CURRENT_FOCUS_TARGET,
        &args![interface, tile, 0xfc3u32, u32::from(focus)],
    );
    e.vcall(this.addr(), 0, &args![tile]);
    if e.call(FLAG_004B71D0, &args![]).bool() {
        return;
    }
    if tile != 0 {
        let container = e.get(this, ListBox::pContainerRect);
        let menu_tile = e.call(TILE_GET_MENU_TILE, &args![container]).u32();
        let menu = e.call(TILE_GET_MENU, &args![menu_tile]).u32();
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfaau32]).f64();
        let id = e.call(FTOL2, &args![value]).u32();
        e.vcall(menu, 0x14, &args![id, tile]);
    }
    e.vcall(this.addr(), 0, &args![0u32]);
}

/// The tile (word +0) of a list node's item, reached through `006815c0`.
fn item_tile(e: &mut Engine, node: u32) -> u32 {
    let item = node_item(e, node);
    e.mem.u32(item)
}

/// The word at `011c3f2c`: the object whose list (+0x10, `00460140`) holds
/// the map markers `CalculatePlayerMarkerSpace` walks.
const MARKER_OWNER_011C3F2C: u32 = 0x011c_3f2c;
/// The `float` `CalculatePlayerMarkerSpace` copies into an unused local
/// (`01016970`, `FLT_MAX` in the shipped exe).
const LARGEST_VALUE_01016970: u32 = 0x0101_6970;
/// `MapMarkerData::GetLocationName` (engine map), thiscall.
const MARKER_GET_LOCATION_NAME: u32 = 0x0040_8da0;
/// `PathingLocation::PathingLocation` (engine map), thiscall; the `ov6`
/// overload (point, reference), `RET 8`; `operator=` (location), `RET 4`;
/// and the destructor `004ff7e0`.
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
const PATHING_LOCATION_CONSTRUCT_AT: u32 = 0x006d_cea0;
const PATHING_LOCATION_ASSIGN: u32 = 0x006d_cce0;
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// Thiscall on a `PathingLocation`: `AL`.
const PATHING_LOCATION_FLAG_006DE250: u32 = 0x006d_e250;
/// `BSstristr` (engine map), cdecl (text, pattern).
const BS_STRISTR: u32 = 0x004b_75a0;
/// `"test"` (`01017bfc`).
const TEST_TEXT: u32 = 0x0101_7bfc;
/// Thiscall on a reference: `AL`.
const REFERENCE_FLAG_00586210: u32 = 0x0058_6210;
/// Thiscall on a form: a string (`CalculatePlayerMarkerSpace` compares the
/// text from +0x1D on).
const FORM_NAME_00586440: u32 = 0x0058_6440;
/// Cdecl (text): its length.
const TEXT_LENGTH_0044A670: u32 = 0x0044_a670;
/// Cdecl (text, text, count): a comparison of the first `count` characters
/// (0 when equal).
const TEXT_COMPARE_N_004564F0: u32 = 0x0045_64f0;

// Translated from 007984d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007984d0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the two PathingLocation locals) is not translated.
/// `MapMenu::CalculatePlayerMarkerSpace` (Xbox PDB): walks the map markers
/// of the list at `[011c3f2c] + 0x10`; a marker counts when its location
/// name is not empty and does not start with `"test"` (`BSstristr`), when
/// `00586210` is false, when the point (`x`, `y`, `z`, adjusted by
/// `TESWorldSpace::AdjustMapMarkerCoord` with 1) as a `PathingLocation` of
/// the marker passes `006de250`, `pRootMapSpace` is set, and the texts
/// (`00586440`) of both, at least 0x1F long, agree in their three
/// characters from +0x1D. Returns the first marker that counts, else the
/// list's first item.
pub fn map_menu_calculate_player_marker_space(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    x: u32,
    y: u32,
    z: u32,
) -> u32 {
    let mut found = 0u32;
    // The code copies this float into a local it never reads.
    let _largest = e.mem.u32(LARGEST_VALUE_01016970);
    let owner = e.mem.u32(MARKER_OWNER_011C3F2C);
    let mut node = e.call(PLUS_10, &args![owner]).u32();
    e.with_stack(0x5c, |e, frame| {
        // The frame's locals: the point (+0), the location (+0xC) and the
        // temporary location (+0x34).
        let point = frame.addr();
        let location = point + 0xc;
        let temporary = point + 0x34;
        while node != 0 && node_item(e, node) != 0 {
            let marker = node_item(e, node);
            let name = e
                .call(MARKER_GET_LOCATION_NAME, &args![marker + 0x18])
                .u32();
            e.mem.set_u32(point, x);
            e.mem.set_u32(point + 4, y);
            e.mem.set_u32(point + 8, z);
            e.call(PATHING_LOCATION_CONSTRUCT, &args![location]);
            let skip = name == 0
                || e.mem.u8(name) == 0
                || e.call(BS_STRISTR, &args![name, TEST_TEXT]).u32() == name
                || e.call(REFERENCE_FLAG_00586210, &args![marker]).bool();
            if !skip {
                e.call(ADJUST_MAP_MARKER_COORD, &args![marker, point, 1u32]);
                let made = e
                    .call(
                        PATHING_LOCATION_CONSTRUCT_AT,
                        &args![temporary, point, marker],
                    )
                    .u32();
                e.call(PATHING_LOCATION_ASSIGN, &args![location, made]);
                e.call(PATHING_LOCATION_DESTRUCT, &args![temporary]);
                let root = e.get(this, MapMenu::pRootMapSpace);
                if e.call(PATHING_LOCATION_FLAG_006DE250, &args![location])
                    .bool()
                    && root != 0
                    && marker != 0
                {
                    let root = e.get(this, MapMenu::pRootMapSpace);
                    let root_text = e.call(FORM_NAME_00586440, &args![root]).u32();
                    let long_enough = e.call(TEXT_LENGTH_0044A670, &args![root_text]).u32() >= 0x1f
                        && {
                            let text = e.call(FORM_NAME_00586440, &args![marker]).u32();
                            e.call(TEXT_LENGTH_0044A670, &args![text]).u32() >= 0x1f
                        };
                    if long_enough {
                        let text = e.call(FORM_NAME_00586440, &args![marker]).u32();
                        let root = e.get(this, MapMenu::pRootMapSpace);
                        let root_text = e.call(FORM_NAME_00586440, &args![root]).u32();
                        let differs = e
                            .call(
                                TEXT_COMPARE_N_004564F0,
                                &args![root_text.wrapping_add(0x1d), text.wrapping_add(0x1d), 3u32],
                            )
                            .u32()
                            != 0;
                        if !differs {
                            found = marker;
                            e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
                            break;
                        }
                    }
                }
            }
            e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
            node = e.call(WORD_AT_4, &args![node]).u32();
        }
    });
    if found == 0 {
        let owner = e.mem.u32(MARKER_OWNER_011C3F2C);
        let head = e.call(PLUS_10, &args![owner]).u32();
        found = node_item(e, head);
    }
    found
}

/// `Interface::GetMessageMenuResult` (engine map), cdecl: `AL`.
const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// Thiscall on the interface manager (callback), `RET 4`: the callback the
/// fast travel question runs next (`0070f690`).
const SET_MESSAGE_CALLBACK_0070F690: u32 = 0x0070_f690;
/// The callback `00798710` hands on: `00798a00`.
const FAST_TRAVEL_CONFIRMED_CALLBACK: u32 = 0x0079_8a00;
/// `PlayerCharacter::SetPlayerMapMarker` (engine map), thiscall (x, y, z,
/// world space), `RET 0x10`, and `RemovePlayerMapMarker`, thiscall.
const SET_PLAYER_MAP_MARKER: u32 = 0x0095_2e60;
const REMOVE_PLAYER_MAP_MARKER: u32 = 0x0095_2f90;
/// `PlayerCharacter::RequestPositionPlayer` (engine map), thiscall (request),
/// `RET 4`.
const REQUEST_POSITION_PLAYER: u32 = 0x0093_be30;
/// Thiscall: the constructor of the 0x34-byte request `00798a00` makes.
const POSITION_REQUEST_CONSTRUCT: u32 = 0x0045_6540;

/// The block `00798710` and `00798840` end with: unless `004b71d0`, tile 6
/// is centred on the cursor (`007697f0` and `007983c0` on the interface
/// manager, minus tile 6's owner's `00a013d0` / `00a01440` and half of tile
/// 6's traits `0xfb1` / `0xfb0`, into `0xfa1` / `0xfa2`); then tile 6's trait
/// `0xfa3` is set to 1.
fn centre_cursor_tile(e: &mut Engine) {
    if !e.call(FLAG_004B71D0, &args![]).bool() {
        let two: f64 = e.global(TWO_DOUBLE);
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let cursor_x = e.call(CURSOR_X_007697F0, &args![interface]).f64();
        let tile = map_tile(e, map_menu(e), 6);
        let owner = e.call(WORD_AT_28, &args![tile]).u32();
        let left = e.call(TILE_GETTER_00A013D0, &args![owner]).f64();
        let x = x87_sub(cursor_x, left);
        let tile = map_tile(e, map_menu(e), 6);
        let width = e.call(TILE_GET_FLOAT, &args![tile, 0xfb1u32]).f64();
        let x = x87_quiet_bits(x87_store32(x87_sub(x, x87_div(width, two))));
        let tile = map_tile(e, map_menu(e), 6);
        e.call(TILE_SET_FLOAT, &args![tile, 0xfa1u32, x, 1u32]);
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let cursor_y = f64::from(fn_007983c0(e, Ptr::new(interface)));
        let tile = map_tile(e, map_menu(e), 6);
        let owner = e.call(WORD_AT_28, &args![tile]).u32();
        let top = e.call(TILE_GETTER_00A01440, &args![owner]).f64();
        let y = x87_sub(cursor_y, top);
        let tile = map_tile(e, map_menu(e), 6);
        let height = e.call(TILE_GET_FLOAT, &args![tile, 0xfb0u32]).f64();
        let y = x87_quiet_bits(x87_store32(x87_sub(y, x87_div(height, two))));
        let tile = map_tile(e, map_menu(e), 6);
        e.call(TILE_SET_FLOAT, &args![tile, 0xfa2u32, y, 1u32]);
    }
    let tile = map_tile(e, map_menu(e), 6);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
}

// Translated from 00798710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the fast travel question's callback): on
/// "no" (`Interface::GetMessageMenuResult` 0) the interface manager gets
/// `00798a00` as the next callback (`0070f690`), otherwise
/// `pFastTravelMarker` is cleared; then the cursor tile is centred (see
/// [`centre_cursor_tile`]).
pub fn fn_00798710(e: &mut Engine) {
    if !e.call(GET_MESSAGE_MENU_RESULT, &args![]).bool() {
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        e.call(
            SET_MESSAGE_CALLBACK_0070F690,
            &args![interface, FAST_TRAVEL_CONFIRMED_CALLBACK],
        );
    } else {
        let menu = map_menu(e);
        e.mem.set_u32(menu + 0x118, 0);
    }
    centre_cursor_tile(e);
}

// Translated from 00798840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::SetPlayerMarkerCallback` (Xbox PDB): by the message box's
/// result (`Interface::GetMessageMenuResult`): 0 or 3 set the player's map
/// marker at `PlayerMarkerPos` in `PlayerMarkerSpace`
/// (`PlayerCharacter::SetPlayerMapMarker`), delete the old marker on 0
/// (`DeletePlayerMarker`) and create the new one (`CreatePlayerMarker`); 1
/// removes the player's marker (`RemovePlayerMapMarker`) and deletes it.
/// Then `PlayerMarkerPos` is reset from `011f426c`, `PlayerMarkerSpace`
/// cleared and the cursor tile centred (see [`centre_cursor_tile`]).
pub fn map_menu_set_player_marker_callback(e: &mut Engine) {
    let result = e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8();
    match result {
        0 | 3 => {
            let menu = map_menu(e);
            let space = e.mem.u32(menu + 0xf8);
            let x = e.mem.u32(menu + 0xfc);
            let y = e.mem.u32(menu + 0x100);
            let z = e.mem.u32(menu + 0x104);
            let player = e.mem.u32(PLAYER_011DEA3C);
            e.call(SET_PLAYER_MAP_MARKER, &args![player, x, y, z, space]);
            if result == 0 {
                let menu = map_menu(e);
                map_menu_delete_player_marker(e, Ptr::new(menu));
            }
            let menu = map_menu(e);
            map_menu_create_player_marker(e, Ptr::new(menu));
        }
        1 => {
            let player = e.mem.u32(PLAYER_011DEA3C);
            e.call(REMOVE_PLAYER_MAP_MARKER, &args![player]);
            let menu = map_menu(e);
            map_menu_delete_player_marker(e, Ptr::new(menu));
        }
        _ => {}
    }
    let menu = map_menu(e);
    for word in 0..3 {
        let value = e.mem.u32(POINT_011F426C + 4 * word);
        e.mem.set_u32(menu + 0xfc + 4 * word, value);
    }
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0xf8, 0);
    centre_cursor_tile(e);
}

// Translated from 00798a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00798a00 - the C++ exception frame (FS:[0] chain, unwind state 0 around the request's construction) is not translated.
/// Unnamed in the engine map (the callback `00798710` hands on): with an
/// open map menu and a `pFastTravelMarker`, makes a 0x34-byte request
/// (`00456540`), puts the marker at its +0x30, gives it to
/// `PlayerCharacter::RequestPositionPlayer` and clears `pFastTravelMarker`.
pub fn fn_00798a00(e: &mut Engine) {
    if map_menu(e) == 0 || e.mem.u32(map_menu(e) + 0x118) == 0 {
        return;
    }
    let memory = e.call(ALLOCATE, &args![0x34u32]).u32();
    let request = if memory != 0 {
        e.call(POSITION_REQUEST_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    let marker = e.mem.u32(map_menu(e) + 0x118);
    e.mem.set_u32(request + 0x30, marker);
    let player = e.mem.u32(PLAYER_011DEA3C);
    e.call(REQUEST_POSITION_PLAYER, &args![player, request]);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x118, 0);
}

/// `"UIPipBoyHolotapeStop"` (`01074e90`).
const HOLOTAPE_STOP_SOUND: u32 = 0x0107_4e90;
/// `BSSoundHandle::Stop` (engine map), thiscall.
const SOUND_STOP: u32 = 0x00ad_88f0;
/// Thiscall on the sound list (+0x98): empties it.
const SOUND_LIST_CLEAR_0076B7A0: u32 = 0x0076_b7a0;
/// `BSAudio::ExitDialogue` (engine map), thiscall.
const AUDIO_EXIT_DIALOGUE: u32 = 0x00ad_8650;
/// Cdecl, no arguments (`StopNoteSound`'s last call).
const AFTER_NOTE_SOUND_007624C0: u32 = 0x0076_24c0;

/// The part `StopNoteSound` and `ClearAudioLogSubtites` share, on the menu
/// `menu` gives (read again at each step, as the code does): the current
/// sound stopped when it plays, the sound list emptied, the subtitle array
/// released, `pCurrentSound`, the float at +0xC0, `iSoundPlayStartTime` and
/// `bSoundsLoading` cleared.
fn stop_note_sounds(e: &mut Engine, menu: impl Fn(&Engine) -> u32) {
    let node = e.mem.u32(menu(e) + 0xb8);
    if node != 0 {
        let handle = e.call(SELF, &args![node]).u32();
        if e.call(SOUND_IS_PLAYING, &args![handle]).bool() {
            let node = e.mem.u32(menu(e) + 0xb8);
            let handle = e.call(SELF, &args![node]).u32();
            e.call(SOUND_STOP, &args![handle]);
        }
    }
    let sounds = menu(e) + 0x98;
    e.call(SOUND_LIST_CLEAR_0076B7A0, &args![sounds]);
    let subtitles = menu(e) + 0xa8;
    fn_007a1c30(e, Ptr::new(subtitles), 1);
    let base = menu(e);
    e.mem.set_u32(base + 0xb8, 0);
    let base = menu(e);
    e.mem.set_u32(base + 0xc0, 0.0f32.to_bits());
    let base = menu(e);
    e.mem.set_u32(base + 0xc4, 0);
    let base = menu(e);
    e.mem.set_u8(base + 0xbc, 0);
}

// Translated from 00798ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00798ad0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the stop sound's handle) is not translated.
/// `MapMenu::StopNoteSound` (Xbox PDB): stops and clears the note's sounds
/// and subtitles (see [`stop_note_sounds`]), plays
/// `"UIPipBoyHolotapeStop"`, leaves dialogue audio
/// (`BSAudio::ExitDialogue`) and calls `007624c0`.
pub fn map_menu_stop_note_sound(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    stop_note_sounds(e, |_| base);
    play_named_sound(e, HOLOTAPE_STOP_SOUND);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_EXIT_DIALOGUE, &args![audio]);
    e.call(AFTER_NOTE_SOUND_007624C0, &args![]);
}

// Translated from 00798bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::ClearAudioLogSubtites` (Xbox PDB): with an open map menu, its
/// note sounds and subtitles are stopped and cleared (see
/// [`stop_note_sounds`]).
pub fn map_menu_clear_audio_log_subtites(e: &mut Engine) {
    if map_menu(e) == 0 {
        return;
    }
    stop_note_sounds(e, map_menu);
}

/// `"_ItemType"` (`010739a0`).
const ITEM_TYPE_TEXT: u32 = 0x0107_39a0;
/// Thiscall: the word at +0x14 (`00825c00`; on a list box its scroll bar,
/// on the timer the current time).
const WORD_AT_14: u32 = 0x0082_5c00;
/// The knob index `DoEnter` last moved the pip-boy knob to (`011da400`).
const KNOB_INDEX_011DA400: u32 = 0x011d_a400;
/// The two settings whose floats (`00403e20`) `DoEnter` moves the knob by.
const KNOB_STEP_SETTING: u32 = 0x011d_b2f0;
const KNOB_TIME_SETTING: u32 = 0x011d_b358;
/// `FOPipboyManager::MoveKnob` (engine map), thiscall (word, float, float,
/// word), `RET 0x10`.
const PIPBOY_MOVE_KNOB: u32 = 0x007f_8610;
/// The objective list filter callback `DoEnter` passes (`00799100`).
const OBJECTIVE_FILTER_CALLBACK_00799100: u32 = 0x0079_9100;
/// The objective list filter callback `DoEnter` passes to `Filter`
/// (`0079bc50`).
const OBJECTIVE_QUEST_CALLBACK_0079BC50: u32 = 0x0079_bc50;

/// The index (trait `0xfac`, through `_ftol2`) of the tile the list box at
/// `list` returns from its vtable +4, or 0 without one.
fn list_box_current_index(e: &mut Engine, list: u32) -> i32 {
    if e.vcall(list, 4, &args![]).u32() == 0 {
        return 0;
    }
    let tile = e.vcall(list, 4, &args![]).u32();
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    e.call(FTOL2, &args![value]).i32()
}

// Translated from 00798cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoEnter` (Xbox PDB): by id (the compiler's byte table at
/// `007990e0` maps `id + 1`, 0 to 0x1A): -1 sends a gamepad 0xF (vtable
/// +0x10) when `tile`'s owner (`0045cd60`) is the objective list's scroll
/// bar; 14 disables the note list; 15 disables the quest list and enables
/// the objective list; 0x17 shows the quest of `tile` (its objectives
/// filtered with `0079bc50` and `00799100`, the item type 4 on tile 13, the
/// quest target flags on tiles 12 and 11); 0x18 loads the note (or in
/// challenge mode the challenge) of `tile`; 0x19 reads the radio list's
/// index. For 0x17 to 0x19 the pip-boy knob then moves by one step up or
/// down when the index differs from the last one (`011da400`).
pub fn map_menu_do_enter(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut index = 0i32;
    match id.wrapping_add(1) as u32 {
        0 => {
            if tile != 0 {
                let owner = e.call(WORD_AT_28, &args![tile]).u32();
                let scroll_bar = e.call(WORD_AT_14, &args![base + 0x1c0]).u32();
                if owner == scroll_bar {
                    e.vcall(base, 0x10, &args![0xfu32, 0u32]);
                }
            }
        }
        15 => list_box_bgs_note_set_enabled(e, Ptr::new(base + 0x160), 0),
        16 => {
            list_box_tes_quest_set_enabled(e, Ptr::new(base + 0x130), 0);
            list_box_bgs_quest_objective_set_enabled(e, Ptr::new(base + 0x1c0), 1);
        }
        0x18 => {
            if tile != 0 {
                let quest = e
                    .call(LIST_BOX_ITEM_OF_007E1110, &args![base + 0x130, tile])
                    .u32();
                e.set(this, MapMenu::pDisplayedQuest, quest);
                list_box_bgs_quest_objective_filter(
                    e,
                    Ptr::new(base + 0x1c0),
                    OBJECTIVE_QUEST_CALLBACK_0079BC50,
                );
                index = if tile != 0 {
                    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                    e.call(FTOL2, &args![value]).i32()
                } else {
                    0
                };
                let item_type = e.call(TEXT_TO_TRAIT, &args![ITEM_TYPE_TEXT]).u32();
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, item_type, 4u32]);
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 1u32]);
                e.set(this, MapMenu::pDisplayedQuestTarget, 0);
                e.call(
                    LIST_BOX_FILTER_007314C0,
                    &args![
                        base + 0x1c0,
                        OBJECTIVE_FILTER_CALLBACK_00799100,
                        0u32,
                        0x7fff_ffffu32
                    ],
                );
                let has_target = u32::from(e.get(this, MapMenu::pDisplayedQuestTarget) != 0);
                let tile_12 = map_tile(e, base, 12);
                e.call(TILE_SET_INT, &args![tile_12, 0xfafu32, has_target]);
                let quest = e.get(this, MapMenu::pDisplayedQuest);
                let open = quest != 0
                    && !{
                        let quest = e.get(this, MapMenu::pDisplayedQuest);
                        e.call(QUEST_FLAG_0059E400, &args![quest]).bool()
                    };
                let tile_11 = map_tile(e, base, 11);
                e.call(TILE_SET_INT, &args![tile_11, 0xfafu32, u32::from(open)]);
            }
        }
        0x19 => {
            if e.mem.u8(CHALLENGE_MODE_011DA36D) == 0 {
                let note = e
                    .call(LIST_BOX_ITEM_OF_007E1110, &args![base + 0x160, tile])
                    .u32();
                index = list_box_current_index(e, base + 0x160);
                if e.call(FLAG_004B71D0, &args![]).bool()
                    || (note != 0 && e.get(this, MapMenu::pSelectedNote) == 0)
                {
                    map_menu_load_note(e, this, note);
                }
                let kind = e.call(NOTE_TYPE, &args![note]).u32();
                if kind == 0 || e.call(NOTE_TYPE, &args![note]).u32() == 3 {
                    let playing = note == e.get(this, MapMenu::pSelectedNote)
                        && e.get(this, MapMenu::pCurrentSound) != 0;
                    let tile_13 = map_tile(e, base, 13);
                    e.call(TILE_SET_INT, &args![tile_13, 0x1004u32, u32::from(playing)]);
                } else if fn_00798420(e, Ptr::new(note)) == 0 {
                    let now = e.call(TIMER_NOW, &args![TIMER_011F6394]).u32();
                    e.set(this, MapMenu::uiNoteEnterStartTime, now);
                }
            } else {
                let challenge = e
                    .call(LIST_BOX_ITEM_OF_007E1110, &args![base + 0x1f0, tile])
                    .u32();
                if e.call(FLAG_004B71D0, &args![]).bool() || challenge != 0 {
                    map_menu_load_challenge(e, this, challenge);
                }
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 1u32]);
                e.call(PLAY_MENU_SOUND, &args![4u32]);
            }
        }
        0x1a => index = list_box_current_index(e, base + 0x190),
        _ => {}
    }
    let last = e.mem.u32(KNOB_INDEX_011DA400) as i32;
    if last == index || !(0x17..=0x19).contains(&id) {
        return;
    }
    let step = e
        .call(OBJECT_VALUE_ADDRESS_00403E20, &args![KNOB_STEP_SETTING])
        .u32();
    let step = x87_quiet_bits(e.mem.u32(step));
    // `FCHS` flips the sign bit.
    let step = if last < index {
        step
    } else {
        step ^ 0x8000_0000
    };
    let time = e
        .call(OBJECT_VALUE_ADDRESS_00403E20, &args![KNOB_TIME_SETTING])
        .u32();
    let time = x87_quiet_bits(e.mem.u32(time));
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let pipboy = e.call(WORD_AT_174, &args![interface]).u32();
    e.call(PIPBOY_MOVE_KNOB, &args![pipboy, 0u32, step, time, 1u32]);
    e.mem.set_u32(KNOB_INDEX_011DA400, index as u32);
}

/// Thiscall on a quest objective: `AL` (`00799100` skips the objective when
/// it is set).
const OBJECTIVE_FLAG_005A5DC0: u32 = 0x005a_5dc0;
/// Thiscall on a quest objective: its first target node.
const OBJECTIVE_TARGETS_007D6BB0: u32 = 0x007d_6bb0;
/// `BaseProcess::GetActorPackageThatIsRunning` (engine map; folded),
/// thiscall on a quest target: the condition `TESCondition::IsTrue` is
/// asked.
const TARGET_CONDITION_00717E50: u32 = 0x0071_7e50;
/// `TESCondition::IsTrue` (engine map), thiscall (reference, word), `RET 8`.
const CONDITION_IS_TRUE: u32 = 0x0068_0c30;

// Translated from 00799100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; `DoEnter`'s objective list callback):
/// until the map menu has a `pDisplayedQuestTarget`, takes the first target
/// of `objective` (when `005a5dc0` is false) whose condition is true for its
/// reference (`TESQuestTarget::GetReference(0)`, `TESCondition::IsTrue(..,
/// 0)`). `tile` is not read.
pub fn fn_00799100(e: &mut Engine, _unused_0: u32, objective: u32) {
    if e.mem.u32(map_menu(e) + 0xd0) != 0 || objective == 0 {
        return;
    }
    if e.call(OBJECTIVE_FLAG_005A5DC0, &args![objective]).bool() {
        return;
    }
    let mut node = e.call(OBJECTIVE_TARGETS_007D6BB0, &args![objective]).u32();
    while e.mem.u32(map_menu(e) + 0xd0) == 0 && node != 0 && node_item(e, node) != 0 {
        let target = node_item(e, node);
        let reference = e
            .call(QUEST_TARGET_GET_REFERENCE, &args![target, 0u32])
            .u32();
        let condition = e.call(TARGET_CONDITION_00717E50, &args![target]).u32();
        if e.call(CONDITION_IS_TRUE, &args![condition, reference, 0u32])
            .bool()
        {
            let menu = map_menu(e);
            e.mem.set_u32(menu + 0xd0, target);
        }
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
}

/// `"MM_DataText"`, `"MM_TextScrollbar"`, `"_ScrollDelta"` and the challenge
/// text format `"%s\n%d\%d\n\n%s"` (`01074ea8`).
const DATA_TEXT_NAME: u32 = 0x0107_4edc;
const TEXT_SCROLLBAR_NAME: u32 = 0x0107_4ec8;
const SCROLL_DELTA_CAPITAL_TEXT: u32 = 0x0107_4eb8;
const CHALLENGE_TEXT_FORMAT: u32 = 0x0107_4ea8;
/// `LowProcess::GetNumberOfItemsActivated` (engine map; folded), thiscall
/// on the challenge: a value the challenge text shows.
const CHALLENGE_VALUE_006286D0: u32 = 0x0062_86d0;
/// Thiscall on the challenge: a value the challenge text combines with the
/// one above (`004a8f20`, cdecl).
const CHALLENGE_VALUE_005E3FA0: u32 = 0x005e_3fa0;
const COMBINE_004A8F20: u32 = 0x004a_8f20;
/// Cdecl: the font manager (`FontManager::CalculateStringDimensions` is
/// called on it).
const FONT_MANAGER_005BD5B0: u32 = 0x005b_d5b0;
/// `FontManager::CalculateStringDimensions` (engine map), thiscall (result,
/// text, font, width, word), `RET 0x14`: the result's address.
const FONT_CALCULATE_STRING_DIMENSIONS: u32 = 0x00a1_b020;
/// The wrap width `LoadChallenge` measures the text with (`010311a8`, 400.0
/// in the shipped exe).
const CHALLENGE_TEXT_WIDTH_FLOAT: u32 = 0x0103_11a8;
/// Cdecl (float, float): a float in `ST0` (`LoadChallenge` passes 0.0 and
/// the overflow).
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;

// Translated from 007991b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007991b0 - the /GS security cookie check (00ec408c) is not translated.
/// `MapMenu::LoadChallenge` (Xbox PDB): tile 13 shows item type 1 and, in
/// its `"MM_DataText"` child, the challenge's text (`"%s\n%d\%d\n\n%s"`
/// with its name, `004a8f20` of its two values, its `006286d0` value and
/// its description, vtable +0x10 of +0x24 with `'DESC'`), in a 0x200-byte
/// buffer. The `"MM_TextScrollbar"` child gets `"_number_of_items"` =
/// `004e4430(004 04010(0.0, text height - data text's owner's 0xfb0) /
/// "_ScrollDelta") + 1.0`, `"_current_value"` 0.0 and `"_SetInCode"` poked
/// with 1.0.
pub fn map_menu_load_challenge(e: &mut Engine, this: Ptr<MapMenu>, challenge: u32) {
    let base = this.addr();
    let tile_13 = map_tile(e, base, 13);
    let data_text = e
        .call(TILE_GET_CHILD_BY_NAME, &args![tile_13, DATA_TEXT_NAME])
        .u32();
    let tile_13 = map_tile(e, base, 13);
    let scrollbar = e
        .call(TILE_GET_CHILD_BY_NAME, &args![tile_13, TEXT_SCROLLBAR_NAME])
        .u32();
    let delta_id = e
        .call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_CAPITAL_TEXT])
        .u32();
    let delta = x87_store32(e.call(TILE_GET_FLOAT, &args![scrollbar, delta_id]).f64());
    let item_type = e.call(TEXT_TO_TRAIT, &args![ITEM_TYPE_TEXT]).u32();
    let tile_13 = map_tile(e, base, 13);
    e.call(TILE_SET_INT, &args![tile_13, item_type, 1u32]);
    let description = e
        .vcall(challenge + 0x24, 0x10, &args![0u32, 0x4353_4544u32])
        .u32();
    let first = e.call(CHALLENGE_VALUE_006286D0, &args![challenge]).u32();
    let second = e.call(CHALLENGE_VALUE_006286D0, &args![challenge]).u32();
    let third = e.call(CHALLENGE_VALUE_005E3FA0, &args![challenge]).u32();
    let combined = e.call(COMBINE_004A8F20, &args![third, second]).u32();
    let name = e
        .call(MARKER_GET_LOCATION_NAME, &args![challenge + 0x18])
        .u32();
    e.with_stack(0x200, |e, text| {
        e.call(
            BUFFER_FORMAT,
            &args![
                text,
                0x200u32,
                CHALLENGE_TEXT_FORMAT,
                name,
                combined,
                first,
                description
            ],
        );
        e.call(TILE_SET_STRING, &args![data_text, 0xfc4u32, text, 1u32]);
        set_text_scrollbar(e, data_text, scrollbar, delta, text.addr());
    });
}

/// Thiscall on a note: the object whose vtable +0x10 (note, `'TNAM'`) gives
/// its text.
const NOTE_TEXT_OBJECT_005E91D0: u32 = 0x005e_91d0;
/// Thiscall on a note: the object `MapMarkerData::GetLocationName` gives the
/// image path of (`005e9160`).
const NOTE_IMAGE_005E9160: u32 = 0x005e_9160;
/// `"MM_DataImage"`, `"--:--:- %s"`, `"_ItemStringInfo"`,
/// `"_AudioFileCompletion"`.
const DATA_IMAGE_NAME: u32 = 0x0107_4f1c;
const AUDIO_TIME_FORMAT: u32 = 0x0107_4f10;
const ITEM_STRING_INFO_TEXT: u32 = 0x0107_4f00;
const AUDIO_FILE_COMPLETION_TEXT: u32 = 0x0107_4ee8;
/// The object whose string (`00403df0`) the audio note's time line shows.
const AUDIO_LABEL_011D3960: u32 = 0x011d_3960;

/// The text part of `LoadNote` and `LoadChallenge`'s scroll bar set-up:
/// the `"MM_TextScrollbar"` child of tile 13 (`scrollbar`, whose
/// `"_ScrollDelta"` is `delta`) gets `"_number_of_items"` from the measured
/// height of `text` (wrap width `010311a8`) over the data text's owner's
/// height, `"_current_value"` 0.0 and `"_SetInCode"` poked with 1.0.
fn set_text_scrollbar(e: &mut Engine, data_text: u32, scrollbar: u32, delta: u32, text: u32) {
    e.with_stack(0x10, |e, dimensions| {
        let width = float_constant(e, CHALLENGE_TEXT_WIDTH_FLOAT);
        let font = e.call(FONT_MANAGER_005BD5B0, &args![]).u32();
        let result = e
            .call(
                FONT_CALCULATE_STRING_DIMENSIONS,
                &args![font, dimensions, text, 1u32, width, 0u32],
            )
            .u32();
        let height = x87_quiet_bits(e.mem.u32(result + 4));
        let owner = e.call(WORD_AT_28, &args![data_text]).u32();
        let visible = e.call(TILE_GET_FLOAT, &args![owner, 0xfb0u32]).f64();
        let overflow = x87_quiet_bits(x87_store32(x87_sub(x87_m32(height), visible)));
        let overflow = e
            .call(FLOAT_PAIR_00404010, &args![0.0f32.to_bits(), overflow])
            .f64();
        let overflow = x87_store32(overflow);
        let steps = x87_quiet_bits(x87_store32(x87_div(x87_load32(overflow), x87_m32(delta))));
        let rounded = e.call(ROUND_FLOAT, &args![steps]).f64();
        let one: f64 = e.global(ONE_DOUBLE);
        let items = x87_quiet_bits(x87_store32(x87_add(rounded, one)));
        let number_of_items = e.call(TEXT_TO_TRAIT, &args![NUMBER_OF_ITEMS_TEXT]).u32();
        e.call(
            TILE_SET_FLOAT,
            &args![scrollbar, number_of_items, items, 1u32],
        );
    });
    let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    e.call(
        TILE_SET_FLOAT,
        &args![scrollbar, current_value, 0.0f32.to_bits(), 0u32],
    );
    let set_in_code = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    e.call(TILE_POKE, &args![scrollbar, set_in_code, 1.0f32.to_bits()]);
}

// Translated from 007993d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007993d0 - the /GS security cookie check (00ec408c) is not translated.
/// `MapMenu::LoadNote` (Xbox PDB): by the note's type (`005e8d40`; the
/// compiler's jump table at `00799680`): 1 (text) shows item type 1 and the
/// note's text (vtable +0x10 of `005e91d0` with the note and `'TNAM'`) in
/// tile 13's `"MM_DataText"`, with the text scroll bar set up (see
/// [`set_text_scrollbar`]); 2 (image) puts the image path into
/// `"MM_DataImage"`'s trait `0xfcc` and shows item type 2; 0 and 3 (audio)
/// show item type 3 and, without a playing sound, the time line
/// `"--:--:- %s"` (a 0x40-byte buffer) as `"_ItemStringInfo"` and 0.0 as
/// `"_AudioFileCompletion"`. Tile 13's trait `0xfa3` is then set to 1.
pub fn map_menu_load_note(e: &mut Engine, this: Ptr<MapMenu>, note: u32) {
    let base = this.addr();
    match e.call(NOTE_TYPE, &args![note]).u32() {
        1 => {
            let tile_13 = map_tile(e, base, 13);
            let data_text = e
                .call(TILE_GET_CHILD_BY_NAME, &args![tile_13, DATA_TEXT_NAME])
                .u32();
            let tile_13 = map_tile(e, base, 13);
            let scrollbar = e
                .call(TILE_GET_CHILD_BY_NAME, &args![tile_13, TEXT_SCROLLBAR_NAME])
                .u32();
            let delta_id = e
                .call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_CAPITAL_TEXT])
                .u32();
            let delta = x87_store32(e.call(TILE_GET_FLOAT, &args![scrollbar, delta_id]).f64());
            let item_type = e.call(TEXT_TO_TRAIT, &args![ITEM_TYPE_TEXT]).u32();
            let tile_13 = map_tile(e, base, 13);
            e.call(TILE_SET_INT, &args![tile_13, item_type, 1u32]);
            let object = e.call(NOTE_TEXT_OBJECT_005E91D0, &args![note]).u32();
            let text = e.vcall(object, 0x10, &args![note, 0x4d41_4e54u32]).u32();
            e.call(TILE_SET_STRING, &args![data_text, 0xfc4u32, text, 1u32]);
            let object = e.call(NOTE_TEXT_OBJECT_005E91D0, &args![note]).u32();
            let text = e.vcall(object, 0x10, &args![note, 0x4d41_4e54u32]).u32();
            set_text_scrollbar(e, data_text, scrollbar, delta, text);
        }
        2 => {
            let image = e.call(NOTE_IMAGE_005E9160, &args![note]).u32();
            let path = e.call(MARKER_GET_LOCATION_NAME, &args![image]).u32();
            let tile_13 = map_tile(e, base, 13);
            let data_image = e
                .call(TILE_GET_CHILD_BY_NAME, &args![tile_13, DATA_IMAGE_NAME])
                .u32();
            e.call(TILE_SET_STRING, &args![data_image, 0xfccu32, path, 1u32]);
            let item_type = e.call(TEXT_TO_TRAIT, &args![ITEM_TYPE_TEXT]).u32();
            let tile_13 = map_tile(e, base, 13);
            e.call(TILE_SET_INT, &args![tile_13, item_type, 2u32]);
        }
        0 | 3 => {
            let item_type = e.call(TEXT_TO_TRAIT, &args![ITEM_TYPE_TEXT]).u32();
            let tile_13 = map_tile(e, base, 13);
            e.call(TILE_SET_INT, &args![tile_13, item_type, 3u32]);
            if e.get(this, MapMenu::pCurrentSound) == 0 {
                let label = e
                    .call(OBJECT_FIELD_00403DF0, &args![AUDIO_LABEL_011D3960])
                    .u32();
                e.with_stack(0x40, |e, text| {
                    e.call(
                        BUFFER_FORMAT,
                        &args![text, 0x40u32, AUDIO_TIME_FORMAT, label],
                    );
                    let info = e.call(TEXT_TO_TRAIT, &args![ITEM_STRING_INFO_TEXT]).u32();
                    let tile_13 = map_tile(e, base, 13);
                    e.call(TILE_SET_STRING, &args![tile_13, info, text, 1u32]);
                });
                let completion = e
                    .call(TEXT_TO_TRAIT, &args![AUDIO_FILE_COMPLETION_TEXT])
                    .u32();
                let tile_13 = map_tile(e, base, 13);
                e.call(
                    TILE_SET_FLOAT,
                    &args![tile_13, completion, 0.0f32.to_bits(), 1u32],
                );
            }
        }
        _ => {}
    }
    let tile_13 = map_tile(e, base, 13);
    e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 1u32]);
}

// Translated from 00799690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoLeave` (Xbox PDB): by id (the compiler's byte table at
/// `00799770` maps `id + 1`, 0 to 0x19): -1 sends vtable +0x14 (0xF, 0) when
/// `tile`'s owner (`0045cd60`) is the objective list's scroll bar; 14
/// enables the note list; 15 enables the quest list and disables the
/// objective list; 0x18 clears `uiNoteEnterStartTime` when `004b71d0` or no
/// note is selected.
pub fn map_menu_do_leave(e: &mut Engine, this: Ptr<MapMenu>, id: i32, tile: u32) {
    let base = this.addr();
    match id.wrapping_add(1) as u32 {
        0 => {
            if tile != 0 {
                let owner = e.call(WORD_AT_28, &args![tile]).u32();
                let scroll_bar = e.call(WORD_AT_14, &args![base + 0x1c0]).u32();
                if owner == scroll_bar {
                    e.vcall(base, 0x14, &args![0xfu32, 0u32]);
                }
            }
        }
        15 => list_box_bgs_note_set_enabled(e, Ptr::new(base + 0x160), 1),
        16 => {
            list_box_tes_quest_set_enabled(e, Ptr::new(base + 0x130), 1);
            list_box_bgs_quest_objective_set_enabled(e, Ptr::new(base + 0x1c0), 0);
        }
        0x19 => {
            if e.call(FLAG_004B71D0, &args![]).bool() || e.get(this, MapMenu::pSelectedNote) == 0 {
                e.set(this, MapMenu::uiNoteEnterStartTime, 0);
            }
        }
        _ => {}
    }
}

/// Thiscall on the input object (`00877720`'s result) (word, axis), `RET 8`:
/// a stick axis as an integer.
const INPUT_AXIS_00A23390: u32 = 0x00a2_3390;
/// Cdecl (float): a float in `ST0` (`DoGamepad` compares it with the dead
/// zones and scales the zoom by it).
const MAGNITUDE_00408820: u32 = 0x0040_8820;
/// The `double`s `DoGamepad` uses: the zoom stick's dead zone (`010718f0`),
/// the axis range (`01029790`), the zoom factor (`01074f38`), the pan
/// sticks' dead zone (`010745f8`) and the pan speeds (`01074f30`,
/// `0102fc70`).
const ZOOM_DEAD_ZONE_DOUBLE: u32 = 0x0107_18f0;
const AXIS_RANGE_DOUBLE: u32 = 0x0102_9790;
const ZOOM_FACTOR_DOUBLE: u32 = 0x0107_4f38;
const PAN_DEAD_ZONE_DOUBLE: u32 = 0x0107_45f8;
const PAN_SPEED_Y_DOUBLE: u32 = 0x0107_4f30;
const PAN_SPEED_X_DOUBLE: u32 = 0x0102_fc70;
/// The `float` the zoom buttons zoom by (`01018204`).
const ZOOM_STEP_FLOAT: u32 = 0x0101_8204;
/// The time `DoGamepad` last scrolled a text with the stick (`011da404`),
/// and the setting whose float (`00403e20`) is the repeat delay.
const SCROLL_TIME_011DA404: u32 = 0x011d_a404;
const SCROLL_DELAY_SETTING: u32 = 0x011d_8b88;
/// `InterfaceManager::PlayMenuSound` (engine map), cdecl (sound).
const INTERFACE_PLAY_MENU_SOUND: u32 = 0x0071_7280;
/// `Interface::SetInventoryMenuVisible` (cdecl, three words) and
/// `Interface::SetStatsMenuVisible` (cdecl, two words) (engine map).
const SET_INVENTORY_MENU_VISIBLE: u32 = 0x0070_48f0;
const SET_STATS_MENU_VISIBLE: u32 = 0x0070_4c10;
/// Thiscall on an embedded list (tile 4's +4): its first node (`00559450`),
/// and the step (address of the node), `RET 4` (`0057cbe0`): the address
/// of the item.
const LIST_FIRST_00559450: u32 = 0x0055_9450;
const LIST_STEP_0057CBE0: u32 = 0x0057_cbe0;

/// `FILD`, `FSTP float [ESP]`, `00408820`: whether the result is above
/// `limit` or unordered (`FCOMP; TEST AH,0x41; JP` taken).
fn axis_beyond(e: &mut Engine, axis: i32, limit: u32) -> bool {
    let bits = x87_store32(f64::from(axis));
    let value = e.call(MAGNITUDE_00408820, &args![bits]).f64();
    let limit: f64 = e.global(limit);
    !matches!(
        value.partial_cmp(&limit),
        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
    )
}

/// An axis value read from the input object (`00877720` on `011dea0c`,
/// then `00a23390(0, axis)`).
fn input_axis(e: &mut Engine, axis: u32) -> i32 {
    let owner = e.mem.u32(OBJECT_011DEA0C);
    let input = e.call(OBJECT_00877720, &args![owner]).u32();
    e.call(INPUT_AXIS_00A23390, &args![input, 0u32, axis]).i32()
}

/// `DoGamepad`'s stick scrolling on pages 0x22 and 0x23: with the scroll
/// bar enabled (trait `0xfa3` not 0.0) and axis 10 moved, once the repeat
/// delay has passed since the last step (`011da404`, 0 for none), the
/// scroll bar's `"_current_value"` moves by one (down for a negative axis),
/// `"_SetInCode"` is poked when the value changed, and the time is kept.
fn gamepad_scroll(e: &mut Engine, scroll_bar: u32, axis_x: i32) -> bool {
    let enabled = e.call(TILE_GET_FLOAT, &args![scroll_bar, 0xfa3u32]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    if equal(enabled, zero) {
        return false;
    }
    let step = match axis_x {
        0 => 0,
        x if x < 0 => 1,
        _ => -1,
    };
    if step == 0 {
        return false;
    }
    let last = e.mem.u32(SCROLL_TIME_011DA404);
    if last != 0 {
        let now = e.call(WORD_AT_14, &args![TIMER_011F6394]).u32();
        let elapsed = f64::from(now.wrapping_sub(last));
        let delay = e
            .call(OBJECT_VALUE_ADDRESS_00403E20, &args![SCROLL_DELAY_SETTING])
            .u32();
        let delay = x87_load32(e.mem.u32(delay));
        if !below(delay, elapsed) {
            return false;
        }
    }
    let current_value = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    let value = e
        .call(TILE_GET_FLOAT, &args![scroll_bar, current_value])
        .f64();
    let current = e.call(FTOL2, &args![value]).i32();
    let next = x87_store32(f64::from(current.wrapping_add(step)));
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_bar, current_value, next, 0u32],
    );
    let value = e
        .call(TILE_GET_FLOAT, &args![scroll_bar, current_value])
        .f64();
    if !equal(f64::from(current), value) {
        let set_in_code = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
        e.call(TILE_POKE, &args![scroll_bar, set_in_code, 1.0f32.to_bits()]);
    }
    let now = e.call(WORD_AT_14, &args![TIMER_011F6394]).u32();
    e.mem.set_u32(SCROLL_TIME_011DA404, now);
    true
}

/// `DoGamepad`'s stick part (with `00709d40`): see [`map_menu_do_gamepad`].
fn gamepad_sticks(e: &mut Engine, this: Ptr<MapMenu>) -> bool {
    let base = this.addr();
    let mut handled = false;
    // The code keeps the input object and 0x7FFF in locals it never reads.
    let owner = e.mem.u32(OBJECT_011DEA0C);
    e.call(OBJECT_00877720, &args![owner]);
    if e.get(this, MapMenu::iCurrentPage) == 0x20 || e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let mut zoom = input_axis(e, 8);
        if !axis_beyond(e, zoom, ZOOM_DEAD_ZONE_DOUBLE) {
            zoom = 0;
        }
        if zoom != 0 {
            let range: f64 = e.global(AXIS_RANGE_DOUBLE);
            let scaled = x87_quiet_bits(x87_store32(x87_div(f64::from(zoom), range)));
            let amount = x87_store32(e.call(MAGNITUDE_00408820, &args![scaled]).f64());
            let factor: f64 = e.global(ZOOM_FACTOR_DOUBLE);
            let amount = x87_add(x87_mul(x87_load32(amount), factor), 1.0);
            let amount = x87_quiet_bits(x87_store32(amount));
            map_menu_zoom_map(e, this, u8::from(zoom > 0), f32::from_bits(amount));
            handled = true;
        }
    }
    let mut axis_y = input_axis(e, 9);
    let mut axis_x = input_axis(e, 10);
    if !axis_beyond(e, axis_y, PAN_DEAD_ZONE_DOUBLE) {
        axis_y = 0;
    }
    if !axis_beyond(e, axis_x, PAN_DEAD_ZONE_DOUBLE) {
        axis_x = 0;
    }
    match e.get(this, MapMenu::iCurrentPage).wrapping_sub(0x20) as u32 {
        0 | 1 => {
            if e.get(this, MapMenu::pMapTile) != 0 {
                if axis_y != 0 || axis_x != 0 {
                    for id in [0xfeau32, 0xfeb] {
                        let tile = e.get(this, MapMenu::pMapTile);
                        e.call(TILE_SET_FLOAT, &args![tile, id, 1.0f32.to_bits(), 1u32]);
                    }
                    let range: f64 = e.global(AXIS_RANGE_DOUBLE);
                    let speed: f64 = e.global(PAN_SPEED_Y_DOUBLE);
                    let pan_y = x87_div(x87_mul(f64::from(axis_y), speed), range);
                    let pan_y = x87_quiet_bits(x87_store32(pan_y));
                    let tile = e.get(this, MapMenu::pMapTile);
                    e.call(TILE_POKE, &args![tile, 0xfeeu32, pan_y]);
                    let speed: f64 = e.global(PAN_SPEED_X_DOUBLE);
                    let pan_x = x87_div(x87_mul(f64::from(axis_x), speed), range);
                    let pan_x = x87_quiet_bits(x87_store32(pan_x));
                    let tile = e.get(this, MapMenu::pMapTile);
                    e.call(TILE_POKE, &args![tile, 0xfefu32, pan_x]);
                    handled = true;
                } else {
                    for id in [0xfeau32, 0xfeb] {
                        let still = float_constant(e, MINUS_ONE_FLOAT);
                        let tile = e.get(this, MapMenu::pMapTile);
                        e.call(TILE_SET_FLOAT, &args![tile, id, still, 1u32]);
                    }
                }
                fn_00799dc0(e, this);
            }
        }
        2 | 3 => {
            let scroll_bar = if e.get(this, MapMenu::iCurrentPage) == 0x22 {
                e.call(WORD_AT_14, &args![base + 0x1c0]).u32()
            } else {
                let tile_13 = map_tile(e, base, 13);
                e.call(TILE_GET_CHILD_BY_NAME, &args![tile_13, TEXT_SCROLLBAR_NAME])
                    .u32()
            };
            if scroll_bar != 0 && gamepad_scroll(e, scroll_bar, axis_x) {
                handled = true;
            }
        }
        _ => {}
    }
    handled
}

// Translated from 00799790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoGamepad` (Xbox PDB): false unless the map menu is on top and
/// the pip-boy is up (`00974d90`). With `00709d40`, the sticks: on the maps
/// axis 8 beyond its dead zone zooms (`ZoomMap`); axes 9 and 10 beyond
/// theirs pan the map tile (`0xfea`/`0xfeb` 1.0 and `0xfee`/`0xfef` poked
/// with the scaled axes; -1.0 when both are still) and the position is
/// shown; on pages 0x22 and 0x23 axis 10 scrolls the objective list's (or
/// the text's) scroll bar (see [`gamepad_scroll`]). Then `button` (the
/// compiler's byte table at `00799da4` maps `button - 3`, 0 to 0xD): 3 and
/// 4 step the page (wrapping 0x20..=0x24, vtable +0xC); 9 on the local map
/// clicks the selected marker (id 0x1A); 0xD and 0xE open the inventory or
/// the stats; 0xF and 0x10 zoom on the maps. Returns whether anything was
/// handled. `_unused_1` is not read.
pub fn map_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    button: i32,
    _unused_1: u32,
) -> bool {
    let base = this.addr();
    if !e.call(IS_TOP_MENU_ID, &args![0x3ffu32]).bool() {
        return false;
    }
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if !e.call(PIPBOY_FLAG_00974D90, &args![pipboy]).bool() {
        return false;
    }
    let mut handled = false;
    if !e.call(IS_TOP_MENU_ID, &args![0x3ffu32]).bool() {
        return handled;
    }
    if e.call(FLAG_00709D40, &args![]).bool() && gamepad_sticks(e, this) {
        handled = true;
    }
    match button.wrapping_sub(3) as u32 {
        0 | 1 => {
            let step = if button != 4 { 1 } else { -1 };
            let page = e.get(this, MapMenu::iCurrentPage).wrapping_add(step);
            e.set(this, MapMenu::iCurrentPage, page);
            let page = if e.get(this, MapMenu::iCurrentPage) > 0x24 {
                0x20
            } else {
                e.get(this, MapMenu::iCurrentPage)
            };
            e.set(this, MapMenu::iCurrentPage, page);
            let page = if e.get(this, MapMenu::iCurrentPage) < 0x20 {
                0x24
            } else {
                e.get(this, MapMenu::iCurrentPage)
            };
            e.set(this, MapMenu::iCurrentPage, page);
            let page = e.get(this, MapMenu::iCurrentPage);
            e.vcall(base, 0xc, &args![page, 0u32]);
            handled = true;
        }
        6 => {
            if e.get(this, MapMenu::iCurrentPage) == 0x21 {
                e.with_stack(4, |e, node| {
                    let tile_4 = map_tile(e, base, 4);
                    let first = e.call(LIST_FIRST_00559450, &args![tile_4 + 4]).u32();
                    e.mem.set_u32(node.addr(), first);
                    while e.mem.u32(node.addr()) != 0 {
                        let tile_4 = map_tile(e, base, 4);
                        let item = e.call(LIST_STEP_0057CBE0, &args![tile_4 + 4, node]).u32();
                        let marker = e.mem.u32(item);
                        let selected = e.mem.u32(TRAIT_SELECTED);
                        if e.call(TILE_IS_TRUE, &args![marker, selected]).bool() {
                            e.vcall(base, 0xc, &args![0x1au32, marker]);
                            handled = true;
                            e.mem.set_u32(node.addr(), 0);
                        }
                    }
                });
            }
        }
        10 => {
            e.call(INTERFACE_PLAY_MENU_SOUND, &args![0x24u32]);
            e.call(SET_INVENTORY_MENU_VISIBLE, &args![1u32, 0u32, 1u32]);
            handled = true;
        }
        11 => {
            e.call(INTERFACE_PLAY_MENU_SOUND, &args![0x24u32]);
            e.call(SET_STATS_MENU_VISIBLE, &args![1u32, 0u32]);
            handled = true;
        }
        12 | 13 => {
            if e.get(this, MapMenu::iCurrentPage) == 0x20
                || e.get(this, MapMenu::iCurrentPage) == 0x21
            {
                let step = float_constant(e, ZOOM_STEP_FLOAT);
                map_menu_zoom_map(e, this, u8::from(button == 0x10), f32::from_bits(step));
                handled = true;
            }
        }
        _ => {}
    }
    handled
}

/// Thiscall on an `NiPoint2` (x, y), `RET 8`: sets it; (factor), `RET 4`:
/// scales it; (result, other), `RET 8`: the difference into `result`; and
/// its length in `ST0` (`00589850`).
const POINT2_SET_00452DC0: u32 = 0x0045_2dc0;
const POINT2_SCALE_006AD180: u32 = 0x006a_d180;
const POINT2_SUBTRACT_004E8880: u32 = 0x004e_8880;
const POINT2_LENGTH_00589850: u32 = 0x0058_9850;
/// Thiscall on a tile: a string the marker test compares (`00700300`).
const TILE_NAME_00700300: u32 = 0x0070_0300;
/// Cdecl (text, text): 0 when equal (`00ec7750`).
const TEXT_COMPARE_00EC7750: u32 = 0x00ec_7750;
/// `" Marker"` (`01074f54`) and `"UIPipBoyHighlight"` (`01074f40`).
const MARKER_TEXT: u32 = 0x0107_4f54;
const HIGHLIGHT_SOUND: u32 = 0x0107_4f40;
/// The `double` a map marker tile's trait `0xfaa` is compared with
/// (`01074f60`, 26.0 in the shipped exe).
const MARKER_TILE_KIND_DOUBLE: u32 = 0x0107_4f60;

// Translated from 00799dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00799dc0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the highlight sound's handle) is not translated.
/// Unnamed in the engine map (`MapMenu`, the cursor over the map): with a
/// map tile, takes the centre of tile 6 (the cursor) relative to the map
/// tile and looks among the map tile's children for marker tiles (trait
/// `0xfaa` equal to `01074f60`, text `" Marker"`) the nearest one closer
/// than half of tile 6's height (the marker position from the static trait
/// ids `011da350`/`011da34c`, scaled by the map tile's width), clearing
/// each one's selected trait. The nearest gets it set; when it is new,
/// `"UIPipBoyHighlight"` plays, its `"_LocationName"` becomes tile 6's
/// title unless it is `"Companion"`, it becomes `pHighlightedMapMarker` and
/// tile 11's trait `0xfaf` follows its trait `0x1004`. Without one the title
/// is emptied, `pHighlightedMapMarker` cleared and tile 11's `0xfaf` set to
/// 0.
pub fn fn_00799dc0(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    if e.get(this, MapMenu::pMapTile) == 0 {
        return;
    }
    let two: f64 = e.global(TWO_DOUBLE);
    let tile_6 = map_tile(e, base, 6);
    let top = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfa2u32]).f64();
    let tile_6 = map_tile(e, base, 6);
    let height = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfb0u32]).f64();
    let centre_y = x87_quiet_bits(x87_store32(x87_add(x87_div(height, two), top)));
    let tile_6 = map_tile(e, base, 6);
    let left = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfa1u32]).f64();
    let tile_6 = map_tile(e, base, 6);
    let width = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfb1u32]).f64();
    let centre_x = x87_quiet_bits(x87_store32(x87_add(x87_div(width, two), left)));
    let found = e.with_stack(0x1c, |e, frame| {
        // The frame's locals: the cursor point (+0), the node (+8), the
        // marker point (+0xC) and the difference (+0x14).
        let cursor = frame.addr();
        let node = cursor + 8;
        let marker_point = cursor + 0xc;
        let difference = cursor + 0x14;
        e.call(POINT2_SET_00452DC0, &args![cursor, centre_x, centre_y]);
        let map = e.get(this, MapMenu::pMapTile);
        let first = e.call(LIST_FIRST_00559450, &args![map + 4]).u32();
        e.mem.set_u32(node, first);
        let mut best = x87_quiet_bits(e.mem.u32(LARGEST_VALUE_01016970));
        let mut found = 0u32;
        let map = e.get(this, MapMenu::pMapTile);
        let map_x = e.call(TILE_GET_FLOAT, &args![map, 0xfa1u32]).f64();
        let x = x87_store32(x87_sub(x87_m32(e.mem.u32(cursor)), map_x));
        e.mem.set_u32(cursor, x);
        let map = e.get(this, MapMenu::pMapTile);
        let map_y = e.call(TILE_GET_FLOAT, &args![map, 0xfa2u32]).f64();
        let y = x87_store32(x87_sub(x87_m32(e.mem.u32(cursor + 4)), map_y));
        e.mem.set_u32(cursor + 4, y);
        while e.mem.u32(node) != 0 {
            let map = e.get(this, MapMenu::pMapTile);
            let item = e.call(LIST_STEP_0057CBE0, &args![map + 4, node]).u32();
            let tile = e.mem.u32(item);
            let kind = e.call(TILE_GET_FLOAT, &args![tile, 0xfaau32]).f64();
            let marker_kind: f64 = e.global(MARKER_TILE_KIND_DOUBLE);
            if !equal(kind, marker_kind) {
                continue;
            }
            let text = e.call(TILE_NAME_00700300, &args![tile]).u32();
            if e.call(TEXT_COMPARE_00EC7750, &args![text, MARKER_TEXT])
                .u32()
                != 0
            {
                continue;
            }
            let y_id = e.mem.u32(TRAIT_Y);
            let marker_y = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, y_id]).f64());
            let x_id = e.mem.u32(TRAIT_X);
            let marker_x = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, x_id]).f64());
            e.call(
                POINT2_SET_00452DC0,
                &args![marker_point, marker_x, marker_y],
            );
            let map = e.get(this, MapMenu::pMapTile);
            let scale = x87_store32(e.call(TILE_GET_FLOAT, &args![map, 0xfb1u32]).f64());
            e.call(POINT2_SCALE_006AD180, &args![marker_point, scale]);
            let result = e
                .call(
                    POINT2_SUBTRACT_004E8880,
                    &args![cursor, difference, marker_point],
                )
                .u32();
            let distance = x87_store32(e.call(POINT2_LENGTH_00589850, &args![result]).f64());
            if above(x87_load32(best), x87_load32(distance)) {
                let tile_6 = map_tile(e, base, 6);
                let height = e.call(TILE_GET_FLOAT, &args![tile_6, 0xfb0u32]).f64();
                let half = x87_div(height, two);
                if below(x87_load32(distance), half) {
                    best = x87_quiet_bits(distance);
                    found = tile;
                }
            }
            let selected = e.mem.u32(TRAIT_SELECTED);
            e.call(TILE_SET_INT, &args![tile, selected, 0u32]);
        }
        found
    });
    if found == 0 {
        let title = e
            .call(ADD_USER_TRAIT, &args![TITLE_TEXT, 0xffff_ffffu32])
            .u32();
        let tile_6 = map_tile(e, base, 6);
        e.call(TILE_SET_STRING, &args![tile_6, title, EMPTY_TEXT, 1u32]);
        e.set(this, MapMenu::pHighlightedMapMarker, 0);
        let tile_11 = map_tile(e, base, 11);
        e.call(TILE_SET_INT, &args![tile_11, 0xfafu32, 0u32]);
        return;
    }
    let selected = e.mem.u32(TRAIT_SELECTED);
    e.call(TILE_SET_INT, &args![found, selected, 1u32]);
    if found == e.get(this, MapMenu::pHighlightedMapMarker) {
        return;
    }
    play_named_sound(e, HIGHLIGHT_SOUND);
    let name_id = e
        .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
        .u32();
    let name = e.call(TILE_GET_STRING, &args![found, name_id]).u32();
    if e.call(STRICMP, &args![name, COMPANION_TEXT]).u32() != 0 {
        let name_id = e
            .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
            .u32();
        let name = e.call(TILE_GET_STRING, &args![found, name_id]).u32();
        let title = e
            .call(ADD_USER_TRAIT, &args![TITLE_TEXT, 0xffff_ffffu32])
            .u32();
        let tile_6 = map_tile(e, base, 6);
        e.call(TILE_SET_STRING, &args![tile_6, title, name, 1u32]);
    }
    e.set(this, MapMenu::pHighlightedMapMarker, found);
    let companion = u32::from(e.call(TILE_IS_TRUE, &args![found, 0x1004u32]).bool());
    let tile_11 = map_tile(e, base, 11);
    e.call(TILE_SET_INT, &args![tile_11, 0xfafu32, companion]);
}

/// `BSSoundHandle::IsValid` and `BSSoundHandle::GetDuration` (engine map),
/// thiscall.
const SOUND_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_GET_DURATION: u32 = 0x00ad_8b30;
/// `BSAudio::SetRadioVolume` (engine map), thiscall (float), `RET 4`.
const AUDIO_SET_RADIO_VOLUME: u32 = 0x00ad_7c00;
/// Thiscall on the subtitle array (index), `RET 4`: the element's address.
const ARRAY_ELEMENT_006950D0: u32 = 0x0069_50d0;
/// `Interface::ShowSubtitle` (engine map), cdecl (text, sound handle by
/// value, position by value, speaker, word).
const SHOW_SUBTITLE: u32 = 0x0070_5210;
/// The time a note's next sound started waiting (`011da410`).
const SOUND_WAIT_011DA410: u32 = 0x011d_a410;
/// `"%02d:%02d:%01d %s"` and `"00:00:0 %s"`.
const PLAY_TIME_FORMAT: u32 = 0x0107_4f84;
const PLAY_DONE_FORMAT: u32 = 0x0107_4f78;
/// Thiscall on a `BSSimpleArray`: whether it should shrink (`006f3170`),
/// and the reserve it shrinks to (`00869600`).
const ARRAY_SHOULD_SHRINK_006F3170: u32 = 0x006f_3170;
const ARRAY_SHRUNK_SIZE_00869600: u32 = 0x0086_9600;
/// Thiscall on a `BSSimpleArray` (to, from, count), `RET 0xC`: moves
/// elements.
const ARRAY_MOVE_00692480: u32 = 0x0069_2480;

/// The subtitle of the note's current sound: `Interface::ShowSubtitle`
/// with the first subtitle's text, a copy of the sound handle (`00418900`
/// into a stack slot, passed by value), the player's position (its vtable
/// +0x1F4, passed by value), the player and `flag`; then the first
/// subtitle removed (`007a1b10(0, 1)`).
fn show_note_subtitle(e: &mut Engine, this: Ptr<MapMenu>, flag: u32) {
    let base = this.addr();
    let player = e.mem.u32(PLAYER_011DEA3C);
    let position = e.vcall(player, 0x1f4, &args![]).u32();
    let x = e.mem.u32(position);
    let y = e.mem.u32(position + 4);
    let z = e.mem.u32(position + 8);
    e.with_stack(0xc, |e, copy| {
        let node = e.get(this, MapMenu::pCurrentSound);
        let handle = e.call(SELF, &args![node]).u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![copy, handle]);
        let element = e
            .call(ARRAY_ELEMENT_006950D0, &args![base + 0xa8, 0u32])
            .u32();
        let text = e.call(LIST_FIRST_00559450, &args![element]).u32();
        let h0 = e.mem.u32(copy.addr());
        let h1 = e.mem.u32(copy.addr() + 4);
        let h2 = e.mem.u32(copy.addr() + 8);
        e.call(
            SHOW_SUBTITLE,
            &args![text, h0, h1, h2, x, y, z, player, flag],
        );
    });
    fn_007a1b10(e, Ptr::new(base + 0xa8), 0, 1);
}

/// The play time text of an audio note into tile 13's `"_ItemStringInfo"`
/// (a 0x40-byte buffer, the format's last word the `011d3960` label).
fn set_play_time_text(e: &mut Engine, this: Ptr<MapMenu>, words: &[u32]) {
    let base = this.addr();
    e.with_stack(0x40, |e, text| {
        let mut call = vec![text.addr(), 0x40];
        call.extend_from_slice(words);
        e.call(BUFFER_FORMAT, &call);
        let info = e.call(TEXT_TO_TRAIT, &args![ITEM_STRING_INFO_TEXT]).u32();
        let tile_13 = map_tile(e, base, 13);
        e.call(TILE_SET_STRING, &args![tile_13, info, text, 1u32]);
    });
}

// Translated from 0079a660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079a660 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the sound handles passed by value) and the /GS security cookie check (00ec408c) are not translated.
/// `MapMenu::UpdatePlayingSounds` (Xbox PDB): while the note's sounds load
/// (`bSoundsLoading`), walks the sound list: no valid sound stops the note
/// and restores the radio volume; all with a duration end the loading, set
/// the total length (+0xC0, the durations plus 500 per gap), play the first
/// one, keep the start time and show its subtitle (flag 0). Otherwise, with
/// a length, shows the remaining play time (`"%02d:%02d:%01d %s"`) and the
/// completion. Then, when the current sound has ended: after more than 500
/// ms (from `011da410`) the next sound plays with its subtitle (flag 1); at
/// the last one the time reads `"00:00:0 %s"`, the completion 1.0, tile
/// 13's trait `0x1004` 0, and the note stops.
pub fn map_menu_update_playing_sounds(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();

    if e.get(this, MapMenu::bSoundsLoading) != 0 {
        let mut all_ready = true;
        let mut none_valid = true;
        let mut total = 0u32;
        let mut count = 0u32;
        let mut node = base + 0x98;
        while node != 0 {
            let handle = e.call(SELF, &args![node]).u32();
            if e.call(SOUND_IS_VALID, &args![handle]).bool() {
                none_valid = false;
                if e.get(this, MapMenu::pCurrentSound) == 0 {
                    e.set(this, MapMenu::pCurrentSound, node);
                }
                let handle = e.call(SELF, &args![node]).u32();
                if e.call(SOUND_GET_DURATION, &args![handle]).u32() == 0 {
                    all_ready = false;
                } else {
                    let handle = e.call(SELF, &args![node]).u32();
                    let duration = e.call(SOUND_GET_DURATION, &args![handle]).u32();
                    total = duration.wrapping_add(total);
                    count = count.wrapping_add(1);
                }
            }
            node = e.call(WORD_AT_C, &args![node]).u32();
        }
        if none_valid {
            map_menu_stop_note_sound(e, this);
            let volume = x87_quiet_bits(e.mem.u32(base + 0xc8));
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            e.call(AUDIO_SET_RADIO_VOLUME, &args![audio, volume]);
        } else if all_ready {
            e.set(this, MapMenu::bSoundsLoading, 0);
            let length = count
                .wrapping_sub(1)
                .wrapping_mul(0x1f4)
                .wrapping_add(total);
            e.mem
                .set_u32(base + 0xc0, x87_store32(f64::from(length as i32)));
            let node = e.get(this, MapMenu::pCurrentSound);
            let handle = e.call(SELF, &args![node]).u32();
            e.call(SOUND_PLAY, &args![handle, 0u32]);
            let now = e.call(WORD_AT_14, &args![TIMER_011F6394]).u32();
            e.set(this, MapMenu::iSoundPlayStartTime, now);
            show_note_subtitle(e, this, 0);
        }
    } else {
        let zero: f64 = e.global(ZERO_DOUBLE);
        if above(x87_load32(e.mem.u32(base + 0xc0)), zero) {
            let now = e.call(WORD_AT_14, &args![TIMER_011F6394]).u32();
            let start = e.get(this, MapMenu::iSoundPlayStartTime);
            // `FILD qword` of the difference with a zero high word.
            let elapsed = x87_store32(f64::from(now.wrapping_sub(start)));
            let length = x87_quiet_bits(e.mem.u32(base + 0xc0));
            let played = e
                .call(FLOAT_PAIR_0040EBD0, &args![x87_quiet_bits(elapsed), length])
                .f64();
            let played = x87_store32(played);
            let remaining = x87_sub(x87_load32(e.mem.u32(base + 0xc0)), x87_m32(played));
            let remaining = e.call(FTOL2, &args![remaining]).i32();
            let label = e
                .call(OBJECT_FIELD_00403DF0, &args![AUDIO_LABEL_011D3960])
                .u32();
            let tenths = (remaining % 1000) / 100;
            let seconds = (remaining % 60000) / 1000;
            let minutes = remaining / 60000;
            set_play_time_text(
                e,
                this,
                &[
                    PLAY_TIME_FORMAT,
                    minutes as u32,
                    seconds as u32,
                    tenths as u32,
                    label,
                ],
            );
            let fraction = x87_div(x87_load32(played), x87_m32(e.mem.u32(base + 0xc0)));
            let fraction = x87_quiet_bits(x87_store32(fraction));
            let completion = e
                .call(TEXT_TO_TRAIT, &args![AUDIO_FILE_COMPLETION_TEXT])
                .u32();
            let tile_13 = map_tile(e, base, 13);
            e.call(TILE_SET_FLOAT, &args![tile_13, completion, fraction, 1u32]);
        }
    }
    let node = e.get(this, MapMenu::pCurrentSound);
    if node == 0 {
        return;
    }
    let handle = e.call(SELF, &args![node]).u32();
    if e.call(SOUND_IS_VALID, &args![handle]).bool() {
        return;
    }
    if e.mem.u32(SOUND_WAIT_011DA410) == 0 {
        let now = e.call(WORD_AT_14, &args![TIMER_011F6394]).u32();
        e.mem.set_u32(SOUND_WAIT_011DA410, now);
    }
    let node = e.get(this, MapMenu::pCurrentSound);
    let has_next = e.call(WORD_AT_C, &args![node]).u32() != 0;
    let waited = has_next && {
        let now = e.call(WORD_AT_14, &args![TIMER_011F6394]).u32();
        now.wrapping_sub(e.mem.u32(SOUND_WAIT_011DA410)) > 0x1f4
    };
    if waited {
        let node = e.get(this, MapMenu::pCurrentSound);
        let next = e.call(WORD_AT_C, &args![node]).u32();
        e.set(this, MapMenu::pCurrentSound, next);
        let handle = e.call(SELF, &args![next]).u32();
        e.call(SOUND_PLAY, &args![handle, 0u32]);
        e.mem.set_u32(SOUND_WAIT_011DA410, 0);
        show_note_subtitle(e, this, 1);
        return;
    }
    let node = e.get(this, MapMenu::pCurrentSound);
    if e.call(WORD_AT_C, &args![node]).u32() != 0 {
        return;
    }
    let label = e
        .call(OBJECT_FIELD_00403DF0, &args![AUDIO_LABEL_011D3960])
        .u32();
    set_play_time_text(e, this, &[PLAY_DONE_FORMAT, label]);
    let completion = e
        .call(TEXT_TO_TRAIT, &args![AUDIO_FILE_COMPLETION_TEXT])
        .u32();
    let tile_13 = map_tile(e, base, 13);
    e.call(
        TILE_SET_FLOAT,
        &args![tile_13, completion, 1.0f32.to_bits(), 1u32],
    );
    let tile_13 = map_tile(e, base, 13);
    e.call(TILE_SET_INT, &args![tile_13, 0x1004u32, 0u32]);
    map_menu_stop_note_sound(e, this);
}

// Translated from 007a1b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSSimpleArray<BSStringT<char>,1024>`):
/// removes the element at `index`. With `shrink` and `006f3170`, moves the
/// elements before it into a new buffer (vtable +4 with the `00869600`
/// size), destroys the element, moves `iSize - 1` elements from after it
/// (the count the code uses), frees the old buffer and keeps the new one;
/// otherwise destroys it and moves the rest down. The size drops by one.
pub fn fn_007a1b10(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, shrink: u8) {
    if shrink != 0 && e.call(ARRAY_SHOULD_SHRINK_006F3170, &args![this]).bool() {
        let reserve = e.call(ARRAY_SHRUNK_SIZE_00869600, &args![this]).u32();
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(ARRAY_MOVE_00692480, &args![this, buffer, old, index]);
        let old = e.get(this, BSSimpleArray::pBuffer);
        fn_007a63b0(e, this, old.wrapping_add(index.wrapping_mul(8)), 1);
        let count = e.get(this, BSSimpleArray::iSize).wrapping_sub(1);
        let old = e.get(this, BSSimpleArray::pBuffer);
        let from = old.wrapping_add(index.wrapping_mul(8)).wrapping_add(8);
        let to = buffer.wrapping_add(index.wrapping_mul(8));
        e.call(ARRAY_MOVE_00692480, &args![this, to, from, count]);
        e.call(ARRAY_FREE_006A8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    } else {
        let old = e.get(this, BSSimpleArray::pBuffer);
        fn_007a63b0(e, this, old.wrapping_add(index.wrapping_mul(8)), 1);
        let count = e
            .get(this, BSSimpleArray::iSize)
            .wrapping_sub(index)
            .wrapping_sub(1);
        let old = e.get(this, BSSimpleArray::pBuffer);
        let from = old.wrapping_add(index.wrapping_mul(8)).wrapping_add(8);
        let old = e.get(this, BSSimpleArray::pBuffer);
        let to = old.wrapping_add(index.wrapping_mul(8));
        e.call(ARRAY_MOVE_00692480, &args![this, to, from, count]);
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_sub(1);
    e.set(this, BSSimpleArray::iSize, size);
}

// Translated from 0079af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::UpdateCurrentPage` (Xbox PDB): disables the quest, objective,
/// note and challenge lists (clearing the note and challenge containers'
/// trait `0xfa3`), resets tiles 11, 12, 18 and 21 (on `pMe`), then by the
/// open menu's page (the compiler's jump table at `0079b220`): 0x20 makes
/// tile 2 the map tile and updates the local map unless `007a1750`; 0x21
/// makes tile 4 the map tile and updates the world map; 0x22 updates the
/// quests; 0x23 shows tile 18 and, outside challenge mode, saves the note
/// list's state, updates the notes, reloads the selected note (selecting
/// its tile) and restores the list (`007a3580` with 1), then `0079b600` and
/// `ToggleNotesChallengeMode(0)`; 0x24 hides tile 12, keeps the radio
/// list's highlighted index (-1.0 without one), updates the radio list,
/// focuses the tile at that index and calls the list's vtable +0x10.
pub fn map_menu_update_current_page(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    list_box_tes_quest_set_enabled(e, Ptr::new(base + 0x130), 0);
    list_box_bgs_quest_objective_set_enabled(e, Ptr::new(base + 0x1c0), 0);
    list_box_bgs_note_set_enabled(e, Ptr::new(base + 0x160), 0);
    fn_007a5a30(e, Ptr::new(base + 0x160), 0);
    list_box_tes_challenge_set_enabled(e, Ptr::new(base + 0x1f0), 0);
    fn_007a5a30(e, Ptr::new(base + 0x1f0), 0);
    for (index, id, value) in [
        (11u32, 0xfafu32, 1u32),
        (12, 0xfa3, 1),
        (18, 0xfa3, 0),
        (21, 0xfa3, 0),
    ] {
        let tile = map_tile(e, map_menu(e), index);
        e.call(TILE_SET_INT, &args![tile, id, value]);
    }
    let page = e.mem.u32(map_menu(e) + 0x80);
    match page.wrapping_sub(0x20) {
        0 => {
            let menu = map_menu(e);
            let tile = map_tile(e, map_menu(e), 2);
            e.mem.set_u32(menu + 0x84, tile);
            if !fn_007a1750(e) {
                let menu = map_menu(e);
                map_menu_update_local_map(e, Ptr::new(menu));
            }
        }
        1 => {
            let menu = map_menu(e);
            let tile = map_tile(e, map_menu(e), 4);
            e.mem.set_u32(menu + 0x84, tile);
            let menu = map_menu(e);
            map_menu_update_world_map(e, Ptr::new(menu));
        }
        2 => {
            let menu = map_menu(e);
            map_menu_update_quests(e, Ptr::new(menu));
        }
        3 => {
            let tile_18 = map_tile(e, map_menu(e), 18);
            e.call(TILE_SET_INT, &args![tile_18, 0xfa3u32, 1u32]);
            if e.mem.u8(CHALLENGE_MODE_011DA36D) == 0 {
                let menu = map_menu(e);
                list_box_bgs_note_save_list_state(e, Ptr::new(menu + 0x160));
                let menu = map_menu(e);
                map_menu_update_notes(e, Ptr::new(menu));
                if e.get(this, MapMenu::pSelectedNote) != 0
                    && e.mem.u8(CHALLENGE_MODE_011DA36D) == 0
                {
                    let tile = e
                        .call(LIST_BOX_GET_TILE_FOR, &args![base + 0x160, base + 0x90])
                        .u32();
                    let note = e.get(this, MapMenu::pSelectedNote);
                    map_menu_load_note(e, this, note);
                    if tile != 0 {
                        let selected = e.mem.u32(TRAIT_SELECTED);
                        e.call(TILE_SET_INT, &args![tile, selected, 1u32]);
                    }
                }
                let menu = map_menu(e);
                fn_007a3580(e, Ptr::new(menu + 0x160), 1);
            }
            let menu = map_menu(e);
            fn_0079b600(e, Ptr::new(menu));
            map_menu_toggle_notes_challenge_mode(e, this, 0);
        }
        4 => {
            let tile_12 = map_tile(e, map_menu(e), 12);
            e.call(TILE_SET_INT, &args![tile_12, 0xfa3u32, 0u32]);
            let list = base + 0x190;
            let value = if e.vcall(list, 4, &args![]).u32() != 0 {
                let tile = e.vcall(list, 4, &args![]).u32();
                x87_store32(e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64())
            } else {
                e.mem.u32(MINUS_ONE_FLOAT)
            };
            let index = e.call(FTOL2, &args![x87_load32(value)]).u32();
            let menu = map_menu(e);
            map_menu_update_radio_list(e, Ptr::new(menu));
            let tile = e.vcall(list, 0x14, &args![index, 0u32]).u32();
            if tile != 0 {
                let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
                e.call(
                    SET_CURRENT_FOCUS_TARGET,
                    &args![interface, tile, 0xfc3u32, 1u32],
                );
            }
            e.vcall(list, 0x10, &args![]);
        }
        _ => {}
    }
}

/// Thiscall on the player: the number of notes it has (`00966a50`), the
/// note at an index (`00966bb0`, `RET 4`) and a word the notes filter
/// compares with (`005cbb50`).
const PLAYER_NOTE_COUNT_00966A50: u32 = 0x0096_6a50;
const PLAYER_NOTE_AT_00966BB0: u32 = 0x0096_6bb0;
const PLAYER_QUEST_005CBB50: u32 = 0x005c_bb50;
/// Thiscall on a note: its first list node (`004fadc0`).
const NOTE_LIST_004FADC0: u32 = 0x004f_adc0;
/// The notes filter callback `UpdateNotes` passes (`0079b390`).
const NOTES_FILTER_CALLBACK_0079B390: u32 = 0x0079_b390;
/// The challenge list callbacks `0079b600` passes (`0079b720` filter,
/// `0079b820` sort).
const CHALLENGES_FILTER_CALLBACK_0079B720: u32 = 0x0079_b720;
const CHALLENGES_SORT_CALLBACK_0079B820: u32 = 0x0079_b820;
/// The objects whose strings (`00403df0`) the notes page shows as its
/// heading: challenge mode (`011d2d18`) and notes (`011d2a78`).
const CHALLENGES_HEADING_011D2D18: u32 = 0x011d_2d18;
const NOTES_HEADING_011D2A78: u32 = 0x011d_2a78;
/// Thiscall on the object at `011c3f2c`: the first node of its challenge
/// list (`00461150`).
const CHALLENGE_LIST_00461150: u32 = 0x0046_1150;
/// Thiscall on a challenge: `EAX` values `0079b600` and `0079b720` test.
const CHALLENGE_STATE_005F5930: u32 = 0x005f_5930;
const CHALLENGE_STATE_005A60D0: u32 = 0x005a_60d0;
const CHALLENGE_STATE_005A6150: u32 = 0x005a_6150;
const CHALLENGE_KIND_006A1B60: u32 = 0x006a_1b60;
const CHALLENGE_STATE_005DEF70: u32 = 0x005d_ef70;
/// The `double` 128.0 (`0102e430`) the challenge sort compares the text
/// alpha with (besides `FULL_ALPHA_DOUBLE`).
const HALF_ALPHA_DOUBLE: u32 = 0x0102_e430;

/// The text alpha of a list item: `HALF_ALPHA_FLOAT` when `dim`, else
/// `FULL_ALPHA_FLOAT`, into the static trait `011da344`.
fn set_item_alpha(e: &mut Engine, tile: u32, dim: bool) {
    let alpha = if dim {
        float_constant(e, HALF_ALPHA_FLOAT)
    } else {
        float_constant(e, FULL_ALPHA_FLOAT)
    };
    let alpha_id = e.mem.u32(TRAIT_TEXT_ALPHA);
    e.call(TILE_SET_FLOAT, &args![tile, alpha_id, alpha, 1u32]);
}

// Translated from 0079b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::UpdateNotes` (Xbox PDB): with an open map menu, empties the
/// note list (vtable +0x1C) and inserts each of the player's notes (its name
/// from +0x48) with trait `0xfaa` 0x18 and its text alpha (half when
/// `00798420`), enables and filters the list (`0079b390`), and gives tile
/// 12 `"_ShowingAllNotes"` and trait `0xfaf` (the player's `005cbb50` set).
pub fn map_menu_update_notes(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    if map_menu(e) == 0 {
        return;
    }
    e.vcall(base + 0x160, 0x1c, &args![]);
    let mut index = 0i32;
    loop {
        let player = e.mem.u32(PLAYER_011DEA3C);
        if index >= e.call(PLAYER_NOTE_COUNT_00966A50, &args![player]).i32() {
            break;
        }
        let player = e.mem.u32(PLAYER_011DEA3C);
        let note = e.call(PLAYER_NOTE_AT_00966BB0, &args![player, index]).u32();
        let name = e.call(MARKER_GET_LOCATION_NAME, &args![note + 0x48]).u32();
        let tile = fn_007a2930(e, Ptr::new(base + 0x160), note, name, 0, 0);
        e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x18u32]);
        let dim = fn_00798420(e, Ptr::new(note)) != 0;
        set_item_alpha(e, tile, dim);
        index = index.wrapping_add(1);
    }
    list_box_bgs_note_set_enabled(e, Ptr::new(base + 0x160), 1);
    list_box_bgs_note_filter(e, Ptr::new(base + 0x160), NOTES_FILTER_CALLBACK_0079B390);
    let showing = u32::from(e.get(this, MapMenu::bShowingAllNotes));
    let showing_id = e.call(TEXT_TO_TRAIT, &args![SHOWING_ALL_NOTES_TEXT]).u32();
    let tile_12 = map_tile(e, base, 12);
    e.call(TILE_SET_INT, &args![tile_12, showing_id, showing]);
    let player = e.mem.u32(PLAYER_011DEA3C);
    let word = e.call(PLAYER_QUEST_005CBB50, &args![player]).u32();
    let tile_12 = map_tile(e, base, 12);
    e.call(
        TILE_SET_INT,
        &args![tile_12, 0xfafu32, u32::from(word != 0)],
    );
}

// Translated from 0079b390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; the notes filter callback): 1 (filtered
/// out) unless the map menu shows only some notes and the note's list
/// (`004fadc0`) holds the player's `005cbb50` word; 0 when the menu shows
/// all notes.
pub fn fn_0079b390(e: &mut Engine, note: u32) -> u8 {
    let mut filtered = 1u8;
    if map_menu(e) != 0 && e.mem.u8(map_menu(e) + 0x8c) == 0 {
        let mut node = e.call(NOTE_LIST_004FADC0, &args![note]).u32();
        while node != 0 && node_item(e, node) != 0 {
            let at = e.call(SELF, &args![node]).u32();
            let player = e.mem.u32(PLAYER_011DEA3C);
            let word = e.call(PLAYER_QUEST_005CBB50, &args![player]).u32();
            if e.mem.u32(at) == word {
                filtered = 0;
            }
            node = e.call(WORD_AT_4, &args![node]).u32();
        }
    } else if map_menu(e) != 0 && e.mem.u8(map_menu(e) + 0x8c) != 0 {
        filtered = 0;
    }
    filtered
}

// Translated from 0079b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::ToggleNotesChallengeMode` (Xbox PDB): with `toggle` flips the
/// challenge mode byte (`011da36d`); enables the challenge list and its
/// container (`0xfa3`) in challenge mode and the note list otherwise, shows
/// tile 13, empties its `"MM_DataText"` when toggling, and sets the page
/// heading (tile 18) and tile 21 (the challenge type name from `011a03e4`)
/// or tile 12's trait `0xfaf`; with `toggle`, `UpdateCurrentPage`.
pub fn map_menu_toggle_notes_challenge_mode(e: &mut Engine, this: Ptr<MapMenu>, toggle: u8) {
    let base = this.addr();
    if toggle != 0 {
        let mode = e.mem.u8(CHALLENGE_MODE_011DA36D);
        e.mem.set_u8(CHALLENGE_MODE_011DA36D, u8::from(mode == 0));
    }
    let mode = e.mem.u8(CHALLENGE_MODE_011DA36D);
    list_box_tes_challenge_set_enabled(e, Ptr::new(base + 0x1f0), mode);
    let mode = e.mem.u8(CHALLENGE_MODE_011DA36D);
    fn_007a5a30(e, Ptr::new(base + 0x1f0), mode);
    let notes = u8::from(e.mem.u8(CHALLENGE_MODE_011DA36D) == 0);
    list_box_bgs_note_set_enabled(e, Ptr::new(base + 0x160), notes);
    let notes = u8::from(e.mem.u8(CHALLENGE_MODE_011DA36D) == 0);
    fn_007a5a30(e, Ptr::new(base + 0x160), notes);
    let tile_13 = map_tile(e, base, 13);
    e.call(TILE_SET_INT, &args![tile_13, 0xfa3u32, 1u32]);
    if toggle != 0 {
        let tile_13 = map_tile(e, base, 13);
        let data_text = e
            .call(TILE_GET_CHILD_BY_NAME, &args![tile_13, DATA_TEXT_NAME])
            .u32();
        e.call(
            TILE_SET_STRING,
            &args![data_text, 0xfc4u32, EMPTY_TEXT, 1u32],
        );
    }
    if e.mem.u8(CHALLENGE_MODE_011DA36D) != 0 {
        let heading = e
            .call(OBJECT_FIELD_00403DF0, &args![CHALLENGES_HEADING_011D2D18])
            .u32();
        let tile_18 = map_tile(e, map_menu(e), 18);
        e.call(TILE_SET_STRING, &args![tile_18, 0xfc4u32, heading, 1u32]);
        let tile_21 = map_tile(e, map_menu(e), 21);
        e.call(TILE_SET_INT, &args![tile_21, 0xfa3u32, 1u32]);
        let kind = e.mem.u8(CHALLENGE_TYPE_011DA36E) as i8 as i32 as u32;
        let names = e
            .mem
            .u32(CHALLENGE_TYPE_NAMES_011A03E4.wrapping_add(kind.wrapping_mul(4)));
        let name = e.call(OBJECT_FIELD_00403DF0, &args![names]).u32();
        let tile_21 = map_tile(e, base, 21);
        e.call(TILE_SET_STRING, &args![tile_21, 0xfc4u32, name, 1u32]);
    } else {
        let heading = e
            .call(OBJECT_FIELD_00403DF0, &args![NOTES_HEADING_011D2A78])
            .u32();
        let tile_18 = map_tile(e, map_menu(e), 18);
        e.call(TILE_SET_STRING, &args![tile_18, 0xfc4u32, heading, 1u32]);
        let tile_12 = map_tile(e, map_menu(e), 12);
        e.call(TILE_SET_INT, &args![tile_12, 0xfafu32, 1u32]);
        let tile_21 = map_tile(e, map_menu(e), 21);
        e.call(TILE_SET_INT, &args![tile_21, 0xfa3u32, 0u32]);
    }
    if toggle != 0 {
        map_menu_update_current_page(e, this);
    }
}

// Translated from 0079b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`MapMenu`, the challenge list): empties the
/// challenge list (vtable +0x1C) and inserts every challenge of the list at
/// `[011c3f2c]` (`00461150`) for which `005f5930` is 0 or `005a60d0` is set,
/// with trait `0xfaa` 0x18 and its text alpha (half when `005a6150`); then
/// filters it (`0079b720`) and sorts it (vtable +0x20 with `0079b820`).
pub fn fn_0079b600(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    e.vcall(base + 0x1f0, 0x1c, &args![]);
    let owner = e.mem.u32(MARKER_OWNER_011C3F2C);
    let mut node = e.call(CHALLENGE_LIST_00461150, &args![owner]).u32();
    while node != 0 {
        let challenge = node_item(e, node);
        if challenge != 0
            && (e.call(CHALLENGE_STATE_005F5930, &args![challenge]).u32() == 0
                || e.call(CHALLENGE_STATE_005A60D0, &args![challenge]).u32() != 0)
        {
            let name = e
                .call(MARKER_GET_LOCATION_NAME, &args![challenge + 0x18])
                .u32();
            let tile = fn_007a5230(e, Ptr::new(base + 0x1f0), challenge, name, 0, 0);
            e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x18u32]);
            let dim = e.call(CHALLENGE_STATE_005A6150, &args![challenge]).u32() != 0;
            set_item_alpha(e, tile, dim);
        }
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    list_box_tes_challenge_filter(
        e,
        Ptr::new(base + 0x1f0),
        CHALLENGES_FILTER_CALLBACK_0079B720,
    );
    e.vcall(
        base + 0x1f0,
        0x20,
        &args![CHALLENGES_SORT_CALLBACK_0079B820],
    );
}

// Translated from 0079b720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; the challenge filter callback): 1
/// (filtered out) for a challenge with no `005e3fa0` above 0, no `0079b800`
/// and no `005def70`; otherwise 0 when its kind (`006a1b60`) belongs to the
/// displayed challenge type (`011da36e`: 0 for kinds 0 to 2, 1 for 3 and
/// 8, 2 for 4 to 6 and 9 to 0xD), else 1. (`005a60d0` is asked with an open
/// map menu; either answer keeps 1.)
pub fn fn_0079b720(e: &mut Engine, challenge: u32) -> u8 {
    let result = 1u8;
    if map_menu(e) != 0 {
        // Both outcomes leave the result at 1.
        e.call(CHALLENGE_STATE_005A60D0, &args![challenge]);
    }
    let kind = e.call(CHALLENGE_KIND_006A1B60, &args![challenge]).u32();
    if e.call(CHALLENGE_VALUE_005E3FA0, &args![challenge]).i32() <= 0
        && !fn_0079b800(e, Ptr::new(challenge))
        && e.call(CHALLENGE_STATE_005DEF70, &args![challenge]).u32() == 0
    {
        return 1;
    }
    let shown = e.mem.u8(CHALLENGE_TYPE_011DA36E) as i8;
    if shown == 0 && matches!(kind, 0..=2) {
        return 0;
    }
    if shown == 1 && matches!(kind, 3 | 8) {
        return 0;
    }
    if shown == 2 && matches!(kind, 0xb | 0xa | 9 | 6 | 5 | 4 | 0xc | 0xd) {
        return 0;
    }
    result
}

// Translated from 0079b820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; the challenge sort callback on two list
/// items): 1 when the first item's text alpha (static trait `011da344`) is
/// 128.0 (`0102e430`) and the second's 255.0 (`FULL_ALPHA_DOUBLE`), -1 for the
/// reverse, else the comparison (`00408b20`) of their names (+0x18 of the
/// data).
pub fn fn_0079b820(e: &mut Engine, first: u32, second: u32) -> i32 {
    let alpha_id = e.mem.u32(TRAIT_TEXT_ALPHA);
    let tile = e.mem.u32(first);
    let a = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, alpha_id]).f64());
    let alpha_id = e.mem.u32(TRAIT_TEXT_ALPHA);
    let tile = e.mem.u32(second);
    let b = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, alpha_id]).f64());
    let half: f64 = e.global(HALF_ALPHA_DOUBLE);
    let full: f64 = e.global(FULL_ALPHA_DOUBLE);
    if equal(x87_load32(a), half) && equal(x87_load32(b), full) {
        return 1;
    }
    if equal(x87_load32(a), full) && equal(x87_load32(b), half) {
        return -1;
    }
    let data = e.mem.u32(second + 4);
    let second_name = e.call(MARKER_GET_LOCATION_NAME, &args![data + 0x18]).u32();
    let data = e.mem.u32(first + 4);
    let first_name = e.call(MARKER_GET_LOCATION_NAME, &args![data + 0x18]).u32();
    e.call(COMPARE_00408B20, &args![first_name, second_name])
        .i32()
}

/// Thiscall on a quest node: `AL`, the end of the walk (`008256d0`).
const QUEST_NODE_END_008256D0: u32 = 0x0082_56d0;
/// Thiscall on a quest objective: `AL` (`005a5e70`, the objective is
/// shown) and its text (`005ec330`).
const OBJECTIVE_SHOWN_005A5E70: u32 = 0x005a_5e70;
const OBJECTIVE_NAME_005EC330: u32 = 0x005e_c330;
/// Thiscall on a tile's child list (+4) (node), `RET 4`: the address of
/// the child (`006317a0`).
const CHILD_OF_006317A0: u32 = 0x0063_17a0;
/// `"_ShowEmptyMarker"` and the float an objective's child gets as trait
/// `0xfba` (`01074ffc`, 320.0 in the shipped exe).
const SHOW_EMPTY_MARKER_TEXT: u32 = 0x0107_4fe8;
const OBJECTIVE_WIDTH_FLOAT: u32 = 0x0107_4ffc;
/// The quest sort callback `UpdateQuests` passes to both lists (`0079bc90`).
const QUESTS_SORT_CALLBACK_0079BC90: u32 = 0x0079_bc90;
/// Thiscall on a radio station: its name (`0055d520`).
const STATION_NAME_0055D520: u32 = 0x0055_d520;
/// `BGSTalkingActivator::GetRadioStationsInRangeOfRef` (engine map), cdecl
/// (reference, list, word).
const GET_RADIO_STATIONS_IN_RANGE: u32 = 0x004f_f1a0;
/// Thiscall on a tile (text), `RET 4` (`00707e10`).
const TILE_TEXT_SETTER_00707E10: u32 = 0x0070_7e10;
/// Thiscall on a list (address of an item), `RET 4`: `AL`, whether it holds
/// the item (`005f65d0`).
const LIST_CONTAINS_005F65D0: u32 = 0x005f_65d0;
/// The radio list callbacks `UpdateRadioList` passes (`0079be30` filter,
/// `0079bd70` sort).
const RADIO_LIST_FILTER_CALLBACK_0079BE30: u32 = 0x0079_be30;
const RADIO_SORT_CALLBACK_0079BD70: u32 = 0x0079_bd70;

/// `FCOMP double; TEST AH,0x44`: whether the tile's text alpha (static
/// trait `011da344`, asked each time) equals the `double` at `limit`.
fn alpha_equals(e: &mut Engine, tile: u32, limit: u32) -> bool {
    let alpha_id = e.mem.u32(TRAIT_TEXT_ALPHA);
    let alpha = e.call(TILE_GET_FLOAT, &args![tile, alpha_id]).f64();
    let limit: f64 = e.global(limit);
    equal(alpha, limit)
}

// Translated from 0079b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::UpdateQuests` (Xbox PDB): empties the quest and objective
/// lists (vtable +0x1C) and walks the player's objectives (`0079bc30`, until
/// `008256d0`): each shown one (`005a5e70`) is inserted with its text, its
/// first child's trait `0xfba` set to `01074ffc`, `"_ShowEmptyMarker"` 1,
/// `"_enabled"` 0 and its text alpha (half and selected when `005a5dc0`,
/// half when its quest is done, else full); its quest (`0044edb0`) is added
/// once (name from +0x30, trait `0xfaa` 0x17, alpha by `0059e400`, selected
/// when it is the player's active quest). Both lists are sorted with
/// `0079bc90` and the quest list enabled; with an active quest it is
/// clicked (vtable +0x10 with id 0x17) and, with `004b71d0`, focused and the
/// list's vtable +0x10 called; without one the objective list is filtered
/// with `0079bc50`.
pub fn map_menu_update_quests(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    e.vcall(base + 0x130, 0x1c, &args![]);
    e.vcall(base + 0x1c0, 0x1c, &args![]);
    let player = e.mem.u32(PLAYER_011DEA3C);
    let mut node = fn_0079bc30(e, Ptr::new(player));
    e.with_stack(4, |e, quest_slot| {
        while node != 0 && !e.call(QUEST_NODE_END_008256D0, &args![node]).bool() {
            let objective = node_item(e, node);
            let quest = e.call(PROCESS_VALUE_0044EDB0, &args![objective]).u32();
            e.mem.set_u32(quest_slot.addr(), quest);
            let objective = node_item(e, node);
            if e.call(OBJECTIVE_SHOWN_005A5E70, &args![objective]).bool() {
                let objective = node_item(e, node);
                let text = e.call(OBJECTIVE_NAME_005EC330, &args![objective]).u32();
                let objective = node_item(e, node);
                let tile = fn_007a44e0(e, Ptr::new(base + 0x1c0), objective, text, 0, 0);
                let width = float_constant(e, OBJECTIVE_WIDTH_FLOAT);
                let first = e.call(WORD_AT_4, &args![tile + 4]).u32();
                let child = e.call(CHILD_OF_006317A0, &args![tile + 4, first]).u32();
                let child = e.mem.u32(child);
                e.call(TILE_SET_FLOAT, &args![child, 0xfbau32, width, 1u32]);
                let empty_marker = e.call(TEXT_TO_TRAIT, &args![SHOW_EMPTY_MARKER_TEXT]).u32();
                e.call(TILE_SET_INT, &args![tile, empty_marker, 1u32]);
                let enabled = e.call(TEXT_TO_TRAIT, &args![ENABLED_TEXT]).u32();
                e.call(TILE_SET_INT, &args![tile, enabled, 0u32]);
                let objective = node_item(e, node);
                if e.call(OBJECTIVE_FLAG_005A5DC0, &args![objective]).bool() {
                    set_item_alpha(e, tile, true);
                    let selected = e.mem.u32(TRAIT_SELECTED);
                    e.call(TILE_SET_INT, &args![tile, selected, 1u32]);
                } else {
                    let quest = e.mem.u32(quest_slot.addr());
                    let done = e.call(QUEST_FLAG_0059E400, &args![quest]).bool();
                    set_item_alpha(e, tile, done);
                }
                let listed = e
                    .call(LIST_BOX_GET_TILE_FOR, &args![base + 0x130, quest_slot])
                    .u32();
                if listed == 0 {
                    let quest = e.mem.u32(quest_slot.addr());
                    let name = e.call(MARKER_GET_LOCATION_NAME, &args![quest + 0x30]).u32();
                    let quest = e.mem.u32(quest_slot.addr());
                    let quest_tile = fn_007a1dd0(e, Ptr::new(base + 0x130), quest, name, 0, 0);
                    e.call(TILE_SET_INT, &args![quest_tile, 0xfaau32, 0x17u32]);
                    let quest = e.mem.u32(quest_slot.addr());
                    let done = e.call(QUEST_FLAG_0059E400, &args![quest]).bool();
                    set_item_alpha(e, quest_tile, done);
                    let player = e.mem.u32(PLAYER_011DEA3C);
                    let active = e.call(PLAYER_QUEST_005CBB50, &args![player]).u32();
                    if e.mem.u32(quest_slot.addr()) == active {
                        let selected = e.mem.u32(TRAIT_SELECTED);
                        e.call(TILE_SET_INT, &args![quest_tile, selected, 1u32]);
                    }
                }
            }
            node = e.call(WORD_AT_4, &args![node]).u32();
        }
    });
    e.vcall(base + 0x1c0, 0x20, &args![QUESTS_SORT_CALLBACK_0079BC90]);
    e.vcall(base + 0x130, 0x20, &args![QUESTS_SORT_CALLBACK_0079BC90]);
    list_box_tes_quest_set_enabled(e, Ptr::new(base + 0x130), 1);
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.call(PLAYER_QUEST_005CBB50, &args![player]).u32() == 0 {
        list_box_bgs_quest_objective_filter(
            e,
            Ptr::new(base + 0x1c0),
            OBJECTIVE_QUEST_CALLBACK_0079BC50,
        );
        return;
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    let active = e.call(PLAYER_QUEST_005CBB50, &args![player]).u32();
    let tile = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), active);
        e.call(LIST_BOX_GET_TILE_FOR, &args![base + 0x130, slot])
            .u32()
    });
    e.vcall(base, 0x10, &args![0x17u32, tile]);
    if e.call(FLAG_004B71D0, &args![]).bool() {
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        e.call(
            SET_CURRENT_FOCUS_TARGET,
            &args![interface, tile, 0xfc3u32, 1u32],
        );
        e.vcall(base + 0x130, 0x10, &args![]);
    }
}

// Translated from 0079bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; `UpdateQuests`' sort callback on two
/// list items): 1 when the first item's text alpha is 128.0 and the
/// second's 255.0, -1 for the reverse; otherwise by their indices (trait
/// `0xfac`, as `float`s): -1 when the second's is above the first's, 1 when
/// it is below, else 0.
pub fn fn_0079bc90(e: &mut Engine, first: u32, second: u32) -> i32 {
    let alpha_id = e.mem.u32(TRAIT_TEXT_ALPHA);
    let tile = e.mem.u32(first);
    let a = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, alpha_id]).f64());
    let alpha_id = e.mem.u32(TRAIT_TEXT_ALPHA);
    let tile = e.mem.u32(second);
    let b = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, alpha_id]).f64());
    let half: f64 = e.global(HALF_ALPHA_DOUBLE);
    let full: f64 = e.global(FULL_ALPHA_DOUBLE);
    if equal(x87_load32(a), half) && equal(x87_load32(b), full) {
        return 1;
    }
    if equal(x87_load32(a), full) && equal(x87_load32(b), half) {
        return -1;
    }
    let tile = e.mem.u32(first);
    let first_index = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64());
    let tile = e.mem.u32(second);
    let second_index = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64());
    if above(x87_load32(second_index), x87_load32(first_index)) {
        return -1;
    }
    if below(x87_load32(second_index), x87_load32(first_index)) {
        return 1;
    }
    0
}

// Translated from 0079bd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; `UpdateRadioList`'s sort callback on
/// two list items): the code tests only the first item's text alpha against
/// 255.0 (`FULL_ALPHA_DOUBLE`), each time asking again: -1 when it is equal
/// and then not; 1 when it is not and then is; otherwise the comparison
/// (`00408b20`) of the stations' names (`0055d520`).
pub fn fn_0079bd70(e: &mut Engine, first: u32, second: u32) -> i32 {
    let tile = e.mem.u32(first);
    if alpha_equals(e, tile, FULL_ALPHA_DOUBLE) {
        let tile = e.mem.u32(first);
        if !alpha_equals(e, tile, FULL_ALPHA_DOUBLE) {
            return -1;
        }
    }
    let tile = e.mem.u32(first);
    if !alpha_equals(e, tile, FULL_ALPHA_DOUBLE) {
        let tile = e.mem.u32(first);
        if alpha_equals(e, tile, FULL_ALPHA_DOUBLE) {
            return 1;
        }
    }
    let station = e.mem.u32(second + 4);
    let second_name = e.call(STATION_NAME_0055D520, &args![station]).u32();
    let station = e.mem.u32(first + 4);
    let first_name = e.call(STATION_NAME_0055D520, &args![station]).u32();
    e.call(COMPARE_00408B20, &args![first_name, second_name])
        .i32()
}

// Translated from 0079bea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079bea0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the local station list) is not translated.
/// `MapMenu::UpdateRadioList` (Xbox PDB): gathers the stations in range of
/// the player into a local list (`GetRadioStationsInRangeOfRef`), empties
/// the radio list (vtable +0x1C) and inserts every station of `011dd59c`
/// (name `0055d520`, trait `0xfaa` 0x19, `00707e10` with the name, selected
/// when it is the station `0079c070` points at, full alpha when in range
/// else half); then filters it (`0079be30`), sorts it (vtable +0x20 with
/// `0079bd70`) and empties and destroys the local list.
pub fn map_menu_update_radio_list(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    e.with_stack(8, |e, in_range| {
        e.call(LIST_CONSTRUCT, &args![in_range]);
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(GET_RADIO_STATIONS_IN_RANGE, &args![player, in_range, 0u32]);
        e.vcall(base + 0x190, 0x1c, &args![]);
        let mut node = fn_0079c080(e);
        while node != 0 && node_item(e, node) != 0 {
            let station = node_item(e, node);
            let name = e.call(STATION_NAME_0055D520, &args![station]).u32();
            let station = node_item(e, node);
            let tile = fn_007a3820(e, Ptr::new(base + 0x190), station, name, 0, 0);
            e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x19u32]);
            let station = node_item(e, node);
            let name = e.call(STATION_NAME_0055D520, &args![station]).u32();
            e.call(TILE_TEXT_SETTER_00707E10, &args![tile, name]);
            let selected = fn_0079c070(e) != 0 && {
                let at = e.call(SELF, &args![node]).u32();
                let current = fn_0079c070(e);
                e.mem.u32(at) == e.mem.u32(current)
            };
            let selected_id = e.mem.u32(TRAIT_SELECTED);
            e.call(TILE_SET_INT, &args![tile, selected_id, u32::from(selected)]);
            let at = e.call(SELF, &args![node]).u32();
            let reachable = e.call(LIST_CONTAINS_005F65D0, &args![in_range, at]).bool();
            set_item_alpha(e, tile, !reachable);
            node = e.call(WORD_AT_4, &args![node]).u32();
        }
        list_box_tes_object_refr_filter(
            e,
            Ptr::new(base + 0x190),
            RADIO_LIST_FILTER_CALLBACK_0079BE30,
        );
        e.vcall(base + 0x190, 0x20, &args![RADIO_SORT_CALLBACK_0079BD70]);
        e.call(LIST_REMOVE_ALL, &args![in_range]);
        e.call(LIST_DESTRUCT, &args![in_range]);
    });
}

/// The object whose string (`00403df0`) is the waveform image format
/// (`011d505c`).
const WAVEFORM_FORMAT_011D505C: u32 = 0x011d_505c;
/// Cdecl: a millisecond clock (`00457fe0`).
const CLOCK_00457FE0: u32 = 0x0045_7fe0;
/// Thiscall (word), `RET 4`, and its destructor (`0045cec0`): the local
/// `0079c0c0` builds and destroys around the first frame.
const WAVEFORM_LOCAL_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
const WAVEFORM_LOCAL_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// Thiscall on the shader property (float, float), `RET 8` (`0077a450`).
const PROPERTY_OFFSET_0077A450: u32 = 0x0077_a450;
/// The waveform's scroll offset (`011da414`, a `float`) and the time of its
/// last step (`011da418`).
const WAVEFORM_OFFSET_011DA414: u32 = 0x011d_a414;
const WAVEFORM_STEP_TIME_011DA418: u32 = 0x011d_a418;
/// The `double`s: the offset step (`0101ffa0`) and the waveform's frame time
/// (`01075000`).
const WAVEFORM_STEP_DOUBLE: u32 = 0x0101_ffa0;
const WAVEFORM_FRAME_DOUBLE: u32 = 0x0107_5000;
/// Cdecl: a random word (`00487f50`).
const RANDOM_00487F50: u32 = 0x0048_7f50;
/// Cdecl (buffer, size, text): copies the text (`00406d30`).
const BUFFER_COPY_00406D30: u32 = 0x0040_6d30;
/// `"Forward"`, `"Backward"`, `"Left"`.
const WAVEFORM_FORWARD_TEXT: u32 = 0x0104_8458;
const WAVEFORM_BACKWARD_TEXT: u32 = 0x0104_844c;
const WAVEFORM_LEFT_TEXT: u32 = 0x0104_8444;

// Translated from 0079c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079c0c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the first frame's local) and the /GS security cookie check (00ec408c) are not translated.
/// Unnamed in the engine map (`MapMenu`, the radio waveform on tile 16):
/// with tile 16's shader property, starts the animation when
/// `fWaveformAnimationStartTime` is 0.0 (the image path from the `011d505c`
/// format and trait `0xfd2` into `0xfcc`, the clock as the start time,
/// trait `0xfa3` 1); every 10 ms moves the property's offset (`0077a450`)
/// by the step, wrapping above 1.0; after the frame time (`01075000`) picks
/// `"Forward"`, `"Backward"` or `"Left"` by a random number mod 3 as trait
/// `0xfd2` and clears the start time.
pub fn fn_0079c0c0(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    // The code keeps 0x2EE and 0xA in locals it never reads.
    let tile_16 = map_tile(e, map_menu(e), 16);
    let node = if e.call(TILE_OBJECT_0056C7F0, &args![tile_16]).u32() != 0 {
        let tile_16 = map_tile(e, map_menu(e), 16);
        let object = e.call(TILE_OBJECT_0056C7F0, &args![tile_16]).u32();
        e.call(OBJECT_NODE_0045BC00, &args![object, 0u32]).u32()
    } else {
        0
    };
    let property = if node != 0 {
        let kind = e.call(PROPERTY_TYPE_00438220, &args![]).u32();
        e.call(AV_OBJECT_GET_PROPERTY, &args![node, kind]).u32()
    } else {
        0
    };
    if property == 0 {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if equal(x87_load32(e.mem.u32(base + 0x88)), zero) {
        e.with_stack(0x44, |e, frame| {
            let local = frame.addr();
            let text = local + 4;
            e.call(WAVEFORM_LOCAL_CONSTRUCT_00633C90, &args![local, 0u32]);
            let tile_16 = map_tile(e, base, 16);
            let name = e.call(TILE_GET_STRING, &args![tile_16, 0xfd2u32]).u32();
            let format = e
                .call(OBJECT_FIELD_00403DF0, &args![WAVEFORM_FORMAT_011D505C])
                .u32();
            e.call(BUFFER_FORMAT, &args![text, 0x40u32, format, name]);
            let tile_16 = map_tile(e, map_menu(e), 16);
            e.call(TILE_SET_STRING, &args![tile_16, 0xfccu32, text, 1u32]);
            let now = e.call(CLOCK_00457FE0, &args![]).u32();
            e.mem.set_u32(base + 0x88, x87_store32(f64::from(now)));
            let tile_16 = map_tile(e, base, 16);
            e.call(TILE_SET_INT, &args![tile_16, 0xfa3u32, 1u32]);
            e.call(WAVEFORM_LOCAL_DESTRUCT_0045CEC0, &args![local]);
        });
    }
    let now = e.call(CLOCK_00457FE0, &args![]).u32();
    if now.wrapping_sub(e.mem.u32(WAVEFORM_STEP_TIME_011DA418)) >= 10 {
        let offset = x87_quiet_bits(e.mem.u32(WAVEFORM_OFFSET_011DA414));
        e.call(
            PROPERTY_OFFSET_0077A450,
            &args![property, offset, 0.0f32.to_bits()],
        );
        let step: f64 = e.global(WAVEFORM_STEP_DOUBLE);
        let offset = x87_store32(x87_add(
            x87_load32(e.mem.u32(WAVEFORM_OFFSET_011DA414)),
            step,
        ));
        e.mem.set_u32(WAVEFORM_OFFSET_011DA414, offset);
        let one: f64 = e.global(ONE_DOUBLE);
        if above(x87_load32(e.mem.u32(WAVEFORM_OFFSET_011DA414)), one) {
            let offset = x87_store32(x87_sub(
                x87_load32(e.mem.u32(WAVEFORM_OFFSET_011DA414)),
                one,
            ));
            e.mem.set_u32(WAVEFORM_OFFSET_011DA414, offset);
        }
        let now = e.call(CLOCK_00457FE0, &args![]).u32();
        e.mem.set_u32(WAVEFORM_STEP_TIME_011DA418, now);
    }
    let now = e.call(CLOCK_00457FE0, &args![]).u32();
    let elapsed = x87_sub(f64::from(now), x87_m32(e.mem.u32(base + 0x88)));
    let frame: f64 = e.global(WAVEFORM_FRAME_DOUBLE);
    // `FCOMP; TEST AH,1; JNZ`: on unless below or unordered.
    if matches!(
        elapsed.partial_cmp(&frame),
        Some(std::cmp::Ordering::Less) | None
    ) {
        return;
    }
    let pick = e.call(RANDOM_00487F50, &args![]).u32() % 3;
    let name = match pick {
        0 => WAVEFORM_FORWARD_TEXT,
        1 => WAVEFORM_BACKWARD_TEXT,
        _ => WAVEFORM_LEFT_TEXT,
    };
    e.with_stack(0x40, |e, text| {
        e.call(BUFFER_COPY_00406D30, &args![text, 0x40u32, name]);
        let tile_16 = map_tile(e, base, 16);
        e.call(TILE_SET_STRING, &args![tile_16, 0xfd2u32, text, 1u32]);
    });
    e.mem.set_u32(base + 0x88, 0.0f32.to_bits());
}

/// The world map's scale and offset (`01075010`, `01075008`) and their
/// inverse pair (`01075020`, `01075018`), `double`s.
const WORLD_MAP_SCALE_DOUBLE: u32 = 0x0107_5010;
const WORLD_MAP_OFFSET_DOUBLE: u32 = 0x0107_5008;
const WORLD_MAP_INVERSE_SCALE_DOUBLE: u32 = 0x0107_5020;
const WORLD_MAP_INVERSE_OFFSET_DOUBLE: u32 = 0x0107_5018;
/// Thiscall on the interface manager: the wheel delta (`0070ec70`).
const WHEEL_DELTA_0070EC70: u32 = 0x0070_ec70;
/// The `float` the wheel zooms by (`01056528`).
const WHEEL_ZOOM_FLOAT: u32 = 0x0105_6528;

/// `x - from` over `to - from` (`FLD; FSUB float; FLD; FSUB float;
/// FDIVP`), stored as a `float`.
fn map_fraction(value: u32, from: u32, to: u32) -> u32 {
    let numerator = x87_sub(x87_load32(value), x87_m32(from));
    let denominator = x87_sub(x87_load32(to), x87_m32(from));
    x87_store32(x87_div(numerator, denominator))
}

// Translated from 0079c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::WorldToMapCoord` (Xbox PDB): on page 0x21 with a world space
/// the point is adjusted first (`TESWorldSpace::AdjustMapMarkerCoord` with
/// 0, in place on the arguments); the result is the point's fraction of the
/// rectangle from +0xE8 to +0xF0 (x and y, z 0.0), on page 0x21 scaled and
/// offset by `01075010`/`01075008`. Returns `result`.
pub fn map_menu_world_to_map_coord(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    result: u32,
    x: u32,
    y: u32,
    z: u32,
    space: u32,
) -> u32 {
    let base = this.addr();
    let (x, y) = e.with_stack(0xc, |e, point| {
        e.mem.set_u32(point.addr(), x);
        e.mem.set_u32(point.addr() + 4, y);
        e.mem.set_u32(point.addr() + 8, z);
        // The code runs `006815c0` on its result local (a no-op).
        if space != 0 && e.get(this, MapMenu::iCurrentPage) == 0x21 {
            e.call(ADJUST_MAP_MARKER_COORD, &args![space, point, 0u32]);
        }
        (e.mem.u32(point.addr()), e.mem.u32(point.addr() + 4))
    });
    let mut map_x = map_fraction(x, e.mem.u32(base + 0xe8), e.mem.u32(base + 0xf0));
    let mut map_y = map_fraction(y, e.mem.u32(base + 0xec), e.mem.u32(base + 0xf4));
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let scale: f64 = e.global(WORLD_MAP_SCALE_DOUBLE);
        let offset: f64 = e.global(WORLD_MAP_OFFSET_DOUBLE);
        map_x = x87_store32(x87_add(x87_mul(x87_load32(map_x), scale), offset));
        map_y = x87_store32(x87_add(x87_mul(x87_load32(map_y), scale), offset));
    }
    e.mem.set_u32(result, map_x);
    e.mem.set_u32(result + 4, map_y);
    e.mem.set_u32(result + 8, 0.0f32.to_bits());
    result
}

// Translated from 0079c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::MapToWorldCoord` (Xbox PDB): the inverse of
/// `WorldToMapCoord`: on page 0x21 the map point is unscaled
/// (`01075020`/`01075018`), the world point is the rectangle from +0xE8 to
/// +0xF0 at that fraction (z 0.0), adjusted on page 0x21 with a world space
/// (`TESWorldSpace::AdjustMapMarkerCoord` with 1). `z` is copied but not
/// used. Returns `result`.
pub fn map_menu_map_to_world_coord(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    result: u32,
    x: u32,
    y: u32,
    _unused_3: u32,
    space: u32,
) -> u32 {
    let base = this.addr();
    let (mut x, mut y) = (x, y);
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let scale: f64 = e.global(WORLD_MAP_INVERSE_SCALE_DOUBLE);
        let offset: f64 = e.global(WORLD_MAP_INVERSE_OFFSET_DOUBLE);
        x = x87_store32(x87_sub(x87_mul(x87_load32(x), scale), offset));
        y = x87_store32(x87_sub(x87_mul(x87_load32(y), scale), offset));
    }
    let world = |from: u32, to: u32, fraction: u32| -> u32 {
        let span = x87_sub(x87_load32(to), x87_m32(from));
        x87_store32(x87_add(x87_mul(span, x87_m32(fraction)), x87_m32(from)))
    };
    let world_x = world(e.mem.u32(base + 0xe8), e.mem.u32(base + 0xf0), x);
    let world_y = world(e.mem.u32(base + 0xec), e.mem.u32(base + 0xf4), y);
    let point = e.with_stack(0xc, |e, point| {
        e.mem.set_u32(point.addr(), world_x);
        e.mem.set_u32(point.addr() + 4, world_y);
        e.mem.set_u32(point.addr() + 8, 0.0f32.to_bits());
        if space != 0 && e.get(this, MapMenu::iCurrentPage) == 0x21 {
            e.call(ADJUST_MAP_MARKER_COORD, &args![space, point, 1u32]);
        }
        [
            e.mem.u32(point.addr()),
            e.mem.u32(point.addr() + 4),
            e.mem.u32(point.addr() + 8),
        ]
    });
    for (index, word) in point.iter().enumerate() {
        e.mem.set_u32(result + 4 * index as u32, *word);
    }
    result
}

// Translated from 0079c530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DoWheelMove` (Xbox PDB): on the maps (pages 0x20 and 0x21),
/// zooms by `01056528` (`ZoomMap`), in when the interface manager's wheel
/// delta (`0070ec70`) over 120 is above 0. The two words are not read.
pub fn map_menu_do_wheel_move(e: &mut Engine, this: Ptr<MapMenu>, _unused_0: u32, _unused_1: u32) {
    if e.get(this, MapMenu::iCurrentPage) != 0x21 && e.get(this, MapMenu::iCurrentPage) != 0x20 {
        return;
    }
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let delta = e.call(WHEEL_DELTA_0070EC70, &args![interface]).i32();
    let steps = delta.wrapping_div(0x78);
    let zero: f64 = e.global(ZERO_DOUBLE);
    let zoom_in = above(f64::from(steps), zero);
    let factor = float_constant(e, WHEEL_ZOOM_FLOAT);
    map_menu_zoom_map(e, this, u8::from(zoom_in), f32::from_bits(factor));
}

/// The settings `ZoomMap` reads (each a float through `00403e20`): the
/// local map's magnification limits and the three traits' ranges, then the
/// world map's.
const LOCAL_ZOOM_MIN_SETTING: u32 = 0x011d_4c60;
const LOCAL_ZOOM_MAX_SETTING: u32 = 0x011d_2f64;
const LOCAL_ZOOM_TRAITS: [(u32, u32, u32); 3] = [
    (0x1004, 0x011d_4564, 0x011d_42b8),
    (0x1005, 0x011d_29d0, 0x011d_26c4),
    (0x1006, 0x011d_32e8, 0x011d_38f4),
];
const WORLD_ZOOM_MIN_SETTING: u32 = 0x011d_22f8;
const WORLD_ZOOM_MAX_SETTING: u32 = 0x011d_3d3c;
const WORLD_ZOOM_TRAITS: [(u32, u32, u32); 3] = [
    (0x1004, 0x011d_3ce8, 0x011d_3bbc),
    (0x1005, 0x011d_342c, 0x011d_3c58),
    (0x1006, 0x011d_4870, 0x011d_3df0),
];
/// The `double` the local map's cell size is divided by (`010240b8`, the
/// result unused) and the knob's zoom divisor (`01075028`).
const CELL_DIVISOR_DOUBLE: u32 = 0x0102_40b8;
const ZOOM_KNOB_DIVISOR_DOUBLE: u32 = 0x0107_5028;
/// The object `005f36f0` is asked on (`011dea10`): a non-zero answer
/// doubles the local map's cell size.
const OBJECT_011DEA10: u32 = 0x011d_ea10;
const OBJECT_CELL_005F36F0: u32 = 0x005f_36f0;
/// Thiscall on a cell tile's node (float), `RET 4`: its scale (`00440490`);
/// (x, y, z), `RET 0xC`: its translation (`004bc1f0`); (matrix), `RET 4`:
/// its rotation (`00a59c60`); and its geometry data (`005495f0`) whose
/// `NiGeometryData::SetConsistency` gets 0x4000.
const NODE_SET_SCALE_00440490: u32 = 0x0044_0490;
const NODE_SET_TRANSLATE_004BC1F0: u32 = 0x004b_c1f0;
const NODE_SET_ROTATE_00A59C60: u32 = 0x00a5_9c60;
const NODE_GEOMETRY_DATA_005495F0: u32 = 0x0054_95f0;
const GEOMETRY_SET_CONSISTENCY: u32 = 0x00a6_7050;
/// Thiscall on a matrix (float, word, word), `RET 0xC` (`0043d410`).
const MATRIX_SET_0043D410: u32 = 0x0043_d410;

/// The float a setting holds (`00403e20`), loaded (a signalling NaN
/// quieted).
fn setting_float(e: &mut Engine, setting: u32) -> u32 {
    let address = e.call(OBJECT_VALUE_ADDRESS_00403E20, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(address))
}

/// `ZoomMap`'s clamp of the magnification and the three traits that follow
/// it: the magnification `0040ebd0(mag, max)` then `00404010(.., min)`, its
/// fraction of the range, and each trait set to `low + (high - low) *
/// fraction` (the settings read in the code's order). Returns the clamped
/// magnification.
fn zoom_traits(
    e: &mut Engine,
    map: u32,
    magnification: u32,
    limits: (u32, u32),
    traits: &[(u32, u32, u32); 3],
) -> u32 {
    let (min_setting, max_setting) = limits;
    let low = setting_float(e, min_setting);
    let high = setting_float(e, max_setting);
    let clamped = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(magnification), high],
        )
        .f64();
    let clamped = x87_quiet_bits(x87_store32(clamped));
    let clamped = e.call(FLOAT_PAIR_00404010, &args![clamped, low]).f64();
    let magnification = x87_store32(clamped);
    let low = setting_float(e, min_setting);
    let above_low = x87_sub(x87_load32(magnification), x87_m32(low));
    let high = setting_float(e, max_setting);
    let low = setting_float(e, min_setting);
    let span = x87_sub(x87_load32(high), x87_m32(low));
    let fraction = x87_store32(x87_div(above_low, span));
    for &(id, from, to) in traits {
        let start = x87_load32(setting_float(e, from));
        let end = x87_load32(setting_float(e, to));
        let from_again = setting_float(e, from);
        let value = x87_sub(end, x87_load32(from_again));
        let value = x87_add(x87_mul(value, x87_m32(fraction)), start);
        let value = x87_quiet_bits(x87_store32(value));
        e.call(TILE_SET_FLOAT, &args![map, id, value, 1u32]);
    }
    magnification
}

/// `ZoomMap`'s local map cells (page 0x20): every cell of the map tile's
/// object (`0056c7f0`, a grid of `n * n`, `n` the word at `0043d4d0` on
/// `011c63cc`) is scaled to the magnification and placed at its column and
/// row (cell size 0x400, 0x800 when `005f36f0` on `011dea10` is set, times
/// the magnification), with a zero rotation, and its geometry marked.
fn zoom_cell_grid(e: &mut Engine, map: u32, magnification: u32) {
    let object = e.call(TILE_OBJECT_0056C7F0, &args![map]).u32();
    let side = e.call(WORD_POINTER_0043D4D0, &args![OBJECT_011C63CC]).u32();
    let side = e.mem.u32(side);
    let two: f64 = e.global(TWO_DOUBLE);
    let mut index = 0u32;
    while object != 0 && index < side.wrapping_mul(side) {
        let cell = e.call(OBJECT_ITEM_0043B4A0, &args![object, index]).u32();
        if cell != 0 {
            let owner = e.mem.u32(OBJECT_011DEA10);
            let interior = e.call(OBJECT_CELL_005F36F0, &args![owner]).u32() != 0;
            let size: i32 = if interior { 0x800 } else { 0x400 };
            let cell_size = x87_store32(x87_mul(f64::from(size), x87_m32(magnification)));
            // The code also stores the cell size over `010240b8` in a local it
            // never reads.
            let divisor: f64 = e.global(CELL_DIVISOR_DOUBLE);
            let _unused = x87_store32(x87_div(x87_load32(cell_size), divisor));
            e.call(
                NODE_SET_SCALE_00440490,
                &args![cell, x87_quiet_bits(magnification)],
            );
            let row = index / side;
            let column = index % side;
            let half = x87_div(x87_load32(cell_size), two);
            let y = x87_add(x87_mul(f64::from(row), x87_m32(cell_size)), half);
            let y = x87_sub(y, x87_mul(f64::from(side), x87_m32(cell_size)));
            let y = x87_quiet_bits(x87_store32(y));
            let half = x87_div(x87_load32(cell_size), two);
            let x = x87_add(x87_mul(f64::from(column), x87_m32(cell_size)), half);
            let x = x87_quiet_bits(x87_store32(x));
            e.call(
                NODE_SET_TRANSLATE_004BC1F0,
                &args![cell, x, 0.0f32.to_bits(), y],
            );
            e.with_stack(0x24, |e, matrix| {
                e.call(
                    MATRIX_SET_0043D410,
                    &args![matrix, 0.0f32.to_bits(), 0u32, 0u32],
                );
                e.call(NODE_SET_ROTATE_00A59C60, &args![cell, matrix]);
            });
            let data = e.call(NODE_GEOMETRY_DATA_005495F0, &args![cell]).u32();
            e.call(GEOMETRY_SET_CONSISTENCY, &args![data, 0x4000u32]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0079c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::ZoomMap` (Xbox PDB): with a map tile (and on page 0x20 only
/// when `007a1750` is false), the magnification (`iUT_Magnification`) is
/// multiplied by `factor` (zooming in) or divided by it, clamped to the
/// page's limits with the traits `0x1004` to `0x1006` following it (see
/// [`zoom_traits`]); the point under the map's centre is kept in place
/// (traits `0xfa1`/`0xfa2` from the owner's half size, the scaled point and
/// `0xfee`/`0xfef`); on page 0x20 the local map cells follow (see
/// [`zoom_cell_grid`]). When the magnification changed the pip-boy knob
/// turns by `(factor - 1.0) / 01075028` steps, negative zooming in. Then
/// `UpdateCompanionMarkers`.
pub fn map_menu_zoom_map(e: &mut Engine, this: Ptr<MapMenu>, zoom_in: u8, factor: f32) {
    if e.get(this, MapMenu::pMapTile) == 0 {
        return;
    }
    if e.get(this, MapMenu::iCurrentPage) == 0x20 && fn_007a1750(e) {
        return;
    }
    let factor = factor.to_bits();
    let scale = if zoom_in != 0 {
        x87_load32(factor)
    } else {
        x87_div(1.0, x87_load32(factor))
    };
    let map = e.get(this, MapMenu::pMapTile);
    let magnification_id = e.mem.u32(TRAIT_MAGNIFICATION);
    let current = e.call(TILE_GET_FLOAT, &args![map, magnification_id]).f64();
    let mut magnification = x87_store32(x87_mul(current, scale));
    let two: f64 = e.global(TWO_DOUBLE);
    let point = e.with_stack(8, |e, point| {
        let map = e.get(this, MapMenu::pMapTile);
        let owner = e.call(WORD_AT_28, &args![map]).u32();
        let height = e.call(TILE_GET_FLOAT, &args![owner, 0xfb0u32]).f64();
        let map = e.get(this, MapMenu::pMapTile);
        let top = e.call(TILE_GET_FLOAT, &args![map, 0xfa2u32]).f64();
        let y = x87_quiet_bits(x87_store32(x87_sub(x87_div(height, two), top)));
        let map = e.get(this, MapMenu::pMapTile);
        let owner = e.call(WORD_AT_28, &args![map]).u32();
        let width = e.call(TILE_GET_FLOAT, &args![owner, 0xfb1u32]).f64();
        let map = e.get(this, MapMenu::pMapTile);
        let left = e.call(TILE_GET_FLOAT, &args![map, 0xfa1u32]).f64();
        let x = x87_quiet_bits(x87_store32(x87_sub(x87_div(width, two), left)));
        e.call(POINT2_SET_00452DC0, &args![point, x, y]);
        let map = e.get(this, MapMenu::pMapTile);
        magnification = if e.get(this, MapMenu::iCurrentPage) == 0x20 {
            zoom_traits(
                e,
                map,
                magnification,
                (LOCAL_ZOOM_MIN_SETTING, LOCAL_ZOOM_MAX_SETTING),
                &LOCAL_ZOOM_TRAITS,
            )
        } else {
            zoom_traits(
                e,
                map,
                magnification,
                (WORLD_ZOOM_MIN_SETTING, WORLD_ZOOM_MAX_SETTING),
                &WORLD_ZOOM_TRAITS,
            )
        };
        let map = e.get(this, MapMenu::pMapTile);
        let magnification_id = e.mem.u32(TRAIT_MAGNIFICATION);
        let current = e.call(TILE_GET_FLOAT, &args![map, magnification_id]).f64();
        let changed = !equal(x87_load32(magnification), current);
        let map = e.get(this, MapMenu::pMapTile);
        let size = e.call(TILE_GET_FLOAT, &args![map, 0xfcdu32]).f64();
        let size = x87_mul(size, x87_m32(magnification));
        let map = e.get(this, MapMenu::pMapTile);
        let width = e.call(TILE_GET_FLOAT, &args![map, 0xfb1u32]).f64();
        let ratio = x87_quiet_bits(x87_store32(x87_div(size, width)));
        e.call(POINT2_SCALE_006AD180, &args![point, ratio]);
        let map = e.get(this, MapMenu::pMapTile);
        let magnification_id = e.mem.u32(TRAIT_MAGNIFICATION);
        e.call(
            TILE_SET_FLOAT,
            &args![map, magnification_id, x87_quiet_bits(magnification), 1u32],
        );
        if e.get(this, MapMenu::iCurrentPage) == 0x20 {
            let map = e.get(this, MapMenu::pMapTile);
            zoom_cell_grid(e, map, magnification);
        }
        (
            changed,
            e.mem.u32(point.addr()),
            e.mem.u32(point.addr() + 4),
        )
    });
    let (changed, point_x, point_y) = point;
    for (size_id, along, offset_id, target) in [
        (0xfb1u32, point_x, 0xfeeu32, 0xfa1u32),
        (0xfb0, point_y, 0xfef, 0xfa2),
    ] {
        let map = e.get(this, MapMenu::pMapTile);
        let owner = e.call(WORD_AT_28, &args![map]).u32();
        let size = e.call(TILE_GET_FLOAT, &args![owner, size_id]).f64();
        let centre = x87_sub(x87_div(size, two), x87_m32(along));
        let map = e.get(this, MapMenu::pMapTile);
        let offset = e.call(TILE_GET_FLOAT, &args![map, offset_id]).f64();
        let value = x87_quiet_bits(x87_store32(x87_sub(centre, offset)));
        let map = e.get(this, MapMenu::pMapTile);
        e.call(TILE_SET_FLOAT, &args![map, target, value, 0u32]);
    }
    if changed {
        let one: f64 = e.global(ONE_DOUBLE);
        let divisor: f64 = e.global(ZOOM_KNOB_DIVISOR_DOUBLE);
        let steps = x87_store32(x87_div(x87_sub(x87_load32(factor), one), divisor));
        let step = setting_float(e, KNOB_STEP_SETTING);
        // `FCHS` flips the sign bit.
        let step = if zoom_in != 0 {
            step ^ 0x8000_0000
        } else {
            step
        };
        let turn = x87_mul(x87_load32(step), x87_m32(steps));
        let time = x87_load32(setting_float(e, KNOB_TIME_SETTING));
        let time = x87_quiet_bits(x87_store32(x87_add(time, time)));
        let turn = x87_quiet_bits(x87_store32(turn));
        let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
        let pipboy = e.call(WORD_AT_174, &args![interface]).u32();
        e.call(PIPBOY_MOVE_KNOB, &args![pipboy, 0u32, turn, time, 1u32]);
    }
    map_menu_update_companion_markers(e, this);
}

/// `TESWorldSpace::GetWorldMapData` (engine map), thiscall: the map data
/// (usable size words at +0 and +4, the cell corners as 16-bit words at +8,
/// +0xA, +0xC, +0xE).
const WORLD_GET_WORLD_MAP_DATA: u32 = 0x0058_6400;
/// Cdecl (form id): a form (`004839c0`); `__RTDynamicCast` (engine map),
/// cdecl (object, word, source type, target type, word); and the two type
/// descriptors `UpdateWorldMap` passes (`01183028`, `01183fd0`).
const FORM_BY_ID_004839C0: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const SOURCE_TYPE_01183028: u32 = 0x0118_3028;
const TARGET_TYPE_01183FD0: u32 = 0x0118_3fd0;
/// `TESWorldSpace::BuildMapMarkerList` (engine map), thiscall (word),
/// `RET 4`.
const WORLD_BUILD_MAP_MARKER_LIST: u32 = 0x0058_82a0;
/// `MapMarkerData::GetVisible` (engine map), thiscall: `AL`; and the icon
/// kind (`00438f30`) that indexes the icon paths at `011a0404`.
const MARKER_GET_VISIBLE: u32 = 0x0043_8ed0;
const MARKER_ICON_KIND_00438F30: u32 = 0x0043_8f30;
const MARKER_ICONS_011A0404: u32 = 0x011a_0404;
/// `TESReputation::GetReputationTitle` (engine map), thiscall.
const REPUTATION_GET_TITLE: u32 = 0x0061_6710;
/// `"MapMarkerTemplate"`, `"%s\n(%s)"` and the undiscovered icon path.
const MAP_MARKER_TEMPLATE: u32 = 0x0107_506c;
const MARKER_REPUTATION_FORMAT: u32 = 0x0107_5064;
const UNDISCOVERED_ICON_PATH: u32 = 0x0107_5030;
/// The running marker index `UpdateWorldMap` gives (`011da370`).
const MARKER_INDEX_011DA370: u32 = 0x011d_a370;

/// The world map data of `space` (`TESWorldSpace::GetWorldMapData`), 0
/// without a space.
fn world_map_data(e: &mut Engine, space: u32) -> u32 {
    if space == 0 {
        return 0;
    }
    e.call(WORLD_GET_WORLD_MAP_DATA, &args![space]).u32()
}

/// Whether the root space and its world map data's usable size are set
/// (`data`, `[data]` and `[data + 4]` all non-zero).
fn world_map_ready(e: &Engine, root: u32, data: u32) -> bool {
    root != 0 && data != 0 && e.mem.u32(data) != 0 && e.mem.u32(data + 4) != 0
}

/// A map corner from two cell coordinates in world units: an `NiPoint2`
/// (`00452dc0`) built in a stack slot, its two words returned.
fn map_corner(e: &mut Engine, x: i32, y: i32) -> (u32, u32) {
    e.with_stack(8, |e, corner| {
        let x = x87_store32(f64::from(x));
        let y = x87_store32(f64::from(y));
        let made = e.call(POINT2_SET_00452DC0, &args![corner, x, y]).u32();
        (e.mem.u32(made), e.mem.u32(made + 4))
    })
}

/// A 16-bit cell coordinate of the world map data (`MOVSX; SHL 0xC`).
fn map_cell(e: &Engine, data: u32, offset: u32) -> i32 {
    i32::from(e.mem.u16(data + offset) as i16) << 0xc
}

// Translated from 0079cdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079cdb0 - the /GS security cookie check (00ec408c) is not translated.
/// `MapMenu::UpdateWorldMap` (Xbox PDB): without a usable root space map
/// (`TESWorldSpace::GetWorldMapData`), takes form 0x3C cast to a world space
/// (`__RTDynamicCast`), then the first world space of `[011c3f2c] + 0x10`
/// (each marking `bRootSpaceChanged`); the root becomes `pMapSpace`,
/// `pAdjustNorthRotationCell` is cleared. With map data: a cleared texture
/// (`011da36c`) or a changed root puts the root's texture and usable size
/// into tile 4 (`0xfcc`, `0xfcd`, `0xfce`); the map zooms by 1.0; the map
/// rectangle (+0xE8, +0xF0) is set from the data's cells (4096 units each);
/// unless tile 4 has a child 0x1A, every visible marker of the root (see
/// [`add_world_map_marker`]) gets a tile; the built list is emptied and
/// deleted. Then `AddMapIcons`.
pub fn map_menu_update_world_map(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let root = e.get(this, MapMenu::pRootMapSpace);
    let mut data = world_map_data(e, root);
    let root = e.get(this, MapMenu::pRootMapSpace);
    if !world_map_ready(e, root, data) {
        let form = e.call(FORM_BY_ID_004839C0, &args![0x3cu32]).u32();
        let space = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, SOURCE_TYPE_01183028, TARGET_TYPE_01183FD0, 0u32],
            )
            .u32();
        e.set(this, MapMenu::pRootMapSpace, space);
        let root = e.get(this, MapMenu::pRootMapSpace);
        data = world_map_data(e, root);
        e.set(this, MapMenu::bRootSpaceChanged, 1);
    }
    let root = e.get(this, MapMenu::pRootMapSpace);
    if !world_map_ready(e, root, data) {
        let owner = e.mem.u32(MARKER_OWNER_011C3F2C);
        let head = e.call(PLUS_10, &args![owner]).u32();
        let first = node_item(e, head);
        e.set(this, MapMenu::pRootMapSpace, first);
        let root = e.get(this, MapMenu::pRootMapSpace);
        data = world_map_data(e, root);
        e.set(this, MapMenu::bRootSpaceChanged, 1);
    }
    let root = e.get(this, MapMenu::pRootMapSpace);
    e.set(this, MapMenu::pMapSpace, root);
    e.set(this, MapMenu::pAdjustNorthRotationCell, 0);
    if e.get(this, MapMenu::pRootMapSpace) == 0 || data == 0 {
        return;
    }
    if e.mem.u8(TEXTURE_CLEARED_011DA36C) != 0 || e.get(this, MapMenu::bRootSpaceChanged) != 0 {
        let root = e.get(this, MapMenu::pRootMapSpace);
        let texture = e.call(FORM_NAME_00586440, &args![root]).u32();
        let tile_4 = map_tile(e, base, 4);
        e.call(TILE_SET_STRING, &args![tile_4, 0xfccu32, texture, 1u32]);
        let width = e.mem.u32(data);
        let tile_4 = map_tile(e, base, 4);
        e.call(TILE_SET_COUNT_00715C60, &args![tile_4, 0xfcdu32, width]);
        let height = e.mem.u32(data + 4);
        let tile_4 = map_tile(e, base, 4);
        e.call(TILE_SET_COUNT_00715C60, &args![tile_4, 0xfceu32, height]);
        e.mem.set_u8(TEXTURE_CLEARED_011DA36C, 0);
        e.set(this, MapMenu::bRootSpaceChanged, 0);
    }
    map_menu_zoom_map(e, this, 1, 1.0);
    let y = map_cell(e, data, 0xa).wrapping_add(0x1000);
    let x = map_cell(e, data, 8);
    let (x, y) = map_corner(e, x, y);
    e.mem.set_u32(base + 0xe8, x);
    e.mem.set_u32(base + 0xec, y);
    let y = map_cell(e, data, 0xe);
    let x = map_cell(e, data, 0xc).wrapping_add(0x1000);
    let (x, y) = map_corner(e, x, y);
    e.mem.set_u32(base + 0xf0, x);
    e.mem.set_u32(base + 0xf4, y);
    let tile_4 = map_tile(e, base, 4);
    if e.call(TILE_GET_CHILD_BY_ID, &args![tile_4, 0x1au32]).u32() == 0 {
        let root = e.get(this, MapMenu::pRootMapSpace);
        let list = e
            .call(WORLD_BUILD_MAP_MARKER_LIST, &args![root, 0u32])
            .u32();
        e.mem.set_u32(MARKER_INDEX_011DA370, 0);
        let mut node = list;
        while node != 0 && node_item(e, node) != 0 {
            let marker = node_item(e, node);
            add_world_map_marker(e, this, marker);
            node = e.call(WORD_AT_4, &args![node]).u32();
        }
        e.call(LIST_REMOVE_ALL, &args![list]);
        if list != 0 {
            e.call(NODE_DELETE, &args![list, 1u32]);
        }
    }
    map_menu_add_map_icons(e, this);
}

/// One marker of `UpdateWorldMap`'s list: a visible marker (not
/// `00440da0`) whose top world space is the root gets a
/// `"MapMarkerTemplate"` tile (`Menu::RenderTemplate` on `pMe`, under tile
/// 4) with its name (with its reputation title, `"%s\n(%s)"`, when it has a
/// reputation) as `"_LocationName"`, `00707e10` with the name, trait `0xfaa`
/// 0x1A, its map position (`WorldToMapCoord`), its icon (by `00438f30` from
/// `011a0404`) or the undiscovered icon with trait `0x1004` 0, the running
/// index as `"_MarkerIndex"`, and is added to the marker list (+0xD8).
fn add_world_map_marker(e: &mut Engine, this: Ptr<MapMenu>, marker: u32) {
    let base = this.addr();
    let position = e.vcall(marker, 0x1f4, &args![]).u32();
    let x = e.mem.u32(position);
    let y = e.mem.u32(position + 4);
    let z = e.mem.u32(position + 8);
    let mut space = e.call(REFR_GET_WORLD_SPACE, &args![marker]).u32();
    while space != 0 && e.call(WORLD_GET_PARENT, &args![space]).u32() != 0 {
        space = e.call(WORLD_GET_PARENT, &args![space]).u32();
    }
    let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
    if !e.call(MARKER_GET_VISIBLE, &args![data]).bool()
        || e.call(FORM_FLAG_00440DA0, &args![marker]).bool()
    {
        return;
    }
    if space != e.get(this, MapMenu::pRootMapSpace) {
        return;
    }
    let tile_4 = map_tile(e, base, 4);
    let menu = map_menu(e);
    let tile = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu, tile_4, MAP_MARKER_TEMPLATE, 0u32],
        )
        .u32();
    if tile == 0 {
        return;
    }
    let (map_x, map_y) = e.with_stack(0xc, |e, map_point| {
        let space = e.call(REFR_GET_WORLD_SPACE, &args![marker]).u32();
        map_menu_world_to_map_coord(e, this, map_point.addr(), x, y, z, space);
        (e.mem.u32(map_point.addr()), e.mem.u32(map_point.addr() + 4))
    });
    let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
    let reputation = e.call(PROCESS_VALUE_0044EDB0, &args![data]).u32();
    if reputation != 0 {
        let title = e.call(REPUTATION_GET_TITLE, &args![reputation]).u32();
        let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
        let name = e.call(MARKER_NAME_00438EB0, &args![data]).u32();
        e.with_stack(0x100, |e, text| {
            e.call(
                BUFFER_FORMAT,
                &args![text, 0x100u32, MARKER_REPUTATION_FORMAT, name, title],
            );
            let name_id = e
                .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
                .u32();
            e.call(TILE_SET_STRING, &args![tile, name_id, text, 1u32]);
        });
    } else {
        let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
        let name = e.call(MARKER_NAME_00438EB0, &args![data]).u32();
        let name_id = e
            .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
            .u32();
        e.call(TILE_SET_STRING, &args![tile, name_id, name, 1u32]);
    }
    let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
    let name = e.call(MARKER_NAME_00438EB0, &args![data]).u32();
    e.call(TILE_TEXT_SETTER_00707E10, &args![tile, name]);
    e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x1au32]);
    let x_id = e.mem.u32(TRAIT_X);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, x_id, x87_quiet_bits(map_x), 1u32],
    );
    let y_id = e.mem.u32(TRAIT_Y);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, y_id, x87_quiet_bits(map_y), 1u32],
    );
    let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
    if e.call(MARKER_GET_TRAVEL_LOC, &args![data]).bool() {
        let data = e.call(REFR_GET_MAP_MARKER_DATA, &args![marker]).u32();
        let kind = e.call(MARKER_ICON_KIND_00438F30, &args![data]).u32();
        let icon = e
            .mem
            .u32(MARKER_ICONS_011A0404.wrapping_add(kind.wrapping_mul(4)));
        e.call(TILE_SET_STRING, &args![tile, 0xfccu32, icon, 1u32]);
    } else {
        e.call(
            TILE_SET_STRING,
            &args![tile, 0xfccu32, UNDISCOVERED_ICON_PATH, 1u32],
        );
        e.call(TILE_SET_INT, &args![tile, 0x1004u32, 0u32]);
    }
    let index = e.mem.u32(MARKER_INDEX_011DA370);
    let index_id = e
        .call(ADD_USER_TRAIT, &args![MARKER_INDEX_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(TILE_SET_INT, &args![tile, index_id, index]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), marker);
        e.call(LIST_ADD_TAIL_00905820, &args![base + 0xd8, slot]);
    });
    let index = e.mem.u32(MARKER_INDEX_011DA370).wrapping_add(1);
    e.mem.set_u32(MARKER_INDEX_011DA370, index);
}

/// Thiscall on the player: its current cell (`008d6f30`).
const PLAYER_CELL_008D6F30: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetInterior` (engine map), thiscall: `AL`.
const REFR_GET_INTERIOR: u32 = 0x0057_5d10;
/// Cdecl (float): an integer (`00406d90`).
const FLOAT_TO_INT_00406D90: u32 = 0x0040_6d90;
/// The `double` added to the local map's corners (`01017a10`, 4096.0 in the
/// shipped exe) and the fog divisor (`0101db80`).
const CELL_SIZE_DOUBLE: u32 = 0x0101_7a10;
const FOG_DIVISOR_DOUBLE: u32 = 0x0101_db80;
/// Thiscall on tile 2's +4: `AL` (`0076b610`; the local map's door markers
/// are added when set).
const LOCAL_MAP_READY_0076B610: u32 = 0x0076_b610;
/// Thiscall on a reference: `AL`s that skip it as a door marker
/// (`00477ba0`, `0057b4b0`), and on its base door (`00517fd0`).
const REFERENCE_FLAG_00477BA0: u32 = 0x0047_7ba0;
const REFERENCE_FLAG_0057B4B0: u32 = 0x0057_b4b0;
const DOOR_FLAG_00517FD0: u32 = 0x0051_7fd0;
/// `TESObjectCELL::GetSeenValue` (engine map), cdecl (point): the fog value.
const CELL_GET_SEEN_VALUE: u32 = 0x0055_6870;
/// Thiscall on a reference: its teleport data (`00568e50`), and
/// `DoorTeleportData::GetTeleportCellMapName` (engine map), thiscall
/// (string), `RET 4`.
const REFERENCE_TELEPORT_00568E50: u32 = 0x0056_8e50;
const TELEPORT_GET_CELL_MAP_NAME: u32 = 0x0043_a2e0;
/// The local map's door icon path (`01075334`).
const DOOR_ICON_PATH: u32 = 0x0107_5334;

/// A point (`00416870`, x, y, z as integers stored as floats) built in a
/// stack slot, its three words returned.
fn set_point(e: &mut Engine, x: i32, y: i32) -> [u32; 3] {
    e.with_stack(0xc, |e, point| {
        let x = x87_store32(f64::from(x));
        let y = x87_store32(f64::from(y));
        e.call(POINT_SET_00416870, &args![point, x, y, 0.0f32.to_bits()]);
        [
            e.mem.u32(point.addr()),
            e.mem.u32(point.addr() + 4),
            e.mem.u32(point.addr() + 8),
        ]
    })
}

/// The local map grid's side (the word `0043d4d0` returns on `011c63cc`).
fn local_grid_side(e: &mut Engine) -> i32 {
    let side = e.call(WORD_POINTER_0043D4D0, &args![OBJECT_011C63CC]).u32();
    e.mem.u32(side) as i32
}

/// The grid cell of a coordinate: `(00406d90(value) - offset) >> 12` minus
/// half the grid side (arithmetic shifts).
fn local_grid_cell(e: &mut Engine, value: u32, offset: i32) -> i32 {
    let whole = e
        .call(FLOAT_TO_INT_00406D90, &args![x87_quiet_bits(value)])
        .i32();
    let cell = whole.wrapping_sub(offset) >> 0xc;
    cell.wrapping_sub(local_grid_side(e) >> 1)
}

// Translated from 0079d410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079d410 - the C++ exception frame (FS:[0] chain, unwind state 0 around the door name string) is not translated.
/// `MapMenu::UpdateLocalMap` (Xbox PDB): around the player's position (in
/// an interior, `pAdjustNorthRotationCell` is the player's cell and the
/// position is rotated to north with cells offset by 0x800), the map
/// rectangle (+0xE8, +0xF0) covers the grid of cells centred on the
/// player's; the map zooms by 1.0; `pMapSpace` is the world space, else the
/// cell. With a map space: when tile 2's `0076b610` is set, every door
/// marker of the list at +0xE0 (a door base, not `00477ba0`/`0057b4b0`/
/// `00517fd0`) visible through the fog (`GetSeenValue` / `0101db80` times
/// 255.0 when `bFogOfWarEnabled`) and inside the map gets a
/// `"MapMarkerTemplate"` tile (position, alpha `0xfa9`, door icon, and the
/// teleport cell's map name as `"_LocationName"`); then `AddMapIcons`.
/// Tiles 2 and 3 get trait `0xfa3` 1.
pub fn map_menu_update_local_map(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let player = e.mem.u32(PLAYER_011DEA3C);
    let space = e.call(REFR_GET_WORLD_SPACE, &args![player]).u32();
    let player = e.mem.u32(PLAYER_011DEA3C);
    let cell = e.call(PLAYER_CELL_008D6F30, &args![player]).u32();
    let player = e.mem.u32(PLAYER_011DEA3C);
    let position = e.vcall(player, 0x1f4, &args![]).u32();
    let position = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    let player = e.mem.u32(PLAYER_011DEA3C);
    let interior = e.call(REFR_GET_INTERIOR, &args![player]).bool();
    e.set(
        this,
        MapMenu::pAdjustNorthRotationCell,
        if interior { cell } else { 0 },
    );
    let one_cell: f64 = e.global(CELL_SIZE_DOUBLE);
    let (first, second) = if e.get(this, MapMenu::pAdjustNorthRotationCell) != 0 {
        let rotated = e.with_stack(0x18, |e, points| {
            let original = points.addr();
            let rotated = original + 0xc;
            for (index, word) in position.iter().enumerate() {
                e.mem.set_u32(original + 4 * index as u32, *word);
                e.mem.set_u32(rotated + 4 * index as u32, *word);
            }
            let north = e.get(this, MapMenu::pAdjustNorthRotationCell);
            e.call(
                ADJUST_FOR_NORTH_ROTATION,
                &args![north, original, rotated, 1u32],
            );
            [e.mem.u32(rotated), e.mem.u32(rotated + 4)]
        });
        let column = local_grid_cell(e, rotated[0], 0x800);
        let row = local_grid_cell(e, rotated[1], 0x800);
        let side = local_grid_side(e);
        let top = set_point(
            e,
            (column << 0xc).wrapping_add(0x800),
            (row.wrapping_add(side).wrapping_sub(1) << 0xc).wrapping_add(0x800),
        );
        let side = local_grid_side(e);
        let bottom = set_point(
            e,
            (column.wrapping_add(side).wrapping_sub(1) << 0xc).wrapping_add(0x800),
            (row << 0xc).wrapping_add(0x800),
        );
        (top, bottom)
    } else {
        let column = local_grid_cell(e, position[0], 0);
        let row = local_grid_cell(e, position[1], 0);
        let side = local_grid_side(e);
        let top = set_point(
            e,
            column << 0xc,
            row.wrapping_add(side).wrapping_sub(1) << 0xc,
        );
        let side = local_grid_side(e);
        let bottom = set_point(
            e,
            column.wrapping_add(side).wrapping_sub(1) << 0xc,
            row << 0xc,
        );
        (top, bottom)
    };
    let y = x87_quiet_bits(x87_store32(x87_add(x87_load32(first[1]), one_cell)));
    let (x, y) = map_corner_bits(e, x87_quiet_bits(first[0]), y);
    e.mem.set_u32(base + 0xe8, x);
    e.mem.set_u32(base + 0xec, y);
    let x = x87_quiet_bits(x87_store32(x87_add(x87_load32(second[0]), one_cell)));
    let (x, y) = map_corner_bits(e, x, x87_quiet_bits(second[1]));
    e.mem.set_u32(base + 0xf0, x);
    e.mem.set_u32(base + 0xf4, y);
    map_menu_zoom_map(e, this, 1, 1.0);
    e.set(
        this,
        MapMenu::pMapSpace,
        if space != 0 { space } else { cell },
    );
    if e.get(this, MapMenu::pMapSpace) != 0 {
        let tile_2 = map_tile(e, base, 2);
        if e.call(LOCAL_MAP_READY_0076B610, &args![tile_2 + 4]).bool() {
            let mut node = base + 0xe0;
            while node != 0 && node_item(e, node) != 0 {
                let marker = node_item(e, node);
                add_door_marker(e, this, marker);
                node = e.call(WORD_AT_4, &args![node]).u32();
            }
        }
        map_menu_add_map_icons(e, this);
    }
    let tile_2 = map_tile(e, base, 2);
    e.call(TILE_SET_INT, &args![tile_2, 0xfa3u32, 1u32]);
    let tile_3 = map_tile(e, base, 3);
    e.call(TILE_SET_INT, &args![tile_3, 0xfa3u32, 1u32]);
}

/// [`map_corner`] for float bit patterns already computed.
fn map_corner_bits(e: &mut Engine, x: u32, y: u32) -> (u32, u32) {
    e.with_stack(8, |e, corner| {
        let made = e.call(POINT2_SET_00452DC0, &args![corner, x, y]).u32();
        (e.mem.u32(made), e.mem.u32(made + 4))
    })
}

/// One reference of `UpdateLocalMap`'s marker list (see there).
fn add_door_marker(e: &mut Engine, this: Ptr<MapMenu>, reference: u32) {
    let base = this.addr();
    let object = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![reference]).u32();
    let door = if e.call(FORM_TYPE, &args![object]).u32() == 0x1c {
        e.call(SAVE_FORM_BUFFER_GET_FORM, &args![reference]).u32()
    } else {
        0
    };
    let position = e.vcall(reference, 0x1f4, &args![]).u32();
    let position = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    if e.call(REFERENCE_FLAG_00477BA0, &args![reference]).bool()
        || e.call(REFERENCE_FLAG_0057B4B0, &args![reference]).bool()
        || door == 0
        || e.call(DOOR_FLAG_00517FD0, &args![door]).bool()
    {
        return;
    }
    e.with_stack(0x18, |e, frame| {
        // The frame's locals: the position (+0) and the map point (+0xC).
        let point = frame.addr();
        let map_point = point + 0xc;
        for (index, word) in position.iter().enumerate() {
            e.mem.set_u32(point + 4 * index as u32, *word);
        }
        let north = e.get(this, MapMenu::pAdjustNorthRotationCell);
        if north != 0 {
            e.call(ADJUST_FOR_NORTH_ROTATION, &args![north, point, point, 1u32]);
        }
        let mut alpha = float_constant(e, FULL_ALPHA_FLOAT);
        let x = e.mem.u32(point);
        let y = e.mem.u32(point + 4);
        let z = e.mem.u32(point + 8);
        map_menu_world_to_map_coord(e, this, map_point, x, y, z, 0);
        if e.get(this, MapMenu::bFogOfWarEnabled) != 0 {
            let seen = e.call(CELL_GET_SEEN_VALUE, &args![point]).i32();
            let divisor: f64 = e.global(FOG_DIVISOR_DOUBLE);
            let value = x87_mul(x87_div(f64::from(seen), divisor), x87_m32(alpha));
            alpha = x87_store32(value);
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        let one: f64 = e.global(ONE_DOUBLE);
        let map_x = x87_load32(e.mem.u32(map_point));
        let map_y = x87_load32(e.mem.u32(map_point + 4));
        // `TEST AH,1; JNZ` skips below 0.0 (or unordered); `TEST AH,0x41;
        // JP` skips above 1.0 (or unordered).
        let inside = |value: f64| {
            !matches!(
                value.partial_cmp(&zero),
                Some(std::cmp::Ordering::Less) | None
            ) && !matches!(
                value.partial_cmp(&one),
                Some(std::cmp::Ordering::Greater) | None
            )
        };
        if !above(x87_load32(alpha), zero) || !inside(map_x) || !inside(map_y) {
            return;
        }
        let tile_2 = map_tile(e, base, 2);
        let menu = map_menu(e);
        let tile = e
            .call(
                MENU_RENDER_TEMPLATE,
                &args![menu, tile_2, MAP_MARKER_TEMPLATE, 0u32],
            )
            .u32();
        let teleport = e.call(REFERENCE_TELEPORT_00568E50, &args![reference]).u32();
        e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x1au32]);
        let x_id = e.mem.u32(TRAIT_X);
        let map_x = x87_quiet_bits(e.mem.u32(map_point));
        e.call(TILE_SET_FLOAT, &args![tile, x_id, map_x, 1u32]);
        let y_id = e.mem.u32(TRAIT_Y);
        let map_y = x87_quiet_bits(e.mem.u32(map_point + 4));
        e.call(TILE_SET_FLOAT, &args![tile, y_id, map_y, 1u32]);
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfa9u32, x87_quiet_bits(alpha), 1u32],
        );
        e.call(
            TILE_SET_STRING,
            &args![tile, 0xfccu32, DOOR_ICON_PATH, 1u32],
        );
        if teleport != 0 {
            e.with_stack(8, |e, name| {
                e.call(STRING_CONSTRUCT, &args![name]);
                e.call(TELEPORT_GET_CELL_MAP_NAME, &args![teleport, name]);
                let text = e.call(STRING_TEXT, &args![name]).u32();
                let name_id = e
                    .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
                    .u32();
                e.call(TILE_SET_STRING, &args![tile, name_id, text, 1u32]);
                e.call(STRING_DESTRUCT, &args![name]);
            });
        }
    });
}

/// `TeleportPath::TeleportPath` and `~TeleportPath` (engine map), thiscall
/// (0x38 bytes, `Doors` at +0x10).
const TELEPORT_PATH_CONSTRUCT: u32 = 0x006f_48b0;
const TELEPORT_PATH_DESTRUCT: u32 = 0x006f_4930;
/// `ActorLockData::ActorLockData` (engine map), thiscall (actor), `RET 4`
/// (0xC bytes).
const ACTOR_LOCK_DATA_CONSTRUCT: u32 = 0x0050_2670;
/// `PathingLocation::PathingLocation_ov3` (engine map), thiscall
/// (reference), `RET 4`.
const PATHING_LOCATION_OF_REFERENCE: u32 = 0x006d_cd70;
/// `Pathing::BuildTeleportDoorPath` (engine map), cdecl (to, from, path,
/// word, float, lock data, word): `AL`.
const BUILD_TELEPORT_DOOR_PATH: u32 = 0x006d_4d20;
/// Thiscall on a `BSSimpleArray`: its size (`0044ddc0`) and its first
/// element's address (`006cedb0`).
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
const ARRAY_FIRST_006CEDB0: u32 = 0x006c_edb0;
/// Thiscall on a cell: its north rotation in `ST0` (`00555ad0`).
const CELL_NORTH_ROTATION_00555AD0: u32 = 0x0055_5ad0;
/// The `double` subtracted from the negated heading (`010290d0`).
const HEADING_OFFSET_DOUBLE: u32 = 0x0102_90d0;
/// `Tile::HasFadeControl` (thiscall (trait), `RET 4`) and
/// `Tile::AddFadeControl` (thiscall (trait, float, float, float, word), `RET
/// 0x14`) (engine map).
const TILE_HAS_FADE_CONTROL: u32 = 0x00a0_7ed0;
const TILE_ADD_FADE_CONTROL: u32 = 0x00a0_7c60;

/// `AddMapIcons`' player reference on the local map in an interior: the
/// player's word +0x604; without one, the teleport door path from the
/// player (`Pathing::BuildTeleportDoorPath`) gives the first door's
/// teleport reference (`00568e50`, `00559450`), else the player.
fn local_map_player_reference(e: &mut Engine, this: Ptr<MapMenu>) {
    let player = e.mem.u32(PLAYER_011DEA3C);
    let reference = e.mem.u32(player + 0x604);
    e.set(this, MapMenu::pMapPlayerRef, reference);
    if e.get(this, MapMenu::pMapPlayerRef) != 0 {
        return;
    }
    e.with_stack(0x94, |e, frame| {
        // The frame's locals: the teleport path (+0), the lock data
        // (+0x38) and the two pathing locations (+0x44, +0x6C).
        let path = frame.addr();
        let lock = path + 0x38;
        let from = path + 0x44;
        let to = path + 0x6c;
        e.call(TELEPORT_PATH_CONSTRUCT, &args![path]);
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(ACTOR_LOCK_DATA_CONSTRUCT, &args![lock, player]);
        let from = e.call(PATHING_LOCATION_CONSTRUCT, &args![from]).u32();
        let player = e.mem.u32(PLAYER_011DEA3C);
        let to = e
            .call(PATHING_LOCATION_OF_REFERENCE, &args![to, player])
            .u32();
        let found = e
            .call(
                BUILD_TELEPORT_DOOR_PATH,
                &args![to, from, path, 3u32, 0.0f32.to_bits(), lock, 2u32],
            )
            .bool();
        e.call(PATHING_LOCATION_DESTRUCT, &args![path + 0x6c]);
        e.call(PATHING_LOCATION_DESTRUCT, &args![path + 0x44]);
        if found {
            if e.call(ARRAY_SIZE_0044DDC0, &args![path + 0x10]).u32() > 0 {
                let first = e.call(ARRAY_FIRST_006CEDB0, &args![path + 0x10]).u32();
                let door = e.mem.u32(first);
                let teleport = e.call(REFERENCE_TELEPORT_00568E50, &args![door]).u32();
                let reference = e.call(STRING_TEXT, &args![teleport]).u32();
                e.set(this, MapMenu::pMapPlayerRef, reference);
            }
            if e.get(this, MapMenu::pMapPlayerRef) == 0 {
                let player = e.mem.u32(PLAYER_011DEA3C);
                e.set(this, MapMenu::pMapPlayerRef, player);
            }
        }
        e.call(TELEPORT_PATH_DESTRUCT, &args![path]);
    });
}

// Translated from 0079dbb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079dbb0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 around the teleport path and the pathing locations) is not translated.
/// `MapMenu::AddMapIcons` (Xbox PDB): the player icon (tile 5 on page
/// 0x21, else tile 3) starts at (0.5, 0.5); `pMapPlayerRef` is the player,
/// or in an interior on page 0x21 the reference of
/// [`local_map_player_reference`]. With one, the icon shows at its map
/// position (north-rotated in a rotated cell), with the heading (vtable
/// +0x2BC of the player with 0, negated, minus `010290d0`, minus the cell's
/// north rotation in an interior) as trait `0xffa`; without one it is
/// hidden. The icon gets a fade control `0xff5` when it has none; the map
/// tile is moved to centre it; then `AddQuestMarkers`,
/// `AddCompanionMarkers` and, when the player has a map marker
/// (`00798400`), the player marker is deleted and created again.
pub fn map_menu_add_map_icons(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let icon = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        map_tile(e, base, 5)
    } else {
        map_tile(e, base, 3)
    };
    let half = float_constant(e, HALF_FLOAT);
    let half_again = float_constant(e, HALF_FLOAT);
    let mut point = e.with_stack(0xc, |e, point| {
        e.call(
            POINT_SET_00416870,
            &args![point, half_again, half, 0.0f32.to_bits()],
        );
        [
            e.mem.u32(point.addr()),
            e.mem.u32(point.addr() + 4),
            e.mem.u32(point.addr() + 8),
        ]
    });
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.get(this, MapMenu::iCurrentPage) == 0x21
        && e.call(REFR_GET_INTERIOR, &args![player]).bool()
    {
        local_map_player_reference(e, this);
    } else {
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.set(this, MapMenu::pMapPlayerRef, player);
    }
    if e.get(this, MapMenu::pMapPlayerRef) != 0 {
        let reference = e.get(this, MapMenu::pMapPlayerRef);
        let position = e.vcall(reference, 0x1f4, &args![]).u32();
        let position = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        point = e.with_stack(0x18, |e, frame| {
            let world = frame.addr();
            let result = world + 0xc;
            for (index, word) in position.iter().enumerate() {
                e.mem.set_u32(world + 4 * index as u32, *word);
            }
            let north = e.get(this, MapMenu::pAdjustNorthRotationCell);
            if north != 0 {
                e.call(ADJUST_FOR_NORTH_ROTATION, &args![north, world, world, 1u32]);
            }
            let reference = e.get(this, MapMenu::pMapPlayerRef);
            let space = e.call(REFR_GET_WORLD_SPACE, &args![reference]).u32();
            let x = e.mem.u32(world);
            let y = e.mem.u32(world + 4);
            let z = e.mem.u32(world + 8);
            let made = map_menu_world_to_map_coord(e, this, result, x, y, z, space);
            [e.mem.u32(made), e.mem.u32(made + 4), e.mem.u32(made + 8)]
        });
        e.call(TILE_SET_INT, &args![icon, 0xfa3u32, 1u32]);
        let x_id = e.mem.u32(TRAIT_X);
        e.call(
            TILE_SET_FLOAT,
            &args![icon, x_id, x87_quiet_bits(point[0]), 1u32],
        );
        let y_id = e.mem.u32(TRAIT_Y);
        e.call(
            TILE_SET_FLOAT,
            &args![icon, y_id, x87_quiet_bits(point[1]), 1u32],
        );
        let player = e.mem.u32(PLAYER_011DEA3C);
        let heading = e.vcall(player, 0x2bc, &args![0u32]).f64();
        let offset: f64 = e.global(HEADING_OFFSET_DOUBLE);
        let heading = x87_store32(x87_sub(-heading, offset));
        let player = e.mem.u32(PLAYER_011DEA3C);
        let rotation = if e.call(REFR_GET_INTERIOR, &args![player]).bool() {
            let player = e.mem.u32(PLAYER_011DEA3C);
            let cell = e.call(PLAYER_CELL_008D6F30, &args![player]).u32();
            x87_store32(e.call(CELL_NORTH_ROTATION_00555AD0, &args![cell]).f64())
        } else {
            0.0f32.to_bits()
        };
        let heading = x87_store32(x87_sub(x87_load32(heading), x87_m32(rotation)));
        e.call(
            TILE_SET_FLOAT,
            &args![icon, 0xffau32, x87_quiet_bits(heading), 1u32],
        );
    } else {
        e.call(TILE_SET_INT, &args![icon, 0xfa3u32, 0u32]);
    }
    if !e.call(TILE_HAS_FADE_CONTROL, &args![icon, 0xff5u32]).bool() {
        let low = float_constant(e, HALF_ALPHA_FLOAT);
        let high = float_constant(e, FULL_ALPHA_FLOAT);
        e.call(
            TILE_ADD_FADE_CONTROL,
            &args![icon, 0xff5u32, low, high, 1.0f32.to_bits(), 1u32],
        );
    }
    let two: f64 = e.global(TWO_DOUBLE);
    for (size_id, along, target) in [(0xfb1u32, point[0], 0xfa1u32), (0xfb0, point[1], 0xfa2)] {
        let map = e.get(this, MapMenu::pMapTile);
        let owner = e.call(WORD_AT_28, &args![map]).u32();
        let size = e.call(TILE_GET_FLOAT, &args![owner, size_id]).f64();
        let centre = x87_div(size, two);
        let map = e.get(this, MapMenu::pMapTile);
        let span = e.call(TILE_GET_FLOAT, &args![map, size_id]).f64();
        let value = x87_sub(centre, x87_mul(span, x87_m32(along)));
        let value = x87_quiet_bits(x87_store32(value));
        let map = e.get(this, MapMenu::pMapTile);
        e.call(TILE_SET_FLOAT, &args![map, target, value, 0u32]);
    }
    map_menu_add_quest_markers(e, this);
    map_menu_add_companion_markers(e, this);
    let player = e.mem.u32(PLAYER_011DEA3C);
    if fn_00798400(e, Ptr::new(player)) != 0 {
        map_menu_delete_player_marker(e, this);
        map_menu_create_player_marker(e, this);
    }
}

/// Thiscall on a tile's child list (+4) (address of a child), `RET 4`:
/// removes it (`0057c730`); `Tile::Release` (engine map), thiscall.
const CHILD_LIST_REMOVE_0057C730: u32 = 0x0057_c730;
const TILE_RELEASE: u32 = 0x009f_f690;
/// `PlayerCharacter::GetCurrentTargetList` (engine map), thiscall.
const PLAYER_GET_CURRENT_TARGET_LIST: u32 = 0x0095_2ba0;
/// The object whose byte (`00408d60`) enables the quest markers
/// (`011e07f0`).
const QUEST_MARKERS_SETTING_011E07F0: u32 = 0x011e_07f0;
/// Thiscall on a quest target: the reference the local map tracks
/// (`005cbb70`).
const TARGET_LOCAL_REFERENCE_005CBB70: u32 = 0x005c_bb70;
/// `"WorldMapQuestMarkerTemplate"`, `"LocalMapQuestMarkerTemplate"` and
/// the marker icon path.
const WORLD_QUEST_MARKER_TEMPLATE: u32 = 0x0107_53a8;
const LOCAL_QUEST_MARKER_TEMPLATE: u32 = 0x0107_538c;
const QUEST_MARKER_ICON_PATH: u32 = 0x0107_5364;

/// Removes and releases every child with id `id` of the map tile (the
/// child list's `0057c730`, `Tile::Release`, then its deleting destructor
/// with 1).
fn remove_map_children(e: &mut Engine, this: Ptr<MapMenu>, id: u32) {
    e.with_stack(4, |e, slot| loop {
        let map = e.get(this, MapMenu::pMapTile);
        let child = e.call(TILE_GET_CHILD_BY_ID, &args![map, id]).u32();
        e.mem.set_u32(slot.addr(), child);
        if child == 0 {
            break;
        }
        let map = e.get(this, MapMenu::pMapTile);
        e.call(CHILD_LIST_REMOVE_0057C730, &args![map + 4, slot]);
        let child = e.mem.u32(slot.addr());
        e.call(TILE_RELEASE, &args![child]);
        let child = e.mem.u32(slot.addr());
        if child != 0 {
            e.vcall(child, 0, &args![1u32]);
        }
    });
}

/// The marker tile of a tracked reference (quest and companion markers):
/// its position (north-rotated in a rotated cell) on the map
/// (`WorldToMapCoord` with its world space), returned as x and y.
fn tracked_map_position(e: &mut Engine, this: Ptr<MapMenu>, reference: u32) -> (u32, u32) {
    let position = e.vcall(reference, 0x1f4, &args![]).u32();
    let position = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    e.with_stack(0x18, |e, frame| {
        let world = frame.addr();
        let result = world + 0xc;
        for (index, word) in position.iter().enumerate() {
            e.mem.set_u32(world + 4 * index as u32, *word);
        }
        let north = e.get(this, MapMenu::pAdjustNorthRotationCell);
        if north != 0 {
            e.call(ADJUST_FOR_NORTH_ROTATION, &args![north, world, world, 1u32]);
        }
        let space = e.call(REFR_GET_WORLD_SPACE, &args![reference]).u32();
        let x = e.mem.u32(world);
        let y = e.mem.u32(world + 4);
        let z = e.mem.u32(world + 8);
        let made = map_menu_world_to_map_coord(e, this, result, x, y, z, space);
        (e.mem.u32(made), e.mem.u32(made + 4))
    })
}

// Translated from 0079e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::AddQuestMarkers` (Xbox PDB): removes the map tile's children
/// 0x1B and 0x1D, then while the quest markers setting (`011e07f0`) is on
/// walks the player's current targets: the tracked reference is, on page
/// 0x21, `GetWorldMapTrackingRef` (and a found one gets a waypoint path from
/// `pMapPlayerRef`, id 0x1D), else `005cbb70`; without one, the target's
/// reference is taken on page 0x21 outside interiors or on page 0x20 in the
/// player's cell. A tracked reference gets a quest marker tile (world or
/// local template) at its map position with the marker icon, trait `0xfaa`
/// 0x1B and its fade (`0xff5`) following the player icon's.
pub fn map_menu_add_quest_markers(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    remove_map_children(e, this, 0x1b);
    remove_map_children(e, this, 0x1d);
    let player = e.mem.u32(PLAYER_011DEA3C);
    let mut node = e.call(PLAYER_GET_CURRENT_TARGET_LIST, &args![player]).u32();
    loop {
        let setting = e
            .call(BYTE_OF_00408D60, &args![QUEST_MARKERS_SETTING_011E07F0])
            .u32();
        if e.mem.u8(setting) == 0 || node == 0 || node_item(e, node) == 0 {
            break;
        }
        let target = node_item(e, node);
        let reference = e
            .call(QUEST_TARGET_GET_REFERENCE, &args![target, 1u32])
            .u32();
        let mut tracked = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
            let target = node_item(e, node);
            let list = e.call(PLUS_10, &args![target]).u32();
            map_menu_get_world_map_tracking_ref(e, this, reference, list)
        } else {
            let target = node_item(e, node);
            e.call(TARGET_LOCAL_REFERENCE_005CBB70, &args![target])
                .u32()
        };
        if tracked != 0 {
            if e.get(this, MapMenu::iCurrentPage) == 0x21 {
                let target = node_item(e, node);
                let list = e.call(PLUS_10, &args![target]).u32();
                let from = e.get(this, MapMenu::pMapPlayerRef);
                map_menu_create_waypoint_path(e, this, from, tracked, list, 0x1d);
            }
        } else if reference != 0 {
            let take = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
                !e.call(REFR_GET_INTERIOR, &args![reference]).bool()
            } else {
                e.get(this, MapMenu::iCurrentPage) == 0x20 && {
                    let cell = e.call(PLAYER_CELL_008D6F30, &args![reference]).u32();
                    let player = e.mem.u32(PLAYER_011DEA3C);
                    cell == e.call(PLAYER_CELL_008D6F30, &args![player]).u32()
                }
            };
            if take {
                let target = node_item(e, node);
                tracked = e
                    .call(QUEST_TARGET_GET_REFERENCE, &args![target, 1u32])
                    .u32();
            }
        }
        if tracked != 0 {
            let (map_x, map_y) = tracked_map_position(e, this, tracked);
            let template = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
                WORLD_QUEST_MARKER_TEMPLATE
            } else {
                LOCAL_QUEST_MARKER_TEMPLATE
            };
            let map = e.get(this, MapMenu::pMapTile);
            let menu = map_menu(e);
            let tile = e
                .call(MENU_RENDER_TEMPLATE, &args![menu, map, template, 0u32])
                .u32();
            let x_id = e.mem.u32(TRAIT_X);
            e.call(
                TILE_SET_FLOAT,
                &args![tile, x_id, x87_quiet_bits(map_x), 1u32],
            );
            let y_id = e.mem.u32(TRAIT_Y);
            e.call(
                TILE_SET_FLOAT,
                &args![tile, y_id, x87_quiet_bits(map_y), 1u32],
            );
            e.call(
                TILE_SET_STRING,
                &args![tile, 0xfccu32, QUEST_MARKER_ICON_PATH, 1u32],
            );
            e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x1bu32]);
            let value = e.call(TILE_GET_VALUE, &args![tile, 0xff5u32]).u32();
            e.call(VALUE_CLEAR_ACTIONS, &args![value]);
            let icon = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
                map_tile(e, base, 5)
            } else {
                map_tile(e, base, 3)
            };
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![value, 0x7d0u32, icon, 0xff5u32],
            );
            e.call(VALUE_CALCULATE, &args![value, 0u32]);
        }
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
}

/// Thiscall on the player: its first companion node (`005eb7e0`).
const PLAYER_COMPANIONS_005EB7E0: u32 = 0x005e_b7e0;
/// `PlayerCharacter::BuildPathToTarget` (engine map), thiscall (target,
/// path, word), `RET 0xC`.
const PLAYER_BUILD_PATH_TO_TARGET: u32 = 0x0095_2d60;
/// Cdecl (text, text): 0 when equal (`00404dc0`).
const TEXT_COMPARE_00404DC0: u32 = 0x0040_4dc0;
/// `sprintf` (engine map), cdecl.
const SPRINTF: u32 = 0x00ec_623a;
/// `"%s Marker"` and the companion icon path.
const NAME_MARKER_FORMAT: u32 = 0x0107_53c4;
const COMPANION_ICON_PATH: u32 = 0x0107_53d0;
/// The `double`s the companion label sits above the companion by: on the
/// world map (`0101ecb8`) and on the local map (`HALF_ALPHA_DOUBLE`,
/// 128.0).
const WORLD_LABEL_OFFSET_DOUBLE: u32 = 0x0101_ecb8;
/// The `float`s the companion label tiles get: width `0xfba`
/// (`0102226c`), height `0xfb0` (`01034294`); and the marker's depth `0xfad`
/// (`0101712c`).
const LABEL_WIDTH_FLOAT: u32 = 0x0102_226c;
const LABEL_HEIGHT_FLOAT: u32 = 0x0103_4294;
const MARKER_DEPTH_FLOAT: u32 = 0x0101_712c;
/// `TESObjectREFR::GetDistanceFromReference` (engine map), thiscall
/// (reference, word, word), `RET 0xC`, and the distance under which a
/// companion's world map marker is left out (`01016968`).
const REFR_GET_DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
const COMPANION_NEAR_DOUBLE: u32 = 0x0101_6968;
/// `Tile::BuildTile` (engine map), cdecl (type).
const TILE_BUILD_TILE: u32 = 0x00a0_0c20;

/// The companion's tracked reference: on page 0x21 the path to it is built
/// (`PlayerCharacter::BuildPathToTarget`) and `GetWorldMapTrackingRef`
/// asked, else the companion itself.
fn companion_tracked(e: &mut Engine, this: Ptr<MapMenu>, companion: u32, path: u32) -> u32 {
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(
            PLAYER_BUILD_PATH_TO_TARGET,
            &args![player, companion, path, 1u32],
        );
    }
    if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        map_menu_get_world_map_tracking_ref(e, this, companion, path)
    } else {
        companion
    }
}

/// The companion and its label's map positions: the tracked reference's
/// position and the label's (raised by `0101ecb8` on page 0x21, by 128.0
/// otherwise), both through `WorldToMapCoord` (only the first is
/// north-rotated). Returns the marker's x, y and the label's x, y.
fn companion_positions(e: &mut Engine, this: Ptr<MapMenu>, tracked: u32) -> [u32; 4] {
    let position = e.vcall(tracked, 0x1f4, &args![]).u32();
    let position = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    let offset: f64 = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        e.global(WORLD_LABEL_OFFSET_DOUBLE)
    } else {
        e.global(HALF_ALPHA_DOUBLE)
    };
    let label_y = x87_store32(x87_sub(x87_load32(position[1]), offset));
    e.with_stack(0x24, |e, frame| {
        let world = frame.addr();
        let marker = world + 0xc;
        let label = world + 0x18;
        for (index, word) in position.iter().enumerate() {
            e.mem.set_u32(world + 4 * index as u32, *word);
        }
        // The code runs `006815c0` on its two result locals (no-ops).
        let north = e.get(this, MapMenu::pAdjustNorthRotationCell);
        if north != 0 {
            e.call(ADJUST_FOR_NORTH_ROTATION, &args![north, world, world, 1u32]);
        }
        let space = e.call(REFR_GET_WORLD_SPACE, &args![tracked]).u32();
        let x = e.mem.u32(world);
        let y = e.mem.u32(world + 4);
        let z = e.mem.u32(world + 8);
        let made = map_menu_world_to_map_coord(e, this, marker, x, y, z, space);
        let marker_x = e.mem.u32(made);
        let marker_y = e.mem.u32(made + 4);
        let space = e.call(REFR_GET_WORLD_SPACE, &args![tracked]).u32();
        let made =
            map_menu_world_to_map_coord(e, this, label, position[0], label_y, position[2], space);
        [marker_x, marker_y, e.mem.u32(made), e.mem.u32(made + 4)]
    })
}

/// The map tile's size along `id` (`0xfb1` or `0xfb0`) times `fraction`,
/// each stored as a `float`.
fn map_extent(e: &mut Engine, this: Ptr<MapMenu>, id: u32, fraction: u32) -> u32 {
    let map = e.get(this, MapMenu::pMapTile);
    let size = x87_store32(e.call(TILE_GET_FLOAT, &args![map, id]).f64());
    x87_store32(x87_mul(x87_load32(size), x87_m32(fraction)))
}

/// A tile's value `id` made to follow another tile's trait (action
/// `0x7d0`), cleared first and calculated after.
fn follow_trait(e: &mut Engine, tile: u32, id: u32, source: u32) {
    let value = e.call(TILE_GET_VALUE, &args![tile, id]).u32();
    e.call(VALUE_CLEAR_ACTIONS, &args![value]);
    e.call(VALUE_ADD_ACTION_TILE, &args![value, 0x7d0u32, source, id]);
    e.call(VALUE_CALCULATE, &args![value, 0u32]);
}

/// The companion label tile's traits (shared by `UpdateCompanionMarkers`
/// and `AddCompanionMarkers`): label position (`0xfa1`, `0xfa2`), the
/// companion's name (`0xfc4`), width, kind 0x26, depth 2.0, shown, the two
/// words 2 (`0xfb7`, `0xfb9`) and height. `UpdateCompanionMarkers` also sets
/// `0xfae` (`refresh`).
fn set_label_traits(e: &mut Engine, label: u32, x: u32, y: u32, companion: u32, refresh: bool) {
    e.call(
        TILE_SET_FLOAT,
        &args![label, 0xfa1u32, x87_quiet_bits(x), 1u32],
    );
    e.call(
        TILE_SET_FLOAT,
        &args![label, 0xfa2u32, x87_quiet_bits(y), 1u32],
    );
    let name = e.call(STATION_NAME_0055D520, &args![companion]).u32();
    e.call(TILE_SET_STRING, &args![label, 0xfc4u32, name, 1u32]);
    let width = float_constant(e, LABEL_WIDTH_FLOAT);
    e.call(TILE_SET_FLOAT, &args![label, 0xfbau32, width, 1u32]);
    e.call(TILE_SET_INT, &args![label, 0xfaau32, 0x26u32]);
    let depth = float_constant(e, TWO_FLOAT);
    e.call(TILE_SET_FLOAT, &args![label, 0xfadu32, depth, 1u32]);
    e.call(TILE_SET_INT, &args![label, 0xfa3u32, 1u32]);
    if refresh {
        e.call(TILE_SET_INT, &args![label, 0xfaeu32, 1u32]);
    }
    e.call(TILE_SET_INT, &args![label, 0xfb7u32, 2u32]);
    e.call(TILE_SET_INT, &args![label, 0xfb9u32, 2u32]);
    let height = float_constant(e, LABEL_HEIGHT_FLOAT);
    e.call(TILE_SET_FLOAT, &args![label, 0xfb0u32, height, 1u32]);
}

// Translated from 0079e470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079e470 - the C++ exception frame (FS:[0] chain, unwind state 0 around each companion's teleport path) and the /GS security cookie check (00ec408c) are not translated.
/// `MapMenu::UpdateCompanionMarkers` (Xbox PDB): for each of the player's
/// companions, the tracked reference's (see [`companion_tracked`]) child of
/// the map tile named after it is updated when its text (`0xfc4`) is the
/// companion's name: the marker and label positions, the label traits (see
/// [`set_label_traits`]) and, with a `"%s Marker"` child, the label
/// following that marker (`0xfa1`, `0xfa2` raised by `01034294`); then the
/// tile is queued for an update (0x3FF).
pub fn map_menu_update_companion_markers(e: &mut Engine, this: Ptr<MapMenu>) {
    let player = e.mem.u32(PLAYER_011DEA3C);
    let mut node = e.call(PLAYER_COMPANIONS_005EB7E0, &args![player]).u32();
    while node != 0 {
        let companion = node_item(e, node);
        e.with_stack(0x38, |e, path| {
            e.call(TELEPORT_PATH_CONSTRUCT, &args![path]);
            let tracked = companion_tracked(e, this, companion, path.addr());
            let label = if tracked != 0 {
                let name = e.call(STATION_NAME_0055D520, &args![tracked]).u32();
                let map = e.get(this, MapMenu::pMapTile);
                e.call(TILE_GET_CHILD_BY_NAME, &args![map, name]).u32()
            } else {
                0
            };
            let current =
                label != 0 && e.call(TILE_GET_STRING, &args![label, 0xfc4u32]).u32() != 0 && {
                    let text = e.call(TILE_GET_STRING, &args![label, 0xfc4u32]).u32();
                    let name = e.call(STATION_NAME_0055D520, &args![companion]).u32();
                    e.call(TEXT_COMPARE_00404DC0, &args![name, text]).u32() == 0
                };
            if current {
                update_companion_label(e, this, companion, tracked, label);
            }
            e.call(TELEPORT_PATH_DESTRUCT, &args![path]);
        });
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
}

/// `UpdateCompanionMarkers`' work on one current label (see there).
fn update_companion_label(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    companion: u32,
    tracked: u32,
    label: u32,
) {
    let marker_tile = e.with_stack(0x40, |e, text| {
        let name = if tracked != 0 {
            e.call(STATION_NAME_0055D520, &args![tracked]).u32()
        } else {
            0
        };
        e.call(SPRINTF, &args![text, NAME_MARKER_FORMAT, name]);
        if tracked != 0 {
            let map = e.get(this, MapMenu::pMapTile);
            e.call(TILE_GET_CHILD_BY_NAME, &args![map, text]).u32()
        } else {
            0
        }
    });
    let [_marker_x, _marker_y, label_x, label_y] = companion_positions(e, this, tracked);
    let x = map_extent(e, this, 0xfb1, label_x);
    let y = map_extent(e, this, 0xfb0, label_y);
    let x_id = e.mem.u32(TRAIT_X);
    e.call(
        TILE_SET_FLOAT,
        &args![label, x_id, x87_quiet_bits(label_x), 1u32],
    );
    let y_id = e.mem.u32(TRAIT_Y);
    e.call(
        TILE_SET_FLOAT,
        &args![label, y_id, x87_quiet_bits(label_y), 1u32],
    );
    set_label_traits(e, label, x, y, companion, true);
    if marker_tile != 0 {
        follow_marker(e, label, marker_tile);
    }
    e.call(TILE_ADD_NEEDS_UPDATE, &args![label, 0x3ffu32]);
}

// Translated from 0079ea30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079ea30 - the C++ exception frame (FS:[0] chain, unwind state 0 around each companion's teleport path) and the /GS security cookie check (00ec408c) are not translated.
/// `MapMenu::AddCompanionMarkers` (Xbox PDB): removes the map tile's
/// children 0x25 and 0x26, then for each of the player's companions with a
/// tracked reference (see [`companion_tracked`]) that its vtable +0x22C
/// (with 0) does not reject and, on page 0x20, in the player's cell or
/// world space: on page 0x21 a companion nearer than `01016968` is left out;
/// a companion on page 0x21 outside interiors or on page 0x20 in the
/// player's cell is used itself. It gets a marker tile (quest marker
/// template, `"%s Marker"`, the landmark icon, kind 0x1A, depth
/// `0101712c`, `"_LocationName"` `"Companion"`, its fade following the
/// player icon's) and a label tile (`Tile::BuildTile(0x387)` under the map
/// tile, see [`set_label_traits`], following the marker).
pub fn map_menu_add_companion_markers(e: &mut Engine, this: Ptr<MapMenu>) {
    remove_map_children(e, this, 0x25);
    remove_map_children(e, this, 0x26);
    let player = e.mem.u32(PLAYER_011DEA3C);
    let mut node = e.call(PLAYER_COMPANIONS_005EB7E0, &args![player]).u32();
    while node != 0 {
        let companion = node_item(e, node);
        e.with_stack(0x38, |e, path| {
            e.call(TELEPORT_PATH_CONSTRUCT, &args![path]);
            add_companion_marker(e, this, companion, path.addr());
            e.call(TELEPORT_PATH_DESTRUCT, &args![path]);
        });
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
}

/// Whether `reference` shares the player's cell (`008d6f30`).
fn in_player_cell(e: &mut Engine, reference: u32) -> bool {
    let cell = e.call(PLAYER_CELL_008D6F30, &args![reference]).u32();
    let player = e.mem.u32(PLAYER_011DEA3C);
    cell == e.call(PLAYER_CELL_008D6F30, &args![player]).u32()
}

/// `AddCompanionMarkers`' work on one companion (see there).
fn add_companion_marker(e: &mut Engine, this: Ptr<MapMenu>, companion: u32, path: u32) {
    let base = this.addr();
    let mut tracked = companion_tracked(e, this, companion, path);
    if tracked == 0 || e.vcall(tracked, 0x22c, &args![0u32]).bool() {
        return;
    }
    if e.get(this, MapMenu::iCurrentPage) == 0x20 && !in_player_cell(e, companion) {
        let space = e.call(REFR_GET_WORLD_SPACE, &args![companion]).u32();
        let player = e.mem.u32(PLAYER_011DEA3C);
        if space != e.call(REFR_GET_WORLD_SPACE, &args![player]).u32() {
            return;
        }
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    let distance = e
        .call(
            REFR_GET_DISTANCE_FROM_REFERENCE,
            &args![companion, player, 0u32, 0u32],
        )
        .f64();
    let distance = x87_load32(x87_store32(distance));
    let near: f64 = e.global(COMPANION_NEAR_DOUBLE);
    if below(distance, near) && e.get(this, MapMenu::iCurrentPage) == 0x21 {
        return;
    }
    if companion != 0 {
        let take = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
            !e.call(REFR_GET_INTERIOR, &args![companion]).bool()
        } else {
            e.get(this, MapMenu::iCurrentPage) == 0x20 && in_player_cell(e, companion)
        };
        if take {
            tracked = companion;
        }
    }
    if tracked == 0 {
        return;
    }
    let [marker_x, marker_y, label_x, label_y] = companion_positions(e, this, tracked);
    let icon = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        map_tile(e, base, 5)
    } else {
        map_tile(e, base, 3)
    };
    let template = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        WORLD_QUEST_MARKER_TEMPLATE
    } else {
        LOCAL_QUEST_MARKER_TEMPLATE
    };
    let map = e.get(this, MapMenu::pMapTile);
    let menu = map_menu(e);
    let marker = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, map, template, 0u32])
        .u32();
    let x_id = e.mem.u32(TRAIT_X);
    e.call(
        TILE_SET_FLOAT,
        &args![marker, x_id, x87_quiet_bits(marker_x), 1u32],
    );
    let y_id = e.mem.u32(TRAIT_Y);
    e.call(
        TILE_SET_FLOAT,
        &args![marker, y_id, x87_quiet_bits(marker_y), 1u32],
    );
    e.with_stack(0x40, |e, text| {
        let name = e.call(STATION_NAME_0055D520, &args![tracked]).u32();
        e.call(SPRINTF, &args![text, NAME_MARKER_FORMAT, name]);
        e.call(TILE_TEXT_SETTER_00707E10, &args![marker, text]);
    });
    e.call(
        TILE_SET_STRING,
        &args![marker, 0xfccu32, COMPANION_ICON_PATH, 1u32],
    );
    e.call(TILE_SET_INT, &args![marker, 0xfaau32, 0x1au32]);
    let depth = float_constant(e, MARKER_DEPTH_FLOAT);
    e.call(TILE_SET_FLOAT, &args![marker, 0xfadu32, depth, 1u32]);
    let name_id = e
        .call(ADD_USER_TRAIT, &args![LOCATION_NAME_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(
        TILE_SET_STRING,
        &args![marker, name_id, COMPANION_TEXT, 1u32],
    );
    follow_trait(e, marker, 0xff5, icon);
    let x = map_extent(e, this, 0xfb1, label_x);
    let y = map_extent(e, this, 0xfb0, label_y);
    let label = e.call(TILE_BUILD_TILE, &args![0x387u32]).u32();
    let name = e.call(STATION_NAME_0055D520, &args![companion]).u32();
    let map = e.get(this, MapMenu::pMapTile);
    e.vcall(label, 4, &args![map, name, 0u32]);
    let x_id = e.mem.u32(TRAIT_X);
    e.call(
        TILE_SET_FLOAT,
        &args![label, x_id, x87_quiet_bits(label_x), 1u32],
    );
    let y_id = e.mem.u32(TRAIT_Y);
    e.call(
        TILE_SET_FLOAT,
        &args![label, y_id, x87_quiet_bits(label_y), 1u32],
    );
    let name = e.call(STATION_NAME_0055D520, &args![tracked]).u32();
    e.call(TILE_TEXT_SETTER_00707E10, &args![label, name]);
    set_label_traits(e, label, x, y, companion, false);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![label, 2u32]);
    follow_marker(e, label, marker);
    follow_trait(e, label, 0xff5, icon);
}

/// The label's position values following the marker tile: both values
/// (`0xfa1`, `0xfa2`) are looked up first, then each is cleared, made to
/// follow the marker's same trait (action `0x7d0`; the second also raised
/// by `01034294`, action `0x7d1`) and calculated.
fn follow_marker(e: &mut Engine, label: u32, marker: u32) {
    let along_x = e.call(TILE_GET_VALUE, &args![label, 0xfa1u32]).u32();
    let along_y = e.call(TILE_GET_VALUE, &args![label, 0xfa2u32]).u32();
    e.call(VALUE_CLEAR_ACTIONS, &args![along_x]);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![along_x, 0x7d0u32, marker, 0xfa1u32],
    );
    e.call(VALUE_CALCULATE, &args![along_x, 0u32]);
    e.call(VALUE_CLEAR_ACTIONS, &args![along_y]);
    e.call(
        VALUE_ADD_ACTION_TILE,
        &args![along_y, 0x7d0u32, marker, 0xfa2u32],
    );
    let height = float_constant(e, LABEL_HEIGHT_FLOAT);
    e.call(VALUE_ADD_ACTION_FLOAT, &args![along_y, 0x7d1u32, height]);
    e.call(VALUE_CALCULATE, &args![along_y, 0u32]);
}

/// Thiscall on the player: the marker reference `CreatePlayerMarker` tracks
/// on the local map (`0077a400`).
const PLAYER_MARKER_REFERENCE_0077A400: u32 = 0x0077_a400;
/// The player marker icon path (`010753f0`).
const PLAYER_MARKER_ICON_PATH: u32 = 0x0107_53f0;
/// Thiscall on a teleport path's `Spaces` array (index), `RET 4`: the
/// address of the space's byte (`006a1440`); on its `Doors` array (index),
/// `RET 4`: the address of the door's reference (`00558dd0`).
const PATH_SPACE_FLAG_006A1440: u32 = 0x006a_1440;
const PATH_DOOR_00558DD0: u32 = 0x0055_8dd0;
/// Thiscall on a sound handle (word), `RET 4`: `AL` (`0076b780`); on a sound
/// list node (word), `RET 4`: `AL`, an empty node (`0076a3b0`); and the
/// node constructor (handle), `RET 4` (`0076b900`).
const SOUND_HANDLE_FLAG_0076B780: u32 = 0x0076_b780;
const SOUND_NODE_EMPTY_0076A3B0: u32 = 0x0076_a3b0;
const SOUND_NODE_CONSTRUCT_0076B900: u32 = 0x0076_b900;

// Translated from 0079f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::CreatePlayerMarker` (Xbox PDB): the player's map marker
/// (`00798400`) is tracked on page 0x21 through `GetWorldMapTrackingRef`
/// with the player's path (+0x6F8), getting a waypoint path from
/// `pMapPlayerRef` (id 0x1E) when found; elsewhere the player's `0077a400`
/// reference. Without one, the marker itself is used on page 0x21 outside
/// interiors or on page 0x20 in the player's cell. A tracked reference gets
/// a marker tile (quest marker template, at its map position, the player
/// marker icon, kind 0x1C, depth `0101712c`, its fade following the player
/// icon's).
pub fn map_menu_create_player_marker(e: &mut Engine, this: Ptr<MapMenu>) {
    let base = this.addr();
    let mut tracked = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        let player = e.mem.u32(PLAYER_011DEA3C);
        let path = fn_0079f620(e, Ptr::new(player));
        let player = e.mem.u32(PLAYER_011DEA3C);
        let marker = fn_00798400(e, Ptr::new(player));
        map_menu_get_world_map_tracking_ref(e, this, marker, path)
    } else {
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(PLAYER_MARKER_REFERENCE_0077A400, &args![player])
            .u32()
    };
    let player = e.mem.u32(PLAYER_011DEA3C);
    if tracked != 0 {
        if e.get(this, MapMenu::iCurrentPage) == 0x21 {
            let path = fn_0079f620(e, Ptr::new(player));
            let from = e.get(this, MapMenu::pMapPlayerRef);
            map_menu_create_waypoint_path(e, this, from, tracked, path, 0x1e);
        }
    } else if fn_00798400(e, Ptr::new(player)) != 0 {
        let take = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
            let marker = fn_00798400(e, Ptr::new(player));
            !e.call(REFR_GET_INTERIOR, &args![marker]).bool()
        } else {
            e.get(this, MapMenu::iCurrentPage) == 0x20 && {
                let marker = fn_00798400(e, Ptr::new(player));
                in_player_cell(e, marker)
            }
        };
        if take {
            tracked = fn_00798400(e, Ptr::new(player));
        }
    }
    if tracked == 0 {
        return;
    }
    let (map_x, map_y) = tracked_map_position(e, this, tracked);
    let icon = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        map_tile(e, base, 5)
    } else {
        map_tile(e, base, 3)
    };
    let template = if e.get(this, MapMenu::iCurrentPage) == 0x21 {
        WORLD_QUEST_MARKER_TEMPLATE
    } else {
        LOCAL_QUEST_MARKER_TEMPLATE
    };
    let map = e.get(this, MapMenu::pMapTile);
    let menu = map_menu(e);
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, map, template, 0u32])
        .u32();
    let x_id = e.mem.u32(TRAIT_X);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, x_id, x87_quiet_bits(map_x), 1u32],
    );
    let y_id = e.mem.u32(TRAIT_Y);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, y_id, x87_quiet_bits(map_y), 1u32],
    );
    e.call(
        TILE_SET_STRING,
        &args![tile, 0xfccu32, PLAYER_MARKER_ICON_PATH, 1u32],
    );
    e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x1cu32]);
    let depth = float_constant(e, MARKER_DEPTH_FLOAT);
    e.call(TILE_SET_FLOAT, &args![tile, 0xfadu32, depth, 1u32]);
    follow_trait(e, tile, 0xff5, icon);
}

// Translated from 0079f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this + 0x6F8` (on the player: the teleport
/// path `CreatePlayerMarker` passes).
pub fn fn_0079f620(e: &mut Engine, this: Ptr) -> u32 {
    let _ = e;
    this.addr().wrapping_add(0x6f8)
}

// Translated from 0079f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::SetPlayerMarker` (Xbox PDB): with an open map menu and a
/// player map marker (`00798400`), `PlayerMarkerPos` becomes the marker's
/// position and `PlayerMarkerSpace` its world space, else its cell.
pub fn map_menu_set_player_marker(e: &mut Engine) {
    if map_menu(e) == 0 {
        return;
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    if fn_00798400(e, Ptr::new(player)) == 0 {
        return;
    }
    let marker = fn_00798400(e, Ptr::new(player));
    let position = e.vcall(marker, 0x1f4, &args![]).u32();
    let menu = map_menu(e);
    for word in 0..3 {
        let value = e.mem.u32(position + 4 * word);
        e.mem.set_u32(menu + 0xfc + 4 * word, value);
    }
    let marker = fn_00798400(e, Ptr::new(player));
    let space = e.call(REFR_GET_WORLD_SPACE, &args![marker]).u32();
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0xf8, space);
    if e.mem.u32(map_menu(e) + 0xf8) == 0 {
        let marker = fn_00798400(e, Ptr::new(player));
        let cell = e.call(PLAYER_CELL_008D6F30, &args![marker]).u32();
        let menu = map_menu(e);
        e.mem.set_u32(menu + 0xf8, cell);
    }
}

// Translated from 0079f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::DeletePlayerMarker` (Xbox PDB): removes and releases the map
/// tile's first child 0x1C (once) and every child 0x1E (see
/// [`remove_map_children`]).
pub fn map_menu_delete_player_marker(e: &mut Engine, this: Ptr<MapMenu>) {
    e.with_stack(4, |e, slot| {
        let map = e.get(this, MapMenu::pMapTile);
        let child = e.call(TILE_GET_CHILD_BY_ID, &args![map, 0x1cu32]).u32();
        e.mem.set_u32(slot.addr(), child);
        if child != 0 {
            let map = e.get(this, MapMenu::pMapTile);
            e.call(CHILD_LIST_REMOVE_0057C730, &args![map + 4, slot]);
            let child = e.mem.u32(slot.addr());
            e.call(TILE_RELEASE, &args![child]);
            let child = e.mem.u32(slot.addr());
            if child != 0 {
                e.vcall(child, 0, &args![1u32]);
            }
        }
    });
    remove_map_children(e, this, 0x1e);
}

// Translated from 0079f7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::GetWorldMapTrackingRef` (Xbox PDB): 0 without a path or with
/// both its arrays empty; `reference` when it is null or not in an
/// interior; otherwise the door of the last space (from the end) whose
/// flag byte is set and that has a door at that index, else
/// `pMapPlayerRef`.
pub fn map_menu_get_world_map_tracking_ref(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    reference: u32,
    path: u32,
) -> u32 {
    if path == 0 {
        return 0;
    }
    if e.call(ARRAY_SIZE_0044DDC0, &args![path]).u32() == 0
        && e.call(ARRAY_SIZE_0044DDC0, &args![path + 0x10]).u32() == 0
    {
        return 0;
    }
    if reference == 0 || !e.call(REFR_GET_INTERIOR, &args![reference]).bool() {
        return reference;
    }
    let mut found = false;
    let mut result = 0u32;
    let mut index = e
        .call(ARRAY_SIZE_0044DDC0, &args![path])
        .u32()
        .wrapping_sub(1) as i32;
    while !found && index >= 0 {
        let flag = e.call(PATH_SPACE_FLAG_006A1440, &args![path, index]).u32();
        if e.mem.u8(flag) != 0
            && (index as u32) < e.call(ARRAY_SIZE_0044DDC0, &args![path + 0x10]).u32()
        {
            let door = e.call(PATH_DOOR_00558DD0, &args![path + 0x10, index]).u32();
            result = e.mem.u32(door);
            found = true;
        }
        index = index.wrapping_sub(1);
    }
    if !found {
        result = e.get(this, MapMenu::pMapPlayerRef);
    }
    result
}

// Translated from 007a19a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007a19a0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new node's construction) is not translated.
/// `BSSimpleList<BSSoundHandle>::AddTail` (Xbox PDB): unless `0076b780(0)`
/// says so of the handle, walks to the last node (next at +0xC); when
/// `0076a3b0(0)` is set of it a new 0x10-byte node (`0076b900` with the
/// handle) is linked after it, otherwise it gets the handle assigned
/// (`00418900`).
pub fn fn_007a19a0(e: &mut Engine, this: Ptr, handle: u32) {
    if e.call(SOUND_HANDLE_FLAG_0076B780, &args![handle, 0u32])
        .bool()
    {
        return;
    }
    let mut node = this.addr();
    while e.mem.u32(node + 0xc) != 0 {
        node = e.mem.u32(node + 0xc);
    }
    if e.call(SOUND_NODE_EMPTY_0076A3B0, &args![node, 0u32]).bool() {
        let memory = e.call(ALLOCATE, &args![0x10u32]).u32();
        let made = if memory != 0 {
            e.call(SOUND_NODE_CONSTRUCT_0076B900, &args![memory, handle])
                .u32()
        } else {
            0
        };
        e.mem.set_u32(node + 0xc, made);
    } else {
        e.call(SOUND_HANDLE_ASSIGN, &args![node, handle]);
    }
}

/// `"WaypointTemplate"`, `"PathLineTemplate"` and the interior path line
/// texture.
const WAYPOINT_TEMPLATE: u32 = 0x0107_5454;
const PATH_LINE_TEMPLATE: u32 = 0x0107_5440;
const INTERIOR_PATH_TEXTURE: u32 = 0x0107_5410;
/// Cdecl (float, word): a float in `ST0` (`00685d20`; `CreateWaypointPath`
/// passes 2 and sums the two results); cdecl (float): a float in `ST0`
/// (`004579e0`, of the sum); `atan2` (engine map), cdecl (float, float).
const FLOAT_POWER_00685D20: u32 = 0x0068_5d20;
const FLOAT_ROOT_004579E0: u32 = 0x0045_79e0;
const ATAN2: u32 = 0x004b_5550;

/// The map point of `reference` (its world space asked first, then its
/// position, vtable +0x1F4), through `WorldToMapCoord`: x and y.
fn reference_map_point(e: &mut Engine, this: Ptr<MapMenu>, reference: u32) -> (u32, u32) {
    let space = e.call(REFR_GET_WORLD_SPACE, &args![reference]).u32();
    let position = e.vcall(reference, 0x1f4, &args![]).u32();
    let x = e.mem.u32(position);
    let y = e.mem.u32(position + 4);
    let z = e.mem.u32(position + 8);
    e.with_stack(0xc, |e, result| {
        let made = map_menu_world_to_map_coord(e, this, result.addr(), x, y, z, space);
        (e.mem.u32(made), e.mem.u32(made + 4))
    })
}

/// A `"WaypointTemplate"` tile under the map tile at a map point, kind
/// `id`.
fn new_waypoint(e: &mut Engine, this: Ptr<MapMenu>, point: (u32, u32), id: u32) -> u32 {
    let map = e.get(this, MapMenu::pMapTile);
    let menu = map_menu(e);
    let tile = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu, map, WAYPOINT_TEMPLATE, 0u32],
        )
        .u32();
    let x_id = e.mem.u32(TRAIT_X);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, x_id, x87_quiet_bits(point.0), 1u32],
    );
    let y_id = e.mem.u32(TRAIT_Y);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, y_id, x87_quiet_bits(point.1), 1u32],
    );
    e.call(TILE_SET_INT, &args![tile, 0xfaau32, id]);
    tile
}

/// The teleport reference of a door (`00568e50`, then `00559450`).
fn door_destination(e: &mut Engine, door: u32) -> u32 {
    let teleport = e.call(REFERENCE_TELEPORT_00568E50, &args![door]).u32();
    e.call(STRING_TEXT, &args![teleport]).u32()
}

/// The door at `index` of a teleport path's `Doors` (+0x10).
fn path_door(e: &mut Engine, path: u32, index: u32) -> u32 {
    let door = e.call(PATH_DOOR_00558DD0, &args![path + 0x10, index]).u32();
    e.mem.u32(door)
}

/// Appends a tile to the local list (through a stack slot holding it).
fn list_append(e: &mut Engine, list: u32, tile: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), tile);
        e.call(LIST_ADD_TAIL_00905820, &args![list, slot]);
    });
}

/// The two float traits (`TRAIT_X`, `TRAIT_Y`, the y asked first) of the
/// tile of a list node, as an `NiPoint2` (`00452dc0`).
fn node_point(e: &mut Engine, node: u32) -> (u32, u32) {
    let y_id = e.mem.u32(TRAIT_Y);
    let tile = node_item(e, node);
    let y = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, y_id]).f64());
    let x_id = e.mem.u32(TRAIT_X);
    let tile = node_item(e, node);
    let x = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, x_id]).f64());
    map_corner_bits(e, x, y)
}

// Translated from 0079f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079f8d0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the local tile list) is not translated.
/// `MapMenu::CreateWaypointPath` (Xbox PDB): with `from`, `to` and a
/// teleport path, puts a `"WaypointTemplate"` tile (kind `id`) at `from`
/// (alpha `0xfa9` 0.0, trait `0x1004` 1 when it is a door into an
/// interior), one at each exterior door between them (the doors from just
/// after `from` up to just before `to`; the door's own reference, else its
/// teleport reference; trait `0x1004` 1 when the teleport leads into an
/// interior), and one at `to` (alpha 0.0); then joins consecutive tiles
/// with `"PathLineTemplate"` tiles (kind `id`) whose length (`0x1004`,
/// scaled by the map's `0xfcd`/`0xfce`) and angle (`0xffa`, `-atan2`) span
/// them, the interior texture when the start's owner has `0x1004`.
pub fn map_menu_create_waypoint_path(
    e: &mut Engine,
    this: Ptr<MapMenu>,
    from: u32,
    to: u32,
    path: u32,
    id: u32,
) {
    if from == 0 || to == 0 || path == 0 {
        return;
    }
    e.with_stack(8, |e, list| {
        let list = list.addr();
        e.call(LIST_CONSTRUCT, &args![list]);
        let from_point = reference_map_point(e, this, from);
        let to_point = reference_map_point(e, this, to);
        let first = new_waypoint(e, this, from_point, id);
        e.call(
            TILE_SET_FLOAT,
            &args![first, 0xfa9u32, 0.0f32.to_bits(), 1u32],
        );
        let object = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![from]).u32();
        if e.call(FORM_TYPE, &args![object]).u32() == 0x1c {
            let destination = door_destination(e, from);
            if e.call(REFR_GET_INTERIOR, &args![destination]).bool() {
                e.call(TILE_SET_INT, &args![first, 0x1004u32, 1u32]);
            }
        }
        list_append(e, list, first);
        let mut start = 0i32;
        let mut end = e
            .call(ARRAY_SIZE_0044DDC0, &args![path + 0x10])
            .u32()
            .wrapping_sub(1) as i32;
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE_0044DDC0, &args![path + 0x10]).u32() {
            if path_door(e, path, index) == from || {
                let door = path_door(e, path, index);
                door_destination(e, door) == from
            } {
                start = index.wrapping_add(1) as i32;
            }
            if path_door(e, path, index) == to || {
                let door = path_door(e, path, index);
                door_destination(e, door) == to
            } {
                end = index.wrapping_sub(1) as i32;
            }
            index = index.wrapping_add(1);
        }
        let mut step = start;
        while step <= end {
            let door = path_door(e, path, step as u32);
            let mut reference = if e.call(REFR_GET_INTERIOR, &args![door]).bool() {
                0
            } else {
                path_door(e, path, step as u32)
            };
            if reference == 0 {
                let door = path_door(e, path, step as u32);
                reference = door_destination(e, door);
            }
            if reference != 0 && !e.call(REFR_GET_INTERIOR, &args![reference]).bool() {
                let map = e.get(this, MapMenu::pMapTile);
                let menu = map_menu(e);
                let tile = e
                    .call(
                        MENU_RENDER_TEMPLATE,
                        &args![menu, map, WAYPOINT_TEMPLATE, 0u32],
                    )
                    .u32();
                let point = reference_map_point(e, this, reference);
                let x_id = e.mem.u32(TRAIT_X);
                e.call(
                    TILE_SET_FLOAT,
                    &args![tile, x_id, x87_quiet_bits(point.0), 1u32],
                );
                let y_id = e.mem.u32(TRAIT_Y);
                e.call(
                    TILE_SET_FLOAT,
                    &args![tile, y_id, x87_quiet_bits(point.1), 1u32],
                );
                e.call(TILE_SET_INT, &args![tile, 0xfaau32, id]);
                let door = path_door(e, path, step as u32);
                let destination = door_destination(e, door);
                if e.call(REFR_GET_INTERIOR, &args![destination]).bool() {
                    e.call(TILE_SET_INT, &args![tile, 0x1004u32, 1u32]);
                }
                list_append(e, list, tile);
            }
            step = step.wrapping_add(1);
        }
        let last = new_waypoint(e, this, to_point, id);
        e.call(
            TILE_SET_FLOAT,
            &args![last, 0xfa9u32, 0.0f32.to_bits(), 1u32],
        );
        list_append(e, list, last);
        let scale = |e: &mut Engine, trait_id: u32| -> u32 {
            if e.get(this, MapMenu::pMapTile) == 0 {
                return 0.0f32.to_bits();
            }
            let map = e.get(this, MapMenu::pMapTile);
            x87_store32(e.call(TILE_GET_FLOAT, &args![map, trait_id]).f64())
        };
        let scale_x = scale(e, 0xfcd);
        let scale_y = scale(e, 0xfce);
        let mut node = list;
        while node != 0 && node_item(e, node) != 0 && e.call(WORD_AT_4, &args![node]).u32() != 0 {
            let tile = node_item(e, node);
            let menu = map_menu(e);
            let line = e
                .call(
                    MENU_RENDER_TEMPLATE,
                    &args![menu, tile, PATH_LINE_TEMPLATE, 0u32],
                )
                .u32();
            let (ax, ay) = node_point(e, node);
            let (bx, by) = node_point_of_next(e, node);
            e.call(TILE_SET_INT, &args![line, 0xfaau32, id]);
            let dx = x87_store32(x87_mul(
                x87_sub(x87_load32(bx), x87_m32(ax)),
                x87_m32(scale_x),
            ));
            let square_x = e
                .call(FLOAT_POWER_00685D20, &args![x87_quiet_bits(dx), 2u32])
                .f64();
            let dy = x87_store32(x87_mul(
                x87_sub(x87_load32(by), x87_m32(ay)),
                x87_m32(scale_y),
            ));
            let square_y = e
                .call(FLOAT_POWER_00685D20, &args![x87_quiet_bits(dy), 2u32])
                .f64();
            let sum = x87_quiet_bits(x87_store32(x87_add(square_y, square_x)));
            let length = x87_store32(e.call(FLOAT_ROOT_004579E0, &args![sum]).f64());
            e.call(TILE_SET_FLOAT, &args![line, 0x1004u32, length, 1u32]);
            let across = x87_quiet_bits(x87_store32(x87_sub(x87_load32(bx), x87_m32(ax))));
            let down = x87_quiet_bits(x87_store32(x87_sub(x87_load32(by), x87_m32(ay))));
            let angle = e.call(ATAN2, &args![down, across]).f64();
            let angle = x87_store32(-angle);
            e.call(TILE_SET_FLOAT, &args![line, 0xffau32, angle, 1u32]);
            let owner = e.call(WORD_AT_28, &args![line]).u32();
            if e.call(TILE_IS_TRUE, &args![owner, 0x1004u32]).bool() {
                e.call(
                    TILE_SET_STRING,
                    &args![line, 0xfccu32, INTERIOR_PATH_TEXTURE, 1u32],
                );
            }
            node = e.call(WORD_AT_4, &args![node]).u32();
        }
        e.call(LIST_DESTRUCT, &args![list]);
    });
}

/// [`node_point`] of the node after `node`, the next pointer (`00726070`)
/// read again for each trait as the code does.
fn node_point_of_next(e: &mut Engine, node: u32) -> (u32, u32) {
    let y_id = e.mem.u32(TRAIT_Y);
    let next = e.call(WORD_AT_4, &args![node]).u32();
    let tile = node_item(e, next);
    let y = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, y_id]).f64());
    let x_id = e.mem.u32(TRAIT_X);
    let next = e.call(WORD_AT_4, &args![node]).u32();
    let tile = node_item(e, next);
    let x = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, x_id]).f64());
    map_corner_bits(e, x, y)
}

/// The `double` added to the player's height (`01020758`, 10.0).
const HEIGHT_OFFSET_DOUBLE: u32 = 0x0102_0758;
/// Cdecl: a float in `ST0`, the local map's texture coordinate scale
/// (`00879d90`).
const LOCAL_MAP_UV_SCALE_00879D90: u32 = 0x0087_9d90;
/// The four words saved, replaced from `011a9bd0` and restored around the
/// local map's geometry (`011ff3a8`), and the byte set to 1 meanwhile
/// (`011ff103`).
const RENDER_STATE_011FF3A8: u32 = 0x011f_f3a8;
const RENDER_STATE_SOURCE_011A9BD0: u32 = 0x011a_9bd0;
const RENDER_FLAG_011FF103: u32 = 0x011f_f103;
/// Thiscall on a colour (r, g, b, a), `RET 0x10` (`00414430`).
const COLOR_SET_00414430: u32 = 0x0041_4430;
/// `SystemColorManager::GetInstance` (cdecl) and `GetColor` (thiscall
/// (index, colour), `RET 8`) (engine map).
const SYSTEM_COLOR_MANAGER_GET_INSTANCE: u32 = 0x0071_8b60;
const SYSTEM_COLOR_MANAGER_GET_COLOR: u32 = 0x0071_9060;
/// `TESWorldSpace::GetTerrainManager` (engine map), thiscall; on it
/// `0043b230` gives an object whose byte `00456610` reads and `00450f90`
/// (word, `RET 4`) sets.
const WORLD_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
const TERRAIN_OBJECT_0043B230: u32 = 0x0043_b230;
const OBJECT_FLAG_00456610: u32 = 0x0045_6610;
const OBJECT_SET_FLAG_00450F90: u32 = 0x0045_0f90;
/// `TESObjectCELL::GetIntSeenSection` (thiscall (x, y, word), `RET 0xC`),
/// `GetInteriorLocalMapTexture` (thiscall (x, y, texture), `RET 0xC`),
/// `0054e640` (thiscall (texture), `RET 4`, the exterior texture),
/// `0054dcf0` (thiscall (list), `RET 4`) (engine map), and
/// `ExtraDataList::GetSeenData` (thiscall).
const CELL_GET_INT_SEEN_SECTION: u32 = 0x0055_6ef0;
const CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE: u32 = 0x0054_e750;
const CELL_GET_EXTERIOR_TEXTURE_0054E640: u32 = 0x0054_e640;
const CELL_ADD_MARKERS_0054DCF0: u32 = 0x0054_dcf0;
const EXTRA_GET_SEEN_DATA: u32 = 0x0055_5bc0;
/// Cdecl: the renderer object (`0043c4b0`) whose `004a0370` and `004a03c0`
/// (thiscall) bracket the texture's making.
const RENDERER_0043C4B0: u32 = 0x0043_c4b0;
const RENDERER_BEGIN_004A0370: u32 = 0x004a_0370;
const RENDERER_END_004A03C0: u32 = 0x004a_03c0;
/// `TESWorldSpace::GetCellFromCellCoord` (engine map), thiscall (x, y),
/// `RET 8`; `TES::GetWorldSpace` (engine map), thiscall.
const WORLD_GET_CELL_FROM_CELL_COORD: u32 = 0x0058_75a0;
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// The import slot `0079ffb0` calls with a directory path and 0 (the
/// directories of the saved local maps).
const CREATE_DIRECTORY_IMPORT: u32 = 0x00fd_f0b8;
/// The directory and file name formats of the saved local maps.
const LOCAL_MAPS_DIRECTORY: u32 = 0x0107_5520;
const LOCAL_MAPS_INTERIORS_DIRECTORY: u32 = 0x0107_5508;
const LOCAL_MAPS_EXTERIORS_DIRECTORY: u32 = 0x0107_54f0;
const INTERIOR_DIRECTORY_FORMAT: u32 = 0x0107_54d4;
const INTERIOR_FILE_FORMAT: u32 = 0x0107_54ac;
const EXTERIOR_DIRECTORY_FORMAT: u32 = 0x0107_5490;
const EXTERIOR_FILE_FORMAT: u32 = 0x0107_5468;
/// `D3DTexture_LockRect` (engine map; the word at +0x24), cdecl
/// `00653270(object, word)`, its object (`011f4418`), and the thunk of the
/// texture saving import (`00ee6dc2`, stdcall (file, format, texture, palette)).
const D3D_TEXTURE_LOCK_RECT: u32 = 0x0059_bb30;
const TEXTURE_LOOKUP_00653270: u32 = 0x0065_3270;
const TEXTURE_TABLE_011F4418: u32 = 0x011f_4418;
const SAVE_TEXTURE_JUMP_00EE6DC2: u32 = 0x00ee_6dc2;
/// The vector constructor iterator (`00401050`, stdcall (array, size,
/// count, constructor)) and the element constructors it gets (`006815c0`
/// for points, `004a7800` for colours).
const VECTOR_CONSTRUCTOR_00401050: u32 = 0x0040_1050;
const COLOR_CONSTRUCTOR_004A7800: u32 = 0x004a_7800;
/// Cdecl (size): the two allocators the local map geometry uses
/// (`00aa1070`, `00aa13e0`).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// Thiscall on a point (point), `RET 4`: adds it (`0063c8a0`).
const POINT_ADD_0063C8A0: u32 = 0x0063_c8a0;
/// The `float` the fog alpha starts at (`0101e570`).
const FOG_ALPHA_FLOAT: u32 = 0x0101_e570;
/// `InterfaceManager::GetScreenWidth` (engine map) and `00715da0`, cdecl,
/// floats in `ST0`.
const SCREEN_WIDTH_00715D40: u32 = 0x0071_5d40;
const SCREEN_HEIGHT_00715DA0: u32 = 0x0071_5da0;
/// `BSScissorTriShape::BSScissorTriShape` (engine map), thiscall (13 words),
/// `RET 0x34`.
const SCISSOR_TRI_SHAPE_CONSTRUCT: u32 = 0x00c5_cf30;
/// `TileShaderProperty::TileShaderProperty` (thiscall (word), `RET 4`).
const TILE_SHADER_PROPERTY_CONSTRUCT: u32 = 0x00bb_7c30;
/// Thiscall on the shader property (word), `RET 4` (`008cc6d0`); on the
/// texture pointer (word), `RET 4` (`0066b0d0`).
const PROPERTY_SET_008CC6D0: u32 = 0x008c_c6d0;
const TEXTURE_POINTER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `NiAVObject::AttachProperty` (thiscall (property), `RET 4`) and
/// `NiAVObject::UpdateProperties` (thiscall) (engine map).
const AV_OBJECT_ATTACH_PROPERTY: u32 = 0x0043_9410;
const AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The alpha property: constructor (`004391c0`) and its setters (word,
/// `RET 4`: `00439340`, `00439390`, `0049ed90`).
const ALPHA_PROPERTY_CONSTRUCT_004391C0: u32 = 0x0043_91c0;
const ALPHA_PROPERTY_SET_00439340: u32 = 0x0043_9340;
const ALPHA_PROPERTY_SET_00439390: u32 = 0x0043_9390;
const ALPHA_PROPERTY_SET_0049ED90: u32 = 0x0049_ed90;
/// `BSShaderManager::PrepareObject` (engine map), cdecl (object, word,
/// word).
const SHADER_MANAGER_PREPARE_OBJECT: u32 = 0x00b5_7e30;

/// An allocation of `count` elements of `size` bytes (`MUL; SETO; NEG;
/// OR`: all ones when the product overflows) by `ALLOCATE`, then the vector
/// constructor iterator over it with `constructor` when it is not null.
fn new_array(e: &mut Engine, count: u32, size: u32, constructor: u32) -> u32 {
    let (bytes, overflow) = count.overflowing_mul(size);
    let bytes = if overflow { 0xffff_ffff } else { bytes };
    let array = e.call(ALLOCATE, &args![bytes]).u32();
    if array == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCTOR_00401050,
        &args![array, size, count, constructor],
    );
    array
}

/// A point (`00416870`) built in a stack slot from three float bit
/// patterns, its words returned.
fn point_words(e: &mut Engine, x: u32, y: u32, z: u32) -> [u32; 3] {
    e.with_stack(0xc, |e, point| {
        let made = e.call(POINT_SET_00416870, &args![point, x, y, z]).u32();
        [e.mem.u32(made), e.mem.u32(made + 4), e.mem.u32(made + 8)]
    })
}

/// The flag of the player's terrain object (`00586170`, `0043b230` on the
/// world space): `00456610` of it, then `00450f90` with `value`, or 1 when
/// there is no such object.
fn terrain_object(e: &mut Engine) -> u32 {
    let player = e.mem.u32(PLAYER_011DEA3C);
    let space = e.call(REFR_GET_WORLD_SPACE, &args![player]).u32();
    let manager = e.call(WORLD_GET_TERRAIN_MANAGER, &args![space]).u32();
    e.call(TERRAIN_OBJECT_0043B230, &args![manager]).u32()
}

// Translated from 0079ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0079ffb0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 around the texture pointer and the geometry's allocations) and the /GS security cookie check (00ec408c) are not translated.
/// Unnamed in the engine map (`MapMenu`, one step of the local map's
/// making, cdecl): with an open map menu whose tile 2 has an object and
/// `007a1750`, the next grid cell (`uiNextLocalMapGrid`, column and row of
/// the `0043d4d0` side) around the player gets its texture (an interior
/// through the player's cell, rotated to north, `GetIntSeenSection` and
/// `GetInteriorLocalMapTexture`; an exterior through its world space's cell
/// at that coordinate, `GetSeenData` and `0054e640`), the markers of the
/// first cell going to the marker list (+0xE0). With `save` the texture is
/// written to `(LocalMaps)\...\%s.%i.%i.png` instead. Otherwise a 17 x 17
/// vertex `BSScissorTriShape` (cells of 0x400, 0x800 in interiors, over
/// 16; alternating diagonals; colour alpha from the fog of war) with a
/// `TileShaderProperty` of the texture and an alpha property is placed at
/// the cell and attached to tile 2's object; tile 2 gets the map size
/// (`0xfcd`, `0xfce`) and the system colour 4 (`0xfb2` to `0xfb4`), a zero
/// rotation, and the local map is updated when the grid is complete and
/// tile 2 is the map tile. The render state words (`011ff3a8`), the byte
/// `011ff103` and the terrain flags are restored.
pub fn fn_0079ffb0(e: &mut Engine, save: u8) {
    if map_menu(e) == 0 {
        return;
    }
    let tile_2 = map_tile(e, map_menu(e), 2);
    if e.call(TILE_OBJECT_0056C7F0, &args![tile_2]).u32() == 0 || !fn_007a1750(e) {
        return;
    }
    e.with_stack(0x60, |e, frame| {
        // The frame's locals: the player's position (+0), its north-rotated
        // copy (+0xC), the map corner (+0x18), the colour (+0x24), the
        // texture pointer (+0x34), a point (+0x38) and the zero rotation
        // matrix (+0x3C, 0x24 bytes).
        let position = frame.addr();
        let rotated = position + 0xc;
        let corner = position + 0x18;
        let color = position + 0x24;
        let texture = position + 0x34;
        let world = position + 0x38;
        local_map_step(e, save, [position, rotated, corner, color, texture, world]);
    });
}

/// The body of [`fn_0079ffb0`] with its locals' addresses.
fn local_map_step(e: &mut Engine, save: u8, locals: [u32; 6]) {
    let [position, rotated, corner, color, texture, world] = locals;
    let player = e.mem.u32(PLAYER_011DEA3C);
    let at = e.vcall(player, 0x1f4, &args![]).u32();
    for word in 0..3 {
        let value = e.mem.u32(at + 4 * word);
        e.mem.set_u32(position + 4 * word, value);
    }
    let lift: f64 = e.global(HEIGHT_OFFSET_DOUBLE);
    let z = x87_store32(x87_add(x87_load32(e.mem.u32(position + 8)), lift));
    e.mem.set_u32(position + 8, z);
    let player = e.mem.u32(PLAYER_011DEA3C);
    let interior = e.call(REFR_GET_INTERIOR, &args![player]).bool();
    let cell_size: i32 = if interior { 0x800 } else { 0x400 };
    let quads_per_side = 0x10i32;
    let quads = quads_per_side.wrapping_mul(quads_per_side);
    let step = cell_size / quads_per_side;
    let vertices_per_side = 0x11i32;
    let vertices = vertices_per_side.wrapping_mul(vertices_per_side);
    let side = local_grid_side(e);
    let span = cell_size.wrapping_mul(side);
    let uv_scale = x87_store32(e.call(LOCAL_MAP_UV_SCALE_00879D90, &args![]).f64());
    let mut saved = [0u32; 4];
    for (index, word) in saved.iter_mut().enumerate() {
        *word = e.mem.u32(RENDER_STATE_011FF3A8 + 4 * index as u32);
    }
    for index in 0..4 {
        let value = e.mem.u32(RENDER_STATE_SOURCE_011A9BD0 + 4 * index);
        e.mem.set_u32(RENDER_STATE_011FF3A8 + 4 * index, value);
    }
    let saved_flag = e.mem.u8(RENDER_FLAG_011FF103);
    e.mem.set_u8(RENDER_FLAG_011FF103, 1);
    let zero = 0.0f32.to_bits();
    e.call(COLOR_SET_00414430, &args![color, zero, zero, zero, zero]);
    let manager = e.call(SYSTEM_COLOR_MANAGER_GET_INSTANCE, &args![]).u32();
    e.call(SYSTEM_COLOR_MANAGER_GET_COLOR, &args![manager, 4u32, color]);
    let mut terrain_flag = 1u8;
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.call(REFR_GET_WORLD_SPACE, &args![player]).u32() != 0 && terrain_object(e) != 0 {
        let object = terrain_object(e);
        terrain_flag = e.call(OBJECT_FLAG_00456610, &args![object]).u8();
        let object = terrain_object(e);
        e.call(OBJECT_SET_FLAG_00450F90, &args![object, 1u32]);
    }
    let other = fn_007a1480(e);
    let other_flag = e.call(OBJECT_FLAG_00456610, &args![other]).u8();
    if other_flag == 0 {
        e.call(OBJECT_SET_FLAG_00450F90, &args![other, 1u32]);
    }
    let next = e.mem.u32(map_menu(e) + 0x128);
    let column_offset = next % local_grid_side(e) as u32;
    let next = e.mem.u32(map_menu(e) + 0x128);
    let row_offset = next / local_grid_side(e) as u32;
    e.call(WAVEFORM_LOCAL_CONSTRUCT_00633C90, &args![texture, 0u32]);
    let mut cell = 0u32;
    for word in 0..3 {
        let value = e.mem.u32(POINT_011F426C + 4 * word);
        e.mem.set_u32(corner + 4 * word, value);
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.call(REFR_GET_INTERIOR, &args![player]).bool() {
        let player = e.mem.u32(PLAYER_011DEA3C);
        cell = e.call(PLAYER_CELL_008D6F30, &args![player]).u32();
        if cell != 0 {
            for word in 0..3 {
                let value = e.mem.u32(position + 4 * word);
                e.mem.set_u32(rotated + 4 * word, value);
            }
            e.call(
                ADJUST_FOR_NORTH_ROTATION,
                &args![cell, position, rotated, 1u32],
            );
            let x = e
                .call(
                    FLOAT_TO_INT_00406D90,
                    &args![x87_quiet_bits(e.mem.u32(rotated))],
                )
                .i32();
            let cell_x = x.wrapping_sub(0x800) >> 0xc;
            let y = e
                .call(
                    FLOAT_TO_INT_00406D90,
                    &args![x87_quiet_bits(e.mem.u32(rotated + 4))],
                )
                .i32();
            let cell_y = y.wrapping_sub(0x800) >> 0xc;
            let column = cell_x
                .wrapping_add(column_offset as i32)
                .wrapping_sub(side >> 1);
            let row = cell_y
                .wrapping_add(row_offset as i32)
                .wrapping_sub(side >> 1);
            let point = point_words(
                e,
                x87_store32(f64::from((column << 0xc).wrapping_add(0x800))),
                x87_store32(f64::from((row << 0xc).wrapping_add(0x800))),
                zero,
            );
            for (index, word) in point.iter().enumerate() {
                e.mem.set_u32(corner + 4 * index as u32, *word);
            }
            e.call(CELL_GET_INT_SEEN_SECTION, &args![cell, column, row, 0u32]);
            let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
            e.call(RENDERER_BEGIN_004A0370, &args![renderer]);
            e.call(
                CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE,
                &args![cell, column, row, texture],
            );
            let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
            e.call(RENDERER_END_004A03C0, &args![renderer]);
            if column_offset == 0 && row_offset == 0 {
                let menu = map_menu(e);
                e.call(CELL_ADD_MARKERS_0054DCF0, &args![cell, menu + 0xe0]);
            }
        }
    } else {
        let player = e.mem.u32(PLAYER_011DEA3C);
        let space = e.call(REFR_GET_WORLD_SPACE, &args![player]).u32();
        let x = e
            .call(
                FLOAT_TO_INT_00406D90,
                &args![x87_quiet_bits(e.mem.u32(position))],
            )
            .i32();
        let cell_x = x >> 0xc;
        let y = e
            .call(
                FLOAT_TO_INT_00406D90,
                &args![x87_quiet_bits(e.mem.u32(position + 4))],
            )
            .i32();
        let cell_y = y >> 0xc;
        let column = cell_x
            .wrapping_add(column_offset as i32)
            .wrapping_sub(side >> 1);
        let row = cell_y
            .wrapping_add(row_offset as i32)
            .wrapping_sub(side >> 1);
        let point = point_words(
            e,
            x87_store32(f64::from(column << 0xc)),
            x87_store32(f64::from(row << 0xc)),
            zero,
        );
        for (index, word) in point.iter().enumerate() {
            e.mem.set_u32(corner + 4 * index as u32, *word);
        }
        if space != 0 {
            cell = e
                .call(WORLD_GET_CELL_FROM_CELL_COORD, &args![space, column, row])
                .u32();
        }
        if cell != 0 {
            e.call(EXTRA_GET_SEEN_DATA, &args![cell]);
            let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
            e.call(RENDERER_BEGIN_004A0370, &args![renderer]);
            e.call(CELL_GET_EXTERIOR_TEXTURE_0054E640, &args![cell, texture]);
            let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
            e.call(RENDERER_END_004A03C0, &args![renderer]);
            let menu = map_menu(e);
            e.call(CELL_ADD_MARKERS_0054DCF0, &args![cell, menu + 0xe0]);
        }
    }
    if save != 0 {
        save_local_map_texture(e, position, texture, column_offset, row_offset);
        let menu = map_menu(e);
        let next = e.mem.u32(menu + 0x128).wrapping_add(1);
        let menu = map_menu(e);
        e.mem.set_u32(menu + 0x128, next);
        e.call(WAVEFORM_LOCAL_DESTRUCT_0045CEC0, &args![texture]);
        return;
    }
    let positions = new_array(e, vertices as u32, 0xc, SELF);
    let normals = new_array(e, vertices as u32, 0xc, SELF);
    let uvs = new_array(e, vertices as u32, 8, SELF);
    let colors = new_array(e, vertices as u32, 0x10, COLOR_CONSTRUCTOR_004A7800);
    let indices = e
        .call(ALLOCATE_00AA1070, &args![(quads << 1).wrapping_mul(3) << 1])
        .u32();
    let two: f64 = e.global(TWO_DOUBLE);
    let mut index = 0u32;
    for across in 0..vertices_per_side {
        for down in 0..vertices_per_side {
            let half = x87_div(f64::from(cell_size), two);
            let x = x87_store32(x87_sub(f64::from(across.wrapping_mul(step)), half));
            let half = x87_div(f64::from(cell_size), two);
            let y = x87_store32(x87_sub(f64::from(down.wrapping_mul(step)), half));
            let point = point_words(e, y, zero, x);
            for (word, value) in point.iter().enumerate() {
                e.mem
                    .set_u32(positions + index * 0xc + 4 * word as u32, *value);
            }
            index = index.wrapping_add(1);
        }
    }
    let one = 1.0f32.to_bits();
    let mut index = 0u32;
    for across in 0..vertices_per_side {
        for down in 0..vertices_per_side {
            let normal = point_words(e, zero, one, zero);
            for (word, value) in normal.iter().enumerate() {
                e.mem
                    .set_u32(normals + index * 0xc + 4 * word as u32, *value);
            }
            let u = x87_store32(x87_sub(
                1.0,
                x87_div(f64::from(across), f64::from(vertices_per_side)),
            ));
            let v = x87_store32(x87_div(f64::from(down), f64::from(vertices_per_side)));
            let (first, second) = map_corner_bits(e, v, u);
            e.mem.set_u32(uvs + index * 8, first);
            e.mem.set_u32(uvs + index * 8 + 4, second);
            let world_x = x87_store32(x87_mul(f64::from(down), x87_m32(uv_scale)));
            let world_y = x87_store32(x87_mul(f64::from(across), x87_m32(uv_scale)));
            e.call(POINT_SET_00416870, &args![world, world_x, world_y, zero]);
            e.call(POINT_ADD_0063C8A0, &args![world, corner]);
            let mut alpha = float_constant(e, FOG_ALPHA_FLOAT);
            if e.mem.u8(map_menu(e) + 0x12c) != 0 {
                let seen = e.call(CELL_GET_SEEN_VALUE, &args![world]).i32();
                alpha = x87_store32(f64::from(seen));
            }
            let divisor: f64 = e.global(FOG_DIVISOR_DOUBLE);
            let alpha = x87_quiet_bits(x87_store32(x87_div(x87_load32(alpha), divisor)));
            let made = e.with_stack(0x10, |e, value| {
                let made = e
                    .call(COLOR_SET_00414430, &args![value, one, one, one, alpha])
                    .u32();
                [
                    e.mem.u32(made),
                    e.mem.u32(made + 4),
                    e.mem.u32(made + 8),
                    e.mem.u32(made + 0xc),
                ]
            });
            for (word, value) in made.iter().enumerate() {
                e.mem
                    .set_u32(colors + index * 0x10 + 4 * word as u32, *value);
            }
            index = index.wrapping_add(1);
        }
    }
    let mut index = 0u32;
    for across in 0..quads_per_side {
        for down in 0..quads_per_side {
            let row_start = across.wrapping_mul(vertices_per_side);
            let next_row = across.wrapping_add(1).wrapping_mul(vertices_per_side);
            // `AND 0x80000001` with the sign fix-up: the remainder by 2.
            let odd = (across % 2) ^ (down % 2) != 0;
            let corners = if odd {
                [
                    next_row.wrapping_add(down),
                    row_start.wrapping_add(down),
                    row_start.wrapping_add(down).wrapping_add(1),
                    row_start.wrapping_add(down).wrapping_add(1),
                    next_row.wrapping_add(down).wrapping_add(1),
                    next_row.wrapping_add(down),
                ]
            } else {
                [
                    next_row.wrapping_add(down).wrapping_add(1),
                    next_row.wrapping_add(down),
                    row_start.wrapping_add(down),
                    row_start.wrapping_add(down),
                    row_start.wrapping_add(down).wrapping_add(1),
                    next_row.wrapping_add(down).wrapping_add(1),
                ]
            };
            for value in corners {
                e.mem.set_u16(indices + index * 2, value as u16);
                index = index.wrapping_add(1);
            }
        }
    }
    let memory = e.call(ALLOCATE_00AA13E0, &args![0xd4u32]).u32();
    let shape = if memory != 0 {
        let height = e.call(SCREEN_HEIGHT_00715DA0, &args![]).f64();
        let height = e.call(FTOL2, &args![height]).u32();
        let width = e.call(SCREEN_WIDTH_00715D40, &args![]).f64();
        let width = e.call(FTOL2, &args![width]).u32();
        e.call(
            SCISSOR_TRI_SHAPE_CONSTRUCT,
            &args![
                memory,
                vertices as u16 as u32,
                positions,
                normals,
                colors,
                uvs,
                1u32,
                0u32,
                (quads << 1) as u32,
                indices,
                0u32,
                0u32,
                width,
                height
            ],
        )
        .u32()
    } else {
        0
    };
    let memory = e.call(ALLOCATE_00AA13E0, &args![0xb0u32]).u32();
    let property = if memory != 0 {
        e.call(TILE_SHADER_PROPERTY_CONSTRUCT, &args![memory, 0u32])
            .u32()
    } else {
        0
    };
    let picture = e.call(STRING_TEXT, &args![texture]).u32();
    e.call(SET_TILE_TEXTURE, &args![property, picture]);
    e.call(PROPERTY_SET_008CC6D0, &args![property, 0u32]);
    fn_007a1450(e, Ptr::new(property), color);
    fn_007a1430(e, Ptr::new(property), 1);
    e.call(TEXTURE_POINTER_SET_0066B0D0, &args![texture, 0u32]);
    e.call(AV_OBJECT_ATTACH_PROPERTY, &args![shape, property]);
    let memory = e.call(ALLOCATE_00AA13E0, &args![0x1cu32]).u32();
    let alpha = if memory != 0 {
        e.call(ALPHA_PROPERTY_CONSTRUCT_004391C0, &args![memory])
            .u32()
    } else {
        0
    };
    e.call(ALPHA_PROPERTY_SET_00439340, &args![alpha, 6u32]);
    e.call(ALPHA_PROPERTY_SET_00439390, &args![alpha, 0u32]);
    e.call(ALPHA_PROPERTY_SET_0049ED90, &args![alpha, 1u32]);
    e.call(AV_OBJECT_ATTACH_PROPERTY, &args![shape, alpha]);
    e.call(SHADER_MANAGER_PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    let half = x87_div(f64::from(cell_size), two);
    let along = x87_add(f64::from(cell_size.wrapping_mul(row_offset as i32)), half);
    let along = x87_quiet_bits(x87_store32(x87_sub(along, f64::from(span))));
    let half = x87_div(f64::from(cell_size), two);
    let across = x87_add(
        f64::from(cell_size.wrapping_mul(column_offset as i32)),
        half,
    );
    let across = x87_quiet_bits(x87_store32(across));
    e.call(
        NODE_SET_TRANSLATE_004BC1F0,
        &args![shape, across, zero, along],
    );
    e.call(AV_OBJECT_UPDATE_PROPERTIES, &args![shape]);
    let tile_2 = map_tile(e, map_menu(e), 2);
    let node = e.call(TILE_OBJECT_0056C7F0, &args![tile_2]).u32();
    e.vcall(node, 0xdc, &args![shape, 1u32]);
    let tile_2 = map_tile(e, map_menu(e), 2);
    fn_007a1400(e, Ptr::new(tile_2), 0x20);
    for (index, word) in saved.iter().enumerate() {
        e.mem
            .set_u32(RENDER_STATE_011FF3A8 + 4 * index as u32, *word);
    }
    e.mem.set_u8(RENDER_FLAG_011FF103, saved_flag);
    let player = e.mem.u32(PLAYER_011DEA3C);
    if terrain_flag == 0 && e.call(REFR_GET_WORLD_SPACE, &args![player]).u32() != 0 {
        let object = terrain_object(e);
        e.call(
            OBJECT_SET_FLAG_00450F90,
            &args![object, u32::from(terrain_flag)],
        );
    }
    if other_flag == 0 {
        e.call(OBJECT_SET_FLAG_00450F90, &args![other, 0u32]);
    }
    let tile_2 = map_tile(e, map_menu(e), 2);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile_2, 0x108u32]);
    for id in [0xfcdu32, 0xfce] {
        let tile_2 = map_tile(e, map_menu(e), 2);
        e.call(
            TILE_SET_INT,
            &args![tile_2, id, cell_size.wrapping_mul(side)],
        );
    }
    let full: f64 = e.global(FULL_ALPHA_DOUBLE);
    for (index, id) in [0xfb2u32, 0xfb3, 0xfb4].iter().enumerate() {
        let channel = e.mem.u32(color + 4 * index as u32);
        let value = x87_quiet_bits(x87_store32(x87_mul(x87_load32(channel), full)));
        let tile_2 = map_tile(e, map_menu(e), 2);
        e.call(TILE_SET_FLOAT, &args![tile_2, *id, value, 1u32]);
    }
    e.with_stack(0x24, |e, matrix| {
        e.call(MATRIX_SET_0043D410, &args![matrix, zero, 0u32, 0u32]);
        let tile_2 = map_tile(e, map_menu(e), 2);
        let node = e.call(TILE_OBJECT_0056C7F0, &args![tile_2]).u32();
        e.call(NODE_SET_ROTATE_00A59C60, &args![node, matrix]);
    });
    let menu = map_menu(e);
    let next = e.mem.u32(menu + 0x128).wrapping_add(1);
    let menu = map_menu(e);
    e.mem.set_u32(menu + 0x128, next);
    if !fn_007a1750(e) {
        let menu = map_menu(e);
        if e.mem.u32(menu + 0x84) == map_tile(e, menu, 2) {
            map_menu_update_local_map(e, Ptr::new(menu));
        }
    }
    e.call(WAVEFORM_LOCAL_DESTRUCT_0045CEC0, &args![texture]);
}

/// `0079ffb0`'s saving branch: creates `(LocalMaps)`, its `Interiors` and
/// `Exteriors` directories and the directory of the current cell (an
/// interior's, `005f36f0` on `011dea10`, else the world space's,
/// `TES::GetWorldSpace`; names through vtable +0x130), then saves the
/// texture (`00653270` of its `D3DTexture_LockRect`, vtable +0x9C, no stack
/// word) as `...\%s\%s.%i.%i.png` with the grid offsets (an interior) or the
/// player's cell plus the offsets (an exterior), format 3, palette 0.
fn save_local_map_texture(
    e: &mut Engine,
    position: u32,
    texture: u32,
    column_offset: u32,
    row_offset: u32,
) {
    for directory in [
        LOCAL_MAPS_DIRECTORY,
        LOCAL_MAPS_INTERIORS_DIRECTORY,
        LOCAL_MAPS_EXTERIORS_DIRECTORY,
    ] {
        e.call(CREATE_DIRECTORY_IMPORT, &args![directory, 0u32]);
    }
    e.with_stack(0x208, |e, buffers| {
        let directory = buffers.addr();
        let file = directory + 0x104;
        let owner = e.mem.u32(OBJECT_011DEA10);
        if e.call(OBJECT_CELL_005F36F0, &args![owner]).u32() != 0 {
            let owner = e.mem.u32(OBJECT_011DEA10);
            let cell = e.call(OBJECT_CELL_005F36F0, &args![owner]).u32();
            let name = e.vcall(cell, 0x130, &args![]).u32();
            e.call(
                BUFFER_FORMAT,
                &args![directory, 0x104u32, INTERIOR_DIRECTORY_FORMAT, name],
            );
            e.call(CREATE_DIRECTORY_IMPORT, &args![directory, 0u32]);
            let owner = e.mem.u32(OBJECT_011DEA10);
            let first = e.call(OBJECT_CELL_005F36F0, &args![owner]).u32();
            let owner = e.mem.u32(OBJECT_011DEA10);
            let second = e.call(OBJECT_CELL_005F36F0, &args![owner]).u32();
            let first_name = e.vcall(first, 0x130, &args![]).u32();
            let second_name = e.vcall(second, 0x130, &args![]).u32();
            e.call(
                BUFFER_FORMAT,
                &args![
                    file,
                    0x104u32,
                    INTERIOR_FILE_FORMAT,
                    second_name,
                    first_name,
                    column_offset,
                    row_offset
                ],
            );
        } else {
            let owner = e.mem.u32(OBJECT_011DEA10);
            let space = e.call(TES_GET_WORLD_SPACE, &args![owner]).u32();
            let name = e.vcall(space, 0x130, &args![]).u32();
            e.call(
                BUFFER_FORMAT,
                &args![directory, 0x104u32, EXTERIOR_DIRECTORY_FORMAT, name],
            );
            e.call(CREATE_DIRECTORY_IMPORT, &args![directory, 0u32]);
            let owner = e.mem.u32(OBJECT_011DEA10);
            let first = e.call(TES_GET_WORLD_SPACE, &args![owner]).u32();
            let owner = e.mem.u32(OBJECT_011DEA10);
            let second = e.call(TES_GET_WORLD_SPACE, &args![owner]).u32();
            let y = e
                .call(
                    FLOAT_TO_INT_00406D90,
                    &args![x87_quiet_bits(e.mem.u32(position + 4))],
                )
                .i32();
            let row = (y >> 0xc).wrapping_add(row_offset as i32);
            let x = e
                .call(
                    FLOAT_TO_INT_00406D90,
                    &args![x87_quiet_bits(e.mem.u32(position))],
                )
                .i32();
            let column = (x >> 0xc).wrapping_add(column_offset as i32);
            let first_name = e.vcall(first, 0x130, &args![]).u32();
            let second_name = e.vcall(second, 0x130, &args![]).u32();
            e.call(
                BUFFER_FORMAT,
                &args![
                    file,
                    0x104u32,
                    EXTERIOR_FILE_FORMAT,
                    second_name,
                    first_name,
                    column,
                    row
                ],
            );
        }
        let picture = e.call(STRING_TEXT, &args![texture]).u32();
        let surface = e.call(D3D_TEXTURE_LOCK_RECT, &args![picture]).u32();
        let object = e
            .call(
                TEXTURE_LOOKUP_00653270,
                &args![TEXTURE_TABLE_011F4418, surface],
            )
            .u32();
        // 007a0775: PUSH 0 is the import's fourth word (the palette); the slot (+0x9c,
        // 00e68800: MOV EAX,[ECX+0x64]; RET) reads no stack word (007a077d..007a078b).
        let target = e.vcall(object, 0x9c, &args![]).u32();
        // 007a078d..007a0797: PUSH EAX; PUSH 3; PUSH the file; CALL 00ee6dc2 (four words).
        e.call(SAVE_TEXTURE_JUMP_00EE6DC2, &args![file, 3u32, target, 0u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007a_1aa0, fn_007a1aa0(Ptr)),
        entry!(
            0x0072_d040,
            map_menu_attach_tile_by_id(Ptr<MapMenu>, i32, u32)
        ),
        entry!(0x0073_12e0, list_box_bgs_note_save_list_state(Ptr<ListBox>)),
        entry!(0x0079_6740, map_menu_map_menu(Ptr<MapMenu>) -> Ptr<MapMenu>),
        entry!(0x0079_6a40, map_menu_get_class(Ptr<MapMenu>) -> u32),
        entry!(
            0x0079_6a50,
            map_menu_scalar_deleting_destructor(Ptr<MapMenu>, u32) -> Ptr<MapMenu>
        ),
        entry!(0x0079_6a80, fn_00796a80(Ptr<MapMenu>)),
        entry!(0x0079_6b90, map_menu_create() -> u32),
        entry!(0x0079_6f40, fn_00796f40(Ptr, u8)),
        entry!(0x0079_6f90, fn_00796f90(Ptr<MapMenu>) -> bool),
        entry!(0x0079_6fd0, map_menu_do_click(Ptr<MapMenu>, i32, u32)),
        entry!(0x0079_8450, fn_00798450(Ptr, u32) -> Ptr),
        entry!(0x0079_a130, map_menu_do_idle(Ptr<MapMenu>)),
        entry!(0x0079_aba0, map_menu_update_menu()),
        entry!(0x0079_bc30, fn_0079bc30(Ptr) -> u32),
        entry!(0x0079_c090, fn_0079c090(Ptr<MapMenu>)),
        entry!(0x007a_1480, fn_007a1480() -> u32),
        entry!(0x007a_1490, map_menu_clear_map_memory()),
        entry!(0x007a_1670, map_menu_clear_world_map_texture()),
        entry!(0x007a_17a0, fn_007a17a0() -> f32),
        entry!(0x007a_17e0, fn_007a17e0() -> f32),
        entry!(0x007a_1880, map_menu_set_root_map_space(u32)),
        entry!(0x007a_1910, fn_007a1910(Ptr<ListBox>) -> u32),
        entry!(0x007a_1930, fn_007a1930(Ptr<ListBox>, u8)),
        entry!(0x007a_1950, fn_007a1950(Ptr<ListBox>, u8, u8)),
        entry!(
            0x007a_1a70,
            fn_007a1a70(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x007a_1c30, fn_007a1c30(Ptr<BSSimpleArray>, u8)),
        entry!(0x007a_62e0, fn_007a62e0(Ptr, u32, u32) -> Ptr),
        entry!(0x007a_63f0, fn_007a63f0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x007a_1c90, fn_007a1c90(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_1d60, fn_007a1d60(Ptr<ListBox>)),
        entry!(0x007a_2070, list_box_tes_quest_remove_all(Ptr<ListBox>)),
        entry!(
            0x007a_23e0,
            list_box_tes_quest_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_6550,
            list_box_tes_quest_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_6600,
            list_box_tes_quest_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_6920,
            list_box_tes_quest_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_27f0, fn_007a27f0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_28c0, fn_007a28c0(Ptr<ListBox>)),
        entry!(0x007a_2bd0, list_box_bgs_note_remove_all(Ptr<ListBox>)),
        entry!(
            0x007a_3170,
            list_box_bgs_note_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_6bd0,
            list_box_bgs_note_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_6c80,
            list_box_bgs_note_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_7080,
            list_box_bgs_note_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_36e0, fn_007a36e0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_37b0, fn_007a37b0(Ptr<ListBox>)),
        entry!(
            0x007a_3ac0,
            list_box_tes_object_refr_remove_all(Ptr<ListBox>)
        ),
        entry!(
            0x007a_3f90,
            list_box_tes_object_refr_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_7330,
            list_box_tes_object_refr_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_73e0,
            list_box_tes_object_refr_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_7700,
            list_box_tes_object_refr_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_43a0, fn_007a43a0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_4470, fn_007a4470(Ptr<ListBox>)),
        entry!(
            0x007a_4780,
            list_box_bgs_quest_objective_remove_all(Ptr<ListBox>)
        ),
        entry!(
            0x007a_4ce0,
            list_box_bgs_quest_objective_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_79b0,
            list_box_bgs_quest_objective_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_7a60,
            list_box_bgs_quest_objective_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_7d80,
            list_box_bgs_quest_objective_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007a_50f0, fn_007a50f0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007a_51c0, fn_007a51c0(Ptr<ListBox>)),
        entry!(0x007a_54d0, list_box_tes_challenge_remove_all(Ptr<ListBox>)),
        entry!(
            0x007a_5a60,
            list_box_tes_challenge_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007a_81b0,
            list_box_tes_challenge_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007a_8260,
            list_box_tes_challenge_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007a_8580,
            list_box_tes_challenge_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x0079_83c0, fn_007983c0(Ptr) -> f32),
        entry!(0x0079_8400, fn_00798400(Ptr) -> u32),
        entry!(0x0079_8420, fn_00798420(Ptr) -> u8),
        entry!(0x0079_8440, fn_00798440() -> u8),
        entry!(0x0079_8480, map_menu_do_down_click(Ptr<MapMenu>, i32, u32)),
        entry!(0x0079_a610, map_menu_do_controller_switch(Ptr<MapMenu>, u8)),
        entry!(0x0079_b430, fn_0079b430(u32)),
        entry!(0x0079_b800, fn_0079b800(Ptr) -> bool),
        entry!(0x0079_bc50, fn_0079bc50(u32) -> u8),
        entry!(0x0079_be30, fn_0079be30(u32) -> u8),
        entry!(0x0079_be80, fn_0079be80(u32)),
        entry!(0x0079_c070, fn_0079c070() -> u32),
        entry!(0x0079_c080, fn_0079c080() -> u32),
        entry!(0x0079_c340, fn_0079c340(u8)),
        entry!(0x007a_1400, fn_007a1400(Ptr, u32)),
        entry!(0x007a_1430, fn_007a1430(Ptr, u8)),
        entry!(0x007a_1450, fn_007a1450(Ptr, u32)),
        entry!(0x007a_1750, fn_007a1750() -> bool),
        entry!(0x007a_1820, fn_007a1820(f32)),
        entry!(0x007a_1850, fn_007a1850(f32)),
        entry!(0x007a_1ac0, fn_007a1ac0(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(0x007a_6100, fn_007a6100(Ptr, u32) -> Ptr),
        entry!(0x007a_6130, fn_007a6130(Ptr<ListBox>, u32) -> Ptr<ListBox>),
        entry!(0x007a_6160, fn_007a6160(Ptr<ListBox>, u32) -> Ptr<ListBox>),
        entry!(0x007a_6190, fn_007a6190(Ptr<ListBox>, u32) -> Ptr<ListBox>),
        entry!(0x007a_61c0, fn_007a61c0(Ptr<ListBox>, u32) -> Ptr<ListBox>),
        entry!(0x007a_61f0, fn_007a61f0(Ptr<ListBox>, u32) -> Ptr<ListBox>),
        entry!(0x007a_6310, fn_007a6310(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x007a_63b0, fn_007a63b0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x007a_86b0, fn_007a86b0(Ptr, u32) -> Ptr),
        entry!(0x007a_5a30, fn_007a5a30(Ptr<ListBox>, u8)),
        entry!(0x007a_22a0, list_box_tes_quest_sort(Ptr<ListBox>, u32)),
        entry!(0x007a_2e00, list_box_bgs_note_sort(Ptr<ListBox>, u32)),
        entry!(
            0x007a_3cf0,
            list_box_tes_object_refr_sort(Ptr<ListBox>, u32)
        ),
        entry!(
            0x007a_49b0,
            list_box_bgs_quest_objective_sort(Ptr<ListBox>, u32)
        ),
        entry!(0x007a_5700, list_box_tes_challenge_sort(Ptr<ListBox>, u32)),
        entry!(
            0x007a_2350,
            list_box_tes_quest_set_enabled(Ptr<ListBox>, u8)
        ),
        entry!(0x007a_30e0, list_box_bgs_note_set_enabled(Ptr<ListBox>, u8)),
        entry!(
            0x007a_4c50,
            list_box_bgs_quest_objective_set_enabled(Ptr<ListBox>, u8)
        ),
        entry!(
            0x007a_59a0,
            list_box_tes_challenge_set_enabled(Ptr<ListBox>, u8)
        ),
        entry!(
            0x007a_2450,
            list_box_tes_quest_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x007a_31e0,
            list_box_bgs_note_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x007a_4000,
            list_box_tes_object_refr_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x007a_4d50,
            list_box_bgs_quest_objective_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x007a_5ad0,
            list_box_tes_challenge_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x007a_1dd0,
            fn_007a1dd0(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x007a_2930,
            fn_007a2930(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x007a_3820,
            fn_007a3820(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x007a_44e0,
            fn_007a44e0(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x007a_5230,
            fn_007a5230(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x007a_2e30, list_box_bgs_note_filter(Ptr<ListBox>, u32)),
        entry!(
            0x007a_3d20,
            list_box_tes_object_refr_filter(Ptr<ListBox>, u32)
        ),
        entry!(
            0x007a_49e0,
            list_box_bgs_quest_objective_filter(Ptr<ListBox>, u32)
        ),
        entry!(
            0x007a_5730,
            list_box_tes_challenge_filter(Ptr<ListBox>, u32)
        ),
        entry!(0x007a_6220, fn_007a6220(Ptr<ListBox>) -> u32),
        entry!(0x007a_6250, fn_007a6250(Ptr<ListBox>) -> u32),
        entry!(0x007a_6280, fn_007a6280(Ptr<ListBox>) -> u32),
        entry!(0x007a_62b0, fn_007a62b0(Ptr<ListBox>) -> u32),
        entry!(0x007a_6480, list_box_tes_quest_synchronize(Ptr<ListBox>)),
        entry!(0x007a_6b00, list_box_bgs_note_synchronize(Ptr<ListBox>)),
        entry!(
            0x007a_7260,
            list_box_tes_object_refr_synchronize(Ptr<ListBox>)
        ),
        entry!(
            0x007a_78e0,
            list_box_bgs_quest_objective_synchronize(Ptr<ListBox>)
        ),
        entry!(
            0x007a_80e0,
            list_box_tes_challenge_synchronize(Ptr<ListBox>)
        ),
        entry!(0x007a_6a50, fn_007a6a50(Ptr<ListBox>) -> u32),
        entry!(
            0x007a_71b0,
            list_box_tes_object_refr_num_unfiltered(Ptr<ListBox>) -> u32
        ),
        entry!(0x007a_7830, fn_007a7830(Ptr<ListBox>) -> u32),
        entry!(
            0x007a_8030,
            list_box_tes_challenge_num_unfiltered(Ptr<ListBox>) -> u32
        ),
        entry!(
            0x007a_86e0,
            list_box_tes_quest_num_unfiltered(Ptr<ListBox>) -> u32
        ),
        entry!(0x007a_3580, fn_007a3580(Ptr<ListBox>, u8)),
        entry!(
            0x0079_84d0,
            map_menu_calculate_player_marker_space(Ptr<MapMenu>, u32, u32, u32) -> u32
        ),
        entry!(0x0079_8710, fn_00798710()),
        entry!(0x0079_8840, map_menu_set_player_marker_callback()),
        entry!(0x0079_8a00, fn_00798a00()),
        entry!(0x0079_8ad0, map_menu_stop_note_sound(Ptr<MapMenu>)),
        entry!(0x0079_8bf0, map_menu_clear_audio_log_subtites()),
        entry!(0x0079_8cb0, map_menu_do_enter(Ptr<MapMenu>, i32, u32)),
        entry!(0x0079_9100, fn_00799100(u32, u32)),
        entry!(0x0079_91b0, map_menu_load_challenge(Ptr<MapMenu>, u32)),
        entry!(0x0079_93d0, map_menu_load_note(Ptr<MapMenu>, u32)),
        entry!(0x0079_9690, map_menu_do_leave(Ptr<MapMenu>, i32, u32)),
        entry!(
            0x0079_9790,
            map_menu_do_gamepad(Ptr<MapMenu>, i32, u32) -> bool
        ),
        entry!(0x0079_9dc0, fn_00799dc0(Ptr<MapMenu>)),
        entry!(0x0079_a660, map_menu_update_playing_sounds(Ptr<MapMenu>)),
        entry!(0x007a_1b10, fn_007a1b10(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x0079_af30, map_menu_update_current_page(Ptr<MapMenu>)),
        entry!(0x0079_b240, map_menu_update_notes(Ptr<MapMenu>)),
        entry!(0x0079_b390, fn_0079b390(u32) -> u8),
        entry!(
            0x0079_b460,
            map_menu_toggle_notes_challenge_mode(Ptr<MapMenu>, u8)
        ),
        entry!(0x0079_b600, fn_0079b600(Ptr<MapMenu>)),
        entry!(0x0079_b720, fn_0079b720(u32) -> u8),
        entry!(0x0079_b820, fn_0079b820(u32, u32) -> i32),
        entry!(0x0079_b8e0, map_menu_update_quests(Ptr<MapMenu>)),
        entry!(0x0079_bc90, fn_0079bc90(u32, u32) -> i32),
        entry!(0x0079_bd70, fn_0079bd70(u32, u32) -> i32),
        entry!(0x0079_bea0, map_menu_update_radio_list(Ptr<MapMenu>)),
        entry!(0x0079_c0c0, fn_0079c0c0(Ptr<MapMenu>)),
        entry!(
            0x0079_c380,
            map_menu_world_to_map_coord(Ptr<MapMenu>, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x0079_c450,
            map_menu_map_to_world_coord(Ptr<MapMenu>, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0079_c530, map_menu_do_wheel_move(Ptr<MapMenu>, u32, u32)),
        entry!(0x0079_c5a0, map_menu_zoom_map(Ptr<MapMenu>, u8, f32)),
        entry!(0x0079_cdb0, map_menu_update_world_map(Ptr<MapMenu>)),
        entry!(0x0079_d410, map_menu_update_local_map(Ptr<MapMenu>)),
        entry!(0x0079_dbb0, map_menu_add_map_icons(Ptr<MapMenu>)),
        entry!(0x0079_e0a0, map_menu_add_quest_markers(Ptr<MapMenu>)),
        entry!(0x0079_e470, map_menu_update_companion_markers(Ptr<MapMenu>)),
        entry!(0x0079_ea30, map_menu_add_companion_markers(Ptr<MapMenu>)),
        entry!(0x0079_f360, map_menu_create_player_marker(Ptr<MapMenu>)),
        entry!(0x0079_f620, fn_0079f620(Ptr) -> u32),
        entry!(0x0079_f640, map_menu_set_player_marker()),
        entry!(0x0079_f6f0, map_menu_delete_player_marker(Ptr<MapMenu>)),
        entry!(
            0x0079_f7e0,
            map_menu_get_world_map_tracking_ref(Ptr<MapMenu>, u32, u32) -> u32
        ),
        entry!(0x007a_19a0, fn_007a19a0(Ptr, u32)),
        entry!(
            0x0079_f8d0,
            map_menu_create_waypoint_path(Ptr<MapMenu>, u32, u32, u32, u32)
        ),
        entry!(0x0079_ffb0, fn_0079ffb0(u8)),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_007a1aa0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x007a_1aa0, ARRAY_VTABLE_01075530, ARRAY_RELEASE_007A1C30);
    }
}

#[cfg(test)]
mod map_menu_tests {
    use super::*;

    /// An engine with the pages of the statics and constants the map menu
    /// code reads mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_a000,
            0x011d_e000,
            0x011d_8000,
            0x011d_1000,
            0x011d_2000,
            0x011d_3000,
            0x011d_4000,
            0x011d_5000,
            0x011c_3000,
            0x011f_4000,
            0x011f_6000,
            0x011a_0000,
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_e000,
            0x0102_3000,
            0x0103_0000,
            0x0107_0000,
            0x0107_4000,
        ] {
            e.map(page, 0x1000);
        }
        e
    }

    /// Doubles that do nothing and return 0.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// A double returning `value` in `EAX`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double returning `value` in `ST0`.
    fn returns_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn called(e: &Engine, addr: u32) -> bool {
        !calls_to(e, addr).is_empty()
    }

    /// A map menu object with `pTiles` filled with distinct tiles
    /// (`tile_base + index`).
    fn menu_with_tiles(e: &mut Engine, tile_base: u32) -> Ptr<MapMenu> {
        let menu: Ptr<MapMenu> = e.new_object();
        for index in 0..0x16 {
            e.mem
                .set_u32(menu.addr() + 0x28 + 4 * index, tile_base + index);
        }
        menu
    }

    /// A vtable at `addr` whose slots are `slots`, and an object using it.
    fn object_with_vtable(e: &mut Engine, addr: u32, slots: &[u32]) -> u32 {
        e.put_vtable(addr, slots);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, addr);
        object
    }

    #[test]
    fn save_local_map_texture_hands_the_save_import_four_words() {
        // 007a0775: PUSH 0 is the import's fourth word (the palette); the texture data's
        // vtable +0x9c (00e68800: MOV EAX,[ECX+0x64]; RET) takes no stack word; 007a078d..
        // 007a0796 push the texture, 3 and the file; 00ee6dc2 jumps to the import with all four.
        let mut e = engine();
        let cell = object_with_vtable(&mut e, 0x0107_9000, &[0x0050_0000; 0x4d]);
        e.mem.set_u32(0x0107_9130, 0x0050_0130);
        returns(&mut e, 0x0050_0130, 0x6000);
        let data = object_with_vtable(&mut e, 0x0107_a000, &[0x0050_0000; 0x28]);
        e.mem.set_u32(0x0107_a09c, 0x0050_009c);
        returns(&mut e, 0x0050_009c, 0x6300);
        returns(&mut e, OBJECT_CELL_005F36F0, cell);
        returns(&mut e, STRING_TEXT, 0x6100);
        returns(&mut e, D3D_TEXTURE_LOCK_RECT, 0x6200);
        returns(&mut e, TEXTURE_LOOKUP_00653270, data);
        quiet(
            &mut e,
            &[CREATE_DIRECTORY_IMPORT, BUFFER_FORMAT, 0x00fd_f3d4],
        );
        e.call_log = Some(vec![]);
        save_local_map_texture(&mut e, 0, 0x6400, 1, 2);
        let file = calls_to(&e, BUFFER_FORMAT)[1][0];
        assert_eq!(calls_to(&e, 0x0050_009c), vec![vec![data]]);
        assert_eq!(calls_to(&e, 0x00fd_f3d4), vec![vec![file, 3, 0x6300, 0]]);
    }

    #[test]
    fn attach_tile_by_id_stores_only_ids_in_range() {
        let mut e = engine();
        let menu: Ptr<MapMenu> = e.new_object();
        e.call(0x0072_d040, &args![menu, 3i32, 0x1234u32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x28 + 12), 0x1234);
        e.call(0x0072_d040, &args![menu, -1i32, 0x5555u32]);
        e.call(0x0072_d040, &args![menu, 0x16i32, 0x5555u32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x24), 0);
        assert_eq!(e.mem.u32(menu.addr() + 0x80), 0);
    }

    #[test]
    fn save_list_state_reads_the_index_and_scroll_value() {
        let mut e = engine();
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0xfac { 4.0 } else { 7.5 },
            ..Ret::default()
        });
        returns(&mut e, TEXT_TO_TRAIT, 0x77);
        let list: Ptr<ListBox> = e.new_object();
        e.call(0x0073_12e0, &args![list]);
        assert_eq!(e.mem.f32(list.addr() + 0x24), -1.0);
        assert_eq!(e.mem.f32(list.addr() + 0x28), -1.0);
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x9000));
        e.set(list, ListBox::pScrollBar, Ptr::new(0x9100));
        e.call_log = Some(vec![]);
        e.call(0x0073_12e0, &args![list]);
        assert_eq!(e.mem.f32(list.addr() + 0x24), 4.0);
        assert_eq!(e.mem.f32(list.addr() + 0x28), 7.5);
        assert_eq!(
            calls_to(&e, TILE_GET_FLOAT),
            vec![vec![0x9000, 0xfac], vec![0x9100, 0x77]]
        );
    }

    /// Doubles for every callee of the constructor.
    fn constructor_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                MENU_CONSTRUCT,
                SOUND_LIST_CONSTRUCT,
                LIST_CONSTRUCT,
                SELF,
                WAYPOINTS_CONSTRUCT,
                LIST_BOX_BASE_CONSTRUCT,
                LIST_BOX_REGISTER,
                ARRAY_CONSTRUCT_ELEMENTS_007A6310,
            ],
        );
        e.register(TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        returns(e, AUDIO_INSTANCE, 0x5000);
        returns_float(e, AUDIO_GET_RADIO_VOLUME, 0.25);
    }

    #[test]
    fn constructor_builds_the_members_and_the_start_values() {
        let mut e = engine();
        constructor_doubles(&mut e);
        e.set_global(POINT_011F426C, 1.0f32);
        e.set_global(POINT_011F426C + 4, 2.0f32);
        e.set_global(POINT_011F426C + 8, 3.0f32);
        let menu: Ptr<MapMenu> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6740, &args![menu]).u32(), menu.addr());
        let base = menu.addr();
        assert_eq!(e.mem.u32(base), MAP_MENU_VTABLE);
        assert_eq!(e.mem.u32(base + 0xa8), ARRAY_VTABLE_01075530);
        for (offset, vtable) in [
            (0x130u32, QUEST_LIST.vtable),
            (0x160, NOTE_LIST.vtable),
            (0x190, REFERENCE_LIST.vtable),
            (0x1c0, OBJECTIVE_LIST.vtable),
            (0x1f0, CHALLENGE_LIST.vtable),
        ] {
            assert_eq!(e.mem.u32(base + offset), vtable);
        }
        assert_eq!(
            calls_to(&e, LIST_CONSTRUCT)[..2],
            [vec![base + 0xd8], vec![base + 0xe0]]
        );
        assert_eq!(e.mem.u32(TRAIT_TITLE), TITLE_TEXT & 0xffff);
        assert_eq!(e.mem.u32(TRAIT_MAGNIFICATION), MAGNIFICATION_TEXT & 0xffff);
        assert_eq!(e.get(menu, MapMenu::iCurrentPage), 0x24);
        assert_eq!(e.get(menu, MapMenu::bShowingAllNotes), 1);
        assert_eq!(e.get(menu, MapMenu::bFogOfWarEnabled), 1);
        assert_eq!(e.get(menu, MapMenu::fOldPipboyRadioVolume), 0.25);
        assert_eq!(e.mem.f32(base + 0x100), 2.0);
        assert_eq!(calls_to(&e, AUDIO_GET_RADIO_VOLUME), vec![vec![0x5000]]);
    }

    #[test]
    fn get_class_is_the_map_menu_id() {
        let mut e = engine();
        assert_eq!(e.call(0x0079_6a40, &args![0u32]).u32(), 0x3ff);
    }

    /// Doubles for every callee of the destructor body.
    fn destructor_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                WAYPOINTS_DESTRUCT,
                LIST_DESTRUCT,
                SOUND_LIST_DESTRUCT,
                MENU_DESTRUCT,
                LIST_BOX_UNREGISTER,
                TEXT_TO_TRAIT,
                ADD_USER_TRAIT,
                ARRAY_RELEASE_007A1C30,
                FREE,
            ],
        );
    }

    #[test]
    fn destructor_body_clears_the_singleton_and_destroys_the_members() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let menu: Ptr<MapMenu> = e.new_object();
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.call_log = Some(vec![]);
        e.call(0x0079_6a80, &args![menu]);
        let base = menu.addr();
        assert_eq!(e.mem.u32(MAP_MENU_SINGLETON), 0);
        assert_eq!(e.mem.u32(base), MAP_MENU_VTABLE);
        assert_eq!(calls_to(&e, WAYPOINTS_DESTRUCT), vec![vec![base + 0x220]]);
        assert_eq!(
            calls_to(&e, LIST_BOX_UNREGISTER),
            vec![
                vec![base + 0x1f0],
                vec![base + 0x1c0],
                vec![base + 0x190],
                vec![base + 0x160],
                vec![base + 0x130]
            ]
        );
        let lists = calls_to(&e, LIST_DESTRUCT);
        assert_eq!(
            lists[lists.len() - 2..],
            [vec![base + 0xe0], vec![base + 0xd8]]
        );
        assert_eq!(
            calls_to(&e, ARRAY_RELEASE_007A1C30),
            vec![vec![base + 0xa8, 1]]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCT), vec![vec![base]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_flag_one() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let menu: Ptr<MapMenu> = e.new_object();
        for (flags, frees) in [(0u32, false), (1, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0079_6a50, &args![menu, flags]).u32(), menu.addr());
            assert_eq!(called(&e, FREE), frees);
            assert!(called(&e, MENU_DESTRUCT));
        }
    }

    /// Doubles for `Create`: the menu read from the file is `menu` (its
    /// vtable slot 0x34 returns `class`).
    fn create_doubles(e: &mut Engine, menu: u32, class: u32) {
        let vtable = 0x0107_4d00;
        e.put_vtable(vtable, &[0x0050_0000; 0x10]);
        e.mem.set_u32(menu, vtable);
        e.mem.set_u32(vtable + 0x34, 0x0050_0034);
        returns(e, 0x0050_0034, class);
        quiet(
            e,
            &[
                0x0050_0000,
                MENU_REMOVE_FROM_XML_ARCHIVE,
                CALENDAR_MARK_STALE,
                MENU_SET_MENU_TILE,
                PRINT_WARNING,
                TILE_SET_FLOAT,
                TUTORIAL_SHOW_MESSAGE,
                OBJECT_FIELD_00403DF0,
                INITIALIZE_TABLINE,
                LIST_BOX_INITIALIZE,
                TILE_OBJECT_0056C7F0,
                TILE_SET_STRING,
                MENU_PREP_FOR_VISIBILITY,
                ADD_USER_TRAIT,
                TILE_SET_INT,
                TILE_DELETE_CHILDREN,
                TILE_REMOVE_FADE_CONTROL,
                LIST_REMOVE_ALL,
                WAYPOINTS_CLEAR,
            ],
        );
        returns(e, INTERFACE_MANAGER, 0x6000);
        returns(e, TILE_GET_MENU_BY_CLASS, 0);
        returns(e, READER_00586150, 0x6100);
        returns(e, TILE_READ_FILE, 0x6200);
        returns(e, TILE_GET_MENU, menu);
        returns(e, WORD_AT_4, 0x6300);
        returns(e, MAP_MENU_ID_00720DF0, 0x3ff);
        returns_float(e, TILE_GET_FLOAT, 102.0);
        returns_float(e, TILE_GET_MAXIMUM_DEPTH, 9.0);
        e.set_global(CREATE_VALUE_A, 6006.0f64);
        e.set_global(CREATE_VALUE_B, 102.0f64);
    }

    #[test]
    fn create_rejects_a_menu_that_is_not_the_map_menu() {
        let mut e = engine();
        let menu = e.mem.alloc(0x230);
        create_doubles(&mut e, menu, 0x3f0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6b90, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(MAP_MENU_SINGLETON), menu);
        // The menu is deleted through vtable slot 0.
        assert_eq!(calls_to(&e, 0x0050_0000), vec![vec![menu, 1]]);
        assert!(!called(&e, MENU_SET_MENU_TILE));
    }

    #[test]
    fn create_warns_when_a_tile_is_missing() {
        let mut e = engine();
        let menu = e.mem.alloc(0x230);
        create_doubles(&mut e, menu, 0x3ff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6b90, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![CREATION_FAILED_TEXT]]
        );
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![menu, 0x6200, 0]]
        );
    }

    #[test]
    fn create_sets_the_menu_up() {
        let mut e = engine();
        e.map(0x7000, 0x1000);
        let menu = menu_with_tiles(&mut e, 0x7000).addr();
        create_doubles(&mut e, menu, 0x3ff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_6b90, &args![]).u32(), 0x6200);
        // Trait 0xfa7 is 102: the depth trait is set.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6200, 0xfad, 9.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, TUTORIAL_SHOW_MESSAGE),
            vec![vec![0x6000 + 0x4d4, 0x1a, 0x3ff, 0x200]]
        );
        let lists = calls_to(&e, LIST_BOX_INITIALIZE);
        assert_eq!(lists[0], vec![menu + 0x130, 0x7007, LIST_MARKER_TEMPLATE]);
        assert_eq!(lists[4], vec![menu + 0x190, 0x700a, LIST_MARKER_TEMPLATE]);
        // Tiles 5 and 3 get bit 0x8000 of their word +0x30.
        assert_ne!(e.mem.u32(0x7005 + 0x30) & 0x8000, 0);
        assert_ne!(e.mem.u32(0x7003 + 0x30) & 0x8000, 0);
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 1]]);
    }

    #[test]
    fn flag_8000_is_set_and_cleared() {
        let mut e = engine();
        let tile = e.mem.alloc(0x40);
        e.mem.set_u32(tile + 0x30, 0x0001_0001);
        e.call(0x0079_6f40, &args![tile, 1u8]);
        assert_eq!(e.mem.u32(tile + 0x30), 0x0001_8001);
        e.call(0x0079_6f40, &args![tile, 0u8]);
        assert_eq!(e.mem.u32(tile + 0x30), 0x0001_0001);
    }

    #[test]
    fn all_tiles_present_check() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        assert!(e.call(0x0079_6f90, &args![menu]).bool());
        e.mem.set_u32(menu.addr() + 0x28 + 4 * 0x15, 0);
        assert!(!e.call(0x0079_6f90, &args![menu]).bool());
    }

    #[test]
    fn do_click_on_the_markers_clicks_the_highlighted_marker() {
        let mut e = engine();
        let vtable = 0x0107_4d00;
        e.put_vtable(vtable, &[0x0050_0000; 0x10]);
        quiet(&mut e, &[0x0050_0000]);
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(menu.addr(), vtable);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 2i32, 0u32]);
        assert!(!called(&e, 0x0050_0000));
        e.set(menu, MapMenu::pHighlightedMapMarker, 0x9999);
        e.call(0x0079_6fd0, &args![menu, 4i32, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0050_0000),
            vec![vec![menu.addr(), 0x1a, 0x9999]]
        );
        // Ids outside the table do nothing.
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 3i32, 0u32]);
        e.call(0x0079_6fd0, &args![menu, 0x25i32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn do_click_activates_the_highlighted_quest() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5150);
        returns(&mut e, QUEST_FLAG_0059E400, 0);
        quiet(
            &mut e,
            &[SET_ACTIVE_QUEST, PLAY_MENU_SOUND, UPDATE_CURRENT_PAGE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x17i32, 0u32]);
        assert_eq!(calls_to(&e, SET_ACTIVE_QUEST), vec![vec![0x4444, 0x5150]]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![3]]);
        // A quest for which `0059e400` is set stays.
        returns(&mut e, QUEST_FLAG_0059E400, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x17i32, 0u32]);
        assert!(!called(&e, SET_ACTIVE_QUEST));
    }

    #[test]
    fn do_click_switches_the_page() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(TRAIT_CURRENT_TAB, 0x55);
        e.set_global(HALF_FLOAT, 0.5f32);
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 3.0);
        returns(&mut e, OBJECT_VALUE_ADDRESS_00403E20, setting);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns(&mut e, WORD_AT_174, 0x6100);
        returns(&mut e, WORD_AT_28, 0x6200);
        returns(&mut e, FLAG_00709D40, 0);
        returns(&mut e, GET_PIPBOY, 0);
        returns(&mut e, PIPBOY_FLAG_00974D90, 0);
        quiet(
            &mut e,
            &[
                TILE_SET_INT,
                TRIGGER_BURST,
                TILE_SET_FLOAT,
                TAB_KNOB_0079C340,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x23i32, 0u32]);
        assert_eq!(e.get(menu, MapMenu::iCurrentPage), 0x23);
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints[0], vec![0x7011, 0x55, 3]);
        assert_eq!(ints[1], vec![0x700d, 0xfa3, 0]);
        assert_eq!(ints[2], vec![0x700c, 0xfaf, 1]);
        assert_eq!(
            calls_to(&e, TRIGGER_BURST),
            vec![vec![
                0x6100,
                3.0f32.to_bits(),
                3.0f32.to_bits(),
                0.5f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6200, 0xfa9, 255.0f32.to_bits(), 1]]
        );
        assert!(!called(&e, TAB_KNOB_0079C340));
        // The world map page leaves the pip-boy alpha alone.
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x20i32, 0u32]);
        assert!(!called(&e, TILE_SET_FLOAT));
        assert_eq!(calls_to(&e, TAB_KNOB_0079C340), vec![vec![1]]);
    }

    #[test]
    fn do_click_toggles_the_notes_filter_or_steps_the_challenge_type() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x23);
        e.set(menu, MapMenu::bShowingAllNotes, 1);
        returns(&mut e, TEXT_TO_TRAIT, 0x66);
        quiet(
            &mut e,
            &[
                NOTE_LIST_FILTER,
                CHALLENGE_LIST_FILTER,
                TILE_SET_INT,
                TILE_SET_STRING,
            ],
        );
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x8888);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0xci32, 0u32]);
        assert_eq!(e.get(menu, MapMenu::bShowingAllNotes), 0);
        assert_eq!(
            calls_to(&e, NOTE_LIST_FILTER),
            vec![vec![menu.addr() + 0x160, NOTE_FILTER_CALLBACK]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT)[1], vec![0x700c, 0x66, 0]);
        // Challenge mode: the type wraps from 2 to 0.
        e.mem.set_u8(CHALLENGE_MODE_011DA36D, 1);
        e.mem.set_u8(CHALLENGE_TYPE_011DA36E, 2);
        e.mem.set_u32(CHALLENGE_TYPE_NAMES_011A03E4, 0x1230);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0xci32, 0u32]);
        assert_eq!(e.mem.u8(CHALLENGE_TYPE_011DA36E), 0);
        assert_eq!(calls_to(&e, OBJECT_FIELD_00403DF0), vec![vec![0x1230]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x7015, 0xfc4, 0x8888, 1]]
        );
    }

    #[test]
    fn do_click_sets_the_player_marker_on_the_local_map() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set(menu, MapMenu::pMapTile, 0x7100);
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns_float(&mut e, CURSOR_Y_007983C0, 20.0);
        returns_float(&mut e, CURSOR_X_007697F0, 10.0);
        returns_float(&mut e, TILE_GETTER_00A013D0, 1.0);
        returns_float(&mut e, TILE_GETTER_00A01440, 2.0);
        returns_float(&mut e, TILE_GET_FLOAT, 0.5);
        e.register(POINT_SET_00416870, |e, a| {
            for (i, word) in a[1..4].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, *word);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(POINT_SCALE_0053D280, |e, a| {
            let scale = f32::from_bits(a[2]);
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * scale;
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(MAP_TO_WORLD_COORD, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[1] + 4 * i, a[2 + i as usize]);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        returns(&mut e, CALCULATE_PLAYER_MARKER_SPACE, 0x3300);
        returns(&mut e, AUDIO_INSTANCE, 0x5000);
        e.register(AUDIO_SOUND_BY_NAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        returns(&mut e, PLAYER_FLAG_00798400, 0);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x8800);
        quiet(
            &mut e,
            &[
                ADJUST_MAP_MARKER_COORD,
                TILE_SET_INT,
                SOUND_PLAY,
                SOUND_HANDLE_DESTRUCT,
                SHOW_MESSAGE_BOX,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0xci32, 0u32]);
        let base = menu.addr();
        // Cursor (10, 20) minus the tile's (1, 2), scaled by 0.5.
        assert_eq!(e.mem.f32(base + 0xfc), 4.5);
        assert_eq!(e.mem.f32(base + 0x100), 9.0);
        assert_eq!(e.get(menu, MapMenu::PlayerMarkerSpace), 0x3300);
        assert_eq!(
            calls_to(&e, ADJUST_MAP_MARKER_COORD),
            vec![vec![0x3300, base + 0xfc, 1]]
        );
        assert_eq!(
            calls_to(&e, AUDIO_SOUND_BY_NAME)[0][2..],
            [MARKER_ADDED_SOUND, 0x121]
        );
        let boxes = calls_to(&e, SHOW_MESSAGE_BOX);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].len(), 11);
        assert_eq!(boxes[0][3..6], [SET_PLAYER_MARKER_CALLBACK, 3, 0x17]);
    }

    #[test]
    fn do_click_opens_an_audio_note() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.mem.set_u32(TRAIT_SELECTED, 0x41);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x42);
        e.set_global(HALF_ALPHA_FLOAT, 128.0f32);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5100);
        returns(&mut e, NOTE_TYPE, 0);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, LIST_BOX_GET_TILE_FOR, 0x7300);
        returns(&mut e, NOTE_FLAG_00798420, 0);
        returns(&mut e, NOTE_SOUND_005E9100, 0x5200);
        returns(&mut e, WORD_AT_C, 0x99);
        returns(&mut e, AUDIO_INSTANCE, 0x5000);
        e.register(AUDIO_SOUND_BY_ID, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(AUDIO_SOUND_BY_NAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCT,
                TILE_SET_INT,
                LOAD_NOTE,
                TILE_SET_FLOAT,
                NOTE_SET_READ_005E9300,
                SOUND_LIST_ADD_TAIL,
                SOUND_PLAY,
                DIALOG_FLAG_00762150,
                AUDIO_ENTER_DIALOGUE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x18i32, 0x7400u32]);
        assert_eq!(calls_to(&e, LOAD_NOTE), vec![vec![base, 0x5100]]);
        assert_eq!(e.get(menu, MapMenu::pSelectedNote), 0x5100);
        assert_eq!(
            calls_to(&e, LIST_BOX_GET_TILE_FOR),
            vec![vec![base + 0x160, base + 0x90]]
        );
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints[0], vec![0x7300, 0x41, 0]);
        assert_eq!(ints[1], vec![0x7400, 0x41, 1]);
        assert_eq!(ints[2], vec![0x700d, 0x1004, 1]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7400, 0x42, 128.0f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, AUDIO_SOUND_BY_ID)[0][2..], [0x99, 0x101]);
        assert_eq!(calls_to(&e, SOUND_LIST_ADD_TAIL)[0][0], base + 0x98);
        assert_eq!(e.get(menu, MapMenu::bSoundsLoading), 1);
        assert_eq!(
            calls_to(&e, AUDIO_SOUND_BY_NAME)[0][2..],
            [HOLOTAPE_START_SOUND, 0x121]
        );
        assert_eq!(calls_to(&e, AUDIO_ENTER_DIALOGUE), vec![vec![0x5000]]);
        // In challenge mode the click does nothing.
        e.mem.set_u8(CHALLENGE_MODE_011DA36D, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x18i32, 0x7400u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_click_runs_a_conversation_note() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5100);
        returns(&mut e, NOTE_TYPE, 3);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, LIST_BOX_GET_TILE_FOR, 0);
        returns(&mut e, NOTE_FLAG_00798420, 1);
        returns(&mut e, NOTE_TOPIC_005E9240, 0x5300);
        returns(&mut e, NOTE_GET_SPEAKER, 0x5400);
        let speaker = object_with_vtable(&mut e, 0x0107_9000, &[0x0050_0000; 8]);
        e.mem.set_u32(0x0107_9010, 0x0050_0010);
        let blocks = [speaker, 0x6600];
        let mut next = 0;
        e.register_double(ALLOCATE, move |_, _| {
            let block = blocks[next];
            next += 1;
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(CHARACTER_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CONVERSATION_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, GET_CURRENT_RESPONSE, 0x6700);
        returns(&mut e, FIRST_RESPONSE, 1);
        let mut more = 1;
        e.register_double(NEXT_RESPONSE, move |_, _| {
            let value = more;
            more = 0;
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, WORD_AT_C, 0x6800);
        e.register(PLUS_10, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        returns(&mut e, STRING_TEXT, 0x6900);
        returns(&mut e, AUDIO_INSTANCE, 0x5000);
        e.register(AUDIO_SOUND_BY_FILENAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(AUDIO_SOUND_BY_NAME, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.set_global(NOTE_VOLUME_FLOAT, 0.9f32);
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCT,
                TILE_SET_INT,
                LOAD_NOTE,
                FORM_SET_TEMPORARY,
                REFR_SET_OBJECT_REFERENCE,
                CONVERSATION_FIRST_ITEM,
                SUBTITLES_ADD_007A1AC0,
                TOPIC_INFO_RUN_RESULT,
                SOUND_HANDLE_ASSIGN,
                SOUND_SET_VOLUME,
                SOUND_LIST_ADD_TAIL,
                CONVERSATION_DESTRUCT,
                FREE,
                0x0050_0010,
                SOUND_PLAY,
                DIALOG_FLAG_00762150,
                AUDIO_ENTER_DIALOGUE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x18i32, 0x7400u32]);
        assert_eq!(calls_to(&e, CHARACTER_CONSTRUCT), vec![vec![speaker, 0]]);
        assert_eq!(
            calls_to(&e, REFR_SET_OBJECT_REFERENCE),
            vec![vec![speaker, 0x5400]]
        );
        assert_eq!(
            calls_to(&e, CONVERSATION_CONSTRUCT),
            vec![vec![0x6600, speaker, 0x4444, 0x5300]]
        );
        // Two responses, each with its result script before and after.
        assert_eq!(
            calls_to(&e, TOPIC_INFO_RUN_RESULT),
            vec![
                vec![0x6800, 0, speaker],
                vec![0x6800, 1, speaker],
                vec![0x6800, 0, speaker],
                vec![0x6800, 1, speaker]
            ]
        );
        assert_eq!(
            calls_to(&e, AUDIO_SOUND_BY_FILENAME)[0][2..],
            [0x6900, 0x105, 0]
        );
        assert_eq!(calls_to(&e, SOUND_SET_VOLUME)[0][1], 0.9f32.to_bits());
        assert_eq!(
            calls_to(&e, SUBTITLES_ADD_007A1AC0)[0],
            vec![base + 0xa8, 0x6700]
        );
        assert_eq!(calls_to(&e, CONVERSATION_DESTRUCT), vec![vec![0x6600]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x6600]]);
        assert_eq!(calls_to(&e, 0x0050_0010), vec![vec![speaker, 1]]);
        assert!(called(&e, AUDIO_ENTER_DIALOGUE));
    }

    #[test]
    fn do_click_switches_the_radio_station() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.put_vtable(0x0107_9000, &[0x0050_0000, 0x0050_0004]);
        e.mem.set_u32(base + 0x190, 0x0107_9000);
        returns(&mut e, 0x0050_0004, 0x7700);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x42);
        e.mem.set_u32(TRAIT_SELECTED, 0x41);
        e.set_global(FULL_ALPHA_DOUBLE, 255.0f64);
        returns_float(&mut e, TILE_GET_FLOAT, 255.0);
        returns(&mut e, TILE_IS_TRUE, 0);
        returns(&mut e, FLAG_00798440, 1);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x5500);
        quiet(
            &mut e,
            &[
                TILE_SET_INT,
                LIST_BOX_FILTER_007314C0,
                RADIO_ENABLE,
                RADIO_SET_STATION_00832240,
                UPDATE_CURRENT_PAGE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x19i32, 0u32]);
        assert_eq!(calls_to(&e, RADIO_ENABLE), vec![vec![0], vec![1]]);
        assert_eq!(
            calls_to(&e, RADIO_SET_STATION_00832240),
            vec![vec![0x5500, 0]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FILTER_007314C0),
            vec![vec![base + 0x190, RADIO_FILTER_CALLBACK, 0, 0x7fff_ffff]]
        );
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints.last().unwrap(), &vec![0x7700, 0x41, 1]);
        // A station that is not fully visible is not switched.
        returns_float(&mut e, TILE_GET_FLOAT, 128.0);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x19i32, 0u32]);
        assert!(!called(&e, RADIO_ENABLE));
    }

    #[test]
    fn do_click_offers_fast_travel_to_the_clicked_marker() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        let base = menu.addr();
        e.put_vtable(0x0107_9000, &[0x0050_0000; 8]);
        e.mem.set_u32(base, 0x0107_9000);
        e.mem.set_u32(0x0107_9014, 0x0050_0014);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        // Two markers in the list at +0xd8.
        let second = e.mem.alloc(8);
        e.mem.set_u32(base + 0xd8, 0x3100);
        e.mem.set_u32(base + 0xdc, second);
        e.mem.set_u32(second, 0x3200);
        returns(&mut e, FLAG_004B71D0, 1);
        returns(&mut e, TILE_IS_TRUE, 1);
        returns(&mut e, PLAYER_FLAG_0093D660, 1);
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        returns(&mut e, ADD_USER_TRAIT, 0x61);
        returns_float(&mut e, TILE_GET_FLOAT, 1.0);
        returns(&mut e, REFR_GET_MAP_MARKER_DATA, 0x3400);
        returns(&mut e, MARKER_GET_TRAVEL_LOC, 1);
        returns(&mut e, MARKER_NAME_00438EB0, 0x3500);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3600);
        quiet(
            &mut e,
            &[0x0050_0014, BUFFER_FORMAT, TILE_SET_INT, SHOW_MESSAGE_BOX],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x1ai32, 0x7400u32]);
        // `"_MarkerIndex"` 1.0 picks the second marker.
        assert_eq!(e.get(menu, MapMenu::pFastTravelMarker), 0x3200);
        assert_eq!(calls_to(&e, 0x0050_0014), vec![vec![base, 0x1a, 0x7400]]);
        let format = calls_to(&e, BUFFER_FORMAT);
        assert_eq!(format[0][1..], [0x100, FAST_TRAVEL_FORMAT, 0x3600, 0x3500]);
        let boxes = calls_to(&e, SHOW_MESSAGE_BOX);
        assert_eq!(boxes[0][0], format[0][0]);
        assert_eq!(boxes[0][3], FAST_TRAVEL_CALLBACK);
        // No travel location: the marker is dropped.
        returns(&mut e, MARKER_GET_TRAVEL_LOC, 0);
        e.call(0x0079_6fd0, &args![menu, 0x1ai32, 0x7400u32]);
        assert_eq!(e.get(menu, MapMenu::pFastTravelMarker), 0);
    }

    #[test]
    fn do_click_shows_the_companion_message() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set_global(TWO_FLOAT, 2.0f32);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, TILE_IS_TRUE, 0);
        returns(&mut e, ADD_USER_TRAIT, 0x61);
        returns(&mut e, TILE_GET_STRING, 0x3700);
        returns(&mut e, STRICMP, 1);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3600);
        quiet(&mut e, &[SHOW_MESSAGE_007052F0]);
        e.call_log = Some(vec![]);
        e.call(0x0079_6fd0, &args![menu, 0x1ai32, 0x7400u32]);
        assert_eq!(calls_to(&e, STRICMP), vec![vec![0x3700, COMPANION_TEXT]]);
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE_007052F0),
            vec![vec![0x3600, 2, 0, 0, 2.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn conversation_deleting_destructor_frees_on_flag_one() {
        let mut e = engine();
        quiet(&mut e, &[CONVERSATION_DESTRUCT, FREE]);
        for (flags, frees) in [(0u32, false), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0079_8450, &args![0x4000u32, flags]).u32(), 0x4000);
            assert_eq!(calls_to(&e, CONVERSATION_DESTRUCT), vec![vec![0x4000]]);
            assert_eq!(called(&e, FREE), frees);
        }
    }

    #[test]
    fn do_idle_pauses_and_resumes_the_note_sound() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::pCurrentSound, 0x5600);
        e.set(menu, MapMenu::iSoundPlayStartTime, 100);
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, SOUND_IS_PLAYING, 1);
        returns(&mut e, IS_TOP_MENU_ID, 1);
        returns(&mut e, TIMER_NOW, 1000);
        returns(&mut e, GET_PIPBOY, 0);
        returns(&mut e, PIPBOY_FLAG_00974D90, 0);
        quiet(&mut e, &[SOUND_PAUSE, SOUND_PLAY]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, SOUND_PAUSE), vec![vec![0x5600]]);
        assert_eq!(e.mem.u8(SOUND_PAUSED_011DA36F), 1);
        assert_eq!(e.mem.u32(PAUSE_TIME_011DA40C), 1000);
        // Menu 0x3f5 gone: the sound plays on, the pause added to the start.
        returns(&mut e, IS_TOP_MENU_ID, 0);
        returns(&mut e, TIMER_NOW, 1500);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, SOUND_PLAY), vec![vec![0x5600, 0]]);
        assert_eq!(e.mem.u8(SOUND_PAUSED_011DA36F), 0);
        assert_eq!(e.get(menu, MapMenu::iSoundPlayStartTime), 600);
    }

    #[test]
    fn do_idle_fades_the_pipboy_in_under_another_menu() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set(menu, MapMenu::bSoundsLoading, 1);
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        returns(&mut e, IS_TOP_MENU_ID, 0);
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns(&mut e, WORD_AT_28, 0x6200);
        quiet(&mut e, &[TILE_SET_FLOAT, UPDATE_PLAYING_SOUNDS]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6200, 0xfa9, 255.0f32.to_bits(), 1]]
        );
        assert_eq!(e.mem.u8(FADED_011DA408), 1);
        assert_eq!(calls_to(&e, UPDATE_PLAYING_SOUNDS), vec![vec![menu.addr()]]);
        // Once faded it is not done again.
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert!(!called(&e, TILE_SET_FLOAT));
    }

    #[test]
    fn do_idle_shows_the_cursor_position_on_the_local_map() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(MAP_WIDTH_DOUBLE, 850.0f64);
        e.set_global(MAP_HEIGHT_DOUBLE, 500.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(FADED_011DA408, 1u8);
        e.register(IS_TOP_MENU_ID, |_, a| Ret {
            eax: u32::from(a[0] == 0x3ff),
            ..Ret::default()
        });
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, INTERFACE_MANAGER, 0x6000);
        returns(&mut e, WORD_AT_28, 0x6200);
        returns_float(&mut e, CURSOR_X_007697F0, 110.0);
        returns_float(&mut e, CURSOR_Y_007983C0, 70.0);
        returns_float(&mut e, TILE_GETTER_00A013D0, 10.0);
        returns_float(&mut e, TILE_GETTER_00A01440, 20.0);
        returns_float(&mut e, TILE_GET_FLOAT, 40.0);
        returns(&mut e, OBJECT_00877720, 0x6400);
        returns(&mut e, OBJECT_CHECK_00A23A50, 0);
        quiet(&mut e, &[TILE_SET_FLOAT, SHOW_POSITION_00799DC0]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        let floats = calls_to(&e, TILE_SET_FLOAT);
        // Inside the map: the pip-boy alpha 0, then x 100 - 20 and y 50 - 20.
        assert_eq!(floats[0], vec![0x6200, 0xfa9, 0, 1]);
        assert_eq!(floats[1], vec![0x7006, 0xfa1, 80.0f32.to_bits(), 1]);
        assert_eq!(floats[2], vec![0x7006, 0xfa2, 30.0f32.to_bits(), 1]);
        assert_eq!(
            calls_to(&e, SHOW_POSITION_00799DC0),
            vec![vec![menu.addr()]]
        );
        assert_eq!(e.mem.u8(FADED_011DA408), 0);
        // Off the map (x above 850): alpha 255.
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        returns_float(&mut e, CURSOR_X_007697F0, 900.0);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[0],
            vec![0x6200, 0xfa9, 255.0f32.to_bits(), 1]
        );
    }

    #[test]
    fn do_idle_marks_the_selected_note_read_after_its_delay() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x23);
        e.set(menu, MapMenu::uiNoteEnterStartTime, 1000);
        e.set(menu, MapMenu::pSelectedNote, 0x5100);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x42);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(HALF_ALPHA_FLOAT, 128.0f32);
        let delay = e.mem.alloc(4);
        e.mem.set_f32(delay, 2.0);
        returns(&mut e, OBJECT_VALUE_ADDRESS_00403E20, delay);
        e.register(IS_TOP_MENU_ID, |_, a| Ret {
            eax: u32::from(a[0] == 0x3ff),
            ..Ret::default()
        });
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_004B71D0, 0);
        returns(&mut e, TIMER_NOW, 2500);
        returns(&mut e, LIST_BOX_GET_TILE_FOR, 0x7300);
        quiet(&mut e, &[TILE_SET_FLOAT, NOTE_SET_READ_005E9300]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        // 1500 ms of 2000: nothing yet.
        assert!(!called(&e, NOTE_SET_READ_005E9300));
        returns(&mut e, TIMER_NOW, 3000);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, NOTE_SET_READ_005E9300), vec![vec![0x5100, 1]]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7300, 0x42, 128.0f32.to_bits(), 1]]
        );
        assert_eq!(e.get(menu, MapMenu::uiNoteEnterStartTime), 0);
    }

    #[test]
    fn do_idle_updates_the_radio_page() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x24);
        e.register(IS_TOP_MENU_ID, |_, a| Ret {
            eax: u32::from(a[0] == 0x3ff),
            ..Ret::default()
        });
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns(&mut e, FLAG_00798440, 0);
        quiet(&mut e, &[TILE_SET_INT, PAGE_0079C0C0]);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x7010, 0xfa3, 0]]);
        returns(&mut e, FLAG_00798440, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_a130, &args![menu]);
        assert_eq!(calls_to(&e, PAGE_0079C0C0), vec![vec![menu.addr()]]);
    }

    #[test]
    fn update_menu_formats_the_date_and_time() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.mem.set_u32(PLAYER_011DEA3C, 0x4444);
        e.set_global(SIXTY_DOUBLE, 60.0f64);
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns_float(&mut e, CALENDAR_GET_HOUR, 13.5);
        e.register(FTOL2, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        e.register(CALENDAR_GET_DATE_STRING, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(STRING_TEXT, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3800);
        returns(&mut e, COMPARE_00408B20, 0);
        quiet(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                STRING_FORMAT,
                TILE_SET_STRING,
                PLAYER_MESSAGE_00578870,
                UPDATE_CURRENT_PAGE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_aba0, &args![]);
        let format = calls_to(&e, STRING_FORMAT);
        assert_eq!(format.len(), 1);
        let text = format[0][0];
        assert_eq!(format[0][1..], [DATE_TIME_FORMAT, text + 8 + 0x100, 13, 30]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![
                vec![0x7001, 0xfc4, text + 0x100, 1],
                vec![0x7000, 0xfc4, text + 0x100, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, PLAYER_MESSAGE_00578870),
            vec![vec![0x4444, text]]
        );
        assert_eq!(
            calls_to(&e, STRING_DESTRUCT),
            vec![vec![text + 8], vec![text]]
        );
        assert_eq!(calls_to(&e, UPDATE_CURRENT_PAGE), vec![vec![menu.addr()]]);
    }

    #[test]
    fn update_menu_sets_the_titles_for_the_language() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.mem.set_u32(TRAIT_TITLE, 0x51);
        returns(&mut e, GET_PIPBOY, 0x6000);
        returns(&mut e, PIPBOY_FLAG_00974D90, 1);
        returns_float(&mut e, CALENDAR_GET_HOUR, 1.0);
        returns(&mut e, FTOL2, 1);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x3800);
        returns(&mut e, COMPARE_00408B20, 1);
        e.register(TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xff,
            ..Ret::default()
        });
        returns(&mut e, TILE_GET_VALUE, 0x3900);
        returns(&mut e, TILE_GET_CHILD_BY_ID, 0x3a00);
        returns(&mut e, WORD_AT_4, 0x3b00);
        returns(&mut e, TILE_GET_CHILD_BY_NAME, 0x3c00);
        quiet(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                STRING_FORMAT,
                STRING_TEXT,
                CALENDAR_GET_DATE_STRING,
                TILE_SET_STRING,
                PLAYER_MESSAGE_00578870,
                UPDATE_CURRENT_PAGE,
                VALUE_CALCULATE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_aba0, &args![]);
        let strings = calls_to(&e, TILE_SET_STRING);
        assert_eq!(strings.len(), 2 + 10 + 2);
        assert_eq!(strings[2], vec![0x700b, TITLE_N_TEXT[0] & 0xff, 0x3800, 1]);
        assert_eq!(strings[11], vec![0x700c, TITLE_N_TEXT[4] & 0xff, 0x3800, 1]);
        assert_eq!(strings[12], vec![0x3a00, 0xfc4, 0x3800, 1]);
        assert_eq!(strings[13], vec![0x3c00, 0xfc4, 0x3800, 1]);
        assert_eq!(
            calls_to(&e, TILE_GET_VALUE),
            vec![vec![0x700b, 0x51], vec![0x700c, 0x51]]
        );
        assert_eq!(calls_to(&e, TILE_GET_CHILD_BY_ID), vec![vec![0x7011, 0x24]]);
        assert_eq!(
            calls_to(&e, TILE_GET_CHILD_BY_NAME),
            vec![vec![0x3b00, HEADLINE_TITLE_TEXT]]
        );
        // Without a map menu nothing happens.
        e.mem.set_u32(MAP_MENU_SINGLETON, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_aba0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn offset_6bc() {
        let mut e = engine();
        assert_eq!(e.call(0x0079_bc30, &args![0x1000u32]).u32(), 0x16bc);
    }

    #[test]
    fn waveform_reset_clears_the_time_and_tile_16() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.set(menu, MapMenu::fWaveformAnimationStartTime, 5.0);
        quiet(&mut e, &[TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0079_c090, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::fWaveformAnimationStartTime), 0.0);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x7010, 0xfa3, 0]]);
    }

    #[test]
    fn word_011dea14() {
        let mut e = engine();
        e.mem.set_u32(VALUE_011DEA14, 0x1357);
        assert_eq!(e.call(0x007a_1480, &args![]).u32(), 0x1357);
    }

    /// Doubles for the shader-property chain of `ClearMapMemory` and
    /// `ClearWorldMapTexture`.
    fn texture_doubles(e: &mut Engine, object: u32) {
        returns(e, TILE_OBJECT_0056C7F0, object);
        returns(e, OBJECT_NODE_0045BC00, 0x3d00);
        returns(e, PROPERTY_TYPE_00438220, 3);
        returns(e, AV_OBJECT_GET_PROPERTY, 0x3e00);
        returns(e, PROPERTY_CHECK_009C8640, 1);
        quiet(e, &[SET_TILE_TEXTURE, TILE_SET_STRING]);
    }

    #[test]
    fn clear_map_memory_releases_the_map_tiles() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::pMapTile, 0x7100);
        e.set(menu, MapMenu::uiNextLocalMapGrid, 7);
        let object = object_with_vtable(&mut e, 0x0107_9000, &[0x0050_0000; 0x40]);
        e.mem.set_u32(0x0107_90e8, 0x0050_00e8);
        texture_doubles(&mut e, object);
        returns(&mut e, OBJECT_COUNT_0043B480, 3);
        e.register(OBJECT_ITEM_0043B4A0, |_, a| Ret {
            eax: if a[1] == 1 { 0 } else { 0x100 + a[1] },
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x0050_00e8,
                TILE_DELETE_CHILDREN,
                TILE_SET_INT,
                TILE_REMOVE_FADE_CONTROL,
                LIST_REMOVE_ALL,
                WAYPOINTS_CLEAR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x007a_1490, &args![]);
        let base = menu.addr();
        assert_eq!(
            calls_to(&e, 0x0050_00e8),
            vec![vec![object, 0x100], vec![object, 0x102]]
        );
        assert_eq!(
            calls_to(&e, TILE_DELETE_CHILDREN),
            vec![vec![0x7002], vec![0x7004]]
        );
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x7003, 0xff5], vec![0x7005, 0xff5]]
        );
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ALL),
            vec![vec![base + 0xe0], vec![base + 0xd8]]
        );
        assert_eq!(calls_to(&e, WAYPOINTS_CLEAR), vec![vec![base + 0x220, 1]]);
        assert_eq!(e.get(menu, MapMenu::pMapTile), 0);
        assert_eq!(e.get(menu, MapMenu::uiNextLocalMapGrid), 0);
        assert_eq!(calls_to(&e, SET_TILE_TEXTURE), vec![vec![0x3e00, 0]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x7010, 0xfcc, EMPTY_TEXT, 1]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT).last().unwrap(),
            &vec![0x7010, 0xfa3, 0]
        );
        // Without the tile object nothing else runs.
        returns(&mut e, TILE_OBJECT_0056C7F0, 0);
        e.call_log = Some(vec![]);
        e.call(0x007a_1490, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn clear_world_map_texture_skips_the_open_local_map() {
        let mut e = engine();
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        texture_doubles(&mut e, 0x3f00);
        returns(&mut e, IS_TOP_MENU_ID, 1);
        e.call_log = Some(vec![]);
        e.call(0x007a_1670, &args![]);
        assert!(!called(&e, SET_TILE_TEXTURE));
        returns(&mut e, IS_TOP_MENU_ID, 0);
        e.call_log = Some(vec![]);
        e.call(0x007a_1670, &args![]);
        assert_eq!(calls_to(&e, TILE_OBJECT_0056C7F0)[0], vec![0x7004]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x7004, 0xfcc, EMPTY_TEXT, 1]]
        );
        assert_eq!(e.mem.u8(TEXTURE_CLEARED_011DA36C), 1);
    }

    #[test]
    fn magnification_of_tiles_2_and_4() {
        let mut e = engine();
        assert_eq!(e.call(0x007a_17a0, &args![]).f32(), 1.0);
        assert_eq!(e.call(0x007a_17e0, &args![]).f32(), 1.0);
        let menu = menu_with_tiles(&mut e, 0x7000);
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.mem.set_u32(TRAIT_MAGNIFICATION, 0x45);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: f64::from(a[0] - 0x7000),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_17a0, &args![]).f32(), 2.0);
        assert_eq!(e.call(0x007a_17e0, &args![]).f32(), 4.0);
        assert_eq!(
            calls_to(&e, TILE_GET_FLOAT),
            vec![vec![0x7002, 0x45], vec![0x7004, 0x45]]
        );
    }

    #[test]
    fn set_root_map_space_walks_to_the_top_world() {
        let mut e = engine();
        let menu: Ptr<MapMenu> = e.new_object();
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        e.register(WORLD_GET_PARENT, |_, a| Ret {
            eax: match a[0] {
                0x100 => 0x200,
                0x200 => 0x300,
                _ => 0,
            },
            ..Ret::default()
        });
        e.call(0x007a_1880, &args![0x100u32]);
        assert_eq!(e.get(menu, MapMenu::pRootMapSpace), 0x300);
        assert_eq!(e.get(menu, MapMenu::bRootSpaceChanged), 1);
        e.call(0x007a_1880, &args![0x200u32]);
        assert_eq!(e.get(menu, MapMenu::bRootSpaceChanged), 0);
        // Without a map menu it does nothing.
        e.mem.set_u32(MAP_MENU_SINGLETON, 0);
        e.call(0x007a_1880, &args![0x100u32]);
    }

    #[test]
    fn highlighted_item_of_a_list_box() {
        let mut e = engine();
        let list: Ptr<ListBox> = e.new_object();
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x9000));
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_1910, &args![list]).u32(), 0x77);
        assert_eq!(
            calls_to(&e, LIST_BOX_ITEM_OF_007E1110),
            vec![vec![list.addr(), 0x9000]]
        );
    }

    #[test]
    fn list_box_flags_bits() {
        let mut e = engine();
        let list: Ptr<ListBox> = e.new_object();
        e.call(0x007a_1950, &args![list, 3u8, 1u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 8);
        e.call(0x007a_1930, &args![list, 1u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 9);
        e.call(0x007a_1930, &args![list, 0u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 8);
        // The shift count is masked to 5 bits: 35 is bit 3.
        e.call(0x007a_1950, &args![list, 35u8, 0u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 0);
        // Bits above 15 fall off the 16-bit flags.
        e.call(0x007a_1950, &args![list, 20u8, 1u8]);
        assert_eq!(e.get(list, ListBox::usFlags), 0);
    }

    #[test]
    fn string_array_constructor_and_release() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr() + 4, 0x1111);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_1a70, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr()), ARRAY_VTABLE_01075530);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        quiet(
            &mut e,
            &[ARRAY_DESTROY_ELEMENTS_007A63B0, ARRAY_FREE_006A8500],
        );
        // Without a buffer the release does nothing.
        e.set(array, BSSimpleArray::iSize, 5);
        e.call(0x007a_1c30, &args![array, 1u8]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 5);
        e.set(array, BSSimpleArray::pBuffer, 0x2000);
        e.set(array, BSSimpleArray::iReservedSize, 8);
        e.call_log = Some(vec![]);
        e.call(0x007a_1c30, &args![array, 0u8]);
        assert_eq!(
            calls_to(&e, ARRAY_DESTROY_ELEMENTS_007A63B0),
            vec![vec![array.addr(), 0x2000, 5]]
        );
        assert!(!called(&e, ARRAY_FREE_006A8500));
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x2000);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        e.call(0x007a_1c30, &args![array, 1u8]);
        assert_eq!(calls_to(&e, ARRAY_FREE_006A8500), vec![vec![array.addr()]]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn list_box_item_construction() {
        let mut e = engine();
        let item = e.mem.alloc(0xc);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0x4321);
        e.mem.set_u8(item + 8, 1);
        assert_eq!(
            e.call(0x007a_62e0, &args![item, 0x7000u32, data]).u32(),
            item
        );
        assert_eq!(e.mem.u32(item), 0x7000);
        assert_eq!(e.mem.u32(item + 4), 0x4321);
        assert_eq!(e.mem.u8(item + 8), 0);
    }

    #[test]
    fn string_array_constructor_body_reserves_and_constructs() {
        let mut e = engine();
        let array = object_with_vtable(&mut e, 0x0107_9000, &[0x0050_0000, 0x0050_0004]);
        e.register(0x0050_0004, |_, a| Ret {
            eax: 0x8000 + a[1],
            ..Ret::default()
        });
        quiet(&mut e, &[ARRAY_CONSTRUCT_ELEMENTS_007A6310]);
        e.call_log = Some(vec![]);
        e.call(0x007a_63f0, &args![array, 2u32, 5u32]);
        assert_eq!(e.mem.u32(array + 4), 0x8005);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_eq!(e.mem.u32(array + 0xc), 5);
        assert_eq!(
            calls_to(&e, ARRAY_CONSTRUCT_ELEMENTS_007A6310),
            vec![vec![array, 0x8005, 5]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007a_63f0, &args![array, 6u32, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0x8006);
        assert_eq!(e.mem.u32(array + 8), 0);
        assert!(!called(&e, ARRAY_CONSTRUCT_ELEMENTS_007A6310));
        e.call(0x007a_63f0, &args![array, 0u32, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
    }

    /// The five instances' functions: constructor, destructor, `RemoveAll`,
    /// `Highlight`, `IsEnabled`, `UpdateHighlightBoxPos`,
    /// `SetTotalListHeight`.
    const INSTANCES: [(ListBoxKind, [u32; 7]); 5] = [
        (
            QUEST_LIST,
            [
                0x007a_1c90,
                0x007a_1d60,
                0x007a_2070,
                0x007a_23e0,
                0x007a_6550,
                0x007a_6600,
                0x007a_6920,
            ],
        ),
        (
            NOTE_LIST,
            [
                0x007a_27f0,
                0x007a_28c0,
                0x007a_2bd0,
                0x007a_3170,
                0x007a_6bd0,
                0x007a_6c80,
                0x007a_7080,
            ],
        ),
        (
            REFERENCE_LIST,
            [
                0x007a_36e0,
                0x007a_37b0,
                0x007a_3ac0,
                0x007a_3f90,
                0x007a_7330,
                0x007a_73e0,
                0x007a_7700,
            ],
        ),
        (
            OBJECTIVE_LIST,
            [
                0x007a_43a0,
                0x007a_4470,
                0x007a_4780,
                0x007a_4ce0,
                0x007a_79b0,
                0x007a_7a60,
                0x007a_7d80,
            ],
        ),
        (
            CHALLENGE_LIST,
            [
                0x007a_50f0,
                0x007a_51c0,
                0x007a_54d0,
                0x007a_5a60,
                0x007a_81b0,
                0x007a_8260,
                0x007a_8580,
            ],
        ),
    ];

    #[test]
    fn list_box_constructors_set_the_vtable_and_register() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            quiet(
                &mut e,
                &[LIST_BOX_BASE_CONSTRUCT, LIST_CONSTRUCT, LIST_BOX_REGISTER],
            );
            let list: Ptr<ListBox> = e.new_object();
            e.mem.set_u32(list.addr() + 0x14, 0x55);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(functions[0], &args![list]).u32(), list.addr());
            assert_eq!(e.mem.u32(list.addr()), kind.vtable);
            assert_eq!(e.mem.u32(list.addr() + 0x14), 0);
            assert_eq!(e.get(list, ListBox::usFlags), 1);
            assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![list.addr() + 4]]);
            assert_eq!(calls_to(&e, LIST_BOX_REGISTER), vec![vec![list.addr()]]);
        }
    }

    /// Doubles for `RemoveAll` and what it runs.
    fn remove_all_doubles(e: &mut Engine) {
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        returns(e, LIST_BOX_HAS_FLAG, 1);
        returns(e, TEXT_TO_TRAIT, 0x66);
        returns_float(e, TILE_GET_FLOAT, 0.0);
        quiet(
            e,
            &[
                NODE_SET_NEXT,
                NODE_DELETE,
                ITEM_DATA_DELETE,
                FREE,
                TILE_SET_FLOAT,
                TILE_SET_INT,
                ADD_USER_TRAIT,
                0x0050_0000,
                LIST_BOX_UNREGISTER,
                LIST_DESTRUCT,
            ],
        );
    }

    /// A list box with a container and two items (tiles with the vtable at
    /// `0107_9000`, data words `0xd1` and `0xd2`): the first in the
    /// embedded node, the second in a node after it.
    fn list_with_two_items(e: &mut Engine) -> (Ptr<ListBox>, [u32; 2], u32) {
        e.put_vtable(0x0107_9000, &[0x0050_0000]);
        let list: Ptr<ListBox> = e.new_object();
        e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
        e.set(list, ListBox::usNextIndex, 2);
        e.set(list, ListBox::fTotalListHeight, 40.0);
        let mut items = [0u32; 2];
        for (i, item) in items.iter_mut().enumerate() {
            let tile = e.mem.alloc(8);
            e.mem.set_u32(tile, 0x0107_9000);
            *item = e.mem.alloc(0xc);
            e.mem.set_u32(*item, tile);
            e.mem.set_u32(*item + 4, 0xd1 + i as u32);
        }
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, items[1]);
        e.mem.set_u32(list.addr() + 4, items[0]);
        e.mem.set_u32(list.addr() + 8, node);
        (list, items, node)
    }

    #[test]
    fn list_box_remove_all_deletes_every_item() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            remove_all_doubles(&mut e);
            let (list, items, node) = list_with_two_items(&mut e);
            e.call_log = Some(vec![]);
            e.call(functions[2], &args![list]);
            assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
            assert_eq!(
                calls_to(&e, ITEM_DATA_DELETE),
                vec![vec![0xd2, 1], vec![0xd1, 1]]
            );
            assert_eq!(calls_to(&e, FREE), vec![vec![items[1]], vec![items[0]]]);
            assert_eq!(calls_to(&e, 0x0050_0000).len(), 2);
            assert_eq!(e.mem.u32(list.addr() + 4), 0);
            assert_eq!(e.get(list, ListBox::usNextIndex), 0);
            assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x9100, 0x66, 0, 1]]);
            // `UpdateHighlightBoxPos` without a highlighted item.
            let y = e.mem.u32(kind.highlight_y.cache);
            assert_eq!(calls_to(&e, TILE_SET_INT)[0], vec![0x9100, y, 0xffff_ffff]);
            // With the keep-tiles byte set the tiles stay.
            let (list, _, _) = list_with_two_items(&mut e);
            e.mem.set_u8(KEEP_ITEM_TILES, 1);
            e.call_log = Some(vec![]);
            e.call(functions[2], &args![list]);
            assert!(!called(&e, 0x0050_0000));
            e.mem.set_u8(KEEP_ITEM_TILES, 0);
        }
    }

    #[test]
    fn list_box_destructors_empty_the_list() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            remove_all_doubles(&mut e);
            let (list, _, _) = list_with_two_items(&mut e);
            e.call_log = Some(vec![]);
            e.call(functions[1], &args![list]);
            assert_eq!(e.mem.u32(list.addr()), kind.vtable);
            assert_eq!(calls_to(&e, FREE).len(), 2);
            assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![list.addr()]]);
            assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![list.addr() + 4]]);
        }
    }

    #[test]
    fn list_box_is_enabled_looks_the_trait_up_once() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            returns(&mut e, ADD_USER_TRAIT, 0x71);
            returns(&mut e, TILE_IS_TRUE, 1);
            let list: Ptr<ListBox> = e.new_object();
            e.call_log = Some(vec![]);
            assert!(!e.call(functions[4], &args![list]).bool());
            assert_eq!(e.mem.u32(kind.enabled.cache), 0x71);
            assert_eq!(e.mem.u32(kind.enabled.guard) & 1, 1);
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            assert!(e.call(functions[4], &args![list]).bool());
            assert_eq!(
                calls_to(&e, ADD_USER_TRAIT),
                vec![vec![ENABLED_TEXT, 0xffff_ffff]]
            );
            assert_eq!(calls_to(&e, TILE_IS_TRUE), vec![vec![0x9100, 0x71]]);
        }
    }

    #[test]
    fn list_box_highlight_needs_an_enabled_item() {
        for (_, functions) in INSTANCES {
            let mut e = engine();
            let list: Ptr<ListBox> = e.new_object();
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            returns(&mut e, ADD_USER_TRAIT, 0x71);
            returns(&mut e, TILE_IS_TRUE, 1);
            returns(&mut e, LIST_BOX_BELONGS_TO, 0);
            returns(&mut e, TILE_GET_VALUE, 0x3900);
            returns(&mut e, TILE_FLAG_GETTER_0071AA80, 1);
            returns(&mut e, TILE_FIND_BY_NAME, 0);
            quiet(
                &mut e,
                &[VALUE_ADD_ACTION_TILE, VALUE_CALCULATE, TILE_SET_INT],
            );
            assert!(!e.call(functions[3], &args![list, 0x7400u32]).bool());
            returns(&mut e, LIST_BOX_BELONGS_TO, 1);
            assert!(e.call(functions[3], &args![list, 0x7400u32]).bool());
            assert_eq!(e.get(list, ListBox::pHighlightedItem).addr(), 0x7400);
            // A null tile clears the highlight.
            assert!(e.call(functions[3], &args![list, 0u32]).bool());
            assert!(e.get(list, ListBox::pHighlightedItem).is_null());
        }
    }

    #[test]
    fn list_box_update_highlight_box_pos_marks_the_box() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            let list: Ptr<ListBox> = e.new_object();
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            e.set(list, ListBox::pHighlightedItem, Ptr::new(0x7400));
            e.register(ADD_USER_TRAIT, |_, a| Ret {
                eax: a[0] & 0xff,
                ..Ret::default()
            });
            e.register(TILE_GET_VALUE, |_, a| Ret {
                eax: 0x3900 + a[1],
                ..Ret::default()
            });
            returns(&mut e, TILE_FLAG_GETTER_0071AA80, 1);
            returns(&mut e, TILE_FIND_BY_NAME, 0x3a00);
            e.register(TILE_GET_CHILD_BY_NAME, |_, a| Ret {
                eax: a[1] & 0xfff,
                ..Ret::default()
            });
            quiet(
                &mut e,
                &[
                    VALUE_ADD_ACTION_TILE,
                    VALUE_CALCULATE,
                    TILE_FLAG_SETTER_0071AA60,
                    TILE_ADD_NEEDS_UPDATE,
                ],
            );
            e.call_log = Some(vec![]);
            e.call(functions[5], &args![list]);
            let y = HIGHLIGHT_Y_TEXT & 0xff;
            let height = SELECTED_HEIGHT_TEXT & 0xff;
            assert_eq!(e.mem.u32(kind.highlight_y.cache), y);
            assert_eq!(e.mem.u32(kind.selected_height.cache), height);
            assert_eq!(e.mem.u32(kind.highlight_y.guard) & 3, 3);
            assert_eq!(
                calls_to(&e, VALUE_ADD_ACTION_TILE),
                vec![
                    vec![0x3900 + y, 0x7d0, 0x7400, 0xfa2],
                    vec![0x3900 + height, 0x7d0, 0x7400, 0xfb0]
                ]
            );
            let marked: Vec<u32> = calls_to(&e, TILE_FLAG_SETTER_0071AA60)
                .iter()
                .map(|c| c[0])
                .collect();
            assert_eq!(
                marked,
                vec![
                    TOP_TEXT & 0xfff,
                    BOTTOM_TEXT & 0xfff,
                    LEFT_TEXT & 0xfff,
                    RIGHT_TEXT & 0xfff,
                    0x3a00
                ]
            );
            assert!(calls_to(&e, TILE_FLAG_SETTER_0071AA60)
                .iter()
                .all(|c| c[1] == 1));
        }
    }

    #[test]
    fn list_box_set_total_list_height_computes_the_steps() {
        for (kind, functions) in INSTANCES {
            let mut e = engine();
            e.set_global(ZERO_DOUBLE, 0.0f64);
            let list: Ptr<ListBox> = e.new_object();
            e.set(list, ListBox::pContainerRect, Ptr::new(0x9100));
            e.set(list, ListBox::pScrollBar, Ptr::new(0x9200));
            e.set(list, ListBox::fTotalListHeight, 300.0);
            returns_float(&mut e, TILE_GET_FLOAT, 100.0);
            returns(&mut e, kind.num_unfiltered, 6);
            e.register(ROUND_FLOAT, |_, a| Ret {
                st0: f64::from(f32::from_bits(a[0])),
                ..Ret::default()
            });
            e.register(FTOL2, |_, a| Ret {
                eax: f64::take(a, &mut 0) as i32 as u32,
                ..Ret::default()
            });
            e.register(INT_PAIR_00647B70, |_, a| Ret {
                eax: a[0].max(a[1]),
                ..Ret::default()
            });
            e.register(TEXT_TO_TRAIT, |_, a| Ret {
                eax: a[0] & 0xff,
                ..Ret::default()
            });
            quiet(&mut e, &[TILE_SET_FLOAT, TILE_SET_INT]);
            e.call_log = Some(vec![]);
            e.call(functions[6], &args![list]);
            // 6 * (300 - 100) / 300 = 4: five steps of (300 - 100) / 4.
            assert_eq!(calls_to(&e, INT_PAIR_00647B70), vec![vec![5, 2]]);
            assert_eq!(
                calls_to(&e, TILE_SET_FLOAT),
                vec![vec![0x9100, SCROLL_DELTA_TEXT & 0xff, 50.0f32.to_bits(), 1]]
            );
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x9200, NUMBER_OF_ITEMS_TEXT & 0xff, 5]]
            );
            // A list that fits: one step, no scroll delta.
            e.set(list, ListBox::fTotalListHeight, 50.0);
            e.call_log = Some(vec![]);
            e.call(functions[6], &args![list]);
            assert!(!called(&e, TILE_SET_FLOAT));
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x9200, NUMBER_OF_ITEMS_TEXT & 0xff, 1]]
            );
        }
    }
}

#[cfg(test)]
mod n056_tests {
    use super::*;

    /// Every callee outside this file the batch's translations call.
    const EXTERNALS: &[u32] = &[
        ACTOR_LOCK_DATA_CONSTRUCT,
        ADD_USER_TRAIT,
        ADJUST_FOR_NORTH_ROTATION,
        ADJUST_MAP_MARKER_COORD,
        AFTER_NOTE_SOUND_007624C0,
        ALLOCATE,
        ALLOCATE_00AA1070,
        ALLOCATE_00AA13E0,
        ALPHA_PROPERTY_CONSTRUCT_004391C0,
        ALPHA_PROPERTY_SET_00439340,
        ALPHA_PROPERTY_SET_00439390,
        ALPHA_PROPERTY_SET_0049ED90,
        ARRAY_ADD_SLOT_00845460,
        ARRAY_ELEMENT_006950D0,
        ARRAY_FIRST_006CEDB0,
        ARRAY_FREE_006A8500,
        ARRAY_MOVE_00692480,
        ARRAY_SHOULD_SHRINK_006F3170,
        ARRAY_SHRUNK_SIZE_00869600,
        ARRAY_SIZE_0044DDC0,
        ATAN2,
        AUDIO_EXIT_DIALOGUE,
        AUDIO_INSTANCE,
        AUDIO_SET_RADIO_VOLUME,
        AUDIO_SOUND_BY_NAME,
        AV_OBJECT_ATTACH_PROPERTY,
        AV_OBJECT_GET_PROPERTY,
        AV_OBJECT_UPDATE_PROPERTIES,
        BS_STRISTR,
        BUFFER_COPY_00406D30,
        BUFFER_FORMAT,
        BUILD_TELEPORT_DOOR_PATH,
        BYTE_OF_00408D60,
        CELL_ADD_MARKERS_0054DCF0,
        CELL_GET_EXTERIOR_TEXTURE_0054E640,
        CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE,
        CELL_GET_INT_SEEN_SECTION,
        CELL_GET_SEEN_VALUE,
        CELL_NORTH_ROTATION_00555AD0,
        CHALLENGE_KIND_006A1B60,
        CHALLENGE_LIST_00461150,
        CHALLENGE_STATE_005A60D0,
        CHALLENGE_STATE_005A6150,
        CHALLENGE_STATE_005DEF70,
        CHALLENGE_STATE_005F5930,
        CHALLENGE_VALUE_005E3FA0,
        CHALLENGE_VALUE_006286D0,
        CHILD_LIST_REMOVE_0057C730,
        CHILD_OF_006317A0,
        CLOCK_00457FE0,
        COLOR_SET_00414430,
        COMBINE_004A8F20,
        COMPARE_00408B20,
        CONDITION_IS_TRUE,
        CREATE_DIRECTORY_IMPORT,
        CURSOR_X_007697F0,
        D3D_TEXTURE_LOCK_RECT,
        DOOR_FLAG_00517FD0,
        EXTRA_GET_SEEN_DATA,
        FLAG_004B71D0,
        FLAG_00709D40,
        FLOAT_PAIR_00404010,
        FLOAT_PAIR_0040EBD0,
        FLOAT_PAIR_004BD510,
        FLOAT_POWER_00685D20,
        FLOAT_ROOT_004579E0,
        FLOAT_TO_INT_00406D90,
        FONT_CALCULATE_STRING_DIMENSIONS,
        FONT_MANAGER_005BD5B0,
        FORM_BY_ID_004839C0,
        FORM_FLAG_00440DA0,
        FORM_FLAG_004FEE60,
        FORM_NAME_00586440,
        FORM_TYPE,
        FREE,
        FTOL2,
        GEOMETRY_SET_CONSISTENCY,
        GET_MESSAGE_MENU_RESULT,
        GET_PIPBOY,
        GET_RADIO_STATIONS_IN_RANGE,
        INPUT_AXIS_00A23390,
        INTERFACE_MANAGER,
        INTERFACE_PLAY_MENU_SOUND,
        INT_PAIR_00647B70,
        IS_TOP_MENU_ID,
        ITEM_DATA_DELETE,
        LIST_ADD_TAIL_00905820,
        LIST_BOX_BELONGS_TO,
        LIST_BOX_COUNT_00726C40,
        LIST_BOX_FILTER_007314C0,
        LIST_BOX_GET_TILE_FOR,
        LIST_BOX_HAS_FLAG,
        LIST_BOX_INIT_TILE_POSITION,
        LIST_BOX_ITEM_OF_007E1110,
        LIST_BOX_UNREGISTER,
        LIST_CONSTRUCT,
        LIST_CONTAINS_005F65D0,
        LIST_DESTRUCT,
        LIST_FIRST_00559450,
        LIST_INSERT_SORTED_007A7EB0,
        LIST_REMOVE_ALL,
        LIST_SORT_0083FD60,
        LIST_STEP_0057CBE0,
        LOCAL_MAP_READY_0076B610,
        LOCAL_MAP_UV_SCALE_00879D90,
        MAGNITUDE_00408820,
        MARKER_GET_LOCATION_NAME,
        MARKER_GET_TRAVEL_LOC,
        MARKER_GET_VISIBLE,
        MARKER_ICON_KIND_00438F30,
        MARKER_NAME_00438EB0,
        MATRIX_SET_0043D410,
        MENU_RENDER_TEMPLATE,
        NODE_DELETE,
        NODE_GEOMETRY_DATA_005495F0,
        NODE_SET_NEXT,
        NODE_SET_ROTATE_00A59C60,
        NODE_SET_SCALE_00440490,
        NODE_SET_TRANSLATE_004BC1F0,
        NOTE_IMAGE_005E9160,
        NOTE_LIST_004FADC0,
        NOTE_TEXT_OBJECT_005E91D0,
        NOTE_TYPE,
        OBJECTIVE_FLAG_005A5DC0,
        OBJECTIVE_NAME_005EC330,
        OBJECTIVE_SHOWN_005A5E70,
        OBJECTIVE_TARGETS_007D6BB0,
        OBJECT_00877720,
        OBJECT_CELL_005F36F0,
        OBJECT_FIELD_00403DF0,
        OBJECT_FLAG_00456610,
        OBJECT_ITEM_0043B4A0,
        OBJECT_NODE_0045BC00,
        OBJECT_SET_FLAG_00450F90,
        OBJECT_VALUE_ADDRESS_00403E20,
        PATHING_LOCATION_ASSIGN,
        PATHING_LOCATION_CONSTRUCT,
        PATHING_LOCATION_CONSTRUCT_AT,
        PATHING_LOCATION_DESTRUCT,
        PATHING_LOCATION_FLAG_006DE250,
        PATHING_LOCATION_OF_REFERENCE,
        PATH_DOOR_00558DD0,
        PATH_SPACE_FLAG_006A1440,
        PIPBOY_FLAG_00974D90,
        PIPBOY_MOVE_KNOB,
        PLACEMENT_NEW_006E6DA0,
        PLAYER_BUILD_PATH_TO_TARGET,
        PLAYER_CELL_008D6F30,
        PLAYER_COMPANIONS_005EB7E0,
        PLAYER_GET_CURRENT_TARGET_LIST,
        PLAYER_MARKER_REFERENCE_0077A400,
        PLAYER_NOTE_AT_00966BB0,
        PLAYER_NOTE_COUNT_00966A50,
        PLAYER_QUEST_005CBB50,
        PLAY_MENU_SOUND,
        PLUS_10,
        POINT2_LENGTH_00589850,
        POINT2_SCALE_006AD180,
        POINT2_SET_00452DC0,
        POINT2_SUBTRACT_004E8880,
        POINT_ADD_0063C8A0,
        POINT_SET_00416870,
        POSITION_REQUEST_CONSTRUCT,
        PROCESS_VALUE_0044EDB0,
        PROPERTY_OFFSET_0077A450,
        PROPERTY_SET_008CC6D0,
        PROPERTY_TYPE_00438220,
        QUEST_FLAG_0059E400,
        QUEST_NODE_END_008256D0,
        QUEST_TARGET_GET_REFERENCE,
        RANDOM_00487F50,
        REFERENCE_FLAG_00477BA0,
        REFERENCE_FLAG_0057B4B0,
        REFERENCE_FLAG_00586210,
        REFERENCE_TELEPORT_00568E50,
        REFR_GET_DISTANCE_FROM_REFERENCE,
        REFR_GET_INTERIOR,
        REFR_GET_MAP_MARKER_DATA,
        REFR_GET_WORLD_SPACE,
        REMOVE_PLAYER_MAP_MARKER,
        RENDERER_0043C4B0,
        RENDERER_BEGIN_004A0370,
        RENDERER_END_004A03C0,
        RENDERER_VALUE_004DC200,
        REPUTATION_GET_TITLE,
        REQUEST_POSITION_PLAYER,
        ROUND_FLOAT,
        RT_DYNAMIC_CAST,
        SAVE_FORM_BUFFER_GET_FORM,
        SAVE_TEXTURE_JUMP_00EE6DC2,
        SCISSOR_TRI_SHAPE_CONSTRUCT,
        SCREEN_HEIGHT_00715DA0,
        SCREEN_WIDTH_00715D40,
        SELF,
        SET_CURRENT_FOCUS_TARGET,
        SET_INVENTORY_MENU_VISIBLE,
        SET_MESSAGE_CALLBACK_0070F690,
        SET_PLAYER_MAP_MARKER,
        SET_STATS_MENU_VISIBLE,
        SET_TAB_KNOB_POSITION,
        SET_TILE_TEXTURE,
        SHADER_MANAGER_PREPARE_OBJECT,
        SHOW_SUBTITLE,
        SOUND_GET_DURATION,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_DESTRUCT,
        SOUND_HANDLE_FLAG_0076B780,
        SOUND_IS_PLAYING,
        SOUND_IS_VALID,
        SOUND_LIST_CLEAR_0076B7A0,
        SOUND_NODE_CONSTRUCT_0076B900,
        SOUND_NODE_EMPTY_0076A3B0,
        SOUND_PLAY,
        SOUND_STOP,
        SPRINTF,
        STATION_NAME_0055D520,
        STRICMP,
        STRING_ASSIGN_00501C10,
        STRING_CONSTRUCT,
        STRING_DESTRUCT,
        STRING_TEXT,
        SYSTEM_COLOR_MANAGER_GET_COLOR,
        SYSTEM_COLOR_MANAGER_GET_INSTANCE,
        TARGET_CONDITION_00717E50,
        TARGET_LOCAL_REFERENCE_005CBB70,
        TELEPORT_GET_CELL_MAP_NAME,
        TELEPORT_PATH_CONSTRUCT,
        TELEPORT_PATH_DESTRUCT,
        TERRAIN_OBJECT_0043B230,
        TES_GET_WORLD_SPACE,
        TEXTURE_LOOKUP_00653270,
        TEXTURE_POINTER_SET_0066B0D0,
        TEXT_COMPARE_00404DC0,
        TEXT_COMPARE_00EC7750,
        TEXT_COMPARE_N_004564F0,
        TEXT_LENGTH_0044A670,
        TEXT_TO_TRAIT,
        TILE_ADD_FADE_CONTROL,
        TILE_ADD_NEEDS_UPDATE,
        TILE_BUILD_TILE,
        TILE_FIND_BY_NAME,
        TILE_FLAG_GETTER_0071AA80,
        TILE_FLAG_SETTER_0071AA60,
        TILE_GETTER_00A013D0,
        TILE_GETTER_00A01440,
        TILE_GET_CHILD_BY_ID,
        TILE_GET_CHILD_BY_NAME,
        TILE_GET_FLOAT,
        TILE_GET_MENU,
        TILE_GET_MENU_TILE,
        TILE_GET_STRING,
        TILE_GET_VALUE,
        TILE_GET_VALUE_Q,
        TILE_HAS_FADE_CONTROL,
        TILE_IS_TRUE,
        TILE_NAME_00700300,
        TILE_OBJECT_0056C7F0,
        TILE_POKE,
        TILE_RELEASE,
        TILE_SET_COUNT_00715C60,
        TILE_SET_FLOAT,
        TILE_SET_INT,
        TILE_SET_STRING,
        TILE_SHADER_PROPERTY_CONSTRUCT,
        TILE_TEXT_SETTER_00707E10,
        TIMER_NOW,
        VALUE_ADD_ACTION_FLOAT,
        VALUE_ADD_ACTION_TILE,
        VALUE_CALCULATE,
        VALUE_CLEAR_ACTIONS,
        VECTOR_CONSTRUCTOR_00401050,
        WAVEFORM_LOCAL_CONSTRUCT_00633C90,
        WAVEFORM_LOCAL_DESTRUCT_0045CEC0,
        WHEEL_DELTA_0070EC70,
        WORD_AT_14,
        WORD_AT_174,
        WORD_AT_28,
        WORD_AT_4,
        WORD_AT_C,
        WORD_POINTER_0043D4D0,
        WORLD_BUILD_MAP_MARKER_LIST,
        WORLD_GET_CELL_FROM_CELL_COORD,
        WORLD_GET_PARENT,
        WORLD_GET_TERRAIN_MANAGER,
        WORLD_GET_WORLD_MAP_DATA,
    ];

    /// An engine with the statics' and constants' pages mapped and every
    /// external callee doubled by a quiet double (0 in `EAX` and `ST0`);
    /// `006815c0` returns `this`, `00726070` the word at +4 and `0084e3a0`
    /// the word at +0xC, as the list code needs.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0101_e000,
            0x0101_f000,
            0x0102_0000,
            0x0102_2000,
            0x0102_3000,
            0x0102_9000,
            0x0102_e000,
            0x0102_f000,
            0x0103_0000,
            0x0103_1000,
            0x0103_4000,
            0x0105_6000,
            0x0106_e000,
            0x0106_f000,
            0x0107_0000,
            0x0107_1000,
            0x0107_4000,
            0x0107_5000,
            0x011a_0000,
            0x011a_9000,
            0x011c_3000,
            0x011c_6000,
            0x011d_2000,
            0x011d_3000,
            0x011d_4000,
            0x011d_8000,
            0x011d_a000,
            0x011d_b000,
            0x011d_d000,
            0x011d_e000,
            0x011e_0000,
            0x011f_4000,
            0x011f_6000,
            0x011f_f000,
        ] {
            e.map(page, 0x1000);
        }
        for &addr in EXTERNALS {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(WORD_AT_C, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        // The quest markers setting reads as off.
        let off = e.mem.alloc(4);
        returns(&mut e, BYTE_OF_00408D60, off);
        e
    }

    /// A double returning `value` in `EAX`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double returning `value` in `ST0`.
    fn returns_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// A double returning `values` in `EAX`, one per call (the last again
    /// after the end).
    fn returns_each(e: &mut Engine, addr: u32, values: &[u32]) {
        let values = values.to_vec();
        let mut next = 0usize;
        e.register_double(addr, move |_, _| {
            let value = values[next.min(values.len() - 1)];
            next += 1;
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The order of the calls to `addrs` in the log.
    fn order_of(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addrs.contains(a))
            .map(|(a, _)| *a)
            .collect()
    }

    /// A map menu object with `pTiles` filled (`tile_base + index`) made the
    /// open one (`pMe`).
    fn open_menu(e: &mut Engine, tile_base: u32) -> Ptr<MapMenu> {
        let menu: Ptr<MapMenu> = e.new_object();
        for index in 0..0x16 {
            e.mem
                .set_u32(menu.addr() + 0x28 + 4 * index, tile_base + index);
        }
        e.mem.set_u32(MAP_MENU_SINGLETON, menu.addr());
        menu
    }

    /// An object whose vtable has `slots` (byte offset, target) and room
    /// for `size` bytes.
    fn with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x400);
        for &(slot, target) in slots {
            e.mem.set_u32(table + slot, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    /// A list node chain over `items` (item at +0, next at +4); returns the
    /// first node.
    fn node_chain(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// A `ListBoxItem` (tile, data, filtered byte).
    fn list_item(e: &mut Engine, tile: u32, data: u32, filtered: u8) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        e.mem.set_u8(item + 8, filtered);
        item
    }

    /// A list box whose embedded list (+4) holds `items` (the first in the
    /// list box itself, the rest in nodes).
    fn list_box_with(e: &mut Engine, items: &[u32]) -> Ptr<ListBox> {
        let list: Ptr<ListBox> = e.new_object();
        if let Some((&first, rest)) = items.split_first() {
            e.mem.set_u32(list.addr() + 4, first);
            let chain = node_chain(e, rest);
            e.mem.set_u32(list.addr() + 8, chain);
        }
        list
    }

    #[test]
    fn fn_007983c0_scales_the_cursor_y_to_960_lines() {
        let mut e = engine();
        e.set_global::<f64>(SCALE_0106E7F8, 960.0);
        returns(&mut e, RENDERER_VALUE_004DC200, 480);
        let interface = e.mem.alloc(0x500);
        e.mem.set_u32(interface + 0x4b0, 100.0f32.to_bits());
        let value = e.call(0x0079_83c0, &args![interface]).f64();
        assert_eq!(value, 200.0);
    }

    #[test]
    fn small_getters_read_their_fields() {
        let mut e = engine();
        let object = e.mem.alloc(0x700);
        e.mem.set_u32(object + 0x6f4, 0x1234);
        e.mem.set_u8(object + 0x7d, 7);
        e.mem.set_u16(object + 0x5c, 4);
        e.mem.set_u8(FLAG_011DD434, 9);
        e.mem.set_u32(VALUE_011DD42C, 0x55);
        assert_eq!(e.call(0x0079_8400, &args![object]).u32(), 0x1234);
        assert_eq!(e.call(0x0079_8420, &args![object]).u8(), 7);
        assert_eq!(e.call(0x0079_8440, &args![]).u8(), 9);
        assert!(e.call(0x0079_b800, &args![object]).bool());
        e.mem.set_u16(object + 0x5c, 3);
        assert!(!e.call(0x0079_b800, &args![object]).bool());
        assert_eq!(e.call(0x0079_c070, &args![]).u32(), 0x55);
        assert_eq!(e.call(0x0079_c080, &args![]).u32(), 0x011d_d59c);
        assert_eq!(e.call(0x0079_f620, &args![object]).u32(), object + 0x6f8);
    }

    #[test]
    fn do_down_click_keeps_the_drag_start_on_the_world_map_only() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: f64::from(a[1] - 0xfa0),
            ..Ret::default()
        });
        e.call(0x0079_8480, &args![menu, 3i32, 0u32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x120), 0);
        e.call(0x0079_8480, &args![menu, 4i32, 0u32]);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0x120)), 1.0);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0x124)), 2.0);
    }

    #[test]
    fn do_controller_switch_moves_tile_6() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.mem.set_u32(CONTROLLER_X_FLOAT, 348.0f32.to_bits());
        e.mem.set_u32(CONTROLLER_Y_FLOAT, 170.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x0079_a610, &args![menu, 0u32]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        e.call(0x0079_a610, &args![menu, 1u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x106, 0xfa1, 348.0f32.to_bits(), 1],
                vec![0x106, 0xfa2, 170.0f32.to_bits(), 1],
            ]
        );
    }

    #[test]
    fn fn_0079b430_clears_only_the_matching_selected_note() {
        let mut e = engine();
        e.call(0x0079_b430, &args![5u32]);
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::pSelectedNote, 7);
        e.call(0x0079_b430, &args![5u32]);
        assert_eq!(e.get(menu, MapMenu::pSelectedNote), 7);
        e.call(0x0079_b430, &args![7u32]);
        assert_eq!(e.get(menu, MapMenu::pSelectedNote), 0);
    }

    #[test]
    fn fn_0079bc50_compares_with_the_displayed_quest() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::pDisplayedQuest, 0x77);
        returns(&mut e, PROCESS_VALUE_0044EDB0, 0x77);
        assert_eq!(e.call(0x0079_bc50, &args![0u32]).u8(), 0);
        assert_eq!(e.call(0x0079_bc50, &args![9u32]).u8(), 0);
        returns(&mut e, PROCESS_VALUE_0044EDB0, 0x78);
        assert_eq!(e.call(0x0079_bc50, &args![9u32]).u8(), 1);
    }

    #[test]
    fn fn_0079be30_filters_null_and_flagged_stations() {
        let mut e = engine();
        assert_eq!(e.call(0x0079_be30, &args![0u32]).u8(), 1);
        assert_eq!(e.call(0x0079_be30, &args![9u32]).u8(), 0);
        returns(&mut e, SAVE_FORM_BUFFER_GET_FORM, 0x44);
        e.call_log = Some(vec![]);
        returns(&mut e, FORM_FLAG_004FEE60, 1);
        assert_eq!(e.call(0x0079_be30, &args![9u32]).u8(), 1);
        assert_eq!(calls_to(&e, FORM_FLAG_004FEE60), vec![vec![0x44, 0]]);
        returns(&mut e, FORM_FLAG_00440DA0, 1);
        assert_eq!(e.call(0x0079_be30, &args![9u32]).u8(), 1);
    }

    #[test]
    fn fn_0079be80_clears_the_selected_trait() {
        let mut e = engine();
        e.mem.set_u32(TRAIT_SELECTED, 0x3344);
        e.call_log = Some(vec![]);
        e.call(0x0079_be80, &args![0x99u32]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x99, 0x3344, 0]]);
    }

    #[test]
    fn fn_0079c340_moves_the_tab_knob_to_the_page() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0079_c340, &args![1u8]);
        assert!(calls_to(&e, SET_TAB_KNOB_POSITION).is_empty());
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::iCurrentPage, 0x22);
        returns(&mut e, WORD_AT_174, 0x5000);
        e.call(0x0079_c340, &args![1u8]);
        assert_eq!(
            calls_to(&e, SET_TAB_KNOB_POSITION),
            vec![vec![0x5000, 2, 1]]
        );
    }

    #[test]
    fn fn_007a1400_clears_set_bits_of_the_mask() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x30, 0x31);
        e.call(0x007a_1400, &args![object, 0x20u32]);
        assert_eq!(e.mem.u32(object + 0x30), 0x11);
        e.call(0x007a_1400, &args![object, 0x40u32]);
        assert_eq!(e.mem.u32(object + 0x30), 0x11);
    }

    #[test]
    fn fn_007a1430_and_007a1450_store_their_values() {
        let mut e = engine();
        let object = e.mem.alloc(0x100);
        e.call(0x007a_1430, &args![object, 5u8]);
        assert_eq!(e.mem.u8(object + 0x92), 5);
        let source = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(source + 4 * word, 10 + word);
        }
        e.call(0x007a_1450, &args![object, source]);
        for word in 0..4 {
            assert_eq!(e.mem.u32(object + 0x68 + 4 * word), 10 + word);
        }
    }

    #[test]
    fn fn_007a1750_compares_the_next_grid_with_the_squared_side() {
        let mut e = engine();
        assert!(!e.call(0x007a_1750, &args![]).bool());
        let menu = open_menu(&mut e, 0x100);
        let side = e.mem.alloc(4);
        e.mem.set_u32(side, 3);
        returns(&mut e, WORD_POINTER_0043D4D0, side);
        e.set(menu, MapMenu::uiNextLocalMapGrid, 8);
        assert!(e.call(0x007a_1750, &args![]).bool());
        e.set(menu, MapMenu::uiNextLocalMapGrid, 9);
        assert!(!e.call(0x007a_1750, &args![]).bool());
    }

    #[test]
    fn magnification_setters_use_tiles_2_and_4() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x007a_1820, &args![2.0f32]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        open_menu(&mut e, 0x100);
        e.mem.set_u32(TRAIT_MAGNIFICATION, 0x77);
        e.call(0x007a_1820, &args![2.0f32]);
        e.call(0x007a_1850, &args![3.0f32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x102, 0x77, 2.0f32.to_bits(), 1],
                vec![0x104, 0x77, 3.0f32.to_bits(), 1],
            ]
        );
    }

    #[test]
    fn fn_007a1ac0_appends_a_string() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(0x40);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        returns(&mut e, ARRAY_ADD_SLOT_00845460, 2);
        returns(&mut e, PLACEMENT_NEW_006E6DA0, buffer + 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_1ac0, &args![array, 0x5555u32]).u32(), 2);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW_006E6DA0),
            vec![vec![8, buffer + 0x10]]
        );
        assert_eq!(calls_to(&e, STRING_CONSTRUCT), vec![vec![buffer + 0x10]]);
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_00501C10),
            vec![vec![buffer + 0x10, 0x5555]]
        );
    }

    #[test]
    fn deleting_destructors_free_on_flag_one() {
        for (address, destructor) in [
            (0x007a_6100u32, 0x007a_1aa0u32),
            (0x007a_6130, 0x007a_1d60),
            (0x007a_6160, 0x007a_28c0),
            (0x007a_6190, 0x007a_37b0),
            (0x007a_61c0, 0x007a_4470),
            (0x007a_61f0, 0x007a_51c0),
            (0x007a_86b0, STRING_DESTRUCT),
        ] {
            let mut e = engine();
            // The list box destructors run for real (direct calls); their
            // callees outside this file are doubled.
            for addr in [destructor, LIST_BOX_UNREGISTER, LIST_DESTRUCT] {
                e.register(addr, |_, _| Ret::default());
            }
            let object = e.mem.alloc(0x40);
            e.call_log = Some(vec![]);
            let result = e.call(address, &args![object, 0u32]).u32();
            assert_eq!(result, object);
            assert!(calls_to(&e, FREE).is_empty());
            e.call(address, &args![object, 1u32]);
            assert_eq!(calls_to(&e, FREE), vec![vec![object]]);
        }
    }

    #[test]
    fn fn_007a6310_and_007a63b0_walk_the_elements() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x20);
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007a_6310, &args![0u32, buffer, 2u32]);
        assert_eq!(
            calls_to(&e, STRING_CONSTRUCT),
            vec![vec![buffer], vec![buffer + 8]]
        );
        e.call(0x007a_63b0, &args![0u32, buffer, 3u32]);
        assert_eq!(
            calls_to(&e, STRING_DESTRUCT),
            vec![vec![buffer], vec![buffer + 8], vec![buffer + 16]]
        );
        assert!(calls_to(&e, FREE).is_empty());
    }

    #[test]
    fn fn_007a5a30_sets_the_container_trait() {
        let mut e = engine();
        let list = list_box_with(&mut e, &[]);
        e.set(list, ListBox::pContainerRect, Ptr::new(0x900));
        e.call_log = Some(vec![]);
        e.call(0x007a_5a30, &args![list, 1u8]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x900, 0xfa3, 1]]);
    }

    /// The list box instances: (Sort, Synchronize, SetTotalListHeight's
    /// `num_unfiltered`) per kind.
    const SORTS: [u32; 5] = [
        0x007a_22a0,
        0x007a_2e00,
        0x007a_3cf0,
        0x007a_49b0,
        0x007a_5700,
    ];

    #[test]
    fn list_box_sort_sorts_then_synchronizes() {
        for address in SORTS {
            let mut e = engine();
            let first = list_item(&mut e, 0x501, 0, 0);
            let second = list_item(&mut e, 0x502, 0, 1);
            let list = list_box_with(&mut e, &[first, second]);
            e.call_log = Some(vec![]);
            e.call(address, &args![list, 0x0040_0000u32]);
            assert_eq!(
                calls_to(&e, LIST_SORT_0083FD60),
                vec![vec![list.addr() + 4, 0x0040_0000]]
            );
            // Synchronize: only the unfiltered item gets an index.
            assert_eq!(calls_to(&e, TILE_SET_INT)[0], vec![0x501, 0xfac, 0]);
            assert_eq!(
                calls_to(&e, LIST_BOX_INIT_TILE_POSITION),
                vec![vec![list.addr(), 0x501]]
            );
            assert_eq!(e.get(list, ListBox::usNextIndex), 1);
        }
    }

    #[test]
    fn list_box_synchronize_indexes_the_unfiltered_items() {
        for address in [
            0x007a_6480u32,
            0x007a_6b00,
            0x007a_7260,
            0x007a_78e0,
            0x007a_80e0,
        ] {
            let mut e = engine();
            let first = list_item(&mut e, 0x501, 0, 1);
            let second = list_item(&mut e, 0x502, 0, 0);
            let third = list_item(&mut e, 0x503, 0, 0);
            let list = list_box_with(&mut e, &[first, second, third]);
            e.set(list, ListBox::usNextIndex, 9);
            e.set(list, ListBox::fTotalListHeight, 5.0);
            e.call_log = Some(vec![]);
            e.call(address, &args![list]);
            let indexed: Vec<_> = calls_to(&e, TILE_SET_INT)
                .into_iter()
                .filter(|c| c[1] == 0xfac)
                .collect();
            assert_eq!(indexed, vec![vec![0x502, 0xfac, 0], vec![0x503, 0xfac, 1]]);
            assert_eq!(e.get(list, ListBox::usNextIndex), 2);
            assert_eq!(e.get(list, ListBox::fTotalListHeight), 0.0);
        }
    }

    #[test]
    fn list_box_set_enabled_looks_its_static_up_once() {
        for (address, guard) in [
            (0x007a_2350u32, 0x011d_a420u32),
            (0x007a_30e0, 0x011d_a430),
            (0x007a_4c50, 0x011d_a448),
            (0x007a_59a0, 0x011d_a458),
        ] {
            let mut e = engine();
            returns(&mut e, ADD_USER_TRAIT, 0x4444);
            let list = list_box_with(&mut e, &[]);
            e.set(list, ListBox::pContainerRect, Ptr::new(0x900));
            e.call_log = Some(vec![]);
            e.call(address, &args![list, 1u8]);
            e.call(address, &args![list, 0u8]);
            assert_eq!(calls_to(&e, ADD_USER_TRAIT).len(), 1);
            assert_eq!(e.mem.u32(guard) & 1, 1);
            assert_eq!(
                calls_to(&e, TILE_SET_INT),
                vec![vec![0x900, 0x4444, 1], vec![0x900, 0x4444, 0]]
            );
        }
    }

    /// A list box of `count` enabled items whose index trait is their
    /// position, with tile `0x600 + index`.
    fn indexed_list(e: &mut Engine, count: u32) -> Ptr<ListBox> {
        let items: Vec<u32> = (0..count).map(|i| list_item(e, 0x600 + i, 0, 0)).collect();
        let list = list_box_with(e, &items);
        e.set(list, ListBox::pContainerRect, Ptr::new(0x900));
        e.set(list, ListBox::usNextIndex, count as u16);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: f64::from(a[0].wrapping_sub(0x600)),
            ..Ret::default()
        });
        e.register(FTOL2, |_, a| Ret {
            eax: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.register(TILE_IS_TRUE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        list
    }

    #[test]
    fn list_box_get_highlight_target_steps_through_the_items() {
        for address in [
            0x007a_2450u32,
            0x007a_31e0,
            0x007a_4000,
            0x007a_4d50,
            0x007a_5ad0,
        ] {
            let mut e = engine();
            let list = indexed_list(&mut e, 4);
            // No highlight: down starts at the first item, up at the last.
            assert_eq!(e.call(address, &args![list, 2u32]).u32(), 0x600);
            assert_eq!(e.call(address, &args![list, 1u32]).u32(), 0x603);
            // From the item at index 1: down gives index 2.
            e.set(list, ListBox::pHighlightedItem, Ptr::new(0x601));
            assert_eq!(e.call(address, &args![list, 2u32]).u32(), 0x602);
            // Up from index 0 stays at 0, the highlighted one: none.
            e.set(list, ListBox::pHighlightedItem, Ptr::new(0x600));
            assert_eq!(e.call(address, &args![list, 1u32]).u32(), 0);
            // Other directions do nothing.
            assert_eq!(e.call(address, &args![list, 5u32]).u32(), 0);
        }
    }

    #[test]
    fn list_box_get_highlight_target_walks_back_past_disabled_items() {
        let mut e = engine();
        let list = indexed_list(&mut e, 3);
        // The last item is disabled; the step down from index 1 walks back
        // from last - 1 through vtable +0x14.
        e.register(TILE_IS_TRUE, |_, a| Ret {
            eax: u32::from(a[0] != 0x602),
            ..Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x14, 0x0010_0014);
        e.mem.set_u32(list.addr(), table);
        e.register(0x0010_0014, |_, a| Ret {
            eax: 0x600 + a[1],
            ..Ret::default()
        });
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x601));
        e.call_log = Some(vec![]);
        // Index 2 is disabled, 3 is past the end: index 1 through the
        // vtable is the highlighted one again, so the result is 0.
        assert_eq!(e.call(0x007a_2450, &args![list, 2u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0010_0014), vec![vec![list.addr(), 1, 0]]);
    }

    #[test]
    fn list_box_get_highlight_target_pages_by_the_scroll_delta() {
        let mut e = engine();
        let list = indexed_list(&mut e, 6);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: match a[1] {
                0xfb0 => 30.0,
                0xfac => f64::from(a[0].wrapping_sub(0x600)),
                _ => 10.0,
            },
            ..Ret::default()
        });
        e.register(FLOAT_PAIR_004BD510, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x601));
        assert_eq!(e.call(0x007a_2450, &args![list, 0x10u32]).u32(), 0x604);
        e.set(list, ListBox::pHighlightedItem, Ptr::new(0x605));
        assert_eq!(e.call(0x007a_2450, &args![list, 0xfu32]).u32(), 0x602);
    }

    #[test]
    fn list_box_insert_renders_and_appends_or_sorts() {
        for address in [
            0x007a_1dd0u32,
            0x007a_2930,
            0x007a_3820,
            0x007a_44e0,
            0x007a_5230,
        ] {
            let mut e = engine();
            let list = list_box_with(&mut e, &[]);
            // Without a container or template: 0.
            assert_eq!(
                e.call(address, &args![list, 1u32, 0u32, 0u32, 0u32]).u32(),
                0
            );
            e.set(list, ListBox::pContainerRect, Ptr::new(0x900));
            assert_eq!(
                e.call(address, &args![list, 1u32, 0u32, 0u32, 0u32]).u32(),
                0
            );
            e.set(list, ListBox::pTemplateName, Ptr::new(0x0107_0000));
            returns(&mut e, TILE_GET_MENU, 0x800);
            returns(&mut e, MENU_RENDER_TEMPLATE, 0x700);
            let memory = e.mem.alloc(0xc);
            returns(&mut e, ALLOCATE, memory);
            returns(&mut e, LIST_BOX_HAS_FLAG, 1);
            returns(&mut e, LIST_BOX_COUNT_00726C40, 1);
            returns(&mut e, ADD_USER_TRAIT, 0x3030);
            returns_float(&mut e, TILE_GET_FLOAT, 4.0);
            returns(&mut e, TILE_GET_VALUE_Q, 0x5050);
            e.call_log = Some(vec![]);
            let tile = e
                .call(address, &args![list, 0x77u32, 0x0107_1000u32, 0u32, 0u32])
                .u32();
            assert_eq!(tile, 0x700);
            assert_eq!(
                calls_to(&e, MENU_RENDER_TEMPLATE),
                vec![vec![0x800, 0x900, 0x0107_0000, 0]]
            );
            assert_eq!(
                calls_to(&e, TILE_SET_INT)[0],
                vec![0x700, 0xfaa, 0xffff_ffff]
            );
            assert_eq!(
                calls_to(&e, TILE_SET_STRING),
                vec![vec![0x700, 0xfc4, 0x0107_1000, 1]]
            );
            // The item holds the tile and the data word.
            assert_eq!(e.mem.u32(memory), 0x700);
            assert_eq!(e.mem.u32(memory + 4), 0x77);
            let appended = calls_to(&e, LIST_ADD_TAIL_00905820);
            assert_eq!(appended.len(), 1);
            assert_eq!(appended[0][0], list.addr() + 4);
            assert_eq!(e.get(list, ListBox::usNextIndex), 1);
            assert_eq!(
                calls_to(&e, VALUE_ADD_ACTION_FLOAT),
                vec![vec![0x5050, 0x7d3, 4.0f32.to_bits()]]
            );
            // With a compare function the item is inserted sorted.
            e.call_log = Some(vec![]);
            e.call(address, &args![list, 0x78u32, 0u32, 0x0040_0000u32, 0u32]);
            assert_eq!(
                calls_to(&e, LIST_INSERT_SORTED_007A7EB0),
                vec![vec![list.addr() + 4, memory, 0x0040_0000]]
            );
        }
    }

    #[test]
    fn list_box_filter_counts_filtered_items_and_keeps_the_highlight() {
        for address in [0x007a_2e30u32, 0x007a_3d20, 0x007a_49e0, 0x007a_5730] {
            let mut e = engine();
            let first = list_item(&mut e, 0x501, 1, 0);
            let second = list_item(&mut e, 0x502, 2, 0);
            let third = list_item(&mut e, 0x503, 3, 0);
            let list = list_box_with(&mut e, &[first, second, third]);
            e.set(list, ListBox::pContainerRect, Ptr::new(0x900));
            e.set(list, ListBox::pHighlightedItem, Ptr::new(0x501));
            // The callback filters out the data 1.
            e.register(0x0040_1234, |_, a| Ret {
                eax: u32::from(a[0] == 1),
                ..Ret::default()
            });
            returns_float(&mut e, TILE_GET_FLOAT, 0.0);
            returns(&mut e, ADD_USER_TRAIT, 0x2222);
            let table = e.mem.alloc(0x20);
            e.mem.set_u32(table + 0x14, 0x0010_0014);
            e.mem.set_u32(list.addr(), table);
            returns(&mut e, 0x0010_0014, 0x777);
            e.call_log = Some(vec![]);
            e.call(address, &args![list, 0x0040_1234u32]);
            assert_eq!(e.mem.u8(first + 8), 1);
            assert_eq!(e.mem.u8(second + 8), 0);
            assert_eq!(
                calls_to(&e, TILE_SET_COUNT_00715C60),
                vec![vec![0x900, 0x2222, 1]]
            );
            // The highlighted item was filtered with index 0; the item that
            // took index 0 became the highlight.
            assert_eq!(e.get(list, ListBox::pHighlightedItem).addr(), 0x502);
            assert_eq!(e.get(list, ListBox::usNextIndex), 2);
            assert!(calls_to(&e, 0x0010_0014).is_empty());
        }
    }

    #[test]
    fn list_box_num_unfiltered_and_last_tile() {
        for (count_address, last_address) in [
            (0x007a_6a50u32, Some(0x007a_6220u32)),
            (0x007a_71b0, Some(0x007a_6250)),
            (0x007a_7830, Some(0x007a_6280)),
            (0x007a_8030, Some(0x007a_62b0)),
            (0x007a_86e0, None),
        ] {
            let mut e = engine();
            let list = list_box_with(&mut e, &[]);
            e.set(list, ListBox::pContainerRect, Ptr::new(0x900));
            returns(&mut e, LIST_BOX_COUNT_00726C40, 7);
            returns_float(&mut e, TILE_GET_FLOAT, 2.9);
            assert_eq!(e.call(count_address, &args![list]).u32(), 5);
            if let Some(last) = last_address {
                let table = e.mem.alloc(0x20);
                e.mem.set_u32(table + 0x14, 0x0010_0014);
                e.mem.set_u32(list.addr(), table);
                returns(&mut e, 0x0010_0014, 0x888);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(last, &args![list]).u32(), 0x888);
                assert_eq!(calls_to(&e, 0x0010_0014), vec![vec![list.addr(), 4, 0]]);
            }
        }
    }

    #[test]
    fn fn_007a3580_restores_the_saved_state() {
        let mut e = engine();
        let list = list_box_with(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x007a_3580, &args![list, 1u8]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        e.set(list, ListBox::pContainerRect, Ptr::new(0x900));
        e.set(list, ListBox::pScrollBar, Ptr::new(0x910));
        e.set(list, ListBox::fSavedScrollbarValue, 3.0);
        e.set(list, ListBox::fSavedHighlightIndex, 2.0);
        e.set_global::<f64>(ONE_DOUBLE, 1.0);
        returns(&mut e, TEXT_TO_TRAIT, 0x1111);
        returns(&mut e, LIST_BOX_COUNT_00726C40, 4);
        returns_float(&mut e, FLOAT_PAIR_0040EBD0, 2.0);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table, 0x0010_0000);
        e.mem.set_u32(table + 0x14, 0x0010_0014);
        e.mem.set_u32(list.addr(), table);
        returns(&mut e, 0x0010_0014, 0x777);
        e.register(0x0010_0000, |_, _| Ret::default());
        returns(&mut e, INTERFACE_MANAGER, 0x4000);
        let menu = with_vtable(&mut e, 0x10, &[(0x14, 0x0010_0114)]);
        returns(&mut e, TILE_GET_MENU, menu);
        e.register(0x0010_0114, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007a_3580, &args![list, 1u8]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x910, 0x1111, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, FLOAT_PAIR_0040EBD0),
            vec![vec![2.0f32.to_bits(), 3.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0010_0014), vec![vec![list.addr(), 2, 0]]);
        assert_eq!(
            calls_to(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x4000, 0x777, 0xfc3, 1]]
        );
        assert_eq!(
            calls_to(&e, 0x0010_0000),
            vec![vec![list.addr(), 0x777], vec![list.addr(), 0]]
        );
    }

    #[test]
    fn calculate_player_marker_space_takes_the_matching_marker() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::pRootMapSpace, 0x3000);
        // Two markers; the second matches.
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        let head = list_box_with(&mut e, &[first, second]).addr() + 4;
        returns(&mut e, PLUS_10, head);
        let name = e.mem.alloc(8);
        e.mem.set_u8(name, b'a');
        returns(&mut e, MARKER_GET_LOCATION_NAME, name);
        returns(&mut e, PATHING_LOCATION_FLAG_006DE250, 1);
        returns(&mut e, TEXT_LENGTH_0044A670, 0x20);
        returns_each(&mut e, TEXT_COMPARE_N_004564F0, &[1, 0]);
        e.call_log = Some(vec![]);
        let found = e.call(0x0079_84d0, &args![menu, 1u32, 2u32, 3u32]).u32();
        assert_eq!(found, second);
        let adjusted = calls_to(&e, ADJUST_MAP_MARKER_COORD);
        assert_eq!(adjusted.len(), 2);
        assert_eq!(adjusted[1][0], second);
        assert_eq!(adjusted[1][2], 1);
        // Every location built is destroyed (the temporary and the local
        // for each marker).
        assert_eq!(calls_to(&e, PATHING_LOCATION_DESTRUCT).len(), 4);
    }

    #[test]
    fn calculate_player_marker_space_falls_back_to_the_first_marker() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let first = e.mem.alloc(0x40);
        let head = list_box_with(&mut e, &[first]).addr() + 4;
        returns(&mut e, PLUS_10, head);
        // No name: skipped.
        let found = e.call(0x0079_84d0, &args![menu, 1u32, 2u32, 3u32]).u32();
        assert_eq!(found, first);
    }

    /// The doubles the cursor-tile centring needs: the interface manager's
    /// cursor, the owner's position and the tile's size.
    fn cursor_doubles(e: &mut Engine) {
        e.set_global::<f64>(TWO_DOUBLE, 2.0);
        returns_float(e, CURSOR_X_007697F0, 100.0);
        returns_float(e, TILE_GETTER_00A013D0, 10.0);
        returns_float(e, TILE_GETTER_00A01440, 20.0);
        returns_float(e, TILE_GET_FLOAT, 8.0);
        returns(e, RENDERER_VALUE_004DC200, 960);
        e.set_global::<f64>(SCALE_0106E7F8, 960.0);
        returns(e, INTERFACE_MANAGER, 0x4000);
        e.map(0x4000, 0x1000);
        e.mem.set_u32(0x4000 + 0x4b0, 50.0f32.to_bits());
    }

    #[test]
    fn fast_travel_callback_hands_on_or_clears_the_marker() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        cursor_doubles(&mut e);
        e.set(menu, MapMenu::pFastTravelMarker, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0079_8710, &args![]);
        assert_eq!(
            calls_to(&e, SET_MESSAGE_CALLBACK_0070F690),
            vec![vec![0x4000, 0x0079_8a00]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x106, 0xfa1, 86.0f32.to_bits(), 1],
                vec![0x106, 0xfa2, 26.0f32.to_bits(), 1],
            ]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x106, 0xfa3, 1]]);
        returns(&mut e, GET_MESSAGE_MENU_RESULT, 1);
        returns(&mut e, FLAG_004B71D0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_8710, &args![]);
        assert_eq!(e.get(menu, MapMenu::pFastTravelMarker), 0);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
    }

    #[test]
    fn set_player_marker_callback_sets_or_removes_the_marker() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        returns(&mut e, FLAG_004B71D0, 1);
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        e.set(menu, MapMenu::PlayerMarkerSpace, 0x99);
        e.set(menu, MapMenu::PlayerMarkerPos, 1.0);
        e.mem.set_u32(POINT_011F426C, 5.0f32.to_bits());
        e.call_log = Some(vec![]);
        // Result 3: set without deleting first.
        returns(&mut e, GET_MESSAGE_MENU_RESULT, 3);
        e.call(0x0079_8840, &args![]);
        assert_eq!(
            calls_to(&e, SET_PLAYER_MAP_MARKER),
            vec![vec![player, 1.0f32.to_bits(), 0, 0, 0x99]]
        );
        assert_eq!(e.get(menu, MapMenu::PlayerMarkerSpace), 0);
        assert_eq!(e.get(menu, MapMenu::PlayerMarkerPos), 5.0);
        // Result 1: removed.
        returns(&mut e, GET_MESSAGE_MENU_RESULT, 1);
        e.call(0x0079_8840, &args![]);
        assert_eq!(calls_to(&e, REMOVE_PLAYER_MAP_MARKER), vec![vec![player]]);
    }

    #[test]
    fn fn_00798a00_requests_the_fast_travel() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x0079_8a00, &args![]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        e.set(menu, MapMenu::pFastTravelMarker, 0x55);
        let request = e.mem.alloc(0x34);
        returns(&mut e, ALLOCATE, request);
        returns(&mut e, POSITION_REQUEST_CONSTRUCT, request);
        e.mem.set_u32(PLAYER_011DEA3C, 0x7000);
        e.call(0x0079_8a00, &args![]);
        assert_eq!(e.mem.u32(request + 0x30), 0x55);
        assert_eq!(
            calls_to(&e, REQUEST_POSITION_PLAYER),
            vec![vec![0x7000, request]]
        );
        assert_eq!(e.get(menu, MapMenu::pFastTravelMarker), 0);
    }

    #[test]
    fn stop_note_sound_and_clear_subtitles_reset_the_sounds() {
        for address in [0x0079_8ad0u32, 0x0079_8bf0] {
            let mut e = engine();
            let menu = open_menu(&mut e, 0x100);
            e.set(menu, MapMenu::pCurrentSound, 0x600);
            e.set(menu, MapMenu::bSoundsLoading, 1);
            e.mem.set_u32(menu.addr() + 0xc4, 9);
            returns(&mut e, SOUND_IS_PLAYING, 1);
            e.call_log = Some(vec![]);
            e.call(address, &args![menu]);
            assert_eq!(calls_to(&e, SOUND_STOP), vec![vec![0x600]]);
            assert_eq!(
                calls_to(&e, SOUND_LIST_CLEAR_0076B7A0),
                vec![vec![menu.addr() + 0x98]]
            );
            assert_eq!(e.get(menu, MapMenu::pCurrentSound), 0);
            assert_eq!(e.get(menu, MapMenu::bSoundsLoading), 0);
            assert_eq!(e.mem.u32(menu.addr() + 0xc4), 0);
            let stopped = !calls_to(&e, AUDIO_EXIT_DIALOGUE).is_empty();
            assert_eq!(stopped, address == 0x0079_8ad0);
        }
    }

    #[test]
    fn do_enter_shows_the_entered_quest() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        returns(&mut e, LIST_BOX_ITEM_OF_007E1110, 0x4400);
        returns(&mut e, TEXT_TO_TRAIT, 0x2020);
        e.call_log = Some(vec![]);
        e.call(0x0079_8cb0, &args![menu, 0x17i32, 0x500u32]);
        assert_eq!(e.get(menu, MapMenu::pDisplayedQuest), 0x4400);
        assert_eq!(
            calls_to(&e, LIST_BOX_FILTER_007314C0),
            vec![vec![menu.addr() + 0x1c0, 0x0079_9100, 0, 0x7fff_ffff]]
        );
        let ints = calls_to(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![0x10d, 0x2020, 4]));
        assert!(ints.contains(&vec![0x10c, 0xfaf, 0]));
        assert!(ints.contains(&vec![0x10b, 0xfaf, 1]));
        // The index is 0 and the last knob index 0: no knob move.
        assert!(calls_to(&e, PIPBOY_MOVE_KNOB).is_empty());
    }

    #[test]
    fn do_enter_moves_the_knob_for_the_radio_list() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let radio = with_vtable(&mut e, 0x10, &[(4, 0x0010_0004)]);
        // The radio list box lives at +0x190: give it the vtable.
        let table = e.mem.u32(radio);
        e.mem.set_u32(menu.addr() + 0x190, table);
        returns(&mut e, 0x0010_0004, 0x601);
        returns(&mut e, FTOL2, 3);
        let step = e.mem.alloc(4);
        e.mem.set_u32(step, 2.0f32.to_bits());
        returns(&mut e, OBJECT_VALUE_ADDRESS_00403E20, step);
        returns(&mut e, WORD_AT_174, 0x5000);
        e.mem.set_u32(KNOB_INDEX_011DA400, 5);
        e.call_log = Some(vec![]);
        e.call(0x0079_8cb0, &args![menu, 0x19i32, 0u32]);
        // Last 5 is not below 3: the step is negated.
        assert_eq!(
            calls_to(&e, PIPBOY_MOVE_KNOB),
            vec![vec![0x5000, 0, (-2.0f32).to_bits(), 2.0f32.to_bits(), 1]]
        );
        assert_eq!(e.mem.u32(KNOB_INDEX_011DA400), 3);
    }

    #[test]
    fn fn_00799100_takes_the_first_true_target() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let targets = node_chain(&mut e, &[0x801, 0x802]);
        returns(&mut e, OBJECTIVE_TARGETS_007D6BB0, targets);
        returns_each(&mut e, CONDITION_IS_TRUE, &[0, 1]);
        e.call(0x0079_9100, &args![0u32, 0x900u32]);
        assert_eq!(e.get(menu, MapMenu::pDisplayedQuestTarget), 0x802);
        // A target already shown: nothing is asked.
        e.call_log = Some(vec![]);
        e.call(0x0079_9100, &args![0u32, 0x900u32]);
        assert!(calls_to(&e, OBJECTIVE_FLAG_005A5DC0).is_empty());
    }

    /// The doubles the text scroll bar needs: the font manager's dimensions
    /// result and the scroll delta.
    fn scrollbar_doubles(e: &mut Engine) -> u32 {
        let dimensions = e.mem.alloc(8);
        e.mem.set_u32(dimensions + 4, 100.0f32.to_bits());
        returns(e, FONT_CALCULATE_STRING_DIMENSIONS, dimensions);
        returns_float(e, TILE_GET_FLOAT, 20.0);
        returns_float(e, FLOAT_PAIR_00404010, 80.0);
        returns_float(e, ROUND_FLOAT, 4.0);
        e.set_global::<f64>(ONE_DOUBLE, 1.0);
        returns(e, TEXT_TO_TRAIT, 0x3333);
        returns(e, TILE_GET_CHILD_BY_NAME, 0x950);
        dimensions
    }

    #[test]
    fn load_challenge_formats_the_text_and_sets_the_scroll_bar() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        scrollbar_doubles(&mut e);
        let challenge = with_vtable(&mut e, 0x40, &[]);
        let description = with_vtable(&mut e, 0x10, &[(0x10, 0x0010_0010)]);
        let table = e.mem.u32(description);
        e.mem.set_u32(challenge + 0x24, table);
        returns(&mut e, 0x0010_0010, 0x4567);
        returns(&mut e, MARKER_GET_LOCATION_NAME, 0x4568);
        e.call_log = Some(vec![]);
        e.call(0x0079_91b0, &args![menu, challenge]);
        let formatted = calls_to(&e, BUFFER_FORMAT);
        assert_eq!(formatted.len(), 1);
        assert_eq!(&formatted[0][1..4], &[0x200, CHALLENGE_TEXT_FORMAT, 0x4568]);
        assert_eq!(formatted[0][6], 0x4567);
        assert_eq!(
            calls_to(&e, 0x0010_0010),
            vec![vec![challenge + 0x24, 0, 0x4353_4544]]
        );
        let floats = calls_to(&e, TILE_SET_FLOAT);
        assert_eq!(floats[0], vec![0x950, 0x3333, 5.0f32.to_bits(), 1]);
        assert_eq!(floats[1], vec![0x950, 0x3333, 0, 0]);
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x950, 0x3333, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn load_note_shows_each_kind() {
        // Text.
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        scrollbar_doubles(&mut e);
        returns(&mut e, NOTE_TYPE, 1);
        let text = with_vtable(&mut e, 0x10, &[(0x10, 0x0010_0010)]);
        returns(&mut e, NOTE_TEXT_OBJECT_005E91D0, text);
        returns(&mut e, 0x0010_0010, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0079_93d0, &args![menu, 0x900u32]);
        assert!(calls_to(&e, TILE_SET_STRING).contains(&vec![0x950, 0xfc4, 0x6000, 1]));
        assert_eq!(calls_to(&e, 0x0010_0010)[0], vec![text, 0x900, 0x4d41_4e54]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT).last().unwrap(),
            &vec![0x10d, 0xfa3, 1]
        );
        // Audio without a sound: the empty time line.
        returns(&mut e, NOTE_TYPE, 3);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x6100);
        e.call_log = Some(vec![]);
        e.call(0x0079_93d0, &args![menu, 0x900u32]);
        let formatted = calls_to(&e, BUFFER_FORMAT);
        assert_eq!(&formatted[0][1..4], &[0x40, AUDIO_TIME_FORMAT, 0x6100]);
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x10d, 0x3333, 3]));
        // Image.
        returns(&mut e, NOTE_TYPE, 2);
        returns(&mut e, MARKER_GET_LOCATION_NAME, 0x6200);
        e.call_log = Some(vec![]);
        e.call(0x0079_93d0, &args![menu, 0x900u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x950, 0xfcc, 0x6200, 1]]
        );
        // Another kind: only tile 13 shown.
        returns(&mut e, NOTE_TYPE, 7);
        e.call_log = Some(vec![]);
        e.call(0x0079_93d0, &args![menu, 0x900u32]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x10d, 0xfa3, 1]]);
    }

    #[test]
    fn do_leave_enables_and_disables_the_lists() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        returns(&mut e, ADD_USER_TRAIT, 0x1212);
        for list in [0x130u32, 0x160, 0x1c0] {
            e.mem.set_u32(menu.addr() + list + 0xc, 0x900 + list);
        }
        e.call_log = Some(vec![]);
        e.call(0x0079_9690, &args![menu, 15i32, 0u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0xa30, 0x1212, 1], vec![0xac0, 0x1212, 0]]
        );
        e.set(menu, MapMenu::uiNoteEnterStartTime, 9);
        e.call(0x0079_9690, &args![menu, 0x18i32, 0u32]);
        assert_eq!(e.get(menu, MapMenu::uiNoteEnterStartTime), 0);
    }

    /// The map menu on top with the pip-boy up.
    fn gamepad_ready(e: &mut Engine) {
        returns(e, IS_TOP_MENU_ID, 1);
        returns(e, PIPBOY_FLAG_00974D90, 1);
    }

    #[test]
    fn do_gamepad_needs_the_menu_on_top() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        assert!(!e.call(0x0079_9790, &args![menu, 3i32, 0u32]).bool());
        returns(&mut e, IS_TOP_MENU_ID, 1);
        assert!(!e.call(0x0079_9790, &args![menu, 3i32, 0u32]).bool());
    }

    #[test]
    fn do_gamepad_steps_the_page_and_wraps() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        gamepad_ready(&mut e);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0xc, 0x0010_000c);
        e.mem.set_u32(menu.addr(), table);
        e.register(0x0010_000c, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.set(menu, MapMenu::iCurrentPage, 0x24);
        assert!(e.call(0x0079_9790, &args![menu, 3i32, 0u32]).bool());
        assert_eq!(e.get(menu, MapMenu::iCurrentPage), 0x20);
        assert!(e.call(0x0079_9790, &args![menu, 4i32, 0u32]).bool());
        assert_eq!(e.get(menu, MapMenu::iCurrentPage), 0x24);
        assert_eq!(
            calls_to(&e, 0x0010_000c),
            vec![vec![menu.addr(), 0x20, 0], vec![menu.addr(), 0x24, 0]]
        );
        // 0xD opens the inventory, 0xE the stats.
        assert!(e.call(0x0079_9790, &args![menu, 0xdi32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, SET_INVENTORY_MENU_VISIBLE),
            vec![vec![1, 0, 1]]
        );
        assert!(e.call(0x0079_9790, &args![menu, 0xei32, 0u32]).bool());
        assert_eq!(calls_to(&e, SET_STATS_MENU_VISIBLE), vec![vec![1, 0]]);
        // An unknown button does nothing.
        assert!(!e.call(0x0079_9790, &args![menu, 0x20i32, 0u32]).bool());
    }

    #[test]
    fn do_gamepad_pans_the_map_with_the_sticks() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        gamepad_ready(&mut e);
        returns(&mut e, FLAG_00709D40, 1);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set(menu, MapMenu::pMapTile, 0x104);
        e.set_global::<f64>(ZOOM_DEAD_ZONE_DOUBLE, 7849.0);
        e.set_global::<f64>(PAN_DEAD_ZONE_DOUBLE, 8689.0);
        e.set_global::<f64>(AXIS_RANGE_DOUBLE, 32767.0);
        e.set_global::<f64>(PAN_SPEED_Y_DOUBLE, -20.0);
        e.set_global::<f64>(PAN_SPEED_X_DOUBLE, 20.0);
        // Axis 8 still, 9 and 10 moved.
        e.register(INPUT_AXIS_00A23390, |_, a| Ret {
            eax: if a[2] == 8 { 0 } else { 32767 },
            ..Ret::default()
        });
        e.register(MAGNITUDE_00408820, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).abs(),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0079_9790, &args![menu, 0i32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![
                vec![0x104, 0xfee, (-20.0f32).to_bits()],
                vec![0x104, 0xfef, 20.0f32.to_bits()],
            ]
        );
        // The position is shown (`00799dc0` reads the map tile).
        assert!(!calls_to(&e, TILE_GET_FLOAT).is_empty());
    }

    #[test]
    fn show_position_highlights_the_nearest_marker() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::pMapTile, 0x104);
        e.set_global::<f64>(TWO_DOUBLE, 2.0);
        e.set_global::<f64>(MARKER_TILE_KIND_DOUBLE, 26.0);
        e.mem.set_u32(LARGEST_VALUE_01016970, f32::MAX.to_bits());
        e.mem.set_u32(TRAIT_X, 0x2001);
        e.mem.set_u32(TRAIT_Y, 0x2002);
        // Tile 6 (the cursor) is at 10 + 4/2 on both axes; the map at 0.
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: match (a[0], a[1]) {
                (0x106, 0xfa1 | 0xfa2) => 10.0,
                (0x106, 0xfb0 | 0xfb1) => 4.0,
                (0x104, _) => 0.0,
                (_, 0xfaa) => 26.0,
                _ => 0.0,
            },
            ..Ret::default()
        });
        let marker = e.mem.alloc(4);
        e.mem.set_u32(marker, 0x777);
        returns_each(&mut e, LIST_FIRST_00559450, &[1]);
        // One child: the walk ends when the node goes back to 0.
        e.register_double(LIST_STEP_0057CBE0, move |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret {
                eax: marker,
                ..Ret::default()
            }
        });
        // The marker is 1.0 away (under half of tile 6's height, 2.0).
        returns_float(&mut e, POINT2_LENGTH_00589850, 1.0);
        let made = e.mem.alloc(8);
        returns(&mut e, POINT2_SUBTRACT_004E8880, made);
        returns(&mut e, ADD_USER_TRAIT, 0x4444);
        returns(&mut e, TILE_GET_STRING, 0x5555);
        returns(&mut e, STRICMP, 1);
        e.mem.set_u32(TRAIT_SELECTED, 0x3333);
        e.call_log = Some(vec![]);
        e.call(0x0079_9dc0, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::pHighlightedMapMarker), 0x777);
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints[0], vec![0x777, 0x3333, 0]);
        assert_eq!(ints[1], vec![0x777, 0x3333, 1]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x106, 0x4444, 0x5555, 1]]
        );
        let set = calls_to(&e, POINT2_SET_00452DC0);
        assert_eq!(set[0][1..], [12.0f32.to_bits(), 12.0f32.to_bits()]);
    }

    #[test]
    fn show_position_clears_the_title_without_a_marker() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::pMapTile, 0x104);
        e.set(menu, MapMenu::pHighlightedMapMarker, 0x777);
        returns(&mut e, ADD_USER_TRAIT, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0079_9dc0, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::pHighlightedMapMarker), 0);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x106, 0x4444, EMPTY_TEXT, 1]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x10b, 0xfaf, 0]]);
    }

    #[test]
    fn update_playing_sounds_starts_the_loaded_sounds() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::bSoundsLoading, 1);
        // One sound node (the list head at +0x98), valid with 2000 ms.
        returns(&mut e, SOUND_IS_VALID, 1);
        returns(&mut e, SOUND_GET_DURATION, 2000);
        returns(&mut e, WORD_AT_14, 77);
        let player = with_vtable(&mut e, 0x10, &[(0x1f4, 0x0010_01f4)]);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let position = e.mem.alloc(0xc);
        returns(&mut e, 0x0010_01f4, position);
        e.call_log = Some(vec![]);
        e.call(0x0079_a660, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::bSoundsLoading), 0);
        assert_eq!(e.get(menu, MapMenu::pCurrentSound), menu.addr() + 0x98);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xc0)), 2000.0);
        assert_eq!(e.get(menu, MapMenu::iSoundPlayStartTime), 77);
        assert_eq!(calls_to(&e, SOUND_PLAY), vec![vec![menu.addr() + 0x98, 0]]);
        let subtitle = calls_to(&e, SHOW_SUBTITLE);
        assert_eq!(subtitle.len(), 1);
        assert_eq!(subtitle[0][7..], [player, 0]);
    }

    #[test]
    fn update_playing_sounds_shows_the_time_and_ends() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.mem.set_u32(menu.addr() + 0xc0, 2000.0f32.to_bits());
        e.set(menu, MapMenu::iSoundPlayStartTime, 1000);
        returns(&mut e, WORD_AT_14, 1500);
        returns_float(&mut e, FLOAT_PAIR_0040EBD0, 500.0);
        e.register(FTOL2, |_, a| Ret {
            eax: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32,
            ..Ret::default()
        });
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x6100);
        let node = e.mem.alloc(0x10);
        e.set(menu, MapMenu::pCurrentSound, node);
        e.call_log = Some(vec![]);
        e.call(0x0079_a660, &args![menu]);
        let formatted = calls_to(&e, BUFFER_FORMAT);
        // 1500 ms left: 0 minutes, 1 second, 5 tenths.
        assert_eq!(formatted[0][2..], [PLAY_TIME_FORMAT, 0, 1, 5, 0x6100]);
        // The current sound has ended (not valid) and there is no next one:
        // the time reads zero and the note stops.
        assert_eq!(formatted[1][2..], [PLAY_DONE_FORMAT, 0x6100]);
        assert_eq!(e.get(menu, MapMenu::pCurrentSound), 0);
    }

    #[test]
    fn update_current_page_runs_the_page_update() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::iCurrentPage, 0x22);
        // The quest page: both lists are emptied through their vtables.
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, 0x0010_001c);
        e.mem.set_u32(table + 0x20, 0x0010_0020);
        for list in [0x130u32, 0x1c0] {
            e.mem.set_u32(menu.addr() + list, table);
        }
        e.register(0x0010_001c, |_, _| Ret::default());
        e.register(0x0010_0020, |_, _| Ret::default());
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        e.call_log = Some(vec![]);
        e.call(0x0079_af30, &args![menu]);
        let ints = calls_to(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![0x10b, 0xfaf, 1]));
        assert!(ints.contains(&vec![0x10c, 0xfa3, 1]));
        assert!(ints.contains(&vec![0x112, 0xfa3, 0]));
        assert!(ints.contains(&vec![0x115, 0xfa3, 0]));
        assert_eq!(calls_to(&e, 0x0010_001c).len(), 2);
    }

    #[test]
    fn update_current_page_takes_the_world_map_tile() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        // The world map update stops without a root space map.
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(MARKER_OWNER_011C3F2C, owner);
        let head = list_box_with(&mut e, &[]).addr() + 4;
        returns(&mut e, PLUS_10, head);
        e.call(0x0079_af30, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::pMapTile), 0x104);
        assert_eq!(e.get(menu, MapMenu::bRootSpaceChanged), 1);
    }

    #[test]
    fn update_notes_inserts_the_players_notes() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, 0x0010_001c);
        e.mem.set_u32(menu.addr() + 0x160, table);
        e.register(0x0010_001c, |_, _| Ret::default());
        e.set(menu, MapMenu::bShowingAllNotes, 1);
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        returns(&mut e, PLAYER_NOTE_COUNT_00966A50, 2);
        let notes = [e.mem.alloc(0x80), e.mem.alloc(0x80)];
        e.mem.set_u8(notes[1] + 0x7d, 1);
        let mut next = 0usize;
        e.register_double(PLAYER_NOTE_AT_00966BB0, move |_, _| {
            next += 1;
            Ret {
                eax: notes[next - 1],
                ..Ret::default()
            }
        });
        e.mem.set_u32(menu.addr() + 0x160 + 0xc, 0x900);
        e.mem.set_u32(menu.addr() + 0x160 + 0x18, 0x0107_0000);
        returns(&mut e, MENU_RENDER_TEMPLATE, 0x700);
        let memory = e.mem.alloc(0x40);
        returns(&mut e, ALLOCATE, memory);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x2121);
        e.mem.set_u32(HALF_ALPHA_FLOAT, 128.0f32.to_bits());
        e.mem.set_u32(FULL_ALPHA_FLOAT, 255.0f32.to_bits());
        returns(&mut e, TEXT_TO_TRAIT, 0x3131);
        returns(&mut e, PLAYER_QUEST_005CBB50, 5);
        e.call_log = Some(vec![]);
        e.call(0x0079_b240, &args![menu]);
        let alphas: Vec<_> = calls_to(&e, TILE_SET_FLOAT)
            .into_iter()
            .filter(|c| c[1] == 0x2121)
            .map(|c| f32::from_bits(c[2]))
            .collect();
        assert_eq!(alphas, vec![255.0, 128.0]);
        let ints = calls_to(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![0x10c, 0x3131, 1]));
        assert!(ints.contains(&vec![0x10c, 0xfaf, 1]));
    }

    #[test]
    fn notes_filter_keeps_the_notes_of_the_active_quest() {
        let mut e = engine();
        assert_eq!(e.call(0x0079_b390, &args![0x900u32]).u8(), 1);
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::bShowingAllNotes, 1);
        assert_eq!(e.call(0x0079_b390, &args![0x900u32]).u8(), 0);
        e.set(menu, MapMenu::bShowingAllNotes, 0);
        let list = node_chain(&mut e, &[3, 5]);
        returns(&mut e, NOTE_LIST_004FADC0, list);
        returns(&mut e, PLAYER_QUEST_005CBB50, 5);
        assert_eq!(e.call(0x0079_b390, &args![0x900u32]).u8(), 0);
        returns(&mut e, PLAYER_QUEST_005CBB50, 6);
        assert_eq!(e.call(0x0079_b390, &args![0x900u32]).u8(), 1);
    }

    #[test]
    fn toggle_notes_challenge_mode_flips_and_sets_the_heading() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.mem.set_u32(menu.addr() + 0x1f0 + 0xc, 0xaf0);
        e.mem.set_u32(menu.addr() + 0x160 + 0xc, 0xa60);
        returns(&mut e, ADD_USER_TRAIT, 0x1212);
        returns(&mut e, OBJECT_FIELD_00403DF0, 0x6100);
        e.call_log = Some(vec![]);
        e.call(0x0079_b460, &args![menu, 0u8]);
        assert_eq!(e.mem.u8(CHALLENGE_MODE_011DA36D), 0);
        let ints = calls_to(&e, TILE_SET_INT);
        assert_eq!(ints[0], vec![0xaf0, 0x1212, 0]);
        assert_eq!(ints[1], vec![0xaf0, 0xfa3, 0]);
        assert_eq!(ints[2], vec![0xa60, 0x1212, 1]);
        assert_eq!(ints[3], vec![0xa60, 0xfa3, 1]);
        assert!(calls_to(&e, OBJECT_FIELD_00403DF0).contains(&vec![NOTES_HEADING_011D2A78]));
        // Toggling into challenge mode: the challenge type's name.
        e.mem.set_u32(CHALLENGE_TYPE_NAMES_011A03E4 + 4, 0x6600);
        e.mem.set_u8(CHALLENGE_TYPE_011DA36E, 1);
        e.mem.set_u32(menu.addr() + 0x80, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x0079_b460, &args![menu, 1u8]);
        assert_eq!(e.mem.u8(CHALLENGE_MODE_011DA36D), 1);
        let fields = calls_to(&e, OBJECT_FIELD_00403DF0);
        assert!(fields.contains(&vec![CHALLENGES_HEADING_011D2D18]));
        assert!(fields.contains(&vec![0x6600]));
    }

    #[test]
    fn challenge_list_is_filled_filtered_and_sorted() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, 0x0010_001c);
        e.mem.set_u32(table + 0x20, 0x0010_0020);
        e.mem.set_u32(menu.addr() + 0x1f0, table);
        e.register(0x0010_001c, |_, _| Ret::default());
        e.register(0x0010_0020, |_, _| Ret::default());
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(MARKER_OWNER_011C3F2C, owner);
        let challenges = node_chain(&mut e, &[0x801, 0x802]);
        returns(&mut e, CHALLENGE_LIST_00461150, challenges);
        // The first challenge is left out (005f5930 set, 005a60d0 clear).
        e.register(CHALLENGE_STATE_005F5930, |_, a| Ret {
            eax: u32::from(a[0] == 0x801),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0079_b600, &args![menu]);
        let names = calls_to(&e, MARKER_GET_LOCATION_NAME);
        assert_eq!(names, vec![vec![0x802 + 0x18]]);
        assert_eq!(
            calls_to(&e, 0x0010_0020),
            vec![vec![menu.addr() + 0x1f0, 0x0079_b820]]
        );
    }

    #[test]
    fn challenge_filter_keeps_the_displayed_type() {
        let mut e = engine();
        let challenge = e.mem.alloc(0x80);
        // Neither shown nor started: filtered out.
        assert_eq!(e.call(0x0079_b720, &args![challenge]).u8(), 1);
        returns(&mut e, CHALLENGE_VALUE_005E3FA0, 1);
        returns(&mut e, CHALLENGE_KIND_006A1B60, 3);
        e.mem.set_u8(CHALLENGE_TYPE_011DA36E, 1);
        assert_eq!(e.call(0x0079_b720, &args![challenge]).u8(), 0);
        e.mem.set_u8(CHALLENGE_TYPE_011DA36E, 2);
        assert_eq!(e.call(0x0079_b720, &args![challenge]).u8(), 1);
        returns(&mut e, CHALLENGE_KIND_006A1B60, 0xd);
        assert_eq!(e.call(0x0079_b720, &args![challenge]).u8(), 0);
    }

    #[test]
    fn sort_callbacks_order_by_alpha_then_name_or_index() {
        let mut e = engine();
        e.set_global::<f64>(HALF_ALPHA_DOUBLE, 128.0);
        e.set_global::<f64>(FULL_ALPHA_DOUBLE, 255.0);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x2121);
        let first = list_item(&mut e, 0x501, 0x801, 0);
        let second = list_item(&mut e, 0x502, 0x802, 0);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: match (a[0], a[1]) {
                (0x501, 0x2121) => 128.0,
                (0x502, 0x2121) => 255.0,
                (0x501, _) => 3.0,
                _ => 1.0,
            },
            ..Ret::default()
        });
        assert_eq!(e.call(0x0079_b820, &args![first, second]).i32(), 1);
        assert_eq!(e.call(0x0079_b820, &args![second, first]).i32(), -1);
        assert_eq!(e.call(0x0079_bc90, &args![first, second]).i32(), 1);
        assert_eq!(e.call(0x0079_bc90, &args![second, first]).i32(), -1);
        // Same alpha: by name / by index.
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0x2121 {
                255.0
            } else {
                f64::from(a[0] - 0x500)
            },
            ..Ret::default()
        });
        returns(&mut e, COMPARE_00408B20, 0xffff_ffff);
        assert_eq!(e.call(0x0079_b820, &args![first, second]).i32(), -1);
        assert_eq!(e.call(0x0079_bc90, &args![first, second]).i32(), -1);
        assert_eq!(e.call(0x0079_bc90, &args![first, first]).i32(), 0);
        // The radio sort compares the stations' names.
        e.call_log = Some(vec![]);
        returns(&mut e, STATION_NAME_0055D520, 0x4400);
        assert_eq!(e.call(0x0079_bd70, &args![first, second]).i32(), -1);
        assert_eq!(
            calls_to(&e, STATION_NAME_0055D520),
            vec![vec![0x802], vec![0x801]]
        );
    }

    #[test]
    fn update_quests_lists_the_shown_objectives() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, 0x0010_001c);
        e.mem.set_u32(table + 0x20, 0x0010_0020);
        for list in [0x130u32, 0x1c0] {
            e.mem.set_u32(menu.addr() + list, table);
            e.mem.set_u32(menu.addr() + list + 0xc, 0x900 + list);
            e.mem.set_u32(menu.addr() + list + 0x18, 0x0107_0000);
        }
        e.register(0x0010_001c, |_, _| Ret::default());
        e.register(0x0010_0020, |_, _| Ret::default());
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let objectives = node_chain(&mut e, &[0x801]);
        e.mem.set_u32(player + 0x6bc, e.mem.u32(objectives));
        e.mem.set_u32(player + 0x6c0, 0);
        returns(&mut e, OBJECTIVE_SHOWN_005A5E70, 1);
        returns(&mut e, PROCESS_VALUE_0044EDB0, 0x4400);
        let tile = e.mem.alloc(0x40);
        returns(&mut e, MENU_RENDER_TEMPLATE, tile);
        let memory = e.mem.alloc(0x40);
        returns(&mut e, ALLOCATE, memory);
        let child = e.mem.alloc(4);
        e.mem.set_u32(child, 0x710);
        returns(&mut e, CHILD_OF_006317A0, child);
        e.call_log = Some(vec![]);
        e.call(0x0079_b8e0, &args![menu]);
        // The objective's child gets its width; the quest is added once.
        assert!(calls_to(&e, TILE_SET_FLOAT)
            .iter()
            .any(|c| c[0] == 0x710 && c[1] == 0xfba));
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![tile, 0xfaa, 0x17]));
        assert_eq!(
            calls_to(&e, 0x0010_0020),
            vec![
                vec![menu.addr() + 0x1c0, 0x0079_bc90],
                vec![menu.addr() + 0x130, 0x0079_bc90],
            ]
        );
    }

    #[test]
    fn update_radio_list_inserts_the_stations() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, 0x0010_001c);
        e.mem.set_u32(table + 0x20, 0x0010_0020);
        e.mem.set_u32(menu.addr() + 0x190, table);
        e.mem.set_u32(menu.addr() + 0x190 + 0xc, 0xa90);
        e.mem.set_u32(menu.addr() + 0x190 + 0x18, 0x0107_0000);
        e.register(0x0010_001c, |_, _| Ret::default());
        e.register(0x0010_0020, |_, _| Ret::default());
        // One station at 011dd59c, the selected one.
        e.mem.set_u32(OBJECT_011DD59C, 0x801);
        e.mem.set_u32(OBJECT_011DD59C + 4, 0);
        let current = e.mem.alloc(4);
        e.mem.set_u32(current, 0x801);
        e.mem.set_u32(VALUE_011DD42C, current);
        returns(&mut e, MENU_RENDER_TEMPLATE, 0x700);
        let memory = e.mem.alloc(0x40);
        returns(&mut e, ALLOCATE, memory);
        e.mem.set_u32(TRAIT_SELECTED, 0x3333);
        returns(&mut e, LIST_CONTAINS_005F65D0, 1);
        e.mem.set_u32(TRAIT_TEXT_ALPHA, 0x2121);
        e.mem.set_u32(FULL_ALPHA_FLOAT, 255.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x0079_bea0, &args![menu]);
        let ints = calls_to(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![0x700, 0xfaa, 0x19]));
        assert!(ints.contains(&vec![0x700, 0x3333, 1]));
        assert!(calls_to(&e, TILE_SET_FLOAT).contains(&vec![0x700, 0x2121, 255.0f32.to_bits(), 1]));
        assert_eq!(
            calls_to(&e, 0x0010_0020),
            vec![vec![menu.addr() + 0x190, 0x0079_bd70]]
        );
        assert_eq!(calls_to(&e, LIST_DESTRUCT).len(), 1);
    }

    #[test]
    fn radio_waveform_starts_steps_and_picks_a_frame() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        returns(&mut e, TILE_OBJECT_0056C7F0, 0x40);
        returns(&mut e, OBJECT_NODE_0045BC00, 0x41);
        returns(&mut e, AV_OBJECT_GET_PROPERTY, 0x42);
        returns(&mut e, CLOCK_00457FE0, 1000);
        e.set_global::<f64>(WAVEFORM_STEP_DOUBLE, 0.5);
        e.set_global::<f64>(ONE_DOUBLE, 1.0);
        e.set_global::<f64>(WAVEFORM_FRAME_DOUBLE, 750.0);
        e.mem.set_u32(WAVEFORM_OFFSET_011DA414, 0.75f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x0079_c0c0, &args![menu]);
        // Started at 1000.
        assert_eq!(e.get(menu, MapMenu::fWaveformAnimationStartTime), 1000.0);
        // Stepped: 0.75 + 0.5 wraps to 0.25.
        assert_eq!(f32::from_bits(e.mem.u32(WAVEFORM_OFFSET_011DA414)), 0.25);
        assert_eq!(e.mem.u32(WAVEFORM_STEP_TIME_011DA418), 1000);
        assert_eq!(
            calls_to(&e, PROPERTY_OFFSET_0077A450),
            vec![vec![0x42, 0.75f32.to_bits(), 0]]
        );
        // 750 ms later a frame is picked (random 4: 4 mod 3 = 1, backward).
        returns(&mut e, CLOCK_00457FE0, 1750);
        returns(&mut e, RANDOM_00487F50, 4);
        e.call_log = Some(vec![]);
        e.call(0x0079_c0c0, &args![menu]);
        let copied = calls_to(&e, BUFFER_COPY_00406D30);
        assert_eq!(copied[0][1..], [0x40, WAVEFORM_BACKWARD_TEXT]);
        assert_eq!(e.get(menu, MapMenu::fWaveformAnimationStartTime), 0.0);
    }

    /// A map menu whose map rectangle runs from (0, 0) to (100, 200).
    fn menu_with_rectangle(e: &mut Engine) -> Ptr<MapMenu> {
        let menu = open_menu(e, 0x100);
        e.mem.set_u32(menu.addr() + 0xf0, 100.0f32.to_bits());
        e.mem.set_u32(menu.addr() + 0xf4, 200.0f32.to_bits());
        e.set_global::<f64>(WORLD_MAP_SCALE_DOUBLE, 0.5);
        e.set_global::<f64>(WORLD_MAP_OFFSET_DOUBLE, 0.25);
        e.set_global::<f64>(WORLD_MAP_INVERSE_SCALE_DOUBLE, 2.0);
        e.set_global::<f64>(WORLD_MAP_INVERSE_OFFSET_DOUBLE, 0.5);
        menu
    }

    #[test]
    fn world_to_map_coord_takes_the_fraction() {
        let mut e = engine();
        let menu = menu_with_rectangle(&mut e);
        let result = e.mem.alloc(0xc);
        let x = 50.0f32.to_bits();
        let y = 50.0f32.to_bits();
        let made = e
            .call(0x0079_c380, &args![menu, result, x, y, 0u32, 0x900u32])
            .u32();
        assert_eq!(made, result);
        assert_eq!(f32::from_bits(e.mem.u32(result)), 0.5);
        assert_eq!(f32::from_bits(e.mem.u32(result + 4)), 0.25);
        // On the world map the point is adjusted and scaled.
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.call_log = Some(vec![]);
        e.call(0x0079_c380, &args![menu, result, x, y, 0u32, 0x900u32]);
        let adjusted = calls_to(&e, ADJUST_MAP_MARKER_COORD);
        assert_eq!(adjusted.len(), 1);
        assert_eq!(adjusted[0][2], 0);
        assert_eq!(f32::from_bits(e.mem.u32(result)), 0.5);
        assert_eq!(f32::from_bits(e.mem.u32(result + 4)), 0.375);
    }

    #[test]
    fn map_to_world_coord_inverts_it() {
        let mut e = engine();
        let menu = menu_with_rectangle(&mut e);
        let result = e.mem.alloc(0xc);
        e.call(
            0x0079_c450,
            &args![
                menu,
                result,
                0.5f32.to_bits(),
                0.25f32.to_bits(),
                9u32,
                0x900u32
            ],
        );
        assert_eq!(f32::from_bits(e.mem.u32(result)), 50.0);
        assert_eq!(f32::from_bits(e.mem.u32(result + 4)), 50.0);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.call_log = Some(vec![]);
        e.call(
            0x0079_c450,
            &args![
                menu,
                result,
                0.5f32.to_bits(),
                0.375f32.to_bits(),
                9u32,
                0x900u32
            ],
        );
        assert_eq!(f32::from_bits(e.mem.u32(result)), 50.0);
        assert_eq!(f32::from_bits(e.mem.u32(result + 4)), 50.0);
        assert_eq!(calls_to(&e, ADJUST_MAP_MARKER_COORD)[0][2], 1);
    }

    #[test]
    fn do_wheel_move_zooms_on_the_maps() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        returns(&mut e, WHEEL_DELTA_0070EC70, 240);
        e.call_log = Some(vec![]);
        // Not a map page: nothing.
        e.set(menu, MapMenu::iCurrentPage, 0x22);
        e.call(0x0079_c530, &args![menu, 0u32, 0u32]);
        assert!(calls_to(&e, WHEEL_DELTA_0070EC70).is_empty());
        // On the world map without a map tile the zoom stops at once.
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.call(0x0079_c530, &args![menu, 0u32, 0u32]);
        assert_eq!(calls_to(&e, WHEEL_DELTA_0070EC70).len(), 1);
        assert!(calls_to(&e, TILE_GET_FLOAT).is_empty());
    }

    #[test]
    fn zoom_map_clamps_and_moves_the_knob() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::iCurrentPage, 0x21);
        e.set(menu, MapMenu::pMapTile, 0x104);
        e.set_global::<f64>(TWO_DOUBLE, 2.0);
        e.set_global::<f64>(ONE_DOUBLE, 1.0);
        e.set_global::<f64>(ZOOM_KNOB_DIVISOR_DOUBLE, 0.5);
        e.mem.set_u32(TRAIT_MAGNIFICATION, 0x7777);
        // Every setting reads 2.0; the current magnification is 1.0.
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 2.0f32.to_bits());
        returns(&mut e, OBJECT_VALUE_ADDRESS_00403E20, setting);
        returns_float(&mut e, TILE_GET_FLOAT, 1.0);
        returns_float(&mut e, FLOAT_PAIR_0040EBD0, 1.5);
        returns_float(&mut e, FLOAT_PAIR_00404010, 1.5);
        returns(&mut e, WORD_AT_174, 0x5000);
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        e.call_log = Some(vec![]);
        e.call(0x0079_c5a0, &args![menu, 1u8, 1.5f32]);
        assert_eq!(
            calls_to(&e, FLOAT_PAIR_0040EBD0),
            vec![vec![1.5f32.to_bits(), 2.0f32.to_bits()]]
        );
        let floats = calls_to(&e, TILE_SET_FLOAT);
        assert!(floats.contains(&vec![0x104, 0x7777, 1.5f32.to_bits(), 1]));
        // (1.5 - 1.0) / 0.5 = 1 step, negated when zooming in.
        assert_eq!(
            calls_to(&e, PIPBOY_MOVE_KNOB),
            vec![vec![0x5000, 0, (-2.0f32).to_bits(), 4.0f32.to_bits(), 1]]
        );
        // The companion markers are updated (the player's list read).
        assert!(!calls_to(&e, PLAYER_COMPANIONS_005EB7E0).is_empty());
    }

    #[test]
    fn update_world_map_sets_the_rectangle_from_the_map_data() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::pRootMapSpace, 0x3000);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0x800);
        e.mem.set_u32(data + 4, 0x400);
        e.mem.set_u16(data + 8, (-2i16) as u16);
        e.mem.set_u16(data + 0xa, 3);
        e.mem.set_u16(data + 0xc, 4);
        e.mem.set_u16(data + 0xe, (-1i16) as u16);
        returns(&mut e, WORLD_GET_WORLD_MAP_DATA, data);
        returns(&mut e, FORM_NAME_00586440, 0x6000);
        e.mem.set_u8(TEXTURE_CLEARED_011DA36C, 1);
        e.register(POINT2_SET_00452DC0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // Tile 4 already has its markers (child 0x1A).
        returns_each(&mut e, TILE_GET_CHILD_BY_ID, &[0x40, 0]);
        icon_player(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0079_cdb0, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::pMapSpace), 0x3000);
        assert_eq!(e.mem.u8(TEXTURE_CLEARED_011DA36C), 0);
        assert_eq!(
            calls_to(&e, TILE_SET_COUNT_00715C60),
            vec![vec![0x104, 0xfcd, 0x800], vec![0x104, 0xfce, 0x400]]
        );
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xe8)), -8192.0);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xec)), 16384.0);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xf0)), 20480.0);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xf4)), -4096.0);
        assert!(calls_to(&e, WORLD_BUILD_MAP_MARKER_LIST).is_empty());
    }

    #[test]
    fn update_local_map_sets_the_rectangle_around_the_player() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        icon_player(&mut e);
        let side = e.mem.alloc(4);
        e.mem.set_u32(side, 3);
        returns(&mut e, WORD_POINTER_0043D4D0, side);
        returns(&mut e, FLOAT_TO_INT_00406D90, 0x2400);
        e.register(POINT_SET_00416870, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(POINT2_SET_00452DC0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.set_global::<f64>(CELL_SIZE_DOUBLE, 4096.0);
        returns(&mut e, REFR_GET_WORLD_SPACE, 0x3000);
        e.call_log = Some(vec![]);
        e.call(0x0079_d410, &args![menu]);
        // Cell 2 minus half of 3: 1; the rectangle spans cells 1..=3.
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xe8)), 4096.0);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xec)), 16384.0);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xf0)), 16384.0);
        assert_eq!(f32::from_bits(e.mem.u32(menu.addr() + 0xf4)), 4096.0);
        assert_eq!(e.get(menu, MapMenu::pMapSpace), 0x3000);
        let ints = calls_to(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![0x102, 0xfa3, 1]));
        assert!(ints.contains(&vec![0x103, 0xfa3, 1]));
    }

    #[test]
    fn add_map_icons_places_the_player_icon() {
        let mut e = engine();
        let menu = menu_with_rectangle(&mut e);
        e.set(menu, MapMenu::pMapTile, 0x104);
        let player = with_vtable(&mut e, 0x800, &[(0x1f4, 0x0010_01f4), (0x2bc, 0x0010_02bc)]);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let position = e.mem.alloc(0xc);
        e.mem.set_u32(position, 25.0f32.to_bits());
        e.mem.set_u32(position + 4, 50.0f32.to_bits());
        returns(&mut e, 0x0010_01f4, position);
        returns_float(&mut e, 0x0010_02bc, 1.0);
        e.set_global::<f64>(HEADING_OFFSET_DOUBLE, 0.5);
        e.mem.set_u32(TRAIT_X, 0x2001);
        e.mem.set_u32(TRAIT_Y, 0x2002);
        e.call_log = Some(vec![]);
        e.call(0x0079_dbb0, &args![menu]);
        assert_eq!(e.get(menu, MapMenu::pMapPlayerRef), player);
        let floats = calls_to(&e, TILE_SET_FLOAT);
        assert!(floats.contains(&vec![0x103, 0x2001, 0.25f32.to_bits(), 1]));
        assert!(floats.contains(&vec![0x103, 0x2002, 0.25f32.to_bits(), 1]));
        assert!(floats.contains(&vec![0x103, 0xffa, (-1.5f32).to_bits(), 1]));
        assert_eq!(
            calls_to(&e, TILE_ADD_FADE_CONTROL)[0][1..],
            [
                0xff5,
                e.mem.u32(HALF_ALPHA_FLOAT),
                e.mem.u32(FULL_ALPHA_FLOAT),
                1.0f32.to_bits(),
                1
            ]
        );
    }

    #[test]
    fn add_quest_markers_marks_the_tracked_targets() {
        let mut e = engine();
        let menu = menu_with_rectangle(&mut e);
        e.set(menu, MapMenu::pMapTile, 0x104);
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let enabled = e.mem.alloc(4);
        e.mem.set_u8(enabled, 1);
        returns(&mut e, BYTE_OF_00408D60, enabled);
        let targets = node_chain(&mut e, &[0x801]);
        returns(&mut e, PLAYER_GET_CURRENT_TARGET_LIST, targets);
        let tracked = with_vtable(&mut e, 0x40, &[(0x1f4, 0x0010_01f4)]);
        returns(&mut e, TARGET_LOCAL_REFERENCE_005CBB70, tracked);
        let position = e.mem.alloc(0xc);
        returns(&mut e, 0x0010_01f4, position);
        returns(&mut e, MENU_RENDER_TEMPLATE, 0x700);
        returns(&mut e, TILE_GET_VALUE, 0x5050);
        e.call_log = Some(vec![]);
        e.call(0x0079_e0a0, &args![menu]);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE)[0][2],
            LOCAL_QUEST_MARKER_TEMPLATE
        );
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x700, 0xfaa, 0x1b]));
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![vec![0x5050, 0x7d0, 0x103, 0xff5]]
        );
    }

    /// A player with one companion (`0x801`'s node) that is its own
    /// tracked reference, with a position.
    fn companion_setup(e: &mut Engine) -> u32 {
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let companion = with_vtable(e, 0x40, &[(0x1f4, 0x0010_01f4), (0x22c, 0x0010_022c)]);
        let companions = node_chain(e, &[companion]);
        returns(e, PLAYER_COMPANIONS_005EB7E0, companions);
        let position = e.mem.alloc(0xc);
        returns(e, 0x0010_01f4, position);
        e.register(0x0010_022c, |_, _| Ret::default());
        e.set_global::<f64>(HALF_ALPHA_DOUBLE, 128.0);
        companion
    }

    #[test]
    fn update_companion_markers_updates_the_current_label() {
        let mut e = engine();
        let menu = menu_with_rectangle(&mut e);
        e.set(menu, MapMenu::pMapTile, 0x104);
        companion_setup(&mut e);
        returns(&mut e, TILE_GET_CHILD_BY_NAME, 0x720);
        returns(&mut e, TILE_GET_STRING, 0x6000);
        returns(&mut e, TILE_GET_VALUE, 0x5050);
        e.call_log = Some(vec![]);
        e.call(0x0079_e470, &args![menu]);
        let ints = calls_to(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![0x720, 0xfaa, 0x26]));
        assert!(ints.contains(&vec![0x720, 0xfae, 1]));
        assert_eq!(
            calls_to(&e, TILE_ADD_NEEDS_UPDATE),
            vec![vec![0x720, 0x3ff]]
        );
        // The label follows its marker (both values looked up first).
        assert_eq!(
            order_of(&e, &[TILE_GET_VALUE, VALUE_CLEAR_ACTIONS]),
            vec![
                TILE_GET_VALUE,
                TILE_GET_VALUE,
                VALUE_CLEAR_ACTIONS,
                VALUE_CLEAR_ACTIONS
            ]
        );
        assert_eq!(calls_to(&e, TELEPORT_PATH_DESTRUCT).len(), 1);
    }

    #[test]
    fn add_companion_markers_builds_marker_and_label() {
        let mut e = engine();
        let menu = menu_with_rectangle(&mut e);
        e.set(menu, MapMenu::pMapTile, 0x104);
        e.set(menu, MapMenu::iCurrentPage, 0x20);
        let companion = companion_setup(&mut e);
        // In the player's cell.
        returns(&mut e, MENU_RENDER_TEMPLATE, 0x700);
        let label = with_vtable(&mut e, 0x40, &[(4, 0x0010_0004)]);
        returns(&mut e, TILE_BUILD_TILE, label);
        e.register(0x0010_0004, |_, _| Ret::default());
        returns(&mut e, STATION_NAME_0055D520, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0079_ea30, &args![menu]);
        assert_eq!(calls_to(&e, TILE_BUILD_TILE), vec![vec![0x387]]);
        assert_eq!(
            calls_to(&e, 0x0010_0004),
            vec![vec![label, 0x104, 0x6000, 0]]
        );
        let ints = calls_to(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![0x700, 0xfaa, 0x1a]));
        assert!(ints.contains(&vec![label, 0xfaa, 0x26]));
        assert!(!ints.contains(&vec![label, 0xfae, 1]));
        assert!(calls_to(&e, TILE_SET_STRING).contains(&vec![
            0x700,
            0xfcc,
            COMPANION_ICON_PATH,
            1
        ]));
        let _ = companion;
    }

    #[test]
    fn player_marker_is_created_set_and_deleted() {
        let mut e = engine();
        let menu = menu_with_rectangle(&mut e);
        e.set(menu, MapMenu::pMapTile, 0x104);
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let marker = with_vtable(&mut e, 0x40, &[(0x1f4, 0x0010_01f4)]);
        let position = e.mem.alloc(0xc);
        e.mem.set_u32(position, 7.0f32.to_bits());
        returns(&mut e, 0x0010_01f4, position);
        returns(&mut e, PLAYER_MARKER_REFERENCE_0077A400, marker);
        returns(&mut e, MENU_RENDER_TEMPLATE, 0x700);
        e.call_log = Some(vec![]);
        e.call(0x0079_f360, &args![menu]);
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x700, 0xfaa, 0x1c]));
        assert!(calls_to(&e, TILE_SET_STRING).contains(&vec![
            0x700,
            0xfcc,
            PLAYER_MARKER_ICON_PATH,
            1
        ]));
        // SetPlayerMarker copies the player's marker position and space.
        e.mem.set_u32(player + 0x6f4, marker);
        returns(&mut e, PLAYER_CELL_008D6F30, 0x3300);
        e.call(0x0079_f640, &args![]);
        assert_eq!(e.get(menu, MapMenu::PlayerMarkerPos), 7.0);
        assert_eq!(e.get(menu, MapMenu::PlayerMarkerSpace), 0x3300);
        // DeletePlayerMarker removes child 0x1C once and every 0x1E.
        returns_each(&mut e, TILE_GET_CHILD_BY_ID, &[0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0079_f6f0, &args![menu]);
        assert_eq!(
            calls_to(&e, TILE_GET_CHILD_BY_ID),
            vec![vec![0x104, 0x1c], vec![0x104, 0x1e]]
        );
    }

    #[test]
    fn world_map_tracking_ref_takes_the_last_flagged_door() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        e.set(menu, MapMenu::pMapPlayerRef, 0x999);
        assert_eq!(e.call(0x0079_f7e0, &args![menu, 5u32, 0u32]).u32(), 0);
        returns(&mut e, ARRAY_SIZE_0044DDC0, 2);
        // Not in an interior: the reference itself.
        assert_eq!(e.call(0x0079_f7e0, &args![menu, 5u32, 0x40u32]).u32(), 5);
        returns(&mut e, REFR_GET_INTERIOR, 1);
        let flags = e.mem.alloc(2);
        e.mem.set_u8(flags, 1);
        e.register_double(PATH_SPACE_FLAG_006A1440, move |_, a| Ret {
            eax: flags + a[1],
            ..Ret::default()
        });
        let door = e.mem.alloc(4);
        e.mem.set_u32(door, 0x777);
        returns(&mut e, PATH_DOOR_00558DD0, door);
        // Index 1's flag is clear, index 0's set.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0079_f7e0, &args![menu, 5u32, 0x40u32]).u32(),
            0x777
        );
        assert_eq!(calls_to(&e, PATH_DOOR_00558DD0), vec![vec![0x50, 0]]);
        e.mem.set_u8(flags, 0);
        assert_eq!(
            e.call(0x0079_f7e0, &args![menu, 5u32, 0x40u32]).u32(),
            0x999
        );
    }

    #[test]
    fn create_waypoint_path_joins_the_waypoints() {
        let mut e = engine();
        point2_double(&mut e);
        let menu = menu_with_rectangle(&mut e);
        e.set(menu, MapMenu::pMapTile, 0x104);
        let from = with_vtable(&mut e, 0x40, &[(0x1f4, 0x0010_01f4)]);
        let to = with_vtable(&mut e, 0x40, &[(0x1f4, 0x0010_01f4)]);
        let position = e.mem.alloc(0xc);
        returns(&mut e, 0x0010_01f4, position);
        // No doors: the two ends are joined by one line.
        let tiles = [e.mem.alloc(4), e.mem.alloc(4)];
        let mut next = 0usize;
        e.register_double(MENU_RENDER_TEMPLATE, move |_, _| {
            next += 1;
            Ret {
                eax: if next <= 2 { tiles[next - 1] } else { 0x790 },
                ..Ret::default()
            }
        });
        // The local list: appending links real nodes.
        e.register(LIST_ADD_TAIL_00905820, |e, a| {
            let tile = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) != 0 {
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                let fresh = e.mem.alloc(8);
                e.mem.set_u32(node + 4, fresh);
                node = fresh;
            }
            e.mem.set_u32(node, tile);
            Ret::default()
        });
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        returns_float(&mut e, FLOAT_ROOT_004579E0, 5.0);
        returns_float(&mut e, ATAN2, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0079_f8d0, &args![menu, from, to, 0x40u32, 0x1du32]);
        let rendered = calls_to(&e, MENU_RENDER_TEMPLATE);
        assert_eq!(rendered.len(), 3);
        assert_eq!(rendered[2][1..3], [tiles[0], PATH_LINE_TEMPLATE]);
        let floats = calls_to(&e, TILE_SET_FLOAT);
        assert!(floats.contains(&vec![0x790, 0x1004, 5.0f32.to_bits(), 1]));
        assert!(floats.contains(&vec![0x790, 0xffa, (-0.5f32).to_bits(), 1]));
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x790, 0xfaa, 0x1d]));
        // Missing ends: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0079_f8d0, &args![menu, from, 0u32, 0x40u32, 0x1du32]);
        assert!(calls_to(&e, LIST_CONSTRUCT).is_empty());
    }

    #[test]
    fn local_map_step_builds_the_cell_geometry() {
        let mut e = engine();
        let menu = open_menu(&mut e, 0x100);
        let tile_2 = e.mem.alloc(0x40);
        e.mem.set_u32(menu.addr() + 0x30, tile_2);
        returns(&mut e, TILE_OBJECT_0056C7F0, 0x40);
        let side = e.mem.alloc(4);
        e.mem.set_u32(side, 3);
        returns(&mut e, WORD_POINTER_0043D4D0, side);
        let player = with_vtable(&mut e, 0x800, &[(0x1f4, 0x0010_01f4)]);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let position = e.mem.alloc(0xc);
        returns(&mut e, 0x0010_01f4, position);
        // A grid with room for four cells: this is cell 4 (column 1, row 1).
        e.set(menu, MapMenu::uiNextLocalMapGrid, 4);
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(ALLOCATE_00AA1070, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(POINT_SET_00416870, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(POINT2_SET_00452DC0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(COLOR_SET_00414430, |e, a| {
            for word in 0..4 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.mem.set_u32(FOG_ALPHA_FLOAT, 4.0f32.to_bits());
        e.set_global::<f64>(FOG_DIVISOR_DOUBLE, 4.0);
        e.set_global::<f64>(TWO_DOUBLE, 2.0);
        returns(&mut e, ALLOCATE_00AA13E0, 0x7700);
        returns(&mut e, SCISSOR_TRI_SHAPE_CONSTRUCT, 0x7800);
        returns(&mut e, TILE_SHADER_PROPERTY_CONSTRUCT, 0x7900);
        e.map(0x7900, 0x100);
        let node = with_vtable(&mut e, 0x10, &[(0xdc, 0x0010_00dc)]);
        returns(&mut e, TILE_OBJECT_0056C7F0, node);
        e.register(0x0010_00dc, |_, _| Ret::default());
        e.mem.set_u8(RENDER_FLAG_011FF103, 7);
        e.call_log = Some(vec![]);
        e.call(0x0079_ffb0, &args![0u8]);
        let shapes = calls_to(&e, SCISSOR_TRI_SHAPE_CONSTRUCT);
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0][1], 0x121);
        assert_eq!(shapes[0][8], 0x200);
        // The first index triangle of quad (0, 0) (even): 18, 17, 0.
        let indices = shapes[0][9];
        assert_eq!(e.mem.u16(indices), 18);
        assert_eq!(e.mem.u16(indices + 2), 17);
        assert_eq!(e.mem.u16(indices + 4), 0);
        // The vertex colour alpha: 4.0 / 4.0.
        let colors = shapes[0][4];
        assert_eq!(f32::from_bits(e.mem.u32(colors + 0xc)), 1.0);
        assert_eq!(calls_to(&e, 0x0010_00dc), vec![vec![node, 0x7800, 1]]);
        assert_eq!(e.get(menu, MapMenu::uiNextLocalMapGrid), 5);
        assert_eq!(e.mem.u8(RENDER_FLAG_011FF103), 7);
        // Placed at column 1, row 1 of the 0x400 cells, minus the span.
        let placed = calls_to(&e, NODE_SET_TRANSLATE_004BC1F0);
        assert_eq!(placed[0][1], 1536.0f32.to_bits());
        assert_eq!(placed[0][3], (-1536.0f32).to_bits());
    }

    #[test]
    fn sound_list_add_tail_fills_or_links() {
        let mut e = engine();
        let head = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x007a_19a0, &args![head, 0x900u32]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_ASSIGN), vec![vec![head, 0x900]]);
        returns(&mut e, SOUND_NODE_EMPTY_0076A3B0, 1);
        let fresh = e.mem.alloc(0x10);
        returns(&mut e, ALLOCATE, fresh);
        returns(&mut e, SOUND_NODE_CONSTRUCT_0076B900, fresh);
        e.call(0x007a_19a0, &args![head, 0x901u32]);
        assert_eq!(e.mem.u32(head + 0xc), fresh);
        // A handle `0076b780` rejects is not added.
        returns(&mut e, SOUND_HANDLE_FLAG_0076B780, 1);
        e.call_log = Some(vec![]);
        e.call(0x007a_19a0, &args![head, 0x902u32]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_FLAG_0076B780).len(), 1);
        assert!(calls_to(&e, SOUND_NODE_EMPTY_0076A3B0).is_empty());
    }

    #[test]
    fn simple_array_remove_moves_the_rest_down() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(0x40);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, 4);
        e.call_log = Some(vec![]);
        e.call(0x007a_1b10, &args![array, 1u32, 0u8]);
        assert_eq!(calls_to(&e, STRING_DESTRUCT), vec![vec![buffer + 8]]);
        assert_eq!(
            calls_to(&e, ARRAY_MOVE_00692480),
            vec![vec![array.addr(), buffer + 8, buffer + 0x10, 2]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        // Shrinking: a new buffer, the old one freed.
        returns(&mut e, ARRAY_SHOULD_SHRINK_006F3170, 1);
        returns(&mut e, ARRAY_SHRUNK_SIZE_00869600, 2);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 4, 0x0010_0004);
        e.mem.set_u32(array.addr(), table);
        returns(&mut e, 0x0010_0004, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x007a_1b10, &args![array, 0u32, 1u8]);
        assert_eq!(
            calls_to(&e, ARRAY_MOVE_00692480),
            vec![
                vec![array.addr(), 0x9000, buffer, 0],
                vec![array.addr(), 0x9000, buffer + 8, 2],
            ]
        );
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x9000);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
    }

    /// A player with a position (vtable +0x1F4) and a heading (+0x2BC), the
    /// one `AddMapIcons` reads.
    fn icon_player(e: &mut Engine) -> u32 {
        let player = with_vtable(e, 0x800, &[(0x1f4, 0x0010_01f4), (0x2bc, 0x0010_02bc)]);
        e.mem.set_u32(PLAYER_011DEA3C, player);
        let position = e.mem.alloc(0xc);
        returns(e, 0x0010_01f4, position);
        e.register(0x0010_02bc, |_, _| Ret::default());
        player
    }

    /// `NiPoint2`'s set (`00452dc0`) writing its two words.
    fn point2_double(e: &mut Engine) {
        e.register(POINT2_SET_00452DC0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }
}
