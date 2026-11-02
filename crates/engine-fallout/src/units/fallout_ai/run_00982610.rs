//! `fallout/ai/run_00982610` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00982610 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder (no name in the engine map): hands `this` (`ECX`) to
/// `00aa5460` and returns nothing (plain `RET`, no stack words).
pub fn fn_00982610(e: &mut Engine, this: Ptr) {
    e.call(0x00aa_5460, &args![this]);
}

/// The constants the two functions below read: `double` 256.0, 8.0, 32.0,
/// 255.0 and 1.5 (their values in the file's data).
const DOUBLE_256: u32 = 0x0102_31d8;
const DOUBLE_8: u32 = 0x0102_40b0;
const DOUBLE_32: u32 = 0x0102_f070;
const DOUBLE_255: u32 = 0x0101_e568;
const DOUBLE_1_5: u32 = 0x0101_6ff0;
/// `00404040` (cdecl, one float; a float in ST0), whose result the first
/// function truncates (`FISTP` under round-toward-zero).
const ROUND_HELPER: u32 = 0x0040_4040;

/// `00404040` on `value`, truncated to its low byte (`FNSTCW`, `OR 0xC00`,
/// `FLDCW`, `FISTP`, `FLDCW`).
fn rounded_byte(e: &mut Engine, value: u32) -> u8 {
    let result = e.call(ROUND_HELPER, &args![x87_quiet_bits(value)]).f64();
    crate::x87::fistp_chop(result) as u8
}

// Translated from 00982630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits the distance between the points at +0x28 and +0x34 of `source`
/// (`00439ef0`, `00457990`) into a count (byte 0) and a step (byte 1): up to
/// 256 the step is `00404040(d / 8)` and the count `00404040(d / step)`;
/// beyond (or for a NaN) the step is 32 and the count `00404040(d / 32)`,
/// or 255 when that quotient is above 255 (`RET 4`).
pub fn fn_00982630(e: &mut Engine, this: Ptr, source: Ptr) {
    let at = this.addr();
    let distance = e.with_stack(0xc, |e, difference| {
        let result = e
            .call(
                0x0043_9ef0,
                &args![source.addr() + 0x28, difference, source.addr() + 0x34],
            )
            .u32();
        x87_store32(e.call(0x0045_7990, &args![result]).f64())
    });
    // `FCOMP 256.0; TEST AH,0x41; JP`: above 256 or unordered goes on below.
    if x87_load32(distance) <= e.mem.f64(DOUBLE_256) {
        let quotient = x87_store32(x87_div(x87_load32(distance), e.mem.f64(DOUBLE_8)));
        let step = rounded_byte(e, quotient);
        e.mem.set_u8(at + 1, step);
        let step = f64::from(e.mem.u8(at + 1));
        let quotient = x87_store32(x87_div(x87_m32(distance), step));
        let count = rounded_byte(e, quotient);
        e.mem.set_u8(at, count);
        return;
    }
    let quotient = x87_store32(x87_div(x87_load32(distance), e.mem.f64(DOUBLE_32)));
    // `FCOMP 255.0; TEST AH,0x41; JNZ`: only above 255 (ordered) is capped.
    if x87_load32(quotient) > e.mem.f64(DOUBLE_255) {
        e.mem.set_u8(at, 0xff);
    } else {
        let count = rounded_byte(e, quotient);
        e.mem.set_u8(at, count);
    }
    e.mem.set_u8(at + 1, 0x20);
}

// Translated from 00982750 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `value` times 1.5 below the count (byte 0) answers 1; otherwise
/// `_ftol2(00476b20(value * 1.5 / count))` (`00ec62c0` on the float in ST0).
/// Answers the low byte (`RET 4`).
pub fn fn_00982750(e: &mut Engine, this: Ptr, value: f32) -> u8 {
    let scaled = x87_store32(x87_mul(x87_load32(value.to_bits()), e.mem.f64(DOUBLE_1_5)));
    let count = f64::from(e.mem.u8(this.addr()));
    // `FCOMPP; TEST AH,5; JP`: below the count (ordered) answers 1.
    if x87_load32(scaled) < count {
        return 1;
    }
    let count = f64::from(e.mem.u8(this.addr()));
    let ratio = x87_store32(x87_div(x87_m32(scaled), count));
    let result = e.call(0x0047_6b20, &args![x87_quiet_bits(ratio)]).f64();
    e.call(0x00ec_62c0, &args![result]).u8()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_2610, fn_00982610(Ptr)),
        entry!(0x0098_2630, fn_00982630(Ptr, Ptr)),
        entry!(0x0098_2750, fn_00982750(Ptr, f32) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00982610_passes_this_on() {
        let mut e = Engine::new();
        e.register(0x00aa_5460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_2610, &args![0x4440u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00aa_5460).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x4440]);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn map_data(e: &mut Engine) {
        e.map(0x0101_0000, 0x0002_0000);
        e.map(0x0102_0000, 0x0002_0000);
        for (address, value) in [
            (DOUBLE_256, 256.0f64),
            (DOUBLE_8, 8.0),
            (DOUBLE_32, 32.0),
            (DOUBLE_255, 255.0),
            (DOUBLE_1_5, 1.5),
        ] {
            e.mem.set_f64(address, value);
        }
    }

    fn with_distance(e: &mut Engine, distance: f64) {
        e.register(0x0043_9ef0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register_double(0x0045_7990, move |_, _| Ret {
            st0: distance,
            ..Ret::default()
        });
        // The helper here: its float argument back (the exe's rounds it).
        e.register(ROUND_HELPER, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
    }

    #[test]
    fn fn_00982630_splits_a_short_distance_by_eighths() {
        let mut e = Engine::new();
        map_data(&mut e);
        with_distance(&mut e, 100.0);
        let this = e.mem.alloc(2);
        let source = e.mem.alloc(0x40);
        e.call(0x0098_2630, &[this, source]);
        // 100 / 8 = 12.5 -> 12; 100 / 12 = 8.33 -> 8.
        assert_eq!(e.mem.u8(this + 1), 12);
        assert_eq!(e.mem.u8(this), 8);
    }

    #[test]
    fn fn_00982630_uses_steps_of_32_beyond_256() {
        let mut e = Engine::new();
        map_data(&mut e);
        with_distance(&mut e, 1000.0);
        let this = e.mem.alloc(2);
        let source = e.mem.alloc(0x40);
        e.call(0x0098_2630, &[this, source]);
        assert_eq!(e.mem.u8(this), 31);
        assert_eq!(e.mem.u8(this + 1), 0x20);
        with_distance(&mut e, 10000.0);
        e.call(0x0098_2630, &[this, source]);
        assert_eq!(e.mem.u8(this), 0xff);
    }

    #[test]
    fn fn_00982750_answers_one_below_the_count() {
        let mut e = Engine::new();
        map_data(&mut e);
        let this = e.mem.alloc(2);
        e.mem.set_u8(this, 10);
        assert_eq!(e.call(0x0098_2750, &[this, 2.0f32.to_bits()]).u8(), 1);
        e.register(0x0047_6b20, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 10.0,
            ..Ret::default()
        });
        e.register(0x00ec_62c0, |_, a| Ret {
            eax: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32,
            ..Ret::default()
        });
        // 8 * 1.5 = 12, / 10 = 1.2, * 10 = 12.
        assert_eq!(e.call(0x0098_2750, &[this, 8.0f32.to_bits()]).u8(), 12);
    }
}
