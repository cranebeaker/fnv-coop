//! `NiMain/run_00a7ec90` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a7ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `count` scaled points to `count` points (the engine map has no
/// name; `cdecl`): `dest[i] += source[i] * scale` for each `NiPoint3`, each
/// product stored as a `float` before the sums (x87). The exe unrolls the
/// loop by four while at least four points remain (signed count), then
/// finishes one by one; the unrolled and the single-point code put the
/// operands of some `FADD`s and the tail's first `FMUL` in a different
/// order, which only matters for which of two NaNs comes out, and is kept.
pub fn fn_00a7ec90(e: &mut Engine, dest: Ptr, scale: f32, source: Ptr, count: u32) {
    // `true`: `FLD dest; FADD product`; `false`: `FLD product; FADD dest`.
    const UNROLLED_ORDER: [[bool; 3]; 4] = [
        [true, true, true],
        [true, false, true],
        [false, true, true],
        [true, true, false],
    ];
    const TAIL_ORDER: [bool; 3] = [false, true, true];
    let scale = x87_load32(scale.to_bits());
    let unrolled = if count as i32 >= 4 {
        ((count - 4) >> 2).wrapping_add(1).wrapping_mul(4)
    } else {
        0
    };
    let mut index = 0u32;
    while index < count {
        let point_source = source.addr().wrapping_add(index.wrapping_mul(12));
        let point_dest = dest.addr().wrapping_add(index.wrapping_mul(12));
        let in_tail = index >= unrolled;
        let mut products = [0u32; 3];
        for (i, product) in products.iter_mut().enumerate() {
            let component = e.mem.u32(point_source + 4 * i as u32);
            let value = if in_tail && i == 0 {
                x87_mul(scale, x87_m32(component))
            } else {
                x87_mul(x87_load32(component), scale)
            };
            *product = x87_store32(value);
        }
        let order = if in_tail {
            TAIL_ORDER
        } else {
            UNROLLED_ORDER[(index % 4) as usize]
        };
        for (i, product) in products.iter().enumerate() {
            let address = point_dest + 4 * i as u32;
            let current = e.mem.u32(address);
            let sum = if order[i] {
                x87_add(x87_load32(current), x87_m32(*product))
            } else {
                x87_add(x87_load32(*product), x87_m32(current))
            };
            e.mem.set_u32(address, x87_store32(sum));
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_ec90, fn_00a7ec90(Ptr, f32, Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaled_points_are_added_one_by_one() {
        let mut e = Engine::new();
        // Five points: one unrolled group of four, then one more.
        let source = e.mem.alloc(5 * 12);
        let dest = e.mem.alloc(6 * 12);
        for i in 0..15 {
            e.mem.set_f32(source + 4 * i, i as f32);
            e.mem.set_f32(dest + 4 * i, 100.0);
        }
        e.mem.set_f32(dest + 60, 7.0);
        e.call(0x00a7_ec90, &args![dest, 0.5f32, source, 5u32]);
        for i in 0..15 {
            assert_eq!(e.mem.f32(dest + 4 * i), 100.0 + i as f32 * 0.5);
        }
        assert_eq!(e.mem.f32(dest + 60), 7.0);
    }

    #[test]
    fn scaled_points_quiet_a_signalling_nan() {
        // A signalling NaN in the source comes out quiet.
        let mut e = Engine::new();
        let source = e.mem.alloc(12);
        let dest = e.mem.alloc(12);
        e.mem.set_u32(source, 0x7f80_0001);
        e.call(0x00a7_ec90, &args![dest, 2.0f32, source, 1u32]);
        assert_eq!(e.mem.u32(dest), 0x7fc0_0001);
        assert_eq!(e.mem.f32(dest + 4), 0.0);
    }
}
