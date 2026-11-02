//! `fallout shared/sky/run_00633b40` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00633b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A point times a scalar (no PDB name; an `NiPoint3` operator): writes
/// `this * scale` (each of the three floats multiplied, x87, stored as
/// floats) to `result` and returns `result`.
pub fn fn_00633b40(e: &mut Engine, this: Ptr, result: Ptr, scale: f32) -> Ptr {
    let mut product = [0u32; 3];
    for (i, word) in product.iter_mut().enumerate() {
        let component = e.mem.u32(this.addr() + 4 * i as u32);
        *word = x87_store32(x87_mul(x87_load32(component), x87_m32(scale.to_bits())));
    }
    for (i, word) in product.iter().enumerate() {
        e.mem.set_u32(result.addr() + 4 * i as u32, *word);
    }
    result
}

// Translated from 00633ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears the bits `mask` of the 16-bit flags at
/// `this + 0x18` (no PDB name; the moon passes `0x2000`).
pub fn fn_00633ba0(e: &mut Engine, this: Ptr, set: bool, mask: u16) {
    let flags = e.mem.u16(this.addr() + 0x18);
    if set {
        e.mem.set_u16(this.addr() + 0x18, flags | mask);
    } else {
        e.mem.set_u16(this.addr() + 0x18, flags & !mask);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_3b40, fn_00633b40(Ptr, Ptr, f32) -> Ptr),
        entry!(0x0063_3ba0, fn_00633ba0(Ptr, bool, u16)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00633b40_scales_the_point() {
        let mut e = Engine::new();
        let point = e.mem.alloc(12);
        let result = e.mem.alloc(12);
        for (i, v) in [1.0f32, -2.0, 0.5].iter().enumerate() {
            e.mem.set_f32(point + 4 * i as u32, *v);
        }
        assert_eq!(
            e.call(0x0063_3b40, &args![point, result, 4.0f32]).u32(),
            result
        );
        let scaled: Vec<f32> = (0..3).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(scaled, vec![4.0, -8.0, 2.0]);
        // The point itself is left as it was.
        assert_eq!(e.mem.f32(point), 1.0);
    }

    #[test]
    fn fn_00633ba0_sets_and_clears_the_mask() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 0x18, 0x0001);
        e.call(0x0063_3ba0, &args![this, true, 0x2000u32]);
        assert_eq!(e.mem.u16(this + 0x18), 0x2001);
        e.call(0x0063_3ba0, &args![this, false, 0x2001u32]);
        assert_eq!(e.mem.u16(this + 0x18), 0);
    }
}
