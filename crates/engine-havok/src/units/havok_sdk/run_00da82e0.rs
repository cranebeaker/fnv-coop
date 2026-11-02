//! `Havok SDK/run_00da82e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{addss, load_vec, mulps, shufps, store_vec, subps};

// Translated from 00da82e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `eax` and `ecx` (vector pointers `p` and `q`), then the stack word `weights`
/// (a vector pointer), cdecl, the result in `ST0`: `s * |s|` for `s = z + (y + x)` of
/// `(p - q) * weights` (`SUBPS`, `MULPS`, the lanes summed with `ADDSS`). The exe stores `s` as a
/// float, loads it twice, stores `|s|`, multiplies by it and stores and reloads the product, so
/// the result is `s * |s|` rounded to a float, with the x87 choice of NaN. The exe's name for it
/// is unknown (a signed squared weighted distance along a plane normal).
pub fn fn_00da82e0(e: &mut Engine, eax: u32, ecx: u32, weights: u32) -> f32 {
    let p = load_vec(e, eax);
    let q = load_vec(e, ecx);
    let w = load_vec(e, weights);
    let weighted = mulps(subps(p, q), w);
    let sum = addss(shufps(weighted, weighted, 0x55), weighted);
    let total = addss(shufps(weighted, weighted, 0xaa), sum);
    let stored = total[0].to_bits();
    let value = x87_load32(stored);
    let magnitude = f64::from_bits(value.to_bits() & 0x7fff_ffff_ffff_ffff);
    let magnitude_bits = x87_store32(magnitude);
    let product = x87_mul(value, x87_m32(magnitude_bits));
    f32::from_bits(x87_quiet_bits(x87_store32(product)))
}

// Translated from 00da8350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `eax` (the output vector) and `ecx` (vector `p`), then the stack words `(q,
/// normal, copy)` (vector pointers), cdecl: copies `normal` to `copy`, multiplies it by `p - q`
/// (`MULPS` with `normal` first) and stores `z + (y + x)` of the product in lane 0 of `eax`
/// (the other lanes are its `z`: the exe's `SHUFPS 0xaa` leaves `z` in all four lanes and the
/// `ADDSS` only changes lane 0). The exe's name for it is unknown (a plane equation).
pub fn fn_00da8350(e: &mut Engine, eax: u32, ecx: u32, q: u32, normal: u32, copy: u32) {
    let p = load_vec(e, ecx);
    let q = load_vec(e, q);
    let difference = subps(p, q);
    let n = load_vec(e, normal);
    store_vec(e, copy, n);
    let product = mulps(n, difference);
    let sum = addss(shufps(product, product, 0x55), product);
    let total = addss(shufps(product, product, 0xaa), sum);
    store_vec(e, eax, total);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_82e0, fn_00da82e0(u32, u32, u32) -> f32),
        entry!(0x00da_8350, fn_00da8350(u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use crate::units::havok_sdk::run_00d6bf60::Vec4;

    fn vector(e: &mut Engine, v: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, v);
        address
    }

    #[test]
    fn the_signed_square_keeps_the_sign_of_the_weighted_sum() {
        let mut e = Engine::new();
        let q = vector(&mut e, [1.0, 2.0, 3.0, 9.0]);
        let w = vector(&mut e, [2.0, 1.0, 0.5, 7.0]);
        let p = vector(&mut e, [5.0, 7.0, 9.0, 4.0]);
        assert_eq!(e.call(0x00da_82e0, &args![p, q, w]).st0, 256.0);
        let p = vector(&mut e, [-3.0, -3.0, -3.0, 4.0]);
        // (-4, -5, -6) * w = -8 - 5 - 3.
        assert_eq!(e.call(0x00da_82e0, &args![p, q, w]).st0, -256.0);
    }

    #[test]
    fn the_lanes_are_added_z_first_after_y_and_x() {
        let mut e = Engine::new();
        let q = vector(&mut e, [0.0; 4]);
        let w = vector(&mut e, [1.0; 4]);
        // z + (y + x) loses the 1 (1 - 1e8 rounds to -1e8); (z + x) + y would keep it.
        let p = vector(&mut e, [-1.0e8, 1.0, 1.0e8, 0.0]);
        assert_eq!(e.call(0x00da_82e0, &args![p, q, w]).st0, 0.0);
    }

    #[test]
    fn a_nan_weight_gives_a_nan() {
        let mut e = Engine::new();
        let q = vector(&mut e, [0.0; 4]);
        let w = vector(&mut e, [f32::NAN, 1.0, 1.0, 1.0]);
        let p = vector(&mut e, [1.0; 4]);
        assert!(e.call(0x00da_82e0, &args![p, q, w]).st0.is_nan());
    }

    #[test]
    fn the_square_is_rounded_to_a_float() {
        let mut e = Engine::new();
        let q = vector(&mut e, [0.0; 4]);
        let w = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let p = vector(&mut e, [1.0e20, 0.0, 0.0, 0.0]);
        // 1e40 is out of the float range.
        assert_eq!(e.call(0x00da_82e0, &args![p, q, w]).st0, f64::INFINITY);
        let p = vector(&mut e, [3.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x00da_82e0, &args![p, q, w]).st0, 9.0);
    }

    #[test]
    fn the_plane_function_copies_the_normal_and_sums_the_product() {
        let mut e = Engine::new();
        let p = vector(&mut e, [5.0, 7.0, 9.0, 1.0]);
        let q = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let normal = vector(&mut e, [2.0, 3.0, 4.0, 6.0]);
        let copy = vector(&mut e, [0.0; 4]);
        let out = vector(&mut e, [0.0; 4]);
        e.call(0x00da_8350, &args![out, p, q, normal, copy]);
        // (4, 5, 6) * (2, 3, 4): 8, 15, 24.
        assert_eq!(load_vec(&e, out), [47.0, 24.0, 24.0, 24.0]);
        assert_eq!(load_vec(&e, copy), [2.0, 3.0, 4.0, 6.0]);
    }

    #[test]
    fn a_nan_in_the_normal_is_kept_with_its_payload() {
        let mut e = Engine::new();
        let p = vector(&mut e, [1.0, 1.0, 1.0, 0.0]);
        let q = vector(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let normal = vector(&mut e, [f32::from_bits(0x7fc0_0001), 1.0, 1.0, 1.0]);
        let copy = vector(&mut e, [0.0; 4]);
        let out = vector(&mut e, [0.0; 4]);
        e.call(0x00da_8350, &args![out, p, q, normal, copy]);
        let result = load_vec(&e, out);
        assert_eq!(result[0].to_bits(), 0x7fc0_0001);
        assert_eq!(result[1], 1.0);
    }
}
