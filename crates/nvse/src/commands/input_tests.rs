//! Tests of the input commands through their records, on a `Controls` object with bindings set
//! by the tests and the key states the DirectInput hook keeps; each against
//! `Commands_Input.cpp` (xNVSE).

use super::super::test_support::*;
use super::*;
use crate::input::{FLAG_INSERTED_STATE, FLAG_RAW_STATE};
use std::cell::RefCell;

/// The `Controls` object `g_OSInputGlobals` points at, and its bindings.
const CONTROLS: u32 = 0x0200_4000;
const BINDS: u32 = CONTROLS + 0x1b94;
/// The console the prints go to.
const CONSOLE: u32 = 0x0200_7000;

thread_local! {
    static PRINTED: RefCell<Vec<(Vec<u8>, Vec<u32>)>> = const { RefCell::new(Vec::new()) };
}

fn ret(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

/// `00a24080` as the exe has it: the gamepad binding's button mask.
fn binding_to_mask(_: &mut Engine, a: &[u32]) -> Ret {
    ret(match a[1] {
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
    })
}

/// `ConsoleManager::Print(console, format, va_list)`: the format and the first three words.
fn console_print_fake(e: &mut Engine, a: &[u32]) -> Ret {
    let text = crate::table::guest_text(e, a[1]);
    let words = (0..3).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
    PRINTED.with(|p| p.borrow_mut().push((text, words)));
    ret(0)
}

fn printed() -> Vec<(Vec<u8>, Vec<u32>)> {
    PRINTED.with(|p| p.borrow_mut().drain(..).collect())
}

/// Bindings: control 0 (forward) W on the keyboard, the joystick's button 3, the pad's A;
/// control 1 S; control 4 (attack) the left mouse button only; control 5 E and the right
/// button; every other unbound. The script is of mod 5. No console.
fn setup() -> Engine {
    let mut e = engine_with_table();
    e.map(0x011f_3000, 0x1000);
    e.mem.set_u32(0x011f_35cc, CONTROLS);
    for i in 0..4 * MAX_CONTROL_BINDS {
        e.mem.set_u8(BINDS + i, 0xff);
    }
    let bind = |e: &mut Engine, kind: u32, control: u32, key: u8| {
        e.mem.set_u8(BINDS + 0x1c * kind + control, key);
    };
    bind(&mut e, KEYBOARD, 0, 0x11);
    bind(&mut e, KEYBOARD, 1, 0x1f);
    bind(&mut e, KEYBOARD, 5, 0x12);
    bind(&mut e, MOUSE, 4, 0);
    bind(&mut e, MOUSE, 5, 1);
    bind(&mut e, JOYSTICK, 0, 3);
    bind(&mut e, GAMEPAD, 0, 10);
    e.register(BINDING_TO_MASK, binding_to_mask);
    e.register(0x0071_b160, |_, _| ret(0));
    e.register(0x00fd_f060, |_, _| ret(777));
    e.mem.set_u32(SCRIPT + 0xc, 0x0500_0d00);
    e
}

/// The console open, a console line running, its prints kept.
fn console(e: &mut Engine) {
    e.mem.set_u8(0x011d_ea2e, 1);
    let tls = e.tls();
    e.mem.set_u8(tls + 0x268, 1);
    e.register(0x0071_b160, |_, _| ret(CONSOLE));
    e.register(0x0071_d0a0, console_print_fake);
}

fn r(e: &mut Engine, opcode: u32, args: &[i32]) -> f64 {
    let args: Vec<Vec<u8>> = args.iter().map(|&v| n(v)).collect();
    let (ok, result) = call(e, opcode, 0, &args);
    assert_eq!(ok, 1);
    result
}

const IS_KEY_PRESSED: u32 = 0x1453;
const TAP_KEY: u32 = 0x1454;
const HOLD_KEY: u32 = 0x1455;
const RELEASE_KEY: u32 = 0x1456;
const DISABLE_KEY: u32 = 0x1457;
const ENABLE_KEY: u32 = 0x1458;
const GET_NUM_KEYS_PRESSED: u32 = 0x1459;
const GET_KEY_PRESS: u32 = 0x145a;
const GET_NUM_MOUSE_BUTTONS_PRESSED: u32 = 0x145b;
const GET_MOUSE_BUTTON_PRESS: u32 = 0x145c;
const GET_CONTROL: u32 = 0x145d;
const GET_ALT_CONTROL: u32 = 0x145e;
const MENU_TAP_KEY: u32 = 0x145f;
const MENU_HOLD_KEY: u32 = 0x1460;
const MENU_RELEASE_KEY: u32 = 0x1461;
const DISABLE_CONTROL: u32 = 0x1462;
const ENABLE_CONTROL: u32 = 0x1463;
const TAP_CONTROL: u32 = 0x1464;
const SET_CONTROL: u32 = 0x1465;
const SET_ALT_CONTROL: u32 = 0x1466;
const SET_IS_CONTROL: u32 = 0x1467;
const IS_CONTROL: u32 = 0x1468;
const IS_KEY_DISABLED: u32 = 0x1469;
const IS_CONTROL_DISABLED: u32 = 0x146a;
const IS_CONTROL_PRESSED: u32 = 0x146b;
const GET_DISABLED_KEYS: u32 = 0x1679;
const GET_PRESSED_KEYS: u32 = 0x167b;

fn binding(e: &Engine, kind: u32, control: u32) -> u8 {
    e.mem.u8(BINDS + 0x1c * kind + control)
}

/// The numbers of an array a command made, in order.
fn array_numbers(e: &Engine, id: f64) -> Vec<f64> {
    let arr = vars::array(e, id as u32).expect("an array");
    let vars::Container::Array(elements) = &arr.elements else {
        panic!("a packed array");
    };
    elements
        .iter()
        .map(|el| match el.data {
            vars::Data::Number(v) => v,
            _ => panic!("a number"),
        })
        .collect()
}

/// The keys down for one keyboard read, as the hook processes it.
fn keyboard_read(e: &mut Engine, down: &[usize]) {
    let mut bytes = [0u8; 256];
    for &k in down {
        bytes[k] = 0x80;
    }
    input::state(e).process_keyboard_data(&mut bytes);
}

/// The records: names, parameters, condition evals, return types as xNVSE registers them.
#[test]
fn the_records_are_xnvses() {
    let e = setup();
    let record = |opcode: u32| crate::table::Record::read(&e, crate::table::record_of(&e, opcode));
    for (opcode, count, flags) in [
        (IS_KEY_PRESSED, 2, 1),
        (TAP_KEY, 1, 0),
        (DISABLE_KEY, 2, 0),
        (GET_NUM_KEYS_PRESSED, 1, 0),
        (GET_NUM_MOUSE_BUTTONS_PRESSED, 0, 0),
        (SET_CONTROL, 3, 0),
        (SET_ALT_CONTROL, 2, 0),
        (IS_CONTROL_PRESSED, 2, 1),
        (GET_DISABLED_KEYS, 0, 0),
        (GET_PRESSED_KEYS, 1, 0),
    ] {
        let r = record(opcode);
        assert_eq!((r.param_count, r.flags, r.needs_ref), (count, flags, 0));
        assert_eq!(r.eval != 0, flags == 1);
    }
    let r = record(GET_NUM_KEYS_PRESSED);
    assert_eq!(e.mem.u32(r.params + 8), 1, "optional");
    assert_eq!(
        crate::table::return_type(&e, GET_PRESSED_KEYS),
        Returns::Array
    );
    assert_eq!(
        crate::table::return_type(&e, GET_DISABLED_KEYS),
        Returns::Array
    );
    assert_eq!(crate::table::return_type(&e, GET_CONTROL), Returns::Default);
}

/// `GetControl` per device: the keyboard's and joystick's byte, the mouse's from 256, the pad's
/// as a button mask (an unbound pad control is mask 0); -1 unbound, past the 28 controls or for
/// a device past the pad. `GetAltControl` drops the mouse's 256.
#[test]
fn get_control_reads_each_device() {
    let mut e = setup();
    for (args, want) in [
        (&[0][..], 17.0),
        (&[0, 0], 17.0),
        (&[0, 1], -1.0),
        (&[4, 1], 256.0),
        (&[5, 1], 257.0),
        (&[0, 2], 3.0),
        (&[1, 2], -1.0),
        (&[0, 3], 4096.0),
        (&[1, 3], 0.0),
        (&[0, 4], -1.0),
        (&[28], -1.0),
        (&[4], -1.0),
    ] {
        assert_eq!(r(&mut e, GET_CONTROL, args), want, "{args:?}");
    }
    assert_eq!(r(&mut e, GET_ALT_CONTROL, &[4]), 0.0);
    assert_eq!(r(&mut e, GET_ALT_CONTROL, &[5]), 1.0);
    assert_eq!(r(&mut e, GET_ALT_CONTROL, &[0]), -1.0);
    assert_eq!(r(&mut e, GET_ALT_CONTROL, &[40]), -1.0);
}

/// `SetControl` swaps with the control that had the key; the mouse's codes from 256 lose it; the
/// pad's mask becomes its binding (30 for an unknown one) and its start, back, guide and
/// directions are refused; a code past a byte is kept as its low byte and swaps with nothing.
#[test]
fn set_control_swaps_bindings() {
    let mut e = setup();
    r(&mut e, SET_CONTROL, &[1, 0x11]);
    assert_eq!(
        (binding(&e, KEYBOARD, 0), binding(&e, KEYBOARD, 1)),
        (0x1f, 0x11)
    );
    r(&mut e, SET_CONTROL, &[0, 0x101, 1]);
    assert_eq!((binding(&e, MOUSE, 0), binding(&e, MOUSE, 5)), (1, 0xff));
    r(&mut e, SET_ALT_CONTROL, &[4, 2]);
    assert_eq!(binding(&e, MOUSE, 4), 2);
    r(&mut e, SET_CONTROL, &[2, 0x0001, 3]);
    r(&mut e, SET_CONTROL, &[2, 0x0400, 3]);
    assert_eq!(binding(&e, GAMEPAD, 2), 0xff, "refused");
    r(&mut e, SET_CONTROL, &[2, 0x1000, 3]);
    assert_eq!(
        (binding(&e, GAMEPAD, 0), binding(&e, GAMEPAD, 2)),
        (0xff, 10)
    );
    r(&mut e, SET_CONTROL, &[3, 0x9999, 3]);
    assert_eq!(binding(&e, GAMEPAD, 3), 30);
    r(&mut e, SET_CONTROL, &[6, 600]);
    assert_eq!(binding(&e, KEYBOARD, 6), 88);
    r(&mut e, SET_CONTROL, &[28, 0x20]);
    r(&mut e, SET_CONTROL, &[7, 0x20, 4]);
    assert_eq!(binding(&e, KEYBOARD, 7), 0xff);
}

/// `IsControl`: 1 for a bound key (a mouse button from 256), 2 for one a mod registered, 3 for
/// both; unregistering leaves 0. Key 255 matches an unbound control's byte.
#[test]
fn is_control_and_custom_controls() {
    let mut e = setup();
    assert_eq!(r(&mut e, IS_CONTROL, &[0x11]), 1.0);
    assert_eq!(r(&mut e, IS_CONTROL, &[256]), 1.0);
    assert_eq!(r(&mut e, IS_CONTROL, &[0x50]), 0.0);
    assert_eq!(r(&mut e, IS_CONTROL, &[255]), 1.0);
    r(&mut e, SET_IS_CONTROL, &[0x50, 1]);
    assert_eq!(r(&mut e, IS_CONTROL, &[0x50]), 2.0);
    r(&mut e, SET_IS_CONTROL, &[0x11, 1]);
    assert_eq!(r(&mut e, IS_CONTROL, &[0x11]), 3.0);
    r(&mut e, SET_IS_CONTROL, &[0x50, 0]);
    assert_eq!(r(&mut e, IS_CONTROL, &[0x50]), 0.0);
    r(&mut e, SET_IS_CONTROL, &[300, 1]);
    assert!(!input::state(&mut e).registered_controls.contains_key(&300));
    assert_eq!(
        input::state(&mut e).registered_controls[&0x11],
        std::collections::BTreeSet::from([5u8])
    );
}

/// `DisableControl` and `EnableControl`: the control's mask and its key and button with it; a
/// mask of 0 disables the keys but not the control; an unbound key is key 255, which
/// `GetDisabledKeys` leaves out.
#[test]
fn disabling_controls_disables_their_keys() {
    let mut e = setup();
    r(&mut e, DISABLE_CONTROL, &[5]);
    assert_eq!(r(&mut e, IS_CONTROL_DISABLED, &[5]), 1.0);
    assert_eq!(r(&mut e, IS_KEY_DISABLED, &[0x12]), 1.0);
    assert_eq!(r(&mut e, IS_KEY_DISABLED, &[257]), 1.0);
    let keys = r(&mut e, GET_DISABLED_KEYS, &[]);
    assert_eq!(array_numbers(&e, keys), [18.0, 257.0]);
    r(&mut e, ENABLE_CONTROL, &[5, 2]);
    assert_eq!(
        r(&mut e, IS_CONTROL_DISABLED, &[5]),
        1.0,
        "the user bit left"
    );
    assert!(input::state(&mut e).keys[0x12].user_disable);
    assert!(!input::state(&mut e).keys[0x12].script_disable);
    r(&mut e, ENABLE_CONTROL, &[5]);
    assert_eq!(r(&mut e, IS_CONTROL_DISABLED, &[5]), 0.0);
    assert_eq!(r(&mut e, IS_KEY_DISABLED, &[0x12]), 0.0);
    r(&mut e, DISABLE_CONTROL, &[0, 0]);
    assert_eq!(r(&mut e, IS_CONTROL_DISABLED, &[0]), 0.0);
    assert_eq!(r(&mut e, IS_KEY_DISABLED, &[0x11]), 1.0);
    assert_eq!(r(&mut e, IS_KEY_DISABLED, &[0xff]), 1.0);
    let keys = r(&mut e, GET_DISABLED_KEYS, &[]);
    assert_eq!(array_numbers(&e, keys), [17.0]);
    r(&mut e, DISABLE_CONTROL, &[28]);
    assert_eq!(r(&mut e, IS_CONTROL_DISABLED, &[28]), 0.0);
}

/// The key commands set the hook's states; `TapControl` taps the mouse button before the key,
/// 0 when neither is bound; the menu keys are queued with the tick count, codes from 256 not.
#[test]
fn key_commands_set_the_hooks_states() {
    let mut e = setup();
    r(&mut e, TAP_KEY, &[0x30]);
    r(&mut e, HOLD_KEY, &[0x31]);
    r(&mut e, DISABLE_KEY, &[0x32, 1]);
    r(&mut e, DISABLE_KEY, &[0x33]);
    let k = input::state(&mut e).keys;
    assert!(k[0x30].tap && k[0x31].hold);
    assert!(k[0x32].user_disable && !k[0x32].script_disable);
    assert!(k[0x33].user_disable && k[0x33].script_disable);
    r(&mut e, RELEASE_KEY, &[0x31]);
    r(&mut e, ENABLE_KEY, &[0x33, 2]);
    let k = input::state(&mut e).keys;
    assert!(!k[0x31].hold);
    assert!(k[0x33].user_disable && !k[0x33].script_disable);
    assert_eq!(r(&mut e, TAP_CONTROL, &[4]), 1.0);
    assert!(input::state(&mut e).keys[256].tap);
    assert_eq!(r(&mut e, TAP_CONTROL, &[0]), 1.0);
    assert!(input::state(&mut e).keys[0x11].tap);
    assert_eq!(r(&mut e, TAP_CONTROL, &[7]), 0.0);
    r(&mut e, MENU_TAP_KEY, &[0x1c]);
    r(&mut e, MENU_HOLD_KEY, &[0x2a]);
    r(&mut e, MENU_HOLD_KEY, &[300]);
    r(&mut e, MENU_RELEASE_KEY, &[0x2a]);
    let queued: Vec<_> = input::state(&mut e).buffered.iter().copied().collect();
    assert_eq!(
        queued,
        [
            [0x1c, 0x80, 777, 0, u32::MAX],
            [0x1c, 0, 777, 0, u32::MAX],
            [0x2a, 0x80, 777, 0, u32::MAX],
            [0x2a, 0, 777, 0, u32::MAX],
        ]
    );
}

/// `MenuTapKey` with the size caps row on queues up to the cap: a tap at the last place keeps its
/// press and drops its release.
#[test]
fn menu_keys_respect_the_size_caps_row() {
    let mut e = setup();
    crate::commands::array::set_fix(&mut e, crate::commands::array::SIZE_CAPS, true);
    for _ in 0..input::MAX_QUEUED - 1 {
        r(&mut e, MENU_HOLD_KEY, &[0x10]);
    }
    r(&mut e, MENU_TAP_KEY, &[0x1c]);
    let s = input::state(&mut e);
    assert_eq!(s.buffered.len(), input::MAX_QUEUED);
    assert_eq!(s.buffered.back(), Some(&[0x1c, 0x80, 777, 0, u32::MAX]));
}

/// The readers over the hook's states: `IsKeyPressed` and `IsControlPressed` (also as
/// conditions), the counts, the n-th key and button, the array of keys down.
#[test]
fn readers_count_and_find_the_keys_down() {
    let mut e = setup();
    input::state(&mut e).set_key_held_state(0x20, true);
    keyboard_read(&mut e, &[0x11, 0x1e, 0xff]);
    let mut buttons = [0, 0x80, 0, 0x80, 0, 0, 0, 0];
    input::state(&mut e).process_mouse_data(&mut buttons, 0);
    assert_eq!(r(&mut e, IS_KEY_PRESSED, &[0x11]), 1.0);
    assert_eq!(
        r(&mut e, IS_KEY_PRESSED, &[0x20, FLAG_RAW_STATE as i32]),
        0.0
    );
    assert_eq!(
        r(&mut e, IS_KEY_PRESSED, &[0x20, FLAG_INSERTED_STATE as i32]),
        1.0
    );
    assert_eq!(eval(&mut e, IS_KEY_PRESSED, 0, 0x1e, 0), 1.0);
    assert_eq!(eval(&mut e, IS_KEY_PRESSED, 0, 0x1f, 0), 0.0);
    assert_eq!(r(&mut e, IS_CONTROL_PRESSED, &[0]), 1.0, "W");
    assert_eq!(r(&mut e, IS_CONTROL_PRESSED, &[5]), 1.0, "the right button");
    assert_eq!(r(&mut e, IS_CONTROL_PRESSED, &[4]), 0.0);
    assert_eq!(eval(&mut e, IS_CONTROL_PRESSED, 0, 1, 0), 0.0);
    assert_eq!(eval(&mut e, IS_CONTROL_PRESSED, 0, 0, FLAG_RAW_STATE), 1.0);
    // 0xff is down but not counted.
    assert_eq!(r(&mut e, GET_NUM_KEYS_PRESSED, &[]), 3.0);
    assert_eq!(
        r(&mut e, GET_NUM_KEYS_PRESSED, &[FLAG_RAW_STATE as i32]),
        2.0
    );
    assert_eq!(r(&mut e, GET_KEY_PRESS, &[0]), 17.0);
    assert_eq!(r(&mut e, GET_KEY_PRESS, &[2]), 32.0);
    assert_eq!(r(&mut e, GET_KEY_PRESS, &[3]), -1.0);
    assert_eq!(
        r(&mut e, GET_KEY_PRESS, &[1, FLAG_INSERTED_STATE as i32]),
        -1.0
    );
    assert_eq!(r(&mut e, GET_NUM_MOUSE_BUTTONS_PRESSED, &[]), 2.0);
    assert_eq!(r(&mut e, GET_MOUSE_BUTTON_PRESS, &[1]), 259.0);
    assert_eq!(r(&mut e, GET_MOUSE_BUTTON_PRESS, &[2]), -1.0);
    let keys = r(&mut e, GET_PRESSED_KEYS, &[]);
    assert_eq!(array_numbers(&e, keys), [17.0, 30.0, 32.0, 257.0, 259.0]);
    let keys = r(&mut e, GET_PRESSED_KEYS, &[FLAG_INSERTED_STATE as i32]);
    assert_eq!(array_numbers(&e, keys), [32.0]);
}

/// In the console `GetControl`, `GetAltControl` and the counts print what xNVSE prints; out of
/// it they print nothing.
#[test]
fn console_lines_print_as_xnvse_prints() {
    let mut e = setup();
    r(&mut e, GET_CONTROL, &[0]);
    console(&mut e);
    printed();
    r(&mut e, GET_CONTROL, &[0]);
    r(&mut e, GET_ALT_CONTROL, &[4]);
    r(&mut e, GET_CONTROL, &[9, 1]);
    r(&mut e, GET_NUM_KEYS_PRESSED, &[]);
    r(&mut e, GET_NUM_MOUSE_BUTTONS_PRESSED, &[]);
    let [lo17, hi17] = double_words(17.0);
    let [lo0, hi0] = double_words(0.0);
    let [lom1, him1] = double_words(-1.0);
    let want: [(&[u8], &[u32]); 5] = [
        (b"GetControl %d >> %.0f", &[0, lo17, hi17]),
        (b"GetAltControl %d >> %.0f", &[4, lo0, hi0]),
        (b"GetControl %d >> %.0f", &[9, lom1, him1]),
        (b"keysPressed: %d", &[0]),
        (b"buttonsPressed: %d", &[0]),
    ];
    let got = printed();
    assert_eq!(got.len(), want.len());
    for ((text, words), (want_text, want_words)) in got.iter().zip(want) {
        assert_eq!(
            (&text[..], &words[..want_words.len()]),
            (want_text, want_words)
        );
    }
}

/// The bindings both games load from the test saves' `FalloutPrefs.ini` (`[Controls]`, the 28
/// controls in order): per control `0x00KKMMPP`, the keyboard's, mouse's and gamepad's bytes.
/// No joystick is bound (the real game answers -1 for every control on device 2).
const PREFS_CONTROLS: [u32; 28] = [
    0x0011_ff13, // Forward
    0x001f_ff14, // Back
    0x001e_ff17, // Slide Left
    0x0020_ff16, // Slide Right
    0x00ff_0011, // Use
    0x0012_ff0a, // Activate
    0x0038_0110, // Block
    0x0013_ff0c, // Ready Item
    0x001d_ff08, // Crouch/Sneak
    0x002a_ffff, // Run
    0x003a_ffff, // Always Run
    0x0010_ffff, // Auto Move
    0x0039_ff0d, // Jump
    0x0021_020f, // Toggle POV
    0x000f_ff0b, // Menu Mode
    0x0014_ff07, // Rest
    0x002f_ff0e, // Vats
    0x0002_ffff, // Hotkey1
    0x0003_ff01, // Ammo Swap
    0x0004_ffff, // Hotkey3
    0x0005_ffff, // Hotkey4
    0x0006_ffff, // Hotkey5
    0x0007_ffff, // Hotkey6
    0x0008_ffff, // Hotkey7
    0x0009_ffff, // Hotkey8
    0x003f_ffff, // QuickSave
    0x0043_ffff, // QuickLoad
    0x002c_ff09, // Grab
];

/// [`setup`] with the INI's bindings.
fn setup_prefs() -> Engine {
    let e = setup();
    for (control, value) in PREFS_CONTROLS.iter().enumerate() {
        let [pad, mouse, keyboard, _] = value.to_le_bytes();
        for (kind, byte) in [
            (KEYBOARD, keyboard),
            (MOUSE, mouse),
            (JOYSTICK, 0xff),
            (GAMEPAD, pad),
        ] {
            e.mem.set_u8(BINDS + 0x1c * kind + control as u32, byte);
        }
    }
    e
}

/// What `gr_o_input.txt`'s `rArr` prints for an array: its size, then each number.
fn array_line(e: &Engine, id: f64) -> String {
    let numbers = array_numbers(e, id);
    let mut line = format!("{}:", numbers.len());
    for v in numbers {
        line += &format!(" {v}");
    }
    line
}

/// The lines `real-tests/gr_o_input.txt` prints, made here by the same commands in the same
/// order: at the first `PostLoadGame` the bindings, the bound keys, custom controls, a swap and
/// back, the mouse and pad, disabled controls; then one step a frame (holds, taps and disables
/// on keys 22-25 and mouse button 3, every source of `IsKeyPressed`, the counts), with the
/// game's keyboard and mouse reads between frames.
fn o_input_lines() -> Vec<String> {
    let mut e = setup_prefs();
    let mut out = Vec::new();
    for c in 0..30 {
        let v = [
            r(&mut e, GET_CONTROL, &[c]),
            r(&mut e, GET_CONTROL, &[c, 1]),
            r(&mut e, GET_CONTROL, &[c, 2]),
            r(&mut e, GET_CONTROL, &[c, 3]),
            r(&mut e, GET_ALT_CONTROL, &[c]),
            r(&mut e, GET_CONTROL, &[c, 4]),
        ];
        out.push(format!(
            "ctl {c} {} {} {} {} {} {}",
            v[0], v[1], v[2], v[3], v[4], v[5]
        ));
    }
    let mut line = "isctl".to_string();
    for k in 0..270 {
        let v = r(&mut e, IS_CONTROL, &[k]);
        if v != 0.0 {
            line += &format!(" {k}={v}");
        }
    }
    out.push(line);
    r(&mut e, SET_IS_CONTROL, &[22, 1]);
    let a = r(&mut e, IS_CONTROL, &[22]);
    r(&mut e, SET_IS_CONTROL, &[17, 1]);
    let b = r(&mut e, IS_CONTROL, &[17]);
    r(&mut e, SET_IS_CONTROL, &[22, 0]);
    let d = r(&mut e, IS_CONTROL, &[22]);
    r(&mut e, SET_IS_CONTROL, &[17, 0]);
    let f = r(&mut e, IS_CONTROL, &[17]);
    out.push(format!("custom {a} {b} {d} {f}"));
    r(&mut e, SET_CONTROL, &[15, 17]);
    let a = r(&mut e, GET_CONTROL, &[0]);
    let b = r(&mut e, GET_CONTROL, &[15]);
    r(&mut e, SET_CONTROL, &[15, 20]);
    let d = r(&mut e, GET_CONTROL, &[0]);
    let f = r(&mut e, GET_CONTROL, &[15]);
    out.push(format!("swap {a} {b} {d} {f}"));
    r(&mut e, SET_ALT_CONTROL, &[6, 3]);
    let a = r(&mut e, GET_ALT_CONTROL, &[6]);
    let b = r(&mut e, GET_CONTROL, &[6, 1]);
    r(&mut e, SET_ALT_CONTROL, &[6, 1]);
    let d = r(&mut e, GET_ALT_CONTROL, &[6]);
    let g = r(&mut e, GET_CONTROL, &[5, 3]);
    r(&mut e, SET_CONTROL, &[5, 32768, 3]);
    let pe = r(&mut e, GET_CONTROL, &[5, 3]);
    let k = r(&mut e, GET_CONTROL, &[12, 3]);
    r(&mut e, SET_CONTROL, &[5, g as i32, 3]);
    let pf = r(&mut e, GET_CONTROL, &[5, 3]);
    let x = r(&mut e, GET_CONTROL, &[12, 3]);
    out.push(format!("alt {a} {b} {d} pad {g} {pe} {k} {pf} {x}"));
    r(&mut e, DISABLE_CONTROL, &[0]);
    let a = r(&mut e, IS_CONTROL_DISABLED, &[0]);
    let b = r(&mut e, IS_KEY_DISABLED, &[17]);
    r(&mut e, DISABLE_CONTROL, &[4, 0]);
    let d = r(&mut e, IS_CONTROL_DISABLED, &[4]);
    let f1 = r(&mut e, IS_KEY_DISABLED, &[255]);
    let f2 = r(&mut e, IS_KEY_DISABLED, &[256]);
    out.push(format!("dis {a} {b} {d} {f1} {f2}"));
    let arr = r(&mut e, GET_DISABLED_KEYS, &[]);
    out.push(array_line(&e, arr));
    r(&mut e, ENABLE_CONTROL, &[0, 2]);
    let a = r(&mut e, IS_CONTROL_DISABLED, &[0]);
    let b = r(&mut e, IS_KEY_DISABLED, &[17]);
    r(&mut e, ENABLE_CONTROL, &[0]);
    r(&mut e, ENABLE_CONTROL, &[4]);
    let d = r(&mut e, IS_CONTROL_DISABLED, &[0]);
    let f1 = r(&mut e, IS_KEY_DISABLED, &[17]);
    let f2 = r(&mut e, IS_KEY_DISABLED, &[255]);
    out.push(format!("en {a} {b} {d} {f1} {f2}"));
    let arr = r(&mut e, GET_DISABLED_KEYS, &[]);
    out.push(array_line(&e, arr));
    let a = r(&mut e, TAP_CONTROL, &[28]);
    out.push(format!("tapctl {a}"));
    for step in 1..=6 {
        if step == 1 {
            r(&mut e, HOLD_KEY, &[22]);
            r(&mut e, TAP_KEY, &[23]);
            r(&mut e, DISABLE_KEY, &[24, 1]);
            r(&mut e, DISABLE_KEY, &[25, 2]);
            r(&mut e, TAP_KEY, &[25]);
            r(&mut e, HOLD_KEY, &[259]);
        }
        for k in [22, 23, 24, 25, 259] {
            let v = [
                r(&mut e, IS_KEY_PRESSED, &[k]),
                r(&mut e, IS_KEY_PRESSED, &[k, 2]),
                r(&mut e, IS_KEY_PRESSED, &[k, 4]),
                r(&mut e, IS_KEY_PRESSED, &[k, 9]),
                r(&mut e, IS_KEY_PRESSED, &[k, 17]),
                r(&mut e, IS_KEY_DISABLED, &[k]),
            ];
            out.push(format!(
                "f {step} k {k} {} {} {} {} {} {}",
                v[0], v[1], v[2], v[3], v[4], v[5]
            ));
        }
        let v = [
            r(&mut e, GET_NUM_KEYS_PRESSED, &[4]),
            r(&mut e, GET_KEY_PRESS, &[0, 4]),
            r(&mut e, GET_NUM_MOUSE_BUTTONS_PRESSED, &[]),
            r(&mut e, GET_MOUSE_BUTTON_PRESS, &[0]),
            r(&mut e, GET_NUM_KEYS_PRESSED, &[2]),
        ];
        out.push(format!(
            "f {step} counts {} {} {} {} {}",
            v[0], v[1], v[2], v[3], v[4]
        ));
        let arr = r(&mut e, GET_PRESSED_KEYS, &[4]);
        out.push(array_line(&e, arr));
        match step {
            3 => {
                r(&mut e, ENABLE_KEY, &[25, 2]);
            }
            4 => {
                r(&mut e, RELEASE_KEY, &[22]);
                r(&mut e, RELEASE_KEY, &[259]);
                r(&mut e, ENABLE_KEY, &[24]);
            }
            _ => {}
        }
        // The frame's reads of the keyboard and the mouse, nothing down.
        keyboard_read(&mut e, &[]);
        input::state(&mut e).process_mouse_data(&mut [0; 8], 0);
    }
    out
}

/// What the real game printed for `real-tests/gr_o_input.txt` (xNVSE 6.4.9 with JIP LN 57.30,
/// run `inp3`, `nvsecos4` loaded, 2026-11-08), in order, except two lines marked below. JIP
/// replaces six of xNVSE's input commands with its own when it loads (`patches_cmd.h`:
/// `DisableControl`, `EnableControl` and `TapControl` without the mask and skipping unbound
/// keys, `IsControlDisabled` from the bound keys, `IsKeyPressed` and `IsControlPressed` reading
/// only the sources of the flags); they come with JIP's own crate. Where they would answer
/// differently, the lines are xNVSE's: the real game printed `dis 1 1 1 0 1` (control 4 counted
/// disabled from its button, unbound key 255 left alone) and `en 0 0 0 0 0` (the mask ignored).
/// Every other line is the same under both.
const REAL_O_INPUT: &str = "\
ctl 0 17 -1 -1 0 -1 -1\n\
ctl 1 31 -1 -1 0 -1 -1\n\
ctl 2 30 -1 -1 0 -1 -1\n\
ctl 3 32 -1 -1 0 -1 -1\n\
ctl 4 -1 256 -1 0 0 -1\n\
ctl 5 18 -1 -1 4096 -1 -1\n\
ctl 6 56 257 -1 0 1 -1\n\
ctl 7 19 -1 -1 16384 -1 -1\n\
ctl 8 29 -1 -1 64 -1 -1\n\
ctl 9 42 -1 -1 0 -1 -1\n\
ctl 10 58 -1 -1 0 -1 -1\n\
ctl 11 16 -1 -1 0 -1 -1\n\
ctl 12 57 -1 -1 32768 -1 -1\n\
ctl 13 33 258 -1 256 2 -1\n\
ctl 14 15 -1 -1 8192 -1 -1\n\
ctl 15 20 -1 -1 32 -1 -1\n\
ctl 16 47 -1 -1 512 -1 -1\n\
ctl 17 2 -1 -1 0 -1 -1\n\
ctl 18 3 -1 -1 1 -1 -1\n\
ctl 19 4 -1 -1 0 -1 -1\n\
ctl 20 5 -1 -1 0 -1 -1\n\
ctl 21 6 -1 -1 0 -1 -1\n\
ctl 22 7 -1 -1 0 -1 -1\n\
ctl 23 8 -1 -1 0 -1 -1\n\
ctl 24 9 -1 -1 0 -1 -1\n\
ctl 25 63 -1 -1 0 -1 -1\n\
ctl 26 67 -1 -1 0 -1 -1\n\
ctl 27 44 -1 -1 128 -1 -1\n\
ctl 28 -1 -1 -1 -1 -1 -1\n\
ctl 29 -1 -1 -1 -1 -1 -1\n\
isctl 2=1 3=1 4=1 5=1 6=1 7=1 8=1 9=1 15=1 16=1 17=1 18=1 19=1 20=1 29=1 30=1 31=1 32=1 33=1 42=1 44=1 47=1 56=1 57=1 58=1 63=1 67=1 255=1 256=1 257=1 258=1\n\
custom 2 3 0 1\n\
swap 20 17 17 20\n\
alt 3 259 1 pad 4096 32768 4096 4096 32768\n\
dis 1 1 0 1 1\n\
2: 17 256\n\
en 1 1 0 0 0\n\
0:\n\
tapctl 0\n\
f 1 k 22 0 0 0 0 0 0\n\
f 1 k 23 0 0 0 0 0 0\n\
f 1 k 24 0 0 0 0 0 1\n\
f 1 k 25 0 0 0 0 0 1\n\
f 1 k 259 0 0 0 0 0 0\n\
f 1 counts 0 -1 0 -1 0\n\
0:\n\
f 2 k 22 1 0 1 1 1 0\n\
f 2 k 23 1 0 1 1 1 0\n\
f 2 k 24 0 0 0 0 0 1\n\
f 2 k 25 0 0 0 0 0 1\n\
f 2 k 259 1 0 1 1 1 0\n\
f 2 counts 2 22 1 259 0\n\
3: 22 23 259\n\
f 3 k 22 1 0 1 1 1 0\n\
f 3 k 23 0 0 0 0 0 0\n\
f 3 k 24 0 0 0 0 0 1\n\
f 3 k 25 0 0 0 0 0 1\n\
f 3 k 259 1 0 1 1 1 0\n\
f 3 counts 1 22 1 259 0\n\
2: 22 259\n\
f 4 k 22 1 0 1 1 1 0\n\
f 4 k 23 0 0 0 0 0 0\n\
f 4 k 24 0 0 0 0 0 1\n\
f 4 k 25 1 0 1 1 1 0\n\
f 4 k 259 1 0 1 1 1 0\n\
f 4 counts 2 22 1 259 0\n\
3: 22 25 259\n\
f 5 k 22 0 0 0 0 0 0\n\
f 5 k 23 0 0 0 0 0 0\n\
f 5 k 24 0 0 0 0 0 0\n\
f 5 k 25 0 0 0 0 0 0\n\
f 5 k 259 0 0 0 0 0 0\n\
f 5 counts 0 -1 0 -1 0\n\
0:\n\
f 6 k 22 0 0 0 0 0 0\n\
f 6 k 23 0 0 0 0 0 0\n\
f 6 k 24 0 0 0 0 0 0\n\
f 6 k 25 0 0 0 0 0 0\n\
f 6 k 259 0 0 0 0 0 0\n\
f 6 counts 0 -1 0 -1 0\n\
0:";

/// The same lines from the port's commands, on the same bindings and the same frames.
#[test]
fn the_real_games_frame_by_frame_run_is_reproduced() {
    let want: Vec<&str> = REAL_O_INPUT.lines().collect();
    let got = o_input_lines();
    assert_eq!(got.len(), want.len());
    for (i, (g, w)) in got.iter().zip(&want).enumerate() {
        assert_eq!(g, w, "line {i}");
    }
}
