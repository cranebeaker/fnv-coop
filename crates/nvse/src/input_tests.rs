//! Tests of xNVSE's DirectInput hook (I-11) in front of a stand-in of the platform's DirectInput:
//! an `IDirectInput8A` and devices whose keys, mouse state, buffered events and answers sit in
//! guest memory where the tests set them. Each against `Hooks_DirectInput8Create.cpp` (xNVSE).

use super::*;

const AREA: u32 = 0x0200_0000;
/// The stand-in's code: its `IDirectInput8A` slots and its devices' slots.
const REAL_DI_CODE: u32 = 0x0060_0000;
const REAL_DEVICE_CODE: u32 = 0x0060_1000;
/// Its objects and vtables: the interface, the devices made one after another.
const REAL_DI: u32 = AREA + 0x100;
const REAL_DI_VTABLE: u32 = AREA + 0x200;
const REAL_DEVICE_VTABLE: u32 = AREA + 0x300;
const NEXT_DEVICE: u32 = AREA + 0x4fc;
const DEVICES: u32 = AREA + 0x500;
/// The GUIDs the game passes, the interface id and the game's out words.
const GUID_KEYBOARD: u32 = AREA + 0x10;
const GUID_MOUSE: u32 = AREA + 0x20;
const GUID_JOYSTICK: u32 = AREA + 0x30;
const IID: u32 = AREA + 0x40;
const OUT: u32 = AREA + 0x50;
/// What the stand-in was asked last and its counts: the out word `DirectInput8Create` got, the
/// devices' releases, the interface's releases.
const SEEN_OUT: u32 = AREA + 0x60;
const DEVICE_RELEASES: u32 = AREA + 0x64;
const DI_RELEASES: u32 = AREA + 0x68;
/// The answer the devices give (0: `DI_OK`).
const ANSWER: u32 = AREA + 0x6c;
/// The keys down on the stand-in's keyboard (256 bytes) and its mouse state (20 bytes).
const RAW_KEYS: u32 = AREA + 0x1000;
const RAW_MOUSE: u32 = AREA + 0x1100;
/// The stand-in keyboard's buffered events (20 bytes each) and their count.
const EVENTS: u32 = AREA + 0x2000;
const EVENT_COUNT: u32 = AREA + 0x1ffc;
/// The game's buffers.
const BUF: u32 = AREA + 0x3000;
const RECORDS: u32 = AREA + 0x3200;
const COUNT: u32 = AREA + 0x33fc;

const DIERR_NOTACQUIRED: u32 = 0x8007_000c;
const DIERR_NOTBUFFERED: u32 = 0x8004_0207;

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

/// The platform's `DirectInput8Create`: version 0x800 gives the interface through `out`, any
/// other an error (and 0 through `out`, as the platform answers).
fn real_create(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(SEEN_OUT, a[3]);
    if a[1] != 0x0800 {
        e.mem.set_u32(a[3], 0);
        return eax(0x8007_047e);
    }
    e.mem.set_u32(a[3], REAL_DI);
    eax(DI_OK)
}

/// `CreateDevice(guid, out, outer)`: a new device whose `+4` is the GUID's first byte (0x61
/// keyboard, 0x60 mouse).
fn real_create_device(e: &mut Engine, a: &[u32]) -> Ret {
    let device = e.mem.u32(NEXT_DEVICE).max(DEVICES);
    e.mem.set_u32(NEXT_DEVICE, device + 0x10);
    e.mem.set_u32(device, REAL_DEVICE_VTABLE);
    e.mem.set_u32(device + 4, u32::from(e.mem.u8(a[1])));
    e.mem.set_u32(a[2], device);
    eax(DI_OK)
}

fn real_di_release(e: &mut Engine, _: &[u32]) -> Ret {
    let n = e.mem.u32(DI_RELEASES) + 1;
    e.mem.set_u32(DI_RELEASES, n);
    eax(0)
}

fn real_device_release(e: &mut Engine, _: &[u32]) -> Ret {
    let n = e.mem.u32(DEVICE_RELEASES) + 1;
    e.mem.set_u32(DEVICE_RELEASES, n);
    eax(0)
}

/// `GetDeviceState(size, data)`: the keyboard's bytes or the mouse's state, `size` of them.
fn real_get_device_state(e: &mut Engine, a: &[u32]) -> Ret {
    let answer = e.mem.u32(ANSWER);
    if answer != DI_OK {
        return eax(answer);
    }
    let from = if e.mem.u32(a[0] + 4) == 0x61 {
        RAW_KEYS
    } else {
        RAW_MOUSE
    };
    let bytes = e.mem.bytes(from, a[1]);
    e.mem.write(a[2], &bytes);
    eax(DI_OK)
}

/// `GetDeviceData(size, records, count, flags)`: as many events as `*count` asks (all with no
/// records and `INFINITE`), written when there are records, taken out unless peeking.
fn real_get_device_data(e: &mut Engine, a: &[u32]) -> Ret {
    let answer = e.mem.u32(ANSWER);
    if answer != DI_OK {
        return eax(answer);
    }
    let (records, count, flags) = (a[2], a[3], a[4]);
    let have = e.mem.u32(EVENT_COUNT);
    let wanted = e.mem.u32(count);
    let n = if records == 0 && wanted == u32::MAX {
        have
    } else if records == 0 {
        0
    } else {
        wanted.min(have)
    };
    if records != 0 {
        let bytes = e.mem.bytes(EVENTS, 0x14 * n);
        e.mem.write(records, &bytes);
    }
    if flags & DIGDD_PEEK == 0 {
        let rest = e.mem.bytes(EVENTS + 0x14 * n, 0x14 * (have - n));
        e.mem.write(EVENTS, &rest);
        e.mem.set_u32(EVENT_COUNT, have - n);
    }
    e.mem.set_u32(count, n);
    eax(DI_OK)
}

fn tick(_: &mut Engine, _: &[u32]) -> Ret {
    eax(1234)
}

/// The stand-in platform at the import slot, the hook installed over it, and the game's
/// `DirectInput8Create` made: answers the engine and xNVSE's interface.
fn game() -> (Engine, u32) {
    let mut e = Engine::new();
    e.map(AREA, 0x10000);
    e.mem.write(GUID_KEYBOARD, &GUID_SYS_KEYBOARD);
    e.mem.write(GUID_MOUSE, &GUID_SYS_MOUSE);
    e.mem.write(GUID_JOYSTICK, &[7; 16]);
    e.register(DIRECT_INPUT8_CREATE.addr(), real_create);
    e.register(GET_TICK_COUNT, tick);
    e.mem.set_u32(REAL_DI, REAL_DI_VTABLE);
    let interface: [(u32, AbiFn); 2] = [(2, real_di_release), (3, real_create_device)];
    for (slot, f) in interface {
        e.register(REAL_DI_CODE + 4 * slot, f);
        e.mem
            .set_u32(REAL_DI_VTABLE + 4 * slot, REAL_DI_CODE + 4 * slot);
    }
    let device: [(u32, AbiFn); 3] = [
        (2, real_device_release),
        (9, real_get_device_state),
        (10, real_get_device_data),
    ];
    for (slot, f) in device {
        e.register(REAL_DEVICE_CODE + 4 * slot, f);
        e.mem
            .set_u32(REAL_DEVICE_VTABLE + 4 * slot, REAL_DEVICE_CODE + 4 * slot);
    }
    install(&mut e);
    let hr = e
        .call(
            DIRECT_INPUT8_CREATE.addr(),
            &[0x0040_0000, 0x0800, IID, OUT, 0],
        )
        .u32();
    assert_eq!(hr, DI_OK);
    let di = e.mem.u32(OUT);
    (e, di)
}

/// `CreateDevice` through xNVSE's interface: the device the game gets.
fn device(e: &mut Engine, di: u32, guid: u32) -> u32 {
    assert_eq!(e.vcall(di, CREATE_DEVICE, &[guid, OUT, 0]).u32(), DI_OK);
    e.mem.u32(OUT)
}

/// The game's keyboard read (`Controls::Poll`): the 256 bytes it gets.
fn read_keys(e: &mut Engine, keyboard: u32) -> Vec<u8> {
    assert_eq!(
        e.vcall(keyboard, GET_DEVICE_STATE, &[256, BUF]).u32(),
        DI_OK
    );
    e.mem.bytes(BUF, 256)
}

fn down(keys: &[u8]) -> Vec<usize> {
    keys.iter()
        .enumerate()
        .filter(|(_, &b)| b != 0)
        .map(|(i, &b)| {
            assert_eq!(b, 0x80, "key {i:#x}");
            i
        })
        .collect()
}

fn keys(e: &mut Engine) -> &mut State {
    state(e)
}

/// The import slot gives xNVSE's interface around the platform's, which got a local of xNVSE's
/// as its out word; the keyboard and the mouse come behind xNVSE's devices, any other device
/// is the platform's own. Each fake counts its own references and releases the real object
/// once, at 0.
#[test]
fn the_import_gives_xnvses_interface_and_devices() {
    let (mut e, di) = game();
    assert_ne!(
        e.mem.u32(SEEN_OUT),
        OUT,
        "the platform wrote into xNVSE's local"
    );
    assert_eq!(e.mem.u32(e.mem.u32(di)), DIRECT_INPUT_CODE);
    assert_eq!(e.mem.u32(di + REAL), REAL_DI);
    let keyboard = device(&mut e, di, GUID_KEYBOARD);
    let mouse = device(&mut e, di, GUID_MOUSE);
    let joystick = device(&mut e, di, GUID_JOYSTICK);
    for (fake, kind, raw) in [
        (keyboard, DEVICE_KEYBOARD, 0x61),
        (mouse, DEVICE_MOUSE, 0x60),
    ] {
        assert_eq!(e.mem.u32(e.mem.u32(fake) + 4 * 9), DEVICE_CODE + 4 * 9);
        assert_eq!(e.mem.u32(fake + DEVICE_TYPE), kind);
        assert_eq!(e.mem.u32(e.mem.u32(fake + REAL) + 4), raw);
    }
    assert_eq!(
        e.mem.u32(joystick),
        REAL_DEVICE_VTABLE,
        "the platform's own"
    );
    assert_eq!(e.vcall(keyboard, 4, &[]).u32(), 2);
    assert_eq!(e.vcall(keyboard, RELEASE, &[]).u32(), 1);
    assert_eq!(e.mem.u32(DEVICE_RELEASES), 0);
    assert_eq!(e.vcall(keyboard, RELEASE, &[]).u32(), 0);
    assert_eq!(e.mem.u32(DEVICE_RELEASES), 1);
    assert_eq!(e.vcall(di, RELEASE, &[]).u32(), 0);
    assert_eq!(e.mem.u32(DI_RELEASES), 1);
    // A failed create is answered as it came, the game's word untouched.
    e.mem.set_u32(OUT, 0x5555);
    let hr = e
        .call(DIRECT_INPUT8_CREATE.addr(), &[0, 0x0700, IID, OUT, 0])
        .u32();
    assert_eq!((hr, e.mem.u32(OUT)), (0x8007_047e, 0x5555));
}

/// The keyboard read through `DIHookControl`: a player-disabled key reads up, held and tapped
/// keys down (the tap once), any non-zero byte as 0x80; a script disable blocks the holds and
/// taps and keeps the tap; each source of `IsKeyPressed` as xNVSE keeps it.
#[test]
fn keyboard_reads_go_through_the_key_states() {
    let (mut e, di) = game();
    let keyboard = device(&mut e, di, GUID_KEYBOARD);
    e.mem.set_u8(RAW_KEYS + 0x11, 0x80);
    e.mem.set_u8(RAW_KEYS + 0x1e, 0x01);
    let s = keys(&mut e);
    s.set_key_held_state(0x20, true);
    s.tap_key(0x21);
    s.set_key_disable_state(0x11, true, DISABLE_USER);
    assert_eq!(down(&read_keys(&mut e, keyboard)), [0x1e, 0x20, 0x21]);
    let s = keys(&mut e);
    assert!(s.is_key_pressed(0x11, FLAG_RAW_STATE));
    assert!(!s.is_key_pressed(0x11, 0), "the game's state by default");
    assert!(!s.is_key_pressed(0x11, FLAG_RAW_STATE | FLAG_IGNORE_DISABLED_USER));
    assert!(s.is_key_pressed(0x11, FLAG_RAW_STATE | FLAG_IGNORE_DISABLED_SCRIPT));
    assert!(s.is_key_pressed(0x20, FLAG_INSERTED_STATE));
    assert!(!s.is_key_pressed(0x20, FLAG_RAW_STATE));
    assert!(s.is_key_pressed(0x21, FLAG_GAME_STATE | FLAG_RAW_STATE));
    assert!(!s.is_key_pressed(0x1e, FLAG_INSERTED_STATE));
    assert!(
        !s.is_key_pressed(0x1e, FLAG_IGNORE_DISABLED_USER),
        "no source"
    );
    assert!(!s.is_key_pressed(MAX_MACROS, FLAG_RAW_STATE));
    // The tap was one read.
    assert_eq!(down(&read_keys(&mut e, keyboard)), [0x1e, 0x20]);
    assert!(!keys(&mut e).is_key_pressed(0x21, 0));
    // Scripts disabled: no hold, and a tap waits.
    let s = keys(&mut e);
    s.set_key_disable_state(0x20, true, DISABLE_SCRIPT);
    s.tap_key(0x20);
    assert_eq!(down(&read_keys(&mut e, keyboard)), [0x1e]);
    assert!(keys(&mut e).keys[0x20].tap);
    keys(&mut e).set_key_disable_state(0x20, false, 0);
    keys(&mut e).set_key_held_state(0x20, false);
    assert_eq!(down(&read_keys(&mut e, keyboard)), [0x1e, 0x20]);
    assert_eq!(down(&read_keys(&mut e, keyboard)), [0x1e]);
    // A shorter read gets that much; a failed read is answered, nothing processed.
    e.mem.write(BUF, &[0x55; 32]);
    assert_eq!(
        e.vcall(keyboard, GET_DEVICE_STATE, &[0x10, BUF]).u32(),
        DI_OK
    );
    assert_eq!(e.mem.u8(BUF + 0xf), 0);
    assert_eq!(e.mem.u8(BUF + 0x10), 0x55);
    keys(&mut e).tap_key(0x30);
    e.mem.set_u32(ANSWER, DIERR_NOTACQUIRED);
    assert_eq!(
        e.vcall(keyboard, GET_DEVICE_STATE, &[256, BUF]).u32(),
        DIERR_NOTACQUIRED
    );
    assert!(keys(&mut e).keys[0x30].tap);
    assert!(
        keys(&mut e).is_key_pressed(0x1e, 0),
        "the last read's state"
    );
}

/// The mouse read in place: buttons through the key states from 256, the wheel's motion as
/// keys 264 (up) and 265 (down) not given back; any size but a `DIMOUSESTATE2`'s stops (xNVSE's
/// assertion).
#[test]
fn mouse_reads_go_through_the_button_and_wheel_states() {
    let (mut e, di) = game();
    let mouse = device(&mut e, di, GUID_MOUSE);
    for (i, w) in [5i32, -3, 120].into_iter().enumerate() {
        e.mem.set_i32(RAW_MOUSE + 4 * i as u32, w);
    }
    e.mem.set_u8(RAW_MOUSE + 0xc + 1, 0x80);
    e.mem.set_u8(RAW_MOUSE + 0xc + 2, 0x01);
    let s = keys(&mut e);
    s.set_key_held_state(MOUSE_BUTTON_OFFSET + 3, true);
    s.set_key_disable_state(MOUSE_BUTTON_OFFSET + 1, true, DISABLE_USER);
    assert_eq!(e.vcall(mouse, GET_DEVICE_STATE, &[0x14, BUF]).u32(), DI_OK);
    assert_eq!([0, 4, 8].map(|o| e.mem.i32(BUF + o)), [5, -3, 120]);
    assert_eq!(e.mem.bytes(BUF + 0xc, 8), [0, 0, 0x80, 0x80, 0, 0, 0, 0]);
    let s = keys(&mut e);
    assert!(s.is_key_pressed(MOUSE_BUTTON_OFFSET + 1, FLAG_RAW_STATE));
    assert!(s.is_key_pressed(MOUSE_WHEEL_OFFSET, 0));
    assert!(!s.is_key_pressed(MOUSE_WHEEL_OFFSET + 1, 0));
    s.tap_key(MOUSE_WHEEL_OFFSET + 1);
    e.mem.set_i32(RAW_MOUSE + 8, 0);
    assert_eq!(e.vcall(mouse, GET_DEVICE_STATE, &[0x14, BUF]).u32(), DI_OK);
    assert_eq!(e.mem.i32(BUF + 8), 0, "the wheel's keys are not given back");
    let s = keys(&mut e);
    assert!(!s.is_key_pressed(MOUSE_WHEEL_OFFSET, 0));
    assert!(s.is_key_pressed(MOUSE_WHEEL_OFFSET + 1, FLAG_INSERTED_STATE));
    let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.vcall(mouse, GET_DEVICE_STATE, &[0x10, BUF]);
    }));
    assert!(stop.is_err());
}

/// Queued keys follow the real events in the buffered reads, one record at a time as the game
/// reads them; a peek with no records counts them, a flush drops them, a failed read keeps
/// them. With nothing queued the call is the device's.
#[test]
fn buffered_reads_append_the_queued_keys() {
    let (mut e, di) = game();
    let keyboard = device(&mut e, di, GUID_KEYBOARD);
    let read_one = |e: &mut Engine| -> (u32, u32, Vec<u32>) {
        e.mem.set_u32(COUNT, 1);
        let hr = e
            .vcall(keyboard, GET_DEVICE_DATA, &[0x14, RECORDS, COUNT, 0])
            .u32();
        let record = (0..5).map(|i| e.mem.u32(RECORDS + 4 * i)).collect();
        (hr, e.mem.u32(COUNT), record)
    };
    e.mem.set_u32(EVENT_COUNT, 1);
    e.mem.write(EVENTS, &[0x1c, 0, 0, 0, 0x80, 0, 0, 0]);
    keys(&mut e).buffered_key(0x30, 1234, true, true, false);
    let (hr, n, record) = read_one(&mut e);
    assert_eq!((hr, n, record[0], record[1]), (DI_OK, 1, 0x1c, 0x80));
    assert_eq!(keys(&mut e).buffered.len(), 2, "a real event first");
    let (_, n, record) = read_one(&mut e);
    assert_eq!((n, record), (1, vec![0x30, 0x80, 1234, 0, u32::MAX]));
    let (_, n, record) = read_one(&mut e);
    assert_eq!((n, record[0], record[1]), (1, 0x30, 0));
    let (hr, n, _) = read_one(&mut e);
    assert_eq!((hr, n), (DI_OK, 0), "the device's own call");
    // A peek counts without taking.
    keys(&mut e).buffered_key(0x31, 5, true, true, false);
    e.mem.set_u32(COUNT, 5);
    e.vcall(keyboard, GET_DEVICE_DATA, &[0x14, 0, COUNT, DIGDD_PEEK]);
    assert_eq!(e.mem.u32(COUNT), 2);
    assert_eq!(keys(&mut e).buffered.len(), 2);
    // A failed read: its count, the queue kept.
    e.mem.set_u32(ANSWER, DIERR_NOTBUFFERED);
    e.mem.set_u32(COUNT, 1);
    let hr = e
        .vcall(keyboard, GET_DEVICE_DATA, &[0x14, RECORDS, COUNT, 0])
        .u32();
    assert_eq!((hr, e.mem.u32(COUNT)), (DIERR_NOTBUFFERED, 1));
    assert_eq!(keys(&mut e).buffered.len(), 2);
    e.mem.set_u32(ANSWER, DI_OK);
    // `Controls::ClearKeystrokes`: no records, INFINITE: the real ones and the queue go.
    e.mem.set_u32(EVENT_COUNT, 1);
    e.mem.set_u32(COUNT, u32::MAX);
    e.vcall(keyboard, GET_DEVICE_DATA, &[0x14, 0, COUNT, 0]);
    assert_eq!(e.mem.u32(COUNT), 3);
    assert!(keys(&mut e).buffered.is_empty());
    // A peek with records stops (xNVSE's HALT).
    keys(&mut e).buffered_key(0x31, 5, true, true, false);
    let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.mem.set_u32(COUNT, 5);
        e.vcall(
            keyboard,
            GET_DEVICE_DATA,
            &[0x14, RECORDS, COUNT, DIGDD_PEEK],
        );
    }));
    assert!(stop.is_err());
}

/// Several queued records in one read: after the first, xNVSE writes each at the second
/// record's place (its pointer is set to `records + size`, not moved on), so the third
/// overwrites the second and the third place is left alone; the count says three.
#[test]
fn several_queued_records_overwrite_the_second_place() {
    let (mut e, di) = game();
    let keyboard = device(&mut e, di, GUID_KEYBOARD);
    for key in [0x10, 0x11, 0x12] {
        keys(&mut e).buffered_key(key, 9, true, false, false);
    }
    e.mem.write(RECORDS, &[0xaa; 0x3c]);
    e.mem.set_u32(COUNT, 3);
    e.vcall(keyboard, GET_DEVICE_DATA, &[0x14, RECORDS, COUNT, 0]);
    assert_eq!(e.mem.u32(COUNT), 3);
    assert_eq!(
        [0, 0x14, 0x28].map(|o| e.mem.u32(RECORDS + o)),
        [0x10, 0x12, 0xaaaa_aaaa]
    );
}

/// `KeyInfo::Process` alone: a user disable wins over the raw key, a script disable over the
/// holds, a hold and a tap together insert once and use the tap.
#[test]
fn process_follows_xnvses_order() {
    let mut k = KeyInfo {
        user_disable: true,
        hold: true,
        ..KeyInfo::default()
    };
    assert!(k.process(true), "held, though the player's key is disabled");
    assert_eq!(
        (k.raw_state, k.game_state, k.inserted_state),
        (true, true, true)
    );
    k.script_disable = true;
    assert!(!k.process(true));
    assert_eq!((k.raw_state, k.inserted_state), (true, false));
    let mut k = KeyInfo {
        hold: true,
        tap: true,
        ..KeyInfo::default()
    };
    assert!(k.process(false));
    assert!(!k.tap);
}

/// The queue of menu keys: with the size caps row a key past [`MAX_QUEUED`] is dropped (a tap's
/// release with it); without the row the queue grows until the out-of-memory stop (here at a
/// small bound in place of [`OUT_OF_MEMORY_QUEUED`]).
#[test]
fn the_key_queue_is_capped_or_stops_as_out_of_memory() {
    let mut s = State::default();
    for _ in 0..MAX_QUEUED - 1 {
        s.buffered_key(0x10, 1, true, false, true);
    }
    s.buffered_key(0x1c, 2, true, true, true);
    assert_eq!(s.buffered.len(), MAX_QUEUED);
    assert_eq!(s.buffered.back(), Some(&[0x1c, 0x80, 2, 0, u32::MAX]));
    s.buffered_key(0x1c, 3, true, false, true);
    assert_eq!(s.buffered.len(), MAX_QUEUED, "dropped");
    s.buffered_key(0x1c, 3, true, false, false);
    assert_eq!(s.buffered.len(), MAX_QUEUED + 1, "no cap without the row");
    let mut s = State::default();
    s.queue([1, 0x80, 0, 0, 0], usize::MAX, 2);
    s.queue([1, 0, 0, 0, 0], usize::MAX, 2);
    let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        s.queue([2, 0x80, 0, 0, 0], usize::MAX, 2);
    }));
    assert!(stop.is_err());
}
