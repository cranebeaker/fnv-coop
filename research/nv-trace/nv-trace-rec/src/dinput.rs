//! Scripted keys fed into the game's own DirectInput keyboard, the way the port's platform
//! layer feeds them (`crates/engine-core/src/units/platform/dinput.rs`): no system-wide input
//! is synthesised, so nothing can reach another program.
//!
//! The exe's `DINPUT8!DirectInput8Create` import is redirected ([`crate::iat`]) to
//! [`direct_input8_create`], which patches the `IDirectInput8A` vtable's `CreateDevice` (slot
//! 3). When the game creates the system keyboard (`GUID_SysKeyboard`), that device's vtable gets
//! `GetDeviceState` (slot 9: the scripted keys held are set to 0x80 in the 256-byte state) and
//! `GetDeviceData` (slot 10: the scripted key changes are appended as buffered
//! `DIDEVICEOBJECTDATA` records, after the real ones). What the real device answers is passed
//! through unchanged otherwise.

use nv_win as win;
use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering::*};
use std::sync::Mutex;

type Create =
    unsafe extern "system" fn(*mut c_void, u32, *const u8, *mut *mut c_void, *mut c_void) -> i32;
type CreateDevice =
    unsafe extern "system" fn(*mut c_void, *const [u8; 16], *mut *mut c_void, *mut c_void) -> i32;
type GetDeviceState = unsafe extern "system" fn(*mut c_void, u32, *mut u8) -> i32;
type GetDeviceData = unsafe extern "system" fn(*mut c_void, u32, *mut u8, *mut u32, u32) -> i32;

pub static ORIG_CREATE: AtomicUsize = AtomicUsize::new(0);
static ORIG_CREATE_DEVICE: AtomicUsize = AtomicUsize::new(0);
static ORIG_STATE: AtomicUsize = AtomicUsize::new(0);
static ORIG_DATA: AtomicUsize = AtomicUsize::new(0);
/// The game's keyboard device, once created.
static KEYBOARD: AtomicUsize = AtomicUsize::new(0);
/// The game's mouse device, once created: while the key script runs it reads no motion and
/// no buttons, as in the port's scripted runs (a mouse over the window switches the menus to
/// their mouse path and the keys miss).
static MOUSE: AtomicUsize = AtomicUsize::new(0);

/// `GUID_SysKeyboard` {6F1D2B61-D5A0-11CF-BFC7-444553540000} in memory order.
/// `GUID_SysMouse` {6F1D2B60-D5A0-11CF-BFC7-444553540000}.
const SYS_MOUSE: [u8; 16] = [
    0x60, 0x2b, 0x1d, 0x6f, 0xa0, 0xd5, 0xcf, 0x11, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00,
];
const SYS_KEYBOARD: [u8; 16] = [
    0x61, 0x2b, 0x1d, 0x6f, 0xa0, 0xd5, 0xcf, 0x11, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00,
];
/// `DIDEVICEOBJECTDATA` (DirectInput 8): offset, data, time stamp, sequence, app data.
const OBJECT_DATA: u32 = 20;
const DIGDD_PEEK: u32 = 1;

struct Keys {
    held: [bool; 256],
    pending: VecDeque<(u8, bool)>,
    sequence: u32,
}

static KEYS: Mutex<Keys> = Mutex::new(Keys {
    held: [false; 256],
    pending: VecDeque::new(),
    sequence: 0x4e56_0000,
});

/// A scripted key change, seen by the game's next keyboard reads.
pub fn key(dik: u8, down: bool) -> bool {
    if let Ok(mut k) = KEYS.lock() {
        k.held[dik as usize] = down;
        k.pending.push_back((dik, down));
    }
    KEYBOARD.load(Acquire) != 0
}

pub(crate) unsafe fn patch_slot(
    vtable: *mut usize,
    slot: usize,
    ours: usize,
    orig: &AtomicUsize,
) -> bool {
    let at = vtable.add(slot);
    if *at == ours {
        return true;
    }
    let mut old = 0u32;
    if win::VirtualProtect(at as *mut c_void, 4, win::PAGE_READWRITE, &mut old) == 0 {
        return false;
    }
    orig.store(*at, Release);
    *at = ours;
    let mut ignored = 0;
    win::VirtualProtect(at as *mut c_void, 4, old, &mut ignored);
    true
}

pub unsafe extern "system" fn direct_input8_create(
    inst: *mut c_void,
    version: u32,
    riid: *const u8,
    out: *mut *mut c_void,
    outer: *mut c_void,
) -> i32 {
    let orig: Create = std::mem::transmute(ORIG_CREATE.load(Acquire));
    let hr = orig(inst, version, riid, out, outer);
    if hr >= 0 && !out.is_null() && !(*out).is_null() {
        let vtable = *(*out as *mut *mut usize);
        let ok = patch_slot(
            vtable,
            3,
            create_device as *const () as usize,
            &ORIG_CREATE_DEVICE,
        );
        crate::log(&format!(
            "dinput: DirectInput8Create {hr:#x}; CreateDevice patched: {ok}"
        ));
    }
    hr
}

unsafe extern "system" fn create_device(
    this: *mut c_void,
    guid: *const [u8; 16],
    out: *mut *mut c_void,
    outer: *mut c_void,
) -> i32 {
    let orig: CreateDevice = std::mem::transmute(ORIG_CREATE_DEVICE.load(Acquire));
    let hr = orig(this, guid, out, outer);
    if hr >= 0 && !guid.is_null() && *guid == SYS_MOUSE && !out.is_null() && !(*out).is_null() {
        let dev = *out;
        let vtable = *(dev as *mut *mut usize);
        let a = patch_slot(
            vtable,
            9,
            get_device_state as *const () as usize,
            &ORIG_STATE,
        );
        let b = patch_slot(
            vtable,
            10,
            get_device_data as *const () as usize,
            &ORIG_DATA,
        );
        MOUSE.store(dev as usize, Release);
        crate::log(&format!(
            "dinput: mouse created; state/data patched: {a}/{b}"
        ));
    }
    if hr >= 0 && !guid.is_null() && *guid == SYS_KEYBOARD && !out.is_null() && !(*out).is_null() {
        let dev = *out;
        let vtable = *(dev as *mut *mut usize);
        let a = patch_slot(
            vtable,
            9,
            get_device_state as *const () as usize,
            &ORIG_STATE,
        );
        let b = patch_slot(
            vtable,
            10,
            get_device_data as *const () as usize,
            &ORIG_DATA,
        );
        KEYBOARD.store(dev as usize, Release);
        crate::log(&format!(
            "dinput: keyboard created; state/data patched: {a}/{b}"
        ));
    }
    hr
}

unsafe extern "system" fn get_device_state(this: *mut c_void, size: u32, data: *mut u8) -> i32 {
    if this as usize == KEYBOARD.load(Acquire) {
        crate::input::on_poll();
    }
    let orig: GetDeviceState = std::mem::transmute(ORIG_STATE.load(Acquire));
    let hr = orig(this, size, data);
    if hr >= 0
        && this as usize == MOUSE.load(Acquire)
        && crate::input::mouse_muted()
        && !data.is_null()
    {
        std::ptr::write_bytes(data, 0, size as usize);
        return hr;
    }
    if hr >= 0 && this as usize == KEYBOARD.load(Acquire) && size == 256 && !data.is_null() {
        // `keyboard_mute=always`: the real keyboard reads nothing, only the script's keys.
        if crate::input::KEYBOARD_MUTE.load(std::sync::atomic::Ordering::Relaxed) {
            std::ptr::write_bytes(data, 0, 256);
        }
        if let Ok(k) = KEYS.lock() {
            for (i, &down) in k.held.iter().enumerate() {
                if down {
                    *data.add(i) = 0x80;
                }
            }
        }
    }
    hr
}

unsafe extern "system" fn get_device_data(
    this: *mut c_void,
    record_size: u32,
    records: *mut u8,
    count: *mut u32,
    flags: u32,
) -> i32 {
    let orig: GetDeviceData = std::mem::transmute(ORIG_DATA.load(Acquire));
    // `count` comes in as the room and goes out as the records given.
    let room = if count.is_null() { 0 } else { *count };
    let hr = orig(this, record_size, records, count, flags);
    if hr >= 0
        && this as usize == MOUSE.load(Acquire)
        && crate::input::mouse_muted()
        && !count.is_null()
    {
        *count = 0;
        return hr;
    }
    if hr < 0
        || this as usize != KEYBOARD.load(Acquire)
        || records.is_null()
        || count.is_null()
        || record_size != OBJECT_DATA
        || flags & DIGDD_PEEK != 0
    {
        return hr;
    }
    // The caller's buffer holds as many records as it asked for; the real ones are in front.
    let Ok(mut k) = KEYS.lock() else { return hr };
    if k.pending.is_empty() {
        return hr;
    }
    let mut n = if crate::input::KEYBOARD_MUTE.load(std::sync::atomic::Ordering::Relaxed) {
        // The real key events are dropped; only the script's go in.
        0
    } else {
        *count
    };
    while n < room {
        let Some((dik, down)) = k.pending.pop_front() else {
            break;
        };
        k.sequence = k.sequence.wrapping_add(1);
        let at = records.add((n * OBJECT_DATA) as usize) as *mut u32;
        *at = u32::from(dik);
        *at.add(1) = if down { 0x80 } else { 0 };
        *at.add(2) = win::GetTickCount();
        *at.add(3) = k.sequence;
        *at.add(4) = 0;
        n += 1;
        crate::log(&format!(
            "dinput: key {dik:02x} {} delivered",
            if down { "down" } else { "up" }
        ));
    }
    *count = n;
    hr
}
