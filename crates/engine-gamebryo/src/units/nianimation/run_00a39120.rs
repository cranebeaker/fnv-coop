//! `NiAnimation/run_00a39120` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Sets a quaternion from four `float`s (`00a6db50`, thiscall on the
/// result, `RET 0x10`; unnamed in the engine map).
const QUATERNION_SET_00A6DB50: u32 = 0x00a6_db50;

// Translated from 00a39120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quaternion at `this` scaled by `scale` into `out` (thiscall,
/// `RET 8`; unnamed in the engine map): each of the four components
/// multiplied by `scale` on the x87 (last to first, each stored as a
/// `float`), then `00a6db50(out, c0, c1, c2, c3)` with the products in
/// component order. Returns `out`.
pub fn fn_00a39120(e: &mut Engine, this: Ptr, out: Ptr, scale: f32) -> Ptr {
    let q = this.addr();
    let t = x87_load32(scale.to_bits());
    let component = |e: &Engine, i: u32| e.mem.u32(q + 4 * i);
    // FMULP ST2: the last component times the scale.
    let last = x87_store32(x87_mul(x87_load32(component(e, 3)), t));
    // FMUL ST0,ST1: component 2, then 1, times the scale.
    let third = x87_store32(x87_mul(x87_load32(component(e, 2)), t));
    let second = x87_store32(x87_mul(x87_load32(component(e, 1)), t));
    // FMUL m32: the scale times component 0.
    let first = x87_store32(x87_mul(t, x87_m32(component(e, 0))));
    e.call(
        QUATERNION_SET_00A6DB50,
        &args![out, first, second, third, last],
    );
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_9120, fn_00a39120(Ptr, Ptr, f32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_each_component_into_the_result() {
        let mut e = Engine::new();
        e.register(QUATERNION_SET_00A6DB50, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        let this = e.mem.alloc(16);
        for (i, value) in [1.0f32, -2.0, 0.5, 3.0].iter().enumerate() {
            e.mem.set_f32(this + 4 * i as u32, *value);
        }
        let out = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00a3_9120, &args![this, out, 2.0f32.to_bits()])
                .u32(),
            out
        );
        let result: Vec<f32> = (0..4).map(|i| e.mem.f32(out + 4 * i)).collect();
        assert_eq!(result, vec![2.0, -4.0, 1.0, 6.0]);
        // A signalling NaN component comes out quieted.
        e.mem.set_u32(this + 8, 0x7f80_0001);
        e.call(0x00a3_9120, &args![this, out, 2.0f32.to_bits()]);
        assert_eq!(e.mem.u32(out + 8), 0x7fc0_0001);
    }
}
