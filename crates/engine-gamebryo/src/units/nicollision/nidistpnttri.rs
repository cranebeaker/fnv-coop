//! `NiCollision/nidistpnttri.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// The double 2.0 at `01011590` (`FLD qword`).
pub(crate) const TWO_01011590: u32 = 0x0101_1590;

/// A value stored as a float and loaded back.
pub(crate) fn stored(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// The squared distance as every return of the point-triangle and point-parallelogram distances
/// produces it: stored, its absolute value (`FABS`) stored, and loaded.
pub(crate) fn finish(value: f64) -> f64 {
    stored(stored(value).abs())
}

/// The quantities of a point's distance to a triangle or parallelogram (origin, two edges), in
/// Eberly's notation: `a00 = e0.e0`, `a01 = e0.e1`, `a11 = e1.e1`, `b0 = e0.d`, `b1 = e1.d` and
/// `c = d.d` with `d = origin - point`, all stored floats, computed in the exe's operand order
/// (`00eb5450` and `00eb6150` share it).
#[derive(Clone, Copy)]
pub(crate) struct Terms {
    pub(crate) a00: f64,
    pub(crate) a01: f64,
    pub(crate) a11: f64,
    pub(crate) b0: f64,
    pub(crate) b1: f64,
    pub(crate) c: f64,
}

impl Terms {
    /// The terms for `point` and the shape at `shape`, and `det = |a00 a11 - a01^2|` (stored).
    pub(crate) fn new(e: &Engine, point: u32, shape: u32) -> (Terms, f64) {
        let ld = |e: &Engine, offset: u32| load_float(e, shape.wrapping_add(offset));
        let op = |e: &Engine, offset: u32| float_operand(e, shape.wrapping_add(offset));
        let [dx, dy, dz] = [0u32, 4, 8]
            .map(|i| stored(x87_sub(ld(e, i), float_operand(e, point.wrapping_add(i)))));
        let dot = |e: &Engine, u: u32, v: u32| {
            let s = x87_add(
                x87_mul(ld(e, u + 4), op(e, v + 4)),
                x87_mul(ld(e, u), op(e, v)),
            );
            stored(x87_add(s, x87_mul(ld(e, u + 8), op(e, v + 8))))
        };
        let a00 = dot(e, 0xc, 0xc);
        let a01 = dot(e, 0x18, 0xc);
        let a11 = dot(e, 0x18, 0x18);
        let s = x87_add(x87_mul(ld(e, 0xc), dx), x87_mul(ld(e, 0x10), dy));
        let b0 = stored(x87_add(x87_mul(ld(e, 0x14), dz), s));
        let s = x87_add(x87_mul(ld(e, 0x1c), dy), x87_mul(ld(e, 0x18), dx));
        let b1 = stored(x87_add(s, x87_mul(ld(e, 0x20), dz)));
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

    /// Writes the unnormalized parameters `s = a01 b1 - a11 b0` and `t = a01 b0 - a00 b1` (each
    /// stored) and answers them as the exe compares them: `s` as a memory operand, `t` loaded.
    pub(crate) fn write_parameters(self, e: &mut Engine, s_out: u32, t_out: u32) -> (f64, f64) {
        let s = x87_store32(x87_sub(
            x87_mul(self.b1, self.a01),
            x87_mul(self.b0, self.a11),
        ));
        e.mem.set_u32(s_out, s);
        let t = stored(x87_sub(
            x87_mul(self.b0, self.a01),
            x87_mul(self.b1, self.a00),
        ));
        e.mem.set_u32(t_out, x87_store32(t));
        (x87_m32(s), t)
    }

    /// `edge = 0` with the other parameter on `[0, 1]` (`b` and `a` of that edge, written to
    /// `out`): 0 when `b >= 0`, else `-b / a` (stored) when `-b < a` (unordered too), else 1.
    pub(crate) fn clamp_edge(self, e: &mut Engine, b: f64, a: f64, out: u32) -> f64 {
        if 0.0 <= b {
            e.mem.set_u32(out, 0);
            return finish(self.c);
        }
        let minus = stored(-b);
        if minus < a || minus.is_nan() || a.is_nan() {
            let q = stored(x87_div(minus, a));
            e.mem.set_u32(out, x87_store32(q));
            finish(x87_add(x87_mul(b, q), self.c))
        } else {
            e.mem.set_u32(out, x87_store32(1.0));
            finish(x87_add(x87_add(a, x87_add(b, b)), self.c))
        }
    }

    /// The point projects inside: `s` and `t` (in memory) times `1 / det` (each stored, written
    /// back) and the distance `s (a00 s + a01 t + 2 b0) + t (a01 s + a11 t + 2 b1) + c` in the
    /// exe's order (the 2 a double at `01011590`).
    pub(crate) fn interior(self, e: &mut Engine, det: f64, s_out: u32, t_out: u32) -> f64 {
        let Terms {
            a00,
            a01,
            a11,
            b0,
            b1,
            c,
        } = self;
        let two = x87_load64(e.mem.u64(TWO_01011590));
        let inverse = stored(x87_div(1.0, det));
        let s = x87_store32(x87_mul(load_float(e, s_out), inverse));
        e.mem.set_u32(s_out, s);
        let t = stored(x87_mul(inverse, float_operand(e, t_out)));
        e.mem.set_u32(t_out, x87_store32(t));
        let s = x87_load32(s);
        let x = x87_add(x87_mul(a11, t), x87_mul(s, a01));
        let w = x87_add(x87_mul(b1, two), x);
        let wt = x87_mul(w, t);
        let v = x87_add(x87_mul(a01, t), x87_mul(a00, s));
        let v = x87_add(v, x87_mul(two, b0));
        let total = x87_add(x87_mul(v, s), wt);
        finish(x87_add(total, c))
    }

    /// The far edge `s + t = 1` once `numer = (b1 + a11) - (b0 + a01)` (or its mirror) is
    /// positive: `denom = a00 - 2 a01 + a11` (stored); at least `denom` puts the point at the
    /// corner (`first = 1`, `second = 0`, `2 b_first + a_first + c`), otherwise
    /// `first = numer / denom`, `second = 1 - first` and
    /// `(2 b_second + a_second + c) - numer * first`.
    fn far_edge(
        self,
        e: &mut Engine,
        numer: f64,
        two: f64,
        first: (u32, f64, f64),
        second: (u32, f64, f64),
    ) -> f64 {
        let (first_out, first_b, first_a) = first;
        let (second_out, second_b, second_a) = second;
        let denom = stored(x87_add(x87_sub(self.a00, x87_mul(self.a01, two)), self.a11));
        if denom <= numer {
            e.mem.set_u32(first_out, x87_store32(1.0));
            e.mem.set_u32(second_out, 0);
            return finish(x87_add(x87_add(x87_mul(two, first_b), first_a), self.c));
        }
        let q = stored(x87_div(numer, denom));
        e.mem.set_u32(first_out, x87_store32(q));
        e.mem.set_u32(second_out, x87_store32(x87_sub(1.0, q)));
        let a = x87_add(x87_mul(two, second_b), second_a);
        let b = x87_add(a, self.c);
        finish(x87_sub(b, x87_mul(numer, q)))
    }
}

// Translated from 00eb5450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSqrDistance::Compute_ov4` (Xbox PDB; `cdecl`: a point, a triangle (origin, two edges), the
/// out parameters `s` and `t`): the squared distance from the point to the triangle, by the
/// regions of Eberly's method. With the terms of `Terms::new` and `det = |a00 a11 - a01^2|`,
/// `s = a01 b1 - a11 b0` and `t = a01 b0 - a00 b1` are written first; the region of `(s, t)`
/// then clamps them to the triangle (regions 0 to 6 as in the exe's branches, a NaN following
/// the branch the flag tests take) and the answer is the absolute value of the squared distance
/// there, each value stored as a float in the exe's order.
pub fn ni_sqr_distance_compute_ov4(
    e: &mut Engine,
    point: Ptr,
    triangle: Ptr,
    out_s: Ptr,
    out_t: Ptr,
) -> f64 {
    let (s_out, t_out) = (out_s.addr(), out_t.addr());
    let (terms, det) = Terms::new(e, point.addr(), triangle.addr());
    let Terms {
        a00,
        a01,
        a11,
        b0,
        b1,
        c,
    } = terms;
    let (s, t) = terms.write_parameters(e, s_out, t_out);
    let two = x87_load64(e.mem.u64(TWO_01011590));
    if x87_add(t, s) <= det {
        if 0.0 > s {
            // Regions 4 and 3.
            if t < 0.0 && b0 < 0.0 {
                e.mem.set_u32(t_out, 0);
                return terms.clamp_edge(e, b0, a00, s_out);
            }
            e.mem.set_u32(s_out, 0);
            return terms.clamp_edge(e, b1, a11, t_out);
        }
        if 0.0 > t {
            // Region 5.
            e.mem.set_u32(t_out, 0);
            return terms.clamp_edge(e, b0, a00, s_out);
        }
        // Region 0: inside.
        return terms.interior(e, det, s_out, t_out);
    }
    if 0.0 > s {
        // Region 2.
        let tmp0 = stored(x87_add(b0, a01));
        let tmp1 = stored(x87_add(b1, a11));
        if tmp0 < tmp1 {
            let numer = stored(x87_sub(tmp1, tmp0));
            return terms.far_edge(e, numer, two, (s_out, b0, a00), (t_out, b1, a11));
        }
        e.mem.set_u32(s_out, 0);
        if tmp1 <= 0.0 {
            e.mem.set_u32(t_out, x87_store32(1.0));
            return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
        }
        if b1 >= 0.0 {
            e.mem.set_u32(t_out, 0);
            return finish(c);
        }
        let q = stored(x87_div(-b1, a11));
        e.mem.set_u32(t_out, x87_store32(q));
        return finish(x87_add(x87_mul(b1, q), c));
    }
    if 0.0 > t {
        // Region 6.
        let tmp0 = stored(x87_add(b1, a01));
        let tmp1 = stored(x87_add(b0, a00));
        if tmp0 < tmp1 {
            let numer = stored(x87_sub(tmp1, tmp0));
            let denom = stored(x87_add(x87_sub(a00, x87_mul(a01, two)), a11));
            if denom <= numer {
                e.mem.set_u32(t_out, x87_store32(1.0));
                e.mem.set_u32(s_out, 0);
                return finish(x87_add(x87_add(x87_mul(two, b1), a11), c));
            }
            let q = stored(x87_div(numer, denom));
            e.mem.set_u32(t_out, x87_store32(q));
            e.mem.set_u32(s_out, x87_store32(x87_sub(1.0, q)));
            let a = x87_add(a00, x87_mul(two, b0));
            let b = x87_add(a, c);
            return finish(x87_sub(b, x87_mul(numer, q)));
        }
        e.mem.set_u32(t_out, 0);
        if tmp1 <= 0.0 {
            e.mem.set_u32(s_out, x87_store32(1.0));
            return finish(x87_add(x87_add(a00, x87_add(b0, b0)), c));
        }
        if b0 >= 0.0 {
            e.mem.set_u32(s_out, 0);
            return finish(c);
        }
        let q = stored(x87_div(-b0, a00));
        e.mem.set_u32(s_out, x87_store32(q));
        return finish(x87_add(x87_mul(b0, q), c));
    }
    // Region 1.
    let numer = stored(x87_sub(x87_add(b1, a11), x87_add(b0, a01)));
    if numer <= 0.0 {
        e.mem.set_u32(s_out, 0);
        e.mem.set_u32(t_out, x87_store32(1.0));
        return finish(x87_add(x87_add(a11, x87_add(b1, b1)), c));
    }
    terms.far_edge(e, numer, two, (s_out, b0, a00), (t_out, b1, a11))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_5450,
        ni_sqr_distance_compute_ov4(Ptr, Ptr, Ptr, Ptr) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The unit right triangle at the origin and the outs; answers (distance, s, t).
    fn distance(e: &mut Engine, point: [f32; 3]) -> (f64, f32, f32) {
        let tri = e.mem.alloc(0x24);
        e.mem.set_f32(tri + 0xc, 1.0);
        e.mem.set_f32(tri + 0x1c, 1.0);
        let p = e.mem.alloc(12);
        for (i, v) in (0u32..).zip(point) {
            e.mem.set_f32(p + 4 * i, v);
        }
        let outs = e.mem.alloc(8);
        let d = e.call(0x00eb_5450, &args![p, tri, outs, outs + 4]).f64();
        (d, e.mem.f32(outs), e.mem.f32(outs + 4))
    }

    #[test]
    fn regions_clamp_to_the_triangle() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f64(TWO_01011590, 2.0);
        // Region 0: above the inside.
        assert_eq!(distance(&mut e, [0.25, 0.25, 1.0]), (1.0, 0.25, 0.25));
        // Region 1: beyond the far edge.
        assert_eq!(distance(&mut e, [2.0, 2.0, 0.0]), (4.5, 0.5, 0.5));
        // Region 4: behind the origin corner.
        assert_eq!(distance(&mut e, [-1.0, -1.0, 0.0]), (2.0, 0.0, 0.0));
        // Region 3: beside the second edge.
        assert_eq!(distance(&mut e, [-1.0, 0.5, 0.0]), (1.0, 0.0, 0.5));
        // Region 6: past the first edge's end.
        assert_eq!(distance(&mut e, [3.0, -1.0, 0.0]), (5.0, 1.0, 0.0));
        // Region 5: below the first edge.
        assert_eq!(distance(&mut e, [0.5, -2.0, 0.0]), (4.0, 0.5, 0.0));
        // Region 2: past the second edge's end.
        assert_eq!(distance(&mut e, [-1.0, 3.0, 0.0]), (5.0, 0.0, 1.0));
    }
}
