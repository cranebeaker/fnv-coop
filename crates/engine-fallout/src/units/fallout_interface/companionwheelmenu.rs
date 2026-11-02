//! `fallout/interface/menus/companionwheelmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The companion wheel menu's class (0x433).
const COMPANION_WHEEL_MENU_CLASS: u32 = 0x433;
/// `Tile::GetMenuByClass` (engine map, `00a09030`; `cdecl`, one word) and `Tile::GetMenu`
/// (engine map, `00a03c90`; `thiscall`, no stack words).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) and the type descriptors
/// `.?AVMenu@@` and `.?AVCompanionWheelMenu@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MENU: u32 = 0x0119_f34c;
const TYPE_COMPANION_WHEEL_MENU: u32 = 0x0119_ff50;
/// `CompanionWheelMenu::Create` (engine map, `00754de0`; `cdecl`, one word).
const COMPANION_WHEEL_MENU_CREATE: u32 = 0x0075_4de0;

// Translated from 00754cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::CreateCompanionWheelMenu` (engine map; `cdecl`, one word; the
/// answer in `AL`): when a menu of class 0x433 is open, its menu (`Tile::GetMenu`) cast to
/// `CompanionWheelMenu` (`__RTDynamicCast`) is deleted (its vtable slot 0 with 1); then
/// answers whether `CompanionWheelMenu::Create(argument)` made one.
pub fn companion_wheel_menu_create_companion_wheel_menu(e: &mut Engine, argument: u32) -> bool {
    let tile = e
        .call(GET_MENU_BY_CLASS, &args![COMPANION_WHEEL_MENU_CLASS])
        .u32();
    let menu = if tile != 0 {
        let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        e.call(
            RT_DYNAMIC_CAST,
            &args![menu, 0u32, TYPE_MENU, TYPE_COMPANION_WHEEL_MENU, 0u32],
        )
        .u32()
    } else {
        0
    };
    if menu != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    e.call(COMPANION_WHEEL_MENU_CREATE, &args![argument]).u32() != 0
}

layout! {
    /// `CompanionWheelMenu` (Xbox PDB), a `Menu` (0x28 bytes); the PC code uses the PDB's
    /// offsets (sixteen tiles in `pTiles` from +0x28).
    pub struct CompanionWheelMenu: 0x88 {
        /// `pTiles` (Xbox PDB): `Tile*[16]`.
        0x28 pTiles: u32,
        /// `m_pRelatedCompanion` (Xbox PDB): `Actor*`.
        0x68 m_pRelatedCompanion: u32,
        /// `m_bFollowingLong` (Xbox PDB).
        0x6C m_bFollowingLong: u8,
        /// `m_bAggressiveStance` (Xbox PDB).
        0x6D m_bAggressiveStance: u8,
        /// `m_bRangedCombat` (Xbox PDB).
        0x6E m_bRangedCombat: u8,
        /// `m_bFollowing` (Xbox PDB; set when the companion is waiting).
        0x6F m_bFollowing: u8,
        /// `m_CurrentlySelectedTile` (Xbox PDB): the selected button, -1 for none.
        0x70 m_CurrentlySelectedTile: i32,
        /// `bTalkToOnClose` (Xbox PDB).
        0x74 bTalkToOnClose: u8,
        /// `m_nTimeToClearSubtitle` (Xbox PDB).
        0x78 m_nTimeToClearSubtitle: u32,
        /// `m_hCurrentReactivityBark` (Xbox PDB): a `BSSoundHandle`, 0xc bytes.
        0x7C m_hCurrentReactivityBark: u32,
    }
}

/// `CompanionWheelMenu::pMe` (Xbox PDB static; `011d92b8`).
const OPEN_WHEEL: u32 = 0x011d_92b8;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The `CompanionWheelMenu` vtable (`01071d0c`).
const COMPANION_WHEEL_MENU_VTABLE: u32 = 0x0107_1d0c;
/// `MemoryManager` free (`00401030`, `cdecl`, one word).
const FREE: u32 = 0x0040_1030;
/// The interface manager (`004b7210`, no arguments: the word at `011d8a80`), its root tile
/// (`00586150`, `thiscall`) and `004b71d0` (no arguments: a byte answer).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const MANAGER_ROOT_00586150: u32 = 0x0058_6150;
const INTERFACE_TEST_004B71D0: u32 = 0x004b_71d0;
/// `NonActorMagicCaster::SetCurrentSpell` (engine map name, `0041fd00`; `thiscall` on the
/// interface manager, one word; unconfirmed for this body).
const MANAGER_0041FD00: u32 = 0x0041_fd00;
/// `00483710` (`thiscall`, no stack words): the `BSSoundHandle` destructor body.
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// The `Menu` destructor body (`00a1c520`, `thiscall`).
const MENU_DESTROY: u32 = 0x00a1_c520;
/// `Tile::ReadFile` (engine map, `00a01b00`; `thiscall`, one word), `Menu::SetMenuTile`
/// (`00a1dc70`, two words), `Menu::PrepForVisibility` (`00a1dc20`, one word),
/// `Menu::StartFadeOut` (`00a1d910`) and `Tile::GetMaximumDepth` (`00a01530`, no arguments,
/// `ST0`).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// The menu's tile (`00726070`, `thiscall`: the word at +4, `Menu::pMenu`).
const MENU_TILE_00726070: u32 = 0x0072_6070;
/// `007b78a0` (`thiscall` on the menu, a byte answer: whether the menu's tiles are there).
const MENU_CHECK_007B78A0: u32 = 0x007b_78a0;
/// The tile trait setters: from an integer (`00700320`, two words), a string (`00a01350`,
/// three words: trait, text, 1) and a `float` (`00a012d0`, three words: trait, value, 1);
/// the `float` getter (`00a011b0`, one word, `ST0`).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// The debug message printer (`005b5e40`, `cdecl`).
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
/// `BSAudio::QInstance` (`00453a70`), `BSAudio::GetSoundHandleByName` (`00ad7550`, three
/// words: out handle, name, flags), `BSSoundHandle::Play` (`00ad8830`, one word) and
/// `BSSoundHandle::FadeOutAndRelease` (`00ad8da0`, one word).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_FADE_OUT_AND_RELEASE: u32 = 0x00ad_8da0;
/// An actor's script variable as a `float` (`008c1740`, `thiscall`, one word: the name;
/// `ST0`) and its setter (`008c16c0`, three words: the name and a `double`).
const ACTOR_GET_VARIABLE: u32 = 0x008c_1740;
const ACTOR_SET_VARIABLE: u32 = 0x008c_16c0;
/// A game setting's string (`00403df0`, `thiscall` on the setting) and the address of its
/// value (`00403e20`).
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `Interface::IsTopMenuID` (`00702450`, `cdecl`, one word) and
/// `InterfaceManager::PlayMenuSound` (`00717280`, `cdecl`, one word).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
/// `Actor::CanSpeak` (`00884480`) and `Actor::GetShouldAttackActor` (`008b06d0`, four
/// words).
const ACTOR_CAN_SPEAK: u32 = 0x0088_4480;
const ACTOR_GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
/// The object at `011ddf38` and its test (`0042ce10`, `thiscall`, a byte), and
/// `ProcessLists::Update3DAfterResting` (`00975d10`) on the process lists (`011e0e80`).
const OBJECT_011DDF38: u32 = 0x011d_df38;
const OBJECT_TEST_0042CE10: u32 = 0x0042_ce10;
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LISTS_UPDATE_3D_AFTER_RESTING: u32 = 0x0097_5d10;
/// The time in milliseconds (`00457fe0`: `timeGetTime` through the import slot).
const TIME_MS: u32 = 0x0045_7fe0;
/// `abs` (`00ec7d40`, `cdecl`) and `strlen` (`00ec6130`, `cdecl`).
const ABS: u32 = 0x00ec_7d40;
const STRLEN: u32 = 0x00ec_6130;
/// `005dc330` (`cdecl`, one `float`, `ST0`; it hands the value to `004b1460`).
const MATH_005DC330: u32 = 0x005d_c330;
/// The type descriptors given to `__RTDynamicCast`.
const TYPE_TESFORM: u32 = 0x0118_3028;
const TYPE_TESTOPIC: u32 = 0x0118_4720;
const TYPE_TESBOUNDOBJECT: u32 = 0x0118_3108;
const TYPE_ALCHEMYITEM: u32 = 0x0118_30cc;
const TYPE_TILE: u32 = 0x0119_f4cc;
const TYPE_RADIALTILE: u32 = 0x0119_ff74;
/// `TESForm::GetFormByEditorID` (`00483a00`, `cdecl`, one word) and
/// `BGSDefaultObjectManager::GetDefaultObject` (`0058db10`, `cdecl`, one word).
const GET_FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
/// `TESObjectREFR::GetInventoryItem` (`00576260`, two words), the entry's form (`0044ddc0`:
/// the word at +8), the entry's extra list (`00559450` then `006815c0`, the word there) and
/// the entry's deleting destructor (`004459e0`, one word).
const GET_INVENTORY_ITEM: u32 = 0x0057_6260;
const WORD_AT_8_0044DDC0: u32 = 0x0044_ddc0;
const WORD_AT_0_00559450: u32 = 0x0055_9450;
const NODE_SELF_006815C0: u32 = 0x0068_15c0;
const ENTRY_DELETE_004459E0: u32 = 0x0044_59e0;
/// The companion's calls: `Actor::GetHealthPercentage` (`00893590`, `ST0`),
/// `CombatFormulas::GetBodyPartCondition` (`00646800`, `cdecl`, three words, `ST0`),
/// `Actor::DrinkPotion` (`008c1f80`, three words), `Actor::UpdateMagic` (`008c3c40`, two
/// words), `Actor::RestoreActorValue` (`0088b740`, two words: value, `float`), `008a7760`,
/// `008a0cd0` (`cdecl`, two words, `ST0`) and `008a0c20` (`ST0`).
const ACTOR_GET_HEALTH_PERCENTAGE: u32 = 0x0089_3590;
const GET_BODY_PART_CONDITION: u32 = 0x0064_6800;
const ACTOR_DRINK_POTION: u32 = 0x008c_1f80;
const ACTOR_UPDATE_MAGIC: u32 = 0x008c_3c40;
const ACTOR_RESTORE_ACTOR_VALUE: u32 = 0x0088_b740;
const ACTOR_008A7760: u32 = 0x008a_7760;
const ACTOR_VALUE_008A0CD0: u32 = 0x008a_0cd0;
const ACTOR_VALUE_008A0C20: u32 = 0x008a_0c20;
/// `FollowerBarks::GetBarkTopicByType` (`008d5910`, `cdecl`, one word).
const GET_BARK_TOPIC_BY_TYPE: u32 = 0x008d_5910;
/// `ItemChange::GetFullName` (engine map, `004be2d0`).
const ITEM_GET_FULL_NAME: u32 = 0x004b_e2d0;
/// The bounded formatter (`00406d00`, `cdecl`: buffer, size, format, ...) and `_ftol2`
/// (`00ec62c0`, `ST0`).
const FORMAT_00406D00: u32 = 0x0040_6d00;
const FTOL2: u32 = 0x00ec_62c0;
/// `TESTopic::GetMatchingInfo` (`0061a720`, two words), `TESTopicInfo::GetResultScript`
/// (`0061eb60`, one word), `00500940` (the script's data), the companion's extra list
/// (`005d43c0`), `ExtraDataList::GetScriptLocals` (`00418830`) and `Script::Run`
/// (`005ac1e0`, four words).
const TOPIC_GET_MATCHING_INFO: u32 = 0x0061_a720;
const INFO_GET_RESULT_SCRIPT: u32 = 0x0061_eb60;
const SCRIPT_DATA_00500940: u32 = 0x0050_0940;
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
const EXTRA_GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
const SCRIPT_RUN: u32 = 0x005a_c1e0;
/// `LowProcess::GetSecondGenericLocation` (engine map name, `006733e0`; `thiscall` on the
/// info: a word handed to the `DialogueItem` constructor), the `DialogueItem` constructor
/// (`0083c520`, four words), `FirstResponse` (`0083c7b0`), `GetCurrentResponse`
/// (`0083c820`) and the destructor (`0083c670`).
const INFO_WORD_006733E0: u32 = 0x0067_33e0;
const DIALOGUE_ITEM_CONSTRUCT: u32 = 0x0083_c520;
const DIALOGUE_ITEM_FIRST_RESPONSE: u32 = 0x0083_c7b0;
const DIALOGUE_ITEM_GET_CURRENT_RESPONSE: u32 = 0x0083_c820;
const DIALOGUE_ITEM_DESTROY: u32 = 0x0083_c670;
/// The response's calls (all `thiscall`, no stack words): `007af430` (a form),
/// `0084e3a0`, `00441110`, `009611e0`, `004048e0` (on its text), `00460140`; the
/// companion's speech handle (`00933150`, five words), the sound handle's assignment
/// (`00418900`, one word: also its copy constructor), the `BSStringT` constructor from a
/// `char*` (`004047f0`, one word) and its destructor (`004037d0`).
const RESPONSE_FORM_007AF430: u32 = 0x007a_f430;
const VALUE_OF_0084E3A0: u32 = 0x0084_e3a0;
const RESPONSE_00441110: u32 = 0x0044_1110;
const RESPONSE_009611E0: u32 = 0x0096_11e0;
const STRING_LENGTH_004048E0: u32 = 0x0040_48e0;
const RESPONSE_00460140: u32 = 0x0046_0140;
const ACTOR_SPEECH_HANDLE_00933150: u32 = 0x0093_3150;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const STRING_CONSTRUCT: u32 = 0x0040_47f0;
const STRING_DESTROY: u32 = 0x0040_37d0;
/// The subtitles setting's text (`00408d60` on `011d8928`), `Actor::AlwaysShowSubtitles`
/// (`008c1bc0`) and `BSSoundHandle::GetDuration` (`00ad8b30`).
const STRING_CHARS_00408D60: u32 = 0x0040_8d60;
const SUBTITLES_SETTING: u32 = 0x011d_8928;
const ACTOR_ALWAYS_SHOW_SUBTITLES: u32 = 0x008c_1bc0;
const SOUND_HANDLE_GET_DURATION: u32 = 0x00ad_8b30;

/// `"Data\Menus\companion_wheel_menu.xml"`, the creation failure message,
/// `"UIPopUpMessageGeneral"`, `"UIPipBoyScroll"` and `""`.
const MENU_FILE_TEXT: u32 = 0x0107_1dfc;
const CREATION_FAILED_TEXT: u32 = 0x0107_1d58;
const POP_UP_SOUND_TEXT: u32 = 0x0102_5cc4;
const SCROLL_SOUND_TEXT: u32 = 0x0107_1e30;
const EMPTY_TEXT: u32 = 0x0101_1584;
/// The companion's variables `"FollowerSwitchAggressive"`, `"IsFollowingLong"`,
/// `"CombatStyleRanged"` and `"Waiting"`.
const SWITCH_AGGRESSIVE_TEXT: u32 = 0x0107_1de0;
const FOLLOWING_LONG_TEXT: u32 = 0x0107_1dd0;
const COMBAT_STYLE_RANGED_TEXT: u32 = 0x0107_1dbc;
const WAITING_TEXT: u32 = 0x0107_1db4;
/// The context formats `"%s  %d/%d\n%s  %d"` (`01071e8c`), `"%s"` (`01019f08`) and
/// `"%s  %d/%d"` (`01071e80`).
const STIMPAK_FORMAT: u32 = 0x0107_1e8c;
const NAME_FORMAT: u32 = 0x0101_9f08;
const HEALTH_FORMAT: u32 = 0x0107_1e80;
/// The follower topics' editor ids.
const DISTANCE_LONG_TEXT: u32 = 0x0107_1ec0;
const DISTANCE_DEFAULT_TEXT: u32 = 0x0107_1ea0;
const COMBAT_AGGRESSIVE_TEXT: u32 = 0x0107_1f00;
const COMBAT_PASSIVE_TEXT: u32 = 0x0107_1ee0;
const COMBAT_RANGED_TEXT: u32 = 0x0107_1f40;
const COMBAT_MELEE_TEXT: u32 = 0x0107_1f24;
const FOLLOWERS_WAIT_TEXT: u32 = 0x0107_1f70;
const FOLLOWERS_LETS_GO_TEXT: u32 = 0x0107_1f60;
const FOLLOWERS_TRADE_TEXT: u32 = 0x0107_1f80;
/// `0.0` and `1.0` (`double`s), the `6006` and `102` the menu's trait `0xfa7` is compared
/// with (`double`s).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const VALUE_6006_DOUBLE: u32 = 0x0107_04f0;
const VALUE_102_DOUBLE: u32 = 0x0107_04e8;
/// The `float` the use-stimpak path restores each value by (`01013974`).
const RESTORE_AMOUNT: u32 = 0x0101_3974;
/// The `double` the subtitle time is scaled by (`01017b70`) and its game setting
/// (`011d2178`, a `float` per character).
const SUBTITLE_SCALE_DOUBLE: u32 = 0x0101_7b70;
const SUBTITLE_SECONDS_SETTING: u32 = 0x011d_2178;
/// The thumbstick words (`011d8a5c`, `011d8a5e`, `011d8a74`; signed 16-bit).
const STICK_X: u32 = 0x011d_8a5c;
const STICK_Y: u32 = 0x011d_8a5e;
const STICK_OTHER: u32 = 0x011d_8a74;
/// The stick angle constants: the angles with an upright or flat stick (`01071e2c`,
/// `01071e28`, `float`s), the half and full turns added (`01071e20`, `0104eea0`, `double`s).
const ANGLE_UP: u32 = 0x0107_1e2c;
const ANGLE_DOWN: u32 = 0x0107_1e28;
const HALF_TURN_DOUBLE: u32 = 0x0107_1e20;
const FULL_TURN_DOUBLE: u32 = 0x0104_eea0;
/// The angle bands `SetButtonAsHighlighted(float)` snaps (`double` bounds, `float`
/// results).
const BAND_1_HIGH: u32 = 0x0107_1e78;
const BAND_1_VALUE: u32 = 0x0104_e2b4;
const BAND_2_LOW: u32 = 0x0107_1e70;
const BAND_2_VALUE: u32 = 0x0107_1e68;
const BAND_3_LOW: u32 = 0x0107_1e60;
const BAND_3_HIGH: u32 = 0x0107_1e58;
const BAND_3_VALUE: u32 = 0x0107_1e50;
const BAND_4_LOW: u32 = 0x0107_1e48;
const BAND_4_VALUE: u32 = 0x0107_1e40;

/// The settings whose strings the buttons' context line shows (`Setting` objects).
const SETTING_011D3534: u32 = 0x011d_3534;
const SETTING_011D33FC: u32 = 0x011d_33fc;
const SETTING_011D3EC8: u32 = 0x011d_3ec8;
const SETTING_011D27D8: u32 = 0x011d_27d8;
const SETTING_011D4288: u32 = 0x011d_4288;
const SETTING_011D2D90: u32 = 0x011d_2d90;
const SETTING_011D4540: u32 = 0x011d_4540;
const SETTING_011D288C: u32 = 0x011d_288c;
const SETTING_011D40CC: u32 = 0x011d_40cc;
const SETTING_011D4FA8: u32 = 0x011d_4fa8;
const SETTING_011D2268: u32 = 0x011d_2268;
const SETTING_011D4660: u32 = 0x011d_4660;
const SETTING_011D2C88: u32 = 0x011d_2c88;
const SETTING_011D34C8: u32 = 0x011d_34c8;
const SETTING_011D3BA4: u32 = 0x011d_3ba4;
const SETTING_011D2598: u32 = 0x011d_2598;
const SETTING_011D300C: u32 = 0x011d_300c;
const SETTING_011D2568: u32 = 0x011d_2568;
const SETTING_011D1E60: u32 = 0x011d_1e60;

/// The open wheel (`pMe`).
fn open_wheel(e: &Engine) -> u32 {
    e.global(OPEN_WHEEL)
}

/// `pTiles[index]` of `menu` (the code indexes without a bounds check).
fn tile_at(e: &Engine, menu: u32, index: u32) -> u32 {
    e.mem
        .u32(menu.wrapping_add(0x28).wrapping_add(index.wrapping_mul(4)))
}

/// A tile's integer trait (`00700320`).
fn set_int(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
}

/// A tile's string trait (`00a01350` with 1).
fn set_string(e: &mut Engine, tile: u32, trait_id: u32, text: u32) {
    e.call(TILE_SET_STRING, &args![tile, trait_id, text, 1u32]);
}

/// A game setting's string (`00403df0`).
fn setting_string(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_STRING, &args![setting]).u32()
}

/// The code's `NEG; SBB; ADD 1` on a byte: 1 when it is 0, else 0.
fn is_zero(byte: u8) -> u32 {
    u32::from(byte == 0)
}

/// Plays the named interface sound once: `BSAudio::GetSoundHandleByName(local, name, 0x121)`
/// on `BSAudio::QInstance()`, `Play(0)` on the handle it answers, and the local handle's
/// destructor.
fn play_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, local| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let handle = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, local, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![local]);
    });
}

/// The `TESTopic` with the editor id `name` (`TESForm::GetFormByEditorID`, cast from
/// `TESForm` to `TESTopic` by `__RTDynamicCast`).
fn topic_by_editor_id(e: &mut Engine, name: u32) -> u32 {
    let form = e.call(GET_FORM_BY_EDITOR_ID, &args![name]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TESFORM, TYPE_TESTOPIC, 0u32],
    )
    .u32()
}

/// The shape every follower command of the wheel shares: the topic of `name`, its
/// reactivity bark and its command script.
fn follower_command(e: &mut Engine, this: Ptr<CompanionWheelMenu>, name: u32) {
    let topic = topic_by_editor_id(e, name);
    companion_wheel_menu_play_reactivity_bark(e, this, topic);
    companion_wheel_menu_run_follower_command_script(e, this, topic);
}

/// The companion's actor-value owner (`companion + 0xa4`, 0 for no companion).
fn values_of(companion: u32) -> u32 {
    if companion == 0 {
        0
    } else {
        companion.wrapping_add(0xa4)
    }
}

/// The player's stimpak entry: the default object 0 (`GetDefaultObject(0)` cast from
/// `TESForm` to `TESBoundObject`) and the player's inventory entry for it
/// (`GetInventoryItem(object, 0)`); 0 when either is missing.
fn stimpak_entry(e: &mut Engine) -> u32 {
    let object = e.call(GET_DEFAULT_OBJECT, &args![0u32]).u32();
    let object = e
        .call(
            RT_DYNAMIC_CAST,
            &args![object, 0u32, TYPE_TESFORM, TYPE_TESBOUNDOBJECT, 0u32],
        )
        .u32();
    if object == 0 {
        return 0;
    }
    let player: u32 = e.global(PLAYER);
    e.call(GET_INVENTORY_ITEM, &args![player, object, 0u32])
        .u32()
}

/// An inventory entry's extra list word (`00559450`; when set, the word at
/// `006815c0` of it), 0 otherwise.
fn entry_extra(e: &mut Engine, entry: u32) -> u32 {
    if e.call(WORD_AT_0_00559450, &args![entry]).u32() == 0 {
        return 0;
    }
    let list = e.call(WORD_AT_0_00559450, &args![entry]).u32();
    let node = e.call(NODE_SELF_006815C0, &args![list]).u32();
    e.mem.u32(node)
}

// Translated from 00754bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::GetClass` (engine map; `thiscall`): `0x433`.
pub fn companion_wheel_menu_get_class(_e: &mut Engine, _this: Ptr<CompanionWheelMenu>) -> u32 {
    COMPANION_WHEEL_MENU_CLASS
}

// Translated from 00754c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00754c60 - the C++ exception frame (FS:[0] chain, handler 00f0a4d3, unwind states 1 and 0 for the sound handle and the Menu base) is not translated
/// The `CompanionWheelMenu` destructor body (unnamed in the engine map; `thiscall`): the
/// vtable `01071d0c`; with `bTalkToOnClose` set, `0041fd00(1)` on the interface manager and
/// the talk-to (`00756980`); `pMe` cleared, the bark handle (+0x7c) destroyed (`00483710`)
/// and the `Menu` destructor body (`00a1c520`).
pub fn fn_00754c60(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    e.mem.set_u32(this.addr(), COMPANION_WHEEL_MENU_VTABLE);
    if e.get(this, CompanionWheelMenu::bTalkToOnClose) != 0 {
        let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
        e.call(MANAGER_0041FD00, &args![manager, 1u32]);
        fn_00756980(e, this);
    }
    e.set_global(OPEN_WHEEL, 0u32);
    e.call(SOUND_HANDLE_DESTROY, &args![this.addr() + 0x7c]);
    e.call(MENU_DESTROY, &args![this]);
}

// Translated from 00754c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::scalar deleting destructor` (engine map; `RET 4`): the destructor
/// body (`00754c60`), then the memory freed (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn companion_wheel_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    flags: u32,
) -> Ptr<CompanionWheelMenu> {
    fn_00754c60(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00754d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::GetShouldCompanionWheelOpen` (engine map; `cdecl`, one word; the
/// answer in `AL`): false for no actor or one that cannot speak (`Actor::CanSpeak`); else
/// whether `Actor::GetShouldAttackActor(player, 0, &local, 0)` answers false.
pub fn companion_wheel_menu_get_should_companion_wheel_open(e: &mut Engine, actor: u32) -> bool {
    if actor == 0 {
        return false;
    }
    if !e.call(ACTOR_CAN_SPEAK, &args![actor]).bool() {
        return false;
    }
    e.with_stack(4, |e, local| {
        let player: u32 = e.global(PLAYER);
        !e.call(
            ACTOR_GET_SHOULD_ATTACK_ACTOR,
            &args![actor, player, 0u32, local, 0u32],
        )
        .bool()
    })
}

/// The flag `Create` reads from a companion variable: 1 when `008c1740(name)` equals the
/// `double` at `constant` (`FCOMP; TEST AH,0x44; JP`: ordered equal).
fn variable_is(e: &mut Engine, name: u32, constant: u32) -> u8 {
    let companion = e.mem.u32(open_wheel(e) + 0x68);
    let value = e.call(ACTOR_GET_VARIABLE, &args![companion, name]).f64();
    let constant: f64 = e.global(constant);
    u8::from(value == constant)
}

// Translated from 00754de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00754de0 - the C++ exception frame (FS:[0] chain, handler 00f0a4f8, unwind state 0 for the sound handle) is not translated
/// `CompanionWheelMenu::Create` (engine map; `cdecl`, one word: the companion). Returns the
/// menu's tile, or 0.
///
/// An open `0x433` menu (`Tile::GetMenuByClass`) is deleted (its vtable slot 0 with 1).
/// `companion_wheel_menu.xml` is read (`Tile::ReadFile` on the interface manager's root) and
/// its menu becomes `pMe`. When that is a `0x433` menu (vtable +0x34): the companion is
/// stored; with a companion, `m_bAggressiveStance`, `m_bFollowingLong` and `m_bRangedCombat`
/// are whether its `"FollowerSwitchAggressive"`, `"IsFollowingLong"` and
/// `"CombatStyleRanged"` are 1.0, `m_bFollowing` whether its `"Waiting"` is 0.0. Tiles 0, 2,
/// 5 and 7 get trait `0x100f` from the flags (tile 2 the inverse), tile 12 an empty string,
/// tiles 8, 13, 14 and 15 the strings of four settings as trait `0xfc4`. The tile becomes
/// the menu's (`Menu::SetMenuTile`); when `007b78a0` answers false a message is logged and
/// 0 returned. Otherwise `"UIPopUpMessageGeneral"` plays, a trait `0xfa7` of 6006 or 102
/// gets the maximum depth as trait `0xfad`, the menu is prepared for visibility and the
/// tile returned. When no `0x433` menu was made, a menu that has a tile is deleted (vtable
/// slot 0 with 1) and 0 returned.
pub fn companion_wheel_menu_create(e: &mut Engine, companion: u32) -> u32 {
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let class = fn_00755260(e);
    let open = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if open != 0 {
        e.vcall(open, 0, &args![1u32]);
    }
    let root = e.call(MANAGER_ROOT_00586150, &args![manager]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.set_global(OPEN_WHEEL, menu);
    let made = open_wheel(e) != 0 && {
        let class = e.vcall(open_wheel(e), 0x34, &args![]).u32();
        class == fn_00755260(e)
    };
    if !made {
        if open_wheel(e) != 0 && e.call(MENU_TILE_00726070, &args![open_wheel(e)]).u32() != 0 {
            let menu = open_wheel(e);
            if menu != 0 {
                e.vcall(menu, 0, &args![1u32]);
            }
        }
        return 0;
    }
    e.mem.set_u32(open_wheel(e) + 0x68, companion);
    if e.mem.u32(open_wheel(e) + 0x68) != 0 {
        let flag = variable_is(e, SWITCH_AGGRESSIVE_TEXT, ONE_DOUBLE);
        e.mem.set_u8(open_wheel(e) + 0x6d, flag);
        let flag = variable_is(e, FOLLOWING_LONG_TEXT, ONE_DOUBLE);
        e.mem.set_u8(open_wheel(e) + 0x6c, flag);
        let flag = variable_is(e, COMBAT_STYLE_RANGED_TEXT, ONE_DOUBLE);
        e.mem.set_u8(open_wheel(e) + 0x6e, flag);
        let flag = variable_is(e, WAITING_TEXT, ZERO_DOUBLE);
        e.mem.set_u8(open_wheel(e) + 0x6f, flag);
    }
    for (index, flag, inverse) in [
        (0u32, 0x6du32, false),
        (2, 0x6f, true),
        (5, 0x6c, false),
        (7, 0x6e, false),
    ] {
        if tile_at(e, open_wheel(e), index) != 0 {
            let byte = e.mem.u8(open_wheel(e) + flag);
            let value = if inverse {
                is_zero(byte)
            } else {
                u32::from(byte)
            };
            let target = tile_at(e, open_wheel(e), index);
            set_int(e, target, 0x100f, value);
        }
    }
    if tile_at(e, open_wheel(e), 12) != 0 {
        let target = tile_at(e, open_wheel(e), 12);
        set_string(e, target, 0xfc4, EMPTY_TEXT);
    }
    for (index, setting) in [
        (8u32, SETTING_011D3534),
        (13, SETTING_011D33FC),
        (14, SETTING_011D3EC8),
        (15, SETTING_011D27D8),
    ] {
        if tile_at(e, open_wheel(e), index) != 0 {
            let text = setting_string(e, setting);
            let target = tile_at(e, open_wheel(e), index);
            set_string(e, target, 0xfc4, text);
        }
    }
    let menu = open_wheel(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
    let menu = open_wheel(e);
    if !e.call(MENU_CHECK_007B78A0, &args![menu]).bool() {
        e.call(DEBUG_MESSAGE, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
    play_sound(e, POP_UP_SOUND_TEXT);
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
    let menu = open_wheel(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    tile
}

// Translated from 00755260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the wheel's menu class, `0x433`.
pub fn fn_00755260(_e: &mut Engine) -> u32 {
    COMPANION_WHEEL_MENU_CLASS
}

// Translated from 00755270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::Close` (engine map; `cdecl`, one word: a byte). With a wheel open:
/// when `fade` is set its bark handle fades out (`FadeOutAndRelease(100)`); unless the
/// object at `011ddf38` answers `0042ce10`, `ProcessLists::Update3DAfterResting`; the
/// wheel's tile gets trait `0x1772` = 1 and the wheel starts fading out.
pub fn companion_wheel_menu_close(e: &mut Engine, fade: u8) {
    if open_wheel(e) == 0 {
        return;
    }
    if fade != 0 {
        let handle = open_wheel(e) + 0x7c;
        e.call(SOUND_HANDLE_FADE_OUT_AND_RELEASE, &args![handle, 100u32]);
    }
    let object: u32 = e.global(OBJECT_011DDF38);
    if !e.call(OBJECT_TEST_0042CE10, &args![object]).bool() {
        e.call(PROCESS_LISTS_UPDATE_3D_AFTER_RESTING, &args![PROCESS_LISTS]);
    }
    let menu = open_wheel(e);
    let tile = e.call(MENU_TILE_00726070, &args![menu]).u32();
    set_int(e, tile, 0x1772, 1);
    let menu = open_wheel(e);
    e.call(MENU_START_FADE_OUT, &args![menu]);
}

// Translated from 007552e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::DoClick` (engine map; `RET 8`: the button id and its tile, which
/// the code fills from `pTiles` when 0 but does not use). Only while the wheel is the top
/// menu (`Interface::IsTopMenuID(0x433)`), by button (the code's jump table):
/// 0 aggressive/passive, 1 use stimpak, 2 stay/follow, 5 near/far and 7 ranged/melee run
/// their command and highlight the button again (`SetButtonAsHighlighted(id, 1)`); 3 talk
/// (`00756960`), 4 open the inventory (`00756930`) and 6 trade (`007568d0`) run theirs;
/// each then plays menu sound 1. Button 11 plays menu sound 1 and closes the wheel with the
/// fade; other ids do nothing.
pub fn companion_wheel_menu_do_click(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    id: i32,
    _tile: u32,
) {
    if !e
        .call(IS_TOP_MENU_ID, &args![COMPANION_WHEEL_MENU_CLASS])
        .bool()
    {
        return;
    }
    let highlight = match id {
        0 => {
            companion_wheel_menu_do_toggle_aggressive_passive(e, this);
            true
        }
        1 => {
            companion_wheel_menu_do_use_stim(e, this);
            true
        }
        2 => {
            companion_wheel_menu_do_toggle_stay_follow(e, this);
            true
        }
        3 => {
            fn_00756960(e, this);
            false
        }
        4 => {
            fn_00756930(e, this);
            false
        }
        5 => {
            companion_wheel_menu_do_toggle_near_far(e, this);
            true
        }
        6 => {
            fn_007568d0(e, this);
            false
        }
        7 => {
            companion_wheel_menu_do_toggle_ranged_melee(e, this);
            true
        }
        11 => {
            e.call(PLAY_MENU_SOUND, &args![1u32]);
            companion_wheel_menu_close(e, 1);
            return;
        }
        _ => return,
    };
    if highlight {
        companion_wheel_menu_set_button_as_highlighted(e, this, id, 1);
    }
    e.call(PLAY_MENU_SOUND, &args![1u32]);
}

// Translated from 00755480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::DoIdle` (engine map; `thiscall`). A subtitle whose clear time
/// (`m_nTimeToClearSubtitle`) has passed (`00457fe0` above it, unsigned) is cleared (the
/// time to 0, tile 12's trait `0xfc4` to `""`). Then, while the wheel is the top menu, when
/// `004b71d0` answers true and the interface manager's `00755680` false (both asked twice)
/// and either stick word (`abs` of `011d8a5e` or `011d8a74`) is above `0x1ea9`, the stick
/// (`011d8a5c`, minus `011d8a5e`) gives an angle: `00755660(-x / y)` for a non-zero `y`
/// (else `01071e2c` for `x` above 0, `01071e28` otherwise), plus a half turn for `y` above
/// 0, plus a full turn when both are below 0; the button there is highlighted
/// (`SetButtonAsHighlighted(float)`).
pub fn companion_wheel_menu_do_idle(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    let base = this.addr();
    if e.mem.u32(base + 0x78) != 0 {
        let now = e.call(TIME_MS, &args![]).u32();
        if now > e.mem.u32(base + 0x78) {
            e.mem.set_u32(base + 0x78, 0);
            if e.mem.u32(base + 0x58) != 0 {
                let tile = e.mem.u32(base + 0x58);
                set_string(e, tile, 0xfc4, EMPTY_TEXT);
            }
        }
    }
    if !e
        .call(IS_TOP_MENU_ID, &args![COMPANION_WHEEL_MENU_CLASS])
        .bool()
    {
        return;
    }
    let manager_ready = |e: &mut Engine| {
        e.call(INTERFACE_MANAGER, &args![]);
        if !e.call(INTERFACE_TEST_004B71D0, &args![]).bool() {
            return false;
        }
        let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
        !fn_00755680(e, Ptr::new(manager))
    };
    // The code starts the stick's x and y at 0.0 and goes on only after reading them (FILD
    // of the words, FSTP to floats).
    let moved = manager_ready(e) && manager_ready(e) && {
        let first = i32::from(e.mem.u16(STICK_Y) as i16);
        e.call(ABS, &args![first]).i32() > 0x1ea9 || {
            let second = i32::from(e.mem.u16(STICK_OTHER) as i16);
            e.call(ABS, &args![second]).i32() > 0x1ea9
        }
    };
    if !moved {
        return;
    }
    let x = x87_store32(f64::from(e.mem.u16(STICK_X) as i16));
    let mut y = x87_store32(f64::from(e.mem.u16(STICK_Y) as i16));
    // FCHS on the loaded float, stored back.
    y ^= 0x8000_0000;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let mut angle = if x87_load32(y) == zero {
        // FCOMP; TEST AH,0x41; JNZ: the up angle only when x is strictly above 0.
        if x87_load32(x) > zero {
            x87_quiet_bits(e.mem.u32(ANGLE_UP))
        } else {
            x87_quiet_bits(e.mem.u32(ANGLE_DOWN))
        }
    } else {
        // FLD x; FCHS; FDIV y; FSTP to a local, then that float is pushed.
        let quotient = x87_store32(x87_div(-x87_load32(x), x87_m32(y)));
        x87_store32(fn_00755660(e, f32::from_bits(quotient)))
    };
    if x87_load32(y) > zero {
        let half: f64 = e.global(HALF_TURN_DOUBLE);
        angle = x87_store32(x87_add(x87_load32(angle), half));
    }
    // TEST AH,5; JP: goes on only when strictly below (ordered).
    if x87_load32(x) < zero && x87_load32(y) < zero {
        let full: f64 = e.global(FULL_TURN_DOUBLE);
        angle = x87_store32(x87_add(x87_load32(angle), full));
    }
    companion_wheel_menu_set_button_as_highlighted_ov2(e, this, x87_quiet_bits(angle));
}

// Translated from 00755660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one `float`; `ST0`): passes the value on to
/// `005dc330` (an `FLD`/`FSTP` copy of it) and answers what that answers.
pub fn fn_00755660(e: &mut Engine, value: f32) -> f64 {
    e.call(MATH_005DC330, &args![x87_quiet_bits(value.to_bits())])
        .f64()
}

// Translated from 00755680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the interface manager; the answer in `AL`):
/// true when `004b71d0` answers false or the manager's byte +0x7d is 1.
pub fn fn_00755680(e: &mut Engine, this: Ptr) -> bool {
    !e.call(INTERFACE_TEST_004B71D0, &args![]).bool() || e.mem.u8(this.addr() + 0x7d) == 1
}

// Translated from 007556b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::DoGamepad` (engine map; `RET 8`: the button and a word it does not
/// read; the answer in `AL`). Only while the wheel is the top menu: button 9 clicks the
/// selected button (vtable +0xC with the id and its tile) or, with none selected, button 11
/// with `pTiles[11]`; button 10 clicks button 11; both answer true. Everything else answers
/// false.
pub fn companion_wheel_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    if !e
        .call(IS_TOP_MENU_ID, &args![COMPANION_WHEEL_MENU_CLASS])
        .bool()
    {
        return false;
    }
    let base = this.addr();
    let click_close = |e: &mut Engine| {
        let tile = e.mem.u32(base + 0x54);
        e.vcall(base, 0xc, &args![0xbu32, tile]);
    };
    match button {
        9 => {
            let selected = e.mem.u32(base + 0x70);
            if selected != 0xffff_ffff {
                let tile = tile_at(e, base, selected);
                e.vcall(base, 0xc, &args![selected, tile]);
            } else {
                click_close(e);
            }
            true
        }
        10 => {
            click_close(e);
            true
        }
        _ => false,
    }
}

// Translated from 00755760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::DoEnter` (engine map; `RET 8`: the button id and its tile, which the
/// code fills from `pTiles` when 0 but does not use): while the wheel is the top menu, a
/// button 0 to 7 or 11 is highlighted (`SetButtonAsHighlighted(id, 0)`).
pub fn companion_wheel_menu_do_enter(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    id: i32,
    _tile: u32,
) {
    if !e
        .call(IS_TOP_MENU_ID, &args![COMPANION_WHEEL_MENU_CLASS])
        .bool()
    {
        return;
    }
    if (0..=7).contains(&id) || id == 0xb {
        companion_wheel_menu_set_button_as_highlighted(e, this, id, 0);
    }
}

// Translated from 007557d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007557d0 - the C++ exception frame (FS:[0] chain, handler 00f0a528, unwind state 0 for the sound handle) is not translated
/// `CompanionWheelMenu::SetButtonAsHighlighted` (engine map; `RET 8`: the button id and a
/// byte that forces it). Nothing happens when the button is already selected and not
/// forced. Otherwise `"UIPipBoyScroll"` plays; the old selection's tile (when there is one)
/// gets trait `0x100e` = 0 and, for buttons 0, 2, 5 and 7, trait `0x100f` from
/// `m_bAggressiveStance`, the inverse of `m_bFollowing`, `m_bFollowingLong` and
/// `m_bRangedCombat`. The new id is selected and its tile gets `0x100e` = 1. The context
/// line (`00755d90`) gets the string of the button's setting (two for the toggles, picked by
/// their flag; `""` for ids other than 0 to 7 and 11), the toggles' tile `0x100f` is set
/// again (button 2 from `m_bFollowing` itself, the others inverted) and the context text is
/// shown (`SetButtonContextText`).
pub fn companion_wheel_menu_set_button_as_highlighted(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    id: i32,
    force: u8,
) {
    let base = this.addr();
    if force == 0 && e.mem.u32(base + 0x70) == id as u32 {
        return;
    }
    play_sound(e, SCROLL_SOUND_TEXT);
    let old = e.mem.u32(base + 0x70);
    if old != 0xffff_ffff && tile_at(e, base, old) != 0 {
        let tile = tile_at(e, base, old);
        set_int(e, tile, 0x100e, 0);
    }
    let old = e.mem.u32(base + 0x70);
    let restore = match old {
        0 => Some((0x6du32, false)),
        2 => Some((0x6f, true)),
        5 => Some((0x6c, false)),
        7 => Some((0x6e, false)),
        _ => None,
    };
    if let Some((flag, inverse)) = restore {
        if tile_at(e, base, old) != 0 {
            let byte = e.mem.u8(base + flag);
            let value = if inverse {
                is_zero(byte)
            } else {
                u32::from(byte)
            };
            let tile = tile_at(e, base, old);
            set_int(e, tile, 0x100f, value);
        }
    }
    e.mem.set_u32(base + 0x70, id as u32);
    let selected = e.mem.u32(base + 0x70);
    if selected != 0xffff_ffff && tile_at(e, base, selected) != 0 {
        let tile = tile_at(e, base, selected);
        set_int(e, tile, 0x100e, 1);
    }
    // The toggles: (flag, setting when set, setting when clear, whether the tile gets the
    // flag inverted).
    let toggle = |flag: u32, set: u32, clear: u32, inverse: bool| Some((flag, set, clear, inverse));
    let plain = match id {
        0 => toggle(0x6d, SETTING_011D40CC, SETTING_011D4FA8, true),
        2 => toggle(0x6f, SETTING_011D2D90, SETTING_011D4540, false),
        5 => toggle(0x6c, SETTING_011D3BA4, SETTING_011D2598, true),
        7 => toggle(0x6e, SETTING_011D4660, SETTING_011D2C88, true),
        _ => None,
    };
    if let Some((flag, set, clear, inverse)) = plain {
        let setting = if e.mem.u8(base + flag) != 0 {
            set
        } else {
            clear
        };
        let text = setting_string(e, setting);
        fn_00755d90(e, this, text);
        let index = id as u32;
        if tile_at(e, base, index) != 0 {
            let byte = e.mem.u8(base + flag);
            let value = if inverse {
                is_zero(byte)
            } else {
                u32::from(byte)
            };
            let tile = tile_at(e, base, index);
            set_int(e, tile, 0x100f, value);
        }
    } else {
        let setting = match id {
            1 => Some(SETTING_011D2268),
            3 => Some(SETTING_011D4288),
            4 => Some(SETTING_011D288C),
            6 => Some(SETTING_011D34C8),
            11 => Some(SETTING_011D33FC),
            _ => None,
        };
        let text = match setting {
            Some(setting) => setting_string(e, setting),
            None => EMPTY_TEXT,
        };
        fn_00755d90(e, this, text);
    }
    let selected = e.mem.u32(base + 0x70);
    companion_wheel_menu_set_button_context_text(e, this, selected as i32);
}

// Translated from 00755c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::SetButtonAsHighlighted_ov2` (engine map; `RET 4`: a `float` angle).
/// The angle is first snapped in four bands (each from the `double` bounds at `01012060`/
/// `01071e78`, `01071e70`/`01071e20`, `01071e60`/`01071e58` and from `01071e48` up, to the
/// `float`s at `0104e2b4`, `01071e68`, `01071e50` and `01071e40`; the first band that holds
/// it wins). Then the first of the sixteen tiles that is a `RadialTile` (`__RTDynamicCast`
/// from `Tile`) whose traits `0x1006` and `0x1007` (stored as `float`s) hold the angle
/// (`low <= angle < high`, ordered) is highlighted (`SetButtonAsHighlighted(index, 0)`).
pub fn companion_wheel_menu_set_button_as_highlighted_ov2(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    angle: u32,
) {
    let mut angle = angle;
    let at = |e: &Engine, address: u32| -> f64 { e.global(address) };
    let value = x87_load32(angle);
    // Each band: FCOMP low; TEST AH,1; JNZ (below or unordered fails), then FCOMP high;
    // TEST AH,5; JP (only strictly below goes on).
    if value >= at(e, ZERO_DOUBLE) && value < at(e, BAND_1_HIGH) {
        angle = x87_quiet_bits(e.mem.u32(BAND_1_VALUE));
    } else if value >= at(e, BAND_2_LOW) && value < at(e, HALF_TURN_DOUBLE) {
        angle = x87_quiet_bits(e.mem.u32(BAND_2_VALUE));
    } else if value >= at(e, BAND_3_LOW) && value < at(e, BAND_3_HIGH) {
        angle = x87_quiet_bits(e.mem.u32(BAND_3_VALUE));
    } else if value >= at(e, BAND_4_LOW) {
        angle = x87_quiet_bits(e.mem.u32(BAND_4_VALUE));
    }
    let base = this.addr();
    for index in 0..16u32 {
        let tile = tile_at(e, base, index);
        let radial = e
            .call(
                RT_DYNAMIC_CAST,
                &args![tile, 0u32, TYPE_TILE, TYPE_RADIALTILE, 0u32],
            )
            .u32();
        if radial == 0 {
            continue;
        }
        let low = x87_store32(e.call(TILE_GET_FLOAT, &args![radial, 0x1006u32]).f64());
        let high = x87_store32(e.call(TILE_GET_FLOAT, &args![radial, 0x1007u32]).f64());
        let value = x87_load32(angle);
        // FLD angle; FLD low; FCOMPP; TEST AH,0x41; JP: low below or equal to the angle.
        if !(x87_load32(low) <= value) {
            continue;
        }
        // FLD angle; FLD high; FCOMPP; TEST AH,0x41; JNZ: high strictly above it.
        if !(x87_load32(high) > value) {
            continue;
        }
        companion_wheel_menu_set_button_as_highlighted(e, this, index as i32, 0);
        return;
    }
}

// Translated from 00755d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the context tile (`pTiles[9]`, +0x4c), when there is
/// one, gets `text` as trait `0xfc4`.
pub fn fn_00755d90(e: &mut Engine, this: Ptr<CompanionWheelMenu>, text: u32) {
    let tile = e.mem.u32(this.addr() + 0x4c);
    if tile != 0 {
        set_string(e, tile, 0xfc4, text);
    }
}

// Translated from 00755dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00755dc0 - the /GS stack cookie check (00ec408c) is not translated
/// `CompanionWheelMenu::SetButtonContextText` (engine map; `RET 4`: the button id). Writes a
/// line into a 0x104-byte stack buffer (empty by default) with the formatter `00406d00`, by
/// button, when there is a companion: 1 (stimpak) `"%s  %d/%d\n%s  %d"` with a setting's
/// string, the companion's value 0x10 (vtable +0xC of its +0xa4, truncated by `_ftol2`) and
/// its base value 0x10 (slot 0), a second setting's string and the count of the player's
/// stimpak entry (`00726070` of it, the entry then deleted with `004459e0(1)`; 0 without
/// one); 6 (health) `"%s  %d/%d"` with a setting's string, `008a0cd0(values, 0xd)` and the
/// companion's `008a0c20`, both truncated; 7 the full name (`ItemChange::GetFullName`) of
/// the companion's slot +0x3bc(6), when it has one. The context tile (`pTiles[10]`, +0x50)
/// then gets the buffer as trait `0xfc4`.
pub fn companion_wheel_menu_set_button_context_text(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    id: i32,
) {
    let base = this.addr();
    e.with_stack(0x10c, |e, buffer| {
        let buffer = buffer.addr();
        e.mem.set_u8(buffer, 0);
        let companion = e.mem.u32(base + 0x68);
        match id {
            1 if companion != 0 => {
                let companion = e.mem.u32(base + 0x68);
                let value = e
                    .vcall(companion.wrapping_add(0xa4), 0xc, &args![0x10u32])
                    .f64();
                let current = e.call(FTOL2, &args![value]).u32();
                let companion = e.mem.u32(base + 0x68);
                let maximum = e
                    .vcall(companion.wrapping_add(0xa4), 0, &args![0x10u32])
                    .u32();
                let mut count = 0u32;
                let entry = stimpak_entry(e);
                if entry != 0 {
                    count = e.call(MENU_TILE_00726070, &args![entry]).u32();
                    e.call(ENTRY_DELETE_004459E0, &args![entry, 1u32]);
                }
                let second = setting_string(e, SETTING_011D2568);
                let first = setting_string(e, SETTING_011D300C);
                e.call(
                    FORMAT_00406D00,
                    &args![
                        buffer,
                        0x104u32,
                        STIMPAK_FORMAT,
                        first,
                        current,
                        maximum,
                        second,
                        count
                    ],
                );
            }
            7 if companion != 0 => {
                let companion = e.mem.u32(base + 0x68);
                let item = e.vcall(companion, 0x3bc, &args![6u32]).u32();
                if item != 0 {
                    let name = e.call(ITEM_GET_FULL_NAME, &args![item]).u32();
                    e.call(FORMAT_00406D00, &args![buffer, 0x104u32, NAME_FORMAT, name]);
                }
            }
            6 if companion != 0 => {
                let companion = e.mem.u32(base + 0x68);
                let values = values_of(companion);
                let value = e.call(ACTOR_VALUE_008A0CD0, &args![values, 0xdu32]).f64();
                let current = e.call(FTOL2, &args![value]).u32();
                let companion = e.mem.u32(base + 0x68);
                let value = e.call(ACTOR_VALUE_008A0C20, &args![companion]).f64();
                let maximum = e.call(FTOL2, &args![value]).u32();
                let text = setting_string(e, SETTING_011D1E60);
                e.call(
                    FORMAT_00406D00,
                    &args![buffer, 0x104u32, HEALTH_FORMAT, text, current, maximum],
                );
            }
            _ => {}
        }
        let tile = e.mem.u32(base + 0x50);
        if tile != 0 {
            set_string(e, tile, 0xfc4, buffer);
        }
    });
}

/// The toggles' shared tail: when the command ran and the tile at `tile_offset` is set, it
/// gets trait `0x100f` = `value`.
fn toggle_tile(e: &mut Engine, base: u32, tile_offset: u32, value: u32) {
    if e.mem.u32(base + tile_offset) != 0 {
        let tile = e.mem.u32(base + tile_offset);
        set_int(e, tile, 0x100f, value);
    }
}

// Translated from 007560c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::Do_ToggleNearFar` (engine map; `thiscall`): flips
/// `m_bFollowingLong`, runs the follower command `"FollowersTacticsDistanceLong"` (now set)
/// or `"FollowersTacticsDistanceDefault"`, and sets tile 5's trait `0x100f` to the flag.
pub fn companion_wheel_menu_do_toggle_near_far(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    let base = this.addr();
    let flag = is_zero(e.mem.u8(base + 0x6c)) as u8;
    e.mem.set_u8(base + 0x6c, flag);
    let name = if e.mem.u8(base + 0x6c) != 0 {
        DISTANCE_LONG_TEXT
    } else {
        DISTANCE_DEFAULT_TEXT
    };
    follower_command(e, this, name);
    let value = u32::from(e.mem.u8(base + 0x6c));
    toggle_tile(e, base, 0x3c, value);
}

// Translated from 007561b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::Do_ToggleAggressivePassive` (engine map; `thiscall`): flips
/// `m_bAggressiveStance`, runs `"FollowersTacticsCombatAggressive"` (now set) or
/// `"FollowersTacticsCombatPassive"`, sets the open wheel's companion's
/// `"FollowerSwitchAggressive"` to 1.0 or 0.0 (`008c16c0`, a `double`) and tile 0's trait
/// `0x100f` to the flag.
pub fn companion_wheel_menu_do_toggle_aggressive_passive(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
) {
    let base = this.addr();
    let flag = is_zero(e.mem.u8(base + 0x6d)) as u8;
    e.mem.set_u8(base + 0x6d, flag);
    let name = if e.mem.u8(base + 0x6d) != 0 {
        COMBAT_AGGRESSIVE_TEXT
    } else {
        COMBAT_PASSIVE_TEXT
    };
    follower_command(e, this, name);
    // FLD1 or FLDZ, FSTP to a float, then FSTP double for the push.
    let value = if e.mem.u8(base + 0x6d) != 0 {
        1.0f64
    } else {
        0.0
    };
    let companion = e.mem.u32(open_wheel(e) + 0x68);
    e.call(
        ACTOR_SET_VARIABLE,
        &args![companion, SWITCH_AGGRESSIVE_TEXT, value],
    );
    let value = u32::from(e.mem.u8(base + 0x6d));
    toggle_tile(e, base, 0x28, value);
}

// Translated from 007562b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::Do_ToggleRangedMelee` (engine map; `thiscall`): with a companion,
/// flips `m_bRangedCombat`, runs `"FollowersTacticsCombatRanged"` (now set) or
/// `"FollowersTacticsCombatMelee"`, and sets tile 7's trait `0x100f` to the open wheel's
/// flag.
pub fn companion_wheel_menu_do_toggle_ranged_melee(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    let base = this.addr();
    if e.mem.u32(base + 0x68) == 0 {
        return;
    }
    let flag = is_zero(e.mem.u8(base + 0x6e)) as u8;
    e.mem.set_u8(base + 0x6e, flag);
    let name = if e.mem.u8(base + 0x6e) != 0 {
        COMBAT_RANGED_TEXT
    } else {
        COMBAT_MELEE_TEXT
    };
    follower_command(e, this, name);
    let value = u32::from(e.mem.u8(open_wheel(e) + 0x6e));
    toggle_tile(e, base, 0x44, value);
}

// Translated from 007563a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::Do_ToggleStayFollow` (engine map; `thiscall`): flips
/// `m_bFollowing`, runs `"FollowersLetsGo"` (now set) or `"FollowersWait"`, and sets tile
/// 2's trait `0x100f` to the inverse of the flag.
pub fn companion_wheel_menu_do_toggle_stay_follow(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    let base = this.addr();
    let flag = is_zero(e.mem.u8(base + 0x6f)) as u8;
    e.mem.set_u8(base + 0x6f, flag);
    let name = if e.mem.u8(base + 0x6f) == 0 {
        FOLLOWERS_WAIT_TEXT
    } else {
        FOLLOWERS_LETS_GO_TEXT
    };
    follower_command(e, this, name);
    let value = is_zero(e.mem.u8(base + 0x6f));
    toggle_tile(e, base, 0x30, value);
}

/// Whether a body part condition (`GetBodyPartCondition(values, part, 0)`, stored as a
/// `float`) is 0.0 (`FCOMP; TEST AH,0x44; JP`: ordered equal).
fn part_crippled(e: &mut Engine, this: Ptr<CompanionWheelMenu>, part: u32) -> bool {
    let values = values_of(e.mem.u32(this.addr() + 0x68));
    let condition = e
        .call(GET_BODY_PART_CONDITION, &args![values, part, 0u32])
        .f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    x87_load32(x87_store32(condition)) == zero
}

// Translated from 00756490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::Do_UseStim` (engine map; `thiscall`). With a companion and the
/// player, and the player's stimpak entry (see `stimpak_entry`): the companion's health
/// percentage and the conditions of body parts 0x1b, 0x1d, 0x1e, 0x1c, 0x19 and 0x1a (in
/// that order) are read, counting those at 0.0. When the health is below 1.0 or a part is
/// crippled: below 1.0 the companion drinks the stimpak (`Actor::DrinkPotion` with the
/// entry's form cast from `TESBoundObject` to `AlchemyItem`, its extra list and 1); the
/// player's slot +0x17c removes one (the entry's form, its extra list, 1, 0, 0, 0, 0, 0, 1,
/// 0), the companion's magic updates (`UpdateMagic(0, 1)`), values 0x1f, 0x1d, 0x1e, 0x1b,
/// 0x1c, 0x1a and 0x19 are restored by the `float` at `01013974` and the bark of type 0xd
/// plays (`PlayReactivityBark`). The entry is then deleted (`004459e0(1)`).
pub fn companion_wheel_menu_do_use_stim(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    let base = this.addr();
    if e.mem.u32(base + 0x68) == 0 || e.global::<u32>(PLAYER) == 0 {
        return;
    }
    let entry = stimpak_entry(e);
    if entry == 0 {
        return;
    }
    let companion = e.mem.u32(base + 0x68);
    let health = x87_store32(e.call(ACTOR_GET_HEALTH_PERCENTAGE, &args![companion]).f64());
    let mut crippled = 0i32;
    for part in [0x1bu32, 0x1d, 0x1e, 0x1c, 0x19, 0x1a] {
        if part_crippled(e, this, part) {
            crippled += 1;
        }
    }
    let one: f64 = e.global(ONE_DOUBLE);
    // FCOMP 1.0; TEST AH,5; JNP: strictly below goes in; otherwise a crippled part does.
    let low = x87_load32(health) < one;
    if low || crippled > 0 {
        if x87_load32(health) < one {
            let extra = entry_extra(e, entry);
            let form = e.call(WORD_AT_8_0044DDC0, &args![entry]).u32();
            let potion = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TESBOUNDOBJECT, TYPE_ALCHEMYITEM, 0u32],
                )
                .u32();
            let companion = e.mem.u32(base + 0x68);
            e.call(ACTOR_DRINK_POTION, &args![companion, potion, extra, 1u32]);
        }
        let extra = entry_extra(e, entry);
        let form = e.call(WORD_AT_8_0044DDC0, &args![entry]).u32();
        let player: u32 = e.global(PLAYER);
        e.vcall(
            player,
            0x17c,
            &args![form, extra, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
        let companion = e.mem.u32(base + 0x68);
        e.call(ACTOR_UPDATE_MAGIC, &args![companion, 0u32, 1u32]);
        for value in [0x1fu32, 0x1d, 0x1e, 0x1b, 0x1c, 0x1a, 0x19] {
            let amount = x87_quiet_bits(e.mem.u32(RESTORE_AMOUNT));
            let companion = e.mem.u32(base + 0x68);
            e.call(ACTOR_RESTORE_ACTOR_VALUE, &args![companion, value, amount]);
        }
        let topic = e.call(GET_BARK_TOPIC_BY_TYPE, &args![0xdu32]).u32();
        companion_wheel_menu_play_reactivity_bark(e, this, topic);
    }
    e.call(ENTRY_DELETE_004459E0, &args![entry, 1u32]);
}

// Translated from 007568d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the wheel's trade button): closes the wheel with
/// the fade, then with a companion runs the bark and command script of the topic
/// `"FollowersTrade"`.
pub fn fn_007568d0(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    companion_wheel_menu_close(e, 1);
    let topic = topic_by_editor_id(e, FOLLOWERS_TRADE_TEXT);
    if e.mem.u32(this.addr() + 0x68) != 0 {
        companion_wheel_menu_play_reactivity_bark(e, this, topic);
        companion_wheel_menu_run_follower_command_script(e, this, topic);
    }
}

// Translated from 00756930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the wheel's inventory button): the companion's
/// `008a7760` when there is one, then the wheel closes with the fade.
pub fn fn_00756930(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    let companion = e.mem.u32(this.addr() + 0x68);
    if companion != 0 {
        e.call(ACTOR_008A7760, &args![companion]);
    }
    companion_wheel_menu_close(e, 1);
}

// Translated from 00756960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the wheel's talk button): sets `bTalkToOnClose`
/// and closes the wheel with the fade.
pub fn fn_00756960(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    e.set(this, CompanionWheelMenu::bTalkToOnClose, 1);
    companion_wheel_menu_close(e, 1);
}

/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name, `008d8520`; `thiscall` on an
/// actor: its process, the same callee `DialogMenu::Create` reads the process with).
const GET_PROCESS: u32 = 0x008d_8520;
/// The talk-to path's actor calls (all `thiscall`): `00493bb0`, `00566950`, `00608d80`,
/// `0087f3d0`, `008ace90`, `008b3bb0` and `008b3c30` (byte answers), `008a8e50` (twice per
/// path: one per actor), `008a7a90`, `008bb520` (four words: a position by value and 0),
/// `MobileObject::GetCurrentPackage` (`009344a0`), the player's `004997b0` (a byte) and
/// `MagicCaster::InterruptCast` (`00815b00`, on the actor's +0x88).
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
const ACTOR_TEST_00566950: u32 = 0x0056_6950;
const ACTOR_TEST_00608D80: u32 = 0x0060_8d80;
const ACTOR_TEST_0087F3D0: u32 = 0x0087_f3d0;
const ACTOR_TEST_008ACE90: u32 = 0x008a_ce90;
const ACTOR_IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
const ACTOR_IS_ROTATING: u32 = 0x008b_3c30;
const ACTOR_STOP_ATTACK: u32 = 0x008a_8e50;
const ACTOR_008A7A90: u32 = 0x008a_7a90;
const ACTOR_FACE_008BB520: u32 = 0x008b_b520;
const MOBILE_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
const PLAYER_TEST_004997B0: u32 = 0x0049_97b0;
const MAGIC_CASTER_INTERRUPT_CAST: u32 = 0x0081_5b00;
/// `TESPackage::IsInterruptPackage` (`00678610`), the package's type (`0041ca90`) and its
/// `0041d8a0` (a word whose `00726070` is tested).
const PACKAGE_IS_INTERRUPT: u32 = 0x0067_8610;
const PACKAGE_TYPE_0041CA90: u32 = 0x0041_ca90;
const PACKAGE_WORD_0041D8A0: u32 = 0x0041_d8a0;
/// `VATS::QuitVATSPlayback` (`009c8950`, two words, on `011f2250`) and
/// `Interface::QueueMenuCreate` (`00709470`, `cdecl`, six words).
const VATS: u32 = 0x011f_2250;
const VATS_QUIT_PLAYBACK: u32 = 0x009c_8950;
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// `BSAudio::EndAllSoundsOfType` (`00ad8780`, one word) and `Interface::GetMenuModeType`
/// (`00702640`, no arguments).
const AUDIO_END_ALL_SOUNDS_OF_TYPE: u32 = 0x00ad_8780;
const GET_MENU_MODE_TYPE: u32 = 0x0070_2640;
/// `TESTopic::GetTopic` (`0061a2d0`, `cdecl`, two words), `TESTopic::CreateDialogueItem`
/// (`0061b320`, five words), the item's `0084e3a0` object and its `00619df0` (a byte),
/// `DialogueItem::FirstResponse` (`0083c7b0`) and `NextResponse` (`0083c7e0`), the speaker's
/// `0057b7c0` (three words) and the item's deleting destructor (`005c90d0`, one word).
const TOPIC_GET_TOPIC: u32 = 0x0061_a2d0;
const TOPIC_CREATE_DIALOGUE_ITEM: u32 = 0x0061_b320;
const OBJECT_TEST_00619DF0: u32 = 0x0061_9df0;
const DIALOGUE_ITEM_NEXT_RESPONSE: u32 = 0x0083_c7e0;
const SPEAKER_0057B7C0: u32 = 0x0057_b7c0;
const DIALOGUE_ITEM_DELETE: u32 = 0x005c_90d0;
/// The companion's slot +0x428 object's test (`0047c850`), the reference name
/// (`0055d520`), `sprintf` (`00ec623a`, `cdecl`), the message shower (`007052f0`, `cdecl`,
/// six words), the setting at `011cf594`, `"%s %s"` (`01012058`) and the `float` at
/// `010162c0`.
const OBJECT_TEST_0047C850: u32 = 0x0047_c850;
const REFERENCE_NAME_0055D520: u32 = 0x0055_d520;
const SPRINTF: u32 = 0x00ec_623a;
const SHOW_MESSAGE_007052F0: u32 = 0x0070_52f0;
const SETTING_011CF594: u32 = 0x011c_f594;
const TWO_STRINGS_FORMAT: u32 = 0x0101_2058;
const MESSAGE_SECONDS: u32 = 0x0101_62c0;
/// `TESPackage` and `DialoguePackage` type descriptors.
const TYPE_TESPACKAGE: u32 = 0x0118_46a0;
const TYPE_DIALOGUEPACKAGE: u32 = 0x0119_9c3c;

/// The dialogue package's word at +0x8c of `actor`'s current package (cast from
/// `TESPackage` to `DialoguePackage`), or `current` when it is not one.
fn dialogue_package_word(e: &mut Engine, actor: u32, current: u32) -> u32 {
    let package = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32();
    let dialogue = e
        .call(
            RT_DYNAMIC_CAST,
            &args![package, 0u32, TYPE_TESPACKAGE, TYPE_DIALOGUEPACKAGE, 0u32],
        )
        .u32();
    if dialogue != 0 {
        e.mem.u32(dialogue + 0x8c)
    } else {
        current
    }
}

/// The byte at +0xe of the actor's process's slot +0x4d4 object (asked twice, as the code
/// does) is above 0x13; false when the process has none.
fn process_object_above_13(e: &mut Engine, actor: u32) -> bool {
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if e.vcall(process, 0x4d4, &args![]).u32() == 0 {
        return false;
    }
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    let object = e.vcall(process, 0x4d4, &args![]).u32();
    e.mem.u8(object + 0xe) > 0x13
}

/// The package word's `0041d8a0` is set and its `00726070` is 0.
fn package_word_free(e: &mut Engine, package: u32) -> bool {
    e.call(PACKAGE_WORD_0041D8A0, &args![package]).u32() != 0 && {
        let word = e.call(PACKAGE_WORD_0041D8A0, &args![package]).u32();
        e.call(MENU_TILE_00726070, &args![word]).u32() == 0
    }
}

/// The dialogue menu queued for `target` (`VATS::QuitVATSPlayback(0, 0)`, then
/// `Interface::QueueMenuCreate(4, target, word, 0, 1, 0)`).
fn queue_dialogue(e: &mut Engine, target: u32, word: u32) {
    e.call(VATS_QUIT_PLAYBACK, &args![VATS, 0u32, 0u32]);
    e.call(
        QUEUE_MENU_CREATE,
        &args![4u32, target, word, 0u32, 1u32, 0u32],
    );
}

/// The ends of the talk-to path.
enum TalkTo {
    /// Straight to the end of the function.
    Done,
    /// The item (when made) is deleted first.
    DeleteItem(u32),
}

// Translated from 00756980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00756980 - the /GS stack cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`thiscall`; run by the destructor when `bTalkToOnClose` is
/// set): starts a conversation with the wheel's companion. `speaker` is the player as read
/// on entry (compared with the player global, read again, at each test).
///
/// Nothing happens without a companion, for one whose slot +0x2e8 answers true, without a
/// process (`008d8520`), when the process's slot +0x52c is set and the companion's
/// +0x22c(0) false, or when the process's +0x610 is set. The speaker's and the companion's
/// processes' slot +0x27c words are read. When the companion's `00493bb0` and `00566950`
/// answer true and the speaker is the player, the dialogue menu is queued for the companion
/// and nothing else runs. The talk itself runs when the companion is not an `00493bb0`
/// actor or its +0x230 answers true, and either the speaker is not the player, or the
/// companion's +0x214 is 0, 9 or 4, or its process's +0x4d4 object's byte +0xe is above
/// 0x13; otherwise (and without a companion) the fallback runs: for the player as speaker
/// and a companion whose +0x428 object passes `0047c850`, `"%s %s"` of the companion's name
/// and a setting's string is shown (`007052f0`).
///
/// The talk stops for a companion whose `00608d80` or +0x22c(0) answers true, or (speaker
/// the player) whose +0x230 and `0087f3d0` do; a speaker other than the player needs the
/// companion to be the player and its own package to be an interrupt package or of type 0
/// or 9; with the player as speaker, a companion whose +0x230 answers true, or the player's
/// `004997b0` true with the companion's `00566950` false, needs `008ace90`. A greeting
/// topic (`TESTopic::GetTopic(0, 0)`, for the player or a speaker whose process's +0x604 is
/// set) makes a dialogue item (`CreateDialogueItem`). An item whose object passes
/// `00619df0`, with a first response and no next one, is spoken: a companion with +0x214 9
/// gets its process's +0x600(1) and its +0x418, and the function ends; otherwise
/// `0057b7c0(topic, 0, 0)` runs on the speaker (for the player as companion) or the
/// companion. Else the conversation is set up: the process's flag 0x200 (+0x614) when its
/// +0x4d4 object's byte is above 0x13; a companion with +0x214 0 faces the speaker
/// (`008bb520`); then for the player as speaker the companion turns to it (process +0x294,
/// unless pathing or rotating), both stop attacking, the speaker's dialogue package word is
/// taken, the process's +0x288 runs with the speaker and 2 or the companion and 1 (when the
/// companion's +0x2c8 is the player and the companion's package word is of type 0), dialogue
/// sounds end (type 4), the companion's cast is interrupted and either the dialogue menu is
/// queued or (+0x214 9) its process's +0x600(1) and its +0x418 run. For another speaker
/// (unless the menu mode is `0x3f1` or the player's process's +0x3fc refuses the speaker)
/// the same is done for the speaker, with flag 0x400 (+0x614) or `008a7a90` for each. The
/// item is then deleted.
pub fn fn_00756980(e: &mut Engine, this: Ptr<CompanionWheelMenu>) {
    let speaker: u32 = e.global(PLAYER);
    let companion = e.mem.u32(this.addr() + 0x68);
    if companion == 0 || e.vcall(companion, 0x2e8, &args![]).bool() {
        return;
    }
    let process = e.call(GET_PROCESS, &args![companion]).u32();
    if process == 0 {
        return;
    }
    if e.vcall(process, 0x52c, &args![]).u32() != 0
        && !e.vcall(companion, 0x22c, &args![0u32]).bool()
    {
        return;
    }
    if e.vcall(process, 0x610, &args![]).u32() != 0 {
        return;
    }
    let speaker_process = e.call(GET_PROCESS, &args![speaker]).u32();
    let speaker_package = e.vcall(speaker_process, 0x27c, &args![]).u32();
    let companion_package = e.vcall(process, 0x27c, &args![]).u32();
    if speaker == 0 {
        return;
    }
    let is_player = |e: &Engine, actor: u32| actor == e.global::<u32>(PLAYER);
    if e.call(ACTOR_TEST_00493BB0, &args![companion]).bool()
        && is_player(e, speaker)
        && e.call(ACTOR_TEST_00566950, &args![companion]).bool()
    {
        queue_dialogue(e, companion, 0);
        return;
    }
    let talks = !e.call(ACTOR_TEST_00493BB0, &args![companion]).bool()
        || e.vcall(companion, 0x230, &args![]).bool();
    let talks = talks
        && (!is_player(e, speaker)
            || e.vcall(companion, 0x214, &args![]).u32() == 0
            || e.vcall(companion, 0x214, &args![]).u32() == 9
            || e.vcall(companion, 0x214, &args![]).u32() == 4
            || process_object_above_13(e, companion));
    if !talks {
        fallback_message(e, companion, speaker);
        return;
    }
    match talk_to(
        e,
        this,
        companion,
        process,
        speaker,
        speaker_package,
        companion_package,
    ) {
        TalkTo::Done => {}
        TalkTo::DeleteItem(item) => {
            if item != 0 {
                e.call(DIALOGUE_ITEM_DELETE, &args![item, 1u32]);
            }
        }
    }
}

/// `00756980`'s fallback (`0075732a`): see there.
fn fallback_message(e: &mut Engine, companion: u32, speaker: u32) {
    if companion == 0 || e.vcall(companion, 0x428, &args![]).u32() == 0 {
        return;
    }
    if speaker != e.global::<u32>(PLAYER) {
        return;
    }
    let object = e.vcall(companion, 0x428, &args![]).u32();
    if !e.call(OBJECT_TEST_0047C850, &args![object]).bool() {
        return;
    }
    let text = setting_string(e, SETTING_011CF594);
    let name = e.call(REFERENCE_NAME_0055D520, &args![companion]).u32();
    e.with_stack(0xc8, |e, buffer| {
        e.call(SPRINTF, &args![buffer, TWO_STRINGS_FORMAT, name, text]);
        let seconds = x87_quiet_bits(e.mem.u32(MESSAGE_SECONDS));
        e.call(
            SHOW_MESSAGE_007052F0,
            &args![buffer, 0u32, 0u32, 0u32, seconds, 0u32],
        );
    });
}

/// `00756980`'s talk (`00756bb4` on): see there.
fn talk_to(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    companion: u32,
    process: u32,
    speaker: u32,
    speaker_package: u32,
    companion_package: u32,
) -> TalkTo {
    let is_player = |e: &Engine, actor: u32| actor == e.global::<u32>(PLAYER);
    if e.call(ACTOR_TEST_00608D80, &args![companion]).bool() {
        return TalkTo::Done;
    }
    if e.vcall(companion, 0x22c, &args![0u32]).bool() {
        return TalkTo::Done;
    }
    if is_player(e, speaker)
        && e.vcall(companion, 0x230, &args![]).bool()
        && e.call(ACTOR_TEST_0087F3D0, &args![companion]).bool()
    {
        return TalkTo::Done;
    }
    if !is_player(e, speaker) {
        if !is_player(e, companion) {
            return TalkTo::Done;
        }
        let package = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![speaker]).u32();
        if !e.call(PACKAGE_IS_INTERRUPT, &args![package]).bool() {
            let package = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![speaker]).u32();
            if e.call(PACKAGE_TYPE_0041CA90, &args![package]).u32() != 0 {
                let package = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![speaker]).u32();
                if e.call(PACKAGE_TYPE_0041CA90, &args![package]).u32() != 9 {
                    return TalkTo::Done;
                }
            }
        }
    }
    if is_player(e, speaker) {
        let needs_check = e.vcall(companion, 0x230, &args![]).bool() || {
            let player: u32 = e.global(PLAYER);
            e.call(PLAYER_TEST_004997B0, &args![player]).bool()
                && !e.call(ACTOR_TEST_00566950, &args![companion]).bool()
        };
        if needs_check && !e.call(ACTOR_TEST_008ACE90, &args![companion]).bool() {
            return TalkTo::Done;
        }
    }
    let speaker_may_greet = |e: &mut Engine| {
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(process, 0x604, &args![]).bool()
    };
    let topic = if is_player(e, speaker) || speaker_may_greet(e) {
        e.call(TOPIC_GET_TOPIC, &args![0u32, 0u32]).u32()
    } else {
        0
    };
    let mut item = 0u32;
    if topic != 0 {
        if is_player(e, speaker) {
            let player: u32 = e.global(PLAYER);
            item = e
                .call(
                    TOPIC_CREATE_DIALOGUE_ITEM,
                    &args![topic, companion, player, 0u32, 0u32, 0u32],
                )
                .u32();
        } else if speaker_may_greet(e) {
            let player: u32 = e.global(PLAYER);
            item = e
                .call(
                    TOPIC_CREATE_DIALOGUE_ITEM,
                    &args![topic, speaker, player, 0u32, 0u32, 0u32],
                )
                .u32();
        }
    }
    if item != 0 {
        let object = e.call(VALUE_OF_0084E3A0, &args![item]).u32();
        let spoken = e.call(OBJECT_TEST_00619DF0, &args![object]).bool()
            && e.call(DIALOGUE_ITEM_FIRST_RESPONSE, &args![item]).bool()
            && !e.call(DIALOGUE_ITEM_NEXT_RESPONSE, &args![item]).bool();
        if spoken {
            if e.vcall(companion, 0x214, &args![]).u32() == 9 {
                let process = e.call(GET_PROCESS, &args![companion]).u32();
                e.vcall(process, 0x600, &args![1u32]);
                e.vcall(companion, 0x418, &args![]);
                return TalkTo::Done;
            }
            let target = if is_player(e, companion) {
                speaker
            } else {
                companion
            };
            e.call(SPEAKER_0057B7C0, &args![target, topic, 0u32, 0u32]);
            return TalkTo::DeleteItem(item);
        }
    }
    if process_object_above_13(e, companion) {
        let process = e.call(GET_PROCESS, &args![companion]).u32();
        e.vcall(process, 0x614, &args![0x200u32]);
    }
    if e.vcall(companion, 0x214, &args![]).u32() == 0 {
        let position = e.vcall(speaker, 0x1f4, &args![]).u32();
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.u32(position + 8);
        e.call(ACTOR_FACE_008BB520, &args![companion, x, y, z, 0u32]);
    }
    let mut word = 0u32;
    if is_player(e, speaker) {
        if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![companion]).bool()
            && !e.call(ACTOR_IS_ROTATING, &args![companion]).bool()
        {
            let process = e.call(GET_PROCESS, &args![companion]).u32();
            e.vcall(process, 0x294, &args![companion]);
        }
        e.call(ACTOR_STOP_ATTACK, &args![speaker]);
        e.call(ACTOR_STOP_ATTACK, &args![companion]);
        word = dialogue_package_word(e, speaker, word);
        // The code also keeps the wheel's companion in a local it never reads.
        let _ = e.mem.u32(this.addr() + 0x68);
        let target = e.vcall(companion, 0x2c8, &args![]).u32();
        if target == e.global::<u32>(PLAYER)
            && companion_package != 0
            && e.call(PACKAGE_TYPE_0041CA90, &args![companion_package])
                .u32()
                == 0
        {
            if package_word_free(e, companion_package) {
                e.vcall(process, 0x288, &args![speaker, 2u32]);
            } else {
                e.vcall(process, 0x288, &args![companion, 1u32]);
            }
        }
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(AUDIO_END_ALL_SOUNDS_OF_TYPE, &args![audio, 4u32]);
        e.call(MAGIC_CASTER_INTERRUPT_CAST, &args![companion + 0x88]);
        if e.vcall(companion, 0x214, &args![]).u32() != 9 {
            let target = if is_player(e, speaker) {
                companion
            } else {
                speaker
            };
            queue_dialogue(e, target, word);
        } else {
            let process = e.call(GET_PROCESS, &args![companion]).u32();
            e.vcall(process, 0x600, &args![1u32]);
            e.vcall(companion, 0x418, &args![]);
        }
        return TalkTo::DeleteItem(item);
    }
    if e.call(GET_MENU_MODE_TYPE, &args![]).u32() == 0x3f1 {
        return TalkTo::DeleteItem(item);
    }
    let player: u32 = e.global(PLAYER);
    let player_process = e.call(GET_PROCESS, &args![player]).u32();
    if !e.vcall(player_process, 0x3fc, &args![speaker]).bool() {
        return TalkTo::Done;
    }
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_END_ALL_SOUNDS_OF_TYPE, &args![audio, 4u32]);
    e.call(ACTOR_STOP_ATTACK, &args![speaker]);
    e.call(ACTOR_STOP_ATTACK, &args![companion]);
    if !e.call(ACTOR_TEST_00493BB0, &args![speaker]).bool() {
        word = dialogue_package_word(e, speaker, word);
    }
    let mode = if package_word_free(e, speaker_package) {
        2u32
    } else {
        1
    };
    let process = e.call(GET_PROCESS, &args![speaker]).u32();
    e.vcall(process, 0x288, &args![speaker, mode]);
    e.call(MAGIC_CASTER_INTERRUPT_CAST, &args![companion + 0x88]);
    for actor in [companion, speaker] {
        if is_player(e, actor) {
            e.call(ACTOR_008A7A90, &args![actor]);
        } else {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, 0x614, &args![0x400u32]);
        }
    }
    let target = if is_player(e, speaker) {
        companion
    } else {
        speaker
    };
    queue_dialogue(e, target, word);
    TalkTo::DeleteItem(item)
}

// Translated from 007573d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007573d0 - the C++ exception frame (FS:[0] chain, handler 00f0a568, unwind states 0 to 2 for the dialogue item, the sound handle and the by-value arguments) is not translated
/// `CompanionWheelMenu::PlayReactivityBark` (engine map; `RET 4`: a topic). The topic's
/// info for the companion and the player (`TESTopic::GetMatchingInfo`) makes a stack
/// `DialogueItem` (`0083c520` with the info's `006733e0`, the topic, the info and the
/// companion); its first response is taken. For a response: with a form (`007af430`) the
/// companion's speech handle for the form's `0084e3a0` (`00933150(local, value, 0, 0x102,
/// 1)`) is assigned to `m_hCurrentReactivityBark` (`00418900`) and the local destroyed;
/// without one the companion's slot +0x284 speaks it (thirteen words: the response's
/// `00460140` object's word, the bark handle, the response's `0044ddc0`, `0084e3a0`, text
/// length, `009611e0` and `00441110` words, the player, 1, 1, 0, 0, 1; its `float` answer is
/// dropped). Then the subtitle is shown (`DisplayCompanionWheelSubtitle` with a copy of the
/// response's text and of the bark handle). The dialogue item is destroyed.
pub fn companion_wheel_menu_play_reactivity_bark(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    topic: u32,
) {
    let base = this.addr();
    let player: u32 = e.global(PLAYER);
    let companion = e.mem.u32(base + 0x68);
    let info = if topic != 0 {
        e.call(TOPIC_GET_MATCHING_INFO, &args![topic, companion, player])
            .u32()
    } else {
        0
    };
    if info == 0 {
        return;
    }
    e.with_stack(0x1c, |e, item| {
        let word = e.call(INFO_WORD_006733E0, &args![info]).u32();
        e.call(
            DIALOGUE_ITEM_CONSTRUCT,
            &args![item, word, topic, info, companion],
        );
        e.call(DIALOGUE_ITEM_FIRST_RESPONSE, &args![item]);
        let response = e
            .call(DIALOGUE_ITEM_GET_CURRENT_RESPONSE, &args![item])
            .u32();
        if response != 0 {
            if e.call(RESPONSE_FORM_007AF430, &args![response]).u32() != 0 {
                let form = e.call(RESPONSE_FORM_007AF430, &args![response]).u32();
                let value = e.call(VALUE_OF_0084E3A0, &args![form]).u32();
                e.with_stack(0xc, |e, local| {
                    let handle = e
                        .call(
                            ACTOR_SPEECH_HANDLE_00933150,
                            &args![companion, local, value, 0u32, 0x102u32, 1u32],
                        )
                        .u32();
                    e.call(SOUND_HANDLE_ASSIGN, &args![base + 0x7c, handle]);
                    e.call(SOUND_HANDLE_DESTROY, &args![local]);
                });
            } else {
                let worldspace = e.call(RESPONSE_00441110, &args![response]).u32();
                let word = e.call(RESPONSE_009611E0, &args![response]).u32();
                let text = e.call(NODE_SELF_006815C0, &args![response]).u32();
                let length = e.call(STRING_LENGTH_004048E0, &args![text]).u32();
                let value = e.call(VALUE_OF_0084E3A0, &args![response]).u32();
                let form = e.call(WORD_AT_8_0044DDC0, &args![response]).u32();
                let object = e.call(RESPONSE_00460140, &args![response]).u32();
                let object = e.call(WORD_AT_0_00559450, &args![object]).u32();
                e.vcall(
                    companion,
                    0x284,
                    &args![
                        object,
                        base + 0x7c,
                        form,
                        value,
                        length,
                        word,
                        worldspace,
                        player,
                        1u32,
                        1u32,
                        0u32,
                        0u32,
                        1u32
                    ],
                );
            }
            // The by-value arguments: a `BSStringT` (+0) and a `BSSoundHandle` (+8), built in
            // the stack area the callee owns and destroys.
            e.with_stack(0x14, |e, block| {
                e.call(SOUND_HANDLE_ASSIGN, &args![block.addr() + 8, base + 0x7c]);
                let text = e.call(NODE_SELF_006815C0, &args![response]).u32();
                e.call(STRING_CONSTRUCT, &args![block, text]);
                display_subtitle_at(e, this, block.addr());
            });
        }
        e.call(DIALOGUE_ITEM_DESTROY, &args![item]);
    });
}

// Translated from 007575b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CompanionWheelMenu::RunFollowerCommandScript` (engine map; `RET 4`: a topic). For the
/// topic's info for the companion and the player (`TESTopic::GetMatchingInfo`), each of its
/// two result scripts (`GetResultScript(0)`, then `(1)`) that has compiled data (the word at
/// +8 of `00500940`) runs on the companion with its script locals (`Script::Run(companion,
/// locals, 0, 1)`).
pub fn companion_wheel_menu_run_follower_command_script(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    topic: u32,
) {
    let base = this.addr();
    let player: u32 = e.global(PLAYER);
    let companion = e.mem.u32(base + 0x68);
    let info = if topic != 0 {
        e.call(TOPIC_GET_MATCHING_INFO, &args![topic, companion, player])
            .u32()
    } else {
        0
    };
    if info == 0 {
        return;
    }
    for which in [0u32, 1] {
        let script = e.call(INFO_GET_RESULT_SCRIPT, &args![info, which]).u32();
        if script == 0 {
            continue;
        }
        let data = e.call(SCRIPT_DATA_00500940, &args![script]).u32();
        if e.mem.u32(data + 8) == 0 {
            continue;
        }
        let companion = e.mem.u32(base + 0x68);
        let extra = e.call(EXTRA_LIST_005D43C0, &args![companion]).u32();
        let locals = e.call(EXTRA_GET_SCRIPT_LOCALS, &args![extra]).u32();
        let companion = e.mem.u32(base + 0x68);
        e.call(SCRIPT_RUN, &args![script, companion, locals, 0u32, 1u32]);
    }
}

/// `DisplayCompanionWheelSubtitle` on its by-value arguments at `block` (the `BSStringT` at
/// +0, the `BSSoundHandle` at +8); see `companion_wheel_menu_display_companion_wheel_subtitle`.
fn display_subtitle_at(e: &mut Engine, this: Ptr<CompanionWheelMenu>, block: u32) {
    let base = this.addr();
    let text = block;
    let handle = block + 8;
    let mut time = 0.0f32.to_bits();
    let shown = e.call(STRING_LENGTH_004048E0, &args![text]).u32() != 0 && {
        let setting = e
            .call(STRING_CHARS_00408D60, &args![SUBTITLES_SETTING])
            .u32();
        e.mem.u8(setting) != 0 || {
            let companion = e.mem.u32(base + 0x68);
            companion != 0
                && e.call(ACTOR_ALWAYS_SHOW_SUBTITLES, &args![companion])
                    .bool()
        }
    };
    if shown {
        if e.mem.u32(base + 0x58) != 0 {
            let chars = e.call(WORD_AT_0_00559450, &args![text]).u32();
            let tile = e.mem.u32(base + 0x58);
            set_string(e, tile, 0xfc4, chars);
        }
        let duration = e.call(SOUND_HANDLE_GET_DURATION, &args![handle]).i32();
        time = x87_store32(f64::from(duration));
        let zero: f64 = e.global(ZERO_DOUBLE);
        if x87_load32(time) == zero {
            let chars = e.call(WORD_AT_0_00559450, &args![text]).u32();
            let length = e.call(STRLEN, &args![chars]).u32();
            // FILD qword of the length (high word 0), FSTP double.
            let length = f64::from(length);
            let per_character = e
                .call(SETTING_VALUE, &args![SUBTITLE_SECONDS_SETTING])
                .u32();
            let scale: f64 = e.global(SUBTITLE_SCALE_DOUBLE);
            let value = x87_mul(x87_load32(e.mem.u32(per_character)), length);
            time = x87_store32(x87_mul(value, scale));
        }
        let now = e.call(TIME_MS, &args![]).u32();
        let sum = x87_add(f64::from(now), x87_m32(time));
        let clear = x87_fistp64_chop(X87Ext::from(sum)) as u32;
        e.mem.set_u32(base + 0x78, clear);
    }
    let _ = time;
    e.call(STRING_DESTROY, &args![text]);
    e.call(SOUND_HANDLE_DESTROY, &args![handle]);
}

// Translated from 00757690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00757690 - the C++ exception frame (FS:[0] chain, handler 00f0a5a0, unwind states 1 and 0 that destroy the by-value arguments) is not translated
/// `CompanionWheelMenu::DisplayCompanionWheelSubtitle` (engine map; `RET 0x14`: a
/// `BSStringT` and a `BSSoundHandle` by value, which it destroys). With a non-empty text
/// and subtitles on (the setting's text at `011d8928` non-empty, or a companion that always
/// shows subtitles): tile 12 (+0x58) gets the text as trait `0xfc4`; the clear time
/// (`m_nTimeToClearSubtitle`) becomes the time now (`00457fe0`) plus the handle's duration
/// (`GetDuration`, as a `float`), or, for a duration of 0, the text's length times the
/// `float` setting at `011d2178` times the `double` at `01017b70`; truncated (64-bit
/// `FISTP`, low word). The string (`004037d0`) and the handle (`00483710`) are destroyed.
pub fn companion_wheel_menu_display_companion_wheel_subtitle(
    e: &mut Engine,
    this: Ptr<CompanionWheelMenu>,
    text_0: u32,
    text_1: u32,
    handle_0: u32,
    handle_1: u32,
    handle_2: u32,
) {
    e.with_stack(0x14, |e, block| {
        let block = block.addr();
        for (offset, word) in [
            (0u32, text_0),
            (4, text_1),
            (8, handle_0),
            (0xc, handle_1),
            (0x10, handle_2),
        ] {
            e.mem.set_u32(block + offset, word);
        }
        display_subtitle_at(e, this, block);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0075_4cf0,
            companion_wheel_menu_create_companion_wheel_menu(u32) -> bool
        ),
        entry!(
            0x0075_4bf0,
            companion_wheel_menu_get_class(Ptr<CompanionWheelMenu>) -> u32
        ),
        entry!(
            0x0075_4c30,
            companion_wheel_menu_scalar_deleting_destructor(
                Ptr<CompanionWheelMenu>,
                u32,
            ) -> Ptr<CompanionWheelMenu>
        ),
        entry!(0x0075_4c60, fn_00754c60(Ptr<CompanionWheelMenu>)),
        entry!(
            0x0075_4d90,
            companion_wheel_menu_get_should_companion_wheel_open(u32) -> bool
        ),
        entry!(0x0075_4de0, companion_wheel_menu_create(u32) -> u32),
        entry!(0x0075_5260, fn_00755260() -> u32),
        entry!(0x0075_5270, companion_wheel_menu_close(u8)),
        entry!(
            0x0075_52e0,
            companion_wheel_menu_do_click(Ptr<CompanionWheelMenu>, i32, u32)
        ),
        entry!(
            0x0075_5480,
            companion_wheel_menu_do_idle(Ptr<CompanionWheelMenu>)
        ),
        entry!(0x0075_5660, fn_00755660(f32) -> f64),
        entry!(0x0075_5680, fn_00755680(Ptr) -> bool),
        entry!(
            0x0075_56b0,
            companion_wheel_menu_do_gamepad(Ptr<CompanionWheelMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0075_5760,
            companion_wheel_menu_do_enter(Ptr<CompanionWheelMenu>, i32, u32)
        ),
        entry!(
            0x0075_57d0,
            companion_wheel_menu_set_button_as_highlighted(Ptr<CompanionWheelMenu>, i32, u8)
        ),
        entry!(
            0x0075_5c50,
            companion_wheel_menu_set_button_as_highlighted_ov2(Ptr<CompanionWheelMenu>, u32)
        ),
        entry!(0x0075_5d90, fn_00755d90(Ptr<CompanionWheelMenu>, u32)),
        entry!(
            0x0075_5dc0,
            companion_wheel_menu_set_button_context_text(Ptr<CompanionWheelMenu>, i32)
        ),
        entry!(
            0x0075_60c0,
            companion_wheel_menu_do_toggle_near_far(Ptr<CompanionWheelMenu>)
        ),
        entry!(
            0x0075_61b0,
            companion_wheel_menu_do_toggle_aggressive_passive(Ptr<CompanionWheelMenu>)
        ),
        entry!(
            0x0075_62b0,
            companion_wheel_menu_do_toggle_ranged_melee(Ptr<CompanionWheelMenu>)
        ),
        entry!(
            0x0075_63a0,
            companion_wheel_menu_do_toggle_stay_follow(Ptr<CompanionWheelMenu>)
        ),
        entry!(
            0x0075_6490,
            companion_wheel_menu_do_use_stim(Ptr<CompanionWheelMenu>)
        ),
        entry!(0x0075_68d0, fn_007568d0(Ptr<CompanionWheelMenu>)),
        entry!(0x0075_6930, fn_00756930(Ptr<CompanionWheelMenu>)),
        entry!(0x0075_6960, fn_00756960(Ptr<CompanionWheelMenu>)),
        entry!(0x0075_6980, fn_00756980(Ptr<CompanionWheelMenu>)),
        entry!(
            0x0075_73d0,
            companion_wheel_menu_play_reactivity_bark(Ptr<CompanionWheelMenu>, u32)
        ),
        entry!(
            0x0075_75b0,
            companion_wheel_menu_run_follower_command_script(Ptr<CompanionWheelMenu>, u32)
        ),
        entry!(
            0x0075_7690,
            companion_wheel_menu_display_companion_wheel_subtitle(
                Ptr<CompanionWheelMenu>,
                u32,
                u32,
                u32,
                u32,
                u32,
            )
        ),
    ]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn rig(tile: u32, cast: u32, created: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(GET_MENU_BY_CLASS, move |_, _| tile.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0x0c00_0000u32.into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, _| cast.into_ret());
        e.register_double(COMPANION_WHEEL_MENU_CREATE, move |_, _| created.into_ret());
        e.put_vtable(0x0090_1000, &[0x0090_0000]);
        e.register(0x0090_0000, |_, _| Ret::default());
        e
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

    #[test]
    fn an_open_wheel_is_deleted_before_the_new_one() {
        let mut e = rig(0x5000, 0, 0x6000);
        let menu = e.mem.alloc(4);
        e.mem.set_u32(menu, 0x0090_1000);
        e.register_double(RT_DYNAMIC_CAST, move |_, _| menu.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4cf0, &args![7u32]).bool());
        assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![0x433]]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![
                0x0c00_0000,
                0,
                TYPE_MENU,
                TYPE_COMPANION_WHEEL_MENU,
                0
            ]]
        );
        assert_eq!(calls(&e, 0x0090_0000), vec![vec![menu, 1]]);
        assert_eq!(calls(&e, COMPANION_WHEEL_MENU_CREATE), vec![vec![7]]);
    }

    #[test]
    fn without_an_open_wheel_only_the_create_runs() {
        let mut e = rig(0, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0075_4cf0, &args![7u32]).bool());
        assert!(calls(&e, TILE_GET_MENU).is_empty());
        assert!(calls(&e, 0x0090_0000).is_empty());
        // A cast that finds nothing deletes nothing.
        let mut e = rig(0x5000, 0, 0x6000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4cf0, &args![7u32]).bool());
        assert!(calls(&e, 0x0090_0000).is_empty());
    }
}

#[cfg(test)]
mod n087_tests {
    use super::*;

    /// The test vtable every object gets, and its slot functions (`SLOT + offset`).
    const VTABLE: u32 = 0x0e00_0000;
    const SLOT: u32 = 0x0e10_0000;

    const EXTERNAL: &[u32] = &[
        GET_MENU_BY_CLASS,
        TILE_GET_MENU,
        RT_DYNAMIC_CAST,
        FREE,
        INTERFACE_MANAGER,
        MANAGER_ROOT_00586150,
        INTERFACE_TEST_004B71D0,
        MANAGER_0041FD00,
        SOUND_HANDLE_DESTROY,
        MENU_DESTROY,
        TILE_READ_FILE,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        TILE_GET_MAXIMUM_DEPTH,
        MENU_TILE_00726070,
        MENU_CHECK_007B78A0,
        SET_TRAIT_FROM_INT,
        TILE_SET_STRING,
        TILE_SET_FLOAT,
        TILE_GET_FLOAT,
        DEBUG_MESSAGE,
        AUDIO_INSTANCE,
        AUDIO_GET_SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_FADE_OUT_AND_RELEASE,
        ACTOR_GET_VARIABLE,
        ACTOR_SET_VARIABLE,
        SETTING_STRING,
        SETTING_VALUE,
        IS_TOP_MENU_ID,
        PLAY_MENU_SOUND,
        ACTOR_CAN_SPEAK,
        ACTOR_GET_SHOULD_ATTACK_ACTOR,
        OBJECT_TEST_0042CE10,
        PROCESS_LISTS_UPDATE_3D_AFTER_RESTING,
        TIME_MS,
        ABS,
        STRLEN,
        MATH_005DC330,
        GET_FORM_BY_EDITOR_ID,
        GET_DEFAULT_OBJECT,
        GET_INVENTORY_ITEM,
        WORD_AT_8_0044DDC0,
        WORD_AT_0_00559450,
        NODE_SELF_006815C0,
        ENTRY_DELETE_004459E0,
        ACTOR_GET_HEALTH_PERCENTAGE,
        GET_BODY_PART_CONDITION,
        ACTOR_DRINK_POTION,
        ACTOR_UPDATE_MAGIC,
        ACTOR_RESTORE_ACTOR_VALUE,
        ACTOR_008A7760,
        ACTOR_VALUE_008A0CD0,
        ACTOR_VALUE_008A0C20,
        GET_BARK_TOPIC_BY_TYPE,
        ITEM_GET_FULL_NAME,
        FORMAT_00406D00,
        FTOL2,
        TOPIC_GET_MATCHING_INFO,
        INFO_GET_RESULT_SCRIPT,
        SCRIPT_DATA_00500940,
        EXTRA_LIST_005D43C0,
        EXTRA_GET_SCRIPT_LOCALS,
        SCRIPT_RUN,
        INFO_WORD_006733E0,
        DIALOGUE_ITEM_CONSTRUCT,
        DIALOGUE_ITEM_FIRST_RESPONSE,
        DIALOGUE_ITEM_GET_CURRENT_RESPONSE,
        DIALOGUE_ITEM_DESTROY,
        RESPONSE_FORM_007AF430,
        VALUE_OF_0084E3A0,
        RESPONSE_00441110,
        RESPONSE_009611E0,
        STRING_LENGTH_004048E0,
        RESPONSE_00460140,
        ACTOR_SPEECH_HANDLE_00933150,
        SOUND_HANDLE_ASSIGN,
        STRING_CONSTRUCT,
        STRING_DESTROY,
        STRING_CHARS_00408D60,
        ACTOR_ALWAYS_SHOW_SUBTITLES,
        SOUND_HANDLE_GET_DURATION,
        GET_PROCESS,
        ACTOR_TEST_00493BB0,
        ACTOR_TEST_00566950,
        ACTOR_TEST_00608D80,
        ACTOR_TEST_0087F3D0,
        ACTOR_TEST_008ACE90,
        ACTOR_IS_PATHING_COMPLETE,
        ACTOR_IS_ROTATING,
        ACTOR_STOP_ATTACK,
        ACTOR_008A7A90,
        ACTOR_FACE_008BB520,
        MOBILE_GET_CURRENT_PACKAGE,
        PLAYER_TEST_004997B0,
        MAGIC_CASTER_INTERRUPT_CAST,
        PACKAGE_IS_INTERRUPT,
        PACKAGE_TYPE_0041CA90,
        PACKAGE_WORD_0041D8A0,
        VATS_QUIT_PLAYBACK,
        QUEUE_MENU_CREATE,
        AUDIO_END_ALL_SOUNDS_OF_TYPE,
        GET_MENU_MODE_TYPE,
        TOPIC_GET_TOPIC,
        TOPIC_CREATE_DIALOGUE_ITEM,
        OBJECT_TEST_00619DF0,
        DIALOGUE_ITEM_NEXT_RESPONSE,
        SPEAKER_0057B7C0,
        DIALOGUE_ITEM_DELETE,
        OBJECT_TEST_0047C850,
        REFERENCE_NAME_0055D520,
        SPRINTF,
        SHOW_MESSAGE_007052F0,
    ];

    struct Rig {
        e: Engine,
        menu: u32,
        companion: u32,
        player: u32,
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        object
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x8_0000);
        e.map(0x011d_0000, 0x2_0000);
        for &callee in EXTERNAL {
            e.register(callee, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..0x200).map(|i| SLOT + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(VALUE_6006_DOUBLE, 6006.0f64);
        e.set_global(VALUE_102_DOUBLE, 102.0f64);
        let menu = object(&mut e, 0x88);
        let companion = object(&mut e, 0x100);
        let player = object(&mut e, 0x100);
        e.mem.set_u32(menu + 0x68, companion);
        e.mem.set_u32(menu + 0x70, 0xffff_ffff);
        e.set_global(OPEN_WHEEL, menu);
        e.set_global(PLAYER, player);
        e.call_log = Some(vec![]);
        Rig {
            e,
            menu,
            companion,
            player,
        }
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn answer_f64(e: &mut Engine, address: u32, value: f64) {
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

    fn tiles(r: &mut Rig) {
        for index in 0..16u32 {
            r.e.mem
                .set_u32(r.menu + 0x28 + 4 * index, 0x0d00_0000 + index);
        }
    }

    #[test]
    fn get_class_answers_0x433() {
        let mut r = rig();
        assert_eq!(r.e.call(0x0075_4bf0, &args![r.menu]).u32(), 0x433);
        assert_eq!(r.e.call(0x0075_5260, &args![]).u32(), 0x433);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut r = rig();
        let menu = r.menu;
        assert_eq!(r.e.call(0x0075_4c30, &args![menu, 1u32]).u32(), menu);
        assert_eq!(calls(&r.e, FREE), vec![vec![menu]]);
        assert_eq!(calls(&r.e, MENU_DESTROY), vec![vec![menu]]);
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_4c30, &args![menu, 2u32]);
        assert!(calls(&r.e, FREE).is_empty());
    }

    #[test]
    fn destructor_talks_when_asked() {
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_4c60, &args![menu]);
        assert_eq!(r.e.mem.u32(menu), COMPANION_WHEEL_MENU_VTABLE);
        assert_eq!(r.e.global::<u32>(OPEN_WHEEL), 0);
        assert_eq!(calls(&r.e, SOUND_HANDLE_DESTROY), vec![vec![menu + 0x7c]]);
        assert!(calls(&r.e, MANAGER_0041FD00).is_empty());
        let mut r = rig();
        let menu = r.menu;
        answer(&mut r.e, INTERFACE_MANAGER, 0x0c00_0000);
        r.e.mem.set_u8(menu + 0x74, 1);
        r.e.call(0x0075_4c60, &args![menu]);
        assert_eq!(calls(&r.e, MANAGER_0041FD00), vec![vec![0x0c00_0000, 1]]);
        // The talk-to asked the companion (slot +0x2e8) first.
        assert_eq!(calls(&r.e, SLOT + 0x2e8), vec![vec![r.companion]]);
    }

    #[test]
    fn should_open_needs_a_speaking_friendly_actor() {
        let mut r = rig();
        assert!(!r.e.call(0x0075_4d90, &args![0u32]).bool());
        let actor = r.companion;
        assert!(!r.e.call(0x0075_4d90, &args![actor]).bool());
        answer(&mut r.e, ACTOR_CAN_SPEAK, 1);
        assert!(r.e.call(0x0075_4d90, &args![actor]).bool());
        let words = calls(&r.e, ACTOR_GET_SHOULD_ATTACK_ACTOR);
        assert_eq!(words[0][..3], [actor, r.player, 0]);
        assert_eq!(words[0][4], 0);
        answer(&mut r.e, ACTOR_GET_SHOULD_ATTACK_ACTOR, 1);
        assert!(!r.e.call(0x0075_4d90, &args![actor]).bool());
    }

    #[test]
    fn create_reads_the_flags_and_shows_the_wheel() {
        let mut r = rig();
        tiles(&mut r);
        let menu = r.menu;
        let companion = r.companion;
        r.e.set_global(OPEN_WHEEL, 0u32);
        answer(&mut r.e, TILE_READ_FILE, 0x0c40_0000);
        answer(&mut r.e, TILE_GET_MENU, menu);
        answer(&mut r.e, SLOT + 0x34, 0x433);
        answer(&mut r.e, MENU_CHECK_007B78A0, 1);
        r.e.register_double(ACTOR_GET_VARIABLE, |_, a| {
            let value = if a[1] == WAITING_TEXT { 0.0 } else { 1.0 };
            value.into_ret()
        });
        answer(&mut r.e, SETTING_STRING, 0x0c50_0000);
        answer_f64(&mut r.e, TILE_GET_FLOAT, 102.0);
        answer_f64(&mut r.e, TILE_GET_MAXIMUM_DEPTH, 3.0);
        assert_eq!(r.e.call(0x0075_4de0, &args![companion]).u32(), 0x0c40_0000);
        assert_eq!(r.e.mem.u32(menu + 0x68), companion);
        assert_eq!(
            [0x6c, 0x6d, 0x6e, 0x6f].map(|o| r.e.mem.u8(menu + o)),
            [1, 1, 1, 1]
        );
        assert_eq!(
            calls(&r.e, SET_TRAIT_FROM_INT),
            vec![
                vec![0x0d00_0000, 0x100f, 1],
                vec![0x0d00_0002, 0x100f, 0],
                vec![0x0d00_0005, 0x100f, 1],
                vec![0x0d00_0007, 0x100f, 1],
            ]
        );
        let strings = calls(&r.e, TILE_SET_STRING);
        assert_eq!(strings[0], vec![0x0d00_000c, 0xfc4, EMPTY_TEXT, 1]);
        assert_eq!(strings.len(), 5);
        assert_eq!(
            calls(&r.e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2..],
            [POP_UP_SOUND_TEXT, 0x121]
        );
        assert_eq!(
            calls(&r.e, TILE_SET_FLOAT),
            vec![vec![0x0c40_0000, 0xfad, 3.0f32.to_bits(), 1]]
        );
        assert_eq!(calls(&r.e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
    }

    #[test]
    fn create_fails_on_a_wrong_menu_or_missing_tiles() {
        let mut r = rig();
        let menu = r.menu;
        answer(&mut r.e, TILE_GET_MENU, menu);
        answer(&mut r.e, SLOT + 0x34, 0x400);
        answer(&mut r.e, MENU_TILE_00726070, 0x0c60_0000);
        assert_eq!(r.e.call(0x0075_4de0, &args![0u32]).u32(), 0);
        assert_eq!(calls(&r.e, SLOT), vec![vec![menu, 1]]);
        let mut r = rig();
        let menu = r.menu;
        answer(&mut r.e, TILE_GET_MENU, menu);
        answer(&mut r.e, SLOT + 0x34, 0x433);
        assert_eq!(r.e.call(0x0075_4de0, &args![0u32]).u32(), 0);
        assert_eq!(calls(&r.e, DEBUG_MESSAGE), vec![vec![CREATION_FAILED_TEXT]]);
        assert!(calls(&r.e, ACTOR_GET_VARIABLE).is_empty());
    }

    #[test]
    fn close_fades_the_bark_and_the_menu() {
        let mut r = rig();
        let menu = r.menu;
        answer(&mut r.e, MENU_TILE_00726070, 0x0c60_0000);
        r.e.call(0x0075_5270, &args![1u32]);
        assert_eq!(
            calls(&r.e, SOUND_HANDLE_FADE_OUT_AND_RELEASE),
            vec![vec![menu + 0x7c, 100]]
        );
        assert_eq!(
            calls(&r.e, PROCESS_LISTS_UPDATE_3D_AFTER_RESTING),
            vec![vec![PROCESS_LISTS]]
        );
        assert_eq!(
            calls(&r.e, SET_TRAIT_FROM_INT),
            vec![vec![0x0c60_0000, 0x1772, 1]]
        );
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT), vec![vec![menu]]);
        let mut r = rig();
        answer(&mut r.e, OBJECT_TEST_0042CE10, 1);
        r.e.call(0x0075_5270, &args![0u32]);
        assert!(calls(&r.e, SOUND_HANDLE_FADE_OUT_AND_RELEASE).is_empty());
        assert!(calls(&r.e, PROCESS_LISTS_UPDATE_3D_AFTER_RESTING).is_empty());
    }

    #[test]
    fn do_click_dispatches_by_button() {
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_52e0, &args![menu, 3u32, 0u32]);
        assert!(calls(&r.e, PLAY_MENU_SOUND).is_empty());
        answer(&mut r.e, IS_TOP_MENU_ID, 1);
        r.e.call(0x0075_52e0, &args![menu, 3u32, 0u32]);
        assert_eq!(r.e.mem.u8(menu + 0x74), 1);
        assert_eq!(calls(&r.e, PLAY_MENU_SOUND), vec![vec![1]]);
        r.e.call(0x0075_52e0, &args![menu, 9u32, 0u32]);
        assert_eq!(calls(&r.e, PLAY_MENU_SOUND).len(), 1);
        r.e.call(0x0075_52e0, &args![menu, 11u32, 0u32]);
        assert_eq!(calls(&r.e, PLAY_MENU_SOUND).len(), 2);
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT).len(), 2);
        // Button 5 toggles and highlights (the scroll sound plays once).
        r.e.call(0x0075_52e0, &args![menu, 5u32, 0u32]);
        assert_eq!(r.e.mem.u8(menu + 0x6c), 1);
        assert_eq!(r.e.mem.u32(menu + 0x70), 5);
        assert_eq!(
            calls(&r.e, GET_FORM_BY_EDITOR_ID),
            vec![vec![DISTANCE_LONG_TEXT]]
        );
    }

    #[test]
    fn do_idle_clears_the_subtitle_and_turns_the_stick_into_an_angle() {
        let mut r = rig();
        let menu = r.menu;
        tiles(&mut r);
        r.e.mem.set_u32(menu + 0x78, 50);
        answer(&mut r.e, TIME_MS, 60);
        answer(&mut r.e, IS_TOP_MENU_ID, 1);
        answer(&mut r.e, INTERFACE_TEST_004B71D0, 1);
        answer(&mut r.e, INTERFACE_MANAGER, 0x0c00_0000);
        r.e.map(0x0c00_0000, 0x100);
        r.e.register(ABS, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        r.e.mem.set_u16(STICK_X, 1000);
        r.e.mem.set_u16(STICK_Y, (-20000i16) as u16);
        r.e.register(MATH_005DC330, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        r.e.call(0x0075_5480, &args![menu]);
        assert_eq!(r.e.mem.u32(menu + 0x78), 0);
        assert_eq!(
            calls(&r.e, TILE_SET_STRING),
            vec![vec![0x0d00_000c, 0xfc4, EMPTY_TEXT, 1]]
        );
        // y = 20000 after the sign flip: the angle is atan-input -1000 / 20000, then the
        // half turn (0 here) is added; each radial tile is cast.
        let quotient = (-1000.0f32 / 20000.0f32).to_bits();
        assert_eq!(calls(&r.e, MATH_005DC330), vec![vec![quotient]]);
        assert_eq!(calls(&r.e, RT_DYNAMIC_CAST).len(), 16);
    }

    #[test]
    fn fn_00755660_passes_the_float_on() {
        let mut r = rig();
        answer_f64(&mut r.e, MATH_005DC330, 0.5);
        let answer = r.e.call(0x0075_5660, &args![0.25f32.to_bits()]).f64();
        assert_eq!(answer, 0.5);
        assert_eq!(calls(&r.e, MATH_005DC330), vec![vec![0.25f32.to_bits()]]);
    }

    #[test]
    fn fn_00755680_reads_the_managers_byte() {
        let mut r = rig();
        let manager = r.e.mem.alloc(0x80);
        assert!(r.e.call(0x0075_5680, &args![manager]).bool());
        answer(&mut r.e, INTERFACE_TEST_004B71D0, 1);
        assert!(!r.e.call(0x0075_5680, &args![manager]).bool());
        r.e.mem.set_u8(manager + 0x7d, 1);
        assert!(r.e.call(0x0075_5680, &args![manager]).bool());
    }

    #[test]
    fn do_gamepad_clicks_the_selection_or_close() {
        let mut r = rig();
        let menu = r.menu;
        tiles(&mut r);
        assert!(!r.e.call(0x0075_56b0, &args![menu, 9u32, 0u32]).bool());
        answer(&mut r.e, IS_TOP_MENU_ID, 1);
        assert!(r.e.call(0x0075_56b0, &args![menu, 9u32, 0u32]).bool());
        assert_eq!(calls(&r.e, SLOT + 0xc), vec![vec![menu, 0xb, 0x0d00_000b]]);
        r.e.mem.set_u32(menu + 0x70, 2);
        assert!(r.e.call(0x0075_56b0, &args![menu, 9u32, 0u32]).bool());
        assert_eq!(calls(&r.e, SLOT + 0xc)[1], vec![menu, 2, 0x0d00_0002]);
        assert!(r.e.call(0x0075_56b0, &args![menu, 10u32, 0u32]).bool());
        assert_eq!(calls(&r.e, SLOT + 0xc)[2], vec![menu, 0xb, 0x0d00_000b]);
        assert!(!r.e.call(0x0075_56b0, &args![menu, 8u32, 0u32]).bool());
    }

    #[test]
    fn do_enter_highlights_buttons_0_to_7_and_11() {
        let mut r = rig();
        let menu = r.menu;
        answer(&mut r.e, IS_TOP_MENU_ID, 1);
        r.e.call(0x0075_5760, &args![menu, 8u32, 0u32]);
        assert_eq!(r.e.mem.u32(menu + 0x70), 0xffff_ffff);
        r.e.call(0x0075_5760, &args![menu, 11u32, 0u32]);
        assert_eq!(r.e.mem.u32(menu + 0x70), 11);
        r.e.call(0x0075_5760, &args![menu, 0xffff_ffffu32, 0u32]);
        assert_eq!(r.e.mem.u32(menu + 0x70), 11);
    }

    #[test]
    fn set_button_as_highlighted_moves_the_selection() {
        let mut r = rig();
        let menu = r.menu;
        tiles(&mut r);
        r.e.mem.set_u32(menu + 0x70, 2);
        r.e.mem.set_u8(menu + 0x6d, 1);
        r.e.register(SETTING_STRING, |_, a| a[0].into_ret());
        r.e.call(0x0075_57d0, &args![menu, 0u32, 0u32]);
        assert_eq!(r.e.mem.u32(menu + 0x70), 0);
        assert_eq!(
            calls(&r.e, SET_TRAIT_FROM_INT),
            vec![
                vec![0x0d00_0002, 0x100e, 0],
                vec![0x0d00_0002, 0x100f, 1],
                vec![0x0d00_0000, 0x100e, 1],
                vec![0x0d00_0000, 0x100f, 0],
            ]
        );
        assert_eq!(
            calls(&r.e, TILE_SET_STRING)[0],
            vec![0x0d00_0009, 0xfc4, SETTING_011D40CC, 1]
        );
        // Not forced and already selected: nothing.
        let before = r.e.call_log.as_ref().unwrap().len();
        r.e.call(0x0075_57d0, &args![menu, 0u32, 0u32]);
        assert_eq!(r.e.call_log.as_ref().unwrap().len(), before + 1);
        // An id without a setting shows "".
        r.e.call(0x0075_57d0, &args![menu, 9u32, 1u32]);
        let context: Vec<Vec<u32>> = calls(&r.e, TILE_SET_STRING)
            .into_iter()
            .filter(|w| w[0] == 0x0d00_0009)
            .collect();
        assert_eq!(
            context.last().unwrap(),
            &vec![0x0d00_0009, 0xfc4, EMPTY_TEXT, 1]
        );
    }

    #[test]
    fn highlight_by_angle_snaps_and_finds_the_radial_tile() {
        let mut r = rig();
        let menu = r.menu;
        tiles(&mut r);
        for (address, value) in [
            (BAND_1_HIGH, 0.5f64),
            (BAND_2_LOW, 2.5),
            (HALF_TURN_DOUBLE, 3.0),
            (BAND_3_LOW, 3.0),
            (BAND_3_HIGH, 3.5),
            (BAND_4_LOW, 5.5),
        ] {
            r.e.set_global(address, value);
        }
        r.e.set_global(BAND_1_VALUE, 0.25f32);
        r.e.register(RT_DYNAMIC_CAST, |_, a| {
            (if a[0] == 0x0d00_0003 { a[0] } else { 0 }).into_ret()
        });
        r.e.register(TILE_GET_FLOAT, |_, a| {
            (if a[1] == 0x1006 { 0.25f64 } else { 0.5 }).into_ret()
        });
        r.e.call(0x0075_5c50, &args![menu, 0.1f32.to_bits()]);
        assert_eq!(r.e.mem.u32(menu + 0x70), 3);
        // An angle outside the radial tile's range selects nothing.
        r.e.mem.set_u32(menu + 0x70, 0xffff_ffff);
        r.e.call(0x0075_5c50, &args![menu, 2.0f32.to_bits()]);
        assert_eq!(r.e.mem.u32(menu + 0x70), 0xffff_ffff);
    }

    #[test]
    fn fn_00755d90_sets_the_context_tile() {
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_5d90, &args![menu, 0x1234u32]);
        assert!(calls(&r.e, TILE_SET_STRING).is_empty());
        r.e.mem.set_u32(menu + 0x4c, 0x0d00_0009);
        r.e.call(0x0075_5d90, &args![menu, 0x1234u32]);
        assert_eq!(
            calls(&r.e, TILE_SET_STRING),
            vec![vec![0x0d00_0009, 0xfc4, 0x1234, 1]]
        );
    }

    #[test]
    fn context_text_for_health_formats_the_values() {
        let mut r = rig();
        let menu = r.menu;
        let companion = r.companion;
        r.e.mem.set_u32(menu + 0x50, 0x0d00_000a);
        r.e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as u32).into_ret()
        });
        answer_f64(&mut r.e, ACTOR_VALUE_008A0CD0, 40.0);
        answer_f64(&mut r.e, ACTOR_VALUE_008A0C20, 90.0);
        answer(&mut r.e, SETTING_STRING, 0x0c50_0000);
        r.e.call(0x0075_5dc0, &args![menu, 6u32]);
        assert_eq!(
            calls(&r.e, ACTOR_VALUE_008A0CD0),
            vec![vec![companion + 0xa4, 0xd]]
        );
        let format = calls(&r.e, FORMAT_00406D00);
        assert_eq!(format[0][1..], [0x104, HEALTH_FORMAT, 0x0c50_0000, 40, 90]);
        assert_eq!(calls(&r.e, TILE_SET_STRING)[0][..2], [0x0d00_000a, 0xfc4]);
        // Without a companion only the empty line is shown.
        let mut r = rig();
        let menu = r.menu;
        r.e.mem.set_u32(menu + 0x68, 0);
        r.e.mem.set_u32(menu + 0x50, 0x0d00_000a);
        r.e.call(0x0075_5dc0, &args![menu, 1u32]);
        assert!(calls(&r.e, FORMAT_00406D00).is_empty());
        assert_eq!(calls(&r.e, TILE_SET_STRING).len(), 1);
    }

    fn toggle(address: u32, flag: u32, set: u32, clear: u32, tile: u32, inverse: bool) {
        for start in [0u8, 1] {
            let mut r = rig();
            let menu = r.menu;
            tiles(&mut r);
            r.e.mem.set_u8(menu + flag, start);
            r.e.call(address, &args![menu]);
            let now = r.e.mem.u8(menu + flag);
            assert_eq!(now, 1 - start);
            let name = if now != 0 { set } else { clear };
            assert_eq!(calls(&r.e, GET_FORM_BY_EDITOR_ID), vec![vec![name]]);
            let value = if inverse {
                u32::from(now == 0)
            } else {
                u32::from(now)
            };
            assert_eq!(
                calls(&r.e, SET_TRAIT_FROM_INT),
                vec![vec![0x0d00_0000 + tile, 0x100f, value]]
            );
        }
    }

    #[test]
    fn toggle_near_far() {
        toggle(
            0x0075_60c0,
            0x6c,
            DISTANCE_LONG_TEXT,
            DISTANCE_DEFAULT_TEXT,
            5,
            false,
        );
    }

    #[test]
    fn toggle_aggressive_passive_sets_the_variable() {
        toggle(
            0x0075_61b0,
            0x6d,
            COMBAT_AGGRESSIVE_TEXT,
            COMBAT_PASSIVE_TEXT,
            0,
            false,
        );
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_61b0, &args![menu]);
        let words = calls(&r.e, ACTOR_SET_VARIABLE);
        assert_eq!(words[0][..2], [r.companion, SWITCH_AGGRESSIVE_TEXT]);
        assert_eq!(words[0][2..], args![1.0f64][..]);
    }

    #[test]
    fn toggle_ranged_melee_needs_a_companion() {
        toggle(
            0x0075_62b0,
            0x6e,
            COMBAT_RANGED_TEXT,
            COMBAT_MELEE_TEXT,
            7,
            false,
        );
        let mut r = rig();
        let menu = r.menu;
        r.e.mem.set_u32(menu + 0x68, 0);
        r.e.call(0x0075_62b0, &args![menu]);
        assert_eq!(r.e.mem.u8(menu + 0x6e), 0);
        assert!(calls(&r.e, GET_FORM_BY_EDITOR_ID).is_empty());
    }

    #[test]
    fn toggle_stay_follow() {
        toggle(
            0x0075_63a0,
            0x6f,
            FOLLOWERS_LETS_GO_TEXT,
            FOLLOWERS_WAIT_TEXT,
            2,
            true,
        );
    }

    #[test]
    fn use_stim_heals_a_hurt_companion() {
        let mut r = rig();
        let menu = r.menu;
        let companion = r.companion;
        let player = r.player;
        let entry = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(entry + 8, 0x0c70_0000);
        r.e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        answer(&mut r.e, GET_DEFAULT_OBJECT, 0x0c71_0000);
        answer(&mut r.e, GET_INVENTORY_ITEM, entry);
        answer(&mut r.e, WORD_AT_8_0044DDC0, 0x0c70_0000);
        answer_f64(&mut r.e, ACTOR_GET_HEALTH_PERCENTAGE, 0.5);
        answer_f64(&mut r.e, GET_BODY_PART_CONDITION, 1.0);
        r.e.set_global(RESTORE_AMOUNT, 1000.0f32);
        r.e.call(0x0075_6490, &args![menu]);
        assert_eq!(
            calls(&r.e, ACTOR_DRINK_POTION),
            vec![vec![companion, 0x0c70_0000, 0, 1]]
        );
        let removed = calls(&r.e, SLOT + 0x17c);
        assert_eq!(
            removed,
            vec![vec![player, 0x0c70_0000, 0, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        let restored: Vec<u32> = calls(&r.e, ACTOR_RESTORE_ACTOR_VALUE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(restored, vec![0x1f, 0x1d, 0x1e, 0x1b, 0x1c, 0x1a, 0x19]);
        assert_eq!(calls(&r.e, GET_BARK_TOPIC_BY_TYPE), vec![vec![0xd]]);
        assert_eq!(calls(&r.e, ENTRY_DELETE_004459E0), vec![vec![entry, 1]]);
    }

    #[test]
    fn use_stim_on_a_healthy_companion_only_deletes_the_entry() {
        let mut r = rig();
        let menu = r.menu;
        let entry = r.e.mem.alloc(0x10);
        r.e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        answer(&mut r.e, GET_DEFAULT_OBJECT, 0x0c71_0000);
        answer(&mut r.e, GET_INVENTORY_ITEM, entry);
        answer_f64(&mut r.e, ACTOR_GET_HEALTH_PERCENTAGE, 1.0);
        answer_f64(&mut r.e, GET_BODY_PART_CONDITION, 1.0);
        r.e.call(0x0075_6490, &args![menu]);
        assert!(calls(&r.e, ACTOR_DRINK_POTION).is_empty());
        assert!(calls(&r.e, SLOT + 0x17c).is_empty());
        assert_eq!(calls(&r.e, ENTRY_DELETE_004459E0), vec![vec![entry, 1]]);
        // A crippled part (condition 0.0) heals without the potion.
        answer_f64(&mut r.e, GET_BODY_PART_CONDITION, 0.0);
        r.e.call(0x0075_6490, &args![menu]);
        assert!(calls(&r.e, ACTOR_DRINK_POTION).is_empty());
        assert_eq!(calls(&r.e, SLOT + 0x17c).len(), 1);
    }

    #[test]
    fn trade_closes_and_runs_its_command() {
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_68d0, &args![menu]);
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT).len(), 1);
        assert_eq!(
            calls(&r.e, GET_FORM_BY_EDITOR_ID),
            vec![vec![FOLLOWERS_TRADE_TEXT]]
        );
    }

    #[test]
    fn inventory_button_opens_the_companions_inventory() {
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_6930, &args![menu]);
        assert_eq!(calls(&r.e, ACTOR_008A7760), vec![vec![r.companion]]);
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT).len(), 1);
    }

    #[test]
    fn talk_button_marks_the_talk_and_closes() {
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_6960, &args![menu]);
        assert_eq!(r.e.mem.u8(menu + 0x74), 1);
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT).len(), 1);
    }

    #[test]
    fn talk_to_queues_the_dialogue_menu() {
        let mut r = rig();
        let menu = r.menu;
        let companion = r.companion;
        let process = object(&mut r.e, 0x40);
        answer(&mut r.e, GET_PROCESS, process);
        // No companion process: nothing.
        let mut quiet = rig();
        let quiet_menu = quiet.menu;
        quiet.e.call(0x0075_6980, &args![quiet_menu]);
        assert!(calls(&quiet.e, QUEUE_MENU_CREATE).is_empty());
        // 00493bb0 and 00566950 true with the player speaking: queued at once.
        answer(&mut r.e, ACTOR_TEST_00493BB0, 1);
        answer(&mut r.e, ACTOR_TEST_00566950, 1);
        r.e.call(0x0075_6980, &args![menu]);
        assert_eq!(
            calls(&r.e, QUEUE_MENU_CREATE),
            vec![vec![4, companion, 0, 0, 1, 0]]
        );
        assert_eq!(calls(&r.e, VATS_QUIT_PLAYBACK), vec![vec![VATS, 0, 0]]);
        assert!(calls(&r.e, TOPIC_GET_TOPIC).is_empty());
    }

    #[test]
    fn talk_to_sets_up_the_conversation() {
        let mut r = rig();
        let menu = r.menu;
        let companion = r.companion;
        let player = r.player;
        let process = object(&mut r.e, 0x40);
        answer(&mut r.e, GET_PROCESS, process);
        answer(&mut r.e, TOPIC_GET_TOPIC, 0x0c80_0000);
        answer(&mut r.e, TOPIC_CREATE_DIALOGUE_ITEM, 0x0c81_0000);
        r.e.register(SLOT + 0x214, |_, _| 4u32.into_ret());
        r.e.register(SLOT + 0x2c8, |_, _| 0u32.into_ret());
        r.e.call(0x0075_6980, &args![menu]);
        assert_eq!(
            calls(&r.e, TOPIC_CREATE_DIALOGUE_ITEM),
            vec![vec![0x0c80_0000, companion, player, 0, 0, 0]]
        );
        assert_eq!(
            calls(&r.e, ACTOR_STOP_ATTACK),
            vec![vec![player], vec![companion]]
        );
        assert_eq!(calls(&r.e, AUDIO_END_ALL_SOUNDS_OF_TYPE)[0][1], 4);
        assert_eq!(
            calls(&r.e, QUEUE_MENU_CREATE),
            vec![vec![4, companion, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls(&r.e, DIALOGUE_ITEM_DELETE),
            vec![vec![0x0c81_0000, 1]]
        );
        // The process turned the companion to the player (not pathing or rotating).
        assert_eq!(calls(&r.e, SLOT + 0x294), vec![vec![process, companion]]);
    }

    #[test]
    fn talk_to_falls_back_to_the_message() {
        let mut r = rig();
        let menu = r.menu;
        let companion = r.companion;
        let process = object(&mut r.e, 0x40);
        answer(&mut r.e, GET_PROCESS, process);
        // 00493bb0 true, +0x230 false: no talk; the companion's +0x428 object is shown.
        answer(&mut r.e, ACTOR_TEST_00493BB0, 1);
        r.e.register(SLOT + 0x428, |_, _| 0x0c90_0000u32.into_ret());
        answer(&mut r.e, OBJECT_TEST_0047C850, 1);
        answer(&mut r.e, SETTING_STRING, 0x0c50_0000);
        answer(&mut r.e, REFERENCE_NAME_0055D520, 0x0c51_0000);
        r.e.set_global(MESSAGE_SECONDS, 2.0f32);
        r.e.call(0x0075_6980, &args![menu]);
        let printed = calls(&r.e, SPRINTF);
        assert_eq!(
            printed[0][1..],
            [TWO_STRINGS_FORMAT, 0x0c51_0000, 0x0c50_0000]
        );
        let shown = calls(&r.e, SHOW_MESSAGE_007052F0);
        assert_eq!(shown[0][1..], [0, 0, 0, 2.0f32.to_bits(), 0]);
        assert_eq!(calls(&r.e, REFERENCE_NAME_0055D520), vec![vec![companion]]);
    }

    #[test]
    fn reactivity_bark_speaks_and_shows_the_subtitle() {
        let mut r = rig();
        let menu = r.menu;
        let companion = r.companion;
        let player = r.player;
        answer(&mut r.e, TOPIC_GET_MATCHING_INFO, 0x0ca0_0000);
        answer(&mut r.e, INFO_WORD_006733E0, 0x0ca1_0000);
        answer(&mut r.e, DIALOGUE_ITEM_GET_CURRENT_RESPONSE, 0x0ca2_0000);
        answer(&mut r.e, RESPONSE_00441110, 0x11);
        answer(&mut r.e, RESPONSE_009611E0, 0x22);
        answer(&mut r.e, STRING_LENGTH_004048E0, 0);
        answer(&mut r.e, VALUE_OF_0084E3A0, 0x44);
        answer(&mut r.e, WORD_AT_8_0044DDC0, 0x55);
        answer(&mut r.e, WORD_AT_0_00559450, 0x66);
        r.e.call(0x0075_73d0, &args![menu, 0x0c80_0000u32]);
        let item = calls(&r.e, DIALOGUE_ITEM_CONSTRUCT)[0].clone();
        assert_eq!(
            item[1..],
            [0x0ca1_0000, 0x0c80_0000, 0x0ca0_0000, companion]
        );
        assert_eq!(
            calls(&r.e, SLOT + 0x284),
            vec![vec![
                companion,
                0x66,
                menu + 0x7c,
                0x55,
                0x44,
                0,
                0x22,
                0x11,
                player,
                1,
                1,
                0,
                0,
                1
            ]]
        );
        // The by-value arguments are built and destroyed.
        assert_eq!(calls(&r.e, STRING_CONSTRUCT).len(), 1);
        assert_eq!(calls(&r.e, STRING_DESTROY).len(), 1);
        assert_eq!(calls(&r.e, DIALOGUE_ITEM_DESTROY), vec![vec![item[0]]]);
        // Without a matching info nothing more runs.
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x0075_73d0, &args![menu, 0x0c80_0000u32]);
        assert!(calls(&r.e, DIALOGUE_ITEM_CONSTRUCT).is_empty());
    }

    #[test]
    fn follower_command_runs_both_result_scripts() {
        let mut r = rig();
        let menu = r.menu;
        let companion = r.companion;
        answer(&mut r.e, TOPIC_GET_MATCHING_INFO, 0x0ca0_0000);
        let data = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(data + 8, 1);
        r.e.register(INFO_GET_RESULT_SCRIPT, |_, a| {
            (0x0cb0_0000 + a[1]).into_ret()
        });
        answer(&mut r.e, SCRIPT_DATA_00500940, data);
        answer(&mut r.e, EXTRA_GET_SCRIPT_LOCALS, 0x0cc0_0000);
        r.e.call(0x0075_75b0, &args![menu, 0x0c80_0000u32]);
        assert_eq!(
            calls(&r.e, SCRIPT_RUN),
            vec![
                vec![0x0cb0_0000, companion, 0x0cc0_0000, 0, 1],
                vec![0x0cb0_0001, companion, 0x0cc0_0000, 0, 1],
            ]
        );
        r.e.mem.set_u32(data + 8, 0);
        r.e.call(0x0075_75b0, &args![menu, 0u32]);
        assert_eq!(calls(&r.e, SCRIPT_RUN).len(), 2);
    }

    #[test]
    fn subtitle_shows_and_times_from_the_text() {
        let mut r = rig();
        let menu = r.menu;
        r.e.mem.set_u32(menu + 0x58, 0x0d00_000c);
        answer(&mut r.e, STRING_LENGTH_004048E0, 5);
        let setting = r.e.mem.alloc(4);
        r.e.mem.set_u8(setting, b'1');
        answer(&mut r.e, STRING_CHARS_00408D60, setting);
        answer(&mut r.e, WORD_AT_0_00559450, 0x0cd0_0000);
        answer(&mut r.e, STRLEN, 10);
        let per_character = r.e.mem.alloc(4);
        r.e.mem.set_f32(per_character, 0.5);
        answer(&mut r.e, SETTING_VALUE, per_character);
        r.e.set_global(SUBTITLE_SCALE_DOUBLE, 1000.0f64);
        answer(&mut r.e, TIME_MS, 100);
        r.e.call(
            0x0075_7690,
            &args![menu, 0x1u32, 0x2u32, 0x3u32, 0x4u32, 0x5u32],
        );
        assert_eq!(
            calls(&r.e, TILE_SET_STRING),
            vec![vec![0x0d00_000c, 0xfc4, 0x0cd0_0000, 1]]
        );
        assert_eq!(r.e.mem.u32(menu + 0x78), 100 + 5000);
        assert_eq!(calls(&r.e, STRING_DESTROY).len(), 1);
        assert_eq!(calls(&r.e, SOUND_HANDLE_DESTROY).len(), 1);
        // A handle with a duration uses it.
        answer(&mut r.e, SOUND_HANDLE_GET_DURATION, 700);
        r.e.call(
            0x0075_7690,
            &args![menu, 0x1u32, 0x2u32, 0x3u32, 0x4u32, 0x5u32],
        );
        assert_eq!(r.e.mem.u32(menu + 0x78), 800);
    }

    #[test]
    fn subtitle_needs_text_and_subtitles_on() {
        let mut r = rig();
        let menu = r.menu;
        let setting = r.e.mem.alloc(4);
        r.e.mem.set_u8(setting, 0);
        answer(&mut r.e, STRING_CHARS_00408D60, setting);
        answer(&mut r.e, STRING_LENGTH_004048E0, 5);
        r.e.call(
            0x0075_7690,
            &args![menu, 0x1u32, 0x2u32, 0x3u32, 0x4u32, 0x5u32],
        );
        assert_eq!(calls(&r.e, ACTOR_ALWAYS_SHOW_SUBTITLES).len(), 1);
        assert_eq!(r.e.mem.u32(menu + 0x78), 0);
        assert_eq!(calls(&r.e, STRING_DESTROY).len(), 1);
    }
}
