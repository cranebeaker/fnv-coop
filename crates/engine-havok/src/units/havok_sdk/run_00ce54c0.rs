//! `Havok SDK/run_00ce54c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{mulps, splat};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec};

/// The initial minimum (`0104da48`, `3.4028e38`) and maximum (`0104e148`, the same negated) that
/// `00ce5560` splats into its two result vectors.
const MINIMUM_START: u32 = 0x0104_da48;
const MAXIMUM_START: u32 = 0x0104_e148;

// Translated from 00ce54c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(object, scale)`: multiplies by `scale` (all four lanes, `splat(scale) * vector`) the
/// vector at `+0x10` of each `0x40`-byte record of the array at `object + 0x18` (count: the signed
/// word at `+0x1c`), then the vectors at `+0x10` and `+0x40` of each `0x70`-byte record of the
/// array at `object + 0x24` (count at `+0x28`). The data pointers and the counts are read again
/// for every record, as the exe does. The class is unconfirmed.
pub fn fn_00ce54c0(e: &mut Engine, object: Ptr, scale: f32) {
    let base = object.addr();
    let scale = splat(scale);
    let mut index = 0;
    let mut offset = 0u32;
    if e.mem.i32(base + 0x1c) > 0 {
        loop {
            let at = e
                .mem
                .u32(base + 0x18)
                .wrapping_add(offset)
                .wrapping_add(0x10);
            let vector = load_vec(e, at);
            store_vec(e, at, mulps(scale, vector));
            index += 1;
            offset = offset.wrapping_add(0x40);
            if index >= e.mem.i32(base + 0x1c) {
                break;
            }
        }
    }
    let mut index = 0;
    let mut offset = 0u32;
    if e.mem.i32(base + 0x28) > 0 {
        loop {
            let at = e
                .mem
                .u32(base + 0x24)
                .wrapping_add(offset)
                .wrapping_add(0x10);
            let vector = load_vec(e, at);
            store_vec(e, at, mulps(scale, vector));
            let at = e
                .mem
                .u32(base + 0x24)
                .wrapping_add(offset)
                .wrapping_add(0x40);
            let vector = load_vec(e, at);
            store_vec(e, at, mulps(scale, vector));
            index += 1;
            offset = offset.wrapping_add(0x70);
            if index >= e.mem.i32(base + 0x28) {
                break;
            }
        }
    }
}

// Translated from 00ce5560 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(vectors, count, minimum, maximum)`: the vectors at `0x30`-byte intervals from `vectors`
/// (`count`, signed) are folded into a lane-wise minimum and maximum. `minimum` starts as the float
/// at `0104da48` in every lane and `maximum` as the float at `0104e148`; for each vector `v` the
/// minimum becomes `MINPS(minimum, v)` (`minimum` where `minimum < v`, otherwise `v`, so a NaN
/// or an equal pair gives `v`) and the maximum `MAXPS(maximum, v)` (`maximum` where `maximum >
/// v`, otherwise `v`), both stored every step.
pub fn fn_00ce5560(e: &mut Engine, vectors: Ptr, count: i32, minimum: Ptr, maximum: Ptr) {
    let low = e.mem.f32(MINIMUM_START);
    store_vec(e, minimum.addr(), splat(low));
    let high = e.mem.f32(MAXIMUM_START);
    store_vec(e, maximum.addr(), splat(high));
    let mut cursor = vectors.addr();
    let mut remaining = count;
    if remaining > 0 {
        loop {
            let vector = load_vec(e, cursor);
            let current = load_vec(e, minimum.addr());
            let folded: [f32; 4] = std::array::from_fn(|lane| {
                if current[lane] < vector[lane] {
                    current[lane]
                } else {
                    vector[lane]
                }
            });
            store_vec(e, minimum.addr(), folded);
            let vector = load_vec(e, cursor);
            let current = load_vec(e, maximum.addr());
            cursor = cursor.wrapping_add(0x30);
            remaining -= 1;
            let folded: [f32; 4] = std::array::from_fn(|lane| {
                if current[lane] > vector[lane] {
                    current[lane]
                } else {
                    vector[lane]
                }
            });
            store_vec(e, maximum.addr(), folded);
            if remaining == 0 {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_54c0, fn_00ce54c0(Ptr, f32)),
        entry!(0x00ce_5560, fn_00ce5560(Ptr, i32, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(MINIMUM_START, 0x7f7f_ffee);
        e.mem.set_u32(MAXIMUM_START, 0xff7f_ffee);
        e
    }

    fn put(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (lane, value) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
    }

    fn get(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }

    /// Only the named vectors of the 0x40- and 0x70-byte records are scaled, in all lanes.
    #[test]
    fn scales_the_named_vectors_of_both_record_arrays() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        let first = e.mem.alloc(0x40 * 2);
        let second = e.mem.alloc(0x70 * 2);
        e.mem.set_u32(object + 0x18, first);
        e.mem.set_u32(object + 0x1c, 2);
        e.mem.set_u32(object + 0x24, second);
        e.mem.set_u32(object + 0x28, 2);
        for record in 0..2u32 {
            put(&mut e, first + 0x40 * record, [9.0; 4]);
            put(&mut e, first + 0x40 * record + 0x10, [1.0, 2.0, 3.0, 4.0]);
            put(&mut e, second + 0x70 * record + 0x10, [1.0, 2.0, 3.0, 4.0]);
            put(&mut e, second + 0x70 * record + 0x40, [5.0, 6.0, 7.0, 8.0]);
            put(&mut e, second + 0x70 * record + 0x20, [9.0; 4]);
        }
        e.call(0x00ce_54c0, &args![object, 2.0f32]);
        for record in 0..2u32 {
            assert_eq!(get(&e, first + 0x40 * record + 0x10), [2.0, 4.0, 6.0, 8.0]);
            assert_eq!(get(&e, first + 0x40 * record), [9.0; 4]);
            assert_eq!(get(&e, second + 0x70 * record + 0x10), [2.0, 4.0, 6.0, 8.0]);
            assert_eq!(
                get(&e, second + 0x70 * record + 0x40),
                [10.0, 12.0, 14.0, 16.0]
            );
            assert_eq!(get(&e, second + 0x70 * record + 0x20), [9.0; 4]);
        }
        // Counts of zero or less change nothing.
        e.mem.set_u32(object + 0x1c, 0);
        e.mem.set_u32(object + 0x28, 0xffff_ffff);
        e.call(0x00ce_54c0, &args![object, 2.0f32]);
        assert_eq!(get(&e, first + 0x10), [2.0, 4.0, 6.0, 8.0]);
        assert_eq!(get(&e, second + 0x10), [2.0, 4.0, 6.0, 8.0]);
    }

    /// The bounds fold lane by lane over vectors `0x30` bytes apart; with no vectors the start
    /// values are written; a NaN lane takes the vector's value (the second operand).
    #[test]
    fn bounds_are_folded_lane_by_lane() {
        let mut e = engine();
        let vectors = e.mem.alloc(0x30 * 3);
        put(&mut e, vectors, [1.0, -5.0, 3.0, 0.0]);
        put(&mut e, vectors + 0x30, [2.0, -6.0, 1.0, 0.0]);
        put(&mut e, vectors + 0x60, [0.5, -1.0, 9.0, f32::NAN]);
        let low = e.mem.alloc(0x10);
        let high = e.mem.alloc(0x10);
        e.call(0x00ce_5560, &args![vectors, 3i32, low, high]);
        let min = get(&e, low);
        let max = get(&e, high);
        assert_eq!(min[..3], [0.5, -6.0, 1.0]);
        assert_eq!(max[..3], [2.0, -1.0, 9.0]);
        assert!(min[3].is_nan() && max[3].is_nan());
        e.call(0x00ce_5560, &args![vectors, 0i32, low, high]);
        assert_eq!(e.mem.u32(low), 0x7f7f_ffee);
        assert_eq!(e.mem.u32(high + 12), 0xff7f_ffee);
    }
}
