//! `BSDevices/xcontrols.obj` (Xbox PDB source unit), subsystem `BSDevices`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The game's `Controls` object reading its devices each frame. Its device calls go through the
//! `IDirectInputDevice8A` vtables (the platform's DirectInput 8 layer,
//! `units::platform::dinput`) and `XInputGetState`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::x87_fmul;

/// `Controls` fields (offsets as `00a23010` uses them): the joystick devices (`+0xc`, one word
/// each), the joystick states (`+0x34`, 0xa0 bytes apart: this frame's 0x50 bytes, the last
/// frame's at `+0x50` after it), the keyboard (`+0x2c`) and mouse (`+0x30`) devices, the joystick
/// count (`+0x18f4`), the keyboard state (`+0x18f8`, the last frame's at `+0x19f8`), the mouse
/// state (`+0x1b24`, the last frame's at `+0x1b38`) and the 8 bytes cleared each poll at
/// `+0x1b58`.
const JOYSTICK_DEVICES: u32 = 0xc;
const JOYSTICK_STATES: u32 = 0x34;
const JOYSTICK_STRIDE: u32 = 0xa0;
const JOYSTICK_STATE_SIZE: u32 = 0x50;
const KEYBOARD: u32 = 0x2c;
const MOUSE: u32 = 0x30;
const JOYSTICK_COUNT: u32 = 0x18f4;
const KEYBOARD_STATE: u32 = 0x18f8;
const KEYBOARD_LAST: u32 = 0x19f8;
const KEYBOARD_STATE_SIZE: u32 = 0x100;
const MOUSE_STATE: u32 = 0x1b24;
const MOUSE_LAST: u32 = 0x1b38;
const MOUSE_STATE_SIZE: u32 = 0x14;
const MOUSE_CLEARED: u32 = 0x1b58;
/// The pad state `XInputGetState` fills (`011f35a8`, 16 bytes), last frame's copy (`011f35b8`)
/// and the "pad connected" byte (`011f35c8`).
const PAD_STATE: u32 = 0x011f_35a8;
const PAD_LAST: u32 = 0x011f_35b8;
const PAD_CONNECTED: u32 = 0x011f_35c8;
/// The `XInputGetState` thunk (`009f996e`), `memcpy` and `memset` (CRT, `cdecl`).
const XINPUT_GET_STATE: u32 = 0x009f_996e;
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;
/// `IDirectInputDevice8A` slots: `Acquire`, `GetDeviceState`, `Poll`.
const ACQUIRE: u32 = 0x1c;
const GET_DEVICE_STATE: u32 = 0x24;
const POLL: u32 = 0x64;
/// The mouse reader's end (`00a232c0`).
const AFTER_MOUSE: u32 = 0x00a2_32c0;

/// One device: acquire it; on `DI_OK` or `S_FALSE` (already acquired) poll it and read its
/// state into `state` (`size` bytes); otherwise clear `state`.
fn read_device(e: &mut Engine, device: u32, state: u32, size: u32) {
    let result = e.vcall(device, ACQUIRE, &args![]).u32();
    if result == 0 || result == 1 {
        e.vcall(device, POLL, &args![]);
        e.vcall(device, GET_DEVICE_STATE, &args![size, state]);
    } else {
        e.call(MEMSET, &args![state, 0u32, size]);
    }
}

// Translated from 00a23010 (disassembled, FalloutNV.exe 1.4.0.525)
/// `Controls::Poll` (engine map; `thiscall`, no stack words, plain `RET`): the pad state is kept
/// as last frame's (`011f35a8` -> `011f35b8`, four words) and read again
/// (`XInputGetState(0, 011f35a8)`); the pad counts as connected (`011f35c8` = 1) when that
/// answers 0 and the byte at `+0` is clear, else 0. Then, for each joystick (the count compared
/// signed), for the keyboard and for the mouse (when there is one): this frame's state becomes
/// last frame's (`memcpy`), the device is acquired and, on `DI_OK` / `S_FALSE`, polled and read
/// (`GetDeviceState`: 0x50, 0x100, 0x14 bytes), else its state is cleared; the mouse also
/// clears the 8 bytes at `+0x1b58` before and ends with `00a232c0`.
pub fn controls_poll(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for i in 0..4 {
        let word = e.mem.u32(PAD_STATE + 4 * i);
        e.mem.set_u32(PAD_LAST + 4 * i, word);
    }
    let result = e.call(XINPUT_GET_STATE, &args![0u32, PAD_STATE]).u32();
    let connected = result == 0 && e.mem.u8(base) == 0;
    e.mem.set_u8(PAD_CONNECTED, u8::from(connected));
    let mut i: i32 = 0;
    while i < e.mem.i32(base + JOYSTICK_COUNT) {
        let at = base + (i as u32).wrapping_mul(JOYSTICK_STRIDE);
        e.call(
            MEMCPY,
            &args![
                at + JOYSTICK_STATES + JOYSTICK_STATE_SIZE,
                at + JOYSTICK_STATES,
                JOYSTICK_STATE_SIZE
            ],
        );
        let device = e.mem.u32(base + JOYSTICK_DEVICES + 4 * i as u32);
        read_device(e, device, at + JOYSTICK_STATES, JOYSTICK_STATE_SIZE);
        i += 1;
    }
    if e.mem.u32(base + KEYBOARD) != 0 {
        e.call(
            MEMCPY,
            &args![
                base + KEYBOARD_LAST,
                base + KEYBOARD_STATE,
                KEYBOARD_STATE_SIZE
            ],
        );
        let keyboard = e.mem.u32(base + KEYBOARD);
        read_device(e, keyboard, base + KEYBOARD_STATE, KEYBOARD_STATE_SIZE);
    }
    if e.mem.u32(base + MOUSE) != 0 {
        e.call(
            MEMCPY,
            &args![base + MOUSE_LAST, base + MOUSE_STATE, MOUSE_STATE_SIZE],
        );
        e.call(MEMSET, &args![base + MOUSE_CLEARED, 0u32, 8u32]);
        let mouse = e.mem.u32(base + MOUSE);
        read_device(e, mouse, base + MOUSE_STATE, MOUSE_STATE_SIZE);
        e.call(AFTER_MOUSE, &args![this]);
    }
}

// Translated from 00a232c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`): does nothing (it keeps
/// `this` in a local and returns).
pub fn fn_00a232c0(_e: &mut Engine, _this: Ptr) {}

/// The per-action clear `Controls::ClearUserActions` calls (`00a25410`, `thiscall`, one stack
/// word: the action index) and the action count it runs over.
const CLEAR_USER_ACTION: u32 = 0x00a2_5410;
const USER_ACTION_COUNT: i32 = 0x1c;

// Translated from 00a253d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `Controls::ClearUserActions` (engine map; `thiscall`, no stack words, plain `RET`): `00a25410`
/// for each action index 0 to 0x1b (the loop compares signed, `JGE`).
pub fn controls_clear_user_actions(e: &mut Engine, this: Ptr) {
    let mut i: i32 = 0;
    while i < USER_ACTION_COUNT {
        e.call(CLEAR_USER_ACTION, &args![this, i as u32]);
        i += 1;
    }
}

/// `Controls` fields of the action clear and the rumble: the button bound to each action per
/// device (`+0x1b94`, 0x1c bytes a device, one byte an action, 0xff for none), the
/// keyboard's current bytes (`+0x18f8`), the mouse's buttons (`+0x1b30`), the joystick's
/// buttons (`+0x64`) and words (`+0x54`); the rumble's "enabled" byte (`+0x1b88`) and its
/// queue (`+0x1b80`).
const ACTION_BINDINGS: u32 = 0x1b94;
const ACTION_STRIDE: u32 = 0x1c;
const MOUSE_BUTTONS: u32 = 0x1b30;
const JOYSTICK_BUTTONS: u32 = 0x64;
const JOYSTICK_WORDS: u32 = 0x54;
const RUMBLE_ENABLED: u32 = 0x1b88;
const RUMBLE_QUEUE: u32 = 0x1b80;
/// The pad bytes and words cleared after an action clear (`011f35ac` .. `011f35b7`; part of the
/// pad state at `011f35a8`).
const PAD_LEFT_TRIGGER: u32 = 0x011f_35ae;
const PAD_RIGHT_TRIGGER: u32 = 0x011f_35af;
const PAD_THUMB_WORDS: [u32; 4] = [0x011f_35b0, 0x011f_35b2, 0x011f_35b4, 0x011f_35b6];
const PAD_BUTTONS: u32 = 0x011f_35ac;
/// The memory manager's allocation (`00aa3e40`, `thiscall` on `011f6238`, one word: the size),
/// the queue's add (`00a26310`, `thiscall`, two words: a channel and the effect), the
/// `XInputSetState` thunk (`00ee820a`, two words: the pad and the vibration) and the double
/// the strengths are scaled by (`010217d0`, read from the executable's data).
const ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const QUEUE_ADD: u32 = 0x00a2_6310;
const XINPUT_SET_STATE: u32 = 0x00ee_820a;
const STRENGTH_SCALE: u32 = 0x0102_17d0;
/// The import slot of KERNEL32 `WaitForSingleObject`.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;

// Translated from 00a25410 (disassembled, FalloutNV.exe 1.4.0.525)
/// The clear of one user action (no label; `thiscall`, one word: the action index, `RET 4`).
/// For each device 0 to 3 (signed loop) the button bound to the action (the byte at `+0x1b94 +
/// device * 0x1c + action`) is cleared unless it is 0xff: device 0 the keyboard byte
/// (`+0x18f8 + button`), device 1 the mouse byte (`+0x1b30 + button`), device 2 the joystick
/// byte (`+0x64 + button`) for a button below 0x20, else the joystick word at `+0x54 + ((button
/// >> 16) & 0xffff) * 4` (the byte shifted, so `+0x54`); device 3 nothing. Then the pad's two
/// trigger bytes and five words (`011f35ac` .. `011f35b7`) are cleared.
pub fn controls_clear_user_action(e: &mut Engine, this: Ptr, action: u32) {
    let base = this.addr();
    let mut device: i32 = 0;
    while device < 4 {
        let at = (device as u32)
            .wrapping_mul(ACTION_STRIDE)
            .wrapping_add(action)
            .wrapping_add(base)
            .wrapping_add(ACTION_BINDINGS);
        let button = u32::from(e.mem.u8(at));
        if button != 0xff {
            match device {
                0 => e
                    .mem
                    .set_u8(base.wrapping_add(button).wrapping_add(KEYBOARD_STATE), 0),
                1 => e
                    .mem
                    .set_u8(base.wrapping_add(button).wrapping_add(MOUSE_BUTTONS), 0),
                2 => {
                    if (button as i32) < 0x20 {
                        e.mem
                            .set_u8(base.wrapping_add(button).wrapping_add(JOYSTICK_BUTTONS), 0);
                    } else {
                        let word = (button >> 16) & 0xffff;
                        e.mem
                            .set_u32(base.wrapping_add(word * 4).wrapping_add(JOYSTICK_WORDS), 0);
                    }
                }
                _ => {}
            }
        }
        device += 1;
    }
    e.mem.set_u8(PAD_LEFT_TRIGGER, 0);
    e.mem.set_u8(PAD_RIGHT_TRIGGER, 0);
    for word in PAD_THUMB_WORDS {
        e.mem.set_u16(word, 0);
    }
    e.mem.set_u16(PAD_BUTTONS, 0);
}

/// `FISTP dword` inside the compiler's truncating sequence: the value rounded toward zero, the
/// integer indefinite 0x80000000 for a NaN or a value outside the 32-bit range.
fn fistp32_chop(x: X87Ext) -> u32 {
    let wide = x87_fistp64_chop(x);
    if wide < i64::from(i32::MIN) || wide > i64::from(i32::MAX) {
        0x8000_0000
    } else {
        wide as u32
    }
}

/// One motor's strength for `XInputSetState`: 0xffff when the strength truncated to an
/// integer has a low half word above 1 (`MOVZX`, `CMP 1`, `JLE`), otherwise the low half word of
/// `strength * [010217d0]` truncated.
fn motor_speed(strength: X87Ext, scale: X87Ext) -> u16 {
    if (fistp32_chop(strength) & 0xffff) as i32 > 1 {
        0xffff
    } else {
        (fistp32_chop(x87_fmul(strength, scale)) & 0xffff) as u16
    }
}

// Translated from 00a255b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `Controls::Rumble` (engine map; `thiscall`, seven words: the left and right strengths as
/// floats, three words, a byte and a "forced" byte; `RET 0x1c`). Nothing when the rumble byte
/// (`+0x1b88`) is clear, or when not forced and the queue (`+0x1b80`) has entries (its +0xc
/// plus its +0x20 not 0). Otherwise a 0x24-byte effect is allocated (`00aa3e40` on the memory
/// manager) and zeroed (`memset`; a null block is used as address 0, as the code does), its
/// +8, +0xc, +4 set from the three words, its +0x10 byte from the byte, +0x14 and +0x1c
/// cleared; its first word is the right strength times `[010217d0]` truncated (`FISTP qword`,
/// low word) and it is added to the queue on channel 1 (`00a26310`), then the same with the
/// left strength on channel 0. Last `XInputSetState(0, vibration)` with the two motor speeds
/// (left, then right; [`motor_speed`]).
#[allow(clippy::too_many_arguments)]
pub fn controls_rumble(
    e: &mut Engine,
    this: Ptr,
    left: u32,
    right: u32,
    word_8: u32,
    word_c: u32,
    word_4: u32,
    byte_10: u32,
    forced: u32,
) {
    let base = this.addr();
    if e.mem.u8(base + RUMBLE_ENABLED) == 0 {
        return;
    }
    if forced as u8 == 0 {
        let queue = e.mem.u32(base + RUMBLE_QUEUE);
        let pending = e
            .mem
            .u32(queue.wrapping_add(0xc))
            .wrapping_add(e.mem.u32(queue.wrapping_add(0x20)));
        if pending != 0 {
            return;
        }
    }
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x24u32]).u32();
    let effect = if block != 0 {
        e.call(MEMSET, &args![block, 0u32, 0x24u32]);
        block
    } else {
        0
    };
    e.mem.set_u32(effect.wrapping_add(8), word_8);
    e.mem.set_u32(effect.wrapping_add(0xc), word_c);
    e.mem.set_u32(effect.wrapping_add(4), word_4);
    e.mem.set_u8(effect.wrapping_add(0x10), byte_10 as u8);
    e.mem.set_u32(effect.wrapping_add(0x14), 0);
    e.mem.set_u32(effect.wrapping_add(0x1c), 0);
    let scale = X87Ext::load64(f64::from_bits(e.mem.u64(STRENGTH_SCALE)));
    let right = X87Ext::load32(right);
    let left = X87Ext::load32(left);
    let level = x87_fistp64_chop(x87_fmul(right, scale)) as u32;
    e.mem.set_u32(effect, level);
    let queue = e.mem.u32(base + RUMBLE_QUEUE);
    e.call(QUEUE_ADD, &args![queue, 1u32, effect]);
    let level = x87_fistp64_chop(x87_fmul(left, scale)) as u32;
    e.mem.set_u32(effect, level);
    let queue = e.mem.u32(base + RUMBLE_QUEUE);
    e.call(QUEUE_ADD, &args![queue, 0u32, effect]);
    let left_speed = motor_speed(left, scale);
    let right_speed = motor_speed(right, scale);
    e.with_stack(4, |e, vibration| {
        e.mem.set_u16(vibration.addr(), left_speed);
        e.mem.set_u16(vibration.addr() + 2, right_speed);
        e.call(XINPUT_SET_STATE, &args![0u32, vibration.addr()]);
    });
}

// Translated from 00ec1800 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the code (the engine map's `Controls::SetCurrentController` label; `thiscall`,
/// no stack words, plain `RET`): with a handle at `+8`, the byte at `+0x20` is set, the handle
/// is waited for (`WaitForSingleObject(handle, INFINITE)`), the handle cleared and the byte
/// cleared again.
pub fn wait_for_handle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 8) == 0 {
        return;
    }
    e.mem.set_u8(base + 0x20, 1);
    let handle = e.mem.u32(base + 8);
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0x20, 0);
}

/// `00a24080` (`thiscall`, one word; answers a mask in `EAX`) and the 16-bit words at +4 of the
/// pad state (`011f35ac`) and of last frame's copy (`011f35bc`).
const CALLEE_00A24080: u32 = 0x00a2_4080;
const PAD_WORD: u32 = PAD_STATE + 4;
const PAD_LAST_WORD: u32 = PAD_LAST + 4;

// Translated from 00a25560 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`xcontrols.obj`; `thiscall`, one word, `RET 4`): the mask
/// `00a24080(word)` answers picks which bits of last frame's pad word (`011f35bc`) take the
/// current pad word's (`011f35ac`): `last = (last & !mask) | (current & mask)`, in 16 bits.
pub fn fn_00a25560(e: &mut Engine, this: Ptr, word: u32) {
    let mask = e.call(CALLEE_00A24080, &args![this, word]).u32();
    let kept = u32::from(e.mem.u16(PAD_LAST_WORD)) & !mask;
    e.mem.set_u16(PAD_LAST_WORD, kept as u16);
    let taken = u32::from(e.mem.u16(PAD_WORD)) & mask;
    let last = u32::from(e.mem.u16(PAD_LAST_WORD)) | taken;
    e.mem.set_u16(PAD_LAST_WORD, last as u16);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_5560, fn_00a25560(Ptr, u32)),
        entry!(0x00a2_3010, controls_poll(Ptr)),
        entry!(0x00a2_32c0, fn_00a232c0(Ptr)),
        entry!(0x00a2_53d0, controls_clear_user_actions(Ptr)),
        entry!(0x00a2_5410, controls_clear_user_action(Ptr, u32)),
        entry!(
            0x00a2_55b0,
            controls_rumble(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00ec_1800, wait_for_handle(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A device double at `object` whose `Acquire` answers `acquire`; its slots record calls.
    fn device(e: &mut Engine, object: u32, table: u32, acquire: u32, log: &Log) {
        let mut slots = vec![0u32; 26];
        for (slot, code) in [
            (7usize, 0x00ab_0000u32),
            (9, 0x00ab_0004),
            (25, 0x00ab_0008),
        ] {
            slots[slot] = code + (table & 0xff00);
        }
        e.put_vtable(table, &slots);
        e.mem.set_u32(object, table);
        for (k, code) in slots.iter().enumerate().filter(|(_, c)| **c != 0) {
            let log = log.clone();
            let code = *code;
            e.register_double(code, move |_, a| {
                log.borrow_mut().push((code, a.to_vec()));
                crate::abi::RetVal::into_ret(if k == 7 { acquire } else { 0u32 })
            });
        }
    }

    #[test]
    fn poll_reads_each_device_and_keeps_last_frames_state() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.map(0x0010_0000, 0x4000);
        let log: Log = std::rc::Rc::default();
        let this = 0x0010_0000u32;
        // A keyboard that acquires, a mouse that doesn't (cleared), no joystick.
        device(&mut e, 0x0010_3000, 0x0300_0100, 0, &log);
        device(&mut e, 0x0010_3010, 0x0300_0200, 0x8007_000c, &log);
        e.mem.set_u32(this + KEYBOARD, 0x0010_3000);
        e.mem.set_u32(this + MOUSE, 0x0010_3010);
        e.mem.set_u32(PAD_STATE, 0x55);
        for (f, ret) in [
            (XINPUT_GET_STATE, 0x48fu32),
            (MEMCPY, 0),
            (MEMSET, 0),
            (AFTER_MOUSE, 0),
        ] {
            let log = log.clone();
            e.register_double(f, move |_, a| {
                log.borrow_mut().push((f, a.to_vec()));
                crate::abi::RetVal::into_ret(ret)
            });
        }
        e.call(0x00a2_3010, &args![this]);
        let calls: Vec<(u32, Vec<u32>)> = log.borrow().clone();
        assert_eq!(e.mem.u32(PAD_LAST), 0x55);
        assert_eq!(e.mem.u8(PAD_CONNECTED), 0);
        assert_eq!(
            calls,
            vec![
                (XINPUT_GET_STATE, vec![0, PAD_STATE]),
                (
                    MEMCPY,
                    vec![this + KEYBOARD_LAST, this + KEYBOARD_STATE, 0x100]
                ),
                (0x00ab_0100, vec![0x0010_3000]),
                (0x00ab_0108, vec![0x0010_3000]),
                (0x00ab_0104, vec![0x0010_3000, 0x100, this + KEYBOARD_STATE]),
                (MEMCPY, vec![this + MOUSE_LAST, this + MOUSE_STATE, 0x14]),
                (MEMSET, vec![this + MOUSE_CLEARED, 0, 8]),
                (0x00ab_0200, vec![0x0010_3010]),
                (MEMSET, vec![this + MOUSE_STATE, 0, 0x14]),
                (AFTER_MOUSE, vec![this]),
            ]
        );
    }

    #[test]
    fn clear_user_actions_clears_each_of_the_0x1c_actions_in_order() {
        let mut e = Engine::new();
        e.register(CLEAR_USER_ACTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_53d0, &args![0x1000u32]);
        let log = e.call_log.as_ref().unwrap();
        let cleared: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == CLEAR_USER_ACTION)
            .map(|c| c.1.clone())
            .collect();
        let want: Vec<Vec<u32>> = (0..0x1c).map(|i| vec![0x1000, i]).collect();
        assert_eq!(cleared, want);
    }
}

#[cfg(test)]
mod tests_actions_rumble {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .cloned()
            .collect()
    }

    /// A `Controls` object of 0x2000 bytes at `0x0800_0000` with the pad state mapped.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let this = 0x0800_0000;
        e.map(this, 0x2000);
        e.map(PAD_STATE, 0x20);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn clear_user_action_clears_each_devices_binding() {
        let (mut e, this) = engine();
        e.mem.write(this, &[0xaa; 0x2000]);
        e.mem.write(PAD_STATE, &[0xaa; 0x10]);
        // Action 3: keyboard 0x10, mouse 2, joystick 5, device 3 bound to 7 (ignored).
        let action = 3;
        for (device, button) in [(0u32, 0x10u8), (1, 2), (2, 5), (3, 7)] {
            e.mem.set_u8(
                this + ACTION_BINDINGS + device * ACTION_STRIDE + action,
                button,
            );
        }
        controls_clear_user_action(&mut e, Ptr::new(this), action);
        assert_eq!(e.mem.u8(this + KEYBOARD_STATE + 0x10), 0);
        assert_eq!(e.mem.u8(this + MOUSE_BUTTONS + 2), 0);
        assert_eq!(e.mem.u8(this + JOYSTICK_BUTTONS + 5), 0);
        assert_eq!(e.mem.u8(this + KEYBOARD_STATE + 0x11), 0xaa);
        assert_eq!(e.mem.u32(this + JOYSTICK_WORDS), 0xaaaa_aaaa);
        // The pad bytes and words from 011f35ac to 011f35b7 are cleared, the rest kept.
        assert_eq!(e.mem.bytes(PAD_STATE + 4, 12), vec![0; 12]);
        assert_eq!(e.mem.bytes(PAD_STATE, 4), vec![0xaa; 4]);
        // A joystick button at or above 0x20 clears the word at +0x54; 0xff is skipped.
        e.mem.write(this, &[0xaa; 0x2000]);
        e.mem
            .set_u8(this + ACTION_BINDINGS + 2 * ACTION_STRIDE + action, 0x20);
        controls_clear_user_action(&mut e, Ptr::new(this), action);
        assert_eq!(e.mem.u32(this + JOYSTICK_WORDS), 0);
        assert_eq!(e.mem.u8(this + JOYSTICK_BUTTONS + 0x20), 0xaa);
        assert_eq!(e.mem.u8(this + KEYBOARD_STATE + 0xaa), 0);
        e.mem.write(this, &[0xff; 0x2000]);
        controls_clear_user_action(&mut e, Ptr::new(this), action);
        assert_eq!(e.mem.bytes(this, 0x2000), vec![0xff; 0x2000]);
    }

    /// The rumble with the queue at `0x0900_0000`, the scale 65535.0, the allocation answering
    /// `0x0a00_0000` and the queue add recording the effect's first word.
    fn rumble_engine() -> (Engine, u32, std::rc::Rc<std::cell::RefCell<Vec<u32>>>) {
        let (mut e, this) = engine();
        e.mem.set_u8(this + RUMBLE_ENABLED, 1);
        let queue = 0x0900_0000;
        e.map(queue, 0x100);
        e.mem.set_u32(this + RUMBLE_QUEUE, queue);
        e.map(STRENGTH_SCALE, 8);
        e.mem.set_u64(STRENGTH_SCALE, 65535.0f64.to_bits());
        let block = 0x0a00_0000;
        e.map(block, 0x100);
        e.mem.write(block, &[0xcc; 0x24]);
        e.register(ALLOCATE, |_, _| 0x0a00_0000u32.into_ret());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        let levels = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = levels.clone();
        e.register_double(QUEUE_ADD, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            Ret::default()
        });
        e.register(XINPUT_SET_STATE, |_, _| Ret::default());
        (e, this, levels)
    }

    #[test]
    fn rumble_queues_both_motors_and_sets_the_pad() {
        let (mut e, this, levels) = rumble_engine();
        let left = 0.5f32.to_bits();
        let right = 0.25f32.to_bits();
        e.register_double(XINPUT_SET_STATE, |e, a| {
            assert_eq!(a[0], 0);
            assert_eq!((e.mem.u16(a[1]), e.mem.u16(a[1] + 2)), (32767, 16383));
            Ret::default()
        });
        controls_rumble(&mut e, Ptr::new(this), left, right, 8, 0xc, 4, 0x1ff, 0);
        // Right first on channel 1, then left on channel 0, each scaled and truncated.
        let queue = 0x0900_0000;
        assert_eq!(
            calls(&e, QUEUE_ADD),
            vec![
                (QUEUE_ADD, vec![queue, 1, 0x0a00_0000]),
                (QUEUE_ADD, vec![queue, 0, 0x0a00_0000])
            ]
        );
        assert_eq!(*levels.borrow(), vec![16383, 32767]);
        let effect = 0x0a00_0000;
        assert_eq!(
            (
                e.mem.u32(effect + 4),
                e.mem.u32(effect + 8),
                e.mem.u32(effect + 0xc)
            ),
            (4, 8, 0xc)
        );
        assert_eq!(e.mem.u8(effect + 0x10), 0xff);
        assert_eq!(
            calls(&e, ALLOCATE),
            vec![(ALLOCATE, vec![MEMORY_MANAGER, 0x24])]
        );
        assert_eq!(calls(&e, XINPUT_SET_STATE).len(), 1);
    }

    #[test]
    fn rumble_strengths_above_one_run_the_motor_full() {
        let (mut e, this, _) = rumble_engine();
        e.register_double(XINPUT_SET_STATE, |e, a| {
            assert_eq!((e.mem.u16(a[1]), e.mem.u16(a[1] + 2)), (0xffff, 0));
            Ret::default()
        });
        controls_rumble(
            &mut e,
            Ptr::new(this),
            2.5f32.to_bits(),
            0.0f32.to_bits(),
            0,
            0,
            0,
            0,
            0,
        );
        assert_eq!(calls(&e, XINPUT_SET_STATE).len(), 1);
    }

    #[test]
    fn rumble_waits_for_the_queue_unless_forced_and_needs_the_switch() {
        let (mut e, this, _) = rumble_engine();
        let queue = 0x0900_0000;
        e.mem.set_u32(queue + 0x20, 1);
        controls_rumble(&mut e, Ptr::new(this), 0, 0, 0, 0, 0, 0, 0x100);
        assert!(calls(&e, ALLOCATE).is_empty());
        controls_rumble(&mut e, Ptr::new(this), 0, 0, 0, 0, 0, 0, 1);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
        e.mem.set_u8(this + RUMBLE_ENABLED, 0);
        controls_rumble(&mut e, Ptr::new(this), 0, 0, 0, 0, 0, 0, 1);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn fistp32_chop_truncates_and_flags_out_of_range() {
        assert_eq!(fistp32_chop(X87Ext::load64(2.9)), 2);
        assert_eq!(fistp32_chop(X87Ext::load64(-2.9)), (-2i32) as u32);
        assert_eq!(fistp32_chop(X87Ext::load64(3e9)), 0x8000_0000);
        assert_eq!(fistp32_chop(X87Ext::load64(f64::NAN)), 0x8000_0000);
    }

    #[test]
    fn wait_for_handle_waits_and_clears() {
        let (mut e, this) = engine();
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| 0u32.into_ret());
        wait_for_handle(&mut e, Ptr::new(this));
        assert!(calls(&e, WAIT_FOR_SINGLE_OBJECT).is_empty());
        e.mem.set_u32(this + 8, 0x44);
        e.mem.set_u8(this + 0x20, 7);
        wait_for_handle(&mut e, Ptr::new(this));
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![(WAIT_FOR_SINGLE_OBJECT, vec![0x44, 0xffff_ffff])]
        );
        assert_eq!((e.mem.u32(this + 8), e.mem.u8(this + 0x20)), (0, 0));
    }
}

#[cfg(test)]
mod pad_word_tests {
    use super::*;

    #[test]
    fn the_mask_picks_the_bits_last_frame_takes() {
        let mut e = Engine::new();
        e.map(PAD_STATE, 0x20);
        e.mem.set_u32(PAD_WORD, 0xffff_00f0);
        e.mem.set_u32(PAD_LAST_WORD, 0xeeee_0f0f);
        e.register(CALLEE_00A24080, |_, a| {
            assert_eq!(a[1], 3);
            0xffff_00ffu32.into_ret()
        });
        e.call(0x00a2_5560, &args![0x100u32, 3u32]);
        // Low byte from the current word, the rest kept; the word above untouched.
        assert_eq!(e.mem.u32(PAD_LAST_WORD), 0xeeee_0ff0);
    }
}
