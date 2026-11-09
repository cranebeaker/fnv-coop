//! xNVSE's DirectInput hook (`Hooks_DirectInput8Create.cpp` (xNVSE)), build step I-11: the
//! keyboard and mouse the game reads go through xNVSE's `DIHookControl`, which keeps each key's
//! state for the input commands ([`crate::commands::input`]) and lets scripts hold, tap and
//! disable keys and buttons.
//!
//! xNVSE writes the exe's `DirectInput8Create` import slot (`00FDF02C`,
//! `Hook_DirectInput8Create_Init`): the game then gets xNVSE's `IDirectInput8A`
//! (`FakeDirectInput`), whose `CreateDevice` puts the system keyboard and mouse behind xNVSE's
//! `IDirectInputDevice8A` (`FakeDirectInputDevice`). Its `GetDeviceState` reads the real device
//! and passes every key and button through `DIHookControl` before the game sees them; its
//! `GetDeviceData` (the buffered keys menus and the console read) appends the keys
//! `MenuTapKey`, `MenuHoldKey` and `MenuReleaseKey` queued. Every other call goes to the real
//! object. The port's import slot holds the platform's DirectInput (`engine-platform`'s
//! `dinput`, fed by the window's and the viewer's keyboard and mouse); the wrap
//! [`DIRECT_INPUT8_CREATE`] puts the same two objects in front of it, in the guest's memory as
//! COM objects are (a vtable pointer first, the vtables' slots in the extenders' code range).
//!
//! Left out, with nothing that reads them: xNVSE's `FramerateTracker` (updated on each mouse
//! read, read by nothing in xNVSE 6.4.9) and its log line. The plugin interface's
//! `DIHookControl` (`kNVSEData_DIHookControl`) comes with native plugins.

use crate::Nvse;
use engine_core::extend::{Owner, WrapSlot, ROUTINE_BASE};
use engine_core::prelude::*;
use std::collections::VecDeque;

/// The `DirectInput8Create` import slot xNVSE writes.
pub const DIRECT_INPUT8_CREATE: WrapSlot = WrapSlot::new(Owner::Nvse, 0x00fd_f02c);

/// `kMacro_MouseButtonOffset`, `kMacro_MouseWheelOffset`, `kMaxMacros` (xNVSE): the 256
/// keyboard key codes, then the 8 mouse buttons, then the wheel up and down.
pub const MOUSE_BUTTON_OFFSET: u32 = 256;
pub const MOUSE_WHEEL_OFFSET: u32 = MOUSE_BUTTON_OFFSET + 8;
pub const MAX_MACROS: u32 = MOUSE_WHEEL_OFFSET + 2;

/// `IsKeyPressed`'s flags (`DIHookControl::kFlag_*` (xNVSE)): the data sources, then the
/// modifiers.
pub const FLAG_GAME_STATE: u32 = 1;
pub const FLAG_RAW_STATE: u32 = 2;
pub const FLAG_INSERTED_STATE: u32 = 4;
pub const FLAG_IGNORE_DISABLED_USER: u32 = 8;
pub const FLAG_IGNORE_DISABLED_SCRIPT: u32 = 16;
/// `kFlag_DefaultBackCompat`: what no flags means.
pub const FLAG_DEFAULT: u32 = FLAG_GAME_STATE;

/// `DIHookControl::kDisable_*` (xNVSE): what a disable stops.
pub const DISABLE_USER: u32 = 1;
pub const DISABLE_SCRIPT: u32 = 2;
pub const DISABLE_ALL: u32 = DISABLE_USER | DISABLE_SCRIPT;

/// `kDeviceType_Keyboard`, `kDeviceType_Mouse` (xNVSE).
const DEVICE_KEYBOARD: u32 = 1;
const DEVICE_MOUSE: u32 = 2;

/// `GUID_SysMouse` {6F1D2B60-D5A0-11CF-BFC7-444553540000} and `GUID_SysKeyboard` (…2B61…), in
/// memory order.
const GUID_SYS_MOUSE: [u8; 16] = [
    0x60, 0x2b, 0x1d, 0x6f, 0xa0, 0xd5, 0xcf, 0x11, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00,
];
const GUID_SYS_KEYBOARD: [u8; 16] = [
    0x61, 0x2b, 0x1d, 0x6f, 0xa0, 0xd5, 0xcf, 0x11, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00,
];

/// `DI_OK`, `DI_BUFFEROVERFLOW` (dinput.h) and `DIGDD_PEEK`.
const DI_OK: u32 = 0;
const DI_BUFFEROVERFLOW: u32 = 1;
const DIGDD_PEEK: u32 = 1;
/// `sizeof(DIDEVICEOBJECTDATA)` (DirectInput 8) and `sizeof(DIMOUSESTATE2)`.
const OBJECT_DATA_SIZE: u32 = 0x14;
const MOUSE_STATE_SIZE: u32 = 0x14;
/// `DIMOUSESTATE2`: the wheel (`lZ`) and the eight button bytes.
const MOUSE_WHEEL: u32 = 8;
const MOUSE_BUTTONS: u32 = 0xc;

/// The vtable slots the fakes call on the real objects (`IUnknown::Release`,
/// `IDirectInput8A::CreateDevice`, `IDirectInputDevice8A::GetDeviceState`, `GetDeviceData`).
const RELEASE: u32 = 0x08;
const CREATE_DEVICE: u32 = 0x0c;
const GET_DEVICE_STATE: u32 = 0x24;
const GET_DEVICE_DATA: u32 = 0x28;

/// `GetTickCount`'s import slot: xNVSE stamps the keys it queues with the system's tick count.
const GET_TICK_COUNT: u32 = 0x00fd_f060;

/// Where the fakes' slots sit (code addresses in the extenders' range, past the table's
/// routines): `FakeDirectInput`'s 11, then `FakeDirectInputDevice`'s 32.
pub const DIRECT_INPUT_CODE: u32 = ROUTINE_BASE + 0x100;
pub const DEVICE_CODE: u32 = ROUTINE_BASE + 0x200;

/// The fakes' fields (`FakeDirectInput`: vtable, `m_realDInput`, `m_refs`;
/// `FakeDirectInputDevice`: vtable, `m_device`, `m_deviceType`, `m_refs`).
const REAL: u32 = 4;
const DIRECT_INPUT_REFS: u32 = 8;
const DIRECT_INPUT_SIZE: u32 = 12;
const DEVICE_TYPE: u32 = 8;
const DEVICE_REFS: u32 = 0xc;
const DEVICE_SIZE: u32 = 16;

/// One key's state (`DIHookControl::KeyInfo` (xNVSE)).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyInfo {
    /// The state DirectInput gave at the last read.
    pub raw_state: bool,
    /// The state the game was given at the last read.
    pub game_state: bool,
    /// A script pushed or held the key down at the last read.
    pub inserted_state: bool,
    /// `HoldKey`.
    pub hold: bool,
    /// `TapKey`, until the next read.
    pub tap: bool,
    /// The player can't press the key.
    pub user_disable: bool,
    /// Scripts can't press the key.
    pub script_disable: bool,
}

impl KeyInfo {
    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::KeyInfo::Process.
    /// One read of the key: what the device says, filtered (a player-disabled key up; a held or
    /// tapped key down unless scripts are disabled on it, the tap used up). `ShouldIgnoreKey` is
    /// false for every key in xNVSE.
    pub fn process(&mut self, mut key_down: bool) -> bool {
        self.inserted_state = false;
        self.raw_state = key_down;
        if self.user_disable {
            key_down = false;
        }
        if !self.script_disable {
            if self.hold {
                self.inserted_state = true;
            }
            if self.tap {
                self.inserted_state = true;
                self.tap = false;
            }
            if self.inserted_state {
                key_down = true;
            }
        }
        self.game_state = key_down;
        key_down
    }
}

/// The most keys `MenuTapKey`, `MenuHoldKey` and `MenuReleaseKey` queue with the size caps row
/// (`NV_NVSE_SIZE_CAPS`); the rest are dropped.
pub const MAX_QUEUED: usize = 4096;
/// The queued keys past which the game stops as out of memory without the row: xNVSE's queue (a
/// `std::deque` of 20-byte records) holds about 1.3 GB by then, about what its 32-bit process has.
pub const OUT_OF_MEMORY_QUEUED: usize = 1 << 26;

/// A `DIDEVICEOBJECTDATA`: offset (the key), data (0x80 down), time stamp, sequence,
/// application data.
pub type ObjectData = [u32; 5];

/// `DIHookControl` (xNVSE) and what `Commands_Input.cpp` keeps beside it.
#[derive(Debug)]
pub struct State {
    /// `m_keys`: the keyboard's 256, the mouse's 8 buttons, the wheel's 2.
    pub keys: [KeyInfo; MAX_MACROS as usize],
    /// `m_bufferedPresses`: keys for `GetDeviceData`, oldest first.
    pub buffered: VecDeque<ObjectData>,
    /// `s_disabledControls` (`Commands_Input.cpp`): the disable masks of the 28 controls.
    pub disabled_controls: [u8; 28],
    /// `s_registeredControls`: a key's custom-control registrations, the mod indexes.
    pub registered_controls: std::collections::BTreeMap<u32, std::collections::BTreeSet<u8>>,
    /// The fakes' vtables, made at install.
    direct_input_vtable: u32,
    device_vtable: u32,
}

impl Default for State {
    fn default() -> State {
        State {
            keys: [KeyInfo::default(); MAX_MACROS as usize],
            buffered: VecDeque::new(),
            disabled_controls: [0; 28],
            registered_controls: Default::default(),
            direct_input_vtable: 0,
            device_vtable: 0,
        }
    }
}

impl State {
    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::IsKeyPressed.
    /// Whether `keycode` is down in the sources `flags` names (no flags: the game's), counting a
    /// disabled key as up when the flags say so.
    pub fn is_key_pressed(&self, keycode: u32, flags: u32) -> bool {
        if keycode >= MAX_MACROS {
            return false;
        }
        let flags = if flags == 0 { FLAG_DEFAULT } else { flags };
        let info = &self.keys[keycode as usize];
        let mut result = false;
        if flags & FLAG_GAME_STATE != 0 {
            result |= info.game_state;
        }
        if flags & FLAG_RAW_STATE != 0 {
            result |= info.raw_state;
        }
        if flags & FLAG_INSERTED_STATE != 0 {
            result |= info.inserted_state;
        }
        let disable = (flags & FLAG_IGNORE_DISABLED_USER != 0 && info.user_disable)
            || (flags & FLAG_IGNORE_DISABLED_SCRIPT != 0 && info.script_disable);
        result && !disable
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::IsKeyDisabled.
    pub fn is_key_disabled(&self, keycode: u32) -> bool {
        self.keys
            .get(keycode as usize)
            .is_some_and(|k| k.user_disable || k.script_disable)
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::SetKeyDisableState.
    /// Disables or enables `keycode` for the player, scripts or both (`mask`; 0 is both).
    pub fn set_key_disable_state(&mut self, keycode: u32, disable: bool, mask: u32) {
        let mask = if mask == 0 { DISABLE_ALL } else { mask };
        if let Some(info) = self.keys.get_mut(keycode as usize) {
            if mask & DISABLE_USER != 0 {
                info.user_disable = disable;
            }
            if mask & DISABLE_SCRIPT != 0 {
                info.script_disable = disable;
            }
        }
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::SetKeyHeldState.
    pub fn set_key_held_state(&mut self, keycode: u32, hold: bool) {
        if let Some(info) = self.keys.get_mut(keycode as usize) {
            info.hold = hold;
        }
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::TapKey.
    pub fn tap_key(&mut self, keycode: u32) {
        if let Some(info) = self.keys.get_mut(keycode as usize) {
            info.tap = true;
        }
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::BufferedKeyTap, BufferedKeyPress, BufferedKeyRelease.
    /// Queues `key` going down (`down`), up, or (`tap`) down then up, stamped `time` (one
    /// stamp for both of a tap), sequence 0, application data -1. xNVSE's queue has no end, and
    /// the game drains it only when it reads buffered keys: with the size caps row (`caps`) a key
    /// past [`MAX_QUEUED`] is dropped; without it the game stops as out of memory past
    /// [`OUT_OF_MEMORY_QUEUED`], about where xNVSE's 32-bit process would.
    pub fn buffered_key(&mut self, key: u32, time: u32, down: bool, tap: bool, caps: bool) {
        let event = |data: u32| [key, data, time, 0, u32::MAX];
        let cap = if caps { MAX_QUEUED } else { usize::MAX };
        if tap {
            self.queue(event(0x80), cap, OUT_OF_MEMORY_QUEUED);
            self.queue(event(0), cap, OUT_OF_MEMORY_QUEUED);
        } else {
            self.queue(
                event(if down { 0x80 } else { 0 }),
                cap,
                OUT_OF_MEMORY_QUEUED,
            );
        }
    }

    /// One key into the queue: dropped at `cap`, the out-of-memory stop at `out_of_memory_at`.
    fn queue(&mut self, event: ObjectData, cap: usize, out_of_memory_at: usize) {
        let len = self.buffered.len();
        if len >= cap {
            return;
        }
        if len >= out_of_memory_at {
            crate::commands::array::out_of_memory("the queue of keys for the buffered reads");
        }
        self.buffered.push_back(event);
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::ProcessKeyboardData.
    /// The keyboard's 256 bytes as the device gave them, made the game's (0x80 down, 0 up).
    pub fn process_keyboard_data(&mut self, data: &mut [u8]) {
        for (idx, byte) in data.iter_mut().enumerate().take(256) {
            let down = self.keys[idx].process(*byte != 0);
            *byte = if down { 0x80 } else { 0 };
        }
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
    // DIHookControl::ProcessMouseData.
    /// The mouse's eight button bytes, made the game's; the wheel's motion `wheel` read as the
    /// two wheel keys (not given back to the game).
    pub fn process_mouse_data(&mut self, buttons: &mut [u8; 8], wheel: i32) {
        for (idx, byte) in buttons.iter_mut().enumerate() {
            let down = self.keys[MOUSE_BUTTON_OFFSET as usize + idx].process(*byte != 0);
            *byte = if down { 0x80 } else { 0 };
        }
        for (idx, down) in [wheel > 0, wheel < 0].into_iter().enumerate() {
            self.keys[MOUSE_WHEEL_OFFSET as usize + idx].process(down);
        }
    }
}

pub(crate) fn state(e: &mut Engine) -> &mut State {
    &mut e.ext::<Nvse>().input
}

fn ret(eax: u32) -> Ret {
    Ret {
        eax,
        ..Ret::default()
    }
}

/// A slot the fake answers by calling the same slot of the real object with the same words
/// (`m_device->X(...)`, `m_realDInput->X(...)`); both fakes keep the real object at `+4`.
fn forward<const SLOT: u32>(e: &mut Engine, a: &[u32]) -> Ret {
    let real = e.mem.u32(a[0] + REAL);
    e.vcall(real, 4 * SLOT, &a[1..])
}

/// `AddRef` and `Release` of either fake, its count at `refs`: the real object is released
/// only when the fake's count reaches 0, and the fake goes then (`delete this`).
fn add_ref(e: &mut Engine, this: u32, refs: u32) -> Ret {
    let count = e.mem.u32(this + refs).wrapping_add(1);
    e.mem.set_u32(this + refs, count);
    ret(count)
}

fn release(e: &mut Engine, this: u32, refs: u32) -> Ret {
    let count = e.mem.u32(this + refs).wrapping_sub(1);
    e.mem.set_u32(this + refs, count);
    if count != 0 {
        return ret(count);
    }
    let real = e.mem.u32(this + REAL);
    e.vcall(real, RELEASE, &[]);
    e.mem.free(this);
    ret(0)
}

fn direct_input_add_ref(e: &mut Engine, a: &[u32]) -> Ret {
    add_ref(e, a[0], DIRECT_INPUT_REFS)
}

fn direct_input_release(e: &mut Engine, a: &[u32]) -> Ret {
    release(e, a[0], DIRECT_INPUT_REFS)
}

fn device_add_ref(e: &mut Engine, a: &[u32]) -> Ret {
    add_ref(e, a[0], DEVICE_REFS)
}

fn device_release(e: &mut Engine, a: &[u32]) -> Ret {
    release(e, a[0], DEVICE_REFS)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
// Hook_DirectInput8Create_Execute.
/// The wrap of `DirectInput8Create(instance, version, interface id, out, outer)`: the real
/// object into a local; on `DI_OK` a `FakeDirectInput` around it (its count 1) through `out`.
/// A failure is answered as it came, `out` untouched.
fn direct_input8_create(e: &mut Engine, a: &[u32]) -> Ret {
    let w = |i: usize| a.get(i).copied().unwrap_or(0);
    let (instance, version, interface, out, outer) = (w(0), w(1), w(2), w(3), w(4));
    let (hr, real) = e.with_stack(4, |e, local| {
        let hr = e
            .call_next(
                DIRECT_INPUT8_CREATE.addr(),
                &[instance, version, interface, local.addr(), outer],
            )
            .u32();
        (hr, e.mem.u32(local.addr()))
    });
    if hr != DI_OK {
        return ret(hr);
    }
    let vtable = state(e).direct_input_vtable;
    let fake = e.mem.alloc(DIRECT_INPUT_SIZE);
    e.mem.set_u32(fake, vtable);
    e.mem.set_u32(fake + REAL, real);
    e.mem.set_u32(fake + DIRECT_INPUT_REFS, 1);
    e.mem.set_u32(out, fake);
    ret(DI_OK)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
// FakeDirectInput::CreateDevice.
/// `CreateDevice(guid, out, outer)`: the system keyboard and mouse made by the real object into
/// a local and, on `DI_OK`, put behind a `FakeDirectInputDevice` (its count 1, its type) through
/// `out`; any other device is the real object's call as it is.
fn direct_input_create_device(e: &mut Engine, a: &[u32]) -> Ret {
    let (this, guid, out, outer) = (a[0], a[1], a[2], a[3]);
    let real = e.mem.u32(this + REAL);
    let id = e.mem.bytes(guid, 16);
    let kind = if id == GUID_SYS_KEYBOARD {
        DEVICE_KEYBOARD
    } else if id == GUID_SYS_MOUSE {
        DEVICE_MOUSE
    } else {
        return e.vcall(real, CREATE_DEVICE, &[guid, out, outer]);
    };
    let (hr, device) = e.with_stack(4, |e, local| {
        let hr = e
            .vcall(real, CREATE_DEVICE, &[guid, local.addr(), outer])
            .u32();
        (hr, e.mem.u32(local.addr()))
    });
    if hr != DI_OK {
        return ret(hr);
    }
    let vtable = state(e).device_vtable;
    let fake = e.mem.alloc(DEVICE_SIZE);
    e.mem.set_u32(fake, vtable);
    e.mem.set_u32(fake + REAL, device);
    e.mem.set_u32(fake + DEVICE_TYPE, kind);
    e.mem.set_u32(fake + DEVICE_REFS, 1);
    e.mem.set_u32(out, fake);
    ret(hr)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
// FakeDirectInputDevice::GetDeviceState.
/// `GetDeviceState(size, data)`. The keyboard: the real device's 256 bytes into a buffer of
/// xNVSE's, through [`State::process_keyboard_data`], then as many as `size` asks (256 at most)
/// to `data`. The mouse: `size` must be a `DIMOUSESTATE2`'s (xNVSE's assertion stops the game
/// otherwise), read in place and its buttons and wheel through [`State::process_mouse_data`].
/// A failed read is answered as it came, nothing processed.
fn device_get_device_state(e: &mut Engine, a: &[u32]) -> Ret {
    let (this, size, data) = (a[0], a[1], a[2]);
    let real = e.mem.u32(this + REAL);
    if e.mem.u32(this + DEVICE_TYPE) == DEVICE_KEYBOARD {
        let hr = e.with_stack(MAX_MACROS.next_multiple_of(4), |e, raw| {
            let hr = e.vcall(real, GET_DEVICE_STATE, &[256, raw.addr()]).u32();
            if hr == DI_OK {
                let mut bytes = e.mem.bytes(raw.addr(), 256);
                state(e).process_keyboard_data(&mut bytes);
                e.mem.write(data, &bytes[..size.min(256) as usize]);
            }
            hr
        });
        return ret(hr);
    }
    assert!(
        size == MOUSE_STATE_SIZE,
        "Assertion failed in Hooks_DirectInput8Create.cpp (86): outDataLen == sizeof(DIMOUSESTATE2)"
    );
    let hr = e
        .vcall(real, GET_DEVICE_STATE, &[MOUSE_STATE_SIZE, data])
        .u32();
    if hr != DI_OK {
        return ret(hr);
    }
    let mut buttons = [0u8; 8];
    for (i, b) in buttons.iter_mut().enumerate() {
        *b = e.mem.u8(data + MOUSE_BUTTONS + i as u32);
    }
    let wheel = e.mem.i32(data + MOUSE_WHEEL);
    state(e).process_mouse_data(&mut buttons, wheel);
    e.mem.write(data + MOUSE_BUTTONS, &buttons);
    ret(hr)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_DirectInput8Create.cpp:
// FakeDirectInputDevice::GetDeviceData, DIHookControl::ProcessBufferedData.
/// `GetDeviceData(record size, records, count, flags)`, the same for both fakes. With nothing
/// queued it is the real device's call. Otherwise the real records first (as many as `*count`
/// asks), then queued keys for the rest of the count; peeking only counts them, and peeking
/// with records stops the game (xNVSE's `HALT`). After the first queued record each one is
/// written at the second record's place (xNVSE moves its pointer to `records + size`, not on
/// from where it was); the game reads one record at a time, where this never shows.
fn device_get_device_data(e: &mut Engine, a: &[u32]) -> Ret {
    let (this, size, records, count, flags) = (a[0], a[1], a[2], a[3], a[4]);
    assert!(
        size == OBJECT_DATA_SIZE,
        "Assertion failed in Hooks_DirectInput8Create.cpp (405): dataSize == sizeof(DIDEVICEOBJECTDATA)"
    );
    let device = e.mem.u32(this + REAL);
    if state(e).buffered.is_empty() {
        return e.vcall(device, GET_DEVICE_DATA, &[size, records, count, flags]);
    }
    let requested = e.mem.u32(count);
    let (hr, real_events) = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), requested);
        let hr = e
            .vcall(
                device,
                GET_DEVICE_DATA,
                &[size, records, local.addr(), flags],
            )
            .u32();
        (hr, e.mem.u32(local.addr()))
    });
    if hr != DI_OK && hr != DI_BUFFEROVERFLOW {
        e.mem.set_u32(count, real_events);
        return ret(hr);
    }
    let mut virtual_out = records.wrapping_add(size.wrapping_mul(real_events));
    let virtual_requested = requested.wrapping_sub(real_events);
    let mut virtual_events = 0u32;
    if flags & DIGDD_PEEK != 0 {
        if records != 0 {
            panic!("DIHookControl::ProcessBufferedData: can't handle non-NULL data in peek mode");
        }
        let available = state(e).buffered.len() as u32;
        virtual_events = available.min(virtual_requested);
    } else {
        while virtual_events < virtual_requested {
            let Some(event) = state(e).buffered.pop_front() else {
                break;
            };
            if records != 0 {
                for (k, word) in event.iter().enumerate() {
                    e.mem.set_u32(virtual_out + 4 * k as u32, *word);
                }
                virtual_out = records.wrapping_add(size);
            }
            virtual_events += 1;
        }
    }
    e.mem
        .set_u32(count, real_events.wrapping_add(virtual_events));
    ret(hr)
}

/// `FakeDirectInput`'s slots (`IDirectInput8A`, dinput.h order).
const DIRECT_INPUT_SLOTS: [AbiFn; 11] = [
    forward::<0>,
    direct_input_add_ref,
    direct_input_release,
    direct_input_create_device,
    forward::<4>,
    forward::<5>,
    forward::<6>,
    forward::<7>,
    forward::<8>,
    forward::<9>,
    forward::<10>,
];

/// `FakeDirectInputDevice`'s slots (`IDirectInputDevice8A`, dinput.h order).
const DEVICE_SLOTS: [AbiFn; 32] = [
    forward::<0>,
    device_add_ref,
    device_release,
    forward::<3>,
    forward::<4>,
    forward::<5>,
    forward::<6>,
    forward::<7>,
    forward::<8>,
    device_get_device_state,
    device_get_device_data,
    forward::<11>,
    forward::<12>,
    forward::<13>,
    forward::<14>,
    forward::<15>,
    forward::<16>,
    forward::<17>,
    forward::<18>,
    forward::<19>,
    forward::<20>,
    forward::<21>,
    forward::<22>,
    forward::<23>,
    forward::<24>,
    forward::<25>,
    forward::<26>,
    forward::<27>,
    forward::<28>,
    forward::<29>,
    forward::<30>,
    forward::<31>,
];

/// The time stamp xNVSE gives a queued key: the system's tick count.
pub(crate) fn tick_count(e: &mut Engine) -> u32 {
    e.call(GET_TICK_COUNT, &[]).u32()
}

/// `Hook_DirectInput8Create_Init` and `Commands_Input_Init` (xNVSE): the fakes' code and vtables,
/// and the wrap of the import slot.
pub fn install(e: &mut Engine) {
    let direct_input_vtable = crate::table::alloc(e, 4 * DIRECT_INPUT_SLOTS.len() as u32);
    for (i, f) in DIRECT_INPUT_SLOTS.into_iter().enumerate() {
        let at = DIRECT_INPUT_CODE + 4 * i as u32;
        e.register(at, f);
        e.mem.set_u32(direct_input_vtable + 4 * i as u32, at);
    }
    let device_vtable = crate::table::alloc(e, 4 * DEVICE_SLOTS.len() as u32);
    for (i, f) in DEVICE_SLOTS.into_iter().enumerate() {
        let at = DEVICE_CODE + 4 * i as u32;
        e.register(at, f);
        e.mem.set_u32(device_vtable + 4 * i as u32, at);
    }
    let s = state(e);
    s.direct_input_vtable = direct_input_vtable;
    s.device_vtable = device_vtable;
    e.wrap(DIRECT_INPUT8_CREATE, direct_input8_create);
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
