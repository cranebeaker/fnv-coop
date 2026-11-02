//! `NiCollision/run_00eaff20` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// `NiSqrDistance::Compute_ov2` (`00eaf310`, Xbox PDB; cdecl: two segments, two out floats): the
/// squared segment-segment distance, in ST0.
const SQR_DISTANCE_SEGMENT_SEGMENT: u32 = 0x00ea_f310;
/// `00eb6150` (cdecl: a point, a parallelogram, two out floats): the squared point-parallelogram
/// distance, in ST0.
const SQR_DISTANCE_POINT_PARALLELOGRAM: u32 = 0x00eb_6150;
/// `00439e90` (thiscall on a point: the out point, the other point): the sum; answers the out
/// point.
const POINT_PLUS_POINT: u32 = 0x0043_9e90;
/// `NiPoint3::Dot` (`004b6190`, thiscall on one vector: the other; answers in ST0).
const POINT_DOT: u32 = 0x004b_6190;
/// The CRT square root (`00ec6040`): the value in ST0 (a leading `f64`), the root in ST0.
const CRT_SQRT: u32 = 0x00ec_6040;
/// The double at `0101dae8` (about `1e-5`): `|n . d|` at or below it times `|n| |d|` makes the
/// segment parallel to the parallelogram.
const PARALLEL_FACTOR_0101DAE8: u32 = 0x0101_dae8;
/// The float at `010718c0` (1e-5): a determinant's magnitude below it counts as parallel.
const EPSILON_010718C0: u32 = 0x0107_18c0;
/// The double 2.0 at `01011590`.
const TWO_01011590: u32 = 0x0101_1590;

/// The frame slots (offsets from ESP after the exe's four register pushes): the segment block
/// the segment-segment calls take, the sum `00439e90` writes, the segment's end and the three
/// out floats of the candidate calls.
const BLOCK: u32 = 0x64;
const SUM: u32 = 0x58;
const END: u32 = 0x4c;
const FIRST: u32 = 0x18;
const SECOND: u32 = 0x14;
const OTHER: u32 = 0x10;

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

/// Where a parameter lies: 0 below 0, 2 above 1 (or a NaN), 1 on `[0, 1]`, as the exe's flag
/// tests sort it.
fn region(value: f64) -> u8 {
    if value < 0.0 {
        0
    } else if !(value <= 1.0) {
        2
    } else {
        1
    }
}

/// An out parameter's value: a frame slot (copied as a float) or a constant.
#[derive(Clone, Copy)]
enum Out {
    Slot(u32),
    Value(f64),
}

/// The state of `00eaff20`: the segment, the parallelogram, the out parameters, the frame and
/// the best distance so far (stored).
struct SegmentParallelogram {
    segment: u32,
    shape: u32,
    r_out: u32,
    s_out: u32,
    t_out: u32,
    frame: u32,
    best: u32,
}

impl SegmentParallelogram {
    fn slot(&self, offset: u32) -> u32 {
        self.frame + offset
    }

    /// The block as the parallelogram's origin and one of its edges (+0xc or +0x18).
    fn origin_block(&self, e: &mut Engine, edge: u32) {
        copy_words(e, self.shape, self.slot(BLOCK), 3);
        copy_words(e, self.shape + edge, self.slot(BLOCK) + 0xc, 3);
    }

    /// The block from the corner `origin + corner` (`00439e90` into the sum slot) along `edge`.
    fn corner_block(&self, e: &mut Engine, corner: u32, edge: u32) {
        let o = self.shape;
        let sum = e
            .call(POINT_PLUS_POINT, &args![o, self.slot(SUM), o + corner])
            .u32();
        copy_words(e, sum, self.slot(BLOCK), 3);
        copy_words(e, o + edge, self.slot(BLOCK) + 0xc, 3);
    }

    /// The segment against the block (`00eaf310`), the distance stored.
    fn against_block(&self, e: &mut Engine, first: u32, second: u32) -> u32 {
        let d = e
            .call(
                SQR_DISTANCE_SEGMENT_SEGMENT,
                &args![self.segment, self.slot(BLOCK), first, second],
            )
            .f64();
        x87_store32(d)
    }

    /// A point against the parallelogram (`00eb6150`), the distance stored.
    fn against_shape(&self, e: &mut Engine, point: u32, first: u32, second: u32) -> u32 {
        let d = e
            .call(
                SQR_DISTANCE_POINT_PARALLELOGRAM,
                &args![point, self.shape, first, second],
            )
            .f64();
        x87_store32(d)
    }

    /// The segment's end (`origin + direction`) through `00439e90`, copied to the end slot.
    fn end_by_call(&self, e: &mut Engine) -> u32 {
        let s = self.segment;
        let sum = e
            .call(POINT_PLUS_POINT, &args![s, self.slot(SUM), s + 0xc])
            .u32();
        copy_words(e, sum, self.slot(END), 3);
        self.slot(END)
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

    /// Writes a constant to an out parameter or a frame slot.
    fn set(&self, e: &mut Engine, at: u32, value: f64) {
        e.mem.set_u32(at, x87_store32(value));
    }

    /// Writes the outs: `r`, `s` and `t` each from a frame slot or a constant.
    fn write(&self, e: &mut Engine, values: [Out; 3]) {
        for (value, out) in values.into_iter().zip([self.r_out, self.s_out, self.t_out]) {
            match value {
                Out::Slot(offset) => move_float(e, self.slot(offset), out),
                Out::Value(v) => self.set(e, out, v),
            }
        }
    }

    /// A second edge candidate (`00eaf310` into the first and other slots): `(r, s, edge_t)`
    /// when nearer; `mark` also writes `edge_t` to the second slot first, as the exe does when
    /// a point candidate follows.
    fn try_edge(&mut self, e: &mut Engine, edge_t: f64, mark: bool) {
        let candidate = self.against_block(e, self.slot(FIRST), self.slot(OTHER));
        if mark {
            self.set(e, self.slot(SECOND), edge_t);
        }
        if self.nearer(candidate) {
            self.write(e, [Out::Slot(FIRST), Out::Slot(OTHER), Out::Value(edge_t)]);
        }
    }

    /// The edge candidates of the `(s, t)` region outside the parallelogram: the nearest edge
    /// straight into the outs, then for a corner region the other edge through the corner.
    fn edges(&mut self, e: &mut Engine, s: u8, t: u8, mark: bool) {
        let (r_out, s_out, t_out) = (self.r_out, self.s_out, self.t_out);
        match (s, t) {
            (1, 0) => {
                self.origin_block(e, 0xc);
                self.best = self.against_block(e, r_out, s_out);
                self.set(e, t_out, 0.0);
            }
            (1, _) => {
                self.corner_block(e, 0x18, 0xc);
                self.best = self.against_block(e, r_out, s_out);
                self.set(e, t_out, 1.0);
            }
            (0, _) => {
                self.origin_block(e, 0x18);
                self.best = self.against_block(e, r_out, t_out);
                self.set(e, s_out, 0.0);
                if t == 0 {
                    self.origin_block(e, 0xc);
                    self.try_edge(e, 0.0, mark);
                } else if t == 2 {
                    self.corner_block(e, 0x18, 0xc);
                    self.try_edge(e, 1.0, mark);
                }
            }
            _ => {
                self.corner_block(e, 0xc, 0x18);
                self.best = self.against_block(e, r_out, t_out);
                self.set(e, s_out, 1.0);
                if t == 0 {
                    self.origin_block(e, 0xc);
                    self.try_edge(e, 0.0, mark);
                } else if t == 2 {
                    self.corner_block(e, 0x18, 0xc);
                    self.try_edge(e, 1.0, mark);
                }
            }
        }
    }

    /// A segment end against the parallelogram as a candidate (`00eb6150` into the other and
    /// second slots): `(r_end, s, t)` when nearer.
    fn try_end(&mut self, e: &mut Engine, point: u32, r_end: f64) {
        let candidate = self.against_shape(e, point, self.slot(OTHER), self.slot(SECOND));
        if self.nearer(candidate) {
            self.write(e, [Out::Value(r_end), Out::Slot(OTHER), Out::Slot(SECOND)]);
        }
    }
}

// Translated from 00eaff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: a segment (origin, direction), a parallelogram (origin,
/// two edges: a segment and its displacement), the out parameters `r` (along the segment), `s`
/// and `t` (along the edges)): the squared distance between them. With `d = o_par - o_seg`, the
/// matrix of the directions' products (`a00 = D.D`, `a01 = -(e0.D)`, `a02 = -(e1.D)`,
/// `a11 = e0.e0`, `a12 = e1.e0`, `a22 = e1.e1`) and the normal `e0 x e1` (all stored): when
/// `|n.D|` is at most `|n| |D|` (`00ec6040` twice) times the double at `0101dae8` (or unordered),
/// or the cofactor determinant's magnitude is below the float at `010718c0`, the segment is
/// parallel and the least of six candidates is taken: the segment against the four edges
/// (`00eaf310`) and its two ends against the parallelogram (`00eb6150`). Otherwise the cofactors
/// solve for `(r, s, t)` (written) and, by the region of each (below 0, on `[0, 1]`, above 1),
/// the matching edges (through `00439e90` for the far corners) and the segment's start or end
/// give the candidates; inside all three the answer is the absolute value of the quadratic form
/// (`004b6190` for `d.d`). Each candidate replaces the outs only when strictly nearer; each
/// value is stored as a float in the exe's order.
pub fn fn_00eaff20(
    e: &mut Engine,
    segment: Ptr,
    shape: Ptr,
    out_r: Ptr,
    out_s: Ptr,
    out_t: Ptr,
) -> f64 {
    let (seg, par) = (segment.addr(), shape.addr());
    let (e0, e1) = (par + 0xc, par + 0x18);
    let dir = seg + 0xc;
    e.with_stack(0x7c, |e, frame| {
        let mut st = SegmentParallelogram {
            segment: seg,
            shape: par,
            r_out: out_r.addr(),
            s_out: out_s.addr(),
            t_out: out_t.addr(),
            frame: frame.addr(),
            best: 0,
        };
        let sum = st.slot(SUM);
        // The normal e0 x e1 (into the sum slot) and the parallel test.
        let nx = x87_sub(
            x87_mul(load_float(e, e1 + 8), float_operand(e, e0 + 4)),
            x87_mul(load_float(e, e1 + 4), float_operand(e, e0 + 8)),
        );
        let ny = x87_sub(
            x87_mul(load_float(e, e0 + 8), float_operand(e, e1)),
            x87_mul(load_float(e, e0), float_operand(e, e1 + 8)),
        );
        let nz = x87_sub(
            x87_mul(load_float(e, e1 + 4), float_operand(e, e0)),
            x87_mul(load_float(e, e1), float_operand(e, e0 + 4)),
        );
        for (i, n) in (0u32..).zip([nx, ny, nz]) {
            e.mem.set_u32(sum + 4 * i, x87_store32(n));
        }
        let [nx, ny, nz] = [0u32, 4, 8].map(|i| load_float(e, sum + i));
        let [dx, dy, dz] = [0u32, 4, 8].map(|i| load_float(e, dir + i));
        let along = x87_add(x87_mul(ny, dy), x87_mul(nx, dx));
        let along = x87_add(along, x87_mul(nz, float_operand(e, dir + 8)));
        let along = stored(stored(along).abs());
        let s = x87_add(x87_mul(ny, ny), x87_mul(nx, nx));
        let normal_squared = stored(x87_add(s, x87_mul(nz, nz)));
        let normal_length = stored(e.call(CRT_SQRT, &args![normal_squared]).f64());
        let s = x87_add(x87_mul(dx, dx), x87_mul(dy, dy));
        let direction_squared = stored(x87_add(s, x87_mul(dz, dz)));
        let direction_length = stored(e.call(CRT_SQRT, &args![direction_squared]).f64());
        let factor = x87_load64(e.mem.u64(PARALLEL_FACTOR_0101DAE8));
        let limit = stored(x87_mul(x87_mul(direction_length, normal_length), factor));
        let parallel = !(along > limit);
        let mut solution = None;
        if !parallel {
            for i in [0u32, 4, 8] {
                let v = x87_sub(load_float(e, par + i), float_operand(e, seg + i));
                e.mem.set_u32(sum + i, x87_store32(v));
            }
            let a00 = stored(dot(e, dir, dir));
            let a01 = stored(-stored(dot(e, e0, dir)));
            let a02 = stored(-stored(dot(e, e1, dir)));
            let a11 = stored(dot(e, e0, e0));
            let a12 = stored(dot(e, e1, e0));
            let a22 = stored(dot(e, e1, e1));
            let b0 = stored(-stored(dot(e, dir, sum)));
            let b1 = stored(dot(e, e0, sum));
            let b2 = stored(dot(e, e1, sum));
            let cof00 = stored(x87_sub(x87_mul(a22, a11), x87_mul(a12, a12)));
            let cof01 = stored(x87_sub(x87_mul(a12, a02), x87_mul(a22, a01)));
            let cof02 = stored(x87_sub(x87_mul(a12, a01), x87_mul(a11, a02)));
            let det = x87_add(x87_mul(cof01, a01), x87_mul(cof00, a00));
            let det = stored(x87_add(det, x87_mul(cof02, a02)));
            st.set(e, st.slot(OTHER), cof00);
            st.set(e, st.slot(SECOND), a22);
            st.set(e, st.slot(FIRST), a02);
            let epsilon = float_operand(e, EPSILON_010718C0);
            // A determinant below the epsilon (or unordered) leaves the segment parallel.
            if stored(det.abs()) >= epsilon {
                let cof12 = stored(x87_sub(x87_mul(a02, a01), x87_mul(a12, a00)));
                let inverse = stored(x87_div(1.0, det));
                let u0 = stored(x87_mul(-b0, inverse));
                let u1 = stored(x87_mul(-b1, inverse));
                let u2 = stored(x87_mul(-b2, inverse));
                let r = x87_add(x87_mul(u1, cof01), x87_mul(u0, cof00));
                let r = x87_store32(x87_add(r, x87_mul(u2, cof02)));
                e.mem.set_u32(st.r_out, r);
                let cof11 = stored(x87_sub(x87_mul(a22, a00), x87_mul(a02, a02)));
                let s = x87_add(x87_mul(cof11, u1), x87_mul(u0, cof01));
                let s = x87_store32(x87_add(s, x87_mul(u2, cof12)));
                e.mem.set_u32(st.s_out, s);
                let cof22 = stored(x87_sub(x87_mul(a11, a00), x87_mul(a01, a01)));
                let t = x87_add(x87_mul(u0, cof02), x87_mul(u1, cof12));
                let t = stored(x87_add(t, x87_mul(cof22, u2)));
                e.mem.set_u32(st.t_out, x87_store32(t));
                let terms = [a00, a01, a02, a11, a12, a22, b0, b1, b2];
                solution = Some((x87_m32(r), x87_m32(s), t, terms));
            }
        }
        let Some((r, s, t, terms)) = solution else {
            // Parallel: the four edges and both ends.
            st.origin_block(e, 0xc);
            st.best = st.against_block(e, st.r_out, st.s_out);
            st.set(e, st.t_out, 0.0);
            copy_words(e, e1, st.slot(BLOCK) + 0xc, 3);
            let candidate = st.against_block(e, st.slot(FIRST), st.slot(SECOND));
            st.set(e, st.slot(OTHER), 0.0);
            if st.nearer(candidate) {
                st.write(e, [Out::Slot(FIRST), Out::Value(0.0), Out::Slot(SECOND)]);
            }
            // The edge e0 from the corner origin + e1, the sums computed inline.
            for i in [0u32, 4, 8] {
                let v = x87_add(load_float(e, e1 + i), float_operand(e, par + i));
                e.mem.set_u32(sum + i, x87_store32(v));
            }
            copy_words(e, sum, st.slot(BLOCK), 3);
            copy_words(e, e0, st.slot(BLOCK) + 0xc, 3);
            let candidate = st.against_block(e, st.slot(FIRST), st.slot(OTHER));
            st.set(e, st.slot(SECOND), 1.0);
            if st.nearer(candidate) {
                st.write(e, [Out::Slot(FIRST), Out::Slot(OTHER), Out::Value(1.0)]);
            }
            // The edge e1 from the corner origin + e0.
            for i in [0u32, 4, 8] {
                let v = x87_add(load_float(e, e0 + i), float_operand(e, par + i));
                e.mem.set_u32(sum + i, x87_store32(v));
            }
            copy_words(e, sum, st.slot(BLOCK), 3);
            copy_words(e, e1, st.slot(BLOCK) + 0xc, 3);
            let candidate = st.against_block(e, st.slot(FIRST), st.slot(SECOND));
            st.set(e, st.slot(OTHER), 1.0);
            if st.nearer(candidate) {
                st.write(e, [Out::Slot(FIRST), Out::Value(1.0), Out::Slot(SECOND)]);
            }
            // The segment's start.
            let candidate = st.against_shape(e, seg, st.slot(OTHER), st.slot(SECOND));
            st.set(e, st.slot(FIRST), 0.0);
            if st.nearer(candidate) {
                st.write(e, [Out::Value(0.0), Out::Slot(OTHER), Out::Slot(SECOND)]);
            }
            // The segment's end, the sum computed inline.
            let x = x87_add(load_float(e, seg), float_operand(e, dir));
            let y = x87_add(load_float(e, dir + 4), float_operand(e, seg + 4));
            let z = x87_add(load_float(e, dir + 8), float_operand(e, seg + 8));
            for (i, v) in (0u32..).zip([x, y, z]) {
                e.mem.set_u32(sum + 4 * i, x87_store32(v));
            }
            copy_words(e, sum, st.slot(END), 3);
            st.try_end(e, st.slot(END), 1.0);
            return x87_load32(st.best);
        };
        let (rr, sr, tr) = (region(r), region(s), region(t));
        if (sr, tr) == (1, 1) {
            match rr {
                0 => {
                    st.best = st.against_shape(e, seg, st.s_out, st.t_out);
                    st.set(e, st.r_out, 0.0);
                }
                2 => {
                    let end = st.end_by_call(e);
                    st.best = st.against_shape(e, end, st.s_out, st.t_out);
                    st.set(e, st.r_out, 1.0);
                }
                _ => {
                    // Inside the segment and the parallelogram: the quadratic form.
                    let [a00, a01, a02, a11, a12, a22, b0, b1, b2] = terms;
                    let q = e.call(POINT_DOT, &args![sum, sum]).f64();
                    let r = load_float(e, st.r_out);
                    let s = load_float(e, st.s_out);
                    let two = x87_load64(e.mem.u64(TWO_01011590));
                    let x = x87_add(x87_mul(r, a01), x87_mul(s, a11));
                    let x = x87_add(x87_mul(t, a12), x);
                    let x = x87_add(x87_mul(b1, two), x);
                    let xs = x87_mul(x, float_operand(e, st.s_out));
                    let y = x87_add(x87_mul(a01, s), x87_mul(r, a00));
                    let y = x87_add(y, x87_mul(t, a02));
                    let y = x87_add(y, x87_mul(b0, two));
                    let z = x87_add(x87_mul(y, r), xs);
                    let w = x87_add(x87_mul(a12, s), x87_mul(r, a02));
                    let w = x87_add(w, x87_mul(t, a22));
                    let w = x87_add(w, x87_mul(two, b2));
                    let total = x87_add(q, x87_add(x87_mul(w, t), z));
                    return stored(stored(total).abs());
                }
            }
            return x87_load32(st.best);
        }
        st.edges(e, sr, tr, rr != 1);
        match rr {
            0 => st.try_end(e, seg, 0.0),
            2 => {
                if (sr, tr) == (0, 1) {
                    // The start's distance is computed and dropped.
                    st.against_shape(e, seg, st.slot(OTHER), st.slot(SECOND));
                }
                let end = st.end_by_call(e);
                st.try_end(e, end, 1.0);
            }
            _ => {}
        }
        x87_load32(st.best)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ea_ff20,
        fn_00eaff20(Ptr, Ptr, Ptr, Ptr, Ptr) -> f64
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
            (SQR_DISTANCE_POINT_PARALLELOGRAM, 3),
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
        e.register(POINT_DOT, |e, a| {
            let mut sum = 0.0f64;
            for i in 0..3 {
                sum += f64::from(e.mem.f32(a[0] + 4 * i)) * f64::from(e.mem.f32(a[1] + 4 * i));
            }
            sum.into_ret()
        });
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_d000, 0x1000);
        e.map(0x0107_1000, 0x1000);
        e.mem.set_f64(TWO_01011590, 2.0);
        e.mem.set_f64(PARALLEL_FACTOR_0101DAE8, 1e-5);
        e.mem.set_f32(EPSILON_010718C0, 1e-5);
        log
    }

    /// The unit square at the origin.
    fn square(e: &mut Engine) -> u32 {
        floats(e, &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    }

    fn run(e: &mut Engine, segment: u32, shape: u32) -> (f64, Vec<f32>) {
        let outs = e.mem.alloc(12);
        let d = e
            .call(
                0x00ea_ff20,
                &args![segment, shape, outs, outs + 4, outs + 8],
            )
            .f64();
        (d, read(e, outs, 3))
    }

    #[test]
    fn a_crossing_segment_meets_the_square() {
        let mut e = Engine::new();
        let log = doubles(&mut e, vec![]);
        let shape = square(&mut e);
        let segment = floats(&mut e, &[0.25, 0.5, -1.0, 0.0, 0.0, 2.0]);
        assert_eq!(run(&mut e, segment, shape), (0.0, vec![0.5, 0.25, 0.5]));
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_segment_short_of_the_square_uses_its_start() {
        let mut e = Engine::new();
        let log = doubles(&mut e, vec![1.0]);
        let shape = square(&mut e);
        let segment = floats(&mut e, &[0.25, 0.5, 1.0, 0.0, 0.0, 2.0]);
        assert_eq!(run(&mut e, segment, shape), (1.0, vec![0.0, 0.25, 0.75]));
        assert_eq!(
            log.borrow().clone(),
            vec![(SQR_DISTANCE_POINT_PARALLELOGRAM, vec![0.25, 0.5, 1.0])]
        );
    }

    #[test]
    fn beyond_the_far_corner_and_the_end() {
        let mut e = Engine::new();
        // The edge from corner e0 answers 4, the edge from corner e1 3, the end 2.
        let log = doubles(&mut e, vec![4.0, 3.0, 2.0]);
        let shape = square(&mut e);
        let segment = floats(&mut e, &[2.0, 2.0, -3.0, 0.0, 0.0, 2.0]);
        assert_eq!(run(&mut e, segment, shape), (2.0, vec![1.0, 0.25, 0.75]));
        let calls = log.borrow().clone();
        assert_eq!(calls[0].1, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(calls[1].1, vec![0.0, 1.0, 0.0, 1.0, 0.0, 0.0]);
        assert_eq!(
            calls[2],
            (SQR_DISTANCE_POINT_PARALLELOGRAM, vec![2.0, 2.0, -1.0])
        );
    }

    #[test]
    fn a_parallel_segment_tries_every_edge_and_end() {
        let mut e = Engine::new();
        let log = doubles(&mut e, vec![5.0, 4.0, 6.0, 3.0, 2.0, 2.0]);
        let shape = square(&mut e);
        let segment = floats(&mut e, &[0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
        assert_eq!(run(&mut e, segment, shape), (2.0, vec![0.0, 0.25, 0.75]));
        let blocks: Vec<_> = log.borrow().iter().map(|(_, b)| b.clone()).collect();
        assert_eq!(blocks[0], vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        assert_eq!(blocks[1], vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(blocks[2], vec![0.0, 1.0, 0.0, 1.0, 0.0, 0.0]);
        assert_eq!(blocks[3], vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(blocks[4], vec![0.0, 0.0, 1.0]);
        assert_eq!(blocks[5], vec![1.0, 0.0, 1.0]);
    }
}
