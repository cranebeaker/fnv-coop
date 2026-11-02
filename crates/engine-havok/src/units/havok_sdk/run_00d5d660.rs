//! `Havok SDK/run_00d5d660` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float `2^32` (`01099378`) added to a negative seed, and the double `2^-32` (`010229c0`).
const TWO_POW_32: u32 = 0x0109_9378;
const TWO_POW_MINUS_32: u32 = 0x0102_29c0;

// Translated from 00d5d660 (decompiled, FalloutNV.exe 1.4.0.525)
/// A linear congruential generator filling four floats (thiscall, `RET 4`: one stack word `out`;
/// no map name): four times, the seed at `this + 4` becomes `seed * 0x19660d + 0x3c6ef35f` (32-bit
/// wrap-around) and `out[i]` is `float(seed as an unsigned number * 2^-32)`: the seed is loaded as
/// a signed integer, the float `2^32` added when it is negative, and the double product with
/// `2^-32` rounded to a float.
pub fn fn_00d5d660(e: &mut Engine, this: Ptr, out: Ptr) {
    let scale = e.mem.f64(TWO_POW_MINUS_32);
    for lane in 0..4u32 {
        let seed = e
            .mem
            .u32(this.addr() + 4)
            .wrapping_mul(0x0019_660d)
            .wrapping_add(0x3c6e_f35f);
        e.mem.set_u32(this.addr() + 4, seed);
        let mut value = f64::from(seed as i32);
        if (seed as i32) < 0 {
            value = x87_add(value, x87_m32(e.mem.u32(TWO_POW_32)));
        }
        let product = x87_mul(value, scale);
        e.mem.set_u32(out.addr() + lane * 4, x87_store32(product));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d5_d660, fn_00d5d660(Ptr, Ptr))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d5d660_steps_the_generator_four_times() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(TWO_POW_32, 0x4f80_0000);
        e.mem.set_f64(TWO_POW_MINUS_32, 2.0f64.powi(-32));
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 12345);
        e.call(0x00d5_d660, &args![this, out]);
        let mut seed = 12345u32;
        for lane in 0..4u32 {
            seed = seed.wrapping_mul(0x0019_660d).wrapping_add(0x3c6e_f35f);
            let expected = (f64::from(seed) / 4294967296.0) as f32;
            assert_eq!(e.mem.f32(out + lane * 4), expected, "lane {lane}");
        }
        assert_eq!(e.mem.u32(this + 4), seed);
    }

    #[test]
    fn fn_00d5d660_covers_both_signs_of_the_seed() {
        // The seeds after the first step: 0 -> 0x3c6ef35f (positive), 0x7fffffff -> negative.
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(TWO_POW_32, 0x4f80_0000);
        e.mem.set_f64(TWO_POW_MINUS_32, 2.0f64.powi(-32));
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7fff_ffff);
        e.call(0x00d5_d660, &args![this, out]);
        let first = 0x7fff_ffffu32
            .wrapping_mul(0x0019_660d)
            .wrapping_add(0x3c6e_f35f);
        assert!((first as i32) < 0);
        assert_eq!(e.mem.f32(out), (f64::from(first) / 4294967296.0) as f32);
    }
}
