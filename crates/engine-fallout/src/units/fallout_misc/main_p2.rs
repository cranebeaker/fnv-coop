//! `fallout/misc/main.cpp` (Xbox PDB source unit), part 2: its functions from `008705c0` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::main`]; anything public there may be used here.
//!
//! The first session covers `008705c0` to `00872f50`: the per-frame drawing
//! sequence of the main loop (the world, menu and 1st-person passes), the
//! render-target and window helpers around it, and the `HighActorCuller` /
//! `BSFadeNodeCuller` constructors. The second covers `008731a0` to
//! `00877260`: the world pass with the scene sorting, the 1st-person and
//! menu passes, the frame wind-down, and the archive, start-cell and
//! player-placement setup. The third covers `00877430` to `00877f90`, the
//! end of the range: the teardown to the main menu, the sky setup, and the
//! small `NiTArray`, message-queue and registry-setting helpers.
//!
//! Conventions of the compiled code, used throughout:
//!
//! - `this` of the draw methods is the `Main` object (the pointer in the
//!   global `0x011dea0c`). Its PC offsets used here: `+0x03` bGameActive,
//!   `+0x08` the window handle, `+0x88` the world accumulator `NiPointer`,
//!   `+0x8c` the 1st-person accumulator `NiPointer`, `+0x98` the rendered
//!   menu accumulator `NiPointer`, `+0x9c` the rendering-menu flag. (The
//!   Xbox PDB has these 0x10 higher, the window handle 8 higher.)
//! - The compiler pushes a call's stack arguments before it evaluates the
//!   object expression of a method call, so `PUSH x; CALL getter; MOV
//!   ECX,EAX; CALL method` passes `x` to `method`. Every call's arguments
//!   were read from the disassembly.
//! - Functions with an exception-handling frame have their unwinding left
//!   out; their scope guards are constructed and destroyed on the normal
//!   path only.
//! - x87: floats are computed in `f64` and rounded to `f32` where the code
//!   stores a `float`.
//! - Callees whose purpose could not be confirmed from the exe are called
//!   by address and described by what they do to their arguments.

#[allow(unused_imports)]
use super::main::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

// ---------------------------------------------------------------------------
// Callees outside this file, by exe address.
// ---------------------------------------------------------------------------

/// `NiPointer<T>::operator T*` (`00559450`): the pointer stored at `this`.
const POINTER_GET: u32 = 0x0055_9450;
/// Game-setting accessor (`0043d4d0`): the address of the setting's 4-byte
/// value (`this + 4`; a static zero when `this` is null).
const SETTING_DWORD_PTR: u32 = 0x0043_d4d0;
/// Game-setting accessor (`00408d60`): the address of the setting's 1-byte value.
const SETTING_BYTE_PTR: u32 = 0x0040_8d60;
/// Game-setting accessor (`00403e20`): the address of the setting's float value.
const SETTING_FLOAT_PTR: u32 = 0x0040_3e20;

/// Returns the pointer in the global `0x011f4748`. Its vtable slot `0xc8`
/// returns an object whose slots `0x8c` / `0x90` answer a width and a
/// height for index 0 (a render-target group).
const GET_GLOBAL_011F4748: u32 = 0x0043_c4b0;
/// Returns the pointer in the global `0x011f91ac`.
const GET_GLOBAL_011F91AC: u32 = 0x004e_3270;
/// Returns the pointer in the global `0x011f91a8`: the object that
/// `00b6e110` (create a render target) and `00b6da10` (release one) are
/// methods of.
const GET_GLOBAL_011F91A8: u32 = 0x004a_0ea0;
/// Returns the pointer in the global `0x011d8a80`.
const GET_GLOBAL_011D8A80: u32 = 0x004b_7210;
/// Returns the pointer in the global `0x011df1a8`.
const GET_GLOBAL_011DF1A8: u32 = 0x0045_37b0;
/// Returns the pointer in the global `0x011ca438`.
const GET_GLOBAL_011CA438: u32 = 0x0054_f4c0;
/// Returns the multithreaded-rendering system (the constant address
/// `0x01200088`).
const GET_MT_SYSTEM: u32 = 0x004e_a970;
/// `MTRenderingSystem::SetThreadStage` (`00ba30f0`, Xbox PDB name): `this`,
/// thread index, stage; the body only reads the thread index.
const MT_SET_THREAD_STAGE: u32 = 0x00ba_30f0;
/// The sibling of `00ba30f0` (`00ba3130`): `this`, thread index, stage.
const MT_ADVANCE_THREAD_STAGE: u32 = 0x00ba_3130;
/// Whole-system calls on the multithreaded-rendering system
/// (`00ba2f30`, `00ba2fa0`), each with only `this`.
const MT_BEGIN_FRAME: u32 = 0x00ba_2f30;
const MT_END_FRAME: u32 = 0x00ba_2fa0;
/// The pointer at index `i` of the global array at `0x011f91c8` (`00450b80`).
const INDEXED_GLOBAL: u32 = 0x0045_0b80;
/// The object held by the `NiPointer` global `0x011deb7c` (`0045c670`).
const GET_ROOT: u32 = 0x0045_c670;
/// The object held by the `NiPointer` at `this + 0xac` (`006629f0`).
const GET_ROOT_CHILD: u32 = 0x0066_29f0;
/// `GET_ROOT` then `GET_ROOT_CHILD` (`00524c90`).
const GET_ROOT_CHILD_OF_ROOT: u32 = 0x0052_4c90;
/// The address `this + 0x8c` (`0045bb80`): a three-float member.
const MEMBER_8C: u32 = 0x0045_bb80;
/// The address `this + 0x8c` (`0045bbe0`, a second body of the same shape).
const MEMBER_8C_B: u32 = 0x0045_bbe0;

/// `Interface::IsMenuIDVisible` (`00702680`, Xbox PDB), cdecl: menu id, flag.
const IS_MENU_ID_VISIBLE: u32 = 0x0070_2680;
/// `Interface::IsInMenuMode` (`00702360`, Xbox PDB).
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `Interface::IsTopMenuID` (`00702450`, Xbox PDB), cdecl: menu id.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `Interface::IsInPipboyMenu` (`00705a00`, Xbox PDB).
const IS_IN_PIPBOY_MENU: u32 = 0x0070_5a00;
/// `Interface::IsInRenderedMenu` (`007079b0`, Xbox PDB).
const IS_IN_RENDERED_MENU: u32 = 0x0070_79b0;
/// `Interface::GetCurrentRenderedMenu` (`00707ad0`, Xbox PDB).
const GET_CURRENT_RENDERED_MENU: u32 = 0x0070_7ad0;
/// `Interface::IsCurrentRenderedMenuTopmost` (`00707b30`, Xbox PDB).
const IS_CURRENT_RENDERED_MENU_TOPMOST: u32 = 0x0070_7b30;
/// `Interface::LastMinuteUpdate` (`007058e0`, Xbox PDB).
const LAST_MINUTE_UPDATE: u32 = 0x0070_58e0;
/// `Interface::GetPipboy` (`00705990`, Xbox PDB).
const GET_PIPBOY: u32 = 0x0070_5990;
/// `FORenderedMenu::SetScreenTexture` (`007fae70`, Xbox PDB).
const SET_SCREEN_TEXTURE: u32 = 0x007f_ae70;
/// `Tile::GetMenuByClass` (`00a09030`, Xbox PDB), cdecl: menu id.
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::Lock` / `Tile::Unlock` (`00a044f0`, `00a04500`, Xbox PDB).
const TILE_LOCK: u32 = 0x00a0_44f0;
const TILE_UNLOCK: u32 = 0x00a0_4500;

/// Constructor of a 16-byte value of four floats (`00414430`).
const COLOR_CTOR: u32 = 0x0041_4430;
/// Constructor of a 12-byte value of three words (`0043d410`).
const POINT3_CTOR: u32 = 0x0043_d410;
/// Constructor of an 8-byte value of two floats (`00452dc0`); returns `this`.
const POINT2_CTOR: u32 = 0x0045_2dc0;
/// Method (`00712e60`) of the object that takes a pointer to a four-float
/// value and stores it; the draw routines call it on the root's child.
const SET_COLOR: u32 = 0x0071_2e60;
/// Profiler scope guard constructor (`00404eb0`): byte, flag, file name, line.
const PROFILE_SCOPE_CTOR: u32 = 0x0040_4eb0;
/// Profiler scope guard destructor (`00404ee0`).
const PROFILE_SCOPE_DTOR: u32 = 0x0040_4ee0;
/// The source file name the scope guards pass (`0x010829c4`).
const SOURCE_FILE_NAME: u32 = 0x0108_29c4;
/// Float helper (`00404010`), cdecl: two floats, returns a float in ST0.
const FLOAT_HELPER_A: u32 = 0x0040_4010;
/// Float helper (`0040ebd0`), cdecl: two floats, returns a float in ST0.
const FLOAT_HELPER_B: u32 = 0x0040_ebd0;
/// Float helper (`008731a0`), cdecl: a double and a float, returns a
/// double in ST0 (also part of this unit, beyond this session's range).
const FLOAT_HELPER_C: u32 = 0x0087_31a0;

/// `BSTimer::Enable` / `BSTimer::Disable` (Xbox PDB).
const TIMER_ENABLE: u32 = 0x00aa_5040;
const TIMER_DISABLE: u32 = 0x00aa_50a0;
/// `Controls::GetInstance`, `Controls::ClearKeystrokes`, `Controls::Poll`,
/// `Controls::ClearUserActions` (Xbox PDB names).
const CONTROLS_GET_INSTANCE: u32 = 0x007f_df30;
const CONTROLS_CLEAR_KEYSTROKES: u32 = 0x00a2_37b0;
const CONTROLS_POLL: u32 = 0x00a2_3010;
const CONTROLS_CLEAR_USER_ACTIONS: u32 = 0x00a2_53d0;

// Import slots (kernel32 / user32).
/// `GetActiveWindow`.
const IMPORT_GET_ACTIVE_WINDOW: u32 = 0x00fd_f2fc;
/// `GetWindowLongA`.
const IMPORT_GET_WINDOW_LONG: u32 = 0x00fd_f2e8;
/// `AdjustWindowRectEx`.
const IMPORT_ADJUST_WINDOW_RECT_EX: u32 = 0x00fd_f2f8;
/// `SetWindowPos`.
const IMPORT_SET_WINDOW_POS: u32 = 0x00fd_f2a4;
/// `DebugBreak`.
const IMPORT_DEBUG_BREAK: u32 = 0x00fd_f0c8;

// Globals.
/// The global holding the registry-settings collection (`RegSettingCollection`).
const REG_SETTING_COLLECTION: u32 = 0x0120_4368;
/// The `Main` object.
const MAIN_OBJECT: u32 = 0x011d_ea0c;
/// The pointer in `0x011dea10` (the world / cell grid owner).
const WORLD_OBJECT: u32 = 0x011d_ea10;
/// The player-character pointer (`0x011dea3c`).
const PLAYER_OBJECT: u32 = 0x011d_ea3c;
/// Integer setting; above 1, the extra worker threads run.
const SETTING_THREADS: u32 = 0x011c_3ea4;
/// The `NiPointer` holding the root object (`0x011deb7c`).
const ROOT_POINTER: u32 = 0x011d_eb7c;
/// The `NiPointer` holding the menu-background render target (`0x011ded3c`).
const MENU_TARGET_POINTER: u32 = 0x011d_ed3c;
/// The `NiPointer` holding the offscreen-interface render target (`0x011dec64`).
const OFFSCREEN_TARGET_POINTER: u32 = 0x011d_ec64;

/// Main-object offsets (see the module documentation).
const MAIN_GAME_ACTIVE: u32 = 0x03;
const MAIN_WINDOW: u32 = 0x08;
const MAIN_WORLD_ACCUM: u32 = 0x88;
const MAIN_FIRST_PERSON_ACCUM: u32 = 0x8c;
const MAIN_MENU_ACCUM: u32 = 0x98;
const MAIN_RENDERING_MENU: u32 = 0x9c;

// ---------------------------------------------------------------------------
// Small helpers.
// ---------------------------------------------------------------------------

fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

/// The int value of a game setting.
fn setting_dword(e: &mut Engine, setting: u32) -> i32 {
    let value = e.call(SETTING_DWORD_PTR, &args![setting]).u32();
    e.mem.i32(value)
}

/// The byte value of a game setting.
fn setting_byte(e: &mut Engine, setting: u32) -> u8 {
    let value = e.call(SETTING_BYTE_PTR, &args![setting]).u32();
    e.mem.u8(value)
}

/// `MTRenderingSystem::SetThreadStage(thread, stage)` on the system object.
fn mt_set_stage(e: &mut Engine, thread: u32, stage: u32) {
    let system = e.call(GET_MT_SYSTEM, &args![]).u32();
    e.call(MT_SET_THREAD_STAGE, &args![system, thread, stage]);
}

/// The `00ba3130` call on the same system object.
fn mt_advance_stage(e: &mut Engine, thread: u32, stage: u32) {
    let system = e.call(GET_MT_SYSTEM, &args![]).u32();
    e.call(MT_ADVANCE_THREAD_STAGE, &args![system, thread, stage]);
}

/// Both calls, for thread 0 then thread 1, as the draw sequence does per stage.
fn mt_both_threads(e: &mut Engine, stage: u32) {
    mt_set_stage(e, 0, stage);
    mt_advance_stage(e, 1, stage);
}

fn is_menu_visible(e: &mut Engine, menu_id: u32) -> bool {
    e.call(IS_MENU_ID_VISIBLE, &args![menu_id, 0u32]).bool()
}

/// Runs `body` between a profiler scope guard's constructor and destructor.
fn with_profile_scope<R>(
    e: &mut Engine,
    kind: u32,
    line: u32,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    e.with_stack(0x10, |e, guard| {
        e.call(
            PROFILE_SCOPE_CTOR,
            &args![guard, kind, 1u32, SOURCE_FILE_NAME, line],
        );
        let result = body(e);
        e.call(PROFILE_SCOPE_DTOR, &args![guard]);
        result
    })
}

/// Builds a four-float value at `at` with `00414430`.
fn color_ctor(e: &mut Engine, at: u32, a: f32, b: f32, c: f32, d: f32) {
    e.call(COLOR_CTOR, &args![at, a, b, c, d]);
}

/// Builds the four-float value `(0, width_ratio, height_ratio, 0)` and
/// hands it to `00712e60` on the root's child.
fn set_ratio_color(e: &mut Engine, width_ratio: f32, height_ratio: f32) {
    e.with_stack(0x10, |e, color| {
        let at = color.addr();
        color_ctor(e, at, 0.0, width_ratio, height_ratio, 0.0);
        let child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
        e.call(SET_COLOR, &args![child, at]);
    });
}

/// The low dword of a 64-bit `FISTP` in truncating rounding mode.
fn truncate_low_dword(value: f32) -> u32 {
    (value as f64 as i64) as u32
}

// ---------------------------------------------------------------------------
// The translated functions.
// ---------------------------------------------------------------------------

// Translated from 008705c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte flag in the global `0x01189184`.
pub fn fn_008705c0(e: &mut Engine) -> u8 {
    e.global::<u8>(0x0118_9184)
}

// Translated from 008705d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An idle-time step: `0086f640`, `Main::OnIdle_UpdateProcessLists`
/// (`0086f890`, Xbox PDB), `00552570` when the thread setting is above 1,
/// then `0086f670`. (The decompiler dropped the second call.)
pub fn fn_008705d0(e: &mut Engine, this: Ptr) {
    e.call(0x0086_f640, &args![this]);
    e.call(0x0086_f890, &args![this]);
    if setting_dword(e, SETTING_THREADS) > 1 {
        e.call(0x0055_2570, &args![]);
    }
    e.call(0x0086_f670, &args![this]);
}

// Translated from 00870610 (decompiled, FalloutNV.exe 1.4.0.525)
/// An idle-time step: delayed script actions
/// (`Script::RunDelayedScriptActionsOnReferences`, `005aa720`, Xbox PDB);
/// with more than one thread, three methods (`0087a710`, `0087a850`,
/// `0087a6f0`) of the object in the global `0x011df1a8`;
/// `Interface::HandleQueuedMenuOpen` (`007094f0`, Xbox PDB); `00870680` on
/// the object in `0x011d8a80`; `DebugText::Instance(1)` and its
/// `HandleQueuedPrints` (Xbox PDB).
pub fn fn_00870610(e: &mut Engine, _this: Ptr) {
    e.call(0x005a_a720, &args![]);
    if setting_dword(e, SETTING_THREADS) > 1 {
        for method in [0x0087_a710, 0x0087_a850, 0x0087_a6f0] {
            let object = e.call(GET_GLOBAL_011DF1A8, &args![]).u32();
            e.call(method, &args![object]);
        }
    }
    e.call(0x0070_94f0, &args![]);
    let interface = e.call(GET_GLOBAL_011D8A80, &args![]).u32();
    fn_00870680(e, Ptr::new(interface));
    let debug_text = e.call(0x00a0_d9e0, &args![1u32]).u32();
    e.call(0x00a0_e770, &args![debug_text]);
}

// Translated from 00870680 (decompiled, FalloutNV.exe 1.4.0.525)
/// If the byte at `+0x4cc` is set, closes all support menus
/// (`Interface::CloseAllSupportMenus(0)`, `00706320`, Xbox PDB) and clears
/// the byte.
pub fn fn_00870680(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + 0x4cc) != 0 {
        e.call(0x0070_6320, &args![0u32]);
        e.mem.set_u8(this.addr() + 0x4cc, 0);
    }
}

// Translated from 008706b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the frame's draw routine. First copies the three words at `+0x8c`
/// of the root's child into the `+0x1f0` triple of the indexed global 0
/// (`00870790`), after reading the byte `+0x131` of that object
/// (`00870770`). Then draws the menu-background frame (`008707c0`) when
/// `menu_background` is set and menu `0x3f5` is not visible, otherwise
/// the frame `00870bd0` (byte clear) or `00870a00` (byte set).
pub fn fn_008706b0(e: &mut Engine, this: Ptr, target: u32, menu_background: u8, flag: u8) {
    let first = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
    let byte_131 = fn_00870770(e, Ptr::new(first));
    let root = e.call(GET_ROOT, &args![]).u32();
    let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
    let position = e.call(MEMBER_8C, &args![child]).u32();
    let words = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    let second = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
    fn_00870790(e, Ptr::new(second), words[0], words[1], words[2]);
    if menu_background != 0 && !is_menu_visible(e, 0x3f5) {
        fn_008707c0(e, this, target, byte_131);
    } else if byte_131 == 0 {
        fn_00870bd0(e, this, target, flag);
    } else {
        fn_00870a00(e, this, target, flag);
    }
}

// Translated from 00870770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x131`.
pub fn fn_00870770(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x131)
}

// Translated from 00870790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three words (a position) at `+0x1f0`, `+0x1f4`, `+0x1f8`.
pub fn fn_00870790(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) {
    e.mem.set_u32(this.addr() + 0x1f0, first);
    e.mem.set_u32(this.addr() + 0x1f4, second);
    e.mem.set_u32(this.addr() + 0x1f8, third);
}

// Translated from 008707c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The draw sequence of a frame with a menu background: resets the render
/// stages `1` to `0x10`, builds the frame (`00872f50`), draws the menu pass
/// (`00874b90`), then, depending on the pipboy menu, either hands the
/// conversation target (`008d80e0`) the first-person accumulator and the
/// root child's... sky object and draws the world (`00875110`), or only
/// advances stage `0x16`; then the overlays and `00876850`.
///
/// `target` is the object the frame renders into (zero for the default
/// target) and `flag` is the byte `+0x131` of the indexed global 0.
pub fn fn_008707c0(e: &mut Engine, this: Ptr, target: u32, flag: u8) {
    let global_91ac = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
    let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
    let byte_flag = e.call(0x004d_c310, &args![]).u8();
    let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    let rendered = if menu != 0 {
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        e.call(0x007f_a950, &args![menu]).u32()
    } else {
        0
    };
    fn_00872ad0(e, this);
    for stage in 1..0x11u32 {
        mt_both_threads(e, stage);
    }
    fn_00871290(e, this);
    let frame = fn_00872f50(e, this, renderer, target, flag, byte_flag);
    e.call(
        0x0087_4b90,
        &args![this, global_91ac, renderer, byte_flag, target],
    );
    if setting_dword(e, SETTING_THREADS) > 1 {
        e.call(0x008c_80e0, &args![0u32]);
    }
    mt_both_threads(e, 0x15);
    if !e.call(IS_IN_PIPBOY_MENU, &args![]).bool() {
        mt_both_threads(e, 0x16);
    } else {
        let sky = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
        let root = e.call(GET_ROOT, &args![]).u32();
        let conversation_target = e.call(0x008d_80e0, &args![root]).u32();
        let accumulator = pointer_get(e, this.addr() + MAIN_FIRST_PERSON_ACCUM);
        e.call(0x004a_0fd0, &args![conversation_target, accumulator]);
        e.call(0x0041_fd00, &args![conversation_target, sky]);
        let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        e.call(0x00b5_e870, &args![indexed, conversation_target]);
        e.call(0x0087_4c10, &args![this]);
        e.call(0x0087_5110, &args![this, renderer, rendered, 0u32, frame]);
    }
    e.call(0x0087_5bf0, &args![this]);
    if fn_008709f0(e) != 0 && flag == 0 {
        e.call(
            0x0087_5fd0,
            &args![this, byte_flag, frame, renderer, target],
        );
        if rendered != 0 {
            e.call(0x0087_61e0, &args![this, rendered, renderer]);
        }
    } else if target != 0 {
        e.call(0x0087_6830, &args![this, byte_flag, target]);
    }
    e.call(
        0x0087_6850,
        &args![this, frame, global_91ac, renderer, 0u32],
    );
}

// Translated from 008709f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte flag in the global `0x011f91a4`.
pub fn fn_008709f0(e: &mut Engine) -> u8 {
    e.global::<u8>(0x011f_91a4)
}

/// The test `00870a00` and `00870bd0` make before drawing the world: false
/// when `forced_off` or `other` is set, when `005585e0` of the object from
/// `00705910` is non-zero, or when `00705100` is set.
fn world_draw_enabled(e: &mut Engine, forced_off: bool, other: u8) -> bool {
    if forced_off || other != 0 {
        return false;
    }
    let interface = e.call(0x0070_5910, &args![]).u32();
    if e.call(0x0055_85e0, &args![interface]).u32() != 0 {
        return false;
    }
    e.call(0x0070_5100, &args![]).u8() == 0
}

// Translated from 00870a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The draw sequence of a frame (variant with the byte `+0x131` set):
/// builds the frame (`00872f50`), draws the world through `00873200`,
/// `00874c10`, `00874b50` and `00875110` when `world_draw_enabled`
/// allows it (otherwise only advances stage `0x16`), the overlays, and
/// finishes with `00876850`.
pub fn fn_00870a00(e: &mut Engine, this: Ptr, target: u32, skip_world: u8) {
    let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
    let global_91ac = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
    let player = e.mem.u32(PLAYER_OBJECT);
    let player_child = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
    let flag_a = e.call(0x0045_6610, &args![player_child]).u8();
    let byte_flag = e.call(0x004d_c310, &args![]).u8();
    e.call(0x00b9_8480, &args![1u32, 1u32]);
    fn_00872ad0(e, this);
    let accumulator = fn_00872b00(e, this);
    fn_008727d0(e, this);
    let accumulator_flag = e.call(0x0087_49b0, &args![this]).u8();
    fn_00871290(e, this);
    let frame = fn_00872f50(e, this, renderer, target, 0, byte_flag);
    let draw_world = world_draw_enabled(e, flag_a != 0, skip_world);
    e.call(
        0x0087_3200,
        &args![this, accumulator, draw_world as u32, 1u32, accumulator_flag],
    );
    if draw_world {
        e.call(0x0087_4c10, &args![this]);
        e.call(0x0087_4b50, &args![this, frame, byte_flag, 0u32]);
        e.call(
            0x0087_5110,
            &args![this, renderer, 0u32, accumulator, frame],
        );
    } else {
        mt_both_threads(e, 0x16);
    }
    if target != 0 {
        e.call(0x0087_6830, &args![this, byte_flag, target]);
    }
    e.call(0x0071_4c20, &args![1u32]);
    if fn_008709f0(e) != 0 {
        e.call(
            0x0087_5fd0,
            &args![this, byte_flag, frame, renderer, target],
        );
    } else if target != 0 {
        e.call(0x0087_6830, &args![this, byte_flag, target]);
    }
    e.call(
        0x0087_6850,
        &args![this, frame, global_91ac, renderer, 0u32],
    );
}

// Translated from 00870bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The draw sequence of a frame (variant with the byte `+0x131` clear):
/// like `00870a00`, plus the screen-split margins (`00b4f540`) when
/// `00709cb0` says so, the water pass (`008746d0`, `00875e40`) when the
/// water object of the renderer group passes `00871220`, the shadow
/// preparation of the world accumulator (`00874920`), and the targeted
/// reference's screen position (`00a6fdb0`, `00b8b1e0`).
///
/// The compiler left a branch that is never taken (`XOR ECX,ECX; JZ`) in
/// front of a call of `00875fa0`; it is dead and not translated.
pub fn fn_00870bd0(e: &mut Engine, this: Ptr, target: u32, skip_world: u8) {
    let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
    let global_91ac = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
    let player = e.mem.u32(PLAYER_OBJECT);
    let player_child = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
    let flag_a = e.call(0x0045_6610, &args![player_child]).u8();
    let byte_flag = e.call(0x004d_c310, &args![]).u8();
    fn_00872ad0(e, this);
    let accumulator = fn_00872b00(e, this);
    fn_008727d0(e, this);
    let accumulator_flag = e.call(0x0087_49b0, &args![this]).u8();
    fn_00871290(e, this);
    let frame = fn_00872f50(e, this, renderer, target, 0, byte_flag);
    e.call(0x0087_4ac0, &args![this, accumulator_flag]);
    e.call(0x0087_4b50, &args![this, frame, byte_flag, 5u32]);
    let draw_world = world_draw_enabled(e, flag_a != 0, skip_world);

    // The accumulator's owner flag is saved and forced on while the world
    // is set up.
    let mut saved_owner_flag = 0u8;
    if draw_world && accumulator != 0 {
        let owner = e.call(0x0043_b230, &args![accumulator]).u32();
        saved_owner_flag = e.call(0x0045_6610, &args![owner]).u8();
        let owner = e.call(0x0043_b230, &args![accumulator]).u32();
        e.call(0x0045_0f90, &args![owner, 1u32]);
    }

    let mut margins_enabled = true;
    if e.call(0x004d_e080, &args![]).bool() {
        margins_enabled = false;
    }
    let margins_wanted = e.call(0x0070_9cb0, &args![]).u8();
    if margins_wanted != 0 {
        let width = e.call(0x004d_ee10, &args![renderer]).u32();
        let height = e.call(0x004d_ee70, &args![renderer]).u32();
        let aspect_setting = e.call(SETTING_FLOAT_PTR, &args![0x011d_ea8cu32]).u32();
        let aspect = e.call(0x0070_6e70, &args![]).f32();
        let scale: f64 = e.global(0x0101_1588);
        let factor = ((aspect as f64 * e.mem.f32(aspect_setting) as f64) * scale) as f32;
        let inner_width = (width as f64 * factor as f64) as f32;
        let margin = (width as f64 - inner_width as f64) as f32;
        if margins_enabled {
            e.call(
                0x00b4_f540,
                &args![
                    truncate_low_dword(inner_width),
                    0u32,
                    truncate_low_dword(margin),
                    height
                ],
            );
        }
    }

    e.call(
        0x0087_3200,
        &args![this, accumulator, draw_world as u32, 0u32, accumulator_flag],
    );
    if draw_world && accumulator != 0 {
        let owner = e.call(0x0043_b230, &args![accumulator]).u32();
        e.call(0x0045_0f90, &args![owner, saved_owner_flag as u32]);
    }

    let water_group = e.call(0x004e_bbc0, &args![global_91ac, 4u32]).u32();
    let water_active = fn_00871220(e, Ptr::new(water_group));
    let bit_nine = e.call(0x00b8_b390, &args![global_91ac, 9u32]).bool();
    let reference_marker = bit_nine && {
        let world = e.mem.u32(WORLD_OBJECT);
        e.call(0x005f_36f0, &args![world]).u32() == 0 && setting_byte(e, 0x011d_eadc) != 0
    };

    let mut water_drawn = false;
    if water_active && draw_world {
        water_drawn = true;
        e.call(0x0087_46d0, &args![this, frame]);
        e.call(0x0087_4b50, &args![this, frame, byte_flag, 0u32]);
    }
    if e.call(0x005b_9b00, &args![]).bool() {
        let accumulator_slot = pointer_get(e, this.addr() + MAIN_WORLD_ACCUM);
        if e.call(0x00b6_3a90, &args![accumulator_slot]).u32() != 0 {
            let accumulator_slot = pointer_get(e, this.addr() + MAIN_WORLD_ACCUM);
            e.call(0x0087_4920, &args![this, accumulator_slot, frame]);
        }
    }

    if draw_world {
        e.call(0x0087_4c10, &args![this]);
        e.call(0x0087_4b50, &args![this, frame, byte_flag, 0u32]);
        if !water_drawn {
            mt_advance_stage(e, 1, 0x16);
        }
        e.call(
            0x0087_5110,
            &args![this, renderer, 0u32, accumulator, frame],
        );
    } else {
        mt_both_threads(e, 0x16);
    }

    if water_active {
        let accumulator_slot = pointer_get(e, this.addr() + MAIN_WORLD_ACCUM);
        e.call(0x0087_5e40, &args![this, accumulator_slot, frame]);
    }

    if reference_marker {
        let world = e.mem.u32(WORLD_OBJECT);
        if e.call(0x005f_36f0, &args![world]).u32() == 0 {
            screen_position_of_reference(e);
        }
    }

    if fn_008709f0(e) != 0 {
        e.call(
            0x0087_5fd0,
            &args![this, byte_flag, frame, renderer, target],
        );
        if (fn_00871260(e) || e.call(0x005b_ac20, &args![]).bool())
            && !e.call(IS_IN_PIPBOY_MENU, &args![]).bool()
        {
            e.call(0x0087_6a20, &args![this, renderer]);
        }
    } else if target != 0 {
        e.call(0x0087_6830, &args![this, byte_flag, target]);
    }

    if margins_wanted != 0 {
        if margins_enabled {
            e.call(0x00b5_40d0, &args![]);
        }
        if e.call(0x004d_e080, &args![]).bool() {
            let renderer_object = e.call(GET_GLOBAL_011F4748, &args![]).u32();
            e.vcall(renderer_object, 0xac, &args![0x011a_9bd0u32]);
            e.with_stack(0x10, |e, color| {
                let at = color.addr();
                color_ctor(e, at, 0.0, 0.0, 0.0, 0.0);
                e.mem.set_f32(at, 0.0);
                let second: f32 = e.global(0x0102_6994);
                e.mem.set_f32(at + 4, second);
                e.mem.set_f32(at + 8, 1.0);
                e.mem.set_f32(at + 12, 0.0);
                let renderer_object = e.call(GET_GLOBAL_011F4748, &args![]).u32();
                e.call(0x0071_48c0, &args![renderer_object, at, 1u32]);
                let first: f32 = e.global(0x0103_40a4);
                e.mem.set_f32(at, first);
                e.mem.set_f32(at + 4, 1.0);
                e.mem.set_f32(at + 8, 1.0);
                e.mem.set_f32(at + 12, 0.0);
                let renderer_object = e.call(GET_GLOBAL_011F4748, &args![]).u32();
                e.call(0x0071_48c0, &args![renderer_object, at, 1u32]);
            });
        }
    }
    e.call(
        0x0087_6850,
        &args![this, frame, global_91ac, renderer, accumulator],
    );
}

/// The block of `00870bd0` that projects the position of the reference in
/// the global `0x011dea20` to the screen (`00a6fdb0` on the root child's
/// child) and hands the result to `00b8b1e0` through a two-float value.
fn screen_position_of_reference(e: &mut Engine) {
    let picked = e.mem.u32(0x011d_ea20);
    if picked == 0 || e.call(0x0045_cd60, &args![picked]).u32() == 0 {
        return;
    }
    let picked = e.mem.u32(0x011d_ea20);
    let reference = e.call(0x0045_cd60, &args![picked]).u32();
    let node = e.vcall(reference, 4, &args![]).u32();
    let position = e.call(MEMBER_8C, &args![node]).u32();
    e.with_stack(0x20, |e, block| {
        // The position (3 words) at +0x0 and three output floats.
        let at = block.addr();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(at + 4 * i, word);
        }
        let (output_a, output_b, output_c) = (at + 0xc, at + 0x10, at + 0x14);
        e.mem.set_f32(output_a, 0.0);
        e.mem.set_f32(output_b, 0.0);
        e.mem.set_f32(output_c, 0.0);
        let camera = e.call(GET_ROOT, &args![]).u32();
        let camera_child = e.call(GET_ROOT_CHILD, &args![camera]).u32();
        let last_argument: f32 = e.global(0x0107_18c0);
        e.call(
            0x00a6_fdb0,
            &args![
                camera_child,
                at,
                output_a,
                output_b,
                output_c,
                last_argument
            ],
        );
        let value_a = e.mem.f32(output_a);
        let value_b = e.mem.f32(output_b);
        e.with_stack(8, |e, pair| {
            let pair = e.call(POINT2_CTOR, &args![pair, value_a, value_b]).u32();
            let global = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
            e.call(0x00b8_b1e0, &args![global, pair]);
        });
    });
}

// Translated from 00871220 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the float at `+0x70` is greater than the double at `0x01012060`.
pub fn fn_00871220(e: &mut Engine, this: Ptr) -> bool {
    let value = e.mem.f32(this.addr() + 0x70) as f64;
    let limit: f64 = e.global(0x0101_2060);
    value > limit
}

// Translated from 00871260 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the `NiPointer` global `0x011c7810` holds an object for which
/// `00453470` returns non-zero.
pub fn fn_00871260(e: &mut Engine) -> bool {
    if pointer_get(e, 0x011c_7810) == 0 {
        return false;
    }
    let object = pointer_get(e, 0x011c_7810);
    e.call(0x0045_3470, &args![object]).u32() != 0
}

// Translated from 00871290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame shadow preparation, in three parts:
///
/// - When `005bf7b0` and `00525420` are both set, remembers the first in
///   `0x011def08`, calls `005bf770(0)`, prepares the shadow scene node
///   (the indexed global 0, `00b5b1f0`) and calls `00b6ee70` with 0 on the
///   object in `0x011f91a8`. When that state ends (the flag is set and
///   `00525420` is not), calls `005bf770(1)` and `00b6ee70` with the
///   shadow-caster limit.
/// - Then, when `005bf7b0` and the byte setting `0x011c74d4` are set,
///   prepares the scene node (`00b4f570`, `00b5b060`), tells `00b6ee70` the
///   limit, and with a non-zero limit picks the shadow casters (the player
///   and the actors of the list at `0x011e0e80` that pass the checks, up to
///   the limit; each goes to `00b5cbd0`) and walks the scene node's lights
///   (`00b5afc0` / `00b5b010`), recomputing and queuing the usable ones
///   and switching the others off (`shadow_light`). With a zero limit it
///   only calls `00b5c3c0`.
/// - Otherwise advances stages `0x11` to `0x14` on both threads.
///
/// Ends with `Main::UpdateOffscreenInterface` (`00871a50`).
pub fn fn_00871290(e: &mut Engine, this: Ptr) {
    if e.call(0x005b_f7b0, &args![]).bool() && e.call(0x0052_5420, &args![]).bool() {
        let value = e.call(0x005b_f7b0, &args![]).u8();
        e.set_global(0x011d_ef08, value);
        e.call(0x005b_f770, &args![0u32]);
        let scene = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        e.call(0x00b5_b1f0, &args![scene]);
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
        e.call(0x00b6_ee70, &args![factory, renderer, 0u32]);
    } else if e.global::<u8>(0x011d_ef08) != 0 && !e.call(0x0052_5420, &args![]).bool() {
        e.set_global(0x011d_ef08, 0u8);
        e.call(0x005b_f770, &args![1u32]);
        let limit = shadow_caster_limit(e);
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
        e.call(0x00b6_ee70, &args![factory, renderer, limit]);
    }

    if e.call(0x005b_f7b0, &args![]).bool() && fn_00871a10(e) != 0 {
        e.call(0x00b4_f570, &args![]);
        let scene = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        e.call(0x00b5_b060, &args![scene]);
        let limit = shadow_caster_limit(e);
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
        e.call(0x00b6_ee70, &args![factory, renderer, limit]);
        if limit == 0 {
            e.call(0x00b5_c3c0, &args![scene]);
        } else {
            pick_shadow_casters_and_walk_lights(e, scene, limit as i32);
        }
    } else {
        for stage in 0x11..0x15u32 {
            mt_both_threads(e, stage);
        }
    }
    main_update_offscreen_interface(e, this);
}

/// The shadow-caster limit: the setting `0x011c72d8` when `005f36f0` of the
/// world object is non-zero, otherwise `0x011c72e4`.
fn shadow_caster_limit(e: &mut Engine) -> u32 {
    let world = e.mem.u32(WORLD_OBJECT);
    let setting = if e.call(0x005f_36f0, &args![world]).u32() != 0 {
        0x011c_72d8
    } else {
        0x011c_72e4
    };
    let value = e.call(SETTING_DWORD_PTR, &args![setting]).u32();
    e.mem.u32(value)
}

/// The second part of `00871290` for a non-zero `limit`: picks the shadow
/// casters (the player's, then actors of the list at `0x011e0e80`) and
/// walks the scene node's lights.
fn pick_shadow_casters_and_walk_lights(e: &mut Engine, scene: u32, limit: i32) {
    let mut first_caster = 0u32;
    let mut chosen = 0i32;
    let player = e.mem.u32(PLAYER_OBJECT);
    if e.call(0x004e_af60, &args![player]).bool() || e.call(0x0095_0090, &args![player]).bool() {
        let player = e.mem.u32(PLAYER_OBJECT);
        if !e.call(0x008c_51c0, &args![player]).bool() {
            let player = e.mem.u32(PLAYER_OBJECT);
            let child = e.call(0x0095_0bb0, &args![player, 0u32]).u32();
            first_caster = e.call(0x00b5_cbd0, &args![scene, child]).u32();
            chosen += 1;
        }
    }
    e.call(0x0040_fbf0, &args![0x011f_1180u32, 0u32]);
    let actors = fn_00871a30(e, Ptr::new(0x011e_0e80)).addr();
    let count = e.call(0x0047_2380, &args![0x011e_0e80u32]).i32();
    let mut index = 0i32;
    while index < count && chosen < limit {
        let actor = e.mem.u32(actors + 4 * index as u32);
        index += 1;
        if e.vcall(actor, 0x1d0, &args![]).u32() == 0 {
            continue;
        }
        let process = e.vcall(actor, 0x1d0, &args![]).u32();
        if e.call(0x0096_11e0, &args![process]).u32() == 0 {
            continue;
        }
        if !e.call(0x0087_f200, &args![actor]).bool() {
            continue;
        }
        if e.vcall(actor, 0x214, &args![]).u32() == 4 {
            continue;
        }
        if e.call(0x008c_51c0, &args![actor]).bool() {
            continue;
        }
        let extra_key = e.call(0x0050_ea90, &args![]).u32();
        let process = e.vcall(actor, 0x1d0, &args![]).u32();
        let node = e.call(0x00a5_bdd0, &args![process, extra_key]).u32();
        let position = if node != 0 {
            e.call(0x0050_0940, &args![node]).u32()
        } else {
            0x011f_426c
        };
        let x = e.mem.f32(position);
        let y = e.mem.f32(position + 4);
        let z = e.mem.f32(position + 8);
        let partial = e.call(FLOAT_HELPER_A, &args![x, y]).f32();
        let largest = e.call(FLOAT_HELPER_A, &args![partial, z]).f32();
        let scale: f64 = e.global(0x0102_1928);
        let threshold = largest as f64 * scale;
        let process = e.vcall(actor, 0x1d0, &args![]).u32();
        let bound = e.call(0x0043_d450, &args![process]).u32();
        let radius = e.call(0x0084_d030, &args![bound]).f32() as f64;
        if radius < threshold {
            let process = e.vcall(actor, 0x1d0, &args![]).u32();
            e.call(0x00b5_cbd0, &args![scene, process]);
            chosen += 1;
        }
    }
    e.call(0x0040_fba0, &args![0x011f_1180u32]);
    e.call(0x00b5_cde0, &args![scene, first_caster]);

    let player = e.mem.u32(PLAYER_OBJECT);
    let player_child = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
    if player_child == 0 {
        return;
    }
    let root = e.call(GET_ROOT, &args![]).u32();
    let conversation_target = e.call(0x008d_80e0, &args![root]).u32();
    let sky = e.call(GET_ROOT_CHILD, &args![root]).u32();
    e.call(0x0041_fd00, &args![conversation_target, sky]);
    let sky = e.call(GET_ROOT_CHILD, &args![root]).u32();
    let member = e.call(MEMBER_8C_B, &args![sky]).u32();
    e.call(0x00a6_94a0, &args![conversation_target, member]);
    let child_flag = e.call(0x0045_6610, &args![player_child]).u8();
    e.call(0x0045_0f90, &args![player_child, 1u32]);
    let saved_object = e.call(GET_GLOBAL_011CA438, &args![]).u32();
    e.call(0x004e_20b0, &args![0u32]);
    let mut saved_flag = 0u8;
    if saved_object != 0 {
        saved_flag = e.call(0x0045_6610, &args![saved_object]).u8();
        if setting_byte(e, 0x011c_71dc) == 0 {
            e.call(0x0045_0f90, &args![saved_object, 1u32]);
        }
    }

    let mut light = e.call(0x00b5_afc0, &args![scene]).u32();
    let mut processed = 0i32;
    while light != 0 {
        shadow_light(e, light, conversation_target, limit, &mut processed);
        light = e.call(0x00b5_b010, &args![scene]).u32();
    }
    while processed + 0x11 < 0x15 {
        mt_set_stage(e, 0, (processed + 0x11) as u32);
        processed += 1;
    }
    if saved_object != 0 {
        e.call(0x0045_0f90, &args![saved_object, saved_flag as u32]);
    }
    e.call(0x004e_20b0, &args![1u32]);
    e.call(0x00b5_b880, &args![scene, conversation_target]);
    e.call(0x0045_0f90, &args![player_child, child_flag as u32]);
}

/// One light of the walk in `00871290`: lights with geometry and a source
/// are first reset (`00b9bb10`); then, if fewer than `limit` have been
/// queued and neither the geometry nor the source is flagged, the light is
/// recomputed (`00b9d150`, `00b9e970`, `00b9dfc0`, `00b9fba0`) and queued
/// (`00b5d300`, `00ba02b0`) when its count is not `0xff` and its weight
/// (`00979260`) reaches the cutoff `0x01082cb0`; any other light has its
/// geometry flagged with `00450f90(.., 1)`.
fn shadow_light(
    e: &mut Engine,
    light: u32,
    conversation_target: u32,
    limit: i32,
    processed: &mut i32,
) {
    let geometry = e.call(0x004e_6540, &args![light]).u32();
    if geometry == 0 {
        return;
    }
    let source = e.call(0x005b_f820, &args![light]).u32();
    if source == 0 {
        return;
    }
    if !e.call(0x004e_6580, &args![light]).bool() {
        let geometry = e.call(0x004e_6540, &args![light]).u32();
        if e.call(0x0045_6610, &args![geometry]).bool() {
            let source = e.call(0x005b_f820, &args![light]).u32();
            if e.call(0x0045_6610, &args![source]).bool() {
                e.call(0x00b9_bb10, &args![light, 0.0f32, 1u32]);
            } else {
                if *processed < limit {
                    let geometry = e.call(0x004e_6540, &args![light]).u32();
                    e.call(0x0045_0f90, &args![geometry, 0u32]);
                }
                e.call(0x00b9_bb10, &args![light, 0.0f32, 0u32]);
            }
        }
    }

    let mut queued = false;
    let usable = *processed < limit && {
        let geometry = e.call(0x004e_6540, &args![light]).u32();
        !e.call(0x0045_6610, &args![geometry]).bool() && {
            let source = e.call(0x005b_f820, &args![light]).u32();
            !e.call(0x0045_6610, &args![source]).bool()
        }
    };
    if usable {
        let source = e.call(0x005b_f820, &args![light]).u32();
        // The source's four-word bound is copied to a local the code never reads.
        e.call(0x0043_d450, &args![source]);
        let source = e.call(0x005b_f820, &args![light]).u32();
        let position = e.call(MEMBER_8C, &args![source]).u32();
        let words = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        e.call(
            0x00b9_d150,
            &args![light, 0u32, words[0], words[1], words[2]],
        );
        e.call(0x00b9_e970, &args![light, conversation_target]);
        if e.call(0x004e_6560, &args![light]).u16() != 0xff {
            let weight = e.call(0x0097_9260, &args![light]).f32() as f64;
            let cutoff: f64 = e.global(0x0108_2cb0);
            // `FCOMP` then `TEST AH,1`: skipped when below the cutoff or unordered.
            if weight >= cutoff {
                *processed += 1;
                e.with_stack(0x10, |e, point| {
                    e.call(POINT3_CTOR, &args![point, 0.0f32, 0u32, 0u32]);
                    let geometry = e.call(0x004e_6540, &args![light]).u32();
                    e.call(0x00a5_9c60, &args![geometry, point]);
                });
                e.call(
                    0x00b9_dfc0,
                    &args![light, conversation_target, (*processed - 1) as u32],
                );
                e.call(0x00b9_fba0, &args![light]);
                let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
                e.call(0x0045_4b30, &args![indexed]);
                let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
                e.call(0x00b5_d300, &args![indexed, light]);
                if setting_byte(e, 0x011c_7664) != 0 {
                    let source = e.call(0x005b_f820, &args![light]).u32();
                    e.call(0x00ba_0110, &args![light, source]);
                }
                e.call(0x00ba_02b0, &args![light]);
                queued = true;
            }
        }
    }
    if !queued {
        let geometry = e.call(0x004e_6540, &args![light]).u32();
        e.call(0x0045_0f90, &args![geometry, 1u32]);
    }
}

// Translated from 00871a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte value of the game setting at `0x011c74d4`.
pub fn fn_00871a10(e: &mut Engine) -> u8 {
    setting_byte(e, 0x011c_74d4)
}

// Translated from 00871a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `+0x88`.
pub fn fn_00871a30(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0x88)
}

// Translated from 00871a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::UpdateOffscreenInterface` (Xbox PDB): when in menu mode, one of
/// the byte settings `0x011db2cc` / `0x011d8ba0` is set and a rendered menu
/// is up: creates the offscreen target (`00b6e110`) if it does not
/// exist, unless in the pipboy sets the pipboy's screen texture to 0, and,
/// when the rendered menu is topmost or one of the menus `0x3e9`, `0x3f8`,
/// `0x423` exists, calls slot `0x0c` of the rendered menu with the
/// target (the reference of menu `0x3e9` is flagged during the call when in
/// the pipboy and that menu is visible). Otherwise runs
/// `Interface::LastMinuteUpdate` when in menu mode.
pub fn main_update_offscreen_interface(e: &mut Engine, _this: Ptr) {
    let enabled = e.call(IS_IN_MENU_MODE, &args![]).bool()
        && (setting_byte(e, 0x011d_b2cc) != 0 || setting_byte(e, 0x011d_8ba0) != 0)
        && e.call(IS_IN_RENDERED_MENU, &args![]).bool();
    if !enabled {
        if e.call(IS_IN_MENU_MODE, &args![]).bool() {
            e.call(LAST_MINUTE_UPDATE, &args![]);
        }
        return;
    }
    with_profile_scope(e, 0xd, 0x1ade, |e| {
        let in_pipboy = e.call(IS_IN_PIPBOY_MENU, &args![]).bool();
        if e.call(IS_IN_MENU_MODE, &args![]).bool() {
            e.call(LAST_MINUTE_UPDATE, &args![]);
        }
        if pointer_get(e, OFFSCREEN_TARGET_POINTER) == 0 {
            let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
            let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
            let target = e
                .call(
                    0x00b6_e110,
                    &args![factory, renderer, 0x2du32, 0u32, 0u32, 0u32],
                )
                .u32();
            e.call(0x0066_b0d0, &args![OFFSCREEN_TARGET_POINTER, target]);
        }
        if !in_pipboy {
            let pipboy = e.call(GET_PIPBOY, &args![]).u32();
            e.call(SET_SCREEN_TEXTURE, &args![pipboy, 0u32]);
        }
        let wanted = e.call(IS_CURRENT_RENDERED_MENU_TOPMOST, &args![]).bool()
            || e.call(GET_MENU_BY_CLASS, &args![0x3e9u32]).u32() != 0
            || e.call(GET_MENU_BY_CLASS, &args![0x3f8u32]).u32() != 0
            || e.call(GET_MENU_BY_CLASS, &args![0x423u32]).u32() != 0;
        if e.call(IS_IN_RENDERED_MENU, &args![]).bool() && wanted {
            let marked = in_pipboy && is_menu_visible(e, 0x3e9);
            let mut reference = 0u32;
            let mut reference_flag = 0u8;
            if marked {
                let menu = e.call(GET_MENU_BY_CLASS, &args![0x3e9u32]).u32();
                reference = e.call(0x0056_c7f0, &args![menu]).u32();
                reference_flag = e.call(0x0045_6610, &args![reference]).u8();
                e.call(0x0045_0f90, &args![reference, 1u32]);
            }
            let rendered_menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
            let target = pointer_get(e, OFFSCREEN_TARGET_POINTER);
            e.vcall(rendered_menu, 0x0c, &args![target, 1u32, 0u32]);
            if marked {
                e.call(0x0045_0f90, &args![reference, reference_flag as u32]);
            }
        }
    });
}

// Translated from 00871c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::SetActive` (Xbox PDB): with the byte setting `0x011deed8` set,
/// `active` becomes 1. When the controls object exists, clears keystrokes,
/// polls and clears the user actions; enables (active) or disables the
/// timer at `0x011f6394`; stores `active` at `+0x03`.
pub fn main_set_active(e: &mut Engine, this: Ptr, active: u8) {
    let mut active = active;
    if setting_byte(e, 0x011d_eed8) != 0 {
        active = 1;
    }
    if e.call(CONTROLS_GET_INSTANCE, &args![]).u32() != 0 {
        for method in [
            CONTROLS_CLEAR_KEYSTROKES,
            CONTROLS_POLL,
            CONTROLS_CLEAR_USER_ACTIONS,
        ] {
            let controls = e.call(CONTROLS_GET_INSTANCE, &args![]).u32();
            e.call(method, &args![controls]);
        }
    }
    if active != 0 {
        e.call(TIMER_ENABLE, &args![0x011f_6394u32]);
    } else {
        e.call(TIMER_DISABLE, &args![0x011f_6394u32]);
    }
    e.mem.set_u8(this.addr() + MAIN_GAME_ACTIVE, active);
}

// Translated from 00871d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// False unless `GetActiveWindow()` is the window at `+0x08`, the game is
/// in menu mode and `00711e00` of the controls object is non-zero. Then,
/// unless menu `0x41e` is on top, calls `00703080` with the two ints
/// divided by the display's width (`0x0118947c`, for `x`) and height
/// (`0x01189480`, for `y`) as floats; with menu `0x41e` on top calls
/// `00706230(x, y)`. Returns true.
pub fn fn_00871d10(e: &mut Engine, this: Ptr, x: i32, y: i32) -> bool {
    let window = e.call(IMPORT_GET_ACTIVE_WINDOW, &args![]).u32();
    if window != e.mem.u32(this.addr() + MAIN_WINDOW) {
        return false;
    }
    if !e.call(IS_IN_MENU_MODE, &args![]).bool() {
        return false;
    }
    let controls = e.call(CONTROLS_GET_INSTANCE, &args![]).u32();
    if e.call(0x0071_1e00, &args![controls]).u32() == 0 {
        return false;
    }
    if !e.call(IS_TOP_MENU_ID, &args![0x41eu32]).bool() {
        let height = e.call(0x004d_c200, &args![]).i32();
        let fraction_y = (y as f64 / height as f64) as f32;
        let width = e.call(0x004d_c1f0, &args![]).i32();
        let fraction_x = (x as f64 / width as f64) as f32;
        e.call(0x0070_3080, &args![fraction_x, fraction_y]);
    } else {
        e.call(0x0070_6230, &args![x, y]);
    }
    true
}

// Translated from 00871dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::RenderMenuBackground` (Xbox PDB): needs the player object
/// (`0x011dea3c`) to pass `008d6f30` and to return a non-zero from its
/// vtable slot `0x1d0`. Marks the `Main` object as rendering the menu,
/// re-creates the menu-background target (`00b6e110`), and when its width
/// or height differs from the renderer group's sets the root child's value
/// to the (clamped) size ratios; draws the frame through `008706b0`; while
/// menu `0x40c` is visible and `004a4040` holds, accumulates the rendered
/// menu's scene (`00b6bee0`, `00b6c0d0`); and restores the state.
pub fn main_render_menu_background(e: &mut Engine, this: Ptr) {
    with_profile_scope(e, 0xd, 0x1c19, |e| {
        let player = e.mem.u32(PLAYER_OBJECT);
        if e.call(0x008d_6f30, &args![player]).u32() == 0
            || e.vcall(player, 0x1d0, &args![]).u32() == 0
        {
            return;
        }
        let saved_flag = e.global::<u8>(0x011a_d884);
        e.set_global(0x011a_d884, 1u8);
        e.mem.set_u8(this.addr() + MAIN_RENDERING_MENU, 1);
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        e.call(0x004a_0370, &args![renderer]);
        e.call(TILE_LOCK, &args![]);
        let system = e.call(GET_MT_SYSTEM, &args![]).u32();
        e.call(MT_BEGIN_FRAME, &args![system]);

        if pointer_get(e, MENU_TARGET_POINTER) != 0 {
            let target = pointer_get(e, MENU_TARGET_POINTER);
            let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
            e.call(0x00b6_da10, &args![factory, target]);
        }
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
        let new_target = e
            .call(
                0x00b6_e110,
                &args![factory, renderer, 6u32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.call(0x0066_b0d0, &args![MENU_TARGET_POINTER, new_target]);

        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        let group = e.vcall(renderer, 0xc8, &args![]).u32();
        let target = pointer_get(e, MENU_TARGET_POINTER);
        let target_width = fn_00872370(e, Ptr::new(target), 0);
        let group_width = e.vcall(group, 0x8c, &args![0u32]).u32();
        let mut changed = group_width != target_width;
        if !changed {
            let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
            let group = e.vcall(renderer, 0xc8, &args![]).u32();
            let target = pointer_get(e, MENU_TARGET_POINTER);
            let target_height = fn_008723d0(e, Ptr::new(target), 0);
            let group_height = e.vcall(group, 0x90, &args![0u32]).u32();
            changed = group_height != target_height;
        }
        if changed {
            let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
            let group = e.vcall(renderer, 0xc8, &args![]).u32();
            let target = pointer_get(e, MENU_TARGET_POINTER);
            let target_width = fn_00872370(e, Ptr::new(target), 0);
            let group_width = e.vcall(group, 0x8c, &args![0u32]).u32();
            let ratio = (group_width as f64 / target_width as f64) as f32;
            let width_ratio = e.call(FLOAT_HELPER_B, &args![ratio, 1.0f32]).f32();
            let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
            let group = e.vcall(renderer, 0xc8, &args![]).u32();
            let target = pointer_get(e, MENU_TARGET_POINTER);
            let target_height = fn_008723d0(e, Ptr::new(target), 0);
            let group_height = e.vcall(group, 0x90, &args![0u32]).u32();
            let ratio = (group_height as f64 / target_height as f64) as f32;
            let height_ratio = e.call(FLOAT_HELPER_B, &args![ratio, 1.0f32]).f32();
            set_ratio_color(e, width_ratio, height_ratio);
        } else if e.global::<u8>(0x011f_9426) != 0 {
            let child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
            e.call(SET_COLOR, &args![child, 0x011a_d840u32]);
        }

        let interface = e.call(GET_GLOBAL_011D8A80, &args![]).u32();
        let image_space = e.call(0x0071_8ab0, &args![interface]).u32();
        let modifier = e.call(0x005d_2860, &args![]).u32();
        e.call(0x0052_9c90, &args![modifier]);
        if image_space != 0 {
            e.call(0x0052_99a0, &args![image_space, 1.0f32, 0u32]);
            e.call(0x0086_fd90, &args![this, 0u32]);
        }
        let mut in_pipboy = e.call(IS_IN_PIPBOY_MENU, &args![]).u8();
        if is_menu_visible(e, 0x3f5) {
            in_pipboy = 0;
        }
        let target = pointer_get(e, MENU_TARGET_POINTER);
        fn_008706b0(e, this, target, 0, in_pipboy);

        if is_menu_visible(e, 0x40c) && e.call(0x004a_4040, &args![]).bool() {
            // 004a0eb0 builds a 0xc8-byte object (its base 00a69400 fills 0x90 bytes, then
            // +0x90, +0xbc, +0xc0 and the pointer at +0xc4); the exe keeps it at EBP-0xec
            // (`EBP + 0xffffff14`) of 00871dc0.
            e.with_stack(0xc8, |e, guard| {
                let at = guard.addr();
                e.call(0x004a_0eb0, &args![at, 0u32]);
                let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
                let scene = e.call(0x008b_6200, &args![menu]).u32();
                e.call(0x0041_fd00, &args![at, scene]);
                let accumulator = pointer_get(e, this.addr() + MAIN_MENU_ACCUM);
                e.call(0x004a_0fd0, &args![at, accumulator]);
                let indexed = e.call(INDEXED_GLOBAL, &args![1u32]).u32();
                let accumulator = pointer_get(e, this.addr() + MAIN_MENU_ACCUM);
                e.call(0x004a_1020, &args![accumulator, indexed]);
                let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
                let cell = e.call(0x0055_85e0, &args![menu]).u32();
                e.call(0x00b6_bee0, &args![scene, cell, at]);
                let accumulator = pointer_get(e, this.addr() + MAIN_MENU_ACCUM);
                e.call(0x00b6_c0d0, &args![scene, accumulator, 0u32]);
                e.call(0x004a_0fd0, &args![at, 0u32]);
                e.call(0x004a_0f60, &args![at]);
            });
        }
        if image_space != 0 {
            e.call(0x0052_9c90, &args![image_space]);
            e.call(0x0086_fd90, &args![this, 0u32]);
        }
        mt_both_threads(e, 0x17);
        let system = e.call(GET_MT_SYSTEM, &args![]).u32();
        e.call(MT_END_FRAME, &args![system]);
        e.set_global(0x011d_ea29, 1u8);
        e.with_stack(0x10, |e, color| {
            let at = color.addr();
            color_ctor(e, at, 0.0, 1.0, 1.0, 0.0);
            let child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
            e.call(SET_COLOR, &args![child, at]);
        });
        e.call(TILE_UNLOCK, &args![]);
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        e.call(0x004a_03c0, &args![renderer]);
        e.mem.set_u8(this.addr() + MAIN_RENDERING_MENU, 0);
        e.set_global(0x011a_d884, saved_flag);
    });
}

// Translated from 00872370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x94` of the object held by the `NiPointer` at
/// `this + 0x30 + 4 * index` (the width the callers compare with the
/// display's), 0 when the pointer is empty.
pub fn fn_00872370(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let object = pointer_get(e, this.addr() + 0x30 + 4 * index);
    if object != 0 {
        e.vcall(object, 0x94, &args![]).u32()
    } else {
        0
    }
}

// Translated from 008723d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x98` of the object held by the `NiPointer` at
/// `this + 0x30 + 4 * index` (the height the callers compare with the
/// display's), 0 when the pointer is empty.
pub fn fn_008723d0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let object = pointer_get(e, this.addr() + 0x30 + 4 * index);
    if object != 0 {
        e.vcall(object, 0x98, &args![]).u32()
    } else {
        0
    }
}

// Translated from 00872430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the text file named by two strings (`directory` then `name`, joined
/// into a 260-byte buffer with `00406d30` / `00406d50`, opened with the
/// mode string at `0x01082cb8`) line by line, up to 512 bytes a line. A
/// line that does not start with `#` and is longer than one character loses
/// a trailing newline and is looked up (`00462f40`) in the object in the
/// global `0x011c3f2c`; a hit is passed to `00471d00` with 1. Returns 1 if
/// any line hit. The stack-cookie check is left out.
pub fn fn_00872430(e: &mut Engine, _this: Ptr, directory: u32, name: u32) -> u8 {
    let mut found = 0u8;
    e.with_stack(0x104, |e, path| {
        let path = path.addr();
        e.call(0x0040_6d30, &args![path, 0x104u32, directory]);
        e.call(0x0040_6d50, &args![path, 0x104u32, name]);
        let file = e.call(0x00ec_9a47, &args![path, 0x0108_2cb8u32]).u32();
        if file == 0 {
            return;
        }
        e.with_stack(0x200, |e, line| {
            let line = line.addr();
            while e.call(0x00ec_c104, &args![line, 0x200u32, file]).u32() != 0 {
                let length = e.call(0x0044_a670, &args![line]).u32();
                if e.mem.i8(line) as i32 != 0x23 && length > 1 {
                    if e.mem.i8(line + length - 1) as i32 == 0xa {
                        e.mem.set_u8(line + length - 1, 0);
                    }
                    let list = e.mem.u32(0x011c_3f2c);
                    let entry = e.call(0x0046_2f40, &args![list, line]).u32();
                    if entry != 0 {
                        e.call(0x0047_1d00, &args![entry, 1u32]);
                        found = 1;
                    }
                }
            }
        });
        e.call(0x00ec_9907, &args![file]);
    });
    found
}

// Translated from 00872570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frame housekeeping, cdecl; always returns 1. With `release` set: waits
/// until `0086bdf0` is true (sleeping `0040fca0(5)` between checks), stops
/// the object in `0x0126fac4` (`00ec29b0`, `00ec1800`) when `00ec17c0` says
/// so and `00ec25e0` when `00872780` does, runs `00b631d0` on the object
/// from `00b4f5c0`, clears the word at `+4` of the second out value of
/// `005477f0` for every cell slot of the world's grid, calls
/// `Main::KillMenuBGTexture` (`00877430`, Xbox PDB), and, when `00446e10`,
/// resizes the game window to the display size (`GetWindowLongA`,
/// `AdjustWindowRectEx`, `SetWindowPos`). Without it: `00bad4a0` when the
/// byte `0x011f941e` is set, and the water system calls `004e6620(1, 0)`
/// and `004e65d0` on the world's `0070ec90` object when `005d2a40`.
pub fn fn_00872570(e: &mut Engine, release: u8) -> u8 {
    if release != 0 {
        while !e.call(0x0086_bdf0, &args![]).bool() {
            e.call(0x0040_fca0, &args![5u32]);
        }
        let movie_player = e.mem.u32(0x0126_fac4);
        if e.call(0x00ec_17c0, &args![movie_player]).bool() {
            e.call(0x00ec_29b0, &args![movie_player]);
            e.call(0x00ec_1800, &args![movie_player]);
        }
        let movie_player = e.mem.u32(0x0126_fac4);
        if fn_00872780(e, Ptr::new(movie_player)) != 0 {
            e.call(0x00ec_25e0, &args![movie_player]);
        }
        let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
        if accumulator != 0 {
            e.call(0x00b6_31d0, &args![accumulator]);
        }
        let world = e.mem.u32(WORLD_OBJECT);
        let grid = e.call(0x0044_ddc0, &args![world]).u32();
        let size = e.call(0x0084_e3a0, &args![grid]).i32();
        for x in 0..size {
            for y in 0..size {
                let world = e.mem.u32(WORLD_OBJECT);
                let slot = e.call(0x0045_7050, &args![world, x, y]).u32();
                let cell = e.mem.u32(slot);
                if cell != 0 {
                    let (first, second) = e.with_stack(8, |e, out| {
                        let at = out.addr();
                        e.call(0x0054_77f0, &args![cell, at, at + 4]);
                        (e.mem.u32(at), e.mem.u32(at + 4))
                    });
                    if first != 0 && second != 0 && e.mem.u32(second + 4) != 0 {
                        e.mem.set_u32(second + 4, 0);
                    }
                }
            }
        }
        let main_object = e.mem.u32(MAIN_OBJECT);
        e.call(0x0087_7430, &args![main_object]);
        if e.call(0x0044_6e10, &args![]).bool() {
            resize_window_to_display(e);
        }
    } else {
        if e.global::<u8>(0x011f_941e) != 0 {
            e.call(0x00ba_d4a0, &args![]);
        }
        let world = e.mem.u32(WORLD_OBJECT);
        if e.call(0x0070_ec90, &args![world]).u32() != 0 && e.call(0x005d_2a40, &args![]).bool() {
            let world = e.mem.u32(WORLD_OBJECT);
            let water = e.call(0x0070_ec90, &args![world]).u32();
            e.call(0x004e_6620, &args![water, 1u32, 0u32]);
            let world = e.mem.u32(WORLD_OBJECT);
            let water = e.call(0x0070_ec90, &args![world]).u32();
            e.call(0x004e_65d0, &args![water]);
        }
    }
    1
}

/// The window resize of `00872570`: a rectangle `(0, 0, width, height)`
/// (`0x0118947c`, `0x01189480`) is grown for the window style and the
/// window set to it.
fn resize_window_to_display(e: &mut Engine) {
    e.with_stack(0x10, |e, rect| {
        let rect = rect.addr();
        e.mem.set_i32(rect, 0);
        e.mem.set_i32(rect + 4, 0);
        let width = e.call(0x004d_c1f0, &args![]).u32();
        e.mem.set_u32(rect + 8, width);
        let height = e.call(0x004d_c200, &args![]).u32();
        e.mem.set_u32(rect + 12, height);
        let main_object = e.mem.u32(MAIN_OBJECT);
        let window = e.mem.u32(main_object + MAIN_WINDOW);
        // GWL_STYLE is -16.
        let style = e
            .call(IMPORT_GET_WINDOW_LONG, &args![window, -0x10i32])
            .u32();
        e.call(
            IMPORT_ADJUST_WINDOW_RECT_EX,
            &args![rect, style, 0u32, 0u32],
        );
        let left = e.mem.i32(rect);
        let top = e.mem.i32(rect + 4);
        let right = e.mem.i32(rect + 8);
        let bottom = e.mem.i32(rect + 12);
        let main_object = e.mem.u32(MAIN_OBJECT);
        let window = e.mem.u32(main_object + MAIN_WINDOW);
        // As compiled: X = left, Y = bottom, cx = right - left, cy = top - bottom.
        e.call(
            IMPORT_SET_WINDOW_POS,
            &args![
                window,
                0u32,
                left,
                bottom,
                right.wrapping_sub(left),
                top.wrapping_sub(bottom),
                0u32
            ],
        );
    });
}

// Translated from 00872780 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object at `+0x0c` exists and `008727b0` holds for it.
pub fn fn_00872780(e: &mut Engine, this: Ptr) -> u8 {
    let inner = e.mem.u32(this.addr() + 0x0c);
    if inner != 0 {
        fn_008727b0(e, Ptr::new(inner))
    } else {
        0
    }
}

// Translated from 008727b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the word at `+0x20` is non-zero.
pub fn fn_008727b0(e: &mut Engine, this: Ptr) -> u8 {
    (e.mem.u32(this.addr() + 0x20) != 0) as u8
}

// Translated from 008727d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the byte setting `0x011c7adc` set: sets the word `0x011a9608` to 1
/// for the duration, and either (when `005468f0(0xb)` is false and the
/// world object has no `005f36f0` result or `004518e0` holds for it) has
/// `004e1bc0` of the world's `0070ec90` object draw with the indexed global
/// 0 flagged by `005f9af0`, or resets the state (`0x011ff104`,
/// `0x011ff8c4`, the first word of the `0070ec90` object, the `Main`
/// object's `004e2190` / `004e2120`, `00872930(0)`). Always ends by
/// advancing the stages after the first word of that object up to `0x10` on
/// both threads.
pub fn fn_008727d0(e: &mut Engine, _this: Ptr) {
    if setting_byte(e, 0x011c_7adc) != 0 {
        let saved = e.global::<u32>(0x011a_9608);
        e.set_global(0x011a_9608, 1u32);
        let blocked = e.call(0x0054_68f0, &args![0xbu32]).bool();
        let first_path = !blocked && {
            let world = e.mem.u32(WORLD_OBJECT);
            if e.call(0x005f_36f0, &args![world]).u32() == 0 {
                true
            } else {
                let world = e.mem.u32(WORLD_OBJECT);
                let object = e.call(0x005f_36f0, &args![world]).u32();
                e.call(0x0045_18e0, &args![object]).bool()
            }
        };
        if first_path {
            let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            let flag = e.call(0x005f_9ad0, &args![indexed]).u8();
            e.call(0x005f_9af0, &args![indexed, 1u32]);
            let child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
            let world = e.mem.u32(WORLD_OBJECT);
            let cell = e.call(0x0070_ec90, &args![world]).u32();
            e.call(0x004e_1bc0, &args![cell, child, indexed]);
            e.call(0x005f_9af0, &args![indexed, flag as u32]);
        } else {
            e.set_global(0x011f_f104, 0u8);
            let world = e.mem.u32(WORLD_OBJECT);
            let cell = e.call(0x0070_ec90, &args![world]).u32();
            e.mem.set_u32(cell, 0);
            let main_object = e.mem.u32(MAIN_OBJECT);
            let water = e.call(0x004e_2190, &args![main_object]).u32();
            e.call(0x004e_2120, &args![water, 0u32]);
            e.set_global(0x011f_f8c4, 0u8);
            let world = e.mem.u32(WORLD_OBJECT);
            e.call(0x0070_ec90, &args![world]);
            fn_00872930(e, 0);
        }
        e.set_global(0x011a_9608, saved);
    }
    let world = e.mem.u32(WORLD_OBJECT);
    let cell = e.call(0x0070_ec90, &args![world]).u32();
    let first = e.mem.i32(cell);
    let mut stage = first.wrapping_add(1);
    while stage < 0x11 {
        mt_both_threads(e, stage as u32);
        stage += 1;
    }
}

// Translated from 00872930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte in the global `0x011c7a59` (cdecl).
pub fn fn_00872930(e: &mut Engine, value: u8) {
    e.set_global(0x011c_7a59, value);
}

// Translated from 00872940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The draw sequence of a frame that only needs the interface: builds the
/// frame (`00872f50`), draws the menu pass (`00874b90`), calls the update
/// routine of each visible menu (`0x41e` to `00709ae0`, `0x3f6` to
/// `00709af0`, `0x438` to `00709b00`, `0x439` to `00709b10`, `0x43a` to
/// `00709b20`, `0x43b` to `00709b30`, `0x432` to `007948d0`), finishes with
/// `00876850`, and calls `007c9ca0` when menu `0x424` is visible.
pub fn fn_00872940(e: &mut Engine, this: Ptr, target: u32) {
    let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
    let global_91ac = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
    let byte_flag = e.call(0x004d_c310, &args![]).u8();
    fn_00872ad0(e, this);
    let frame = fn_00872f50(e, this, renderer, target, 0, byte_flag);
    e.call(
        0x0087_4b90,
        &args![this, global_91ac, renderer, byte_flag, target],
    );
    for (menu, update) in [
        (0x41eu32, 0x0070_9ae0u32),
        (0x3f6, 0x0070_9af0),
        (0x438, 0x0070_9b00),
        (0x439, 0x0070_9b10),
        (0x43a, 0x0070_9b20),
        (0x43b, 0x0070_9b30),
        (0x432, 0x0079_48d0),
    ] {
        if is_menu_visible(e, menu) {
            e.call(update, &args![]);
        }
    }
    if fn_008709f0(e) != 0 {
        e.call(
            0x0087_5fd0,
            &args![this, byte_flag, frame, renderer, target],
        );
    } else if target != 0 {
        e.call(0x0087_6830, &args![this, byte_flag, target]);
    }
    e.call(
        0x0087_6850,
        &args![this, frame, global_91ac, renderer, 0u32],
    );
    if is_menu_visible(e, 0x424) {
        e.call(0x007c_9ca0, &args![]);
    }
}

// Translated from 00872ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `00683a60` returns something: calls `00a81a80` if `00714a00` says
/// so, then `00714960(3)`.
pub fn fn_00872ad0(e: &mut Engine, _this: Ptr) {
    if e.call(0x0068_3a60, &args![]).u32() != 0 {
        if e.call(0x0071_4a00, &args![]).bool() {
            e.call(0x00a8_1a80, &args![]);
        }
        e.call(0x0071_4960, &args![3u32]);
    }
}

// Translated from 00872b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame camera and value setup; returns the object that the callers
/// pass to `00873200` as the world accumulator. In order: sets the root
/// child's value to the constant at `0x011ad840`; unless `0044ddc0` of
/// `0x011f2250` is 4, calls `00c52020` on the root with the float of
/// `00710ab0` of the player object (and `0`, `0`, `1`); calls `00b54000`
/// with that float; when the byte `+0x9c` of `this` is set, sets the root
/// child's value to `(0, 1, 1, 0)`; with the byte setting `0x011ded80` set,
/// constructs the static `HighActorCuller` at `0x011def10` on first use
/// (`00872e00`, `atexit` of `0x00fd9380`) and runs `008d6f50` on it with
/// the root pointer's child; resets the values of the `NiPointer` globals
/// `0x011deb34` / `0x011deda4`; moves `0x011f9fc0` to `0x011f9fc1`;
/// stores two settings through `00872dd0` / `00872df0`; clears
/// `0x011f94a5`; and returns the result of `0045cd60` on the world's
/// `008d8520` object when its `00476c90` is 2 or 3, else 0.
pub fn fn_00872b00(e: &mut Engine, this: Ptr) -> u32 {
    let root = e.call(GET_ROOT, &args![]).u32();
    let root_child = e.call(GET_ROOT_CHILD, &args![root]).u32();
    e.call(SET_COLOR, &args![root_child, 0x011a_d840u32]);
    if e.call(0x0044_ddc0, &args![0x011f_2250u32]).u32() != 4 {
        let player = e.mem.u32(PLAYER_OBJECT);
        let field_of_view = e.call(0x0071_0ab0, &args![player]).f32();
        let root = e.call(GET_ROOT, &args![]).u32();
        e.call(0x00c5_2020, &args![root, field_of_view, 0u32, 0u32, 1u32]);
    }
    let player = e.mem.u32(PLAYER_OBJECT);
    let field_of_view = e.call(0x0071_0ab0, &args![player]).f32();
    e.call(0x00b5_4000, &args![field_of_view]);
    if e.mem.u8(this.addr() + MAIN_RENDERING_MENU) != 0 {
        e.with_stack(0x10, |e, color| {
            let at = color.addr();
            color_ctor(e, at, 0.0, 1.0, 1.0, 0.0);
            let root = e.call(GET_ROOT, &args![]).u32();
            let root_child = e.call(GET_ROOT_CHILD, &args![root]).u32();
            e.call(SET_COLOR, &args![root_child, at]);
        });
    }
    let root_pointer = pointer_get(e, ROOT_POINTER);
    let accumulator = e.call(GET_ROOT_CHILD, &args![root_pointer]).u32();
    // The compiled function has a local that stays 0 here, so the
    // `HighActorCuller` below always gets `accumulator`.
    if setting_byte(e, 0x011d_ed80) != 0 {
        let guard = e.global::<u32>(0x011d_efb4);
        if guard & 1 == 0 {
            e.set_global(0x011d_efb4, guard | 1);
            fn_00872e00(e, Ptr::new(0x011d_ef10));
            e.call(0x00ec_658f, &args![0x00fd_9380u32]);
        }
        e.call(0x008d_6f50, &args![0x011d_ef10u32, accumulator]);
    }
    for slot in [0x011d_eb34u32, 0x011d_eda4] {
        let root_pointer = pointer_get(e, ROOT_POINTER);
        let node = e.call(0x0055_8310, &args![root_pointer]).u32();
        let value = e.call(0x0043_c490, &args![node]).u32();
        let target = pointer_get(e, slot);
        e.call(0x0044_0460, &args![target, value]);
        e.with_stack(0x10, |e, point| {
            e.call(POINT3_CTOR, &args![point, 0.0f32, 0u32, 0u32]);
            let target = pointer_get(e, slot);
            e.call(0x00a5_9c60, &args![target, point]);
        });
    }
    if e.global::<u8>(0x011f_9fc0) != 0 {
        e.set_global(0x011f_9fc0, 0u8);
        e.set_global(0x011f_9fc1, 1u8);
    }
    let first = setting_byte(e, 0x011d_ecd8);
    fn_00872dd0(e, first, 1);
    let second = setting_byte(e, 0x011d_eb60);
    fn_00872df0(e, second);
    e.set_global(0x011f_94a5, 0u8);

    let mut result = 0u32;
    let world = e.mem.u32(WORLD_OBJECT);
    let highest = e.call(0x008d_8520, &args![world]).u32();
    if highest != 0 && e.call(0x0045_cd60, &args![highest]).u32() != 0 {
        let world = e.mem.u32(WORLD_OBJECT);
        let object = e.call(0x008d_8520, &args![world]).u32();
        let mode = e.call(0x0047_6c90, &args![object]).u32();
        let mode_matches = if mode == 3 {
            true
        } else {
            let world = e.mem.u32(WORLD_OBJECT);
            let object = e.call(0x008d_8520, &args![world]).u32();
            e.call(0x0047_6c90, &args![object]).u32() == 2
        };
        if mode_matches {
            let world = e.mem.u32(WORLD_OBJECT);
            let object = e.call(0x008d_8520, &args![world]).u32();
            result = e.call(0x0045_cd60, &args![object]).u32();
        }
    }
    result
}

// Translated from 00872dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores two bytes in the globals `0x011f9fc3` and `0x011f9fc4` (cdecl).
pub fn fn_00872dd0(e: &mut Engine, first: u8, second: u8) {
    e.set_global(0x011f_9fc3, first);
    e.set_global(0x011f_9fc4, second);
}

// Translated from 00872df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in the global `0x011f9fc5` (cdecl).
pub fn fn_00872df0(e: &mut Engine, value: u8) {
    e.set_global(0x011f_9fc5, value);
}

// Translated from 00872e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `HighActorCuller` (RTTI `.?AVHighActorCuller@@`, vtable
/// `0x01082cc0`): the base constructor `005d8b10`, the vtable, the
/// embedded `BSFadeNodeCuller` at `+0x08` (`00872ea0`) and three members
/// at `+0x98`, `+0x9c`, `+0xa0` (`00633c90` with 0). Returns `this`.
pub fn fn_00872e00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x005d_8b10, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_2cc0);
    fn_00872ea0(e, Ptr::new(this.addr() + 0x08), 0);
    for offset in [0x98, 0x9c, 0xa0] {
        e.call(0x0063_3c90, &args![this.addr() + offset, 0u32]);
    }
    this
}

// Translated from 00872ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSFadeNodeCuller` (RTTI `.?AVBSFadeNodeCuller@@`, vtable
/// `0x01082ccc`): the base constructor `00a69400` with `size`, then the
/// vtable. Returns `this`.
pub fn fn_00872ea0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(0x00a6_9400, &args![this, size]);
    e.mem.set_u32(this.addr(), 0x0108_2ccc);
    this
}

// Translated from 00872ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::AppendVirtual` (Xbox PDB): the body is a single
/// `DebugBreak()`; the one stack argument is not read.
pub fn ni_culling_process_append_virtual(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    e.call(IMPORT_DEBUG_BREAK, &args![]);
}

// Translated from 00872ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNodeCuller::GetRTTI` (Xbox PDB): the address `0x01201f7c`.
pub fn bs_fade_node_culler_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_1f7c
}

// Translated from 00872f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNodeCuller::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body (`00872f30`) and, with bit 0 of `flags` set, frees the
/// object (`00401030`, cdecl). Returns `this`.
pub fn bs_fade_node_culler_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00872f30(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00872f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSFadeNodeCuller` (inferred from its caller): the
/// base destructor `00a693e0`.
pub fn fn_00872f30(e: &mut Engine, this: Ptr) {
    e.call(0x00a6_93e0, &args![this]);
}

// Translated from 00872f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Selects the frame's render target. When `008709f0` is set and
/// `skip_new_target` is 0: creates a target for `renderer` (`00b6e110`, kind
/// 4), records it in `0x011f9438`, and if the target's width or height
/// differs from the renderer's, sets the root child's value to the
/// (clamped) ratios (`set_ratio_color`); then makes the target's group
/// current (`00b6b8d0(7, group)`). Otherwise makes `screen`'s group
/// current (`00b6b260(screen)`, or slot `0xc8` of `renderer` when
/// `use_renderer_group`), or with no `screen` the default (`00b6b890(7,
/// 0)`). Returns the created target (0 when none).
pub fn fn_00872f50(
    e: &mut Engine,
    _this: Ptr,
    renderer: u32,
    screen: u32,
    skip_new_target: u8,
    use_renderer_group: u8,
) -> u32 {
    let mut created = 0u32;
    if fn_008709f0(e) != 0 && skip_new_target == 0 {
        with_profile_scope(e, 0x27, 0x1e6d, |e| {
            let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
            created = e
                .call(
                    0x00b6_e110,
                    &args![factory, renderer, 4u32, 0u32, 0u32, 0u32],
                )
                .u32();
            e.call(0x0087_31d0, &args![created]);
            e.set_global(0x011f_9438, created);
            let width = e.call(0x004d_ee10, &args![renderer]).u32();
            let target_width = fn_00872370(e, Ptr::new(created), 0);
            let mut changed = width != target_width;
            if !changed {
                let height = e.call(0x004d_ee70, &args![renderer]).u32();
                let target_height = fn_008723d0(e, Ptr::new(created), 0);
                changed = height != target_height;
            }
            if changed {
                let scale: f64 = e.global(0x0108_2d20);
                let width = e.call(0x004d_ee10, &args![renderer]).u32();
                let target_width = fn_00872370(e, Ptr::new(created), 0);
                let ratio = (width as f64 * scale) / target_width as f64;
                let width_ratio = e.call(FLOAT_HELPER_C, &args![ratio, 1.0f32]).f64() as f32;
                let height = e.call(0x004d_ee70, &args![renderer]).u32();
                let target_height = fn_008723d0(e, Ptr::new(created), 0);
                let ratio = (height as f64 * scale) / target_height as f64;
                let height_ratio = e.call(FLOAT_HELPER_C, &args![ratio, 1.0f32]).f64() as f32;
                set_ratio_color(e, width_ratio, height_ratio);
            }
            let group = e.call(0x00b6_b260, &args![created]).u32();
            e.call(0x00b6_b8d0, &args![7u32, group]);
        });
    } else {
        with_profile_scope(e, 0x26, 0x1e84, |e| {
            if screen != 0 {
                if use_renderer_group != 0 {
                    let group = e.vcall(renderer, 0xc8, &args![]).u32();
                    e.call(0x00b6_b8d0, &args![7u32, group]);
                } else {
                    let group = e.call(0x00b6_b260, &args![screen]).u32();
                    e.call(0x00b6_b8d0, &args![7u32, group]);
                }
            } else {
                e.call(0x00b6_b890, &args![7u32, 0u32]);
            }
        });
    }
    created
}

// ---------------------------------------------------------------------------
// Second session: `008731a0` to `00877260`.
//
// Conventions of this session, besides those above:
//
// - `00705910` returns the interface object the draw routines pass to
//   `FOVATSEffectManager::DrawTargetToTexture` (`00800f30`); it takes no
//   argument.
// - `00950bb0(player, n)` returns a node of the player; `00456610` reads a
//   flag byte of a node and `00450f90` sets it.
// - `0045c670` returns the root, `006629f0` its child, `008d80e0` the
//   object the scene is accumulated into (the Xbox PDB names that body
//   `DialoguePackage::GetTargetOfConversation`, a folded name).
// - A callee's stack arguments follow its `RET n`: a word pushed before a
//   getter whose `RET` pops nothing belongs to the call that follows it.
// - Locals the game passes by address (lists, the culling process, vectors,
//   `NiPointer`s) are blocks of [`Engine::with_stack`].
// - The security-cookie check and the exception frames are not translated.
// ---------------------------------------------------------------------------

/// `BSShaderUtil::AccumulateScene` (`00b6bee0`, Xbox PDB), cdecl: three
/// words.
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
/// `BSShaderUtil::AccumulateSceneList` (`00b6bfc0`, Xbox PDB), cdecl: scene,
/// list, culling process.
const ACCUMULATE_SCENE_LIST: u32 = 0x00b6_bfc0;
/// `BSSceneGraph::SetCameraFOV` (`00c52020`, Xbox PDB): `this`, the field
/// of view and three more words.
const SET_CAMERA_FOV: u32 = 0x00c5_2020;
/// `MTRenderingSystem::AddAccumTask` (`00ba3390`, Xbox PDB), on the
/// multithreaded-rendering system, nine words.
const ADD_ACCUM_TASK: u32 = 0x00ba_3390;
/// `FOVATSEffectManager::DrawTargetToTexture` (`00800f30`, Xbox PDB), on
/// the object `00705910` returns; six words.
const DRAW_TARGET_TO_TEXTURE: u32 = 0x0080_0f30;
/// The interface object (`00705910`): the result of `00602170` on the
/// object `004b7210` returns when its `009373f0` flag is set, else 0.
const GET_INTERFACE_OBJECT: u32 = 0x0070_5910;
/// Returns the pointer in the global `0x011f9508` (an object with vtable
/// slots `0xac`, `0xb0` and `0xb4`).
const GET_GLOBAL_011F9508: u32 = 0x004a_0e90;
/// The player's node for the argument (`00950bb0`).
const GET_PLAYER_NODE: u32 = 0x0095_0bb0;
/// The scene object `008d80e0` returns for the root.
const GET_SCENE_TARGET: u32 = 0x008d_80e0;
/// `NiPointer<T>::operator=(T*)` (`0066b0d0`): `this` is the pointer slot.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer` copy constructor from a slot address (`00559a40`).
const NI_POINTER_COPY_CTOR: u32 = 0x0055_9a40;
/// `NiPointer` constructor from a pointer (`00633c90`).
const NI_POINTER_CTOR: u32 = 0x0063_3c90;
/// `NiPointer` destructor (`0045cec0`).
const NI_POINTER_DTOR: u32 = 0x0045_cec0;
/// Sets a flag byte of a node (`00450f90`).
const SET_NODE_FLAG: u32 = 0x0045_0f90;
/// Reads that flag byte (`00456610`).
const GET_NODE_FLAG: u32 = 0x0045_6610;
/// The object a rendered menu keeps at `+4` (`007fa950`).
const GET_MENU_SCREEN: u32 = 0x007f_a950;
/// `Interface::IsInPipboyMenu` is [`IS_IN_PIPBOY_MENU`]; the byte test of
/// the actor (`008c51c0`, `Actor::IsRefractive` in the Xbox PDB).
const ACTOR_FLAG: u32 = 0x008c_51c0;
/// The constant colour the draw routines hand to `00712e60`.
const CONSTANT_COLOR: u32 = 0x011a_d840;
/// The `NiPointer` global `0x011deb38`: the offscreen render target of the
/// 1st-person pass.
const FIRST_PERSON_TARGET_POINTER: u32 = 0x011d_eb38;
/// The 3 floats at `0x011f426c` (a zero vector the code resets nodes to).
const ZERO_VECTOR: u32 = 0x011f_426c;
/// Scene-node accessors: the child count (`0043b480`), the child at an
/// index (`0043b4a0`) and the child for a slot (`0045bc00`).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
const CHILD_FOR_SLOT: u32 = 0x0045_bc00;
/// `NiTPointerList` constructor, `AddTail`, `RemoveAll` and destructor
/// (`0048f200`, `004ed8c0`, `004ed900`, `004a1a30`).
const LIST_CTOR: u32 = 0x0048_f200;
const LIST_PUSH: u32 = 0x004e_d8c0;
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
const LIST_DTOR: u32 = 0x004a_1a30;

fn player(e: &Engine) -> u32 {
    e.mem.u32(PLAYER_OBJECT)
}

fn node_flag(e: &mut Engine, node: u32) -> u8 {
    e.call(GET_NODE_FLAG, &args![node]).u8()
}

/// `00950bb0(player, n)`.
fn player_node(e: &mut Engine, n: u32) -> u32 {
    let player = player(e);
    e.call(GET_PLAYER_NODE, &args![player, n]).u32()
}

/// Whether the renderer test of the draw routines holds: the global
/// `0x011f9426` is set and the renderer's vtable slot `0xcc` answers 0 or
/// the same as its slot `0xc8`. Reads the renderer (`0043c4b0`) the way the
/// code does.
fn renderer_color_applies(e: &mut Engine) -> bool {
    if e.global::<u8>(0x011f_9426) == 0 {
        return false;
    }
    let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
    if e.vcall(renderer, 0xcc, &args![]).u32() == 0 {
        return true;
    }
    let first = e.call(GET_GLOBAL_011F4748, &args![]).u32();
    let second = e.call(GET_GLOBAL_011F4748, &args![]).u32();
    let wanted = e.vcall(second, 0xc8, &args![]).u32();
    e.vcall(first, 0xcc, &args![]).u32() == wanted
}

/// The field of view the draw routines pass to `SetCameraFOV`.
fn camera_fov(e: &mut Engine) -> f32 {
    let player = player(e);
    fn_00874900(e, Ptr::new(player))
}

/// `SetCameraFOV(root, fov, 0, 0, 0)` then `00b54000(fov)`.
fn set_camera_fov(e: &mut Engine, root: u32) {
    let fov = camera_fov(e);
    e.call(SET_CAMERA_FOV, &args![root, fov, 0u32, 0u32, 0u32]);
    let fov = camera_fov(e);
    e.call(0x00b5_4000, &args![fov]);
}

/// `007fa950` on the rendered menu `00707ad0` returns.
fn rendered_menu_screen(e: &mut Engine) -> u32 {
    let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    e.call(GET_MENU_SCREEN, &args![menu]).u32()
}

// Translated from 008731a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl float helper: the smaller of `value` and `limit` (`limit` when they
/// are equal or unordered), returned as a double in `ST0`. (The decompiler
/// casts `value` to a float; the code compares with the double.)
pub fn fn_008731a0(_e: &mut Engine, value: f64, limit: f32) -> f64 {
    if limit as f64 > value {
        value
    } else {
        limit as f64
    }
}

// Translated from 008731d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// If the `NiPointer` at `+0x20` holds an object, passes it to `006e5cc0`
/// (`NiPointer<PathingDebugGeometryData>::operator_`, the Xbox PDB name of a
/// shared body) on the member at `+0x08`.
pub fn fn_008731d0(e: &mut Engine, this: Ptr) {
    if pointer_get(e, this.addr() + 0x20) != 0 {
        e.call(0x006e_5cc0, &args![this.addr() + 0x08, this.addr() + 0x20]);
    }
}

/// Appends `value` to the `NiTPointerList` at `list` (`004ed8c0` takes the
/// address of a cell holding the pointer).
fn list_push(e: &mut Engine, list: u32, cell: u32, value: u32) {
    e.mem.set_u32(cell, value);
    e.call(LIST_PUSH, &args![list, cell]);
}

/// Appends `node` to `list` when it is not null and its flag byte is clear.
fn push_unflagged(e: &mut Engine, list: u32, cell: u32, node: u32) {
    if node != 0 && node_flag(e, node) == 0 {
        list_push(e, list, cell, node);
    }
}

/// Appends `value` to `list_a` when `index` is odd, else to `list_b`.
fn push_by_parity(e: &mut Engine, lists: (u32, u32), cell: u32, index: u32, value: u32) {
    if index & 1 != 0 {
        list_push(e, lists.0, cell, value);
    } else {
        list_push(e, lists.1, cell, value);
    }
}

/// The scene sorting of `00873200` when the global `0x011f94b4` is set:
/// the children of the indexed global 0 are distributed over three lists
/// (`a` for odd positions, `b` for even ones, `c` for the fixed children),
/// the multithreaded task takes `a`, and `AccumulateSceneList` takes `b`
/// and `c` through a `BSCullingProcess`.
fn sort_and_accumulate_scene(e: &mut Engine, this_addr: u32, target: u32, child: u32) {
    e.with_stack(0x110, |e, frame| {
        let frame = frame.addr();
        let list_a = frame;
        let list_b = frame + 0x0c;
        let list_c = frame + 0x18;
        let cell = frame + 0x24;
        let iter_a = frame + 0x28;
        let iter_b = frame + 0x2c;
        let culling = frame + 0x40;
        let lists = (list_a, list_b);
        for list in [list_a, list_b, list_c] {
            e.call(LIST_CTOR, &args![list]);
        }
        let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        let mut group = 0u32;
        let node = e.call(CHILD_FOR_SLOT, &args![indexed, 0u32]).u32();
        push_unflagged(e, list_c, cell, node);
        let node = e.call(CHILD_FOR_SLOT, &args![indexed, 1u32]).u32();
        push_unflagged(e, list_c, cell, node);
        let node = e.call(CHILD_FOR_SLOT, &args![indexed, 2u32]).u32();
        if node != 0 && node_flag(e, node) == 0 {
            group = e.vcall(node, 0xc, &args![]).u32();
        }
        if group != 0 {
            for slot in 0..3u32 {
                let node = e.call(CHILD_FOR_SLOT, &args![group, slot]).u32();
                push_unflagged(e, list_a, cell, node);
            }
        }
        let node = e.call(CHILD_FOR_SLOT, &args![indexed, 3u32]).u32();
        group = e.vcall(node, 0xc, &args![]).u32();
        let world = e.mem.u32(WORLD_OBJECT);
        let interior = e.call(0x005f_36f0, &args![world]).u32();
        if interior == 0 {
            // Exterior: two levels of groups, sorted by the outer index.
            if group != 0 && node_flag(e, group) == 0 {
                let mut i = 0u32;
                while i < e.call(CHILD_COUNT, &args![group]).u32() {
                    let outer = e.call(CHILD_AT, &args![group, i]).u32();
                    if outer != 0 {
                        let middle = e.vcall(outer, 0xc, &args![]).u32();
                        if middle != 0 && node_flag(e, middle) == 0 {
                            let mut j = 0u32;
                            while j < e.call(CHILD_COUNT, &args![middle]).u32() {
                                let item = e.call(CHILD_AT, &args![middle, j]).u32();
                                if item != 0 {
                                    let inner = e.vcall(item, 0xc, &args![]).u32();
                                    if inner == 0 {
                                        push_by_parity(e, lists, cell, i, item);
                                    } else if node_flag(e, inner) == 0 {
                                        let mut k = (j == 6) as u32;
                                        while k < e.call(CHILD_COUNT, &args![inner]).u32() {
                                            let leaf = e.call(CHILD_AT, &args![inner, k]).u32();
                                            if leaf != 0 && node_flag(e, leaf) == 0 {
                                                push_by_parity(e, lists, cell, i, leaf);
                                            }
                                            k += 1;
                                        }
                                    }
                                }
                                j += 1;
                            }
                        }
                    }
                    i += 1;
                }
            }
        } else {
            let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            if fn_00874650(e, Ptr::new(indexed_zero)) != 0 {
                let node = e.call(CHILD_FOR_SLOT, &args![group, 0u32]).u32();
                push_unflagged(e, list_b, cell, node);
                let node = e.call(CHILD_FOR_SLOT, &args![group, 1u32]).u32();
                push_unflagged(e, list_a, cell, node);
                let node = e.call(CHILD_FOR_SLOT, &args![group, 2u32]).u32();
                if node != 0 {
                    group = e.vcall(node, 0xc, &args![]).u32();
                    if group != 0 && node_flag(e, group) == 0 {
                        let mut i = 0u32;
                        while i < e.call(CHILD_COUNT, &args![group]).u32() {
                            let outer = e.call(CHILD_AT, &args![group, i]).u32();
                            if outer != 0 && i != 4 && i != 0 {
                                let middle = e.vcall(outer, 0xc, &args![]).u32();
                                if middle != 0 && node_flag(e, middle) == 0 {
                                    let mut k = (i == 6) as u32;
                                    while k < e.call(CHILD_COUNT, &args![middle]).u32() {
                                        let leaf = e.call(CHILD_AT, &args![middle, k]).u32();
                                        if leaf != 0 && node_flag(e, leaf) == 0 {
                                            push_by_parity(e, lists, cell, k, leaf);
                                        }
                                        k += 1;
                                    }
                                }
                            }
                            i += 1;
                        }
                    }
                }
            } else {
                let node = e.call(CHILD_FOR_SLOT, &args![group, 2u32]).u32();
                if node != 0 {
                    group = e.vcall(node, 0xc, &args![]).u32();
                    if group != 0 && node_flag(e, group) == 0 {
                        let node = e.call(CHILD_FOR_SLOT, &args![group, 7u32]).u32();
                        if node != 0 {
                            group = e.vcall(node, 0xc, &args![]).u32();
                            if group != 0 {
                                let mut i = 0u32;
                                while i < e.call(CHILD_COUNT, &args![group]).u32() {
                                    let leaf = e.call(CHILD_AT, &args![group, i]).u32();
                                    if leaf != 0 && node_flag(e, leaf) == 0 {
                                        push_by_parity(e, lists, cell, i, leaf);
                                    }
                                    i += 1;
                                }
                            }
                        }
                    }
                }
            }
            // The renderable objects of the indexed global's tree.
            let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            let owner = e.call(0x0045_4b30, &args![indexed_zero]).u32();
            let list_head = if owner != 0 {
                e.call(0x0043_6aa0, &args![owner]).u32()
            } else {
                0
            };
            let first = if list_head != 0 {
                pointer_get(e, list_head)
            } else {
                0
            };
            let mut counter = 0u32;
            e.mem.set_u32(iter_a, first);
            while e.mem.u32(iter_a) != 0 {
                let list = e.call(0x0043_6aa0, &args![owner]).u32();
                let slot = e.call(0x0057_cbe0, &args![list, iter_a]).u32();
                let holder = pointer_get(e, slot);
                if holder != 0 {
                    let members = e.call(0x006a_b360, &args![holder]).u32();
                    if e.call(0x0076_b610, &args![members]).u8() == 0 {
                        let members = e.call(0x006a_b360, &args![holder]).u32();
                        let first_member = pointer_get(e, members);
                        if first_member != 0 {
                            e.mem.set_u32(iter_b, first_member);
                            while e.mem.u32(iter_b) != 0 {
                                let members = e.call(0x006a_b360, &args![holder]).u32();
                                let slot = e.call(0x0057_cbe0, &args![members, iter_b]).u32();
                                let item = e.mem.u32(slot);
                                if item != 0 {
                                    let geometry = e.call(0x0045_c4c0, &args![item]).u32();
                                    if e.call(0x00c4_f320, &args![target, geometry]).u8() != 0 {
                                        let geometry = e.call(0x0045_c4c0, &args![item]).u32();
                                        if geometry != 0 {
                                            if fn_00874480(e, Ptr::new(item)) != 0
                                                && e.call(0x009b_4440, &args![item]).u32() != 0
                                            {
                                                let shared =
                                                    e.call(0x0084_e3a0, &args![target]).u32();
                                                let indexed_zero =
                                                    e.call(INDEXED_GLOBAL, &args![0u32]).u32();
                                                e.call(
                                                    0x00b5_ba80,
                                                    &args![indexed_zero, shared, item],
                                                );
                                            }
                                            push_by_parity(e, lists, cell, counter, geometry);
                                            counter += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            if fn_00874650(e, Ptr::new(indexed_zero)) != 0 {
                let list = e.call(0x007d_6bb0, &args![owner]).u32();
                let first = pointer_get(e, list);
                e.mem.set_u32(iter_a, first);
                while e.mem.u32(iter_a) != 0 {
                    let list = e.call(0x007d_6bb0, &args![owner]).u32();
                    let slot = e.call(0x0057_cbe0, &args![list, iter_a]).u32();
                    let item = e.mem.u32(slot);
                    if item != 0 {
                        let geometry = e.call(0x0045_c4c0, &args![item]).u32();
                        if e.call(0x00c4_f320, &args![target, geometry]).u8() != 0 {
                            let geometry = e.call(0x0045_c4c0, &args![item]).u32();
                            if geometry != 0 {
                                push_by_parity(e, lists, cell, counter, geometry);
                                counter += 1;
                            }
                        }
                    }
                }
            }
            let tes_object = e.mem.u32(WORLD_OBJECT);
            let current = e.call(0x0045_7070, &args![tes_object]).u32();
            let portal_graph = e.call(0x009d_9f20, &args![current]).u32();
            if portal_graph != 0 {
                let mut i = 0u32;
                while i < e.call(0x00c5_b5d0, &args![portal_graph]).u32() {
                    let always = e.call(0x00c5_b5a0, &args![portal_graph, i]).u32();
                    if always != 0 {
                        list_push(e, list_c, cell, always);
                    }
                    i += 1;
                }
            }
        }
        // The fixed children and the tail of the indexed global's children.
        let node = e.call(CHILD_FOR_SLOT, &args![indexed, 4u32]).u32();
        push_unflagged(e, list_c, cell, node);
        let node = e.call(CHILD_FOR_SLOT, &args![indexed, 5u32]).u32();
        push_unflagged(e, list_c, cell, node);
        let node = e.call(CHILD_FOR_SLOT, &args![indexed, 6u32]).u32();
        push_unflagged(e, list_a, cell, node);
        let mut i = 7u32;
        while i < e.call(CHILD_COUNT, &args![indexed]).u32() {
            let node = e.call(CHILD_AT, &args![indexed, i]).u32();
            if node != 0 && node_flag(e, node) == 0 {
                push_by_parity(e, lists, cell, i, node);
            }
            i += 1;
        }
        e.call(0x0041_fd00, &args![target, child]);
        let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        e.call(0x00b5_e870, &args![indexed_zero, target]);
        let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        if fn_00874670(e, Ptr::new(indexed_zero)).addr() != 0 {
            let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            let operations = fn_00874670(e, Ptr::new(indexed_zero)).addr();
            e.call(0x00c4_9850, &args![operations]);
        }
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        e.vcall(accumulator, 0x8c, &args![child]);
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        e.call(0x00b5_4ac0, &args![accumulator]);
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        e.call(0x004d_c540, &args![renderer, accumulator]);
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        let operations = fn_00874670(e, Ptr::new(indexed_zero)).addr();
        let system = e.call(GET_MT_SYSTEM, &args![]).u32();
        e.call(
            ADD_ACCUM_TASK,
            &args![
                system,
                child,
                operations,
                0u32,
                list_a,
                0u32,
                accumulator,
                0u32,
                0x15u32,
                1u32
            ],
        );
        mt_set_stage(e, 0, 0x15);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2bc, 0);
        e.call(0x004a_0eb0, &args![culling, 0u32]);
        e.call(0x0041_fd00, &args![culling, child]);
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        e.call(0x004a_0fd0, &args![culling, accumulator]);
        let indexed_zero = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        let operations = fn_00874670(e, Ptr::new(indexed_zero)).addr();
        fn_00874460(e, Ptr::new(culling), operations);
        e.call(ACCUMULATE_SCENE_LIST, &args![child, list_b, culling]);
        fn_00874460(e, Ptr::new(culling), 0);
        e.call(ACCUMULATE_SCENE_LIST, &args![child, list_c, culling]);
        mt_advance_stage(e, 1, 0x15);
        e.call(LIST_REMOVE_ALL, &args![list_a]);
        e.call(LIST_REMOVE_ALL, &args![list_b]);
        e.call(0x004a_0f60, &args![culling]);
        e.call(LIST_DTOR, &args![list_c]);
        e.call(LIST_DTOR, &args![list_b]);
        e.call(LIST_DTOR, &args![list_a]);
    });
}

/// The two short passes after the world pass: the same sequence with the
/// objects of `00874690` (`second` false) and of `008746a0` / `008746b0`
/// (`second` true) as the scene lists.
fn extra_pass(e: &mut Engine, this_addr: u32, world_target: u32, second: bool) {
    fn_008746c0(e);
    e.call(GET_GLOBAL_011F9508, &args![]);
    let root = e.call(GET_ROOT, &args![]).u32();
    let target = e.call(GET_SCENE_TARGET, &args![root]).u32();
    e.call(0x004f_b0b0, &args![target, 1u32]);
    let root_pointer = pointer_get(e, ROOT_POINTER);
    let child = e.call(GET_ROOT_CHILD, &args![root_pointer]).u32();
    e.call(0x00b8_03b0, &args![child]);
    let first = if second {
        fn_008746b0(e)
    } else {
        fn_00874690(e)
    };
    e.call(0x0041_fd00, &args![target, first]);
    let accumulator = pointer_get(e, this_addr + 0x94);
    e.call(0x004a_0fd0, &args![target, accumulator]);
    if second {
        let middle = fn_008746a0(e);
        let list = fn_008746b0(e);
        e.call(ACCUMULATE_SCENE, &args![list, middle, target]);
    } else {
        let middle = e.call(0x005b_ac50, &args![]).u32();
        let list = fn_00874690(e);
        e.call(ACCUMULATE_SCENE, &args![list, middle, target]);
    }
    let accumulator = pointer_get(e, this_addr + 0x94);
    let list = if second {
        fn_008746b0(e)
    } else {
        fn_00874690(e)
    };
    e.call(0x00b6_c0d0, &args![list, accumulator, 0u32]);
    e.call(0x004a_0fd0, &args![world_target, 0u32]);
    e.call(0x004f_b0b0, &args![target, 0u32]);
}

// Translated from 00873200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world pass of the main draw sequence. `this` is the `Main` object.
/// `object` is an optional object that gets `00642860` first and
/// `006426a0` at the end; `skip_pass` clears the flag byte `+0x38` of the
/// world accumulator (and keeps `+0x3a` clear); `overlay` draws the
/// overlay target (`00800f30`) after the scene.
///
/// With the global `0x011f94b4` clear the scene is accumulated directly;
/// otherwise it is sorted into three lists by
/// [`sort_and_accumulate_scene`]. The wind-down of the exception frame and
/// the security-cookie check are not translated.
pub fn fn_00873200(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    _unused_1: u32,
    skip_pass: u8,
    overlay: u8,
) {
    let this_addr = this.addr();
    let root = e.call(GET_ROOT, &args![]).u32();
    let target = e.call(GET_SCENE_TARGET, &args![root]).u32();
    let root_child = e.call(GET_ROOT_CHILD, &args![root]).u32();
    e.set_global(0x011f_9684, root_child);
    if object != 0 {
        e.call(0x0064_2860, &args![object]);
    }
    let node = player_node(e, 1);
    let saved_flag = node_flag(e, node);
    let node = player_node(e, 1);
    e.call(SET_NODE_FLAG, &args![node, 1u32]);
    let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
    e.mem.set_u8(accumulator + 0x38, (skip_pass == 0) as u8);
    let water = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
    let texture = e.call(0x004e_bbc0, &args![water, 4u32]).u32();
    let use_texture = skip_pass == 0 && fn_00871220(e, Ptr::new(texture));
    let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
    e.mem.set_u8(accumulator + 0x3a, use_texture as u8);
    let use_texture = skip_pass == 0 && fn_00871220(e, Ptr::new(texture));
    let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
    e.mem.set_u8(accumulator + 0x3a, use_texture as u8);

    e.with_stack(0x10, |e, guard| {
        let guard = guard.addr();
        e.call(
            PROFILE_SCOPE_CTOR,
            &args![guard, 0xfu32, 1u32, SOURCE_FILE_NAME, 0x1eb7u32],
        );
        let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.call(0x0041_fd00, &args![target, child]);
        let frustum = e.call(MEMBER_8C_B, &args![child]).u32();
        e.call(0x00a6_94a0, &args![target, frustum]);
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        e.call(0x004a_0fd0, &args![target, accumulator]);
        if e.global::<u8>(0x011f_94b4) == 0 {
            e.call(ACCUMULATE_SCENE, &args![child, root, target]);
        } else {
            sort_and_accumulate_scene(e, this_addr, target, child);
        }
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        let root_child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.call(0x00b6_ba20, &args![root_child, accumulator, 1u32]);
        if overlay != 0 {
            e.call(0x00b9_8380, &args![0u32, 1u32]);
            let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
            e.call(
                DRAW_TARGET_TO_TEXTURE,
                &args![interface, 0u32, 2u32, 1u32, 0u32, 0u32, 0u32],
            );
            e.call(0x004e_ced0, &args![1u32]);
        }
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        let root_child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.call(0x00b6_b930, &args![root_child, accumulator, 1u32]);
        if e.global::<u8>(0x011f_94b4) != 0 {
            let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            e.call(0x00a8_9af0, &args![indexed, target]);
        }
        e.call(0x004a_0fd0, &args![target, 0u32]);
        e.call(0x0040_4f70, &args![guard]);
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        e.mem.set_u8(accumulator + 0x38, 0);
        let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
        e.mem.set_u8(accumulator + 0x3a, 0);
        extra_pass(e, this_addr, target, false);
        extra_pass(e, this_addr, target, true);
        if setting_dword(e, SETTING_THREADS) > 1 {
            e.call(0x008c_80e0, &args![0u32]);
        }
        if e.global::<u8>(0x011f_94a9) != 0 {
            let root_child = e.call(GET_ROOT_CHILD, &args![root]).u32();
            fn_008744a0(e, root_child);
        }
        if object != 0 {
            let root_pointer = pointer_get(e, ROOT_POINTER);
            let root_child = e.call(GET_ROOT_CHILD, &args![root_pointer]).u32();
            let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
            e.call(0x0064_26a0, &args![object, 1u32, accumulator, root_child]);
        }
        fn_00872dd0(e, 0, 0);
        let node = player_node(e, 1);
        e.call(SET_NODE_FLAG, &args![node, saved_flag as u32]);
        e.call(PROFILE_SCOPE_DTOR, &args![guard]);
    });
}

// Translated from 00874460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at `+0xc0`.
pub fn fn_00874460(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xc0, value);
}

// Translated from 00874480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xfc`.
pub fn fn_00874480(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xfc)
}

// Translated from 008744a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl. With a null `object` copies the three floats of `0x011a9490`
/// to `0x012003d4` and clears the byte `0x012003cc`. Otherwise reads the
/// position of `object` (the vector at its `+0x8c`) and a second vector
/// (`0045bba0`), subtracts the previously stored pair (`0x012003e0` and
/// `0x012003d4`, `00439ef0`), divides each difference by the stored value
/// (`00416870` builds the results) and sets `0x012003cc` when any
/// component of either quotient exceeds 0.01 (the double at
/// `0x01031148`); then stores the two vectors for the next call.
pub fn fn_008744a0(e: &mut Engine, object: u32) {
    if object == 0 {
        for (from, to) in [
            (0x011a_9490u32, 0x0120_03d4u32),
            (0x011a_9494, 0x0120_03d8),
            (0x011a_9498, 0x0120_03dc),
        ] {
            let word: u32 = e.global(from);
            e.set_global(to, word);
        }
        e.set_global(0x0120_03cc, 0u8);
        return;
    }
    let member = e.call(MEMBER_8C, &args![object]).u32();
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        let position = frame;
        let second = frame + 0x10;
        let difference_a = frame + 0x20;
        let difference_b = frame + 0x30;
        let quotient_a = frame + 0x40;
        let quotient_b = frame + 0x50;
        for k in 0..3 {
            let word = e.mem.u32(member + 4 * k);
            e.mem.set_u32(position + 4 * k, word);
        }
        e.call(0x0045_bba0, &args![object, second]);
        e.call(0x0043_9ef0, &args![position, difference_a, 0x0120_03e0u32]);
        e.call(0x0043_9ef0, &args![second, difference_b, 0x0120_03d4u32]);
        let quotient = |e: &Engine, from: u32, k: u32, global: u32| -> f32 {
            let numerator = e.mem.f32(from + 4 * k) as f64;
            let divisor = e.global::<f32>(global) as f64;
            (numerator / divisor) as f32
        };
        let x = quotient(e, difference_a, 0, 0x0120_03e0);
        let y = quotient(e, difference_a, 1, 0x0120_03e4);
        let z = quotient(e, difference_a, 2, 0x0120_03e8);
        e.call(0x0041_6870, &args![quotient_a, x, y, z]);
        let x = quotient(e, difference_b, 0, 0x0120_03d4);
        let y = quotient(e, difference_b, 1, 0x0120_03d8);
        let z = quotient(e, difference_b, 2, 0x0120_03dc);
        e.call(0x0041_6870, &args![quotient_b, x, y, z]);
        let limit: f64 = e.global(0x0103_1148);
        let mut changed = false;
        for (block, k) in [
            (quotient_a, 0),
            (quotient_a, 1),
            (quotient_a, 2),
            (quotient_b, 0),
            (quotient_b, 1),
            (quotient_b, 2),
        ] {
            if limit < e.mem.f32(block + 4 * k) as f64 {
                changed = true;
                break;
            }
        }
        e.set_global(0x0120_03cc, changed as u8);
        for k in 0..3 {
            let word = e.mem.u32(position + 4 * k);
            e.set_global(0x0120_03e0 + 4 * k, word);
            let word = e.mem.u32(second + 4 * k);
            e.set_global(0x0120_03d4 + 4 * k, word);
        }
    });
}

// Translated from 00874650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x1dc`.
pub fn fn_00874650(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x1dc)
}

// Translated from 00874670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `this + 0x138`.
pub fn fn_00874670(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0x138)
}

// Translated from 00874690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object held by the `NiPointer` global `0x011c7840` (`00559450`).
pub fn fn_00874690(e: &mut Engine) -> u32 {
    pointer_get(e, 0x011c_7840)
}

// Translated from 008746a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object held by the `NiPointer` global `0x011c7810`.
pub fn fn_008746a0(e: &mut Engine) -> u32 {
    pointer_get(e, 0x011c_7810)
}

// Translated from 008746b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object held by the `NiPointer` global `0x011c781c`.
pub fn fn_008746b0(e: &mut Engine) -> u32 {
    pointer_get(e, 0x011c_781c)
}

// Translated from 008746c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word `0x011ff0f0`.
pub fn fn_008746c0(e: &mut Engine) {
    e.set_global(0x011f_f0f0, 0u32);
}

// Translated from 008746d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pass that draws the player's node into the offscreen interface
/// target: when the actor is aiming (`008bbc10` on the player) the
/// borrowed render target is made current first (and released at the end).
/// One stack word is not read.
pub fn fn_008746d0(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let water = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
    let decal = e.call(GET_GLOBAL_011F9508, &args![]).u32();
    let node = player_node(e, 1);
    let root = e.call(GET_ROOT, &args![]).u32();
    let target = e.call(GET_SCENE_TARGET, &args![root]).u32();
    let player_value = player(e);
    let aiming = e.call(0x008b_bc10, &args![player_value]).u8();
    e.with_stack(0x10, |e, color| {
        let color = color.addr();
        color_ctor(e, color, 0.0, 0.0, 0.0, 0.0);
        if aiming == 0 {
            e.call(0x00b9_8380, &args![0u32, 1u32]);
        } else {
            let borrowed = e.call(0x004e_bbc0, &args![water, 4u32]).u32();
            let texture = fn_008748d0(e, Ptr::new(borrowed));
            e.call(0x00b6_b790, &args![]);
            e.vcall(decal, 0xb4, &args![color]);
            e.vcall(decal, 0xb0, &args![0x011f_4998u32]);
            let group = e.call(0x00b6_b260, &args![texture]).u32();
            e.call(0x00b6_b8d0, &args![1u32, group]);
        }
        let saved_flag = node_flag(e, node);
        e.call(SET_NODE_FLAG, &args![node, 0u32]);
        let player_value = player(e);
        let fov = fn_00874900(e, Ptr::new(player_value));
        e.call(SET_CAMERA_FOV, &args![root, fov, 0u32, 0u32, 0u32]);
        let player_value = player(e);
        let fov = fn_00874900(e, Ptr::new(player_value));
        e.call(0x00b5_4000, &args![fov]);
        let accumulator = pointer_get(e, this.addr() + 0x90);
        e.call(0x004a_0fd0, &args![target, accumulator]);
        let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.call(ACCUMULATE_SCENE, &args![child, node, target]);
        let accumulator = pointer_get(e, this.addr() + 0x90);
        let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.call(0x00b6_c0d0, &args![child, accumulator, 0u32]);
        e.call(0x004a_0fd0, &args![target, 0u32]);
        let player_value = player(e);
        let fov = e.call(0x0071_0ab0, &args![player_value]).f32();
        e.call(SET_CAMERA_FOV, &args![root, fov, 0u32, 0u32, 0u32]);
        let player_value = player(e);
        let fov = e.call(0x0071_0ab0, &args![player_value]).f32();
        e.call(0x00b5_4000, &args![fov]);
        e.call(SET_NODE_FLAG, &args![node, saved_flag as u32]);
        if aiming == 0 {
            e.call(0x004e_ced0, &args![1u32]);
        } else {
            e.vcall(decal, 0xac, &args![color]);
            e.call(0x00b6_b790, &args![]);
        }
    });
}

// Translated from 008748d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::BorrowRenderedTexture` (`00ba3840`, Xbox PDB) with
/// the argument `0x27` on the member at `+0x7c`, then
/// `ImageSpaceTexture::GetRenderedTexture` (`00ba3770`) on it.
pub fn fn_008748d0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x00ba_3840, &args![this.addr() + 0x7c, 0x27u32]);
    e.call(0x00ba_3770, &args![this.addr() + 0x7c]).u32()
}

// Translated from 00874900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0x674`, returned in `ST0`.
pub fn fn_00874900(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x674)
}

// Translated from 00874920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under a profiler scope (kind `0x27`, line `0x20d5`), accumulates the
/// root's child into `accumulator` (`00b64570`, with `target`). `this` is
/// not read.
pub fn fn_00874920(e: &mut Engine, _this: Ptr, accumulator: u32, target: u32) {
    with_profile_scope(e, 0x27, 0x20d5, |e| {
        let root = e.call(GET_ROOT, &args![]).u32();
        let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.call(0x00b6_4570, &args![accumulator, child, target]);
    });
}

// Translated from 008749b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The two overlay draws of the offscreen target. Needs the interface
/// object (`00705910`) to hold a pointer (`005585e0`); creates the
/// render target of the global `0x011deb38` (`00b6e110` with `0x2f`) when it
/// is empty; then, with the interface's own pointer empty, the target set
/// and the interface byte `+0x118` set (`00874aa0`), calls
/// `DrawTargetToTexture` twice with the target and returns 1.
pub fn fn_008749b0(e: &mut Engine, _this: Ptr) -> u8 {
    let mut drew = 0u8;
    let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
    if e.call(0x0055_85e0, &args![interface]).u32() == 0 {
        return drew;
    }
    if pointer_get(e, FIRST_PERSON_TARGET_POINTER) == 0 {
        let renderer = e.call(GET_GLOBAL_011F4748, &args![]).u32();
        let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
        let created = e
            .call(
                0x00b6_e110,
                &args![factory, renderer, 0x2fu32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.call(
            NI_POINTER_ASSIGN,
            &args![FIRST_PERSON_TARGET_POINTER, created],
        );
    }
    let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
    if pointer_get(e, interface) == 0 && pointer_get(e, FIRST_PERSON_TARGET_POINTER) != 0 {
        let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
        if e.call(0x0055_85e0, &args![interface]).u32() != 0 {
            let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
            if fn_00874aa0(e, Ptr::new(interface)) != 0 {
                let target = pointer_get(e, FIRST_PERSON_TARGET_POINTER);
                let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
                e.call(
                    DRAW_TARGET_TO_TEXTURE,
                    &args![interface, target, 0u32, 1u32, 0u32, 7u32, 0u32],
                );
                let target = pointer_get(e, FIRST_PERSON_TARGET_POINTER);
                let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
                e.call(
                    DRAW_TARGET_TO_TEXTURE,
                    &args![interface, target, 1u32, 1u32, 0u32, 1u32, 1u32],
                );
                drew = 1;
            }
        }
    }
    drew
}

// Translated from 00874aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x118`.
pub fn fn_00874aa0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x118)
}

// Translated from 00874ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// If the interface object (`00705910`) holds a pointer (`005585e0`): makes
/// the stage `00b6b790`, sets `00b98380(0, 1)`, draws the target of
/// `0x011deb38` to the texture, `004eced0(1)`, and draws it a second time.
/// One stack word is not read.
pub fn fn_00874ac0(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
    if e.call(0x0055_85e0, &args![interface]).u32() != 0 {
        e.call(0x00b6_b790, &args![]);
        e.call(0x00b9_8380, &args![0u32, 1u32]);
        let target = pointer_get(e, FIRST_PERSON_TARGET_POINTER);
        let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
        e.call(
            DRAW_TARGET_TO_TEXTURE,
            &args![interface, target, 0u32, 0u32, 1u32, 7u32, 0u32],
        );
        e.call(0x004e_ced0, &args![1u32]);
        let target = pointer_get(e, FIRST_PERSON_TARGET_POINTER);
        let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
        e.call(
            DRAW_TARGET_TO_TEXTURE,
            &args![interface, target, 0u32, 0u32, 0u32, 1u32, 1u32],
        );
    }
}

// Translated from 00874b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// If `004e9510` holds: makes the group of `texture` current
/// (`00b6b260` then `00b6b8d0(group_slot, group)`). `this` and the
/// second word are not read.
pub fn fn_00874b50(e: &mut Engine, _this: Ptr, texture: u32, _unused_1: u32, slot: u32) {
    if e.call(0x004e_9510, &args![]).u8() != 0 {
        e.call(GET_GLOBAL_011F91AC, &args![]);
        e.call(GET_GLOBAL_011F9508, &args![]);
        let group = e.call(0x00b6_b260, &args![texture]).u32();
        e.call(0x00b6_b8d0, &args![slot, group]);
    }
}

// Translated from 00874b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// If the `NiPointer` global `0x011ded3c` holds a target: draws it into
/// the image-space pass (`00b97550` on the first stack word, with `0x22`,
/// the second word, the target, `mask` unless `skip_mask` is set, 0 and
/// 1). `this` is not read.
pub fn fn_00874b90(e: &mut Engine, _this: Ptr, first: u32, second: u32, skip_mask: u8, mask: u32) {
    if pointer_get(e, MENU_TARGET_POINTER) != 0 {
        e.call(0x00b9_8380, &args![7u32, 1u32]);
        e.call(0x00b9_7fa0, &args![0u32, 1u32]);
        let mask = if skip_mask != 0 { 0 } else { mask };
        let target = pointer_get(e, MENU_TARGET_POINTER);
        e.call(
            0x00b9_7550,
            &args![first, 0x22u32, second, target, mask, 0u32, 1u32],
        );
        e.call(0x0071_4ae0, &args![1u32]);
        e.call(0x004e_ced0, &args![1u32]);
    }
}

// Translated from 00874c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::DrawWorld_MTSetup1stPerson` (Xbox PDB): the 1st-person pass of
/// the multithreaded draw. Under a profiler scope (kind `0x34`, line
/// `0x21b1`): prepares the player's node (flag cleared, colour), and when
/// exactly one of the node's two flags is set (and `005bb4d0` on the
/// `Main` object is clear) moves the camera by the node's position
/// (`004a0bd0`, `004f00e0`, `00688430`...), records the position through
/// `00870790` and `008750e0`, sets `0x011de9d1`; otherwise clears
/// `0x011de9d1`. Finally queues the accumulation task (`00ba3390`) unless
/// a rendered menu other than the pipboy is current, and sets the thread
/// stage 0 to `0x16`.
pub fn main_draw_world_mt_setup_1st_person(e: &mut Engine, this: Ptr) {
    let this_addr = this.addr();
    with_profile_scope(e, 0x34, 0x21b1, |e| {
        let root = e.call(GET_ROOT, &args![]).u32();
        let root_child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
        let node = player_node(e, 1);
        e.call(SET_NODE_FLAG, &args![node, 0u32]);
        if renderer_color_applies(e) {
            let child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
            e.call(SET_COLOR, &args![child, CONSTANT_COLOR]);
        }
        let fov = camera_fov(e);
        e.call(SET_CAMERA_FOV, &args![root, fov, 0u32, 0u32, 0u32]);
        let first = {
            let n = player_node(e, 0);
            node_flag(e, n)
        };
        let enter = if first == 0 && {
            let n = player_node(e, 1);
            node_flag(e, n) != 0
        } {
            true
        } else {
            let y = {
                let n = player_node(e, 0);
                node_flag(e, n)
            };
            if y == 0 {
                false
            } else {
                let n = player_node(e, 1);
                node_flag(e, n) == 0
            }
        };
        let main_object = e.mem.u32(MAIN_OBJECT);
        if enter && e.call(0x005b_b4d0, &args![main_object]).u8() == 0 {
            e.with_stack(0x100, |e, frame| {
                let frame = frame.addr();
                let out_a = frame;
                let point = frame + 0x10;
                let block = frame + 0x20;
                let sum_a = frame + 0x60;
                let sum_b = frame + 0x70;
                let sum_c = frame + 0x80;
                let saved = frame + 0x90;
                let member = e.call(MEMBER_8C, &args![node]).u32();
                e.call(0x004a_0bd0, &args![member, out_a]);
                e.call(POINT3_CTOR, &args![point, 0u32, 0u32, 0u32]);
                e.set_global(0x011d_e9d1, 1u8);
                let value = e.call(0x0064_47f0, &args![root_child]).f32();
                let camera = pointer_get(e, this_addr + 0xa0);
                e.call(0x0050_7700, &args![camera, value]);
                let frustum = e.call(MEMBER_8C_B, &args![root_child]).u32();
                let camera = pointer_get(e, this_addr + 0xa0);
                e.call(0x00a6_faf0, &args![camera, frustum]);
                let color = e.call(0x004a_0d10, &args![root_child]).u32();
                let camera = pointer_get(e, this_addr + 0xa0);
                e.call(SET_COLOR, &args![camera, color]);
                let source = e.call(0x0046_1130, &args![root_child]).u32();
                for k in 0..13 {
                    let word = e.mem.u32(source + 4 * k);
                    e.mem.set_u32(block + 4 * k, word);
                }
                let other = e.mem.u32(0x011e_07d0);
                let member = e.call(MEMBER_8C, &args![other]).u32();
                let sum = e.call(0x0043_9e90, &args![member, sum_a, out_a]).u32();
                let camera = pointer_get(e, this_addr + 0xa0);
                e.call(0x004f_00e0, &args![camera, sum]);
                let camera = pointer_get(e, this_addr + 0xa0);
                e.call(0x0068_8430, &args![camera, block]);
                let player_value = player(e);
                if e.call(0x0077_21a0, &args![player_value]).u32() != 0 {
                    let player_value = player(e);
                    if e.call(0x008a_8870, &args![player_value]).u8() == 0
                        && e.call(0x0052_5430, &args![0x011f_2250u32]).u8() == 0
                    {
                        let player_value = player(e);
                        let held = e.call(0x0077_21a0, &args![player_value]).u32();
                        let member = e.call(MEMBER_8C, &args![held]).u32();
                        let sum = e.call(0x0043_9e90, &args![member, sum_b, out_a]).u32();
                        let camera = pointer_get(e, this_addr + 0xa0);
                        e.call(0x004f_00e0, &args![camera, sum]);
                    }
                }
                let position = e.call(0x0043_c490, &args![node]).u32();
                for k in 0..3 {
                    let word = e.mem.u32(position + 4 * k);
                    e.mem.set_u32(saved + 4 * k, word);
                }
                e.call(0x0044_0460, &args![node, ZERO_VECTOR]);
                e.call(0x00a5_9c60, &args![node, point]);
                e.call(0x0044_0460, &args![node, saved]);
                let player_value = player(e);
                let pipboy_node = if e.call(0x0096_7ae0, &args![player_value]).u8() != 0 {
                    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
                    e.call(0x0068_38b0, &args![pipboy]).u32()
                } else {
                    0
                };
                e.call(0x0054_6780, &args![node, 1u32]);
                if pipboy_node != 0 {
                    let member = e.call(MEMBER_8C, &args![pipboy_node]).u32();
                    let difference = e.call(0x0043_9ef0, &args![member, sum_c, out_a]).u32();
                    e.call(0x004f_00e0, &args![pipboy_node, difference]);
                }
                let camera = pointer_get(e, this_addr + 0xa0);
                let words = e.call(MEMBER_8C, &args![camera]).u32();
                let words = [e.mem.u32(words), e.mem.u32(words + 4), e.mem.u32(words + 8)];
                let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
                fn_00870790(e, Ptr::new(indexed), words[0], words[1], words[2]);
                let words = [e.mem.u32(out_a), e.mem.u32(out_a + 4), e.mem.u32(out_a + 8)];
                let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
                fn_008750e0(e, Ptr::new(indexed), words[0], words[1], words[2]);
            });
        } else {
            e.set_global(0x011d_e9d1, 0u8);
        }
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        let queue = if menu == 0 {
            true
        } else {
            let again = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
            let pipboy = e.call(GET_PIPBOY, &args![]).u32();
            again == pipboy
        };
        if queue {
            let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
            let camera = pointer_get(e, this_addr + 0xa0);
            let system = e.call(GET_MT_SYSTEM, &args![]).u32();
            e.call(
                ADD_ACCUM_TASK,
                &args![
                    system,
                    camera,
                    0u32,
                    node,
                    0u32,
                    0u32,
                    accumulator,
                    0u32,
                    0x16u32,
                    0u32
                ],
            );
        }
        mt_set_stage(e, 0, 0x16);
    });
}

// Translated from 008750e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three words (a position) at `+0x1e4`, `+0x1e8`, `+0x1ec`.
pub fn fn_008750e0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) {
    e.mem.set_u32(this.addr() + 0x1e4, first);
    e.mem.set_u32(this.addr() + 0x1e8, second);
    e.mem.set_u32(this.addr() + 0x1ec, third);
}

// Translated from 00875110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second half of the per-frame draw. Stack words: `interface_arg`
/// (for `007148c0`), `menu_node` (a node the rendered-menu pass clears the
/// flag of), `overlay_node` (an object whose holder, `0043b230`, is drawn
/// last) and `target_texture`. Under a profiler scope (kind `0x34`, line
/// `0x221f`) it sets the camera colour (the fixed colour when the
/// renderer test holds, else a built one), the field of view and the
/// frustum, then:
///
/// - with `menu_node` and a rendered menu on screen (`007079b0`): draws the
///   menu's scene into a new `BSShaderAccumulator` (`0x280` bytes,
///   constructed with `0x63, 1, 0x2f7`), with the three pipboy arrays
///   when the pipboy menu is up and `008c51c0` holds;
/// - unless a rendered menu other than the pipboy is current (or the
///   pipboy with `008c51c0`): the 1st-person block (`00b98380(0xf, 1)`,
///   stage `0x16` on thread 1, a walk of the list `0043c490` gives for the object at `0x011e0e80`,
///   whose entries that answer true at vtable slot `0xc0` have their
///   screen (slot `0x98`) accumulated);
/// - with `005b9b00` set, a nested scope (kind `0x27`, line `0x22b5`)
///   accumulates `target_texture` into the 1st-person accumulator;
/// - with the byte `0x011de9d1` set, resets the player node and records
///   the zero vector (`00870790`, `008750e0`);
/// - the mode field of view, the holder of `overlay_node` (when its flag
///   is clear), `006426a0` for it, and the field of view again.
pub fn fn_00875110(
    e: &mut Engine,
    this: Ptr,
    interface_arg: u32,
    menu_node: u32,
    overlay_node: u32,
    target_texture: u32,
) {
    let this_addr = this.addr();
    let root = e.call(GET_ROOT, &args![]).u32();
    let target = e.call(GET_SCENE_TARGET, &args![root]).u32();
    let node = player_node(e, 1);
    e.call(SET_NODE_FLAG, &args![node, 0u32]);
    let root_child = e.call(GET_ROOT_CHILD, &args![root]).u32();
    mt_advance_stage(e, 1, 0x16);
    let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
    e.mem.set_u8(accumulator + 0x3a, 0);
    e.with_stack(0x200, |e, frame| {
        let frame = frame.addr();
        let guard = frame;
        let color = frame + 0x10;
        let pointer_a = frame + 0x20;
        let pointer_b = frame + 0x24;
        let pointer_c = frame + 0x28;
        let guard_b = frame + 0x30;
        let zero = frame + 0x50;
        let vector = frame + 0x60;
        let out_a = frame + 0x70;
        let out_b = frame + 0x80;
        let temp = frame + 0x90;
        e.call(
            PROFILE_SCOPE_CTOR,
            &args![guard, 0x34u32, 1u32, SOURCE_FILE_NAME, 0x221fu32],
        );
        e.call(0x0071_48c0, &args![interface_arg, 0u32, 4u32]);
        if renderer_color_applies(e) {
            let camera = pointer_get(e, this_addr + 0xa0);
            e.call(SET_COLOR, &args![camera, CONSTANT_COLOR]);
        } else {
            color_ctor(e, color, 0.0, 1.0, 1.0, 0.0);
            let camera = pointer_get(e, this_addr + 0xa0);
            e.call(SET_COLOR, &args![camera, color]);
        }
        set_camera_fov(e, root);
        let frustum = e.call(MEMBER_8C_B, &args![root_child]).u32();
        let camera = pointer_get(e, this_addr + 0xa0);
        e.call(0x00a6_faf0, &args![camera, frustum]);

        if menu_node != 0 {
            let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
            e.call(SET_SCREEN_TEXTURE, &args![menu, 0u32]);
            e.call(SET_NODE_FLAG, &args![menu_node, 0u32]);
            if e.call(IS_IN_RENDERED_MENU, &args![]).u8() != 0 {
                if e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0 {
                    let player_value = player(e);
                    if e.call(ACTOR_FLAG, &args![player_value]).u8() != 0 {
                        let screen = rendered_menu_screen(e);
                        e.call(SET_NODE_FLAG, &args![screen, 1u32]);
                    }
                }
                let memory = e.call(0x00aa_13e0, &args![0x280u32]).u32();
                let accumulator = if memory != 0 {
                    e.call(0x00b6_60d0, &args![memory, 0x63u32, 1u32, 0x2f7u32])
                        .u32()
                } else {
                    0
                };
                e.call(NI_POINTER_CTOR, &args![pointer_a, accumulator]);
                e.call(0x00c4_f270, &args![target, 1u32]);
                let property = {
                    let zero_arg = e.call(0x0040_df90, &args![]).u32();
                    let screen = rendered_menu_screen(e);
                    e.call(0x00a5_9d30, &args![screen, zero_arg]).u32()
                };
                if property != 0 {
                    e.call(0x0049_ed90, &args![property, 0u32]);
                }
                let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
                e.call(SET_SCREEN_TEXTURE, &args![menu, 0u32]);
                let held = pointer_get(e, pointer_a);
                e.call(0x004a_0fd0, &args![target, held]);
                if e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0 {
                    let player_value = player(e);
                    if e.call(ACTOR_FLAG, &args![player_value]).u8() != 0 {
                        for index in 0..3u32 {
                            let pipboy = e.call(GET_PIPBOY, &args![]).u32();
                            let element = fn_00875bd0(e, Ptr::new(pipboy), index);
                            let camera = pointer_get(e, this_addr + 0xa0);
                            e.call(ACCUMULATE_SCENE, &args![camera, element, target]);
                        }
                    }
                }
                let screen = rendered_menu_screen(e);
                let camera = pointer_get(e, this_addr + 0xa0);
                e.call(ACCUMULATE_SCENE, &args![camera, screen, target]);
                let held = pointer_get(e, pointer_a);
                let camera = pointer_get(e, this_addr + 0xa0);
                e.call(0x00b6_c0d0, &args![camera, held, 0u32]);
                e.call(0x004a_0fd0, &args![target, 0u32]);
                e.call(NI_POINTER_ASSIGN, &args![pointer_a, 0u32]);
                e.call(0x00c4_f2d0, &args![target]);
                if e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0 {
                    let player_value = player(e);
                    if e.call(ACTOR_FLAG, &args![player_value]).u8() != 0 {
                        let screen = rendered_menu_screen(e);
                        e.call(SET_NODE_FLAG, &args![screen, 0u32]);
                    }
                }
                e.call(NI_POINTER_DTOR, &args![pointer_a]);
            }
        }

        // The pass for the 1st-person node, unless another rendered menu
        // than the pipboy is current.
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        let other_menu = menu != 0 && {
            let again = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
            let pipboy = e.call(GET_PIPBOY, &args![]).u32();
            again != pipboy
        };
        let skipped = other_menu || {
            e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0 && {
                let player_value = player(e);
                e.call(ACTOR_FLAG, &args![player_value]).u8() != 0
            }
        };
        if !skipped {
            e.call(0x00b9_8380, &args![0xfu32, 1u32]);
            mt_advance_stage(e, 1, 0x16);
            let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
            let camera = pointer_get(e, this_addr + 0xa0);
            e.call(0x00b6_c0d0, &args![camera, accumulator, 0u32]);
            e.call(0x004e_ced0, &args![1u32]);
            let group = e.call(0x0043_c490, &args![0x011e_0e80u32]).u32();
            if group != 0 && e.call(0x004a_4460, &args![group]).u8() == 0 {
                e.call(0x0041_6870, &args![zero, 0u32, 0u32, 0u32]);
                let player_value = player(e);
                if e.call(0x0077_21a0, &args![player_value]).u32() != 0 {
                    let player_value = player(e);
                    if e.call(0x008a_8870, &args![player_value]).u8() == 0
                        && e.call(0x0052_5430, &args![0x011f_2250u32]).u8() == 0
                    {
                        let other = e.mem.u32(0x011e_07d0);
                        let other_member = e.call(MEMBER_8C, &args![other]).u32();
                        let player_value = player(e);
                        let held = e.call(0x0077_21a0, &args![player_value]).u32();
                        let held_member = e.call(MEMBER_8C, &args![held]).u32();
                        let result = e
                            .call(0x0043_9ef0, &args![held_member, out_a, other_member])
                            .u32();
                        for k in 0..3 {
                            let word = e.mem.u32(result + 4 * k);
                            e.mem.set_u32(zero + 4 * k, word);
                        }
                    }
                }
                let root_b = e.call(GET_ROOT, &args![]).u32();
                let child_b = e.call(GET_ROOT_CHILD, &args![root_b]).u32();
                let target_b = e.call(GET_SCENE_TARGET, &args![root_b]).u32();
                e.call(NI_POINTER_COPY_CTOR, &args![pointer_b, this_addr + 0x88]);
                let member = e.call(MEMBER_8C, &args![child_b]).u32();
                for k in 0..3 {
                    let word = e.mem.u32(member + 4 * k);
                    e.mem.set_u32(vector + 4 * k, word);
                }
                let sum = e.call(0x0043_9e90, &args![vector, out_b, zero]).u32();
                e.call(0x004f_00e0, &args![child_b, sum]);
                let held = pointer_get(e, pointer_b);
                e.call(0x004a_0fd0, &args![target_b, held]);
                let held = pointer_get(e, pointer_b);
                e.call(0x0093_6aa0, &args![held, 1u32]);
                let mut flagged = false;
                let mut current = group;
                while current != 0 {
                    let link = e.call(0x0068_15c0, &args![current]).u32();
                    let entry = pointer_get(e, link);
                    current = e.call(0x0072_6070, &args![current]).u32();
                    if entry != 0 && e.vcall(entry, 0xc0, &args![]).u8() != 0 {
                        flagged = true;
                        let screen = e.vcall(entry, 0x98, &args![]).u32();
                        e.call(SET_NODE_FLAG, &args![screen, 0u32]);
                        let screen = e.vcall(entry, 0x98, &args![]).u32();
                        e.call(ACCUMULATE_SCENE, &args![child_b, screen, target_b]);
                        let screen = e.vcall(entry, 0x98, &args![]).u32();
                        e.call(SET_NODE_FLAG, &args![screen, 1u32]);
                    }
                }
                if flagged {
                    let held = pointer_get(e, pointer_b);
                    e.call(0x00b6_c0d0, &args![child_b, held, 0u32]);
                }
                e.call(0x004a_0fd0, &args![target_b, 0u32]);
                let held = pointer_get(e, pointer_b);
                e.call(0x0093_6aa0, &args![held, 0u32]);
                e.call(NI_POINTER_ASSIGN, &args![pointer_b, 0u32]);
                e.call(0x004f_00e0, &args![child_b, vector]);
                e.call(NI_POINTER_DTOR, &args![pointer_b]);
            }
        }

        if e.call(0x005b_9b00, &args![]).u8() != 0 {
            e.call(
                PROFILE_SCOPE_CTOR,
                &args![guard_b, 0x27u32, 1u32, SOURCE_FILE_NAME, 0x22b5u32],
            );
            if e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0
                && e.call(0x005b_9b00, &args![]).u8() != 0
            {
                let player_value = player(e);
                if e.call(ACTOR_FLAG, &args![player_value]).u8() != 0 {
                    e.call(0x00b6_b790, &args![]);
                    let held = e.mem.u32(0x011f_9438);
                    e.call(0x00a2_9680, &args![held]);
                    let group = e.call(0x00b6_b260, &args![target_texture]).u32();
                    e.call(0x00b6_b8d0, &args![6u32, group]);
                    if e.global::<u8>(0x011f_9426) != 0 {
                        let child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
                        e.call(SET_COLOR, &args![child, CONSTANT_COLOR]);
                    }
                }
            }
            e.call(GET_ROOT, &args![]);
            let camera = pointer_get(e, this_addr + 0xa0);
            let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
            e.call(0x00b6_4570, &args![accumulator, camera, target_texture]);
            fn_00874b50(e, this, target_texture, 0, 0);
            e.call(PROFILE_SCOPE_DTOR, &args![guard_b]);
        }

        if e.global::<u8>(0x011d_e9d1) != 0 {
            e.call(POINT3_CTOR, &args![temp, 0u32, 0u32, 0u32]);
            e.call(0x00a5_9c60, &args![node, temp]);
            e.call(0x0054_6780, &args![node, 0u32]);
            let owner = e.call(0x0096_11e0, &args![root_child]).u32();
            let position = e.call(0x0043_c490, &args![owner]).u32();
            let words = [
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            ];
            let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            fn_00870790(e, Ptr::new(indexed), words[0], words[1], words[2]);
            let words = [
                e.global::<u32>(ZERO_VECTOR),
                e.global::<u32>(ZERO_VECTOR + 4),
                e.global::<u32>(ZERO_VECTOR + 8),
            ];
            let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            fn_008750e0(e, Ptr::new(indexed), words[0], words[1], words[2]);
        }

        if e.call(0x0044_ddc0, &args![0x011f_2250u32]).u32() != 4 {
            let player_value = player(e);
            let fov = e.call(0x0071_0ab0, &args![player_value]).f32();
            let root_now = e.call(GET_ROOT, &args![]).u32();
            e.call(SET_CAMERA_FOV, &args![root_now, fov, 0u32, 0u32, 0u32]);
        }
        let player_value = player(e);
        let fov = e.call(0x0071_0ab0, &args![player_value]).f32();
        e.call(0x00b5_4000, &args![fov]);

        if overlay_node != 0 {
            let holder = e.call(0x0043_b230, &args![overlay_node]).u32();
            if node_flag(e, holder) == 0 {
                let holder = e.call(0x0043_b230, &args![overlay_node]).u32();
                let saved_flag = node_flag(e, holder);
                let holder = e.call(0x0043_b230, &args![overlay_node]).u32();
                e.call(SET_NODE_FLAG, &args![holder, 0u32]);
                let root_c = e.call(GET_ROOT, &args![]).u32();
                let child_c = e.call(GET_ROOT_CHILD, &args![root_c]).u32();
                let target_c = e.call(GET_SCENE_TARGET, &args![root_c]).u32();
                e.call(NI_POINTER_COPY_CTOR, &args![pointer_c, this_addr + 0x88]);
                let held = pointer_get(e, pointer_c);
                e.call(0x004a_0fd0, &args![target_c, held]);
                let held = pointer_get(e, pointer_c);
                e.call(0x0093_6aa0, &args![held, 1u32]);
                let holder = e.call(0x0043_b230, &args![overlay_node]).u32();
                e.call(ACCUMULATE_SCENE, &args![child_c, holder, target_c]);
                let held = pointer_get(e, pointer_c);
                e.call(0x00b6_c0d0, &args![child_c, held, 0u32]);
                e.call(0x004a_0fd0, &args![target_c, 0u32]);
                let held = pointer_get(e, pointer_c);
                e.call(0x0093_6aa0, &args![held, 0u32]);
                e.call(NI_POINTER_ASSIGN, &args![pointer_c, 0u32]);
                let holder = e.call(0x0043_b230, &args![overlay_node]).u32();
                e.call(SET_NODE_FLAG, &args![holder, saved_flag as u32]);
                e.call(NI_POINTER_DTOR, &args![pointer_c]);
            }
        }
        if overlay_node != 0 {
            let root_pointer = pointer_get(e, ROOT_POINTER);
            let child = e.call(GET_ROOT_CHILD, &args![root_pointer]).u32();
            let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
            e.call(0x0064_26a0, &args![overlay_node, 2u32, accumulator, child]);
        }
        set_camera_fov(e, root);
        e.call(PROFILE_SCOPE_DTOR, &args![guard]);
    });
}

// Translated from 00875bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object held by the `NiPointer` at `this + 0xf4 + index * 4`.
pub fn fn_00875bd0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    pointer_get(e, this.addr() + 0xf4 + index * 4)
}

// Translated from 00875bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pass that draws the current rendered menu into the menu
/// accumulator (`+0x98`). Does nothing when the pointer global
/// `0x011deb38` holds an object, when no rendered menu is current or when
/// it is the pipboy. Otherwise accumulates the menu's scene twice (the
/// vtable slot `0x08` of the menu gives the scene), with the render state
/// of the pipboy overlay between the two.
pub fn fn_00875bf0(e: &mut Engine, this: Ptr) {
    let this_addr = this.addr();
    if pointer_get(e, FIRST_PERSON_TARGET_POINTER) != 0 {
        return;
    }
    let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    if menu == 0 {
        return;
    }
    let again = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    if again == pipboy {
        return;
    }
    let root = e.call(GET_ROOT, &args![]).u32();
    let target = e.call(GET_SCENE_TARGET, &args![root]).u32();
    e.call(0x00c4_f270, &args![target, 1u32]);
    let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    let menu_node = if e.call(0x008b_6200, &args![menu]).u32() != 0 {
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        e.call(0x008b_6200, &args![menu]).u32()
    } else {
        e.call(GET_ROOT_CHILD, &args![root]).u32()
    };
    e.call(0x0041_fd00, &args![target, menu_node]);
    let accumulator = pointer_get(e, this_addr + MAIN_MENU_ACCUM);
    e.call(0x004a_0fd0, &args![target, accumulator]);
    let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
    let accumulator = pointer_get(e, this_addr + MAIN_MENU_ACCUM);
    e.call(0x004a_1020, &args![accumulator, indexed]);
    if renderer_color_applies(e) {
        e.call(SET_COLOR, &args![menu_node, CONSTANT_COLOR]);
    } else {
        e.with_stack(0x10, |e, color| {
            let color = color.addr();
            color_ctor(e, color, 0.0, 1.0, 1.0, 0.0);
            e.call(SET_COLOR, &args![menu_node, color]);
        });
    }
    let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    // The pushed target is the third word of the cdecl `00b6bee0` (`ADD ESP,0xc` after it, at
    // 00875d72 and 00875df5); the menu's +8 takes none (00875d6b, 00875dee).
    let scene = e.vcall(menu, 0x8, &args![]).u32();
    e.call(ACCUMULATE_SCENE, &args![menu_node, scene, target]);
    e.call(0x00b4_f450, &args![0xfu32]);
    e.call(0x00b9_7e30, &args![0u32, 1u32]);
    e.call(0x00b9_8380, &args![8u32, 1u32]);
    let accumulator = pointer_get(e, this_addr + MAIN_MENU_ACCUM);
    e.call(0x00b6_c0d0, &args![menu_node, accumulator, 0u32]);
    e.call(0x00b4_f450, &args![0u32]);
    e.call(0x004e_ced0, &args![1u32]);
    e.call(0x0071_4a60, &args![1u32]);
    let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
    // The pushed target is the third word of the cdecl `00b6bee0` (`ADD ESP,0xc` after it, at
    // 00875d72 and 00875df5); the menu's +8 takes none (00875d6b, 00875dee).
    let scene = e.vcall(menu, 0x8, &args![]).u32();
    e.call(ACCUMULATE_SCENE, &args![menu_node, scene, target]);
    let accumulator = pointer_get(e, this_addr + MAIN_MENU_ACCUM);
    e.call(0x00b6_c0d0, &args![menu_node, accumulator, 0u32]);
    e.call(0x004a_0fd0, &args![target, 0u32]);
    e.call(0x00c4_f2d0, &args![target]);
}

// Translated from 00875e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the field of view, then (under a profiler scope, kind `0x27`,
/// line `0x234c`) accumulates the root's child through `00b65550` with a
/// temporary object (`004ad0c0` / `004ad1d0`, built with `0x101`) and
/// destroys the object that child's vtable slot `0x48` returned (its slot
/// 0 with 1). `this` is not read.
pub fn fn_00875e40(e: &mut Engine, _this: Ptr, accumulator: u32, last: u32) {
    let root = e.call(GET_ROOT, &args![]).u32();
    if e.call(0x0044_ddc0, &args![0x011f_2250u32]).u32() != 4 {
        let player_value = player(e);
        let fov = e.call(0x0071_0ab0, &args![player_value]).f32();
        e.call(SET_CAMERA_FOV, &args![root, fov, 0u32, 0u32, 0u32]);
    }
    let player_value = player(e);
    let fov = e.call(0x0071_0ab0, &args![player_value]).f32();
    e.call(0x00b5_4000, &args![fov]);
    e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        let temporary = frame;
        let guard = frame + 0x10;
        e.call(0x004a_d0c0, &args![temporary, 0x101u32]);
        let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        let made = e.vcall(child, 0x48, &args![temporary]).u32();
        if e.call(0x0044_ddc0, &args![0x011f_2250u32]).u32() != 4 {
            let fov = camera_fov(e);
            e.call(SET_CAMERA_FOV, &args![root, fov, 0u32, made, 0u32]);
        }
        e.call(
            PROFILE_SCOPE_CTOR,
            &args![guard, 0x27u32, 1u32, SOURCE_FILE_NAME, 0x234cu32],
        );
        let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.call(0x00b6_5550, &args![accumulator, child, made, last]);
        if made != 0 {
            e.vcall(made, 0x0, &args![1u32]);
        }
        e.call(PROFILE_SCOPE_DTOR, &args![guard]);
        e.call(0x004a_d1d0, &args![temporary]);
    });
}

// Translated from 00875fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00b651e0` on `accumulator` with the root's child and `last`.
/// `this` is not read.
pub fn fn_00875fa0(e: &mut Engine, _this: Ptr, accumulator: u32, last: u32) {
    let root = e.call(GET_ROOT, &args![]).u32();
    let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
    e.call(0x00b6_51e0, &args![accumulator, child, last]);
}

// Translated from 00875fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pass that draws the main scene (`00b55ac0`) under a profiler scope
/// (kind `0x27`, line `0x2362`): prepares the render state (`00b6b790`
/// unless `004e9510`, `00702870` when no menu is showing), draws the
/// target of the global `0x011deb38` (`00801a60`), sets the scene globals
/// for a rendered menu, runs `00b55ac0(scene_a, texture, scene_b)`,
/// releases the target and resets the colour. The first stack word is not
/// read.
pub fn fn_00875fd0(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    texture: u32,
    scene_a: u32,
    scene_b: u32,
) {
    let this_addr = this.addr();
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let guard = frame;
        let color = frame + 0x20;
        e.call(
            PROFILE_SCOPE_CTOR,
            &args![guard, 0x27u32, 1u32, SOURCE_FILE_NAME, 0x2362u32],
        );
        if e.call(0x004e_9510, &args![]).u8() == 0 {
            e.call(0x00b6_b790, &args![]);
        }
        if e.call(IS_IN_PIPBOY_MENU, &args![]).u8() == 0
            && e.call(IS_IN_MENU_MODE, &args![]).u8() == 0
            && e.call(0x0070_5e80, &args![]).u8() == 0
        {
            let camera_owner = e.mem.u32(0x011d_df38);
            if e.call(0x0042_ce10, &args![camera_owner]).u8() == 0 {
                let camera_owner = e.mem.u32(0x011d_df38);
                if e.call(0x0056_21d0, &args![camera_owner]).u8() == 0
                    && e.mem.u8(this_addr + MAIN_RENDERING_MENU) == 0
                {
                    e.call(0x0070_2870, &args![]);
                }
            }
        }
        if pointer_get(e, FIRST_PERSON_TARGET_POINTER) != 0 {
            let held = pointer_get(e, FIRST_PERSON_TARGET_POINTER);
            let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
            e.call(0x0080_1a60, &args![interface, texture, held, 0u32, 0u32]);
        }
        let rendering_menu = e.call(IS_IN_RENDERED_MENU, &args![]).u8() != 0
            && e.mem.u8(this_addr + MAIN_RENDERING_MENU) == 0
            && e.global::<u8>(0x011d_ea29) != 0;
        if rendering_menu {
            e.set_global(0x0120_05a2, 1u8);
            let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
            let value = e.vcall(menu, 0x2c, &args![]).f32();
            e.set_global(0x011a_dd08, value);
        }
        let root = e.call(GET_ROOT, &args![]).u32();
        let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
        e.set_global(0x011f_917c, child);
        e.call(0x00b5_5ac0, &args![scene_a, texture, scene_b]);
        if pointer_get(e, FIRST_PERSON_TARGET_POINTER) != 0 {
            let interface = e.call(GET_INTERFACE_OBJECT, &args![]).u32();
            e.call(0x0080_19d0, &args![interface]);
            let held = pointer_get(e, FIRST_PERSON_TARGET_POINTER);
            let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
            e.call(0x00b6_da10, &args![factory, held]);
            e.call(NI_POINTER_ASSIGN, &args![FIRST_PERSON_TARGET_POINTER, 0u32]);
        }
        if rendering_menu {
            e.set_global(0x0120_05a2, 0u8);
            e.set_global(0x011a_dd08, 1.0f32);
        }
        color_ctor(e, color, 0.0, 1.0, 1.0, 0.0);
        let child = e.call(GET_ROOT_CHILD_OF_ROOT, &args![]).u32();
        e.call(SET_COLOR, &args![child, color]);
        e.call(PROFILE_SCOPE_DTOR, &args![guard]);
    });
}

// Translated from 008761e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pass that draws the pipboy-style rendered menu (`00707ad0`): its
/// node's transform is reset to the identity while the pipboy is active, the
/// scene is accumulated into the menu accumulators (`+0x8c`, `+0x98`) and
/// the transform is put back. `interface_arg` goes to `007148c0`. The
/// first stack word is not read.
pub fn fn_008761e0(e: &mut Engine, this: Ptr, _unused_0: u32, interface_arg: u32) {
    let this_addr = this.addr();
    e.with_stack(0x1a0, |e, frame| {
        let frame = frame.addr();
        let saved_transform = frame;
        let transform = frame + 0x34;
        let point = frame + 0x68;
        let color = frame + 0x78;
        let out_a = frame + 0x88;
        let controls = frame + 0x98;
        let root = e.call(GET_ROOT, &args![]).u32();
        let target = e.call(GET_SCENE_TARGET, &args![root]).u32();
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        let menu_scene = e.call(0x008b_6200, &args![menu]).u32();
        e.call(0x0071_48c0, &args![interface_arg, 0u32, 4u32]);
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        let menu_screen = e.call(0x0055_85e0, &args![menu]).u32();
        e.call(0x004f_b0b0, &args![target, 1u32]);
        e.call(POINT3_CTOR, &args![point, 0u32, 0u32, 0u32]);
        e.call(0x0047_6a80, &args![transform]);
        let flag = fn_00876810(e, Ptr::new(menu_screen));
        let scene_node = if menu_scene != 0 {
            menu_scene
        } else {
            e.call(GET_ROOT_CHILD, &args![root]).u32()
        };
        let player_value = player(e);
        if e.call(0x0096_7ae0, &args![player_value]).u8() != 0 {
            let source = e.call(0x006a_9540, &args![menu_screen]).u32();
            for k in 0..13 {
                let word = e.mem.u32(source + 4 * k);
                e.mem.set_u32(saved_transform + 4 * k, word);
            }
            e.call(0x0044_0460, &args![menu_screen, ZERO_VECTOR]);
            e.call(0x00a5_9c60, &args![menu_screen, point]);
            // The translation of the saved copy (offset 0x24 of the transform).
            e.call(0x0044_0460, &args![menu_screen, saved_transform + 0x24]);
            e.call(0x00a8_6bf0, &args![transform]);
            if scene_node != 0 {
                let owner = e.call(0x0096_11e0, &args![scene_node]).u32();
                if owner != 0 {
                    let owner = e.call(0x0096_11e0, &args![scene_node]).u32();
                    let source = e.call(0x006a_9540, &args![owner]).u32();
                    for k in 0..13 {
                        let word = e.mem.u32(source + 4 * k);
                        e.mem.set_u32(transform + 4 * k, word);
                    }
                    let other = e.mem.u32(0x011e_07d0);
                    let member = e.call(MEMBER_8C, &args![other]).u32();
                    let owner = e.call(0x0096_11e0, &args![scene_node]).u32();
                    e.call(0x0044_0460, &args![owner, member]);
                    let owner = e.call(0x0096_11e0, &args![scene_node]).u32();
                    e.call(0x00a5_9c60, &args![owner, point]);
                }
            }
            // Still inside the Pip-Boy test (00876295 jumps past it to 00876376): the menu
            // screen's position becomes the scene node's offset only while the Pip-Boy is up.
            let member = e.call(MEMBER_8C, &args![menu_screen]).u32();
            let result = e.call(0x004a_0bd0, &args![member, out_a]).u32();
            let words = [
                e.mem.u32(result),
                e.mem.u32(result + 4),
                e.mem.u32(result + 8),
            ];
            let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            fn_008750e0(e, Ptr::new(indexed), words[0], words[1], words[2]);
        }
        if e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0 {
            let positions = controls;
            e.mem.set_u32(positions + 8, 0);
            let object = e.call(0x0087_7720, &args![this_addr]).u32();
            let first = e.call(0x00a2_3a50, &args![object, 0u32, 2u32]).i32() > 0;
            let object = e.call(0x0087_7720, &args![this_addr]).u32();
            let second = e.call(0x00a2_3a50, &args![object, 0u32, 1u32]).i32() > 0;
            let pipboy = e.call(GET_PIPBOY, &args![]).u32();
            let result = e
                .call(
                    0x007f_8720,
                    &args![
                        pipboy,
                        second as u32,
                        first as u32,
                        positions,
                        positions + 4,
                        positions + 8
                    ],
                )
                .u8();
            let x = e.mem.f32(positions);
            let y = e.mem.f32(positions + 4);
            let z = e.mem.u32(positions + 8);
            e.call(0x0070_9b50, &args![result, x, y, z]);
        }
        let scene = if menu_scene != 0 {
            menu_scene
        } else {
            e.call(GET_ROOT_CHILD, &args![root]).u32()
        };
        let mut scene = scene;
        if scene == 0 {
            scene = pointer_get(e, this_addr + 0xa0);
        }
        let color_source = e.call(0x004a_0d10, &args![scene]).u32();
        for k in 0..4 {
            let word = e.mem.u32(color_source + 4 * k);
            e.mem.set_u32(color + 4 * k, word);
        }
        if e.global::<u8>(0x011f_9426) != 0 {
            let first = e.call(GET_GLOBAL_011F4748, &args![]).u32();
            let second = e.call(GET_GLOBAL_011F4748, &args![]).u32();
            let wanted = e.vcall(second, 0xc8, &args![]).u32();
            if e.vcall(first, 0xcc, &args![]).u32() == wanted {
                e.call(SET_COLOR, &args![scene, CONSTANT_COLOR]);
            }
        }
        e.call(0x00b9_8380, &args![0u32, 1u32]);
        e.call(0x0041_fd00, &args![target, scene]);
        let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
        e.call(0x004a_0fd0, &args![target, accumulator]);
        let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
        e.call(0x004a_1020, &args![accumulator, indexed]);
        e.call(ACCUMULATE_SCENE, &args![scene, menu_screen, target]);
        let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
        e.call(0x00b6_c0d0, &args![scene, accumulator, 0u32]);
        let accumulator = pointer_get(e, this_addr + MAIN_FIRST_PERSON_ACCUM);
        e.call(0x00b6_3b90, &args![accumulator, 6u32]);
        e.call(0x004a_0fd0, &args![target, 0u32]);
        e.call(0x004e_ced0, &args![1u32]);
        let zero_arg = e.call(0x0040_df90, &args![]).u32();
        let screen = rendered_menu_screen(e);
        let property = e.call(0x00a5_9d30, &args![screen, zero_arg]).u32();
        if property != 0 {
            e.call(0x0049_ed90, &args![property, 1u32]);
            e.call(0x0043_9340, &args![property, 6u32]);
            e.call(0x0043_9390, &args![property, 0u32]);
            let screen = rendered_menu_screen(e);
            e.call(0x00a5_a040, &args![screen]);
        }
        let render_target = pointer_get(e, OFFSCREEN_TARGET_POINTER);
        let texture = e.call(0x004b_c320, &args![render_target, 0u32]).u32();
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        e.call(SET_SCREEN_TEXTURE, &args![menu, texture]);
        e.call(0x0041_fd00, &args![target, scene]);
        let accumulator = pointer_get(e, this_addr + MAIN_MENU_ACCUM);
        e.call(0x004a_0fd0, &args![target, accumulator]);
        e.call(0x00b9_7e30, &args![0u32, 1u32]);
        let player_value = player(e);
        if e.call(ACTOR_FLAG, &args![player_value]).u8() != 0
            && e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0
        {
            for index in 0..3u32 {
                let pipboy = e.call(GET_PIPBOY, &args![]).u32();
                let element = fn_00875bd0(e, Ptr::new(pipboy), index);
                e.call(ACCUMULATE_SCENE, &args![scene, element, target]);
            }
            e.call(0x00b9_7de0, &args![0u32, 1u32]);
        }
        let screen = rendered_menu_screen(e);
        e.call(ACCUMULATE_SCENE, &args![scene, screen, target]);
        let accumulator = pointer_get(e, this_addr + MAIN_MENU_ACCUM);
        e.call(0x00b6_c0d0, &args![scene, accumulator, 0u32]);
        e.call(0x0071_4a60, &args![1u32]);
        let player_value = player(e);
        if e.call(ACTOR_FLAG, &args![player_value]).u8() != 0
            && e.call(IS_IN_PIPBOY_MENU, &args![]).u8() != 0
        {
            e.call(0x0071_4a40, &args![1u32]);
        }
        e.call(0x004a_0fd0, &args![target, 0u32]);
        let menu = e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32();
        e.call(SET_SCREEN_TEXTURE, &args![menu, 0u32]);
        if property != 0 {
            e.call(0x0049_ed90, &args![property, 0u32]);
        }
        e.call(SET_COLOR, &args![scene, color]);
        let player_value = player(e);
        if e.call(0x0096_7ae0, &args![player_value]).u8() != 0 {
            e.call(0x00a5_9c60, &args![menu_screen, point]);
            e.call(0x0054_6780, &args![menu_screen, flag as u32]);
            if scene_node != 0 {
                let owner = e.call(0x0096_11e0, &args![scene_node]).u32();
                if owner != 0 {
                    let owner = e.call(0x0096_11e0, &args![scene_node]).u32();
                    // The translation of the identity-reset transform.
                    e.call(0x0044_0460, &args![owner, transform + 0x24]);
                    let owner = e.call(0x0096_11e0, &args![scene_node]).u32();
                    e.call(0x00a5_9c60, &args![owner, point]);
                }
            }
            let words = [
                e.global::<u32>(ZERO_VECTOR),
                e.global::<u32>(ZERO_VECTOR + 4),
                e.global::<u32>(ZERO_VECTOR + 8),
            ];
            let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            fn_008750e0(e, Ptr::new(indexed), words[0], words[1], words[2]);
        }
        e.call(0x004f_b0b0, &args![target, 0u32]);
    });
}

// Translated from 00876810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00456630(this, 0x800)`: a flag query on the node with the mask `0x800`.
pub fn fn_00876810(e: &mut Engine, this: Ptr) -> u8 {
    e.call(0x0045_6630, &args![this, 0x800u32]).u8()
}

// Translated from 00876830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00b6b790`; the two stack words are not read.
pub fn fn_00876830(e: &mut Engine, _this: Ptr, _unused_0: u32, _unused_1: u32) {
    e.call(0x00b6_b790, &args![]);
}

// Translated from 00876850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wind-down of the frame: gives `object` (if set) to `00642890`,
/// finishes the root's child (`00ba9420`), returns `render_target` (if
/// set) to the object `004a0ea0` returns (`00b6da10`), resets the
/// accumulators (`+0x88` with `00b65660`, `+0x94` with `00b630c0`) and
/// the shader state (`00b540b0`, `00b4f510`), returns the rendered texture
/// borrowed from `water` (`004ebbc0` with 4, `00876a00`), calls the
/// stdcall slots `0x1a0` and `0x190` of the object `004dc020` returns for
/// the one of `004a0e90` (the object is passed as the first stack word)
/// and, when the `NiPointer` global `0x011fa008` holds a value, builds a
/// `0x30`-byte object with it (`004d61b0`, title string `0x01082d28`,
/// 800 x 600 at `0x80000000, 0x80000000`) and drops the pointer. The
/// third stack word is not read.
pub fn fn_00876850(
    e: &mut Engine,
    this: Ptr,
    render_target: u32,
    water: u32,
    _unused_2: u32,
    object: u32,
) {
    let this_addr = this.addr();
    let root = e.call(GET_ROOT, &args![]).u32();
    e.call(GET_SCENE_TARGET, &args![root]);
    if object != 0 {
        e.call(0x0064_2890, &args![object]);
    }
    let child = e.call(GET_ROOT_CHILD, &args![root]).u32();
    e.call(0x00ba_9420, &args![child]);
    if render_target != 0 {
        let factory = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
        e.call(0x00b6_da10, &args![factory, render_target]);
    }
    let accumulator = pointer_get(e, this_addr + MAIN_WORLD_ACCUM);
    e.call(0x00b6_5660, &args![accumulator]);
    let accumulator = pointer_get(e, this_addr + 0x94);
    e.call(0x00b6_30c0, &args![accumulator]);
    e.call(0x00b5_40b0, &args![]);
    e.call(0x00b4_f510, &args![]);
    let borrowed = e.call(0x004e_bbc0, &args![water, 4u32]).u32();
    fn_00876a00(e, Ptr::new(borrowed));
    if e.global::<u8>(0x011f_9440) != 0 {
        e.call(0x00ba_d4a0, &args![]);
    }
    let decal = e.call(GET_GLOBAL_011F9508, &args![]).u32();
    let device = e.call(0x004d_c020, &args![decal]).u32();
    // `SetIndices(NULL)` and `SetStreamSource(0, NULL, 0, 0)` (008768f6..: the exe pushes the
    // arguments, then the device once as the COM `this`; `vcall` supplies that word).
    e.vcall(device, 0x1a0, &args![0u32]);
    e.vcall(device, 0x190, &args![0u32, 0u32, 0u32, 0u32]);
    if e.call(0x0068_3a60, &args![]).u32() != 0 {
        e.call(0x0071_4900, &args![]);
    }
    if pointer_get(e, 0x011f_a008) != 0 {
        let memory = e.call(ALLOCATE_OBJECT, &args![0x30u32]).u32();
        if memory != 0 {
            let held = pointer_get(e, 0x011f_a008);
            let first_value = e.call(0x0044_ddc0, &args![this_addr]).u32();
            let second_value = e.call(0x0084_e3a0, &args![this_addr]).u32();
            e.call(
                0x004d_61b0,
                &args![
                    memory,
                    second_value,
                    first_value,
                    held,
                    0x0108_2d28u32,
                    0x8000_0000u32,
                    0x8000_0000u32,
                    0x320u32,
                    0x258u32
                ],
            );
        }
        e.call(NI_POINTER_ASSIGN, &args![0x011f_a008u32, 0u32]);
    }
    e.call(0x00b9_88e0, &args![]);
}

// Translated from 00876a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::ReturnRenderedTexture` (`00ba39a0`, Xbox PDB) on the
/// member at `+0x7c`.
pub fn fn_00876a00(e: &mut Engine, this: Ptr) {
    e.call(0x00ba_39a0, &args![this.addr() + 0x7c]);
}

// Translated from 00876a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the byte `0x011a9604` cleared around the calls: `004e9bb0(object,
/// 00874690())` and `00b65ec0` on the `NiPointer` at `+0x94`.
pub fn fn_00876a20(e: &mut Engine, this: Ptr, object: u32) {
    let saved: u8 = e.global(0x011a_9604);
    e.set_global(0x011a_9604, 0u8);
    let held = fn_00874690(e);
    e.call(0x004e_9bb0, &args![object, held]);
    let accumulator = pointer_get(e, this.addr() + 0x94);
    e.call(0x00b6_5ec0, &args![accumulator]);
    e.set_global(0x011a_9604, saved);
}

// Translated from 00876a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl. Reads the "Installed Path" value (name at `0x01082d38`) of the
/// machine key "Software\Bethesda Softworks\FalloutNV" (`0x01082d48`):
/// `RegOpenKeyExA` (import slot `0x00fdf00c`, key `0x80000002`, access 1),
/// `RegQueryValueExA` (`0x00fdf008`, into `buffer` with `size` as the
/// in/out size) and `RegCloseKey` (`0x00fdf004`). The key handle is closed
/// even when the open failed.
pub fn fn_00876a70(e: &mut Engine, buffer: u32, size: u32) {
    e.with_stack(0x10, |e, cells| {
        let cells = cells.addr();
        let handle = cells;
        let size_cell = cells + 4;
        e.mem.set_u32(size_cell, size);
        let status = e
            .call(
                0x00fd_f00c,
                &args![0x8000_0002u32, 0x0108_2d48u32, 0u32, 1u32, handle],
            )
            .u32();
        if status == 0 {
            let key = e.mem.u32(handle);
            e.call(
                0x00fd_f008,
                &args![key, 0x0108_2d38u32, 0u32, 0u32, buffer, size_cell],
            );
        }
        let key = e.mem.u32(handle);
        e.call(0x00fd_f004, &args![key]);
    });
}

// Translated from 00876ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl. Builds the INI file name into `buffer` (`0x104` bytes) and
/// prepares the folders around it. Formats `".\%s"` twice (`00406d00`,
/// a `_snprintf`-style call, with the strings in `0x011a2ff4` and
/// `0x011a2ff0`) and reads `GetPrivateProfileIntA("General",
/// "bUseMyGamesDirectory", 1, buffer)` (import slot `0x00fdf0c4`) after
/// each. When both answer nonzero it makes the per-user folders
/// (`SHGetFolderPathA`, slot `0x00fdf274`, with the folder ids `0x1c` and
/// `5`; `00406d50` appends `"\FalloutNV\"`, `"\My Games\"`,
/// `"FalloutNV\"`; `CreateDirectoryA`, slot `0x00fdf0b8`) and copies the
/// paths with `00406d30` into the buffers `0086d480` and `004dc110`
/// return; otherwise both buffers get `".\"`. The result is the buffer of
/// `004dc110` followed by the string in `0x011a2ff0`. The three file names
/// of `00c3bb80`, `00c3bbd0` and `00c3bc20` are deleted (`DeleteFileA`,
/// slot `0x00fdf0d4`); when the INI file does not exist (`_access`,
/// `00ecbf05`) `"Fallout_default.ini"` is copied to it (`CopyFileA`, slot
/// `0x00fdf1b8`).
///
/// The `/GS` cookie check at the end is not translated.
pub fn fn_00876ad0(e: &mut Engine, buffer: u32) {
    e.with_stack(0x300, |e, frame| {
        let frame = frame.addr();
        let folder = frame;
        let default_file = frame + 0x110;
        let mut ok = true;
        let first: u32 = e.global(0x011a_2ff4);
        e.call(0x0040_6d00, &args![buffer, 0x104u32, 0x0108_2dc4u32, first]);
        if e.call(
            0x00fd_f0c4,
            &args![0x0105_d548u32, 0x0108_2dacu32, 1u32, buffer],
        )
        .u32()
            == 0
        {
            ok = false;
        }
        let second: u32 = e.global(0x011a_2ff0);
        e.call(
            0x0040_6d00,
            &args![buffer, 0x104u32, 0x0108_2dc4u32, second],
        );
        if ok
            && e.call(
                0x00fd_f0c4,
                &args![0x0105_d548u32, 0x0108_2dacu32, 1u32, buffer],
            )
            .u32()
                == 0
        {
            ok = false;
        }
        if ok {
            e.mem.set_u8(buffer, 0);
            e.call(0x00fd_f274, &args![0u32, 0x1cu32, 0u32, 0u32, folder]);
            e.call(0x0040_6d50, &args![folder, 0x104u32, 0x0108_2da0u32]);
            e.call(0x00fd_f0b8, &args![folder, 0u32]);
            let target = e.call(0x0086_d480, &args![]).u32();
            e.call(0x0040_6d30, &args![target, 0x104u32, folder]);
            e.call(0x00fd_f274, &args![0u32, 5u32, 0u32, 0u32, folder]);
            e.call(0x0040_6d50, &args![folder, 0x104u32, 0x0108_2d94u32]);
            e.call(0x00fd_f0b8, &args![folder, 0u32]);
            e.call(0x0040_6d50, &args![folder, 0x104u32, 0x0108_2d88u32]);
            e.call(0x00fd_f0b8, &args![folder, 0u32]);
            let target = e.call(0x004d_c110, &args![]).u32();
            e.call(0x0040_6d30, &args![target, 0x104u32, folder]);
        } else {
            let target = e.call(0x0086_d480, &args![]).u32();
            e.call(0x0040_6d30, &args![target, 0x104u32, 0x0108_2d84u32]);
            let target = e.call(0x004d_c110, &args![]).u32();
            e.call(0x0040_6d30, &args![target, 0x104u32, 0x0108_2d84u32]);
        }
        let source = e.call(0x004d_c110, &args![]).u32();
        e.call(0x0040_6d30, &args![buffer, 0x104u32, source]);
        let name: u32 = e.global(0x011a_2ff0);
        e.call(0x0040_6d50, &args![buffer, 0x104u32, name]);
        for handle in [0x00c3_bb80u32, 0x00c3_bbd0, 0x00c3_bc20] {
            let value = e.call(handle, &args![]).u32();
            e.call(0x00fd_f0d4, &args![value]);
        }
        if e.call(0x00ec_bf05, &args![buffer, 0u32]).i32() == -1 {
            e.call(0x0040_6d30, &args![default_file, 0x104u32, 0x0108_2d70u32]);
            e.call(0x00fd_f1b8, &args![default_file, buffer, 1u32]);
        }
    });
}

// Translated from 00876d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main_InitArchive` (Xbox PDB): initialises the archive manager from
/// the game settings: `00af43a0` (a byte setting, two strings, `0x200000`),
/// `00af4460` (a byte setting), `00af4470` (three integer settings),
/// `00af4490` (a byte setting and a string), then
/// `ArchiveManager::OpenMasterArchives` (`00af4550`, Xbox PDB).
pub fn main_init_archive(e: &mut Engine) {
    let first = e.call(GET_POOLED_TEXT, &args![0x011d_eb94u32]).u32();
    let second = e.call(GET_POOLED_TEXT, &args![0x011c_4030u32]).u32();
    let flag = setting_byte(e, 0x011d_ee3c);
    e.call(0x00af_43a0, &args![flag, second, first, 0x20_0000u32]);
    let flag = setting_byte(e, 0x011d_ebdc);
    e.call(0x00af_4460, &args![flag]);
    let a = setting_dword(e, 0x011d_ec40) as u32;
    let b = setting_dword(e, 0x011d_eaa0) as u32;
    let c = setting_dword(e, 0x011d_eb24) as u32;
    e.call(0x00af_4470, &args![c, b, a]);
    let text = e.call(GET_POOLED_TEXT, &args![0x011d_ec68u32]).u32();
    let flag = setting_byte(e, 0x011d_ec4c);
    e.call(0x00af_4490, &args![flag, text]);
    e.call(0x00af_4550, &args![]);
}

// Translated from 00876dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main_InitStartCell` (Xbox PDB): finds the cell the game starts in and
/// puts the player there. Cdecl: `cell_name` (an editor id string or 0) and
/// `position` (three floats it fills in). The start cell comes from, in
/// this order: `cell_name` when `0044a670` accepts it, the setting string
/// `0x011ded5c`, or the world-space name and coordinates of the settings
/// `0x011dee8c` / `0x011debc4` (a world space is searched in the list
/// `00460140` returns for the data handler `0x011c3f2c`, by the name in
/// the setting `0x011deb48`; `00461c20` creates the cell). With a cell
/// found (a loaded cell goes through `00453dc0`, an unloaded one sets the
/// world space, the position `x/y << 12 + 0x01016968` and `00454450`).
/// When nothing was loaded the data handler is told (`004650a0(-1)`) and
/// the world is refreshed (`004515a0`, `004ba7d0`); a failure logs a
/// "CELLS: Could not find starting cell ..." message through `005b5e40`
/// (naming the cell setting, or the coordinates and world space).
pub fn main_init_start_cell(e: &mut Engine, cell_name: u32, position: u32) {
    let mut cell = 0u32;
    let mut message_wanted = true;
    let mut run = true;
    let named = cell_name != 0 && e.call(0x0044_a670, &args![cell_name]).u32() != 0;
    let by_setting = named || {
        let text = pooled_text(e, 0x011d_ed5c);
        e.call(0x00ec_6130, &args![text]).u32() != 0
    };
    if by_setting {
        let named_again = cell_name != 0 && e.call(0x0044_a670, &args![cell_name]).u32() != 0;
        if named_again {
            let handler = data_handler(e);
            cell = e.call(0x0046_1ae0, &args![handler, cell_name]).u32();
        } else {
            let text = pooled_text(e, 0x011d_ed5c);
            let parsed = e.call(0x0046_4f30, &args![text, 0u32]).u32();
            let handler = data_handler(e);
            cell = e.call(0x0046_1ae0, &args![handler, parsed]).u32();
        }
        if cell == 0 {
            e.with_stack(0x10, |e, outputs| {
                let out_x = outputs.addr();
                let out_y = outputs.addr() + 4;
                e.mem.set_u32(out_x, 0);
                e.mem.set_u32(out_y, 0);
                let text = pooled_text(e, 0x011d_ed5c);
                let parsed = e.call(0x0046_4f30, &args![text, 0u32]).u32();
                let handler = data_handler(e);
                let space = e
                    .call(0x0046_1cf0, &args![handler, parsed, out_x, out_y])
                    .u32();
                if space != 0 {
                    let world = e.mem.u32(WORLD_OBJECT);
                    e.call(0x0045_8200, &args![world, space]);
                    let x = e.mem.i32(out_x);
                    let y = e.mem.i32(out_y);
                    place_at_cell_coordinates(e, position, x, y);
                    let world = e.mem.u32(WORLD_OBJECT);
                    e.call(0x0045_4450, &args![world, position]);
                    let handler = data_handler(e);
                    let x = e.mem.u32(out_x);
                    let y = e.mem.u32(out_y);
                    cell = e
                        .call(0x0046_1c20, &args![handler, x, y, space, 0u32])
                        .u32();
                    run = false;
                }
            });
        }
    } else {
        let space_text = pooled_text(e, 0x011d_ee8c);
        if e.call(0x00ec_6130, &args![space_text]).u32() != 0 && {
            let coords_text = pooled_text(e, 0x011d_ebc4);
            e.call(0x00ec_6130, &args![coords_text]).u32() != 0
        } {
            let text = pooled_text(e, 0x011d_ee8c);
            let parsed = e.call(0x0046_4f30, &args![text, 0u32]).u32();
            let x = e.call(0x00ec_a6d3, &args![parsed]).u32();
            let text = pooled_text(e, 0x011d_ebc4);
            let parsed = e.call(0x0046_4f30, &args![text, 0u32]).u32();
            let y = e.call(0x00ec_a6d3, &args![parsed]).u32();
            let handler = data_handler(e);
            let list = e.call(0x0046_0140, &args![handler]).u32();
            let link = e.call(0x0068_15c0, &args![list]).u32();
            let mut world_space = e.mem.u32(link);
            let handler = data_handler(e);
            let mut current = e.call(0x0046_0140, &args![handler]).u32();
            while current != 0 {
                if e.call(0x0082_56d0, &args![current]).u8() != 0 {
                    break;
                }
                let link = e.call(0x0068_15c0, &args![current]).u32();
                let candidate = e.mem.u32(link);
                let wanted = pooled_text(e, 0x011d_eb48);
                let name = e.vcall(candidate, 0x130, &args![]).u32();
                if e.call(0x0040_4dc0, &args![name, wanted]).u32() == 0 {
                    world_space = candidate;
                    break;
                }
                current = e.call(0x0072_6070, &args![current]).u32();
            }
            if world_space != 0 {
                cell = e.call(0x0058_5b30, &args![world_space, x, y]).u32();
                if cell == 0 {
                    let handler = data_handler(e);
                    cell = e
                        .call(0x0046_1c20, &args![handler, x, y, world_space, 1u32])
                        .u32();
                }
            }
        } else {
            message_wanted = false;
            run = false;
        }
    }
    if !run {
        return;
    }
    if cell != 0 {
        if e.call(0x0042_5fd0, &args![cell]).u8() != 0 {
            let world = e.mem.u32(WORLD_OBJECT);
            e.call(0x0045_3dc0, &args![world, cell, position]);
        } else {
            let space = e.call(0x0054_ddd0, &args![cell]).u32();
            let world = e.mem.u32(WORLD_OBJECT);
            e.call(0x0045_8200, &args![world, space]);
            let x = e.call(0x0054_4c30, &args![cell]).i32();
            let y = e.call(0x0054_4c60, &args![cell]).i32();
            place_at_cell_coordinates(e, position, x, y);
            let world = e.mem.u32(WORLD_OBJECT);
            e.call(0x0045_4450, &args![world, position]);
        }
        run = false;
    } else if message_wanted {
        report_missing_start_cell(e);
    }
    let handler = data_handler(e);
    e.call(0x0046_50a0, &args![handler, u32::MAX]);
    if run {
        let world = e.mem.u32(WORLD_OBJECT);
        e.call(0x0045_15a0, &args![world, position, 1u32]);
        let world = e.mem.u32(WORLD_OBJECT);
        let grid = e.call(0x0044_ddc0, &args![world]).u32();
        e.call(0x004b_a7d0, &args![grid]);
    }
}

/// The data handler global `0x011c3f2c`.
fn data_handler(e: &Engine) -> u32 {
    e.global(0x011c_3f2c)
}

/// The string `00403df0` returns for the setting at `setting`.
fn pooled_text(e: &mut Engine, setting: u32) -> u32 {
    e.call(GET_POOLED_TEXT, &args![setting]).u32()
}

/// Whether the string at `text` is not empty (its first byte, signed).
fn text_not_empty(e: &Engine, text: u32) -> bool {
    e.mem.i8(text) != 0
}

/// Stores the position `(x << 12) + d, (y << 12) + d, 0` (with `d` the
/// double at `0x01016968`) as floats at `position`.
fn place_at_cell_coordinates(e: &mut Engine, position: u32, x: i32, y: i32) {
    let bias: f64 = e.global(0x0101_6968);
    e.mem.set_f32(position, ((x << 12) as f64 + bias) as f32);
    e.mem
        .set_f32(position + 4, ((y << 12) as f64 + bias) as f32);
    e.mem.set_f32(position + 8, 0.0);
}

/// The log message of `main_init_start_cell` when no start cell was found.
fn report_missing_start_cell(e: &mut Engine) {
    let cell_text = pooled_text(e, 0x011d_ed5c);
    if cell_text != 0 && {
        let text = pooled_text(e, 0x011d_ed5c);
        text_not_empty(e, text)
    } {
        let text = pooled_text(e, 0x011d_ed5c);
        e.call(0x005b_5e40, &args![0x0108_2e94u32, text]);
        return;
    }
    let space_text = pooled_text(e, 0x011d_ee8c);
    let usable = space_text != 0
        && pooled_text(e, 0x011d_ebc4) != 0
        && {
            let text = pooled_text(e, 0x011d_ee8c);
            text_not_empty(e, text)
        }
        && {
            let text = pooled_text(e, 0x011d_ebc4);
            text_not_empty(e, text)
        };
    if !usable {
        e.call(0x005b_5e40, &args![0x0108_2dccu32]);
        return;
    }
    let name_text = pooled_text(e, 0x011d_eb48);
    if name_text != 0 && {
        let text = pooled_text(e, 0x011d_eb48);
        text_not_empty(e, text)
    } {
        let name = pooled_text(e, 0x011d_eb48);
        let coords = pooled_text(e, 0x011d_ebc4);
        let space = pooled_text(e, 0x011d_ee8c);
        e.call(0x005b_5e40, &args![0x0108_2e50u32, space, coords, name]);
    } else {
        let coords = pooled_text(e, 0x011d_ebc4);
        let space = pooled_text(e, 0x011d_ee8c);
        e.call(0x005b_5e40, &args![0x0108_2e00u32, space, coords]);
    }
}

// Translated from 00877260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, taking a position by value (three words). Moves the player to
/// the position: finds the cell (`005f36f0`, else the grid cell of the
/// setting `0x011c63cc` halved, `00457050`), lets it place the position
/// (`0054cfd0` with the rotation `0x011f426c`), moves the player (vtable
/// slots `0x2c4` with the z word and `0x2a8` with the position), adds the
/// player to the cell (`00548230`, `00456520`), runs a profiler scope
/// (kind `0x34`, line `0x2612`) that updates the player's node properties
/// (`00a5a040`) and `00b5cbd0`, runs the player's script (`00565730`) and
/// resets the root child's position and orientation.
pub fn fn_00877260(e: &mut Engine, x: u32, y: u32, z: u32) {
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let position = frame;
        let rotation = frame + 0x10;
        let orientation = frame + 0x20;
        let point = frame + 0x30;
        let guard = frame + 0x40;
        e.mem.set_u32(position, x);
        e.mem.set_u32(position + 4, y);
        e.mem.set_u32(position + 8, z);
        for k in 0..3 {
            let word = e.global::<u32>(ZERO_VECTOR + 4 * k);
            e.mem.set_u32(rotation + 4 * k, word);
        }
        let world = e.mem.u32(WORLD_OBJECT);
        let mut cell = e.call(0x005f_36f0, &args![world]).u32();
        if cell == 0 {
            let first = setting_dword(e, 0x011c_63cc) as u32 >> 1;
            let second = setting_dword(e, 0x011c_63cc) as u32 >> 1;
            let world = e.mem.u32(WORLD_OBJECT);
            let slot = e.call(0x0045_7050, &args![world, second, first]).u32();
            cell = e.mem.u32(slot);
        }
        if cell != 0 {
            e.call(0x0054_cfd0, &args![cell, position, rotation]);
        }
        let player_value = player(e);
        let height = e.mem.f32(rotation + 8);
        e.vcall(player_value, 0x2c4, &args![height]);
        let player_value = player(e);
        e.vcall(player_value, 0x2a8, &args![position]);
        if cell != 0 {
            let player_value = player(e);
            e.call(0x0054_8230, &args![cell, player_value, 0u32]);
            let loader: u32 = e.global(0x0120_2d98);
            e.call(0x0045_6520, &args![loader]);
            e.call(
                PROFILE_SCOPE_CTOR,
                &args![guard, 0x34u32, 1u32, SOURCE_FILE_NAME, 0x2612u32],
            );
            let node = player_node(e, 0);
            e.call(0x00a5_a040, &args![node]);
            let node = player_node(e, 0);
            let indexed = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
            e.call(0x00b5_cbd0, &args![indexed, node]);
            e.call(PROFILE_SCOPE_DTOR, &args![guard]);
        }
        let player_value = player(e);
        e.call(0x0056_5730, &args![player_value]);
        let root = e.call(GET_ROOT, &args![]).u32();
        let child = e.call(0x0055_8310, &args![root]).u32();
        e.call(0x0044_0460, &args![child, position]);
        let player_value = player(e);
        let current = e.call(0x0056_fa00, &args![player_value, orientation]).u32();
        let root = e.call(GET_ROOT, &args![]).u32();
        let child = e.call(0x0055_8310, &args![root]).u32();
        e.call(0x0043_fa80, &args![child, current]);
        e.call(POINT3_CTOR, &args![point, 0u32, 0u32, 0u32]);
        let root = e.call(GET_ROOT, &args![]).u32();
        let child = e.call(0x0055_8310, &args![root]).u32();
        e.call(0x00a5_9c60, &args![child, point]);
    });
}

// Translated from 00877430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::KillMenuBGTexture` (Xbox PDB). If the `NiPointer` global
/// `0x011ded3c` (the menu-background render target) holds a texture,
/// returns it to the texture manager (`BSTextureManager::ReturnRenderedTexture`,
/// `00b6da10`, Xbox PDB, on the object of `004a0ea0`), clears the pointer
/// (`0066b0d0`) and calls `007123f0` (a float, 0) and `007123a0` (a word, 0)
/// on the object `004e3270` returns. Always clears the byte at `0x011dea29`.
pub fn main_kill_menu_bg_texture(e: &mut Engine, _this: Ptr) {
    let texture = pointer_get(e, MENU_TARGET_POINTER);
    if texture != 0 {
        let manager = e.call(GET_GLOBAL_011F91A8, &args![]).u32();
        e.call(0x00b6_da10, &args![manager, texture]);
        e.call(NI_POINTER_ASSIGN, &args![MENU_TARGET_POINTER, 0u32]);
        let object = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
        e.call(0x0071_23f0, &args![object, 0.0f32]);
        let object = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
        e.call(0x0071_23a0, &args![object, 0u32]);
    }
    e.mem.set_u8(0x011d_ea29, 0);
}

// Translated from 008774a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tears the running game down to the main menu. `this` is an object whose
/// byte at `+4` is read (`004f1540`) and written back at the end
/// (`004f15a0`); `call_interface_step` (one stack word, read as a byte)
/// decides whether `007053f0` runs near the end. In order: saves that byte
/// and clears it; fades out the player's sound (`00877700`); calls
/// `007d6bd0` with 0 on the object of the global `0x011ddf38` (remembering
/// its byte result); ends all sounds of type -1 (`00ad8780` on the object
/// `00453a70` returns); disables the Pipboy radio (`008324e0`); removes the
/// player from his cell (`008d6f30` gives the cell, `0054ca90`); lets the
/// player's two nodes (`00950bb0` with 1 and 0) tell their owner (vtable
/// slot `0xc`, then slot `0xe8` of the owner's `009611e0` object); resets
/// the world object (`004539a0`, `007037c0` and `0061cc40` with
/// `0x7fffffff`), the obstacle manager (`006c0720`, `006c09f0`), the
/// garbage collector (`00868d70`), `00c459d0`, the player (vtable slot
/// `0x1cc`), the `0x011ddf38` object (`0084a840`), the process lists
/// (`00970d50`), the data handler (`004614e0`), the shader manager
/// (`00b4f5c0`, `00b631d0`), `Error`, the support menus (`00706320`) and
/// the world (`0045ac80`); then calls `007d6bd0` again with the remembered
/// result, resets the `Main` object (`008776e0`) and writes the saved
/// byte back.
pub fn fn_008774a0(e: &mut Engine, this: Ptr, call_interface_step: u8) {
    let saved_byte = e.call(0x004f_1540, &args![this]).u32() & 0xff;
    e.call(0x004f_15a0, &args![this, 0u32]);
    let player = e.mem.u32(PLAYER_OBJECT);
    fn_00877700(e, Ptr::new(player));
    let object: u32 = e.global(0x011d_df38);
    let remembered = e.call(0x007d_6bd0, &args![object, 0u32]).u32() & 0xff;
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.call(0x00ad_8780, &args![audio, 0xffff_ffffu32]);
    e.call(0x0083_24e0, &args![0u32]);
    let player = e.mem.u32(PLAYER_OBJECT);
    if e.call(0x008d_6f30, &args![player]).u32() != 0 {
        let player = e.mem.u32(PLAYER_OBJECT);
        let cell = e.call(0x008d_6f30, &args![player]).u32();
        e.call(0x0054_ca90, &args![cell, player]);
    }
    for flag in [1u32, 0] {
        let player = e.mem.u32(PLAYER_OBJECT);
        let node = e.call(GET_PLAYER_NODE, &args![player, flag]).u32();
        let owner = if node != 0 {
            e.vcall(node, 0xc, &args![]).u32()
        } else {
            0
        };
        if owner != 0 && e.call(0x0096_11e0, &args![owner]).u32() != 0 {
            let target = e.call(0x0096_11e0, &args![owner]).u32();
            e.vcall(target, 0xe8, &args![node]);
        }
    }
    let world = e.mem.u32(WORLD_OBJECT);
    e.call(0x0045_39a0, &args![world, 0u32, 0u32]);
    e.call(0x0070_37c0, &args![world, 0x7fff_ffffu32]);
    e.call(0x0061_cc40, &args![world, 0x7fff_ffffu32]);
    let manager = e.call(0x006c_0720, &args![]).u32();
    e.call(0x006c_09f0, &args![manager]);
    e.call(0x0086_8d70, &args![0u32]);
    e.call(0x00c4_59d0, &args![0u32]);
    let player = e.mem.u32(PLAYER_OBJECT);
    e.vcall(player, 0x1cc, &args![0u32, 0u32]);
    let object: u32 = e.global(0x011d_df38);
    e.call(0x0084_a840, &args![object]);
    e.call(0x0097_0d50, &args![0x011e_0e80u32]);
    let data_handler: u32 = e.global(0x011c_3f2c);
    e.call(0x0046_14e0, &args![data_handler]);
    let shader_manager = e.call(0x00b4_f5c0, &args![]).u32();
    e.call(0x00b6_31d0, &args![shader_manager]);
    e.call(ERROR_LOG, &args![]);
    e.call(0x0070_6320, &args![0u32]);
    let world = e.mem.u32(WORLD_OBJECT);
    e.call(0x0045_ac80, &args![world]);
    if call_interface_step != 0 {
        e.call(0x0070_53f0, &args![]);
    }
    let object: u32 = e.global(0x011d_df38);
    e.call(0x007d_6bd0, &args![object, remembered]);
    let main_object = e.mem.u32(MAIN_OBJECT);
    fn_008776e0(e, Ptr::new(main_object));
    let main_object = e.mem.u32(MAIN_OBJECT);
    e.call(0x004f_15a0, &args![main_object, saved_byte]);
}

// Translated from 008776e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bytes at `+2` and `+5` of the `Main` object.
pub fn fn_008776e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 2, 0);
    e.mem.set_u8(this.addr() + 5, 0);
}

// Translated from 00877700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fades out and releases the sound handle at `this + 0x77c` over 1000
/// milliseconds (`BSSoundHandle::FadeOutAndRelease`, `00ad8da0`, Xbox PDB).
pub fn fn_00877700(e: &mut Engine, this: Ptr) {
    e.call(0x00ad_8da0, &args![this.addr() + 0x77c, 0x3e8u32]);
}

// Translated from 00877720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this` is ignored; returns `Controls::GetInstance` (`007fdf30`, Xbox PDB).
pub fn fn_00877720(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call(CONTROLS_GET_INSTANCE, &args![]).ptr()
}

// Translated from 00877730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Main::InitSky` (Xbox PDB). Inside a profiler scope (kind `0x21`, line
/// `0x26bd`): logs the message at `0x01082ec0` (`Error`), initialises the
/// sky object of the global `0x011dea20` (`Sky::Initialize`, `0063a630`,
/// Xbox PDB) with the `NiPointer`s `0x011deb34` and `0x011deb00`, sets the
/// sun light (vtable slot `0xdc` of the `0x011deda4` object, then
/// `ShadowSceneNode::SetSunLight`, `00b5aac0`), updates the properties of
/// the `0x011deb34` node and hands `00b8b200` the byte setting
/// `0x011deadc`; when that setting is non-zero builds a 0x24-byte property
/// object (`0049ec80`, then `0049ed90` with 1, `0049ede0` with 2,
/// `0050f9a0` with `0xff`) and attaches it to the `0x011deb34` node
/// (`NiAVObject::AttachProperty`, `00439410`). Finally gives both nodes a
/// zero position (`00a59c60`), updates the sun node's properties and
/// prepares both objects (`BSShaderManager::PrepareObject`, `00b57e30`).
/// (The exception frame is left out.)
/// No arguments (`main` calls it after `0046dd00` with nothing pushed; `ECX` is not read).
pub fn main_init_sky(e: &mut Engine) {
    with_profile_scope(e, 0x21, 0x26bd, |e| {
        e.call(ERROR_LOG, &args![0x0108_2ec0u32]);
        let node_a = pointer_get(e, 0x011d_eb00);
        let node_b = pointer_get(e, 0x011d_eb34);
        let sky: u32 = e.global(0x011d_ea20);
        e.call(0x0063_a630, &args![sky, node_b, node_a]);
        let sun = pointer_get(e, 0x011d_eda4);
        let sky: u32 = e.global(0x011d_ea20);
        let sky_part = e.call(0x0045_cd60, &args![sky]).u32();
        let light = e.call(0x0043_b230, &args![sky_part]).u32();
        e.vcall(sun, 0xdc, &args![light, 1u32]);
        let sky: u32 = e.global(0x011d_ea20);
        let sun_light = e.call(0x0045_05a0, &args![sky]).u32();
        let scene = e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        e.call(0x00b5_aac0, &args![scene, sun_light]);
        let node_b = pointer_get(e, 0x011d_eb34);
        e.call(0x00a5_a040, &args![node_b]);
        let setting = setting_byte(e, 0x011d_eadc);
        let target = e.call(GET_GLOBAL_011F91AC, &args![]).u32();
        e.call(0x00b8_b200, &args![target, setting as u32]);
        if setting_byte(e, 0x011d_eadc) != 0 {
            let block = e.call(0x00aa_13e0, &args![0x24u32]).u32();
            let property = if block != 0 {
                e.call(0x0049_ec80, &args![block]).u32()
            } else {
                0
            };
            e.call(0x0049_ed90, &args![property, 1u32]);
            e.call(0x0049_ede0, &args![property, 2u32]);
            e.call(0x0050_f9a0, &args![property, 0xffu32]);
            let node_b = pointer_get(e, 0x011d_eb34);
            e.call(0x0043_9410, &args![node_b, property]);
        }
        e.with_stack(0x20, |e, frame| {
            let first = frame.addr();
            let second = frame.addr() + 0x10;
            e.call(POINT3_CTOR, &args![first, 0.0f32, 0u32, 0u32]);
            let node_b = pointer_get(e, 0x011d_eb34);
            e.call(0x00a5_9c60, &args![node_b, first]);
            let sun = pointer_get(e, 0x011d_eda4);
            e.call(0x00a5_a040, &args![sun]);
            e.call(POINT3_CTOR, &args![second, 0.0f32, 0u32, 0u32]);
            let sun = pointer_get(e, 0x011d_eda4);
            e.call(0x00a5_9c60, &args![sun, second]);
        });
        let node_b = pointer_get(e, 0x011d_eb34);
        e.call(0x00b5_7e30, &args![node_b, 0u32, 0u32]);
        let sky: u32 = e.global(0x011d_ea20);
        let sky_part = e.call(0x0045_cd60, &args![sky]).u32();
        let light = e.call(0x0043_b230, &args![sky_part]).u32();
        e.call(0x00b5_7e30, &args![light, 0u32, 0u32]);
    });
}

// Translated from 00877950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the registry-settings collection if needed (`00877960`) and
/// returns the global `0x01204368` that holds it.
pub fn fn_00877950(e: &mut Engine) -> u32 {
    fn_00877960(e);
    e.global(REG_SETTING_COLLECTION)
}

// Translated from 00877960 (decompiled, FalloutNV.exe 1.4.0.525)
/// If the global `0x01204368` is empty, allocates a 0x114-byte
/// `RegSettingCollection`, constructs it (`008779f0`) and stores it there
/// (the exception-handling frame of the compiled function is left out).
pub fn fn_00877960(e: &mut Engine) {
    if e.global::<u32>(REG_SETTING_COLLECTION) == 0 {
        let block = e.call(ALLOCATE_OBJECT, &args![0x114u32]).u32();
        let collection = if block != 0 {
            fn_008779f0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set_global(REG_SETTING_COLLECTION, collection);
    }
}

// Translated from 008779f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RegSettingCollection` constructor: the base constructor (`0044f700`),
/// then the vtable `0x01082ed8`. Returns `this`.
pub fn fn_008779f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0044_f700, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_2ed8);
    this
}

// Translated from 00877a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0044f650` on `this` (the base-class destructor body of the
/// `RegSettingCollection`) and returns its result.
pub fn fn_00877a10(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0044_f650, &args![this]).u32()
}

// Translated from 00877a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<T *>` element address: `m_pBase + index * 4`.
pub fn fn_00877a30(e: &mut Engine, this: Ptr<NiTArray>, index: u32) -> u32 {
    e.get(this, NiTArray::m_pBase)
        .wrapping_add(index.wrapping_mul(4))
}

// Translated from 00877a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<T *>::Add`: stores the value (a pointer to the word) at the
/// array's end (`m_usSize`) with `00877e10` and returns the index used.
pub fn fn_00877a50(e: &mut Engine, this: Ptr<NiTArray>, value_slot: u32) -> u32 {
    let end = e.get(this, NiTArray::m_usSize) as u32;
    fn_00877e10(e, this, end, value_slot)
}

// Translated from 00877a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonScrapHeapMessageQueue<BSPackedTask>` constructor: the base
/// constructor (`00877d80`), the vtable `0x01082f04`, the scrap heap in
/// `+8`, an empty head at `+0xc` and the tail `+0x10` pointing at the head
/// slot. Returns `this`.
pub fn fn_00877a80(e: &mut Engine, this: Ptr, scrap_heap: u32) -> Ptr {
    fn_00877d80(e, this);
    let at = this.addr();
    e.mem.set_u32(at, 0x0108_2f04);
    e.mem.set_u32(at + 8, scrap_heap);
    e.mem.set_u32(at + 0xc, 0);
    e.mem.set_u32(at + 0x10, at + 0xc);
    this
}

// Translated from 00877ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSTCommonScrapHeapMessageQueue<BSPackedTask>`: sets
/// its vtable (`0x01082f04`), pops every queued task (`006ec390`, the
/// identical `TryPop` body, into a 0x20-byte local) until it fails, then
/// runs the base destructor body (`00877b40`). (The stack-cookie check and
/// the exception frame are left out.)
pub fn fn_00877ac0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_2f04);
    e.with_stack(0x20, |e, task| {
        while e.call(0x006e_c390, &args![this, task]).u32() & 0xff != 0 {}
    });
    fn_00877b40(e, this);
}

// Translated from 00877b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSTCommonMessageQueue<BSPackedTask>`: sets the
/// vtable `0x01082f24`, then runs `00877b60`.
pub fn fn_00877b40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_2f24);
    fn_00877b60(e, this);
}

// Translated from 00877b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSTMessageQueue<BSPackedTask>`: sets the vtable
/// `0x01082f44`.
pub fn fn_00877b60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_2f44);
}

// Translated from 00877b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonMessageQueue<BSPackedTask>` scalar deleting destructor: the
/// body (`00877b40`), then frees `this` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00877b80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00877b40(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00877bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTMessageQueue<BSPackedTask>::_scalar_deleting_destructor_` (Xbox
/// PDB): the body (`00877b60`), then frees `this` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_00877bb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00877b60(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00877be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonScrapHeapMessageQueue<BSPackedTask>::DoTryPush` (Xbox PDB):
/// takes a 0x24-byte node from the scrap heap at `+8` (`ScrapHeap::Allocate`,
/// `00aa54a0`, size `0x24`, the alignment word of `0x010a2720`), constructs
/// it in place (`006e6da0`, `00877db0`), copies the 32-byte task into the
/// node at `+4` and appends the node to the list (head `+0xc`, tail
/// `+0x10`). Always returns true. (The exception frame is left out.)
pub fn fn_00877be0(e: &mut Engine, this: Ptr, task: u32) -> bool {
    let at = this.addr();
    let heap = e.mem.u32(at + 8);
    let alignment: u32 = e.global(0x010a_2720);
    let node = e.call(0x00aa_54a0, &args![heap, 0x24u32, alignment]).u32();
    let placed = e.call(0x006e_6da0, &args![0x24u32, node]).u32();
    if placed != 0 {
        fn_00877db0(e, Ptr::new(placed));
    }
    for word in 0..8 {
        let value = e.mem.u32(task + word * 4);
        e.mem.set_u32(node + 4 + word * 4, value);
    }
    if e.mem.u32(at + 0xc) == 0 {
        e.mem.set_u32(at + 0x10, at + 0xc);
    }
    let tail = e.mem.u32(at + 0x10);
    e.mem.set_u32(tail, node);
    e.mem.set_u32(node, 0);
    e.mem.set_u32(at + 0x10, node);
    true
}

// Translated from 00877cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonScrapHeapMessageQueue<BSPackedTask>::DoTryPop` (Xbox PDB):
/// with an empty list returns false; otherwise copies the head node's
/// 32-byte task to `out`, unlinks the head, destroys it (`007b3fa0` with 0),
/// gives it back to the scrap heap (`ScrapHeap::Deallocate`, `00aa5610`) and
/// returns true.
pub fn fn_00877cc0(e: &mut Engine, this: Ptr, out: u32) -> bool {
    let at = this.addr();
    let head = e.mem.u32(at + 0xc);
    if head == 0 {
        return false;
    }
    for word in 0..8 {
        let value = e.mem.u32(head + 4 + word * 4);
        e.mem.set_u32(out + word * 4, value);
    }
    let next = e.mem.u32(head);
    e.mem.set_u32(at + 0xc, next);
    e.call(0x007b_3fa0, &args![head, 0u32]);
    let heap = e.mem.u32(at + 8);
    e.call(0x00aa_5610, &args![heap, head]);
    true
}

// Translated from 00877d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x40` to 1.
pub fn fn_00877d30(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x40, 1);
}

// Translated from 00877d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonScrapHeapMessageQueue<BSPackedTask>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor body (`00877ac0`), then frees `this` when bit
/// 0 of `flags` is set. Returns `this`.
pub fn fn_00877d50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00877ac0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00877d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonMessageQueue<BSPackedTask>` constructor: the base constructor
/// (`00877df0`), the vtable `0x01082f24` and a zero word at `+4`. Returns
/// `this`.
pub fn fn_00877d80(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00877df0(e, this);
    e.mem.set_u32(this.addr(), 0x0108_2f24);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 00877db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a queue node: the next pointer (`+0`) and the 32-byte
/// task (`+4`) are zeroed. Returns `this`.
pub fn fn_00877db0(e: &mut Engine, this: Ptr) -> Ptr {
    for word in 0..9 {
        e.mem.set_u32(this.addr() + word * 4, 0);
    }
    this
}

// Translated from 00877df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTMessageQueue<BSPackedTask>` constructor: the vtable `0x01082f44`.
/// Returns `this`.
pub fn fn_00877df0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0108_2f44);
    this
}

// Translated from 00877e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<T *>` store with growth: when `index` is at or past the
/// capacity (`m_usMaxSize`), resizes to `m_usGrowBy + index` (`0096ad30`);
/// then stores the word at `value_slot` at `index` (`00877e50`). Returns
/// `index`.
pub fn fn_00877e10(e: &mut Engine, this: Ptr<NiTArray>, index: u32, value_slot: u32) -> u32 {
    let capacity = e.get(this, NiTArray::m_usMaxSize) as u32;
    if index >= capacity {
        let grow_by = e.get(this, NiTArray::m_usGrowBy) as u32;
        e.call(0x0096_ad30, &args![this, grow_by.wrapping_add(index)]);
    }
    fn_00877e50(e, this, index, value_slot);
    index
}

// Translated from 00877e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<T *>::SetAt` body: stores the word at `value_slot` at `index`,
/// keeping `m_usSize` (one past the highest index) and `m_usESize` (the
/// number of elements different from the null word at `0x01011d78`) right.
pub fn fn_00877e50(e: &mut Engine, this: Ptr<NiTArray>, index: u32, value_slot: u32) {
    let null: u32 = e.global(0x0101_1d78);
    let value = e.mem.u32(value_slot);
    let size = e.get(this, NiTArray::m_usSize) as u32;
    let base = e.get(this, NiTArray::m_pBase);
    let element = base.wrapping_add(index.wrapping_mul(4));
    let count = e.get(this, NiTArray::m_usESize);
    if index >= size {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if value != null {
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else if value != null {
        if e.mem.u32(element) == null {
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else if e.mem.u32(element) != null {
        e.set(this, NiTArray::m_usESize, count.wrapping_sub(1));
    }
    e.mem.set_u32(element, value);
}

// Translated from 00877f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<RegSettingCollection>` constructor: the base constructor
/// (`00404920`, with the two stack words), the vtable `0x010839ac`, then
/// registers `this` with the collection (vtable slot `4` of what `00877950`
/// returns). Returns `this`. (The exception frame is left out.)
pub fn fn_00877f10(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(0x0040_4920, &args![this, first, second]);
    e.mem.set_u32(this.addr(), 0x0108_39ac);
    let collection = fn_00877950(e);
    e.vcall(collection, 4, &args![this]);
    this
}

// Translated from 00877f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<RegSettingCollection>::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor body (`00877fc0`, outside this range), then frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00877f90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0087_7fc0, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008705c0, fn_008705c0() -> u8),
        entry!(0x008705d0, fn_008705d0(Ptr)),
        entry!(0x00870610, fn_00870610(Ptr)),
        entry!(0x00870680, fn_00870680(Ptr)),
        entry!(0x008706b0, fn_008706b0(Ptr, u32, u8, u8)),
        entry!(0x00870770, fn_00870770(Ptr) -> u8),
        entry!(0x00870790, fn_00870790(Ptr, u32, u32, u32)),
        entry!(0x008707c0, fn_008707c0(Ptr, u32, u8)),
        entry!(0x008709f0, fn_008709f0() -> u8),
        entry!(0x00870a00, fn_00870a00(Ptr, u32, u8)),
        entry!(0x00870bd0, fn_00870bd0(Ptr, u32, u8)),
        entry!(0x00871220, fn_00871220(Ptr) -> bool),
        entry!(0x00871260, fn_00871260() -> bool),
        entry!(0x00871290, fn_00871290(Ptr)),
        entry!(0x00871a10, fn_00871a10() -> u8),
        entry!(0x00871a30, fn_00871a30(Ptr) -> Ptr),
        entry!(0x00871a50, main_update_offscreen_interface(Ptr)),
        entry!(0x00871c90, main_set_active(Ptr, u8)),
        entry!(0x00871d10, fn_00871d10(Ptr, i32, i32) -> bool),
        entry!(0x00871dc0, main_render_menu_background(Ptr)),
        entry!(0x00872370, fn_00872370(Ptr, u32) -> u32),
        entry!(0x008723d0, fn_008723d0(Ptr, u32) -> u32),
        entry!(0x00872430, fn_00872430(Ptr, u32, u32) -> u8),
        entry!(0x00872570, fn_00872570(u8) -> u8),
        entry!(0x00872780, fn_00872780(Ptr) -> u8),
        entry!(0x008727b0, fn_008727b0(Ptr) -> u8),
        entry!(0x008727d0, fn_008727d0(Ptr)),
        entry!(0x00872930, fn_00872930(u8)),
        entry!(0x00872940, fn_00872940(Ptr, u32)),
        entry!(0x00872ad0, fn_00872ad0(Ptr)),
        entry!(0x00872b00, fn_00872b00(Ptr) -> u32),
        entry!(0x00872dd0, fn_00872dd0(u8, u8)),
        entry!(0x00872df0, fn_00872df0(u8)),
        entry!(0x00872e00, fn_00872e00(Ptr) -> Ptr),
        entry!(0x00872ea0, fn_00872ea0(Ptr, u32) -> Ptr),
        entry!(0x00872ed0, ni_culling_process_append_virtual(Ptr, u32)),
        entry!(0x00872ef0, bs_fade_node_culler_get_rtti(Ptr) -> u32),
        entry!(
            0x00872f00,
            bs_fade_node_culler_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00872f30, fn_00872f30(Ptr)),
        entry!(0x00872f50, fn_00872f50(Ptr, u32, u32, u8, u8) -> u32),
        entry!(0x008731a0, fn_008731a0(f64, f32) -> f64),
        entry!(0x008731d0, fn_008731d0(Ptr)),
        entry!(0x00873200, fn_00873200(Ptr, u32, u32, u8, u8)),
        entry!(0x00874460, fn_00874460(Ptr, u32)),
        entry!(0x00874480, fn_00874480(Ptr) -> u32),
        entry!(0x008744a0, fn_008744a0(u32)),
        entry!(0x00874650, fn_00874650(Ptr) -> u8),
        entry!(0x00874670, fn_00874670(Ptr) -> Ptr),
        entry!(0x00874690, fn_00874690() -> u32),
        entry!(0x008746a0, fn_008746a0() -> u32),
        entry!(0x008746b0, fn_008746b0() -> u32),
        entry!(0x008746c0, fn_008746c0()),
        entry!(0x008746d0, fn_008746d0(Ptr, u32)),
        entry!(0x008748d0, fn_008748d0(Ptr) -> u32),
        entry!(0x00874900, fn_00874900(Ptr) -> f32),
        entry!(0x00874920, fn_00874920(Ptr, u32, u32)),
        entry!(0x008749b0, fn_008749b0(Ptr) -> u8),
        entry!(0x00874aa0, fn_00874aa0(Ptr) -> u8),
        entry!(0x00874ac0, fn_00874ac0(Ptr, u32)),
        entry!(0x00874b50, fn_00874b50(Ptr, u32, u32, u32)),
        entry!(0x00874b90, fn_00874b90(Ptr, u32, u32, u8, u32)),
        entry!(0x00874c10, main_draw_world_mt_setup_1st_person(Ptr)),
        entry!(0x008750e0, fn_008750e0(Ptr, u32, u32, u32)),
        entry!(0x00875110, fn_00875110(Ptr, u32, u32, u32, u32)),
        entry!(0x00875bd0, fn_00875bd0(Ptr, u32) -> u32),
        entry!(0x00875bf0, fn_00875bf0(Ptr)),
        entry!(0x00875e40, fn_00875e40(Ptr, u32, u32)),
        entry!(0x00875fa0, fn_00875fa0(Ptr, u32, u32)),
        entry!(0x00875fd0, fn_00875fd0(Ptr, u32, u32, u32, u32)),
        entry!(0x008761e0, fn_008761e0(Ptr, u32, u32)),
        entry!(0x00876810, fn_00876810(Ptr) -> u8),
        entry!(0x00876830, fn_00876830(Ptr, u32, u32)),
        entry!(0x00876850, fn_00876850(Ptr, u32, u32, u32, u32)),
        entry!(0x00876a00, fn_00876a00(Ptr)),
        entry!(0x00876a20, fn_00876a20(Ptr, u32)),
        entry!(0x00876a70, fn_00876a70(u32, u32)),
        entry!(0x00876ad0, fn_00876ad0(u32)),
        entry!(0x00876d20, main_init_archive()),
        entry!(0x00876dd0, main_init_start_cell(u32, u32)),
        entry!(0x00877260, fn_00877260(u32, u32, u32)),
        entry!(0x00877430, main_kill_menu_bg_texture(Ptr)),
        entry!(0x008774a0, fn_008774a0(Ptr, u8)),
        entry!(0x008776e0, fn_008776e0(Ptr)),
        entry!(0x00877700, fn_00877700(Ptr)),
        entry!(0x00877720, fn_00877720(Ptr) -> Ptr),
        entry!(0x00877730, main_init_sky()),
        entry!(0x00877950, fn_00877950() -> u32),
        entry!(0x00877960, fn_00877960()),
        entry!(0x008779f0, fn_008779f0(Ptr) -> Ptr),
        entry!(0x00877a10, fn_00877a10(Ptr) -> u32),
        entry!(0x00877a30, fn_00877a30(Ptr<NiTArray>, u32) -> u32),
        entry!(0x00877a50, fn_00877a50(Ptr<NiTArray>, u32) -> u32),
        entry!(0x00877a80, fn_00877a80(Ptr, u32) -> Ptr),
        entry!(0x00877ac0, fn_00877ac0(Ptr)),
        entry!(0x00877b40, fn_00877b40(Ptr)),
        entry!(0x00877b60, fn_00877b60(Ptr)),
        entry!(0x00877b80, fn_00877b80(Ptr, u32) -> Ptr),
        entry!(0x00877bb0, fn_00877bb0(Ptr, u32) -> Ptr),
        entry!(0x00877be0, fn_00877be0(Ptr, u32) -> bool),
        entry!(0x00877cc0, fn_00877cc0(Ptr, u32) -> bool),
        entry!(0x00877d30, fn_00877d30(Ptr)),
        entry!(0x00877d50, fn_00877d50(Ptr, u32) -> Ptr),
        entry!(0x00877d80, fn_00877d80(Ptr) -> Ptr),
        entry!(0x00877db0, fn_00877db0(Ptr) -> Ptr),
        entry!(0x00877df0, fn_00877df0(Ptr) -> Ptr),
        entry!(0x00877e10, fn_00877e10(Ptr<NiTArray>, u32, u32) -> u32),
        entry!(0x00877e50, fn_00877e50(Ptr<NiTArray>, u32, u32)),
        entry!(0x00877f10, fn_00877f10(Ptr, u32, u32) -> Ptr),
        entry!(0x00877f90, fn_00877f90(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Returns = Rc<RefCell<HashMap<u32, Vec<Ret>>>>;

    /// Every address this part calls (functions and, harmlessly, globals):
    /// each gets a test double so that nothing another unit translated runs
    /// for real.
    const CALLEES: &[u32] = &[
        0x0040_1030,
        0x0040_3e20,
        0x0040_4010,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_6d30,
        0x0040_6d50,
        0x0040_8d60,
        0x0040_ebd0,
        0x0040_fba0,
        0x0040_fbf0,
        0x0040_fca0,
        0x0041_4430,
        0x0041_fd00,
        0x0043_b230,
        0x0043_c490,
        0x0043_c4b0,
        0x0043_d410,
        0x0043_d450,
        0x0043_d4d0,
        0x0044_0460,
        0x0044_6e10,
        0x0044_a670,
        0x0044_ddc0,
        0x0045_0b80,
        0x0045_0f90,
        0x0045_18e0,
        0x0045_2dc0,
        0x0045_3470,
        0x0045_37b0,
        0x0045_4b30,
        0x0045_6610,
        0x0045_7050,
        0x0045_bb80,
        0x0045_bbe0,
        0x0045_c670,
        0x0045_cd60,
        0x0046_2f40,
        0x0047_1d00,
        0x0047_2380,
        0x0047_6c90,
        0x004a_0370,
        0x004a_03c0,
        0x004a_0ea0,
        0x004a_0eb0,
        0x004a_0f60,
        0x004a_0fd0,
        0x004a_1020,
        0x004a_4040,
        0x004b_7210,
        0x004d_c1f0,
        0x004d_c200,
        0x004d_c310,
        0x004d_e080,
        0x004d_ee10,
        0x004d_ee70,
        0x004e_1bc0,
        0x004e_20b0,
        0x004e_2120,
        0x004e_2190,
        0x004e_3270,
        0x004e_6540,
        0x004e_6560,
        0x004e_6580,
        0x004e_65d0,
        0x004e_6620,
        0x004e_a970,
        0x004e_af60,
        0x004e_bbc0,
        0x0050_0940,
        0x0050_ea90,
        0x0052_4c90,
        0x0052_5420,
        0x0052_99a0,
        0x0052_9c90,
        0x0054_68f0,
        0x0054_77f0,
        0x0054_f4c0,
        0x0055_2570,
        0x0055_8310,
        0x0055_85e0,
        0x0055_9450,
        0x0056_c7f0,
        0x005a_a720,
        0x005b_9b00,
        0x005b_ac20,
        0x005b_f770,
        0x005b_f7b0,
        0x005b_f820,
        0x005d_2860,
        0x005d_2a40,
        0x005d_8b10,
        0x005f_36f0,
        0x005f_9ad0,
        0x005f_9af0,
        0x0063_3c90,
        0x0066_29f0,
        0x0066_b0d0,
        0x0068_3a60,
        0x0070_2360,
        0x0070_2450,
        0x0070_2680,
        0x0070_3080,
        0x0070_5100,
        0x0070_58e0,
        0x0070_5910,
        0x0070_5990,
        0x0070_5a00,
        0x0070_6230,
        0x0070_6320,
        0x0070_6e70,
        0x0070_79b0,
        0x0070_7ad0,
        0x0070_7b30,
        0x0070_94f0,
        0x0070_9ae0,
        0x0070_9af0,
        0x0070_9b00,
        0x0070_9b10,
        0x0070_9b20,
        0x0070_9b30,
        0x0070_9cb0,
        0x0070_ec90,
        0x0071_0ab0,
        0x0071_1e00,
        0x0071_2e60,
        0x0071_48c0,
        0x0071_4960,
        0x0071_4a00,
        0x0071_4c20,
        0x0071_8ab0,
        0x0079_48d0,
        0x007c_9ca0,
        0x007f_a950,
        0x007f_ae70,
        0x007f_df30,
        0x0084_d030,
        0x0084_e3a0,
        0x0086_bdf0,
        0x0086_f640,
        0x0086_f670,
        0x0086_f890,
        0x0086_fd90,
        0x0087_31a0,
        0x0087_31d0,
        0x0087_3200,
        0x0087_46d0,
        0x0087_4920,
        0x0087_49b0,
        0x0087_4ac0,
        0x0087_4b50,
        0x0087_4b90,
        0x0087_4c10,
        0x0087_5110,
        0x0087_5bf0,
        0x0087_5e40,
        0x0087_5fd0,
        0x0087_61e0,
        0x0087_6830,
        0x0087_6850,
        0x0087_6a20,
        0x0087_7430,
        0x0087_a6f0,
        0x0087_a710,
        0x0087_a850,
        0x0087_f200,
        0x008b_6200,
        0x008c_51c0,
        0x008c_80e0,
        0x008d_6f30,
        0x008d_6f50,
        0x008d_80e0,
        0x008d_8520,
        0x0095_0090,
        0x0095_0bb0,
        0x0096_11e0,
        0x0097_9260,
        0x00a0_44f0,
        0x00a0_4500,
        0x00a0_9030,
        0x00a0_d9e0,
        0x00a0_e770,
        0x00a2_3010,
        0x00a2_37b0,
        0x00a2_53d0,
        0x00a5_9c60,
        0x00a5_bdd0,
        0x00a6_93e0,
        0x00a6_9400,
        0x00a6_94a0,
        0x00a6_fdb0,
        0x00a8_1a80,
        0x00aa_5040,
        0x00aa_50a0,
        0x00b4_f540,
        0x00b4_f570,
        0x00b4_f5c0,
        0x00b5_4000,
        0x00b5_40d0,
        0x00b5_afc0,
        0x00b5_b010,
        0x00b5_b060,
        0x00b5_b1f0,
        0x00b5_b880,
        0x00b5_c3c0,
        0x00b5_cbd0,
        0x00b5_cde0,
        0x00b5_d300,
        0x00b5_e870,
        0x00b6_31d0,
        0x00b6_3a90,
        0x00b6_b260,
        0x00b6_b890,
        0x00b6_b8d0,
        0x00b6_bee0,
        0x00b6_c0d0,
        0x00b6_da10,
        0x00b6_e110,
        0x00b6_ee70,
        0x00b8_b1e0,
        0x00b8_b390,
        0x00b9_8480,
        0x00b9_bb10,
        0x00b9_d150,
        0x00b9_dfc0,
        0x00b9_e970,
        0x00b9_fba0,
        0x00ba_0110,
        0x00ba_02b0,
        0x00ba_2f30,
        0x00ba_2fa0,
        0x00ba_30f0,
        0x00ba_3130,
        0x00ba_d4a0,
        0x00c5_2020,
        0x00ec_17c0,
        0x00ec_1800,
        0x00ec_25e0,
        0x00ec_29b0,
        0x00ec_658f,
        0x00ec_9907,
        0x00ec_9a47,
        0x00ec_c104,
        0x00fd_9380,
        0x00fd_f0c8,
        0x00fd_f2a4,
        0x00fd_f2e8,
        0x00fd_f2f8,
        0x00fd_f2fc,
        0x0101_1588,
        0x0101_2060,
        0x0102_1928,
        0x0102_6994,
        0x0103_40a4,
        0x0107_18c0,
        0x0108_29c4,
        0x0108_2cb0,
        0x0108_2cb8,
        0x0108_2cc0,
        0x0108_2ccc,
        0x0108_2d20,
        0x0118_9184,
        0x011a_9608,
        0x011a_9bd0,
        0x011a_d840,
        0x011a_d884,
        0x011c_3ea4,
        0x011c_3f2c,
        0x011c_71dc,
        0x011c_72d8,
        0x011c_72e4,
        0x011c_74d4,
        0x011c_7664,
        0x011c_7810,
        0x011c_7a59,
        0x011c_7adc,
        0x011d_8ba0,
        0x011d_b2cc,
        0x011d_ea0c,
        0x011d_ea10,
        0x011d_ea20,
        0x011d_ea29,
        0x011d_ea3c,
        0x011d_ea8c,
        0x011d_eadc,
        0x011d_eb34,
        0x011d_eb60,
        0x011d_eb7c,
        0x011d_ec64,
        0x011d_ecd8,
        0x011d_ed3c,
        0x011d_ed80,
        0x011d_eda4,
        0x011d_eed8,
        0x011d_ef08,
        0x011d_ef10,
        0x011d_efb4,
        0x011e_0e80,
        0x011f_1180,
        0x011f_2250,
        0x011f_426c,
        0x011f_6394,
        0x011f_91a4,
        0x011f_941e,
        0x011f_9426,
        0x011f_9438,
        0x011f_94a5,
        0x011f_9fc0,
        0x011f_9fc1,
        0x011f_9fc3,
        0x011f_9fc4,
        0x011f_9fc5,
        0x011f_f104,
        0x011f_f8c4,
        0x0120_1f7c,
        0x0126_fac4,
    ];

    /// The engine with a double for every callee, the exe's data pages
    /// mapped (zeroed), and the settings accessors pointing at a zero value.
    struct Rig {
        e: Engine,
        returns: Returns,
        zero: u32,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            for page in (0x0100_0000u32..0x0130_0000).step_by(0x1000) {
                e.map(page, 0x1000);
            }
            let returns: Returns = Rc::new(RefCell::new(HashMap::new()));
            for &addr in CALLEES {
                let table = returns.clone();
                e.register_double(addr, move |_, _| {
                    let mut table = table.borrow_mut();
                    match table.get_mut(&addr) {
                        Some(queue) if queue.len() > 1 => queue.remove(0),
                        Some(queue) if queue.len() == 1 => queue[0],
                        _ => Ret::default(),
                    }
                });
            }
            let zero = e.mem.alloc(0x100);
            let mut rig = Rig { e, returns, zero };
            for setting in [SETTING_DWORD_PTR, SETTING_BYTE_PTR, SETTING_FLOAT_PTR] {
                rig.set(setting, &[zero]);
            }
            // The object `0070ec90` returns is read by `008727d0`.
            let cell = rig.e.mem.alloc(0x40);
            rig.set(0x0070_ec90, &[cell]);
            rig
        }

        /// The values the double at `addr` returns in `eax`, in order; the
        /// last one repeats.
        fn set(&mut self, addr: u32, values: &[u32]) {
            let queue = values
                .iter()
                .map(|&eax| Ret {
                    eax,
                    ..Ret::default()
                })
                .collect();
            self.returns.borrow_mut().insert(addr, queue);
        }

        /// A double that returns `value` in ST0.
        fn set_float(&mut self, addr: u32, value: f64) {
            let ret = Ret {
                st0: value,
                ..Ret::default()
            };
            self.returns.borrow_mut().insert(addr, vec![ret]);
        }

        fn object(&mut self, size: u32) -> u32 {
            self.e.mem.alloc(size)
        }

        /// An object at a fixed address (one mapped page) whose vtable slots
        /// (byte offset, result) are doubles returning the result in `eax`.
        fn object_at(&mut self, object: u32, slots: &[(u32, u32)]) {
            self.e.map(object, 0x1000);
            let vtable = object + 0x100;
            self.e.mem.set_u32(object, vtable);
            for &(offset, result) in slots {
                let target = 0x00f2_0000 + object + offset;
                self.e.register_double(target, move |_, _| Ret {
                    eax: result,
                    ..Ret::default()
                });
                self.e.mem.set_u32(vtable + offset, target);
            }
        }

        /// A settings accessor that answers by the setting it is asked
        /// about (`values`: setting address, value); any other setting reads 0.
        fn settings(&mut self, accessor: u32, values: &[(u32, u32)]) {
            let zero = self.zero;
            let mut blocks: HashMap<u32, u32> = HashMap::new();
            for &(setting, value) in values {
                let block = self.e.mem.alloc(8);
                self.e.mem.set_u32(block, value);
                blocks.insert(setting, block);
            }
            self.e.register_double(accessor, move |_, a| Ret {
                eax: blocks.get(&a[0]).copied().unwrap_or(zero),
                ..Ret::default()
            });
        }

        /// `NiPointer` reads that answer by the slot asked about (`slots`:
        /// slot address, pointer); any other slot is empty.
        fn pointers(&mut self, slots: &[(u32, u32)]) {
            let table: HashMap<u32, u32> = slots.iter().copied().collect();
            self.e.register_double(POINTER_GET, move |_, a| Ret {
                eax: table.get(&a[0]).copied().unwrap_or(0),
                ..Ret::default()
            });
        }

        /// An object whose vtable slots (byte offset, result) are doubles
        /// returning the given result in `eax`.
        fn object_with_slots(&mut self, slots: &[(u32, u32)]) -> u32 {
            let object = self.e.mem.alloc(0x40);
            let vtable = self.e.mem.alloc(0x400);
            self.e.mem.set_u32(object, vtable);
            for &(offset, result) in slots {
                let target = 0x00f0_0000 + (vtable & 0xffff) * 0x400 + offset;
                self.e.register_double(target, move |_, _| Ret {
                    eax: result,
                    ..Ret::default()
                });
                self.e.mem.set_u32(vtable + offset, target);
            }
            object
        }

        /// Makes the settings accessor `accessor` hand out, call by call,
        /// the addresses of blocks holding `values`.
        fn setting(&mut self, accessor: u32, values: &[u32]) {
            let blocks: Vec<u32> = values
                .iter()
                .map(|&value| {
                    let block = self.e.mem.alloc(8);
                    self.e.mem.set_u32(block, value);
                    block
                })
                .collect();
            self.set(accessor, &blocks);
        }

        /// Runs `addr` and returns its result and the calls it made.
        fn run(&mut self, addr: u32, args: &[u32]) -> (Ret, Log) {
            self.e.call_log = Some(vec![]);
            let ret = self.e.call(addr, args);
            let mut log = self.e.call_log.take().unwrap();
            log.remove(0);
            (ret, log)
        }
    }

    fn addrs(log: &Log) -> Vec<u32> {
        log.iter().map(|(addr, _)| *addr).collect()
    }

    /// The addresses of `log` that are in `wanted`, in call order.
    fn only(log: &Log, wanted: &[u32]) -> Vec<u32> {
        addrs(log)
            .into_iter()
            .filter(|addr| wanted.contains(addr))
            .collect()
    }

    /// The argument lists of the calls to `addr`.
    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn flag_getters_read_their_globals() {
        let mut r = Rig::new();
        r.e.set_global(0x0118_9184, 7u8);
        r.e.set_global(0x011f_91a4, 1u8);
        assert_eq!(r.e.call(0x008705c0, &[]).u8(), 7);
        assert_eq!(r.e.call(0x008709f0, &[]).u8(), 1);
    }

    #[test]
    fn idle_step_runs_the_cell_update_only_with_several_threads() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.setting(SETTING_DWORD_PTR, &[2]);
        let (_, log) = r.run(0x008705d0, &args![this]);
        assert_eq!(
            addrs(&log),
            vec![
                0x0086_f640,
                0x0086_f890,
                SETTING_DWORD_PTR,
                0x0055_2570,
                0x0086_f670
            ]
        );
        assert_eq!(calls_to(&log, 0x0086_f640), vec![vec![this]]);
        r.setting(SETTING_DWORD_PTR, &[1]);
        let (_, log) = r.run(0x008705d0, &args![this]);
        assert_eq!(
            addrs(&log),
            vec![0x0086_f640, 0x0086_f890, SETTING_DWORD_PTR, 0x0086_f670]
        );
    }

    #[test]
    fn delayed_actions_step_closes_support_menus_and_prints() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        let interface = r.object(0x600);
        r.e.mem.set_u8(interface + 0x4cc, 1);
        r.set(GET_GLOBAL_011D8A80, &[interface]);
        r.set(GET_GLOBAL_011DF1A8, &[0x1234]);
        r.set(0x00a0_d9e0, &[0x77]);
        r.setting(SETTING_DWORD_PTR, &[2]);
        let (_, log) = r.run(0x00870610, &args![this]);
        assert_eq!(
            addrs(&log),
            vec![
                0x005a_a720,
                SETTING_DWORD_PTR,
                GET_GLOBAL_011DF1A8,
                0x0087_a710,
                GET_GLOBAL_011DF1A8,
                0x0087_a850,
                GET_GLOBAL_011DF1A8,
                0x0087_a6f0,
                0x0070_94f0,
                GET_GLOBAL_011D8A80,
                0x0070_6320,
                0x00a0_d9e0,
                0x00a0_e770,
            ]
        );
        assert_eq!(calls_to(&log, 0x0087_a850), vec![vec![0x1234]]);
        assert_eq!(calls_to(&log, 0x0070_6320), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x00a0_d9e0), vec![vec![1]]);
        assert_eq!(calls_to(&log, 0x00a0_e770), vec![vec![0x77]]);
        assert_eq!(r.e.mem.u8(interface + 0x4cc), 0);

        // One thread: the three object methods are skipped (and the flag
        // is already cleared, so no menu is closed).
        r.setting(SETTING_DWORD_PTR, &[1]);
        let (_, log) = r.run(0x00870610, &args![this]);
        assert!(!addrs(&log).contains(&0x0087_a710));
        assert!(!addrs(&log).contains(&0x0070_6320));
    }

    #[test]
    fn support_menus_close_only_when_flagged() {
        let mut r = Rig::new();
        let this = r.object(0x600);
        let (_, log) = r.run(0x00870680, &args![this]);
        assert!(log.is_empty());
        r.e.mem.set_u8(this + 0x4cc, 1);
        let (_, log) = r.run(0x00870680, &args![this]);
        assert_eq!(log, vec![(0x0070_6320, vec![0])]);
        assert_eq!(r.e.mem.u8(this + 0x4cc), 0);
    }

    #[test]
    fn byte_and_position_accessors() {
        let mut r = Rig::new();
        let object = r.object(0x200);
        r.e.mem.set_u8(object + 0x131, 9);
        assert_eq!(r.e.call(0x00870770, &args![object]).u8(), 9);
        r.e.call(0x00870790, &args![object, 1u32, 2u32, 3u32]);
        assert_eq!(r.e.mem.u32(object + 0x1f0), 1);
        assert_eq!(r.e.mem.u32(object + 0x1f4), 2);
        assert_eq!(r.e.mem.u32(object + 0x1f8), 3);
    }

    /// A rig for the draw sequences: a `Main` object, distinct fake objects
    /// from the getters, and the indexed global 0 with the byte `+0x131`.
    fn frame_rig(byte_131: u8) -> (Rig, u32, u32) {
        let mut r = Rig::new();
        let this = r.object(0x200);
        let indexed = r.object(0x200);
        r.e.mem.set_u8(indexed + 0x131, byte_131);
        r.set(INDEXED_GLOBAL, &[indexed]);
        r.set(GET_GLOBAL_011F4748, &[0x1000]);
        // The renderer-like object: slot 0xc8 returns a group, 0xac a value.
        r.object_at(0x1000, &[(0xc8, 0x6c00), (0xac, 0)]);
        r.set(GET_GLOBAL_011F91AC, &[0x2000]);
        r.set(GET_ROOT, &[0x3000]);
        r.set(GET_ROOT_CHILD, &[0x4000]);
        r.set(GET_ROOT_CHILD_OF_ROOT, &[0x4000]);
        let water_group = r.object(0x100);
        r.set(0x004e_bbc0, &[water_group]);
        let position = r.object(16);
        for (i, word) in [10u32, 20, 30].iter().enumerate() {
            r.e.mem.set_u32(position + 4 * i as u32, *word);
        }
        r.set(MEMBER_8C, &[position]);
        (r, this, indexed)
    }

    #[test]
    fn frame_selector_stores_the_camera_position_and_picks_a_draw() {
        // Byte clear: the plain world frame (the unique `00874ac0` call).
        let (mut r, this, indexed) = frame_rig(0);
        let (_, log) = r.run(0x008706b0, &args![this, 0x5000u32, 0u32, 0u32]);
        assert_eq!(r.e.mem.u32(indexed + 0x1f0), 10);
        assert_eq!(r.e.mem.u32(indexed + 0x1f4), 20);
        assert_eq!(r.e.mem.u32(indexed + 0x1f8), 30);
        assert!(addrs(&log).contains(&0x0087_4ac0));
        assert!(!addrs(&log).contains(&0x00b9_8480));

        // Byte set: the variant with the `00b98480` call.
        let (mut r, this, _) = frame_rig(1);
        let (_, log) = r.run(0x008706b0, &args![this, 0x5000u32, 0u32, 0u32]);
        assert!(addrs(&log).contains(&0x00b9_8480));
        assert!(!addrs(&log).contains(&0x0087_4ac0));

        // Menu background requested and menu 0x3f5 not visible.
        let (mut r, this, _) = frame_rig(1);
        let (_, log) = r.run(0x008706b0, &args![this, 0x5000u32, 1u32, 0u32]);
        assert_eq!(calls_to(&log, IS_MENU_ID_VISIBLE)[0], vec![0x3f5, 0]);
        assert!(addrs(&log).contains(&0x0087_5bf0));
        assert!(!addrs(&log).contains(&0x00b9_8480));

        // Menu background requested but 0x3f5 visible: the byte decides.
        let (mut r, this, _) = frame_rig(1);
        r.set(IS_MENU_ID_VISIBLE, &[1]);
        let (_, log) = r.run(0x008706b0, &args![this, 0x5000u32, 1u32, 0u32]);
        assert!(addrs(&log).contains(&0x00b9_8480));
    }

    /// The stage argument of every call to a `SetThreadStage`-style address.
    fn stages(log: &Log, addr: u32) -> Vec<u32> {
        calls_to(log, addr).iter().map(|a| a[2]).collect()
    }

    #[test]
    fn menu_background_frame_resets_the_stages_and_draws_the_menu_pass() {
        let (mut r, this, _) = frame_rig(0);
        r.set(GET_MT_SYSTEM, &[0x0120_0088]);
        r.set(0x004d_c310, &[1]);
        let (_, log) = r.run(0x008707c0, &args![this, 0x5000u32, 0u32]);
        let expected: Vec<u32> = (1..=0x16).collect();
        assert_eq!(stages(&log, MT_SET_THREAD_STAGE), expected);
        assert_eq!(stages(&log, MT_ADVANCE_THREAD_STAGE), expected);
        let first = calls_to(&log, MT_SET_THREAD_STAGE)[0].clone();
        assert_eq!(first, vec![0x0120_0088, 0, 1]);
        let first = calls_to(&log, MT_ADVANCE_THREAD_STAGE)[0].clone();
        assert_eq!(first, vec![0x0120_0088, 1, 1]);
        // Not the first-person frame (its `8709f0` flag is clear): the
        // target is cleared with `00876830`.
        assert_eq!(
            calls_to(&log, 0x0087_4b90),
            vec![vec![this, 0x2000, 0x1000, 1, 0x5000]]
        );
        assert_eq!(calls_to(&log, 0x0087_6830), vec![vec![this, 1, 0x5000]]);
        assert_eq!(
            calls_to(&log, 0x0087_6850),
            vec![vec![this, 0, 0x2000, 0x1000, 0]]
        );
        assert!(!addrs(&log).contains(&0x0087_5110));
        // The byte flag is set and a target is given: the renderer's own
        // group (vtable slot 0xc8) is made current.
        assert_eq!(calls_to(&log, 0x00b6_b8d0), vec![vec![7, 0x6c00]]);
    }

    #[test]
    fn menu_background_frame_in_the_pipboy_draws_the_conversation_target() {
        let (mut r, this, indexed) = frame_rig(0);
        r.set(IS_IN_PIPBOY_MENU, &[1]);
        r.set(GET_CURRENT_RENDERED_MENU, &[0x6000]);
        r.set(0x007f_a950, &[0x7000]);
        r.set(0x008d_80e0, &[0x8000]);
        r.pointers(&[(this + MAIN_FIRST_PERSON_ACCUM, 0x9000)]);
        let (_, log) = r.run(0x008707c0, &args![this, 0u32, 0u32]);
        assert_eq!(calls_to(&log, 0x004a_0fd0), vec![vec![0x8000, 0x9000]]);
        assert_eq!(calls_to(&log, 0x0041_fd00), vec![vec![0x8000, 0x4000]]);
        assert_eq!(calls_to(&log, 0x00b5_e870), vec![vec![indexed, 0x8000]]);
        assert_eq!(
            calls_to(&log, 0x0087_5110),
            vec![vec![this, 0x1000, 0x7000, 0, 0]]
        );
        // Stage 0x16 is not advanced in the pipboy.
        assert!(!stages(&log, MT_SET_THREAD_STAGE).contains(&0x16));
    }

    #[test]
    fn menu_background_frame_with_the_overlay_flag_draws_the_rendered_menu() {
        let (mut r, this, _) = frame_rig(0);
        r.e.set_global(0x011f_91a4, 1u8);
        r.set(GET_CURRENT_RENDERED_MENU, &[0x6000]);
        r.set(0x007f_a950, &[0x7000]);
        r.set(0x00b6_e110, &[0xf000]);
        r.set(0x004d_c310, &[1]);
        r.setting(SETTING_DWORD_PTR, &[2]);
        let (_, log) = r.run(0x008707c0, &args![this, 0u32, 0u32]);
        // The multithread setting is above 1.
        assert_eq!(calls_to(&log, 0x008c_80e0), vec![vec![0]]);
        assert_eq!(
            calls_to(&log, 0x0087_5fd0),
            vec![vec![this, 1, 0xf000, 0x1000, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0087_61e0),
            vec![vec![this, 0x7000, 0x1000]]
        );
        assert_eq!(
            calls_to(&log, 0x0087_6850),
            vec![vec![this, 0xf000, 0x2000, 0x1000, 0]]
        );

        // With the byte `+0x131` set (the flag argument), the target is
        // not created and `00876830` runs for a non-zero target instead.
        let (mut r, this, _) = frame_rig(0);
        r.e.set_global(0x011f_91a4, 1u8);
        let (_, log) = r.run(0x008707c0, &args![this, 0x5000u32, 1u32]);
        assert!(!addrs(&log).contains(&0x0087_5fd0));
        assert_eq!(calls_to(&log, 0x0087_6830), vec![vec![this, 0, 0x5000]]);
    }

    #[test]
    fn world_frame_with_depth_pass_draws_the_world_when_allowed() {
        let (mut r, this, _) = frame_rig(1);
        r.set(0x0087_49b0, &[5]);
        r.set(0x004d_c310, &[1]);
        let (_, log) = r.run(0x00870a00, &args![this, 0x5000u32, 0u32]);
        assert_eq!(calls_to(&log, 0x0087_3200), vec![vec![this, 0, 1, 1, 5]]);
        assert_eq!(
            only(&log, &[0x0087_4c10, 0x0087_4b50, 0x0087_5110, 0x0087_6830]),
            vec![
                0x0087_4c10,
                0x0087_4b50,
                0x0087_5110,
                0x0087_6830,
                0x0087_6830
            ]
        );
        assert_eq!(calls_to(&log, 0x0087_4b50), vec![vec![this, 0, 1, 0]]);
        assert_eq!(
            calls_to(&log, 0x0087_5110),
            vec![vec![this, 0x1000, 0, 0, 0]]
        );
        // Once before and once after the overlay test (flag clear).
        assert_eq!(
            calls_to(&log, 0x0087_6830),
            vec![vec![this, 1, 0x5000], vec![this, 1, 0x5000]]
        );
        assert_eq!(calls_to(&log, 0x0071_4c20), vec![vec![1]]);
        assert_eq!(calls_to(&log, 0x00b9_8480), vec![vec![1, 1]]);
        assert_eq!(
            calls_to(&log, 0x0087_6850),
            vec![vec![this, 0, 0x2000, 0x1000, 0]]
        );
    }

    #[test]
    fn world_frame_with_depth_pass_skips_the_world_for_each_reason() {
        // The skip argument.
        let (mut r, this, _) = frame_rig(1);
        let (_, log) = r.run(0x00870a00, &args![this, 0u32, 1u32]);
        assert_eq!(calls_to(&log, 0x0087_3200), vec![vec![this, 0, 0, 1, 0]]);
        assert!(!addrs(&log).contains(&0x0087_5110));
        let set: Vec<u32> = (1..=0x14).chain([0x16]).collect();
        assert_eq!(stages(&log, MT_SET_THREAD_STAGE), set);
        // The flag of the player's child: the other tests are not asked.
        let (mut r, this, _) = frame_rig(1);
        r.set(0x0045_6610, &[1]);
        let (_, log) = r.run(0x00870a00, &args![this, 0u32, 0u32]);
        assert!(!addrs(&log).contains(&0x0055_85e0));
        assert!(!addrs(&log).contains(&0x0087_5110));
        // `005585e0` of the interface object.
        let (mut r, this, _) = frame_rig(1);
        r.set(0x0070_5910, &[0x77]);
        r.set(0x0055_85e0, &[1]);
        let (_, log) = r.run(0x00870a00, &args![this, 0u32, 0u32]);
        assert_eq!(calls_to(&log, 0x0055_85e0), vec![vec![0x77]]);
        assert!(!addrs(&log).contains(&0x0070_5100));
        assert!(!addrs(&log).contains(&0x0087_5110));
        // `00705100`.
        let (mut r, this, _) = frame_rig(1);
        r.set(0x0070_5100, &[1]);
        let (_, log) = r.run(0x00870a00, &args![this, 0u32, 0u32]);
        assert!(!addrs(&log).contains(&0x0087_5110));
    }

    #[test]
    fn world_frame_with_depth_pass_uses_the_overlay_draw_when_flagged() {
        let (mut r, this, _) = frame_rig(1);
        r.e.set_global(0x011f_91a4, 1u8);
        r.set(0x004d_c310, &[1]);
        let (_, log) = r.run(0x00870a00, &args![this, 0x5000u32, 0u32]);
        // The first `8709f0` also makes `00872f50` create a target.
        assert_eq!(calls_to(&log, 0x0087_5fd0).len(), 1);
        assert_eq!(calls_to(&log, 0x0087_5fd0)[0][..3], [this, 1, 0]);
        assert_eq!(calls_to(&log, 0x0087_6830).len(), 1);
    }

    #[test]
    fn plain_world_frame_orders_its_passes() {
        let (mut r, this, _) = frame_rig(0);
        r.set(0x0087_49b0, &[5]);
        r.set(0x004d_c310, &[1]);
        let (_, log) = r.run(0x00870bd0, &args![this, 0x5000u32, 0u32]);
        assert_eq!(calls_to(&log, 0x0087_4ac0), vec![vec![this, 5]]);
        assert_eq!(
            calls_to(&log, 0x0087_4b50),
            vec![vec![this, 0, 1, 5], vec![this, 0, 1, 0]]
        );
        assert_eq!(calls_to(&log, 0x0087_3200), vec![vec![this, 0, 1, 0, 5]]);
        assert_eq!(
            calls_to(&log, 0x0087_5110),
            vec![vec![this, 0x1000, 0, 0, 0]]
        );
        // No water pass: the second thread's stage 0x16 is advanced.
        let advanced: Vec<u32> = (1..=0x14).chain([0x16]).collect();
        assert_eq!(stages(&log, MT_ADVANCE_THREAD_STAGE), advanced);
        assert_eq!(
            calls_to(&log, 0x0087_6850),
            vec![vec![this, 0, 0x2000, 0x1000, 0]]
        );
        assert_eq!(calls_to(&log, 0x0087_6830), vec![vec![this, 1, 0x5000]]);
    }

    #[test]
    fn plain_world_frame_saves_and_restores_the_accumulator_flag() {
        let (mut r, this, _) = frame_rig(0);
        r.set(0x0043_b230, &[0x7777]);
        // `00872b00` returns the object `0045cd60` gives for the world's
        // `008d8520`, when `00476c90` is 2 or 3.
        r.set(0x008d_8520, &[0x6000]);
        r.set(0x0045_cd60, &[0x6100]);
        r.set(0x0047_6c90, &[3]);
        let (_, log) = r.run(0x00870bd0, &args![this, 0u32, 0u32]);
        assert_eq!(calls_to(&log, 0x0087_3200)[0][1], 0x6100);
        // Forced on, then restored to the flag that was read (0).
        assert_eq!(
            calls_to(&log, 0x0045_0f90),
            vec![vec![0x7777, 1], vec![0x7777, 0]]
        );
    }

    #[test]
    fn plain_world_frame_letterboxes_with_the_scaled_width() {
        let (mut r, this, _) = frame_rig(0);
        r.set(0x0070_9cb0, &[1]);
        r.set(0x004d_ee10, &[1000]);
        r.set(0x004d_ee70, &[500]);
        r.set_float(0x0070_6e70, 1.0);
        // The aspect setting (a float) and the scale constant.
        let aspect_block = r.object(8);
        r.e.mem.set_f32(aspect_block, 1.0);
        r.set(SETTING_FLOAT_PTR, &[aspect_block]);
        r.e.set_global(0x0101_1588, 0.5f64);
        let (_, log) = r.run(0x00870bd0, &args![this, 0u32, 0u32]);
        assert_eq!(calls_to(&log, 0x00b4_f540), vec![vec![500, 0, 500, 500]]);
        assert_eq!(calls_to(&log, 0x00b5_40d0).len(), 1);
    }

    #[test]
    fn plain_world_frame_letterbox_is_skipped_when_disabled() {
        let (mut r, this, _) = frame_rig(0);
        r.set(0x0070_9cb0, &[1]);
        r.set(0x004d_e080, &[1]);
        r.set(0x004d_ee10, &[1000]);
        let renderer = r.object_with_slots(&[(0xac, 0)]);
        r.set(GET_GLOBAL_011F4748, &[renderer]);
        let (_, log) = r.run(0x00870bd0, &args![this, 0u32, 0u32]);
        assert!(!addrs(&log).contains(&0x00b4_f540));
        assert!(!addrs(&log).contains(&0x00b5_40d0));
    }

    #[test]
    fn plain_world_frame_resets_the_clear_colors_after_the_letterbox() {
        let (mut r, this, _) = frame_rig(0);
        r.set(0x0070_9cb0, &[1]);
        r.set(0x004d_e080, &[1]);
        r.e.set_global(0x0102_6994, 0.25f32);
        r.e.set_global(0x0103_40a4, 0.75f32);
        let renderer = r.object_with_slots(&[(0xac, 0)]);
        r.set(GET_GLOBAL_011F4748, &[renderer]);
        let colors: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(vec![]));
        {
            let colors = colors.clone();
            r.e.register_double(0x0071_48c0, move |e, a| {
                let at = a[1];
                colors.borrow_mut().push([
                    e.mem.f32(at),
                    e.mem.f32(at + 4),
                    e.mem.f32(at + 8),
                    e.mem.f32(at + 12),
                ]);
                Ret::default()
            });
        }
        let (_, log) = r.run(0x00870bd0, &args![this, 0u32, 0u32]);
        assert_eq!(
            *colors.borrow(),
            vec![[0.0, 0.25, 1.0, 0.0], [0.75, 1.0, 1.0, 0.0]]
        );
        // The color is built with zeros first; both are set on the renderer.
        assert_eq!(calls_to(&log, 0x0041_4430)[0][1..], [0, 0, 0, 0]);
        assert_eq!(calls_to(&log, 0x0071_48c0).len(), 2);
        assert_eq!(calls_to(&log, 0x0071_48c0)[0][0], renderer);
        assert_eq!(calls_to(&log, 0x0071_48c0)[0][2], 1);
    }

    #[test]
    fn plain_world_frame_draws_the_water_pass_when_the_level_is_high_enough() {
        let (mut r, this, _) = frame_rig(0);
        let water = r.object(0x100);
        r.e.mem.set_f32(water + 0x70, 2.0);
        r.e.set_global(0x0101_2060, 1.0f64);
        r.set(0x004e_bbc0, &[water]);
        r.pointers(&[(this + MAIN_WORLD_ACCUM, 0x9000)]);
        r.set(0x005b_9b00, &[1]);
        r.set(0x00b6_3a90, &[3]);
        let (_, log) = r.run(0x00870bd0, &args![this, 0u32, 0u32]);
        assert_eq!(
            only(&log, &[0x0087_46d0, 0x0087_4920, 0x0087_4c10, 0x0087_5e40]),
            vec![0x0087_46d0, 0x0087_4920, 0x0087_4c10, 0x0087_5e40]
        );
        assert_eq!(calls_to(&log, 0x0087_46d0), vec![vec![this, 0]]);
        assert_eq!(calls_to(&log, 0x0087_4920), vec![vec![this, 0x9000, 0]]);
        assert_eq!(calls_to(&log, 0x0087_5e40), vec![vec![this, 0x9000, 0]]);
        // The water pass already advanced the second thread: no advance.
        assert!(!stages(&log, MT_ADVANCE_THREAD_STAGE).contains(&0x16));
    }

    #[test]
    fn plain_world_frame_projects_the_picked_reference() {
        let (mut r, this, _) = frame_rig(0);
        r.set(0x00b8_b390, &[1]);
        r.settings(SETTING_BYTE_PTR, &[(0x011d_eadc, 1)]);
        r.e.set_global(0x011d_ea20, 0x5555u32);
        let node = r.object(0x40);
        let reference = r.object_with_slots(&[(4, node)]);
        r.set(0x0045_cd60, &[reference]);
        let pair: Rc<RefCell<Vec<f32>>> = Rc::new(RefCell::new(vec![]));
        r.e.register_double(0x00a6_fdb0, |e, a| {
            // The outputs are the third and fourth arguments.
            e.mem.set_f32(a[2], 2.5);
            e.mem.set_f32(a[3], 3.5);
            Ret::default()
        });
        {
            let pair = pair.clone();
            r.e.register_double(POINT2_CTOR, move |_, a| {
                pair.borrow_mut()
                    .extend([f32::from_bits(a[1]), f32::from_bits(a[2])]);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let (_, log) = r.run(0x00870bd0, &args![this, 0u32, 0u32]);
        assert_eq!(*pair.borrow(), vec![2.5, 3.5]);
        let projection = calls_to(&log, 0x00a6_fdb0);
        assert_eq!(projection.len(), 1);
        assert_eq!(projection[0][0], 0x4000);
        let handed = calls_to(&log, 0x00b8_b1e0);
        assert_eq!(handed.len(), 1);
        assert_eq!(handed[0][0], 0x2000);
    }

    #[test]
    fn plain_world_frame_draws_the_overlays_when_flagged() {
        let (mut r, this, _) = frame_rig(0);
        r.e.set_global(0x011f_91a4, 1u8);
        r.set(0x0045_3470, &[1]);
        r.pointers(&[(0x011c_7810, 0x5000)]);
        let (_, log) = r.run(0x00870bd0, &args![this, 0x6000u32, 0u32]);
        assert_eq!(calls_to(&log, 0x0087_5fd0).len(), 1);
        assert_eq!(calls_to(&log, 0x0087_6a20), vec![vec![this, 0x1000]]);
        assert!(!addrs(&log).contains(&0x0087_6830));
        // In the pipboy the extra pass is skipped.
        let (mut r, this, _) = frame_rig(0);
        r.e.set_global(0x011f_91a4, 1u8);
        r.set(0x0045_3470, &[1]);
        r.pointers(&[(0x011c_7810, 0x5000)]);
        r.set(IS_IN_PIPBOY_MENU, &[1]);
        let (_, log) = r.run(0x00870bd0, &args![this, 0x6000u32, 0u32]);
        assert!(!addrs(&log).contains(&0x0087_6a20));
    }

    #[test]
    fn water_level_compares_against_the_double() {
        let mut r = Rig::new();
        let object = r.object(0x100);
        r.e.set_global(0x0101_2060, 1.0f64);
        r.e.mem.set_f32(object + 0x70, 2.0);
        assert!(r.e.call(0x0087_1220, &args![object]).bool());
        r.e.mem.set_f32(object + 0x70, 1.0);
        assert!(!r.e.call(0x0087_1220, &args![object]).bool());
        r.e.mem.set_f32(object + 0x70, f32::NAN);
        assert!(!r.e.call(0x0087_1220, &args![object]).bool());
    }

    #[test]
    fn pointer_test_needs_an_object_with_a_non_zero_answer() {
        let mut r = Rig::new();
        assert!(!r.e.call(0x0087_1260, &args![]).bool());
        r.pointers(&[(0x011c_7810, 0x5000)]);
        assert!(!r.e.call(0x0087_1260, &args![]).bool());
        r.set(0x0045_3470, &[2]);
        let (ret, log) = r.run(0x0087_1260, &args![]);
        assert!(ret.bool());
        assert_eq!(calls_to(&log, 0x0045_3470), vec![vec![0x5000]]);
    }

    /// A float's bit pattern as an argument word.
    fn word(value: f32) -> u32 {
        value.to_bits()
    }

    /// The address a vtable slot of `object` points at.
    fn slot_target(r: &Rig, object: u32, offset: u32) -> u32 {
        let vtable = r.e.mem.u32(object);
        r.e.mem.u32(vtable + offset)
    }

    #[test]
    fn shadow_preparation_starts_the_special_state() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.set(0x005b_f7b0, &[1]);
        r.set(0x0052_5420, &[1]);
        r.set(GET_GLOBAL_011F4748, &[0x1111]);
        r.set(GET_GLOBAL_011F91A8, &[0x2222]);
        r.set(INDEXED_GLOBAL, &[0x3333]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011d_ef08), 1);
        assert_eq!(calls_to(&log, 0x005b_f770), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x00b5_b1f0), vec![vec![0x3333]]);
        assert_eq!(calls_to(&log, 0x00b6_ee70), vec![vec![0x2222, 0x1111, 0]]);
        // The shadow setting byte is clear: the stages 0x11 to 0x14 advance.
        assert_eq!(
            stages(&log, MT_SET_THREAD_STAGE),
            vec![0x11, 0x12, 0x13, 0x14]
        );
        assert_eq!(
            stages(&log, MT_ADVANCE_THREAD_STAGE),
            vec![0x11, 0x12, 0x13, 0x14]
        );
        // It ends with the offscreen-interface update.
        assert_eq!(*addrs(&log).last().unwrap(), IS_IN_MENU_MODE);
    }

    #[test]
    fn shadow_preparation_ends_the_special_state_with_the_caster_limit() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.e.set_global(0x011d_ef08, 1u8);
        r.set(GET_GLOBAL_011F4748, &[0x1111]);
        r.set(GET_GLOBAL_011F91A8, &[0x2222]);
        r.settings(SETTING_DWORD_PTR, &[(0x011c_72e4, 7), (0x011c_72d8, 9)]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011d_ef08), 0);
        assert_eq!(calls_to(&log, 0x005b_f770), vec![vec![1]]);
        assert_eq!(calls_to(&log, 0x00b6_ee70), vec![vec![0x2222, 0x1111, 7]]);

        // With `005f36f0` non-zero the other setting is the limit.
        r.e.set_global(0x011d_ef08, 1u8);
        r.set(0x005f_36f0, &[1]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(calls_to(&log, 0x00b6_ee70), vec![vec![0x2222, 0x1111, 9]]);

        // The state is not ended while `00525420` still holds.
        r.e.set_global(0x011d_ef08, 1u8);
        r.set(0x0052_5420, &[1]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert!(!addrs(&log).contains(&0x005b_f770));
        assert_eq!(r.e.global::<u8>(0x011d_ef08), 1);
    }

    #[test]
    fn shadow_preparation_with_no_casters_only_resets_the_scene() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.set(0x005b_f7b0, &[1]);
        r.set(INDEXED_GLOBAL, &[0x3333]);
        r.set(GET_GLOBAL_011F4748, &[0x1111]);
        r.set(GET_GLOBAL_011F91A8, &[0x2222]);
        r.settings(SETTING_BYTE_PTR, &[(0x011c_74d4, 1)]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(
            only(
                &log,
                &[
                    0x00b4_f570,
                    0x00b5_b060,
                    0x00b6_ee70,
                    0x00b5_c3c0,
                    0x00b5_cbd0
                ]
            ),
            vec![0x00b4_f570, 0x00b5_b060, 0x00b6_ee70, 0x00b5_c3c0]
        );
        assert_eq!(calls_to(&log, 0x00b5_b060), vec![vec![0x3333]]);
        assert_eq!(calls_to(&log, 0x00b5_c3c0), vec![vec![0x3333]]);
        // The stage loop of the other branch did not run.
        assert!(stages(&log, MT_SET_THREAD_STAGE).is_empty());
    }

    /// The rig for the caster pick: scene `0x3333`, the player object, one
    /// actor in the list at `0x011e0f08`, limit `limit`.
    fn caster_rig(limit: u32) -> (Rig, u32, u32) {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.set(0x005b_f7b0, &[1]);
        r.set(INDEXED_GLOBAL, &[0x3333]);
        r.set(GET_GLOBAL_011F4748, &[0x1111]);
        r.set(GET_GLOBAL_011F91A8, &[0x2222]);
        r.settings(SETTING_BYTE_PTR, &[(0x011c_74d4, 1)]);
        r.settings(SETTING_DWORD_PTR, &[(0x011c_72e4, limit)]);
        r.e.set_global(PLAYER_OBJECT, 0x5100u32);
        r.set(0x004e_af60, &[1]);
        r.set(0x0095_0bb0, &[0x6000, 0]);
        r.set(0x00b5_cbd0, &[0x6100]);
        let actor = r.object_with_slots(&[(0x1d0, 0x7000), (0x214, 1)]);
        r.e.mem.set_u32(0x011e_0f08, actor);
        r.set(0x0047_2380, &[1]);
        r.set(0x0096_11e0, &[1]);
        r.set(0x0087_f200, &[1]);
        r.set(0x0050_ea90, &[0x7001]);
        r.set(0x0043_d450, &[0x7002]);
        r.set_float(0x0084_d030, -1.0);
        (r, this, actor)
    }

    #[test]
    fn casters_are_the_player_then_the_actors_that_pass_the_checks() {
        let (mut r, this, _) = caster_rig(2);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(
            calls_to(&log, 0x00b5_cbd0),
            vec![vec![0x3333, 0x6000], vec![0x3333, 0x7000]]
        );
        assert_eq!(calls_to(&log, 0x0040_fbf0), vec![vec![0x011f_1180, 0]]);
        assert_eq!(calls_to(&log, 0x0040_fba0), vec![vec![0x011f_1180]]);
        // The extra-data key and process were asked for the radius test.
        assert_eq!(calls_to(&log, 0x00a5_bdd0), vec![vec![0x7000, 0x7001]]);
        assert_eq!(calls_to(&log, 0x0043_d450), vec![vec![0x7000]]);
        // The player's first caster is handed back to the scene.
        assert_eq!(calls_to(&log, 0x00b5_cde0), vec![vec![0x3333, 0x6100]]);
        // The player's child is gone the second time: the walk is skipped.
        assert!(!addrs(&log).contains(&0x00b5_afc0));
    }

    #[test]
    fn casters_stop_at_the_limit() {
        let (mut r, this, _) = caster_rig(1);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(calls_to(&log, 0x00b5_cbd0), vec![vec![0x3333, 0x6000]]);
        assert_eq!(calls_to(&log, 0x0096_11e0).len(), 0);
    }

    #[test]
    fn actors_that_fail_a_check_are_not_casters() {
        // Not loaded, no 3D data, dead (mode 4), refractive, too far.
        for (case, addr, value) in [
            (0, 0x0096_11e0u32, 0u32),
            (1, 0x0087_f200, 0),
            (3, 0x008c_51c0, 1),
        ] {
            let (mut r, this, _) = caster_rig(2);
            r.set(addr, &[value]);
            if case == 3 {
                // The player check also asks `008c51c0`; only the actor's
                // answer is changed.
                r.set(addr, &[0, 1]);
            }
            let (_, log) = r.run(0x0087_1290, &args![this]);
            assert_eq!(calls_to(&log, 0x00b5_cbd0).len(), 1, "case {case}");
        }
        let (mut r, this, actor) = caster_rig(2);
        let mode_slot = slot_target(&r, actor, 0x214);
        r.e.register_double(mode_slot, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(calls_to(&log, 0x00b5_cbd0).len(), 1);
        // Too far: the radius is not below the scaled position.
        let (mut r, this, _) = caster_rig(2);
        r.set_float(0x0084_d030, 5.0);
        r.set_float(FLOAT_HELPER_A, 1.0);
        r.e.set_global(0x0102_1928, 3.0f64);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(calls_to(&log, 0x00b5_cbd0).len(), 1);
        // Near enough: 2.0 < 1.0 * 3.0.
        let (mut r, this, _) = caster_rig(2);
        r.set_float(0x0084_d030, 2.0);
        r.set_float(FLOAT_HELPER_A, 1.0);
        r.e.set_global(0x0102_1928, 3.0f64);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(calls_to(&log, 0x00b5_cbd0).len(), 2);
    }

    #[test]
    fn the_light_walk_ends_with_the_remaining_stages_and_the_scene_update() {
        let (mut r, this, _) = caster_rig(1);
        r.set(0x0095_0bb0, &[0x6000]);
        r.set(GET_ROOT, &[0x3000]);
        r.set(GET_ROOT_CHILD, &[0x4000]);
        r.set(0x008d_80e0, &[0x8000]);
        r.set(MEMBER_8C_B, &[0x9000]);
        r.set(0x00b5_afc0, &[0x7100]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(calls_to(&log, 0x0041_fd00), vec![vec![0x8000, 0x4000]]);
        assert_eq!(calls_to(&log, 0x00a6_94a0), vec![vec![0x8000, 0x9000]]);
        // The child is flagged, then restored to the flag read (0).
        assert_eq!(
            calls_to(&log, 0x0045_0f90),
            vec![vec![0x6000, 1], vec![0x6000, 0]]
        );
        assert_eq!(calls_to(&log, 0x004e_20b0), vec![vec![0], vec![1]]);
        // One light, without geometry: the walk moves on; no light was
        // queued, so the four remaining stages run on thread 0.
        assert_eq!(calls_to(&log, 0x004e_6540), vec![vec![0x7100]]);
        assert_eq!(calls_to(&log, 0x00b5_b010), vec![vec![0x3333]]);
        assert_eq!(
            stages(&log, MT_SET_THREAD_STAGE),
            vec![0x11, 0x12, 0x13, 0x14]
        );
        assert!(stages(&log, MT_ADVANCE_THREAD_STAGE).is_empty());
        assert_eq!(calls_to(&log, 0x00b5_b880), vec![vec![0x3333, 0x8000]]);
    }

    #[test]
    fn the_saved_object_is_flagged_during_the_walk_unless_the_setting_says_not() {
        let (mut r, this, _) = caster_rig(1);
        r.set(0x0095_0bb0, &[0x6000]);
        r.set(GET_GLOBAL_011CA438, &[0x6500]);
        r.set(0x0045_6610, &[0]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        let flags = calls_to(&log, 0x0045_0f90);
        assert_eq!(
            flags,
            vec![
                vec![0x6000, 1],
                vec![0x6500, 1],
                vec![0x6500, 0],
                vec![0x6000, 0]
            ]
        );
        // With the byte setting `0x011c71dc` the object is left alone first.
        let (mut r, this, _) = caster_rig(1);
        r.set(0x0095_0bb0, &[0x6000]);
        r.set(GET_GLOBAL_011CA438, &[0x6500]);
        r.settings(SETTING_BYTE_PTR, &[(0x011c_74d4, 1), (0x011c_71dc, 1)]);
        let (_, log) = r.run(0x0087_1290, &args![this]);
        assert_eq!(
            calls_to(&log, 0x0045_0f90),
            vec![vec![0x6000, 1], vec![0x6500, 0], vec![0x6000, 0]]
        );
    }

    /// Calls `shadow_light` for a light `0x7100` with the objects set up.
    fn light_rig() -> (Rig, u32) {
        let mut r = Rig::new();
        let indexed = r.object(0x40);
        r.set(INDEXED_GLOBAL, &[indexed]);
        r.set(0x004e_6540, &[0x5001]);
        r.set(0x005b_f820, &[0x5002]);
        r.set(0x004e_6580, &[1]);
        let position = r.object(16);
        for (i, word) in [1u32, 2, 3].iter().enumerate() {
            r.e.mem.set_u32(position + 4 * i as u32, *word);
        }
        r.set(MEMBER_8C, &[position]);
        r.set(0x004e_6560, &[0x10]);
        r.set_float(0x0097_9260, 1.0);
        r.e.set_global(0x0108_2cb0, 0.05f64);
        (r, indexed)
    }

    fn walk_light(r: &mut Rig, limit: i32, processed: i32) -> (i32, Log) {
        r.e.call_log = Some(vec![]);
        let mut processed = processed;
        shadow_light(&mut r.e, 0x7100, 0x8000, limit, &mut processed);
        (processed, r.e.call_log.take().unwrap())
    }

    #[test]
    fn a_light_without_geometry_or_source_is_left_alone() {
        let (mut r, _) = light_rig();
        r.set(0x004e_6540, &[0]);
        let (processed, log) = walk_light(&mut r, 4, 0);
        assert_eq!(processed, 0);
        assert_eq!(addrs(&log), vec![0x004e_6540]);
        let (mut r, _) = light_rig();
        r.set(0x005b_f820, &[0]);
        let (_, log) = walk_light(&mut r, 4, 0);
        assert_eq!(addrs(&log), vec![0x004e_6540, 0x005b_f820]);
    }

    #[test]
    fn a_usable_light_is_recomputed_and_queued() {
        let (mut r, indexed) = light_rig();
        r.settings(SETTING_BYTE_PTR, &[(0x011c_7664, 1)]);
        let (processed, log) = walk_light(&mut r, 4, 1);
        assert_eq!(processed, 2);
        assert_eq!(calls_to(&log, 0x00b9_d150), vec![vec![0x7100, 0, 1, 2, 3]]);
        assert_eq!(calls_to(&log, 0x00b9_e970), vec![vec![0x7100, 0x8000]]);
        // The ordinal is the count before this light.
        assert_eq!(calls_to(&log, 0x00b9_dfc0), vec![vec![0x7100, 0x8000, 1]]);
        assert_eq!(
            calls_to(&log, 0x00a5_9c60),
            vec![vec![0x5001, calls_to(&log, 0x0043_d410)[0][0]]]
        );
        assert_eq!(calls_to(&log, 0x0043_d410)[0][1..], [0, 0, 0]);
        assert_eq!(calls_to(&log, 0x00b5_d300), vec![vec![indexed, 0x7100]]);
        assert_eq!(calls_to(&log, 0x00ba_0110), vec![vec![0x7100, 0x5002]]);
        assert_eq!(calls_to(&log, 0x00ba_02b0), vec![vec![0x7100]]);
        assert!(calls_to(&log, 0x0045_0f90).is_empty());
        // Without the setting the light is not added to the source's list.
        let (mut r, _) = light_rig();
        let (_, log) = walk_light(&mut r, 4, 0);
        assert!(calls_to(&log, 0x00ba_0110).is_empty());
    }

    #[test]
    fn a_light_that_is_too_dim_or_full_is_switched_off() {
        // Weight below the cutoff.
        let (mut r, _) = light_rig();
        r.set_float(0x0097_9260, 0.01);
        let (processed, log) = walk_light(&mut r, 4, 0);
        assert_eq!(processed, 0);
        assert_eq!(calls_to(&log, 0x0045_0f90), vec![vec![0x5001, 1]]);
        assert!(calls_to(&log, 0x00b9_dfc0).is_empty());
        // Count 0xff.
        let (mut r, _) = light_rig();
        r.set(0x004e_6560, &[0xff]);
        let (processed, log) = walk_light(&mut r, 4, 0);
        assert_eq!(processed, 0);
        assert_eq!(calls_to(&log, 0x0045_0f90), vec![vec![0x5001, 1]]);
        // The limit is reached: nothing is recomputed.
        let (mut r, _) = light_rig();
        let (processed, log) = walk_light(&mut r, 2, 2);
        assert_eq!(processed, 2);
        assert!(calls_to(&log, 0x00b9_d150).is_empty());
        assert_eq!(calls_to(&log, 0x0045_0f90), vec![vec![0x5001, 1]]);
        // Geometry flagged by `00456610`.
        let (mut r, _) = light_rig();
        r.set(0x0045_6610, &[1]);
        let (_, log) = walk_light(&mut r, 4, 0);
        assert!(calls_to(&log, 0x00b9_d150).is_empty());
    }

    #[test]
    fn a_light_with_a_dirty_state_is_reset_before_the_decision() {
        // `004e6580` is false and the geometry is flagged: the source flag
        // decides between the two resets.
        let (mut r, _) = light_rig();
        r.set(0x004e_6580, &[0]);
        r.set(0x0045_6610, &[1, 1, 1, 1]);
        let (_, log) = walk_light(&mut r, 4, 0);
        assert_eq!(calls_to(&log, 0x00b9_bb10), vec![vec![0x7100, 0, 1]]);
        // The source is not flagged: below the limit the geometry is
        // cleared first.
        let (mut r, _) = light_rig();
        r.set(0x004e_6580, &[0]);
        r.set(0x0045_6610, &[1, 0]);
        let (_, log) = walk_light(&mut r, 4, 0);
        assert_eq!(
            only(&log, &[0x0045_0f90, 0x00b9_bb10]),
            vec![0x0045_0f90, 0x00b9_bb10]
        );
        assert_eq!(calls_to(&log, 0x00b9_bb10), vec![vec![0x7100, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0045_0f90)[0], vec![0x5001, 0]);
        // At the limit the geometry is left alone by the reset.
        let (mut r, _) = light_rig();
        r.set(0x004e_6580, &[0]);
        r.set(0x0045_6610, &[1, 0, 1]);
        let (_, log) = walk_light(&mut r, 2, 2);
        assert_eq!(calls_to(&log, 0x00b9_bb10), vec![vec![0x7100, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0045_0f90), vec![vec![0x5001, 1]]);
    }

    #[test]
    fn setting_byte_getter_and_member_address() {
        let mut r = Rig::new();
        r.settings(SETTING_BYTE_PTR, &[(0x011c_74d4, 3)]);
        assert_eq!(r.e.call(0x0087_1a10, &[]).u8(), 3);
        assert_eq!(r.e.call(0x0087_1a30, &args![0x1000u32]).u32(), 0x1088);
    }

    #[test]
    fn offscreen_update_does_nothing_outside_a_rendered_menu() {
        let mut r = Rig::new();
        let (_, log) = r.run(0x0087_1a50, &args![0u32]);
        assert_eq!(addrs(&log), vec![IS_IN_MENU_MODE, IS_IN_MENU_MODE]);
        // In menu mode the last-minute update runs once.
        r.set(IS_IN_MENU_MODE, &[1]);
        let (_, log) = r.run(0x0087_1a50, &args![0u32]);
        assert_eq!(only(&log, &[LAST_MINUTE_UPDATE]), vec![LAST_MINUTE_UPDATE]);
        // A setting is on but no rendered menu is showing.
        r.settings(SETTING_BYTE_PTR, &[(0x011d_b2cc, 1)]);
        let (_, log) = r.run(0x0087_1a50, &args![0u32]);
        assert_eq!(
            addrs(&log),
            vec![
                IS_IN_MENU_MODE,
                SETTING_BYTE_PTR,
                IS_IN_RENDERED_MENU,
                IS_IN_MENU_MODE,
                LAST_MINUTE_UPDATE
            ]
        );
        // The second setting alone is enough.
        r.settings(SETTING_BYTE_PTR, &[(0x011d_8ba0, 1)]);
        let (_, log) = r.run(0x0087_1a50, &args![0u32]);
        assert!(addrs(&log).contains(&IS_IN_RENDERED_MENU));
    }

    #[test]
    fn offscreen_update_creates_the_target_and_refreshes_the_rendered_menu() {
        let mut r = Rig::new();
        r.set(IS_IN_MENU_MODE, &[1]);
        r.set(IS_IN_RENDERED_MENU, &[1]);
        r.settings(SETTING_BYTE_PTR, &[(0x011d_b2cc, 1)]);
        r.set(GET_GLOBAL_011F4748, &[0x1111]);
        r.set(GET_GLOBAL_011F91A8, &[0x2222]);
        r.set(0x00b6_e110, &[0x5555]);
        r.set(GET_PIPBOY, &[0x6666]);
        r.set(IS_CURRENT_RENDERED_MENU_TOPMOST, &[1]);
        let menu = r.object_with_slots(&[(0x0c, 0)]);
        r.set(GET_CURRENT_RENDERED_MENU, &[menu]);
        let refresh = slot_target(&r, menu, 0x0c);
        let (_, log) = r.run(0x0087_1a50, &args![0u32]);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR);
        assert_eq!(guard.len(), 1);
        assert_eq!(guard[0][1..], [0xd, 1, SOURCE_FILE_NAME, 0x1ade]);
        assert_eq!(
            calls_to(&log, 0x00b6_e110),
            vec![vec![0x2222, 0x1111, 0x2d, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0066_b0d0),
            vec![vec![OFFSCREEN_TARGET_POINTER, 0x5555]]
        );
        assert_eq!(calls_to(&log, SET_SCREEN_TEXTURE), vec![vec![0x6666, 0]]);
        // The pointer read after the creation still comes back empty here.
        assert_eq!(calls_to(&log, refresh), vec![vec![menu, 0, 1, 0]]);
        assert_eq!(calls_to(&log, PROFILE_SCOPE_DTOR).len(), 1);
        assert_eq!(*addrs(&log).last().unwrap(), PROFILE_SCOPE_DTOR);
    }

    #[test]
    fn offscreen_update_flags_the_pipboy_reference_around_the_refresh() {
        let mut r = Rig::new();
        r.set(IS_IN_MENU_MODE, &[1]);
        r.set(IS_IN_RENDERED_MENU, &[1]);
        r.set(IS_IN_PIPBOY_MENU, &[1]);
        r.set(IS_MENU_ID_VISIBLE, &[1]);
        r.settings(SETTING_BYTE_PTR, &[(0x011d_b2cc, 1)]);
        r.pointers(&[(OFFSCREEN_TARGET_POINTER, 0x5555)]);
        r.set(GET_MENU_BY_CLASS, &[0x7000]);
        r.set(0x0056_c7f0, &[0x7100]);
        r.set(0x0045_6610, &[0]);
        let menu = r.object_with_slots(&[(0x0c, 0)]);
        r.set(GET_CURRENT_RENDERED_MENU, &[menu]);
        let refresh = slot_target(&r, menu, 0x0c);
        let (_, log) = r.run(0x0087_1a50, &args![0u32]);
        // No target creation, and no pipboy screen texture in the pipboy.
        assert!(!addrs(&log).contains(&0x00b6_e110));
        assert!(!addrs(&log).contains(&SET_SCREEN_TEXTURE));
        assert_eq!(calls_to(&log, 0x0056_c7f0), vec![vec![0x7000]]);
        assert_eq!(
            only(&log, &[0x0045_0f90, refresh]),
            vec![0x0045_0f90, refresh, 0x0045_0f90]
        );
        assert_eq!(
            calls_to(&log, 0x0045_0f90),
            vec![vec![0x7100, 1], vec![0x7100, 0]]
        );
        assert_eq!(calls_to(&log, refresh), vec![vec![menu, 0x5555, 1, 0]]);
    }

    #[test]
    fn offscreen_update_needs_a_topmost_menu_or_one_of_the_three_menus() {
        for (menu_class, expected) in [(0x3e9u32, true), (0x3f8, true), (0x423, true), (0, false)] {
            let mut r = Rig::new();
            r.set(IS_IN_MENU_MODE, &[1]);
            r.set(IS_IN_RENDERED_MENU, &[1]);
            r.settings(SETTING_BYTE_PTR, &[(0x011d_b2cc, 1)]);
            r.pointers(&[(OFFSCREEN_TARGET_POINTER, 0x5555)]);
            let menu = r.object_with_slots(&[(0x0c, 0)]);
            r.set(GET_CURRENT_RENDERED_MENU, &[menu]);
            let refresh = slot_target(&r, menu, 0x0c);
            let class = menu_class;
            r.e.register_double(GET_MENU_BY_CLASS, move |_, a| Ret {
                eax: (a[0] == class) as u32,
                ..Ret::default()
            });
            let (_, log) = r.run(0x0087_1a50, &args![0u32]);
            assert_eq!(
                calls_to(&log, refresh).len(),
                expected as usize,
                "{menu_class:x}"
            );
        }
    }

    #[test]
    fn set_active_toggles_the_timer_and_clears_the_controls() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.set(CONTROLS_GET_INSTANCE, &[0x7777]);
        let (_, log) = r.run(0x0087_1c90, &args![this, 1u32]);
        assert_eq!(
            addrs(&log),
            vec![
                SETTING_BYTE_PTR,
                CONTROLS_GET_INSTANCE,
                CONTROLS_GET_INSTANCE,
                CONTROLS_CLEAR_KEYSTROKES,
                CONTROLS_GET_INSTANCE,
                CONTROLS_POLL,
                CONTROLS_GET_INSTANCE,
                CONTROLS_CLEAR_USER_ACTIONS,
                TIMER_ENABLE
            ]
        );
        assert_eq!(calls_to(&log, CONTROLS_POLL), vec![vec![0x7777]]);
        assert_eq!(calls_to(&log, TIMER_ENABLE), vec![vec![0x011f_6394]]);
        assert_eq!(r.e.mem.u8(this + 3), 1);
        // Inactive, and no controls object.
        r.set(CONTROLS_GET_INSTANCE, &[0]);
        let (_, log) = r.run(0x0087_1c90, &args![this, 0u32]);
        assert_eq!(
            addrs(&log),
            vec![SETTING_BYTE_PTR, CONTROLS_GET_INSTANCE, TIMER_DISABLE]
        );
        assert_eq!(r.e.mem.u8(this + 3), 0);
    }

    #[test]
    fn the_always_active_setting_forces_the_flag() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.settings(SETTING_BYTE_PTR, &[(0x011d_eed8, 1)]);
        let (_, log) = r.run(0x0087_1c90, &args![this, 0u32]);
        assert_eq!(addrs(&log).last(), Some(&TIMER_ENABLE));
        assert_eq!(r.e.mem.u8(this + 3), 1);
    }

    #[test]
    fn pointer_forwarding_needs_the_active_window_and_menu_mode() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.e.mem.set_u32(this + 8, 0x1234);
        r.set(IMPORT_GET_ACTIVE_WINDOW, &[0x9999]);
        let (ret, log) = r.run(0x0087_1d10, &args![this, 10i32, 20i32]);
        assert!(!ret.bool());
        assert_eq!(addrs(&log), vec![IMPORT_GET_ACTIVE_WINDOW]);
        r.set(IMPORT_GET_ACTIVE_WINDOW, &[0x1234]);
        let (ret, log) = r.run(0x0087_1d10, &args![this, 10i32, 20i32]);
        assert!(!ret.bool());
        assert_eq!(addrs(&log), vec![IMPORT_GET_ACTIVE_WINDOW, IS_IN_MENU_MODE]);
        r.set(IS_IN_MENU_MODE, &[1]);
        let (ret, log) = r.run(0x0087_1d10, &args![this, 10i32, 20i32]);
        assert!(!ret.bool());
        assert_eq!(
            addrs(&log),
            vec![
                IMPORT_GET_ACTIVE_WINDOW,
                IS_IN_MENU_MODE,
                CONTROLS_GET_INSTANCE,
                0x0071_1e00
            ]
        );
    }

    #[test]
    fn pointer_forwarding_divides_by_the_display_size() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.e.mem.set_u32(this + 8, 0x1234);
        r.set(IMPORT_GET_ACTIVE_WINDOW, &[0x1234]);
        r.set(IS_IN_MENU_MODE, &[1]);
        r.set(CONTROLS_GET_INSTANCE, &[0x7777]);
        r.set(0x0071_1e00, &[1]);
        r.set(0x004d_c1f0, &[1000]);
        r.set(0x004d_c200, &[500]);
        let (ret, log) = r.run(0x0087_1d10, &args![this, 250i32, 100i32]);
        assert!(ret.bool());
        assert_eq!(
            only(&log, &[0x004d_c200, 0x004d_c1f0, 0x0070_3080, 0x0070_6230]),
            vec![0x004d_c200, 0x004d_c1f0, 0x0070_3080]
        );
        assert_eq!(
            calls_to(&log, 0x0070_3080),
            vec![vec![word(0.25), word(0.2)]]
        );
        assert_eq!(calls_to(&log, IS_TOP_MENU_ID), vec![vec![0x41e]]);
        // With menu 0x41e on top the position goes through unchanged.
        r.set(IS_TOP_MENU_ID, &[1]);
        let (ret, log) = r.run(0x0087_1d10, &args![this, -5i32, 100i32]);
        assert!(ret.bool());
        assert_eq!(calls_to(&log, 0x0070_6230), vec![vec![(-5i32) as u32, 100]]);
        assert!(!addrs(&log).contains(&0x0070_3080));
    }

    #[test]
    fn target_sizes_come_from_the_pointed_to_object() {
        let mut r = Rig::new();
        let this = r.object(0x80);
        let target = r.object_with_slots(&[(0x94, 640), (0x98, 480)]);
        r.pointers(&[(this + 0x30, target), (this + 0x38, target)]);
        assert_eq!(r.e.call(0x0087_2370, &args![this, 0u32]).u32(), 640);
        assert_eq!(r.e.call(0x0087_23d0, &args![this, 0u32]).u32(), 480);
        assert_eq!(r.e.call(0x0087_2370, &args![this, 2u32]).u32(), 640);
        assert_eq!(r.e.call(0x0087_23d0, &args![this, 2u32]).u32(), 480);
        // An empty slot gives 0.
        assert_eq!(r.e.call(0x0087_2370, &args![this, 1u32]).u32(), 0);
        assert_eq!(r.e.call(0x0087_23d0, &args![this, 1u32]).u32(), 0);
    }

    /// `log` (without the function's own entry) as the frame model's callees.
    fn model_log(log: &Log) -> Vec<world::frame::interface::Callee> {
        addrs(log)
            .into_iter()
            .map(world::frame::interface::Callee::Direct)
            .collect()
    }

    fn check_model(
        address: u32,
        s: &world::frame::interface::InterfaceState,
        log: &Log,
        direct: &[u32],
    ) {
        use world::frame::interface::{follows, function};
        let model = function(address).expect("modelled");
        let log = model_log(log);
        // Same-unit Rust calls do not pass through Engine::call. Check their
        // observable work below instead of expecting a dispatch-log entry.
        let direct: Vec<_> = direct
            .iter()
            .copied()
            .map(world::frame::interface::Callee::Direct)
            .collect();
        follows(model, s, &log, &direct).unwrap_or_else(|why| panic!("{address:08x} {s:?}: {why}"));
    }

    /// `Main::PostSwapProcess` (`008705d0`) and `Main::OnIdle_PostThreadsProcess`
    /// (`00870610`) follow `world::frame::interface`'s model for one and more threads
    /// (docs/FRAME_SKELETON.md, PR 7).
    #[test]
    fn post_swap_and_post_threads_follow_the_frame_model() {
        use world::frame::interface::InterfaceState;
        for threads in [1u32, 2] {
            let s = InterfaceState {
                threads: threads as i32,
                ..InterfaceState::default()
            };
            let mut r = Rig::new();
            let this = r.object(0x10);
            r.setting(SETTING_DWORD_PTR, &[threads]);
            let (_, log) = r.run(0x008705d0, &args![this]);
            check_model(0x0087_05d0, &s, &log, &[]);
            let mut r = Rig::new();
            let this = r.object(0x10);
            let interface = r.object(0x600);
            r.e.mem.set_u8(interface + 0x4cc, 1);
            r.set(GET_GLOBAL_011D8A80, &[interface]);
            r.setting(SETTING_DWORD_PTR, &[threads]);
            let (_, log) = r.run(0x00870610, &args![this]);
            check_model(0x0087_0610, &s, &log, &[0x0087_0680]);
            assert_eq!(r.e.mem.u8(interface + 0x4cc), 0);
            assert_eq!(calls_to(&log, 0x0070_6320), vec![vec![0]]);
            assert_eq!(
                only(&log, &[0x0070_94f0, 0x0070_6320, 0x00a0_e770]),
                vec![0x0070_94f0, 0x0070_6320, 0x00a0_e770]
            );
        }
    }

    /// `Main::RenderMenuBackground` (`00871dc0`): nothing without the player's cell and
    /// 3D; else the world drawn once under the menus' modifier.
    #[test]
    fn menu_background_follows_the_frame_model() {
        use world::frame::interface::InterfaceState;
        let hidden = InterfaceState {
            player_shown: false,
            ..InterfaceState::default()
        };
        let mut r = Rig::new();
        let this = r.object(0x200);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        check_model(0x0087_1dc0, &hidden, &log, &[]);
        let (mut r, this, _, _) = menu_background_rig((800, 300));
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        check_model(
            0x0087_1dc0,
            &InterfaceState::default(),
            &log,
            &[0x0087_06b0],
        );
        // The direct draw helper copies the root-child position into the
        // indexed scene object before drawing; that observable work proves
        // it ran, without manufacturing a missing call-log entry.
        let indexed = r.e.call(INDEXED_GLOBAL, &args![0u32]).u32();
        assert_eq!(
            [
                r.e.mem.u32(indexed + 0x1f0),
                r.e.mem.u32(indexed + 0x1f4),
                r.e.mem.u32(indexed + 0x1f8)
            ],
            [10, 20, 30],
        );
        assert_eq!(
            only(&log, &[0x0052_9c90, 0x0087_4ac0]),
            vec![0x0052_9c90, 0x0087_4ac0]
        );
    }

    /// The rig of the menu-background render: a player with a vtable, a
    /// renderer whose group answers `800 x 300`, and a menu target whose
    /// first buffer answers `target_size`.
    fn menu_background_rig(target_size: (u32, u32)) -> (Rig, u32, u32, u32) {
        let (mut r, this, _) = frame_rig(0);
        let player = r.object_with_slots(&[(0x1d0, 1)]);
        r.e.set_global(PLAYER_OBJECT, player);
        r.set(0x008d_6f30, &[1]);
        let group = r.object_with_slots(&[(0x8c, 800), (0x90, 300)]);
        r.object_at(0x1000, &[(0xc8, group), (0xac, 0)]);
        let buffer = r.object_with_slots(&[(0x94, target_size.0), (0x98, target_size.1)]);
        let target = r.object(0x40);
        r.pointers(&[(MENU_TARGET_POINTER, target), (target + 0x30, buffer)]);
        r.set(GET_GLOBAL_011F91A8, &[0x2222]);
        r.set(0x00b6_e110, &[0x5555]);
        (r, this, target, group)
    }

    #[test]
    fn menu_background_render_needs_a_player_with_a_process() {
        let mut r = Rig::new();
        let this = r.object(0x200);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert_eq!(
            addrs(&log),
            vec![PROFILE_SCOPE_CTOR, 0x008d_6f30, PROFILE_SCOPE_DTOR]
        );
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR)[0][1..],
            [0xd, 1, SOURCE_FILE_NAME, 0x1c19]
        );
        // The player's vtable slot answers 0.
        let player = r.object_with_slots(&[(0x1d0, 0)]);
        r.e.set_global(PLAYER_OBJECT, player);
        r.set(0x008d_6f30, &[1]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert_eq!(addrs(&log).len(), 4);
        assert_eq!(r.e.mem.u8(this + 0x9c), 0);
    }

    #[test]
    fn menu_background_render_recreates_the_target_and_keeps_a_matching_size() {
        let (mut r, this, target, _) = menu_background_rig((800, 300));
        r.e.set_global(0x011a_d884, 5u8);
        r.set(GET_MT_SYSTEM, &[0x0120_0088]);
        r.set(GET_CURRENT_RENDERED_MENU, &[0x7a00]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert_eq!(calls_to(&log, TILE_LOCK).len(), 1);
        assert_eq!(calls_to(&log, MT_BEGIN_FRAME), vec![vec![0x0120_0088]]);
        assert_eq!(calls_to(&log, 0x00b6_da10), vec![vec![0x2222, target]]);
        assert_eq!(
            calls_to(&log, 0x00b6_e110),
            vec![vec![0x2222, 0x1000, 6, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0066_b0d0),
            vec![vec![MENU_TARGET_POINTER, 0x5555]]
        );
        assert_eq!(
            only(
                &log,
                &[
                    TILE_LOCK,
                    MT_BEGIN_FRAME,
                    0x00b6_da10,
                    0x00b6_e110,
                    0x0066_b0d0
                ]
            ),
            vec![
                TILE_LOCK,
                MT_BEGIN_FRAME,
                0x00b6_da10,
                0x00b6_e110,
                0x0066_b0d0
            ]
        );
        // The size matches and the flag byte is clear: no ratio color.
        let colors = calls_to(&log, SET_COLOR);
        assert_eq!(
            colors.iter().filter(|c| c[1] == 0x011a_d840).count(),
            1,
            "only the frame's own"
        );
        assert_eq!(calls_to(&log, FLOAT_HELPER_B).len(), 0);
        // The state is restored and the end of the frame is marked.
        assert_eq!(r.e.mem.u8(this + 0x9c), 0);
        assert_eq!(r.e.global::<u8>(0x011a_d884), 5);
        assert_eq!(r.e.global::<u8>(0x011d_ea29), 1);
        assert!(stages(&log, MT_SET_THREAD_STAGE).contains(&0x17));
        assert_eq!(calls_to(&log, MT_END_FRAME), vec![vec![0x0120_0088]]);
        assert_eq!(calls_to(&log, TILE_UNLOCK).len(), 1);
        assert_eq!(*addrs(&log).iter().rev().nth(1).unwrap(), 0x004a_03c0);
        assert_eq!(calls_to(&log, 0x004a_03c0), vec![vec![0x1000]]);
    }

    #[test]
    fn menu_background_render_keeps_the_old_target_released_only_when_there_is_one() {
        let (mut r, this, _, _) = menu_background_rig((800, 300));
        r.pointers(&[]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert!(calls_to(&log, 0x00b6_da10).is_empty());
    }

    #[test]
    fn menu_background_render_sets_the_size_ratios_when_the_target_differs() {
        // Width differs (800 against 400) and height differs (300 against 600).
        let (mut r, this, _, _) = menu_background_rig((400, 600));
        let calls = Rc::new(RefCell::new(0));
        {
            let calls = calls.clone();
            r.e.register_double(FLOAT_HELPER_B, move |_, _| {
                let mut calls = calls.borrow_mut();
                *calls += 1;
                Ret {
                    st0: if *calls == 1 { 1.5 } else { 0.75 },
                    ..Ret::default()
                }
            });
        }
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert_eq!(
            calls_to(&log, FLOAT_HELPER_B),
            vec![vec![word(2.0), word(1.0)], vec![word(0.5), word(1.0)]]
        );
        let first_color = calls_to(&log, COLOR_CTOR)[0].clone();
        assert_eq!(first_color[1..], [0, word(1.5), word(0.75), 0]);
        let set = calls_to(&log, SET_COLOR)[0].clone();
        assert_eq!(set, vec![0x4000, first_color[0]]);
    }

    #[test]
    fn menu_background_render_height_alone_also_counts() {
        let (mut r, this, _, _) = menu_background_rig((800, 100));
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert_eq!(calls_to(&log, FLOAT_HELPER_B).len(), 2);
    }

    #[test]
    fn menu_background_render_resets_the_clear_value_from_the_flag_byte() {
        let (mut r, this, _, _) = menu_background_rig((800, 300));
        r.e.set_global(0x011f_9426, 1u8);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        let own = calls_to(&log, SET_COLOR)
            .iter()
            .filter(|c| c[1] == 0x011a_d840)
            .count();
        assert_eq!(own, 2);
    }

    #[test]
    fn menu_background_render_fades_the_image_space_around_the_frame() {
        let (mut r, this, target, _) = menu_background_rig((800, 300));
        r.set(GET_GLOBAL_011D8A80, &[0x4440]);
        r.set(0x0071_8ab0, &[0x6660]);
        r.set(0x005d_2860, &[0x7770]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert_eq!(calls_to(&log, 0x0071_8ab0), vec![vec![0x4440]]);
        assert_eq!(
            calls_to(&log, 0x0052_9c90),
            vec![vec![0x7770], vec![0x6660]]
        );
        assert_eq!(
            calls_to(&log, 0x0052_99a0),
            vec![vec![0x6660, word(1.0), 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0086_fd90),
            vec![vec![this, 0], vec![this, 0]]
        );
        // The frame goes to the menu target: `00870bd0` is the plain frame.
        assert!(addrs(&log).contains(&0x0087_4ac0));
        let _ = target;
    }

    #[test]
    fn menu_background_render_skips_the_world_in_the_pipboy_unless_menu_3f5_shows() {
        // In the pipboy: the frame is asked to skip the world.
        let (mut r, this, _, _) = menu_background_rig((800, 300));
        r.set(IS_IN_PIPBOY_MENU, &[1]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert!(addrs(&log).contains(&0x0087_4ac0));
        assert!(!addrs(&log).contains(&0x0087_5110));
        // With menu 0x3f5 visible the flag is dropped.
        let (mut r, this, _, _) = menu_background_rig((800, 300));
        r.set(IS_IN_PIPBOY_MENU, &[1]);
        r.set(IS_MENU_ID_VISIBLE, &[1, 0]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert!(addrs(&log).contains(&0x0087_5110));
    }

    #[test]
    fn menu_background_render_accumulates_the_rendered_menu_scene() {
        let (mut r, this, _, _) = menu_background_rig((800, 300));
        r.set(IS_MENU_ID_VISIBLE, &[0, 1]);
        r.set(0x004a_4040, &[1]);
        r.set(GET_CURRENT_RENDERED_MENU, &[0x7a00]);
        r.set(0x008b_6200, &[0x7a10]);
        r.e.register_double(0x0055_85e0, |_, a| Ret {
            eax: if a[0] == 0x7a00 { 0x7b00 } else { 0 },
            ..Ret::default()
        });
        r.pointers(&[(MENU_TARGET_POINTER, 0), (this + MAIN_MENU_ACCUM, 0x9a00)]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        let object = calls_to(&log, 0x004a_0eb0)[0].clone();
        let at = object[0];
        assert_eq!(object[1], 0);
        assert_eq!(calls_to(&log, 0x0041_fd00), vec![vec![at, 0x7a10]]);
        assert_eq!(
            calls_to(&log, 0x004a_0fd0),
            vec![vec![at, 0x9a00], vec![at, 0]]
        );
        assert_eq!(calls_to(&log, 0x004a_1020).len(), 1);
        assert_eq!(calls_to(&log, 0x004a_1020)[0][0], 0x9a00);
        assert_eq!(calls_to(&log, 0x00b6_bee0), vec![vec![0x7a10, 0x7b00, at]]);
        assert_eq!(calls_to(&log, 0x00b6_c0d0), vec![vec![0x7a10, 0x9a00, 0]]);
        assert_eq!(calls_to(&log, 0x004a_0f60), vec![vec![at]]);
        // Not when `004a4040` refuses.
        r.set(0x004a_4040, &[0]);
        let (_, log) = r.run(0x0087_1dc0, &args![this]);
        assert!(!addrs(&log).contains(&0x004a_0eb0));
    }

    /// A rig for the list-file reader: the lines the file yields, the names
    /// the data handler knows.
    fn list_file_rig(lines: &[&str], known: &[&str]) -> (Rig, Rc<RefCell<Vec<String>>>) {
        let mut r = Rig::new();
        let lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        let next = Rc::new(RefCell::new(0usize));
        r.e.register_double(0x00ec_9a47, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        r.e.register_double(0x00ec_c104, move |e, a| {
            let mut next = next.borrow_mut();
            if *next >= lines.len() {
                return Ret::default();
            }
            e.mem.set_cstr(a[0], lines[*next].as_bytes());
            *next += 1;
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        r.e.register_double(0x0044_a670, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        let asked: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
        {
            let asked = asked.clone();
            let known: Vec<String> = known.iter().map(|k| k.to_string()).collect();
            r.e.register_double(0x0046_2f40, move |e, a| {
                let name = String::from_utf8(e.mem.cstr(a[1])).unwrap();
                asked.borrow_mut().push(name.clone());
                let index = known.iter().position(|k| *k == name);
                Ret {
                    eax: index.map_or(0, |i| 0x9000 + i as u32),
                    ..Ret::default()
                }
            });
        }
        (r, asked)
    }

    #[test]
    fn list_file_lines_activate_the_known_entries() {
        let (mut r, asked) = list_file_rig(
            &["# comment\n", "a.esp\n", "\n", "b.esp", "missing.esp\n"],
            &["a.esp", "b.esp"],
        );
        r.e.set_global(0x011c_3f2c, 0x4242u32);
        let this = r.object(0x10);
        let (ret, log) = r.run(0x0087_2430, &args![this, 0x1111u32, 0x2222u32]);
        assert_eq!(ret.u8(), 1);
        assert_eq!(*asked.borrow(), vec!["a.esp", "b.esp", "missing.esp"]);
        assert_eq!(
            calls_to(&log, 0x0047_1d00),
            vec![vec![0x9000, 1], vec![0x9001, 1]]
        );
        let first = calls_to(&log, 0x0040_6d30)[0].clone();
        assert_eq!(first[1..], [0x104, 0x1111]);
        let second = calls_to(&log, 0x0040_6d50)[0].clone();
        assert_eq!(second, vec![first[0], 0x104, 0x2222]);
        assert_eq!(
            calls_to(&log, 0x00ec_9a47),
            vec![vec![first[0], 0x0108_2cb8]]
        );
        assert_eq!(calls_to(&log, 0x00ec_9907), vec![vec![0x77]]);
        assert_eq!(calls_to(&log, 0x0046_2f40)[0][0], 0x4242);
        // Every line is read into the same 512-byte buffer from the file.
        assert_eq!(calls_to(&log, 0x00ec_c104)[0][1..], [0x200, 0x77]);
    }

    #[test]
    fn list_file_without_hits_or_without_a_file_returns_zero() {
        let (mut r, _) = list_file_rig(&["x.esp\n"], &[]);
        let this = r.object(0x10);
        let (ret, log) = r.run(0x0087_2430, &args![this, 1u32, 2u32]);
        assert_eq!(ret.u8(), 0);
        assert!(calls_to(&log, 0x0047_1d00).is_empty());
        let (mut r, _) = list_file_rig(&[], &[]);
        r.e.register_double(0x00ec_9a47, |_, _| Ret::default());
        let (ret, log) = r.run(0x0087_2430, &args![this, 1u32, 2u32]);
        assert_eq!(ret.u8(), 0);
        assert!(!addrs(&log).contains(&0x00ec_c104));
        assert!(!addrs(&log).contains(&0x00ec_9907));
    }

    #[test]
    fn list_file_skips_comments_and_one_character_lines() {
        let (mut r, asked) = list_file_rig(&["#a.esp\n", "x", "\n"], &["a.esp", "x"]);
        let this = r.object(0x10);
        let (ret, _) = r.run(0x0087_2430, &args![this, 1u32, 2u32]);
        assert_eq!(ret.u8(), 0);
        assert!(asked.borrow().is_empty());
    }

    #[test]
    fn housekeeping_without_release_clears_adaptation_and_wakes_the_water() {
        let mut r = Rig::new();
        r.e.set_global(0x011f_941e, 1u8);
        r.set(0x0070_ec90, &[0x6600]);
        r.set(0x005d_2a40, &[1]);
        let (ret, log) = r.run(0x0087_2570, &args![0u32]);
        assert_eq!(ret.u8(), 1);
        assert_eq!(
            addrs(&log),
            vec![
                0x00ba_d4a0,
                0x0070_ec90,
                0x005d_2a40,
                0x0070_ec90,
                0x004e_6620,
                0x0070_ec90,
                0x004e_65d0
            ]
        );
        assert_eq!(calls_to(&log, 0x004e_6620), vec![vec![0x6600, 1, 0]]);
        assert_eq!(calls_to(&log, 0x004e_65d0), vec![vec![0x6600]]);
        // No adaptation flag and nothing from `005d2a40`.
        r.e.set_global(0x011f_941e, 0u8);
        r.set(0x005d_2a40, &[0]);
        let (ret, log) = r.run(0x0087_2570, &args![0u32]);
        assert_eq!(ret.u8(), 1);
        assert_eq!(addrs(&log), vec![0x0070_ec90, 0x005d_2a40]);
        // No object at all.
        r.set(0x0070_ec90, &[0]);
        let (_, log) = r.run(0x0087_2570, &args![0u32]);
        assert_eq!(addrs(&log), vec![0x0070_ec90]);
    }

    /// The rig of the release housekeeping: a movie player at `0x0126fac4`,
    /// the `Main` object, a 2 x 2 cell grid.
    fn release_rig() -> (Rig, u32, u32) {
        let mut r = Rig::new();
        let player = r.object(0x40);
        let inner = r.object(0x40);
        r.e.mem.set_u32(player + 0xc, inner);
        r.e.set_global(0x0126_fac4, player);
        let main_object = r.object(0x40);
        r.e.mem.set_u32(main_object + 8, 0x1234);
        r.e.set_global(MAIN_OBJECT, main_object);
        r.set(0x0086_bdf0, &[1]);
        r.set(0x0044_ddc0, &[0x6000]);
        (r, player, inner)
    }

    #[test]
    fn release_housekeeping_waits_for_the_loading_thread_and_stops_the_movie() {
        let (mut r, player, inner) = release_rig();
        r.set(0x0086_bdf0, &[0, 0, 1]);
        r.set(0x00ec_17c0, &[1]);
        r.e.mem.set_u32(inner + 0x20, 1);
        r.set(0x00b4_f5c0, &[0x6600]);
        let (ret, log) = r.run(0x0087_2570, &args![1u32]);
        assert_eq!(ret.u8(), 1);
        assert_eq!(calls_to(&log, 0x0040_fca0), vec![vec![5], vec![5]]);
        assert_eq!(calls_to(&log, 0x0086_bdf0).len(), 3);
        assert_eq!(
            only(&log, &[0x00ec_17c0, 0x00ec_29b0, 0x00ec_1800, 0x00ec_25e0]),
            vec![0x00ec_17c0, 0x00ec_29b0, 0x00ec_1800, 0x00ec_25e0]
        );
        for addr in [0x00ec_17c0, 0x00ec_29b0, 0x00ec_1800, 0x00ec_25e0] {
            assert_eq!(calls_to(&log, addr), vec![vec![player]]);
        }
        assert_eq!(calls_to(&log, 0x00b6_31d0), vec![vec![0x6600]]);
        assert_eq!(calls_to(&log, 0x0087_7430).len(), 1);
    }

    #[test]
    fn release_housekeeping_leaves_the_movie_alone_when_idle() {
        let (mut r, _, _) = release_rig();
        let (_, log) = r.run(0x0087_2570, &args![1u32]);
        assert!(!addrs(&log).contains(&0x00ec_29b0));
        assert!(!addrs(&log).contains(&0x00ec_25e0));
        // No accumulator either.
        assert!(!addrs(&log).contains(&0x00b6_31d0));
    }

    #[test]
    fn release_housekeeping_clears_the_cached_reference_of_each_cell() {
        let (mut r, _, _) = release_rig();
        r.e.set_global(WORLD_OBJECT, 0x6100u32);
        // A 2 x 2 grid of slots holding cells 0x7000, none, 0x7002, 0x7003.
        let slots = r.object(0x20);
        for (i, cell) in [0x7000u32, 0, 0x7002, 0x7003].iter().enumerate() {
            r.e.mem.set_u32(slots + 4 * i as u32, *cell);
        }
        r.set(0x0084_e3a0, &[2]);
        r.e.register_double(0x0045_7050, move |_, a| Ret {
            eax: slots + 4 * (a[1] * 2 + a[2]),
            ..Ret::default()
        });
        // The cells' second out values: 0x8000 (with data), 0x8010 (without).
        let with_data = r.object(0x20);
        r.e.mem.set_u32(with_data + 4, 0x1234);
        let without_data = r.object(0x20);
        let untouched = r.object(0x20);
        r.e.mem.set_u32(untouched + 4, 0x55);
        r.e.register_double(0x0054_77f0, move |e, a| {
            let (first, second) = match a[0] {
                0x7000 => (1, with_data),
                0x7002 => (0, untouched),
                _ => (1, without_data),
            };
            e.mem.set_u32(a[1], first);
            e.mem.set_u32(a[2], second);
            Ret::default()
        });
        let (_, log) = r.run(0x0087_2570, &args![1u32]);
        let asked: Vec<u32> = calls_to(&log, 0x0054_77f0).iter().map(|c| c[0]).collect();
        assert_eq!(asked, vec![0x7000, 0x7002, 0x7003]);
        assert_eq!(
            calls_to(&log, 0x0045_7050),
            vec![
                vec![0x6100, 0, 0],
                vec![0x6100, 0, 1],
                vec![0x6100, 1, 0],
                vec![0x6100, 1, 1]
            ]
        );
        assert_eq!(r.e.mem.u32(with_data + 4), 0);
        assert_eq!(r.e.mem.u32(untouched + 4), 0x55);
    }

    #[test]
    fn release_housekeeping_resizes_the_window_to_the_display() {
        let (mut r, _, _) = release_rig();
        r.set(0x0044_6e10, &[1]);
        r.set(0x004d_c1f0, &[1280]);
        r.set(0x004d_c200, &[720]);
        r.set(IMPORT_GET_WINDOW_LONG, &[0x16cf_0000]);
        let seen: Rc<RefCell<Vec<[i32; 4]>>> = Rc::new(RefCell::new(vec![]));
        {
            let seen = seen.clone();
            r.e.register_double(IMPORT_ADJUST_WINDOW_RECT_EX, move |e, a| {
                seen.borrow_mut().push([
                    e.mem.i32(a[0]),
                    e.mem.i32(a[0] + 4),
                    e.mem.i32(a[0] + 8),
                    e.mem.i32(a[0] + 12),
                ]);
                // The rectangle grows by the frame.
                e.mem.set_i32(a[0], -8);
                e.mem.set_i32(a[0] + 4, -31);
                e.mem.set_i32(a[0] + 8, 1288);
                e.mem.set_i32(a[0] + 12, 727);
                Ret::default()
            });
        }
        let (_, log) = r.run(0x0087_2570, &args![1u32]);
        assert_eq!(*seen.borrow(), vec![[0, 0, 1280, 720]]);
        assert_eq!(
            calls_to(&log, IMPORT_GET_WINDOW_LONG),
            vec![vec![0x1234, 0xffff_fff0]]
        );
        let adjust = calls_to(&log, IMPORT_ADJUST_WINDOW_RECT_EX);
        assert_eq!(adjust[0][1..], [0x16cf_0000, 0, 0]);
        // As compiled: x = left, y = bottom, cx = right - left, cy = top - bottom.
        assert_eq!(
            calls_to(&log, IMPORT_SET_WINDOW_POS),
            vec![vec![
                0x1234,
                0,
                (-8i32) as u32,
                727,
                1296,
                (-31i32 - 727) as u32,
                0
            ]]
        );
        // Without the window request nothing is resized.
        r.set(0x0044_6e10, &[0]);
        let (_, log) = r.run(0x0087_2570, &args![1u32]);
        assert!(!addrs(&log).contains(&IMPORT_SET_WINDOW_POS));
    }

    #[test]
    fn current_movie_and_inner_flag_accessors() {
        let mut r = Rig::new();
        let outer = r.object(0x40);
        assert_eq!(r.e.call(0x0087_2780, &args![outer]).u8(), 0);
        let inner = r.object(0x40);
        r.e.mem.set_u32(outer + 0xc, inner);
        assert_eq!(r.e.call(0x0087_2780, &args![outer]).u8(), 0);
        r.e.mem.set_u32(inner + 0x20, 0x1234);
        assert_eq!(r.e.call(0x0087_2780, &args![outer]).u8(), 1);
        assert_eq!(r.e.call(0x0087_27b0, &args![inner]).u8(), 1);
        r.e.mem.set_u32(inner + 0x20, 0);
        assert_eq!(r.e.call(0x0087_27b0, &args![inner]).u8(), 0);
    }

    #[test]
    fn stage_reset_only_advances_when_the_water_setting_is_off() {
        let mut r = Rig::new();
        let cell = r.object(0x40);
        r.e.mem.set_i32(cell, 3);
        r.set(0x0070_ec90, &[cell]);
        let this = r.object(0x10);
        let (_, log) = r.run(0x0087_27d0, &args![this]);
        let expected: Vec<u32> = (4..0x11).collect();
        assert_eq!(stages(&log, MT_SET_THREAD_STAGE), expected);
        assert_eq!(stages(&log, MT_ADVANCE_THREAD_STAGE), expected);
        assert_eq!(
            addrs(&log)[..3],
            [SETTING_BYTE_PTR, 0x0070_ec90, GET_MT_SYSTEM]
        );
    }

    #[test]
    fn stage_reset_draws_with_the_flagged_object_when_the_cell_is_usable() {
        let mut r = Rig::new();
        let cell = r.object(0x40);
        let this = r.object(0x10);
        r.set(0x0070_ec90, &[cell]);
        r.settings(SETTING_BYTE_PTR, &[(0x011c_7adc, 1)]);
        r.e.set_global(0x011a_9608, 0x55u32);
        let indexed = r.object(0x10);
        r.set(INDEXED_GLOBAL, &[indexed]);
        r.set(0x005f_9ad0, &[0]);
        r.set(GET_ROOT_CHILD_OF_ROOT, &[0x4000]);
        let during: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        {
            let during = during.clone();
            r.e.register_double(0x004e_1bc0, move |e, _| {
                during.borrow_mut().push(e.global::<u32>(0x011a_9608));
                Ret::default()
            });
        }
        let (_, log) = r.run(0x0087_27d0, &args![this]);
        assert_eq!(*during.borrow(), vec![1]);
        assert_eq!(r.e.global::<u32>(0x011a_9608), 0x55);
        assert_eq!(calls_to(&log, 0x0054_68f0), vec![vec![0xb]]);
        assert_eq!(calls_to(&log, 0x005f_9ad0), vec![vec![indexed]]);
        assert_eq!(
            calls_to(&log, 0x005f_9af0),
            vec![vec![indexed, 1], vec![indexed, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x004e_1bc0),
            vec![vec![cell, 0x4000, indexed]]
        );
        // The first path does not reset the water state.
        assert!(!addrs(&log).contains(&0x004e_2120));
    }

    #[test]
    fn stage_reset_takes_the_first_path_for_a_world_with_the_check_true() {
        let mut r = Rig::new();
        let cell = r.object(0x40);
        let this = r.object(0x10);
        r.set(0x0070_ec90, &[cell]);
        r.settings(SETTING_BYTE_PTR, &[(0x011c_7adc, 1)]);
        r.set(0x005f_36f0, &[0x6000]);
        r.set(0x0045_18e0, &[1]);
        let indexed = r.object(0x10);
        r.set(INDEXED_GLOBAL, &[indexed]);
        let (_, log) = r.run(0x0087_27d0, &args![this]);
        assert_eq!(calls_to(&log, 0x0045_18e0), vec![vec![0x6000]]);
        assert_eq!(calls_to(&log, 0x004e_1bc0).len(), 1);
        // The check false: the other path.
        r.set(0x0045_18e0, &[0]);
        let (_, log) = r.run(0x0087_27d0, &args![this]);
        assert!(calls_to(&log, 0x004e_1bc0).is_empty());
        assert_eq!(calls_to(&log, 0x004e_2120).len(), 1);
    }

    #[test]
    fn stage_reset_clears_the_water_state_when_the_check_blocks() {
        let mut r = Rig::new();
        let cell = r.object(0x40);
        r.e.mem.set_i32(cell, 15);
        let this = r.object(0x10);
        r.set(0x0070_ec90, &[cell]);
        r.settings(SETTING_BYTE_PTR, &[(0x011c_7adc, 1)]);
        r.set(0x0054_68f0, &[1]);
        let main_object = r.object(0x40);
        r.e.set_global(MAIN_OBJECT, main_object);
        r.set(0x004e_2190, &[0x6600]);
        r.e.set_global(0x011f_f104, 1u8);
        r.e.set_global(0x011f_f8c4, 1u8);
        r.e.set_global(0x011c_7a59, 5u8);
        let (_, log) = r.run(0x0087_27d0, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011f_f104), 0);
        assert_eq!(r.e.global::<u8>(0x011f_f8c4), 0);
        assert_eq!(r.e.global::<u8>(0x011c_7a59), 0);
        // The first word of the `0070ec90` object is cleared, so the final
        // stage loop starts at 1.
        assert_eq!(r.e.mem.u32(cell), 0);
        assert_eq!(calls_to(&log, 0x004e_2190), vec![vec![main_object]]);
        assert_eq!(calls_to(&log, 0x004e_2120), vec![vec![0x6600, 0]]);
        assert_eq!(
            stages(&log, MT_SET_THREAD_STAGE),
            (1..0x11).collect::<Vec<u32>>()
        );
        // `005f36f0` was not asked: the stage check already blocked.
        assert!(!addrs(&log).contains(&0x005f_36f0));
        assert_eq!(r.e.global::<u32>(0x011a_9608), 0);
    }

    #[test]
    fn small_stores_write_their_globals() {
        let mut r = Rig::new();
        r.e.call(0x0087_2930, &args![7u32]);
        assert_eq!(r.e.global::<u8>(0x011c_7a59), 7);
        r.e.call(0x0087_2dd0, &args![3u32, 4u32]);
        assert_eq!(r.e.global::<u8>(0x011f_9fc3), 3);
        assert_eq!(r.e.global::<u8>(0x011f_9fc4), 4);
        r.e.call(0x0087_2df0, &args![9u32]);
        assert_eq!(r.e.global::<u8>(0x011f_9fc5), 9);
    }

    #[test]
    fn interface_frame_calls_the_update_of_each_visible_menu() {
        let mut r = Rig::new();
        let this = r.object(0x200);
        r.set(GET_GLOBAL_011F4748, &[0x1000]);
        r.set(GET_GLOBAL_011F91AC, &[0x2000]);
        r.set(0x004d_c310, &[1]);
        r.e.register_double(IS_MENU_ID_VISIBLE, |_, a| Ret {
            eax: [0x41e, 0x432, 0x424].contains(&a[0]) as u32,
            ..Ret::default()
        });
        let (_, log) = r.run(0x0087_2940, &args![this, 0u32]);
        assert_eq!(
            only(
                &log,
                &[
                    0x0070_9ae0,
                    0x0070_9af0,
                    0x0070_9b00,
                    0x0070_9b10,
                    0x0070_9b20,
                    0x0070_9b30,
                    0x0079_48d0,
                    0x007c_9ca0
                ]
            ),
            vec![0x0070_9ae0, 0x0079_48d0, 0x007c_9ca0]
        );
        // The menu ids are asked in order, then 0x424 at the end.
        let asked: Vec<u32> = calls_to(&log, IS_MENU_ID_VISIBLE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(
            asked,
            vec![0x41e, 0x3f6, 0x438, 0x439, 0x43a, 0x43b, 0x432, 0x424]
        );
        assert_eq!(
            calls_to(&log, 0x0087_4b90),
            vec![vec![this, 0x2000, 0x1000, 1, 0]]
        );
        // The frame is built for the default target (no screen given).
        assert_eq!(calls_to(&log, 0x00b6_b890), vec![vec![7, 0]]);
        assert_eq!(
            calls_to(&log, 0x0087_6850),
            vec![vec![this, 0, 0x2000, 0x1000, 0]]
        );
        assert_eq!(*addrs(&log).last().unwrap(), 0x007c_9ca0);
    }

    #[test]
    fn interface_frame_without_menus_only_finishes() {
        let mut r = Rig::new();
        let this = r.object(0x200);
        r.set(GET_GLOBAL_011F4748, &[0x1000]);
        r.set(GET_GLOBAL_011F91AC, &[0x2000]);
        let (_, log) = r.run(0x0087_2940, &args![this, 0x5000u32]);
        assert!(!addrs(&log).contains(&0x007c_9ca0));
        assert!(!addrs(&log).contains(&0x0070_9ae0));
        // A target is cleared with `00876830` (byte flag 0, no overlay flag).
        assert_eq!(calls_to(&log, 0x0087_6830), vec![vec![this, 0, 0x5000]]);
        // With the overlay flag the overlay draw replaces it.
        r.e.set_global(0x011f_91a4, 1u8);
        let (_, log) = r.run(0x0087_2940, &args![this, 0x5000u32]);
        assert!(!addrs(&log).contains(&0x0087_6830));
        assert_eq!(calls_to(&log, 0x0087_5fd0).len(), 1);
    }

    #[test]
    fn interface_prelude_runs_only_when_the_object_exists() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        let (_, log) = r.run(0x0087_2ad0, &args![this]);
        assert_eq!(addrs(&log), vec![0x0068_3a60]);
        r.set(0x0068_3a60, &[1]);
        let (_, log) = r.run(0x0087_2ad0, &args![this]);
        assert_eq!(addrs(&log), vec![0x0068_3a60, 0x0071_4a00, 0x0071_4960]);
        assert_eq!(calls_to(&log, 0x0071_4960), vec![vec![3]]);
        r.set(0x0071_4a00, &[1]);
        let (_, log) = r.run(0x0087_2ad0, &args![this]);
        assert_eq!(
            addrs(&log),
            vec![0x0068_3a60, 0x0071_4a00, 0x00a8_1a80, 0x0071_4960]
        );
    }

    /// The rig of the per-frame setup: a root, its child and a `Main`.
    fn setup_rig() -> (Rig, u32) {
        let mut r = Rig::new();
        let this = r.object(0x200);
        r.set(GET_ROOT, &[0x3000]);
        r.set(GET_ROOT_CHILD, &[0x4000]);
        r.set(0x0044_ddc0, &[4]);
        (r, this)
    }

    #[test]
    fn per_frame_setup_sets_the_color_and_the_field_of_view() {
        let (mut r, this) = setup_rig();
        r.set_float(0x0071_0ab0, 1.25);
        r.e.set_global(PLAYER_OBJECT, 0x5100u32);
        let (ret, log) = r.run(0x0087_2b00, &args![this]);
        assert_eq!(ret.u32(), 0);
        assert_eq!(calls_to(&log, SET_COLOR)[0], vec![0x4000, 0x011a_d840]);
        assert_eq!(calls_to(&log, 0x0044_ddc0)[0], vec![0x011f_2250]);
        // The mode is 4: no field of view on the root, but `00b54000` runs.
        assert!(calls_to(&log, 0x00c5_2020).is_empty());
        assert_eq!(calls_to(&log, 0x0071_0ab0), vec![vec![0x5100]]);
        assert_eq!(calls_to(&log, 0x00b5_4000), vec![vec![word(1.25)]]);
        // Any other mode sets it on the root first.
        r.set(0x0044_ddc0, &[3]);
        let (_, log) = r.run(0x0087_2b00, &args![this]);
        assert_eq!(
            calls_to(&log, 0x00c5_2020),
            vec![vec![0x3000, word(1.25), 0, 0, 1]]
        );
        assert_eq!(calls_to(&log, 0x0071_0ab0).len(), 2);
    }

    #[test]
    fn per_frame_setup_marks_the_rendering_menu_with_a_color() {
        let (mut r, this) = setup_rig();
        let (_, log) = r.run(0x0087_2b00, &args![this]);
        assert!(calls_to(&log, COLOR_CTOR).is_empty());
        r.e.mem.set_u8(this + 0x9c, 1);
        let (_, log) = r.run(0x0087_2b00, &args![this]);
        let ctor = calls_to(&log, COLOR_CTOR);
        assert_eq!(ctor.len(), 1);
        assert_eq!(ctor[0][1..], [0, word(1.0), word(1.0), 0]);
        assert_eq!(calls_to(&log, SET_COLOR)[1], vec![0x4000, ctor[0][0]]);
    }

    #[test]
    fn per_frame_setup_builds_the_actor_culler_once() {
        let (mut r, this) = setup_rig();
        r.settings(SETTING_BYTE_PTR, &[(0x011d_ed80, 1)]);
        r.pointers(&[(ROOT_POINTER, 0x3100)]);
        let (_, log) = r.run(0x0087_2b00, &args![this]);
        // Construction, then the `atexit` of its destructor, then the call.
        assert_eq!(
            only(&log, &[0x005d_8b10, 0x0063_3c90, 0x00ec_658f, 0x008d_6f50]),
            vec![
                0x005d_8b10,
                0x0063_3c90,
                0x0063_3c90,
                0x0063_3c90,
                0x00ec_658f,
                0x008d_6f50
            ]
        );
        assert_eq!(calls_to(&log, 0x005d_8b10), vec![vec![0x011d_ef10]]);
        assert_eq!(calls_to(&log, 0x00ec_658f), vec![vec![0x00fd_9380]]);
        assert_eq!(calls_to(&log, 0x008d_6f50), vec![vec![0x011d_ef10, 0x4000]]);
        assert_eq!(r.e.mem.u32(0x011d_ef10), 0x0108_2cc0);
        assert_eq!(r.e.mem.u32(0x011d_ef18), 0x0108_2ccc);
        assert_eq!(r.e.global::<u32>(0x011d_efb4) & 1, 1);
        // The second frame only calls the method.
        let (_, log) = r.run(0x0087_2b00, &args![this]);
        assert!(!addrs(&log).contains(&0x005d_8b10));
        assert!(!addrs(&log).contains(&0x00ec_658f));
        assert_eq!(calls_to(&log, 0x008d_6f50).len(), 1);
    }

    #[test]
    fn per_frame_setup_resets_the_two_value_holders() {
        let (mut r, this) = setup_rig();
        r.pointers(&[
            (ROOT_POINTER, 0x3100),
            (0x011d_eb34, 0xaaa0),
            (0x011d_eda4, 0xbbb0),
        ]);
        r.set(0x0055_8310, &[0x3200]);
        r.set(0x0043_c490, &[0x3300]);
        let (_, log) = r.run(0x0087_2b00, &args![this]);
        assert_eq!(
            calls_to(&log, 0x0055_8310),
            vec![vec![0x3100], vec![0x3100]]
        );
        assert_eq!(
            calls_to(&log, 0x0043_c490),
            vec![vec![0x3200], vec![0x3200]]
        );
        assert_eq!(
            calls_to(&log, 0x0044_0460),
            vec![vec![0xaaa0, 0x3300], vec![0xbbb0, 0x3300]]
        );
        let points = calls_to(&log, POINT3_CTOR);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0][1..], [0, 0, 0]);
        assert_eq!(
            calls_to(&log, 0x00a5_9c60),
            vec![vec![0xaaa0, points[0][0]], vec![0xbbb0, points[1][0]]]
        );
    }

    #[test]
    fn per_frame_setup_moves_the_flag_and_stores_the_settings() {
        let (mut r, this) = setup_rig();
        r.e.set_global(0x011f_9fc0, 1u8);
        r.e.set_global(0x011f_94a5, 1u8);
        r.settings(SETTING_BYTE_PTR, &[(0x011d_ecd8, 3), (0x011d_eb60, 4)]);
        r.run(0x0087_2b00, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011f_9fc0), 0);
        assert_eq!(r.e.global::<u8>(0x011f_9fc1), 1);
        assert_eq!(r.e.global::<u8>(0x011f_9fc3), 3);
        assert_eq!(r.e.global::<u8>(0x011f_9fc4), 1);
        assert_eq!(r.e.global::<u8>(0x011f_9fc5), 4);
        assert_eq!(r.e.global::<u8>(0x011f_94a5), 0);
        // With the flag clear, `0x011f9fc1` is left alone.
        r.e.set_global(0x011f_9fc1, 7u8);
        r.run(0x0087_2b00, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011f_9fc1), 7);
    }

    #[test]
    fn per_frame_setup_returns_the_object_of_a_rider_or_follower_mode() {
        let (mut r, this) = setup_rig();
        r.set(0x008d_8520, &[0x6000]);
        r.set(0x0045_cd60, &[0x6100]);
        r.set(0x0047_6c90, &[3]);
        let (ret, log) = r.run(0x0087_2b00, &args![this]);
        assert_eq!(ret.u32(), 0x6100);
        assert_eq!(calls_to(&log, 0x0047_6c90).len(), 1);
        // Mode 2 (asked a second time).
        r.set(0x0047_6c90, &[2]);
        let (ret, log) = r.run(0x0087_2b00, &args![this]);
        assert_eq!(ret.u32(), 0x6100);
        assert_eq!(calls_to(&log, 0x0047_6c90).len(), 2);
        // Any other mode, no object, or no `0045cd60` answer.
        r.set(0x0047_6c90, &[1]);
        assert_eq!(r.run(0x0087_2b00, &args![this]).0.u32(), 0);
        r.set(0x0047_6c90, &[3]);
        r.set(0x0045_cd60, &[0]);
        assert_eq!(r.run(0x0087_2b00, &args![this]).0.u32(), 0);
        r.set(0x0045_cd60, &[0x6100]);
        r.set(0x008d_8520, &[0]);
        assert_eq!(r.run(0x0087_2b00, &args![this]).0.u32(), 0);
    }

    #[test]
    fn actor_culler_constructors_set_their_vtables() {
        let mut r = Rig::new();
        let culler = r.object(0xb0);
        let ret = r.e.call(0x0087_2e00, &args![culler]);
        assert_eq!(ret.u32(), culler);
        assert_eq!(r.e.mem.u32(culler), 0x0108_2cc0);
        assert_eq!(r.e.mem.u32(culler + 8), 0x0108_2ccc);
        let (_, log) = r.run(0x0087_2e00, &args![culler]);
        assert_eq!(
            addrs(&log),
            vec![
                0x005d_8b10,
                0x00a6_9400,
                0x0063_3c90,
                0x0063_3c90,
                0x0063_3c90
            ]
        );
        assert_eq!(calls_to(&log, 0x00a6_9400), vec![vec![culler + 8, 0]]);
        assert_eq!(
            calls_to(&log, 0x0063_3c90),
            vec![
                vec![culler + 0x98, 0],
                vec![culler + 0x9c, 0],
                vec![culler + 0xa0, 0]
            ]
        );
    }

    #[test]
    fn fade_node_culler_constructor_and_destructors() {
        let mut r = Rig::new();
        let culler = r.object(0x40);
        let ret = r.e.call(0x0087_2ea0, &args![culler, 5u32]);
        assert_eq!(ret.u32(), culler);
        assert_eq!(r.e.mem.u32(culler), 0x0108_2ccc);
        let (_, log) = r.run(0x0087_2ea0, &args![culler, 5u32]);
        assert_eq!(log, vec![(0x00a6_9400, vec![culler, 5])]);
        // The destructor body, then the free for an odd flag.
        let (ret, log) = r.run(0x0087_2f00, &args![culler, 1u32]);
        assert_eq!(ret.u32(), culler);
        assert_eq!(
            log,
            vec![(0x00a6_93e0, vec![culler]), (0x0040_1030, vec![culler])]
        );
        let (ret, log) = r.run(0x0087_2f00, &args![culler, 0u32]);
        assert_eq!(ret.u32(), culler);
        assert_eq!(log, vec![(0x00a6_93e0, vec![culler])]);
        let (_, log) = r.run(0x0087_2f00, &args![culler, 2u32]);
        assert_eq!(log, vec![(0x00a6_93e0, vec![culler])]);
        let (_, log) = r.run(0x0087_2f30, &args![culler]);
        assert_eq!(log, vec![(0x00a6_93e0, vec![culler])]);
    }

    #[test]
    fn culling_process_stubs() {
        let mut r = Rig::new();
        let (_, log) = r.run(0x0087_2ed0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(log, vec![(IMPORT_DEBUG_BREAK, vec![])]);
        assert_eq!(r.e.call(0x0087_2ef0, &args![0x1000u32]).u32(), 0x0120_1f7c);
    }

    /// The rig of the target selection: a target whose first buffer
    /// answers `buffer_size`, a renderer answering `display`.
    fn target_rig(display: (u32, u32), buffer_size: (u32, u32)) -> (Rig, u32) {
        let mut r = Rig::new();
        r.e.set_global(0x011f_91a4, 1u8);
        r.set(GET_GLOBAL_011F91A8, &[0x2222]);
        let buffer = r.object_with_slots(&[(0x94, buffer_size.0), (0x98, buffer_size.1)]);
        let target = r.object(0x40);
        r.pointers(&[(target + 0x30, buffer)]);
        r.set(0x00b6_e110, &[target]);
        r.set(0x004d_ee10, &[display.0]);
        r.set(0x004d_ee70, &[display.1]);
        r.set(0x00b6_b260, &[0x6600]);
        r.set(GET_ROOT_CHILD_OF_ROOT, &[0x4000]);
        (r, target)
    }

    #[test]
    fn target_selection_creates_the_target_and_makes_its_group_current() {
        let (mut r, target) = target_rig((1280, 720), (1280, 720));
        let this = r.object(0x10);
        let (ret, log) = r.run(0x0087_2f50, &args![this, 0x1000u32, 0x5000u32, 0u32, 0u32]);
        assert_eq!(ret.u32(), target);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0].clone();
        assert_eq!(guard[1..], [0x27, 1, SOURCE_FILE_NAME, 0x1e6d]);
        assert_eq!(
            calls_to(&log, 0x00b6_e110),
            vec![vec![0x2222, 0x1000, 4, 0, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x0087_31d0), vec![vec![target]]);
        assert_eq!(r.e.global::<u32>(0x011f_9438), target);
        assert_eq!(calls_to(&log, 0x00b6_b260), vec![vec![target]]);
        assert_eq!(calls_to(&log, 0x00b6_b8d0), vec![vec![7, 0x6600]]);
        // The sizes match: no color.
        assert!(calls_to(&log, COLOR_CTOR).is_empty());
        assert_eq!(*addrs(&log).last().unwrap(), PROFILE_SCOPE_DTOR);
    }

    #[test]
    fn target_selection_sets_the_ratios_when_the_width_or_height_differs() {
        for display in [(1920, 720), (1280, 1080)] {
            let (mut r, _) = target_rig(display, (1280, 720));
            r.e.set_global(0x0108_2d20, 1.0f64);
            let calls = Rc::new(RefCell::new(0));
            {
                let calls = calls.clone();
                r.e.register_double(FLOAT_HELPER_C, move |_, _| {
                    let mut calls = calls.borrow_mut();
                    *calls += 1;
                    Ret {
                        st0: if *calls == 1 { 1.25 } else { 0.75 },
                        ..Ret::default()
                    }
                });
            }
            let this = r.object(0x10);
            let (_, log) = r.run(0x0087_2f50, &args![this, 0x1000u32, 0u32, 0u32, 0u32]);
            let ratios = calls_to(&log, FLOAT_HELPER_C);
            assert_eq!(ratios.len(), 2);
            let first = f64::from_bits(ratios[0][0] as u64 | (ratios[0][1] as u64) << 32);
            let second = f64::from_bits(ratios[1][0] as u64 | (ratios[1][1] as u64) << 32);
            assert_eq!(first, display.0 as f64 / 1280.0);
            assert_eq!(second, display.1 as f64 / 720.0);
            assert_eq!(ratios[0][2], word(1.0));
            let color = calls_to(&log, COLOR_CTOR)[0].clone();
            assert_eq!(color[1..], [0, word(1.25), word(0.75), 0]);
            assert_eq!(calls_to(&log, SET_COLOR), vec![vec![0x4000, color[0]]]);
        }
    }

    #[test]
    fn target_selection_scales_the_ratio_by_the_constant() {
        let (mut r, _) = target_rig((1000, 500), (1000, 500));
        // A different size than the display makes the ratios appear.
        r.set(0x004d_ee10, &[2000]);
        r.e.set_global(0x0108_2d20, 0.5f64);
        let this = r.object(0x10);
        let (_, log) = r.run(0x0087_2f50, &args![this, 0x1000u32, 0u32, 0u32, 0u32]);
        let ratios = calls_to(&log, FLOAT_HELPER_C);
        let first = f64::from_bits(ratios[0][0] as u64 | (ratios[0][1] as u64) << 32);
        assert_eq!(first, 2000.0 * 0.5 / 1000.0);
    }

    #[test]
    fn target_selection_without_a_new_target_picks_a_group() {
        let mut r = Rig::new();
        let this = r.object(0x10);
        r.set(0x00b6_b260, &[0x6600]);
        r.object_at(0x1000, &[(0xc8, 0x6c00)]);
        // The new-target flag stops the creation even with `008709f0`.
        r.e.set_global(0x011f_91a4, 1u8);
        let (ret, log) = r.run(0x0087_2f50, &args![this, 0x1000u32, 0x5000u32, 1u32, 0u32]);
        assert_eq!(ret.u32(), 0);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0].clone();
        assert_eq!(guard[1..], [0x26, 1, SOURCE_FILE_NAME, 0x1e84]);
        assert_eq!(calls_to(&log, 0x00b6_b260), vec![vec![0x5000]]);
        assert_eq!(calls_to(&log, 0x00b6_b8d0), vec![vec![7, 0x6600]]);
        assert!(!addrs(&log).contains(&0x00b6_e110));
        // The renderer's own group.
        let (_, log) = r.run(0x0087_2f50, &args![this, 0x1000u32, 0x5000u32, 1u32, 1u32]);
        assert_eq!(calls_to(&log, 0x00b6_b8d0), vec![vec![7, 0x6c00]]);
        assert!(!addrs(&log).contains(&0x00b6_b260));
        // No screen: the default group.
        let (_, log) = r.run(0x0087_2f50, &args![this, 0x1000u32, 0u32, 1u32, 1u32]);
        assert_eq!(calls_to(&log, 0x00b6_b890), vec![vec![7, 0]]);
        assert!(!addrs(&log).contains(&0x00b6_b8d0));
        // `008709f0` clear: also the second branch.
        r.e.set_global(0x011f_91a4, 0u8);
        let (_, log) = r.run(0x0087_2f50, &args![this, 0x1000u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls_to(&log, 0x00b6_b890), vec![vec![7, 0]]);
        assert!(!addrs(&log).contains(&0x00b6_e110));
    }

    // END TESTS
}

#[cfg(test)]
mod tests_second {
    //! Tests of `008731a0` to `00877260`. Every address the translations
    //! call outside this file has a test double, so that nothing another
    //! unit translated runs for real.
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Returns = Rc<RefCell<HashMap<u32, Vec<Ret>>>>;

    const CALLEES: &[u32] = &[
        0x0040_1000,
        0x0040_3df0,
        0x0040_4dc0,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_4f70,
        0x0040_6d00,
        0x0040_6d30,
        0x0040_6d50,
        0x0040_8d60,
        0x0040_df90,
        0x0041_4430,
        0x0041_6870,
        0x0041_fd00,
        0x0042_5fd0,
        0x0042_ce10,
        0x0043_6aa0,
        0x0043_9340,
        0x0043_9390,
        0x0043_9e90,
        0x0043_9ef0,
        0x0043_b230,
        0x0043_b480,
        0x0043_b4a0,
        0x0043_c490,
        0x0043_c4b0,
        0x0043_d410,
        0x0043_d4d0,
        0x0043_fa80,
        0x0044_0460,
        0x0044_a670,
        0x0044_ddc0,
        0x0045_0b80,
        0x0045_0f90,
        0x0045_15a0,
        0x0045_3dc0,
        0x0045_4450,
        0x0045_4b30,
        0x0045_6520,
        0x0045_6610,
        0x0045_6630,
        0x0045_7050,
        0x0045_7070,
        0x0045_8200,
        0x0045_bb80,
        0x0045_bba0,
        0x0045_bbe0,
        0x0045_bc00,
        0x0045_c4c0,
        0x0045_c670,
        0x0045_cec0,
        0x0046_0140,
        0x0046_1130,
        0x0046_1ae0,
        0x0046_1c20,
        0x0046_1cf0,
        0x0046_4f30,
        0x0046_50a0,
        0x0047_6a80,
        0x0048_f200,
        0x0049_ed90,
        0x004a_0bd0,
        0x004a_0d10,
        0x004a_0e90,
        0x004a_0ea0,
        0x004a_0eb0,
        0x004a_0f60,
        0x004a_0fd0,
        0x004a_1020,
        0x004a_1a30,
        0x004a_4460,
        0x004a_d0c0,
        0x004a_d1d0,
        0x004b_a7d0,
        0x004b_c320,
        0x004d_61b0,
        0x004d_c020,
        0x004d_c110,
        0x004d_c540,
        0x004e_3270,
        0x004e_9510,
        0x004e_9bb0,
        0x004e_a970,
        0x004e_bbc0,
        0x004e_ced0,
        0x004e_d8c0,
        0x004e_d900,
        0x004f_00e0,
        0x004f_b0b0,
        0x0050_7700,
        0x0052_4c90,
        0x0052_5430,
        0x0054_4c30,
        0x0054_4c60,
        0x0054_6780,
        0x0054_8230,
        0x0054_cfd0,
        0x0054_ddd0,
        0x0055_8310,
        0x0055_85e0,
        0x0055_9a40,
        0x0056_21d0,
        0x0056_5730,
        0x0056_fa00,
        0x0057_cbe0,
        0x0058_5b30,
        0x005b_5e40,
        0x005b_9b00,
        0x005b_ac50,
        0x005b_b4d0,
        0x005f_36f0,
        0x0063_3c90,
        0x0064_26a0,
        0x0064_2860,
        0x0064_2890,
        0x0064_47f0,
        0x0066_29f0,
        0x0066_b0d0,
        0x0068_15c0,
        0x0068_38b0,
        0x0068_3a60,
        0x0068_8430,
        0x006a_9540,
        0x006a_b360,
        0x006e_5cc0,
        0x0070_2360,
        0x0070_2870,
        0x0070_5910,
        0x0070_5990,
        0x0070_5a00,
        0x0070_5e80,
        0x0070_79b0,
        0x0070_7ad0,
        0x0070_9b50,
        0x0071_0ab0,
        0x0071_2e60,
        0x0071_48c0,
        0x0071_4900,
        0x0071_4a40,
        0x0071_4a60,
        0x0071_4ae0,
        0x0072_6070,
        0x0076_b610,
        0x0077_21a0,
        0x007d_6bb0,
        0x007f_8720,
        0x007f_a950,
        0x007f_ae70,
        0x0080_0f30,
        0x0080_19d0,
        0x0080_1a60,
        0x0082_56d0,
        0x0084_e3a0,
        0x0086_d480,
        0x0087_7720,
        0x008a_8870,
        0x008b_6200,
        0x008b_bc10,
        0x008c_51c0,
        0x008c_80e0,
        0x008d_80e0,
        0x0093_6aa0,
        0x0095_0bb0,
        0x0096_11e0,
        0x0096_7ae0,
        0x009b_4440,
        0x009d_9f20,
        0x00a2_3a50,
        0x00a2_9680,
        0x00a5_9c60,
        0x00a5_9d30,
        0x00a5_a040,
        0x00a6_94a0,
        0x00a6_faf0,
        0x00a8_6bf0,
        0x00a8_9af0,
        0x00aa_13e0,
        0x00af_43a0,
        0x00af_4460,
        0x00af_4470,
        0x00af_4490,
        0x00af_4550,
        0x00b4_f450,
        0x00b4_f510,
        0x00b5_4000,
        0x00b5_40b0,
        0x00b5_4ac0,
        0x00b5_5ac0,
        0x00b5_ba80,
        0x00b5_cbd0,
        0x00b5_e870,
        0x00b6_30c0,
        0x00b6_3b90,
        0x00b6_4570,
        0x00b6_51e0,
        0x00b6_5550,
        0x00b6_5660,
        0x00b6_5ec0,
        0x00b6_60d0,
        0x00b6_b260,
        0x00b6_b790,
        0x00b6_b8d0,
        0x00b6_b930,
        0x00b6_ba20,
        0x00b6_bee0,
        0x00b6_bfc0,
        0x00b6_c0d0,
        0x00b6_da10,
        0x00b6_e110,
        0x00b8_03b0,
        0x00b9_7550,
        0x00b9_7de0,
        0x00b9_7e30,
        0x00b9_7fa0,
        0x00b9_8380,
        0x00b9_88e0,
        0x00ba_30f0,
        0x00ba_3130,
        0x00ba_3390,
        0x00ba_3770,
        0x00ba_3840,
        0x00ba_39a0,
        0x00ba_9420,
        0x00ba_d4a0,
        0x00c3_bb80,
        0x00c3_bbd0,
        0x00c3_bc20,
        0x00c4_9850,
        0x00c4_f270,
        0x00c4_f2d0,
        0x00c4_f320,
        0x00c5_2020,
        0x00c5_b5a0,
        0x00c5_b5d0,
        0x00ec_6130,
        0x00ec_a6d3,
        0x00ec_bf05,
        0x00fd_f004,
        0x00fd_f008,
        0x00fd_f00c,
        0x00fd_f0b8,
        0x00fd_f0c4,
        0x00fd_f0d4,
        0x00fd_f1b8,
        0x00fd_f274,
    ];

    /// The engine with a double for every callee and the data pages
    /// mapped (zeroed). `00559450` (`NiPointer` read) reads memory, and
    /// the settings accessors point at a zero value.
    struct Rig {
        e: Engine,
        returns: Returns,
        zero: u32,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            for page in (0x0100_0000u32..0x0130_0000).step_by(0x1000) {
                e.map(page, 0x1000);
            }
            let returns: Returns = Rc::new(RefCell::new(HashMap::new()));
            for &addr in CALLEES {
                let table = returns.clone();
                e.register_double(addr, move |_, _| {
                    let mut table = table.borrow_mut();
                    match table.get_mut(&addr) {
                        Some(queue) if queue.len() > 1 => queue.remove(0),
                        Some(queue) if queue.len() == 1 => queue[0],
                        _ => Ret::default(),
                    }
                });
            }
            e.register_double(POINTER_GET, |e, a| Ret {
                eax: e.mem.u32(a[0]),
                ..Ret::default()
            });
            // `NiPointer` assignment and construction store the pointer.
            for addr in [NI_POINTER_ASSIGN, NI_POINTER_CTOR] {
                e.register_double(addr, |e, a| {
                    e.mem.set_u32(a[0], a[1]);
                    Ret {
                        eax: a[0],
                        ..Ret::default()
                    }
                });
            }
            e.register_double(NI_POINTER_COPY_CTOR, |e, a| {
                let value = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], value);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            let zero = e.mem.alloc(0x100);
            let mut rig = Rig { e, returns, zero };
            for setting in [SETTING_DWORD_PTR, SETTING_BYTE_PTR] {
                rig.set(setting, &[zero]);
            }
            rig
        }

        /// The values the double at `addr` returns in `eax`, in order; the
        /// last one repeats.
        fn set(&mut self, addr: u32, values: &[u32]) {
            let queue = values
                .iter()
                .map(|&eax| Ret {
                    eax,
                    ..Ret::default()
                })
                .collect();
            self.returns.borrow_mut().insert(addr, queue);
        }

        /// A double that returns `value` in ST0.
        fn set_float(&mut self, addr: u32, value: f64) {
            let ret = Ret {
                st0: value,
                ..Ret::default()
            };
            self.returns.borrow_mut().insert(addr, vec![ret]);
        }

        fn object(&mut self, size: u32) -> u32 {
            self.e.mem.alloc(size)
        }

        /// An object whose vtable slots (byte offset, result) are doubles
        /// returning the given result in `eax`.
        fn object_with_slots(&mut self, slots: &[(u32, u32)]) -> u32 {
            let object = self.e.mem.alloc(0x40);
            let vtable = self.e.mem.alloc(0x400);
            self.e.mem.set_u32(object, vtable);
            for &(offset, result) in slots {
                let target = 0x00f0_0000 + (vtable & 0xffff) * 0x400 + offset;
                self.e.register_double(target, move |_, _| Ret {
                    eax: result,
                    ..Ret::default()
                });
                self.e.mem.set_u32(vtable + offset, target);
            }
            object
        }

        /// A settings accessor that answers by the setting it is asked
        /// about (`values`: setting address, value); any other setting
        /// reads 0.
        fn settings(&mut self, accessor: u32, values: &[(u32, u32)]) {
            let zero = self.zero;
            let mut blocks: HashMap<u32, u32> = HashMap::new();
            for &(setting, value) in values {
                let block = self.e.mem.alloc(8);
                self.e.mem.set_u32(block, value);
                blocks.insert(setting, block);
            }
            self.e.register_double(accessor, move |_, a| Ret {
                eax: blocks.get(&a[0]).copied().unwrap_or(zero),
                ..Ret::default()
            });
        }

        /// Runs `addr` and returns its result and the calls it made.
        fn run(&mut self, addr: u32, args: &[u32]) -> (Ret, Log) {
            self.e.call_log = Some(vec![]);
            let ret = self.e.call(addr, args);
            let mut log = self.e.call_log.take().unwrap();
            log.remove(0);
            // The pointer reads are noise for these tests.
            log.retain(|(addr, _)| *addr != POINTER_GET);
            (ret, log)
        }
    }

    fn addrs(log: &Log) -> Vec<u32> {
        log.iter().map(|(addr, _)| *addr).collect()
    }

    /// The addresses of `log` that are in `wanted`, in call order.
    fn only(log: &Log, wanted: &[u32]) -> Vec<u32> {
        addrs(log)
            .into_iter()
            .filter(|addr| wanted.contains(addr))
            .collect()
    }

    /// The argument lists of the calls to `addr`.
    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn word(value: f32) -> u32 {
        value.to_bits()
    }

    #[test]
    fn float_helper_returns_the_smaller_value() {
        let mut r = Rig::new();
        assert_eq!(fn_008731a0(&mut r.e, 3.0, 5.0), 3.0);
        assert_eq!(fn_008731a0(&mut r.e, 7.0, 5.0), 5.0);
        // Equal or unordered: the float.
        assert_eq!(fn_008731a0(&mut r.e, 5.0, 5.0), 5.0);
        assert_eq!(fn_008731a0(&mut r.e, f64::NAN, 5.0), 5.0);
        let ret = r.e.call(0x0087_31a0, &args![2.5f64, 9.0f32]);
        assert_eq!(ret.f64(), 2.5);
    }

    #[test]
    fn pointer_member_is_passed_on_when_set() {
        let mut r = Rig::new();
        let this = r.object(0x40);
        let (_, log) = r.run(0x0087_31d0, &args![this]);
        assert!(calls_to(&log, 0x006e_5cc0).is_empty());
        r.e.mem.set_u32(this + 0x20, 0x7000);
        let (_, log) = r.run(0x0087_31d0, &args![this]);
        assert_eq!(
            calls_to(&log, 0x006e_5cc0),
            vec![vec![this + 8, this + 0x20]]
        );
    }

    #[test]
    fn small_accessors_read_and_write_their_fields() {
        let mut r = Rig::new();
        let this = r.object(0x200);
        r.e.mem.set_u32(this + 0xfc, 0x1234);
        r.e.mem.set_u8(this + 0x1dc, 9);
        r.e.mem.set_u8(this + 0x118, 5);
        r.e.mem.set_f32(this + 0x674, 1.5);
        assert_eq!(r.e.call(0x0087_4480, &args![this]).u32(), 0x1234);
        assert_eq!(r.e.call(0x0087_4650, &args![this]).u8(), 9);
        assert_eq!(r.e.call(0x0087_4aa0, &args![this]).u8(), 5);
        assert_eq!(r.e.call(0x0087_4670, &args![this]).u32(), this + 0x138);
        assert_eq!(r.e.call(0x0087_4900, &args![this]).f32(), 1.5);
        r.e.call(0x0087_4460, &args![this, 0x55u32]);
        assert_eq!(r.e.mem.u32(this + 0xc0), 0x55);
        r.e.call(0x0087_50e0, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(
            [
                r.e.mem.u32(this + 0x1e4),
                r.e.mem.u32(this + 0x1e8),
                r.e.mem.u32(this + 0x1ec)
            ],
            [1, 2, 3]
        );
        r.e.set_global(0x011f_f0f0, 7u32);
        r.e.call(0x0087_46c0, &args![]);
        assert_eq!(r.e.global::<u32>(0x011f_f0f0), 0);
    }

    #[test]
    fn global_pointer_getters_read_their_slots() {
        let mut r = Rig::new();
        r.e.mem.set_u32(0x011c_7840, 0x11);
        r.e.mem.set_u32(0x011c_7810, 0x22);
        r.e.mem.set_u32(0x011c_781c, 0x33);
        assert_eq!(r.e.call(0x0087_4690, &args![]).u32(), 0x11);
        assert_eq!(r.e.call(0x0087_46a0, &args![]).u32(), 0x22);
        assert_eq!(r.e.call(0x0087_46b0, &args![]).u32(), 0x33);
    }

    #[test]
    fn pipboy_array_reader_indexes_the_pointer_array() {
        let mut r = Rig::new();
        let this = r.object(0x200);
        r.e.mem.set_u32(this + 0xf4 + 8, 0x77);
        assert_eq!(r.e.call(0x0087_5bd0, &args![this, 2u32]).u32(), 0x77);
    }

    /// The address of vtable slot `offset` of `object` (a double made by
    /// [`Rig::object_with_slots`]).
    fn slot_target(r: &Rig, object: u32, offset: u32) -> u32 {
        let vtable = r.e.mem.u32(object);
        r.e.mem.u32(vtable + offset)
    }

    #[test]
    fn movement_detector_flags_a_change_and_stores_the_vectors() {
        let mut r = Rig::new();
        r.e.set_global(0x011a_9490, 1.0f32);
        r.e.set_global(0x011a_9494, 2.0f32);
        r.e.set_global(0x011a_9498, 3.0f32);
        r.e.set_global(0x0120_03cc, 1u8);
        // A null object copies the defaults and clears the flag.
        r.e.call(0x0087_44a0, &args![0u32]);
        assert_eq!(r.e.global::<f32>(0x0120_03d4), 1.0);
        assert_eq!(r.e.global::<f32>(0x0120_03d8), 2.0);
        assert_eq!(r.e.global::<f32>(0x0120_03dc), 3.0);
        assert_eq!(r.e.global::<u8>(0x0120_03cc), 0);

        let member = r.object(0x10);
        for (k, v) in [1.5f32, 1.0, 1.0].iter().enumerate() {
            r.e.mem.set_f32(member + 4 * k as u32, *v);
        }
        r.set(MEMBER_8C, &[member]);
        r.e.register_double(0x0045_bba0, |e, a| {
            for k in 0..3 {
                e.mem.set_f32(a[1] + 4 * k, 3.0);
            }
            Ret::default()
        });
        r.e.register_double(0x0043_9ef0, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        r.e.register_double(0x0041_6870, |e, a| {
            for k in 0..3 {
                e.mem.set_u32(a[0] + 4 * k, a[1 + k as usize]);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        r.e.set_global(0x0103_1148, 0.01f64);
        for k in 0..3 {
            r.e.set_global(0x0120_03e0 + 4 * k, 1.0f32);
            r.e.set_global(0x0120_03d4 + 4 * k, 3.0f32);
        }
        r.e.call(0x0087_44a0, &args![0x1234u32]);
        // The x position moved by 0.5 of 1.0: flagged.
        assert_eq!(r.e.global::<u8>(0x0120_03cc), 1);
        assert_eq!(r.e.global::<f32>(0x0120_03e0), 1.5);
        assert_eq!(r.e.global::<f32>(0x0120_03e4), 1.0);
        assert_eq!(r.e.global::<f32>(0x0120_03d4), 3.0);
        // The same vectors again: nothing moved.
        r.e.call(0x0087_44a0, &args![0x1234u32]);
        assert_eq!(r.e.global::<u8>(0x0120_03cc), 0);
    }

    /// The rig of the passes that take the root, the player and the scene
    /// target: root `0x4000`, its child `0x6000`, target `0x5000`, a player
    /// whose field of view (`+0x674`) is 1.25.
    fn scene_rig() -> (Rig, u32) {
        let mut r = Rig::new();
        // The accumulators and the camera sit at fixed addresses.
        r.e.map(0x8000, 0x3000);
        let texture = r.object(0x100);
        r.set(0x004e_bbc0, &[texture]);
        let player = r.object(0x700);
        r.e.mem.set_f32(player + 0x674, 1.25);
        r.e.set_global(PLAYER_OBJECT, player);
        r.set(GET_ROOT, &[0x4000]);
        r.set(GET_ROOT_CHILD, &[0x6000]);
        r.set(GET_SCENE_TARGET, &[0x5000]);
        r.set(GET_PLAYER_NODE, &[0x3000]);
        r.set_float(0x0071_0ab0, 2.5);
        let this = r.object(0x200);
        r.e.mem.set_u32(this + 0x88, 0x8800);
        r.e.mem.set_u32(this + 0x8c, 0x8c00);
        r.e.mem.set_u32(this + 0x90, 0x9000);
        r.e.mem.set_u32(this + 0x94, 0x9400);
        r.e.mem.set_u32(this + 0x98, 0x9800);
        r.e.mem.set_u32(this + 0xa0, 0xa000);
        (r, this)
    }

    #[test]
    fn offscreen_interface_pass_borrows_the_target_only_when_aiming() {
        let (mut r, this) = scene_rig();
        let decal = r.object_with_slots(&[(0xac, 0), (0xb0, 0), (0xb4, 0)]);
        r.set(GET_GLOBAL_011F9508, &[decal]);
        r.set(GET_NODE_FLAG, &[1]);
        let (_, log) = r.run(0x0087_46d0, &args![this, 0u32]);
        // Not aiming: only the 4eced0 wind-down, no vtable calls.
        assert_eq!(calls_to(&log, 0x00b9_8380), vec![vec![0, 1]]);
        assert_eq!(calls_to(&log, 0x004e_ced0), vec![vec![1]]);
        assert!(calls_to(&log, slot_target(&r, decal, 0xb4)).is_empty());
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![vec![0x6000, 0x3000, 0x5000]]
        );
        assert_eq!(
            calls_to(&log, SET_CAMERA_FOV)[0],
            vec![0x4000, word(1.25), 0, 0, 0]
        );
        assert_eq!(
            calls_to(&log, SET_CAMERA_FOV)[1],
            vec![0x4000, word(2.5), 0, 0, 0]
        );
        assert_eq!(
            calls_to(&log, 0x004a_0fd0),
            vec![vec![0x5000, 0x9000], vec![0x5000, 0]]
        );
        assert_eq!(calls_to(&log, 0x00b6_c0d0), vec![vec![0x6000, 0x9000, 0]]);
        // The node's flag is cleared, then restored from the first read.
        assert_eq!(
            calls_to(&log, SET_NODE_FLAG),
            vec![vec![0x3000, 0], vec![0x3000, 1]]
        );

        r.set(0x008b_bc10, &[1]);
        r.set(0x004e_bbc0, &[0x8000]);
        r.set(0x00ba_3770, &[0x9100]);
        r.set(0x00b6_b260, &[0x9200]);
        let (_, log) = r.run(0x0087_46d0, &args![this, 0u32]);
        assert_eq!(calls_to(&log, 0x00ba_3840), vec![vec![0x8000 + 0x7c, 0x27]]);
        assert_eq!(calls_to(&log, 0x00b6_b260), vec![vec![0x9100]]);
        assert_eq!(calls_to(&log, 0x00b6_b8d0), vec![vec![1, 0x9200]]);
        let first = calls_to(&log, slot_target(&r, decal, 0xb4));
        assert_eq!(first.len(), 1);
        assert_eq!(first[0][0], decal);
        assert_eq!(
            calls_to(&log, slot_target(&r, decal, 0xb0)),
            vec![vec![decal, 0x011f_4998]]
        );
        let last = calls_to(&log, slot_target(&r, decal, 0xac));
        assert_eq!(last, vec![vec![decal, first[0][1]]]);
        assert_eq!(calls_to(&log, 0x00b6_b790).len(), 2);
        assert!(calls_to(&log, 0x004e_ced0).is_empty());
    }

    #[test]
    fn borrowing_the_image_space_texture_returns_its_rendered_texture() {
        let mut r = Rig::new();
        r.set(0x00ba_3770, &[0x1357]);
        let (ret, log) = r.run(0x0087_48d0, &args![0x2000u32]);
        assert_eq!(ret.u32(), 0x1357);
        assert_eq!(calls_to(&log, 0x00ba_3840), vec![vec![0x207c, 0x27]]);
        assert_eq!(calls_to(&log, 0x00ba_3770), vec![vec![0x207c]]);
    }

    #[test]
    fn scoped_accumulation_runs_between_the_profiler_calls() {
        let (mut r, this) = scene_rig();
        let (_, log) = r.run(0x0087_4920, &args![this, 0x1111u32, 0x2222u32]);
        assert_eq!(
            addrs(&log),
            vec![
                PROFILE_SCOPE_CTOR,
                GET_ROOT,
                GET_ROOT_CHILD,
                0x00b6_4570,
                PROFILE_SCOPE_DTOR
            ]
        );
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0][0];
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR),
            vec![vec![guard, 0x27, 1, SOURCE_FILE_NAME, 0x20d5]]
        );
        assert_eq!(
            calls_to(&log, 0x00b6_4570),
            vec![vec![0x1111, 0x6000, 0x2222]]
        );
        assert_eq!(calls_to(&log, PROFILE_SCOPE_DTOR), vec![vec![guard]]);
    }

    #[test]
    fn overlay_targets_are_drawn_only_with_all_conditions() {
        let (mut r, this) = scene_rig();
        let interface = r.object(0x200);
        r.set(GET_INTERFACE_OBJECT, &[interface]);
        // The interface holds no pointer at all: nothing happens.
        let (ret, log) = r.run(0x0087_49b0, &args![this]);
        assert_eq!(ret.u8(), 0);
        assert_eq!(addrs(&log), vec![GET_INTERFACE_OBJECT, 0x0055_85e0]);
        r.set(0x0055_85e0, &[1]);
        r.set(GET_GLOBAL_011F4748, &[0x1500]);
        r.set(GET_GLOBAL_011F91A8, &[0x1600]);
        r.set(0x00b6_e110, &[0x7777]);
        // The byte at +0x118 is clear: the target is created, no draw.
        let (ret, log) = r.run(0x0087_49b0, &args![this]);
        assert_eq!(ret.u8(), 0);
        assert_eq!(
            calls_to(&log, 0x00b6_e110),
            vec![vec![0x1600, 0x1500, 0x2f, 0, 0, 0]]
        );
        assert_eq!(r.e.mem.u32(0x011d_eb38), 0x7777);
        assert!(calls_to(&log, DRAW_TARGET_TO_TEXTURE).is_empty());
        // With the byte set both draws run, with the existing target.
        r.e.mem.set_u8(interface + 0x118, 1);
        let (ret, log) = r.run(0x0087_49b0, &args![this]);
        assert_eq!(ret.u8(), 1);
        assert!(calls_to(&log, 0x00b6_e110).is_empty());
        assert_eq!(
            calls_to(&log, DRAW_TARGET_TO_TEXTURE),
            vec![
                vec![interface, 0x7777, 0, 1, 0, 7, 0],
                vec![interface, 0x7777, 1, 1, 0, 1, 1]
            ]
        );
        // The interface's own pointer set: no draw.
        r.e.mem.set_u32(interface, 0x99);
        let (ret, _) = r.run(0x0087_49b0, &args![this]);
        assert_eq!(ret.u8(), 0);
    }

    #[test]
    fn target_pass_draws_twice_around_the_state_change() {
        let (mut r, this) = scene_rig();
        let interface = r.object(0x200);
        r.set(GET_INTERFACE_OBJECT, &[interface]);
        r.e.mem.set_u32(0x011d_eb38, 0x7777);
        let (_, log) = r.run(0x0087_4ac0, &args![this, 0u32]);
        assert!(calls_to(&log, DRAW_TARGET_TO_TEXTURE).is_empty());
        r.set(0x0055_85e0, &[1]);
        let (_, log) = r.run(0x0087_4ac0, &args![this, 0u32]);
        assert_eq!(
            only(
                &log,
                &[
                    0x00b6_b790,
                    0x00b9_8380,
                    DRAW_TARGET_TO_TEXTURE,
                    0x004e_ced0
                ]
            ),
            vec![
                0x00b6_b790,
                0x00b9_8380,
                DRAW_TARGET_TO_TEXTURE,
                0x004e_ced0,
                DRAW_TARGET_TO_TEXTURE
            ]
        );
        assert_eq!(
            calls_to(&log, DRAW_TARGET_TO_TEXTURE),
            vec![
                vec![interface, 0x7777, 0, 0, 1, 7, 0],
                vec![interface, 0x7777, 0, 0, 0, 1, 1]
            ]
        );
    }

    #[test]
    fn group_selection_needs_the_renderer_check() {
        let (mut r, this) = scene_rig();
        r.set(0x00b6_b260, &[0x3333]);
        let (_, log) = r.run(0x0087_4b50, &args![this, 0x2000u32, 0u32, 6u32]);
        assert!(calls_to(&log, 0x00b6_b8d0).is_empty());
        r.set(0x004e_9510, &[1]);
        let (_, log) = r.run(0x0087_4b50, &args![this, 0x2000u32, 0u32, 6u32]);
        assert_eq!(calls_to(&log, 0x00b6_b260), vec![vec![0x2000]]);
        assert_eq!(calls_to(&log, 0x00b6_b8d0), vec![vec![6, 0x3333]]);
    }

    #[test]
    fn menu_target_draw_masks_the_last_word() {
        let mut r = Rig::new();
        let this = r.object(0x40);
        let (_, log) = r.run(0x0087_4b90, &args![this, 0x10u32, 0x20u32, 0u32, 0x30u32]);
        assert!(addrs(&log).is_empty());
        r.e.mem.set_u32(0x011d_ed3c, 0x6600);
        let (_, log) = r.run(0x0087_4b90, &args![this, 0x10u32, 0x20u32, 0u32, 0x30u32]);
        assert_eq!(
            addrs(&log),
            vec![
                0x00b9_8380,
                0x00b9_7fa0,
                0x00b9_7550,
                0x0071_4ae0,
                0x004e_ced0
            ]
        );
        assert_eq!(calls_to(&log, 0x00b9_8380), vec![vec![7, 1]]);
        assert_eq!(calls_to(&log, 0x00b9_7fa0), vec![vec![0, 1]]);
        assert_eq!(
            calls_to(&log, 0x00b9_7550),
            vec![vec![0x10, 0x22, 0x20, 0x6600, 0x30, 0, 1]]
        );
        let (_, log) = r.run(0x0087_4b90, &args![this, 0x10u32, 0x20u32, 1u32, 0x30u32]);
        assert_eq!(
            calls_to(&log, 0x00b9_7550),
            vec![vec![0x10, 0x22, 0x20, 0x6600, 0, 0, 1]]
        );
    }

    #[test]
    fn first_person_setup_without_an_active_node_pair_clears_the_flag() {
        let (mut r, this) = scene_rig();
        r.set(GET_ROOT_CHILD_OF_ROOT, &[0x6100]);
        r.set(GET_MT_SYSTEM, &[0x1234]);
        r.e.set_global(0x011d_e9d1, 1u8);
        let (_, log) = r.run(0x0087_4c10, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011d_e9d1), 0);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0][0];
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR),
            vec![vec![guard, 0x34, 1, SOURCE_FILE_NAME, 0x21b1]]
        );
        assert_eq!(calls_to(&log, PROFILE_SCOPE_DTOR), vec![vec![guard]]);
        // No rendered menu: the accumulation task is queued, then stage 0x16.
        assert_eq!(
            calls_to(&log, ADD_ACCUM_TASK),
            vec![vec![0x1234, 0xa000, 0, 0x3000, 0, 0, 0x8c00, 0, 0x16, 0]]
        );
        assert_eq!(
            calls_to(&log, MT_SET_THREAD_STAGE),
            vec![vec![0x1234, 0, 0x16]]
        );
        assert_eq!(
            calls_to(&log, SET_CAMERA_FOV),
            vec![vec![0x4000, word(1.25), 0, 0, 0]]
        );
        assert_eq!(calls_to(&log, SET_NODE_FLAG), vec![vec![0x3000, 0]]);
        // Another rendered menu than the pipboy: no task, still the stage.
        r.set(GET_CURRENT_RENDERED_MENU, &[0x7000]);
        r.set(GET_PIPBOY, &[0x7100]);
        let (_, log) = r.run(0x0087_4c10, &args![this]);
        assert!(calls_to(&log, ADD_ACCUM_TASK).is_empty());
        assert_eq!(calls_to(&log, MT_SET_THREAD_STAGE).len(), 1);
    }

    #[test]
    fn first_person_setup_moves_the_camera_when_one_flag_is_set() {
        let (mut r, this) = scene_rig();
        r.set(GET_ROOT_CHILD_OF_ROOT, &[0x6100]);
        r.set(GET_MT_SYSTEM, &[0x1234]);
        // Flag of the first node clear, of the second set: the body runs.
        r.set(GET_NODE_FLAG, &[0, 1]);
        let block = r.object(0x40);
        for (k, v) in [4.0f32, 5.0, 6.0].iter().enumerate() {
            r.e.mem.set_f32(block + 4 * k as u32, *v);
        }
        r.set(MEMBER_8C, &[block]);
        r.set(MEMBER_8C_B, &[0x6200]);
        r.set(0x004a_0d10, &[0x6300]);
        r.set(0x0046_1130, &[block]);
        r.set(0x0043_c490, &[block]);
        r.set(0x0043_9e90, &[0x6400]);
        r.set_float(0x0064_47f0, 0.75);
        r.e.register_double(0x0045_bba0, |_, _| Ret::default());
        r.e.register_double(0x004a_0bd0, |e, a| {
            for (k, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * k as u32, *v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let indexed = r.object(0x300);
        r.set(INDEXED_GLOBAL, &[indexed]);
        let (_, log) = r.run(0x0087_4c10, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011d_e9d1), 1);
        assert_eq!(calls_to(&log, 0x0050_7700), vec![vec![0xa000, word(0.75)]]);
        assert_eq!(calls_to(&log, 0x00a6_faf0), vec![vec![0xa000, 0x6200]]);
        assert_eq!(calls_to(&log, SET_COLOR)[0], vec![0xa000, 0x6300]);
        assert_eq!(calls_to(&log, 0x004f_00e0), vec![vec![0xa000, 0x6400]]);
        let ctor = calls_to(&log, POINT3_CTOR);
        assert_eq!(calls_to(&log, 0x00a5_9c60), vec![vec![0x3000, ctor[0][0]]]);
        assert_eq!(calls_to(&log, 0x0044_0460)[0], vec![0x3000, ZERO_VECTOR]);
        assert_eq!(calls_to(&log, 0x0054_6780), vec![vec![0x3000, 1]]);
        // The camera's position and the recorded vector reach the indexed
        // global's members.
        assert_eq!(r.e.mem.f32(indexed + 0x1f0), 4.0);
        assert_eq!(r.e.mem.f32(indexed + 0x1f8), 6.0);
        assert_eq!(r.e.mem.f32(indexed + 0x1e4), 1.0);
        assert_eq!(r.e.mem.f32(indexed + 0x1ec), 3.0);
        // The main object's script check blocks the body.
        r.set(GET_NODE_FLAG, &[0, 1]);
        r.e.set_global(MAIN_OBJECT, 0x7777u32);
        r.set(0x005b_b4d0, &[1]);
        r.e.set_global(0x011d_e9d1, 1u8);
        r.run(0x0087_4c10, &args![this]);
        assert_eq!(r.e.global::<u8>(0x011d_e9d1), 0);
    }

    #[test]
    fn menu_pass_draws_the_rendered_menu_scene_twice() {
        let (mut r, this) = scene_rig();
        let menu = r.object_with_slots(&[(8, 0x7a00)]);
        r.set(GET_CURRENT_RENDERED_MENU, &[menu]);
        r.set(GET_PIPBOY, &[0x1111]);
        let (_, log) = r.run(0x0087_5bf0, &args![this]);
        assert_eq!(calls_to(&log, ACCUMULATE_SCENE).len(), 2);
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE)[0],
            vec![0x6000, 0x7a00, 0x5000]
        );
        // 8b6200 finds no node, so the root's child is the node; with the
        // global 0x011f9426 clear the colour is built and given to it.
        assert_eq!(calls_to(&log, COLOR_CTOR).len(), 1);
        assert_eq!(calls_to(&log, SET_COLOR).len(), 1);
        assert_eq!(calls_to(&log, SET_COLOR)[0][0], 0x6000);
        assert_eq!(calls_to(&log, 0x004a_1020), vec![vec![0x9800, 0]]);
        assert_eq!(calls_to(&log, 0x00b4_f450), vec![vec![0xf], vec![0]]);
        assert_eq!(calls_to(&log, 0x00b9_8380), vec![vec![8, 1]]);
        assert_eq!(
            calls_to(&log, 0x00b6_c0d0),
            vec![vec![0x6000, 0x9800, 0], vec![0x6000, 0x9800, 0]]
        );
        assert_eq!(calls_to(&log, 0x00c4_f2d0), vec![vec![0x5000]]);
        // The renderer test: both vtable slots agree, so the fixed colour.
        let renderer = r.object_with_slots(&[(0xc8, 5), (0xcc, 5)]);
        r.set(GET_GLOBAL_011F4748, &[renderer]);
        r.e.set_global(0x011f_9426, 1u8);
        let (_, log) = r.run(0x0087_5bf0, &args![this]);
        assert!(calls_to(&log, COLOR_CTOR).is_empty());
        assert_eq!(
            calls_to(&log, SET_COLOR),
            vec![vec![0x6000, CONSTANT_COLOR]]
        );
        // The node of the menu (8b6200) is used when it exists.
        r.set(0x008b_6200, &[0x6600]);
        let (_, log) = r.run(0x0087_5bf0, &args![this]);
        assert_eq!(calls_to(&log, 0x0041_fd00), vec![vec![0x5000, 0x6600]]);
    }

    #[test]
    fn menu_pass_asks_the_menu_scene_without_a_stack_word() {
        let (mut r, this) = scene_rig();
        let menu = r.object_with_slots(&[(8, 0x7a00)]);
        r.set(GET_CURRENT_RENDERED_MENU, &[menu]);
        r.set(GET_PIPBOY, &[0x1111]);
        let slot = r.e.mem.u32(r.e.mem.u32(menu) + 8);
        let (_, log) = r.run(0x0087_5bf0, &args![this]);
        // 00875d6b / 00875dee: the target pushed before the menu's +8 (plain RET) is the
        // third word of the cdecl 00b6bee0 (ADD ESP,0xc), which gets it.
        assert_eq!(calls_to(&log, slot), vec![vec![menu]; 2]);
        assert_eq!(calls_to(&log, ACCUMULATE_SCENE)[0][2], 0x5000);
    }

    #[test]
    fn menu_pass_stops_early() {
        let (mut r, this) = scene_rig();
        // The offscreen target exists: nothing.
        r.e.mem.set_u32(0x011d_eb38, 0x5555);
        let (_, log) = r.run(0x0087_5bf0, &args![this]);
        assert!(addrs(&log).is_empty());
        r.e.mem.set_u32(0x011d_eb38, 0);
        // No rendered menu.
        let (_, log) = r.run(0x0087_5bf0, &args![this]);
        assert_eq!(addrs(&log), vec![GET_CURRENT_RENDERED_MENU]);
        // The rendered menu is the pipboy.
        r.set(GET_CURRENT_RENDERED_MENU, &[0x7000]);
        r.set(GET_PIPBOY, &[0x7000]);
        let (_, log) = r.run(0x0087_5bf0, &args![this]);
        assert_eq!(
            addrs(&log),
            vec![
                GET_CURRENT_RENDERED_MENU,
                GET_CURRENT_RENDERED_MENU,
                GET_PIPBOY
            ]
        );
    }

    #[test]
    fn temporary_pass_destroys_the_object_its_child_made() {
        let (mut r, this) = scene_rig();
        let made = r.object_with_slots(&[(0, 0)]);
        let child = r.object_with_slots(&[(0x48, made)]);
        r.set(GET_ROOT_CHILD, &[child]);
        r.set(0x0044_ddc0, &[4]);
        let (_, log) = r.run(0x0087_5e40, &args![this, 0x1111u32, 0x2222u32]);
        // The mode is 4: only the plain field of view is set.
        assert!(calls_to(&log, SET_CAMERA_FOV).is_empty());
        assert_eq!(calls_to(&log, 0x00b5_4000), vec![vec![word(2.5)]]);
        let temporary = calls_to(&log, 0x004a_d0c0)[0].clone();
        assert_eq!(temporary[1], 0x101);
        assert_eq!(
            calls_to(&log, slot_target(&r, child, 0x48)),
            vec![vec![child, temporary[0]]]
        );
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0][0];
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR),
            vec![vec![guard, 0x27, 1, SOURCE_FILE_NAME, 0x234c]]
        );
        assert_eq!(
            calls_to(&log, 0x00b6_5550),
            vec![vec![0x1111, child, made, 0x2222]]
        );
        assert_eq!(
            calls_to(&log, slot_target(&r, made, 0)),
            vec![vec![made, 1]]
        );
        assert_eq!(calls_to(&log, 0x004a_d1d0), vec![temporary[..1].to_vec()]);
        // Any other mode also sets the camera field of view, the second
        // time with the made object.
        r.set(0x0044_ddc0, &[3]);
        let (_, log) = r.run(0x0087_5e40, &args![this, 0x1111u32, 0x2222u32]);
        let fovs = calls_to(&log, SET_CAMERA_FOV);
        assert_eq!(fovs[0], vec![0x4000, word(2.5), 0, 0, 0]);
        assert_eq!(fovs[1], vec![0x4000, word(1.25), 0, made, 0]);
        // Without a made object nothing is destroyed.
        let child = r.object_with_slots(&[(0x48, 0)]);
        r.set(GET_ROOT_CHILD, &[child]);
        let (_, log) = r.run(0x0087_5e40, &args![this, 0x1111u32, 0x2222u32]);
        assert!(calls_to(&log, slot_target(&r, made, 0)).is_empty());
    }

    #[test]
    fn accumulator_forwarder_passes_the_root_child() {
        let (mut r, this) = scene_rig();
        let (_, log) = r.run(0x0087_5fa0, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(calls_to(&log, 0x00b6_51e0), vec![vec![0x11, 0x6000, 0x22]]);
    }

    #[test]
    fn main_scene_pass_releases_the_target_and_resets_the_colour() {
        let (mut r, this) = scene_rig();
        r.set(GET_ROOT_CHILD_OF_ROOT, &[0x6100]);
        let (_, log) = r.run(0x0087_5fd0, &args![this, 0u32, 0x10u32, 0x20u32, 0x30u32]);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0][0];
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR),
            vec![vec![guard, 0x27, 1, SOURCE_FILE_NAME, 0x2362]]
        );
        // 004e9510 is false: the state is prepared; no menu is showing: the
        // 00702870 call; no target pointer: no draw.
        assert_eq!(calls_to(&log, 0x00b6_b790).len(), 1);
        assert_eq!(calls_to(&log, 0x0070_2870).len(), 1);
        assert!(calls_to(&log, 0x0080_1a60).is_empty());
        assert_eq!(calls_to(&log, 0x00b5_5ac0), vec![vec![0x20, 0x10, 0x30]]);
        assert_eq!(r.e.global::<u32>(0x011f_917c), 0x6000);
        let ctor = calls_to(&log, COLOR_CTOR);
        assert_eq!(ctor[0][1..], [0, word(1.0), word(1.0), 0]);
        assert_eq!(calls_to(&log, SET_COLOR), vec![vec![0x6100, ctor[0][0]]]);
        // A menu showing stops the 00702870 call.
        r.set(IS_IN_MENU_MODE, &[1]);
        let (_, log) = r.run(0x0087_5fd0, &args![this, 0u32, 0x10u32, 0x20u32, 0x30u32]);
        assert!(calls_to(&log, 0x0070_2870).is_empty());
        // The target pointer set: it is drawn and released.
        let interface = r.object(0x100);
        r.set(GET_INTERFACE_OBJECT, &[interface]);
        r.set(GET_GLOBAL_011F91A8, &[0x1600]);
        r.e.mem.set_u32(0x011d_eb38, 0x7777);
        r.set(0x004e_9510, &[1]);
        let (_, log) = r.run(0x0087_5fd0, &args![this, 0u32, 0x10u32, 0x20u32, 0x30u32]);
        assert_eq!(calls_to(&log, 0x00b6_b790).len(), 0);
        assert_eq!(
            calls_to(&log, 0x0080_1a60),
            vec![vec![interface, 0x10, 0x7777, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x0080_19d0), vec![vec![interface]]);
        assert_eq!(calls_to(&log, 0x00b6_da10), vec![vec![0x1600, 0x7777]]);
        assert_eq!(r.e.mem.u32(0x011d_eb38), 0);
    }

    #[test]
    fn main_scene_pass_scales_the_scene_for_a_rendered_menu() {
        let (mut r, this) = scene_rig();
        let menu = r.object_with_slots(&[(0x2c, 0)]);
        // The vtable slot answers in ST0.
        let target = slot_target(&r, menu, 0x2c);
        r.e.register_double(target, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        r.set(GET_CURRENT_RENDERED_MENU, &[menu]);
        r.set(IS_IN_RENDERED_MENU, &[1]);
        r.e.set_global(0x011d_ea29, 1u8);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let seen_in = seen.clone();
        r.e.register_double(0x00b5_5ac0, move |e, _| {
            seen_in
                .borrow_mut()
                .push((e.global::<u8>(0x0120_05a2), e.global::<f32>(0x011a_dd08)));
            Ret::default()
        });
        r.run(0x0087_5fd0, &args![this, 0u32, 0x10u32, 0x20u32, 0x30u32]);
        assert_eq!(*seen.borrow(), vec![(1, 0.5)]);
        // Afterwards the globals are back.
        assert_eq!(r.e.global::<u8>(0x0120_05a2), 0);
        assert_eq!(r.e.global::<f32>(0x011a_dd08), 1.0);
    }

    #[test]
    fn frame_pass_without_menus_sets_the_fov_and_queues_the_stage() {
        let (mut r, this) = scene_rig();
        r.set(MEMBER_8C_B, &[0x6200]);
        r.set(GET_MT_SYSTEM, &[0x1234]);
        let (_, log) = r.run(0x0087_5110, &args![this, 0x1000u32, 0u32, 0u32, 0x40u32]);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0][0];
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR),
            vec![vec![guard, 0x34, 1, SOURCE_FILE_NAME, 0x221f]]
        );
        assert_eq!(calls_to(&log, 0x0071_48c0), vec![vec![0x1000, 0, 4]]);
        // The first-person accumulator's byte +0x3a is cleared.
        assert_eq!(r.e.mem.u8(0x8c00 + 0x3a), 0);
        // With the renderer check false, a colour is built for the camera.
        let ctor = calls_to(&log, COLOR_CTOR);
        assert_eq!(ctor[0][1..], [0, word(1.0), word(1.0), 0]);
        assert_eq!(calls_to(&log, SET_COLOR), vec![vec![0xa000, ctor[0][0]]]);
        assert_eq!(calls_to(&log, 0x00a6_faf0), vec![vec![0xa000, 0x6200]]);
        // Stage 0x16 on thread 1 twice (start and the 1st-person block).
        assert_eq!(
            calls_to(&log, MT_ADVANCE_THREAD_STAGE),
            vec![vec![0x1234, 1, 0x16]; 2]
        );
        assert_eq!(calls_to(&log, 0x00b9_8380), vec![vec![0xf, 1]]);
        assert_eq!(calls_to(&log, 0x00b6_c0d0), vec![vec![0xa000, 0x8c00, 0]]);
        // The three field-of-view calls: the 1st-person one, the mode one
        // and the final one.
        assert_eq!(
            calls_to(&log, SET_CAMERA_FOV),
            vec![
                vec![0x4000, word(1.25), 0, 0, 0],
                vec![0x4000, word(2.5), 0, 0, 0],
                vec![0x4000, word(1.25), 0, 0, 0]
            ]
        );
        assert_eq!(calls_to(&log, PROFILE_SCOPE_DTOR), vec![vec![guard]]);
        // No nested scope without 005b9b00.
        assert_eq!(calls_to(&log, PROFILE_SCOPE_CTOR).len(), 1);
    }

    #[test]
    fn frame_pass_draws_the_rendered_menu_and_the_pipboy_arrays() {
        let (mut r, this) = scene_rig();
        let menu = r.object(0x40);
        r.set(GET_CURRENT_RENDERED_MENU, &[menu]);
        let pipboy = r.object(0x200);
        for k in 0..3 {
            r.e.mem.set_u32(pipboy + 0xf4 + 4 * k, 0x7a00 + k);
        }
        r.set(GET_PIPBOY, &[pipboy]);
        r.set(IS_IN_RENDERED_MENU, &[1]);
        r.set(IS_IN_PIPBOY_MENU, &[1]);
        r.set(ACTOR_FLAG, &[1]);
        r.set(GET_MENU_SCREEN, &[0x7100]);
        r.set(0x00aa_13e0, &[0x2800]);
        r.set(0x00b6_60d0, &[0x2900]);
        r.set(0x00a5_9d30, &[0x2a00]);
        let (_, log) = r.run(
            0x0087_5110,
            &args![this, 0x1000u32, 0x6500u32, 0u32, 0x40u32],
        );
        assert_eq!(
            calls_to(&log, SET_SCREEN_TEXTURE),
            vec![vec![menu, 0], vec![menu, 0]]
        );
        assert_eq!(calls_to(&log, SET_NODE_FLAG)[..1], [vec![0x3000, 0]]);
        assert_eq!(
            calls_to(&log, 0x00b6_60d0),
            vec![vec![0x2800, 0x63, 1, 0x2f7]]
        );
        assert_eq!(calls_to(&log, 0x00c4_f270), vec![vec![0x5000, 1]]);
        assert_eq!(calls_to(&log, 0x00a5_9d30), vec![vec![0x7100, 0]]);
        assert_eq!(calls_to(&log, 0x0049_ed90), vec![vec![0x2a00, 0]]);
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![
                vec![0xa000, 0x7a00, 0x5000],
                vec![0xa000, 0x7a01, 0x5000],
                vec![0xa000, 0x7a02, 0x5000],
                vec![0xa000, 0x7100, 0x5000]
            ]
        );
        // The accumulator pointer built from 0x2900 is dropped at the end,
        // and the pass for another rendered menu is skipped.
        assert_eq!(calls_to(&log, NI_POINTER_DTOR).len(), 1);
        assert_eq!(calls_to(&log, 0x00b6_c0d0).len(), 1);
        assert_eq!(calls_to(&log, 0x00c4_f2d0), vec![vec![0x5000]]);
        // The pipboy screen's flag is raised first and cleared at the end.
        assert_eq!(
            calls_to(&log, SET_NODE_FLAG),
            vec![
                vec![0x3000, 0],
                vec![0x6500, 0],
                vec![0x7100, 1],
                vec![0x7100, 0]
            ]
        );
    }

    #[test]
    fn frame_pass_accumulates_the_target_texture_in_a_nested_scope() {
        let (mut r, this) = scene_rig();
        r.set(0x005b_9b00, &[1]);
        r.set(0x0043_b230, &[0xb000]);
        let (_, log) = r.run(
            0x0087_5110,
            &args![this, 0x1000u32, 0u32, 0x9999u32, 0x40u32],
        );
        let scopes = calls_to(&log, PROFILE_SCOPE_CTOR);
        assert_eq!(scopes.len(), 2);
        assert_eq!(scopes[1][1..], [0x27, 1, SOURCE_FILE_NAME, 0x22b5]);
        assert_eq!(
            calls_to(&log, 0x00b6_4570),
            vec![vec![0x8c00, 0xa000, 0x40]]
        );
        assert_eq!(calls_to(&log, PROFILE_SCOPE_DTOR).len(), 2);
        // The overlay node: its holder is accumulated and its flag restored.
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![vec![0x6000, 0xb000, 0x5000]]
        );
        assert_eq!(
            calls_to(&log, SET_NODE_FLAG),
            vec![vec![0x3000, 0], vec![0xb000, 0], vec![0xb000, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0064_26a0),
            vec![vec![0x9999, 2, 0x8800, 0x6000]]
        );
        // A set flag on the holder skips the overlay accumulation.
        r.set(GET_NODE_FLAG, &[1]);
        let (_, log) = r.run(
            0x0087_5110,
            &args![this, 0x1000u32, 0u32, 0x9999u32, 0x40u32],
        );
        assert!(calls_to(&log, ACCUMULATE_SCENE).is_empty());
        assert_eq!(calls_to(&log, 0x0064_26a0).len(), 1);
    }

    #[test]
    fn frame_pass_walks_the_node_list_of_the_first_person_block() {
        let (mut r, this) = scene_rig();
        let block = r.object(0x40);
        r.set(MEMBER_8C, &[block]);
        let entry = r.object_with_slots(&[(0xc0, 1), (0x98, 0x7c00)]);
        let list = r.object(0x40);
        r.set(0x0043_c490, &[list]);
        r.set(0x006815c0, &[list]);
        r.e.mem.set_u32(list, entry);
        r.set(0x0072_6070, &[0]);
        r.set(GET_ROOT_CHILD, &[0x6000]);
        r.set(0x0044_ddc0, &[4]);
        let (_, log) = r.run(0x0087_5110, &args![this, 0x1000u32, 0u32, 0u32, 0x40u32]);
        // One flagged entry: its screen is drawn between flag changes.
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![vec![0x6000, 0x7c00, 0x5000]]
        );
        let flags = calls_to(&log, SET_NODE_FLAG);
        assert_eq!(flags[1], vec![0x7c00, 0]);
        assert_eq!(flags[2], vec![0x7c00, 1]);
        // `00b6c0d0` runs twice: the block's own and the one after the list.
        assert_eq!(calls_to(&log, 0x00b6_c0d0).len(), 2);
        // An entry whose flag slot answers 0 is skipped.
        let entry = r.object_with_slots(&[(0xc0, 0), (0x98, 0x7c00)]);
        r.e.mem.set_u32(list, entry);
        let (_, log) = r.run(0x0087_5110, &args![this, 0x1000u32, 0u32, 0u32, 0x40u32]);
        assert!(calls_to(&log, ACCUMULATE_SCENE).is_empty());
        assert_eq!(calls_to(&log, 0x00b6_c0d0).len(), 1);
    }

    #[test]
    fn menu_background_pass_resets_the_view_without_the_pipboy() {
        let (mut r, this) = scene_rig();
        r.set(GET_CURRENT_RENDERED_MENU, &[0x7000]);
        r.set(0x0055_85e0, &[0x7200]);
        r.set(GET_MENU_SCREEN, &[0x7300]);
        r.set(MEMBER_8C, &[0x7400]);
        let block = r.object(0x40);
        r.set(0x004a_0d10, &[block]);
        for k in 0..4 {
            r.e.mem.set_u32(block + 4 * k, 0x100 + k);
        }
        r.e.register_double(0x004a_0bd0, |e, a| {
            e.mem.set_u32(a[1], 0xaa);
            e.mem.set_u32(a[1] + 4, 0xbb);
            e.mem.set_u32(a[1] + 8, 0xcc);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let indexed = r.object(0x300);
        r.set(INDEXED_GLOBAL, &[indexed]);
        let (_, log) = r.run(0x0087_61e0, &args![this, 0u32, 0x1000u32]);
        assert_eq!(calls_to(&log, 0x0071_48c0), vec![vec![0x1000, 0, 4]]);
        assert_eq!(
            calls_to(&log, 0x004f_b0b0),
            vec![vec![0x5000, 1], vec![0x5000, 0]]
        );
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![vec![0x6000, 0x7200, 0x5000], vec![0x6000, 0x7300, 0x5000]]
        );
        assert_eq!(calls_to(&log, 0x00b6_3b90), vec![vec![0x8c00, 6]]);
        assert_eq!(calls_to(&log, 0x004a_1020), vec![vec![0x8c00, indexed]]);
        // The colour read from the scene is given back at the end.
        let colors = calls_to(&log, SET_COLOR);
        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0][0], 0x6000);
        // Without the Pip-Boy the menu screen's position does not reach the scene node's
        // offset (00876295 jumps over 00876371): the race menu's face lights read 0 there.
        assert_eq!(r.e.mem.u32(indexed + 0x1e4), 0);
        assert_eq!(r.e.mem.u32(indexed + 0x1ec), 0);
        assert!(calls_to(&log, 0x004a_0bd0).is_empty());
        // The pipboy-only parts did not run.
        assert!(calls_to(&log, 0x007f_8720).is_empty());
        assert!(calls_to(&log, 0x0044_0460).is_empty());
        assert_eq!(calls_to(&log, 0x00b9_7e30), vec![vec![0, 1]]);
        assert_eq!(calls_to(&log, 0x0071_4a60), vec![vec![1]]);
    }

    #[test]
    fn menu_background_pass_restores_the_pipboy_transform() {
        let (mut r, this) = scene_rig();
        let color = r.object(0x40);
        r.set(0x004a_0d10, &[color]);
        let pipboy = r.object(0x200);
        r.set(GET_CURRENT_RENDERED_MENU, &[0x7000]);
        r.set(GET_PIPBOY, &[pipboy]);
        r.set(0x0055_85e0, &[0x7200]);
        r.set(GET_MENU_SCREEN, &[0x7300]);
        r.set(IS_IN_PIPBOY_MENU, &[1]);
        r.set(ACTOR_FLAG, &[1]);
        r.set(0x0096_7ae0, &[1]);
        let transform = r.object(0x40);
        r.set(0x006a_9540, &[transform]);
        r.set(0x0096_11e0, &[0x7500]);
        r.set(0x0087_7720, &[0x7600]);
        r.set(0x00a2_3a50, &[5, 0]);
        r.set(0x007f_8720, &[3]);
        r.e.register_double(0x007f_8720, |e, a| {
            e.mem.set_f32(a[3], 1.5);
            e.mem.set_f32(a[4], 2.5);
            e.mem.set_u32(a[5], 9);
            Ret {
                eax: 3,
                ..Ret::default()
            }
        });
        let indexed = r.object(0x300);
        r.set(INDEXED_GLOBAL, &[indexed]);
        r.e.set_global(ZERO_VECTOR, 0x41u32);
        r.e.register_double(0x004a_0bd0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let (_, log) = r.run(0x0087_61e0, &args![this, 0u32, 0x1000u32]);
        // The menu's two reads of the controls: stick up/down answers.
        assert_eq!(
            calls_to(&log, 0x00a2_3a50),
            vec![vec![0x7600, 0, 2], vec![0x7600, 0, 1]]
        );
        let sticks = calls_to(&log, 0x007f_8720);
        assert_eq!(sticks[0][..4], [pipboy, 0, 1, sticks[0][3]]);
        assert_eq!(sticks[0][4], sticks[0][3] + 4);
        assert_eq!(sticks[0][5], sticks[0][3] + 8);
        assert_eq!(
            calls_to(&log, 0x0070_9b50),
            vec![vec![3, word(1.5), word(2.5), 9]]
        );
        // The screen's translation is set to the zero vector, to its saved
        // copy, then restored to the identity-reset copy.
        let moves = calls_to(&log, 0x0044_0460);
        assert_eq!(moves[0], vec![0x7200, ZERO_VECTOR]);
        assert_eq!(moves[1][0], 0x7200);
        assert_eq!(moves.last().unwrap()[0], 0x7500);
        assert_eq!(moves.last().unwrap()[1], moves[1][1] + 0x34);
        // The pipboy arrays are accumulated.
        assert_eq!(calls_to(&log, ACCUMULATE_SCENE).len(), 5);
        assert_eq!(calls_to(&log, 0x00b9_7de0), vec![vec![0, 1]]);
        assert_eq!(calls_to(&log, 0x0071_4a40), vec![vec![1]]);
        // The last recorded position is the zero vector.
        assert_eq!(r.e.mem.u32(indexed + 0x1e4), 0x41);
    }

    /// A scene whose pointer lists, tree and `NiTPointerList` calls are
    /// doubles: `nodes` answer `00450b80` / `0045bc00` / `0043b480` /
    /// `0043b4a0` and their vtable slot `0xc`; the list constructor and
    /// `AddTail` record the lists and what was pushed.
    #[derive(Default)]
    struct Tree {
        slots: HashMap<(u32, u32), u32>,
        kids: HashMap<u32, Vec<u32>>,
        groups: HashMap<u32, u32>,
        lists: Vec<u32>,
        pushes: Vec<(u32, u32)>,
    }

    /// Installs the tree doubles; returns the shared tree and the shared
    /// vtable's object factory.
    fn install_tree(r: &mut Rig) -> Rc<RefCell<Tree>> {
        let tree = Rc::new(RefCell::new(Tree::default()));
        let t = tree.clone();
        r.e.register_double(CHILD_FOR_SLOT, move |_, a| Ret {
            eax: t.borrow().slots.get(&(a[0], a[1])).copied().unwrap_or(0),
            ..Ret::default()
        });
        let t = tree.clone();
        r.e.register_double(CHILD_COUNT, move |_, a| Ret {
            eax: t.borrow().kids.get(&a[0]).map_or(0, |k| k.len() as u32),
            ..Ret::default()
        });
        let t = tree.clone();
        r.e.register_double(CHILD_AT, move |_, a| Ret {
            eax: t.borrow().kids.get(&a[0]).map_or(0, |k| k[a[1] as usize]),
            ..Ret::default()
        });
        let t = tree.clone();
        r.e.register_double(LIST_CTOR, move |_, a| {
            t.borrow_mut().lists.push(a[0]);
            Ret::default()
        });
        let t = tree.clone();
        r.e.register_double(LIST_PUSH, move |e, a| {
            let value = e.mem.u32(a[1]);
            t.borrow_mut().pushes.push((a[0], value));
            Ret::default()
        });
        let t = tree.clone();
        r.e.register_double(0x00f2_000c, move |_, a| Ret {
            eax: t.borrow().groups.get(&a[0]).copied().unwrap_or(0),
            ..Ret::default()
        });
        tree
    }

    /// A node object whose vtable slot `0xc` is the tree's group lookup.
    fn tree_node(r: &mut Rig) -> u32 {
        let node = r.object(0x40);
        let vtable = r.object(0x40);
        r.e.mem.set_u32(node, vtable);
        r.e.mem.set_u32(vtable + 0xc, 0x00f2_000c);
        node
    }

    /// The values pushed to the list `list` (by position in the
    /// constructor order), in order.
    fn pushed(tree: &Tree, list: usize) -> Vec<u32> {
        tree.pushes
            .iter()
            .filter(|(l, _)| *l == tree.lists[list])
            .map(|(_, v)| *v)
            .collect()
    }

    #[test]
    fn world_pass_accumulates_the_scene_directly() {
        let (mut r, this) = scene_rig();
        let texture = r.object(0x100);
        r.e.mem.set_f32(texture + 0x70, 5.0);
        r.e.set_global(0x0101_2060, 1.0f64);
        r.set(GET_GLOBAL_011F91AC, &[0x2000]);
        r.set(0x004e_bbc0, &[texture]);
        r.set(GET_NODE_FLAG, &[1]);
        r.set(GET_MT_SYSTEM, &[0x1234]);
        r.set(MEMBER_8C_B, &[0x6200]);
        r.e.mem.set_u32(0x011c_7840, 0x11);
        r.e.mem.set_u32(0x011c_7810, 0x22);
        r.e.mem.set_u32(0x011c_781c, 0x33);
        r.set(0x005b_ac50, &[0x44]);
        // Snapshot the accumulator bytes at the first accumulation.
        let seen = Rc::new(RefCell::new(Vec::new()));
        let seen_in = seen.clone();
        r.e.register_double(ACCUMULATE_SCENE, move |e, _| {
            seen_in.borrow_mut().push((
                e.mem.u8(0x8800 + 0x38),
                e.mem.u8(0x8800 + 0x3a),
                e.mem.u8(0x8c00 + 0x3a),
            ));
            Ret::default()
        });
        r.e.set_global(0x011f_9fc3, 9u8);
        let (_, log) = r.run(0x0087_3200, &args![this, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(seen.borrow()[0], (1, 1, 1));
        assert_eq!(r.e.global::<u32>(0x011f_9684), 0x6000);
        assert_eq!(r.e.mem.u8(0x8800 + 0x38), 0);
        assert_eq!(r.e.mem.u8(0x8800 + 0x3a), 0);
        assert_eq!(r.e.global::<u8>(0x011f_9fc3), 0);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0][0];
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR),
            vec![vec![guard, 0xf, 1, SOURCE_FILE_NAME, 0x1eb7]]
        );
        assert_eq!(calls_to(&log, 0x0040_4f70), vec![vec![guard]]);
        assert_eq!(calls_to(&log, PROFILE_SCOPE_DTOR), vec![vec![guard]]);
        assert_eq!(calls_to(&log, 0x0041_fd00)[0], vec![0x5000, 0x6000]);
        assert_eq!(calls_to(&log, 0x00a6_94a0), vec![vec![0x5000, 0x6200]]);
        // The three accumulations: the world, and the two extra passes.
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE),
            vec![
                vec![0x6000, 0x4000, 0x5000],
                vec![0x11, 0x44, 0x5000],
                vec![0x33, 0x22, 0x5000]
            ]
        );
        assert_eq!(
            calls_to(&log, 0x00b6_c0d0),
            vec![vec![0x11, 0x9400, 0], vec![0x33, 0x9400, 0]]
        );
        assert_eq!(calls_to(&log, 0x00b6_ba20), vec![vec![0x6000, 0x8800, 1]]);
        assert_eq!(calls_to(&log, 0x00b6_b930), vec![vec![0x6000, 0x8800, 1]]);
        assert_eq!(calls_to(&log, 0x00b8_03b0), vec![vec![0x6000]; 2]);
        // The player node's flag is set during the pass and restored.
        assert_eq!(
            calls_to(&log, SET_NODE_FLAG),
            vec![vec![0x3000, 1], vec![0x3000, 1]]
        );
        // No overlay, no object, one thread.
        assert!(calls_to(&log, DRAW_TARGET_TO_TEXTURE).is_empty());
        assert!(calls_to(&log, 0x0064_26a0).is_empty());
        assert!(calls_to(&log, 0x008c_80e0).is_empty());
    }

    #[test]
    fn world_pass_options_add_the_overlay_the_object_and_the_thread_call() {
        let (mut r, this) = scene_rig();
        let interface = r.object(0x100);
        r.set(GET_INTERFACE_OBJECT, &[interface]);
        r.settings(SETTING_DWORD_PTR, &[(SETTING_THREADS, 2)]);
        let (_, log) = r.run(0x0087_3200, &args![this, 0x9100u32, 0u32, 1u32, 1u32]);
        // Skipping clears the accumulator flag.
        assert_eq!(calls_to(&log, 0x0064_2860), vec![vec![0x9100]]);
        assert_eq!(
            calls_to(&log, DRAW_TARGET_TO_TEXTURE),
            vec![vec![interface, 0, 2, 1, 0, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x00b9_8380), vec![vec![0, 1]]);
        assert_eq!(calls_to(&log, 0x008c_80e0), vec![vec![0]]);
        assert_eq!(
            calls_to(&log, 0x0064_26a0),
            vec![vec![0x9100, 1, 0x8800, 0x6000]]
        );
    }

    #[test]
    fn world_pass_skip_flag_clears_the_accumulator_bytes() {
        let (mut r, this) = scene_rig();
        let texture = r.object(0x100);
        r.e.mem.set_f32(texture + 0x70, 5.0);
        r.e.set_global(0x0101_2060, 1.0f64);
        r.set(0x004e_bbc0, &[texture]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let seen_in = seen.clone();
        r.e.register_double(ACCUMULATE_SCENE, move |e, _| {
            seen_in.borrow_mut().push((
                e.mem.u8(0x8800 + 0x38),
                e.mem.u8(0x8800 + 0x3a),
                e.mem.u8(0x8c00 + 0x3a),
            ));
            Ret::default()
        });
        r.run(0x0087_3200, &args![this, 0u32, 0u32, 1u32, 0u32]);
        // `skip_pass` set: +0x38 is 0 (the pass is drawn), +0x3a stays 0.
        assert_eq!(seen.borrow()[0], (0, 0, 0));
    }

    /// The rig of the scene sorting: the global `0x011f94b4` is set, the
    /// indexed global is an object whose children come from the tree.
    fn sorting_rig() -> (Rig, u32, u32, Rc<RefCell<Tree>>) {
        let (mut r, this) = scene_rig();
        r.e.set_global(0x011f_94b4, 1u8);
        let indexed = r.object(0x300);
        r.set(INDEXED_GLOBAL, &[indexed]);
        r.set(GET_MT_SYSTEM, &[0x1234]);
        let tree = install_tree(&mut r);
        // The world accumulator has a vtable (slot 0x8c is called).
        let vtable = r.object(0x100);
        r.e.mem.set_u32(0x8800, vtable);
        r.e.mem.set_u32(vtable + 0x8c, 0x00f2_008c);
        r.e.register_double(0x00f2_008c, |_, _| Ret::default());
        (r, this, indexed, tree)
    }

    #[test]
    fn exterior_scene_sorting_splits_the_children_by_parity() {
        let (mut r, this, indexed, tree) = sorting_rig();
        let n: Vec<u32> = (0..7).map(|_| tree_node(&mut r)).collect();
        let o0 = tree_node(&mut r);
        let o1 = tree_node(&mut r);
        let i0 = tree_node(&mut r);
        let i1 = tree_node(&mut r);
        let i2 = tree_node(&mut r);
        {
            let mut t = tree.borrow_mut();
            for (k, node) in n.iter().enumerate() {
                t.slots.insert((indexed, k as u32), *node);
            }
            // Slot 2 holds a group of three children.
            t.groups.insert(n[2], 0x9100);
            for k in 0..3u32 {
                t.slots.insert((0x9100, k), 0x9101 + k);
            }
            // Slot 3 is the exterior tree: H -> O0, O1 -> M0, M1 -> items.
            t.groups.insert(n[3], 0x9200);
            t.kids.insert(0x9200, vec![o0, o1]);
            t.groups.insert(o0, 0x9300);
            t.groups.insert(o1, 0x9400);
            t.kids.insert(0x9300, vec![i0]);
            t.kids.insert(0x9400, vec![i1, i2]);
            t.groups.insert(i2, 0x9500);
            t.kids.insert(0x9500, vec![0x9501]);
            // The tail of the indexed global's children: only 7 counts.
            t.kids.insert(indexed, (0..8).map(|k| 0x9600 + k).collect());
        }
        let system = 0x1234;
        let tls = r.e.tls();
        r.e.mem.set_u32(tls + 0x2bc, 7);
        let snapshots = Rc::new(RefCell::new(Vec::new()));
        let snapshots_in = snapshots.clone();
        r.e.register_double(ACCUMULATE_SCENE_LIST, move |e, a| {
            snapshots_in
                .borrow_mut()
                .push((a[1], e.mem.u32(a[2] + 0xc0)));
            Ret::default()
        });
        let (_, log) = r.run(0x0087_3200, &args![this, 0u32, 0u32, 0u32, 0u32]);
        let tree = tree.borrow();
        assert_eq!(tree.lists.len(), 3);
        assert_eq!(pushed(&tree, 2), vec![n[0], n[1], n[4], n[5]]);
        assert_eq!(
            pushed(&tree, 0),
            vec![0x9101, 0x9102, 0x9103, i1, 0x9501, n[6], 0x9607]
        );
        assert_eq!(pushed(&tree, 1), vec![i0]);
        // Lists a and c go to the task, b and c to the scene lists with the
        // culling process; `+0xc0` of the process is the operation list.
        let ops = indexed + 0x138;
        let child = 0x6000;
        let culling = calls_to(&log, 0x004a_0eb0)[0][0];
        assert_eq!(calls_to(&log, 0x004a_0eb0), vec![vec![culling, 0]]);
        assert_eq!(
            calls_to(&log, ACCUMULATE_SCENE_LIST),
            vec![
                vec![child, tree.lists[1], culling],
                vec![child, tree.lists[2], culling]
            ]
        );
        assert_eq!(
            *snapshots.borrow(),
            vec![(tree.lists[1], ops), (tree.lists[2], 0)]
        );
        assert_eq!(
            calls_to(&log, ADD_ACCUM_TASK),
            vec![vec![
                system,
                child,
                ops,
                0,
                tree.lists[0],
                0,
                0x8800,
                0,
                0x15,
                1
            ]]
        );
        assert_eq!(
            calls_to(&log, MT_SET_THREAD_STAGE),
            vec![vec![system, 0, 0x15]]
        );
        assert_eq!(
            calls_to(&log, MT_ADVANCE_THREAD_STAGE),
            vec![vec![system, 1, 0x15]]
        );
        assert_eq!(r.e.mem.u32(tls + 0x2bc), 0);
        // The lists are cleared (a, b) and destroyed (c, b, a); the process
        // is destroyed.
        assert_eq!(
            calls_to(&log, LIST_REMOVE_ALL),
            vec![vec![tree.lists[0]], vec![tree.lists[1]]]
        );
        assert_eq!(
            calls_to(&log, LIST_DTOR),
            vec![
                vec![tree.lists[2]],
                vec![tree.lists[1]],
                vec![tree.lists[0]]
            ]
        );
        assert_eq!(calls_to(&log, 0x004a_0f60), vec![vec![culling]]);
        // The compound frustum is built from the operations.
        assert_eq!(calls_to(&log, 0x00c4_9850), vec![vec![ops]]);
        assert_eq!(calls_to(&log, 0x00b5_e870), vec![vec![indexed, 0x5000]]);
        // Unfinished accumulation: the final scene step of the world pass.
        assert_eq!(calls_to(&log, 0x00a8_9af0), vec![vec![indexed, 0x5000]]);
        assert_eq!(calls_to(&log, ACCUMULATE_SCENE).len(), 2);
        assert_eq!(calls_to(&log, 0x00f2_008c), vec![vec![0x8800, child]]);
    }

    #[test]
    fn interior_scene_sorting_without_the_interior_byte_walks_two_groups() {
        let (mut r, this, indexed, tree) = sorting_rig();
        r.set(0x005f_36f0, &[1]);
        let n3 = tree_node(&mut r);
        let q = tree_node(&mut r);
        let s = tree_node(&mut r);
        {
            let mut t = tree.borrow_mut();
            t.slots.insert((indexed, 3), n3);
            t.groups.insert(n3, 0xa100);
            t.slots.insert((0xa100, 2), q);
            t.groups.insert(q, 0xa200);
            t.slots.insert((0xa200, 7), s);
            t.groups.insert(s, 0xa300);
            t.kids.insert(0xa300, vec![0xa301, 0xa302, 0xa303]);
        }
        r.run(0x0087_3200, &args![this, 0u32, 0u32, 0u32, 0u32]);
        let tree = tree.borrow();
        assert_eq!(pushed(&tree, 0), vec![0xa302]);
        assert_eq!(pushed(&tree, 1), vec![0xa301, 0xa303]);
        assert!(pushed(&tree, 2).is_empty());
    }

    #[test]
    fn interior_scene_sorting_with_the_interior_byte_takes_the_first_three_groups() {
        let (mut r, this, indexed, tree) = sorting_rig();
        r.set(0x005f_36f0, &[1]);
        r.e.mem.set_u8(indexed + 0x1dc, 1);
        let n3 = tree_node(&mut r);
        let z = tree_node(&mut r);
        let v1 = tree_node(&mut r);
        let v6 = tree_node(&mut r);
        let v4 = tree_node(&mut r);
        let others: Vec<u32> = (0..4).map(|_| tree_node(&mut r)).collect();
        let owner = r.object(0x40);
        let list = r.object(0x40);
        r.set(0x0045_4b30, &[owner]);
        r.set(0x007d_6bb0, &[list]);
        {
            let mut t = tree.borrow_mut();
            t.slots.insert((indexed, 3), n3);
            t.groups.insert(n3, 0xa100);
            t.slots.insert((0xa100, 0), 0xa101);
            t.slots.insert((0xa100, 1), 0xa102);
            t.slots.insert((0xa100, 2), z);
            t.groups.insert(z, 0xa200);
            // Positions 0 and 4 are skipped; 1 and 6 have members.
            t.kids.insert(
                0xa200,
                vec![others[0], v1, others[1], others[2], v4, others[3], v6],
            );
            t.groups.insert(v1, 0xa300);
            t.kids.insert(0xa300, vec![0xa301, 0xa302]);
            t.groups.insert(v4, 0xa400);
            t.kids.insert(0xa400, vec![0xa401]);
            t.groups.insert(v6, 0xa500);
            t.kids.insert(0xa500, vec![0xa501, 0xa502, 0xa503]);
        }
        r.run(0x0087_3200, &args![this, 0u32, 0u32, 0u32, 0u32]);
        let tree = tree.borrow();
        // The two direct children, then the members by their own index
        // (the node at position 6 starts at member 1).
        assert_eq!(pushed(&tree, 1), vec![0xa101, 0xa301, 0xa503]);
        assert_eq!(pushed(&tree, 0), vec![0xa102, 0xa302, 0xa502]);
    }

    #[test]
    fn interior_scene_sorting_adds_the_geometry_the_culler_accepts() {
        let (mut r, this, indexed, tree) = sorting_rig();
        r.set(0x005f_36f0, &[1]);
        let n3 = tree_node(&mut r);
        tree.borrow_mut().slots.insert((indexed, 3), n3);
        let item = r.object(0x200);
        let owner = r.object(0x40);
        let list_head = r.object(0x40);
        let holder_slot = r.object(0x40);
        let item_slot = r.object(0x40);
        let members = r.object(0x40);
        r.e.mem.set_u32(members, 0x55);
        r.e.mem.set_u32(list_head, 0x66);
        r.e.mem.set_u32(holder_slot, 0x7a7a);
        r.e.mem.set_u32(item_slot, item);
        r.set(0x0045_4b30, &[owner]);
        r.set(0x0043_6aa0, &[list_head]);
        r.set(0x006a_b360, &[members]);
        r.set(0x0045_c4c0, &[0x5a5a]);
        r.set(0x00c4_f320, &[1]);
        // Outer, then inner iteration: each step ends the loop.
        let steps = Rc::new(RefCell::new(vec![holder_slot, item_slot]));
        r.e.register_double(0x0057_cbe0, move |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret {
                eax: steps.borrow_mut().remove(0),
                ..Ret::default()
            }
        });
        let (_, log) = r.run(0x0087_3200, &args![this, 0u32, 0u32, 0u32, 0u32]);
        let tree = tree.borrow();
        assert_eq!(pushed(&tree, 1), vec![0x5a5a]);
        // The culler was asked about the geometry's owner with the target.
        assert_eq!(calls_to(&log, 0x00c4_f320), vec![vec![0x5000, 0x5a5a]]);
        // The item's `+0xfc` is clear: no shared compound frustum.
        assert!(calls_to(&log, 0x00b5_ba80).is_empty());
    }

    #[test]
    fn interior_scene_sorting_builds_a_shared_frustum_for_marked_geometry() {
        let (mut r, this, indexed, tree) = sorting_rig();
        r.set(0x005f_36f0, &[1]);
        let n3 = tree_node(&mut r);
        tree.borrow_mut().slots.insert((indexed, 3), n3);
        let item = r.object(0x200);
        r.e.mem.set_u32(item + 0xfc, 3);
        let owner = r.object(0x40);
        let list_head = r.object(0x40);
        let holder_slot = r.object(0x40);
        let item_slot = r.object(0x40);
        let members = r.object(0x40);
        r.e.mem.set_u32(members, 0x55);
        r.e.mem.set_u32(list_head, 0x66);
        r.e.mem.set_u32(holder_slot, 0x7a7a);
        r.e.mem.set_u32(item_slot, item);
        r.set(0x0045_4b30, &[owner]);
        r.set(0x0043_6aa0, &[list_head]);
        r.set(0x006a_b360, &[members]);
        r.set(0x0045_c4c0, &[0x5a5a]);
        r.set(0x00c4_f320, &[1]);
        r.set(0x009b_4440, &[1]);
        r.set(0x0084_e3a0, &[0x8e8e]);
        let steps = Rc::new(RefCell::new(vec![holder_slot, item_slot]));
        r.e.register_double(0x0057_cbe0, move |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret {
                eax: steps.borrow_mut().remove(0),
                ..Ret::default()
            }
        });
        let (_, log) = r.run(0x0087_3200, &args![this, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls_to(&log, 0x00b5_ba80),
            vec![vec![indexed, 0x8e8e, item]]
        );
        assert_eq!(calls_to(&log, 0x0084_e3a0), vec![vec![0x5000]]);
    }

    #[test]
    fn small_forwarders_call_their_targets() {
        let mut r = Rig::new();
        r.set(0x0045_6630, &[1]);
        let (ret, log) = r.run(0x0087_6810, &args![0x4000u32]);
        assert_eq!(ret.u8(), 1);
        assert_eq!(calls_to(&log, 0x0045_6630), vec![vec![0x4000, 0x800]]);
        let (_, log) = r.run(0x0087_6830, &args![0x4000u32, 1u32, 2u32]);
        assert_eq!(addrs(&log), vec![0x00b6_b790]);
        let (_, log) = r.run(0x0087_6a00, &args![0x4000u32]);
        assert_eq!(calls_to(&log, 0x00ba_39a0), vec![vec![0x407c]]);
    }

    #[test]
    fn texture_return_clears_the_byte_while_it_runs() {
        let (mut r, this) = scene_rig();
        r.e.set_global(0x011a_9604, 5u8);
        r.e.mem.set_u32(0x011c_7840, 0x11);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let seen_in = seen.clone();
        r.e.register_double(0x004e_9bb0, move |e, a| {
            seen_in
                .borrow_mut()
                .push((e.global::<u8>(0x011a_9604), a.to_vec()));
            Ret::default()
        });
        let (_, log) = r.run(0x0087_6a20, &args![this, 0x2222u32]);
        assert_eq!(*seen.borrow(), vec![(0, vec![0x2222, 0x11])]);
        assert_eq!(calls_to(&log, 0x00b6_5ec0), vec![vec![0x9400]]);
        assert_eq!(r.e.global::<u8>(0x011a_9604), 5);
    }

    #[test]
    fn frame_end_releases_what_the_frame_used() {
        let (mut r, this) = scene_rig();
        let decal = r.object(0x40);
        let device = r.object_with_slots(&[(0x1a0, 0), (0x190, 0)]);
        r.set(GET_GLOBAL_011F9508, &[decal]);
        r.set(0x004d_c020, &[device]);
        r.set(GET_GLOBAL_011F91A8, &[0x1600]);
        r.set(0x004e_bbc0, &[0x8000]);
        r.set(0x0068_3a60, &[1]);
        r.e.set_global(0x011f_9440, 1u8);
        let (_, log) = r.run(
            0x0087_6850,
            &args![this, 0x7001u32, 0x2000u32, 0u32, 0x9999u32],
        );
        assert_eq!(calls_to(&log, 0x0064_2890), vec![vec![0x9999]]);
        assert_eq!(calls_to(&log, 0x00ba_9420), vec![vec![0x6000]]);
        assert_eq!(calls_to(&log, 0x00b6_da10), vec![vec![0x1600, 0x7001]]);
        assert_eq!(calls_to(&log, 0x00b6_5660), vec![vec![0x8800]]);
        assert_eq!(calls_to(&log, 0x00b6_30c0), vec![vec![0x9400]]);
        assert_eq!(calls_to(&log, 0x004e_bbc0), vec![vec![0x2000, 4]]);
        assert_eq!(calls_to(&log, 0x00ba_39a0), vec![vec![0x8000 + 0x7c]]);
        assert_eq!(calls_to(&log, 0x00ba_d4a0).len(), 1);
        assert_eq!(calls_to(&log, 0x004d_c020), vec![vec![decal]]);
        assert_eq!(
            calls_to(&log, slot_target(&r, device, 0x1a0)),
            vec![vec![device, 0]]
        );
        assert_eq!(
            calls_to(&log, slot_target(&r, device, 0x190)),
            vec![vec![device, 0, 0, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x0071_4900).len(), 1);
        assert_eq!(calls_to(&log, 0x00b9_88e0).len(), 1);
        // No object kept in the global 0x011fa008: nothing built.
        assert!(calls_to(&log, 0x004d_61b0).is_empty());
        // Without a render target, an object or the flags the extra calls
        // are skipped.
        r.set(0x0068_3a60, &[0]);
        r.e.set_global(0x011f_9440, 0u8);
        let (_, log) = r.run(0x0087_6850, &args![this, 0u32, 0x2000u32, 0u32, 0u32]);
        assert!(calls_to(&log, 0x0064_2890).is_empty());
        assert!(calls_to(&log, 0x00b6_da10).is_empty());
        assert!(calls_to(&log, 0x00ba_d4a0).is_empty());
        assert!(calls_to(&log, 0x0071_4900).is_empty());
    }

    #[test]
    fn frame_end_builds_the_object_of_the_pointer_global() {
        let (mut r, this) = scene_rig();
        let decal = r.object(0x40);
        let device = r.object_with_slots(&[(0x1a0, 0), (0x190, 0)]);
        r.set(GET_GLOBAL_011F9508, &[decal]);
        r.set(0x004d_c020, &[device]);
        r.set(ALLOCATE_OBJECT, &[0x5500]);
        r.set(0x0044_ddc0, &[0x20]);
        r.set(0x0084_e3a0, &[0x30]);
        r.e.mem.set_u32(0x011f_a008, 0x7777);
        let (_, log) = r.run(0x0087_6850, &args![this, 0u32, 0x2000u32, 0u32, 0u32]);
        assert_eq!(calls_to(&log, ALLOCATE_OBJECT), vec![vec![0x30]]);
        assert_eq!(
            calls_to(&log, 0x004d_61b0),
            vec![vec![
                0x5500,
                0x30,
                0x20,
                0x7777,
                0x0108_2d28,
                0x8000_0000,
                0x8000_0000,
                0x320,
                0x258
            ]]
        );
        assert_eq!(r.e.mem.u32(0x011f_a008), 0);
        // An allocation failure builds nothing but still drops the pointer.
        r.e.mem.set_u32(0x011f_a008, 0x7777);
        r.set(ALLOCATE_OBJECT, &[0]);
        let (_, log) = r.run(0x0087_6850, &args![this, 0u32, 0x2000u32, 0u32, 0u32]);
        assert!(calls_to(&log, 0x004d_61b0).is_empty());
        assert_eq!(r.e.mem.u32(0x011f_a008), 0);
    }

    #[test]
    fn registry_value_is_read_and_the_key_closed() {
        let mut r = Rig::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let seen_in = seen.clone();
        r.e.register_double(0x00fd_f00c, move |e, a| {
            seen_in.borrow_mut().push(("open", a.to_vec()));
            e.mem.set_u32(a[4], 0x5151);
            Ret::default()
        });
        let seen_in = seen.clone();
        r.e.register_double(0x00fd_f008, move |e, a| {
            seen_in.borrow_mut().push(("query", a.to_vec()));
            // The size cell holds the size given by the caller.
            assert_eq!(e.mem.u32(a[5]), 0x40);
            Ret::default()
        });
        let seen_in = seen.clone();
        r.e.register_double(0x00fd_f004, move |_, a| {
            seen_in.borrow_mut().push(("close", a.to_vec()));
            Ret::default()
        });
        r.e.call(0x0087_6a70, &args![0x6000u32, 0x40u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 3);
        assert_eq!(seen[0].0, "open");
        assert_eq!(seen[0].1[..4], [0x8000_0002, 0x0108_2d48, 0, 1]);
        assert_eq!(seen[1].1[..5], [0x5151, 0x0108_2d38, 0, 0, 0x6000]);
        assert_eq!(seen[2], ("close", vec![0x5151]));
    }

    #[test]
    fn registry_value_is_not_queried_when_the_open_fails() {
        let mut r = Rig::new();
        r.set(0x00fd_f00c, &[2]);
        let (_, log) = r.run(0x0087_6a70, &args![0x6000u32, 0x40u32]);
        assert!(calls_to(&log, 0x00fd_f008).is_empty());
        // The handle is closed anyway (it stays 0).
        assert_eq!(calls_to(&log, 0x00fd_f004), vec![vec![0]]);
    }

    #[test]
    fn game_folder_is_prepared_from_the_ini_settings() {
        let mut r = Rig::new();
        let buffer = r.object(0x110);
        r.e.set_global(0x011a_2ff4, 0x1111u32);
        r.e.set_global(0x011a_2ff0, 0x2222u32);
        r.set(0x00fd_f0c4, &[1]);
        r.set(0x0086_d480, &[0x7001]);
        r.set(0x004d_c110, &[0x7002]);
        r.set(0x00c3_bb80, &[0xa1]);
        r.set(0x00c3_bbd0, &[0xa2]);
        r.set(0x00c3_bc20, &[0xa3]);
        r.set(0x00ec_bf05, &[u32::MAX]);
        let (_, log) = r.run(0x0087_6ad0, &args![buffer]);
        assert_eq!(
            calls_to(&log, 0x0040_6d00),
            vec![
                vec![buffer, 0x104, 0x0108_2dc4, 0x1111],
                vec![buffer, 0x104, 0x0108_2dc4, 0x2222]
            ]
        );
        assert_eq!(
            calls_to(&log, 0x00fd_f0c4)[0][..3],
            [0x0105_d548, 0x0108_2dac, 1]
        );
        // The per-user folders are made: local application data, then
        // the documents folder with two sub-folders.
        let folders = calls_to(&log, 0x00fd_f274);
        assert_eq!(folders.len(), 2);
        assert_eq!(folders[0][..4], [0, 0x1c, 0, 0]);
        assert_eq!(folders[1][..4], [0, 5, 0, 0]);
        assert_eq!(calls_to(&log, 0x00fd_f0b8).len(), 3);
        assert_eq!(calls_to(&log, 0x0040_6d30)[0][..2], [0x7001, 0x104]);
        assert_eq!(calls_to(&log, 0x0040_6d30)[1][..2], [0x7002, 0x104]);
        // The handles are closed.
        assert_eq!(
            calls_to(&log, 0x00fd_f0d4),
            vec![vec![0xa1], vec![0xa2], vec![0xa3]]
        );
        // The file is missing: the default is copied to it.
        let access = calls_to(&log, 0x00ec_bf05);
        assert_eq!(access, vec![vec![buffer, 0]]);
        let copy = calls_to(&log, 0x00fd_f1b8);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0][1..], [buffer, 1]);
    }

    #[test]
    fn game_folder_falls_back_to_the_plain_folder_without_the_ini_flag() {
        let mut r = Rig::new();
        let buffer = r.object(0x110);
        r.set(0x00fd_f0c4, &[0]);
        r.set(0x0086_d480, &[0x7001]);
        r.set(0x004d_c110, &[0x7002]);
        r.set(0x00ec_bf05, &[0]);
        let (_, log) = r.run(0x0087_6ad0, &args![buffer]);
        assert!(calls_to(&log, 0x00fd_f274).is_empty());
        assert!(calls_to(&log, 0x00fd_f0b8).is_empty());
        assert_eq!(
            calls_to(&log, 0x0040_6d30)[..2],
            [
                vec![0x7001, 0x104, 0x0108_2d84],
                vec![0x7002, 0x104, 0x0108_2d84]
            ]
        );
        // One check only: the first failed, so the second is skipped.
        assert_eq!(calls_to(&log, 0x00fd_f0c4).len(), 1);
        // The file exists: no copy.
        assert!(calls_to(&log, 0x00fd_f1b8).is_empty());
    }

    #[test]
    fn archive_setup_passes_the_settings_on() {
        let mut r = Rig::new();
        r.e.register_double(GET_POOLED_TEXT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        r.settings(
            SETTING_BYTE_PTR,
            &[(0x011d_ee3c, 1), (0x011d_ebdc, 2), (0x011d_ec4c, 3)],
        );
        r.settings(
            SETTING_DWORD_PTR,
            &[(0x011d_ec40, 10), (0x011d_eaa0, 20), (0x011d_eb24, 30)],
        );
        let (_, log) = r.run(0x0087_6d20, &args![]);
        assert_eq!(
            calls_to(&log, 0x00af_43a0),
            vec![vec![1, 0x011c_4031, 0x011d_eb95, 0x20_0000]]
        );
        assert_eq!(calls_to(&log, 0x00af_4460), vec![vec![2]]);
        assert_eq!(calls_to(&log, 0x00af_4470), vec![vec![30, 20, 10]]);
        assert_eq!(calls_to(&log, 0x00af_4490), vec![vec![3, 0x011d_ec69]]);
        assert_eq!(calls_to(&log, 0x00af_4550).len(), 1);
        assert_eq!(addrs(&log).last(), Some(&0x00af_4550));
    }

    /// The rig of the start-cell search: the setting strings are in the
    /// table `texts` (setting address, text); the others are empty;
    /// `00ec6130` measures them.
    fn start_cell_rig(texts: &[(u32, &str)]) -> Rig {
        let mut r = Rig::new();
        let mut table: HashMap<u32, u32> = HashMap::new();
        for (setting, text) in texts {
            let at = r.object(0x40);
            r.e.mem.set_cstr(at, text.as_bytes());
            table.insert(*setting, at);
        }
        let empty = r.object(0x10);
        r.e.register_double(GET_POOLED_TEXT, move |_, a| Ret {
            eax: table.get(&a[0]).copied().unwrap_or(empty),
            ..Ret::default()
        });
        r.e.register_double(0x00ec_6130, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        r.e.set_global(0x011c_3f2c, 0x4100u32);
        r.e.set_global(WORLD_OBJECT, 0x4200u32);
        r.e.set_global(0x0101_6968, 0.5f64);
        r.set(0x0046_0140, &[0x4300]);
        r
    }

    #[test]
    fn start_cell_by_name_loads_a_loaded_cell() {
        let mut r = start_cell_rig(&[]);
        let position = r.object(0x10);
        r.set(0x0044_a670, &[1]);
        r.set(0x0046_1ae0, &[0x7000]);
        r.set(0x0042_5fd0, &[1]);
        let (_, log) = r.run(0x0087_6dd0, &args![0x6100u32, position]);
        assert_eq!(calls_to(&log, 0x0046_1ae0), vec![vec![0x4100, 0x6100]]);
        assert_eq!(
            calls_to(&log, 0x0045_3dc0),
            vec![vec![0x4200, 0x7000, position]]
        );
        assert_eq!(calls_to(&log, 0x0046_50a0), vec![vec![0x4100, u32::MAX]]);
        // A found cell ends the work: no refresh.
        assert!(calls_to(&log, 0x0045_15a0).is_empty());
        assert!(calls_to(&log, 0x005b_5e40).is_empty());
    }

    #[test]
    fn start_cell_by_name_places_an_unloaded_cell_by_its_coordinates() {
        let mut r = start_cell_rig(&[]);
        let position = r.object(0x10);
        r.set(0x0044_a670, &[1]);
        r.set(0x0046_1ae0, &[0x7000]);
        r.set(0x0054_ddd0, &[0x7300]);
        r.set(0x0054_4c30, &[2]);
        r.set(0x0054_4c60, &[0xffff_fffe]);
        let (_, log) = r.run(0x0087_6dd0, &args![0x6100u32, position]);
        assert_eq!(calls_to(&log, 0x0045_8200), vec![vec![0x4200, 0x7300]]);
        assert_eq!(r.e.mem.f32(position), 8192.5);
        assert_eq!(r.e.mem.f32(position + 4), -8191.5);
        assert_eq!(r.e.mem.f32(position + 8), 0.0);
        assert_eq!(calls_to(&log, 0x0045_4450), vec![vec![0x4200, position]]);
        assert_eq!(calls_to(&log, 0x0046_50a0).len(), 1);
    }

    #[test]
    fn start_cell_by_setting_creates_the_cell_of_a_found_world_space() {
        let mut r = start_cell_rig(&[(0x011d_ed5c, "Goodsprings")]);
        let position = r.object(0x10);
        // No cell by that name; the world-space lookup answers and writes
        // the grid coordinates.
        r.set(0x0046_1ae0, &[0]);
        r.e.register_double(0x0046_1cf0, |e, a| {
            e.mem.set_u32(a[2], 3);
            e.mem.set_u32(a[3], 0xffff_fffd);
            Ret {
                eax: 0x7500,
                ..Ret::default()
            }
        });
        r.set(0x0046_1c20, &[0x7600]);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        assert_eq!(calls_to(&log, 0x0045_8200), vec![vec![0x4200, 0x7500]]);
        assert_eq!(r.e.mem.f32(position), 12288.5);
        assert_eq!(r.e.mem.f32(position + 4), -12287.5);
        assert_eq!(calls_to(&log, 0x0045_4450), vec![vec![0x4200, position]]);
        assert_eq!(
            calls_to(&log, 0x0046_1c20),
            vec![vec![0x4100, 3, 0xffff_fffd, 0x7500, 0]]
        );
        // The cell was created and positioned: nothing else is done.
        assert!(calls_to(&log, 0x0046_50a0).is_empty());
    }

    #[test]
    fn start_cell_by_world_space_name_searches_the_list() {
        let mut r = start_cell_rig(&[
            (0x011d_ee8c, "WastelandNV"),
            (0x011d_ebc4, "4"),
            (0x011d_eb48, "Mojave"),
        ]);
        let position = r.object(0x10);
        let list = r.object(0x10);
        let default_space = r.object_with_slots(&[(0x130, 0x6666)]);
        let chosen = r.object_with_slots(&[(0x130, 0x6666)]);
        r.e.mem.set_u32(list, default_space);
        r.set(0x0046_0140, &[list]);
        r.set(0x0068_15c0, &[list]);
        r.set(0x0072_6070, &[0]);
        r.set(0x00ec_a6d3, &[5, 6]);
        r.set(0x0058_5b30, &[0]);
        r.set(0x0046_1c20, &[0x7700]);
        // The name does not match: the default (first) world space is used.
        r.set(0x0040_4dc0, &[1]);
        r.set(0x0042_5fd0, &[1]);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        assert_eq!(calls_to(&log, 0x0058_5b30), vec![vec![default_space, 5, 6]]);
        assert_eq!(
            calls_to(&log, 0x0046_1c20),
            vec![vec![0x4100, 5, 6, default_space, 1]]
        );
        assert_eq!(
            calls_to(&log, 0x0045_3dc0),
            vec![vec![0x4200, 0x7700, position]]
        );
        // The name matches: that world space is taken.
        r.e.mem.set_u32(list, chosen);
        r.set(0x0040_4dc0, &[0]);
        r.set(0x00ec_a6d3, &[5, 6]);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        assert_eq!(calls_to(&log, 0x0058_5b30), vec![vec![chosen, 5, 6]]);
        // A found cell skips the creation.
        r.set(0x0058_5b30, &[0x7800]);
        r.set(0x00ec_a6d3, &[5, 6]);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        assert!(calls_to(&log, 0x0046_1c20).is_empty());
        assert_eq!(
            calls_to(&log, 0x0045_3dc0),
            vec![vec![0x4200, 0x7800, position]]
        );
    }

    #[test]
    fn start_cell_without_any_setting_does_nothing() {
        let mut r = start_cell_rig(&[]);
        let position = r.object(0x10);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        assert!(calls_to(&log, 0x0046_50a0).is_empty());
        assert!(calls_to(&log, 0x005b_5e40).is_empty());
    }

    #[test]
    fn start_cell_failure_logs_a_message_and_refreshes_the_world() {
        let mut r = start_cell_rig(&[(0x011d_ed5c, "Nowhere")]);
        let position = r.object(0x10);
        r.set(0x0046_1ae0, &[0]);
        r.set(0x0046_1cf0, &[0]);
        r.set(0x0044_ddc0, &[0x4400]);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        assert_eq!(calls_to(&log, 0x005b_5e40).len(), 1);
        assert_eq!(calls_to(&log, 0x005b_5e40)[0][0], 0x0108_2e94);
        assert_eq!(calls_to(&log, 0x0046_50a0), vec![vec![0x4100, u32::MAX]]);
        assert_eq!(calls_to(&log, 0x0045_15a0), vec![vec![0x4200, position, 1]]);
        assert_eq!(calls_to(&log, 0x004b_a7d0), vec![vec![0x4400]]);
    }

    #[test]
    fn start_cell_failure_by_world_space_names_the_missing_pieces() {
        let list = |r: &mut Rig| {
            let list = r.object(0x10);
            r.set(0x0046_0140, &[list]);
            r.set(0x0068_15c0, &[list]);
            // The list node reports the end at once.
            r.set(0x0082_56d0, &[1]);
        };
        // World space and coordinates, no name setting.
        let mut r = start_cell_rig(&[(0x011d_ee8c, "Space"), (0x011d_ebc4, "7")]);
        list(&mut r);
        let position = r.object(0x10);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        let message = calls_to(&log, 0x005b_5e40);
        assert_eq!(message.len(), 1);
        assert_eq!(message[0][0], 0x0108_2e00);
        assert_eq!(message[0].len(), 3);
        // With a name setting too: the longer message.
        let mut r = start_cell_rig(&[
            (0x011d_ee8c, "Space"),
            (0x011d_ebc4, "7"),
            (0x011d_eb48, "Name"),
        ]);
        list(&mut r);
        let position = r.object(0x10);
        let (_, log) = r.run(0x0087_6dd0, &args![0u32, position]);
        let message = calls_to(&log, 0x005b_5e40);
        assert_eq!(message[0][0], 0x0108_2e50);
        assert_eq!(message[0].len(), 4);
        // A name accepted but no cell and no settings at all: the plain message.
        let mut r = start_cell_rig(&[]);
        let position = r.object(0x10);
        r.set(0x0044_a670, &[1]);
        r.set(0x0046_1ae0, &[0]);
        r.set(0x0046_1cf0, &[0]);
        let (_, log) = r.run(0x0087_6dd0, &args![0x6100u32, position]);
        assert_eq!(calls_to(&log, 0x005b_5e40), vec![vec![0x0108_2dcc]]);
    }

    #[test]
    fn player_move_places_the_player_in_the_cell() {
        let mut r = Rig::new();
        let player = r.object_with_slots(&[(0x2c4, 0), (0x2a8, 0)]);
        r.e.set_global(PLAYER_OBJECT, player);
        r.e.set_global(WORLD_OBJECT, 0x4200u32);
        r.e.set_global(0x0120_2d98, 0x4900u32);
        for (k, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            r.e.set_global(ZERO_VECTOR + 4 * k as u32, *v);
        }
        r.set(0x005f_36f0, &[0x7000]);
        r.set(GET_ROOT, &[0x4000]);
        r.set(0x0055_8310, &[0x4a00]);
        r.set(GET_PLAYER_NODE, &[0x3000]);
        r.set(INDEXED_GLOBAL, &[0x4b00]);
        r.set(0x0056_fa00, &[0x4c00]);
        let (_, log) = r.run(0x0087_7260, &args![word(10.0), word(20.0), word(30.0)]);
        let moved = calls_to(&log, 0x0054_cfd0);
        assert_eq!(moved.len(), 1);
        assert_eq!(moved[0][0], 0x7000);
        let pos = moved[0][1];
        // The player is moved by the z of the cell's rotation and the position.
        let height = calls_to(&log, slot_target(&r, player, 0x2c4));
        assert_eq!(height, vec![vec![player, word(3.0)]]);
        assert_eq!(
            calls_to(&log, slot_target(&r, player, 0x2a8)),
            vec![vec![player, pos]]
        );
        assert_eq!(calls_to(&log, 0x0054_8230), vec![vec![0x7000, player, 0]]);
        assert_eq!(calls_to(&log, 0x0045_6520), vec![vec![0x4900]]);
        let guard = calls_to(&log, PROFILE_SCOPE_CTOR)[0][0];
        assert_eq!(
            calls_to(&log, PROFILE_SCOPE_CTOR),
            vec![vec![guard, 0x34, 1, SOURCE_FILE_NAME, 0x2612]]
        );
        assert_eq!(calls_to(&log, 0x00a5_a040), vec![vec![0x3000]]);
        assert_eq!(calls_to(&log, 0x00b5_cbd0), vec![vec![0x4b00, 0x3000]]);
        assert_eq!(calls_to(&log, 0x0056_5730), vec![vec![player]]);
        assert_eq!(calls_to(&log, 0x0044_0460), vec![vec![0x4a00, pos]]);
        assert_eq!(calls_to(&log, 0x0043_fa80), vec![vec![0x4a00, 0x4c00]]);
        let point = calls_to(&log, POINT3_CTOR)[0][0];
        assert_eq!(calls_to(&log, 0x00a5_9c60), vec![vec![0x4a00, point]]);
        // The position handed over holds the arguments.
        assert_eq!(r.e.mem.f32(pos), 10.0);
        assert_eq!(r.e.mem.f32(pos + 8), 30.0);
    }

    #[test]
    fn player_move_finds_the_cell_by_the_grid_when_the_world_has_none() {
        let mut r = Rig::new();
        let player = r.object_with_slots(&[(0x2c4, 0), (0x2a8, 0)]);
        r.e.set_global(PLAYER_OBJECT, player);
        r.e.set_global(WORLD_OBJECT, 0x4200u32);
        r.settings(SETTING_DWORD_PTR, &[(0x011c_63cc, 9)]);
        let slot = r.object(0x10);
        r.set(0x0045_7050, &[slot]);
        let (_, log) = r.run(0x0087_7260, &args![0u32, 0u32, 0u32]);
        // 9 >> 1 for both grid coordinates; no cell: no placement or add.
        assert_eq!(calls_to(&log, 0x0045_7050), vec![vec![0x4200, 4, 4]]);
        assert!(calls_to(&log, 0x0054_cfd0).is_empty());
        assert!(calls_to(&log, 0x0054_8230).is_empty());
        assert_eq!(calls_to(&log, 0x0056_5730), vec![vec![player]]);
    }
}

#[cfg(test)]
mod tests_third {
    //! Tests of `00877430` to `00877f90`. Every callee outside this file
    //! has a test double (an unlisted callee panics as an open function).
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// The engine with data pages mapped and a double for each listed
    /// callee, returning the value set for it (default 0) in `eax`.
    struct Rig {
        e: Engine,
        returns: Rc<RefCell<HashMap<u32, u32>>>,
        next_slot_target: u32,
    }

    impl Rig {
        fn new(callees: &[u32]) -> Rig {
            let mut e = Engine::new();
            for page in (0x0100_0000u32..0x0130_0000).step_by(0x1000) {
                e.map(page, 0x1000);
            }
            let returns: Rc<RefCell<HashMap<u32, u32>>> = Rc::new(RefCell::new(HashMap::new()));
            for &addr in callees {
                let table = returns.clone();
                e.register_double(addr, move |_, _| Ret {
                    eax: table.borrow().get(&addr).copied().unwrap_or(0),
                    ..Ret::default()
                });
            }
            e.register_double(POINTER_GET, |e, a| Ret {
                eax: e.mem.u32(a[0]),
                ..Ret::default()
            });
            Rig {
                e,
                returns,
                next_slot_target: 0x00f1_0000,
            }
        }

        fn set(&mut self, addr: u32, value: u32) {
            self.returns.borrow_mut().insert(addr, value);
        }

        /// An object whose vtable slots (offset, result) are doubles.
        fn object_with_slots(&mut self, slots: &[(u32, u32)]) -> u32 {
            let object = self.e.mem.alloc(0x40);
            let vtable = self.e.mem.alloc(0x400);
            self.e.mem.set_u32(object, vtable);
            for &(offset, result) in slots {
                let target = self.next_slot_target;
                self.next_slot_target += 0x10;
                self.e.register_double(target, move |_, _| Ret {
                    eax: result,
                    ..Ret::default()
                });
                self.e.mem.set_u32(vtable + offset, target);
            }
            object
        }

        /// Runs `addr` and returns its result and the calls it made.
        fn run(&mut self, addr: u32, args: &[u32]) -> (Ret, Log) {
            self.e.call_log = Some(vec![]);
            let ret = self.e.call(addr, args);
            let mut log = self.e.call_log.take().unwrap();
            log.remove(0);
            log.retain(|(addr, _)| *addr != POINTER_GET);
            (ret, log)
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn addrs(log: &Log) -> Vec<u32> {
        log.iter().map(|(addr, _)| *addr).collect()
    }

    #[test]
    fn kill_menu_bg_texture_returns_the_texture_and_clears_the_pointer() {
        let mut r = Rig::new(&[
            0x004a_0ea0,
            0x004e_3270,
            0x00b6_da10,
            0x0066_b0d0,
            0x0071_23f0,
            0x0071_23a0,
        ]);
        r.e.mem.set_u32(MENU_TARGET_POINTER, 0x5000);
        r.e.mem.set_u8(0x011d_ea29, 1);
        r.set(0x004a_0ea0, 0x6000);
        r.set(0x004e_3270, 0x7000);
        let (_, log) = r.run(0x0087_7430, &args![0u32]);
        assert_eq!(
            log,
            vec![
                (0x004a_0ea0, vec![]),
                (0x00b6_da10, vec![0x6000, 0x5000]),
                (0x0066_b0d0, vec![MENU_TARGET_POINTER, 0]),
                (0x004e_3270, vec![]),
                (0x0071_23f0, vec![0x7000, 0]),
                (0x004e_3270, vec![]),
                (0x0071_23a0, vec![0x7000, 0]),
            ]
        );
        assert_eq!(r.e.mem.u8(0x011d_ea29), 0);
    }

    #[test]
    fn kill_menu_bg_texture_without_a_texture_only_clears_the_byte() {
        let mut r = Rig::new(&[]);
        r.e.mem.set_u8(0x011d_ea29, 1);
        let (_, log) = r.run(0x0087_7430, &args![0u32]);
        assert!(log.is_empty());
        assert_eq!(r.e.mem.u8(0x011d_ea29), 0);
    }

    const TEARDOWN_CALLEES: &[u32] = &[
        0x004f_1540,
        0x004f_15a0,
        0x00ad_8da0,
        0x007d_6bd0,
        0x0045_3a70,
        0x00ad_8780,
        0x0083_24e0,
        0x008d_6f30,
        0x0054_ca90,
        0x0095_0bb0,
        0x0096_11e0,
        0x0045_39a0,
        0x0070_37c0,
        0x0061_cc40,
        0x006c_0720,
        0x006c_09f0,
        0x0086_8d70,
        0x00c4_59d0,
        0x0084_a840,
        0x0097_0d50,
        0x0046_14e0,
        0x00b4_f5c0,
        0x00b6_31d0,
        0x0040_fbe0,
        0x0070_6320,
        0x0045_ac80,
        0x0070_53f0,
    ];

    fn teardown_rig() -> (Rig, u32) {
        let mut r = Rig::new(TEARDOWN_CALLEES);
        r.e.map(0x3000, 0x1000);
        let player = r.object_with_slots(&[(0x1cc, 0)]);
        r.e.set_global(PLAYER_OBJECT, player);
        r.e.set_global(WORLD_OBJECT, 0x3200u32);
        r.e.set_global(MAIN_OBJECT, 0x3300u32);
        r.e.set_global(0x011d_df38u32, 0x3100u32);
        r.e.set_global(0x011c_3f2cu32, 0x3400u32);
        r.e.mem.set_u8(0x3300 + 2, 1);
        r.set(0x004f_1540, 0x1_07);
        r.set(0x007d_6bd0, 0x1_01);
        r.set(0x0045_3a70, 0x4100);
        r.set(0x006c_0720, 0x4200);
        r.set(0x00b4_f5c0, 0x4300);
        (r, player)
    }

    #[test]
    fn teardown_runs_every_step_and_restores_the_saved_bytes() {
        let (mut r, player) = teardown_rig();
        r.set(0x008d_6f30, 0x4400);
        let owner_node = r.object_with_slots(&[(0xc, 0x4500)]);
        let owner_target = r.object_with_slots(&[(0xe8, 0)]);
        r.set(0x0095_0bb0, owner_node);
        r.set(0x0096_11e0, owner_target);
        let (_, log) = r.run(0x0087_74a0, &args![0x3000u32, 1u32]);
        assert_eq!(calls_to(&log, 0x004f_1540), vec![vec![0x3000]]);
        assert_eq!(
            calls_to(&log, 0x004f_15a0),
            vec![vec![0x3000, 0], vec![0x3300, 7]]
        );
        assert_eq!(
            calls_to(&log, 0x00ad_8da0),
            vec![vec![player + 0x77c, 0x3e8]]
        );
        assert_eq!(
            calls_to(&log, 0x007d_6bd0),
            vec![vec![0x3100, 0], vec![0x3100, 1]]
        );
        assert_eq!(calls_to(&log, 0x00ad_8780), vec![vec![0x4100, 0xffff_ffff]]);
        assert_eq!(calls_to(&log, 0x0083_24e0), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x0054_ca90), vec![vec![0x4400, player]]);
        assert_eq!(
            calls_to(&log, 0x0095_0bb0),
            vec![vec![player, 1], vec![player, 0]]
        );
        // Each node's owner, then the owner's target is told about the node.
        assert_eq!(calls_to(&log, 0x0096_11e0).len(), 4);
        assert_eq!(
            calls_to(&log, r.e.mem.u32(r.e.mem.u32(owner_target) + 0xe8)).len(),
            2
        );
        assert_eq!(calls_to(&log, 0x0045_39a0), vec![vec![0x3200, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0070_37c0), vec![vec![0x3200, 0x7fff_ffff]]);
        assert_eq!(calls_to(&log, 0x0061_cc40), vec![vec![0x3200, 0x7fff_ffff]]);
        assert_eq!(calls_to(&log, 0x006c_09f0), vec![vec![0x4200]]);
        assert_eq!(calls_to(&log, 0x0086_8d70), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x00c4_59d0), vec![vec![0]]);
        assert_eq!(
            calls_to(&log, r.e.mem.u32(r.e.mem.u32(player) + 0x1cc)),
            vec![vec![player, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x0084_a840), vec![vec![0x3100]]);
        assert_eq!(calls_to(&log, 0x0097_0d50), vec![vec![0x011e_0e80]]);
        assert_eq!(calls_to(&log, 0x0046_14e0), vec![vec![0x3400]]);
        assert_eq!(calls_to(&log, 0x00b6_31d0), vec![vec![0x4300]]);
        assert_eq!(calls_to(&log, 0x0040_fbe0), vec![vec![]]);
        assert_eq!(calls_to(&log, 0x0070_6320), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x0045_ac80), vec![vec![0x3200]]);
        assert_eq!(calls_to(&log, 0x0070_53f0), vec![vec![]]);
        // The main object's bytes are cleared before the saved byte is written.
        assert_eq!(r.e.mem.u8(0x3300 + 2), 0);
        assert_eq!(addrs(&log).last(), Some(&0x004f_15a0));
    }

    #[test]
    fn teardown_skips_the_cell_removal_nodes_and_optional_step_when_absent() {
        let (mut r, player) = teardown_rig();
        let (_, log) = r.run(0x0087_74a0, &args![0x3000u32, 0u32]);
        assert_eq!(calls_to(&log, 0x008d_6f30), vec![vec![player]]);
        assert!(calls_to(&log, 0x0054_ca90).is_empty());
        assert_eq!(calls_to(&log, 0x0095_0bb0).len(), 2);
        assert!(calls_to(&log, 0x0096_11e0).is_empty());
        assert!(calls_to(&log, 0x0070_53f0).is_empty());
    }

    #[test]
    fn main_flags_and_sound_fade() {
        let mut r = Rig::new(&[0x00ad_8da0, 0x007f_df30]);
        r.e.map(0x3000, 0x1000);
        r.e.mem.set_u8(0x3002, 1);
        r.e.mem.set_u8(0x3005, 1);
        r.run(0x0087_76e0, &args![0x3000u32]);
        assert_eq!((r.e.mem.u8(0x3002), r.e.mem.u8(0x3005)), (0, 0));
        let (_, log) = r.run(0x0087_7700, &args![0x3000u32]);
        assert_eq!(log, vec![(0x00ad_8da0, vec![0x377c, 0x3e8])]);
        r.set(0x007f_df30, 0x9000);
        let (ret, _) = r.run(0x0087_7720, &args![0u32]);
        assert_eq!(ret.u32(), 0x9000);
    }

    const SKY_CALLEES: &[u32] = &[
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_fbe0,
        0x0063_a630,
        0x0045_cd60,
        0x0043_b230,
        0x0045_05a0,
        0x0045_0b80,
        0x00b5_aac0,
        0x00a5_a040,
        0x0040_8d60,
        0x004e_3270,
        0x00b8_b200,
        0x00aa_13e0,
        0x0049_ec80,
        0x0049_ed90,
        0x0049_ede0,
        0x0050_f9a0,
        0x0043_9410,
        0x0043_d410,
        0x00a5_9c60,
        0x00b5_7e30,
    ];

    fn sky_rig(setting: u8) -> (Rig, u32) {
        let mut r = Rig::new(SKY_CALLEES);
        let sun = r.object_with_slots(&[(0xdc, 0)]);
        r.e.mem.set_u32(0x011d_eda4, sun);
        r.e.mem.set_u32(0x011d_eb00, 0xa000);
        r.e.mem.set_u32(0x011d_eb34, 0xb000);
        r.e.set_global(0x011d_ea20u32, 0xc000u32);
        let flag = r.e.mem.alloc(8);
        r.e.mem.set_u8(flag, setting);
        r.set(0x0040_8d60, flag);
        r.set(0x0045_cd60, 0xc100);
        r.set(0x0043_b230, 0xc200);
        r.set(0x0045_05a0, 0xc300);
        r.set(0x0045_0b80, 0xc400);
        r.set(0x004e_3270, 0xc500);
        r.set(0x00aa_13e0, 0xd000);
        r.set(0x0049_ec80, 0xd100);
        (r, sun)
    }

    #[test]
    fn init_sky_with_the_setting_attaches_the_property() {
        let (mut r, sun) = sky_rig(1);
        let (_, log) = r.run(0x0087_7730, &args![0u32]);
        let guard = calls_to(&log, 0x0040_4eb0)[0][0];
        assert_eq!(
            calls_to(&log, 0x0040_4eb0),
            vec![vec![guard, 0x21, 1, SOURCE_FILE_NAME, 0x26bd]]
        );
        assert_eq!(calls_to(&log, 0x0040_fbe0), vec![vec![0x0108_2ec0]]);
        assert_eq!(
            calls_to(&log, 0x0063_a630),
            vec![vec![0xc000, 0xb000, 0xa000]]
        );
        let slot = r.e.mem.u32(r.e.mem.u32(sun) + 0xdc);
        assert_eq!(calls_to(&log, slot), vec![vec![sun, 0xc200, 1]]);
        assert_eq!(calls_to(&log, 0x00b5_aac0), vec![vec![0xc400, 0xc300]]);
        assert_eq!(calls_to(&log, 0x00b8_b200), vec![vec![0xc500, 1]]);
        assert_eq!(calls_to(&log, 0x00aa_13e0), vec![vec![0x24]]);
        assert_eq!(calls_to(&log, 0x0049_ec80), vec![vec![0xd000]]);
        assert_eq!(calls_to(&log, 0x0049_ed90), vec![vec![0xd100, 1]]);
        assert_eq!(calls_to(&log, 0x0049_ede0), vec![vec![0xd100, 2]]);
        assert_eq!(calls_to(&log, 0x0050_f9a0), vec![vec![0xd100, 0xff]]);
        assert_eq!(calls_to(&log, 0x0043_9410), vec![vec![0xb000, 0xd100]]);
        let points: Vec<u32> = calls_to(&log, 0x0043_d410).iter().map(|a| a[0]).collect();
        assert_eq!(
            calls_to(&log, 0x00a5_9c60),
            vec![vec![0xb000, points[0]], vec![sun, points[1]]]
        );
        assert_eq!(calls_to(&log, 0x00a5_a040), vec![vec![0xb000], vec![sun]]);
        assert_eq!(
            calls_to(&log, 0x00b5_7e30),
            vec![vec![0xb000, 0, 0], vec![0xc200, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x0040_4ee0), vec![vec![guard]]);
    }

    #[test]
    fn init_sky_without_the_setting_skips_the_property() {
        let (mut r, _) = sky_rig(0);
        let (_, log) = r.run(0x0087_7730, &args![0u32]);
        assert!(calls_to(&log, 0x00aa_13e0).is_empty());
        assert!(calls_to(&log, 0x0043_9410).is_empty());
        assert_eq!(calls_to(&log, 0x00b8_b200), vec![vec![0xc500, 0]]);
        assert_eq!(calls_to(&log, 0x00b5_7e30).len(), 2);
    }

    #[test]
    fn registry_collection_is_created_once() {
        let mut r = Rig::new(&[0x0040_1000, 0x0044_f700]);
        r.set(0x0040_1000, 0x5000);
        r.e.map(0x5000, 0x1000);
        let (_, log) = r.run(0x0087_7960, &args![]);
        assert_eq!(
            log,
            vec![(0x0040_1000, vec![0x114]), (0x0044_f700, vec![0x5000])]
        );
        assert_eq!(r.e.global::<u32>(0x0120_4368), 0x5000);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2ed8);
        let (_, log) = r.run(0x0087_7960, &args![]);
        assert!(log.is_empty());
        let (ret, log) = r.run(0x0087_7950, &args![]);
        assert!(log.is_empty());
        assert_eq!(ret.u32(), 0x5000);
    }

    #[test]
    fn registry_collection_stays_empty_when_the_allocation_fails() {
        let mut r = Rig::new(&[0x0040_1000, 0x0044_f700]);
        let (_, log) = r.run(0x0087_7960, &args![]);
        assert_eq!(addrs(&log), vec![0x0040_1000]);
        assert_eq!(r.e.global::<u32>(0x0120_4368), 0);
    }

    #[test]
    fn registry_collection_constructor_and_base_destructor_call() {
        let mut r = Rig::new(&[0x0044_f700, 0x0044_f650]);
        r.e.map(0x5000, 0x1000);
        r.set(0x0044_f650, 77);
        let (ret, _) = r.run(0x0087_79f0, &args![0x5000u32]);
        assert_eq!(ret.u32(), 0x5000);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2ed8);
        let (ret, log) = r.run(0x0087_7a10, &args![0x5000u32]);
        assert_eq!(ret.u32(), 77);
        assert_eq!(log, vec![(0x0044_f650, vec![0x5000])]);
    }

    /// An `NiTArray` of four slots at 0x5000, elements at 0x5100.
    fn array_rig() -> Rig {
        let mut r = Rig::new(&[0x0096_ad30]);
        r.e.map(0x5000, 0x1000);
        r.e.mem.set_u32(0x5004, 0x5100);
        r.e.mem.set_u16(0x5008, 4);
        r.e.mem.set_u16(0x500e, 3);
        r
    }

    #[test]
    fn array_element_address() {
        let mut r = array_rig();
        assert_eq!(r.run(0x0087_7a30, &args![0x5000u32, 3u32]).0.u32(), 0x510c);
    }

    #[test]
    fn array_set_at_counts_elements_and_size() {
        let mut r = array_rig();
        r.e.map(0x6000, 0x1000);
        // Past the end with a value: size and element count grow.
        r.e.mem.set_u32(0x6000, 0x1234);
        r.run(0x0087_7e50, &args![0x5000u32, 2u32, 0x6000u32]);
        assert_eq!(r.e.mem.u16(0x500a), 3);
        assert_eq!(r.e.mem.u16(0x500c), 1);
        assert_eq!(r.e.mem.u32(0x5108), 0x1234);
        // Past the end with the null word: size grows only.
        r.e.mem.set_u32(0x6004, 0);
        r.run(0x0087_7e50, &args![0x5000u32, 3u32, 0x6004u32]);
        assert_eq!(r.e.mem.u16(0x500a), 4);
        assert_eq!(r.e.mem.u16(0x500c), 1);
        // Inside: filling an empty slot counts, replacing does not.
        r.run(0x0087_7e50, &args![0x5000u32, 1u32, 0x6000u32]);
        assert_eq!(r.e.mem.u16(0x500c), 2);
        r.e.mem.set_u32(0x6008, 0x99);
        r.run(0x0087_7e50, &args![0x5000u32, 1u32, 0x6008u32]);
        assert_eq!(r.e.mem.u16(0x500c), 2);
        assert_eq!(r.e.mem.u32(0x5104), 0x99);
        // Inside: clearing a filled slot counts down; clearing an empty one does not.
        r.run(0x0087_7e50, &args![0x5000u32, 1u32, 0x6004u32]);
        assert_eq!(r.e.mem.u16(0x500c), 1);
        r.run(0x0087_7e50, &args![0x5000u32, 1u32, 0x6004u32]);
        assert_eq!(r.e.mem.u16(0x500c), 1);
    }

    #[test]
    fn array_set_at_grow_resizes_only_past_the_capacity() {
        let mut r = array_rig();
        r.e.map(0x6000, 0x1000);
        r.e.mem.set_u32(0x6000, 0x55);
        let (ret, log) = r.run(0x0087_7e10, &args![0x5000u32, 2u32, 0x6000u32]);
        assert_eq!(ret.u32(), 2);
        assert!(log.is_empty());
        let (ret, log) = r.run(0x0087_7e10, &args![0x5000u32, 4u32, 0x6000u32]);
        assert_eq!(ret.u32(), 4);
        assert_eq!(log, vec![(0x0096_ad30, vec![0x5000, 7])]);
    }

    #[test]
    fn array_add_stores_at_the_end() {
        let mut r = array_rig();
        r.e.map(0x6000, 0x1000);
        r.e.mem.set_u16(0x500a, 2);
        r.e.mem.set_u32(0x6000, 0x55);
        let (ret, _) = r.run(0x0087_7a50, &args![0x5000u32, 0x6000u32]);
        assert_eq!(ret.u32(), 2);
        assert_eq!(r.e.mem.u32(0x5108), 0x55);
        assert_eq!(r.e.mem.u16(0x500a), 3);
    }

    #[test]
    fn queue_constructors_set_vtables_and_links() {
        let mut r = Rig::new(&[]);
        r.e.map(0x5000, 0x1000);
        r.run(0x0087_7df0, &args![0x5000u32]);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2f44);
        r.e.mem.set_u32(0x5004, 9);
        r.run(0x0087_7d80, &args![0x5000u32]);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2f24);
        assert_eq!(r.e.mem.u32(0x5004), 0);
        r.e.mem.set_u32(0x500c, 9);
        let (ret, _) = r.run(0x0087_7a80, &args![0x5000u32, 0x7777u32]);
        assert_eq!(ret.u32(), 0x5000);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2f04);
        assert_eq!(r.e.mem.u32(0x5008), 0x7777);
        assert_eq!(r.e.mem.u32(0x500c), 0);
        assert_eq!(r.e.mem.u32(0x5010), 0x500c);
    }

    #[test]
    fn queue_destructor_bodies_step_down_the_vtables() {
        let mut r = Rig::new(&[]);
        r.e.map(0x5000, 0x1000);
        r.run(0x0087_7b60, &args![0x5000u32]);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2f44);
        r.run(0x0087_7b40, &args![0x5000u32]);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2f44);
    }

    #[test]
    fn queue_destructor_drains_the_tasks_first() {
        let mut r = Rig::new(&[0x006e_c390]);
        r.e.map(0x5000, 0x1000);
        let calls = Rc::new(RefCell::new(0u32));
        let counter = calls.clone();
        r.e.register_double(0x006e_c390, move |_, _| {
            let mut n = counter.borrow_mut();
            *n += 1;
            Ret {
                eax: if *n < 3 { 0x100 | 1 } else { 0x100 },
                ..Ret::default()
            }
        });
        r.run(0x0087_7ac0, &args![0x5000u32]);
        // Only the low byte counts as success: two pops, then the empty one.
        assert_eq!(*calls.borrow(), 3);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_2f44);
    }

    #[test]
    fn deleting_destructors_free_only_when_asked() {
        for (addr, vtable) in [
            (0x0087_7b80u32, 0x0108_2f44u32),
            (0x0087_7bb0, 0x0108_2f44),
            (0x0087_7d50, 0x0108_2f44),
        ] {
            let mut r = Rig::new(&[0x006e_c390, 0x0040_1030]);
            r.e.map(0x5000, 0x1000);
            let (ret, log) = r.run(addr, &args![0x5000u32, 0u32]);
            assert_eq!(ret.u32(), 0x5000);
            assert!(calls_to(&log, 0x0040_1030).is_empty());
            let (ret, log) = r.run(addr, &args![0x5000u32, 1u32]);
            assert_eq!(ret.u32(), 0x5000);
            assert_eq!(calls_to(&log, 0x0040_1030), vec![vec![0x5000]]);
            assert_eq!(r.e.mem.u32(0x5000), vtable);
        }
    }

    #[test]
    fn push_appends_nodes_to_the_list() {
        let mut r = Rig::new(&[0x00aa_54a0, 0x006e_6da0]);
        r.e.map(0x5000, 0x1000);
        r.e.mem.set_u32(0x010a_2720, 4);
        r.e.mem.set_u32(0x5008, 0x7000);
        r.e.mem.set_u32(0x5010, 0x500c);
        for word in 0..8 {
            r.e.mem.set_u32(0x5100 + word * 4, 0x10 + word);
            r.e.mem.set_u32(0x5200 + word * 4, 0x20 + word);
        }
        r.set(0x00aa_54a0, 0x5300);
        r.set(0x006e_6da0, 0x5300);
        let (ret, log) = r.run(0x0087_7be0, &args![0x5000u32, 0x5100u32]);
        assert!(ret.bool());
        assert_eq!(calls_to(&log, 0x00aa_54a0), vec![vec![0x7000, 0x24, 4]]);
        assert_eq!(calls_to(&log, 0x006e_6da0), vec![vec![0x24, 0x5300]]);
        assert_eq!(r.e.mem.u32(0x500c), 0x5300);
        assert_eq!(r.e.mem.u32(0x5010), 0x5300);
        assert_eq!(r.e.mem.u32(0x5300), 0);
        assert_eq!(r.e.mem.u32(0x5304), 0x10);
        assert_eq!(r.e.mem.u32(0x5320), 0x17);
        // A second node is linked after the first.
        r.set(0x00aa_54a0, 0x5400);
        r.set(0x006e_6da0, 0x5400);
        r.run(0x0087_7be0, &args![0x5000u32, 0x5200u32]);
        assert_eq!(r.e.mem.u32(0x5300), 0x5400);
        assert_eq!(r.e.mem.u32(0x5010), 0x5400);
        assert_eq!(r.e.mem.u32(0x500c), 0x5300);
        assert_eq!(r.e.mem.u32(0x5404), 0x20);
    }

    #[test]
    fn pop_takes_the_head_and_returns_it_to_the_heap() {
        let mut r = Rig::new(&[0x007b_3fa0, 0x00aa_5610]);
        r.e.map(0x5000, 0x1000);
        r.e.mem.set_u32(0x5008, 0x7000);
        // Two nodes: 0x5300 -> 0x5400.
        r.e.mem.set_u32(0x500c, 0x5300);
        r.e.mem.set_u32(0x5300, 0x5400);
        r.e.mem.set_u32(0x5304, 0xaa);
        r.e.mem.set_u32(0x5324, 0xbb);
        let (ret, log) = r.run(0x0087_7cc0, &args![0x5000u32, 0x5600u32]);
        assert!(ret.bool());
        assert_eq!(r.e.mem.u32(0x5600), 0xaa);
        assert_eq!(r.e.mem.u32(0x500c), 0x5400);
        assert_eq!(
            log,
            vec![
                (0x007b_3fa0, vec![0x5300, 0]),
                (0x00aa_5610, vec![0x7000, 0x5300])
            ]
        );
        // An empty list pops nothing.
        r.e.mem.set_u32(0x500c, 0);
        let (ret, log) = r.run(0x0087_7cc0, &args![0x5000u32, 0x5600u32]);
        assert!(!ret.bool());
        assert!(log.is_empty());
    }

    #[test]
    fn small_queue_helpers() {
        let mut r = Rig::new(&[]);
        r.e.map(0x5000, 0x1000);
        r.e.mem.set_u32(0x5004, 7);
        r.run(0x0087_7d30, &args![0x5000u32]);
        assert_eq!(r.e.mem.u8(0x5040), 1);
        for word in 0..9 {
            r.e.mem.set_u32(0x5100 + word * 4, 0xffff_ffff);
        }
        r.e.mem.set_u32(0x5124, 0xffff_ffff);
        let (ret, _) = r.run(0x0087_7db0, &args![0x5100u32]);
        assert_eq!(ret.u32(), 0x5100);
        for word in 0..9 {
            assert_eq!(r.e.mem.u32(0x5100 + word * 4), 0);
        }
        // The word after the node is untouched.
        assert_eq!(r.e.mem.u32(0x5124), 0xffff_ffff);
    }

    #[test]
    fn setting_constructor_registers_with_the_collection() {
        let mut r = Rig::new(&[0x0040_4920]);
        r.e.map(0x5000, 0x1000);
        let collection = r.object_with_slots(&[(4, 0)]);
        r.e.set_global(0x0120_4368u32, collection);
        let (ret, log) = r.run(0x0087_7f10, &args![0x5000u32, 0x11u32, 0x22u32]);
        assert_eq!(ret.u32(), 0x5000);
        assert_eq!(r.e.mem.u32(0x5000), 0x0108_39ac);
        assert_eq!(calls_to(&log, 0x0040_4920), vec![vec![0x5000, 0x11, 0x22]]);
        let slot = r.e.mem.u32(r.e.mem.u32(collection) + 4);
        assert_eq!(calls_to(&log, slot), vec![vec![collection, 0x5000]]);
    }

    #[test]
    fn setting_deleting_destructor_calls_the_body_then_frees() {
        let mut r = Rig::new(&[0x0087_7fc0, 0x0040_1030]);
        let (ret, log) = r.run(0x0087_7f90, &args![0x5000u32, 1u32]);
        assert_eq!(ret.u32(), 0x5000);
        assert_eq!(
            log,
            vec![(0x0087_7fc0, vec![0x5000]), (0x0040_1030, vec![0x5000])]
        );
        let (_, log) = r.run(0x0087_7f90, &args![0x5000u32, 0u32]);
        assert_eq!(log, vec![(0x0087_7fc0, vec![0x5000])]);
    }
}
