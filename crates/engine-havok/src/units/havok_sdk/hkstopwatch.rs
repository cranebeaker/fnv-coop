//! `Havok SDK/hkstopwatch.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkStopwatch`'s static functions forward to the system clock object kept at `01269cc0`
//! (its virtual slots `0xc` and `0x10` are the tick counter and the tick rate).

#[allow(unused_imports)]
use crate::prelude::*;

/// The system clock object (written at run time).
const SYSTEM_CLOCK: u32 = 0x0126_9cc0;

// Translated from 00d60550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStopwatch::divide64` (Xbox PDB), cdecl, two 64-bit words `(dividend, divisor)`, result in
/// `ST0`: both are loaded as signed 64-bit integers (`FILD`), divided on the x87 stack, rounded to a
/// `float` through memory and loaded back (a signalling NaN comes back quieted).
pub fn hk_stopwatch_divide64(_e: &mut Engine, dividend: u64, divisor: u64) -> f32 {
    let quotient = x87_div(dividend as i64 as f64, divisor as i64 as f64);
    x87_quiet(f32::from_bits(x87_store32(quotient)))
}

// Translated from 00d60570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStopwatch::getTickCounter` (Xbox PDB), no arguments: tail-calls slot `0xc` of the system
/// clock at `01269cc0` and answers what it answers (64 bits in `EDX:EAX`).
pub fn hk_stopwatch_get_tick_counter(e: &mut Engine) -> u64 {
    let clock = e.mem.u32(SYSTEM_CLOCK);
    e.vcall(clock, 0xc, &args![]).u64()
}

// Translated from 00d60580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStopwatch::getTicksPerSecond` (Xbox PDB), no arguments: tail-calls slot `0x10` of the system
/// clock at `01269cc0` and answers what it answers (64 bits in `EDX:EAX`).
pub fn hk_stopwatch_get_ticks_per_second(e: &mut Engine) -> u64 {
    let clock = e.mem.u32(SYSTEM_CLOCK);
    e.vcall(clock, 0x10, &args![]).u64()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d60550, hk_stopwatch_divide64(u64, u64) -> f32),
        entry!(0x00d60570, hk_stopwatch_get_tick_counter() -> u64),
        entry!(0x00d60580, hk_stopwatch_get_ticks_per_second() -> u64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A system clock whose slots `0xc` and `0x10` answer the 64-bit values `counter` and `rate`.
    fn clock(e: &mut Engine, counter: u64, rate: u64) {
        e.map(0x0126_9000, 0x1000);
        e.register_double(0x0b00_000c, move |_, _| Ret {
            eax: counter as u32,
            edx: (counter >> 32) as u32,
            ..Ret::default()
        });
        e.register_double(0x0b00_0010, move |_, _| Ret {
            eax: rate as u32,
            edx: (rate >> 32) as u32,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0b00_000c);
        e.mem.set_u32(vtable + 0x10, 0x0b00_0010);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(SYSTEM_CLOCK, object);
    }

    #[test]
    fn the_tick_functions_forward_to_the_clock_slots() {
        let mut e = Engine::new();
        clock(&mut e, 0x1_2345_6789, 0x2_0000_0001);
        assert_eq!(e.call(0x00d6_0570, &args![]).u64(), 0x1_2345_6789);
        assert_eq!(e.call(0x00d6_0580, &args![]).u64(), 0x2_0000_0001);
    }

    #[test]
    fn divide64_divides_signed_64_bit_integers_into_a_float() {
        let mut e = Engine::new();
        let divide =
            |e: &mut Engine, a: i64, b: i64| e.call(0x00d6_0550, &args![a as u64, b as u64]).f32();
        assert_eq!(divide(&mut e, 1, 4), 0.25);
        assert_eq!(divide(&mut e, -9, 3), -3.0);
        assert_eq!(divide(&mut e, 10_000_000_000, 4), 2.5e9);
        // Rounded to a float: 1/3.
        assert_eq!(divide(&mut e, 1, 3), 1.0f32 / 3.0f32);
        assert!(divide(&mut e, 1, 0).is_infinite());
        assert!(divide(&mut e, 0, 0).is_nan());
    }
}
