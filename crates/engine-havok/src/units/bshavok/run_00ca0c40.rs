//! `BSHavok/run_00ca0c40` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ca0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, two words (`RET 8`), SSE (all arithmetic in `f32`
/// per lane). With `d = point - [this + 0x140]`, `a = [this + 0x1c0]` and
/// `b = [this + 0x1b0]` (four floats each) it writes `b + a x d` to `out`:
/// the cross product of `a` and `d` in the first three lanes, and in the
/// fourth lane `a3*d3 - a3*d3` (which is 0, or NaN for a non-finite
/// product). The code stores the cross product first and then reads `b`,
/// and so does this.
pub fn fn_00ca0c40(e: &mut Engine, this: Ptr, point: Ptr, out: Ptr) {
    let read = |e: &Engine, base: u32| -> [f32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.f32(base + offset))
    };
    let origin = read(e, this.addr() + 0x140);
    let point = read(e, point.addr());
    let axis = read(e, this.addr() + 0x1c0);
    use crate::sse::{addps, mulps, subps};
    // `SUBPS`, `MULPS`, `ADDPS` with the SSE NaN rules, the destination operand
    // first: `(axis.yzxw * d.zxyw) - (axis.zxyw * d.yzxw)`, whose fourth lane is
    // `a3*d3 - a3*d3` (0, or NaN).
    let d = subps(point, origin);
    let yzxw = |v: [f32; 4]| [v[1], v[2], v[0], v[3]];
    let zxyw = |v: [f32; 4]| [v[2], v[0], v[1], v[3]];
    let cross = subps(mulps(yzxw(axis), zxyw(d)), mulps(zxyw(axis), yzxw(d)));
    for (i, lane) in cross.iter().enumerate() {
        e.mem.set_f32(out.addr() + 4 * i as u32, *lane);
    }
    let base = read(e, this.addr() + 0x1b0);
    for (i, lane) in addps(base, cross).iter().enumerate() {
        e.mem.set_f32(out.addr() + 4 * i as u32, *lane);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ca0c40, fn_00ca0c40(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put_floats_00ca0c40(e: &mut Engine, addr: u32, values: [f32; 4]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *value);
        }
    }

    #[test]
    fn fn_00ca0c40_adds_the_cross_product_to_the_base() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        put_floats_00ca0c40(&mut e, this + 0x140, [1.0, 1.0, 1.0, 0.0]);
        put_floats_00ca0c40(&mut e, this + 0x1b0, [10.0, 20.0, 30.0, 40.0]);
        put_floats_00ca0c40(&mut e, this + 0x1c0, [0.0, 0.0, 2.0, 5.0]);
        let point = e.mem.alloc(16);
        put_floats_00ca0c40(&mut e, point, [4.0, 6.0, 1.0, 7.0]);
        let out = e.mem.alloc(16);
        e.call(
            0x00ca_0c40,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(point),
                Ptr::<()>::new(out)
            ],
        );
        // d = (3, 5, 0, 7); a x d = (0*0 - 2*5, 2*3 - 0*0, 0*5 - 0*3, 0).
        let result = [0, 4, 8, 12].map(|offset| e.mem.f32(out + offset));
        assert_eq!(result, [0.0, 26.0, 30.0, 40.0]);
    }

    #[test]
    fn fn_00ca0c40_fourth_lane_is_nan_for_a_non_finite_product() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        put_floats_00ca0c40(&mut e, this + 0x1c0, [0.0, 0.0, 0.0, f32::INFINITY]);
        let point = e.mem.alloc(16);
        put_floats_00ca0c40(&mut e, point, [0.0, 0.0, 0.0, 1.0]);
        let out = e.mem.alloc(16);
        e.call(
            0x00ca_0c40,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(point),
                Ptr::<()>::new(out)
            ],
        );
        assert!(e.mem.f32(out + 12).is_nan());
    }
}
