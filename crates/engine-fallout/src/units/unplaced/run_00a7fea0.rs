//! `(unplaced)/run_00a7fea0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a7fea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves an `NiBound` (centre +0, radius +0xC) by an `NiTransform`
/// (rotation rows +0 to +0x20, translation +0x24, scale +0x30): the centre
/// becomes rotation times centre, times the scale, plus the translation;
/// the radius is multiplied by the scale. Each x87 step follows the exe
/// (the first row sums its second product first).
pub fn fn_00a7fea0(e: &mut Engine, this: Ptr, transform: Ptr) {
    let bound = this.addr();
    let t = transform.addr();
    let m = |e: &Engine, i: u32| x87_load32(e.mem.u32(t + 4 * i));
    let c = |e: &Engine, i: u32| x87_m32(e.mem.u32(bound + 4 * i));
    let row0 = x87_add(x87_mul(m(e, 1), c(e, 1)), x87_mul(m(e, 0), c(e, 0)));
    let row0 = x87_store32(x87_add(row0, x87_mul(m(e, 2), c(e, 2))));
    let row1 = x87_add(x87_mul(m(e, 3), c(e, 0)), x87_mul(m(e, 4), c(e, 1)));
    let row1 = x87_store32(x87_add(row1, x87_mul(m(e, 5), c(e, 2))));
    let row2 = x87_add(x87_mul(m(e, 6), c(e, 0)), x87_mul(m(e, 7), c(e, 1)));
    let row2 = x87_store32(x87_add(row2, x87_mul(m(e, 8), c(e, 2))));
    let scale = x87_load32(x87_quiet_bits(e.mem.u32(t + 0x30)));
    let scaled0 = x87_store32(x87_mul(x87_load32(row0), scale));
    let scaled1 = x87_store32(x87_mul(x87_load32(row1), scale));
    let scaled2 = x87_store32(x87_mul(scale, x87_m32(row2)));
    let x = x87_store32(x87_add(m(e, 9), x87_m32(scaled0)));
    let y = x87_store32(x87_add(m(e, 10), x87_m32(scaled1)));
    e.mem.set_u32(bound, x);
    let z = x87_store32(x87_add(m(e, 11), x87_m32(scaled2)));
    e.mem.set_u32(bound + 4, y);
    e.mem.set_u32(bound + 8, z);
    let radius = x87_store32(x87_mul(m(e, 12), c(e, 3)));
    e.mem.set_u32(bound + 0xc, radius);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7fea0, fn_00a7fea0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_is_rotated_scaled_and_moved() {
        let mut e = Engine::new();
        let bound = e.mem.alloc(0x10);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(bound + 4 * i as u32, *v);
        }
        // A rotation swapping x and y, translation (10, 20, 30), scale 2.
        let transform = e.mem.alloc(0x34);
        let values = [
            0.0f32, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0, 20.0, 30.0, 2.0,
        ];
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(transform + 4 * i as u32, *v);
        }
        e.call(0x00a7_fea0, &args![bound, transform]);
        assert_eq!(e.mem.f32(bound), 14.0);
        assert_eq!(e.mem.f32(bound + 4), 22.0);
        assert_eq!(e.mem.f32(bound + 8), 36.0);
        assert_eq!(e.mem.f32(bound + 0xc), 8.0);
    }
}
