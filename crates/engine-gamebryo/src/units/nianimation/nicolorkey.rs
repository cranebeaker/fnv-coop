//! `NiAnimation/nicolorkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nianimation::niposkey::{key_ratio, search_keys, LOWEST_TIME};

/// The colour keys' interpolation functions, one word per key type
/// (`011f3cd8`; each `cdecl (ratio, key, next key, &result)`).
const COLOR_KEY_INTERPOLATIONS: u32 = 0x011f_3cd8;

// Translated from 00a296c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorKey::GenInterp` (Xbox PDB, `cdecl`, the colour returned through
/// `result`): with one key, or at time `-FLT_MAX` (the double at
/// `010241b0`; a NaN is not equal), the first key's colour (+4, four
/// words). Otherwise the key search the `GenInterp` functions share
/// (`niposkey::search_keys`, the same code inlined here) and the key type's
/// interpolation function (`011f3cd8`) between the keys `index` and
/// `next_index` with the ratio, into a local colour (cleared to 0.0 first)
/// copied to `result`; the last index becomes `index`. Answers `result`.
pub fn ni_color_key_gen_interp(
    e: &mut Engine,
    result: Ptr,
    time: f32,
    keys: Ptr,
    key_type: u32,
    num_keys: u32,
    last_index: Ptr,
    key_size: u8,
) -> Ptr {
    let first = keys.addr().wrapping_add(4);
    if num_keys == 1 || x87_load32(time.to_bits()) == f64::from_bits(e.global(LOWEST_TIME)) {
        for word in 0..4u32 {
            let value = e.mem.u32(first.wrapping_add(4 * word));
            e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
        }
        return result;
    }
    let time = x87_load32(time.to_bits());
    let size = u32::from(key_size);
    let last = e.mem.u32(last_index.addr());
    let search = search_keys(e, time, keys.addr(), num_keys, last, size);
    let ratio = key_ratio(time, &search);
    let key = keys.addr().wrapping_add(size.wrapping_mul(search.index));
    let next_key = keys
        .addr()
        .wrapping_add(size.wrapping_mul(search.next_index));
    let function = e
        .mem
        .u32(COLOR_KEY_INTERPOLATIONS.wrapping_add(key_type.wrapping_mul(4)));
    e.with_stack(16, |e, color| {
        for word in 0..4u32 {
            e.mem.set_u32(color.addr() + 4 * word, 0.0f32.to_bits());
        }
        e.call(function, &args![ratio, key, next_key, color]);
        e.mem.set_u32(last_index.addr(), search.index);
        for word in 0..4u32 {
            let value = e.mem.u32(color.addr() + 4 * word);
            e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
        }
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a296c0,
        ni_color_key_gen_interp(Ptr, f32, Ptr, u32, u32, Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTERPOLATE: u32 = 0x7200_0000;

    /// Five keys of 0x14 bytes (time, then r, g, b, a) at times 0, 1, 2, 3, 4,
    /// the colour words `0x100 * key + channel`.
    fn keys(e: &mut Engine) -> u32 {
        let keys = e.mem.alloc(5 * 0x14);
        for k in 0..5u32 {
            e.mem.set_f32(keys + k * 0x14, k as f32);
            for c in 0..4u32 {
                e.mem.set_u32(keys + k * 0x14 + 4 + 4 * c, 0x100 * k + c);
            }
        }
        keys
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0102_4000, 0x1000);
        e.set_global(LOWEST_TIME, f64::from(-f32::MAX));
        e.map(0x011f_3000, 0x1000);
        e.set_global(COLOR_KEY_INTERPOLATIONS + 2 * 4, INTERPOLATE);
        // The double writes the ratio into red and both key addresses into
        // green and blue; alpha keeps the 0.0 the caller cleared.
        e.register(INTERPOLATE, |e, a| {
            e.mem.set_u32(a[3], a[0]);
            e.mem.set_u32(a[3] + 4, a[1]);
            e.mem.set_u32(a[3] + 8, a[2]);
            Ret::default()
        });
        e
    }

    #[test]
    fn one_key_or_the_lowest_time_copies_the_first_colour() {
        let mut e = engine();
        let keys = keys(&mut e);
        let result = e.mem.alloc(16);
        let last = e.mem.alloc(4);
        e.call(
            0x00a2_96c0,
            &args![result, 2.5f32, keys, 2u32, 1u32, last, 0x14u32],
        );
        assert_eq!(e.mem.u32(result + 12), 3);
        e.mem.set_u32(result + 12, 0);
        e.call(
            0x00a2_96c0,
            &args![result, -f32::MAX, keys, 2u32, 5u32, last, 0x14u32],
        );
        assert_eq!(e.mem.u32(result + 12), 3);
    }

    #[test]
    fn interpolates_between_the_surrounding_keys() {
        let mut e = engine();
        let keys = keys(&mut e);
        let result = e.mem.alloc(16);
        let last = e.mem.alloc(4);
        e.mem.set_u32(last, 4);
        let answer = e
            .call(
                0x00a2_96c0,
                &args![result, 2.25f32, keys, 2u32, 5u32, last, 0x14u32],
            )
            .u32();
        assert_eq!(answer, result);
        // Key 4 is after the time: the search starts at key 0.
        assert_eq!(e.mem.u32(last), 2);
        assert_eq!(e.mem.f32(result), 0.25);
        assert_eq!(e.mem.u32(result + 4), keys + 2 * 0x14);
        assert_eq!(e.mem.u32(result + 8), keys + 3 * 0x14);
        assert_eq!(e.mem.f32(result + 12), 0.0);
    }
}
