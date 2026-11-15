//! The game's DirectInput joysticks from the PC's game controllers (docs/coop/SPECS.md S39,
//! "controllers"), and the system's XInput and DirectInput behind [`crate::pads`].
//!
//! What the game does with a joystick (FalloutNV.exe 1.4.0.525, from this repository's
//! translations): its input start-up (`00a252d0`, when the setting at `011f35d0` is on; its
//! start-up value is 1) calls `EnumDevices(0, 00a24830, controls, DIEDFL_ATTACHEDONLY)`. The
//! callback (`00a24830`) takes a device whose type byte is 0x14 or 0x15 (`DI8DEVTYPE_JOYSTICK`,
//! `DI8DEVTYPE_GAMEPAD`), keeps its `DIDEVICEINSTANCEA`, `CreateDevice`s it by its instance GUID,
//! sets the cooperative level (`DISCL_NONEXCLUSIVE | DISCL_FOREGROUND`), the data format at
//! `01100ebc` (0x50 bytes of state), reads `GetCapabilities`, `EnumObjects`s it into two masks,
//! and sets the axis range -100 to 100 (`DIPROP_RANGE`, `00a236f0`) and the dead zone
//! (`DIPROP_DEADZONE`, `00a23750`). `Controls::Poll` (`00a23010`) then `Acquire`s, `Poll`s and
//! reads each one's state (`GetDeviceState`, 0x50 bytes) every frame, or clears it when the
//! acquire fails. Stopping (`00a252d0(0)`) `Unacquire`s and `Release`s them.
//!
//! Here: the platform's `EnumDevices` lists, after the keyboard and the mouse, the game
//! controllers [`crate::pads`] shows the game (`NV_PAD`; an Xbox pad's own DirectInput side never:
//! it is the XInput pad), each with the system's own `DIDEVICEINSTANCEA` (no pointers: the same
//! bytes in a 32-bit and a 64-bit process). `CreateDevice` on one of them makes a device of this
//! module's ([`JOYSTICK_CODE`]) with a system DirectInput device behind it. Its calls are passed
//! on: the game's 32-bit `DIDATAFORMAT` read and made again in the 64-bit layout with the same
//! GUIDs, offsets, types and flags, so the system fills the state in the game's layout;
//! `DIDEVCAPS`, the `DIDEVICEOBJECTINSTANCEA`s (no pointers) and the property blocks copied as
//! they are. The cooperative level is checked and kept here, not passed on: the system device
//! stays in DirectInput's default (non-exclusive, background) and this module stands in for the
//! foreground rule: with the default `NV_PAD`, while no window of this process is in front,
//! `Acquire` answers `DIERR_OTHERAPPHASPRIO` and `GetDeviceState` `DIERR_INPUTLOST`, as
//! DirectInput does for a foreground device in the background; a seat (`NV_PAD` set) reads in the
//! background too. In `NV_REPEATABLE` and scripted runs no game controller is listed.
//!
//! An Xbox pad is told apart by the `IG_` mark in its device path (`DIPROP_GUIDANDPATH`), the
//! check Microsoft's "XInput and DirectInput" article gives. Slots a joystick device does not
//! provide stop the run as an open function (as the keyboard's and mouse's do, `dinput`).

use crate::dinput::{DIERR_INVALIDPARAM, DIERR_NOTACQUIRED, DI_OK, E_NOINTERFACE};
use crate::pads::pads;
use crate::{Engine, Ptr};

/// Where this module's joystick device functions sit (code addresses no exe code uses, after
/// `dinput`'s): slot `n` of `IDirectInputDevice8A` at `JOYSTICK_CODE + 4 * n`.
pub const JOYSTICK_CODE: u32 = crate::dinput::CODE_BASE + 0x200;
/// `DIERR_OTHERAPPHASPRIO` (`E_ACCESSDENIED`) and `DIERR_INPUTLOST` (dinput.h).
pub const DIERR_OTHERAPPHASPRIO: u32 = 0x8007_0005;
pub const DIERR_INPUTLOST: u32 = 0x8007_001e;
/// `DIPROP_RANGE` and `DIPROP_DEADZONE` (`MAKEDIPROP(4)`, `MAKEDIPROP(5)`).
pub const DIPROP_RANGE: u32 = 4;
pub const DIPROP_DEADZONE: u32 = 5;
/// The sizes of `DIDEVICEINSTANCEA`, `DIDEVCAPS` and `DIDEVICEOBJECTINSTANCEA`.
pub const INSTANCE_SIZE: usize = 0x244;
pub const CAPS_SIZE: usize = 0x2c;
pub const OBJECT_INSTANCE_SIZE: usize = 0x13c;
/// A 32-bit `DIDATAFORMAT` (0x18 bytes) and `DIOBJECTDATAFORMAT` (0x10).
const FORMAT_SIZE: u32 = 0x18;
const OBJECT_FORMAT_SIZE: u32 = 0x10;
/// The largest state and object count provided (the game's format: 0x50 bytes).
const MAX_DATA_SIZE: u32 = 0x1000;
const MAX_OBJECTS: u32 = 0x100;
/// `SetCooperativeLevel` flags.
const DISCL_EXCLUSIVE: u32 = 1;
const DISCL_NONEXCLUSIVE: u32 = 2;
const DISCL_FOREGROUND: u32 = 4;
const DISCL_BACKGROUND: u32 = 8;
/// `DIENUM_CONTINUE`.
const DIENUM_CONTINUE: u32 = 1;

/// One object of a data format: its type GUID (`None`: any), offset, type and flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFormat {
    pub guid: Option<[u8; 16]>,
    pub offset: u32,
    pub kind: u32,
    pub flags: u32,
}

/// A `DIDATAFORMAT` as the game gave it, without its pointers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFormat {
    pub flags: u32,
    pub data_size: u32,
    pub objects: Vec<ObjectFormat>,
}

/// A system DirectInput device on a game controller (a fake one in tests); the answers are
/// DirectInput's result codes.
pub trait Joystick: Send {
    fn set_data_format(&mut self, format: &DataFormat) -> u32;
    /// `GetCapabilities` into the 0x2c bytes of a `DIDEVCAPS`.
    fn capabilities(&mut self, caps: &mut [u8]) -> u32;
    /// `EnumObjects(flags)`: each object's `DIDEVICEOBJECTINSTANCEA`.
    fn objects(&mut self, flags: u32) -> Result<Vec<Vec<u8>>, u32>;
    /// `SetProperty(MAKEDIPROP(property), block)`, the block as the game gave it.
    fn set_property(&mut self, property: u32, block: &[u8]) -> u32;
    fn acquire(&mut self) -> u32;
    fn unacquire(&mut self) -> u32;
    fn poll(&mut self) -> u32;
    /// `GetDeviceState(data.len(), data)`.
    fn state(&mut self, data: &mut [u8]) -> u32;
}

/// One of the game's joystick devices.
struct Device {
    references: u32,
    system: Box<dyn Joystick>,
    data_size: u32,
    acquired: bool,
}

/// The game's joystick devices (`Pads::joysticks`): the vtable once made, and each device by
/// its address.
#[derive(Default)]
pub struct Devices {
    vtable: u32,
    devices: std::collections::BTreeMap<u32, Device>,
}

impl std::fmt::Debug for Devices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Devices")
            .field("vtable", &self.vtable)
            .field("devices", &self.devices.keys().collect::<Vec<_>>())
            .finish()
    }
}

/// The `DIDEVICEINSTANCEA`s of the game controllers `EnumDevices` lists for the game (after the
/// keyboard and the mouse; `dinput::enum_devices`): none in a repeatable run.
pub fn instances(e: &mut Engine) -> Vec<Vec<u8>> {
    if e.repeatable {
        return Vec::new();
    }
    pads(e)
        .game_joysticks()
        .into_iter()
        .map(|j| j.instance)
        .collect()
}

/// `CreateDevice` for an instance GUID `instances` listed: a new joystick device (reference
/// count 1, no data format, not acquired); `None` for another GUID (`dinput::create_device`).
pub fn create(e: &mut Engine, guid: &[u8]) -> Option<u32> {
    if e.repeatable {
        return None;
    }
    let guid: [u8; 16] = guid.try_into().ok()?;
    let system = pads(e).open_joystick(&guid)?;
    if pads(e).joysticks.vtable == 0 {
        register_code(e);
        let slots = crate::dinput::DEVICE_SLOTS.len() as u32;
        let table = e.mem.alloc(4 * slots);
        for i in 0..slots {
            e.mem.set_u32(table + 4 * i, JOYSTICK_CODE + 4 * i);
        }
        pads(e).joysticks.vtable = table;
    }
    let object = e.mem.alloc(8);
    let table = pads(e).joysticks.vtable;
    e.mem.set_u32(object, table);
    pads(e).joysticks.devices.insert(
        object,
        Device {
            references: 1,
            system,
            data_size: 0,
            acquired: false,
        },
    );
    Some(object)
}

fn register_code(e: &mut Engine) {
    let slots: [(u32, crate::AbiFn); 12] = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (3, crate::entry!(0, get_capabilities(Ptr, Ptr) -> u32).1),
        (
            4,
            crate::entry!(0, enum_objects(Ptr, u32, u32, u32) -> u32).1,
        ),
        (6, crate::entry!(0, set_property(Ptr, u32, Ptr) -> u32).1),
        (7, crate::entry!(0, acquire(Ptr) -> u32).1),
        (8, crate::entry!(0, unacquire(Ptr) -> u32).1),
        (
            9,
            crate::entry!(0, get_device_state(Ptr, u32, Ptr) -> u32).1,
        ),
        (11, crate::entry!(0, set_data_format(Ptr, Ptr) -> u32).1),
        (
            13,
            crate::entry!(0, set_cooperative_level(Ptr, u32, u32) -> u32).1,
        ),
        (25, crate::entry!(0, poll(Ptr) -> u32).1),
    ];
    for (slot, f) in slots {
        e.register(JOYSTICK_CODE + 4 * slot, f);
    }
}

fn device(e: &mut Engine, this: Ptr) -> &mut Device {
    pads(e)
        .joysticks
        .devices
        .get_mut(&this.addr())
        .expect("a call on a joystick device this layer did not make")
}

/// `QueryInterface`: no other interface is offered (`E_NOINTERFACE`, `*out` = 0).
fn query_interface(e: &mut Engine, _this: Ptr, _interface: Ptr, out: Ptr) -> u32 {
    if !out.is_null() {
        e.mem.set_u32(out.addr(), 0);
    }
    E_NOINTERFACE
}

fn add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references += 1;
    d.references
}

/// `Release`: at 0 the device goes, and the system device with it.
fn release(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.references -= 1;
    let left = d.references;
    if left == 0 {
        pads(e).joysticks.devices.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `GetCapabilities(caps)`: `caps->dwSize` must be 0x2c (else `DIERR_INVALIDPARAM`); the system
/// device's `DIDEVCAPS`, written on success.
fn get_capabilities(e: &mut Engine, this: Ptr, caps: Ptr) -> u32 {
    if e.mem.u32(caps.addr()) != CAPS_SIZE as u32 {
        return DIERR_INVALIDPARAM;
    }
    let mut bytes = vec![0u8; CAPS_SIZE];
    bytes[0..4].copy_from_slice(&(CAPS_SIZE as u32).to_le_bytes());
    let answer = device(e, this).system.capabilities(&mut bytes);
    if answer == DI_OK {
        e.mem.write(caps.addr(), &bytes);
    }
    answer
}

/// `EnumObjects(callback, reference, flags)`: each of the system device's objects (those of
/// `flags`) through `callback(object, reference)` (`stdcall`) until it answers other than
/// `DIENUM_CONTINUE`; `DI_OK`, or the system's error.
fn enum_objects(e: &mut Engine, this: Ptr, callback: u32, reference: u32, flags: u32) -> u32 {
    let objects = match device(e, this).system.objects(flags) {
        Ok(objects) => objects,
        Err(error) => return error,
    };
    for object in objects {
        let go_on = e.with_stack(object.len() as u32, |e, record| {
            e.mem.write(record.addr(), &object);
            e.call(callback, &[record.addr(), reference]).u32()
        });
        if go_on != DIENUM_CONTINUE {
            break;
        }
    }
    DI_OK
}

/// `SetProperty(property, header)`: `DIPROP_RANGE` and `DIPROP_DEADZONE` (the game's two), the
/// block (`header->dwSize` bytes, 0x10 to 0x40) passed to the system device; other properties
/// are not provided.
fn set_property(e: &mut Engine, this: Ptr, property: u32, header: Ptr) -> u32 {
    if property != DIPROP_RANGE && property != DIPROP_DEADZONE {
        panic!("IDirectInputDevice8A::SetProperty({property:#x}) on a joystick is not provided");
    }
    let size = e.mem.u32(header.addr());
    if !(0x10..=0x40).contains(&size) {
        return DIERR_INVALIDPARAM;
    }
    let block = e.mem.bytes(header.addr(), size);
    device(e, this).system.set_property(property, &block)
}

/// `Acquire()`: needs a data format (`DIERR_INVALIDPARAM`); `DIERR_OTHERAPPHASPRIO` while the
/// controllers may not be read (the default `NV_PAD` with no window of this process in front);
/// else the system device's answer.
fn acquire(e: &mut Engine, this: Ptr) -> u32 {
    if device(e, this).data_size == 0 {
        return DIERR_INVALIDPARAM;
    }
    if !pads(e).readable() {
        let d = device(e, this);
        if std::mem::take(&mut d.acquired) {
            d.system.unacquire();
        }
        return DIERR_OTHERAPPHASPRIO;
    }
    let d = device(e, this);
    let answer = d.system.acquire();
    if answer & 0x8000_0000 == 0 {
        d.acquired = true;
    }
    answer
}

/// `Unacquire()`: the system device's answer.
fn unacquire(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.acquired = false;
    d.system.unacquire()
}

/// `Poll()`: `DIERR_NOTACQUIRED` when not acquired, else the system device's answer.
fn poll(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    if !d.acquired {
        return DIERR_NOTACQUIRED;
    }
    d.system.poll()
}

/// `GetDeviceState(size, data)`: acquired (`DIERR_NOTACQUIRED`), `size` the format's
/// (`DIERR_INVALIDPARAM`), the controllers readable now (else unacquired and
/// `DIERR_INPUTLOST`); the system device's state, written on success.
fn get_device_state(e: &mut Engine, this: Ptr, size: u32, data: Ptr) -> u32 {
    let d = device(e, this);
    if !d.acquired {
        return DIERR_NOTACQUIRED;
    }
    if size != d.data_size {
        return DIERR_INVALIDPARAM;
    }
    if !pads(e).readable() {
        let d = device(e, this);
        d.acquired = false;
        d.system.unacquire();
        return DIERR_INPUTLOST;
    }
    let mut bytes = vec![0u8; size as usize];
    let answer = device(e, this).system.state(&mut bytes);
    if answer & 0x8000_0000 == 0 {
        e.mem.write(data.addr(), &bytes);
    }
    answer
}

/// `SetDataFormat(format)`: the game's 32-bit `DIDATAFORMAT` (`dwSize` 0x18 and `dwObjSize` 0x10,
/// else `DIERR_INVALIDPARAM`) read with its objects (each type GUID read where its pointer is not
/// null) and passed to the system device; kept on success. Not while acquired is the system's
/// rule (`DIERR_ACQUIRED`).
fn set_data_format(e: &mut Engine, this: Ptr, format: Ptr) -> u32 {
    let at = format.addr();
    if e.mem.u32(at) != FORMAT_SIZE || e.mem.u32(at + 4) != OBJECT_FORMAT_SIZE {
        return DIERR_INVALIDPARAM;
    }
    let (flags, data_size, count, objects) = (
        e.mem.u32(at + 8),
        e.mem.u32(at + 0xc),
        e.mem.u32(at + 0x10),
        e.mem.u32(at + 0x14),
    );
    assert!(
        data_size <= MAX_DATA_SIZE && count <= MAX_OBJECTS,
        "IDirectInputDevice8A::SetDataFormat on a joystick: {data_size:#x} bytes, {count} objects \
         is not provided"
    );
    let objects = (0..count)
        .map(|i| {
            let o = objects + OBJECT_FORMAT_SIZE * i;
            let guid = e.mem.u32(o);
            ObjectFormat {
                guid: (guid != 0).then(|| {
                    let mut g = [0u8; 16];
                    g.copy_from_slice(&e.mem.bytes(guid, 16));
                    g
                }),
                offset: e.mem.u32(o + 4),
                kind: e.mem.u32(o + 8),
                flags: e.mem.u32(o + 0xc),
            }
        })
        .collect();
    let format = DataFormat {
        flags,
        data_size,
        objects,
    };
    let d = device(e, this);
    let answer = d.system.set_data_format(&format);
    if answer & 0x8000_0000 == 0 {
        d.data_size = data_size;
    }
    answer
}

/// `SetCooperativeLevel(window, flags)`: exactly one of exclusive / non-exclusive and one of
/// foreground / background (else `DIERR_INVALIDPARAM`); `DI_OK`. Not passed on (see the
/// module's text).
fn set_cooperative_level(e: &mut Engine, this: Ptr, _window: u32, flags: u32) -> u32 {
    let _ = device(e, this);
    let one_of = |a: u32, b: u32| (flags & a != 0) != (flags & b != 0);
    if !one_of(DISCL_EXCLUSIVE, DISCL_NONEXCLUSIVE) || !one_of(DISCL_FOREGROUND, DISCL_BACKGROUND) {
        return DIERR_INVALIDPARAM;
    }
    DI_OK
}

/// The system's controllers for [`crate::pads`]: its XInput and DirectInput, and the XInput
/// DLL's name when one loaded; `None` off Windows.
pub fn system() -> (
    Option<Box<dyn crate::pads::Controllers>>,
    Option<&'static str>,
) {
    system::load()
}

/// The system's XInput and DirectInput (Windows).
#[cfg(windows)]
mod system {
    use super::{DataFormat, Joystick, CAPS_SIZE, INSTANCE_SIZE, OBJECT_INSTANCE_SIZE};
    use crate::pads::{
        Controllers, JoystickInfo, CAPABILITIES_SIZE, ERROR_DEVICE_NOT_CONNECTED, STATE_SIZE,
        VIBRATION_SIZE,
    };
    use std::ffi::c_void;
    use windows::core::{s, w, Interface, GUID, PCWSTR};
    use windows::Win32::Devices::HumanInterfaceDevice::{
        DirectInput8Create, IDirectInput8A, IDirectInputDevice8A, DI8DEVCLASS_GAMECTRL,
        DIDATAFORMAT, DIDEVCAPS, DIDEVICEINSTANCEA, DIDEVICEOBJECTINSTANCEA, DIEDFL_ATTACHEDONLY,
        DIOBJECTDATAFORMAT, DIPROPGUIDANDPATH, DIPROPHEADER,
    };
    use windows::Win32::Foundation::{BOOL, HINSTANCE};
    use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

    type GetState = unsafe extern "system" fn(u32, *mut u8) -> u32;
    type SetState = unsafe extern "system" fn(u32, *mut u8) -> u32;
    type GetCapabilities = unsafe extern "system" fn(u32, u32, *mut u8) -> u32;
    /// What `GetProcAddress` answers.
    type Proc = unsafe extern "system" fn() -> isize;
    /// `DIRECTINPUT_VERSION` and `DIPROP_GUIDANDPATH` (`MAKEDIPROP(12)`).
    const DIRECTINPUT_VERSION: u32 = 0x0800;
    const DIPROP_GUIDANDPATH: usize = 12;

    /// A structure's bytes at a 4-byte alignment (its widest field's, or more).
    #[repr(C, align(4))]
    struct Aligned<const N: usize>([u8; N]);

    /// The XInput DLL's three functions.
    #[derive(Clone, Copy)]
    struct XInput {
        get_state: GetState,
        set_state: SetState,
        get_capabilities: Option<GetCapabilities>,
    }

    /// The first of the DLLs that loads with `XInputGetState` and `XInputSetState`, and its
    /// name, once for the process.
    fn xinput() -> Option<(XInput, &'static str)> {
        static XINPUT: std::sync::OnceLock<Option<(XInput, &'static str)>> =
            std::sync::OnceLock::new();
        *XINPUT.get_or_init(|| {
            let dlls: [(PCWSTR, &'static str); 3] = [
                (w!("xinput1_4.dll"), "xinput1_4.dll"),
                (w!("xinput1_3.dll"), "xinput1_3.dll"),
                (w!("xinput9_1_0.dll"), "xinput9_1_0.dll"),
            ];
            dlls.into_iter().find_map(|(wide, name)| {
                // SAFETY: Microsoft's system DLLs, found by Windows' search order for system
                // DLLs; the functions are cast to their documented signatures (xinput.h).
                unsafe {
                    let module = LoadLibraryW(wide).ok()?;
                    let get_state = GetProcAddress(module, s!("XInputGetState"))?;
                    let set_state = GetProcAddress(module, s!("XInputSetState"))?;
                    let get_capabilities = GetProcAddress(module, s!("XInputGetCapabilities"));
                    Some((
                        XInput {
                            get_state: std::mem::transmute::<Proc, GetState>(get_state),
                            set_state: std::mem::transmute::<Proc, SetState>(set_state),
                            get_capabilities: get_capabilities
                                .map(|f| std::mem::transmute::<Proc, GetCapabilities>(f)),
                        },
                        name,
                    ))
                }
            })
        })
    }

    /// The system's controllers: its XInput (when a DLL loaded) and its DirectInput (made on
    /// first use).
    struct System {
        xinput: Option<XInput>,
        direct_input: Option<Option<IDirectInput8A>>,
    }

    // SAFETY: DirectInput's objects are not apartment objects (`DirectInput8Create` is not
    // `CoCreateInstance`) and the platform calls them one at a time, under the kernel lock.
    unsafe impl Send for System {}

    pub fn load() -> (Option<Box<dyn Controllers>>, Option<&'static str>) {
        let found = xinput();
        let system = System {
            xinput: found.map(|(x, _)| x),
            direct_input: None,
        };
        (Some(Box::new(system)), found.map(|(_, name)| name))
    }

    /// DirectInput's answer as its result code.
    fn code(result: windows::core::Result<()>) -> u32 {
        match result {
            Ok(()) => 0,
            Err(error) => error.code().0 as u32,
        }
    }

    /// The GUID of its 16 bytes (as they lie in memory).
    fn guid(bytes: &[u8; 16]) -> GUID {
        let word = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
        let mut tail = [0u8; 8];
        tail.copy_from_slice(&bytes[8..16]);
        GUID::from_values(
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            word(4),
            word(6),
            tail,
        )
    }

    unsafe extern "system" fn collect_device(
        instance: *mut DIDEVICEINSTANCEA,
        list: *mut c_void,
    ) -> BOOL {
        let list = &mut *(list as *mut Vec<Vec<u8>>);
        list.push(std::slice::from_raw_parts(instance as *const u8, INSTANCE_SIZE).to_vec());
        BOOL(1)
    }

    unsafe extern "system" fn collect_object(
        object: *mut DIDEVICEOBJECTINSTANCEA,
        list: *mut c_void,
    ) -> BOOL {
        let list = &mut *(list as *mut Vec<Vec<u8>>);
        list.push(std::slice::from_raw_parts(object as *const u8, OBJECT_INSTANCE_SIZE).to_vec());
        BOOL(1)
    }

    impl System {
        fn direct_input(&mut self) -> Option<&IDirectInput8A> {
            self.direct_input
                .get_or_insert_with(|| {
                    assert_eq!(std::mem::size_of::<DIDEVICEINSTANCEA>(), INSTANCE_SIZE);
                    assert_eq!(
                        std::mem::size_of::<DIDEVICEOBJECTINSTANCEA>(),
                        OBJECT_INSTANCE_SIZE
                    );
                    // SAFETY: the documented call; the object comes back counted.
                    unsafe {
                        let instance = HINSTANCE(GetModuleHandleW(None).ok()?.0);
                        let mut out: *mut c_void = std::ptr::null_mut();
                        DirectInput8Create(
                            instance,
                            DIRECTINPUT_VERSION,
                            &IDirectInput8A::IID,
                            &mut out,
                            None,
                        )
                        .ok()?;
                        Some(IDirectInput8A::from_raw(out))
                    }
                })
                .as_ref()
        }

        fn open(&mut self, id: &[u8; 16]) -> Option<IDirectInputDevice8A> {
            let direct_input = self.direct_input()?;
            let mut device = None;
            // SAFETY: the documented call with a GUID it listed.
            unsafe { direct_input.CreateDevice(&guid(id), &mut device, None) }.ok()?;
            device
        }

        /// Whether the device's path has the `IG_` mark of an XInput controller.
        fn is_xinput(device: &IDirectInputDevice8A) -> bool {
            let mut property = DIPROPGUIDANDPATH {
                diph: DIPROPHEADER {
                    dwSize: std::mem::size_of::<DIPROPGUIDANDPATH>() as u32,
                    dwHeaderSize: std::mem::size_of::<DIPROPHEADER>() as u32,
                    dwObj: 0,
                    dwHow: 0,
                },
                ..Default::default()
            };
            // SAFETY: the property's documented block; MAKEDIPROP(12) is its id.
            let read = unsafe {
                device.GetProperty(DIPROP_GUIDANDPATH as *const GUID, &mut property.diph)
            };
            if read.is_err() {
                return false;
            }
            let end = property.wszPath.iter().position(|&c| c == 0).unwrap_or(260);
            String::from_utf16_lossy(&property.wszPath[..end])
                .to_ascii_uppercase()
                .contains("IG_")
        }
    }

    impl Controllers for System {
        fn get_state(&mut self, pad: u32, state: &mut [u8; STATE_SIZE]) -> u32 {
            let Some(x) = self.xinput else {
                return ERROR_DEVICE_NOT_CONNECTED;
            };
            let mut out = Aligned([0; STATE_SIZE]);
            // SAFETY: `out` is an `XINPUT_STATE`'s size and alignment.
            let answer = unsafe { (x.get_state)(pad, out.0.as_mut_ptr()) };
            *state = out.0;
            answer
        }

        fn set_state(&mut self, pad: u32, vibration: &[u8; VIBRATION_SIZE]) -> u32 {
            let Some(x) = self.xinput else {
                return ERROR_DEVICE_NOT_CONNECTED;
            };
            let mut v = Aligned(*vibration);
            // SAFETY: `v` is an `XINPUT_VIBRATION`'s size and alignment.
            unsafe { (x.set_state)(pad, v.0.as_mut_ptr()) }
        }

        fn get_capabilities(
            &mut self,
            pad: u32,
            flags: u32,
            capabilities: &mut [u8; CAPABILITIES_SIZE],
        ) -> u32 {
            let Some(f) = self.xinput.and_then(|x| x.get_capabilities) else {
                return ERROR_DEVICE_NOT_CONNECTED;
            };
            let mut out = Aligned([0; CAPABILITIES_SIZE]);
            // SAFETY: `out` is an `XINPUT_CAPABILITIES`'s size and alignment.
            let answer = unsafe { f(pad, flags, out.0.as_mut_ptr()) };
            *capabilities = out.0;
            answer
        }

        fn joysticks(&mut self) -> Vec<JoystickInfo> {
            let Some(direct_input) = self.direct_input() else {
                return Vec::new();
            };
            let mut list: Vec<Vec<u8>> = Vec::new();
            // SAFETY: the documented call; the callback only copies each record.
            let _ = unsafe {
                direct_input.EnumDevices(
                    DI8DEVCLASS_GAMECTRL,
                    Some(collect_device),
                    &mut list as *mut _ as *mut c_void,
                    DIEDFL_ATTACHEDONLY,
                )
            };
            list.into_iter()
                .map(|instance| {
                    let mut id = [0u8; 16];
                    id.copy_from_slice(&instance[4..0x14]);
                    let xinput = self.open(&id).is_some_and(|d| Self::is_xinput(&d));
                    JoystickInfo { instance, xinput }
                })
                .collect()
        }

        fn open_joystick(&mut self, id: &[u8; 16]) -> Option<Box<dyn Joystick>> {
            let device = self.open(id)?;
            Some(Box::new(Stick { device }))
        }

        fn in_front(&mut self) -> bool {
            // SAFETY: plain queries of the foreground window and its process.
            unsafe {
                let window = GetForegroundWindow();
                if window.0.is_null() {
                    return false;
                }
                let mut process = 0u32;
                GetWindowThreadProcessId(window, Some(&mut process));
                process == std::process::id()
            }
        }
    }

    /// A system DirectInput device on a game controller.
    struct Stick {
        device: IDirectInputDevice8A,
    }

    // SAFETY: as for [`System`].
    unsafe impl Send for Stick {}

    impl Drop for Stick {
        fn drop(&mut self) {
            // SAFETY: the documented call; the device is released after it.
            let _ = unsafe { self.device.Unacquire() };
        }
    }

    impl Joystick for Stick {
        fn set_data_format(&mut self, format: &DataFormat) -> u32 {
            let guids: Vec<GUID> = format
                .objects
                .iter()
                .map(|o| o.guid.as_ref().map(guid).unwrap_or_default())
                .collect();
            let mut objects: Vec<DIOBJECTDATAFORMAT> = format
                .objects
                .iter()
                .zip(&guids)
                .map(|(o, g)| DIOBJECTDATAFORMAT {
                    pguid: if o.guid.is_some() {
                        g as *const GUID
                    } else {
                        std::ptr::null()
                    },
                    dwOfs: o.offset,
                    dwType: o.kind,
                    dwFlags: o.flags,
                })
                .collect();
            let mut f = DIDATAFORMAT {
                dwSize: std::mem::size_of::<DIDATAFORMAT>() as u32,
                dwObjSize: std::mem::size_of::<DIOBJECTDATAFORMAT>() as u32,
                dwFlags: format.flags,
                dwDataSize: format.data_size,
                dwNumObjs: objects.len() as u32,
                rgodf: objects.as_mut_ptr(),
            };
            // SAFETY: the format and its objects and GUIDs live across the call (DirectInput
            // copies them).
            code(unsafe { self.device.SetDataFormat(&mut f) })
        }

        fn capabilities(&mut self, caps: &mut [u8]) -> u32 {
            let mut c = DIDEVCAPS {
                dwSize: CAPS_SIZE as u32,
                ..Default::default()
            };
            // SAFETY: the documented call on a `DIDEVCAPS`.
            let answer = code(unsafe { self.device.GetCapabilities(&mut c) });
            // SAFETY: `DIDEVCAPS` is 0x2c bytes of plain words.
            let bytes =
                unsafe { std::slice::from_raw_parts(&c as *const _ as *const u8, CAPS_SIZE) };
            caps.copy_from_slice(bytes);
            answer
        }

        fn objects(&mut self, flags: u32) -> Result<Vec<Vec<u8>>, u32> {
            let mut list: Vec<Vec<u8>> = Vec::new();
            // SAFETY: the documented call; the callback only copies each record.
            let answer = code(unsafe {
                self.device.EnumObjects(
                    Some(collect_object),
                    &mut list as *mut _ as *mut c_void,
                    flags,
                )
            });
            if answer & 0x8000_0000 != 0 {
                return Err(answer);
            }
            Ok(list)
        }

        fn set_property(&mut self, property: u32, block: &[u8]) -> u32 {
            let mut words = vec![0u32; block.len().div_ceil(4)];
            // SAFETY: `words` holds the block's bytes, 4-byte aligned.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    block.as_ptr(),
                    words.as_mut_ptr() as *mut u8,
                    block.len(),
                );
                code(self.device.SetProperty(
                    property as usize as *const GUID,
                    words.as_mut_ptr() as *mut DIPROPHEADER,
                ))
            }
        }

        fn acquire(&mut self) -> u32 {
            // SAFETY: the documented call.
            code(unsafe { self.device.Acquire() })
        }

        fn unacquire(&mut self) -> u32 {
            // SAFETY: the documented call.
            code(unsafe { self.device.Unacquire() })
        }

        fn poll(&mut self) -> u32 {
            // SAFETY: the documented call.
            code(unsafe { self.device.Poll() })
        }

        fn state(&mut self, data: &mut [u8]) -> u32 {
            let mut words = vec![0u32; data.len().div_ceil(4)];
            // SAFETY: `words` is at least the data format's size, 4-byte aligned.
            let answer = code(unsafe {
                self.device
                    .GetDeviceState(data.len() as u32, words.as_mut_ptr() as *mut c_void)
            });
            // SAFETY: `words` holds at least `data.len()` bytes.
            let bytes =
                unsafe { std::slice::from_raw_parts(words.as_ptr() as *const u8, data.len()) };
            data.copy_from_slice(bytes);
            answer
        }
    }
}

/// No XInput or DirectInput off Windows.
#[cfg(not(windows))]
mod system {
    pub fn load() -> (
        Option<Box<dyn crate::pads::Controllers>>,
        Option<&'static str>,
    ) {
        (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pads::fake::{instance, Fake, Stick};
    use crate::pads::{use_controllers, JoystickInfo, Seat};

    /// The game's joystick data format at `at`: `c_dfDIJoystick`'s shape (0x50 bytes), two
    /// objects, the first with a type GUID.
    fn game_format(e: &mut Engine, at: u32) -> u32 {
        let guid = at + 0x40;
        e.mem.write(guid, &[0x5a; 16]);
        let objects = at + 0x20;
        for (i, words) in [[guid, 0, 0x0080_0003, 0x100], [0, 0x30, 0x0080_000c, 0]]
            .iter()
            .enumerate()
        {
            for (k, w) in words.iter().enumerate() {
                e.mem.set_u32(objects + 0x10 * i as u32 + 4 * k as u32, *w);
            }
        }
        for (k, w) in [0x18u32, 0x10, 1, 0x50, 2, objects].iter().enumerate() {
            e.mem.set_u32(at + 4 * k as u32, *w);
        }
        at
    }

    /// A machine with one DirectInput game controller (a 0x50-byte state, capabilities 1..0x2c,
    /// two objects) and an engine with its DirectInput open.
    fn setup(seat: Seat) -> (Fake, Engine, u32) {
        let fake = Fake::default();
        {
            let mut m = fake.machine();
            m.in_front = true;
            m.sticks = vec![
                Stick {
                    info: Some(JoystickInfo {
                        instance: instance(0x44, 0x0215, "Stick"),
                        xinput: true,
                    }),
                    ..Stick::default()
                },
                Stick {
                    info: Some(JoystickInfo {
                        instance: instance(0x55, 0x0115, "Pad"),
                        xinput: false,
                    }),
                    capabilities: (1..=0x2c).collect(),
                    objects: vec![vec![1; OBJECT_INSTANCE_SIZE], vec![2; OBJECT_INSTANCE_SIZE]],
                    state: (0..0x50).collect(),
                    ..Stick::default()
                },
            ];
        }
        let mut e = Engine::new();
        use_controllers(&mut e, seat, Box::new(fake.clone()));
        e.mem.map(0x1000, 0x2000);
        e.mem.write(
            0x1000,
            &[
                0x30, 0x80, 0x79, 0xbf, 0x3a, 0x48, 0xa2, 0x4d, 0xaa, 0x99, 0x5d, 0x64, 0xed, 0x36,
                0x97, 0x00,
            ],
        );
        let r = crate::dinput::direct_input8_create(
            &mut e,
            0,
            0x0800,
            Ptr::new(0x1000),
            Ptr::new(0x1100),
            0,
        );
        assert_eq!(r, DI_OK);
        let di = e.mem.u32(0x1100);
        (fake, e, di)
    }

    /// A stdcall callback at `code` that copies each record's word at `word_at` (the type,
    /// `+0x24`, or the first) and the reference into a list at 0x1900, answering `answer` (both
    /// kept at 0x18f8, as the callback captures nothing).
    fn recorder(e: &mut Engine, code: u32, word_at: u32, answer: u32) {
        e.mem.set_u32(0x18f8, word_at);
        e.mem.set_u32(0x18fc, answer);
        e.register(code, |e, a| {
            let n = e.mem.u32(0x1900);
            let word_at = e.mem.u32(0x18f8);
            e.mem.set_u32(0x1904 + 8 * n, e.mem.u32(a[0] + word_at));
            e.mem.set_u32(0x1908 + 8 * n, a[1]);
            e.mem.set_u32(0x1900, n + 1);
            crate::abi::RetVal::into_ret(e.mem.u32(0x18fc))
        });
    }

    /// `EnumDevices` over all classes: the mouse, the keyboard, then the PC's game controller
    /// that is not an XInput one (the Xbox pad's DirectInput side left out); the game-controller
    /// class gives it alone.
    #[test]
    fn enum_devices_lists_the_pcs_game_controllers_after_the_system_devices() {
        let (_fake, mut e, di) = setup(Seat::Any { background: false });
        recorder(&mut e, 0x00ab_0000, 0x24, 1);
        assert_eq!(e.vcall(di, 0x10, &[0, 0x00ab_0000, 0x77, 1]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1900), 3);
        assert_eq!(
            [0x1904, 0x190c, 0x1914, 0x1918].map(|a| e.mem.u32(a)),
            [0x0112, 0x0413, 0x0115, 0x77]
        );
        e.mem.set_u32(0x1900, 0);
        assert_eq!(e.vcall(di, 0x10, &[4, 0x00ab_0000, 0x77, 1]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1900), 1);
        // The whole record, as the system gave it.
        e.register(0x00ab_0100, |e, a| {
            e.mem
                .write(0x1a00, &e.mem.bytes(a[0], INSTANCE_SIZE as u32));
            crate::abi::RetVal::into_ret(0u32)
        });
        e.vcall(di, 0x10, &[4, 0x00ab_0100, 0, 1]);
        assert_eq!(
            e.mem.bytes(0x1a00, INSTANCE_SIZE as u32),
            instance(0x55, 0x0115, "Pad")
        );
    }

    /// The game's whole use of a joystick, as `00a24830` and `Controls::Poll` make it: create
    /// by the listed GUID, cooperative level, data format, capabilities, objects, range and dead
    /// zone, then acquire, poll and read; release.
    #[test]
    fn the_game_reads_a_joystick_through_the_system_device() {
        let (fake, mut e, di) = setup(Seat::Any { background: false });
        e.mem.write(0x1010, &[0x55; 16]);
        assert_eq!(e.vcall(di, 0xc, &[0x1010, 0x1104, 0]).u32(), DI_OK);
        let j = e.mem.u32(0x1104);
        assert_ne!(j, 0);
        assert_eq!(fake.machine().sticks[1].open, 1);
        // Not acquired without a format.
        assert_eq!(e.vcall(j, 0x1c, &[]).u32(), DIERR_INVALIDPARAM);
        assert_eq!(e.vcall(j, 0x34, &[0x1234, 6]).u32(), DI_OK);
        assert_eq!(e.vcall(j, 0x34, &[0x1234, 3]).u32(), DIERR_INVALIDPARAM);
        let f = game_format(&mut e, 0x1200);
        assert_eq!(e.vcall(j, 0x2c, &[f]).u32(), DI_OK);
        assert_eq!(
            fake.machine().sticks[1].format,
            Some(DataFormat {
                flags: 1,
                data_size: 0x50,
                objects: vec![
                    ObjectFormat {
                        guid: Some([0x5a; 16]),
                        offset: 0,
                        kind: 0x0080_0003,
                        flags: 0x100
                    },
                    ObjectFormat {
                        guid: None,
                        offset: 0x30,
                        kind: 0x0080_000c,
                        flags: 0
                    },
                ]
            })
        );
        // Capabilities: the size checked, the system's 0x2c bytes.
        e.mem.set_u32(0x1300, 0x14);
        assert_eq!(e.vcall(j, 0xc, &[0x1300]).u32(), DIERR_INVALIDPARAM);
        e.mem.set_u32(0x1300, 0x2c);
        assert_eq!(e.vcall(j, 0xc, &[0x1300]).u32(), DI_OK);
        assert_eq!(e.mem.bytes(0x1300, 0x2c), (1..=0x2c).collect::<Vec<u8>>());
        // Objects: each record to the callback; a stop ends the list.
        recorder(&mut e, 0x00ab_0200, 0, 1);
        assert_eq!(e.vcall(j, 0x10, &[0x00ab_0200, 0x99, 0]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1900), 2);
        assert_eq!(
            [0x1904, 0x1908, 0x190c].map(|a| e.mem.u32(a)),
            [0x0101_0101, 0x99, 0x0202_0202]
        );
        e.mem.set_u32(0x1900, 0);
        recorder(&mut e, 0x00ab_0300, 0, 0);
        e.vcall(j, 0x10, &[0x00ab_0300, 0x99, 0]);
        assert_eq!(e.mem.u32(0x1900), 1);
        // Range and dead zone, as the game's blocks.
        let range = [0x18u32, 0x10, 0, 0, (-100i32) as u32, 100];
        for (k, w) in range.iter().enumerate() {
            e.mem.set_u32(0x1400 + 4 * k as u32, *w);
        }
        assert_eq!(e.vcall(j, 0x18, &[4, 0x1400]).u32(), DI_OK);
        for (k, w) in [0x14u32, 0x10, 0, 0, 2500].iter().enumerate() {
            e.mem.set_u32(0x1500 + 4 * k as u32, *w);
        }
        assert_eq!(e.vcall(j, 0x18, &[5, 0x1500]).u32(), DI_OK);
        let props = fake.machine().sticks[1].properties.clone();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].0, 4);
        assert_eq!(props[0].1, e.mem.bytes(0x1400, 0x18));
        assert_eq!((props[1].0, props[1].1.len()), (5, 0x14));
        // Poll's sequence: acquire, poll, read the 0x50 bytes.
        assert_eq!(e.vcall(j, 0x64, &[]).u32(), DIERR_NOTACQUIRED);
        assert_eq!(e.vcall(j, 0x24, &[0x50, 0x1600]).u32(), DIERR_NOTACQUIRED);
        assert_eq!(e.vcall(j, 0x1c, &[]).u32(), DI_OK);
        assert_eq!(e.vcall(j, 0x1c, &[]).u32(), 1, "already acquired");
        assert_eq!(e.vcall(j, 0x64, &[]).u32(), DI_OK);
        assert_eq!(e.vcall(j, 0x24, &[0x14, 0x1600]).u32(), DIERR_INVALIDPARAM);
        assert_eq!(e.vcall(j, 0x24, &[0x50, 0x1600]).u32(), DI_OK);
        assert_eq!(e.mem.bytes(0x1600, 0x50), (0..0x50).collect::<Vec<u8>>());
        assert_eq!(fake.machine().sticks[1].polls, 1);
        // Stop: unacquire and release; the system device goes with the last reference.
        assert_eq!(e.vcall(j, 0x20, &[]).u32(), DI_OK);
        assert!(!fake.machine().sticks[1].acquired);
        assert_eq!(e.vcall(j, 4, &[]).u32(), 2);
        assert_eq!(e.vcall(j, 8, &[]).u32(), 1);
        assert_eq!(e.vcall(j, 8, &[]).u32(), 0);
        assert_eq!(fake.machine().sticks[1].open, 0);
    }

    /// The Xbox pad's DirectInput side, or a GUID never listed, is not a device the game gets.
    #[test]
    fn an_xbox_pads_directinput_side_is_not_created() {
        let (fake, mut e, di) = setup(Seat::Any { background: true });
        for tag in [0x44u8, 0x66] {
            e.mem.write(0x1010, &[tag; 16]);
            assert_eq!(
                e.vcall(di, 0xc, &[0x1010, 0x1104, 0]).u32(),
                crate::dinput::DIERR_DEVICENOTREG
            );
        }
        assert_eq!(fake.machine().sticks[0].open, 0);
    }

    /// With the default `NV_PAD`, a joystick is read only while this process's window is in
    /// front: in the back the acquire fails as DirectInput's does for a foreground device (and
    /// the game clears its state); a seat reads on.
    #[test]
    fn the_default_reads_a_joystick_only_in_front() {
        for (seat, in_back) in [
            (Seat::Any { background: false }, DIERR_OTHERAPPHASPRIO),
            (Seat::Pad(0), DI_OK),
        ] {
            let (fake, mut e, di) = setup(seat);
            e.mem.write(0x1010, &[0x55; 16]);
            assert_eq!(e.vcall(di, 0xc, &[0x1010, 0x1104, 0]).u32(), DI_OK);
            let j = e.mem.u32(0x1104);
            let f = game_format(&mut e, 0x1200);
            assert_eq!(e.vcall(j, 0x2c, &[f]).u32(), DI_OK);
            assert_eq!(e.vcall(j, 0x1c, &[]).u32(), DI_OK);
            fake.machine().in_front = false;
            let read = e.vcall(j, 0x24, &[0x50, 0x1600]).u32();
            assert_eq!(read == DI_OK, in_back == DI_OK);
            if in_back != DI_OK {
                assert_eq!(read, DIERR_INPUTLOST);
                assert!(!fake.machine().sticks[1].acquired);
            }
            assert_eq!(
                e.vcall(j, 0x1c, &[]).u32() & 0x8000_0000,
                in_back & 0x8000_0000
            );
            assert_eq!(
                e.vcall(j, 0x1c, &[]).u32() == DIERR_OTHERAPPHASPRIO,
                in_back != DI_OK
            );
        }
    }

    /// `NV_REPEATABLE` and a run without controllers list no game controller and create none.
    #[test]
    fn a_repeatable_run_lists_no_game_controller() {
        let (fake, mut e, di) = setup(Seat::Any { background: true });
        e.start_repeatable_clock();
        recorder(&mut e, 0x00ab_0000, 0x24, 1);
        assert_eq!(e.vcall(di, 0x10, &[4, 0x00ab_0000, 0, 1]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1900), 0);
        e.mem.write(0x1010, &[0x55; 16]);
        assert_eq!(
            e.vcall(di, 0xc, &[0x1010, 0x1104, 0]).u32(),
            crate::dinput::DIERR_DEVICENOTREG
        );
        assert_eq!(fake.machine().listings, 0);
        let (_, mut e, di) = setup(Seat::None);
        recorder(&mut e, 0x00ab_0000, 0x24, 1);
        assert_eq!(e.vcall(di, 0x10, &[4, 0x00ab_0000, 0, 1]).u32(), DI_OK);
        assert_eq!(e.mem.u32(0x1900), 0);
    }
}
