//! `NiCollision/niobbox.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `NiOBBox` holds its model box (center at +0, axes at +0xc, +0x18 and +0x24, extents at
//! +0x30) and its world box in the same layout from +0x3c (center +0x3c, axes +0x48, +0x54 and
//! +0x60, extents +0x6c).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPoint3::Unitize` (`004a0c10`, thiscall on the vector).
const UNITIZE: u32 = 0x004a_0c10;
/// The `double` 0.0001 at `01032980` below which a unitized extent counts as flat.
const FLAT_EXTENT_01032980: u32 = 0x0103_2980;
/// The float at `01015f5c` (`-FLT_MAX`) the ray's entry starts at.
const RAY_START_01015F5C: u32 = 0x0101_5f5c;
/// The float at `01016970` (`FLT_MAX`) the ray's exit starts at.
const RAY_END_01016970: u32 = 0x0101_6970;
/// The `double` 1e-7 at `0109a3a0` below which the ray runs parallel to a slab.
const PARALLEL_0109A3A0: u32 = 0x0109_a3a0;

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

/// The product of the float at `loaded` (`FLD`) and the float operand at `multiplier`.
fn product(e: &Engine, loaded: u32, multiplier: u32) -> f64 {
    x87_mul(load(e, loaded), operand(e, multiplier))
}

/// `(first + second) + third` of three such products.
fn sum3(e: &Engine, terms: [(u32, u32); 3]) -> f64 {
    let [a, b, c] = terms.map(|(loaded, multiplier)| product(e, loaded, multiplier));
    x87_add(x87_add(a, b), c)
}

// Translated from 00eba480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBox::Transform` (Xbox PDB, thiscall): the world box from the model box and the transform
/// `transform` (rotation rows at +0, +0xc and +0x18, translation at +0x24, scale at +0x30). The
/// center is `translation + scale * (rotation * center)` (the rotated center, the scaled values
/// and the sums each stored), each axis `rotation * axis` and each extent `extent * scale`.
pub fn ni_obb_box_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    let b = this.addr();
    let t = transform.addr();
    let rotated = [
        sum3(e, [(t, b), (t + 4, b + 4), (t + 8, b + 8)]),
        sum3(e, [(t + 0xc, b), (t + 0x10, b + 4), (t + 0x14, b + 8)]),
        sum3(e, [(t + 0x1c, b + 4), (t + 0x18, b), (t + 0x20, b + 8)]),
    ]
    .map(x87_store32);
    let scale = round(load(e, t + 0x30));
    let scaled = [
        x87_store32(x87_mul(x87_load32(rotated[0]), scale)),
        x87_store32(x87_mul(x87_load32(rotated[1]), scale)),
        x87_store32(x87_mul(scale, x87_m32(rotated[2]))),
    ];
    for (axis, word) in scaled.iter().enumerate() {
        let offset = 4 * axis as u32;
        let center = x87_add(load(e, t + 0x24 + offset), x87_m32(*word));
        e.mem.set_u32(b + 0x3c + offset, x87_store32(center));
    }
    let x0 = sum3(e, [(t, b + 0xc), (b + 0x10, t + 4), (t + 8, b + 0x14)]);
    let y0 = sum3(
        e,
        [
            (t + 0xc, b + 0xc),
            (b + 0x10, t + 0x10),
            (t + 0x14, b + 0x14),
        ],
    );
    let z0 = sum3(
        e,
        [
            (b + 0x10, t + 0x1c),
            (t + 0x18, b + 0xc),
            (t + 0x20, b + 0x14),
        ],
    );
    for (offset, value) in [(0x48u32, x0), (0x4c, y0), (0x50, z0)] {
        e.mem.set_u32(b + offset, x87_store32(value));
    }
    let x1 = sum3(e, [(b + 0x1c, t + 4), (b + 0x18, t), (b + 0x20, t + 8)]);
    let y1 = sum3(
        e,
        [
            (b + 0x1c, t + 0x10),
            (b + 0x18, t + 0xc),
            (t + 0x14, b + 0x20),
        ],
    );
    let z1 = sum3(
        e,
        [
            (b + 0x1c, t + 0x1c),
            (t + 0x18, b + 0x18),
            (b + 0x20, t + 0x20),
        ],
    );
    for (offset, value) in [(0x54u32, x1), (0x58, y1), (0x5c, z1)] {
        e.mem.set_u32(b + offset, x87_store32(value));
    }
    let x2 = sum3(e, [(b + 0x24, t), (b + 0x28, t + 4), (b + 0x2c, t + 8)]);
    let y2 = sum3(
        e,
        [
            (b + 0x24, t + 0xc),
            (b + 0x28, t + 0x10),
            (t + 0x14, b + 0x2c),
        ],
    );
    let z2 = sum3(
        e,
        [
            (t + 0x1c, b + 0x28),
            (t + 0x18, b + 0x24),
            (b + 0x2c, t + 0x20),
        ],
    );
    for (offset, value) in [(0x60u32, x2), (0x64, y2), (0x68, z2)] {
        e.mem.set_u32(b + offset, x87_store32(value));
    }
    for axis in 0..3u32 {
        let extent = product(e, b + 0x30 + 4 * axis, t + 0x30);
        e.mem.set_u32(b + 0x6c + 4 * axis, x87_store32(extent));
    }
}

// Translated from 00ebf4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00ebf4a0`, thiscall on the box: `point`, `normal`): the world box's normal at
/// `point`. The world extents are copied and unitized (`004a0c10`); when one of them is at or
/// below the `double` 0.0001 at `01032980` (checked in axis order) the normal is that world
/// axis, copied. Otherwise with `d = point - center` (stored) each component is the box-frame
/// coordinate `axis . d` (stored) divided by the extent squared, and the result is unitized.
pub fn fn_00ebf4a0(e: &mut Engine, this: Ptr, point: Ptr, normal: Ptr) {
    let b = this.addr();
    let out = normal.addr();
    let unitized = e.with_stack(12, |e, local| {
        for offset in [0u32, 4, 8] {
            let extent = x87_store32(load(e, b + 0x6c + offset));
            e.mem.set_u32(local.addr() + offset, extent);
        }
        e.call(UNITIZE, &args![local]);
        [0u32, 4, 8].map(|offset| load(e, local.addr() + offset))
    });
    let flat = x87_load64(e.mem.u64(FLAT_EXTENT_01032980));
    for (axis, extent) in unitized.iter().enumerate() {
        if *extent <= flat {
            let source = b + 0x48 + 0xc * axis as u32;
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(source + offset);
                e.mem.set_u32(out.wrapping_add(offset), word);
            }
            return;
        }
    }
    let p = point.addr();
    let [dx, dy, dz] = [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, p.wrapping_add(offset)),
            operand(e, b + 0x3c + offset),
        ))
    });
    let along = |e: &Engine, axis: u32| {
        let x = x87_mul(load(e, axis), dx);
        let y = x87_mul(load(e, axis + 4), dy);
        let z = x87_mul(load(e, axis + 8), dz);
        x87_add(x87_add(x, y), z)
    };
    let first = round(along(e, b + 0x48));
    let e0 = load(e, b + 0x6c);
    e.mem
        .set_u32(out, x87_store32(x87_div(first, x87_mul(e0, e0))));
    let second = x87_store32(along(e, b + 0x54));
    let e1 = load(e, b + 0x70);
    let value = x87_div(x87_m32(second), x87_mul(e1, e1));
    e.mem.set_u32(out.wrapping_add(4), x87_store32(value));
    let third = x87_store32(along(e, b + 0x60));
    let e2 = load(e, b + 0x74);
    let value = x87_div(x87_m32(third), x87_mul(e2, e2));
    e.mem.set_u32(out.wrapping_add(8), x87_store32(value));
    e.call(UNITIZE, &args![normal]);
}

// Translated from 00ebf610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBox::TestIntersection` (Xbox PDB, the ray overload, thiscall: `origin`, `direction`):
/// the slab test of the ray against the world box. The entry starts at the float at `01015f5c`
/// and the exit at the float at `01016970`; per axis with `s = axis . (center - origin)` and
/// `f = axis . direction` (stored): `|f|` (stored) at or below the `double` 1e-7 at `0109a3a0`
/// misses when `|s|` exceeds `|extent|`; otherwise the two slab times `(s + |extent|) / f` and
/// `(s - |extent|) / f` (with `1 / f` stored, both stored) narrow the interval, which misses when
/// the entry passes the exit or the exit is behind the origin. Answers whether all three slabs
/// leave an interval.
pub fn ni_obb_box_test_intersection_ray(
    e: &mut Engine,
    this: Ptr,
    origin: Ptr,
    direction: Ptr,
) -> bool {
    let b = this.addr();
    let o = origin.addr();
    let u = direction.addr();
    let mut entry = x87_store32(load(e, RAY_START_01015F5C));
    let mut exit = round(load(e, RAY_END_01016970));
    let extents = [0x6cu32, 0x70, 0x74].map(|offset| x87_store32(round(load(e, b + offset).abs())));
    let [d0, d1, d2] = [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, b + 0x3c + offset),
            operand(e, o.wrapping_add(offset)),
        ))
    });
    let parallel = x87_load64(e.mem.u64(PARALLEL_0109A3A0));
    for (index, extent) in extents.iter().enumerate() {
        let axis = b + 0x48 + 0xc * index as u32;
        let s = x87_add(
            x87_add(
                x87_mul(d1, operand(e, axis + 4)),
                x87_mul(d0, operand(e, axis)),
            ),
            x87_mul(d2, operand(e, axis + 8)),
        );
        let s = x87_store32(s);
        let f = round(x87_add(
            x87_add(product(e, u, axis), product(e, axis + 4, u.wrapping_add(4))),
            product(e, u.wrapping_add(8), axis + 8),
        ));
        if round(f.abs()) > parallel {
            let inverse = round(x87_div(1.0, f));
            let start = x87_load32(s);
            let near = round(x87_mul(x87_add(x87_load32(*extent), start), inverse));
            let far = round(x87_mul(x87_sub(start, x87_m32(*extent)), inverse));
            let (high, low) = if far < near {
                (round(near), round(far))
            } else {
                (far, near)
            };
            if x87_load32(entry) < low {
                entry = x87_store32(low);
            }
            if high < exit {
                exit = round(high);
            }
            if x87_load32(entry) > exit || 0.0 > exit {
                return false;
            }
        } else if x87_load32(*extent) < round(x87_load32(s).abs()) {
            return false;
        }
    }
    true
}

/// `NiPoint3::Cross` (`004b3800`, thiscall on the left vector: the out vector, the right
/// vector; answers the out vector).
const POINT_CROSS: u32 = 0x004b_3800;
/// `NiPoint3::Dot` (`004b6190`, thiscall on one vector: the other; answers on the x87 stack).
const POINT_DOT: u32 = 0x004b_6190;

/// A value stored as a float and its absolute value (`FSTP`, `FABS`, `FSTP`, `FLD`).
fn stored_abs(value: f64) -> f64 {
    round(value).abs()
}

/// `axis . vector` with the axis's floats in memory and the vector's on the x87 stack, the x and
/// y products added first (stored).
fn project(e: &Engine, axis: u32, vector: [f64; 3]) -> f64 {
    let term = |i: usize| x87_mul(load(e, axis + 4 * i as u32), vector[i]);
    round(x87_add(x87_add(term(0), term(1)), term(2)))
}

/// A face axis's test: `start` and `end` are the boxes' separation along the axis now and after
/// the motion, `radius` the boxes' summed projected half sizes; separated when both lie beyond
/// `radius` or both below `-radius` (stored), a NaN never separating.
fn face_separates(start: f64, end: f64, radius: f64) -> bool {
    if radius < start && end > radius {
        return true;
    }
    let minus = round(-radius);
    minus > start && end < minus
}

/// An edge-pair axis's test, in the exe's branch order: `start` beyond `radius` separates when
/// `end` is too, otherwise both below `-radius` (stored) separate.
fn edge_separates(start: f64, end: f64, radius: f64) -> bool {
    if radius < start {
        return end > radius;
    }
    let minus = round(-radius);
    minus > start && end < minus
}

/// The radius of an edge-pair axis: two pairs of `|extent * c|` (each stored), each pair's sum
/// stored and the two sums added (stored).
fn edge_radius(e: &Engine, terms: [(u32, f64); 4]) -> f64 {
    let [a, b, c, d] = terms.map(|(extent, c)| stored_abs(x87_mul(load(e, extent), c)));
    round(x87_add(round(x87_add(a, b)), round(x87_add(c, d))))
}

/// One swept term `(p, cp, q, cq, extent)`: `|p cp - q cq|` (stored) times the extent.
type SweptTerm = (f64, f64, f64, f64, u32);

/// A swept axis's test (the cross product of the motion and the centers' offset, tested against
/// one box axis): each `|p cp - q cq|` stored and times its extent, the three summed (stored)
/// plus `own` (stored); separated when that falls below `distance`.
fn swept_separates(e: &Engine, distance: f64, terms: [SweptTerm; 3], own: f64) -> bool {
    let [a, b, c] = terms.map(|(p, cp, q, cq, extent)| {
        let t = stored_abs(x87_sub(x87_mul(p, cp), x87_mul(q, cq)));
        x87_mul(t, operand(e, extent))
    });
    let radius = round(x87_add(round(x87_add(x87_add(a, b), c)), own));
    radius < distance
}

/// `first.0 * extent + second.0 * extent` (stored), the swept axis's own box's part.
fn swept_own(e: &Engine, first: (f64, u32), second: (f64, u32)) -> f64 {
    round(x87_add(
        x87_mul(first.0, operand(e, first.1)),
        x87_mul(second.0, operand(e, second.1)),
    ))
}

/// One edge pair's radius terms `(extent, c)` and its two separation terms `(face, c)`: the
/// separation is `s[p] c_p - s[q] c_q` of the face separations `s`.
type EdgePair = ([(u32, f64); 4], (usize, f64), (usize, f64));

/// `c_ij = A_i . B_j` (each stored, the products of the other box's axis loaded) of this box
/// `a` and the other box `b`.
fn axis_products(e: &Engine, a: u32, b: u32) -> [[f64; 3]; 3] {
    [0u32, 1, 2].map(|i| {
        [0u32, 1, 2].map(|j| {
            let (p, q) = (b + 0x48 + 0xc * j, a + 0x48 + 0xc * i);
            round(x87_add(
                x87_add(product(e, p, q), product(e, p + 4, q + 4)),
                product(e, p + 8, q + 8),
            ))
        })
    })
}

/// The nine edge pairs `A_i x B_j` of this box `a` and the other box `b` in the exe's order:
/// the radius terms (an extent and its `c_ij`) and the separation terms.
fn edge_pairs(a: u32, b: u32, c: &[[f64; 3]; 3]) -> [EdgePair; 9] {
    let (ax, ay, az) = (a + 0x6c, a + 0x70, a + 0x74);
    let (bx, by, bz) = (b + 0x6c, b + 0x70, b + 0x74);
    [
        (
            [(by, c[0][2]), (bz, c[0][1]), (ay, c[2][0]), (az, c[1][0])],
            (2, c[1][0]),
            (1, c[2][0]),
        ),
        (
            [(bx, c[0][2]), (bz, c[0][0]), (ay, c[2][1]), (az, c[1][1])],
            (2, c[1][1]),
            (1, c[2][1]),
        ),
        (
            [(bx, c[0][1]), (by, c[0][0]), (ay, c[2][2]), (az, c[1][2])],
            (2, c[1][2]),
            (1, c[2][2]),
        ),
        (
            [(ax, c[2][0]), (az, c[0][0]), (by, c[1][2]), (bz, c[1][1])],
            (0, c[2][0]),
            (2, c[0][0]),
        ),
        (
            [(bx, c[1][2]), (bz, c[1][0]), (ax, c[2][1]), (az, c[0][1])],
            (0, c[2][1]),
            (2, c[0][1]),
        ),
        (
            [(bx, c[1][1]), (by, c[1][0]), (ax, c[2][2]), (az, c[0][2])],
            (0, c[2][2]),
            (2, c[0][2]),
        ),
        (
            [(by, c[2][2]), (bz, c[2][1]), (ax, c[1][0]), (ay, c[0][0])],
            (1, c[0][0]),
            (0, c[1][0]),
        ),
        (
            [(bx, c[2][2]), (bz, c[2][0]), (ax, c[1][1]), (ay, c[0][1])],
            (1, c[0][1]),
            (0, c[1][1]),
        ),
        (
            [(bx, c[2][1]), (by, c[2][0]), (ax, c[1][2]), (ay, c[0][2])],
            (1, c[0][2]),
            (0, c[1][2]),
        ),
    ]
}

// Translated from 00eba680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBox::TestIntersection` (Xbox PDB, thiscall: the other box): whether the two world boxes
/// meet during the motion. Each box's owner at +0x78 holds the motion's time at +4 and its
/// position at +0x10; `D` is the other center minus this center, `V` the other position minus
/// this position and `c_ij = A_i . B_j` (stored) for this box's axes `A` and the other's `B`.
/// Along each of the six face axes the separation now (`axis . D`) and after the motion (plus
/// `time * axis . V`, each stored) both beyond the summed projected extents separates; then the
/// nine edge-pair axes the same way from the `c_ij` and the face separations; then `N = V x D`
/// (`004b3800`, into the exe's frame) is tested along the six box axes against the extents
/// projected on `V x axis` (the dot products by `004b6190`). Answers false at the first
/// separating axis and true when none separates, each value stored as a float in the exe's order.
pub fn ni_obb_box_test_intersection(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    e.with_stack(0x6c, |e, frame| {
        let frame = frame.addr();
        // The exe's frame: D at +0x3c, V at +0x30 and N at +0x18.
        let (d_at, v_at, n_at) = (frame + 0x3c, frame + 0x30, frame + 0x18);
        let (root_b, root_a) = (e.mem.u32(b + 0x78), e.mem.u32(a + 0x78));
        for i in [0u32, 4, 8] {
            let d = x87_sub(load(e, b + 0x3c + i), operand(e, a + 0x3c + i));
            e.mem.set_u32(d_at + i, x87_store32(d));
        }
        for i in [0u32, 4, 8] {
            let v = x87_sub(load(e, root_b + 0x10 + i), operand(e, root_a + 0x10 + i));
            e.mem.set_u32(v_at + i, x87_store32(v));
        }
        let time = round(load(e, root_a + 4));
        let d = [0u32, 4, 8].map(|i| load(e, d_at + i));
        let v = [0u32, 4, 8].map(|i| load(e, v_at + i));
        let axis_a = |i: usize| a + 0x48 + 0xc * i as u32;
        let axis_b = |j: usize| b + 0x48 + 0xc * j as u32;
        let extent_a = |i: usize| a + 0x6c + 4 * i as u32;
        let extent_b = |j: usize| b + 0x6c + 4 * j as u32;
        let c = axis_products(e, a, b);
        // This box's face axes.
        let start = [0usize, 1, 2].map(|i| project(e, axis_a(i), d));
        let end = [0usize, 1, 2]
            .map(|i| round(x87_add(x87_mul(project(e, axis_a(i), v), time), start[i])));
        for i in [0usize, 1, 2] {
            let [p, q, r] =
                [0usize, 1, 2].map(|j| stored_abs(x87_mul(load(e, extent_b(j)), c[i][j])));
            let sum = round(x87_add(x87_add(p, q), r));
            let radius = round(x87_add(sum, load(e, extent_a(i)).abs()));
            if face_separates(start[i], end[i], radius) {
                return false;
            }
        }
        // The other box's face axes.
        for j in [0usize, 1, 2] {
            let s0 = project(e, axis_b(j), d);
            let s1 = round(x87_add(x87_mul(project(e, axis_b(j), v), time), s0));
            let [p, q, r] =
                [0usize, 1, 2].map(|i| stored_abs(x87_mul(load(e, extent_a(i)), c[i][j])));
            let sum = round(x87_add(x87_add(p, q), r));
            let radius = round(x87_add(sum, load(e, extent_b(j)).abs()));
            if face_separates(s0, s1, radius) {
                return false;
            }
        }
        // The edge pairs A_i x B_j in the exe's order.
        let (ax, ay, az) = (extent_a(0), extent_a(1), extent_a(2));
        let (bx, by, bz) = (extent_b(0), extent_b(1), extent_b(2));
        for (terms, (p, cp), (q, cq)) in edge_pairs(a, b, &c) {
            let radius = edge_radius(e, terms);
            let s0 = round(x87_sub(x87_mul(start[p], cp), x87_mul(start[q], cq)));
            let s1 = round(x87_sub(x87_mul(end[p], cp), x87_mul(end[q], cq)));
            if edge_separates(s0, s1, radius) {
                return false;
            }
        }
        // The swept axes: N = V x D against each box axis.
        e.call(POINT_CROSS, &args![v_at, n_at, d_at]);
        let dot = |e: &mut Engine, this: u32, other: u32| {
            round(e.call(POINT_DOT, &args![this, other]).f64())
        };
        let distance =
            |e: &mut Engine, axis: u32| round(e.call(POINT_DOT, &args![axis, n_at]).f64().abs());
        let q1 = dot(e, v_at, axis_a(1));
        let q2 = dot(e, v_at, axis_a(2));
        let (aq1, aq2) = (q1.abs(), q2.abs());
        let n = distance(e, axis_a(0));
        let own = swept_own(e, (aq2, ay), (aq1, az));
        let terms = [
            (q2, c[1][1], q1, c[2][1], by),
            (q2, c[1][0], q1, c[2][0], bx),
            (q2, c[1][2], q1, c[2][2], bz),
        ];
        if swept_separates(e, n, terms, own) {
            return false;
        }
        let q0 = dot(e, v_at, axis_a(0));
        let aq0 = q0.abs();
        let n = distance(e, axis_a(1));
        let own = swept_own(e, (aq2, ax), (aq0, az));
        let terms = [
            (q0, c[2][1], q2, c[0][1], by),
            (q0, c[2][0], q2, c[0][0], bx),
            (q0, c[2][2], q2, c[0][2], bz),
        ];
        if swept_separates(e, n, terms, own) {
            return false;
        }
        let n = distance(e, axis_a(2));
        let own = swept_own(e, (aq1, ax), (aq0, ay));
        let terms = [
            (q1, c[0][1], q0, c[1][1], by),
            (q1, c[0][0], q0, c[1][0], bx),
            (q1, c[0][2], q0, c[1][2], bz),
        ];
        if swept_separates(e, n, terms, own) {
            return false;
        }
        let r1 = dot(e, v_at, axis_b(1));
        let r2 = dot(e, v_at, axis_b(2));
        let (ar1, ar2) = (r1.abs(), r2.abs());
        let n = distance(e, axis_b(0));
        let own = swept_own(e, (ar1, bz), (ar2, by));
        let terms = [
            (r2, c[0][1], r1, c[0][2], ax),
            (r2, c[1][1], r1, c[1][2], ay),
            (r2, c[2][1], r1, c[2][2], az),
        ];
        if swept_separates(e, n, terms, own) {
            return false;
        }
        let r0 = dot(e, v_at, axis_b(0));
        let ar0 = r0.abs();
        let n = distance(e, axis_b(1));
        let own = swept_own(e, (ar0, bz), (ar2, bx));
        let terms = [
            (r0, c[0][2], r2, c[0][0], ax),
            (r0, c[1][2], r2, c[1][0], ay),
            (r0, c[2][2], r2, c[2][0], az),
        ];
        if swept_separates(e, n, terms, own) {
            return false;
        }
        let n = distance(e, axis_b(2));
        let own = swept_own(e, (ar0, by), (ar1, bx));
        let terms = [
            (r1, c[0][0], r0, c[0][1], ax),
            (r1, c[1][0], r0, c[1][1], ay),
            (r1, c[2][0], r0, c[2][1], az),
        ];
        !swept_separates(e, n, terms, own)
    })
}

/// `00439e90` (thiscall on a point: the out point, the other point): the sum; answers the out
/// point.
const POINT_PLUS_POINT: u32 = 0x0043_9e90;
/// `004a3760` (cdecl: the out point, a float, a point): the point scaled; answers the out point.
const POINT_SCALE: u32 = 0x004a_3760;
/// The float 1e-6 at `010959f4`: an edge pair at or below `|1 - c^2|` of it counts as parallel.
const PARALLEL_010959F4: u32 = 0x0109_59f4;

/// One axis of the swept test: the separations `start` (now) and `end` (after the motion) and the
/// summed projected half sizes `radius`.
enum Sweep {
    /// Apart along the axis for the whole motion.
    Separated,
    /// Apart now, meeting after the time (stored) on the given side.
    Contact(f64, i32),
    /// Overlapping now along the axis.
    Overlapping,
}

/// The swept test of one axis in the exe's branch order: beyond `radius` (the other side when
/// not, against `-radius` stored), separated when `end` stays beyond, otherwise the time
/// `(bound - start) * time / (end - start)` (stored) at which it reaches the bound.
fn sweep(start: f64, end: f64, radius: f64, time: f64) -> Sweep {
    if radius < start {
        if end > radius {
            return Sweep::Separated;
        }
        let at = x87_div(x87_mul(x87_sub(radius, start), time), x87_sub(end, start));
        return Sweep::Contact(round(at), 1);
    }
    let minus = round(-radius);
    if minus > start {
        if end < minus {
            return Sweep::Separated;
        }
        let at = x87_div(x87_mul(x87_sub(minus, start), time), x87_sub(end, start));
        return Sweep::Contact(round(at), -1);
    }
    Sweep::Overlapping
}

/// The latest contact so far: its time (stored), its axis (1 to 15, 0 for none) and its side.
struct FirstContact {
    time: f64,
    axis: u32,
    side: i32,
}

impl FirstContact {
    /// Takes one axis's test: false when it separates the boxes; a contact later than the
    /// latest becomes the latest.
    fn take(&mut self, axis: u32, sweep: Sweep) -> bool {
        match sweep {
            Sweep::Separated => false,
            Sweep::Contact(time, side) => {
                if time > self.time {
                    *self = FirstContact { time, axis, side };
                }
                true
            }
            Sweep::Overlapping => true,
        }
    }
}

/// `side * polarity * extent` (stored) by the sign of `c`: the extent's sign toward the contact,
/// 0 when `c` is 0 or a NaN.
fn signed_extent(e: &Engine, c: f64, polarity: i32, side: i32, extent: u32) -> f64 {
    let factor = if c > 0.0 {
        polarity * side
    } else if c < 0.0 {
        -polarity * side
    } else {
        return 0.0;
    };
    round(x87_mul(f64::from(factor), operand(e, extent)))
}

/// The sign of the permutation `(i, m, n)` of `(0, 1, 2)`.
fn permutation_sign(i: usize, m: usize) -> i32 {
    if (i + 1) % 3 == m {
        1
    } else {
        -1
    }
}

/// The contact point `center + time * velocity + x A0 + y A1 + z A2` through `004a3760` and
/// `00439e90` in the exe's order, into the frame slots `slots` (the four scaled points, then the
/// four sums), copied to `out`.
fn contact_point(
    e: &mut Engine,
    frame: u32,
    from: (u32, u32, u32),
    coefficients: [f64; 3],
    time: u32,
    slots: [u32; 8],
    out: u32,
) {
    let (center, velocity, axes) = from;
    let [x, y, z] = coefficients.map(x87_store32);
    let scale = |e: &mut Engine, slot: u32, factor: u32, point: u32| {
        e.call(POINT_SCALE, &args![frame + slot, factor, point])
            .u32()
    };
    let along_z = scale(e, slots[0], z, axes + 0x18);
    let along_y = scale(e, slots[1], y, axes + 0xc);
    let along_x = scale(e, slots[2], x, axes);
    let moved = scale(e, slots[3], time, velocity);
    let mut point = center;
    for (slot, term) in slots[4..].iter().zip([moved, along_x, along_y, along_z]) {
        point = e
            .call(POINT_PLUS_POINT, &args![point, frame + slot, term])
            .u32();
    }
    for i in [0u32, 4, 8] {
        let word = e.mem.u32(point + i);
        e.mem.set_u32(out + i, word);
    }
}

// Translated from 00ebc2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall: the other box, the out time, the out point; an
/// `NiOBBox` method, NiCollision): the first contact of the two world boxes during the motion.
/// With `D`, `V` (the owners' velocities at +0x10 subtracted), the time at this owner's +4 and
/// `c_ij` as in `NiOBBox::TestIntersection`, each of the fifteen separating axes is swept: apart
/// for the whole motion answers false; apart now, the time it reaches the summed extents is a
/// candidate and the latest candidate (from 0) gives the contact's axis and side. The exe's
/// second and third face axes compare their radii against the first face axis's separations
/// (as compiled). The time is written; for a face axis the contact point is the other box's
/// (or this box's) corner toward the contact, moved by the time; for an edge pair the point on
/// this box's edge nearest the other's (solved through `004b6190` after moving `D` by the time
/// along `V`, unless the edges are parallel). Answers true.
pub fn fn_00ebc2a0(e: &mut Engine, this: Ptr, other: Ptr, out_time: Ptr, out_point: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    e.with_stack(0x490, |e, frame| {
        let frame = frame.addr();
        // The exe's frame: D at +0x48, V at +0x54, the doubles at +0x20.
        let (d_at, v_at, wide) = (frame + 0x48, frame + 0x54, frame + 0x20);
        for i in [0u32, 4, 8] {
            let d = x87_sub(load(e, b + 0x3c + i), operand(e, a + 0x3c + i));
            e.mem.set_u32(d_at + i, x87_store32(d));
        }
        let (root_b, root_a) = (e.mem.u32(b + 0x78), e.mem.u32(a + 0x78));
        for i in [0u32, 4, 8] {
            let v = x87_sub(load(e, root_b + 0x10 + i), operand(e, root_a + 0x10 + i));
            e.mem.set_u32(v_at + i, x87_store32(v));
        }
        let time = round(load(e, root_a + 4));
        let d = [0u32, 4, 8].map(|i| load(e, d_at + i));
        let v = [0u32, 4, 8].map(|i| load(e, v_at + i));
        let axis_a = |i: usize| a + 0x48 + 0xc * i as u32;
        let axis_b = |j: usize| b + 0x48 + 0xc * j as u32;
        let extent_a = |i: usize| a + 0x6c + 4 * i as u32;
        let extent_b = |j: usize| b + 0x6c + 4 * j as u32;
        let c = axis_products(e, a, b);
        let mut first = FirstContact {
            time: 0.0,
            axis: 0,
            side: 0,
        };
        // This box's face axes, each against the first one's separations.
        let start = [0usize, 1, 2].map(|i| project(e, axis_a(i), d));
        let end = [0usize, 1, 2]
            .map(|i| round(x87_add(x87_mul(project(e, axis_a(i), v), time), start[i])));
        for i in [0usize, 1, 2] {
            let [p, q, r] =
                [0usize, 1, 2].map(|j| stored_abs(x87_mul(load(e, extent_b(j)), c[i][j])));
            let sum = round(x87_add(x87_add(p, q), r));
            let radius = round(x87_add(sum, load(e, extent_a(i)).abs()));
            if !first.take(i as u32 + 1, sweep(start[0], end[0], radius, time)) {
                return false;
            }
        }
        // The other box's face axes.
        for j in [0usize, 1, 2] {
            let s0 = project(e, axis_b(j), d);
            let s1 = round(x87_add(x87_mul(project(e, axis_b(j), v), time), s0));
            let [p, q, r] =
                [0usize, 1, 2].map(|i| stored_abs(x87_mul(load(e, extent_a(i)), c[i][j])));
            let sum = round(x87_add(x87_add(p, q), r));
            let radius = round(x87_add(sum, load(e, extent_b(j)).abs()));
            if !first.take(j as u32 + 4, sweep(s0, s1, radius, time)) {
                return false;
            }
        }
        // The edge pairs.
        for (axis, (terms, (p, cp), (q, cq))) in (7u32..).zip(edge_pairs(a, b, &c)) {
            let radius = edge_radius(e, terms);
            let s0 = round(x87_sub(x87_mul(start[p], cp), x87_mul(start[q], cq)));
            let s1 = round(x87_sub(x87_mul(end[p], cp), x87_mul(end[q], cq)));
            if !first.take(axis, sweep(s0, s1, radius, time)) {
                return false;
            }
        }
        let latest = x87_store32(first.time);
        e.mem.set_u32(out_time.addr(), latest);
        let velocity_a = e.mem.u32(a + 0x78) + 0x10;
        let side = first.side;
        let (slots, from, coefficients) = match first.axis {
            1..=3 => {
                // The other box's corner toward this box's face.
                let i = first.axis as usize - 1;
                let [x, y, z] =
                    [0usize, 1, 2].map(|j| signed_extent(e, c[i][j], -1, side, extent_b(j)));
                let slots = [0x54, 0x30, 0x410, 0x230, 0x47c, 0x20, 0x3c, 0x48];
                (slots, (b + 0x3c, root_b + 0x10, b + 0x48), [x, y, z])
            }
            4..=6 => {
                // This box's corner toward the other box's face.
                let j = first.axis as usize - 4;
                let [x, y, z] =
                    [0usize, 1, 2].map(|i| signed_extent(e, c[i][j], 1, side, extent_a(i)));
                let slots = [0x440, 0x2a8, 0x398, 0x2c0, 0x110, 0xf8, 0xe0, 0xc8];
                (slots, (a + 0x3c, velocity_a, a + 0x48), [x, y, z])
            }
            7..=15 => {
                let pair = first.axis as usize - 7;
                let (i, j) = (pair / 3, pair % 3);
                // The ends of the other edges toward the contact.
                let mut coef_a = [0.0f64; 3];
                for m in (0..3).filter(|&m| m != i) {
                    let n = 3 - i - m;
                    let polarity = -permutation_sign(i, m);
                    coef_a[m] = signed_extent(e, c[n][j], polarity, side, extent_a(m));
                }
                let mut coef_b = [0.0f64; 3];
                for k in (0..3).filter(|&k| k != j) {
                    let l = 3 - j - k;
                    let polarity = -permutation_sign(j, k);
                    coef_b[k] = signed_extent(e, c[i][l], polarity, side, extent_b(k));
                }
                let cij = c[i][j];
                let den = round(x87_sub(1.0, x87_mul(cij, cij)));
                let mut along = 0.0;
                if round(den.abs()) > operand(e, PARALLEL_010959F4) {
                    // D moved by the time along V (through 004a3760), into the D and V slots.
                    let moved = e
                        .call(POINT_SCALE, &args![frame + 0x94, latest, v_at])
                        .u32();
                    let shifted = [0u32, 4, 8]
                        .map(|k| x87_store32(x87_add(load(e, d_at + k), operand(e, moved + k))));
                    for (k, word) in (0u32..).zip(shifted) {
                        e.mem.set_u32(v_at + 4 * k, word);
                        e.mem.set_u32(d_at + 4 * k, word);
                    }
                    let (lo, hi) = match i {
                        0 => (1, 2),
                        1 => (0, 2),
                        _ => (0, 1),
                    };
                    let first_term = x87_mul(coef_a[hi], c[hi][j]);
                    let sum = x87_add(first_term, x87_mul(coef_a[lo], c[lo][j]));
                    e.mem.set_u64(wide, sum.to_bits());
                    let on_b = e.call(POINT_DOT, &args![axis_b(j), d_at]).f64();
                    let gap = round(x87_sub(x87_load64(e.mem.u64(wide)), on_b));
                    let scaled = x87_mul(gap, cij);
                    e.mem.set_u64(wide, scaled.to_bits());
                    let on_a = e.call(POINT_DOT, &args![axis_a(i), d_at]).f64();
                    let mut total = x87_add(on_a, x87_load64(e.mem.u64(wide)));
                    for k in (0..3).filter(|&k| k != j) {
                        total = x87_add(total, x87_mul(coef_b[k], c[i][k]));
                    }
                    along = round(x87_div(total, x87_m32(x87_store32(den))));
                }
                coef_a[i] = along;
                let slots = EDGE_SLOTS[pair];
                (slots, (a + 0x3c, velocity_a, a + 0x48), coef_a)
            }
            _ => return true,
        };
        contact_point(
            e,
            frame,
            from,
            coefficients,
            latest,
            slots,
            out_point.addr(),
        );
        true
    })
}

/// The frame slots of each edge pair's contact point (the four scaled points, then the four
/// sums), offsets from ESP after the exe's register pushes.
const EDGE_SLOTS: [[u32; 8]; 9] = [
    [0x128, 0x140, 0x158, 0x170, 0x470, 0x2f0, 0x3b0, 0x2d8],
    [0x188, 0x1a0, 0x1b8, 0x1d0, 0x428, 0x320, 0x3c8, 0x308],
    [0x1e8, 0x200, 0x218, 0xa4, 0x458, 0x350, 0x3e0, 0x338],
    [0x248, 0x260, 0x278, 0x290, 0xb0, 0x380, 0x3f8, 0x368],
    [0xbc, 0xec, 0x11c, 0x14c, 0x164, 0x134, 0x104, 0xd4],
    [0x17c, 0x1ac, 0x1dc, 0x20c, 0x224, 0x1f4, 0x1c4, 0x194],
    [0x23c, 0x26c, 0x29c, 0x2cc, 0x2e4, 0x2b4, 0x284, 0x254],
    [0x2fc, 0x32c, 0x35c, 0x38c, 0x3a4, 0x374, 0x344, 0x314],
    [0x3bc, 0x3ec, 0x41c, 0x44c, 0x464, 0x434, 0x404, 0x3d4],
];

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00eb_a480, ni_obb_box_transform(Ptr, Ptr)),
        entry!(
            0x00eb_a680,
            ni_obb_box_test_intersection(Ptr, Ptr) -> bool
        ),
        entry!(0x00eb_c2a0, fn_00ebc2a0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00eb_f4a0, fn_00ebf4a0(Ptr, Ptr, Ptr)),
        entry!(
            0x00eb_f610,
            ni_obb_box_test_intersection_ray(Ptr, Ptr, Ptr) -> bool
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

    /// An `NiOBBox` whose world box is `center`, the unit axes and `extents`.
    fn world_box(e: &mut Engine, center: [f32; 3], extents: [f32; 3]) -> u32 {
        let mut words = vec![0.0f32; 15];
        words.extend_from_slice(&center);
        words.extend_from_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        words.extend_from_slice(&extents);
        floats(e, &words)
    }

    #[test]
    fn transform_rotates_scales_and_moves_the_box() {
        let mut e = Engine::new();
        // Model box: center (1, 2, 3), the unit axes, extents (1, 2, 3).
        let mut model = vec![1.0f32, 2.0, 3.0];
        model.extend_from_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0]);
        model.extend_from_slice(&[0.0; 15]);
        let this = floats(&mut e, &model);
        // A quarter turn about z (x goes to y), translation (10, 20, 30), scale 2.
        let transform = floats(
            &mut e,
            &[
                0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0, 20.0, 30.0, 2.0,
            ],
        );
        e.call(0x00eb_a480, &args![this, transform]);
        assert_eq!(read(&e, this + 0x3c, 3), vec![6.0, 22.0, 36.0]);
        assert_eq!(
            read(&e, this + 0x48, 9),
            vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, this + 0x6c, 3), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn normal_follows_the_extents() {
        let mut e = Engine::new();
        e.map(0x0103_2000, 0x1000);
        e.mem.set_f64(FLAT_EXTENT_01032980, 0.0001);
        e.register(UNITIZE, |e, a| {
            let v: Vec<f64> = (0..3).map(|i| f64::from(e.mem.f32(a[0] + 4 * i))).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, c) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, (c / length) as f32);
            }
            Ret::default()
        });
        let out = e.mem.alloc(12);
        let point = floats(&mut e, &[1.0, 4.0, 0.0]);
        let off_axis = floats(&mut e, &[2.0, 0.0, 0.5]);
        let cube = world_box(&mut e, [1.0, 0.0, 0.0], [1.0, 2.0, 1.0]);
        e.call(0x00eb_f4a0, &args![cube, point, out]);
        assert_eq!(read(&e, out, 3), vec![0.0, 1.0, 0.0]);
        // A flat second extent answers its axis.
        let flat = world_box(&mut e, [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]);
        e.call(0x00eb_f4a0, &args![flat, off_axis, out]);
        assert_eq!(read(&e, out, 3), vec![0.0, 1.0, 0.0]);
        let flat = world_box(&mut e, [1.0, 0.0, 0.0], [0.0, 1.0, 1.0]);
        e.call(0x00eb_f4a0, &args![flat, off_axis, out]);
        assert_eq!(read(&e, out, 3), vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn ray_slab_test() {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x2000);
        e.map(0x0109_a000, 0x1000);
        e.mem.set_u32(RAY_START_01015F5C, 0xff7f_ffff);
        e.mem.set_f32(RAY_END_01016970, f32::MAX);
        e.mem.set_f64(PARALLEL_0109A3A0, 1e-7);
        let cube = world_box(&mut e, [5.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let origin = floats(&mut e, &[0.0, 0.0, 0.0]);
        let beside = floats(&mut e, &[0.0, 3.0, 0.0]);
        let inside = floats(&mut e, &[5.0, 0.5, 0.0]);
        let mut hits = |direction: [f32; 3], start: u32| {
            let d = floats(&mut e, &direction);
            e.call(0x00eb_f610, &args![cube, start, d]).bool()
        };
        assert!(hits([1.0, 0.0, 0.0], origin));
        assert!(hits([1.0, 0.15, 0.1], origin));
        // Behind, past the slab, and parallel outside a slab.
        assert!(!hits([-1.0, 0.0, 0.0], origin));
        assert!(!hits([1.0, 0.5, 0.0], origin));
        assert!(!hits([1.0, 0.0, 0.0], beside));
        assert!(hits([0.0, 0.0, 1.0], inside));
    }
}

#[cfg(test)]
mod intersection_tests {
    use super::*;

    /// An `NiOBBox` with the unit axes, `center` and `extents`, owned by an object at
    /// `position` whose motion lasts `time`.
    fn moving_box(e: &mut Engine, center: [f32; 3], extents: [f32; 3], position: [f32; 3]) -> u32 {
        let owner = e.mem.alloc(0x20);
        e.mem.set_f32(owner + 4, 1.0);
        for (i, value) in position.iter().enumerate() {
            e.mem.set_f32(owner + 0x10 + 4 * i as u32, *value);
        }
        let block = e.mem.alloc(0x7c);
        let mut words = center.to_vec();
        words.extend_from_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        words.extend_from_slice(&extents);
        for (i, value) in words.iter().enumerate() {
            e.mem.set_f32(block + 0x3c + 4 * i as u32, *value);
        }
        e.mem.set_u32(block + 0x78, owner);
        block
    }

    fn vector_engine() -> Engine {
        let mut e = Engine::new();
        e.register(POINT_DOT, |e, a| {
            let mut sum = 0.0f64;
            for i in 0..3 {
                sum += f64::from(e.mem.f32(a[0] + 4 * i)) * f64::from(e.mem.f32(a[1] + 4 * i));
            }
            sum.into_ret()
        });
        e.register(POINT_CROSS, |e, a| {
            let p = |e: &Engine, at: u32| [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)];
            let (l, r) = (p(e, a[0]), p(e, a[2]));
            let out = [
                l[1] * r[2] - l[2] * r[1],
                l[2] * r[0] - l[0] * r[2],
                l[0] * r[1] - l[1] * r[0],
            ];
            for (i, v) in out.into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, v);
            }
            a[1].into_ret()
        });
        e
    }

    #[test]
    fn separating_axes_now_and_after_the_motion() {
        let mut e = vector_engine();
        let unit = [1.0, 1.0, 1.0];
        let here = moving_box(&mut e, [0.0, 0.0, 0.0], unit, [0.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        // Overlapping boxes meet.
        let near = moving_box(&mut e, [1.5, 0.0, 0.0], unit, [0.0, 0.0, 0.0]);
        assert!(e.call(0x00eb_a680, &args![here, near]).bool());
        // Apart and staying apart along x.
        let far = moving_box(&mut e, [5.0, 0.0, 0.0], unit, [0.0, 0.0, 0.0]);
        assert!(!e.call(0x00eb_a680, &args![here, far]).bool());
        // Apart now but swept through this box (the motion offset -5 along x).
        let sweeping = moving_box(&mut e, [5.0, 0.0, 0.0], unit, [-5.0, 0.0, 0.0]);
        assert!(e.call(0x00eb_a680, &args![here, sweeping]).bool());
        // Swept along y past this box at x = 5: still apart along x.
        let passing = moving_box(&mut e, [5.0, 3.0, 0.0], unit, [0.0, -6.0, 0.0]);
        assert!(!e.call(0x00eb_a680, &args![here, passing]).bool());
        let log = e.call_log.take().unwrap();
        let crossings = log.iter().filter(|(at, _)| *at == POINT_CROSS).count();
        assert_eq!(crossings, 2);
    }
}

#[cfg(test)]
mod first_contact_tests {
    use super::*;

    /// An `NiOBBox` world box (center, axes, extents) owned by an object whose motion lasts 1
    /// at `velocity`.
    fn moving_box(e: &mut Engine, center: [f32; 3], axes: [f32; 9], velocity: [f32; 3]) -> u32 {
        let owner = e.mem.alloc(0x20);
        e.mem.set_f32(owner + 4, 1.0);
        for (i, value) in (0u32..).zip(velocity) {
            e.mem.set_f32(owner + 0x10 + 4 * i, value);
        }
        let block = e.mem.alloc(0x7c);
        let mut words = center.to_vec();
        words.extend_from_slice(&axes);
        words.extend_from_slice(&[1.0, 1.0, 1.0]);
        for (i, value) in (0u32..).zip(words) {
            e.mem.set_f32(block + 0x3c + 4 * i, value);
        }
        e.mem.set_u32(block + 0x78, owner);
        block
    }

    const UNIT_AXES: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

    fn vector_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_5000, 0x1000);
        e.mem.set_f32(PARALLEL_010959F4, 1e-6);
        e.register(POINT_DOT, |e, a| {
            let mut sum = 0.0f64;
            for i in 0..3 {
                sum += f64::from(e.mem.f32(a[0] + 4 * i)) * f64::from(e.mem.f32(a[1] + 4 * i));
            }
            sum.into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            let s = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i) * s;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_PLUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e
    }

    fn run(e: &mut Engine, this: u32, other: u32) -> (bool, f32, [f32; 3]) {
        let outs = e.mem.alloc(16);
        let hit = e
            .call(0x00eb_c2a0, &args![this, other, outs, outs + 4])
            .bool();
        let point = [0u32, 4, 8].map(|i| e.mem.f32(outs + 4 + i));
        (hit, e.mem.f32(outs), point)
    }

    #[test]
    fn faces_edges_and_separation() {
        let mut e = vector_engine();
        let still = [0.0, 0.0, 0.0];
        let here = moving_box(&mut e, [0.0; 3], UNIT_AXES, still);
        // Apart along x for the whole motion.
        let apart = moving_box(&mut e, [5.0, 0.0, 0.0], UNIT_AXES, still);
        assert!(!run(&mut e, here, apart).0);
        // Closing along x: this box's first face meets the other's corner side at 0.6.
        let closing = moving_box(&mut e, [5.0, 0.0, 0.0], UNIT_AXES, [-5.0, 0.0, 0.0]);
        let (hit, time, point) = run(&mut e, here, closing);
        assert!(hit);
        assert_eq!(time, 0.6);
        assert_eq!(point, [1.0, 0.0, 0.0]);
        // A box turned 45 degrees about x falling onto this one: its lowest edge meets this
        // box's top along the edge pair A1 x B0, solved through the dot products.
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let turned = [1.0, 0.0, 0.0, 0.0, h, h, 0.0, -h, h];
        let falling = moving_box(&mut e, [0.0, 0.0, 5.0], turned, [0.0, 0.0, -5.0]);
        e.call_log = Some(vec![]);
        let (hit, time, point) = run(&mut e, here, falling);
        let log = e.call_log.take().unwrap();
        assert!(hit);
        assert!((time - 0.517_157_3).abs() < 1e-6);
        assert_eq!(point, [0.0, 0.0, 1.0]);
        assert_eq!(log.iter().filter(|(at, _)| *at == POINT_DOT).count(), 2);
    }
}
