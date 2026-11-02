//! `NiCollision/run_00eb6150` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::load_float;
use crate::units::nicollision::nidistpnttri::{finish, stored, Terms};

/// The float `1e-5` at `010718c0`: the smallest `|a00 a11 - a01^2|` handled as a parallelogram.
const EPSILON_010718C0: u32 = 0x0107_18c0;
/// `NiSqrDistance::Compute` (`00eaf1f0`, Xbox PDB; cdecl: a point, a segment (origin,
/// direction), the out parameter): their squared distance, in ST0.
const SQR_DISTANCE_POINT_SEGMENT: u32 = 0x00ea_f1f0;

/// The far side `t = 1` from `s < 0` or inside (`tmp = a01 + b0` below 0, `tmp_stored` its
/// stored value): `-tmp` (stored) below `a00` (unordered too) gives `s = -tmp / a00` and
/// `tmp s + a11 + 2 b1 + c`, otherwise `s = 1` and `(a00 + a11 + c) + 2 (tmp + b1)`.
pub(crate) fn far_side_t(
    e: &mut Engine,
    terms: Terms,
    tmp: f64,
    tmp_stored: f64,
    s_out: u32,
) -> f64 {
    let Terms {
        a00, a11, b1, c, ..
    } = terms;
    let minus = stored(-tmp_stored);
    if minus < a00 || minus.is_nan() || a00.is_nan() {
        let q = stored(x87_div(minus, a00));
        e.mem.set_u32(s_out, x87_store32(q));
        let sum = x87_add(x87_mul(tmp_stored, q), a11);
        return finish(x87_add(x87_add(sum, x87_add(b1, b1)), c));
    }
    e.mem.set_u32(s_out, x87_store32(1.0));
    let sum = x87_add(x87_add(a00, a11), c);
    let x = x87_add(tmp, b1);
    finish(x87_add(sum, x87_add(x, x)))
}

/// The far side `s = 1` (already written): with `tmp = b1 + a01` (stored) not below 0, `t = 0`
/// and `a00 + 2 b0 + c`; otherwise `-tmp` (stored) below `a11` (unordered too) gives
/// `t = -tmp / a11` and `a00 + tmp t + 2 b0 + c`, else `t = 1` and
/// `2 ((a01 + b0) + b1) + (a00 + a11 + c)`.
fn far_side_s(e: &mut Engine, terms: Terms, t_out: u32) -> f64 {
    let Terms {
        a00,
        a01,
        a11,
        b0,
        b1,
        c,
    } = terms;
    let tmp = stored(x87_add(b1, a01));
    if tmp < 0.0 || tmp.is_nan() {
        let minus = stored(-tmp);
        if minus < a11 || minus.is_nan() || a11.is_nan() {
            let q = stored(x87_div(minus, a11));
            e.mem.set_u32(t_out, x87_store32(q));
            let sum = x87_add(a00, x87_mul(tmp, q));
            return finish(x87_add(x87_add(sum, x87_add(b0, b0)), c));
        }
        e.mem.set_u32(t_out, x87_store32(1.0));
        let x = x87_add(x87_add(a01, b0), b1);
        let sum = x87_add(x87_add(a00, a11), c);
        return finish(x87_add(x87_add(x, x), sum));
    }
    e.mem.set_u32(t_out, 0);
    finish(x87_add(x87_add(a00, x87_add(b0, b0)), c))
}

// Translated from 00eb6150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: a point, a parallelogram (origin, two edges), the out
/// parameters `s` and `t`; `00eaff20` calls it): the squared distance from the point to the
/// parallelogram `origin + s e0 + t e1`, `s` and `t` on `[0, 1]`. The terms are those of
/// `00eb5450` (`Terms::new`). When `det = |a00 a11 - a01^2|` is below the float at `010718c0`
/// (or unordered) the edges are parallel: the point's distance to the segment along the longer
/// edge (`e0` unless `a00 < a11` or unordered; `00eaf1f0`, its parameter to `s` or `t`, the other
/// 0). Otherwise `s = a01 b1 - a11 b0` and `t = a01 b0 - a00 b1` are written and clamped by the
/// nine regions of `(s, t)` against `[0, det]` (the interior normalized by `1 / det`), each value
/// stored as a float in the exe's order; the answer is the absolute value of the squared distance.
pub fn fn_00eb6150(e: &mut Engine, point: Ptr, shape: Ptr, out_s: Ptr, out_t: Ptr) -> f64 {
    let (p, sh, s_out, t_out) = (point.addr(), shape.addr(), out_s.addr(), out_t.addr());
    let (terms, det) = Terms::new(e, p, sh);
    let Terms {
        a00,
        a01,
        a11,
        b0,
        b1,
        c,
    } = terms;
    let epsilon = load_float(e, EPSILON_010718C0);
    if !(epsilon <= det) {
        // Parallel edges: the distance to the longer edge as a segment.
        return e.with_stack(0x18, |e, segment| {
            let segment = segment.addr();
            for i in [0u32, 4, 8] {
                let word = e.mem.u32(sh.wrapping_add(i));
                e.mem.set_u32(segment + i, word);
            }
            let along_second = a00 < a11 || a00.is_nan() || a11.is_nan();
            let (edge, zeroed, out) = if along_second {
                (0x18u32, s_out, t_out)
            } else {
                (0xc, t_out, s_out)
            };
            for i in [0u32, 4, 8] {
                let word = e.mem.u32(sh.wrapping_add(edge + i));
                e.mem.set_u32(segment + 0xc + i, word);
            }
            e.mem.set_u32(zeroed, 0);
            e.call(SQR_DISTANCE_POINT_SEGMENT, &args![p, segment, out])
                .f64()
        });
    }
    let (s, t) = terms.write_parameters(e, s_out, t_out);
    if 0.0 > s {
        if 0.0 > t {
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
        // The corner s = 0, t = 1.
        let tmp = x87_add(a01, b0);
        let tmp_stored = stored(tmp);
        if tmp_stored < 0.0 {
            e.mem.set_u32(t_out, x87_store32(1.0));
            return far_side_t(e, terms, tmp, tmp_stored, s_out);
        }
        e.mem.set_u32(s_out, 0);
        return terms.clamp_edge(e, b1, a11, t_out);
    }
    if s <= det {
        if t < 0.0 {
            e.mem.set_u32(t_out, 0);
            return terms.clamp_edge(e, b0, a00, s_out);
        }
        if t <= det {
            return terms.interior(e, det, s_out, t_out);
        }
        e.mem.set_u32(t_out, x87_store32(1.0));
        let tmp = x87_add(a01, b0);
        let tmp_stored = stored(tmp);
        if tmp_stored >= 0.0 {
            e.mem.set_u32(s_out, 0);
            return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
        }
        return far_side_t(e, terms, tmp, tmp_stored, s_out);
    }
    if t < 0.0 {
        // The corner s = 1, t = 0.
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
    let tmp0 = x87_add(b0, a01);
    let tmp0_stored = stored(tmp0);
    let minus = stored(-tmp0_stored);
    if minus < a00 {
        e.mem.set_u32(t_out, x87_store32(1.0));
        if tmp0_stored < 0.0 || tmp0_stored.is_nan() {
            let q = stored(x87_div(minus, a00));
            e.mem.set_u32(s_out, x87_store32(q));
            let sum = x87_add(x87_mul(tmp0_stored, q), a11);
            return finish(x87_add(x87_add(sum, x87_add(b1, b1)), c));
        }
        e.mem.set_u32(s_out, 0);
        return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
    }
    e.mem.set_u32(s_out, x87_store32(1.0));
    let tmp1 = stored(x87_add(a01, b1));
    if tmp1 < 0.0 || tmp1.is_nan() {
        let minus = stored(-tmp1);
        if minus < a11 || minus.is_nan() || a11.is_nan() {
            let q = stored(x87_div(minus, a11));
            e.mem.set_u32(t_out, x87_store32(q));
            let sum = x87_add(a00, x87_mul(tmp1, q));
            return finish(x87_add(x87_add(sum, x87_add(b0, b0)), c));
        }
        e.mem.set_u32(t_out, x87_store32(1.0));
        // The exe keeps `b0 + a01` as a double (`FST qword`) for this sum.
        let y = x87_add(b1, tmp0);
        let sum = x87_add(x87_add(a00, a11), c);
        return finish(x87_add(sum, x87_add(y, y)));
    }
    e.mem.set_u32(t_out, 0);
    finish(x87_add(x87_add(a00, x87_add(b0, b0)), c))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_6150,
        fn_00eb6150(Ptr, Ptr, Ptr, Ptr) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nicollision::nidistpnttri::TWO_01011590;

    /// The unit square at the origin (or `edges` instead) and the outs; answers (distance, s, t).
    fn distance(e: &mut Engine, edges: [f32; 6], point: [f32; 3]) -> (f64, f32, f32) {
        let shape = e.mem.alloc(0x24);
        for (i, v) in (3u32..).zip(edges) {
            e.mem.set_f32(shape + 4 * i, v);
        }
        let p = e.mem.alloc(12);
        for (i, v) in (0u32..).zip(point) {
            e.mem.set_f32(p + 4 * i, v);
        }
        let outs = e.mem.alloc(8);
        e.mem.set_u32(outs, 0x5555);
        e.mem.set_u32(outs + 4, 0x5555);
        let d = e.call(0x00eb_6150, &args![p, shape, outs, outs + 4]).f64();
        (d, e.mem.f32(outs), e.mem.f32(outs + 4))
    }

    const SQUARE: [f32; 6] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0];

    fn setup(e: &mut Engine) {
        e.map(0x0101_1000, 0x1000);
        e.map(0x0107_1000, 0x1000);
        e.mem.set_f64(TWO_01011590, 2.0);
        e.mem.set_f32(EPSILON_010718C0, 1e-5);
    }

    #[test]
    fn regions_clamp_to_the_square() {
        let mut e = Engine::new();
        setup(&mut e);
        assert_eq!(distance(&mut e, SQUARE, [0.5, 0.25, 2.0]), (4.0, 0.5, 0.25));
        assert_eq!(distance(&mut e, SQUARE, [-1.0, -1.0, 0.0]), (2.0, 0.0, 0.0));
        assert_eq!(distance(&mut e, SQUARE, [-2.0, 0.5, 0.0]), (4.0, 0.0, 0.5));
        assert_eq!(distance(&mut e, SQUARE, [-1.0, 3.0, 0.0]), (5.0, 0.0, 1.0));
        assert_eq!(distance(&mut e, SQUARE, [0.5, 3.0, 0.0]), (4.0, 0.5, 1.0));
        assert_eq!(distance(&mut e, SQUARE, [0.5, -3.0, 0.0]), (9.0, 0.5, 0.0));
        assert_eq!(distance(&mut e, SQUARE, [3.0, -1.0, 0.0]), (5.0, 1.0, 0.0));
        assert_eq!(distance(&mut e, SQUARE, [3.0, 0.5, 0.0]), (4.0, 1.0, 0.5));
        assert_eq!(distance(&mut e, SQUARE, [2.0, 3.0, 0.0]), (5.0, 1.0, 1.0));
    }

    #[test]
    fn parallel_edges_measure_to_the_longer_one() {
        let mut e = Engine::new();
        setup(&mut e);
        e.register(SQR_DISTANCE_POINT_SEGMENT, |e, a| {
            e.mem.set_f32(a[2], 0.75);
            Ret {
                st0: f64::from(e.mem.f32(a[1] + 0xc)),
                ..Ret::default()
            }
        });
        // e0 = (2,0,0), e1 = (1,0,0): along e0, t = 0.
        let edges = [2.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        assert_eq!(distance(&mut e, edges, [0.0; 3]), (2.0, 0.75, 0.0));
        // e0 = (1,0,0), e1 = (3,0,0): along e1, s = 0.
        let edges = [1.0, 0.0, 0.0, 3.0, 0.0, 0.0];
        assert_eq!(distance(&mut e, edges, [0.0; 3]), (3.0, 0.0, 0.75));
    }
}
