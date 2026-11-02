//! `Havok SDK/run_00dd31b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{load_vec4, mul4};

// Translated from 00dd31b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first: `a`
/// (`ECX`, a 16-byte vector) and `out` (`EAX`, a 16-byte area); then the cdecl stack word `b` (a
/// 16-byte vector; not popped by the callee). Writes at `out` a mask per lane: all ones where
/// `a * b` is below 0.0 (`CMPLTPS`: false for a NaN and for -0.0), else 0.
pub fn fn_00dd31b0(e: &mut Engine, a: u32, out: u32, b: u32) {
    let vb = load_vec4(e, b);
    let va = load_vec4(e, a);
    let product = mul4(va, vb);
    for (i, lane) in product.into_iter().enumerate() {
        let mask: u32 = if lane < 0.0 { 0xffff_ffff } else { 0 };
        e.mem.set_u32(out + 4 * i as u32, mask);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dd31b0, fn_00dd31b0(u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkgeometryprocessing::store_vec4;

    #[test]
    fn a_lane_is_all_ones_only_when_the_product_is_negative() {
        let mut e = Engine::new();
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        store_vec4(&mut e, a, [1.0, -1.0, -0.0, f32::NAN]);
        store_vec4(&mut e, b, [2.0, 2.0, 1.0, 1.0]);
        e.call(0x00dd_31b0, &args![a, out, b]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0, 0xffff_ffff, 0, 0]);
        // A negative times a positive infinity is negative; two negatives are not.
        store_vec4(&mut e, a, [-1.0, -1.0, 3.0, 0.5]);
        store_vec4(&mut e, b, [f32::INFINITY, -2.0, -f32::INFINITY, 4.0]);
        e.call(0x00dd_31b0, &args![a, out, b]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0xffff_ffff, 0, 0xffff_ffff, 0]);
    }
}
