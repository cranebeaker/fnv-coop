//! `fallout/interface/menus/sleepwaitmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The menu's six tile slots start at +0x28 (one word each).
const TILES: u32 = 0x28;

// Translated from 007a8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::AttachTileByID` (Xbox PDB): stores `tile` in slot `id`
/// of the six tile slots at +0x28; ids outside 0..6 (signed) are ignored.
pub fn sleep_wait_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..6).contains(&id) {
        let slot = this
            .addr()
            .wrapping_add(TILES)
            .wrapping_add((id as u32).wrapping_mul(4));
        e.mem.set_u32(slot, tile);
    }
}

/// The object in `011dea0c` whose `00877720` (thiscall, no stack words) answers the controls
/// object, and the controls' state query (`00a24660`, thiscall, two words: control and mode;
/// mode 2 "went up", 1 "went down").
const CONTROLS_HOLDER: u32 = 0x011d_ea0c;
const CONTROLS_OF: u32 = 0x0087_7720;
const CONTROL_STATE: u32 = 0x00a2_4660;
/// The control both queries ask about (0xF).
const CONTROL_0F: u32 = 0xf;
/// The player (`PlayerCharacter *`, `011dea3c`), `PlayerCharacter::IsSleepingorResting`
/// (engine map, `0094df60`), `004d1360` (thiscall: a byte), `PlayerCharacter::
/// UpdateHardcoreMode` (engine map, `00969c30`), `0094df80` and `005cde20` (thiscall: a
/// word); none takes stack words.
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
const PLAYER_TEST_004D1360: u32 = 0x004d_1360;
const PLAYER_UPDATE_HARDCORE_MODE: u32 = 0x0096_9c30;
const PLAYER_STEP_0094DF80: u32 = 0x0094_df80;
const PLAYER_VALUE_005CDE20: u32 = 0x005c_de20;
/// `Interface::IsConsoleVisible` (engine map, `00703d50`; no arguments).
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
/// The frame timer (`011f6394`) and its elapsed seconds (`0084d030`, thiscall, `ST0`).
const FRAME_TIMER: u32 = 0x011f_6394;
const ELAPSED_SECONDS: u32 = 0x0084_d030;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// The object in `011dea10` and its `00455490` (thiscall, no stack words).
const OBJECT_011DEA10: u32 = 0x011d_ea10;
const OBJECT_STEP_00455490: u32 = 0x0045_5490;
/// `Tile::TextToTrait` (engine map, `00a01860`; cdecl, one word), the trait setter from an
/// integer (`00700320`, thiscall on a tile: trait, value) and `Tile::Poke` (engine map,
/// `00a01290`, thiscall on a tile: trait, `float`).
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
const TILE_POKE: u32 = 0x00a0_1290;
/// `"_current_value"` and `"_SetInCode"`.
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;
/// `SleepWaitMenu::SetTimeText` (engine map, `007c0000`; thiscall, no stack words).
const SET_TIME_TEXT: u32 = 0x007c_0000;
/// `pTiles[1]`, `fElapsedSecs`, `bRestingActive` and `bButtonReleased` (Xbox PDB offsets,
/// matching the PC code).
const TILE_1: u32 = 0x2c;
const ELAPSED_SECS: u32 = 0x40;
const RESTING_ACTIVE: u32 = 0x44;
const BUTTON_RELEASED: u32 = 0x46;

/// The state of control 0xF in `mode` (`00877720` on `011dea0c`'s object, then `00a24660`).
fn control_state(e: &mut Engine, mode: u32) -> u32 {
    let holder: u32 = e.global(CONTROLS_HOLDER);
    let controls = e.call(CONTROLS_OF, &args![holder]).u32();
    e.call(CONTROL_STATE, &args![controls, CONTROL_0F, mode])
        .u32()
}

// Translated from 007c0580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::UpdateSleeping` (engine map; `ECX` only, the answer in `AL`). Control 0xF
/// going up sets `bButtonReleased`. When that is set and the control then goes down, or
/// when resting is active and the player is no longer sleeping or resting, the menu closes
/// unless the console is open: its vtable +0xC with (5, 0), answer false. Otherwise, while
/// resting is active, the frame's seconds (`0084d030` on `011f6394`) are added to
/// `fElapsedSecs`; once that reaches 1.0 (ordered) it restarts at 0, an hour passes
/// (`00455490` on `011dea10`'s object; `UpdateHardcoreMode` when the player's `004d1360`
/// answers true; the player's `0094df80`), the tile `pTiles[1]` gets `_current_value` =
/// the player's `005cde20` minus 1 and `_SetInCode` = 1.0, the time text is updated and the
/// answer is true. Every other path answers false.
pub fn sleep_wait_menu_update_sleeping(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if control_state(e, 2) != 0 {
        e.mem.set_u8(base + BUTTON_RELEASED, 1);
    }
    let close = if e.mem.u8(base + BUTTON_RELEASED) != 0 && control_state(e, 1) != 0 {
        true
    } else if e.mem.u8(base + RESTING_ACTIVE) == 0 {
        false
    } else {
        let player: u32 = e.global(PLAYER);
        !e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool()
    };
    if close && !e.call(IS_CONSOLE_VISIBLE, &args![]).bool() {
        e.vcall(base, 0xc, &args![5u32, 0u32]);
        return false;
    }
    if e.mem.u8(base + RESTING_ACTIVE) == 0 {
        return false;
    }
    let seconds = e.call(ELAPSED_SECONDS, &args![FRAME_TIMER]).st0;
    let sum = x87_add(seconds, x87_m32(e.mem.u32(base + ELAPSED_SECS)));
    e.mem.set_u32(base + ELAPSED_SECS, x87_store32(sum));
    let one: f64 = e.global(ONE);
    // FCOMP; TEST AH,1: goes on only when not below (C0 clear: ordered and >=).
    if !(x87_load32(e.mem.u32(base + ELAPSED_SECS)) >= one) {
        return false;
    }
    e.mem.set_u32(base + ELAPSED_SECS, 0);
    let object: u32 = e.global(OBJECT_011DEA10);
    e.call(OBJECT_STEP_00455490, &args![object]);
    let player: u32 = e.global(PLAYER);
    if e.call(PLAYER_TEST_004D1360, &args![player]).bool() {
        let player: u32 = e.global(PLAYER);
        e.call(PLAYER_UPDATE_HARDCORE_MODE, &args![player]);
    }
    let player: u32 = e.global(PLAYER);
    e.call(PLAYER_STEP_0094DF80, &args![player]);
    let player: u32 = e.global(PLAYER);
    let value = e
        .call(PLAYER_VALUE_005CDE20, &args![player])
        .u32()
        .wrapping_sub(1);
    let trait_id = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    let tile = e.mem.u32(base + TILE_1);
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
    let trait_id = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    let tile = e.mem.u32(base + TILE_1);
    e.call(TILE_POKE, &args![tile, trait_id, 1.0f32]);
    e.call(SET_TIME_TEXT, &args![this]);
    true
}

/// `SleepWaitMenu::pMe` (Xbox PDB static; `011da920`) and `iUT_CurrentValue` (Xbox PDB
/// static; `011da918`: the `"_current_value"` trait id).
const OPEN_MENU: u32 = 0x011d_a920;
const CURRENT_VALUE_TRAIT: u32 = 0x011d_a918;
/// The `SleepWaitMenu` vtable (`010763ac`) and the menu class (`0x3f4`).
const SLEEP_WAIT_MENU_VTABLE: u32 = 0x0107_63ac;
const SLEEP_WAIT_MENU_CLASS: u32 = 0x3f4;
/// `bSleepingInBed` and `fPrevQuantity` (Xbox PDB offsets, matching the PC code).
const SLEEPING_IN_BED: u32 = 0x45;
const PREV_QUANTITY: u32 = 0x48;
/// The `Menu` constructor (`00a1c4a0`) and destructor (`00a1c520`) bodies, and `MemoryManager`
/// free (`00401030`, `cdecl`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const MENU_DESTROY: u32 = 0x00a1_c520;
const FREE: u32 = 0x0040_1030;
/// The interface manager (`004b7210`) and its root tile (`00586150`); `00705770` (no
/// arguments: the menu class the code compares with).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const MANAGER_ROOT_00586150: u32 = 0x0058_6150;
const MENU_CLASS_00705770: u32 = 0x0070_5770;
/// `Tile::GetMenuByClass` (`00a09030`, `cdecl`), `Tile::ReadFile` (`00a01b00`),
/// `Tile::GetMenu` (`00a03c90`), `Menu::SetMenuTile` (`00a1dc70`),
/// `Menu::PrepForVisibility` (`00a1dc20`), `Menu::StartFadeOut` (`00a1d910`),
/// `Tile::GetMaximumDepth` (`00a01530`, `ST0`), `Tile::GetChildByName` (`00a03da0`) and the
/// menu's tile (`00726070`).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const MENU_TILE_00726070: u32 = 0x0072_6070;
/// `007abdb0` (`thiscall` on the menu, a byte: whether its tiles are there).
const MENU_CHECK_007ABDB0: u32 = 0x007a_bdb0;
/// The tile trait calls: string (`00a01350`), `float` set (`00a012d0`) and get (`00a011b0`,
/// `ST0`), `Tile::IsTrue` (`00a01230`, a byte).
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// The debug message printer (`005b5e40`, `cdecl`).
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
/// `BSAudio::QInstance` (`00453a70`), `GetSoundHandleByName` (`00ad7550`, three words),
/// `BSSoundHandle::Play` (`00ad8830`, one word) and the handle's destructor (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// A game setting's string (`00403df0`) and the address of its value (`00403e20`).
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The bounded string copy (`00406d30`, `cdecl`: buffer, size, text), the bounded formatter
/// (`00406d00`, `cdecl`), `toupper` (`00eca7f4`, `cdecl`), the `float` to integer helper
/// (`00406d90`, `cdecl`, one `float`) and `_ftol2` (`00ec62c0`, `ST0`).
const COPY_00406D30: u32 = 0x0040_6d30;
const FORMAT_00406D00: u32 = 0x0040_6d00;
const TOUPPER: u32 = 0x00ec_a7f4;
const FLOAT_TO_INT_00406D90: u32 = 0x0040_6d90;
const FTOL2: u32 = 0x00ec_62c0;
/// The calendar (`011de7b8`): `Calendar::GetHour` (`00867da0`, `ST0`),
/// `Calendar::GetDateString` (`00867970`, one word: the `BSStringT` it fills) and
/// `00867f10` (a text); the `BSStringT` destructor (`004037d0`) and its characters
/// (`00559450`).
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR_GET_DATE_STRING: u32 = 0x0086_7970;
const CALENDAR_TEXT_00867F10: u32 = 0x0086_7f10;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_CHARS: u32 = 0x0055_9450;
/// The object at `011ddf38` and its test (`0042ce10`), the process lists (`011e0e80`) with
/// `Update3DAfterResting` (`00975d10`), `StopAllActorsInDialog` (`00974fc0`) and
/// `KillAllProjectiles` (`00975160`).
const OBJECT_011DDF38: u32 = 0x011d_df38;
const OBJECT_TEST_0042CE10: u32 = 0x0042_ce10;
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LISTS_UPDATE_3D_AFTER_RESTING: u32 = 0x0097_5d10;
const PROCESS_LISTS_STOP_ALL_ACTORS_IN_DIALOG: u32 = 0x0097_4fc0;
const PROCESS_LISTS_KILL_ALL_PROJECTILES: u32 = 0x0097_5160;
/// The fader manager (`011d8804`): `FaderManager::CreateFader` (`00700960`, three words) and
/// `RemoveFader` (`007010e0`, two words).
const FADER_MANAGER: u32 = 0x011d_8804;
const FADER_CREATE: u32 = 0x0070_0960;
const FADER_REMOVE: u32 = 0x0070_10e0;
/// `InterfaceManager::PlayMenuSound` (`00717280`, `cdecl`), `Interface::PlayMenuSound`
/// (engine map name, `00706f30`, `cdecl`), the player's wait (`005c1a00`, two words) and
/// test (`005c7870`, a byte), `MiscStatManager::Increment` (`004d5c60`, `cdecl`), the
/// setting string at `00408d60` and `00850a40` on the object at `011de134`.
const INTERFACE_MANAGER_PLAY_MENU_SOUND: u32 = 0x0071_7280;
const INTERFACE_PLAY_MENU_SOUND: u32 = 0x0070_6f30;
const PLAYER_WAIT_005C1A00: u32 = 0x005c_1a00;
const PLAYER_TEST_005C7870: u32 = 0x005c_7870;
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const STRING_CHARS_00408D60: u32 = 0x0040_8d60;
const OBJECT_011DE134: u32 = 0x011d_e134;
const SAVE_00850A40: u32 = 0x0085_0a40;

/// `"Data\Menus\sleep_wait_menu.xml"`, the creation failure message,
/// `"UIPopUpMessageGeneral"`, `"%s %s?"`, `"1 %s"`, `"SWM_Background"`,
/// `"%s, %s, %d:%02d %s"` and `"%d %s"`.
const MENU_FILE_TEXT: u32 = 0x0107_6468;
const CREATION_FAILED_TEXT: u32 = 0x0107_6410;
const POP_UP_SOUND_TEXT: u32 = 0x0102_5cc4;
const QUESTION_FORMAT: u32 = 0x0107_4e34;
const ONE_HOUR_FORMAT: u32 = 0x0107_6404;
const BACKGROUND_TEXT: u32 = 0x0107_63f4;
const TIME_FORMAT: u32 = 0x0107_6488;
const HOURS_FORMAT: u32 = 0x0107_29ac;
/// `0.0`, `255.0`, `60.0`, `13.0` and `12.0` (`double`s); `6006` and `102` (`double`s);
/// `-2.0` and `2.0` (`float`s).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const BYTE_SCALE_DOUBLE: u32 = 0x0101_e568;
const SIXTY_DOUBLE: u32 = 0x0101_2638;
const THIRTEEN_DOUBLE: u32 = 0x0103_5708;
const TWELVE_DOUBLE: u32 = 0x0103_5710;
const VALUE_6006_DOUBLE: u32 = 0x0107_04f0;
const VALUE_102_DOUBLE: u32 = 0x0107_04e8;
const STEP_DOWN: u32 = 0x0104_a494;
const STEP_UP: u32 = 0x0101_62c0;
/// The settings the menu's texts come from (`Setting` objects).
const SETTING_011D3948: u32 = 0x011d_3948;
const SETTING_011D23AC: u32 = 0x011d_23ac;
const SETTING_011D2910: u32 = 0x011d_2910;
const SETTING_011D448C: u32 = 0x011d_448c;
const SETTING_011D29F4: u32 = 0x011d_29f4;
const SETTING_011D3174: u32 = 0x011d_3174;
const SETTING_011D24FC: u32 = 0x011d_24fc;
const SETTING_011D27F0: u32 = 0x011d_27f0;
const SETTING_011D3AE0: u32 = 0x011d_3ae0;
const SETTING_011C3E7C: u32 = 0x011c_3e7c;
const SETTING_011D89F0: u32 = 0x011d_89f0;
const SETTING_011D8974: u32 = 0x011d_8974;

/// A tile's string trait `0xfc4` (`00a01350` with 1).
fn set_text(e: &mut Engine, tile: u32, text: u32) {
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
}

/// A game setting's string (`00403df0`).
fn setting_string(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_STRING, &args![setting]).u32()
}

/// The open menu (`pMe`).
fn open_menu(e: &Engine) -> u32 {
    e.global(OPEN_MENU)
}

// Translated from 007bfb30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bfb30 - the C++ exception frame (FS:[0] chain, handler 00f0d8f8, unwind state 0 for the Menu base) is not translated
/// `SleepWaitMenu::SleepWaitMenu` (engine map; `thiscall`): the `Menu` base (`00a1c4a0`),
/// the vtable `010763ac`, `fElapsedSecs` and `fPrevQuantity` 0.0, `bRestingActive` and
/// `bButtonReleased` cleared, and `iUT_CurrentValue` looked up (`Tile::TextToTrait`).
/// Returns `this`.
pub fn sleep_wait_menu_sleep_wait_menu(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, SLEEP_WAIT_MENU_VTABLE);
    e.mem.set_u32(base + ELAPSED_SECS, 0.0f32.to_bits());
    e.mem.set_u8(base + RESTING_ACTIVE, 0);
    e.mem.set_u32(base + PREV_QUANTITY, 0.0f32.to_bits());
    e.mem.set_u8(base + BUTTON_RELEASED, 0);
    let id = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    e.set_global(CURRENT_VALUE_TRAIT, id);
    this
}

// Translated from 007bfbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::GetClass` (engine map; `thiscall`): `0x3f4`.
pub fn sleep_wait_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    SLEEP_WAIT_MENU_CLASS
}

// Translated from 007bfbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::scalar deleting destructor` (engine map; `RET 4`): the destructor body
/// (`007bfc00`), then the memory freed (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn sleep_wait_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007bfc00(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007bfc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SleepWaitMenu` destructor body (unnamed in the engine map; `thiscall`): the vtable
/// `010763ac`, `pMe` cleared, the `Menu` destructor body (`00a1c520`).
pub fn fn_007bfc00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SLEEP_WAIT_MENU_VTABLE);
    e.set_global(OPEN_MENU, 0u32);
    e.call(MENU_DESTROY, &args![this]);
}

// Translated from 007bfc30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bfc30 - the C++ exception frame (FS:[0] chain, handler 00f0dc6b, unwind state 0 for the sound handle) and the /GS stack cookie check (00ec408c) are not translated
/// `SleepWaitMenu::Create` (engine map; `cdecl`, one word: a byte, sleeping in a bed).
/// Returns the menu's tile, or 0.
///
/// An open menu of class `00705770()` is deleted (vtable slot 0 with 1);
/// `sleep_wait_menu.xml` is read (`Tile::ReadFile` on the interface manager's root) and its
/// menu becomes `pMe`. When that is of the class (vtable +0x34): the tile becomes the
/// menu's; when `007abdb0` answers false a message is logged and 0 returned. Otherwise
/// `"UIPopUpMessageGeneral"` plays, a trait `0xfa7` of 6006 or 102 gets the maximum depth
/// as trait `0xfad`, `bSleepingInBed` is stored and the texts are set: the verb (a setting's
/// string, one for sleeping and one for waiting, copied into a stack buffer) in
/// `"%s %s?"` after a setting's string on tile 0; the verb with its first letter upper-cased
/// (`toupper`) on tile 4; `"1 %s"` of a setting's string on tile 2; the time text
/// (`SetTimeText`); a setting's string on tile 5; trait `0xfa9` of the child
/// `"SWM_Background"` of the menu's tile = the `float` setting at `011d3174` times 255.0.
/// The menu is prepared for visibility and the tile returned. When no such menu was made, a
/// menu that has a tile is deleted (vtable slot 0 with 1) and 0 returned.
pub fn sleep_wait_menu_create(e: &mut Engine, sleeping: u8) -> u32 {
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let class = e.call(MENU_CLASS_00705770, &args![]).u32();
    let open = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if open != 0 {
        e.vcall(open, 0, &args![1u32]);
    }
    let root = e.call(MANAGER_ROOT_00586150, &args![manager]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.set_global(OPEN_MENU, menu);
    let made = open_menu(e) != 0 && {
        let class = e.vcall(open_menu(e), 0x34, &args![]).u32();
        class == e.call(MENU_CLASS_00705770, &args![]).u32()
    };
    if !made {
        if open_menu(e) != 0 && e.call(MENU_TILE_00726070, &args![open_menu(e)]).u32() != 0 {
            let menu = open_menu(e);
            if menu != 0 {
                e.vcall(menu, 0, &args![1u32]);
            }
        }
        return 0;
    }
    let menu = open_menu(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
    let menu = open_menu(e);
    if !e.call(MENU_CHECK_007ABDB0, &args![menu]).bool() {
        e.call(DEBUG_MESSAGE, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
    e.with_stack(0xc, |e, local| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let handle = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, local, POP_UP_SOUND_TEXT, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![local]);
    });
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
    e.mem.set_u8(open_menu(e) + SLEEPING_IN_BED, sleeping);
    e.with_stack(0x210, |e, buffers| {
        let verb = buffers.addr();
        let line = verb + 0x108;
        let setting = if sleeping != 0 {
            SETTING_011D3948
        } else {
            SETTING_011D23AC
        };
        let text = setting_string(e, setting);
        e.call(COPY_00406D30, &args![verb, 0x104u32, text]);
        let text = setting_string(e, SETTING_011D2910);
        e.call(
            FORMAT_00406D00,
            &args![line, 0x104u32, QUESTION_FORMAT, text, verb],
        );
        let tile = e.mem.u32(open_menu(e) + 0x28);
        set_text(e, tile, line);
        let first = e.mem.u8(verb) as i8 as i32;
        let upper = e.call(TOUPPER, &args![first]).u32();
        e.mem.set_u8(verb, upper as u8);
        let tile = e.mem.u32(open_menu(e) + 0x38);
        set_text(e, tile, verb);
        let text = setting_string(e, SETTING_011D448C);
        e.call(
            FORMAT_00406D00,
            &args![line, 0x104u32, ONE_HOUR_FORMAT, text],
        );
        let tile = e.mem.u32(open_menu(e) + 0x30);
        set_text(e, tile, line);
    });
    let menu = open_menu(e);
    sleep_wait_menu_set_time_text(e, Ptr::new(menu));
    let text = setting_string(e, SETTING_011D29F4);
    let tile_5 = e.mem.u32(open_menu(e) + 0x3c);
    set_text(e, tile_5, text);
    let value = e.call(SETTING_VALUE, &args![SETTING_011D3174]).u32();
    let scale: f64 = e.global(BYTE_SCALE_DOUBLE);
    let alpha = x87_store32(x87_mul(x87_load32(e.mem.u32(value)), scale));
    let menu = open_menu(e);
    let root = e.call(MENU_TILE_00726070, &args![menu]).u32();
    let child = e
        .call(TILE_GET_CHILD_BY_NAME, &args![root, BACKGROUND_TEXT])
        .u32();
    e.call(
        TILE_SET_FLOAT,
        &args![child, 0xfa9u32, x87_quiet_bits(alpha), 1u32],
    );
    let menu = open_menu(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    tile
}

// Translated from 007c0000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c0000 - the C++ exception frame (FS:[0] chain, handler 00f0dcab, unwind state 0 for the date string) and the /GS stack cookie check (00ec408c) are not translated
/// `SleepWaitMenu::SetTimeText` (engine map; `thiscall`). The calendar's hour
/// (`Calendar::GetHour`, stored as a `float`) gives the hour (`_ftol2`) and minutes
/// (`_ftol2` of the fraction times 60.0); the hour shown is 12 for 0 to below 1, 12 less
/// from 13 on; the suffix is one setting's string below 12 and another from 12. Tile 3
/// (+0x34) gets `"%s, %s, %d:%02d %s"` of the calendar's `00867f10` text, its date string
/// (`Calendar::GetDateString` into a stack `BSStringT`, destroyed after), the hour, the
/// minutes and the suffix.
pub fn sleep_wait_menu_set_time_text(e: &mut Engine, this: Ptr) {
    let hour = x87_store32(e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64());
    let whole = e.call(FTOL2, &args![x87_load32(hour)]).i32();
    let sixty: f64 = e.global(SIXTY_DOUBLE);
    // FILD whole; FSUBR hour; FMUL 60.0.
    let fraction = x87_sub(x87_m32(hour), f64::from(whole));
    let minutes = e.call(FTOL2, &args![x87_mul(fraction, sixty)]).u32();
    let zero: f64 = e.global(ZERO_DOUBLE);
    let one: f64 = e.global(ONE);
    let thirteen: f64 = e.global(THIRTEEN_DOUBLE);
    let twelve: f64 = e.global(TWELVE_DOUBLE);
    let value = x87_load32(hour);
    let mut shown = whole;
    // FCOMP 0.0; TEST AH,1; JNZ (below or unordered fails), FCOMP 1.0; TEST AH,5; JP.
    if value >= zero && value < one {
        shown = 12;
    } else if value >= thirteen {
        shown = shown.wrapping_sub(12);
    }
    let setting = if value < twelve {
        SETTING_011D24FC
    } else {
        SETTING_011D27F0
    };
    let suffix = setting_string(e, setting);
    e.with_stack(0x118, |e, frame| {
        let date = frame.addr();
        let buffer = date + 0xc;
        let string = e
            .call(CALENDAR_GET_DATE_STRING, &args![CALENDAR, date])
            .u32();
        let chars = e.call(STRING_CHARS, &args![string]).u32();
        let text = e.call(CALENDAR_TEXT_00867F10, &args![CALENDAR]).u32();
        e.call(
            FORMAT_00406D00,
            &args![
                buffer,
                0x104u32,
                TIME_FORMAT,
                text,
                chars,
                shown,
                minutes,
                suffix
            ],
        );
        e.call(STRING_DESTROY, &args![date]);
        let tile = e.mem.u32(this.addr() + 0x34);
        set_text(e, tile, buffer);
    });
}

// Translated from 007c01b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::Close` (engine map; no arguments). With a menu open: unless the object
/// at `011ddf38` answers `0042ce10`, `ProcessLists::Update3DAfterResting`; when sleeping in
/// a bed the fader is removed (`FaderManager::RemoveFader(0, 0)`); the menu's tile gets
/// trait `0x1772` = 1 and the menu starts fading out.
pub fn sleep_wait_menu_close(e: &mut Engine) {
    if open_menu(e) == 0 {
        return;
    }
    let object: u32 = e.global(OBJECT_011DDF38);
    if !e.call(OBJECT_TEST_0042CE10, &args![object]).bool() {
        e.call(PROCESS_LISTS_UPDATE_3D_AFTER_RESTING, &args![PROCESS_LISTS]);
    }
    if e.mem.u8(open_menu(e) + SLEEPING_IN_BED) != 0 {
        let faders: u32 = e.global(FADER_MANAGER);
        e.call(FADER_REMOVE, &args![faders, 0u32, 0u32]);
    }
    let menu = open_menu(e);
    let tile = e.call(MENU_TILE_00726070, &args![menu]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0x1772u32, 1u32]);
    let menu = open_menu(e);
    e.call(MENU_START_FADE_OUT, &args![menu]);
}

// Translated from 007c0220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::DoClick` (engine map; `RET 8`: the button id and its tile, which the code
/// fills from `pTiles` when 0 but does not use). Button 4 (rest): menu sound 1; `00850a40`
/// on `011de134`'s object when the setting string for the mode (`011d89f0` in a bed,
/// `011d8974` otherwise) is not empty; in a bed a fader is created (`CreateFader(0, the
/// float setting at 011c3e7c, 0)`); resting starts; tiles 4 and 1 get trait `0xfaf` = 0;
/// the player waits (`005c1a00`) the hours of tile 1's `iUT_CurrentValue` (`_ftol2`) plus
/// one, with the bed flag; in a bed miscellaneous stat 0x17 counts; unless the player's
/// `005c7870` answers true the actors in dialogue stop; projectiles are killed. Button 5
/// (cancel): menu sound 2, the player's `005c1a00(0, 0)` and `Close`.
pub fn sleep_wait_menu_do_click(e: &mut Engine, this: Ptr, id: i32, _tile: u32) {
    let base = this.addr();
    match id {
        4 => {
            e.call(INTERFACE_MANAGER_PLAY_MENU_SOUND, &args![1u32]);
            let in_bed = e.mem.u8(base + SLEEPING_IN_BED) != 0;
            let setting_set = |e: &mut Engine, setting: u32| {
                let text = e.call(STRING_CHARS_00408D60, &args![setting]).u32();
                e.mem.u8(text) != 0
            };
            let save = (in_bed && setting_set(e, SETTING_011D89F0))
                || (e.mem.u8(base + SLEEPING_IN_BED) == 0 && setting_set(e, SETTING_011D8974));
            if save {
                let object: u32 = e.global(OBJECT_011DE134);
                e.call(SAVE_00850A40, &args![object]);
            }
            if e.mem.u8(base + SLEEPING_IN_BED) != 0 {
                let value = e.call(SETTING_VALUE, &args![SETTING_011C3E7C]).u32();
                let seconds = x87_quiet_bits(e.mem.u32(value));
                let faders: u32 = e.global(FADER_MANAGER);
                e.call(FADER_CREATE, &args![faders, 0u32, seconds, 0u32]);
            }
            e.mem.set_u8(base + RESTING_ACTIVE, 1);
            let tile = e.mem.u32(base + 0x38);
            e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfafu32, 0u32]);
            let tile = e.mem.u32(base + TILE_1);
            e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfafu32, 0u32]);
            let in_bed = u32::from(e.mem.u8(base + SLEEPING_IN_BED));
            let trait_id: u32 = e.global(CURRENT_VALUE_TRAIT);
            let tile = e.mem.u32(base + TILE_1);
            let value = e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64();
            let hours = e.call(FTOL2, &args![value]).u32().wrapping_add(1);
            let player: u32 = e.global(PLAYER);
            e.call(PLAYER_WAIT_005C1A00, &args![player, hours, in_bed]);
            if e.mem.u8(base + SLEEPING_IN_BED) != 0 {
                e.call(MISC_STAT_INCREMENT, &args![0x17u32]);
            }
            let player: u32 = e.global(PLAYER);
            if !e.call(PLAYER_TEST_005C7870, &args![player]).bool() {
                e.call(
                    PROCESS_LISTS_STOP_ALL_ACTORS_IN_DIALOG,
                    &args![PROCESS_LISTS],
                );
            }
            e.call(PROCESS_LISTS_KILL_ALL_PROJECTILES, &args![PROCESS_LISTS]);
        }
        5 => {
            e.call(INTERFACE_MANAGER_PLAY_MENU_SOUND, &args![2u32]);
            let player: u32 = e.global(PLAYER);
            e.call(PLAYER_WAIT_005C1A00, &args![player, 0u32, 0u32]);
            sleep_wait_menu_close(e);
        }
        _ => {}
    }
}

// Translated from 007c03a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c03a0 - the /GS stack cookie check (00ec408c) is not translated
/// `SleepWaitMenu::DoIdle` (engine map; `thiscall`). When tile 1's `iUT_CurrentValue`
/// differs from `fPrevQuantity` (`FUCOMPP; TEST AH,0x44; JNP`: unordered counts as
/// different): menu sound 4 (`00706f30`) when tile 4's trait `0xfaf` is true; the value is
/// stored (as a `float`); tile 2 (+0x30) gets `"%d %s"` of `00406d90(value + 1.0)` and one
/// setting's string above 0.0, another otherwise.
pub fn sleep_wait_menu_do_idle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let trait_id: u32 = e.global(CURRENT_VALUE_TRAIT);
    let tile = e.mem.u32(base + TILE_1);
    let value = e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64();
    if x87_load32(e.mem.u32(base + PREV_QUANTITY)) == value {
        return;
    }
    let tile = e.mem.u32(base + 0x38);
    if e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool() {
        e.call(INTERFACE_PLAY_MENU_SOUND, &args![4u32]);
    }
    let trait_id: u32 = e.global(CURRENT_VALUE_TRAIT);
    let tile = e.mem.u32(base + TILE_1);
    let value = e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64();
    e.mem.set_u32(base + PREV_QUANTITY, x87_store32(value));
    let zero: f64 = e.global(ZERO_DOUBLE);
    // FCOMP 0.0; TEST AH,0x41; JNZ: the first string only when strictly above.
    let setting = if x87_load32(e.mem.u32(base + PREV_QUANTITY)) > zero {
        SETTING_011D3AE0
    } else {
        SETTING_011D448C
    };
    let text = setting_string(e, setting);
    let one: f64 = e.global(ONE);
    let next = x87_store32(x87_add(x87_load32(e.mem.u32(base + PREV_QUANTITY)), one));
    let count = e
        .call(FLOAT_TO_INT_00406D90, &args![x87_quiet_bits(next)])
        .u32();
    e.with_stack(0x110, |e, buffer| {
        e.call(
            FORMAT_00406D00,
            &args![buffer, 0x104u32, HOURS_FORMAT, count, text],
        );
        let tile = e.mem.u32(base + 0x30);
        set_text(e, tile, buffer.addr());
    });
}

// Translated from 007c04d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::DoGamepad` (engine map; `RET 8`: the button and a word it does not read;
/// the answer in `AL`). While tile 4's trait `0xfaf` is true, buttons 0xf and 0x10 move
/// tile 1's `iUT_CurrentValue` by the `float` at `0104a494` (-2.0) or `010162c0` (2.0) (set
/// with 0), poke `"_SetInCode"` with 1.0 and answer true; everything else answers false.
pub fn sleep_wait_menu_do_gamepad(e: &mut Engine, this: Ptr, button: i32, _unused_2: u32) -> bool {
    let base = this.addr();
    let tile = e.mem.u32(base + 0x38);
    if !e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool() {
        return false;
    }
    if button != 0xf && button != 0x10 {
        return false;
    }
    let step = if button == 0xf {
        x87_quiet_bits(e.mem.u32(STEP_DOWN))
    } else {
        x87_quiet_bits(e.mem.u32(STEP_UP))
    };
    let trait_id: u32 = e.global(CURRENT_VALUE_TRAIT);
    let tile = e.mem.u32(base + TILE_1);
    let value = e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64();
    let moved = x87_store32(x87_add(value, x87_m32(step)));
    let trait_id: u32 = e.global(CURRENT_VALUE_TRAIT);
    let tile = e.mem.u32(base + TILE_1);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, trait_id, x87_quiet_bits(moved), 0u32],
    );
    let trait_id = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    let tile = e.mem.u32(base + TILE_1);
    e.call(TILE_POKE, &args![tile, trait_id, 1.0f32]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007a_8ab0,
            sleep_wait_menu_attach_tile_by_id(Ptr, i32, u32)
        ),
        entry!(0x007c_0580, sleep_wait_menu_update_sleeping(Ptr) -> bool),
        entry!(0x007b_fb30, sleep_wait_menu_sleep_wait_menu(Ptr) -> Ptr),
        entry!(0x007b_fbc0, sleep_wait_menu_get_class(Ptr) -> u32),
        entry!(
            0x007b_fbd0,
            sleep_wait_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x007b_fc00, fn_007bfc00(Ptr)),
        entry!(0x007b_fc30, sleep_wait_menu_create(u8) -> u32),
        entry!(0x007c_0000, sleep_wait_menu_set_time_text(Ptr)),
        entry!(0x007c_01b0, sleep_wait_menu_close()),
        entry!(0x007c_0220, sleep_wait_menu_do_click(Ptr, i32, u32)),
        entry!(0x007c_03a0, sleep_wait_menu_do_idle(Ptr)),
        entry!(
            0x007c_04d0,
            sleep_wait_menu_do_gamepad(Ptr, i32, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attaches_only_ids_zero_to_five() {
        let mut e = Engine::new();
        let menu = e.mem.alloc(0x48);
        e.call(0x007a_8ab0, &args![menu, 0i32, 0x11u32]);
        e.call(0x007a_8ab0, &args![menu, 5i32, 0x55u32]);
        e.call(0x007a_8ab0, &args![menu, 6i32, 0x66u32]);
        e.call(0x007a_8ab0, &args![menu, -1i32, 0x99u32]);
        assert_eq!(e.mem.u32(menu + 0x28), 0x11);
        assert_eq!(e.mem.u32(menu + 0x3c), 0x55);
        assert_eq!(e.mem.u32(menu + 0x40), 0);
        assert_eq!(e.mem.u32(menu + 0x24), 0);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The menu's close slot (vtable +0xC) as the tests place it.
    const CLOSE_SLOT: u32 = 0x0090_000c;

    struct Rig {
        e: Engine,
        menu: u32,
    }

    /// `went_up` and `went_down` are the control state answers for modes 2 and 1,
    /// `sleeping` the player's sleeping-or-resting answer, `console` whether the console is
    /// open, `seconds` the frame's elapsed seconds.
    fn rig(went_up: u32, went_down: u32, sleeping: bool, console: bool, seconds: f64) -> Rig {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(ONE, 1.0f64);
        e.set_global(CONTROLS_HOLDER, 0x0c00_0000u32);
        e.set_global(PLAYER, 0x0c10_0000u32);
        e.set_global(OBJECT_011DEA10, 0x0c20_0000u32);
        e.register(CONTROLS_OF, |_, _| 0x0c30_0000u32.into_ret());
        e.register_double(CONTROL_STATE, move |_, a| {
            assert_eq!(a[..2], [0x0c30_0000, CONTROL_0F]);
            (if a[2] == 2 { went_up } else { went_down }).into_ret()
        });
        e.register_double(PLAYER_IS_SLEEPING_OR_RESTING, move |_, _| {
            u32::from(sleeping).into_ret()
        });
        e.register_double(IS_CONSOLE_VISIBLE, move |_, _| {
            u32::from(console).into_ret()
        });
        e.register_double(ELAPSED_SECONDS, move |_, a| {
            assert_eq!(a[0], FRAME_TIMER);
            seconds.into_ret()
        });
        e.register(PLAYER_TEST_004D1360, |_, _| 1u32.into_ret());
        e.register(PLAYER_VALUE_005CDE20, |_, _| 8u32.into_ret());
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        for callee in [
            CLOSE_SLOT,
            OBJECT_STEP_00455490,
            PLAYER_UPDATE_HARDCORE_MODE,
            PLAYER_STEP_0094DF80,
            SET_TRAIT_FROM_INT,
            TILE_POKE,
            SET_TIME_TEXT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0090_1000, &[0, 0, 0, CLOSE_SLOT]);
        let menu = e.mem.alloc(0x4c);
        e.mem.set_u32(menu, 0x0090_1000);
        e.mem.set_u32(menu + TILE_1, 0x0d00_0000);
        Rig { e, menu }
    }

    fn update(rig: &mut Rig) -> (bool, Vec<(u32, Vec<u32>)>) {
        rig.e.call_log = Some(vec![]);
        let answer = rig.e.call(0x007c_0580, &args![rig.menu]).bool();
        (answer, rig.e.call_log.take().unwrap())
    }

    fn called(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn released_then_pressed_closes_the_menu() {
        let mut r = rig(1, 1, true, false, 0.0);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert_eq!(r.e.mem.u8(r.menu + BUTTON_RELEASED), 1);
        assert_eq!(called(&log, CLOSE_SLOT), vec![vec![r.menu, 5, 0]]);
        assert!(called(&log, ELAPSED_SECONDS).is_empty());
    }

    #[test]
    fn the_console_keeps_the_menu_open() {
        let mut r = rig(1, 1, true, true, 0.25);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert!(called(&log, CLOSE_SLOT).is_empty());
        // Not resting: nothing more.
        assert!(called(&log, ELAPSED_SECONDS).is_empty());
    }

    #[test]
    fn resting_stops_when_the_player_no_longer_sleeps() {
        let mut r = rig(0, 0, false, false, 0.25);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert_eq!(called(&log, CLOSE_SLOT).len(), 1);
        // Without the release, the went-down state is not asked for.
        let modes: Vec<u32> = called(&log, CONTROL_STATE).iter().map(|w| w[2]).collect();
        assert_eq!(modes, vec![2]);
    }

    #[test]
    fn resting_accumulates_below_one_second() {
        let mut r = rig(0, 0, true, false, 0.25);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        r.e.mem.set_f32(r.menu + ELAPSED_SECS, 0.5);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert_eq!(r.e.mem.f32(r.menu + ELAPSED_SECS), 0.75);
        assert!(called(&log, OBJECT_STEP_00455490).is_empty());
        assert!(called(&log, CLOSE_SLOT).is_empty());
    }

    #[test]
    fn a_full_second_passes_an_hour() {
        let mut r = rig(0, 0, true, false, 0.5);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        r.e.mem.set_f32(r.menu + ELAPSED_SECS, 0.5);
        let (answer, log) = update(&mut r);
        assert!(answer);
        assert_eq!(r.e.mem.f32(r.menu + ELAPSED_SECS), 0.0);
        assert_eq!(called(&log, OBJECT_STEP_00455490), vec![vec![0x0c20_0000]]);
        assert_eq!(
            called(&log, PLAYER_UPDATE_HARDCORE_MODE),
            vec![vec![0x0c10_0000]]
        );
        assert_eq!(called(&log, PLAYER_STEP_0094DF80), vec![vec![0x0c10_0000]]);
        assert_eq!(
            called(&log, SET_TRAIT_FROM_INT),
            vec![vec![0x0d00_0000, CURRENT_VALUE_TEXT & 0xffff, 7]]
        );
        assert_eq!(
            called(&log, TILE_POKE),
            vec![vec![
                0x0d00_0000,
                SET_IN_CODE_TEXT & 0xffff,
                1.0f32.to_bits()
            ]]
        );
        assert_eq!(called(&log, SET_TIME_TEXT), vec![vec![r.menu]]);
    }

    #[test]
    fn a_nan_sum_does_not_pass_an_hour() {
        let mut r = rig(0, 0, true, false, f64::NAN);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert!(r.e.mem.f32(r.menu + ELAPSED_SECS).is_nan());
        assert!(called(&log, OBJECT_STEP_00455490).is_empty());
    }

    #[test]
    fn without_the_hardcore_byte_the_mode_is_not_updated() {
        let mut r = rig(0, 0, true, false, 1.0);
        r.e.register(PLAYER_TEST_004D1360, |_, _| 0x100u32.into_ret());
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        let seen = Rc::new(RefCell::new(0u32));
        let count = seen.clone();
        r.e.register_double(PLAYER_UPDATE_HARDCORE_MODE, move |_, _| {
            *count.borrow_mut() += 1;
            Ret::default()
        });
        let (answer, _) = update(&mut r);
        assert!(answer);
        assert_eq!(*seen.borrow(), 0);
    }
}

#[cfg(test)]
mod n087_tests {
    use super::*;

    /// The test vtable every object gets, and its slot functions (`SLOT + offset`).
    const VTABLE: u32 = 0x0e00_0000;
    const SLOT: u32 = 0x0e10_0000;

    const EXTERNAL: &[u32] = &[
        CONTROLS_OF,
        CONTROL_STATE,
        PLAYER_IS_SLEEPING_OR_RESTING,
        PLAYER_TEST_004D1360,
        PLAYER_UPDATE_HARDCORE_MODE,
        PLAYER_STEP_0094DF80,
        PLAYER_VALUE_005CDE20,
        IS_CONSOLE_VISIBLE,
        ELAPSED_SECONDS,
        OBJECT_STEP_00455490,
        TEXT_TO_TRAIT,
        SET_TRAIT_FROM_INT,
        TILE_POKE,
        MENU_CONSTRUCT,
        MENU_DESTROY,
        FREE,
        INTERFACE_MANAGER,
        MANAGER_ROOT_00586150,
        MENU_CLASS_00705770,
        GET_MENU_BY_CLASS,
        TILE_READ_FILE,
        TILE_GET_MENU,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        TILE_GET_MAXIMUM_DEPTH,
        TILE_GET_CHILD_BY_NAME,
        MENU_TILE_00726070,
        MENU_CHECK_007ABDB0,
        TILE_SET_STRING,
        TILE_SET_FLOAT,
        TILE_GET_FLOAT,
        TILE_IS_TRUE,
        DEBUG_MESSAGE,
        AUDIO_INSTANCE,
        AUDIO_GET_SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_DESTROY,
        SETTING_STRING,
        SETTING_VALUE,
        COPY_00406D30,
        FORMAT_00406D00,
        TOUPPER,
        FLOAT_TO_INT_00406D90,
        FTOL2,
        CALENDAR_GET_HOUR,
        CALENDAR_GET_DATE_STRING,
        CALENDAR_TEXT_00867F10,
        STRING_DESTROY,
        STRING_CHARS,
        OBJECT_TEST_0042CE10,
        PROCESS_LISTS_UPDATE_3D_AFTER_RESTING,
        PROCESS_LISTS_STOP_ALL_ACTORS_IN_DIALOG,
        PROCESS_LISTS_KILL_ALL_PROJECTILES,
        FADER_CREATE,
        FADER_REMOVE,
        INTERFACE_MANAGER_PLAY_MENU_SOUND,
        INTERFACE_PLAY_MENU_SOUND,
        PLAYER_WAIT_005C1A00,
        PLAYER_TEST_005C7870,
        MISC_STAT_INCREMENT,
        STRING_CHARS_00408D60,
        SAVE_00850A40,
    ];

    struct Rig {
        e: Engine,
        menu: u32,
        player: u32,
    }

    fn rig() -> Rig {
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
        let slots: Vec<u32> = (0..0x20).map(|i| SLOT + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(VALUE_6006_DOUBLE, 6006.0f64);
        e.set_global(VALUE_102_DOUBLE, 102.0f64);
        e.set_global(SIXTY_DOUBLE, 60.0f64);
        e.set_global(THIRTEEN_DOUBLE, 13.0f64);
        e.set_global(TWELVE_DOUBLE, 12.0f64);
        e.set_global(BYTE_SCALE_DOUBLE, 255.0f64);
        e.set_global(STEP_DOWN, -2.0f32);
        e.set_global(STEP_UP, 2.0f32);
        let menu = e.mem.alloc(0x4c);
        e.mem.set_u32(menu, VTABLE);
        for index in 0..6u32 {
            e.mem.set_u32(menu + 0x28 + 4 * index, 0x0d00_0000 + index);
        }
        let player = e.mem.alloc(0x10);
        e.set_global(PLAYER, player);
        e.set_global(OPEN_MENU, menu);
        e.set_global(CURRENT_VALUE_TRAIT, 0x0f00u32);
        e.call_log = Some(vec![]);
        Rig { e, menu, player }
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

    #[test]
    fn constructor_clears_the_fields_and_looks_up_the_trait() {
        let mut r = rig();
        let menu = r.e.mem.alloc(0x4c);
        r.e.mem.set_u8(menu + RESTING_ACTIVE, 1);
        r.e.mem.set_u32(menu + PREV_QUANTITY, 5);
        answer(&mut r.e, TEXT_TO_TRAIT, 0x77);
        assert_eq!(r.e.call(0x007b_fb30, &args![menu]).u32(), menu);
        assert_eq!(calls(&r.e, MENU_CONSTRUCT), vec![vec![menu]]);
        assert_eq!(r.e.mem.u32(menu), SLEEP_WAIT_MENU_VTABLE);
        assert_eq!(r.e.mem.u8(menu + RESTING_ACTIVE), 0);
        assert_eq!(r.e.mem.u32(menu + PREV_QUANTITY), 0);
        assert_eq!(calls(&r.e, TEXT_TO_TRAIT), vec![vec![CURRENT_VALUE_TEXT]]);
        assert_eq!(r.e.global::<u32>(CURRENT_VALUE_TRAIT), 0x77);
    }

    #[test]
    fn get_class_answers_0x3f4() {
        let mut r = rig();
        let menu = r.menu;
        assert_eq!(r.e.call(0x007b_fbc0, &args![menu]).u32(), 0x3f4);
    }

    #[test]
    fn destructors_clear_the_open_menu_and_free_with_bit_0() {
        let mut r = rig();
        let menu = r.menu;
        r.e.call(0x007b_fc00, &args![menu]);
        assert_eq!(r.e.global::<u32>(OPEN_MENU), 0);
        assert_eq!(calls(&r.e, MENU_DESTROY), vec![vec![menu]]);
        assert_eq!(r.e.call(0x007b_fbd0, &args![menu, 1u32]).u32(), menu);
        assert_eq!(calls(&r.e, FREE), vec![vec![menu]]);
        r.e.call(0x007b_fbd0, &args![menu, 0u32]);
        assert_eq!(calls(&r.e, FREE).len(), 1);
    }

    #[test]
    fn create_fills_the_texts() {
        let mut r = rig();
        let menu = r.menu;
        r.e.set_global(OPEN_MENU, 0u32);
        answer(&mut r.e, MENU_CLASS_00705770, 0x3f4);
        answer(&mut r.e, TILE_READ_FILE, 0x0c40_0000);
        answer(&mut r.e, TILE_GET_MENU, menu);
        answer(&mut r.e, SLOT + 0x34, 0x3f4);
        answer(&mut r.e, MENU_CHECK_007ABDB0, 1);
        r.e.register(SETTING_STRING, |_, a| a[0].into_ret());
        r.e.register(COPY_00406D30, |e, a| {
            e.mem.set_u8(a[0], b'w');
            Ret::default()
        });
        r.e.register(TOUPPER, |_, a| (a[0] - 0x20).into_ret());
        let alpha = r.e.mem.alloc(4);
        r.e.mem.set_f32(alpha, 0.5);
        answer(&mut r.e, SETTING_VALUE, alpha);
        answer(&mut r.e, TILE_GET_CHILD_BY_NAME, 0x0c41_0000);
        assert_eq!(r.e.call(0x007b_fc30, &args![1u32]).u32(), 0x0c40_0000);
        assert_eq!(r.e.mem.u8(menu + SLEEPING_IN_BED), 1);
        assert_eq!(
            calls(&r.e, COPY_00406D30)[0][1..],
            [0x104, SETTING_011D3948]
        );
        let texts = calls(&r.e, TILE_SET_STRING);
        let tiles: Vec<u32> = texts.iter().map(|w| w[0]).collect();
        assert_eq!(
            tiles,
            vec![
                0x0d00_0000,
                0x0d00_0004,
                0x0d00_0002,
                0x0d00_0003,
                0x0d00_0005
            ]
        );
        let verb = texts[1][2];
        assert_eq!(r.e.mem.u8(verb), b'W');
        assert_eq!(
            calls(&r.e, TILE_SET_FLOAT),
            vec![vec![0x0c41_0000, 0xfa9, 127.5f32.to_bits(), 1]]
        );
        assert_eq!(calls(&r.e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
    }

    #[test]
    fn create_fails_without_the_tiles() {
        let mut r = rig();
        let menu = r.menu;
        answer(&mut r.e, MENU_CLASS_00705770, 0x3f4);
        answer(&mut r.e, TILE_GET_MENU, menu);
        answer(&mut r.e, SLOT + 0x34, 0x3f4);
        assert_eq!(r.e.call(0x007b_fc30, &args![0u32]).u32(), 0);
        assert_eq!(calls(&r.e, DEBUG_MESSAGE), vec![vec![CREATION_FAILED_TEXT]]);
        // Another class: a menu with a tile is deleted.
        answer(&mut r.e, SLOT + 0x34, 0x3f5);
        answer(&mut r.e, MENU_TILE_00726070, 1);
        assert_eq!(r.e.call(0x007b_fc30, &args![0u32]).u32(), 0);
        assert_eq!(calls(&r.e, SLOT), vec![vec![menu, 1]]);
    }

    #[test]
    fn time_text_shows_a_twelve_hour_clock() {
        let mut r = rig();
        let menu = r.menu;
        answer_f64(&mut r.e, CALENDAR_GET_HOUR, 13.5);
        r.e.register(SETTING_STRING, |_, a| a[0].into_ret());
        answer(&mut r.e, STRING_CHARS, 0x0c42_0000);
        answer(&mut r.e, CALENDAR_TEXT_00867F10, 0x0c43_0000);
        r.e.call(0x007c_0000, &args![menu]);
        let format = calls(&r.e, FORMAT_00406D00);
        assert_eq!(
            format[0][1..],
            [
                0x104,
                TIME_FORMAT,
                0x0c43_0000,
                0x0c42_0000,
                1,
                30,
                SETTING_011D27F0
            ]
        );
        assert_eq!(calls(&r.e, STRING_DESTROY).len(), 1);
        assert_eq!(calls(&r.e, TILE_SET_STRING)[0][..2], [0x0d00_0003, 0xfc4]);
        answer_f64(&mut r.e, CALENDAR_GET_HOUR, 0.25);
        r.e.call(0x007c_0000, &args![menu]);
        let format = calls(&r.e, FORMAT_00406D00);
        assert_eq!(format[1][5..], [12, 15, SETTING_011D24FC]);
    }

    #[test]
    fn close_removes_the_fader_in_a_bed() {
        let mut r = rig();
        let menu = r.menu;
        r.e.mem.set_u8(menu + SLEEPING_IN_BED, 1);
        r.e.set_global(FADER_MANAGER, 0x0c44_0000u32);
        answer(&mut r.e, MENU_TILE_00726070, 0x0c45_0000);
        r.e.call(0x007c_01b0, &args![]);
        assert_eq!(calls(&r.e, FADER_REMOVE), vec![vec![0x0c44_0000, 0, 0]]);
        assert_eq!(
            calls(&r.e, SET_TRAIT_FROM_INT),
            vec![vec![0x0c45_0000, 0x1772, 1]]
        );
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT), vec![vec![menu]]);
        r.e.set_global(OPEN_MENU, 0u32);
        r.e.call(0x007c_01b0, &args![]);
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT).len(), 1);
    }

    #[test]
    fn do_click_rests_or_cancels() {
        let mut r = rig();
        let menu = r.menu;
        let player = r.player;
        r.e.mem.set_u8(menu + SLEEPING_IN_BED, 1);
        let seconds = r.e.mem.alloc(4);
        r.e.mem.set_f32(seconds, 3.0);
        answer(&mut r.e, SETTING_VALUE, seconds);
        let text = r.e.mem.alloc(4);
        r.e.mem.set_u8(text, b'x');
        answer(&mut r.e, STRING_CHARS_00408D60, text);
        answer_f64(&mut r.e, TILE_GET_FLOAT, 4.0);
        r.e.call(0x007c_0220, &args![menu, 4u32, 0u32]);
        assert_eq!(
            calls(&r.e, INTERFACE_MANAGER_PLAY_MENU_SOUND),
            vec![vec![1]]
        );
        assert_eq!(calls(&r.e, SAVE_00850A40).len(), 1);
        assert_eq!(calls(&r.e, FADER_CREATE)[0][1..], [0, 3.0f32.to_bits(), 0]);
        assert_eq!(r.e.mem.u8(menu + RESTING_ACTIVE), 1);
        assert_eq!(calls(&r.e, PLAYER_WAIT_005C1A00), vec![vec![player, 5, 1]]);
        assert_eq!(calls(&r.e, MISC_STAT_INCREMENT), vec![vec![0x17]]);
        assert_eq!(
            calls(&r.e, PROCESS_LISTS_STOP_ALL_ACTORS_IN_DIALOG).len(),
            1
        );
        assert_eq!(calls(&r.e, PROCESS_LISTS_KILL_ALL_PROJECTILES).len(), 1);
        r.e.call(0x007c_0220, &args![menu, 5u32, 0u32]);
        assert_eq!(calls(&r.e, PLAYER_WAIT_005C1A00)[1], vec![player, 0, 0]);
        assert_eq!(calls(&r.e, MENU_START_FADE_OUT).len(), 1);
        r.e.call(0x007c_0220, &args![menu, 3u32, 0u32]);
        assert_eq!(calls(&r.e, INTERFACE_MANAGER_PLAY_MENU_SOUND).len(), 2);
    }

    #[test]
    fn do_idle_updates_the_hours_text_on_a_change() {
        let mut r = rig();
        let menu = r.menu;
        answer_f64(&mut r.e, TILE_GET_FLOAT, 3.0);
        answer(&mut r.e, TILE_IS_TRUE, 1);
        r.e.register(SETTING_STRING, |_, a| a[0].into_ret());
        r.e.register(FLOAT_TO_INT_00406D90, |_, a| {
            (f32::from_bits(a[0]) as u32).into_ret()
        });
        r.e.call(0x007c_03a0, &args![menu]);
        assert_eq!(calls(&r.e, INTERFACE_PLAY_MENU_SOUND), vec![vec![4]]);
        assert_eq!(r.e.mem.f32(menu + PREV_QUANTITY), 3.0);
        assert_eq!(
            calls(&r.e, FORMAT_00406D00)[0][1..],
            [0x104, HOURS_FORMAT, 4, SETTING_011D3AE0]
        );
        assert_eq!(calls(&r.e, TILE_SET_STRING)[0][..2], [0x0d00_0002, 0xfc4]);
        // No change: nothing more.
        r.e.call(0x007c_03a0, &args![menu]);
        assert_eq!(calls(&r.e, FORMAT_00406D00).len(), 1);
    }

    #[test]
    fn do_gamepad_steps_the_hours() {
        let mut r = rig();
        let menu = r.menu;
        assert!(!r.e.call(0x007c_04d0, &args![menu, 0xfu32, 0u32]).bool());
        answer(&mut r.e, TILE_IS_TRUE, 1);
        answer_f64(&mut r.e, TILE_GET_FLOAT, 5.0);
        answer(&mut r.e, TEXT_TO_TRAIT, 0x66);
        assert!(r.e.call(0x007c_04d0, &args![menu, 0xfu32, 0u32]).bool());
        assert_eq!(
            calls(&r.e, TILE_SET_FLOAT),
            vec![vec![0x0d00_0001, 0x0f00, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls(&r.e, TILE_POKE),
            vec![vec![0x0d00_0001, 0x66, 1.0f32.to_bits()]]
        );
        assert!(r.e.call(0x007c_04d0, &args![menu, 0x10u32, 0u32]).bool());
        assert_eq!(calls(&r.e, TILE_SET_FLOAT)[1][2], 7.0f32.to_bits());
        assert!(!r.e.call(0x007c_04d0, &args![menu, 0x11u32, 0u32]).bool());
    }
}
