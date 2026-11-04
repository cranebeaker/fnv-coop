//! `fallout/interface/menus/containermenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The container menu (`ContainerMenu`, class `0x3f0`): the player's items
//! (`lbItems`, +0x98) beside the container's (`lbContainer`, +0xc8), in one
//! of the `CONTAINER_MENU_MODE`s at +0x80 (the code handles 1 to 4; 2 is
//! pickpocketing, 3 the companion inventory, 4 the weapon mod list). The
//! singleton `ContainerMenu::pMe` is `011d93f8`, the selected item
//! (`pCurrItem`, an `ItemChange *`) `011d93fc`.
//!
//! The compiler's exception-unwinding frames (`FS:[0]`) and the `/GS`
//! cookie checks are not translated; the functions that have them carry a
//! `Translation incomplete` marker. Sound handles and lists the code keeps
//! on its stack are modelled as [`Engine::with_stack`] blocks.

#[allow(unused_imports)]
use crate::prelude::*;

/// `ContainerMenu::pMe` (Xbox PDB static): the menu singleton.
const CONTAINER_MENU: u32 = 0x011d_93f8;
/// `ContainerMenu::pCurrItem` (Xbox PDB static): the selected item.
const CURRENT_ITEM: u32 = 0x011d_93fc;
/// A byte static set when an item is entered and cleared when one is
/// clicked (its name is not confirmed; `DoIdle` refreshes the item card
/// while it is set).
const ITEM_ENTERED: u32 = 0x011a_002c;
/// An object whose byte at +0x4cd `0075b6e0` sets (the global at
/// `011d8a80`; its class is not confirmed).
const FLAG_HOLDER: u32 = 0x011d_8a80;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;
/// The menu class number.
const CONTAINER_MENU_CLASS: u32 = 0x3f0;
/// `ContainerMenu`'s vtable.
const CONTAINER_MENU_VTABLE: u32 = 0x0107_21ac;

/// `Menu::Menu`.
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The `Menu` destructor body.
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `ListBox<ItemChange *>::ListBox<ItemChange *>`.
const LIST_BOX_CONSTRUCTOR: u32 = 0x0073_0810;
/// The list box's destructor body.
const LIST_BOX_DESTRUCTOR: u32 = 0x0073_08e0;
/// A `BSSimpleList`'s constructor, clearing and destructor bodies.
const SIMPLE_LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const SIMPLE_LIST_CLEAR: u32 = 0x0047_0470;
const SIMPLE_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// A `BSSoundHandle`'s constructor, constructor from a word, assignment and
/// destructor bodies.
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
const SOUND_HANDLE_FROM_WORD: u32 = 0x0057_bd10;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// `BSSoundHandle::FadeOutAndRelease` (`this`, time).
const SOUND_HANDLE_FADE_OUT: u32 = 0x00ad_8da0;
/// `BSSoundHandle::Play` (`this`, flag).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `BSAudio::QInstance` (no arguments).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByName` (`this`, result, name, flags).
const AUDIO_SOUND_BY_NAME: u32 = 0x00ad_7550;
/// `BSAudio::GetSoundHandleByFilename` (`this`, result, file, flags, form).
const AUDIO_SOUND_BY_FILE: u32 = 0x00ad_7480;
/// `operator new` and `operator delete` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
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
/// `Menu::SetMenuTile` (`this`, tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// `Menu::StartFadeOut`.
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
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
/// `Tile::GetString` (`this`, trait).
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// `Tile::IsTrue` (`this`, trait).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetMaximumDepth` (no arguments; result in ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Tile::TextToTrait` (cdecl, one word).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// Finds a child tile by name (cdecl: tile, name).
const TILE_FIND_CHILD: u32 = 0x00a0_8b20;
/// The tutorial message shower (`this` = interface + 0x4d4; id, class, time).
const SHOW_TUTORIAL: u32 = 0x0071_8630;
/// `ListBox<unsigned int>::Initialize` (`this`, container tile, template).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// The word at +4 of an object (a menu's `pMenu` tile, a list node's next).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// The address of an object's first word (a list node's item).
const FIRST_WORD_ADDRESS: u32 = 0x0068_15c0;
/// The first word of an object.
const FIRST_WORD: u32 = 0x0055_9450;
/// The word at +0x28 of a tile (its parent).
const WORD_AT_0X28: u32 = 0x0045_cd60;
/// A setting's string value (`ECX` = the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// A setting's value address (`ECX` = the setting).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// `Interface::PlayMenuSound` (cdecl, one word).
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
/// Whether a gamepad is in use (no arguments).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
/// The form of a reference or of an item (`this` only).
const FORM_OF: u32 = 0x007a_f430;
/// The item's object (`this` only).
const ITEM_OBJECT: u32 = 0x0044_ddc0;
/// A form's type byte (`this` only).
const FORM_TYPE: u32 = 0x0040_1170;
/// The display name of a reference (`this` only).
const REFERENCE_NAME: u32 = 0x0055_d520;
/// `TESObjectREFR::GetOwner`.
const REFERENCE_OWNER: u32 = 0x0056_7790;
/// `Actor::StealAlarm` (`this`, reference, two words, value, owner).
const STEAL_ALARM: u32 = 0x008b_fa40;
/// A test on the global at `011f2250` (`this` only).
const STATE_TEST_011F2250: u32 = 0x0052_5430;
/// The actor's process (`this` only; the map's name for the folded code is
/// `MiddleHighProcess::GetSavedAcquireObject`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `TESCreature::InitDefaultWorn` (`this`, reference, three words).
const INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// The NPC's worn-item initialisation (`this`, reference, word, flag).
const NPC_INIT_WORN: u32 = 0x0060_6540;
/// Two actor state calls (`this`; `this`, flag).
const ACTOR_STATE_008A6970: u32 = 0x008a_6970;
const ACTOR_SET_STATE_008A6840: u32 = 0x008a_6840;
/// `DialogMenu::ServiceStart` and `ServiceEnd` (`ServiceEnd` cdecl, one word).
const DIALOG_SERVICE_START: u32 = 0x0076_3ff0;
const DIALOG_SERVICE_END: u32 = 0x0076_40a0;
/// `PlayerCharacter::UpdatePlayer3d`.
const UPDATE_PLAYER_3D: u32 = 0x0095_0030;
/// `HUDMainMenu::RefreshInfoForRef` (no arguments).
const HUD_REFRESH_INFO: u32 = 0x0077_8b10;
/// The sound forms of a container base (`this` only): open and close.
const CONTAINER_OPEN_SOUND: u32 = 0x008c_e390;
const CONTAINER_CLOSE_SOUND: u32 = 0x0051_6bf0;
/// The text `00511840` answers for a sound form (`this` only; it calls
/// the code the map names `MapMarkerData::GetLocationName`).
const SOUND_FILE_NAME: u32 = 0x0051_1840;
/// `Actor::GetCurrentWeapon`.
const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// The full name of a form's name component (`this` only).
const NAME_OF: u32 = 0x0040_8da0;
/// `TESForm::GetFormByEditorID` (cdecl, one word).
const FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
/// `__RTDynamicCast` (cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `ContainerMenu::PlayReactivityBark` (`this`, topic).
const PLAY_REACTIVITY_BARK: u32 = 0x0075_ec60;
/// `ContainerMenu::PickpocketCheck` and `GetStolenValue`.
const PICKPOCKET_CHECK: u32 = 0x0075_e0b0;
const GET_STOLEN_VALUE: u32 = 0x0075_e240;
/// The player's weapon mod list (`this` = player).
const PLAYER_MOD_LIST: u32 = 0x0075_cc70;
/// The address of an entry of the mod list (`this`, index).
const MOD_LIST_ENTRY: u32 = 0x006a_7ad0;
/// `TESContainer::IsGold` (cdecl, one word).
const IS_GOLD: u32 = 0x0048_1f10;
/// `TESWeightForm::GetFormWeight` (cdecl: form, flag; ST0).
const FORM_WEIGHT: u32 = 0x0048_ebc0;
/// A byte state of the player (`this` only).
const PLAYER_STATE_004D1360: u32 = 0x004d_1360;
/// `ListBox::SaveListState`, `RestoreListState`, `Filter`,
/// `NumUnfiltered` and `AddItem`, and the list's item count.
const LIST_SAVE_STATE: u32 = 0x0073_12e0;
const LIST_RESTORE_STATE: u32 = 0x0073_1360;
const LIST_FILTER: u32 = 0x0073_0bb0;
const LIST_NUM_UNFILTERED: u32 = 0x0071_ae60;
const LIST_ADD_ITEM: u32 = 0x0071_a320;
const LIST_COUNT: u32 = 0x0072_6c40;
/// `ListBox::RemoveItemGroup` (`this`, form).
const LIST_REMOVE_ITEM_GROUP: u32 = 0x0071_a200;
/// The list's fill from an inventory (`this`, reference, filter, four words).
const LIST_FILL: u32 = 0x0071_9ef0;
/// The list box's walk over its items with a callback.
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
/// `ListBox::GetTileFor` (`this`, address of the item).
const LIST_GET_TILE_FOR: u32 = 0x007a_22d0;
/// The list's selected item (`this` only).
const LIST_SELECTED_ITEM: u32 = 0x007a_1910;
/// The highlighted tile of a list (`this` only).
const LIST_HIGHLIGHTED: u32 = 0x007d_6e90;
/// `QuantityMenu::Create` (cdecl: count, callback, maximum).
const QUANTITY_MENU_CREATE: u32 = 0x007a_ba00;
/// `InterfaceManager::SetCurrentFocusTarget` (`this`, tile, trait, flag).
const SET_FOCUS_TARGET: u32 = 0x0071_5860;
/// `InventoryChanges::GetInventoryChanges` (cdecl, one word).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetItemGroup` (`this`, form, list).
const GET_ITEM_GROUP: u32 = 0x004d_4b00;
/// `InventoryChanges::GetObjectCount` (`this`, form).
const GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `ItemChange::GetFullName`, `GetWorn`, `GetIconOne`, `GetItemOwnership`.
const ITEM_FULL_NAME: u32 = 0x004b_e2d0;
const ITEM_GET_WORN: u32 = 0x004b_ddd0;
const ITEM_GET_ICON: u32 = 0x004b_e200;
const ITEM_GET_OWNERSHIP: u32 = 0x004b_d740;
/// `ItemChange::ItemChange` (`this`, form, count).
const ITEM_CHANGE_CONSTRUCTOR: u32 = 0x004b_c550;
/// `TESObjectWEAP::GetCurrentAmmo` (`this`, actor).
const WEAPON_CURRENT_AMMO: u32 = 0x0052_5980;
/// `ExtraDataList::ExtraDataList`, `AddDroppedItem`, `SetHealth`,
/// `GetCount`, `GetItemDropper`.
const EXTRA_DATA_LIST_CONSTRUCTOR: u32 = 0x0041_0360;
const EXTRA_ADD_DROPPED_ITEM: u32 = 0x0041_de40;
const EXTRA_SET_HEALTH: u32 = 0x0041_9970;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
const EXTRA_GET_ITEM_DROPPER: u32 = 0x0041_de00;
/// The list's add-at-head (`this`, address of the item).
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
/// The reference's dropped-weapon lookup (`005d43c0` then `0041df00`).
const REFERENCE_EXTRA: u32 = 0x005d_43c0;
const EXTRA_DROPPED_REFERENCE: u32 = 0x0041_df00;
/// A test on the dropped reference (`this` only).
const REFERENCE_TEST_00437B90: u32 = 0x0043_7b90;
/// The reference's health (`this` only; ST0).
const REFERENCE_HEALTH: u32 = 0x0056_8ad0;
/// `BSStringT` constructor, destructor and `Format` (cdecl).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
/// The tile of a list item (`this` = item + 4).
const LIST_ITEM_TILE: u32 = 0x007b_5370;
/// Two values of the player into two floats (`this`, two out addresses):
/// the first from its actor value `0x2e`, the second from `008a0c20`.
const PLAYER_WEIGHTS: u32 = 0x0093_abe0;
/// Rounds the first float to a multiple of the second (cdecl; ST0).
const ROUND_TO: u32 = 0x004b_d510;
/// The float rounded to an integer (cdecl, one float).
const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// The bounded `sprintf`, string copy and append (cdecl).
const PRINT_BOUNDED: u32 = 0x0040_6d00;
const COPY_BOUNDED: u32 = 0x0040_6d30;
const APPEND_BOUNDED: u32 = 0x0040_6d50;
/// `_strupr_s` behind a wrapper (cdecl: buffer, size).
const TO_UPPER: u32 = 0x0073_0490;
/// `strcmp` behind a wrapper (cdecl, two strings).
const COMPARE_TEXT: u32 = 0x0040_8b20;
/// `_mbscmp` behind a wrapper (cdecl, two strings).
const COMPARE_STRINGS: u32 = 0x0046_9880;
/// `Interface::PopulateItemStatsDisplay` (cdecl: item, tile).
const POPULATE_ITEM_STATS: u32 = 0x0070_7e30;
/// Sets a tile's text from a C string (`this`, text).
const TILE_SET_TEXT: u32 = 0x0070_7e10;
/// The tile of a list item for `InitFunc` (`this` = item + 4).
const LIST_ITEM_TILE_INIT: u32 = 0x0073_ac90;
/// Fills the item card tiles (cdecl: three tiles, item, flag).
const SET_ITEM_CARD: u32 = 0x0071_9790;
/// Two float getters on an object (`this` only; ST0).
const OBJECT_VALUE_004BE180: u32 = 0x004b_e180;
const OBJECT_VALUE_004BE060: u32 = 0x004b_e060;
/// `GetTickCount` behind a wrapper (no arguments).
const CURRENT_TIME: u32 = 0x0045_7fe0;
/// The message box shower (cdecl, six words).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// The two float values the companion branch compares (`this`, flag;
/// `this`).
const ACTOR_VALUE_00577250: u32 = 0x0057_7250;
const ACTOR_VALUE_008A0C20: u32 = 0x008a_0c20;
/// The player's item equip call (`this`, form, two flags).
const PLAYER_EQUIP_STATE: u32 = 0x008a_ded0;
/// The deleting destructor of an `ItemChange` (`this`, flags).
const ITEM_CHANGE_DELETE: u32 = 0x0044_59e0;
/// `PlayerCharacter::RemoveRockItAmmo` and `AddRockItAmmo`.
const REMOVE_ROCK_IT_AMMO: u32 = 0x0096_8be0;
const ADD_ROCK_IT_AMMO: u32 = 0x0096_87b0;
/// A projectile-related call of the weapon branch (cdecl, four words).
const WEAPON_TRANSFER_009BD5C0: u32 = 0x009b_d5c0;
/// The weapon's ammo of the actor (`this`, actor).
const WEAPON_AMMO_FOR: u32 = 0x0052_5a90;
/// The smaller of two integers (cdecl).
const MIN_OF: u32 = 0x004a_8f20;
/// Two tests on a form (`this` only).
const FORM_TEST_00440DA0: u32 = 0x0044_0da0;
const FORM_TEST_00440D80: u32 = 0x0044_0d80;
/// The checks of `0075d510`.
const PLAYER_TEST_004997B0: u32 = 0x0049_97b0;
const LIST_TEST_008256D0: u32 = 0x0082_56d0;
const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;
const OBJECT_TEST_004C0BF0: u32 = 0x004c_0bf0;
const AMMO_FLAGS: u32 = 0x004f_d4f0;
/// The checks of `FirstPassFilterFunc`.
const MOD_LIST_OF: u32 = 0x0047_4a40;
const LIST_FORM_INDEX: u32 = 0x0058_ff60;
const BIPED_MODEL_OF: u32 = 0x0048_0db0;
const BIPED_PLAYABLE: u32 = 0x0048_0d90;
const FORM_TEST_0048CEE0: u32 = 0x0048_cee0;
const WEAPON_TEST_0047BCF0: u32 = 0x0047_bcf0;
const AMMO_TEST_004C94D0: u32 = 0x004c_94d0;
/// The font manager (no arguments), a font of it (`this`, index) and
/// `FontManager::CalculateStringDimensions`.
const FONT_MANAGER: u32 = 0x005b_d5b0;
const FONT_MANAGER_FONT: u32 = 0x005b_d5c0;
const CALCULATE_STRING_DIMENSIONS: u32 = 0x00a1_b020;
/// `memset` (cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// `_ftol2_sse`: the value in ST0 truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;

/// The menu file name, the missing-tile message and the list template.
const MENU_FILE: u32 = 0x0107_2258;
const MISSING_TILE_MESSAGE: u32 = 0x0107_2200;
const ITEM_TEMPLATE_NAME: u32 = 0x0107_04d4;
/// The child tile name the menu clears and its empty text.
const SUBTITLE_TILE_NAME: u32 = 0x0107_21f4;
const EMPTY_TEXT: u32 = 0x0101_1584;
/// Sound names.
const CLOSE_SOUND: u32 = 0x0107_2278;
const OPEN_SOUND_NAME: u32 = 0x0107_229c;
const CLOSE_SOUND_NAME: u32 = 0x0107_2288;
const COMPANION_SOUND: u32 = 0x0107_22b0;
const TAKE_ALL_SOUND: u32 = 0x0107_22c0;
/// The editor ids of the companion barks and of the filter's form list.
const COMPANION_OPEN_TOPIC: u32 = 0x0107_1f80;
const COMPANION_FULL_TOPIC: u32 = 0x0107_22dc;
const FILTER_FORM_LIST: u32 = 0x0107_22f4;
/// The RTTI type descriptors the casts use.
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_TOPIC: u32 = 0x0118_4720;
const TYPE_FORM_BASE: u32 = 0x0118_3108;
const TYPE_PLAYABLE_FORM: u32 = 0x0118_3158;
/// Formats.
const ITEM_NAME_FORMAT: u32 = 0x0101_2058;
const WEIGHT_FORMAT: u32 = 0x0107_22d0;
const COUNT_FORMAT: u32 = 0x0106_eb00;
const ELLIPSIS: u32 = 0x0107_2308;
/// The `double` stacking types the depth is set for, and constants.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const DISABLED_VALUE: u32 = 0x0102_e430;
const MESSAGE_TIME: u32 = 0x0101_62c0;
const DROPPED_ALPHA: u32 = 0x0101_e704;
const TITLE_MARGIN: u32 = 0x0107_2310;
const TITLE_ICON_MARGIN: u32 = 0x0103_56b0;
const TITLE_FONT_SCALE: u32 = 0x0102_1928;
const LARGEST_FLOAT: u32 = 0x0101_6970;
/// The trait name the tab tiles' labels are set with.
const TAB_LABEL_TRAIT_NAME: u32 = 0x0106_ed20;
/// The callbacks the lists are walked, filtered and sorted with.
const FIRST_PASS_FILTER: u32 = 0x0075_cfc0;
const INIT_FUNC: u32 = 0x0075_d160;
const FILTER_MODE_FUNC: u32 = 0x0075_e650;
const COMPARE_FUNC: u32 = 0x0075_d130;
const TAKE_ALL_FUNC: u32 = 0x0075_da60;
const ITEM_SELECT_CALLBACK: u32 = 0x0075_dc80;
/// Tile traits.
const TRAIT_VISIBLE: i32 = 0xfa3;
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_STRING: i32 = 0xfc4;
const TRAIT_IMAGE: i32 = 0xfcc;
const TRAIT_0XFA9: i32 = 0xfa9;
const TRAIT_0XFB0: i32 = 0xfb0;

layout! {
    /// `ContainerMenu` (Xbox PDB), 0x10c bytes; the PC code uses the PDB's
    /// offsets.
    pub struct ContainerMenu: 0x10c {
        /// `pContainerRef` (Xbox PDB): `TESObjectREFR *`.
        0x74 pContainerRef: u32,
        /// `eContainerMode` (Xbox PDB).
        0x80 eContainerMode: i32,
        /// `iValueStolen` (Xbox PDB).
        0x84 iValueStolen: i32,
        /// `bPickPocketTallied` (Xbox PDB).
        0x88 bPickPocketTallied: u8,
        /// `bPickpocketFail` (Xbox PDB).
        0x89 bPickpocketFail: u8,
        /// `eItemsFilterMode` (Xbox PDB).
        0x8C eItemsFilterMode: i32,
        /// `eContainerFilterMode` (Xbox PDB).
        0x90 eContainerFilterMode: i32,
        /// `eFilterSound` (Xbox PDB).
        0x94 eFilterSound: i32,
        /// `plbCurrListbox` (Xbox PDB).
        0xF8 plbCurrListbox: u32,
        /// `m_nTimeToClearSubtitle` (Xbox PDB).
        0xFC m_nTimeToClearSubtitle: u32,
    }
}

/// Offsets of `ActiveMineList`, `lbItems`, `lbContainer` and
/// `m_hCurrentReactivityBark` (Xbox PDB).
const ACTIVE_MINE_LIST: u32 = 0x78;
const ITEMS: u32 = 0x98;
const CONTAINER: u32 = 0xc8;
const REACTIVITY_BARK: u32 = 0x100;

fn me(e: &Engine) -> u32 {
    e.global::<u32>(CONTAINER_MENU)
}

fn menu(e: &Engine) -> Ptr<ContainerMenu> {
    Ptr::new(me(e))
}

fn current_item(e: &Engine) -> u32 {
    e.global::<u32>(CURRENT_ITEM)
}

fn player_actor(e: &Engine) -> u32 {
    e.global::<u32>(PLAYER)
}

fn tile_at(e: &Engine, object: u32, offset: u32) -> u32 {
    e.mem.u32(object.wrapping_add(offset))
}

fn set_trait_from_int(e: &mut Engine, tile: u32, trait_id: i32, value: i32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
}

fn set_string(e: &mut Engine, tile: u32, trait_id: i32, text: u32) {
    e.call(SET_STRING_TRAIT, &args![tile, trait_id, text, true]);
}

/// The type byte of the object of an item.
fn item_type(e: &mut Engine, item: u32) -> u8 {
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    e.call(FORM_TYPE, &args![object]).u8()
}

/// Whether the reference is an actor (its slot `+0x100`).
fn is_actor(e: &mut Engine, reference: u32) -> bool {
    e.vcall(reference, 0x100, &args![]).bool()
}

/// Plays the named sound once: a sound handle the code keeps on its stack
/// (`BSAudio::GetSoundHandleByName` with flags `0x121`, `Play(0)`, then
/// the handle's destructor).
fn play_named_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let sound = e
            .call(AUDIO_SOUND_BY_NAME, &args![audio, handle, name, 0x121u32])
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
    });
}

/// Plays the reactivity bark of the topic found by editor id.
fn play_bark(e: &mut Engine, editor_id: u32) {
    let form = e.call(FORM_BY_EDITOR_ID, &args![editor_id]).u32();
    let topic = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0i32, TYPE_FORM, TYPE_TOPIC, 0i32],
        )
        .u32();
    let menu = me(e);
    e.call(PLAY_REACTIVITY_BARK, &args![menu, topic]);
}

// Translated from 0075b0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::ContainerMenu` (Xbox PDB name from the call graph):
/// constructs the `Menu` base, stores the class's vtable, constructs the
/// mine list, both item lists and the reactivity bark handle, clears
/// `pCurrItem`, the current list, the stolen value, the two pickpocket
/// flags and the filter modes, sets the filter sound to 3 and assigns a
/// fresh handle (`0057bd10(handle, 0)`) to the bark handle. Returns `this`.
// Translation incomplete: 0075b0d0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn container_menu_container_menu(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
) -> Ptr<ContainerMenu> {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), CONTAINER_MENU_VTABLE);
    e.call(
        SIMPLE_LIST_CONSTRUCTOR,
        &args![this.byte_add(ACTIVE_MINE_LIST)],
    );
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.byte_add(ITEMS)]);
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.byte_add(CONTAINER)]);
    e.call(
        SOUND_HANDLE_CONSTRUCTOR,
        &args![this.byte_add(REACTIVITY_BARK)],
    );
    e.set_global(CURRENT_ITEM, 0u32);
    e.set(this, ContainerMenu::plbCurrListbox, 0);
    e.set(this, ContainerMenu::iValueStolen, 0);
    e.set(this, ContainerMenu::bPickPocketTallied, 0);
    e.set(this, ContainerMenu::bPickpocketFail, 0);
    e.set(this, ContainerMenu::eContainerFilterMode, 0);
    e.set(this, ContainerMenu::eItemsFilterMode, 0);
    e.set(this, ContainerMenu::eFilterSound, 3);
    e.with_stack(0xc, |e, handle| {
        e.call(SOUND_HANDLE_FROM_WORD, &args![handle, 0u32]);
        e.call(
            SOUND_HANDLE_ASSIGN,
            &args![this.byte_add(REACTIVITY_BARK), handle],
        );
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
    });
    this
}

// Translated from 0075b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::GetClass` (Xbox PDB): the class number `0x3f0`.
pub fn container_menu_get_class(_e: &mut Engine, _this: Ptr<ContainerMenu>) -> u32 {
    CONTAINER_MENU_CLASS
}

// Translated from 0075b210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::AttachTileByID` (Xbox PDB): stores `tile` as
/// `pTiles[id]` when `0 <= id < 19`.
pub fn container_menu_attach_tile_by_id(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    id: i32,
    tile: u32,
) {
    if (0..0x13).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(0x28 + 4 * id as u32), tile);
    }
}

// Translated from 0075b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn container_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    flags: u32,
) -> Ptr<ContainerMenu> {
    fn_0075b270(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0075b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ContainerMenu` destructor: stores the class's vtable, clears
/// `pMe`, destroys the bark handle, both item lists and the mine list (in
/// that order) and runs the `Menu` destructor.
// Translation incomplete: 0075b270 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_0075b270(e: &mut Engine, this: Ptr<ContainerMenu>) {
    e.mem.set_u32(this.addr(), CONTAINER_MENU_VTABLE);
    e.set_global(CONTAINER_MENU, 0u32);
    e.call(
        SOUND_HANDLE_DESTRUCTOR,
        &args![this.byte_add(REACTIVITY_BARK)],
    );
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(CONTAINER)]);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(ITEMS)]);
    e.call(
        SIMPLE_LIST_DESTRUCTOR,
        &args![this.byte_add(ACTIVE_MINE_LIST)],
    );
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 0075b310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::Create` (Xbox PDB, cdecl): opens the menu on the
/// container reference in the mode. Clears the flag of `0075b6e0`; without
/// a reference only mode 4 is accepted (else 0). Deletes an open menu of
/// the class, reads the menu file (`01072258`; no menu without a root
/// tile) and, for a menu of this class, attaches the tile, checks all 19
/// tiles (else prints a message and answers 0), sets the depth for the two
/// stacking types, shows tutorial message `0x16`, sets up both item lists,
/// the three title tiles and clears the subtitle child tile, stores the
/// reference and the mode, runs `InitContainerMode`, labels the tab tiles
/// that exist, prepares the menu for showing and fills the lists. Returns
/// the root tile. A menu of another class is deleted (when it has a tile)
/// and the result is 0.
pub fn container_menu_create(e: &mut Engine, reference: u32, mode: i32) -> u32 {
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    fn_0075b6e0(e, 0);
    if reference == 0 && mode != 4 {
        return 0;
    }
    let class = fn_0075b700(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let found = if root != 0 {
        e.call(TILE_GET_MENU, &args![root]).u32()
    } else {
        0
    };
    e.set_global(CONTAINER_MENU, found);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_0075b700(e) {
            return finish_create(e, root, interface, reference, mode);
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let old = me(e);
        e.vcall(old, 0, &args![1u32]);
    }
    0
}

/// The part of [`container_menu_create`] after the menu of the class was
/// read.
fn finish_create(e: &mut Engine, root: u32, interface: u32, reference: u32, mode: i32) -> u32 {
    let this = me(e);
    e.call(MENU_SET_MENU_TILE, &args![this, root, 0u32]);
    if !fn_0075b710(e, menu(e)) {
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
    let class = fn_0075b700(e);
    e.call(
        SHOW_TUTORIAL,
        &args![interface.wrapping_add(0x4d4), 0x16u32, class, 0x200u32],
    );
    for (list, container) in [(ITEMS, 0x38u32), (CONTAINER, 0x48)] {
        let tile = tile_at(e, me(e), container);
        let list = me(e).wrapping_add(list);
        e.call(LIST_BOX_INITIALIZE, &args![list, tile, ITEM_TEMPLATE_NAME]);
    }
    for (offset, setting) in [
        (0x2cu32, 0x011d_4378u32),
        (0x50, 0x011d_2d0c),
        (0x54, 0x011d_33fc),
    ] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = tile_at(e, me(e), offset);
        set_string(e, tile, TRAIT_STRING, text);
    }
    let menu_tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    let subtitle = e
        .call(TILE_FIND_CHILD, &args![menu_tile, SUBTITLE_TILE_NAME])
        .u32();
    if subtitle != 0 {
        set_string(e, subtitle, TRAIT_STRING, EMPTY_TEXT);
    }
    e.set(menu(e), ContainerMenu::pContainerRef, reference);
    e.set(menu(e), ContainerMenu::eContainerMode, mode);
    container_menu_init_container_mode(e, menu(e));
    let label_trait = e
        .call(TILE_TEXT_TO_TRAIT, &args![TAB_LABEL_TRAIT_NAME])
        .i32();
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
            set_string(e, tile, label_trait, text);
        }
    }
    let this = me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![this, 0u32]);
    container_menu_update_lists(e, menu(e), 0);
    root
}

// Translated from 0075b6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0x4cd of the object at `011d8a80` (when there is one)
/// to `value` (cdecl).
pub fn fn_0075b6e0(e: &mut Engine, value: u8) {
    let holder = e.global::<u32>(FLAG_HOLDER);
    if holder != 0 {
        e.mem.set_u8(holder.wrapping_add(0x4cd), value);
    }
}

// Translated from 0075b700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x3f0`, without an object (no `this`).
pub fn fn_0075b700(_e: &mut Engine) -> u32 {
    CONTAINER_MENU_CLASS
}

// Translated from 0075b710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all 19 tiles of `pTiles` are set.
pub fn fn_0075b710(e: &mut Engine, this: Ptr<ContainerMenu>) -> bool {
    (0..0x13).all(|i| tile_at(e, this.addr(), 0x28 + 4 * i) != 0)
}

// Translated from 0075b750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::Close` (Xbox PDB, no arguments). With a menu open: fades
/// out the reactivity bark (100), then by mode: 1 plays the container's
/// close sound; 2 (pickpocketing) raises the steal alarm for the stolen
/// value when the reference is not an actor, then plays the close sound; 4
/// asks the player's slot `+0x340` with 9 unless the test on `011f2250`
/// passes; 3 (companion) re-initialises the companion's worn items (a
/// creature's default worn items, or the NPC's), resets a state of an actor
/// whose process answers its slot `+0x148` false, plays the close sound
/// `01072278` and ends the dialogue service. Then clears the mine list,
/// sets the menu tile's trait `0x1772` to 1, starts the fade out, updates
/// the player's 3D when the player's process asks for it (slot `+0x474`
/// with 1) and refreshes the HUD.
// Translation incomplete: 0075b750 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handle is not translated
pub fn container_menu_close(e: &mut Engine) {
    if me(e) == 0 {
        return;
    }
    let bark = me(e).wrapping_add(REACTIVITY_BARK);
    e.call(SOUND_HANDLE_FADE_OUT, &args![bark, 0x64u32]);
    match e.get(menu(e), ContainerMenu::eContainerMode) {
        1 => {
            let reference = e.get(menu(e), ContainerMenu::pContainerRef);
            container_menu_play_container_sound(e, menu(e), reference, 0);
        }
        2 => {
            let reference = e.get(menu(e), ContainerMenu::pContainerRef);
            let actor = if is_actor(e, reference) {
                e.get(menu(e), ContainerMenu::pContainerRef)
            } else {
                0
            };
            if actor == 0 && e.get(menu(e), ContainerMenu::iValueStolen) > 0 {
                let reference = e.get(menu(e), ContainerMenu::pContainerRef);
                let owner = e.call(REFERENCE_OWNER, &args![reference]).u32();
                let stolen = e.get(menu(e), ContainerMenu::iValueStolen);
                let reference = e.get(menu(e), ContainerMenu::pContainerRef);
                let player = player_actor(e);
                e.call(
                    STEAL_ALARM,
                    &args![player, reference, 0u32, 0u32, stolen, owner],
                );
            }
            let reference = e.get(menu(e), ContainerMenu::pContainerRef);
            container_menu_play_container_sound(e, menu(e), reference, 0);
        }
        4 => {
            if !e.call(STATE_TEST_011F2250, &args![0x011f_2250u32]).bool() {
                let player = player_actor(e);
                e.vcall(player, 0x340, &args![9u32]);
            }
        }
        3 => close_companion(e),
        _ => {}
    }
    let list = me(e).wrapping_add(ACTIVE_MINE_LIST);
    e.call(SIMPLE_LIST_CLEAR, &args![list]);
    let tile = tile_at(e, me(e), 4);
    set_trait_from_int(e, tile, 0x1772, 1);
    let this = me(e);
    e.call(MENU_START_FADE_OUT, &args![this]);
    let player = player_actor(e);
    let process = e.call(ACTOR_PROCESS, &args![player]).u32();
    if e.vcall(process, 0x474, &args![1u32]).bool() {
        let player = player_actor(e);
        e.call(UPDATE_PLAYER_3D, &args![player]);
    }
    e.call(HUD_REFRESH_INFO, &args![]);
}

/// Mode 3 of [`container_menu_close`].
fn close_companion(e: &mut Engine) {
    let mut reset_state = false;
    let reference = e.get(menu(e), ContainerMenu::pContainerRef);
    if is_actor(e, reference) {
        let reference = e.get(menu(e), ContainerMenu::pContainerRef);
        if e.vcall(reference, 0x1e4, &args![]).u32() != 0 {
            let reference = e.get(menu(e), ContainerMenu::pContainerRef);
            if e.call(ACTOR_PROCESS, &args![reference]).u32() != 0 {
                let reference = e.get(menu(e), ContainerMenu::pContainerRef);
                let process = e.call(ACTOR_PROCESS, &args![reference]).u32();
                if e.vcall(process, 0x148, &args![]).u32() != 0 {
                    reset_state = true;
                }
            }
        }
    }
    let reference = e.get(menu(e), ContainerMenu::pContainerRef);
    if e.vcall(reference, 0x21c, &args![]).bool() {
        let reference = e.get(menu(e), ContainerMenu::pContainerRef);
        let base = e.call(FORM_OF, &args![reference]).u32();
        let reference = e.get(menu(e), ContainerMenu::pContainerRef);
        e.call(INIT_DEFAULT_WORN, &args![base, reference, 1u32, 1u32, 1u32]);
    } else {
        let reference = e.get(menu(e), ContainerMenu::pContainerRef);
        let base = e.call(FORM_OF, &args![reference]).u32();
        let reference = e.get(menu(e), ContainerMenu::pContainerRef);
        let word = e.vcall(reference, 0x1e8, &args![]).u32();
        let reference = e.get(menu(e), ContainerMenu::pContainerRef);
        e.call(NPC_INIT_WORN, &args![base, reference, word, 1u32]);
    }
    if reset_state {
        let reference = e.get(menu(e), ContainerMenu::pContainerRef);
        if e.call(ACTOR_STATE_008A6970, &args![reference]).bool() {
            let reference = e.get(menu(e), ContainerMenu::pContainerRef);
            let process = e.call(ACTOR_PROCESS, &args![reference]).u32();
            if e.vcall(process, 0x148, &args![]).u32() == 0 {
                let reference = e.get(menu(e), ContainerMenu::pContainerRef);
                e.call(ACTOR_SET_STATE_008A6840, &args![reference, 0u32]);
            }
        }
    }
    play_named_sound(e, CLOSE_SOUND);
    e.call(DIALOG_SERVICE_END, &args![0u32]);
}

// Translated from 0075baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::PlayContainerSound` (Xbox PDB): for a container base
/// (form type `0x1b`) plays its open or close sound form by file name; for
/// form types `0x2a` and `0x2b` plays the named open (`0107229c`) or close
/// (`01072288`) sound. Nothing for a null reference.
// Translation incomplete: 0075baf0 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handles is not translated
pub fn container_menu_play_container_sound(
    e: &mut Engine,
    _this: Ptr<ContainerMenu>,
    reference: u32,
    opening: u8,
) {
    if reference == 0 {
        return;
    }
    let base = e.call(FORM_OF, &args![reference]).u32();
    let kind = e.call(FORM_TYPE, &args![base]).u32() as i32;
    let mut sound = 0u32;
    if kind == 0x1b {
        let base = e.call(FORM_OF, &args![reference]).u32();
        sound = if opening != 0 {
            e.call(CONTAINER_OPEN_SOUND, &args![base]).u32()
        } else {
            e.call(CONTAINER_CLOSE_SOUND, &args![base]).u32()
        };
    } else if kind > 0x29 && kind <= 0x2b {
        let name = if opening != 0 {
            OPEN_SOUND_NAME
        } else {
            CLOSE_SOUND_NAME
        };
        play_named_sound(e, name);
    }
    if sound != 0 {
        e.with_stack(0xc, |e, handle| {
            let file = e.call(SOUND_FILE_NAME, &args![sound]).u32();
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let played = e
                .call(
                    AUDIO_SOUND_BY_FILE,
                    &args![audio, handle, file, 0x121u32, sound],
                )
                .u32();
            e.call(SOUND_HANDLE_PLAY, &args![played, 0u32]);
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
        });
    }
}

// Translated from 0075bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::InitContainerMode` (Xbox PDB): modes 1 and 2 title the
/// container with the reference's name and play its open sound; mode 3
/// (companion) titles it, plays `010722b0`, starts the dialogue service and
/// plays the companion's open bark, then hides the tile at +0x50; mode 4
/// titles the list with a setting and, when the player has a weapon, the
/// container with the weapon's name, and hides five tiles.
// Translation incomplete: 0075bc80 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handle is not translated
pub fn container_menu_init_container_mode(e: &mut Engine, this: Ptr<ContainerMenu>) {
    match e.get(this, ContainerMenu::eContainerMode) {
        1 | 2 => {
            if e.get(this, ContainerMenu::pContainerRef) != 0 {
                let reference = e.get(this, ContainerMenu::pContainerRef);
                let name = e.call(REFERENCE_NAME, &args![reference]).u32();
                container_menu_set_container_title(e, this, name);
                let reference = e.get(this, ContainerMenu::pContainerRef);
                container_menu_play_container_sound(e, this, reference, 1);
            }
        }
        3 => {
            if e.get(this, ContainerMenu::pContainerRef) != 0 {
                let reference = e.get(this, ContainerMenu::pContainerRef);
                let name = e.call(REFERENCE_NAME, &args![reference]).u32();
                container_menu_set_container_title(e, this, name);
                play_named_sound(e, COMPANION_SOUND);
                e.call(DIALOG_SERVICE_START, &args![]);
                play_bark(e, COMPANION_OPEN_TOPIC);
            }
            let tile = tile_at(e, this.addr(), 0x50);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        }
        4 => {
            let text = e.call(SETTING_STRING, &args![0x011d_3c40u32]).u32();
            let tile = tile_at(e, this.addr(), 0x2c);
            set_string(e, tile, TRAIT_STRING, text);
            let player = player_actor(e);
            if e.call(ACTOR_GET_CURRENT_WEAPON, &args![player]).u32() != 0 {
                let player = player_actor(e);
                let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![player]).u32();
                let name = e.call(NAME_OF, &args![weapon.wrapping_add(0x30)]).u32();
                container_menu_set_container_title(e, this, name);
            }
            for offset in [0x50u32, 0x28, 0x30, 0x3c, 0x44] {
                let tile = tile_at(e, this.addr(), offset);
                set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
            }
        }
        _ => {}
    }
}

// Translated from 0075be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoClick` (Xbox PDB). A null tile is taken from
/// `pTiles[id]` for `0 <= id < 19` (the value is not used again). By id:
/// - 0, 1, 2: steps the items filter mode (0 back, else forward, wrapping
///   0..5) until the filtered list is not empty or the mode is 0, with one
///   filter sound;
/// - 5, 6, 7: the same for the container filter mode (5 back);
/// - 10: takes all (the container list walked with `0075da60`), plays
///   `010722c0` and closes;
/// - 11: closes;
/// - 20: with a selected item: clears the entered flag, decides whether a
///   companion may refuse a heavy item, and moves the item: a stack above
///   the quantity setting opens the quantity menu, else
///   `ItemSelectCallback` runs with the count (companion) or 1;
/// - others: nothing.
// Translation incomplete: 0075be80 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handle is not translated
pub fn container_menu_do_click(e: &mut Engine, this: Ptr<ContainerMenu>, id: i32, tile: u32) {
    if tile == 0 && (0..0x13).contains(&id) {
        let _ = tile_at(e, this.addr(), 0x28 + 4 * id as u32);
    }
    match id {
        0..=2 => {
            let sound = e.get(this, ContainerMenu::eFilterSound);
            e.call(PLAY_MENU_SOUND, &args![sound]);
            step_filter(
                e,
                this,
                id != 0,
                ContainerMenu::eItemsFilterMode,
                0x2c,
                ITEMS,
            );
        }
        5..=7 => {
            let sound = e.get(this, ContainerMenu::eFilterSound);
            e.call(PLAY_MENU_SOUND, &args![sound]);
            step_filter(
                e,
                this,
                id != 5,
                ContainerMenu::eContainerFilterMode,
                0x40,
                CONTAINER,
            );
        }
        10 => {
            let list = this.addr() + CONTAINER;
            e.call(
                LIST_BOX_FOR_EACH,
                &args![list, TAKE_ALL_FUNC, 0u32, 0x7fff_ffffu32],
            );
            play_named_sound(e, TAKE_ALL_SOUND);
            container_menu_close(e);
        }
        11 => container_menu_close(e),
        20 => click_item(e, this),
        _ => {}
    }
}

/// Steps a filter mode of [`container_menu_do_click`] until its list shows
/// something or the mode is 0.
fn step_filter(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    forward: bool,
    field: Field<ContainerMenu, i32>,
    title: u32,
    list: u32,
) {
    loop {
        let step = if forward { 1i32 } else { -1 };
        let mut mode = e.get(this, field).wrapping_add(step);
        if mode < 0 {
            mode = 5;
        } else if mode > 5 {
            mode = 0;
        }
        e.set(this, field, mode);
        let tile = tile_at(e, this.addr(), title);
        container_menu_set_filter_mode_title(e, this, tile);
        e.call(LIST_FILTER, &args![this.addr() + list, FILTER_MODE_FUNC]);
        if e.call(LIST_NUM_UNFILTERED, &args![this.addr() + list])
            .u32()
            != 0
        {
            break;
        }
        if e.get(this, field) == 0 {
            break;
        }
    }
}

/// Id 20 of [`container_menu_do_click`].
fn click_item(e: &mut Engine, this: Ptr<ContainerMenu>) {
    if current_item(e) == 0 {
        return;
    }
    e.set_global(ITEM_ENTERED, 0u8);
    let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    let light = e.call(IS_GOLD, &args![object]).bool() || {
        let player = player_actor(e);
        let flag = e.call(PLAYER_STATE_004D1360, &args![player]).u8();
        let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
        let weight = e.call(FORM_WEIGHT, &args![object, flag]).f64();
        // FCOMP; TEST AH,0x41; JP: weightless unless above 0.0 (or NaN).
        let zero = e.global::<f64>(ZERO_DOUBLE);
        weight <= zero
    };
    let companion_take = light
        && matches!(e.get(this, ContainerMenu::eContainerMode), 1 | 2)
        && e.get(this, ContainerMenu::plbCurrListbox) == this.addr() + CONTAINER;
    let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).i32();
    let setting = e.call(SETTING_VALUE_ADDRESS, &args![0x011d_2418u32]).u32();
    if count > e.mem.i32(setting) {
        let reference = e.get(this, ContainerMenu::pContainerRef);
        let player = player_actor(e);
        let item = current_item(e);
        if !fn_0075d510(e, this, item, player, reference) && !companion_take {
            let list = e.get(this, ContainerMenu::plbCurrListbox);
            e.call(LIST_SAVE_STATE, &args![list]);
            let list = e.get(this, ContainerMenu::plbCurrListbox);
            e.vcall(list, 0, &args![0u32]);
            let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).u32();
            e.call(
                QUANTITY_MENU_CREATE,
                &args![count, ITEM_SELECT_CALLBACK, 0x7fff_ffffu32],
            );
            return;
        }
    }
    if companion_take {
        let count = e.call(WORD_AT_FOUR, &args![current_item(e)]).i32();
        container_menu_item_select_callback(e, count);
    } else {
        container_menu_item_select_callback(e, 1);
    }
}

// Translated from 0075c250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoKey` (Xbox PDB): key `0x20` closes the menu and
/// answers true; other keys answer false.
pub fn container_menu_do_key(e: &mut Engine, _this: Ptr<ContainerMenu>, key: i32) -> bool {
    if key == 0x20 {
        container_menu_close(e);
        true
    } else {
        false
    }
}

// Translated from 0075c280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::UpdateLists` (Xbox PDB): saves the current list's state,
/// refills the lists (for one form, or all: mode 4 the player's mod list
/// against the player's items; otherwise the player's and the container's
/// inventories, the weapon's current ammo too when the form is a weapon;
/// the companion mode refills the container from the reference), adds the
/// active mines and a companion's dropped weapon to the container list,
/// walks, filters and sorts both lists, writes two values of the player
/// (`0093abe0`, rounded) into the tile at +0x34 of `pMe`, and restores the current
/// list (choosing a list with a gamepad, or clicking 2/7 and entering
/// `0x15`/`0x16` when it has nothing shown).
// Translation incomplete: 0075c280 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn container_menu_update_lists(e: &mut Engine, this: Ptr<ContainerMenu>, form: u32) {
    let me_addr = this.addr();
    let current = e.get(this, ContainerMenu::plbCurrListbox);
    if current != 0 && e.vcall(current, 4, &args![]).u32() != 0 {
        let current = e.get(this, ContainerMenu::plbCurrListbox);
        e.call(LIST_SAVE_STATE, &args![current]);
    }
    if e.get(this, ContainerMenu::eContainerMode) == 4 {
        let player = player_actor(e);
        let mods = e.call(PLAYER_MOD_LIST, &args![player]).u32();
        e.vcall(me_addr + CONTAINER, 0x1c, &args![]);
        let mut index = 0u32;
        while index < e.call(ITEM_OBJECT, &args![mods]).u32() {
            let slot = e.call(MOD_LIST_ENTRY, &args![mods, index]).u32();
            let item = e.mem.u32(slot);
            let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
            let slot = e.call(MOD_LIST_ENTRY, &args![mods, index]).u32();
            let item = e.mem.u32(slot);
            e.call(
                LIST_ADD_ITEM,
                &args![me_addr + CONTAINER, item, name, 0u32, 0u32],
            );
            index = index.wrapping_add(1);
        }
        if form != 0 {
            let player = player_actor(e);
            let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
            container_menu_update_item_in_list(e, this, form, changes, me_addr + ITEMS);
        } else {
            let player = player_actor(e);
            fill_list(e, me_addr + ITEMS, player);
        }
    } else if form != 0 {
        let player = player_actor(e);
        let player_changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
        let reference = e.get(this, ContainerMenu::pContainerRef);
        let container_changes = if reference != 0 {
            let reference = e.get(this, ContainerMenu::pContainerRef);
            e.call(GET_INVENTORY_CHANGES, &args![reference]).u32()
        } else {
            0
        };
        let ammo = if e.call(FORM_TYPE, &args![form]).u32() == 0x28 {
            let reference = e.get(this, ContainerMenu::pContainerRef);
            let actor = if is_actor(e, reference) {
                e.get(this, ContainerMenu::pContainerRef)
            } else {
                0
            };
            e.call(WEAPON_CURRENT_AMMO, &args![form, actor]).u32()
        } else {
            0
        };
        container_menu_update_item_in_list(e, this, form, player_changes, me_addr + ITEMS);
        if ammo != 0 {
            container_menu_update_item_in_list(e, this, ammo, player_changes, me_addr + ITEMS);
        }
        if e.get(this, ContainerMenu::eContainerMode) == 3 {
            let reference = e.get(this, ContainerMenu::pContainerRef);
            fill_list(e, me_addr + CONTAINER, reference);
        } else {
            container_menu_update_item_in_list(
                e,
                this,
                form,
                container_changes,
                me_addr + CONTAINER,
            );
            if ammo != 0 {
                container_menu_update_item_in_list(
                    e,
                    this,
                    ammo,
                    container_changes,
                    me_addr + CONTAINER,
                );
            }
        }
    } else {
        let player = player_actor(e);
        fill_list(e, me_addr + ITEMS, player);
        let reference = e.get(this, ContainerMenu::pContainerRef);
        fill_list(e, me_addr + CONTAINER, reference);
    }
    add_active_mines(e, this, form);
    add_dropped_weapon(e, this, form);
    for list in [ITEMS, CONTAINER] {
        e.call(
            LIST_BOX_FOR_EACH,
            &args![me_addr + list, INIT_FUNC, 0u32, 0x7fff_ffffu32],
        );
        e.call(LIST_FILTER, &args![me_addr + list, FILTER_MODE_FUNC]);
        e.vcall(me_addr + list, 0x20, &args![COMPARE_FUNC]);
    }
    show_weights(e);
    restore_current_list(e, this);
}

/// Fills a list from a reference's inventory with the first-pass filter.
fn fill_list(e: &mut Engine, list: u32, reference: u32) {
    e.call(
        LIST_FILL,
        &args![list, reference, FIRST_PASS_FILTER, 0u32, 0u32, 0u32, 0u32],
    );
}

/// The active mines of [`container_menu_update_lists`]: each mine of the
/// list (all, or the one of the form) is added to the container list as a
/// new item named with the setting `011d3f70`, its tile's trait `0xfa9`
/// set to the float at `0101e704`.
fn add_active_mines(e: &mut Engine, this: Ptr<ContainerMenu>, form: u32) {
    let me_addr = this.addr();
    let mut node = me_addr + ACTIVE_MINE_LIST;
    while node != 0 {
        let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
        let mine = e.mem.u32(slot);
        if mine == 0 {
            break;
        }
        if form == 0 || form == mine {
            let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
            let item = if block != 0 {
                let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
                let mine = e.mem.u32(slot);
                e.call(ITEM_CHANGE_CONSTRUCTOR, &args![block, mine, 1i32])
                    .u32()
            } else {
                0
            };
            e.with_stack(8, |e, text| {
                e.call(STRING_CONSTRUCT, &args![text]);
                let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
                let setting = e.call(SETTING_STRING, &args![0x011d_3f70u32]).u32();
                e.call(STRING_FORMAT, &args![text, ITEM_NAME_FORMAT, setting, name]);
                let c_str = e.call(FIRST_WORD, &args![text]).u32();
                let added = e
                    .call(
                        LIST_ADD_ITEM,
                        &args![me_addr + CONTAINER, item, c_str, 0u32, 0u32],
                    )
                    .u32();
                let alpha = x87_quiet_bits(e.global::<u32>(DROPPED_ALPHA));
                let slot = e.call(LIST_ITEM_TILE, &args![added.wrapping_add(4)]).u32();
                let tile = e.mem.u32(slot);
                e.call(SET_FLOAT_TRAIT, &args![tile, TRAIT_0XFA9, alpha, true]);
                e.call(STRING_DESTROY, &args![text]);
            });
        }
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
    }
}

/// The companion's dropped weapon of [`container_menu_update_lists`].
fn add_dropped_weapon(e: &mut Engine, this: Ptr<ContainerMenu>, form: u32) {
    let me_addr = this.addr();
    if e.get(this, ContainerMenu::pContainerRef) == 0 {
        return;
    }
    let reference = e.get(this, ContainerMenu::pContainerRef);
    if !is_actor(e, reference) {
        return;
    }
    let reference = e.get(this, ContainerMenu::pContainerRef);
    if !e.vcall(reference, 0x22c, &args![0u32]).bool() {
        return;
    }
    let reference = e.get(this, ContainerMenu::pContainerRef);
    let extra = e.call(REFERENCE_EXTRA, &args![reference]).u32();
    let dropped = e.call(EXTRA_DROPPED_REFERENCE, &args![extra]).u32();
    if dropped == 0 || e.call(REFERENCE_TEST_00437B90, &args![dropped]).bool() {
        return;
    }
    if form != 0 && form != e.call(FORM_OF, &args![dropped]).u32() {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let item = if block != 0 {
        let base = e.call(FORM_OF, &args![dropped]).u32();
        e.call(ITEM_CHANGE_CONSTRUCTOR, &args![block, base, 1i32])
            .u32()
    } else {
        0
    };
    let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
    e.call(
        LIST_ADD_ITEM,
        &args![me_addr + CONTAINER, item, name, 0u32, 0u32],
    );
    if e.call(FIRST_WORD, &args![item]).u32() != 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(SIMPLE_LIST_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(item, list);
    }
    let list = e.call(FIRST_WORD, &args![item]).u32();
    let slot = e.call(FIRST_WORD_ADDRESS, &args![list]).u32();
    if e.mem.u32(slot) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
        let extra_list = if block != 0 {
            e.call(EXTRA_DATA_LIST_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        e.with_stack(4, |e, holder| {
            e.mem.set_u32(holder.addr(), extra_list);
            let list = e.call(FIRST_WORD, &args![item]).u32();
            e.call(LIST_ADD_HEAD, &args![list, holder]);
        });
    }
    let list = e.call(FIRST_WORD, &args![item]).u32();
    let slot = e.call(FIRST_WORD_ADDRESS, &args![list]).u32();
    let extra_list = e.mem.u32(slot);
    e.call(EXTRA_ADD_DROPPED_ITEM, &args![extra_list, dropped]);
    let health = e.call(REFERENCE_HEALTH, &args![dropped]).f64();
    let list = e.call(FIRST_WORD, &args![item]).u32();
    let slot = e.call(FIRST_WORD_ADDRESS, &args![list]).u32();
    let extra_list = e.mem.u32(slot);
    e.call(EXTRA_SET_HEALTH, &args![extra_list, x87_store32(health)]);
}

/// The weight line of [`container_menu_update_lists`].
fn show_weights(e: &mut Engine) {
    e.with_stack(0x4c, |e, frame| {
        // [EBP-0x58] the first value, [EBP-0x5c] the second, [EBP-0x54] the text.
        let carried = frame.addr() + 4;
        let capacity = frame.addr();
        let text = frame.addr() + 8;
        let player = player_actor(e);
        e.call(PLAYER_WEIGHTS, &args![player, carried, capacity]);
        let rounded = e
            .call(
                ROUND_TO,
                &args![x87_quiet_bits(e.mem.u32(capacity)), 1.0f32],
            )
            .f64();
        let capacity_text = e.call(FLOAT_TO_INT, &args![x87_store32(rounded)]).u32();
        let rounded = e
            .call(ROUND_TO, &args![x87_quiet_bits(e.mem.u32(carried)), 1.0f32])
            .f64();
        let carried_text = e.call(FLOAT_TO_INT, &args![x87_store32(rounded)]).u32();
        let label = e.call(SETTING_STRING, &args![0x011d_1e60u32]).u32();
        e.call(
            PRINT_BOUNDED,
            &args![
                text,
                0x40u32,
                WEIGHT_FORMAT,
                label,
                carried_text,
                capacity_text
            ],
        );
        let tile = tile_at(e, me(e), 0x34);
        set_string(e, tile, TRAIT_STRING, text);
    });
}

/// The restore of the current list at the end of
/// [`container_menu_update_lists`].
fn restore_current_list(e: &mut Engine, this: Ptr<ContainerMenu>) {
    let me_addr = this.addr();
    if e.get(this, ContainerMenu::plbCurrListbox) == 0 && e.call(GAMEPAD_IN_USE, &args![]).bool() {
        e.set(this, ContainerMenu::plbCurrListbox, me_addr + CONTAINER);
    }
    if e.get(this, ContainerMenu::plbCurrListbox) == 0 {
        return;
    }
    let current = e.get(this, ContainerMenu::plbCurrListbox);
    if e.call(LIST_NUM_UNFILTERED, &args![current]).u32() != 0 {
        let current = e.get(this, ContainerMenu::plbCurrListbox);
        e.call(LIST_RESTORE_STATE, &args![current, 1u32]);
        return;
    }
    e.set(this, ContainerMenu::eFilterSound, -1);
    let id = if e.get(this, ContainerMenu::plbCurrListbox) != me_addr + ITEMS {
        7u32
    } else {
        2
    };
    e.vcall(me_addr, 0xc, &args![id, 0u32]);
    e.set(this, ContainerMenu::eFilterSound, 3);
    let current = e.get(this, ContainerMenu::plbCurrListbox);
    if e.call(LIST_NUM_UNFILTERED, &args![current]).u32() != 0 {
        let current = e.get(this, ContainerMenu::plbCurrListbox);
        e.call(LIST_RESTORE_STATE, &args![current, 1u32]);
        return;
    }
    let id = u32::from(e.get(this, ContainerMenu::plbCurrListbox) == me_addr + ITEMS) + 0x15;
    e.vcall(me_addr, 0x10, &args![id, 0u32]);
    if e.call(GAMEPAD_IN_USE, &args![]).bool() {
        let current = e.get(this, ContainerMenu::plbCurrListbox);
        let highlighted = e.call(LIST_HIGHLIGHTED, &args![current]).u32();
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(
            SET_FOCUS_TARGET,
            &args![interface, highlighted, 0xfc3u32, 1u32],
        );
    }
}

// Translated from 0075cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::UpdateItemInList` (Xbox PDB name from the call graph):
/// with an inventory, a form and a list, takes the inventory's item group
/// of the form into a temporary list, removes the form's group from the
/// list box and adds each item of the group again with its full name.
// Translation incomplete: 0075cc90 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary list is not translated
pub fn container_menu_update_item_in_list(
    e: &mut Engine,
    _this: Ptr<ContainerMenu>,
    form: u32,
    changes: u32,
    list: u32,
) {
    e.with_stack(8, |e, group| {
        e.call(SIMPLE_LIST_CONSTRUCTOR, &args![group]);
        if changes != 0 && form != 0 && list != 0 {
            e.call(GET_ITEM_GROUP, &args![changes, form, group]);
            e.call(LIST_REMOVE_ITEM_GROUP, &args![list, form]);
            let mut node = group.addr();
            while node != 0 {
                let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
                let name = e.call(ITEM_FULL_NAME, &args![e.mem.u32(slot)]).u32();
                let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
                let item = e.mem.u32(slot);
                e.call(LIST_ADD_ITEM, &args![list, item, name, 0u32, 0u32]);
                node = e.call(WORD_AT_FOUR, &args![node]).u32();
            }
            e.call(SIMPLE_LIST_CLEAR, &args![group]);
        }
        e.call(SIMPLE_LIST_DESTRUCTOR, &args![group]);
    });
}

// Translated from 0075cd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoEnter` (Xbox PDB). A null tile is taken from
/// `pTiles[id]` for `0 <= id < 19`. Id `0x15` leaves (`DoLeave(0x14, 0)`),
/// clears the container list's highlight and makes the items list current;
/// `0x16` the same the other way; `0x14` (an item) sets the entered flag,
/// switches lists when the tile's parent is the other list's container
/// (entering `0x15` or `0x16`), takes the current list's selected item as
/// `pCurrItem` and shows its stats, icon and the side marker (trait
/// `0x1004`: 1 items, 2 container).
pub fn container_menu_do_enter(e: &mut Engine, this: Ptr<ContainerMenu>, id: i32, tile: u32) {
    let me_addr = this.addr();
    let tile = if tile == 0 && (0..0x13).contains(&id) {
        tile_at(e, me_addr, 0x28 + 4 * id as u32)
    } else {
        tile
    };
    match id {
        0x15 | 0x16 => {
            let (cleared, current) = if id == 0x15 {
                (CONTAINER, ITEMS)
            } else {
                (ITEMS, CONTAINER)
            };
            e.vcall(me_addr, 0x14, &args![0x14u32, 0u32]);
            e.vcall(me_addr + cleared, 0, &args![0u32]);
            e.set(this, ContainerMenu::plbCurrListbox, me_addr + current);
        }
        0x14 => {
            if tile == 0 {
                return;
            }
            e.set_global(ITEM_ENTERED, 1u8);
            let on_items = e.get(this, ContainerMenu::plbCurrListbox) == me_addr + ITEMS;
            let switched = !on_items && {
                let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
                parent == tile_at(e, me_addr, 0x38)
            };
            if switched {
                e.vcall(me_addr, 0x10, &args![0x15u32, 0u32]);
            } else if e.get(this, ContainerMenu::plbCurrListbox) != me_addr + CONTAINER {
                let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
                if parent == tile_at(e, me_addr, 0x48) {
                    e.vcall(me_addr, 0x10, &args![0x16u32, 0u32]);
                }
            }
            let current = e.get(this, ContainerMenu::plbCurrListbox);
            let item = e.call(LIST_SELECTED_ITEM, &args![current]).u32();
            e.set_global(CURRENT_ITEM, item);
            if current_item(e) == 0 {
                return;
            }
            let stats = tile_at(e, me_addr, 0x5c);
            e.call(POPULATE_ITEM_STATS, &args![current_item(e), stats]);
            set_trait_from_int(e, stats, TRAIT_VISIBLE, 1);
            let icon = e.call(ITEM_GET_ICON, &args![current_item(e), 0u32]).u32();
            let icon_tile = tile_at(e, me_addr, 0x58);
            set_string(e, icon_tile, TRAIT_IMAGE, icon);
            set_trait_from_int(e, icon_tile, TRAIT_VISIBLE, 1);
            let side = i32::from(e.get(this, ContainerMenu::plbCurrListbox) != me_addr + ITEMS) + 1;
            let marker = tile_at(e, me_addr, 0x4c);
            set_trait_from_int(e, marker, 0x1004, side);
            let marker = tile_at(e, me_addr, 0x4c);
            set_trait_from_int(e, marker, TRAIT_VISIBLE, 1);
        }
        _ => {}
    }
}

// Translated from 0075cf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoLeave` (Xbox PDB): hides the stats, icon and side
/// marker tiles (+0x5c, +0x58, +0x4c). The two words are not read.
pub fn container_menu_do_leave(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    _unused_0: u32,
    _unused_1: u32,
) {
    for offset in [0x5cu32, 0x58, 0x4c] {
        let tile = tile_at(e, this.addr(), offset);
        set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
    }
}

// Translated from 0075cfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::FirstPassFilterFunc` (Xbox PDB name from the call graph,
/// cdecl): whether an item is left out of a list. In mode 4 an item not in
/// the current weapon's mod list is left out; otherwise an item without an
/// object, an unplayable biped model, an object that is not a playable
/// form (the cast and `0048cee0`), a weapon failing `0047bcf0` or ammo
/// failing `004c94d0` is left out. False without a menu.
pub fn container_menu_first_pass_filter_func(e: &mut Engine, item: u32) -> bool {
    if me(e) == 0 {
        return false;
    }
    let object = if item != 0 {
        e.call(ITEM_OBJECT, &args![item]).u32()
    } else {
        0
    };
    if e.get(menu(e), ContainerMenu::eContainerMode) == 4 {
        let player = player_actor(e);
        let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![player]).u32();
        let mods = e.call(MOD_LIST_OF, &args![weapon.wrapping_add(0xa4)]).u32();
        return e.call(LIST_FORM_INDEX, &args![mods, object]).i32() < 0;
    }
    let biped = e.call(BIPED_MODEL_OF, &args![object]).u32();
    let playable = e
        .call(
            RT_DYNAMIC_CAST,
            &args![object, 0i32, TYPE_FORM_BASE, TYPE_PLAYABLE_FORM, 0i32],
        )
        .u32();
    let weapon = if item_type(e, item) == 0x28 {
        e.call(ITEM_OBJECT, &args![item]).u32()
    } else {
        0
    };
    let ammo = if item_type(e, item) == 0x29 {
        e.call(ITEM_OBJECT, &args![item]).u32()
    } else {
        0
    };
    if object == 0 {
        return true;
    }
    if biped != 0 && !e.call(BIPED_PLAYABLE, &args![biped]).bool() {
        return true;
    }
    if playable == 0 || e.call(FORM_TEST_0048CEE0, &args![playable]).u32() == 0 {
        return true;
    }
    if weapon != 0 && !e.call(WEAPON_TEST_0047BCF0, &args![weapon]).bool() {
        return true;
    }
    ammo != 0 && !e.call(AMMO_TEST_004C94D0, &args![ammo]).bool()
}

// Translated from 0075d130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lists' sort function (cdecl): compares the string traits of the two
/// items' tiles (`00469880`, which calls `_mbscmp`); the second tile's string is read
/// first.
pub fn fn_0075d130(e: &mut Engine, first: u32, second: u32) -> i32 {
    let second_text = e
        .call(TILE_GET_STRING, &args![e.mem.u32(second), TRAIT_STRING])
        .u32();
    let first_text = e
        .call(TILE_GET_STRING, &args![e.mem.u32(first), TRAIT_STRING])
        .u32();
    e.call(COMPARE_STRINGS, &args![first_text, second_text])
        .i32()
}

// Translated from 0075d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::InitFunc` (Xbox PDB name from the call graph, cdecl):
/// sets up an item's tile: shows the worn marker of the item's tile
/// (`0073ac90`) when the item is worn, writes `"name (count)"` with the
/// format `0106eb00` for a stack of more than one, sets the tile's text to
/// the name, copies the tile's string to its second tile and clears the
/// trait `0xfb0`.
// Translation incomplete: 0075d160 - the /GS stack cookie check (00ec408c) is not translated
pub fn container_menu_init_func(e: &mut Engine, tile: u32, item: u32) {
    if e.call(ITEM_GET_WORN, &args![item, 0u32]).bool() {
        let slot = e
            .call(LIST_ITEM_TILE_INIT, &args![tile.wrapping_add(4)])
            .u32();
        let marker = e.mem.u32(slot);
        set_trait_from_int(e, marker, TRAIT_VISIBLE, 1);
    }
    if e.call(WORD_AT_FOUR, &args![item]).i32() > 1 {
        e.with_stack(0x80, |e, text| {
            let count = e.call(WORD_AT_FOUR, &args![item]).u32();
            let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
            e.call(
                PRINT_BOUNDED,
                &args![text, 0x80u32, COUNT_FORMAT, name, count],
            );
            set_string(e, tile, TRAIT_STRING, text.addr());
        });
    }
    let name = e.call(ITEM_FULL_NAME, &args![item]).u32();
    e.call(TILE_SET_TEXT, &args![tile, name]);
    let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
    let slot = e.call(LIST_ITEM_TILE, &args![tile.wrapping_add(4)]).u32();
    let second = e.mem.u32(slot);
    set_string(e, second, TRAIT_STRING, text);
    e.call(SET_FLOAT_TRAIT, &args![tile, TRAIT_0XFB0, 0.0f32, true]);
}

// Translated from 0075d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether moving the item is the special case of a weapon taken from a
/// pickpocketed actor: in mode 2 with the items list current, the player as
/// `to` passing `004997b0`, an actor `from` that answers its slots `+0x22c`
/// and `+0x1a0` false, a weapon item and the mine list's test passing, the
/// answer is whether the weapon's ammo for the actor has flags `0x20000`;
/// an essential actor or one whose process answers its slot `+0x388` true
/// answers false.
pub fn fn_0075d510(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    item: u32,
    to: u32,
    from: u32,
) -> bool {
    if e.get(this, ContainerMenu::eContainerMode) != 2
        || e.get(this, ContainerMenu::plbCurrListbox) != this.addr() + ITEMS
        || to != player_actor(e)
    {
        return false;
    }
    let player = player_actor(e);
    if !e.call(PLAYER_TEST_004997B0, &args![player]).bool()
        || !is_actor(e, from)
        || e.vcall(from, 0x22c, &args![0u32]).bool()
        || e.vcall(from, 0x1a0, &args![0u32]).bool()
        || item_type(e, item) != 0x28
        || !e
            .call(LIST_TEST_008256D0, &args![this.addr() + ACTIVE_MINE_LIST])
            .bool()
    {
        return false;
    }
    let mut actor = 0u32;
    if is_actor(e, from) {
        actor = from;
        if e.call(ACTOR_GET_ESSENTIAL, &args![actor]).bool() {
            return false;
        }
        if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            if e.vcall(process, 0x388, &args![]).bool() {
                return false;
            }
        }
    }
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    if !e.call(OBJECT_TEST_004C0BF0, &args![object]).bool() {
        return false;
    }
    if e.call(WEAPON_AMMO_FOR, &args![object, actor]).u32() == 0 {
        return false;
    }
    let ammo = e.call(WEAPON_AMMO_FOR, &args![object, actor]).u32();
    e.call(AMMO_FLAGS, &args![ammo]).u32() == 0x20000
}

// Translated from 0075d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::SwitchContainer` (Xbox PDB name from the call graph):
/// moves `count` of the item from one side to the other (from the
/// container to the player when the items list is current, else back).
/// The special weapon case (`0075d510`) transfers the weapon's projectile
/// state (`009bd5c0`), adds the object to the mine list and moves it with
/// the receiver's slot `+0x17c`; an item dropped by someone else is given
/// back through the player's slot `+0x3d0`; mode 4 moves Rock-It ammo stack
/// by stack; otherwise the receiver's slot `+0x17c` moves it (the
/// pickpocket flag set in mode 2). Nothing without both words.
pub fn container_menu_switch_container(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    tile: u32,
    item: u32,
    count: i32,
) {
    if item == 0 || tile == 0 {
        return;
    }
    let on_items = e.get(this, ContainerMenu::plbCurrListbox) == this.addr() + ITEMS;
    let from = if on_items {
        e.get(this, ContainerMenu::pContainerRef)
    } else {
        player_actor(e)
    };
    let on_items = e.get(this, ContainerMenu::plbCurrListbox) == this.addr() + ITEMS;
    let to = if on_items {
        player_actor(e)
    } else {
        e.get(this, ContainerMenu::pContainerRef)
    };
    let special = fn_0075d510(e, this, item, to, from);
    let extra = if e.call(FIRST_WORD, &args![item]).u32() != 0 {
        let list = e.call(FIRST_WORD, &args![item]).u32();
        let slot = e.call(FIRST_WORD_ADDRESS, &args![list]).u32();
        e.mem.u32(slot)
    } else {
        0
    };
    if special {
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        let weapon = e.call(ITEM_OBJECT, &args![item]).u32();
        let ammo = e.call(WEAPON_AMMO_FOR, &args![weapon, 0u32]).u32();
        e.call(WEAPON_TRANSFER_009BD5C0, &args![ammo, to, from, object]);
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), object);
            e.call(LIST_ADD_HEAD, &args![this.addr() + ACTIVE_MINE_LIST, slot]);
        });
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        e.vcall(
            to,
            0x17c,
            &args![object, extra, count, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
        return;
    }
    if extra != 0 && e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32() != 0 {
        let dropper = e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32();
        if !e.call(FORM_TEST_00440DA0, &args![dropper]).bool() {
            let dropper = e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32();
            if !e.call(FORM_TEST_00440D80, &args![dropper]).bool() {
                let dropper = e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32();
                let player = player_actor(e);
                e.vcall(player, 0x3d0, &args![dropper, 1u32, 0u32]);
                return;
            }
        }
    }
    if e.get(this, ContainerMenu::eContainerMode) == 4 {
        move_rock_it_ammo(e, item, from, to, count);
        return;
    }
    let pickpocket = u32::from(e.get(this, ContainerMenu::eContainerMode) == 2);
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    e.vcall(
        to,
        0x17c,
        &args![object, extra, count, pickpocket, 0u32, from, 0u32, 0u32, 1u32, 0u32],
    );
}

/// Mode 4 of [`container_menu_switch_container`]: Rock-It ammo, stack by
/// stack of the item's extra-data lists.
fn move_rock_it_ammo(e: &mut Engine, item: u32, from: u32, to: u32, count: i32) {
    let mut left = count;
    let mut node = e.call(FIRST_WORD, &args![item]).u32();
    while left > 0 {
        let extra = if node != 0 {
            let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
            e.mem.u32(slot)
        } else {
            0
        };
        let stack = if extra != 0 {
            i32::from(e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16)
        } else {
            left
        };
        let taken = e.call(MIN_OF, &args![stack, left]).i32();
        let player = player_actor(e);
        if from == player {
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            let player = player_actor(e);
            let removed = e
                .call(
                    REMOVE_ROCK_IT_AMMO,
                    &args![player, object, extra, taken, 1u32],
                )
                .u32();
            let removed_extra = if e.call(FIRST_WORD, &args![removed]).u32() != 0 {
                let list = e.call(FIRST_WORD, &args![removed]).u32();
                let slot = e.call(FIRST_WORD_ADDRESS, &args![list]).u32();
                e.mem.u32(slot)
            } else {
                0
            };
            let removed_count = e.call(WORD_AT_FOUR, &args![removed]).u32();
            let object = e.call(ITEM_OBJECT, &args![removed]).u32();
            let player = player_actor(e);
            e.vcall(player, 0x190, &args![object, removed_extra, removed_count]);
            if removed != 0 {
                e.call(ITEM_CHANGE_DELETE, &args![removed, 1u32]);
            }
            node = e.call(FIRST_WORD, &args![item]).u32();
        } else if to == player {
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            e.call(ADD_ROCK_IT_AMMO, &args![player, object, extra, taken]);
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            e.vcall(
                to,
                0x17c,
                &args![object, extra, taken, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            node = if node != 0 {
                e.call(WORD_AT_FOUR, &args![node]).u32()
            } else {
                0
            };
        }
        left = left.wrapping_sub(taken);
    }
}

// Translated from 0075dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::ItemSelectCallback` (Xbox PDB, cdecl): moves `count` of
/// the selected item. Nothing moves for a zero count, without a selected
/// item or its tile, or for a tile whose trait `0xfa9` equals the double at
/// `0102e430`. A companion (mode 3, items list current) refuses an item
/// when `008a0c20` of it is not above `00577250` of it plus the items'
/// weight, with a message and the bark `010722dc`. When pickpocketing (mode 2) an owned item (or any item
/// of an actor), the stolen value is counted and a failed pickpocket check
/// closes the menu. Otherwise the item moves (`SwitchContainer`), the lists
/// are updated for its object and, when the player's weapon is gone from
/// the inventory, the player's process slot `+0x168` is called with 0.
// Translation incomplete: 0075dc80 - the /GS stack cookie check (00ec408c) is not translated
pub fn container_menu_item_select_callback(e: &mut Engine, count: i32) {
    let current = e.get(menu(e), ContainerMenu::plbCurrListbox);
    let tile = e
        .call(LIST_GET_TILE_FOR, &args![current, CURRENT_ITEM])
        .u32();
    let reference = e.get(menu(e), ContainerMenu::pContainerRef);
    let actor = if reference != 0 && is_actor(e, reference) {
        e.get(menu(e), ContainerMenu::pContainerRef)
    } else {
        0
    };
    let mut refused = false;
    let target = if e.get(menu(e), ContainerMenu::plbCurrListbox) == me(e).wrapping_add(ITEMS) {
        player_actor(e)
    } else {
        e.get(menu(e), ContainerMenu::pContainerRef)
    };
    if count == 0 || current_item(e) == 0 || tile == 0 {
        refused = true;
    }
    let slot = e.call(LIST_ITEM_TILE, &args![tile.wrapping_add(4)]).u32();
    let value = e
        .call(GET_FLOAT_TRAIT, &args![e.mem.u32(slot), TRAIT_0XFA9])
        .f64();
    if value == e.global::<f64>(DISABLED_VALUE) {
        refused = true;
    }
    if e.get(menu(e), ContainerMenu::eContainerMode) == 3
        && e.get(menu(e), ContainerMenu::plbCurrListbox) == me(e).wrapping_add(ITEMS)
        && actor != 0
        && companion_is_full(e, actor, count)
    {
        e.with_stack(0x104, |e, text| {
            let setting = e.call(SETTING_STRING, &args![0x011d_2628u32]).u32();
            let name = e.call(REFERENCE_NAME, &args![actor]).u32();
            e.call(
                PRINT_BOUNDED,
                &args![text, 0x104u32, ITEM_NAME_FORMAT, name, setting],
            );
            let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
            e.call(SHOW_MESSAGE, &args![text, 2u32, 0u32, 0u32, time, 0u32]);
        });
        refused = true;
        play_bark(e, COMPANION_FULL_TOPIC);
    }
    if me(e) != 0 && e.get(menu(e), ContainerMenu::eContainerMode) == 2 {
        let player = player_actor(e);
        let counts = if target == player && actor != 0 {
            true
        } else if target == player {
            false
        } else {
            let owner = e.call(ITEM_GET_OWNERSHIP, &args![current_item(e)]).u32();
            let player = player_actor(e);
            owner != e.call(FORM_OF, &args![player]).u32()
        };
        if counts {
            let taking =
                e.get(menu(e), ContainerMenu::plbCurrListbox) == me(e).wrapping_add(CONTAINER);
            let stolen = e
                .call(
                    GET_STOLEN_VALUE,
                    &args![me(e), current_item(e), count, taking],
                )
                .i32();
            let sign = if taking { 1i32 } else { -1 };
            let mut value = stolen.wrapping_mul(sign);
            let total = e
                .get(menu(e), ContainerMenu::iValueStolen)
                .wrapping_add(value);
            e.set(menu(e), ContainerMenu::iValueStolen, total);
            if !taking {
                value = 1;
            }
            if actor != 0 && !e.call(PICKPOCKET_CHECK, &args![me(e), actor, value]).bool() {
                refused = true;
                container_menu_close(e);
            }
        }
    }
    if refused {
        return;
    }
    let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    let player = player_actor(e);
    let process = e.call(ACTOR_PROCESS, &args![player]).u32();
    let weapon = e.vcall(process, 0x14c, &args![]).u32();
    let player = player_actor(e);
    e.call(PLAYER_EQUIP_STATE, &args![player, object, 1u32, 0u32]);
    container_menu_switch_container(e, menu(e), tile, current_item(e), count);
    container_menu_update_lists(e, menu(e), object);
    if weapon != 0 {
        let weapon_object = e.call(ITEM_OBJECT, &args![weapon]).u32();
        let player = player_actor(e);
        let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
        if e.call(GET_OBJECT_COUNT, &args![changes, weapon_object])
            .i32()
            == 0
        {
            let player = player_actor(e);
            let process = e.call(ACTOR_PROCESS, &args![player]).u32();
            e.vcall(process, 0x168, &args![0u32]);
        }
    }
}

/// The companion's capacity test of
/// [`container_menu_item_select_callback`]: `00577250(actor, 0)` plus
/// `count` times the item's weight (x87, through `double` temporaries)
/// against `008a0c20(actor)`; full when that value is not above the sum
/// (NaN is not full).
fn companion_is_full(e: &mut Engine, actor: u32, count: i32) -> bool {
    let carried = e.call(ACTOR_VALUE_00577250, &args![actor, 0u32]).f64();
    let player = player_actor(e);
    let flag = e.call(PLAYER_STATE_004D1360, &args![player]).u8();
    let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
    let weight = e.call(FORM_WEIGHT, &args![object, flag]).f64();
    let total = x87_add(x87_mul(weight, f64::from(count)), carried);
    let capacity = e.call(ACTOR_VALUE_008A0C20, &args![actor]).f64();
    // FCOMP; TEST AH,0x41; JP: the message is skipped when the capacity
    // is above the total or unordered.
    capacity <= total
}

// Translated from 0075e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::SetFilterModeTitle` (Xbox PDB name from the call graph):
/// writes the title of the filter mode of the list whose title tile it is
/// (+0x2c the items list, any other the container list) into the tile:
/// mode 0 the items list's setting (the container list gets the container's
/// name instead), 1 to 5 their settings; the text is upper-cased (`00730490`, which
/// calls `_strupr_s`) when the setting `011c3cb4` equals the text at
/// `01018348` (`00408b20`, which calls `strcmp`). Nothing without a
/// reference or a tile; a mode above 5 writes the buffer as it is (the
/// exe's buffer is uninitialised stack there; the model's block is zeroed).
// Translation incomplete: 0075e380 - the /GS stack cookie check (00ec408c) is not translated
pub fn container_menu_set_filter_mode_title(e: &mut Engine, this: Ptr<ContainerMenu>, tile: u32) {
    let mode = if tile == tile_at(e, this.addr(), 0x2c) {
        e.get(this, ContainerMenu::eItemsFilterMode)
    } else {
        e.get(this, ContainerMenu::eContainerFilterMode)
    };
    let language = e.call(SETTING_STRING, &args![0x011c_3cb4u32]).u32();
    let keep_case = e.call(COMPARE_TEXT, &args![language, 0x0101_8348u32]).u32() != 0;
    if e.get(this, ContainerMenu::pContainerRef) == 0 || tile == 0 {
        return;
    }
    e.with_stack(0x104, |e, text| {
        let setting = match mode {
            0 => {
                if tile == tile_at(e, this.addr(), 0x2c) {
                    Some(0x011d_4378u32)
                } else {
                    let reference = e.get(this, ContainerMenu::pContainerRef);
                    let name = e.call(REFERENCE_NAME, &args![reference]).u32();
                    container_menu_set_container_title(e, this, name);
                    None
                }
            }
            1 => Some(0x011d_29b8),
            2 => Some(0x011d_4d50),
            3 => Some(0x011d_40b4),
            4 => Some(0x011d_2acc),
            5 => Some(0x011d_4918),
            _ => None,
        };
        if let Some(setting) = setting {
            let source = e.call(SETTING_STRING, &args![setting]).u32();
            e.call(COPY_BOUNDED, &args![text, 0x104u32, source]);
            if !keep_case {
                e.call(TO_UPPER, &args![text, 0x104u32]);
            }
            if mode == 0 {
                set_string(e, tile, TRAIT_STRING, text.addr());
            }
        }
        if mode != 0 {
            set_string(e, tile, TRAIT_STRING, text.addr());
        }
    });
}

// Translated from 0075e650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::FilterModeFunc` (Xbox PDB name from the call graph,
/// cdecl): whether an item is hidden by the filter mode of the list that
/// shows it (`pMe`'s items list, else the container list). Mode 1 shows
/// weapons (`0x28`), 2 armour and clothing (`0x18`, `0x1a`), 3 aid
/// (`0x1d`, `0x2f`, `0x19`), 4 hides those kinds and the rest of the
/// listed types, 5 shows ammo (`0x29`); ammo in the form list `010722f4`
/// or failing `004c94d0` is hidden. In the items list keys (`0x2e`) and
/// objects answering their slot `+0x94` true are hidden; in the container
/// list when pickpocketing, worn items are.
pub fn container_menu_filter_mode_func(e: &mut Engine, item: u32) -> bool {
    let kind = i32::from(item_type(e, item));
    let (tile, mode, in_items) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        let list = me(e).wrapping_add(ITEMS);
        let tile = e.call(LIST_GET_TILE_FOR, &args![list, slot]).u32();
        if tile != 0 {
            (tile, e.get(menu(e), ContainerMenu::eItemsFilterMode), true)
        } else {
            let list = me(e).wrapping_add(CONTAINER);
            let tile = e.call(LIST_GET_TILE_FOR, &args![list, slot]).u32();
            (
                tile,
                e.get(menu(e), ContainerMenu::eContainerFilterMode),
                false,
            )
        }
    });
    let _ = tile;
    let mut hidden = match mode {
        1 => kind != 0x28,
        2 => !(kind == 0x18 || kind == 0x1a),
        3 => !(kind == 0x1d || kind == 0x2f || kind == 0x19),
        4 => matches!(
            kind,
            0x28 | 0x18 | 0x1a | 0x1d | 0x2f | 0x29 | 0x19 | 0x6c | 0x73 | 0x74
        ),
        5 => kind != 0x29,
        _ => false,
    };
    if kind == 0x29 {
        let list = e.call(FORM_BY_EDITOR_ID, &args![FILTER_FORM_LIST]).u32();
        let object = e.call(ITEM_OBJECT, &args![item]).u32();
        if list != 0 && e.call(LIST_FORM_INDEX, &args![list, object]).i32() >= 0 {
            hidden = true;
        }
        if object != 0 && !e.call(AMMO_TEST_004C94D0, &args![object]).bool() {
            hidden = true;
        }
    }
    if in_items {
        hidden = hidden || kind == 0x2e || {
            let object = e.call(ITEM_OBJECT, &args![item]).u32();
            e.vcall(object, 0x94, &args![]).bool()
        };
    } else if e.get(menu(e), ContainerMenu::eContainerMode) == 2 {
        hidden = hidden || e.call(ITEM_GET_WORN, &args![item, 0u32]).bool();
    }
    hidden
}

// Translated from 0075e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::SetContainerTitle` (Xbox PDB name from the call graph):
/// writes `text` into the title tile at +0x40. When the text is wider than
/// the room (the parent's trait `0xfb1` less the margin `01072310`, and
/// less `010356b0` when the tile at +0x3c is shown), it is copied into a
/// new 0x104-byte buffer glyph by glyph (the widths of the font one below
/// the tile's, x87 sums) while it fits, `01072308` is appended and the
/// buffer written and freed. The copy does not stop at the text's end: it
/// stops on the width or after 0x104 bytes, as the exe does. Nothing for
/// an empty text.
pub fn container_menu_set_container_title(e: &mut Engine, this: Ptr<ContainerMenu>, text: u32) {
    if text == 0 || e.mem.i8(text) == 0 {
        return;
    }
    let title = tile_at(e, this.addr(), 0x40);
    let font_value = e.call(GET_FLOAT_TRAIT, &args![title, 0xfb9u32]).f64();
    let font = e.call(FTOL, &args![font_value]).i32();
    let dimensions = e.with_stack(0xc, |e, size| {
        let largest = x87_quiet_bits(e.global::<u32>(LARGEST_FLOAT));
        let fonts = e.call(FONT_MANAGER, &args![]).u32();
        e.call(
            CALCULATE_STRING_DIMENSIONS,
            &args![fonts, size, text, font, largest, 0u32],
        );
        e.mem.u32(size.addr())
    });
    let parent = e.call(WORD_AT_0X28, &args![title]).u32();
    let width = e.call(GET_FLOAT_TRAIT, &args![parent, 0xfb1u32]).f64();
    let mut room = x87_store32(x87_sub(width, e.global::<f64>(TITLE_MARGIN)));
    let icon = tile_at(e, this.addr(), 0x3c);
    if e.call(TILE_IS_TRUE, &args![icon, TRAIT_VISIBLE]).bool() {
        room = x87_store32(x87_sub(x87_m32(room), e.global::<f64>(TITLE_ICON_MARGIN)));
    }
    let needed = x87_m32(dimensions);
    let room_value = x87_m32(room);
    // FCOMPP; TEST AH,5; JP: the text fits unless the room is below it.
    if !(room_value < needed) {
        set_string(e, title, TRAIT_STRING, text);
        return;
    }
    let buffer = e.call(OPERATOR_NEW, &args![0x104u32]).u32();
    let fonts = e.call(FONT_MANAGER, &args![]).u32();
    let small = e
        .call(FONT_MANAGER_FONT, &args![fonts, font.wrapping_sub(1)])
        .u32();
    let glyphs = e.mem.u32(small.wrapping_add(0x38)).wrapping_add(0x128);
    let mut used = 0u32;
    let mut index = 0i32;
    e.call(MEMSET, &args![buffer, 0i32, 0x104u32]);
    let dots = x87_add(
        x87_add(
            x87_m32(e.mem.u32(glyphs.wrapping_add(0xa3c))),
            x87_m32(e.mem.u32(glyphs.wrapping_add(0xa34))),
        ),
        x87_m32(e.mem.u32(glyphs.wrapping_add(0xa40))),
    );
    let dots = x87_mul(dots, e.global::<f64>(TITLE_FONT_SCALE));
    used = x87_store32(x87_add(dots, x87_m32(used)));
    loop {
        // FCOMPP; TEST AH,1; JNZ: stop when the room is below the width or
        // either is NaN.
        let room_value = x87_m32(room);
        let width = x87_m32(used);
        if room_value < width || room_value.is_nan() || width.is_nan() {
            break;
        }
        if index >= 0x104 {
            break;
        }
        let byte = e.mem.u8(text.wrapping_add(index as u32));
        e.mem.set_u8(buffer.wrapping_add(index as u32), byte);
        let glyph = i32::from(byte as i8).wrapping_mul(0x38) as u32;
        let base = glyphs.wrapping_add(glyph);
        let sum = x87_add(
            x87_add(
                x87_m32(e.mem.u32(base.wrapping_add(0x2c))),
                x87_m32(e.mem.u32(base.wrapping_add(0x24))),
            ),
            x87_m32(e.mem.u32(base.wrapping_add(0x30))),
        );
        used = x87_store32(x87_add(sum, x87_m32(used)));
        index += 1;
    }
    e.call(APPEND_BOUNDED, &args![buffer, 0x104u32, ELLIPSIS]);
    let title = tile_at(e, this.addr(), 0x40);
    set_string(e, title, TRAIT_STRING, buffer);
    e.call(OPERATOR_DELETE, &args![buffer]);
}

// Translated from 0075eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoIdle` (Xbox PDB): once the subtitle time has passed,
/// clears it and the subtitle child tile's text; while the container list
/// has items and an item was entered, fills the item card for a selected
/// weapon (the tiles at +0x60, +0x64, +0x68) or armour (`0x18`: the tiles
/// at +0x6c and +0x70 when the object's two values are not 0.0).
pub fn container_menu_do_idle(e: &mut Engine, this: Ptr<ContainerMenu>) {
    let me_addr = this.addr();
    if e.get(this, ContainerMenu::m_nTimeToClearSubtitle) != 0
        && e.call(CURRENT_TIME, &args![]).u32() > e.get(this, ContainerMenu::m_nTimeToClearSubtitle)
    {
        e.set(this, ContainerMenu::m_nTimeToClearSubtitle, 0);
        let menu_tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
        let subtitle = e
            .call(TILE_FIND_CHILD, &args![menu_tile, SUBTITLE_TILE_NAME])
            .u32();
        if subtitle != 0 {
            set_string(e, subtitle, TRAIT_STRING, EMPTY_TEXT);
        }
    }
    if e.call(LIST_COUNT, &args![me_addr + CONTAINER]).u32() == 0
        || e.global::<u8>(ITEM_ENTERED) == 0
    {
        return;
    }
    if current_item(e) != 0 && item_type(e, current_item(e)) == 0x28 {
        let first = tile_at(e, me_addr, 0x60);
        let second = tile_at(e, me_addr, 0x64);
        let third = tile_at(e, me_addr, 0x68);
        let item = current_item(e);
        e.call(SET_ITEM_CARD, &args![first, second, third, item, 0u32]);
    } else if current_item(e) != 0 && item_type(e, current_item(e)) == 0x18 {
        let zero = e.global::<f64>(ZERO_DOUBLE);
        let second = if current_item(e) != 0 && {
            let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
            e.call(OBJECT_VALUE_004BE180, &args![object]).f64() != zero
        } {
            tile_at(e, me_addr, 0x70)
        } else {
            0
        };
        let first = if current_item(e) != 0 && {
            let object = e.call(ITEM_OBJECT, &args![current_item(e)]).u32();
            e.call(OBJECT_VALUE_004BE060, &args![object]).f64() != zero
        } {
            tile_at(e, me_addr, 0x6c)
        } else {
            0
        };
        let item = current_item(e);
        e.call(SET_ITEM_CARD, &args![first, second, 0u32, item, 0u32]);
    }
}

/// `Interface::IsTopMenuID` (cdecl: class).
const IS_TOP_MENU: u32 = 0x0070_2450;
/// The interface manager's wheel direction (`0070ec70`): negative for one way.
const WHEEL_DIRECTION_0070EC70: u32 = 0x0070_ec70;
/// The list box's virtual slot +4 (`DoGamepad` tests it for 0 before entering the other list).
const LIST_BOX_SLOT_4: u32 = 0x4;
/// `ExtraDataList`: the owner word `GetStolenValue` compares (`00418660`; what it holds is not
/// confirmed beyond that use).
const EXTRA_OWNER_00418660: u32 = 0x0041_8660;
/// `ItemChange::GetItemValue` (`ST0`).
const ITEM_GET_VALUE: u32 = 0x004b_d400;
/// `ActorValueOwner::GetClampedActorValue` (index) and the pickpocket chance (`00643400`,
/// cdecl: the player's value, the target's value, the item value as a float).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const PICKPOCKET_CHANCE_00643400: u32 = 0x0064_3400;
/// The random generator (`00476c00`) and `BSRandom::UnsignedInt` (bound).
const RANDOM: u32 = 0x0047_6c00;
const RANDOM_UNSIGNED: u32 = 0x00aa_5230;
/// `MiscStatManager::Increment` (cdecl: stat).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// The player's reaction to a caught pickpocket (`008c00e0`, `this` = player: target, 0, 0).
const PLAYER_CAUGHT_008C00E0: u32 = 0x008c_00e0;
/// The setting of the "caught" message and of the karma reward (`Setting` objects).
const SETTING_CAUGHT_MESSAGE: u32 = 0x011d_3f94;
const SETTING_KARMA: u32 = 0x011c_de20;
/// A setting's float address (`00403e20`).
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// The base data of a reference (`004181e0`), `TESActorBaseData::GetAlignmentForKarma`
/// (cdecl: float) and `PlayerCharacter::RewardKarma` (int).
const REFERENCE_BASE_DATA_004181E0: u32 = 0x0041_81e0;
const ALIGNMENT_FOR_KARMA: u32 = 0x0047_e040;
const REWARD_KARMA: u32 = 0x0094_fd30;
/// `_ftol2` (`ST0` passed as an `f64` argument).
const FTOL2: u32 = 0x00ec_62c0;
/// `TESTopic::GetMatchingInfo` (speaker, target).
const TOPIC_GET_MATCHING_INFO: u32 = 0x0061_a720;
/// The quest of a topic info (`006733e0`; the engine map's name for the folded body is
/// `LowProcess::GetSecondGenericLocation`).
const INFO_QUEST_006733E0: u32 = 0x0067_33e0;
/// `DialogueItem`: constructor (quest, topic, info, speaker), destructor, `FirstResponse`,
/// `GetCurrentResponse`.
const DIALOGUE_ITEM_CONSTRUCT: u32 = 0x0083_c520;
const DIALOGUE_ITEM_DESTRUCT: u32 = 0x0083_c670;
const DIALOGUE_ITEM_FIRST_RESPONSE: u32 = 0x0083_c7b0;
const DIALOGUE_ITEM_CURRENT_RESPONSE: u32 = 0x0083_c820;
/// The text of a `Tile::Value`-style string holder (`0084e3a0`).
const VALUE_STRING_0084E3A0: u32 = 0x0084_e3a0;
/// The actor's sound for a voice file (`00933150`: handle, file, 0, 0x102, 1).
const ACTOR_VOICE_SOUND_00933150: u32 = 0x0093_3150;
/// Accessors of a dialogue response used for the speech call: `00441110`, `009611e0`,
/// `004048e0` (of the response's +`006815c0` field), `00460140`.
const RESPONSE_00441110: u32 = 0x0044_1110;
const RESPONSE_009611E0: u32 = 0x0096_11e0;
const STRING_004048E0: u32 = 0x0040_48e0;
const RESPONSE_00460140: u32 = 0x0046_0140;
/// `BSStringT` copy constructor (`004047f0`, source).
const STRING_COPY_CONSTRUCT: u32 = 0x0040_47f0;
/// The type descriptors of `TESObjectREFR` and `Actor` for `__RTDynamicCast` (read as the
/// code passes them).
const TYPE_011841CC: u32 = 0x0118_41cc;
const TYPE_011846D4: u32 = 0x0118_46d4;
/// The subtitles setting's text (`00408d60` on `011d8928`) and
/// `Actor::AlwaysShowSubtitles`.
const SUBTITLES_STRING_00408D60: u32 = 0x0040_8d60;
const SUBTITLES_SETTING: u32 = 0x011d_8928;
const ALWAYS_SHOW_SUBTITLES: u32 = 0x008c_1bc0;
/// `BSSoundHandle::GetDuration`, `strlen` (`00ec6130`), the subtitle speed setting and the
/// double 1000.0.
const SOUND_HANDLE_GET_DURATION: u32 = 0x00ad_8b30;
const STRLEN: u32 = 0x00ec_6130;
const SETTING_SUBTITLE_SPEED: u32 = 0x011d_2178;
const THOUSAND_01017B70: u32 = 0x0101_7b70;
/// `ExtraDataList::GetCount` (`i16` in `AX`) is `EXTRA_GET_COUNT`; the item's sound tile
/// name `CM_Subtitle`.
const SUBTITLE_TILE: u32 = 0x0107_21f4;

/// The tile a list item shows (`007b5370` on its +4, then the word there).
fn item_tile(e: &mut Engine, tile: u32) -> u32 {
    let slot = e.call(LIST_ITEM_TILE, &args![tile.wrapping_add(4)]).u32();
    e.mem.u32(slot)
}

/// The first extra data list of an item change (the item of its list's first node), or 0.
fn first_extra_list(e: &mut Engine, item: u32) -> u32 {
    if e.call(FIRST_WORD, &args![item]).u32() == 0 {
        return 0;
    }
    let node = e.call(FIRST_WORD, &args![item]).u32();
    let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
    e.mem.u32(slot)
}

/// The container reference as an `Actor` (`__RTDynamicCast`), or 0 when it is none.
fn container_actor(e: &mut Engine) -> u32 {
    let reference = e.mem.u32(me(e).wrapping_add(0x74));
    if reference != 0 && is_actor(e, reference) {
        let reference = e.mem.u32(me(e).wrapping_add(0x74));
        e.call(
            RT_DYNAMIC_CAST,
            &args![reference, 0u32, TYPE_011841CC, TYPE_011846D4, 0u32],
        )
        .u32()
    } else {
        0
    }
}

// Translated from 0075cc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, used on the player): the address `this + 0xdf4` (the weapon mod
/// list `ContainerMenu` walks).
pub fn fn_0075cc70(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0xdf4)
}

// Translated from 0075d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoGamepad` (Xbox PDB; `thiscall`, `RET 8`: button, an unread word): with
/// the menu on top, buttons 3 and 4 click (`DoClick`, slot +0xc) the filter buttons of the
/// current list (0 or 2 for the player's items, 5 or 7 for the container's) between saving
/// and restoring its list state, outside mode 4; buttons 13 and 14 enter (`DoEnter`, slot
/// +0x10, ids 0x15 and 0x16) the other list when the list's slot +4 is 0 and it has items.
/// True when handled.
pub fn container_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    let base = this.addr();
    if !e.call(IS_TOP_MENU, &args![CONTAINER_MENU_CLASS]).bool() {
        return false;
    }
    match button {
        3 | 4 => {
            if e.get(this, ContainerMenu::eContainerMode) == 4 {
                return false;
            }
            if e.get(this, ContainerMenu::plbCurrListbox) == base.wrapping_add(ITEMS) {
                e.call(LIST_SAVE_STATE, &args![base.wrapping_add(ITEMS)]);
                let id = if button != 4 { 2u32 } else { 0 };
                e.vcall(base, 0xc, &args![id, 0u32]);
                e.call(LIST_RESTORE_STATE, &args![base.wrapping_add(ITEMS), 0u32]);
            } else {
                e.call(LIST_SAVE_STATE, &args![base.wrapping_add(CONTAINER)]);
                let id = if button != 4 { 7u32 } else { 5 };
                e.vcall(base, 0xc, &args![id, 0u32]);
                e.call(
                    LIST_RESTORE_STATE,
                    &args![base.wrapping_add(CONTAINER), 0u32],
                );
            }
            true
        }
        0xd | 0xe => {
            let (from, to, id) = if button == 0xd {
                (ITEMS, CONTAINER, 0x15u32)
            } else {
                (CONTAINER, ITEMS, 0x16)
            };
            if e.vcall(base.wrapping_add(from), LIST_BOX_SLOT_4, &args![])
                .u32()
                == 0
                && e.call(LIST_COUNT, &args![base.wrapping_add(from)]).u32() != 0
            {
                e.call(LIST_SAVE_STATE, &args![base.wrapping_add(to)]);
                e.vcall(base, 0x10, &args![id, 0u32]);
                e.call(LIST_RESTORE_STATE, &args![base.wrapping_add(from), 1u32]);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

// Translated from 0075d430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoGamepadUpEvent` (Xbox PDB; `thiscall`, `RET 8`: button, an unread
/// word): button 11 clicks id 10 (take all) outside modes 3 and 4 while the menu is shown
/// (fade state +0x24 is 1); button 10 closes the menu. True when handled.
pub fn container_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    let mode = e.get(this, ContainerMenu::eContainerMode);
    if button == 0xb && mode != 4 && mode != 3 && e.mem.u32(this.addr().wrapping_add(0x24)) == 1 {
        e.vcall(this.addr(), 0xc, &args![10u32, 0u32]);
        true
    } else if button == 10 {
        container_menu_close(e);
        true
    } else {
        false
    }
}

// Translated from 0075d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DoWheelMove` (Xbox PDB; `thiscall`, `RET 8`: wheel id, an unread word): for
/// ids 1 and 6, clicks (`DoClick`) the list's previous or next filter button: 0 or 5 when the
/// interface's wheel value (`0070ec70`) is negative, 2 or 7 otherwise.
pub fn container_menu_do_wheel_move(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    id: i32,
    _unused_2: u32,
) {
    if id != 1 && id != 6 {
        return;
    }
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let direction = e.call(WHEEL_DIRECTION_0070EC70, &args![interface]).i32();
    let first = if id != 1 { 5u32 } else { 0 };
    let button = if direction < 0 { first } else { first + 2 };
    e.vcall(this.addr(), 0xc, &args![button, 0u32]);
}

// Translated from 0075da60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::TakeAllFn` (Xbox PDB; `cdecl`, the list box callback: tile, item): unless
/// a pickpocket already failed (+0x89), moves one item: an item dropped by someone who is not
/// the player and not a follower goes back through the player's slot +0x3d0 (dropper, 1, 0);
/// an item whose tile's 0xfa9 is 128 stays. While pickpocketing (mode 2) the stolen value
/// (`GetStolenValue` of the whole stack) is added up and checked (`PickpocketCheck`), a
/// failure setting +0x89. The item then moves through the container's slot +0x17c (object,
/// first extra list, count, stolen flag, 0, player, 0, 0, 1, 0).
pub fn container_menu_take_all_fn(e: &mut Engine, tile: u32, item: u32) {
    if item == 0 || me(e) == 0 || tile == 0 || e.mem.u8(me(e).wrapping_add(0x89)) != 0 {
        return;
    }
    let extra = first_extra_list(e, item);
    if extra != 0 && e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32() != 0 {
        let dropper = e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32();
        if !e.call(FORM_TEST_00440DA0, &args![dropper]).bool() {
            let dropper = e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32();
            if !e.call(FORM_TEST_00440D80, &args![dropper]).bool() {
                let dropper = e.call(EXTRA_GET_ITEM_DROPPER, &args![extra]).u32();
                let player = player_actor(e);
                e.vcall(player, 0x3d0, &args![dropper, 1u32, 0u32]);
                return;
            }
        }
    }
    let shown = item_tile(e, tile);
    let alpha = e.call(GET_FLOAT_TRAIT, &args![shown, TRAIT_0XFA9]).f64();
    // `FCOMP; TEST AH,0x44; JP`: the item stays when the value equals 128.
    if alpha == e.global::<f64>(DISABLED_VALUE) {
        return;
    }
    let menu = menu(e);
    if e.get(menu, ContainerMenu::eContainerMode) == 2 {
        let reference = e.get(menu, ContainerMenu::pContainerRef);
        let actor = if is_actor(e, reference) {
            e.get(menu, ContainerMenu::pContainerRef)
        } else {
            0
        };
        let count = e.call(WORD_AT_FOUR, &args![item]).i32();
        let value = container_menu_get_stolen_value(e, menu, item, count, 1);
        let total = e.get(menu, ContainerMenu::iValueStolen).wrapping_add(value);
        e.set(menu, ContainerMenu::iValueStolen, total);
        let passed = container_menu_pickpocket_check(e, menu, actor, value);
        e.set(menu, ContainerMenu::bPickpocketFail, u8::from(!passed));
    }
    let extra = first_extra_list(e, item);
    let stolen = u32::from(e.get(menu, ContainerMenu::eContainerMode) == 2);
    let count = e.call(WORD_AT_FOUR, &args![item]).u32();
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    let player = player_actor(e);
    let container = e.get(menu, ContainerMenu::pContainerRef);
    e.vcall(
        container,
        0x17c,
        &args![object, extra, count, stolen, 0u32, player, 0u32, 0u32, 1u32, 0u32],
    );
}

// Translated from 0075e0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::PickpocketCheck` (Xbox PDB; `thiscall`, `RET 8`: target, value): true
/// without a target, for a value of 0 or less, or when the target's slot +0x230 says so.
/// Otherwise the chance (`00643400` of the player's clamped actor value 0x2a, the target's
/// actor value 0x2a and the value as a float) is rolled against `BSRandom::UnsignedInt(100)`
/// after the player's slot +0x47c (0x2a, 1): a success counts the misc stat 13 once (when the
/// container list is current); a failure makes the player react (`008c00e0`), shows the
/// caught message and sets +0x89. Either way, unless the target's karma alignment is 2 or 4,
/// the player is given the karma setting.
pub fn container_menu_pickpocket_check(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    target: u32,
    value: i32,
) -> bool {
    if target == 0 || value <= 0 || e.vcall(target, 0x230, &args![]).bool() {
        return true;
    }
    let mut passed = true;
    let amount = x87_store32(f64::from(value));
    let target_value = e.vcall(target.wrapping_add(0xa4), 8, &args![0x2au32]).u32();
    let player = player_actor(e);
    let player_value = e
        .call(
            GET_CLAMPED_ACTOR_VALUE,
            &args![player.wrapping_add(0xa4), 0x2au32],
        )
        .u32();
    let chance = e
        .call(
            PICKPOCKET_CHANCE_00643400,
            &args![player_value, target_value, amount],
        )
        .u32();
    let player = player_actor(e);
    e.vcall(player, 0x47c, &args![0x2au32, 1u32]);
    let random = e.call(RANDOM, &args![]).u32();
    let roll = e.call(RANDOM_UNSIGNED, &args![random, 100u32]).u32();
    if roll < chance {
        if e.get(this, ContainerMenu::plbCurrListbox) == this.addr().wrapping_add(CONTAINER)
            && e.get(this, ContainerMenu::bPickPocketTallied) == 0
        {
            e.call(MISC_STAT_INCREMENT, &args![0xdu32]);
            e.set(this, ContainerMenu::bPickPocketTallied, 1);
        }
    } else {
        let player = player_actor(e);
        e.call(PLAYER_CAUGHT_008C00E0, &args![player, target, 0u32, 0u32]);
        let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
        let text = e.call(SETTING_STRING, &args![SETTING_CAUGHT_MESSAGE]).u32();
        e.call(SHOW_MESSAGE, &args![text, 2u32, 0u32, 0u32, time, 0u32]);
        passed = false;
        e.set(this, ContainerMenu::bPickpocketFail, 1);
    }
    e.call(REFERENCE_BASE_DATA_004181E0, &args![target]);
    let base = e
        .call(REFERENCE_BASE_DATA_004181E0, &args![target])
        .u32()
        .wrapping_add(0x100);
    let karma = e.vcall(base, 0xc, &args![0x17u32]).f64();
    let alignment = e
        .call(ALIGNMENT_FOR_KARMA, &args![x87_store32(karma)])
        .u32();
    if alignment != 2 && alignment != 4 {
        let setting = e.call(SETTING_FLOAT_ADDRESS, &args![SETTING_KARMA]).u32();
        let reward = x87_load32(e.mem.u32(setting));
        let reward = e.call(FTOL2, &args![reward]).u32();
        let player = player_actor(e);
        e.call(REWARD_KARMA, &args![player, reward]);
    }
    passed
}

// Translated from 0075e240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::GetStolenValue` (Xbox PDB; `thiscall`, `RET 0xc`: item, count, whether
/// unowned items count): the item's value (`GetItemValue`, through `00406d90`) times the
/// count of each of its extra lists (as `i16`) whose owner word (`00418660`) is the
/// container's owner (the container itself for an actor, else `TESObjectREFR::GetOwner`), or
/// unset when unowned items count; the count left after the lists counts too when unowned
/// items do.
pub fn container_menu_get_stolen_value(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    item: u32,
    count: i32,
    unowned: u8,
) -> i32 {
    let value = e.call(ITEM_GET_VALUE, &args![item]).f64();
    let value = e.call(FLOAT_TO_INT, &args![x87_store32(value)]).i32();
    let mut left = count;
    let mut total = 0i32;
    let reference = e.get(this, ContainerMenu::pContainerRef);
    let owner = if is_actor(e, reference) {
        e.get(this, ContainerMenu::pContainerRef)
    } else {
        let reference = e.get(this, ContainerMenu::pContainerRef);
        e.call(REFERENCE_OWNER, &args![reference]).u32()
    };
    let mut node = e.call(FIRST_WORD, &args![item]).u32();
    while left > 0 && node != 0 {
        let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
        let extra = e.mem.u32(slot);
        let extra_owner = e.call(EXTRA_OWNER_00418660, &args![extra]).u32();
        let counts = if extra_owner == 0 && unowned != 0 {
            true
        } else if extra_owner != 0 {
            let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
            let extra = e.mem.u32(slot);
            e.call(EXTRA_OWNER_00418660, &args![extra]).u32() == owner
        } else {
            false
        };
        if counts {
            let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
            let extra = e.mem.u32(slot);
            let stack = i32::from(e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16);
            total = stack.wrapping_mul(value).wrapping_add(total);
        }
        let slot = e.call(FIRST_WORD_ADDRESS, &args![node]).u32();
        let extra = e.mem.u32(slot);
        let stack = i32::from(e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16);
        left = left.wrapping_sub(stack);
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
    }
    if unowned != 0 {
        total = left.wrapping_mul(value).wrapping_add(total);
    }
    total
}

// Translated from 0075ec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::PlayReactivityBark` (Xbox PDB; `thiscall`, `RET 4`: topic): finds the
/// topic's info for the container actor and the player and builds a `DialogueItem` on the
/// stack. With a response, the bark sound (`m_hCurrentReactivityBark`, +0x100) becomes the
/// actor's sound for the response's voice file (when it has one), or the actor speaks it
/// through its slot +0x284 (13 words; its float result is not used); then the subtitle shows
/// (`DisplayContainerMenuSubtitle`, given a copy of the response text and of the bark sound
/// by value).
// Translation incomplete: 0075ec60 - the C++ exception frame (FS:[0], unwind states for the dialogue item and the temporary handles) is not translated.
pub fn container_menu_play_reactivity_bark(e: &mut Engine, this: Ptr<ContainerMenu>, topic: u32) {
    use crate::units::fallout_dialogue::dialogueitem::DialogueItem;
    let base = this.addr();
    let player = player_actor(e);
    let actor = container_actor(e);
    let info = if topic != 0 {
        e.call(TOPIC_GET_MATCHING_INFO, &args![topic, actor, player])
            .u32()
    } else {
        0
    };
    if info == 0 {
        return;
    }
    let quest = e.call(INFO_QUEST_006733E0, &args![info]).u32();
    e.with_stack(DialogueItem::SIZE, |e, dialogue| {
        e.call(
            DIALOGUE_ITEM_CONSTRUCT,
            &args![dialogue, quest, topic, info, actor],
        );
        e.call(DIALOGUE_ITEM_FIRST_RESPONSE, &args![dialogue]);
        let response = e
            .call(DIALOGUE_ITEM_CURRENT_RESPONSE, &args![dialogue])
            .u32();
        if response != 0 {
            if e.call(FORM_OF, &args![response]).u32() != 0 {
                let voice = e.call(FORM_OF, &args![response]).u32();
                let file = e.call(VALUE_STRING_0084E3A0, &args![voice]).u32();
                e.with_stack(0xc, |e, handle| {
                    let sound = e
                        .call(
                            ACTOR_VOICE_SOUND_00933150,
                            &args![actor, handle, file, 0u32, 0x102u32, 1u32],
                        )
                        .u32();
                    e.call(
                        SOUND_HANDLE_ASSIGN,
                        &args![base.wrapping_add(REACTIVITY_BARK), sound],
                    );
                    e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                });
            } else {
                let first = e.call(RESPONSE_00441110, &args![response]).u32();
                let second = e.call(RESPONSE_009611E0, &args![response]).u32();
                let field = e.call(FIRST_WORD_ADDRESS, &args![response]).u32();
                let third = e.call(STRING_004048E0, &args![field]).u32();
                let fourth = e.call(VALUE_STRING_0084E3A0, &args![response]).u32();
                let fifth = e.call(ITEM_OBJECT, &args![response]).u32();
                let holder = e.call(RESPONSE_00460140, &args![response]).u32();
                let sixth = e.call(FIRST_WORD, &args![holder]).u32();
                e.vcall(
                    actor,
                    0x284,
                    &args![
                        sixth,
                        base.wrapping_add(REACTIVITY_BARK),
                        fifth,
                        fourth,
                        third,
                        second,
                        first,
                        player,
                        1u32,
                        1u32,
                        0u32,
                        0u32,
                        1u32
                    ],
                );
            }
            // The two by-value arguments are built where the call's arguments go: the sound
            // handle copy (`00418900`) at +8, the string copy (`004047f0`) at +0.
            let words = e.with_stack(0x14, |e, arguments| {
                e.call(
                    SOUND_HANDLE_ASSIGN,
                    &args![
                        arguments.addr().wrapping_add(8),
                        base.wrapping_add(REACTIVITY_BARK)
                    ],
                );
                let text = e.call(FIRST_WORD_ADDRESS, &args![response]).u32();
                e.call(STRING_COPY_CONSTRUCT, &args![arguments, text]);
                let mut words = [0u32; 5];
                for (i, word) in words.iter_mut().enumerate() {
                    *word = e.mem.u32(arguments.addr().wrapping_add(4 * i as u32));
                }
                words
            });
            container_menu_display_container_menu_subtitle(
                e, this, words[0], words[1], words[2], words[3], words[4],
            );
        }
        e.call(DIALOGUE_ITEM_DESTRUCT, &args![dialogue]);
    });
}

// Translated from 0075eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::DisplayContainerMenuSubtitle` (Xbox PDB; `thiscall`, `RET 0x14`: a
/// `BSStringT` and a `BSSoundHandle` by value, five words): for a text that is not empty, when
/// subtitles are on (the setting's text at `011d8928` is not empty) or the container actor
/// always shows them, writes the text into the `CM_Subtitle` tile and sets the time to clear
/// it (`m_nTimeToClearSubtitle`, +0xfc): now plus the sound's duration, or, without one, the
/// text's length times the subtitle speed setting times 1000 (`FISTP qword` with chop). Then
/// destroys the two by-value arguments.
// Translation incomplete: 0075eea0 - the C++ exception frame (FS:[0], unwind states for the by-value arguments) is not translated.
pub fn container_menu_display_container_menu_subtitle(
    e: &mut Engine,
    this: Ptr<ContainerMenu>,
    text_0: u32,
    text_1: u32,
    sound_0: u32,
    sound_1: u32,
    sound_2: u32,
) {
    e.with_stack(0x14, |e, arguments| {
        let text = arguments.addr();
        let sound = text.wrapping_add(8);
        for (i, word) in [text_0, text_1, sound_0, sound_1, sound_2]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(text.wrapping_add(4 * i as u32), word);
        }
        let actor = container_actor(e);
        if e.call(STRING_004048E0, &args![text]).u32() != 0 {
            let subtitles = e
                .call(SUBTITLES_STRING_00408D60, &args![SUBTITLES_SETTING])
                .u32();
            let shown = e.mem.u8(subtitles) != 0
                || (actor != 0 && e.call(ALWAYS_SHOW_SUBTITLES, &args![actor]).bool());
            if shown {
                let menu_tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
                let tile = e
                    .call(TILE_FIND_CHILD, &args![menu_tile, SUBTITLE_TILE])
                    .u32();
                if tile != 0 {
                    let string = e.call(FIRST_WORD, &args![text]).u32();
                    set_string(e, tile, TRAIT_STRING, string);
                }
                let duration = e.call(SOUND_HANDLE_GET_DURATION, &args![sound]).i32();
                let mut time = x87_store32(f64::from(duration));
                // `FCOMP; TEST AH,0x44; JP`: the length counts only when the duration is 0.
                if x87_m32(time) == e.global::<f64>(ZERO_DOUBLE) {
                    let string = e.call(FIRST_WORD, &args![text]).u32();
                    let length = e.call(STRLEN, &args![string]).u32();
                    let speed = e
                        .call(SETTING_FLOAT_ADDRESS, &args![SETTING_SUBTITLE_SPEED])
                        .u32();
                    let speed = x87_load32(e.mem.u32(speed));
                    let product = x87_mul(speed, f64::from(length));
                    time = x87_store32(x87_mul(product, e.global::<f64>(THOUSAND_01017B70)));
                }
                let now = e.call(CURRENT_TIME, &args![]).u32();
                let clear = x87_add(f64::from(now), x87_m32(time));
                let clear = x87_fistp64_chop(X87Ext::load64(clear)) as u32;
                e.set(this, ContainerMenu::m_nTimeToClearSubtitle, clear);
            }
        }
        e.call(STRING_DESTROY, &args![text]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![sound]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0075_cc70, fn_0075cc70(Ptr) -> u32),
        entry!(
            0x0075_d250,
            container_menu_do_gamepad(Ptr<ContainerMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0075_d430,
            container_menu_do_gamepad_up_event(Ptr<ContainerMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0075_d4a0,
            container_menu_do_wheel_move(Ptr<ContainerMenu>, i32, u32)
        ),
        entry!(0x0075_da60, container_menu_take_all_fn(u32, u32)),
        entry!(
            0x0075_e0b0,
            container_menu_pickpocket_check(Ptr<ContainerMenu>, u32, i32) -> bool
        ),
        entry!(
            0x0075_e240,
            container_menu_get_stolen_value(Ptr<ContainerMenu>, u32, i32, u8) -> i32
        ),
        entry!(
            0x0075_ec60,
            container_menu_play_reactivity_bark(Ptr<ContainerMenu>, u32)
        ),
        entry!(
            0x0075_eea0,
            container_menu_display_container_menu_subtitle(
                Ptr<ContainerMenu>,
                u32,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x0075_b0d0,
            container_menu_container_menu(Ptr<ContainerMenu>) -> Ptr<ContainerMenu>
        ),
        entry!(
            0x0075_b200,
            container_menu_get_class(Ptr<ContainerMenu>) -> u32
        ),
        entry!(
            0x0075_b210,
            container_menu_attach_tile_by_id(Ptr<ContainerMenu>, i32, u32)
        ),
        entry!(
            0x0075_b240,
            container_menu_scalar_deleting_destructor(
                Ptr<ContainerMenu>,
                u32,
            ) -> Ptr<ContainerMenu>
        ),
        entry!(0x0075_b270, fn_0075b270(Ptr<ContainerMenu>)),
        entry!(0x0075_b310, container_menu_create(u32, i32) -> u32),
        entry!(0x0075_b6e0, fn_0075b6e0(u8)),
        entry!(0x0075_b700, fn_0075b700() -> u32),
        entry!(0x0075_b710, fn_0075b710(Ptr<ContainerMenu>) -> bool),
        entry!(0x0075_b750, container_menu_close()),
        entry!(
            0x0075_baf0,
            container_menu_play_container_sound(Ptr<ContainerMenu>, u32, u8)
        ),
        entry!(
            0x0075_bc80,
            container_menu_init_container_mode(Ptr<ContainerMenu>)
        ),
        entry!(
            0x0075_be80,
            container_menu_do_click(Ptr<ContainerMenu>, i32, u32)
        ),
        entry!(
            0x0075_c250,
            container_menu_do_key(Ptr<ContainerMenu>, i32) -> bool
        ),
        entry!(
            0x0075_c280,
            container_menu_update_lists(Ptr<ContainerMenu>, u32)
        ),
        entry!(
            0x0075_cc90,
            container_menu_update_item_in_list(Ptr<ContainerMenu>, u32, u32, u32)
        ),
        entry!(
            0x0075_cd70,
            container_menu_do_enter(Ptr<ContainerMenu>, i32, u32)
        ),
        entry!(
            0x0075_cf70,
            container_menu_do_leave(Ptr<ContainerMenu>, u32, u32)
        ),
        entry!(0x0075_cfc0, container_menu_first_pass_filter_func(u32) -> bool),
        entry!(0x0075_d130, fn_0075d130(u32, u32) -> i32),
        entry!(0x0075_d160, container_menu_init_func(u32, u32)),
        entry!(
            0x0075_d510,
            fn_0075d510(Ptr<ContainerMenu>, u32, u32, u32) -> bool
        ),
        entry!(
            0x0075_d6b0,
            container_menu_switch_container(Ptr<ContainerMenu>, u32, u32, i32)
        ),
        entry!(0x0075_dc80, container_menu_item_select_callback(i32)),
        entry!(
            0x0075_e380,
            container_menu_set_filter_mode_title(Ptr<ContainerMenu>, u32)
        ),
        entry!(0x0075_e650, container_menu_filter_mode_func(u32) -> bool),
        entry!(
            0x0075_e8c0,
            container_menu_set_container_title(Ptr<ContainerMenu>, u32)
        ),
        entry!(0x0075_eac0, container_menu_do_idle(Ptr<ContainerMenu>)),
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

    /// Doubles at `0x00c1_0000 + slot` for an object's vtable slots.
    fn object_with_vtable(e: &mut Engine, size: u32, base: u32) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x500);
        e.mem.set_u32(object, vtable);
        for slot in (0..0x500).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        object
    }

    fn globals(e: &mut Engine) {
        for page in [
            CONTAINER_MENU,
            ITEM_ENTERED,
            FLAG_HOLDER,
            PLAYER,
            0x0101_2000,
            0x0101_6000,
            0x0101_e000,
            0x0102_1000,
            0x0102_e000,
            0x0103_5000,
            0x0107_0000,
            0x0107_2000,
        ] {
            e.map(page & !0xfff, 0x1000);
        }
    }

    /// A menu set as `pMe`, its tiles at `0x100 + 4 * i`, its vtable slots
    /// at `0x00c0_0000 + slot` and its lists' slots at `0x00c2_0000 + slot`
    /// (items) and `0x00c3_0000 + slot` (container).
    fn container(e: &mut Engine) -> Ptr<ContainerMenu> {
        globals(e);
        let this = object_with_vtable(e, 0x10c, 0x00c0_0000);
        for i in 0..19 {
            e.mem.set_u32(this + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        let items = e.mem.alloc(0x500);
        e.mem.set_u32(this + 0x98, items);
        for slot in (0..0x500).step_by(4) {
            e.mem.set_u32(items + slot, 0x00c2_0000 + slot);
        }
        let list = e.mem.alloc(0x500);
        e.mem.set_u32(this + 0xc8, list);
        for slot in (0..0x500).step_by(4) {
            e.mem.set_u32(list + slot, 0x00c3_0000 + slot);
        }
        e.set_global(CONTAINER_MENU, this);
        Ptr::new(this)
    }

    /// The tile at `offset` of [`container`]'s menu.
    fn tile(offset: u32) -> u32 {
        0x100 + offset - 0x28
    }

    #[test]
    fn constructor_builds_the_parts() {
        let mut e = Engine::new();
        globals(&mut e);
        e.set_global(CURRENT_ITEM, 5u32);
        stub(
            &mut e,
            &[
                MENU_CONSTRUCTOR,
                SIMPLE_LIST_CONSTRUCTOR,
                LIST_BOX_CONSTRUCTOR,
                SOUND_HANDLE_CONSTRUCTOR,
                SOUND_HANDLE_FROM_WORD,
                SOUND_HANDLE_DESTRUCTOR,
            ],
        );
        stub(&mut e, &[SOUND_HANDLE_ASSIGN]);
        let this = e.new_object::<ContainerMenu>();
        e.set(this, ContainerMenu::eFilterSound, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_b0d0, &args![this]).u32(), this.addr());
        let me_addr = this.addr();
        assert_eq!(e.mem.u32(me_addr), CONTAINER_MENU_VTABLE);
        assert_eq!(
            calls_to(&e, LIST_BOX_CONSTRUCTOR),
            vec![vec![me_addr + 0x98], vec![me_addr + 0xc8]]
        );
        assert_eq!(e.global::<u32>(CURRENT_ITEM), 0);
        assert_eq!(e.get(this, ContainerMenu::eFilterSound), 3);
        assert_eq!(calls_to(&e, SOUND_HANDLE_ASSIGN)[0][0], me_addr + 0x100);
        assert_eq!(calls_to(&e, SOUND_HANDLE_FROM_WORD)[0][1], 0);
    }

    #[test]
    fn class_tiles_and_deleting_destructor() {
        let mut e = Engine::new();
        let this = container(&mut e);
        assert_eq!(e.call(0x0075_b200, &args![this]).u32(), 0x3f0);
        assert_eq!(e.call(0x0075_b700, &args![]).u32(), 0x3f0);
        assert!(e.call(0x0075_b710, &args![this]).bool());
        e.call(0x0075_b210, &args![this, 18i32, 0u32]);
        assert!(!e.call(0x0075_b710, &args![this]).bool());
        e.call(0x0075_b210, &args![this, 19i32, 0x55u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x74), 0);
        stub(
            &mut e,
            &[
                SOUND_HANDLE_DESTRUCTOR,
                LIST_BOX_DESTRUCTOR,
                SIMPLE_LIST_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_b240, &args![this, 1u32]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x0075_b240,
                SOUND_HANDLE_DESTRUCTOR,
                LIST_BOX_DESTRUCTOR,
                LIST_BOX_DESTRUCTOR,
                SIMPLE_LIST_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE
            ]
        );
        assert_eq!(e.global::<u32>(CONTAINER_MENU), 0);
        e.call_log = Some(vec![]);
        e.call(0x0075_b270, &args![this]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn flag_holder_byte() {
        let mut e = Engine::new();
        globals(&mut e);
        e.call(0x0075_b6e0, &args![1u32]);
        let holder = e.mem.alloc(0x500);
        e.set_global(FLAG_HOLDER, holder);
        e.call(0x0075_b6e0, &args![1u32]);
        assert_eq!(e.mem.u8(holder + 0x4cd), 1);
    }

    #[test]
    fn create_needs_a_reference_except_in_mode_4() {
        let mut e = Engine::new();
        globals(&mut e);
        stub(&mut e, &[INTERFACE_SINGLETON]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_b310, &args![0u32, 1i32]).u32(), 0);
        assert!(calls_to(&e, TILE_GET_MENU_BY_CLASS).is_empty());
    }

    #[test]
    fn create_sets_up_the_menu() {
        let mut e = Engine::new();
        let this = container(&mut e);
        let me_addr = this.addr();
        let vtable = e.mem.u32(me_addr);
        e.mem.set_u32(vtable + 0x34, 0x0075_b200);
        e.set_global(STACKING_TYPE_ONE, 102.0f64);
        answer(&mut e, INTERFACE_SINGLETON, 0x1000);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, me_addr);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 102.0);
        answer_st0(&mut e, TILE_GET_MAXIMUM_DEPTH, 6.0);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_FIND_CHILD, 0x950);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x99);
        answer(&mut e, SETTING_STRING, 0x5000);
        // Mode 4 without a reference: InitContainerMode's mode-4 branch.
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0);
        // UpdateLists with no current list in mode 4.
        answer(&mut e, PLAYER_MOD_LIST, 0x6000);
        answer(&mut e, ITEM_OBJECT, 0);
        stub(
            &mut e,
            &[
                TILE_READER,
                MENU_SET_MENU_TILE,
                SET_FLOAT_TRAIT,
                SHOW_TUTORIAL,
                LIST_BOX_INITIALIZE,
                SET_STRING_TRAIT,
                SET_TRAIT_FROM_INT,
                MENU_PREP_FOR_VISIBILITY,
                0x00c3_001c,
                0x00c2_0020,
                0x00c3_0020,
                LIST_FILL,
                LIST_BOX_FOR_EACH,
                LIST_FILTER,
                FIRST_WORD_ADDRESS,
                PLAYER_WEIGHTS,
                ROUND_TO,
                FLOAT_TO_INT,
                PRINT_BOUNDED,
                GAMEPAD_IN_USE,
            ],
        );
        e.call_log = Some(vec![]);
        // The active-mine list is empty: its first node's item is 0.
        e.register(FIRST_WORD_ADDRESS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x0075_b310, &args![0u32, 4i32]).u32(), 0x4444);
        assert_eq!(e.get(this, ContainerMenu::eContainerMode), 4);
        assert_eq!(
            calls_to(&e, SHOW_TUTORIAL),
            vec![vec![0x1000 + 0x4d4, 0x16, 0x3f0, 0x200]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![
                vec![me_addr + 0x98, tile(0x38), ITEM_TEMPLATE_NAME],
                vec![me_addr + 0xc8, tile(0x48), ITEM_TEMPLATE_NAME]
            ]
        );
        let strings = calls_to(&e, SET_STRING_TRAIT);
        assert!(strings.contains(&vec![0x950, 0xfc4, EMPTY_TEXT, 1]));
        assert!(strings.contains(&vec![tile(0x60), 0x99, 0x5000, 1]));
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![me_addr, 0]]
        );
        assert_eq!(
            calls_to(&e, LIST_FILL).len(),
            1,
            "mode 4 fills the items list"
        );
    }

    #[test]
    fn create_reports_missing_tiles() {
        let mut e = Engine::new();
        let this = container(&mut e);
        let vtable = e.mem.u32(this.addr());
        e.mem.set_u32(vtable + 0x34, 0x0075_b200);
        e.mem.set_u32(this.addr() + 0x28, 0);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this.addr());
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_READER,
                MENU_SET_MENU_TILE,
                PRINT_MESSAGE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_b310, &args![0x2000u32, 1i32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    #[test]
    fn close_in_mode_1_plays_the_close_sound_and_fades_out() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::eContainerMode, 1);
        e.set(this, ContainerMenu::pContainerRef, 0);
        let process = object_with_vtable(&mut e, 0x10, 0x00c4_0000);
        answer(&mut e, ACTOR_PROCESS, process);
        answer(&mut e, 0x00c4_0474, 1);
        stub(
            &mut e,
            &[
                SOUND_HANDLE_FADE_OUT,
                SIMPLE_LIST_CLEAR,
                SET_TRAIT_FROM_INT,
                MENU_START_FADE_OUT,
                UPDATE_PLAYER_3D,
                HUD_REFRESH_INFO,
            ],
        );
        e.mem.set_u32(this.addr() + 4, 0x904);
        e.call_log = Some(vec![]);
        e.call(0x0075_b750, &args![]);
        let me_addr = this.addr();
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_FADE_OUT),
            vec![vec![me_addr + 0x100, 100]]
        );
        assert_eq!(calls_to(&e, SIMPLE_LIST_CLEAR), vec![vec![me_addr + 0x78]]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x904, 0x1772, 1]]
        );
        assert_eq!(calls_to(&e, 0x00c4_0474), vec![vec![process, 1]]);
        assert_eq!(calls_to(&e, UPDATE_PLAYER_3D).len(), 1);
        assert_eq!(calls_to(&e, HUD_REFRESH_INFO).len(), 1);
    }

    #[test]
    fn close_while_pickpocketing_raises_the_alarm() {
        let mut e = Engine::new();
        let this = container(&mut e);
        let reference = object_with_vtable(&mut e, 0x10, 0x00c5_0000);
        e.set(this, ContainerMenu::eContainerMode, 2);
        e.set(this, ContainerMenu::pContainerRef, reference);
        e.set(this, ContainerMenu::iValueStolen, 30);
        e.set_global(PLAYER, 0x3000u32);
        answer(&mut e, 0x00c5_0100, 0);
        answer(&mut e, REFERENCE_OWNER, 0x77);
        answer(&mut e, FORM_OF, 0x2100);
        answer(&mut e, FORM_TYPE, 0x10);
        let process = object_with_vtable(&mut e, 0x10, 0x00c4_0000);
        answer(&mut e, ACTOR_PROCESS, process);
        stub(
            &mut e,
            &[
                SOUND_HANDLE_FADE_OUT,
                STEAL_ALARM,
                SIMPLE_LIST_CLEAR,
                SET_TRAIT_FROM_INT,
                MENU_START_FADE_OUT,
                HUD_REFRESH_INFO,
                0x00c4_0474,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_b750, &args![]);
        assert_eq!(
            calls_to(&e, STEAL_ALARM),
            vec![vec![0x3000, reference, 0, 0, 30, 0x77]]
        );
    }

    #[test]
    fn play_container_sound_by_form_or_by_name() {
        let mut e = Engine::new();
        let this = container(&mut e);
        answer(&mut e, FORM_OF, 0x2100);
        answer(&mut e, FORM_TYPE, 0x1b);
        answer(&mut e, CONTAINER_OPEN_SOUND, 0x2200);
        answer(&mut e, SOUND_FILE_NAME, 0x2300);
        answer(&mut e, AUDIO_INSTANCE, 0x2400);
        answer(&mut e, AUDIO_SOUND_BY_FILE, 0x2500);
        answer(&mut e, AUDIO_SOUND_BY_NAME, 0x2600);
        stub(&mut e, &[SOUND_HANDLE_PLAY, SOUND_HANDLE_DESTRUCTOR]);
        e.call_log = Some(vec![]);
        e.call(0x0075_baf0, &args![this, 0x2000u32, 1u32]);
        let by_file = calls_to(&e, AUDIO_SOUND_BY_FILE);
        assert_eq!(by_file.len(), 1);
        assert_eq!(by_file[0][0], 0x2400);
        assert_eq!(&by_file[0][2..], &[0x2300, 0x121, 0x2200]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY), vec![vec![0x2500, 0]]);
        answer(&mut e, FORM_TYPE, 0x2a);
        e.call_log = Some(vec![]);
        e.call(0x0075_baf0, &args![this, 0x2000u32, 0u32]);
        let by_name = calls_to(&e, AUDIO_SOUND_BY_NAME);
        assert_eq!(&by_name[0][2..], &[CLOSE_SOUND_NAME, 0x121]);
        assert!(calls_to(&e, AUDIO_SOUND_BY_FILE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0075_baf0, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn init_container_mode_4_titles_and_hides() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::eContainerMode, 4);
        answer(&mut e, SETTING_STRING, 0x5000);
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0);
        stub(&mut e, &[SET_STRING_TRAIT, SET_TRAIT_FROM_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0075_bc80, &args![this]);
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![tile(0x2c), 0xfc4, 0x5000, 1]]
        );
        assert_eq!(calls_to(&e, SET_TRAIT_FROM_INT).len(), 5);
        assert_eq!(calls_to(&e, SETTING_STRING), vec![vec![0x011d_3c40]]);
    }

    #[test]
    fn init_container_mode_3_without_a_reference_hides_one_tile() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::eContainerMode, 3);
        stub(&mut e, &[SET_TRAIT_FROM_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0075_bc80, &args![this]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![tile(0x50), 0xfa3, 0]]
        );
    }

    #[test]
    fn do_click_steps_the_items_filter_until_something_shows() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::pContainerRef, 0);
        e.set(this, ContainerMenu::eItemsFilterMode, 3);
        e.set(this, ContainerMenu::eFilterSound, 3);
        let mut asked = 0;
        e.register_double(LIST_NUM_UNFILTERED, move |_, _| {
            asked += 1;
            Ret {
                eax: u32::from(asked == 2),
                ..Ret::default()
            }
        });
        stub(
            &mut e,
            &[PLAY_MENU_SOUND, LIST_FILTER, SETTING_STRING, COMPARE_TEXT],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_be80, &args![this, 1i32, 0u32]);
        assert_eq!(e.get(this, ContainerMenu::eItemsFilterMode), 5);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![3]]);
        assert_eq!(calls_to(&e, LIST_FILTER).len(), 2);
    }

    #[test]
    fn do_click_take_all_and_close() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set_global(CONTAINER_MENU, 0u32);
        answer(&mut e, AUDIO_INSTANCE, 0x2400);
        answer(&mut e, AUDIO_SOUND_BY_NAME, 0x2600);
        stub(
            &mut e,
            &[
                LIST_BOX_FOR_EACH,
                SOUND_HANDLE_PLAY,
                SOUND_HANDLE_DESTRUCTOR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_be80, &args![this, 10i32, 0u32]);
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![this.addr() + 0xc8, TAKE_ALL_FUNC, 0, 0x7fff_ffff]]
        );
        assert_eq!(
            &calls_to(&e, AUDIO_SOUND_BY_NAME)[0][2..],
            &[TAKE_ALL_SOUND, 0x121]
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_be80, &args![this, 3i32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_click_on_a_big_stack_opens_the_quantity_menu() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        e.set_global(ITEM_ENTERED, 1u8);
        e.set(this, ContainerMenu::eContainerMode, 1);
        e.set(this, ContainerMenu::plbCurrListbox, this.addr() + 0x98);
        let setting = e.mem.alloc(4);
        e.mem.set_i32(setting, 1);
        answer(&mut e, SETTING_VALUE_ADDRESS, setting);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, IS_GOLD, 1);
        answer(&mut e, WORD_AT_FOUR, 5);
        stub(
            &mut e,
            &[LIST_SAVE_STATE, 0x00c2_0000, QUANTITY_MENU_CREATE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_be80, &args![this, 20i32, 0u32]);
        assert_eq!(e.global::<u8>(ITEM_ENTERED), 0);
        assert_eq!(
            calls_to(&e, LIST_SAVE_STATE),
            vec![vec![this.addr() + 0x98]]
        );
        assert_eq!(
            calls_to(&e, QUANTITY_MENU_CREATE),
            vec![vec![5, ITEM_SELECT_CALLBACK, 0x7fff_ffff]]
        );
    }

    #[test]
    fn do_key_closes_on_0x20() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set_global(CONTAINER_MENU, 0u32);
        assert!(e.call(0x0075_c250, &args![this, 0x20i32]).bool());
        assert!(!e.call(0x0075_c250, &args![this, 0x21i32]).bool());
    }

    #[test]
    fn update_lists_refills_both_lists_and_restores_the_current_one() {
        let mut e = Engine::new();
        let this = container(&mut e);
        let me_addr = this.addr();
        let reference = object_with_vtable(&mut e, 0x10, 0x00c5_0000);
        answer(&mut e, 0x00c5_0100, 0);
        e.set(this, ContainerMenu::eContainerMode, 1);
        e.set(this, ContainerMenu::pContainerRef, reference);
        e.set(this, ContainerMenu::plbCurrListbox, me_addr + 0x98);
        e.set_global(PLAYER, 0x3000u32);
        e.register(FIRST_WORD_ADDRESS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        answer(&mut e, 0x00c2_0004, 1);
        answer(&mut e, LIST_NUM_UNFILTERED, 3);
        answer(&mut e, SETTING_STRING, 0x5000);
        answer(&mut e, FLOAT_TO_INT, 12);
        stub(
            &mut e,
            &[
                LIST_SAVE_STATE,
                LIST_FILL,
                LIST_BOX_FOR_EACH,
                LIST_FILTER,
                0x00c2_0020,
                0x00c3_0020,
                PLAYER_WEIGHTS,
                ROUND_TO,
                PRINT_BOUNDED,
                SET_STRING_TRAIT,
                LIST_RESTORE_STATE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_c280, &args![this, 0u32]);
        assert_eq!(calls_to(&e, LIST_SAVE_STATE), vec![vec![me_addr + 0x98]]);
        assert_eq!(
            calls_to(&e, LIST_FILL),
            vec![
                vec![me_addr + 0x98, 0x3000, FIRST_PASS_FILTER, 0, 0, 0, 0],
                vec![me_addr + 0xc8, reference, FIRST_PASS_FILTER, 0, 0, 0, 0]
            ]
        );
        assert_eq!(calls_to(&e, LIST_FILTER).len(), 2);
        let print = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(&print[0][1..], &[0x40, WEIGHT_FORMAT, 0x5000, 12, 12]);
        assert_eq!(
            calls_to(&e, LIST_RESTORE_STATE),
            vec![vec![me_addr + 0x98, 1]]
        );
    }

    #[test]
    fn update_item_in_list_re_adds_the_group() {
        let mut e = Engine::new();
        let this = container(&mut e);
        // The group list's first node holds one item.
        e.register(GET_ITEM_GROUP, |e, a| {
            e.mem.set_u32(a[2], 0x7000);
            Ret::default()
        });
        e.register(FIRST_WORD_ADDRESS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        answer(&mut e, ITEM_FULL_NAME, 0x7100);
        answer(&mut e, WORD_AT_FOUR, 0);
        stub(
            &mut e,
            &[
                SIMPLE_LIST_CONSTRUCTOR,
                LIST_REMOVE_ITEM_GROUP,
                LIST_ADD_ITEM,
                SIMPLE_LIST_CLEAR,
                SIMPLE_LIST_DESTRUCTOR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_cc90, &args![this, 0x2100u32, 0x2200u32, 0x2300u32]);
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ITEM_GROUP),
            vec![vec![0x2300, 0x2100]]
        );
        assert_eq!(
            calls_to(&e, LIST_ADD_ITEM),
            vec![vec![0x2300, 0x7000, 0x7100, 0, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_cc90, &args![this, 0x2100u32, 0u32, 0x2300u32]);
        assert!(calls_to(&e, GET_ITEM_GROUP).is_empty());
        assert_eq!(calls_to(&e, SIMPLE_LIST_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn do_enter_switches_lists_and_shows_the_item() {
        let mut e = Engine::new();
        let this = container(&mut e);
        let me_addr = this.addr();
        e.set(this, ContainerMenu::plbCurrListbox, me_addr + 0x98);
        stub(&mut e, &[0x00c0_0014, 0x00c3_0000]);
        e.call(0x0075_cd70, &args![this, 0x15i32, 0u32]);
        assert_eq!(e.get(this, ContainerMenu::plbCurrListbox), me_addr + 0x98);
        answer(&mut e, WORD_AT_0X28, tile(0x40));
        answer(&mut e, LIST_SELECTED_ITEM, 0x7000);
        answer(&mut e, ITEM_GET_ICON, 0x7300);
        stub(
            &mut e,
            &[POPULATE_ITEM_STATS, SET_TRAIT_FROM_INT, SET_STRING_TRAIT],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_cd70, &args![this, 0x14i32, 0x600u32]);
        assert_eq!(e.global::<u8>(ITEM_ENTERED), 1);
        assert_eq!(e.global::<u32>(CURRENT_ITEM), 0x7000);
        assert_eq!(
            calls_to(&e, POPULATE_ITEM_STATS),
            vec![vec![0x7000, tile(0x5c)]]
        );
        let ints = calls_to(&e, SET_TRAIT_FROM_INT);
        assert!(ints.contains(&vec![tile(0x4c), 0x1004, 1]));
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![tile(0x58), 0xfcc, 0x7300, 1]]
        );
    }

    #[test]
    fn do_leave_hides_three_tiles() {
        let mut e = Engine::new();
        let this = container(&mut e);
        stub(&mut e, &[SET_TRAIT_FROM_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0075_cf70, &args![this, 0u32, 0u32]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![
                vec![tile(0x5c), 0xfa3, 0],
                vec![tile(0x58), 0xfa3, 0],
                vec![tile(0x4c), 0xfa3, 0]
            ]
        );
    }

    #[test]
    fn first_pass_filter_keeps_playable_items() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::eContainerMode, 1);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, BIPED_MODEL_OF, 0);
        answer(&mut e, RT_DYNAMIC_CAST, 0x7200);
        answer(&mut e, FORM_TYPE, 0x10);
        answer(&mut e, FORM_TEST_0048CEE0, 1);
        assert!(!e.call(0x0075_cfc0, &args![0x7000u32]).bool());
        answer(&mut e, FORM_TEST_0048CEE0, 0);
        assert!(e.call(0x0075_cfc0, &args![0x7000u32]).bool());
        e.set(this, ContainerMenu::eContainerMode, 4);
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0x8000);
        answer(&mut e, MOD_LIST_OF, 0x8100);
        answer(&mut e, LIST_FORM_INDEX, u32::MAX);
        assert!(e.call(0x0075_cfc0, &args![0x7000u32]).bool());
        e.set_global(CONTAINER_MENU, 0u32);
        assert!(!e.call(0x0075_cfc0, &args![0x7000u32]).bool());
    }

    #[test]
    fn compare_reads_the_second_string_first() {
        let mut e = Engine::new();
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(first, 0x601);
        e.mem.set_u32(second, 0x602);
        e.register(TILE_GET_STRING, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        answer(&mut e, COMPARE_STRINGS, u32::MAX);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_d130, &args![first, second]).i32(), -1);
        assert_eq!(
            calls_to(&e, TILE_GET_STRING),
            vec![vec![0x602, 0xfc4], vec![0x601, 0xfc4]]
        );
        assert_eq!(calls_to(&e, COMPARE_STRINGS), vec![vec![0x1601, 0x1602]]);
    }

    #[test]
    fn init_func_marks_worn_items_and_counts() {
        let mut e = Engine::new();
        let marker = e.mem.alloc(4);
        e.mem.set_u32(marker, 0x880);
        answer(&mut e, ITEM_GET_WORN, 1);
        answer(&mut e, LIST_ITEM_TILE_INIT, marker);
        answer(&mut e, LIST_ITEM_TILE, marker);
        answer(&mut e, WORD_AT_FOUR, 3);
        answer(&mut e, ITEM_FULL_NAME, 0x7100);
        answer(&mut e, TILE_GET_STRING, 0x7200);
        stub(
            &mut e,
            &[
                SET_TRAIT_FROM_INT,
                PRINT_BOUNDED,
                SET_STRING_TRAIT,
                TILE_SET_TEXT,
                SET_FLOAT_TRAIT,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_d160, &args![0x600u32, 0x7000u32]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x880, 0xfa3, 1]]
        );
        let print = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(&print[0][1..], &[0x80, COUNT_FORMAT, 0x7100, 3]);
        assert_eq!(calls_to(&e, TILE_SET_TEXT), vec![vec![0x600, 0x7100]]);
        let strings = calls_to(&e, SET_STRING_TRAIT);
        assert_eq!(strings[1], vec![0x880, 0xfc4, 0x7200, 1]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x600, 0xfb0, 0, 1]]
        );
    }

    #[test]
    fn special_weapon_case_needs_pickpocketing() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set_global(PLAYER, 0x3000u32);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0075_d510, &args![this, 0x7000u32, 0x3000u32, 0x2000u32])
            .bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let from = object_with_vtable(&mut e, 0x10, 0x00c5_0000);
        e.set(this, ContainerMenu::eContainerMode, 2);
        e.set(this, ContainerMenu::plbCurrListbox, this.addr() + 0x98);
        answer(&mut e, PLAYER_TEST_004997B0, 1);
        answer(&mut e, 0x00c5_0100, 1);
        stub(&mut e, &[0x00c5_022c, 0x00c5_01a0]);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x28);
        answer(&mut e, LIST_TEST_008256D0, 1);
        answer(&mut e, ACTOR_GET_ESSENTIAL, 0);
        answer(&mut e, ACTOR_PROCESS, 0);
        answer(&mut e, OBJECT_TEST_004C0BF0, 1);
        answer(&mut e, WEAPON_AMMO_FOR, 0x7400);
        answer(&mut e, AMMO_FLAGS, 0x20000);
        assert!(e
            .call(0x0075_d510, &args![this, 0x7000u32, 0x3000u32, from])
            .bool());
        answer(&mut e, ACTOR_GET_ESSENTIAL, 1);
        assert!(!e
            .call(0x0075_d510, &args![this, 0x7000u32, 0x3000u32, from])
            .bool());
    }

    #[test]
    fn switch_container_moves_with_the_receivers_slot() {
        let mut e = Engine::new();
        let this = container(&mut e);
        let player = object_with_vtable(&mut e, 0x10, 0x00c6_0000);
        e.set_global(PLAYER, player);
        e.set(this, ContainerMenu::eContainerMode, 2);
        e.set(this, ContainerMenu::pContainerRef, 0x2000);
        e.set(this, ContainerMenu::plbCurrListbox, this.addr() + 0xc8);
        answer(&mut e, FIRST_WORD, 0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        stub(&mut e, &[0x00c6_017c]);
        // From the player to the container: `to` is the reference, whose
        // vtable the test gives slot +0x17c through a second object.
        let reference = object_with_vtable(&mut e, 0x10, 0x00c7_0000);
        e.set(this, ContainerMenu::pContainerRef, reference);
        stub(&mut e, &[0x00c7_017c]);
        e.call_log = Some(vec![]);
        e.call(0x0075_d6b0, &args![this, 0x600u32, 0x7000u32, 2i32]);
        assert_eq!(
            calls_to(&e, 0x00c7_017c),
            vec![vec![reference, 0x7100, 0, 2, 1, 0, player, 0, 0, 1, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_d6b0, &args![this, 0u32, 0x7000u32, 2i32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn item_select_callback_moves_the_item() {
        let mut e = Engine::new();
        let this = container(&mut e);
        let me_addr = this.addr();
        let player = object_with_vtable(&mut e, 0x10, 0x00c6_0000);
        e.set_global(PLAYER, player);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        e.set(this, ContainerMenu::eContainerMode, 1);
        e.set(this, ContainerMenu::plbCurrListbox, me_addr + 0x98);
        let tile_slot = e.mem.alloc(4);
        e.mem.set_u32(tile_slot, 0x880);
        answer(&mut e, LIST_GET_TILE_FOR, 0x600);
        answer(&mut e, LIST_ITEM_TILE, tile_slot);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 1.0);
        e.set_global(DISABLED_VALUE, 0.5f64);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        let process = object_with_vtable(&mut e, 0x10, 0x00c4_0000);
        answer(&mut e, ACTOR_PROCESS, process);
        answer(&mut e, 0x00c4_014c, 0);
        answer(&mut e, FIRST_WORD, 0);
        e.register(FIRST_WORD_ADDRESS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        // SwitchContainer and UpdateLists run for real (same unit).
        stub(
            &mut e,
            &[
                PLAYER_EQUIP_STATE,
                0x00c6_017c,
                0x00c2_0004,
                GET_INVENTORY_CHANGES,
                FORM_TYPE,
                SIMPLE_LIST_CONSTRUCTOR,
                SIMPLE_LIST_DESTRUCTOR,
                LIST_BOX_FOR_EACH,
                LIST_FILTER,
                0x00c2_0020,
                0x00c3_0020,
                PLAYER_WEIGHTS,
                ROUND_TO,
                FLOAT_TO_INT,
                SETTING_STRING,
                PRINT_BOUNDED,
                SET_STRING_TRAIT,
                LIST_NUM_UNFILTERED,
                0x00c0_000c,
                0x00c0_0010,
                GAMEPAD_IN_USE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_dc80, &args![3i32]);
        assert_eq!(
            calls_to(&e, LIST_GET_TILE_FOR),
            vec![vec![me_addr + 0x98, CURRENT_ITEM]]
        );
        assert_eq!(
            calls_to(&e, PLAYER_EQUIP_STATE),
            vec![vec![player, 0x7100, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x00c6_017c),
            vec![vec![player, 0x7100, 0, 3, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(calls_to(&e, LIST_FILTER).len(), 2, "the lists were updated");
    }

    #[test]
    fn item_select_callback_refuses_a_disabled_tile() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        e.set(this, ContainerMenu::eContainerMode, 1);
        let tile_slot = e.mem.alloc(4);
        answer(&mut e, LIST_GET_TILE_FOR, 0x600);
        answer(&mut e, LIST_ITEM_TILE, tile_slot);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 0.5);
        e.set_global(DISABLED_VALUE, 0.5f64);
        e.call_log = Some(vec![]);
        e.call(0x0075_dc80, &args![3i32]);
        assert!(calls_to(&e, ITEM_OBJECT).is_empty());
    }

    #[test]
    fn set_filter_mode_title_upper_cases_and_writes() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::pContainerRef, 0x2000);
        e.set(this, ContainerMenu::eContainerFilterMode, 2);
        answer(&mut e, SETTING_STRING, 0x5000);
        answer(&mut e, COMPARE_TEXT, 0);
        stub(&mut e, &[COPY_BOUNDED, TO_UPPER, SET_STRING_TRAIT]);
        e.call_log = Some(vec![]);
        e.call(0x0075_e380, &args![this, tile(0x40)]);
        assert_eq!(
            calls_to(&e, SETTING_STRING),
            vec![vec![0x011c_3cb4], vec![0x011d_4d50]]
        );
        let copy = calls_to(&e, COPY_BOUNDED);
        assert_eq!(&copy[0][1..], &[0x104, 0x5000]);
        assert_eq!(calls_to(&e, TO_UPPER), vec![vec![copy[0][0], 0x104]]);
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![tile(0x40), 0xfc4, copy[0][0], 1]]
        );
        e.set(this, ContainerMenu::pContainerRef, 0);
        e.call_log = Some(vec![]);
        e.call(0x0075_e380, &args![this, tile(0x40)]);
        assert!(calls_to(&e, SET_STRING_TRAIT).is_empty());
    }

    #[test]
    fn filter_mode_func_by_mode() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::eItemsFilterMode, 1);
        e.set(this, ContainerMenu::eContainerFilterMode, 5);
        answer(&mut e, ITEM_OBJECT, 0);
        answer(&mut e, FORM_TYPE, 0x28);
        e.register(LIST_GET_TILE_FOR, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x7000, "the item by address");
            Ret {
                eax: 0x600,
                ..Ret::default()
            }
        });
        // A weapon in the items list with the weapons filter: shown, unless
        // its object answers slot +0x94 true.
        let object = object_with_vtable(&mut e, 0x10, 0x00c8_0000);
        answer(&mut e, ITEM_OBJECT, object);
        answer(&mut e, 0x00c8_0094, 0);
        assert!(!e.call(0x0075_e650, &args![0x7000u32]).bool());
        answer(&mut e, 0x00c8_0094, 1);
        assert!(e.call(0x0075_e650, &args![0x7000u32]).bool());
        // In the container list with the ammo filter: a weapon is hidden.
        answer(&mut e, LIST_GET_TILE_FOR, 0);
        assert!(e.call(0x0075_e650, &args![0x7000u32]).bool());
    }

    #[test]
    fn set_container_title_fits_or_truncates() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set_global(TITLE_MARGIN, 10.0f64);
        e.set_global(TITLE_FONT_SCALE, 1.0f64);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"ABCDEFG");
        answer_st0(&mut e, GET_FLOAT_TRAIT, 40.0);
        answer(&mut e, FTOL, 2);
        answer(&mut e, FONT_MANAGER, 0x9000);
        e.register(CALCULATE_STRING_DIMENSIONS, |e, a| {
            e.mem.set_f32(a[1], 20.0);
            Ret::default()
        });
        answer(&mut e, WORD_AT_0X28, 0x700);
        answer(&mut e, TILE_IS_TRUE, 0);
        stub(&mut e, &[SET_STRING_TRAIT]);
        e.call_log = Some(vec![]);
        e.call(0x0075_e8c0, &args![this, text]);
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![tile(0x40), 0xfc4, text, 1]]
        );
        // 40 - 10 = 30 is below 50: the text is cut to the glyphs that fit.
        e.register(CALCULATE_STRING_DIMENSIONS, |e, a| {
            e.mem.set_f32(a[1], 50.0);
            Ret::default()
        });
        let font = e.mem.alloc(0x40);
        let glyphs = e.mem.alloc(0x2000);
        e.mem.set_u32(font + 0x38, glyphs - 0x128);
        // The three dots are 6 wide; each letter 10.
        e.mem.set_f32(glyphs + 0xa3c, 6.0);
        for letter in b'A'..=b'G' {
            e.mem
                .set_f32(glyphs + 0x38 * u32::from(letter) + 0x24, 10.0);
        }
        answer(&mut e, FONT_MANAGER_FONT, font);
        let buffer = e.mem.alloc(0x104);
        answer(&mut e, OPERATOR_NEW, buffer);
        e.register(MEMSET, |e, a| {
            assert_eq!(a[2], 0x104);
            e.mem.write(a[0], &[0u8; 0x104]);
            Ret::default()
        });
        stub(&mut e, &[APPEND_BOUNDED, OPERATOR_DELETE]);
        e.call_log = Some(vec![]);
        e.call(0x0075_e8c0, &args![this, text]);
        assert_eq!(calls_to(&e, FONT_MANAGER_FONT), vec![vec![0x9000, 1]]);
        // 6, 16, 26 fit 30; the third letter makes 36.
        assert_eq!(e.mem.cstr(buffer), b"ABC".to_vec());
        assert_eq!(
            calls_to(&e, APPEND_BOUNDED),
            vec![vec![buffer, 0x104, ELLIPSIS]]
        );
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![tile(0x40), 0xfc4, buffer, 1]]
        );
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![buffer]]);
    }

    #[test]
    fn do_idle_clears_the_subtitle_and_fills_the_card() {
        let mut e = Engine::new();
        let this = container(&mut e);
        e.set(this, ContainerMenu::m_nTimeToClearSubtitle, 100);
        e.set_global(ITEM_ENTERED, 1u8);
        e.set_global(CURRENT_ITEM, 0x7000u32);
        answer(&mut e, CURRENT_TIME, 101);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer(&mut e, TILE_FIND_CHILD, 0x950);
        answer(&mut e, LIST_COUNT, 2);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x28);
        stub(&mut e, &[SET_STRING_TRAIT, SET_ITEM_CARD]);
        e.call_log = Some(vec![]);
        e.call(0x0075_eac0, &args![this]);
        assert_eq!(e.get(this, ContainerMenu::m_nTimeToClearSubtitle), 0);
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![0x950, 0xfc4, EMPTY_TEXT, 1]]
        );
        assert_eq!(
            calls_to(&e, SET_ITEM_CARD),
            vec![vec![tile(0x60), tile(0x64), tile(0x68), 0x7000, 0]]
        );
        answer(&mut e, FORM_TYPE, 0x18);
        answer_st0(&mut e, OBJECT_VALUE_004BE180, 1.0);
        answer_st0(&mut e, OBJECT_VALUE_004BE060, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x0075_eac0, &args![this]);
        assert_eq!(
            calls_to(&e, SET_ITEM_CARD),
            vec![vec![0, tile(0x70), 0, 0x7000, 0]]
        );
    }
}

#[cfg(test)]
mod n053_tests {
    use super::*;

    /// The callees of this batch's functions outside this file, stubbed to return 0.
    const FOREIGN: &[u32] = &[
        IS_TOP_MENU,
        LIST_SAVE_STATE,
        LIST_RESTORE_STATE,
        LIST_COUNT,
        INTERFACE_SINGLETON,
        WHEEL_DIRECTION_0070EC70,
        FIRST_WORD,
        FIRST_WORD_ADDRESS,
        EXTRA_GET_ITEM_DROPPER,
        FORM_TEST_00440DA0,
        FORM_TEST_00440D80,
        LIST_ITEM_TILE,
        GET_FLOAT_TRAIT,
        WORD_AT_FOUR,
        ITEM_OBJECT,
        GET_CLAMPED_ACTOR_VALUE,
        PICKPOCKET_CHANCE_00643400,
        RANDOM,
        RANDOM_UNSIGNED,
        MISC_STAT_INCREMENT,
        PLAYER_CAUGHT_008C00E0,
        SETTING_STRING,
        SHOW_MESSAGE,
        REFERENCE_BASE_DATA_004181E0,
        ALIGNMENT_FOR_KARMA,
        SETTING_FLOAT_ADDRESS,
        REWARD_KARMA,
        ITEM_GET_VALUE,
        FLOAT_TO_INT,
        REFERENCE_OWNER,
        EXTRA_OWNER_00418660,
        EXTRA_GET_COUNT,
        RT_DYNAMIC_CAST,
        TOPIC_GET_MATCHING_INFO,
        INFO_QUEST_006733E0,
        DIALOGUE_ITEM_CONSTRUCT,
        DIALOGUE_ITEM_DESTRUCT,
        DIALOGUE_ITEM_FIRST_RESPONSE,
        DIALOGUE_ITEM_CURRENT_RESPONSE,
        FORM_OF,
        VALUE_STRING_0084E3A0,
        ACTOR_VOICE_SOUND_00933150,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_DESTRUCTOR,
        SOUND_HANDLE_FADE_OUT,
        RESPONSE_00441110,
        RESPONSE_009611E0,
        STRING_004048E0,
        RESPONSE_00460140,
        STRING_COPY_CONSTRUCT,
        SUBTITLES_STRING_00408D60,
        ALWAYS_SHOW_SUBTITLES,
        TILE_FIND_CHILD,
        SET_STRING_TRAIT,
        SOUND_HANDLE_GET_DURATION,
        STRLEN,
        CURRENT_TIME,
        STRING_DESTROY,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &callee in FOREIGN {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        for page in [
            0x0101_2000u32,
            0x0101_6000,
            0x0101_7000,
            0x0102_e000,
            0x011d_9000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(DISABLED_VALUE, 128.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(THOUSAND_01017B70, 1000.0f64);
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

    /// A menu with a vtable whose slots +0xc (`DoClick`) and +0x10 (`DoEnter`) are recorded,
    /// made `pMe`.
    fn new_menu(e: &mut Engine) -> u32 {
        e.put_vtable(0x0f10_0000, &[0, 0, 0, 0x0f10_0100, 0x0f10_0200]);
        e.register(0x0f10_0100, |_, _| Ret::default());
        e.register(0x0f10_0200, |_, _| Ret::default());
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this, 0x0f10_0000);
        e.set_global(CONTAINER_MENU, this);
        this
    }

    /// A reference with a vtable: slot +0x100 (is an actor) answers `actor`, slots +0x17c,
    /// +0x230, +0x284, +0x3d0 and +0x47c are recorded.
    fn reference(e: &mut Engine, vtable: u32, actor: bool) -> u32 {
        let mut slots = vec![0u32; 0x480 / 4];
        slots[0x100 / 4] = vtable + 0x100;
        slots[0x17c / 4] = vtable + 0x17c;
        slots[0x230 / 4] = vtable + 0x230;
        slots[0x284 / 4] = vtable + 0x284;
        slots[0x3d0 / 4] = vtable + 0x3d0;
        slots[0x47c / 4] = vtable + 0x47c;
        e.put_vtable(vtable, &slots);
        if actor {
            e.register(vtable + 0x100, |_, _| 1u32.into_ret());
        } else {
            e.register(vtable + 0x100, |_, _| Ret::default());
        }
        for slot in [0x17c, 0x230, 0x284, 0x3d0, 0x47c] {
            e.register(vtable + slot, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// What `PickpocketCheck` reads of a target: its actor values (+0xa4, slot +8 answers 30),
    /// its base data (`004181e0`, the karma slot +0xc of +0x100 answers -5.0) and the karma
    /// setting (-2.5).
    fn pickpocket_target(e: &mut Engine, target: u32) {
        e.put_vtable(0x0f15_0000, &[0, 0, 0x0f15_0100]);
        e.register(0x0f15_0100, |_, _| 30u32.into_ret());
        e.mem.set_u32(target + 0xa4, 0x0f15_0000);
        let base = e.mem.alloc(0x110);
        e.put_vtable(0x0f15_1000, &[0, 0, 0, 0x0f15_1100]);
        e.register(0x0f15_1100, |_, _| float_ret(-5.0));
        e.mem.set_u32(base + 0x100, 0x0f15_1000);
        e.register_double(REFERENCE_BASE_DATA_004181E0, move |_, _| base.into_ret());
        let karma = e.mem.alloc(4);
        e.mem.set_u32(karma, (-2.5f32).to_bits());
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, _| karma.into_ret());
    }

    #[test]
    fn fn_0075cc70_is_the_mod_list_address() {
        let mut e = engine();
        assert_eq!(e.call(0x0075_cc70, &args![0x1000u32]).u32(), 0x1df4);
    }

    #[test]
    fn do_gamepad_clicks_filters_and_enters_the_other_list() {
        let mut e = engine();
        let this = new_menu(&mut e);
        assert!(!e.call(0x0075_d250, &args![this, 3u32, 0u32]).bool());
        e.register(IS_TOP_MENU, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 0xf8, this + ITEMS);
        assert!(e.call(0x0075_d250, &args![this, 3u32, 0u32]).bool());
        assert!(e.call(0x0075_d250, &args![this, 4u32, 0u32]).bool());
        e.mem.set_u32(this + 0xf8, this + CONTAINER);
        assert!(e.call(0x0075_d250, &args![this, 3u32, 0u32]).bool());
        assert!(e.call(0x0075_d250, &args![this, 4u32, 0u32]).bool());
        let ids: Vec<_> = calls(&e, 0x0f10_0100).iter().map(|w| w[1]).collect();
        assert_eq!(ids, vec![2, 0, 7, 5]);
        assert_eq!(calls(&e, LIST_RESTORE_STATE)[3], vec![this + CONTAINER, 0]);
        // Mode 4 ignores the filter buttons.
        e.mem.set_u32(this + 0x80, 4);
        assert!(!e.call(0x0075_d250, &args![this, 3u32, 0u32]).bool());
        // Button 13 enters the container list from the items list.
        e.put_vtable(0x0f10_1000, &[0, 0x0f10_1100]);
        e.register(0x0f10_1100, |_, _| Ret::default());
        e.mem.set_u32(this + ITEMS, 0x0f10_1000);
        assert!(!e.call(0x0075_d250, &args![this, 0xdu32, 0u32]).bool());
        e.register(LIST_COUNT, |_, _| 2u32.into_ret());
        assert!(e.call(0x0075_d250, &args![this, 0xdu32, 0u32]).bool());
        assert_eq!(calls(&e, 0x0f10_0200), vec![vec![this, 0x15, 0]]);
        assert_eq!(
            calls(&e, LIST_SAVE_STATE).last().unwrap(),
            &vec![this + CONTAINER]
        );
        assert_eq!(
            calls(&e, LIST_RESTORE_STATE).last().unwrap(),
            &vec![this + ITEMS, 1]
        );
        assert!(!e.call(0x0075_d250, &args![this, 5u32, 0u32]).bool());
    }

    #[test]
    fn do_gamepad_up_event_takes_all_or_closes() {
        let mut e = engine();
        let this = new_menu(&mut e);
        assert!(!e.call(0x0075_d430, &args![this, 0xbu32, 0u32]).bool());
        e.mem.set_u32(this + 0x24, 1);
        assert!(e.call(0x0075_d430, &args![this, 0xbu32, 0u32]).bool());
        assert_eq!(calls(&e, 0x0f10_0100), vec![vec![this, 10, 0]]);
        e.mem.set_u32(this + 0x80, 3);
        assert!(!e.call(0x0075_d430, &args![this, 0xbu32, 0u32]).bool());
        e.set_global(CONTAINER_MENU, 0u32);
        assert!(e.call(0x0075_d430, &args![this, 10u32, 0u32]).bool());
        assert!(!e.call(0x0075_d430, &args![this, 9u32, 0u32]).bool());
    }

    #[test]
    fn do_wheel_move_clicks_by_direction() {
        let mut e = engine();
        let this = new_menu(&mut e);
        e.call(0x0075_d4a0, &args![this, 2u32, 0u32]);
        assert!(calls(&e, 0x0f10_0100).is_empty());
        e.register(WHEEL_DIRECTION_0070EC70, |_, _| (-1i32).into_ret());
        e.call(0x0075_d4a0, &args![this, 1u32, 0u32]);
        e.call(0x0075_d4a0, &args![this, 6u32, 0u32]);
        e.register(WHEEL_DIRECTION_0070EC70, |_, _| 1u32.into_ret());
        e.call(0x0075_d4a0, &args![this, 1u32, 0u32]);
        e.call(0x0075_d4a0, &args![this, 6u32, 0u32]);
        let ids: Vec<_> = calls(&e, 0x0f10_0100).iter().map(|w| w[1]).collect();
        assert_eq!(ids, vec![0, 5, 2, 7]);
    }

    #[test]
    fn take_all_moves_an_item_and_checks_a_pickpocket() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let player = reference(&mut e, 0x0f19_0000, true);
        e.set_global(PLAYER, player);
        let container = reference(&mut e, 0x0f11_0000, true);
        pickpocket_target(&mut e, container);
        e.mem.set_u32(this + 0x74, container);
        e.mem.set_u32(this + 0x80, 2);
        e.mem.set_u32(this + 0x84, 5);
        let shown = e.mem.alloc(4);
        e.mem.set_u32(shown, 0x6100);
        e.register_double(LIST_ITEM_TILE, move |_, _| shown.into_ret());
        e.register(WORD_AT_FOUR, |_, _| 3u32.into_ret());
        e.register(ITEM_OBJECT, |_, _| 0x7300u32.into_ret());
        // The stolen value: an item value of 4 and no extra lists: 3 * 4.
        e.register(FLOAT_TO_INT, |_, _| 4u32.into_ret());
        e.call(0x0075_da60, &args![0x6000u32, 0x7200u32]);
        assert_eq!(e.mem.u32(this + 0x84), 17);
        // The chance is 0, so the roll fails: the menu records the failure.
        assert_eq!(e.mem.u8(this + 0x89), 1);
        assert_eq!(
            calls(&e, 0x0f11_017c),
            vec![vec![container, 0x7300, 0, 3, 1, 0, player, 0, 0, 1, 0]]
        );
        // After a failure nothing moves.
        e.call(0x0075_da60, &args![0x6000u32, 0x7200u32]);
        assert_eq!(calls(&e, 0x0f11_017c).len(), 1);
    }

    #[test]
    fn take_all_returns_a_dropped_item_and_skips_a_disabled_one() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let player = reference(&mut e, 0x0f12_0000, true);
        e.set_global(PLAYER, player);
        e.mem.set_u32(this + 0x80, 1);
        let extra_slot = e.mem.alloc(4);
        e.mem.set_u32(extra_slot, 0x7400);
        e.register(FIRST_WORD, |_, _| 0x7500u32.into_ret());
        e.register_double(FIRST_WORD_ADDRESS, move |_, _| extra_slot.into_ret());
        e.register(EXTRA_GET_ITEM_DROPPER, |_, _| 0x7600u32.into_ret());
        e.call(0x0075_da60, &args![0x6000u32, 0x7200u32]);
        assert_eq!(calls(&e, 0x0f12_03d0), vec![vec![player, 0x7600, 1, 0]]);
        assert!(calls(&e, GET_FLOAT_TRAIT).is_empty());
        // A follower's drop goes on to the tile test: 128 stays.
        e.register(FORM_TEST_00440D80, |_, _| 1u32.into_ret());
        let shown = e.mem.alloc(4);
        e.register_double(LIST_ITEM_TILE, move |_, _| shown.into_ret());
        e.register(GET_FLOAT_TRAIT, |_, _| float_ret(128.0));
        e.call(0x0075_da60, &args![0x6000u32, 0x7200u32]);
        assert_eq!(calls(&e, GET_FLOAT_TRAIT).len(), 1);
        assert!(calls(&e, ITEM_OBJECT).is_empty());
    }

    #[test]
    fn pickpocket_check_rolls_against_the_chance() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let player = reference(&mut e, 0x0f13_0000, true);
        e.set_global(PLAYER, player);
        let target = reference(&mut e, 0x0f14_0000, true);
        pickpocket_target(&mut e, target);
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| 50u32.into_ret());
        e.register(PICKPOCKET_CHANCE_00643400, |_, _| 60u32.into_ret());
        e.register(RANDOM_UNSIGNED, |_, _| 10u32.into_ret());
        e.register(ALIGNMENT_FOR_KARMA, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 0xf8, this + CONTAINER);
        assert!(e.call(0x0075_e0b0, &args![this, 0u32, 5u32]).bool());
        assert!(e.call(0x0075_e0b0, &args![this, target, 0u32]).bool());
        assert!(e.call(0x0075_e0b0, &args![this, target, 7u32]).bool());
        assert_eq!(
            calls(&e, PICKPOCKET_CHANCE_00643400),
            vec![vec![50, 30, 7.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, 0x0f15_0100), vec![vec![target + 0xa4, 0x2a]]);
        assert_eq!(calls(&e, 0x0f13_047c), vec![vec![player, 0x2a, 1]]);
        assert_eq!(calls(&e, RANDOM_UNSIGNED)[0][1], 100);
        assert_eq!(calls(&e, MISC_STAT_INCREMENT), vec![vec![0xd]]);
        assert_eq!(e.mem.u8(this + 0x88), 1);
        assert_eq!(
            calls(&e, ALIGNMENT_FOR_KARMA),
            vec![vec![(-5.0f32).to_bits()]]
        );
        assert_eq!(calls(&e, REWARD_KARMA), vec![vec![player, (-2i32) as u32]]);
        // A failed roll: the player reacts, the message shows, +0x89 is set.
        e.register(RANDOM_UNSIGNED, |_, _| 60u32.into_ret());
        e.register(ALIGNMENT_FOR_KARMA, |_, _| 4u32.into_ret());
        assert!(!e.call(0x0075_e0b0, &args![this, target, 7u32]).bool());
        assert_eq!(
            calls(&e, PLAYER_CAUGHT_008C00E0),
            vec![vec![player, target, 0, 0]]
        );
        assert_eq!(calls(&e, SHOW_MESSAGE)[0][1..4], [2, 0, 0]);
        assert_eq!(e.mem.u8(this + 0x89), 1);
        assert_eq!(calls(&e, REWARD_KARMA).len(), 1);
    }

    #[test]
    fn the_stolen_value_counts_owned_and_unowned_stacks() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let container = reference(&mut e, 0x0f16_0000, false);
        e.mem.set_u32(this + 0x74, container);
        e.register(REFERENCE_OWNER, |_, _| 0x7700u32.into_ret());
        e.register(FLOAT_TO_INT, |_, _| 10u32.into_ret());
        // Two extra lists: one owned by 0x7700 (2 items), one unowned (1 item).
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x7a00);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x7b00);
        e.register_double(FIRST_WORD, move |_, _| first.into_ret());
        e.register(FIRST_WORD_ADDRESS, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EXTRA_OWNER_00418660, |_, a| {
            (if a[0] == 0x7a00 { 0x7700u32 } else { 0 }).into_ret()
        });
        e.register(EXTRA_GET_COUNT, |_, a| {
            (if a[0] == 0x7a00 { 2u32 } else { 1 }).into_ret()
        });
        assert_eq!(
            e.call(0x0075_e240, &args![this, 0x7200u32, 5i32, 0u32])
                .i32(),
            20
        );
        // Unowned items count too, and so do the 2 items left after the lists.
        assert_eq!(
            e.call(0x0075_e240, &args![this, 0x7200u32, 5i32, 1u32])
                .i32(),
            50
        );
    }

    #[test]
    fn a_reactivity_bark_speaks_and_shows_its_subtitle() {
        let mut e = engine();
        let this = new_menu(&mut e);
        e.set_global(PLAYER, 0x7100u32);
        let container = reference(&mut e, 0x0f17_0000, true);
        e.mem.set_u32(this + 0x74, container);
        let actor = reference(&mut e, 0x0f18_0000, true);
        e.register_double(RT_DYNAMIC_CAST, move |_, _| actor.into_ret());
        e.register(TOPIC_GET_MATCHING_INFO, |_, _| 0x7c00u32.into_ret());
        e.register(INFO_QUEST_006733E0, |_, _| 0x7d00u32.into_ret());
        e.register(DIALOGUE_ITEM_CURRENT_RESPONSE, |_, _| 0x7e00u32.into_ret());
        e.register(RESPONSE_00460140, |_, _| 0x7f00u32.into_ret());
        e.register(FIRST_WORD, |_, a| (a[0] + 1).into_ret());
        e.register(FIRST_WORD_ADDRESS, |_, a| (a[0] + 8).into_ret());
        e.register(STRING_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xaaaa);
            e.mem.set_u32(a[0] + 4, 0xbbbb);
            Ret::default()
        });
        e.register(SOUND_HANDLE_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], 0xcccc);
            Ret::default()
        });
        e.call(0x0075_ec60, &args![this, 0x7b00u32]);
        assert_eq!(
            calls(&e, TOPIC_GET_MATCHING_INFO),
            vec![vec![0x7b00, actor, 0x7100]]
        );
        let built = calls(&e, DIALOGUE_ITEM_CONSTRUCT);
        assert_eq!(built[0][1..], [0x7d00, 0x7b00, 0x7c00, actor]);
        let spoken = calls(&e, 0x0f18_0284);
        assert_eq!(spoken.len(), 1);
        assert_eq!(
            spoken[0],
            vec![
                actor,
                0x7f01,
                this + 0x100,
                0,
                0,
                0,
                0,
                0,
                0x7100,
                1,
                1,
                0,
                0,
                1
            ]
        );
        assert_eq!(calls(&e, STRING_COPY_CONSTRUCT)[0][1], 0x7e08);
        assert_eq!(calls(&e, SOUND_HANDLE_ASSIGN)[0][1], this + 0x100);
        // The subtitle got the copies: its two destructors run on them.
        assert_eq!(calls(&e, STRING_DESTROY).len(), 1);
        assert_eq!(calls(&e, SOUND_HANDLE_DESTRUCTOR).len(), 1);
        assert_eq!(calls(&e, DIALOGUE_ITEM_DESTRUCT), vec![vec![built[0][0]]]);
        // No info: nothing is built.
        e.register(TOPIC_GET_MATCHING_INFO, |_, _| Ret::default());
        e.call(0x0075_ec60, &args![this, 0x7b00u32]);
        assert_eq!(calls(&e, DIALOGUE_ITEM_CONSTRUCT).len(), 1);
    }

    #[test]
    fn a_reactivity_bark_with_a_voice_file_takes_the_actor_sound() {
        let mut e = engine();
        let this = new_menu(&mut e);
        e.register(TOPIC_GET_MATCHING_INFO, |_, _| 0x7c00u32.into_ret());
        e.register(DIALOGUE_ITEM_CURRENT_RESPONSE, |_, _| 0x7e00u32.into_ret());
        e.register(FORM_OF, |_, _| 0x8100u32.into_ret());
        e.register(VALUE_STRING_0084E3A0, |_, _| 0x8200u32.into_ret());
        e.register(ACTOR_VOICE_SOUND_00933150, |_, _| 0x8300u32.into_ret());
        e.call(0x0075_ec60, &args![this, 0x7b00u32]);
        let sound = calls(&e, ACTOR_VOICE_SOUND_00933150);
        assert_eq!(sound.len(), 1);
        assert_eq!(sound[0][0], 0);
        assert_eq!(sound[0][2..], [0x8200, 0, 0x102, 1]);
        assert_eq!(
            calls(&e, SOUND_HANDLE_ASSIGN)[0],
            vec![this + 0x100, 0x8300]
        );
    }

    #[test]
    fn the_subtitle_time_comes_from_the_sound_or_the_text() {
        let mut e = engine();
        let this = new_menu(&mut e);
        let on = e.mem.alloc(4);
        e.mem.set_u8(on, b'1');
        e.register_double(SUBTITLES_STRING_00408D60, move |_, _| on.into_ret());
        e.register(STRING_004048E0, |_, _| 5u32.into_ret());
        e.register(WORD_AT_FOUR, |_, _| 0x6000u32.into_ret());
        e.register(TILE_FIND_CHILD, |_, _| 0x6100u32.into_ret());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CURRENT_TIME, |_, _| 1000u32.into_ret());
        e.register(SOUND_HANDLE_GET_DURATION, |_, _| 2500u32.into_ret());
        e.call(0x0075_eea0, &args![this, 0x9000u32, 5u32, 1u32, 0u32, 0u32]);
        assert_eq!(
            calls(&e, TILE_FIND_CHILD),
            vec![vec![0x6000, SUBTITLE_TILE]]
        );
        assert_eq!(
            calls(&e, SET_STRING_TRAIT),
            vec![vec![0x6100, TRAIT_STRING as u32, 0x9000, 1]]
        );
        assert_eq!(e.mem.u32(this + 0xfc), 3500);
        let text = calls(&e, STRING_DESTROY)[0][0];
        assert_eq!(calls(&e, SOUND_HANDLE_DESTRUCTOR), vec![vec![text + 8]]);
        // No duration: length 4 times the speed 0.25 times 1000.
        e.register(SOUND_HANDLE_GET_DURATION, |_, _| Ret::default());
        e.register(STRLEN, |_, _| 4u32.into_ret());
        let speed = e.mem.alloc(4);
        e.mem.set_u32(speed, 0.25f32.to_bits());
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, _| speed.into_ret());
        e.call(0x0075_eea0, &args![this, 0x9000u32, 5u32, 1u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xfc), 2000);
        // An empty text sets nothing but still destroys the arguments.
        e.mem.set_u32(this + 0xfc, 0);
        e.register(STRING_004048E0, |_, _| Ret::default());
        e.call(0x0075_eea0, &args![this, 0x9000u32, 5u32, 1u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xfc), 0);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 3);
    }
}
