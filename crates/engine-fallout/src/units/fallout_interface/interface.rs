//! `fallout/interface/interface.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! This file holds the range `00000000`..`00705a90` of the unit. Almost
//! every function is a static `Interface::` wrapper over the interface
//! manager (`InterfaceManager`, Xbox PDB): `004b7210` returns the manager
//! object (the word at `011d8a80`) and `009373f0` reads its first byte
//! (`bFirstInit`); a wrapper does nothing (returns 0) unless the manager
//! exists and that byte is set. The wrappers read the manager once where
//! the code reads the global again for each call; the global does not
//! change in between.
//!
//! Offsets of the manager object used here (PC offsets match the Xbox PDB):
//! `+0x0c cMenuMode`, `+0x7c bDebugTextVisible`, `+0x9c pMenusRoot`,
//! `+0xdd bFullHelp`, `+0xf0 pPickRef`, `+0x110 bFuzzyActivatePick`,
//! `+0x148 bClickMultithreaded`.
//!
//! The compiler's exception-unwinding frames (`Init`, `007031e0`) are not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `004b7210`: the interface manager object (the word at `011d8a80`).
const GET_MANAGER: u32 = 0x004b_7210;
/// `009373f0`: the byte at the start of the manager (`bFirstInit`).
const MANAGER_READY: u32 = 0x0093_73f0;
/// `InterfaceManager::GetEnterStackTop` (Xbox PDB), on the manager.
const GET_ENTER_STACK_TOP: u32 = 0x0071_4f00;
/// `InterfaceManager::GetEnterStack` (Xbox PDB): the entry of the menu
/// stack at the given index.
const GET_ENTER_STACK: u32 = 0x0071_4f70;
/// `Tile::GetMenuByClass` (Xbox PDB), `cdecl`: the menu with the given id.
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::GetMenu` (Xbox PDB): the menu a tile belongs to.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// Reads the word at `+0x24` of a menu (the engine map names `0059bb30`
/// `D3DTexture_LockRect` because identical code was folded; the body is a
/// plain getter). `1` means the menu is faded in.
const MENU_STATE: u32 = 0x0059_bb30;
/// The menu id on top of the stack that means "one of the five menus in
/// [`TOP_MENU_CANDIDATES`]".
const TOP_PLACEHOLDER: i32 = 1;
/// The menu ids that stand in for [`TOP_PLACEHOLDER`], in the order the code
/// tries them.
const TOP_MENU_CANDIDATES: [i32; 5] = [0x3eb, 0x3ea, 0x40b, 0x425, 0x3ff];
/// First and last menu id of the per-menu flag table at
/// [`MENU_FLAG_TABLE`].
const FIRST_MENU_ID: i32 = 0x3e9;
const LAST_MENU_ID: i32 = 0x43c;
/// Byte table indexed by menu id (`011f308f + id`).
const MENU_FLAG_TABLE: u32 = 0x011f_308f;
/// `00709c00`: true when the manager's `+0x4bc` word is 3.
const MANAGER_MODE_IS_THREE: u32 = 0x0070_9c00;
/// `MoviePlayer::GetPlayingSequence` (Xbox PDB), on the movie player whose
/// pointer is at `0126fac4`.
const MOVIE_PLAYER_PLAYING: u32 = 0x00ec_17c0;
/// The movie player pointer.
const MOVIE_PLAYER: u32 = 0x0126_fac4;

/// Tile trait setter (`00700320`): tile, trait id, value.
const TILE_SET_VALUE: u32 = 0x0070_0320;
/// Tile trait getter (`00a011b0`, value in ST0): tile, trait id.
const TILE_GET_VALUE: u32 = 0x00a0_11b0;
/// `Tile::GetChildByID` (Xbox PDB).
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
/// Trait id whose value (110.0, 111.0 or 112.0) tells the isolate code what
/// part a tile plays (`0xfaa`).
const TRAIT_ROLE: u32 = 0xfaa;
/// Trait id read as "enabled" (`0xfa3`).
const TRAIT_ENABLED: u32 = 0xfa3;
/// Trait id written as "visible" (`0x1779`).
const TRAIT_VISIBLE: u32 = 0x1779;
/// `110.0`, `111.0`, `112.0` and `0.0` (`double`s the compares read).
const DOUBLE_110: u32 = 0x0106_ebc8;
const DOUBLE_111: u32 = 0x0106_ebc0;
const DOUBLE_112: u32 = 0x0106_ebd0;
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `0056c7f0`: the node a tile keeps (the first word of its `+0x2c`).
const TILE_NODE: u32 = 0x0056_c7f0;
/// `00450f90`: sets a flag (byte argument) on a node.
const SET_NODE_FLAG: u32 = 0x0045_0f90;
/// `00559450`: the first word of the object (a list head, a string's
/// character pointer).
const FIRST_WORD: u32 = 0x0055_9450;
/// `00586150`: the manager's `pMenusRoot` (`+0x9c`).
const GET_MENUS_ROOT: u32 = 0x0058_6150;
/// `0057cbe0`: list iterator step. The first argument word is the list
/// object, the second the address of the cursor word; it returns the
/// address of the current item and moves the cursor to the next node.
const LIST_NEXT: u32 = 0x0057_cbe0;
/// `00905820` on the manager's sub-object at `+0x4c4`; argument: the
/// address of a tile pointer.
const SUBOBJECT_ADD_TILE: u32 = 0x0090_5820;
/// `00470470` on that sub-object, no arguments.
const SUBOBJECT_CLEAR: u32 = 0x0047_0470;
/// Offset of that sub-object in the manager.
const MANAGER_SUBOBJECT: u32 = 0x4c4;
/// `005e3fc0` / `004fd3c0`: getters on the manager for two nodes whose
/// flag the isolate code sets.
const MANAGER_NODE_A: u32 = 0x005e_3fc0;
const MANAGER_NODE_B: u32 = 0x004f_d3c0;

/// `InterfaceManager::Initialize` (Xbox PDB), `cdecl`: two words.
const MANAGER_INITIALIZE: u32 = 0x0070_9fd0;
/// Scope guard constructor (`00404eb0`, `thiscall`: 0xd, 1, file, line) and
/// destructor (`00404ee0`); the guard is a 4-byte local.
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\Interface\Interface.cpp"`.
const INTERFACE_CPP_PATH: u32 = 0x0106_eb78;

/// Menu ids the visibility functions of this unit switch between.
const MENU_STATS: i32 = 0x3eb;
const MENU_INVENTORY: i32 = 0x3ea;
const MENU_REPAIR: i32 = 0x40b;
const MENU_ITEM_MOD: i32 = 0x425;
const MENU_MAP: i32 = 0x3ff;
/// `Menu::StartFadeOut` / `Menu::StartFadeIn` (Xbox PDB), on a menu.
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const MENU_START_FADE_IN: u32 = 0x00a1_db20;
/// Sets the state word (`+0x24`) of a menu; argument: the state.
const MENU_SET_STATE: u32 = 0x0070_37c0;
/// State the menus are put in when they are hidden.
const MENU_STATE_HIDDEN: u32 = 4;
/// The tile of a menu (the word at `+4`).
const MENU_TILE: u32 = 0x0072_6070;
/// Trait of the menus root tile that holds the id of the menu the pause
/// tabs last made visible (`0x1771`); the visibility functions write the id
/// they show.
const TRAIT_CURRENT_MENU: u32 = 0x1771;
/// `_ftol2` (x87 value in `ST0`).
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// Getter on the manager for the word at `+0x174`; the object that
/// `007f8a80` and `007fa010` work on (the engine map puts both in
/// `fopipboymanager.cpp`).
const MANAGER_PIPBOY: u32 = 0x0070_4370;
const PIPBOY_REFRESH: u32 = 0x007f_8a80;
/// Takes the object and a tab number.
const PIPBOY_SELECT_TAB: u32 = 0x007f_a010;
/// `00974d90` on that object: true when the word at its start is set.
const PIPBOY_QUERY: u32 = 0x0097_4d90;
/// `MenuConsole::Instance` (Xbox PDB), `cdecl`: `0` answers the instance
/// if it exists, `1` creates it when needed.
const CONSOLE_INSTANCE: u32 = 0x0071_b160;
/// `004a4020` on the console: true when it is visible.
const CONSOLE_IS_VISIBLE: u32 = 0x004a_4020;
/// `MenuConsole::ToggleVisible` (Xbox PDB), on the console.
const CONSOLE_TOGGLE_VISIBLE: u32 = 0x0071_d580;
/// `005b6f70`: true when the console may take text while it is closed.
const CONSOLE_ACCEPTS_TEXT: u32 = 0x005b_6f70;
/// `0071d0a0` on the console: format string pointer and a `va_list`.
const CONSOLE_PRINT: u32 = 0x0071_d0a0;
/// `BSString::dtor` (`004037d0`) and constructor (`004037b0`) of the
/// 8-byte string local.
const STRING_CTOR: u32 = 0x0040_37b0;
const STRING_DTOR: u32 = 0x0040_37d0;
/// Message creation of `messagemenu.cpp` (`007a8e60`): fifteen words, the
/// seventh a `va_list`.
const MESSAGE_CREATE: u32 = 0x007a_8e60;
/// The byte the message-box loop of `00704010` waits on (`-1` until the
/// callback `00704130` stores the button).
const MESSAGE_RESULT: u32 = 0x0119_f349;
/// The object read from the global `011dea0c` (used as the `this` of
/// `00877720`).
const CONTROLS_OBJECT_GLOBAL: u32 = 0x011d_ea0c;
/// `FaderManager::RemoveFader` (Xbox PDB) and the global holding the fader
/// manager.
const REMOVE_FADER: u32 = 0x0070_10e0;
const FADER_MANAGER_GLOBAL: u32 = 0x011d_8804;
/// `StartMenu::Close` (Xbox PDB, `cdecl`), `InterfaceManager::
/// EnterRenderedMenu`, `PopFromEnterStack` and `AddToEnterStack` (Xbox PDB),
/// the last three on the manager.
const START_MENU_CLOSE: u32 = 0x007c_e7a0;
const ENTER_RENDERED_MENU: u32 = 0x0071_81d0;
const POP_FROM_ENTER_STACK: u32 = 0x0071_4fd0;
const ADD_TO_ENTER_STACK: u32 = 0x0071_4d90;
/// `00403df0` on a setting-like object: its text; `004046f0(buffer, text)`
/// copies a string (`cdecl`); `00438390` on a string local assigns it a
/// character pointer; `BSstristr` (`cdecl`) and `sprintf` (`cdecl`).
const SETTING_TEXT: u32 = 0x0040_3df0;
const STRING_COPY: u32 = 0x0040_46f0;
const STRING_ASSIGN: u32 = 0x0043_8390;
const STRING_STRISTR: u32 = 0x004b_75a0;
const SPRINTF: u32 = 0x00ec_623a;
/// `00877720` on the global at `011dea0c`: the controls object, which
/// `a23010` (`Controls::Poll`, Xbox PDB) polls.
const CONTROLS_OBJECT: u32 = 0x0087_7720;
const CONTROLS_POLL: u32 = 0x00a2_3010;
/// `00403d30(block, value, size)`: fills memory (`cdecl`).
const MEMORY_FILL: u32 = 0x0040_3d30;
/// Imports of the message pump: `PeekMessageA`, `TranslateMessage`,
/// `DispatchMessageA` (by their import slots).
const PEEK_MESSAGE: u32 = 0x00fd_f29c;
const TRANSLATE_MESSAGE: u32 = 0x00fd_f298;
const DISPATCH_MESSAGE: u32 = 0x00fd_f294;
/// `StatsMenu::ToggleHealingMode`, `RepairMenu::ResetMenu`,
/// `ItemModMenu::ResetMenu`, `MapMenu::UpdateMenu`,
/// `InventoryMenu::UpdateInventoryMenu`, `InventoryMenu::SavePageState` and
/// `StatsMenu::UpdateHealingOptions` (Xbox PDB; the last takes a menu).
const STATS_TOGGLE_HEALING_MODE: u32 = 0x0070_4390;
const REPAIR_RESET_MENU: u32 = 0x007b_6fb0;
const ITEM_MOD_RESET_MENU: u32 = 0x0078_46a0;
const MAP_MENU_UPDATE: u32 = 0x0079_aba0;
const INVENTORY_UPDATE_MENU: u32 = 0x0078_2a90;
const INVENTORY_SAVE_PAGE_STATE: u32 = 0x0077_ffd0;
const STATS_UPDATE_HEALING_OPTIONS: u32 = 0x007d_f230;

/// Reads the manager and applies the wrappers' guard: `Some(manager)` only
/// when it exists and `009373f0` says so.
fn ready_manager(e: &mut Engine) -> Option<u32> {
    if e.call(GET_MANAGER, &[]).u32() == 0 {
        return None;
    }
    let manager = e.call(GET_MANAGER, &[]).u32();
    if e.call(MANAGER_READY, &args![manager]).bool() {
        Some(manager)
    } else {
        None
    }
}

fn tile_value(e: &mut Engine, tile: u32, id: u32) -> f64 {
    e.call(TILE_GET_VALUE, &args![tile, id]).f32() as f64
}

fn set_tile_visible(e: &mut Engine, tile: u32, visible: u32) {
    e.call(TILE_SET_VALUE, &args![tile, TRAIT_VISIBLE, visible]);
}

/// `true` when the tile's "enabled" trait or its "visible" trait is zero:
/// the value the code gives the tile node's flag.
fn node_flag_for(e: &mut Engine, tile: u32) -> bool {
    let zero: f64 = e.global(DOUBLE_ZERO);
    tile_value(e, tile, TRAIT_ENABLED) == zero || tile_value(e, tile, TRAIT_VISIBLE) == zero
}

/// Sets the flag of the tile's node (`00450f90` on `0056c7f0`'s result).
fn set_tile_node_flag(e: &mut Engine, tile: u32, flag: bool) {
    let node = e.call(TILE_NODE, &args![tile]).u32();
    e.call(SET_NODE_FLAG, &args![node, flag]);
}

/// The walk the isolate and restore code does over a tile list: the cursor
/// is a word in memory that `0057cbe0` advances; each item is a pointer to
/// a tile.
fn for_each_list_item(
    e: &mut Engine,
    first: u32,
    list_owner: u32,
    mut body: impl FnMut(&mut Engine, u32),
) {
    e.with_stack(4, |e, cursor| {
        e.mem.set_u32(cursor.addr(), first);
        while e.mem.u32(cursor.addr()) != 0 {
            let item = e.call(LIST_NEXT, &args![list_owner, cursor]).u32();
            let value = e.mem.u32(item);
            body(e, value);
        }
    });
}

// Translated from 00702250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::Init` (Xbox PDB): inside a scope guard (line 0xb0 of
/// `Interface.cpp`), initializes the interface manager with `(1, flag)`.
pub fn interface_init(e: &mut Engine, flag: u8) {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0xdu32, 1u32, INTERFACE_CPP_PATH, 0xb0u32],
        );
        e.call(MANAGER_INITIALIZE, &args![1u32, flag]);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 007022c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As [`interface_init`] with line 0xbd and the arguments `(2, 1)`.
pub fn fn_007022c0(e: &mut Engine) {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0xdu32, 1u32, INTERFACE_CPP_PATH, 0xbdu32],
        );
        e.call(MANAGER_INITIALIZE, &args![2u32, 1u32]);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 00702330 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: calls `0070a0b0` on it.
pub fn fn_00702330(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0070_a0b0, &args![manager]);
    }
}

// Translated from 00702360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsInMenuMode` (Xbox PDB): the manager is ready and its menu
/// mode is not 1.
pub fn interface_is_in_menu_mode(e: &mut Engine) -> bool {
    match ready_manager(e) {
        Some(manager) => fn_007023a0(e, manager),
        None => false,
    }
}

// Translated from 007023a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cMenuMode` of the manager (`+0x0c`) is not 1.
pub fn fn_007023a0(e: &mut Engine, this: u32) -> bool {
    e.mem.u32(this + 0x0c) != 1
}

// Translated from 007023c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetTopMenuID` (Xbox PDB): `GetEnterStackTop` of the manager,
/// 0 when it is not ready.
pub fn interface_get_top_menu_id(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(GET_ENTER_STACK_TOP, &args![manager]).u32(),
        None => 0,
    }
}

// Translated from 00702400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetEnterStack(index)` of the manager, 0 when it is not ready.
pub fn fn_00702400(e: &mut Engine, index: u32) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(GET_ENTER_STACK, &args![manager, index]).u32(),
        None => 0,
    }
}

// Translated from 00702440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the constant 10.
pub fn fn_00702440(_e: &mut Engine) -> u32 {
    10
}

// Translated from 00702450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsTopMenuID` (Xbox PDB): whether the top of the menu stack
/// is the menu `menu_id`. When the top is the placeholder id 1 and
/// `menu_id` is one of the five ids that stand in for it, the answer is
/// whether that menu is visible instead.
pub fn interface_is_top_menu_id(e: &mut Engine, menu_id: i32) -> u8 {
    let Some(manager) = ready_manager(e) else {
        return 0;
    };
    let top = e.call(GET_ENTER_STACK_TOP, &args![manager]).i32();
    if top == TOP_PLACEHOLDER && TOP_MENU_CANDIDATES.contains(&menu_id) {
        return interface_is_menu_id_visible(e, menu_id, 0);
    }
    (top == menu_id) as u8
}

// Translated from 007024e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsTopMenuFadedIn` (Xbox PDB): finds the top menu (for the
/// placeholder id 1, the first visible one of the five stand-in menus) and
/// returns whether its state word (`0059bb30`) is 1.
pub fn interface_is_top_menu_faded_in(e: &mut Engine) -> bool {
    let Some(manager) = ready_manager(e) else {
        return false;
    };
    let top = e.call(GET_ENTER_STACK_TOP, &args![manager]).i32();
    let mut menu = 0;
    if top == TOP_PLACEHOLDER {
        for id in TOP_MENU_CANDIDATES {
            if interface_is_menu_id_visible(e, id, 0) != 0 {
                menu = e.call(GET_MENU_BY_CLASS, &args![id]).u32();
                break;
            }
        }
    } else {
        menu = e.call(GET_MENU_BY_CLASS, &args![top]).u32();
    }
    if menu == 0 {
        return false;
    }
    let inner = e.call(TILE_GET_MENU, &args![menu]).u32();
    if inner == 0 {
        return false;
    }
    let inner = e.call(TILE_GET_MENU, &args![menu]).u32();
    e.call(MENU_STATE, &args![inner]).u32() == 1
}

// Translated from 00702640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetMenuModeType` (Xbox PDB): `GetEnterStack(0)` of the
/// manager, 0 when it is not ready.
pub fn interface_get_menu_mode_type(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(GET_ENTER_STACK, &args![manager, 0u32]).u32(),
        None => 0,
    }
}

// Translated from 00702680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsMenuIDVisible` (Xbox PDB). For the placeholder id 1:
/// `00709c00` must hold and one of the five stand-in menus must be visible
/// (with the same mask). Otherwise, when `mask` is nonzero and the menu
/// exists with a menu tile, the menu's state word must have a bit of
/// `mask`; then the answer is the per-id flag byte ([`fn_007027b0`]).
pub fn interface_is_menu_id_visible(e: &mut Engine, menu_id: i32, mask: u32) -> u8 {
    if ready_manager(e).is_none() {
        return 0;
    }
    if menu_id == TOP_PLACEHOLDER {
        if !e.call(MANAGER_MODE_IS_THREE, &[]).bool() {
            return 0;
        }
        for id in TOP_MENU_CANDIDATES {
            if interface_is_menu_id_visible(e, id, mask) != 0 {
                return 1;
            }
        }
        return 0;
    }
    if mask != 0 {
        let menu = e.call(GET_MENU_BY_CLASS, &args![menu_id]).u32();
        if menu != 0 {
            let inner = e.call(TILE_GET_MENU, &args![menu]).u32();
            if inner != 0 {
                let inner = e.call(TILE_GET_MENU, &args![menu]).u32();
                let state = e.call(MENU_STATE, &args![inner]).u32();
                if state & mask == 0 {
                    return 0;
                }
            }
        }
    }
    fn_007027b0(e, menu_id)
}

// Translated from 007027b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The flag byte of menu `menu_id` in the table at `011f308f`, 0 for an id
/// outside `0x3e9..=0x43c`.
pub fn fn_007027b0(e: &mut Engine, menu_id: i32) -> u8 {
    if !(FIRST_MENU_ID..=LAST_MENU_ID).contains(&menu_id) {
        return 0;
    }
    e.mem.u8(MENU_FLAG_TABLE.wrapping_add(menu_id as u32))
}

// Translated from 007027e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `InterfaceManager::PreIdleStuff` (Xbox PDB).
pub fn fn_007027e0(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0070_b8f0, &args![manager]);
    }
}

// Translated from 00702810 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `InterfaceManager::Idle` (Xbox PDB).
pub fn fn_00702810(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0070_c4a0, &args![manager]);
    }
}

// Translated from 00702840 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: calls `00711ea0` on it.
pub fn fn_00702840(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0071_1ea0, &args![manager]);
    }
}

/// `MoviePlayer::GetPlayingSequence` of the global movie player.
fn movie_playing(e: &mut Engine) -> bool {
    let player = e.mem.u32(MOVIE_PLAYER);
    e.call(MOVIE_PLAYER_PLAYING, &args![player]).bool()
}

// Translated from 00702870 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready and no movie sequence is playing: calls
/// `00713f00` on the manager.
pub fn fn_00702870(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        if !movie_playing(e) {
            e.call(0x0071_3f00, &args![manager]);
        }
    }
}

// Translated from 007028b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::Click` (Xbox PDB): when the manager is ready and no movie is
/// playing, hands the click `(x, y)` to `00713fb0`; otherwise sets the
/// render thread stage of both rendering threads to 0x17 (`004ea970` returns
/// the rendering system, `00ba30f0` / `00ba3130` are its stage setters).
/// Always clears `bClickMultithreaded` (`+0x148`) afterwards.
pub fn interface_click(e: &mut Engine, x: u32, y: u32) {
    let mut handled = false;
    if let Some(manager) = ready_manager(e) {
        if !movie_playing(e) {
            e.call(0x0071_3fb0, &args![manager, x, y]);
            handled = true;
        }
    }
    if !handled {
        // 0070290a / 0070291a: `004ea970` takes no word (plain RET, a global); the pushed
        // stage and 0x17 are the two words of `00ba30f0` / `00ba3130` (RET 8).
        let system = e.call(0x004e_a970, &args![]).u32();
        e.call(0x00ba_30f0, &args![system, 0u32, 0x17u32]);
        let system = e.call(0x004e_a970, &args![]).u32();
        e.call(0x00ba_3130, &args![system, 1u32, 0x17u32]);
    }
    let manager = e.call(GET_MANAGER, &[]).u32();
    fn_00702940(e, manager, 0);
}

// Translated from 00702940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` in `bClickMultithreaded` (`+0x148`) of the
/// manager.
pub fn fn_00702940(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0x148, value);
}

// Translated from 00702960 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: asks the movie player whether a sequence is
/// playing and drops the answer.
pub fn fn_00702960(e: &mut Engine) {
    if ready_manager(e).is_some() {
        movie_playing(e);
    }
}

// Translated from 00702990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the visibility of one menu tile `tile` and of those children
/// whose role trait (`0xfaa`) is 110.0 or 111.0; `flag_a`, `flag_b` and
/// `flag_c` are the bytes `IsolateMenuElements` passes (`flag_c` is always 1
/// there). Does nothing for a null tile.
pub fn fn_00702990(e: &mut Engine, tile: u32, flag_a: u8, flag_b: u8, flag_c: u8) {
    if tile == 0 {
        return;
    }
    let manager = e.call(GET_MANAGER, &[]).u32();
    let menus_root = e.call(GET_MENUS_ROOT, &args![manager]).u32();
    let manager = e.call(GET_MANAGER, &[]).u32();
    let role = tile_value(e, tile, TRAIT_ROLE);
    let is_112 = role == e.global::<f64>(DOUBLE_112);
    let visible = if (flag_b != 0 && !is_112) || (flag_b == 0 && is_112) {
        0
    } else {
        1
    };
    set_tile_visible(e, tile, visible);
    let first = e.call(FIRST_WORD, &args![tile + 4]).u32();
    for_each_list_item(e, first, menus_root + 4, |e, child| {
        let role = tile_value(e, child, TRAIT_ROLE);
        let tile_110 = if role == e.global::<f64>(DOUBLE_110) {
            child
        } else {
            0
        };
        let role = tile_value(e, child, TRAIT_ROLE);
        let tile_111 = if role == e.global::<f64>(DOUBLE_111) {
            child
        } else {
            0
        };
        if tile_110 != 0 {
            if flag_a == 0 {
                set_tile_visible(e, tile_110, 0);
            } else {
                set_tile_visible(e, tile_110, 1);
                if flag_c != 0 && flag_b == 0 && !is_112 {
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), tile_110);
                        e.call(
                            SUBOBJECT_ADD_TILE,
                            &args![manager + MANAGER_SUBOBJECT, slot],
                        );
                    });
                }
            }
        }
        if tile_111 != 0 {
            set_tile_visible(e, tile_111, (flag_a == 0) as u32);
        }
        if tile_110 != 0 && e.call(TILE_NODE, &args![tile_110]).u32() != 0 {
            let flag = node_flag_for(e, tile_110);
            set_tile_node_flag(e, tile_110, flag);
        }
        if tile_111 != 0 && e.call(TILE_NODE, &args![tile_111]).u32() != 0 {
            let flag = flag_a != 0 || node_flag_for(e, tile_111);
            set_tile_node_flag(e, tile_111, flag);
        }
    });
    if e.call(TILE_NODE, &args![tile]).u32() != 0 {
        let flag = node_flag_for(e, tile);
        set_tile_node_flag(e, tile, flag);
    }
}

// Translated from 00702c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsolateMenuElements` (Xbox PDB): when the manager is ready,
/// applies [`fn_00702990`] with `(tile, a, b, 1)` to every menu tile of the
/// menus root (when `a` is set and `b` is not, first clears the manager's
/// sub-object at `+0x4c4`), then sets the flag of the manager's two nodes
/// (`005e3fc0`, `004fd3c0`) to 0 when both `a` and `b` are zero, else 1.
pub fn interface_isolate_menu_elements(e: &mut Engine, a: u8, b: u8) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    if a != 0 && b == 0 {
        e.call(SUBOBJECT_CLEAR, &args![manager + MANAGER_SUBOBJECT]);
    }
    let menus_root = e.call(GET_MENUS_ROOT, &args![manager]).u32();
    let first = e.call(FIRST_WORD, &args![menus_root + 4]).u32();
    for_each_list_item(e, first, menus_root + 4, |e, menu| {
        fn_00702990(e, menu, a, b, 1);
    });
    let flag = !(a == 0 && b == 0);
    for getter in [MANAGER_NODE_A, MANAGER_NODE_B] {
        if e.call(getter, &args![manager]).u32() != 0 {
            let node = e.call(getter, &args![manager]).u32();
            e.call(SET_NODE_FLAG, &args![node, flag]);
        }
    }
}

// Translated from 00702dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::RestoreMenuElements` (Xbox PDB): for every menu tile of the
/// menus root, makes it and its children `0x6e` and `0x6f` visible and
/// recomputes their node flags.
pub fn interface_restore_menu_elements(e: &mut Engine) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let menus_root = e.call(GET_MENUS_ROOT, &args![manager]).u32();
    let first = e.call(FIRST_WORD, &args![menus_root + 4]).u32();
    for_each_list_item(e, first, menus_root + 4, |e, menu| {
        let child_6e = e.call(TILE_GET_CHILD_BY_ID, &args![menu, 0x6eu32]).u32();
        let child_6f = e.call(TILE_GET_CHILD_BY_ID, &args![menu, 0x6fu32]).u32();
        set_tile_visible(e, menu, 1);
        if child_6e != 0 {
            set_tile_visible(e, child_6e, 1);
        }
        if child_6f != 0 {
            set_tile_visible(e, child_6f, 1);
        }
        for tile in [menu, child_6e, child_6f] {
            if tile != 0 && e.call(TILE_NODE, &args![tile]).u32() != 0 {
                let flag = node_flag_for(e, tile);
                set_tile_node_flag(e, tile, flag);
            }
        }
    });
}

// Translated from 00702fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsDebugTextVisible` (Xbox PDB): `00644640` on the manager
/// when it is ready.
pub fn interface_is_debug_text_visible(e: &mut Engine) -> bool {
    match ready_manager(e) {
        Some(manager) => e.call(0x0064_4640, &args![manager]).bool(),
        None => false,
    }
}

// Translated from 00703000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ToggleDebugTextVisible` (Xbox PDB): when the manager is
/// ready, [`fn_00703030`] on it.
pub fn interface_toggle_debug_text_visible(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        fn_00703030(e, manager);
    }
}

// Translated from 00703030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips `bDebugTextVisible` (`+0x7c`) through [`fn_00703060`].
pub fn fn_00703030(e: &mut Engine, this: u32) {
    let visible = e.mem.u8(this + 0x7c) != 0;
    fn_00703060(e, this, !visible as u8);
}

// Translated from 00703060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` in `bDebugTextVisible` (`+0x7c`).
pub fn fn_00703060(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0x7c, value);
}

// Translated from 00703080 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `007151b0(manager, a, b)` with two floats.
pub fn fn_00703080(e: &mut Engine, a: f32, b: f32) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0071_51b0, &args![manager, a, b]);
    }
}

// Translated from 007030c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `00715e00(manager, a, b)` with two words.
pub fn fn_007030c0(e: &mut Engine, a: u32, b: u32) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0071_5e00, &args![manager, a, b]);
    }
}

// Translated from 00703100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ToggleFullHelp` (Xbox PDB): flips `bFullHelp` (`+0xdd`).
pub fn interface_toggle_full_help(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        let full = e.mem.u8(manager + 0xdd) != 0;
        e.mem.set_u8(manager + 0xdd, !full as u8);
    }
}

// Translated from 00703150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetFullHelp` (Xbox PDB): `bFullHelp` (`+0xdd`), 0 when the
/// manager is not ready.
pub fn interface_get_full_help(e: &mut Engine) -> u8 {
    match ready_manager(e) {
        Some(manager) => e.mem.u8(manager + 0xdd),
        None => 0,
    }
}

// Translated from 00703180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetTargetREFR` (Xbox PDB): `pPickRef` through
/// [`fn_007031c0`], 0 when the manager is not ready.
pub fn interface_get_target_refr(e: &mut Engine) -> u32 {
    let manager = e.call(GET_MANAGER, &[]).u32();
    if manager != 0 && e.call(MANAGER_READY, &args![manager]).bool() {
        fn_007031c0(e, manager)
    } else {
        0
    }
}

// Translated from 007031c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pPickRef` (`+0xf0`) of the manager.
pub fn fn_007031c0(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this + 0xf0)
}

// Translated from 007031e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: for a non-null reference prints
/// `"<name>" (<form id>)` as debug text (`DebugText::Print`), then calls
/// `00714d70(manager, reference)` in every case. The text goes at x =
/// 640.0 (`0103a1c4`), y = the integer the setting at `011debac` holds,
/// with the arguments `2, -1, -1.0, 0, 0`.
///
/// A chain of getters (`0043d4d0`, `005bd5b0`, `005bd5c0`, `005bd580`,
/// `_ftol2`) computes a line number that the code never uses; the calls
/// are kept.
pub fn fn_007031e0(e: &mut Engine, reference: u32) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    e.with_stack(8, |e, text| {
        e.call(0x0040_37b0, &args![text]);
        if reference != 0 {
            let form_id = e.call(0x0084_e3a0, &args![reference]).u32();
            let name = e.call(0x0055_d520, &args![reference]).u32();
            e.call(0x0040_6f60, &args![text, 0x0103_a1c8u32, name, form_id]);
            let setting = e.call(0x0043_d4d0, &args![0x011f_33c8u32]).u32();
            let value = e.mem.u32(setting).wrapping_sub(1);
            // 00703275: `005bd5b0` takes no word (plain RET, a global); the pushed value
            // is the word of `005bd5c0` (RET 4).
            let a = e.call(0x005b_d5b0, &args![]).u32();
            let b = e.call(0x005b_d5c0, &args![a, value]).u32();
            let height = e.call(0x005b_d580, &args![b]).f32();
            let _unused_line = e
                .call(0x00ec_62c0, &args![height as f64])
                .i32()
                .wrapping_add(3);
            let y_setting = e.call(0x0043_d4d0, &args![0x011d_ebacu32]).u32();
            let y = e.mem.i32(y_setting);
            let characters = e.call(FIRST_WORD, &args![text]).u32();
            let x: f32 = e.global(0x0103_a1c4);
            let minus_one: f32 = e.global(0x0101_2054);
            let debug_text = e.call(0x00a0_d9e0, &args![1u32]).u32();
            e.call(
                0x00a0_f8b0,
                &args![debug_text, characters, x, y as f32, 2u32, -1i32, minus_one, 0u32, 0u32],
            );
        }
        e.call(0x0071_4d70, &args![manager, reference]);
        e.call(0x0040_37d0, &args![text]);
    });
}

// Translated from 00703310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetPickREFR` (Xbox PDB): `009bdf60(manager, reference)`
/// when the manager is ready.
pub fn interface_set_pick_refr(e: &mut Engine, reference: u32) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x009b_df60, &args![manager, reference]);
    }
}

// Translated from 00703350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00874480` on the manager when it is ready, else 0.
pub fn fn_00703350(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(0x0087_4480, &args![manager]).u32(),
        None => 0,
    }
}

// Translated from 00703390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetTelekinesisREFR` (Xbox PDB): `009b4440` on the manager
/// when it is ready, else 0.
pub fn interface_get_telekinesis_refr(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(0x009b_4440, &args![manager]).u32(),
        None => 0,
    }
}

// Translated from 007033d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a 3-float point at `out` and returns `out`: `009b7010(manager,
/// out)` fills it when the manager is ready, else it is copied from the
/// three words at `011f426c`.
pub fn fn_007033d0(e: &mut Engine, out: Ptr) -> Ptr {
    if let Some(manager) = ready_manager(e) {
        e.call(0x009b_7010, &args![manager, out]);
        return out;
    }
    for i in 0..3 {
        let word = e.mem.u32(0x011f_426c + 4 * i);
        e.mem.set_u32(out.addr() + 4 * i, word);
    }
    out
}

// Translated from 00703430 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00703470`] on the manager when it is ready, else false.
pub fn fn_00703430(e: &mut Engine) -> bool {
    match ready_manager(e) {
        Some(manager) => fn_00703470(e, manager),
        None => false,
    }
}

// Translated from 00703470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bFuzzyActivatePick` (`+0x110`) of the manager.
pub fn fn_00703470(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 0x110) != 0
}

/// Runs `f` with the address of the given words in memory, the way a
/// `va_list` points at the caller's stack words.
fn with_va_list<R>(e: &mut Engine, words: &[u32], f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(4 * words.len().max(1) as u32, |e, block| {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(block.addr() + 4 * i as u32, *word);
        }
        f(e, block.addr())
    })
}

/// The same for the callers that pass `&va_list`: the address of a local
/// word that holds the address of the words.
fn with_va_list_cell<R>(e: &mut Engine, words: &[u32], f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    with_va_list(e, words, |e, list| {
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), list);
            f(e, cell.addr())
        })
    })
}

/// Starts the fade-out of the menu of `tile` (`Tile::GetMenu`, then
/// `Menu::StartFadeOut`).
fn menu_fade_out(e: &mut Engine, tile: u32) {
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(MENU_START_FADE_OUT, &args![menu]);
}

/// The same with `Menu::StartFadeIn`.
fn menu_fade_in(e: &mut Engine, tile: u32) {
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(MENU_START_FADE_IN, &args![menu]);
}

/// Writes the menu id into the menus root's [`TRAIT_CURRENT_MENU`].
fn set_current_menu(e: &mut Engine, manager: u32, menu_id: i32) {
    let root = e.call(GET_MENUS_ROOT, &args![manager]).u32();
    e.call(TILE_SET_VALUE, &args![root, TRAIT_CURRENT_MENU, menu_id]);
}

/// The Pipboy tab refresh the visibility functions end with.
fn refresh_pipboy_tab(e: &mut Engine, manager: u32, tab: u32) {
    let pipboy = e.call(MANAGER_PIPBOY, &args![manager]).u32();
    e.call(PIPBOY_REFRESH, &args![pipboy]);
    let pipboy = e.call(MANAGER_PIPBOY, &args![manager]).u32();
    e.call(PIPBOY_SELECT_TAB, &args![pipboy, tab]);
}

/// Calls `callee(0)` when the Pipboy object answers true.
fn call_if_pipboy_query(e: &mut Engine, manager: u32, callee: u32) {
    let pipboy = e.call(MANAGER_PIPBOY, &args![manager]).u32();
    if e.call(PIPBOY_QUERY, &args![pipboy]).bool() {
        e.call(callee, &args![0u32]);
    }
}

// Translated from 00703490 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `0070bc20` on it.
pub fn fn_00703490(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0070_bc20, &args![manager]);
    }
}

// Translated from 007034c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `00984f60(manager, value)`.
pub fn fn_007034c0(e: &mut Engine, value: u32) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0098_4f60, &args![manager, value]);
    }
}

// Translated from 00703500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ShowMenus` (Xbox PDB): shows the menu the menus root's
/// [`TRAIT_CURRENT_MENU`] names (when it is 0, `00717600` runs on the
/// manager and the stats menu is shown): the stats menu for `0x3eb`, the
/// map for `0x3ff`, the inventory for `0x3ea`, `0x40b` and `0x425`; any
/// other id shows nothing.
pub fn interface_show_menus(e: &mut Engine) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let root = e.call(GET_MENUS_ROOT, &args![manager]).u32();
    let current = tile_value(e, root, TRAIT_CURRENT_MENU);
    let mut menu_id = e.call(FLOAT_TO_INT, &args![current]).i32();
    if menu_id == 0 {
        e.call(0x0071_7600, &args![manager]);
        menu_id = MENU_STATS;
    }
    match menu_id {
        MENU_INVENTORY | MENU_REPAIR | MENU_ITEM_MOD => {
            interface_set_inventory_menu_visible(e, 1, 0, 1)
        }
        MENU_STATS => {
            // `SetStatsMenuVisible` (`00704c10`), `cdecl`.
            e.call(0x0070_4c10, &args![1u32, 0u32]);
        }
        MENU_MAP => interface_set_map_menu_visible(e, 1, 0),
        _ => {}
    }
}

// Translated from 00703610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::HideMenus` (Xbox PDB): for each of the stats, inventory,
/// repair, item-mod and map menus that exist, puts its menu in state 4
/// (`MENU_SET_STATE`) and sets the "enabled" trait (`0xfa3`) of its tile
/// to 0. Then `00704f50`, closes the start menu (`StartMenu::Close(1)`)
/// when `004a4040` says it is open, and
/// `InterfaceManager::EnterRenderedMenu(0)`.
pub fn interface_hide_menus(e: &mut Engine) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let mut tiles = [0u32; 5];
    for (slot, id) in tiles.iter_mut().zip([
        MENU_STATS,
        MENU_INVENTORY,
        MENU_REPAIR,
        MENU_ITEM_MOD,
        MENU_MAP,
    ]) {
        *slot = e.call(GET_MENU_BY_CLASS, &args![id]).u32();
    }
    for tile in tiles {
        if tile != 0 {
            let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            e.call(MENU_SET_STATE, &args![menu, MENU_STATE_HIDDEN]);
            let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            let menu_tile = e.call(MENU_TILE, &args![menu]).u32();
            e.call(TILE_SET_VALUE, &args![menu_tile, TRAIT_ENABLED, 0u32]);
        }
    }
    e.call(0x0070_4f50, &[]);
    if e.call(0x004a_4040, &[]).bool() {
        e.call(START_MENU_CLOSE, &args![1u32]);
    }
    e.call(ENTER_RENDERED_MENU, &args![manager, 0u32]);
}

// Translated from 007037e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::EmergencyCloseAllMenusAndBreakStuff` (Xbox PDB): the
/// manager's method of the same name, when the manager is ready.
pub fn interface_emergency_close_all_menus_and_break_stuff(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0071_4c60, &args![manager]);
    }
}

// Translated from 00703810 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: sets the flag of the node of the menus root
/// tile to `value == 0`.
pub fn fn_00703810(e: &mut Engine, value: u8) {
    if let Some(manager) = ready_manager(e) {
        let root = e.call(GET_MENUS_ROOT, &args![manager]).u32();
        set_tile_node_flag(e, root, value == 0);
    }
}

// Translated from 00703860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetCrosshairTargetType` (Xbox PDB): when the object at
/// `011dea3c` exists and its slot `0x1d0` answers nonzero,
/// `HUDMainMenu::SetTargetType(target_type, 0)`.
pub fn interface_set_crosshair_target_type(e: &mut Engine, target_type: u32) {
    let object = e.global::<u32>(0x011d_ea3c);
    if object != 0 && e.vcall(object, 0x1d0, &[]).u32() != 0 {
        e.call(0x0077_2d10, &args![target_type, 0u32]);
    }
}

// Translated from 007038a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011d96b4`.
pub fn fn_007038a0(e: &mut Engine) -> u32 {
    e.global::<u32>(0x011d_96b4)
}

// Translated from 007038b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetEnemyActor` (Xbox PDB): `HUDMainMenu::SetEnemyActor`.
pub fn interface_set_enemy_actor(e: &mut Engine, actor: u32) {
    e.call(0x0077_3190, &args![actor]);
}

// Translated from 007038d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `007732b0` (in `hudmainmenu.cpp`), no arguments; answers its value (the
/// exe ends on that call and leaves its `EAX`, which the callers compare).
pub fn fn_007038d0(e: &mut Engine) -> Ptr {
    e.call(0x0077_32b0, &[]).ptr()
}

// Translated from 007038e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ToggleSafeZone` (Xbox PDB): `InterfaceManager::ToggleSafeZone`
/// with the argument, when the manager is ready.
pub fn interface_toggle_safe_zone(e: &mut Engine, value: u32) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0071_5ec0, &args![manager, value]);
    }
}

// Translated from 00703920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetFirstChanceTextureRelease` (Xbox PDB): when the manager
/// is ready, stores 1 in its byte at `+0xec` ([`fn_00703960`]) and runs
/// `Tile::UpdateAll(0)` (`cdecl`).
pub fn interface_set_first_chance_texture_release(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        fn_00703960(e, manager, 1);
        e.call(0x00a0_4200, &args![0u32]);
    }
}

// Translated from 00703960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `+0xec` of the manager.
pub fn fn_00703960(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0xec, value);
}

// Translated from 00703980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::ForceTextureRelease` (Xbox PDB) on the manager when
/// it is ready.
pub fn fn_00703980(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(0x0071_60b0, &args![manager]);
    }
}

// Translated from 007039b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetControlPushTextFor` (Xbox PDB), `cdecl`: writes the push
/// text of control `control` (`0..=0x1c`; any other value does nothing)
/// into the character buffer `buffer`.
///
/// The control's setting is chosen from [`fn_00703be0`] on the controls
/// object (`00877720` on the global `011dea0c`), by mode: when `004b71d0`
/// answers true mode 3 and the table at `011d5268`; otherwise mode 1 with
/// the table at `011d5240` (index below 9), else mode 0 with `011d52f0`
/// (index below `0xee`), else mode 2 with `011d51b0` (index below 8). Its
/// text (`00403df0`) is copied to the buffer (`004046f0`). When `with_label`
/// is set the buffer then becomes `"<label> <text>"` (`sprintf`) where the
/// label is the setting at `011d4a08` if the setting's text (`+8`) contains
/// "trigger" (`BSstristr`), else the one at `011d3fc4`, and the text is the
/// buffer itself, or the setting at `011d37b0` when the buffer is empty.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn interface_get_control_push_text_for(
    e: &mut Engine,
    control: i32,
    buffer: u32,
    with_label: u8,
) {
    if !(0..=0x1c).contains(&control) {
        return;
    }
    let mut setting = 0u32;
    e.mem.set_u8(buffer, 0);
    if e.call(0x004b_71d0, &[]).bool() {
        let index = control_byte_for(e, control, 3);
        setting = e.mem.u32(0x011d_5268 + 4 * index as u32);
    } else {
        let index = control_byte_for(e, control, 1);
        if index < 9 {
            setting = e.mem.u32(0x011d_5240 + 4 * index as u32);
        } else {
            let index = control_byte_for(e, control, 0);
            if index < 0xee {
                setting = e.mem.u32(0x011d_52f0 + 4 * index as u32);
            } else {
                let index = control_byte_for(e, control, 2);
                if index < 8 {
                    setting = e.mem.u32(0x011d_51b0 + 4 * index as u32);
                }
            }
        }
    }
    if setting != 0 && e.call(SETTING_TEXT, &args![setting]).u32() != 0 {
        let text = e.call(SETTING_TEXT, &args![setting]).u32();
        e.call(STRING_COPY, &args![buffer, text]);
    }
    if with_label == 0 {
        return;
    }
    e.with_stack(8, |e, text| {
        e.call(STRING_CTOR, &args![text]);
        if e.mem.u8(buffer) != 0 {
            e.call(STRING_ASSIGN, &args![text, buffer]);
        } else {
            let fallback = e.call(SETTING_TEXT, &args![0x011d_37b0u32]).u32();
            e.call(STRING_ASSIGN, &args![text, fallback]);
        }
        let is_trigger = setting != 0 && e.mem.u32(setting + 8) != 0 && {
            let name = e.mem.u32(setting + 8);
            e.call(STRING_STRISTR, &args![name, 0x0106_ebd8u32]).u32() != 0
        };
        let characters = e.call(FIRST_WORD, &args![text]).u32();
        let label_setting = if is_trigger {
            0x011d_4a08u32
        } else {
            0x011d_3fc4u32
        };
        let label = e.call(SETTING_TEXT, &args![label_setting]).u32();
        e.call(SPRINTF, &args![buffer, 0x0101_2058u32, label, characters]);
        e.call(STRING_DTOR, &args![text]);
    });
}

/// `00703be0` on the controls object (`00877720` on the global).
fn control_byte_for(e: &mut Engine, control: i32, mode: u32) -> u8 {
    let global = e.global::<u32>(CONTROLS_OBJECT_GLOBAL);
    let object = e.call(CONTROLS_OBJECT, &args![global]).u32();
    fn_00703be0(e, object, control as u32, mode)
}

// Translated from 00703be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x1b94 + mode * 0x1c + control` of the object.
pub fn fn_00703be0(e: &mut Engine, this: u32, control: u32, mode: u32) -> u8 {
    let offset = mode.wrapping_mul(0x1c).wrapping_add(control);
    e.mem.u8(this.wrapping_add(0x1b94).wrapping_add(offset))
}

/// The console exists and is visible (`MenuConsole::Instance(0)`, then
/// `Instance(1)` and its visibility test).
fn console_visible(e: &mut Engine) -> bool {
    if e.call(CONSOLE_INSTANCE, &args![0u32]).u32() == 0 {
        return false;
    }
    let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
    e.call(CONSOLE_IS_VISIBLE, &args![console]).bool()
}

/// Whether the console takes text: it is visible, or `005b6f70` says text
/// is accepted anyway.
fn console_takes_text(e: &mut Engine) -> bool {
    console_visible(e) || e.call(CONSOLE_ACCEPTS_TEXT, &[]).bool()
}

// Translated from 00703c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prints a formatted line on the console (`cdecl`, varargs: the format
/// string pointer, then the words the format uses): when the manager is
/// ready and the console takes text, `0071d0a0(console, format, va_list)`.
pub fn fn_00703c00(e: &mut Engine, format: u32, va: &[u32]) {
    if ready_manager(e).is_none() {
        return;
    }
    if console_takes_text(e) {
        with_va_list(e, va, |e, list| {
            let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
            e.call(CONSOLE_PRINT, &args![console, format, list]);
        });
    }
}

/// The registered form of `00703c00`: the format and the variable words.
fn fn_00703c00_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(!words.is_empty(), "00703c00 needs the format argument word");
    fn_00703c00(e, words[0], &words[1..]);
    Ret::default()
}

// Translated from 00703c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::PrintLine` (Xbox PDB), `cdecl`, varargs: the first argument
/// is a `BSString` passed by value (its two words, `text_pointer` and
/// `text_length`; the function destroys it) and the variable words follow.
/// Like [`fn_00703c00`] it prints `0071d0a0(console, characters, va_list)`
/// with the string's characters as format.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn interface_print_line(e: &mut Engine, text_pointer: u32, text_length: u32, va: &[u32]) {
    e.with_stack(8, |e, text| {
        e.mem.set_u32(text.addr(), text_pointer);
        e.mem.set_u32(text.addr() + 4, text_length);
        if ready_manager(e).is_some() && console_takes_text(e) {
            with_va_list(e, va, |e, list| {
                let characters = e.call(FIRST_WORD, &args![text]).u32();
                let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
                e.call(CONSOLE_PRINT, &args![console, characters, list]);
            });
        }
        e.call(STRING_DTOR, &args![text]);
    });
}

/// The registered form of `Interface::PrintLine`.
fn interface_print_line_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "PrintLine needs the two words of its string"
    );
    interface_print_line(e, words[0], words[1], &words[2..]);
    Ret::default()
}

// Translated from 00703d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsConsoleVisible` (Xbox PDB): the console exists and is
/// visible, when the manager is ready.
pub fn interface_is_console_visible(e: &mut Engine) -> bool {
    ready_manager(e).is_some() && console_visible(e)
}

// Translated from 00703da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CloseConsole` (Xbox PDB): when the manager is ready and the
/// console exists and is visible, toggles it and pops entry 3 from the
/// enter stack (`InterfaceManager::PopFromEnterStack(3, 0)`).
pub fn interface_close_console(e: &mut Engine) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    if console_visible(e) {
        let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
        e.call(CONSOLE_TOGGLE_VISIBLE, &args![console]);
        e.call(POP_FROM_ENTER_STACK, &args![manager, 3u32, 0u32]);
    }
}

// Translated from 00703e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::OpenConsole` (Xbox PDB): when the manager is ready and the
/// console exists but is not visible, toggles it and pushes entry 3 on the
/// enter stack (`InterfaceManager::AddToEnterStack(3)`).
pub fn interface_open_console(e: &mut Engine) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    if e.call(CONSOLE_INSTANCE, &args![0u32]).u32() == 0 {
        return;
    }
    let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
    if e.call(CONSOLE_IS_VISIBLE, &args![console]).bool() {
        return;
    }
    let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
    e.call(CONSOLE_TOGGLE_VISIBLE, &args![console]);
    e.call(ADD_TO_ENTER_STACK, &args![manager, 3u32]);
}

// Translated from 00703e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates a message (`cdecl`, varargs): when the manager is ready,
/// `007a8e60(arg0, arg1, arg2, 0, arg3, arg4, arg5, arg8, &va_list, 1, -1,
/// -1, 0.0, arg6, arg7)` and its byte answer; false otherwise. `arg6` and
/// `arg7` are floats.
#[allow(clippy::too_many_arguments)]
pub fn fn_00703e80(
    e: &mut Engine,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: f32,
    arg7: f32,
    arg8: u32,
    va: &[u32],
) -> bool {
    if ready_manager(e).is_none() {
        return false;
    }
    with_va_list_cell(e, va, |e, list| {
        e.call(
            MESSAGE_CREATE,
            &args![
                arg0, arg1, arg2, 0u32, arg3, arg4, arg5, arg8, list, 1u32, -1i32, -1i32, 0.0f32,
                arg6, arg7
            ],
        )
        .bool()
    })
}

/// The registered form of `00703e80`: nine fixed words and the variable ones.
fn fn_00703e80_entry(e: &mut Engine, w: &[u32]) -> Ret {
    assert!(
        w.len() >= 9,
        "00703e80 needs 9 argument words, got {}",
        w.len()
    );
    let ok = fn_00703e80(
        e,
        w[0],
        w[1],
        w[2],
        w[3],
        w[4],
        w[5],
        f32::from_bits(w[6]),
        f32::from_bits(w[7]),
        w[8],
        &w[9..],
    );
    Ret {
        eax: ok as u32,
        ..Ret::default()
    }
}

// Translated from 00703f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same as [`fn_00703e80`] with ten fixed words: `007a8e60(arg0, arg1,
/// arg2, arg3, arg4, arg5, arg6, arg9, &va_list, 1, -1, -1, 0.0, arg7,
/// arg8)`; `arg7` and `arg8` are floats.
#[allow(clippy::too_many_arguments)]
pub fn fn_00703f10(
    e: &mut Engine,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: f32,
    arg8: f32,
    arg9: u32,
    va: &[u32],
) -> bool {
    if ready_manager(e).is_none() {
        return false;
    }
    with_va_list_cell(e, va, |e, list| {
        e.call(
            MESSAGE_CREATE,
            &args![
                arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg9, list, 1u32, -1i32, -1i32, 0.0f32,
                arg7, arg8
            ],
        )
        .bool()
    })
}

/// The registered form of `00703f10`: ten fixed words and the variable ones.
fn fn_00703f10_entry(e: &mut Engine, w: &[u32]) -> Ret {
    assert!(
        w.len() >= 10,
        "00703f10 needs 10 argument words, got {}",
        w.len()
    );
    let ok = fn_00703f10(
        e,
        w[0],
        w[1],
        w[2],
        w[3],
        w[4],
        w[5],
        w[6],
        f32::from_bits(w[7]),
        f32::from_bits(w[8]),
        w[9],
        &w[10..],
    );
    Ret {
        eax: ok as u32,
        ..Ret::default()
    }
}

// Translated from 00703fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetMessageMenuResult` (Xbox PDB): the byte at `+0xe4` of the
/// manager, which it resets to `0xff`. (It does not check that the manager
/// is ready.)
pub fn interface_get_message_menu_result(e: &mut Engine) -> u8 {
    let manager = e.call(GET_MANAGER, &[]).u32();
    let result = e.mem.u8(manager + 0xe4);
    let manager = e.call(GET_MANAGER, &[]).u32();
    e.mem.set_u8(manager + 0xe4, 0xff);
    result
}

// Translated from 00703fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `007aa5c0` (in `messagemenu.cpp`), no arguments.
pub fn fn_00703fd0(e: &mut Engine) {
    e.call(0x007a_a5c0, &[]);
}

// Translated from 00703fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageMenu::DisableMessageMenu` (Xbox PDB) with the byte.
pub fn fn_00703fe0(e: &mut Engine, value: u8) {
    e.call(0x007a_a6b0, &args![value as u32]);
}

// Translated from 00704000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CloseMessageMenu` (Xbox PDB):
/// `MessageMenu::ClearMessageQueueHead`.
pub fn interface_close_message_menu(e: &mut Engine) {
    e.call(0x007a_a530, &[]);
}

// Translated from 00704010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows a message and waits for its answer (`cdecl`, varargs: four fixed
/// words). It sets the result byte (`0119f349`) to `0xff`, removes the
/// fader 1 (`FaderManager::RemoveFader(1, 1)`) and, when the manager is
/// ready, creates the message (`007a8e60(arg0, 0, 0, 0, 00704130, arg1,
/// arg2, arg3, &va_list, 1, -1, -1, 0.0, 0.0, 0.0)`). If that failed it
/// answers 0; otherwise it runs the interface loop (pumps the window
/// messages, `Controls::Poll`, the per-frame interface calls and a click)
/// until [`fn_00704130`] stores the answer, and returns it.
pub fn fn_00704010(e: &mut Engine, arg0: u32, arg1: u32, arg2: u32, arg3: u32, va: &[u32]) -> u8 {
    e.mem.set_u8(MESSAGE_RESULT, 0xff);
    let mut created = false;
    let fader_manager = e.global::<u32>(FADER_MANAGER_GLOBAL);
    e.call(REMOVE_FADER, &args![fader_manager, 1u32, 1u32]);
    if ready_manager(e).is_some() {
        created = with_va_list_cell(e, va, |e, list| {
            e.call(
                MESSAGE_CREATE,
                &args![
                    arg0,
                    0u32,
                    0u32,
                    0u32,
                    0x0070_4130u32,
                    arg1,
                    arg2,
                    arg3,
                    list,
                    1u32,
                    -1i32,
                    -1i32,
                    0.0f32,
                    0.0f32,
                    0.0f32
                ],
            )
            .bool()
        });
    }
    if !created {
        return 0;
    }
    while e.mem.u8(MESSAGE_RESULT) as i8 == -1 {
        e.with_stack(0x1c, |e, message| {
            e.call(MEMORY_FILL, &args![message, 0u32, 0x1cu32]);
            while e
                .call(PEEK_MESSAGE, &args![message, 0u32, 0u32, 0u32, 1u32])
                .u32()
                != 0
            {
                e.call(TRANSLATE_MESSAGE, &args![message]);
                e.call(DISPATCH_MESSAGE, &args![message]);
            }
        });
        let global = e.global::<u32>(CONTROLS_OBJECT_GLOBAL);
        let object = e.call(CONTROLS_OBJECT, &args![global]).u32();
        e.call(CONTROLS_POLL, &args![object]);
        fn_007027e0(e);
        fn_00702810(e);
        fn_00702840(e);
        interface_click(e, 0, 0);
        fn_00702960(e);
    }
    e.mem.u8(MESSAGE_RESULT)
}

/// The registered form of `00704010`: four fixed words and the variable ones.
fn fn_00704010_entry(e: &mut Engine, w: &[u32]) -> Ret {
    assert!(
        w.len() >= 4,
        "00704010 needs 4 argument words, got {}",
        w.len()
    );
    let answer = fn_00704010(e, w[0], w[1], w[2], w[3], &w[4..]);
    Ret {
        eax: answer as u32,
        ..Ret::default()
    }
}

// Translated from 00704130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The message callback of [`fn_00704010`]: stores
/// [`interface_get_message_menu_result`] in the result byte (`0119f349`).
pub fn fn_00704130(e: &mut Engine) {
    let result = interface_get_message_menu_result(e);
    e.mem.set_u8(MESSAGE_RESULT, result);
}

// Translated from 00704140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMenu::Create` (`00796b90`) when the manager is ready, else 0.
pub fn fn_00704140(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(_) => e.call(0x0079_6b90, &[]).u32(),
        None => 0,
    }
}

// Translated from 00704170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetMapMenuVisible` (Xbox PDB), `cdecl`: shows (`show` set)
/// or hides the map menu. `check_state` makes the "already visible" test
/// look at the menu's state bits `0xb` ([`interface_is_menu_id_visible`]).
///
/// To show it while it is not visible: the stats, repair, item-mod and
/// inventory menus fade out (the first three also reset: `StatsMenu::
/// ToggleHealingMode`, `RepairMenu::ResetMenu`, `ItemModMenu::ResetMenu`),
/// the map menu is created if it does not exist and fades in with its id in
/// the menus root's [`TRAIT_CURRENT_MENU`], then `MapMenu::UpdateMenu`. To
/// hide it while visible: it fades out and its slot `0x14` runs with
/// `(0, 0)`. Showing always ends with the Pipboy refresh (tab 3) and
/// `0079c340(0)` when `00974d90` says so.
pub fn interface_set_map_menu_visible(e: &mut Engine, show: u8, check_state: u8) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let stats = e.call(GET_MENU_BY_CLASS, &args![MENU_STATS]).u32();
    let inventory = e.call(GET_MENU_BY_CLASS, &args![MENU_INVENTORY]).u32();
    let repair = e.call(GET_MENU_BY_CLASS, &args![MENU_REPAIR]).u32();
    let item_mod = e.call(GET_MENU_BY_CLASS, &args![MENU_ITEM_MOD]).u32();
    let mut map = e.call(GET_MENU_BY_CLASS, &args![MENU_MAP]).u32();
    let mask = if check_state != 0 { 0xb } else { 0 };
    let visible = interface_is_menu_id_visible(e, MENU_MAP, mask) != 0;
    if show != 0 && !visible {
        if stats != 0 {
            menu_fade_out(e, stats);
            e.call(STATS_TOGGLE_HEALING_MODE, &[]);
        }
        if repair != 0 {
            menu_fade_out(e, repair);
            e.call(REPAIR_RESET_MENU, &[]);
        }
        if item_mod != 0 {
            menu_fade_out(e, item_mod);
            e.call(ITEM_MOD_RESET_MENU, &[]);
        }
        if inventory != 0 {
            menu_fade_out(e, inventory);
        }
        if map == 0 {
            map = fn_00704140(e);
        }
        if map != 0 {
            set_current_menu(e, manager, MENU_MAP);
            menu_fade_in(e, map);
        }
        e.call(MAP_MENU_UPDATE, &[]);
    } else if show == 0 && visible && map != 0 {
        menu_fade_out(e, map);
        let menu = e.call(TILE_GET_MENU, &args![map]).u32();
        e.vcall(menu, 0x14, &args![0u32, 0u32]);
    }
    if show != 0 {
        refresh_pipboy_tab(e, manager, 3);
        call_if_pipboy_query(e, manager, 0x0079_c340);
    }
}

// Translated from 00704490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateRepairMenu` (Xbox PDB): `RepairMenu::Create` when the
/// manager is ready, else 0.
pub fn interface_create_repair_menu(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(_) => e.call(0x007b_5520, &[]).u32(),
        None => 0,
    }
}

// Translated from 007044c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows or hides the repair menu (`cdecl`: `show`, `check_state` as in
/// [`interface_set_map_menu_visible`], `refresh_pipboy`). Showing creates
/// the menu if needed ([`interface_create_repair_menu`]); when it was not
/// visible the stats (with `StatsMenu::ToggleHealingMode`), map and item-mod
/// menus fade out, the repair menu fades in and the inventory fades out.
/// Its id goes into [`TRAIT_CURRENT_MENU`]; with `refresh_pipboy` the Pipboy
/// refresh runs (tab 2); and `00782470(0)` runs when `00974d90` says so.
/// Hiding fades the menu out when it is visible.
pub fn fn_007044c0(e: &mut Engine, show: u8, check_state: u8, refresh_pipboy: u8) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let stats = e.call(GET_MENU_BY_CLASS, &args![MENU_STATS]).u32();
    let inventory = e.call(GET_MENU_BY_CLASS, &args![MENU_INVENTORY]).u32();
    let mut repair = e.call(GET_MENU_BY_CLASS, &args![MENU_REPAIR]).u32();
    let item_mod = e.call(GET_MENU_BY_CLASS, &args![MENU_ITEM_MOD]).u32();
    let map = e.call(GET_MENU_BY_CLASS, &args![MENU_MAP]).u32();
    let mask = if check_state != 0 { 0xb } else { 0 };
    let visible = interface_is_menu_id_visible(e, MENU_REPAIR, mask) != 0;
    if show != 0 {
        if repair == 0 {
            repair = interface_create_repair_menu(e);
        }
        if !visible {
            if stats != 0 {
                menu_fade_out(e, stats);
                e.call(STATS_TOGGLE_HEALING_MODE, &[]);
            }
            if map != 0 {
                menu_fade_out(e, map);
            }
            if item_mod != 0 {
                menu_fade_out(e, item_mod);
            }
            if repair != 0 {
                menu_fade_in(e, repair);
            }
            if inventory != 0 {
                menu_fade_out(e, inventory);
            }
        }
        if repair != 0 {
            set_current_menu(e, manager, MENU_REPAIR);
        }
        if refresh_pipboy != 0 {
            refresh_pipboy_tab(e, manager, 2);
        }
        call_if_pipboy_query(e, manager, 0x0078_2470);
    } else if visible && repair != 0 {
        menu_fade_out(e, repair);
    }
}

// Translated from 00704690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::Create` (`cdecl`) with the argument, when the
/// manager is ready.
pub fn fn_00704690(e: &mut Engine, value: u32) {
    if ready_manager(e).is_some() {
        e.call(0x007b_7570, &args![value]);
    }
}

// Translated from 007046c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateItemModMenu` (Xbox PDB): `ItemModMenu::Create` when
/// the manager is ready, else 0.
pub fn interface_create_item_mod_menu(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(_) => e.call(0x0078_3440, &[]).u32(),
        None => 0,
    }
}

// Translated from 007046f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows or hides the item-mod menu: the same as [`fn_007044c0`] with the
/// item-mod menu (`0x425`, created by [`interface_create_item_mod_menu`]);
/// when it was not visible the stats (with `StatsMenu::ToggleHealingMode`),
/// map and repair menus fade out, the item-mod menu fades in and the
/// inventory fades out.
pub fn fn_007046f0(e: &mut Engine, show: u8, check_state: u8, refresh_pipboy: u8) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let stats = e.call(GET_MENU_BY_CLASS, &args![MENU_STATS]).u32();
    let inventory = e.call(GET_MENU_BY_CLASS, &args![MENU_INVENTORY]).u32();
    let repair = e.call(GET_MENU_BY_CLASS, &args![MENU_REPAIR]).u32();
    let map = e.call(GET_MENU_BY_CLASS, &args![MENU_MAP]).u32();
    let mut item_mod = e.call(GET_MENU_BY_CLASS, &args![MENU_ITEM_MOD]).u32();
    let mask = if check_state != 0 { 0xb } else { 0 };
    let visible = interface_is_menu_id_visible(e, MENU_ITEM_MOD, mask) != 0;
    if show != 0 {
        if item_mod == 0 {
            item_mod = interface_create_item_mod_menu(e);
        }
        if !visible {
            if stats != 0 {
                menu_fade_out(e, stats);
                e.call(STATS_TOGGLE_HEALING_MODE, &[]);
            }
            if map != 0 {
                menu_fade_out(e, map);
            }
            if repair != 0 {
                menu_fade_out(e, repair);
            }
            if item_mod != 0 {
                menu_fade_in(e, item_mod);
            }
            if inventory != 0 {
                menu_fade_out(e, inventory);
            }
        }
        if item_mod != 0 {
            set_current_menu(e, manager, MENU_ITEM_MOD);
        }
        if refresh_pipboy != 0 {
            refresh_pipboy_tab(e, manager, 2);
        }
        call_if_pipboy_query(e, manager, 0x0078_2470);
    } else if visible && item_mod != 0 {
        menu_fade_out(e, item_mod);
    }
}

// Translated from 007048c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::Create` when the manager is ready, else 0.
pub fn fn_007048c0(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(_) => e.call(0x0077_fc10, &[]).u32(),
        None => 0,
    }
}

// Translated from 007048f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetInventoryMenuVisible` (Xbox PDB), `cdecl`: shows or hides
/// the inventory menu, as [`fn_007044c0`] does the repair menu. Showing
/// creates it if needed ([`fn_007048c0`]); when it was not visible the
/// stats (with `StatsMenu::ToggleHealingMode`), map, repair (with
/// `RepairMenu::ResetMenu`) and item-mod (with `ItemModMenu::ResetMenu`)
/// menus fade out and the inventory fades in; then
/// `InventoryMenu::UpdateInventoryMenu` runs and the id goes into
/// [`TRAIT_CURRENT_MENU`]. Hiding a visible one fades it out and runs
/// `InventoryMenu::SavePageState`.
pub fn interface_set_inventory_menu_visible(
    e: &mut Engine,
    show: u8,
    check_state: u8,
    refresh_pipboy: u8,
) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let stats = e.call(GET_MENU_BY_CLASS, &args![MENU_STATS]).u32();
    let mut inventory = e.call(GET_MENU_BY_CLASS, &args![MENU_INVENTORY]).u32();
    let repair = e.call(GET_MENU_BY_CLASS, &args![MENU_REPAIR]).u32();
    let item_mod = e.call(GET_MENU_BY_CLASS, &args![MENU_ITEM_MOD]).u32();
    let map = e.call(GET_MENU_BY_CLASS, &args![MENU_MAP]).u32();
    let mask = if check_state != 0 { 0xb } else { 0 };
    let visible = interface_is_menu_id_visible(e, MENU_INVENTORY, mask) != 0;
    if show != 0 {
        if inventory == 0 {
            inventory = fn_007048c0(e);
        }
        if !visible {
            if stats != 0 {
                menu_fade_out(e, stats);
                e.call(STATS_TOGGLE_HEALING_MODE, &[]);
            }
            if map != 0 {
                menu_fade_out(e, map);
            }
            if repair != 0 {
                menu_fade_out(e, repair);
                e.call(REPAIR_RESET_MENU, &[]);
            }
            if item_mod != 0 {
                menu_fade_out(e, item_mod);
                e.call(ITEM_MOD_RESET_MENU, &[]);
            }
            if inventory != 0 {
                menu_fade_in(e, inventory);
            }
        }
        if inventory != 0 {
            e.call(INVENTORY_UPDATE_MENU, &[]);
            set_current_menu(e, manager, MENU_INVENTORY);
        }
        if refresh_pipboy != 0 {
            refresh_pipboy_tab(e, manager, 2);
        }
        call_if_pipboy_query(e, manager, 0x0078_2470);
    } else if visible && inventory != 0 {
        menu_fade_out(e, inventory);
        e.call(INVENTORY_SAVE_PAGE_STATE, &[]);
    }
}

// Translated from 00704ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetInventoryMenuVisible` (Xbox PDB): the flag byte of the
/// inventory menu id ([`fn_007027b0`]).
pub fn interface_get_inventory_menu_visible(e: &mut Engine) -> u8 {
    fn_007027b0(e, MENU_INVENTORY)
}

// Translated from 00704af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready, refreshes whichever menu is showing: the
/// inventory (`InventoryMenu::UpdateInventoryMenu`) if it is visible, else
/// the stats menu (`StatsMenu::UpdateHealingOptions(menu, 4)`) when
/// `00704df0` says so, else `00730690(1)` when menu `0x41d` is visible,
/// else `00728a30(1)` when `0x435` is, else `00704bc0` when `0x3f0` is.
pub fn fn_00704af0(e: &mut Engine) {
    if ready_manager(e).is_none() {
        return;
    }
    if interface_get_inventory_menu_visible(e) != 0 {
        e.call(INVENTORY_UPDATE_MENU, &[]);
    } else if e.call(0x0070_4df0, &[]).bool() {
        let stats = e.call(GET_MENU_BY_CLASS, &args![MENU_STATS]).u32();
        let menu = e.call(TILE_GET_MENU, &args![stats]).u32();
        e.call(STATS_UPDATE_HEALING_OPTIONS, &args![menu, 4u32]);
    } else if interface_is_menu_id_visible(e, 0x41d, 0) != 0 {
        e.call(0x0073_0690, &args![1u32]);
    } else if interface_is_menu_id_visible(e, 0x435, 0) != 0 {
        e.call(0x0072_8a30, &args![1u32]);
    } else if interface_is_menu_id_visible(e, 0x3f0, 0) != 0 {
        e.call(0x0070_4bc0, &[]);
    }
}

/// The type descriptors the menu casts pass to `__RTDynamicCast`
/// (`00ec43fb`, `cdecl`): the source type is always `Menu`, the targets
/// `StatsMenu`, `DialogMenu`, `SleepWaitMenu` and `VATSMenu`.
const MENU_TYPE: u32 = 0x0119_f34c;
const STATS_MENU_TYPE: u32 = 0x0119_f37c;
const DIALOG_MENU_TYPE: u32 = 0x0119_f394;
const SLEEP_WAIT_MENU_TYPE: u32 = 0x0119_f3cc;
const VATS_MENU_TYPE: u32 = 0x0119_f3e8;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Menu ids of the menus this part looks up.
const MENU_DIALOG: i32 = 0x3f1;
const MENU_SLEEP_WAIT: i32 = 0x3f4;
const MENU_VATS: i32 = 0x420;
/// Menu ids `00705000` and `00705020` test for existence.
const MENU_435: i32 = 0x435;
const MENU_41D: i32 = 0x41d;
/// `StatsMenu::Create` (Xbox PDB), `StatsMenu::UpdateAll` (on the menu),
/// `StatsMenu::SetKnobPosition` (`cdecl`, one word),
/// `StatsMenu::UpdateAllActorValues` and `StatsMenu::UpdateActorValue` (the
/// last two on the menu, the second with the value).
const STATS_MENU_CREATE: u32 = 0x007d_a2c0;
const STATS_UPDATE_ALL: u32 = 0x007d_cc80;
const STATS_SET_KNOB_POSITION: u32 = 0x007d_fc90;
const STATS_UPDATE_ALL_ACTOR_VALUES: u32 = 0x007d_cfc0;
const STATS_UPDATE_ACTOR_VALUE: u32 = 0x007d_d090;
/// The actor value that updates the whole stats screen (`0x4d`).
const ACTOR_VALUE_ALL: u32 = 0x4d;
/// The global holding the container menu object (`011d93f8`) and
/// `ContainerMenu::UpdateLists` (Xbox PDB) on it.
const CONTAINER_MENU_GLOBAL: u32 = 0x011d_93f8;
const CONTAINER_UPDATE_LISTS: u32 = 0x0075_c280;
/// The word at `+0x80` of the object at [`CONTAINER_MENU_GLOBAL`].
const CONTAINER_MENU_FIELD: u32 = 0x80;
/// `ContainerMenu::Create` (`cdecl`) and `ContainerMenu::Close` (Xbox PDB).
const CONTAINER_MENU_CREATE: u32 = 0x0075_b310;
const CONTAINER_MENU_CLOSE: u32 = 0x0075_b750;
/// `TESObjectREFR::GetOwner` and `TESObjectREFR::IsAnOwner` (Xbox PDB) on a
/// reference; the latter takes the word at `011dea3c` and a flag.
const REFR_GET_OWNER: u32 = 0x0056_7790;
const REFR_IS_AN_OWNER: u32 = 0x0057_85e0;
const OWNER_GLOBAL: u32 = 0x011d_ea3c;
/// Slot `0x100` of a reference (a getter answering a byte).
const REFR_SLOT_100: u32 = 0x100;
/// `BarterMenu::Create` and `RecipeMenu::Create` (Xbox PDB), `cdecl`, two
/// words each.
const BARTER_MENU_CREATE: u32 = 0x0072_d250;
const RECIPE_MENU_CREATE: u32 = 0x0072_6ff0;
/// `BSTimer::SetGlobalTimeMultiplier` (Xbox PDB) on the timer at
/// `011f6394`: a float and a flag.
const SET_GLOBAL_TIME_MULTIPLIER: u32 = 0x00aa_4db0;
const GLOBAL_TIMER: u32 = 0x011f_6394;
/// `DialogMenu::Create` (`cdecl`, three words), `DialogMenu::SetPause` (on
/// the menu, one word), `DialogMenu::ServiceStart` and the getter
/// `004fb070` on a dialog menu.
const DIALOG_MENU_CREATE: u32 = 0x0076_1a20;
const DIALOG_SET_PAUSE: u32 = 0x0076_4200;
const DIALOG_SERVICE_START: u32 = 0x0076_3ff0;
const DIALOG_MENU_GETTER: u32 = 0x004f_b070;
/// The byte set while a dialog is open (`011d9514`).
const IN_DIALOG_FLAG: u32 = 0x011d_9514;
/// The HUD main menu object (the word at `011d96c0`) and, on it,
/// `HUDMainMenu::ShowSubtitle` (Xbox PDB), `00775380` and `00775920`.
const HUD_MAIN_MENU: u32 = 0x011d_96c0;
const HUD_SHOW_SUBTITLE: u32 = 0x0077_4fd0;
const HUD_SHOW_SUBTITLE_TIMED: u32 = 0x0077_5380;
const HUD_HIDE_SUBTITLE: u32 = 0x0077_5920;
/// `HUDMainMenu::SetRegionText` (Xbox PDB, `cdecl`): a pointer to a
/// 0x10c-byte request (a word, a 260-byte text, a flag byte at `+0x108`).
const HUD_SET_REGION_TEXT: u32 = 0x0077_28f0;
/// `strcpy_s(buffer, size, text)` (`00406d30`, `cdecl`).
const STRING_COPY_BOUNDED: u32 = 0x0040_6d30;
/// The copy (`00418900`: `this`, source pointer; a word, a byte and a word)
/// and the empty destructor (`00483710`) of the 12-byte value `ShowSubtitle`
/// takes by value.
const SUBTITLE_VALUE_COPY: u32 = 0x0041_8900;
const SUBTITLE_VALUE_DTOR: u32 = 0x0048_3710;
/// `StartMenu::Create(first, second)` (Xbox PDB, `cdecl`).
const START_MENU_CREATE: u32 = 0x007c_b7d0;
/// `LockPickMenu::Create` (Xbox PDB, `cdecl`, three words).
const LOCK_PICK_MENU_CREATE: u32 = 0x0078_db00;
/// `SleepWaitMenu::Create` (`cdecl`, one word), `SleepWaitMenu::
/// UpdateSleeping` (on the menu) and `SleepWaitMenu::Close` (Xbox PDB).
const SLEEP_MENU_CREATE: u32 = 0x007b_fc30;
const SLEEP_UPDATE_SLEEPING: u32 = 0x007c_0580;
const SLEEP_MENU_CLOSE: u32 = 0x007c_01b0;
/// `VATSMenu::Create` and `VATSMenu::Close` (Xbox PDB).
const VATS_MENU_CREATE: u32 = 0x007e_9200;
const VATS_MENU_CLOSE: u32 = 0x007e_bd50;
/// `00709be0` and `00709bc0`: true when the manager's `+0x4bc` word is 0
/// and 2 (`00709c00`, [`MANAGER_MODE_IS_THREE`], tests for 3).
const MANAGER_MODE_IS_ZERO: u32 = 0x0070_9be0;
const MANAGER_MODE_IS_TWO: u32 = 0x0070_9bc0;
/// `RaceSexMenu::Create` (Xbox PDB, `cdecl`) and the words the deferred
/// path writes: the pending menu kind (`011d8950`) and its argument
/// (`011d8960`).
const RACE_SEX_MENU_CREATE: u32 = 0x007a_c730;
const PENDING_MENU_KIND: u32 = 0x011d_8950;
const PENDING_MENU_ARGUMENT: u32 = 0x011d_8960;
/// The pending menu kind of the race/sex menu.
const PENDING_RACE_SEX: u32 = 5;
/// `0043d4d0` on `011c3ea4`: the address of a counter word.
const COUNTER_ACCESSOR: u32 = 0x0043_d4d0;
const COUNTER_OBJECT: u32 = 0x011c_3ea4;
/// Methods of the manager the thin wrappers forward to.
const MANAGER_LAST_MINUTE_UPDATE: u32 = 0x0071_3c70;
const MANAGER_GETTER_A: u32 = 0x0060_2170;
const MANAGER_GETTER_B: u32 = 0x0046_0fb0;
const MANAGER_CALL_A: u32 = 0x0071_33f0;
const MANAGER_CALL_B: u32 = 0x0071_7e30;

/// The menu of class `menu_id` cast to `target_type`: `GetMenuByClass`,
/// `Tile::GetMenu` and `__RTDynamicCast`. 0 when there is no such menu or
/// it is another class.
fn menu_cast(e: &mut Engine, menu_id: i32, target_type: u32) -> u32 {
    let tile = e.call(GET_MENU_BY_CLASS, &args![menu_id]).u32();
    if tile == 0 {
        return 0;
    }
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![menu, 0u32, MENU_TYPE, target_type, 0u32],
    )
    .u32()
}

// Translated from 00704bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the container menu object (`011d93f8`) exists:
/// `ContainerMenu::UpdateLists(0)` on it.
pub fn fn_00704bc0(e: &mut Engine) {
    let menu = e.global::<u32>(CONTAINER_MENU_GLOBAL);
    if menu != 0 {
        e.call(CONTAINER_UPDATE_LISTS, &args![menu, 0u32]);
    }
}

// Translated from 00704be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::Create` when the manager is ready, else 0.
pub fn fn_00704be0(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(_) => e.call(STATS_MENU_CREATE, &[]).u32(),
        None => 0,
    }
}

// Translated from 00704c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetStatsMenuVisible` (Xbox PDB), `cdecl`: shows (`show`
/// set) or hides the stats menu; `check_state` is as in
/// [`interface_set_map_menu_visible`].
///
/// To show it while it is not visible: the inventory, repair (with
/// `RepairMenu::ResetMenu`), item-mod (with `ItemModMenu::ResetMenu`) and
/// map menus fade out, the stats menu is created if it does not exist, its
/// id goes into [`TRAIT_CURRENT_MENU`], `StatsMenu::UpdateAll` runs and it
/// fades in. Hiding a visible one fades it out. Showing always ends with
/// the Pipboy refresh (tab 1) and `StatsMenu::SetKnobPosition(0)` when
/// `00974d90` says so.
pub fn interface_set_stats_menu_visible(e: &mut Engine, show: u8, check_state: u8) {
    let Some(manager) = ready_manager(e) else {
        return;
    };
    let mut stats = e.call(GET_MENU_BY_CLASS, &args![MENU_STATS]).u32();
    let inventory = e.call(GET_MENU_BY_CLASS, &args![MENU_INVENTORY]).u32();
    let repair = e.call(GET_MENU_BY_CLASS, &args![MENU_REPAIR]).u32();
    let item_mod = e.call(GET_MENU_BY_CLASS, &args![MENU_ITEM_MOD]).u32();
    let map = e.call(GET_MENU_BY_CLASS, &args![MENU_MAP]).u32();
    let mask = if check_state != 0 { 0xb } else { 0 };
    let visible = interface_is_menu_id_visible(e, MENU_STATS, mask) != 0;
    if show != 0 && !visible {
        if inventory != 0 {
            menu_fade_out(e, inventory);
        }
        if repair != 0 {
            menu_fade_out(e, repair);
            e.call(REPAIR_RESET_MENU, &[]);
        }
        if item_mod != 0 {
            menu_fade_out(e, item_mod);
            e.call(ITEM_MOD_RESET_MENU, &[]);
        }
        if map != 0 {
            menu_fade_out(e, map);
        }
        if stats == 0 {
            stats = fn_00704be0(e);
        }
        if stats != 0 {
            set_current_menu(e, manager, MENU_STATS);
            let menu = e.call(TILE_GET_MENU, &args![stats]).u32();
            e.call(STATS_UPDATE_ALL, &args![menu]);
            menu_fade_in(e, stats);
        }
    } else if show == 0 && visible && stats != 0 {
        menu_fade_out(e, stats);
    }
    if show != 0 {
        refresh_pipboy_tab(e, manager, 1);
        call_if_pipboy_query(e, manager, STATS_SET_KNOB_POSITION);
    }
}

// Translated from 00704df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The flag byte of the stats menu id ([`fn_007027b0`]).
pub fn fn_00704df0(e: &mut Engine) -> u8 {
    fn_007027b0(e, MENU_STATS)
}

// Translated from 00704e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::UpdateActorValue` (Xbox PDB), `cdecl`: when the manager is
/// ready, the stats menu is showing (state bits `0xb`) and really is a
/// `StatsMenu`: `UpdateAllActorValues` for the value `0x4d`, else
/// `UpdateActorValue(value)`.
pub fn interface_update_actor_value(e: &mut Engine, actor_value: u32) {
    if ready_manager(e).is_none() {
        return;
    }
    if interface_is_menu_id_visible(e, MENU_STATS, 0xb) == 0 {
        return;
    }
    let menu = menu_cast(e, MENU_STATS, STATS_MENU_TYPE);
    if menu == 0 {
        return;
    }
    if actor_value == ACTOR_VALUE_ALL {
        e.call(STATS_UPDATE_ALL_ACTOR_VALUES, &args![menu]);
    } else {
        e.call(STATS_UPDATE_ACTOR_VALUE, &args![menu, actor_value]);
    }
}

// Translated from 00704ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateContainerMenu` (Xbox PDB), `cdecl(reference, mode)`:
/// when the manager is ready, `ContainerMenu::Create`. Mode 1 becomes 2
/// when the reference's slot `0x100` answers false, it has an owner
/// (`TESObjectREFR::GetOwner`) and `IsAnOwner(011dea3c, 1)` is false.
pub fn interface_create_container_menu(e: &mut Engine, reference: u32, mode: u32) -> u32 {
    if ready_manager(e).is_none() {
        return 0;
    }
    let mut mode = mode;
    if mode == 1
        && e.vcall(reference, REFR_SLOT_100, &[]).u8() == 0
        && e.call(REFR_GET_OWNER, &args![reference]).u32() != 0
    {
        let player = e.global::<u32>(OWNER_GLOBAL);
        if e.call(REFR_IS_AN_OWNER, &args![reference, player, 1u32])
            .u8()
            == 0
        {
            mode = 2;
        }
    }
    e.call(CONTAINER_MENU_CREATE, &args![reference, mode]).u32()
}

// Translated from 00704f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerMenu::Close` when the manager is ready.
pub fn fn_00704f50(e: &mut Engine) {
    if ready_manager(e).is_some() {
        e.call(CONTAINER_MENU_CLOSE, &[]);
    }
}

// Translated from 00704f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BarterMenu::Create(a, b)` (`cdecl`) when the manager is ready.
pub fn fn_00704f80(e: &mut Engine, first: u32, second: u32) {
    if ready_manager(e).is_some() {
        e.call(BARTER_MENU_CREATE, &args![first, second]);
    }
}

// Translated from 00704fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecipeMenu::Create(a, b)` (`cdecl`) when the manager is ready.
pub fn fn_00704fc0(e: &mut Engine, first: u32, second: u32) {
    if ready_manager(e).is_some() {
        e.call(RECIPE_MENU_CREATE, &args![first, second]);
    }
}

// Translated from 00705000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a menu of class `0x435` exists.
pub fn fn_00705000(e: &mut Engine) -> bool {
    e.call(GET_MENU_BY_CLASS, &args![MENU_435]).u32() != 0
}

// Translated from 00705020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a menu of class `0x41d` exists.
pub fn fn_00705020(e: &mut Engine) -> bool {
    e.call(GET_MENU_BY_CLASS, &args![MENU_41D]).u32() != 0
}

// Translated from 00705040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns what [`fn_00705050`] returns (the code calls it and leaves `eax`).
pub fn fn_00705040(e: &mut Engine) -> u32 {
    fn_00705050(e)
}

// Translated from 00705050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x80` of the object at `011d93f8`, 0 when there is none.
pub fn fn_00705050(e: &mut Engine) -> u32 {
    let menu = e.global::<u32>(CONTAINER_MENU_GLOBAL);
    if menu == 0 {
        0
    } else {
        e.mem.u32(menu + CONTAINER_MENU_FIELD)
    }
}

// Translated from 00705070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateDialogMenu` (Xbox PDB), `cdecl`: first
/// `BSTimer::SetGlobalTimeMultiplier(1.0, 1)` on the timer at `011f6394`,
/// then `DialogMenu::Create` (three words) when the manager is ready, else
/// 0.
pub fn interface_create_dialog_menu(e: &mut Engine, first: u32, second: u32, third: u32) -> u32 {
    e.call(
        SET_GLOBAL_TIME_MULTIPLIER,
        &args![GLOBAL_TIMER, 1.0f32.to_bits(), 1u32],
    );
    if ready_manager(e).is_none() {
        return 0;
    }
    e.call(DIALOG_MENU_CREATE, &args![first, second, third])
        .u32()
}

// Translated from 007050d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::InDialog` (Xbox PDB): the dialog flag ([`fn_00705100`]) when
/// the manager is ready, else 0.
pub fn interface_in_dialog(e: &mut Engine) -> u8 {
    match ready_manager(e) {
        Some(_) => fn_00705100(e),
        None => 0,
    }
}

// Translated from 00705100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dialog flag byte (`011d9514`).
pub fn fn_00705100(e: &mut Engine) -> u8 {
    e.global::<u8>(IN_DIALOG_FLAG)
}

// Translated from 00705110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetDialogPause` (Xbox PDB), `cdecl`: when the manager is
/// ready and the menu of class `0x3f1` is a `DialogMenu`,
/// `DialogMenu::SetPause(pause)` on it.
pub fn interface_set_dialog_pause(e: &mut Engine, pause: u8) {
    if ready_manager(e).is_none() {
        return;
    }
    let menu = menu_cast(e, MENU_DIALOG, DIALOG_MENU_TYPE);
    if menu != 0 {
        e.call(DIALOG_SET_PAUSE, &args![menu, pause]);
    }
}

// Translated from 00705190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::InDialogWith` (Xbox PDB): the result of `004fb070` on the
/// dialog menu when the manager is ready and the menu of class `0x3f1` is a
/// `DialogMenu`, else 0.
pub fn interface_in_dialog_with(e: &mut Engine) -> u32 {
    if ready_manager(e).is_none() {
        return 0;
    }
    let menu = menu_cast(e, MENU_DIALOG, DIALOG_MENU_TYPE);
    if menu == 0 {
        return 0;
    }
    e.call(DIALOG_MENU_GETTER, &args![menu]).u32()
}

// Translated from 00705210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ShowSubtitle` (Xbox PDB), `cdecl`, nine words: a word
/// (`speaker`), a 12-byte value by value (`text_a/b/c`: a word, a byte and
/// a word), three words (`position_*`), a word and a byte. Inside a scope
/// guard (line 0x8ec of `Interface.cpp`): when the manager is ready and the
/// HUD main menu object (`011d96c0`) exists, copies the 12-byte value
/// (`00418900`) and calls `HUDMainMenu::ShowSubtitle` on the object with the
/// speaker, the copy, the three words, the word and the byte. The by-value
/// argument is then destroyed (`00483710`, which does nothing). Always
/// returns 0. The exception frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn interface_show_subtitle(
    e: &mut Engine,
    speaker: u32,
    text_a: u32,
    text_b: u32,
    text_c: u32,
    position_a: u32,
    position_b: u32,
    position_c: u32,
    word: u32,
    flag: u8,
) -> u8 {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0xdu32, 1u32, INTERFACE_CPP_PATH, 0x8ecu32],
        );
        e.with_stack(12, |e, text| {
            e.mem.set_u32(text.addr(), text_a);
            e.mem.set_u32(text.addr() + 4, text_b);
            e.mem.set_u32(text.addr() + 8, text_c);
            let hud = e.global::<u32>(HUD_MAIN_MENU);
            if ready_manager(e).is_some() && hud != 0 {
                e.with_stack(12, |e, copy| {
                    e.call(SUBTITLE_VALUE_COPY, &args![copy, text]);
                    let copy_a = e.mem.u32(copy.addr());
                    let copy_b = e.mem.u32(copy.addr() + 4);
                    let copy_c = e.mem.u32(copy.addr() + 8);
                    e.call(
                        HUD_SHOW_SUBTITLE,
                        &args![
                            hud, speaker, copy_a, copy_b, copy_c, position_a, position_b,
                            position_c, word, flag
                        ],
                    );
                });
            }
            e.call(SCOPE_GUARD_DTOR, &args![guard]);
            e.call(SUBTITLE_VALUE_DTOR, &args![text]);
        });
    });
    0
}

// Translated from 007052f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`, six words (the fifth a float, the sixth a byte): inside a scope
/// guard (line 0x901 of `Interface.cpp`), when the manager is ready and the
/// HUD main menu object (`011d96c0`) exists, calls `00775380` on it with
/// the six words. Always returns 0. The exception frame is not translated.
pub fn fn_007052f0(
    e: &mut Engine,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    value: f32,
    flag: u8,
) -> u8 {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0xdu32, 1u32, INTERFACE_CPP_PATH, 0x901u32],
        );
        let hud = e.global::<u32>(HUD_MAIN_MENU);
        if ready_manager(e).is_some() && hud != 0 {
            e.call(
                HUD_SHOW_SUBTITLE_TIMED,
                &args![hud, first, second, third, fourth, value.to_bits(), flag],
            );
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
    0
}

// Translated from 007053b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready and the HUD main menu object (`011d96c0`)
/// exists: `00775920` on it.
pub fn fn_007053b0(e: &mut Engine) {
    if ready_manager(e).is_none() {
        return;
    }
    let hud = e.global::<u32>(HUD_MAIN_MENU);
    if hud != 0 {
        e.call(HUD_HIDE_SUBTITLE, &args![hud]);
    }
}

// Translated from 007053f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::Create(0, 0)` when the manager is ready.
pub fn fn_007053f0(e: &mut Engine) {
    if ready_manager(e).is_some() {
        e.call(START_MENU_CREATE, &args![0u32, 0u32]);
    }
}

// Translated from 00705420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl(region, text, flag)` where `text` is an 8-byte string by value
/// (two words, the first its character pointer): when the manager is
/// ready, builds a request on the stack (the word, the text copied with
/// `strcpy_s(buffer, 0x104, text)` or an empty string, the flag byte at
/// `+0x108`) and passes it to `HUDMainMenu::SetRegionText`. The string
/// argument is destroyed at the end (`004037d0`). The stack-protector
/// cookie and the exception frame are not translated.
pub fn fn_00705420(e: &mut Engine, region: u32, text_a: u32, text_b: u32, flag: u8) {
    e.with_stack(8, |e, text| {
        e.mem.set_u32(text.addr(), text_a);
        e.mem.set_u32(text.addr() + 4, text_b);
        if ready_manager(e).is_some() {
            e.with_stack(0x10c, |e, request| {
                e.mem.set_u8(request.addr() + 0x108, flag);
                e.mem.set_u32(request.addr(), region);
                let buffer = request.addr() + 4;
                if e.call(FIRST_WORD, &args![text]).u32() == 0 {
                    e.mem.set_u8(buffer, 0);
                } else {
                    let characters = e.call(FIRST_WORD, &args![text]).u32();
                    e.call(STRING_COPY_BOUNDED, &args![buffer, 0x104u32, characters]);
                }
                e.call(HUD_SET_REGION_TEXT, &args![request]);
            });
        }
        e.call(STRING_DTOR, &args![text]);
    });
}

// Translated from 007054f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateSleepMenu` (Xbox PDB), `cdecl(flag)`: when the manager
/// is ready and `00709be0` (its `+0x4bc` word is 0) says so: an existing
/// sleep/wait menu (class `0x3f4`) is destroyed through slot 0 with 1, then
/// `SleepWaitMenu::Create(flag)`.
pub fn interface_create_sleep_menu(e: &mut Engine, flag: u8) {
    if ready_manager(e).is_none() || !e.call(MANAGER_MODE_IS_ZERO, &[]).bool() {
        return;
    }
    let menu = menu_cast(e, MENU_SLEEP_WAIT, SLEEP_WAIT_MENU_TYPE);
    if menu != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    e.call(SLEEP_MENU_CREATE, &args![flag]);
}

// Translated from 007055c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CloseSleepMenu` (Xbox PDB): when the manager is ready and
/// the menu of class `0x3f4` is a `SleepWaitMenu`: `SleepWaitMenu::Close`.
pub fn interface_close_sleep_menu(e: &mut Engine) {
    if ready_manager(e).is_none() {
        return;
    }
    if menu_cast(e, MENU_SLEEP_WAIT, SLEEP_WAIT_MENU_TYPE) != 0 {
        e.call(SLEEP_MENU_CLOSE, &[]);
    }
}

// Translated from 00705640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateVATSMenu` (Xbox PDB): when the manager is ready, an
/// existing VATS menu (class `0x420`) is destroyed through slot 0 with 1,
/// then `VATSMenu::Create`.
pub fn interface_create_vats_menu(e: &mut Engine) {
    if ready_manager(e).is_none() {
        return;
    }
    let menu = menu_cast(e, MENU_VATS, VATS_MENU_TYPE);
    if menu != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    e.call(VATS_MENU_CREATE, &[]);
}

// Translated from 007056f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::UpdateSleeping` (Xbox PDB): looks up the menu of the
/// sleep/wait id ([`fn_00705770`]) first; then, when the manager is ready
/// and that menu exists, `SleepWaitMenu::UpdateSleeping` on it decides the
/// result (1 or 0).
pub fn interface_update_sleeping(e: &mut Engine) -> u8 {
    let id = fn_00705770(e);
    let tile = e.call(GET_MENU_BY_CLASS, &args![id]).u32();
    let menu = if tile == 0 {
        0
    } else {
        e.call(TILE_GET_MENU, &args![tile]).u32()
    };
    if ready_manager(e).is_some() && menu != 0 && e.call(SLEEP_UPDATE_SLEEPING, &args![menu]).bool()
    {
        1
    } else {
        0
    }
}

// Translated from 00705770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu id of the sleep/wait menu, `0x3f4`.
pub fn fn_00705770(_e: &mut Engine) -> u32 {
    MENU_SLEEP_WAIT as u32
}

// Translated from 00705780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CloseVATSMenu` (Xbox PDB): when the manager is ready and the
/// menu of class `0x420` is a `VATSMenu`: `VATSMenu::Close`.
pub fn interface_close_vats_menu(e: &mut Engine) {
    if ready_manager(e).is_none() {
        return;
    }
    if menu_cast(e, MENU_VATS, VATS_MENU_TYPE) != 0 {
        e.call(VATS_MENU_CLOSE, &[]);
    }
}

// Translated from 00705800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StartMenu::Create(1, 1)` when the manager is ready.
pub fn fn_00705800(e: &mut Engine) {
    if ready_manager(e).is_some() {
        e.call(START_MENU_CREATE, &args![1u32, 1u32]);
    }
}

// Translated from 00705830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::Create(a, b, flag)` (`cdecl`) when the manager is ready.
pub fn fn_00705830(e: &mut Engine, first: u32, second: u32, flag: u8) {
    if ready_manager(e).is_some() {
        e.call(LOCK_PICK_MENU_CREATE, &args![first, second, flag]);
    }
}

// Translated from 00705870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateRaceSexMenu` (Xbox PDB), `cdecl(argument)`: when the
/// dialog flag ([`fn_00705100`]) is set, `DialogMenu::ServiceStart` runs
/// first. Then, if the counter that `0043d4d0` finds on `011c3ea4` is above
/// 1 (signed) and no menu is pending (`011d8950` is 0), the creation is
/// deferred: `011d8950` becomes 5 and `011d8960` the argument. Otherwise
/// `RaceSexMenu::Create(argument)` runs now.
pub fn interface_create_race_sex_menu(e: &mut Engine, argument: u32) {
    if fn_00705100(e) != 0 {
        e.call(DIALOG_SERVICE_START, &[]);
    }
    let counter = e.call(COUNTER_ACCESSOR, &args![COUNTER_OBJECT]).u32();
    if e.mem.i32(counter) > 1 && e.global::<u32>(PENDING_MENU_KIND) == 0 {
        e.set_global::<u32>(PENDING_MENU_KIND, PENDING_RACE_SEX);
        e.set_global::<u32>(PENDING_MENU_ARGUMENT, argument);
    } else {
        e.call(RACE_SEX_MENU_CREATE, &args![argument]);
    }
}

// Translated from 007058c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fastcall` getter: the word at `+0xc4` of `this`.
pub fn fn_007058c0(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this + 0xc4)
}

// Translated from 007058e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::LastMinuteUpdate` (Xbox PDB): `00713c70` on the manager when
/// it is ready.
pub fn interface_last_minute_update(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(MANAGER_LAST_MINUTE_UPDATE, &args![manager]);
    }
}

// Translated from 00705910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00602170` on the manager when it is ready, else 0.
pub fn fn_00705910(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(MANAGER_GETTER_A, &args![manager]).u32(),
        None => 0,
    }
}

// Translated from 00705950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00460fb0` on the manager when it is ready, else 0.
pub fn fn_00705950(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(MANAGER_GETTER_B, &args![manager]).u32(),
        None => 0,
    }
}

// Translated from 00705990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetPipboy` (Xbox PDB): the manager's Pipboy object (getter
/// `00704370`) when it is ready, else 0.
pub fn interface_get_pipboy(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(MANAGER_PIPBOY, &args![manager]).u32(),
        None => 0,
    }
}

// Translated from 007059d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `007133f0` on the manager when it is ready.
pub fn fn_007059d0(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(MANAGER_CALL_A, &args![manager]);
    }
}

// Translated from 00705a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsInPipboyMenu` (Xbox PDB): when the manager is ready, true
/// if its `+0x4bc` word is 2 (`00709bc0`) or 3 ([`MANAGER_MODE_IS_THREE`]).
pub fn interface_is_in_pipboy_menu(e: &mut Engine) -> u8 {
    if ready_manager(e).is_none() {
        return 0;
    }
    if e.call(MANAGER_MODE_IS_TWO, &[]).bool() || e.call(MANAGER_MODE_IS_THREE, &[]).bool() {
        1
    } else {
        0
    }
}

// Translated from 00705a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00717e30` on the manager when it is ready.
pub fn fn_00705a60(e: &mut Engine) {
    if let Some(manager) = ready_manager(e) {
        e.call(MANAGER_CALL_B, &args![manager]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00702250, interface_init(u8)),
        entry!(0x007022c0, fn_007022c0()),
        entry!(0x00702330, fn_00702330()),
        entry!(0x00702360, interface_is_in_menu_mode() -> bool),
        entry!(0x007023a0, fn_007023a0(u32) -> bool),
        entry!(0x007023c0, interface_get_top_menu_id() -> u32),
        entry!(0x00702400, fn_00702400(u32) -> u32),
        entry!(0x00702440, fn_00702440() -> u32),
        entry!(0x00702450, interface_is_top_menu_id(i32) -> u8),
        entry!(0x007024e0, interface_is_top_menu_faded_in() -> bool),
        entry!(0x00702640, interface_get_menu_mode_type() -> u32),
        entry!(0x00702680, interface_is_menu_id_visible(i32, u32) -> u8),
        entry!(0x007027b0, fn_007027b0(i32) -> u8),
        entry!(0x007027e0, fn_007027e0()),
        entry!(0x00702810, fn_00702810()),
        entry!(0x00702840, fn_00702840()),
        entry!(0x00702870, fn_00702870()),
        entry!(0x007028b0, interface_click(u32, u32)),
        entry!(0x00702940, fn_00702940(u32, u8)),
        entry!(0x00702960, fn_00702960()),
        entry!(0x00702990, fn_00702990(u32, u8, u8, u8)),
        entry!(0x00702c80, interface_isolate_menu_elements(u8, u8)),
        entry!(0x00702dd0, interface_restore_menu_elements()),
        entry!(0x00702fc0, interface_is_debug_text_visible() -> bool),
        entry!(0x00703000, interface_toggle_debug_text_visible()),
        entry!(0x00703030, fn_00703030(u32)),
        entry!(0x00703060, fn_00703060(u32, u8)),
        entry!(0x00703080, fn_00703080(f32, f32)),
        entry!(0x007030c0, fn_007030c0(u32, u32)),
        entry!(0x00703100, interface_toggle_full_help()),
        entry!(0x00703150, interface_get_full_help() -> u8),
        entry!(0x00703180, interface_get_target_refr() -> u32),
        entry!(0x007031c0, fn_007031c0(u32) -> u32),
        entry!(0x007031e0, fn_007031e0(u32)),
        entry!(0x00703310, interface_set_pick_refr(u32)),
        entry!(0x00703350, fn_00703350() -> u32),
        entry!(0x00703390, interface_get_telekinesis_refr() -> u32),
        entry!(0x007033d0, fn_007033d0(Ptr) -> Ptr),
        entry!(0x00703430, fn_00703430() -> bool),
        entry!(0x00703470, fn_00703470(u32) -> bool),
        entry!(0x00703490, fn_00703490()),
        entry!(0x007034c0, fn_007034c0(u32)),
        entry!(0x00703500, interface_show_menus()),
        entry!(0x00703610, interface_hide_menus()),
        entry!(
            0x007037e0,
            interface_emergency_close_all_menus_and_break_stuff()
        ),
        entry!(0x00703810, fn_00703810(u8)),
        entry!(0x00703860, interface_set_crosshair_target_type(u32)),
        entry!(0x007038a0, fn_007038a0() -> u32),
        entry!(0x007038b0, interface_set_enemy_actor(u32)),
        entry!(0x007038d0, fn_007038d0() -> Ptr),
        entry!(0x007038e0, interface_toggle_safe_zone(u32)),
        entry!(0x00703920, interface_set_first_chance_texture_release()),
        entry!(0x00703960, fn_00703960(u32, u8)),
        entry!(0x00703980, fn_00703980()),
        entry!(
            0x007039b0,
            interface_get_control_push_text_for(i32, u32, u8)
        ),
        entry!(0x00703be0, fn_00703be0(u32, u32, u32) -> u8),
        (0x00703c00, fn_00703c00_entry as AbiFn),
        (0x00703c80, interface_print_line_entry as AbiFn),
        entry!(0x00703d50, interface_is_console_visible() -> bool),
        entry!(0x00703da0, interface_close_console()),
        entry!(0x00703e10, interface_open_console()),
        (0x00703e80, fn_00703e80_entry as AbiFn),
        (0x00703f10, fn_00703f10_entry as AbiFn),
        entry!(0x00703fa0, interface_get_message_menu_result() -> u8),
        entry!(0x00703fd0, fn_00703fd0()),
        entry!(0x00703fe0, fn_00703fe0(u8)),
        entry!(0x00704000, interface_close_message_menu()),
        (0x00704010, fn_00704010_entry as AbiFn),
        entry!(0x00704130, fn_00704130()),
        entry!(0x00704140, fn_00704140() -> u32),
        entry!(0x00704170, interface_set_map_menu_visible(u8, u8)),
        entry!(0x00704490, interface_create_repair_menu() -> u32),
        entry!(0x007044c0, fn_007044c0(u8, u8, u8)),
        entry!(0x00704690, fn_00704690(u32)),
        entry!(0x007046c0, interface_create_item_mod_menu() -> u32),
        entry!(0x007046f0, fn_007046f0(u8, u8, u8)),
        entry!(0x007048c0, fn_007048c0() -> u32),
        entry!(0x007048f0, interface_set_inventory_menu_visible(u8, u8, u8)),
        entry!(0x00704ad0, interface_get_inventory_menu_visible() -> u8),
        entry!(0x00704af0, fn_00704af0()),
        entry!(0x00704bc0, fn_00704bc0()),
        entry!(0x00704be0, fn_00704be0() -> u32),
        entry!(0x00704c10, interface_set_stats_menu_visible(u8, u8)),
        entry!(0x00704df0, fn_00704df0() -> u8),
        entry!(0x00704e10, interface_update_actor_value(u32)),
        entry!(0x00704ed0, interface_create_container_menu(u32, u32) -> u32),
        entry!(0x00704f50, fn_00704f50()),
        entry!(0x00704f80, fn_00704f80(u32, u32)),
        entry!(0x00704fc0, fn_00704fc0(u32, u32)),
        entry!(0x00705000, fn_00705000() -> bool),
        entry!(0x00705020, fn_00705020() -> bool),
        entry!(0x00705040, fn_00705040() -> u32),
        entry!(0x00705050, fn_00705050() -> u32),
        entry!(0x00705070, interface_create_dialog_menu(u32, u32, u32) -> u32),
        entry!(0x007050d0, interface_in_dialog() -> u8),
        entry!(0x00705100, fn_00705100() -> u8),
        entry!(0x00705110, interface_set_dialog_pause(u8)),
        entry!(0x00705190, interface_in_dialog_with() -> u32),
        entry!(
            0x00705210,
            interface_show_subtitle(u32, u32, u32, u32, u32, u32, u32, u32, u8) -> u8
        ),
        entry!(0x007052f0, fn_007052f0(u32, u32, u32, u32, f32, u8) -> u8),
        entry!(0x007053b0, fn_007053b0()),
        entry!(0x007053f0, fn_007053f0()),
        entry!(0x00705420, fn_00705420(u32, u32, u32, u8)),
        entry!(0x007054f0, interface_create_sleep_menu(u8)),
        entry!(0x007055c0, interface_close_sleep_menu()),
        entry!(0x00705640, interface_create_vats_menu()),
        entry!(0x007056f0, interface_update_sleeping() -> u8),
        entry!(0x00705770, fn_00705770() -> u32),
        entry!(0x00705780, interface_close_vats_menu()),
        entry!(0x00705800, fn_00705800()),
        entry!(0x00705830, fn_00705830(u32, u32, u8)),
        entry!(0x00705870, interface_create_race_sex_menu(u32)),
        entry!(0x007058c0, fn_007058c0(u32) -> u32),
        entry!(0x007058e0, interface_last_minute_update()),
        entry!(0x00705910, fn_00705910() -> u32),
        entry!(0x00705950, fn_00705950() -> u32),
        entry!(0x00705990, interface_get_pipboy() -> u32),
        entry!(0x007059d0, fn_007059d0()),
        entry!(0x00705a00, interface_is_in_pipboy_menu() -> u8),
        entry!(0x00705a60, fn_00705a60()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Registers a double that returns a fixed value in `eax`.
    macro_rules! stub {
        ($e:expr, $addr:expr, $value:expr) => {
            $e.register($addr, |_, _| Ret {
                eax: $value,
                ..Ret::default()
            })
        };
    }

    /// An engine whose manager exists and is ready; the pages holding the
    /// globals the code reads are mapped.
    fn ui_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0103_a000,
            0x0106_e000,
            0x011d_8000,
            0x011d_9000,
            0x011d_e000,
            0x011f_3000,
            0x011f_4000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
        let manager = e.mem.alloc(0x500);
        e.mem.set_u32(0x011d_8a80, manager);
        e.mem.set_u8(manager, 1);
        e.register(MANAGER_READY, |e, a| Ret {
            eax: e.mem.u8(a[0]) as u32,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, manager)
    }

    /// The same, with no manager object.
    fn engine_without_manager() -> Engine {
        let (mut e, _) = ui_engine();
        e.mem.set_u32(0x011d_8a80, 0);
        e
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Checks a wrapper that calls `callee(manager)` when the manager is
    /// ready and nothing otherwise.
    fn check_forwards_to_manager(wrapper: u32, callee: u32) {
        let (mut e, manager) = ui_engine();
        e.register(callee, |_, _| Ret::default());
        e.call(wrapper, &[]);
        assert_eq!(logged(&e, callee), vec![vec![manager]]);

        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(wrapper, &[]);
        assert!(logged(&e, callee).is_empty());

        let mut e = engine_without_manager();
        e.register(callee, |_, _| Ret::default());
        e.call(wrapper, &[]);
        assert!(logged(&e, callee).is_empty());
    }

    /// A list node `[next, 0, item]`, where the item is a tile pointer.
    fn list_node(e: &mut Engine, next: u32, tile: u32) -> u32 {
        let node = e.mem.alloc(12);
        e.mem.set_u32(node, next);
        e.mem.set_u32(node + 8, tile);
        node
    }

    /// Doubles for the list helpers: the first word of an object and the
    /// iterator step (returns the address of the node's item, moves the
    /// cursor to the next node).
    fn list_doubles(e: &mut Engine) {
        e.register(FIRST_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(LIST_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            Ret {
                eax: node + 8,
                ..Ret::default()
            }
        });
    }

    /// A tile trait double: values by `(tile, trait)`, 0.0 otherwise.
    fn tile_values(e: &mut Engine, values: &[((u32, u32), f32)]) {
        let table: HashMap<(u32, u32), f32> = values.iter().copied().collect();
        e.register_double(TILE_GET_VALUE, move |_, a| Ret {
            st0: table.get(&(a[0], a[1])).copied().unwrap_or(0.0) as f64,
            ..Ret::default()
        });
    }

    #[test]
    fn init_runs_the_manager_initializer_inside_the_guard() {
        let (mut e, _) = ui_engine();
        e.register(SCOPE_GUARD_CTOR, |_, _| Ret::default());
        e.register(SCOPE_GUARD_DTOR, |_, _| Ret::default());
        e.register(MANAGER_INITIALIZE, |_, _| Ret::default());
        e.call(0x0070_2250, &args![5u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x0070_2250,
                SCOPE_GUARD_CTOR,
                MANAGER_INITIALIZE,
                SCOPE_GUARD_DTOR
            ]
        );
        let guard = log[1].1[0];
        assert_eq!(&log[1].1[1..], &[0xd, 1, INTERFACE_CPP_PATH, 0xb0]);
        assert_eq!(log[2].1, vec![1, 5]);
        assert_eq!(log[3].1, vec![guard]);
    }

    #[test]
    fn fn_007022c0_initializes_with_two_and_one() {
        let (mut e, _) = ui_engine();
        e.register(SCOPE_GUARD_CTOR, |_, _| Ret::default());
        e.register(SCOPE_GUARD_DTOR, |_, _| Ret::default());
        e.register(MANAGER_INITIALIZE, |_, _| Ret::default());
        e.call(0x0070_22c0, &[]);
        assert_eq!(logged(&e, MANAGER_INITIALIZE), vec![vec![2, 1]]);
        assert_eq!(logged(&e, SCOPE_GUARD_CTOR)[0][4], 0xbd);
    }

    #[test]
    fn fn_00702330_calls_on_the_ready_manager() {
        check_forwards_to_manager(0x0070_2330, 0x0070_a0b0);
    }

    #[test]
    fn is_in_menu_mode_is_menu_mode_other_than_one() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(manager + 0x0c, 1);
        assert!(!e.call(0x0070_2360, &[]).bool());
        e.mem.set_u32(manager + 0x0c, 2);
        assert!(e.call(0x0070_2360, &[]).bool());
        e.mem.set_u8(manager, 0);
        assert!(!e.call(0x0070_2360, &[]).bool());
    }

    #[test]
    fn fn_007023a0_compares_the_menu_mode_with_one() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(manager + 0x0c, 1);
        assert!(!e.call(0x0070_23a0, &args![manager]).bool());
        e.mem.set_u32(manager + 0x0c, 0);
        assert!(e.call(0x0070_23a0, &args![manager]).bool());
    }

    #[test]
    fn get_top_menu_id_asks_the_enter_stack() {
        let (mut e, manager) = ui_engine();
        stub!(e, GET_ENTER_STACK_TOP, 7);
        assert_eq!(e.call(0x0070_23c0, &[]).u32(), 7);
        assert_eq!(logged(&e, GET_ENTER_STACK_TOP), vec![vec![manager]]);
        let mut e = engine_without_manager();
        assert_eq!(e.call(0x0070_23c0, &[]).u32(), 0);
    }

    #[test]
    fn fn_00702400_asks_the_enter_stack_at_an_index() {
        let (mut e, manager) = ui_engine();
        e.register(GET_ENTER_STACK, |_, a| Ret {
            eax: a[1] + 100,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0070_2400, &args![3u32]).u32(), 103);
        assert_eq!(logged(&e, GET_ENTER_STACK), vec![vec![manager, 3]]);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_2400, &args![3u32]).u32(), 0);
    }

    #[test]
    fn fn_00702440_returns_ten() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0070_2440, &[]).u32(), 10);
    }

    #[test]
    fn is_top_menu_id_compares_or_checks_visibility_for_the_placeholder() {
        let (mut e, manager) = ui_engine();
        stub!(e, GET_ENTER_STACK_TOP, 5);
        assert_eq!(e.call(0x0070_2450, &args![5u32]).u8(), 1);
        assert_eq!(e.call(0x0070_2450, &args![6u32]).u8(), 0);
        // The placeholder (1) on top: the stand-in menus use the flag table.
        stub!(e, GET_ENTER_STACK_TOP, 1);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x3eb, 1);
        assert_eq!(e.call(0x0070_2450, &args![0x3ebu32]).u8(), 1);
        assert_eq!(e.call(0x0070_2450, &args![0x3eau32]).u8(), 0);
        // Another id is compared with the top.
        assert_eq!(e.call(0x0070_2450, &args![7u32]).u8(), 0);
        assert_eq!(e.call(0x0070_2450, &args![1u32]).u8(), 1);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_2450, &args![1u32]).u8(), 0);
    }

    #[test]
    fn is_top_menu_faded_in_reads_the_menu_state() {
        let (mut e, _) = ui_engine();
        stub!(e, GET_ENTER_STACK_TOP, 7);
        stub!(e, GET_MENU_BY_CLASS, 0x1234);
        stub!(e, TILE_GET_MENU, 0x2345);
        stub!(e, MENU_STATE, 1);
        assert!(e.call(0x0070_24e0, &[]).bool());
        assert_eq!(logged(&e, GET_MENU_BY_CLASS), vec![vec![7]]);
        assert_eq!(logged(&e, MENU_STATE), vec![vec![0x2345]]);
        stub!(e, MENU_STATE, 2);
        assert!(!e.call(0x0070_24e0, &[]).bool());
        stub!(e, TILE_GET_MENU, 0);
        assert!(!e.call(0x0070_24e0, &[]).bool());
        stub!(e, GET_MENU_BY_CLASS, 0);
        assert!(!e.call(0x0070_24e0, &[]).bool());
    }

    #[test]
    fn is_top_menu_faded_in_picks_the_first_visible_stand_in() {
        let (mut e, _) = ui_engine();
        stub!(e, GET_ENTER_STACK_TOP, 1);
        stub!(e, GET_MENU_BY_CLASS, 0x1234);
        stub!(e, TILE_GET_MENU, 0x2345);
        stub!(e, MENU_STATE, 1);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x40b, 1);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x3ff, 1);
        assert!(e.call(0x0070_24e0, &[]).bool());
        assert_eq!(logged(&e, GET_MENU_BY_CLASS), vec![vec![0x40b]]);
        // None visible: no menu, not faded in.
        e.mem.set_u8(MENU_FLAG_TABLE + 0x40b, 0);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x3ff, 0);
        assert!(!e.call(0x0070_24e0, &[]).bool());
        let mut e = engine_without_manager();
        assert!(!e.call(0x0070_24e0, &[]).bool());
    }

    #[test]
    fn get_menu_mode_type_asks_for_stack_entry_zero() {
        let (mut e, manager) = ui_engine();
        stub!(e, GET_ENTER_STACK, 9);
        assert_eq!(e.call(0x0070_2640, &[]).u32(), 9);
        assert_eq!(logged(&e, GET_ENTER_STACK), vec![vec![manager, 0]]);
        let mut e = engine_without_manager();
        assert_eq!(e.call(0x0070_2640, &[]).u32(), 0);
    }

    #[test]
    fn is_menu_id_visible_handles_the_placeholder_and_the_mask() {
        let (mut e, manager) = ui_engine();
        stub!(e, MANAGER_MODE_IS_THREE, 0);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x3ea, 1);
        // Placeholder: needs 00709c00 and a visible stand-in.
        assert_eq!(e.call(0x0070_2680, &args![1u32, 0u32]).u8(), 0);
        stub!(e, MANAGER_MODE_IS_THREE, 1);
        assert_eq!(e.call(0x0070_2680, &args![1u32, 0u32]).u8(), 1);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x3ea, 0);
        assert_eq!(e.call(0x0070_2680, &args![1u32, 0u32]).u8(), 0);
        // A plain id: the flag table; a mask tests the menu's state word.
        e.mem.set_u8(MENU_FLAG_TABLE + 0x3ee, 1);
        assert_eq!(e.call(0x0070_2680, &args![0x3eeu32, 0u32]).u8(), 1);
        stub!(e, GET_MENU_BY_CLASS, 0x1234);
        stub!(e, TILE_GET_MENU, 0x2345);
        stub!(e, MENU_STATE, 2);
        assert_eq!(e.call(0x0070_2680, &args![0x3eeu32, 4u32]).u8(), 0);
        stub!(e, MENU_STATE, 6);
        assert_eq!(e.call(0x0070_2680, &args![0x3eeu32, 4u32]).u8(), 1);
        // No menu tile for the id: the mask is not tested.
        stub!(e, GET_MENU_BY_CLASS, 0);
        stub!(e, MENU_STATE, 2);
        assert_eq!(e.call(0x0070_2680, &args![0x3eeu32, 4u32]).u8(), 1);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_2680, &args![0x3eeu32, 0u32]).u8(), 0);
    }

    #[test]
    fn fn_007027b0_reads_the_flag_table_inside_the_id_range() {
        let (mut e, _) = ui_engine();
        e.mem.set_u8(MENU_FLAG_TABLE + 0x3e9, 1);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x43c, 1);
        e.mem.set_u8(MENU_FLAG_TABLE + 0x43d, 1);
        assert_eq!(e.call(0x0070_27b0, &args![0x3e9u32]).u8(), 1);
        assert_eq!(e.call(0x0070_27b0, &args![0x43cu32]).u8(), 1);
        assert_eq!(e.call(0x0070_27b0, &args![0x43du32]).u8(), 0);
        assert_eq!(e.call(0x0070_27b0, &args![0x3e8u32]).u8(), 0);
        assert_eq!(e.call(0x0070_27b0, &args![0x3eau32]).u8(), 0);
    }

    #[test]
    fn fn_007027e0_runs_pre_idle() {
        check_forwards_to_manager(0x0070_27e0, 0x0070_b8f0);
    }

    #[test]
    fn fn_00702810_runs_idle() {
        check_forwards_to_manager(0x0070_2810, 0x0070_c4a0);
    }

    #[test]
    fn fn_00702840_calls_on_the_ready_manager() {
        check_forwards_to_manager(0x0070_2840, 0x0071_1ea0);
    }

    /// The calls logged so far, as the frame model's callees.
    fn model_log(e: &Engine) -> Vec<world::frame::interface::Callee> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| world::frame::interface::Callee::Direct(*a))
            .collect()
    }

    fn check_model(address: u32, s: &world::frame::interface::InterfaceState, e: &Engine) {
        use world::frame::interface::{follows, function};
        let model = function(address).expect("modelled");
        follows(model, s, &model_log(e), &[])
            .unwrap_or_else(|why| panic!("{address:08x} {s:?}: {why}"));
    }

    /// The interface idle's wrappers and `Interface::LastMinuteUpdate` follow
    /// `world::frame::interface`'s model with the manager ready, not ready, and
    /// missing (docs/FRAME_SKELETON.md, PR 7).
    #[test]
    fn wrappers_follow_the_frame_model() {
        use world::frame::interface::InterfaceState;
        for (wrapper, callee) in [
            (0x0070_27e0u32, 0x0070_b8f0u32),
            (0x0070_2810, 0x0070_c4a0),
            (0x0070_2840, 0x0071_1ea0),
            (0x0070_58e0, MANAGER_LAST_MINUTE_UPDATE),
        ] {
            for ready in [true, false] {
                let (mut e, manager) = ui_engine();
                e.register(callee, |_, _| Ret::default());
                e.mem.set_u8(manager, u8::from(ready));
                e.call(wrapper, &[]);
                let s = InterfaceState {
                    manager_ready: ready,
                    ..InterfaceState::default()
                };
                check_model(wrapper, &s, &e);
            }
            let mut e = engine_without_manager();
            e.register(callee, |_, _| Ret::default());
            e.call(wrapper, &[]);
            let s = InterfaceState {
                manager_ready: false,
                ..InterfaceState::default()
            };
            check_model(wrapper, &s, &e);
        }
    }

    /// `Interface::UpdateSleeping` (`007056f0`) and `Interface::OpenConsole`
    /// (`00703e10`) follow the model.
    #[test]
    fn sleeping_and_the_console_follow_the_frame_model() {
        use world::frame::interface::InterfaceState;
        for (class, ready) in [(0x3f4i32, true), (0x3eb, true), (0x3f4, false)] {
            let (mut e, manager, _) = menu_object_world(class, SLEEP_WAIT_MENU_TYPE);
            stub!(e, SLEEP_UPDATE_SLEEPING, 1);
            e.mem.set_u8(manager, u8::from(ready));
            e.call_log = Some(vec![]);
            e.call(0x0070_56f0, &[]);
            let s = InterfaceState {
                manager_ready: ready,
                sleep_menu: class == 0x3f4,
                ..InterfaceState::default()
            };
            check_model(0x0070_56f0, &s, &e);
        }
        for (exists, visible) in [(true, false), (true, true), (false, false)] {
            let (mut e, _) = console_world(exists, visible, false);
            e.call_log = Some(vec![]);
            e.call(0x0070_3e10, &[]);
            let s = InterfaceState {
                console_exists: exists,
                console_visible: visible,
                ..InterfaceState::default()
            };
            check_model(0x0070_3e10, &s, &e);
        }
    }

    #[test]
    fn fn_00702870_skips_while_a_movie_plays() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(MOVIE_PLAYER, 0x77);
        e.register(0x0071_3f00, |_, _| Ret::default());
        stub!(e, MOVIE_PLAYER_PLAYING, 1);
        e.call(0x0070_2870, &[]);
        assert!(logged(&e, 0x0071_3f00).is_empty());
        assert_eq!(logged(&e, MOVIE_PLAYER_PLAYING), vec![vec![0x77]]);
        stub!(e, MOVIE_PLAYER_PLAYING, 0);
        e.call(0x0070_2870, &[]);
        assert_eq!(logged(&e, 0x0071_3f00), vec![vec![manager]]);
    }

    #[test]
    fn click_forwards_the_click_or_sets_the_thread_stages() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager + 0x148, 1);
        stub!(e, MOVIE_PLAYER_PLAYING, 0);
        stub!(e, 0x0071_3fb0, 0);
        stub!(e, 0x004e_a970, 0x500);
        e.register(0x00ba_30f0, |_, _| Ret::default());
        e.register(0x00ba_3130, |_, _| Ret::default());
        e.call(0x0070_28b0, &args![10u32, 20u32]);
        assert_eq!(logged(&e, 0x0071_3fb0), vec![vec![manager, 10, 20]]);
        assert!(logged(&e, 0x004e_a970).is_empty());
        assert_eq!(e.mem.u8(manager + 0x148), 0);

        // A movie plays: the stages are set instead.
        e.mem.set_u8(manager + 0x148, 1);
        stub!(e, MOVIE_PLAYER_PLAYING, 1);
        e.call_log = Some(vec![]);
        e.call(0x0070_28b0, &args![10u32, 20u32]);
        assert!(logged(&e, 0x0071_3fb0).is_empty());
        assert_eq!(logged(&e, 0x004e_a970), vec![Vec::<u32>::new(); 2]);
        assert_eq!(logged(&e, 0x00ba_30f0), vec![vec![0x500, 0, 0x17]]);
        assert_eq!(logged(&e, 0x00ba_3130), vec![vec![0x500, 1, 0x17]]);
        assert_eq!(e.mem.u8(manager + 0x148), 0);
    }

    #[test]
    fn click_without_a_manager_sets_the_stages() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager, 0);
        stub!(e, 0x004e_a970, 0x500);
        e.register(0x00ba_30f0, |_, _| Ret::default());
        e.register(0x00ba_3130, |_, _| Ret::default());
        e.call(0x0070_28b0, &args![1u32, 2u32]);
        assert_eq!(logged(&e, 0x004e_a970).len(), 2);
    }

    #[test]
    fn fn_00702940_stores_the_click_flag() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_2940, &args![manager, 1u8]);
        assert_eq!(e.mem.u8(manager + 0x148), 1);
        e.call(0x0070_2940, &args![manager, 0u8]);
        assert_eq!(e.mem.u8(manager + 0x148), 0);
    }

    #[test]
    fn fn_00702960_polls_the_movie_player_when_ready() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(MOVIE_PLAYER, 0x77);
        stub!(e, MOVIE_PLAYER_PLAYING, 0);
        e.call(0x0070_2960, &[]);
        assert_eq!(logged(&e, MOVIE_PLAYER_PLAYING), vec![vec![0x77]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_2960, &[]);
        assert!(logged(&e, MOVIE_PLAYER_PLAYING).is_empty());
    }

    /// A menu tile whose child list (at `+4`) holds the given children.
    fn tile_with_children(e: &mut Engine, children: &[u32]) -> u32 {
        let tile = e.mem.alloc(16);
        let mut next = 0;
        for child in children.iter().rev() {
            next = list_node(e, next, *child);
        }
        e.mem.set_u32(tile + 4, next);
        tile
    }

    fn visible_calls(e: &Engine) -> Vec<(u32, u32)> {
        logged(e, TILE_SET_VALUE)
            .into_iter()
            .map(|w| {
                assert_eq!(w[1], TRAIT_VISIBLE);
                (w[0], w[2])
            })
            .collect()
    }

    #[test]
    fn fn_00702990_sets_the_visibility_of_the_menu_and_its_role_children() {
        let (mut e, manager) = ui_engine();
        list_doubles(&mut e);
        e.mem.set_f64(DOUBLE_110, 110.0);
        e.mem.set_f64(DOUBLE_111, 111.0);
        e.mem.set_f64(DOUBLE_112, 112.0);
        stub!(e, GET_MENUS_ROOT, 0x6000);
        e.register(TILE_SET_VALUE, |_, _| Ret::default());
        e.register(SUBOBJECT_ADD_TILE, |_, _| Ret::default());
        e.register(SET_NODE_FLAG, |_, _| Ret::default());
        e.register(TILE_NODE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        // Children: a role-110 tile, a role-111 tile and one with another role.
        let (child_a, child_b, child_c) = (0x1000, 0x2000, 0x3000);
        let tile = tile_with_children(&mut e, &[child_a, child_b, child_c]);
        tile_values(
            &mut e,
            &[
                ((child_a, TRAIT_ROLE), 110.0),
                ((child_b, TRAIT_ROLE), 111.0),
                ((child_c, TRAIT_ROLE), 5.0),
                // The enabled trait is set and the visible trait is zero for
                // the menu tile, nonzero for the others.
                ((tile, TRAIT_ENABLED), 1.0),
                ((child_a, TRAIT_ENABLED), 1.0),
                ((child_a, TRAIT_VISIBLE), 1.0),
                ((child_b, TRAIT_ENABLED), 1.0),
                ((child_b, TRAIT_VISIBLE), 1.0),
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0070_2990, &args![tile, 1u8, 0u8, 1u8]);
        // The menu is made visible (b = 0 and not role 112), the 110 tile
        // visible, the 111 tile hidden.
        assert_eq!(
            visible_calls(&e),
            vec![(tile, 1), (child_a, 1), (child_b, 0)]
        );
        // The 110 tile is added to the manager's sub-object by address.
        let added = logged(&e, SUBOBJECT_ADD_TILE);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], manager + MANAGER_SUBOBJECT);
        // Node flags: tile A (both traits set: false), tile B (a != 0: true),
        // the menu (visible trait zero: true).
        let flags: Vec<(u32, u32)> = logged(&e, SET_NODE_FLAG)
            .into_iter()
            .map(|w| (w[0], w[1]))
            .collect();
        assert_eq!(
            flags,
            vec![(child_a + 1, 0), (child_b + 1, 1), (tile + 1, 1)]
        );
    }

    #[test]
    fn fn_00702990_without_a_and_for_role_112() {
        let (mut e, _) = ui_engine();
        list_doubles(&mut e);
        e.mem.set_f64(DOUBLE_110, 110.0);
        e.mem.set_f64(DOUBLE_111, 111.0);
        e.mem.set_f64(DOUBLE_112, 112.0);
        stub!(e, GET_MENUS_ROOT, 0x6000);
        e.register(TILE_SET_VALUE, |_, _| Ret::default());
        e.register(SUBOBJECT_ADD_TILE, |_, _| Ret::default());
        e.register(SET_NODE_FLAG, |_, _| Ret::default());
        stub!(e, TILE_NODE, 0);
        let (child_a, child_b) = (0x1000, 0x2000);
        let tile = tile_with_children(&mut e, &[child_a, child_b]);
        tile_values(
            &mut e,
            &[
                ((tile, TRAIT_ROLE), 112.0),
                ((child_a, TRAIT_ROLE), 110.0),
                ((child_b, TRAIT_ROLE), 111.0),
            ],
        );
        e.call_log = Some(vec![]);
        // a = 0, b = 0, the menu has role 112: hidden; 110 hidden, 111 shown.
        e.call(0x0070_2990, &args![tile, 0u8, 0u8, 1u8]);
        assert_eq!(
            visible_calls(&e),
            vec![(tile, 0), (child_a, 0), (child_b, 1)]
        );
        assert!(logged(&e, SUBOBJECT_ADD_TILE).is_empty());
        // A null tile does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0070_2990, &args![0u32, 0u8, 0u8, 1u8]);
        assert!(e.call_log.as_ref().unwrap().len() == 1);
    }

    /// A menus root with the given menu tiles in the list at `+4`.
    fn menus_root_with(e: &mut Engine, menus: &[u32]) -> u32 {
        tile_with_children(e, menus)
    }

    #[test]
    fn isolate_menu_elements_applies_the_menu_update_and_flags_the_nodes() {
        let (mut e, manager) = ui_engine();
        list_doubles(&mut e);
        e.mem.set_f64(DOUBLE_112, 112.0);
        e.mem.set_f64(DOUBLE_110, 110.0);
        e.mem.set_f64(DOUBLE_111, 111.0);
        let menu = tile_with_children(&mut e, &[]);
        let root = menus_root_with(&mut e, &[menu]);
        e.register(GET_MENUS_ROOT, |_, _| Ret::default());
        e.register_double(GET_MENUS_ROOT, move |_, _| Ret {
            eax: root,
            ..Ret::default()
        });
        e.register(TILE_SET_VALUE, |_, _| Ret::default());
        e.register(SUBOBJECT_CLEAR, |_, _| Ret::default());
        e.register(SET_NODE_FLAG, |_, _| Ret::default());
        stub!(e, TILE_NODE, 0);
        stub!(e, MANAGER_NODE_A, 0x900);
        stub!(e, MANAGER_NODE_B, 0);
        tile_values(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0070_2c80, &args![1u8, 0u8]);
        assert_eq!(logged(&e, SUBOBJECT_CLEAR), vec![vec![manager + 0x4c4]]);
        // The menu is visible (b = 0, role is not 112).
        assert_eq!(visible_calls(&e), vec![(menu, 1)]);
        // Only the first node getter returns a node; the flag is 1.
        assert_eq!(logged(&e, SET_NODE_FLAG), vec![vec![0x900, 1]]);

        // a = 0, b = 0: no clear, the node flag is 0.
        e.call_log = Some(vec![]);
        e.call(0x0070_2c80, &args![0u8, 0u8]);
        assert!(logged(&e, SUBOBJECT_CLEAR).is_empty());
        assert_eq!(logged(&e, SET_NODE_FLAG), vec![vec![0x900, 0]]);

        // Not ready: nothing.
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_2c80, &args![1u8, 1u8]);
        assert!(logged(&e, SET_NODE_FLAG).is_empty());
    }

    #[test]
    fn restore_menu_elements_shows_the_menu_and_two_children() {
        let (mut e, manager) = ui_engine();
        list_doubles(&mut e);
        let menu = 0x4000;
        let root = menus_root_with(&mut e, &[menu]);
        e.register_double(GET_MENUS_ROOT, move |_, _| Ret {
            eax: root,
            ..Ret::default()
        });
        e.register(TILE_GET_CHILD_BY_ID, |_, a| Ret {
            eax: if a[1] == 0x6e { 0x5000 } else { 0 },
            ..Ret::default()
        });
        e.register(TILE_SET_VALUE, |_, _| Ret::default());
        e.register(SET_NODE_FLAG, |_, _| Ret::default());
        e.register(TILE_NODE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        // Menu: enabled and visible (flag false); child 0x6e: not enabled.
        tile_values(
            &mut e,
            &[
                ((menu, TRAIT_ENABLED), 1.0),
                ((menu, TRAIT_VISIBLE), 1.0),
                ((0x5000, TRAIT_ENABLED), 0.0),
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0070_2dd0, &[]);
        assert_eq!(visible_calls(&e), vec![(menu, 1), (0x5000, 1)]);
        let flags: Vec<(u32, u32)> = logged(&e, SET_NODE_FLAG)
            .into_iter()
            .map(|w| (w[0], w[1]))
            .collect();
        assert_eq!(flags, vec![(menu + 1, 0), (0x5001, 1)]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_2dd0, &[]);
        assert!(visible_calls(&e).is_empty());
    }

    #[test]
    fn is_debug_text_visible_forwards_the_answer() {
        let (mut e, manager) = ui_engine();
        stub!(e, 0x0064_4640, 1);
        assert!(e.call(0x0070_2fc0, &[]).bool());
        assert_eq!(logged(&e, 0x0064_4640), vec![vec![manager]]);
        stub!(e, 0x0064_4640, 0);
        assert!(!e.call(0x0070_2fc0, &[]).bool());
        let mut e = engine_without_manager();
        assert!(!e.call(0x0070_2fc0, &[]).bool());
    }

    #[test]
    fn toggle_debug_text_visible_flips_the_flag_when_ready() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_3000, &[]);
        assert_eq!(e.mem.u8(manager + 0x7c), 1);
        e.call(0x0070_3000, &[]);
        assert_eq!(e.mem.u8(manager + 0x7c), 0);
        e.mem.set_u8(manager, 0);
        e.call(0x0070_3000, &[]);
        assert_eq!(e.mem.u8(manager + 0x7c), 0);
    }

    #[test]
    fn fn_00703030_flips_the_debug_text_flag() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_3030, &args![manager]);
        assert_eq!(e.mem.u8(manager + 0x7c), 1);
        e.call(0x0070_3030, &args![manager]);
        assert_eq!(e.mem.u8(manager + 0x7c), 0);
    }

    #[test]
    fn fn_00703060_stores_the_debug_text_flag() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_3060, &args![manager, 1u8]);
        assert_eq!(e.mem.u8(manager + 0x7c), 1);
        e.call(0x0070_3060, &args![manager, 0u8]);
        assert_eq!(e.mem.u8(manager + 0x7c), 0);
    }

    #[test]
    fn fn_00703080_passes_two_floats() {
        let (mut e, manager) = ui_engine();
        e.register(0x0071_51b0, |_, _| Ret::default());
        e.call(0x0070_3080, &args![1.5f32, 2.5f32]);
        assert_eq!(
            logged(&e, 0x0071_51b0),
            vec![vec![manager, 1.5f32.to_bits(), 2.5f32.to_bits()]]
        );
        let mut e = engine_without_manager();
        e.register(0x0071_51b0, |_, _| Ret::default());
        e.call(0x0070_3080, &args![1.5f32, 2.5f32]);
        assert!(logged(&e, 0x0071_51b0).is_empty());
    }

    #[test]
    fn fn_007030c0_passes_two_words() {
        let (mut e, manager) = ui_engine();
        e.register(0x0071_5e00, |_, _| Ret::default());
        e.call(0x0070_30c0, &args![11u32, 22u32]);
        assert_eq!(logged(&e, 0x0071_5e00), vec![vec![manager, 11, 22]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_30c0, &args![11u32, 22u32]);
        assert!(logged(&e, 0x0071_5e00).is_empty());
    }

    #[test]
    fn toggle_full_help_flips_the_flag() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_3100, &[]);
        assert_eq!(e.mem.u8(manager + 0xdd), 1);
        e.mem.set_u8(manager + 0xdd, 7);
        e.call(0x0070_3100, &[]);
        assert_eq!(e.mem.u8(manager + 0xdd), 0);
        e.mem.set_u8(manager, 0);
        e.call(0x0070_3100, &[]);
        assert_eq!(e.mem.u8(manager + 0xdd), 0);
    }

    #[test]
    fn get_full_help_reads_the_flag() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager + 0xdd, 1);
        assert_eq!(e.call(0x0070_3150, &[]).u8(), 1);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_3150, &[]).u8(), 0);
    }

    #[test]
    fn get_target_refr_reads_the_pick_reference() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(manager + 0xf0, 0x1234);
        assert_eq!(e.call(0x0070_3180, &[]).u32(), 0x1234);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_3180, &[]).u32(), 0);
        let mut e = engine_without_manager();
        assert_eq!(e.call(0x0070_3180, &[]).u32(), 0);
    }

    #[test]
    fn fn_007031c0_reads_the_pick_reference() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(manager + 0xf0, 0x4321);
        assert_eq!(e.call(0x0070_31c0, &args![manager]).u32(), 0x4321);
    }

    #[test]
    fn fn_007031e0_prints_the_reference_as_debug_text() {
        let (mut e, manager) = ui_engine();
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 100);
        e.mem.set_f32(0x0103_a1c4, 640.0);
        e.mem.set_f32(0x0101_2054, -1.0);
        e.register(0x0040_37b0, |_, _| Ret::default());
        e.register(0x0040_37d0, |_, _| Ret::default());
        stub!(e, 0x0084_e3a0, 0xf00d);
        stub!(e, 0x0055_d520, 0x7000);
        // The formatter stores a character pointer in the first word.
        e.register(0x0040_6f60, |e, a| {
            e.mem.set_u32(a[0], 0xc0de);
            Ret::default()
        });
        e.register_double(0x0043_d4d0, move |_, _| Ret {
            eax: setting,
            ..Ret::default()
        });
        e.register(0x005b_d5b0, |_, _| Ret::default());
        e.register(0x005b_d5c0, |_, _| Ret::default());
        e.register(0x005b_d580, |_, _| Ret::default());
        stub!(e, 0x00ec_62c0, 0);
        stub!(e, 0x00a0_d9e0, 0xd00d);
        e.register(0x00a0_f8b0, |_, _| Ret::default());
        e.register(0x0071_4d70, |_, _| Ret::default());
        e.register(FIRST_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.call(0x0070_31e0, &args![0x9000u32]);
        let format = &logged(&e, 0x0040_6f60)[0];
        assert_eq!(&format[1..], &[0x0103_a1c8, 0x7000, 0xf00d]);
        assert_eq!(logged(&e, 0x005b_d5b0), vec![Vec::<u32>::new()]);
        assert_eq!(logged(&e, 0x005b_d5c0)[0][1], 99);
        assert_eq!(
            logged(&e, 0x00a0_f8b0),
            vec![vec![
                0xd00d,
                0xc0de,
                640.0f32.to_bits(),
                100.0f32.to_bits(),
                2,
                0xffff_ffff,
                (-1.0f32).to_bits(),
                0,
                0
            ]]
        );
        assert_eq!(logged(&e, 0x0071_4d70), vec![vec![manager, 0x9000]]);

        // A null reference prints nothing but still calls 00714d70.
        e.call_log = Some(vec![]);
        e.call(0x0070_31e0, &args![0u32]);
        assert!(logged(&e, 0x00a0_f8b0).is_empty());
        assert_eq!(logged(&e, 0x0071_4d70), vec![vec![manager, 0]]);

        // Not ready: nothing at all.
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_31e0, &args![0x9000u32]);
        assert!(logged(&e, 0x0071_4d70).is_empty());
    }

    #[test]
    fn set_pick_refr_forwards_the_reference() {
        let (mut e, manager) = ui_engine();
        e.register(0x009b_df60, |_, _| Ret::default());
        e.call(0x0070_3310, &args![0x55u32]);
        assert_eq!(logged(&e, 0x009b_df60), vec![vec![manager, 0x55]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_3310, &args![0x55u32]);
        assert!(logged(&e, 0x009b_df60).is_empty());
    }

    #[test]
    fn fn_00703350_forwards_the_answer() {
        let (mut e, manager) = ui_engine();
        stub!(e, 0x0087_4480, 0x66);
        assert_eq!(e.call(0x0070_3350, &[]).u32(), 0x66);
        assert_eq!(logged(&e, 0x0087_4480), vec![vec![manager]]);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_3350, &[]).u32(), 0);
    }

    #[test]
    fn get_telekinesis_refr_forwards_the_answer() {
        let (mut e, manager) = ui_engine();
        stub!(e, 0x009b_4440, 0x77);
        assert_eq!(e.call(0x0070_3390, &[]).u32(), 0x77);
        assert_eq!(logged(&e, 0x009b_4440), vec![vec![manager]]);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_3390, &[]).u32(), 0);
    }

    #[test]
    fn fn_007033d0_fills_the_point_or_copies_the_default() {
        let (mut e, manager) = ui_engine();
        let out = e.mem.alloc(12);
        e.register(0x009b_7010, |e, a| {
            e.mem.set_u32(a[1], 9);
            Ret::default()
        });
        let back = e.call(0x0070_33d0, &args![Ptr::<()>::new(out)]).u32();
        assert_eq!(back, out);
        assert_eq!(e.mem.u32(out), 9);
        assert_eq!(logged(&e, 0x009b_7010), vec![vec![manager, out]]);

        e.mem.set_u8(manager, 0);
        for (i, word) in [1u32, 2, 3].into_iter().enumerate() {
            e.mem.set_u32(0x011f_426c + 4 * i as u32, word);
        }
        let back = e.call(0x0070_33d0, &args![Ptr::<()>::new(out)]).u32();
        assert_eq!(back, out);
        assert_eq!(
            [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)],
            [1, 2, 3]
        );
    }

    #[test]
    fn fn_00703430_reads_the_fuzzy_pick_flag() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager + 0x110, 1);
        assert!(e.call(0x0070_3430, &[]).bool());
        e.mem.set_u8(manager, 0);
        assert!(!e.call(0x0070_3430, &[]).bool());
    }

    #[test]
    fn fn_00703470_reads_the_fuzzy_pick_flag() {
        let (mut e, manager) = ui_engine();
        assert!(!e.call(0x0070_3470, &args![manager]).bool());
        e.mem.set_u8(manager + 0x110, 1);
        assert!(e.call(0x0070_3470, &args![manager]).bool());
    }

    // ---- Tests of 00703490..00704af0 --------------------------------------

    use std::cell::RefCell;
    use std::rc::Rc;

    type Shared<T> = Rc<RefCell<T>>;

    /// Every logged call to one of `addrs`, in order, with its arguments.
    fn trace(e: &Engine, addrs: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addrs.contains(a))
            .cloned()
            .collect()
    }

    /// Registers a double that does nothing for each address.
    fn ignore(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// The tile the menu doubles answer for a menu id.
    fn tile_of(id: i32) -> u32 {
        id as u32 * 0x100
    }

    /// The menu of a tile (`Tile::GetMenu` double: tile + 1).
    fn menu_of(id: i32) -> u32 {
        tile_of(id) + 1
    }

    const MENU_ADDRESSES: [u32; 17] = [
        MENU_START_FADE_OUT,
        MENU_START_FADE_IN,
        STATS_TOGGLE_HEALING_MODE,
        REPAIR_RESET_MENU,
        ITEM_MOD_RESET_MENU,
        MAP_MENU_UPDATE,
        INVENTORY_UPDATE_MENU,
        INVENTORY_SAVE_PAGE_STATE,
        TILE_SET_VALUE,
        PIPBOY_REFRESH,
        PIPBOY_SELECT_TAB,
        0x0078_2470,
        0x0079_c340,
        STATS_UPDATE_HEALING_OPTIONS,
        0x0073_0690,
        0x0072_8a30,
        0x0070_4bc0,
    ];

    /// A manager with the menus of `present` created (their tiles are
    /// [`tile_of`], their menus [`menu_of`]), the creation functions
    /// answering the tile of the menu they make, and doubles that log every
    /// menu action. `pipboy_query` is the answer of `00974d90`.
    fn menu_world(present: &[i32], pipboy_query: bool) -> (Engine, u32) {
        let (mut e, manager) = ui_engine();
        let present = present.to_vec();
        e.register_double(GET_MENU_BY_CLASS, move |_, a| Ret {
            eax: if present.contains(&(a[0] as i32)) {
                tile_of(a[0] as i32)
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(TILE_GET_MENU, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        ignore(&mut e, &MENU_ADDRESSES);
        stub!(e, GET_MENUS_ROOT, 0x6000);
        stub!(e, MANAGER_PIPBOY, 0x7000);
        e.register_double(PIPBOY_QUERY, move |_, _| Ret {
            eax: pipboy_query as u32,
            ..Ret::default()
        });
        stub!(e, 0x007b_5520, 0x3b00);
        stub!(e, 0x0078_3440, 0x3c00);
        stub!(e, 0x0077_fc10, 0x3d00);
        stub!(e, 0x0079_6b90, 0x3e00);
        stub!(e, 0x0070_4df0, 0);
        e.call_log = Some(vec![]);
        (e, manager)
    }

    /// The addresses [`trace`] follows in the menu tests.
    fn menu_trace(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        let mut addrs = MENU_ADDRESSES.to_vec();
        addrs.extend([0x007b_5520, 0x0078_3440, 0x0077_fc10, 0x0079_6b90]);
        trace(e, &addrs)
    }

    #[test]
    fn fn_00703490_forwards_to_the_manager() {
        check_forwards_to_manager(0x0070_3490, 0x0070_bc20);
    }

    #[test]
    fn fn_007034c0_passes_its_word() {
        let (mut e, manager) = ui_engine();
        ignore(&mut e, &[0x0098_4f60]);
        e.call(0x0070_34c0, &args![9u32]);
        assert_eq!(logged(&e, 0x0098_4f60), vec![vec![manager, 9]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_34c0, &args![9u32]);
        assert!(logged(&e, 0x0098_4f60).is_empty());
    }

    /// A world for `ShowMenus`: the menus root's current-menu trait holds
    /// `current`; `_ftol2` converts it.
    fn show_world(current: f32) -> (Engine, u32) {
        let (mut e, manager) = menu_world(&[], false);
        tile_values(&mut e, &[((0x6000, TRAIT_CURRENT_MENU), current)]);
        e.register(FLOAT_TO_INT, |_, a| Ret {
            eax: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32,
            ..Ret::default()
        });
        ignore(&mut e, &[0x0071_7600, 0x0070_4c10]);
        e.call_log = Some(vec![]);
        (e, manager)
    }

    #[test]
    fn show_menus_picks_the_menu_from_the_current_id() {
        // Stats menu.
        let (mut e, _) = show_world(1003.0);
        e.call(0x0070_3500, &[]);
        assert_eq!(logged(&e, 0x0070_4c10), vec![vec![1, 0]]);
        assert!(logged(&e, 0x0071_7600).is_empty());
        // Id 0: the manager's `00717600` runs and the stats menu is shown.
        let (mut e, manager) = show_world(0.0);
        e.call(0x0070_3500, &[]);
        assert_eq!(logged(&e, 0x0071_7600), vec![vec![manager]]);
        assert_eq!(logged(&e, 0x0070_4c10), vec![vec![1, 0]]);
        // Map menu (created on the way).
        let (mut e, _) = show_world(1023.0);
        e.call(0x0070_3500, &[]);
        assert_eq!(logged(&e, 0x0079_6b90), vec![vec![]]);
        assert_eq!(logged(&e, MAP_MENU_UPDATE).len(), 1);
        // Inventory for its id and for the repair and item-mod ids.
        for id in [1002.0, 1035.0, 1061.0] {
            let (mut e, _) = show_world(id);
            e.call(0x0070_3500, &[]);
            assert_eq!(logged(&e, 0x0077_fc10).len(), 1);
            assert_eq!(logged(&e, INVENTORY_UPDATE_MENU).len(), 1);
        }
        // An unknown id shows nothing; neither does a manager that is not
        // ready.
        let (mut e, manager) = show_world(1100.0);
        e.call(0x0070_3500, &[]);
        assert!(logged(&e, 0x0070_4c10).is_empty());
        assert!(logged(&e, 0x0077_fc10).is_empty());
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_3500, &[]);
        assert!(logged(&e, GET_MENUS_ROOT).is_empty());
    }

    #[test]
    fn hide_menus_puts_each_existing_menu_in_state_four() {
        let (mut e, manager) = menu_world(&[0x3eb, 0x3ff], false);
        ignore(&mut e, &[MENU_SET_STATE, 0x0070_4f50, ENTER_RENDERED_MENU]);
        e.register(MENU_TILE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register_double(0x004a_4040, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        ignore(&mut e, &[START_MENU_CLOSE]);
        e.call(0x0070_3610, &[]);
        let log = trace(
            &e,
            &[
                MENU_SET_STATE,
                TILE_SET_VALUE,
                0x0070_4f50,
                START_MENU_CLOSE,
                ENTER_RENDERED_MENU,
            ],
        );
        assert_eq!(
            log,
            vec![
                (MENU_SET_STATE, vec![menu_of(0x3eb), 4]),
                (TILE_SET_VALUE, vec![menu_of(0x3eb) + 1, TRAIT_ENABLED, 0]),
                (MENU_SET_STATE, vec![menu_of(0x3ff), 4]),
                (TILE_SET_VALUE, vec![menu_of(0x3ff) + 1, TRAIT_ENABLED, 0]),
                (0x0070_4f50, vec![]),
                (START_MENU_CLOSE, vec![1]),
                (ENTER_RENDERED_MENU, vec![manager, 0]),
            ]
        );
        // The start menu is not closed when it is not open.
        e.register(0x004a_4040, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_3610, &[]);
        assert!(logged(&e, START_MENU_CLOSE).is_empty());
        assert_eq!(logged(&e, ENTER_RENDERED_MENU), vec![vec![manager, 0]]);
        // Not ready: nothing.
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_3610, &[]);
        assert!(logged(&e, ENTER_RENDERED_MENU).is_empty());
    }

    #[test]
    fn emergency_close_forwards_to_the_manager() {
        check_forwards_to_manager(0x0070_37e0, 0x0071_4c60);
    }

    #[test]
    fn fn_00703810_sets_the_root_node_flag_from_the_byte() {
        let (mut e, manager) = ui_engine();
        stub!(e, GET_MENUS_ROOT, 0x6000);
        stub!(e, TILE_NODE, 0x900);
        ignore(&mut e, &[SET_NODE_FLAG]);
        e.call(0x0070_3810, &args![0u8]);
        e.call(0x0070_3810, &args![5u8]);
        assert_eq!(
            logged(&e, SET_NODE_FLAG),
            vec![vec![0x900, 1], vec![0x900, 0]]
        );
        assert_eq!(logged(&e, TILE_NODE), vec![vec![0x6000], vec![0x6000]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_3810, &args![0u8]);
        assert!(logged(&e, SET_NODE_FLAG).is_empty());
    }

    #[test]
    fn set_crosshair_target_type_needs_the_object_and_its_answer() {
        let (mut e, _) = ui_engine();
        ignore(&mut e, &[0x0077_2d10]);
        // No object: nothing.
        e.call(0x0070_3860, &args![7u32]);
        assert!(logged(&e, 0x0077_2d10).is_empty());
        // An object whose slot 0x1d0 answers nonzero.
        let answer = Rc::new(RefCell::new(1u32));
        let seen = answer.clone();
        e.register_double(0x00c0_0001, move |_, _| Ret {
            eax: *seen.borrow(),
            ..Ret::default()
        });
        let mut slots = vec![0u32; 0x75];
        slots[0x74] = 0x00c0_0001;
        e.put_vtable(0x0140_0000, &slots);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0140_0000);
        e.set_global(0x011d_ea3c, object);
        e.call(0x0070_3860, &args![7u32]);
        assert_eq!(logged(&e, 0x0077_2d10), vec![vec![7, 0]]);
        *answer.borrow_mut() = 0;
        e.call_log = Some(vec![]);
        e.call(0x0070_3860, &args![7u32]);
        assert!(logged(&e, 0x0077_2d10).is_empty());
    }

    #[test]
    fn fn_007038a0_reads_its_global() {
        let (mut e, _) = ui_engine();
        e.map(0x011d_9000, 0x1000);
        e.set_global(0x011d_96b4, 0x55u32);
        assert_eq!(e.call(0x0070_38a0, &[]).u32(), 0x55);
    }

    #[test]
    fn set_enemy_actor_and_fn_007038d0_forward() {
        let (mut e, _) = ui_engine();
        ignore(&mut e, &[0x0077_3190, 0x0077_32b0]);
        e.call(0x0070_38b0, &args![0x1234u32]);
        e.call(0x0070_38d0, &[]);
        assert_eq!(logged(&e, 0x0077_3190), vec![vec![0x1234]]);
        assert_eq!(logged(&e, 0x0077_32b0), vec![vec![]]);
    }

    #[test]
    fn toggle_safe_zone_passes_the_argument_to_the_manager() {
        let (mut e, manager) = ui_engine();
        ignore(&mut e, &[0x0071_5ec0]);
        e.call(0x0070_38e0, &args![3u32]);
        assert_eq!(logged(&e, 0x0071_5ec0), vec![vec![manager, 3]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_38e0, &args![3u32]);
        assert!(logged(&e, 0x0071_5ec0).is_empty());
    }

    #[test]
    fn first_chance_texture_release_sets_the_byte_and_updates_tiles() {
        let (mut e, manager) = ui_engine();
        ignore(&mut e, &[0x00a0_4200]);
        e.call(0x0070_3920, &[]);
        assert_eq!(e.mem.u8(manager + 0xec), 1);
        assert_eq!(logged(&e, 0x00a0_4200), vec![vec![0]]);
        e.mem.set_u8(manager, 0);
        e.mem.set_u8(manager + 0xec, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_3920, &[]);
        assert_eq!(e.mem.u8(manager + 0xec), 0);
        assert!(logged(&e, 0x00a0_4200).is_empty());
    }

    #[test]
    fn fn_00703960_stores_the_byte() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_3960, &args![manager, 7u8]);
        assert_eq!(e.mem.u8(manager + 0xec), 7);
    }

    #[test]
    fn fn_00703980_forwards_to_the_manager() {
        check_forwards_to_manager(0x0070_3980, 0x0071_60b0);
    }

    /// A world for `GetControlPushTextFor`: the controls object (global
    /// `011dea0c`) holds `mode_bytes[mode]` for control 5, `settings` are
    /// the table entries, and `device_mode` is the answer of `004b71d0`.
    struct ControlWorld {
        e: Engine,
        buffer: u32,
    }

    fn control_world(mode_bytes: [u8; 4], device_mode: bool, trigger: bool) -> ControlWorld {
        let (mut e, _) = ui_engine();
        for page in [0x011d_5000, 0x011d_3000, 0x011d_4000] {
            e.map(page, 0x1000);
        }
        let object = e.mem.alloc(0x1c00);
        for (mode, byte) in mode_bytes.iter().enumerate() {
            e.mem
                .set_u8(object + 0x1b94 + mode as u32 * 0x1c + 5, *byte);
        }
        e.set_global(CONTROLS_OBJECT_GLOBAL, object);
        e.register(CONTROLS_OBJECT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(0x004b_71d0, move |_, _| Ret {
            eax: device_mode as u32,
            ..Ret::default()
        });
        // The tables hold the address of a "setting" object per index; its
        // text pointer (+8) is nonzero and says "trigger" or not.
        for (table, count) in [
            (0x011d_5268u32, 4u32),
            (0x011d_5240, 9),
            (0x011d_52f0, 0xee),
            (0x011d_51b0, 8),
        ] {
            for index in 0..count {
                let setting = e.mem.alloc(16);
                e.mem.set_u32(setting + 8, 0x7000 + table + index);
                e.mem.set_u32(table + 4 * index, setting);
            }
        }
        e.register(SETTING_TEXT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register_double(STRING_STRISTR, move |_, _| Ret {
            eax: trigger as u32,
            ..Ret::default()
        });
        e.register(FIRST_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        ignore(
            &mut e,
            &[
                STRING_COPY,
                STRING_CTOR,
                STRING_DTOR,
                STRING_ASSIGN,
                SPRINTF,
            ],
        );
        e.call_log = Some(vec![]);
        let buffer = e.mem.alloc(64);
        ControlWorld { e, buffer }
    }

    fn setting_at(e: &Engine, table: u32, index: u32) -> u32 {
        e.mem.u32(table + 4 * index)
    }

    #[test]
    fn get_control_push_text_for_picks_the_mode_and_copies_the_text() {
        // Device mode: mode 3, table 011d5268.
        let mut w = control_world([0, 0, 0, 2], true, false);
        w.e.mem.set_u8(w.buffer, b'x');
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 0u8]);
        let setting = setting_at(&w.e, 0x011d_5268, 2);
        assert_eq!(logged(&w.e, STRING_COPY), vec![vec![w.buffer, setting + 1]]);
        assert_eq!(w.e.mem.u8(w.buffer), 0);
        // Not device mode: mode 1 first (index below 9).
        let mut w = control_world([0, 4, 0, 0], false, false);
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 0u8]);
        let setting = setting_at(&w.e, 0x011d_5240, 4);
        assert_eq!(logged(&w.e, STRING_COPY), vec![vec![w.buffer, setting + 1]]);
        // Mode 1 index 9 or more: mode 0 (below 0xee).
        let mut w = control_world([0xed, 9, 0, 0], false, false);
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 0u8]);
        let setting = setting_at(&w.e, 0x011d_52f0, 0xed);
        assert_eq!(logged(&w.e, STRING_COPY), vec![vec![w.buffer, setting + 1]]);
        // Mode 0 index 0xee or more: mode 2 (below 8).
        let mut w = control_world([0xee, 200, 7, 0], false, false);
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 0u8]);
        let setting = setting_at(&w.e, 0x011d_51b0, 7);
        assert_eq!(logged(&w.e, STRING_COPY), vec![vec![w.buffer, setting + 1]]);
        // Mode 2 index 8 or more: no setting, nothing copied.
        let mut w = control_world([0xee, 200, 8, 0], false, false);
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 0u8]);
        assert!(logged(&w.e, STRING_COPY).is_empty());
    }

    #[test]
    fn get_control_push_text_for_ignores_controls_out_of_range() {
        for control in [-1i32, 0x1d] {
            let mut w = control_world([0; 4], true, false);
            w.e.mem.set_u8(w.buffer, b'x');
            w.e.call(0x0070_39b0, &args![control, w.buffer, 1u8]);
            assert_eq!(w.e.mem.u8(w.buffer), b'x');
            assert!(logged(&w.e, STRING_CTOR).is_empty());
        }
        // The edges are valid.
        let mut w = control_world([0; 4], true, false);
        w.e.call(0x0070_39b0, &args![0x1ci32, w.buffer, 0u8]);
        assert_eq!(logged(&w.e, STRING_COPY).len(), 1);
    }

    #[test]
    fn get_control_push_text_for_builds_the_label() {
        // A trigger control with an empty buffer: the fallback text goes
        // into the string and the label is the one at 011d4a08.
        let mut w = control_world([0, 0, 0, 1], true, true);
        let text_word = w.e.mem.alloc(4);
        w.e.register_double(STRING_ASSIGN, move |e, a| {
            e.mem.set_u32(a[0], text_word);
            Ret::default()
        });
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 1u8]);
        assert_eq!(logged(&w.e, STRING_ASSIGN)[0][1], 0x011d_37b1);
        let log = logged(&w.e, SPRINTF);
        assert_eq!(
            log,
            vec![vec![w.buffer, 0x0101_2058, 0x011d_4a09, text_word]]
        );
        assert_eq!(logged(&w.e, STRING_DTOR).len(), 1);
        // Not a trigger: the other label.
        let mut w = control_world([0, 0, 0, 1], true, false);
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 1u8]);
        assert_eq!(logged(&w.e, SPRINTF)[0][2], 0x011d_3fc5);
        // The buffer text is kept when it is not empty.
        let mut w = control_world([0, 0, 0, 1], true, false);
        w.e.register(STRING_COPY, |e, a| {
            e.mem.set_u8(a[0], b'A');
            Ret::default()
        });
        w.e.call(0x0070_39b0, &args![5i32, w.buffer, 1u8]);
        assert_eq!(logged(&w.e, STRING_ASSIGN)[0][1], w.buffer);
    }

    #[test]
    fn fn_00703be0_indexes_by_mode_and_control() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(0x1c00);
        e.mem.set_u8(object + 0x1b94 + 2 * 0x1c + 5, 0x42);
        assert_eq!(e.call(0x0070_3be0, &args![object, 5u32, 2u32]).u8(), 0x42);
        assert_eq!(e.call(0x0070_3be0, &args![object, 5u32, 1u32]).u8(), 0);
    }

    /// Doubles for the console: `exists`, `visible` and `accepts_text`.
    fn console_world(exists: bool, visible: bool, accepts_text: bool) -> (Engine, u32) {
        let (mut e, manager) = ui_engine();
        e.register_double(CONSOLE_INSTANCE, move |_, a| Ret {
            eax: if a[0] == 0 { exists as u32 } else { 0xc0de },
            ..Ret::default()
        });
        e.register_double(CONSOLE_IS_VISIBLE, move |_, _| Ret {
            eax: visible as u32,
            ..Ret::default()
        });
        e.register_double(CONSOLE_ACCEPTS_TEXT, move |_, _| Ret {
            eax: accepts_text as u32,
            ..Ret::default()
        });
        ignore(
            &mut e,
            &[
                CONSOLE_TOGGLE_VISIBLE,
                POP_FROM_ENTER_STACK,
                ADD_TO_ENTER_STACK,
                STRING_DTOR,
            ],
        );
        e.register(FIRST_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        (e, manager)
    }

    /// A `0071d0a0` double that records the words its `va_list` points at.
    fn record_prints(e: &mut Engine, words: usize) -> Shared<Vec<(u32, u32, Vec<u32>)>> {
        let seen = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(CONSOLE_PRINT, move |e, a| {
            let list: Vec<u32> = (0..words).map(|i| e.mem.u32(a[2] + 4 * i as u32)).collect();
            record.borrow_mut().push((a[0], a[1], list));
            Ret::default()
        });
        seen
    }

    #[test]
    fn fn_00703c00_prints_when_the_console_takes_text() {
        // Visible console.
        let (mut e, _) = console_world(true, true, false);
        let seen = record_prints(&mut e, 2);
        e.call(0x0070_3c00, &args![0x500u32, 11u32, 22u32]);
        assert_eq!(*seen.borrow(), vec![(0xc0de, 0x500, vec![11, 22])]);
        // Hidden console, but text is accepted anyway.
        let (mut e, _) = console_world(true, false, true);
        let seen = record_prints(&mut e, 0);
        e.call(0x0070_3c00, &args![0x500u32]);
        assert_eq!(seen.borrow().len(), 1);
        // Neither: nothing.
        let (mut e, _) = console_world(false, false, false);
        let seen = record_prints(&mut e, 0);
        e.call(0x0070_3c00, &args![0x500u32]);
        assert!(seen.borrow().is_empty());
        // Not ready: nothing.
        let (mut e, manager) = console_world(true, true, true);
        let seen = record_prints(&mut e, 0);
        e.mem.set_u8(manager, 0);
        e.call(0x0070_3c00, &args![0x500u32]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn print_line_prints_the_string_and_destroys_it() {
        let (mut e, _) = console_world(true, true, false);
        let seen = record_prints(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x0070_3c80, &args![0x500u32, 5u32, 77u32]);
        assert_eq!(*seen.borrow(), vec![(0xc0de, 0x500, vec![77])]);
        let dtor = logged(&e, STRING_DTOR);
        assert_eq!(dtor.len(), 1);
        // The string is destroyed even when nothing is printed.
        let (mut e, manager) = console_world(true, true, true);
        let seen = record_prints(&mut e, 0);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_3c80, &args![0x500u32, 5u32]);
        assert!(seen.borrow().is_empty());
        assert_eq!(logged(&e, STRING_DTOR).len(), 1);
    }

    #[test]
    fn is_console_visible_needs_manager_console_and_visibility() {
        let (mut e, manager) = console_world(true, true, false);
        assert!(e.call(0x0070_3d50, &[]).bool());
        let (mut e2, _) = console_world(true, false, false);
        assert!(!e2.call(0x0070_3d50, &[]).bool());
        let (mut e3, _) = console_world(false, true, false);
        assert!(!e3.call(0x0070_3d50, &[]).bool());
        e.mem.set_u8(manager, 0);
        assert!(!e.call(0x0070_3d50, &[]).bool());
    }

    #[test]
    fn close_console_toggles_a_visible_console_and_pops_the_stack() {
        let (mut e, manager) = console_world(true, true, false);
        e.call_log = Some(vec![]);
        e.call(0x0070_3da0, &[]);
        assert_eq!(logged(&e, CONSOLE_TOGGLE_VISIBLE), vec![vec![0xc0de]]);
        assert_eq!(logged(&e, POP_FROM_ENTER_STACK), vec![vec![manager, 3, 0]]);
        // A hidden console is left alone.
        let (mut e, _) = console_world(true, false, false);
        e.call_log = Some(vec![]);
        e.call(0x0070_3da0, &[]);
        assert!(logged(&e, CONSOLE_TOGGLE_VISIBLE).is_empty());
    }

    #[test]
    fn open_console_toggles_a_hidden_console_and_pushes_the_stack() {
        let (mut e, manager) = console_world(true, false, false);
        e.call_log = Some(vec![]);
        e.call(0x0070_3e10, &[]);
        assert_eq!(logged(&e, CONSOLE_TOGGLE_VISIBLE), vec![vec![0xc0de]]);
        assert_eq!(logged(&e, ADD_TO_ENTER_STACK), vec![vec![manager, 3]]);
        // A visible console, or none: nothing.
        for (exists, visible) in [(true, true), (false, false)] {
            let (mut e, _) = console_world(exists, visible, false);
            e.call_log = Some(vec![]);
            e.call(0x0070_3e10, &[]);
            assert!(logged(&e, ADD_TO_ENTER_STACK).is_empty());
        }
    }

    /// A `007a8e60` double: records its words and the words the
    /// `va_list` (a pointer, or a pointer to the pointer) leads to.
    fn record_message(
        e: &mut Engine,
        va_words: usize,
        answer: u32,
    ) -> Shared<Vec<(Vec<u32>, Vec<u32>)>> {
        let seen = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(MESSAGE_CREATE, move |e, a| {
            let list = e.mem.u32(a[8]);
            let words = (0..va_words)
                .map(|i| e.mem.u32(list + 4 * i as u32))
                .collect();
            record.borrow_mut().push((a.to_vec(), words));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        seen
    }

    #[test]
    fn fn_00703e80_forwards_nine_words_and_the_variable_ones() {
        let (mut e, manager) = ui_engine();
        let seen = record_message(&mut e, 2, 1);
        let words = args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 1.5f32, 2.5f32, 9u32, 70u32, 80u32];
        assert!(e.call(0x0070_3e80, &words).bool());
        let log = seen.borrow();
        let (call, va) = &log[0];
        assert_eq!(call[..8], [1, 2, 3, 0, 4, 5, 6, 9]);
        assert_eq!(
            call[9..],
            [1, u32::MAX, u32::MAX, 0, 1.5f32.to_bits(), 2.5f32.to_bits()]
        );
        assert_eq!(*va, vec![70, 80]);
        drop(log);
        e.mem.set_u8(manager, 0);
        assert!(!e.call(0x0070_3e80, &words).bool());
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn fn_00703f10_forwards_ten_words_and_the_variable_ones() {
        let (mut e, manager) = ui_engine();
        let seen = record_message(&mut e, 1, 1);
        let words = args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 1.5f32, 2.5f32, 10u32, 70u32];
        assert!(e.call(0x0070_3f10, &words).bool());
        let log = seen.borrow();
        let (call, va) = &log[0];
        assert_eq!(call[..8], [1, 2, 3, 4, 5, 6, 7, 10]);
        assert_eq!(
            call[9..],
            [1, u32::MAX, u32::MAX, 0, 1.5f32.to_bits(), 2.5f32.to_bits()]
        );
        assert_eq!(*va, vec![70]);
        drop(log);
        e.mem.set_u8(manager, 0);
        assert!(!e.call(0x0070_3f10, &words).bool());
    }

    #[test]
    fn get_message_menu_result_reads_and_resets_the_byte() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager + 0xe4, 3);
        assert_eq!(e.call(0x0070_3fa0, &[]).u8(), 3);
        assert_eq!(e.mem.u8(manager + 0xe4), 0xff);
        assert_eq!(e.call(0x0070_3fa0, &[]).u8(), 0xff);
    }

    #[test]
    fn message_menu_wrappers_forward() {
        let (mut e, _) = ui_engine();
        ignore(&mut e, &[0x007a_a5c0, 0x007a_a6b0, 0x007a_a530]);
        e.call(0x0070_3fd0, &[]);
        e.call(0x0070_3fe0, &args![1u8]);
        e.call(0x0070_4000, &[]);
        assert_eq!(logged(&e, 0x007a_a5c0), vec![vec![]]);
        assert_eq!(logged(&e, 0x007a_a6b0), vec![vec![1]]);
        assert_eq!(logged(&e, 0x007a_a530), vec![vec![]]);
    }

    /// Doubles for the interface loop of `00704010`.
    fn message_loop_world(created: u32) -> Engine {
        let (mut e, _) = ui_engine();
        e.map(0x0119_f000, 0x1000);
        ignore(
            &mut e,
            &[
                REMOVE_FADER,
                MEMORY_FILL,
                TRANSLATE_MESSAGE,
                DISPATCH_MESSAGE,
                0x0070_b8f0,
                0x0070_c4a0,
                0x0071_1ea0,
                0x0071_3fb0,
            ],
        );
        e.register_double(MESSAGE_CREATE, move |_, _| Ret {
            eax: created,
            ..Ret::default()
        });
        e.register(MOVIE_PLAYER_PLAYING, |_, _| Ret::default());
        stub!(e, CONTROLS_OBJECT, 0x4400);
        // One message per frame is pumped; the second frame's poll stores
        // the answer.
        let pumped = Rc::new(RefCell::new(0u32));
        let peeks = pumped.clone();
        e.register_double(PEEK_MESSAGE, move |_, _| {
            let mut n = peeks.borrow_mut();
            *n += 1;
            Ret {
                eax: (*n % 2 == 1) as u32,
                ..Ret::default()
            }
        });
        let polls = Rc::new(RefCell::new(0u32));
        e.register_double(CONTROLS_POLL, move |e, _| {
            *polls.borrow_mut() += 1;
            if *polls.borrow() == 2 {
                e.mem.set_u8(MESSAGE_RESULT, 6);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00704010_runs_the_loop_until_the_answer_arrives() {
        let mut e = message_loop_world(1);
        let words = args![0x10u32, 0x20u32, 0x30u32, 0x40u32, 99u32];
        assert_eq!(e.call(0x0070_4010, &words).u8(), 6);
        let create = trace(&e, &[MESSAGE_CREATE]);
        assert_eq!(create.len(), 1);
        assert_eq!(
            create[0].1[..8],
            [0x10, 0, 0, 0, 0x0070_4130, 0x20, 0x30, 0x40]
        );
        assert_eq!(create[0].1[9..], [1, u32::MAX, u32::MAX, 0, 0, 0]);
        assert_eq!(logged(&e, REMOVE_FADER), vec![vec![0, 1, 1]]);
        // Two frames: two polls, a message pumped in each frame.
        assert_eq!(logged(&e, CONTROLS_POLL).len(), 2);
        assert_eq!(logged(&e, TRANSLATE_MESSAGE).len(), 2);
        assert_eq!(logged(&e, 0x0070_b8f0).len(), 2);
        assert_eq!(logged(&e, 0x0070_c4a0).len(), 2);
    }

    #[test]
    fn fn_00704010_answers_zero_when_no_message_was_made() {
        let mut e = message_loop_world(0);
        let words = args![1u32, 2u32, 3u32, 4u32];
        assert_eq!(e.call(0x0070_4010, &words).u8(), 0);
        assert!(logged(&e, CONTROLS_POLL).is_empty());
        assert_eq!(e.mem.u8(MESSAGE_RESULT), 0xff);
        // The same when the manager is not ready.
        let mut e = message_loop_world(1);
        e.mem.set_u8(e.mem.u32(0x011d_8a80), 0);
        assert_eq!(e.call(0x0070_4010, &words).u8(), 0);
        assert!(trace(&e, &[MESSAGE_CREATE]).is_empty());
    }

    #[test]
    fn fn_00704130_stores_the_message_result() {
        let (mut e, manager) = ui_engine();
        e.map(0x0119_f000, 0x1000);
        e.mem.set_u8(manager + 0xe4, 4);
        e.call(0x0070_4130, &[]);
        assert_eq!(e.mem.u8(MESSAGE_RESULT), 4);
        assert_eq!(e.mem.u8(manager + 0xe4), 0xff);
    }

    #[test]
    fn creation_wrappers_call_the_menu_creators_when_ready() {
        for (wrapper, creator) in [
            (0x0070_4140u32, 0x0079_6b90u32),
            (0x0070_4490, 0x007b_5520),
            (0x0070_46c0, 0x0078_3440),
            (0x0070_48c0, 0x0077_fc10),
        ] {
            let (mut e, manager) = ui_engine();
            e.register(creator, |_, _| Ret {
                eax: 0xabc,
                ..Ret::default()
            });
            assert_eq!(e.call(wrapper, &[]).u32(), 0xabc);
            e.mem.set_u8(manager, 0);
            assert_eq!(e.call(wrapper, &[]).u32(), 0);
        }
    }

    #[test]
    fn fn_00704690_passes_its_word() {
        let (mut e, manager) = ui_engine();
        ignore(&mut e, &[0x007b_7570]);
        e.call(0x0070_4690, &args![5u32]);
        assert_eq!(logged(&e, 0x007b_7570), vec![vec![5]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_4690, &args![5u32]);
        assert!(logged(&e, 0x007b_7570).is_empty());
    }

    /// Sets the flag byte that makes a menu id "visible".
    fn make_visible(e: &mut Engine, id: i32) {
        e.mem.set_u8(MENU_FLAG_TABLE + id as u32, 1);
    }

    /// The Pipboy refresh the show paths end with.
    fn pipboy_tail(tab: u32, query_target: Option<u32>) -> Vec<(u32, Vec<u32>)> {
        let mut tail = vec![
            (PIPBOY_REFRESH, vec![0x7000]),
            (PIPBOY_SELECT_TAB, vec![0x7000, tab]),
        ];
        if let Some(target) = query_target {
            tail.push((target, vec![0]));
        }
        tail
    }

    fn set_trait(id: i32) -> (u32, Vec<u32>) {
        (TILE_SET_VALUE, vec![0x6000, TRAIT_CURRENT_MENU, id as u32])
    }

    #[test]
    fn set_map_menu_visible_shows_the_map_and_fades_out_the_others() {
        let (mut e, _) = menu_world(&[0x3eb, 0x3ea, 0x40b, 0x425], true);
        e.call(0x0070_4170, &args![1u8, 0u8]);
        let mut expected = vec![
            (MENU_START_FADE_OUT, vec![menu_of(0x3eb)]),
            (STATS_TOGGLE_HEALING_MODE, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x40b)]),
            (REPAIR_RESET_MENU, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x425)]),
            (ITEM_MOD_RESET_MENU, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3ea)]),
            (0x0079_6b90, vec![]),
            set_trait(0x3ff),
            (MENU_START_FADE_IN, vec![0x3e01]),
            (MAP_MENU_UPDATE, vec![]),
        ];
        expected.extend(pipboy_tail(3, Some(0x0079_c340)));
        assert_eq!(menu_trace(&e), expected);
        // The pipboy query false: no `0079c340`.
        let (mut e, _) = menu_world(&[0x3ff], false);
        e.call(0x0070_4170, &args![1u8, 0u8]);
        let log = menu_trace(&e);
        assert_eq!(log.last().unwrap().0, PIPBOY_SELECT_TAB);
        assert!(!log.iter().any(|(a, _)| *a == 0x0079_6b90));
        // Not ready: nothing.
        let (mut e, manager2) = menu_world(&[0x3ff], true);
        e.mem.set_u8(manager2, 0);
        e.call(0x0070_4170, &args![1u8, 0u8]);
        assert!(menu_trace(&e).is_empty());
    }

    #[test]
    fn set_map_menu_visible_hides_a_visible_map() {
        let (mut e, _) = menu_world(&[0x3ff], true);
        make_visible(&mut e, 0x3ff);
        // Slot 0x14 of the map's menu.
        e.register(0x00c0_0014, |_, _| Ret::default());
        let mut slots = vec![0u32; 6];
        slots[5] = 0x00c0_0014;
        e.put_vtable(0x0140_0100, &slots);
        // `Tile::GetMenu` answers an object with that vtable.
        let menu = e.mem.alloc(8);
        e.mem.set_u32(menu, 0x0140_0100);
        e.register_double(TILE_GET_MENU, move |_, _| Ret {
            eax: menu,
            ..Ret::default()
        });
        e.call(0x0070_4170, &args![0u8, 0u8]);
        assert_eq!(
            trace(&e, &[MENU_START_FADE_OUT, 0x00c0_0014, PIPBOY_REFRESH]),
            vec![
                (MENU_START_FADE_OUT, vec![menu]),
                (0x00c0_0014, vec![menu, 0, 0])
            ]
        );
        // Hiding a map that is not visible does nothing.
        let (mut e, _) = menu_world(&[0x3ff], true);
        e.call(0x0070_4170, &args![0u8, 0u8]);
        assert!(menu_trace(&e).is_empty());
        // The state bits are checked when asked: visible flag, state 0.
        let (mut e, _) = menu_world(&[0x3ff], true);
        make_visible(&mut e, 0x3ff);
        e.register(MENU_STATE, |_, _| Ret::default());
        e.call(0x0070_4170, &args![1u8, 1u8]);
        // State 0 has none of the bits 0xb: not visible, so it is shown.
        assert!(menu_trace(&e).iter().any(|(a, _)| *a == MAP_MENU_UPDATE));
    }

    #[test]
    fn fn_007044c0_shows_the_repair_menu() {
        let (mut e, _) = menu_world(&[0x3eb, 0x3ea, 0x425, 0x3ff], true);
        e.call(0x0070_44c0, &args![1u8, 0u8, 1u8]);
        let mut expected = vec![
            (0x007b_5520, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3eb)]),
            (STATS_TOGGLE_HEALING_MODE, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3ff)]),
            (MENU_START_FADE_OUT, vec![menu_of(0x425)]),
            (MENU_START_FADE_IN, vec![0x3b01]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3ea)]),
            set_trait(0x40b),
        ];
        expected.extend(pipboy_tail(2, Some(0x0078_2470)));
        assert_eq!(menu_trace(&e), expected);
        // Without the Pipboy refresh flag and with the query false, and the
        // menu already visible: no fades, only the trait.
        let (mut e, _) = menu_world(&[0x40b, 0x3eb], false);
        make_visible(&mut e, 0x40b);
        e.call(0x0070_44c0, &args![1u8, 0u8, 0u8]);
        assert_eq!(menu_trace(&e), vec![set_trait(0x40b)]);
    }

    #[test]
    fn fn_007044c0_hides_a_visible_repair_menu() {
        let (mut e, _) = menu_world(&[0x40b], true);
        make_visible(&mut e, 0x40b);
        e.call(0x0070_44c0, &args![0u8, 0u8, 1u8]);
        assert_eq!(
            menu_trace(&e),
            vec![(MENU_START_FADE_OUT, vec![menu_of(0x40b)])]
        );
        let (mut e, manager) = menu_world(&[0x40b], true);
        e.call(0x0070_44c0, &args![0u8, 0u8, 1u8]);
        assert!(menu_trace(&e).is_empty());
        e.mem.set_u8(manager, 0);
        e.call(0x0070_44c0, &args![1u8, 0u8, 1u8]);
        assert!(menu_trace(&e).is_empty());
    }

    #[test]
    fn fn_007046f0_shows_the_item_mod_menu() {
        let (mut e, _) = menu_world(&[0x3eb, 0x3ea, 0x40b, 0x3ff], true);
        e.call(0x0070_46f0, &args![1u8, 0u8, 1u8]);
        let mut expected = vec![
            (0x0078_3440, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3eb)]),
            (STATS_TOGGLE_HEALING_MODE, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3ff)]),
            (MENU_START_FADE_OUT, vec![menu_of(0x40b)]),
            (MENU_START_FADE_IN, vec![0x3c01]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3ea)]),
            set_trait(0x425),
        ];
        expected.extend(pipboy_tail(2, Some(0x0078_2470)));
        assert_eq!(menu_trace(&e), expected);
        // Hiding a visible one fades it out.
        let (mut e, _) = menu_world(&[0x425], false);
        make_visible(&mut e, 0x425);
        e.call(0x0070_46f0, &args![0u8, 0u8, 0u8]);
        assert_eq!(
            menu_trace(&e),
            vec![(MENU_START_FADE_OUT, vec![menu_of(0x425)])]
        );
    }

    #[test]
    fn set_inventory_menu_visible_shows_the_inventory() {
        let (mut e, _) = menu_world(&[0x3eb, 0x40b, 0x425, 0x3ff], true);
        e.call(0x0070_48f0, &args![1u8, 0u8, 1u8]);
        let mut expected = vec![
            (0x0077_fc10, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3eb)]),
            (STATS_TOGGLE_HEALING_MODE, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3ff)]),
            (MENU_START_FADE_OUT, vec![menu_of(0x40b)]),
            (REPAIR_RESET_MENU, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x425)]),
            (ITEM_MOD_RESET_MENU, vec![]),
            (MENU_START_FADE_IN, vec![0x3d01]),
            (INVENTORY_UPDATE_MENU, vec![]),
            set_trait(0x3ea),
        ];
        expected.extend(pipboy_tail(2, Some(0x0078_2470)));
        assert_eq!(menu_trace(&e), expected);
    }

    #[test]
    fn set_inventory_menu_visible_hides_a_visible_inventory() {
        let (mut e, _) = menu_world(&[0x3ea], true);
        make_visible(&mut e, 0x3ea);
        e.call(0x0070_48f0, &args![0u8, 0u8, 1u8]);
        assert_eq!(
            menu_trace(&e),
            vec![
                (MENU_START_FADE_OUT, vec![menu_of(0x3ea)]),
                (INVENTORY_SAVE_PAGE_STATE, vec![]),
            ]
        );
        // Visible already and shown again: refreshed, not faded.
        let (mut e, _) = menu_world(&[0x3ea], false);
        make_visible(&mut e, 0x3ea);
        e.call(0x0070_48f0, &args![1u8, 0u8, 0u8]);
        assert_eq!(
            menu_trace(&e),
            vec![(INVENTORY_UPDATE_MENU, vec![]), set_trait(0x3ea)]
        );
    }

    #[test]
    fn get_inventory_menu_visible_reads_the_flag_table() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_4ad0, &[]).u8(), 0);
        make_visible(&mut e, 0x3ea);
        assert_eq!(e.call(0x0070_4ad0, &[]).u8(), 1);
    }

    #[test]
    fn fn_00704af0_refreshes_the_menu_that_is_showing() {
        // The inventory.
        let (mut e, _) = menu_world(&[0x3eb], false);
        make_visible(&mut e, 0x3ea);
        e.call(0x0070_4af0, &[]);
        assert_eq!(menu_trace(&e), vec![(INVENTORY_UPDATE_MENU, vec![])]);
        // The stats menu (`00704df0` true): the healing options of its menu.
        let (mut e, _) = menu_world(&[0x3eb], false);
        stub!(e, 0x0070_4df0, 1);
        e.call(0x0070_4af0, &[]);
        assert_eq!(
            menu_trace(&e),
            vec![(STATS_UPDATE_HEALING_OPTIONS, vec![menu_of(0x3eb), 4])]
        );
        // Menus 0x41d, 0x435 and 0x3f0 in that order.
        for (id, callee, args_) in [
            (0x41d, 0x0073_0690u32, vec![1u32]),
            (0x435, 0x0072_8a30, vec![1]),
            (0x3f0, 0x0070_4bc0, vec![]),
        ] {
            let (mut e, _) = menu_world(&[], false);
            make_visible(&mut e, id);
            e.call(0x0070_4af0, &[]);
            assert_eq!(menu_trace(&e), vec![(callee, args_)]);
        }
        // Nothing showing, or not ready: nothing.
        let (mut e, manager) = menu_world(&[], false);
        e.call(0x0070_4af0, &[]);
        assert!(menu_trace(&e).is_empty());
        make_visible(&mut e, 0x3ea);
        e.mem.set_u8(manager, 0);
        e.call(0x0070_4af0, &[]);
        assert!(menu_trace(&e).is_empty());
    }

    // ---- 00704bc0 .. 00705a60 ----

    /// Stands for the manager in the expected arguments of [`guarded`].
    const MANAGER_MARK: u32 = u32::MAX;

    /// Runs `wrapper(args)` three times: manager ready, manager flag clear
    /// and no manager. `callee` must be called once, with `expected`
    /// (`MANAGER_MARK` replaced by the manager), only in the first. Returns
    /// the three `eax` results.
    fn guarded(wrapper: u32, args: &[u32], callee: u32, expected: &[u32]) -> [u32; 3] {
        let (mut e, manager) = ui_engine();
        e.register(callee, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        let ready = e.call(wrapper, args).u32();
        let want: Vec<u32> = expected
            .iter()
            .map(|w| if *w == MANAGER_MARK { manager } else { *w })
            .collect();
        assert_eq!(logged(&e, callee), vec![want]);

        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        let not_ready = e.call(wrapper, args).u32();
        assert!(logged(&e, callee).is_empty());

        let mut e = engine_without_manager();
        e.register(callee, |_, _| Ret::default());
        let none = e.call(wrapper, args).u32();
        assert!(logged(&e, callee).is_empty());
        [ready, not_ready, none]
    }

    /// A world with one menu of class `class`, a real object with a
    /// destructor slot, and a cast double that succeeds only for
    /// `cast_target`. The slot-0 double is at `0x03001000`.
    fn menu_object_world(class: i32, cast_target: u32) -> (Engine, u32, u32) {
        let (mut e, manager) = ui_engine();
        let menu = e.mem.alloc(8);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(menu, 0x0300_0000);
        e.register(0x0300_1000, |_, _| Ret::default());
        e.register_double(GET_MENU_BY_CLASS, move |_, a| Ret {
            eax: if a[0] as i32 == class { 0x5000 } else { 0 },
            ..Ret::default()
        });
        e.register(TILE_GET_MENU, |e, a| Ret {
            eax: if a[0] == 0x5000 {
                e.mem.u32(0x0126_f200)
            } else {
                0
            },
            ..Ret::default()
        });
        e.mem.set_u32(0x0126_f200, menu);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| Ret {
            eax: if a[2] == MENU_TYPE && a[3] == cast_target {
                a[0]
            } else {
                0
            },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, manager, menu)
    }

    #[test]
    fn fn_00704bc0_updates_the_container_lists() {
        let (mut e, _) = ui_engine();
        e.register(CONTAINER_UPDATE_LISTS, |_, _| Ret::default());
        e.call(0x0070_4bc0, &[]);
        assert!(logged(&e, CONTAINER_UPDATE_LISTS).is_empty());
        e.mem.set_u32(CONTAINER_MENU_GLOBAL, 0x7777);
        e.call(0x0070_4bc0, &[]);
        assert_eq!(logged(&e, CONTAINER_UPDATE_LISTS), vec![vec![0x7777, 0]]);
    }

    #[test]
    fn fn_00704be0_creates_the_stats_menu_when_ready() {
        assert_eq!(
            guarded(0x0070_4be0, &[], STATS_MENU_CREATE, &[]),
            [0x1234, 0, 0]
        );
    }

    /// The Pipboy tail and the knob call of the stats menu.
    fn stats_trace(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        let mut addrs = MENU_ADDRESSES.to_vec();
        addrs.extend([STATS_MENU_CREATE, STATS_UPDATE_ALL, STATS_SET_KNOB_POSITION]);
        trace(e, &addrs)
    }

    #[test]
    fn set_stats_menu_visible_shows_the_stats_menu_and_fades_out_the_others() {
        let (mut e, _) = menu_world(&[0x3eb, 0x3ea, 0x40b, 0x425, 0x3ff], true);
        ignore(&mut e, &[STATS_UPDATE_ALL, STATS_SET_KNOB_POSITION]);
        e.call(0x0070_4c10, &args![1u8, 0u8]);
        let mut expected = vec![
            (MENU_START_FADE_OUT, vec![menu_of(0x3ea)]),
            (MENU_START_FADE_OUT, vec![menu_of(0x40b)]),
            (REPAIR_RESET_MENU, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x425)]),
            (ITEM_MOD_RESET_MENU, vec![]),
            (MENU_START_FADE_OUT, vec![menu_of(0x3ff)]),
            set_trait(0x3eb),
            (STATS_UPDATE_ALL, vec![menu_of(0x3eb)]),
            (MENU_START_FADE_IN, vec![menu_of(0x3eb)]),
        ];
        expected.extend(pipboy_tail(1, Some(STATS_SET_KNOB_POSITION)));
        assert_eq!(stats_trace(&e), expected);
    }

    #[test]
    fn set_stats_menu_visible_creates_the_menu_when_missing() {
        let (mut e, _) = menu_world(&[], false);
        ignore(&mut e, &[STATS_UPDATE_ALL, STATS_SET_KNOB_POSITION]);
        e.register(STATS_MENU_CREATE, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        e.call(0x0070_4c10, &args![1u8, 0u8]);
        let mut expected = vec![
            (STATS_MENU_CREATE, vec![]),
            set_trait(0x3eb),
            (STATS_UPDATE_ALL, vec![0x5001]),
            (MENU_START_FADE_IN, vec![0x5001]),
        ];
        expected.extend(pipboy_tail(1, None));
        assert_eq!(stats_trace(&e), expected);
    }

    #[test]
    fn set_stats_menu_visible_visible_and_hidden_cases() {
        // Visible already and shown again: only the Pipboy tail.
        let (mut e, _) = menu_world(&[0x3eb, 0x3ea], false);
        ignore(&mut e, &[STATS_UPDATE_ALL, STATS_SET_KNOB_POSITION]);
        make_visible(&mut e, 0x3eb);
        e.call(0x0070_4c10, &args![1u8, 0u8]);
        assert_eq!(stats_trace(&e), pipboy_tail(1, None));
        // Hiding a visible one fades it out and does nothing else.
        let (mut e, _) = menu_world(&[0x3eb, 0x3ea], false);
        ignore(&mut e, &[STATS_UPDATE_ALL, STATS_SET_KNOB_POSITION]);
        make_visible(&mut e, 0x3eb);
        e.call(0x0070_4c10, &args![0u8, 0u8]);
        assert_eq!(
            stats_trace(&e),
            vec![(MENU_START_FADE_OUT, vec![menu_of(0x3eb)])]
        );
        // Hiding a menu that is not visible: nothing.
        let (mut e, _) = menu_world(&[0x3eb], false);
        e.call(0x0070_4c10, &args![0u8, 0u8]);
        assert!(stats_trace(&e).is_empty());
        // Not ready: nothing.
        let (mut e, manager) = menu_world(&[0x3eb], false);
        e.mem.set_u8(manager, 0);
        e.call(0x0070_4c10, &args![1u8, 0u8]);
        assert!(stats_trace(&e).is_empty());
    }

    #[test]
    fn fn_00704df0_reads_the_stats_menu_flag() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_4df0, &[]).u8(), 0);
        make_visible(&mut e, 0x3eb);
        assert_eq!(e.call(0x0070_4df0, &[]).u8(), 1);
    }

    #[test]
    fn update_actor_value_updates_all_or_one_value() {
        let (mut e, _, menu) = menu_object_world(0x3eb, STATS_MENU_TYPE);
        ignore(
            &mut e,
            &[STATS_UPDATE_ALL_ACTOR_VALUES, STATS_UPDATE_ACTOR_VALUE],
        );
        stub!(e, MENU_STATE, 1);
        // Not visible: nothing.
        e.call(0x0070_4e10, &args![5u32]);
        assert!(logged(&e, STATS_UPDATE_ACTOR_VALUE).is_empty());
        make_visible(&mut e, 0x3eb);
        e.call(0x0070_4e10, &args![0x4du32]);
        e.call(0x0070_4e10, &args![5u32]);
        assert_eq!(logged(&e, STATS_UPDATE_ALL_ACTOR_VALUES), vec![vec![menu]]);
        assert_eq!(logged(&e, STATS_UPDATE_ACTOR_VALUE), vec![vec![menu, 5]]);
        // The visible state bits do not match the mask: nothing.
        stub!(e, MENU_STATE, 4);
        e.call_log = Some(vec![]);
        e.call(0x0070_4e10, &args![5u32]);
        assert!(logged(&e, STATS_UPDATE_ACTOR_VALUE).is_empty());
    }

    #[test]
    fn update_actor_value_needs_a_stats_menu_object() {
        let (mut e, _, _) = menu_object_world(0x3eb, DIALOG_MENU_TYPE);
        ignore(&mut e, &[STATS_UPDATE_ACTOR_VALUE]);
        stub!(e, MENU_STATE, 1);
        make_visible(&mut e, 0x3eb);
        e.call(0x0070_4e10, &args![5u32]);
        assert!(logged(&e, STATS_UPDATE_ACTOR_VALUE).is_empty());
        let mut e = engine_without_manager();
        e.call(0x0070_4e10, &args![5u32]);
    }

    /// Calls `CreateContainerMenu(reference, 1)` with the answers given:
    /// slot `0x100`, the owner, `IsAnOwner`. Returns the mode that reached
    /// `ContainerMenu::Create`.
    fn container_mode(slot: u32, owner: u32, is_owner: bool, mode: u32) -> Vec<Vec<u32>> {
        let (mut e, _) = ui_engine();
        let mut slots = vec![0u32; 0x41];
        slots[0x40] = 0x0300_2000;
        e.put_vtable(0x0300_0000, &slots);
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x0300_0000);
        e.mem.set_u32(OWNER_GLOBAL, 0x4242);
        e.register_double(0x0300_2000, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        e.register_double(REFR_GET_OWNER, move |_, _| Ret {
            eax: owner,
            ..Ret::default()
        });
        e.register_double(REFR_IS_AN_OWNER, move |_, _| Ret {
            eax: is_owner as u32,
            ..Ret::default()
        });
        stub!(e, CONTAINER_MENU_CREATE, 0x99);
        assert_eq!(e.call(0x0070_4ed0, &args![reference, mode]).u32(), 0x99);
        let owner_calls = logged(&e, REFR_IS_AN_OWNER);
        assert!(owner_calls.iter().all(|c| *c == vec![reference, 0x4242, 1]));
        logged(&e, CONTAINER_MENU_CREATE)
            .into_iter()
            .map(|c| c[1..].to_vec())
            .collect()
    }

    #[test]
    fn create_container_menu_downgrades_mode_one_for_foreign_owners() {
        assert_eq!(container_mode(0, 1, false, 1), vec![vec![2]]);
        assert_eq!(container_mode(1, 1, false, 1), vec![vec![1]]);
        assert_eq!(container_mode(0, 0, false, 1), vec![vec![1]]);
        assert_eq!(container_mode(0, 1, true, 1), vec![vec![1]]);
        assert_eq!(container_mode(0, 1, false, 3), vec![vec![3]]);
    }

    #[test]
    fn create_container_menu_needs_a_ready_manager() {
        let mut e = engine_without_manager();
        stub!(e, CONTAINER_MENU_CREATE, 0x99);
        assert_eq!(e.call(0x0070_4ed0, &args![0x1000u32, 2u32]).u32(), 0);
        assert!(logged(&e, CONTAINER_MENU_CREATE).is_empty());
    }

    #[test]
    fn container_barter_and_recipe_wrappers_forward_when_ready() {
        guarded(0x0070_4f50, &[], CONTAINER_MENU_CLOSE, &[]);
        guarded(0x0070_4f80, &[7, 8], BARTER_MENU_CREATE, &[7, 8]);
        guarded(0x0070_4fc0, &[9, 10], RECIPE_MENU_CREATE, &[9, 10]);
    }

    #[test]
    fn menu_existence_tests() {
        let (mut e, _) = menu_world(&[0x435], false);
        assert!(e.call(0x0070_5000, &[]).bool());
        assert!(!e.call(0x0070_5020, &[]).bool());
        let (mut e, _) = menu_world(&[0x41d], false);
        assert!(!e.call(0x0070_5000, &[]).bool());
        assert!(e.call(0x0070_5020, &[]).bool());
    }

    #[test]
    fn container_menu_field_reads_the_word_at_0x80() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_5050, &[]).u32(), 0);
        assert_eq!(e.call(0x0070_5040, &[]).u32(), 0);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x80, 0xabcd);
        e.mem.set_u32(CONTAINER_MENU_GLOBAL, object);
        assert_eq!(e.call(0x0070_5050, &[]).u32(), 0xabcd);
        assert_eq!(e.call(0x0070_5040, &[]).u32(), 0xabcd);
    }

    #[test]
    fn create_dialog_menu_resets_the_time_multiplier_first() {
        let (mut e, manager) = ui_engine();
        e.register(SET_GLOBAL_TIME_MULTIPLIER, |_, _| Ret::default());
        stub!(e, DIALOG_MENU_CREATE, 0x55);
        assert_eq!(e.call(0x0070_5070, &args![1u32, 2u32, 3u32]).u32(), 0x55);
        assert_eq!(
            logged(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![GLOBAL_TIMER, 0x3f80_0000, 1]]
        );
        assert_eq!(logged(&e, DIALOG_MENU_CREATE), vec![vec![1, 2, 3]]);
        // Not ready: the multiplier is still set, the menu not created.
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_5070, &args![1u32, 2u32, 3u32]).u32(), 0);
        assert_eq!(logged(&e, SET_GLOBAL_TIME_MULTIPLIER).len(), 1);
        assert!(logged(&e, DIALOG_MENU_CREATE).is_empty());
    }

    #[test]
    fn in_dialog_reads_the_flag_when_ready() {
        let (mut e, manager) = ui_engine();
        assert_eq!(e.call(0x0070_5100, &[]).u8(), 0);
        e.mem.set_u8(IN_DIALOG_FLAG, 1);
        assert_eq!(e.call(0x0070_5100, &[]).u8(), 1);
        assert_eq!(e.call(0x0070_50d0, &[]).u8(), 1);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_50d0, &[]).u8(), 0);
    }

    #[test]
    fn set_dialog_pause_needs_a_dialog_menu() {
        let (mut e, _, menu) = menu_object_world(0x3f1, DIALOG_MENU_TYPE);
        stub!(e, DIALOG_SET_PAUSE, 0);
        e.call(0x0070_5110, &args![1u8]);
        assert_eq!(logged(&e, DIALOG_SET_PAUSE), vec![vec![menu, 1]]);
        let (mut e, _, _) = menu_object_world(0x3f1, STATS_MENU_TYPE);
        stub!(e, DIALOG_SET_PAUSE, 0);
        e.call(0x0070_5110, &args![1u8]);
        assert!(logged(&e, DIALOG_SET_PAUSE).is_empty());
        let (mut e, _, _) = menu_object_world(0x3eb, DIALOG_MENU_TYPE);
        stub!(e, DIALOG_SET_PAUSE, 0);
        e.call(0x0070_5110, &args![1u8]);
        assert!(logged(&e, DIALOG_SET_PAUSE).is_empty());
    }

    #[test]
    fn in_dialog_with_answers_the_dialog_menu_getter() {
        let (mut e, _, menu) = menu_object_world(0x3f1, DIALOG_MENU_TYPE);
        stub!(e, DIALOG_MENU_GETTER, 0x6677);
        assert_eq!(e.call(0x0070_5190, &[]).u32(), 0x6677);
        assert_eq!(logged(&e, DIALOG_MENU_GETTER), vec![vec![menu]]);
        let (mut e, _, _) = menu_object_world(0x3f1, STATS_MENU_TYPE);
        stub!(e, DIALOG_MENU_GETTER, 0x6677);
        assert_eq!(e.call(0x0070_5190, &[]).u32(), 0);
        assert!(logged(&e, DIALOG_MENU_GETTER).is_empty());
    }

    /// Doubles for the subtitle functions: the guard, the 12-byte copy
    /// (word, byte, word; the padding stays zero) and its destructor.
    fn subtitle_world() -> (Engine, u32) {
        let (mut e, manager) = ui_engine();
        ignore(
            &mut e,
            &[
                SCOPE_GUARD_CTOR,
                SCOPE_GUARD_DTOR,
                SUBTITLE_VALUE_DTOR,
                HUD_SHOW_SUBTITLE,
                HUD_SHOW_SUBTITLE_TIMED,
                HUD_HIDE_SUBTITLE,
            ],
        );
        e.register(SUBTITLE_VALUE_COPY, |e, a| {
            let (word, byte, last) = (e.mem.u32(a[1]), e.mem.u8(a[1] + 4), e.mem.u32(a[1] + 8));
            e.mem.set_u32(a[0], word);
            e.mem.set_u8(a[0] + 4, byte);
            e.mem.set_u32(a[0] + 8, last);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        (e, manager)
    }

    #[test]
    fn show_subtitle_copies_the_value_and_always_returns_zero() {
        let (mut e, _) = subtitle_world();
        e.mem.set_u32(HUD_MAIN_MENU, 0x8000);
        let args_ = args![
            1u32,
            0x1111_1111u32,
            0x2222_2244u32,
            0x3333_3333u32,
            4u32,
            5u32,
            6u32,
            7u32,
            8u8
        ];
        assert_eq!(e.call(0x0070_5210, &args_).u32() & 0xff, 0);
        assert_eq!(
            logged(&e, HUD_SHOW_SUBTITLE),
            vec![vec![
                0x8000,
                1,
                0x1111_1111,
                0x44,
                0x3333_3333,
                4,
                5,
                6,
                7,
                8
            ]]
        );
        let guard = logged(&e, SCOPE_GUARD_CTOR);
        assert_eq!(guard[0][1..], [0xd, 1, INTERFACE_CPP_PATH, 0x8ec]);
        assert_eq!(logged(&e, SCOPE_GUARD_DTOR).len(), 1);
        let dtor = logged(&e, SUBTITLE_VALUE_DTOR);
        assert_eq!(dtor.len(), 1);
        // No HUD main menu: guarded and destroyed, nothing shown.
        e.mem.set_u32(HUD_MAIN_MENU, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_5210, &args_).u32() & 0xff, 0);
        assert!(logged(&e, HUD_SHOW_SUBTITLE).is_empty());
        assert_eq!(logged(&e, SCOPE_GUARD_DTOR).len(), 1);
        assert_eq!(logged(&e, SUBTITLE_VALUE_DTOR).len(), 1);
    }

    #[test]
    fn show_subtitle_needs_a_ready_manager() {
        let (mut e, manager) = subtitle_world();
        e.mem.set_u32(HUD_MAIN_MENU, 0x8000);
        e.mem.set_u8(manager, 0);
        e.call(
            0x0070_5210,
            &args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u8],
        );
        assert!(logged(&e, HUD_SHOW_SUBTITLE).is_empty());
    }

    #[test]
    fn fn_007052f0_passes_six_words_to_the_hud() {
        let (mut e, manager) = subtitle_world();
        e.mem.set_u32(HUD_MAIN_MENU, 0x8000);
        let call = args![1u32, 2u32, 3u32, 4u32, 2.5f32, 9u8];
        assert_eq!(e.call(0x0070_52f0, &call).u32() & 0xff, 0);
        assert_eq!(
            logged(&e, HUD_SHOW_SUBTITLE_TIMED),
            vec![vec![0x8000, 1, 2, 3, 4, 2.5f32.to_bits(), 9]]
        );
        assert_eq!(
            logged(&e, SCOPE_GUARD_CTOR)[0][1..],
            [0xd, 1, INTERFACE_CPP_PATH, 0x901]
        );
        assert_eq!(logged(&e, SCOPE_GUARD_DTOR).len(), 1);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_52f0, &call);
        assert!(logged(&e, HUD_SHOW_SUBTITLE_TIMED).is_empty());
        assert_eq!(logged(&e, SCOPE_GUARD_DTOR).len(), 1);
        e.mem.set_u8(manager, 1);
        e.mem.set_u32(HUD_MAIN_MENU, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_52f0, &call);
        assert!(logged(&e, HUD_SHOW_SUBTITLE_TIMED).is_empty());
    }

    #[test]
    fn fn_007053b0_needs_the_hud_main_menu() {
        let (mut e, manager) = subtitle_world();
        e.call(0x0070_53b0, &[]);
        assert!(logged(&e, HUD_HIDE_SUBTITLE).is_empty());
        e.mem.set_u32(HUD_MAIN_MENU, 0x8000);
        e.call(0x0070_53b0, &[]);
        assert_eq!(logged(&e, HUD_HIDE_SUBTITLE), vec![vec![0x8000]]);
        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_53b0, &[]);
        assert!(logged(&e, HUD_HIDE_SUBTITLE).is_empty());
    }

    #[test]
    fn start_menu_wrappers_pass_their_constants() {
        guarded(0x0070_53f0, &[], START_MENU_CREATE, &[0, 0]);
        guarded(0x0070_5800, &[], START_MENU_CREATE, &[1, 1]);
    }

    /// Doubles for `SetRegionText`: the request is recorded in memory
    /// (`0126f300`: region word, `+4` flag, `+8` first text bytes).
    fn region_world() -> (Engine, u32) {
        let (mut e, manager) = ui_engine();
        e.register(FIRST_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(STRING_DTOR, |e, a| {
            let second = e.mem.u32(a[0] + 4);
            e.mem.set_u32(0x0126_f30c, second);
            Ret::default()
        });
        e.register(STRING_COPY_BOUNDED, |e, a| {
            let text = e.mem.cstr(a[2]);
            for (i, byte) in text.iter().enumerate() {
                e.mem.set_u8(a[0] + i as u32, *byte);
            }
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            Ret::default()
        });
        e.register(HUD_SET_REGION_TEXT, |e, a| {
            let region = e.mem.u32(a[0]);
            let flag = e.mem.u8(a[0] + 0x108);
            let text = e.mem.u32(a[0] + 4);
            e.mem.set_u32(0x0126_f300, region);
            e.mem.set_u8(0x0126_f304, flag);
            e.mem.set_u32(0x0126_f308, text);
            Ret::default()
        });
        (e, manager)
    }

    #[test]
    fn fn_00705420_builds_the_region_request() {
        let (mut e, _) = region_world();
        let text = e.mem.alloc(16);
        for (i, b) in b"Vegas\0".iter().enumerate() {
            e.mem.set_u8(text + i as u32, *b);
        }
        e.call(0x0070_5420, &args![0x77u32, text, 5u32, 1u8]);
        assert_eq!(e.mem.u32(0x0126_f300), 0x77);
        assert_eq!(e.mem.u8(0x0126_f304), 1);
        assert_eq!(e.mem.u32(0x0126_f308), u32::from_le_bytes(*b"Vega"));
        assert_eq!(logged(&e, STRING_COPY_BOUNDED)[0][1..], [0x104, text]);
        assert_eq!(logged(&e, STRING_DTOR).len(), 1);
        // The string argument handed to the destructor held the words.
        assert_eq!(e.mem.u32(0x0126_f30c), 5);
    }

    #[test]
    fn fn_00705420_uses_an_empty_text_for_a_null_string() {
        let (mut e, _) = region_world();
        e.call(0x0070_5420, &args![0x78u32, 0u32, 0u32, 0u8]);
        assert_eq!(e.mem.u32(0x0126_f300), 0x78);
        assert_eq!(e.mem.u32(0x0126_f308) & 0xff, 0);
        assert!(logged(&e, STRING_COPY_BOUNDED).is_empty());
        assert_eq!(logged(&e, STRING_DTOR).len(), 1);
    }

    #[test]
    fn fn_00705420_still_destroys_the_string_when_not_ready() {
        let (mut e, manager) = region_world();
        e.mem.set_u8(manager, 0);
        e.call(0x0070_5420, &args![0x79u32, 0u32, 0u32, 0u8]);
        assert!(logged(&e, HUD_SET_REGION_TEXT).is_empty());
        assert_eq!(logged(&e, STRING_DTOR).len(), 1);
    }

    #[test]
    fn create_sleep_menu_destroys_an_existing_menu_first() {
        let (mut e, _, menu) = menu_object_world(0x3f4, SLEEP_WAIT_MENU_TYPE);
        stub!(e, MANAGER_MODE_IS_ZERO, 1);
        stub!(e, SLEEP_MENU_CREATE, 0);
        e.call(0x0070_54f0, &args![1u8]);
        assert_eq!(logged(&e, 0x0300_1000), vec![vec![menu, 1]]);
        assert_eq!(logged(&e, SLEEP_MENU_CREATE), vec![vec![1]]);
        // No sleep menu yet: created without a destroy.
        let (mut e, _, _) = menu_object_world(0x3eb, SLEEP_WAIT_MENU_TYPE);
        stub!(e, MANAGER_MODE_IS_ZERO, 1);
        stub!(e, SLEEP_MENU_CREATE, 0);
        e.call(0x0070_54f0, &args![0u8]);
        assert!(logged(&e, 0x0300_1000).is_empty());
        assert_eq!(logged(&e, SLEEP_MENU_CREATE), vec![vec![0]]);
        // The mode test fails: nothing.
        let (mut e, _, _) = menu_object_world(0x3f4, SLEEP_WAIT_MENU_TYPE);
        stub!(e, MANAGER_MODE_IS_ZERO, 0);
        stub!(e, SLEEP_MENU_CREATE, 0);
        e.call(0x0070_54f0, &args![1u8]);
        assert!(logged(&e, SLEEP_MENU_CREATE).is_empty());
        assert!(logged(&e, 0x0300_1000).is_empty());
    }

    #[test]
    fn close_sleep_menu_needs_a_sleep_wait_menu() {
        let (mut e, _, _) = menu_object_world(0x3f4, SLEEP_WAIT_MENU_TYPE);
        stub!(e, SLEEP_MENU_CLOSE, 0);
        e.call(0x0070_55c0, &[]);
        assert_eq!(logged(&e, SLEEP_MENU_CLOSE), vec![Vec::<u32>::new()]);
        let (mut e, _, _) = menu_object_world(0x3f4, VATS_MENU_TYPE);
        stub!(e, SLEEP_MENU_CLOSE, 0);
        e.call(0x0070_55c0, &[]);
        assert!(logged(&e, SLEEP_MENU_CLOSE).is_empty());
    }

    #[test]
    fn create_vats_menu_destroys_an_existing_menu_first() {
        let (mut e, _, menu) = menu_object_world(0x420, VATS_MENU_TYPE);
        stub!(e, VATS_MENU_CREATE, 0);
        e.call(0x0070_5640, &[]);
        assert_eq!(logged(&e, 0x0300_1000), vec![vec![menu, 1]]);
        assert_eq!(logged(&e, VATS_MENU_CREATE).len(), 1);
        let (mut e, _, _) = menu_object_world(0x3eb, VATS_MENU_TYPE);
        stub!(e, VATS_MENU_CREATE, 0);
        e.call(0x0070_5640, &[]);
        assert!(logged(&e, 0x0300_1000).is_empty());
        assert_eq!(logged(&e, VATS_MENU_CREATE).len(), 1);
        let mut e = engine_without_manager();
        stub!(e, VATS_MENU_CREATE, 0);
        e.call(0x0070_5640, &[]);
        assert!(logged(&e, VATS_MENU_CREATE).is_empty());
    }

    #[test]
    fn close_vats_menu_needs_a_vats_menu() {
        let (mut e, _, _) = menu_object_world(0x420, VATS_MENU_TYPE);
        stub!(e, VATS_MENU_CLOSE, 0);
        e.call(0x0070_5780, &[]);
        assert_eq!(logged(&e, VATS_MENU_CLOSE).len(), 1);
        let (mut e, _, _) = menu_object_world(0x420, SLEEP_WAIT_MENU_TYPE);
        stub!(e, VATS_MENU_CLOSE, 0);
        e.call(0x0070_5780, &[]);
        assert!(logged(&e, VATS_MENU_CLOSE).is_empty());
    }

    #[test]
    fn update_sleeping_asks_the_sleep_wait_menu() {
        assert_eq!(ui_engine().0.call(0x0070_5770, &[]).u32(), 0x3f4);
        let (mut e, _, menu) = menu_object_world(0x3f4, SLEEP_WAIT_MENU_TYPE);
        stub!(e, SLEEP_UPDATE_SLEEPING, 1);
        assert_eq!(e.call(0x0070_56f0, &[]).u8(), 1);
        assert_eq!(logged(&e, SLEEP_UPDATE_SLEEPING), vec![vec![menu]]);
        stub!(e, SLEEP_UPDATE_SLEEPING, 0);
        assert_eq!(e.call(0x0070_56f0, &[]).u8(), 0);
        // No such menu.
        let (mut e, _, _) = menu_object_world(0x3eb, SLEEP_WAIT_MENU_TYPE);
        stub!(e, SLEEP_UPDATE_SLEEPING, 1);
        assert_eq!(e.call(0x0070_56f0, &[]).u8(), 0);
        assert!(logged(&e, SLEEP_UPDATE_SLEEPING).is_empty());
        // Not ready.
        let (mut e, manager, _) = menu_object_world(0x3f4, SLEEP_WAIT_MENU_TYPE);
        stub!(e, SLEEP_UPDATE_SLEEPING, 1);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_56f0, &[]).u8(), 0);
        assert!(logged(&e, SLEEP_UPDATE_SLEEPING).is_empty());
    }

    #[test]
    fn fn_00705830_passes_three_words() {
        guarded(0x0070_5830, &[3, 4, 1], LOCK_PICK_MENU_CREATE, &[3, 4, 1]);
    }

    /// A world for `CreateRaceSexMenu`: the counter word is at `0126f400`.
    fn race_sex_world(counter: i32) -> Engine {
        let (mut e, _) = ui_engine();
        e.mem.set_u32(0x0126_f400, counter as u32);
        e.register(COUNTER_ACCESSOR, |_, _| Ret {
            eax: 0x0126_f400,
            ..Ret::default()
        });
        stub!(e, DIALOG_SERVICE_START, 0);
        stub!(e, RACE_SEX_MENU_CREATE, 0);
        e
    }

    #[test]
    fn create_race_sex_menu_defers_when_the_counter_is_above_one() {
        let mut e = race_sex_world(2);
        e.call(0x0070_5870, &args![0x42u32]);
        assert_eq!(e.mem.u32(PENDING_MENU_KIND), 5);
        assert_eq!(e.mem.u32(PENDING_MENU_ARGUMENT), 0x42);
        assert!(logged(&e, RACE_SEX_MENU_CREATE).is_empty());
        assert!(logged(&e, DIALOG_SERVICE_START).is_empty());
    }

    #[test]
    fn create_race_sex_menu_creates_now_otherwise() {
        // Counter 1: created at once.
        let mut e = race_sex_world(1);
        e.call(0x0070_5870, &args![0x42u32]);
        assert_eq!(logged(&e, RACE_SEX_MENU_CREATE), vec![vec![0x42]]);
        assert_eq!(e.mem.u32(PENDING_MENU_KIND), 0);
        // Counter negative (signed compare): created at once.
        let mut e = race_sex_world(-3);
        e.call(0x0070_5870, &args![0x43u32]);
        assert_eq!(logged(&e, RACE_SEX_MENU_CREATE), vec![vec![0x43]]);
        // A menu already pending: created at once, pending unchanged.
        let mut e = race_sex_world(5);
        e.mem.set_u32(PENDING_MENU_KIND, 7);
        e.call(0x0070_5870, &args![0x44u32]);
        assert_eq!(logged(&e, RACE_SEX_MENU_CREATE), vec![vec![0x44]]);
        assert_eq!(e.mem.u32(PENDING_MENU_KIND), 7);
    }

    #[test]
    fn create_race_sex_menu_starts_the_dialog_service_first() {
        let mut e = race_sex_world(1);
        e.mem.set_u8(IN_DIALOG_FLAG, 1);
        e.call(0x0070_5870, &args![0x42u32]);
        let calls: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            calls,
            vec![
                0x0070_5870,
                DIALOG_SERVICE_START,
                COUNTER_ACCESSOR,
                RACE_SEX_MENU_CREATE
            ]
        );
    }

    #[test]
    fn fn_007058c0_reads_the_word_at_0xc4() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xc4, 0x1357);
        assert_eq!(e.call(0x0070_58c0, &args![object]).u32(), 0x1357);
    }

    #[test]
    fn manager_forwarders_pass_the_manager() {
        guarded(
            0x0070_58e0,
            &[],
            MANAGER_LAST_MINUTE_UPDATE,
            &[MANAGER_MARK],
        );
        guarded(0x0070_59d0, &[], MANAGER_CALL_A, &[MANAGER_MARK]);
        guarded(0x0070_5a60, &[], MANAGER_CALL_B, &[MANAGER_MARK]);
        assert_eq!(
            guarded(0x0070_5910, &[], MANAGER_GETTER_A, &[MANAGER_MARK]),
            [0x1234, 0, 0]
        );
        assert_eq!(
            guarded(0x0070_5950, &[], MANAGER_GETTER_B, &[MANAGER_MARK]),
            [0x1234, 0, 0]
        );
        assert_eq!(
            guarded(0x0070_5990, &[], MANAGER_PIPBOY, &[MANAGER_MARK]),
            [0x1234, 0, 0]
        );
    }

    #[test]
    fn is_in_pipboy_menu_accepts_modes_two_and_three() {
        for (two, three, want) in [(0, 0, 0), (1, 0, 1), (0, 1, 1), (1, 1, 1)] {
            let (mut e, _) = ui_engine();
            e.register_double(MANAGER_MODE_IS_TWO, move |_, _| Ret {
                eax: two,
                ..Ret::default()
            });
            e.register_double(MANAGER_MODE_IS_THREE, move |_, _| Ret {
                eax: three,
                ..Ret::default()
            });
            assert_eq!(e.call(0x0070_5a00, &[]).u8(), want);
        }
        let mut e = engine_without_manager();
        assert_eq!(e.call(0x0070_5a00, &[]).u8(), 0);
    }

    #[test]
    fn fn_007038d0_answers_the_callees_value() {
        let (mut e, _) = ui_engine();
        e.register(0x0077_32b0, |_, _| 0x9000u32.into_ret());
        assert_eq!(e.call(0x0070_38d0, &[]).u32(), 0x9000);
    }
}
