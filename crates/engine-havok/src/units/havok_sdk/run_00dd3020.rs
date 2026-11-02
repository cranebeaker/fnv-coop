//! `Havok SDK/run_00dd3020` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{add4, load_bits, load_vec4, mul4, Vec4};

/// `ANDPS`/`ANDNPS`/`ORPS` select: `(mask & value) | (!mask & old)` on the bits of each lane.
fn blend(mask: [u32; 4], value: Vec4, old: [u32; 4]) -> [u32; 4] {
    [0, 1, 2, 3].map(|i| (mask[i] & value[i].to_bits()) | (!mask[i] & old[i]))
}

fn store_bits(e: &mut Engine, at: u32, bits: [u32; 4]) {
    for (i, word) in bits.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, word);
    }
}

// Translated from 00dd3020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `out_b` (`EAX`), `mask_b` (`EDX`) and `mask_a` (`ESI`) (each a pointer to 16-byte vectors);
/// then the cdecl stack words `p`, `q` (three 16-byte vectors each), `out_a` (three vectors
/// written), `r` and `s` (one 16-byte vector each).
///
/// For vector `k` of 0 to 2 the value is `v_k = q[k] * r + s * p[k]` (`k = 0`: `q[0]` times `r`
/// plus `s` times `p[0]`; for `k` 1 and 2 `p[k] * s + q[k] * r`: the order of the exe's `MULPS`
/// operands) and each lane of `out_a[k]` becomes `v_k` where the corresponding lane of the
/// 16-byte mask at `mask_a` has its bits set and keeps its old bits where it does not;
/// `out_b[k]` is blended the same way with `mask_b`. (`ANDPS`/`ANDNPS`/`ORPS` on the bits.)
pub fn fn_00dd3020(
    e: &mut Engine,
    out_b: u32,
    mask_b: u32,
    mask_a: u32,
    p: u32,
    q: u32,
    out_a: u32,
    r: u32,
    s: u32,
) {
    // k = 0
    let old_a = load_bits(e, out_a);
    let x = load_vec4(e, q);
    let y = load_vec4(e, r);
    let z = load_vec4(e, p);
    let first = mul4(x, y);
    let w = load_vec4(e, s);
    let value = add4(first, mul4(w, z));
    let mask = load_bits(e, mask_a);
    store_bits(e, out_a, blend(mask, value, old_a));
    let mask = load_bits(e, mask_b);
    let old_b = load_bits(e, out_b);
    store_bits(e, out_b, blend(mask, value, old_b));
    // k = 1 and 2
    for k in 1..3u32 {
        let at = 0x10 * k;
        let x = load_vec4(e, p + at);
        let y = load_vec4(e, s);
        let old_a = load_bits(e, out_a + at);
        let first = mul4(x, y);
        let z = load_vec4(e, q + at);
        let w = load_vec4(e, r);
        let value = add4(first, mul4(z, w));
        let mask = load_bits(e, mask_a);
        store_bits(e, out_a + at, blend(mask, value, old_a));
        let mask = load_bits(e, mask_b);
        let old_b = load_bits(e, out_b + at);
        store_bits(e, out_b + at, blend(mask, value, old_b));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dd3020,
        fn_00dd3020(u32, u32, u32, u32, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkgeometryprocessing::store_vec4;

    fn vectors(e: &mut Engine, list: &[Vec4]) -> u32 {
        let at = e.mem.alloc(16 * list.len() as u32);
        for (i, v) in list.iter().enumerate() {
            store_vec4(e, at + 16 * i as u32, *v);
        }
        at
    }

    fn mask(e: &mut Engine, lanes: [bool; 4]) -> u32 {
        let at = e.mem.alloc(16);
        for (i, on) in lanes.into_iter().enumerate() {
            e.mem
                .set_u32(at + 4 * i as u32, if on { 0xffff_ffff } else { 0 });
        }
        at
    }

    #[test]
    fn each_output_takes_the_new_value_only_in_the_masked_lanes() {
        let mut e = Engine::new();
        let p = vectors(&mut e, &[[2.0; 4], [1.0; 4], [1.0, 0.0, 1.0, 0.0]]);
        let q = vectors(&mut e, &[[1.0; 4], [2.0; 4], [0.0, 1.0, 0.0, 1.0]]);
        let r = vectors(&mut e, &[[1.0, 2.0, 3.0, 4.0]]);
        let s = vectors(&mut e, &[[10.0, 20.0, 30.0, 40.0]]);
        let out_a = vectors(&mut e, &[[-1.0; 4]; 3]);
        let out_b = vectors(&mut e, &[[-2.0; 4]; 3]);
        let mask_a = mask(&mut e, [true, false, true, false]);
        let mask_b = mask(&mut e, [false, true, false, true]);
        e.call(
            0x00dd_3020,
            &args![out_b, mask_b, mask_a, p, q, out_a, r, s],
        );
        // q[0] * r + s * p[0] = (21, 42, 63, 84); p[1] * s + q[1] * r = (12, 24, 36, 48);
        // p[2] * s + q[2] * r = (10, 2, 30, 4).
        let read = |e: &Engine, at: u32, k: u32| load_vec4(e, at + 16 * k);
        assert_eq!(read(&e, out_a, 0), [21.0, -1.0, 63.0, -1.0]);
        assert_eq!(read(&e, out_b, 0), [-2.0, 42.0, -2.0, 84.0]);
        assert_eq!(read(&e, out_a, 1), [12.0, -1.0, 36.0, -1.0]);
        assert_eq!(read(&e, out_b, 1), [-2.0, 24.0, -2.0, 48.0]);
        assert_eq!(read(&e, out_a, 2), [10.0, -1.0, 30.0, -1.0]);
        assert_eq!(read(&e, out_b, 2), [-2.0, 2.0, -2.0, 4.0]);
    }

    #[test]
    fn an_all_zero_mask_keeps_the_old_bits_even_for_a_nan() {
        let mut e = Engine::new();
        let p = vectors(&mut e, &[[f32::NAN; 4]; 3]);
        let q = vectors(&mut e, &[[1.0; 4]; 3]);
        let r = vectors(&mut e, &[[1.0; 4]]);
        let s = vectors(&mut e, &[[1.0; 4]]);
        let out_a = vectors(&mut e, &[[7.0; 4]; 3]);
        let out_b = vectors(&mut e, &[[8.0; 4]; 3]);
        let off = mask(&mut e, [false; 4]);
        e.call(0x00dd_3020, &args![out_b, off, off, p, q, out_a, r, s]);
        for k in 0..3 {
            assert_eq!(load_vec4(&e, out_a + 16 * k), [7.0; 4]);
            assert_eq!(load_vec4(&e, out_b + 16 * k), [8.0; 4]);
        }
    }
}
