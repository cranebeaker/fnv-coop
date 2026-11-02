//! `NiCollision/nidistsegtri.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// `NiSqrDistance::Compute_ov2` (`00eaf310`, Xbox PDB; cdecl: two segments, two out floats): the
/// squared segment-segment distance, in ST0.
const SQR_DISTANCE_SEGMENT_SEGMENT: u32 = 0x00ea_f310;
/// `NiSqrDistance::Compute_ov4` (`00eb5450`, Xbox PDB; cdecl: a point, a triangle, two out
/// floats): the squared point-triangle distance, in ST0.
const SQR_DISTANCE_POINT_TRIANGLE: u32 = 0x00eb_5450;
/// `00439e90` and `00439ef0` (thiscall on a point: the out point, the other point): the sum and
/// the difference; each answers the out point.
const POINT_PLUS_POINT: u32 = 0x0043_9e90;
const POINT_MINUS_POINT: u32 = 0x0043_9ef0;
/// The CRT square root (`00ec6040`): the value in ST0 (a leading `f64`), the root in ST0.
const CRT_SQRT: u32 = 0x00ec_6040;
/// The double at `0101dae8` (about `1e-5`): `|n . d|` at or below it times `|n| |d|` makes the
/// segment parallel to the triangle.
const PARALLEL_FACTOR_0101DAE8: u32 = 0x0101_dae8;
/// The double 1.0 at `01012070`, the bound of `s + t`.
const ONE_01012070: u32 = 0x0101_2070;

/// A value stored as a float and loaded back.
fn stored(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `FLD dword [from]; FSTP dword [to]`.
fn move_float(e: &mut Engine, from: u32, to: u32) {
    let word = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, word);
}

fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// `u.v` of two float triples as the exe sums it: `(u.y v.y + u.x v.x) + u.z v.z`, each `u`
/// component loaded and each `v` component a memory operand.
fn dot(e: &Engine, u: u32, v: u32) -> f64 {
    let s = x87_add(
        x87_mul(load_float(e, u + 4), float_operand(e, v + 4)),
        x87_mul(load_float(e, u), float_operand(e, v)),
    );
    x87_add(s, x87_mul(load_float(e, u + 8), float_operand(e, v + 8)))
}

/// The state of `00eb1d20`: the segment, the triangle, the out parameters and the frame slots
/// the callees write (offsets from ESP after the exe's four register pushes).
struct SegmentTriangle {
    segment: u32,
    triangle: u32,
    r_out: u32,
    s_out: u32,
    t_out: u32,
    frame: u32,
    best: u32,
}

impl SegmentTriangle {
    fn slot(&self, offset: u32) -> u32 {
        self.frame + offset
    }

    /// The segment block the calls take (+0x48: origin, direction).
    fn block(&self) -> u32 {
        self.slot(0x48)
    }

    /// The block as the triangle's origin and one of its edges (+0xc or +0x18).
    fn edge_block(&self, e: &mut Engine, edge: u32) {
        copy_words(e, self.triangle, self.block(), 3);
        copy_words(e, self.triangle + edge, self.block() + 0xc, 3);
    }

    /// The block as the far edge, from `origin + e0` along `e1 - e0`, through `00439e90` and
    /// `00439ef0` (their outs at +0x60).
    fn far_edge_by_calls(&self, e: &mut Engine) {
        let (o, e0, e1) = (self.triangle, self.triangle + 0xc, self.triangle + 0x18);
        let scratch = self.slot(0x60);
        let sum = e.call(POINT_PLUS_POINT, &args![o, scratch, e0]).u32();
        copy_words(e, sum, self.block(), 3);
        let difference = e.call(POINT_MINUS_POINT, &args![e1, scratch, e0]).u32();
        copy_words(e, difference, self.block() + 0xc, 3);
    }

    /// The same far edge computed inline (each value stored).
    fn far_edge_inline(&self, e: &mut Engine) {
        let (o, e0, e1) = (self.triangle, self.triangle + 0xc, self.triangle + 0x18);
        let b = self.block();
        let x = x87_add(load_float(e, o), float_operand(e, e0));
        let y = x87_add(load_float(e, e0 + 4), float_operand(e, o + 4));
        let z = x87_add(load_float(e, e0 + 8), float_operand(e, o + 8));
        for (i, v) in (0u32..).zip([x, y, z]) {
            e.mem.set_u32(b + 4 * i, x87_store32(v));
        }
        for i in [0u32, 4, 8] {
            let v = x87_sub(load_float(e, e1 + i), float_operand(e, e0 + i));
            e.mem.set_u32(b + 0xc + i, x87_store32(v));
        }
    }

    /// The segment's end (`origin + direction`) through `00439e90`, at +0x3c.
    fn end_by_call(&self, e: &mut Engine) -> u32 {
        let s = self.segment;
        let sum = e
            .call(POINT_PLUS_POINT, &args![s, self.slot(0x60), s + 0xc])
            .u32();
        copy_words(e, sum, self.slot(0x3c), 3);
        self.slot(0x3c)
    }

    /// The segment's end computed inline (each value stored), at +0x3c.
    fn end_inline(&self, e: &mut Engine) -> u32 {
        let s = self.segment;
        let x = x87_add(load_float(e, s), float_operand(e, s + 0xc));
        let y = x87_add(load_float(e, s + 0x10), float_operand(e, s + 4));
        let z = x87_add(load_float(e, s + 0x14), float_operand(e, s + 8));
        let end = self.slot(0x3c);
        for (i, v) in (0u32..).zip([x, y, z]) {
            e.mem.set_u32(end + 4 * i, x87_store32(v));
        }
        end
    }

    /// The segment against the block (`00eaf310`), the distance stored.
    fn against_block(&self, e: &mut Engine, first: u32, second: u32) -> u32 {
        let d = e
            .call(
                SQR_DISTANCE_SEGMENT_SEGMENT,
                &args![self.segment, self.block(), first, second],
            )
            .f64();
        x87_store32(d)
    }

    /// A point against the triangle (`00eb5450`), the distance stored.
    fn point_against_triangle(&self, e: &mut Engine, point: u32, first: u32, second: u32) -> u32 {
        let d = e
            .call(
                SQR_DISTANCE_POINT_TRIANGLE,
                &args![point, self.triangle, first, second],
            )
            .f64();
        x87_store32(d)
    }

    /// Whether `candidate` is strictly nearer than the best so far (a NaN never is); it
    /// becomes the best.
    fn nearer(&mut self, candidate: u32) -> bool {
        if x87_load32(self.best) > x87_load32(candidate) {
            self.best = candidate;
            true
        } else {
            false
        }
    }

    /// Writes the outs: `r`, `s` and `t` each from a slot or a constant.
    fn write(&self, e: &mut Engine, values: [Out; 3]) {
        for (value, out) in values.into_iter().zip([self.r_out, self.s_out, self.t_out]) {
            match value {
                Out::Slot(offset) => move_float(e, self.slot(offset), out),
                Out::Value(v) => e.mem.set_u32(out, x87_store32(v)),
            }
        }
    }

    /// The second triangle edge from the origin as a candidate (`00eaf310` into +0x18 and
    /// +0x10): `(r, s, 0)`.
    fn try_first_edge(&mut self, e: &mut Engine) {
        self.edge_block(e, 0xc);
        let d = self.against_block(e, self.slot(0x18), self.slot(0x10));
        if self.nearer(d) {
            self.write(e, [Out::Slot(0x18), Out::Slot(0x10), Out::Value(0.0)]);
        }
    }

    /// The far edge as a candidate (`00eaf310` into +0x18 and +0x14; `u` the second):
    /// `(r, 1 - u, u)`.
    fn try_far_edge(&mut self, e: &mut Engine, inline: bool) {
        if inline {
            self.far_edge_inline(e);
        } else {
            self.far_edge_by_calls(e);
        }
        let d = self.against_block(e, self.slot(0x18), self.slot(0x14));
        let u = load_float(e, self.slot(0x14));
        e.mem.set_u32(self.slot(0x10), x87_store32(x87_sub(1.0, u)));
        if self.nearer(d) {
            self.write(e, [Out::Slot(0x18), Out::Slot(0x10), Out::Slot(0x14)]);
        }
    }

    /// A segment end against the triangle as a candidate (`00eb5450` into +0x10 and +0x14):
    /// `(r_end, s, t)`.
    fn try_end(&mut self, e: &mut Engine, point: u32, r_end: f64) {
        let d = self.point_against_triangle(e, point, self.slot(0x10), self.slot(0x14));
        if self.nearer(d) {
            self.write(e, [Out::Value(r_end), Out::Slot(0x10), Out::Slot(0x14)]);
        }
    }

    /// The first candidate: the segment against one edge from the origin, straight into the
    /// outs (`r` and the edge's own parameter), the other parameter 0.
    fn start_with_edge(&mut self, e: &mut Engine, edge: u32) {
        self.edge_block(e, edge);
        let (own, other) = if edge == 0x18 {
            (self.t_out, self.s_out)
        } else {
            (self.s_out, self.t_out)
        };
        self.best = self.against_block(e, self.r_out, own);
        e.mem.set_u32(other, 0);
    }

    /// The first candidate on the far edge, straight into `r` and `t`, then `s = 1 - t`.
    fn start_with_far_edge(&mut self, e: &mut Engine) {
        self.far_edge_by_calls(e);
        self.best = self.against_block(e, self.r_out, self.t_out);
        let s = x87_sub(1.0, load_float(e, self.t_out));
        e.mem.set_u32(self.s_out, x87_store32(s));
    }
}

/// An out parameter's value: a frame slot (copied as a float) or a constant.
#[derive(Clone, Copy)]
enum Out {
    Slot(u32),
    Value(f64),
}

// Translated from 00eb1d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSqrDistance::Compute_ov3` (Xbox PDB; `cdecl`: a segment (origin, direction), a triangle
/// (origin, two edges), the out parameters `r` (along the segment), `s` and `t` (along the
/// edges)): the squared distance between them. With `d = o_tri - o_seg`, the matrix of the
/// directions' products (`a00 = D.D`, `a01 = -(e0.D)`, `a02 = -(e1.D)`, `a11 = e0.e0`,
/// `a12 = e1.e0`, `a22 = e1.e1`) and the triangle's normal `n` (all stored): when `|n.D|` is at
/// most `|n| |D|` (`00ec6040` twice) times the double at `0101dae8` (or unordered), the segment is
/// parallel and the least of five candidates is taken: the segment against the first edge, the
/// second edge and the far edge (`00eaf310`) and its two ends against the triangle
/// (`00eb5450`). Otherwise the cofactors solve for `(r, s, t)` (written) and, by the region of
/// `r` (below 0, on `[0, 1]`, above 1) and of `(s, t)` (inside, beyond each edge, beyond the far
/// edge), the least of the matching edge and end candidates is taken (inside the segment and the
/// triangle the answer is 0). Each candidate replaces the outs only when strictly nearer; each
/// value is stored as a float in the exe's order.
pub fn ni_sqr_distance_compute_ov3(
    e: &mut Engine,
    segment: Ptr,
    triangle: Ptr,
    out_r: Ptr,
    out_s: Ptr,
    out_t: Ptr,
) -> f64 {
    let (seg, tri) = (segment.addr(), triangle.addr());
    let (e0, e1) = (tri + 0xc, tri + 0x18);
    let dir = seg + 0xc;
    e.with_stack(0x7c, |e, frame| {
        let mut st = SegmentTriangle {
            segment: seg,
            triangle: tri,
            r_out: out_r.addr(),
            s_out: out_s.addr(),
            t_out: out_t.addr(),
            frame: frame.addr(),
            best: 0,
        };
        let d = st.slot(0x48);
        for i in [0u32, 4, 8] {
            let v = x87_sub(load_float(e, tri + i), float_operand(e, seg + i));
            e.mem.set_u32(d + i, x87_store32(v));
        }
        let a00 = stored(dot(e, dir, dir));
        let a01 = stored(-stored(dot(e, e0, dir)));
        let a02 = stored(-stored(dot(e, e1, dir)));
        let a11 = stored(dot(e, e0, e0));
        let a12 = {
            let s = x87_add(
                x87_mul(load_float(e, e1 + 4), float_operand(e, e0 + 4)),
                x87_mul(load_float(e, e0), float_operand(e, e1)),
            );
            stored(x87_add(
                s,
                x87_mul(load_float(e, e1 + 8), float_operand(e, e0 + 8)),
            ))
        };
        let a22 = stored(dot(e, e1, e1));
        let nx = stored(x87_sub(
            x87_mul(load_float(e, e1 + 8), float_operand(e, e0 + 4)),
            x87_mul(load_float(e, e1 + 4), float_operand(e, e0 + 8)),
        ));
        let ny = stored(x87_sub(
            x87_mul(load_float(e, e1), float_operand(e, e0 + 8)),
            x87_mul(load_float(e, e1 + 8), float_operand(e, e0)),
        ));
        let nz = stored(x87_sub(
            x87_mul(load_float(e, e0), float_operand(e, e1 + 4)),
            x87_mul(load_float(e, e1), float_operand(e, e0 + 4)),
        ));
        // The parallel test.
        let [dx, dy, dz] = [0u32, 4, 8].map(|i| load_float(e, dir + i));
        let s = x87_add(x87_mul(nx, dx), x87_mul(ny, dy));
        let along = x87_add(s, x87_mul(nz, float_operand(e, dir + 8)));
        let along = stored(stored(along).abs());
        let s = x87_add(x87_mul(ny, ny), x87_mul(nx, nx));
        let normal_squared = stored(x87_add(x87_mul(nz, nz), s));
        let normal_length = stored(e.call(CRT_SQRT, &args![normal_squared]).f64());
        let s = x87_add(x87_mul(dx, dx), x87_mul(dy, dy));
        let direction_squared = stored(x87_add(s, x87_mul(dz, dz)));
        let direction_length = stored(e.call(CRT_SQRT, &args![direction_squared]).f64());
        let factor = f64::from_bits(e.mem.u64(PARALLEL_FACTOR_0101DAE8));
        let limit = stored(x87_mul(x87_mul(direction_length, normal_length), factor));
        if !(along > limit) {
            // Parallel: every edge and both ends.
            st.start_with_edge(e, 0xc);
            st.edge_block(e, 0x18);
            let candidate = st.against_block(e, st.slot(0x18), st.slot(0x14));
            e.mem.set_u32(st.slot(0x10), 0);
            if st.nearer(candidate) {
                st.write(e, [Out::Slot(0x18), Out::Value(0.0), Out::Slot(0x14)]);
            }
            st.try_far_edge(e, true);
            let candidate = st.point_against_triangle(e, seg, st.slot(0x10), st.slot(0x14));
            e.mem.set_u32(st.slot(0x18), 0);
            if st.nearer(candidate) {
                st.write(e, [Out::Value(0.0), Out::Slot(0x10), Out::Slot(0x14)]);
            }
            let end = st.end_inline(e);
            st.try_end(e, end, 1.0);
            return x87_load32(st.best);
        }
        // The cofactors and the solution.
        let cof00 = stored(x87_sub(x87_mul(a22, a11), x87_mul(a12, a12)));
        let cof01 = stored(x87_sub(x87_mul(a12, a02), x87_mul(a22, a01)));
        let cof02 = stored(x87_sub(x87_mul(a12, a01), x87_mul(a11, a02)));
        let cof12 = stored(x87_sub(x87_mul(a02, a01), x87_mul(a12, a00)));
        let det = x87_add(x87_mul(cof01, a01), x87_mul(cof00, a00));
        let det = stored(x87_add(det, x87_mul(cof02, a02)));
        let inverse = stored(x87_div(1.0, det));
        let q0 = stored(dot(e, dir, d));
        // Negated twice (`FCHS; FCHS`).
        let negated = -q0;
        let u0 = stored(x87_mul(-negated, inverse));
        let q1 = stored(dot(e, e0, d));
        let u1 = stored(x87_mul(-q1, inverse));
        let q2 = stored(dot(e, e1, d));
        let u2 = stored(x87_mul(-q2, inverse));
        let r = x87_add(x87_mul(u1, cof01), x87_mul(u0, cof00));
        let r = x87_store32(x87_add(r, x87_mul(u2, cof02)));
        e.mem.set_u32(st.r_out, r);
        let cof11 = stored(x87_sub(x87_mul(a22, a00), x87_mul(a02, a02)));
        let s = x87_add(x87_mul(cof11, u1), x87_mul(u0, cof01));
        let s = x87_store32(x87_add(s, x87_mul(u2, cof12)));
        e.mem.set_u32(st.s_out, s);
        let cof22 = stored(x87_sub(x87_mul(a00, a11), x87_mul(a01, a01)));
        let t = x87_add(x87_mul(u0, cof02), x87_mul(u1, cof12));
        let t = stored(x87_add(t, x87_mul(cof22, u2)));
        e.mem.set_u32(st.t_out, x87_store32(t));
        let r = x87_m32(r);
        let s = x87_m32(s);
        let one = f64::from_bits(e.mem.u64(ONE_01012070));
        let within = x87_add(load_float(e, st.s_out), t) <= one;
        if 0.0 > r {
            if within {
                if 0.0 > s {
                    st.start_with_edge(e, 0x18);
                    if 0.0 > t {
                        st.try_first_edge(e);
                        e.mem.set_u32(st.slot(0x14), 0);
                    }
                } else if 0.0 > t {
                    st.start_with_edge(e, 0xc);
                } else {
                    st.best = st.point_against_triangle(e, seg, st.s_out, st.t_out);
                    e.mem.set_u32(st.r_out, 0);
                    return x87_load32(st.best);
                }
            } else if 0.0 > s {
                st.start_with_edge(e, 0x18);
                st.try_far_edge(e, true);
            } else if 0.0 > t {
                st.start_with_edge(e, 0xc);
                st.try_far_edge(e, false);
            } else {
                st.start_with_far_edge(e);
            }
            st.try_end(e, seg, 0.0);
            return x87_load32(st.best);
        }
        if !(1.0 < r || r.is_nan()) {
            if within {
                if 0.0 > s {
                    st.start_with_edge(e, 0x18);
                    if 0.0 > t {
                        st.try_first_edge(e);
                    }
                } else if 0.0 > t {
                    st.start_with_edge(e, 0xc);
                } else {
                    // Inside the segment and the triangle: they meet.
                    return 0.0;
                }
            } else if 0.0 > s {
                st.start_with_edge(e, 0x18);
                st.try_far_edge(e, false);
            } else if 0.0 > t {
                st.start_with_edge(e, 0xc);
                st.try_far_edge(e, false);
            } else {
                st.start_with_far_edge(e);
            }
            return x87_load32(st.best);
        }
        if within {
            if 0.0 > s {
                st.start_with_edge(e, 0x18);
                if 0.0 > t {
                    st.try_first_edge(e);
                    e.mem.set_u32(st.slot(0x14), 0);
                }
            } else if 0.0 > t {
                st.start_with_edge(e, 0xc);
            } else {
                let end = st.end_by_call(e);
                st.best = st.point_against_triangle(e, end, st.s_out, st.t_out);
                e.mem.set_u32(st.r_out, x87_store32(1.0));
                return x87_load32(st.best);
            }
        } else if 0.0 > s {
            st.start_with_edge(e, 0x18);
            st.try_far_edge(e, false);
        } else if 0.0 > t {
            st.start_with_edge(e, 0xc);
            st.try_far_edge(e, false);
        } else {
            st.start_with_far_edge(e);
        }
        let end = st.end_by_call(e);
        st.try_end(e, end, 1.0);
        x87_load32(st.best)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_1d20,
        ni_sqr_distance_compute_ov3(Ptr, Ptr, Ptr, Ptr, Ptr) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<f32>)>>>;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(at + 4 * i, *v);
        }
        at
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Doubles for the callees: each distance call answers the next value of `answers` and
    /// writes 0.25 and 0.75 to its outs; the log keeps (callee, the block or point it got).
    fn doubles(e: &mut Engine, answers: Vec<f64>) -> Log {
        let log: Log = Default::default();
        let queue = std::rc::Rc::new(std::cell::RefCell::new(answers));
        for (callee, words) in [
            (SQR_DISTANCE_SEGMENT_SEGMENT, 6u32),
            (SQR_DISTANCE_POINT_TRIANGLE, 3),
        ] {
            let (log, queue) = (log.clone(), queue.clone());
            e.register_double(callee, move |e, a| {
                let at = if callee == SQR_DISTANCE_SEGMENT_SEGMENT {
                    a[1]
                } else {
                    a[0]
                };
                log.borrow_mut().push((callee, read(e, at, words)));
                e.mem.set_f32(a[2], 0.25);
                e.mem.set_f32(a[3], 0.75);
                Ret {
                    st0: queue.borrow_mut().remove(0),
                    ..Ret::default()
                }
            });
        }
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.register(POINT_PLUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_MINUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_d000, 0x1000);
        e.mem.set_f64(PARALLEL_FACTOR_0101DAE8, 1e-5);
        e.mem.set_f64(ONE_01012070, 1.0);
        log
    }

    /// The unit right triangle at the origin.
    fn triangle(e: &mut Engine) -> u32 {
        floats(e, &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    }

    fn run(e: &mut Engine, segment: u32, tri: u32) -> (f64, Vec<f32>) {
        let outs = e.mem.alloc(12);
        let d = e
            .call(0x00eb_1d20, &args![segment, tri, outs, outs + 4, outs + 8])
            .f64();
        (d, read(e, outs, 3))
    }

    #[test]
    fn a_crossing_segment_meets_the_triangle() {
        let mut e = Engine::new();
        let log = doubles(&mut e, vec![]);
        let tri = triangle(&mut e);
        let segment = floats(&mut e, &[0.25, 0.25, -1.0, 0.0, 0.0, 2.0]);
        assert_eq!(run(&mut e, segment, tri), (0.0, vec![0.5, 0.25, 0.25]));
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_segment_short_of_the_triangle_uses_its_start() {
        let mut e = Engine::new();
        let log = doubles(&mut e, vec![1.0]);
        let tri = triangle(&mut e);
        let segment = floats(&mut e, &[0.25, 0.25, 1.0, 0.0, 0.0, 2.0]);
        assert_eq!(run(&mut e, segment, tri), (1.0, vec![0.0, 0.25, 0.75]));
        assert_eq!(
            log.borrow().clone(),
            vec![(SQR_DISTANCE_POINT_TRIANGLE, vec![0.25, 0.25, 1.0])]
        );
    }

    #[test]
    fn beyond_the_far_edge_and_the_end() {
        let mut e = Engine::new();
        // The far edge answers 4, the end 3 (nearer).
        let log = doubles(&mut e, vec![4.0, 3.0]);
        let tri = triangle(&mut e);
        let segment = floats(&mut e, &[2.0, 2.0, -3.0, 0.0, 0.0, 2.0]);
        assert_eq!(run(&mut e, segment, tri), (3.0, vec![1.0, 0.25, 0.75]));
        let calls = log.borrow().clone();
        assert_eq!(
            calls[0],
            (
                SQR_DISTANCE_SEGMENT_SEGMENT,
                vec![1.0, 0.0, 0.0, -1.0, 1.0, 0.0]
            )
        );
        assert_eq!(
            calls[1],
            (SQR_DISTANCE_POINT_TRIANGLE, vec![2.0, 2.0, -1.0])
        );
    }

    #[test]
    fn a_parallel_segment_tries_every_edge_and_end() {
        let mut e = Engine::new();
        // Edge 0: 5, edge 1: 4, far edge: 6, start: 2, end: 2 (not nearer).
        let log = doubles(&mut e, vec![5.0, 4.0, 6.0, 2.0, 2.0]);
        let tri = triangle(&mut e);
        let segment = floats(&mut e, &[0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
        assert_eq!(run(&mut e, segment, tri), (2.0, vec![0.0, 0.25, 0.75]));
        let blocks: Vec<_> = log.borrow().iter().map(|(_, b)| b.clone()).collect();
        assert_eq!(blocks[0], vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        assert_eq!(blocks[1], vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(blocks[2], vec![1.0, 0.0, 0.0, -1.0, 1.0, 0.0]);
        assert_eq!(blocks[3], vec![0.0, 0.0, 1.0]);
        assert_eq!(blocks[4], vec![1.0, 0.0, 1.0]);
    }
}
