//! `NiCollision/nidistpntseg.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// A stored float loaded back (`FSTP` then `FLD`).
fn round(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

// Translated from 00eaf1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSqrDistance::Compute` (Xbox PDB, `cdecl`): the squared distance from `point` to the segment
/// (origin at +0, direction at +0xc), as a float in ST0, with the closest point's parameter in
/// `parameter`. With `d = point - origin` (stored) and `t = d . direction` (stored, written to
/// `parameter`): for `t <= 0` the parameter is 0 and `d` is kept; otherwise with
/// `l = |direction|^2` (stored) the parameter is 1 and `d -= direction` when `l <= t`, else the
/// parameter is `t / l` and `d -= parameter * direction` (each product and difference stored).
/// The answer is `|d|^2` (stored), summed `(y^2 + x^2) + z^2`.
pub fn ni_sqr_distance_compute(e: &mut Engine, point: Ptr, segment: Ptr, parameter: Ptr) -> f64 {
    let p = point.addr();
    let s = segment.addr();
    let out = parameter.addr();
    let mut d = [0u32, 4, 8].map(|offset| {
        x87_store32(x87_sub(
            load(e, p.wrapping_add(offset)),
            operand(e, s.wrapping_add(offset)),
        ))
    });
    let (dx, dy, dz) = (x87_load32(d[0]), x87_load32(d[1]), x87_load32(d[2]));
    let along = x87_add(
        x87_mul(load(e, s.wrapping_add(0x10)), dy),
        x87_mul(dx, operand(e, s.wrapping_add(0xc))),
    );
    let along = x87_add(x87_mul(load(e, s.wrapping_add(0x14)), dz), along);
    let along = round(along);
    e.mem.set_u32(out, x87_store32(along));
    if along <= 0.0 {
        e.mem.set_u32(out, 0);
    } else {
        let (ux, uy, uz) = (
            load(e, s.wrapping_add(0xc)),
            load(e, s.wrapping_add(0x10)),
            load(e, s.wrapping_add(0x14)),
        );
        let length = round(x87_add(
            x87_add(x87_mul(uy, uy), x87_mul(ux, ux)),
            x87_mul(uz, uz),
        ));
        if length <= along {
            e.mem.set_u32(out, 1.0f32.to_bits());
            for (axis, value) in [dx, dy, dz].iter().enumerate() {
                let offset = 4 * axis as u32;
                d[axis] = x87_store32(x87_sub(*value, operand(e, s.wrapping_add(0xc + offset))));
            }
        } else {
            let ratio = round(x87_div(along, length));
            e.mem.set_u32(out, x87_store32(ratio));
            let step = [
                x87_store32(x87_mul(load(e, s.wrapping_add(0xc)), ratio)),
                x87_store32(x87_mul(load(e, s.wrapping_add(0x10)), ratio)),
                x87_store32(x87_mul(ratio, operand(e, s.wrapping_add(0x14)))),
            ];
            for (axis, value) in [dx, dy, dz].iter().enumerate() {
                d[axis] = x87_store32(x87_sub(*value, x87_m32(step[axis])));
            }
        }
    }
    let (dx, dy, dz) = (x87_load32(d[0]), x87_load32(d[1]), x87_load32(d[2]));
    let sum = x87_add(x87_mul(dy, dy), x87_mul(dx, dx));
    round(x87_add(x87_mul(dz, dz), sum))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ea_f1f0,
        ni_sqr_distance_compute(Ptr, Ptr, Ptr) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    #[test]
    fn distance_to_the_segment_and_its_ends() {
        let mut e = Engine::new();
        let segment = floats(&mut e, &[0.0, 0.0, 0.0, 4.0, 0.0, 0.0]);
        let out = e.mem.alloc(4);
        let mut check = |p: [f32; 3], squared: f64, t: f32| {
            let point = floats(&mut e, &p);
            let r = e.call(0x00ea_f1f0, &args![point, segment, out]).f64();
            assert_eq!(r, squared, "{p:?}");
            assert_eq!(e.mem.f32(out), t, "{p:?}");
        };
        check([1.0, 2.0, 0.0], 4.0, 0.25);
        check([-3.0, 0.0, 4.0], 25.0, 0.0);
        check([7.0, 0.0, 4.0], 25.0, 1.0);
    }
}
