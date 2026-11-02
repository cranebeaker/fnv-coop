//! `fallout/interface/interface.cpp` (Xbox PDB source unit), part 2: its functions from `00705a90` up to
//! (not including) `00709b50` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::interface`]; anything public there may be used here.
//!
//! This part covers `00705a90`..`00709b40` (all of it). As in the first part, most
//! functions are static `Interface::` wrappers over the interface manager
//! (`InterfaceManager`, Xbox PDB): `004b7210` returns the manager and
//! `009373f0` reads its `bFirstInit` byte; a wrapper does nothing unless the
//! manager exists and that byte is set. Manager offsets used here (PC
//! offsets match the Xbox PDB): `+0x9c pMenusRoot`, `+0xbc` and `+0xc4`
//! words set by `00706cb0` and `00705b10`, `+0xcc`/`+0xd4` tile slots
//! cleared by `TileIsBeingDeleted`, `+0x4bc` a mode word and the sub-object
//! at `+0x4d4`.
//!
//! The compiler's exception-unwinding frames (`CreateHackingMenu`,
//! `CreateComputersMenu`) and stack-cookie checks are not translated.

#[allow(unused_imports)]
use super::interface::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `004b7210`: the interface manager object.
const GET_MANAGER: u32 = 0x004b_7210;
/// `009373f0`: the byte at the start of the manager (`bFirstInit`).
const MANAGER_READY: u32 = 0x0093_73f0;
/// `00586150`: the manager's `pMenusRoot`.
const GET_MENUS_ROOT: u32 = 0x0058_6150;
/// `00559450`: the first word of the object (a list head).
const FIRST_WORD: u32 = 0x0055_9450;
/// `0057cbe0`: list iterator step (list, address of the cursor word); it
/// returns the address of the current item and advances the cursor.
const LIST_NEXT: u32 = 0x0057_cbe0;
/// `0057c730`: takes the address of a word holding an item and returns the
/// object it removes from the list (0 when there is none).
const LIST_REMOVE: u32 = 0x0057_c730;
/// `Tile::GetMenu` (Xbox PDB): the menu a tile belongs to.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// Tile trait getter (`00a011b0`, value in ST0): tile, trait id.
const TILE_GET_VALUE: u32 = 0x00a0_11b0;
/// Tile trait setter (`00700320`): tile, trait id, value.
const TILE_SET_VALUE: u32 = 0x0070_0320;
/// Trait id (`0x1771`) of the menus root tile that is saved and restored
/// with the game.
const TRAIT_SAVED_VALUE: u32 = 0x1771;
/// `Tile::UpdateAll` (Xbox PDB), `cdecl`: one word.
const TILE_UPDATE_ALL: u32 = 0x00a0_4200;
/// `00401000`: `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `FORenderedMenu::FORenderedMenu` (Xbox PDB): constructs the menu in the
/// given block and returns it.
const RENDERED_MENU_CTOR: u32 = 0x0070_5fe0;
/// Size of a `FORenderedMenu`.
const RENDERED_MENU_SIZE: u32 = 0xf0;
/// `00984f60` on the rendered menu: takes the menu it renders.
const RENDERED_MENU_SET_MENU: u32 = 0x0098_4f60;
/// `0056c7f0`: the node a menu keeps.
const MENU_NODE: u32 = 0x0056_c7f0;
/// `00705fc0` on the rendered menu: takes that node.
const RENDERED_MENU_SET_NODE: u32 = 0x0070_5fc0;
/// `InterfaceManager::EnterRenderedMenu` (Xbox PDB), on the manager.
const ENTER_RENDERED_MENU: u32 = 0x0071_81d0;
/// `00408d60` called with `ECX = 011db620`: returns the address of a byte
/// flag (nonzero when rendered menus are used).
const RENDERED_MENUS_FLAG_GETTER: u32 = 0x0040_8d60;
/// The object `00408d60` is called on.
const RENDERED_MENUS_FLAG_OWNER: u32 = 0x011d_b620;
/// `HackingMenu::Create` (Xbox PDB), `cdecl`: one word.
const HACKING_MENU_CREATE: u32 = 0x0076_5b80;
/// `ComputersMenu::Create` (Xbox PDB), `cdecl`: one word.
const COMPUTERS_MENU_CREATE: u32 = 0x0075_7b70;
/// `__RTDynamicCast` (`00ec43fb`): object, vfptr delta, source type,
/// target type, is-reference.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Source type descriptor of the casts `CloseAllSupportMenus` makes.
const CAST_SOURCE_TYPE: u32 = 0x0119_f34c;
/// Target type descriptor of the cast for menu id `0x424` (the menu whose
/// close function is `SPECIALBookMenu::Close`).
const SPECIAL_BOOK_MENU_TYPE: u32 = 0x0119_f47c;
/// Target type descriptor of the cast for menu id `0x432`.
const MENU_432_TYPE: u32 = 0x0119_f45c;
/// `SPECIALBookMenu::Close` (Xbox PDB), no arguments.
const SPECIAL_BOOK_MENU_CLOSE: u32 = 0x007c_6120;
/// `007917a0`, no arguments: the close function for menu id `0x432`.
const MENU_432_CLOSE: u32 = 0x0079_17a0;
/// `004a4040`: returns a flag that decides whether menu `0x3f5` is closed.
const CLOSE_MENU_3F5_ALLOWED: u32 = 0x004a_4040;
/// `_ftol2_sse` (`00ec62c0`): the value is passed as an `f64`.
const FTOL: u32 = 0x00ec_62c0;
/// `004a8f20`, `cdecl`, two words (its body is not part of this unit).
const HELPER_4A8F20: u32 = 0x004a_8f20;
/// Byte set to 1 while `CloseAllSupportMenus` runs.
const CLOSING_SUPPORT_MENUS: u32 = 0x011d_8909;
/// The loading menu object (0 when none).
const LOADING_MENU: u32 = 0x011d_a0c0;
/// `007027b0` (`cdecl`, one menu id), translated in the first part.
const MENU_ID_FLAG: u32 = 0x0070_27b0;
/// `00703fe0`, `cdecl`, one flag word.
const SET_LOADING_FLAG: u32 = 0x0070_3fe0;
/// `LoadingMenu::Create` (Xbox PDB), `cdecl`: three words.
const LOADING_MENU_CREATE: u32 = 0x0078_8cc0;
/// The pointer to the game's save object (`011de45c`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// Writes `(pointer, size)` bytes into the save game (`008579b0`).
const SAVE_GAME_WRITE: u32 = 0x0085_79b0;
/// `008df040` on the object in `011de45c`: returns a byte.
const SAVE_OBJECT_BYTE: u32 = 0x008d_f040;
/// Save game buffer write (`00865e50`): `(pointer, size, 0)`.
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;
/// Load game buffer read (`00864980`): `(pointer, size)`.
const LOAD_BUFFER_READ: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadString` (Xbox PDB): reads a string into the given
/// buffer.
const LOAD_BUFFER_LOAD_STRING: u32 = 0x0086_49a0;
/// `memset` (`00403d30`): destination, value, length.
const MEMSET: u32 = 0x0040_3d30;
/// The state getters written by `007066d0` (bytes, then floats) and the
/// setters `LoadGame` hands them to.
const STATS_PAGE_GET: u32 = 0x007d_fcd0;
const INVENTORY_BYTE_GET: u32 = 0x0078_24b0;
const MAP_FLOAT_GET_A: u32 = 0x007a_17a0;
const MAP_FLOAT_GET_B: u32 = 0x007a_17e0;
const STATS_PAGE_SET: u32 = 0x007d_fd00;
const MAP_FLOAT_SET_A: u32 = 0x007a_1820;
const MAP_FLOAT_SET_B: u32 = 0x007a_1850;
/// The manager's sub-object at `+0x4d4`: `007187a0` writes a string into a
/// buffer, `00718890` takes one.
const MANAGER_SUBOBJECT: u32 = 0x4d4;
const SUBOBJECT_GET_STRING: u32 = 0x0071_87a0;
const SUBOBJECT_SET_STRING: u32 = 0x0071_8890;
/// Size of the string buffer saved and loaded with the menu state.
const STATE_STRING_SIZE: u32 = 100;
/// Pointer to the object whose `+0x80` the byte `00706830` reads.
const STATE_OBJECT: u32 = 0x011d_a368;
/// Pointer to the object `00706a40` updates, and the word holding the trait
/// id it sets.
const TRAIT_OBJECT: u32 = 0x011d_9ea4;
const TRAIT_OBJECT_TRAIT_ID: u32 = 0x011d_9eac;
/// Byte set by `00706ae0`.
const FINISH_FLAG: u32 = 0x011d_8a85;
/// The object `00953060` is called on, in `00706ae0`.
const FINISH_OBJECT: u32 = 0x011d_ea3c;
/// The `float` at `0106ec38` (an aspect ratio).
const ASPECT_RATIO_CONSTANT: u32 = 0x0106_ec38;
/// The table of `0x1c` pointers (`011d51d0`) `00707330` searches.
const TEXT_TABLE: u32 = 0x011d_51d0;
/// The object (`011dea3c`) the player-name, race and sex texts come from.
const TEXT_OWNER: u32 = 0x011d_ea3c;
/// `"%.2f"`.
const FLOAT_FORMAT: u32 = 0x0103_0018;
/// `"%i"`.
const INTEGER_FORMAT: u32 = 0x0102_0774;
/// `"PCName"`, `"PCRace"`, `"PCSex"`, `"PCSexPronoun"`, `"PCSexPossessive"`.
const PC_NAME_TEXT: u32 = 0x0106_ec6c;
const PC_RACE_TEXT: u32 = 0x0106_ec64;
const PC_SEX_TEXT: u32 = 0x0106_ec5c;
const PC_SEX_PRONOUN_TEXT: u32 = 0x0106_ec4c;
const PC_SEX_POSSESSIVE_TEXT: u32 = 0x0106_ec3c;
/// `":LANGUAGE"`.
const LANGUAGE_SUFFIX: u32 = 0x0106_ec84;
/// `"&true;"` and `"&false;"`.
const TRUE_TEXT: u32 = 0x0106_ec7c;
const FALSE_TEXT: u32 = 0x0106_ec74;
/// The model loader (`011c3b3c`), and the byte (`011d8908`) that makes
/// `00707660` load without it.
const MODEL_LOADER: u32 = 0x011c_3b3c;
const LOADER_DISABLED: u32 = 0x011d_8908;
/// The temporary model (`011d890c`) `00707660` builds and `00707820` frees.
const TEMP_MODEL: u32 = 0x011d_890c;
/// Bytes of the `BSStream` `00707660` keeps on its stack.
const BS_STREAM_SIZE: u32 = 0x5e0;
/// Bytes of the cloning process `00707870` keeps on its stack.
const CLONING_PROCESS_SIZE: u32 = 0x20;
/// The object (`011dea10`) whose deep copy `00707870` calls.
const DEEP_COPY_OWNER: u32 = 0x011d_ea10;

/// The manager now (each wrapper asks for it again where the game does).
fn manager(e: &mut Engine) -> u32 {
    e.call(GET_MANAGER, &[]).u32()
}

/// The wrappers' guard: `Some(manager)` only when it exists and
/// `009373f0` says so.
fn ready_manager(e: &mut Engine) -> Option<u32> {
    if manager(e) == 0 {
        return None;
    }
    let current = manager(e);
    if e.call(MANAGER_READY, &args![current]).bool() {
        Some(current)
    } else {
        None
    }
}

/// `_ftol2_sse` on a `float`.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

// Translated from 00705a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `007058c0` on it (0 otherwise).
pub fn fn_00705a90(e: &mut Engine) -> u32 {
    match ready_manager(e) {
        Some(manager) => e.call(0x0070_58c0, &args![manager]).u32(),
        None => 0,
    }
}

// Translated from 00705ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: stores `value` in it (`00705b10`).
pub fn fn_00705ad0(e: &mut Engine, value: u32) {
    if ready_manager(e).is_some() {
        let current = manager(e);
        fn_00705b10(e, current, value);
    }
}

// Translated from 00705b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the manager's word at `+0xc4`.
pub fn fn_00705b10(e: &mut Engine, this: u32, value: u32) {
    e.mem.set_u32(this + 0xc4, value);
}

// Translated from 00705b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `00716010(manager, value)`.
pub fn fn_00705b30(e: &mut Engine, value: u32) {
    if ready_manager(e).is_some() {
        let current = manager(e);
        e.call(0x0071_6010, &args![current, value]);
    }
}

// Translated from 00705b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: gives it three values, each the byte (as a
/// signed byte) raised to at least 0 (`00705c40`) and passed through
/// `004a8f20(4, ..)` (`0070b790`, `0070b810`, `0070b870`), then the menu id
/// (`0070b8d0`), which is `menu_id` when it is `0x3eb`, `0x3ea` or `0x3ff`
/// and `0x3eb` otherwise.
pub fn fn_00705b70(e: &mut Engine, first: u8, second: u8, third: u8, menu_id: i32) {
    if ready_manager(e).is_none() {
        return;
    }
    for (setter, byte) in [
        (0x0070_b790, first),
        (0x0070_b810, second),
        (0x0070_b870, third),
    ] {
        let clamped = fn_00705c40(e, 0, byte as i8);
        let value = e.call(HELPER_4A8F20, &args![4u32, clamped]).u32();
        let current = manager(e);
        e.call(setter, &args![current, value]);
    }
    let menu_id = if menu_id == 0x3eb || menu_id == 0x3ea || menu_id == 0x3ff {
        menu_id
    } else {
        0x3eb
    };
    let current = manager(e);
    e.call(0x0070_b8d0, &args![current, menu_id]);
}

// Translated from 00705c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The larger of `a` and the signed byte `b`.
pub fn fn_00705c40(_e: &mut Engine, a: i32, b: i8) -> i32 {
    if a > b as i32 {
        a
    } else {
        b as i32
    }
}

// Translated from 00705c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `0047ee30` on it, a byte (`0xff` otherwise).
pub fn fn_00705c70(e: &mut Engine) -> u8 {
    match ready_manager(e) {
        Some(manager) => e.call(0x0047_ee30, &args![manager]).u8(),
        None => 0xff,
    }
}

// Translated from 00705cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: the byte at its `+0x13` (`0xff` otherwise).
pub fn fn_00705cb0(e: &mut Engine) -> u8 {
    match ready_manager(e) {
        Some(manager) => fn_00705cf0(e, manager),
        None => 0xff,
    }
}

// Translated from 00705cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x13` of the manager.
pub fn fn_00705cf0(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this + 0x13)
}

// Translated from 00705d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `00843150` on it, a byte (`0xff` otherwise).
pub fn fn_00705d10(e: &mut Engine) -> u8 {
    match ready_manager(e) {
        Some(manager) => e.call(0x0084_3150, &args![manager]).u8(),
        None => 0xff,
    }
}

// Translated from 00705d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `00982bc0` on it, a byte (`0xff` otherwise).
pub fn fn_00705d50(e: &mut Engine) -> u8 {
    match ready_manager(e) {
        Some(manager) => e.call(0x0098_2bc0, &args![manager]).u8(),
        None => 0xff,
    }
}

// Translated from 00705d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `00705dd0` on it (`-1` otherwise).
pub fn fn_00705d90(e: &mut Engine) -> i32 {
    match ready_manager(e) {
        Some(manager) => fn_00705dd0(e, manager),
        None => -1,
    }
}

// Translated from 00705dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Trait `0x1771` of the tile at `+0x9c` of the manager (`pMenusRoot`),
/// truncated to an integer.
pub fn fn_00705dd0(e: &mut Engine, this: u32) -> i32 {
    let root = e.mem.u32(this + 0x9c);
    let value = e
        .call(TILE_GET_VALUE, &args![root, TRAIT_SAVED_VALUE])
        .f32();
    float_to_int(e, value)
}

// Translated from 00705e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateLoadingMenu` (Xbox PDB): sets the loading flag
/// (`00703fe0(1)`), then `LoadingMenu::Create(arg, 1, flag)`.
pub fn interface_create_loading_menu(e: &mut Engine, arg: u32, flag: u8) -> u32 {
    e.call(SET_LOADING_FLAG, &args![1u32]);
    e.call(LOADING_MENU_CREATE, &args![arg, 1u32, flag as u32])
        .u32()
}

// Translated from 00705e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CloseLoadingMenu` (Xbox PDB): destroys the loading menu
/// object when there is one (its virtual slot 0 with the argument 1), then
/// clears the loading flag (`00703fe0(0)`).
pub fn interface_close_loading_menu(e: &mut Engine) {
    let menu = e.mem.u32(LOADING_MENU);
    if menu != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    e.call(SET_LOADING_FLAG, &args![0u32]);
}

// Translated from 00705e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetLoadingMenuVisible` (Xbox PDB): `007027b0(0x3ef)`.
pub fn interface_get_loading_menu_visible(e: &mut Engine) -> u8 {
    e.call(MENU_ID_FLAG, &args![0x3efu32]).u8()
}

// Translated from 00705ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsInGameLoadingMenuOpen` (Xbox PDB): whether a loading menu
/// object exists.
pub fn interface_is_in_game_loading_menu_open(e: &mut Engine) -> bool {
    e.mem.u32(LOADING_MENU) != 0
}

/// The tail both rendered-menu creators share: wraps `menu` in a new
/// `FORenderedMenu` and enters it.
fn enter_rendered_menu(e: &mut Engine, menu: u32) {
    let block = e.call(OPERATOR_NEW, &args![RENDERED_MENU_SIZE]).u32();
    let rendered = if block != 0 {
        e.call(RENDERED_MENU_CTOR, &args![block]).u32()
    } else {
        0
    };
    e.call(RENDERED_MENU_SET_MENU, &args![rendered, menu]);
    let node = e.call(MENU_NODE, &args![menu]).u32();
    e.call(RENDERED_MENU_SET_NODE, &args![rendered, node]);
    let current = manager(e);
    e.call(ENTER_RENDERED_MENU, &args![current, rendered]);
}

/// Whether rendered menus are in use (the byte `00408d60` points at).
fn rendered_menus_in_use(e: &mut Engine) -> bool {
    let flag = e
        .call(
            RENDERED_MENUS_FLAG_GETTER,
            &args![RENDERED_MENUS_FLAG_OWNER],
        )
        .u32();
    e.mem.u8(flag) != 0
}

// Translated from 00705ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateHackingMenu` (Xbox PDB): when the manager is ready,
/// creates the hacking menu (`HackingMenu::Create`); when rendered menus
/// are in use, updates all tiles and enters the menu inside a new
/// `FORenderedMenu`. Returns whether the manager was ready. The
/// compiler's exception frame is not translated.
pub fn interface_create_hacking_menu(e: &mut Engine, arg: u32) -> bool {
    if ready_manager(e).is_none() {
        return false;
    }
    let menu = e.call(HACKING_MENU_CREATE, &args![arg]).u32();
    if rendered_menus_in_use(e) {
        e.call(TILE_UPDATE_ALL, &args![0u32]);
        enter_rendered_menu(e, menu);
    }
    true
}

// Translated from 00706130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateComputersMenu` (Xbox PDB): as
/// [`interface_create_hacking_menu`] for `ComputersMenu::Create`, except that
/// all tiles are updated before the rendered-menus check, whatever its
/// result.
pub fn interface_create_computers_menu(e: &mut Engine, arg: u32) -> bool {
    if ready_manager(e).is_none() {
        return false;
    }
    let menu = e.call(COMPUTERS_MENU_CREATE, &args![arg]).u32();
    e.call(TILE_UPDATE_ALL, &args![0u32]);
    if rendered_menus_in_use(e) {
        enter_rendered_menu(e, menu);
    }
    true
}

// Translated from 00706230 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `007e41f0(a, b)`.
pub fn fn_00706230(e: &mut Engine, a: u32, b: u32) {
    if ready_manager(e).is_some() {
        e.call(0x007e_41f0, &args![a, b]);
    }
}

// Translated from 00706270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateLevelUpMenu` (Xbox PDB): when the manager is ready,
/// `LevelUpMenu::Create`.
pub fn interface_create_level_up_menu(e: &mut Engine) {
    if ready_manager(e).is_some() {
        e.call(0x0078_4c80, &[]);
    }
}

// Translated from 007062a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the manager is ready: `CharGenMenu::Create(a, b, c, d)`.
pub fn fn_007062a0(e: &mut Engine, a: u32, b: u32, c: u8, d: u8) {
    if ready_manager(e).is_some() {
        e.call(0x0075_3420, &args![a, b, c as u32, d as u32]);
    }
}

// Translated from 007062e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetInfoForRef` (Xbox PDB): when the manager is ready,
/// `HUDMainMenu::SetInfoForRef(reference, b, c)` (a byte; 0 otherwise).
pub fn interface_set_info_for_ref(e: &mut Engine, reference: u32, b: u8, c: u8) -> u8 {
    if ready_manager(e).is_none() {
        return 0;
    }
    e.call(0x0077_5a00, &args![reference, b as u32, c as u32])
        .u8()
}

/// Removes the tile whose pointer is at `tile_slot` from `list` (`0057c730`)
/// and, if that returns an object, calls its virtual slot 0 with the
/// argument 1.
fn remove_tile_and_delete(e: &mut Engine, list: u32, tile_slot: u32) {
    let removed = e.call(LIST_REMOVE, &args![list, tile_slot]).u32();
    if removed != 0 {
        e.vcall(removed, 0, &args![1u32]);
    }
}

/// `__RTDynamicCast` of the tile's menu to `target_type` (0 when the tile is
/// null).
fn cast_tile_menu(e: &mut Engine, tile: u32, target_type: u32) -> u32 {
    if tile == 0 {
        return 0;
    }
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![menu, 0u32, CAST_SOURCE_TYPE, target_type, 0u32],
    )
    .u32()
}

// Translated from 00706320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CloseAllSupportMenus` (Xbox PDB): with the byte at
/// `011d8909` set during the walk, goes over the children of the menus root
/// and, for each tile that belongs to a menu, acts on the menu id (virtual
/// slot `0x34` of the menu):
/// - ids `0x3ea 0x3eb 0x3ec 0x3ef 0x3ff 0x40b 0x425`: kept;
/// - `0x3f5`: removed and deleted only when `004a4040` says so;
/// - `0x424`: `SPECIALBookMenu::Close` when the menu casts to its class,
///   then removed and deleted;
/// - `0x432`: `007917a0` when the menu casts to its class, then removed and
///   deleted;
/// - any other id: removed and deleted.
///
/// The caller pushes one word the function never reads.
pub fn interface_close_all_support_menus(e: &mut Engine, _unused_0: u32) {
    e.mem.set_u8(CLOSING_SUPPORT_MENUS, 1);
    let current = manager(e);
    let root = e.call(GET_MENUS_ROOT, &args![current]).u32();
    let list = root + 4;
    let first = e.call(FIRST_WORD, &args![list]).u32();
    // Two words of the game's stack: the list cursor and the current tile.
    e.with_stack(8, |e, slots| {
        let cursor = slots.addr();
        let tile_slot = slots.addr() + 4;
        e.mem.set_u32(cursor, first);
        while e.mem.u32(cursor) != 0 {
            let item = e.call(LIST_NEXT, &args![list, cursor]).u32();
            let tile = e.mem.u32(item);
            e.mem.set_u32(tile_slot, tile);
            if tile == 0 || e.call(TILE_GET_MENU, &args![tile]).u32() == 0 {
                continue;
            }
            let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            let menu_id = e.vcall(menu, 0x34, &[]).u32();
            match menu_id {
                0x3ea | 0x3eb | 0x3ec | 0x3ef | 0x3ff | 0x40b | 0x425 => {}
                0x3f5 => {
                    if e.call(CLOSE_MENU_3F5_ALLOWED, &[]).bool() {
                        remove_tile_and_delete(e, list, tile_slot);
                    }
                }
                0x424 => {
                    if cast_tile_menu(e, tile, SPECIAL_BOOK_MENU_TYPE) != 0 {
                        e.call(SPECIAL_BOOK_MENU_CLOSE, &[]);
                    }
                    remove_tile_and_delete(e, list, tile_slot);
                }
                0x432 => {
                    if cast_tile_menu(e, tile, MENU_432_TYPE) != 0 {
                        e.call(MENU_432_CLOSE, &[]);
                    }
                    remove_tile_and_delete(e, list, tile_slot);
                }
                _ => remove_tile_and_delete(e, list, tile_slot),
            }
        }
    });
    e.mem.set_u8(CLOSING_SUPPORT_MENUS, 0);
}

// Translated from 007065c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 8, or 9 when the byte `008df040` returns for the object in `011de45c`
/// is at least `0x5d`.
pub fn fn_007065c0(e: &mut Engine) -> u16 {
    let owner = e.mem.u32(SAVE_LOAD_GAME);
    let byte = e.call(SAVE_OBJECT_BYTE, &args![owner]).u8();
    if byte >= 0x5d {
        9
    } else {
        8
    }
}

// Translated from 00706610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SaveGame` (Xbox PDB): writes four bytes (`00705c70`,
/// `00705cb0`, `00705d10`, `00705d50`, each raised to at least 1 as a
/// signed byte) and the word `00705d90` into the save game (`008579b0` on
/// the object in `011de45c`).
pub fn interface_save_game(e: &mut Engine) {
    // The game's locals: the word at +0, then the bytes in the order
    // first, fourth, second, third.
    e.with_stack(8, |e, frame| {
        let base = frame.addr();
        let (first, second, third, fourth) = (base + 4, base + 6, base + 7, base + 5);
        let value = fn_00705c70(e);
        e.mem.set_u8(first, value);
        let value = fn_00705cb0(e);
        e.mem.set_u8(second, value);
        let value = fn_00705d10(e);
        e.mem.set_u8(third, value);
        let value = fn_00705d50(e);
        e.mem.set_u8(fourth, value);
        let word = fn_00705d90(e);
        e.mem.set_i32(base, word);
        for byte in [first, second, third, fourth] {
            if e.mem.i8(byte) < 1 {
                e.mem.set_i8(byte, 1);
            }
        }
        for byte in [first, second, third, fourth] {
            let owner = e.mem.u32(SAVE_LOAD_GAME);
            e.call(SAVE_GAME_WRITE, &args![owner, byte, 1u32]);
        }
        let owner = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_GAME_WRITE, &args![owner, base, 4u32]);
    });
}

// Translated from 007066d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the menu state into the save game buffer `stream`: trait `0x1771`
/// of the menus root as an integer, the bytes of `007dfcd0`, `007824b0` and
/// `00706830`, the floats of `007a17a0` and `007a17e0`, then the word
/// `00706810` and the string of the manager's sub-object (`007187a0` into a
/// 100-byte buffer, written with that length). [`interface_load_game`] reads
/// it back.
pub fn fn_007066d0(e: &mut Engine, stream: u32) {
    // The game's frame, from its lowest local: the length word at +0, the
    // 100-byte buffer at +8, the bytes at +7, +0x7b and +0x83, the trait
    // value at +0x74, the floats at +0x84 and +0x7c.
    e.with_stack(0x88, |e, frame| {
        let base = frame.addr();
        let (length, text) = (base, base + 8);
        let (first_byte, second_byte, third_byte) = (base + 7, base + 0x7b, base + 0x83);
        let (trait_value, first_float, second_float) = (base + 0x74, base + 0x84, base + 0x7c);

        let current = manager(e);
        let root = e.call(GET_MENUS_ROOT, &args![current]).u32();
        let value = e
            .call(TILE_GET_VALUE, &args![root, TRAIT_SAVED_VALUE])
            .f32();
        let truncated = float_to_int(e, value);
        e.mem.set_i32(trait_value, truncated);
        let byte = e.call(STATS_PAGE_GET, &[]).u8();
        e.mem.set_u8(first_byte, byte);
        let byte = e.call(INVENTORY_BYTE_GET, &[]).u8();
        e.mem.set_u8(second_byte, byte);
        let byte = fn_00706830(e);
        e.mem.set_u8(third_byte, byte);
        for (data, size) in [
            (trait_value, 4u32),
            (first_byte, 1),
            (second_byte, 1),
            (third_byte, 1),
        ] {
            e.call(SAVE_BUFFER_WRITE, &args![stream, data, size, 0u32]);
        }
        let float = e.call(MAP_FLOAT_GET_A, &[]).f32();
        e.mem.set_f32(first_float, float);
        let float = e.call(MAP_FLOAT_GET_B, &[]).f32();
        e.mem.set_f32(second_float, float);
        e.call(SAVE_BUFFER_WRITE, &args![stream, first_float, 4u32, 0u32]);
        e.call(SAVE_BUFFER_WRITE, &args![stream, second_float, 4u32, 0u32]);

        e.call(MEMSET, &args![text, 0u32, STATE_STRING_SIZE]);
        let current = manager(e);
        let size = fn_00706810(e, current + MANAGER_SUBOBJECT);
        e.mem.set_i32(length, size);
        let current = manager(e);
        e.call(
            SUBOBJECT_GET_STRING,
            &args![current + MANAGER_SUBOBJECT, text],
        );
        e.call(SAVE_BUFFER_WRITE, &args![stream, length, 4u32, 0u32]);
        e.call(SAVE_BUFFER_WRITE, &args![stream, text, size as u32, 0u32]);
    });
}

// Translated from 00706810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004a8f20(1, 1) + 5` (the object it is called on is not used).
pub fn fn_00706810(e: &mut Engine, _unused_0: u32) -> i32 {
    e.call(HELPER_4A8F20, &args![1u32, 1u32])
        .i32()
        .wrapping_add(5)
}

// Translated from 00706830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low byte of `(word at +0x80 of the object in 011da368) - 0x20`, or 4
/// when there is no object.
pub fn fn_00706830(e: &mut Engine) -> u8 {
    let object = e.mem.u32(STATE_OBJECT);
    if object == 0 {
        4
    } else {
        e.mem.u32(object + 0x80).wrapping_sub(0x20) as u8
    }
}

// Translated from 00706860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::LoadGame` (Xbox PDB): reads from the load game buffer
/// `stream` the state [`fn_007066d0`] wrote (a word, three bytes, two
/// floats) and applies it: the word to trait `0x1771` of the menus root,
/// the bytes to `StatsMenu::SetCurrentStatsPage`, `00706a20` and `007069d0`,
/// the floats to `007a1820` and `007a1850`. Then, if the stream's version
/// byte (virtual slot 0) is at least 12, reads a length and that many bytes
/// of text into a 100-byte buffer for the manager's sub-object
/// (`00718890`); otherwise reads a string (`LoadString`) that nobody uses.
pub fn interface_load_game(e: &mut Engine, stream: u32) {
    // The game's frame, from its lowest local: the LoadString buffer at +0,
    // the length word at +0x10c, the 100-byte buffer at +0x110, the saved
    // word and bytes at +0x184, +0x183, +0x18b and +0x193, the floats at
    // +0x194 and +0x18c.
    e.with_stack(0x198, |e, frame| {
        let base = frame.addr();
        let (word, first_byte, second_byte, third_byte) =
            (base + 0x184, base + 0x183, base + 0x18b, base + 0x193);
        let (first_float, second_float) = (base + 0x194, base + 0x18c);
        e.mem.set_u32(word, 0);
        e.mem.set_u8(first_byte, 0);
        e.mem.set_u8(second_byte, 0);
        e.mem.set_u8(third_byte, 0);
        e.call(LOAD_BUFFER_READ, &args![stream, word, 4u32]);
        e.call(LOAD_BUFFER_READ, &args![stream, first_byte, 1u32]);
        e.call(LOAD_BUFFER_READ, &args![stream, second_byte, 1u32]);
        e.call(LOAD_BUFFER_READ, &args![stream, third_byte, 1u32]);
        let current = manager(e);
        let root = e.call(GET_MENUS_ROOT, &args![current]).u32();
        let saved = e.mem.u32(word);
        e.call(TILE_SET_VALUE, &args![root, TRAIT_SAVED_VALUE, saved]);
        let byte = e.mem.u8(first_byte);
        e.call(STATS_PAGE_SET, &args![byte as u32]);
        let byte = e.mem.u8(second_byte);
        fn_00706a20(e, byte);
        let byte = e.mem.u8(third_byte);
        fn_007069d0(e, byte);
        e.mem.set_f32(first_float, 0.0);
        e.mem.set_f32(second_float, 0.0);
        e.call(LOAD_BUFFER_READ, &args![stream, first_float, 4u32]);
        e.call(LOAD_BUFFER_READ, &args![stream, second_float, 4u32]);
        let float = e.mem.f32(first_float);
        e.call(MAP_FLOAT_SET_A, &args![float]);
        let float = e.mem.f32(second_float);
        e.call(MAP_FLOAT_SET_B, &args![float]);
        let version = e.vcall(stream, 0, &[]).u8();
        if version >= 12 {
            // Under `SUB ESP,0x198` (00706863): the length word is EBP-0x8c
            // (00706968/00706974), the buffer EBP-0x88 (00706959, 0070698a,
            // 00706999).
            let (length, text) = (base + 0x10c, base + 0x110);
            e.call(MEMSET, &args![text, 0u32, STATE_STRING_SIZE]);
            e.mem.set_u32(length, 0);
            e.call(LOAD_BUFFER_READ, &args![stream, length, 4u32]);
            let size = e.mem.u32(length);
            e.call(LOAD_BUFFER_READ, &args![stream, text, size]);
            let current = manager(e);
            e.call(
                SUBOBJECT_SET_STRING,
                &args![current + MANAGER_SUBOBJECT, text],
            );
        } else {
            e.call(LOAD_BUFFER_LOAD_STRING, &args![stream, base]);
        }
    });
}

// Translated from 007069d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object in `011da368` exists: `007069f0(object, value)`.
pub fn fn_007069d0(e: &mut Engine, value: u8) {
    let object = e.mem.u32(STATE_OBJECT);
    if object != 0 {
        fn_007069f0(e, object, value as u32);
    }
}

// Translated from 007069f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual slot `0xc` of the object with `(value + 0x20, 0)`.
pub fn fn_007069f0(e: &mut Engine, this: u32, value: u32) {
    e.vcall(this, 0xc, &args![value.wrapping_add(0x20), 0u32]);
}

// Translated from 00706a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00706a40` with the byte.
pub fn fn_00706a20(e: &mut Engine, value: u8) {
    fn_00706a40(e, value as u32);
}

// Translated from 00706a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object in `011d9ea4` exists: runs `value` through
/// `004a8f20(value, 4)`, sets that trait (the id in `011d9eac`) on the
/// object's tile (`+0x5c`) and stores it at `+0x84`.
pub fn fn_00706a40(e: &mut Engine, value: u32) {
    let object = e.mem.u32(TRAIT_OBJECT);
    if object == 0 {
        return;
    }
    let value = e.call(HELPER_4A8F20, &args![value, 4u32]).u32();
    let trait_id = e.mem.u32(TRAIT_OBJECT_TRAIT_ID);
    let tile = e.mem.u32(object + 0x5c);
    e.call(TILE_SET_VALUE, &args![tile, trait_id, value]);
    e.mem.set_u32(object + 0x84, value);
}

// Translated from 00706a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::FinishLoadGame` (Xbox PDB): calls, in order, `007dd710`,
/// `007df4e0`, `007df5d0`, `0079f640`, `00706ac0`, `007a1490` and
/// `007e5890`.
pub fn interface_finish_load_game(e: &mut Engine) {
    e.call(0x007d_d710, &[]);
    e.call(0x007d_f4e0, &[]);
    e.call(0x007d_f5d0, &[]);
    e.call(0x0079_f640, &[]);
    fn_00706ac0(e);
    e.call(0x007a_1490, &[]);
    e.call(0x007e_5890, &[]);
}

// Translated from 00706ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `+0x90` of the object in `011da368`, if any.
pub fn fn_00706ac0(e: &mut Engine) {
    let object = e.mem.u32(STATE_OBJECT);
    if object != 0 {
        e.mem.set_u32(object + 0x90, 0);
    }
}

// Translated from 00706ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte `011d8a85`, runs the functions `007053b0` and `00773050`,
/// and when `007050d0` says so sets a value (`00705190(0.0, 1)`) on the
/// object in `011dea3c` (`00953060`) and runs `0083e510`; then `00705780`,
/// `007f6920`, `CloseAllSupportMenus`, `007aa480`, `007dda60`, `00706ac0`,
/// `00798bf0`, `00706fd0(1, 1)`, `EnterRenderedMenu(0)`, clears the
/// manager's word at `+0x4bc` and runs `00706b90` on its sub-object.
pub fn fn_00706ae0(e: &mut Engine) {
    e.mem.set_u8(FINISH_FLAG, 1);
    e.call(0x0070_53b0, &[]);
    e.call(0x0077_3050, &[]);
    if e.call(0x0070_50d0, &[]).bool() {
        // 00706b08: `00705190` takes no word (plain RET, no stack read); the pushed 0.0
        // and 1 are the last two words of `00953060` (RET 0xc).
        let value = e.call(0x0070_5190, &args![]).u32();
        let object = e.mem.u32(FINISH_OBJECT);
        e.call(0x0095_3060, &args![object, value, 0.0f32, 1u32]);
        e.call(0x0083_e510, &[]);
    }
    e.call(0x0070_5780, &[]);
    e.call(0x007f_6920, &[]);
    interface_close_all_support_menus(e, 0);
    e.call(0x007a_a480, &[]);
    e.call(0x007d_da60, &[]);
    fn_00706ac0(e);
    e.call(0x0079_8bf0, &[]);
    e.call(0x0070_6fd0, &args![1u32, 1u32]);
    let current = manager(e);
    e.call(ENTER_RENDERED_MENU, &args![current, 0u32]);
    let current = manager(e);
    e.mem.set_u32(current + 0x4bc, 0);
    let current = manager(e);
    fn_00706b90(e, current + MANAGER_SUBOBJECT);
}

// Translated from 00706b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the manager sub-object's table: clears bits 0 and 1 of each of
/// its first `0x29` words and keeps only their low byte, then sets `+0xa8`
/// to 0 and `+0xa4` to `0x29`.
pub fn fn_00706b90(e: &mut Engine, this: u32) {
    for index in 0..0x29u32 {
        let slot = this + 4 * index;
        let mut word = e.mem.u32(slot);
        word &= 0xffff_fffe;
        word &= 0xffff_fffd;
        word &= 0xff;
        e.mem.set_u32(slot, word);
    }
    e.mem.set_u32(this + 0xa8, 0);
    e.mem.set_u32(this + 0xa4, 0x29);
}

// Translated from 00706c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::TileIsBeingDeleted` (Xbox PDB): forgets `tile` in the
/// manager: clears its slots at `+0xcc`/`+0xd0` and `+0xd4`/`+0xd8` when
/// they hold it, runs `00706cb0` when `0097ae90` returns it, and finally
/// `00713de0(manager, tile)`.
pub fn interface_tile_is_being_deleted(e: &mut Engine, tile: u32) {
    let current = manager(e);
    if e.mem.u32(current + 0xcc) == tile {
        e.mem.set_u32(current + 0xcc, 0);
        e.mem.set_u32(current + 0xd0, 0);
    }
    if e.mem.u32(current + 0xd4) == tile {
        e.mem.set_u32(current + 0xd4, 0);
        e.mem.set_u32(current + 0xd8, 0);
    }
    if e.call(0x0097_ae90, &args![current]).u32() == tile {
        fn_00706cb0(e, current);
    }
    e.call(0x0071_3de0, &args![current, tile]);
}

// Translated from 00706cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the manager's word at `+0xbc`.
pub fn fn_00706cb0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this + 0xbc, 0);
}

// Translated from 00706cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetMenusRoot` (Xbox PDB): `00586150` on the manager.
pub fn interface_get_menus_root(e: &mut Engine) -> u32 {
    let current = manager(e);
    e.call(0x0058_6150, &args![current]).u32()
}

// Translated from 00706cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004fd400` on the manager (a getter; its body is not part of this unit).
pub fn fn_00706cf0(e: &mut Engine) -> u32 {
    let current = manager(e);
    e.call(0x004f_d400, &args![current]).u32()
}

// Translated from 00706d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetDebugTextRoot` (Xbox PDB): `005e3fc0` on the manager.
pub fn interface_get_debug_text_root(e: &mut Engine) -> u32 {
    let current = manager(e);
    e.call(0x005e_3fc0, &args![current]).u32()
}

// Translated from 00706d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetInterfaceRoot` (Xbox PDB): `004fb070` on the manager.
pub fn interface_get_interface_root(e: &mut Engine) -> u32 {
    let current = manager(e);
    e.call(0x004f_b070, &args![current]).u32()
}

// Translated from 00706d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::AttachDefaultAlphaProperty` (Xbox PDB): takes the property
/// `006629f0` returns for the manager and attaches it to `node` with
/// `NiAVObject::AttachProperty` (`00439410`).
pub fn interface_attach_default_alpha_property(e: &mut Engine, node: u32) {
    let current = manager(e);
    let property = e.call(0x0066_29f0, &args![current]).u32();
    e.call(0x0043_9410, &args![node, property]);
}

// Translated from 00706d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetFirstChanceTextureRelease` (Xbox PDB): the byte at
/// `+0xec` of the manager (see `00706d90`).
pub fn interface_get_first_chance_texture_release(e: &mut Engine) -> u8 {
    let current = manager(e);
    fn_00706d90(e, current)
}

// Translated from 00706d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xec` of `this` (the manager's first-chance texture
/// release flag, which `00703960` sets).
pub fn fn_00706d90(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this + 0xec)
}

// Translated from 00706db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00703960(manager, value)`: sets the manager's byte at `+0xec`.
pub fn fn_00706db0(e: &mut Engine, value: u8) {
    let current = manager(e);
    e.call(0x0070_3960, &args![current, value]);
}

// Translated from 00706dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte in the manager's byte at `+0xdc`.
pub fn fn_00706dd0(e: &mut Engine, value: u8) {
    let current = manager(e);
    e.mem.set_u8(current + 0xdc, value);
}

// Translated from 00706df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetScreenHeight` (Xbox PDB): the `float` `00715da0` returns
/// in `ST0`.
pub fn interface_get_screen_height(e: &mut Engine) -> f32 {
    e.call(0x0071_5da0, &[]).f32()
}

// Translated from 00706e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetScreenWidth` (Xbox PDB): the `float`
/// `InterfaceManager::GetScreenWidth` (`00715d40`) returns in `ST0`.
pub fn interface_get_screen_width(e: &mut Engine) -> f32 {
    e.call(0x0071_5d40, &[]).f32()
}

// Translated from 00706e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetRealScreenHeight` (Xbox PDB): `00706e20`.
pub fn interface_get_real_screen_height(e: &mut Engine) -> f32 {
    fn_00706e20(e)
}

// Translated from 00706e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The renderer's integer `004dc200` (the word at `01189480`) as a float
/// in `ST0`.
pub fn fn_00706e20(e: &mut Engine) -> f32 {
    e.call(0x004d_c200, &[]).i32() as f32
}

// Translated from 00706e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetRealScreenWidth` (Xbox PDB): `00706e50`.
pub fn interface_get_real_screen_width(e: &mut Engine) -> f32 {
    fn_00706e50(e)
}

// Translated from 00706e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The renderer's integer `004dc1f0` as a float in `ST0`.
pub fn fn_00706e50(e: &mut Engine) -> f32 {
    e.call(0x004d_c1f0, &[]).i32() as f32
}

// Translated from 00706e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetScreenAspectRatio` (Xbox PDB): the manager's `float` at
/// `+0x4d0`.
pub fn interface_get_screen_aspect_ratio(e: &mut Engine) -> f32 {
    let current = manager(e);
    e.mem.f32(current + 0x4d0)
}

// Translated from 00706e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` constant at `0106ec38` (an aspect ratio).
pub fn fn_00706e80(e: &mut Engine) -> f32 {
    e.global::<f32>(ASPECT_RATIO_CONSTANT)
}

// Translated from 00706e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float from virtual calls on the object `0043c4b0` returns: the
/// constant of `00706e80` times (slot `0x90` of one result of that object's
/// slot `0xc8`, called with 0) divided by (slot `0x8c` of another result of
/// the same call, called with 0), the two counts read as unsigned.
pub fn fn_00706e90(e: &mut Engine) -> f32 {
    let object = e.call(0x0043_c4b0, &[]).u32();
    let first = e.vcall(object, 0xc8, &[]).u32();
    let second = e.vcall(object, 0xc8, &[]).u32();
    let divisor = e.vcall(second, 0x8c, &args![0u32]).u32() as f64;
    let dividend = e.vcall(first, 0x90, &args![0u32]).u32() as f64;
    let ratio = dividend / divisor;
    let base = fn_00706e80(e) as f64;
    (base * ratio) as f32
}

// Translated from 00706f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constant `0x3ec`.
pub fn fn_00706f20(_e: &mut Engine) -> u32 {
    0x3ec
}

// Translated from 00706f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::PlayMenuSound` (Xbox PDB): `InterfaceManager::PlayMenuSound`
/// (`00717280`, `cdecl`) with the sound id.
pub fn interface_play_menu_sound(e: &mut Engine, sound: u32) {
    e.call(0x0071_7280, &args![sound]);
}

// Translated from 00706f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::NewTimer` (Xbox PDB): `InterfaceManager::NewTimer`
/// (`007164c0`, `cdecl`) with the word and the `float`.
pub fn interface_new_timer(e: &mut Engine, id: u32, seconds: f32) {
    e.call(0x0071_64c0, &args![id, seconds]);
}

// Translated from 00706f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::UpdateAllTimers` (Xbox PDB): `InterfaceManager::
/// UpdateAllTimers` (`00716320`) on the manager.
pub fn interface_update_all_timers(e: &mut Engine) {
    let current = manager(e);
    e.call(0x0071_6320, &args![current]);
}

// Translated from 00706f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ClearTimer` (Xbox PDB): `InterfaceManager::ClearTimer`
/// (`007165d0`, `cdecl`, finds the manager itself) with the timer id. The
/// game also calls `004b7210` first and ignores the result.
pub fn interface_clear_timer(e: &mut Engine, id: u32) {
    manager(e);
    e.call(0x0071_65d0, &args![id]);
}

// Translated from 00706fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::AddToEnterStack` (Xbox PDB): `InterfaceManager::
/// AddToEnterStack` (`00714d90`) on the manager.
pub fn interface_add_to_enter_stack(e: &mut Engine, menu_id: u32) -> u32 {
    let current = manager(e);
    e.call(0x0071_4d90, &args![current, menu_id]).u32()
}

// Translated from 00706fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::PopFromEnterStack` (Xbox PDB): `InterfaceManager::
/// PopFromEnterStack` (`00714fd0`) on the manager, with the menu id and a
/// byte flag.
pub fn interface_pop_from_enter_stack(e: &mut Engine, menu_id: u32, flag: u8) -> u32 {
    let current = manager(e);
    e.call(0x0071_4fd0, &args![current, menu_id, flag]).u32()
}

// Translated from 00706ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetCurrentFocusTarget` (Xbox PDB): `InterfaceManager::
/// SetCurrentFocusTarget` (`00715860`) on the manager with the tile, the
/// word `0xfc3` and the flag 1.
pub fn interface_set_current_focus_target(e: &mut Engine, tile: u32) {
    let current = manager(e);
    e.call(0x0071_5860, &args![current, tile, 0xfc3u32, 1u32]);
}

// Translated from 00707010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::RecursiveFade` (Xbox PDB): `InterfaceManager::RecursiveFade`
/// (`00712450`) on the manager with the tile and the two floats.
pub fn interface_recursive_fade(e: &mut Engine, tile: u32, from: f32, to: f32) {
    let current = manager(e);
    e.call(0x0071_2450, &args![current, tile, from, to]);
}

// Translated from 00707040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's word at `+0xcc`.
pub fn fn_00707040(e: &mut Engine) -> u32 {
    let current = manager(e);
    e.mem.u32(current + 0xcc)
}

// Translated from 00707050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word in the manager's `+0xcc`.
pub fn fn_00707050(e: &mut Engine, value: u32) {
    let current = manager(e);
    e.mem.set_u32(current + 0xcc, value);
}

// Translated from 00707070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's word at `+0xd0`.
pub fn fn_00707070(e: &mut Engine) -> u32 {
    let current = manager(e);
    e.mem.u32(current + 0xd0)
}

// Translated from 00707080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word in the manager's `+0xd0`.
pub fn fn_00707080(e: &mut Engine, value: u32) {
    let current = manager(e);
    e.mem.set_u32(current + 0xd0, value);
}

// Translated from 007070a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0045cd60` on the manager (a getter; its body is not part of this unit).
pub fn fn_007070a0(e: &mut Engine) -> u32 {
    let current = manager(e);
    e.call(0x0045_cd60, &args![current]).u32()
}

/// The bounded string copy (`00406d30`: destination, size, source) the
/// text replacement uses.
fn copy_text(e: &mut Engine, dest: u32, size: u32, src: u32) {
    e.call(0x0040_6d30, &args![dest, size, src]);
}

/// Whether the constant `name` equals the string at `label` (`00404dc0`
/// returns 0 for equal).
fn name_is(e: &mut Engine, name: u32, label: u32) -> bool {
    e.call(0x0040_4dc0, &args![name, label]).i32() == 0
}

// Translated from 007070c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::FindTextReplacementString` (Xbox PDB): writes into `dest`
/// (`size` bytes) the text that replaces the constant `name` and returns
/// whether it found one. In order, the first that applies wins:
///
/// 1. a name `00707330` knows (an index below `0x1c`):
///    `GetControlPushTextFor(index, dest, flag)`;
/// 2. a form with that editor ID (`00483a00`) whose type byte (`+4`) is 6:
///    its value, formatted as `"%.2f"` when its byte at `+0x20` is `'f'` and
///    as `"%i"` otherwise;
/// 3. `"PCName"` (`0055d520` on the object in `011dea3c`), `"PCRace"`
///    (`0087f6c0`), `"PCSex"`, `"PCSexPronoun"` and `"PCSexPossessive"`
///    (a string of the tables at `0119b354`, `0119b364`, `0119b35c`,
///    indexed by `0087f4c0`), each compared with `00404dc0`;
/// 4. `007073d0` (a game-setting lookup).
///
/// The result is false for a null or empty name, buffer or size.
pub fn interface_find_text_replacement_string(
    e: &mut Engine,
    name: u32,
    dest: u32,
    size: u32,
    flag: u8,
) -> bool {
    if name == 0 || e.mem.u8(name) == 0 || dest == 0 || size == 0 {
        return false;
    }
    let mut found = false;
    let index = fn_00707330(e, name);
    if index < 0x1c {
        e.call(0x0070_39b0, &args![index, dest, flag]);
        found = true;
    }
    if !found {
        let form = e.call(0x0048_3a00, &args![name]).u32();
        if form != 0 && e.call(0x0040_1170, &args![form]).u32() == 6 {
            let kind = e.call(0x0052_9ea0, &args![form]).u32() as u8 as i8;
            let value = e.call(0x0052_6ac0, &args![form]).f32();
            if kind == 0x66 {
                e.call(0x0040_6d00, &args![dest, size, FLOAT_FORMAT, value as f64]);
            } else {
                let whole = float_to_int(e, value);
                e.call(0x0040_6d00, &args![dest, size, INTEGER_FORMAT, whole]);
            }
            found = true;
        }
    }
    if !found && name_is(e, name, PC_NAME_TEXT) {
        let owner = e.mem.u32(TEXT_OWNER);
        let text = e.call(0x0055_d520, &args![owner]).u32();
        copy_text(e, dest, size, text);
        found = true;
    }
    if !found && name_is(e, name, PC_RACE_TEXT) {
        let owner = e.mem.u32(TEXT_OWNER);
        let text = e.call(0x0087_f6c0, &args![owner]).u32();
        copy_text(e, dest, size, text);
        found = true;
    }
    for (label, table) in [
        (PC_SEX_TEXT, 0x0119_b354u32),
        (PC_SEX_PRONOUN_TEXT, 0x0119_b364),
        (PC_SEX_POSSESSIVE_TEXT, 0x0119_b35c),
    ] {
        if !found && name_is(e, name, label) {
            let owner = e.mem.u32(TEXT_OWNER);
            let which = e.call(0x0087_f4c0, &args![owner]).u32();
            let entry = e.mem.u32(table.wrapping_add(which.wrapping_mul(4)));
            let text = e.call(0x0040_3df0, &args![entry]).u32();
            copy_text(e, dest, size, text);
            found = true;
        }
    }
    if !found {
        found = fn_007073d0(e, name, dest);
    }
    found
}

// Translated from 00707330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::MatchTextReplaceConstantToGameSettings` (Xbox PDB): the
/// index in the table of `0x1c` pointers at `011d51d0` of the entry equal to
/// the game setting `name` (`004f8a30` on the object `00404a70` returns),
/// else of the first whose `00403df0` text matches `name` by `00469880`;
/// `0x1c` when none does.
///
/// `00404a70` and `004f8a30` are called as the game calls them: the first
/// leaves `name` on the stack for the second.
pub fn fn_00707330(e: &mut Engine, name: u32) -> i32 {
    let settings = e.call(0x0040_4a70, &[]).u32();
    let wanted = e.call(0x004f_8a30, &args![settings, name]).u32();
    let mut index = 0u32;
    while index < 0x1c && e.mem.u32(TEXT_TABLE + 4 * index) != wanted {
        index += 1;
    }
    if index == 0x1c {
        index = 0;
        while index < 0x1c {
            let entry = e.mem.u32(TEXT_TABLE + 4 * index);
            let text = e.call(0x0040_3df0, &args![entry]).u32();
            if e.call(0x0046_9880, &args![text, name]).i32() == 0 {
                break;
            }
            index += 1;
        }
    }
    index as i32
}

// Translated from 007073d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::TestConstantForGameSettings` (Xbox PDB): looks the constant
/// `name` up as a game setting and writes its value as text into `dest`;
/// false (and nothing written) when `name` is null or empty or no setting
/// is found.
///
/// A leading `&` and then a leading `-` are skipped, the rest is copied into
/// a `0x104`-byte buffer and a trailing `;` is cut. The setting is looked up
/// by `004f8a30` (on the object `00404a70` returns); when it is missing,
/// `":LANGUAGE"` is appended and `005e02b0` (on the object `0044f560`
/// returns) tries again. The first letter of the buffer gives the type: `s`
/// copies the setting's text, `i` formats its integer with `"%i"`, `f`
/// formats its float, also with `"%i"` (as the exe does), and `b` copies
/// `"&true;"` when its byte is 1 and `"&false;"` otherwise. The stack cookie
/// check is not translated.
pub fn fn_007073d0(e: &mut Engine, name: u32, dest: u32) -> bool {
    if name == 0 || e.mem.u8(name) == 0 {
        return false;
    }
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        let mut skip = 0u32;
        if e.mem.u8(name) == b'&' {
            skip += 1;
        }
        if e.mem.u8(name + skip) == b'-' {
            skip += 1;
        }
        e.call(0x0040_6d30, &args![buffer, 0x104u32, name + skip]);
        let length = e.call(0x0044_a670, &args![buffer]).u32();
        if e.mem.u8(buffer + length.wrapping_sub(1)) == b';' {
            let length = e.call(0x0044_a670, &args![buffer]).u32();
            e.mem.set_u8(buffer + length.wrapping_sub(1), 0);
        }
        let settings = e.call(0x0040_4a70, &[]).u32();
        let mut setting = e.call(0x004f_8a30, &args![settings, buffer]).u32();
        if setting == 0 {
            e.call(0x0040_6d50, &args![buffer, 0x104u32, LANGUAGE_SUFFIX]);
            let owner = e.call(0x0044_f560, &[]).u32();
            setting = e.call(0x005e_02b0, &args![owner, buffer]).u32();
        }
        if setting == 0 {
            return false;
        }
        match e.mem.u8(buffer) {
            b's' | b'S' => {
                let text = e.call(0x0040_3df0, &args![setting]).u32();
                e.call(0x0040_46f0, &args![dest, text]);
            }
            b'i' | b'I' => {
                let slot = e.call(0x0043_d4d0, &args![setting]).u32();
                let value = e.mem.u32(slot);
                e.call(0x00ec_623a, &args![buffer, INTEGER_FORMAT, value]);
                e.call(0x0040_46f0, &args![dest, buffer]);
            }
            b'f' | b'F' => {
                let slot = e.call(0x0040_3e20, &args![setting]).u32();
                let value = e.mem.f32(slot) as f64;
                e.call(0x00ec_623a, &args![buffer, INTEGER_FORMAT, value]);
                e.call(0x0040_46f0, &args![dest, buffer]);
            }
            b'b' | b'B' => {
                let slot = e.call(0x0040_8d60, &args![setting]).u32();
                let text = if e.mem.u8(slot) == 1 {
                    TRUE_TEXT
                } else {
                    FALSE_TEXT
                };
                e.call(0x0040_46f0, &args![dest, text]);
            }
            _ => {}
        }
        true
    })
}

// Translated from 00707640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ReleaseModelFile` (Xbox PDB): when the model loader
/// (`011c3b3c`) exists, `0045a5e0(loader, model)` (releases the model).
pub fn interface_release_model_file(e: &mut Engine, model: u32) {
    let loader = e.mem.u32(MODEL_LOADER);
    if loader != 0 {
        e.call(0x0045_a5e0, &args![loader, model]);
    }
}

// Translated from 00707660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::LoadModelFile` (Xbox PDB): loads the model file `path` and
/// returns the model (0 when there is none). The byte at `loaded_by_loader`
/// is set to 1 when the model loader (`011c3b3c`) did it.
///
/// With a loader and the byte `011d8908` clear, `ModelLoader::LoadFile`
/// (`00447080`) does everything. Otherwise, if the file exists
/// (`FileFinder::Exist`, `00456a20`) it is read into a `BSStream` on the
/// stack; the previous temporary model in `011d890c` is destroyed, a new
/// 0x10-byte `Model` (`0043aaf0`) is built from the stream, kept in
/// `011d890c`, and `0043b230` on it gives the result. The compiler's
/// exception-unwinding frame and stack cookie are not translated.
pub fn interface_load_model_file(e: &mut Engine, path: u32, loaded_by_loader: u32) -> u32 {
    e.mem.set_u8(loaded_by_loader, 0);
    let loader = e.mem.u32(MODEL_LOADER);
    if loader != 0 && e.mem.u8(LOADER_DISABLED) == 0 {
        e.mem.set_u8(loaded_by_loader, 1);
        return e
            .call(
                0x0044_7080,
                &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
    }
    if e.call(0x0045_6a20, &args![path, 0u32, 0u32, -1i32]).u32() == 0 {
        return 0;
    }
    e.with_stack(BS_STREAM_SIZE, |e, stream| {
        let stream = stream.addr();
        e.call(0x0043_cfd0, &args![stream]);
        let loaded = e.call(0x00c3_a8a0, &args![stream, path, 0u32]).bool();
        if !loaded {
            e.call(0x0043_d090, &args![stream]);
            return 0;
        }
        let previous = e.mem.u32(TEMP_MODEL);
        if previous != 0 {
            e.call(0x0044_31f0, &args![previous, 1u32]);
        }
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let model = if block != 0 {
            e.call(0x0043_aaf0, &args![block, path, stream, 1u32, 0u32])
                .u32()
        } else {
            0
        };
        e.mem.set_u32(TEMP_MODEL, model);
        let result = e.call(0x0043_b230, &args![model]).u32();
        e.call(0x0043_d090, &args![stream]);
        result
    })
}

// Translated from 00707820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ClearTempModel` (Xbox PDB): destroys the model in `011d890c`
/// (`004431f0(model, 1)`) if there is one and clears the pointer.
pub fn interface_clear_temp_model(e: &mut Engine) {
    let model = e.mem.u32(TEMP_MODEL);
    if model != 0 {
        e.call(0x0044_31f0, &args![model, 1u32]);
    }
    e.mem.set_u32(TEMP_MODEL, 0);
}

// Translated from 00707860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `0x101dccc` of a string in the exe (`"Meshes"`).
pub fn fn_00707860(_e: &mut Engine) -> u32 {
    0x0101_dccc
}

// Translated from 00707870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CopyOrDeepCopyNode` (Xbox PDB): copies the node, with a
/// cloning-process object (`004ad050`, scale 1.0, destroyed by `004ad270`)
/// on the stack. A node `004b5bf0` accepts is copied by
/// `TES::CreateDeepCopySameTextures` (`00457ba0`, on the object in
/// `011dea10`); any other by `NiObject::Clone` (`00a5d2c0`). The compiler's
/// exception-unwinding frame is not translated.
pub fn interface_copy_or_deep_copy_node(e: &mut Engine, node: u32) -> u32 {
    e.with_stack(CLONING_PROCESS_SIZE, |e, process| {
        let process = process.addr();
        e.call(0x004a_d050, &args![process, 1.0f32]);
        let copy = if e.call(0x004b_5bf0, &args![node]).bool() {
            let owner = e.mem.u32(DEEP_COPY_OWNER);
            e.call(0x0045_7ba0, &args![owner, node, process]).u32()
        } else {
            e.call(0x00a5_d2c0, &args![node, process]).u32()
        };
        e.call(0x004a_d270, &args![process]);
        copy
    })
}

/// `InterfaceManager::IsInPipboyMenu` (Xbox PDB), static, returns a byte.
const MANAGER_IS_IN_PIPBOY_MENU: u32 = 0x0071_78a0;
/// `InterfaceManager::IsCurrentRenderedMenuTopmost` (Xbox PDB), static.
const MANAGER_IS_CURRENT_RENDERED_MENU_TOPMOST: u32 = 0x0071_7990;
/// `Interface::IsInPipboyMenu` (Xbox PDB), returns a byte.
const INTERFACE_IS_IN_PIPBOY_MENU: u32 = 0x0070_5a00;
/// `Interface::GetPipboy` (Xbox PDB).
const INTERFACE_GET_PIPBOY: u32 = 0x0070_5990;
/// `00974d90` on the Pipboy manager: returns a byte.
const PIPBOY_QUERY: u32 = 0x0097_4d90;
/// `00726070`: returns the word at `+4` of its object (a list's first node,
/// an item's count, a menu's tile).
const WORD_AT_4: u32 = 0x0072_6070;
/// `MenuManager::Instance` (Xbox PDB), `cdecl`: the create flag.
const MENU_MANAGER_INSTANCE: u32 = 0x0071_e290;
/// `MenuManager::CreateMenuByClass` (Xbox PDB), on the menu manager: the
/// menu class id.
const MENU_MANAGER_CREATE_MENU_BY_CLASS: u32 = 0x0071_e420;
/// `BSAnimGroupSequence::PlaySounds` (Xbox PDB), `cdecl`.
const ANIM_GROUP_PLAY_SOUNDS: u32 = 0x004e_ef00;
/// Trait ids of a menu tile that `Interface::IsRenderedMenu` reads
/// (truncated to integers): the first is compared with `0x70`, the second
/// with `0x421`, `0x41f` and `0x40c`.
const TRAIT_0FAA: u32 = 0xfaa;
const TRAIT_0FA4: u32 = 0xfa4;

/// The static guard (`011d8a4c`, bit 0) and the cached `float`
/// (`011d8a48`) of `00707b60`.
const SCREEN_RATIO_GUARD: u32 = 0x011d_8a4c;
const SCREEN_RATIO_CACHE: u32 = 0x011d_8a48;
/// `004dc200` (`renderer.cpp`): returns an integer.
const RENDERER_DIMENSION: u32 = 0x004d_c200;
/// The byte at `011c70eb`, returned by `00707b50`.
const RETURNED_FLAG_011C70EB: u32 = 0x011c_70eb;

/// `BSStringT<char>` (8 bytes: characters pointer, length word, capacity
/// word): constructor, `Set(characters, 0)`, length, destructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_SET: u32 = 0x0040_37f0;
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_DESTROY: u32 = 0x0040_37d0;
/// `00438390` on a string: `Set(characters)`.
const STRING_ASSIGN: u32 = 0x0043_8390;
/// `Tile::TextToTrait` (Xbox PDB), `cdecl`: the trait id of a name.
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// Tile trait setter for a `float` (`00a012d0`): tile, trait id, value,
/// flag word.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// Tile trait setter for a string (`00a01350`): tile, trait id, characters,
/// flag word.
const TILE_SET_TEXT: u32 = 0x00a0_1350;
/// `Tile::GetChildByName` (Xbox PDB): tile, name.
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `00a08b20`, `cdecl`: finds a (nested) child of a tile by name.
const TILE_FIND_CHILD: u32 = 0x00a0_8b20;
/// `Menu::RenderTemplate` (Xbox PDB), on the menu: parent tile, template
/// name, a word stored in the menu.
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// Names (strings in the exe) `00707be0` looks up or turns into traits.
const TAB_BUTTON_TEMPLATE: u32 = 0x0106_ecb4;
const LEFT_LINE_LENGTH_NAME: u32 = 0x0106_eca4;
const BUTTON_COUNT_NAME: u32 = 0x0106_ec94;
const X_NAME: u32 = 0x0106_ec90;
/// Trait `0xfb1` of a tile (the size `00707be0` adds up).
const TRAIT_0FB1: u32 = 0xfb1;

/// `Tile::SetParent` (Xbox PDB): tile, parent, a word.
const TILE_SET_PARENT: u32 = 0x00a0_87d0;
/// `Menu::SetMenuTile` (Xbox PDB), on the menu: tile, a word.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `007ab690`: constructor of the menu object (vtable `010757fc`) that
/// `HandleQueuedMenuOpen` builds for queued menu kind 9 (the tile it is given
/// is named "Player Name Entry Menu").
const NAME_ENTRY_MENU_CONSTRUCT: u32 = 0x007a_b690;
/// `00409480` on the tile: takes that menu.
const NAME_ENTRY_MENU_FINISH: u32 = 0x0040_9480;
/// The tile name `HandleQueuedMenuOpen` gives queued menu kind 9.
const NAME_ENTRY_MENU_NAME: u32 = 0x0106_ed28;
/// `Interface::CreateContainerMenu` (Xbox PDB), `cdecl`: two words.
const CREATE_CONTAINER_MENU: u32 = 0x0070_4ed0;
/// `Interface::CreateDialogMenu` (Xbox PDB), `cdecl`: three words.
const CREATE_DIALOG_MENU: u32 = 0x0070_5070;
/// `007023a0` on the manager: a flag that blocks dialog menus.
const DIALOG_BLOCKED: u32 = 0x0070_23a0;
/// `RaceSexMenu::Create` (Xbox PDB), `cdecl`: one word.
const RACE_SEX_MENU_CREATE: u32 = 0x007a_c730;
/// `StartMenu::Create` (Xbox PDB), `cdecl`: two words.
const START_MENU_CREATE: u32 = 0x007c_b7d0;
/// `HUDMainMenu::SetMenuMode` (Xbox PDB), `cdecl`: the mode.
const HUD_SET_MENU_MODE: u32 = 0x0077_1700;
/// `MessageMenu::Create` (Xbox PDB), no arguments.
const MESSAGE_MENU_CREATE: u32 = 0x007a_8ba0;
/// `CompanionWheelMenu::CreateCompanionWheelMenu` (Xbox PDB), `cdecl`.
const COMPANION_WHEEL_MENU_CREATE: u32 = 0x0075_4cf0;
/// Source and target type descriptors of the cast made for queued menu
/// kind 8.
const COMPANION_CAST_SOURCE: u32 = 0x0118_41cc;
const COMPANION_CAST_TARGET: u32 = 0x0118_46d4;
/// `008c7aa0`: returns a byte; `QueueMenuCreate` opens the menu at once only
/// when it is clear.
const QUEUE_MENU_BLOCKED: u32 = 0x008c_7aa0;
/// The lock (`011d89c0`) held while the queue is handled: `0040fbf0`
/// enter (one more word, 0), `0040fba0` leave.
const QUEUE_LOCK: u32 = 0x011d_89c0;
const QUEUE_LOCK_ENTER: u32 = 0x0040_fbf0;
const QUEUE_LOCK_LEAVE: u32 = 0x0040_fba0;
/// The first queued menu: kind (`011d8950`), then its four words at
/// `+4`, `+8`, `+0xc` and `+0x14` (`+0x10` is written elsewhere).
const QUEUE_KIND: u32 = 0x011d_8950;
const QUEUE_FIRST: u32 = 0x011d_8954;
const QUEUE_SECOND: u32 = 0x011d_8958;
const QUEUE_THIRD: u32 = 0x011d_895c;
const QUEUE_RACE_SEX: u32 = 0x011d_8960;
const QUEUE_FOURTH: u32 = 0x011d_8964;
/// The second queued menu (only kind 7, the message menu): `011d8998`, its
/// first word at `011d899c`, then `011d89a0`, `011d89a4` and `011d89ac`.
const MESSAGE_QUEUE_KIND: u32 = 0x011d_8998;
const MESSAGE_QUEUE_FIRST: u32 = 0x011d_899c;
const MESSAGE_QUEUE_SECOND: u32 = 0x011d_89a0;
const MESSAGE_QUEUE_THIRD: u32 = 0x011d_89a4;
const MESSAGE_QUEUE_FOURTH: u32 = 0x011d_89ac;

/// Vtables of `Tile` (`0106ed9c`), `TileRect` (`0106ed70`) and `TileMenu`
/// (`0106ed44`) (RTTI `.?AVTile@@`, `.?AVTileRect@@`, `.?AVTileMenu@@`).
const TILE_VTABLE: u32 = 0x0106_ed9c;
const TILE_RECT_VTABLE: u32 = 0x0106_ed70;
const TILE_MENU_VTABLE: u32 = 0x0106_ed44;
/// `Tile::~Tile` (Xbox PDB), `Tile::Release` (Xbox PDB), and
/// `00a1eff0` (the `TileMenu` destructor, `tilemenu.cpp`).
const TILE_DESTRUCTOR: u32 = 0x009f_f340;
const TILE_RELEASE: u32 = 0x009f_f690;
const TILE_MENU_DESTRUCTOR: u32 = 0x00a1_eff0;
/// `operator delete` (`00401030`), `cdecl`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Constructors the `Tile` constructor runs on its members: `0048f200`
/// (at `+4`), `00709e80` (at `+0x10`), the string constructor (at
/// `+0x20`), and `00633c90` and `0066b0d0` (both at `+0x2c`, one word 0).
const TILE_MEMBER_AT_4: u32 = 0x0048_f200;
const TILE_MEMBER_AT_10: u32 = 0x0070_9e80;
const TILE_MEMBER_AT_2C_FIRST: u32 = 0x0063_3c90;
const TILE_MEMBER_AT_2C_SECOND: u32 = 0x0066_b0d0;
/// The `float` at `01016264`, returned by `00709ac0`.
const RETURNED_SCALE_01016264: u32 = 0x0101_6264;
/// Static menu functions `00709ae0` .. `00709b40` forward to.
const SURGERY_MENU_FN: u32 = 0x007e_47a0;
const LOCKPICK_MENU_FN: u32 = 0x0078_e7c0;
const SLOT_MACHINE_MENU_FN: u32 = 0x007c_19c0;
const BLACKJACK_MENU_FN: u32 = 0x0073_3500;
const ROULETTE_MENU_FN: u32 = 0x007b_bcf0;
const CARAVAN_MENU_FN: u32 = 0x0074_0f30;
const HUD_MAIN_MENU_FN: u32 = 0x0077_f0d0;

/// Item stats display (`00707e30`): the exe's strings and constants, and the
/// scratch frame the game keeps on the stack (the `float` the entry-point
/// calls fill, and the text buffers).
const TITLE_NAME: u32 = 0x0106_ed20;
const VALUE_NAME: u32 = 0x0106_ed18;
const CONDITION_ARROWS_NAME: u32 = 0x0106_ed0c;
const CONDITION_METER_NAME: u32 = 0x0106_ecfc;
const STATS_FRAME_SIZE: u32 = 0x400;
const FRAME_CURSOR: u32 = 0x000;
const FRAME_ENTRY_OUT: u32 = 0x004;
const FRAME_VALUE_TEXT: u32 = 0x010;
const FRAME_WEIGHT_TEXT: u32 = 0x020;
const FRAME_ARMOR_TEXT: u32 = 0x040;
const FRAME_WEAPON_TEXT: u32 = 0x0c0;
const FRAME_MOD_TEXT: u32 = 0x140;
const FRAME_AMMO_TEXT: u32 = 0x1d0;
const FRAME_EFFECT_TEXT: u32 = 0x2e0;
/// The player object (a pointer global).
const PLAYER: u32 = 0x011d_ea3c;
/// `ItemChange` (Xbox PDB name of the `004bd400` family; the entry of an
/// inventory list) getters: value, form (`+8`), health, mod slots, ownership,
/// a mod effect flag, the mod item in a slot (on the form).
const ITEM_GET_VALUE: u32 = 0x004b_d400;
const ITEM_GET_FORM: u32 = 0x0044_ddc0;
const ITEM_GET_HEALTH: u32 = 0x004b_cdb0;
const ITEM_GET_MOD_SLOTS: u32 = 0x004b_d820;
const ITEM_GET_OWNERSHIP: u32 = 0x004b_d740;
const ITEM_HAS_MOD_EFFECT: u32 = 0x004b_da70;
const ITEM_FORM_GET_MOD: u32 = 0x004b_d570;
/// `004d1360` on the player: a flag passed to `TESWeightForm::GetFormWeight`.
const PLAYER_WEIGHT_FLAG: u32 = 0x004d_1360;
/// `TESWeightForm::GetFormWeight` (Xbox PDB), `cdecl`: form, flag.
const GET_FORM_WEIGHT: u32 = 0x0048_ebc0;
/// The form's type byte (`00401170`).
const FORM_GET_TYPE: u32 = 0x0040_1170;
/// `00683520`: list iteration (list, address of the cursor word); returns the
/// address of the current item and advances the cursor.
const LIST_ITERATE: u32 = 0x0068_3520;
/// `00406d00`: `sprintf`-style (destination, size, format, ...), and the
/// strlen `0044a670`.
const FORMAT_TEXT: u32 = 0x0040_6d00;
const STRING_LENGTH_C: u32 = 0x0044_a670;
/// `00403df0` on a game setting object: its string.
const SETTING_GET_STRING: u32 = 0x0040_3df0;
/// `00403e20` on a game setting object: the address of its `float`.
const FLOAT_SETTING_GET: u32 = 0x0040_3e20;
/// `006815c0`: returns its `this`.
const IDENTITY: u32 = 0x0068_15c0;
/// `00408da0` on a form's name string at `+0x30`: the characters.
const GET_NAME: u32 = 0x0040_8da0;
/// `0040ebd0`: the smaller of two `float`s.
const MIN_FLOAT: u32 = 0x0040_ebd0;
/// `004bd510(value, divisor)` (`cdecl`, two `float`s, result in `ST0`): with
/// `q = value / divisor` and `n` its truncation, `(n + 1 if q - n >= 0.5
/// else n) * divisor`.
const ROUND_DIVIDE: u32 = 0x004b_d510;
/// `00476b20` (`cdecl`): one `float` in, a `float` out (it calls `00404090`).
const ROUND_FLOAT: u32 = 0x0047_6b20;
/// `TESHealthForm::GetFormHealth` (Xbox PDB), `cdecl`.
const GET_FORM_HEALTH: u32 = 0x0048_73d0;
/// `CombatFormulas::CalcArmorRating` (Xbox PDB), `cdecl`: integer, `float`.
const CALC_ARMOR_RATING: u32 = 0x0064_6360;
/// `004be060` and `004be180` on an armor form: the two values (`ST0`) whose
/// truncation `CalcArmorRating` takes.
const ARMOR_RATING_INPUT_A: u32 = 0x004b_e060;
const ARMOR_RATING_INPUT_B: u32 = 0x004b_e180;
/// Source and target type descriptors of the armor cast, and the three
/// pointer globals to the game settings that title the armor cell.
const ARMOR_CAST_SOURCE: u32 = 0x0118_3108;
const ARMOR_CAST_TARGET: u32 = 0x0118_3a34;
const ARMOR_SETTING_A: u32 = 0x0118_a644;
const ARMOR_SETTING_B: u32 = 0x0118_a648;
const ARMOR_SETTING_C: u32 = 0x0118_a640;
/// Flag getters on the cast object's sub-object at `+0x70` (byte at `+8`,
/// tested with `0x80` and `0x08`).
const ARMOR_SUBOBJECT_FLAG_80: u32 = 0x004c_0bd0;
const ARMOR_SUBOBJECT_FLAG_08: u32 = 0x0051_4450;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), `cdecl`: entry point id,
/// the player, then the form (some entry points) and the address of the
/// `float` it updates.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `TESObjectWEAP::GetFormWeight` (Xbox PDB), on the weapon: a flag.
const WEAPON_GET_WEIGHT: u32 = 0x004b_e380;
/// `00663b60` on the weapon: the word at `+0x19c`.
const WEAPON_FIELD_19C: u32 = 0x0066_3b60;
/// `00645380` (weapon damage, `cdecl`, 13 words) and
/// `CombatFormulas::CalcWeaponDamageForDisplay` (Xbox PDB, `cdecl`).
const CALC_WEAPON_DAMAGE: u32 = 0x0064_5380;
const CALC_WEAPON_DAMAGE_FOR_DISPLAY: u32 = 0x0064_50f0;
/// `TESObjectWEAP::GetNumProjectiles`, `GetCurrentAmmo`,
/// `GetFormClipRounds` and `GetAmmoRegenRate` (Xbox PDB), on the weapon.
const WEAPON_GET_NUM_PROJECTILES: u32 = 0x0052_5b20;
const WEAPON_GET_CURRENT_AMMO: u32 = 0x0052_5980;
const WEAPON_GET_CLIP_ROUNDS: u32 = 0x004f_e160;
const WEAPON_GET_AMMO_REGEN_RATE: u32 = 0x0070_9430;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB), on the player.
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `00663b40` on an ammo form: its name characters (word at `+0xc4`).
const AMMO_NAME: u32 = 0x0066_3b40;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), `cdecl`: the player;
/// `InventoryChanges::GetObjectCount` (Xbox PDB), on it: a form.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const INVENTORY_GET_COUNT: u32 = 0x004c_8f30;
/// `TESAmmo::BuildMenuString` (Xbox PDB) and `EffectItemList::BuildMenuString`
/// (Xbox PDB): buffer, size.
const AMMO_BUILD_MENU_STRING: u32 = 0x0050_3a70;
const EFFECT_LIST_BUILD_MENU_STRING: u32 = 0x0040_6620;
/// `TESEnchantableForm::GetFormEnchanting` (Xbox PDB), `cdecl`, and the flag
/// test `0040e210` on the enchantment (one word).
const GET_FORM_ENCHANTING: u32 = 0x004b_e330;
const SPELL_ITEM_FLAG: u32 = 0x0040_e210;
/// The record tag `'DESC'` passed to the description getter of a form of
/// type `0x67` (slot `0x10` of the sub-object at `+0x74`).
const DESCRIPTION_TAG: u32 = 0x4353_4544;
/// Game setting objects and their strings used by the item stats display.
const SETTING_MODS_TITLE: u32 = 0x011d_4024;
const LANGUAGE_SETTING: u32 = 0x011c_3cb4;
const WEIGHT_LIMIT_SETTING: u32 = 0x011c_6478;
const WEIGHT_FACTOR_SETTING: u32 = 0x011c_64a8;
/// `00408b20(a, b)`: string comparison (0 when equal).
const COMPARE_TEXT: u32 = 0x0040_8b20;
const ENGLISH_TEXT: u32 = 0x0101_8348;
/// Constants in the exe: `100.0`, `10.0`, `41.0`, `1.0`, `2.0`, `20.0`
/// (doubles) and `255.0` (`float`).
const PERCENT_DIVISOR: u32 = 0x0101_7a40;
const HEAVY_WEIGHT: u32 = 0x0102_0758;
const MOD_ROW_HEIGHT: u32 = 0x0103_5808;
const ONE: u32 = 0x0101_2070;
const TWO: u32 = 0x0101_1590;
const TWENTY: u32 = 0x0102_fc70;
const CELL_10_TRAIT_FA9: u32 = 0x0102_3cd8;
/// Texts in the exe: "--", "%d", "%s", "%s (%i/%i)", "%.1fx%d", "%.0f",
/// "%.1f" and "Mod 1".."Mod 3".
const DASHES: u32 = 0x0104_7008;
const FORMAT_INTEGER: u32 = 0x0102_0764;
const FORMAT_STRING: u32 = 0x0101_9f08;
const FORMAT_AMMO: u32 = 0x0106_ece8;
const FORMAT_PROJECTILES: u32 = 0x0106_ecf4;
const FORMAT_NO_DECIMAL: u32 = 0x0106_ecc8;
const FORMAT_ONE_DECIMAL: u32 = 0x0102_0768;
const MOD_1_TEXT: u32 = 0x0106_ece0;
const MOD_2_TEXT: u32 = 0x0106_ecd8;
const MOD_3_TEXT: u32 = 0x0106_ecd0;
/// Trait ids of the item stats cells: visibility (`0xfa3`), two layout
/// traits and the one the display copies from its condition meter to the
/// condition arrows (`0xfb0`, `0xfa2`).
const TRAIT_0FA3: u32 = 0xfa3;
const TRAIT_0FA2: u32 = 0xfa2;
const TRAIT_0FB0: u32 = 0xfb0;
const TRAIT_0FA9: u32 = 0xfa9;
/// The game settings whose strings title the cells when the language is not
/// English, with the cell each goes to.
const LANGUAGE_TITLES: [(u32, usize); 9] = [
    (0x011d_20c4, 0),
    (0x011d_412c, 1),
    (0x011d_2c7c, 11),
    (0x011d_2aa8, 2),
    (0x011d_26a0, 3),
    (0x011d_4da4, 4),
    (0x011d_4edc, 6),
    (0x011d_4024, 7),
    (0x011d_330c, 10),
];
/// The `flags` bit that shows each cell, in the order the game sets the
/// visibility trait (cell 10 is handled separately).
const VISIBILITY_BITS: [(u32, usize); 12] = [
    (0x1, 0),
    (0x1000, 12),
    (0x2, 1),
    (0x800, 11),
    (0x4, 2),
    (0x8, 3),
    (0x10, 4),
    (0x20, 5),
    (0x40, 6),
    (0x80, 7),
    (0x100, 8),
    (0x200, 9),
];

/// The `ST0`-to-integer conversion for an `f64` (`00ec62c0`).
fn double_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// A tile trait read (`00a011b0`) as a `float`.
fn tile_float(e: &mut Engine, tile: u32, trait_id: u32) -> f32 {
    e.call(TILE_GET_VALUE, &args![tile, trait_id]).f32()
}

/// A tile trait set from an integer (`00700320` converts it to a `float`).
fn tile_set_int(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    e.call(TILE_SET_VALUE, &args![tile, trait_id, value]);
}

/// The float trait setter, with the flag word 1 every caller here passes.
fn tile_set_float(e: &mut Engine, tile: u32, trait_id: u32, value: f32) {
    e.call(TILE_SET_FLOAT, &args![tile, trait_id, value, 1u32]);
}

/// The string trait setter, with the flag word 1 every caller here passes.
fn tile_set_text(e: &mut Engine, tile: u32, trait_id: u32, text: u32) {
    e.call(TILE_SET_TEXT, &args![tile, trait_id, text, 1u32]);
}

// Translated from 00707930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::SetNeedToUpdate` (Xbox PDB): sets the manager's
/// `bNeedToUpdate` (`00707950`); it does not check that the manager is
/// ready.
pub fn interface_set_need_to_update(e: &mut Engine, value: u8) {
    let this = manager(e);
    fn_00707950(e, this, value);
}

// Translated from 00707950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the manager's byte at `+0xb0` (`bNeedToUpdate`,
/// Xbox PDB).
pub fn fn_00707950(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0xb0, value);
}

// Translated from 00707970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`: `BSAnimGroupSequence::PlaySounds(sequence, 0)` (Xbox PDB name of
/// `004eef00`), whose answer it answers (the exe ends on that call and leaves
/// its `EAX`; the caller tests `AL`).
pub fn fn_00707970(e: &mut Engine, sequence: u32) -> bool {
    e.call(ANIM_GROUP_PLAY_SOUNDS, &args![sequence, 0u32])
        .bool()
}

// Translated from 00707990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::CreateMenuByClass` (Xbox PDB; a `Menu*` there), `cdecl`: asks
/// the menu manager (`MenuManager::Instance(1)`) to create the menu of class
/// `class_id` and answers what that gives (the exe ends on that call and
/// leaves its `EAX`, which the caller keeps).
pub fn interface_create_menu_by_class(e: &mut Engine, class_id: u32) -> u32 {
    let menu_manager = e.call(MENU_MANAGER_INSTANCE, &args![1u32]).u32();
    e.call(
        MENU_MANAGER_CREATE_MENU_BY_CLASS,
        &args![menu_manager, class_id],
    )
    .u32()
}

// Translated from 007079b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsInRenderedMenu` (Xbox PDB): when the Pipboy menu is open
/// and `00974d90` on the Pipboy manager says no, false; otherwise
/// [`fn_007079f0`] on the interface manager.
pub fn interface_is_in_rendered_menu(e: &mut Engine) -> bool {
    if e.call(INTERFACE_IS_IN_PIPBOY_MENU, &[]).u8() != 0 {
        let pipboy = e.call(INTERFACE_GET_PIPBOY, &[]).u32();
        if e.call(PIPBOY_QUERY, &args![pipboy]).u8() == 0 {
            return false;
        }
    }
    let this = manager(e);
    fn_007079f0(e, this)
}

// Translated from 007079f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's `bIsInRenderedMenu` (Xbox PDB, byte at `+0x168`) is set or
/// `InterfaceManager::IsInPipboyMenu` (`007178a0`) says so.
pub fn fn_007079f0(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 0x168) != 0 || e.call(MANAGER_IS_IN_PIPBOY_MENU, &[]).u8() != 0
}

// Translated from 00707a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsRenderedMenu` (Xbox PDB), `cdecl`: false for no menu or a
/// menu without a tile (the word at `+4`, `00726070`); otherwise true when
/// the tile's trait `0xfaa` (as an integer) is `0x70` or its trait `0xfa4`
/// is `0x421`, `0x41f` or `0x40c`.
pub fn interface_is_rendered_menu(e: &mut Engine, menu: u32) -> bool {
    let tile = if menu != 0 {
        e.call(WORD_AT_4, &args![menu]).u32()
    } else {
        0
    };
    if tile == 0 {
        return false;
    }
    let first = tile_float(e, tile, TRAIT_0FAA);
    let first = float_to_int(e, first);
    let second = tile_float(e, tile, TRAIT_0FA4);
    let second = float_to_int(e, second);
    first == 0x70 || second == 0x421 || second == 0x41f || second == 0x40c
}

// Translated from 00707ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::GetCurrentRenderedMenu` (Xbox PDB): [`fn_00707af0`] on the
/// interface manager.
pub fn interface_get_current_rendered_menu(e: &mut Engine) -> u32 {
    let this = manager(e);
    fn_00707af0(e, this)
}

// Translated from 00707af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's `pPipboy` (Xbox PDB, `+0x174`) when
/// `InterfaceManager::IsInPipboyMenu` (`007178a0`) says so, otherwise its
/// `pCurrentRenderedMenu` (`+0x16c`).
pub fn fn_00707af0(e: &mut Engine, this: u32) -> u32 {
    if e.call(MANAGER_IS_IN_PIPBOY_MENU, &[]).u8() != 0 {
        e.mem.u32(this + 0x174)
    } else {
        e.mem.u32(this + 0x16c)
    }
}

// Translated from 00707b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::IsCurrentRenderedMenuTopmost` (Xbox PDB): forwards to
/// `InterfaceManager::IsCurrentRenderedMenuTopmost` (`00717990`, static).
/// The game first calls `004b7210` (the manager getter) and ignores the
/// result.
pub fn interface_is_current_rendered_menu_topmost(e: &mut Engine) -> bool {
    manager(e);
    e.call(MANAGER_IS_CURRENT_RENDERED_MENU_TOPMOST, &[]).u8() != 0
}

// Translated from 00707b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns [`fn_00707b50`].
pub fn fn_00707b40(e: &mut Engine) -> u8 {
    fn_00707b50(e)
}

// Translated from 00707b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte global at `011c70eb`.
pub fn fn_00707b50(e: &mut Engine) -> u8 {
    e.global::<u8>(RETURNED_FLAG_011C70EB)
}

// Translated from 00707b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function-local static `float`: the first call sets the guard bit
/// (`011d8a4c`) and stores `Interface::GetScreenHeight()` divided by the
/// integer `004dc200` returns (`011d8a48`); every call returns the stored
/// value. The compiler's exception-unwinding frame is not translated.
pub fn fn_00707b60(e: &mut Engine) -> f32 {
    if e.global::<u32>(SCREEN_RATIO_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(SCREEN_RATIO_GUARD);
        e.set_global(SCREEN_RATIO_GUARD, guard | 1);
        let height = interface_get_screen_height(e);
        let divisor = e.call(RENDERER_DIMENSION, &[]).i32();
        e.set_global(SCREEN_RATIO_CACHE, (height as f64 / divisor as f64) as f32);
    }
    e.global::<f32>(SCREEN_RATIO_CACHE)
}

// Translated from 00707be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::InitializeTabline` (Xbox PDB), `cdecl`, varargs: lays out a
/// tab line on `tile`. `first_name` and the words of `more_names` are the
/// names (character pointers) of the tab buttons; the list ends at a null
/// pointer (or at the end of `more_names`, where the game would read the
/// next stack word).
///
/// For each name a "TabButtonTemplate" is rendered into `tile` by its menu
/// (`Menu::RenderTemplate`), given the name as its trait `0xfc4` text and
/// as its tile name (`00707e10`), the trait `0xfac` = its index and trait
/// `0xfaa` = `base_index` + its index, then updated with
/// `Tile::UpdateAll(0)`; the sizes (trait `0xfb1`, truncated) are added up.
/// The line length left over, `(tile's 0xfb1 - sum) / (count + 1)`
/// truncated, is stored as `_LeftLineLength` and the count as `_ButtonCount`
/// on `tile`. A second pass sets `_x` of each child found by name: the first
/// at the left line length, each next at the previous plus its size plus the
/// left line length. The temporary `BSStringT` is built and destroyed on the
/// stack; the compiler's exception-unwinding frame is not translated.
pub fn interface_initialize_tabline(
    e: &mut Engine,
    tile: u32,
    base_index: u32,
    first_name: u32,
    more_names: &[u32],
) {
    let name_at = |index: usize| -> u32 {
        if index == 0 {
            first_name
        } else {
            more_names.get(index - 1).copied().unwrap_or(0)
        }
    };
    e.with_stack(8, |e, string| {
        let string = string.addr();
        e.call(STRING_CONSTRUCT, &args![string]);
        e.call(STRING_SET, &args![string, first_name, 0u32]);
        let mut count: u32 = 0;
        let mut total: i32 = 0;
        while e.call(STRING_LENGTH, &args![string]).u32() != 0 {
            let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            let button = e
                .call(
                    MENU_RENDER_TEMPLATE,
                    &args![menu, tile, TAB_BUTTON_TEMPLATE, 0u32],
                )
                .u32();
            let characters = e.call(FIRST_WORD, &args![string]).u32();
            tile_set_text(e, button, 0xfc4, characters);
            let characters = e.call(FIRST_WORD, &args![string]).u32();
            fn_00707e10(e, button, characters);
            tile_set_int(e, button, 0xfac, count);
            tile_set_int(e, button, 0xfaa, base_index.wrapping_add(count));
            e.call(TILE_UPDATE_ALL, &args![0u32]);
            let size = tile_float(e, button, TRAIT_0FB1);
            total = total.wrapping_add(float_to_int(e, size));
            count += 1;
            e.call(STRING_ASSIGN, &args![string, name_at(count as usize)]);
        }
        let line_size = tile_float(e, tile, TRAIT_0FB1) as f64;
        let share = (line_size - total as f64) / (count as f64 + 1.0);
        let left_line_length = double_to_int(e, share);
        let trait_id = e
            .call(TILE_TEXT_TO_TRAIT, &args![LEFT_LINE_LENGTH_NAME])
            .u32();
        tile_set_int(e, tile, trait_id, left_line_length as u32);
        let trait_id = e.call(TILE_TEXT_TO_TRAIT, &args![BUTTON_COUNT_NAME]).u32();
        tile_set_int(e, tile, trait_id, count);

        e.call(STRING_SET, &args![string, first_name, 0u32]);
        let mut position = left_line_length;
        let mut index = 0usize;
        while e.call(STRING_LENGTH, &args![string]).u32() != 0 {
            let characters = e.call(FIRST_WORD, &args![string]).u32();
            let child = e
                .call(TILE_GET_CHILD_BY_NAME, &args![tile, characters])
                .u32();
            let trait_id = e.call(TILE_TEXT_TO_TRAIT, &args![X_NAME]).u32();
            tile_set_int(e, child, trait_id, position as u32);
            let size = tile_float(e, child, TRAIT_0FB1);
            position = float_to_int(e, size)
                .wrapping_add(left_line_length)
                .wrapping_add(position);
            index += 1;
            e.call(STRING_ASSIGN, &args![string, name_at(index)]);
        }
        e.call(STRING_DESTROY, &args![string]);
    });
}

/// The registered form of `00707be0`: the tile, the base index, the first
/// name, then the further names.
fn interface_initialize_tabline_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 3,
        "00707be0 needs the tile, the base index and the first name"
    );
    interface_initialize_tabline(e, words[0], words[1], words[2], &words[3..]);
    Ret::default()
}

// Translated from 00707e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `name` to the string at `+0x20` of `this` (a tile's name):
/// `00438390`.
pub fn fn_00707e10(e: &mut Engine, this: u32, name: u32) {
    e.call(STRING_ASSIGN, &args![this + 0x20, name]);
}

// Translated from 00707e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::PopulateItemStatsDisplay` (Xbox PDB), `cdecl`: fills the item
/// stats display tile `display` for the inventory entry `item` (an
/// `ItemChange`, Xbox PDB name of the `004bd400` family: `+4` its count, `+8`
/// its form). Does nothing for a null `display` (a null `item` is
/// dereferenced before that check).
///
/// The first 13 children of `display` (list at `+4`) are the cells,
/// `cells[0..13]`; `flags` collects which of them are shown (see
/// `VISIBILITY_BITS`). The form's type byte (`00401170`) selects what is
/// shown:
///
/// * `0x18` and `0x1a`: cell 4 gets trait `0x100a` = 0; the condition
///   `GetItemHealth(1) / 100` and the two armor values (`004be060`,
///   `004be180`, truncated) go through `CombatFormulas::CalcArmorRating`
///   and are shown in cells 0 and 12 (or "--" in cell 0 when both are 0);
///   the title of cell 5 is the string of one of three game settings chosen
///   by two flag bytes of the cast object's sub-object at `+0x70`; cell
///   4's trait `0x1009` gets `GetItemHealth(0)` over the form's health
///   (`TESHealthForm::GetFormHealth`, or 1), at most 1. `flags` = `0x3c`,
///   plus `1` and `0x1000` for the cells 0 and 12.
/// * `0x28` (weapon): cell 4 gets `0x100a` = 1; the damage
///   (`00645380`, cell 1), the damage per shot
///   (`CalcWeaponDamageForDisplay`, cell 11, as "%.1fx%d" when the weapon
///   fires several projectiles), the weapon's word at `+0x19c` less the
///   truncated entry point `0x35` (cell 10), the ammunition text with its
///   loaded and spare counts (cell 5) and the weight, with entry point
///   `0x49` for weapons of at least 10 (the constant at `01020758`).
///   `flags` = `0xc3e`.
/// * `0x29` (ammo): `TESAmmo::BuildMenuString` fills cell 6 when it is not
///   empty; `flags` = `0xc` (plus `0x40` when cell 6 is filled).
/// * anything else: `flags` = `0xc`.
///
/// Then the effect list of the form (types `0x1d`, `0x2f`, or an
/// enchantment the form has) gives the text of cell 6 (`0x40`), a form of
/// type `0x67` its description, the mod slots fill the cells 7..9 (`0x80`,
/// `0x100`, `0x200`) and move the heights of the cells below, and the value
/// (cell 3) and weight (cell 2) texts are set. When the language setting is
/// not "ENGLISH", nine cells get their title from game settings. At the end
/// every cell's trait `0xfa3` is set from `flags` and the heights (trait
/// `0xfa2`) of cells 6..9 from the display's trait `0xfb0`.
///
/// The compiler's stack-cookie check is not translated.
pub fn interface_populate_item_stats_display(e: &mut Engine, item: u32, display: u32) {
    e.with_stack(STATS_FRAME_SIZE, |e, frame| {
        populate_item_stats(e, frame.addr(), item, display);
    });
}

/// `ItemChange`'s form (`0044ddc0`, the word at `+8`).
fn item_form(e: &mut Engine, item: u32) -> u32 {
    e.call(ITEM_GET_FORM, &args![item]).u32()
}

/// The form's type byte (`00401170`).
fn form_type(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_GET_TYPE, &args![form]).u32()
}

/// A game setting's string (`00403df0` on the setting object).
fn setting_text(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_GET_STRING, &args![setting]).u32()
}

/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB) on the player.
fn saved_acquire_object(e: &mut Engine) -> u32 {
    let player = e.global::<u32>(PLAYER);
    e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32()
}

/// `00406d00(dest, size, format, word)`.
fn format_word(e: &mut Engine, dest: u32, size: u32, format: u32, word: u32) {
    e.call(FORMAT_TEXT, &args![dest, size, format, word]);
}

/// `FISTP` to a 64-bit integer with truncation, low 32 bits (what
/// `FISTP qword` leaves when read as a dword): NaN and out-of-range values
/// give the "integer indefinite" `0x8000000000000000`.
fn truncate_low_dword(value: f32) -> i32 {
    let value = value as f64;
    if value.is_nan()
        || !(-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        0
    } else {
        (value as i64) as i32
    }
}

/// The body of `00707e30` with the scratch `frame` the game keeps on its
/// stack.
fn populate_item_stats(e: &mut Engine, frame: u32, item: u32, display: u32) {
    let cursor = frame + FRAME_CURSOR;
    let entry_out = frame + FRAME_ENTRY_OUT;
    let armor_text = frame + FRAME_ARMOR_TEXT;
    let weapon_text = frame + FRAME_WEAPON_TEXT;
    let ammo_text = frame + FRAME_AMMO_TEXT;
    let effect_text = frame + FRAME_EFFECT_TEXT;
    let mod_text = frame + FRAME_MOD_TEXT;
    let value_text = frame + FRAME_VALUE_TEXT;
    let weight_text = frame + FRAME_WEIGHT_TEXT;

    let mut flags: u32;
    let title_trait = e.call(TILE_TEXT_TO_TRAIT, &args![TITLE_NAME]).u32();
    let value_trait = e.call(TILE_TEXT_TO_TRAIT, &args![VALUE_NAME]).u32();
    let meter_trait: u32 = 0x1009;
    let mut height_a: f32 = 0.0;
    let mut height_b: f32 = 0.0;
    let item_value = e.call(ITEM_GET_VALUE, &args![item]).f32();
    let player = e.global::<u32>(PLAYER);
    let weight_flag = e.call(PLAYER_WEIGHT_FLAG, &args![player]).bool();
    let form = item_form(e, item);
    let mut weight = e.call(GET_FORM_WEIGHT, &args![form, weight_flag]).f32();
    if item == 0 || display == 0 {
        return;
    }

    // The cells: the first 13 children of the display tile.
    let list = display + 4;
    let first_node = e.call(WORD_AT_4, &args![list]).u32();
    e.mem.set_u32(cursor, first_node);
    let mut cells = [0u32; 13];
    let mut filled = 0usize;
    while e.mem.u32(cursor) != 0 && filled < 13 {
        let slot = e.call(LIST_ITERATE, &args![list, cursor]).u32();
        cells[filled] = e.mem.u32(slot);
        filled += 1;
    }

    let arrows = e
        .call(TILE_FIND_CHILD, &args![display, CONDITION_ARROWS_NAME])
        .u32();
    let meter = e
        .call(TILE_FIND_CHILD, &args![display, CONDITION_METER_NAME])
        .u32();
    let value = tile_float(e, meter, TRAIT_0FB0);
    tile_set_float(e, arrows, TRAIT_0FB0, value);
    let value = tile_float(e, meter, TRAIT_0FA2);
    tile_set_float(e, arrows, TRAIT_0FA2, value);
    let has_mods = e.call(ITEM_GET_MOD_SLOTS, &args![item]).u8() != 0;

    let form = item_form(e, item);
    let kind = form_type(e, form);
    match kind {
        0x18 | 0x1a => {
            tile_set_int(e, cells[4], 0x100a, 0);
            let condition = (e.call(ITEM_GET_HEALTH, &args![item, 1u32]).f32() as f64
                / e.global::<f64>(PERCENT_DIVISOR)) as f32;
            flags = 0x3c;
            copy_text(e, armor_text, 0x80, DASHES);
            let form = item_form(e, item);
            let cast = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, ARMOR_CAST_SOURCE, ARMOR_CAST_TARGET, 0u32],
                )
                .u32();
            if cast != 0 {
                let sub_object = cast + 0x70;
                let setting_pointer = if e.call(ARMOR_SUBOBJECT_FLAG_80, &args![sub_object]).bool()
                {
                    ARMOR_SETTING_A
                } else if e.call(ARMOR_SUBOBJECT_FLAG_08, &args![sub_object]).bool() {
                    ARMOR_SETTING_B
                } else {
                    ARMOR_SETTING_C
                };
                let setting = e.global::<u32>(setting_pointer);
                let text = setting_text(e, setting);
                copy_text(e, armor_text, 0x80, text);
            }
            tile_set_text(e, cells[5], title_trait, armor_text);

            let form = item_form(e, item);
            let input = e.call(ARMOR_RATING_INPUT_A, &args![form]).f32();
            let input = float_to_int(e, input);
            let rating_a = e.call(CALC_ARMOR_RATING, &args![input, condition]).f32();
            let form = item_form(e, item);
            let input = e.call(ARMOR_RATING_INPUT_B, &args![form]).f32();
            let input = float_to_int(e, input);
            let rating_b = e.call(CALC_ARMOR_RATING, &args![input, condition]).f32();
            if rating_a == 0.0 && rating_b == 0.0 {
                flags |= 1;
                copy_text(e, armor_text, 0x80, DASHES);
                tile_set_text(e, cells[0], value_trait, armor_text);
            } else {
                if rating_a != 0.0 {
                    let rounded = e.call(ROUND_FLOAT, &args![rating_a]).f32();
                    let rounded = float_to_int(e, rounded);
                    format_word(e, armor_text, 0x80, FORMAT_INTEGER, rounded as u32);
                    tile_set_text(e, cells[0], value_trait, armor_text);
                    flags |= 1;
                }
                if rating_b != 0.0 {
                    let rounded = e.call(ROUND_FLOAT, &args![rating_b]).f32();
                    let rounded = float_to_int(e, rounded);
                    format_word(e, armor_text, 0x80, FORMAT_INTEGER, rounded as u32);
                    tile_set_text(e, cells[12], value_trait, armor_text);
                    flags |= 0x1000;
                }
            }

            let form = item_form(e, item);
            let health = e.call(GET_FORM_HEALTH, &args![form]).u32();
            let health = if health != 0 {
                let form = item_form(e, item);
                e.call(GET_FORM_HEALTH, &args![form]).u32()
            } else {
                1
            };
            let current = e.call(ITEM_GET_HEALTH, &args![item, 0u32]).f32();
            let fraction = (current as f64 / health as f64) as f32;
            let level = e.call(MIN_FLOAT, &args![1.0f32, fraction]).f32();
            tile_set_float(e, cells[4], meter_trait, level);
        }
        0x28 => {
            let weapon = item_form(e, item);
            tile_set_int(e, cells[4], 0x100a, 1);
            let mut saved_ammo = 0u32;
            let mut saved_process = 0u32;
            if saved_acquire_object(e) != 0 {
                let held = saved_acquire_object(e);
                if e.vcall(held, 0x148, &[]).u32() != 0 {
                    let held = saved_acquire_object(e);
                    let held_item = e.vcall(held, 0x148, &[]).u32();
                    let held_form = item_form(e, held_item);
                    if weapon == held_form {
                        let held = saved_acquire_object(e);
                        if e.vcall(held, 0x14c, &[]).u32() != 0 {
                            let held = saved_acquire_object(e);
                            let ammo_item = e.vcall(held, 0x14c, &[]).u32();
                            saved_ammo = item_form(e, ammo_item);
                        } else {
                            saved_ammo = 0;
                        }
                        saved_process = e.global::<u32>(PLAYER);
                    }
                }
            }
            let with_effect = e.call(ITEM_HAS_MOD_EFFECT, &args![item, 4u32]).u8();
            weight = e
                .call(WEAPON_GET_WEIGHT, &args![weapon, with_effect as u32])
                .f32();
            if weight as f64 >= e.global::<f64>(HEAVY_WEIGHT) {
                e.mem.set_f32(entry_out, 1.0);
                let player = e.global::<u32>(PLAYER);
                e.call(HANDLE_ENTRY_POINT, &args![0x49u32, player, entry_out]);
                weight = (weight as f64 * e.mem.f32(entry_out) as f64) as f32;
            }
            let condition = (e.call(ITEM_GET_HEALTH, &args![item, 1u32]).f32() as f64
                / e.global::<f64>(PERCENT_DIVISOR)) as f32;
            let mut field_19c = e.call(WEAPON_FIELD_19C, &args![weapon]).u32() as i32;
            let player = e.global::<u32>(PLAYER);
            let actor_values = if player != 0 { player + 0xa4 } else { 0 };
            let damage = e
                .call(
                    CALC_WEAPON_DAMAGE,
                    &args![
                        actor_values,
                        weapon,
                        condition,
                        1u32,
                        item,
                        0u32,
                        0u32,
                        0xffff_ffffu32,
                        0.0f32,
                        0.0f32,
                        0u32,
                        0u32,
                        saved_ammo
                    ],
                )
                .f32();
            let damage_for_display = if condition > 0.0 || condition.is_nan() {
                e.call(
                    CALC_WEAPON_DAMAGE_FOR_DISPLAY,
                    &args![weapon, condition, item, saved_ammo],
                )
                .f32()
            } else {
                0.0
            };
            let rounded = e.call(ROUND_DIVIDE, &args![damage, 1.0f32]).f32();
            let rounded = float_to_int(e, rounded);
            format_word(e, weapon_text, 0x80, FORMAT_INTEGER, rounded as u32);
            tile_set_text(e, cells[1], value_trait, weapon_text);

            let with_effect = e.call(ITEM_HAS_MOD_EFFECT, &args![item, 0xcu32]).u8();
            let projectiles = e
                .call(
                    WEAPON_GET_NUM_PROJECTILES,
                    &args![weapon, with_effect as u32, 0u32, saved_process],
                )
                .u8() as u32;
            if projectiles > 1 {
                let each = (damage_for_display as f64 / projectiles as f64) as f32;
                e.call(
                    FORMAT_TEXT,
                    &args![
                        weapon_text,
                        0x80u32,
                        FORMAT_PROJECTILES,
                        each as f64,
                        projectiles
                    ],
                );
            } else {
                let rounded = e
                    .call(ROUND_DIVIDE, &args![damage_for_display, 1.0f32])
                    .f32();
                let rounded = float_to_int(e, rounded);
                format_word(e, weapon_text, 0x80, FORMAT_INTEGER, rounded as u32);
            }
            tile_set_text(e, cells[11], value_trait, weapon_text);

            let level = e.call(MIN_FLOAT, &args![condition, 1.0f32]).f32();
            tile_set_float(e, cells[4], meter_trait, level);
            e.mem.set_f32(entry_out, 0.0);
            let player = e.global::<u32>(PLAYER);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x35u32, player, weapon, entry_out],
            );
            let reduction = truncate_low_dword(e.mem.f32(entry_out));
            field_19c = field_19c.wrapping_sub(reduction);
            if field_19c < 0 {
                field_19c = 0;
            }
            format_word(e, weapon_text, 0x80, FORMAT_INTEGER, field_19c as u32);
            if cells[10] != 0 {
                tile_set_text(e, cells[10], value_trait, weapon_text);
            }

            let ammo = e
                .call(WEAPON_GET_CURRENT_AMMO, &args![weapon, saved_process])
                .u32();
            if ammo != 0 {
                let held = saved_acquire_object(e);
                let held_item = e.vcall(held, 0x148, &[]).u32();
                let held = saved_acquire_object(e);
                let held_ammo_item = e.vcall(held, 0x14c, &[]).u32();
                let ammo = e
                    .call(WEAPON_GET_CURRENT_AMMO, &args![weapon, saved_process])
                    .u32();
                let mut name = e.call(AMMO_NAME, &args![ammo]).u32();
                let ammo = e
                    .call(WEAPON_GET_CURRENT_AMMO, &args![weapon, saved_process])
                    .u32();
                let player = e.global::<u32>(PLAYER);
                let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
                let total = e.call(INVENTORY_GET_COUNT, &args![changes, ammo]).u32();
                let mut loaded: u32 = 0;
                if name == 0 || e.mem.u8(name) == 0 {
                    let ammo = e
                        .call(WEAPON_GET_CURRENT_AMMO, &args![weapon, saved_process])
                        .u32();
                    name = e.call(GET_NAME, &args![ammo + 0x30]).u32();
                }
                let mut loaded_known = false;
                if held_item != 0
                    && e.call(FIRST_WORD, &args![held_item]).u32() != 0
                    && e.call(FIRST_WORD, &args![item]).u32() != 0
                {
                    let held_first = e.call(FIRST_WORD, &args![held_item]).u32();
                    let held_slot = e.call(IDENTITY, &args![held_first]).u32();
                    let item_first = e.call(FIRST_WORD, &args![item]).u32();
                    let item_slot = e.call(IDENTITY, &args![item_first]).u32();
                    if e.mem.u32(held_slot) == e.mem.u32(item_slot) {
                        loaded = if held_ammo_item != 0 {
                            e.call(WORD_AT_4, &args![held_ammo_item]).u32()
                        } else {
                            0
                        };
                        loaded_known = true;
                    }
                }
                if !loaded_known {
                    let with_effect = e.call(ITEM_HAS_MOD_EFFECT, &args![item, 2u32]).u8();
                    let rounds = e
                        .call(WEAPON_GET_CLIP_ROUNDS, &args![weapon, with_effect as u32])
                        .u32();
                    loaded = e.call(HELPER_4A8F20, &args![rounds, total]).u32();
                }
                let regeneration = e
                    .call(WEAPON_GET_AMMO_REGEN_RATE, &args![weapon, 0u32])
                    .f32();
                if regeneration != 0.0 {
                    format_word(e, weapon_text, 0x80, FORMAT_STRING, name);
                } else {
                    e.call(
                        FORMAT_TEXT,
                        &args![
                            weapon_text,
                            0x80u32,
                            FORMAT_AMMO,
                            name,
                            loaded,
                            total.wrapping_sub(loaded)
                        ],
                    );
                }
            } else {
                copy_text(e, weapon_text, 0x80, DASHES);
            }
            tile_set_text(e, cells[5], title_trait, weapon_text);
            flags = 0xc3e;
        }
        0x29 => {
            flags = 0xc;
            let ammo = item_form(e, item);
            e.call(AMMO_BUILD_MENU_STRING, &args![ammo, ammo_text, 0x100u32]);
            if e.call(STRING_LENGTH_C, &args![ammo_text]).u32() > 0 {
                tile_set_text(e, cells[6], value_trait, ammo_text);
                flags |= 0x40;
            } else {
                flags &= !0x40;
            }
        }
        _ => flags = 0xc,
    }

    // The effect list whose text goes to cell 6.
    let mut effects: u32 = 0;
    let form = item_form(e, item);
    if form_type(e, form) == 0x1d {
        let ownership = e.call(ITEM_GET_OWNERSHIP, &args![item]).u32();
        effects = if ownership != 0 { ownership + 0x3c } else { 0 };
    } else {
        let form = item_form(e, item);
        if form_type(e, form) == 0x2f {
            let form = item_form(e, item);
            effects = if form != 0 { form + 0x3c } else { 0 };
        } else {
            let form = item_form(e, item);
            let enchanting = e.call(GET_FORM_ENCHANTING, &args![form]).u32();
            if enchanting != 0 && !e.call(SPELL_ITEM_FLAG, &args![enchanting, 4u32]).bool() {
                effects = enchanting + 0x24;
            }
        }
    }
    let has_effects = effects != 0 && {
        let list = e.call(IDENTITY, &args![effects + 4]).u32();
        e.mem.u32(list) != 0
    };
    if has_effects {
        e.call(
            EFFECT_LIST_BUILD_MENU_STRING,
            &args![effects, effect_text, 0x100u32],
        );
        if e.call(STRING_LENGTH_C, &args![effect_text]).u32() > 0 {
            tile_set_text(e, cells[6], value_trait, effect_text);
            flags |= 0x40;
        } else {
            flags &= !0x40;
        }
    } else {
        let form = item_form(e, item);
        if form_type(e, form) != 0x29 {
            flags &= !0x40;
        }
    }

    let form = item_form(e, item);
    if form_type(e, form) == 0x67 {
        let form = item_form(e, item);
        let description = e
            .vcall(form + 0x74, 0x10, &args![0u32, DESCRIPTION_TAG])
            .u32();
        tile_set_text(e, cells[6], value_trait, description);
        flags |= 0x40;
    }

    // The mod slots.
    if !has_mods {
        flags &= !0x80;
    } else {
        let mod_form = item_form(e, item);
        let slots = e.call(ITEM_GET_MOD_SLOTS, &args![item]).u8();
        let mut titled = false;
        if slots & 1 != 0 {
            flags |= 0x80;
            let text = setting_text(e, SETTING_MODS_TITLE);
            tile_set_text(e, cells[7], title_trait, text);
            titled = true;
            height_a = (height_a as f64 + e.global::<f64>(MOD_ROW_HEIGHT)) as f32;
            height_b = (height_b as f64 + e.global::<f64>(MOD_ROW_HEIGHT)) as f32;
            let mod_item = e.call(ITEM_FORM_GET_MOD, &args![mod_form, 1u32]).u32();
            if mod_item != 0 {
                let name = e.call(GET_NAME, &args![mod_item + 0x30]).u32();
                copy_text(e, mod_text, 0x80, name);
            } else {
                copy_text(e, mod_text, 0x80, MOD_1_TEXT);
            }
            tile_set_text(e, cells[7], value_trait, mod_text);
        }
        if slots & 2 != 0 {
            flags |= 0x100;
            let mod_item = e.call(ITEM_FORM_GET_MOD, &args![mod_form, 2u32]).u32();
            if mod_item != 0 {
                let name = e.call(GET_NAME, &args![mod_item + 0x30]).u32();
                copy_text(e, mod_text, 0x80, name);
            } else {
                copy_text(e, mod_text, 0x10, MOD_2_TEXT);
            }
            tile_set_text(e, cells[8], value_trait, mod_text);
            if !titled {
                let text = setting_text(e, SETTING_MODS_TITLE);
                tile_set_text(e, cells[8], title_trait, text);
                titled = true;
            }
            height_b = (height_b as f64 + e.global::<f64>(MOD_ROW_HEIGHT)) as f32;
        }
        if slots & 4 != 0 {
            let mod_item = e.call(ITEM_FORM_GET_MOD, &args![mod_form, 4u32]).u32();
            if mod_item != 0 {
                let name = e.call(GET_NAME, &args![mod_item + 0x30]).u32();
                copy_text(e, mod_text, 0x80, name);
            } else {
                copy_text(e, mod_text, 0x10, MOD_3_TEXT);
            }
            tile_set_text(e, cells[9], value_trait, mod_text);
            flags |= 0x200;
            if !titled {
                let text = setting_text(e, SETTING_MODS_TITLE);
                tile_set_text(e, cells[9], title_trait, text);
            }
        }
        flags &= !0x40;
    }

    // The value text.
    if item_value <= 0.0 || item_value.is_nan() {
        copy_text(e, value_text, 0x10, DASHES);
    } else {
        let count = e.call(WORD_AT_4, &args![item]).u32() as i32;
        let product = count as f64 * item_value as f64;
        let format = if (item_value as f64) < e.global::<f64>(ONE) {
            FORMAT_ONE_DECIMAL
        } else {
            FORMAT_NO_DECIMAL
        };
        e.call(FORMAT_TEXT, &args![value_text, 0x10u32, format, product]);
    }
    tile_set_text(e, cells[3], value_trait, value_text);

    // The weight text.
    e.mem.set_f32(entry_out, 0.0);
    let player = e.global::<u32>(PLAYER);
    e.call(HANDLE_ENTRY_POINT, &args![0x37u32, player, entry_out]);
    if e.mem.f32(entry_out) > 0.0 {
        let limit = e
            .call(FLOAT_SETTING_GET, &args![WEIGHT_LIMIT_SETTING])
            .u32();
        if weight <= e.mem.f32(limit) {
            let factor = e
                .call(FLOAT_SETTING_GET, &args![WEIGHT_FACTOR_SETTING])
                .u32();
            weight = (weight as f64 * e.mem.f32(factor) as f64) as f32;
        }
    }
    if weight <= 0.0 || weight.is_nan() {
        copy_text(e, weight_text, 0x20, DASHES);
    } else {
        let count = e.call(WORD_AT_4, &args![item]).u32() as i32;
        let product = count as f64 * weight as f64;
        e.call(
            FORMAT_TEXT,
            &args![weight_text, 0x20u32, FLOAT_FORMAT, product],
        );
    }
    tile_set_text(e, cells[2], value_trait, weight_text);

    // Titles for languages other than English.
    let language = setting_text(e, LANGUAGE_SETTING);
    if e.call(COMPARE_TEXT, &args![language, ENGLISH_TEXT]).i32() != 0 {
        for (setting, cell) in LANGUAGE_TITLES {
            let text = setting_text(e, setting);
            tile_set_text(e, cells[cell], title_trait, text);
        }
    }

    // Which cells are shown.
    for (bit, cell) in VISIBILITY_BITS {
        tile_set_int(e, cells[cell], TRAIT_0FA3, (flags & bit != 0) as u32);
    }
    if cells[10] != 0 {
        tile_set_int(e, cells[10], TRAIT_0FA3, (flags & 0x400 != 0) as u32);
        let constant = e.global::<f32>(CELL_10_TRAIT_FA9);
        tile_set_float(e, cells[10], TRAIT_0FA9, constant);
    }

    // The heights of the cells below.
    let extent = tile_float(e, display, TRAIT_0FB0);
    let mut base = ((extent as f64 / e.global::<f64>(TWO)) - e.global::<f64>(TWENTY)) as f32;
    if flags & 0x10 != 0 || flags & 0x20 != 0 {
        base += base;
    }
    tile_set_float(e, cells[6], TRAIT_0FA2, base);
    tile_set_float(e, cells[7], TRAIT_0FA2, base);
    tile_set_float(
        e,
        cells[8],
        TRAIT_0FA2,
        (base as f64 + height_a as f64) as f32,
    );
    tile_set_float(
        e,
        cells[9],
        TRAIT_0FA2,
        (base as f64 + height_b as f64) as f32,
    );
}

// Translated from 00709470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::QueueMenuCreate` (Xbox PDB), `cdecl`: remembers the menu to
/// open (kind and four words). Kind 7 (the message menu) goes to its own
/// slot (`011d8998`, words `011d899c`, `011d89a0`, `011d89a4`, `011d89ac`);
/// any other kind to the first (`011d8950`, `011d8954`, `011d8958`,
/// `011d895c`, `011d8964`). When `008c7aa0` says nothing blocks, the queue
/// is handled at once ([`interface_handle_queued_menu_open`]) and then both
/// slots are cleared of their kind and first word ([`fn_00709a90`]).
pub fn interface_queue_menu_create(
    e: &mut Engine,
    kind: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) {
    if kind == 7 {
        e.mem.set_u32(MESSAGE_QUEUE_KIND, kind);
        e.mem.set_u32(MESSAGE_QUEUE_FIRST, first);
        e.mem.set_u32(MESSAGE_QUEUE_SECOND, second);
        e.mem.set_u32(MESSAGE_QUEUE_THIRD, third);
        e.mem.set_u32(MESSAGE_QUEUE_FOURTH, fourth);
    } else {
        e.mem.set_u32(QUEUE_KIND, kind);
        e.mem.set_u32(QUEUE_FIRST, first);
        e.mem.set_u32(QUEUE_SECOND, second);
        e.mem.set_u32(QUEUE_THIRD, third);
        e.mem.set_u32(QUEUE_FOURTH, fourth);
    }
    if !e.call(QUEUE_MENU_BLOCKED, &[]).bool() {
        interface_handle_queued_menu_open(e);
        fn_00709a90(e);
    }
}

// Translated from 007094f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::HandleQueuedMenuOpen` (Xbox PDB): with the queue lock
/// (`011d89c0`) held, opens the queued menus when the manager's mode word
/// (`+0x4bc`) is 0 or the kind is 6 or 7, and clears a slot once its menu
/// was created.
///
/// First slot, by kind: 1 container menu (`00704ed0(first, fourth)`), 2
/// hacking menu, 3 computers menu, 4 dialog menu (`00705070(first, second,
/// third)`, only when `007023a0` on the manager is clear), 5 race/sex menu
/// (`011d8960`), 6 start menu (`007cb7d0(1, 0)`, then `HUDMainMenu::
/// SetMenuMode(4)` when it was created), 7 message menu, 8 companion wheel
/// menu (the first word cast `0118_41cc -> 0118_46d4`), 9 a player name
/// entry menu built here: a `TileMenu` (`0x40` bytes, [`fn_00709810`]) put
/// under the menus root, named "Player Name Entry Menu", given to a new menu
/// object (`007ab690`, `0x2c` bytes) through `Menu::SetMenuTile`. A created
/// menu clears the slot's kind and first word. The second slot only holds
/// kind 7 (the message menu) and is cleared the same way.
///
/// The compiler's exception-unwinding frame (around the two `operator new`
/// calls) is not translated.
pub fn interface_handle_queued_menu_open(e: &mut Engine) {
    e.call(QUEUE_LOCK_ENTER, &args![QUEUE_LOCK, 0u32]);
    let kind = e.mem.u32(QUEUE_KIND);
    if kind != 0 {
        let current = manager(e);
        if e.mem.u32(current + 0x4bc) == 0 || kind == 6 || kind == 7 {
            let mut created = false;
            match kind {
                1 => {
                    let first = e.mem.u32(QUEUE_FIRST);
                    let fourth = e.mem.u32(QUEUE_FOURTH);
                    created = e.call(CREATE_CONTAINER_MENU, &args![first, fourth]).u32() != 0;
                }
                2 => {
                    let first = e.mem.u32(QUEUE_FIRST);
                    created = interface_create_hacking_menu(e, first);
                }
                3 => {
                    let first = e.mem.u32(QUEUE_FIRST);
                    created = interface_create_computers_menu(e, first);
                }
                4 => {
                    let current = manager(e);
                    if !e.call(DIALOG_BLOCKED, &args![current]).bool() {
                        let first = e.mem.u32(QUEUE_FIRST);
                        let second = e.mem.u32(QUEUE_SECOND);
                        let third = e.mem.u32(QUEUE_THIRD);
                        created = e
                            .call(CREATE_DIALOG_MENU, &args![first, second, third])
                            .u32()
                            != 0;
                    }
                }
                5 => {
                    let race_sex = e.mem.u32(QUEUE_RACE_SEX);
                    created = e.call(RACE_SEX_MENU_CREATE, &args![race_sex]).u32() != 0;
                }
                6 => {
                    created = e.call(START_MENU_CREATE, &args![1u32, 0u32]).bool();
                    if created {
                        e.call(HUD_SET_MENU_MODE, &args![4u32]);
                    }
                }
                7 => created = e.call(MESSAGE_MENU_CREATE, &[]).bool(),
                8 => {
                    let first = e.mem.u32(QUEUE_FIRST);
                    let wheel = e
                        .call(
                            DYNAMIC_CAST,
                            &args![
                                first,
                                0u32,
                                COMPANION_CAST_SOURCE,
                                COMPANION_CAST_TARGET,
                                0u32
                            ],
                        )
                        .u32();
                    created = e.call(COMPANION_WHEEL_MENU_CREATE, &args![wheel]).bool();
                }
                9 => {
                    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
                    let menu_tile = if block != 0 {
                        fn_00709810(e, block, 0, 0)
                    } else {
                        0
                    };
                    let block = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
                    let menu = if block != 0 {
                        e.call(NAME_ENTRY_MENU_CONSTRUCT, &args![block]).u32()
                    } else {
                        0
                    };
                    let current = manager(e);
                    let root = e.call(GET_MENUS_ROOT, &args![current]).u32();
                    e.call(TILE_SET_PARENT, &args![menu_tile, root, 0u32]);
                    fn_00707e10(e, menu_tile, NAME_ENTRY_MENU_NAME);
                    e.call(MENU_SET_MENU_TILE, &args![menu, menu_tile, 1u32]);
                    e.call(NAME_ENTRY_MENU_FINISH, &args![menu_tile, menu]);
                    created = true;
                }
                _ => {}
            }
            if created {
                e.mem.set_u32(QUEUE_KIND, 0);
                e.mem.set_u32(QUEUE_FIRST, 0);
            }
        }
    }
    let message_kind = e.mem.u32(MESSAGE_QUEUE_KIND);
    if message_kind != 0 {
        let current = manager(e);
        if e.mem.u32(current + 0x4bc) == 0 || message_kind == 6 || message_kind == 7 {
            let mut created = false;
            if message_kind == 7 {
                created = e.call(MESSAGE_MENU_CREATE, &[]).bool();
            }
            if created {
                e.mem.set_u32(MESSAGE_QUEUE_KIND, 0);
                e.mem.set_u32(MESSAGE_QUEUE_FIRST, 0);
            }
        }
    }
    e.call(QUEUE_LOCK_LEAVE, &args![QUEUE_LOCK]);
}

// Translated from 00709810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `TileMenu` (vtable `0106ed44`, RTTI `.?AVTileMenu@@`),
/// `RET 8`: the `TileRect` constructor ([`fn_00709840`]), the vtable, and
/// the word at `+0x3c` cleared. Neither parameter is read. Returns `this`.
pub fn fn_00709810(e: &mut Engine, this: u32, _unused_1: u32, _unused_2: u32) -> u32 {
    fn_00709840(e, this);
    e.mem.set_u32(this, TILE_MENU_VTABLE);
    e.mem.set_u32(this + 0x3c, 0);
    this
}

// Translated from 00709840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `TileRect` (vtable `0106ed70`, RTTI `.?AVTileRect@@`): the
/// `Tile` constructor ([`fn_00709860`]) and the vtable. Returns `this`.
pub fn fn_00709840(e: &mut Engine, this: u32) -> u32 {
    fn_00709860(e, this);
    e.mem.set_u32(this, TILE_RECT_VTABLE);
    this
}

// Translated from 00709860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `Tile` (vtable `0106ed9c`, RTTI `.?AVTile@@`): constructs
/// the members at `+4` (`0048f200`), `+0x10` (`00709e80(0x10, 0)`), `+0x20`
/// (a `BSStringT`) and `+0x2c` (`00633c90(0)` then `0066b0d0(0)`), and
/// clears the words at `+0x28` and `+0x30` and the byte at `+0x34`.
/// Returns `this`. The compiler's exception-unwinding frame is not
/// translated.
pub fn fn_00709860(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, TILE_VTABLE);
    e.call(TILE_MEMBER_AT_4, &args![this + 4]);
    e.call(TILE_MEMBER_AT_10, &args![this + 0x10, 0x10u32, 0u32]);
    e.call(STRING_CONSTRUCT, &args![this + 0x20]);
    e.call(TILE_MEMBER_AT_2C_FIRST, &args![this + 0x2c, 0u32]);
    e.call(TILE_MEMBER_AT_2C_SECOND, &args![this + 0x2c, 0u32]);
    e.mem.set_u32(this + 0x28, 0);
    e.mem.set_u32(this + 0x30, 0);
    e.mem.set_u8(this + 0x34, 0);
    this
}

// Translated from 00709920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetTypeName` (Xbox PDB): the address of the string "TILE"
/// (`0106edc4`).
pub fn tile_get_type_name(_e: &mut Engine) -> u32 {
    0x0106_edc4
}

// Translated from 00709930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::_scalar_deleting_destructor_` (Xbox PDB): `Tile::~Tile`
/// (`009ff340`), then `operator delete` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn tile_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(TILE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00709960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileRect::GetType` (Xbox PDB): `0x385`.
pub fn tile_rect_get_type(_e: &mut Engine) -> u32 {
    0x385
}

// Translated from 00709970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileRect::GetTypeName` (Xbox PDB): the address of the string "RECT"
/// (`0106edcc`).
pub fn tile_rect_get_type_name(_e: &mut Engine) -> u32 {
    0x0106_edcc
}

// Translated from 00709980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileRect::_scalar_deleting_destructor_` (Xbox PDB): the `TileRect`
/// destructor ([`fn_007099b0`]), then `operator delete` (`00401030`) when
/// bit 0 of `flags` is set. Returns `this`.
pub fn tile_rect_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_007099b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007099b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `TileRect`: sets the `TileRect` vtable, runs
/// `Tile::Release` (`009ff690`) unless [`fn_00709a20`] says the tile has the
/// flag `0x2000`, then `Tile::~Tile` (`009ff340`). The compiler's
/// exception-unwinding frame is not translated.
pub fn fn_007099b0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, TILE_RECT_VTABLE);
    if !fn_00709a20(e, this) {
        e.call(TILE_RELEASE, &args![this]);
    }
    e.call(TILE_DESTRUCTOR, &args![this]);
}

// Translated from 00709a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the tile's flag word at `+0x30` has bit `0x2000`.
pub fn fn_00709a20(e: &mut Engine, this: u32) -> bool {
    e.mem.u32(this + 0x30) & 0x2000 != 0
}

// Translated from 00709a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileMenu::GetType` (Xbox PDB): `0x389`.
pub fn tile_menu_get_type(_e: &mut Engine) -> u32 {
    0x389
}

// Translated from 00709a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileMenu::GetTypeName` (Xbox PDB): the address of the string "MENU"
/// (`0106edd4`).
pub fn tile_menu_get_type_name(_e: &mut Engine) -> u32 {
    0x0106_edd4
}

// Translated from 00709a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileMenu::_scalar_deleting_destructor_` (Xbox PDB): the `TileMenu`
/// destructor (`00a1eff0`), then `operator delete` (`00401030`) when bit 0
/// of `flags` is set. Returns `this`.
pub fn tile_menu_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(TILE_MENU_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00709a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the kind and the first word of both queued menus (`011d8950`,
/// `011d8954`, `011d8998`, `011d899c`).
pub fn fn_00709a90(e: &mut Engine) {
    e.mem.set_u32(QUEUE_KIND, 0);
    e.mem.set_u32(QUEUE_FIRST, 0);
    e.mem.set_u32(MESSAGE_QUEUE_KIND, 0);
    e.mem.set_u32(MESSAGE_QUEUE_FIRST, 0);
}

// Translated from 00709ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` constant at `01016264` (0.75 in the installed exe).
pub fn fn_00709ac0(e: &mut Engine) -> f32 {
    e.global::<f32>(RETURNED_SCALE_01016264)
}

// Translated from 00709ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `007e47a0` (`surgerymenu.cpp`).
pub fn fn_00709ae0(e: &mut Engine) {
    e.call(SURGERY_MENU_FN, &[]);
}

// Translated from 00709af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `0078e7c0` (`lockpickmenu.cpp`).
pub fn fn_00709af0(e: &mut Engine) {
    e.call(LOCKPICK_MENU_FN, &[]);
}

// Translated from 00709b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `007c19c0` (`slotmachinemenu.cpp`).
pub fn fn_00709b00(e: &mut Engine) {
    e.call(SLOT_MACHINE_MENU_FN, &[]);
}

// Translated from 00709b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `00733500` (`blackjackmenu.cpp`).
pub fn fn_00709b10(e: &mut Engine) {
    e.call(BLACKJACK_MENU_FN, &[]);
}

// Translated from 00709b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `007bbcf0` (`roulettemenu.cpp`).
pub fn fn_00709b20(e: &mut Engine) {
    e.call(ROULETTE_MENU_FN, &[]);
}

// Translated from 00709b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `00740f30` (`caravanmenu.cpp`).
pub fn fn_00709b30(e: &mut Engine) {
    e.call(CARAVAN_MENU_FN, &[]);
}

// Translated from 00709b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static wrapper: calls `0077f0d0` (`hudmainmenu.cpp`).
pub fn fn_00709b40(e: &mut Engine) {
    e.call(HUD_MAIN_MENU_FN, &[]);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00705a90, fn_00705a90() -> u32),
        entry!(0x00705ad0, fn_00705ad0(u32)),
        entry!(0x00705b10, fn_00705b10(u32, u32)),
        entry!(0x00705b30, fn_00705b30(u32)),
        entry!(0x00705b70, fn_00705b70(u8, u8, u8, i32)),
        entry!(0x00705c40, fn_00705c40(i32, i8) -> i32),
        entry!(0x00705c70, fn_00705c70() -> u8),
        entry!(0x00705cb0, fn_00705cb0() -> u8),
        entry!(0x00705cf0, fn_00705cf0(u32) -> u8),
        entry!(0x00705d10, fn_00705d10() -> u8),
        entry!(0x00705d50, fn_00705d50() -> u8),
        entry!(0x00705d90, fn_00705d90() -> i32),
        entry!(0x00705dd0, fn_00705dd0(u32) -> i32),
        entry!(0x00705e00, interface_create_loading_menu(u32, u8) -> u32),
        entry!(0x00705e30, interface_close_loading_menu()),
        entry!(0x00705e80, interface_get_loading_menu_visible() -> u8),
        entry!(0x00705ea0, interface_is_in_game_loading_menu_open() -> bool),
        entry!(0x00705ec0, interface_create_hacking_menu(u32) -> bool),
        entry!(0x00706130, interface_create_computers_menu(u32) -> bool),
        entry!(0x00706230, fn_00706230(u32, u32)),
        entry!(0x00706270, interface_create_level_up_menu()),
        entry!(0x007062a0, fn_007062a0(u32, u32, u8, u8)),
        entry!(0x007062e0, interface_set_info_for_ref(u32, u8, u8) -> u8),
        entry!(0x00706320, interface_close_all_support_menus(u32)),
        entry!(0x007065c0, fn_007065c0() -> u16),
        entry!(0x00706610, interface_save_game()),
        entry!(0x007066d0, fn_007066d0(u32)),
        entry!(0x00706810, fn_00706810(u32) -> i32),
        entry!(0x00706830, fn_00706830() -> u8),
        entry!(0x00706860, interface_load_game(u32)),
        entry!(0x007069d0, fn_007069d0(u8)),
        entry!(0x007069f0, fn_007069f0(u32, u32)),
        entry!(0x00706a20, fn_00706a20(u8)),
        entry!(0x00706a40, fn_00706a40(u32)),
        entry!(0x00706a90, interface_finish_load_game()),
        entry!(0x00706ac0, fn_00706ac0()),
        entry!(0x00706ae0, fn_00706ae0()),
        entry!(0x00706b90, fn_00706b90(u32)),
        entry!(0x00706c20, interface_tile_is_being_deleted(u32)),
        entry!(0x00706cb0, fn_00706cb0(u32)),
        entry!(0x00706cd0, interface_get_menus_root() -> u32),
        entry!(0x00706cf0, fn_00706cf0() -> u32),
        entry!(0x00706d10, interface_get_debug_text_root() -> u32),
        entry!(0x00706d30, interface_get_interface_root() -> u32),
        entry!(0x00706d50, interface_attach_default_alpha_property(u32)),
        entry!(0x00706d70, interface_get_first_chance_texture_release() -> u8),
        entry!(0x00706d90, fn_00706d90(u32) -> u8),
        entry!(0x00706db0, fn_00706db0(u8)),
        entry!(0x00706dd0, fn_00706dd0(u8)),
        entry!(0x00706df0, interface_get_screen_height() -> f32),
        entry!(0x00706e00, interface_get_screen_width() -> f32),
        entry!(0x00706e10, interface_get_real_screen_height() -> f32),
        entry!(0x00706e20, fn_00706e20() -> f32),
        entry!(0x00706e40, interface_get_real_screen_width() -> f32),
        entry!(0x00706e50, fn_00706e50() -> f32),
        entry!(0x00706e70, interface_get_screen_aspect_ratio() -> f32),
        entry!(0x00706e80, fn_00706e80() -> f32),
        entry!(0x00706e90, fn_00706e90() -> f32),
        entry!(0x00706f20, fn_00706f20() -> u32),
        entry!(0x00706f30, interface_play_menu_sound(u32)),
        entry!(0x00706f50, interface_new_timer(u32, f32)),
        entry!(0x00706f70, interface_update_all_timers()),
        entry!(0x00706f90, interface_clear_timer(u32)),
        entry!(0x00706fb0, interface_add_to_enter_stack(u32) -> u32),
        entry!(0x00706fd0, interface_pop_from_enter_stack(u32, u8) -> u32),
        entry!(0x00706ff0, interface_set_current_focus_target(u32)),
        entry!(0x00707010, interface_recursive_fade(u32, f32, f32)),
        entry!(0x00707040, fn_00707040() -> u32),
        entry!(0x00707050, fn_00707050(u32)),
        entry!(0x00707070, fn_00707070() -> u32),
        entry!(0x00707080, fn_00707080(u32)),
        entry!(0x007070a0, fn_007070a0() -> u32),
        entry!(
            0x007070c0,
            interface_find_text_replacement_string(u32, u32, u32, u8) -> bool
        ),
        entry!(0x00707330, fn_00707330(u32) -> i32),
        entry!(0x007073d0, fn_007073d0(u32, u32) -> bool),
        entry!(0x00707640, interface_release_model_file(u32)),
        entry!(0x00707660, interface_load_model_file(u32, u32) -> u32),
        entry!(0x00707820, interface_clear_temp_model()),
        entry!(0x00707860, fn_00707860() -> u32),
        entry!(0x00707870, interface_copy_or_deep_copy_node(u32) -> u32),
        entry!(0x00707930, interface_set_need_to_update(u8)),
        entry!(0x00707950, fn_00707950(u32, u8)),
        entry!(0x00707970, fn_00707970(u32) -> bool),
        entry!(0x00707990, interface_create_menu_by_class(u32) -> u32),
        entry!(0x007079b0, interface_is_in_rendered_menu() -> bool),
        entry!(0x007079f0, fn_007079f0(u32) -> bool),
        entry!(0x00707a30, interface_is_rendered_menu(u32) -> bool),
        entry!(0x00707ad0, interface_get_current_rendered_menu() -> u32),
        entry!(0x00707af0, fn_00707af0(u32) -> u32),
        entry!(0x00707b30, interface_is_current_rendered_menu_topmost() -> bool),
        entry!(0x00707b40, fn_00707b40() -> u8),
        entry!(0x00707b50, fn_00707b50() -> u8),
        entry!(0x00707b60, fn_00707b60() -> f32),
        (0x00707be0, interface_initialize_tabline_entry as AbiFn),
        entry!(0x00707e10, fn_00707e10(u32, u32)),
        entry!(0x00707e30, interface_populate_item_stats_display(u32, u32)),
        entry!(
            0x00709470,
            interface_queue_menu_create(u32, u32, u32, u32, u32)
        ),
        entry!(0x007094f0, interface_handle_queued_menu_open()),
        entry!(0x00709810, fn_00709810(u32, u32, u32) -> u32),
        entry!(0x00709840, fn_00709840(u32) -> u32),
        entry!(0x00709860, fn_00709860(u32) -> u32),
        entry!(0x00709920, tile_get_type_name() -> u32),
        entry!(0x00709930, tile_scalar_deleting_destructor(u32, u32) -> u32),
        entry!(0x00709960, tile_rect_get_type() -> u32),
        entry!(0x00709970, tile_rect_get_type_name() -> u32),
        entry!(0x00709980, tile_rect_scalar_deleting_destructor(u32, u32) -> u32),
        entry!(0x007099b0, fn_007099b0(u32)),
        entry!(0x00709a20, fn_00709a20(u32) -> bool),
        entry!(0x00709a40, tile_menu_get_type() -> u32),
        entry!(0x00709a50, tile_menu_get_type_name() -> u32),
        entry!(0x00709a60, tile_menu_scalar_deleting_destructor(u32, u32) -> u32),
        entry!(0x00709a90, fn_00709a90()),
        entry!(0x00709ac0, fn_00709ac0() -> f32),
        entry!(0x00709ae0, fn_00709ae0()),
        entry!(0x00709af0, fn_00709af0()),
        entry!(0x00709b00, fn_00709b00()),
        entry!(0x00709b10, fn_00709b10()),
        entry!(0x00709b20, fn_00709b20()),
        entry!(0x00709b30, fn_00709b30()),
        entry!(0x00709b40, fn_00709b40()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Registers a double that returns a fixed value in `eax`.
    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    /// An engine whose manager exists and is ready, with the pages holding
    /// the globals the code reads mapped, and the call log on.
    fn ui_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x011d_8000,
            0x011d_9000,
            0x011d_a000,
            0x011d_b000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        let manager = e.mem.alloc(0x600);
        e.mem.set_u32(0x011d_8a80, manager);
        e.mem.set_u8(manager, 1);
        e.register(MANAGER_READY, |e, a| ret(e.mem.u8(a[0]) as u32));
        e.call_log = Some(vec![]);
        (e, manager)
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

    /// The addresses called, in order, including the top-level call.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a != GET_MANAGER && *a != MANAGER_READY)
            .collect()
    }

    /// Checks a wrapper that calls `callee` with `expected(manager)` when
    /// the manager is ready (returning `ready_result`) and does nothing,
    /// returning `default`, when the manager is not ready or missing.
    fn check_forward(
        wrapper: u32,
        wrapper_args: &[u32],
        callee: u32,
        expected: impl Fn(u32) -> Vec<u32>,
        ready_result: u32,
        default: u32,
    ) {
        let (mut e, manager) = ui_engine();
        stub(&mut e, callee, ready_result);
        assert_eq!(e.call(wrapper, wrapper_args).u32(), ready_result);
        assert_eq!(logged(&e, callee), vec![expected(manager)]);

        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(wrapper, wrapper_args).u32(), default);
        assert!(logged(&e, callee).is_empty());

        e.mem.set_u32(0x011d_8a80, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(wrapper, wrapper_args).u32(), default);
        assert!(logged(&e, callee).is_empty());
    }

    #[test]
    fn fn_00705a90_forwards_to_the_manager() {
        check_forward(0x0070_5a90, &[], 0x0070_58c0, |m| vec![m], 7, 0);
    }

    #[test]
    fn fn_00705ad0_stores_the_word_when_ready() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_5ad0, &args![0x1234u32]);
        assert_eq!(e.mem.u32(manager + 0xc4), 0x1234);
        e.mem.set_u8(manager, 0);
        e.call(0x0070_5ad0, &args![0x5678u32]);
        assert_eq!(e.mem.u32(manager + 0xc4), 0x1234);
    }

    #[test]
    fn fn_00705b10_stores_the_word() {
        let (mut e, manager) = ui_engine();
        e.call(0x0070_5b10, &args![manager, 9u32]);
        assert_eq!(e.mem.u32(manager + 0xc4), 9);
    }

    #[test]
    fn fn_00705b30_forwards_the_value() {
        check_forward(
            0x0070_5b30,
            &args![0x42u32],
            0x0071_6010,
            |m| vec![m, 0x42],
            0,
            0,
        );
    }

    #[test]
    fn fn_00705b70_clamps_the_bytes_and_the_menu_id() {
        let (mut e, manager) = ui_engine();
        // 004a8f20(4, x) doubles x.
        e.register(HELPER_4A8F20, |_, a| ret(a[1] * 2));
        for setter in [0x0070_b790, 0x0070_b810, 0x0070_b870, 0x0070_b8d0] {
            stub(&mut e, setter, 0);
        }
        e.call(0x0070_5b70, &args![5u32, 0xffu32, 7u32, 0x999u32]);
        assert_eq!(logged(&e, 0x0070_b790), vec![vec![manager, 10]]);
        assert_eq!(logged(&e, 0x0070_b810), vec![vec![manager, 0]]);
        assert_eq!(logged(&e, 0x0070_b870), vec![vec![manager, 14]]);
        assert_eq!(logged(&e, 0x0070_b8d0), vec![vec![manager, 0x3eb]]);

        e.call_log = Some(vec![]);
        e.call(0x0070_5b70, &args![0u32, 0u32, 0u32, 0x3ffu32]);
        assert_eq!(logged(&e, 0x0070_b8d0), vec![vec![manager, 0x3ff]]);
        e.call_log = Some(vec![]);
        e.call(0x0070_5b70, &args![0u32, 0u32, 0u32, 0x3eau32]);
        assert_eq!(logged(&e, 0x0070_b8d0), vec![vec![manager, 0x3ea]]);

        e.mem.set_u8(manager, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_5b70, &args![1u32, 1u32, 1u32, 0x3eau32]);
        assert!(logged(&e, 0x0070_b8d0).is_empty());
    }

    #[test]
    fn fn_00705c40_is_the_larger_with_a_signed_byte() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_5c40, &args![0u32, 5u32]).i32(), 5);
        assert_eq!(e.call(0x0070_5c40, &args![0u32, 0xffu32]).i32(), 0);
        assert_eq!(e.call(0x0070_5c40, &args![-3i32, 0xfdu32]).i32(), -3);
        assert_eq!(e.call(0x0070_5c40, &args![-3i32, 0xfeu32]).i32(), -2);
    }

    #[test]
    fn fn_00705c70_forwards_a_byte() {
        check_forward(0x0070_5c70, &[], 0x0047_ee30, |m| vec![m], 9, 0xff);
    }

    #[test]
    fn fn_00705cb0_reads_the_manager_byte() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager + 0x13, 6);
        assert_eq!(e.call(0x0070_5cb0, &[]).u32(), 6);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_5cb0, &[]).u32(), 0xff);
    }

    #[test]
    fn fn_00705cf0_reads_the_byte_at_13() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager + 0x13, 0x81);
        assert_eq!(e.call(0x0070_5cf0, &args![manager]).u32(), 0x81);
    }

    #[test]
    fn fn_00705d10_forwards_a_byte() {
        check_forward(0x0070_5d10, &[], 0x0084_3150, |m| vec![m], 3, 0xff);
    }

    #[test]
    fn fn_00705d50_forwards_a_byte() {
        check_forward(0x0070_5d50, &[], 0x0098_2bc0, |m| vec![m], 4, 0xff);
    }

    #[test]
    fn fn_00705d90_is_the_trait_value_or_minus_one() {
        let (mut e, manager) = ui_engine();
        let root = e.mem.alloc(8);
        e.mem.set_u32(manager + 0x9c, root);
        e.register_double(TILE_GET_VALUE, move |_, a| {
            assert_eq!(a, &[root, 0x1771]);
            Ret {
                st0: -2.5,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0070_5d90, &[]).i32(), -2);
        e.mem.set_u8(manager, 0);
        assert_eq!(e.call(0x0070_5d90, &[]).i32(), -1);
    }

    #[test]
    fn fn_00705dd0_truncates_the_root_trait() {
        let (mut e, manager) = ui_engine();
        let root = e.mem.alloc(8);
        e.mem.set_u32(manager + 0x9c, root);
        e.register(TILE_GET_VALUE, |_, _| Ret {
            st0: 7.9,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0070_5dd0, &args![manager]).i32(), 7);
    }

    #[test]
    fn create_loading_menu_sets_the_flag_then_creates() {
        let (mut e, _) = ui_engine();
        stub(&mut e, SET_LOADING_FLAG, 0);
        stub(&mut e, LOADING_MENU_CREATE, 0x77);
        assert_eq!(e.call(0x0070_5e00, &args![0x10u32, 0x1ffu32]).u32(), 0x77);
        assert_eq!(
            order(&e),
            vec![0x0070_5e00, SET_LOADING_FLAG, LOADING_MENU_CREATE]
        );
        assert_eq!(logged(&e, SET_LOADING_FLAG), vec![vec![1]]);
        assert_eq!(logged(&e, LOADING_MENU_CREATE), vec![vec![0x10, 1, 0xff]]);
    }

    #[test]
    fn close_loading_menu_deletes_the_menu_then_clears_the_flag() {
        let (mut e, _) = ui_engine();
        stub(&mut e, SET_LOADING_FLAG, 0);
        stub(&mut e, 0x0200_0000, 0);
        e.put_vtable(0x0200_1000, &[0x0200_0000]);
        let menu = e.mem.alloc(8);
        e.mem.set_u32(menu, 0x0200_1000);
        e.mem.set_u32(LOADING_MENU, menu);
        e.call(0x0070_5e30, &[]);
        assert_eq!(order(&e), vec![0x0070_5e30, 0x0200_0000, SET_LOADING_FLAG]);
        assert_eq!(logged(&e, 0x0200_0000), vec![vec![menu, 1]]);
        assert_eq!(logged(&e, SET_LOADING_FLAG), vec![vec![0]]);

        e.mem.set_u32(LOADING_MENU, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_5e30, &[]);
        assert_eq!(order(&e), vec![0x0070_5e30, SET_LOADING_FLAG]);
    }

    #[test]
    fn get_loading_menu_visible_asks_for_menu_3ef() {
        let (mut e, _) = ui_engine();
        stub(&mut e, MENU_ID_FLAG, 1);
        assert_eq!(e.call(0x0070_5e80, &[]).u32(), 1);
        assert_eq!(logged(&e, MENU_ID_FLAG), vec![vec![0x3ef]]);
    }

    #[test]
    fn is_in_game_loading_menu_open_tests_the_object() {
        let (mut e, _) = ui_engine();
        assert!(!e.call(0x0070_5ea0, &[]).bool());
        e.mem.set_u32(LOADING_MENU, 0x1234);
        assert!(e.call(0x0070_5ea0, &[]).bool());
    }

    /// An engine for the rendered-menu creators: the creators' callees are
    /// doubles; the rendered-menus flag is `flag`.
    fn menu_engine(flag: bool) -> (Engine, u32) {
        let (mut e, manager) = ui_engine();
        stub(&mut e, HACKING_MENU_CREATE, 0x5000);
        stub(&mut e, COMPUTERS_MENU_CREATE, 0x5000);
        let flag_byte = e.mem.alloc(8);
        e.mem.set_u8(flag_byte, flag as u8);
        stub(&mut e, RENDERED_MENUS_FLAG_GETTER, flag_byte);
        stub(&mut e, TILE_UPDATE_ALL, 0);
        let block = e.mem.alloc(RENDERED_MENU_SIZE);
        stub(&mut e, OPERATOR_NEW, block);
        e.register(RENDERED_MENU_CTOR, |_, a| ret(a[0]));
        stub(&mut e, RENDERED_MENU_SET_MENU, 0);
        stub(&mut e, MENU_NODE, 0x6000);
        stub(&mut e, RENDERED_MENU_SET_NODE, 0);
        stub(&mut e, ENTER_RENDERED_MENU, 0);
        (e, manager)
    }

    #[test]
    fn create_hacking_menu_enters_a_rendered_menu_when_used() {
        let (mut e, manager) = menu_engine(true);
        assert!(e.call(0x0070_5ec0, &args![0x33u32]).bool());
        assert_eq!(
            order(&e),
            vec![
                0x0070_5ec0,
                HACKING_MENU_CREATE,
                RENDERED_MENUS_FLAG_GETTER,
                TILE_UPDATE_ALL,
                OPERATOR_NEW,
                RENDERED_MENU_CTOR,
                RENDERED_MENU_SET_MENU,
                MENU_NODE,
                RENDERED_MENU_SET_NODE,
                ENTER_RENDERED_MENU,
            ]
        );
        let block = logged(&e, RENDERED_MENU_CTOR)[0][0];
        assert_eq!(logged(&e, HACKING_MENU_CREATE), vec![vec![0x33]]);
        assert_eq!(logged(&e, OPERATOR_NEW), vec![vec![0xf0]]);
        assert_eq!(
            logged(&e, RENDERED_MENU_SET_MENU),
            vec![vec![block, 0x5000]]
        );
        assert_eq!(
            logged(&e, RENDERED_MENU_SET_NODE),
            vec![vec![block, 0x6000]]
        );
        assert_eq!(logged(&e, ENTER_RENDERED_MENU), vec![vec![manager, block]]);
    }

    #[test]
    fn create_hacking_menu_without_rendered_menus_only_creates() {
        let (mut e, _) = menu_engine(false);
        assert!(e.call(0x0070_5ec0, &args![0x33u32]).bool());
        assert_eq!(
            order(&e),
            vec![0x0070_5ec0, HACKING_MENU_CREATE, RENDERED_MENUS_FLAG_GETTER]
        );
    }

    #[test]
    fn create_hacking_menu_needs_a_ready_manager() {
        let (mut e, manager) = menu_engine(true);
        e.mem.set_u8(manager, 0);
        assert!(!e.call(0x0070_5ec0, &args![0x33u32]).bool());
        assert_eq!(order(&e), vec![0x0070_5ec0]);
    }

    #[test]
    fn create_hacking_menu_with_a_failed_allocation_passes_null() {
        let (mut e, _) = menu_engine(true);
        stub(&mut e, OPERATOR_NEW, 0);
        e.call(0x0070_5ec0, &args![0x33u32]);
        assert!(logged(&e, RENDERED_MENU_CTOR).is_empty());
        assert_eq!(logged(&e, RENDERED_MENU_SET_MENU), vec![vec![0, 0x5000]]);
    }

    #[test]
    fn create_computers_menu_updates_the_tiles_before_the_check() {
        let (mut e, manager) = menu_engine(true);
        assert!(e.call(0x0070_6130, &args![0x44u32]).bool());
        assert_eq!(
            order(&e),
            vec![
                0x0070_6130,
                COMPUTERS_MENU_CREATE,
                TILE_UPDATE_ALL,
                RENDERED_MENUS_FLAG_GETTER,
                OPERATOR_NEW,
                RENDERED_MENU_CTOR,
                RENDERED_MENU_SET_MENU,
                MENU_NODE,
                RENDERED_MENU_SET_NODE,
                ENTER_RENDERED_MENU,
            ]
        );
        assert_eq!(logged(&e, COMPUTERS_MENU_CREATE), vec![vec![0x44]]);
        assert_eq!(logged(&e, TILE_UPDATE_ALL), vec![vec![0]]);
        assert_eq!(logged(&e, ENTER_RENDERED_MENU)[0][0], manager);

        let (mut e, manager) = menu_engine(false);
        assert!(e.call(0x0070_6130, &args![0x44u32]).bool());
        assert_eq!(
            order(&e),
            vec![
                0x0070_6130,
                COMPUTERS_MENU_CREATE,
                TILE_UPDATE_ALL,
                RENDERED_MENUS_FLAG_GETTER
            ]
        );
        e.mem.set_u8(manager, 0);
        assert!(!e.call(0x0070_6130, &args![0x44u32]).bool());
    }

    #[test]
    fn fn_00706230_forwards_both_words() {
        check_forward(
            0x0070_6230,
            &args![3u32, 4u32],
            0x007e_41f0,
            |_| vec![3, 4],
            0,
            0,
        );
    }

    #[test]
    fn create_level_up_menu_runs_when_ready() {
        check_forward(0x0070_6270, &[], 0x0078_4c80, |_| vec![], 0, 0);
    }

    #[test]
    fn fn_007062a0_forwards_the_four_words() {
        check_forward(
            0x0070_62a0,
            &args![1u32, 2u32, 0x1ffu32, 0x2aau32],
            0x0075_3420,
            |_| vec![1, 2, 0xff, 0xaa],
            0,
            0,
        );
    }

    #[test]
    fn set_info_for_ref_forwards_and_returns_a_byte() {
        check_forward(
            0x0070_62e0,
            &args![0x99u32, 1u32, 0u32],
            0x0077_5a00,
            |_| vec![0x99, 1, 0],
            1,
            0,
        );
    }

    /// What the `CloseAllSupportMenus` scenario saw.
    struct Closing {
        e: Engine,
        tiles: Vec<u32>,
        removed: Vec<u32>,
        flag_when_removed: Vec<u8>,
    }

    const MENU_ID_SLOT_FUNCTION: u32 = 0x0200_0100;
    const DELETE_FUNCTION: u32 = 0x0200_0200;

    /// Runs `CloseAllSupportMenus` over tiles whose menus have the given
    /// ids (0 = a tile with no menu). `allow_3f5` is the answer of
    /// `004a4040`, `cast_ok` that of the dynamic casts.
    fn run_close(menu_ids: &[u32], allow_3f5: bool, cast_ok: bool) -> Closing {
        let (mut e, _) = ui_engine();
        let root = e.mem.alloc(16);
        stub(&mut e, GET_MENUS_ROOT, root);
        let mut menu_slots = [0u32; 14];
        menu_slots[13] = MENU_ID_SLOT_FUNCTION;
        e.put_vtable(0x0200_2000, &menu_slots);
        e.register(MENU_ID_SLOT_FUNCTION, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.put_vtable(0x0200_3000, &[DELETE_FUNCTION]);
        stub(&mut e, DELETE_FUNCTION, 0);
        let mut tiles = vec![];
        let mut menus = std::collections::HashMap::new();
        for &id in menu_ids {
            let tile = e.mem.alloc(8);
            tiles.push(tile);
            if id != 0 {
                let menu = e.mem.alloc(16);
                e.mem.set_u32(menu, 0x0200_2000);
                e.mem.set_u32(menu + 8, id);
                menus.insert(tile, menu);
            }
        }
        e.register_double(TILE_GET_MENU, move |_, a| {
            ret(menus.get(&a[0]).copied().unwrap_or(0))
        });
        // The list at root + 4: nodes [next, 0, tile].
        let mut next = 0;
        for &tile in tiles.iter().rev() {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, tile);
            next = node;
        }
        e.mem.set_u32(root + 4, next);
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            let following = e.mem.u32(node);
            e.mem.set_u32(a[1], following);
            ret(node + 8)
        });
        let removed = Rc::new(RefCell::new(vec![]));
        let flags = Rc::new(RefCell::new(vec![]));
        let (removed_log, flag_log) = (removed.clone(), flags.clone());
        let victim = e.mem.alloc(8);
        e.mem.set_u32(victim, 0x0200_3000);
        e.register_double(LIST_REMOVE, move |e, a| {
            removed_log.borrow_mut().push(e.mem.u32(a[1]));
            flag_log.borrow_mut().push(e.mem.u8(CLOSING_SUPPORT_MENUS));
            ret(victim)
        });
        stub(&mut e, CLOSE_MENU_3F5_ALLOWED, allow_3f5 as u32);
        stub(&mut e, DYNAMIC_CAST, if cast_ok { 0x7777 } else { 0 });
        stub(&mut e, SPECIAL_BOOK_MENU_CLOSE, 0);
        stub(&mut e, MENU_432_CLOSE, 0);
        e.call(0x0070_6320, &args![0u32]);
        let removed = removed.borrow().clone();
        let flag_when_removed = flags.borrow().clone();
        Closing {
            e,
            tiles,
            removed,
            flag_when_removed,
        }
    }

    #[test]
    fn close_all_support_menus_keeps_the_listed_menus() {
        let run = run_close(
            &[0x3eb, 0x3ea, 0x3ec, 0x3ef, 0x3ff, 0x40b, 0x425],
            true,
            true,
        );
        assert!(run.removed.is_empty());
        assert!(logged(&run.e, DELETE_FUNCTION).is_empty());
        assert_eq!(run.e.mem.u8(CLOSING_SUPPORT_MENUS), 0);
    }

    #[test]
    fn close_all_support_menus_removes_other_menus_and_skips_menuless_tiles() {
        let run = run_close(&[0x100, 0, 0x3eb, 0x3f6], true, true);
        assert_eq!(run.removed, vec![run.tiles[0], run.tiles[3]]);
        assert_eq!(run.flag_when_removed, vec![1, 1]);
        assert_eq!(run.e.mem.u8(CLOSING_SUPPORT_MENUS), 0);
        assert_eq!(logged(&run.e, DELETE_FUNCTION).len(), 2);
        assert_eq!(logged(&run.e, DELETE_FUNCTION)[0][1], 1);
    }

    #[test]
    fn close_all_support_menus_removes_menu_3f5_only_when_allowed() {
        let run = run_close(&[0x3f5], false, true);
        assert!(run.removed.is_empty());
        let run = run_close(&[0x3f5], true, true);
        assert_eq!(run.removed, run.tiles);
        assert_eq!(logged(&run.e, DELETE_FUNCTION).len(), 1);
    }

    #[test]
    fn close_all_support_menus_closes_menu_424_when_the_cast_works() {
        let run = run_close(&[0x424], true, true);
        assert_eq!(logged(&run.e, SPECIAL_BOOK_MENU_CLOSE).len(), 1);
        assert_eq!(run.removed, run.tiles);
        let cast = &logged(&run.e, DYNAMIC_CAST)[0];
        assert_eq!(
            &cast[1..],
            &[0, CAST_SOURCE_TYPE, SPECIAL_BOOK_MENU_TYPE, 0]
        );

        let run = run_close(&[0x424], true, false);
        assert!(logged(&run.e, SPECIAL_BOOK_MENU_CLOSE).is_empty());
        assert_eq!(run.removed, run.tiles);
    }

    #[test]
    fn close_all_support_menus_closes_menu_432_when_the_cast_works() {
        let run = run_close(&[0x432], true, true);
        assert_eq!(logged(&run.e, MENU_432_CLOSE).len(), 1);
        assert_eq!(run.removed, run.tiles);
        assert_eq!(logged(&run.e, DYNAMIC_CAST)[0][3], MENU_432_TYPE);

        let run = run_close(&[0x432], true, false);
        assert!(logged(&run.e, MENU_432_CLOSE).is_empty());
        assert_eq!(run.removed, run.tiles);
    }

    #[test]
    fn fn_007065c0_is_nine_from_0x5d() {
        let (mut e, _) = ui_engine();
        let owner = e.mem.alloc(8);
        e.mem.set_u32(SAVE_LOAD_GAME, owner);
        stub(&mut e, SAVE_OBJECT_BYTE, 0x5c);
        assert_eq!(e.call(0x0070_65c0, &[]).u32(), 8);
        assert_eq!(logged(&e, SAVE_OBJECT_BYTE), vec![vec![owner]]);
        stub(&mut e, SAVE_OBJECT_BYTE, 0x5d);
        assert_eq!(e.call(0x0070_65c0, &[]).u32(), 9);
    }

    /// Registers a double on `addr` that records the bytes `(pointer, size)`
    /// found at argument words 1 and 2 (the save buffer writers).
    fn record_writes(e: &mut Engine, addr: u32) -> Rc<RefCell<Vec<Vec<u8>>>> {
        let log = Rc::new(RefCell::new(vec![]));
        let sink = log.clone();
        e.register_double(addr, move |e, a| {
            sink.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            ret(0)
        });
        log
    }

    /// `memset` double: fills the destination with the value byte.
    fn stub_memset(e: &mut Engine) {
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
    }

    #[test]
    fn save_game_writes_the_clamped_bytes_and_the_word() {
        let (mut e, manager) = ui_engine();
        let owner = e.mem.alloc(8);
        e.mem.set_u32(SAVE_LOAD_GAME, owner);
        e.mem.set_u8(manager + 0x13, 5);
        let root = e.mem.alloc(8);
        e.mem.set_u32(manager + 0x9c, root);
        stub(&mut e, 0x0047_ee30, 0);
        stub(&mut e, 0x0084_3150, 0x80);
        stub(&mut e, 0x0098_2bc0, 9);
        e.register(TILE_GET_VALUE, |_, _| Ret {
            st0: 300.0,
            ..Ret::default()
        });
        let writes = record_writes(&mut e, SAVE_GAME_WRITE);
        e.call(0x0070_6610, &[]);
        assert_eq!(
            *writes.borrow(),
            vec![
                vec![1],
                vec![5],
                vec![1],
                vec![9],
                300u32.to_le_bytes().to_vec()
            ]
        );
        assert_eq!(logged(&e, SAVE_GAME_WRITE)[0][0], owner);
    }

    #[test]
    fn save_game_without_a_manager_writes_the_defaults() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager, 0);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(SAVE_LOAD_GAME, owner);
        let writes = record_writes(&mut e, SAVE_GAME_WRITE);
        e.call(0x0070_6610, &[]);
        // 0xff is -1 as a signed byte, raised to 1; the word is -1.
        assert_eq!(
            *writes.borrow(),
            vec![
                vec![1],
                vec![1],
                vec![1],
                vec![1],
                u32::MAX.to_le_bytes().to_vec()
            ]
        );
    }

    #[test]
    fn fn_007066d0_writes_the_menu_state() {
        let (mut e, manager) = ui_engine();
        let root = e.mem.alloc(8);
        stub(&mut e, GET_MENUS_ROOT, root);
        e.register(TILE_GET_VALUE, |_, _| Ret {
            st0: 300.0,
            ..Ret::default()
        });
        stub(&mut e, STATS_PAGE_GET, 0x11);
        stub(&mut e, INVENTORY_BYTE_GET, 0x22);
        let floats = Rc::new(RefCell::new(vec![1.5f64, 2.5]));
        e.register_double(MAP_FLOAT_GET_A, {
            let floats = floats.clone();
            move |_, _| Ret {
                st0: floats.borrow()[0],
                ..Ret::default()
            }
        });
        e.register_double(MAP_FLOAT_GET_B, move |_, _| Ret {
            st0: floats.borrow()[1],
            ..Ret::default()
        });
        stub_memset(&mut e);
        stub(&mut e, HELPER_4A8F20, 3);
        e.register(SUBOBJECT_GET_STRING, |e, a| {
            e.mem.set_cstr(a[1], b"hello");
            ret(0)
        });
        let stream = e.mem.alloc(8);
        let writes = record_writes(&mut e, SAVE_BUFFER_WRITE);
        e.call(0x0070_66d0, &args![stream]);
        let mut text = b"hello".to_vec();
        text.resize(8, 0);
        assert_eq!(
            *writes.borrow(),
            vec![
                300u32.to_le_bytes().to_vec(),
                vec![0x11],
                vec![0x22],
                vec![4],
                1.5f32.to_le_bytes().to_vec(),
                2.5f32.to_le_bytes().to_vec(),
                8u32.to_le_bytes().to_vec(),
                text,
            ]
        );
        assert_eq!(logged(&e, SAVE_BUFFER_WRITE)[0][0], stream);
        assert_eq!(logged(&e, SAVE_BUFFER_WRITE)[7][3], 0);
        assert_eq!(logged(&e, SUBOBJECT_GET_STRING)[0][0], manager + 0x4d4);
        assert_eq!(logged(&e, MEMSET)[0][1..], [0, 100]);
    }

    #[test]
    fn fn_00706810_is_five_more_than_the_helper() {
        let (mut e, _) = ui_engine();
        stub(&mut e, HELPER_4A8F20, 10);
        assert_eq!(e.call(0x0070_6810, &args![0x1234u32]).i32(), 15);
        assert_eq!(logged(&e, HELPER_4A8F20), vec![vec![1, 1]]);
    }

    #[test]
    fn fn_00706830_is_the_state_word_minus_0x20_or_4() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_6830, &[]).u32(), 4);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(STATE_OBJECT, object);
        e.mem.set_u32(object + 0x80, 0x25);
        assert_eq!(e.call(0x0070_6830, &[]).u32(), 5);
        e.mem.set_u32(object + 0x80, 0x10);
        assert_eq!(e.call(0x0070_6830, &[]).u32(), 0xf0);
    }

    /// Doubles for `LoadGame`: the reads deliver `data` in order. Returns
    /// the stream (whose version byte is `version`) and the log of the
    /// float setters.
    fn load_engine(version: u32, data: Vec<Vec<u8>>) -> (Engine, u32, u32) {
        let (mut e, manager) = ui_engine();
        let root = e.mem.alloc(8);
        stub(&mut e, GET_MENUS_ROOT, root);
        let queue = Rc::new(RefCell::new(std::collections::VecDeque::from(data)));
        e.register_double(LOAD_BUFFER_READ, move |e, a| {
            let next = queue.borrow_mut().pop_front().expect("unexpected read");
            assert_eq!(a[2] as usize, next.len());
            e.mem.write(a[1], &next);
            ret(0)
        });
        stub(&mut e, TILE_SET_VALUE, 0);
        stub(&mut e, STATS_PAGE_SET, 0);
        stub(&mut e, MAP_FLOAT_SET_A, 0);
        stub(&mut e, MAP_FLOAT_SET_B, 0);
        stub_memset(&mut e);
        stub(&mut e, SUBOBJECT_SET_STRING, 0);
        stub(&mut e, LOAD_BUFFER_LOAD_STRING, 0);
        stub(&mut e, 0x0200_0000, version);
        e.put_vtable(0x0200_1000, &[0x0200_0000]);
        let stream = e.mem.alloc(8);
        e.mem.set_u32(stream, 0x0200_1000);
        (e, manager, stream)
    }

    fn load_data(tail: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut data = vec![
            300u32.to_le_bytes().to_vec(),
            vec![0x33],
            vec![5],
            vec![7],
            1.5f32.to_le_bytes().to_vec(),
            2.5f32.to_le_bytes().to_vec(),
        ];
        data.extend(tail);
        data
    }

    #[test]
    fn load_game_applies_the_state_and_reads_the_text_from_version_12() {
        let data = load_data(vec![5u32.to_le_bytes().to_vec(), b"world".to_vec()]);
        let (mut e, manager, stream) = load_engine(12, data);
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(SUBOBJECT_SET_STRING, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            ret(0)
        });
        e.call(0x0070_6860, &args![stream]);
        let root = logged(&e, GET_MENUS_ROOT);
        assert_eq!(root.len(), 1);
        let set = &logged(&e, TILE_SET_VALUE)[0];
        assert_eq!(set[1..], [0x1771, 300]);
        assert_eq!(logged(&e, STATS_PAGE_SET), vec![vec![0x33]]);
        assert_eq!(logged(&e, MAP_FLOAT_SET_A), vec![vec![1.5f32.to_bits()]]);
        assert_eq!(logged(&e, MAP_FLOAT_SET_B), vec![vec![2.5f32.to_bits()]]);
        assert_eq!(*seen.borrow(), vec![(manager + 0x4d4, b"world".to_vec())]);
        assert!(logged(&e, LOAD_BUFFER_LOAD_STRING).is_empty());
    }

    #[test]
    fn load_game_keeps_its_locals_at_the_games_frame_offsets() {
        // The saved word is the first read, at +0x184 of the frame
        // (EBP-0x14 under SUB ESP,0x198); the length word is at +0x10c
        // (EBP-0x8c) and the 100-byte buffer at +0x110 (EBP-0x88).
        let data = load_data(vec![5u32.to_le_bytes().to_vec(), b"world".to_vec()]);
        let (mut e, _, stream) = load_engine(12, data);
        e.call(0x0070_6860, &args![stream]);
        let reads = logged(&e, LOAD_BUFFER_READ);
        let base = reads[0][1] - 0x184;
        assert_eq!(reads[6][1..], [base + 0x10c, 4]);
        assert_eq!(reads[7][1..], [base + 0x110, 5]);
        assert_eq!(logged(&e, MEMSET), vec![vec![base + 0x110, 0, 100]]);
        assert_eq!(logged(&e, SUBOBJECT_SET_STRING)[0][1], base + 0x110);

        // Before version 12 the LoadString buffer is the lowest local
        // (EBP-0x198, 007069b3).
        let (mut e, _, stream) = load_engine(11, load_data(vec![]));
        e.call(0x0070_6860, &args![stream]);
        let base = logged(&e, LOAD_BUFFER_READ)[0][1] - 0x184;
        assert_eq!(
            logged(&e, LOAD_BUFFER_LOAD_STRING),
            vec![vec![stream, base]]
        );
    }

    #[test]
    fn load_game_before_version_12_reads_a_string_and_drops_it() {
        let (mut e, _, stream) = load_engine(11, load_data(vec![]));
        e.call(0x0070_6860, &args![stream]);
        assert_eq!(logged(&e, LOAD_BUFFER_LOAD_STRING).len(), 1);
        assert_eq!(logged(&e, LOAD_BUFFER_LOAD_STRING)[0][0], stream);
        assert!(logged(&e, SUBOBJECT_SET_STRING).is_empty());
        assert!(logged(&e, MEMSET).is_empty());
    }

    #[test]
    fn load_game_hands_the_bytes_to_the_state_object_and_trait_object() {
        let (mut e, _, stream) = load_engine(11, load_data(vec![]));
        let state = e.mem.alloc(0x100);
        e.mem.set_u32(STATE_OBJECT, state);
        e.put_vtable(0x0200_4000, &[0, 0, 0, 0x0200_0100]);
        e.mem.set_u32(state, 0x0200_4000);
        stub(&mut e, 0x0200_0100, 0);
        let trait_object = e.mem.alloc(0x100);
        e.mem.set_u32(TRAIT_OBJECT, trait_object);
        e.mem.set_u32(trait_object + 0x5c, 0x4444);
        e.mem.set_u32(TRAIT_OBJECT_TRAIT_ID, 0x55);
        stub(&mut e, HELPER_4A8F20, 0x66);
        e.call(0x0070_6860, &args![stream]);
        // The third saved byte (7) goes to the virtual slot, the second (5)
        // to the trait object.
        assert_eq!(logged(&e, 0x0200_0100), vec![vec![state, 0x27, 0]]);
        assert_eq!(logged(&e, HELPER_4A8F20), vec![vec![5, 4]]);
        assert_eq!(logged(&e, TILE_SET_VALUE)[1], vec![0x4444, 0x55, 0x66]);
    }

    #[test]
    fn fn_007069d0_calls_the_state_object() {
        let (mut e, _) = ui_engine();
        e.call(0x0070_69d0, &args![0x41u32]);
        let state = e.mem.alloc(0x100);
        e.mem.set_u32(STATE_OBJECT, state);
        e.put_vtable(0x0200_4000, &[0, 0, 0, 0x0200_0100]);
        e.mem.set_u32(state, 0x0200_4000);
        stub(&mut e, 0x0200_0100, 0);
        e.call_log = Some(vec![]);
        e.call(0x0070_69d0, &args![0x41u32]);
        assert_eq!(logged(&e, 0x0200_0100), vec![vec![state, 0x61, 0]]);
    }

    #[test]
    fn fn_007069f0_passes_the_value_plus_0x20_and_zero_to_slot_c() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0200_4000, &[0, 0, 0, 0x0200_0100]);
        e.mem.set_u32(object, 0x0200_4000);
        stub(&mut e, 0x0200_0100, 0);
        e.call(0x0070_69f0, &args![object, 5u32]);
        assert_eq!(logged(&e, 0x0200_0100), vec![vec![object, 0x25, 0]]);
    }

    #[test]
    fn fn_00706a20_zero_extends_the_byte() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(TRAIT_OBJECT, object);
        e.register(HELPER_4A8F20, |_, a| ret(a[0]));
        stub(&mut e, TILE_SET_VALUE, 0);
        e.call(0x0070_6a20, &args![0x1ffu32]);
        assert_eq!(logged(&e, HELPER_4A8F20), vec![vec![0xff, 4]]);
    }

    #[test]
    fn fn_00706a40_sets_the_trait_and_stores_the_value() {
        let (mut e, _) = ui_engine();
        stub(&mut e, TILE_SET_VALUE, 0);
        e.register(HELPER_4A8F20, |_, a| ret(a[0] + 100));
        e.call(0x0070_6a40, &args![5u32]);
        assert!(logged(&e, TILE_SET_VALUE).is_empty());

        let object = e.mem.alloc(0x100);
        e.mem.set_u32(TRAIT_OBJECT, object);
        e.mem.set_u32(object + 0x5c, 0x4444);
        e.mem.set_u32(TRAIT_OBJECT_TRAIT_ID, 0x55);
        e.call(0x0070_6a40, &args![5u32]);
        assert_eq!(logged(&e, TILE_SET_VALUE), vec![vec![0x4444, 0x55, 105]]);
        assert_eq!(e.mem.u32(object + 0x84), 105);
    }

    #[test]
    fn finish_load_game_runs_the_hooks_in_order() {
        let (mut e, _) = ui_engine();
        let hooks = [
            0x007d_d710,
            0x007d_f4e0,
            0x007d_f5d0,
            0x0079_f640,
            0x007a_1490,
            0x007e_5890,
        ];
        for hook in hooks {
            stub(&mut e, hook, 0);
        }
        let state = e.mem.alloc(0x100);
        e.mem.set_u32(state + 0x90, 7);
        e.mem.set_u32(STATE_OBJECT, state);
        e.call(0x0070_6a90, &[]);
        assert_eq!(order(&e), [&[0x0070_6a90][..], &hooks].concat());
        assert_eq!(e.mem.u32(state + 0x90), 0);
    }

    #[test]
    fn fn_00706ac0_clears_the_word_when_there_is_an_object() {
        let (mut e, _) = ui_engine();
        e.call(0x0070_6ac0, &[]);
        let state = e.mem.alloc(0x100);
        e.mem.set_u32(state + 0x90, 7);
        e.mem.set_u32(STATE_OBJECT, state);
        e.call(0x0070_6ac0, &[]);
        assert_eq!(e.mem.u32(state + 0x90), 0);
    }

    /// Doubles for every callee of `00706ae0`; `value_hook` is the answer of
    /// `007050d0`.
    fn finish_engine(value_hook: bool) -> (Engine, u32) {
        let (mut e, manager) = ui_engine();
        for hook in [
            0x0070_53b0,
            0x0077_3050,
            0x0070_5190,
            0x0095_3060,
            0x0083_e510,
            0x0070_5780,
            0x007f_6920,
            0x007a_a480,
            0x007d_da60,
            0x0079_8bf0,
            0x0070_6fd0,
            ENTER_RENDERED_MENU,
        ] {
            stub(&mut e, hook, 0);
        }
        stub(&mut e, 0x0070_50d0, value_hook as u32);
        stub(&mut e, 0x0070_5190, 0x99);
        let root = e.mem.alloc(16);
        stub(&mut e, GET_MENUS_ROOT, root);
        stub(&mut e, FIRST_WORD, 0);
        (e, manager)
    }

    #[test]
    fn fn_00706ae0_resets_the_interface_with_the_value_hook() {
        let (mut e, manager) = finish_engine(true);
        e.mem.set_u32(FINISH_OBJECT, 0x1234);
        e.mem.set_u32(manager + 0x4bc, 5);
        e.mem.set_u32(manager + 0x4d4, 0xffff);
        e.call(0x0070_6ae0, &[]);
        assert_eq!(
            order(&e),
            vec![
                0x0070_6ae0,
                0x0070_53b0,
                0x0077_3050,
                0x0070_50d0,
                0x0070_5190,
                0x0095_3060,
                0x0083_e510,
                0x0070_5780,
                0x007f_6920,
                GET_MENUS_ROOT,
                FIRST_WORD,
                0x007a_a480,
                0x007d_da60,
                0x0079_8bf0,
                0x0070_6fd0,
                ENTER_RENDERED_MENU,
            ]
        );
        assert_eq!(logged(&e, 0x0070_5190), vec![Vec::<u32>::new()]);
        assert_eq!(logged(&e, 0x0095_3060), vec![vec![0x1234, 0x99, 0, 1]]);
        assert_eq!(logged(&e, 0x0070_6fd0), vec![vec![1, 1]]);
        assert_eq!(logged(&e, ENTER_RENDERED_MENU), vec![vec![manager, 0]]);
        assert_eq!(e.mem.u8(FINISH_FLAG), 1);
        assert_eq!(e.mem.u8(CLOSING_SUPPORT_MENUS), 0);
        assert_eq!(e.mem.u32(manager + 0x4bc), 0);
        assert_eq!(e.mem.u32(manager + 0x4d4), 0xfc);
        assert_eq!(e.mem.u32(manager + 0x4d4 + 0xa4), 0x29);
    }

    #[test]
    fn fn_00706ae0_skips_the_value_hook_when_it_says_no() {
        let (mut e, _) = finish_engine(false);
        e.call(0x0070_6ae0, &[]);
        assert!(logged(&e, 0x0070_5190).is_empty());
        assert!(logged(&e, 0x0095_3060).is_empty());
        assert!(logged(&e, 0x0083_e510).is_empty());
        assert_eq!(logged(&e, 0x0070_5780).len(), 1);
    }

    #[test]
    fn fn_00706b90_clears_bits_and_resets_the_counters() {
        let (mut e, _) = ui_engine();
        let table = e.mem.alloc(0xb0);
        for index in 0..0x2au32 {
            e.mem.set_u32(table + 4 * index, 0xffff_ffff);
        }
        e.mem.set_u32(table + 0xa8, 7);
        e.call(0x0070_6b90, &args![table]);
        for index in 0..0x29u32 {
            assert_eq!(e.mem.u32(table + 4 * index), 0xfc);
        }
        assert_eq!(e.mem.u32(table + 0xa4), 0x29);
        assert_eq!(e.mem.u32(table + 0xa8), 0);
    }

    #[test]
    fn tile_is_being_deleted_clears_the_matching_slots() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0097_ae90, 0x5555);
        stub(&mut e, 0x0071_3de0, 0);
        let tile = 0x4444;
        e.mem.set_u32(manager + 0xcc, tile);
        e.mem.set_u32(manager + 0xd0, 1);
        e.mem.set_u32(manager + 0xd4, tile);
        e.mem.set_u32(manager + 0xd8, 2);
        e.mem.set_u32(manager + 0xbc, 3);
        e.call(0x0070_6c20, &args![tile]);
        for offset in [0xcc, 0xd0, 0xd4, 0xd8] {
            assert_eq!(e.mem.u32(manager + offset), 0);
        }
        assert_eq!(e.mem.u32(manager + 0xbc), 3);
        assert_eq!(logged(&e, 0x0071_3de0), vec![vec![manager, tile]]);
    }

    #[test]
    fn tile_is_being_deleted_leaves_other_slots_and_clears_the_current_one() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0097_ae90, 0x4444);
        stub(&mut e, 0x0071_3de0, 0);
        e.mem.set_u32(manager + 0xcc, 0x1111);
        e.mem.set_u32(manager + 0xd0, 1);
        e.mem.set_u32(manager + 0xd4, 0x2222);
        e.mem.set_u32(manager + 0xd8, 2);
        e.mem.set_u32(manager + 0xbc, 3);
        e.call(0x0070_6c20, &args![0x4444u32]);
        assert_eq!(e.mem.u32(manager + 0xcc), 0x1111);
        assert_eq!(e.mem.u32(manager + 0xd0), 1);
        assert_eq!(e.mem.u32(manager + 0xd4), 0x2222);
        assert_eq!(e.mem.u32(manager + 0xd8), 2);
        assert_eq!(e.mem.u32(manager + 0xbc), 0);
    }

    #[test]
    fn fn_00706cb0_clears_the_word_at_bc() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(manager + 0xbc, 9);
        e.call(0x0070_6cb0, &args![manager]);
        assert_eq!(e.mem.u32(manager + 0xbc), 0);
    }

    // ---- 00706cd0 .. 00707870 ----

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine as `ui_engine` with the pages the text and model code reads
    /// mapped too.
    fn text_engine() -> (Engine, u32) {
        let (mut e, manager) = ui_engine();
        for page in [
            0x0106_e000,
            0x0101_d000,
            0x0119_b000,
            0x011c_3000,
            0x011c_8000,
            0x011d_5000,
        ] {
            e.map(page, 0x1000);
        }
        (e, manager)
    }

    /// A zero-terminated string in fresh memory.
    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        for (i, byte) in text.bytes().enumerate() {
            e.mem.set_u8(at + i as u32, byte);
        }
        e.mem.set_u8(at + text.len() as u32, 0);
        at
    }

    /// The string at `at`.
    fn text_at(e: &Engine, at: u32) -> String {
        let mut out = String::new();
        let mut i = 0;
        while e.mem.u8(at + i) != 0 {
            out.push(e.mem.u8(at + i) as char);
            i += 1;
        }
        out
    }

    /// A getter that passes the manager to `callee` and returns its result.
    fn check_manager_getter(wrapper: u32, callee: u32) {
        let (mut e, manager) = ui_engine();
        stub(&mut e, callee, 0x55);
        assert_eq!(e.call(wrapper, &[]).u32(), 0x55);
        assert_eq!(logged(&e, callee), vec![vec![manager]]);
    }

    #[test]
    fn get_menus_root_asks_the_manager() {
        check_manager_getter(0x0070_6cd0, 0x0058_6150);
    }

    #[test]
    fn fn_00706cf0_asks_the_manager() {
        check_manager_getter(0x0070_6cf0, 0x004f_d400);
    }

    #[test]
    fn get_debug_text_root_asks_the_manager() {
        check_manager_getter(0x0070_6d10, 0x005e_3fc0);
    }

    #[test]
    fn get_interface_root_asks_the_manager() {
        check_manager_getter(0x0070_6d30, 0x004f_b070);
    }

    #[test]
    fn attach_default_alpha_property_attaches_the_managers_property() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0066_29f0, 0x77);
        stub(&mut e, 0x0043_9410, 0);
        e.call(0x0070_6d50, &args![0x1234u32]);
        assert_eq!(logged(&e, 0x0066_29f0), vec![vec![manager]]);
        assert_eq!(logged(&e, 0x0043_9410), vec![vec![0x1234, 0x77]]);
    }

    #[test]
    fn get_first_chance_texture_release_reads_the_managers_byte() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager + 0xec, 1);
        assert_eq!(e.call(0x0070_6d70, &[]).u8(), 1);
        e.mem.set_u8(manager + 0xec, 0);
        assert_eq!(e.call(0x0070_6d70, &[]).u8(), 0);
    }

    #[test]
    fn fn_00706d90_reads_the_byte_at_ec() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u8(object + 0xec, 9);
        assert_eq!(e.call(0x0070_6d90, &args![object]).u8(), 9);
    }

    #[test]
    fn fn_00706db0_passes_the_byte_to_the_setter() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0070_3960, 0);
        e.call(0x0070_6db0, &args![0xabu32]);
        assert_eq!(logged(&e, 0x0070_3960), vec![vec![manager, 0xab]]);
    }

    #[test]
    fn fn_00706dd0_stores_only_a_byte_without_the_ready_check() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager, 0);
        e.mem.set_u32(manager + 0xdc, 0x1122_3344);
        e.call(0x0070_6dd0, &args![0x5u32]);
        assert_eq!(e.mem.u32(manager + 0xdc), 0x1122_3305);
    }

    #[test]
    fn screen_height_and_width_return_the_managers_floats() {
        let (mut e, _) = ui_engine();
        e.register(0x0071_5da0, |_, _| float_ret(1080.0));
        e.register(0x0071_5d40, |_, _| float_ret(1920.0));
        assert_eq!(e.call(0x0070_6df0, &[]).f32(), 1080.0);
        assert_eq!(e.call(0x0070_6e00, &[]).f32(), 1920.0);
    }

    #[test]
    fn real_screen_height_is_the_renderers_integer_as_a_float() {
        let (mut e, _) = ui_engine();
        stub(&mut e, 0x004d_c200, 1050);
        assert_eq!(e.call(0x0070_6e20, &[]).f32(), 1050.0);
        assert_eq!(e.call(0x0070_6e10, &[]).f32(), 1050.0);
    }

    #[test]
    fn real_screen_width_is_the_renderers_integer_as_a_float() {
        let (mut e, _) = ui_engine();
        stub(&mut e, 0x004d_c1f0, 1680);
        assert_eq!(e.call(0x0070_6e50, &[]).f32(), 1680.0);
        assert_eq!(e.call(0x0070_6e40, &[]).f32(), 1680.0);
    }

    #[test]
    fn screen_aspect_ratio_reads_the_managers_float() {
        let (mut e, manager) = ui_engine();
        e.mem.set_f32(manager + 0x4d0, 1.75);
        assert_eq!(e.call(0x0070_6e70, &[]).f32(), 1.75);
    }

    #[test]
    fn fn_00706e80_reads_the_constant() {
        let (mut e, _) = text_engine();
        e.set_global(ASPECT_RATIO_CONSTANT, 1.5f32);
        assert_eq!(e.call(0x0070_6e80, &[]).f32(), 1.5);
    }

    #[test]
    fn fn_00706e90_scales_the_constant_by_the_two_counts() {
        let (mut e, _) = text_engine();
        e.set_global(ASPECT_RATIO_CONSTANT, 2.0f32);
        // Two objects with their own tables: the result of slot 0xc8 is
        // the object itself, slot 0x8c gives 400 and slot 0x90 gives 1000.
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0200_2000);
        e.put_vtable(0x0200_2000, &[0u32; 0x40]);
        e.mem.set_u32(0x0200_2000 + 0xc8, 0x0200_0001);
        e.mem.set_u32(0x0200_2000 + 0x8c, 0x0200_0002);
        e.mem.set_u32(0x0200_2000 + 0x90, 0x0200_0003);
        stub(&mut e, 0x0043_c4b0, object);
        stub(&mut e, 0x0200_0001, object);
        stub(&mut e, 0x0200_0002, 400);
        stub(&mut e, 0x0200_0003, 1000);
        // 2.0 * (1000 / 400).
        assert_eq!(e.call(0x0070_6e90, &[]).f32(), 5.0);
        assert_eq!(logged(&e, 0x0200_0002), vec![vec![object, 0]]);
        assert_eq!(logged(&e, 0x0200_0003), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00706f20_returns_0x3ec() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_6f20, &[]).u32(), 0x3ec);
    }

    #[test]
    fn play_menu_sound_forwards_the_sound() {
        let (mut e, _) = ui_engine();
        stub(&mut e, 0x0071_7280, 0);
        e.call(0x0070_6f30, &args![0x42u32]);
        assert_eq!(logged(&e, 0x0071_7280), vec![vec![0x42]]);
    }

    #[test]
    fn new_timer_forwards_the_float() {
        let (mut e, _) = ui_engine();
        stub(&mut e, 0x0071_64c0, 0);
        e.call(0x0070_6f50, &args![3u32, 2.5f32]);
        assert_eq!(logged(&e, 0x0071_64c0), vec![vec![3, 2.5f32.to_bits()]]);
    }

    #[test]
    fn update_all_timers_runs_on_the_manager() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0071_6320, 0);
        e.call(0x0070_6f70, &[]);
        assert_eq!(logged(&e, 0x0071_6320), vec![vec![manager]]);
    }

    #[test]
    fn clear_timer_forwards_the_id_without_the_manager() {
        let (mut e, _) = ui_engine();
        stub(&mut e, 0x0071_65d0, 0);
        e.call(0x0070_6f90, &args![8u32]);
        assert_eq!(logged(&e, 0x0071_65d0), vec![vec![8]]);
    }

    #[test]
    fn add_to_enter_stack_returns_the_managers_result() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0071_4d90, 5);
        assert_eq!(e.call(0x0070_6fb0, &args![3u32]).u32(), 5);
        assert_eq!(logged(&e, 0x0071_4d90), vec![vec![manager, 3]]);
    }

    #[test]
    fn pop_from_enter_stack_passes_the_id_and_the_byte() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0071_4fd0, 1);
        assert_eq!(e.call(0x0070_6fd0, &args![0x3eau32, 1u32]).u32(), 1);
        assert_eq!(logged(&e, 0x0071_4fd0), vec![vec![manager, 0x3ea, 1]]);
    }

    #[test]
    fn set_current_focus_target_adds_the_constant_arguments() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0071_5860, 0);
        e.call(0x0070_6ff0, &args![0x99u32]);
        assert_eq!(logged(&e, 0x0071_5860), vec![vec![manager, 0x99, 0xfc3, 1]]);
    }

    #[test]
    fn recursive_fade_passes_the_tile_and_both_floats() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, 0x0071_2450, 0);
        e.call(0x0070_7010, &args![0x99u32, 0.5f32, 1.0f32]);
        assert_eq!(
            logged(&e, 0x0071_2450),
            vec![vec![manager, 0x99, 0.5f32.to_bits(), 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn manager_slot_words_are_read_and_written() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u32(manager + 0xcc, 11);
        e.mem.set_u32(manager + 0xd0, 22);
        assert_eq!(e.call(0x0070_7040, &[]).u32(), 11);
        assert_eq!(e.call(0x0070_7070, &[]).u32(), 22);
        e.call(0x0070_7050, &args![33u32]);
        e.call(0x0070_7080, &args![44u32]);
        assert_eq!(e.mem.u32(manager + 0xcc), 33);
        assert_eq!(e.mem.u32(manager + 0xd0), 44);
    }

    #[test]
    fn fn_007070a0_asks_the_manager() {
        check_manager_getter(0x0070_70a0, 0x0045_cd60);
    }

    /// Doubles for the lookups of `FindTextReplacementString` that every
    /// test needs: no known index, no form with that editor ID, names that
    /// never equal a label.
    fn find_text_engine() -> Engine {
        let (mut e, _) = text_engine();
        // `00707330`: the settings object, no setting, entries of the table
        // that match nothing.
        stub(&mut e, 0x0040_4a70, 0);
        stub(&mut e, 0x004f_8a30, 0xdead);
        stub(&mut e, 0x0040_3df0, 0);
        stub(&mut e, 0x0046_9880, 1);
        stub(&mut e, 0x0048_3a00, 0);
        stub(&mut e, 0x0040_4dc0, 1);
        e.set_global(TEXT_OWNER, 0x7000u32);
        e
    }

    #[test]
    fn find_text_replacement_string_rejects_empty_input() {
        let mut e = find_text_engine();
        let name = put_text(&mut e, "PCName");
        let empty = put_text(&mut e, "");
        let dest = e.mem.alloc(32);
        for (n, d, s) in [
            (0, dest, 32),
            (empty, dest, 32),
            (name, 0, 32),
            (name, dest, 0),
        ] {
            let words = args![n, d, s, 0u32];
            assert!(!e.call(0x0070_70c0, &words).bool());
        }
        // Only the top-level calls: nothing was looked up.
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|(a, _)| *a == 0x0070_70c0));
    }

    #[test]
    fn find_text_replacement_string_uses_a_known_index() {
        let mut e = find_text_engine();
        // The first table entry equals the setting.
        e.set_global(TEXT_TABLE, 0x5000u32);
        stub(&mut e, 0x004f_8a30, 0x5000);
        stub(&mut e, 0x0070_39b0, 0);
        let name = put_text(&mut e, "Jump");
        let dest = e.mem.alloc(32);
        assert!(e.call(0x0070_70c0, &args![name, dest, 32u32, 1u32]).bool());
        assert_eq!(logged(&e, 0x0070_39b0), vec![vec![0, dest, 1]]);
        assert!(logged(&e, 0x0048_3a00).is_empty());
    }

    #[test]
    fn find_text_replacement_string_formats_a_global_value() {
        let mut e = find_text_engine();
        stub(&mut e, 0x0048_3a00, 0x6000);
        stub(&mut e, 0x0040_1170, 6);
        stub(&mut e, 0x0040_6d00, 0);
        e.register(0x0052_6ac0, |_, _| float_ret(3.75));
        // A float global ('f'), then an integer one.
        stub(&mut e, 0x0052_9ea0, 0x66);
        let name = put_text(&mut e, "Gravity");
        let dest = e.mem.alloc(32);
        assert!(e.call(0x0070_70c0, &args![name, dest, 32u32, 0u32]).bool());
        let mut expected = vec![dest, 32, FLOAT_FORMAT];
        expected.extend_from_slice(&args![3.75f64]);
        assert_eq!(logged(&e, 0x0040_6d00), vec![expected]);

        stub(&mut e, 0x0052_9ea0, 0x73);
        stub(&mut e, 0x00ec_62c0, 3);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0070_70c0, &args![name, dest, 32u32, 0u32]).bool());
        assert_eq!(
            logged(&e, 0x0040_6d00),
            vec![vec![dest, 32, INTEGER_FORMAT, 3]]
        );
    }

    #[test]
    fn find_text_replacement_string_skips_a_form_of_another_type() {
        let mut e = find_text_engine();
        stub(&mut e, 0x0048_3a00, 0x6000);
        stub(&mut e, 0x0040_1170, 5);
        // The game-setting lookup (`007073d0`) runs for real on its helpers.
        stub(&mut e, 0x0040_6d30, 0);
        stub(&mut e, 0x0044_a670, 5);
        let name = put_text(&mut e, "Thing");
        let dest = e.mem.alloc(32);
        // Falls through to the game-setting lookup, which finds a setting
        // (`004f8a30`) whose type letter is unknown: found, nothing written.
        assert!(e.call(0x0070_70c0, &args![name, dest, 32u32, 0u32]).bool());
        assert!(logged(&e, 0x0040_6d00).is_empty());
        assert_eq!(logged(&e, 0x004f_8a30).len(), 2);
    }

    #[test]
    fn find_text_replacement_string_copies_the_player_texts() {
        let mut e = find_text_engine();
        stub(&mut e, 0x0040_6d30, 0);
        stub(&mut e, 0x0055_d520, 0x111);
        stub(&mut e, 0x0087_f6c0, 0x222);
        stub(&mut e, 0x0087_f4c0, 2);
        e.set_global(0x0119_b354u32 + 8, 0xa000u32);
        e.set_global(0x0119_b364u32 + 8, 0xb000u32);
        e.set_global(0x0119_b35cu32 + 8, 0xc000u32);
        e.register(0x0040_3df0, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        // `00404dc0` is 0 only for the label the test is checking.
        let dest = e.mem.alloc(32);
        let name = put_text(&mut e, "X");
        for (label, source) in [
            (PC_NAME_TEXT, 0x111),
            (PC_RACE_TEXT, 0x222),
            (PC_SEX_TEXT, 0xa001),
            (PC_SEX_PRONOUN_TEXT, 0xb001),
            (PC_SEX_POSSESSIVE_TEXT, 0xc001),
        ] {
            e.register_double(0x0040_4dc0, move |_, a| Ret {
                eax: (a[1] != label) as u32,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            assert!(e.call(0x0070_70c0, &args![name, dest, 32u32, 0u32]).bool());
            assert_eq!(logged(&e, 0x0040_6d30), vec![vec![dest, 32, source]]);
        }
    }

    #[test]
    fn find_text_replacement_string_asks_the_owner_for_the_player_name() {
        let mut e = find_text_engine();
        stub(&mut e, 0x0040_6d30, 0);
        stub(&mut e, 0x0055_d520, 0x111);
        e.register_double(0x0040_4dc0, |_, a| Ret {
            eax: (a[1] != PC_NAME_TEXT) as u32,
            ..Ret::default()
        });
        let name = put_text(&mut e, "PCName");
        let dest = e.mem.alloc(32);
        assert!(e.call(0x0070_70c0, &args![name, dest, 32u32, 0u32]).bool());
        assert_eq!(logged(&e, 0x0055_d520), vec![vec![0x7000]]);
    }

    #[test]
    fn find_text_replacement_string_falls_back_to_the_game_setting_lookup() {
        let mut e = find_text_engine();
        let name = put_text(&mut e, "sTest");
        let dest = e.mem.alloc(32);
        for i in 0..0x1c {
            e.set_global(TEXT_TABLE + 4 * i, 0x5000 + i);
        }
        stub(&mut e, 0x004f_8a30, 0);
        stub(&mut e, 0x0040_6d30, 0);
        stub(&mut e, 0x0044_a670, 5);
        stub(&mut e, 0x0040_6d50, 0);
        stub(&mut e, 0x0044_f560, 0);
        stub(&mut e, 0x005e_02b0, 0);
        assert!(!e.call(0x0070_70c0, &args![name, dest, 32u32, 0u32]).bool());
    }

    #[test]
    fn match_text_replace_constant_finds_an_equal_setting_or_a_matching_text() {
        let (mut e, _) = text_engine();
        stub(&mut e, 0x0040_4a70, 0x300);
        e.register(0x004f_8a30, |_, a| Ret {
            eax: if a[1] == 0x77 { 0x5004 } else { 0 },
            ..Ret::default()
        });
        for i in 0..0x1c {
            e.set_global(TEXT_TABLE + 4 * i, 0x5000 + i);
        }
        // Found by the setting: entry 4 is `0x5004`.
        assert_eq!(e.call(0x0070_7330, &args![0x77u32]).i32(), 4);
        // Not found: the texts are compared; entry 6's text matches.
        e.register(0x0046_9880, |_, a| Ret {
            eax: (a[0] != 0x5006) as u32,
            ..Ret::default()
        });
        e.register(0x0040_3df0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x0070_7330, &args![0x78u32]).i32(), 6);
        // Nothing matches.
        stub(&mut e, 0x0046_9880, 1);
        assert_eq!(e.call(0x0070_7330, &args![0x78u32]).i32(), 0x1c);
    }

    /// Doubles for `007073d0`'s helpers: the copy and length functions work
    /// on real memory so the buffer is the game's.
    fn setting_engine() -> Engine {
        let (mut e, _) = text_engine();
        e.register(0x0040_6d30, |e, a| {
            let mut i = 0;
            loop {
                let byte = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, byte);
                if byte == 0 {
                    break;
                }
                i += 1;
            }
            Ret::default()
        });
        e.register(0x0044_a670, |e, a| {
            let mut i = 0;
            while e.mem.u8(a[0] + i) != 0 {
                i += 1;
            }
            Ret {
                eax: i,
                ..Ret::default()
            }
        });
        e.register(0x0040_46f0, |e, a| {
            let mut i = 0;
            loop {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
                if byte == 0 {
                    break;
                }
                i += 1;
            }
            Ret::default()
        });
        stub(&mut e, 0x0040_4a70, 0);
        stub(&mut e, 0x0040_6d50, 0);
        stub(&mut e, 0x0044_f560, 0);
        e
    }

    #[test]
    fn test_constant_for_game_settings_rejects_empty_names() {
        let mut e = setting_engine();
        let empty = put_text(&mut e, "");
        let dest = e.mem.alloc(32);
        assert!(!e.call(0x0070_73d0, &args![0u32, dest]).bool());
        assert!(!e.call(0x0070_73d0, &args![empty, dest]).bool());
    }

    #[test]
    fn test_constant_for_game_settings_writes_each_type() {
        let mut e = setting_engine();
        let setting = e.mem.alloc(16);
        let value_slot = e.mem.alloc(8);
        stub(&mut e, 0x004f_8a30, setting);
        let text = put_text(&mut e, "hello");
        e.mem.set_u32(value_slot, 42);
        stub(&mut e, 0x0040_3df0, text);
        stub(&mut e, 0x0043_d4d0, value_slot);
        stub(&mut e, 0x0040_3e20, value_slot);
        stub(&mut e, 0x0040_8d60, value_slot);
        e.register(0x00ec_623a, |e, a| {
            // Formats the first value as decimal text.
            let digits = format!("{}", a[2]);
            for (i, byte) in digits.bytes().enumerate() {
                e.mem.set_u8(a[0] + i as u32, byte);
            }
            e.mem.set_u8(a[0] + digits.len() as u32, 0);
            Ret::default()
        });
        let dest = e.mem.alloc(0x100);

        // `s`: the setting's text, after skipping `&-` and cutting `;`.
        let name = put_text(&mut e, "&-sName;");
        assert!(e.call(0x0070_73d0, &args![name, dest]).bool());
        assert_eq!(text_at(&e, dest), "hello");
        // `i`: formatted integer.
        let name = put_text(&mut e, "iCount");
        assert!(e.call(0x0070_73d0, &args![name, dest]).bool());
        assert_eq!(text_at(&e, dest), "42");
        // `f`: the float goes to the formatter as a double (and, as in the
        // exe, with the integer format).
        e.mem.set_f32(value_slot, 1.0);
        let name = put_text(&mut e, "fScale");
        assert!(e.call(0x0070_73d0, &args![name, dest]).bool());
        let formats = logged(&e, 0x00ec_623a);
        assert_eq!(formats.len(), 2);
        assert_eq!(formats[1][1..], [INTEGER_FORMAT, 0, 0x3ff0_0000]);
        // `b`: true when the byte is 1, false otherwise.
        e.mem.set_u32(value_slot, 1);
        let name = put_text(&mut e, "bFlag");
        let copy_log = |e: &mut Engine| {
            e.call_log = Some(vec![]);
            assert!(e.call(0x0070_73d0, &args![name, dest]).bool());
            logged(e, 0x0040_46f0)
        };
        assert_eq!(copy_log(&mut e), vec![vec![dest, TRUE_TEXT]]);
        e.mem.set_u32(value_slot, 0);
        assert_eq!(copy_log(&mut e), vec![vec![dest, FALSE_TEXT]]);
        // Any other letter: found, nothing written.
        let name = put_text(&mut e, "xOther");
        assert!(e.call(0x0070_73d0, &args![name, dest]).bool());
    }

    #[test]
    fn test_constant_for_game_settings_retries_with_the_language_suffix() {
        let mut e = setting_engine();
        let setting = e.mem.alloc(16);
        let text = put_text(&mut e, "translated");
        stub(&mut e, 0x0040_3df0, text);
        stub(&mut e, 0x004f_8a30, 0);
        stub(&mut e, 0x005e_02b0, setting);
        let dest = e.mem.alloc(0x100);
        let name = put_text(&mut e, "sGreeting");
        assert!(e.call(0x0070_73d0, &args![name, dest]).bool());
        assert_eq!(text_at(&e, dest), "translated");
        assert_eq!(logged(&e, 0x0040_6d50).len(), 1);
        assert_eq!(logged(&e, 0x0040_6d50)[0][2], LANGUAGE_SUFFIX);

        // Missing in both lookups: false, nothing written.
        stub(&mut e, 0x005e_02b0, 0);
        e.mem.set_u8(dest, b'z');
        assert!(!e.call(0x0070_73d0, &args![name, dest]).bool());
        assert_eq!(e.mem.u8(dest), b'z');
    }

    #[test]
    fn release_model_file_needs_the_loader() {
        let (mut e, _) = text_engine();
        stub(&mut e, 0x0045_a5e0, 0);
        e.call(0x0070_7640, &args![0x44u32]);
        assert!(logged(&e, 0x0045_a5e0).is_empty());
        e.set_global(MODEL_LOADER, 0x9000u32);
        e.call(0x0070_7640, &args![0x44u32]);
        assert_eq!(logged(&e, 0x0045_a5e0), vec![vec![0x9000, 0x44]]);
    }

    #[test]
    fn load_model_file_uses_the_loader_when_it_can() {
        let (mut e, _) = text_engine();
        e.set_global(MODEL_LOADER, 0x9000u32);
        stub(&mut e, 0x0044_7080, 0xaaa);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 7);
        assert_eq!(e.call(0x0070_7660, &args![0x44u32, flag]).u32(), 0xaaa);
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(
            logged(&e, 0x0044_7080),
            vec![vec![0x9000, 0x44, 0, 1, 0, 0, 0]]
        );
    }

    #[test]
    fn load_model_file_builds_a_temporary_model_from_a_stream() {
        let (mut e, _) = text_engine();
        let flag = e.mem.alloc(4);
        stub(&mut e, 0x0045_6a20, 1);
        stub(&mut e, 0x0043_cfd0, 0);
        stub(&mut e, 0x00c3_a8a0, 1);
        stub(&mut e, 0x0043_d090, 0);
        stub(&mut e, 0x0044_31f0, 0);
        stub(&mut e, OPERATOR_NEW, 0xb00);
        stub(&mut e, 0x0043_aaf0, 0xb00);
        stub(&mut e, 0x0043_b230, 0xc0de);
        // The old temporary model is destroyed first.
        e.set_global(TEMP_MODEL, 0x1111u32);
        e.mem.set_u8(flag, 5);
        assert_eq!(e.call(0x0070_7660, &args![0x44u32, flag]).u32(), 0xc0de);
        assert_eq!(e.mem.u8(flag), 0);
        assert_eq!(e.global::<u32>(TEMP_MODEL), 0xb00);
        assert_eq!(logged(&e, 0x0044_31f0), vec![vec![0x1111, 1]]);
        assert_eq!(logged(&e, 0x0045_6a20), vec![vec![0x44, 0, 0, u32::MAX]]);
        let build = &logged(&e, 0x0043_aaf0)[0];
        assert_eq!(build[0], 0xb00);
        assert_eq!(build[1], 0x44);
        assert_eq!((build[3], build[4]), (1, 0));
        assert_eq!(logged(&e, 0x0043_d090).len(), 1);
    }

    #[test]
    fn load_model_file_stops_without_the_file_or_when_the_stream_fails() {
        let (mut e, _) = text_engine();
        let flag = e.mem.alloc(4);
        // The loader exists but the byte forces the slow path; no file.
        e.set_global(MODEL_LOADER, 0x9000u32);
        e.mem.set_u8(LOADER_DISABLED, 1);
        stub(&mut e, 0x0045_6a20, 0);
        assert_eq!(e.call(0x0070_7660, &args![0x44u32, flag]).u32(), 0);
        assert_eq!(e.mem.u8(flag), 0);

        // The file exists but the stream cannot load it.
        stub(&mut e, 0x0045_6a20, 1);
        stub(&mut e, 0x0043_cfd0, 0);
        stub(&mut e, 0x00c3_a8a0, 0);
        stub(&mut e, 0x0043_d090, 0);
        e.set_global(TEMP_MODEL, 0x1111u32);
        assert_eq!(e.call(0x0070_7660, &args![0x44u32, flag]).u32(), 0);
        assert_eq!(e.global::<u32>(TEMP_MODEL), 0x1111);
        assert_eq!(logged(&e, 0x0043_d090).len(), 1);
    }

    #[test]
    fn clear_temp_model_destroys_the_model_and_clears_the_pointer() {
        let (mut e, _) = text_engine();
        stub(&mut e, 0x0044_31f0, 0);
        e.call(0x0070_7820, &[]);
        assert!(logged(&e, 0x0044_31f0).is_empty());
        e.set_global(TEMP_MODEL, 0x1111u32);
        e.call(0x0070_7820, &[]);
        assert_eq!(logged(&e, 0x0044_31f0), vec![vec![0x1111, 1]]);
        assert_eq!(e.global::<u32>(TEMP_MODEL), 0);
    }

    #[test]
    fn fn_00707860_returns_the_meshes_string_address() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_7860, &[]).u32(), 0x0101_dccc);
    }

    #[test]
    fn copy_or_deep_copy_node_picks_the_copy_by_node_kind() {
        let (mut e, _) = text_engine();
        stub(&mut e, 0x004a_d050, 0);
        stub(&mut e, 0x004a_d270, 0);
        stub(&mut e, 0x0045_7ba0, 0xdee9);
        stub(&mut e, 0x00a5_d2c0, 0xc10e);
        e.set_global(DEEP_COPY_OWNER, 0x9100u32);
        stub(&mut e, 0x004b_5bf0, 1);
        assert_eq!(e.call(0x0070_7870, &args![0x55u32]).u32(), 0xdee9);
        let deep = logged(&e, 0x0045_7ba0);
        assert_eq!(deep[0][0], 0x9100);
        assert_eq!(deep[0][1], 0x55);
        let process = deep[0][2];
        assert_eq!(
            logged(&e, 0x004a_d050),
            vec![vec![process, 1.0f32.to_bits()]]
        );
        assert_eq!(logged(&e, 0x004a_d270), vec![vec![process]]);

        stub(&mut e, 0x004b_5bf0, 0);
        assert_eq!(e.call(0x0070_7870, &args![0x55u32]).u32(), 0xc10e);
        assert_eq!(logged(&e, 0x00a5_d2c0)[0][0], 0x55);
    }

    // ---- 00707930 .. 00709b40 ----

    use std::collections::HashMap;

    fn ret_float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    fn put_at(e: &mut Engine, at: u32, text: &str) {
        e.mem.set_cstr(at, text.as_bytes());
    }

    #[test]
    fn set_need_to_update_stores_the_byte_without_checking_readiness() {
        let (mut e, manager) = ui_engine();
        e.mem.set_u8(manager, 0);
        e.call(0x0070_7930, &args![1u32]);
        assert_eq!(e.mem.u8(manager + 0xb0), 1);
        e.call(0x0070_7930, &args![0u32]);
        assert_eq!(e.mem.u8(manager + 0xb0), 0);
    }

    #[test]
    fn fn_00707950_stores_the_byte_at_0xb0() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(0x100);
        e.call(0x0070_7950, &args![object, 0x7fu32]);
        assert_eq!(e.mem.u8(object + 0xb0), 0x7f);
    }

    #[test]
    fn fn_00707970_plays_the_sounds_with_zero() {
        let (mut e, _) = ui_engine();
        stub(&mut e, ANIM_GROUP_PLAY_SOUNDS, 0);
        e.call(0x0070_7970, &args![0x4411u32]);
        assert_eq!(logged(&e, ANIM_GROUP_PLAY_SOUNDS), vec![vec![0x4411, 0]]);
    }

    #[test]
    fn create_menu_by_class_goes_through_the_menu_manager() {
        let (mut e, _) = ui_engine();
        stub(&mut e, MENU_MANAGER_INSTANCE, 0x7000);
        stub(&mut e, MENU_MANAGER_CREATE_MENU_BY_CLASS, 0);
        e.call(0x0070_7990, &args![0x3f1u32]);
        assert_eq!(logged(&e, MENU_MANAGER_INSTANCE), vec![vec![1]]);
        assert_eq!(
            logged(&e, MENU_MANAGER_CREATE_MENU_BY_CLASS),
            vec![vec![0x7000, 0x3f1]]
        );
    }

    #[test]
    fn is_in_rendered_menu_asks_the_pipboy_when_it_is_open() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, INTERFACE_IS_IN_PIPBOY_MENU, 0);
        stub(&mut e, INTERFACE_GET_PIPBOY, 0x7777);
        stub(&mut e, PIPBOY_QUERY, 0);
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 0);
        // Not in the Pipboy menu: only the manager's flag and the Pipboy
        // test count.
        assert!(!e.call(0x0070_79b0, &[]).bool());
        e.mem.set_u8(manager + 0x168, 1);
        assert!(e.call(0x0070_79b0, &[]).bool());
        assert!(logged(&e, PIPBOY_QUERY).is_empty());

        // In the Pipboy menu and the Pipboy manager says no: false whatever
        // the flag.
        stub(&mut e, INTERFACE_IS_IN_PIPBOY_MENU, 1);
        assert!(!e.call(0x0070_79b0, &[]).bool());
        assert_eq!(logged(&e, PIPBOY_QUERY), vec![vec![0x7777]]);
        // ... and when it says yes the manager's flag decides.
        stub(&mut e, PIPBOY_QUERY, 1);
        assert!(e.call(0x0070_79b0, &[]).bool());
        e.mem.set_u8(manager + 0x168, 0);
        assert!(!e.call(0x0070_79b0, &[]).bool());
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 1);
        assert!(e.call(0x0070_79b0, &[]).bool());
    }

    #[test]
    fn fn_007079f0_is_the_flag_or_the_pipboy_test() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(0x200);
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 0);
        assert!(!e.call(0x0070_79f0, &args![object]).bool());
        e.call_log = Some(vec![]);
        e.mem.set_u8(object + 0x168, 1);
        e.call_log = Some(vec![]);
        assert!(logged(&e, MANAGER_IS_IN_PIPBOY_MENU).is_empty());
        e.mem.set_u8(object + 0x168, 0);
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 1);
        assert!(e.call(0x0070_79f0, &args![object]).bool());
    }

    #[test]
    fn is_rendered_menu_checks_the_two_traits_of_the_menu_tile() {
        let (mut e, _) = ui_engine();
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        let traits: Rc<RefCell<HashMap<u32, f32>>> = Rc::default();
        let table = traits.clone();
        e.register_double(TILE_GET_VALUE, move |_, a| {
            ret_float(*table.borrow().get(&a[1]).unwrap_or(&0.0))
        });
        let menu = e.mem.alloc(8);
        let tile = e.mem.alloc(8);
        // No menu, and a menu without a tile.
        assert!(!e.call(0x0070_7a30, &args![0u32]).bool());
        assert!(!e.call(0x0070_7a30, &args![menu]).bool());
        e.mem.set_u32(menu + 4, tile);
        let mut check = |faa: f32, fa4: f32| {
            traits.borrow_mut().insert(TRAIT_0FAA, faa);
            traits.borrow_mut().insert(TRAIT_0FA4, fa4);
            e.call(0x0070_7a30, &args![menu]).bool()
        };
        assert!(!check(0.0, 0.0));
        assert!(check(112.9, 0.0));
        assert!(!check(113.0, 0.0));
        assert!(check(0.0, 0x421 as f32));
        assert!(check(0.0, 0x41f as f32));
        assert!(check(0.0, 0x40c as f32));
        assert!(!check(0.0, 0x40d as f32));
    }

    #[test]
    fn get_current_rendered_menu_reads_the_manager() {
        let (mut e, manager) = ui_engine();
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 0);
        e.mem.set_u32(manager + 0x16c, 0x1111);
        e.mem.set_u32(manager + 0x174, 0x2222);
        assert_eq!(e.call(0x0070_7ad0, &[]).u32(), 0x1111);
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 1);
        assert_eq!(e.call(0x0070_7ad0, &[]).u32(), 0x2222);
    }

    #[test]
    fn fn_00707af0_picks_the_pipboy_or_the_current_menu() {
        let (mut e, _) = ui_engine();
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object + 0x16c, 0xaaaa);
        e.mem.set_u32(object + 0x174, 0xbbbb);
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 0);
        assert_eq!(e.call(0x0070_7af0, &args![object]).u32(), 0xaaaa);
        stub(&mut e, MANAGER_IS_IN_PIPBOY_MENU, 1);
        assert_eq!(e.call(0x0070_7af0, &args![object]).u32(), 0xbbbb);
    }

    #[test]
    fn is_current_rendered_menu_topmost_forwards_the_result() {
        let (mut e, _) = ui_engine();
        stub(&mut e, MANAGER_IS_CURRENT_RENDERED_MENU_TOPMOST, 1);
        assert!(e.call(0x0070_7b30, &[]).bool());
        stub(&mut e, MANAGER_IS_CURRENT_RENDERED_MENU_TOPMOST, 0);
        assert!(!e.call(0x0070_7b30, &[]).bool());
        assert_eq!(
            logged(&e, MANAGER_IS_CURRENT_RENDERED_MENU_TOPMOST).len(),
            2
        );
    }

    #[test]
    fn fn_00707b40_returns_the_byte_global() {
        let (mut e, _) = ui_engine();
        e.map(0x011c_7000, 0x1000);
        e.mem.set_u8(RETURNED_FLAG_011C70EB, 0x5a);
        assert_eq!(e.call(0x0070_7b40, &[]).u8(), 0x5a);
    }

    #[test]
    fn fn_00707b50_returns_the_byte_global() {
        let (mut e, _) = ui_engine();
        e.map(0x011c_7000, 0x1000);
        assert_eq!(e.call(0x0070_7b50, &[]).u8(), 0);
        e.mem.set_u8(RETURNED_FLAG_011C70EB, 1);
        assert_eq!(e.call(0x0070_7b50, &[]).u8(), 1);
    }

    #[test]
    fn fn_00707b60_computes_the_ratio_once() {
        let (mut e, _) = ui_engine();
        e.register(0x0071_5da0, |_, _| ret_float(9.0));
        stub(&mut e, RENDERER_DIMENSION, 4);
        assert_eq!(e.call(0x0070_7b60, &[]).f32(), 2.25);
        assert_eq!(e.global::<u32>(SCREEN_RATIO_GUARD) & 1, 1);
        // The value is cached: a new height or divisor changes nothing.
        e.register(0x0071_5da0, |_, _| ret_float(100.0));
        stub(&mut e, RENDERER_DIMENSION, 1);
        assert_eq!(e.call(0x0070_7b60, &[]).f32(), 2.25);
        assert_eq!(logged(&e, RENDERER_DIMENSION).len(), 1);
    }

    /// The doubles of the tab line: a string object that keeps its
    /// characters pointer at `+0`, tiles of `0x40` bytes whose name string
    /// is at `+0x20`, and trait values per tile.
    struct TabWorld {
        e: Engine,
        tile: u32,
        widths: Rc<RefCell<HashMap<u32, f32>>>,
        buttons: Rc<RefCell<Vec<u32>>>,
    }

    fn tab_world() -> TabWorld {
        let (mut e, _) = ui_engine();
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            ret(a[0])
        });
        e.register(STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(STRING_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(STRING_LENGTH, |e, a| {
            let characters = e.mem.u32(a[0]);
            let mut length = 0;
            while characters != 0 && e.mem.u8(characters + length) != 0 {
                length += 1;
            }
            ret(length)
        });
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        stub(&mut e, STRING_DESTROY, 0);
        e.register(TILE_TEXT_TO_TRAIT, |_, a| ret(a[0]));
        stub(&mut e, TILE_GET_MENU, 0x9000);
        stub(&mut e, TILE_UPDATE_ALL, 0);
        stub(&mut e, TILE_SET_VALUE, 0);
        stub(&mut e, TILE_SET_TEXT, 0);
        let buttons: Rc<RefCell<Vec<u32>>> = Rc::default();
        let made = buttons.clone();
        e.register_double(MENU_RENDER_TEMPLATE, move |e, _| {
            let button = e.mem.alloc(0x40);
            made.borrow_mut().push(button);
            ret(button)
        });
        let found = buttons.clone();
        e.register_double(TILE_GET_CHILD_BY_NAME, move |e, a| {
            let wanted = a[1];
            let child = found
                .borrow()
                .iter()
                .copied()
                .find(|b| e.mem.u32(b + 0x20) == wanted)
                .expect("no button with that name");
            ret(child)
        });
        let widths: Rc<RefCell<HashMap<u32, f32>>> = Rc::default();
        let table = widths.clone();
        e.register_double(TILE_GET_VALUE, move |_, a| {
            assert_eq!(a[1], TRAIT_0FB1);
            ret_float(*table.borrow().get(&a[0]).unwrap_or(&0.0))
        });
        let tile = e.mem.alloc(0x40);
        widths.borrow_mut().insert(tile, 100.0);
        e.call_log = Some(vec![]);
        TabWorld {
            e,
            tile,
            widths,
            buttons,
        }
    }

    #[test]
    fn initialize_tabline_lays_out_the_buttons() {
        let mut w = tab_world();
        let names: Vec<u32> = ["Alpha", "Beta", "Gamma"]
            .iter()
            .map(|n| put_text(&mut w.e, n))
            .collect();
        // The widths the buttons will report, in creation order.
        let widths = w.widths.clone();
        let buttons = w.buttons.clone();
        w.e.register_double(MENU_RENDER_TEMPLATE, move |e, _| {
            let button = e.mem.alloc(0x40);
            let width = 10.0 * (buttons.borrow().len() as f32 + 1.0);
            widths.borrow_mut().insert(button, width);
            buttons.borrow_mut().push(button);
            ret(button)
        });
        // The registered form takes the varargs as further words.
        w.e.call(
            0x0070_7be0,
            &args![w.tile, 5u32, names[0], names[1], names[2], 0u32],
        );
        let buttons = w.buttons.borrow().clone();
        assert_eq!(buttons.len(), 3);
        // Each button is rendered by the menu of the tile and named.
        let renders = logged(&w.e, MENU_RENDER_TEMPLATE);
        assert_eq!(renders.len(), 3);
        assert_eq!(renders[0], vec![0x9000, w.tile, TAB_BUTTON_TEMPLATE, 0]);
        for (index, button) in buttons.iter().enumerate() {
            assert_eq!(w.e.mem.u32(button + 0x20), names[index]);
        }
        let sets = logged(&w.e, TILE_SET_VALUE);
        let set_for = |tile: u32, trait_id: u32| -> Vec<u32> {
            sets.iter()
                .filter(|s| s[0] == tile && s[1] == trait_id)
                .map(|s| s[2])
                .collect()
        };
        for (index, button) in buttons.iter().enumerate() {
            assert_eq!(set_for(*button, 0xfac), vec![index as u32]);
            assert_eq!(set_for(*button, 0xfaa), vec![5 + index as u32]);
        }
        assert_eq!(logged(&w.e, TILE_UPDATE_ALL), vec![vec![0]; 3]);
        // 100 - (10 + 20 + 30) shared by 3 + 1 gaps.
        assert_eq!(set_for(w.tile, LEFT_LINE_LENGTH_NAME), vec![10]);
        assert_eq!(set_for(w.tile, BUTTON_COUNT_NAME), vec![3]);
        // x: the first at 10, the next after the previous width and a gap.
        assert_eq!(set_for(buttons[0], X_NAME), vec![10]);
        assert_eq!(set_for(buttons[1], X_NAME), vec![30]);
        assert_eq!(set_for(buttons[2], X_NAME), vec![60]);
        let texts = logged(&w.e, TILE_SET_TEXT);
        assert_eq!(texts.len(), 3);
        assert_eq!(texts[1], vec![buttons[1], 0xfc4, names[1], 1]);
        assert_eq!(logged(&w.e, STRING_DESTROY).len(), 1);
    }

    #[test]
    fn initialize_tabline_without_names_sets_the_whole_line_as_the_gap() {
        let mut w = tab_world();
        w.e.call(0x0070_7be0, &args![w.tile, 0u32, 0u32]);
        assert!(w.buttons.borrow().is_empty());
        let sets = logged(&w.e, TILE_SET_VALUE);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0], vec![w.tile, LEFT_LINE_LENGTH_NAME, 100]);
        assert_eq!(sets[1], vec![w.tile, BUTTON_COUNT_NAME, 0]);
        assert!(logged(&w.e, MENU_RENDER_TEMPLATE).is_empty());
    }

    #[test]
    fn fn_00707e10_assigns_the_name_at_0x20() {
        let (mut e, _) = ui_engine();
        stub(&mut e, STRING_ASSIGN, 0);
        e.call(0x0070_7e10, &args![0x5000u32, 0x6000u32]);
        assert_eq!(logged(&e, STRING_ASSIGN), vec![vec![0x5020, 0x6000]]);
    }

    /// A small `_snprintf`: `%d`, `%i`, `%s` and `%.Nf` (the `double` takes
    /// two words), writing at most `size - 1` characters and a NUL.
    fn fake_format(e: &mut Engine, a: &[u32]) -> Ret {
        let (dest, size, format) = (a[0], a[1], a[2]);
        let mut words = a[3..].iter().copied();
        let mut out = String::new();
        let template = text_at(e, format);
        let mut chars = template.chars().peekable();
        while let Some(c) = chars.next() {
            if c != '%' {
                out.push(c);
                continue;
            }
            let mut precision = None;
            if chars.peek() == Some(&'.') {
                chars.next();
                let mut digits = String::new();
                while let Some(d) = chars.peek().copied().filter(char::is_ascii_digit) {
                    digits.push(d);
                    chars.next();
                }
                precision = digits.parse::<usize>().ok();
            }
            match chars.next() {
                Some('d') | Some('i') => {
                    out.push_str(&(words.next().unwrap() as i32).to_string());
                }
                Some('s') => {
                    let pointer = words.next().unwrap();
                    out.push_str(&text_at(e, pointer));
                }
                Some('f') => {
                    let low = words.next().unwrap() as u64;
                    let high = words.next().unwrap() as u64;
                    let value = f64::from_bits(low | high << 32);
                    out.push_str(&format!("{:.*}", precision.unwrap_or(6), value));
                }
                other => panic!("format {other:?} is not supported by the test double"),
            }
        }
        let bytes: Vec<u8> = out
            .bytes()
            .take((size as usize).saturating_sub(1))
            .collect();
        e.mem.set_cstr(dest, &bytes);
        ret(bytes.len() as u32)
    }

    /// What the test world answers for the item stats display.
    #[derive(Default)]
    struct StatsCfg {
        item_value: f32,
        weight: f32,
        weapon_weight: f32,
        weight_flag: bool,
        mod_slots: u32,
        mods: HashMap<u32, u32>,
        mod_effects: HashMap<u32, u32>,
        /// `GetItemHealth(0)` and `GetItemHealth(1)`.
        health: [f32; 2],
        cast: u32,
        flag80: bool,
        flag08: bool,
        /// The values `004be060` / `004be180` return.
        ratings: [f32; 2],
        form_health: u32,
        damage: f32,
        damage_display: f32,
        field_19c: u32,
        projectiles: u32,
        current_ammo: u32,
        ammo_name: String,
        total: u32,
        rounds: u32,
        regen: f32,
        entry: HashMap<u32, f32>,
        enchanting: u32,
        enchanting_flag: bool,
        ownership: u32,
        ammo_text: String,
        effect_text: String,
        saved_object: u32,
        held_item: u32,
        held_ammo_item: u32,
        settings: HashMap<u32, String>,
        extent: f32,
    }

    /// One recorded write to a tile.
    #[derive(Debug, Clone, PartialEq)]
    enum Write {
        Text(String),
        Float(f32),
        Int(u32),
    }

    struct Stats {
        e: Engine,
        cfg: Rc<RefCell<StatsCfg>>,
        writes: Rc<RefCell<Vec<(u32, u32, Write)>>>,
        item: u32,
        form: u32,
        display: u32,
        cells: Vec<u32>,
    }

    impl Stats {
        fn new(form_type: u8) -> Stats {
            let (mut e, _) = text_engine();
            for page in [
                0x0101_1000,
                0x0101_2000,
                0x0101_7000,
                0x0101_8000,
                0x0101_9000,
                0x0102_0000,
                0x0102_3000,
                0x0102_f000,
                0x0103_0000,
                0x0103_5000,
                0x0104_7000,
                0x0118_a000,
            ] {
                e.map(page, 0x1000);
            }
            e.set_global(PERCENT_DIVISOR, 100.0f64);
            e.set_global(HEAVY_WEIGHT, 10.0f64);
            e.set_global(MOD_ROW_HEIGHT, 41.0f64);
            e.set_global(ONE, 1.0f64);
            e.set_global(TWO, 2.0f64);
            e.set_global(TWENTY, 20.0f64);
            e.set_global(CELL_10_TRAIT_FA9, 255.0f32);
            for (at, text) in [
                (DASHES, "--"),
                (FORMAT_INTEGER, "%d"),
                (FORMAT_STRING, "%s"),
                (FORMAT_AMMO, "%s (%i/%i)"),
                (FORMAT_PROJECTILES, "%.1fx%d"),
                (FORMAT_NO_DECIMAL, "%.0f"),
                (FORMAT_ONE_DECIMAL, "%.1f"),
                (FLOAT_FORMAT, "%.2f"),
                (MOD_1_TEXT, "Mod 1"),
                (MOD_2_TEXT, "Mod 2"),
                (MOD_3_TEXT, "Mod 3"),
                (ENGLISH_TEXT, "ENGLISH"),
            ] {
                put_at(&mut e, at, text);
            }
            let cfg: Rc<RefCell<StatsCfg>> = Rc::default();
            cfg.borrow_mut().extent = 200.0;
            cfg.borrow_mut().item_value = 2.5;
            cfg.borrow_mut().weight = 1.5;
            let writes: Rc<RefCell<Vec<(u32, u32, Write)>>> = Rc::default();

            // The objects: the player, the form, the item (count 4), the
            // display tile with 13 cells in its child list, and the two
            // condition tiles.
            let player = e.mem.alloc(0x800);
            e.set_global(PLAYER, player);
            let form = e.mem.alloc(0x200);
            e.mem.set_u8(form + 4, form_type);
            let item = e.mem.alloc(0x40);
            e.mem.set_u32(item + 4, 4);
            e.mem.set_u32(item + 8, form);
            let display = e.mem.alloc(0x40);
            let mut cells = vec![];
            let mut next_node = 0;
            for _ in 0..13 {
                let cell = e.mem.alloc(0x40);
                let node = e.mem.alloc(0x10);
                e.mem.set_u32(node + 4, next_node);
                e.mem.set_u32(node + 8, cell);
                next_node = node;
                cells.insert(0, cell);
            }
            // The list's first node is the word at `+4` of the list at
            // `display + 4`.
            e.mem.set_u32(display + 8, next_node);
            let arrows = e.mem.alloc(0x40);
            let meter = e.mem.alloc(0x40);
            // The saved acquire object: its slots `0x148` and `0x14c` give
            // the held item and the held ammunition item from `cfg`.
            let acquire = e.mem.alloc(8);
            let table = 0x0200_4000;
            e.put_vtable(table, &[0; 0x60]);
            e.mem.set_u32(acquire, table);
            e.mem.set_u32(table + 0x148, 0x0200_0148);
            e.mem.set_u32(table + 0x14c, 0x0200_014c);
            let c = cfg.clone();
            e.register_double(0x0200_0148, move |_, _| ret(c.borrow().held_item));
            let c = cfg.clone();
            e.register_double(0x0200_014c, move |_, _| ret(c.borrow().held_ammo_item));
            cfg.borrow_mut().saved_object = acquire;
            cfg.borrow_mut()
                .settings
                .insert(LANGUAGE_SETTING, "ENGLISH".to_string());

            e.register(TILE_TEXT_TO_TRAIT, |_, a| ret(a[0]));
            e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
            e.register(ITEM_GET_FORM, |e, a| ret(e.mem.u32(a[0] + 8)));
            e.register(FORM_GET_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
            e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
            e.register(IDENTITY, |_, a| ret(a[0]));
            e.register(LIST_ITERATE, |e, a| {
                let node = e.mem.u32(a[1]);
                let next = e.mem.u32(node + 4);
                e.mem.set_u32(a[1], next);
                ret(node + 8)
            });
            e.register_double(TILE_FIND_CHILD, move |_, a| {
                if a[1] == CONDITION_ARROWS_NAME {
                    ret(arrows)
                } else {
                    ret(meter)
                }
            });
            let c = cfg.clone();
            e.register_double(TILE_GET_VALUE, move |_, a| {
                let c = c.borrow();
                // The display's extent; the meter's two traits.
                ret_float(match a[1] {
                    TRAIT_0FB0 if a[0] == display => c.extent,
                    TRAIT_0FB0 => 12.0,
                    TRAIT_0FA2 => 7.0,
                    _ => 0.0,
                })
            });
            let w = writes.clone();
            e.register_double(TILE_SET_TEXT, move |e, a| {
                let text = text_at(e, a[2]);
                assert_eq!(a[3], 1);
                w.borrow_mut().push((a[0], a[1], Write::Text(text)));
                ret(0)
            });
            let w = writes.clone();
            e.register_double(TILE_SET_FLOAT, move |_, a| {
                assert_eq!(a[3], 1);
                w.borrow_mut()
                    .push((a[0], a[1], Write::Float(f32::from_bits(a[2]))));
                ret(0)
            });
            let w = writes.clone();
            e.register_double(TILE_SET_VALUE, move |_, a| {
                w.borrow_mut().push((a[0], a[1], Write::Int(a[2])));
                ret(0)
            });
            let c = cfg.clone();
            e.register_double(ITEM_GET_VALUE, move |_, _| ret_float(c.borrow().item_value));
            let c = cfg.clone();
            e.register_double(PLAYER_WEIGHT_FLAG, move |_, _| {
                ret(c.borrow().weight_flag as u32)
            });
            let c = cfg.clone();
            e.register_double(GET_FORM_WEIGHT, move |_, a| {
                assert_eq!(a[1], c.borrow().weight_flag as u32);
                ret_float(c.borrow().weight)
            });
            let c = cfg.clone();
            e.register_double(ITEM_GET_MOD_SLOTS, move |_, _| ret(c.borrow().mod_slots));
            let c = cfg.clone();
            e.register_double(ITEM_FORM_GET_MOD, move |_, a| {
                ret(*c.borrow().mods.get(&a[1]).unwrap_or(&0))
            });
            let c = cfg.clone();
            e.register_double(ITEM_HAS_MOD_EFFECT, move |_, a| {
                ret(*c.borrow().mod_effects.get(&a[1]).unwrap_or(&0))
            });
            let c = cfg.clone();
            e.register_double(ITEM_GET_HEALTH, move |_, a| {
                ret_float(c.borrow().health[a[1] as usize])
            });
            let c = cfg.clone();
            e.register_double(DYNAMIC_CAST, move |_, a| {
                assert_eq!(
                    (a[1], a[2], a[3]),
                    (0, ARMOR_CAST_SOURCE, ARMOR_CAST_TARGET)
                );
                ret(c.borrow().cast)
            });
            let c = cfg.clone();
            e.register_double(ARMOR_SUBOBJECT_FLAG_80, move |_, _| {
                ret(c.borrow().flag80 as u32)
            });
            let c = cfg.clone();
            e.register_double(ARMOR_SUBOBJECT_FLAG_08, move |_, _| {
                ret(c.borrow().flag08 as u32)
            });
            let c = cfg.clone();
            e.register_double(ARMOR_RATING_INPUT_A, move |_, _| {
                ret_float(c.borrow().ratings[0])
            });
            let c = cfg.clone();
            e.register_double(ARMOR_RATING_INPUT_B, move |_, _| {
                ret_float(c.borrow().ratings[1])
            });
            // The rating is the integer input times the condition.
            e.register(CALC_ARMOR_RATING, |_, a| {
                ret_float(a[0] as i32 as f32 * f32::from_bits(a[1]))
            });
            e.register(ROUND_FLOAT, |_, a| ret_float(f32::from_bits(a[0])));
            e.register(ROUND_DIVIDE, |_, a| ret_float(f32::from_bits(a[0])));
            let c = cfg.clone();
            e.register_double(GET_FORM_HEALTH, move |_, _| ret(c.borrow().form_health));
            e.register(MIN_FLOAT, |_, a| {
                ret_float(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
            });
            let c = cfg.clone();
            e.register_double(GET_SAVED_ACQUIRE_OBJECT, move |_, a| {
                assert_eq!(a[0], player);
                ret(c.borrow().saved_object)
            });
            let c = cfg.clone();
            e.register_double(WEAPON_GET_WEIGHT, move |_, _| {
                ret_float(c.borrow().weapon_weight)
            });
            let c = cfg.clone();
            e.register_double(WEAPON_FIELD_19C, move |_, _| ret(c.borrow().field_19c));
            let c = cfg.clone();
            e.register_double(CALC_WEAPON_DAMAGE, move |_, a| {
                assert_eq!(a.len(), 13);
                ret_float(c.borrow().damage)
            });
            let c = cfg.clone();
            e.register_double(CALC_WEAPON_DAMAGE_FOR_DISPLAY, move |_, _| {
                ret_float(c.borrow().damage_display)
            });
            let c = cfg.clone();
            e.register_double(WEAPON_GET_NUM_PROJECTILES, move |_, _| {
                ret(c.borrow().projectiles)
            });
            let c = cfg.clone();
            e.register_double(WEAPON_GET_CURRENT_AMMO, move |_, _| {
                ret(c.borrow().current_ammo)
            });
            let c = cfg.clone();
            e.register_double(AMMO_NAME, move |e, _| {
                let name = c.borrow().ammo_name.clone();
                if name.is_empty() {
                    ret(0)
                } else {
                    ret(put_text(e, &name))
                }
            });
            e.register(GET_NAME, |_, a| ret(a[0]));
            let c = cfg.clone();
            e.register_double(GET_INVENTORY_CHANGES, move |_, _| ret(0x1234));
            e.register_double(INVENTORY_GET_COUNT, move |_, _| ret(c.borrow().total));
            let c = cfg.clone();
            e.register_double(WEAPON_GET_CLIP_ROUNDS, move |_, _| ret(c.borrow().rounds));
            e.register(HELPER_4A8F20, |_, a| ret(a[0].min(a[1])));
            let c = cfg.clone();
            e.register_double(WEAPON_GET_AMMO_REGEN_RATE, move |_, _| {
                ret_float(c.borrow().regen)
            });
            let c = cfg.clone();
            e.register_double(HANDLE_ENTRY_POINT, move |e, a| {
                let value = c.borrow().entry.get(&a[0]).copied().unwrap_or(0.0);
                let out = *a.last().unwrap();
                e.mem.set_f32(out, value);
                ret(0)
            });
            let c = cfg.clone();
            e.register_double(GET_FORM_ENCHANTING, move |_, _| ret(c.borrow().enchanting));
            let c = cfg.clone();
            e.register_double(SPELL_ITEM_FLAG, move |_, _| {
                ret(c.borrow().enchanting_flag as u32)
            });
            let c = cfg.clone();
            e.register_double(ITEM_GET_OWNERSHIP, move |_, _| ret(c.borrow().ownership));
            let c = cfg.clone();
            e.register_double(AMMO_BUILD_MENU_STRING, move |e, a| {
                let text = c.borrow().ammo_text.clone();
                e.mem.set_cstr(a[1], text.as_bytes());
                ret(0)
            });
            let c = cfg.clone();
            e.register_double(EFFECT_LIST_BUILD_MENU_STRING, move |e, a| {
                let text = c.borrow().effect_text.clone();
                e.mem.set_cstr(a[1], text.as_bytes());
                ret(0)
            });
            e.register(FORMAT_TEXT, fake_format);
            e.register(0x0040_6d30, |e, a| {
                let text = text_at(e, a[2]);
                let bytes: Vec<u8> = text.bytes().take(a[1] as usize - 1).collect();
                e.mem.set_cstr(a[0], &bytes);
                ret(0)
            });
            e.register(STRING_LENGTH_C, |e, a| ret(text_at(e, a[0]).len() as u32));
            let c = cfg.clone();
            e.register_double(SETTING_GET_STRING, move |e, a| {
                let text = c
                    .borrow()
                    .settings
                    .get(&a[0])
                    .cloned()
                    .unwrap_or_else(|| format!("S{:x}", a[0]));
                ret(put_text(e, &text))
            });
            e.register(COMPARE_TEXT, |e, a| {
                let (left, right) = (text_at(e, a[0]), text_at(e, a[1]));
                ret(left.cmp(&right) as i32 as u32)
            });
            let weight_setting = e.mem.alloc(8);
            let factor_setting = e.mem.alloc(8);
            e.mem.set_f32(weight_setting, 20.0);
            e.mem.set_f32(factor_setting, 0.5);
            e.register_double(FLOAT_SETTING_GET, move |_, a| {
                ret(if a[0] == WEIGHT_LIMIT_SETTING {
                    weight_setting
                } else {
                    assert_eq!(a[0], WEIGHT_FACTOR_SETTING);
                    factor_setting
                })
            });
            e.call_log = Some(vec![]);
            Stats {
                e,
                cfg,
                writes,
                item,
                form,
                display,
                cells,
            }
        }

        fn run(&mut self) {
            let (item, display) = (self.item, self.display);
            self.e.call(0x0070_7e30, &args![item, display]);
        }

        /// The last write of `trait_id` to cell `cell`.
        fn last(&self, cell: usize, trait_id: u32) -> Option<Write> {
            self.writes
                .borrow()
                .iter()
                .rev()
                .find(|(tile, id, _)| *tile == self.cells[cell] && *id == trait_id)
                .map(|(_, _, write)| write.clone())
        }

        fn title(&self, cell: usize) -> Option<String> {
            match self.last(cell, TITLE_NAME) {
                Some(Write::Text(text)) => Some(text),
                _ => None,
            }
        }

        fn value(&self, cell: usize) -> Option<String> {
            match self.last(cell, VALUE_NAME) {
                Some(Write::Text(text)) => Some(text),
                _ => None,
            }
        }

        /// Which cells the visibility pass shows, as a bit mask over the
        /// cell indices.
        fn shown(&self) -> u32 {
            let mut mask = 0;
            for cell in 0..13 {
                if self.last(cell, TRAIT_0FA3) == Some(Write::Int(1)) {
                    mask |= 1 << cell;
                }
            }
            mask
        }

        fn height(&self, cell: usize) -> f32 {
            match self.last(cell, TRAIT_0FA2) {
                Some(Write::Float(height)) => height,
                other => panic!("no height for cell {cell}: {other:?}"),
            }
        }
    }

    #[test]
    fn populate_item_stats_display_does_nothing_without_a_display() {
        let mut s = Stats::new(0x30);
        let item = s.item;
        s.e.call(0x0070_7e30, &args![item, 0u32]);
        assert!(s.writes.borrow().is_empty());
        // The value and the weight are still asked first.
        assert_eq!(logged(&s.e, ITEM_GET_VALUE).len(), 1);
        assert_eq!(logged(&s.e, GET_FORM_WEIGHT).len(), 1);
    }

    #[test]
    fn populate_item_stats_display_plain_item_shows_value_and_weight() {
        let mut s = Stats::new(0x30);
        s.run();
        // Value 2.5 each, 4 of them: "%.0f" for a value of at least 1.
        assert_eq!(s.value(3).as_deref(), Some("10"));
        // Weight 1.5 each: "%.2f".
        assert_eq!(s.value(2).as_deref(), Some("6.00"));
        // flags 0xc: cells 2 and 3 only; cell 10 is hidden but still set.
        assert_eq!(s.shown(), 0xc);
        assert_eq!(s.last(10, TRAIT_0FA3), Some(Write::Int(0)));
        assert_eq!(s.last(10, TRAIT_0FA9), Some(Write::Float(255.0)));
        // The condition arrows copy the meter's two traits.
        let writes = s.writes.borrow();
        assert_eq!(writes[0].1, TRAIT_0FB0);
        assert_eq!(writes[0].2, Write::Float(12.0));
        assert_eq!(writes[1].2, Write::Float(7.0));
        drop(writes);
        // The heights: 200 / 2 - 20, unchanged without mods.
        for cell in 6..10 {
            assert_eq!(s.height(cell), 80.0);
        }
    }

    #[test]
    fn populate_item_stats_display_small_values_get_one_decimal_and_heavy_ones_the_factor() {
        let mut s = Stats::new(0x30);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.item_value = 0.5;
            cfg.weight = 8.0;
            cfg.entry.insert(0x37, 1.0);
        }
        s.run();
        assert_eq!(s.value(3).as_deref(), Some("2.0"));
        // Entry point 0x37 > 0 and the weight (8) is at most the limit
        // setting (20): halved by the factor setting, times 4 items.
        assert_eq!(s.value(2).as_deref(), Some("16.00"));

        // A weight above the limit stays; no value and no weight give "--".
        let mut s = Stats::new(0x30);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.item_value = 0.0;
            cfg.weight = 30.0;
            cfg.entry.insert(0x37, 1.0);
        }
        s.run();
        assert_eq!(s.value(3).as_deref(), Some("--"));
        assert_eq!(s.value(2).as_deref(), Some("120.00"));
        let mut s = Stats::new(0x30);
        s.cfg.borrow_mut().weight = 0.0;
        s.run();
        assert_eq!(s.value(2).as_deref(), Some("--"));
    }

    #[test]
    fn populate_item_stats_display_ammo_shows_its_text_when_there_is_one() {
        let mut s = Stats::new(0x29);
        s.cfg.borrow_mut().ammo_text = "Piercing".to_string();
        s.run();
        assert_eq!(s.value(6).as_deref(), Some("Piercing"));
        // flags 0xc | 0x40: the text cell is shown (the effect pass keeps
        // bit 0x40 for ammunition).
        assert_eq!(s.shown(), 0x4c);

        let mut s = Stats::new(0x29);
        s.run();
        assert_eq!(s.value(6), None);
        assert_eq!(s.shown(), 0xc);
    }

    #[test]
    fn populate_item_stats_display_armor_shows_ratings_and_condition() {
        let mut s = Stats::new(0x18);
        let cast = s.e.mem.alloc(0x100);
        s.e.set_global(ARMOR_SETTING_B, 0xb0b0u32);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.cast = cast;
            cfg.flag08 = true;
            cfg.health = [100.0, 50.0];
            cfg.form_health = 200;
            cfg.ratings = [12.7, 20.0];
        }
        s.run();
        // The condition is 50 / 100 = 0.5: the ratings are 12 * 0.5 and
        // 20 * 0.5.
        assert_eq!(s.value(0).as_deref(), Some("6"));
        assert_eq!(s.value(12).as_deref(), Some("10"));
        // The title is the string of the second setting, chosen by the flag
        // of the sub-object at +0x70 of the cast object.
        assert_eq!(s.title(5).as_deref(), Some("Sb0b0"));
        assert_eq!(
            logged(&s.e, ARMOR_SUBOBJECT_FLAG_80),
            vec![vec![cast + 0x70]]
        );
        // 100 / 200 of the health, at most 1.
        assert_eq!(s.last(4, 0x1009), Some(Write::Float(0.5)));
        assert_eq!(s.last(4, 0x100a), Some(Write::Int(0)));
        // flags 0x3c | 1 | 0x1000.
        assert_eq!(s.shown(), 0x103d);
        // Cells 4 and 5 are shown: the height doubles.
        assert_eq!(s.height(6), 160.0);
    }

    #[test]
    fn populate_item_stats_display_armor_without_ratings_or_cast() {
        let mut s = Stats::new(0x1a);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.health = [300.0, 50.0];
            cfg.form_health = 0;
        }
        s.run();
        // No cast object: the title stays "--"; no ratings: "--" in cell 0.
        assert_eq!(s.title(5).as_deref(), Some("--"));
        assert_eq!(s.value(0).as_deref(), Some("--"));
        assert_eq!(s.value(12), None);
        assert_eq!(s.shown(), 0x3d);
        // Health 300 over the form health 0, taken as 1: capped at 1.
        assert_eq!(s.last(4, 0x1009), Some(Write::Float(1.0)));
        // Only the first form health query was made.
        assert_eq!(logged(&s.e, GET_FORM_HEALTH).len(), 1);
    }

    #[test]
    fn populate_item_stats_display_armor_picks_the_title_setting() {
        for (flag80, flag08, setting) in [
            (true, true, ARMOR_SETTING_A),
            (false, true, ARMOR_SETTING_B),
            (false, false, ARMOR_SETTING_C),
        ] {
            let mut s = Stats::new(0x18);
            let cast = s.e.mem.alloc(0x100);
            s.e.set_global(setting, 0xc0deu32);
            {
                let mut cfg = s.cfg.borrow_mut();
                cfg.cast = cast;
                cfg.flag80 = flag80;
                cfg.flag08 = flag08;
                cfg.health = [1.0, 1.0];
            }
            s.run();
            assert_eq!(s.title(5).as_deref(), Some("Sc0de"));
        }
    }

    #[test]
    fn populate_item_stats_display_weapon_shows_damage_clip_and_ammo() {
        let mut s = Stats::new(0x28);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.weapon_weight = 12.0;
            cfg.entry.insert(0x49, 0.5);
            cfg.entry.insert(0x35, 2.9);
            cfg.health = [0.0, 80.0];
            cfg.damage = 25.4;
            cfg.damage_display = 22.6;
            cfg.field_19c = 7;
            cfg.projectiles = 1;
            cfg.current_ammo = 0x9100;
            cfg.ammo_name = "Slugs".to_string();
            cfg.total = 30;
            cfg.rounds = 12;
        }
        s.run();
        assert_eq!(s.value(1).as_deref(), Some("25"));
        assert_eq!(s.value(11).as_deref(), Some("22"));
        // 7 less the truncated entry point 0x35.
        assert_eq!(s.value(10).as_deref(), Some("5"));
        // 12 loaded of 30 in the inventory.
        assert_eq!(s.title(5).as_deref(), Some("Slugs (12/18)"));
        // The meter shows the condition 0.8; the 0x100a trait is 1.
        assert_eq!(s.last(4, 0x1009), Some(Write::Float(0.8)));
        assert_eq!(s.last(4, 0x100a), Some(Write::Int(1)));
        // The weight 12 is at least 10: entry point 0x49 halves it; the
        // value of 4 items is 4 * 6.
        assert_eq!(s.value(2).as_deref(), Some("24.00"));
        // flags 0xc3e, with bit 0x400 shown for cell 10.
        assert_eq!(s.shown(), 0xc3e);
        // The entry point 0x49 got 1.0 first, then the player and the
        // weapon for 0x35.
        let entries = logged(&s.e, HANDLE_ENTRY_POINT);
        assert_eq!(entries[0][0], 0x49);
        assert_eq!(entries[1][0], 0x35);
        assert_eq!(entries[1][2], s.form);
        // The damage call gets 13 words: the player's actor values (+0xa4),
        // the weapon, the condition, 1, the item, two zeros, -1, two 0.0
        // floats, two zeros and the saved ammo form (0).
        let player = s.e.global::<u32>(PLAYER);
        let damage = logged(&s.e, CALC_WEAPON_DAMAGE);
        assert_eq!(
            damage[0],
            vec![
                player + 0xa4,
                s.form,
                0.8f32.to_bits(),
                1,
                s.item,
                0,
                0,
                0xffff_ffff,
                0,
                0,
                0,
                0,
                0
            ]
        );
    }

    #[test]
    fn populate_item_stats_display_weapon_with_several_projectiles_and_regeneration() {
        let mut s = Stats::new(0x28);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.weapon_weight = 3.0;
            cfg.health = [0.0, 0.0];
            cfg.damage = 10.0;
            cfg.damage_display = 22.6;
            cfg.field_19c = 1;
            cfg.entry.insert(0x35, 5.0);
            cfg.projectiles = 8;
            cfg.current_ammo = 0x9100;
            cfg.ammo_name = "Cells".to_string();
            cfg.total = 3;
            cfg.rounds = 12;
            cfg.regen = 0.25;
        }
        s.run();
        // No damage query for the display when the condition is not above 0.
        assert!(logged(&s.e, CALC_WEAPON_DAMAGE_FOR_DISPLAY).is_empty());
        assert_eq!(s.value(11).as_deref(), Some("0.0x8"));
        // The strength-like word 1 less 5 is raised to 0.
        assert_eq!(s.value(10).as_deref(), Some("0"));
        // A regenerating weapon shows only the ammunition name.
        assert_eq!(s.title(5).as_deref(), Some("Cells"));
        // The weight is under 10: no entry point 0x49.
        assert!(!logged(&s.e, HANDLE_ENTRY_POINT)
            .iter()
            .any(|c| c[0] == 0x49));

        // With a condition above 0 the per-shot damage is shared.
        let mut s = Stats::new(0x28);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.health = [0.0, 100.0];
            cfg.damage_display = 22.6;
            cfg.projectiles = 8;
        }
        s.run();
        assert_eq!(s.value(11).as_deref(), Some("2.8x8"));
        // No ammunition: "--".
        assert_eq!(s.title(5).as_deref(), Some("--"));
    }

    #[test]
    fn populate_item_stats_display_weapon_reads_the_held_weapon() {
        let mut s = Stats::new(0x28);
        // The saved acquire object: slot 0x148 gives an item with this
        // weapon's form, slot 0x14c an ammunition item.
        let held_item = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(held_item + 8, s.form);
        let ammo_form = s.e.mem.alloc(0x40);
        let ammo_item = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(ammo_item + 8, ammo_form);
        s.e.mem.set_u32(ammo_item + 4, 9);
        // The two items point to the same record: the loaded count is the
        // word at +4 of the held ammunition item.
        let record = s.e.mem.alloc(8);
        s.e.mem.set_u32(record, 0x77);
        let (item, held) = (s.item, held_item);
        s.e.mem.set_u32(item, record);
        s.e.mem.set_u32(held, record);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.held_item = held_item;
            cfg.held_ammo_item = ammo_item;
            cfg.health = [0.0, 100.0];
            cfg.current_ammo = 0x9100;
            cfg.ammo_name = "Rounds".to_string();
            cfg.total = 40;
            cfg.projectiles = 1;
        }
        s.run();
        let player = s.e.global::<u32>(PLAYER);
        // The damage query gets the ammunition form; the projectile query
        // the player as process.
        assert_eq!(logged(&s.e, CALC_WEAPON_DAMAGE)[0][12], ammo_form);
        assert_eq!(
            logged(&s.e, WEAPON_GET_NUM_PROJECTILES)[0],
            vec![s.form, 0, 0, player]
        );
        // The loaded count is 9: "(9/31)".
        assert_eq!(s.title(5).as_deref(), Some("Rounds (9/31)"));
    }

    #[test]
    fn populate_item_stats_display_effects_and_descriptions_fill_cell_6() {
        // Type 0x1d: the ownership's effect list at +0x3c.
        let mut s = Stats::new(0x1d);
        let ownership = s.e.mem.alloc(0x80);
        s.e.mem.set_u32(ownership + 0x3c + 4, 0x55);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.ownership = ownership;
            cfg.effect_text = "Heal 5".to_string();
        }
        s.run();
        assert_eq!(s.value(6).as_deref(), Some("Heal 5"));
        assert_eq!(s.shown() & 0x40, 0x40);
        assert_eq!(
            logged(&s.e, EFFECT_LIST_BUILD_MENU_STRING)[0][0],
            ownership + 0x3c
        );

        // An empty text clears the bit again.
        let mut s = Stats::new(0x1d);
        let ownership = s.e.mem.alloc(0x80);
        s.e.mem.set_u32(ownership + 0x3c + 4, 0x55);
        s.cfg.borrow_mut().ownership = ownership;
        s.run();
        assert_eq!(s.value(6), None);
        assert_eq!(s.shown() & 0x40, 0);

        // Type 0x2f: the form's own list at +0x3c.
        let mut s = Stats::new(0x2f);
        s.e.mem.set_u32(s.form + 0x3c + 4, 0x55);
        s.cfg.borrow_mut().effect_text = "Rad 2".to_string();
        s.run();
        assert_eq!(s.value(6).as_deref(), Some("Rad 2"));

        // Any other form with an enchantment (not flagged): its list at +0x24.
        let mut s = Stats::new(0x18);
        let enchantment = s.e.mem.alloc(0x80);
        s.e.mem.set_u32(enchantment + 0x24 + 4, 0x55);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.enchanting = enchantment;
            cfg.effect_text = "Glow".to_string();
            cfg.health = [1.0, 1.0];
        }
        s.run();
        assert_eq!(s.value(6).as_deref(), Some("Glow"));
        assert_eq!(logged(&s.e, SPELL_ITEM_FLAG)[0], vec![enchantment, 4]);
        // With the flag set the enchantment is ignored.
        let mut s = Stats::new(0x18);
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.enchanting = enchantment;
            cfg.enchanting_flag = true;
            cfg.health = [1.0, 1.0];
        }
        s.run();
        assert!(logged(&s.e, EFFECT_LIST_BUILD_MENU_STRING).is_empty());

        // Type 0x67: the description from slot 0x10 of the object at +0x74.
        let mut s = Stats::new(0x67);
        let table = 0x0200_5000;
        s.e.put_vtable(table, &[0; 8]);
        s.e.mem.set_u32(s.form + 0x74, table);
        s.e.mem.set_u32(table + 0x10, 0x0200_0010);
        let text = put_text(&mut s.e, "A note");
        stub(&mut s.e, 0x0200_0010, text);
        s.run();
        assert_eq!(s.value(6).as_deref(), Some("A note"));
        assert_eq!(
            logged(&s.e, 0x0200_0010)[0],
            vec![s.form + 0x74, 0, 0x4353_4544]
        );
        assert_eq!(s.shown() & 0x40, 0x40);
    }

    #[test]
    fn populate_item_stats_display_mod_slots() {
        let mut s = Stats::new(0x30);
        let mod_item = s.e.mem.alloc(0x80);
        s.e.mem.set_cstr(mod_item + 0x30, b"Laser");
        {
            let mut cfg = s.cfg.borrow_mut();
            cfg.mod_slots = 7;
            cfg.mods.insert(1, mod_item);
        }
        s.run();
        // The game setting names are pointers; GET_NAME double returns the
        // pointer it was given less 0x30.
        assert_eq!(s.title(7).as_deref(), Some("S11d4024"));
        assert_eq!(s.value(7).as_deref(), Some("Laser"));
        assert_eq!(s.value(8).as_deref(), Some("Mod 2"));
        assert_eq!(s.value(9).as_deref(), Some("Mod 3"));
        // Only the first slot titles the group.
        assert_eq!(s.title(8), None);
        assert_eq!(s.title(9), None);
        // flags 0xc | 0x80 | 0x100 | 0x200.
        assert_eq!(s.shown(), 0x38c);
        assert_eq!(s.height(8), 80.0 + 41.0);
        assert_eq!(s.height(9), 80.0 + 82.0);

        // Only the second and third slots: the second titles the group.
        let mut s = Stats::new(0x30);
        s.cfg.borrow_mut().mod_slots = 2;
        s.run();
        assert_eq!(s.title(8).as_deref(), Some("S11d4024"));
        assert_eq!(s.value(8).as_deref(), Some("Mod 2"));
        assert_eq!(s.height(8), 80.0);
        assert_eq!(s.height(9), 80.0 + 41.0);
        let mut s = Stats::new(0x30);
        s.cfg.borrow_mut().mod_slots = 4;
        s.run();
        assert_eq!(s.title(9).as_deref(), Some("S11d4024"));
        assert_eq!(s.height(9), 80.0);
    }

    #[test]
    fn populate_item_stats_display_titles_other_languages() {
        let mut s = Stats::new(0x30);
        s.cfg
            .borrow_mut()
            .settings
            .insert(LANGUAGE_SETTING, "FRENCH".to_string());
        s.run();
        for (setting, cell) in LANGUAGE_TITLES {
            assert_eq!(s.title(cell), Some(format!("S{setting:x}")));
        }
        // English leaves them alone.
        let mut s = Stats::new(0x30);
        s.cfg
            .borrow_mut()
            .settings
            .insert(LANGUAGE_SETTING, "ENGLISH".to_string());
        s.run();
        assert_eq!(s.title(0), None);
    }

    /// Doubles for the menu creators `HandleQueuedMenuOpen` calls, on top of
    /// [`menu_engine`].
    fn queue_engine() -> (Engine, u32) {
        let (mut e, manager) = menu_engine(false);
        stub(&mut e, QUEUE_LOCK_ENTER, 0);
        stub(&mut e, QUEUE_LOCK_LEAVE, 0);
        stub(&mut e, CREATE_CONTAINER_MENU, 1);
        stub(&mut e, DIALOG_BLOCKED, 0);
        stub(&mut e, CREATE_DIALOG_MENU, 1);
        stub(&mut e, RACE_SEX_MENU_CREATE, 1);
        stub(&mut e, START_MENU_CREATE, 1);
        stub(&mut e, HUD_SET_MENU_MODE, 0);
        stub(&mut e, MESSAGE_MENU_CREATE, 1);
        stub(&mut e, DYNAMIC_CAST, 0x7c7c);
        stub(&mut e, COMPANION_WHEEL_MENU_CREATE, 1);
        (e, manager)
    }

    fn queue_request(e: &mut Engine, kind: u32, first: u32, second: u32, third: u32, fourth: u32) {
        e.mem.set_u32(QUEUE_KIND, kind);
        e.mem.set_u32(QUEUE_FIRST, first);
        e.mem.set_u32(QUEUE_SECOND, second);
        e.mem.set_u32(QUEUE_THIRD, third);
        e.mem.set_u32(QUEUE_FOURTH, fourth);
    }

    #[test]
    fn queue_menu_create_remembers_the_request() {
        let (mut e, _) = queue_engine();
        stub(&mut e, QUEUE_MENU_BLOCKED, 1);
        e.call(
            0x0070_9470,
            &args![3u32, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        assert_eq!(e.mem.u32(QUEUE_KIND), 3);
        assert_eq!(e.mem.u32(QUEUE_FIRST), 0x11);
        assert_eq!(e.mem.u32(QUEUE_SECOND), 0x22);
        assert_eq!(e.mem.u32(QUEUE_THIRD), 0x33);
        assert_eq!(e.mem.u32(QUEUE_FOURTH), 0x44);
        // Kind 7 goes to the message slot and leaves the first alone.
        e.call(
            0x0070_9470,
            &args![7u32, 0xa1u32, 0xa2u32, 0xa3u32, 0xa4u32],
        );
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_KIND), 7);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_FIRST), 0xa1);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_SECOND), 0xa2);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_THIRD), 0xa3);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_FOURTH), 0xa4);
        assert_eq!(e.mem.u32(QUEUE_KIND), 3);
        // Blocked: nothing was opened.
        assert!(logged(&e, QUEUE_LOCK_ENTER).is_empty());
    }

    #[test]
    fn queue_menu_create_opens_the_menu_at_once_when_nothing_blocks() {
        let (mut e, _) = queue_engine();
        stub(&mut e, QUEUE_MENU_BLOCKED, 0);
        e.call(0x0070_9470, &args![2u32, 0x33u32, 0u32, 0u32, 0u32]);
        assert_eq!(logged(&e, HACKING_MENU_CREATE), vec![vec![0x33]]);
        assert_eq!(logged(&e, QUEUE_LOCK_ENTER), vec![vec![QUEUE_LOCK, 0]]);
        assert_eq!(logged(&e, QUEUE_LOCK_LEAVE), vec![vec![QUEUE_LOCK]]);
        // Both slots are cleared afterwards, even a pending message.
        e.mem.set_u32(MESSAGE_QUEUE_KIND, 9);
        e.mem.set_u32(MESSAGE_QUEUE_FIRST, 9);
        e.mem.set_u32(QUEUE_KIND, 9);
        e.mem.set_u32(QUEUE_FIRST, 9);
        e.call(0x0070_9470, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(QUEUE_KIND), 0);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_KIND), 0);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_FIRST), 0);
    }

    #[test]
    fn handle_queued_menu_open_creates_the_menu_of_each_kind() {
        // (kind, creator, the words the creator gets)
        let cases: [(u32, u32, Vec<u32>); 7] = [
            (1, CREATE_CONTAINER_MENU, vec![0x11, 0x44]),
            (2, HACKING_MENU_CREATE, vec![0x11]),
            (3, COMPUTERS_MENU_CREATE, vec![0x11]),
            (4, CREATE_DIALOG_MENU, vec![0x11, 0x22, 0x33]),
            (5, RACE_SEX_MENU_CREATE, vec![0x55]),
            (7, MESSAGE_MENU_CREATE, vec![]),
            (6, START_MENU_CREATE, vec![1, 0]),
        ];
        for (kind, creator, words) in cases {
            let (mut e, _) = queue_engine();
            queue_request(&mut e, kind, 0x11, 0x22, 0x33, 0x44);
            e.mem.set_u32(QUEUE_RACE_SEX, 0x55);
            e.call(0x0070_94f0, &[]);
            assert_eq!(logged(&e, creator), vec![words], "kind {kind}");
            assert_eq!(e.mem.u32(QUEUE_KIND), 0, "kind {kind}");
            assert_eq!(e.mem.u32(QUEUE_FIRST), 0, "kind {kind}");
            // The lock is held around the whole.
            assert_eq!(logged(&e, QUEUE_LOCK_ENTER), vec![vec![QUEUE_LOCK, 0]]);
            assert_eq!(logged(&e, QUEUE_LOCK_LEAVE), vec![vec![QUEUE_LOCK]]);
            // Only the start menu sets the HUD mode.
            assert_eq!(logged(&e, HUD_SET_MENU_MODE).len(), (kind == 6) as usize);
        }
    }

    #[test]
    fn handle_queued_menu_open_keeps_the_request_when_creation_fails() {
        let (mut e, _) = queue_engine();
        stub(&mut e, CREATE_CONTAINER_MENU, 0);
        queue_request(&mut e, 1, 0x11, 0, 0, 0x44);
        e.call(0x0070_94f0, &[]);
        assert_eq!(e.mem.u32(QUEUE_KIND), 1);
        assert_eq!(e.mem.u32(QUEUE_FIRST), 0x11);

        // A dialog is not created while 007023a0 says it is blocked.
        let (mut e, _) = queue_engine();
        stub(&mut e, DIALOG_BLOCKED, 1);
        queue_request(&mut e, 4, 0x11, 0x22, 0x33, 0);
        e.call(0x0070_94f0, &[]);
        assert!(logged(&e, CREATE_DIALOG_MENU).is_empty());
        assert_eq!(e.mem.u32(QUEUE_KIND), 4);

        // An unknown kind stays queued.
        let (mut e, _) = queue_engine();
        queue_request(&mut e, 12, 0x11, 0, 0, 0);
        e.call(0x0070_94f0, &[]);
        assert_eq!(e.mem.u32(QUEUE_KIND), 12);

        // A start menu that was not created does not change the HUD mode.
        let (mut e, _) = queue_engine();
        stub(&mut e, START_MENU_CREATE, 0);
        queue_request(&mut e, 6, 0, 0, 0, 0);
        e.call(0x0070_94f0, &[]);
        assert!(logged(&e, HUD_SET_MENU_MODE).is_empty());
        assert_eq!(e.mem.u32(QUEUE_KIND), 6);
    }

    #[test]
    fn handle_queued_menu_open_waits_while_the_mode_word_is_set() {
        let (mut e, manager) = queue_engine();
        e.mem.set_u32(manager + 0x4bc, 2);
        queue_request(&mut e, 1, 0x11, 0, 0, 0x44);
        e.mem.set_u32(MESSAGE_QUEUE_KIND, 7);
        e.call(0x0070_94f0, &[]);
        // Kind 1 waits, the message menu (kind 7) does not.
        assert!(logged(&e, CREATE_CONTAINER_MENU).is_empty());
        assert_eq!(e.mem.u32(QUEUE_KIND), 1);
        assert_eq!(logged(&e, MESSAGE_MENU_CREATE).len(), 1);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_KIND), 0);
        // The start menu (kind 6) does not wait either.
        queue_request(&mut e, 6, 0, 0, 0, 0);
        e.call(0x0070_94f0, &[]);
        assert_eq!(e.mem.u32(QUEUE_KIND), 0);
        // With nothing queued the creators are not asked.
        assert_eq!(logged(&e, QUEUE_LOCK_ENTER).len(), 2);
    }

    #[test]
    fn handle_queued_menu_open_message_slot_ignores_other_kinds() {
        let (mut e, _) = queue_engine();
        e.mem.set_u32(MESSAGE_QUEUE_KIND, 6);
        e.mem.set_u32(MESSAGE_QUEUE_FIRST, 0x99);
        e.call(0x0070_94f0, &[]);
        assert!(logged(&e, MESSAGE_MENU_CREATE).is_empty());
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_KIND), 6);
        // A message menu that is not created stays queued.
        stub(&mut e, MESSAGE_MENU_CREATE, 0);
        e.mem.set_u32(MESSAGE_QUEUE_KIND, 7);
        e.call(0x0070_94f0, &[]);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_KIND), 7);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_FIRST), 0x99);
    }

    #[test]
    fn handle_queued_menu_open_casts_the_companion_wheel_argument() {
        let (mut e, _) = queue_engine();
        queue_request(&mut e, 8, 0x1234, 0, 0, 0);
        e.call(0x0070_94f0, &[]);
        assert_eq!(
            logged(&e, DYNAMIC_CAST),
            vec![vec![
                0x1234,
                0,
                COMPANION_CAST_SOURCE,
                COMPANION_CAST_TARGET,
                0
            ]]
        );
        assert_eq!(logged(&e, COMPANION_WHEEL_MENU_CREATE), vec![vec![0x7c7c]]);
        assert_eq!(e.mem.u32(QUEUE_KIND), 0);
    }

    /// Doubles for the tile constructor chain.
    fn tile_chain_engine() -> Engine {
        let (mut e, _) = ui_engine();
        for callee in [
            TILE_MEMBER_AT_4,
            TILE_MEMBER_AT_10,
            STRING_CONSTRUCT,
            TILE_MEMBER_AT_2C_FIRST,
            TILE_MEMBER_AT_2C_SECOND,
        ] {
            stub(&mut e, callee, 0);
        }
        e
    }

    #[test]
    fn handle_queued_menu_open_builds_the_player_name_entry_menu() {
        let (mut e, manager) = queue_engine();
        for callee in [
            TILE_MEMBER_AT_4,
            TILE_MEMBER_AT_10,
            STRING_CONSTRUCT,
            TILE_MEMBER_AT_2C_FIRST,
            TILE_MEMBER_AT_2C_SECOND,
            TILE_SET_PARENT,
            STRING_ASSIGN,
            MENU_SET_MENU_TILE,
            NAME_ENTRY_MENU_FINISH,
        ] {
            stub(&mut e, callee, 0);
        }
        let tile_block = e.mem.alloc(0x40);
        let menu_block = e.mem.alloc(0x2c);
        let blocks = Rc::new(RefCell::new(vec![menu_block, tile_block]));
        let source = blocks.clone();
        e.register_double(OPERATOR_NEW, move |_, _| {
            ret(source.borrow_mut().pop().unwrap())
        });
        stub(&mut e, NAME_ENTRY_MENU_CONSTRUCT, 0x0abc);
        stub(&mut e, GET_MENUS_ROOT, 0x7007);
        queue_request(&mut e, 9, 0, 0, 0, 0);
        e.call(0x0070_94f0, &[]);
        assert_eq!(logged(&e, OPERATOR_NEW), vec![vec![0x40], vec![0x2c]]);
        // The tile is a TileMenu now; the menu was built in its block.
        assert_eq!(e.mem.u32(tile_block), TILE_MENU_VTABLE);
        assert_eq!(
            logged(&e, NAME_ENTRY_MENU_CONSTRUCT),
            vec![vec![menu_block]]
        );
        assert_eq!(logged(&e, GET_MENUS_ROOT), vec![vec![manager]]);
        assert_eq!(
            logged(&e, TILE_SET_PARENT),
            vec![vec![tile_block, 0x7007, 0]]
        );
        assert_eq!(
            logged(&e, STRING_ASSIGN),
            vec![vec![tile_block + 0x20, NAME_ENTRY_MENU_NAME]]
        );
        assert_eq!(
            logged(&e, MENU_SET_MENU_TILE),
            vec![vec![0x0abc, tile_block, 1]]
        );
        assert_eq!(
            logged(&e, NAME_ENTRY_MENU_FINISH),
            vec![vec![tile_block, 0x0abc]]
        );
        assert_eq!(e.mem.u32(QUEUE_KIND), 0);
    }

    #[test]
    fn handle_queued_menu_open_passes_null_blocks_on() {
        // A failed allocation gives null objects but the menu is still
        // reported created.
        let (mut e, _) = queue_engine();
        for callee in [
            TILE_SET_PARENT,
            STRING_ASSIGN,
            MENU_SET_MENU_TILE,
            NAME_ENTRY_MENU_FINISH,
        ] {
            stub(&mut e, callee, 0);
        }
        stub(&mut e, OPERATOR_NEW, 0);
        stub(&mut e, GET_MENUS_ROOT, 0x7007);
        queue_request(&mut e, 9, 0, 0, 0, 0);
        e.call(0x0070_94f0, &[]);
        assert!(logged(&e, NAME_ENTRY_MENU_CONSTRUCT).is_empty());
        assert_eq!(logged(&e, TILE_SET_PARENT), vec![vec![0, 0x7007, 0]]);
        assert_eq!(logged(&e, MENU_SET_MENU_TILE), vec![vec![0, 0, 1]]);
        assert_eq!(e.mem.u32(QUEUE_KIND), 0);
    }

    #[test]
    fn fn_00709810_builds_a_tile_menu() {
        let mut e = tile_chain_engine();
        let tile = e.mem.alloc(0x40);
        e.mem.set_u32(tile + 0x3c, 0xdead);
        assert_eq!(e.call(0x0070_9810, &args![tile, 0u32, 0u32]).u32(), tile);
        assert_eq!(e.mem.u32(tile), TILE_MENU_VTABLE);
        assert_eq!(e.mem.u32(tile + 0x3c), 0);
        // The Tile constructor ran (its member calls).
        assert_eq!(logged(&e, TILE_MEMBER_AT_4), vec![vec![tile + 4]]);
    }

    #[test]
    fn fn_00709840_builds_a_tile_rect() {
        let mut e = tile_chain_engine();
        let tile = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0070_9840, &args![tile]).u32(), tile);
        assert_eq!(e.mem.u32(tile), TILE_RECT_VTABLE);
        assert_eq!(logged(&e, TILE_MEMBER_AT_4), vec![vec![tile + 4]]);
    }

    #[test]
    fn fn_00709860_builds_a_tile() {
        let mut e = tile_chain_engine();
        let tile = e.mem.alloc(0x40);
        for offset in [0x28, 0x30] {
            e.mem.set_u32(tile + offset, 0xdead);
        }
        e.mem.set_u8(tile + 0x34, 0xff);
        assert_eq!(e.call(0x0070_9860, &args![tile]).u32(), tile);
        assert_eq!(e.mem.u32(tile), TILE_VTABLE);
        assert_eq!(e.mem.u32(tile + 0x28), 0);
        assert_eq!(e.mem.u32(tile + 0x30), 0);
        assert_eq!(e.mem.u8(tile + 0x34), 0);
        assert_eq!(
            order(&e),
            vec![
                0x0070_9860,
                TILE_MEMBER_AT_4,
                TILE_MEMBER_AT_10,
                STRING_CONSTRUCT,
                TILE_MEMBER_AT_2C_FIRST,
                TILE_MEMBER_AT_2C_SECOND,
            ]
        );
        assert_eq!(
            logged(&e, TILE_MEMBER_AT_10),
            vec![vec![tile + 0x10, 0x10, 0]]
        );
        assert_eq!(logged(&e, STRING_CONSTRUCT), vec![vec![tile + 0x20]]);
        assert_eq!(
            logged(&e, TILE_MEMBER_AT_2C_FIRST),
            vec![vec![tile + 0x2c, 0]]
        );
        assert_eq!(
            logged(&e, TILE_MEMBER_AT_2C_SECOND),
            vec![vec![tile + 0x2c, 0]]
        );
    }

    #[test]
    fn tile_get_type_name_is_the_tile_string() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_9920, &[]).u32(), 0x0106_edc4);
    }

    #[test]
    fn tile_scalar_deleting_destructor_deletes_on_request() {
        let (mut e, _) = ui_engine();
        stub(&mut e, TILE_DESTRUCTOR, 0);
        stub(&mut e, OPERATOR_DELETE, 0);
        assert_eq!(e.call(0x0070_9930, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0070_9930, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![0x5000]]);
        // Other flag bits do not delete.
        assert_eq!(e.call(0x0070_9930, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert_eq!(logged(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(logged(&e, TILE_DESTRUCTOR).len(), 3);
    }

    #[test]
    fn tile_rect_get_type_is_0x385() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_9960, &[]).u32(), 0x385);
    }

    #[test]
    fn tile_rect_get_type_name_is_the_rect_string() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_9970, &[]).u32(), 0x0106_edcc);
    }

    #[test]
    fn tile_rect_scalar_deleting_destructor_runs_the_destructor_then_deletes() {
        let (mut e, _) = ui_engine();
        let tile = e.mem.alloc(0x40);
        stub(&mut e, TILE_RELEASE, 0);
        stub(&mut e, TILE_DESTRUCTOR, 0);
        stub(&mut e, OPERATOR_DELETE, 0);
        assert_eq!(e.call(0x0070_9980, &args![tile, 1u32]).u32(), tile);
        assert_eq!(
            order(&e),
            vec![0x0070_9980, TILE_RELEASE, TILE_DESTRUCTOR, OPERATOR_DELETE]
        );
        e.call_log = Some(vec![]);
        e.call(0x0070_9980, &args![tile, 0u32]);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn fn_007099b0_releases_unless_the_flag_is_set() {
        let (mut e, _) = ui_engine();
        let tile = e.mem.alloc(0x40);
        stub(&mut e, TILE_RELEASE, 0);
        stub(&mut e, TILE_DESTRUCTOR, 0);
        e.mem.set_u32(tile, 0x1111);
        e.call(0x0070_99b0, &args![tile]);
        assert_eq!(e.mem.u32(tile), TILE_RECT_VTABLE);
        assert_eq!(order(&e), vec![0x0070_99b0, TILE_RELEASE, TILE_DESTRUCTOR]);
        e.mem.set_u32(tile + 0x30, 0x2000);
        e.call_log = Some(vec![]);
        e.call(0x0070_99b0, &args![tile]);
        assert_eq!(order(&e), vec![0x0070_99b0, TILE_DESTRUCTOR]);
    }

    #[test]
    fn fn_00709a20_tests_the_0x2000_flag() {
        let (mut e, _) = ui_engine();
        let tile = e.mem.alloc(0x40);
        assert!(!e.call(0x0070_9a20, &args![tile]).bool());
        e.mem.set_u32(tile + 0x30, 0x2000);
        assert!(e.call(0x0070_9a20, &args![tile]).bool());
        e.mem.set_u32(tile + 0x30, !0x2000u32);
        assert!(!e.call(0x0070_9a20, &args![tile]).bool());
    }

    #[test]
    fn tile_menu_get_type_is_0x389() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_9a40, &[]).u32(), 0x389);
    }

    #[test]
    fn tile_menu_get_type_name_is_the_menu_string() {
        let (mut e, _) = ui_engine();
        assert_eq!(e.call(0x0070_9a50, &[]).u32(), 0x0106_edd4);
    }

    #[test]
    fn tile_menu_scalar_deleting_destructor_deletes_on_request() {
        let (mut e, _) = ui_engine();
        stub(&mut e, TILE_MENU_DESTRUCTOR, 0);
        stub(&mut e, OPERATOR_DELETE, 0);
        assert_eq!(e.call(0x0070_9a60, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0070_9a60, &args![0x5000u32, 3u32]).u32(), 0x5000);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![0x5000]]);
        assert_eq!(logged(&e, TILE_MENU_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn fn_00709a90_clears_both_queued_menus() {
        let (mut e, _) = ui_engine();
        for at in [
            QUEUE_KIND,
            QUEUE_FIRST,
            QUEUE_SECOND,
            MESSAGE_QUEUE_KIND,
            MESSAGE_QUEUE_FIRST,
            MESSAGE_QUEUE_SECOND,
        ] {
            e.mem.set_u32(at, 0x77);
        }
        e.call(0x0070_9a90, &[]);
        assert_eq!(e.mem.u32(QUEUE_KIND), 0);
        assert_eq!(e.mem.u32(QUEUE_FIRST), 0);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_KIND), 0);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_FIRST), 0);
        // The other words of the slots stay.
        assert_eq!(e.mem.u32(QUEUE_SECOND), 0x77);
        assert_eq!(e.mem.u32(MESSAGE_QUEUE_SECOND), 0x77);
    }

    #[test]
    fn fn_00709ac0_returns_the_float_constant() {
        let (mut e, _) = ui_engine();
        e.map(0x0101_6000, 0x1000);
        e.set_global(RETURNED_SCALE_01016264, 0.75f32);
        assert_eq!(e.call(0x0070_9ac0, &[]).f32(), 0.75);
    }

    /// A static wrapper with no arguments that calls `callee` once.
    fn check_static_forward(wrapper: u32, callee: u32) {
        let (mut e, _) = ui_engine();
        stub(&mut e, callee, 0);
        e.call(wrapper, &[]);
        assert_eq!(logged(&e, callee), vec![Vec::<u32>::new()]);
    }

    #[test]
    fn fn_00709ae0_calls_the_surgery_menu_function() {
        check_static_forward(0x0070_9ae0, SURGERY_MENU_FN);
    }

    #[test]
    fn fn_00709af0_calls_the_lockpick_menu_function() {
        check_static_forward(0x0070_9af0, LOCKPICK_MENU_FN);
    }

    #[test]
    fn fn_00709b00_calls_the_slot_machine_menu_function() {
        check_static_forward(0x0070_9b00, SLOT_MACHINE_MENU_FN);
    }

    #[test]
    fn fn_00709b10_calls_the_blackjack_menu_function() {
        check_static_forward(0x0070_9b10, BLACKJACK_MENU_FN);
    }

    #[test]
    fn fn_00709b20_calls_the_roulette_menu_function() {
        check_static_forward(0x0070_9b20, ROULETTE_MENU_FN);
    }

    #[test]
    fn fn_00709b30_calls_the_caravan_menu_function() {
        check_static_forward(0x0070_9b30, CARAVAN_MENU_FN);
    }

    #[test]
    fn fn_00709b40_calls_the_hud_main_menu_function() {
        check_static_forward(0x0070_9b40, HUD_MAIN_MENU_FN);
    }

    #[test]
    fn sound_and_menu_wrappers_answer_their_callees() {
        let (mut e, _) = ui_engine();
        stub(&mut e, ANIM_GROUP_PLAY_SOUNDS, 1);
        assert!(e.call(0x0070_7970, &args![0x4411u32]).bool());
        stub(&mut e, MENU_MANAGER_INSTANCE, 0x7000);
        stub(&mut e, MENU_MANAGER_CREATE_MENU_BY_CLASS, 0x7400);
        assert_eq!(e.call(0x0070_7990, &args![0x3f1u32]).u32(), 0x7400);
    }
}
