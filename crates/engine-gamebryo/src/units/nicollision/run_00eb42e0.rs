//! `NiCollision/run_00eb42e0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The squared distance from a point and from a segment to an oriented box (`NiBox` layout: the
//! center at +0, the three axes at +0xc, +0x18 and +0x24, the three extents at +0x30). The
//! segment case reflects the segment's direction into the positive octant in the box's frame and
//! sorts by which direction components are zero, the case split of Eberly's line-box distance.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` 0.0 at `01012060` the point-box distance adds to its first squared excess.
const ZERO_01012060: u32 = 0x0101_2060;

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

fn put(e: &mut Engine, address: u32, value: f64) {
    e.mem.set_u32(address, x87_store32(value));
}

/// The float `index` of the array at `base`.
fn at(base: u32, index: u32) -> u32 {
    base.wrapping_add(index.wrapping_mul(4))
}

/// `sum += excess^2`: the excess stored as a float first, the sum a float operand and stored.
fn add_square(e: &mut Engine, sum: u32, excess: f64) {
    let excess = round(excess);
    let total = x87_add(x87_mul(excess, excess), operand(e, sum));
    put(e, sum, total);
}

/// Clamps component `index` of `point` to `[-extent, extent]` (the extents array at `extents`),
/// adding the squared excess to `sum`. The excess below is `point + extent` (`point_first`) or
/// `extent + point`, the operand order of the caller's `FADD`; above it is `point - extent`.
fn clamp_component(
    e: &mut Engine,
    point: u32,
    extents: u32,
    sum: u32,
    index: u32,
    point_first: bool,
) {
    let component = at(point, index);
    let limit = at(extents, index);
    let value = load(e, component);
    let extent = load(e, limit);
    if -extent > value {
        let excess = if point_first {
            x87_add(value, operand(e, limit))
        } else {
            x87_add(extent, operand(e, component))
        };
        add_square(e, sum, excess);
        put(e, component, -extent);
    } else if extent < value {
        let excess = x87_sub(value, operand(e, limit));
        add_square(e, sum, excess);
        put(e, component, extent);
    }
}

/// One coordinate of `offset` in the box frame: the dot product with the axis at `axis`, summed
/// `(x + y) + z`; with `z_operand` the axis's z is a float operand of the `FMUL`.
fn axis_dot(e: &Engine, axis: u32, offset: [f64; 3], z_operand: bool) -> f64 {
    let x = x87_mul(load(e, axis), offset[0]);
    let y = x87_mul(load(e, axis.wrapping_add(4)), offset[1]);
    let z_axis = if z_operand {
        operand(e, axis.wrapping_add(8))
    } else {
        load(e, axis.wrapping_add(8))
    };
    x87_add(x87_add(x, y), x87_mul(z_axis, offset[2]))
}

// Translated from 00eb42e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eb42e0`, cdecl): the squared distance from `point` to the box `nibox`, as a
/// float in ST0, with the closest point's box-frame coordinates in `out0`, `out1` and `out2`.
/// `d = point - center` (stored); each coordinate is the dot product of an axis with `d`
/// (stored). Below `-extent` it becomes `-extent` and the stored excess `coordinate + extent` is
/// squared into the sum; above `extent` it becomes `extent` with the excess
/// `coordinate - extent`. The first axis adds its square to the `double` 0.0 at `01012060`, the
/// others to the stored sum.
pub fn fn_00eb42e0(e: &mut Engine, point: Ptr, nibox: Ptr, out0: Ptr, out1: Ptr, out2: Ptr) -> f64 {
    let p = point.addr();
    let b = nibox.addr();
    let mut sum = 0u32;
    let d = [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, p + offset),
            operand(e, b.wrapping_add(offset)),
        ))
    });
    let mut coordinates = [0.0f64; 3];
    for (axis, coordinate) in coordinates.iter_mut().enumerate() {
        let axis = axis as u32;
        let mut value = round(axis_dot(e, b.wrapping_add(0xc + 0xc * axis), d, false));
        let limit = b.wrapping_add(0x30 + 4 * axis);
        let extent = load(e, limit);
        let base = |e: &Engine, sum: u32| {
            if axis == 0 {
                x87_load64(e.mem.u64(ZERO_01012060))
            } else {
                x87_m32(sum)
            }
        };
        if -extent > value {
            let excess = round(x87_add(value, operand(e, limit)));
            sum = x87_store32(x87_add(x87_mul(excess, excess), base(e, sum)));
            value = round(-extent);
        } else if extent < value {
            let excess = round(x87_sub(value, operand(e, limit)));
            sum = x87_store32(x87_add(x87_mul(excess, excess), base(e, sum)));
            value = round(extent);
        }
        *coordinate = value;
    }
    e.mem.set_u32(out0.addr(), x87_store32(coordinates[0]));
    e.mem.set_u32(out1.addr(), x87_store32(coordinates[1]));
    e.mem.set_u32(out2.addr(), x87_store32(coordinates[2]));
    x87_load32(sum)
}

// Translated from 00eb4490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eb4490`), a register convention: `point` is ECX (three floats, box frame),
/// `nibox` EDX and `sum` ESI. The case of a segment direction with no positive component: clamps
/// each of the three coordinates to the box extents (+0x30), adding each squared excess to
/// `sum` (below the extent the excess is `extent + coordinate`).
pub fn fn_00eb4490(e: &mut Engine, point: Ptr, nibox: Ptr, sum: Ptr) {
    let extents = nibox.addr().wrapping_add(0x30);
    for index in 0..3 {
        clamp_component(e, point.addr(), extents, sum.addr(), index, false);
    }
}

/// The addresses `fn_00eb4590` works on.
struct Face {
    point: u32,
    direction: u32,
    extents: u32,
    offsets: u32,
    parameter: u32,
    sum: u32,
    i0: u32,
    i1: u32,
    i2: u32,
}

/// The two lengths and three products the edge cases of `fn_00eb4590` carry.
#[derive(Clone, Copy)]
struct Edge {
    /// The stored squared length of the two direction components in play.
    length: f64,
    /// `direction[i0] * offsets[i0]`.
    a: f64,
    /// `direction[other] * plus[other]`.
    b: f64,
    /// The stored edge test value.
    test: f64,
}

impl Face {
    fn d(&self, e: &Engine, index: u32) -> f64 {
        load(e, at(self.direction, index))
    }

    fn d_operand(&self, e: &Engine, index: u32) -> f64 {
        operand(e, at(self.direction, index))
    }

    fn extent(&self, index: u32) -> u32 {
        at(self.extents, index)
    }

    fn p(&self, index: u32) -> u32 {
        at(self.point, index)
    }

    fn m(&self, index: u32) -> u32 {
        at(self.offsets, index)
    }

    /// Stores the new sum (`total + sum`, the sum a float operand with `sum_operand`, else
    /// loaded) and the parameter, then the three coordinates.
    fn finish(
        &self,
        e: &mut Engine,
        total: f64,
        sum_operand: bool,
        parameter: f64,
        coordinates: [f64; 3],
    ) {
        let old = if sum_operand {
            operand(e, self.sum)
        } else {
            load(e, self.sum)
        };
        put(e, self.sum, x87_add(total, old));
        put(e, self.parameter, parameter);
        put(e, self.p(self.i0), coordinates[0]);
        put(e, self.p(self.i1), coordinates[1]);
        put(e, self.p(self.i2), coordinates[2]);
    }

    /// `(-delta) / stored(length + direction[index]^2)`, stored.
    fn parameter(&self, e: &Engine, delta: f64, length: f64, index: u32) -> f64 {
        let d = self.d(e, index);
        let total = x87_store32(x87_add(length, x87_mul(d, d)));
        round(x87_div(-delta, x87_m32(total)))
    }

    /// The edge along `i1` (with `plus` the stored `point + extent` per axis): the stored length
    /// `d0^2 + d2^2` and test `length * plus[i1] - (b + a) * d1`.
    fn first_edge(&self, e: &Engine, plus: &[u32; 3]) -> Edge {
        let (d0, d2) = (self.d(e, self.i0), self.d(e, self.i2));
        let length = round(x87_add(x87_mul(d0, d0), x87_mul(d2, d2)));
        let a = x87_mul(d0, operand(e, self.m(self.i0)));
        let b = x87_mul(d2, x87_m32(plus[self.i2 as usize]));
        let plus1 = x87_load32(plus[self.i1 as usize]);
        let test = round(x87_sub(
            x87_mul(plus1, length),
            x87_mul(x87_add(b, a), self.d_operand(e, self.i1)),
        ));
        Edge { length, a, b, test }
    }

    /// The edge along `i2`: the stored length `d0^2 + d1^2`, `b` the sum `a + d1 * plus[i1]`, and
    /// the test `length * plus[i2] - d2 * b`.
    fn second_edge(&self, e: &Engine, plus: &[u32; 3]) -> Edge {
        let (d0, d1) = (self.d(e, self.i0), self.d(e, self.i1));
        let length = round(x87_add(x87_mul(d0, d0), x87_mul(d1, d1)));
        let a = x87_mul(d0, operand(e, self.m(self.i0)));
        let b = x87_add(a, x87_mul(d1, x87_m32(plus[self.i1 as usize])));
        let test = round(x87_sub(
            x87_mul(length, x87_m32(plus[self.i2 as usize])),
            x87_mul(self.d(e, self.i2), b),
        ));
        Edge { length, a, b, test }
    }

    /// The closest point on the `i1` edge (`sum_operand`: the caller adds the old sum as a float
    /// operand).
    fn finish_first_edge(&self, e: &mut Engine, edge: &Edge, plus: &[u32; 3], sum_operand: bool) {
        let Edge { length, a, b, test } = *edge;
        let plus1 = x87_load32(plus[self.i1 as usize]);
        let plus2 = x87_load32(plus[self.i2 as usize]);
        let m0 = load(e, self.m(self.i0));
        let extent0 = load(e, self.extent(self.i0));
        let extent2 = load(e, self.extent(self.i2));
        let d1 = self.d(e, self.i1);
        let limit = x87_mul(x87_add(length, length), operand(e, self.extent(self.i1)));
        if !(limit < test) {
            let t = round(x87_div(test, length));
            let step = round(x87_sub(plus1, t));
            let delta = round(x87_add(b, x87_add(a, x87_mul(d1, step))));
            let parameter = self.parameter(e, delta, length, self.i1);
            let total = x87_add(x87_mul(m0, m0), x87_mul(step, step));
            let total = x87_add(x87_mul(plus2, plus2), total);
            let total = x87_add(total, x87_mul(delta, parameter));
            let along = x87_sub(t, operand(e, self.extent(self.i1)));
            self.finish(e, total, sum_operand, parameter, [extent0, along, -extent2]);
        } else {
            let c = x87_mul(d1, operand(e, self.m(self.i1)));
            let delta = round(x87_add(b, x87_add(a, c)));
            let parameter = self.parameter(e, delta, length, self.i1);
            let m1 = load(e, self.m(self.i1));
            let total = x87_add(x87_mul(m0, m0), x87_mul(m1, m1));
            let total = x87_add(total, x87_mul(plus2, plus2));
            let total = x87_add(total, x87_mul(delta, parameter));
            let extent1 = load(e, self.extent(self.i1));
            self.finish(e, total, false, parameter, [extent0, extent1, -extent2]);
        }
    }

    /// The closest point on the `i2` edge.
    fn finish_second_edge(&self, e: &mut Engine, edge: &Edge, plus: &[u32; 3]) {
        let Edge {
            length, b, test, ..
        } = *edge;
        let plus1 = x87_load32(plus[self.i1 as usize]);
        let plus2 = x87_load32(plus[self.i2 as usize]);
        let m0 = load(e, self.m(self.i0));
        let extent0 = load(e, self.extent(self.i0));
        let extent1 = load(e, self.extent(self.i1));
        let d2 = self.d(e, self.i2);
        let limit = x87_mul(x87_add(length, length), operand(e, self.extent(self.i2)));
        if !(limit < test) {
            let t = round(x87_div(test, length));
            let step = round(x87_sub(plus2, t));
            let delta = round(x87_add(b, x87_mul(d2, step)));
            let parameter = self.parameter(e, delta, length, self.i2);
            let total = x87_add(x87_mul(m0, m0), x87_mul(plus1, plus1));
            let total = x87_add(x87_mul(step, step), total);
            let total = x87_add(total, x87_mul(delta, parameter));
            let along = x87_sub(t, operand(e, self.extent(self.i2)));
            self.finish(e, total, false, parameter, [extent0, -extent1, along]);
        } else {
            let delta = round(x87_add(b, x87_mul(d2, operand(e, self.m(self.i2)))));
            let parameter = self.parameter(e, delta, length, self.i2);
            let m2 = load(e, self.m(self.i2));
            let total = x87_add(x87_mul(m0, m0), x87_mul(plus1, plus1));
            let total = x87_add(x87_mul(m2, m2), total);
            let total = x87_add(total, x87_mul(delta, parameter));
            let extent2 = load(e, self.extent(self.i2));
            self.finish(e, total, false, parameter, [extent0, -extent1, extent2]);
        }
    }
}

// Translated from 00eb4590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eb4590`), a register convention: `point` is EBX (box-frame coordinates,
/// updated to the closest box point), `i1` EDX, `i2` ESI and `direction` EDI; the cdecl stack
/// words are `nibox`, `i0`, `offsets` (`point - extent` per axis), `parameter` and `sum`. The
/// line meets the face `x[i0] = extent[i0]` side: with `plus = point + extent` (stored) for
/// `i1` and `i2`, it compares `d[i1] * offsets[i0]` with `d[i0] * plus[i1]` and
/// `offsets[i0] * d[i2]` with `d[i0] * plus[i2]`. Both inside: the line crosses the face and
/// the point moves along it by `1 / d[i0]` (stored) with the parameter `-offsets[i0] / d[i0]`.
/// Otherwise the closest point is on the `i1` edge, the `i2` edge or the corner (both edge tests
/// below zero), each adding its squared distance (summed in axis order, plus `delta *
/// parameter`) to `sum` and writing the parameter `-delta / length` and the box point.
pub fn fn_00eb4590(
    e: &mut Engine,
    point: Ptr,
    i1: u32,
    i2: u32,
    direction: Ptr,
    nibox: Ptr,
    i0: u32,
    offsets: Ptr,
    parameter: Ptr,
    sum: Ptr,
) {
    let face = Face {
        point: point.addr(),
        direction: direction.addr(),
        extents: nibox.addr().wrapping_add(0x30),
        offsets: offsets.addr(),
        parameter: parameter.addr(),
        sum: sum.addr(),
        i0,
        i1,
        i2,
    };
    let mut plus = [0u32; 3];
    for index in [i1, i2] {
        plus[index as usize] = x87_store32(x87_add(
            load(e, face.p(index)),
            operand(e, face.extent(index)),
        ));
    }
    let d0 = face.d(e, i0);
    let first = {
        let t1 = x87_mul(d0, x87_m32(plus[i1 as usize]));
        let t2 = x87_mul(face.d(e, i1), operand(e, face.m(i0)));
        t2 <= t1
    };
    let second = {
        let t3 = x87_mul(d0, x87_m32(plus[i2 as usize]));
        let t4 = x87_mul(load(e, face.m(i0)), face.d_operand(e, i2));
        t4 <= t3
    };
    if first && second {
        let extent0 = load(e, face.extent(i0));
        put(e, face.p(i0), extent0);
        let inverse = round(x87_div(1.0, d0));
        let shift = x87_mul(x87_mul(face.d(e, i1), operand(e, face.m(i0))), inverse);
        let moved = x87_sub(operand(e, face.p(i1)), shift);
        put(e, face.p(i1), moved);
        let shift = x87_mul(x87_mul(load(e, face.m(i0)), face.d_operand(e, i2)), inverse);
        let moved = x87_sub(load(e, face.p(i2)), shift);
        put(e, face.p(i2), moved);
        let value = x87_mul(-load(e, face.m(i0)), inverse);
        put(e, face.parameter, value);
    } else if first {
        let edge = face.first_edge(e, &plus);
        face.finish_first_edge(e, &edge, &plus, true);
    } else if second {
        let edge = face.second_edge(e, &plus);
        face.finish_second_edge(e, &edge, &plus);
    } else {
        let edge = face.first_edge(e, &plus);
        if edge.test >= 0.0 {
            face.finish_first_edge(e, &edge, &plus, false);
            return;
        }
        let other = face.second_edge(e, &plus);
        if other.test >= 0.0 {
            face.finish_second_edge(e, &other, &plus);
            return;
        }
        let delta = round(x87_add(other.b, edge.b));
        let d2 = face.d(e, i2);
        let length = round(x87_add(x87_mul(d2, d2), other.length));
        let value = round(x87_div(-delta, length));
        let m0 = load(e, face.m(i0));
        let plus1 = x87_load32(plus[i1 as usize]);
        let plus2 = x87_load32(plus[i2 as usize]);
        let total = x87_add(x87_mul(plus1, plus1), x87_mul(m0, m0));
        let total = x87_add(x87_mul(plus2, plus2), total);
        let total = x87_add(total, x87_mul(delta, value));
        let extent0 = load(e, face.extent(i0));
        let extent1 = load(e, face.extent(i1));
        let extent2 = load(e, face.extent(i2));
        face.finish(e, total, false, value, [extent0, -extent1, -extent2]);
    }
}

// Translated from 00eb4cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eb4cd0`), a register convention: `nibox` is ECX, `parameter` EAX and `sum`
/// EDX; the cdecl stack words are `point` (box frame) and `direction` (every component
/// positive). With `m = point - extent` (stored) it picks the face the line leaves through from
/// the stored products `d1 * m0`, `d0 * m1`, `m0 * d2` / `m1 * d2` and `d0 * m2` / `d1 * m2`,
/// and runs `fn_00eb4590` with the axes (0, 1, 2), (1, 2, 0) or (2, 0, 1).
pub fn fn_00eb4cd0(
    e: &mut Engine,
    nibox: Ptr,
    parameter: Ptr,
    sum: Ptr,
    point: Ptr,
    direction: Ptr,
) {
    let p = point.addr();
    let d = direction.addr();
    let extents = nibox.addr().wrapping_add(0x30);
    e.with_stack(12, |e, locals| {
        let m = locals.addr();
        for index in 0..3 {
            let value = x87_sub(load(e, at(p, index)), operand(e, at(extents, index)));
            put(e, at(m, index), value);
        }
        let m0 = load(e, m);
        let m1 = load(e, m + 4);
        let a = round(x87_mul(load(e, d + 4), m0));
        let b = round(x87_mul(load(e, d), m1));
        let (i0, i1, i2) = if b <= a {
            let c = round(x87_mul(m0, operand(e, d + 8)));
            let f = round(x87_mul(load(e, d), operand(e, m + 8)));
            if f <= c {
                (0, 1, 2)
            } else {
                (2, 0, 1)
            }
        } else {
            let c = round(x87_mul(m1, operand(e, d + 8)));
            let f = round(x87_mul(load(e, d + 4), operand(e, m + 8)));
            if f <= c {
                (1, 2, 0)
            } else {
                (2, 0, 1)
            }
        };
        fn_00eb4590(
            e, point, i1, i2, direction, nibox, i0, locals, parameter, sum,
        );
    });
}

// Translated from 00eb4de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eb4de0`), a register convention: `i0` is EDX, `i1` ECX, `point` EDI (box
/// frame, updated) and `direction` ESI; the cdecl stack words are `nibox`, `i2` (the axis whose
/// direction component is zero), `parameter` and `sum`. The line in the `i0`-`i1` plane: with
/// `m = point - extent` (stored) and the stored products `d1 * m0` and `d0 * m1`, the larger
/// picks the face it reaches. `delta = product - d * (point + extent)` (stored) at or above zero
/// adds `delta^2 / (d0^2 + d1^2)` to `sum` and moves the other coordinate to `-extent`;
/// otherwise the coordinate moves by `product / d` along the line. Last `point[i2]` is clamped
/// to its extent with the squared excess added to `sum`.
pub fn fn_00eb4de0(
    e: &mut Engine,
    i0: u32,
    i1: u32,
    point: Ptr,
    direction: Ptr,
    nibox: Ptr,
    i2: u32,
    parameter: Ptr,
    sum: Ptr,
) {
    let p = point.addr();
    let d = direction.addr();
    let extents = nibox.addr().wrapping_add(0x30);
    let s = sum.addr();
    let out = parameter.addr();
    let minus0 = round(x87_sub(load(e, at(p, i0)), operand(e, at(extents, i0))));
    let minus1 = round(x87_sub(load(e, at(p, i1)), operand(e, at(extents, i1))));
    let product0 = round(x87_mul(load(e, at(d, i1)), minus0));
    let product1 = round(x87_mul(load(e, at(d, i0)), minus1));
    let d0 = load(e, at(d, i0));
    let d1 = load(e, at(d, i1));
    let inverse_length =
        |d0: f64, d1: f64| round(x87_div(1.0, x87_add(x87_mul(d0, d0), x87_mul(d1, d1))));
    if product1 <= product0 {
        let extent0 = load(e, at(extents, i0));
        put(e, at(p, i0), extent0);
        let plus1 = round(x87_add(load(e, at(p, i1)), operand(e, at(extents, i1))));
        let delta = round(x87_sub(product0, x87_mul(d0, plus1)));
        if delta >= 0.0 {
            let inverse = inverse_length(d0, d1);
            let total = x87_add(x87_mul(x87_mul(delta, delta), inverse), load(e, s));
            put(e, s, total);
            let extent1 = load(e, at(extents, i1));
            put(e, at(p, i1), -extent1);
            let along = x87_mul(plus1, operand(e, at(d, i1)));
            let across = x87_add(x87_mul(minus0, d0), along);
            put(e, out, -x87_mul(across, inverse));
        } else {
            let inverse = round(x87_div(1.0, d0));
            let moved = x87_sub(load(e, at(p, i1)), x87_mul(product0, inverse));
            put(e, at(p, i1), moved);
            put(e, out, x87_mul(-minus0, inverse));
        }
    } else {
        let extent1 = load(e, at(extents, i1));
        put(e, at(p, i1), extent1);
        let plus0 = round(x87_add(load(e, at(extents, i0)), operand(e, at(p, i0))));
        let delta = round(x87_sub(product1, x87_mul(d1, plus0)));
        if delta >= 0.0 {
            let inverse = inverse_length(d0, d1);
            let total = x87_add(x87_mul(x87_mul(delta, delta), inverse), load(e, s));
            put(e, s, total);
            let extent0 = load(e, at(extents, i0));
            put(e, at(p, i0), -extent0);
            let across = x87_add(x87_mul(minus1, d1), x87_mul(plus0, operand(e, at(d, i0))));
            put(e, out, -x87_mul(across, inverse));
        } else {
            let inverse = round(x87_div(1.0, d1));
            let moved = x87_sub(load(e, at(p, i0)), x87_mul(product1, inverse));
            put(e, at(p, i0), moved);
            put(e, out, x87_mul(-minus1, inverse));
        }
    }
    clamp_component(e, p, extents, s, i2, true);
}

// Translated from 00eb5010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eb5010`), a register convention: `point` is ECX (box frame, updated), `i1`
/// ESI, `i2` EDI and `sum` EBX; the cdecl stack words are `nibox`, `i0` (the one axis with a
/// nonzero direction component), `direction` and `parameter`. The parameter is
/// `(extent[i0] - point[i0]) / direction[i0]` and `point[i0]` becomes `extent[i0]`; the other
/// two coordinates are clamped to their extents, the squared excesses added to `sum`.
pub fn fn_00eb5010(
    e: &mut Engine,
    point: Ptr,
    i1: u32,
    i2: u32,
    sum: Ptr,
    nibox: Ptr,
    i0: u32,
    direction: Ptr,
    parameter: Ptr,
) {
    let p = point.addr();
    let extents = nibox.addr().wrapping_add(0x30);
    let limit = load(e, at(extents, i0));
    let value = x87_div(
        x87_sub(limit, operand(e, at(p, i0))),
        operand(e, at(direction.addr(), i0)),
    );
    put(e, parameter.addr(), value);
    put(e, at(p, i0), limit);
    clamp_component(e, p, extents, sum.addr(), i1, true);
    clamp_component(e, p, extents, sum.addr(), i2, false);
}

// Translated from 00eb50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00eb50f0`, cdecl): the squared distance from the segment `segment` (origin at
/// +0, direction at +0xc) to the box `nibox`, as a float in ST0, with the segment parameter in
/// `parameter` and the closest box point's box-frame coordinates in `out0`..`out2`. The origin
/// offset and the direction go into the box frame (each stored); negative direction components
/// are reflected (both values negated, remembered per axis). The sum and parameter start at 0
/// and the case follows the positive components: all three `fn_00eb4cd0`, two `fn_00eb4de0`,
/// one `fn_00eb5010`, none `fn_00eb4490`. A parameter below 0 (or NaN) becomes 0 and above 1
/// becomes 1, and the answer is then the point-box distance (`fn_00eb42e0`) of that segment end;
/// otherwise the reflected coordinates are restored and the sum answered.
pub fn fn_00eb50f0(
    e: &mut Engine,
    segment: Ptr,
    nibox: Ptr,
    parameter: Ptr,
    out0: Ptr,
    out1: Ptr,
    out2: Ptr,
) -> f64 {
    let s = segment.addr();
    let b = nibox.addr();
    let out = parameter.addr();
    let d = [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, s + offset),
            operand(e, b.wrapping_add(offset)),
        ))
    });
    let mut local = [0u32; 6];
    for axis in 0..3u32 {
        let along = axis_dot(e, b.wrapping_add(0xc + 0xc * axis), d, axis == 2);
        local[axis as usize] = x87_store32(along);
    }
    let u = s.wrapping_add(0xc);
    for axis in 0..3u32 {
        let a = b.wrapping_add(0xc + 0xc * axis);
        let x = x87_mul(load(e, a), operand(e, u));
        let y = x87_mul(load(e, a + 4), operand(e, u + 4));
        let z = x87_mul(load(e, a + 8), operand(e, u + 8));
        local[3 + axis as usize] = x87_store32(x87_add(x87_add(y, x), z));
    }
    let mut reflected = [false; 3];
    for axis in 0..3 {
        if 0.0 > x87_m32(local[3 + axis]) {
            reflected[axis] = true;
            local[axis] = x87_store32(-x87_load32(local[axis]));
            local[3 + axis] = x87_store32(-x87_load32(local[3 + axis]));
        }
    }
    e.mem.set_u32(out, 0);
    let positive = [3usize, 4, 5].map(|slot| 0.0 < x87_m32(local[slot]));
    let (sum_value, coordinates) = e.with_stack(0x1c, |e, frame| {
        let sum = frame.addr();
        let point = sum + 4;
        let direction = sum + 0x10;
        e.mem.set_u32(sum, 0);
        for (slot, word) in local.iter().enumerate() {
            e.mem.set_u32(point + 4 * slot as u32, *word);
        }
        let (p, dir, total) = (Ptr::new(point), Ptr::new(direction), Ptr::new(sum));
        match positive {
            [true, true, true] => fn_00eb4cd0(e, nibox, parameter, total, p, dir),
            [true, true, false] => fn_00eb4de0(e, 0, 1, p, dir, nibox, 2, parameter, total),
            [true, false, true] => fn_00eb4de0(e, 0, 2, p, dir, nibox, 1, parameter, total),
            [true, false, false] => fn_00eb5010(e, p, 1, 2, total, nibox, 0, dir, parameter),
            [false, true, true] => fn_00eb4de0(e, 1, 2, p, dir, nibox, 0, parameter, total),
            [false, true, false] => fn_00eb5010(e, p, 0, 2, total, nibox, 1, dir, parameter),
            [false, false, true] => fn_00eb5010(e, p, 0, 1, total, nibox, 2, dir, parameter),
            [false, false, false] => fn_00eb4490(e, p, nibox, total),
        }
        let coordinates = [0u32, 4, 8].map(|offset| e.mem.u32(point + offset));
        (e.mem.u32(sum), coordinates)
    });
    let value = load(e, out);
    if !(value >= 0.0) {
        e.mem.set_u32(out, 0);
        return round(fn_00eb42e0(e, segment, nibox, out0, out1, out2));
    }
    if 1.0 < value {
        e.mem.set_u32(out, 1.0f32.to_bits());
        return e.with_stack(12, |e, end| {
            let x = x87_add(load(e, s), operand(e, u));
            let y = x87_add(load(e, u + 4), operand(e, s + 4));
            let z = x87_add(load(e, u + 8), operand(e, s + 8));
            put(e, end.addr(), x);
            put(e, end.addr() + 4, y);
            put(e, end.addr() + 8, z);
            round(fn_00eb42e0(e, end, nibox, out0, out1, out2))
        });
    }
    let mut coordinates = coordinates;
    for (axis, word) in coordinates.iter_mut().enumerate() {
        if reflected[axis] {
            *word = x87_store32(-x87_load32(*word));
        }
    }
    for (target, word) in [out0, out1, out2].iter().zip(coordinates) {
        e.mem.set_u32(target.addr(), x87_store32(x87_load32(word)));
    }
    x87_load32(sum_value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_42e0,
            fn_00eb42e0(Ptr, Ptr, Ptr, Ptr, Ptr) -> f64
        ),
        entry!(0x00eb_4490, fn_00eb4490(Ptr, Ptr, Ptr)),
        entry!(
            0x00eb_4590,
            fn_00eb4590(Ptr, u32, u32, Ptr, Ptr, u32, Ptr, Ptr, Ptr)
        ),
        entry!(0x00eb_4cd0, fn_00eb4cd0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00eb_4de0,
            fn_00eb4de0(u32, u32, Ptr, Ptr, Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00eb_5010,
            fn_00eb5010(Ptr, u32, u32, Ptr, Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00eb_50f0,
            fn_00eb50f0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> f64
        ),
    ]
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

    fn read(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(address + 4 * i)).collect()
    }

    fn close(got: f32, want: f64) -> bool {
        (f64::from(got) - want).abs() <= 1e-5 * (1.0 + want.abs())
    }

    /// An axis-aligned box at the origin with the extents `extents`.
    fn unit_box(e: &mut Engine, extents: [f32; 3]) -> u32 {
        let mut words = vec![0.0f32; 3];
        words.extend_from_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        words.extend_from_slice(&extents);
        floats(e, &words)
    }

    #[test]
    fn point_box_distance_clamps_each_axis() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ZERO_01012060, 0.0);
        let b = unit_box(&mut e, [1.0, 2.0, 3.0]);
        let point = floats(&mut e, &[3.0, -4.0, 0.5]);
        let outs = e.mem.alloc(12);
        let words = args![point, b, outs, outs + 4, outs + 8];
        assert_eq!(e.call(0x00eb_42e0, &words).f64(), 8.0);
        assert_eq!(read(&e, outs, 3), vec![1.0, -2.0, 0.5]);
        let inside = floats(&mut e, &[0.5, 1.0, -2.0]);
        let words = args![inside, b, outs, outs + 4, outs + 8];
        assert_eq!(e.call(0x00eb_42e0, &words).f64(), 0.0);
        assert_eq!(read(&e, outs, 3), vec![0.5, 1.0, -2.0]);
    }

    #[test]
    fn no_positive_direction_clamps_all_three() {
        let mut e = Engine::new();
        let b = unit_box(&mut e, [1.0, 1.0, 1.0]);
        let point = floats(&mut e, &[2.0, -3.0, 0.5]);
        let sum = floats(&mut e, &[0.0]);
        e.call(0x00eb_4490, &args![point, b, sum]);
        assert_eq!(e.mem.f32(sum), 5.0);
        assert_eq!(read(&e, point, 3), vec![1.0, -1.0, 0.5]);
    }

    #[test]
    fn face_crossing_and_corner() {
        let mut e = Engine::new();
        let b = unit_box(&mut e, [1.0, 1.0, 1.0]);
        let parameter = e.mem.alloc(4);
        let sum = floats(&mut e, &[0.0]);
        // The line crosses the x = 1 face.
        let point = floats(&mut e, &[-3.0, 0.0, 0.0]);
        let direction = floats(&mut e, &[6.0, 0.5, 0.25]);
        let offsets = floats(&mut e, &[-4.0, -1.0, -1.0]);
        let words = args![point, 1u32, 2u32, direction, b, 0u32, offsets, parameter, sum];
        e.call(0x00eb_4590, &words);
        let p = read(&e, point, 3);
        assert_eq!(p[0], 1.0);
        assert!(close(p[1], 1.0 / 3.0) && close(p[2], 1.0 / 6.0), "{p:?}");
        assert!(close(e.mem.f32(parameter), 2.0 / 3.0));
        assert_eq!(e.mem.f32(sum), 0.0);
        // Both edge tests below zero: the corner (1, -1, -1).
        let point = floats(&mut e, &[2.0, -3.0, -3.0]);
        let direction = floats(&mut e, &[1.0, 0.1, 0.1]);
        let offsets = floats(&mut e, &[1.0, -4.0, -4.0]);
        let words = args![point, 1u32, 2u32, direction, b, 0u32, offsets, parameter, sum];
        e.call(0x00eb_4590, &words);
        assert_eq!(read(&e, point, 3), vec![1.0, -1.0, -1.0]);
        assert!(close(e.mem.f32(parameter), -0.6 / 1.02));
        assert!(
            close(e.mem.f32(sum), 9.0 - 0.36 / 1.02),
            "{}",
            e.mem.f32(sum)
        );
    }

    #[test]
    fn no_zero_components_picks_the_face() {
        let mut e = Engine::new();
        let b = unit_box(&mut e, [1.0, 1.0, 1.0]);
        let parameter = e.mem.alloc(4);
        let sum = floats(&mut e, &[0.0]);
        let point = floats(&mut e, &[-3.0, 0.0, 0.0]);
        let direction = floats(&mut e, &[6.0, 0.5, 0.25]);
        e.call(0x00eb_4cd0, &args![b, parameter, sum, point, direction]);
        let p = read(&e, point, 3);
        assert_eq!(p[0], 1.0);
        assert!(close(p[1], 1.0 / 3.0) && close(p[2], 1.0 / 6.0), "{p:?}");
        assert!(close(e.mem.f32(parameter), 2.0 / 3.0));
        assert_eq!(e.mem.f32(sum), 0.0);
    }

    #[test]
    fn one_zero_component_in_both_orders() {
        let mut e = Engine::new();
        let b = unit_box(&mut e, [1.0, 1.0, 1.0]);
        let parameter = e.mem.alloc(4);
        let direction = floats(&mut e, &[1.0, 1.0, 0.0]);
        for (start, end) in [
            ([3.0f32, 0.0, 5.0], [1.0f32, -1.0, 1.0]),
            ([0.0, 3.0, 5.0], [-1.0, 1.0, 1.0]),
        ] {
            let point = floats(&mut e, &start);
            let sum = floats(&mut e, &[0.0]);
            let words = args![0u32, 1u32, point, direction, b, 2u32, parameter, sum];
            e.call(0x00eb_4de0, &words);
            assert_eq!(read(&e, point, 3), end.to_vec());
            assert_eq!(e.mem.f32(parameter), -1.5);
            assert_eq!(e.mem.f32(sum), 16.5);
        }
        // Delta below zero: the coordinate moves along the line instead.
        let point = floats(&mut e, &[3.0, 2.5, 0.0]);
        let sum = floats(&mut e, &[0.0]);
        let words = args![0u32, 1u32, point, direction, b, 2u32, parameter, sum];
        e.call(0x00eb_4de0, &words);
        assert_eq!(read(&e, point, 3), vec![1.0, 0.5, 0.0]);
        assert_eq!(e.mem.f32(parameter), -2.0);
        assert_eq!(e.mem.f32(sum), 0.0);
    }

    #[test]
    fn two_zero_components_reach_the_face() {
        let mut e = Engine::new();
        let b = unit_box(&mut e, [1.0, 1.0, 1.0]);
        let parameter = e.mem.alloc(4);
        let sum = floats(&mut e, &[0.0]);
        let point = floats(&mut e, &[-3.0, 5.0, 0.5]);
        let direction = floats(&mut e, &[2.0, 0.0, 0.0]);
        let words = args![point, 1u32, 2u32, sum, b, 0u32, direction, parameter];
        e.call(0x00eb_5010, &words);
        assert_eq!(e.mem.f32(parameter), 2.0);
        assert_eq!(read(&e, point, 3), vec![1.0, 1.0, 0.5]);
        assert_eq!(e.mem.f32(sum), 16.0);
    }

    /// The squared distance from `x` to the box (`center`, unit `axes`, `extents`).
    fn box_distance(center: [f64; 3], axes: [[f64; 3]; 3], extents: [f64; 3], x: [f64; 3]) -> f64 {
        let d = [x[0] - center[0], x[1] - center[1], x[2] - center[2]];
        (0..3)
            .map(|i| {
                let c = axes[i][0] * d[0] + axes[i][1] * d[1] + axes[i][2] * d[2];
                let excess = (c.abs() - extents[i]).max(0.0);
                excess * excess
            })
            .sum()
    }

    #[test]
    fn segment_box_distance_matches_a_search() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ZERO_01012060, 0.0);
        let center = [0.5, -0.25, 1.0];
        let axes = [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let extents = [1.0, 2.0, 0.5];
        let mut words = vec![0.5f32, -0.25, 1.0];
        words.extend_from_slice(&[0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 0.5]);
        let b = floats(&mut e, &words);
        let segment = e.mem.alloc(0x18);
        let parameter = e.mem.alloc(4);
        let outs = e.mem.alloc(12);
        let mut seed = 0x1234_5678u32;
        let mut next = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            f64::from(seed >> 8) / f64::from(1u32 << 24)
        };
        for round_index in 0..300 {
            let mut values = [0.0f32; 6];
            for (i, v) in values.iter_mut().enumerate() {
                let scale = if i < 3 { 4.0 } else { 6.0 };
                *v = ((next() * 2.0 - 1.0) * scale) as f32;
                if i >= 3 && next() < 0.25 {
                    *v = 0.0;
                }
            }
            for (i, v) in values.iter().enumerate() {
                e.mem.set_f32(segment + 4 * i as u32, *v);
            }
            let o: Vec<f64> = values[..3].iter().map(|v| f64::from(*v)).collect();
            let u: Vec<f64> = values[3..].iter().map(|v| f64::from(*v)).collect();
            let at = |t: f64| [o[0] + t * u[0], o[1] + t * u[1], o[2] + t * u[2]];
            let f = |t: f64| box_distance(center, axes, extents, at(t));
            let (mut lo, mut hi) = (0.0f64, 1.0f64);
            for _ in 0..200 {
                let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
                if f(m1) <= f(m2) {
                    hi = m2;
                } else {
                    lo = m1;
                }
            }
            let want = f(lo);
            let call = args![segment, b, parameter, outs, outs + 4, outs + 8];
            let got = e.call(0x00eb_50f0, &call).f64();
            let tolerance = 1e-3 * (1.0 + want);
            assert!(
                (got - want).abs() <= tolerance,
                "{round_index}: {values:?} {got} {want}"
            );
            let t = f64::from(e.mem.f32(parameter));
            assert!((0.0..=1.0).contains(&t), "{round_index}: {t}");
            assert!(
                (f(t) - want).abs() <= tolerance,
                "{round_index}: {t} {}",
                f(t)
            );
            // The box point is the closest one to the segment point at the parameter.
            let x = at(t);
            let d = [x[0] - center[0], x[1] - center[1], x[2] - center[2]];
            let out = read(&e, outs, 3);
            let gap: f64 = (0..3)
                .map(|i| {
                    let c = axes[i][0] * d[0] + axes[i][1] * d[1] + axes[i][2] * d[2];
                    (c - f64::from(out[i])).powi(2)
                })
                .sum();
            assert!(
                (gap - want).abs() <= tolerance,
                "{round_index}: {out:?} {gap} {want}"
            );
        }
    }
}
