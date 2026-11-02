//! `fallout/interface/menus/hackingmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00700270` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const TEST_TARGET_00700270: u32 = 0x0070_0270;

// Translated from 0076a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (hacking menu, only caller `00769dfe`, which pushes the address
/// of a byte local and tests `AL`): forwards `this` and that pointer to `00700270`, which
/// returns 1 when `007002a0` returns 0 in `AL` and 0 otherwise (`MOVZX; NEG; SBB; ADD 1`). The
/// wrapper returns it as a byte.
pub fn fn_0076a3d0(e: &mut Engine, this: Ptr, byte_address: Ptr) -> bool {
    e.call(TEST_TARGET_00700270, &args![this, byte_address])
        .bool()
}

/// `1280.0` (`double`, `0106e960`).
const SCALE_0106E960: u32 = 0x0106_e960;
/// `004dc1f0` (`renderer.cpp`, no words): the integer at `0118947c` (the screen width
/// the scale is divided by, by its use here; not confirmed).
const SCREEN_SIZE_004DC1F0: u32 = 0x004d_c1f0;

// Translated from 007697f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (hacking menu; `thiscall`, plain `RET`, the result in ST0):
/// the float +0x4ac times 1280.0 (`0106e960`), divided by the integer `004dc1f0`
/// returns, times the float +0x4d0, rounded to a `float`.
pub fn fn_007697f0(e: &mut Engine, this: Ptr) -> f64 {
    let base = this.addr();
    let scale = f64::from_bits(e.mem.u64(SCALE_0106E960));
    let scaled = x87_mul(x87_load32(e.mem.u32(base + 0x4ac)), scale);
    let size = e.call(SCREEN_SIZE_004DC1F0, &args![]).i32();
    // FDIVR double: the stored product over the integer.
    let ratio = x87_div(scaled, f64::from(size));
    let result = x87_store32(x87_mul(ratio, x87_m32(e.mem.u32(base + 0x4d0))));
    x87_load32(result)
}

// ---------------------------------------------------------------------------
// `HackingMenu` (class `0x41f`): the terminal hacking mini-game.
// ---------------------------------------------------------------------------

/// `HackingMenu::pMe` (Xbox PDB static): the menu singleton (the destructor
/// clears it, `Create` stores the menu it read).
const HACKING_MENU: u32 = 0x011d_95b8;
/// `HackingMenu::pim` (Xbox PDB static, `InterfaceManager *`): `Create` stores
/// the interface manager singleton (`004b7210`) here.
const INTERFACE_MANAGER: u32 = 0x011d_95bc;
/// A `u32` static set to the game time plus a delay when the menu closes
/// (`dwRetryTime` in the Xbox PDB's statics, by its use; not confirmed).
const RETRY_TIME: u32 = 0x011d_95c0;
/// The menu class number.
const HACKING_MENU_CLASS: u32 = 0x41f;
/// `HackingMenu`'s vtable.
const HACKING_MENU_VTABLE: u32 = 0x0107_28f4;

layout! {
    /// `HackingMenu` (Xbox PDB), 0x1dc bytes; the PC code uses the PDB's
    /// offsets.
    pub struct HackingMenu: 0x1dc {
        /// `pTerminal` (Xbox PDB): `BGSTerminal *`.
        0x194 pTerminal: u32,
        /// `pTerminalRef` (Xbox PDB): `TESObjectREFR *`.
        0x198 pTerminalRef: u32,
        /// `pPassword` (Xbox PDB): `const char *`.
        0x19c pPassword: u32,
        /// `iNumGuesses` (Xbox PDB).
        0x1a0 iNumGuesses: u32,
        /// `iTotGuesses` (Xbox PDB).
        0x1a4 iTotGuesses: u32,
        /// `iWordsHave` (Xbox PDB).
        0x1a8 iWordsHave: u32,
        /// `iWordLength` (Xbox PDB).
        0x1ac iWordLength: u32,
        /// `iBufferPos` (Xbox PDB): `u32 *`.
        0x1b0 iBufferPos: u32,
        /// `iYStart` (Xbox PDB).
        0x1b4 iYStart: i32,
        /// `iYInterval` (Xbox PDB).
        0x1b8 iYInterval: i32,
        /// `iLastOffset` (Xbox PDB).
        0x1bc iLastOffset: i32,
        /// `pLastTarget` (Xbox PDB): `Tile *`.
        0x1c0 pLastTarget: u32,
        /// `eStage` (Xbox PDB): 0 intro, 1 pre-game, 2 in game, 3 shut-down,
        /// 4 finished.
        0x1c4 eStage: u32,
        /// `pOverTile` (Xbox PDB): `Tile *`.
        0x1c8 pOverTile: u32,
        /// `iCursorX` (Xbox PDB).
        0x1cc iCursorX: i32,
        /// `dwAccessTime` (Xbox PDB).
        0x1d0 dwAccessTime: u32,
        /// `pCursorVis` (Xbox PDB): `Tile::Value *`.
        0x1d4 pCursorVis: u32,
        /// `bSuppressEntry` (Xbox PDB).
        0x1d8 bSuppressEntry: bool,
        /// `bUsedReset` (Xbox PDB).
        0x1d9 bUsedReset: bool,
        /// `bTutorial` (Xbox PDB).
        0x1da bTutorial: bool,
    }
}

/// `IOLines` (Xbox PDB): 46 `TerminalIO *` from +0x28.
const IO_LINES: u32 = 0x28;
const IO_LINE_COUNT: u32 = 0x2e;
/// `xEntryText` (Xbox PDB, `AnimatingText`).
const ENTRY_TEXT: u32 = 0xe0;
/// `pTiles` (Xbox PDB): eight `Tile *` (intro, header, file, log, cursor,
/// entry text, two lock-out tiles).
const TILES: u32 = 0x10c;
const TILE_COUNT: i32 = 8;
/// `Sounds` (Xbox PDB): five `BSSoundHandle`s of 0xc bytes.
const SOUNDS: u32 = 0x12c;
/// `xLogTiles`, `xGuessTiles` (`NiTPointerList<Tile *>`), `wordList`
/// (`BSSimpleList<char *>`) and `xUsedBonuses` (Xbox PDB).
const LOG_TILES: u32 = 0x168;
const GUESS_TILES: u32 = 0x174;
const WORD_LIST: u32 = 0x180;
const USED_BONUSES: u32 = 0x188;

/// `pTiles[i]` of a hacking menu.
fn hacking_tile(e: &Engine, menu: u32, index: u32) -> u32 {
    e.mem.u32(menu.wrapping_add(TILES + 4 * index))
}

/// `Menu::Menu` and the `Menu` destructor body.
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `AnimatingText`'s constructor and destructor bodies (`thiscall`).
const ANIMATING_TEXT_CONSTRUCTOR: u32 = 0x0072_6c90;
const ANIMATING_TEXT_DESTRUCTOR: u32 = 0x0072_6cf0;
/// The `AnimatingText` constructor taking one word (`0076b300` passes its first
/// argument).
const ANIMATING_TEXT_CONSTRUCTOR_WITH_ARGUMENT: u32 = 0x0075_9eb0;
/// `eh vector constructor iterator` (`stdcall`: array, element size, count,
/// constructor, destructor) and `eh vector destructor iterator` (array,
/// element size, count, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// `BSSoundHandle`'s constructor and destructor bodies (the iterator's
/// arguments).
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// `BSSoundHandle` assignment (`this`, source).
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSoundHandle::Stop` and `BSSoundHandle::Release` (Xbox PDB).
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
/// The `NiTPointerList` constructor body (`thiscall`) used for the three lists.
const POINTER_LIST_CONSTRUCTOR: u32 = 0x0048_f200;
/// The `NiTPointerList` destructor body used for the three lists.
const POINTER_LIST_DESTRUCTOR: u32 = 0x004a_1a30;
/// The word list's constructor (`thiscall`).
const WORD_LIST_CONSTRUCTOR: u32 = 0x0068_0890;
/// `NiTPointerListBase<..>::RemoveAll` (Xbox PDB name of the shared body).
const POINTER_LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// Whether a pointer list is empty (`this` only; a byte).
const POINTER_LIST_IS_EMPTY: u32 = 0x0076_b610;
/// Removes a pointer list's head and returns its element (`this` only).
const POINTER_LIST_REMOVE_HEAD: u32 = 0x007b_5390;
/// The simple list's head removal (`this` only).
const SIMPLE_LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The simple list's destructor body (`thiscall`).
const SIMPLE_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// Adds the word at an address to a list (`this`, address of the word).
const LIST_ADD_WORD: u32 = 0x005a_e3d0;
/// `memset` (cdecl: destination, byte, count).
const MEMSET: u32 = 0x0040_3d30;
/// The copy the code uses for `memcpy` (cdecl: destination, source, count).
const MEMCPY: u32 = 0x0040_1460;
/// `strlen` (cdecl, one word).
const STRLEN: u32 = 0x0044_a670;
/// `strcpy_s` (cdecl: destination, size, source).
const STRCPY_S: u32 = 0x0040_6d30;
/// `strchr` (cdecl: string, character).
const STRCHR: u32 = 0x00ec_7690;
/// `isalpha` (cdecl, one word).
const ISALPHA: u32 = 0x00ec_86e1;
/// `operator new` and `operator delete` (cdecl, one word).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vector deleting destructor the grids' row arrays are freed with
/// (`thiscall`, flags).
const ROW_ARRAY_DELETE: u32 = 0x0070_21b0;
/// The grid row's constructor and destructor bodies (the iterator's
/// arguments).
const GRID_ROW_CONSTRUCTOR: u32 = 0x0096_a2d0;
const GRID_ROW_DESTRUCTOR: u32 = 0x0070_2220;
/// The word at +0 of an object (a grid row's `ucData`, a grid's `pData`).
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// The word at +4 of an object (a menu's `pMenu` tile).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// The larger of two unsigned integers (cdecl, two words).
const MAX_OF_TWO: u32 = 0x0040_3940;
/// The smaller of two unsigned integers (cdecl, two words).
const MIN_OF_TWO: u32 = 0x0042_f5a0;
/// A random integer below the argument (cdecl, one word).
const RANDOM_BELOW: u32 = 0x005a_00a0;
/// `BSAudioManager::QInstance` and `BSAudioManager::GetSoundHandleByName`
/// (Xbox PDB; `this`, result, name, flags).
const AUDIO_MANAGER: u32 = 0x00ad_9060;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ae_5680;
/// The game time in milliseconds (no arguments).
const GAME_TIME: u32 = 0x0045_7fe0;
/// A setting's value word (`ECX` = the setting; answers the address of the
/// value).
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// Tile traits (`this` = the tile): float value (ST0), set float (trait,
/// value, flag), set integer (trait, value), is true (trait), string
/// (trait), the value object of a trait (`Tile::GetValue`, `GetValueQ`).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_GET_STRING: u32 = 0x00a0_11f0;
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// A tile's parent (`this` only).
const TILE_PARENT: u32 = 0x0045_cd60;
/// A tile's screen position (`this` only; ST0).
const TILE_SCREEN_X: u32 = 0x00a0_13d0;
/// Sets an integer trait relative to the current value (`thiscall`: trait,
/// value).
const TILE_SET_INT_FROM_VALUE: u32 = 0x0071_5c60;
/// `Menu::RenderTemplate` (Xbox PDB; `this`, parent tile, template name, flag).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// `Menu::StartFadeOut` (Xbox PDB).
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// Connects an `AnimatingText` to a string and a visibility value
/// (`this`, string value, visibility value).
const ANIMATING_TEXT_CONNECT: u32 = 0x006f_fe80;
/// `AnimatingText::ResetBuffer` (Xbox PDB; `this`, flag).
const ANIMATING_TEXT_RESET_BUFFER: u32 = 0x0075_9f20;
/// `_ftol2_sse` (the value in ST0, as a leading `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// Rounds the first float to the nearest multiple of the second (cdecl:
/// value, step; ST0).
const ROUND_TO_MULTIPLE: u32 = 0x004b_d510;
/// The interface manager's gamepad test (`004b71d0`, no arguments).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
/// The interface manager singleton getter (no arguments).
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// The interface manager's mouse queries (`this` = the manager): whether the
/// cursor has a float position (`007079f0`), the integer position
/// (`00747d00`).
const CURSOR_HAS_FLOAT_POSITION: u32 = 0x0070_79f0;
const CURSOR_INT_POSITION: u32 = 0x0074_7d00;
/// The terminal reference of the interface manager (`this` = the manager).
const MANAGER_TERMINAL_REFERENCE: u32 = 0x0070_7af0;
/// `FORenderedTerminal::PlayShutdownSound` and `FORenderedTerminal::FadeOut`
/// (Xbox PDB).
const PLAY_SHUTDOWN_SOUND: u32 = 0x007f_fe40;
const RENDERED_TERMINAL_FADE_OUT: u32 = 0x007f_faf0;
/// The two values `HackingMenu::Close` compares: `0063d040` (`sky.cpp`) on
/// the interface manager's terminal reference and `0056c7f0`
/// (`tesobjectrefr.cpp`) on the menu tile (`thiscall`, no arguments).
const CLOSE_COMPARE_REFERENCE: u32 = 0x0063_d040;
const CLOSE_COMPARE_TILE: u32 = 0x0056_c7f0;
/// `__RTDynamicCast` (cdecl: object, offset, source type, target type, flag).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The two type descriptors of the cast in `HackingMenu::Close`.
const CAST_SOURCE_TYPE: u32 = 0x0119_f43c;
const CAST_TARGET_TYPE: u32 = 0x0119_f418;
/// `SystemColorManager::GetInstance` (no word, plain RET, no stack read) and
/// `SystemColorManager::SetColor` (Xbox PDB; `thiscall`, RET 8: index, colour).
const SYSTEM_COLOR_GET_INSTANCE: u32 = 0x0071_8b60;
const SYSTEM_COLOR_SET_COLOR: u32 = 0x0071_90e0;
/// `HackingMenu::SaveColor` (Xbox PDB static, `NiColorA`).
const SAVE_COLOR: u32 = 0x011d_9674;
/// The setting the destructor adds to the game time for the retry time.
const RETRY_DELAY_SETTING: u32 = 0x011d_0b90;
/// The fade float `Close` sets on the menu tile.
const CLOSE_FADE: u32 = 0x0101_6264;
/// The three settings with the display intervals of the terminal lines.
const LINE_INTERVAL_SETTINGS: [u32; 3] = [0x011d_3dc0, 0x011d_4e4c, 0x011d_3510];
/// `double` divided by a line's interval setting.
const INTERVAL_SCALE: u32 = 0x0101_7b70;
/// The float a line uses when its interval setting is 0 or 1.
const DEFAULT_INTERVAL: u32 = 0x0101_3974;
/// The prompt template and the prompt text of a terminal line.
const PROMPT_TEMPLATE: u32 = 0x0107_293c;
const PROMPT_TEXT: u32 = 0x0106_92d0;
/// `"0x"`, the prefix of a hexadecimal address.
const HEX_PREFIX: u32 = 0x0107_2a50;
/// The opening and the closing brackets of the garbage strings (`char *`
/// statics).
const OPEN_BRACKETS: u32 = 0x011a_0104;
const CLOSE_BRACKETS: u32 = 0x011a_0100;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;

/// Tile traits.
const TRAIT_X: i32 = 0xfa1;
const TRAIT_Y: i32 = 0xfa2;
const TRAIT_VISIBLE: i32 = 0xfa3;
const TRAIT_STRING: i32 = 0xfc4;
const TRAIT_WIDTH: i32 = 0xfb1;

fn me_hacking(e: &Engine) -> u32 {
    e.global::<u32>(HACKING_MENU)
}

/// `_ftol2_sse` on the value in ST0.
fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// Calls the `scalar deleting destructor` (vtable slot 0) of `object` with
/// flag 1 when `object` is not null.
fn delete_object(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

/// The `new[]` size of `count` elements of 8 bytes behind a count word,
/// saturated to `0xffffffff` as the compiler's overflow checks do.
fn row_array_size(count: u32) -> u32 {
    let (bytes, overflow) = count.overflowing_mul(8);
    let bytes = if overflow { u32::MAX } else { bytes };
    let (total, carry) = bytes.overflowing_add(4);
    if carry {
        u32::MAX
    } else {
        total
    }
}

/// Allocates and constructs an array of `count` grid rows (the count word
/// before the rows); answers the rows, or 0 when the allocation fails.
fn new_grid_rows(e: &mut Engine, count: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![row_array_size(count)]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            block + 4,
            8u32,
            count,
            GRID_ROW_CONSTRUCTOR,
            GRID_ROW_DESTRUCTOR
        ],
    );
    block + 4
}

// Translated from 007654f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::HackingMenu` (Xbox PDB): constructs the `Menu` base, stores
/// the class's vtable, constructs the entry text, the five sound handles,
/// the log, guess and bonus lists and the word list, sets `iYStart` 0 and
/// `iYInterval` 30, clears the tiles and the game state (`iLastOffset` -1),
/// takes the five sounds by name (flags `0x31`, then `0x21` four times) and
/// clears the 46 terminal lines. Returns `this`.
// Translation incomplete: 007654f0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_hacking_menu(e: &mut Engine, this: Ptr<HackingMenu>) -> Ptr<HackingMenu> {
    let base = this.addr();
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, HACKING_MENU_VTABLE);
    e.call(ANIMATING_TEXT_CONSTRUCTOR, &args![base + ENTRY_TEXT]);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + SOUNDS,
            0xcu32,
            5u32,
            SOUND_HANDLE_CONSTRUCTOR,
            SOUND_HANDLE_DESTRUCTOR
        ],
    );
    e.call(POINTER_LIST_CONSTRUCTOR, &args![base + LOG_TILES]);
    e.call(POINTER_LIST_CONSTRUCTOR, &args![base + GUESS_TILES]);
    e.call(WORD_LIST_CONSTRUCTOR, &args![base + WORD_LIST]);
    e.call(POINTER_LIST_CONSTRUCTOR, &args![base + USED_BONUSES]);
    e.set(this, HackingMenu::iYStart, 0);
    e.set(this, HackingMenu::iYInterval, 0x1e);
    e.call(MEMSET, &args![base + TILES, 0u32, 0x20u32]);
    e.set(this, HackingMenu::pPassword, 0);
    e.set(this, HackingMenu::iTotGuesses, 0);
    e.set(this, HackingMenu::iNumGuesses, 0);
    e.set(this, HackingMenu::iWordLength, 0);
    e.set(this, HackingMenu::iWordsHave, 0);
    e.set(this, HackingMenu::iCursorX, 0);
    e.set(this, HackingMenu::iBufferPos, 0);
    e.set(this, HackingMenu::eStage, 0);
    e.set(this, HackingMenu::iLastOffset, -1);
    e.set(this, HackingMenu::pOverTile, 0);
    e.set(this, HackingMenu::pLastTarget, 0);
    e.set(this, HackingMenu::bTutorial, false);
    e.set(this, HackingMenu::bUsedReset, false);
    e.set(this, HackingMenu::bSuppressEntry, false);
    e.set(this, HackingMenu::dwAccessTime, 0);
    e.set(this, HackingMenu::pCursorVis, 0);
    let manager = e.call(AUDIO_MANAGER, &args![]).u32();
    for (name, flags, slot) in [
        (0x0107_1fecu32, 0x31u32, 0u32),
        (0x0107_28d8, 0x21, 1),
        (0x0107_28c4, 0x21, 2),
        (0x0107_1fd8, 0x21, 3),
        (0x0107_1fc4, 0x21, 4),
    ] {
        e.with_stack(0xc, |e, handle| {
            let found = e
                .call(
                    GET_SOUND_HANDLE_BY_NAME,
                    &args![manager, handle, name, flags],
                )
                .u32();
            e.call(
                SOUND_HANDLE_ASSIGN,
                &args![base + SOUNDS + 0xc * slot, found],
            );
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
        });
    }
    e.call(MEMSET, &args![base + IO_LINES, 0u32, 0xb8u32]);
    this
}

// Translated from 00765810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word list's destructor (`HackingMenu`'s
/// `wordList`, a `BSSimpleList<char *>`): frees every word (`0076b2a0`) and
/// runs the simple list's destructor body (`0046ffb0`).
// Translation incomplete: 00765810 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_00765810(e: &mut Engine, this: Ptr) {
    fn_0076b2a0(e, this);
    e.call(SIMPLE_LIST_DESTRUCTOR, &args![this]);
}

// Translated from 00765870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::GetClass` (Xbox PDB): the class number `0x41f`.
pub fn hacking_menu_get_class(_e: &mut Engine, _this: Ptr<HackingMenu>) -> u32 {
    HACKING_MENU_CLASS
}

// Translated from 00765880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]`
/// when `0 <= id < 8`.
pub fn hacking_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<HackingMenu>, id: i32, tile: u32) {
    if (0..TILE_COUNT).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(TILES + 4 * id as u32), tile);
    }
}

// Translated from 007658b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn hacking_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HackingMenu>,
    flags: u32,
) -> Ptr<HackingMenu> {
    hacking_menu_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007658e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::~HackingMenu` (Xbox PDB): stores the class's vtable, deletes
/// every tile of the log and guess lists, empties the bonus list, sets the
/// retry time to the game time plus the setting `011d0b90`, frees the words
/// and `iBufferPos`, clears `pMe`, stops and releases the five sounds,
/// deletes the 46 terminal lines, restores the system colour 3 from
/// `SaveColor`, then destroys the members in reverse order and runs the
/// `Menu` destructor.
// Translation incomplete: 007658e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_destructor(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    e.mem.set_u32(base, HACKING_MENU_VTABLE);
    for list in [base + LOG_TILES, base + GUESS_TILES] {
        while !e.call(POINTER_LIST_IS_EMPTY, &args![list]).bool() {
            let tile = e.call(POINTER_LIST_REMOVE_HEAD, &args![list]).u32();
            delete_object(e, tile);
        }
    }
    e.call(POINTER_LIST_REMOVE_ALL, &args![base + USED_BONUSES]);
    let now = e.call(GAME_TIME, &args![]).u32();
    let delay = e.call(SETTING_VALUE, &args![RETRY_DELAY_SETTING]).u32();
    let delay = e.mem.u32(delay);
    e.set_global(RETRY_TIME, now.wrapping_add(delay));
    fn_0076b2a0(e, Ptr::new(base + WORD_LIST));
    let buffer = e.get(this, HackingMenu::iBufferPos);
    e.call(OPERATOR_DELETE, &args![buffer]);
    e.set_global(HACKING_MENU, 0u32);
    for i in 0..5 {
        let sound = base + SOUNDS + 0xc * i;
        e.call(SOUND_HANDLE_STOP, &args![sound]);
        e.call(SOUND_HANDLE_RELEASE, &args![sound]);
    }
    for i in 0..IO_LINE_COUNT {
        let line = e.mem.u32(base + IO_LINES + 4 * i);
        if line != 0 {
            fn_00765b50(e, Ptr::new(line), 1);
        }
    }
    let colors = e.call(SYSTEM_COLOR_GET_INSTANCE, &args![]).u32();
    // 00765aa0: `00718b60` takes no word; the pushed 3 and colour are the two words of
    // `007190e0` (RET 8).
    e.call(SYSTEM_COLOR_SET_COLOR, &args![colors, 3u32, SAVE_COLOR]);
    e.call(POINTER_LIST_DESTRUCTOR, &args![base + USED_BONUSES]);
    fn_00765810(e, Ptr::new(base + WORD_LIST));
    e.call(POINTER_LIST_DESTRUCTOR, &args![base + GUESS_TILES]);
    e.call(POINTER_LIST_DESTRUCTOR, &args![base + LOG_TILES]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![base + SOUNDS, 0xcu32, 5u32, SOUND_HANDLE_DESTRUCTOR],
    );
    e.call(ANIMATING_TEXT_DESTRUCTOR, &args![base + ENTRY_TEXT]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 00765b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `HackingMenu::TerminalIO` scalar deleting destructor: runs the
/// destructor (`0076b560`) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn fn_00765b50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0076b560(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00766a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the `AnimatingText`'s
/// `dwDisplayInterval` (+0x10) to the larger of 1 and `interval`
/// (`00403940`).
pub fn fn_00766a10(e: &mut Engine, this: Ptr, interval: u32) {
    let value = e.call(MAX_OF_TWO, &args![1u32, interval]).u32();
    e.mem.set_u32(this.addr() + 0x10, value);
}

// Translated from 00766a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x41f`, without an object (no `this`).
pub fn fn_00766a40(_e: &mut Engine) -> u32 {
    HACKING_MENU_CLASS
}

// Translated from 00766a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all eight `pTiles` are set; the index checked is written to
/// `*index` as it goes (the first missing one stays there).
pub fn fn_00766a50(e: &mut Engine, this: Ptr<HackingMenu>, index: Ptr) -> bool {
    e.mem.set_i32(index.addr(), 0);
    loop {
        let i = e.mem.i32(index.addr());
        if i >= TILE_COUNT {
            return true;
        }
        if e.mem.u32(
            this.addr()
                .wrapping_add(TILES)
                .wrapping_add((i as u32).wrapping_mul(4)),
        ) == 0
        {
            return false;
        }
        e.mem.set_i32(index.addr(), i.wrapping_add(1));
    }
}

// Translated from 00766aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::Close` (Xbox PDB, no arguments): with a menu, plays the
/// rendered terminal's shut-down sound, sets the menu tile's trait `0xfc1`
/// to the float `01016264`, fades the rendered terminal out (the interface
/// manager's terminal reference cast through `__RTDynamicCast`) when that
/// reference is set and `0063d040` on it equals `0056c7f0` on the menu
/// tile, sets the trait `0x1772` to
/// 1, starts the menu's fade-out and sets the retry time to the game time
/// plus 8000.
pub fn hacking_menu_close(e: &mut Engine) {
    if me_hacking(e) == 0 {
        return;
    }
    e.call(PLAY_SHUTDOWN_SOUND, &args![]);
    let fade = e.global::<u32>(CLOSE_FADE);
    let tile = e.call(WORD_AT_FOUR, &args![me_hacking(e)]).u32();
    e.call(
        TILE_SET_FLOAT,
        &args![tile, 0xfc1u32, x87_quiet_bits(fade), 1u32],
    );
    let manager = e.global::<u32>(INTERFACE_MANAGER);
    if e.call(MANAGER_TERMINAL_REFERENCE, &args![manager]).u32() != 0 {
        let manager = e.global::<u32>(INTERFACE_MANAGER);
        let reference = e.call(MANAGER_TERMINAL_REFERENCE, &args![manager]).u32();
        let left = e.call(CLOSE_COMPARE_REFERENCE, &args![reference]).u32();
        let tile = e.call(WORD_AT_FOUR, &args![me_hacking(e)]).u32();
        let right = e.call(CLOSE_COMPARE_TILE, &args![tile]).u32();
        if left == right {
            let manager = e.global::<u32>(INTERFACE_MANAGER);
            let reference = e.call(MANAGER_TERMINAL_REFERENCE, &args![manager]).u32();
            let terminal = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![reference, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                )
                .u32();
            e.call(RENDERED_TERMINAL_FADE_OUT, &args![terminal]);
        }
    }
    let tile = e.call(WORD_AT_FOUR, &args![me_hacking(e)]).u32();
    e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
    e.call(MENU_START_FADE_OUT, &args![me_hacking(e)]);
    let now = e.call(GAME_TIME, &args![]).u32();
    e.set_global(RETRY_TIME, now.wrapping_add(0x1f40));
}

// Translated from 0076ae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a `HackingMenu::GridRow`, `ucData` and
/// `iSize`): writes the index (as a byte) of every element equal to `value`
/// to `out`, one after another, and answers how many it wrote.
pub fn fn_0076ae30(e: &mut Engine, this: Ptr, value: i32, out: Ptr) -> u8 {
    let mut count = 0u8;
    let mut i = 0u32;
    while i < e.mem.u32(this.addr() + 4) {
        let data = e.mem.u32(this.addr());
        if i32::from(e.mem.u8(data.wrapping_add(i))) == value {
            e.mem
                .set_u8(out.addr().wrapping_add(u32::from(count)), i as u8);
            count = count.wrapping_add(1);
        }
        i += 1;
    }
    count
}

// Translated from 0076ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a `HackingMenu::GridRow`): counts the
/// elements below 32 by value and answers the largest count (as a byte,
/// through `0076af60`).
// Translation incomplete: 0076ae90 - the /GS cookie check (00ec408c) is not translated
pub fn fn_0076ae90(e: &mut Engine, this: Ptr) -> u8 {
    let mut counts = [0u8; 0x20];
    let mut i = 0u32;
    while i < e.mem.u32(this.addr() + 4) {
        let data = e.mem.u32(this.addr());
        let value = e.mem.u8(data.wrapping_add(i));
        if value < 0x20 {
            counts[value as usize] = counts[value as usize].wrapping_add(1);
        }
        i += 1;
    }
    let mut largest = 0u8;
    for count in counts {
        largest = fn_0076af60(e, largest, count);
    }
    largest
}

// Translated from 0076af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): the larger of two bytes.
pub fn fn_0076af60(_e: &mut Engine, a: u8, b: u8) -> u8 {
    if a > b {
        a
    } else {
        b
    }
}

// Translated from 0076af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` = a `HackingMenu::Grid`): constructs in
/// `result` a `size` x `size` grid whose cell (i, j) is this grid's cell
/// (`order[i]`, `order[j]`): the grid is built in a local (`00768810`),
/// copied into `result` (`0076b090`) and destroyed (`007687d0`). Returns
/// `result`.
// Translation incomplete: 0076af90 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_0076af90(e: &mut Engine, this: Ptr, result: Ptr, order: Ptr, size: u8) -> Ptr {
    e.with_stack(8, |e, grid| {
        fn_00768810(e, grid, u32::from(size));
        for i in 0..i32::from(size) {
            for j in 0..i32::from(size) {
                let row_index = e.mem.u8(order.addr().wrapping_add(i as u32));
                let rows = e.mem.u32(this.addr());
                let source_row = rows.wrapping_add(8 * u32::from(row_index));
                let source = e.call(WORD_AT_ZERO, &args![source_row]).u32();
                let column = e.mem.u8(order.addr().wrapping_add(j as u32));
                let target_rows = e.call(WORD_AT_ZERO, &args![grid]).u32();
                let target_row = target_rows.wrapping_add(8 * i as u32);
                let target = e.call(WORD_AT_ZERO, &args![target_row]).u32();
                let value = e.mem.u8(source.wrapping_add(u32::from(column)));
                e.mem.set_u8(target.wrapping_add(j as u32), value);
            }
        }
        fn_0076b090(e, result, grid);
        fn_007687d0(e, grid);
    });
    result
}

// Translated from 0076b090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `HackingMenu::Grid` copy constructor:
/// clears `pData` and assigns `source` (`0076b0c0`). Returns `this`.
pub fn fn_0076b090(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    fn_0076b0c0(e, this, source);
    this
}

// Translated from 0076b0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `HackingMenu::Grid` assignment: takes the
/// source's `iSize`, deletes the old rows (`007021b0`, flags 3), allocates
/// and constructs `iSize` new rows (`new[]` of 8-byte rows behind a count)
/// and assigns each from the source's row (`0076b200`). Returns `this`.
// Translation incomplete: 0076b0c0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_0076b0c0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let base = this.addr();
    let size = e.mem.u32(source.addr() + 4);
    e.mem.set_u32(base + 4, size);
    let old = e.mem.u32(base);
    if old != 0 {
        e.call(ROW_ARRAY_DELETE, &args![old, 3u32]);
    }
    let rows = new_grid_rows(e, size);
    e.mem.set_u32(base, rows);
    let mut i = 0u32;
    while i < e.mem.u32(base + 4) {
        let source_row = e.mem.u32(source.addr()).wrapping_add(8 * i);
        let row = e.mem.u32(base).wrapping_add(8 * i);
        fn_0076b200(e, Ptr::new(row), Ptr::new(source_row));
        i += 1;
    }
    this
}

// Translated from 0076b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `HackingMenu::GridRow` assignment: resizes the
/// row to the source's `iSize` (`00768920`) and copies the bytes. Returns
/// `this`.
pub fn fn_0076b200(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let size = e.mem.u32(source.addr() + 4);
    fn_00768920(e, this, size);
    let count = e.mem.u32(this.addr() + 4);
    let from = e.mem.u32(source.addr());
    let to = e.mem.u32(this.addr());
    e.call(MEMCPY, &args![to, from, count]);
    this
}

// Translated from 0076b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` = the word list): adds a heap copy of
/// `word` (`strlen + 1` bytes, `strcpy_s`) to the list (`005ae3d0`).
pub fn fn_0076b240(e: &mut Engine, this: Ptr, word: u32) {
    let size = e.call(STRLEN, &args![word]).u32().wrapping_add(1);
    let copy = e.call(OPERATOR_NEW, &args![size]).u32();
    e.call(STRCPY_S, &args![copy, size, word]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), copy);
        e.call(LIST_ADD_WORD, &args![this, slot]);
    });
}

// Translated from 0076b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` = the word list): while the list's head
/// holds a word, frees it and removes the head (`0063f7b0`).
pub fn fn_0076b2a0(e: &mut Engine, this: Ptr) {
    while e.mem.u32(this.addr()) != 0 {
        let word = e.mem.u32(this.addr());
        e.call(OPERATOR_DELETE, &args![word]);
        e.call(SIMPLE_LIST_REMOVE_HEAD, &args![this]);
    }
}

// Translated from 0076b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, no arguments): twice the random
/// integer `005a00a0(2)` minus 1 (-1 or 1).
pub fn fn_0076b2e0(e: &mut Engine) -> i32 {
    let random = e.call(RANDOM_BELOW, &args![2u32]).i32();
    random.wrapping_add(random).wrapping_sub(1)
}

// Translated from 0076b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::TerminalIO::TerminalIO` (Xbox PDB): constructs the
/// `AnimatingText` with `text`, clears the two tiles and, with a menu:
/// sets the display interval from the line kind's setting (`kind` 0..2:
/// `011d3dc0`, `011d4e4c`, `011d3510`; a value above 1 gives
/// `004bd510(double 01017b70 / value, 1.0)`, rounded to a whole number; otherwise the float
/// `01013974`, truncated), renders `template` under `parent` as the text
/// tile and connects the animation to its string and visibility values.
/// A kind-1 line also renders the prompt template under the intro tile and
/// sets its text. For `y >= 0` the tiles are placed at `y` and at the menu
/// tile's trait `0x1007` (the text tile after the prompt's width when there
/// is a prompt). Returns `this`.
// Translation incomplete: 0076b300 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_terminal_io_terminal_io(
    e: &mut Engine,
    this: Ptr,
    text: u32,
    parent: u32,
    template: u32,
    y: i32,
    kind: u8,
) -> Ptr {
    let base = this.addr();
    e.call(ANIMATING_TEXT_CONSTRUCTOR_WITH_ARGUMENT, &args![this, text]);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x2c, 0);
    if me_hacking(e) == 0 {
        return this;
    }
    // The three setting values sit in a local array indexed by `kind` (the
    // callers pass 0, 1 or 2; a larger kind reads past the array).
    let value = e.with_stack(0xc, |e, values| {
        for (i, setting) in LINE_INTERVAL_SETTINGS.into_iter().enumerate() {
            let word = e.call(SETTING_VALUE, &args![setting]).u32();
            let word = e.mem.u32(word);
            e.mem.set_u32(values.addr() + 4 * i as u32, word);
        }
        e.mem.u32(values.addr().wrapping_add(4 * u32::from(kind)))
    });
    let interval_bits = if value > 1 {
        let scale = e.mem.f64(INTERVAL_SCALE);
        let ratio = x87_store32(x87_div(scale, f64::from(value)));
        let clamped = e
            .call(ROUND_TO_MULTIPLE, &args![ratio, 1.0f32.to_bits()])
            .f64();
        x87_store32(clamped)
    } else {
        x87_quiet_bits(e.mem.u32(DEFAULT_INTERVAL))
    };
    let interval = x87_fistp64_chop(X87Ext::load32(interval_bits)) as u32;
    fn_00766a10(e, this, interval);
    let menu = me_hacking(e);
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, parent, template, 0u32])
        .u32();
    e.mem.set_u32(base + 0x2c, tile);
    if tile == 0 {
        return this;
    }
    let visible = e.call(TILE_GET_VALUE_Q, &args![tile, TRAIT_VISIBLE]).u32();
    let string = e.call(TILE_GET_VALUE, &args![tile, TRAIT_STRING]).u32();
    e.call(ANIMATING_TEXT_CONNECT, &args![this, string, visible]);
    if kind == 1 {
        let menu = me_hacking(e);
        let intro = hacking_tile(e, menu, 0);
        let prompt = e
            .call(
                MENU_RENDER_TEMPLATE,
                &args![menu, intro, PROMPT_TEMPLATE, 0u32],
            )
            .u32();
        e.mem.set_u32(base + 0x30, prompt);
        e.call(
            TILE_SET_STRING,
            &args![prompt, TRAIT_STRING, PROMPT_TEXT, 1u32],
        );
    }
    if y >= 0 {
        let tile = e.mem.u32(base + 0x2c);
        e.call(TILE_SET_INT, &args![tile, TRAIT_Y, y]);
        let menu_tile = e.call(WORD_AT_FOUR, &args![me_hacking(e)]).u32();
        let left = e.call(TILE_GET_FLOAT, &args![menu_tile, 0x1007u32]).f64();
        let x = ftol(e, left);
        let prompt = e.mem.u32(base + 0x30);
        if prompt != 0 {
            e.call(TILE_SET_INT, &args![prompt, TRAIT_X, x]);
            let prompt = e.mem.u32(base + 0x30);
            e.call(TILE_SET_INT, &args![prompt, TRAIT_Y, y]);
            let width = e.call(STRLEN, &args![PROMPT_TEXT]).i32();
            let tile = e.mem.u32(base + 0x2c);
            e.call(
                TILE_SET_INT_FROM_VALUE,
                &args![tile, TRAIT_X, width.wrapping_mul(0x11).wrapping_add(x)],
            );
        } else {
            let tile = e.mem.u32(base + 0x2c);
            e.call(TILE_SET_INT, &args![tile, TRAIT_X, x]);
        }
    }
    this
}

// Translated from 0076b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `HackingMenu::TerminalIO` destructor: deletes the text tile (+0x2c)
/// and the prompt tile (+0x30) and runs the `AnimatingText` destructor.
// Translation incomplete: 0076b560 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_0076b560(e: &mut Engine, this: Ptr) {
    let text_tile = e.mem.u32(this.addr() + 0x2c);
    delete_object(e, text_tile);
    let prompt_tile = e.mem.u32(this.addr() + 0x30);
    delete_object(e, prompt_tile);
    e.call(ANIMATING_TEXT_DESTRUCTOR, &args![this]);
}

// Translated from 00768810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `HackingMenu::Grid` constructor: `size`
/// rows (`new[]` behind a count), each sized to `size` (`00768920`).
/// Returns `this`.
// Translation incomplete: 00768810 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_00768810(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 4, size);
    let rows = new_grid_rows(e, size);
    e.mem.set_u32(base, rows);
    let mut i = 0u32;
    while i < e.mem.u32(base + 4) {
        let size = e.mem.u32(base + 4);
        let row = e.mem.u32(base).wrapping_add(8 * i);
        fn_00768920(e, Ptr::new(row), size);
        i += 1;
    }
    this
}

// Translated from 00768920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: resizes a `HackingMenu::GridRow`: frees the
/// data, sets `iSize` and allocates `size` new bytes.
pub fn fn_00768920(e: &mut Engine, this: Ptr, size: u32) {
    let data = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![data]);
    e.mem.set_u32(this.addr() + 4, size);
    let data = e.call(OPERATOR_NEW, &args![size]).u32();
    e.mem.set_u32(this.addr(), data);
}

// Translated from 00768970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` unused): writes `value` as `"0x"` and
/// four upper-case hexadecimal digits into `buffer` (seven bytes with the
/// terminator).
pub fn fn_00768970(e: &mut Engine, _this: Ptr, value: u16, buffer: Ptr) {
    e.call(MEMCPY, &args![buffer, HEX_PREFIX, 2u32]);
    for i in 0..4u32 {
        let digit = ((u32::from(value) & (0xf << (4 * i))) >> (4 * i)) as u8;
        let character = if digit < 10 {
            digit + 0x30
        } else {
            digit + 0x37
        };
        e.mem.set_u8(buffer.addr() + 5 - i, character);
    }
    e.mem.set_u8(buffer.addr() + 6, 0);
}

// Translated from 007687b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `0083fd60(this, 0076b2e0)` (the list
/// operation given the random sign function `0076b2e0`).
pub fn fn_007687b0(e: &mut Engine, this: Ptr) {
    e.call(0x0083_fd60, &args![this, 0x0076_b2e0u32]);
}

// Translated from 007687d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `HackingMenu::Grid` destructor: deletes
/// the rows (`007021b0`, flags 3) when there are any.
pub fn fn_007687d0(e: &mut Engine, this: Ptr) {
    let rows = e.mem.u32(this.addr());
    if rows != 0 {
        e.call(ROW_ARRAY_DELETE, &args![rows, 3u32]);
    }
}

// Translated from 00768a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::CommonPositions` (Xbox PDB; `this` unused): how many of the
/// first `min(strlen(a), strlen(b))` characters of the two words are the
/// same; 0 when either is null.
pub fn hacking_menu_common_positions(e: &mut Engine, _this: Ptr, a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    let mut same = 0u32;
    let length_b = e.call(STRLEN, &args![b]).u32();
    let length_a = e.call(STRLEN, &args![a]).u32();
    let length = e.call(MIN_OF_TWO, &args![length_a, length_b]).u32();
    for i in 0..length {
        if e.mem.i8(a.wrapping_add(i)) == e.mem.i8(b.wrapping_add(i)) {
            same = same.wrapping_add(1);
        }
    }
    same
}

// Translated from 007694f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default `HackingMenu::TerminalIO`
/// constructor: the `AnimatingText` and two cleared tiles. Returns `this`.
pub fn fn_007694f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ANIMATING_TEXT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr() + 0x30, 0);
    e.mem.set_u32(this.addr() + 0x2c, 0);
    this
}

// Translated from 00769700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::GetCharHit` (Xbox PDB): the character column under the
/// cursor in the entry tile (`pim` + 0xcc): the cursor x (`iCursorX` with a
/// gamepad, else the manager's float position `007697f0` or its integer
/// position, as a `float`), relative to the tile's screen x, in steps of 17;
/// -1 left of the tile or right of its width (`0xfb1`) less 1.
pub fn hacking_menu_get_char_hit(e: &mut Engine, this: Ptr<HackingMenu>) -> i32 {
    e.call(INTERFACE_SINGLETON, &args![]);
    let cursor = if e.call(GAMEPAD_IN_USE, &args![]).bool() {
        e.get(this, HackingMenu::iCursorX)
    } else {
        let manager = e.global::<u32>(INTERFACE_MANAGER);
        let position = if e.call(CURSOR_HAS_FLOAT_POSITION, &args![manager]).bool() {
            let manager = e.global::<u32>(INTERFACE_MANAGER);
            x87_store32(fn_007697f0(e, Ptr::new(manager)))
        } else {
            let manager = e.global::<u32>(INTERFACE_MANAGER);
            let position = e.call(CURSOR_INT_POSITION, &args![manager]).i32();
            x87_store32(f64::from(position))
        };
        ftol(e, x87_load32(position))
    };
    let tile = e
        .mem
        .u32(e.global::<u32>(INTERFACE_MANAGER).wrapping_add(0xcc));
    let screen_x = e.call(TILE_SCREEN_X, &args![tile]).f64();
    let left = ftol(e, screen_x);
    let tile = e
        .mem
        .u32(e.global::<u32>(INTERFACE_MANAGER).wrapping_add(0xcc));
    let width = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_WIDTH]).f64();
    let one = e.mem.f64(ONE_DOUBLE);
    let right = ftol(e, x87_sub(x87_add(width, f64::from(left)), one));
    if left > cursor || cursor > right {
        -1
    } else {
        cursor.wrapping_sub(left) / 0x11
    }
}

// Translated from 0076a3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether `value` differs from the word at +0
/// of `this` (`00559450`).
pub fn fn_0076a3b0(e: &mut Engine, this: Ptr, value: u32) -> bool {
    value != e.call(WORD_AT_ZERO, &args![this]).u32()
}

// Translated from 0076a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: once the game time is past `dwAccessTime`,
/// goes back to the computers menu (`HackingMenu::TransitionToComputersMenu`).
pub fn fn_0076a510(e: &mut Engine, this: Ptr<HackingMenu>) {
    let now = e.call(GAME_TIME, &args![]).u32();
    if now > e.get(this, HackingMenu::dwAccessTime) {
        hacking_menu_transition_to_computers_menu(e, this);
    }
}

// Translated from 0076a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::GetGarbageString` (Xbox PDB; `this` unused): copies the
/// garbage run that starts at `source` into `destination` (not
/// terminated): one character, or, when it is an opening bracket (in the
/// string at `011a0104`), up to and including the matching closing bracket
/// (`011a0100`, same position); a letter or the end of the string before
/// the closing bracket makes it one character again.
pub fn hacking_menu_get_garbage_string(e: &mut Engine, _this: Ptr, destination: u32, source: u32) {
    let mut length = 1u32;
    let open = e.global::<u32>(OPEN_BRACKETS);
    let first = i32::from(e.mem.i8(source));
    let found = e.call(STRCHR, &args![open, first]).u32();
    if found != 0 {
        loop {
            let character = e.mem.i8(source.wrapping_add(length));
            if character == 0 || e.call(ISALPHA, &args![i32::from(character)]).i32() != 0 {
                length = 1;
                break;
            }
            let index = found.wrapping_sub(e.global::<u32>(OPEN_BRACKETS));
            let close = e.global::<u32>(CLOSE_BRACKETS);
            if character == e.mem.i8(close.wrapping_add(index)) {
                length += 1;
                break;
            }
            length += 1;
        }
    }
    e.call(MEMCPY, &args![destination, source, length]);
}

// Translated from 0076ab40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::UpdateCursorPos` (Xbox PDB): puts the cursor tile after the
/// entry text: x = 17 per character of the entry tile's string (when it is
/// visible) plus the x of the entry tile and each parent up to the cursor
/// tile's parent.
pub fn hacking_menu_update_cursor_pos(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let mut length = 0i32;
    let mut x = 0i32;
    let entry = hacking_tile(e, base, 5);
    if e.call(TILE_IS_TRUE, &args![entry, TRAIT_VISIBLE]).bool() {
        let entry = hacking_tile(e, base, 5);
        let text = e.call(TILE_GET_STRING, &args![entry, TRAIT_STRING]).u32();
        if text != 0 {
            length = e.call(STRLEN, &args![text]).i32();
        }
    }
    let mut tile = hacking_tile(e, base, 5);
    while tile != 0 {
        let cursor = hacking_tile(e, base, 4);
        if tile == e.call(TILE_PARENT, &args![cursor]).u32() {
            break;
        }
        let value = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_X]).f64();
        x = ftol(e, value).wrapping_add(x);
        tile = e.call(TILE_PARENT, &args![tile]).u32();
    }
    let cursor = hacking_tile(e, base, 4);
    e.call(
        TILE_SET_INT,
        &args![cursor, TRAIT_X, length.wrapping_mul(0x11).wrapping_add(x)],
    );
}

// Translated from 00767b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::DoGamepadUpEvent` (Xbox PDB): button `0xa` finishes the menu
/// (`eStage` 4) and closes it, answering true; other buttons answer false.
pub fn hacking_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr<HackingMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    if button == 0xa {
        e.set(this, HackingMenu::eStage, 4);
        hacking_menu_close(e);
        true
    } else {
        false
    }
}

// Translated from 00767ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::DoLeave` (Xbox PDB): clears the entry
/// (`HackingMenu::ClearEntry`), `bSuppressEntry` and `pLastTarget`.
pub fn hacking_menu_do_leave(
    e: &mut Engine,
    this: Ptr<HackingMenu>,
    _unused_1: u32,
    _unused_2: u32,
) {
    hacking_menu_clear_entry(e, this);
    e.set(this, HackingMenu::bSuppressEntry, false);
    e.set(this, HackingMenu::pLastTarget, 0);
}

// Translated from 00768020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::ClearEntry` (Xbox PDB): resets the entry text's buffer
/// (`AnimatingText::ResetBuffer(0)`) and sets the traits `0x1005` and
/// `0x1008` of the file tile (`pTiles[2]`) to -1.
pub fn hacking_menu_clear_entry(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    e.call(ANIMATING_TEXT_RESET_BUFFER, &args![base + ENTRY_TEXT, 0u32]);
    let entry = hacking_tile(e, base, 2);
    e.call(TILE_SET_INT, &args![entry, 0x1005u32, -1i32]);
    let entry = hacking_tile(e, base, 2);
    e.call(TILE_SET_INT, &args![entry, 0x1008u32, -1i32]);
}

/// `ComputersMenu::Create` (Xbox PDB, cdecl: the terminal reference).
const COMPUTERS_MENU_CREATE: u32 = 0x0075_7b70;
/// `Tile::GetMenu` and `Tile::UpdateAll` (Xbox PDB; the latter cdecl, one
/// word).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_UPDATE_ALL: u32 = 0x00a0_4200;
/// Sets the byte at +0x1c of `this` to 1 (`bRenderedMenu` on a menu).
const SET_BYTE_AT_0X1C: u32 = 0x0086_7a20;
/// `Interface::GetCurrentRenderedMenu` (Xbox PDB, no arguments).
const GET_CURRENT_RENDERED_MENU: u32 = 0x0070_7ad0;
/// The print function a missing rendered menu is reported with (cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The message printed when there is no rendered menu.
const NO_RENDERED_MENU_MESSAGE: u32 = 0x0107_2b00;
/// `FORenderedMenu::FORenderedMenu` (Xbox PDB).
const RENDERED_MENU_CONSTRUCTOR: u32 = 0x0070_5fe0;
/// The rendered menu's two set-up calls (`thiscall`, one word each): the
/// computers menu tile (`00984f60`) and `0056c7f0` on that tile
/// (`00705fc0`).
const RENDERED_MENU_SET_TILE: u32 = 0x0098_4f60;
const RENDERED_MENU_SET_FROM_TILE: u32 = 0x0070_5fc0;
/// `BSSoundHandle::Play` (Xbox PDB; `this`, flag).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// A float of a tile (`this` only; ST0) that `DoShutdown` adds to the trait
/// `0xfb0`.
const TILE_BOTTOM_00A01440: u32 = 0x00a0_1440;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

// Translated from 0076a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::DoShutdown` (Xbox PDB): once the game time is past
/// `dwAccessTime`, scrolls the menu tile's trait `0x1005` up by `iYInterval`
/// times the steps elapsed (one step per 50 ms since `dwAccessTime`, plus
/// 1; 1 when it is 0), sets `dwAccessTime` to the time plus 50 and plays
/// sound 3. Then, when the entry tile's float `00a01440` plus its trait
/// `0xfb0` is below 0.0, sets the menu tile's trait `0x1006` and finishes
/// the menu (`eStage` 4).
pub fn hacking_menu_do_shutdown(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let now = e.call(GAME_TIME, &args![]).u32();
    if now > e.get(this, HackingMenu::dwAccessTime) {
        let steps = if e.get(this, HackingMenu::dwAccessTime) != 0 {
            let now = e.call(GAME_TIME, &args![]).u32();
            now.wrapping_sub(e.get(this, HackingMenu::dwAccessTime)) / 0x32 + 1
        } else {
            1
        };
        let menu_tile = e.mem.u32(base + 4);
        // The flag pushed before the getter is the setter's last argument.
        let current = e.call(TILE_GET_FLOAT, &args![menu_tile, 0x1005u32]).f64();
        let distance = (e.get(this, HackingMenu::iYInterval) as u32).wrapping_mul(steps);
        let value = x87_store32(x87_add(current, f64::from(distance)));
        let menu_tile = e.mem.u32(base + 4);
        e.call(TILE_SET_FLOAT, &args![menu_tile, 0x1005u32, value, 1u32]);
        let now = e.call(GAME_TIME, &args![]).u32();
        e.set(this, HackingMenu::dwAccessTime, now.wrapping_add(0x32));
        e.call(SOUND_HANDLE_PLAY, &args![base + SOUNDS + 0x24, 0u32]);
    }
    let entry = hacking_tile(e, base, 5);
    let position = e.call(TILE_BOTTOM_00A01440, &args![entry]).f64();
    let entry = hacking_tile(e, base, 5);
    let height = e.call(TILE_GET_FLOAT, &args![entry, 0xfb0u32]).f64();
    let total = x87_add(height, position);
    // FCOMP; TEST AH,5; JP: taken unless only C0 is set (below, ordered).
    if total < e.mem.f64(ZERO_DOUBLE) {
        let menu_tile = e.mem.u32(base + 4);
        e.call(TILE_SET_INT, &args![menu_tile, 0x1006u32, 1u32]);
        e.set(this, HackingMenu::eStage, 4);
    }
}

// Translated from 0076a540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::TransitionToComputersMenu` (Xbox PDB): sets the menu
/// tile's trait `0x1772`, starts the fade-out and, with a terminal, opens
/// the computers menu on the terminal reference, marks it rendered,
/// updates all tiles and hands it to the current rendered menu (cast
/// through `__RTDynamicCast`; without one, prints a message and makes a
/// new `FORenderedMenu`, 0xf0 bytes).
// Translation incomplete: 0076a540 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_transition_to_computers_menu(e: &mut Engine, this: Ptr<HackingMenu>) {
    let menu_tile = e.mem.u32(this.addr() + 4);
    e.call(TILE_SET_INT, &args![menu_tile, 0x1772u32, 1u32]);
    e.call(MENU_START_FADE_OUT, &args![this]);
    if e.get(this, HackingMenu::pTerminal) == 0 {
        return;
    }
    let reference = e.get(this, HackingMenu::pTerminalRef);
    let tile = e.call(COMPUTERS_MENU_CREATE, &args![reference]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(SET_BYTE_AT_0X1C, &args![menu]);
    e.call(TILE_UPDATE_ALL, &args![0u32]);
    let current = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    let mut rendered = e
        .call(
            RT_DYNAMIC_CAST,
            &args![current, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .u32();
    if rendered == 0 {
        e.call(PRINT_MESSAGE, &args![NO_RENDERED_MENU_MESSAGE]);
        let block = e.call(OPERATOR_NEW, &args![0xf0u32]).u32();
        rendered = if block != 0 {
            e.call(RENDERED_MENU_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
    }
    e.call(RENDERED_MENU_SET_TILE, &args![rendered, tile]);
    let value = e.call(CLOSE_COMPARE_TILE, &args![tile]).u32();
    e.call(RENDERED_MENU_SET_FROM_TILE, &args![rendered, value]);
}

/// `FlashCursor`'s function-local statics: the flash period (`float`), its
/// guard bit (bit 0 of the word) and the time of the next flash.
const FLASH_PERIOD: u32 = 0x011d_96a4;
const FLASH_PERIOD_INITIALIZED: u32 = 0x011d_96a8;
const NEXT_FLASH_TIME: u32 = 0x011d_96a0;
/// The user trait name the flash period is read with.
const FLASH_PERIOD_TRAIT_NAME: u32 = 0x0107_2108;
/// `Tile::AddUserTrait` (Xbox PDB, cdecl: name, value).
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::Value::SetFloat` (Xbox PDB; `this`, value, flag).
const TILE_VALUE_SET_FLOAT: u32 = 0x00a0_a270;
/// The guess block template and the format of its label.
const GUESS_BLOCK_TEMPLATE: u32 = 0x0107_2b90;
const GUESS_BLOCK_FORMAT: u32 = 0x0107_2b88;
/// A pointer list's element count (`this` only) and its tail insertion
/// (`this`, address of the element).
const POINTER_LIST_COUNT: u32 = 0x0044_ddc0;
const POINTER_LIST_ADD_TAIL: u32 = 0x004e_d8c0;
/// `BSStringT<char>`'s constructor, destructor and `sprintf` (cdecl:
/// string, format, words).
const STRING_CONSTRUCTOR: u32 = 0x0040_37b0;
const STRING_DESTRUCTOR: u32 = 0x0040_37d0;
const STRING_SPRINTF: u32 = 0x0040_6f60;
/// A tile call that answers the word the guess block's label is formatted with
/// (`thiscall`, no stack word: the pointer at +0x20).
const TILE_VALUE_00700300: u32 = 0x0070_0300;
/// Sets a tile's text (`thiscall`: text).
const TILE_SET_TEXT: u32 = 0x0070_7e10;

// Translated from 0076a9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::FlashCursor` (Xbox PDB): on its first call reads the flash
/// period (a `static float`) from the cursor tile's user trait
/// (`01072108`). Does nothing once the menu is finished with the menu
/// tile's trait `0x1006` true. Once the game time is past the next flash
/// time and the cursor's visibility value is known, toggles it (1.0 -> 0.0,
/// anything else -> 1.0) and sets the next flash time to the time plus the
/// period (truncated); without the value, it looks it up (`0xfa3` of the
/// cursor tile).
// Translation incomplete: 0076a9d0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_flash_cursor(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let guard = e.global::<u32>(FLASH_PERIOD_INITIALIZED);
    if guard & 1 == 0 {
        e.set_global(FLASH_PERIOD_INITIALIZED, guard | 1);
        let trait_id = e
            .call(TILE_ADD_USER_TRAIT, &args![FLASH_PERIOD_TRAIT_NAME, -1i32])
            .u32();
        let cursor = hacking_tile(e, base, 4);
        let period = e.call(TILE_GET_FLOAT, &args![cursor, trait_id]).f64();
        e.set_global(FLASH_PERIOD, x87_store32(period));
    }
    if e.get(this, HackingMenu::eStage) == 4 {
        let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
        if e.call(TILE_IS_TRUE, &args![menu_tile, 0x1006u32]).bool() {
            return;
        }
    }
    let now = e.call(GAME_TIME, &args![]).u32();
    if now > e.global::<u32>(NEXT_FLASH_TIME) && e.get(this, HackingMenu::pCursorVis) != 0 {
        let value = e.get(this, HackingMenu::pCursorVis);
        let current = x87_load32(e.mem.u32(value + 8));
        // FCOMP; TEST AH,0x44; JP: 0.0 only when equal (C3 alone).
        let next = if current == e.mem.f64(ONE_DOUBLE) {
            0.0f32
        } else {
            1.0f32
        };
        let value = e.get(this, HackingMenu::pCursorVis);
        e.call(TILE_VALUE_SET_FLOAT, &args![value, next.to_bits(), 1u32]);
        let now = e.call(GAME_TIME, &args![]).u32();
        let period = x87_m32(e.global::<u32>(FLASH_PERIOD));
        let next_time = x87_fistp64_chop(X87Ext::load64(x87_add(f64::from(now), period)));
        e.set_global(NEXT_FLASH_TIME, next_time as u32);
    } else if e.get(this, HackingMenu::pCursorVis) == 0 {
        let cursor = hacking_tile(e, base, 4);
        let value = e
            .call(TILE_GET_VALUE_Q, &args![cursor, TRAIT_VISIBLE])
            .u32();
        e.set(this, HackingMenu::pCursorVis, value);
    }
}

// Translated from 0076ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::UpdateScrollingCursor` (Xbox PDB): puts the cursor tile at
/// the end of the line's text tile: its x plus 17 per character (none when
/// the text is null or starts with a space) and its y, plus the x and y of
/// each parent up to the cursor tile's parent. Nothing without a line or
/// text tile.
pub fn hacking_menu_update_scrolling_cursor(e: &mut Engine, this: Ptr<HackingMenu>, line: Ptr) {
    let base = this.addr();
    if line.addr() == 0 || e.mem.u32(line.addr() + 0x2c) == 0 {
        return;
    }
    let text_tile = e.mem.u32(line.addr() + 0x2c);
    let text = e
        .call(TILE_GET_STRING, &args![text_tile, TRAIT_STRING])
        .u32();
    let length = if text != 0 && e.mem.i8(text) != 0x20 {
        e.call(STRLEN, &args![text]).i32()
    } else {
        0
    };
    let text_tile = e.mem.u32(line.addr() + 0x2c);
    let left = e.call(TILE_GET_FLOAT, &args![text_tile, TRAIT_X]).f64();
    let mut x = ftol(e, x87_add(left, f64::from(length.wrapping_mul(0x11))));
    let text_tile = e.mem.u32(line.addr() + 0x2c);
    let top = e.call(TILE_GET_FLOAT, &args![text_tile, TRAIT_Y]).f64();
    let mut y = ftol(e, top);
    let text_tile = e.mem.u32(line.addr() + 0x2c);
    let mut tile = e.call(TILE_PARENT, &args![text_tile]).u32();
    while tile != 0 {
        let cursor = hacking_tile(e, base, 4);
        if tile == e.call(TILE_PARENT, &args![cursor]).u32() {
            break;
        }
        let value = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_X]).f64();
        x = ftol(e, value).wrapping_add(x);
        let value = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_Y]).f64();
        y = ftol(e, value).wrapping_add(y);
        tile = e.call(TILE_PARENT, &args![tile]).u32();
    }
    let cursor = hacking_tile(e, base, 4);
    e.call(TILE_SET_INT, &args![cursor, TRAIT_X, x]);
    let cursor = hacking_tile(e, base, 4);
    e.call(TILE_SET_INT, &args![cursor, TRAIT_Y, y]);
}

// Translated from 0076ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::MakeGuessBlock` (Xbox PDB): renders the guess block
/// template (`01072b90`) under the header tile (through `pMe`), sets its
/// trait `0xfac` relative to the number of guess tiles, appends it to the
/// guess tiles and sets its text to the label formatted (`01072b88`) with
/// `00700300(count)` of the tile.
// Translation incomplete: 0076ad50 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_make_guess_block(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let header = hacking_tile(e, base, 1);
    let menu = me_hacking(e);
    let tile = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu, header, GUESS_BLOCK_TEMPLATE, 0u32],
        )
        .u32();
    let count = e.call(POINTER_LIST_COUNT, &args![base + GUESS_TILES]).u32();
    e.call(TILE_SET_INT_FROM_VALUE, &args![tile, 0xfacu32, count]);
    e.with_stack(8, |e, slot| {
        e.mem.set_u32(slot.addr(), tile);
        e.call(POINTER_LIST_ADD_TAIL, &args![base + GUESS_TILES, slot]);
    });
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCTOR, &args![text]);
        let count = e.call(POINTER_LIST_COUNT, &args![base + GUESS_TILES]).u32();
        // 0076ade6: `00700300` takes no stack word (plain RET); the pushed count is
        // the last word of the cdecl sprintf (ADD ESP,0x10; the format at 01072b88
        // is "%s_%d").
        let value = e.call(TILE_VALUE_00700300, &args![tile]).u32();
        e.call(
            STRING_SPRINTF,
            &args![text, GUESS_BLOCK_FORMAT, value, count],
        );
        let characters = e.call(WORD_AT_ZERO, &args![text]).u32();
        e.call(TILE_SET_TEXT, &args![tile, characters]);
        e.call(STRING_DESTRUCTOR, &args![text]);
    });
}

/// `ListBox`-style iteration of a pointer list: answers the address of the
/// current element and advances the node at the address passed
/// (`thiscall`: list, address of the node).
const POINTER_LIST_NEXT: u32 = 0x0057_cbe0;
/// The template of a log line.
const LOG_LINE_TEMPLATE: u32 = 0x0107_29c8;
/// `Interface::IsTopMenuID` (Xbox PDB, cdecl: class).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// The tutorial manager's test whether message `id` is showing (`this` =
/// interface manager + 0x4d4; id).
const TUTORIAL_IS_SHOWING: u32 = 0x0071_8840;
/// `AnimatingText::Update` (Xbox PDB).
const ANIMATING_TEXT_UPDATE: u32 = 0x006f_fea0;
/// Whether an `AnimatingText` has shown all its text (`this` only; a byte).
const ANIMATING_TEXT_FINISHED: u32 = 0x0075_9d80;
/// `BSSoundHandle::PlayAfter` (Xbox PDB; `this`, delay, flag).
const SOUND_HANDLE_PLAY_AFTER: u32 = 0x00ad_8870;
/// The typing sound `DoIntro` plays per character (flags `0x121`).
const TYPING_SOUND: u32 = 0x0107_2130;
/// `DoIntro`'s function-local statics: the time of the next step, the
/// per-character sound spacing and its guard bit.
const INTRO_NEXT_TIME: u32 = 0x011d_969c;
const TYPING_SPACING: u32 = 0x011d_9694;
const TYPING_SPACING_INITIALIZED: u32 = 0x011d_9698;

// Translated from 00767bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::FindTile` (Xbox PDB): the text tile of the first terminal
/// line whose trait `0xfaf` is true and whose traits `0xfac` and `0x1004`
/// equal `line_index` (0..16) and `column` (0 or 1); 0 when there is none. (A null line's tile is
/// read at +0x2c as the exe reads it.)
pub fn hacking_menu_find_tile(
    e: &mut Engine,
    this: Ptr<HackingMenu>,
    line_index: u32,
    column: u32,
) -> u32 {
    let base = this.addr();
    for i in 0..IO_LINE_COUNT {
        let line = e.mem.u32(base + IO_LINES + 4 * i);
        let tile = e.mem.u32(line.wrapping_add(0x2c));
        if tile == 0 || !e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool() {
            continue;
        }
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
        // FILD; FUCOMPP; TEST AH,0x44; JP: on only when equal.
        if f64::from(line_index) != value {
            continue;
        }
        let line = e.mem.u32(base + IO_LINES + 4 * i);
        let tile = e.mem.u32(line.wrapping_add(0x2c));
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
        if f64::from(column) != value {
            continue;
        }
        let line = e.mem.u32(base + IO_LINES + 4 * i);
        return e.mem.u32(line.wrapping_add(0x2c));
    }
    0
}

// Translated from 00767ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::ScrollLog` (Xbox PDB): moves every log tile `count` rows
/// up (trait `0xfac` plus `count`), then renders `count` new log lines
/// (`010729c8`) under the log tile, row `count - i - 1` relative, with the
/// texts `lines[i]`, and appends them to the log tiles.
pub fn hacking_menu_scroll_log(e: &mut Engine, this: Ptr<HackingMenu>, count: u32, lines: Ptr) {
    let base = this.addr();
    let list = base + LOG_TILES;
    e.with_stack(4, |e, node| {
        let head = e.call(WORD_AT_ZERO, &args![list]).u32();
        e.mem.set_u32(node.addr(), head);
        while e.mem.u32(node.addr()) != 0 {
            let element = e.call(POINTER_LIST_NEXT, &args![list, node]).u32();
            let tile = e.mem.u32(element);
            // The flag pushed before the getter is the setter's last argument.
            let row = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
            let row = x87_store32(x87_add(row, f64::from(count)));
            e.call(TILE_SET_FLOAT, &args![tile, 0xfacu32, row, 1u32]);
        }
    });
    let mut i = 0u32;
    while i < count {
        let log = hacking_tile(e, base, 3);
        let tile = e
            .call(
                MENU_RENDER_TEMPLATE,
                &args![this, log, LOG_LINE_TEMPLATE, 0u32],
            )
            .u32();
        e.call(
            TILE_SET_INT_FROM_VALUE,
            &args![tile, 0xfacu32, count.wrapping_sub(i).wrapping_sub(1)],
        );
        let text = e.mem.u32(lines.addr().wrapping_add(4 * i));
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), tile);
            e.call(POINTER_LIST_ADD_TAIL, &args![list, slot]);
        });
        i += 1;
    }
}

// Translated from 00767c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::DoIdle` (Xbox PDB). Only with the fade state 1, as the top
/// menu, and (when `bTutorial`) while tutorial message `0x13` is showing:
/// with a gamepad the manager's selected tile (+0xcc) becomes `pOverTile`;
/// updates the entry text, flashes the cursor, then by `eStage`:
/// 0 `DoIntro`; 1 types the first unfinished terminal line (sound 3,
/// update, scrolling cursor) or, when all are done, puts the cursor at the
/// entry tile's height and starts the game (`eStage` 2); 2 updates the
/// cursor, the second line when one guess is left, and handles the
/// selected tile when it sits in the file tile (`00769b50`); 3 updates the
/// cursor and shuts down; 4 updates the cursor and, with guesses left and
/// the fade state not 2, may go back to the computers menu (`0076a510`).
pub fn hacking_menu_do_idle(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    if e.mem.u32(base + 0x24) != 1 {
        return;
    }
    let class = fn_00766a40(e);
    if !e.call(IS_TOP_MENU_ID, &args![class]).bool() {
        return;
    }
    if e.get(this, HackingMenu::bTutorial) {
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let showing = e
            .call(
                TUTORIAL_IS_SHOWING,
                &args![manager.wrapping_add(0x4d4), 0x13u32],
            )
            .bool();
        if !showing {
            return;
        }
    }
    e.call(INTERFACE_SINGLETON, &args![]);
    if e.call(GAMEPAD_IN_USE, &args![]).bool() {
        let manager = e.global::<u32>(INTERFACE_MANAGER);
        let over = e.get(this, HackingMenu::pOverTile);
        e.mem.set_u32(manager.wrapping_add(0xcc), over);
    }
    e.call(ANIMATING_TEXT_UPDATE, &args![base + ENTRY_TEXT]);
    hacking_menu_flash_cursor(e, this);
    match e.get(this, HackingMenu::eStage) {
        0 => hacking_menu_do_intro(e, this),
        1 => {
            let mut i = 0u32;
            while i < IO_LINE_COUNT {
                let line = e.mem.u32(base + IO_LINES + 4 * i);
                if line != 0 && !e.call(ANIMATING_TEXT_FINISHED, &args![line]).bool() {
                    e.call(SOUND_HANDLE_PLAY, &args![base + SOUNDS + 0x24, 0u32]);
                    let line = e.mem.u32(base + IO_LINES + 4 * i);
                    e.call(ANIMATING_TEXT_UPDATE, &args![line]);
                    let line = e.mem.u32(base + IO_LINES + 4 * i);
                    hacking_menu_update_scrolling_cursor(e, this, Ptr::new(line));
                    break;
                }
                i += 1;
            }
            if i == IO_LINE_COUNT {
                let mut y = 0i32;
                let mut tile = hacking_tile(e, base, 5);
                while tile != 0 {
                    let cursor = hacking_tile(e, base, 4);
                    if tile == e.call(TILE_PARENT, &args![cursor]).u32() {
                        break;
                    }
                    let value = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_Y]).f64();
                    y = ftol(e, value).wrapping_add(y);
                    tile = e.call(TILE_PARENT, &args![tile]).u32();
                }
                let cursor = hacking_tile(e, base, 4);
                e.call(TILE_SET_INT, &args![cursor, TRAIT_Y, y]);
                e.set(this, HackingMenu::eStage, 2);
            }
        }
        2 => {
            hacking_menu_update_cursor_pos(e, this);
            if e.get(this, HackingMenu::iNumGuesses) == 1 {
                let line = e.mem.u32(base + IO_LINES + 4);
                e.call(ANIMATING_TEXT_UPDATE, &args![line]);
            }
            let manager = e.global::<u32>(INTERFACE_MANAGER);
            if e.mem.u32(manager.wrapping_add(0xcc)) != 0 {
                let manager = e.global::<u32>(INTERFACE_MANAGER);
                let selected = e.mem.u32(manager.wrapping_add(0xcc));
                let parent = e.call(TILE_PARENT, &args![selected]).u32();
                if parent == hacking_tile(e, base, 2) {
                    fn_00769b50(e, this);
                }
            }
        }
        3 => {
            hacking_menu_update_cursor_pos(e, this);
            hacking_menu_do_shutdown(e, this);
        }
        4 => {
            hacking_menu_update_cursor_pos(e, this);
            if e.get(this, HackingMenu::iNumGuesses) != 0 && e.mem.u32(base + 0x24) != 2 {
                fn_0076a510(e, this);
            }
        }
        _ => {}
    }
}

/// The per-character sound spacing `DoIntro` computes once: like a terminal
/// line's display interval (`004bd510(double 01017b70 / value, 1.0)`, rounded to a whole number, for a
/// setting value above 1, else the float `01013974`), converted with
/// `_ftol2`.
fn typing_spacing(e: &mut Engine) -> i32 {
    let setting = LINE_INTERVAL_SETTINGS[1];
    let word = e.call(SETTING_VALUE, &args![setting]).u32();
    let bits = if e.mem.u32(word) > 1 {
        let word = e.call(SETTING_VALUE, &args![setting]).u32();
        let value = e.mem.u32(word);
        let scale = e.mem.f64(INTERVAL_SCALE);
        let ratio = x87_store32(x87_div(scale, f64::from(value)));
        let clamped = e
            .call(ROUND_TO_MULTIPLE, &args![ratio, 1.0f32.to_bits()])
            .f64();
        x87_store32(clamped)
    } else {
        x87_quiet_bits(e.mem.u32(DEFAULT_INTERVAL))
    };
    ftol(e, x87_load32(bits))
}

// Translated from 00769830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::DoIntro` (Xbox PDB): shows the first line's text only until
/// the retry time; types the first of the first 14 lines that is not
/// finished: a line with a prompt shows the prompt first (the next step a
/// second later), then on each step types more of the line with one typing
/// sound per new character, spaced over the per-character spacing, and
/// plays sound 4 (the next step 500 ms later) when the line is done; a line
/// without a prompt is updated on each step with sound 3. Then puts the
/// cursor after that line. When all 14 are finished and the step time has
/// passed, builds the password file (`MakePasswordFile`).
// Translation incomplete: 00769830 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_do_intro(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let mut i = 0i32;
    while i < 0xe {
        let now = e.call(GAME_TIME, &args![]).u32();
        let visible = u32::from(now <= e.global::<u32>(RETRY_TIME));
        let first = e.mem.u32(base + IO_LINES);
        let tile = e.mem.u32(first.wrapping_add(0x2c));
        e.call(TILE_SET_INT, &args![tile, TRAIT_VISIBLE, visible]);
        let line_address = base + IO_LINES + 4 * i as u32;
        let line = e.mem.u32(line_address);
        if line == 0 || e.call(ANIMATING_TEXT_FINISHED, &args![line]).bool() {
            i += 1;
            continue;
        }
        if e.mem.u32(e.mem.u32(line_address) + 0x30) != 0 {
            let prompt = e.mem.u32(e.mem.u32(line_address) + 0x30);
            if !e.call(TILE_IS_TRUE, &args![prompt, TRAIT_VISIBLE]).bool() {
                let prompt = e.mem.u32(e.mem.u32(line_address) + 0x30);
                e.call(TILE_SET_INT, &args![prompt, TRAIT_VISIBLE, 1u32]);
                let now = e.call(GAME_TIME, &args![]).u32();
                e.set_global(INTRO_NEXT_TIME, now.wrapping_add(0x3e8));
            }
            let now = e.call(GAME_TIME, &args![]).u32();
            if now > e.global::<u32>(INTRO_NEXT_TIME) {
                let tile = e.mem.u32(e.mem.u32(line_address) + 0x2c);
                let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
                let before = e.call(STRLEN, &args![text]).u32();
                let line = e.mem.u32(line_address);
                e.call(ANIMATING_TEXT_UPDATE, &args![line]);
                let tile = e.mem.u32(e.mem.u32(line_address) + 0x2c);
                let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
                let after = e.call(STRLEN, &args![text]).u32();
                let mut k = 0u32;
                while k < after.wrapping_sub(before) {
                    let guard = e.global::<u32>(TYPING_SPACING_INITIALIZED);
                    if guard & 1 == 0 {
                        e.set_global(TYPING_SPACING_INITIALIZED, guard | 1);
                        let spacing = typing_spacing(e);
                        e.set_global(TYPING_SPACING, spacing);
                    }
                    let spacing = e.global::<u32>(TYPING_SPACING);
                    let delay = k.wrapping_mul(spacing) / after.wrapping_sub(before);
                    e.with_stack(0xc, |e, handle| {
                        let manager = e.call(AUDIO_MANAGER, &args![]).u32();
                        let found = e
                            .call(
                                GET_SOUND_HANDLE_BY_NAME,
                                &args![manager, handle, TYPING_SOUND, 0x121u32],
                            )
                            .u32();
                        e.call(SOUND_HANDLE_PLAY_AFTER, &args![found, delay, 0u32]);
                        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                    });
                    k += 1;
                }
                let line = e.mem.u32(line_address);
                if e.call(ANIMATING_TEXT_FINISHED, &args![line]).bool() {
                    e.call(SOUND_HANDLE_PLAY, &args![base + SOUNDS + 0x30, 0u32]);
                    let now = e.call(GAME_TIME, &args![]).u32();
                    e.set_global(INTRO_NEXT_TIME, now.wrapping_add(0x1f4));
                }
            }
        } else {
            let now = e.call(GAME_TIME, &args![]).u32();
            if now > e.global::<u32>(INTRO_NEXT_TIME) {
                let line = e.mem.u32(line_address);
                e.call(ANIMATING_TEXT_UPDATE, &args![line]);
                e.call(SOUND_HANDLE_PLAY, &args![base + SOUNDS + 0x24, 0u32]);
            }
        }
        let line = e.mem.u32(line_address);
        hacking_menu_update_scrolling_cursor(e, this, Ptr::new(line));
        break;
    }
    if i == 0xe {
        let now = e.call(GAME_TIME, &args![]).u32();
        if now > e.global::<u32>(INTRO_NEXT_TIME) {
            hacking_menu_make_password_file(e, this);
        }
    }
}

/// Answers `this` (`006815c0`, `thiscall`, no arguments; the code uses it
/// on a list node).
const IDENTITY: u32 = 0x0068_15c0;
/// The string comparison the dud removal uses (cdecl: two strings; 0 when
/// equal).
const STRING_COMPARE: u32 = 0x0040_8b20;
/// Removes a node from the word list (`thiscall`: node).
const WORD_LIST_REMOVE: u32 = 0x0090_5330;

// Translated from 0076a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes a random dud word: picks a word other
/// than the password, overwrites its characters in the file with dots
/// (the line from its buffer position: 12 characters per line from line 4,
/// the column after the 7-character address; the rest continues on the next
/// line when that line's tile has the trait `0xfaf`), shifts the following
/// buffer positions down (the last becomes `0x198`), frees the word, removes
/// it from the word list and decrements `iWordsHave`.
// Translation incomplete: 0076a730 - the /GS cookie check (00ec408c) is not translated
pub fn fn_0076a730(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let (index, node) = loop {
        let count = e.get(this, HackingMenu::iWordsHave);
        let index = e.call(RANDOM_BELOW, &args![count]).u32();
        let mut steps = index;
        let mut node = base + WORD_LIST;
        loop {
            let left = steps;
            steps = steps.wrapping_sub(1);
            if left == 0 {
                break;
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        let password = e.get(this, HackingMenu::pPassword);
        let item = e.call(IDENTITY, &args![node]).u32();
        let word = e.mem.u32(item);
        if e.call(STRING_COMPARE, &args![word, password]).i32() != 0 {
            break (index, node);
        }
    };
    let positions = e.get(this, HackingMenu::iBufferPos);
    let mut column = e.mem.u32(positions.wrapping_add(4 * index));
    let mut line = 4u32;
    while column > 0xc {
        column -= 0xc;
        line += 1;
    }
    let mut written = 0u32;
    e.with_stack(0x14, |e, buffer| {
        let line_object = e.mem.u32(base + IO_LINES + 4 * line);
        let tile = e.mem.u32(line_object.wrapping_add(0x2c));
        line += 1;
        let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
        e.call(MEMCPY, &args![buffer, text, 0x14u32]);
        while written < e.get(this, HackingMenu::iWordLength) && column < 0xc {
            e.mem.set_u8(buffer.addr().wrapping_add(column + 7), b'.');
            column += 1;
            written += 1;
        }
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, buffer, 1u32]);
        let next_address = base.wrapping_add(IO_LINES + 4 * line);
        let next = e.mem.u32(next_address);
        if next == 0 || e.mem.u32(next.wrapping_add(0x2c)) == 0 {
            return;
        }
        let next_tile = e.mem.u32(e.mem.u32(next_address).wrapping_add(0x2c));
        if !e.call(TILE_IS_TRUE, &args![next_tile, 0xfafu32]).bool()
            || written >= e.get(this, HackingMenu::iWordLength)
        {
            return;
        }
        let mut column = 0u32;
        let tile = e.mem.u32(e.mem.u32(next_address).wrapping_add(0x2c));
        let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
        e.call(MEMCPY, &args![buffer, text, 0x14u32]);
        loop {
            let before = written;
            written = written.wrapping_add(1);
            if before >= e.get(this, HackingMenu::iWordLength) {
                break;
            }
            // Not bounded by the buffer: the exe writes past it for long words.
            e.mem.set_u8(buffer.addr().wrapping_add(column + 7), b'.');
            column += 1;
        }
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, buffer, 1u32]);
    });
    let mut i = index;
    while i < e.get(this, HackingMenu::iWordsHave).wrapping_sub(1) {
        let positions = e.get(this, HackingMenu::iBufferPos);
        let next = e.mem.u32(positions.wrapping_add(4 * i + 4));
        e.mem.set_u32(positions.wrapping_add(4 * i), next);
        i += 1;
    }
    let positions = e.get(this, HackingMenu::iBufferPos);
    e.mem.set_u32(positions.wrapping_add(4 * i), 0x198);
    let item = e.call(IDENTITY, &args![node]).u32();
    let word = e.mem.u32(item);
    e.call(OPERATOR_DELETE, &args![word]);
    let item = e.call(IDENTITY, &args![node]).u32();
    e.call(WORD_LIST_REMOVE, &args![base + WORD_LIST, item]);
    let count = e.get(this, HackingMenu::iWordsHave);
    e.set(this, HackingMenu::iWordsHave, count.wrapping_sub(1));
}

// Translated from 00769520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`this` = the menu, `grid` = a
/// `HackingMenu::Grid`): a recursive measure of the grid. `m` is the
/// smallest of the rows' largest value counts (`0076ae90`); 0 when there
/// are no rows, `m + 1` when `m <= 2`. Otherwise, for each row whose count
/// is `m`, every value `j` that the row holds `m` times gives a sub-grid of
/// those positions (`0076af90`) measured recursively; the largest of those,
/// when not 0, lowers the result (starting at the grid's size), which is
/// answered plus 1 as soon as it is at most 2, else after all rows.
// Translation incomplete: 00769520 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
pub fn fn_00769520(e: &mut Engine, this: Ptr<HackingMenu>, grid: Ptr) -> u32 {
    let mut smallest = 0xffu8;
    let mut i = 0u32;
    while i < e.call(WORD_AT_FOUR, &args![grid]).u32() {
        let rows = e.call(WORD_AT_ZERO, &args![grid]).u32();
        let largest = fn_0076ae90(e, Ptr::new(rows.wrapping_add(8 * i)));
        if largest < smallest {
            smallest = largest;
        }
        i += 1;
    }
    if smallest == 0xff {
        return 0;
    }
    if smallest <= 2 {
        return u32::from(smallest) + 1;
    }
    let mut best = e.call(WORD_AT_FOUR, &args![grid]).u32();
    let mut row = 0u32;
    while row < e.call(WORD_AT_FOUR, &args![grid]).u32() {
        let rows = e.call(WORD_AT_ZERO, &args![grid]).u32();
        let largest = fn_0076ae90(e, Ptr::new(rows.wrapping_add(8 * row)));
        if largest == smallest {
            let mut deepest = 0u32;
            let mut value = 0u32;
            while value < e.call(WORD_AT_FOUR, &args![grid]).u32() {
                deepest = e.with_stack(0x20, |e, positions| {
                    let rows = e.call(WORD_AT_ZERO, &args![grid]).u32();
                    let count = fn_0076ae30(
                        e,
                        Ptr::new(rows.wrapping_add(8 * row)),
                        value as i32,
                        positions,
                    );
                    if count != smallest {
                        return deepest;
                    }
                    e.with_stack(8, |e, sub_grid| {
                        fn_0076af90(e, grid, sub_grid, positions, smallest);
                        let measure = fn_00769520(e, this, sub_grid);
                        let deeper = e.call(MAX_OF_TWO, &args![deepest, measure]).u32();
                        fn_007687d0(e, sub_grid);
                        deeper
                    })
                });
                value += 1;
            }
            if deepest != 0 {
                best = e.call(MIN_OF_TWO, &args![best, deepest]).u32();
            }
            if best <= 2 {
                return best + 1;
            }
        }
        row += 1;
    }
    best + 1
}

/// `17.0` (`double`): the rows of a file column.
const ROWS_PER_COLUMN: u32 = 0x0103_56f8;
/// `12.0` (`double`): the characters of a file row.
const CHARACTERS_PER_ROW: u32 = 0x0103_5710;
/// `1.5` (`double`): the typing sounds' spacing factor.
const TYPING_SOUND_SCALE: u32 = 0x0101_6ff0;
/// `119.0` (`double`): the x offset of the second file column.
const SECOND_COLUMN_OFFSET: u32 = 0x0107_29c0;
/// `HackingMenu::listSoundsToPlay` (Xbox PDB static,
/// `BSSimpleList<BSSoundHandle>`).
const SOUNDS_TO_PLAY: u32 = 0x011d_9604;
/// The sound list's next node (`thiscall`), its clear and its add
/// (`thiscall`: address of the handle).
const SOUND_LIST_NEXT: u32 = 0x0084_e3a0;
const SOUND_LIST_CLEAR: u32 = 0x0076_b7a0;
const SOUND_LIST_ADD: u32 = 0x0076_b6a0;
/// Whether the bonus list holds the word at an address (`thiscall`:
/// address, flag).
const BONUS_LIST_FIND: u32 = 0x0049_c680;
/// `dwDisplayInterval` (+0x10) of an `AnimatingText` (`0044edb0`, a shared
/// getter body).
const WORD_AT_0X10: u32 = 0x0044_edb0;

// Translated from 00769b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the file tile's hover handling. From the
/// column under the cursor (`GetCharHit` - 7) and the selected tile
/// (`pim` + 0xcc), unchanged since the last call or off the text, nothing
/// happens; off the text clears the entry. While `bSuppressEntry`, the
/// entry is cleared (and the suppression ends off a letter). Otherwise the
/// buffer position `(column * 17 + line) * 12 + hit` (traits `0x1004`, `0xfac`) picks the word under
/// the cursor (or a bonus bracket run, `GetGarbageString`, or the single
/// character); when the entry text changes, the pending typing sounds are
/// released and one typing sound per character is queued, spaced by the
/// display interval times 1.5. Then the highlight traits of the file tile
/// (`0x1004`..`0x1009`) are set: for a word, split where it wraps to the
/// next row (or column, `FindTile`); otherwise at the cursor.
// Translation incomplete: 00769b50 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
pub fn fn_00769b50(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let selected = |e: &Engine| {
        let manager = e.global::<u32>(INTERFACE_MANAGER);
        e.mem.u32(manager.wrapping_add(0xcc))
    };
    let mut hit = hacking_menu_get_char_hit(e, this).wrapping_sub(7);
    let unchanged = hit == e.get(this, HackingMenu::iLastOffset)
        && selected(e) == e.get(this, HackingMenu::pLastTarget);
    if unchanged || (hit < 0 && e.get(this, HackingMenu::iLastOffset) < 0) {
        return;
    }
    e.set(this, HackingMenu::iLastOffset, hit);
    let target = selected(e);
    e.set(this, HackingMenu::pLastTarget, target);
    if hit < 0 {
        hacking_menu_clear_entry(e, this);
        e.set(this, HackingMenu::bSuppressEntry, false);
        return;
    }
    if e.get(this, HackingMenu::bSuppressEntry) {
        hacking_menu_clear_entry(e, this);
        let tile = selected(e);
        let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
        let character = e.mem.i8(text.wrapping_add(hit as u32).wrapping_add(7));
        if e.call(ISALPHA, &args![i32::from(character)]).i32() == 0 {
            e.set(this, HackingMenu::bSuppressEntry, false);
            e.set(this, HackingMenu::pLastTarget, 0);
        }
        return;
    }
    let tile = selected(e);
    let row = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
    let rows = x87_mul(row, e.mem.f64(ROWS_PER_COLUMN));
    let tile = selected(e);
    let column = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    let cell = x87_mul(x87_add(column, rows), e.mem.f64(CHARACTERS_PER_ROW));
    let position = x87_fistp64_chop(X87Ext::load64(x87_add(cell, f64::from(hit)))) as u32;
    let mut node = base + WORD_LIST;
    let mut k = 0u32;
    while k < e.get(this, HackingMenu::iWordsHave) {
        let positions = e.get(this, HackingMenu::iBufferPos);
        let start = e.mem.u32(positions.wrapping_add(4 * k));
        if position >= start && position < start.wrapping_add(e.get(this, HackingMenu::iWordLength))
        {
            break;
        }
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
        k += 1;
    }
    e.with_stack(0x20, |e, buffer| {
        e.call(MEMSET, &args![buffer, 0u32, 0x20u32]);
        if node != 0 {
            let length = e.get(this, HackingMenu::iWordLength);
            let item = e.call(IDENTITY, &args![node]).u32();
            let word = e.mem.u32(item);
            e.call(MEMCPY, &args![buffer, word, length]);
        } else {
            let found = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), position);
                e.call(BONUS_LIST_FIND, &args![base + USED_BONUSES, slot, 0u32])
                    .u32()
            });
            let tile = selected(e);
            let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
            let column = hacking_menu_get_char_hit(e, this) as u32;
            if found == 0 {
                hacking_menu_get_garbage_string(
                    e,
                    Ptr::new(base),
                    buffer.addr(),
                    text.wrapping_add(column),
                );
            } else {
                let character = e.mem.u8(text.wrapping_add(column));
                e.mem.set_u8(buffer.addr(), character);
            }
        }
        if fn_0076a3d0(e, Ptr::new(base + ENTRY_TEXT), buffer) {
            e.call(
                ANIMATING_TEXT_RESET_BUFFER,
                &args![base + ENTRY_TEXT, buffer],
            );
            let mut sound = SOUNDS_TO_PLAY;
            while sound != 0 {
                let handle = e.call(IDENTITY, &args![sound]).u32();
                if !fn_0076a3b0(e, Ptr::new(handle), u32::MAX) {
                    break;
                }
                let handle = e.call(IDENTITY, &args![sound]).u32();
                e.call(SOUND_HANDLE_RELEASE, &args![handle]);
                sound = e.call(SOUND_LIST_NEXT, &args![sound]).u32();
            }
            e.call(SOUND_LIST_CLEAR, &args![SOUNDS_TO_PLAY]);
            let mut left = e.call(STRLEN, &args![buffer]).u32();
            loop {
                let before = left;
                left = left.wrapping_sub(1);
                if before == 0 {
                    break;
                }
                e.with_stack(0xc, |e, handle| {
                    let manager = e.call(AUDIO_MANAGER, &args![]).u32();
                    e.call(
                        GET_SOUND_HANDLE_BY_NAME,
                        &args![manager, handle, TYPING_SOUND, 0x121u32],
                    );
                    e.call(SOUND_LIST_ADD, &args![SOUNDS_TO_PLAY, handle]);
                    if left != 0 {
                        let interval = e.call(WORD_AT_0X10, &args![base + ENTRY_TEXT]).u32();
                        let delay = x87_mul(
                            f64::from(interval.wrapping_mul(left)),
                            e.mem.f64(TYPING_SOUND_SCALE),
                        );
                        let delay = ftol(e, delay);
                        e.call(SOUND_HANDLE_PLAY_AFTER, &args![handle, delay, 0u32]);
                    } else {
                        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
                    }
                    e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                });
            }
        }
        let mut done = false;
        hit = hacking_menu_get_char_hit(e, this);
        if node != 0 {
            let tile = selected(e);
            let text = e.call(TILE_GET_STRING, &args![tile, TRAIT_STRING]).u32();
            loop {
                let character = e.mem.i8(text.wrapping_add(hit as u32).wrapping_sub(1));
                if e.call(ISALPHA, &args![i32::from(character)]).i32() == 0 {
                    break;
                }
                hit = hit.wrapping_sub(1);
            }
            let mut i = 1u32;
            while !done && i < e.get(this, HackingMenu::iWordLength) {
                let tile = selected(e);
                let value = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_X]).f64();
                let mut x = ftol(e, value);
                let tile = selected(e);
                let value = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_Y]).f64();
                let mut y = ftol(e, value);
                let mut second_x = x.wrapping_add(0x77);
                let mut second_y = y;
                let character = e.mem.i8(text.wrapping_add(hit as u32).wrapping_add(i));
                if character == 0 {
                    done = true;
                    x = hit.wrapping_mul(0x11).wrapping_add(x);
                    let tile = selected(e);
                    let column = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                    let next = x87_add(column, e.mem.f64(ONE_DOUBLE));
                    // FCOMP; TEST AH,5; JP: on only when below (ordered).
                    if next < e.mem.f64(ROWS_PER_COLUMN) {
                        let tile = selected(e);
                        let height = e.call(TILE_GET_FLOAT, &args![tile, 0xfb0u32]).f64();
                        second_y = ftol(e, height).wrapping_add(second_y);
                    } else {
                        let tile = selected(e);
                        let row = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
                        let row = x87_fistp64_chop(X87Ext::load64(row)) as u32;
                        let other = hacking_menu_find_tile(e, this, 0, row.wrapping_add(1));
                        let left = e.call(TILE_GET_FLOAT, &args![other, TRAIT_X]).f64();
                        second_x = ftol(e, x87_add(left, e.mem.f64(SECOND_COLUMN_OFFSET)));
                        let top = e.call(TILE_GET_FLOAT, &args![other, TRAIT_Y]).f64();
                        second_y = ftol(e, top);
                    }
                } else if e.call(ISALPHA, &args![i32::from(character)]).i32() == 0 {
                    done = true;
                    i = e.get(this, HackingMenu::iWordLength).wrapping_sub(i);
                    let width = 0x13u32.wrapping_sub(i).wrapping_mul(0x11);
                    let tile = selected(e);
                    let column = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                    // FCOMP; TEST AH,0x44; JNP: the second branch only when equal.
                    if column != e.mem.f64(ZERO_DOUBLE) {
                        x = (width as i32).wrapping_add(x);
                        let tile = selected(e);
                        let height = e.call(TILE_GET_FLOAT, &args![tile, 0xfb0u32]).f64();
                        y = y.wrapping_sub(ftol(e, height));
                    } else {
                        let tile = selected(e);
                        let row = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
                        let row = x87_fistp64_chop(X87Ext::load64(row)) as u32;
                        let other = hacking_menu_find_tile(e, this, 0x10, row.wrapping_sub(1));
                        let left = e.call(TILE_GET_FLOAT, &args![other, TRAIT_X]).f64();
                        x = ftol(e, x87_add(left, f64::from(width)));
                        let top = e.call(TILE_GET_FLOAT, &args![other, TRAIT_Y]).f64();
                        y = ftol(e, top);
                    }
                }
                if done {
                    let file = hacking_tile(e, base, 2);
                    let split = buffer.addr().wrapping_add(i);
                    let saved = e.mem.u8(split);
                    e.mem.set_u8(split, 0);
                    e.call(TILE_SET_STRING, &args![file, 0x1004u32, buffer, 1u32]);
                    let file = hacking_tile(e, base, 2);
                    e.call(TILE_SET_INT, &args![file, 0x1005u32, x]);
                    let file = hacking_tile(e, base, 2);
                    e.call(TILE_SET_INT, &args![file, 0x1006u32, y]);
                    e.mem.set_u8(split, saved);
                    let file = hacking_tile(e, base, 2);
                    e.call(TILE_SET_STRING, &args![file, 0x1007u32, split, 1u32]);
                    let file = hacking_tile(e, base, 2);
                    e.call(TILE_SET_INT, &args![file, 0x1008u32, second_x]);
                    let file = hacking_tile(e, base, 2);
                    e.call(TILE_SET_INT, &args![file, 0x1009u32, second_y]);
                }
                i = i.wrapping_add(1);
            }
        }
        if !done {
            let file = hacking_tile(e, base, 2);
            e.call(TILE_SET_STRING, &args![file, 0x1004u32, buffer, 1u32]);
            // The flags pushed before the getters are the setters' last arguments.
            let tile = selected(e);
            let left = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_X]).f64();
            let x = x87_store32(x87_add(left, f64::from(hit.wrapping_mul(0x11))));
            let file = hacking_tile(e, base, 2);
            e.call(TILE_SET_FLOAT, &args![file, 0x1005u32, x, 1u32]);
            let tile = selected(e);
            let top = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_Y]).f64();
            let file = hacking_tile(e, base, 2);
            e.call(
                TILE_SET_FLOAT,
                &args![file, 0x1006u32, x87_store32(top), 1u32],
            );
            let file = hacking_tile(e, base, 2);
            e.call(TILE_SET_INT, &args![file, 0x1008u32, -1i32]);
        }
    });
}

/// `"Data\Menus\FalloutDict.txt"`, the word file.
const WORD_FILE: u32 = 0x0107_2a34;
/// The message printed when the word file cannot be opened (`%s`).
const WORD_FILE_MESSAGE: u32 = 0x0107_2a00;
/// The debug message naming the password.
const PASSWORD_MESSAGE: u32 = 0x0107_29e8;
/// Sets a `BSStringT` from a C string (`thiscall`: text).
const STRING_SET: u32 = 0x0043_8390;
/// `FileFinder::GetFile_ov2` (Xbox PDB, cdecl: path, mode, buffer size,
/// flags).
const FILE_FINDER_GET_FILE: u32 = 0x00af_df20;
/// `Error` (Xbox PDB, cdecl: format, words).
const ERROR_MESSAGE: u32 = 0x0040_fbe0;

// Translated from 00768070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::Prepare` (Xbox PDB): chooses `count` words of
/// `iWordLength` letters from the word file (closing the menu with a
/// message when it cannot be read): the candidates are shuffled, then for
/// up to 100 ms per round words are taken one by one when no word already
/// taken equals them and (before the last word) they share a letter
/// position with every taken word or a random draw below 10 is under the
/// round number, and (for the last word) no number of shared positions
/// below 8 occurs more than the limit `(count - 2) / iWordLength + 1`; ten
/// rounds raise the limit, up to `count / 2`. The taken words are shuffled
/// four times, a random one becomes `pPassword`, and the shared-position
/// grid of all pairs (`0xff` on the diagonal) gives `iNumGuesses` (at least
/// 4, `00769520`), also stored as `iTotGuesses`.
// Translation incomplete: 00768070 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
pub fn hacking_menu_prepare(e: &mut Engine, this: Ptr<HackingMenu>, count: u32) {
    e.with_stack(8, |e, path| {
        e.call(STRING_CONSTRUCTOR, &args![path]);
        e.with_stack(8, |e, candidates| {
            e.call(WORD_LIST_CONSTRUCTOR, &args![candidates]);
            prepare_words(e, this, count, path, candidates);
            fn_00765810(e, candidates);
        });
        e.call(STRING_DESTRUCTOR, &args![path]);
    });
}

/// The body of [`hacking_menu_prepare`] between its locals' constructors
/// and destructors.
fn prepare_words(e: &mut Engine, this: Ptr<HackingMenu>, count: u32, path: Ptr, candidates: Ptr) {
    let base = this.addr();
    let words = base + WORD_LIST;
    e.call(GAME_TIME, &args![]);
    let mut attempts = 0u32;
    // The exe divides before it tests `iWordLength` for 0 (a zero length
    // faults in the division there, and panics here).
    let mut limit = count.wrapping_sub(2) / e.get(this, HackingMenu::iWordLength) + 1;
    let mut rounds = 0u32;
    if e.get(this, HackingMenu::iWordLength) == 0 {
        return;
    }
    e.call(STRING_SET, &args![path, WORD_FILE]);
    let name = e.call(WORD_AT_ZERO, &args![path]).u32();
    let file = e
        .call(
            FILE_FINDER_GET_FILE,
            &args![name, 0u32, 0x4000u32, 0x100u32],
        )
        .u32();
    if file == 0 || !e.vcall(file, 0x20, &args![0u32, 0u32]).bool() {
        let name = e.call(WORD_AT_ZERO, &args![path]).u32();
        e.call(PRINT_MESSAGE, &args![0x12u32, WORD_FILE_MESSAGE, name]);
        hacking_menu_close(e);
        return;
    }
    e.with_stack(0x20, |e, line| loop {
        e.call(MEMSET, &args![line, 0u32, 0x20u32]);
        if e.vcall(file, 0x34, &args![line, 0x20u32, 0x20u32]).u32() == 0 {
            break;
        }
        if e.mem.i8(line.addr()) == 0 {
            continue;
        }
        if e.call(STRLEN, &args![line]).u32() == e.get(this, HackingMenu::iWordLength) {
            fn_0076b240(e, candidates, line.addr());
        }
    });
    delete_object(e, file);
    for _ in 0..3 {
        fn_007687b0(e, candidates);
    }
    loop {
        let deadline = e.call(GAME_TIME, &args![]).u32().wrapping_add(0x64);
        while e.call(GAME_TIME, &args![]).u32() < deadline {
            attempts = attempts.wrapping_add(1);
            if attempts == 0 {
                break;
            }
            fn_0076b2a0(e, Ptr::new(words));
            e.set(this, HackingMenu::iWordsHave, 0);
            fn_007687b0(e, candidates);
            let mut node = candidates.addr();
            while e.get(this, HackingMenu::iWordsHave) < count && node != 0 {
                let item = e.call(IDENTITY, &args![node]).u32();
                let word = e.mem.u32(item);
                if word == 0 {
                    break;
                }
                let accepted = e.with_stack(8, |e, shared| {
                    let mut accepted = true;
                    e.call(MEMSET, &args![shared, 0u32, 8u32]);
                    let mut other = words;
                    while accepted && other != 0 {
                        let item = e.call(IDENTITY, &args![other]).u32();
                        let taken = e.mem.u32(item);
                        if taken == 0 {
                            break;
                        }
                        let common = hacking_menu_common_positions(e, Ptr::new(base), word, taken);
                        if common == e.get(this, HackingMenu::iWordLength) {
                            accepted = false;
                        } else if e.get(this, HackingMenu::iWordsHave) < count.wrapping_sub(1) {
                            accepted =
                                common > 0 || e.call(RANDOM_BELOW, &args![0xau32]).u32() < rounds;
                        } else if common < 8 {
                            let slot = shared.addr() + common;
                            let seen = e.mem.u8(slot).wrapping_add(1);
                            e.mem.set_u8(slot, seen);
                            accepted = u32::from(e.mem.u8(slot)) <= limit;
                        } else {
                            accepted = true;
                        }
                        other = e.call(WORD_AT_FOUR, &args![other]).u32();
                    }
                    accepted
                });
                if accepted {
                    fn_0076b240(e, Ptr::new(words), word);
                    let have = e.get(this, HackingMenu::iWordsHave);
                    e.set(this, HackingMenu::iWordsHave, have.wrapping_add(1));
                }
                node = e.call(WORD_AT_FOUR, &args![node]).u32();
            }
            if e.get(this, HackingMenu::iWordsHave) == count {
                break;
            }
        }
        if e.get(this, HackingMenu::iWordsHave) == count {
            break;
        }
        rounds += 1;
        if rounds == 0xa {
            rounds = 0;
            limit = limit.wrapping_add(1);
            if limit > count >> 1 {
                break;
            }
        }
    }
    for _ in 0..4 {
        fn_007687b0(e, Ptr::new(words));
    }
    let have = e.get(this, HackingMenu::iWordsHave);
    let pick = e.call(RANDOM_BELOW, &args![have]).u32();
    let mut node = words;
    for _ in 0..pick {
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
    }
    let item = e.call(IDENTITY, &args![node]).u32();
    let password = e.mem.u32(item);
    e.set(this, HackingMenu::pPassword, password);
    e.call(ERROR_MESSAGE, &args![PASSWORD_MESSAGE, password]);
    e.with_stack(8, |e, grid| {
        let have = e.get(this, HackingMenu::iWordsHave);
        fn_00768810(e, grid, have);
        let mut row = 0u32;
        let mut first = words;
        while first != 0 {
            let item = e.call(IDENTITY, &args![first]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            let mut column = 0u32;
            let mut second = words;
            while second != 0 {
                let item = e.call(IDENTITY, &args![second]).u32();
                if e.mem.u32(item) == 0 {
                    break;
                }
                let shared = if first == second {
                    0xff
                } else {
                    let item = e.call(IDENTITY, &args![second]).u32();
                    let b = e.mem.u32(item);
                    let item = e.call(IDENTITY, &args![first]).u32();
                    let a = e.mem.u32(item);
                    hacking_menu_common_positions(e, Ptr::new(base), a, b)
                };
                let rows = e.call(WORD_AT_ZERO, &args![grid]).u32();
                let data = e
                    .call(WORD_AT_ZERO, &args![rows.wrapping_add(8 * row)])
                    .u32();
                e.mem.set_u8(data.wrapping_add(column), shared as u8);
                column += 1;
                second = e.call(WORD_AT_FOUR, &args![second]).u32();
            }
            row += 1;
            first = e.call(WORD_AT_FOUR, &args![first]).u32();
        }
        let guesses = fn_00769520(e, this, grid);
        e.set(this, HackingMenu::iNumGuesses, guesses);
        let guesses = e.call(MAX_OF_TWO, &args![4u32, guesses]).u32();
        e.set(this, HackingMenu::iNumGuesses, guesses);
        e.set(this, HackingMenu::iTotGuesses, guesses);
        fn_007687d0(e, grid);
    });
}

/// A setting's string value (`00403df0`, `ECX` = the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// A setting's string value through the second getter body (`004c69f0`,
/// `ECX` = the setting).
const SETTING_STRING_004C69F0: u32 = 0x004c_69f0;
/// `BSStringT<char>`'s length (`thiscall`).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// The settings and the format of the password file's header lines.
const GUESSES_SETTING: u32 = 0x011d_2e20;
const GUESSES_FORMAT: u32 = 0x0107_29ac;
const HEADER_SETTING_ONE: u32 = 0x011d_33cc;
const HEADER_SETTING_TWO: u32 = 0x011d_430c;
/// The garbage characters of the file (a C string).
const GARBAGE_CHARACTERS: u32 = 0x0107_2ae0;
/// The message printed when the words do not fit.
const WORDS_DO_NOT_FIT_MESSAGE: u32 = 0x0107_2a78;
/// The format of a file line (address, text) and its template.
const FILE_LINE_FORMAT: u32 = 0x0101_2058;
const FILE_LINE_TEMPLATE: u32 = 0x0107_2a54;
/// Clears a word list node's link (`006ecd40`, `thiscall`: flag; the engine
/// map's name for the shared body is `CombatProcedureAttackMelee::Initialize`).
const NODE_CLEAR_LINK: u32 = 0x006e_cd40;
/// The garbage buffer's length (the file's characters).
const FILE_CHARACTERS: u32 = 0x198;

/// `new TerminalIO(text, parent, template, y, kind)` as the code writes it:
/// 0x34 bytes, constructed when the allocation succeeds; answers the line or
/// 0. `text` is computed after the allocation.
fn new_terminal_line(
    e: &mut Engine,
    text: impl FnOnce(&mut Engine) -> u32,
    parent: u32,
    template: u32,
    y: i32,
    kind: u8,
) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
    if block == 0 {
        return 0;
    }
    let text = text(e);
    hacking_menu_terminal_io_terminal_io(e, Ptr::new(block), text, parent, template, y, kind).addr()
}

// Translated from 00768aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::MakePasswordFile` (Xbox PDB): deletes the terminal lines,
/// writes the three header lines (two settings and the guesses line,
/// `010729ac`) under the header tile, places the header's guess text after
/// the third, makes the entry line (`007694f0`) on the header's trait
/// `0x1004`, makes one guess block per guess, then fills a 408-character
/// buffer with random garbage characters and places every word at a random
/// position with at least two garbage characters around it (100 failures
/// in a row start over; after four restarts the words left are dropped with
/// a message). The buffer becomes 34 lines of 12 characters, each with its
/// hexadecimal address (`00768970`, from the low word of the buffer
/// address), 17 rows by 2 columns under the file tile; with a gamepad the
/// first line becomes `pOverTile` and the cursor x its screen x plus 119.
/// Hides the intro tile and enters the pre-game (`eStage` 1).
// Translation incomplete: 00768aa0 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
pub fn hacking_menu_make_password_file(e: &mut Engine, this: Ptr<HackingMenu>) {
    let base = this.addr();
    let mut i = 0u32;
    while i < IO_LINE_COUNT {
        let line = e.mem.u32(base + IO_LINES + 4 * i);
        if line == 0 {
            break;
        }
        fn_00765b50(e, Ptr::new(line), 1);
        i += 1;
    }
    e.call(MEMSET, &args![base + IO_LINES, 0u32, 0xb8u32]);
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCTOR, &args![text]);
        make_password_file_body(e, this, text);
        e.call(STRING_DESTRUCTOR, &args![text]);
    });
}

/// The body of [`hacking_menu_make_password_file`] while its string lives.
fn make_password_file_body(e: &mut Engine, this: Ptr<HackingMenu>, text: Ptr) {
    let base = this.addr();
    let guesses_text = e.call(SETTING_STRING, &args![GUESSES_SETTING]).u32();
    let guesses = e.get(this, HackingMenu::iNumGuesses);
    e.call(
        STRING_SPRINTF,
        &args![text, GUESSES_FORMAT, guesses, guesses_text],
    );
    let header = hacking_tile(e, base, 1);
    let y = e.get(this, HackingMenu::iYStart);
    let line = new_terminal_line(
        e,
        |e| {
            e.call(SETTING_STRING_004C69F0, &args![HEADER_SETTING_ONE])
                .u32()
        },
        header,
        PROMPT_TEMPLATE,
        y,
        0,
    );
    e.mem.set_u32(base + IO_LINES, line);
    let header = hacking_tile(e, base, 1);
    let y = e
        .get(this, HackingMenu::iYStart)
        .wrapping_add(e.get(this, HackingMenu::iYInterval));
    let line = new_terminal_line(
        e,
        |e| {
            e.call(SETTING_STRING_004C69F0, &args![HEADER_SETTING_TWO])
                .u32()
        },
        header,
        PROMPT_TEMPLATE,
        y,
        0,
    );
    e.mem.set_u32(base + IO_LINES + 4, line);
    let header = hacking_tile(e, base, 1);
    let y = e
        .get(this, HackingMenu::iYInterval)
        .wrapping_mul(3)
        .wrapping_add(e.get(this, HackingMenu::iYStart));
    let line = new_terminal_line(
        e,
        |e| e.call(WORD_AT_ZERO, &args![text]).u32(),
        header,
        PROMPT_TEMPLATE,
        y,
        0,
    );
    e.mem.set_u32(base + IO_LINES + 8, line);
    // The flags pushed before the getters are the setters' last arguments.
    let third = e.mem.u32(base + IO_LINES + 8);
    let tile = e.mem.u32(third.wrapping_add(0x2c));
    let left = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_X]).f64();
    let length = e.call(STRING_LENGTH, &args![text]).u32();
    let x = x87_store32(x87_add(f64::from(length.wrapping_mul(0x11)), left));
    let header = hacking_tile(e, base, 1);
    e.call(TILE_SET_FLOAT, &args![header, 0x1005u32, x, 1u32]);
    let third = e.mem.u32(base + IO_LINES + 8);
    let tile = e.mem.u32(third.wrapping_add(0x2c));
    let top = e.call(TILE_GET_FLOAT, &args![tile, TRAIT_Y]).f64();
    let header = hacking_tile(e, base, 1);
    e.call(
        TILE_SET_FLOAT,
        &args![header, 0x1006u32, x87_store32(top), 1u32],
    );
    let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
    let entry = if block != 0 {
        fn_007694f0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.mem.set_u32(base + IO_LINES + 0xc, entry);
    let header = hacking_tile(e, base, 1);
    let value = e.call(TILE_GET_VALUE, &args![header, 0x1004u32]).u32();
    let entry = e.mem.u32(base + IO_LINES + 0xc);
    e.call(ANIMATING_TEXT_CONNECT, &args![entry, 0u32, value]);
    let mut line_index = 4u32;
    let mut g = 0u32;
    while g < e.get(this, HackingMenu::iTotGuesses) {
        hacking_menu_make_guess_block(e, this);
        g += 1;
    }
    let garbage_count = e.call(STRLEN, &args![GARBAGE_CHARACTERS]).u32();
    if e.get(this, HackingMenu::iWordsHave) == 0 {
        return;
    }
    let positions = e.get(this, HackingMenu::iBufferPos);
    e.call(OPERATOR_DELETE, &args![positions]);
    let (bytes, overflow) = e.get(this, HackingMenu::iWordsHave).overflowing_mul(4);
    let bytes = if overflow { u32::MAX } else { bytes };
    let positions = e.call(OPERATOR_NEW, &args![bytes]).u32();
    e.set(this, HackingMenu::iBufferPos, positions);
    e.with_stack(FILE_CHARACTERS + 1, |e, buffer| {
        place_words(e, this, buffer.addr(), garbage_count);
        write_file_lines(e, this, text, buffer.addr(), &mut line_index);
    });
    let intro = hacking_tile(e, base, 0);
    e.call(TILE_SET_INT, &args![intro, TRAIT_VISIBLE, 0u32]);
    e.set(this, HackingMenu::eStage, 1);
}

/// Fills the garbage buffer and places the words (the loop of
/// [`hacking_menu_make_password_file`]).
fn place_words(e: &mut Engine, this: Ptr<HackingMenu>, buffer: u32, garbage_count: u32) {
    let base = this.addr();
    let mut restarts = 0u32;
    loop {
        for i in 0..FILE_CHARACTERS {
            let pick = e.call(RANDOM_BELOW, &args![garbage_count]).u32();
            let character = e.mem.u8(GARBAGE_CHARACTERS.wrapping_add(pick));
            e.mem.set_u8(buffer + i, character);
        }
        e.mem.set_u8(buffer + FILE_CHARACTERS, 0);
        let mut node = base + WORD_LIST;
        let mut placed = 0u32;
        let mut failures = 0u32;
        while node != 0 && failures < 0x64 {
            let item = e.call(IDENTITY, &args![node]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            let length = e.get(this, HackingMenu::iWordLength);
            let start = e
                .call(
                    RANDOM_BELOW,
                    &args![(0x199u32).wrapping_sub(length).wrapping_sub(2)],
                )
                .u32();
            let mut free = true;
            let mut d = -2i32;
            while free && d < (e.get(this, HackingMenu::iWordLength) as i32).wrapping_add(2) {
                let index = (start as i32).wrapping_add(d);
                free = if index < 0 {
                    true
                } else {
                    let character = e.mem.i8(buffer.wrapping_add(index as u32));
                    !(0x41..=0x5a).contains(&character)
                };
                d += 1;
            }
            if free {
                let length = e.get(this, HackingMenu::iWordLength);
                let item = e.call(IDENTITY, &args![node]).u32();
                let word = e.mem.u32(item);
                e.call(MEMCPY, &args![buffer.wrapping_add(start), word, length]);
                let positions = e.get(this, HackingMenu::iBufferPos);
                e.mem.set_u32(positions.wrapping_add(4 * placed), start);
                placed += 1;
                node = e.call(WORD_AT_FOUR, &args![node]).u32();
                failures = 0;
            } else {
                failures += 1;
            }
        }
        if failures == 0 {
            return;
        }
        restarts += 1;
        if restarts <= 4 {
            continue;
        }
        e.call(PRINT_MESSAGE, &args![WORDS_DO_NOT_FIT_MESSAGE]);
        loop {
            let item = e.call(IDENTITY, &args![node]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            let item = e.call(IDENTITY, &args![node]).u32();
            if e.get(this, HackingMenu::pPassword) == e.mem.u32(item) {
                let item = e.call(IDENTITY, &args![node]).u32();
                let word = e.mem.u32(item);
                e.set(this, HackingMenu::pPassword, word);
            }
            let item = e.call(IDENTITY, &args![node]).u32();
            let word = e.mem.u32(item);
            e.call(OPERATOR_DELETE, &args![word]);
            e.call(SIMPLE_LIST_REMOVE_HEAD, &args![node]);
            let have = e.get(this, HackingMenu::iWordsHave);
            e.set(this, HackingMenu::iWordsHave, have.wrapping_sub(1));
            let positions = e.get(this, HackingMenu::iBufferPos);
            e.mem
                .set_u32(positions.wrapping_add(4 * placed), FILE_CHARACTERS);
            placed += 1;
        }
        let mut other = base + WORD_LIST;
        while other != 0 {
            if e.call(WORD_AT_FOUR, &args![other]).u32() == node {
                e.call(NODE_CLEAR_LINK, &args![other, 0u32]);
            }
            other = e.call(WORD_AT_FOUR, &args![other]).u32();
        }
    }
}

/// Makes the file's terminal lines from the buffer (the tail of
/// [`hacking_menu_make_password_file`]).
fn write_file_lines(
    e: &mut Engine,
    this: Ptr<HackingMenu>,
    text: Ptr,
    buffer: u32,
    line_index: &mut u32,
) {
    let base = this.addr();
    let end = buffer + FILE_CHARACTERS;
    let mut column = 0i32;
    let mut row = 0i32;
    let mut cursor = buffer;
    while cursor < end {
        let mut saved = 0u8;
        let long = (end.wrapping_sub(cursor) as i32) > 0xc;
        if long {
            saved = e.mem.u8(cursor + 0xc);
            e.mem.set_u8(cursor + 0xc, 0);
        }
        e.with_stack(8, |e, address| {
            fn_00768970(e, Ptr::new(base), cursor as u16, address);
            e.call(
                STRING_SPRINTF,
                &args![text, FILE_LINE_FORMAT, address, cursor],
            );
        });
        let file = hacking_tile(e, base, 2);
        let line = new_terminal_line(
            e,
            |e| e.call(WORD_AT_ZERO, &args![text]).u32(),
            file,
            FILE_LINE_TEMPLATE,
            -1,
            2,
        );
        let slot = base.wrapping_add(IO_LINES + 4 * *line_index);
        e.mem.set_u32(slot, line);
        let tile = e.mem.u32(e.mem.u32(slot).wrapping_add(0x2c));
        e.call(TILE_SET_INT_FROM_VALUE, &args![tile, 0xfacu32, column]);
        let tile = e.mem.u32(e.mem.u32(slot).wrapping_add(0x2c));
        e.call(TILE_SET_INT_FROM_VALUE, &args![tile, 0x1004u32, row]);
        e.call(INTERFACE_SINGLETON, &args![]);
        if e.call(GAMEPAD_IN_USE, &args![]).bool() && column == 0 && row == 0 {
            let tile = e.mem.u32(e.mem.u32(slot).wrapping_add(0x2c));
            e.set(this, HackingMenu::pOverTile, tile);
            let over = e.get(this, HackingMenu::pOverTile);
            let screen_x = e.call(TILE_SCREEN_X, &args![over]).f64();
            let x = ftol(e, x87_add(screen_x, e.mem.f64(SECOND_COLUMN_OFFSET)));
            e.set(this, HackingMenu::iCursorX, x);
        }
        column += 1;
        if column == 0x11 {
            column = 0;
            row += 1;
        }
        *line_index += 1;
        if (end.wrapping_sub(cursor) as i32) > 0xc {
            e.mem.set_u8(cursor + 0xc, saved);
        }
        cursor += 0xc;
    }
}

/// Whether the character of `pOverTile`'s text under the cursor is a letter
/// (`isalpha`, 0 or 1), reading the text and the column afresh.
fn over_tile_letter(e: &mut Engine, this: Ptr<HackingMenu>) -> bool {
    let over = e.get(this, HackingMenu::pOverTile);
    let text = e.call(TILE_GET_STRING, &args![over, TRAIT_STRING]).u32();
    let hit = hacking_menu_get_char_hit(e, this);
    let character = e.mem.i8(text.wrapping_add(hit as u32));
    e.call(ISALPHA, &args![i32::from(character)]).i32() != 0
}

/// Whether `pOverTile` is set, has a text and has a letter under the cursor.
fn over_tile_on_letter(e: &mut Engine, this: Ptr<HackingMenu>) -> bool {
    let over = e.get(this, HackingMenu::pOverTile);
    if over == 0 {
        return false;
    }
    if e.call(TILE_GET_STRING, &args![over, TRAIT_STRING]).u32() == 0 {
        return false;
    }
    over_tile_letter(e, this)
}

/// `iCursorX += step`.
fn move_cursor_x(e: &mut Engine, this: Ptr<HackingMenu>, step: i32) {
    let x = e.get(this, HackingMenu::iCursorX);
    e.set(this, HackingMenu::iCursorX, x.wrapping_add(step));
}

/// A float trait of `pOverTile` truncated by `_ftol2`.
fn over_tile_int(e: &mut Engine, this: Ptr<HackingMenu>, trait_id: u32) -> i32 {
    let over = e.get(this, HackingMenu::pOverTile);
    let value = e.call(TILE_GET_FLOAT, &args![over, trait_id]).f64();
    ftol(e, value)
}

/// A float trait of `pOverTile` truncated by `FISTP` (rounding toward zero).
fn over_tile_chop(e: &mut Engine, this: Ptr<HackingMenu>, trait_id: u32) -> u32 {
    let over = e.get(this, HackingMenu::pOverTile);
    let value = e.call(TILE_GET_FLOAT, &args![over, trait_id]).f64();
    x87_fistp64_chop(X87Ext::load64(value)) as u32
}

// Translated from 00767610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::DoGamepad` (Xbox PDB): only with a gamepad and as the top
/// menu (else false). The manager's selected tile becomes `pOverTile`.
/// Before the game, button 9 clicks (`DoClick(-1, 0)`, vtable `+0xc`).
/// In the game, buttons 1..4 move the cursor: 1 and 2 to the line above or
/// below (trait `0xfac` minus or plus 1, within 0..16, `FindTile`), 3 and 4
/// right and left by 17 over a whole word, and past the line's end into the
/// other column (`0x1004`), or back where it was when there is none; a move
/// ends the entry suppression. Answers whether the button was handled.
pub fn hacking_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<HackingMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    e.call(INTERFACE_SINGLETON, &args![]);
    if !e.call(GAMEPAD_IN_USE, &args![]).bool() {
        return false;
    }
    if !e.call(IS_TOP_MENU_ID, &args![HACKING_MENU_CLASS]).bool() {
        return false;
    }
    let manager = e.global::<u32>(INTERFACE_MANAGER);
    let over = e.get(this, HackingMenu::pOverTile);
    e.mem.set_u32(manager.wrapping_add(0xcc), over);
    if (e.get(this, HackingMenu::eStage) as i32) < 2 && button == 9 {
        e.vcall(this.addr(), 0xc, &args![-1i32, 0u32]);
        return true;
    }
    if e.get(this, HackingMenu::eStage) != 2 {
        return false;
    }
    match button.wrapping_sub(1) {
        0 => {
            let line = over_tile_int(e, this, 0xfac);
            if line <= 0 {
                return false;
            }
            e.set(this, HackingMenu::bSuppressEntry, false);
            let column = over_tile_chop(e, this, 0x1004);
            let tile = hacking_menu_find_tile(e, this, line.wrapping_sub(1) as u32, column);
            e.set(this, HackingMenu::pOverTile, tile);
            true
        }
        1 => {
            let line = over_tile_int(e, this, 0xfac);
            if line >= 0x10 {
                return false;
            }
            e.set(this, HackingMenu::bSuppressEntry, false);
            let column = over_tile_chop(e, this, 0x1004);
            let tile = hacking_menu_find_tile(e, this, line.wrapping_add(1) as u32, column);
            e.set(this, HackingMenu::pOverTile, tile);
            true
        }
        2 => {
            let saved = e.get(this, HackingMenu::iCursorX);
            let on_letter = over_tile_on_letter(e, this);
            move_cursor_x(e, this, 0x11);
            while on_letter && hacking_menu_get_char_hit(e, this) != -1 && over_tile_letter(e, this)
            {
                move_cursor_x(e, this, 0x11);
            }
            if hacking_menu_get_char_hit(e, this) == -1 {
                let over = e.get(this, HackingMenu::pOverTile);
                let column = e.call(TILE_GET_FLOAT, &args![over, 0x1004u32]).f64();
                // FCOMP; TEST AH,5; JP: on only when below (ordered).
                if column < e.mem.f64(ONE_DOUBLE) {
                    let line = over_tile_int(e, this, 0xfac);
                    let column = over_tile_int(e, this, 0x1004).wrapping_add(1);
                    let tile = hacking_menu_find_tile(e, this, line as u32, column as u32);
                    if tile != 0 {
                        e.set(this, HackingMenu::bSuppressEntry, false);
                        e.set(this, HackingMenu::pOverTile, tile);
                        let over = e.get(this, HackingMenu::pOverTile);
                        let screen_x = e.call(TILE_SCREEN_X, &args![over]).f64();
                        let x = ftol(e, x87_add(screen_x, e.mem.f64(SECOND_COLUMN_OFFSET)));
                        e.set(this, HackingMenu::iCursorX, x);
                    } else {
                        e.set(this, HackingMenu::iCursorX, saved);
                    }
                } else {
                    e.set(this, HackingMenu::iCursorX, saved);
                }
            } else {
                e.set(this, HackingMenu::bSuppressEntry, false);
            }
            true
        }
        3 => {
            let saved = e.get(this, HackingMenu::iCursorX);
            let on_letter = over_tile_on_letter(e, this);
            move_cursor_x(e, this, -0x11);
            while on_letter && hacking_menu_get_char_hit(e, this) >= 7 && over_tile_letter(e, this)
            {
                move_cursor_x(e, this, -0x11);
            }
            if hacking_menu_get_char_hit(e, this) < 7 {
                let over = e.get(this, HackingMenu::pOverTile);
                let column = e.call(TILE_GET_FLOAT, &args![over, 0x1004u32]).f64();
                // FCOMP; TEST AH,0x41; JNZ: back unless above (ordered).
                if column > e.mem.f64(ZERO_DOUBLE) {
                    let line = over_tile_int(e, this, 0xfac);
                    let column = over_tile_int(e, this, 0x1004).wrapping_sub(1);
                    let tile = hacking_menu_find_tile(e, this, line as u32, column as u32);
                    if tile != 0 {
                        e.set(this, HackingMenu::bSuppressEntry, false);
                        e.set(this, HackingMenu::pOverTile, tile);
                        let over = e.get(this, HackingMenu::pOverTile);
                        let screen_x = e.call(TILE_SCREEN_X, &args![over]).f64();
                        let over = e.get(this, HackingMenu::pOverTile);
                        let width = e.call(TILE_GET_FLOAT, &args![over, TRAIT_WIDTH]).f64();
                        let x = x87_sub(x87_add(width, screen_x), e.mem.f64(ROWS_PER_COLUMN));
                        let x = ftol(e, x);
                        e.set(this, HackingMenu::iCursorX, x);
                    } else {
                        e.set(this, HackingMenu::iCursorX, saved);
                    }
                } else {
                    e.set(this, HackingMenu::iCursorX, saved);
                }
            } else {
                e.set(this, HackingMenu::bSuppressEntry, false);
            }
            true
        }
        _ => false,
    }
}

/// `AnimatingText::ForceFinish` (Xbox PDB).
const ANIMATING_TEXT_FORCE_FINISH: u32 = 0x0070_0110;
/// The buffer string of an `AnimatingText` (`00458b50`, `thiscall`).
const ANIMATING_TEXT_BUFFER: u32 = 0x0045_8b50;
/// Shows or hides an `AnimatingText`'s line (`00726d50`, `thiscall`: flag).
const ANIMATING_TEXT_SHOW: u32 = 0x0072_6d50;
/// The log line formats: `"%s%s"`-style prompt and text (`0101996c`), the
/// likeness line (`010729b4`) and a one-word line (`01019f08`).
const LOG_FORMAT: u32 = 0x0101_996c;
const LIKENESS_FORMAT: u32 = 0x0107_29b4;
const PLAIN_FORMAT: u32 = 0x0101_9f08;
/// `NiTPointerListBase<..>::AddHead` (Xbox PDB name of the shared body;
/// `thiscall`: address of the element).
const POINTER_LIST_ADD_HEAD: u32 = 0x0076_b660;
/// Removes a pointer list's tail and answers its element (`thiscall`).
const POINTER_LIST_REMOVE_TAIL: u32 = 0x004e_df00;
/// `BGSTerminal::SetHacked`, `BGSTerminal::GetHackDifficultyLockLevel`,
/// `BGSTerminal::LockOut` (Xbox PDB).
const TERMINAL_SET_HACKED: u32 = 0x0050_18b0;
const TERMINAL_HACK_LOCK_LEVEL: u32 = 0x0050_11a0;
const TERMINAL_LOCK_OUT: u32 = 0x0050_1930;
/// `ExperiencePoints::GetExperiencePoints` (Xbox PDB, cdecl: kind, level).
const GET_EXPERIENCE_POINTS: u32 = 0x0067_05b0;
/// `TESObjectREFR::GetOwner` (Xbox PDB).
const REFERENCE_GET_OWNER: u32 = 0x0056_7790;
/// A form's type byte (`this` only).
const FORM_TYPE: u32 = 0x0040_1170;
/// The player's call made with the terminal reference and its owner
/// (`008c0ec0`, `actor.cpp`; `thiscall`: reference, owner, -1).
const PLAYER_OWNER_CALL: u32 = 0x008c_0ec0;
/// `MiscStatManager::Increment` (Xbox PDB, cdecl: stat).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;

/// Runs `body` with `count` constructed `BSStringT`s in a local array (the
/// `eh vector` constructor and destructor iterators around it).
fn with_strings<R>(e: &mut Engine, count: u32, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(8 * count, |e, strings| {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![strings, 8u32, count, STRING_CONSTRUCTOR, STRING_DESTRUCTOR],
        );
        let result = body(e, strings.addr());
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![strings, 8u32, count, STRING_DESTRUCTOR],
        );
        result
    })
}

/// `sprintf(string, LOG_FORMAT, prompt, text)`.
fn log_line(e: &mut Engine, string: u32, text: u32) {
    e.call(
        STRING_SPRINTF,
        &args![string, LOG_FORMAT, PROMPT_TEXT, text],
    );
}

/// `log_line` with a setting's string.
fn log_setting(e: &mut Engine, string: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    log_line(e, string, text);
}

/// Scrolls the log by the `count` strings of `strings` (their characters
/// gathered in a local array of `slots` pointers).
fn scroll_log_strings(
    e: &mut Engine,
    this: Ptr<HackingMenu>,
    strings: u32,
    slots: u32,
    count: u32,
) {
    e.with_stack(4 * slots, |e, pointers| {
        for i in 0..slots {
            let characters = e.call(WORD_AT_ZERO, &args![strings + 8 * i]).u32();
            e.mem.set_u32(pointers.addr() + 4 * i, characters);
        }
        hacking_menu_scroll_log(e, this, count, pointers);
    });
}

/// The buffer position of the clicked character: `trunc(fac + 0x1004 *
/// 17) * 12 + GetCharHit - 7`, added at the head of the bonus list.
fn add_used_bonus(e: &mut Engine, this: Ptr<HackingMenu>, tile: u32) {
    let column = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
    let scaled = x87_mul(column, e.mem.f64(ROWS_PER_COLUMN));
    let line = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    let cell = x87_fistp64_chop(X87Ext::load64(x87_add(line, scaled))) as u32;
    let hit = hacking_menu_get_char_hit(e, this) as u32;
    let position = cell.wrapping_mul(0xc).wrapping_add(hit).wrapping_sub(7);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), position);
        e.call(
            POINTER_LIST_ADD_HEAD,
            &args![this.addr() + USED_BONUSES, slot],
        );
    });
}

// Translated from 00766b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::DoClick` (Xbox PDB). The clicked tile is `pOverTile` with a
/// gamepad, else `tile`, else `pTiles[id]`. In the intro a click after the
/// retry time builds the password file; in the pre-game it finishes every
/// terminal line. In the game, a click on a file line takes the entry
/// text: a bracket run (two or more characters, not a letter first)
/// removes a dud (or, one time in four before a reset was used, resets the
/// tries and the guess blocks) and is marked used; the password logs
/// success (5 lines, sound 1, the terminal hacked, the experience for its
/// lock level given to the player, the owner call, stat 5) and finishes the
/// menu after 3 s; another word logs the likeness and costs a try (none
/// left: the lock-out lines, `eStage` 3 and the terminal locked out), plays
/// sound 2 and deletes the last guess block. Every guess then rewrites the
/// tries line, clears the entry and suppresses it.
// Translation incomplete: 00766b80 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_do_click(e: &mut Engine, this: Ptr<HackingMenu>, id: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    e.call(INTERFACE_SINGLETON, &args![]);
    if e.call(GAMEPAD_IN_USE, &args![]).bool() {
        tile = e.get(this, HackingMenu::pOverTile);
    }
    if tile == 0 && (0..TILE_COUNT).contains(&id) {
        tile = hacking_tile(e, base, id as u32);
    }
    match e.get(this, HackingMenu::eStage) {
        0 => {
            let now = e.call(GAME_TIME, &args![]).u32();
            if now > e.global::<u32>(RETRY_TIME) {
                hacking_menu_make_password_file(e, this);
            }
            return;
        }
        1 => {
            for i in 0..IO_LINE_COUNT {
                let line = e.mem.u32(base + IO_LINES + 4 * i);
                if line != 0 {
                    e.call(ANIMATING_TEXT_FORCE_FINISH, &args![line]);
                }
            }
            return;
        }
        2 => {}
        _ => return,
    }
    if tile == 0 || e.call(TILE_PARENT, &args![tile]).u32() != hacking_tile(e, base, 2) {
        return;
    }
    e.call(ANIMATING_TEXT_FORCE_FINISH, &args![base + ENTRY_TEXT]);
    let entry = e
        .call(ANIMATING_TEXT_BUFFER, &args![base + ENTRY_TEXT])
        .u32();
    if entry == 0 || e.mem.i8(entry) == 0 {
        return;
    }
    e.call(SOUND_HANDLE_PLAY, &args![base + SOUNDS + 0x30, 0u32]);
    let password = e.get(this, HackingMenu::pPassword);
    let common = hacking_menu_common_positions(e, Ptr::new(base), entry, password);
    let first = i32::from(e.mem.i8(entry));
    let bracket =
        e.call(ISALPHA, &args![first]).i32() == 0 && e.call(STRLEN, &args![entry]).u32() > 1;
    if bracket {
        with_strings(e, 3, |e, strings| {
            log_line(e, strings, entry);
            let count = if e.get(this, HackingMenu::iWordsHave) == 1 {
                log_setting(e, strings + 8, 0x011d_26dc);
                2
            } else if e.get(this, HackingMenu::bUsedReset)
                || e.call(RANDOM_BELOW, &args![4u32]).u32() != 0
            {
                fn_0076a730(e, this);
                add_used_bonus(e, this, tile);
                log_setting(e, strings + 8, 0x011d_1e78);
                2
            } else {
                if e.get(this, HackingMenu::iNumGuesses) == 1 {
                    let text = e
                        .call(SETTING_STRING_004C69F0, &args![HEADER_SETTING_TWO])
                        .u32();
                    let second = e.mem.u32(base + IO_LINES + 4);
                    e.call(ANIMATING_TEXT_RESET_BUFFER, &args![second, text]);
                    let second = e.mem.u32(base + IO_LINES + 4);
                    e.call(ANIMATING_TEXT_SHOW, &args![second, 0u32]);
                    let second = e.mem.u32(base + IO_LINES + 4);
                    e.call(ANIMATING_TEXT_FORCE_FINISH, &args![second]);
                }
                log_setting(e, strings + 8, 0x011d_47e0);
                log_setting(e, strings + 0x10, 0x011d_4f6c);
                let total = e.get(this, HackingMenu::iTotGuesses);
                e.set(this, HackingMenu::iNumGuesses, total);
                e.set(this, HackingMenu::bUsedReset, true);
                add_used_bonus(e, this, tile);
                while e.call(POINTER_LIST_COUNT, &args![base + GUESS_TILES]).u32()
                    < e.get(this, HackingMenu::iTotGuesses)
                {
                    hacking_menu_make_guess_block(e, this);
                }
                3
            };
            scroll_log_strings(e, this, strings, 3, count);
        });
    } else if common == e.get(this, HackingMenu::iWordLength) {
        with_strings(e, 5, |e, strings| {
            log_line(e, strings, entry);
            for (i, setting) in [0x011d_2604u32, 0x011d_3444, 0x011d_4858, 0x011d_4ae0]
                .into_iter()
                .enumerate()
            {
                log_setting(e, strings + 8 * (i as u32 + 1), setting);
            }
            scroll_log_strings(e, this, strings, 5, 5);
            e.call(
                SOUND_HANDLE_PLAY_AFTER,
                &args![base + SOUNDS + 0xc, 0xfau32, 0u32],
            );
            let now = e.call(GAME_TIME, &args![]).u32();
            e.set(this, HackingMenu::dwAccessTime, now.wrapping_add(0xbb8));
            e.set(this, HackingMenu::eStage, 4);
            if e.get(this, HackingMenu::pTerminal) != 0 {
                let reference = e.get(this, HackingMenu::pTerminalRef);
                e.call(TERMINAL_SET_HACKED, &args![reference, 1u32]);
                let terminal = e.get(this, HackingMenu::pTerminal);
                let reference = e.get(this, HackingMenu::pTerminalRef);
                let level = e
                    .call(TERMINAL_HACK_LOCK_LEVEL, &args![terminal, reference])
                    .u32();
                let points = e.call(GET_EXPERIENCE_POINTS, &args![2u32, level]).u32();
                let player = e.global::<u32>(PLAYER);
                e.vcall(player, 0x488, &args![points]);
                let reference = e.get(this, HackingMenu::pTerminalRef);
                let owner = e.call(REFERENCE_GET_OWNER, &args![reference]).u32();
                if owner != 0 {
                    // The type test only chooses which local keeps the owner.
                    e.call(FORM_TYPE, &args![owner]);
                    let reference = e.get(this, HackingMenu::pTerminalRef);
                    let player = e.global::<u32>(PLAYER);
                    e.call(PLAYER_OWNER_CALL, &args![player, reference, owner, -1i32]);
                }
            }
            e.call(MISC_STAT_INCREMENT, &args![5u32]);
        });
    } else {
        with_strings(e, 4, |e, strings| {
            log_line(e, strings, entry);
            log_setting(e, strings + 8, 0x011d_26dc);
            let guesses = e.get(this, HackingMenu::iNumGuesses).wrapping_sub(1);
            e.set(this, HackingMenu::iNumGuesses, guesses);
            if e.get(this, HackingMenu::iNumGuesses) != 0 {
                let text = e.call(SETTING_STRING, &args![0x011d_2b2cu32]).u32();
                let length = e.get(this, HackingMenu::iWordLength);
                e.call(
                    STRING_SPRINTF,
                    &args![
                        strings + 0x10,
                        LIKENESS_FORMAT,
                        PROMPT_TEXT,
                        common,
                        length,
                        text
                    ],
                );
                scroll_log_strings(e, this, strings, 3, 3);
                if e.get(this, HackingMenu::iNumGuesses) == 1 {
                    let text = e
                        .call(SETTING_STRING_004C69F0, &args![0x011d_1f74u32])
                        .u32();
                    let second = e.mem.u32(base + IO_LINES + 4);
                    e.call(ANIMATING_TEXT_RESET_BUFFER, &args![second, text]);
                    let second = e.mem.u32(base + IO_LINES + 4);
                    e.call(ANIMATING_TEXT_FORCE_FINISH, &args![second]);
                    let second = e.mem.u32(base + IO_LINES + 4);
                    e.call(ANIMATING_TEXT_SHOW, &args![second, 1u32]);
                }
            } else {
                log_setting(e, strings + 0x10, 0x011d_3144);
                let text = e.call(SETTING_STRING, &args![0x011d_339cu32]).u32();
                e.call(STRING_SPRINTF, &args![strings + 0x18, PLAIN_FORMAT, text]);
                scroll_log_strings(e, this, strings, 4, 4);
                e.set(this, HackingMenu::eStage, 3);
                if e.get(this, HackingMenu::pTerminal) != 0 {
                    let reference = e.get(this, HackingMenu::pTerminalRef);
                    e.call(TERMINAL_LOCK_OUT, &args![reference, 1u32]);
                }
            }
            e.call(
                SOUND_HANDLE_PLAY_AFTER,
                &args![base + SOUNDS + 0x18, 0xfau32, 0u32],
            );
            let block = e
                .call(POINTER_LIST_REMOVE_TAIL, &args![base + GUESS_TILES])
                .u32();
            delete_object(e, block);
        });
    }
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCTOR, &args![text]);
        let label = e.call(SETTING_STRING, &args![GUESSES_SETTING]).u32();
        let guesses = e.get(this, HackingMenu::iNumGuesses);
        e.call(STRING_SPRINTF, &args![text, GUESSES_FORMAT, guesses, label]);
        let characters = e.call(WORD_AT_ZERO, &args![text]).u32();
        let third = e.mem.u32(base + IO_LINES + 8);
        e.call(ANIMATING_TEXT_RESET_BUFFER, &args![third, characters]);
        let third = e.mem.u32(base + IO_LINES + 8);
        e.call(ANIMATING_TEXT_FORCE_FINISH, &args![third]);
        hacking_menu_clear_entry(e, this);
        e.set(this, HackingMenu::bSuppressEntry, true);
        e.call(STRING_DESTRUCTOR, &args![text]);
    });
}

/// `Tile::GetMenuByClass` (Xbox PDB, cdecl: class).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::GetMaximumDepth` (Xbox PDB, no arguments; ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `SystemColorManager::GetColor` (Xbox PDB; `thiscall`, RET 8: index, colour).
const SYSTEM_COLOR_GET_COLOR: u32 = 0x0071_9060;
/// `BGSSaveFormBuffer::GetForm` (Xbox PDB name of the shared body;
/// `thiscall` on the terminal reference).
const REFERENCE_FORM: u32 = 0x007a_f430;
/// The test on that form that switches the system colour to white
/// (`00757e80`, `computersmenu.cpp`).
const FORM_WANTS_WHITE: u32 = 0x0075_7e80;
/// Builds an `NiColorA` (`thiscall`: r, g, b, a; answers `this`).
const COLOR_SET: u32 = 0x0041_4430;
/// The two settings copied into the globals `0119f27c` and `0119f280`.
const COPIED_SETTING_ONE: u32 = 0x011d_42d0;
const COPIED_SETTING_TWO: u32 = 0x011d_2934;
const COPIED_GLOBAL_ONE: u32 = 0x0119_f27c;
const COPIED_GLOBAL_TWO: u32 = 0x0119_f280;
/// The tile reader of the interface manager (`this` only).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`this` + file name).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Menu::SetMenuTile` (Xbox PDB; `this`, tile, flag) and
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The menu file name and the missing-tile message.
const MENU_FILE: u32 = 0x0107_2990;
const MISSING_TILE_MESSAGE: u32 = 0x0107_2954;
/// The `double` stacking types the depth is set for.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
/// The settings of the two lock-out tiles' texts.
const LOCKOUT_SETTING_ONE: u32 = 0x011d_31bc;
const LOCKOUT_SETTING_TWO: u32 = 0x011d_30e4;
/// `PlayerCharacter::GetTerminalAccess` (Xbox PDB; `this`, reference).
const PLAYER_TERMINAL_ACCESS: u32 = 0x0096_6c60;
/// `BGSTerminal::GetHackMiniGameValue` and
/// `BGSTerminal::GetMinSkillRequiredToHack` (Xbox PDB; `this`, reference).
const TERMINAL_MINI_GAME_VALUE: u32 = 0x0050_1270;
const TERMINAL_MIN_SKILL: u32 = 0x0050_17a0;
/// `InterfaceManager::TutorialManager::ShowMessage` (Xbox PDB).
const SHOW_TUTORIAL: u32 = 0x0071_8630;
/// `ActorValueOwner::GetClampedActorFloatValue` (Xbox PDB; ST0) and
/// `BGSEntryPoint::HandleEntryPoint` (cdecl: point, actor, address of the
/// value).
const CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef50;
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The two settings bounding the number of words.
const WORD_COUNT_SETTING_ONE: u32 = 0x011d_3318;
const WORD_COUNT_SETTING_TWO: u32 = 0x011d_3c4c;
/// The larger and the smaller of two floats (cdecl; ST0).
const FLOAT_MAX: u32 = 0x0040_4010;
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `100.0f`, `100.0` and `0.5`.
const SKILL_CAP: u32 = 0x0101_6410;
const SKILL_RANGE: u32 = 0x0101_7a40;
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The intro lines: the setting of each text, its row (in `iYInterval`
/// steps from `iYStart`) and its kind.
const INTRO_LINES: [(u32, i32, u8); 14] = [
    (0x011d_3f58, 0, 0),
    (0x011d_3a38, 2, 0),
    (0x011d_3c64, 4, 1),
    (0x011d_4c00, 6, 0),
    (0x011d_4d5c, 8, 1),
    (0x011d_3030, 9, 1),
    (0x011d_4b94, 0xb, 0),
    (0x011d_1f44, 0xc, 0),
    (0x011d_1d9c, 0xd, 0),
    (0x011d_306c, 0xe, 0),
    (0x011d_484c, 0xf, 0),
    (0x011d_4f18, 0x10, 0),
    (0x011d_4690, 0x11, 0),
    (0x011d_2130, 0x13, 1),
];

// Translated from 00765b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HackingMenu::Create` (Xbox PDB, cdecl: the terminal reference): deletes
/// an open menu of the class, takes the interface manager as `pim`, saves
/// the system colour 3 (white for a form `00757e80` accepts), copies two
/// settings into globals and reads the menu file. For a menu of this class
/// with all eight tiles (else prints the missing index and answers 0): sets
/// the depth for the two stacking types and the lock-out texts. A terminal
/// the player is locked out of (access 2) finishes at once. Otherwise sets
/// the terminal, the word length (the terminal's mini-game value, 12
/// without one, at most 12), the tutorial flag, and the word count from the
/// player's science against the terminal's minimum skill (with entry point
/// `0x1e`) between the two word-count settings, at most 20 (`Prepare`);
/// connects the entry text, sets its interval and makes the 14 intro lines.
/// Then plays sound 0 and prepares the menu for showing; answers the root
/// tile. A menu of another class is deleted (when it has a tile) and the
/// result is 0.
// Translation incomplete: 00765b80 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn hacking_menu_create(e: &mut Engine, reference: u32) -> u32 {
    let class = fn_00766a40(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    delete_object(e, old);
    let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.set_global(INTERFACE_MANAGER, manager);
    let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
    let depth = x87_store32(depth);
    let colors = e.call(SYSTEM_COLOR_GET_INSTANCE, &args![]).u32();
    // 00765c0b: `00718b60` takes no word; the pushed 3 and colour are the two words of
    // `00719060` (RET 8).
    e.call(SYSTEM_COLOR_GET_COLOR, &args![colors, 3u32, SAVE_COLOR]);
    let form = e.call(REFERENCE_FORM, &args![reference]).u32();
    if e.call(FORM_WANTS_WHITE, &args![form]).bool() {
        e.with_stack(0x10, |e, color| {
            let one = 1.0f32.to_bits();
            let color = e
                .call(COLOR_SET, &args![color, one, one, one, 0.0f32.to_bits()])
                .u32();
            // 00765c54: as above, the 3 and the colour are `007190e0`'s words.
            let colors = e.call(SYSTEM_COLOR_GET_INSTANCE, &args![]).u32();
            e.call(SYSTEM_COLOR_SET_COLOR, &args![colors, 3u32, color]);
        });
    }
    let value = e.call(SETTING_VALUE, &args![COPIED_SETTING_ONE]).u32();
    let value = e.mem.u32(value);
    e.set_global(COPIED_GLOBAL_ONE, value);
    let value = e.call(SETTING_VALUE, &args![COPIED_SETTING_TWO]).u32();
    let value = e.mem.u32(value);
    e.set_global(COPIED_GLOBAL_TWO, value);
    let manager = e.global::<u32>(INTERFACE_MANAGER);
    let reader = e.call(TILE_READER, &args![manager]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    if menu != 0 {
        let menu_class = e.vcall(menu, 0x34, &args![]).u32();
        if menu_class == fn_00766a40(e) {
            return finish_hacking_create(e, reference, root, menu, depth);
        }
    }
    if menu != 0 && e.call(WORD_AT_FOUR, &args![menu]).u32() != 0 {
        delete_object(e, menu);
    }
    0
}

/// The part of [`hacking_menu_create`] once the menu of the class was read.
fn finish_hacking_create(e: &mut Engine, reference: u32, root: u32, menu: u32, depth: u32) -> u32 {
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    e.set_global(HACKING_MENU, menu);
    let complete = e.with_stack(4, |e, index| {
        let complete = fn_00766a50(e, Ptr::new(me_hacking(e)), index);
        if !complete {
            let missing = e.mem.u32(index.addr());
            e.call(PRINT_MESSAGE, &args![MISSING_TILE_MESSAGE, missing]);
        }
        complete
    });
    if !complete {
        return 0;
    }
    let kind = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
    let matches = kind == e.mem.f64(STACKING_TYPE_ONE) || {
        let again = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
        again == e.mem.f64(STACKING_TYPE_TWO)
    };
    if matches {
        e.call(
            TILE_SET_FLOAT,
            &args![root, 0xfadu32, x87_quiet_bits(depth), 1u32],
        );
    }
    for (index, setting) in [(6u32, LOCKOUT_SETTING_ONE), (7, LOCKOUT_SETTING_TWO)] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = hacking_tile(e, me_hacking(e), index);
        e.call(TILE_SET_STRING, &args![tile, TRAIT_STRING, text, 1u32]);
    }
    let form = if reference != 0 {
        e.call(REFERENCE_FORM, &args![reference]).u32()
    } else {
        0
    };
    let locked_out = form != 0 && {
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_TERMINAL_ACCESS, &args![player, reference])
            .u32()
            == 2
    };
    let me = Ptr::<HackingMenu>::new(me_hacking(e));
    if locked_out {
        e.call(TILE_SET_INT, &args![root, 0x1006u32, 1u32]);
        e.set(
            Ptr::<HackingMenu>::new(me_hacking(e)),
            HackingMenu::eStage,
            4,
        );
        let cursor = hacking_tile(e, me_hacking(e), 4);
        e.call(TILE_SET_INT, &args![cursor, TRAIT_VISIBLE, 0u32]);
    } else {
        e.set(me, HackingMenu::pTerminal, form);
        e.set(me, HackingMenu::pTerminalRef, reference);
        let length = if e.get(me, HackingMenu::pTerminal) != 0 {
            let terminal = e.get(me, HackingMenu::pTerminal);
            let reference = e.get(me, HackingMenu::pTerminalRef);
            e.call(TERMINAL_MINI_GAME_VALUE, &args![terminal, reference])
                .u32()
        } else {
            0xc
        };
        e.set(me, HackingMenu::iWordLength, length);
        let length = e.get(me, HackingMenu::iWordLength);
        let length = e.call(MIN_OF_TWO, &args![0xcu32, length]).u32();
        e.set(me, HackingMenu::iWordLength, length);
        let class = fn_00766a40(e);
        let manager = e.global::<u32>(INTERFACE_MANAGER);
        let tutorial = e
            .call(
                SHOW_TUTORIAL,
                &args![manager.wrapping_add(0x4d4), 0x13u32, class, 0x200u32],
            )
            .bool();
        e.set(me, HackingMenu::bTutorial, tutorial);
        let count = word_count(e, me, reference);
        hacking_menu_prepare(e, Ptr::new(me_hacking(e)), count);
        e.call(TILE_SET_STRING, &args![root, 0x1004u32, PROMPT_TEXT, 1u32]);
        let entry = hacking_tile(e, me_hacking(e), 5);
        let visible = e.call(TILE_GET_VALUE_Q, &args![entry, TRAIT_VISIBLE]).u32();
        let entry = hacking_tile(e, me_hacking(e), 5);
        let string = e.call(TILE_GET_VALUE, &args![entry, TRAIT_STRING]).u32();
        let text = me_hacking(e).wrapping_add(ENTRY_TEXT);
        e.call(ANIMATING_TEXT_CONNECT, &args![text, string, visible]);
        let setting = LINE_INTERVAL_SETTINGS[1];
        let word = e.call(SETTING_VALUE, &args![setting]).u32();
        if e.mem.u32(word) != 0 {
            let word = e.call(SETTING_VALUE, &args![setting]).u32();
            let value = e.mem.u32(word);
            let ratio = x87_store32(x87_div(e.mem.f64(INTERVAL_SCALE), f64::from(value)));
            let rounded = e
                .call(ROUND_TO_MULTIPLE, &args![ratio, 1.0f32.to_bits()])
                .f64();
            let interval = x87_fistp64_chop(X87Ext::load64(rounded)) as u32;
            let text = me_hacking(e).wrapping_add(ENTRY_TEXT);
            fn_00766a10(e, Ptr::new(text), interval);
        }
        for (i, (setting, step, kind)) in INTRO_LINES.into_iter().enumerate() {
            let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
            let line = if block != 0 {
                let me = Ptr::<HackingMenu>::new(me_hacking(e));
                let y = e
                    .get(me, HackingMenu::iYInterval)
                    .wrapping_mul(step)
                    .wrapping_add(e.get(me, HackingMenu::iYStart));
                let intro = hacking_tile(e, me_hacking(e), 0);
                let text = e.call(SETTING_STRING_004C69F0, &args![setting]).u32();
                hacking_menu_terminal_io_terminal_io(
                    e,
                    Ptr::new(block),
                    text,
                    intro,
                    PROMPT_TEMPLATE,
                    y,
                    kind,
                )
                .addr()
            } else {
                0
            };
            let slot = me_hacking(e).wrapping_add(IO_LINES + 4 * i as u32);
            e.mem.set_u32(slot, line);
        }
    }
    let sound = me_hacking(e).wrapping_add(SOUNDS);
    e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    root
}

/// The number of words of [`hacking_menu_create`]: the player's science
/// (actor value `0x28`, through entry point `0x1e`) above the terminal's
/// minimum skill (clamped to 0..100) as a share of the range left, mapped
/// from the larger to the smaller word-count setting (both at most 20),
/// rounded (`004bd510`), at most 20.
fn word_count(e: &mut Engine, me: Ptr<HackingMenu>, reference: u32) -> u32 {
    let player = e.global::<u32>(PLAYER);
    let science = e
        .call(
            CLAMPED_ACTOR_VALUE,
            &args![player.wrapping_add(0xa4), 0x28u32],
        )
        .f64();
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), x87_store32(science));
        let player = e.global::<u32>(PLAYER);
        e.call(HANDLE_ENTRY_POINT, &args![0x1eu32, player, value]);
        let word = e.call(SETTING_VALUE, &args![WORD_COUNT_SETTING_ONE]).u32();
        let most = e.call(MIN_OF_TWO, &args![0x14u32, e.mem.u32(word)]).u32();
        let word = e.call(SETTING_VALUE, &args![WORD_COUNT_SETTING_TWO]).u32();
        let least = e.call(MIN_OF_TWO, &args![most, e.mem.u32(word)]).u32();
        let terminal = e.get(me, HackingMenu::pTerminal);
        let required = e
            .call(TERMINAL_MIN_SKILL, &args![terminal, reference])
            .f64();
        let required = e
            .call(FLOAT_MAX, &args![0.0f32.to_bits(), x87_store32(required)])
            .f64();
        let cap = x87_quiet_bits(e.mem.u32(SKILL_CAP));
        let required = e.call(FLOAT_MIN, &args![cap, x87_store32(required)]).f64();
        let required = x87_store32(required);
        let above = x87_store32(x87_sub(
            x87_load32(e.mem.u32(value.addr())),
            x87_m32(required),
        ));
        let above = e.call(FLOAT_MAX, &args![0.0f32.to_bits(), above]).f64();
        let above = x87_store32(above);
        let range = x87_store32(x87_sub(e.mem.f64(SKILL_RANGE), x87_load32(required)));
        // FCOMP; TEST AH,0x44; JNP: the constant only when equal to 0.0.
        let share = if x87_load32(range) == e.mem.f64(ZERO_DOUBLE) {
            e.mem.f64(HALF_DOUBLE)
        } else {
            x87_sub(1.0, x87_div(x87_load32(above), x87_m32(range)))
        };
        let share = x87_store32(share);
        let spread = x87_store32(x87_mul(f64::from(most.wrapping_sub(least)), x87_m32(share)));
        let rounded = e
            .call(ROUND_TO_MULTIPLE, &args![spread, 1.0f32.to_bits()])
            .f64();
        let extra = x87_fistp64_chop(X87Ext::load64(rounded)) as u32;
        let count = extra.wrapping_add(least);
        if count > 0x14 {
            0x14
        } else {
            count
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0076_a3d0, fn_0076a3d0(Ptr, Ptr) -> bool),
        entry!(0x0076_97f0, fn_007697f0(Ptr) -> f64),
        entry!(
            0x0076_54f0,
            hacking_menu_hacking_menu(Ptr<HackingMenu>) -> Ptr<HackingMenu>
        ),
        entry!(0x0076_5810, fn_00765810(Ptr)),
        entry!(0x0076_5870, hacking_menu_get_class(Ptr<HackingMenu>) -> u32),
        entry!(
            0x0076_5880,
            hacking_menu_attach_tile_by_id(Ptr<HackingMenu>, i32, u32)
        ),
        entry!(
            0x0076_58b0,
            hacking_menu_scalar_deleting_destructor(Ptr<HackingMenu>, u32) -> Ptr<HackingMenu>
        ),
        entry!(0x0076_58e0, hacking_menu_destructor(Ptr<HackingMenu>)),
        entry!(0x0076_5b50, fn_00765b50(Ptr, u32) -> Ptr),
        entry!(0x0076_6a10, fn_00766a10(Ptr, u32)),
        entry!(0x0076_6a40, fn_00766a40() -> u32),
        entry!(0x0076_6a50, fn_00766a50(Ptr<HackingMenu>, Ptr) -> bool),
        entry!(0x0076_6aa0, hacking_menu_close()),
        entry!(
            0x0076_7b80,
            hacking_menu_do_gamepad_up_event(Ptr<HackingMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0076_7ff0,
            hacking_menu_do_leave(Ptr<HackingMenu>, u32, u32)
        ),
        entry!(0x0076_8020, hacking_menu_clear_entry(Ptr<HackingMenu>)),
        entry!(0x0076_87b0, fn_007687b0(Ptr)),
        entry!(0x0076_87d0, fn_007687d0(Ptr)),
        entry!(0x0076_8810, fn_00768810(Ptr, u32) -> Ptr),
        entry!(0x0076_8920, fn_00768920(Ptr, u32)),
        entry!(0x0076_8970, fn_00768970(Ptr, u16, Ptr)),
        entry!(
            0x0076_8a10,
            hacking_menu_common_positions(Ptr, u32, u32) -> u32
        ),
        entry!(0x0076_94f0, fn_007694f0(Ptr) -> Ptr),
        entry!(
            0x0076_9700,
            hacking_menu_get_char_hit(Ptr<HackingMenu>) -> i32
        ),
        entry!(0x0076_a3b0, fn_0076a3b0(Ptr, u32) -> bool),
        entry!(0x0076_a3f0, hacking_menu_do_shutdown(Ptr<HackingMenu>)),
        entry!(0x0076_a510, fn_0076a510(Ptr<HackingMenu>)),
        entry!(
            0x0076_a540,
            hacking_menu_transition_to_computers_menu(Ptr<HackingMenu>)
        ),
        entry!(0x0076_a670, hacking_menu_get_garbage_string(Ptr, u32, u32)),
        entry!(0x0076_a9d0, hacking_menu_flash_cursor(Ptr<HackingMenu>)),
        entry!(
            0x0076_ab40,
            hacking_menu_update_cursor_pos(Ptr<HackingMenu>)
        ),
        entry!(
            0x0076_ac10,
            hacking_menu_update_scrolling_cursor(Ptr<HackingMenu>, Ptr)
        ),
        entry!(0x0076_ad50, hacking_menu_make_guess_block(Ptr<HackingMenu>)),
        entry!(0x0076_ae30, fn_0076ae30(Ptr, i32, Ptr) -> u8),
        entry!(0x0076_ae90, fn_0076ae90(Ptr) -> u8),
        entry!(0x0076_af60, fn_0076af60(u8, u8) -> u8),
        entry!(0x0076_af90, fn_0076af90(Ptr, Ptr, Ptr, u8) -> Ptr),
        entry!(0x0076_b090, fn_0076b090(Ptr, Ptr) -> Ptr),
        entry!(0x0076_b0c0, fn_0076b0c0(Ptr, Ptr) -> Ptr),
        entry!(0x0076_b200, fn_0076b200(Ptr, Ptr) -> Ptr),
        entry!(0x0076_b240, fn_0076b240(Ptr, u32)),
        entry!(0x0076_b2a0, fn_0076b2a0(Ptr)),
        entry!(0x0076_b2e0, fn_0076b2e0() -> i32),
        entry!(
            0x0076_b300,
            hacking_menu_terminal_io_terminal_io(Ptr, u32, u32, u32, i32, u8) -> Ptr
        ),
        entry!(0x0076_b560, fn_0076b560(Ptr)),
        entry!(0x0076_5b80, hacking_menu_create(u32) -> u32),
        entry!(
            0x0076_6b80,
            hacking_menu_do_click(Ptr<HackingMenu>, i32, u32)
        ),
        entry!(
            0x0076_7610,
            hacking_menu_do_gamepad(Ptr<HackingMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0076_7bb0,
            hacking_menu_find_tile(Ptr<HackingMenu>, u32, u32) -> u32
        ),
        entry!(0x0076_7c90, hacking_menu_do_idle(Ptr<HackingMenu>)),
        entry!(
            0x0076_7ef0,
            hacking_menu_scroll_log(Ptr<HackingMenu>, u32, Ptr)
        ),
        entry!(0x0076_8070, hacking_menu_prepare(Ptr<HackingMenu>, u32)),
        entry!(
            0x0076_8aa0,
            hacking_menu_make_password_file(Ptr<HackingMenu>)
        ),
        entry!(0x0076_9520, fn_00769520(Ptr<HackingMenu>, Ptr) -> u32),
        entry!(0x0076_9830, hacking_menu_do_intro(Ptr<HackingMenu>)),
        entry!(0x0076_9b50, fn_00769b50(Ptr<HackingMenu>)),
        entry!(0x0076_a730, fn_0076a730(Ptr<HackingMenu>)),
    ]
}

#[cfg(test)]
mod scale_tests {
    use super::*;

    #[test]
    fn fn_007697f0_scales_by_1280_over_the_size() {
        let mut e = Engine::new();
        e.map(0x0106_e000, 0x1000);
        e.mem.set_f64(SCALE_0106E960, 1280.0);
        e.register(SCREEN_SIZE_004DC1F0, |_, _| 640u32.into_ret());
        let this = e.mem.alloc(0x4d4);
        e.mem.set_f32(this + 0x4ac, 0.5);
        e.mem.set_f32(this + 0x4d0, 3.0);
        assert_eq!(e.call(0x0076_97f0, &[this]).f64(), 3.0);
        // A negative size divides as a signed integer.
        e.register(SCREEN_SIZE_004DC1F0, |_, _| (-640i32).into_ret());
        assert_eq!(e.call(0x0076_97f0, &[this]).f64(), -3.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_0076a3d0_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(TEST_TARGET_00700270, |_, a| (0x100 | (a[1] & 1)).into_ret());
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0076_a3d0, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == TEST_TARGET_00700270)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
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

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
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

    /// `_ftol2` as a double: truncation of the leading `f64`.
    fn real_ftol(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32).into_ret()
        });
    }

    fn copy_memory(e: &mut Engine) {
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    /// Maps the pages of the given addresses.
    fn pages(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            if !e.mem.is_mapped(address) {
                e.map(address & !0xfff, 0x1000);
            }
        }
    }

    /// A menu with its eight tiles (`0x100 + 4 * i`) as `pMe`.
    fn menu(e: &mut Engine) -> Ptr<HackingMenu> {
        pages(e, &[HACKING_MENU]);
        let this = e.new_object::<HackingMenu>();
        for i in 0..8 {
            e.mem.set_u32(this.addr() + TILES + 4 * i, 0x100 + 4 * i);
        }
        e.set_global(HACKING_MENU, this.addr());
        this
    }

    #[test]
    fn constructor_builds_members_and_sounds() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                MENU_CONSTRUCTOR,
                ANIMATING_TEXT_CONSTRUCTOR,
                VECTOR_CONSTRUCTOR_ITERATOR,
                POINTER_LIST_CONSTRUCTOR,
                WORD_LIST_CONSTRUCTOR,
                MEMSET,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCTOR,
                AUDIO_MANAGER,
            ],
        );
        answer(&mut e, GET_SOUND_HANDLE_BY_NAME, 0x7777);
        let this = e.new_object::<HackingMenu>();
        e.set(this, HackingMenu::iLastOffset, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_54f0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), HACKING_MENU_VTABLE);
        assert_eq!(e.get(this, HackingMenu::iYInterval), 0x1e);
        assert_eq!(e.get(this, HackingMenu::iLastOffset), -1);
        let names = calls_to(&e, GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(names.len(), 5);
        assert_eq!(names[0][2..], [0x0107_1fec, 0x31]);
        assert_eq!(names[1][2..], [0x0107_28d8, 0x21]);
        let assigns = calls_to(&e, SOUND_HANDLE_ASSIGN);
        assert_eq!(assigns[4], vec![this.addr() + 0x15c, 0x7777]);
        assert_eq!(
            calls_to(&e, MEMSET).last().unwrap(),
            &vec![this.addr() + 0x28, 0, 0xb8]
        );
    }

    #[test]
    fn word_list_destructor_frees_words_then_the_list() {
        let mut e = Engine::new();
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 0x5000);
        // The head removal clears the head.
        e.register(SIMPLE_LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        stub(&mut e, &[OPERATOR_DELETE, SIMPLE_LIST_DESTRUCTOR]);
        e.call_log = Some(vec![]);
        e.call(0x0076_5810, &args![list]);
        assert_eq!(
            order(&e),
            vec![
                0x0076_5810,
                OPERATOR_DELETE,
                SIMPLE_LIST_REMOVE_HEAD,
                SIMPLE_LIST_DESTRUCTOR
            ]
        );
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x5000]]);
        e.call_log = Some(vec![]);
        e.call(0x0076_b2a0, &args![list]);
        assert_eq!(order(&e), vec![0x0076_b2a0]);
    }

    #[test]
    fn class_numbers_and_attach_tile() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0076_5870, &args![0u32]).u32(), 0x41f);
        assert_eq!(e.call(0x0076_6a40, &args![]).u32(), 0x41f);
        let this = e.new_object::<HackingMenu>();
        e.call(0x0076_5880, &args![this, 7i32, 0x99u32]);
        e.call(0x0076_5880, &args![this, 8i32, 0x98u32]);
        e.call(0x0076_5880, &args![this, -1i32, 0x97u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x128), 0x99);
        assert_eq!(e.mem.u32(this.addr() + 0x12c), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x108), 0);
    }

    #[test]
    fn scalar_deleting_destructor_tears_down_and_frees() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[RETRY_TIME]);
        e.set(this, HackingMenu::iBufferPos, 0x4440);
        let line = e.mem.alloc(0x34);
        e.mem.set_u32(this.addr() + 0x28 + 4 * 3, line);
        let mut seen = 0u32;
        e.register_double(POINTER_LIST_IS_EMPTY, move |_, _| {
            seen += 1;
            // The log list has one tile, the guess list none.
            u32::from(seen != 1).into_ret()
        });
        let tile = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(tile, vtable);
        e.mem.set_u32(vtable, 0x00c0_0000);
        answer(&mut e, POINTER_LIST_REMOVE_HEAD, tile);
        answer(&mut e, GAME_TIME, 100);
        let delay = e.mem.alloc(4);
        e.mem.set_u32(delay, 50);
        answer(&mut e, SETTING_VALUE, delay);
        stub(
            &mut e,
            &[
                0x00c0_0000,
                POINTER_LIST_REMOVE_ALL,
                OPERATOR_DELETE,
                SIMPLE_LIST_DESTRUCTOR,
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                ANIMATING_TEXT_DESTRUCTOR,
                SYSTEM_COLOR_GET_INSTANCE,
                SYSTEM_COLOR_SET_COLOR,
                POINTER_LIST_DESTRUCTOR,
                VECTOR_DESTRUCTOR_ITERATOR,
                MENU_DESTRUCTOR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0076_58b0, &args![this, 1u32]);
        assert_eq!(e.global::<u32>(RETRY_TIME), 150);
        assert_eq!(e.global::<u32>(HACKING_MENU), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![tile, 1]]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_STOP).len(), 5);
        assert!(calls_to(&e, ANIMATING_TEXT_DESTRUCTOR).contains(&vec![line]));
        let deletes = calls_to(&e, OPERATOR_DELETE);
        assert_eq!(deletes[0], vec![0x4440]);
        assert_eq!(deletes[1], vec![line]);
        assert_eq!(deletes.last().unwrap(), &vec![this.addr()]);
        assert_eq!(
            calls_to(&e, SYSTEM_COLOR_GET_INSTANCE),
            vec![Vec::<u32>::new()]
        );
        assert_eq!(
            calls_to(&e, SYSTEM_COLOR_SET_COLOR)[0][1..],
            [3, SAVE_COLOR]
        );
        e.call_log = Some(vec![]);
        e.call(0x0076_58e0, &args![this]);
        assert!(!calls_to(&e, OPERATOR_DELETE).contains(&vec![this.addr()]));
    }

    #[test]
    fn terminal_io_scalar_deleting_destructor() {
        let mut e = Engine::new();
        let line = e.mem.alloc(0x34);
        let tile = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(tile, vtable);
        e.mem.set_u32(vtable, 0x00c0_0000);
        e.mem.set_u32(line + 0x30, tile);
        stub(
            &mut e,
            &[0x00c0_0000, ANIMATING_TEXT_DESTRUCTOR, OPERATOR_DELETE],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_5b50, &args![line, 1u32]).u32(), line);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![tile, 1]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![line]]);
        e.call_log = Some(vec![]);
        e.call(0x0076_b560, &args![line]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, ANIMATING_TEXT_DESTRUCTOR), vec![vec![line]]);
    }

    #[test]
    fn display_interval_is_at_least_one() {
        let mut e = Engine::new();
        e.register(MAX_OF_TWO, |_, a| a[0].max(a[1]).into_ret());
        let text = e.mem.alloc(0x34);
        e.call(0x0076_6a10, &args![text, 0u32]);
        assert_eq!(e.mem.u32(text + 0x10), 1);
        e.call(0x0076_6a10, &args![text, 40u32]);
        assert_eq!(e.mem.u32(text + 0x10), 40);
    }

    #[test]
    fn tile_check_reports_the_missing_index() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        let index = e.mem.alloc(4);
        assert!(e.call(0x0076_6a50, &args![this, index]).bool());
        assert_eq!(e.mem.u32(index), 8);
        e.mem.set_u32(this.addr() + TILES + 4 * 5, 0);
        assert!(!e.call(0x0076_6a50, &args![this, index]).bool());
        assert_eq!(e.mem.u32(index), 5);
    }

    #[test]
    fn close_fades_out_and_sets_the_retry_time() {
        let mut e = Engine::new();
        menu(&mut e);
        pages(&mut e, &[CLOSE_FADE, RETRY_TIME]);
        e.set_global(CLOSE_FADE, 0.75f32.to_bits());
        e.set_global(INTERFACE_MANAGER, 0x6000u32);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        answer(&mut e, MANAGER_TERMINAL_REFERENCE, 0x5000);
        answer(&mut e, CLOSE_COMPARE_REFERENCE, 7);
        answer(&mut e, CLOSE_COMPARE_TILE, 7);
        answer(&mut e, RT_DYNAMIC_CAST, 0x5100);
        answer(&mut e, GAME_TIME, 1000);
        stub(
            &mut e,
            &[
                PLAY_SHUTDOWN_SOUND,
                TILE_SET_FLOAT,
                RENDERED_TERMINAL_FADE_OUT,
                TILE_SET_INT,
                MENU_START_FADE_OUT,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0076_6aa0, &args![]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x4000, 0xfc1, 0.75f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, RENDERED_TERMINAL_FADE_OUT), vec![vec![0x5100]]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x5000, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x4000, 0x1772, 1]]);
        assert_eq!(e.global::<u32>(RETRY_TIME), 1000 + 8000);
        // Different values: no fade-out of the terminal.
        answer(&mut e, CLOSE_COMPARE_TILE, 8);
        e.call_log = Some(vec![]);
        e.call(0x0076_6aa0, &args![]);
        assert!(calls_to(&e, RENDERED_TERMINAL_FADE_OUT).is_empty());
        // Without a menu nothing happens.
        e.set_global(HACKING_MENU, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0076_6aa0, &args![]);
        assert_eq!(order(&e), vec![0x0076_6aa0]);
    }

    /// A grid row with the given bytes.
    fn row(e: &mut Engine, bytes: &[u8]) -> u32 {
        let row = e.mem.alloc(8);
        let data = e.mem.alloc(bytes.len().max(1) as u32);
        e.mem.write(data, bytes);
        e.mem.set_u32(row, data);
        e.mem.set_u32(row + 4, bytes.len() as u32);
        row
    }

    #[test]
    fn row_positions_and_largest_count() {
        let mut e = Engine::new();
        let row = row(&mut e, &[3, 1, 3, 0x40, 3, 1]);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x0076_ae30, &args![row, 3i32, out]).u32(), 3);
        assert_eq!(e.mem.bytes(out, 3), vec![0, 2, 4]);
        // 0x40 is not counted (32 and above).
        assert_eq!(e.call(0x0076_ae90, &args![row]).u32(), 3);
        assert_eq!(e.call(0x0076_af60, &args![2u32, 9u32]).u32(), 9);
        assert_eq!(e.call(0x0076_af60, &args![0xffu32, 9u32]).u32(), 0xff);
    }

    /// Doubles for the grid row array and the word getters.
    fn grid_doubles(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        stub(e, &[OPERATOR_DELETE, ROW_ARRAY_DELETE]);
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u64(a[0] + 8 * i, 0);
            }
            Ret::default()
        });
        e.register(WORD_AT_ZERO, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        copy_memory(e);
    }

    #[test]
    fn grid_construct_copy_and_destroy() {
        let mut e = Engine::new();
        grid_doubles(&mut e);
        let grid = e.mem.alloc(8);
        e.call(0x0076_8810, &args![grid, 2u32]);
        assert_eq!(e.mem.u32(grid + 4), 2);
        let rows = e.mem.u32(grid);
        assert_eq!(e.mem.u32(rows - 4), 2, "the count before the rows");
        assert_eq!(e.mem.u32(rows + 8 + 4), 2);
        let data = e.mem.u32(rows + 8);
        e.mem.write(data, &[5, 6]);
        let copy = e.mem.alloc(8);
        e.mem.set_u32(copy, 0x1234);
        e.call(0x0076_b090, &args![copy, grid]);
        let copy_rows = e.mem.u32(copy);
        assert_ne!(copy_rows, rows);
        assert_eq!(e.mem.bytes(e.mem.u32(copy_rows + 8), 2), vec![5, 6]);
        e.call_log = Some(vec![]);
        e.call(0x0076_b0c0, &args![copy, grid]);
        assert_eq!(calls_to(&e, ROW_ARRAY_DELETE), vec![vec![copy_rows, 3]]);
        let copy_rows = e.mem.u32(copy);
        e.call_log = Some(vec![]);
        e.call(0x0076_87d0, &args![copy]);
        assert_eq!(calls_to(&e, ROW_ARRAY_DELETE), vec![vec![copy_rows, 3]]);
        e.mem.set_u32(copy, 0);
        e.call_log = Some(vec![]);
        e.call(0x0076_87d0, &args![copy]);
        assert!(calls_to(&e, ROW_ARRAY_DELETE).is_empty());
        assert_eq!(row_array_size(0x2000_0000), u32::MAX);
        assert_eq!(row_array_size(0x1fff_ffff), 0xffff_fffc);
        assert_eq!(row_array_size(3), 28);
    }

    #[test]
    fn row_resize_and_assignment() {
        let mut e = Engine::new();
        grid_doubles(&mut e);
        let source = row(&mut e, &[1, 2, 3]);
        let target = row(&mut e, &[9]);
        let old = e.mem.u32(target);
        e.call_log = Some(vec![]);
        e.call(0x0076_b200, &args![target, source]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(target + 4), 3);
        assert_eq!(e.mem.bytes(e.mem.u32(target), 3), vec![1, 2, 3]);
        e.call(0x0076_8920, &args![target, 5u32]);
        assert_eq!(e.mem.u32(target + 4), 5);
    }

    #[test]
    fn permuted_grid_takes_the_ordered_cells() {
        let mut e = Engine::new();
        grid_doubles(&mut e);
        let grid = e.mem.alloc(8);
        e.call(0x0076_8810, &args![grid, 3u32]);
        let rows = e.mem.u32(grid);
        for (i, bytes) in [[0u8, 1, 2], [3, 4, 5], [6, 7, 8]].iter().enumerate() {
            let data = e.mem.u32(rows + 8 * i as u32);
            e.mem.write(data, bytes);
        }
        let order_bytes = e.mem.alloc(2);
        e.mem.write(order_bytes, &[2, 0]);
        let result = e.mem.alloc(8);
        assert_eq!(
            e.call(0x0076_af90, &args![grid, result, order_bytes, 2u32])
                .u32(),
            result
        );
        let rows = e.mem.u32(result);
        assert_eq!(e.mem.bytes(e.mem.u32(rows), 2), vec![8, 6]);
        assert_eq!(e.mem.bytes(e.mem.u32(rows + 8), 2), vec![2, 0]);
    }

    #[test]
    fn grid_measure_counts_the_rounds() {
        let mut e = Engine::new();
        grid_doubles(&mut e);
        e.register(MAX_OF_TWO, |_, a| a[0].max(a[1]).into_ret());
        e.register(MIN_OF_TWO, |_, a| a[0].min(a[1]).into_ret());
        let grid = e.mem.alloc(8);
        e.call(0x0076_8810, &args![grid, 3u32]);
        let rows = e.mem.u32(grid);
        for (i, bytes) in [[0xffu8, 1, 1], [1, 0xff, 1], [1, 1, 0xff]]
            .iter()
            .enumerate()
        {
            let data = e.mem.u32(rows + 8 * i as u32);
            e.mem.write(data, bytes);
        }
        // Each row's largest count is 2 (two 1s): the measure is 3.
        assert_eq!(e.call(0x0076_9520, &args![0u32, grid]).u32(), 3);
        let empty = e.mem.alloc(8);
        assert_eq!(e.call(0x0076_9520, &args![0u32, empty]).u32(), 0);
    }

    #[test]
    fn word_list_add_copies_the_word() {
        let mut e = Engine::new();
        let word = e.mem.alloc(8);
        e.mem.set_cstr(word, b"CAT");
        answer(&mut e, STRLEN, 3);
        answer(&mut e, OPERATOR_NEW, 0x6000);
        stub(&mut e, &[STRCPY_S]);
        e.register(LIST_ADD_WORD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x6000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0076_b240, &args![0x3000u32, word]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![4]]);
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![0x6000, 4, word]]);
        assert_eq!(calls_to(&e, LIST_ADD_WORD)[0][0], 0x3000);
    }

    #[test]
    fn random_sign() {
        let mut e = Engine::new();
        answer(&mut e, RANDOM_BELOW, 1);
        assert_eq!(e.call(0x0076_b2e0, &args![]).i32(), 1);
        answer(&mut e, RANDOM_BELOW, 0);
        assert_eq!(e.call(0x0076_b2e0, &args![]).i32(), -1);
    }

    /// Doubles for the terminal line constructor.
    fn line_doubles(e: &mut Engine, value: u32) {
        pages(e, &[INTERVAL_SCALE, DEFAULT_INTERVAL]);
        e.mem.set_f64(INTERVAL_SCALE, 1000.0);
        e.mem.set_f32(DEFAULT_INTERVAL, 1000.0);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, value);
        answer(e, SETTING_VALUE, setting);
        e.register(ROUND_TO_MULTIPLE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).round(),
            ..Ret::default()
        });
        e.register(MAX_OF_TWO, |_, a| a[0].max(a[1]).into_ret());
        stub(
            e,
            &[
                ANIMATING_TEXT_CONSTRUCTOR_WITH_ARGUMENT,
                ANIMATING_TEXT_CONNECT,
                TILE_SET_STRING,
                TILE_SET_INT,
                TILE_SET_INT_FROM_VALUE,
            ],
        );
        answer(e, TILE_GET_VALUE_Q, 0x71);
        answer(e, TILE_GET_VALUE, 0x72);
        answer(e, WORD_AT_FOUR, 0x4000);
        answer_st0(e, TILE_GET_FLOAT, 40.0);
        answer(e, STRLEN, 2);
        real_ftol(e);
    }

    #[test]
    fn terminal_line_with_a_prompt() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        line_doubles(&mut e, 8);
        e.register(MENU_RENDER_TEMPLATE, |_, a| {
            (0x9000 + (a[2] & 0xfff)).into_ret()
        });
        let line = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        let made = e.call(
            0x0076_b300,
            &args![line, 0x55u32, 0x104u32, 0x0107_2000u32, 30i32, 1u32],
        );
        assert_eq!(made.u32(), line);
        // 1000 / 8 = 125.
        assert_eq!(e.mem.u32(line + 0x10), 125);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE),
            vec![
                vec![this.addr(), 0x104, 0x0107_2000, 0],
                vec![this.addr(), 0x100, PROMPT_TEMPLATE, 0]
            ]
        );
        assert_eq!(e.mem.u32(line + 0x2c), 0x9000);
        assert_eq!(e.mem.u32(line + 0x30), 0x993c);
        assert_eq!(
            calls_to(&e, ANIMATING_TEXT_CONNECT),
            vec![vec![line, 0x72, 0x71]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x9000, 0xfa2, 30],
                vec![0x993c, 0xfa1, 40],
                vec![0x993c, 0xfa2, 30]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT_FROM_VALUE),
            vec![vec![0x9000, 0xfa1, 2 * 17 + 40]]
        );
    }

    #[test]
    fn terminal_line_without_a_prompt_or_a_menu() {
        let mut e = Engine::new();
        menu(&mut e);
        line_doubles(&mut e, 1);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x9000);
        let line = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(
            0x0076_b300,
            &args![line, 0x55u32, 0x104u32, 0x0107_2000u32, -1i32, 0u32],
        );
        // The default 1000.0 truncated.
        assert_eq!(e.mem.u32(line + 0x10), 1000);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        e.set_global(HACKING_MENU, 0u32);
        e.call_log = Some(vec![]);
        e.call(
            0x0076_b300,
            &args![line, 0x55u32, 0x104u32, 0x0107_2000u32, 3i32, 0u32],
        );
        assert_eq!(
            order(&e),
            vec![0x0076_b300, ANIMATING_TEXT_CONSTRUCTOR_WITH_ARGUMENT]
        );
    }

    #[test]
    fn hexadecimal_address() {
        let mut e = Engine::new();
        pages(&mut e, &[HEX_PREFIX]);
        e.mem.write(HEX_PREFIX, b"0x");
        copy_memory(&mut e);
        let buffer = e.mem.alloc(8);
        e.call(0x0076_8970, &args![0u32, 0xf4a9u32, buffer]);
        assert_eq!(e.mem.cstr(buffer), b"0xF4A9".to_vec());
    }

    #[test]
    fn shuffle_passes_the_sign_function() {
        let mut e = Engine::new();
        stub(&mut e, &[0x0083_fd60]);
        e.call_log = Some(vec![]);
        e.call(0x0076_87b0, &args![0x3000u32]);
        assert_eq!(calls_to(&e, 0x0083_fd60), vec![vec![0x3000, 0x0076_b2e0]]);
    }

    #[test]
    fn common_positions_counts_equal_letters() {
        let mut e = Engine::new();
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(MIN_OF_TWO, |_, a| a[0].min(a[1]).into_ret());
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_cstr(a, b"SWORD");
        e.mem.set_cstr(b, b"SPOR");
        assert_eq!(e.call(0x0076_8a10, &args![0u32, a, b]).u32(), 3);
        assert_eq!(e.call(0x0076_8a10, &args![0u32, a, 0u32]).u32(), 0);
    }

    #[test]
    fn default_terminal_line() {
        let mut e = Engine::new();
        stub(&mut e, &[ANIMATING_TEXT_CONSTRUCTOR]);
        let line = e.mem.alloc(0x34);
        e.mem.set_u32(line + 0x2c, 9);
        assert_eq!(e.call(0x0076_94f0, &args![line]).u32(), line);
        assert_eq!(e.mem.u32(line + 0x2c), 0);
    }

    #[test]
    fn char_hit_from_the_gamepad_cursor() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[ONE_DOUBLE]);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        let manager = e.mem.alloc(0x100);
        e.set_global(INTERFACE_MANAGER, manager);
        e.mem.set_u32(manager + 0xcc, 0x700);
        stub(&mut e, &[INTERFACE_SINGLETON]);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer_st0(&mut e, TILE_SCREEN_X, 100.0);
        answer_st0(&mut e, TILE_GET_FLOAT, 204.0);
        real_ftol(&mut e);
        e.set(this, HackingMenu::iCursorX, 100 + 3 * 17 + 5);
        assert_eq!(e.call(0x0076_9700, &args![this]).i32(), 3);
        e.set(this, HackingMenu::iCursorX, 99);
        assert_eq!(e.call(0x0076_9700, &args![this]).i32(), -1);
        // Right edge: 100 + 204 - 1.
        e.set(this, HackingMenu::iCursorX, 304);
        assert_eq!(e.call(0x0076_9700, &args![this]).i32(), -1);
        // The mouse: the integer position.
        answer(&mut e, GAMEPAD_IN_USE, 0);
        answer(&mut e, CURSOR_HAS_FLOAT_POSITION, 0);
        answer(&mut e, CURSOR_INT_POSITION, 135);
        assert_eq!(e.call(0x0076_9700, &args![this]).i32(), 2);
    }

    #[test]
    fn differs_from_the_first_word() {
        let mut e = Engine::new();
        answer(&mut e, WORD_AT_ZERO, 5);
        assert!(e.call(0x0076_a3b0, &args![0u32, 4u32]).bool());
        assert!(!e.call(0x0076_a3b0, &args![0u32, 5u32]).bool());
    }

    #[test]
    fn access_time_goes_back_to_the_computers_menu() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set(this, HackingMenu::dwAccessTime, 500);
        answer(&mut e, GAME_TIME, 500);
        e.call_log = Some(vec![]);
        e.call(0x0076_a510, &args![this]);
        assert_eq!(order(&e), vec![0x0076_a510, GAME_TIME]);
        answer(&mut e, GAME_TIME, 501);
        stub(&mut e, &[TILE_SET_INT, MENU_START_FADE_OUT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_a510, &args![this]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![this.addr()]]);
    }

    #[test]
    fn garbage_string_takes_a_bracket_run() {
        let mut e = Engine::new();
        pages(&mut e, &[OPEN_BRACKETS]);
        let open = e.mem.alloc(8);
        let close = e.mem.alloc(8);
        e.mem.set_cstr(open, b"([{<");
        e.mem.set_cstr(close, b")]}>");
        e.set_global(OPEN_BRACKETS, open);
        e.set_global(CLOSE_BRACKETS, close);
        e.register(STRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().position(|&c| i32::from(c) == a[1] as i32) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(ISALPHA, |_, a| {
            u32::from((a[0] as u8).is_ascii_alphabetic()).into_ret()
        });
        copy_memory(&mut e);
        let source = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        for (text, expected) in [
            (&b"[%$]x"[..], &b"[%$]"[..]),
            (b"[%A]", b"["),
            (b"%%", b"%"),
        ] {
            e.mem.write(out, &[0; 16]);
            e.mem.set_cstr(source, text);
            e.call(0x0076_a670, &args![0u32, out, source]);
            assert_eq!(e.mem.cstr(out), expected.to_vec());
        }
    }

    #[test]
    fn cursor_follows_the_entry_text() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        answer(&mut e, TILE_IS_TRUE, 1);
        answer(&mut e, TILE_GET_STRING, 0x5000);
        answer(&mut e, STRLEN, 4);
        // The entry tile (0x114) has parent 0x200, whose parent 0x300 is the
        // cursor's parent.
        e.register(TILE_PARENT, |_, a| {
            match a[0] {
                0x114 => 0x200u32,
                0x200 => 0x300,
                0x110 => 0x300,
                _ => 0,
            }
            .into_ret()
        });
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: f64::from(a[0]),
            ..Ret::default()
        });
        real_ftol(&mut e);
        stub(&mut e, &[TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_ab40, &args![this]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x110, 0xfa1, 4 * 17 + 0x114 + 0x200]]
        );
    }

    #[test]
    fn gamepad_up_event_finishes_on_button_a() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(HACKING_MENU, 0u32);
        assert!(!e.call(0x0076_7b80, &args![this, 3i32, 0u32]).bool());
        assert!(e.call(0x0076_7b80, &args![this, 0xai32, 0u32]).bool());
        assert_eq!(e.get(this, HackingMenu::eStage), 4);
    }

    #[test]
    fn leave_clears_the_entry() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set(this, HackingMenu::bSuppressEntry, true);
        e.set(this, HackingMenu::pLastTarget, 9);
        stub(&mut e, &[ANIMATING_TEXT_RESET_BUFFER, TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_7ff0, &args![this, 0u32, 0u32]);
        assert!(!e.get(this, HackingMenu::bSuppressEntry));
        assert_eq!(e.get(this, HackingMenu::pLastTarget), 0);
        assert_eq!(
            calls_to(&e, ANIMATING_TEXT_RESET_BUFFER),
            vec![vec![this.addr() + 0xe0, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x108, 0x1005, u32::MAX], vec![0x108, 0x1008, u32::MAX]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0076_8020, &args![this]);
        assert_eq!(calls_to(&e, TILE_SET_INT).len(), 2);
    }

    #[test]
    fn shutdown_scrolls_and_finishes() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[ZERO_DOUBLE]);
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        e.mem.set_u32(this.addr() + 4, 0x4000);
        e.set(this, HackingMenu::iYInterval, 30);
        e.set(this, HackingMenu::dwAccessTime, 100);
        answer(&mut e, GAME_TIME, 220);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0x1005 { 10.0 } else { -50.0 },
            ..Ret::default()
        });
        answer_st0(&mut e, TILE_BOTTOM_00A01440, 20.0);
        stub(&mut e, &[TILE_SET_FLOAT, SOUND_HANDLE_PLAY, TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_a3f0, &args![this]);
        // (220 - 100) / 50 + 1 = 3 steps of 30.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x4000, 0x1005, 100.0f32.to_bits(), 1]]
        );
        assert_eq!(e.get(this, HackingMenu::dwAccessTime), 270);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_PLAY),
            vec![vec![this.addr() + 0x150, 0]]
        );
        // -50 + 20 < 0: finished.
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x4000, 0x1006, 1]]);
        assert_eq!(e.get(this, HackingMenu::eStage), 4);
    }

    #[test]
    fn transition_opens_the_computers_menu() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 4, 0x4000);
        e.set(this, HackingMenu::pTerminal, 0x800);
        e.set(this, HackingMenu::pTerminalRef, 0x900);
        answer(&mut e, COMPUTERS_MENU_CREATE, 0x5000);
        answer(&mut e, TILE_GET_MENU, 0x5100);
        answer(&mut e, GET_CURRENT_RENDERED_MENU, 0);
        answer(&mut e, RT_DYNAMIC_CAST, 0);
        answer(&mut e, OPERATOR_NEW, 0x6000);
        answer(&mut e, RENDERED_MENU_CONSTRUCTOR, 0x6000);
        answer(&mut e, CLOSE_COMPARE_TILE, 0x77);
        stub(
            &mut e,
            &[
                TILE_SET_INT,
                MENU_START_FADE_OUT,
                SET_BYTE_AT_0X1C,
                TILE_UPDATE_ALL,
                PRINT_MESSAGE,
                RENDERED_MENU_SET_TILE,
                RENDERED_MENU_SET_FROM_TILE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0076_a540, &args![this]);
        assert_eq!(calls_to(&e, COMPUTERS_MENU_CREATE), vec![vec![0x900]]);
        assert_eq!(calls_to(&e, SET_BYTE_AT_0X1C), vec![vec![0x5100]]);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![NO_RENDERED_MENU_MESSAGE]]
        );
        assert_eq!(
            calls_to(&e, RENDERED_MENU_SET_TILE),
            vec![vec![0x6000, 0x5000]]
        );
        assert_eq!(
            calls_to(&e, RENDERED_MENU_SET_FROM_TILE),
            vec![vec![0x6000, 0x77]]
        );
        e.set(this, HackingMenu::pTerminal, 0);
        e.call_log = Some(vec![]);
        e.call(0x0076_a540, &args![this]);
        assert!(calls_to(&e, COMPUTERS_MENU_CREATE).is_empty());
    }

    #[test]
    fn flash_cursor_toggles_the_visibility() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[FLASH_PERIOD, ONE_DOUBLE]);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        answer(&mut e, TILE_ADD_USER_TRAIT, 0x2000);
        answer_st0(&mut e, TILE_GET_FLOAT, 250.0);
        answer(&mut e, GAME_TIME, 1000);
        let value = e.mem.alloc(0x14);
        e.mem.set_f32(value + 8, 1.0);
        e.set(this, HackingMenu::pCursorVis, value);
        stub(&mut e, &[TILE_VALUE_SET_FLOAT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_a9d0, &args![this]);
        assert_eq!(e.global::<f32>(FLASH_PERIOD), 250.0);
        assert_eq!(
            calls_to(&e, TILE_VALUE_SET_FLOAT),
            vec![vec![value, 0.0f32.to_bits(), 1]]
        );
        assert_eq!(e.global::<u32>(NEXT_FLASH_TIME), 1250);
        // Before the next flash, and without a value: it is looked up.
        e.set(this, HackingMenu::pCursorVis, 0);
        answer(&mut e, TILE_GET_VALUE_Q, 0x3300);
        e.call_log = Some(vec![]);
        e.call(0x0076_a9d0, &args![this]);
        assert_eq!(e.get(this, HackingMenu::pCursorVis), 0x3300);
        assert!(calls_to(&e, TILE_ADD_USER_TRAIT).is_empty());
    }

    #[test]
    fn scrolling_cursor_sits_after_the_line() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        let line = e.mem.alloc(0x34);
        e.mem.set_u32(line + 0x2c, 0x600);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"AB");
        answer(&mut e, TILE_GET_STRING, text);
        answer(&mut e, STRLEN, 2);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: f64::from(a[0] + a[1] - 0xfa1),
            ..Ret::default()
        });
        e.register(TILE_PARENT, |_, a| {
            match a[0] {
                0x600 | 0x110 => 0x700u32,
                _ => 0,
            }
            .into_ret()
        });
        real_ftol(&mut e);
        stub(&mut e, &[TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_ac10, &args![this, line]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x110, 0xfa1, 0x600 + 34], vec![0x110, 0xfa2, 0x601]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0076_ac10, &args![this, 0u32]);
        assert_eq!(order(&e).len(), 1);
    }

    #[test]
    fn guess_block_is_rendered_and_labelled() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x9000);
        answer(&mut e, POINTER_LIST_COUNT, 2);
        answer(&mut e, TILE_VALUE_00700300, 0x31);
        answer(&mut e, WORD_AT_ZERO, 0x5500);
        e.register(POINTER_LIST_ADD_TAIL, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x9000);
            Ret::default()
        });
        stub(
            &mut e,
            &[
                TILE_SET_INT_FROM_VALUE,
                STRING_CONSTRUCTOR,
                STRING_SPRINTF,
                TILE_SET_TEXT,
                STRING_DESTRUCTOR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0076_ad50, &args![this]);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE),
            vec![vec![this.addr(), 0x104, GUESS_BLOCK_TEMPLATE, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT_FROM_VALUE),
            vec![vec![0x9000, 0xfac, 2]]
        );
        assert_eq!(
            calls_to(&e, STRING_SPRINTF)[0][1..],
            [GUESS_BLOCK_FORMAT, 0x31, 2]
        );
        assert_eq!(calls_to(&e, TILE_VALUE_00700300), vec![vec![0x9000]]);
        assert_eq!(calls_to(&e, TILE_SET_TEXT), vec![vec![0x9000, 0x5500]]);
    }

    #[test]
    fn find_tile_matches_line_and_column() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        for i in 0..IO_LINE_COUNT {
            let line = e.mem.alloc(0x34);
            e.mem.set_u32(line + 0x2c, 0x1000 + i);
            e.mem.set_u32(this.addr() + IO_LINES + 4 * i, line);
        }
        answer(&mut e, TILE_IS_TRUE, 1);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0xfac {
                f64::from(a[0] & 0xf)
            } else {
                f64::from((a[0] >> 4) & 1)
            },
            ..Ret::default()
        });
        assert_eq!(e.call(0x0076_7bb0, &args![this, 3u32, 1u32]).u32(), 0x1013);
        assert_eq!(e.call(0x0076_7bb0, &args![this, 0x20u32, 0u32]).u32(), 0);
    }

    #[test]
    fn scroll_log_moves_old_lines_and_adds_new_ones() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 0x7100);
        answer(&mut e, WORD_AT_ZERO, node);
        e.register(POINTER_LIST_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], 0);
            (node + 8).into_ret()
        });
        answer_st0(&mut e, TILE_GET_FLOAT, 1.0);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x9000);
        stub(
            &mut e,
            &[
                TILE_SET_FLOAT,
                TILE_SET_INT_FROM_VALUE,
                TILE_SET_STRING,
                POINTER_LIST_ADD_TAIL,
            ],
        );
        let lines = e.mem.alloc(8);
        e.mem.set_u32(lines, 0x51);
        e.mem.set_u32(lines + 4, 0x52);
        e.call_log = Some(vec![]);
        e.call(0x0076_7ef0, &args![this, 2u32, lines]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x7100, 0xfac, 3.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT_FROM_VALUE),
            vec![vec![0x9000, 0xfac, 1], vec![0x9000, 0xfac, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x9000, 0xfc4, 0x51, 1], vec![0x9000, 0xfc4, 0x52, 1]]
        );
    }
}

#[cfg(test)]
mod n054_flow_tests {
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

    fn menu(e: &mut Engine) -> Ptr<HackingMenu> {
        pages(e, &[HACKING_MENU]);
        let this = e.new_object::<HackingMenu>();
        for i in 0..8 {
            e.mem.set_u32(this.addr() + TILES + 4 * i, 0x100 + 4 * i);
        }
        e.set_global(HACKING_MENU, this.addr());
        this
    }

    /// The word list as a chain of 8-byte nodes holding the words.
    fn word_list(e: &mut Engine, this: Ptr<HackingMenu>, words: &[&[u8]]) -> Vec<u32> {
        let mut node = this.addr() + WORD_LIST;
        let mut texts = vec![];
        for (i, word) in words.iter().enumerate() {
            let text = e.mem.alloc(16);
            e.mem.set_cstr(text, word);
            texts.push(text);
            e.mem.set_u32(node, text);
            let next = if i + 1 < words.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        texts
    }

    fn list_doubles(e: &mut Engine) {
        e.register(IDENTITY, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_ZERO, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(MIN_OF_TWO, |_, a| a[0].min(a[1]).into_ret());
        e.register(MAX_OF_TWO, |_, a| a[0].max(a[1]).into_ret());
        e.register(STRING_COMPARE, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    #[test]
    fn dud_removal_dots_out_a_word() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        list_doubles(&mut e);
        let texts = word_list(&mut e, this, &[b"ABC", b"XYZ"]);
        e.set(this, HackingMenu::pPassword, texts[0]);
        e.set(this, HackingMenu::iWordsHave, 2);
        e.set(this, HackingMenu::iWordLength, 3);
        let positions = e.mem.alloc(8);
        e.mem.set_u32(positions, 5);
        // The second word at buffer position 12 + 2: line 5, column 2.
        e.mem.set_u32(positions + 4, 0xc + 2);
        e.set(this, HackingMenu::iBufferPos, positions);
        let line = e.mem.alloc(0x34);
        e.mem.set_u32(line + 0x2c, 0x600);
        e.mem.set_u32(this.addr() + IO_LINES + 4 * 5, line);
        let text = e.mem.alloc(0x20);
        e.mem.set_cstr(text, b"0xF000 ..XYZ.......");
        answer(&mut e, TILE_GET_STRING, text);
        let mut picks = vec![0u32, 1];
        e.register_double(RANDOM_BELOW, move |_, _| picks.remove(0).into_ret());
        e.register(TILE_SET_STRING, |e, a| {
            assert_eq!(&e.mem.cstr(a[2])[..13], b"0xF000 ......");
            Ret::default()
        });
        stub(&mut e, &[OPERATOR_DELETE, WORD_LIST_REMOVE]);
        e.call_log = Some(vec![]);
        e.call(0x0076_a730, &args![this]);
        assert_eq!(calls_to(&e, TILE_SET_STRING).len(), 1);
        assert_eq!(e.mem.u32(positions + 4), 0x198);
        assert_eq!(e.get(this, HackingMenu::iWordsHave), 1);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![texts[1]]]);
        // The first pick was the password, so it drew again.
        assert_eq!(calls_to(&e, RANDOM_BELOW).len(), 2);
    }

    #[test]
    fn hover_picks_the_word_and_sets_the_highlight() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        list_doubles(&mut e);
        pages(
            &mut e,
            &[ROWS_PER_COLUMN, CHARACTERS_PER_ROW, ONE_DOUBLE, ZERO_DOUBLE],
        );
        e.mem.set_f64(ROWS_PER_COLUMN, 17.0);
        e.mem.set_f64(CHARACTERS_PER_ROW, 12.0);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        let texts = word_list(&mut e, this, &[b"CAT"]);
        e.set(this, HackingMenu::iWordsHave, 1);
        e.set(this, HackingMenu::iWordLength, 3);
        let positions = e.mem.alloc(4);
        e.mem.set_u32(positions, 2);
        e.set(this, HackingMenu::iBufferPos, positions);
        e.set(this, HackingMenu::iLastOffset, -1);
        let manager = e.mem.alloc(0x100);
        e.set_global(INTERFACE_MANAGER, manager);
        e.mem.set_u32(manager + 0xcc, 0x700);
        // The cursor is on column 7 + 3 of the selected line (line 0).
        answer(&mut e, GAMEPAD_IN_USE, 1);
        stub(&mut e, &[INTERFACE_SINGLETON]);
        e.set(this, HackingMenu::iCursorX, 3 * 17);
        answer_st0(&mut e, TILE_SCREEN_X, 0.0);
        let line_text = e.mem.alloc(0x20);
        e.mem.set_cstr(line_text, b"0xF000 ..CAT.......");
        answer(&mut e, TILE_GET_STRING, line_text);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: match a[1] {
                0xfb1 => 400.0,
                0xfa1 => 10.0,
                0xfa2 => 20.0,
                _ => 0.0,
            },
            ..Ret::default()
        });
        real_ftol(&mut e);
        // The entry text changes.
        e.register(TEST_TARGET_00700270, |_, _| 1u32.into_ret());
        e.register(ISALPHA, |_, a| {
            u32::from((a[0] as u8).is_ascii_alphabetic()).into_ret()
        });
        stub(
            &mut e,
            &[
                MEMSET,
                ANIMATING_TEXT_RESET_BUFFER,
                SOUND_LIST_CLEAR,
                AUDIO_MANAGER,
                GET_SOUND_HANDLE_BY_NAME,
                SOUND_LIST_ADD,
                SOUND_HANDLE_PLAY_AFTER,
                SOUND_HANDLE_PLAY,
                SOUND_HANDLE_DESTRUCTOR,
                TILE_SET_STRING,
                TILE_SET_INT,
            ],
        );
        answer(&mut e, WORD_AT_0X10, 20);
        pages(&mut e, &[SOUNDS_TO_PLAY, TYPING_SOUND_SCALE]);
        e.mem.set_f64(TYPING_SOUND_SCALE, 1.5);
        stub(&mut e, &[SOUND_HANDLE_RELEASE, TILE_SET_FLOAT]);
        e.register(ANIMATING_TEXT_RESET_BUFFER, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"CAT".to_vec());
            Ret::default()
        });
        answer(&mut e, SOUND_LIST_NEXT, 0);
        e.set(this, HackingMenu::iCursorX, 9 * 17);
        e.call_log = Some(vec![]);
        e.call(0x0076_9b50, &args![this]);
        assert_eq!(e.get(this, HackingMenu::iLastOffset), 2);
        assert_eq!(e.get(this, HackingMenu::pLastTarget), 0x700);
        // The word was copied into the entry.
        assert_eq!(calls_to(&e, ANIMATING_TEXT_RESET_BUFFER).len(), 1);
        let _ = texts;
        // Three characters: two delayed sounds and one at once.
        let delayed = calls_to(&e, SOUND_HANDLE_PLAY_AFTER);
        assert_eq!(delayed.len(), 2);
        assert_eq!(delayed[0][1..], [60, 0]);
        assert_eq!(delayed[1][1..], [30, 0]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY).len(), 1);
        // The word stays on its line: the plain highlight at the cursor.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x108, 0x1005, 163.0f32.to_bits(), 1],
                vec![0x108, 0x1006, 20.0f32.to_bits(), 1]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x108, 0x1008, u32::MAX]]
        );
        // The same place again: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0076_9b50, &args![this]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
    }
}

#[cfg(test)]
mod n054_menu_tests {
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

    fn menu(e: &mut Engine) -> Ptr<HackingMenu> {
        pages(e, &[HACKING_MENU]);
        let this = e.new_object::<HackingMenu>();
        for i in 0..8 {
            e.mem.set_u32(this.addr() + TILES + 4 * i, 0x100 + 4 * i);
        }
        e.set_global(HACKING_MENU, this.addr());
        this
    }

    /// The state `DoIdle` checks first: fade state 1, on top, no flash.
    fn idle_ready(e: &mut Engine, this: Ptr<HackingMenu>) {
        e.mem.set_u32(this.addr() + 0x24, 1);
        answer(e, IS_TOP_MENU_ID, 1);
        stub(e, &[INTERFACE_SINGLETON, ANIMATING_TEXT_UPDATE]);
        answer(e, GAMEPAD_IN_USE, 0);
        pages(e, &[FLASH_PERIOD]);
        e.set_global(FLASH_PERIOD_INITIALIZED, 1u32);
        e.set_global(NEXT_FLASH_TIME, u32::MAX);
        answer(e, GAME_TIME, 0);
        e.set(this, HackingMenu::pCursorVis, 0x77);
        let manager = e.mem.alloc(0x100);
        e.set_global(INTERFACE_MANAGER, manager);
    }

    #[test]
    fn idle_types_the_first_unfinished_line() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        idle_ready(&mut e, this);
        e.set(this, HackingMenu::eStage, 1);
        let first = e.mem.alloc(0x34);
        let second = e.mem.alloc(0x34);
        e.mem.set_u32(this.addr() + IO_LINES, first);
        e.mem.set_u32(this.addr() + IO_LINES + 8, second);
        e.register_double(ANIMATING_TEXT_FINISHED, move |_, a| {
            u32::from(a[0] == first).into_ret()
        });
        stub(&mut e, &[SOUND_HANDLE_PLAY]);
        e.call_log = Some(vec![]);
        e.call(0x0076_7c90, &args![this]);
        assert_eq!(
            calls_to(&e, ANIMATING_TEXT_UPDATE),
            vec![vec![this.addr() + 0xe0], vec![second]]
        );
        assert_eq!(e.get(this, HackingMenu::eStage), 1);
        // Not on top: nothing.
        answer(&mut e, IS_TOP_MENU_ID, 0);
        e.call_log = Some(vec![]);
        e.call(0x0076_7c90, &args![this]);
        assert!(calls_to(&e, ANIMATING_TEXT_UPDATE).is_empty());
    }

    #[test]
    fn idle_starts_the_game_when_all_lines_are_done() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        idle_ready(&mut e, this);
        e.set(this, HackingMenu::eStage, 1);
        e.register(TILE_PARENT, |_, _| 0u32.into_ret());
        stub(&mut e, &[TILE_SET_INT]);
        answer_st0(&mut e, TILE_GET_FLOAT, 12.0);
        real_ftol(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0076_7c90, &args![this]);
        // The entry tile alone (its parent is null): its y.
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x110, 0xfa2, 12]]);
        assert_eq!(e.get(this, HackingMenu::eStage), 2);
    }

    #[test]
    fn idle_when_finished_waits_for_the_access_time() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        idle_ready(&mut e, this);
        e.set(this, HackingMenu::eStage, 4);
        e.set(this, HackingMenu::iNumGuesses, 2);
        e.set(this, HackingMenu::dwAccessTime, 10);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, WORD_AT_FOUR, 0x4000);
        // UpdateCursorPos with an invisible entry and no parents.
        e.register(TILE_PARENT, |_, _| 0u32.into_ret());
        answer_st0(&mut e, TILE_GET_FLOAT, 0.0);
        real_ftol(&mut e);
        stub(&mut e, &[TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_7c90, &args![this]);
        // Time 0 is not past 10: no transition.
        assert!(calls_to(&e, MENU_START_FADE_OUT).is_empty());
        assert_eq!(calls_to(&e, GAME_TIME).len(), 2);
    }

    #[test]
    fn intro_shows_a_prompt_first() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[RETRY_TIME, INTRO_NEXT_TIME]);
        answer(&mut e, GAME_TIME, 100);
        let first = e.mem.alloc(0x34);
        e.mem.set_u32(first + 0x2c, 0x600);
        e.mem.set_u32(first + 0x30, 0x610);
        e.mem.set_u32(this.addr() + IO_LINES, first);
        answer(&mut e, ANIMATING_TEXT_FINISHED, 0);
        answer(&mut e, TILE_IS_TRUE, 0);
        answer(&mut e, TILE_GET_STRING, 0);
        stub(&mut e, &[TILE_SET_INT]);
        answer_st0(&mut e, TILE_GET_FLOAT, 0.0);
        e.register(TILE_PARENT, |_, _| 0u32.into_ret());
        real_ftol(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0076_9830, &args![this]);
        let sets = calls_to(&e, TILE_SET_INT);
        assert_eq!(sets[0], vec![0x600, 0xfa3, 0]);
        assert_eq!(sets[1], vec![0x610, 0xfa3, 1]);
        assert_eq!(e.global::<u32>(INTRO_NEXT_TIME), 1100);
        // The step time has not passed: no typing yet.
        assert!(calls_to(&e, ANIMATING_TEXT_UPDATE).is_empty());
    }

    #[test]
    fn intro_waits_when_all_lines_are_done() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[RETRY_TIME, INTRO_NEXT_TIME]);
        e.set_global(RETRY_TIME, 1000u32);
        e.set_global(INTRO_NEXT_TIME, 100u32);
        answer(&mut e, GAME_TIME, 100);
        let first = e.mem.alloc(0x34);
        e.mem.set_u32(first + 0x2c, 0x600);
        e.mem.set_u32(this.addr() + IO_LINES, first);
        answer(&mut e, ANIMATING_TEXT_FINISHED, 1);
        stub(&mut e, &[TILE_SET_INT]);
        e.call_log = Some(vec![]);
        e.call(0x0076_9830, &args![this]);
        // Fourteen visibility updates of the first line, then the step time
        // check (not passed): no password file.
        let sets = calls_to(&e, TILE_SET_INT);
        assert_eq!(sets.len(), 14);
        assert_eq!(sets[0], vec![0x600, 0xfa3, 1]);
        assert_eq!(calls_to(&e, GAME_TIME).len(), 15);
    }

    #[test]
    fn gamepad_moves_up_a_line() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        stub(&mut e, &[INTERFACE_SINGLETON]);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, IS_TOP_MENU_ID, 1);
        let manager = e.mem.alloc(0x100);
        e.set_global(INTERFACE_MANAGER, manager);
        e.set(this, HackingMenu::eStage, 2);
        e.set(this, HackingMenu::pOverTile, 0x1003);
        e.set(this, HackingMenu::bSuppressEntry, true);
        for i in 0..IO_LINE_COUNT {
            let line = e.mem.alloc(0x34);
            e.mem.set_u32(line + 0x2c, 0x1000 + i);
            e.mem.set_u32(this.addr() + IO_LINES + 4 * i, line);
        }
        answer(&mut e, TILE_IS_TRUE, 1);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0xfac {
                f64::from(a[0] & 0xf)
            } else {
                0.0
            },
            ..Ret::default()
        });
        real_ftol(&mut e);
        assert!(e.call(0x0076_7610, &args![this, 1i32, 0u32]).bool());
        assert_eq!(e.get(this, HackingMenu::pOverTile), 0x1002);
        assert!(!e.get(this, HackingMenu::bSuppressEntry));
        assert_eq!(e.mem.u32(manager + 0xcc), 0x1003);
        // Line 0 cannot go up.
        e.set(this, HackingMenu::pOverTile, 0x1000);
        assert!(!e.call(0x0076_7610, &args![this, 1i32, 0u32]).bool());
        // Without a gamepad nothing is handled.
        answer(&mut e, GAMEPAD_IN_USE, 0);
        assert!(!e.call(0x0076_7610, &args![this, 2i32, 0u32]).bool());
    }

    #[test]
    fn gamepad_clicks_before_the_game() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        stub(&mut e, &[INTERFACE_SINGLETON]);
        answer(&mut e, GAMEPAD_IN_USE, 1);
        answer(&mut e, IS_TOP_MENU_ID, 1);
        let manager = e.mem.alloc(0x100);
        e.set_global(INTERFACE_MANAGER, manager);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0xc, 0x00c0_000c);
        stub(&mut e, &[0x00c0_000c]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0076_7610, &args![this, 9i32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, 0x00c0_000c),
            vec![vec![this.addr(), u32::MAX, 0]]
        );
    }

    #[test]
    fn click_in_the_pre_game_finishes_the_lines() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        stub(&mut e, &[INTERFACE_SINGLETON, ANIMATING_TEXT_FORCE_FINISH]);
        answer(&mut e, GAMEPAD_IN_USE, 0);
        e.set(this, HackingMenu::eStage, 1);
        let line = e.mem.alloc(0x34);
        e.mem.set_u32(this.addr() + IO_LINES + 8, line);
        e.call_log = Some(vec![]);
        e.call(0x0076_6b80, &args![this, 0i32, 0u32]);
        assert_eq!(calls_to(&e, ANIMATING_TEXT_FORCE_FINISH), vec![vec![line]]);
    }

    #[test]
    fn click_on_a_wrong_word_costs_a_try() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        stub(&mut e, &[INTERFACE_SINGLETON, ANIMATING_TEXT_FORCE_FINISH]);
        answer(&mut e, GAMEPAD_IN_USE, 0);
        e.set(this, HackingMenu::eStage, 2);
        e.set(this, HackingMenu::iNumGuesses, 3);
        e.set(this, HackingMenu::iWordLength, 4);
        answer(&mut e, TILE_PARENT, 0x108);
        let entry = e.mem.alloc(8);
        e.mem.set_cstr(entry, b"WORD");
        answer(&mut e, ANIMATING_TEXT_BUFFER, entry);
        let password = e.mem.alloc(8);
        e.mem.set_cstr(password, b"WARM");
        e.set(this, HackingMenu::pPassword, password);
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(MIN_OF_TWO, |_, a| a[0].min(a[1]).into_ret());
        e.register(ISALPHA, |_, a| {
            u32::from((a[0] as u8).is_ascii_alphabetic()).into_ret()
        });
        stub(
            &mut e,
            &[
                SOUND_HANDLE_PLAY,
                SOUND_HANDLE_PLAY_AFTER,
                VECTOR_CONSTRUCTOR_ITERATOR,
                VECTOR_DESTRUCTOR_ITERATOR,
                STRING_SPRINTF,
                SETTING_STRING,
                STRING_CONSTRUCTOR,
                STRING_DESTRUCTOR,
                ANIMATING_TEXT_RESET_BUFFER,
                TILE_SET_INT,
            ],
        );
        // ScrollLog with an empty log; the strings' characters are 0.
        answer(&mut e, WORD_AT_ZERO, 0);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x9000);
        stub(
            &mut e,
            &[
                TILE_SET_INT_FROM_VALUE,
                TILE_SET_STRING,
                POINTER_LIST_ADD_TAIL,
            ],
        );
        answer(&mut e, POINTER_LIST_REMOVE_TAIL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0076_6b80, &args![this, -1i32, 0x700u32]);
        assert_eq!(e.get(this, HackingMenu::iNumGuesses), 2);
        // "WORD" against "WARM": W and R match.
        let likeness: Vec<_> = calls_to(&e, STRING_SPRINTF)
            .into_iter()
            .filter(|c| c[1] == LIKENESS_FORMAT)
            .collect();
        assert_eq!(likeness[0][3..5], [2, 4]);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_PLAY_AFTER),
            vec![vec![this.addr() + 0x144, 0xfa, 0]]
        );
        assert!(e.get(this, HackingMenu::bSuppressEntry));
        assert_eq!(e.get(this, HackingMenu::eStage), 2);
    }

    #[test]
    fn prepare_closes_without_the_word_file() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set(this, HackingMenu::iWordLength, 5);
        stub(
            &mut e,
            &[
                STRING_CONSTRUCTOR,
                STRING_DESTRUCTOR,
                WORD_LIST_CONSTRUCTOR,
                SIMPLE_LIST_DESTRUCTOR,
                GAME_TIME,
                STRING_SET,
                PRINT_MESSAGE,
                // Close (with no menu tile word).
                PLAY_SHUTDOWN_SOUND,
                TILE_SET_FLOAT,
                TILE_SET_INT,
                MENU_START_FADE_OUT,
            ],
        );
        answer(&mut e, WORD_AT_ZERO, 0x5000);
        answer(&mut e, FILE_FINDER_GET_FILE, 0);
        answer(&mut e, WORD_AT_FOUR, 0);
        answer(&mut e, MANAGER_TERMINAL_REFERENCE, 0);
        pages(&mut e, &[CLOSE_FADE, RETRY_TIME]);
        e.call_log = Some(vec![]);
        e.call(0x0076_8070, &args![this, 8u32]);
        assert_eq!(calls_to(&e, STRING_SET)[0][1], WORD_FILE);
        assert_eq!(
            calls_to(&e, FILE_FINDER_GET_FILE),
            vec![vec![0x5000, 0, 0x4000, 0x100]]
        );
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![0x12, WORD_FILE_MESSAGE, 0x5000]]
        );
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT).len(), 1);
    }

    #[test]
    fn password_file_without_words_stops_after_the_header() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[INTERVAL_SCALE, DEFAULT_INTERVAL]);
        e.mem.set_f32(DEFAULT_INTERVAL, 100.0);
        e.set(this, HackingMenu::iNumGuesses, 4);
        e.set(this, HackingMenu::iTotGuesses, 1);
        e.set(this, HackingMenu::iYInterval, 30);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let zero = e.mem.alloc(4);
        answer(&mut e, SETTING_VALUE, zero);
        e.register(MAX_OF_TWO, |_, a| a[0].max(a[1]).into_ret());
        answer(&mut e, MENU_RENDER_TEMPLATE, 0);
        stub(
            &mut e,
            &[
                MEMSET,
                STRING_CONSTRUCTOR,
                STRING_DESTRUCTOR,
                SETTING_STRING,
                SETTING_STRING_004C69F0,
                STRING_SPRINTF,
                WORD_AT_ZERO,
                ANIMATING_TEXT_CONSTRUCTOR_WITH_ARGUMENT,
                ANIMATING_TEXT_CONSTRUCTOR,
                TILE_SET_FLOAT,
                TILE_GET_VALUE,
                ANIMATING_TEXT_CONNECT,
                STRLEN,
                // One guess block.
                POINTER_LIST_COUNT,
                TILE_SET_INT_FROM_VALUE,
                POINTER_LIST_ADD_TAIL,
                TILE_VALUE_00700300,
                TILE_SET_TEXT,
                STRING_LENGTH,
            ],
        );
        answer_st0(&mut e, TILE_GET_FLOAT, 40.0);
        e.call_log = Some(vec![]);
        e.call(0x0076_8aa0, &args![this]);
        let lines: Vec<u32> = (0..4)
            .map(|i| e.mem.u32(this.addr() + IO_LINES + 4 * i))
            .collect();
        assert!(lines.iter().all(|&line| line != 0));
        // The guess line's text placed after the third header line.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x104, 0x1005, 40.0f32.to_bits(), 1],
                vec![0x104, 0x1006, 40.0f32.to_bits(), 1]
            ]
        );
        assert_eq!(calls_to(&e, MENU_RENDER_TEMPLATE)[2][3], 0);
        assert_eq!(calls_to(&e, TILE_SET_TEXT).len(), 1);
        // No words: the intro stays and the stage too.
        assert_eq!(e.get(this, HackingMenu::eStage), 0);
    }

    #[test]
    fn create_for_a_locked_out_terminal_finishes_at_once() {
        let mut e = Engine::new();
        pages(
            &mut e,
            &[HACKING_MENU, COPIED_GLOBAL_ONE, PLAYER, STACKING_TYPE_ONE],
        );
        let this = e.new_object::<HackingMenu>();
        for i in 0..8 {
            e.mem.set_u32(this.addr() + TILES + 4 * i, 0x100 + 4 * i);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0076_5870);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, INTERFACE_SINGLETON, 0x6000);
        answer_st0(&mut e, TILE_GET_MAXIMUM_DEPTH, 3.0);
        answer(&mut e, REFERENCE_FORM, 0x800);
        answer(&mut e, FORM_WANTS_WHITE, 0);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 7);
        answer(&mut e, SETTING_VALUE, value);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this.addr());
        answer_st0(&mut e, TILE_GET_FLOAT, 0.0);
        answer(&mut e, PLAYER_TERMINAL_ACCESS, 2);
        stub(
            &mut e,
            &[
                SYSTEM_COLOR_GET_INSTANCE,
                SYSTEM_COLOR_GET_COLOR,
                TILE_READER,
                MENU_SET_MENU_TILE,
                SETTING_STRING,
                TILE_SET_STRING,
                TILE_SET_INT,
                SOUND_HANDLE_PLAY,
                MENU_PREP_FOR_VISIBILITY,
                TILE_SET_FLOAT,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_5b80, &args![0x900u32]).u32(), 0x4444);
        assert_eq!(e.global::<u32>(HACKING_MENU), this.addr());
        assert_eq!(e.global::<u32>(INTERFACE_MANAGER), 0x6000);
        assert_eq!(e.global::<u32>(COPIED_GLOBAL_ONE), 7);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x4444, 0xfad, 3.0f32.to_bits(), 1]]
        );
        assert_eq!(e.get(this, HackingMenu::eStage), 4);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x4444, 0x1006, 1], vec![0x110, 0xfa3, 0]]
        );
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_PLAY),
            vec![vec![this.addr() + 0x12c, 0]]
        );
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![this.addr(), 0]]
        );
        // A menu of another class is deleted.
        let other = e.mem.alloc(8);
        let other_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(other, other_vtable);
        e.mem.set_u32(other_vtable, 0x00c0_0000);
        e.mem.set_u32(other_vtable + 0x34, 0x00c0_0034);
        answer(&mut e, 0x00c0_0034, 0x3f5);
        stub(&mut e, &[0x00c0_0000]);
        answer(&mut e, TILE_GET_MENU, other);
        answer(&mut e, WORD_AT_FOUR, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_5b80, &args![0x900u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![other, 1]]);
    }
}
