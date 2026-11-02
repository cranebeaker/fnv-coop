//! `(unplaced)/run_00a237f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The input side of the `Controls` object (the engine map names its
//! neighbours `Controls::Poll` `00a23010`, `Controls::ClearKeystrokes`
//! `00a237b0` and `Controls::ClearUserActions` `00a253d0`; the PC layout is
//! not the Xbox PDB's, so the fields below are named by what the code does
//! with them, at their PC offsets):
//!
//! - `+0x08` an interface pointer whose slot `0x0c` the code calls as
//!   `CreateDevice(guid, out, 0)` and slot `0x10` as
//!   `EnumDevices(0, callback, context, flags)` (DirectInput shapes);
//! - `+0x0c` up to eight joystick device pointers, `+0x2c` the keyboard
//!   device, `+0x30` the mouse device (slots `0x08` Release, `0x0c`
//!   GetCapabilities, `0x10` EnumObjects, `0x20` Unacquire, `0x28`
//!   GetDeviceData, `0x2c` SetDataFormat, `0x34` SetCooperativeLevel, as the
//!   calls here use them);
//! - `+0x34 + n * 0xa0` the current and `+0x84 + n * 0xa0` the previous
//!   0x50-byte state of joystick `n` (button bytes at `+0x30`, the first
//!   hat at `+0x20`: `+0x64` and `+0x54` for the first state);
//! - `+0x18f8` the keyboard state (256 bytes, bit 7 = down) and `+0x19f8`
//!   the previous one; `+0x1b24` the mouse state (x, y, wheel, 8 button
//!   bytes) and `+0x1b38` the previous one;
//! - `+0x1b94` the control bindings: four devices (keyboard, mouse,
//!   joystick, gamepad) of `0x1c` control slots each, `0xff` = unbound.
//!
//! The gamepad globals at `011f35a8` are the `XINPUT_STATE` that `Poll`
//! fills with `XInputGetState`, with the previous poll's copy `0x10` bytes
//! later.
//!
//! Not translated: nothing is left out; the `HRESULT`s the device calls
//! return are ignored by the exe except where noted.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `Controls` (engine map), PC layout. Only the fields these functions
    /// use are declared; the size is a lower bound covering the last binding
    /// byte this unit writes (`+0x1c03`), not the object's real size.
    pub struct Controls: 0x1c08 {
        /// Flag word: bit 0 is set while joysticks are enumerated, bit 3
        /// while the mouse is in the exclusive cooperative mode.
        0x04 flags: u32,
        /// DirectInput-shaped interface pointer (see the module note).
        0x08 direct_input: Ptr,
        /// Keyboard device pointer.
        0x2c keyboard_device: Ptr,
        /// Mouse device pointer.
        0x30 mouse_device: Ptr,
        /// The first joystick's hat (`rgdwPOV[0]` of its 0x50-byte state),
        /// `-1` when centred.
        0x54 first_hat: i32,
        /// Number of joystick devices found.
        0x18f4 joystick_count: i32,
        /// Mouse state word 0 (x movement).
        0x1b24 mouse_x: i32,
        /// Mouse state word 1 (y movement).
        0x1b28 mouse_y: i32,
        /// Mouse state word 2 (wheel).
        0x1b2c mouse_wheel: i32,
        /// Non-zero when the first two mouse buttons are swapped.
        0x1b4c mouse_buttons_swapped: u32,
        /// Control slot being rebound (`fn_00a251e0`).
        0x1b8c rebinding_slot: i32,
        /// Written by `fn_00a251e0` (1 while no key was caught).
        0x1b90 nothing_caught: u8,
        /// Written by `fn_00a251e0` (1 once a key was caught).
        0x1b91 something_caught: u8,
        /// Control flags word (bit 3 is toggled by `fn_00a24660`).
        0x1b92 control_flags: u16,
    }
}

/// `+0x18f8`: keyboard state, 256 bytes.
const KEYBOARD_STATE: u32 = 0x18f8;
/// `+0x19f8`: the previous keyboard state.
const KEYBOARD_STATE_PREVIOUS: u32 = 0x19f8;
/// `+0x1b30`: the mouse button bytes of the current state.
const MOUSE_BUTTONS: u32 = 0x1b30;
/// `+0x1b44`: the mouse button bytes of the previous state.
const MOUSE_BUTTONS_PREVIOUS: u32 = 0x1b44;
/// `+0x1b58`: eight bytes `Poll` clears and the mouse query reads for
/// buttons 0 to 7.
const MOUSE_BUTTON_EXTRA: u32 = 0x1b58;
/// `+0x1b08`: the word `fn_00a23a30` returns.
const WORD_1B08: u32 = 0x1b08;
/// `+0x34`: the joysticks' current states.
const JOYSTICK_STATE: u32 = 0x34;
/// `+0x64`: the button bytes of the first joystick's current state.
const JOYSTICK_BUTTONS: u32 = 0x64;
/// `+0x534`: the device-instance record copied for each joystick.
const JOYSTICK_INSTANCES: u32 = 0x534;
/// Size of one joystick instance record.
const JOYSTICK_INSTANCE_SIZE: u32 = 0x244;
/// `+0x1754`: the capabilities record of each joystick (`0x2c` bytes each).
const JOYSTICK_CAPABILITIES: u32 = 0x1754;
/// `+0x18b4`: two words per joystick filled by the object enumeration.
const JOYSTICK_OBJECT_MASKS: u32 = 0x18b4;
/// `+0x0c`: the joystick device pointers.
const JOYSTICK_DEVICES: u32 = 0x0c;
/// `+0x1b94`: the binding table (four devices of `0x1c` slots).
const BINDINGS: u32 = 0x1b94;
/// Slots per binding device.
const BINDING_SLOTS: i32 = 0x1c;
/// `+0x1be8`: the binding bytes of the gamepad device (device 3).
const GAMEPAD_BINDINGS: u32 = 0x1be8;
/// Marks an unbound slot.
const UNBOUND: u8 = 0xff;

/// The `XINPUT_STATE` the poll fills (`011f35a8`), and its fields.
const GAMEPAD_BUTTONS: u32 = 0x011f_35ac;
const GAMEPAD_LEFT_TRIGGER: u32 = 0x011f_35ae;
const GAMEPAD_RIGHT_TRIGGER: u32 = 0x011f_35af;
const GAMEPAD_THUMB_LX: u32 = 0x011f_35b0;
const GAMEPAD_THUMB_LY: u32 = 0x011f_35b2;
/// The previous poll's copy of the same fields.
const GAMEPAD_BUTTONS_PREVIOUS: u32 = 0x011f_35bc;
const GAMEPAD_LEFT_TRIGGER_PREVIOUS: u32 = 0x011f_35be;
const GAMEPAD_RIGHT_TRIGGER_PREVIOUS: u32 = 0x011f_35bf;
/// Non-zero while a gamepad is connected and used.
const GAMEPAD_ACTIVE: u32 = 0x011f_35c8;

/// Thumb-stick threshold of the "pushed" tests (`0x1ea9`).
const THUMB_THRESHOLD: i32 = 0x1ea9;

/// `GetActiveWindow` import slot (named by the exe's import table).
const IMPORT_GET_ACTIVE_WINDOW: u32 = 0x00fd_f2fc;
/// The function that works out the interface language (`1` to `4`, other
/// values for the default pair).
const GET_LANGUAGE: u32 = 0x00a2_2cb0;
/// The other pad type's state test (`00a234c0`: joystick, key, mode) and its
/// bound-key getter (`00a235f0`: joystick, slot).
const JOYSTICK_BUTTON_TEST: u32 = 0x00a2_34c0;
const JOYSTICK_BOUND_KEY: u32 = 0x00a2_35f0;
/// Joystick axis range setter (`00a236f0`: joystick, minimum, maximum) and
/// dead-zone setter (`00a23750`: joystick, `float`).
const JOYSTICK_SET_RANGE: u32 = 0x00a2_36f0;
const JOYSTICK_SET_DEAD_ZONE: u32 = 0x00a2_3750;
/// The `float` the dead zone is set to (`0.1f`).
const JOYSTICK_DEAD_ZONE: u32 = 0x0101_e2bc;
/// Integer absolute value (`00ec7d40`).
const ABSOLUTE_VALUE: u32 = 0x00ec_7d40;
/// `_ftol2_sse` (`00ec62c0`): the value is passed as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// The `double` `0.5` the hat arithmetic multiplies by.
const HALF: u32 = 0x0101_1588;
/// `_memcpy`, `_memset`, `_memcmp` (cdecl).
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;
const MEMCMP: u32 = 0x00ec_4835;

/// The GUID `CreateDevice` is given for the mouse (`01100f74`).
const MOUSE_GUID: u32 = 0x0110_0f74;
/// The data format `SetDataFormat` is given for the mouse (`01100aac`).
const MOUSE_DATA_FORMAT: u32 = 0x0110_0aac;
/// The data format `SetDataFormat` is given for a joystick (`01100ebc`).
const JOYSTICK_DATA_FORMAT: u32 = 0x0110_0ebc;
/// The six object GUIDs the object enumeration callback compares against
/// (`01100ee4` to `01100f34`, `0x10` bytes apart).
const OBJECT_GUIDS: [u32; 6] = [
    0x0110_0ee4,
    0x0110_0ef4,
    0x0110_0f04,
    0x0110_0f14,
    0x0110_0f24,
    0x0110_0f34,
];
/// The object enumeration callback's address, as `EnumObjects` receives it.
const OBJECT_CALLBACK: u32 = 0x00a2_4a30;
/// The device enumeration callback's address, as `EnumDevices` receives it.
const DEVICE_CALLBACK: u32 = 0x00a2_4830;

/// The 256-byte tables of `fn_00a238a0` (`011a7440` to `011a7d40`): the
/// default pair and one pair per language 1 to 4 (first table when the
/// second argument is zero, second table otherwise).
const TABLE_DEFAULT: [u32; 2] = [0x011a_7440, 0x011a_7540];
const TABLE_LANGUAGE_1: [u32; 2] = [0x011a_7640, 0x011a_7740];
const TABLE_LANGUAGE_2: [u32; 2] = [0x011a_7840, 0x011a_7940];
const TABLE_LANGUAGE_3: [u32; 2] = [0x011a_7a40, 0x011a_7b40];
const TABLE_LANGUAGE_4: [u32; 2] = [0x011a_7c40, 0x011a_7d40];

/// Address of byte `index` of the table at `offset` in `this`.
fn field_addr(this: Ptr<Controls>, offset: u32, index: i32) -> u32 {
    this.addr().wrapping_add(offset).wrapping_add(index as u32)
}

/// Address of binding `slot` of `device`.
fn binding_addr(this: Ptr<Controls>, device: i32, slot: i32) -> u32 {
    field_addr(
        this,
        BINDINGS,
        device.wrapping_mul(BINDING_SLOTS).wrapping_add(slot),
    )
}

fn binding(e: &Engine, this: Ptr<Controls>, device: i32, slot: i32) -> u8 {
    e.mem.u8(binding_addr(this, device, slot))
}

/// Sets bit 7 of the byte at `address`.
fn press(e: &mut Engine, address: u32) {
    let value = e.mem.u8(address);
    e.mem.set_u8(address, value | 0x80);
}

// Translated from 00a237b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `Controls::ClearKeystrokes` (engine map; `thiscall`, no stack words): with a keyboard device
/// (+0x2c), flushes its buffer: `GetDeviceData` (slot `0x28`) with the record size 0x14, no
/// records, a count of 0xFFFFFFFF (`INFINITE`) and no flags. Answers what is left in `EAX`:
/// that call's answer, or `this` without a keyboard.
pub fn controls_clear_keystrokes(e: &mut Engine, this: Ptr<Controls>) -> u32 {
    let keyboard = e.get(this, Controls::keyboard_device).addr();
    if keyboard == 0 {
        return this.addr();
    }
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 0xffff_ffff);
        e.vcall(keyboard, 0x28, &args![0x14u32, 0u32, count, 0u32])
            .u32()
    })
}

// Translated from 00a237f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads one event from the keyboard device (`fn_00a23820`); true when it
/// was a key going down. `out` receives the event's key code whatever the
/// kind of event.
pub fn fn_00a237f0(e: &mut Engine, this: Ptr<Controls>, out: Ptr) -> bool {
    e.get(this, Controls::keyboard_device) != Ptr::NULL && fn_00a23820(e, this, out) == 1
}

// Translated from 00a23820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the keyboard device (slot `0x28`, a one-event read of `0x14` bytes
/// with a count of 1) for one buffered event. With an event: stores its
/// first word (the key) at `out` and returns 1 for a key going down (bit 7
/// of the second word) or 2 for a key going up. 0 when there is no keyboard,
/// the read failed or no event came.
pub fn fn_00a23820(e: &mut Engine, this: Ptr<Controls>, out: Ptr) -> u32 {
    let keyboard = e.get(this, Controls::keyboard_device);
    if keyboard == Ptr::NULL {
        return 0;
    }
    // The event record (0x14 bytes) and the count word.
    e.with_stack(0x18, |e, block| {
        let count = block.addr() + 0x14;
        e.mem.set_u32(count, 1);
        let result = e
            .vcall(
                keyboard.addr(),
                0x28,
                &args![0x14u32, block, Ptr::<()>::new(count), 0u32],
            )
            .u32();
        if result == 0 && e.mem.u32(count) != 0 {
            let key = e.mem.u32(block.addr());
            let flags = e.mem.u32(block.addr() + 4);
            e.mem.set_u32(out.addr(), key);
            2 - u32::from(flags & 0x80 != 0)
        } else {
            0
        }
    })
}

// Translated from 00a238a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up byte `index` (0 to 255) in one of the nine 256-byte tables at
/// `011a7440`..`011a7d40`. `variant == 0xff` reads the table at `011a7540`;
/// otherwise the table depends on the interface language (`00a22cb0`:
/// 1 to 4 select a language pair, anything else the default pair) and on
/// whether `variant` is zero. An index outside 0..=255 gives `-1`. `this` is
/// not read.
pub fn fn_00a238a0(e: &mut Engine, _unused_0: u32, index: i32, variant: i32) -> u32 {
    if !(0..=0xff).contains(&index) {
        return u32::MAX;
    }
    if variant == 0xff {
        return u32::from(e.mem.u8(TABLE_DEFAULT[1] + index as u32));
    }
    let pair = match e.call(GET_LANGUAGE, &args![]).i32() {
        1 => TABLE_LANGUAGE_1,
        2 => TABLE_LANGUAGE_2,
        3 => TABLE_LANGUAGE_3,
        4 => TABLE_LANGUAGE_4,
        _ => TABLE_DEFAULT,
    };
    let table = if variant == 0 { pair[0] } else { pair[1] };
    u32::from(e.mem.u8(table + index as u32))
}

// Translated from 00a239e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Mouse movement word `axis`: 1 = x (`+0x1b24`), 2 = y (`+0x1b28`),
/// 3 = wheel (`+0x1b2c`); 0 for any other value.
pub fn fn_00a239e0(e: &mut Engine, this: Ptr<Controls>, axis: i32) -> i32 {
    match axis {
        1 => e.get(this, Controls::mouse_x),
        2 => e.get(this, Controls::mouse_y),
        3 => e.get(this, Controls::mouse_wheel),
        _ => 0,
    }
}

// Translated from 00a23a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x1b08`.
pub fn fn_00a23a30(e: &mut Engine, this: Ptr<Controls>) -> u32 {
    e.mem.u32(this.addr() + WORD_1B08)
}

// Translated from 00a23a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Mouse query. `button` 8 / 9: the wheel moved up / down (the wheel word is
/// positive / negative). Buttons 0 to 7: with the swap flag (`+0x1b4c`) set,
/// buttons 0 and 1 trade places; then by `mode`: 0 = held (bit 7 of the
/// button byte), 1 = went down since the previous state, 2 = went up,
/// 3 = changed, 4 = the extra byte at `+0x1b58 + button` is set (0 to 7
/// only). Any other `mode` is false.
pub fn fn_00a23a50(e: &mut Engine, this: Ptr<Controls>, button: i32, mode: u32) -> bool {
    if button == 8 || button == 9 {
        let wheel = e.get(this, Controls::mouse_wheel);
        return (button == 8 && wheel > 0) || (button == 9 && wheel < 0);
    }
    let swapped = e.get(this, Controls::mouse_buttons_swapped) != 0;
    let button = match button {
        0 if swapped => 1,
        1 if swapped => 0,
        other => other,
    };
    let now = e.mem.u8(field_addr(this, MOUSE_BUTTONS, button)) & 0x80 != 0;
    let before = e.mem.u8(field_addr(this, MOUSE_BUTTONS_PREVIOUS, button)) & 0x80 != 0;
    match mode {
        0 => now,
        1 => !before && now,
        2 => !now && before,
        3 => now != before,
        4 => {
            (0..=7).contains(&button) && e.mem.u8(field_addr(this, MOUSE_BUTTON_EXTRA, button)) != 0
        }
        _ => false,
    }
}

// Translated from 00a23c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-creates the mouse device. If there is one: Unacquire (slot `0x20`)
/// and Release (slot `0x08`) it, then `CreateDevice` (slot `0x0c` of the
/// interface at `+0x08`) with the GUID at `01100f74`. On success the new
/// device is stored, `SetCooperativeLevel` (slot `0x34`) is called with the
/// active window and 6 (exclusive; flag bit 3 set) when `exclusive` is
/// non-zero or 5 (flag bit 3 cleared), and `SetDataFormat` (slot `0x2c`)
/// with the table at `01100aac`. The cooperative-level result is only
/// compared against three error codes whose branches change nothing.
pub fn fn_00a23c00(e: &mut Engine, this: Ptr<Controls>, exclusive: u8) {
    let old = e.get(this, Controls::mouse_device);
    if old == Ptr::NULL {
        return;
    }
    e.vcall(old.addr(), 0x20, &args![]);
    e.vcall(old.addr(), 0x08, &args![]);
    let interface = e.get(this, Controls::direct_input);
    let (result, created) = e.with_stack(4, |e, out| {
        let result = e
            .vcall(interface.addr(), 0xc, &args![MOUSE_GUID, out, 0u32])
            .u32();
        (result, e.mem.u32(out.addr()))
    });
    if result != 0 {
        return;
    }
    let device = Ptr::<()>::new(created);
    e.set(this, Controls::mouse_device, device);
    let window = e.call(IMPORT_GET_ACTIVE_WINDOW, &args![]).u32();
    let flags = e.get(this, Controls::flags);
    if exclusive != 0 {
        e.vcall(device.addr(), 0x34, &args![window, 6u32]);
        e.set(this, Controls::flags, flags | 8);
    } else {
        e.vcall(device.addr(), 0x34, &args![window, 5u32]);
        e.set(this, Controls::flags, flags & !8);
    }
    e.vcall(device.addr(), 0x2c, &args![MOUSE_DATA_FORMAT]);
}

// Translated from 00a23d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks gamepad control `control` as pressed in the gamepad state (only
/// while a gamepad is active): 0x10 / 0x11 set the left / right trigger to
/// 255; 0x13 / 0x14 push the left stick's Y to `0x7fff` / `-0x7fff`; 0x16 /
/// 0x17 its X likewise; any other value below `0x1d` ORs its button mask
/// (`fn_00a24080`) into the button word.
pub fn fn_00a23d40(e: &mut Engine, this: Ptr<Controls>, control: i32) {
    if e.mem.u8(GAMEPAD_ACTIVE) == 0 {
        return;
    }
    match control {
        0x10 => e.mem.set_u8(GAMEPAD_LEFT_TRIGGER, 0xff),
        0x11 => e.mem.set_u8(GAMEPAD_RIGHT_TRIGGER, 0xff),
        0x13 => e.mem.set_u16(GAMEPAD_THUMB_LY, 0x7fff),
        0x14 => e.mem.set_u16(GAMEPAD_THUMB_LY, 0x8001),
        0x16 => e.mem.set_u16(GAMEPAD_THUMB_LX, 0x7fff),
        0x17 => e.mem.set_u16(GAMEPAD_THUMB_LX, 0x8001),
        _ => {
            if control < 0x1d {
                let mask = fn_00a24080(e, this.addr(), control as u32);
                let buttons = e.mem.u16(GAMEPAD_BUTTONS);
                e.mem.set_u16(GAMEPAD_BUTTONS, buttons | mask as u16);
            }
        }
    }
}

// Translated from 00a23e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gamepad query (false without an active gamepad). 0x13 / 0x14: the left
/// stick's Y is above `0x1ea9` / below `-0x1ea9`; 0x16 / 0x17 the same for X.
/// 0x10 / 0x11: the trigger byte, by `mode` (0 = held, 1 = went down since
/// the previous poll, 2 = went up, 3 = changed). Any other control: its
/// button mask (`fn_00a24080`) tested against the button word and the
/// previous one the same way.
pub fn fn_00a23e10(e: &mut Engine, this: Ptr<Controls>, control: i32, mode: u32) -> bool {
    if e.mem.u8(GAMEPAD_ACTIVE) == 0 {
        return false;
    }
    let mut result = false;
    let mut mask = 0u32;
    match control {
        0x13 => result = i32::from(e.mem.i16(GAMEPAD_THUMB_LY)) > THUMB_THRESHOLD,
        0x14 => result = i32::from(e.mem.i16(GAMEPAD_THUMB_LY)) < -THUMB_THRESHOLD,
        0x16 => result = i32::from(e.mem.i16(GAMEPAD_THUMB_LX)) > THUMB_THRESHOLD,
        0x17 => result = i32::from(e.mem.i16(GAMEPAD_THUMB_LX)) < -THUMB_THRESHOLD,
        _ => mask = fn_00a24080(e, this.addr(), control as u32),
    }
    let trigger = match control {
        0x10 => Some((GAMEPAD_LEFT_TRIGGER, GAMEPAD_LEFT_TRIGGER_PREVIOUS)),
        0x11 => Some((GAMEPAD_RIGHT_TRIGGER, GAMEPAD_RIGHT_TRIGGER_PREVIOUS)),
        _ => None,
    };
    if let Some((now, before)) = trigger {
        let now = e.mem.u8(now) != 0;
        let before = e.mem.u8(before) != 0;
        match mode {
            0 => result |= now,
            1 => result |= now && !before,
            2 => result |= !now && before,
            3 => result |= now != before,
            _ => {}
        }
    } else if !result {
        let now = u32::from(e.mem.u16(GAMEPAD_BUTTONS)) & mask;
        let before = u32::from(e.mem.u16(GAMEPAD_BUTTONS_PREVIOUS)) & mask;
        match mode {
            0 => result = now != 0,
            1 => result = before == 0 && now != 0,
            2 => result = now == 0 && before != 0,
            3 => result = now != before,
            _ => {}
        }
    }
    result
}

// Translated from 00a24080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `XINPUT_GAMEPAD` button mask of gamepad control `control`: 1 -> 1,
/// 2 -> 2, 4 -> 8, 5 -> 4, 6 -> 0x10, 7 -> 0x20, 8 -> 0x40, 9 -> 0x80,
/// 10 -> 0x1000, 11 -> 0x2000, 12 -> 0x4000, 13 -> 0x8000, 14 -> 0x200,
/// 15 -> 0x100; 0 for every other control. `this` is not read.
pub fn fn_00a24080(_e: &mut Engine, _unused_0: u32, control: u32) -> u32 {
    match control {
        1 => 1,
        2 => 2,
        4 => 8,
        5 => 4,
        6 => 0x10,
        7 => 0x20,
        8 => 0x40,
        9 => 0x80,
        10 => 0x1000,
        11 => 0x2000,
        12 => 0x4000,
        13 => 0x8000,
        14 => 0x200,
        15 => 0x100,
        _ => 0,
    }
}

// Translated from 00a24180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keyboard query for key code `key`: `mode` 0 = held (bit 7 of the key's
/// byte in the keyboard state), 1 = went down since the previous state,
/// 2 = went up, 3 = changed; any other `mode` is false.
pub fn fn_00a24180(e: &mut Engine, this: Ptr<Controls>, key: i32, mode: u32) -> bool {
    let now = e.mem.u8(field_addr(this, KEYBOARD_STATE, key)) & 0x80 != 0;
    let before = e.mem.u8(field_addr(this, KEYBOARD_STATE_PREVIOUS, key)) & 0x80 != 0;
    match mode {
        0 => now,
        1 => !before && now,
        2 => !now && before,
        3 => now != before,
        _ => false,
    }
}

// Translated from 00a24280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Presses control `control` (0 to 12) in the input state, from its
/// bindings: the first of keyboard, mouse and joystick bindings that is not
/// unbound (all unbound: nothing). With a gamepad active the gamepad binding
/// is used instead (`fn_00a23d40`). A keyboard binding sets bit 7 of its
/// key's state byte; a mouse binding sets bit 7 of its button byte (swapped
/// buttons 0 and 1 when `+0x1b4c` is set); a joystick binding below 0x20
/// sets bit 7 of that button byte of the first joystick, otherwise it is a
/// hat direction `(binding - 0x20) * 4500`: when the hat already pointed
/// somewhere, an exactly opposite direction (difference 9000) becomes
/// half-way between, and a difference of 27000 becomes 31500.
pub fn fn_00a24280(e: &mut Engine, this: Ptr<Controls>, control: i32) {
    if control >= 0xd {
        return;
    }
    let mut device = i32::from(binding(e, this, 0, control) == UNBOUND);
    if binding(e, this, device, control) == UNBOUND {
        device += 1;
    }
    if binding(e, this, device, control) == UNBOUND {
        return;
    }
    if e.mem.u8(GAMEPAD_ACTIVE) != 0 {
        device = 3;
    }
    match device {
        0 => {
            let key = binding(e, this, 0, control);
            press(e, field_addr(this, KEYBOARD_STATE, i32::from(key)));
        }
        1 => {
            let mut button = binding(e, this, 1, control);
            let swapped = e.get(this, Controls::mouse_buttons_swapped) != 0;
            if button == 0 && swapped {
                button = 1;
            } else if button == 1 && swapped {
                button = 0;
            }
            press(e, field_addr(this, MOUSE_BUTTONS, i32::from(button)));
        }
        3 => {
            let gamepad_control = e.mem.u8(field_addr(this, GAMEPAD_BINDINGS, control));
            fn_00a23d40(e, this, i32::from(gamepad_control));
        }
        _ => {
            let key = binding(e, this, device, control);
            if key < 0x20 {
                press(e, field_addr(this, JOYSTICK_BUTTONS, i32::from(key)));
            } else {
                let old = e.get(this, Controls::first_hat);
                let mut new = (i32::from(key) - 0x20) * 0x1194;
                if old != -1 {
                    let difference = old.wrapping_sub(new);
                    if e.call(ABSOLUTE_VALUE, &args![difference]).i32() == 9000 {
                        let half = e.global::<f64>(HALF);
                        let scaled = f64::from(difference) * half;
                        new = new.wrapping_add(e.call(FTOL, &args![scaled]).i32());
                    } else if e.call(ABSOLUTE_VALUE, &args![difference]).i32() == 27000 {
                        new = 0x7b0c;
                    }
                }
                e.set(this, Controls::first_hat, new);
            }
        }
    }
}

// Translated from 00a24560 (decompiled, FalloutNV.exe 1.4.0.525)
/// State of key or button `key` on device `device`: 0 keyboard
/// (`fn_00a24180`), 1 mouse (`fn_00a23a50`), 3 gamepad (`fn_00a23e10`),
/// `0xff` nothing (0). Device `n` >= 2 is joystick `n - 2` when that exists
/// (below the joystick count): a button below 0x20 asks `00a234c0`, a
/// larger key is true when the joystick's bound key (`00a235f0`, slot 0)
/// equals it. Everything else is 0.
pub fn fn_00a24560(e: &mut Engine, this: Ptr<Controls>, device: i32, key: u8, mode: u32) -> i32 {
    match device {
        3 => return i32::from(fn_00a23e10(e, this, i32::from(key), mode)),
        0 => return i32::from(fn_00a24180(e, this, i32::from(key), mode)),
        1 => return i32::from(fn_00a23a50(e, this, i32::from(key), mode)),
        0xff => return 0,
        _ => {}
    }
    let joystick = device.wrapping_sub(2);
    if joystick >= e.get(this, Controls::joystick_count) {
        return 0;
    }
    if key < 0x20 {
        e.call(
            JOYSTICK_BUTTON_TEST,
            &args![this, joystick, u32::from(key), mode],
        )
        .i32()
    } else {
        let bound = e
            .call(JOYSTICK_BOUND_KEY, &args![this, joystick, 0u32])
            .u32();
        i32::from(bound == u32::from(key))
    }
}

// Translated from 00a24660 (decompiled, FalloutNV.exe 1.4.0.525)
/// State of control `control` (0 to 0x1e) in `mode` (0 held, 1 went down,
/// 2 went up, 3 changed). Controls below 0x1c sum the state of their
/// binding on the keyboard and the mouse (`fn_00a24560`), or just the
/// gamepad binding while a gamepad is active. Control 0xe also toggles bit 3
/// of the flag word at `+0x1b92`: when it was set it is cleared, the result
/// becomes 0, and it is set again if `mode` is 0 or 1. The controls from
/// 0x1c are fixed keys: 0x1c is key 1 (or gamepad 6 with a gamepad), 0x1d is
/// key 0x29, 0x1e is key 0xb7, each asked in mode 1.
pub fn fn_00a24660(e: &mut Engine, this: Ptr<Controls>, control: i32, mode: u32) -> i32 {
    if control >= 0x1c {
        if e.mem.u8(GAMEPAD_ACTIVE) == 0 {
            match control {
                0x1c => {
                    if fn_00a24180(e, this, 1, 1) || fn_00a23e10(e, this, 6, 1) {
                        return 1;
                    }
                }
                0x1d => return i32::from(fn_00a24180(e, this, 0x29, 1)),
                0x1e => return i32::from(fn_00a24180(e, this, 0xb7, 1)),
                _ => {}
            }
        } else if control == 0x1c && fn_00a23e10(e, this, 6, 1) {
            return 1;
        }
        return 0;
    }
    let mut total = 0i32;
    if e.mem.u8(GAMEPAD_ACTIVE) != 0 {
        let key = e.mem.u8(field_addr(this, GAMEPAD_BINDINGS, control));
        total = fn_00a24560(e, this, 3, key, mode);
    } else {
        for device in 0..2 {
            let key = binding(e, this, device, control);
            if key != UNBOUND {
                total = total.wrapping_add(fn_00a24560(e, this, device, key, mode));
            }
        }
    }
    if control == 0xe && total > 0 {
        let flags = e.get(this, Controls::control_flags);
        if flags & 8 != 0 {
            e.set(this, Controls::control_flags, flags ^ 8);
            total = 0;
            if mode == 0 || mode == 1 {
                let flags = e.get(this, Controls::control_flags);
                e.set(this, Controls::control_flags, flags | 8);
            }
        }
    }
    total
}

// Translated from 00a24830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Device-enumeration callback (`info` = the device instance record, `this`
/// the context). For a record whose device type byte (`+0x24`) is `0x14` or
/// `0x15`: copies the `0x244`-byte record into the next joystick slot,
/// creates the device (`CreateDevice` with the record's instance GUID at
/// `info + 4`; the result code is not examined), sets the cooperative level
/// (active window, 6) and data format (`01100ebc`), reads its capabilities
/// into the slot's `0x2c`-byte record, clears the slot's two object-mask
/// words, enumerates its objects with `fn_00a24a30`, stores the device, sets
/// the axis range to -100..100 and the dead zone to the float at
/// `0101e2bc`, and counts it. Returns 0 (stop) when that makes eight
/// joysticks, else 1 (continue); other device types are skipped with 1.
pub fn fn_00a24830(e: &mut Engine, info: Ptr, this: Ptr<Controls>) -> u32 {
    let device_type = e.mem.u32(info.addr() + 0x24) & 0xff;
    if device_type != 0x14 && device_type != 0x15 {
        return 1;
    }
    let slot = e.get(this, Controls::joystick_count);
    let slot_words = slot as u32;
    let instance = this
        .addr()
        .wrapping_add(JOYSTICK_INSTANCES)
        .wrapping_add(slot_words.wrapping_mul(JOYSTICK_INSTANCE_SIZE));
    e.call(MEMCPY, &args![instance, info, JOYSTICK_INSTANCE_SIZE]);
    let interface = e.get(this, Controls::direct_input);
    let device = e.with_stack(4, |e, out| {
        e.vcall(interface.addr(), 0xc, &args![info.addr() + 4, out, 0u32]);
        e.mem.u32(out.addr())
    });
    let window = e.call(IMPORT_GET_ACTIVE_WINDOW, &args![]).u32();
    e.vcall(device, 0x34, &args![window, 6u32]);
    e.vcall(device, 0x2c, &args![JOYSTICK_DATA_FORMAT]);
    let capabilities = this
        .addr()
        .wrapping_add(JOYSTICK_CAPABILITIES)
        .wrapping_add(slot_words.wrapping_mul(0x2c));
    e.call(MEMSET, &args![capabilities, 0u32, 0x2cu32]);
    e.mem.set_u32(capabilities, 0x2c);
    e.vcall(device, 0xc, &args![capabilities]);
    let masks = this
        .addr()
        .wrapping_add(JOYSTICK_OBJECT_MASKS)
        .wrapping_add(slot_words.wrapping_mul(8));
    e.mem.set_u32(masks, 0);
    e.mem.set_u32(masks.wrapping_add(4), 0);
    e.vcall(device, 0x10, &args![OBJECT_CALLBACK, masks, 0u32]);
    e.mem.set_u32(
        this.addr()
            .wrapping_add(JOYSTICK_DEVICES)
            .wrapping_add(slot_words.wrapping_mul(4)),
        device,
    );
    e.call(JOYSTICK_SET_RANGE, &args![this, slot, -100i32, 100i32]);
    let dead_zone = e.global::<f32>(JOYSTICK_DEAD_ZONE);
    e.call(JOYSTICK_SET_DEAD_ZONE, &args![this, slot, dead_zone]);
    let count = slot + 1;
    e.set(this, Controls::joystick_count, count);
    u32::from(count != 8)
}

// Translated from 00a24a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Device-object enumeration callback (`object` = the object instance
/// record, `masks` = the slot's two mask words). Sets bit `n` of the first
/// word when the object's type GUID (`object + 4`) equals the `n`th GUID of
/// the six at `01100ee4`..; if the low byte of the object's type word
/// (`+0x18`) has bit 2 or 3 set, sets bit `(type >> 8) & 0x1f` of the second
/// word. Always 1 (continue).
pub fn fn_00a24a30(e: &mut Engine, object: Ptr, masks: Ptr) -> u32 {
    for (bit, guid) in OBJECT_GUIDS.iter().enumerate() {
        let same = e
            .call(MEMCMP, &args![object.addr() + 4, *guid, 0x10u32])
            .i32()
            == 0;
        if same {
            let word = e.mem.u32(masks.addr());
            e.mem.set_u32(masks.addr(), word | 1 << bit);
        }
    }
    let type_word = e.mem.u32(object.addr() + 0x18);
    if type_word & 0xff & 0xc != 0 {
        let word = e.mem.u32(masks.addr() + 4);
        e.mem
            .set_u32(masks.addr() + 4, (1u32 << ((type_word >> 8) & 0x1f)) | word);
    }
    1
}

/// Stores each `(offset, value)` as a byte at `this + offset`.
fn set_bytes(e: &mut Engine, this: Ptr<Controls>, bytes: &[(u32, u8)]) {
    for (offset, value) in bytes {
        e.mem.set_u8(this.addr() + offset, *value);
    }
}

// Translated from 00a24b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the bindings to the defaults. First every slot of device
/// `which` (all four when `which >= 4`) becomes unbound. Then the defaults
/// are written: keyboard (`which` 0 or 4), mouse (1 or 4: buttons 0, 1, 2
/// on three slots), joystick (2 or 4, and only while bit 0 of the flag word
/// is set) and gamepad (3 or 4).
pub fn fn_00a24b70(e: &mut Engine, this: Ptr<Controls>, which: i32) {
    for slot in 0..BINDING_SLOTS {
        if which < 4 {
            e.mem.set_u8(binding_addr(this, which, slot), UNBOUND);
        } else {
            for device in 0..4 {
                e.mem.set_u8(binding_addr(this, device, slot), UNBOUND);
            }
        }
    }
    if which == 0 || which == 4 {
        set_bytes(
            e,
            this,
            &[
                (0x1b94, 0x11),
                (0x1b95, 0x1f),
                (0x1b96, 0x1e),
                (0x1b97, 0x20),
                (0x1b99, 0x12),
                (0x1b9a, 0x38),
                (0x1b9b, 0x13),
                (0x1b9c, 0x1d),
                (0x1b9d, 0x2a),
                (0x1b9e, 0x3a),
                (0x1b9f, 0x10),
                (0x1ba0, 0x39),
                (0x1ba1, 0x21),
                (0x1ba3, 0x14),
                (0x1ba2, 0x0f),
                (0x1ba4, 0x2f),
                (0x1ba5, 2),
                (0x1ba6, 3),
                (0x1ba7, 4),
                (0x1ba8, 5),
                (0x1ba9, 6),
                (0x1baa, 7),
                (0x1bab, 8),
                (0x1bac, 9),
                (0x1bad, 0x3f),
                (0x1bae, 0x43),
                (0x1baf, 0x2c),
            ],
        );
    }
    if which == 1 || which == 4 {
        set_bytes(e, this, &[(0x1bb4, 0), (0x1bb6, 1), (0x1bbd, 2)]);
    }
    if (which == 2 || which == 4) && e.get(this, Controls::flags) & 1 != 0 {
        set_bytes(
            e,
            this,
            &[
                (0x1bd4, 0),
                (0x1bd8, 1),
                (0x1bd3, 2),
                (0x1bd1, 3),
                (0x1bda, 4),
                (0x1bd9, 5),
                (0x1be7, 7),
                (0x1bd2, 8),
                (0x1bd0, 9),
                (0x1bdb, 10),
                (0x1bde, 0x20),
                (0x1bdd, 0x21),
                (0x1bdf, 0x22),
                (0x1be0, 0x23),
                (0x1be1, 0x24),
                (0x1be2, 0x25),
                (0x1be3, 0x26),
                (0x1be4, 0x27),
            ],
        );
    }
    if which == 3 || which == 4 {
        set_bytes(
            e,
            this,
            &[
                (0x1be8, 0x13),
                (0x1be9, 0x14),
                (0x1bea, 0x17),
                (0x1beb, 0x16),
                (0x1bf0, 8),
                (0x1bf4, 0x0d),
                (0x1bef, 0x0c),
                (0x1bed, 0x0a),
                (0x1bf6, 0x0b),
                (0x1bf5, 0x0f),
                (0x1c03, 9),
                (0x1bee, 0x10),
                (0x1bec, 0x11),
                (0x1bf7, 7),
                (0x1bf8, 0x0e),
                (0x1bfa, 1),
            ],
        );
    }
}

// Translated from 00a24eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unbinds every slot of `device` that holds the binding `key`.
pub fn fn_00a24eb0(e: &mut Engine, this: Ptr<Controls>, device: i32, key: u32) {
    for slot in 0..BINDING_SLOTS {
        let address = binding_addr(this, device, slot);
        if u32::from(e.mem.u8(address)) == key {
            e.mem.set_u8(address, UNBOUND);
        }
    }
}

// Translated from 00a24f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs the three bindings of control `control` (keyboard, mouse and
/// gamepad; the joystick binding is left out) into one word:
/// keyboard << 16 | mouse << 8 | gamepad.
pub fn fn_00a24f10(e: &mut Engine, this: Ptr<Controls>, control: i32) -> u32 {
    u32::from(e.mem.u8(field_addr(this, GAMEPAD_BINDINGS, control)))
        | u32::from(binding(e, this, 1, control)) << 8
        | u32::from(binding(e, this, 0, control)) << 16
}

// Translated from 00a24f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reverse of `fn_00a24f10`: sets the keyboard, mouse and gamepad
/// bindings of control `control` from bytes 2, 1 and 0 of `packed`.
pub fn fn_00a24f60(e: &mut Engine, this: Ptr<Controls>, control: i32, packed: u32) {
    e.mem
        .set_u8(binding_addr(this, 0, control), (packed >> 16) as u8);
    e.mem
        .set_u8(binding_addr(this, 1, control), (packed >> 8) as u8);
    e.mem
        .set_u8(field_addr(this, GAMEPAD_BINDINGS, control), packed as u8);
}

// Translated from 00a24fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Binds `key` to `slot` of `device`, swapping: whichever slot of the device
/// held `key` (the first one) gets the slot's previous binding. Refuses
/// (false) the reserved keys on the keyboard (1, 2 to 9, 0x29, 0x2b, 0x3b to
/// 0x3e, 0xb7, 0xc7) and any gamepad code outside 7 to 0x11. True when
/// bound.
pub fn fn_00a24fc0(e: &mut Engine, this: Ptr<Controls>, slot: i32, device: i32, key: u8) -> bool {
    if device == 0 && matches!(key, 1..=9 | 0x29 | 0xb7 | 0x3b..=0x3e | 0xc7 | 0x2b) {
        return false;
    }
    if device == 3 && !matches!(key, 7..=0x11) {
        return false;
    }
    for held in 0..BINDING_SLOTS {
        if binding(e, this, device, held) == key {
            let previous = binding(e, this, device, slot);
            e.mem.set_u8(binding_addr(this, device, held), previous);
            break;
        }
    }
    e.mem.set_u8(binding_addr(this, device, slot), key);
    true
}

// Translated from 00a25180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Binds `key` to `slot` of `device` after unbinding it elsewhere on that
/// device (`fn_00a24eb0`), except that the keyboard keys 1, 0x29 and 0xb7
/// are not bound at all.
pub fn fn_00a25180(e: &mut Engine, this: Ptr<Controls>, slot: i32, device: i32, key: u8) {
    if device == 0 && matches!(key, 0xb7 | 1 | 0x29) {
        return;
    }
    fn_00a24eb0(e, this, device, u32::from(key));
    e.mem.set_u8(binding_addr(this, device, slot), key);
}

// Translated from 00a251e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits-for-a-key helper of the rebinding screen. A `slot` other than -1
/// is stored as the slot being rebound (`+0x1b8c`). Reads one keyboard event
/// (`fn_00a237f0`); if its key code is not 0 (even for a key going up), binds
/// it to the slot on the keyboard (`fn_00a25180`). Otherwise the first mouse
/// button 1 to 3 that went down is bound on the mouse device. A key was
/// caught: byte `+0x1b90` = 0, `+0x1b91` = 1, true; nothing: `+0x1b91` = 0,
/// `+0x1b90` = 1, false.
pub fn fn_00a251e0(e: &mut Engine, this: Ptr<Controls>, slot: i32) -> bool {
    if slot != -1 {
        e.set(this, Controls::rebinding_slot, slot);
    }
    let key = e.with_stack(4, |e, out| {
        fn_00a237f0(e, this, out);
        e.mem.u32(out.addr())
    });
    if key != 0 {
        let slot = e.get(this, Controls::rebinding_slot);
        fn_00a25180(e, this, slot, 0, key as u8);
        e.set(this, Controls::nothing_caught, 0);
        e.set(this, Controls::something_caught, 1);
        return true;
    }
    for button in 1..=3 {
        if fn_00a23a50(e, this, button, 1) {
            let slot = e.get(this, Controls::rebinding_slot);
            fn_00a25180(e, this, slot, 1, button as u8);
            e.set(this, Controls::nothing_caught, 0);
            e.set(this, Controls::something_caught, 1);
            return true;
        }
    }
    e.set(this, Controls::something_caught, 0);
    e.set(this, Controls::nothing_caught, 1);
    false
}

// Translated from 00a252d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts (`enable` non-zero) or stops joystick use. Starting sets flag bit 0
/// and enumerates the attached devices of the interface at `+0x08` (slot
/// `0x10`: type 0, callback `fn_00a24830`, this object, flag 1), clearing the
/// bit again when no joystick was found. Stopping Unacquires (slot `0x20`)
/// and Releases (slot `0x08`) every joystick device, clears the bit and the
/// count and zeroes the `0x500` bytes of joystick state at `+0x34`.
pub fn fn_00a252d0(e: &mut Engine, this: Ptr<Controls>, enable: u8) {
    if enable != 0 {
        let flags = e.get(this, Controls::flags);
        e.set(this, Controls::flags, flags | 1);
        let interface = e.get(this, Controls::direct_input);
        e.vcall(
            interface.addr(),
            0x10,
            &args![0u32, DEVICE_CALLBACK, this, 1u32],
        );
        if e.get(this, Controls::joystick_count) == 0 {
            let flags = e.get(this, Controls::flags);
            e.set(this, Controls::flags, flags & !1);
        }
    } else {
        let mut index = 0u32;
        while (index as i32) < e.get(this, Controls::joystick_count) {
            let device = e.mem.u32(this.addr() + JOYSTICK_DEVICES + 4 * index);
            e.vcall(device, 0x20, &args![]);
            let device = e.mem.u32(this.addr() + JOYSTICK_DEVICES + 4 * index);
            e.vcall(device, 0x08, &args![]);
            index += 1;
        }
        let flags = e.get(this, Controls::flags);
        e.set(this, Controls::flags, flags & !1);
        e.set(this, Controls::joystick_count, 0);
        e.call(MEMSET, &args![this.addr() + JOYSTICK_STATE, 0u32, 0x500u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a237b0, controls_clear_keystrokes(Ptr<Controls>) -> u32),
        entry!(0x00a237f0, fn_00a237f0(Ptr<Controls>, Ptr) -> bool),
        entry!(0x00a23820, fn_00a23820(Ptr<Controls>, Ptr) -> u32),
        entry!(0x00a238a0, fn_00a238a0(u32, i32, i32) -> u32),
        entry!(0x00a239e0, fn_00a239e0(Ptr<Controls>, i32) -> i32),
        entry!(0x00a23a30, fn_00a23a30(Ptr<Controls>) -> u32),
        entry!(0x00a23a50, fn_00a23a50(Ptr<Controls>, i32, u32) -> bool),
        entry!(0x00a23c00, fn_00a23c00(Ptr<Controls>, u8)),
        entry!(0x00a23d40, fn_00a23d40(Ptr<Controls>, i32)),
        entry!(0x00a23e10, fn_00a23e10(Ptr<Controls>, i32, u32) -> bool),
        entry!(0x00a24080, fn_00a24080(u32, u32) -> u32),
        entry!(0x00a24180, fn_00a24180(Ptr<Controls>, i32, u32) -> bool),
        entry!(0x00a24280, fn_00a24280(Ptr<Controls>, i32)),
        entry!(0x00a24560, fn_00a24560(Ptr<Controls>, i32, u8, u32) -> i32),
        entry!(0x00a24660, fn_00a24660(Ptr<Controls>, i32, u32) -> i32),
        entry!(0x00a24830, fn_00a24830(Ptr, Ptr<Controls>) -> u32),
        entry!(0x00a24a30, fn_00a24a30(Ptr, Ptr) -> u32),
        entry!(0x00a24b70, fn_00a24b70(Ptr<Controls>, i32)),
        entry!(0x00a24eb0, fn_00a24eb0(Ptr<Controls>, i32, u32)),
        entry!(0x00a24f10, fn_00a24f10(Ptr<Controls>, i32) -> u32),
        entry!(0x00a24f60, fn_00a24f60(Ptr<Controls>, i32, u32)),
        entry!(0x00a24fc0, fn_00a24fc0(Ptr<Controls>, i32, i32, u8) -> bool),
        entry!(0x00a25180, fn_00a25180(Ptr<Controls>, i32, i32, u8)),
        entry!(0x00a251e0, fn_00a251e0(Ptr<Controls>, i32) -> bool),
        entry!(0x00a252d0, fn_00a252d0(Ptr<Controls>, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An engine with the pages holding the globals and tables mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011a_7000, 0x011f_3000, 0x0101_1000, 0x0101_e000] {
            e.map(page, 0x1000);
        }
        e
    }

    fn controls(e: &mut Engine) -> Ptr<Controls> {
        e.new_object()
    }

    /// A device object whose vtable has the given `(slot offset, target)`.
    fn device(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        e.map(vtable, 0x1000);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Records every call to `address` (all argument words) and answers
    /// `answer`.
    fn recorder(e: &mut Engine, address: u32, answer: u32) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let seen = calls.clone();
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            ret(answer)
        });
        calls
    }

    fn set_binding(e: &mut Engine, this: Ptr<Controls>, device: i32, slot: i32, key: u8) {
        e.mem.set_u8(binding_addr(this, device, slot), key);
    }

    /// A controls object whose bindings are all unbound.
    fn unbound(e: &mut Engine) -> Ptr<Controls> {
        let this = controls(e);
        for slot in 0..0x70 {
            e.mem.set_u8(this.addr() + BINDINGS + slot, UNBOUND);
        }
        this
    }

    fn gamepad(e: &mut Engine) {
        e.mem.set_u8(GAMEPAD_ACTIVE, 1);
    }

    const KEYBOARD_VTABLE: u32 = 0x7100_0000;
    const KEYBOARD_READ: u32 = 0x7100_1000;

    /// Gives the controls a keyboard whose one-event read returns `result`
    /// and the event `(key, data)`, and sets the count word to `count` when
    /// it is given.
    fn keyboard_with_event(
        e: &mut Engine,
        this: Ptr<Controls>,
        result: u32,
        event: (u32, u32),
        count: Option<u32>,
    ) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let seen = calls.clone();
        e.register_double(KEYBOARD_READ, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[2], event.0);
            e.mem.set_u32(a[2] + 4, event.1);
            if let Some(count) = count {
                e.mem.set_u32(a[3], count);
            }
            ret(result)
        });
        let keyboard = device(e, KEYBOARD_VTABLE, &[(0x28, KEYBOARD_READ)]);
        e.set(this, Controls::keyboard_device, Ptr::new(keyboard));
        calls
    }

    #[test]
    fn clear_keystrokes_flushes_the_keyboard_buffer() {
        let mut e = engine();
        let this = controls(&mut e);
        assert_eq!(
            controls_clear_keystrokes(&mut e, this),
            this.addr(),
            "no keyboard"
        );
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let seen = calls.clone();
        e.register_double(KEYBOARD_READ, move |e, a| {
            let mut words = a.to_vec();
            words.push(e.mem.u32(a[3]));
            seen.borrow_mut().push(words);
            ret(7)
        });
        let keyboard = device(&mut e, KEYBOARD_VTABLE, &[(0x28, KEYBOARD_READ)]);
        e.set(this, Controls::keyboard_device, Ptr::new(keyboard));
        assert_eq!(controls_clear_keystrokes(&mut e, this), 7);
        let call = calls.borrow()[0].clone();
        assert_eq!(
            (call[0], call[1], call[2], call[4], call[5]),
            (keyboard, 0x14, 0, 0, 0xffff_ffff)
        );
    }

    #[test]
    fn keyboard_event_reports_a_key_going_down() {
        let mut e = engine();
        let this = controls(&mut e);
        let out = Ptr::<()>::new(e.mem.alloc(4));
        // No keyboard: false without a read.
        assert!(!fn_00a237f0(&mut e, this, out));
        let calls = keyboard_with_event(&mut e, this, 0, (0x1e, 0x80), None);
        assert!(fn_00a237f0(&mut e, this, out));
        assert_eq!(e.mem.u32(out.addr()), 0x1e);
        // The read asks for one 0x14-byte event and passes no flags.
        let call = calls.borrow()[0].clone();
        assert_eq!((call[1], call[4]), (0x14, 0));
        // A release is not "down", but the key code is still written.
        keyboard_with_event(&mut e, this, 0, (0x2a, 0), None);
        assert!(!fn_00a237f0(&mut e, this, out));
        assert_eq!(e.mem.u32(out.addr()), 0x2a);
    }

    #[test]
    fn keyboard_read_results() {
        let mut e = engine();
        let this = controls(&mut e);
        let out = Ptr::<()>::new(e.mem.alloc(4));
        assert_eq!(fn_00a23820(&mut e, this, out), 0);
        keyboard_with_event(&mut e, this, 0, (7, 0x80), None);
        assert_eq!(fn_00a23820(&mut e, this, out), 1);
        keyboard_with_event(&mut e, this, 0, (8, 0x7f), None);
        assert_eq!(fn_00a23820(&mut e, this, out), 2);
        assert_eq!(e.mem.u32(out.addr()), 8);
        // The read failed, or no event came back: nothing is stored.
        e.mem.set_u32(out.addr(), 0);
        keyboard_with_event(&mut e, this, 1, (9, 0x80), None);
        assert_eq!(fn_00a23820(&mut e, this, out), 0);
        keyboard_with_event(&mut e, this, 0, (9, 0x80), Some(0));
        assert_eq!(fn_00a23820(&mut e, this, out), 0);
        assert_eq!(e.mem.u32(out.addr()), 0);
    }

    #[test]
    fn byte_tables_by_language() {
        let mut e = engine();
        let tables = [
            0x011a_7440u32,
            0x011a_7540,
            0x011a_7640,
            0x011a_7740,
            0x011a_7840,
            0x011a_7940,
            0x011a_7a40,
            0x011a_7b40,
            0x011a_7c40,
            0x011a_7d40,
        ];
        for (i, table) in tables.iter().enumerate() {
            e.mem.set_u8(table + 5, 0x10 + i as u8);
        }
        let language = Rc::new(RefCell::new(0u32));
        let current = language.clone();
        e.register_double(GET_LANGUAGE, move |_, _| ret(*current.borrow()));
        assert_eq!(fn_00a238a0(&mut e, 0, -1, 0), u32::MAX);
        assert_eq!(fn_00a238a0(&mut e, 0, 256, 0), u32::MAX);
        // Variant 0xff always reads the second default table.
        *language.borrow_mut() = 3;
        assert_eq!(fn_00a238a0(&mut e, 0, 5, 0xff), 0x11);
        // (language, variant zero, variant non-zero)
        for (lang, zero, other) in [
            (0u32, 0x10, 0x11),
            (7, 0x10, 0x11),
            (u32::MAX, 0x10, 0x11),
            (1, 0x12, 0x13),
            (2, 0x14, 0x15),
            (3, 0x16, 0x17),
            (4, 0x18, 0x19),
        ] {
            *language.borrow_mut() = lang;
            assert_eq!(fn_00a238a0(&mut e, 0, 5, 0), zero, "language {lang}");
            assert_eq!(fn_00a238a0(&mut e, 0, 5, 1), other, "language {lang}");
        }
    }

    #[test]
    fn mouse_movement_words() {
        let mut e = engine();
        let this = controls(&mut e);
        e.set(this, Controls::mouse_x, -3);
        e.set(this, Controls::mouse_y, 4);
        e.set(this, Controls::mouse_wheel, 120);
        assert_eq!(fn_00a239e0(&mut e, this, 1), -3);
        assert_eq!(fn_00a239e0(&mut e, this, 2), 4);
        assert_eq!(fn_00a239e0(&mut e, this, 3), 120);
        assert_eq!(fn_00a239e0(&mut e, this, 0), 0);
        assert_eq!(fn_00a239e0(&mut e, this, 4), 0);
    }

    #[test]
    fn word_at_1b08() {
        let mut e = engine();
        let this = controls(&mut e);
        e.mem.set_u32(this.addr() + 0x1b08, 0xdead_beef);
        assert_eq!(fn_00a23a30(&mut e, this), 0xdead_beef);
    }

    #[test]
    fn mouse_queries() {
        let mut e = engine();
        let this = controls(&mut e);
        // Wheel up / down.
        e.set(this, Controls::mouse_wheel, 1);
        assert!(fn_00a23a50(&mut e, this, 8, 0));
        assert!(!fn_00a23a50(&mut e, this, 9, 0));
        e.set(this, Controls::mouse_wheel, -1);
        assert!(!fn_00a23a50(&mut e, this, 8, 3));
        assert!(fn_00a23a50(&mut e, this, 9, 3));
        // Button 2 is down now and was up before.
        e.mem.set_u8(this.addr() + MOUSE_BUTTONS + 2, 0x80);
        assert!(fn_00a23a50(&mut e, this, 2, 0));
        assert!(fn_00a23a50(&mut e, this, 2, 1));
        assert!(!fn_00a23a50(&mut e, this, 2, 2));
        assert!(fn_00a23a50(&mut e, this, 2, 3));
        assert!(!fn_00a23a50(&mut e, this, 2, 5));
        // Button 3 was down before and is up now.
        e.mem.set_u8(this.addr() + MOUSE_BUTTONS_PREVIOUS + 3, 0x80);
        assert!(!fn_00a23a50(&mut e, this, 3, 0));
        assert!(fn_00a23a50(&mut e, this, 3, 2));
        // The extra byte, only for buttons 0 to 7.
        e.mem.set_u8(this.addr() + MOUSE_BUTTON_EXTRA + 4, 1);
        assert!(fn_00a23a50(&mut e, this, 4, 4));
        assert!(!fn_00a23a50(&mut e, this, 5, 4));
        // Swapped buttons: asking for 0 reads button 1 and vice versa.
        e.mem.set_u8(this.addr() + MOUSE_BUTTONS + 1, 0x80);
        assert!(!fn_00a23a50(&mut e, this, 0, 0));
        e.set(this, Controls::mouse_buttons_swapped, 1);
        assert!(fn_00a23a50(&mut e, this, 0, 0));
        assert!(!fn_00a23a50(&mut e, this, 1, 0));
    }

    const MOUSE_VTABLE: u32 = 0x7200_0000;
    const INTERFACE_VTABLE: u32 = 0x7200_1000;
    const NEW_VTABLE: u32 = 0x7200_2000;
    const UNACQUIRE: u32 = 0x7300_0020;
    const RELEASE: u32 = 0x7300_0008;
    const CREATE: u32 = 0x7300_000c;
    const COOPERATIVE: u32 = 0x7300_0034;
    const DATA_FORMAT: u32 = 0x7300_002c;
    const ACTIVE_WINDOW: u32 = 0x7300_0099;

    #[test]
    fn mouse_device_is_recreated() {
        // (exclusive argument, cooperative level, flag bits afterwards)
        for (exclusive, level, bits) in [(1u8, 6u32, 8u32), (0, 5, 0)] {
            let mut e = engine();
            let this = controls(&mut e);
            let unacquired = recorder(&mut e, UNACQUIRE, 0);
            let interface = device(&mut e, INTERFACE_VTABLE, &[(0xc, CREATE)]);
            e.set(this, Controls::direct_input, Ptr::new(interface));
            // Nothing happens without a mouse.
            fn_00a23c00(&mut e, this, exclusive);
            assert!(unacquired.borrow().is_empty());
            let old = device(&mut e, MOUSE_VTABLE, &[(0x20, UNACQUIRE), (8, RELEASE)]);
            let new = device(
                &mut e,
                NEW_VTABLE,
                &[(0x34, COOPERATIVE), (0x2c, DATA_FORMAT)],
            );
            e.set(this, Controls::mouse_device, Ptr::new(old));
            // The flag bit starts opposite to what the call sets.
            e.set(this, Controls::flags, if bits == 0 { 8 } else { 0 });
            let released = recorder(&mut e, RELEASE, 0);
            e.register_double(CREATE, move |e, a| {
                e.mem.set_u32(a[2], new);
                ret(0)
            });
            let cooperative = recorder(&mut e, COOPERATIVE, 0);
            let formatted = recorder(&mut e, DATA_FORMAT, 0);
            e.register(IMPORT_GET_ACTIVE_WINDOW, |_, _| ret(ACTIVE_WINDOW));
            fn_00a23c00(&mut e, this, exclusive);
            assert_eq!(unacquired.borrow().len(), 1);
            assert_eq!(released.borrow().len(), 1);
            assert_eq!(e.get(this, Controls::mouse_device), Ptr::new(new));
            assert_eq!(cooperative.borrow()[0][1..], [ACTIVE_WINDOW, level]);
            assert_eq!(formatted.borrow()[0][1], MOUSE_DATA_FORMAT);
            assert_eq!(e.get(this, Controls::flags), bits);
        }
    }

    #[test]
    fn failed_device_creation_changes_nothing() {
        let mut e = engine();
        let this = controls(&mut e);
        let interface = device(&mut e, INTERFACE_VTABLE, &[(0xc, CREATE)]);
        e.set(this, Controls::direct_input, Ptr::new(interface));
        let old = device(&mut e, MOUSE_VTABLE, &[(0x20, UNACQUIRE), (8, RELEASE)]);
        e.set(this, Controls::mouse_device, Ptr::new(old));
        recorder(&mut e, UNACQUIRE, 0);
        recorder(&mut e, RELEASE, 0);
        recorder(&mut e, CREATE, 0x8007_0057);
        let cooperative = recorder(&mut e, COOPERATIVE, 0);
        fn_00a23c00(&mut e, this, 1);
        assert!(cooperative.borrow().is_empty());
        assert_eq!(e.get(this, Controls::mouse_device), Ptr::new(old));
        assert_eq!(e.get(this, Controls::flags), 0);
    }

    #[test]
    fn gamepad_presses() {
        let mut e = engine();
        let this = controls(&mut e);
        // Without a gamepad nothing is written.
        fn_00a23d40(&mut e, this, 0x10);
        assert_eq!(e.mem.u8(GAMEPAD_LEFT_TRIGGER), 0);
        gamepad(&mut e);
        fn_00a23d40(&mut e, this, 0x10);
        fn_00a23d40(&mut e, this, 0x11);
        assert_eq!(e.mem.u8(GAMEPAD_LEFT_TRIGGER), 0xff);
        assert_eq!(e.mem.u8(GAMEPAD_RIGHT_TRIGGER), 0xff);
        fn_00a23d40(&mut e, this, 0x13);
        assert_eq!(e.mem.i16(GAMEPAD_THUMB_LY), 0x7fff);
        fn_00a23d40(&mut e, this, 0x14);
        assert_eq!(e.mem.i16(GAMEPAD_THUMB_LY), -0x7fff);
        fn_00a23d40(&mut e, this, 0x16);
        assert_eq!(e.mem.i16(GAMEPAD_THUMB_LX), 0x7fff);
        fn_00a23d40(&mut e, this, 0x17);
        assert_eq!(e.mem.i16(GAMEPAD_THUMB_LX), -0x7fff);
        // A button control ORs its mask; 0x12 has none; from 0x1d on
        // nothing happens.
        fn_00a23d40(&mut e, this, 6);
        fn_00a23d40(&mut e, this, 1);
        assert_eq!(e.mem.u16(GAMEPAD_BUTTONS), 0x11);
        fn_00a23d40(&mut e, this, 0x12);
        fn_00a23d40(&mut e, this, 0x1d);
        fn_00a23d40(&mut e, this, 0x20);
        assert_eq!(e.mem.u16(GAMEPAD_BUTTONS), 0x11);
        fn_00a23d40(&mut e, this, 0xf);
        assert_eq!(e.mem.u16(GAMEPAD_BUTTONS), 0x111);
    }

    #[test]
    fn gamepad_queries() {
        let mut e = engine();
        let this = controls(&mut e);
        assert!(!fn_00a23e10(&mut e, this, 6, 0));
        gamepad(&mut e);
        // Sticks.
        e.mem.set_u16(GAMEPAD_THUMB_LY, 0x1eaa);
        assert!(fn_00a23e10(&mut e, this, 0x13, 0));
        assert!(!fn_00a23e10(&mut e, this, 0x14, 0));
        e.mem.set_u16(GAMEPAD_THUMB_LY, 0x1ea9);
        assert!(!fn_00a23e10(&mut e, this, 0x13, 0));
        e.mem.set_u16(GAMEPAD_THUMB_LY, (-0x1eaai32) as u16);
        assert!(fn_00a23e10(&mut e, this, 0x14, 0));
        e.mem.set_u16(GAMEPAD_THUMB_LX, 0x2000);
        assert!(fn_00a23e10(&mut e, this, 0x16, 1));
        assert!(!fn_00a23e10(&mut e, this, 0x17, 1));
        e.mem.set_u16(GAMEPAD_THUMB_LX, (-0x2000i32) as u16);
        assert!(fn_00a23e10(&mut e, this, 0x17, 2));
        // Left trigger: down now, up before.
        e.mem.set_u8(GAMEPAD_LEFT_TRIGGER, 200);
        assert!(fn_00a23e10(&mut e, this, 0x10, 0));
        assert!(fn_00a23e10(&mut e, this, 0x10, 1));
        assert!(!fn_00a23e10(&mut e, this, 0x10, 2));
        assert!(fn_00a23e10(&mut e, this, 0x10, 3));
        assert!(!fn_00a23e10(&mut e, this, 0x10, 4));
        // Right trigger: up now, down before.
        e.mem.set_u8(GAMEPAD_RIGHT_TRIGGER_PREVIOUS, 1);
        assert!(!fn_00a23e10(&mut e, this, 0x11, 0));
        assert!(fn_00a23e10(&mut e, this, 0x11, 2));
        assert!(fn_00a23e10(&mut e, this, 0x11, 3));
        // Buttons (control 6 = mask 0x10): down now, up before.
        e.mem.set_u16(GAMEPAD_BUTTONS, 0x10);
        assert!(fn_00a23e10(&mut e, this, 6, 0));
        assert!(fn_00a23e10(&mut e, this, 6, 1));
        assert!(!fn_00a23e10(&mut e, this, 6, 2));
        assert!(fn_00a23e10(&mut e, this, 6, 3));
        e.mem.set_u16(GAMEPAD_BUTTONS_PREVIOUS, 0x10);
        assert!(!fn_00a23e10(&mut e, this, 6, 1));
        assert!(!fn_00a23e10(&mut e, this, 6, 3));
        e.mem.set_u16(GAMEPAD_BUTTONS, 0);
        assert!(fn_00a23e10(&mut e, this, 6, 2));
        // A control without a mask never matches.
        assert!(!fn_00a23e10(&mut e, this, 3, 0));
    }

    #[test]
    fn gamepad_button_masks() {
        let mut e = engine();
        let expected = [
            (0u32, 0u32),
            (1, 1),
            (2, 2),
            (3, 0),
            (4, 8),
            (5, 4),
            (6, 0x10),
            (7, 0x20),
            (8, 0x40),
            (9, 0x80),
            (10, 0x1000),
            (11, 0x2000),
            (12, 0x4000),
            (13, 0x8000),
            (14, 0x200),
            (15, 0x100),
            (16, 0),
            (100, 0),
        ];
        for (control, mask) in expected {
            assert_eq!(fn_00a24080(&mut e, 0, control), mask, "control {control}");
        }
    }

    #[test]
    fn keyboard_queries() {
        let mut e = engine();
        let this = controls(&mut e);
        e.mem.set_u8(this.addr() + KEYBOARD_STATE + 0x1e, 0x80);
        e.mem
            .set_u8(this.addr() + KEYBOARD_STATE_PREVIOUS + 0x1f, 0x80);
        assert!(fn_00a24180(&mut e, this, 0x1e, 0));
        assert!(fn_00a24180(&mut e, this, 0x1e, 1));
        assert!(!fn_00a24180(&mut e, this, 0x1e, 2));
        assert!(fn_00a24180(&mut e, this, 0x1e, 3));
        assert!(!fn_00a24180(&mut e, this, 0x1f, 0));
        assert!(fn_00a24180(&mut e, this, 0x1f, 2));
        assert!(fn_00a24180(&mut e, this, 0x1f, 3));
        assert!(!fn_00a24180(&mut e, this, 0x1f, 1));
        assert!(!fn_00a24180(&mut e, this, 0x1e, 4));
        // A byte without bit 7 is up.
        e.mem.set_u8(this.addr() + KEYBOARD_STATE + 0x20, 0x7f);
        assert!(!fn_00a24180(&mut e, this, 0x20, 0));
    }

    fn hat_engine() -> Engine {
        let mut e = engine();
        e.register(ABSOLUTE_VALUE, |_, a| ret((a[0] as i32).unsigned_abs()));
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.set_global(HALF, 0.5f64);
        e
    }

    #[test]
    fn pressing_a_control_from_its_bindings() {
        let mut e = hat_engine();
        // Out of range: nothing.
        let this = unbound(&mut e);
        set_binding(&mut e, this, 0, 13, 0x1e);
        fn_00a24280(&mut e, this, 13);
        assert_eq!(e.mem.u8(this.addr() + KEYBOARD_STATE + 0x1e), 0);
        // Keyboard binding.
        set_binding(&mut e, this, 0, 2, 0x1e);
        fn_00a24280(&mut e, this, 2);
        assert_eq!(e.mem.u8(this.addr() + KEYBOARD_STATE + 0x1e), 0x80);
        // All unbound: nothing.
        fn_00a24280(&mut e, this, 3);
        assert_eq!(e.mem.u8(this.addr() + MOUSE_BUTTONS), 0);
        // Mouse binding when the keyboard has none; swapped buttons.
        set_binding(&mut e, this, 1, 4, 0);
        fn_00a24280(&mut e, this, 4);
        assert_eq!(e.mem.u8(this.addr() + MOUSE_BUTTONS), 0x80);
        e.set(this, Controls::mouse_buttons_swapped, 1);
        fn_00a24280(&mut e, this, 4);
        assert_eq!(e.mem.u8(this.addr() + MOUSE_BUTTONS + 1), 0x80);
        e.mem.set_u8(this.addr() + MOUSE_BUTTONS, 0);
        set_binding(&mut e, this, 1, 5, 1);
        fn_00a24280(&mut e, this, 5);
        assert_eq!(e.mem.u8(this.addr() + MOUSE_BUTTONS), 0x80);
        // Joystick button.
        set_binding(&mut e, this, 2, 6, 3);
        fn_00a24280(&mut e, this, 6);
        assert_eq!(e.mem.u8(this.addr() + JOYSTICK_BUTTONS + 3), 0x80);
        // Gamepad: the gamepad binding is pressed on the XInput state, not
        // the keyboard binding.
        set_binding(&mut e, this, 3, 7, 0x11);
        set_binding(&mut e, this, 0, 7, 0x1f);
        gamepad(&mut e);
        fn_00a24280(&mut e, this, 7);
        assert_eq!(e.mem.u8(GAMEPAD_RIGHT_TRIGGER), 0xff);
        assert_eq!(e.mem.u8(this.addr() + KEYBOARD_STATE + 0x1f), 0);
    }

    #[test]
    fn hat_directions() {
        // (previous hat, binding, expected hat)
        for (old, key, expected) in [
            (-1i32, 0x20u8, 0i32),
            (-1, 0x22, 9000),
            // 9000 apart: half-way between the two.
            (0, 0x22, 4500),
            (9000, 0x20, 4500),
            // 27000 apart: 31500.
            (27000, 0x20, 0x7b0c),
            // Any other difference keeps the new direction.
            (18000, 0x20, 0),
            (4500, 0x21, 4500),
            (0, 0x23, 13500),
        ] {
            let mut e = hat_engine();
            let this = unbound(&mut e);
            set_binding(&mut e, this, 2, 1, key);
            e.set(this, Controls::first_hat, old);
            fn_00a24280(&mut e, this, 1);
            assert_eq!(
                e.get(this, Controls::first_hat),
                expected,
                "hat {old} binding {key:#x}"
            );
        }
    }

    #[test]
    fn device_state_by_kind() {
        let mut e = engine();
        let this = controls(&mut e);
        e.set(this, Controls::joystick_count, 3);
        e.mem.set_u8(this.addr() + KEYBOARD_STATE + 0x1e, 0x80);
        e.mem.set_u8(this.addr() + MOUSE_BUTTONS + 1, 0x80);
        gamepad(&mut e);
        e.mem.set_u16(GAMEPAD_BUTTONS, 0x10);
        let tests = recorder(&mut e, JOYSTICK_BUTTON_TEST, 5);
        let bound = recorder(&mut e, JOYSTICK_BOUND_KEY, 0x40);
        assert_eq!(fn_00a24560(&mut e, this, 0, 0x1e, 0), 1);
        assert_eq!(fn_00a24560(&mut e, this, 0, 0x1f, 0), 0);
        assert_eq!(fn_00a24560(&mut e, this, 1, 1, 0), 1);
        assert_eq!(fn_00a24560(&mut e, this, 3, 6, 0), 1);
        assert_eq!(fn_00a24560(&mut e, this, 0xff, 6, 0), 0);
        // Joystick 0 (device 2): a button asks the joystick test.
        assert_eq!(fn_00a24560(&mut e, this, 2, 0x1f, 1), 5);
        assert_eq!(tests.borrow()[0][1..], [0, 0x1f, 1]);
        // A larger key is compared with the joystick's bound key.
        assert_eq!(fn_00a24560(&mut e, this, 4, 0x40, 0), 1);
        assert_eq!(bound.borrow()[0][1..], [2, 0]);
        assert_eq!(fn_00a24560(&mut e, this, 4, 0x41, 0), 0);
        // No such joystick.
        assert_eq!(fn_00a24560(&mut e, this, 5, 0x40, 0), 0);
        assert_eq!(fn_00a24560(&mut e, this, 6, 3, 0), 0);
        assert_eq!(bound.borrow().len(), 2);
        assert_eq!(tests.borrow().len(), 1);
    }

    #[test]
    fn control_state_sums_the_bindings() {
        let mut e = engine();
        let this = unbound(&mut e);
        // Control 5: keyboard key 0x1e and mouse button 2, both down.
        set_binding(&mut e, this, 0, 5, 0x1e);
        set_binding(&mut e, this, 1, 5, 2);
        e.mem.set_u8(this.addr() + KEYBOARD_STATE + 0x1e, 0x80);
        e.mem.set_u8(this.addr() + MOUSE_BUTTONS + 2, 0x80);
        assert_eq!(fn_00a24660(&mut e, this, 5, 0), 2);
        // An unbound slot adds nothing; the joystick binding is not asked.
        set_binding(&mut e, this, 1, 5, UNBOUND);
        set_binding(&mut e, this, 2, 5, 3);
        assert_eq!(fn_00a24660(&mut e, this, 5, 0), 1);
        // With a gamepad, only its binding counts.
        set_binding(&mut e, this, 3, 5, 6);
        e.mem.set_u16(GAMEPAD_BUTTONS, 0x10);
        gamepad(&mut e);
        assert_eq!(fn_00a24660(&mut e, this, 5, 0), 1);
        e.mem.set_u16(GAMEPAD_BUTTONS, 0);
        assert_eq!(fn_00a24660(&mut e, this, 5, 0), 0);
    }

    #[test]
    fn control_e_toggles_its_flag() {
        // (mode, key now, key before): each makes the raw result positive.
        for (mode, now, before) in [
            (0u32, 0x80u8, 0u8),
            (1, 0x80, 0),
            (2, 0, 0x80),
            (3, 0x80, 0),
        ] {
            let mut e = engine();
            let this = unbound(&mut e);
            set_binding(&mut e, this, 0, 0xe, 0x1e);
            e.mem.set_u8(this.addr() + KEYBOARD_STATE + 0x1e, now);
            e.mem
                .set_u8(this.addr() + KEYBOARD_STATE_PREVIOUS + 0x1e, before);
            // Flag bit clear: the sum is returned as it is.
            assert_eq!(fn_00a24660(&mut e, this, 0xe, mode), 1, "mode {mode}");
            assert_eq!(e.get(this, Controls::control_flags), 0);
            // Flag bit set: the result is 0 and the bit stays set only in
            // modes 0 and 1.
            e.set(this, Controls::control_flags, 8);
            assert_eq!(fn_00a24660(&mut e, this, 0xe, mode), 0, "mode {mode}");
            let expected = if mode <= 1 { 8 } else { 0 };
            assert_eq!(
                e.get(this, Controls::control_flags),
                expected,
                "mode {mode}"
            );
        }
        // A zero sum leaves the flag alone.
        let mut e = engine();
        let this = unbound(&mut e);
        e.set(this, Controls::control_flags, 8);
        assert_eq!(fn_00a24660(&mut e, this, 0xe, 0), 0);
        assert_eq!(e.get(this, Controls::control_flags), 8);
    }

    #[test]
    fn fixed_key_controls() {
        let mut e = engine();
        let this = controls(&mut e);
        // 0x1c: key 1 went down.
        assert_eq!(fn_00a24660(&mut e, this, 0x1c, 9), 0);
        e.mem.set_u8(this.addr() + KEYBOARD_STATE + 1, 0x80);
        assert_eq!(fn_00a24660(&mut e, this, 0x1c, 9), 1);
        // 0x1d: key 0x29, 0x1e: key 0xb7, only the mode-1 transition.
        e.mem.set_u8(this.addr() + KEYBOARD_STATE + 0x29, 0x80);
        assert_eq!(fn_00a24660(&mut e, this, 0x1d, 0), 1);
        assert_eq!(fn_00a24660(&mut e, this, 0x1e, 0), 0);
        e.mem.set_u8(this.addr() + KEYBOARD_STATE + 0xb7, 0x80);
        assert_eq!(fn_00a24660(&mut e, this, 0x1e, 0), 1);
        e.mem
            .set_u8(this.addr() + KEYBOARD_STATE_PREVIOUS + 0xb7, 0x80);
        assert_eq!(fn_00a24660(&mut e, this, 0x1e, 0), 0);
        assert_eq!(fn_00a24660(&mut e, this, 0x1f, 0), 0);
        // With a gamepad: only 0x1c, through the gamepad's control 6.
        gamepad(&mut e);
        e.mem.set_u16(GAMEPAD_BUTTONS, 0x10);
        assert_eq!(fn_00a24660(&mut e, this, 0x1c, 0), 1);
        assert_eq!(fn_00a24660(&mut e, this, 0x1d, 0), 0);
        e.mem.set_u16(GAMEPAD_BUTTONS, 0);
        assert_eq!(fn_00a24660(&mut e, this, 0x1c, 0), 0);
    }

    #[test]
    fn joystick_is_set_up_by_enumeration() {
        const DEVICE_VTABLE: u32 = 0x7400_0000;
        const CAPABILITIES: u32 = 0x7500_0010;
        const OBJECTS: u32 = 0x7500_0011;
        let mut e = engine();
        e.map(0x7600_0000, 0x1000);
        let this = controls(&mut e);
        // A record with a device type that is neither 0x14 nor 0x15.
        let info = Ptr::<()>::new(0x7600_0000);
        e.mem.set_u32(info.addr() + 0x24, 0x0113);
        assert_eq!(fn_00a24830(&mut e, info, this), 1);
        assert_eq!(e.get(this, Controls::joystick_count), 0);
        // A gamepad-type record.
        e.mem.set_u32(info.addr() + 0x24, 0x0115);
        e.mem.set_u32(info.addr() + 0x100, 0xabcd);
        let joystick = device(
            &mut e,
            DEVICE_VTABLE,
            &[
                (0x34, COOPERATIVE),
                (0x2c, DATA_FORMAT),
                (0xc, CAPABILITIES),
                (0x10, OBJECTS),
            ],
        );
        let interface = device(&mut e, INTERFACE_VTABLE, &[(0xc, CREATE)]);
        e.set(this, Controls::direct_input, Ptr::new(interface));
        let created: Calls = Rc::new(RefCell::new(Vec::new()));
        let seen = created.clone();
        e.register_double(CREATE, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[2], joystick);
            ret(0)
        });
        e.register(IMPORT_GET_ACTIVE_WINDOW, |_, _| ret(ACTIVE_WINDOW));
        let cooperative = recorder(&mut e, COOPERATIVE, 0);
        let formatted = recorder(&mut e, DATA_FORMAT, 0);
        let capable = recorder(&mut e, CAPABILITIES, 0);
        let enumerated = recorder(&mut e, OBJECTS, 0);
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            ret(a[0])
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        let range = recorder(&mut e, JOYSTICK_SET_RANGE, 0);
        let dead_zone = recorder(&mut e, JOYSTICK_SET_DEAD_ZONE, 0);
        e.mem.set_f32(JOYSTICK_DEAD_ZONE, 0.1);
        e.mem.set_u32(this.addr() + 0x18b4, 0xffff_ffff);
        assert_eq!(fn_00a24830(&mut e, info, this), 1);
        assert_eq!(e.get(this, Controls::joystick_count), 1);
        // The record was copied to the first slot.
        assert_eq!(e.mem.u32(this.addr() + 0x534 + 0x100), 0xabcd);
        assert_eq!(created.borrow()[0][1], info.addr() + 4);
        assert_eq!(cooperative.borrow()[0][1..], [ACTIVE_WINDOW, 6]);
        assert_eq!(formatted.borrow()[0][1], JOYSTICK_DATA_FORMAT);
        // The capabilities record starts with its size, 0x2c.
        assert_eq!(capable.borrow()[0][1], this.addr() + 0x1754);
        assert_eq!(e.mem.u32(this.addr() + 0x1754), 0x2c);
        // The object masks were cleared and are handed to the enumeration.
        assert_eq!(
            enumerated.borrow()[0][1..],
            [OBJECT_CALLBACK, this.addr() + 0x18b4, 0]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x18b4), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x0c), joystick);
        assert_eq!(range.borrow()[0], vec![this.addr(), 0, -100i32 as u32, 100]);
        assert_eq!(
            dead_zone.borrow()[0],
            vec![this.addr(), 0, 0.1f32.to_bits()]
        );
        // The eighth joystick stops the enumeration.
        e.set(this, Controls::joystick_count, 7);
        assert_eq!(fn_00a24830(&mut e, info, this), 0);
        assert_eq!(e.get(this, Controls::joystick_count), 8);
        assert_eq!(e.mem.u32(this.addr() + 0x0c + 28), joystick);
    }

    #[test]
    fn object_masks() {
        let mut e = engine();
        e.map(0x7700_0000, 0x1000);
        e.map(0x0110_0000, 0x1000);
        let object = Ptr::<()>::new(0x7700_0000);
        let masks = Ptr::<()>::new(0x7700_0800);
        e.register(MEMCMP, |e, a| {
            let same = (0..a[2]).all(|i| e.mem.u8(a[0] + i) == e.mem.u8(a[1] + i));
            ret(u32::from(!same))
        });
        // Six different GUIDs.
        for (i, guid) in OBJECT_GUIDS.iter().enumerate() {
            for byte in 0..0x10 {
                e.mem.set_u8(guid + byte, (i as u32 * 16 + byte) as u8);
            }
        }
        // No known GUID and a type word without bits 2 / 3: nothing set.
        e.mem.set_u32(object.addr() + 0x18, 0x0000_0301);
        assert_eq!(fn_00a24a30(&mut e, object, masks), 1);
        assert_eq!(
            (e.mem.u32(masks.addr()), e.mem.u32(masks.addr() + 4)),
            (0, 0)
        );
        // The third and the sixth GUID, and type bit 2 with index 3.
        for known in [2usize, 5] {
            for byte in 0..0x10 {
                let value = e.mem.u8(OBJECT_GUIDS[known] + byte);
                e.mem.set_u8(object.addr() + 4 + byte, value);
            }
            e.mem.set_u32(object.addr() + 0x18, 0x0000_0304);
            fn_00a24a30(&mut e, object, masks);
        }
        assert_eq!(e.mem.u32(masks.addr()), 0b100100);
        assert_eq!(e.mem.u32(masks.addr() + 4), 1 << 3);
        // Type bit 3 with index 31 adds a further bit.
        e.mem.set_u32(object.addr() + 0x18, 0x0000_1f08);
        fn_00a24a30(&mut e, object, masks);
        assert_eq!(e.mem.u32(masks.addr() + 4), (1 << 3) | (1 << 31));
    }

    #[test]
    fn default_bindings() {
        let mut e = engine();
        let this = controls(&mut e);
        let garbage = |e: &mut Engine| {
            for i in 0..0x70 {
                e.mem.set_u8(this.addr() + BINDINGS + i, 0x55);
            }
        };
        // Resetting the keyboard touches only the keyboard device.
        garbage(&mut e);
        fn_00a24b70(&mut e, this, 0);
        assert_eq!(binding(&e, this, 0, 0), 0x11);
        assert_eq!(binding(&e, this, 0, 4), UNBOUND);
        assert_eq!(binding(&e, this, 0, 0x1a), 0x43);
        assert_eq!(binding(&e, this, 0, 0x1b), 0x2c);
        assert_eq!(binding(&e, this, 1, 0), 0x55);
        // Mouse: three bindings.
        fn_00a24b70(&mut e, this, 1);
        assert_eq!(binding(&e, this, 1, 0), UNBOUND);
        assert_eq!(binding(&e, this, 1, 4), 0);
        assert_eq!(binding(&e, this, 1, 6), 1);
        assert_eq!(binding(&e, this, 1, 13), 2);
        assert_eq!(binding(&e, this, 2, 0), 0x55);
        // Joystick: only unbound without flag bit 0, bindings with it.
        fn_00a24b70(&mut e, this, 2);
        assert_eq!(binding(&e, this, 2, 8), UNBOUND);
        e.set(this, Controls::flags, 1);
        fn_00a24b70(&mut e, this, 2);
        assert_eq!(binding(&e, this, 2, 8), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x1be4), 0x27);
        assert_eq!(e.mem.u8(this.addr() + 0x1be7), 7);
        // Gamepad.
        fn_00a24b70(&mut e, this, 3);
        assert_eq!(e.mem.u8(this.addr() + 0x1be8), 0x13);
        assert_eq!(e.mem.u8(this.addr() + 0x1c03), 9);
        assert_eq!(e.mem.u8(this.addr() + 0x1bfa), 1);
        // All four at once (with the joystick flag set).
        garbage(&mut e);
        fn_00a24b70(&mut e, this, 4);
        assert_eq!(e.mem.u8(this.addr() + 0x1b94), 0x11);
        assert_eq!(e.mem.u8(this.addr() + 0x1bb6), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x1bd8), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x1beb), 0x16);
        assert_eq!(e.mem.u8(this.addr() + 0x1bb5), UNBOUND);
        assert_eq!(e.mem.u8(this.addr() + 0x1bc0), UNBOUND);
    }

    #[test]
    fn unbinding_a_key() {
        let mut e = engine();
        let this = unbound(&mut e);
        set_binding(&mut e, this, 1, 2, 0x10);
        set_binding(&mut e, this, 1, 9, 0x10);
        set_binding(&mut e, this, 1, 10, 0x11);
        set_binding(&mut e, this, 0, 3, 0x10);
        fn_00a24eb0(&mut e, this, 1, 0x10);
        assert_eq!(binding(&e, this, 1, 2), UNBOUND);
        assert_eq!(binding(&e, this, 1, 9), UNBOUND);
        assert_eq!(binding(&e, this, 1, 10), 0x11);
        assert_eq!(binding(&e, this, 0, 3), 0x10);
    }

    #[test]
    fn packed_bindings() {
        let mut e = engine();
        let this = unbound(&mut e);
        fn_00a24f60(&mut e, this, 4, 0x0011_2233);
        assert_eq!(binding(&e, this, 0, 4), 0x11);
        assert_eq!(binding(&e, this, 1, 4), 0x22);
        assert_eq!(binding(&e, this, 3, 4), 0x33);
        assert_eq!(binding(&e, this, 2, 4), UNBOUND);
        assert_eq!(fn_00a24f10(&mut e, this, 4), 0x0011_2233);
        // The high byte of the word is dropped.
        fn_00a24f60(&mut e, this, 5, 0xaa44_5566);
        assert_eq!(fn_00a24f10(&mut e, this, 5), 0x0044_5566);
    }

    #[test]
    fn binding_with_swapping() {
        let mut e = engine();
        let this = unbound(&mut e);
        // Reserved keyboard keys are refused.
        for key in [1u8, 2, 5, 9, 0x29, 0x2b, 0x3b, 0x3e, 0xb7, 0xc7] {
            assert!(!fn_00a24fc0(&mut e, this, 0, 0, key), "key {key:#x}");
        }
        assert_eq!(binding(&e, this, 0, 0), UNBOUND);
        // 0x0a and 0x3f are allowed; a key held elsewhere swaps.
        assert!(fn_00a24fc0(&mut e, this, 0, 0, 0x0a));
        assert_eq!(binding(&e, this, 0, 0), 0x0a);
        set_binding(&mut e, this, 0, 3, 0x3f);
        assert!(fn_00a24fc0(&mut e, this, 0, 0, 0x3f));
        assert_eq!(binding(&e, this, 0, 0), 0x3f);
        assert_eq!(binding(&e, this, 0, 3), 0x0a);
        // On the mouse nothing is reserved.
        assert!(fn_00a24fc0(&mut e, this, 1, 1, 1));
        assert_eq!(binding(&e, this, 1, 1), 1);
        // Gamepad: only 7 to 0x11.
        assert!(!fn_00a24fc0(&mut e, this, 2, 3, 6));
        assert!(!fn_00a24fc0(&mut e, this, 2, 3, 0x12));
        assert!(fn_00a24fc0(&mut e, this, 2, 3, 7));
        assert!(fn_00a24fc0(&mut e, this, 3, 3, 0x11));
        assert_eq!(
            (binding(&e, this, 3, 2), binding(&e, this, 3, 3)),
            (7, 0x11)
        );
        // The first slot holding the key is the one that is swapped.
        set_binding(&mut e, this, 3, 5, 0x11);
        assert!(fn_00a24fc0(&mut e, this, 6, 3, 0x11));
        assert_eq!(binding(&e, this, 3, 3), UNBOUND);
        assert_eq!(binding(&e, this, 3, 5), 0x11);
        assert_eq!(binding(&e, this, 3, 6), 0x11);
    }

    #[test]
    fn binding_a_key_unbinds_it_elsewhere() {
        let mut e = engine();
        let this = unbound(&mut e);
        set_binding(&mut e, this, 0, 3, 0x1e);
        fn_00a25180(&mut e, this, 5, 0, 0x1e);
        assert_eq!(binding(&e, this, 0, 3), UNBOUND);
        assert_eq!(binding(&e, this, 0, 5), 0x1e);
        // The three reserved keyboard keys are ignored.
        for key in [1u8, 0x29, 0xb7] {
            fn_00a25180(&mut e, this, 6, 0, key);
            assert_eq!(binding(&e, this, 0, 6), UNBOUND);
        }
        // On the mouse they bind.
        fn_00a25180(&mut e, this, 6, 1, 1);
        assert_eq!(binding(&e, this, 1, 6), 1);
    }

    #[test]
    fn rebinding_catches_keys_and_mouse_buttons() {
        let mut e = engine();
        let this = unbound(&mut e);
        // Nothing pressed: false, and the flags say so.
        assert!(!fn_00a251e0(&mut e, this, 4));
        assert_eq!(e.get(this, Controls::rebinding_slot), 4);
        assert_eq!(e.get(this, Controls::something_caught), 0);
        assert_eq!(e.get(this, Controls::nothing_caught), 1);
        // A mouse button went down (button 2); slot -1 keeps the old one.
        e.mem.set_u8(this.addr() + MOUSE_BUTTONS + 2, 0x80);
        assert!(fn_00a251e0(&mut e, this, -1));
        assert_eq!(binding(&e, this, 1, 4), 2);
        assert_eq!(e.get(this, Controls::something_caught), 1);
        assert_eq!(e.get(this, Controls::nothing_caught), 0);
        // A keyboard event wins, even a release.
        keyboard_with_event(&mut e, this, 0, (0x1e, 0), None);
        assert!(fn_00a251e0(&mut e, this, 7));
        assert_eq!(binding(&e, this, 0, 7), 0x1e);
        // The reserved key 1 is caught but not bound.
        keyboard_with_event(&mut e, this, 0, (1, 0x80), None);
        assert!(fn_00a251e0(&mut e, this, 8));
        assert_eq!(binding(&e, this, 0, 8), UNBOUND);
    }

    #[test]
    fn joystick_support_starts_and_stops() {
        const JOYSTICK_VTABLE: u32 = 0x7800_1000;
        const ENUMERATE: u32 = 0x7900_0010;
        let mut e = engine();
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        let this = controls(&mut e);
        let interface = device(&mut e, INTERFACE_VTABLE, &[(0x10, ENUMERATE)]);
        e.set(this, Controls::direct_input, Ptr::new(interface));
        let enumerated = recorder(&mut e, ENUMERATE, 0);
        // Starting with no joystick found clears the flag again.
        fn_00a252d0(&mut e, this, 1);
        assert_eq!(
            enumerated.borrow()[0][1..],
            [0, DEVICE_CALLBACK, this.addr(), 1]
        );
        assert_eq!(e.get(this, Controls::flags), 0);
        // With joysticks found it stays set.
        e.register_double(ENUMERATE, move |e, a| {
            e.mem.set_i32(a[3] + 0x18f4, 2);
            ret(0)
        });
        fn_00a252d0(&mut e, this, 1);
        assert_eq!(e.get(this, Controls::flags), 1);
        // Stopping releases both devices and wipes the state.
        let first = device(&mut e, JOYSTICK_VTABLE, &[(0x20, UNACQUIRE), (8, RELEASE)]);
        let second = device(
            &mut e,
            JOYSTICK_VTABLE + 0x2000,
            &[(0x20, UNACQUIRE), (8, RELEASE)],
        );
        e.mem.set_u32(this.addr() + 0x0c, first);
        e.mem.set_u32(this.addr() + 0x10, second);
        e.mem.set_u8(this.addr() + 0x34, 9);
        e.mem.set_u8(this.addr() + 0x533, 9);
        e.mem.set_u8(this.addr() + 0x534, 9);
        let unacquired = recorder(&mut e, UNACQUIRE, 0);
        let released = recorder(&mut e, RELEASE, 0);
        fn_00a252d0(&mut e, this, 0);
        let order: Vec<u32> = unacquired.borrow().iter().map(|c| c[0]).collect();
        assert_eq!(order, vec![first, second]);
        let order: Vec<u32> = released.borrow().iter().map(|c| c[0]).collect();
        assert_eq!(order, vec![first, second]);
        assert_eq!(e.get(this, Controls::flags), 0);
        assert_eq!(e.get(this, Controls::joystick_count), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x34), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x533), 0);
        // Only 0x500 bytes are wiped.
        assert_eq!(e.mem.u8(this.addr() + 0x534), 9);
    }
}
