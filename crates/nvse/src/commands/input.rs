//! xNVSE's input commands (`Commands_Input.cpp` (xNVSE)), build step I-11: keys and mouse
//! buttons read, held, tapped and disabled through xNVSE's DirectInput hook ([`crate::input`]),
//! and the game's control bindings (`OSInputGlobals`, the exe's `Controls`) read and changed.
//!
//! Key codes are DirectInput's (`DIK_*`, 0-255), then the mouse buttons from 256 and the wheel's
//! up and down at 264 and 265. Controls are the game's 28 (`kMaxControlBinds`), bound per device:
//! keyboard, mouse (given and answered from 256 by the commands), joystick and gamepad (an
//! `XINPUT_GAMEPAD` button mask, through the exe's own conversion `00a24080`).

use super::{console_print, double_words, is_console_mode, Read};
use crate::input::{self, MAX_MACROS, MOUSE_BUTTON_OFFSET};
use crate::table::{Command, Param, Returns};
use crate::vars::{self, Key, Value};
use crate::{handled, CommandArgs, EvalArgs};
use engine_core::prelude::*;

const P_INT: Param = Param {
    name: "int",
    kind: 0x01,
    optional: false,
};
const P_OPT_INT: Param = Param {
    name: "int",
    kind: 0x01,
    optional: true,
};
/// `kParams_OneInt`, `kParams_OneOptionalInt`, `kParams_OneInt_OneOptionalInt`,
/// `kParams_TwoInts`, `kParams_TwoInts_OneOptionalInt` (xNVSE).
const ONE_INT: &[Param] = &[P_INT];
const ONE_OPT_INT: &[Param] = &[P_OPT_INT];
const ONE_INT_ONE_OPT_INT: &[Param] = &[P_INT, P_OPT_INT];
const TWO_INTS: &[Param] = &[P_INT, P_INT];
const TWO_INTS_ONE_OPT_INT: &[Param] = &[P_INT, P_INT, P_OPT_INT];

/// `g_OSInputGlobals` (xNVSE, `0x011F35CC`): the exe's `Controls` object, and its bindings
/// (`keyBinds`, `+0x1B94`: four devices of 28 bytes, `0xFF` unbound).
const OS_INPUT_GLOBALS: u32 = 0x011f_35cc;
const KEY_BINDS: u32 = 0x1b94;
/// `kMaxControlBinds`.
pub const MAX_CONTROL_BINDS: u32 = 0x1c;
/// `kControlType_*`.
pub const KEYBOARD: u32 = 0;
pub const MOUSE: u32 = 1;
pub const JOYSTICK: u32 = 2;
pub const GAMEPAD: u32 = 3;
/// The exe's gamepad binding to `XINPUT_GAMEPAD` mask (`BS2DX` (xNVSE): `0xA24080`, `thiscall`
/// on the `Controls` object).
const BINDING_TO_MASK: u32 = 0x00a2_4080;

/// `XINPUT_GAMEPAD_*` button masks (XInput.h) and xNVSE's `XINPUT_GAMEPAD_GUIDE`.
mod pad {
    pub const DPAD_UP: u32 = 0x0001;
    pub const DPAD_DOWN: u32 = 0x0002;
    pub const DPAD_LEFT: u32 = 0x0004;
    pub const DPAD_RIGHT: u32 = 0x0008;
    pub const START: u32 = 0x0010;
    pub const BACK: u32 = 0x0020;
    pub const LEFT_THUMB: u32 = 0x0040;
    pub const RIGHT_THUMB: u32 = 0x0080;
    pub const LEFT_SHOULDER: u32 = 0x0100;
    pub const RIGHT_SHOULDER: u32 = 0x0200;
    pub const GUIDE: u32 = 0x0400;
    pub const A: u32 = 0x1000;
    pub const B: u32 = 0x2000;
    pub const X: u32 = 0x4000;
    pub const Y: u32 = 0x8000;
}

/// A plain command: no reference, no condition eval, a number.
const fn cmd(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref: false,
        params,
        execute,
        eval: None,
        returns: Returns::Default,
    }
}

pub static COMMANDS: &[Command] = &[
    Command {
        eval: Some(is_key_pressed_eval),
        ..cmd(
            0x1453,
            "IsKeyPressed",
            "",
            "return if a dx scancode is down or up",
            ONE_INT_ONE_OPT_INT,
            is_key_pressed,
        )
    },
    cmd(
        0x1454,
        "TapKey",
        "tk",
        "Fakes a key press for one frame",
        ONE_INT,
        tap_key,
    ),
    cmd(
        0x1455,
        "HoldKey",
        "hk",
        "Fakes a key press indefinately",
        ONE_INT,
        hold_key,
    ),
    cmd(
        0x1456,
        "ReleaseKey",
        "rk",
        "Releases a key held down by HoldKey",
        ONE_INT,
        release_key,
    ),
    cmd(
        0x1457,
        "DisableKey",
        "dk",
        "Prevents a player from using a key",
        ONE_INT_ONE_OPT_INT,
        disable_key,
    ),
    cmd(
        0x1458,
        "EnableKey",
        "ek",
        "Reenables a key previously disabled with DisableKey",
        ONE_INT_ONE_OPT_INT,
        enable_key,
    ),
    cmd(
        0x1459,
        "GetNumKeysPressed",
        "gnkp",
        "Returns how many keyboard keys are currently being held down",
        ONE_OPT_INT,
        get_num_keys_pressed,
    ),
    cmd(
        0x145a,
        "GetKeyPress",
        "gkp",
        "Returns the scancode of the nth key which is currently being held down",
        ONE_INT_ONE_OPT_INT,
        get_key_press,
    ),
    cmd(
        0x145b,
        "GetNumMouseButtonsPressed",
        "gnmbp",
        "Returns how many mouse buttons are currently being held down",
        &[],
        get_num_mouse_buttons_pressed,
    ),
    cmd(
        0x145c,
        "GetMouseButtonPress",
        "gmbp",
        "Returns the code of the nth mouse button which is currently being held down",
        ONE_INT,
        get_mouse_button_press,
    ),
    cmd(
        0x145d,
        "GetControl",
        "",
        "Returns the key assigned to a control",
        ONE_INT_ONE_OPT_INT,
        get_control_cmd,
    ),
    cmd(
        0x145e,
        "GetAltControl",
        "",
        "Returns the mouse button assigned to a control",
        ONE_INT,
        get_alt_control,
    ),
    cmd(
        0x145f,
        "MenuTapKey",
        "mtk",
        "Fakes a key press for one frame in menu mode",
        ONE_INT,
        menu_tap_key,
    ),
    cmd(
        0x1460,
        "MenuHoldKey",
        "mhk",
        "Fakes a key press indefinately in menu mode",
        ONE_INT,
        menu_hold_key,
    ),
    cmd(
        0x1461,
        "MenuReleaseKey",
        "mrk",
        "Releases a key held down by MenuHoldKey",
        ONE_INT,
        menu_release_key,
    ),
    cmd(
        0x1462,
        "DisableControl",
        "dc",
        "disables the key and button bound to a control",
        ONE_INT_ONE_OPT_INT,
        disable_control,
    ),
    cmd(
        0x1463,
        "EnableControl",
        "ec",
        "enables the key and button assigned to a control",
        ONE_INT_ONE_OPT_INT,
        enable_control,
    ),
    cmd(
        0x1464,
        "TapControl",
        "tc",
        "taps the key or mouse button assigned to control",
        ONE_INT,
        tap_control,
    ),
    cmd(
        0x1465,
        "SetControl",
        "",
        "assigns a new keycode to the specified keyboard control",
        TWO_INTS_ONE_OPT_INT,
        set_control_cmd,
    ),
    cmd(
        0x1466,
        "SetAltControl",
        "",
        "assigns a new mouse button code to the specified mouse control",
        TWO_INTS,
        set_alt_control,
    ),
    cmd(
        0x1467,
        "SetIsControl",
        "",
        "sets a key as a custom control",
        TWO_INTS,
        set_is_control,
    ),
    cmd(
        0x1468,
        "IsControl",
        "",
        "returns 1 if key is a game control or 2 if a custom control",
        ONE_INT,
        is_control_cmd,
    ),
    cmd(
        0x1469,
        "IsKeyDisabled",
        "",
        "returns 1 if the key has been disabled by a script",
        ONE_INT,
        is_key_disabled,
    ),
    cmd(
        0x146a,
        "IsControlDisabled",
        "",
        "returns 1 if the control has been disabled by a script",
        ONE_INT,
        is_control_disabled,
    ),
    Command {
        eval: Some(is_control_pressed_eval),
        ..cmd(
            0x146b,
            "IsControlPressed",
            "",
            "returns 1 if the control is pressed",
            ONE_INT_ONE_OPT_INT,
            is_control_pressed,
        )
    },
    Command {
        returns: Returns::Array,
        ..cmd(
            0x1679,
            "GetDisabledKeys",
            "",
            "returns an array of all disabled key scancodes",
            &[],
            get_disabled_keys,
        )
    },
    Command {
        returns: Returns::Array,
        ..cmd(
            0x167b,
            "GetPressedKeys",
            "",
            "returns an array of all pressed key scancodes",
            ONE_OPT_INT,
            get_pressed_keys,
        )
    },
];

/// `IsKeycodeValid` (xNVSE): a keyboard key or mouse button (not the wheel), not `0xFF`.
fn is_keycode_valid(id: u32) -> bool {
    id < MAX_MACROS - 2 && id != 0xff
}

fn key_pressed(e: &mut Engine, keycode: u32, flags: u32) -> bool {
    input::state(e).is_key_pressed(keycode, flags)
}

/// The binding bytes of device `kind` (`globs->keyBinds[kind]`).
fn binds(e: &Engine, kind: u32) -> u32 {
    e.mem
        .u32(OS_INPUT_GLOBALS)
        .wrapping_add(KEY_BINDS)
        .wrapping_add(MAX_CONTROL_BINDS.wrapping_mul(kind))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp: BS2DX.
fn bs2dx(e: &mut Engine, binding: u32) -> u32 {
    let globs = e.mem.u32(OS_INPUT_GLOBALS);
    e.call(BINDING_TO_MASK, &[globs, binding]).u32()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp: DX2BS.
/// An `XINPUT_GAMEPAD` button mask to the game's gamepad binding (30 for any other).
fn dx2bs(mask: u32) -> u8 {
    match mask {
        pad::DPAD_UP => 1,
        pad::DPAD_DOWN => 2,
        pad::DPAD_RIGHT => 4,
        pad::DPAD_LEFT => 5,
        pad::START => 6,
        pad::BACK => 7,
        pad::LEFT_THUMB => 8,
        pad::RIGHT_THUMB => 9,
        pad::A => 10,
        pad::B => 11,
        pad::X => 12,
        pad::Y => 13,
        pad::RIGHT_SHOULDER => 14,
        pad::LEFT_SHOULDER => 15,
        _ => 30,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// IsDisallowedControllerButton.
fn is_disallowed_controller_button(mask: u32) -> bool {
    matches!(
        mask,
        pad::DPAD_UP
            | pad::DPAD_DOWN
            | pad::DPAD_RIGHT
            | pad::DPAD_LEFT
            | pad::START
            | pad::BACK
            | pad::GUIDE
    )
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp: GetControl.
/// The key bound to `control` on device `kind`: the binding byte (the mouse's from 256, the
/// gamepad's as a button mask); `0xFF` unbound, past the 28 controls or for another device.
pub fn get_control(e: &mut Engine, control: u32, kind: u32) -> u32 {
    if control >= MAX_CONTROL_BINDS {
        return 0xff;
    }
    let bind = |e: &Engine, kind: u32| u32::from(e.mem.u8(binds(e, kind).wrapping_add(control)));
    match kind {
        KEYBOARD | JOYSTICK => bind(e, kind),
        MOUSE => {
            let b = bind(e, MOUSE);
            if b != 0xff {
                b + MOUSE_BUTTON_OFFSET
            } else {
                b
            }
        }
        GAMEPAD => {
            let b = bind(e, GAMEPAD);
            bs2dx(e, b)
        }
        _ => 0xff,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp: SetControl.
/// Binds `keycode` to `control` on device `kind` (0 to 3); a control that had the key gets
/// this control's old key. A gamepad's start, back, guide and pad directions are refused.
pub fn set_control(e: &mut Engine, control: u32, kind: u32, keycode: u32) {
    if control >= MAX_CONTROL_BINDS {
        return;
    }
    let at = binds(e, kind);
    let keycode = if kind == GAMEPAD {
        if is_disallowed_controller_button(keycode) {
            return;
        }
        u32::from(dx2bs(keycode))
    } else if keycode >= 0x100 {
        keycode - 0x100
    } else {
        keycode
    };
    for i in 0..MAX_CONTROL_BINDS {
        if u32::from(e.mem.u8(at + i)) == keycode {
            let old = e.mem.u8(at + control);
            e.mem.set_u8(at + i, old);
            break;
        }
    }
    e.mem.set_u8(at + control, keycode as u8);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp: IsControl.
/// Whether some control's keyboard binding (a mouse binding for a key from 256) is `key`.
fn is_control(e: &Engine, key: u32) -> bool {
    let (binds, key) = if key >= 0x100 {
        (binds(e, MOUSE), key - 0x100)
    } else {
        (binds(e, KEYBOARD), key)
    };
    (0..MAX_CONTROL_BINDS).any(|i| u32::from(e.mem.u8(binds + i)) == key)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_IsKeyPressed_Execute.
fn is_key_pressed(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let pressed = key_pressed(e, read.int(0, 0), read.int(1, 0));
        a.set_result(e, f64::from(u8::from(pressed)));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_IsKeyPressed_Eval.
fn is_key_pressed_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    let pressed = key_pressed(e, a.arg1, a.arg2);
    e.mem.set_f64(a.result, f64::from(u8::from(pressed)));
    handled(true)
}

/// The first argument as a key code (0 when it was not read), and whether the read succeeded.
fn one_key(e: &mut Engine, a: &CommandArgs) -> Option<u32> {
    let read = Read::of(e, a);
    read.ok.then(|| read.int(0, 0))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp: Cmd_TapKey_Execute.
fn tap_key(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(key) = one_key(e, &a) {
        input::state(e).tap_key(key);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp: Cmd_HoldKey_Execute.
fn hold_key(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(key) = one_key(e, &a) {
        input::state(e).set_key_held_state(key, true);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_ReleaseKey_Execute.
fn release_key(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(key) = one_key(e, &a) {
        input::state(e).set_key_held_state(key, false);
    }
    handled(true)
}

/// `DisableKey` and `EnableKey`: the key and the mask (0 when not given: both).
fn set_key_disable(e: &mut Engine, w: &[u32], disable: bool) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let (key, mask) = (read.int(0, 0), read.int(1, 0));
        input::state(e).set_key_disable_state(key, disable, mask);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_DisableKey_Execute.
fn disable_key(e: &mut Engine, w: &[u32]) -> Ret {
    set_key_disable(e, w, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_EnableKey_Execute.
fn enable_key(e: &mut Engine, w: &[u32]) -> Ret {
    set_key_disable(e, w, false)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_IsKeyDisabled_Execute.
fn is_key_disabled(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(key) = one_key(e, &a) {
        let disabled = input::state(e).is_key_disabled(key);
        a.set_result(e, f64::from(u8::from(disabled)));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetNumKeysPressed_Execute.
/// The keys and mouse buttons down (`flags` as `IsKeyPressed`'s); printed in the console.
fn get_num_keys_pressed(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let flags = read.int(0, 0);
    let s = input::state(e);
    let count = (0..256u32)
        .filter(|&d| is_keycode_valid(d) && s.is_key_pressed(d, flags))
        .count() as u32;
    a.set_result(e, f64::from(count));
    if is_console_mode(e) {
        console_print(e, "keysPressed: %d", &[count]);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetKeyPress_Execute.
/// The `n`th key down (from 0), -1 for none.
fn get_key_press(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let (mut count, flags) = (read.int(0, 0), read.int(1, 0));
    let s = input::state(e);
    let mut found = None;
    for d in 0..256u32 {
        if is_keycode_valid(d) && s.is_key_pressed(d, flags) {
            if count == 0 {
                found = Some(d);
                break;
            }
            count = count.wrapping_sub(1);
        }
    }
    if let Some(d) = found {
        a.set_result(e, f64::from(d));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetNumMouseButtonsPressed_Execute.
fn get_num_mouse_buttons_pressed(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let s = input::state(e);
    let count = (MOUSE_BUTTON_OFFSET..MAX_MACROS - 2)
        .filter(|&d| is_keycode_valid(d) && s.is_key_pressed(d, 0))
        .count() as u32;
    a.set_result(e, f64::from(count));
    if is_console_mode(e) {
        console_print(e, "buttonsPressed: %d", &[count]);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetMouseButtonPress_Execute.
/// The `n`th mouse button down (256 up), -1 for none.
fn get_mouse_button_press(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let Some(mut count) = one_key(e, &a) else {
        return handled(true);
    };
    let s = input::state(e);
    let mut found = None;
    for d in MOUSE_BUTTON_OFFSET..MAX_MACROS - 2 {
        if s.is_key_pressed(d, 0) {
            if count == 0 {
                found = Some(d);
                break;
            }
            count = count.wrapping_sub(1);
        }
    }
    if let Some(d) = found {
        a.set_result(e, f64::from(d));
    }
    handled(true)
}

/// `MenuTapKey`, `MenuHoldKey`, `MenuReleaseKey`: a keyboard key queued for the buffered reads.
fn menu_key(e: &mut Engine, w: &[u32], down: bool, tap: bool) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(key) = one_key(e, &a) {
        if key < 256 {
            let time = input::tick_count(e);
            let caps = super::array::fix_on(e, super::array::SIZE_CAPS);
            input::state(e).buffered_key(key, time, down, tap, caps);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_MenuTapKey_Execute.
fn menu_tap_key(e: &mut Engine, w: &[u32]) -> Ret {
    menu_key(e, w, true, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_MenuHoldKey_Execute.
fn menu_hold_key(e: &mut Engine, w: &[u32]) -> Ret {
    menu_key(e, w, true, false)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_MenuReleaseKey_Execute.
fn menu_release_key(e: &mut Engine, w: &[u32]) -> Ret {
    menu_key(e, w, false, false)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// SetControlDisableState_Execute.
/// `DisableControl` and `EnableControl`: the control's mask (all when not given) set or cleared,
/// and its keyboard key and mouse button disabled or enabled with it (an unbound one is key
/// `0xFF`, as xNVSE passes it on).
fn set_control_disable(e: &mut Engine, w: &[u32], disable: bool) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let (control, mask) = (read.int(0, 0), read.int(1, input::DISABLE_ALL));
    if control < MAX_CONTROL_BINDS {
        let field = &mut input::state(e).disabled_controls[control as usize];
        if disable {
            *field |= mask as u8;
        } else {
            *field &= !(mask as u8);
        }
        let key = get_control(e, control, KEYBOARD);
        input::state(e).set_key_disable_state(key, disable, mask);
        let button = get_control(e, control, MOUSE);
        input::state(e).set_key_disable_state(button, disable, mask);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_DisableControl_Execute.
fn disable_control(e: &mut Engine, w: &[u32]) -> Ret {
    set_control_disable(e, w, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_EnableControl_Execute.
fn enable_control(e: &mut Engine, w: &[u32]) -> Ret {
    set_control_disable(e, w, false)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_IsControlDisabled_Execute.
fn is_control_disabled(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(control) = one_key(e, &a) {
        if control < MAX_CONTROL_BINDS {
            let mask = input::state(e).disabled_controls[control as usize];
            a.set_result(e, f64::from(u8::from(mask != 0)));
        }
    }
    handled(true)
}

/// A binding as the getters answer it: -1 for `0xFF`.
fn binding_result(binding: u32) -> f64 {
    if binding == 0xff {
        -1.0
    } else {
        f64::from(binding)
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetControl_Execute.
/// The key of a control on a device (the keyboard when not given), -1 unbound; printed in the
/// console. A device past the gamepad answers -1.
fn get_control_cmd(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let read = Read::of(e, &a);
    let (control, device) = (read.int(0, 0), read.int(1, KEYBOARD));
    if !read.ok || device > GAMEPAD {
        return handled(true);
    }
    let result = binding_result(get_control(e, control, device));
    a.set_result(e, result);
    if is_console_mode(e) {
        let [lo, hi] = double_words(result);
        console_print(e, "GetControl %d >> %.0f", &[control, lo, hi]);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetAltControl_Execute.
/// The mouse button of a control: the binding byte (`UInt8 ctrl = GetControl(...)` drops the
/// 256 the mouse's codes carry), -1 unbound; printed in the console.
fn get_alt_control(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, -1.0);
    let Some(control) = one_key(e, &a) else {
        return handled(true);
    };
    let binding = get_control(e, control, MOUSE) as u8;
    let result = binding_result(u32::from(binding));
    a.set_result(e, result);
    if is_console_mode(e) {
        let [lo, hi] = double_words(result);
        console_print(e, "GetAltControl %d >> %.0f", &[control, lo, hi]);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_IsControlPressed_Eval.
/// Whether the control's keyboard key, else its mouse button, is down (`flags` as
/// `IsKeyPressed`'s).
fn control_pressed(e: &mut Engine, control: u32, flags: u32) -> bool {
    let key = get_control(e, control, KEYBOARD);
    if key != 0xff && key_pressed(e, key, flags) {
        return true;
    }
    let button = get_control(e, control, MOUSE);
    button != 0xff && key_pressed(e, button, flags)
}

fn is_control_pressed_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    let pressed = control_pressed(e, a.arg1, a.arg2);
    if pressed {
        e.mem.set_f64(a.result, 1.0);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_IsControlPressed_Execute.
fn is_control_pressed(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let pressed = control_pressed(e, read.int(0, 0), read.int(1, 0));
        a.set_result(e, f64::from(u8::from(pressed)));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_TapControl_Execute.
/// Taps the control's mouse button, else its keyboard key; 1 when one is bound.
fn tap_control(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let Some(control) = one_key(e, &a) else {
        return handled(true);
    };
    let mut key = get_control(e, control, MOUSE);
    if !is_keycode_valid(key) {
        key = get_control(e, control, KEYBOARD);
    }
    if is_keycode_valid(key) {
        a.set_result(e, 1.0);
        input::state(e).tap_key(key);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_SetControl_Execute.
/// `SetControl control key [device]` (the keyboard when not given; a device past the gamepad
/// does nothing).
fn set_control_cmd(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    let (control, key, device) = (read.int(0, 0), read.int(1, 0), read.int(2, KEYBOARD));
    if read.ok && device <= GAMEPAD {
        set_control(e, control, device, key);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_SetAltControl_Execute.
fn set_alt_control(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        set_control(e, read.int(0, 0), MOUSE, read.int(1, 0));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_SetIsControl_Execute.
/// Registers (`1`, the default) or unregisters a key as a custom control of the calling
/// script's mod.
fn set_is_control(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let mod_index = vars::script_mod_index(e, a.script);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    let (key, is_control) = (read.int(0, 0), read.int(1, 1));
    if read.ok && key < MAX_MACROS {
        let registered = &mut input::state(e).registered_controls;
        if is_control != 0 {
            registered.entry(key).or_default().insert(mod_index);
        } else if let Some(mods) = registered.get_mut(&key) {
            mods.remove(&mod_index);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_IsControl_Execute.
/// 1 for a key some control is bound to, 2 for a key a mod registered, 3 for both, else 0.
fn is_control_cmd(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let Some(key) = one_key(e, &a) else {
        return handled(true);
    };
    let mut result = f64::from(u8::from(is_control(e, key)));
    let custom = input::state(e)
        .registered_controls
        .get(&key)
        .is_some_and(|mods| !mods.is_empty());
    if custom {
        result += 2.0;
    }
    a.set_result(e, result);
    handled(true)
}

/// A new array of the keys `keep` keeps, in order, as `GetDisabledKeys` and `GetPressedKeys`
/// make it.
fn key_array(e: &mut Engine, a: &CommandArgs, keep: impl Fn(&input::State, u32) -> bool) {
    let modi = vars::script_mod_index(e, a.script);
    let array = vars::create_array(e, vars::KEY_NUMBER, true, modi);
    a.set_result(e, f64::from(array));
    let keys: Vec<u32> = {
        let s = input::state(e);
        (0..MAX_MACROS - 2)
            .filter(|&d| is_keycode_valid(d) && keep(s, d))
            .collect()
    };
    for (i, d) in keys.into_iter().enumerate() {
        vars::set_element(
            e,
            array,
            &Key::Number(i as f64),
            Value::Number(f64::from(d)),
        );
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetDisabledKeys_Execute.
fn get_disabled_keys(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    key_array(e, &a, |s, d| s.is_key_disabled(d));
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Input.cpp:
// Cmd_GetPressedKeys_Execute.
fn get_pressed_keys(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let flags = read.int(0, 0);
    key_array(e, &a, |s, d| s.is_key_pressed(d, flags));
    handled(true)
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
