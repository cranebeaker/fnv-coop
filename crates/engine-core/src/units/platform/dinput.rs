//! The platform's DirectInput 8 layer: `DirectInput8Create` (DINPUT8 import, slot `00fdf02c`,
//! behind the thunk `00ee7f10`), an `IDirectInput8A` and the `IDirectInputDevice8A`s for the
//! system keyboard and mouse, with the documented DirectInput 8 semantics, fed by the viewer's
//! keyboard and mouse ([`key`], [`mouse_move`], [`mouse_button`]; tests inject the same way).
//!
//! The objects live in the engine's memory as the game sees COM objects: a vtable pointer at
//! `+0`, the vtables in engine memory, each slot the address of a platform function (the
//! addresses from [`CODE_BASE`] up). A slot this layer does not provide is left unregistered:
//! a call to it stops the run as an open function and is recorded as a miss with its address
//! (the slot is `(address - DIRECT_INPUT_CODE) / 4` or `(address - DEVICE_CODE) / 4`, named in
//! [`DIRECT_INPUT_SLOTS`] and [`DEVICE_SLOTS`]); none answers a made-up success.
//!
//! What the game calls (FalloutNV.exe 1.4.0.525): the input object's start-up `00a22660`
//! (`DirectInput8Create`, `CreateDevice` for the keyboard and the mouse, `SetCooperativeLevel`,
//! `SetDataFormat`, `SetProperty(DIPROP_BUFFERSIZE)`, `GetCapabilities`), the joystick search
//! `00a252d0` (`EnumDevices`), `Controls::Poll` `00a23010` (`Acquire`, `Poll`,
//! `GetDeviceState`), the key reader `00a23820` (`GetDeviceData`) and the releases (`Unacquire`,
//! `Release`). There is no joystick: `EnumDevices` lists the keyboard and the mouse only.

use crate::{Engine, Ptr};

/// Where the platform's DirectInput functions sit (code addresses no exe code uses): the
/// `IDirectInput8A` slots from [`DIRECT_INPUT_CODE`], the device slots from [`DEVICE_CODE`].
pub const CODE_BASE: u32 = 0x7e00_0000;
pub const DIRECT_INPUT_CODE: u32 = CODE_BASE;
pub const DEVICE_CODE: u32 = CODE_BASE + 0x100;

/// `IDirectInput8A`'s slots (dinput.h order).
pub const DIRECT_INPUT_SLOTS: [&str; 11] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "CreateDevice",
    "EnumDevices",
    "GetDeviceStatus",
    "RunControlPanel",
    "Initialize",
    "FindDevice",
    "EnumDevicesBySemantics",
    "ConfigureDevices",
];
/// `IDirectInputDevice8A`'s slots (dinput.h order).
pub const DEVICE_SLOTS: [&str; 32] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetCapabilities",
    "EnumObjects",
    "GetProperty",
    "SetProperty",
    "Acquire",
    "Unacquire",
    "GetDeviceState",
    "GetDeviceData",
    "SetDataFormat",
    "SetEventNotification",
    "SetCooperativeLevel",
    "GetObjectInfo",
    "GetDeviceInfo",
    "RunControlPanel",
    "Initialize",
    "CreateEffect",
    "EnumEffects",
    "GetEffectInfo",
    "GetForceFeedbackState",
    "SendForceFeedbackCommand",
    "EnumCreatedEffectObjects",
    "Escape",
    "Poll",
    "SendDeviceData",
    "EnumEffectsInFile",
    "WriteEffectToFile",
    "BuildActionMap",
    "SetActionMap",
    "GetImageInfo",
];

/// Result codes (winerror.h / dinput.h).
pub const DI_OK: u32 = 0;
pub const S_FALSE: u32 = 1;
pub const DI_NOEFFECT: u32 = 1;
pub const DI_BUFFEROVERFLOW: u32 = 1;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const DIERR_INVALIDPARAM: u32 = 0x8007_0057;
pub const DIERR_NOTACQUIRED: u32 = 0x8007_000c;
pub const DIERR_ACQUIRED: u32 = 0x8007_00aa;
pub const DIERR_NOTBUFFERED: u32 = 0x8004_0207;
pub const DIERR_DEVICENOTREG: u32 = 0x8004_0154;
pub const DIERR_UNSUPPORTED: u32 = 0x8000_4001;
pub const DIERR_OLDDIRECTINPUTVERSION: u32 = 0x8007_047e;

/// `IID_IDirectInput8A` {BF798030-483A-4DA2-AA99-5D64ED369700}.
const IID_IDIRECTINPUT8A: [u8; 16] = [
    0x30, 0x80, 0x79, 0xbf, 0x3a, 0x48, 0xa2, 0x4d, 0xaa, 0x99, 0x5d, 0x64, 0xed, 0x36, 0x97, 0x00,
];
/// `GUID_SysMouse` {6F1D2B60-D5A0-11CF-BFC7-444553540000} and `GUID_SysKeyboard` (…2B61…).
pub const GUID_SYS_MOUSE: [u8; 16] = [
    0x60, 0x2b, 0x1d, 0x6f, 0xa0, 0xd5, 0xcf, 0x11, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00,
];
pub const GUID_SYS_KEYBOARD: [u8; 16] = [
    0x61, 0x2b, 0x1d, 0x6f, 0xa0, 0xd5, 0xcf, 0x11, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00,
];
/// `DIPROP_BUFFERSIZE` (`MAKEDIPROP(1)`: the value 1 in place of a GUID pointer).
const DIPROP_BUFFERSIZE: u32 = 1;
/// The DirectInput version the game asks for (`DIRECTINPUT_VERSION` 0x0800).
const DIRECTINPUT_VERSION: u32 = 0x0800;
/// `EnumDevices` device classes and flags.
const DI8DEVCLASS_ALL: u32 = 0;
const DI8DEVCLASS_POINTER: u32 = 2;
const DI8DEVCLASS_KEYBOARD: u32 = 3;
const DI8DEVCLASS_GAMECTRL: u32 = 4;
const DIEDFL_ATTACHEDONLY: u32 = 1;
/// `DIENUM_CONTINUE`.
const DIENUM_CONTINUE: u32 = 1;
/// The device types (`DI8DEVTYPE_*` with their subtypes in bits 8-15): the system mouse
/// (`DI8DEVTYPE_MOUSE`, `DI8DEVTYPEMOUSE_UNKNOWN`) and keyboard (`DI8DEVTYPE_KEYBOARD`,
/// `DI8DEVTYPEKEYBOARD_PCENH`).
const MOUSE_DEVICE_TYPE: u32 = 0x0112;
const KEYBOARD_DEVICE_TYPE: u32 = 0x0413;
/// `DIDEVICEINSTANCEA` (0x244 bytes) and `DIDEVCAPS` (0x2c bytes) sizes,
/// `DIDEVICEOBJECTDATA` (0x14 bytes, DirectInput 8).
const DEVICE_INSTANCE_SIZE: u32 = 0x244;
const DEVICE_CAPS_SIZE: u32 = 0x2c;
const OBJECT_DATA_SIZE: u32 = 0x14;
/// `DIDC_ATTACHED`.
const DIDC_ATTACHED: u32 = 1;
/// `SetCooperativeLevel` flags.
const DISCL_EXCLUSIVE: u32 = 1;
const DISCL_NONEXCLUSIVE: u32 = 2;
const DISCL_FOREGROUND: u32 = 4;
const DISCL_BACKGROUND: u32 = 8;
/// The mouse this layer offers: three axes (x, y, wheel) and the eight buttons `DIMOUSESTATE2`
/// holds; the keyboard: the 256 key codes of its state.
const MOUSE_AXES: u32 = 3;
const MOUSE_BUTTONS: u32 = 8;
const KEYBOARD_KEYS: u32 = 256;

/// One device's state.
#[derive(Debug, Clone)]
struct Device {
    keyboard: bool,
    references: u32,
    /// `SetDataFormat`'s data size (0 until set).
    data_size: u32,
    acquired: bool,
    /// `DIPROP_BUFFERSIZE` (0: not buffered).
    buffer_size: u32,
    /// Buffered events (`dwOfs`, `dwData`, time stamp, sequence) and whether one was lost.
    events: std::collections::VecDeque<[u32; 4]>,
    overflowed: bool,
}

/// The input state behind the devices (`Kernel::input`).
#[derive(Debug, Default)]
pub struct Input {
    /// The vtables once made.
    direct_input_vtable: u32,
    device_vtable: u32,
    /// `IDirectInput8A` objects and their reference counts.
    interfaces: std::collections::BTreeMap<u32, u32>,
    devices: std::collections::BTreeMap<u32, Device>,
    /// Keys down, by DirectInput key code (`DIK_*`, the scan code).
    keys: Vec<bool>,
    /// Keys that went down since `GetAsyncKeyState` last asked about them, by key code.
    async_pressed: Vec<bool>,
    /// Mouse motion since the last read (relative axes) and the buttons down.
    mouse_motion: [i32; 3],
    mouse_buttons: [bool; 8],
    sequence: u32,
}

impl Input {
    fn key_down(&self, code: usize) -> bool {
        self.keys.get(code).copied().unwrap_or(false)
    }
}

fn register_code(e: &mut Engine) {
    let direct_input: [(u32, crate::AbiFn); 5] = [
        (1, crate::entry!(0, direct_input_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, direct_input_release(Ptr) -> u32).1),
        (
            3,
            crate::entry!(0, create_device(Ptr, Ptr, Ptr, u32) -> u32).1,
        ),
        (
            4,
            crate::entry!(0, enum_devices(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
    ];
    for (slot, f) in direct_input {
        e.register(DIRECT_INPUT_CODE + 4 * slot, f);
    }
    let device: [(u32, crate::AbiFn); 13] = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, device_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, device_release(Ptr) -> u32).1),
        (3, crate::entry!(0, get_capabilities(Ptr, Ptr) -> u32).1),
        (6, crate::entry!(0, set_property(Ptr, u32, Ptr) -> u32).1),
        (7, crate::entry!(0, acquire(Ptr) -> u32).1),
        (8, crate::entry!(0, unacquire(Ptr) -> u32).1),
        (
            9,
            crate::entry!(0, get_device_state(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            10,
            crate::entry!(0, get_device_data(Ptr, u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (11, crate::entry!(0, set_data_format(Ptr, Ptr) -> u32).1),
        (
            13,
            crate::entry!(0, set_cooperative_level(Ptr, u32, u32) -> u32).1,
        ),
        (25, crate::entry!(0, poll(Ptr) -> u32).1),
        (
            12,
            crate::entry!(0, set_event_notification(Ptr, u32) -> u32).1,
        ),
    ];
    for (slot, f) in device {
        e.register(DEVICE_CODE + 4 * slot, f);
    }
}

/// The vtable with `slots` entries from `code`, made once.
fn vtable(e: &mut Engine, code: u32, slots: u32) -> u32 {
    let table = e.mem.alloc(4 * slots);
    for i in 0..slots {
        e.mem.set_u32(table + 4 * i, code + 4 * i);
    }
    table
}

// Platform: replaces DINPUT8.DLL DirectInput8Create (import)
/// `DirectInput8Create(instance, version, interface id, out, outer)`: for version 0x0800 and
/// `IID_IDirectInput8A`, a new `IDirectInput8A` (reference count 1) through `out` and `DI_OK`.
/// Another version is `DIERR_OLDDIRECTINPUTVERSION`, another interface `E_NOINTERFACE`, an
/// outer object (aggregation) is not provided.
pub fn direct_input8_create(
    e: &mut Engine,
    _instance: u32,
    version: u32,
    interface: Ptr,
    out: Ptr,
    outer: u32,
) -> u32 {
    if outer != 0 {
        panic!("DirectInput8Create with an outer object (aggregation) is not provided");
    }
    if version != DIRECTINPUT_VERSION {
        e.mem.set_u32(out.addr(), 0);
        return DIERR_OLDDIRECTINPUTVERSION;
    }
    if e.mem.bytes(interface.addr(), 16) != IID_IDIRECTINPUT8A {
        e.mem.set_u32(out.addr(), 0);
        return E_NOINTERFACE;
    }
    if e.kernel.input.direct_input_vtable == 0 {
        register_code(e);
        e.kernel.input.direct_input_vtable =
            vtable(e, DIRECT_INPUT_CODE, DIRECT_INPUT_SLOTS.len() as u32);
        e.kernel.input.device_vtable = vtable(e, DEVICE_CODE, DEVICE_SLOTS.len() as u32);
    }
    let object = e.mem.alloc(8);
    let table = e.kernel.input.direct_input_vtable;
    e.mem.set_u32(object, table);
    e.kernel.input.interfaces.insert(object, 1);
    e.mem.set_u32(out.addr(), object);
    DI_OK
}

/// `QueryInterface`: no other interface is offered (`E_NOINTERFACE`, `*out` = 0).
fn no_interface(e: &mut Engine, _this: Ptr, _interface: Ptr, out: Ptr) -> u32 {
    if !out.is_null() {
        e.mem.set_u32(out.addr(), 0);
    }
    E_NOINTERFACE
}

fn direct_input_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let count = e
        .kernel
        .input
        .interfaces
        .get_mut(&this.addr())
        .expect("AddRef on an IDirectInput8A this layer did not make");
    *count += 1;
    *count
}

fn direct_input_release(e: &mut Engine, this: Ptr) -> u32 {
    let count = e
        .kernel
        .input
        .interfaces
        .get_mut(&this.addr())
        .expect("Release on an IDirectInput8A this layer did not make");
    *count -= 1;
    let left = *count;
    if left == 0 {
        e.kernel.input.interfaces.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `CreateDevice(guid, out, outer)`: the system keyboard or mouse (`GUID_SysKeyboard`,
/// `GUID_SysMouse`), a new device (reference count 1, no data format, not acquired, not
/// buffered) through `out` and `DI_OK`; any other GUID is `DIERR_DEVICENOTREG` (no joystick).
fn create_device(e: &mut Engine, _this: Ptr, guid: Ptr, out: Ptr, outer: u32) -> u32 {
    if outer != 0 {
        panic!("IDirectInput8A::CreateDevice with an outer object is not provided");
    }
    let id = e.mem.bytes(guid.addr(), 16);
    let keyboard = if id == GUID_SYS_KEYBOARD {
        true
    } else if id == GUID_SYS_MOUSE {
        false
    } else {
        e.mem.set_u32(out.addr(), 0);
        return DIERR_DEVICENOTREG;
    };
    let object = e.mem.alloc(8);
    let table = e.kernel.input.device_vtable;
    e.mem.set_u32(object, table);
    e.kernel.input.devices.insert(
        object,
        Device {
            keyboard,
            references: 1,
            data_size: 0,
            acquired: false,
            buffer_size: 0,
            events: std::collections::VecDeque::new(),
            overflowed: false,
        },
    );
    e.mem.set_u32(out.addr(), object);
    DI_OK
}

/// One `DIDEVICEINSTANCEA` (0x244 bytes): size, instance and product GUIDs, type, names.
fn device_instance(e: &mut Engine, at: u32, guid: &[u8; 16], kind: u32, name: &[u8]) {
    for i in 0..DEVICE_INSTANCE_SIZE {
        e.mem.set_u8(at + i, 0);
    }
    e.mem.set_u32(at, DEVICE_INSTANCE_SIZE);
    e.mem.write(at + 4, guid);
    e.mem.write(at + 0x14, guid);
    e.mem.set_u32(at + 0x24, kind);
    e.mem.set_cstr(at + 0x28, name);
    e.mem.set_cstr(at + 0x128, name);
}

/// `EnumDevices(class, callback, reference, flags)`: the attached devices of `class` (all, the
/// pointer class or the keyboard class; the game-controller class has none), each through
/// `callback(instance, reference)` (`stdcall`) until it answers other than `DIENUM_CONTINUE`:
/// the system mouse, then the system keyboard. `DI_OK`.
fn enum_devices(
    e: &mut Engine,
    _this: Ptr,
    class: u32,
    callback: u32,
    reference: u32,
    flags: u32,
) -> u32 {
    if flags & !DIEDFL_ATTACHEDONLY != 0 {
        panic!("IDirectInput8A::EnumDevices with flags {flags:#x} is not provided");
    }
    let mut listed: Vec<(&[u8; 16], u32, &[u8])> = Vec::new();
    match class {
        DI8DEVCLASS_ALL => {
            listed.push((&GUID_SYS_MOUSE, MOUSE_DEVICE_TYPE, b"Mouse"));
            listed.push((&GUID_SYS_KEYBOARD, KEYBOARD_DEVICE_TYPE, b"Keyboard"));
        }
        DI8DEVCLASS_POINTER => listed.push((&GUID_SYS_MOUSE, MOUSE_DEVICE_TYPE, b"Mouse")),
        DI8DEVCLASS_KEYBOARD => {
            listed.push((&GUID_SYS_KEYBOARD, KEYBOARD_DEVICE_TYPE, b"Keyboard"))
        }
        DI8DEVCLASS_GAMECTRL => {}
        _ => return DIERR_INVALIDPARAM,
    }
    for (guid, kind, name) in listed {
        let go_on = e.with_stack(DEVICE_INSTANCE_SIZE, |e, record| {
            device_instance(e, record.addr(), guid, kind, name);
            e.call(callback, &[record.addr(), reference]).u32()
        });
        if go_on != DIENUM_CONTINUE {
            break;
        }
    }
    DI_OK
}

fn device(e: &mut Engine, this: Ptr) -> &mut Device {
    e.kernel
        .input
        .devices
        .get_mut(&this.addr())
        .expect("a call on an IDirectInputDevice8A this layer did not make")
}

fn device_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references += 1;
    d.references
}

fn device_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references -= 1;
    let left = d.references;
    if left == 0 {
        e.kernel.input.devices.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `GetCapabilities(caps)`: `caps->dwSize` must be 0x2c (`DIDEVCAPS`, else
/// `DIERR_INVALIDPARAM`); attached, the device type, the axes and buttons, no POVs.
fn get_capabilities(e: &mut Engine, this: Ptr, caps: Ptr) -> u32 {
    if e.mem.u32(caps.addr()) != DEVICE_CAPS_SIZE {
        return DIERR_INVALIDPARAM;
    }
    let keyboard = device(e, this).keyboard;
    let at = caps.addr();
    for i in 1..(DEVICE_CAPS_SIZE / 4) {
        e.mem.set_u32(at + 4 * i, 0);
    }
    e.mem.set_u32(at + 4, DIDC_ATTACHED);
    if keyboard {
        e.mem.set_u32(at + 8, KEYBOARD_DEVICE_TYPE);
        e.mem.set_u32(at + 0x10, KEYBOARD_KEYS);
    } else {
        e.mem.set_u32(at + 8, MOUSE_DEVICE_TYPE);
        e.mem.set_u32(at + 0xc, MOUSE_AXES);
        e.mem.set_u32(at + 0x10, MOUSE_BUTTONS);
    }
    DI_OK
}

/// `SetProperty(property, header)`: `DIPROP_BUFFERSIZE` with a `DIPROPDWORD` (sizes 0x14 and
/// 0x10, `DIPH_DEVICE`) sets the event buffer (not while acquired: `DIERR_ACQUIRED`); other
/// properties are not provided.
fn set_property(e: &mut Engine, this: Ptr, property: u32, header: Ptr) -> u32 {
    if property != DIPROP_BUFFERSIZE {
        panic!("IDirectInputDevice8A::SetProperty({property:#x}) is not provided");
    }
    let h = header.addr();
    if e.mem.u32(h) != 0x14 || e.mem.u32(h + 4) != 0x10 || e.mem.u32(h + 0xc) != 0 {
        return DIERR_INVALIDPARAM;
    }
    let size = e.mem.u32(h + 0x10);
    let d = device(e, this);
    if d.acquired {
        return DIERR_ACQUIRED;
    }
    d.buffer_size = size;
    DI_OK
}

/// `SetDataFormat(format)`: keeps the format's data size (`DIDATAFORMAT::dwDataSize` at `+0xc`);
/// not while acquired (`DIERR_ACQUIRED`). The game's formats are the standard ones
/// (`c_dfDIKeyboard`, 256 bytes; `c_dfDIMouse2`, 20 bytes), whose layout [`get_device_state`]
/// fills.
fn set_data_format(e: &mut Engine, this: Ptr, format: Ptr) -> u32 {
    let size = e.mem.u32(format.addr() + 0xc);
    let d = device(e, this);
    if d.acquired {
        return DIERR_ACQUIRED;
    }
    let expected = if d.keyboard { 0x100 } else { 0x14 };
    if size != expected {
        panic!("IDirectInputDevice8A::SetDataFormat with a {size:#x}-byte format is not provided");
    }
    d.data_size = size;
    DI_OK
}

/// `SetCooperativeLevel(window, flags)`: exactly one of exclusive / non-exclusive and one of
/// foreground / background (else `DIERR_INVALIDPARAM`); `DI_OK`.
fn set_cooperative_level(e: &mut Engine, this: Ptr, _window: u32, flags: u32) -> u32 {
    let _ = device(e, this);
    let one_of = |a: u32, b: u32| (flags & a != 0) != (flags & b != 0);
    if !one_of(DISCL_EXCLUSIVE, DISCL_NONEXCLUSIVE) || !one_of(DISCL_FOREGROUND, DISCL_BACKGROUND) {
        return DIERR_INVALIDPARAM;
    }
    DI_OK
}

/// `SetEventNotification(event)`: no event is ever signalled by this layer, so only clearing
/// it (0) is provided.
fn set_event_notification(e: &mut Engine, this: Ptr, event: u32) -> u32 {
    let _ = device(e, this);
    if event != 0 {
        panic!("IDirectInputDevice8A::SetEventNotification with an event is not provided");
    }
    DI_OK
}

/// `Acquire()`: needs a data format (`DIERR_INVALIDPARAM`); `S_FALSE` when already acquired.
fn acquire(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    if d.data_size == 0 {
        return DIERR_INVALIDPARAM;
    }
    if d.acquired {
        return S_FALSE;
    }
    d.acquired = true;
    DI_OK
}

/// `Unacquire()`: `DI_NOEFFECT` when not acquired.
fn unacquire(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    if !d.acquired {
        return DI_NOEFFECT;
    }
    d.acquired = false;
    DI_OK
}

/// `Poll()`: the keyboard and mouse are not polled devices: `DI_NOEFFECT` when acquired,
/// `DIERR_NOTACQUIRED` when not.
fn poll(e: &mut Engine, this: Ptr) -> u32 {
    if device(e, this).acquired {
        DI_NOEFFECT
    } else {
        DIERR_NOTACQUIRED
    }
}

/// `GetDeviceState(size, data)`: acquired (`DIERR_NOTACQUIRED`) and `size` the format's
/// (`DIERR_INVALIDPARAM`). The keyboard: 256 bytes, 0x80 for each key down. The mouse
/// (`DIMOUSESTATE2`): x, y and wheel motion since the last read (relative axes, then cleared),
/// eight button bytes (0x80 down).
fn get_device_state(e: &mut Engine, this: Ptr, size: u32, data: Ptr) -> u32 {
    let d = device(e, this);
    if !d.acquired {
        return DIERR_NOTACQUIRED;
    }
    if size != d.data_size {
        return DIERR_INVALIDPARAM;
    }
    let keyboard = d.keyboard;
    let at = data.addr();
    if keyboard {
        for code in 0..KEYBOARD_KEYS {
            let down = e.kernel.input.key_down(code as usize);
            e.mem.set_u8(at + code, if down { 0x80 } else { 0 });
        }
    } else {
        let motion = std::mem::take(&mut e.kernel.input.mouse_motion);
        for (i, m) in motion.into_iter().enumerate() {
            e.mem.set_i32(at + 4 * i as u32, m);
        }
        let buttons = e.kernel.input.mouse_buttons;
        for (i, down) in buttons.into_iter().enumerate() {
            e.mem
                .set_u8(at + 0xc + i as u32, if down { 0x80 } else { 0 });
        }
    }
    DI_OK
}

/// `GetDeviceData(record size, records, in/out count, flags)`: buffered events, oldest first,
/// as `DIDEVICEOBJECTDATA` (0x14 bytes: offset, data, time stamp, sequence, application data
/// 0), up to `*count` taken out of the buffer (with `DIGDD_PEEK`, flag 1, left in); `*count`
/// becomes the number given. Needs a buffer (`DIERR_NOTBUFFERED`), acquisition
/// (`DIERR_NOTACQUIRED`) and the DirectInput 8 record size (`DIERR_INVALIDPARAM`).
/// `DI_BUFFEROVERFLOW` once after events were lost. No records asks only for the count
/// (`*count` 0xFFFFFFFF with no records flushes the buffer).
fn get_device_data(
    e: &mut Engine,
    this: Ptr,
    record_size: u32,
    records: Ptr,
    count: Ptr,
    flags: u32,
) -> u32 {
    let d = device(e, this);
    if d.buffer_size == 0 {
        return DIERR_NOTBUFFERED;
    }
    if !d.acquired {
        return DIERR_NOTACQUIRED;
    }
    if record_size != OBJECT_DATA_SIZE {
        return DIERR_INVALIDPARAM;
    }
    let peek = flags & 1 != 0;
    let overflowed = std::mem::replace(&mut d.overflowed, false);
    let wanted = e.mem.u32(count.addr());
    // The oldest `taken` events, written straight from the buffer (no list of them made); taken
    // out of it as they are written, a drain that a stop on the way still empties whole.
    let (mem, d) = (
        &mut e.mem,
        e.kernel
            .input
            .devices
            .get_mut(&this.addr())
            .expect("a call on an IDirectInputDevice8A this layer did not make"),
    );
    let taken = if records.is_null() {
        if wanted == u32::MAX && !peek {
            d.events.len()
        } else {
            0
        }
    } else {
        (wanted as usize).min(d.events.len())
    };
    let write = |mem: &mut crate::Mem, i: usize, event: [u32; 4]| {
        let at = records.addr() + OBJECT_DATA_SIZE * i as u32;
        for (k, word) in event.iter().enumerate() {
            mem.set_u32(at + 4 * k as u32, *word);
        }
        mem.set_u32(at + 0x10, 0);
    };
    if records.is_null() {
        d.events.drain(..taken);
    } else if peek {
        for (i, &event) in d.events.iter().take(taken).enumerate() {
            write(mem, i, event);
        }
    } else {
        for (i, event) in d.events.drain(..taken).enumerate() {
            write(mem, i, event);
        }
    }
    e.mem.set_u32(count.addr(), taken as u32);
    if overflowed {
        DI_BUFFEROVERFLOW
    } else {
        DI_OK
    }
}

/// A key (DirectInput key code, the scan code) going down or up: the keyboard state, and an
/// event in each acquired, buffered keyboard device (a full buffer loses it and reports the
/// overflow). The time stamp is the platform's tick count (`GetTickCount`).
pub fn key(e: &mut Engine, code: u8, down: bool) {
    let input = &mut e.kernel.input;
    if input.keys.len() < KEYBOARD_KEYS as usize {
        input.keys.resize(KEYBOARD_KEYS as usize, false);
    }
    if input.keys[code as usize] == down {
        return;
    }
    input.keys[code as usize] = down;
    if down {
        if input.async_pressed.len() < KEYBOARD_KEYS as usize {
            input.async_pressed.resize(KEYBOARD_KEYS as usize, false);
        }
        input.async_pressed[code as usize] = true;
    }
    let time = super::tick_count(e);
    let input = &mut e.kernel.input;
    input.sequence = input.sequence.wrapping_add(1);
    let event = [
        u32::from(code),
        if down { 0x80 } else { 0 },
        time,
        input.sequence,
    ];
    for d in input.devices.values_mut() {
        if d.keyboard && d.acquired && d.buffer_size != 0 {
            if d.events.len() < d.buffer_size as usize {
                d.events.push_back(event);
            } else {
                d.overflowed = true;
            }
        }
    }
}

/// `VK_TAB`, `VK_MENU` (either Alt) and the keyboard's key codes for them (`DIK_TAB`,
/// `DIK_LMENU`, `DIK_RMENU`), as Windows defines them.
const VK_TAB: u32 = 0x09;
const VK_MENU: u32 = 0x12;
const DIK_TAB: usize = 0x0f;
const DIK_LMENU: usize = 0x38;
const DIK_RMENU: usize = 0xb8;

// Platform: replaces USER32.dll GetAsyncKeyState (import)
/// `GetAsyncKeyState(virtual key)`: the 16-bit state in `AX`, bit 15 set while the key is down
/// and bit 0 when it went down since the last call that asked about it (Windows documents bit 0
/// as unreliable; the exe reads only bit 15). The exe's only caller is 0086e650 (the main loop's
/// Alt+Tab test), asking for `VK_TAB` and `VK_MENU`, which read the keyboard's `DIK_TAB` and either
/// Alt key. Any other virtual key is not provided (stops the run).
pub fn get_async_key_state(e: &mut Engine, virtual_key: u32) -> u32 {
    let codes: &[usize] = match virtual_key {
        VK_TAB => &[DIK_TAB],
        VK_MENU => &[DIK_LMENU, DIK_RMENU],
        _ => panic!("GetAsyncKeyState({virtual_key:#x}): only VK_TAB and VK_MENU are provided"),
    };
    let input = &mut e.kernel.input;
    let mut state = 0u32;
    for &code in codes {
        if input.key_down(code) {
            state |= 0x8000;
        }
        if let Some(pressed) = input.async_pressed.get_mut(code) {
            if *pressed {
                state |= 1;
                *pressed = false;
            }
        }
    }
    state
}

/// Mouse motion (mickeys) for the next read: x, y and the wheel.
pub fn mouse_move(e: &mut Engine, dx: i32, dy: i32, wheel: i32) {
    let m = &mut e.kernel.input.mouse_motion;
    m[0] = m[0].wrapping_add(dx);
    m[1] = m[1].wrapping_add(dy);
    m[2] = m[2].wrapping_add(wheel);
}

/// A mouse button (0..8) going down or up.
pub fn mouse_button(e: &mut Engine, button: usize, down: bool) {
    e.kernel.input.mouse_buttons[button] = down;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_key_state_reads_tab_and_either_alt() {
        let mut e = Engine::new();
        assert_eq!(get_async_key_state(&mut e, VK_TAB), 0);
        key(&mut e, DIK_TAB as u8, true);
        assert_eq!(
            get_async_key_state(&mut e, VK_TAB),
            0x8001,
            "down, and pressed since"
        );
        assert_eq!(
            get_async_key_state(&mut e, VK_TAB),
            0x8000,
            "bit 0 once per press"
        );
        key(&mut e, DIK_TAB as u8, false);
        assert_eq!(get_async_key_state(&mut e, VK_TAB), 0);
        key(&mut e, DIK_RMENU as u8, true);
        key(&mut e, DIK_RMENU as u8, false);
        assert_eq!(
            get_async_key_state(&mut e, VK_MENU),
            1,
            "went down and up since"
        );
        key(&mut e, DIK_LMENU as u8, true);
        assert_eq!(get_async_key_state(&mut e, VK_MENU), 0x8001);
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.mem.map(0x1000, 0x1000);
        e.mem.write(0x1000, &IID_IDIRECTINPUT8A);
        e.mem.write(0x1010, &GUID_SYS_KEYBOARD);
        e.mem.write(0x1020, &GUID_SYS_MOUSE);
        let r = direct_input8_create(
            &mut e,
            0x0040_0000,
            0x0800,
            Ptr::new(0x1000),
            Ptr::new(0x1100),
            0,
        );
        assert_eq!(r, DI_OK);
        let di = e.mem.u32(0x1100);
        (e, di)
    }

    fn make(e: &mut Engine, di: u32, guid: u32) -> u32 {
        assert_eq!(e.vcall(di, 0xc, &[guid, 0x1104, 0]).u32(), DI_OK);
        e.mem.u32(0x1104)
    }

    /// A data format record with only its data size set (`+0xc`).
    fn format(e: &mut Engine, at: u32, size: u32) -> u32 {
        e.mem.set_u32(at + 0xc, size);
        at
    }

    #[test]
    fn create_checks_the_version_and_the_interface() {
        let (mut e, di) = engine();
        assert_ne!(di, 0);
        assert_eq!(
            direct_input8_create(&mut e, 0, 0x0700, Ptr::new(0x1000), Ptr::new(0x1100), 0),
            DIERR_OLDDIRECTINPUTVERSION
        );
        assert_eq!(
            direct_input8_create(&mut e, 0, 0x0800, Ptr::new(0x1010), Ptr::new(0x1100), 0),
            E_NOINTERFACE
        );
        // The devices: keyboard and mouse; a joystick GUID isn't registered.
        assert_ne!(make(&mut e, di, 0x1010), 0);
        assert_ne!(make(&mut e, di, 0x1020), 0);
        e.mem.write(0x1030, &[7u8; 16]);
        assert_eq!(
            e.vcall(di, 0xc, &[0x1030, 0x1104, 0]).u32(),
            DIERR_DEVICENOTREG
        );
        // AddRef / Release count.
        assert_eq!(e.vcall(di, 4, &[]).u32(), 2);
        assert_eq!(e.vcall(di, 8, &[]).u32(), 1);
    }

    #[test]
    fn a_slot_not_provided_stops_as_an_open_function() {
        let (mut e, di) = engine();
        let keyboard = make(&mut e, di, 0x1010);
        let missed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.vcall(keyboard, 0x38, &[0, 0, 0]);
        }));
        assert!(missed.is_err(), "GetObjectInfo is not provided");
    }

    #[test]
    fn the_keyboard_state_and_its_buffered_events() {
        let (mut e, di) = engine();
        let k = make(&mut e, di, 0x1010);
        // Not acquired without a format; the buffer is set before acquiring.
        assert_eq!(e.vcall(k, 0x1c, &[]).u32(), DIERR_INVALIDPARAM);
        let f = format(&mut e, 0x1200, 0x100);
        assert_eq!(e.vcall(k, 0x2c, &[f]).u32(), DI_OK);
        assert_eq!(e.vcall(k, 0x34, &[1, 0x16]).u32(), DI_OK);
        assert_eq!(e.vcall(k, 0x34, &[1, 3]).u32(), DIERR_INVALIDPARAM);
        for (o, w) in [(0u32, 0x14u32), (4, 0x10), (8, 0), (0xc, 0), (0x10, 2)] {
            e.mem.set_u32(0x1300 + o, w);
        }
        assert_eq!(e.vcall(k, 0x18, &[1, 0x1300]).u32(), DI_OK);
        assert_eq!(e.vcall(k, 0x24, &[0x100, 0x1400]).u32(), DIERR_NOTACQUIRED);
        assert_eq!(e.vcall(k, 0x1c, &[]).u32(), DI_OK);
        assert_eq!(e.vcall(k, 0x1c, &[]).u32(), S_FALSE);
        assert_eq!(e.vcall(k, 0x64, &[]).u32(), DI_NOEFFECT);
        key(&mut e, 0x11, true); // W
        key(&mut e, 0x11, true); // no change, no event
        key(&mut e, 0x1e, true); // A
        key(&mut e, 0x11, false); // W up: the buffer (2) overflows
        assert_eq!(e.vcall(k, 0x24, &[0x100, 0x1400]).u32(), DI_OK);
        assert_eq!((e.mem.u8(0x1411), e.mem.u8(0x141e)), (0, 0x80));
        assert_eq!(e.vcall(k, 0x24, &[0x14, 0x1400]).u32(), DIERR_INVALIDPARAM);
        // One record at a time, as the game reads them.
        e.mem.set_u32(0x1600, 1);
        assert_eq!(
            e.vcall(k, 0x28, &[0x14, 0x1500, 0x1600, 0]).u32(),
            DI_BUFFEROVERFLOW
        );
        assert_eq!(
            (e.mem.u32(0x1600), e.mem.u32(0x1500), e.mem.u32(0x1504)),
            (1, 0x11, 0x80)
        );
        e.mem.set_u32(0x1600, 1);
        assert_eq!(e.vcall(k, 0x28, &[0x14, 0x1500, 0x1600, 0]).u32(), DI_OK);
        assert_eq!((e.mem.u32(0x1600), e.mem.u32(0x1500)), (1, 0x1e));
        e.mem.set_u32(0x1600, 1);
        assert_eq!(e.vcall(k, 0x28, &[0x14, 0x1500, 0x1600, 0]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1600), 0);
        assert_eq!(e.vcall(k, 0x20, &[]).u32(), DI_OK);
        assert_eq!(e.vcall(k, 0x20, &[]).u32(), DI_NOEFFECT);
        assert_eq!(e.vcall(k, 0x64, &[]).u32(), DIERR_NOTACQUIRED);
        assert_eq!(e.vcall(k, 8, &[]).u32(), 0);
    }

    #[test]
    fn the_mouse_reads_relative_motion_and_buttons() {
        let (mut e, di) = engine();
        let m = make(&mut e, di, 0x1020);
        e.mem.set_u32(0x1700, DEVICE_CAPS_SIZE);
        assert_eq!(e.vcall(m, 0xc, &[0x1700]).u32(), DI_OK);
        assert_eq!(
            [4u32, 8, 0xc, 0x10, 0x14].map(|o| e.mem.u32(0x1700 + o)),
            [DIDC_ATTACHED, MOUSE_DEVICE_TYPE, 3, 8, 0]
        );
        let f = format(&mut e, 0x1200, 0x14);
        assert_eq!(e.vcall(m, 0x2c, &[f]).u32(), DI_OK);
        assert_eq!(e.vcall(m, 0x1c, &[]).u32(), DI_OK);
        mouse_move(&mut e, 5, -3, 0);
        mouse_move(&mut e, 1, 0, 120);
        mouse_button(&mut e, 1, true);
        assert_eq!(e.vcall(m, 0x24, &[0x14, 0x1800]).u32(), DI_OK);
        assert_eq!([0, 4, 8].map(|o| e.mem.i32(0x1800 + o)), [6, -3, 120]);
        assert_eq!((e.mem.u8(0x180c), e.mem.u8(0x180d)), (0, 0x80));
        assert_eq!(e.vcall(m, 0x24, &[0x14, 0x1800]).u32(), DI_OK);
        assert_eq!(e.mem.i32(0x1800), 0, "motion is relative");
        // The mouse isn't buffered.
        assert_eq!(
            e.vcall(m, 0x28, &[0x14, 0x1500, 0x1600, 0]).u32(),
            DIERR_NOTBUFFERED
        );
    }

    /// `EnumDevices` over all classes gives the mouse, then the keyboard, to a stdcall callback.
    #[test]
    fn enum_devices_lists_the_mouse_and_the_keyboard() {
        let (mut e, di) = engine();
        e.register(0x00ab_0000, |e, a| {
            let n = e.mem.u32(0x1900);
            e.mem.set_u32(0x1904 + 8 * n, e.mem.u32(a[0] + 0x24));
            e.mem.set_u32(0x1908 + 8 * n, a[1]);
            e.mem.set_u32(0x1900, n + 1);
            crate::abi::RetVal::into_ret(1u32)
        });
        assert_eq!(e.vcall(di, 0x10, &[0, 0x00ab_0000, 0x77, 1]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1900), 2);
        assert_eq!(
            [0x1904, 0x1908, 0x190c, 0x1910].map(|a| e.mem.u32(a)),
            [MOUSE_DEVICE_TYPE, 0x77, KEYBOARD_DEVICE_TYPE, 0x77]
        );
        assert_eq!(e.vcall(di, 0x10, &[4, 0x00ab_0000, 0x77, 1]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1900), 2, "no game controller");
    }
}
