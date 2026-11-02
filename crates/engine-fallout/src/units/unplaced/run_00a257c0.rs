//! `(unplaced)/run_00a257c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Functions of the PC input object (built by `00a22660`): two that work on
//! the 0x40-byte object it keeps at +0x1B80 (whose constructor and set-up
//! method are here too), the 0x14280-byte singleton at `011f3634` and its
//! constructor. None has a name in the engine map; they are described by
//! what the code does.

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global that must be non-zero for `00a257c0` to act.
const ENABLED_FLAG: u32 = 0x011f_35c8;
/// The double (1000.0 in the exe) `00a257c0` scales its argument by.
const SCALE: u32 = 0x0101_7b70;
/// `_ftol2` (CRT; the value in x87 `ST0`, a leading `f64` argument).
const FTOL2: u32 = 0x00ec_62c0;
/// Method `00a269f0` of the object at +0x1B80 (thiscall, one word).
const OBJECT_1B80_START: u32 = 0x00a2_69f0;
/// The `XInputSetState` thunk (`JMP [00fdf398]`; stdcall: user index,
/// state).
const XINPUT_SET_STATE: u32 = 0x00ee_820a;
/// `GetTickCount` (import slot).
const IMPORT_GET_TICK_COUNT: u32 = 0x00fd_f060;
/// The singleton `00a25920` builds on first use.
const SINGLETON: u32 = 0x011f_3634;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00a257c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the byte at `011f35c8` and the input object's byte +0x1B88 are both
/// set, passes `value * 1000.0` (the double at `01017b70`, `FMUL`, then
/// `_ftol2`) to `00a269f0` of the object at +0x1B80.
pub fn fn_00a257c0(e: &mut Engine, this: u32, value: f32) {
    if e.mem.u8(ENABLED_FLAG) == 0 {
        return;
    }
    if e.mem.u8(this + 0x1b88) == 0 {
        return;
    }
    let scaled = x87_mul(x87_load32(value.to_bits()), e.mem.f64(SCALE));
    let whole = e.call(FTOL2, &args![scaled]).u32();
    let object = e.mem.u32(this + 0x1b80);
    e.call(OBJECT_1B80_START, &[object, whole]);
}

// Translated from 00a25810 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the input object's byte +0x1B88 is set: clears the words +0x0C and
/// +0x20 of the object at +0x1B80 and sends user 0 a state of two zero
/// 16-bit values (`XInputSetState(0, &state)`, the state on the stack).
pub fn fn_00a25810(e: &mut Engine, this: u32) {
    if e.mem.u8(this + 0x1b88) == 0 {
        return;
    }
    e.with_stack(4, |e, state| {
        e.mem.set_u16(state.addr(), 0);
        e.mem.set_u16(state.addr() + 2, 0);
        let object = e.mem.u32(this + 0x1b80);
        e.mem.set_u32(object + 0xc, 0);
        e.mem.set_u32(object + 0x20, 0);
        e.call(XINPUT_SET_STATE, &[0, state.addr()]);
    });
}

// Translated from 00a25920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The singleton at `011f3634`: while it is null, allocates 0x14280 bytes
/// and constructs them with [`fn_00a259b0`] (null when the allocation
/// fails). Returns the singleton.
// Translation incomplete: 00a25920 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00a25920(e: &mut Engine) -> u32 {
    if e.mem.u32(SINGLETON) == 0 {
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x14280]).u32();
        let object = if block != 0 { fn_00a259b0(e, block) } else { 0 };
        e.mem.set_u32(SINGLETON, object);
    }
    e.mem.u32(SINGLETON)
}

// Translated from 00a259b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x14280-byte singleton: clears the two words at
/// +0x3AC, the bytes +0x00..+0x03, the words +0x68, +0x50, +0x6C, +0x70,
/// +0x64, the bytes +0x04 and +0x24, the 0x1C bytes at +0x08, the 0x13EC0
/// bytes at +0x3B8 and the 0x320 bytes at +0x8C, and the words +0x14278
/// and +0x3B4; sets +0x74 and +0x78 to 1, +0x7C and +0x80 to -1 and the
/// bytes +0x84..+0x88 to 1, 1, 0, 1, 0. Returns `this`.
pub fn fn_00a259b0(e: &mut Engine, this: u32) -> u32 {
    let pair = this + 0x3ac;
    e.mem.set_u32(pair, 0);
    e.mem.set_u32(pair + 4, 0);
    for index in 0..4 {
        e.mem.set_u8(this + index, 0);
    }
    e.mem.set_u32(this + 0x68, 0);
    e.mem.set_u8(this + 4, 0);
    e.call(MEMSET, &[this + 8, 0, 0x1c]);
    e.mem.set_u8(this + 0x24, 0);
    e.mem.set_u32(this + 0x50, 0);
    e.mem.set_u32(this + 0x6c, 0);
    e.mem.set_u32(this + 0x70, 0);
    e.mem.set_u32(this + 0x64, 0);
    e.mem.set_u32(this + 0x74, 1);
    e.mem.set_u32(this + 0x78, 1);
    e.mem.set_u32(this + 0x80, u32::MAX);
    e.mem.set_u32(this + 0x7c, u32::MAX);
    e.mem.set_u8(this + 0x84, 1);
    e.mem.set_u8(this + 0x85, 1);
    e.mem.set_u8(this + 0x86, 0);
    e.mem.set_u8(this + 0x87, 1);
    e.mem.set_u8(this + 0x88, 0);
    e.call(MEMSET, &[this + 0x3b8, 0, 0x13ec0]);
    e.call(MEMSET, &[this + 0x8c, 0, 0x320]);
    e.mem.set_u32(this + 0x14278, 0);
    e.mem.set_u32(this + 0x3b4, 0);
    this
}

// Translated from 00a262d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x40-byte object the input object keeps at +0x1B80:
/// stores `owner` in its first word and `GetTickCount()` at +0x38. Returns
/// `this`.
pub fn fn_00a262d0(e: &mut Engine, this: u32, owner: u32) -> u32 {
    e.mem.set_u32(this, owner);
    let ticks = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
    e.mem.set_u32(this + 0x38, ticks);
    this
}

// Translated from 00a26310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches `block` to one of the object's two channels (`which` 0: words
/// +0x04 and +0x0C..+0x1C; otherwise +0x08 and +0x20..+0x30): clears the
/// channel's last two words, sets the block's +0x0C to its +0x08 when it is
/// 0, keeps the block and copies its +0x08, +0x14 and +0x1C, and copies the
/// block's first 16-bit value to its +0x1A and +0x20.
pub fn fn_00a26310(e: &mut Engine, this: u32, which: u32, block: u32) {
    let (cleared, kept, copies) = if which == 0 {
        ([0x18, 0x1c], 4, [0xc, 0x10, 0x14])
    } else {
        ([0x2c, 0x30], 8, [0x20, 0x24, 0x28])
    };
    for off in cleared {
        e.mem.set_u32(this + off, 0);
    }
    if e.mem.u32(block + 0xc) == 0 {
        let start = e.mem.u32(block + 8);
        e.mem.set_u32(block + 0xc, start);
    }
    e.mem.set_u32(this + kept, block);
    for (to, from) in copies.into_iter().zip([8, 0x14, 0x1c]) {
        let word = e.mem.u32(block + from);
        e.mem.set_u32(this + to, word);
    }
    let first = e.mem.u16(block);
    e.mem.set_u16(block + 0x1a, first);
    let first = e.mem.u16(block);
    e.mem.set_u16(block + 0x20, first);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_57c0, fn_00a257c0(u32, f32)),
        entry!(0x00a2_5810, fn_00a25810(u32)),
        entry!(0x00a2_5920, fn_00a25920() -> u32),
        entry!(0x00a2_59b0, fn_00a259b0(u32) -> u32),
        entry!(0x00a2_62d0, fn_00a262d0(u32, u32) -> u32),
        entry!(0x00a2_6310, fn_00a26310(u32, u32, u32)),
        entry!(0x00a2_69f0, fn_00a269f0(u32, i32)),
        entry!(0x00a2_6520, fn_00a26520(u32, u32, u32, u32)),
        entry!(0x00a2_5870, fn_00a25870() -> u8),
        entry!(0x00a2_5880, fn_00a25880()),
        entry!(0x00a2_5b00, fn_00a25b00(u32)),
        entry!(0x00a2_5bb0, fn_00a25bb0(u32)),
        entry!(0x00a2_5bc0, fn_00a25bc0(u32)),
        entry!(0x00a2_5bd0, fn_00a25bd0(u32)),
        entry!(0x00a2_5be0, fn_00a25be0(u32)),
        entry!(0x00a2_5bf0, fn_00a25bf0(u32, u32) -> u8),
        entry!(0x00a2_5c20, fn_00a25c20(u32, u32)),
        entry!(0x00a2_5c80, fn_00a25c80(u32)),
        entry!(0x00a2_5d30, fn_00a25d30(u32)),
        entry!(0x00a2_5d40, fn_00a25d40(u32)),
        entry!(0x00a2_5d50, fn_00a25d50(u32) -> u32),
        entry!(0x00a2_5d70, fn_00a25d70(u32) -> u32),
        entry!(0x00a2_5d80, fn_00a25d80(u32) -> bool),
        entry!(0x00a2_5d90, fn_00a25d90(u32)),
        entry!(0x00a2_5da0, fn_00a25da0(u32) -> bool),
        entry!(0x00a2_5db0, fn_00a25db0(u32)),
        entry!(0x00a2_5dc0, fn_00a25dc0(u32, u32)),
        entry!(0x00a2_5dd0, fn_00a25dd0(u32, u32)),
        entry!(0x00a2_5de0, fn_00a25de0(u32)),
        entry!(0x00a2_5df0, fn_00a25df0(u32)),
        entry!(0x00a2_5e00, fn_00a25e00(u32, u32) -> i32),
        entry!(0x00a2_5e20, fn_00a25e20(u32, u32, u32, u32)),
        entry!(0x00a2_5e70, fn_00a25e70(u32, u32, u32, u32)),
        entry!(0x00a2_5ec0, fn_00a25ec0(u32, u32, u32, u32)),
        entry!(0x00a2_5f10, fn_00a25f10(u32, u32, u32, u32)),
        entry!(0x00a2_5f60, fn_00a25f60(u32, u32, u32, u32, u32)),
        entry!(0x00a2_5fb0, fn_00a25fb0(u32, u32, u32)),
        entry!(0x00a2_5fc0, fn_00a25fc0() -> bool),
        entry!(0x00a2_5fd0, fn_00a25fd0(u32) -> bool),
        entry!(0x00a2_6010, fn_00a26010(u32) -> bool),
        entry!(0x00a2_6020, fn_00a26020(u32, u32) -> bool),
        entry!(0x00a2_6090, fn_00a26090(u32, u32) -> bool),
        entry!(0x00a2_6120, fn_00a26120(u32, u32)),
        entry!(0x00a2_61a0, fn_00a261a0(u32, u32) -> u32),
        entry!(0x00a2_6220, fn_00a26220(u32) -> u32),
        entry!(0x00a2_6240, fn_00a26240(u32) -> u32),
        entry!(0x00a2_6300, fn_00a26300(u32)),
        entry!(0x00a2_6410, fn_00a26410(u32, u32, u32) -> u16),
        entry!(0x00a2_6480, fn_00a26480(u32, u32, u32) -> u32),
        entry!(0x00a2_64d0, fn_00a264d0(u32, u32, u32) -> u32),
        entry!(0x00a2_6a50, fn_00a26a50(u32, u32, u32) -> u16),
        entry!(0x00a2_6b00, fn_00a26b00(u32, u32) -> u32),
    ]
}

/// The double 2.0 the half period is divided by.
const TWO: u32 = 0x0101_1590;

/// `FISTP dword` with the rounding control set to chop (`OR 0xC00`): the
/// truncated value, or the integer indefinite 0x80000000 for NaN and values
/// out of range; the code keeps its low 16 bits.
fn chop_low_word(value: f64) -> u16 {
    let whole = value.trunc();
    let word = if whole.is_nan() || whole < i32::MIN as f64 || whole > i32::MAX as f64 {
        0x8000_0000
    } else {
        whole as i32 as u32
    };
    word as u16
}

/// `ST0 > ST1` as `FLD ST1; FLD ST0; FCOMPP; FNSTSW; TEST AH,0x41; JNZ`
/// decides it: false for an unordered pair.
fn above(st0: f64, st1: f64) -> bool {
    st0 > st1
}

// Translated from 00a26520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Steps one channel of the 0x40-byte object (thiscall: channel `which`,
/// the two 16-bit levels at `state`, elapsed `value`): channel 0 is the
/// words +0x0C..+0x18 with the block at +0x04, channel 1 +0x20..+0x2C with
/// the block at +0x08 (as `00a26310` attaches them). Nothing happens while
/// the channel's length (+0) is 0. The channel's time (+0x0C) advances by
/// `value`; past the length both channels' length and time are cleared and
/// both levels set to 0. Otherwise, with the block's period (+0x0C), its
/// half (`period / 2.0`, a float) and the phase (`time % period`, a float),
/// the channel's level comes from the block's kind (+0x04) and amount (+0):
/// kind 1 a square wave (the amount past half the period, 0 before); kind 3
/// a triangle (the amount scaled by the phase's distance into the half);
/// kind 2 computes a ramp from the time against the half, then takes the
/// kind-1 level, which replaces it; any other kind the amount. Channel 1
/// with the block's byte +0x10 set takes the mirrored forms. The level is
/// stored in the channel's half of `state`.
pub fn fn_00a26520(e: &mut Engine, this: u32, which: u32, state: u32, value: u32) {
    let (channel, block) = if which == 0 {
        (this + 0xc, e.mem.u32(this + 4))
    } else {
        (this + 0x20, e.mem.u32(this + 8))
    };
    if e.mem.u32(channel) == 0 {
        return;
    }
    let time = e.mem.u32(channel + 0xc).wrapping_add(value);
    e.mem.set_u32(channel + 0xc, time);
    if e.mem.u32(channel + 0xc) >= e.mem.u32(channel) {
        e.mem.set_u32(this + 0x20, 0);
        e.mem.set_u32(this + 0x2c, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 0x18, 0);
        e.mem.set_u16(state, 0);
        e.mem.set_u16(state + 2, 0);
        return;
    }
    let period = e.mem.u32(block + 0xc);
    let half = x87_store32(x87_div(f64::from(period), e.mem.f64(TWO)));
    let phase = x87_store32(f64::from(e.mem.u32(channel + 0xc) % e.mem.u32(block + 0xc)));
    let mirrored = which != 0 && e.mem.u8(block + 0x10) != 0;
    // FILD qword of the block's amount (a zero-extended word).
    let amount = |e: &Engine| f64::from(e.mem.u32(block));
    // phase / half, and (phase - half) / half.
    let rising = || x87_div(x87_load32(phase), x87_m32(half));
    let falling = || x87_div(x87_sub(x87_load32(phase), x87_m32(half)), x87_m32(half));
    // The square wave: kind 1, and kind 2 after its ramp.
    let square = |e: &Engine| {
        // `TEST AH,5; JP`: the amount (or 0, mirrored) only when half <
        // phase; an unordered pair takes the other branch.
        let past_half = x87_load32(half) < x87_load32(phase);
        if past_half != mirrored {
            e.mem.u16(block)
        } else {
            0
        }
    };
    let level = match e.mem.u32(block + 4) {
        1 => square(e),
        2 => {
            let time = f64::from(e.mem.u32(channel + 0xc));
            let ramp = if !mirrored {
                if above(x87_load32(half), time) {
                    x87_mul(amount(e), rising())
                } else {
                    x87_mul(amount(e), x87_sub(1.0, falling()))
                }
            } else if above(x87_load32(half), time) {
                x87_mul(amount(e), x87_sub(1.0, rising()))
            } else {
                let unit = x87_div(x87_load32(half), x87_m32(half));
                x87_mul(amount(e), x87_sub(x87_m32(phase), unit))
            };
            // The ramp's level is stored, then replaced by the square wave.
            let _replaced = chop_low_word(ramp);
            square(e)
        }
        3 => {
            let up = above(x87_load32(half), x87_load32(phase));
            let scaled = match (mirrored, up) {
                (false, true) => x87_mul(amount(e), rising()),
                (false, false) => x87_mul(amount(e), falling()),
                (true, true) => x87_mul(amount(e), x87_sub(1.0, rising())),
                (true, false) => x87_mul(amount(e), x87_sub(1.0, falling())),
            };
            chop_low_word(scaled)
        }
        _ => e.mem.u16(block),
    };
    if which == 0 {
        e.mem.set_u16(state, level);
    } else {
        e.mem.set_u16(state + 2, level);
    }
}

/// Method `00a26520` of the object at +0x1B80 (thiscall: channel, state,
/// value).
const OBJECT_1B80_CHANNEL: u32 = 0x00a2_6520;

// Translated from 00a269f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of the 0x40-byte object (the one `00a257c0` calls): for a
/// `value` of at least 1, fills a stack state of two zero 16-bit values
/// through `00a26520(0, &state, value)` and `00a26520(1, &state, value)`
/// and sends it to the user index in the object's first word
/// (`XInputSetState`).
pub fn fn_00a269f0(e: &mut Engine, this: u32, value: i32) {
    if value < 1 {
        return;
    }
    e.with_stack(4, |e, state| {
        let state = state.addr();
        e.mem.set_u16(state, 0);
        e.mem.set_u16(state + 2, 0);
        e.call(OBJECT_1B80_CHANNEL, &[this, 0, state, value as u32]);
        e.call(OBJECT_1B80_CHANNEL, &[this, 1, state, value as u32]);
        let user = e.mem.u32(this);
        e.call(XINPUT_SET_STATE, &[user, state]);
    });
}

#[cfg(test)]
mod state_tests {
    use super::*;

    #[test]
    fn positive_values_fill_both_channels_and_send_the_state() {
        let mut e = Engine::new();
        // The channel double writes its value into its half of the state.
        e.register(OBJECT_1B80_CHANNEL, |e, a| {
            e.mem.set_u16(a[2] + a[1] * 2, (a[3] + a[1]) as u16);
            0u32.into_ret()
        });
        let sent = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let record = sent.clone();
        e.register_double(XINPUT_SET_STATE, move |e, a| {
            record.set(e.mem.u32(a[1]));
            0u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a2_69f0, &[this, 500]);
        let log = e.call_log.take().unwrap();
        let channels: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == OBJECT_1B80_CHANNEL)
            .map(|(_, w)| (w[0], w[1], w[3]))
            .collect();
        assert_eq!(channels, vec![(this, 0, 500), (this, 1, 500)]);
        let send = log.iter().find(|(a, _)| *a == XINPUT_SET_STATE).unwrap();
        assert_eq!(send.1[0], 2);
        assert_eq!(sent.get(), (501 << 16) | 500);
        // Zero and negative values do nothing.
        for value in [0i32, -3] {
            e.call_log = Some(vec![]);
            e.call(0x00a2_69f0, &[this, value as u32]);
            assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn scaled_call_needs_both_flags() {
        for (flag, enabled, called) in [(1u8, 1u8, true), (0, 1, false), (1, 0, false)] {
            let mut e = Engine::new();
            e.map(0x011f_3000, 0x1000);
            e.map(0x0101_7000, 0x1000);
            e.mem.set_u8(ENABLED_FLAG, flag);
            e.mem.set_f64(SCALE, 1000.0);
            e.register(FTOL2, |_, a| {
                let value = f64::take(a, &mut 0);
                (value as i32 as u32).into_ret()
            });
            e.register(OBJECT_1B80_START, |_, _| 0u32.into_ret());
            let this = e.mem.alloc(0x1c04);
            e.mem.set_u8(this + 0x1b88, enabled);
            e.mem.set_u32(this + 0x1b80, 0x0055_0000);
            e.call_log = Some(vec![]);
            e.call(0x00a2_57c0, &[this, 0.25f32.to_bits()]);
            let starts = calls_to(&e, OBJECT_1B80_START);
            if called {
                assert_eq!(starts, vec![vec![0x0055_0000, 250]]);
                assert_eq!(calls_to(&e, FTOL2)[0], args![250.0f64]);
            } else {
                assert!(starts.is_empty());
                assert!(calls_to(&e, FTOL2).is_empty());
            }
        }
    }

    #[test]
    fn stop_clears_the_object_and_sends_a_zero_state() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::Cell::new(u32::MAX));
        let record = seen.clone();
        e.register_double(XINPUT_SET_STATE, move |e, a| {
            record.set(e.mem.u32(a[1]));
            0u32.into_ret()
        });
        let this = e.mem.alloc(0x1c04);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xc, 5);
        e.mem.set_u32(object + 0x20, 6);
        e.mem.set_u32(this + 0x1b80, object);
        e.mem.set_u8(this + 0x1b88, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a2_5810, &[this]);
        assert_eq!(calls_to(&e, XINPUT_SET_STATE).len(), 1);
        assert_eq!(calls_to(&e, XINPUT_SET_STATE)[0][0], 0);
        assert_eq!(seen.get(), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        // Flag clear: nothing happens.
        e.mem.set_u8(this + 0x1b88, 0);
        e.mem.set_u32(object + 0xc, 5);
        e.call_log = Some(vec![]);
        e.call(0x00a2_5810, &[this]);
        assert!(calls_to(&e, XINPUT_SET_STATE).is_empty());
        assert_eq!(e.mem.u32(object + 0xc), 5);
    }

    fn memset_engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn singleton_is_built_once() {
        let mut e = memset_engine();
        e.map(0x011f_3000, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        let first = e.call(0x00a2_5920, &[]).u32();
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first + 0x74), 1);
        assert_eq!(e.call(0x00a2_5920, &[]).u32(), first);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14280]]
        );
        // A failed allocation leaves it null, and the next call tries again.
        let mut e = memset_engine();
        e.map(0x011f_3000, 0x1000);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_5920, &[]).u32(), 0);
        assert_eq!(e.call(0x00a2_5920, &[]).u32(), 0);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 2);
    }

    #[test]
    fn singleton_constructor_sets_its_defaults() {
        let mut e = memset_engine();
        let this = e.mem.alloc(0x14280);
        e.mem.write(this, &vec![0xaa; 0x14280]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_59b0, &[this]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![
                vec![this + 8, 0, 0x1c],
                vec![this + 0x3b8, 0, 0x13ec0],
                vec![this + 0x8c, 0, 0x320],
            ]
        );
        for off in [
            0x3ac, 0x3b0, 0x0, 0x68, 0x50, 0x6c, 0x70, 0x64, 0x14278, 0x3b4,
        ] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x74), 1);
        assert_eq!(e.mem.u32(this + 0x78), 1);
        assert_eq!(e.mem.u32(this + 0x7c), u32::MAX);
        assert_eq!(e.mem.u32(this + 0x80), u32::MAX);
        assert_eq!(e.mem.bytes(this + 0x84, 5), vec![1, 1, 0, 1, 0]);
        // Bytes the constructor does not touch keep their value.
        assert_eq!(e.mem.u8(this + 0x25), 0xaa);
        assert_eq!(e.mem.u8(this + 0x89), 0xaa);
    }

    #[test]
    fn object_constructor_keeps_the_owner_and_the_tick_count() {
        let mut e = Engine::new();
        e.register(IMPORT_GET_TICK_COUNT, |_, _| 12345u32.into_ret());
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00a2_62d0, &[this, 0x77]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x77);
        assert_eq!(e.mem.u32(this + 0x38), 12345);
    }

    #[test]
    fn attach_fills_the_chosen_channel_and_fixes_the_block() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        for off in (0..0x40).step_by(4) {
            e.mem.set_u32(this + off, 0x1111_1111);
        }
        let block = e.mem.alloc(0x24);
        e.mem.set_u16(block, 0x0203);
        e.mem.set_u32(block + 8, 0x80);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x14, 0x140);
        e.mem.set_u32(block + 0x1c, 0x1c0);
        e.call(0x00a2_6310, &[this, 0, block]);
        assert_eq!(e.mem.u32(block + 0xc), 0x80);
        assert_eq!(e.mem.u16(block + 0x1a), 0x0203);
        assert_eq!(e.mem.u16(block + 0x20), 0x0203);
        assert_eq!(e.mem.u32(this + 4), block);
        assert_eq!(e.mem.u32(this + 0xc), 0x80);
        assert_eq!(e.mem.u32(this + 0x10), 0x140);
        assert_eq!(e.mem.u32(this + 0x14), 0x1c0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        // The other channel is untouched.
        assert_eq!(e.mem.u32(this + 8), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x20), 0x1111_1111);

        // Channel 1, with the block's +0x0C already set.
        e.mem.set_u32(block + 0xc, 0x99);
        e.mem.set_u16(block, 0x0405);
        e.call(0x00a2_6310, &[this, 1, block]);
        assert_eq!(e.mem.u32(block + 0xc), 0x99);
        assert_eq!(e.mem.u32(this + 8), block);
        assert_eq!(e.mem.u32(this + 0x20), 0x80);
        assert_eq!(e.mem.u32(this + 0x24), 0x140);
        assert_eq!(e.mem.u32(this + 0x28), 0x1c0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u16(block + 0x1a), 0x0405);
        assert_eq!(e.mem.u16(block + 0x20), 0x0405);
    }
}

#[cfg(test)]
mod channel_tests {
    use super::*;

    /// The object with one block (amount 64, period 80) on both channels,
    /// channel lengths 1000 and the given kind and mirror byte; returns the
    /// object and a zeroed state.
    fn setup(e: &mut Engine, kind: u32, mirror: u8) -> (u32, u32) {
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f64(TWO, 2.0);
        let block = e.mem.alloc(0x24);
        e.mem.set_u32(block, 64);
        e.mem.set_u32(block + 4, kind);
        e.mem.set_u32(block + 0xc, 80);
        e.mem.set_u8(block + 0x10, mirror);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, block);
        e.mem.set_u32(this + 8, block);
        e.mem.set_u32(this + 0xc, 1000);
        e.mem.set_u32(this + 0x20, 1000);
        let state = e.mem.alloc(4);
        (this, state)
    }

    /// The level channel `which` gets after advancing its time to `time`.
    fn level(kind: u32, mirror: u8, which: u32, time: u32) -> u16 {
        let mut e = Engine::new();
        let (this, state) = setup(&mut e, kind, mirror);
        e.mem.set_u32(state, 0xffff_ffff);
        e.call(0x00a2_6520, &[this, which, state, time]);
        let time_slot = if which == 0 { this + 0x18 } else { this + 0x2c };
        assert_eq!(e.mem.u32(time_slot), time);
        // Only the channel's own half of the state changes.
        let other = if which == 0 { state + 2 } else { state };
        assert_eq!(e.mem.u16(other), 0xffff);
        e.mem.u16(state + which * 2)
    }

    #[test]
    fn kinds_give_square_triangle_and_flat_levels() {
        // Kind 3: phase 10 of half 40 rises to a quarter; phase 50 falls.
        assert_eq!(level(3, 0, 0, 10), 16);
        assert_eq!(level(3, 0, 0, 50), 16);
        assert_eq!(level(3, 1, 1, 10), 48);
        assert_eq!(level(3, 1, 1, 50), 48);
        // Channel 0 ignores the mirror byte.
        assert_eq!(level(3, 1, 0, 10), 16);
        // Kind 1: the amount past half the period (mirrored: before it).
        assert_eq!(level(1, 0, 0, 10), 0);
        assert_eq!(level(1, 0, 0, 50), 64);
        assert_eq!(level(1, 1, 1, 10), 64);
        assert_eq!(level(1, 1, 1, 50), 0);
        // Kind 2 ends with the square wave.
        assert_eq!(level(2, 0, 0, 10), 0);
        assert_eq!(level(2, 0, 0, 50), 64);
        assert_eq!(level(2, 1, 1, 10), 64);
        assert_eq!(level(2, 1, 1, 50), 0);
        // Other kinds: the amount.
        assert_eq!(level(7, 0, 0, 10), 64);
        assert_eq!(level(7, 1, 1, 50), 64);
        // The phase wraps with the period: time 90 is phase 10.
        assert_eq!(level(3, 0, 0, 90), 16);
    }

    #[test]
    fn idle_channels_wait_and_finished_ones_reset_both() {
        let mut e = Engine::new();
        let (this, state) = setup(&mut e, 3, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(state, 0x1234_5678);
        e.call(0x00a2_6520, &[this, 0, state, 10]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(state), 0x1234_5678);
        // Channel 1 reaches its length: both channels' length and time and
        // both levels are cleared.
        e.mem.set_u32(this + 0xc, 1000);
        e.mem.set_u32(this + 0x18, 5);
        e.mem.set_u32(this + 0x2c, 990);
        e.call(0x00a2_6520, &[this, 1, state, 10]);
        for off in [0xc, 0x18, 0x20, 0x2c] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(state), 0);
    }

    #[test]
    fn chop_truncates_and_marks_out_of_range() {
        assert_eq!(chop_low_word(16.9), 16);
        assert_eq!(chop_low_word(-1.5), 0xffff);
        assert_eq!(chop_low_word(f64::NAN), 0);
        assert_eq!(chop_low_word(3.0e10), 0);
        assert_eq!(chop_low_word(70000.0), 70000u32 as u16);
    }
}

/// The byte global `00a25870` answers.
const FLAG_011F3630: u32 = 0x011f_3630;
/// The memory manager's free method (thiscall, one word: the block).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `memcpy` (CRT, cdecl: destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;
/// `CompareFileTime` (import slot; stdcall: two `FILETIME` pointers).
const IMPORT_COMPARE_FILE_TIME: u32 = 0x00fd_f06c;
/// The global the code reads in `00a25c80` and never uses.
const UNUSED_GLOBAL_011F63A8: u32 = 0x011f_63a8;
/// The word `00a25c80` reads (`011f3638` +4, an address the compiler
/// tested against 0 and folded).
const CHECKED_011F363C: u32 = 0x011f_363c;
/// Creates the settings collection at `011f35a0` on first use (`004de4a0`,
/// no arguments).
const SETTINGS_CREATE: u32 = 0x004d_e4a0;
/// The settings collection `004de4a0` creates.
const SETTINGS: u32 = 0x011f_35a0;
/// Finds a setting by name (`005e0860`, thiscall: the collection, the name,
/// the out pointer that receives the setting).
const SETTINGS_FIND: u32 = 0x005e_0860;
/// `"uiRedeemedContent:GamePlay"`.
const REDEEMED_CONTENT_NAME: u32 = 0x0108_b5bc;
/// The word the code writes 0 to and uses when the setting is missing.
const NULL_SETTING_WORD: u32 = 0x0120_2800;
/// `__stricmp` (CRT, cdecl).
const STRICMP: u32 = 0x00ec_68e4;
/// The four content names `00a261a0` knows, bit 0 to bit 3: `"Classic"`,
/// `"Tribal"`, `"Caravan"`, `"Mercenary"`.
const CONTENT_NAMES: [u32; 4] = [0x0109_599c, 0x0109_5994, 0x0109_598c, 0x0101_b7dc];
/// The double pi / 180 (`01023128`).
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// The double 1.0 (`01012070`).
const ONE: u32 = 0x0101_2070;
/// The LIBCMT math function at `00eca060` (a `double` argument, the result
/// in ST0; which one it is is not confirmed here).
const CRT_MATH_00ECA060: u32 = 0x00ec_a060;

// Translated from 00a25870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the byte global at `011f3630` (no caller or vtable in the exe).
pub fn fn_00a25870(e: &mut Engine) -> u8 {
    e.mem.u8(FLAG_011F3630)
}

// Translated from 00a25880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the singleton at `011f3634` while it is null (0x14280 bytes,
/// constructed by [`fn_00a259b0`], null when the allocation fails), then
/// calls the empty method [`fn_00a25de0`] on it (no caller or vtable in the
/// exe). EAX is left as the paths happen to leave it, so the uniform form
/// answers nothing.
// Translation incomplete: 00a25880 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00a25880(e: &mut Engine) {
    if e.mem.u32(SINGLETON) == 0 {
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x14280]).u32();
        let object = if block != 0 { fn_00a259b0(e, block) } else { 0 };
        e.mem.set_u32(SINGLETON, object);
    }
    let singleton = e.mem.u32(SINGLETON);
    fn_00a25de0(e, singleton);
}

// Translated from 00a25b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the 0x14280-byte singleton class: when the singleton at
/// `011f3634` is set, destroys it (this function on it) and frees it through
/// the memory manager (`00aa4060`), then calls the empty [`fn_00a25dd0`]
/// with 0; in every case clears the list at +0x3AC ([`fn_00a26240`]). The
/// global is never cleared, so with a singleton set the call on it recurses
/// on the same singleton again (as the exe does; it never returns).
// Translation incomplete: 00a25b00 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00a25b00(e: &mut Engine, this: u32) {
    let singleton = e.mem.u32(SINGLETON);
    if singleton != 0 {
        fn_00a25b00(e, singleton);
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, singleton]);
        fn_00a25dd0(e, this, 0);
    }
    fn_00a26240(e, this + 0x3ac);
}

// Translated from 00a25bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25bb0(_e: &mut Engine, _this: u32) {}

// Translated from 00a25bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25bc0(_e: &mut Engine, _this: u32) {}

// Translated from 00a25bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25bd0(_e: &mut Engine, _this: u32) {}

// Translated from 00a25be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25be0(_e: &mut Engine, _this: u32) {}

// Translated from 00a25bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the singleton's byte at the offset its word +0x68 holds (no
/// caller or vtable in the exe; the one stack word is not read).
pub fn fn_00a25bf0(e: &mut Engine, this: u32, _unused_1: u32) -> u8 {
    let index = e.mem.u32(this + 0x68);
    e.mem.u8(this.wrapping_add(index))
}

// Translated from 00a25c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a pair to the singleton's 8-byte records at +0x28 (count at
/// +0x50): when [`fn_00a26010`] answers true and `value` is not 0, the
/// record at the count gets `value` at +4 and the word +0x68 at +0, and the
/// count grows by one (no caller or vtable in the exe).
pub fn fn_00a25c20(e: &mut Engine, this: u32, value: u32) {
    if !fn_00a26010(e, this) || value == 0 {
        return;
    }
    let count = e.mem.u32(this + 0x50);
    let record = this.wrapping_add(count.wrapping_mul(8));
    e.mem.set_u32(record.wrapping_add(0x2c), value);
    let count = e.mem.u32(this + 0x50);
    let record = this.wrapping_add(count.wrapping_mul(8));
    let current = e.mem.u32(this + 0x68);
    e.mem.set_u32(record.wrapping_add(0x28), current);
    let count = e.mem.u32(this + 0x50).wrapping_add(1);
    e.mem.set_u32(this + 0x50, count);
}

// Translated from 00a25c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads state and does nothing with it (no caller or vtable in the exe):
/// when the singleton's byte at the offset in +0x68 and its byte +4 are set
/// and [`fn_00a26010`] answers true, reads the global `011f63a8`, and when
/// the byte +0x87 is clear, the word at `011f363c`. The two blocks that
/// would write `01202800` test the constant address `011f3638` against 0,
/// which the compiler folded, so they never run and are left out.
pub fn fn_00a25c80(e: &mut Engine, this: u32) {
    let index = e.mem.u32(this + 0x68);
    if e.mem.u8(this.wrapping_add(index)) == 0 || e.mem.u8(this + 4) == 0 {
        return;
    }
    if !fn_00a26010(e, this) {
        return;
    }
    let _unused = e.mem.u32(UNUSED_GLOBAL_011F63A8);
    if e.mem.u8(this + 0x87) != 0 {
        return;
    }
    let _unused = e.mem.u32(CHECKED_011F363C);
}

// Translated from 00a25d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25d30(_e: &mut Engine, _this: u32) {}

// Translated from 00a25d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25d40(_e: &mut Engine, _this: u32) {}

// Translated from 00a25d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the singleton's word +0x14278 (no caller or vtable in the exe).
pub fn fn_00a25d50(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this + 0x14278)
}

// Translated from 00a25d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0 (no caller or vtable in the exe).
pub fn fn_00a25d70(_e: &mut Engine, _this: u32) -> u32 {
    0
}

// Translated from 00a25d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false (no caller or vtable in the exe).
pub fn fn_00a25d80(_e: &mut Engine, _this: u32) -> bool {
    false
}

// Translated from 00a25d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25d90(_e: &mut Engine, _this: u32) {}

// Translated from 00a25da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false (no caller or vtable in the exe).
pub fn fn_00a25da0(_e: &mut Engine, _this: u32) -> bool {
    false
}

// Translated from 00a25db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25db0(_e: &mut Engine, _this: u32) {}

// Translated from 00a25dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method taking one word it does not read (no caller or vtable in
/// the exe).
pub fn fn_00a25dc0(_e: &mut Engine, _this: u32, _unused_1: u32) {}

// Translated from 00a25dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method taking one word it does not read (called by the
/// destructor [`fn_00a25b00`] with 0).
pub fn fn_00a25dd0(_e: &mut Engine, _this: u32, _unused_1: u32) {}

// Translated from 00a25de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (called on the singleton by [`fn_00a25880`]).
pub fn fn_00a25de0(_e: &mut Engine, _this: u32) {}

// Translated from 00a25df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a25df0(_e: &mut Engine, _this: u32) {}

// Translated from 00a25e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the `FILETIME`s at +4 of two records (cdecl):
/// `CompareFileTime(first + 4, second + 4)`, whose answer (-1, 0, 1) it
/// passes on (no caller or vtable in the exe).
pub fn fn_00a25e00(e: &mut Engine, first: u32, second: u32) -> i32 {
    e.call(
        IMPORT_COMPARE_FILE_TIME,
        &[first.wrapping_add(4), second.wrapping_add(4)],
    )
    .i32()
}

/// Copies the `size` low bytes of `value` (from the exe's argument slot) to
/// the cursor `*cursor` and advances `*count` and `*cursor` by `size`.
fn write_value(e: &mut Engine, cursor: u32, count: u32, value: u32, size: u32) {
    let destination = e.mem.u32(cursor);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(MEMCPY, &[destination, slot.addr(), size]);
    });
    let advanced = e.mem.u32(count).wrapping_add(size);
    e.mem.set_u32(count, advanced);
    let advanced = e.mem.u32(cursor).wrapping_add(size);
    e.mem.set_u32(cursor, advanced);
}

// Translated from 00a25e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a 4-byte `value` at the cursor `*cursor` (`memcpy` from its
/// argument slot) and advances `*count` and `*cursor` by 4 (no caller or
/// vtable in the exe; `this` is not read).
pub fn fn_00a25e20(e: &mut Engine, _this: u32, cursor: u32, count: u32, value: u32) {
    write_value(e, cursor, count, value, 4);
}

// Translated from 00a25e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the low byte of `value` at the cursor `*cursor` and advances
/// `*count` and `*cursor` by 1 (no caller or vtable in the exe).
pub fn fn_00a25e70(e: &mut Engine, _this: u32, cursor: u32, count: u32, value: u32) {
    write_value(e, cursor, count, value, 1);
}

// Translated from 00a25ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a 4-byte `value` at the cursor (the same code as
/// [`fn_00a25e20`], for another overload) and advances `*count` and
/// `*cursor` by 4 (no caller or vtable in the exe).
pub fn fn_00a25ec0(e: &mut Engine, _this: u32, cursor: u32, count: u32, value: u32) {
    write_value(e, cursor, count, value, 4);
}

// Translated from 00a25f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the low 16 bits of `value` at the cursor `*cursor` and advances
/// `*count` and `*cursor` by 2 (no caller or vtable in the exe).
pub fn fn_00a25f10(e: &mut Engine, _this: u32, cursor: u32, count: u32, value: u32) {
    write_value(e, cursor, count, value, 2);
}

// Translated from 00a25f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `size` bytes from `source` to the cursor `*cursor` (`memcpy`) and
/// advances `*count` and `*cursor` by `size` (no caller or vtable in the
/// exe).
pub fn fn_00a25f60(e: &mut Engine, _this: u32, cursor: u32, count: u32, source: u32, size: u32) {
    let destination = e.mem.u32(cursor);
    e.call(MEMCPY, &[destination, source, size]);
    let advanced = e.mem.u32(count).wrapping_add(size);
    e.mem.set_u32(count, advanced);
    let advanced = e.mem.u32(cursor).wrapping_add(size);
    e.mem.set_u32(cursor, advanced);
}

// Translated from 00a25fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method taking two words it does not read (no caller or vtable in
/// the exe).
pub fn fn_00a25fb0(_e: &mut Engine, _this: u32, _unused_1: u32, _unused_2: u32) {}

// Translated from 00a25fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false (no `this`; no caller or vtable in the exe).
pub fn fn_00a25fc0(_e: &mut Engine) -> bool {
    false
}

// Translated from 00a25fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the singleton's list at +0x3AC (each node: item +0, next +4) up to
/// its first node with a null item or its end, only reading, and answers
/// true (no caller or vtable in the exe).
pub fn fn_00a25fd0(e: &mut Engine, this: u32) -> bool {
    let mut node = this.wrapping_add(0x3ac);
    while node != 0 && e.mem.u32(node) != 0 {
        let _item = e.mem.u32(node);
        node = e.mem.u32(node + 4);
    }
    true
}

// Translated from 00a26010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers true (called by [`fn_00a25c20`] and [`fn_00a25c80`]).
pub fn fn_00a26010(_e: &mut Engine, _this: u32) -> bool {
    true
}

/// The `"uiRedeemedContent:GamePlay"` setting (`004de4a0` makes sure the
/// collection exists, `005e0860` finds it), or 0.
fn redeemed_content_setting(e: &mut Engine) -> u32 {
    e.call(SETTINGS_CREATE, &[]);
    let settings = e.mem.u32(SETTINGS);
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        e.call(
            SETTINGS_FIND,
            &[settings, REDEEMED_CONTENT_NAME, found.addr()],
        );
        e.mem.u32(found.addr())
    })
}

/// The setting's value word (+4), or the word at `01202800` (set to 0
/// first) when the setting is missing.
fn redeemed_content_word(e: &mut Engine, setting: u32) -> u32 {
    if setting != 0 {
        setting + 4
    } else {
        e.mem.set_u32(NULL_SETTING_WORD, 0);
        NULL_SETTING_WORD
    }
}

// Translated from 00a26020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the content `name` is redeemed (called by
/// `IsCodeRedeemedFunction`): the bit [`fn_00a261a0`] gives the name, tested
/// in the value of the `"uiRedeemedContent:GamePlay"` setting; false when
/// the setting is missing.
pub fn fn_00a26020(e: &mut Engine, this: u32, name: u32) -> bool {
    let setting = redeemed_content_setting(e);
    if setting == 0 {
        return false;
    }
    let bit = fn_00a261a0(e, this, name);
    e.mem.u32(setting + 4) & bit != 0
}

// Translated from 00a26090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the content `name` redeemed (called by `005dd960`): sets its bit
/// ([`fn_00a261a0`]) in the `"uiRedeemedContent:GamePlay"` setting's value
/// (in the word at `01202800`, cleared first, when the setting is missing);
/// answers whether the name has a bit.
pub fn fn_00a26090(e: &mut Engine, this: u32, name: u32) -> bool {
    let setting = redeemed_content_setting(e);
    let bit = fn_00a261a0(e, this, name);
    let word = redeemed_content_word(e, setting);
    let value = e.mem.u32(word) | bit;
    e.mem.set_u32(word, value);
    bit != 0
}

// Translated from 00a26120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the content `name`'s bit ([`fn_00a261a0`]) in the
/// `"uiRedeemedContent:GamePlay"` setting's value (in the word at
/// `01202800`, cleared first, when the setting is missing; called by
/// `005dda00`, which ignores EAX).
pub fn fn_00a26120(e: &mut Engine, this: u32, name: u32) {
    let setting = redeemed_content_setting(e);
    let word = redeemed_content_word(e, setting);
    let bit = fn_00a261a0(e, this, name);
    let value = !bit & e.mem.u32(word);
    e.mem.set_u32(word, value);
}

// Translated from 00a261a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The bit of a content name: `1 << n` for the first of `"Classic"`,
/// `"Tribal"`, `"Caravan"`, `"Mercenary"` that `__stricmp` finds equal to
/// `name`, 0 when none is (`this` is not read).
pub fn fn_00a261a0(e: &mut Engine, _this: u32, name: u32) -> u32 {
    let mut index = 0u32;
    while index < 4 {
        if e.call(STRICMP, &[CONTENT_NAMES[index as usize], name])
            .u32()
            == 0
        {
            break;
        }
        index += 1;
    }
    if index == 4 {
        0
    } else {
        1 << index
    }
}

// Translated from 00a26220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the list at `this` ([`fn_00a26240`]) and passes on its EAX (no
/// caller or vtable in the exe).
pub fn fn_00a26220(e: &mut Engine, this: u32) -> u32 {
    fn_00a26240(e, this)
}

// Translated from 00a26240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a singly linked list (word +0 the item, +4 the first node; each
/// node is laid out the same way): unlinks each node, clears it (this
/// function on it, its next already 0) and frees it through the memory
/// manager (`00aa4060`), then sets the item word to 0. EAX is left holding
/// `this` from the loop's last test, which [`fn_00a26220`] passes on.
pub fn fn_00a26240(e: &mut Engine, this: u32) -> u32 {
    while e.mem.u32(this + 4) != 0 {
        let first = e.mem.u32(this + 4);
        let next = e.mem.u32(first + 4);
        e.mem.set_u32(first + 4, 0);
        let node = e.mem.u32(this + 4);
        if node != 0 {
            fn_00a26240(e, node);
            e.call(MEMORY_FREE, &[MEMORY_MANAGER, node]);
        }
        e.mem.set_u32(this + 4, next);
    }
    e.mem.set_u32(this, 0);
    this
}

// Translated from 00a26300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00a26300(_e: &mut Engine, _this: u32) {}

// Translated from 00a26410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The level of a vibration block (no caller or vtable in the exe): kind
/// (+4) 0 the amount (+0) capped at 0xFFFF, kind 1 [`fn_00a26a50`]'s level,
/// any other kind 0.
pub fn fn_00a26410(e: &mut Engine, this: u32, block: u32, value: u32) -> u16 {
    match e.mem.u32(block + 4) {
        0 => {
            let amount = e.mem.u32(block);
            if amount > 0xffff {
                0xffff
            } else {
                amount as u16
            }
        }
        1 => fn_00a26a50(e, this, block, value),
        _ => 0,
    }
}

/// `(high - low) / step * scale` as `00a26480` and `00a264d0` compute it:
/// 0 when the dword at `step` is 0; otherwise the signed quotient of the two
/// 16-bit values' difference by the 16-bit step (a step whose low 16 bits
/// are 0 faults, as `IDIV` does), its low 16 bits times the low 16 bits of
/// `scale`.
fn ramp_step(e: &mut Engine, block: u32, step: u32, low: u32, high: u32, scale: u32) -> u32 {
    if e.mem.u32(block + step) == 0 {
        return 0;
    }
    let difference = i32::from(e.mem.u16(block + high)) - i32::from(e.mem.u16(block + low));
    let divisor = i32::from(e.mem.u16(block + step));
    let quotient = (difference / divisor) as u16;
    u32::from(quotient).wrapping_mul(scale & 0xffff)
}

// Translated from 00a26480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first channel's ramp step of a vibration block (no caller or vtable
/// in the exe): with the dword +0x14 not 0, `(u16 +0x1A - u16 +0x18) /
/// u16 +0x14` (signed) times the low 16 bits of `scale`; else 0.
pub fn fn_00a26480(e: &mut Engine, _this: u32, block: u32, scale: u32) -> u32 {
    ramp_step(e, block, 0x14, 0x18, 0x1a, scale)
}

// Translated from 00a264d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second channel's ramp step (no caller or vtable in the exe): with
/// the dword +0x1C not 0, `(u16 +0x22 - u16 +0x20) / u16 +0x1C` (signed)
/// times the low 16 bits of `scale`; else 0.
pub fn fn_00a264d0(e: &mut Engine, _this: u32, block: u32, scale: u32) -> u32 {
    ramp_step(e, block, 0x1c, 0x20, 0x22, scale)
}

// Translated from 00a26a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The kind-1 level of [`fn_00a26410`]: computes `amount * (f(value * pi /
/// 180) + 1.0) / 2.0` (`f` the LIBCMT function at `00eca060`) and stores it
/// chopped, then replaces it: the amount's low 16 bits when `value % 360` is
/// below 180, else 0.
pub fn fn_00a26a50(e: &mut Engine, _this: u32, block: u32, value: u32) -> u16 {
    let amount = f64::from(e.mem.u32(block));
    let angle = x87_mul(f64::from(value), e.mem.f64(DEGREES_TO_RADIANS));
    let wave = e.call(CRT_MATH_00ECA060, &args![angle]).f64();
    let scaled = x87_div(
        x87_mul(x87_add(wave, e.mem.f64(ONE)), amount),
        e.mem.f64(TWO),
    );
    let _replaced = chop_low_word(scaled);
    if value % 360 < 180 {
        e.mem.u16(block)
    } else {
        0
    }
}

/// The NiAnimation key comparison `00a26b00` starts with (`00a3d520`, cdecl:
/// the two keys; answers in AL).
const KEY_BASE_EQUAL: u32 = 0x00a3_d520;

// Translated from 00a26b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Key equality (cdecl; called by `00a26f30`, `00a2c550` and `00a2d1d0`,
/// and stored as a function pointer by `00a26e20`): when `00a3d520(first,
/// second)` answers false, passes its EAX on; otherwise 1 when the floats
/// at +4 compare equal (`FUCOMPP`; unordered counts as different), else 0.
pub fn fn_00a26b00(e: &mut Engine, first: u32, second: u32) -> u32 {
    let base = e.call(KEY_BASE_EQUAL, &[first, second]).u32();
    if base as u8 == 0 {
        return base;
    }
    let a = x87_load32(e.mem.u32(first + 4));
    let b = x87_load32(e.mem.u32(second + 4));
    u32::from(a == b)
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn byte_global_is_answered() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u8(FLAG_011F3630, 0x5a);
        assert_eq!(e.call(0x00a2_5870, &[]).u8(), 0x5a);
    }

    #[test]
    fn singleton_getter_builds_once() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00a2_5880, &[]);
        let first = e.mem.u32(SINGLETON);
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first + 0x74), 1);
        e.call(0x00a2_5880, &[]);
        assert_eq!(e.mem.u32(SINGLETON), first);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 1);
        // A failed allocation leaves the singleton null.
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00a2_5880, &[]);
        assert_eq!(e.mem.u32(SINGLETON), 0);
    }

    #[test]
    fn destructor_without_singleton_clears_the_list() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x3b4);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x77);
        e.mem.set_u32(this + 0x3ac, 0x55);
        e.mem.set_u32(this + 0x3b0, node);
        e.call_log = Some(vec![]);
        e.call(0x00a2_5b00, &[this]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, node]]);
        assert_eq!(e.mem.u32(this + 0x3ac), 0);
        assert_eq!(e.mem.u32(this + 0x3b0), 0);
    }

    #[test]
    fn empty_methods_and_constant_answers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14280);
        e.call_log = Some(vec![]);
        for addr in [
            0x00a2_5bb0u32,
            0x00a2_5bc0,
            0x00a2_5bd0,
            0x00a2_5be0,
            0x00a2_5d30,
            0x00a2_5d40,
            0x00a2_5d90,
            0x00a2_5db0,
            0x00a2_5de0,
            0x00a2_5df0,
            0x00a2_6300,
        ] {
            e.call(addr, &[this]);
        }
        e.call(0x00a2_5dc0, &[this, 1]);
        e.call(0x00a2_5dd0, &[this, 1]);
        e.call(0x00a2_5fb0, &[this, 1, 2]);
        // Only the calls the test made: none of them calls anything.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 14);
        assert_eq!(e.mem.bytes(this, 0x100), vec![0; 0x100]);
        e.mem.set_u32(this + 0x14278, 0x1234);
        assert_eq!(e.call(0x00a2_5d50, &[this]).u32(), 0x1234);
        assert_eq!(e.call(0x00a2_5d70, &[this]).u32(), 0);
        assert!(!e.call(0x00a2_5d80, &[this]).bool());
        assert!(!e.call(0x00a2_5da0, &[this]).bool());
        assert!(!e.call(0x00a2_5fc0, &[]).bool());
        assert!(e.call(0x00a2_6010, &[this]).bool());
    }

    #[test]
    fn byte_at_the_current_offset() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x68, 0x30);
        e.mem.set_u8(this + 0x30, 0xc3);
        assert_eq!(e.call(0x00a2_5bf0, &[this, 9]).u8(), 0xc3);
    }

    #[test]
    fn records_append_non_zero_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x50, 1);
        e.mem.set_u32(this + 0x68, 0x44);
        e.call(0x00a2_5c20, &[this, 0x99]);
        assert_eq!(e.mem.u32(this + 0x34), 0x99);
        assert_eq!(e.mem.u32(this + 0x30), 0x44);
        assert_eq!(e.mem.u32(this + 0x50), 2);
        e.call(0x00a2_5c20, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x50), 2);
    }

    #[test]
    fn reading_method_changes_nothing() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.map(0x011f_6000, 0x1000);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x68, 0x10);
        e.mem.set_u8(this + 0x10, 1);
        e.mem.set_u8(this + 4, 1);
        let before = e.mem.bytes(this, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x00a2_5c80, &[this]);
        e.mem.set_u8(this + 0x87, 1);
        e.call(0x00a2_5c80, &[this]);
        e.mem.set_u8(this + 4, 0);
        e.call(0x00a2_5c80, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
        let mut expected = before;
        expected[0x87] = 1;
        expected[4] = 0;
        assert_eq!(e.mem.bytes(this, 0x100), expected);
    }

    #[test]
    fn file_times_are_compared_at_plus_4() {
        let mut e = Engine::new();
        e.register(IMPORT_COMPARE_FILE_TIME, |_, _| (-1i32 as u32).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_5e00, &[0x100, 0x200]).i32(), -1);
        assert_eq!(
            calls_to(&e, IMPORT_COMPARE_FILE_TIME),
            vec![vec![0x104, 0x204]]
        );
    }

    fn memcpy_engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn values_are_written_at_the_cursor() {
        for (addr, size) in [
            (0x00a2_5e20u32, 4u32),
            (0x00a2_5e70, 1),
            (0x00a2_5ec0, 4),
            (0x00a2_5f10, 2),
        ] {
            let mut e = memcpy_engine();
            let buffer = e.mem.alloc(8);
            let cursor = e.mem.alloc(4);
            let count = e.mem.alloc(4);
            e.mem.set_u32(cursor, buffer + 1);
            e.mem.set_u32(count, 10);
            e.call(addr, &[0, cursor, count, 0x4433_2211]);
            let expected: Vec<u8> = [0x11, 0x22, 0x33, 0x44][..size as usize].to_vec();
            assert_eq!(e.mem.bytes(buffer + 1, size), expected);
            assert_eq!(e.mem.u8(buffer + 1 + size), 0);
            assert_eq!(e.mem.u32(cursor), buffer + 1 + size);
            assert_eq!(e.mem.u32(count), 10 + size);
        }
    }

    #[test]
    fn blocks_are_copied_at_the_cursor() {
        let mut e = memcpy_engine();
        let buffer = e.mem.alloc(8);
        let source = e.mem.alloc(3);
        e.mem.write(source, &[7, 8, 9]);
        let cursor = e.mem.alloc(4);
        let count = e.mem.alloc(4);
        e.mem.set_u32(cursor, buffer);
        e.call(0x00a2_5f60, &[0, cursor, count, source, 3]);
        assert_eq!(e.mem.bytes(buffer, 4), vec![7, 8, 9, 0]);
        assert_eq!(e.mem.u32(cursor), buffer + 3);
        assert_eq!(e.mem.u32(count), 3);
    }

    #[test]
    fn list_walk_answers_true() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x3b4);
        let node = e.mem.alloc(8);
        e.mem.set_u32(this + 0x3ac, 1);
        e.mem.set_u32(this + 0x3b0, node);
        e.mem.set_u32(node, 2);
        assert!(e.call(0x00a2_5fd0, &[this]).bool());
    }

    /// The setting `content_engine` finds when asked to (in a mapped page).
    const SETTING: u32 = 0x0120_2100;

    /// An engine whose setting lookup finds [`SETTING`] when `found` (else
    /// nothing) and whose `__stricmp` matches names by address.
    fn content_engine(found: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.register(SETTINGS_CREATE, |e, _| {
            e.mem.set_u32(SETTINGS, 0x0066_0000);
            0u32.into_ret()
        });
        let setting = if found { SETTING } else { 0 };
        e.register_double(SETTINGS_FIND, move |e, a| {
            e.mem.set_u32(a[2], setting);
            u32::from(setting != 0).into_ret()
        });
        e.register(STRICMP, |_, a| u32::from(a[0] != a[1]).into_ret());
        e
    }

    #[test]
    fn content_names_have_bits() {
        let mut e = content_engine(false);
        for (index, name) in CONTENT_NAMES.into_iter().enumerate() {
            assert_eq!(e.call(0x00a2_61a0, &[0, name]).u32(), 1 << index);
        }
        assert_eq!(e.call(0x00a2_61a0, &[0, 0x1234]).u32(), 0);
    }

    #[test]
    fn redeemed_test_reads_the_setting() {
        let mut e = content_engine(true);
        e.mem.set_u32(SETTING + 4, 0b0100);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_6020, &[0, CONTENT_NAMES[2]]).bool());
        assert_eq!(
            calls_to(&e, SETTINGS_FIND)[0][..2],
            [0x0066_0000, REDEEMED_CONTENT_NAME]
        );
        assert!(!e.call(0x00a2_6020, &[0, CONTENT_NAMES[1]]).bool());
        // A missing setting: false, without looking at the name.
        let mut e = content_engine(false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a2_6020, &[0, CONTENT_NAMES[2]]).bool());
        assert!(calls_to(&e, STRICMP).is_empty());
    }

    #[test]
    fn redeeming_sets_the_bit() {
        let mut e = content_engine(true);
        e.mem.set_u32(SETTING + 4, 0b0001);
        assert!(e.call(0x00a2_6090, &[0, CONTENT_NAMES[3]]).bool());
        assert_eq!(e.mem.u32(SETTING + 4), 0b1001);
        assert!(!e.call(0x00a2_6090, &[0, 0x1234]).bool());
        assert_eq!(e.mem.u32(SETTING + 4), 0b1001);
        // Without the setting the word at 01202800 takes the change.
        let mut e = content_engine(false);
        e.mem.set_u32(NULL_SETTING_WORD, 0xff00);
        assert!(e.call(0x00a2_6090, &[0, CONTENT_NAMES[1]]).bool());
        assert_eq!(e.mem.u32(NULL_SETTING_WORD), 0b0010);
    }

    #[test]
    fn clearing_clears_the_bit() {
        let mut e = content_engine(true);
        e.mem.set_u32(SETTING + 4, 0b1001);
        e.call(0x00a2_6120, &[0, CONTENT_NAMES[0]]);
        assert_eq!(e.mem.u32(SETTING + 4), 0b1000);
        let mut e = content_engine(false);
        e.mem.set_u32(NULL_SETTING_WORD, 0xff00);
        e.call(0x00a2_6120, &[0, CONTENT_NAMES[1]]);
        assert_eq!(e.mem.u32(NULL_SETTING_WORD), 0);
    }

    #[test]
    fn list_clear_frees_every_node() {
        let mut e = Engine::new();
        e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
        let list = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        e.mem.set_u32(list, 9);
        e.mem.set_u32(list + 4, first);
        e.mem.set_u32(first + 4, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_6220, &[list]).u32(), list);
        assert_eq!(
            calls_to(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, first], vec![MEMORY_MANAGER, second]]
        );
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.call(0x00a2_6240, &[list]).u32(), list);
    }

    /// pi / 180 as the double at `01023128` holds it.
    fn radians_per_degree() -> f64 {
        f64::from_bits(0x3f91_df46_a000_0000)
    }

    fn wave_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x2000);
        e.map(0x0102_3000, 0x1000);
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f64(ONE, 1.0);
        e.mem.set_f64(DEGREES_TO_RADIANS, radians_per_degree());
        e.register(CRT_MATH_00ECA060, |_, _| 0.5f64.into_ret());
        e
    }

    #[test]
    fn block_levels_by_kind() {
        let mut e = wave_engine();
        let block = e.mem.alloc(0x24);
        e.mem.set_u32(block, 0x12345);
        assert_eq!(e.call(0x00a2_6410, &[0, block, 10]).u16(), 0xffff);
        e.mem.set_u32(block, 0x1234);
        assert_eq!(e.call(0x00a2_6410, &[0, block, 10]).u16(), 0x1234);
        e.mem.set_u32(block + 4, 1);
        assert_eq!(e.call(0x00a2_6410, &[0, block, 10]).u16(), 0x1234);
        assert_eq!(e.call(0x00a2_6410, &[0, block, 200]).u16(), 0);
        e.mem.set_u32(block + 4, 2);
        assert_eq!(e.call(0x00a2_6410, &[0, block, 10]).u16(), 0);
    }

    #[test]
    fn half_turns_keep_the_amount() {
        let mut e = wave_engine();
        let block = e.mem.alloc(0x24);
        e.mem.set_u32(block, 0x0001_0040);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_6a50, &[0, block, 370]).u16(), 0x40);
        assert_eq!(
            calls_to(&e, CRT_MATH_00ECA060),
            vec![args![x87_mul(370.0, radians_per_degree())]]
        );
        assert_eq!(e.call(0x00a2_6a50, &[0, block, 180]).u16(), 0);
        assert_eq!(e.call(0x00a2_6a50, &[0, block, 179]).u16(), 0x40);
    }

    #[test]
    fn first_ramp_step_divides_signed() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x24);
        assert_eq!(e.call(0x00a2_6480, &[0, block, 3]).u32(), 0);
        e.mem.set_u32(block + 0x14, 4);
        e.mem.set_u16(block + 0x18, 2);
        e.mem.set_u16(block + 0x1a, 22);
        assert_eq!(e.call(0x00a2_6480, &[0, block, 0xffff_0003]).u32(), 15);
        e.mem.set_u16(block + 0x18, 22);
        e.mem.set_u16(block + 0x1a, 2);
        // -5 keeps its low 16 bits.
        assert_eq!(e.call(0x00a2_6480, &[0, block, 2]).u32(), 0xfffb * 2);
    }

    #[test]
    fn second_ramp_step_divides_signed() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x24);
        assert_eq!(e.call(0x00a2_64d0, &[0, block, 3]).u32(), 0);
        e.mem.set_u32(block + 0x1c, 3);
        e.mem.set_u16(block + 0x20, 1);
        e.mem.set_u16(block + 0x22, 10);
        assert_eq!(e.call(0x00a2_64d0, &[0, block, 7]).u32(), 21);
    }

    #[test]
    fn keys_compare_base_then_value() {
        let mut e = Engine::new();
        e.register(KEY_BASE_EQUAL, |e, a| {
            (0x4400 | u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1]))).into_ret()
        });
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 1.5f32.to_bits());
        e.mem.set_u32(second + 4, 1.5f32.to_bits());
        assert_eq!(e.call(0x00a2_6b00, &[first, second]).u32(), 1);
        e.mem.set_u32(second + 4, 2.5f32.to_bits());
        assert_eq!(e.call(0x00a2_6b00, &[first, second]).u32(), 0);
        e.mem.set_u32(first + 4, f32::NAN.to_bits());
        e.mem.set_u32(second + 4, f32::NAN.to_bits());
        assert_eq!(e.call(0x00a2_6b00, &[first, second]).u32(), 0);
        // The base comparison failing passes its EAX on.
        e.mem.set_u32(first, 1);
        assert_eq!(e.call(0x00a2_6b00, &[first, second]).u32(), 0x4400);
    }
}
