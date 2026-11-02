//! `fallout/interface/menus/dialogmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The dialogue menu (`DialogMenu`) and the `ListBox<int>` instance of the list box template
//! it keeps its topics in. The `ListBox<int>` methods match the `ListBox<BGSPerk_P>`
//! instance of `levelupmenu.cpp` instruction for instruction (only the vtable, the statics
//! and the calls between the instance's own methods differ).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::startmenu::ListBox;

layout! {
    /// `DialogMenu` (Xbox PDB), a `Menu` (0x28 bytes); the PC code uses the PDB's offsets
    /// (four tiles in `pTiles`, the list box right after them at +0x40).
    pub struct DialogMenu: 0x13c {
        /// `eDialogState` (Xbox PDB).
        0x28 eDialogState: u32,
        /// `eGoodbyeState` (Xbox PDB).
        0x2C eGoodbyeState: u32,
        /// `pTiles` (Xbox PDB): four `Tile*` on PC (`AttachTileByID` takes ids 0 to 3).
        0x30 pTiles: u32,
        /// `lbResponseList` (Xbox PDB): a `ListBox<int>`, 0x30 bytes.
        0x40 lbResponseList: u32,
        /// `pTopicManager` (Xbox PDB): `MenuTopicManager*`.
        0x70 pTopicManager: u32,
        /// `hSpeechFile` (Xbox PDB): `BSSoundHandle`, 0xc bytes.
        0x74 hSpeechFile: u32,
        /// `pSpeaker` (Xbox PDB): `MobileObject*`.
        0x80 pSpeaker: u32,
        /// `strCurrTopicID` (Xbox PDB): `char[0x80]`.
        0x84 strCurrTopicID: u8,
        /// `bResponseShown` (Xbox PDB).
        0x104 bResponseShown: u8,
        /// `fElapsedSecs` (Xbox PDB).
        0x108 fElapsedSecs: f32,
        /// `uiIgnoreInputStartTime` (Xbox PDB).
        0x10C uiIgnoreInputStartTime: u32,
        /// `fPercentZoomed` (Xbox PDB).
        0x128 fPercentZoomed: f32,
        /// `pDepthOfFieldInstance` (Xbox PDB): `ImageSpaceModifierInstanceDOF*`.
        0x12C pDepthOfFieldInstance: u32,
        /// `bIsIn3rdPerson` (Xbox PDB).
        0x130 bIsIn3rdPerson: u8,
        /// `bPaused` (Xbox PDB).
        0x131 bPaused: u8,
        /// `ePausedState` (Xbox PDB).
        0x134 ePausedState: u32,
        /// `bInServiceMenu` (Xbox PDB).
        0x138 bInServiceMenu: u8,
        /// `bShowingTutorial` (Xbox PDB).
        0x139 bShowingTutorial: u8,
        /// `bDoServiceLoad` (Xbox PDB).
        0x13A bDoServiceLoad: u8,
    }
}

/// The open dialogue menu (`Create` stores the new menu here, `Close` reads it).
const OPEN_MENU_011D9510: u32 = 0x011d_9510;
/// The byte the destructor and `Close` clear.
const FLAG_011D9514: u32 = 0x011d_9514;
/// The topic info `Create`, `DoClick` and `LoadNextScreen` store (`009611e0` of a topic)
/// and `Close` and `LoadNextScreen` run the result of.
const LAST_INFO_011D9518: u32 = 0x011d_9518;
/// The countdown `DoIdle` state 3 waits on (a `float`).
const COUNTDOWN_011D951C: u32 = 0x011d_951c;
/// The start time `DoIdle` state 1 keeps.
const START_TIME_011D9594: u32 = 0x011d_9594;
/// The trait ids `Create` looks up: `"_DialogVisible"`, `"_ShowingText"`,
/// `"_current_value"`.
const DIALOG_VISIBLE_TRAIT_011D9508: u32 = 0x011d_9508;
const SHOWING_TEXT_TRAIT_011D9500: u32 = 0x011d_9500;
const CURRENT_VALUE_TRAIT_011D9504: u32 = 0x011d_9504;
/// The `float` `Create` sets from the speaker's package and `DoIdle` scales the zoom by
/// (divided by 100 when non-zero).
const ZOOM_PERCENT_011A008C: u32 = 0x011a_008c;
/// The byte `00762150` sets and `007624c0` clears.
const FLAG_011DCFA4: u32 = 0x011d_cfa4;
/// The two bytes `DoIdle` sets when the closing fade ends.
const FLAG_011F21D0: u32 = 0x011f_21d0;
const FLAG_011F21D1: u32 = 0x011f_21d1;
/// The player (`PlayerCharacter*`).
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
/// The timer object at `011f6394`: `00825c00` (the word at +0x14 of its object) gives the
/// time in milliseconds, `0084d030` (`ST0`) the frame's seconds.
const TIMER_011F6394: u32 = 0x011f_6394;
const WORD_AT_14_00825C00: u32 = 0x0082_5c00;
const FRAME_SECONDS_0084D030: u32 = 0x0084_d030;

/// The `DialogMenu` vtable (`0107257c`) and the menu id (`0x3f1`).
const DIALOG_MENU_VTABLE: u32 = 0x0107_257c;
const DIALOG_MENU_CLASS: u32 = 0x3f1;

/// `MemoryManager` allocate and free (`00401000`, `00401030`, `cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `ImageSpaceModifierInstance::Stop` (`005293f0`, `cdecl`).
const IMAGE_SPACE_INSTANCE_STOP: u32 = 0x0052_93f0;
/// `0083e510` (no arguments; in `menutopic.cpp`).
const TOPICS_RELEASE_0083E510: u32 = 0x0083_e510;
/// `00483710` (`thiscall`): the `BSSoundHandle` destructor.
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// `00a1c520` (`thiscall`): the `Menu` destructor.
const MENU_DESTROY: u32 = 0x00a1_c520;
/// `004fb070` (`thiscall`): the word at +0x80 of its object.
const WORD_AT_80_004FB070: u32 = 0x004f_b070;
/// `00401170` (`thiscall`): the byte at +4 of a form (its form type).
const FORM_TYPE_00401170: u32 = 0x0040_1170;
/// `009611e0` (`thiscall`): the word at +0x18 of its object (a topic's info).
const WORD_AT_18_009611E0: u32 = 0x0096_11e0;
/// `0045cd60` (`thiscall`): the word at +0x28 of its object.
const WORD_AT_28_0045CD60: u32 = 0x0045_cd60;
/// `00619df0` (`thiscall`): bit 0 of the byte at +0x25.
const FLAG_00619DF0: u32 = 0x0061_9df0;
/// `TESTopic::GetTopic` (`0061a2d0`, `cdecl`, two words).
const TES_TOPIC_GET_TOPIC: u32 = 0x0061_a2d0;

/// `004b7210` (no arguments): the interface manager; its `+0x4d4` is the tutorial manager
/// (`00718840`, `thiscall`, `RET 4`; `InterfaceManager::TutorialManager::ShowMessage`
/// `00718630`, `RET 0xc`).
const INTERFACE_MANAGER_004B7210: u32 = 0x004b_7210;
const TUTORIAL_00718840: u32 = 0x0071_8840;
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;
/// `Tile::GetMenuByClass` (`00a09030`, `cdecl`).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `005b5e40` (`cdecl`, a format and its words): the debug message.
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
/// `00586150` (`thiscall`): the word at +0x9c of the interface manager.
const WORD_AT_9C_00586150: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`00a01b00`, `thiscall`, `RET 4`), `Tile::GetMenu` (`00a03c90`),
/// `Menu::SetMenuTile` (`00a1dc70`, `RET 8`), `Tile::GetMaximumDepth` (`00a01530`, no
/// arguments, `ST0`), `Menu::PrepForVisibility` (`00a1dc20`, `RET 4`), `Menu::StartFadeOut`
/// (`00a1d910`).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `6006.0` and `102.0` (`double`s) `Create` compares the menu tile's trait `0xfa7` with.
const VALUE_6006_DOUBLE: u32 = 0x0107_04f0;
const VALUE_102_DOUBLE: u32 = 0x0107_04e8;
/// `TESImageSpaceModifier::GetGetHit` (`005d2860`, no arguments) and
/// `ImageSpaceModifierInstanceForm::Stop` (`00529c90`, `cdecl`).
const IMAGE_SPACE_GET_GET_HIT: u32 = 0x005d_2860;
const IMAGE_SPACE_FORM_STOP: u32 = 0x0052_9c90;
/// `MobileObject::GetCurrentPackage` (`009344a0`), `MobileObject::StopCurrentDialogue`
/// (`00934250`), `MobileObject::IsInDialoguewithPlayer` (`00933840`).
const MOBILE_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
const MOBILE_STOP_CURRENT_DIALOGUE: u32 = 0x0093_4250;
const MOBILE_IS_IN_DIALOGUE_WITH_PLAYER: u32 = 0x0093_3840;
/// `005d43c0` (`thiscall`), `ExtraDataList::GetPackageExtra` (`0041cb10`), `0041ca90`
/// (the package's type), `00672850` (`ST0`).
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
const EXTRA_GET_PACKAGE: u32 = 0x0041_cb10;
const PACKAGE_TYPE_0041CA90: u32 = 0x0041_ca90;
const PACKAGE_VALUE_00672850: u32 = 0x0067_2850;
/// `Actor::EndInterruptPackage` (`00881680`, `RET 4`), `Actor::StopMoving` (`008b3ab0`),
/// `Actor::SetIronSights` (`008bb650`, `RET 0xc`), `Actor::AlwaysShowSubtitles`
/// (`008c1bc0`).
const ACTOR_END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
const ACTOR_STOP_MOVING: u32 = 0x008b_3ab0;
const ACTOR_SET_IRON_SIGHTS: u32 = 0x008b_b650;
const ACTOR_ALWAYS_SHOW_SUBTITLES: u32 = 0x008c_1bc0;
/// `MenuTopicManager` methods (`menutopic.cpp`): `Initialize` (`0083e620`, `RET 8`),
/// `GetGreeting` (`0083e4f0`), `GetCurrentTopic` (`0083e4c0`), `0083e430` (`RET 4`),
/// `DoSpeechChallengeCheck` (`0083e850`, `RET 4`), `ProcessCurrentTopic` (`0083ebb0`),
/// `FirstTopic` (`0083e370`, `RET 4`), `NextTopic` (`0083e3e0`), `LoadNextTopicList`
/// (`0083ec30`), `0083f1a0` (`RET 4`), `MenuTopic::GetCurrentResponse` (`0083e170`), the
/// `MenuTopic` constructor `0083db00` (`RET 0x14`).
const TOPICS_INITIALIZE: u32 = 0x0083_e620;
const TOPICS_GET_GREETING: u32 = 0x0083_e4f0;
const TOPICS_GET_CURRENT_TOPIC: u32 = 0x0083_e4c0;
const TOPICS_SELECT_0083E430: u32 = 0x0083_e430;
const TOPICS_DO_SPEECH_CHALLENGE_CHECK: u32 = 0x0083_e850;
const TOPICS_PROCESS_CURRENT_TOPIC: u32 = 0x0083_ebb0;
const TOPICS_FIRST_TOPIC: u32 = 0x0083_e370;
const TOPICS_NEXT_TOPIC: u32 = 0x0083_e3e0;
const TOPICS_LOAD_NEXT_TOPIC_LIST: u32 = 0x0083_ec30;
const TOPICS_SET_LIST_0083F1A0: u32 = 0x0083_f1a0;
const TOPIC_GET_CURRENT_RESPONSE: u32 = 0x0083_e170;
const TOPIC_CONSTRUCT_0083DB00: u32 = 0x0083_db00;
/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`): the process of an actor.
const GET_PROCESS: u32 = 0x008d_8520;
/// `004eaf60` (`thiscall` on the player, a `bool`), `00951a10` (`RET 4`),
/// `PlayerCharacter::FocusOnActor` (`00953060`, `RET 0xc`),
/// `PlayerCharacter::StopIronSightsImageSpace` (`009655a0`),
/// `PlayerCharacter::ResetPlayerGreetFlag` (`00953ce0`), `PlayerCharacter::UpdateCamera`
/// (`0094ae40`, `RET 8`).
const PLAYER_004EAF60: u32 = 0x004e_af60;
const PLAYER_00951A10: u32 = 0x0095_1a10;
const PLAYER_FOCUS_ON_ACTOR: u32 = 0x0095_3060;
const PLAYER_STOP_IRON_SIGHTS_IMAGE_SPACE: u32 = 0x0096_55a0;
const PLAYER_RESET_GREET_FLAG: u32 = 0x0095_3ce0;
const PLAYER_UPDATE_CAMERA: u32 = 0x0094_ae40;
/// `00439ef0` (`thiscall`, `RET 8`) and `00457990` (`thiscall`, `ST0`).
const POSITION_00439EF0: u32 = 0x0043_9ef0;
const POSITION_00457990: u32 = 0x0045_7990;
/// `00403e20` (`thiscall` on a game setting, no arguments): the setting's value address.
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;
/// The game settings read through `00403e20`.
const SETTING_011D46FC: u32 = 0x011d_46fc;
const SETTING_011CD450: u32 = 0x011c_d450;
const SETTING_011D3EE0: u32 = 0x011d_3ee0;
const SETTING_011D243C: u32 = 0x011d_243c;
const SETTING_011D318C: u32 = 0x011d_318c;
const SETTING_011D0A24: u32 = 0x011d_0a24;
/// `ImageSpaceModifierInstanceDOF::Trigger` (`0052a150`, `cdecl`, six words) and `005bbd60`
/// (`RET 4`).
const DOF_TRIGGER: u32 = 0x0052_a150;
const IMAGE_SPACE_005BBD60: u32 = 0x005b_bd60;
/// `ListBox<unsigned_int>::Initialize` (`0075a0d0`, `RET 8`; the template code).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// `00408d60` (`thiscall` on `011d8934`): a string whose first byte `Create` tests.
const STRING_00408D60: u32 = 0x0040_8d60;
const STRING_OWNER_011D8934: u32 = 0x011d_8934;
/// `0055d520` (`thiscall`): the reference's name.
const REFERENCE_NAME_0055D520: u32 = 0x0055_d520;
/// `005d4a40` (no arguments, a `bool`).
const FLAG_005D4A40: u32 = 0x005d_4a40;
/// `BSAudio::QInstance` (`00453a70`), `BSAudio::EnterDialogue` (`00ad85a0`),
/// `BSAudio::ExitDialogue` (`00ad8650`).
const BS_AUDIO_INSTANCE: u32 = 0x0045_3a70;
const BS_AUDIO_ENTER_DIALOGUE: u32 = 0x00ad_85a0;
const BS_AUDIO_EXIT_DIALOGUE: u32 = 0x00ad_8650;
/// `Interface::SetGunScopeVisible` (`00709c40`, `cdecl`), `Interface::IsConsoleVisible`
/// (`00703d50`), `Interface::IsTopMenuID` (`00702450`, `cdecl`).
const INTERFACE_SET_GUN_SCOPE_VISIBLE: u32 = 0x0070_9c40;
const INTERFACE_IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
const INTERFACE_IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `00493bb0` (`thiscall`, a `bool`), `00579200` (`thiscall`, a `bool`),
/// `TESTopicInfo::RunResult` (`0061f170`, `RET 8`), `TESTopicInfo::AddTopicList`
/// (`0061f150`).
const ACTOR_00493BB0: u32 = 0x0049_3bb0;
const INFO_00579200: u32 = 0x0057_9200;
const INFO_RUN_RESULT: u32 = 0x0061_f170;
const INFO_ADD_TOPIC_LIST: u32 = 0x0061_f150;
/// `MiscStatManager::Increment` (`004d5c60`, `cdecl`).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// `ActorValueOwner::GetClampedActorValue` (`0066ef20`, `RET 4`) and
/// `ActorValueOwner::GetClampedActorFloatValue` (`0066ef50`, `RET 4`, `ST0`), on the
/// player's +0xa4.
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const GET_CLAMPED_ACTOR_FLOAT_VALUE: u32 = 0x0066_ef50;
/// `00500940` (`thiscall`), `00413f40` (`thiscall`), `00681490` (`thiscall`, `ST0`): the
/// condition list of an info, an item's condition data, its comparison value.
const CONDITIONS_00500940: u32 = 0x0050_0940;
const CONDITION_DATA_00413F40: u32 = 0x0041_3f40;
const CONDITION_VALUE_00681490: u32 = 0x0068_1490;
/// `0084e3a0` (`thiscall`).
const OBJECT_0084E3A0: u32 = 0x0084_e3a0;
/// `008bc590` (`cdecl`, two words).
const ACTOR_008BC590: u32 = 0x008b_c590;
/// `0063f790` (`thiscall`, `RET 4`, a `float`).
const DOF_0063F790: u32 = 0x0063_f790;
/// `0061af30` (`cdecl`, six words), `0096a2d0` (list constructor), `00905820` (list
/// append, `RET 4`).
const INFO_0061AF30: u32 = 0x0061_af30;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_APPEND: u32 = 0x0090_5820;
/// `00418900` (`thiscall`, `RET 4`): the sound handle assignment.
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `00833c20` (`thiscall`, a `bool`; in `falloutradio.cpp`).
const RADIO_00833C20: u32 = 0x0083_3c20;
/// `00559450` (`thiscall`): the word at +0 of its object; `00403df0` (`thiscall` on a game
/// setting): the setting's string.
const WORD_AT_0_00559450: u32 = 0x0055_9450;
const SETTING_STRING_00403DF0: u32 = 0x0040_3df0;
const SETTING_011D27FC: u32 = 0x011d_27fc;
const SETTING_011D3FE8: u32 = 0x011d_3fe8;
/// The string functions (`cdecl`): `00406d00` (`sprintf_s`-like: buffer, size, format,
/// words), `00406d30` (copy: buffer, size, source), `00406d50` (append: buffer, size,
/// source).
const FORMAT_00406D00: u32 = 0x0040_6d00;
const COPY_00406D30: u32 = 0x0040_6d30;
const APPEND_00406D50: u32 = 0x0040_6d50;
/// `MapMarkerData::GetLocationName` (`00408da0`, `thiscall`): a string object's text.
const STRING_CHARS_00408DA0: u32 = 0x0040_8da0;
/// `00574900`, `005e3fa0`, `BGSSaveFormBuffer::GetForm` (`007af430`), `00516bf0`
/// (`thiscall` each): the container chain `LoadTopicsList` reads for a non-actor speaker.
const OBJECT_00574900: u32 = 0x0057_4900;
const OBJECT_005E3FA0: u32 = 0x005e_3fa0;
const SAVE_FORM_BUFFER_GET_FORM: u32 = 0x007a_f430;
const CONTAINER_00516BF0: u32 = 0x0051_6bf0;
/// `0061f2b0` (`thiscall` on an info), `00648ea0` (`cdecl`, three words; in
/// `gameplayformulas.cpp`).
const INFO_0061F2B0: u32 = 0x0061_f2b0;
const FORMULA_00648EA0: u32 = 0x0064_8ea0;
/// `00707e10` (`thiscall`, `RET 4`; in `interface.cpp`): gives a tile a name.
const TILE_SET_NAME_00707E10: u32 = 0x0070_7e10;
/// `004fb070`-like getter of the `[menu + 4]` word: `00726070` (`thiscall`).
const WORD_AT_4_00726070: u32 = 0x0072_6070;

/// The strings (`.rdata`).
const NULL_SPEAKER_TEXT: u32 = 0x0107_2690;
const MENU_FILE_TEXT: u32 = 0x0107_266c;
const CREATION_FAILED_TEXT: u32 = 0x0107_2618;
const TOPIC_TEMPLATE_TEXT: u32 = 0x0107_2604;
const SCROLLBAR_TEXT: u32 = 0x0107_25f4;
const DIALOG_VISIBLE_TEXT: u32 = 0x0107_25e4;
const SHOWING_TEXT_TEXT: u32 = 0x0107_25d4;
const SHOW_SUBTITLES_TEXT: u32 = 0x0107_25c4;
/// `"_current_value"`.
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
const INVALID_CHOICE_TEXT: u32 = 0x0107_2710;
/// `"%s (%d)"`, `"----"`, `"NULL"`, `"MENUS: (Dialog Menu) Topic %s has no responses..."`,
/// `"%s %s"`, `"%s"`, `" "`.
const TOPIC_ID_FORMAT: u32 = 0x0106_eb00;
const NO_TOPIC_ID_TEXT: u32 = 0x0107_27a8;
const NULL_TEXT: u32 = 0x0101_f7a8;
const NO_RESPONSES_TEXT: u32 = 0x0107_2760;
const TWO_STRINGS_FORMAT: u32 = 0x0101_2058;
const ONE_STRING_FORMAT: u32 = 0x0101_9f08;
const SPACE_TEXT: u32 = 0x0102_0770;
/// `LoadTopicsList`'s strings: `"[%s"`, `"%s, %i"`, `"%"`, `"%s %d/%d"`, `"%s %d"`,
/// `"%s]  %s"`, `"unknown"`, the error format, `"topic_%i"`, `"_line_alpha"`,
/// `"_ScrollbarHeight"`, `"_height"`.
const PROMPT_FORMAT: u32 = 0x0107_2864;
const CHALLENGE_FORMAT: u32 = 0x0107_285c;
const PERCENT_TEXT: u32 = 0x0101_2320;
const SKILL_SHORT_FORMAT: u32 = 0x0107_2850;
const SKILL_FORMAT: u32 = 0x0104_700c;
const PROMPT_CLOSE_FORMAT: u32 = 0x0107_2848;
const UNKNOWN_TEXT: u32 = 0x0107_2840;
const NO_TOPIC_TEXT_ERROR: u32 = 0x0107_27e8;
const TOPIC_NAME_FORMAT: u32 = 0x0107_27d8;
const LINE_ALPHA_TEXT: u32 = 0x0107_27cc;
const SCROLLBAR_HEIGHT_TEXT: u32 = 0x0107_27b8;
const HEIGHT_TEXT: u32 = 0x0107_27b0;

// ---------------------------------------------------------------------------
// The `ListBox<int>` instance.

/// The `ListBox<int>` vtable (`0107286c`).
const INT_LIST_BOX_VTABLE: u32 = 0x0107_286c;
/// `ListBoxBase` construction (`007240c0`), the embedded list's destructor (`0046ffb0`),
/// the flag initialisation (`007a1950`, `RET 8`), the menu manager's list (`00721130` /
/// `007211c0`, `cdecl`).
const LIST_BOX_BASE_CONSTRUCT: u32 = 0x0072_40c0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const LIST_BOX_INIT_FLAGS: u32 = 0x007a_1950;
const LIST_BOX_REGISTER: u32 = 0x0072_1130;
const LIST_BOX_UNREGISTER: u32 = 0x0072_11c0;
/// The list node helpers: `006815c0` (the node's item slot), `006ecd40` (sets the next
/// node, `RET 4`), `004702f0` (deletes the node, `RET 4`).
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_SET_NEXT: u32 = 0x006e_cd40;
const NODE_DELETE: u32 = 0x0047_02f0;
/// `004459e0` (`RET 4`): the item data's deleting call, given 1.
const ITEM_DATA_DELETE: u32 = 0x0044_59e0;
/// The byte that keeps item tiles alive when set.
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// `007242c0` / `007242f0` (`RET 4`): has a flag / owns a tile.
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Tile functions.
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
const TILE_FLAG_GETTER_0071AA80: u32 = 0x0071_aa80;
const TILE_FLAG_SETTER_0071AA60: u32 = 0x0071_aa60;
/// `Menu::RenderTemplate` (`00a1ddb0`, `RET 0xc`).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// The item constructor (`007a62e0`, `RET 8`), the sorted insert (`007a7eb0`, `RET 8`),
/// `InitTilePosition` (`007269d0`, `RET 4`), the item count (`00726c40`).
const ITEM_CONSTRUCT: u32 = 0x007a_62e0;
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
const LIST_BOX_COUNT_00726C40: u32 = 0x0072_6c40;
/// The tile value's actions (`00a09200`, `00a09130`, `00a09080`, `00a09410`).
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
const VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `_ftol2_sse` (`00ec62c0`), `004e4430` (`cdecl`, a `float`, `ST0`), `00647b70`
/// (`cdecl`, two `int`s).
const FTOL2: u32 = 0x00ec_62c0;
const ROUND_FLOAT: u32 = 0x004e_4430;
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
/// `"_number_of_visible_items"`, `"_num_filtered"`, `"_enabled"`, `"_highlight_y"`,
/// `"_selected_height"`, `"lb_highlight_box"`, `"top"`, `"bottom"`, `"left"`, `"right"`,
/// `"_scroll_delta"`, `"_number_of_items"`.
const NUMBER_OF_VISIBLE_ITEMS_TEXT: u32 = 0x0106_f860;
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
const ENABLED_TEXT: u32 = 0x0107_0244;
const HIGHLIGHT_Y_TEXT: u32 = 0x0106_f8c0;
const SELECTED_HEIGHT_TEXT: u32 = 0x0106_f8ac;
const HIGHLIGHT_BOX_TEXT: u32 = 0x0106_f898;
const TOP_TEXT: u32 = 0x0106_f894;
const BOTTOM_TEXT: u32 = 0x0106_f88c;
const LEFT_TEXT: u32 = 0x0106_f884;
const RIGHT_TEXT: u32 = 0x0106_f87c;
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
const NUMBER_OF_ITEMS_TEXT: u32 = 0x0106_f8f8;
/// `0.0`, `1.0`, `0.1`, `500.0`, `100.0` (`double`s).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const TENTH_DOUBLE: u32 = 0x0107_2758;
const FIVE_HUNDRED_DOUBLE: u32 = 0x0103_01a8;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;

/// A trait id kept in a function-local static: the guard word (bit `bit` says it has been
/// looked up) and the id.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

/// `ListBox<int>::IsEnabled`'s `"_enabled"` id.
const ENABLED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_959c,
    bit: 1,
    cache: 0x011d_9598,
    name: ENABLED_TEXT,
};
/// `ListBox<int>::UpdateHighlightBoxPos`'s `"_highlight_y"` (bit 1) and
/// `"_selected_height"` (bit 2) ids.
const HIGHLIGHT_Y_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_95a8,
    bit: 1,
    cache: 0x011d_95a4,
    name: HIGHLIGHT_Y_TEXT,
};
const SELECTED_HEIGHT_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_95a8,
    bit: 2,
    cache: 0x011d_95a0,
    name: SELECTED_HEIGHT_TEXT,
};
/// `ListBox<int>::NumUnfiltered`'s `"_num_filtered"` id.
const NUM_FILTERED_TRAIT: StaticTrait = StaticTrait {
    guard: 0x011d_95b0,
    bit: 1,
    cache: 0x011d_95ac,
    name: NUM_FILTERED_TEXT,
};

/// Looks the static trait id up the first time (the guard bit is set before
/// `Tile::AddUserTrait(name, -1)` runs, as the code does); returns the id read back.
fn static_trait(e: &mut Engine, which: StaticTrait) -> u32 {
    if e.mem.u32(which.guard) & which.bit == 0 {
        let guard = e.mem.u32(which.guard);
        e.mem.set_u32(which.guard, guard | which.bit);
        let id = e
            .call(ADD_USER_TRAIT, &args![which.name, 0xffff_ffffu32])
            .u32();
        e.mem.set_u32(which.cache, id);
    }
    e.mem.u32(which.cache)
}

/// The item word of a list node, reached through `006815c0` as the code does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(node)
}

/// The next node (`00726070`, the word at +4).
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_4_00726070, &args![node]).u32()
}

/// The first node of the embedded list (`this + 4`; the code guards a null `this`).
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: the code goes on only when the value is strictly
/// above the other (not less, equal or unordered).
fn above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

// Translated from 007642f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007642f0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the embedded list is built) is not translated.
/// The `ListBox<int>` constructor (unnamed in the engine map; vtable `0107286c`): the
/// `ListBoxBase` base (`007240c0`), the embedded list (+4, `0096a2d0`), the vtable, the
/// fields cleared, `007a1950(this, 0, 1)` and the list box handed to `00721130`. Returns
/// `this`.
pub fn fn_007642f0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    e.call(LIST_BOX_BASE_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), INT_LIST_BOX_VTABLE);
    e.set(this, ListBox::pScrollBar, Ptr::NULL);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.set(this, ListBox::pContainerRect, Ptr::NULL);
    e.set(this, ListBox::pTemplateName, Ptr::NULL);
    e.set(this, ListBox::usFlags, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fSavedScrollbarValue, 0.0);
    e.set(this, ListBox::fSavedHighlightIndex, 0.0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    e.call(LIST_BOX_INIT_FLAGS, &args![this, 0u32, 1u32]);
    e.call(LIST_BOX_REGISTER, &args![this]);
    this
}

// Translated from 007643c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007643c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `ListBox<int>` destructor (unnamed in the engine map): the vtable, `RemoveAll`, the
/// list box handed to `007211c0`, then the embedded list destroyed (`0046ffb0`).
pub fn fn_007643c0(e: &mut Engine, this: Ptr<ListBox>) {
    e.mem.set_u32(this.addr(), INT_LIST_BOX_VTABLE);
    list_box_int_remove_all(e, this);
    e.call(LIST_BOX_UNREGISTER, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 00764430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00764430 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<int>::AddItem` (engine map; `RET 0x10`): with a container and a template name
/// (`template`, else `pTemplateName`), renders the template in the container's menu
/// (`Menu::RenderTemplate`), gives the new tile trait `0xfaa` -1 when it has none and the
/// text `text` (trait `0xfc4`) when there is one, and adds an item (tile, `data`): sorted
/// with `compare` (then `Synchronize` when flag 0 is set) or appended (then, with flag 0,
/// `InitTilePosition` and `SetTotalListHeight`, and the tile gets the next index as trait
/// `0xfac`). When the list then counts 1 (`00726c40`) and the container's
/// `"_number_of_visible_items"` is above `0.0`, the container's height value (`0xfb0`) is
/// rebuilt from the tile's height times that number. Returns the tile, or 0 without a
/// container or template.
pub fn list_box_int_add_item(
    e: &mut Engine,
    this: Ptr<ListBox>,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    if e.get(this, ListBox::pContainerRect).is_null() {
        return 0;
    }
    if e.get(this, ListBox::pTemplateName).is_null() && template == 0 {
        return 0;
    }
    let name = if template != 0 {
        template
    } else {
        e.get(this, ListBox::pTemplateName).addr()
    };
    let container = e.get(this, ListBox::pContainerRect);
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, container, name, 0u32])
        .u32();
    if e.call(TILE_GET_VALUE, &args![tile, 0xfaau32]).u32() == 0 {
        e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0xffff_ffffu32]);
    }
    if text != 0 {
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    }
    // The item is built with the address of the `data` argument's stack slot.
    let item = e.with_stack(4, |e, data_slot| {
        e.mem.set_u32(data_slot.addr(), data);
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        if block != 0 {
            e.call(ITEM_CONSTRUCT, &args![block, tile, data_slot]).u32()
        } else {
            0
        }
    });
    if compare != 0 {
        e.call(LIST_INSERT_SORTED, &args![this.addr() + 4, item, compare]);
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            list_box_int_synchronize(e, this);
        }
    } else {
        e.with_stack(4, |e, item_slot| {
            e.mem.set_u32(item_slot.addr(), item);
            e.call(LIST_APPEND, &args![this.addr() + 4, item_slot]);
        });
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            e.call(INIT_TILE_POSITION, &args![this, tile]);
            list_box_int_set_total_list_height(e, this);
        }
        let index = e.get(this, ListBox::usNextIndex);
        e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
        e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
    }
    if e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32() == 1 {
        let visible = e
            .call(
                ADD_USER_TRAIT,
                &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
            )
            .u32();
        let container = e.get(this, ListBox::pContainerRect);
        let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if above(count, zero) {
            let container = e.get(this, ListBox::pContainerRect);
            let value = e.call(TILE_GET_VALUE_Q, &args![container, 0xfb0u32]).u32();
            e.call(VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![value, 0x7d0u32, tile, 0xfb0u32],
            );
            let visible = e
                .call(
                    ADD_USER_TRAIT,
                    &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
                )
                .u32();
            let container = e.get(this, ListBox::pContainerRect);
            let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
            e.call(
                VALUE_ADD_ACTION_FLOAT,
                &args![value, 0x7d3u32, x87_store32(count)],
            );
            e.call(VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    tile
}

/// Deletes the `ListBoxItem` (`pTile` +0, `data` +4) that `item` reads each time: the tile
/// through its deleting destructor (vtable slot 0, flag 1) unless `KEEP_ITEM_TILES` is set,
/// the data (`004459e0` with 1) when flag bit 1 is set, then the item freed.
fn delete_item(e: &mut Engine, this: Ptr<ListBox>, item: impl Fn(&mut Engine) -> u32) {
    if e.global::<u8>(KEEP_ITEM_TILES) == 0 {
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

// Translated from 007646d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>::RemoveAll` (engine map): while the second node (+8) and its item are
/// non-null, deletes the item, clears the node's next pointer (`006ecd40` with 0), deletes
/// the node (`004702f0` with 1) and makes the following node the second; then deletes the
/// first item (+4). Clears the first item, `usNextIndex` and `fTotalListHeight`, sets the
/// container's `"_num_filtered"` to `0.0`, then `SetTotalListHeight`, clears
/// `pHighlightedItem` and `UpdateHighlightBoxPos`.
pub fn list_box_int_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    let base = this.addr();
    loop {
        let second = e.mem.u32(base + 8);
        if second == 0 || node_item(e, second) == 0 {
            break;
        }
        let second = e.mem.u32(base + 8);
        let next = node_next(e, second);
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
    list_box_int_set_total_list_height(e, this);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    list_box_int_update_highlight_box_pos(e, this);
}

// Translated from 00764930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>::Synchronize` (engine map): clears `usNextIndex` and `fTotalListHeight`,
/// then for every node with an item whose filtered byte (+8) is clear gives the item's tile
/// the next index (trait `0xfac`) and its position (`InitTilePosition`); then
/// `SetTotalListHeight` and `UpdateHighlightBoxPos`.
pub fn list_box_int_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u8(item + 8) == 0 {
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(INIT_TILE_POSITION, &args![this, tile]);
        }
        node = node_next(e, node);
    }
    list_box_int_set_total_list_height(e, this);
    list_box_int_update_highlight_box_pos(e, this);
}

// Translated from 00764a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>::Highlight` (engine map): when the list is enabled and `tile` is null, or
/// is one of its items (`007242f0`) and `"_enabled"`, makes it `pHighlightedItem`, moves
/// the highlight box and returns true; otherwise false.
pub fn list_box_int_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    if !list_box_int_is_enabled(e, this) {
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
    list_box_int_update_highlight_box_pos(e, this);
    true
}

// Translated from 00764e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00764e40 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<int>::IsEnabled` (engine map): the container exists and its `"_enabled"` trait
/// (a function-local static id) is true.
pub fn list_box_int_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    static_trait(e, ENABLED_TRAIT);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return false;
    }
    let enabled = e.mem.u32(ENABLED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

/// The tile's byte (`0071aa60`) set to `flag` and the tile queued for an update with
/// flags 8.
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_FLAG_SETTER_0071AA60, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

// Translated from 00764ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00764ef0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `ListBox<int>::UpdateHighlightBoxPos` (engine map): looks the `"_highlight_y"` and
/// `"_selected_height"` trait ids up (statics); with a container and no highlighted item,
/// sets them to -1 and 0 on the container; with one, makes both values follow the
/// highlighted tile's traits `0xfa2` and `0xfb0` (action `0x7d0`, then calculated) and gives
/// the container's `"lb_highlight_box"` tile, and its `"top"`, `"bottom"`, `"left"` and
/// `"right"` children when `"top"` exists (the other three are not checked for null), the
/// highlighted tile's byte (`0071aa80`, as 1 or 0) and an update.
pub fn list_box_int_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    static_trait(e, HIGHLIGHT_Y_TRAIT);
    static_trait(e, SELECTED_HEIGHT_TRAIT);
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let y_trait = e.mem.u32(HIGHLIGHT_Y_TRAIT.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, y_trait, 0xffff_ffffu32]);
        let height_trait = e.mem.u32(SELECTED_HEIGHT_TRAIT.cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, height_trait, 0u32]);
        return;
    }
    let y_trait = e.mem.u32(HIGHLIGHT_Y_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_trait = e.mem.u32(SELECTED_HEIGHT_TRAIT.cache);
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
    // The two branches of the code differ only in the byte they pass.
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

// Translated from 00765210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>::SetTotalListHeight` (engine map): with a container, its visible height
/// (trait `0xfb0`, stored as a `float`) and the total list height decide the number of
/// scroll steps: 1, unless both are above `0.0` and the visible height is below the total,
/// then `00647b70(_ftol2(004e4430(unfiltered * (total - visible) / total)) + 1, 2)` (the
/// ratio stored as a `float`). With both above `0.0` and more than one step, the
/// container's `"_scroll_delta"` becomes `(total - visible) / (steps - 1)`; with a scroll
/// bar its `"_number_of_items"` becomes the number of steps.
pub fn list_box_int_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = x87_store32(e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64());
    let mut steps = 1i32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let total_bits = e.mem.u32(this.addr() + 0x20);
    if above(x87_load32(total_bits), zero) && above(x87_load32(visible), zero) {
        // `FLD total; FLD visible; FCOMPP; TEST AH,5; JP`: on only when the visible height
        // is below the total (ordered).
        let total_bits = e.mem.u32(this.addr() + 0x20);
        if x87_load32(visible) < x87_load32(total_bits) {
            let count = list_box_int_num_unfiltered(e, this) as u32;
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

// Translated from 00765340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00765340 - the C++ exception frame (FS:[0] chain, unwind state 0 around the static's look-up) is not translated.
/// `ListBox<int>::NumUnfiltered` (engine map): `00726c40(this)` minus the container's
/// `"_num_filtered"` (a function-local static id; the float truncated by `FISTP qword`).
pub fn list_box_int_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    static_trait(e, NUM_FILTERED_TRAIT);
    let count = e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32();
    let num_filtered = e.mem.u32(NUM_FILTERED_TRAIT.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let value = e
        .call(TILE_GET_FLOAT, &args![container, num_filtered])
        .f64();
    let filtered = x87_fistp64_chop(X87Ext::from(value)) as u32;
    count.wrapping_sub(filtered) as i32
}

// ---------------------------------------------------------------------------
// `DialogMenu`.

/// The open dialogue menu.
fn open_menu(e: &Engine) -> u32 {
    e.global(OPEN_MENU_011D9510)
}

/// The player.
fn player(e: &Engine) -> u32 {
    e.global(PLAYER_011DEA3C)
}

/// The menu's tile (`00726070`, the word at +4: `Menu::pMenu`).
fn menu_tile(e: &mut Engine, menu: u32) -> u32 {
    e.call(WORD_AT_4_00726070, &args![menu]).u32()
}

/// A tile's integer trait (`00700320`).
fn set_int(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

/// The time in milliseconds (`00825c00` on the timer).
fn now(e: &mut Engine) -> u32 {
    e.call(WORD_AT_14_00825C00, &args![TIMER_011F6394]).u32()
}

/// The frame's seconds (`0084d030` on the timer, `ST0`).
fn frame_seconds(e: &mut Engine) -> f64 {
    e.call(FRAME_SECONDS_0084D030, &args![TIMER_011F6394]).f64()
}

/// The value of a game setting: `00403e20` gives its address, the code reads the `float`.
fn setting_float_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE_00403E20, &args![setting]).u32();
    e.mem.u32(value)
}

/// The speech skill check of a topic's info, shared by `DoClick` and `LoadNextScreen`: the
/// info's conditions (`00500940`) are walked until one whose data (`00413f40`) is of type
/// 0xe; its actor value (data +4) is compared with the truncated comparison value
/// (`00681490`, `_ftol2`): the player's clamped value (`GetClampedActorValue` on its
/// +0xa4) below it counts `MiscStatManager::Increment(0x1f)` when `failed` allows it,
/// otherwise `Increment(0xc)`. Nothing happens when no such condition is found.
fn skill_check(e: &mut Engine, topic: u32, failed: bool) {
    let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
    let list = e.call(CONDITIONS_00500940, &args![info]).u32();
    let mut node = e.call(NODE_SELF, &args![list]).u32();
    let data = loop {
        if node == 0 || node_item(e, node) == 0 {
            return;
        }
        let item = node_item(e, node);
        let data = e.call(CONDITION_DATA_00413F40, &args![item]).u32();
        if data != 0 && e.mem.u16(data) == 0xe {
            break data;
        }
        node = node_next(e, node);
    };
    let actor_value = e.mem.u32(data + 4);
    let item = node_item(e, node);
    let value = e.call(CONDITION_VALUE_00681490, &args![item]).f64();
    let required = e.call(FTOL2, &args![value]).i32();
    let values = player(e).wrapping_add(0xa4);
    let current = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![values, actor_value])
        .i32();
    if current < required {
        if failed {
            e.call(MISC_STAT_INCREMENT, &args![0x1fu32]);
        }
    } else {
        e.call(MISC_STAT_INCREMENT, &args![0xcu32]);
    }
}

// Translated from 007618f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::GetClass` (Xbox PDB): `0x3f1`.
pub fn dialog_menu_get_class(_e: &mut Engine, _this: Ptr<DialogMenu>) -> u32 {
    DIALOG_MENU_CLASS
}

// Translated from 00761900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::AttachTileByID` (Xbox PDB; `RET 8`): stores `tile` in `pTiles[id]` for ids
/// 0 to 3 (signed).
pub fn dialog_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<DialogMenu>, id: i32, tile: u32) {
    if (0..4).contains(&id) {
        e.mem.set_u32(this.addr() + 0x30 + 4 * id as u32, tile);
    }
}

// Translated from 00761930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::_scalar_deleting_destructor_` (engine map; `RET 4`): the destructor
/// (`00761960`), then the memory freed when bit 0 of `flags` is set. Returns `this`.
pub fn dialog_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<DialogMenu>,
    flags: u32,
) -> Ptr<DialogMenu> {
    fn_00761960(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00761960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00761960 - the C++ exception frame (FS:[0] chain, unwind states 2 to 0 for the members) is not translated.
/// The `DialogMenu` destructor (unnamed in the engine map): the vtable, the byte at
/// `011d9514` cleared; when the open menu's `pDepthOfFieldInstance` is set, this menu's
/// instance is stopped (`ImageSpaceModifierInstance::Stop`) and cleared; `0083e510()`;
/// the open menu cleared; then `hSpeechFile` (`00483710`), the list box (`007643c0`) and
/// the `Menu` base (`00a1c520`) destroyed.
pub fn fn_00761960(e: &mut Engine, this: Ptr<DialogMenu>) {
    e.mem.set_u32(this.addr(), DIALOG_MENU_VTABLE);
    e.set_global(FLAG_011D9514, 0u8);
    let open = open_menu(e);
    if e.mem.u32(open + 0x12c) != 0 {
        let instance = e.get(this, DialogMenu::pDepthOfFieldInstance);
        e.call(IMAGE_SPACE_INSTANCE_STOP, &args![instance]);
        e.set(this, DialogMenu::pDepthOfFieldInstance, 0);
    }
    e.call(TOPICS_RELEASE_0083E510, &args![]);
    e.set_global(OPEN_MENU_011D9510, 0u32);
    e.call(SOUND_HANDLE_DESTROY, &args![this.addr() + 0x74]);
    fn_007643c0(e, Ptr::new(this.addr() + 0x40));
    e.call(MENU_DESTROY, &args![this]);
}

// Translated from 00761a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::Create` (Xbox PDB; `cdecl`, three words: the speaker, a word passed to the
/// topic manager, and an optional object the topics are for). Returns the menu's tile (or an
/// already open menu's tile, or 0).
///
/// An open `0x3f1` menu (`Tile::GetMenuByClass`) is returned at once; a null speaker logs a
/// message and returns 0. Otherwise it reads `dialog_menu.xml` (`Tile::ReadFile` on the
/// interface manager's +0x9c), takes its menu as the open menu (`011d9510`) and fails (the
/// menu deleted through vtable slot 0 when it has a tile) unless it is a `0x3f1` menu. The
/// tile becomes the menu's (`Menu::SetMenuTile`); without all four tiles a message is
/// logged and 0 returned. A trait `0xfa7` of 6006 or 102 gets the maximum depth as trait
/// `0xfad`. Then, in the exe's order: `pSpeaker`; the get-hit image space stopped; the
/// speaker's slots +0x1ac and +0x1b0 objects (when set) get the byte +0xe2 set
/// (`00762130`); for an actor (slot +0x100) with a package extra of type 0xf, the package's
/// float goes to `011a008c`; unless the current package is of type 0x1c aimed at the
/// player (slot +0x2c8), an actor stops its interrupt package (when slot +0x274(1) is
/// false), stops moving (when it is true) and the speaker's slot +0x280 starts the dialogue
/// with the player. The topic manager is initialized (the greeting sets the goodbye
/// state); the greeting's info is stored (`011d9518`); the player's and an actor's slot
/// +0x278(1) and the actor's process slot +0x288 run; `bIsIn3rdPerson` is set; unless the
/// goodbye state is 2 the player focuses on the speaker; without a depth-of-field instance
/// one is triggered from the distance between the two (`00439ef0`, `00457990`) and a game
/// setting. The player's iron sights end, the list box is initialized with
/// `"DM_TopicTemplate"`, the scroll bar and highlight box tiles get their traits, the three
/// trait ids are looked up, `"_ShowSubtitles"` is set (always, or for an actor that always
/// shows subtitles), the speaker's name goes to tile 1. Unless the tutorial manager shows
/// the dialogue tutorial (then `bShowingTutorial`), the first screen is loaded. The menu is
/// prepared for visibility, the dialogue flag set (`00762150`), the audio enters dialogue,
/// the speaker's slot +0xd0(1) runs and the gun scope is hidden.
pub fn dialog_menu_create(e: &mut Engine, speaker: u32, passed: u32, target: u32) -> u32 {
    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
    let existing = e
        .call(TILE_GET_MENU_BY_CLASS, &args![DIALOG_MENU_CLASS])
        .u32();
    if existing != 0 {
        return existing;
    }
    if speaker == 0 {
        e.call(DEBUG_MESSAGE, &args![NULL_SPEAKER_TEXT]);
        return 0;
    }
    let root = e.call(WORD_AT_9C_00586150, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.set_global(OPEN_MENU_011D9510, menu);
    let fail = |e: &mut Engine| {
        let menu = open_menu(e);
        if menu != 0 && menu_tile(e, menu) != 0 {
            let menu = open_menu(e);
            if menu != 0 {
                e.vcall(menu, 0, &args![1u32]);
            }
        }
        0
    };
    if menu == 0 {
        return fail(e);
    }
    let class = e.vcall(menu, 0x34, &args![]).u32();
    if class != fn_007620e0(e) {
        return fail(e);
    }
    let menu = open_menu(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
    let menu = open_menu(e);
    if !fn_007620f0(e, Ptr::new(menu)) {
        e.call(DEBUG_MESSAGE, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
    let first: f64 = e.global(VALUE_6006_DOUBLE);
    let second: f64 = e.global(VALUE_102_DOUBLE);
    let matches = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64() == first
        || e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64() == second;
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfadu32, x87_store32(depth), 1u32],
        );
    }
    let menu = open_menu(e);
    e.mem.set_u32(menu + 0x80, speaker);
    let get_hit = e.call(IMAGE_SPACE_GET_GET_HIT, &args![]).u32();
    e.call(IMAGE_SPACE_FORM_STOP, &args![get_hit]);
    for slot in [0x1acu32, 0x1b0] {
        if e.vcall(speaker, slot, &args![0u32]).u32() != 0 {
            let object = e.vcall(speaker, slot, &args![0u32]).u32();
            fn_00762130(e, Ptr::new(object), 1);
        }
    }
    let package = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![speaker]).u32();
    if e.vcall(speaker, 0x100, &args![]).bool() {
        let extra_list = e.call(EXTRA_LIST_005D43C0, &args![speaker]).u32();
        let extra = e.call(EXTRA_GET_PACKAGE, &args![extra_list]).u32();
        if extra != 0 && e.call(PACKAGE_TYPE_0041CA90, &args![extra]).u32() == 0xf {
            let value = e.call(PACKAGE_VALUE_00672850, &args![extra]).f64();
            e.set_global(ZOOM_PERCENT_011A008C, x87_store32(value));
        }
    }
    let aimed_at_player = package != 0
        && e.call(PACKAGE_TYPE_0041CA90, &args![package]).u32() == 0x1c
        && e.vcall(speaker, 0x2c8, &args![]).u32() == player(e);
    if !aimed_at_player {
        if e.vcall(speaker, 0x100, &args![]).bool() && !e.vcall(speaker, 0x274, &args![1u32]).bool()
        {
            e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![speaker, 0u32]);
        }
        if e.vcall(speaker, 0x274, &args![1u32]).bool() {
            e.call(ACTOR_STOP_MOVING, &args![speaker]);
        }
        let player = player(e);
        e.vcall(
            speaker,
            0x280,
            &args![player, 0u32, 0u32, 1u32, 0u32, 1u32, 0u32, 0u32, 0u32],
        );
    }
    let topics_for = if target != 0 { target } else { speaker };
    let menu = open_menu(e);
    let topics = e.mem.u32(menu + 0x70);
    if e.call(TOPICS_INITIALIZE, &args![topics, topics_for, passed])
        .bool()
    {
        let menu = open_menu(e);
        let greeting = e
            .call(TOPICS_GET_GREETING, &args![e.mem.u32(menu + 0x70)])
            .u32();
        let menu = open_menu(e);
        dialog_menu_set_goodbye_state(e, Ptr::new(menu), greeting);
    }
    let menu = open_menu(e);
    if e.call(TOPICS_GET_GREETING, &args![e.mem.u32(menu + 0x70)])
        .u32()
        != 0
    {
        let menu = open_menu(e);
        let greeting = e
            .call(TOPICS_GET_GREETING, &args![e.mem.u32(menu + 0x70)])
            .u32();
        let info = e.call(WORD_AT_18_009611E0, &args![greeting]).u32();
        e.set_global(LAST_INFO_011D9518, info);
    }
    e.vcall(player(e), 0x278, &args![1u32]);
    if e.vcall(speaker, 0x100, &args![]).bool() {
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(process, 0x288, &args![speaker, 2u32]);
        e.vcall(speaker, 0x278, &args![1u32]);
    }
    let third_person = e.call(PLAYER_004EAF60, &args![player(e)]).u32() as u8;
    let menu = open_menu(e);
    e.mem.set_u8(menu + 0x130, third_person);
    let menu = open_menu(e);
    if e.mem.u32(menu + 0x2c) != 2 {
        e.call(PLAYER_00951A10, &args![player(e), 1u32]);
        e.call(
            PLAYER_FOCUS_ON_ACTOR,
            &args![player(e), speaker, 0.0f32.to_bits(), 1u32],
        );
    }
    let menu = open_menu(e);
    if e.mem.u32(menu + 0x12c) == 0 {
        let distance = e.with_stack(0xc, |e, local| {
            let player_position = e.vcall(player(e), 0x1f4, &args![]).u32();
            let speaker_position = e.vcall(speaker, 0x1f4, &args![]).u32();
            let offset = e
                .call(
                    POSITION_00439EF0,
                    &args![speaker_position, local, player_position],
                )
                .u32();
            x87_store32(e.call(POSITION_00457990, &args![offset]).f64())
        });
        let setting = setting_float_bits(e, SETTING_011D46FC);
        let instance = e
            .call(
                DOF_TRIGGER,
                &args![
                    x87_quiet_bits(distance),
                    x87_quiet_bits(setting),
                    2u32,
                    0.0f32.to_bits(),
                    0.0f32.to_bits(),
                    0.0f32.to_bits()
                ],
            )
            .u32();
        let menu = open_menu(e);
        e.mem.set_u32(menu + 0x12c, instance);
        let menu = open_menu(e);
        let instance = e.mem.u32(menu + 0x12c);
        e.call(IMAGE_SPACE_005BBD60, &args![instance, 1u32]);
    }
    e.call(ACTOR_SET_IRON_SIGHTS, &args![player(e), 0u32, 0u32, 0u32]);
    e.call(PLAYER_STOP_IRON_SIGHTS_IMAGE_SPACE, &args![player(e)]);
    let menu = open_menu(e);
    let container = e.mem.u32(menu + 0x3c);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![menu + 0x40, container, TOPIC_TEMPLATE_TEXT],
    );
    for (name, trait_id, value) in [
        (SCROLLBAR_TEXT, 0xfa2u32, 0x28u32),
        (HIGHLIGHT_BOX_TEXT, 0xfa1, 0x28),
        (HIGHLIGHT_BOX_TEXT, 0xfb1, 0x3c0),
    ] {
        let menu = open_menu(e);
        let container = e.mem.u32(menu + 0x3c);
        let child = e
            .call(TILE_GET_CHILD_BY_NAME, &args![container, name])
            .u32();
        set_int(e, child, trait_id, value);
    }
    let id = e.call(TEXT_TO_TRAIT, &args![DIALOG_VISIBLE_TEXT]).u32();
    e.set_global(DIALOG_VISIBLE_TRAIT_011D9508, id);
    let id = e.call(TEXT_TO_TRAIT, &args![SHOWING_TEXT_TEXT]).u32();
    e.set_global(SHOWING_TEXT_TRAIT_011D9500, id);
    let id = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    e.set_global(CURRENT_VALUE_TRAIT_011D9504, id);
    let text = e.call(STRING_00408D60, &args![STRING_OWNER_011D8934]).u32();
    let show = e.mem.u8(text) != 0
        || (e.vcall(speaker, 0x100, &args![]).bool()
            && e.call(ACTOR_ALWAYS_SHOW_SUBTITLES, &args![speaker]).bool());
    let subtitles = e.call(TEXT_TO_TRAIT, &args![SHOW_SUBTITLES_TEXT]).u32();
    let menu = open_menu(e);
    let root = menu_tile(e, menu);
    set_int(e, root, subtitles, u32::from(show));
    let name = e.call(REFERENCE_NAME_0055D520, &args![speaker]).u32();
    let menu = open_menu(e);
    let name_tile = e.mem.u32(menu + 0x34);
    e.call(TILE_SET_STRING, &args![name_tile, 0xfc4u32, name, 1u32]);
    let mut load = true;
    if e.call(FLAG_005D4A40, &args![]).bool() {
        let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
        if !e
            .call(
                TUTORIAL_00718840,
                &args![interface.wrapping_add(0x4d4), 0x11u32],
            )
            .bool()
        {
            let menu = open_menu(e);
            e.mem.set_u8(menu + 0x139, 1);
            load = false;
        }
    }
    if load {
        let menu = open_menu(e);
        dialog_menu_load_next_screen(e, Ptr::new(menu), 0);
    }
    let menu = open_menu(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    fn_00762150(e);
    let audio = e.call(BS_AUDIO_INSTANCE, &args![]).u32();
    e.call(BS_AUDIO_ENTER_DIALOGUE, &args![audio]);
    e.vcall(speaker, 0xd0, &args![1u32]);
    e.call(INTERFACE_SET_GUN_SCOPE_VISIBLE, &args![0u32]);
    tile
}

// Translated from 007620e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the menu id `0x3f1` (`Create` compares the new menu's class
/// with it).
pub fn fn_007620e0(_e: &mut Engine) -> u32 {
    DIALOG_MENU_CLASS
}

// Translated from 007620f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): whether all four `pTiles` are set.
pub fn fn_007620f0(e: &mut Engine, this: Ptr<DialogMenu>) -> bool {
    (0..4u32).all(|index| e.mem.u32(this.addr() + 0x30 + 4 * index) != 0)
}

// Translated from 00762130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): stores the argument's low byte at +0xe2 of the object
/// (the objects the speaker's slots +0x1ac / +0x1b0 return).
pub fn fn_00762130(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0xe2), value);
}

// Translated from 00762150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): sets the byte at `011dcfa4`.
pub fn fn_00762150(e: &mut Engine) {
    e.set_global(FLAG_011DCFA4, 1u8);
}

// Translated from 00762160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::Close` (Xbox PDB; no arguments). With an open menu: an invalid choice list
/// (goodbye state 3) is logged; the speaker's slot +0x1ac / +0x1b0 objects (when set) get
/// their byte +0xe2 cleared; the speaker's dialogue stops (`StopCurrentDialogue`, slot
/// +0x278(0), the player's slot +0x278(0), slot +0x288); `011d9514` is cleared. The
/// speaker's process (when there is one) gets slot +0x2ec(0), and for an actor that
/// `00493bb0` does not hold, the actor's process slot +0x24(actor, 1); an actor's process
/// gets slot +0x650(1); an actor not in combat (slot +0x22c(0)) nor slot +0x2e8 gets slot
/// +0x4b4 with a game setting's float; an actor stops moving. The player's greet flag is
/// reset and its process gets slot +0x650(1). With a stored info and goodbye state 2 or 3,
/// unless the info's `00579200` says so, its result script runs (`RunResult(1, speaker)`)
/// and its topics are added, and for an actor that `00493bb0` does not hold, a true process
/// slot +0x24(actor, 1) is followed by slot +0x524(actor). The menu's tile gets trait
/// `0xfa3` 1 and fades out. Always: the audio leaves dialogue and the dialogue flag is
/// cleared (`007624c0`).
pub fn dialog_menu_close(e: &mut Engine) {
    if open_menu(e) != 0 {
        let speaker = e.mem.u32(open_menu(e) + 0x80);
        let actor = if e.vcall(speaker, 0x100, &args![]).bool() {
            speaker
        } else {
            0
        };
        let info: u32 = e.global(LAST_INFO_011D9518);
        let state = e.mem.u32(open_menu(e) + 0x2c);
        let ending = state == 2 || e.mem.u32(open_menu(e) + 0x2c) == 3;
        if e.mem.u32(open_menu(e) + 0x2c) == 3 {
            e.call(DEBUG_MESSAGE, &args![INVALID_CHOICE_TEXT]);
        }
        for slot in [0x1acu32, 0x1b0] {
            if e.vcall(speaker, slot, &args![0u32]).u32() != 0 {
                let object = e.vcall(speaker, slot, &args![0u32]).u32();
                fn_00762130(e, Ptr::new(object), 0);
            }
        }
        e.call(MOBILE_STOP_CURRENT_DIALOGUE, &args![speaker]);
        e.vcall(speaker, 0x278, &args![0u32]);
        let player = player(e);
        e.vcall(player, 0x278, &args![0u32]);
        e.vcall(speaker, 0x288, &args![]);
        e.set_global(FLAG_011D9514, 0u8);
        if e.call(GET_PROCESS, &args![speaker]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![speaker]).u32();
            e.vcall(process, 0x2ec, &args![0u32]);
            if actor != 0 && !e.call(ACTOR_00493BB0, &args![actor]).bool() {
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                e.vcall(process, 0x24, &args![actor, 1u32]);
            }
        }
        if actor != 0 && e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, 0x650, &args![1u32]);
        }
        if e.call(GET_PROCESS, &args![speaker]).u32() != 0
            && actor != 0
            && !e.vcall(actor, 0x22c, &args![0u32]).bool()
            && !e.vcall(actor, 0x2e8, &args![]).bool()
        {
            let process = e.call(GET_PROCESS, &args![speaker]).u32();
            let value = setting_float_bits(e, SETTING_011CD450);
            e.vcall(process, 0x4b4, &args![x87_quiet_bits(value)]);
        }
        if actor != 0 {
            e.call(ACTOR_STOP_MOVING, &args![actor]);
        }
        e.call(PLAYER_RESET_GREET_FLAG, &args![player]);
        let process = e.call(GET_PROCESS, &args![player]).u32();
        e.vcall(process, 0x650, &args![1u32]);
        if info != 0 && ending && !e.call(INFO_00579200, &args![info]).bool() {
            e.call(INFO_RUN_RESULT, &args![info, 1u32, speaker]);
            e.call(INFO_ADD_TOPIC_LIST, &args![info]);
            if actor != 0
                && !e.call(ACTOR_00493BB0, &args![actor]).bool()
                && e.call(GET_PROCESS, &args![actor]).u32() != 0
            {
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                if e.vcall(process, 0x24, &args![actor, 1u32]).bool() {
                    let process = e.call(GET_PROCESS, &args![actor]).u32();
                    e.vcall(process, 0x524, &args![actor]);
                }
            }
        }
        let menu = open_menu(e);
        let root = menu_tile(e, menu);
        set_int(e, root, 0xfa3, 1);
        let menu = open_menu(e);
        e.call(MENU_START_FADE_OUT, &args![menu]);
    }
    let audio = e.call(BS_AUDIO_INSTANCE, &args![]).u32();
    e.call(BS_AUDIO_EXIT_DIALOGUE, &args![audio]);
    fn_007624c0(e);
}

// Translated from 007624c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): clears the byte at `011dcfa4`.
pub fn fn_007624c0(e: &mut Engine) {
    e.set_global(FLAG_011DCFA4, 0u8);
}

// Translated from 007624d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the open menu's speaker (`004fb070`, its word +0x80), or 0
/// without an open menu.
pub fn fn_007624d0(e: &mut Engine) -> u32 {
    let menu = open_menu(e);
    if menu == 0 {
        return 0;
    }
    e.call(WORD_AT_80_004FB070, &args![menu]).u32()
}

// Translated from 007624f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::DoClick` (Xbox PDB; `RET 8`: the clicked id and tile). Nothing in dialogue
/// states 0 and 4, nor within 500 ms of `uiIgnoreInputStartTime` (which is cleared once
/// that has passed). A click on one of the list box's topics (id -1) selects the topic at
/// the tile's index (`0083e430`); when it is selected, the current topic's byte +0x25 is
/// cleared (its old value kept), and its info is checked: a speech challenge
/// (`007627c0`) runs `DoSpeechChallengeCheck` (success takes the new current topic;
/// failure gives the player's slot +0x488 the topic manager's +0x14 count when positive and
/// counts `MiscStatManager::Increment(0xc)`), and marks the next screen as challenged;
/// otherwise a skill prompt (`007627e0` or `00762820`) runs the skill check (a failure is
/// counted only when the byte was set). Then the goodbye state, the topic processed,
/// `bResponseShown` cleared, the info stored and the next screen loaded into
/// `eDialogState`. Otherwise, while the menu's `"_ShowingText"` is true, id 0 sets state 1.
pub fn dialog_menu_do_click(e: &mut Engine, this: Ptr<DialogMenu>, id: i32, tile: u32) {
    let mut challenged = 0u8;
    let state = e.get(this, DialogMenu::eDialogState);
    if state == 0 || e.get(this, DialogMenu::eDialogState) == 4 {
        return;
    }
    if e.get(this, DialogMenu::uiIgnoreInputStartTime) != 0 {
        let elapsed = now(e).wrapping_sub(e.get(this, DialogMenu::uiIgnoreInputStartTime));
        if elapsed < 500 {
            return;
        }
        e.set(this, DialogMenu::uiIgnoreInputStartTime, 0);
    }
    if id == -1
        && e.call(LIST_BOX_BELONGS_TO, &args![this.addr() + 0x40, tile])
            .bool()
    {
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
        let index = e.call(FTOL2, &args![value]).u32();
        let topics = e.get(this, DialogMenu::pTopicManager);
        if e.call(TOPICS_SELECT_0083E430, &args![topics, index]).bool() {
            let mut topic = e
                .call(
                    TOPICS_GET_CURRENT_TOPIC,
                    &args![e.get(this, DialogMenu::pTopicManager)],
                )
                .u32();
            let said = e.mem.u8(topic + 0x25);
            e.mem.set_u8(topic + 0x25, 0);
            let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
            if fn_007627c0(e, Ptr::new(info)) {
                challenged = 1;
                let topics = e.get(this, DialogMenu::pTopicManager);
                if e.call(TOPICS_DO_SPEECH_CHALLENGE_CHECK, &args![topics, topic])
                    .bool()
                {
                    topic = e
                        .call(
                            TOPICS_GET_CURRENT_TOPIC,
                            &args![e.get(this, DialogMenu::pTopicManager)],
                        )
                        .u32();
                } else {
                    let topics = e.mem.u32(open_menu(e) + 0x70);
                    if e.call(WORD_AT_14_00825C00, &args![topics]).i32() > 0 {
                        let topics = e.mem.u32(open_menu(e) + 0x70);
                        let count = e.call(WORD_AT_14_00825C00, &args![topics]).u32();
                        let player = player(e);
                        e.vcall(player, 0x488, &args![count]);
                        e.call(MISC_STAT_INCREMENT, &args![0xcu32]);
                    } else {
                        e.call(MISC_STAT_INCREMENT, &args![0xcu32]);
                    }
                }
            } else {
                let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                let prompt = fn_007627e0(e, Ptr::new(info)) != 0 || {
                    let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                    fn_00762820(e, Ptr::new(info)) != 0
                };
                if prompt {
                    skill_check(e, topic, said != 0);
                }
            }
            dialog_menu_set_goodbye_state(e, this, topic);
            let topics = e.get(this, DialogMenu::pTopicManager);
            e.call(TOPICS_PROCESS_CURRENT_TOPIC, &args![topics]);
            e.set(this, DialogMenu::bResponseShown, 0);
            let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
            e.set_global(LAST_INFO_011D9518, info);
            let state = dialog_menu_load_next_screen(e, this, challenged);
            e.set(this, DialogMenu::eDialogState, state);
            return;
        }
    }
    let showing = e.global::<u32>(SHOWING_TEXT_TRAIT_011D9500);
    let root = menu_tile(e, this.addr());
    if e.call(TILE_IS_TRUE, &args![root, showing]).bool() && id == 0 {
        e.set(this, DialogMenu::eDialogState, 1);
    }
}

// Translated from 007627c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a topic info): bit 7 of the byte at +0x25 (`MOVSX` then `AND
/// 0x80`), in `AL`.
pub fn fn_007627c0(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u8(this.addr().wrapping_add(0x25)) as i8 as i32) & 0x80 != 0
}

// Translated from 007627e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a topic info): its prompt form (+0x40) when set and not of form
/// type 0x56 (`00401170`), else 0.
pub fn fn_007627e0(e: &mut Engine, this: Ptr) -> u32 {
    let prompt = e.mem.u32(this.addr().wrapping_add(0x40));
    if prompt != 0 && e.call(FORM_TYPE_00401170, &args![prompt]).u32() != 0x56 {
        e.mem.u32(this.addr().wrapping_add(0x40))
    } else {
        0
    }
}

// Translated from 00762820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a topic info): its prompt form (+0x40) when set and of form type
/// 0x56 (`00401170`), else 0.
pub fn fn_00762820(e: &mut Engine, this: Ptr) -> u32 {
    let prompt = e.mem.u32(this.addr().wrapping_add(0x40));
    if prompt != 0 && e.call(FORM_TYPE_00401170, &args![prompt]).u32() == 0x56 {
        e.mem.u32(this.addr().wrapping_add(0x40))
    } else {
        0
    }
}

// Translated from 00762860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::SetGoodbyeState` (Xbox PDB; `RET 4`, a topic): a topic whose info has bit
/// 0 of +0x25 (`00619df0`) sets `eGoodbyeState` 2; otherwise a topic whose +0x28 is
/// `TESTopic::GetTopic(1, 2)` sets it 1; else it is left.
pub fn dialog_menu_set_goodbye_state(e: &mut Engine, this: Ptr<DialogMenu>, topic: u32) {
    let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
    if e.call(FLAG_00619DF0, &args![info]).bool() {
        e.set(this, DialogMenu::eGoodbyeState, 2);
        return;
    }
    let form = e.call(WORD_AT_28_0045CD60, &args![topic]).u32();
    if form == e.call(TES_TOPIC_GET_TOPIC, &args![1u32, 2u32]).u32() {
        e.set(this, DialogMenu::eGoodbyeState, 1);
    }
}

/// Closes the menu from `DoIdle`: `Close`, then `fElapsedSecs` 0 and state 4.
fn close_from_idle(e: &mut Engine, this: Ptr<DialogMenu>) {
    dialog_menu_close(e);
    e.set(this, DialogMenu::fElapsedSecs, 0.0);
    e.set(this, DialogMenu::eDialogState, 4);
}

/// `FCOMP double; FNSTSW; TEST AH,0x44; JP`: on only when the two are equal (ordered).
fn equal(a: f64, b: f64) -> bool {
    a == b
}

// Translated from 00762950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::DoIdle` (Xbox PDB). Nothing while paused or the console is shown. Unless
/// already closing, the menu closes (`Close`, state 4) when the speaker has no process or
/// its slot +0x610 is neither 2 nor below 4, and when the process's +0x28 word is set.
/// Then by `eDialogState`:
///
/// - 0 (fading in): with the tutorial shown and no time elapsed, the tutorial manager shows
///   message `0x11` for this menu (the game setting's value truncated, times 1400; the
///   result goes to `bShowingTutorial`), and without it the speaker's byte +0x7c is set and
///   the first screen loaded; without the tutorial a zoom of at least 0.1 shows the menu
///   (`"_DialogVisible"` 1). The elapsed time grows by the frame's seconds and the zoom is
///   `0040ebd0(elapsed / setting, 1.0)`; at 1.0, without the tutorial, the time is reset and
///   the state becomes 3; with it, once the tutorial manager is done and this is the top
///   menu, the menu is shown, the speaker's byte +0x7c set, the screen loaded and the state
///   becomes 3.
/// - 1 (speaking): while the speaker's byte +0x7c is clear, its dialogue stops and, 500 ms
///   after the first time (kept at `011d9594`), `008bc590(0084e3a0(speaker), 1)`; once it
///   is set, the next screen's state is taken and the start time cleared.
/// - 2: in a service menu with `bDoServiceLoad`, state 5.
/// - 3: the countdown at `011d951c` runs down by the frame's seconds; once it is not above
///   0, a speaker with byte +0x7c set moves to state 1 with the time noted.
/// - 4 (fading out): the zoom is `1 - 0040ebd0(elapsed / setting, 1.0)`; at 0.0 the two
///   bytes `011f21d0`/`011f21d1` are set (the second to whether the player was not in third
///   person), the camera is reset (focus with 0.0, `UpdateCamera`, `0063f790(0.0)`), the
///   menu's tile deleted, and it returns.
/// - 5: with `bDoServiceLoad` and the menu hidden (`xFadeState` 4), the next screen is
///   loaded, `bDoServiceLoad` cleared and the state becomes 2.
///
/// Last, the speaker in dialogue with the player gets slot +0x264(frame seconds, 0, 0),
/// and the player focuses on the speaker with the zoom scaled by `011a008c / 100` (1 when
/// it is 0), the camera updated and `0063f790` given a game setting's value times the zoom.
pub fn dialog_menu_do_idle(e: &mut Engine, this: Ptr<DialogMenu>) {
    if e.get(this, DialogMenu::bPaused) != 0 {
        return;
    }
    if e.call(INTERFACE_IS_CONSOLE_VISIBLE, &args![]).bool() {
        return;
    }
    let speaker_of = |e: &Engine| e.get(this, DialogMenu::pSpeaker);
    if e.get(this, DialogMenu::eDialogState) != 4 {
        let close = e.call(GET_PROCESS, &args![speaker_of(e)]).u32() == 0 || {
            let process = e.call(GET_PROCESS, &args![speaker_of(e)]).u32();
            e.vcall(process, 0x610, &args![]).u32() == 2 || {
                let process = e.call(GET_PROCESS, &args![speaker_of(e)]).u32();
                e.vcall(process, 0x610, &args![]).i32() >= 4
            }
        };
        if close {
            close_from_idle(e, this);
        }
    }
    if e.call(GET_PROCESS, &args![speaker_of(e)]).u32() != 0 {
        let process = e.call(GET_PROCESS, &args![speaker_of(e)]).u32();
        if e.call(WORD_AT_28_0045CD60, &args![process]).u32() != 0
            && e.get(this, DialogMenu::eDialogState) != 4
        {
            close_from_idle(e, this);
        }
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let one: f64 = e.global(ONE_DOUBLE);
    match e.get(this, DialogMenu::eDialogState) {
        0 => {
            let elapsed_bits = e.mem.u32(this.addr() + 0x108);
            if e.get(this, DialogMenu::bShowingTutorial) != 0
                && equal(x87_load32(elapsed_bits), zero)
            {
                let value = setting_float_bits(e, SETTING_011D3EE0);
                let whole = x87_fistp64_chop(X87Ext::load32(value)) as i32;
                let id = whole.wrapping_mul(0x578);
                let class = fn_007620e0(e);
                let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
                let shown = e
                    .call(
                        TUTORIAL_SHOW_MESSAGE,
                        &args![interface.wrapping_add(0x4d4), 0x11u32, class, id],
                    )
                    .u32() as u8;
                e.set(this, DialogMenu::bShowingTutorial, shown);
                if e.get(this, DialogMenu::bShowingTutorial) == 0 {
                    let speaker = speaker_of(e);
                    if speaker != 0 {
                        e.mem.set_u8(speaker + 0x7c, 1);
                    }
                    dialog_menu_load_next_screen(e, this, 0);
                }
            } else if e.get(this, DialogMenu::bShowingTutorial) == 0 {
                let tenth: f64 = e.global(TENTH_DOUBLE);
                let zoom_bits = e.mem.u32(this.addr() + 0x128);
                // `TEST AH,1; JNZ`: skipped when below or unordered.
                if !matches!(
                    x87_load32(zoom_bits).partial_cmp(&tenth),
                    Some(std::cmp::Ordering::Less) | None
                ) {
                    let visible = e.global::<u32>(DIALOG_VISIBLE_TRAIT_011D9508);
                    let root = menu_tile(e, this.addr());
                    set_int(e, root, visible, 1);
                }
            }
            advance_zoom(e, this, SETTING_011D3EE0, false);
            let zoom_bits = e.mem.u32(this.addr() + 0x128);
            if equal(x87_load32(zoom_bits), one) {
                if e.get(this, DialogMenu::bShowingTutorial) == 0 {
                    e.set(this, DialogMenu::fElapsedSecs, 0.0);
                    e.set(this, DialogMenu::eDialogState, 3);
                } else {
                    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
                    if e.call(
                        TUTORIAL_00718840,
                        &args![interface.wrapping_add(0x4d4), 0x11u32],
                    )
                    .bool()
                        && e.call(INTERFACE_IS_TOP_MENU_ID, &args![DIALOG_MENU_CLASS])
                            .bool()
                    {
                        e.set(this, DialogMenu::fElapsedSecs, 0.0);
                        let visible = e.global::<u32>(DIALOG_VISIBLE_TRAIT_011D9508);
                        let root = menu_tile(e, this.addr());
                        set_int(e, root, visible, 1);
                        let speaker = speaker_of(e);
                        if speaker != 0 {
                            e.mem.set_u8(speaker + 0x7c, 1);
                        }
                        dialog_menu_load_next_screen(e, this, 0);
                        e.set(this, DialogMenu::eDialogState, 3);
                    }
                }
            }
        }
        1 => {
            let speaker = speaker_of(e);
            if e.mem.u8(speaker + 0x7c) == 0 {
                e.call(MOBILE_STOP_CURRENT_DIALOGUE, &args![speaker]);
                if e.global::<u32>(START_TIME_011D9594) == 0 {
                    let time = now(e);
                    e.set_global(START_TIME_011D9594, time);
                }
                let elapsed = now(e).wrapping_sub(e.global::<u32>(START_TIME_011D9594));
                let limit: f64 = e.global(FIVE_HUNDRED_DOUBLE);
                // `FILD qword` of the unsigned difference; `TEST AH,1; JNZ`: on unless below.
                if !matches!(
                    f64::from(elapsed).partial_cmp(&limit),
                    Some(std::cmp::Ordering::Less) | None
                ) {
                    let object = e.call(OBJECT_0084E3A0, &args![speaker_of(e)]).u32();
                    e.call(ACTOR_008BC590, &args![object, 1u32]);
                }
            } else {
                e.set(this, DialogMenu::fElapsedSecs, 0.0);
                let state = dialog_menu_load_next_screen(e, this, 0);
                e.set(this, DialogMenu::eDialogState, state);
                e.set_global(START_TIME_011D9594, 0u32);
            }
        }
        2 => {
            if e.get(this, DialogMenu::bInServiceMenu) != 0
                && e.get(this, DialogMenu::bDoServiceLoad) != 0
            {
                e.set(this, DialogMenu::eDialogState, 5);
            }
        }
        3 => {
            let countdown_bits: u32 = e.global(COUNTDOWN_011D951C);
            if above(x87_load32(countdown_bits), zero) {
                let seconds = frame_seconds(e);
                let countdown_bits: u32 = e.global(COUNTDOWN_011D951C);
                let left = x87_store32(x87_sub(x87_m32(countdown_bits), seconds));
                e.set_global(COUNTDOWN_011D951C, left);
            } else {
                let speaker = speaker_of(e);
                if speaker != 0 && e.mem.u8(speaker + 0x7c) != 0 {
                    e.set(this, DialogMenu::fElapsedSecs, 0.0);
                    e.set(this, DialogMenu::eDialogState, 1);
                    let time = now(e);
                    e.set(this, DialogMenu::uiIgnoreInputStartTime, time);
                }
            }
        }
        4 => {
            advance_zoom(e, this, SETTING_011D243C, true);
            let zoom_bits = e.mem.u32(this.addr() + 0x128);
            if equal(x87_load32(zoom_bits), zero) {
                e.set_global(FLAG_011F21D0, 1u8);
                let first_person = u8::from(e.get(this, DialogMenu::bIsIn3rdPerson) == 0);
                e.set_global(FLAG_011F21D1, first_person);
                e.call(
                    PLAYER_FOCUS_ON_ACTOR,
                    &args![player(e), speaker_of(e), 0.0f32.to_bits(), 1u32],
                );
                e.call(PLAYER_UPDATE_CAMERA, &args![player(e), 0u32, 0u32]);
                let instance = e.get(this, DialogMenu::pDepthOfFieldInstance);
                e.call(DOF_0063F790, &args![instance, 0.0f32.to_bits()]);
                let root = menu_tile(e, this.addr());
                if root != 0 {
                    e.vcall(root, 0, &args![1u32]);
                }
                return;
            }
        }
        5 => {
            if e.get(this, DialogMenu::bDoServiceLoad) != 0 && e.mem.u32(this.addr() + 0x24) == 4 {
                // `Menu::xFadeState` (Xbox PDB) +0x24: 4 is `FADE_HIDDEN`.
                dialog_menu_load_next_screen(e, this, 0);
                e.set(this, DialogMenu::bDoServiceLoad, 0);
                e.set(this, DialogMenu::eDialogState, 2);
            }
        }
        _ => {}
    }
    if e.call(MOBILE_IS_IN_DIALOGUE_WITH_PLAYER, &args![player(e)])
        .bool()
    {
        let seconds = x87_store32(frame_seconds(e));
        let speaker = speaker_of(e);
        e.vcall(speaker, 0x264, &args![seconds, 0u32, 0u32]);
    }
    let percent_bits: u32 = e.global(ZOOM_PERCENT_011A008C);
    let scale = if equal(x87_load32(percent_bits), zero) {
        1.0
    } else {
        let hundred: f64 = e.global(HUNDRED_DOUBLE);
        x87_div(x87_load32(percent_bits), hundred)
    };
    let closing = u32::from(e.get(this, DialogMenu::eDialogState) == 4);
    let zoom_bits = e.mem.u32(this.addr() + 0x128);
    let zoom = x87_store32(x87_mul(x87_load32(zoom_bits), scale));
    let speaker = speaker_of(e);
    e.call(
        PLAYER_FOCUS_ON_ACTOR,
        &args![player(e), speaker, x87_quiet_bits(zoom), closing],
    );
    e.call(PLAYER_UPDATE_CAMERA, &args![player(e), 0u32, 0u32]);
    let setting = setting_float_bits(e, SETTING_011D318C);
    let zoom_bits = e.mem.u32(this.addr() + 0x128);
    let value = x87_store32(x87_mul(x87_load32(setting), x87_m32(zoom_bits)));
    let instance = e.get(this, DialogMenu::pDepthOfFieldInstance);
    e.call(DOF_0063F790, &args![instance, x87_quiet_bits(value)]);
}

/// `DoIdle`'s zoom step: the elapsed time grows by the frame's seconds (`FADD`, stored),
/// the ratio `elapsed / setting` is stored as a `float` and clamped by `0040ebd0(ratio,
/// 1.0)`; the zoom is the result (fading in) or `1 - result` (fading out, `FLD1; FSUBRP`).
fn advance_zoom(e: &mut Engine, this: Ptr<DialogMenu>, setting: u32, out: bool) {
    let seconds = frame_seconds(e);
    let elapsed_bits = e.mem.u32(this.addr() + 0x108);
    let elapsed = x87_store32(x87_add(seconds, x87_m32(elapsed_bits)));
    e.mem.set_u32(this.addr() + 0x108, elapsed);
    let duration = setting_float_bits(e, setting);
    let elapsed_bits = e.mem.u32(this.addr() + 0x108);
    let ratio = x87_store32(x87_div(x87_load32(elapsed_bits), x87_m32(duration)));
    let clamped = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(ratio), 1.0f32.to_bits()],
        )
        .f64();
    let zoom = if out {
        x87_store32(x87_sub(1.0, clamped))
    } else {
        x87_store32(clamped)
    };
    e.mem.set_u32(this.addr() + 0x128, zoom);
}

/// `0040ebd0` (`cdecl`, two `float`s, `ST0`).
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;

// Translated from 00762ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00762ff0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the sound handle and the two allocations) and the /GS cookie check (00ec408c) are not translated.
/// `DialogMenu::LoadNextScreen` (Xbox PDB; `RET 4`, a byte: the screen follows a speech
/// challenge). Returns the new dialogue state (2 by default).
///
/// The current topic and its current response are taken; `strCurrTopicID` becomes
/// `"%s (%d)"` of the topic's form (its slot +0x130 and `0084e3a0`) or `"----"`.
///
/// With a response: `bResponseShown` is set, the topic moves on (`0083e110`), the speaker
/// says it (slot +0x264(0.0, response, 1)) and its process's sound (slot +0x48c into a
/// local handle) becomes `hSpeechFile`. Outside a service menu the response's text (or the
/// two challenge strings of the game settings at `011d27fc` / `011d3fe8` with it, the first
/// when `00833c20` of the topic manager is true) goes to tile 2 (`0xfc4`), the menu shows
/// the text (`"_ShowingText"` 1) and the state is 3.
///
/// Without one: a topic without responses is logged unless `bResponseShown`. A stored info
/// (`011d9518`) with goodbye state other than 2 runs its result (unless `00579200`); when
/// `0061af30` then gives a new info and there is a topic, a new topic list (`0096a2d0`) with
/// the greeting and a new `MenuTopic` (`0083db00`, started by `0083e0c0`) is set, the first
/// topic taken, the skill or speech check of the new topic made as in `DoClick`, the goodbye
/// state set, the topic processed, `bResponseShown` cleared, its info stored, and the
/// result of loading the next screen again is returned. Without a new info the stored info
/// is cleared. Then, outside a service menu or in state 5: the next topic list is loaded; no
/// first topic with goodbye state 0 makes it 3; a goodbye state closes the menu (state 4),
/// otherwise the text is hidden and the topics list loaded (state 2).
///
/// When paused, `ePausedState` takes the state.
pub fn dialog_menu_load_next_screen(e: &mut Engine, this: Ptr<DialogMenu>, challenged: u8) -> u32 {
    let topics_of = |e: &Engine| e.get(this, DialogMenu::pTopicManager);
    let topic = e.call(TOPICS_GET_CURRENT_TOPIC, &args![topics_of(e)]).u32();
    let response = if topic != 0 {
        e.call(TOPIC_GET_CURRENT_RESPONSE, &args![topic]).u32()
    } else {
        0
    };
    let mut state = 2u32;
    let topic_id = this.addr() + 0x84;
    if topic != 0 {
        let form = e.call(WORD_AT_28_0045CD60, &args![topic]).u32();
        let form_again = e.call(WORD_AT_28_0045CD60, &args![topic]).u32();
        let number = e.call(OBJECT_0084E3A0, &args![form_again]).u32();
        let name = e.vcall(form, 0x130, &args![]).u32();
        e.call(
            FORMAT_00406D00,
            &args![topic_id, 0x80u32, TOPIC_ID_FORMAT, name, number],
        );
    } else {
        e.call(COPY_00406D30, &args![topic_id, 0x80u32, NO_TOPIC_ID_TEXT]);
    }
    e.with_stack(0x220, |e, frame| {
        let buffer = frame.addr();
        let sound = buffer + 0x200;
        let found_byte = buffer + 0x20c;
        let greeting_slot = buffer + 0x210;
        let topic_slot = buffer + 0x214;
        if response != 0 {
            e.set(this, DialogMenu::bResponseShown, 1);
            fn_0083e110(e, Ptr::new(topic));
            let speaker = e.get(this, DialogMenu::pSpeaker);
            e.vcall(speaker, 0x264, &args![0.0f32.to_bits(), response, 1u32]);
            let process = e
                .call(GET_PROCESS, &args![e.get(this, DialogMenu::pSpeaker)])
                .u32();
            let handle = e.vcall(process, 0x48c, &args![sound, 0u32]).u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![this.addr() + 0x74, handle]);
            e.call(SOUND_HANDLE_DESTROY, &args![sound]);
            if e.get(this, DialogMenu::bInServiceMenu) == 0 {
                let response_text = |e: &mut Engine| {
                    let slot = e.call(NODE_SELF, &args![response]).u32();
                    e.call(WORD_AT_0_00559450, &args![slot]).u32()
                };
                if challenged != 0 {
                    let setting = if e.call(RADIO_00833C20, &args![topics_of(e)]).bool() {
                        SETTING_011D27FC
                    } else {
                        SETTING_011D3FE8
                    };
                    if response_text(e) != 0 {
                        let text = response_text(e);
                        let prefix = e.call(SETTING_STRING_00403DF0, &args![setting]).u32();
                        e.call(
                            FORMAT_00406D00,
                            &args![buffer, 0x200u32, TWO_STRINGS_FORMAT, prefix, text],
                        );
                    } else {
                        let prefix = e.call(SETTING_STRING_00403DF0, &args![setting]).u32();
                        e.call(
                            FORMAT_00406D00,
                            &args![buffer, 0x200u32, ONE_STRING_FORMAT, prefix],
                        );
                    }
                } else if response_text(e) != 0 {
                    let text = response_text(e);
                    e.call(COPY_00406D30, &args![buffer, 0x200u32, text]);
                } else {
                    e.call(COPY_00406D30, &args![buffer, 0x200u32, SPACE_TEXT]);
                }
                let text_tile = e.mem.u32(this.addr() + 0x38);
                e.call(TILE_SET_STRING, &args![text_tile, 0xfc4u32, buffer, 1u32]);
                let showing = e.global::<u32>(SHOWING_TEXT_TRAIT_011D9500);
                let root = menu_tile(e, this.addr());
                set_int(e, root, showing, 1);
                state = 3;
            }
        } else {
            if e.get(this, DialogMenu::bResponseShown) == 0 {
                let name = if topic != 0 {
                    let form = e.call(WORD_AT_28_0045CD60, &args![topic]).u32();
                    e.call(STRING_CHARS_00408DA0, &args![form.wrapping_add(0x18)])
                        .u32()
                } else {
                    NULL_TEXT
                };
                e.call(DEBUG_MESSAGE, &args![NO_RESPONSES_TEXT, name]);
            }
            let info: u32 = e.global(LAST_INFO_011D9518);
            if info != 0
                && e.get(this, DialogMenu::eGoodbyeState) != 2
                && !e
                    .call(INFO_00579200, &args![e.global::<u32>(LAST_INFO_011D9518)])
                    .bool()
            {
                let speaker = e.get(this, DialogMenu::pSpeaker);
                let info: u32 = e.global(LAST_INFO_011D9518);
                e.call(INFO_RUN_RESULT, &args![info, 1u32, speaker]);
                let object = e.call(OBJECT_0084E3A0, &args![topics_of(e)]).u32();
                let player = player(e);
                let info: u32 = e.global(LAST_INFO_011D9518);
                let next = e
                    .call(
                        INFO_0061AF30,
                        &args![info, found_byte, object, player, 0u32, 0u32],
                    )
                    .u32();
                if next != 0 && topic != 0 {
                    let block = e.call(ALLOCATE, &args![8u32]).u32();
                    let list = if block != 0 {
                        e.call(LIST_CONSTRUCT, &args![block]).u32()
                    } else {
                        0
                    };
                    let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
                    let new_topic = if block != 0 {
                        let speaker = e.get(this, DialogMenu::pSpeaker);
                        let form = e.call(WORD_AT_28_0045CD60, &args![topic]).u32();
                        let count = e.call(WORD_AT_14_00825C00, &args![topic]).u32();
                        e.call(
                            TOPIC_CONSTRUCT_0083DB00,
                            &args![block, count, form, next, speaker, 0u32],
                        )
                        .u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(topic_slot, new_topic);
                    fn_0083e0c0(e, Ptr::new(new_topic));
                    let greeting = e.call(TOPICS_GET_GREETING, &args![topics_of(e)]).u32();
                    e.mem.set_u32(greeting_slot, greeting);
                    e.call(LIST_APPEND, &args![list, greeting_slot]);
                    e.call(LIST_APPEND, &args![list, topic_slot]);
                    e.call(TOPICS_SET_LIST_0083F1A0, &args![topics_of(e), list]);
                    e.call(TOPICS_FIRST_TOPIC, &args![topics_of(e), 1u32]);
                    let current = e.mem.u32(topic_slot);
                    let info = e.call(WORD_AT_18_009611E0, &args![current]).u32();
                    if fn_007627c0(e, Ptr::new(info)) {
                        let current = e.mem.u32(topic_slot);
                        if e.call(
                            TOPICS_DO_SPEECH_CHALLENGE_CHECK,
                            &args![topics_of(e), current],
                        )
                        .bool()
                        {
                            let current =
                                e.call(TOPICS_GET_CURRENT_TOPIC, &args![topics_of(e)]).u32();
                            e.mem.set_u32(topic_slot, current);
                        } else if e.call(WORD_AT_14_00825C00, &args![topics_of(e)]).i32() > 0 {
                            let count = e.call(WORD_AT_14_00825C00, &args![topics_of(e)]).u32();
                            let player = self::player(e);
                            e.vcall(player, 0x488, &args![count]);
                            e.call(MISC_STAT_INCREMENT, &args![0xcu32]);
                        } else {
                            e.call(MISC_STAT_INCREMENT, &args![0xcu32]);
                        }
                    } else {
                        let current = e.mem.u32(topic_slot);
                        let info = e.call(WORD_AT_18_009611E0, &args![current]).u32();
                        let prompt = fn_007627e0(e, Ptr::new(info)) != 0 || {
                            let info = e.call(WORD_AT_18_009611E0, &args![current]).u32();
                            fn_00762820(e, Ptr::new(info)) != 0
                        };
                        if prompt {
                            skill_check(e, current, true);
                        }
                    }
                    let current = e.mem.u32(topic_slot);
                    dialog_menu_set_goodbye_state(e, this, current);
                    e.call(TOPICS_PROCESS_CURRENT_TOPIC, &args![topics_of(e)]);
                    e.set(this, DialogMenu::bResponseShown, 0);
                    let current = e.mem.u32(topic_slot);
                    let info = e.call(WORD_AT_18_009611E0, &args![current]).u32();
                    e.set_global(LAST_INFO_011D9518, info);
                    return Some(dialog_menu_load_next_screen(e, this, challenged));
                }
                e.set_global(LAST_INFO_011D9518, 0u32);
            }
            if e.get(this, DialogMenu::bInServiceMenu) == 0
                || e.get(this, DialogMenu::eDialogState) == 5
            {
                e.call(TOPICS_LOAD_NEXT_TOPIC_LIST, &args![topics_of(e)]);
                if !e
                    .call(TOPICS_FIRST_TOPIC, &args![topics_of(e), 1u32])
                    .bool()
                    && e.get(this, DialogMenu::eGoodbyeState) == 0
                {
                    e.set(this, DialogMenu::eGoodbyeState, 3);
                }
                if e.get(this, DialogMenu::eGoodbyeState) != 0 {
                    dialog_menu_close(e);
                    e.set(this, DialogMenu::fElapsedSecs, 0.0);
                    state = 4;
                    e.set(this, DialogMenu::eDialogState, 4);
                } else {
                    let showing = e.global::<u32>(SHOWING_TEXT_TRAIT_011D9500);
                    let root = menu_tile(e, this.addr());
                    set_int(e, root, showing, 0);
                    dialog_menu_load_topics_list(e, this);
                    state = 2;
                }
            }
        }
        None
    })
    .unwrap_or_else(|| {
        if e.get(this, DialogMenu::bPaused) != 0 {
            e.set(this, DialogMenu::ePausedState, state);
        }
        state
    })
}

// Translated from 007638b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007638b0 - the /GS cookie check (00ec408c) is not translated.
/// `DialogMenu::LoadTopicsList` (Xbox PDB). Empties the list box (its vtable slot +0x1c),
/// then adds one item per topic of the topic manager (`FirstTopic(1)`, `NextTopic`), the
/// item's data being its index: the text is the topic's prompt (`007627e0` or `00762820`,
/// the string at +0x18) as `"[%s"`, then for a speech challenge (`007627c0`) `"%s, %i"` of
/// the chance (`00648ea0` of the player's actor value 0x2b, the speaker's disposition
/// (slot +0x344 with the player, an actor) or a game setting, and the info's `0061f2b0`)
/// and `"%"`, or for a skill check `"%s %d/%d"` (current/required, the item marked through
/// `0071aa60`) or `"%s %d"`; closed by `"%s]  %s"` with the topic's text. Without a prompt
/// it is the topic's text, or an error is logged with the topic's name (slot +0x130 of its
/// +0x28) or `"unknown"` and the text left empty. Each item tile is named `"topic_%i"`,
/// gets its index (`0xfac`) and `"_line_alpha"` 0xff for a said topic whose info has
/// bit 1 of +0x26 clear (`00763fd0`), else 0x80. The heights of the first three tiles
/// (`0xfb0`, truncated) are summed into the container's `0xfb0`; the list's container's
/// `"_height"` follows the menu's `"_ScrollbarHeight"`, its `"_current_value"` is reset,
/// the list synchronized and `uiIgnoreInputStartTime` set.
pub fn dialog_menu_load_topics_list(e: &mut Engine, this: Ptr<DialogMenu>) {
    let list = this.addr() + 0x40;
    e.vcall(list, 0x1c, &args![]);
    let topics_of = |e: &Engine| e.get(this, DialogMenu::pTopicManager);
    let mut index = 0i32;
    let mut height = 0i32;
    let mut more = e
        .call(TOPICS_FIRST_TOPIC, &args![topics_of(e), 1u32])
        .bool();
    e.with_stack(0x200, |e, buffer| {
        let buffer = buffer.addr();
        while more {
            let topic = e.call(TOPICS_GET_CURRENT_TOPIC, &args![topics_of(e)]).u32();
            let mut marked = false;
            let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
            let prompt = if fn_007627e0(e, Ptr::new(info)) != 0 {
                let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                let form = fn_007627e0(e, Ptr::new(info));
                if form != 0 {
                    form + 0x18
                } else {
                    0
                }
            } else {
                let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                let form = fn_00762820(e, Ptr::new(info));
                if form != 0 {
                    form + 0x18
                } else {
                    0
                }
            };
            let topic_text = |e: &mut Engine| {
                let slot = e.call(NODE_SELF, &args![topic]).u32();
                e.call(WORD_AT_0_00559450, &args![slot]).u32()
            };
            if prompt != 0 {
                let text = e.call(STRING_CHARS_00408DA0, &args![prompt]).u32();
                e.call(
                    FORMAT_00406D00,
                    &args![buffer, 0x200u32, PROMPT_FORMAT, text],
                );
                let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                if fn_007627c0(e, Ptr::new(info)) {
                    let speaker = e.get(this, DialogMenu::pSpeaker);
                    // The object the code works out here is not used afterwards.
                    if !e.vcall(speaker, 0x100, &args![]).bool()
                        && e.call(OBJECT_00574900, &args![speaker]).bool()
                    {
                        let object = e.call(OBJECT_005E3FA0, &args![speaker]).u32();
                        let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![object]).u32();
                        e.call(CONTAINER_00516BF0, &args![form]);
                    }
                    let speaker = e.get(this, DialogMenu::pSpeaker);
                    let disposition = if e.vcall(speaker, 0x100, &args![]).bool() {
                        let player = player(e);
                        let value = e.vcall(speaker, 0x344, &args![player, 0u32]).i32();
                        x87_store32(f64::from(value))
                    } else {
                        x87_quiet_bits(setting_float_bits(e, SETTING_011D0A24))
                    };
                    let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                    let difficulty = e.call(INFO_0061F2B0, &args![info]).u32();
                    let values = player(e).wrapping_add(0xa4);
                    let speech = e
                        .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![values, 0x2bu32])
                        .f64();
                    let chance = e
                        .call(
                            FORMULA_00648EA0,
                            &args![x87_store32(speech), x87_quiet_bits(disposition), difficulty],
                        )
                        .u32();
                    e.call(
                        FORMAT_00406D00,
                        &args![buffer, 0x200u32, CHALLENGE_FORMAT, buffer, chance],
                    );
                    e.call(APPEND_00406D50, &args![buffer, 0x200u32, PERCENT_TEXT]);
                } else {
                    let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                    let conditions = e.call(CONDITIONS_00500940, &args![info]).u32();
                    let mut node = e.call(NODE_SELF, &args![conditions]).u32();
                    while node != 0 && node_item(e, node) != 0 {
                        let item = node_item(e, node);
                        let data = e.call(CONDITION_DATA_00413F40, &args![item]).u32();
                        if data != 0 && e.mem.u16(data) == 0xe {
                            let actor_value = e.mem.u32(data + 4);
                            let item = node_item(e, node);
                            let value = e.call(CONDITION_VALUE_00681490, &args![item]).f64();
                            let required = e.call(FTOL2, &args![value]).i32();
                            let values = player(e).wrapping_add(0xa4);
                            let current = e
                                .call(GET_CLAMPED_ACTOR_VALUE, &args![values, actor_value])
                                .i32();
                            if current < required {
                                e.call(
                                    FORMAT_00406D00,
                                    &args![
                                        buffer,
                                        0x200u32,
                                        SKILL_SHORT_FORMAT,
                                        buffer,
                                        current,
                                        required
                                    ],
                                );
                                marked = true;
                            } else {
                                e.call(
                                    FORMAT_00406D00,
                                    &args![buffer, 0x200u32, SKILL_FORMAT, buffer, required],
                                );
                            }
                            break;
                        }
                        node = node_next(e, node);
                    }
                }
                let text = topic_text(e);
                e.call(
                    FORMAT_00406D00,
                    &args![buffer, 0x200u32, PROMPT_CLOSE_FORMAT, buffer, text],
                );
            } else if topic_text(e) != 0 {
                let text = topic_text(e);
                e.call(COPY_00406D30, &args![buffer, 0x200u32, text]);
            } else {
                e.mem.set_u8(buffer, 0);
                let form = e.mem.u32(topic + 0x28);
                let name = if form != 0 {
                    let form = e.mem.u32(topic + 0x28);
                    e.vcall(form, 0x130, &args![]).u32()
                } else {
                    UNKNOWN_TEXT
                };
                e.call(DEBUG_MESSAGE, &args![NO_TOPIC_TEXT_ERROR, name]);
            }
            let tile = list_box_int_add_item(e, Ptr::new(list), index as u32, buffer, 0, 0);
            e.call(
                FORMAT_00406D00,
                &args![buffer, 0x200u32, TOPIC_NAME_FORMAT, index],
            );
            e.call(TILE_SET_NAME_00707E10, &args![tile, buffer]);
            set_int(e, tile, 0xfac, index as u32);
            let bright = e.mem.u8(topic + 0x25) != 0 && {
                let info = e.call(WORD_AT_18_009611E0, &args![topic]).u32();
                !fn_00763fd0(e, Ptr::new(info))
            };
            let alpha = if bright { 0xffu32 } else { 0x80 };
            let line_alpha = e.call(TEXT_TO_TRAIT, &args![LINE_ALPHA_TEXT]).u32();
            set_int(e, tile, line_alpha, alpha);
            if marked {
                e.call(TILE_FLAG_SETTER_0071AA60, &args![tile, 1u32]);
            }
            if index <= 2 {
                let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfb0u32]).f64();
                let whole = e.call(FTOL2, &args![value]).i32();
                height = whole.wrapping_add(height);
            }
            more = e.call(TOPICS_NEXT_TOPIC, &args![topics_of(e)]).bool();
            index = index.wrapping_add(1);
        }
    });
    let container = e.mem.u32(this.addr() + 0x3c);
    e.call(
        TILE_SET_FLOAT,
        &args![container, 0xfb0u32, x87_store32(f64::from(height)), 0u32],
    );
    let scrollbar_height = e.call(TEXT_TO_TRAIT, &args![SCROLLBAR_HEIGHT_TEXT]).u32();
    let root = menu_tile(e, this.addr());
    let value = x87_store32(e.call(TILE_GET_FLOAT, &args![root, scrollbar_height]).f64());
    let height_trait = e.call(TEXT_TO_TRAIT, &args![HEIGHT_TEXT]).u32();
    let scroll_bar = e.call(WORD_AT_14_00825C00, &args![list]).u32();
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_bar, height_trait, value, 1u32],
    );
    let current_value = e.global::<u32>(CURRENT_VALUE_TRAIT_011D9504);
    let scroll_bar = e.call(WORD_AT_14_00825C00, &args![list]).u32();
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_bar, current_value, 0.0f32.to_bits(), 0u32],
    );
    list_box_int_synchronize(e, Ptr::new(list));
    let time = now(e);
    e.set(this, DialogMenu::uiIgnoreInputStartTime, time);
}

// Translated from 00763fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a topic info): bit 1 of the byte at +0x26, in `AL`.
pub fn fn_00763fd0(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u8(this.addr().wrapping_add(0x26)) as i8 as i32) & 0x2 != 0
}

// Translated from 0083e0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the engine map's `BSSimpleList<MenuTopic_P>::AddTail` name does not fit the
/// body; `thiscall` on a `MenuTopic`): starts its response iteration at its embedded list
/// (+0xc, stored at +0x20) and returns whether that node and its item are non-null.
pub fn fn_0083e0c0(e: &mut Engine, this: Ptr) -> bool {
    let first = this.addr().wrapping_add(0xc);
    e.mem.set_u32(this.addr().wrapping_add(0x20), first);
    let node = e.mem.u32(this.addr().wrapping_add(0x20));
    node != 0 && node_item(e, node) != 0
}

// Translated from 0083e110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `MenuTopic`): moves its response iteration (+0x20) to the next
/// node when there is one, and returns whether the node and its item are non-null.
pub fn fn_0083e110(e: &mut Engine, this: Ptr) -> bool {
    let cursor = this.addr().wrapping_add(0x20);
    if e.mem.u32(cursor) != 0 {
        let next = node_next(e, e.mem.u32(cursor));
        e.mem.set_u32(cursor, next);
    }
    let node = e.mem.u32(cursor);
    node != 0 && node_item(e, node) != 0
}

/// `TESTopic::GetMatchingInfo` (engine map, `0061a720`; `thiscall` on a topic, two words:
/// speaker and target).
const TOPIC_GET_MATCHING_INFO: u32 = 0x0061_a720;
/// `00619d00` (`thiscall` on a topic, one word: an info; no engine-map name).
const TOPIC_INFO_VALUE_00619D00: u32 = 0x0061_9d00;
/// The `MenuTopic` deleting destructor (`00425ff0`, the map's `MenuTopic::~MenuTopic`;
/// `thiscall`, one word: the flags).
const MENU_TOPIC_DELETE: u32 = 0x0042_5ff0;
/// `Menu::StartFadeIn` (engine map, `00a1db20`; `thiscall`, no stack words).
const MENU_START_FADE_IN: u32 = 0x00a1_db20;
/// `BSSoundHandle::Play` (engine map, `00ad8830`; `thiscall`, one word) and
/// `BSSoundHandle::Pause` (`00ad88b0`, `thiscall`, no stack words).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_PAUSE: u32 = 0x00ad_88b0;

/// The speaker of the open dialogue menu (`pSpeaker` of the menu in `011d9510`, read again
/// at each use as the exe does).
fn open_menu_speaker(e: &mut Engine) -> u32 {
    let menu: u32 = e.global(OPEN_MENU_011D9510);
    e.get(Ptr::<DialogMenu>::new(menu), DialogMenu::pSpeaker)
}

// Translated from 007640a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007640a0 - the compiler's exception frame (FS:[0] chain, handler 00ef2e1b, state 0 that frees the 0x2c-byte block when the `MenuTopic` constructor throws) is not translated
/// `DialogMenu::ServiceEnd` (engine map; `cdecl`, one word: a topic). With an open dialogue
/// menu (`011d9510`): clears `bInServiceMenu`; with a topic, finds its info for the speaker
/// and the player (`GetMatchingInfo`), builds a 0x2C-byte `MenuTopic` from them
/// (`0083db00` with `00619d00(info)`, the topic, the info, the speaker and 0; null when the
/// allocation fails), starts its responses ([`fn_0083e0c0`]), has the speaker say the
/// current response (vtable +0x264 with 0.0, the response and 0), sets the speaker's byte
/// +0x7C and deletes the `MenuTopic` again (`00425ff0` with 1) when there is one. Last, the
/// menu fades in (`Menu::StartFadeIn`).
pub fn dialog_menu_service_end(e: &mut Engine, topic: u32) {
    let menu: u32 = e.global(OPEN_MENU_011D9510);
    if menu == 0 {
        return;
    }
    e.set(Ptr::<DialogMenu>::new(menu), DialogMenu::bInServiceMenu, 0);
    if topic != 0 {
        let player: u32 = e.global(PLAYER_011DEA3C);
        let speaker = open_menu_speaker(e);
        let info = e
            .call(TOPIC_GET_MATCHING_INFO, &args![topic, speaker, player])
            .u32();
        let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
        let menu_topic = if block != 0 {
            let speaker = open_menu_speaker(e);
            let value = e.call(TOPIC_INFO_VALUE_00619D00, &args![topic, info]).u32();
            e.call(
                TOPIC_CONSTRUCT_0083DB00,
                &args![block, value, topic, info, speaker, 0u32],
            )
            .u32()
        } else {
            0
        };
        fn_0083e0c0(e, Ptr::new(menu_topic));
        let response = e.call(TOPIC_GET_CURRENT_RESPONSE, &args![menu_topic]).u32();
        let speaker = open_menu_speaker(e);
        e.vcall(speaker, 0x264, &args![0.0f32, response, 0u32]);
        let speaker = open_menu_speaker(e);
        e.mem.set_u8(speaker.wrapping_add(0x7c), 1);
        if menu_topic != 0 {
            e.call(MENU_TOPIC_DELETE, &args![menu_topic, 1u32]);
        }
    }
    let menu: u32 = e.global(OPEN_MENU_011D9510);
    e.call(MENU_START_FADE_IN, &args![menu]);
}

// Translated from 00764200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::SetPause` (engine map; `thiscall`, one word, `RET 4`). Paused and asked to
/// go on: plays the speech again (`BSSoundHandle::Play(0)` on `hSpeechFile`), clears
/// `bPaused` and restores `eDialogState` from `ePausedState`. Not paused and asked to
/// pause: pauses the speech, sets `bPaused`, keeps `eDialogState` in `ePausedState` and
/// sets the state to 2. Otherwise nothing.
pub fn dialog_menu_set_pause(e: &mut Engine, this: Ptr<DialogMenu>, pause: u8) {
    let speech = this.addr() + 0x74;
    if e.get(this, DialogMenu::bPaused) != 0 && pause == 0 {
        e.call(SOUND_HANDLE_PLAY, &args![speech, 0u32]);
        e.set(this, DialogMenu::bPaused, 0);
        let state = e.get(this, DialogMenu::ePausedState);
        e.set(this, DialogMenu::eDialogState, state);
    } else if e.get(this, DialogMenu::bPaused) == 0 && pause != 0 {
        e.call(SOUND_HANDLE_PAUSE, &args![speech]);
        e.set(this, DialogMenu::bPaused, 1);
        let state = e.get(this, DialogMenu::eDialogState);
        e.set(this, DialogMenu::ePausedState, state);
        e.set(this, DialogMenu::eDialogState, 2);
    }
}

// Translated from 007628c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::DoGamepad` (engine map; `RET 8`: the button and a word it does not read;
/// the answer in `AL`). While the dialogue menu is the top menu, button 9 clicks (vtable
/// +0xC): with the menu's tile showing text (the trait at `011d9500` true) as `(0, 0)`,
/// otherwise as `(-1, the list box's vtable +4 answer)`; it answers true. Everything else
/// answers false.
pub fn dialog_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<DialogMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    if !e
        .call(INTERFACE_IS_TOP_MENU_ID, &args![DIALOG_MENU_CLASS])
        .bool()
    {
        return false;
    }
    if button != 9 {
        return false;
    }
    let showing: u32 = e.global(SHOWING_TEXT_TRAIT_011D9500);
    let tile = menu_tile(e, this.addr());
    if e.call(TILE_IS_TRUE, &args![tile, showing]).bool() {
        e.vcall(this.addr(), 0xc, &args![0u32, 0u32]);
    } else {
        let list_box = this.addr() + 0x40;
        let tile = e.vcall(list_box, 4, &args![]).u32();
        e.vcall(this.addr(), 0xc, &args![0xffff_ffffu32, tile]);
    }
    true
}

// Translated from 00763ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::ServiceStart` (engine map; no arguments). With a dialogue menu open whose
/// goodbye state is 0: `bInServiceMenu` and `bDoServiceLoad` are set; when the topic manager
/// has a current topic, the speaker's slot +0x264 says its current response (`0.0`, the
/// response, 0) and the speaker's byte +0x7c is set; the menu starts fading out.
pub fn dialog_menu_service_start(e: &mut Engine) {
    let menu = open_menu(e);
    if menu == 0 || e.mem.u32(menu + 0x2c) != 0 {
        return;
    }
    e.mem.set_u8(open_menu(e) + 0x138, 1);
    e.mem.set_u8(open_menu(e) + 0x13a, 1);
    let topics = e.mem.u32(open_menu(e) + 0x70);
    if e.call(TOPICS_GET_CURRENT_TOPIC, &args![topics]).u32() != 0 {
        let topics = e.mem.u32(open_menu(e) + 0x70);
        let topic = e.call(TOPICS_GET_CURRENT_TOPIC, &args![topics]).u32();
        let response = e.call(TOPIC_GET_CURRENT_RESPONSE, &args![topic]).u32();
        let speaker = e.mem.u32(open_menu(e) + 0x80);
        e.vcall(speaker, 0x264, &args![0.0f32.to_bits(), response, 0u32]);
        let speaker = e.mem.u32(open_menu(e) + 0x80);
        e.mem.set_u8(speaker + 0x7c, 1);
    }
    let menu = open_menu(e);
    e.call(MENU_START_FADE_OUT, &args![menu]);
}

// Translated from 00764290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogMenu::DebugResetSaidOnceFlags` (engine map; no arguments): with a dialogue menu
/// open, every topic of its topic manager (`FirstTopic(1)`, then `NextTopic` while
/// `GetCurrentTopic` answers one) gets its byte +0x25 set.
pub fn dialog_menu_debug_reset_said_once_flags(e: &mut Engine) {
    let menu = open_menu(e);
    if menu == 0 {
        return;
    }
    let topics = e.mem.u32(menu + 0x70);
    e.call(TOPICS_FIRST_TOPIC, &args![topics, 1u32]);
    loop {
        let topic = e.call(TOPICS_GET_CURRENT_TOPIC, &args![topics]).u32();
        if topic == 0 {
            return;
        }
        e.mem.set_u8(topic + 0x25, 1);
        e.call(TOPICS_NEXT_TOPIC, &args![topics]);
    }
}

/// The embedded list's sort (`0083fd60`, `thiscall`, one word).
const LIST_SORT_0083FD60: u32 = 0x0083_fd60;
/// `004bd510` (`cdecl`, two `float`s, `ST0`): the scroll step's rounding.
const SCROLL_ROUND_004BD510: u32 = 0x004b_d510;

// Translated from 00764900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>::Sort` (engine map; `RET 4`): the embedded list (+4) sorted with
/// `compare` (`0083fd60`), then the list box synchronized.
pub fn list_box_int_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    e.call(LIST_SORT_0083FD60, &args![this.addr() + 4, compare]);
    list_box_int_synchronize(e, this);
}

/// [`list_box_int_get_highlight_target`]'s page step: 0 without a container; otherwise its
/// trait `0xfb0` (negated for an upward page, `FCHS`) over its `"_scroll_delta"`, stored as
/// a `float`, through `004bd510` with 1.0 and truncated (`_ftol2`).
fn page_step(e: &mut Engine, this: Ptr<ListBox>, negate: bool) -> i32 {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return 0;
    }
    let height = e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64();
    let height = if negate { -height } else { height };
    let delta_trait = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let delta = e.call(TILE_GET_FLOAT, &args![container, delta_trait]).f64();
    let quotient = x87_store32(x87_div(height, delta));
    let steps = e
        .call(
            SCROLL_ROUND_004BD510,
            &args![x87_quiet_bits(quotient), 1.0f32.to_bits()],
        )
        .f64();
    e.call(FTOL2, &args![steps]).i32()
}

/// Whether `tile` is enabled (its `"_enabled"` trait, looked up with `Tile::AddUserTrait`
/// each time).
fn enabled_tile(e: &mut Engine, tile: u32) -> bool {
    let enabled = e
        .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(TILE_IS_TRUE, &args![tile, enabled]).bool()
}

// Translated from 00764a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>::GetHighlightTarget` (engine map; `RET 4`: the direction): the tile a move
/// highlights. The step is -1 for 1, +1 for 2, a page for 15 (up) and 16 (down), 0
/// otherwise (the code's byte map and jump table). With an enabled list, a step and items,
/// the index starts from the highlighted tile's (`0xfac`, `_ftol2`, plus the step, kept
/// within the list) or the first (step down) or last item; the item tile whose `0xfac`
/// equals the index is searched; a disabled one moves on in the step's direction: past the
/// end backwards from the item before the last, past the start forwards from item 1 (vtable
/// +0x14 with the index and 0, until an enabled tile or the list's end). Answers the tile
/// found, or 0 when it is the highlighted one.
pub fn list_box_int_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    let step = match direction {
        1 => -1,
        2 => 1,
        15 => page_step(e, this, true),
        16 => page_step(e, this, false),
        _ => 0,
    };
    let mut found = 0u32;
    if list_box_int_is_enabled(e, this) && step != 0 && e.get(this, ListBox::usNextIndex) != 0 {
        found = highlight_search(e, this, step);
    }
    let highlighted = e.get(this, ListBox::pHighlightedItem).addr();
    if found != highlighted {
        found
    } else {
        0
    }
}

/// The search loop of [`list_box_int_get_highlight_target`].
fn highlight_search(e: &mut Engine, this: Ptr<ListBox>, step: i32) -> u32 {
    let last = i32::from(e.get(this, ListBox::usNextIndex)) - 1;
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let mut index = if !highlighted.is_null() {
        let value = e.call(TILE_GET_FLOAT, &args![highlighted, 0xfacu32]).f64();
        let current = e.call(FTOL2, &args![value]).i32();
        let index = current.wrapping_add(step);
        if index < 0 {
            0
        } else if index > last {
            last
        } else {
            index
        }
    } else if step > 0 {
        0
    } else {
        last
    };
    let mut found = 0u32;
    loop {
        let mut node = first_node(this);
        while found == 0 && node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
            // FILD index; FUCOMPP; TEST AH,0x44; JP: ordered equal.
            if f64::from(index) == value {
                let item = node_item(e, node);
                found = e.mem.u32(item);
            }
            node = node_next(e, node);
        }
        if found == 0 || enabled_tile(e, found) {
            return found;
        }
        found = 0;
        if step > 0 {
            index = index.wrapping_add(1);
            if index > last {
                index = last - 1;
                loop {
                    if found != 0 && enabled_tile(e, found) {
                        return found;
                    }
                    if index < 0 {
                        return found;
                    }
                    found = e.vcall(this.addr(), 0x14, &args![index as u32, 0u32]).u32();
                    index = index.wrapping_sub(1);
                }
            }
        }
        if step < 0 {
            index = index.wrapping_sub(1);
            if index < 0 {
                index = 1;
                loop {
                    if found != 0 && enabled_tile(e, found) {
                        return found;
                    }
                    if index > last {
                        return found;
                    }
                    found = e.vcall(this.addr(), 0x14, &args![index as u32, 0u32]).u32();
                    index = index.wrapping_add(1);
                }
            }
        }
    }
}

// Translated from 00764e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<int>` scalar deleting destructor (engine map; `RET 4`): the destructor body
/// ([`fn_007643c0`]), then the memory freed (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn list_box_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_007643c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0076_28c0,
            dialog_menu_do_gamepad(Ptr<DialogMenu>, i32, u32) -> bool
        ),
        entry!(0x0076_3ff0, dialog_menu_service_start()),
        entry!(0x0076_4290, dialog_menu_debug_reset_said_once_flags()),
        entry!(0x0076_4900, list_box_int_sort(Ptr<ListBox>, u32)),
        entry!(
            0x0076_4a70,
            list_box_int_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(
            0x0076_4e10,
            list_box_int_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(0x0076_40a0, dialog_menu_service_end(u32)),
        entry!(0x0076_4200, dialog_menu_set_pause(Ptr<DialogMenu>, u8)),
        entry!(0x0076_18f0, dialog_menu_get_class(Ptr<DialogMenu>) -> u32),
        entry!(
            0x0076_1900,
            dialog_menu_attach_tile_by_id(Ptr<DialogMenu>, i32, u32)
        ),
        entry!(
            0x0076_1930,
            dialog_menu_scalar_deleting_destructor(Ptr<DialogMenu>, u32) -> Ptr<DialogMenu>
        ),
        entry!(0x0076_1960, fn_00761960(Ptr<DialogMenu>)),
        entry!(0x0076_1a20, dialog_menu_create(u32, u32, u32) -> u32),
        entry!(0x0076_20e0, fn_007620e0() -> u32),
        entry!(0x0076_20f0, fn_007620f0(Ptr<DialogMenu>) -> bool),
        entry!(0x0076_2130, fn_00762130(Ptr, u8)),
        entry!(0x0076_2150, fn_00762150()),
        entry!(0x0076_2160, dialog_menu_close()),
        entry!(0x0076_24c0, fn_007624c0()),
        entry!(0x0076_24d0, fn_007624d0() -> u32),
        entry!(0x0076_24f0, dialog_menu_do_click(Ptr<DialogMenu>, i32, u32)),
        entry!(0x0076_27c0, fn_007627c0(Ptr) -> bool),
        entry!(0x0076_27e0, fn_007627e0(Ptr) -> u32),
        entry!(0x0076_2820, fn_00762820(Ptr) -> u32),
        entry!(
            0x0076_2860,
            dialog_menu_set_goodbye_state(Ptr<DialogMenu>, u32)
        ),
        entry!(0x0076_2950, dialog_menu_do_idle(Ptr<DialogMenu>)),
        entry!(
            0x0076_2ff0,
            dialog_menu_load_next_screen(Ptr<DialogMenu>, u8) -> u32
        ),
        entry!(0x0076_38b0, dialog_menu_load_topics_list(Ptr<DialogMenu>)),
        entry!(0x0076_3fd0, fn_00763fd0(Ptr) -> bool),
        entry!(0x0076_42f0, fn_007642f0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x0076_43c0, fn_007643c0(Ptr<ListBox>)),
        entry!(
            0x0076_4430,
            list_box_int_add_item(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0076_46d0, list_box_int_remove_all(Ptr<ListBox>)),
        entry!(0x0076_4930, list_box_int_synchronize(Ptr<ListBox>)),
        entry!(
            0x0076_4a00,
            list_box_int_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(0x0076_4e40, list_box_int_is_enabled(Ptr<ListBox>) -> bool),
        entry!(
            0x0076_4ef0,
            list_box_int_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x0076_5210,
            list_box_int_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x0076_5340,
            list_box_int_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(0x0083_e0c0, fn_0083e0c0(Ptr) -> bool),
        entry!(0x0083_e110, fn_0083e110(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod int_list_box_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the statics, the flag byte and the doubles mapped, and the list node
    /// helpers doubled (`006815c0` returns the node, `00726070` its next word).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_8000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ADD_USER_TRAIT, |_, _| 0x66u32.into_ret());
        e
    }

    fn list_box(e: &mut Engine) -> Ptr<ListBox> {
        Ptr::new(e.mem.alloc(0x30))
    }

    fn item(e: &mut Engine, tile: u32, data: u32, filtered: u8) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        e.mem.set_u8(item + 8, filtered);
        item
    }

    #[test]
    fn the_constructor_and_destructor_use_the_int_vtable() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                LIST_BOX_BASE_CONSTRUCT,
                LIST_CONSTRUCT,
                LIST_BOX_INIT_FLAGS,
                LIST_BOX_REGISTER,
                LIST_BOX_UNREGISTER,
                LIST_DESTRUCT,
                TILE_SET_FLOAT,
            ],
        );
        let this = list_box(&mut e);
        for offset in (0x0c..0x30).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_42f0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), INT_LIST_BOX_VTABLE);
        for offset in [0x0c, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(this.addr() + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u16(this.addr() + 0x2c), 0);
        assert_eq!(
            calls_to(&e, LIST_BOX_INIT_FLAGS),
            vec![vec![this.addr(), 0, 1]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_REGISTER), vec![vec![this.addr()]]);
        e.mem.set_u32(this.addr(), 0);
        e.call(0x0076_43c0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), INT_LIST_BOX_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_UNREGISTER), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![this.addr() + 4]]);
    }

    fn add_item_engine() -> Engine {
        let mut e = engine();
        e.register(TILE_GET_MENU, |_, _| 0x5000u32.into_ret());
        e.register(MENU_RENDER_TEMPLATE, |_, _| 0x6000u32.into_ret());
        e.register(TILE_GET_VALUE, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CONSTRUCT, |e, a| {
            let data = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, data);
            a[0].into_ret()
        });
        e.register(LIST_BOX_HAS_FLAG, |_, _| 1u32.into_ret());
        e.register(LIST_BOX_COUNT_00726C40, |_, _| 1u32.into_ret());
        e.register(ADD_USER_TRAIT, |_, _| 0x77u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(3.0));
        e.register(TILE_GET_VALUE_Q, |_, _| 0x9000u32.into_ret());
        stub(
            &mut e,
            &[
                TILE_SET_INT,
                TILE_SET_STRING,
                LIST_INSERT_SORTED,
                LIST_APPEND,
                INIT_TILE_POSITION,
                VALUE_CLEAR_ACTIONS,
                VALUE_ADD_ACTION_TILE,
                VALUE_ADD_ACTION_FLOAT,
                VALUE_CALCULATE,
                TILE_SET_FLOAT,
                TEXT_TO_TRAIT,
                ROUND_FLOAT,
                FTOL2,
                INT_PAIR_00647B70,
            ],
        );
        e
    }

    #[test]
    fn add_item_needs_a_container_and_a_template() {
        let mut e = add_item_engine();
        let this = list_box(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0076_4430, &args![this, 1u32, 0u32, 0u32, 0x4000u32])
                .u32(),
            0
        );
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert_eq!(
            e.call(0x0076_4430, &args![this, 1u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, MENU_RENDER_TEMPLATE).is_empty());
    }

    #[test]
    fn add_item_appends_and_sizes_the_container() {
        let mut e = add_item_engine();
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pTemplateName, Ptr::new(0x4100));
        e.set(this, ListBox::usNextIndex, 5);
        e.call_log = Some(vec![]);
        let tile = e
            .call(0x0076_4430, &args![this, 0x42u32, 0x4200u32, 0u32, 0u32])
            .u32();
        assert_eq!(tile, 0x6000);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x5000, 0x3000, 0x4100, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x6000, 0xfc4, 0x4200, 1]]
        );
        let appended = calls_to(&e, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        let item = e.mem.u32(appended[0][1]);
        assert_eq!((e.mem.u32(item), e.mem.u32(item + 4)), (0x6000, 0x42));
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6000, 0xfaa, 0xffff_ffff], vec![0x6000, 0xfac, 5]]
        );
        assert_eq!(e.get(this, ListBox::usNextIndex), 6);
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_FLOAT),
            vec![vec![0x9000, 0x7d3, 3.0f32.to_bits()]]
        );
    }

    #[test]
    fn add_item_inserts_sorted_with_a_compare_function() {
        let mut e = add_item_engine();
        e.register(LIST_BOX_COUNT_00726C40, |_, _| 2u32.into_ret());
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.call_log = Some(vec![]);
        let tile = e
            .call(
                0x0076_4430,
                &args![this, 0x42u32, 0u32, 0x00ab_cdefu32, 0x4300u32],
            )
            .u32();
        assert_eq!(tile, 0x6000);
        let inserted = calls_to(&e, LIST_INSERT_SORTED);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][0], this.addr() + 4);
        assert_eq!(inserted[0][2], 0x00ab_cdef);
        assert!(calls_to(&e, VALUE_ADD_ACTION_TILE).is_empty());
    }

    #[test]
    fn remove_all_deletes_every_item_and_resets() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                FREE,
                NODE_SET_NEXT,
                NODE_DELETE,
                ITEM_DATA_DELETE,
                TILE_SET_FLOAT,
                TILE_SET_INT,
                TILE_GET_VALUE,
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
            ],
        );
        e.register(LIST_BOX_HAS_FLAG, |_, a| u32::from(a[1] == 1).into_ret());
        e.register(TEXT_TO_TRAIT, |_, _| 0x55u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(0.0));
        e.register(0x0000_7000, |_, _| Ret::default());
        e.put_vtable(0x0f00_0000, &[0x0000_7000]);
        let this = list_box(&mut e);
        let tile = e.mem.alloc(4);
        e.mem.set_u32(tile, 0x0f00_0000);
        let first = item(&mut e, tile, 0x8000, 0);
        let second = item(&mut e, 0, 0, 0);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x3100));
        e.call_log = Some(vec![]);
        e.call(0x0076_46d0, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![second], vec![first]]);
        assert_eq!(calls_to(&e, NODE_DELETE), vec![vec![node, 1]]);
        assert_eq!(calls_to(&e, ITEM_DATA_DELETE), vec![vec![0x8000, 1]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![tile, 1]]);
        assert!(e.get(this, ListBox::pHighlightedItem).is_null());
        assert_eq!(calls_to(&e, TILE_SET_FLOAT), vec![vec![0x3000, 0x55, 0, 1]]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x3000, 0x66, 0xffff_ffff], vec![0x3000, 0x66, 0]]
        );
        assert_eq!(e.mem.u32(0x011d_95a8) & 3, 3);
    }

    #[test]
    fn synchronize_numbers_the_unfiltered_items() {
        let mut e = engine();
        stub(&mut e, &[TILE_SET_INT, INIT_TILE_POSITION]);
        let this = list_box(&mut e);
        let first = item(&mut e, 0x6100, 0, 0);
        let second = item(&mut e, 0x6200, 0, 1);
        let third = item(&mut e, 0x6300, 0, 0);
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, node2);
        e.mem.set_u32(node2, second);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, third);
        e.set(this, ListBox::usNextIndex, 9);
        e.call_log = Some(vec![]);
        e.call(0x0076_4930, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x6100, 0xfac, 0], vec![0x6300, 0xfac, 1]]
        );
        assert_eq!(e.get(this, ListBox::usNextIndex), 2);
    }

    #[test]
    fn highlight_and_is_enabled_need_an_enabled_container() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| 0x44u32.into_ret());
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] != 0x6100).into_ret());
        e.register(LIST_BOX_BELONGS_TO, |_, a| {
            u32::from(a[1] != 0x6200).into_ret()
        });
        stub(
            &mut e,
            &[
                TILE_GET_VALUE,
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
                TILE_SET_INT,
            ],
        );
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| Ret::default());
        e.register(TILE_FIND_BY_NAME, |_, _| Ret::default());
        let this = list_box(&mut e);
        assert!(!e.call(0x0076_4e40, &args![this]).bool());
        assert_eq!(e.mem.u32(0x011d_9598), 0x44);
        assert!(!e.call(0x0076_4a00, &args![this, 0x6000u32]).bool());
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert!(!e.call(0x0076_4a00, &args![this, 0x6100u32]).bool());
        assert!(!e.call(0x0076_4a00, &args![this, 0x6200u32]).bool());
        assert!(e.call(0x0076_4a00, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(this, ListBox::pHighlightedItem).addr(), 0x6000);
    }

    #[test]
    fn update_highlight_box_pos_follows_the_highlighted_tile() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, a| {
            if a[0] == HIGHLIGHT_Y_TEXT {
                0x31u32.into_ret()
            } else {
                0x32u32.into_ret()
            }
        });
        e.register(TILE_GET_VALUE, |_, a| (0x9000 + a[1]).into_ret());
        e.register(TILE_FLAG_GETTER_0071AA80, |_, _| 1u32.into_ret());
        e.register(TILE_FIND_BY_NAME, |_, _| 0x7100u32.into_ret());
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| {
            if a[1] == TOP_TEXT {
                0x7200u32.into_ret()
            } else {
                0x7300u32.into_ret()
            }
        });
        stub(
            &mut e,
            &[
                VALUE_ADD_ACTION_TILE,
                VALUE_CALCULATE,
                TILE_FLAG_SETTER_0071AA60,
                TILE_ADD_NEEDS_UPDATE,
                TILE_SET_INT,
            ],
        );
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pHighlightedItem, Ptr::new(0x6000));
        e.call_log = Some(vec![]);
        e.call(0x0076_4ef0, &args![this]);
        assert_eq!(e.mem.u32(0x011d_95a4), 0x31);
        assert_eq!(e.mem.u32(0x011d_95a0), 0x32);
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_TILE),
            vec![
                vec![0x9031, 0x7d0, 0x6000, 0xfa2],
                vec![0x9032, 0x7d0, 0x6000, 0xfb0]
            ]
        );
        assert_eq!(calls_to(&e, TILE_FLAG_SETTER_0071AA60).len(), 5);
        assert_eq!(calls_to(&e, TILE_ADD_NEEDS_UPDATE).len(), 5);
    }

    #[test]
    fn set_total_list_height_counts_scroll_steps() {
        let mut e = engine();
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0xfb0 { 25.0 } else { 0.0 })
        });
        e.register(ADD_USER_TRAIT, |_, _| 0x99u32.into_ret());
        e.register(LIST_BOX_COUNT_00726C40, |_, _| 8u32.into_ret());
        e.register(ROUND_FLOAT, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).ceil())
        });
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32).into_ret()
        });
        e.register(INT_PAIR_00647B70, |_, a| {
            ((a[0] as i32).max(a[1] as i32) as u32).into_ret()
        });
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
        stub(&mut e, &[TILE_SET_FLOAT, TILE_SET_INT]);
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        e.set(this, ListBox::pScrollBar, Ptr::new(0x3400));
        e.set(this, ListBox::fTotalListHeight, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x0076_5210, &args![this]);
        assert_eq!(calls_to(&e, ROUND_FLOAT), vec![vec![6.0f32.to_bits()]]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x3000, 0x90c, 12.5f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x3400, 0x8f8, 7]]);
        e.set(this, ListBox::fTotalListHeight, 20.0);
        e.call_log = Some(vec![]);
        e.call(0x0076_5210, &args![this]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x3400, 0x8f8, 1]]);
    }

    #[test]
    fn num_unfiltered_subtracts_the_filtered_count() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| 0x99u32.into_ret());
        e.register(LIST_BOX_COUNT_00726C40, |_, _| 7u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(2.9));
        let this = list_box(&mut e);
        e.set(this, ListBox::pContainerRect, Ptr::new(0x3000));
        assert_eq!(e.call(0x0076_5340, &args![this]).i32(), 5);
        assert_eq!(e.mem.u32(0x011d_95ac), 0x99);
        e.register(TILE_GET_FLOAT, |_, _| st0(9.5));
        assert_eq!(e.call(0x0076_5340, &args![this]).i32(), -2);
    }
}

#[cfg(test)]
mod dialog_menu_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// The target of slot `slot` of fake object `index` (each slot its own address).
    fn slot(index: u32, slot: u32) -> u32 {
        0x0e00_0000 + index * 0x1000 + slot
    }

    /// A fake object whose vtable slots each point to their own double (returning 0).
    fn fake_object(e: &mut Engine, index: u32) -> u32 {
        let vtable = e.mem.alloc(0x700);
        for offset in (0..0x700).step_by(4) {
            let target = slot(index, offset);
            e.mem.set_u32(vtable + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The pages of the statics and constants the menu reads, mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_9000,
            0x011d_8000,
            0x011d_c000,
            0x011d_e000,
            0x011a_0000,
            0x011f_2000,
            0x0101_2000,
            0x0101_7000,
            0x0103_0000,
            0x0107_0000,
            0x0107_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(TENTH_DOUBLE, 0.1f64);
        e.set_global(FIVE_HUNDRED_DOUBLE, 500.0f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(VALUE_6006_DOUBLE, 6006.0f64);
        e.set_global(VALUE_102_DOUBLE, 102.0f64);
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_18_009611E0, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        e.register(WORD_AT_28_0045CD60, |e, a| {
            e.mem.u32(a[0] + 0x28).into_ret()
        });
        e.register(WORD_AT_14_00825C00, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(WORD_AT_0_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FORM_TYPE_00401170, |e, a| {
            u32::from(e.mem.u8(a[0] + 4)).into_ret()
        });
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32).into_ret()
        });
        e
    }

    fn menu(e: &mut Engine) -> Ptr<DialogMenu> {
        let menu = e.mem.alloc(0x13c);
        Ptr::new(menu)
    }

    #[test]
    fn the_small_accessors() {
        let mut e = engine();
        let this = menu(&mut e);
        assert_eq!(e.call(0x0076_18f0, &args![this]).u32(), 0x3f1);
        assert_eq!(e.call(0x0076_20e0, &args![]).u32(), 0x3f1);
        for (id, expected) in [(-1i32, None), (0, Some(0x30)), (3, Some(0x3c)), (4, None)] {
            e.call(
                0x0076_1900,
                &args![this, id, 0x7000u32.wrapping_add(id as u32)],
            );
            if let Some(offset) = expected {
                assert_eq!(
                    e.mem.u32(this.addr() + offset),
                    0x7000u32.wrapping_add(id as u32)
                );
            }
        }
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0);
        assert!(!e.call(0x0076_20f0, &args![this]).bool());
        for offset in [0x34u32, 0x38] {
            e.mem.set_u32(this.addr() + offset, 1);
        }
        assert!(e.call(0x0076_20f0, &args![this]).bool());
        let object = e.mem.alloc(0x100);
        e.call(0x0076_2130, &args![object, 0x0000_0101u32]);
        assert_eq!(e.mem.u8(object + 0xe2), 1);
        e.call(0x0076_2150, &args![]);
        assert_eq!(e.global::<u8>(FLAG_011DCFA4), 1);
        e.call(0x0076_24c0, &args![]);
        assert_eq!(e.global::<u8>(FLAG_011DCFA4), 0);
    }

    #[test]
    fn fn_007624d0_asks_the_open_menu_for_its_speaker() {
        let mut e = engine();
        e.register(WORD_AT_80_004FB070, |e, a| {
            e.mem.u32(a[0] + 0x80).into_ret()
        });
        assert_eq!(e.call(0x0076_24d0, &args![]).u32(), 0);
        let this = menu(&mut e);
        e.set(this, DialogMenu::pSpeaker, 0x4444);
        e.set_global(OPEN_MENU_011D9510, this.addr());
        assert_eq!(e.call(0x0076_24d0, &args![]).u32(), 0x4444);
    }

    #[test]
    fn the_info_bits_and_prompts() {
        let mut e = engine();
        let info = e.mem.alloc(0x50);
        assert!(!e.call(0x0076_27c0, &args![info]).bool());
        e.mem.set_u8(info + 0x25, 0x80);
        assert!(e.call(0x0076_27c0, &args![info]).bool());
        assert!(!e.call(0x0076_3fd0, &args![info]).bool());
        e.mem.set_u8(info + 0x26, 0x02);
        assert!(e.call(0x0076_3fd0, &args![info]).bool());
        assert_eq!(e.call(0x0076_27e0, &args![info]).u32(), 0);
        assert_eq!(e.call(0x0076_2820, &args![info]).u32(), 0);
        let prompt = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0x40, prompt);
        e.mem.set_u8(prompt + 4, 0x56);
        assert_eq!(e.call(0x0076_27e0, &args![info]).u32(), 0);
        assert_eq!(e.call(0x0076_2820, &args![info]).u32(), prompt);
        e.mem.set_u8(prompt + 4, 0x55);
        assert_eq!(e.call(0x0076_27e0, &args![info]).u32(), prompt);
        assert_eq!(e.call(0x0076_2820, &args![info]).u32(), 0);
    }

    #[test]
    fn the_response_iteration() {
        let mut e = engine();
        let topic = e.mem.alloc(0x2c);
        assert!(!e.call(0x0083_e0c0, &args![topic]).bool());
        assert_eq!(e.mem.u32(topic + 0x20), topic + 0xc);
        e.mem.set_u32(topic + 0xc, 0x5000);
        assert!(e.call(0x0083_e0c0, &args![topic]).bool());
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x5100);
        e.mem.set_u32(topic + 0x10, node);
        assert!(e.call(0x0083_e110, &args![topic]).bool());
        assert_eq!(e.mem.u32(topic + 0x20), node);
        assert!(!e.call(0x0083_e110, &args![topic]).bool());
        assert_eq!(e.mem.u32(topic + 0x20), 0);
        assert!(!e.call(0x0083_e110, &args![topic]).bool());
    }

    #[test]
    fn set_goodbye_state_by_the_info_flag_or_the_goodbye_topic() {
        let mut e = engine();
        e.register(FLAG_00619DF0, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x25) & 1).into_ret()
        });
        e.register(TES_TOPIC_GET_TOPIC, |_, a| {
            assert_eq!(a, [1, 2]);
            0x9900u32.into_ret()
        });
        let this = menu(&mut e);
        let topic = e.mem.alloc(0x30);
        let info = e.mem.alloc(0x30);
        e.mem.set_u32(topic + 0x18, info);
        e.call(0x0076_2860, &args![this, topic]);
        assert_eq!(e.get(this, DialogMenu::eGoodbyeState), 0);
        e.mem.set_u32(topic + 0x28, 0x9900);
        e.call(0x0076_2860, &args![this, topic]);
        assert_eq!(e.get(this, DialogMenu::eGoodbyeState), 1);
        e.mem.set_u8(info + 0x25, 1);
        e.call(0x0076_2860, &args![this, topic]);
        assert_eq!(e.get(this, DialogMenu::eGoodbyeState), 2);
    }

    #[test]
    fn the_destructors() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                IMAGE_SPACE_INSTANCE_STOP,
                TOPICS_RELEASE_0083E510,
                SOUND_HANDLE_DESTROY,
                MENU_DESTROY,
                FREE,
                LIST_BOX_UNREGISTER,
                LIST_DESTRUCT,
                ADD_USER_TRAIT,
            ],
        );
        let this = menu(&mut e);
        e.set(this, DialogMenu::pDepthOfFieldInstance, 0x4321);
        e.set_global(OPEN_MENU_011D9510, this.addr());
        e.set_global(FLAG_011D9514, 1u8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_1930, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), DIALOG_MENU_VTABLE);
        assert_eq!(e.global::<u8>(FLAG_011D9514), 0);
        assert_eq!(e.global::<u32>(OPEN_MENU_011D9510), 0);
        assert_eq!(calls_to(&e, IMAGE_SPACE_INSTANCE_STOP), vec![vec![0x4321]]);
        assert_eq!(e.get(this, DialogMenu::pDepthOfFieldInstance), 0);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_DESTROY),
            vec![vec![this.addr() + 0x74]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_UNREGISTER),
            vec![vec![this.addr() + 0x40]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x40), INT_LIST_BOX_VTABLE);
        assert_eq!(calls_to(&e, MENU_DESTROY), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this.addr()]]);
        // The open menu's instance decides; without the flag bit nothing is freed.
        let open = menu(&mut e);
        e.set_global(OPEN_MENU_011D9510, open.addr());
        e.set(this, DialogMenu::pDepthOfFieldInstance, 0x4321);
        e.call_log = Some(vec![]);
        e.call(0x0076_1930, &args![this, 0u32]);
        assert!(calls_to(&e, IMAGE_SPACE_INSTANCE_STOP).is_empty());
        assert!(calls_to(&e, FREE).is_empty());
    }

    /// The callees `Create` reaches on its full path, doubled.
    fn create_engine() -> (Engine, u32, u32, Ptr<DialogMenu>) {
        let mut e = engine();
        stub(
            &mut e,
            &[
                INTERFACE_MANAGER_004B7210,
                TILE_GET_MENU_BY_CLASS,
                DEBUG_MESSAGE,
                WORD_AT_9C_00586150,
                MENU_SET_MENU_TILE,
                TILE_SET_FLOAT,
                IMAGE_SPACE_GET_GET_HIT,
                IMAGE_SPACE_FORM_STOP,
                MOBILE_GET_CURRENT_PACKAGE,
                EXTRA_LIST_005D43C0,
                EXTRA_GET_PACKAGE,
                PACKAGE_TYPE_0041CA90,
                ACTOR_END_INTERRUPT_PACKAGE,
                ACTOR_STOP_MOVING,
                TOPICS_INITIALIZE,
                TOPICS_GET_GREETING,
                GET_PROCESS,
                PLAYER_004EAF60,
                PLAYER_00951A10,
                PLAYER_FOCUS_ON_ACTOR,
                POSITION_00439EF0,
                IMAGE_SPACE_005BBD60,
                ACTOR_SET_IRON_SIGHTS,
                PLAYER_STOP_IRON_SIGHTS_IMAGE_SPACE,
                LIST_BOX_INITIALIZE,
                TILE_GET_CHILD_BY_NAME,
                TILE_SET_INT,
                ACTOR_ALWAYS_SHOW_SUBTITLES,
                REFERENCE_NAME_0055D520,
                TILE_SET_STRING,
                TUTORIAL_00718840,
                MENU_PREP_FOR_VISIBILITY,
                BS_AUDIO_INSTANCE,
                BS_AUDIO_ENTER_DIALOGUE,
                INTERFACE_SET_GUN_SCOPE_VISIBLE,
            ],
        );
        e.register(TILE_READ_FILE, |_, _| 0x7700u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(102.0));
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| st0(8.0));
        e.register(POSITION_00457990, |_, _| st0(150.0));
        e.register(SETTING_VALUE_00403E20, |_, a| (a[0] + 0x100).into_ret());
        e.register(DOF_TRIGGER, |_, _| 0x5150u32.into_ret());
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
        e.register(FLAG_005D4A40, |_, _| 1u32.into_ret());
        let speaker = fake_object(&mut e, 1);
        let player = fake_object(&mut e, 2);
        e.set_global(PLAYER_011DEA3C, player);
        let process = fake_object(&mut e, 3);
        e.set_global(0x011d_8f04, process);
        e.register(GET_PROCESS, |e, _| e.mem.u32(0x011d_8f04).into_ret());
        let menu_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(menu_vtable + 0x34, 0x00ab_0034);
        e.mem.set_u32(menu_vtable, 0x00ab_0000);
        e.register(0x00ab_0034, |_, _| 0x3f1u32.into_ret());
        e.register(0x00ab_0000, |_, _| Ret::default());
        let this = menu(&mut e);
        e.mem.set_u32(this.addr(), menu_vtable);
        for offset in [0x30u32, 0x34, 0x38, 0x3c] {
            e.mem.set_u32(this.addr() + offset, 0x6000 + offset);
        }
        e.mem.set_u32(this.addr() + 4, 0x6100);
        e.set(this, DialogMenu::pTopicManager, 0x6200);
        e.mem.set_u32(STRING_OWNER_011D8934, 0);
        e.register(STRING_00408D60, |_, _| 0x011d_8934u32.into_ret());
        let setting_page = 0x011d_4000;
        e.map(setting_page, 0x1000);
        e.mem.set_u32(SETTING_011D46FC + 0x100, 2.5f32.to_bits());
        (e, speaker, player, this)
    }

    #[test]
    fn create_returns_an_open_menu_or_fails_early() {
        let (mut e, speaker, _, this) = create_engine();
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0x9999u32.into_ret());
        assert_eq!(
            e.call(0x0076_1a20, &args![speaker, 0u32, 0u32]).u32(),
            0x9999
        );
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_1a20, &args![0u32, 0u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, DEBUG_MESSAGE), vec![vec![NULL_SPEAKER_TEXT]]);
        // A menu of another class is deleted (it has a tile).
        e.register_double(TILE_GET_MENU, move |_, _| this.addr().into_ret());
        e.register(0x00ab_0034, |_, _| 0x3f0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_1a20, &args![speaker, 0u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00ab_0000), vec![vec![this.addr(), 1]]);
        assert!(calls_to(&e, MENU_SET_MENU_TILE).is_empty());
        // Without its four tiles it logs and returns 0.
        e.register(0x00ab_0034, |_, _| 0x3f1u32.into_ret());
        e.mem.set_u32(this.addr() + 0x38, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_1a20, &args![speaker, 0u32, 0u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, DEBUG_MESSAGE),
            vec![vec![CREATION_FAILED_TEXT]]
        );
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![this.addr(), 0x7700, 0]]
        );
    }

    #[test]
    fn create_sets_the_menu_up_and_shows_the_tutorial() {
        let (mut e, speaker, player, this) = create_engine();
        e.register_double(TILE_GET_MENU, move |_, _| this.addr().into_ret());
        // The speaker is an actor whose slot +0x1ac object gets the byte.
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(0x011d_8f00, object);
        e.register(slot(1, 0x1ac), |e, _| e.mem.u32(0x011d_8f00).into_ret());
        e.register(slot(1, 0x100), |_, _| 1u32.into_ret());
        e.register(slot(1, 0x274), |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let tile = e.call(0x0076_1a20, &args![speaker, 0x55u32, 0u32]).u32();
        assert_eq!(tile, 0x7700);
        assert_eq!(e.global::<u32>(OPEN_MENU_011D9510), this.addr());
        assert_eq!(e.get(this, DialogMenu::pSpeaker), speaker);
        assert_eq!(e.mem.u8(object + 0xe2), 1);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7700, 0xfad, 8.0f32.to_bits(), 1]]
        );
        assert!(calls_to(&e, ACTOR_END_INTERRUPT_PACKAGE).is_empty());
        assert_eq!(calls_to(&e, ACTOR_STOP_MOVING), vec![vec![speaker]]);
        assert_eq!(
            calls_to(&e, slot(1, 0x280)),
            vec![vec![speaker, player, 0, 0, 1, 0, 1, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, TOPICS_INITIALIZE),
            vec![vec![0x6200, speaker, 0x55]]
        );
        assert_eq!(
            calls_to(&e, PLAYER_FOCUS_ON_ACTOR),
            vec![vec![player, speaker, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, DOF_TRIGGER),
            vec![vec![150.0f32.to_bits(), 2.5f32.to_bits(), 2, 0, 0, 0]]
        );
        assert_eq!(e.get(this, DialogMenu::pDepthOfFieldInstance), 0x5150);
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![this.addr() + 0x40, 0x603c, TOPIC_TEMPLATE_TEXT]]
        );
        assert_eq!(e.global::<u32>(DIALOG_VISIBLE_TRAIT_011D9508), 0x5e4);
        assert_eq!(e.global::<u32>(SHOWING_TEXT_TRAIT_011D9500), 0x5d4);
        assert_eq!(e.global::<u32>(CURRENT_VALUE_TRAIT_011D9504), 0x2d8);
        // `"_ShowSubtitles"` is 0: no forced subtitles and the actor does not ask for them.
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x6100, 0x5c4, 0]));
        // The tutorial is shown instead of the first screen.
        assert_eq!(e.get(this, DialogMenu::bShowingTutorial), 1);
        assert!(calls_to(&e, TOPICS_GET_CURRENT_TOPIC).is_empty());
        assert_eq!(e.global::<u8>(FLAG_011DCFA4), 1);
        assert_eq!(calls_to(&e, slot(1, 0xd0)), vec![vec![speaker, 1]]);
        assert_eq!(calls_to(&e, INTERFACE_SET_GUN_SCOPE_VISIBLE), vec![vec![0]]);
    }

    /// `Close` with no open menu only leaves the dialogue.
    fn stub_close(e: &mut Engine) {
        stub(e, &[BS_AUDIO_INSTANCE, BS_AUDIO_EXIT_DIALOGUE]);
    }

    #[test]
    fn close_without_an_open_menu_only_leaves_the_dialogue() {
        let mut e = engine();
        stub_close(&mut e);
        e.set_global(FLAG_011DCFA4, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0076_2160, &args![]);
        assert_eq!(calls_to(&e, BS_AUDIO_EXIT_DIALOGUE).len(), 1);
        assert_eq!(e.global::<u8>(FLAG_011DCFA4), 0);
    }

    #[test]
    fn close_runs_the_info_result_and_fades_out() {
        let mut e = engine();
        stub_close(&mut e);
        stub(
            &mut e,
            &[
                DEBUG_MESSAGE,
                MOBILE_STOP_CURRENT_DIALOGUE,
                ACTOR_00493BB0,
                ACTOR_STOP_MOVING,
                PLAYER_RESET_GREET_FLAG,
                INFO_00579200,
                INFO_RUN_RESULT,
                INFO_ADD_TOPIC_LIST,
                TILE_SET_INT,
                MENU_START_FADE_OUT,
            ],
        );
        let speaker = fake_object(&mut e, 1);
        let player = fake_object(&mut e, 2);
        let process = fake_object(&mut e, 3);
        e.set_global(PLAYER_011DEA3C, player);
        e.set_global(0x011d_8f00, process);
        e.register(GET_PROCESS, |e, _| e.mem.u32(0x011d_8f00).into_ret());
        e.register(slot(1, 0x100), |_, _| 1u32.into_ret());
        e.register(slot(3, 0x24), |_, _| 1u32.into_ret());
        e.register(SETTING_VALUE_00403E20, |_, a| (a[0] + 0x100).into_ret());
        e.map(0x011c_d000, 0x1000);
        e.mem.set_u32(SETTING_011CD450 + 0x100, 4.0f32.to_bits());
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 4, 0x6100);
        e.set(this, DialogMenu::pSpeaker, speaker);
        e.set(this, DialogMenu::eGoodbyeState, 3);
        e.set_global(OPEN_MENU_011D9510, this.addr());
        e.set_global(LAST_INFO_011D9518, 0x4400u32);
        e.set_global(FLAG_011D9514, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0076_2160, &args![]);
        assert_eq!(calls_to(&e, DEBUG_MESSAGE), vec![vec![INVALID_CHOICE_TEXT]]);
        assert_eq!(e.global::<u8>(FLAG_011D9514), 0);
        assert_eq!(calls_to(&e, slot(3, 0x2ec)), vec![vec![process, 0]]);
        assert_eq!(
            calls_to(&e, slot(3, 0x4b4)),
            vec![vec![process, 4.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, ACTOR_STOP_MOVING), vec![vec![speaker]]);
        assert_eq!(
            calls_to(&e, INFO_RUN_RESULT),
            vec![vec![0x4400, 1, speaker]]
        );
        assert_eq!(calls_to(&e, slot(3, 0x524)), vec![vec![process, speaker]]);
        assert_eq!(calls_to(&e, slot(3, 0x650)).len(), 2);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6100, 0xfa3, 1]]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![this.addr()]]);
        // Goodbye state 0: no result script.
        e.set(this, DialogMenu::eGoodbyeState, 0);
        e.call_log = Some(vec![]);
        e.call(0x0076_2160, &args![]);
        assert!(calls_to(&e, INFO_RUN_RESULT).is_empty());
    }

    /// `LoadNextScreen`'s callees, doubled; the topic manager's current topic is the word
    /// at `011d8f10`, its response the word at `011d8f14`.
    fn screen_engine() -> (Engine, Ptr<DialogMenu>, u32, u32) {
        let mut e = engine();
        stub_close(&mut e);
        stub(
            &mut e,
            &[
                FORMAT_00406D00,
                COPY_00406D30,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTROY,
                TILE_SET_STRING,
                TILE_SET_INT,
                DEBUG_MESSAGE,
                STRING_CHARS_00408DA0,
                OBJECT_0084E3A0,
                TOPICS_LOAD_NEXT_TOPIC_LIST,
                TOPICS_FIRST_TOPIC,
                RADIO_00833C20,
                SETTING_STRING_00403DF0,
            ],
        );
        e.register(TOPICS_GET_CURRENT_TOPIC, |e, _| {
            e.mem.u32(0x011d_8f10).into_ret()
        });
        e.register(TOPIC_GET_CURRENT_RESPONSE, |e, _| {
            e.mem.u32(0x011d_8f14).into_ret()
        });
        let speaker = fake_object(&mut e, 1);
        let process = fake_object(&mut e, 3);
        e.set_global(0x011d_8f00, process);
        e.register(GET_PROCESS, |e, _| e.mem.u32(0x011d_8f00).into_ret());
        e.register(slot(3, 0x48c), |_, _| 0x4800u32.into_ret());
        let form = fake_object(&mut e, 4);
        e.register(slot(4, 0x130), |_, _| 0x0101_0130u32.into_ret());
        let topic = e.mem.alloc(0x2c);
        e.mem.set_u32(topic + 0x28, form);
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 4, 0x6100);
        e.mem.set_u32(this.addr() + 0x38, 0x6038);
        e.set(this, DialogMenu::pSpeaker, speaker);
        e.map(0x6000, 0x1000);
        e.set(this, DialogMenu::pTopicManager, 0x6200);
        (e, this, topic, speaker)
    }

    #[test]
    fn load_next_screen_shows_a_response() {
        let (mut e, this, topic, speaker) = screen_engine();
        e.set_global(0x011d_8f10, topic);
        let response = e.mem.alloc(8);
        e.mem.set_u32(response, 0x0101_5000);
        e.set_global(0x011d_8f14, response);
        e.set_global(SHOWING_TEXT_TRAIT_011D9500, 0x5d4u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_2ff0, &args![this, 0u32]).u32(), 3);
        let id = calls_to(&e, FORMAT_00406D00);
        assert_eq!(
            id[0],
            vec![this.addr() + 0x84, 0x80, TOPIC_ID_FORMAT, 0x0101_0130, 0]
        );
        assert_eq!(e.get(this, DialogMenu::bResponseShown), 1);
        assert_eq!(
            calls_to(&e, slot(1, 0x264)),
            vec![vec![speaker, 0, response, 1]]
        );
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_ASSIGN),
            vec![vec![this.addr() + 0x74, 0x4800]]
        );
        let copy = calls_to(&e, COPY_00406D30);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0][1..], [0x200, 0x0101_5000]);
        let text = calls_to(&e, TILE_SET_STRING);
        assert_eq!(text, vec![vec![0x6038, 0xfc4, copy[0][0], 1]]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6100, 0x5d4, 1]]);
        // Paused: the state is kept for later.
        e.set(this, DialogMenu::bPaused, 1);
        e.call(0x0076_2ff0, &args![this, 0u32]);
        assert_eq!(e.get(this, DialogMenu::ePausedState), 3);
    }

    #[test]
    fn load_next_screen_closes_without_responses_and_a_goodbye() {
        let (mut e, this, _, _) = screen_engine();
        e.set(this, DialogMenu::eGoodbyeState, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_2ff0, &args![this, 0u32]).u32(), 4);
        assert_eq!(
            calls_to(&e, COPY_00406D30),
            vec![vec![this.addr() + 0x84, 0x80, NO_TOPIC_ID_TEXT]]
        );
        assert_eq!(
            calls_to(&e, DEBUG_MESSAGE),
            vec![vec![NO_RESPONSES_TEXT, NULL_TEXT]]
        );
        assert_eq!(
            calls_to(&e, TOPICS_LOAD_NEXT_TOPIC_LIST),
            vec![vec![0x6200]]
        );
        assert_eq!(e.get(this, DialogMenu::eDialogState), 4);
        assert_eq!(calls_to(&e, BS_AUDIO_EXIT_DIALOGUE).len(), 1);
        // In a service menu outside state 5 nothing more happens: state 2.
        e.set(this, DialogMenu::bInServiceMenu, 1);
        e.set(this, DialogMenu::eDialogState, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_2ff0, &args![this, 0u32]).u32(), 2);
        assert!(calls_to(&e, TOPICS_LOAD_NEXT_TOPIC_LIST).is_empty());
    }

    #[test]
    fn load_next_screen_follows_a_new_info_and_loads_again() {
        let (mut e, this, topic, _) = screen_engine();
        stub(
            &mut e,
            &[
                INFO_00579200,
                INFO_RUN_RESULT,
                LIST_APPEND,
                TOPICS_SET_LIST_0083F1A0,
                TOPICS_GET_GREETING,
                TOPICS_PROCESS_CURRENT_TOPIC,
                FLAG_00619DF0,
                TES_TOPIC_GET_TOPIC,
            ],
        );
        e.register(INFO_0061AF30, |_, _| 0x4500u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        // The new topic's info: no challenge, no prompt.
        e.register(TOPIC_CONSTRUCT_0083DB00, |e, a| {
            let info = e.mem.alloc(0x50);
            e.mem.set_u32(a[0] + 0x18, info);
            // The second screen has a response.
            let response = e.mem.alloc(8);
            e.mem.set_u32(0x011d_8f14, response);
            a[0].into_ret()
        });
        e.set_global(0x011d_8f10, topic);
        e.set_global(LAST_INFO_011D9518, 0x4400u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_2ff0, &args![this, 0u32]).u32(), 3);
        let made = calls_to(&e, TOPIC_CONSTRUCT_0083DB00);
        assert_eq!(made.len(), 1);
        assert_eq!(
            made[0][2..],
            [
                e.mem.u32(topic + 0x28),
                0x4500,
                e.get(this, DialogMenu::pSpeaker),
                0
            ]
        );
        assert_eq!(calls_to(&e, LIST_APPEND).len(), 2);
        assert_eq!(calls_to(&e, TOPICS_FIRST_TOPIC), vec![vec![0x6200, 1]]);
        let info = e.mem.u32(made[0][0] + 0x18);
        assert_eq!(e.global::<u32>(LAST_INFO_011D9518), info);
        // The new topic's response iteration was started.
        assert_eq!(e.mem.u32(made[0][0] + 0x20), made[0][0] + 0xc);
    }

    #[test]
    fn load_topics_list_builds_one_item_per_topic() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                FORMAT_00406D00,
                COPY_00406D30,
                APPEND_00406D50,
                STRING_CHARS_00408DA0,
                TILE_SET_NAME_00707E10,
                TILE_SET_INT,
                TILE_SET_FLOAT,
                TILE_FLAG_SETTER_0071AA60,
                ADD_USER_TRAIT,
                MISC_STAT_INCREMENT,
            ],
        );
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(30.0));
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| 40u32.into_ret());
        e.register(CONDITION_VALUE_00681490, |_, _| st0(50.0));
        e.register(CONDITIONS_00500940, |e, a| {
            e.mem.u32(a[0] + 0x30).into_ret()
        });
        e.register(CONDITION_DATA_00413F40, |_, a| a[0].into_ret());
        // Two topics: the first with a skill prompt, the second plain.
        e.register(TOPICS_FIRST_TOPIC, |e, _| {
            e.mem.set_u32(0x011d_8f20, 0);
            1u32.into_ret()
        });
        e.register(TOPICS_NEXT_TOPIC, |e, _| {
            let index = e.mem.u32(0x011d_8f20) + 1;
            e.mem.set_u32(0x011d_8f20, index);
            u32::from(index < 2).into_ret()
        });
        e.register(TOPICS_GET_CURRENT_TOPIC, |e, _| {
            let index = e.mem.u32(0x011d_8f20);
            e.mem.u32(0x011d_8f24 + 4 * index).into_ret()
        });
        let list_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(list_vtable + 0x1c, 0x00ab_001c);
        e.register(0x00ab_001c, |_, _| Ret::default());
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0x40, list_vtable);
        e.mem.set_u32(this.addr() + 4, 0x6100);
        e.mem.set_u32(this.addr() + 0x3c, 0x603c);
        e.mem.set_u32(this.addr() + 0x40 + 0x14, 0x6014);
        e.set(this, DialogMenu::pTopicManager, 0x6200);
        e.set_global(CURRENT_VALUE_TRAIT_011D9504, 0x2d8u32);
        e.map(TIMER_011F6394 & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_011DEA3C, player);
        let first = e.mem.alloc(0x30);
        let first_info = e.mem.alloc(0x50);
        let prompt = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x18, first_info);
        e.mem.set_u32(first_info + 0x40, prompt);
        e.mem.set_u8(prompt + 4, 0x55);
        let condition = e.mem.alloc(8);
        e.mem.set_u16(condition, 0xe);
        e.mem.set_u32(condition + 4, 0x20);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, condition);
        e.mem.set_u32(first_info + 0x30, node);
        e.mem.set_u8(first + 0x25, 1);
        let second = e.mem.alloc(0x30);
        let second_info = e.mem.alloc(0x50);
        e.mem.set_u32(second + 0x18, second_info);
        e.mem.set_u32(second, 0x0101_2222);
        e.mem.set_u32(0x011d_8f24, first);
        e.mem.set_u32(0x011d_8f28, second);
        e.call_log = Some(vec![]);
        e.call(0x0076_38b0, &args![this]);
        assert_eq!(calls_to(&e, 0x00ab_001c), vec![vec![this.addr() + 0x40]]);
        let formats = calls_to(&e, FORMAT_00406D00);
        let buffer = formats[0][0];
        assert_eq!(formats[0][1..3], [0x200, PROMPT_FORMAT]);
        // 40 of 50: the short form and the item marked.
        assert_eq!(formats[1][1..], [0x200, SKILL_SHORT_FORMAT, buffer, 40, 50]);
        assert_eq!(formats[2][1..4], [0x200, PROMPT_CLOSE_FORMAT, buffer]);
        assert_eq!(formats[3][1..], [0x200, TOPIC_NAME_FORMAT, 0]);
        assert_eq!(
            calls_to(&e, COPY_00406D30),
            vec![vec![buffer, 0x200, 0x0101_2222]]
        );
        assert_eq!(calls_to(&e, TILE_FLAG_SETTER_0071AA60), vec![vec![0, 1]]);
        // Line alpha: 0xff for the said first topic, 0x80 for the second.
        let alphas: Vec<u32> = calls_to(&e, TILE_SET_INT)
            .into_iter()
            .filter(|c| c[1] == 0x7cc)
            .map(|c| c[2])
            .collect();
        assert_eq!(alphas, [0xff, 0x80]);
        // Two tiles of 30 each.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[0],
            vec![0x603c, 0xfb0, 60.0f32.to_bits(), 0]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT)[1],
            vec![0x6014, 0x7b0, 30.0f32.to_bits(), 1]
        );
        assert_eq!(calls_to(&e, TILE_SET_FLOAT)[2], vec![0x6014, 0x2d8, 0, 0]);
        assert_eq!(e.get(this, DialogMenu::uiIgnoreInputStartTime), 0);
    }

    /// `DoIdle`'s callees that every path reaches, doubled.
    fn idle_engine() -> (Engine, Ptr<DialogMenu>, u32, u32) {
        let mut e = engine();
        stub(
            &mut e,
            &[
                INTERFACE_IS_CONSOLE_VISIBLE,
                MOBILE_IS_IN_DIALOGUE_WITH_PLAYER,
                PLAYER_FOCUS_ON_ACTOR,
                PLAYER_UPDATE_CAMERA,
                DOF_0063F790,
                TILE_SET_INT,
            ],
        );
        e.register(SETTING_VALUE_00403E20, |_, a| (a[0] + 0x100).into_ret());
        e.map(0x011d_0000, 0x4000);
        e.mem.set_u32(SETTING_011D318C + 0x100, 2.0f32.to_bits());
        e.mem.set_u32(SETTING_011D243C + 0x100, 0.5f32.to_bits());
        e.mem.set_u32(SETTING_011D3EE0 + 0x100, 0.5f32.to_bits());
        e.register(FRAME_SECONDS_0084D030, |_, _| st0(0.25));
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        let speaker = fake_object(&mut e, 1);
        let process = fake_object(&mut e, 3);
        e.set_global(0x011d_8f00, process);
        e.register(GET_PROCESS, |e, _| e.mem.u32(0x011d_8f00).into_ret());
        e.register(slot(3, 0x610), |_, _| 1u32.into_ret());
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_011DEA3C, player);
        let this = menu(&mut e);
        e.set(this, DialogMenu::pSpeaker, speaker);
        e.set(this, DialogMenu::pDepthOfFieldInstance, 0x5150);
        (e, this, speaker, player)
    }

    #[test]
    fn do_idle_does_nothing_while_paused_and_moves_services_on() {
        let (mut e, this, speaker, player) = idle_engine();
        e.set(this, DialogMenu::bPaused, 1);
        e.call_log = Some(vec![]);
        e.call(0x0076_2950, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set(this, DialogMenu::bPaused, 0);
        e.set(this, DialogMenu::eDialogState, 2);
        e.set(this, DialogMenu::bInServiceMenu, 1);
        e.set(this, DialogMenu::bDoServiceLoad, 1);
        e.set(this, DialogMenu::fPercentZoomed, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0076_2950, &args![this]);
        assert_eq!(e.get(this, DialogMenu::eDialogState), 5);
        // The tail: focus with the zoom (no percent: scale 1) and the setting times it.
        assert_eq!(
            calls_to(&e, PLAYER_FOCUS_ON_ACTOR),
            vec![vec![player, speaker, 0.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, DOF_0063F790),
            vec![vec![0x5150, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn do_idle_counts_the_countdown_down_then_waits_for_the_speaker() {
        let (mut e, this, speaker, _) = idle_engine();
        e.set(this, DialogMenu::eDialogState, 3);
        e.set_global(COUNTDOWN_011D951C, 1.0f32);
        e.set_global(ZOOM_PERCENT_011A008C, 50.0f32);
        e.set(this, DialogMenu::fPercentZoomed, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x0076_2950, &args![this]);
        assert_eq!(e.global::<f32>(COUNTDOWN_011D951C), 0.75);
        assert_eq!(e.get(this, DialogMenu::eDialogState), 3);
        assert_eq!(calls_to(&e, PLAYER_FOCUS_ON_ACTOR)[0][2], 0.5f32.to_bits());
        e.set_global(COUNTDOWN_011D951C, 0.0f32);
        e.mem.set_u8(speaker + 0x7c, 1);
        e.map(TIMER_011F6394 & !0xfff, 0x1000);
        e.mem.set_u32(TIMER_011F6394 + 0x14, 777);
        e.call(0x0076_2950, &args![this]);
        assert_eq!(e.get(this, DialogMenu::eDialogState), 1);
        assert_eq!(e.get(this, DialogMenu::uiIgnoreInputStartTime), 777);
    }

    #[test]
    fn do_idle_closes_when_the_speaker_has_no_process_and_ends_the_fade() {
        let (mut e, this, speaker, player) = idle_engine();
        stub(&mut e, &[BS_AUDIO_INSTANCE, BS_AUDIO_EXIT_DIALOGUE]);
        e.set_global(0x011d_8f00, 0u32);
        e.set(this, DialogMenu::eDialogState, 1);
        let root = fake_object(&mut e, 5);
        e.mem.set_u32(this.addr() + 4, root);
        e.set(this, DialogMenu::fElapsedSecs, 0.25);
        e.set(this, DialogMenu::bIsIn3rdPerson, 1);
        e.call_log = Some(vec![]);
        e.call(0x0076_2950, &args![this]);
        // Closed (state 4) and then the fade: 1 - min(0.25 / 0.5, 1) = 0.5.
        assert_eq!(calls_to(&e, BS_AUDIO_EXIT_DIALOGUE).len(), 1);
        assert_eq!(e.get(this, DialogMenu::eDialogState), 4);
        assert_eq!(e.get(this, DialogMenu::fPercentZoomed), 0.5);
        // A second idle reaches zoom 0: the camera is reset and the tile deleted.
        e.set(this, DialogMenu::fElapsedSecs, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0076_2950, &args![this]);
        assert_eq!(e.get(this, DialogMenu::fPercentZoomed), 0.0);
        assert_eq!(e.global::<u8>(FLAG_011F21D0), 1);
        assert_eq!(e.global::<u8>(FLAG_011F21D1), 0);
        assert_eq!(
            calls_to(&e, PLAYER_FOCUS_ON_ACTOR),
            vec![vec![player, speaker, 0, 1]]
        );
        assert_eq!(calls_to(&e, DOF_0063F790), vec![vec![0x5150, 0]]);
        assert_eq!(calls_to(&e, slot(5, 0)), vec![vec![root, 1]]);
    }

    #[test]
    fn do_click_ignores_closed_states_and_recent_input() {
        let (mut e, this, _, _) = screen_engine();
        stub(&mut e, &[TILE_IS_TRUE, LIST_BOX_BELONGS_TO]);
        e.call_log = Some(vec![]);
        e.call(0x0076_24f0, &args![this, 0i32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set(this, DialogMenu::eDialogState, 3);
        e.map(TIMER_011F6394 & !0xfff, 0x1000);
        e.mem.set_u32(TIMER_011F6394 + 0x14, 1200);
        e.set(this, DialogMenu::uiIgnoreInputStartTime, 1000);
        e.call(0x0076_24f0, &args![this, 0i32, 0u32]);
        assert_eq!(e.get(this, DialogMenu::uiIgnoreInputStartTime), 1000);
        e.mem.set_u32(TIMER_011F6394 + 0x14, 1500);
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        e.call(0x0076_24f0, &args![this, 0i32, 0u32]);
        assert_eq!(e.get(this, DialogMenu::uiIgnoreInputStartTime), 0);
        // `"_ShowingText"` true and id 0: state 1.
        assert_eq!(e.get(this, DialogMenu::eDialogState), 1);
    }

    #[test]
    fn do_click_on_a_challenge_topic_loads_the_challenged_screen() {
        let (mut e, this, topic, _) = screen_engine();
        stub(
            &mut e,
            &[
                TOPICS_PROCESS_CURRENT_TOPIC,
                FLAG_00619DF0,
                TES_TOPIC_GET_TOPIC,
                MISC_STAT_INCREMENT,
            ],
        );
        e.register(LIST_BOX_BELONGS_TO, |_, _| 1u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(2.0));
        e.register(TOPICS_SELECT_0083E430, |_, a| {
            u32::from(a[1] == 2).into_ret()
        });
        e.register(TOPICS_DO_SPEECH_CHALLENGE_CHECK, |_, _| Ret::default());
        e.register(RADIO_00833C20, |_, _| 1u32.into_ret());
        e.register(SETTING_STRING_00403DF0, |_, a| (a[0] + 1).into_ret());
        let info = e.mem.alloc(0x50);
        e.mem.set_u8(info + 0x25, 0x80);
        e.mem.set_u32(topic + 0x18, info);
        e.mem.set_u8(topic + 0x25, 1);
        e.set_global(0x011d_8f10, topic);
        let response = e.mem.alloc(8);
        e.set_global(0x011d_8f14, response);
        e.set(this, DialogMenu::eDialogState, 2);
        e.set_global(OPEN_MENU_011D9510, this.addr());
        e.call_log = Some(vec![]);
        e.call(0x0076_24f0, &args![this, -1i32, 0x6600u32]);
        assert_eq!(e.mem.u8(topic + 0x25), 0);
        assert_eq!(
            calls_to(&e, TOPICS_DO_SPEECH_CHALLENGE_CHECK),
            vec![vec![0x6200, topic]]
        );
        // The failed challenge: the topic manager's +0x14 count is 0, so only the stat.
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0xc]]);
        assert_eq!(e.global::<u32>(LAST_INFO_011D9518), info);
        // The challenged screen: the setting's string without a response text.
        let formats = calls_to(&e, FORMAT_00406D00);
        assert_eq!(
            formats.last().unwrap()[1..],
            [0x200, ONE_STRING_FORMAT, SETTING_011D27FC + 1]
        );
        assert_eq!(e.get(this, DialogMenu::eDialogState), 3);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// The speaker's say slot (vtable +0x264) as the tests place it.
    const SAY_SLOT: u32 = 0x0090_0264;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An open menu with a speaker; `block` is what the 0x2C-byte allocation answers.
    fn service_rig(block: u32) -> (Engine, Ptr<DialogMenu>, u32) {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let menu: Ptr<DialogMenu> = e.new_object();
        e.set(menu, DialogMenu::bInServiceMenu, 1);
        let speaker = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x268);
        e.mem.set_u32(vtable + 0x264, SAY_SLOT);
        e.mem.set_u32(speaker, vtable);
        e.set(menu, DialogMenu::pSpeaker, speaker);
        e.set_global(OPEN_MENU_011D9510, menu.addr());
        e.set_global(PLAYER_011DEA3C, 0x0c10_0000u32);
        e.register(TOPIC_GET_MATCHING_INFO, |_, _| 0x0c20_0000u32.into_ret());
        e.register_double(ALLOCATE, move |_, a| {
            assert_eq!(a[0], 0x2c);
            block.into_ret()
        });
        e.register(TOPIC_INFO_VALUE_00619D00, |_, _| 0x55u32.into_ret());
        e.register(TOPIC_CONSTRUCT_0083DB00, |_, a| a[0].into_ret());
        // The topic's embedded list node has no item.
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(TOPIC_GET_CURRENT_RESPONSE, |_, _| 0x0c30_0000u32.into_ret());
        for callee in [SAY_SLOT, MENU_TOPIC_DELETE, MENU_START_FADE_IN] {
            e.register(callee, |_, _| Ret::default());
        }
        (e, menu, speaker)
    }

    #[test]
    fn service_end_says_the_matching_response() {
        let block = 0x0d00_0000;
        let (mut e, menu, speaker) = service_rig(block);
        e.map(block, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x0076_40a0, &args![0x0c40_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(menu, DialogMenu::bInServiceMenu), 0);
        assert_eq!(
            calls(&log, TOPIC_GET_MATCHING_INFO),
            vec![vec![0x0c40_0000, speaker, 0x0c10_0000]]
        );
        assert_eq!(
            calls(&log, TOPIC_INFO_VALUE_00619D00),
            vec![vec![0x0c40_0000, 0x0c20_0000]]
        );
        assert_eq!(
            calls(&log, TOPIC_CONSTRUCT_0083DB00),
            vec![vec![block, 0x55, 0x0c40_0000, 0x0c20_0000, speaker, 0]]
        );
        // The response iteration starts at the topic's embedded list.
        assert_eq!(e.mem.u32(block + 0x20), block + 0xc);
        assert_eq!(
            calls(&log, SAY_SLOT),
            vec![vec![speaker, 0.0f32.to_bits(), 0x0c30_0000, 0]]
        );
        assert_eq!(e.mem.u8(speaker + 0x7c), 1);
        assert_eq!(calls(&log, MENU_TOPIC_DELETE), vec![vec![block, 1]]);
        assert_eq!(calls(&log, MENU_START_FADE_IN), vec![vec![menu.addr()]]);
    }

    #[test]
    fn service_end_without_a_topic_only_fades_in() {
        let (mut e, menu, _) = service_rig(0);
        e.call_log = Some(vec![]);
        e.call(0x0076_40a0, &args![0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(menu, DialogMenu::bInServiceMenu), 0);
        assert!(calls(&log, TOPIC_GET_MATCHING_INFO).is_empty());
        assert_eq!(calls(&log, MENU_START_FADE_IN), vec![vec![menu.addr()]]);
    }

    #[test]
    fn service_end_without_an_open_menu_does_nothing() {
        let (mut e, _, _) = service_rig(0);
        e.set_global(OPEN_MENU_011D9510, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0076_40a0, &args![0x0c40_0000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn set_pause_pauses_and_resumes() {
        let mut e = Engine::new();
        e.register(SOUND_HANDLE_PLAY, |_, _| Ret::default());
        e.register(SOUND_HANDLE_PAUSE, |_, _| Ret::default());
        let menu: Ptr<DialogMenu> = e.new_object();
        e.set(menu, DialogMenu::eDialogState, 5);
        e.call_log = Some(vec![]);
        // Not paused, go on: nothing.
        e.call(0x0076_4200, &args![menu, 0u32]);
        // Pause (only the low byte of the word counts).
        e.call(0x0076_4200, &args![menu, 0x100u32]);
        assert_eq!(e.get(menu, DialogMenu::bPaused), 0);
        e.call(0x0076_4200, &args![menu, 1u32]);
        assert_eq!(e.get(menu, DialogMenu::bPaused), 1);
        assert_eq!(e.get(menu, DialogMenu::ePausedState), 5);
        assert_eq!(e.get(menu, DialogMenu::eDialogState), 2);
        // Paused, pause again: nothing.
        e.call(0x0076_4200, &args![menu, 1u32]);
        assert_eq!(e.get(menu, DialogMenu::ePausedState), 5);
        // Resume.
        e.call(0x0076_4200, &args![menu, 0u32]);
        assert_eq!(e.get(menu, DialogMenu::bPaused), 0);
        assert_eq!(e.get(menu, DialogMenu::eDialogState), 5);
        let log = e.call_log.take().unwrap();
        let speech = menu.addr() + 0x74;
        assert_eq!(calls(&log, SOUND_HANDLE_PAUSE), vec![vec![speech]]);
        assert_eq!(calls(&log, SOUND_HANDLE_PLAY), vec![vec![speech, 0]]);
    }
}

#[cfg(test)]
mod n087_tests {
    use super::*;

    const VTABLE: u32 = 0x0e00_0000;
    const SLOT: u32 = 0x0e10_0000;
    const EXTERNAL: &[u32] = &[
        INTERFACE_IS_TOP_MENU_ID,
        WORD_AT_4_00726070,
        TILE_IS_TRUE,
        TOPICS_GET_CURRENT_TOPIC,
        TOPIC_GET_CURRENT_RESPONSE,
        TOPICS_FIRST_TOPIC,
        TOPICS_NEXT_TOPIC,
        MENU_START_FADE_OUT,
        LIST_SORT_0083FD60,
        TILE_GET_FLOAT,
        TEXT_TO_TRAIT,
        SCROLL_ROUND_004BD510,
        FTOL2,
        ADD_USER_TRAIT,
        NODE_SELF,
        FREE,
        LIST_DESTRUCT,
        LIST_BOX_UNREGISTER,
        LIST_BOX_REGISTER,
        LIST_BOX_HAS_FLAG,
        LIST_BOX_BELONGS_TO,
        TILE_SET_FLOAT,
        TILE_GET_VALUE,
        TILE_SET_INT,
        TILE_FIND_BY_NAME,
        TILE_GET_CHILD_BY_NAME,
        TILE_ADD_NEEDS_UPDATE,
        TILE_FLAG_GETTER_0071AA80,
        TILE_FLAG_SETTER_0071AA60,
        NODE_SET_NEXT,
        NODE_DELETE,
        ITEM_DATA_DELETE,
        VALUE_CLEAR_ACTIONS,
        VALUE_ADD_ACTION_TILE,
        VALUE_ADD_ACTION_FLOAT,
        VALUE_CALCULATE,
        ROUND_FLOAT,
        LIST_BOX_COUNT_00726C40,
        INT_PAIR_00647B70,
        INIT_TILE_POSITION,
    ];

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x8_0000);
        e.map(0x011d_0000, 0x2_0000);
        for &callee in EXTERNAL {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        let slots: Vec<u32> = (0..0x100).map(|i| SLOT + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        object
    }

    #[test]
    fn do_gamepad_clicks_on_button_9() {
        let mut e = rig();
        let menu = object(&mut e, 0x13c);
        e.mem.set_u32(menu + 0x40, VTABLE);
        assert!(!e.call(0x0076_28c0, &args![menu, 9u32, 0u32]).bool());
        answer(&mut e, INTERFACE_IS_TOP_MENU_ID, 1);
        assert!(!e.call(0x0076_28c0, &args![menu, 8u32, 0u32]).bool());
        answer(&mut e, SLOT + 4, 0x0d00_0001);
        assert!(e.call(0x0076_28c0, &args![menu, 9u32, 0u32]).bool());
        assert_eq!(
            calls(&e, SLOT + 0xc),
            vec![vec![menu, 0xffff_ffff, 0x0d00_0001]]
        );
        assert_eq!(calls(&e, SLOT + 4), vec![vec![menu + 0x40]]);
        answer(&mut e, TILE_IS_TRUE, 1);
        assert!(e.call(0x0076_28c0, &args![menu, 9u32, 0u32]).bool());
        assert_eq!(calls(&e, SLOT + 0xc)[1], vec![menu, 0, 0]);
    }

    #[test]
    fn service_start_says_the_current_response() {
        let mut e = rig();
        let menu = e.mem.alloc(0x13c);
        let speaker = object(&mut e, 0x100);
        e.mem.set_u32(menu + 0x70, 0x0c00_0000);
        e.mem.set_u32(menu + 0x80, speaker);
        e.set_global(OPEN_MENU_011D9510, menu);
        answer(&mut e, TOPICS_GET_CURRENT_TOPIC, 0x0c10_0000);
        answer(&mut e, TOPIC_GET_CURRENT_RESPONSE, 0x0c20_0000);
        e.call(0x0076_3ff0, &args![]);
        assert_eq!(e.mem.u8(menu + 0x138), 1);
        assert_eq!(e.mem.u8(menu + 0x13a), 1);
        assert_eq!(
            calls(&e, SLOT + 0x264),
            vec![vec![speaker, 0.0f32.to_bits(), 0x0c20_0000, 0]]
        );
        assert_eq!(e.mem.u8(speaker + 0x7c), 1);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
        // A goodbye state keeps the menu.
        e.mem.set_u32(menu + 0x2c, 1);
        e.call(0x0076_3ff0, &args![]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT).len(), 1);
    }

    #[test]
    fn debug_reset_marks_every_topic() {
        let mut e = rig();
        let menu = e.mem.alloc(0x13c);
        e.mem.set_u32(menu + 0x70, 0x0c00_0000);
        e.set_global(OPEN_MENU_011D9510, menu);
        let first = e.mem.alloc(0x30);
        let second = e.mem.alloc(0x30);
        let mut topics = vec![first, second, 0];
        e.register_double(TOPICS_GET_CURRENT_TOPIC, move |_, _| {
            topics.remove(0).into_ret()
        });
        e.call(0x0076_4290, &args![]);
        assert_eq!(e.mem.u8(first + 0x25), 1);
        assert_eq!(e.mem.u8(second + 0x25), 1);
        assert_eq!(calls(&e, TOPICS_FIRST_TOPIC), vec![vec![0x0c00_0000, 1]]);
        assert_eq!(calls(&e, TOPICS_NEXT_TOPIC).len(), 2);
    }

    #[test]
    fn sort_sorts_the_list_and_synchronizes() {
        let mut e = rig();
        let list = list_with(&mut e, 0, &[]);
        e.call(0x0076_4900, &args![list, 0x0c30_0000u32]);
        assert_eq!(
            calls(&e, LIST_SORT_0083FD60),
            vec![vec![list + 4, 0x0c30_0000]]
        );
    }

    /// A list box with `count` items whose tiles have `0xfac` = their index; `disabled`
    /// tiles answer false to `"_enabled"`.
    fn list_with(e: &mut Engine, count: u32, disabled: &'static [u32]) -> u32 {
        let list = object(e, 0x30);
        e.mem.set_u16(list + 0x1c, count as u16);
        e.mem.set_u32(list + 0xc, 0x0d00_0100);
        let mut node = list + 4;
        for index in 0..count {
            let item = e.mem.alloc(8);
            e.mem.set_u32(item, 0x0d00_0000 + index);
            e.mem.set_u32(node, item);
            let next = if index + 1 < count { e.mem.alloc(8) } else { 0 };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(TILE_GET_FLOAT, |_, a| f64::from(a[0] & 0xff).into_ret());
        e.register(ADD_USER_TRAIT, |_, _| 0x55u32.into_ret());
        e.register_double(TILE_IS_TRUE, move |_, a| {
            if a[1] == 0x55 {
                u32::from(!disabled.contains(&(a[0] & 0xff))).into_ret()
            } else {
                1u32.into_ret()
            }
        });
        e.register(SLOT + 0x14, |_, a| (0x0d00_0000 + a[1]).into_ret());
        list
    }

    #[test]
    fn highlight_target_steps_and_skips_disabled_items() {
        let mut e = rig();
        let list = list_with(&mut e, 4, &[2]);
        e.mem.set_u32(list + 0x10, 0x0d00_0001);
        // Down from item 1 past the disabled item 2.
        assert_eq!(e.call(0x0076_4a70, &args![list, 2u32]).u32(), 0x0d00_0003);
        // Up from item 1.
        assert_eq!(e.call(0x0076_4a70, &args![list, 1u32]).u32(), 0x0d00_0000);
        // Another direction does not move.
        assert_eq!(e.call(0x0076_4a70, &args![list, 3u32]).u32(), 0);
        // From the last item down: the index stays at the last, which is highlighted.
        e.mem.set_u32(list + 0x10, 0x0d00_0003);
        assert_eq!(e.call(0x0076_4a70, &args![list, 2u32]).u32(), 0);
    }

    #[test]
    fn highlight_target_pages_by_the_scroll_delta() {
        let mut e = rig();
        let list = list_with(&mut e, 6, &[]);
        e.mem.set_u32(list + 0xc, 0x0d00_0100);
        e.register(SCROLL_ROUND_004BD510, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        e.register(TILE_GET_FLOAT, |_, a| {
            let value = match a[1] {
                0xfb0 => 30.0,
                0xfac => f64::from(a[0] & 0xff),
                _ => 10.0,
            };
            value.into_ret()
        });
        e.mem.set_u32(list + 0x10, 0x0d00_0001);
        // Page down: 30 / 10 = 3 items.
        assert_eq!(e.call(0x0076_4a70, &args![list, 16u32]).u32(), 0x0d00_0004);
        let rounded = calls(&e, SCROLL_ROUND_004BD510);
        assert_eq!(rounded[0], vec![3.0f32.to_bits(), 1.0f32.to_bits()]);
        // Page up: kept at the first item.
        assert_eq!(e.call(0x0076_4a70, &args![list, 15u32]).u32(), 0x0d00_0000);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = rig();
        let list = list_with(&mut e, 0, &[]);
        assert_eq!(e.call(0x0076_4e10, &args![list, 1u32]).u32(), list);
        assert_eq!(calls(&e, FREE), vec![vec![list]]);
        e.call(0x0076_4e10, &args![list, 0u32]);
        assert_eq!(calls(&e, FREE).len(), 1);
    }
}
