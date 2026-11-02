//! `NiCollision/nidistsegseg.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};
use crate::units::nicollision::nidistpnttri::{finish, stored, Terms, TWO_01011590};
use crate::units::nicollision::run_00eb6150::far_side_t;

/// The float `1e-6` at `010959f4`: the squared cross product of the directions at or below
/// which the segments count as parallel.
const PARALLEL_LIMIT_010959F4: u32 = 0x0109_59f4;

/// The terms of two segments (origin +0, direction +0xc) in Eberly's notation, each stored:
/// `d = o0 - o1`, `a00 = d0.d0`, `a01 = -(d1.d0)`, `a11 = d1.d1`, `b0 = d0.d`, `b1 = -(d1.d)`,
/// `c = d.d`, and `det = |a00 a11 - a01^2|`.
fn segment_terms(e: &Engine, first: u32, second: u32) -> (Terms, f64) {
    let ld = |e: &Engine, at: u32, offset: u32| load_float(e, at.wrapping_add(offset));
    let op = |e: &Engine, at: u32, offset: u32| float_operand(e, at.wrapping_add(offset));
    let [dx, dy, dz] = [0u32, 4, 8].map(|i| stored(x87_sub(ld(e, first, i), op(e, second, i))));
    let dot = |e: &Engine, u: u32, v: u32| {
        let s = x87_add(
            x87_mul(ld(e, u, 0x10), op(e, v, 0x10)),
            x87_mul(ld(e, u, 0xc), op(e, v, 0xc)),
        );
        stored(x87_add(s, x87_mul(ld(e, u, 0x14), op(e, v, 0x14))))
    };
    let a00 = dot(e, first, first);
    let a01 = stored(-dot(e, second, first));
    let a11 = dot(e, second, second);
    let s = x87_add(
        x87_mul(ld(e, first, 0xc), dx),
        x87_mul(ld(e, first, 0x10), dy),
    );
    let b0 = stored(x87_add(x87_mul(ld(e, first, 0x14), dz), s));
    let s = x87_add(
        x87_mul(ld(e, second, 0x10), dy),
        x87_mul(ld(e, second, 0xc), dx),
    );
    let b1 = stored(-stored(x87_add(s, x87_mul(ld(e, second, 0x14), dz))));
    let s = x87_add(x87_mul(dy, dy), x87_mul(dx, dx));
    let c = stored(x87_add(x87_mul(dz, dz), s));
    let det = stored(x87_sub(x87_mul(a11, a00), x87_mul(a01, a01)));
    let terms = Terms {
        a00,
        a01,
        a11,
        b0,
        b1,
        c,
    };
    (terms, stored(det.abs()))
}

/// `|d0 x d1|^2` (each coordinate of the cross product stored), stored.
fn cross_squared(e: &Engine, first: u32, second: u32) -> f64 {
    let ld = |e: &Engine, at: u32, offset: u32| load_float(e, at.wrapping_add(offset));
    let op = |e: &Engine, at: u32, offset: u32| float_operand(e, at.wrapping_add(offset));
    let x = stored(x87_sub(
        x87_mul(ld(e, first, 0x10), op(e, second, 0x14)),
        x87_mul(ld(e, first, 0x14), op(e, second, 0x10)),
    ));
    let y = stored(x87_sub(
        x87_mul(ld(e, second, 0xc), op(e, first, 0x14)),
        x87_mul(ld(e, first, 0xc), op(e, second, 0x14)),
    ));
    let z = stored(x87_sub(
        x87_mul(ld(e, first, 0xc), op(e, second, 0x10)),
        x87_mul(ld(e, second, 0xc), op(e, first, 0x10)),
    ));
    let s = x87_add(x87_mul(y, y), x87_mul(x, x));
    stored(x87_add(s, x87_mul(z, z)))
}

/// The far side `s = 1` (already written) of the segment-segment distance: with
/// `tmp = a01 + b1` (stored) not below 0, `t = 0` and `a00 + 2 b0 + c`; otherwise `-tmp`
/// (stored) below `a11` (unordered too) gives `t = -tmp / a11` and `a00 + tmp t + 2 b0 + c`,
/// else `t = 1` and `(a00 + a11 + c) + 2 (tmp + b0)`.
fn far_side_s(e: &mut Engine, terms: Terms, t_out: u32) -> f64 {
    let Terms {
        a00,
        a01,
        a11,
        b0,
        b1,
        c,
    } = terms;
    let tmp = stored(x87_add(a01, b1));
    if tmp < 0.0 || tmp.is_nan() {
        let minus = stored(-tmp);
        if minus < a11 || minus.is_nan() || a11.is_nan() {
            let q = stored(x87_div(minus, a11));
            e.mem.set_u32(t_out, x87_store32(q));
            let sum = x87_add(a00, x87_mul(tmp, q));
            return finish(x87_add(x87_add(sum, x87_add(b0, b0)), c));
        }
        e.mem.set_u32(t_out, x87_store32(1.0));
        let sum = x87_add(x87_add(a00, a11), c);
        let x = x87_add(tmp, b0);
        return finish(x87_add(sum, x87_add(x, x)));
    }
    e.mem.set_u32(t_out, 0);
    finish(x87_add(x87_add(a00, x87_add(b0, b0)), c))
}

/// The parallel segments: `s` and `t` clamped along the first segment's ends.
fn parallel(e: &mut Engine, terms: Terms, s_out: u32, t_out: u32) -> f64 {
    let Terms {
        a00,
        a01,
        a11,
        b0,
        b1,
        c,
    } = terms;
    let two = x87_load64(e.mem.u64(TWO_01011590));
    if 0.0 < a01 {
        if b0 >= 0.0 {
            e.mem.set_u32(s_out, 0);
            e.mem.set_u32(t_out, 0);
            return finish(c);
        }
        let minus = stored(-b0);
        if minus <= a00 {
            e.mem.set_u32(s_out, x87_store32(x87_div(minus, a00)));
            e.mem.set_u32(t_out, 0);
            return finish(x87_add(x87_mul(b0, float_operand(e, s_out)), c));
        }
        e.mem.set_u32(s_out, x87_store32(1.0));
        let v = stored(x87_add(b0, a00));
        let minus = stored(-v);
        if minus < a01 || minus.is_nan() || a01.is_nan() {
            let q = stored(x87_div(minus, a01));
            e.mem.set_u32(t_out, x87_store32(q));
            let z = x87_add(a01, b1);
            let twice = x87_mul(z, two);
            let w = x87_add(x87_mul(a11, q), twice);
            let wq = x87_mul(w, q);
            let a = x87_add(a00, x87_mul(two, b0));
            return finish(x87_add(wq, x87_add(a, c)));
        }
        e.mem.set_u32(t_out, x87_store32(1.0));
        let y = x87_add(x87_add(a01, b0), b1);
        let sum = x87_add(x87_add(a00, a11), c);
        return finish(x87_add(x87_add(y, y), sum));
    }
    let minus = stored(-b0);
    if minus >= a00 {
        e.mem.set_u32(s_out, x87_store32(1.0));
        e.mem.set_u32(t_out, 0);
        return finish(x87_add(x87_add(a00, x87_add(b0, b0)), c));
    }
    if b0 <= 0.0 {
        e.mem.set_u32(s_out, x87_store32(x87_div(minus, a00)));
        e.mem.set_u32(t_out, 0);
        return finish(x87_add(x87_mul(b0, float_operand(e, s_out)), c));
    }
    e.mem.set_u32(s_out, 0);
    if -a01 <= b0 {
        e.mem.set_u32(t_out, x87_store32(1.0));
        return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
    }
    let q = stored(x87_div(minus, a01));
    e.mem.set_u32(t_out, x87_store32(q));
    let w = x87_add(x87_mul(a11, q), x87_add(b1, b1));
    finish(x87_add(x87_mul(w, q), c))
}

// Translated from 00eaf310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSqrDistance::Compute_ov2` (Xbox PDB; `cdecl`: two segments (origin, direction), the out
/// parameters `s` and `t` on `[0, 1]`): the squared distance between the segments by Eberly's
/// method (`segment_terms`). Segments whose directions' squared cross product is at most the
/// float at `010959f4` (or unordered) are handled as parallel (`parallel`); otherwise
/// `s = a01 b1 - a11 b0` and `t = a01 b0 - a00 b1` are written and clamped by the nine regions
/// of `(s, t)` against `[0, det]` (the interior normalized by `1 / det`), each value stored as a
/// float in the exe's order. Answers the absolute value of the squared distance.
pub fn ni_sqr_distance_compute_ov2(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    out_s: Ptr,
    out_t: Ptr,
) -> f64 {
    let (s_out, t_out) = (out_s.addr(), out_t.addr());
    let (terms, det) = segment_terms(e, first.addr(), second.addr());
    let Terms {
        a00,
        a01,
        a11,
        b0,
        b1,
        c,
    } = terms;
    let limit = float_operand(e, PARALLEL_LIMIT_010959F4);
    if !(cross_squared(e, first.addr(), second.addr()) > limit) {
        return parallel(e, terms, s_out, t_out);
    }
    let (s, t) = terms.write_parameters(e, s_out, t_out);
    if 0.0 <= s {
        if s <= det {
            if t < 0.0 || t.is_nan() {
                e.mem.set_u32(t_out, 0);
                return terms.clamp_edge(e, b0, a00, s_out);
            }
            if t <= det {
                return terms.interior(e, det, s_out, t_out);
            }
            e.mem.set_u32(t_out, x87_store32(1.0));
            let tmp = stored(x87_add(a01, b0));
            if tmp >= 0.0 {
                e.mem.set_u32(s_out, 0);
                return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
            }
            return far_side_t(e, terms, tmp, tmp, s_out);
        }
        if t < 0.0 || t.is_nan() {
            let minus = stored(-b0);
            if minus < a00 {
                e.mem.set_u32(t_out, 0);
                if 0.0 <= b0 {
                    e.mem.set_u32(s_out, 0);
                    return finish(c);
                }
                let q = stored(x87_div(minus, a00));
                e.mem.set_u32(s_out, x87_store32(q));
                return finish(x87_add(x87_mul(b0, q), c));
            }
            e.mem.set_u32(s_out, x87_store32(1.0));
            return far_side_s(e, terms, t_out);
        }
        if t <= det {
            e.mem.set_u32(s_out, x87_store32(1.0));
            return far_side_s(e, terms, t_out);
        }
        // The corner s = 1, t = 1.
        let tmp0 = stored(x87_add(b0, a01));
        let minus = stored(-tmp0);
        if minus <= a00 {
            e.mem.set_u32(t_out, x87_store32(1.0));
            if tmp0 < 0.0 || tmp0.is_nan() {
                let q = stored(x87_div(minus, a00));
                e.mem.set_u32(s_out, x87_store32(q));
                let sum = x87_add(x87_mul(tmp0, q), a11);
                return finish(x87_add(x87_add(sum, x87_add(b1, b1)), c));
            }
            e.mem.set_u32(s_out, 0);
            return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
        }
        e.mem.set_u32(s_out, x87_store32(1.0));
        let tmp1 = stored(x87_add(a01, b1));
        let minus = stored(-tmp1);
        if minus < a11 || minus.is_nan() || a11.is_nan() {
            if tmp1 > 0.0 {
                e.mem.set_u32(t_out, 0);
                return finish(x87_add(x87_add(a00, x87_add(b0, b0)), c));
            }
            let q = stored(x87_div(minus, a11));
            e.mem.set_u32(t_out, x87_store32(q));
            let sum = x87_add(a00, x87_mul(tmp1, q));
            return finish(x87_add(x87_add(sum, x87_add(b0, b0)), c));
        }
        e.mem.set_u32(t_out, x87_store32(1.0));
        let sum = x87_add(x87_add(a00, a11), c);
        let x = x87_add(tmp1, b0);
        return finish(x87_add(sum, x87_add(x, x)));
    }
    if 0.0 > t || t.is_nan() {
        // The corner s = 0, t = 0.
        if b0 < 0.0 {
            e.mem.set_u32(t_out, 0);
            return terms.clamp_edge(e, b0, a00, s_out);
        }
        e.mem.set_u32(s_out, 0);
        return terms.clamp_edge(e, b1, a11, t_out);
    }
    if t <= det {
        e.mem.set_u32(s_out, 0);
        return terms.clamp_edge(e, b1, a11, t_out);
    }
    // s < 0, t beyond: the corner s = 0, t = 1.
    let tmp = stored(x87_add(a01, b0));
    if tmp < 0.0 {
        e.mem.set_u32(t_out, x87_store32(1.0));
        return far_side_t(e, terms, tmp, tmp, s_out);
    }
    e.mem.set_u32(s_out, 0);
    let u = stored(x87_add(b1, a11));
    if u <= 0.0 {
        e.mem.set_u32(t_out, x87_store32(1.0));
        return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
    }
    if 0.0 <= b1 {
        e.mem.set_u32(t_out, 0);
        return finish(c);
    }
    let q = stored(x87_div(-b1, a11));
    e.mem.set_u32(t_out, x87_store32(q));
    finish(x87_add(x87_mul(b1, q), c))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ea_f310,
        ni_sqr_distance_compute_ov2(Ptr, Ptr, Ptr, Ptr) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment(e: &mut Engine, origin: [f32; 3], direction: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x18);
        for (i, v) in (0u32..).zip(origin.iter().chain(direction.iter())) {
            e.mem.set_f32(at + 4 * i, *v);
        }
        at
    }

    fn distance(e: &mut Engine, first: u32, second: u32) -> (f64, f32, f32) {
        let outs = e.mem.alloc(8);
        let d = e
            .call(0x00ea_f310, &args![first, second, outs, outs + 4])
            .f64();
        (d, e.mem.f32(outs), e.mem.f32(outs + 4))
    }

    fn setup(e: &mut Engine) {
        e.map(0x0101_1000, 0x1000);
        e.map(0x0109_5000, 0x1000);
        e.mem.set_f64(TWO_01011590, 2.0);
        e.mem.set_f32(PARALLEL_LIMIT_010959F4, 1e-6);
    }

    #[test]
    fn crossing_and_clamped_segments() {
        let mut e = Engine::new();
        setup(&mut e);
        let x_axis = segment(&mut e, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        // Crossing above the middle: both parameters inside.
        let above = segment(&mut e, [1.0, -1.0, 3.0], [0.0, 2.0, 0.0]);
        assert_eq!(distance(&mut e, x_axis, above), (9.0, 0.5, 0.5));
        // Beyond the first segment's end: s = 1.
        let beyond = segment(&mut e, [4.0, -1.0, 0.0], [0.0, 2.0, 0.0]);
        assert_eq!(distance(&mut e, x_axis, beyond), (4.0, 1.0, 0.5));
        // Before both starts: the corner s = 0, t = 0.
        let before = segment(&mut e, [-1.0, 1.0, 0.0], [0.0, 2.0, 0.0]);
        assert_eq!(distance(&mut e, x_axis, before), (2.0, 0.0, 0.0));
    }

    #[test]
    fn parallel_segments() {
        let mut e = Engine::new();
        setup(&mut e);
        let x_axis = segment(&mut e, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        // Same direction, overlapping, one unit away: the second start projects at s = 0.5.
        let same = segment(&mut e, [1.0, 1.0, 0.0], [2.0, 0.0, 0.0]);
        assert_eq!(distance(&mut e, x_axis, same), (1.0, 0.5, 0.0));
        // Opposite direction, starting before the first: both starts.
        let opposite = segment(&mut e, [-1.0, 1.0, 0.0], [-2.0, 0.0, 0.0]);
        assert_eq!(distance(&mut e, x_axis, opposite), (2.0, 0.0, 0.0));
    }
}
