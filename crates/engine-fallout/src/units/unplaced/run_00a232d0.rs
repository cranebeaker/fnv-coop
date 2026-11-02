//! `(unplaced)/run_00a232d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte `00a23390` tests before answering selectors 7 to 10.
const FLAG_011F35C8: u32 = 0x011f_35c8;
/// The four signed 16-bit words selectors 7 to 10 answer (sign-extended).
const VALUES_011F35B0: u32 = 0x011f_35b0;

// Translated from 00a23390 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, two words: an index and a selector; `RET 8`). For the 0xA0-byte
/// entry `index` from `this + 0x34`: selectors 1 to 6 answer the entry's words +0x00 to
/// +0x14; selectors 7 to 10, when the byte at `011f35c8` is not 0, answer the signed 16-bit
/// words at `011f35b0`, `011f35b2`, `011f35b4` and `011f35b6` (sign-extended), else 0; every
/// other selector answers 0 (the exe's jump table at `00a23460`, as `match` arms).
pub fn fn_00a23390(e: &mut Engine, this: Ptr, index: i32, selector: u32) -> i32 {
    let entry = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(0xa0))
        .wrapping_add(0x34);
    match selector {
        1..=6 => e.mem.u32(entry.wrapping_add(4 * (selector - 1))) as i32,
        7..=10 => {
            if e.mem.u8(FLAG_011F35C8) != 0 {
                i32::from(e.mem.u16(VALUES_011F35B0 + 2 * (selector - 7)) as i16)
            } else {
                0
            }
        }
        _ => 0,
    }
}

/// `_ftol2` (`00ec62c0`): the value in x87 `ST0`, truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// The `double` 10000.0 the dead-zone setter scales by (`.rdata`).
const DEAD_ZONE_SCALE: u32 = 0x0102_f078;

/// The address of joystick entry `index` (0xa0 bytes each from `this + 0x34`).
fn joystick_entry(this: Ptr, index: i32) -> u32 {
    this.addr()
        .wrapping_add((index as u32).wrapping_mul(0xa0))
        .wrapping_add(0x34)
}

// Translated from 00a232d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No references in Ghidra (thiscall, `RET 8`: an index and a selector; unoptimised code):
/// selectors 1 to 6 answer bit 0 to bit 5 of the word at `this + index * 8 + 0x18b4` (masked,
/// not shifted down), any other selector 0 (the exe's jump table at `00a2336c`, as `match`
/// arms).
pub fn fn_00a232d0(e: &mut Engine, this: Ptr, index: i32, selector: u32) -> u32 {
    let word = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(8))
        .wrapping_add(0x18b4);
    match selector.wrapping_sub(1) {
        case @ 0..=5 => e.mem.u32(word) & (1 << case),
        _ => 0,
    }
}

// Translated from 00a23490 (decompiled, FalloutNV.exe 1.4.0.525)
/// No references in Ghidra (thiscall, `RET 8`: an index and a bit number): the word at
/// `this + index * 8 + 0x18b8` masked with `1 << bit` (`SHL` by `CL`, so the bit number is
/// taken modulo 32).
pub fn fn_00a23490(e: &mut Engine, this: Ptr, index: i32, bit: u32) -> u32 {
    let word = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(8))
        .wrapping_add(0x18b8);
    (1u32 << (bit & 0x1f)) & e.mem.u32(word)
}

// Translated from 00a234c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joystick button test (thiscall, `RET 0xc`: joystick, key, mode; called by `00a24560` and
/// `007d52a0`): with the current state byte at entry + 0x30 + key and the previous one at
/// entry + 0x80 + key (bit 0x80 meaning down), mode 0 answers down, 1 down now but not before,
/// 2 down before but not now, 3 changed; any other mode 0 (the exe's jump table at `00a235bc`,
/// as `match` arms). The answer is 1 or 0.
pub fn fn_00a234c0(e: &mut Engine, this: Ptr, joystick: i32, key: i32, mode: u32) -> u32 {
    let entry = joystick_entry(this, joystick);
    let current_at = entry.wrapping_add(key as u32).wrapping_add(0x30);
    let previous_at = entry
        .wrapping_add(0x50)
        .wrapping_add(key as u32)
        .wrapping_add(0x30);
    let down = |e: &Engine, at: u32| e.mem.u8(at) & 0x80 != 0;
    let hit = match mode {
        0 => down(e, current_at),
        1 => !down(e, previous_at) && down(e, current_at),
        2 => !down(e, current_at) && down(e, previous_at),
        3 => down(e, current_at) != down(e, previous_at),
        _ => false,
    };
    u32::from(hit)
}

// Translated from 00a235d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No references in Ghidra (thiscall, `RET 8`; neither word is read): returns 1.
pub fn fn_00a235d0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    1
}

// Translated from 00a235f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joystick bound key (thiscall, `RET 8`: joystick, slot; called by `00a24560` and `007d52a0`):
/// the word at entry + 0x20 + slot * 4; -1 when its low 16 bits are all ones, else the word
/// divided by 0x1194 (signed `IDIV`) plus 0x20.
pub fn fn_00a235f0(e: &mut Engine, this: Ptr, joystick: i32, slot: i32) -> i32 {
    let entry = joystick_entry(this, joystick);
    let value = e.mem.u32(
        entry
            .wrapping_add((slot as u32).wrapping_mul(4))
            .wrapping_add(0x20),
    );
    if value & 0xffff == 0xffff {
        return -1;
    }
    (value as i32).wrapping_div(0x1194).wrapping_add(0x20)
}

// Translated from 00a23650 (decompiled, FalloutNV.exe 1.4.0.525)
/// No references in Ghidra (thiscall, `RET 4`: an index): 2 when the second byte of the word
/// at `this + index * 0x244 + 0x558` is 2, else -1.
pub fn fn_00a23650(e: &mut Engine, this: Ptr, index: i32) -> i32 {
    let word = e.mem.u32(
        this.addr()
            .wrapping_add((index as u32).wrapping_mul(0x244))
            .wrapping_add(0x558),
    );
    if (word >> 8) & 0xff == 2 {
        2
    } else {
        -1
    }
}

/// The word at `this + index * 0x2c + offset`, as `00a23690`, `00a236b0` and `00a236d0` read
/// it.
fn pad_word(e: &Engine, this: Ptr, index: i32, offset: u32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add((index as u32).wrapping_mul(0x2c))
            .wrapping_add(offset),
    )
}

// Translated from 00a23690 (decompiled, FalloutNV.exe 1.4.0.525)
/// No references in Ghidra (thiscall, `RET 4`: an index): the word at
/// `this + index * 0x2c + 0x1760`.
pub fn fn_00a23690(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    pad_word(e, this, index, 0x1760)
}

// Translated from 00a236b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joystick button count, as its callers use it (thiscall, `RET 4`: an index; called by
/// `007d52a0` and `007cfe40`): the word at `this + index * 0x2c + 0x1764`.
pub fn fn_00a236b0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    pad_word(e, this, index, 0x1764)
}

// Translated from 00a236d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joystick axis count, as its caller uses it (thiscall, `RET 4`: an index; called by
/// `007d52a0`): the word at `this + index * 0x2c + 0x1768`.
pub fn fn_00a236d0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    pad_word(e, this, index, 0x1768)
}

/// The device at `this + joystick * 4 + 0xc`.
fn joystick_device(e: &Engine, this: Ptr, joystick: i32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add((joystick as u32).wrapping_mul(4))
            .wrapping_add(0xc),
    )
}

// Translated from 00a236f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joystick range setter (thiscall, `RET 0xc`: joystick, minimum, maximum; called by
/// `00a24830`): the device at `this + joystick * 4 + 0xc` gets property 4 (its vtable +0x18,
/// stdcall: device, 4, block) with a 0x18-byte stack block: sizes 0x18 and 0x10, two zero
/// words, `minimum`, `maximum`. Ends in that call, so its answer is returned.
pub fn fn_00a236f0(e: &mut Engine, this: Ptr, joystick: i32, minimum: u32, maximum: u32) -> u32 {
    let device = joystick_device(e, this, joystick);
    e.with_stack(0x18, |e, block| {
        let b = block.addr();
        for (at, value) in [
            (0u32, 0x18u32),
            (4, 0x10),
            (0xc, 0),
            (8, 0),
            (0x10, minimum),
            (0x14, maximum),
        ] {
            e.mem.set_u32(b + at, value);
        }
        e.vcall(device, 0x18, &args![4u32, block]).u32()
    })
}

// Translated from 00a23750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joystick dead-zone setter (thiscall, `RET 8`: joystick, `float`; called by `00a24830`): the
/// device at `this + joystick * 4 + 0xc` gets property 5 (its vtable +0x18, stdcall: device, 5,
/// block) with a 0x14-byte stack block: sizes 0x14 and 0x10, two zero words and
/// `_ftol2(value * 10000.0)` (`FLD dword`, `FMUL qword [0102f078]`). Ends in that call, so its
/// answer is returned.
pub fn fn_00a23750(e: &mut Engine, this: Ptr, joystick: i32, value: f32) -> u32 {
    let device = joystick_device(e, this, joystick);
    e.with_stack(0x14, |e, block| {
        let b = block.addr();
        for (at, word) in [(0u32, 0x14u32), (4, 0x10), (0xc, 0), (8, 0)] {
            e.mem.set_u32(b + at, word);
        }
        let scaled = x87_mul(
            x87_load32(value.to_bits()),
            e.global::<f64>(DEAD_ZONE_SCALE),
        );
        let dead_zone = e.call(FTOL, &args![scaled]).u32();
        e.mem.set_u32(b + 0x10, dead_zone);
        e.vcall(device, 0x18, &args![5u32, block]).u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_3390, fn_00a23390(Ptr, i32, u32) -> i32),
        entry!(0x00a2_32d0, fn_00a232d0(Ptr, i32, u32) -> u32),
        entry!(0x00a2_3490, fn_00a23490(Ptr, i32, u32) -> u32),
        entry!(0x00a2_34c0, fn_00a234c0(Ptr, i32, i32, u32) -> u32),
        entry!(0x00a2_35d0, fn_00a235d0(Ptr, u32, u32) -> u32),
        entry!(0x00a2_35f0, fn_00a235f0(Ptr, i32, i32) -> i32),
        entry!(0x00a2_3650, fn_00a23650(Ptr, i32) -> i32),
        entry!(0x00a2_3690, fn_00a23690(Ptr, i32) -> u32),
        entry!(0x00a2_36b0, fn_00a236b0(Ptr, i32) -> u32),
        entry!(0x00a2_36d0, fn_00a236d0(Ptr, i32) -> u32),
        entry!(0x00a2_36f0, fn_00a236f0(Ptr, i32, u32, u32) -> u32),
        entry!(0x00a2_3750, fn_00a23750(Ptr, i32, f32) -> u32),
    ]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00a23390_answers_the_entry_words_and_the_flagged_globals() {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        let this = e.mem.alloc(0x34 + 2 * 0xa0);
        for word in 0..6u32 {
            e.mem.set_u32(this + 0x34 + 0xa0 + 4 * word, 100 + word);
        }
        for selector in 1..=6u32 {
            assert_eq!(
                e.call(0x00a2_3390, &args![this, 1u32, selector]).i32(),
                99 + selector as i32
            );
        }
        // Selectors 7..=10 read the words only with the flag set.
        for (i, value) in [-5i16, 7, -32768, 12].iter().enumerate() {
            e.mem.set_u16(VALUES_011F35B0 + 2 * i as u32, *value as u16);
        }
        for selector in 7..=10u32 {
            assert_eq!(e.call(0x00a2_3390, &args![this, 1u32, selector]).i32(), 0);
        }
        e.mem.set_u8(FLAG_011F35C8, 1);
        let got: Vec<i32> = (7..=10u32)
            .map(|s| e.call(0x00a2_3390, &args![this, 1u32, s]).i32())
            .collect();
        assert_eq!(got, vec![-5, 7, -32768, 12]);
        // Selector 0 and anything above 10 answer 0.
        for selector in [0u32, 11, 0xffff_ffff] {
            assert_eq!(e.call(0x00a2_3390, &args![this, 1u32, selector]).i32(), 0);
        }
    }
}

#[cfg(test)]
mod n078_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const DEVICE_SET_PROPERTY: u32 = 0x7000_0018;

    /// A 0x2000-byte input object.
    fn object(e: &mut Engine) -> u32 {
        e.mem.alloc(0x2000)
    }

    #[test]
    fn bits_00a232d0_answer_selectors_one_to_six() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.mem.set_u32(this + 8 + 0x18b4, 0x2b);
        let got: Vec<u32> = (0..=7u32)
            .map(|s| e.call(0x00a2_32d0, &args![this, 1u32, s]).u32())
            .collect();
        assert_eq!(got, vec![0, 1, 2, 0, 8, 0, 0x20, 0]);
    }

    #[test]
    fn bit_00a23490_shifts_modulo_32() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.mem.set_u32(this + 0x10 + 0x18b8, 0x8000_0004);
        assert_eq!(e.call(0x00a2_3490, &args![this, 2u32, 2u32]).u32(), 4);
        assert_eq!(e.call(0x00a2_3490, &args![this, 2u32, 34u32]).u32(), 4);
        assert_eq!(
            e.call(0x00a2_3490, &args![this, 2u32, 31u32]).u32(),
            0x8000_0000
        );
        assert_eq!(e.call(0x00a2_3490, &args![this, 2u32, 3u32]).u32(), 0);
    }

    #[test]
    fn button_00a234c0_modes() {
        let mut e = Engine::new();
        let this = object(&mut e);
        let entry = this + 0xa0 + 0x34;
        let set = |e: &mut Engine, now: bool, before: bool| {
            e.mem
                .set_u8(entry + 0x30 + 5, if now { 0x80 } else { 0x7f });
            e.mem
                .set_u8(entry + 0x80 + 5, if before { 0x80 } else { 0 });
        };
        let ask =
            |e: &mut Engine, mode: u32| e.call(0x00a2_34c0, &args![this, 1u32, 5u32, mode]).u32();
        set(&mut e, true, false);
        assert_eq!(
            [
                ask(&mut e, 0),
                ask(&mut e, 1),
                ask(&mut e, 2),
                ask(&mut e, 3),
                ask(&mut e, 4)
            ],
            [1, 1, 0, 1, 0]
        );
        set(&mut e, false, true);
        assert_eq!(
            [
                ask(&mut e, 0),
                ask(&mut e, 1),
                ask(&mut e, 2),
                ask(&mut e, 3)
            ],
            [0, 0, 1, 1]
        );
        set(&mut e, true, true);
        assert_eq!(
            [
                ask(&mut e, 0),
                ask(&mut e, 1),
                ask(&mut e, 2),
                ask(&mut e, 3)
            ],
            [1, 0, 0, 0]
        );
    }

    #[test]
    fn one_00a235d0() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a2_35d0, &args![0u32, 7u32, 8u32]).u32(), 1);
    }

    #[test]
    fn bound_key_00a235f0_divides_or_answers_minus_one() {
        let mut e = Engine::new();
        let this = object(&mut e);
        let entry = this + 0x34;
        e.mem.set_u32(entry + 0x20 + 4, 0x1194 * 3);
        assert_eq!(e.call(0x00a2_35f0, &args![this, 0u32, 1u32]).i32(), 0x23);
        e.mem.set_u32(entry + 0x20 + 8, 0x0001_ffff);
        assert_eq!(e.call(0x00a2_35f0, &args![this, 0u32, 2u32]).i32(), -1);
        // Signed division of a negative word.
        e.mem.set_u32(entry + 0x20 + 12, (-0x1194i32 * 2) as u32);
        assert_eq!(e.call(0x00a2_35f0, &args![this, 0u32, 3u32]).i32(), 0x1e);
    }

    #[test]
    fn kind_00a23650_is_two_or_minus_one() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1000);
        e.mem.set_u32(this + 0x244 + 0x558, 0x0000_0200);
        assert_eq!(e.call(0x00a2_3650, &args![this, 1u32]).i32(), 2);
        e.mem.set_u32(this + 0x244 + 0x558, 0x0000_0302);
        assert_eq!(e.call(0x00a2_3650, &args![this, 1u32]).i32(), -1);
    }

    #[test]
    fn pad_words_00a23690_00a236b0_00a236d0() {
        let mut e = Engine::new();
        let this = object(&mut e);
        for (i, at) in [0x1760u32, 0x1764, 0x1768].iter().enumerate() {
            e.mem.set_u32(this + 0x2c * 3 + at, 0x100 + i as u32);
        }
        assert_eq!(e.call(0x00a2_3690, &args![this, 3u32]).u32(), 0x100);
        assert_eq!(e.call(0x00a2_36b0, &args![this, 3u32]).u32(), 0x101);
        assert_eq!(e.call(0x00a2_36d0, &args![this, 3u32]).u32(), 0x102);
    }

    /// A device whose vtable +0x18 checks and answers.
    fn device(e: &mut Engine, this: u32, joystick: u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x18, DEVICE_SET_PROPERTY);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        e.mem.set_u32(this + 0xc + 4 * joystick, device);
    }

    #[test]
    fn range_00a236f0_sets_property_four() {
        let mut e = Engine::new();
        let this = object(&mut e);
        device(&mut e, this, 1);
        e.register(DEVICE_SET_PROPERTY, |e, a| {
            assert_eq!(a[1], 4);
            let b = a[2];
            let words: Vec<u32> = (0..6).map(|i| e.mem.u32(b + 4 * i)).collect();
            assert_eq!(words, vec![0x18, 0x10, 0, 0, (-100i32) as u32, 100]);
            ret(0x55)
        });
        assert_eq!(
            e.call(0x00a2_36f0, &args![this, 1u32, -100i32, 100u32])
                .u32(),
            0x55
        );
    }

    #[test]
    fn dead_zone_00a23750_scales_and_sets_property_five() {
        let mut e = Engine::new();
        let this = object(&mut e);
        device(&mut e, this, 0);
        e.map(0x0102_f000, 0x1000);
        e.mem.set_u64(DEAD_ZONE_SCALE, 10000.0f64.to_bits());
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ret(value.trunc() as i32 as u32)
        });
        e.register(DEVICE_SET_PROPERTY, |e, a| {
            assert_eq!(a[1], 5);
            let b = a[2];
            let words: Vec<u32> = (0..5).map(|i| e.mem.u32(b + 4 * i)).collect();
            assert_eq!(words, vec![0x14, 0x10, 0, 0, 2500]);
            ret(0x66)
        });
        assert_eq!(e.call(0x00a2_3750, &args![this, 0u32, 0.25f32]).u32(), 0x66);
    }
}
