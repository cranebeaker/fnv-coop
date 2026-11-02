//! `fallout/interface/interfacemanager.cpp` (Xbox PDB source unit), part 2: its functions from `00717230` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::interfacemanager`]; anything public there may be used here.
//!
//! This part covers `00717230`..`00719bb0` (the whole of its range): the system
//! colors (`SystemColorManager::SystemColor` and its hard and soft kinds), the
//! tile refresh and the fade update, and earlier: the menu sound and button
//! helpers of the `InterfaceManager`, its tutorial message table
//! (`InterfaceManager::TutorialManager`) and the `SystemColorManager`
//! constructor and registration helpers.
//!
//! The compiler's exception-unwinding frames (`PlayMenuSound`,
//! `SystemColorManager::GetInstance`, the `SystemColorManager` constructor
//! and its registration helpers) are not translated.
//!
//! The manager pointer is the word at `011d8a80`; `004b7210` returns it. The
//! offsets of its fields used here are in [`InterfaceManagerFields`].

#[allow(unused_imports)]
use super::interfacemanager::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `004b7210`: the interface manager object (the word at `011d8a80`).
const GET_MANAGER: u32 = 0x004b_7210;
/// The manager pointer itself.
const MANAGER_GLOBAL: u32 = 0x011d_8a80;
/// `004b71d0`: true when the manager exists and its helper object reports
/// ready (`bool`).
const MANAGER_READY: u32 = 0x004b_71d0;
/// `004a4040`: `bool` (tests a global object with 1).
const IS_IN_SPECIAL_MODE: u32 = 0x004a_4040;
/// `0070ec00`: true when the word at `011d8950` is not zero.
const IS_FLAG_011D8950_SET: u32 = 0x0070_ec00;
/// `005d4a40`: returns true.
const ALWAYS_TRUE: u32 = 0x005d_4a40;
/// Tile trait setter (`00700320`): tile, trait id, value.
const TILE_SET_VALUE: u32 = 0x0070_0320;
/// Tile trait getter (`00a011b0`, value in ST0): tile, trait id.
const TILE_GET_VALUE: u32 = 0x00a0_11b0;
/// Trait id the pre-load and the rendered menu entry set to 0 (`0xfa3`).
const TRAIT_ENABLED: u32 = 0xfa3;
/// `00702680`, `cdecl`: menu id, second flag; returns `bool` (the menu is
/// visible).
const IS_MENU_ID_VISIBLE: u32 = 0x0070_2680;
/// `00702450`, `cdecl`: true when the menu id is the top one.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `007024e0`: true when the top menu is faded in.
const IS_TOP_MENU_FADED_IN: u32 = 0x0070_24e0;
/// `007023c0`: the id of the menu on top.
const GET_TOP_MENU_ID: u32 = 0x0070_23c0;
/// `InterfaceManager::GetEnterStackTop` (Xbox PDB), on the manager.
const GET_ENTER_STACK_TOP: u32 = 0x0071_4f00;
/// `0071b160`, `cdecl`: an object (called with 1) whose signed byte at
/// `+0x38` `004a4020` tests.
const GET_STATE_OBJECT: u32 = 0x0071_b160;
/// `004a4020`: true when the signed byte at `+0x38` of the object is
/// positive.
const STATE_OBJECT_FLAG: u32 = 0x004a_4020;
/// `Tile::GetMenuByClass` (Xbox PDB), `cdecl`: the menu with the given id.
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `0056c7f0`: the node a tile keeps.
const TILE_NODE: u32 = 0x0056_c7f0;
/// `0063d040`: the node of the rendered menu the manager shows.
const RENDERED_MENU_NODE: u32 = 0x0063_d040;
/// List helpers over the handler list at [`HANDLER_LIST`]: `006815c0`
/// returns its argument (a node's first word is the item) and `00726070` the
/// next node (the word at `+4`).
const LIST_ITEM_REF: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
/// The head node of the global list of input handlers (a static object).
const HANDLER_LIST: u32 = 0x011d_8b54;
/// `operator new` / `operator delete`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `004839c0`, `cdecl`: the form with the given form id (0 when unknown).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`): pointer, 0, source type
/// descriptor, target type descriptor, 0.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors the tutorial message casts use.
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_6130;
/// `00457fe0`: the current time in milliseconds (`u32`).
const GET_TIME: u32 = 0x0045_7fe0;
/// `_ftol2_sse` (`00ec62c0`) with its `ST0` argument passed as an `f64`.
const FTOL: u32 = 0x00ec_62c0;

/// The pointer stored at `011dea3c` (its `+0x66d` byte has a flag bit 4;
/// `0095f530` and `00967ae0` work on it).
const OBJECT_011DEA3C: u32 = 0x011d_ea3c;
/// The pointer stored at `011dea0c` (`00877720` is called on it).
const OBJECT_011DEA0C: u32 = 0x011d_ea0c;
/// The integer at `011a0190` that `00717890` returns.
const INTEGER_011A0190: u32 = 0x011a_0190;
/// Gamepad state `CheckMenuButton` reads: signed 16-bit stick axes, 16-bit
/// button masks (the old state at `58`, the new at `70`) and trigger bytes.
const PAD_BUTTONS_OLD: u32 = 0x011d_8a58;
const PAD_BUTTONS_NEW: u32 = 0x011d_8a70;
const PAD_AXIS_OLD_X: u32 = 0x011d_8a5c;
const PAD_AXIS_OLD_Y: u32 = 0x011d_8a5e;
const PAD_AXIS_NEW_X: u32 = 0x011d_8a74;
const PAD_AXIS_NEW_Y: u32 = 0x011d_8a76;
const PAD_TRIGGER_OLD_LEFT: u32 = 0x011d_8a5a;
const PAD_TRIGGER_OLD_RIGHT: u32 = 0x011d_8a5b;
const PAD_TRIGGER_NEW_LEFT: u32 = 0x011d_8a72;
const PAD_TRIGGER_NEW_RIGHT: u32 = 0x011d_8a73;
/// Dead zone of the stick axes (`0x1ea9`).
const STICK_THRESHOLD: i32 = 0x1ea9;

/// Number of entries of the tutorial message table (`0x29`).
const TUTORIAL_MESSAGE_COUNT: i32 = 0x29;
/// The form id of tutorial message `n` is this plus `n` (`0x168`).
const TUTORIAL_FIRST_FORM_ID: u32 = 0x168;
/// First and last menu id of the menu id range (`0x3e9`..`0x43c`).
const FIRST_MENU_ID: u32 = 0x3e9;
const LAST_MENU_ID: u32 = 0x43c;

/// The `SystemColorManager` singleton pointer.
const SYSTEM_COLOR_MANAGER_INSTANCE: u32 = 0x011d_8a88;

layout! {
    /// `InterfaceManager` (Xbox PDB): the fields this part uses, at their PC
    /// offsets. The Xbox build is 0x478 bytes.
    pub struct InterfaceManagerFields: 0x478 {
        /// `cMenuMode` (Xbox PDB).
        0x0C cMenuMode: u32,
        /// The float `DoWheelMove` divides (`fMouseWheel` in the Xbox PDB,
        /// which has it at +0x44; the PC code reads +0x40).
        0x40 fMouseWheel: f32,
        /// `bShowMouse` (Xbox PDB name at the same offset): the menu button
        /// helpers do nothing while it is set.
        0x7D bShowMouse: bool,
        /// `bPreLoadMainMenus` (Xbox PDB).
        0xC8 bPreLoadMainMenus: bool,
        /// `bFirstChanceLoad` (Xbox PDB).
        0xC9 bFirstChanceLoad: bool,
        /// `bIsInRenderedMenu` (Xbox PDB).
        0x168 bIsInRenderedMenu: bool,
        /// `pCurrentRenderedMenu` (Xbox PDB): `FORenderedMenu*`.
        0x16C pCurrentRenderedMenu: Ptr,
    }

    /// `InterfaceManager::TutorialManager` (Xbox PDB), 0xAC bytes: 0x29
    /// message words, then the two fields below. A message word holds bit 0
    /// and bit 1 flags, in bits 2..7 a menu id relative to `0x3e9` (0 =
    /// none) and in bits 8..31 a delay.
    pub struct TutorialManager: 0xAC {
        /// `eDisplayID` (Xbox PDB): index of the message being shown, `0x29`
        /// for none.
        0xA4 eDisplayID: i32,
        /// `uiNextDisplay` (Xbox PDB): time at which the next message is
        /// due, 0 for none.
        0xA8 uiNextDisplay: u32,
    }

    /// `SystemColorManager` (Xbox PDB), 0x10 bytes: an `NiTPointerList` of
    /// `SystemColor` pointers, then the next id.
    pub struct SystemColorManager: 0x10 {
        /// `iNextID` (Xbox PDB).
        0x0C iNextID: i32,
    }
}

/// The value a callee leaves in `ST0`.
fn st0(r: Ret) -> f64 {
    r.f64()
}

/// The address of the item word of list node `node` (`006815c0`).
fn list_item_word(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_ITEM_REF, &args![node]).u32()
}

/// The next node of the handler list (`00726070`).
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NEXT, &args![node]).u32()
}

/// `_ftol2_sse` on a `float`.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

/// Word `index` of the tutorial message table.
fn message_word(e: &Engine, this: Ptr<TutorialManager>, index: i32) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add((index as u32).wrapping_mul(4)))
}

fn set_message_word(e: &mut Engine, this: Ptr<TutorialManager>, index: i32, value: u32) {
    e.mem.set_u32(
        this.addr().wrapping_add((index as u32).wrapping_mul(4)),
        value,
    );
}

/// The menu id offset (bits 2..7) of a message word.
fn message_menu_field(word: u32) -> u32 {
    (word >> 2) & 0x3f
}

/// The state of a button from whether it is down now and was before: 0 for
/// both, 2 for newly down, 1 for released, `-1` for up in both.
fn button_state(down_now: bool, down_before: bool) -> i32 {
    match (down_now, down_before) {
        (true, true) => 0,
        (true, false) => 2,
        (false, true) => 1,
        (false, false) => -1,
    }
}

// Translated from 00717230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the string `text` is at most as wide as the width kept at
/// `+0x14` of `this`, measured with the font id at `+0x18` of `this`
/// (`FontManager::CalculateStringDimensions` with an unlimited width, then
/// the integer part of the width).
pub fn fn_00717230(e: &mut Engine, this: Ptr, text: u32) -> bool {
    let font = e.mem.u32(this.addr() + 0x18);
    let unlimited: f32 = e.global(0x0101_6970);
    let font_manager = e.call(0x005b_d5b0, &args![]).u32();
    let width = e.with_stack(12, |e, size| {
        e.call(
            0x00a1_b020,
            &args![font_manager, size, text, font, unlimited, 0u32],
        );
        e.mem.f32(size.addr())
    });
    let width = float_to_int(e, width);
    width <= e.mem.i32(this.addr() + 0x14)
}

// Translated from 00717280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::PlayMenuSound` (Xbox PDB), static: plays the menu
/// sound for `sound_id` (ids 1, 2, 3, 4, 8, 10, 19, 20, 21 and 36 have a
/// sound, the others and `-1` play nothing). The sound is looked up by name
/// in the audio system (`BSAudio::GetSoundHandleByName`, flags `0x121`) and
/// played through a `BSSoundHandle`.
pub fn interface_manager_play_menu_sound(e: &mut Engine, sound_id: i32) {
    // Names, read from the exe: "UIMenuOK", "UIMenuCancel",
    // "UIMenuPrevNext", "UIMenuFocus", "UIPopUpQuestNew",
    // "UIPopUpMessageGeneral", "UILevelUp", "UIMenuMode".
    let name = match sound_id {
        1 => Some(0x0106_f380),
        2 | 20 => Some(0x0106_f360),
        3 => Some(0x0106_f38c),
        4 => Some(0x0106_f39c),
        8 => Some(0x0106_f370),
        10 | 19 => Some(0x0102_5cc4),
        21 => Some(0x0106_f354),
        36 => Some(0x0106_f060),
        _ => None,
    };
    e.with_stack(12, |e, handle| {
        // BSSoundHandle constructor (id -1, not playing).
        e.call(0x0041_a250, &args![handle]);
        if let Some(name) = name {
            e.with_stack(12, |e, found| {
                let audio = e.call(0x0045_3a70, &args![]).u32();
                let result = e.call(0x00ad_7550, &args![audio, found, name, 0x121u32]);
                // BSSoundHandle copy (id, flag byte, third word).
                e.call(0x0041_8900, &args![handle, result.u32()]);
                e.call(0x0048_3710, &args![found]);
            });
            // BSSoundHandle::Play (Xbox PDB).
            e.call(0x00ad_8830, &args![handle, 0u32]);
        }
        e.call(0x0048_3710, &args![handle]);
    });
}

// Translated from 00717600 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object at `011dea3c` exists, the manager's first-chance load
/// flag is set and `004516b0` (on the object at `011c3f2c`) is false:
/// pre-loads the main menus without changing `cMenuMode`, then clears the
/// flag.
pub fn fn_00717600(e: &mut Engine, this: Ptr<InterfaceManagerFields>) {
    if e.global::<u32>(OBJECT_011DEA3C) != 0
        && e.get(this, InterfaceManagerFields::bFirstChanceLoad)
    {
        let object = e.global::<u32>(0x011c_3f2c);
        if !e.call(0x0045_16b0, &args![object]).bool() {
            let menu_mode = e.get(this, InterfaceManagerFields::cMenuMode);
            interface_manager_pre_load_main_menus(e, this);
            e.set(this, InterfaceManagerFields::cMenuMode, menu_mode);
            e.set(this, InterfaceManagerFields::bFirstChanceLoad, false);
        }
    }
}

// Translated from 00717660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::PreLoadMainMenus` (Xbox PDB): when `bPreLoadMainMenus`
/// is set, creates each of five menus (`00704be0`, `007048c0`, `00704490`,
/// `007046c0`, `00704140`), sets trait `0xfa3` of each created tile to 0,
/// runs `00705b70(0, 0, 4, 0x3eb)` and `HideMenus` (`00703610`), and clears
/// the flag.
pub fn interface_manager_pre_load_main_menus(e: &mut Engine, this: Ptr<InterfaceManagerFields>) {
    if !e.get(this, InterfaceManagerFields::bPreLoadMainMenus) {
        return;
    }
    for create in [
        0x0070_4be0,
        0x0070_48c0,
        0x0070_4490,
        0x0070_46c0,
        0x0070_4140,
    ] {
        let menu = e.call(create, &args![]).u32();
        if menu != 0 {
            e.call(TILE_SET_VALUE, &args![menu, TRAIT_ENABLED, 0u32]);
        }
    }
    e.call(0x0070_5b70, &args![0u32, 0u32, 4u32, 0x3ebu32]);
    e.call(0x0070_3610, &args![]);
    e.set(this, InterfaceManagerFields::bPreLoadMainMenus, false);
}

// Translated from 00717740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::SetCursorAlpha` (Xbox PDB), static: sets the cursor's
/// alpha. Takes the node of the object `0045cd60` returns for the manager
/// (when there is one), its child `0045bc00(node, 0)`, that child's
/// property `00a59d30(child, 004af350())`, and calls `0068c9f0` on it with
/// `alpha` as a `float`.
pub fn interface_manager_set_cursor_alpha(e: &mut Engine, alpha: i32) {
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let object = e.call(0x0045_cd60, &args![manager]).u32();
    let node = if object != 0 {
        e.call(TILE_NODE, &args![object]).u32()
    } else {
        0
    };
    if node == 0 {
        return;
    }
    let child = e.call(0x0045_bc00, &args![node, 0u32]).u32();
    if child == 0 {
        return;
    }
    let key = e.call(0x004a_f350, &args![]).u32();
    let property = e.call(0x00a5_9d30, &args![child, key]).u32();
    if property != 0 {
        e.call(0x0068_c9f0, &args![property, alpha as f32]);
    }
}

// Translated from 007177c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A screen measure as a `float` in `ST0`: when `004de080` is true,
/// `004de0e0()` times the double at `010290c0` (rounded to `float`);
/// otherwise the integer setting at `011d8bfc` (when `00707b40` is true) or
/// `011d8bd8`, exactly. Returned as an `f64` so the integer stays exact.
pub fn fn_007177c0(e: &mut Engine) -> f64 {
    if e.call(0x004d_e080, &args![]).bool() {
        let value = st0(e.call(0x004d_e0e0, &args![]));
        let scale: f64 = e.global(0x0102_90c0);
        return (value * scale) as f32 as f64;
    }
    let setting = if e.call(0x0070_7b40, &args![]).bool() {
        0x011d_8bfc
    } else {
        0x011d_8bd8
    };
    let value = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.i32(value) as f64
}

// Translated from 00717820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The integer setting at `011d8bc0` (when `00707b40` is true) or
/// `011d8bf0`, as a `float` in `ST0` (exact, returned as an `f64`).
pub fn fn_00717820(e: &mut Engine) -> f64 {
    let setting = if e.call(0x0070_7b40, &args![]).bool() {
        0x011d_8bc0
    } else {
        0x011d_8bf0
    };
    let value = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.i32(value) as f64
}

// Translated from 00717860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::GetMenuHeight` (Xbox PDB), static: the integer
/// `00717890()` divided by the `float` `00715da0()` returns, rounded to
/// `float`.
pub fn interface_manager_get_menu_height(e: &mut Engine) -> f32 {
    let height = fn_00717890(e);
    let divisor = st0(e.call(0x0071_5da0, &args![]));
    (height as f64 / divisor) as f32
}

// Translated from 00717890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The integer at `011a0190`.
pub fn fn_00717890(e: &mut Engine) -> i32 {
    e.global(INTEGER_011A0190)
}

// Translated from 007178a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::IsInPipboyMenu` (Xbox PDB), static: whether any of the
/// menus `0x3eb`, `0x3ea`, `0x40b`, `0x425` and `0x3ff` is visible
/// (`00702680(id, 0)`), tried in that order.
pub fn interface_manager_is_in_pipboy_menu(e: &mut Engine) -> bool {
    [0x3eb_i32, 0x3ea, 0x40b, 0x425, 0x3ff]
        .into_iter()
        .any(|id| e.call(IS_MENU_ID_VISIBLE, &args![id, 0u32]).bool())
}

// Translated from 00717920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::IsPipboyMenuTopmost` (Xbox PDB), static: false when
/// the object `0071b160(1)` has a positive byte at `+0x38`; otherwise
/// whether the top of the enter stack is one of `0x3eb`, `0x3ff`, `0x3ea`,
/// `0x40b`, `0x425` or `1`.
pub fn interface_manager_is_pipboy_menu_topmost(e: &mut Engine) -> bool {
    let state = e.call(GET_STATE_OBJECT, &args![1u32]).u32();
    if e.call(STATE_OBJECT_FLAG, &args![state]).bool() {
        return false;
    }
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let top = e.call(GET_ENTER_STACK_TOP, &args![manager]).i32();
    matches!(top, 0x3eb | 0x3ff | 0x3ea | 0x40b | 0x425 | 1)
}

// Translated from 00717990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::IsCurrentRenderedMenuTopmost` (Xbox PDB), static:
/// false without a current rendered menu (`+0x16c` of the manager). In a
/// Pipboy menu ([`interface_manager_is_in_pipboy_menu`]) it is
/// [`interface_manager_is_pipboy_menu_topmost`]. Otherwise false when the
/// `0071b160(1)` object has a positive flag byte, else whether the node of
/// the top menu (`Tile::GetMenuByClass` of the enter stack top) is the
/// node of the current rendered menu.
pub fn interface_manager_is_current_rendered_menu_topmost(e: &mut Engine) -> bool {
    let manager = Ptr::<InterfaceManagerFields>::new(e.global(MANAGER_GLOBAL));
    if e.get(manager, InterfaceManagerFields::pCurrentRenderedMenu)
        .is_null()
    {
        return false;
    }
    if interface_manager_is_in_pipboy_menu(e) {
        return interface_manager_is_pipboy_menu_topmost(e);
    }
    let state = e.call(GET_STATE_OBJECT, &args![1u32]).u32();
    if e.call(STATE_OBJECT_FLAG, &args![state]).bool() {
        return false;
    }
    let manager_now = e.call(GET_MANAGER, &args![]).u32();
    let top = e.call(GET_ENTER_STACK_TOP, &args![manager_now]).u32();
    let menu = e.call(GET_MENU_BY_CLASS, &args![top]).u32();
    let node = if menu != 0 {
        e.call(TILE_NODE, &args![menu]).u32()
    } else {
        0
    };
    if node == 0 {
        return false;
    }
    let manager = Ptr::<InterfaceManagerFields>::new(e.global(MANAGER_GLOBAL));
    let rendered = e.get(manager, InterfaceManagerFields::pCurrentRenderedMenu);
    let rendered_node = e.call(RENDERED_MENU_NODE, &args![rendered]).u32();
    node == rendered_node
}

// Translated from 00717a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::CheckMenuButton` (Xbox PDB), static: the state of
/// gamepad button `button` (1..18) from the gamepad words at
/// `011d8a58`..`011d8a76`: 0 when it is down in both the old and new state,
/// 2 when only newly down, 1 when only released, `-1` when up in both.
/// With `analog` set, the stick directions of buttons 1 to 4 are first
/// tested against the dead zone `0x1ea9` (a direction pushed in only one of
/// the two sticks decides; buttons 13 and 14 read the trigger bytes).
/// Returns 1 when the manager is not ready or `bShowMouse` is set, and `-1`
/// when that happens at the second check.
pub fn interface_manager_check_menu_button(e: &mut Engine, button: i32, analog: u8) -> i32 {
    e.call(GET_MANAGER, &args![]);
    if !e.call(MANAGER_READY, &args![]).bool() {
        return 1;
    }
    let manager = e.call(GET_MANAGER, &args![]).u32();
    if e.mem.u8(manager + 0x7d) != 0 {
        return 1;
    }
    e.call(GET_MANAGER, &args![]);
    if !e.call(MANAGER_READY, &args![]).bool() {
        return -1;
    }
    let manager = e.call(GET_MANAGER, &args![]).u32();
    if e.mem.u8(manager + 0x7d) != 0 {
        return -1;
    }

    let mask: u32 = match button {
        1 => 0x1,
        2 => 0x2,
        3 => 0x8,
        4 => 0x4,
        5 => 0x10,
        6 => 0x20,
        9 => 0x1000,
        10 => 0x2000,
        11 => 0x4000,
        12 => 0x8000,
        15 => 0x100,
        16 => 0x200,
        17 => 0x40,
        18 => 0x80,
        _ => 0,
    };

    if button == 13 || button == 14 {
        let (new_trigger, old_trigger) = if button == 13 {
            (PAD_TRIGGER_NEW_LEFT, PAD_TRIGGER_OLD_LEFT)
        } else {
            (PAD_TRIGGER_NEW_RIGHT, PAD_TRIGGER_OLD_RIGHT)
        };
        return button_state(e.mem.u8(new_trigger) != 0, e.mem.u8(old_trigger) != 0);
    }

    // (the old stick, the new stick) pushed in the direction of the button.
    let mut old_pushed = false;
    let mut new_pushed = false;
    if analog != 0 {
        let axis = |e: &Engine, address: u32| i32::from(e.mem.i16(address));
        match mask {
            1 => {
                old_pushed = axis(e, PAD_AXIS_OLD_Y) > STICK_THRESHOLD;
                new_pushed = axis(e, PAD_AXIS_NEW_Y) > STICK_THRESHOLD;
            }
            2 => {
                old_pushed = axis(e, PAD_AXIS_OLD_Y) < -STICK_THRESHOLD;
                new_pushed = axis(e, PAD_AXIS_NEW_Y) < -STICK_THRESHOLD;
            }
            4 => {
                old_pushed = axis(e, PAD_AXIS_OLD_X) < -STICK_THRESHOLD;
                new_pushed = axis(e, PAD_AXIS_NEW_X) < -STICK_THRESHOLD;
            }
            8 => {
                old_pushed = axis(e, PAD_AXIS_OLD_X) > STICK_THRESHOLD;
                new_pushed = axis(e, PAD_AXIS_NEW_X) > STICK_THRESHOLD;
            }
            _ => {}
        }
    }
    if old_pushed || new_pushed {
        // The code returns 0 for both, 2 for only the second, 1 for only
        // the first.
        return button_state(new_pushed, old_pushed);
    }

    let down_now = u32::from(e.mem.u16(PAD_BUTTONS_NEW)) & mask != 0;
    let down_before = u32::from(e.mem.u16(PAD_BUTTONS_OLD)) & mask != 0;
    button_state(down_now, down_before)
}

// Translated from 00717de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::ClearMenuButton` (Xbox PDB), static: when the manager
/// is ready and `bShowMouse` is clear, calls `00a25560` on the object
/// `00877720(*011dea0c)` returns with the value [`fn_007189c0`] maps
/// `button` to.
pub fn interface_manager_clear_menu_button(e: &mut Engine, button: i32) {
    if !e.call(MANAGER_READY, &args![]).bool() {
        return;
    }
    let manager = e.call(GET_MANAGER, &args![]).u32();
    if e.mem.u8(manager + 0x7d) != 0 {
        return;
    }
    let object = e.global::<u32>(OBJECT_011DEA0C);
    let target = e.call(0x0087_7720, &args![object]).u32();
    let mapped = fn_007189c0(e, button);
    e.call(0x00a2_5560, &args![target, mapped]);
}

// Translated from 00717e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::DoEnter` (Xbox PDB): walks the global handler list
/// (head node `011d8b54`) and stops at the first handler whose virtual
/// function at `+0x00` accepts `arg_2`; then forwards to `target`'s virtual
/// function at `+0x10` with `arg_1` and `arg_2`, returning its result.
pub fn interface_manager_do_enter(
    e: &mut Engine,
    _this: Ptr,
    target: u32,
    arg_1: u32,
    arg_2: u32,
) -> u32 {
    let mut node = HANDLER_LIST;
    while node != 0 {
        let item_word = list_item_word(e, node);
        if e.mem.u32(item_word) == 0 {
            break;
        }
        let item_word = list_item_word(e, node);
        let handler = e.mem.u32(item_word);
        if e.vcall(handler, 0x00, &args![arg_2]).bool() {
            break;
        }
        node = list_next(e, node);
    }
    e.vcall(target, 0x10, &args![arg_1, arg_2]).u32()
}

// Translated from 00717ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::DoLeave` (Xbox PDB): walks the global handler list and
/// stops at the first handler whose virtual function at `+0x04` returns
/// `arg_2` and whose function at `+0x00` (called with 0) is true; then
/// forwards to `target`'s virtual function at `+0x14` with `arg_1` and
/// `arg_2`, returning its result.
pub fn interface_manager_do_leave(
    e: &mut Engine,
    _this: Ptr,
    target: u32,
    arg_1: u32,
    arg_2: u32,
) -> u32 {
    let mut node = HANDLER_LIST;
    while node != 0 {
        let item_word = list_item_word(e, node);
        if e.mem.u32(item_word) == 0 {
            break;
        }
        let item_word = list_item_word(e, node);
        let handler = e.mem.u32(item_word);
        if e.vcall(handler, 0x04, &args![]).u32() == arg_2 {
            let item_word = list_item_word(e, node);
            let handler = e.mem.u32(item_word);
            if e.vcall(handler, 0x00, &args![0u32]).bool() {
                break;
            }
        }
        node = list_next(e, node);
    }
    e.vcall(target, 0x14, &args![arg_1, arg_2]).u32()
}

// Translated from 00717f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::DoGamepad` (Xbox PDB): offers the gamepad event
/// (`button`, `value`) to `target`'s virtual function at `+0x38`; when it
/// does not take it, looks through the global handler list for a handler
/// that accepts `target` (`+0x0c`), where `target`'s `+0x34` result is the
/// top menu id and the handler's `+0x04` is non-zero. That handler's `+0x08`
/// (with `button`) gives a tile; trait `0xfc3` of the tile is set to 1
/// through `00715860` and the handler's `+0x10` runs. Returns true when
/// either took the event.
pub fn interface_manager_do_gamepad(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    button: u32,
    value: f32,
) -> bool {
    if e.vcall(target, 0x38, &args![button, value]).bool() {
        return true;
    }
    let mut node = HANDLER_LIST;
    while node != 0 {
        let item_word = list_item_word(e, node);
        let handler = e.mem.u32(item_word);
        if handler == 0 {
            break;
        }
        if e.vcall(handler, 0x0c, &args![target]).bool() {
            let top = e.vcall(target, 0x34, &args![]).u32();
            if e.call(IS_TOP_MENU_ID, &args![top]).bool()
                && e.vcall(handler, 0x04, &args![]).u32() != 0
            {
                let tile = e.vcall(handler, 0x08, &args![button]).u32();
                if tile != 0 {
                    e.call(0x0071_5860, &args![this, tile, 0xfc3u32, 1u32]);
                    e.vcall(handler, 0x10, &args![]);
                    return true;
                }
            }
        }
        node = list_next(e, node);
    }
    false
}

// Translated from 00718080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::DoWheelMove` (Xbox PDB): converts the wheel movement
/// (`fMouseWheel` divided by `0070ecb0()`; in a Pipboy menu also divided by
/// the menu height and reduced by the double at `0106f3a8`) to a position.
/// For every handler of the global list whose `+0x04` is non-zero it looks
/// through the children (`+0x14` with an index) for the first whose start
/// `p` (`00a01440`) and width (trait `0xfb0`) satisfy
/// `p <= position <= p + width`, and gives it (or none) to the handler's
/// `+0x00`. Finally forwards to `target`'s `+0x28` with `arg_1` and `arg_2`,
/// returning its result.
pub fn interface_manager_do_wheel_move(
    e: &mut Engine,
    this: Ptr<InterfaceManagerFields>,
    target: u32,
    arg_1: u32,
    arg_2: u32,
) -> u32 {
    let divisor = st0(e.call(0x0070_ecb0, &args![]));
    let mut position = (e.get(this, InterfaceManagerFields::fMouseWheel) as f64 / divisor) as f32;
    if interface_manager_is_pipboy_menu_topmost(e) {
        let height = interface_manager_get_menu_height(e) as f64;
        let offset: f64 = e.global(0x0106_f3a8);
        position = (position as f64 / height - offset) as f32;
    }

    let mut node = HANDLER_LIST;
    while node != 0 {
        let item_word = list_item_word(e, node);
        let handler = e.mem.u32(item_word);
        if handler == 0 {
            break;
        }
        if e.vcall(handler, 0x04, &args![]).u32() != 0 {
            let mut index = 0u32;
            let child = loop {
                let item_word = list_item_word(e, node);
                let handler = e.mem.u32(item_word);
                let child = e.vcall(handler, 0x14, &args![index, 0u32]).u32();
                index += 1;
                if child == 0 {
                    break child;
                }
                let start = st0(e.call(0x00a0_1440, &args![child]));
                if (position as f64) < start {
                    continue;
                }
                let start = st0(e.call(0x00a0_1440, &args![child]));
                let width = st0(e.call(TILE_GET_VALUE, &args![child, 0xfb0u32]));
                if (position as f64) > width + start {
                    continue;
                }
                break child;
            };
            let item_word = list_item_word(e, node);
            let handler = e.mem.u32(item_word);
            e.vcall(handler, 0x00, &args![child]);
        }
        node = list_next(e, node);
    }
    e.vcall(target, 0x28, &args![arg_1, arg_2]).u32()
}

// Translated from 007181d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::EnterRenderedMenu` (Xbox PDB): with a menu, makes it
/// the current rendered menu (`00974d90` decides whether its virtual
/// function `+0x24` runs first, then `+0x30` runs, `bIsInRenderedMenu` is
/// set and trait `0xfa3` of its `009611e0` tile is set to 0); without one,
/// hides the current rendered menu (`007fae70(menu, 0)`, its `+0x34`, and
/// [`fn_007182b0`] unless it is the menu `00704370` names) and clears
/// `bIsInRenderedMenu`. Finally stores `menu` at `+0x16c`.
pub fn interface_manager_enter_rendered_menu(
    e: &mut Engine,
    this: Ptr<InterfaceManagerFields>,
    menu: u32,
) {
    if menu != 0 {
        if !e.call(0x0097_4d90, &args![menu]).bool() {
            e.vcall(menu, 0x24, &args![]);
        }
        e.vcall(menu, 0x30, &args![]);
        e.set(this, InterfaceManagerFields::bIsInRenderedMenu, true);
        if e.call(0x0096_11e0, &args![menu]).u32() != 0 {
            let tile = e.call(0x0096_11e0, &args![menu]).u32();
            e.call(TILE_SET_VALUE, &args![tile, TRAIT_ENABLED, 0u32]);
        }
    } else {
        let current = e.get(this, InterfaceManagerFields::pCurrentRenderedMenu);
        if !current.is_null() {
            e.call(0x007f_ae70, &args![current, 0u32]);
            let current = e.get(this, InterfaceManagerFields::pCurrentRenderedMenu);
            e.vcall(current.addr(), 0x34, &args![]);
            let named = e.call(0x0070_4370, &args![this]).u32();
            let current = e.get(this, InterfaceManagerFields::pCurrentRenderedMenu);
            if current.addr() != named {
                let current = e.get(this, InterfaceManagerFields::pCurrentRenderedMenu);
                fn_007182b0(e, current.addr());
            }
        }
        e.set(this, InterfaceManagerFields::bIsInRenderedMenu, false);
    }
    e.set(
        this,
        InterfaceManagerFields::pCurrentRenderedMenu,
        Ptr::new(menu),
    );
}

// Translated from 007182b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `item` to the list at `011d8b2c` (`004ed8c0`, which takes the
/// address of the item) between `004538a0(0)` and `004538c0` on the object
/// at `011d8c18` (enter and leave of a lock).
pub fn fn_007182b0(e: &mut Engine, item: u32) {
    e.call(0x0045_38a0, &args![0x011d_8c18u32, 0u32]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(0x004e_d8c0, &args![0x011d_8b2cu32, slot]);
    });
    e.call(0x0045_38c0, &args![0x011d_8c18u32]);
}

// Translated from 007182e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::TutorialManager::Update` (Xbox PDB): shows the next
/// due tutorial message and picks the next one.
///
/// Does nothing while `004a4040` or `0070ec00` is true. Without
/// `cMenuMode == 2` it resets the display (`eDisplayID = 0x29`,
/// `uiNextDisplay = 0`). When a message is due (`uiNextDisplay` reached by
/// `00457fe0`) and its menu (if any) is the top menu and faded in, it casts
/// the form `0x168 + id` and shows it with `007e8890`, marking the message
/// with [`fn_007185e0`] (a failure is reported with `005b5e40`), then
/// resets the display. Otherwise it looks for another message with bit 0
/// set and bit 1 clear whose conditions hold, clears bit 0 of the current
/// one, makes the found one current and due `delay` after now; a message
/// with no delay repeats the whole update.
pub fn tutorial_manager_update(e: &mut Engine, this: Ptr<TutorialManager>) {
    if e.call(IS_IN_SPECIAL_MODE, &args![]).bool() || e.call(IS_FLAG_011D8950_SET, &args![]).bool()
    {
        return;
    }
    let mut again = true;
    while again {
        again = false;
        let in_game_menu = e.call(ALWAYS_TRUE, &args![]).bool() && {
            let manager = e.global::<u32>(MANAGER_GLOBAL);
            e.mem.u32(manager + 0x0c) == 2
        };
        if !in_game_menu {
            e.set(this, TutorialManager::uiNextDisplay, 0);
            e.set(this, TutorialManager::eDisplayID, TUTORIAL_MESSAGE_COUNT);
            continue;
        }

        let next_display = e.get(this, TutorialManager::uiNextDisplay);
        let displayed = e.get(this, TutorialManager::eDisplayID);
        if next_display != 0
            && e.call(GET_TIME, &args![]).u32() >= next_display
            && displayed < TUTORIAL_MESSAGE_COUNT
        {
            let field = message_menu_field(message_word(e, this, displayed));
            let menu_ok =
                field == 0 || e.call(IS_TOP_MENU_ID, &args![field + FIRST_MENU_ID]).bool();
            if menu_ok && e.call(IS_TOP_MENU_FADED_IN, &args![]).bool() {
                let form_id = (displayed as u32).wrapping_add(TUTORIAL_FIRST_FORM_ID);
                let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
                let message = e
                    .call(
                        DYNAMIC_CAST,
                        &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                    )
                    .u32();
                let shown = message != 0 && e.call(0x007e_8890, &args![message, 0u32]).bool() && {
                    let current = e.get(this, TutorialManager::eDisplayID);
                    fn_007185e0(e, this, current)
                };
                if !shown {
                    e.call(0x005b_5e40, &args![0x0106_f3b0u32]);
                }
                e.set(this, TutorialManager::uiNextDisplay, 0);
                e.set(this, TutorialManager::eDisplayID, TUTORIAL_MESSAGE_COUNT);
                continue;
            }
        }

        for index in 0..TUTORIAL_MESSAGE_COUNT {
            let word = message_word(e, this, index);
            if word & 1 == 0 || word & 2 != 0 {
                continue;
            }
            if e.get(this, TutorialManager::eDisplayID) == index {
                continue;
            }
            // The code tests the current message's menu field, then the
            // candidate's menu.
            let current = e.get(this, TutorialManager::eDisplayID);
            if message_menu_field(message_word(e, this, current)) != 0 {
                let field = message_menu_field(message_word(e, this, index));
                if !e.call(IS_TOP_MENU_ID, &args![field + FIRST_MENU_ID]).bool() {
                    continue;
                }
            }
            if !e.call(IS_TOP_MENU_FADED_IN, &args![]).bool() {
                continue;
            }
            let current = e.get(this, TutorialManager::eDisplayID);
            if current < TUTORIAL_MESSAGE_COUNT {
                let current_word = message_word(e, this, current);
                let field = message_menu_field(current_word);
                if field == 0 || e.call(IS_TOP_MENU_ID, &args![field + FIRST_MENU_ID]).bool() {
                    set_message_word(e, this, current, current_word & !1);
                }
            }
            e.set(this, TutorialManager::eDisplayID, index);
            let delay = (message_word(e, this, index) >> 8) & 0x00ff_ffff;
            let now = e.call(GET_TIME, &args![]).u32();
            e.set(
                this,
                TutorialManager::uiNextDisplay,
                now.wrapping_add(delay),
            );
            again = delay == 0;
        }
    }
}

// Translated from 007185e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a message index in `0..0x29`: clears bit 0 and sets bit 1 of its
/// word and returns true; false for any other index.
pub fn fn_007185e0(e: &mut Engine, this: Ptr<TutorialManager>, index: i32) -> bool {
    if !(0..TUTORIAL_MESSAGE_COUNT).contains(&index) {
        return false;
    }
    let word = message_word(e, this, index) & !1;
    set_message_word(e, this, index, word);
    let word = message_word(e, this, index) | 2;
    set_message_word(e, this, index, word);
    true
}

// Translated from 00718630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::TutorialManager::ShowMessage` (Xbox PDB): for a
/// message index in `0..0x29`, stores `delay` (24 bits) and `menu_id` (0 or
/// `0x3e9`..`0x43c`, stored relative to `0x3e9`) in its word. For a valid
/// menu id, bit 0 becomes whether the form `0x168 + index` casts to a
/// message that `00590ed0` accepts, `005d4a40` is true and bit 1 is clear.
/// A menu id outside the range leaves bit 0 clear. Returns that bit; false
/// for an invalid index.
pub fn tutorial_manager_show_message(
    e: &mut Engine,
    this: Ptr<TutorialManager>,
    index: i32,
    menu_id: u32,
    delay: u32,
) -> bool {
    let mut result = false;
    if !(0..TUTORIAL_MESSAGE_COUNT).contains(&index) {
        return result;
    }
    let word = (message_word(e, this, index) & 0xff) | ((delay & 0x00ff_ffff) << 8);
    set_message_word(e, this, index, word);
    if (FIRST_MENU_ID..=LAST_MENU_ID).contains(&menu_id) || menu_id == 0 {
        let word = message_word(e, this, index);
        if menu_id != 0 {
            let field = ((menu_id - FIRST_MENU_ID) & 0x3f) << 2;
            set_message_word(e, this, index, (word & 0xffff_ff03) | field);
        } else {
            set_message_word(e, this, index, word & 0xffff_ff03);
        }
        let form_id = (index as u32).wrapping_add(TUTORIAL_FIRST_FORM_ID);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        let message = if form != 0 {
            e.call(
                DYNAMIC_CAST,
                &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
            )
            .u32()
        } else {
            0
        };
        result = message != 0
            && e.call(0x0059_0ed0, &args![message]).bool()
            && e.call(ALWAYS_TRUE, &args![]).bool()
            && message_word(e, this, index) & 2 == 0;
    }
    let word = (message_word(e, this, index) & !1) | u32::from(result);
    set_message_word(e, this, index, word);
    result
}

// Translated from 007187a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs bit 1 of every message word into the byte array `out`, eight
/// messages per byte (message `byte * 8 + bit` is bit `bit`), for as many
/// bytes as `00706810` says; a byte stops at message `0x29`.
pub fn fn_007187a0(e: &mut Engine, this: Ptr<TutorialManager>, out: u32) {
    let mut byte_index = 0u32;
    while byte_index < e.call(0x0070_6810, &args![this]).u32() {
        let mut packed = 0u8;
        for bit in 0..8u32 {
            let index = (bit + byte_index * 8) as i32;
            if index == TUTORIAL_MESSAGE_COUNT {
                break;
            }
            if fn_00718840(e, this, index) {
                packed |= 1 << bit;
            }
        }
        e.mem.set_u8(out + byte_index, packed);
        byte_index += 1;
    }
}

// Translated from 00718840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of message `index` is set (false for an index outside
/// `0..0x29`).
pub fn fn_00718840(e: &mut Engine, this: Ptr<TutorialManager>, index: i32) -> bool {
    (0..TUTORIAL_MESSAGE_COUNT).contains(&index) && message_word(e, this, index) & 2 != 0
}

// Translated from 00718890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of [`fn_007187a0`]: sets bit 1 of every message word from the
/// byte array `bytes`.
pub fn fn_00718890(e: &mut Engine, this: Ptr<TutorialManager>, bytes: u32) {
    let mut byte_index = 0u32;
    while byte_index < e.call(0x0070_6810, &args![this]).u32() {
        let packed = e.mem.u8(bytes + byte_index);
        for bit in 0..8u32 {
            let index = (bit + byte_index * 8) as i32;
            if index == TUTORIAL_MESSAGE_COUNT {
                break;
            }
            let set = u32::from(packed) & (1 << bit) != 0;
            let word = (message_word(e, this, index) & !2) | (u32::from(set) << 1);
            set_message_word(e, this, index, word);
        }
        byte_index += 1;
    }
}

// Translated from 00718930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls [`fn_00718960`] on the object at `011dea3c` with `flag`, then
/// `0095f530(object, flag, 1)`. `this` (the manager) is not used.
pub fn fn_00718930(e: &mut Engine, _this: Ptr, flag: u8) {
    let object = e.global::<u32>(OBJECT_011DEA3C);
    fn_00718960(e, Ptr::new(object), flag);
    e.call(0x0095_f530, &args![object, u32::from(flag), 1u32]);
}

// Translated from 00718960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit 2 (value 4) of the byte at `+0x66d`
/// of `this`.
pub fn fn_00718960(e: &mut Engine, this: Ptr, set: u8) {
    let address = this.addr() + 0x66d;
    let flags = i32::from(e.mem.i8(address));
    let flags = if set != 0 { flags | 4 } else { flags & !4 };
    e.mem.set_u8(address, flags as u8);
}

// Translated from 007189c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Maps gamepad button `button` (1..18) to the id `00a25560` takes, `-1`
/// for 7, 8 and anything out of range.
pub fn fn_007189c0(_e: &mut Engine, button: i32) -> i32 {
    match button {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 5,
        5 => 6,
        6 => 7,
        9 => 0xa,
        10 => 0xb,
        11 => 0xc,
        12 => 0xd,
        13 => 0x10,
        14 => 0x11,
        15 => 0xf,
        16 => 0xe,
        17 => 8,
        18 => 9,
        _ => -1,
    }
}

// Translated from 00718ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::GetCurrentImagespaceMod` (Xbox PDB): the form (looked
/// up by id) of the imagespace modifier for the current state: `0x4eee8`
/// when `004a4040` is true, `0x96389` when `00967ae0` (on the object at
/// `011dea3c`) is true, `0x44f34` when the top menu id is `0x3f6`;
/// otherwise `0x32b38` when the tile lookup `00a03da0` (on `00586150` of the
/// manager, with the string at `0106ed28`) finds nothing, else 0.
pub fn interface_manager_get_current_imagespace_mod(e: &mut Engine, _this: Ptr) -> u32 {
    let mut result = 0;
    let top_menu = e.call(GET_TOP_MENU_ID, &args![]).u32();
    if e.call(IS_IN_SPECIAL_MODE, &args![]).bool() {
        result = e.call(LOOKUP_FORM_BY_ID, &args![0x4eee8u32]).u32();
    } else {
        let object = e.global::<u32>(OBJECT_011DEA3C);
        if e.call(0x0096_7ae0, &args![object]).bool() {
            result = e.call(LOOKUP_FORM_BY_ID, &args![0x96389u32]).u32();
        } else if top_menu == 0x3f6 {
            result = e.call(LOOKUP_FORM_BY_ID, &args![0x44f34u32]).u32();
        } else {
            let manager = e.call(GET_MANAGER, &args![]).u32();
            let owner = e.call(0x0058_6150, &args![manager]).u32();
            if e.call(0x00a0_3da0, &args![owner, 0x0106_ed28u32]).u32() == 0 {
                result = e.call(LOOKUP_FORM_BY_ID, &args![0x32b38u32]).u32();
            }
        }
    }
    result
}

// Translated from 00718b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColorManager::GetInstance` (Xbox PDB): the singleton, created
/// (0x10 bytes and [`system_color_manager_new`]) on first use.
pub fn system_color_manager_get_instance(e: &mut Engine) -> Ptr<SystemColorManager> {
    if e.global::<u32>(SYSTEM_COLOR_MANAGER_INSTANCE) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let manager = if block != 0 {
            system_color_manager_new(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set_global(SYSTEM_COLOR_MANAGER_INSTANCE, manager);
    }
    Ptr::new(e.global(SYSTEM_COLOR_MANAGER_INSTANCE))
}

// Translated from 00718bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the `SystemColorManager` singleton: runs `00719010` on it, then
/// [`fn_00718c40`] with 1 (destructor and free), and clears the pointer.
pub fn fn_00718bf0(e: &mut Engine) {
    let manager = e.global::<u32>(SYSTEM_COLOR_MANAGER_INSTANCE);
    e.call(0x0071_9010, &args![manager]);
    let manager = e.global::<u32>(SYSTEM_COLOR_MANAGER_INSTANCE);
    if manager != 0 {
        fn_00718c40(e, Ptr::new(manager), 1);
    }
    e.set_global(SYSTEM_COLOR_MANAGER_INSTANCE, 0u32);
}

// Translated from 00718c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor shape of `SystemColorManager`: runs
/// [`fn_00718c70`] and frees `this` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_00718c40(
    e: &mut Engine,
    this: Ptr<SystemColorManager>,
    flags: u32,
) -> Ptr<SystemColorManager> {
    fn_00718c70(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00718c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColorManager`'s destructor body: destroys its `NiTPointerList`
/// (`004a1a30`).
pub fn fn_00718c70(e: &mut Engine, this: Ptr<SystemColorManager>) {
    e.call(0x004a_1a30, &args![this]);
}

// Translated from 00718c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColorManager::SystemColorManager` (Xbox PDB): constructs the color
/// list (`0048f200`), starts `iNextID` at 1 and registers six system colors:
/// soft colors for the settings at `011d8ad4` (id 1) and `011d8ae4` (id 4),
/// hard colors named by the strings at `0106f3fc` (id 2), `010293a8` (id 3),
/// `0106f3f0` and `0106f3e8` (both with id -1, which the next id replaces).
pub fn system_color_manager_new(
    e: &mut Engine,
    this: Ptr<SystemColorManager>,
) -> Ptr<SystemColorManager> {
    e.call(0x0048_f200, &args![this]);
    e.set(this, SystemColorManager::iNextID, 1);
    fn_00718db0(e, this, 0x0106_f404, 0xff, 0xb6, 0x42, 0x011d_8ad4, 1);
    fn_00718ee0(e, this, 0x0106_f3fc, 0xff, 0x43, 0x2a, 2);
    fn_00718ee0(e, this, 0x0102_93a8, 0x21, 0xe7, 0x79, 3);
    fn_00718db0(e, this, 0x0101_fb30, 0xff, 0xb6, 0x42, 0x011d_8ae4, 4);
    fn_00718ee0(e, this, 0x0106_f3f0, 0xff, 0xb6, 0x42, -1);
    fn_00718ee0(e, this, 0x0106_f3e8, 0xff, 0xb6, 0x42, -1);
    this
}

// Translated from 00718d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0076b630` on `this` and returns `this`.
pub fn fn_00718d90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0076_b630, &args![this]);
    this
}

// Translated from 00718db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers a soft system color named `name`, backed by the setting at
/// `setting` (`007192e0`): when the setting's value (`0043d4d0`) is still 0
/// it is first set to the packed color of the three bytes with a low byte
/// of `0xff` ([`fn_00718e90`]). The new color is appended to the list and
/// `iNextID` becomes `id + 1`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00718db0(
    e: &mut Engine,
    this: Ptr<SystemColorManager>,
    name: u32,
    byte_3: u32,
    byte_2: u32,
    byte_1: u32,
    setting: u32,
    id: i32,
) {
    let value = e.call(0x0043_d4d0, &args![setting]).u32();
    if e.mem.u32(value) == 0 {
        let packed = fn_00718e90(e, byte_3, byte_2, byte_1, 0xff);
        let value = e.call(0x0043_d4d0, &args![setting]).u32();
        e.mem.set_u32(value, packed);
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let color = if block != 0 {
        e.call(0x0071_92e0, &args![block, name, setting, id]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), color);
        e.call(0x004e_d8c0, &args![this, slot]);
    });
    e.set(this, SystemColorManager::iNextID, id.wrapping_add(1));
}

// Translated from 00718e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs four bytes into a word, `cdecl`: `byte_3` in bits 24..31 (not
/// masked), then `byte_2`, `byte_1` and `byte_0`.
pub fn fn_00718e90(_e: &mut Engine, byte_3: u32, byte_2: u32, byte_1: u32, byte_0: u32) -> u32 {
    (byte_3 << 24) | ((byte_2 & 0xff) << 16) | ((byte_1 & 0xff) << 8) | (byte_0 & 0xff)
}

// Translated from 00718ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers a hard system color named `name` with the packed color of the
/// three bytes and a low byte of `0xff` (`00719250`). The id is raised to
/// `iNextID` when lower; `iNextID` becomes the id plus one.
pub fn fn_00718ee0(
    e: &mut Engine,
    this: Ptr<SystemColorManager>,
    name: u32,
    byte_3: u32,
    byte_2: u32,
    byte_1: u32,
    id: i32,
) {
    let mut id = id;
    let next_id = e.get(this, SystemColorManager::iNextID);
    if id < next_id {
        id = next_id;
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let color = if block != 0 {
        let packed = fn_00718e90(e, byte_3, byte_2, byte_1, 0xff);
        e.call(0x0071_9250, &args![block, name, packed, id]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), color);
        e.call(0x004e_d8c0, &args![this, slot]);
    });
    e.set(this, SystemColorManager::iNextID, id.wrapping_add(1));
}

// Translated from 00718fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColorManager::Find` (Xbox PDB): the system color whose id string
/// (`strXMLID`, `+0x04`) maps to the trait id `id` (`Tile::TextToTrait`), or
/// 0.
pub fn system_color_manager_find(e: &mut Engine, this: Ptr<SystemColorManager>, id: u32) -> u32 {
    let mut iterator = e.call(0x0055_9450, &args![this]).u32();
    // The exe's frame (`SUB ESP,0xc` under `EBP`): the iterator at `EBP-0x4` (+8).
    e.with_stack(0xc, |e, frame| {
        let slot = frame.addr() + 8;
        let mut found = 0;
        while iterator != 0 {
            e.mem.set_u32(slot, iterator);
            let item_ref = e.call(0x0057_cbe0, &args![this, slot]).u32();
            iterator = e.mem.u32(slot);
            let color = e.mem.u32(item_ref);
            if color != 0 {
                let text = e.call(0x007f_a950, &args![color]).u32();
                if e.call(0x00a0_1860, &args![text]).u32() == id {
                    found = color;
                    break;
                }
            }
        }
        found
    })
}

/// `SystemColor`'s vtable, `HardSystemColor`'s and `SoftSystemColor`'s.
const SYSTEM_COLOR_VTABLE: u32 = 0x0106_f418;
const HARD_SYSTEM_COLOR_VTABLE: u32 = 0x0106_f430;
const SOFT_SYSTEM_COLOR_VTABLE: u32 = 0x0106_f448;
/// Vtable of the array of tiles (`BSSimpleArray<Tile *>`).
const TILE_ARRAY_VTABLE: u32 = 0x0106_f798;
/// The format `SystemColor`'s constructor writes its id string with.
const SYSTEM_COLOR_NAME_FORMAT: u32 = 0x0106_f40c;
/// `Tile::TextToTrait` (Xbox PDB), `cdecl`: the trait id of a name.
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `009ff8a0`, `cdecl`: text and an integer (the constructor passes the
/// color's id, the menu refresh a ready flag).
const SET_NAMED_VALUE: u32 = 0x009f_f8a0;
/// `0044ddc0`: the word at `+0x08` of its object.
const GET_WORD_AT_8: u32 = 0x0044_ddc0;
/// `005330e0`, `cdecl`: clamps the `float` at the address to `[min, max]`.
const CLAMP_FLOAT: u32 = 0x0053_30e0;
/// `Tile` value setter with a `float` (`00a012d0`): trait id, value, flag.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `Tile::SetNeedsUpdate` (Xbox PDB): update flags.
const TILE_SET_NEEDS_UPDATE: u32 = 0x00a0_74d0;
/// `00700300`: the name string of a tile (the word at `+0x20`).
const TILE_NAME: u32 = 0x0070_0300;
/// `00408b20`, `cdecl`: compares two strings, 0 when equal.
const COMPARE_STRINGS: u32 = 0x0040_8b20;
/// `Tile::GetMenu` (Xbox PDB): the menu a tile belongs to.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `007d6c30`: refreshes the start menu after the manager changed.
const REFRESH_START_MENU: u32 = 0x007d_6c30;
/// `004de2d0`: stores a byte flag in the object at `011d8c4c`.
const SET_RENDERER_FLAG: u32 = 0x004d_e2d0;
const RENDERER_OBJECT: u32 = 0x011d_8c4c;
/// `005407b0`: stores a byte in the object it is called on.
const STORE_BYTE: u32 = 0x0054_07b0;
/// `00877720`: the object reached from the one at `011dea0c`.
const GET_OBJECT_FROM_011DEA0C: u32 = 0x0087_7720;
/// `004bd510`: rounds the first `float` up to a multiple of the second
/// (result in `ST0`).
const ROUND_UP_TO_MULTIPLE: u32 = 0x004b_d510;
/// The `float` multiplier `SystemColor::GetColor` scales each byte with
/// (`0102f0e4`) and the `double` `SystemColor::SetColor` scales each channel
/// with before rounding it up (`0101e568`, also used by the fade).
const COLOR_BYTE_SCALE: u32 = 0x0102_f0e4;
const COLOR_CHANNEL_SCALE: u32 = 0x0101_e568;
/// The strings the tile refresh compares tile names with and the names it
/// registers or looks up.
const TILE_NAME_FIRST: u32 = 0x0106_f47c;
const TILE_NAME_SECOND: u32 = 0x0106_f46c;
const READY_TRAIT_NAME: u32 = 0x0106_f45c;
const NAMED_VALUE_FIRST: u32 = 0x0106_f490;
const NAMED_VALUE_SECOND: u32 = 0x0106_f488;
const ROOT_TRAIT_NAME: u32 = 0x0106_efcc;
/// The fade state `00719790` keeps: the last time (`u32`), the level
/// (`float`) and the direction (`u8`).
const FADE_LAST_TIME: u32 = 0x011d_8a64;
const FADE_LEVEL: u32 = 0x011d_8a68;
const FADE_FALLING: u32 = 0x011d_8a50;
/// Its constants: the upper and lower bound of the level (`float`), the rate
/// per millisecond (`float`), the maximum (`float`), and the `double`
/// threshold (`0102e430`) above which the inverse level is shown.
const FADE_UPPER_BOUND: u32 = 0x0106_f49c;
const FADE_LOWER_BOUND: u32 = 0x0106_f498;
const FADE_RATE: u32 = 0x0101_622c;
const FLOAT_MAXIMUM: u32 = 0x0102_3cd8;
const INVERSE_THRESHOLD: u32 = 0x0102_e430;
/// Trait id `00719790` sets on the tiles.
const TRAIT_FADE: u32 = 0xfa9;
/// The clock object `00719790` reads the time from.
const CLOCK_OBJECT: u32 = 0x011f_6394;
/// `00401170`: the type byte at `+0x04` of an object; `0x28` is what
/// `00719790` requires of the object it reads a count from.
const GET_TYPE_BYTE: u32 = 0x0040_1170;
const EXPECTED_TYPE: u32 = 0x28;
/// `ActorValueOwner::GetClampedActorFloatValue` (Xbox PDB) on the owner at
/// `+0xA4` of the object at `011dea3c`.
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef50;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), `cdecl`.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `00663b60` and `MiddleHighProcess::GetDesiredTarget` (`008d8160`): the
/// integer the fade compares its value with.
const GET_COUNT_FROM_TYPE: u32 = 0x0066_3b60;
const GET_DESIRED_TARGET: u32 = 0x008d_8160;
/// `MiddleHighProcess::GetLastBoundWeapon` (Xbox PDB).
const GET_LAST_BOUND_WEAPON: u32 = 0x008d_85e0;
/// `00825c00`: the current time (the word at `+0x14` of the object).
const GET_CLOCK_TIME: u32 = 0x0082_5c00;

layout! {
    /// `SystemColorManager::SystemColor` (Xbox PDB), 0x0C bytes: vtable,
    /// then the id string.
    pub struct SystemColor: 0x0C {
        /// `strXMLID` (Xbox PDB): a `BSStringT<char>`.
        0x04 strXMLID: u32,
    }

    /// `SystemColorManager::HardSystemColor` (Xbox PDB), 0x10 bytes.
    pub struct HardSystemColor: 0x10 {
        /// `iRGB` (Xbox PDB): the packed color.
        0x0C iRGB: u32,
    }

    /// `SystemColorManager::SoftSystemColor` (Xbox PDB), 0x10 bytes.
    pub struct SoftSystemColor: 0x10 {
        /// `pSetting` (Xbox PDB): `INIPrefSetting*`.
        0x0C pSetting: u32,
    }
}

/// Clamps `value` to `[min, max]` through `005330e0`, which works on a
/// `float` in memory.
fn clamp_float(e: &mut Engine, value: f32, min: f32, max: f32) -> f32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), value);
        e.call(CLAMP_FLOAT, &args![slot, min, max]);
        e.mem.f32(slot.addr())
    })
}

/// `FISTP` with truncation to a 64-bit integer, the low word kept (a value
/// out of range or not a number gives `0x8000000000000000`).
fn truncate_low_word(value: f64) -> u32 {
    let limit = 9_223_372_036_854_775_808.0f64; // 2^63
    if value.is_nan() || !(-limit..limit).contains(&value) {
        0
    } else {
        value as i64 as u32
    }
}

// Translated from 00719010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the colors of the manager: while its list (`0076b610` tests
/// whether it is empty) has items, takes the first one off (`007b5390`) and
/// deletes it through its virtual destructor (slot 0, flag 1).
pub fn fn_00719010(e: &mut Engine, this: Ptr<SystemColorManager>) {
    while !e.call(0x0076_b610, &args![this]).bool() {
        let color = e.call(0x007b_5390, &args![this]).u32();
        if color != 0 {
            e.vcall(color, 0, &args![1u32]);
        }
    }
}

// Translated from 00719060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColorManager::GetColor` (Xbox PDB): looks the color up with
/// [`system_color_manager_find`] and, when found, writes it into `color`
/// ([`system_color_get_color`]). Returns whether it was found.
pub fn system_color_manager_get_color(
    e: &mut Engine,
    this: Ptr<SystemColorManager>,
    id: u32,
    color: Ptr,
) -> bool {
    let found = system_color_manager_find(e, this, id);
    if found == 0 {
        return false;
    }
    system_color_get_color(e, Ptr::new(found), color);
    true
}

// Translated from 007190a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The packed color of the system color `id` (its virtual `GetColor`,
/// slot 4), 0 when there is none.
pub fn fn_007190a0(e: &mut Engine, this: Ptr<SystemColorManager>, id: u32) -> u32 {
    let found = system_color_manager_find(e, this, id);
    if found == 0 {
        return 0;
    }
    e.vcall(found, 4, &args![]).u32()
}

// Translated from 007190e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColorManager::SetColor` (Xbox PDB): looks the color up and, when
/// found, sets it from `color` ([`system_color_set_color`]). Returns whether
/// it was found.
pub fn system_color_manager_set_color(
    e: &mut Engine,
    this: Ptr<SystemColorManager>,
    id: u32,
    color: Ptr,
) -> bool {
    let found = system_color_manager_find(e, this, id);
    if found == 0 {
        return false;
    }
    system_color_set_color(e, Ptr::new(found), color);
    true
}

// Translated from 00719120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the packed color of the system color `id` through its virtual
/// `SetColor` (slot 8). Returns whether it was found.
pub fn fn_00719120(e: &mut Engine, this: Ptr<SystemColorManager>, id: u32, value: u32) -> bool {
    let found = system_color_manager_find(e, this, id);
    if found == 0 {
        return false;
    }
    e.vcall(found, 8, &args![value]);
    true
}

// Translated from 00719160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColor`'s constructor: sets the vtable, builds the id string
/// (`004037b0`), writes the name into it with the format at `0106f40c`
/// (`00406f60`), runs `004afad0` on it and registers its text under `id`
/// (`009ff8a0`). Returns `this`. The compiler's exception frame is not
/// translated.
pub fn fn_00719160(e: &mut Engine, this: Ptr<SystemColor>, name: u32, id: u32) -> Ptr<SystemColor> {
    e.mem.set_u32(this.addr(), SYSTEM_COLOR_VTABLE);
    let text = this.addr() + 4;
    e.call(0x0040_37b0, &args![text]);
    e.call(0x0040_6f60, &args![text, SYSTEM_COLOR_NAME_FORMAT, name]);
    e.call(0x004a_fad0, &args![text]);
    let characters = e.call(0x0055_9450, &args![text]).u32();
    e.call(SET_NAMED_VALUE, &args![characters, id]);
    this
}

// Translated from 00719200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `SystemColor`: [`fn_00719230`], then
/// frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00719200(e: &mut Engine, this: Ptr<SystemColor>, flags: u32) -> Ptr<SystemColor> {
    fn_00719230(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00719230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColor`'s destructor body: sets its vtable and destroys the id
/// string (`004037d0`).
pub fn fn_00719230(e: &mut Engine, this: Ptr<SystemColor>) {
    e.mem.set_u32(this.addr(), SYSTEM_COLOR_VTABLE);
    e.call(0x0040_37d0, &args![this.addr() + 4]);
}

// Translated from 00719250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HardSystemColor`'s constructor: the base constructor
/// ([`fn_00719160`]) with `name` and `id`, its own vtable and the packed
/// color `rgb`. Returns `this`.
pub fn fn_00719250(
    e: &mut Engine,
    this: Ptr<HardSystemColor>,
    name: u32,
    rgb: u32,
    id: u32,
) -> Ptr<HardSystemColor> {
    fn_00719160(e, Ptr::new(this.addr()), name, id);
    e.mem.set_u32(this.addr(), HARD_SYSTEM_COLOR_VTABLE);
    e.set(this, HardSystemColor::iRGB, rgb);
    this
}

// Translated from 00719290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `HardSystemColor`: [`fn_007192c0`],
/// then frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00719290(e: &mut Engine, this: Ptr<HardSystemColor>, flags: u32) -> Ptr<HardSystemColor> {
    fn_007192c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007192c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HardSystemColor`'s destructor body: sets its vtable, then the base
/// destructor ([`fn_00719230`]).
pub fn fn_007192c0(e: &mut Engine, this: Ptr<HardSystemColor>) {
    e.mem.set_u32(this.addr(), HARD_SYSTEM_COLOR_VTABLE);
    fn_00719230(e, Ptr::new(this.addr()));
}

// Translated from 007192e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SoftSystemColor`'s constructor: the base constructor
/// ([`fn_00719160`]) with `name` and `id`, its own vtable and the setting
/// the color lives in. Returns `this`.
pub fn fn_007192e0(
    e: &mut Engine,
    this: Ptr<SoftSystemColor>,
    name: u32,
    setting: u32,
    id: u32,
) -> Ptr<SoftSystemColor> {
    fn_00719160(e, Ptr::new(this.addr()), name, id);
    e.mem.set_u32(this.addr(), SOFT_SYSTEM_COLOR_VTABLE);
    e.set(this, SoftSystemColor::pSetting, setting);
    this
}

// Translated from 00719320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SoftSystemColor::GetColor` (Xbox PDB): the packed color stored in the
/// setting (`0043d4d0` gives the address of the setting's value).
pub fn soft_system_color_get_color(e: &mut Engine, this: Ptr<SoftSystemColor>) -> u32 {
    let setting = e.get(this, SoftSystemColor::pSetting);
    let value = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.u32(value)
}

// Translated from 00719340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SoftSystemColor::SetColor` (Xbox PDB): stores the packed color in the
/// setting.
pub fn soft_system_color_set_color(e: &mut Engine, this: Ptr<SoftSystemColor>, rgb: u32) {
    let setting = e.get(this, SoftSystemColor::pSetting);
    let value = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.set_u32(value, rgb);
}

// Translated from 00719360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `SoftSystemColor`: [`fn_00719390`],
/// then frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00719360(e: &mut Engine, this: Ptr<SoftSystemColor>, flags: u32) -> Ptr<SoftSystemColor> {
    fn_00719390(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00719390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SoftSystemColor`'s destructor body: sets its vtable, then the base
/// destructor ([`fn_00719230`]).
pub fn fn_00719390(e: &mut Engine, this: Ptr<SoftSystemColor>) {
    e.mem.set_u32(this.addr(), SOFT_SYSTEM_COLOR_VTABLE);
    fn_00719230(e, Ptr::new(this.addr()));
}

// Translated from 007193b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColor::GetColor` (Xbox PDB), the overload that fills a color:
/// takes the packed color from the virtual `GetColor` (slot 4), splits it
/// into bytes ([`fn_00719440`]) and writes the three highest as `float`s
/// (each times the scale at `0102f0e4`) into the three floats at `color`.
pub fn system_color_get_color(e: &mut Engine, this: Ptr<SystemColor>, color: Ptr) {
    let packed = e.vcall(this.addr(), 4, &args![]).u32();
    // The exe's frame (`SUB ESP,0x2c` under `EBP`): the bytes at `EBP-0xc`, `EBP-0x10`, `EBP-0x8`
    // and `EBP-0x4` (+0x20, +0x1c, +0x24, +0x28), the first three for red, green and blue.
    e.with_stack(0x2c, |e, frame| {
        let bytes = [0x20, 0x1c, 0x24, 0x28].map(|offset| frame.addr() + offset);
        fn_00719440(
            e,
            packed,
            Ptr::new(bytes[0]),
            Ptr::new(bytes[1]),
            Ptr::new(bytes[2]),
            Ptr::new(bytes[3]),
        );
        let scale: f32 = e.global(COLOR_BYTE_SCALE);
        for channel in 0..3u32 {
            let byte = e.mem.u32(bytes[channel as usize]);
            e.mem.set_f32(
                color.addr() + 4 * channel,
                (byte as f64 * scale as f64) as f32,
            );
        }
    });
}

// Translated from 00719440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits a packed color, `cdecl`: writes its bytes, highest first, to the
/// four addresses.
pub fn fn_00719440(
    e: &mut Engine,
    packed: u32,
    byte_3: Ptr,
    byte_2: Ptr,
    byte_1: Ptr,
    byte_0: Ptr,
) {
    e.mem.set_u32(byte_3.addr(), (packed >> 24) & 0xff);
    e.mem.set_u32(byte_2.addr(), (packed >> 16) & 0xff);
    e.mem.set_u32(byte_1.addr(), (packed >> 8) & 0xff);
    e.mem.set_u32(byte_0.addr(), packed & 0xff);
}

// Translated from 00719490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SystemColor::SetColor` (Xbox PDB), the overload that takes a color:
/// scales the three `float` channels at `color` by the `double` at
/// `0101e568`, rounds each up with `004bd510(value, 1.0)` and truncates it
/// to an integer (blue, green, then red), packs them with a low byte of
/// `0xff` ([`fn_00718e90`]) and stores the result through the virtual
/// `SetColor` (slot 8).
pub fn system_color_set_color(e: &mut Engine, this: Ptr<SystemColor>, color: Ptr) {
    let scale: f64 = e.global(COLOR_CHANNEL_SCALE);
    let mut channels = [0u32; 3];
    for channel in [2u32, 1, 0] {
        let value = e.mem.f32(color.addr() + 4 * channel);
        let scaled = (value as f64 * scale) as f32;
        let rounded = st0(e.call(ROUND_UP_TO_MULTIPLE, &args![scaled, 1.0f32]));
        channels[channel as usize] = truncate_low_word(rounded);
    }
    let packed = fn_00718e90(e, channels[0], channels[1], channels[2], 0xff);
    e.vcall(this.addr(), 8, &args![packed]);
}

// Translated from 00719580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes a tile and, first, all its children (the list at `+0x04`): a
/// tile whose name equals one of the two strings at `0106f47c` and
/// `0106f46c` gets the trait named by `0106f45c` set to whether the manager
/// is ready (`004b71d0`) and `Tile::SetNeedsUpdate(4)`.
pub fn fn_00719580(e: &mut Engine, tile: Ptr) {
    let children = tile.addr() + 4;
    let mut node = e.call(0x0055_9450, &args![children]).u32();
    // The exe's frame (`SUB ESP,0x8` under `EBP`): the iterator at `EBP-0x8` (+0).
    e.with_stack(0x8, |e, frame| {
        let slot = frame.addr();
        while node != 0 {
            e.mem.set_u32(slot, node);
            let item = e.call(0x0057_cbe0, &args![children, slot]).u32();
            node = e.mem.u32(slot);
            let child = e.mem.u32(item);
            fn_00719580(e, Ptr::new(child));
        }
    });

    let name = e.call(TILE_NAME, &args![tile]).u32();
    let mut matches = e.call(COMPARE_STRINGS, &args![name, TILE_NAME_FIRST]).u32() == 0;
    if !matches {
        let name = e.call(TILE_NAME, &args![tile]).u32();
        matches = e
            .call(COMPARE_STRINGS, &args![name, TILE_NAME_SECOND])
            .u32()
            == 0;
    }
    if matches {
        e.call(GET_MANAGER, &args![]);
        let ready = e.call(MANAGER_READY, &args![]).bool() as u32;
        let trait_id = e.call(TEXT_TO_TRAIT, &args![READY_TRAIT_NAME]).u32();
        e.call(TILE_SET_VALUE, &args![tile, trait_id, ready]);
        e.call(TILE_SET_NEEDS_UPDATE, &args![tile, 4u32]);
    }
}

// Translated from 00719630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the manager's ready state to the interface: registers it under
/// the two names at `0106f490` and `0106f488`, sets the trait named by
/// `0106efcc` on the string root tile (`+0xA0`), refreshes the menus root
/// tile (`+0x9C`) with [`fn_00719580`], then calls virtual slot `0x44` of the
/// menu of each of its children with the ready state, and finally runs
/// `007d6c30`. The word it is passed is not read.
pub fn fn_00719630(e: &mut Engine, this: Ptr<InterfaceManagerFields>, _unused_1: u32) {
    e.call(GET_MANAGER, &args![]);
    let ready = e.call(MANAGER_READY, &args![]).bool() as u32;
    e.call(SET_NAMED_VALUE, &args![NAMED_VALUE_FIRST, ready]);
    e.call(GET_MANAGER, &args![]);
    let ready = e.call(MANAGER_READY, &args![]).bool() as u32;
    e.call(SET_NAMED_VALUE, &args![NAMED_VALUE_SECOND, ready]);
    let ready = e.call(MANAGER_READY, &args![]).bool() as u32;
    let trait_id = e.call(TEXT_TO_TRAIT, &args![ROOT_TRAIT_NAME]).u32();
    // InterfaceManager::pStringRoot (Xbox PDB) +0xA0
    let string_root = e.mem.u32(this.addr() + 0xA0);
    e.call(TILE_SET_VALUE, &args![string_root, trait_id, ready]);

    // InterfaceManager::pMenusRoot (Xbox PDB) +0x9C
    let menus_root = e.mem.u32(this.addr() + 0x9C);
    fn_00719580(e, Ptr::new(menus_root));
    let menus_root = e.mem.u32(this.addr() + 0x9C);
    let mut node = if menus_root != 0 {
        e.call(0x0055_9450, &args![menus_root + 4]).u32()
    } else {
        0
    };
    // The exe's frame (`SUB ESP,0x14` under `EBP`): the iterator at `EBP-0x4` (+0x10).
    e.with_stack(0x14, |e, frame| {
        let slot = frame.addr() + 0x10;
        while node != 0 {
            e.mem.set_u32(slot, node);
            let menus_root = e.mem.u32(this.addr() + 0x9C);
            let item = e.call(0x0057_cbe0, &args![menus_root + 4, slot]).u32();
            node = e.mem.u32(slot);
            let child = e.mem.u32(item);
            if child != 0 && e.call(TILE_GET_MENU, &args![child]).u32() != 0 {
                let menu = e.call(TILE_GET_MENU, &args![child]).u32();
                let ready = e.call(MANAGER_READY, &args![]).bool() as u32;
                e.vcall(menu, 0x44, &args![ready]);
            }
        }
    });
    e.call(REFRESH_START_MENU, &args![]);
}

// Translated from 00719740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `flag` in the object at `011d8c4c` (`004de2d0`), applies the
/// opposite state to the interface ([`fn_00719630`] on the manager, with
/// `flag == 0`) and stores `flag` in the object `00877720` finds from the one
/// at `011dea0c` (`005407b0`). The object it is called on is not read.
pub fn fn_00719740(e: &mut Engine, _unused_0: Ptr<InterfaceManagerFields>, flag: u8) {
    e.call(SET_RENDERER_FLAG, &args![RENDERER_OBJECT, flag]);
    let manager = e.call(GET_MANAGER, &args![]).u32();
    fn_00719630(e, Ptr::new(manager), (flag == 0) as u32);
    let object = e.global::<u32>(OBJECT_011DEA0C);
    let target = e.call(GET_OBJECT_FROM_011DEA0C, &args![object]).u32();
    e.call(STORE_BYTE, &args![target, flag]);
}

/// `005330e0` on the `float` at `address`.
fn clamp_float_at(e: &mut Engine, address: u32, min: f32, max: f32) {
    e.call(CLAMP_FLOAT, &args![address, min, max]);
}

// Translated from 00719790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame update of a fade level kept at `011d8a68` (0 to 255) that drives
/// the trait `0xfa9` of up to three tiles, `cdecl`.
///
/// `tile_a` and `tile_b` (when both exist) get the inverse of the level and
/// the level. `tile_c` (when it and `source` exist) gets 255, or the level
/// (the inverse when that is above 128) when a count read from `source`'s
/// object (type byte `0x28`) is above a limit: with `use_last_weapon` zero the
/// limit is the player's actor value 5 plus the entry point `0x35` result and
/// the count `00663b60`, otherwise it is the actor value of the last bound
/// weapon and the count is `MiddleHighProcess::GetDesiredTarget`. Last, the
/// level moves by the time since the last call times the rate (down while
/// `011d8a50` is set) and is clamped to the two bounds; reaching the upper one
/// sets the falling flag, reaching the lower one clears it.
pub fn fn_00719790(
    e: &mut Engine,
    tile_a: Ptr,
    tile_b: Ptr,
    tile_c: Ptr,
    source: Ptr,
    use_last_weapon: u8,
) {
    let upper: f32 = e.global(FADE_UPPER_BOUND);
    let lower: f32 = e.global(FADE_LOWER_BOUND);
    let rate: f32 = e.global(FADE_RATE);
    let maximum: f32 = e.global(FLOAT_MAXIMUM);
    let stored: f32 = e.global(FADE_LEVEL);
    let level = clamp_float(e, stored, 0.0, maximum);
    let inverse = (e.global::<f64>(COLOR_CHANNEL_SCALE) - level as f64) as f32;
    let inverse = clamp_float(e, inverse, 0.0, maximum);

    if !tile_a.is_null() && !tile_b.is_null() {
        e.call(TILE_SET_FLOAT, &args![tile_a, TRAIT_FADE, inverse, 1u32]);
        e.call(TILE_SET_FLOAT, &args![tile_b, TRAIT_FADE, level, 1u32]);
    }

    let player = e.global::<u32>(OBJECT_011DEA3C);
    if !tile_c.is_null() && !source.is_null() {
        let mut value = maximum;
        if use_last_weapon == 0 {
            let object = e.call(GET_WORD_AT_8, &args![source]).u32();
            let bonus = e.with_stack(4, |e, slot| {
                e.mem.set_f32(slot.addr(), 0.0);
                let owner = e.call(GET_WORD_AT_8, &args![source]).u32();
                e.call(HANDLE_ENTRY_POINT, &args![0x35u32, player, owner, slot]);
                e.mem.f32(slot.addr())
            });
            let actor_value = st0(e.call(GET_CLAMPED_ACTOR_VALUE, &args![player + 0xA4, 5u32]));
            let limit = (actor_value + bonus as f64) as f32;
            if object != 0 && e.call(GET_TYPE_BYTE, &args![object]).u32() == EXPECTED_TYPE {
                let count = e.call(GET_COUNT_FROM_TYPE, &args![object]).i32();
                if (limit as f64) < count as f64 {
                    value = fade_shown_level(e, level, inverse);
                }
            }
        } else {
            let object = e.call(GET_WORD_AT_8, &args![source]).u32();
            let weapon = e.call(GET_LAST_BOUND_WEAPON, &args![object]).u32();
            let actor_value = st0(e.vcall(player + 0xA4, 0xC, &args![weapon])) as f32;
            if object != 0 && e.call(GET_TYPE_BYTE, &args![object]).u32() == EXPECTED_TYPE {
                let count = e.call(GET_DESIRED_TARGET, &args![object]).i32();
                if (actor_value as f64) < count as f64 {
                    value = fade_shown_level(e, level, inverse);
                }
            }
        }
        e.call(TILE_SET_FLOAT, &args![tile_c, TRAIT_FADE, value, 1u32]);
    }

    let now = e.call(GET_CLOCK_TIME, &args![CLOCK_OBJECT]).u32();
    let elapsed = now.wrapping_sub(e.global::<u32>(FADE_LAST_TIME));
    e.set_global(FADE_LAST_TIME, now);
    let current: f32 = e.global(FADE_LEVEL);
    let step = elapsed as f64 * rate as f64;
    let moved = if e.global::<u8>(FADE_FALLING) != 0 {
        current as f64 - step
    } else {
        step + current as f64
    };
    e.set_global(FADE_LEVEL, moved as f32);
    clamp_float_at(e, FADE_LEVEL, lower, upper);
    let current: f32 = e.global(FADE_LEVEL);
    if upper <= current {
        e.set_global(FADE_FALLING, 1u8);
    }
    if current <= lower {
        e.set_global(FADE_FALLING, 0u8);
    }
}

/// The level `00719790` shows on its third tile: the inverse when it is above
/// the `double` at `0102e430` (128), the level otherwise.
fn fade_shown_level(e: &Engine, level: f32, inverse: f32) -> f32 {
    if inverse as f64 > e.global::<f64>(INVERSE_THRESHOLD) {
        inverse
    } else {
        level
    }
}

// Translated from 00719ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs [`fn_00719b00`] on the object at `011dea3c`. The object it is called
/// on is not read.
pub fn fn_00719ae0(e: &mut Engine, _unused_0: Ptr<InterfaceManagerFields>) -> bool {
    let object = e.global::<u32>(OBJECT_011DEA3C);
    fn_00719b00(e, Ptr::new(object))
}

// Translated from 00719b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the byte at `+0x66D` is set.
pub fn fn_00719b00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x66d) & 4 != 0
}

// Translated from 00719b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Linear search of a `BSSimpleArray` of words from index `start`: calls the
/// `cdecl` function `matches(&element, argument)` on each element and returns
/// the index of the first one it accepts, or -1. A null buffer skips the call
/// for every index.
pub fn fn_00719b20(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    argument: u32,
    start: u32,
    matches: u32,
) -> i32 {
    let mut index = start;
    while index < e.get(this, BSSimpleArray::iSize) {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        if buffer != 0 {
            let element = buffer.wrapping_add(index.wrapping_mul(4));
            if e.call(matches, &args![element, argument]).bool() {
                return index as i32;
            }
        }
        index += 1;
    }
    -1
}

// Translated from 00719b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the array of tiles (`BSSimpleArray<Tile *>`): sets the
/// vtable at `0106f798` and runs the base constructor `006b3eb0(0, 0)`.
/// Returns `this`.
pub fn fn_00719b80(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), TILE_ARRAY_VTABLE);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00719bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<Tile_P_1024>`'s scalar deleting destructor (Xbox PDB):
/// runs the destructor body `00719be0`, then frees `this` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00719bb0(e: &mut Engine, this: Ptr<BSSimpleArray>, flags: u32) -> Ptr<BSSimpleArray> {
    e.call(0x0071_9be0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00717230, fn_00717230(Ptr, u32) -> bool),
        entry!(0x00717280, interface_manager_play_menu_sound(i32)),
        entry!(0x00717600, fn_00717600(Ptr<InterfaceManagerFields>)),
        entry!(
            0x00717660,
            interface_manager_pre_load_main_menus(Ptr<InterfaceManagerFields>)
        ),
        entry!(0x00717740, interface_manager_set_cursor_alpha(i32)),
        entry!(0x007177c0, fn_007177c0() -> f64),
        entry!(0x00717820, fn_00717820() -> f64),
        entry!(0x00717860, interface_manager_get_menu_height() -> f32),
        entry!(0x00717890, fn_00717890() -> i32),
        entry!(0x007178a0, interface_manager_is_in_pipboy_menu() -> bool),
        entry!(0x00717920, interface_manager_is_pipboy_menu_topmost() -> bool),
        entry!(0x00717990, interface_manager_is_current_rendered_menu_topmost() -> bool),
        entry!(0x00717a40, interface_manager_check_menu_button(i32, u8) -> i32),
        entry!(0x00717de0, interface_manager_clear_menu_button(i32)),
        entry!(0x00717e70, interface_manager_do_enter(Ptr, u32, u32, u32) -> u32),
        entry!(0x00717ef0, interface_manager_do_leave(Ptr, u32, u32, u32) -> u32),
        entry!(0x00717f80, interface_manager_do_gamepad(Ptr, u32, u32, f32) -> bool),
        entry!(
            0x00718080,
            interface_manager_do_wheel_move(Ptr<InterfaceManagerFields>, u32, u32, u32) -> u32
        ),
        entry!(
            0x007181d0,
            interface_manager_enter_rendered_menu(Ptr<InterfaceManagerFields>, u32)
        ),
        entry!(0x007182b0, fn_007182b0(u32)),
        entry!(0x007182e0, tutorial_manager_update(Ptr<TutorialManager>)),
        entry!(0x007185e0, fn_007185e0(Ptr<TutorialManager>, i32) -> bool),
        entry!(
            0x00718630,
            tutorial_manager_show_message(Ptr<TutorialManager>, i32, u32, u32) -> bool
        ),
        entry!(0x007187a0, fn_007187a0(Ptr<TutorialManager>, u32)),
        entry!(0x00718840, fn_00718840(Ptr<TutorialManager>, i32) -> bool),
        entry!(0x00718890, fn_00718890(Ptr<TutorialManager>, u32)),
        entry!(0x00718930, fn_00718930(Ptr, u8)),
        entry!(0x00718960, fn_00718960(Ptr, u8)),
        entry!(0x007189c0, fn_007189c0(i32) -> i32),
        entry!(0x00718ab0, interface_manager_get_current_imagespace_mod(Ptr) -> u32),
        entry!(0x00718b60, system_color_manager_get_instance() -> Ptr<SystemColorManager>),
        entry!(0x00718bf0, fn_00718bf0()),
        entry!(
            0x00718c40,
            fn_00718c40(Ptr<SystemColorManager>, u32) -> Ptr<SystemColorManager>
        ),
        entry!(0x00718c70, fn_00718c70(Ptr<SystemColorManager>)),
        entry!(
            0x00718c90,
            system_color_manager_new(Ptr<SystemColorManager>) -> Ptr<SystemColorManager>
        ),
        entry!(0x00718d90, fn_00718d90(Ptr) -> Ptr),
        entry!(
            0x00718db0,
            fn_00718db0(Ptr<SystemColorManager>, u32, u32, u32, u32, u32, i32)
        ),
        entry!(0x00718e90, fn_00718e90(u32, u32, u32, u32) -> u32),
        entry!(
            0x00718ee0,
            fn_00718ee0(Ptr<SystemColorManager>, u32, u32, u32, u32, i32)
        ),
        entry!(
            0x00718fb0,
            system_color_manager_find(Ptr<SystemColorManager>, u32) -> u32
        ),
        entry!(0x00719010, fn_00719010(Ptr<SystemColorManager>)),
        entry!(
            0x00719060,
            system_color_manager_get_color(Ptr<SystemColorManager>, u32, Ptr) -> bool
        ),
        entry!(0x007190a0, fn_007190a0(Ptr<SystemColorManager>, u32) -> u32),
        entry!(
            0x007190e0,
            system_color_manager_set_color(Ptr<SystemColorManager>, u32, Ptr) -> bool
        ),
        entry!(
            0x00719120,
            fn_00719120(Ptr<SystemColorManager>, u32, u32) -> bool
        ),
        entry!(
            0x00719160,
            fn_00719160(Ptr<SystemColor>, u32, u32) -> Ptr<SystemColor>
        ),
        entry!(
            0x00719200,
            fn_00719200(Ptr<SystemColor>, u32) -> Ptr<SystemColor>
        ),
        entry!(0x00719230, fn_00719230(Ptr<SystemColor>)),
        entry!(
            0x00719250,
            fn_00719250(Ptr<HardSystemColor>, u32, u32, u32) -> Ptr<HardSystemColor>
        ),
        entry!(
            0x00719290,
            fn_00719290(Ptr<HardSystemColor>, u32) -> Ptr<HardSystemColor>
        ),
        entry!(0x007192c0, fn_007192c0(Ptr<HardSystemColor>)),
        entry!(
            0x007192e0,
            fn_007192e0(Ptr<SoftSystemColor>, u32, u32, u32) -> Ptr<SoftSystemColor>
        ),
        entry!(
            0x00719320,
            soft_system_color_get_color(Ptr<SoftSystemColor>) -> u32
        ),
        entry!(
            0x00719340,
            soft_system_color_set_color(Ptr<SoftSystemColor>, u32)
        ),
        entry!(
            0x00719360,
            fn_00719360(Ptr<SoftSystemColor>, u32) -> Ptr<SoftSystemColor>
        ),
        entry!(0x00719390, fn_00719390(Ptr<SoftSystemColor>)),
        entry!(0x007193b0, system_color_get_color(Ptr<SystemColor>, Ptr)),
        entry!(0x00719440, fn_00719440(u32, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00719490, system_color_set_color(Ptr<SystemColor>, Ptr)),
        entry!(0x00719580, fn_00719580(Ptr)),
        entry!(0x00719630, fn_00719630(Ptr<InterfaceManagerFields>, u32)),
        entry!(0x00719740, fn_00719740(Ptr<InterfaceManagerFields>, u8)),
        entry!(0x00719790, fn_00719790(Ptr, Ptr, Ptr, Ptr, u8)),
        entry!(0x00719ae0, fn_00719ae0(Ptr<InterfaceManagerFields>) -> bool),
        entry!(0x00719b00, fn_00719b00(Ptr) -> bool),
        entry!(
            0x00719b20,
            fn_00719b20(Ptr<BSSimpleArray>, u32, u32, u32) -> i32
        ),
        entry!(
            0x00719b80,
            fn_00719b80(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00719bb0,
            fn_00719bb0(Ptr<BSSimpleArray>, u32) -> Ptr<BSSimpleArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the pages of the globals the code reads mapped and
    /// the call log on.
    fn base_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_6000,
            0x0102_9000,
            0x0106_f000,
            0x011a_0000,
            0x011c_3000,
            0x011d_8000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.call_log = Some(vec![]);
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

    /// The addresses called, in order.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    /// A double that returns a fixed `eax`.
    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// `_ftol2_sse`: truncates its `f64` argument.
    fn ftol_double(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ret(value as i32 as u32)
        });
    }

    /// An object whose vtable has `slots` (byte offset, function address).
    fn object_with_vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x60);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The manager at `011d8a80`.
    fn manager(e: &mut Engine) -> Ptr<InterfaceManagerFields> {
        let manager = e.new_object::<InterfaceManagerFields>();
        e.set_global(MANAGER_GLOBAL, manager.addr());
        manager
    }

    /// A handler object whose vtable slots (byte offsets) are the doubles at
    /// `base + slot`.
    fn handler(e: &mut Engine, base: u32, slots: &[u32]) -> u32 {
        let table: Vec<(u32, u32)> = slots.iter().map(|s| (*s, base + s)).collect();
        object_with_vtable(e, &table)
    }

    /// Builds the global handler list from the handler objects, with doubles
    /// for the list helpers: the item word of a node is its first word and
    /// the next node is the word at `+4`.
    fn handler_list(e: &mut Engine, handlers: &[u32]) {
        let mut next = 0;
        for (i, handler) in handlers.iter().enumerate().rev() {
            let node = if i == 0 { HANDLER_LIST } else { e.mem.alloc(8) };
            e.mem.set_u32(node, *handler);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        e.register(LIST_ITEM_REF, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
    }

    #[test]
    fn fn_00717230_compares_the_truncated_width_with_the_limit() {
        let mut e = base_engine();
        ftol_double(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_i32(this + 0x14, 100);
        e.mem.set_u32(this + 0x18, 3);
        e.mem.set_u32(0x0101_6970, 0x7f7f_ffff);
        stub(&mut e, 0x005b_d5b0, 0x1234_0000);
        e.register(0x00a1_b020, |e, a| {
            e.mem.set_f32(a[1], 100.9);
            Ret::default()
        });
        assert!(e.call(0x0071_7230, &args![this, 0xabcd_u32]).bool());
        let call = logged(&e, 0x00a1_b020)[0].clone();
        assert_eq!(call[0], 0x1234_0000);
        assert_eq!(call[2..], [0xabcd, 3, 0x7f7f_ffff, 0]);

        e.register(0x00a1_b020, |e, a| {
            e.mem.set_f32(a[1], 101.2);
            Ret::default()
        });
        assert!(!e.call(0x0071_7230, &args![this, 0xabcd_u32]).bool());
    }

    #[test]
    fn play_menu_sound_looks_the_sound_up_by_name_and_plays_it() {
        let table = [
            (1, 0x0106_f380),
            (2, 0x0106_f360),
            (3, 0x0106_f38c),
            (4, 0x0106_f39c),
            (8, 0x0106_f370),
            (10, 0x0102_5cc4),
            (19, 0x0102_5cc4),
            (20, 0x0106_f360),
            (21, 0x0106_f354),
            (36, 0x0106_f060),
        ];
        for (sound_id, name) in table {
            let mut e = base_engine();
            nothing(
                &mut e,
                &[0x0041_a250, 0x0041_8900, 0x0048_3710, 0x00ad_8830],
            );
            stub(&mut e, 0x0045_3a70, 0x00a0_d100);
            e.register(0x00ad_7550, |_, a| ret(a[1]));
            e.call(0x0071_7280, &args![sound_id as u32]);
            let lookup = logged(&e, 0x00ad_7550);
            assert_eq!(lookup.len(), 1, "sound {sound_id}");
            assert_eq!(lookup[0][0], 0x00a0_d100);
            assert_eq!(lookup[0][2..], [name, 0x121]);
            // The found handle (the lookup's out parameter) is copied into
            // the played one, the temporary is destroyed, then it plays and
            // the played one is destroyed.
            let copy = logged(&e, 0x0041_8900)[0].clone();
            assert_eq!(copy[1], lookup[0][1]);
            let destroyed = logged(&e, 0x0048_3710);
            assert_eq!(destroyed, vec![vec![lookup[0][1]], vec![copy[0]]]);
            assert_eq!(logged(&e, 0x00ad_8830), vec![vec![copy[0], 0]]);
        }
    }

    #[test]
    fn play_menu_sound_plays_nothing_for_other_ids() {
        for sound_id in [-1, 0, 5, 9, 99] {
            let mut e = base_engine();
            nothing(&mut e, &[0x0041_a250, 0x0048_3710]);
            e.call(0x0071_7280, &args![sound_id as u32]);
            assert_eq!(logged(&e, 0x0041_a250).len(), 1);
            assert_eq!(logged(&e, 0x0048_3710).len(), 1);
            assert_eq!(order(&e).len(), 3, "sound {sound_id}");
        }
    }

    #[test]
    fn fn_00717600_preloads_once_and_keeps_the_menu_mode() {
        let mut e = base_engine();
        let this = manager(&mut e);
        e.set_global(OBJECT_011DEA3C, 0x1111u32);
        e.set_global(0x011c_3f2c, 0x2222u32);
        e.set(this, InterfaceManagerFields::bFirstChanceLoad, true);
        e.set(this, InterfaceManagerFields::bPreLoadMainMenus, true);
        e.set(this, InterfaceManagerFields::cMenuMode, 5);
        stub(&mut e, 0x0045_16b0, 0);
        for create in [
            0x0070_4be0,
            0x0070_48c0,
            0x0070_4490,
            0x0070_46c0,
            0x0070_4140,
        ] {
            stub(&mut e, create, 0);
        }
        // The pre-load changes the menu mode; it must be restored.
        e.register(0x0070_5b70, |e, _| {
            let manager = e.global::<u32>(MANAGER_GLOBAL);
            e.mem.set_u32(manager + 0x0c, 9);
            Ret::default()
        });
        nothing(&mut e, &[0x0070_3610]);
        e.call(0x0071_7600, &args![this]);
        assert_eq!(logged(&e, 0x0045_16b0), vec![vec![0x2222]]);
        assert_eq!(logged(&e, 0x0070_5b70).len(), 1);
        assert_eq!(e.get(this, InterfaceManagerFields::cMenuMode), 5);
        assert!(!e.get(this, InterfaceManagerFields::bFirstChanceLoad));
        assert!(!e.get(this, InterfaceManagerFields::bPreLoadMainMenus));
    }

    #[test]
    fn fn_00717600_does_nothing_without_the_object_the_flag_or_when_busy() {
        // No object at 011dea3c.
        let mut e = base_engine();
        let this = manager(&mut e);
        e.set(this, InterfaceManagerFields::bFirstChanceLoad, true);
        e.call(0x0071_7600, &args![this]);
        assert_eq!(order(&e).len(), 1);
        // Object, but the flag is clear.
        e.set_global(OBJECT_011DEA3C, 0x1111u32);
        e.set(this, InterfaceManagerFields::bFirstChanceLoad, false);
        e.call(0x0071_7600, &args![this]);
        assert_eq!(order(&e).len(), 2);
        // Object and flag, but 004516b0 is true.
        e.set(this, InterfaceManagerFields::bFirstChanceLoad, true);
        stub(&mut e, 0x0045_16b0, 1);
        e.call(0x0071_7600, &args![this]);
        assert_eq!(order(&e).len(), 4);
        assert!(e.get(this, InterfaceManagerFields::bFirstChanceLoad));
    }

    #[test]
    fn pre_load_main_menus_enables_each_created_menu() {
        let mut e = base_engine();
        let this = manager(&mut e);
        e.set(this, InterfaceManagerFields::bPreLoadMainMenus, true);
        stub(&mut e, 0x0070_4be0, 0x111);
        stub(&mut e, 0x0070_48c0, 0);
        stub(&mut e, 0x0070_4490, 0x222);
        stub(&mut e, 0x0070_46c0, 0);
        stub(&mut e, 0x0070_4140, 0x333);
        nothing(&mut e, &[TILE_SET_VALUE, 0x0070_5b70, 0x0070_3610]);
        e.call(0x0071_7660, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x0071_7660,
                0x0070_4be0,
                TILE_SET_VALUE,
                0x0070_48c0,
                0x0070_4490,
                TILE_SET_VALUE,
                0x0070_46c0,
                0x0070_4140,
                TILE_SET_VALUE,
                0x0070_5b70,
                0x0070_3610,
            ]
        );
        assert_eq!(
            logged(&e, TILE_SET_VALUE),
            vec![
                vec![0x111, 0xfa3, 0],
                vec![0x222, 0xfa3, 0],
                vec![0x333, 0xfa3, 0]
            ]
        );
        assert_eq!(logged(&e, 0x0070_5b70), vec![vec![0, 0, 4, 0x3eb]]);
        assert!(!e.get(this, InterfaceManagerFields::bPreLoadMainMenus));

        // With the flag clear nothing runs.
        e.call_log = Some(vec![]);
        e.call(0x0071_7660, &args![this]);
        assert_eq!(order(&e), vec![0x0071_7660]);
    }

    #[test]
    fn set_cursor_alpha_reaches_the_property_and_passes_the_alpha_as_float() {
        let mut e = base_engine();
        stub(&mut e, GET_MANAGER, 0x500);
        stub(&mut e, 0x0045_cd60, 0x600);
        stub(&mut e, TILE_NODE, 0x700);
        stub(&mut e, 0x0045_bc00, 0x800);
        stub(&mut e, 0x004a_f350, 0x900);
        stub(&mut e, 0x00a5_9d30, 0xa00);
        nothing(&mut e, &[0x0068_c9f0]);
        e.call(0x0071_7740, &args![5u32]);
        assert_eq!(logged(&e, 0x0045_cd60), vec![vec![0x500]]);
        assert_eq!(logged(&e, TILE_NODE), vec![vec![0x600]]);
        assert_eq!(logged(&e, 0x0045_bc00), vec![vec![0x700, 0]]);
        assert_eq!(logged(&e, 0x00a5_9d30), vec![vec![0x800, 0x900]]);
        assert_eq!(logged(&e, 0x0068_c9f0), vec![vec![0xa00, 5.0f32.to_bits()]]);
    }

    #[test]
    fn set_cursor_alpha_stops_at_the_first_missing_link() {
        // (object, node, child, property): the last call made.
        for (object, node, child, property, expected_last) in [
            (0, 0x700, 0x800, 0xa00, 0x0045_cd60),
            (0x600, 0, 0x800, 0xa00, TILE_NODE),
            (0x600, 0x700, 0, 0xa00, 0x0045_bc00),
            (0x600, 0x700, 0x800, 0, 0x00a5_9d30),
        ] {
            let mut e = base_engine();
            stub(&mut e, GET_MANAGER, 0x500);
            stub(&mut e, 0x0045_cd60, object);
            stub(&mut e, TILE_NODE, node);
            stub(&mut e, 0x0045_bc00, child);
            stub(&mut e, 0x004a_f350, 0x900);
            stub(&mut e, 0x00a5_9d30, property);
            nothing(&mut e, &[0x0068_c9f0]);
            e.call(0x0071_7740, &args![5u32]);
            assert_eq!(*order(&e).last().unwrap(), expected_last);
            assert!(logged(&e, 0x0068_c9f0).is_empty());
        }
    }

    #[test]
    fn fn_007177c0_scales_a_float_or_reads_an_integer_setting() {
        let mut e = base_engine();
        e.mem.set_f64(0x0102_90c0, 1.5);
        stub(&mut e, 0x004d_e080, 1);
        e.register(0x004d_e0e0, |_, _| ret_float(3.0));
        assert_eq!(e.call(0x0071_77c0, &args![]).f64(), 4.5);

        // Otherwise the setting named by 00707b40.
        let cell = e.mem.alloc(4);
        e.mem.set_i32(cell, -7);
        stub(&mut e, 0x004d_e080, 0);
        stub(&mut e, 0x0043_d4d0, cell);
        stub(&mut e, 0x0070_7b40, 1);
        assert_eq!(e.call(0x0071_77c0, &args![]).f64(), -7.0);
        assert_eq!(logged(&e, 0x0043_d4d0), vec![vec![0x011d_8bfc]]);
        stub(&mut e, 0x0070_7b40, 0);
        assert_eq!(e.call(0x0071_77c0, &args![]).f64(), -7.0);
        assert_eq!(logged(&e, 0x0043_d4d0)[1], vec![0x011d_8bd8]);
    }

    #[test]
    fn fn_00717820_reads_one_of_two_integer_settings() {
        let mut e = base_engine();
        let cell = e.mem.alloc(4);
        e.mem.set_i32(cell, 640);
        stub(&mut e, 0x0043_d4d0, cell);
        stub(&mut e, 0x0070_7b40, 1);
        assert_eq!(e.call(0x0071_7820, &args![]).f64(), 640.0);
        stub(&mut e, 0x0070_7b40, 0);
        assert_eq!(e.call(0x0071_7820, &args![]).f64(), 640.0);
        assert_eq!(
            logged(&e, 0x0043_d4d0),
            vec![vec![0x011d_8bc0], vec![0x011d_8bf0]]
        );
    }

    #[test]
    fn get_menu_height_divides_the_integer_by_the_float() {
        let mut e = base_engine();
        e.set_global(INTEGER_011A0190, 1000i32);
        e.register(0x0071_5da0, |_, _| ret_float(3.0));
        let height = e.call(0x0071_7860, &args![]).f32();
        assert_eq!(height, (1000.0f64 / 3.0) as f32);
        e.register(0x0071_5da0, |_, _| ret_float(0.5));
        assert_eq!(e.call(0x0071_7860, &args![]).f32(), 2000.0);
    }

    #[test]
    fn fn_00717890_returns_the_integer_global() {
        let mut e = base_engine();
        e.set_global(INTEGER_011A0190, -5i32);
        assert_eq!(e.call(0x0071_7890, &args![]).i32(), -5);
    }

    #[test]
    fn is_in_pipboy_menu_tries_the_five_ids_in_order() {
        let mut e = base_engine();
        e.register_double(IS_MENU_ID_VISIBLE, |_, a| ret(u32::from(a[0] == 0x425)));
        assert!(e.call(0x0071_78a0, &args![]).bool());
        let ids: Vec<u32> = logged(&e, IS_MENU_ID_VISIBLE)
            .iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(ids, vec![0x3eb, 0x3ea, 0x40b, 0x425]);
        assert!(logged(&e, IS_MENU_ID_VISIBLE).iter().all(|a| a[1] == 0));

        stub(&mut e, IS_MENU_ID_VISIBLE, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0071_78a0, &args![]).bool());
        assert_eq!(logged(&e, IS_MENU_ID_VISIBLE).len(), 5);
    }

    #[test]
    fn is_pipboy_menu_topmost_checks_the_enter_stack_top() {
        let mut e = base_engine();
        stub(&mut e, GET_STATE_OBJECT, 0x77);
        stub(&mut e, STATE_OBJECT_FLAG, 0);
        stub(&mut e, GET_MANAGER, 0x500);
        for (top, expected) in [
            (0x3eb, true),
            (0x3ff, true),
            (0x3ea, true),
            (0x40b, true),
            (0x425, true),
            (1, true),
            (0, false),
            (0x3ec, false),
        ] {
            stub(&mut e, GET_ENTER_STACK_TOP, top);
            assert_eq!(
                e.call(0x0071_7920, &args![]).bool(),
                expected,
                "top {top:x}"
            );
        }
        assert_eq!(logged(&e, GET_STATE_OBJECT)[0], vec![1]);
        assert_eq!(logged(&e, STATE_OBJECT_FLAG)[0], vec![0x77]);
        assert_eq!(logged(&e, GET_ENTER_STACK_TOP)[0], vec![0x500]);

        // A positive flag byte makes it false without asking the stack.
        stub(&mut e, STATE_OBJECT_FLAG, 1);
        stub(&mut e, GET_ENTER_STACK_TOP, 0x3eb);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0071_7920, &args![]).bool());
        assert!(logged(&e, GET_ENTER_STACK_TOP).is_empty());
    }

    #[test]
    fn is_current_rendered_menu_topmost_compares_nodes() {
        let mut e = base_engine();
        let this = manager(&mut e);
        stub(&mut e, GET_MANAGER, this.addr());
        stub(&mut e, IS_MENU_ID_VISIBLE, 0);
        stub(&mut e, GET_STATE_OBJECT, 0x77);
        stub(&mut e, STATE_OBJECT_FLAG, 0);
        stub(&mut e, GET_ENTER_STACK_TOP, 0x3f0);
        stub(&mut e, GET_MENU_BY_CLASS, 0x66);
        stub(&mut e, TILE_NODE, 0x1234);
        stub(&mut e, RENDERED_MENU_NODE, 0x1234);

        // No rendered menu.
        assert!(!e.call(0x0071_7990, &args![]).bool());
        assert!(logged(&e, IS_MENU_ID_VISIBLE).is_empty());

        e.set(
            this,
            InterfaceManagerFields::pCurrentRenderedMenu,
            Ptr::new(0x5555),
        );
        // Same node: topmost; the rendered menu is the argument.
        assert!(e.call(0x0071_7990, &args![]).bool());
        assert_eq!(logged(&e, GET_MENU_BY_CLASS), vec![vec![0x3f0]]);
        assert_eq!(logged(&e, RENDERED_MENU_NODE), vec![vec![0x5555]]);
        // A different node.
        stub(&mut e, RENDERED_MENU_NODE, 0x4321);
        assert!(!e.call(0x0071_7990, &args![]).bool());
        // No top menu: the node is not asked for.
        stub(&mut e, GET_MENU_BY_CLASS, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0071_7990, &args![]).bool());
        assert!(logged(&e, TILE_NODE).is_empty());
        // The node of the top menu is null.
        stub(&mut e, GET_MENU_BY_CLASS, 0x66);
        stub(&mut e, TILE_NODE, 0);
        assert!(!e.call(0x0071_7990, &args![]).bool());
        // The flag object forbids it.
        stub(&mut e, TILE_NODE, 0x1234);
        stub(&mut e, RENDERED_MENU_NODE, 0x1234);
        stub(&mut e, STATE_OBJECT_FLAG, 1);
        assert!(!e.call(0x0071_7990, &args![]).bool());
    }

    #[test]
    fn is_current_rendered_menu_topmost_in_a_pipboy_menu_defers() {
        let mut e = base_engine();
        let this = manager(&mut e);
        e.set(
            this,
            InterfaceManagerFields::pCurrentRenderedMenu,
            Ptr::new(0x5555),
        );
        stub(&mut e, GET_MANAGER, this.addr());
        stub(&mut e, IS_MENU_ID_VISIBLE, 1);
        stub(&mut e, GET_STATE_OBJECT, 0x77);
        stub(&mut e, STATE_OBJECT_FLAG, 0);
        stub(&mut e, GET_ENTER_STACK_TOP, 1);
        assert!(e.call(0x0071_7990, &args![]).bool());
        stub(&mut e, GET_ENTER_STACK_TOP, 0x3f0);
        assert!(!e.call(0x0071_7990, &args![]).bool());
        assert!(logged(&e, GET_MENU_BY_CLASS).is_empty());
    }

    /// The manager is ready and its `bShowMouse` byte is `show_mouse`.
    fn button_engine(show_mouse: bool) -> Engine {
        let mut e = base_engine();
        let this = manager(&mut e);
        e.set(this, InterfaceManagerFields::bShowMouse, show_mouse);
        stub(&mut e, GET_MANAGER, this.addr());
        stub(&mut e, MANAGER_READY, 1);
        e
    }

    #[test]
    fn check_menu_button_returns_one_when_not_ready_or_the_mouse_shows() {
        let mut e = button_engine(false);
        stub(&mut e, MANAGER_READY, 0);
        assert_eq!(e.call(0x0071_7a40, &args![1u32, 0u32]).i32(), 1);
        let mut e = button_engine(true);
        assert_eq!(e.call(0x0071_7a40, &args![1u32, 0u32]).i32(), 1);
    }

    #[test]
    fn check_menu_button_returns_minus_one_when_readiness_is_lost_in_between() {
        let mut e = button_engine(false);
        let mut calls = 0;
        e.register_double(MANAGER_READY, move |_, _| {
            calls += 1;
            ret(u32::from(calls == 1))
        });
        assert_eq!(e.call(0x0071_7a40, &args![1u32, 0u32]).i32(), -1);
    }

    #[test]
    fn check_menu_button_compares_old_and_new_button_masks() {
        let mut e = button_engine(false);
        for (button, mask) in [(1, 1u16), (2, 2), (3, 8), (4, 4), (5, 0x10), (6, 0x20)] {
            for (new, old, expected) in [
                (mask, mask, 0),
                (mask, 0, 2),
                (0, mask, 1),
                (0, 0, -1),
                (!mask, !mask, -1),
            ] {
                e.mem.set_u16(PAD_BUTTONS_NEW, new);
                e.mem.set_u16(PAD_BUTTONS_OLD, old);
                assert_eq!(
                    e.call(0x0071_7a40, &args![button as u32, 0u32]).i32(),
                    expected,
                    "button {button}"
                );
            }
        }
        for (button, mask) in [
            (9, 0x1000u16),
            (10, 0x2000),
            (11, 0x4000),
            (12, 0x8000),
            (15, 0x100),
            (16, 0x200),
            (17, 0x40),
            (18, 0x80),
        ] {
            e.mem.set_u16(PAD_BUTTONS_NEW, mask);
            e.mem.set_u16(PAD_BUTTONS_OLD, 0);
            assert_eq!(e.call(0x0071_7a40, &args![button as u32, 0u32]).i32(), 2);
        }
        // Buttons without a mask are never down.
        e.mem.set_u16(PAD_BUTTONS_NEW, 0xffff);
        e.mem.set_u16(PAD_BUTTONS_OLD, 0xffff);
        assert_eq!(e.call(0x0071_7a40, &args![7u32, 0u32]).i32(), -1);
        assert_eq!(e.call(0x0071_7a40, &args![0u32, 0u32]).i32(), -1);
        assert_eq!(e.call(0x0071_7a40, &args![19u32, 0u32]).i32(), -1);
    }

    #[test]
    fn check_menu_button_reads_the_sticks_when_analog() {
        let mut e = button_engine(false);
        // Button 1 (mask 1): vertical axes above the dead zone; button 2 below.
        e.mem.set_i16(PAD_AXIS_OLD_Y, 0x2000);
        e.mem.set_i16(PAD_AXIS_NEW_Y, 0x2000);
        assert_eq!(e.call(0x0071_7a40, &args![1u32, 1u32]).i32(), 0);
        e.mem.set_i16(PAD_AXIS_OLD_Y, 0x1000);
        assert_eq!(e.call(0x0071_7a40, &args![1u32, 1u32]).i32(), 2);
        e.mem.set_i16(PAD_AXIS_OLD_Y, 0x2000);
        e.mem.set_i16(PAD_AXIS_NEW_Y, 0x1ea9);
        assert_eq!(e.call(0x0071_7a40, &args![1u32, 1u32]).i32(), 1);
        // Below the dead zone for button 2.
        e.mem.set_i16(PAD_AXIS_OLD_Y, -0x1eaa);
        e.mem.set_i16(PAD_AXIS_NEW_Y, -0x1eaa);
        assert_eq!(e.call(0x0071_7a40, &args![2u32, 1u32]).i32(), 0);
        // Horizontal axes: button 4 (mask 4) is left, button 3 (mask 8) right.
        e.mem.set_i16(PAD_AXIS_OLD_X, -0x2000);
        e.mem.set_i16(PAD_AXIS_NEW_X, 0);
        assert_eq!(e.call(0x0071_7a40, &args![4u32, 1u32]).i32(), 1);
        e.mem.set_i16(PAD_AXIS_OLD_X, 0);
        e.mem.set_i16(PAD_AXIS_NEW_X, 0x2000);
        assert_eq!(e.call(0x0071_7a40, &args![3u32, 1u32]).i32(), 2);
        // Without the analog flag the sticks are ignored.
        assert_eq!(e.call(0x0071_7a40, &args![3u32, 0u32]).i32(), -1);
        // With the sticks quiet the buttons decide.
        e.mem.set_i16(PAD_AXIS_NEW_X, 0);
        e.mem.set_u16(PAD_BUTTONS_NEW, 8);
        assert_eq!(e.call(0x0071_7a40, &args![3u32, 1u32]).i32(), 2);
        // Buttons 5 and 6 have no stick direction.
        e.mem.set_i16(PAD_AXIS_NEW_Y, 0x7000);
        e.mem.set_u16(PAD_BUTTONS_NEW, 0);
        assert_eq!(e.call(0x0071_7a40, &args![5u32, 1u32]).i32(), -1);
    }

    #[test]
    fn check_menu_button_reads_the_trigger_bytes_for_13_and_14() {
        let mut e = button_engine(false);
        for (button, new_addr, old_addr) in [
            (13, PAD_TRIGGER_NEW_LEFT, PAD_TRIGGER_OLD_LEFT),
            (14, PAD_TRIGGER_NEW_RIGHT, PAD_TRIGGER_OLD_RIGHT),
        ] {
            for (new, old, expected) in [(1, 1, 0), (1, 0, 2), (0, 1, 1), (0, 0, -1)] {
                e.mem.set_u8(new_addr, new);
                e.mem.set_u8(old_addr, old);
                assert_eq!(
                    e.call(0x0071_7a40, &args![button as u32, 0u32]).i32(),
                    expected
                );
            }
        }
    }

    #[test]
    fn clear_menu_button_forwards_the_mapped_button() {
        let mut e = button_engine(false);
        e.set_global(OBJECT_011DEA0C, 0x9999u32);
        stub(&mut e, 0x0087_7720, 0xaa);
        nothing(&mut e, &[0x00a2_5560]);
        e.call(0x0071_7de0, &args![3u32]);
        assert_eq!(logged(&e, 0x0087_7720), vec![vec![0x9999]]);
        assert_eq!(logged(&e, 0x00a2_5560), vec![vec![0xaa, 4]]);
        // An unmapped button is passed as -1.
        e.call(0x0071_7de0, &args![7u32]);
        assert_eq!(logged(&e, 0x00a2_5560)[1], vec![0xaa, u32::MAX]);

        // Not ready, or the mouse shows: nothing happens.
        stub(&mut e, MANAGER_READY, 0);
        e.call_log = Some(vec![]);
        e.call(0x0071_7de0, &args![3u32]);
        assert_eq!(order(&e).len(), 2);
        let mut e = button_engine(true);
        e.call(0x0071_7de0, &args![3u32]);
        assert!(logged(&e, 0x00a2_5560).is_empty());
    }

    #[test]
    fn do_enter_stops_at_the_first_accepting_handler() {
        let mut e = base_engine();
        let first = handler(&mut e, 0x7000_0000, &[0x00]);
        let second = handler(&mut e, 0x7100_0000, &[0x00]);
        let third = handler(&mut e, 0x7200_0000, &[0x00]);
        handler_list(&mut e, &[first, second, third]);
        stub(&mut e, 0x7000_0000, 0);
        stub(&mut e, 0x7100_0000, 1);
        stub(&mut e, 0x7200_0000, 1);
        let target = handler(&mut e, 0x7300_0000, &[0x10]);
        stub(&mut e, 0x7300_0010, 0xbeef);
        let result = e.call(0x0071_7e70, &args![0x1000u32, target, 7u32, 9u32]);
        assert_eq!(result.u32(), 0xbeef);
        assert_eq!(logged(&e, 0x7000_0000), vec![vec![first, 9]]);
        assert_eq!(logged(&e, 0x7100_0000), vec![vec![second, 9]]);
        assert!(logged(&e, 0x7200_0000).is_empty());
        assert_eq!(logged(&e, 0x7300_0010), vec![vec![target, 7, 9]]);
    }

    #[test]
    fn do_enter_ends_at_the_list_end_or_an_empty_item() {
        let mut e = base_engine();
        let only = handler(&mut e, 0x7000_0000, &[0x00]);
        handler_list(&mut e, &[only]);
        stub(&mut e, 0x7000_0000, 0);
        let target = handler(&mut e, 0x7300_0000, &[0x10]);
        stub(&mut e, 0x7300_0010, 1);
        e.call(0x0071_7e70, &args![0u32, target, 0u32, 0u32]);
        assert_eq!(logged(&e, 0x7000_0000).len(), 1);
        assert_eq!(logged(&e, 0x7300_0010).len(), 1);
        // An empty head item stops the walk before any handler runs.
        e.mem.set_u32(HANDLER_LIST, 0);
        e.call(0x0071_7e70, &args![0u32, target, 0u32, 0u32]);
        assert_eq!(logged(&e, 0x7000_0000).len(), 1);
        assert_eq!(logged(&e, 0x7300_0010).len(), 2);
    }

    #[test]
    fn do_leave_needs_a_matching_id_and_an_accepting_handler() {
        let mut e = base_engine();
        let first = handler(&mut e, 0x7000_0000, &[0x00, 0x04]);
        let second = handler(&mut e, 0x7100_0000, &[0x00, 0x04]);
        let third = handler(&mut e, 0x7200_0000, &[0x00, 0x04]);
        handler_list(&mut e, &[first, second, third]);
        // first: id differs; second: id matches but declines; third: both.
        stub(&mut e, 0x7000_0004, 5);
        stub(&mut e, 0x7000_0000, 1);
        stub(&mut e, 0x7100_0004, 6);
        stub(&mut e, 0x7100_0000, 0);
        stub(&mut e, 0x7200_0004, 6);
        stub(&mut e, 0x7200_0000, 1);
        let target = handler(&mut e, 0x7300_0000, &[0x14]);
        stub(&mut e, 0x7300_0014, 0x77);
        assert_eq!(
            e.call(0x0071_7ef0, &args![0u32, target, 3u32, 6u32]).u32(),
            0x77
        );
        assert!(logged(&e, 0x7000_0000).is_empty());
        assert_eq!(logged(&e, 0x7100_0000), vec![vec![second, 0]]);
        assert_eq!(logged(&e, 0x7200_0000), vec![vec![third, 0]]);
        assert_eq!(logged(&e, 0x7300_0014), vec![vec![target, 3, 6]]);
    }

    #[test]
    fn do_gamepad_lets_the_target_take_the_event_first() {
        let mut e = base_engine();
        let target = handler(&mut e, 0x7300_0000, &[0x38]);
        stub(&mut e, 0x7300_0038, 1);
        assert!(e
            .call(0x0071_7f80, &args![0x1000u32, target, 4u32, 0.5f32])
            .bool());
        assert_eq!(
            logged(&e, 0x7300_0038),
            vec![vec![target, 4, 0.5f32.to_bits()]]
        );
        assert_eq!(order(&e).len(), 2);
    }

    #[test]
    fn do_gamepad_activates_the_tile_of_an_accepting_handler() {
        let mut e = base_engine();
        let skipped = handler(&mut e, 0x7000_0000, &[0x0c]);
        let taker = handler(&mut e, 0x7100_0000, &[0x0c, 0x04, 0x08, 0x10]);
        handler_list(&mut e, &[skipped, taker]);
        stub(&mut e, 0x7000_000c, 0);
        stub(&mut e, 0x7100_000c, 1);
        stub(&mut e, 0x7100_0004, 1);
        stub(&mut e, 0x7100_0008, 0x4444);
        nothing(&mut e, &[0x7100_0010, 0x0071_5860]);
        let target = handler(&mut e, 0x7300_0000, &[0x38, 0x34]);
        stub(&mut e, 0x7300_0038, 0);
        stub(&mut e, 0x7300_0034, 0x3eb);
        stub(&mut e, IS_TOP_MENU_ID, 1);
        assert!(e
            .call(0x0071_7f80, &args![0x1000u32, target, 4u32, 0.5f32])
            .bool());
        assert_eq!(logged(&e, 0x7100_0008), vec![vec![taker, 4]]);
        assert_eq!(logged(&e, IS_TOP_MENU_ID), vec![vec![0x3eb]]);
        assert_eq!(
            logged(&e, 0x0071_5860),
            vec![vec![0x1000, 0x4444, 0xfc3, 1]]
        );
        assert_eq!(logged(&e, 0x7100_0010), vec![vec![taker]]);
    }

    #[test]
    fn do_gamepad_is_false_without_a_taker() {
        // Each case fails one of the four conditions of the only handler.
        for case in 0..4 {
            let mut e = base_engine();
            let only = handler(&mut e, 0x7100_0000, &[0x0c, 0x04, 0x08, 0x10]);
            handler_list(&mut e, &[only]);
            stub(&mut e, 0x7100_000c, u32::from(case != 0));
            stub(&mut e, 0x7100_0004, u32::from(case != 2));
            stub(&mut e, 0x7100_0008, if case == 3 { 0 } else { 0x4444 });
            nothing(&mut e, &[0x7100_0010, 0x0071_5860]);
            let target = handler(&mut e, 0x7300_0000, &[0x38, 0x34]);
            stub(&mut e, 0x7300_0038, 0);
            stub(&mut e, 0x7300_0034, 0x3eb);
            stub(&mut e, IS_TOP_MENU_ID, u32::from(case != 1));
            assert!(!e
                .call(0x0071_7f80, &args![0u32, target, 4u32, 0.5f32])
                .bool());
            assert!(logged(&e, 0x7100_0010).is_empty(), "case {case}");
            assert!(logged(&e, 0x0071_5860).is_empty());
        }
        // No handler at all.
        let mut e = base_engine();
        handler_list(&mut e, &[0]);
        let target = handler(&mut e, 0x7300_0000, &[0x38]);
        stub(&mut e, 0x7300_0038, 0);
        assert!(!e
            .call(0x0071_7f80, &args![0u32, target, 4u32, 0.5f32])
            .bool());
    }

    /// The wheel move set-up: the wheel float 10.0 over a divisor of 2.0 is
    /// position 5.0; one handler with two children (0x10 and 0x20) whose
    /// starts and widths are given; the Pipboy test answers `pipboy`.
    fn wheel_engine(starts: [f32; 2], widths: [f32; 2], pipboy: bool) -> (Engine, u32, u32, u32) {
        let mut e = base_engine();
        let this = manager(&mut e);
        e.set(this, InterfaceManagerFields::fMouseWheel, 10.0);
        e.register(0x0070_ecb0, |_, _| ret_float(2.0));
        stub(&mut e, GET_STATE_OBJECT, 0x77);
        stub(&mut e, STATE_OBJECT_FLAG, 0);
        stub(&mut e, GET_MANAGER, this.addr());
        stub(&mut e, GET_ENTER_STACK_TOP, if pipboy { 1 } else { 0x500 });
        e.set_global(INTEGER_011A0190, 1080i32);
        e.register(0x0071_5da0, |_, _| ret_float(2.0));
        e.mem.set_f64(0x0106_f3a8, 0.5);
        let item = handler(&mut e, 0x7000_0000, &[0x00, 0x04, 0x14]);
        handler_list(&mut e, &[item]);
        stub(&mut e, 0x7000_0004, 1);
        nothing(&mut e, &[0x7000_0000]);
        e.register_double(0x7000_0014, |_, a| {
            ret(match a[1] {
                0 => 0x10,
                1 => 0x20,
                _ => 0,
            })
        });
        e.register_double(0x00a0_1440, move |_, a| {
            ret_float(f64::from(starts[usize::from(a[0] == 0x20)]))
        });
        e.register_double(TILE_GET_VALUE, move |_, a| {
            assert_eq!(a[1], 0xfb0);
            ret_float(f64::from(widths[usize::from(a[0] == 0x20)]))
        });
        let target = handler(&mut e, 0x7300_0000, &[0x28]);
        stub(&mut e, 0x7300_0028, 0x99);
        (e, this.addr(), item, target)
    }

    #[test]
    fn do_wheel_move_picks_the_child_that_contains_the_position() {
        let (mut e, this, item, target) = wheel_engine([0.0, 4.0], [1.0, 2.0], false);
        let result = e.call(0x0071_8080, &args![this, target, 11u32, 12u32]);
        assert_eq!(result.u32(), 0x99);
        assert_eq!(logged(&e, 0x7000_0000), vec![vec![item, 0x20]]);
        assert_eq!(logged(&e, 0x7300_0028), vec![vec![target, 11, 12]]);
        // The position 5.0 fits the second child up to its end (4 + 1).
        let (mut e, this, item, target) = wheel_engine([0.0, 4.0], [1.0, 1.0], false);
        e.call(0x0071_8080, &args![this, target, 0u32, 0u32]);
        assert_eq!(logged(&e, 0x7000_0000), vec![vec![item, 0x20]]);
        // And fails just past it.
        let (mut e, this, item, target) = wheel_engine([0.0, 4.0], [1.0, 0.5], false);
        e.call(0x0071_8080, &args![this, target, 0u32, 0u32]);
        assert_eq!(logged(&e, 0x7000_0000), vec![vec![item, 0]]);
    }

    #[test]
    fn do_wheel_move_scales_by_the_menu_height_in_a_pipboy_menu() {
        // Position 10 / 2 = 5, then 5 / (1080 / 2) - 0.5 = -0.4907...
        let (mut e, this, item, target) = wheel_engine([-1.0, 4.0], [1.0, 2.0], true);
        e.call(0x0071_8080, &args![this, target, 0u32, 0u32]);
        assert_eq!(logged(&e, 0x7000_0000), vec![vec![item, 0x10]]);
        // Not contained in either child: none is passed.
        let (mut e, this, item, target) = wheel_engine([0.0, 4.0], [1.0, 2.0], true);
        e.call(0x0071_8080, &args![this, target, 0u32, 0u32]);
        assert_eq!(logged(&e, 0x7000_0000), vec![vec![item, 0]]);
    }

    #[test]
    fn do_wheel_move_skips_handlers_with_a_zero_slot_4() {
        let (mut e, this, _, target) = wheel_engine([0.0, 4.0], [1.0, 2.0], false);
        stub(&mut e, 0x7000_0004, 0);
        e.call(0x0071_8080, &args![this, target, 0u32, 0u32]);
        assert!(logged(&e, 0x7000_0000).is_empty());
        assert_eq!(logged(&e, 0x7300_0028).len(), 1);
    }

    #[test]
    fn enter_rendered_menu_makes_the_menu_current() {
        for (accepted, expect_prepare) in [(0, true), (1, false)] {
            let mut e = base_engine();
            let this = manager(&mut e);
            let menu = handler(&mut e, 0x7400_0000, &[0x24, 0x30]);
            nothing(&mut e, &[0x7400_0024, 0x7400_0030]);
            stub(&mut e, 0x0097_4d90, accepted);
            stub(&mut e, 0x0096_11e0, 0x4321);
            nothing(&mut e, &[TILE_SET_VALUE]);
            e.call(0x0071_81d0, &args![this, menu]);
            assert_eq!(logged(&e, 0x7400_0024).len(), usize::from(expect_prepare));
            assert_eq!(logged(&e, 0x7400_0030), vec![vec![menu]]);
            assert_eq!(logged(&e, TILE_SET_VALUE), vec![vec![0x4321, 0xfa3, 0]]);
            assert!(e.get(this, InterfaceManagerFields::bIsInRenderedMenu));
            assert_eq!(
                e.get(this, InterfaceManagerFields::pCurrentRenderedMenu)
                    .addr(),
                menu
            );
        }
        // A menu without a tile has no trait set.
        let mut e = base_engine();
        let this = manager(&mut e);
        let menu = handler(&mut e, 0x7400_0000, &[0x24, 0x30]);
        nothing(&mut e, &[0x7400_0024, 0x7400_0030]);
        stub(&mut e, 0x0097_4d90, 1);
        stub(&mut e, 0x0096_11e0, 0);
        e.call(0x0071_81d0, &args![this, menu]);
        assert!(logged(&e, TILE_SET_VALUE).is_empty());
    }

    #[test]
    fn enter_rendered_menu_without_a_menu_hides_the_current_one() {
        // The menu `00704370` names stays; any other is queued.
        for queued in [false, true] {
            let mut e = base_engine();
            let this = manager(&mut e);
            let current = handler(&mut e, 0x7400_0000, &[0x34]);
            e.set(this, InterfaceManagerFields::bIsInRenderedMenu, true);
            e.set(
                this,
                InterfaceManagerFields::pCurrentRenderedMenu,
                Ptr::new(current),
            );
            nothing(
                &mut e,
                &[
                    0x007f_ae70,
                    0x7400_0034,
                    0x0045_38a0,
                    0x004e_d8c0,
                    0x0045_38c0,
                ],
            );
            stub(&mut e, 0x0070_4370, if queued { 0x6666 } else { current });
            e.call(0x0071_81d0, &args![this, 0u32]);
            assert_eq!(logged(&e, 0x007f_ae70), vec![vec![current, 0]]);
            assert_eq!(logged(&e, 0x7400_0034), vec![vec![current]]);
            assert_eq!(logged(&e, 0x0070_4370), vec![vec![this.addr()]]);
            assert_eq!(logged(&e, 0x004e_d8c0).len(), usize::from(queued));
            assert!(!e.get(this, InterfaceManagerFields::bIsInRenderedMenu));
            assert!(e
                .get(this, InterfaceManagerFields::pCurrentRenderedMenu)
                .is_null());
        }
    }

    #[test]
    fn enter_rendered_menu_without_any_menu_only_clears_the_flag() {
        let mut e = base_engine();
        let this = manager(&mut e);
        e.set(this, InterfaceManagerFields::bIsInRenderedMenu, true);
        e.call(0x0071_81d0, &args![this, 0u32]);
        assert!(!e.get(this, InterfaceManagerFields::bIsInRenderedMenu));
        assert_eq!(order(&e).len(), 1);
    }

    #[test]
    fn fn_007182b0_appends_the_item_under_the_lock() {
        let mut e = base_engine();
        nothing(&mut e, &[0x0045_38a0, 0x0045_38c0]);
        e.register(0x004e_d8c0, |e, a| {
            // The item is passed by address.
            assert_eq!(e.mem.u32(a[1]), 0x4242);
            Ret::default()
        });
        e.call(0x0071_82b0, &args![0x4242u32]);
        assert_eq!(
            order(&e),
            vec![0x0071_82b0, 0x0045_38a0, 0x004e_d8c0, 0x0045_38c0]
        );
        assert_eq!(logged(&e, 0x0045_38a0), vec![vec![0x011d_8c18, 0]]);
        assert_eq!(logged(&e, 0x004e_d8c0)[0][0], 0x011d_8b2c);
        assert_eq!(logged(&e, 0x0045_38c0), vec![vec![0x011d_8c18]]);
    }

    /// A tutorial manager in a game whose menu mode is 2, with doubles for
    /// the callees: the time is 1000, every menu is the top one and faded in,
    /// every form casts to a message that shows.
    fn tutorial_engine() -> (Engine, Ptr<TutorialManager>) {
        let mut e = base_engine();
        let manager = manager(&mut e);
        e.set(manager, InterfaceManagerFields::cMenuMode, 2);
        let this = e.new_object::<TutorialManager>();
        stub(&mut e, IS_IN_SPECIAL_MODE, 0);
        stub(&mut e, IS_FLAG_011D8950_SET, 0);
        stub(&mut e, ALWAYS_TRUE, 1);
        stub(&mut e, GET_TIME, 1000);
        stub(&mut e, IS_TOP_MENU_ID, 1);
        stub(&mut e, IS_TOP_MENU_FADED_IN, 1);
        stub(&mut e, LOOKUP_FORM_BY_ID, 0xf0f0);
        stub(&mut e, DYNAMIC_CAST, 0xabcd);
        stub(&mut e, 0x007e_8890, 1);
        nothing(&mut e, &[0x005b_5e40]);
        e.set(this, TutorialManager::eDisplayID, TUTORIAL_MESSAGE_COUNT);
        (e, this)
    }

    fn word(e: &Engine, this: Ptr<TutorialManager>, index: i32) -> u32 {
        message_word(e, this, index)
    }

    fn set_word(e: &mut Engine, this: Ptr<TutorialManager>, index: i32, value: u32) {
        set_message_word(e, this, index, value);
    }

    #[test]
    fn tutorial_update_does_nothing_in_the_special_modes() {
        let (mut e, this) = tutorial_engine();
        stub(&mut e, IS_IN_SPECIAL_MODE, 1);
        e.set(this, TutorialManager::eDisplayID, 3);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(order(&e), vec![0x0071_82e0, IS_IN_SPECIAL_MODE]);
        stub(&mut e, IS_IN_SPECIAL_MODE, 0);
        stub(&mut e, IS_FLAG_011D8950_SET, 1);
        e.call_log = Some(vec![]);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(
            order(&e),
            vec![0x0071_82e0, IS_IN_SPECIAL_MODE, IS_FLAG_011D8950_SET]
        );
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 3);
    }

    #[test]
    fn tutorial_update_resets_the_display_outside_menu_mode_2() {
        let (mut e, this) = tutorial_engine();
        let manager = e.global::<u32>(MANAGER_GLOBAL);
        e.mem.set_u32(manager + 0x0c, 1);
        e.set(this, TutorialManager::eDisplayID, 3);
        e.set(this, TutorialManager::uiNextDisplay, 500);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 0x29);
        assert_eq!(e.get(this, TutorialManager::uiNextDisplay), 0);
        assert!(logged(&e, GET_TIME).is_empty());
    }

    #[test]
    fn tutorial_update_shows_the_due_message_and_marks_it() {
        let (mut e, this) = tutorial_engine();
        set_word(&mut e, this, 3, (2 << 2) | 1);
        e.set(this, TutorialManager::eDisplayID, 3);
        e.set(this, TutorialManager::uiNextDisplay, 500);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(logged(&e, IS_TOP_MENU_ID), vec![vec![2 + 0x3e9]]);
        assert_eq!(logged(&e, LOOKUP_FORM_BY_ID), vec![vec![3 + 0x168]]);
        assert_eq!(
            logged(&e, DYNAMIC_CAST),
            vec![vec![0xf0f0, 0, 0x0118_3028, 0x0118_6130, 0]]
        );
        assert_eq!(logged(&e, 0x007e_8890), vec![vec![0xabcd, 0]]);
        // Bit 0 cleared, bit 1 set; nothing reported; display reset.
        assert_eq!(word(&e, this, 3), (2 << 2) | 2);
        assert!(logged(&e, 0x005b_5e40).is_empty());
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 0x29);
        assert_eq!(e.get(this, TutorialManager::uiNextDisplay), 0);
    }

    #[test]
    fn tutorial_update_reports_a_message_that_cannot_be_shown() {
        // The message refuses to show.
        let (mut e, this) = tutorial_engine();
        stub(&mut e, 0x007e_8890, 0);
        set_word(&mut e, this, 3, 1);
        e.set(this, TutorialManager::eDisplayID, 3);
        e.set(this, TutorialManager::uiNextDisplay, 500);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(logged(&e, 0x005b_5e40), vec![vec![0x0106_f3b0]]);
        assert_eq!(word(&e, this, 3), 1);
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 0x29);
        // The form is not a message: it is not even shown.
        let (mut e, this) = tutorial_engine();
        stub(&mut e, DYNAMIC_CAST, 0);
        e.set(this, TutorialManager::eDisplayID, 3);
        e.set(this, TutorialManager::uiNextDisplay, 500);
        e.call(0x0071_82e0, &args![this]);
        assert!(logged(&e, 0x007e_8890).is_empty());
        assert_eq!(logged(&e, 0x005b_5e40).len(), 1);
    }

    #[test]
    fn tutorial_update_waits_for_the_due_time_and_the_menu() {
        // Not yet due (time 1000 < 1500): nothing is shown or changed.
        let (mut e, this) = tutorial_engine();
        e.set(this, TutorialManager::eDisplayID, 3);
        e.set(this, TutorialManager::uiNextDisplay, 1500);
        e.call(0x0071_82e0, &args![this]);
        assert!(logged(&e, LOOKUP_FORM_BY_ID).is_empty());
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 3);
        assert_eq!(e.get(this, TutorialManager::uiNextDisplay), 1500);
        // Due, but its menu is not the top one: not shown.
        let (mut e, this) = tutorial_engine();
        set_word(&mut e, this, 3, 2 << 2);
        stub(&mut e, IS_TOP_MENU_ID, 0);
        e.set(this, TutorialManager::eDisplayID, 3);
        e.set(this, TutorialManager::uiNextDisplay, 500);
        e.call(0x0071_82e0, &args![this]);
        assert!(logged(&e, LOOKUP_FORM_BY_ID).is_empty());
        // Due, but the top menu is not faded in: not shown.
        let (mut e, this) = tutorial_engine();
        stub(&mut e, IS_TOP_MENU_FADED_IN, 0);
        e.set(this, TutorialManager::eDisplayID, 3);
        e.set(this, TutorialManager::uiNextDisplay, 500);
        e.call(0x0071_82e0, &args![this]);
        assert!(logged(&e, LOOKUP_FORM_BY_ID).is_empty());
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 3);
    }

    #[test]
    fn tutorial_update_queues_the_pending_messages_in_turn() {
        let (mut e, this) = tutorial_engine();
        set_word(&mut e, this, 7, (300 << 8) | 1);
        set_word(&mut e, this, 9, (50 << 8) | 1);
        e.call(0x0071_82e0, &args![this]);
        // Message 7 is found first (due at 1300), then 9 replaces it
        // (due at 1050) and clears the first one's bit 0.
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 9);
        assert_eq!(e.get(this, TutorialManager::uiNextDisplay), 1050);
        assert_eq!(word(&e, this, 7), 300 << 8);
        assert_eq!(word(&e, this, 9), (50 << 8) | 1);
    }

    #[test]
    fn tutorial_update_shows_a_message_without_delay_at_once() {
        let (mut e, this) = tutorial_engine();
        set_word(&mut e, this, 5, 1);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(logged(&e, LOOKUP_FORM_BY_ID), vec![vec![5 + 0x168]]);
        assert_eq!(word(&e, this, 5), 2);
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 0x29);
        assert_eq!(e.get(this, TutorialManager::uiNextDisplay), 0);
    }

    #[test]
    fn tutorial_update_skips_messages_that_cannot_start() {
        let (mut e, this) = tutorial_engine();
        // Bit 0 clear, bit 1 set, and the current one: all skipped.
        set_word(&mut e, this, 1, 0);
        set_word(&mut e, this, 2, 3);
        set_word(&mut e, this, 4, 1 | (400 << 8));
        e.set(this, TutorialManager::eDisplayID, 4);
        e.set(this, TutorialManager::uiNextDisplay, 5000);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 4);
        assert_eq!(e.get(this, TutorialManager::uiNextDisplay), 5000);
        assert!(logged(&e, IS_TOP_MENU_FADED_IN).is_empty());

        // With a menu on the current message, the candidate's menu must be
        // the top one.
        set_word(&mut e, this, 4, 1 | (1 << 2));
        set_word(&mut e, this, 6, 1 | (3 << 2) | (100 << 8));
        stub(&mut e, IS_TOP_MENU_ID, 0);
        e.call_log = Some(vec![]);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(logged(&e, IS_TOP_MENU_ID), vec![vec![3 + 0x3e9]]);
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 4);
        // And it must be faded in.
        stub(&mut e, IS_TOP_MENU_ID, 1);
        stub(&mut e, IS_TOP_MENU_FADED_IN, 0);
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 4);
        // When both hold, the current message keeps bit 0 only if its own
        // menu is not the top one.
        stub(&mut e, IS_TOP_MENU_FADED_IN, 1);
        e.register_double(IS_TOP_MENU_ID, |_, a| ret(u32::from(a[0] == 3 + 0x3e9)));
        e.call(0x0071_82e0, &args![this]);
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 6);
        assert_eq!(word(&e, this, 4), 1 << 2 | 1);
    }

    #[test]
    fn fn_007185e0_marks_a_message_in_range() {
        let (mut e, this) = tutorial_engine();
        set_word(&mut e, this, 0, 0xff);
        set_word(&mut e, this, 0x28, 1);
        assert!(e.call(0x0071_85e0, &args![this, 0u32]).bool());
        assert!(e.call(0x0071_85e0, &args![this, 0x28u32]).bool());
        assert_eq!(word(&e, this, 0), 0xfe | 2);
        assert_eq!(word(&e, this, 0x28), 2);
        assert!(!e.call(0x0071_85e0, &args![this, 0x29u32]).bool());
        assert!(!e.call(0x0071_85e0, &args![this, u32::MAX]).bool());
    }

    #[test]
    fn show_message_stores_the_delay_menu_and_whether_it_can_show() {
        let (mut e, this) = tutorial_engine();
        stub(&mut e, 0x0059_0ed0, 1);
        set_word(&mut e, this, 4, 0x80);
        let shown = e.call(0x0071_8630, &args![this, 4u32, 0x3eau32, 0x0112_3456u32]);
        assert!(shown.bool());
        // The delay keeps 24 bits, the menu is stored relative to 0x3e9, and
        // bit 0 says the message is pending.
        assert_eq!(word(&e, this, 4), 0x1234_5600 | (1 << 2) | 1);
        assert_eq!(logged(&e, LOOKUP_FORM_BY_ID), vec![vec![4 + 0x168]]);
        assert_eq!(
            logged(&e, DYNAMIC_CAST),
            vec![vec![0xf0f0, 0, 0x0118_3028, 0x0118_6130, 0]]
        );
        assert_eq!(logged(&e, 0x0059_0ed0), vec![vec![0xabcd]]);
    }

    #[test]
    fn show_message_with_no_menu_clears_the_menu_field() {
        let (mut e, this) = tutorial_engine();
        stub(&mut e, 0x0059_0ed0, 1);
        set_word(&mut e, this, 4, 0x3c | 0x80);
        assert!(e.call(0x0071_8630, &args![this, 4u32, 0u32, 7u32]).bool());
        assert_eq!(word(&e, this, 4), (7 << 8) | 1);
    }

    #[test]
    fn show_message_fails_for_bad_indexes_menus_and_casts() {
        let (mut e, this) = tutorial_engine();
        stub(&mut e, 0x0059_0ed0, 1);
        // Index out of range: nothing is written.
        assert!(!e
            .call(0x0071_8630, &args![this, 0x29u32, 0u32, 7u32])
            .bool());
        assert!(!e
            .call(0x0071_8630, &args![this, u32::MAX, 0u32, 7u32])
            .bool());
        assert_eq!(word(&e, this, 0x28), 0);
        // A menu id out of range: the delay is stored, the bit stays clear.
        set_word(&mut e, this, 2, 1);
        assert!(!e
            .call(0x0071_8630, &args![this, 2u32, 0x3e8u32, 7u32])
            .bool());
        assert_eq!(word(&e, this, 2), 7 << 8);
        set_word(&mut e, this, 2, 1);
        assert!(!e
            .call(0x0071_8630, &args![this, 2u32, 0x43du32, 7u32])
            .bool());
        assert_eq!(word(&e, this, 2), 7 << 8);
        assert!(logged(&e, LOOKUP_FORM_BY_ID).is_empty());
        // The last valid menu id.
        assert!(e
            .call(0x0071_8630, &args![this, 2u32, 0x43cu32, 7u32])
            .bool());
        assert_eq!(word(&e, this, 2), (7 << 8) | (0x13 << 2) | 1);
        // No form, no message, a refusing message and an already shown one.
        stub(&mut e, LOOKUP_FORM_BY_ID, 0);
        assert!(!e.call(0x0071_8630, &args![this, 2u32, 0u32, 7u32]).bool());
        stub(&mut e, LOOKUP_FORM_BY_ID, 1);
        stub(&mut e, DYNAMIC_CAST, 0);
        assert!(!e.call(0x0071_8630, &args![this, 2u32, 0u32, 7u32]).bool());
        stub(&mut e, DYNAMIC_CAST, 1);
        stub(&mut e, 0x0059_0ed0, 0);
        assert!(!e.call(0x0071_8630, &args![this, 2u32, 0u32, 7u32]).bool());
        stub(&mut e, 0x0059_0ed0, 1);
        set_word(&mut e, this, 2, 2);
        assert!(!e.call(0x0071_8630, &args![this, 2u32, 0u32, 7u32]).bool());
        assert_eq!(word(&e, this, 2), (7 << 8) | 2);
    }

    #[test]
    fn show_message_needs_the_always_true_test_too() {
        let (mut e, this) = tutorial_engine();
        stub(&mut e, 0x0059_0ed0, 1);
        stub(&mut e, ALWAYS_TRUE, 0);
        assert!(!e.call(0x0071_8630, &args![this, 2u32, 0u32, 7u32]).bool());
        assert_eq!(word(&e, this, 2), 7 << 8);
    }

    #[test]
    fn message_flags_are_packed_into_bytes_and_back() {
        let (mut e, this) = tutorial_engine();
        stub(&mut e, 0x0070_6810, 6);
        for index in [0, 3, 8, 40] {
            set_word(&mut e, this, index, 2);
        }
        set_word(&mut e, this, 1, 1);
        let out = e.mem.alloc(8);
        e.call(0x0071_87a0, &args![this, out]);
        let bytes = e.mem.bytes(out, 6);
        assert_eq!(bytes, vec![0x09, 0x01, 0, 0, 0, 0x01]);

        // Unpacking sets bit 1 from the bytes and keeps the other bits.
        let (mut e, this) = tutorial_engine();
        stub(&mut e, 0x0070_6810, 6);
        set_word(&mut e, this, 1, 3);
        set_word(&mut e, this, 2, 0x81);
        let input = e.mem.alloc(8);
        for (i, byte) in [0x09u8, 0x01, 0, 0, 0, 0x01].iter().enumerate() {
            e.mem.set_u8(input + i as u32, *byte);
        }
        e.call(0x0071_8890, &args![this, input]);
        for index in 0..0x29 {
            let expected = [0, 3, 8, 40].contains(&index);
            assert_eq!(word(&e, this, index) & 2 != 0, expected, "index {index}");
        }
        assert_eq!(word(&e, this, 1), 1);
        assert_eq!(word(&e, this, 2), 0x81);
        // The words after the table are not touched.
        assert_eq!(e.get(this, TutorialManager::eDisplayID), 0x29);
    }

    #[test]
    fn fn_00718840_reads_bit_1_of_a_message() {
        let (mut e, this) = tutorial_engine();
        set_word(&mut e, this, 5, 2);
        set_word(&mut e, this, 6, 0xfd);
        assert!(e.call(0x0071_8840, &args![this, 5u32]).bool());
        assert!(!e.call(0x0071_8840, &args![this, 6u32]).bool());
        assert!(!e.call(0x0071_8840, &args![this, 0x29u32]).bool());
        assert!(!e.call(0x0071_8840, &args![this, u32::MAX]).bool());
    }

    #[test]
    fn fn_00718930_updates_the_flag_and_notifies_the_object() {
        let mut e = base_engine();
        let object = e.mem.alloc(0x700);
        e.set_global(OBJECT_011DEA3C, object);
        nothing(&mut e, &[0x0095_f530]);
        e.mem.set_u8(object + 0x66d, 0x01);
        e.call(0x0071_8930, &args![0x1000u32, 1u32]);
        assert_eq!(e.mem.u8(object + 0x66d), 0x05);
        assert_eq!(logged(&e, 0x0095_f530), vec![vec![object, 1, 1]]);
        e.call(0x0071_8930, &args![0x1000u32, 0u32]);
        assert_eq!(e.mem.u8(object + 0x66d), 0x01);
        assert_eq!(logged(&e, 0x0095_f530)[1], vec![object, 0, 1]);
    }

    #[test]
    fn fn_00718960_sets_or_clears_bit_value_4() {
        let mut e = base_engine();
        let object = e.mem.alloc(0x700);
        e.mem.set_u8(object + 0x66d, 0x80);
        e.call(0x0071_8960, &args![object, 9u32]);
        assert_eq!(e.mem.u8(object + 0x66d), 0x84);
        e.call(0x0071_8960, &args![object, 0u32]);
        assert_eq!(e.mem.u8(object + 0x66d), 0x80);
        e.mem.set_u8(object + 0x66d, 0xff);
        e.call(0x0071_8960, &args![object, 0u32]);
        assert_eq!(e.mem.u8(object + 0x66d), 0xfb);
    }

    #[test]
    fn fn_007189c0_maps_buttons() {
        let mut e = base_engine();
        let expected = [
            (1, 1),
            (2, 2),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, -1),
            (8, -1),
            (9, 0xa),
            (10, 0xb),
            (11, 0xc),
            (12, 0xd),
            (13, 0x10),
            (14, 0x11),
            (15, 0xf),
            (16, 0xe),
            (17, 8),
            (18, 9),
            (0, -1),
            (19, -1),
            (-4, -1),
        ];
        for (button, mapped) in expected {
            assert_eq!(
                e.call(0x0071_89c0, &args![button as u32]).i32(),
                mapped,
                "button {button}"
            );
        }
    }

    #[test]
    fn get_current_imagespace_mod_picks_the_form_by_state() {
        let engine = |special: u32, flagged: u32, top: u32, found: u32| {
            let mut e = base_engine();
            e.set_global(OBJECT_011DEA3C, 0x1234u32);
            stub(&mut e, GET_TOP_MENU_ID, top);
            stub(&mut e, IS_IN_SPECIAL_MODE, special);
            stub(&mut e, 0x0096_7ae0, flagged);
            stub(&mut e, GET_MANAGER, 0x500);
            stub(&mut e, 0x0058_6150, 0x600);
            stub(&mut e, 0x00a0_3da0, found);
            // The form for an id is the id plus one.
            e.register(LOOKUP_FORM_BY_ID, |_, a| ret(a[0] + 1));
            e
        };
        let mut e = engine(1, 1, 0x3f6, 1);
        assert_eq!(e.call(0x0071_8ab0, &args![0u32]).u32(), 0x4eee9);
        let mut e = engine(0, 1, 0x3f6, 1);
        assert_eq!(e.call(0x0071_8ab0, &args![0u32]).u32(), 0x9638a);
        assert_eq!(logged(&e, 0x0096_7ae0), vec![vec![0x1234]]);
        let mut e = engine(0, 0, 0x3f6, 1);
        assert_eq!(e.call(0x0071_8ab0, &args![0u32]).u32(), 0x44f35);
        let mut e = engine(0, 0, 0x3f5, 0);
        assert_eq!(e.call(0x0071_8ab0, &args![0u32]).u32(), 0x32b39);
        assert_eq!(logged(&e, 0x0058_6150), vec![vec![0x500]]);
        assert_eq!(logged(&e, 0x00a0_3da0), vec![vec![0x600, 0x0106_ed28]]);
        let mut e = engine(0, 0, 0x3f5, 1);
        assert_eq!(e.call(0x0071_8ab0, &args![0u32]).u32(), 0);
        assert!(logged(&e, LOOKUP_FORM_BY_ID).is_empty());
    }

    /// Doubles for the color manager's callees: allocation returns heap
    /// blocks, the setting getter returns the address of the setting's value
    /// (`setting + 4`), the constructors return their block, and the list
    /// append records the item it is given by address.
    fn color_engine() -> Engine {
        let mut e = base_engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(0x0043_d4d0, |_, a| ret(a[0] + 4));
        e.register(0x0071_92e0, |_, a| ret(a[0]));
        e.register(0x0071_9250, |_, a| ret(a[0]));
        nothing(&mut e, &[0x0048_f200, 0x004a_1a30, 0x0071_9010]);
        e.register_double(0x004e_d8c0, |e, a| {
            let item = e.mem.u32(a[1]);
            assert_ne!(item, 0);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_00718e90_packs_four_bytes() {
        let mut e = base_engine();
        let pack = |e: &mut Engine, a, b, c, d| {
            e.call(0x0071_8e90, &args![a as u32, b as u32, c as u32, d as u32])
                .u32()
        };
        assert_eq!(pack(&mut e, 0xff, 0xb6, 0x42, 0xff), 0xffb6_42ff);
        assert_eq!(pack(&mut e, 1, 2, 3, 4), 0x0102_0304);
        // Only the highest byte is not masked (the shift drops the excess).
        assert_eq!(pack(&mut e, 0x1ff, 0x1ff, 0x1ff, 0x1ff), 0xffff_ffff);
        assert_eq!(pack(&mut e, 0, 0x100, 0x100, 0x100), 0);
    }

    #[test]
    fn fn_00718db0_registers_a_soft_color_and_defaults_its_setting() {
        let mut e = color_engine();
        let this = e.new_object::<SystemColorManager>();
        e.set(this, SystemColorManager::iNextID, 1);
        e.call(
            0x0071_8db0,
            &args![
                this,
                0x1111u32,
                0xffu32,
                0xb6u32,
                0x42u32,
                0x011d_8ad4u32,
                7u32
            ],
        );
        // The unset setting gets the packed color.
        assert_eq!(e.mem.u32(0x011d_8ad8), 0xffb6_42ff);
        let block = logged(&e, 0x0071_92e0)[0][0];
        assert_eq!(
            logged(&e, 0x0071_92e0),
            vec![vec![block, 0x1111, 0x011d_8ad4, 7]]
        );
        assert_eq!(logged(&e, 0x004e_d8c0)[0][0], this.addr());
        assert_eq!(e.get(this, SystemColorManager::iNextID), 8);

        // A setting that already has a value is kept.
        e.mem.set_u32(0x011d_8ad8, 0x1234_5678);
        e.call_log = Some(vec![]);
        e.call(
            0x0071_8db0,
            &args![
                this,
                0x1111u32,
                0xffu32,
                0xb6u32,
                0x42u32,
                0x011d_8ad4u32,
                2u32
            ],
        );
        assert_eq!(e.mem.u32(0x011d_8ad8), 0x1234_5678);
        assert_eq!(logged(&e, 0x0043_d4d0).len(), 1);
        // The next id follows the given one, even backwards.
        assert_eq!(e.get(this, SystemColorManager::iNextID), 3);
    }

    #[test]
    fn fn_00718db0_appends_a_null_color_when_the_block_is_not_available() {
        let mut e = color_engine();
        stub(&mut e, OPERATOR_NEW, 0);
        e.register_double(0x004e_d8c0, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        let this = e.new_object::<SystemColorManager>();
        e.call(
            0x0071_8db0,
            &args![this, 0x1111u32, 1u32, 2u32, 3u32, 0x011d_8ad4u32, 7u32],
        );
        assert!(logged(&e, 0x0071_92e0).is_empty());
        assert_eq!(logged(&e, 0x004e_d8c0).len(), 1);
        assert_eq!(e.get(this, SystemColorManager::iNextID), 8);
    }

    #[test]
    fn fn_00718ee0_registers_a_hard_color_with_a_raised_id() {
        let mut e = color_engine();
        let this = e.new_object::<SystemColorManager>();
        e.set(this, SystemColorManager::iNextID, 5);
        e.call(
            0x0071_8ee0,
            &args![this, 0x2222u32, 0xffu32, 0x43u32, 0x2au32, u32::MAX],
        );
        let call = logged(&e, 0x0071_9250)[0].clone();
        assert_eq!(call[1..], [0x2222, 0xff43_2aff, 5]);
        assert_eq!(e.get(this, SystemColorManager::iNextID), 6);
        // An id above the next one is kept.
        e.call(0x0071_8ee0, &args![this, 0x3333u32, 1u32, 2u32, 3u32, 9u32]);
        assert_eq!(logged(&e, 0x0071_9250)[1][3], 9);
        assert_eq!(e.get(this, SystemColorManager::iNextID), 10);
        assert_eq!(logged(&e, 0x004e_d8c0).len(), 2);
    }

    #[test]
    fn fn_00718ee0_appends_a_null_color_when_the_block_is_not_available() {
        let mut e = color_engine();
        stub(&mut e, OPERATOR_NEW, 0);
        e.register_double(0x004e_d8c0, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        let this = e.new_object::<SystemColorManager>();
        e.call(0x0071_8ee0, &args![this, 0x2222u32, 1u32, 2u32, 3u32, 4u32]);
        assert!(logged(&e, 0x0071_9250).is_empty());
        assert_eq!(e.get(this, SystemColorManager::iNextID), 5);
    }

    #[test]
    fn system_color_manager_new_registers_the_six_default_colors() {
        let mut e = color_engine();
        let this = e.new_object::<SystemColorManager>();
        let result = e.call(0x0071_8c90, &args![this]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(logged(&e, 0x0048_f200), vec![vec![this.addr()]]);
        // The two soft colors, with their settings defaulted.
        let soft = logged(&e, 0x0071_92e0);
        assert_eq!(soft.len(), 2);
        assert_eq!(soft[0][1..], [0x0106_f404, 0x011d_8ad4, 1]);
        assert_eq!(soft[1][1..], [0x0101_fb30, 0x011d_8ae4, 4]);
        assert_eq!(e.mem.u32(0x011d_8ad8), 0xffb6_42ff);
        assert_eq!(e.mem.u32(0x011d_8ae8), 0xffb6_42ff);
        // The four hard colors; the last two take the next free ids.
        let hard = logged(&e, 0x0071_9250);
        let hard: Vec<[u32; 3]> = hard.iter().map(|c| [c[1], c[2], c[3]]).collect();
        assert_eq!(
            hard,
            vec![
                [0x0106_f3fc, 0xff43_2aff, 2],
                [0x0102_93a8, 0x21e7_79ff, 3],
                [0x0106_f3f0, 0xffb6_42ff, 5],
                [0x0106_f3e8, 0xffb6_42ff, 6],
            ]
        );
        assert_eq!(logged(&e, 0x004e_d8c0).len(), 6);
        assert_eq!(e.get(this, SystemColorManager::iNextID), 7);
    }

    #[test]
    fn system_color_manager_get_instance_creates_the_singleton_once() {
        let mut e = color_engine();
        let first = e.call(0x0071_8b60, &args![]).u32();
        assert_ne!(first, 0);
        assert_eq!(e.global::<u32>(SYSTEM_COLOR_MANAGER_INSTANCE), first);
        assert_eq!(logged(&e, OPERATOR_NEW)[0], vec![0x10]);
        assert_eq!(logged(&e, 0x0048_f200), vec![vec![first]]);
        let second = e.call(0x0071_8b60, &args![]).u32();
        assert_eq!(second, first);
        assert_eq!(logged(&e, 0x0048_f200).len(), 1);
    }

    #[test]
    fn system_color_manager_get_instance_stores_null_when_allocation_fails() {
        let mut e = color_engine();
        stub(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0071_8b60, &args![]).u32(), 0);
        assert!(logged(&e, 0x0048_f200).is_empty());
    }

    #[test]
    fn fn_00718bf0_destroys_the_singleton() {
        let mut e = color_engine();
        let instance = e.mem.alloc(0x10);
        e.set_global(SYSTEM_COLOR_MANAGER_INSTANCE, instance);
        e.call(0x0071_8bf0, &args![]);
        assert_eq!(logged(&e, 0x0071_9010), vec![vec![instance]]);
        assert_eq!(logged(&e, 0x004a_1a30), vec![vec![instance]]);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![instance]]);
        assert_eq!(e.global::<u32>(SYSTEM_COLOR_MANAGER_INSTANCE), 0);
        // Without an instance only the first helper runs.
        e.call_log = Some(vec![]);
        e.call(0x0071_8bf0, &args![]);
        assert_eq!(logged(&e, 0x0071_9010), vec![vec![0]]);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn fn_00718c40_frees_only_when_bit_0_is_set() {
        let mut e = color_engine();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0071_8c40, &args![this, 0u32]).u32(), this);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(logged(&e, 0x004a_1a30), vec![vec![this]]);
        assert_eq!(e.call(0x0071_8c40, &args![this, 3u32]).u32(), this);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(logged(&e, 0x004a_1a30).len(), 2);
    }

    #[test]
    fn fn_00718c70_destroys_the_color_list() {
        let mut e = color_engine();
        e.call(0x0071_8c70, &args![0x4000u32]);
        assert_eq!(logged(&e, 0x004a_1a30), vec![vec![0x4000]]);
    }

    #[test]
    fn fn_00718d90_forwards_and_returns_this() {
        let mut e = base_engine();
        nothing(&mut e, &[0x0076_b630]);
        assert_eq!(e.call(0x0071_8d90, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(logged(&e, 0x0076_b630), vec![vec![0x5000]]);
    }

    #[test]
    fn system_color_manager_find_matches_the_trait_id_of_the_name() {
        let mut e = base_engine();
        // Three list nodes [next, prev, color]; the colors keep their id
        // string at +4 and the middle node holds no color.
        let colors: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x10)).collect();
        e.mem.set_u32(colors[0] + 4, 0xa0);
        e.mem.set_u32(colors[2] + 4, 0xc0);
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(12)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(*node, nodes.get(i + 1).copied().unwrap_or(0));
        }
        e.mem.set_u32(nodes[0] + 8, colors[0]);
        e.mem.set_u32(nodes[1] + 8, 0);
        e.mem.set_u32(nodes[2] + 8, colors[2]);
        let this = e.new_object::<SystemColorManager>();
        e.mem.set_u32(this.addr(), nodes[0]);
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        // The iterator step: returns the item's address, advances.
        e.register(0x0057_cbe0, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            ret(node + 8)
        });
        e.register(0x007f_a950, |e, a| ret(e.mem.u32(a[0] + 4)));
        // TextToTrait: the trait id of a name is the name plus 0x1000.
        e.register(0x00a0_1860, |_, a| ret(a[0] + 0x1000));
        assert_eq!(
            e.call(0x0071_8fb0, &args![this, 0x10c0u32]).u32(),
            colors[2]
        );
        assert_eq!(
            e.call(0x0071_8fb0, &args![this, 0x10a0u32]).u32(),
            colors[0]
        );
        assert_eq!(e.call(0x0071_8fb0, &args![this, 0x1234u32]).u32(), 0);
        // The first color was tried before the third one.
        assert_eq!(logged(&e, 0x00a0_1860)[0], vec![0xa0]);
        // An empty list finds nothing.
        e.mem.set_u32(this.addr(), 0);
        assert_eq!(e.call(0x0071_8fb0, &args![this, 0x10a0u32]).u32(), 0);
    }

    // ---- 00719010 .. 00719bb0: system colors, the tile refresh, the fade ----

    /// The callees `system_color_manager_find` uses, and a manager whose
    /// list holds one node with `color`; the color's id string is `0xa0`,
    /// that is trait id `0x10a0` for the `TextToTrait` double.
    fn manager_with(e: &mut Engine, color: u32) -> Ptr<SystemColorManager> {
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0057_cbe0, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            ret(node + 8)
        });
        e.register(0x007f_a950, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x00a0_1860, |_, a| ret(a[0] + 0x1000));
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, color);
        let manager = e.new_object::<SystemColorManager>();
        e.mem.set_u32(manager.addr(), node);
        e.mem.set_u32(color + 4, 0xa0);
        manager
    }

    /// A system color object (0x10 bytes) whose virtual `GetColor` (slot 4)
    /// returns `packed` and whose `SetColor` (slot 8) is the logged double
    /// `5000_0008`.
    fn color_object(e: &mut Engine, packed: u32) -> u32 {
        e.register_double(0x5000_0004, move |_, _| ret(packed));
        e.register(0x5000_0008, |_, _| Ret::default());
        e.register(0x5000_0000, |_, _| Ret::default());
        let color = e.mem.alloc(0x10);
        e.put_vtable(0x6000_0000, &[0x5000_0000, 0x5000_0004, 0x5000_0008]);
        e.mem.set_u32(color, 0x6000_0000);
        color
    }

    fn float_pages(e: &mut Engine) {
        for page in [
            0x0101_e000,
            0x0102_3000,
            0x0102_e000,
            0x0102_f000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
    }

    #[test]
    fn fn_00719010_deletes_every_color_until_the_list_is_empty() {
        let mut e = base_engine();
        let manager = e.new_object::<SystemColorManager>();
        // Two items in the list, the second one null: only the first is
        // deleted.
        e.mem.set_u32(manager.addr() + 8, 2);
        let color = color_object(&mut e, 0);
        let items = [color, 0];
        e.register(0x0076_b610, |e, a| ret((e.mem.u32(a[0] + 8) == 0) as u32));
        e.register_double(0x007b_5390, move |e, a| {
            let count = e.mem.u32(a[0] + 8) - 1;
            e.mem.set_u32(a[0] + 8, count);
            ret(items[(1 - count) as usize])
        });
        e.call(0x0071_9010, &args![manager]);
        assert_eq!(logged(&e, 0x5000_0000), vec![vec![color, 1]]);
        assert_eq!(e.mem.u32(manager.addr() + 8), 0);
    }

    #[test]
    fn system_color_manager_get_color_fills_the_color_when_found() {
        let mut e = base_engine();
        float_pages(&mut e);
        e.set_global(COLOR_BYTE_SCALE, 0.5f32);
        let color = color_object(&mut e, 0xff80_4020);
        let manager = manager_with(&mut e, color);
        let out = e.mem.alloc(12);
        assert!(e.call(0x0071_9060, &args![manager, 0x10a0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 127.5);
        assert_eq!(e.mem.f32(out + 4), 64.0);
        assert_eq!(e.mem.f32(out + 8), 32.0);
        // An id nobody has leaves the output alone.
        assert!(!e.call(0x0071_9060, &args![manager, 0x1111u32, out]).bool());
        assert_eq!(e.mem.f32(out), 127.5);
    }

    #[test]
    fn fn_007190a0_returns_the_packed_color_or_zero() {
        let mut e = base_engine();
        let color = color_object(&mut e, 0x1234_5678);
        let manager = manager_with(&mut e, color);
        assert_eq!(
            e.call(0x0071_90a0, &args![manager, 0x10a0u32]).u32(),
            0x1234_5678
        );
        assert_eq!(e.call(0x0071_90a0, &args![manager, 0x1111u32]).u32(), 0);
    }

    #[test]
    fn system_color_manager_set_color_packs_the_channels() {
        let mut e = base_engine();
        float_pages(&mut e);
        e.set_global(COLOR_CHANNEL_SCALE, 255.0f64);
        // `004bd510(value, 1.0)`: the value rounded up to a multiple of 1.
        e.register(ROUND_UP_TO_MULTIPLE, |_, a| {
            ret_float((f32::from_bits(a[0]) as f64).ceil())
        });
        let color = color_object(&mut e, 0);
        let manager = manager_with(&mut e, color);
        let input = e.mem.alloc(12);
        e.mem.set_f32(input, 1.0);
        e.mem.set_f32(input + 4, 0.5);
        e.mem.set_f32(input + 8, 0.25);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0071_90e0, &args![manager, 0x10a0u32, input])
            .bool());
        // Blue (63.75 -> 64), green (127.5 -> 128) and red (255), in that
        // order, packed red first with a low byte of 0xff.
        let rounds = logged(&e, ROUND_UP_TO_MULTIPLE);
        assert_eq!(rounds[0], vec![63.75f32.to_bits(), 1.0f32.to_bits()]);
        assert_eq!(rounds[1][0], 127.5f32.to_bits());
        assert_eq!(rounds[2][0], 255.0f32.to_bits());
        assert_eq!(logged(&e, 0x5000_0008), vec![vec![color, 0xff80_40ff]]);
        assert!(!e
            .call(0x0071_90e0, &args![manager, 0x1111u32, input])
            .bool());
    }

    #[test]
    fn fn_00719120_sets_the_packed_color_when_found() {
        let mut e = base_engine();
        let color = color_object(&mut e, 0);
        let manager = manager_with(&mut e, color);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0071_9120, &args![manager, 0x10a0u32, 0xdead_beefu32])
            .bool());
        assert_eq!(logged(&e, 0x5000_0008), vec![vec![color, 0xdead_beef]]);
        assert!(!e.call(0x0071_9120, &args![manager, 0x1111u32, 1u32]).bool());
        assert_eq!(logged(&e, 0x5000_0008).len(), 1);
    }

    /// Doubles for the string and registration calls of the constructor.
    fn constructor_engine() -> Engine {
        let mut e = base_engine();
        nothing(
            &mut e,
            &[
                0x0040_37b0,
                0x0040_37d0,
                0x0040_6f60,
                0x004a_fad0,
                SET_NAMED_VALUE,
            ],
        );
        e.register(0x0055_9450, |_, a| ret(a[0] + 0x100));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00719160_builds_the_id_string_and_registers_it() {
        let mut e = constructor_engine();
        let color = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0071_9160, &args![color, 0x7000u32, 9u32]).u32(),
            color
        );
        assert_eq!(e.mem.u32(color), SYSTEM_COLOR_VTABLE);
        assert_eq!(
            order(&e),
            vec![
                0x0071_9160,
                0x0040_37b0,
                0x0040_6f60,
                0x004a_fad0,
                0x0055_9450,
                SET_NAMED_VALUE
            ]
        );
        assert_eq!(
            logged(&e, 0x0040_6f60),
            vec![vec![color + 4, SYSTEM_COLOR_NAME_FORMAT, 0x7000]]
        );
        assert_eq!(logged(&e, SET_NAMED_VALUE), vec![vec![color + 0x104, 9]]);
    }

    #[test]
    fn fn_00719200_and_fn_00719230_destroy_the_base_color() {
        let mut e = constructor_engine();
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color, 1);
        e.call_log = Some(vec![]);
        e.call(0x0071_9230, &args![color]);
        assert_eq!(e.mem.u32(color), SYSTEM_COLOR_VTABLE);
        assert_eq!(logged(&e, 0x0040_37d0), vec![vec![color + 4]]);
        assert_eq!(e.call(0x0071_9200, &args![color, 0u32]).u32(), color);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0071_9200, &args![color, 1u32]);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![color]]);
    }

    #[test]
    fn fn_00719250_adds_the_hard_color() {
        let mut e = constructor_engine();
        let color = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0071_9250, &args![color, 0x7000u32, 0xff00_00ffu32, 3u32])
                .u32(),
            color
        );
        assert_eq!(e.mem.u32(color), HARD_SYSTEM_COLOR_VTABLE);
        assert_eq!(
            e.get(Ptr::<HardSystemColor>::new(color), HardSystemColor::iRGB),
            0xff00_00ff
        );
        assert_eq!(logged(&e, SET_NAMED_VALUE)[0][1], 3);
    }

    #[test]
    fn fn_00719290_and_fn_007192c0_destroy_the_hard_color() {
        let mut e = constructor_engine();
        let color = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0071_92c0, &args![color]);
        // The vtable ends as the base class's after the base destructor.
        assert_eq!(e.mem.u32(color), SYSTEM_COLOR_VTABLE);
        assert_eq!(logged(&e, 0x0040_37d0).len(), 1);
        assert_eq!(e.call(0x0071_9290, &args![color, 1u32]).u32(), color);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![color]]);
    }

    #[test]
    fn fn_007192e0_adds_the_soft_color() {
        let mut e = constructor_engine();
        let color = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x0071_92e0, &args![color, 0x7000u32, 0x8000u32, 4u32])
                .u32(),
            color
        );
        assert_eq!(e.mem.u32(color), SOFT_SYSTEM_COLOR_VTABLE);
        assert_eq!(e.mem.u32(color + 0xc), 0x8000);
    }

    #[test]
    fn soft_system_color_reads_and_writes_the_settings_value() {
        let mut e = base_engine();
        // `0043d4d0` gives the address of the value inside the setting.
        e.register(0x0043_d4d0, |_, a| ret(a[0] + 4));
        let setting = e.mem.alloc(8);
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color + 0xc, setting);
        e.call(0x0071_9340, &args![color, 0xaabb_ccddu32]);
        assert_eq!(e.mem.u32(setting + 4), 0xaabb_ccdd);
        assert_eq!(e.call(0x0071_9320, &args![color]).u32(), 0xaabb_ccdd);
    }

    #[test]
    fn fn_00719360_and_fn_00719390_destroy_the_soft_color() {
        let mut e = constructor_engine();
        let color = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0071_9390, &args![color]);
        assert_eq!(e.mem.u32(color), SYSTEM_COLOR_VTABLE);
        assert_eq!(e.call(0x0071_9360, &args![color, 0u32]).u32(), color);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0071_9360, &args![color, 1u32]);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![color]]);
    }

    #[test]
    fn fn_00719440_splits_the_bytes_high_first() {
        let mut e = base_engine();
        let out = e.mem.alloc(16);
        e.call(
            0x0071_9440,
            &args![0x1122_3344u32, out, out + 4, out + 8, out + 12],
        );
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn fn_00719580_refreshes_children_before_the_matching_tile() {
        let mut e = base_engine();
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0057_cbe0, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            ret(node + 8)
        });
        // The tile's name is the word at +0x20; a name matches when it is
        // the very string address (0 = equal).
        e.register(TILE_NAME, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(COMPARE_STRINGS, |_, a| ret((a[0] != a[1]) as u32));
        e.register(GET_MANAGER, |_, _| Ret::default());
        e.register(MANAGER_READY, |_, _| ret(1));
        e.register(TEXT_TO_TRAIT, |_, a| ret(a[0] + 1));
        nothing(&mut e, &[TILE_SET_VALUE, TILE_SET_NEEDS_UPDATE]);
        // A parent matching the second name with a child matching the first.
        let parent = e.mem.alloc(0x30);
        let child = e.mem.alloc(0x30);
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, child);
        e.mem.set_u32(parent + 4, node);
        e.mem.set_u32(child + 0x20, TILE_NAME_FIRST);
        e.mem.set_u32(parent + 0x20, TILE_NAME_SECOND);
        e.call_log = Some(vec![]);
        e.call(0x0071_9580, &args![parent]);
        assert_eq!(
            logged(&e, TILE_SET_VALUE),
            vec![
                vec![child, READY_TRAIT_NAME + 1, 1],
                vec![parent, READY_TRAIT_NAME + 1, 1]
            ]
        );
        assert_eq!(
            logged(&e, TILE_SET_NEEDS_UPDATE),
            vec![vec![child, 4], vec![parent, 4]]
        );
        // A tile with another name is left alone.
        e.mem.set_u32(parent + 0x20, 0x1234);
        e.mem.set_u32(parent + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0071_9580, &args![parent]);
        assert!(logged(&e, TILE_SET_VALUE).is_empty());
    }

    /// Doubles for the whole interface refresh; the manager's menus root
    /// holds one child tile whose menu has the virtual slot `0x44`
    /// (`5000_0044`). Returns the interface object and the menu.
    fn refresh_engine(ready: u32) -> (Engine, Ptr<InterfaceManagerFields>, u32) {
        let mut e = base_engine();
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0057_cbe0, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            ret(node + 8)
        });
        e.register(TILE_NAME, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(COMPARE_STRINGS, |_, _| ret(1));
        e.register(GET_MANAGER, |_, _| Ret::default());
        e.register_double(MANAGER_READY, move |_, _| ret(ready));
        e.register(TEXT_TO_TRAIT, |_, a| ret(a[0] + 1));
        nothing(
            &mut e,
            &[
                TILE_SET_VALUE,
                SET_NAMED_VALUE,
                REFRESH_START_MENU,
                SET_RENDERER_FLAG,
                STORE_BYTE,
                0x5000_0044,
            ],
        );
        let menu = e.mem.alloc(8);
        e.put_vtable(0x6000_0100, &[0; 0x12]);
        e.mem.set_u32(0x6000_0100 + 0x44, 0x5000_0044);
        e.mem.set_u32(menu, 0x6000_0100);
        e.register_double(TILE_GET_MENU, move |_, _| ret(menu));
        let child = e.mem.alloc(0x30);
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, child);
        let menus_root = e.mem.alloc(0x30);
        e.mem.set_u32(menus_root + 4, node);
        let this = e.new_object::<InterfaceManagerFields>();
        let string_root = e.mem.alloc(0x30);
        e.mem.set_u32(this.addr() + 0x9c, menus_root);
        e.mem.set_u32(this.addr() + 0xa0, string_root);
        (e, this, menu)
    }

    #[test]
    fn fn_00719630_applies_the_ready_state_to_every_menu() {
        let (mut e, this, menu) = refresh_engine(1);
        let string_root = e.mem.u32(this.addr() + 0xa0);
        e.call_log = Some(vec![]);
        e.call(0x0071_9630, &args![this, 0xffffu32]);
        assert_eq!(
            logged(&e, SET_NAMED_VALUE),
            vec![vec![NAMED_VALUE_FIRST, 1], vec![NAMED_VALUE_SECOND, 1]]
        );
        assert_eq!(
            logged(&e, TILE_SET_VALUE),
            vec![vec![string_root, ROOT_TRAIT_NAME + 1, 1]]
        );
        // The child's menu is looked up twice and told the state.
        assert_eq!(logged(&e, TILE_GET_MENU).len(), 2);
        assert_eq!(logged(&e, 0x5000_0044), vec![vec![menu, 1]]);
        assert_eq!(*order(&e).last().unwrap(), REFRESH_START_MENU);
    }

    #[test]
    fn fn_00719630_skips_the_menu_of_a_child_without_one() {
        let (mut e, this, _) = refresh_engine(0);
        e.register(TILE_GET_MENU, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0071_9630, &args![this, 0u32]);
        assert!(logged(&e, 0x5000_0044).is_empty());
        assert_eq!(logged(&e, SET_NAMED_VALUE)[0], vec![NAMED_VALUE_FIRST, 0]);
        assert_eq!(logged(&e, REFRESH_START_MENU).len(), 1);
    }

    #[test]
    fn fn_00719740_stores_the_flag_around_the_refresh() {
        let (mut e, this, _) = refresh_engine(1);
        let target = e.mem.alloc(8);
        e.set_global(OBJECT_011DEA0C, 0x7777u32);
        e.register_double(GET_MANAGER, move |_, _| ret(this.addr()));
        e.register_double(GET_OBJECT_FROM_011DEA0C, move |_, _| ret(target));
        e.call_log = Some(vec![]);
        e.call(0x0071_9740, &args![0x1000u32, 1u8]);
        assert_eq!(
            logged(&e, SET_RENDERER_FLAG),
            vec![vec![RENDERER_OBJECT, 1]]
        );
        assert_eq!(logged(&e, GET_OBJECT_FROM_011DEA0C), vec![vec![0x7777]]);
        assert_eq!(logged(&e, STORE_BYTE), vec![vec![target, 1]]);
        let calls = order(&e);
        let renderer = calls.iter().position(|a| *a == SET_RENDERER_FLAG).unwrap();
        let refresh = calls.iter().position(|a| *a == REFRESH_START_MENU).unwrap();
        let store = calls.iter().position(|a| *a == STORE_BYTE).unwrap();
        assert!(renderer < refresh && refresh < store);
    }

    /// The state `00719790` reads: constants, the player (with its owner
    /// at `+0xA4` whose slot `0xC` answers `weapon_value`), the clock at
    /// `now`, and doubles for every callee.
    fn fade_engine(level: f32, falling: bool, now: u32, last: u32) -> (Engine, u32) {
        let mut e = base_engine();
        float_pages(&mut e);
        e.set_global(FADE_UPPER_BOUND, 485.0f32);
        e.set_global(FADE_LOWER_BOUND, -230.0f32);
        e.set_global(FADE_RATE, 0.25f32);
        e.set_global(FLOAT_MAXIMUM, 255.0f32);
        e.set_global(COLOR_CHANNEL_SCALE, 255.0f64);
        e.set_global(INVERSE_THRESHOLD, 128.0f64);
        e.set_global(FADE_LEVEL, level);
        e.set_global(FADE_FALLING, falling as u8);
        e.set_global(FADE_LAST_TIME, last);
        e.register(CLAMP_FLOAT, |e, a| {
            let value = e.mem.f32(a[0]);
            let (min, max) = (f32::from_bits(a[1]), f32::from_bits(a[2]));
            if value > max {
                e.mem.set_f32(a[0], max);
            } else if value < min {
                e.mem.set_f32(a[0], min);
            }
            Ret::default()
        });
        e.register_double(GET_CLOCK_TIME, move |_, _| ret(now));
        nothing(&mut e, &[TILE_SET_FLOAT]);
        e.register(GET_WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        // The entry point writes 2.0 through its last argument.
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            e.mem.set_f32(a[3], 2.0);
            Ret::default()
        });
        // The actor value 5 is 3.0.
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| ret_float(3.0));
        e.register(GET_TYPE_BYTE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_COUNT_FROM_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x19c)));
        e.register(GET_DESIRED_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x1bc)));
        e.register(GET_LAST_BOUND_WEAPON, |e, a| ret(e.mem.u32(a[0] + 0x15c)));
        // The weapon's value is the weapon word as a float.
        e.register(0x5000_000c, |_, a| ret_float(a[1] as f64));
        let player = e.mem.alloc(0x700);
        e.put_vtable(0x6000_0200, &[0, 0, 0, 0x5000_000c]);
        e.mem.set_u32(player + 0xa4, 0x6000_0200);
        e.set_global(OBJECT_011DEA3C, player);
        e.call_log = Some(vec![]);
        (e, player)
    }

    fn fade_tiles(e: &mut Engine) -> (u32, u32, u32, u32) {
        let tiles: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        // `source`: the word at +8 is an object of type 0x28 with counts.
        let source = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x200);
        e.mem.set_u8(object + 4, 0x28);
        e.mem.set_u32(source + 8, object);
        e.mem.set_u32(object + 0x19c, 100);
        e.mem.set_u32(object + 0x1bc, 50);
        e.mem.set_u32(object + 0x15c, 10);
        (tiles[0], tiles[1], tiles[2], source)
    }

    #[test]
    fn fn_00719790_sets_inverse_and_level_then_rises() {
        let (mut e, _) = fade_engine(100.0, false, 5000, 4000);
        let (a, b, _, _) = fade_tiles(&mut e);
        e.call(0x0071_9790, &args![a, b, 0u32, 0u32, 0u32]);
        assert_eq!(
            logged(&e, TILE_SET_FLOAT),
            vec![
                vec![a, TRAIT_FADE, 155.0f32.to_bits(), 1],
                vec![b, TRAIT_FADE, 100.0f32.to_bits(), 1]
            ]
        );
        // 1000 ms at 0.25 per ms.
        assert_eq!(e.global::<f32>(FADE_LEVEL), 350.0);
        assert_eq!(e.global::<u32>(FADE_LAST_TIME), 5000);
        assert_eq!(e.global::<u8>(FADE_FALLING), 0);
    }

    #[test]
    fn fn_00719790_clamps_the_level_and_flips_the_direction() {
        // Rising past the upper bound: clamped, now falling.
        let (mut e, _) = fade_engine(400.0, false, 2000, 1000);
        e.call(0x0071_9790, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.global::<f32>(FADE_LEVEL), 485.0);
        assert_eq!(e.global::<u8>(FADE_FALLING), 1);
        // The tiles saw the level clamped to 255 (inverse 0), but none exist.
        assert!(logged(&e, TILE_SET_FLOAT).is_empty());
        // Falling below the lower bound: clamped, rising again.
        let (mut e, _) = fade_engine(20.0, true, 2000, 1000);
        e.call(0x0071_9790, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.global::<f32>(FADE_LEVEL), -230.0);
        assert_eq!(e.global::<u8>(FADE_FALLING), 0);
        // In between nothing flips.
        let (mut e, _) = fade_engine(100.0, true, 1100, 1000);
        e.call(0x0071_9790, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.global::<f32>(FADE_LEVEL), 75.0);
        assert_eq!(e.global::<u8>(FADE_FALLING), 1);
    }

    #[test]
    fn fn_00719790_third_tile_uses_the_entry_point_limit() {
        let (mut e, player) = fade_engine(100.0, false, 0, 0);
        let (_, _, c, source) = fade_tiles(&mut e);
        e.call(0x0071_9790, &args![0u32, 0u32, c, source, 0u8]);
        // Limit 3 + 2 = 5 is below the count 100: the inverse (155 > 128).
        assert_eq!(
            logged(&e, TILE_SET_FLOAT),
            vec![vec![c, TRAIT_FADE, 155.0f32.to_bits(), 1]]
        );
        assert_eq!(
            logged(&e, HANDLE_ENTRY_POINT)[0][..3],
            [0x35, player, e.mem.u32(source + 8)]
        );
        // The inverse is not above 128: the level is shown.
        let (mut e, _) = fade_engine(200.0, false, 0, 0);
        let (_, _, c, source) = fade_tiles(&mut e);
        e.call(0x0071_9790, &args![0u32, 0u32, c, source, 0u8]);
        assert_eq!(logged(&e, TILE_SET_FLOAT)[0][2], 200.0f32.to_bits());
        // A count at or below the limit, or a wrong type, gives 255.
        let (mut e, _) = fade_engine(100.0, false, 0, 0);
        let (_, _, c, source) = fade_tiles(&mut e);
        let object = e.mem.u32(source + 8);
        e.mem.set_u32(object + 0x19c, 5);
        e.call(0x0071_9790, &args![0u32, 0u32, c, source, 0u8]);
        assert_eq!(logged(&e, TILE_SET_FLOAT)[0][2], 255.0f32.to_bits());
        e.mem.set_u32(object + 0x19c, 100);
        e.mem.set_u8(object + 4, 0x27);
        e.call(0x0071_9790, &args![0u32, 0u32, c, source, 0u8]);
        assert_eq!(logged(&e, TILE_SET_FLOAT)[1][2], 255.0f32.to_bits());
    }

    #[test]
    fn fn_00719790_third_tile_uses_the_weapon_value_when_asked() {
        let (mut e, _) = fade_engine(100.0, false, 0, 0);
        let (_, _, c, source) = fade_tiles(&mut e);
        // The weapon's value is 10 (its word), below the target 50.
        e.call(0x0071_9790, &args![0u32, 0u32, c, source, 1u8]);
        assert_eq!(logged(&e, TILE_SET_FLOAT)[0][2], 155.0f32.to_bits());
        assert!(logged(&e, HANDLE_ENTRY_POINT).is_empty());
        assert_eq!(logged(&e, 0x5000_000c)[0][1], 10);
        // A target not above the value gives 255.
        let object = e.mem.u32(source + 8);
        e.mem.set_u32(object + 0x1bc, 10);
        e.call(0x0071_9790, &args![0u32, 0u32, c, source, 1u8]);
        assert_eq!(logged(&e, TILE_SET_FLOAT)[1][2], 255.0f32.to_bits());
        // No source: the tile is not touched.
        e.call(0x0071_9790, &args![0u32, 0u32, c, 0u32, 1u8]);
        assert_eq!(logged(&e, TILE_SET_FLOAT).len(), 2);
    }

    #[test]
    fn fn_00719b00_and_fn_00719ae0_test_bit_2_of_the_byte() {
        let mut e = base_engine();
        let object = e.mem.alloc(0x700);
        e.set_global(OBJECT_011DEA3C, object);
        assert!(!e.call(0x0071_9b00, &args![object]).bool());
        assert!(!e.call(0x0071_9ae0, &args![0x1000u32]).bool());
        e.mem.set_u8(object + 0x66d, 0x04);
        assert!(e.call(0x0071_9b00, &args![object]).bool());
        assert!(e.call(0x0071_9ae0, &args![0x1000u32]).bool());
        e.mem.set_u8(object + 0x66d, 0xfb);
        assert!(!e.call(0x0071_9b00, &args![object]).bool());
    }

    #[test]
    fn fn_00719b20_returns_the_first_index_the_callback_accepts() {
        let mut e = base_engine();
        // The callback accepts an element equal to its argument.
        e.register(0x5000_0100, |e, a| ret((e.mem.u32(a[0]) == a[1]) as u32));
        let array = e.new_object::<BSSimpleArray>();
        let buffer = e.mem.alloc(16);
        for (i, v) in [7u32, 8, 9, 8].iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *v);
        }
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, 4);
        let find = |e: &mut Engine, value: u32, start: u32| {
            e.call(0x0071_9b20, &args![array, value, start, 0x5000_0100u32])
                .i32()
        };
        assert_eq!(find(&mut e, 8, 0), 1);
        assert_eq!(find(&mut e, 8, 2), 3);
        assert_eq!(find(&mut e, 5, 0), -1);
        assert_eq!(find(&mut e, 7, 4), -1);
        // A null buffer never calls the callback.
        e.set(array, BSSimpleArray::pBuffer, 0);
        assert_eq!(find(&mut e, 7, 0), -1);
    }

    #[test]
    fn fn_00719b80_sets_the_vtable_and_runs_the_base_constructor() {
        let mut e = base_engine();
        nothing(&mut e, &[0x006b_3eb0]);
        let array = e.new_object::<BSSimpleArray>();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0071_9b80, &args![array]).ptr::<()>().addr(),
            array.addr()
        );
        assert_eq!(e.mem.u32(array.addr()), TILE_ARRAY_VTABLE);
        assert_eq!(logged(&e, 0x006b_3eb0), vec![vec![array.addr(), 0, 0]]);
    }

    #[test]
    fn fn_00719bb0_destroys_and_frees_on_request() {
        let mut e = base_engine();
        nothing(&mut e, &[0x0071_9be0, OPERATOR_DELETE]);
        let array = e.new_object::<BSSimpleArray>();
        e.call_log = Some(vec![]);
        e.call(0x0071_9bb0, &args![array, 0u32]);
        assert_eq!(logged(&e, 0x0071_9be0), vec![vec![array.addr()]]);
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0071_9bb0, &args![array, 1u32]);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![array.addr()]]);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on the running thread takes (the called function's
    /// own frame, taken at the stack's current top), with the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.next_stack_local(size)
    }

    #[test]
    fn the_color_and_tile_walks_keep_their_locals_at_the_exe_offsets() {
        // 00719580 (8 bytes): the iterator at +0; 00719630 (0x14 bytes): at +0x10.
        let (mut e, this, _) = refresh_engine(1);
        let menus_root = e.mem.u32(this.addr() + 0x9c);
        let frame = next_frame(&mut e, 0x8);
        e.call(0x0071_9580, &args![menus_root]);
        assert_eq!(logged(&e, 0x0057_cbe0)[0], vec![menus_root + 4, frame]);
        let frame = next_frame(&mut e, 0x14);
        e.call(0x0071_9630, &args![this, 0u32]);
        let steps = logged(&e, 0x0057_cbe0);
        assert_eq!(steps[steps.len() - 1], vec![menus_root + 4, frame + 0x10]);
        // 00718fb0 (0xc bytes): the iterator at +8.
        let manager = e.new_object::<SystemColorManager>();
        e.call_log = Some(vec![]);
        e.call(0x0071_8fb0, &args![manager, 0x10a0u32]);
        assert!(logged(&e, 0x0057_cbe0).is_empty());
        let node = e.mem.alloc(12);
        e.mem.set_u32(manager.addr(), node);
        e.register(0x007f_a950, |_, _| ret(0));
        e.register(0x00a0_1860, |_, _| ret(0));
        let frame = next_frame(&mut e, 0xc);
        e.call(0x0071_8fb0, &args![manager, 0x10a0u32]);
        assert_eq!(
            logged(&e, 0x0057_cbe0),
            vec![vec![manager.addr(), frame + 8]]
        );
        // 007193b0 (0x2c bytes): red, green, blue and the fourth byte at +0x20, +0x1c, +0x24
        // and +0x28.
        let color_object = e.mem.alloc(8);
        e.put_vtable(0x6000_0200, &[0; 4]);
        e.mem.set_u32(0x6000_0200 + 4, 0x5000_0104);
        e.mem.set_u32(color_object, 0x6000_0200);
        e.register_double(0x5000_0104, |_, _| ret(0x1122_3344));
        e.map(COLOR_BYTE_SCALE & !0xfff, 0x1000);
        e.set_global(COLOR_BYTE_SCALE, 1.0f64);
        let out = e.mem.alloc(12);
        let frame = next_frame(&mut e, 0x2c);
        e.call(0x0071_93b0, &args![color_object, out]);
        let bytes: Vec<u32> = [0x20, 0x1c, 0x24, 0x28]
            .iter()
            .map(|o| e.mem.u32(frame + o))
            .collect();
        assert_eq!(bytes, vec![0x11, 0x22, 0x33, 0x44]);
    }
}
