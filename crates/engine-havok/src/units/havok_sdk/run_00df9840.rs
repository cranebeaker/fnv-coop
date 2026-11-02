//! `Havok SDK/run_00df9840` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These are the functions of a GJK-style closest-point search between two
//! convex shapes; the executable has no names for them and the Xbox PDB has
//! no entry, so they are named by address and the object layout below comes
//! from this code only (unconfirmed names).
//!
//! The search object (`this`): +0x00, +0x10, +0x20, +0x30 the four column
//! vectors of the transform of the second shape relative to the first (+0x30
//! doubles as the first search direction in `00df9fe0`), +0x40 a tolerance
//! vector (its first float is used), +0x50 a float epsilon, +0x54 and +0x58
//! the two shapes, +0x60, +0x64 and +0x68 pointers to arrays of 16-byte
//! points (the point of the first shape, of the second, and their difference)
//! and +0x6c the number of points of the simplex.

use crate::x87::rsqrt_estimate;

#[allow(unused_imports)]
use crate::prelude::*;

type Vec4 = [f32; 4];

/// Float constants: 0.0, 3.0 and 0.5 (the `RSQRTSS` refinement) and four
/// sign-bit masks, four ones and an all-zero vector.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const SIGN_MASK: u32 = 0x010c_44d0;
const ONES: u32 = 0x010e_c7f0;
const ZERO_VECTOR: u32 = 0x0126_8380;
/// `double` constants: 2^-31 (the random-number scale) and 2^-23.
const RANDOM_SCALE: u32 = 0x0102_29b8;
const TINY: u32 = 0x010c_9300;
/// The origin passed to `00cd7530` (barycentric weights of the closest point
/// of a triangle).
const ORIGIN_POINT: u32 = 0x0126_7e30;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;

/// Callees outside this unit.
const TRIANGLE_WEIGHTS: u32 = 0x00cd_7530;
const SUBTRACT_WRAPPER: u32 = 0x004b_4ab0;
const EDGE_CHECK: u32 = 0x00df_b040;
const CLOSE_HOLE: u32 = 0x00e2_d1b0;
const CLIMB_FACES: u32 = 0x00e2_d550;

/// This unit's functions that call each other.
const SUPPORT_OF_DIFFERENCE: u32 = 0x00df_9b50;
const APPEND_POINT: u32 = 0x00df_9c40;
const BEST_OF_TWO: u32 = 0x00df_9ef0;

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// `first.yzx * second.zxy - first.zxy * second.yzx` (`SHUFPS` 0xc9 and
/// 0xd2, `MULPS`, `MULPS`, `SUBPS`), lane by lane.
fn cross(first: Vec4, second: Vec4) -> Vec4 {
    let plus = [
        first[1] * second[2],
        first[2] * second[0],
        first[0] * second[1],
        first[3] * second[3],
    ];
    let minus = [
        first[2] * second[1],
        first[0] * second[2],
        first[1] * second[0],
        first[3] * second[3],
    ];
    zip_with(plus, minus, |a, b| a - b)
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero
/// guard: `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == 0`.
fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// `vector` with every lane's sign bit flipped (`XORPS` with the sign mask).
fn negated(e: &Engine, vector: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        let mask = e.mem.u32(SIGN_MASK + 4 * lane as u32);
        f32::from_bits(vector[lane].to_bits() ^ mask)
    })
}

/// `FLD float; FCHS; FSTP float`: the negation of a value that went through
/// the x87 stack (a signalling NaN comes back quiet).
fn x87_negated(value: f32) -> f32 {
    f32::from_bits(x87_quiet_bits(value.to_bits()) ^ 0x8000_0000)
}

/// The two axes the perpendicular of `edge` is built from, picked by the
/// smallest absolute components: the result `(first, second)` is where the
/// edge's `second` component (kept) and `first` component (negated) go.
fn perpendicular_axes(edge: Vec4) -> (usize, usize) {
    let mut first = 1usize;
    let mut other = 0usize;
    let mut smallest = edge[0].abs();
    if smallest > edge[1].abs() {
        smallest = edge[1].abs();
        first = 0;
        other = 1;
    }
    let mut second = 2usize;
    if smallest > edge[2].abs() {
        second = other;
    }
    (first, second)
}

// Translated from 00df9ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A pseudo-random float from a seed in `EAX`: the 31-bit value `(0x3039 -
/// seed * 0x3e39b193) & 0x7fffffff` as a signed integer times 2^-31 (the
/// `double` at `010229b8`), rounded to a float on return in ST0.
pub fn fn_00df9ae0(e: &mut Engine, seed: u32) -> f32 {
    let bits = 0x3039u32.wrapping_sub(seed.wrapping_mul(0x3e39_b193)) & 0x7fff_ffff;
    let scale: f64 = e.global(RANDOM_SCALE);
    (bits as i32 as f64 * scale) as f32
}

// Translated from 00df9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Support point of the Minkowski difference along `vec`: the first shape's
/// vertex along `vec` (virtual call, slot +0x38) goes to `out + 0x10`; the
/// second shape's vertex along `-vec` turned into the first shape's frame by
/// the transposed rotation (columns at +0x00, +0x10, +0x20) goes to `out +
/// 0x20` (the vector the shape returned, before the transform); `out + 0x00`
/// gets the difference of the first vertex and the transformed second
/// one (`(y * c1 + x * c0) + (z * c2 + c3)` per lane).
pub fn fn_00df9b50(e: &mut Engine, this: Ptr, vec: Ptr, out: Ptr) {
    let t = this.addr();
    let shape_a = e.mem.u32(t + 0x54);
    e.vcall(shape_a, 0x38, &args![vec, out.addr() + 0x10]);
    let backwards = negated(e, load_vec(e, vec.addr()));
    let c0 = load_vec(e, t);
    let c1 = load_vec(e, t + 0x10);
    let c2 = load_vec(e, t + 0x20);
    let c3 = load_vec(e, t + 0x30);
    let row_x = [c0[0], c1[0], c2[0], c2[1]];
    let row_y = [c0[1], c1[1], c2[1], c2[3]];
    let row_z = [c0[2], c1[2], c2[2], c2[3]];
    let turned: Vec4 = std::array::from_fn(|lane| {
        (backwards[1] * row_y[lane] + backwards[0] * row_x[lane]) + backwards[2] * row_z[lane]
    });
    let shape_b = e.mem.u32(t + 0x58);
    e.with_stack(0x10, |e, tmp| {
        store_vec(e, tmp.addr(), turned);
        e.vcall(shape_b, 0x38, &args![tmp, out.addr() + 0x20]);
    });
    let b = load_vec(e, out.addr() + 0x20);
    let moved: Vec4 = std::array::from_fn(|lane| {
        (b[1] * c1[lane] + b[0] * c0[lane]) + (b[2] * c2[lane] + c3[lane])
    });
    let a = load_vec(e, out.addr() + 0x10);
    store_vec(e, out.addr(), zip_with(a, moved, |x, y| x - y));
}

// Translated from 00df9c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a point to the simplex: the three vectors at `vecs` (+0x00 the
/// difference, +0x10 the first shape's point, +0x20 the second's) go to
/// the arrays at +0x68, +0x60 and +0x64 at index `count`, and the count
/// (+0x6c) grows by one.
pub fn fn_00df9c40(e: &mut Engine, this: Ptr, vecs: Ptr) {
    let t = this.addr();
    let index = e.mem.u32(t + 0x6c);
    let offset = index << 4;
    let w = e.mem.u32(t + 0x68);
    copy_vec(e, vecs.addr(), w.wrapping_add(offset));
    let a = e.mem.u32(t + 0x60);
    copy_vec(e, vecs.addr() + 0x10, a.wrapping_add(offset));
    let b = e.mem.u32(t + 0x64);
    copy_vec(e, vecs.addr() + 0x20, b.wrapping_add(offset));
    e.mem.set_u32(t + 0x6c, index.wrapping_add(1));
}

// Translated from 00df9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Evaluates the support point along `a` and along `-a` (`00df9b50`, results
/// in a local record and in `c`), measures `(result.point - *b) . direction`
/// for both (`b` is a point of the simplex) and keeps the larger one: the
/// larger distance goes to `*d` (the second, with its record left in `c`,
/// when it is larger or unordered; otherwise the first, whose record, 0x34
/// bytes, is copied into `c`).
pub fn fn_00df9ef0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr, d: Ptr) {
    // The word at +0x30 of the local record is never written by the callee
    // (`00df9b50` writes only +0, +0x10 and +0x20), so the exe copies a stack
    // word it never initialised into `c + 0x30` (at `00df9fa3`/`00df9fba`);
    // zeros stand in for it. Nothing reads it: the only caller (`00df9fe0`, three call
    // sites) hands `c` on to `00df9c40`, which copies only the first three
    // vectors, and `00df9fe0` has no other access to those records' +0x30 words.
    e.with_stack(0x60, |e, frame| {
        let negative = frame.addr();
        let first = frame.addr() + 0x10;
        let backwards = negated(e, load_vec(e, a.addr()));
        store_vec(e, negative, backwards);
        e.call(SUPPORT_OF_DIFFERENCE, &args![this, a, first]);
        e.call(SUPPORT_OF_DIFFERENCE, &args![this, negative, c]);
        let reference = load_vec(e, b.addr());
        let direction = load_vec(e, a.addr());
        let first_result = load_vec(e, first);
        let first_distance = sum3(zip_with(
            zip_with(first_result, reference, |x, y| x - y),
            direction,
            |x, y| x * y,
        ));
        let second_result = load_vec(e, c.addr());
        let second_direction = load_vec(e, negative);
        let second_distance = sum3(zip_with(
            zip_with(second_result, reference, |x, y| x - y),
            second_direction,
            |x, y| x * y,
        ));
        if second_distance <= first_distance {
            for vec in 0..3 {
                copy_vec(e, first + 0x10 * vec, c.addr() + 0x10 * vec);
            }
            let word = e.mem.u32(first + 0x30);
            e.mem.set_u32(c.addr() + 0x30, word);
            e.mem.set_f32(d.addr(), x87_quiet(first_distance));
        } else {
            e.mem.set_f32(d.addr(), x87_quiet(second_distance));
        }
    });
}

// Translated from 00df9d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the closest triangle of the polytope `a` (the object of
/// `00e2d550`) to the origin, starting at the face `b`: up to 10 times, the
/// face's three vertex points (pointers at +0x10, +0x20, +0x30) are copied
/// into the difference array, the barycentric weights of the origin are
/// asked of `00cd7530`, and while one of them is negative (ordered) the
/// walk moves to the next face (`00e2d550`). Then the simplex is rebuilt
/// from the face's three vertices (the points of the first and second shape
/// at +0x10 and +0x20 of each vertex record), the count (+0x6c) is set to 3
/// and `c` receives: +0x00 the negated plane normal of the face, +0x10 the
/// closest point on the first shape (weights `w1 * A1 + w0 * A0 + w2 * A2`)
/// and +0x20 the negated plane distance (+0x40 of the face).
pub fn fn_00df9d90(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr) {
    let t = this.addr();
    let mut face = b.addr();
    e.with_stack(0x10, |e, weights| {
        let w = weights.addr();
        let difference = e.mem.u32(t + 0x68);
        let mut round = 0;
        loop {
            for k in 0..3u32 {
                let vertex = e.mem.u32(face + 0x10 * (k + 1));
                copy_vec(e, vertex, difference + 0x10 * k);
            }
            e.call(
                TRIANGLE_WEIGHTS,
                &args![
                    ORIGIN_POINT,
                    difference,
                    difference + 0x10,
                    difference + 0x20,
                    w
                ],
            );
            let s0 = e.mem.f32(w);
            let s1 = e.mem.f32(w + 4);
            let s2 = e.mem.f32(w + 8);
            if s0 < 0.0 || s1 < 0.0 || s2 < 0.0 {
                face = e.call(CLIMB_FACES, &args![a, face]).u32();
                round += 1;
                if round < 10 {
                    continue;
                }
            }
            break;
        }
        let first_points = e.mem.u32(t + 0x60);
        let second_points = e.mem.u32(t + 0x64);
        for k in 0..3u32 {
            let vertex = e.mem.u32(face + 0x10 * (k + 1));
            copy_vec(e, vertex + 0x10, first_points + 0x10 * k);
            copy_vec(e, vertex + 0x20, second_points + 0x10 * k);
        }
        e.mem.set_u32(t + 0x6c, 3);
        let w0 = e.mem.f32(w);
        let w1 = x87_quiet(e.mem.f32(w + 4));
        let w2 = x87_quiet(e.mem.f32(w + 8));
        let a0 = load_vec(e, first_points);
        let a1 = load_vec(e, first_points + 0x10);
        let a2 = load_vec(e, first_points + 0x20);
        let closest: Vec4 =
            std::array::from_fn(|lane| (w1 * a1[lane] + w0 * a0[lane]) + w2 * a2[lane]);
        store_vec(e, c.addr() + 0x10, closest);
        let distance = x87_quiet_bits(e.mem.u32(face + 0x40)) ^ 0x8000_0000;
        e.mem.set_u32(c.addr() + 0x20, distance);
        let normal = negated(e, load_vec(e, face));
        store_vec(e, c.addr(), normal);
    });
}

// Translated from 00df9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries to expand the polytope `a` with the point `*c`: the vectors from
/// the three vertices of the face `b` (pointers at +0x10, +0x20, +0x30) to
/// the point are computed and handed, with the face, to `004b4ab0`; if any of
/// the first three lanes of the result is below the tolerance vector at
/// +0x40, `*d` is 0. Otherwise `00dfb040` is asked (it fills a count `u` and
/// an edge list `v`): a nonzero answer gives `*d = 3`; if the face-stack room
/// of `a` (`a[0xc] - a[0x10]`, plus 100) is below `u` the answer is `*d =
/// 2`; and `00e2d1b0` closing the hole (count `u`, edge list `v`, point `c`)
/// answers `*d = 3` when it fails. Returns 1 when `*d` was set, else 0.
pub fn fn_00df9c90(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr, d: Ptr) -> u32 {
    let t = this.addr();
    e.with_stack(0x50, |e, frame| {
        let result = frame.addr();
        let first = frame.addr() + 0x10;
        let count_word = frame.addr() + 0x40;
        let edge_word = frame.addr() + 0x44;
        let point = load_vec(e, c.addr());
        for k in 0..3u32 {
            let vertex = e.mem.u32(b.addr() + 0x10 * (k + 1));
            let difference = zip_with(point, load_vec(e, vertex), |x, y| x - y);
            store_vec(e, first + 0x10 * k, difference);
        }
        e.call(SUBTRACT_WRAPPER, &args![result, first, b]);
        let found = load_vec(e, result);
        let tolerance = load_vec(e, t + 0x40);
        if found[0] < tolerance[0] || found[1] < tolerance[1] || found[2] < tolerance[2] {
            e.mem.set_u32(d.addr(), 0);
            return 1;
        }
        let checked = e
            .call(EDGE_CHECK, &args![a, b, c, count_word, edge_word])
            .u32();
        if checked != 0 {
            e.mem.set_u32(d.addr(), 3);
            return 1;
        }
        let count = e.mem.i32(count_word);
        let room = e
            .mem
            .i32(a.addr() + 0xc)
            .wrapping_sub(e.mem.i32(a.addr() + 0x10));
        if room.wrapping_add(0x64) < count {
            e.mem.set_u32(d.addr(), 2);
            return 1;
        }
        let edges = e.mem.u32(edge_word);
        let closed = e.call(CLOSE_HOLE, &args![a, count, edges, c]).u32();
        if closed != 0 {
            e.mem.set_u32(d.addr(), 3);
            return 1;
        }
        0
    })
}

/// The steps of the simplex state machine of `00df9fe0`, named by the
/// number of points they start from.
#[derive(Clone, Copy)]
enum Step {
    /// No points.
    Origin,
    /// One point.
    Point,
    /// One point, long enough to search around.
    PointPick,
    /// Two points.
    Segment,
    /// Two points, long enough to search around.
    SegmentLong,
    /// Three points.
    Triangle,
    /// Three points with a normal.
    TriangleSolid,
    /// Three points, the closest point found.
    TriangleFinal,
    /// Three collinear points.
    TriangleDegenerate,
    /// Four points.
    Tetrahedron,
}

// Translated from 00df9fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// One round of the simplex solver (`this` is the search object, `out`
/// receives +0x00 the direction, +0x10 the closest point of the first shape,
/// +0x20 the distance, `status` receives 0 or 3). Works on the number of
/// points in the simplex (+0x6c), 0 to 4 (more: status 3 and true). A stage
/// counter starts at -1 and records the largest simplex size reached this
/// round:
///
/// - four points: the tetrahedron is rejected (status 0, returns false) when
///   `eps * |n|^2 < s^2` for the normal `n = (v0 - v1) x (v1 - v2)` and `s =
///   (v3 - v0) . n`; otherwise status 3 (true) when the stage counter
///   already reached 3, else the simplex drops to the triangle;
/// - three points: a zero normal falls back to the segment (to the degenerate
///   triangle when the stage counter is 2 or more), else the unit normal is
///   used as search direction (`00df9ef0`): if the best distance is above the
///   tolerance at +0x40, the closest point is formed from the barycentric
///   weights (`00cd7530`, status 0, true), else the new point is appended and
///   the tetrahedron step runs;
/// - the degenerate triangle returns a direction perpendicular to the
///   segment `v0 - v1`, normalised, and the point `(1 - t) * A0 + A1 * t`
///   with `t = s / (2 s + 2^-23)` for `s = |v0|` in the 1-norm;
/// - two points: a segment longer than `eps` (squared) is searched along
///   the perpendiculars to it (`00df9ef0` twice) and the better new point
///   appended; a short one falls back to the point (status 3 when the stage
///   counter is at least 1);
/// - one point: longer than `eps` (squared), the support points along `v0`
///   and `-v0` are compared and the farther one appended; a short one falls
///   back to no points (status 3 when the stage counter is at least 0);
/// - no points: the direction at +0x30 (or the vector at `01268380` if
///   too short) gives the first support point, appended; the origin is
///   reached (distance `sqrt(|v0|^2)`, status 0, true) when `|v0|^2 <= eps`.
pub fn fn_00df9fe0(e: &mut Engine, this: Ptr, out: Ptr, status: Ptr) -> bool {
    let t = this.addr();
    let count = e.mem.u32(t + 0x6c);
    let mut step = match count {
        0 => Step::Origin,
        1 => Step::Point,
        2 => Step::Segment,
        3 => Step::Triangle,
        4 => Step::Tetrahedron,
        _ => {
            e.mem.set_u32(status.addr(), 3);
            return true;
        }
    };
    e.with_stack(0x140, |e, frame| {
        let base = frame.addr();
        let mut stage = -1i32;
        let epsilon = e.mem.f32(t + 0x50);
        loop {
            let points = e.mem.u32(t + 0x68);
            match step {
                Step::Tetrahedron => {
                    let v0 = load_vec(e, points);
                    let v1 = load_vec(e, points + 0x10);
                    let v2 = load_vec(e, points + 0x20);
                    let v3 = load_vec(e, points + 0x30);
                    let normal = cross(
                        zip_with(v0, v1, |x, y| x - y),
                        zip_with(v1, v2, |x, y| x - y),
                    );
                    store_vec(e, base + 0x70, normal);
                    let norm_squared = sum3(zip_with(normal, normal, |x, y| x * y));
                    let side = sum3(zip_with(zip_with(v3, v0, |x, y| x - y), normal, |x, y| {
                        x * y
                    }));
                    let bound = (epsilon as f64 * norm_squared as f64) as f32;
                    let accepted = bound as f64 >= side as f64 * side as f64;
                    if !accepted {
                        e.mem.set_u32(status.addr(), 0);
                        return false;
                    }
                    if stage >= 3 {
                        e.mem.set_u32(status.addr(), 3);
                        return true;
                    }
                    e.mem.set_u32(t + 0x6c, 3);
                    step = Step::Triangle;
                }
                Step::Triangle => {
                    let v0 = load_vec(e, points);
                    let v1 = load_vec(e, points + 0x10);
                    let v2 = load_vec(e, points + 0x20);
                    let normal = cross(
                        zip_with(v0, v1, |x, y| x - y),
                        zip_with(v1, v2, |x, y| x - y),
                    );
                    let norm_squared = sum3(zip_with(normal, normal, |x, y| x * y));
                    if norm_squared == 0.0 {
                        if stage >= 2 {
                            step = Step::TriangleDegenerate;
                        } else {
                            e.mem.set_u32(t + 0x6c, 2);
                            step = Step::Segment;
                        }
                    } else {
                        step = Step::TriangleSolid;
                    }
                }
                Step::TriangleSolid => {
                    let v0 = load_vec(e, points);
                    let v1 = load_vec(e, points + 0x10);
                    let v2 = load_vec(e, points + 0x20);
                    let normal = cross(
                        zip_with(v0, v1, |x, y| x - y),
                        zip_with(v1, v2, |x, y| x - y),
                    );
                    let norm_squared = sum3(zip_with(normal, normal, |x, y| x * y));
                    let scale = inverse_length(e, norm_squared);
                    let direction: Vec4 = normal.map(|lane| scale * lane);
                    store_vec(e, base + 0x70, direction);
                    stage = 3;
                    e.call(
                        BEST_OF_TWO,
                        &args![this, base + 0x70, points, base + 0x90, base + 0xc],
                    );
                    let best = e.mem.f32(base + 0xc);
                    let tolerance = e.mem.f32(t + 0x40);
                    if tolerance > best {
                        step = Step::TriangleFinal;
                    } else {
                        e.call(APPEND_POINT, &args![this, base + 0x90]);
                        step = Step::Tetrahedron;
                    }
                }
                Step::TriangleFinal => {
                    e.mem.set_f32(out.addr() + 0x20, 0.0);
                    copy_vec(e, base + 0x70, out.addr());
                    let w = e.mem.u32(t + 0x68);
                    e.call(
                        TRIANGLE_WEIGHTS,
                        &args![ORIGIN_POINT, w, w + 0x10, w + 0x20, base + 0x30],
                    );
                    let w0 = e.mem.f32(base + 0x30);
                    let w1 = x87_quiet(e.mem.f32(base + 0x34));
                    let w2 = x87_quiet(e.mem.f32(base + 0x38));
                    let first_points = e.mem.u32(t + 0x60);
                    let a0 = load_vec(e, first_points);
                    let a1 = load_vec(e, first_points + 0x10);
                    let a2 = load_vec(e, first_points + 0x20);
                    let closest: Vec4 =
                        std::array::from_fn(|lane| (w1 * a1[lane] + w0 * a0[lane]) + w2 * a2[lane]);
                    store_vec(e, out.addr() + 0x10, closest);
                    e.mem.set_u32(status.addr(), 0);
                    return true;
                }
                Step::TriangleDegenerate => {
                    e.mem.set_f32(out.addr() + 0x20, 0.0);
                    let v0 = load_vec(e, points);
                    let v1 = load_vec(e, points + 0x10);
                    let edge = zip_with(v0, v1, |x, y| x - y);
                    let (first, second) = perpendicular_axes(edge);
                    let mut perpendicular = [0.0f32; 4];
                    perpendicular[first] = x87_quiet(edge[second]);
                    perpendicular[second] = x87_negated(edge[first]);
                    let sum = sum3(zip_with(perpendicular, perpendicular, |x, y| x * y));
                    let scale = inverse_length(e, sum);
                    let unit: Vec4 = perpendicular.map(|lane| scale * lane);
                    store_vec(e, out.addr(), unit);
                    let norm1 = v0[2].abs() + (v0[1].abs() + v0[0].abs());
                    let tiny: f64 = e.global(TINY);
                    let t_value = (norm1 as f64 / (norm1 as f64 + norm1 as f64 + tiny)) as f32;
                    let ones = load_vec(e, ONES);
                    let first_points = e.mem.u32(t + 0x60);
                    let a0 = load_vec(e, first_points);
                    let a1 = load_vec(e, first_points + 0x10);
                    let closest: Vec4 = std::array::from_fn(|lane| {
                        (ones[lane] - t_value) * a0[lane] + a1[lane] * t_value
                    });
                    store_vec(e, out.addr() + 0x10, closest);
                    e.mem.set_u32(status.addr(), 0);
                    return true;
                }
                Step::Segment => {
                    let v0 = load_vec(e, points);
                    let v1 = load_vec(e, points + 0x10);
                    let edge = zip_with(v0, v1, |x, y| x - y);
                    let length_squared = sum3(zip_with(edge, edge, |x, y| x * y));
                    if epsilon >= length_squared {
                        if stage >= 1 {
                            e.mem.set_u32(status.addr(), 3);
                            return true;
                        }
                        e.mem.set_u32(t + 0x6c, 1);
                        step = Step::Point;
                    } else {
                        step = Step::SegmentLong;
                    }
                }
                Step::SegmentLong => {
                    let v0 = load_vec(e, points);
                    let v1 = load_vec(e, points + 0x10);
                    let edge = zip_with(v0, v1, |x, y| x - y);
                    stage = 2;
                    let (first, second) = perpendicular_axes(edge);
                    let mut perpendicular = [0.0f32; 4];
                    perpendicular[first] = x87_quiet(edge[second]);
                    perpendicular[second] = x87_negated(edge[first]);
                    let crossed = cross(edge, perpendicular);
                    let perpendicular_sum =
                        sum3(zip_with(perpendicular, perpendicular, |x, y| x * y));
                    let perpendicular_scale = inverse_length(e, perpendicular_sum);
                    let unit_perpendicular: Vec4 =
                        perpendicular.map(|lane| perpendicular_scale * lane);
                    store_vec(e, base + 0x20, unit_perpendicular);
                    let crossed_sum = sum3(zip_with(crossed, crossed, |x, y| x * y));
                    let crossed_scale = inverse_length(e, crossed_sum);
                    let unit_crossed: Vec4 = crossed.map(|lane| crossed_scale * lane);
                    store_vec(e, base + 0xf0, unit_crossed);
                    e.call(
                        BEST_OF_TWO,
                        &args![this, base + 0x20, points, base + 0x90, base + 0x60],
                    );
                    let points_again = e.mem.u32(t + 0x68);
                    e.call(
                        BEST_OF_TWO,
                        &args![this, base + 0xf0, points_again, base + 0x100, base + 0x14],
                    );
                    let second_best = e.mem.f32(base + 0x14);
                    let first_best = e.mem.f32(base + 0x60);
                    if second_best < first_best {
                        e.call(APPEND_POINT, &args![this, base + 0x90]);
                    } else {
                        e.call(APPEND_POINT, &args![this, base + 0x100]);
                    }
                    step = Step::Triangle;
                }
                Step::Point => {
                    let v0 = load_vec(e, points);
                    let length_squared = sum3(zip_with(v0, v0, |x, y| x * y));
                    if epsilon >= length_squared {
                        if stage >= 0 {
                            e.mem.set_u32(status.addr(), 3);
                            return true;
                        }
                        e.mem.set_u32(t + 0x6c, 0);
                        step = Step::Origin;
                    } else {
                        step = Step::PointPick;
                    }
                }
                Step::PointPick => {
                    stage = 1;
                    e.call(SUPPORT_OF_DIFFERENCE, &args![this, points, base + 0x100]);
                    let v0 = load_vec(e, e.mem.u32(t + 0x68));
                    let first_w = load_vec(e, base + 0x100);
                    let first_difference = zip_with(first_w, v0, |x, y| x - y);
                    let first_distance =
                        sum3(zip_with(first_difference, first_difference, |x, y| x * y));
                    let backwards = negated(e, load_vec(e, e.mem.u32(t + 0x68)));
                    store_vec(e, base + 0xe0, backwards);
                    e.call(
                        SUPPORT_OF_DIFFERENCE,
                        &args![this, base + 0xe0, base + 0x90],
                    );
                    let v0_again = load_vec(e, e.mem.u32(t + 0x68));
                    let second_w = load_vec(e, base + 0x90);
                    let second_difference = zip_with(second_w, v0_again, |x, y| x - y);
                    let second_distance =
                        sum3(zip_with(second_difference, second_difference, |x, y| x * y));
                    if second_distance <= first_distance {
                        e.call(APPEND_POINT, &args![this, base + 0x100]);
                    } else {
                        e.call(APPEND_POINT, &args![this, base + 0x90]);
                    }
                    step = Step::Segment;
                }
                Step::Origin => {
                    let direction = load_vec(e, t + 0x30);
                    let length_squared = sum3(zip_with(direction, direction, |x, y| x * y));
                    let backwards = negated(e, direction);
                    store_vec(e, base + 0x80, backwards);
                    if epsilon >= length_squared {
                        copy_vec(e, ZERO_VECTOR, base + 0x80);
                    }
                    e.call(
                        SUPPORT_OF_DIFFERENCE,
                        &args![this, base + 0x80, base + 0x90],
                    );
                    e.call(APPEND_POINT, &args![this, base + 0x90]);
                    let v0 = load_vec(e, e.mem.u32(t + 0x68));
                    let origin_distance = sum3(zip_with(v0, v0, |x, y| x * y));
                    if epsilon >= origin_distance {
                        let root = e.call(X87_SQRT, &args![origin_distance as f64]).f64() as f32;
                        e.mem.set_f32(out.addr() + 0x20, x87_quiet(root));
                        copy_vec(e, base + 0x80, out.addr());
                        copy_vec(e, base + 0xa0, out.addr() + 0x10);
                        e.mem.set_u32(status.addr(), 0);
                        return true;
                    }
                    stage = 0;
                    step = Step::Point;
                }
            }
        }
    })
}

/// `InterlockedExchangeAdd` import slot (stdcall: the counter, the addend; answers the old value).
const INTERLOCKED_EXCHANGE_ADD: u32 = 0x00fd_f1ec;
/// Releases a semaphore (`00d043c0`, cdecl `(semaphore, count)`).
const SEMAPHORE_RELEASE: u32 = 0x00d0_43c0;

// Translated from 00df9840 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall, no stack words: adds 1 to the counter
/// at `*(this + 0x18)` with `InterlockedExchangeAdd` when that pointer is set, then releases the
/// semaphore at `*(this + 0x10)` once (`00d043c0`, `(semaphore, 1)`) when that pointer is set.
/// Returns `EAX` as the exe leaves it: the last call's result, else the counter pointer (0).
pub fn fn_00df9840(e: &mut Engine, this: Ptr) -> u32 {
    let mut eax = e.mem.u32(this.addr().wrapping_add(0x18));
    if eax != 0 {
        eax = e.call(INTERLOCKED_EXCHANGE_ADD, &args![eax, 1u32]).u32();
    }
    let semaphore = e.mem.u32(this.addr().wrapping_add(0x10));
    if semaphore != 0 {
        eax = e.call(SEMAPHORE_RELEASE, &args![semaphore, 1u32]).u32();
    }
    eax
}

// Translated from 00df9880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: four stack words (cdecl, nothing popped), the first two never read
/// (`_unused_0`, `_unused_1`): `source` and `target`. Copies the 0x80 bytes at `source` to
/// `target` (eight blocks of four words). Then, by the byte at `source + 0`: for 0 or 1, when the
/// signed count at `source + 0x28` is above the one at `source + 0x20`, stores the latter at
/// `target + 0x28`, takes it off the count at `source + 0x28`, advances the pointer at
/// `source + 0x24` by that many entries (0x50 bytes for kind 0, 0x40 bytes for kind 1, which also
/// clears the word at `source + 0x30`) and answers 1; every other case answers 0. Ghidra made no function here; the address is an immediate operand at 00da348e.
pub fn fn_00df9880(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    source: u32,
    target: u32,
) -> u32 {
    for block in 0..8u32 {
        for word in 0..4u32 {
            let value = e.mem.u32(source + 16 * block + 4 * word);
            e.mem.set_u32(target + 16 * block + 4 * word, value);
        }
    }
    let kind = e.mem.u8(source);
    if kind != 0 && kind != 1 {
        return 0;
    }
    let step = e.mem.u32(source + 0x20);
    if (e.mem.u32(source + 0x28) as i32) <= (step as i32) {
        return 0;
    }
    e.mem.set_u32(target + 0x28, step);
    let remaining = e.mem.u32(source + 0x28).wrapping_sub(step);
    e.mem.set_u32(source + 0x28, remaining);
    if kind == 1 {
        let advanced = e.mem.u32(source + 0x24).wrapping_add(step << 6);
        e.mem.set_u32(source + 0x24, advanced);
        e.mem.set_u32(source + 0x30, 0);
    } else {
        let advanced = e
            .mem
            .u32(source + 0x24)
            .wrapping_add(step.wrapping_mul(5) << 4);
        e.mem.set_u32(source + 0x24, advanced);
    }
    1
}

/// The job finisher `00df9840` and the result copy `00df7d00` (`cdecl`: job, result block).
const FINISH_JOB_00DF9840: u32 = 0x00df_9840;
const COPY_RESULT_00DF7D00: u32 = 0x00df_7d00;

/// The jump table of `00df9920` for job kinds 0 to 7 (`00df9aa4`, read from the exe).
const TARGETS_00DF9AA4: [u32; 8] = [
    0x00df_993c,
    0x00df_993c,
    0x00df_9941,
    0x00df_99b1,
    0x00df_993c,
    0x00df_9952,
    0x00df_99c8,
    0x00df_99d0,
];

/// The first bytes of the result block header `00df9920` writes (at `out + 0x10`): `first`, 3,
/// 2 and the 16-bit 0x50.
fn result_header(e: &mut Engine, block: u32, first: u8) {
    e.mem.set_u8(block, first);
    e.mem.set_u8(block.wrapping_add(1), 3);
    e.mem.set_u8(block.wrapping_add(2), 2);
    e.mem.set_u16(block.wrapping_add(4), 0x50);
}

// Translated from 00df9920 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00df9920 to the `RET` at 00df9aa1, its jump table after it):
/// the job-finished function `00da3400` registers for job type 3, `cdecl` with four words (the
/// third the job, the fourth the output; the first two not read). By the job's kind (byte `+0`,
/// the jump table at `00df9aa4`): 0, 1 and 4 drop the counter at `*(job + 0x14)` and at 0 finish
/// the job ([`fn_00df9840`]); 2 finishes it; 3 finishes it while `job + 0x38` is below 8
/// (signed); 6 finishes it when the counter is 0; 5 drops the counter (when the pointer is set)
/// and finishes the job when `job + 0x60` is 0, else with the counter at 0 writes a kind-3
/// result header at `out + 0x10`, clears the output word and copies the job's result
/// (`00df7d00`), answering 0; 7 writes a kind-1 result header from the job's words (+0x44 ..
/// +0x6c; the block count `(n - 1) / 64 + 1` (1 for 0) stored at `*(job + 0x58)`), sets the output
/// word to 1 and its byte +0x12 to 1, answering 0. Other kinds, and the paths above that do not
/// say otherwise, answer 1.
pub fn fn_00df9920(e: &mut Engine, _unused_1: u32, _unused_2: u32, job: Ptr, out: Ptr) -> u32 {
    let job = job.addr();
    let out = out.addr();
    let kind = u32::from(e.mem.u8(job));
    let counter = e.mem.u32(job.wrapping_add(0x14));
    if kind > 7 {
        return 1;
    }
    match TARGETS_00DF9AA4[kind as usize] {
        0x00df_993c => {
            let left = e.mem.u32(counter).wrapping_sub(1);
            e.mem.set_u32(counter, left);
            if left == 0 {
                e.call(FINISH_JOB_00DF9840, &args![job]);
            }
            1
        }
        0x00df_9941 => {
            e.call(FINISH_JOB_00DF9840, &args![job]);
            1
        }
        0x00df_99b1 => {
            if (e.mem.u32(job.wrapping_add(0x38)) as i32) < 8 {
                e.call(FINISH_JOB_00DF9840, &args![job]);
            }
            1
        }
        0x00df_99c8 => {
            if e.mem.u32(counter) == 0 {
                e.call(FINISH_JOB_00DF9840, &args![job]);
            }
            1
        }
        0x00df_9952 => {
            if counter != 0 {
                let left = e.mem.u32(counter).wrapping_sub(1);
                e.mem.set_u32(counter, left);
            }
            if e.mem.u32(job.wrapping_add(0x60)) == 0 {
                e.call(FINISH_JOB_00DF9840, &args![job]);
                return 1;
            }
            // 00df995f reads the counter even when its pointer is null, as the exe does.
            if e.mem.u32(counter) != 0 {
                return 1;
            }
            let block = out.wrapping_add(0x10);
            if block != 0 {
                let word = e.mem.u32(job.wrapping_add(0x14));
                result_header(e, block, 3);
                e.mem.set_u16(block.wrapping_add(6), 0xffff);
                e.mem.set_u32(block.wrapping_add(0x10), 0);
                e.mem.set_u32(block.wrapping_add(0x18), 0);
                e.mem.set_u8(block.wrapping_add(2), 1);
                e.mem.set_u32(block.wrapping_add(0x30), 0);
                e.mem.set_u32(block.wrapping_add(0x14), word);
            }
            e.mem.set_u32(out, 0);
            e.call(COPY_RESULT_00DF7D00, &args![job, block]);
            0
        }
        // 00df99d0: kind 7.
        _ => {
            let block = out.wrapping_add(0x10);
            if block != 0 {
                let word_60 = e.mem.u32(job.wrapping_add(0x60));
                let word_44 = e.mem.u32(job.wrapping_add(0x44));
                let word_5c = e.mem.u32(job.wrapping_add(0x5c));
                let word_54 = e.mem.u32(job.wrapping_add(0x54));
                let total = e.mem.u32(job.wrapping_add(0x48));
                let target = e.mem.u32(job.wrapping_add(0x58));
                result_header(e, block, 1);
                e.mem.set_u32(block.wrapping_add(0x10), word_5c);
                e.mem.set_u16(block.wrapping_add(6), 0xffff);
                e.mem.set_u32(block.wrapping_add(0x24), word_44);
                e.mem.set_u32(block.wrapping_add(0x1c), word_54);
                e.mem.set_u32(block.wrapping_add(0x18), 0);
                e.mem.set_u8(block.wrapping_add(2), 1);
                e.mem.set_u32(block.wrapping_add(0x14), target);
                e.mem.set_u32(block.wrapping_add(0x20), 0x40);
                e.mem.set_u32(block.wrapping_add(0x28), total);
                e.mem.set_u32(block.wrapping_add(0x34), word_60);
                // DEC; CDQ; AND EDX, 0x3f; ADD; SAR 6: a signed division by 64 toward zero.
                let blocks = if total == 0 {
                    1
                } else {
                    ((total.wrapping_sub(1) as i32) / 64).wrapping_add(1) as u32
                };
                e.mem.set_u32(target, blocks);
                for offset in [0x38u32, 0x3c, 0x40, 0x2c, 0x30] {
                    e.mem.set_u32(block.wrapping_add(offset), 0);
                }
            }
            for (to, from) in [
                (0x38u32, 0x64u32),
                (0x3c, 0x68),
                (0x40, 0x6c),
                (0x2c, 0x4c),
                (0x30, 0x50),
            ] {
                let word = e.mem.u32(job.wrapping_add(from));
                e.mem.set_u32(block.wrapping_add(to), word);
            }
            e.mem.set_u32(out, 1);
            e.mem.set_u8(out.wrapping_add(0x12), 1);
            0
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_9920, fn_00df9920(u32, u32, Ptr, Ptr) -> u32),
        entry!(0x00df9ae0, fn_00df9ae0(u32) -> f32),
        entry!(0x00df9b50, fn_00df9b50(Ptr, Ptr, Ptr)),
        entry!(0x00df9c40, fn_00df9c40(Ptr, Ptr)),
        entry!(0x00df9c90, fn_00df9c90(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00df9d90, fn_00df9d90(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00df9ef0, fn_00df9ef0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00df9840, fn_00df9840(Ptr) -> u32),
        entry!(0x00df9fe0, fn_00df9fe0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00df_9880, fn_00df9880(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_f32(ONES + 4 * lane, 1.0);
        }
        e.set_global(RANDOM_SCALE, 1.0f64 / 2147483648.0);
        e.set_global(TINY, 1.0f64 / 8388608.0);
        e
    }

    fn assert_close(actual: &[f32], expected: &[f32]) {
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-4, "{actual:?} vs {expected:?}");
        }
    }

    /// A search object with room for the three point arrays (16 points
    /// each), the epsilon and the tolerance.
    fn search(e: &mut Engine, count: u32) -> u32 {
        let this = e.mem.alloc(0x80);
        let first = e.mem.alloc(0x100);
        let second = e.mem.alloc(0x100);
        let difference = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x60, first);
        e.mem.set_u32(this + 0x64, second);
        e.mem.set_u32(this + 0x68, difference);
        e.mem.set_u32(this + 0x6c, count);
        e.mem.set_f32(this + 0x50, 0.1);
        e.mem.set_f32(this + 0x40, 0.5);
        this
    }

    #[test]
    fn the_random_float_follows_the_seed() {
        let mut e = engine();
        let got = e.call(0x00df_9ae0, &args![1u32]).f32();
        let bits = 0x3039u32.wrapping_sub(0x3e39_b193) & 0x7fff_ffff;
        assert_eq!(got, (bits as i32 as f64 / 2147483648.0) as f32);
        let zero_seed = e.call(0x00df_9ae0, &args![0u32]).f32();
        assert_eq!(zero_seed, (0x3039 as f64 / 2147483648.0) as f32);
    }

    /// Two shapes whose support vertex is written by a virtual call.
    fn shapes(e: &mut Engine, this: u32, support_a: Vec4, support_b: Vec4) {
        let table_a = e.mem.alloc(0x40);
        let table_b = e.mem.alloc(0x40);
        let shape_a = e.mem.alloc(8);
        let shape_b = e.mem.alloc(8);
        e.mem.set_u32(shape_a, table_a);
        e.mem.set_u32(shape_b, table_b);
        e.mem.set_u32(this + 0x54, shape_a);
        e.mem.set_u32(this + 0x58, shape_b);
        let slot_a = 0x00d0_0000;
        let slot_b = 0x00d0_0010;
        e.mem.set_u32(table_a + 0x38, slot_a);
        e.mem.set_u32(table_b + 0x38, slot_b);
        e.register_double(slot_a, move |e, a| {
            store_vec(e, a[2], support_a);
            Ret::default()
        });
        e.register_double(slot_b, move |e, a| {
            store_vec(e, a[2], support_b);
            Ret::default()
        });
    }

    #[test]
    fn the_support_point_of_the_difference_uses_both_shapes() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        // The identity rotation with the translation (10, 0, 0).
        store_vec(&mut e, this, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, this + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, this + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, this + 0x30, [10.0, 0.0, 0.0, 1.0]);
        shapes(&mut e, this, [1.0, 2.0, 3.0, 0.0], [4.0, 5.0, 6.0, 0.0]);
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x30);
        e.call(0x00df_9b50, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out + 0x10), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [4.0, 5.0, 6.0, 0.0]);
        // (1,2,3) - ((4,5,6) + (10,0,0)).
        assert_eq!(&load_vec(&e, out)[..3], &[-13.0, -3.0, -3.0]);
    }

    #[test]
    fn appending_a_point_fills_the_three_arrays() {
        let mut e = engine();
        let this = search(&mut e, 1);
        let vecs = e.mem.alloc(0x30);
        store_vec(&mut e, vecs, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, vecs + 0x10, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, vecs + 0x20, [9.0, 10.0, 11.0, 12.0]);
        e.call(APPEND_POINT, &args![this, vecs]);
        assert_eq!(e.mem.u32(this + 0x6c), 2);
        let w = e.mem.u32(this + 0x68);
        let a = e.mem.u32(this + 0x60);
        let b = e.mem.u32(this + 0x64);
        assert_eq!(load_vec(&e, w + 0x10), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, a + 0x10), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vec(&e, b + 0x10), [9.0, 10.0, 11.0, 12.0]);
    }

    #[test]
    fn the_better_of_two_directions_is_kept() {
        let mut e = engine();
        let this = search(&mut e, 0);
        // The difference vector the double writes is the direction itself.
        e.register_double(SUPPORT_OF_DIFFERENCE, |e, a| {
            let direction = load_vec(e, a[1]);
            store_vec(e, a[2], direction);
            Ret::default()
        });
        let reference = e.mem.alloc(16);
        store_vec(&mut e, reference, [0.0; 4]);
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [0.0, 0.0, 2.0, 0.0]);
        let record = e.mem.alloc(0x40);
        let best = e.mem.alloc(4);
        e.call(
            BEST_OF_TWO,
            &args![this, direction, reference, record, best],
        );
        // Both distances are 4: equal keeps the first record (copied into `c`).
        assert_eq!(e.mem.f32(best), 4.0);
        assert_eq!(&load_vec(&e, record)[..3], &[0.0, 0.0, 2.0]);
        // First: (2+1)*2 = 6; second: (-2+1)*-2 = 2; the first wins again.
        store_vec(&mut e, reference, [0.0, 0.0, -1.0, 0.0]);
        e.call(
            BEST_OF_TWO,
            &args![this, direction, reference, record, best],
        );
        assert_eq!(e.mem.f32(best), 6.0);
        // First: (2-1)*2 = 2; second: (-2-1)*-2 = 6: the second wins and its
        // record stays in `c`.
        store_vec(&mut e, reference, [0.0, 0.0, 1.0, 0.0]);
        e.call(
            BEST_OF_TWO,
            &args![this, direction, reference, record, best],
        );
        assert_eq!(e.mem.f32(best), 6.0);
        assert_eq!(&load_vec(&e, record)[..3], &[0.0, 0.0, -2.0]);
    }

    /// A polytope face with three vertex records, each with the point
    /// (+0x00), the first shape's point (+0x10) and the second's (+0x20).
    fn face_with_vertices(e: &mut Engine, vertices: [Vec4; 3]) -> u32 {
        let face = e.mem.alloc(0x50);
        for (k, v) in vertices.into_iter().enumerate() {
            let record = e.mem.alloc(0x40);
            store_vec(e, record, v);
            store_vec(e, record + 0x10, [k as f32 + 1.0, 0.0, 0.0, 0.0]);
            store_vec(e, record + 0x20, [0.0, k as f32 + 1.0, 0.0, 0.0]);
            e.mem.set_u32(face + 0x10 * (k as u32 + 1), record);
        }
        store_vec(e, face, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_f32(face + 0x40, 2.0);
        face
    }

    #[test]
    fn the_closest_face_is_found_without_walking() {
        let mut e = engine();
        let this = search(&mut e, 0);
        let face = face_with_vertices(
            &mut e,
            [
                [1.0, 0.0, 2.0, 0.0],
                [0.0, 1.0, 2.0, 0.0],
                [-1.0, -1.0, 2.0, 0.0],
            ],
        );
        e.register_double(TRIANGLE_WEIGHTS, |e, a| {
            e.mem.set_f32(a[4], 0.5);
            e.mem.set_f32(a[4] + 4, 0.25);
            e.mem.set_f32(a[4] + 8, 0.25);
            Ret::default()
        });
        let out = e.mem.alloc(0x30);
        e.call(0x00df_9d90, &args![this, 0u32, face, out]);
        assert_eq!(e.mem.u32(this + 0x6c), 3);
        let w = e.mem.u32(this + 0x68);
        assert_eq!(load_vec(&e, w + 0x10), [0.0, 1.0, 2.0, 0.0]);
        // Closest point from the first shape's points (1,0,0), (2,0,0), (3,0,0).
        assert_close(&load_vec(&e, out + 0x10)[..3], &[1.75, 0.0, 0.0]);
        assert_eq!(&load_vec(&e, out)[..3], &[0.0, 0.0, -1.0]);
        assert_eq!(e.mem.f32(out + 0x20), -2.0);
    }

    #[test]
    fn a_negative_weight_moves_the_walk_to_the_next_face() {
        let mut e = engine();
        let this = search(&mut e, 0);
        let first = face_with_vertices(&mut e, [[0.0; 4]; 3]);
        let second = face_with_vertices(&mut e, [[0.0; 4]; 3]);
        e.mem.set_f32(second + 0x40, 7.0);
        let calls: CallLog = Default::default();
        let log = calls.clone();
        e.register_double(TRIANGLE_WEIGHTS, move |e, a| {
            let round = log.borrow().len();
            log.borrow_mut().push(a.to_vec());
            let value = if round == 0 { -1.0 } else { 1.0 };
            e.mem.set_f32(a[4], value);
            e.mem.set_f32(a[4] + 4, 0.0);
            e.mem.set_f32(a[4] + 8, 0.0);
            Ret::default()
        });
        e.register_double(CLIMB_FACES, move |_, a| {
            assert_eq!(a[0], 0x1234);
            assert_eq!(a[1], first);
            Ret {
                eax: second,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(0x30);
        e.call(0x00df_9d90, &args![this, 0x1234u32, first, out]);
        assert_eq!(calls.borrow().len(), 2);
        // The distance comes from the face the walk ended on.
        assert_eq!(e.mem.f32(out + 0x20), -7.0);
    }

    #[test]
    fn expanding_stops_when_the_point_is_inside_the_tolerance() {
        let mut e = engine();
        let this = search(&mut e, 0);
        store_vec(&mut e, this + 0x40, [1.0; 4]);
        let polytope = e.mem.alloc(0x40);
        let face = face_with_vertices(&mut e, [[0.0; 4]; 3]);
        let point = e.mem.alloc(16);
        store_vec(&mut e, point, [1.0, 2.0, 3.0, 0.0]);
        e.register_double(SUBTRACT_WRAPPER, |e, a| {
            store_vec(e, a[0], [0.5, 2.0, 2.0, 0.0]);
            Ret::default()
        });
        let state = e.mem.alloc(4);
        let result = e
            .call(0x00df_9c90, &args![this, polytope, face, point, state])
            .u32();
        assert_eq!(result, 1);
        assert_eq!(e.mem.u32(state), 0);
    }

    #[test]
    fn expanding_reports_edge_check_room_and_closing_failures() {
        let mut e = engine();
        let this = search(&mut e, 0);
        store_vec(&mut e, this + 0x40, [1.0; 4]);
        let polytope = e.mem.alloc(0x40);
        let face = face_with_vertices(&mut e, [[0.0; 4]; 3]);
        let point = e.mem.alloc(16);
        store_vec(&mut e, point, [1.0, 2.0, 3.0, 0.0]);
        e.register_double(SUBTRACT_WRAPPER, |e, a| {
            store_vec(e, a[0], [5.0, 5.0, 5.0, 0.0]);
            Ret::default()
        });
        let edge_result: std::rc::Rc<std::cell::Cell<u32>> = Default::default();
        let count: std::rc::Rc<std::cell::Cell<u32>> = Default::default();
        let close_result: std::rc::Rc<std::cell::Cell<u32>> = Default::default();
        let (edge_cell, count_cell) = (edge_result.clone(), count.clone());
        e.register_double(EDGE_CHECK, move |e, a| {
            e.mem.set_u32(a[3], count_cell.get());
            e.mem.set_u32(a[4], 0x7777);
            Ret {
                eax: edge_cell.get(),
                ..Ret::default()
            }
        });
        let close_cell = close_result.clone();
        let closes: CallLog = Default::default();
        let log = closes.clone();
        e.register_double(CLOSE_HOLE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: close_cell.get(),
                ..Ret::default()
            }
        });
        let state = e.mem.alloc(4);
        // The edge check fails: 3.
        edge_result.set(1);
        let r = e
            .call(0x00df_9c90, &args![this, polytope, face, point, state])
            .u32();
        assert_eq!((r, e.mem.u32(state)), (1, 3));
        // Too many edges for the room left: 2 (room is 0 - 0 + 100).
        edge_result.set(0);
        count.set(101);
        let r = e
            .call(0x00df_9c90, &args![this, polytope, face, point, state])
            .u32();
        assert_eq!((r, e.mem.u32(state)), (1, 2));
        // The hole closes: no answer (0); a failure gives 3.
        count.set(4);
        let r = e
            .call(0x00df_9c90, &args![this, polytope, face, point, state])
            .u32();
        assert_eq!(r, 0);
        assert_eq!(closes.borrow()[0], vec![polytope, 4, 0x7777, point]);
        close_result.set(2);
        let r = e
            .call(0x00df_9c90, &args![this, polytope, face, point, state])
            .u32();
        assert_eq!((r, e.mem.u32(state)), (1, 3));
    }

    #[test]
    fn more_than_four_points_end_the_round_with_status_three() {
        let mut e = engine();
        let this = search(&mut e, 5);
        let out = e.mem.alloc(0x30);
        let status = e.mem.alloc(4);
        e.mem.set_u32(status, 9);
        let r = e.call(0x00df_9fe0, &args![this, out, status]).u32() & 0xff;
        assert_eq!(r, 1);
        assert_eq!(e.mem.u32(status), 3);
    }

    #[test]
    fn a_flat_tetrahedron_is_rejected() {
        let mut e = engine();
        let this = search(&mut e, 4);
        let w = e.mem.u32(this + 0x68);
        store_vec(&mut e, w, [0.0; 4]);
        store_vec(&mut e, w + 0x10, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, w + 0x20, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, w + 0x30, [0.0, 0.0, 5.0, 0.0]);
        let out = e.mem.alloc(0x30);
        let status = e.mem.alloc(4);
        e.mem.set_u32(status, 9);
        let r = e.call(0x00df_9fe0, &args![this, out, status]).u32() & 0xff;
        assert_eq!(r, 0);
        assert_eq!(e.mem.u32(status), 0);
    }

    #[test]
    fn a_triangle_beyond_the_tolerance_gives_the_closest_point() {
        let mut e = engine();
        let this = search(&mut e, 3);
        let w = e.mem.u32(this + 0x68);
        let a = e.mem.u32(this + 0x60);
        store_vec(&mut e, w, [0.0; 4]);
        store_vec(&mut e, w + 0x10, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, w + 0x20, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, a, [7.0, 8.0, 9.0, 0.0]);
        e.register_double(BEST_OF_TWO, |e, a| {
            e.mem.set_f32(a[4], 0.0);
            Ret::default()
        });
        e.register_double(TRIANGLE_WEIGHTS, |e, a| {
            e.mem.set_f32(a[4], 1.0);
            e.mem.set_f32(a[4] + 4, 0.0);
            e.mem.set_f32(a[4] + 8, 0.0);
            Ret::default()
        });
        let out = e.mem.alloc(0x30);
        let status = e.mem.alloc(4);
        e.mem.set_u32(status, 9);
        let r = e.call(0x00df_9fe0, &args![this, out, status]).u32() & 0xff;
        assert_eq!(r, 1);
        assert_eq!(e.mem.u32(status), 0);
        assert_close(&load_vec(&e, out)[..3], &[0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [7.0, 8.0, 9.0]);
        assert_eq!(e.mem.f32(out + 0x20), 0.0);
    }

    #[test]
    fn a_collinear_triangle_after_a_segment_search_is_degenerate() {
        let mut e = engine();
        let this = search(&mut e, 2);
        let w = e.mem.u32(this + 0x68);
        let a = e.mem.u32(this + 0x60);
        store_vec(&mut e, w, [0.0; 4]);
        store_vec(&mut e, w + 0x10, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, a, [7.0, 8.0, 9.0, 0.0]);
        // The first search finds 5.0 and the second 1.0: the first record
        // (the point (2, 0, 0)) is appended, making the triangle collinear.
        let counter: std::rc::Rc<std::cell::Cell<u32>> = Default::default();
        let count = counter.clone();
        e.register_double(BEST_OF_TWO, move |e, a| {
            let call = count.get();
            count.set(call + 1);
            e.mem.set_f32(a[4], if call == 0 { 5.0 } else { 1.0 });
            store_vec(e, a[3], [2.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        let out = e.mem.alloc(0x30);
        let status = e.mem.alloc(4);
        e.mem.set_u32(status, 9);
        let r = e.call(0x00df_9fe0, &args![this, out, status]).u32() & 0xff;
        assert_eq!(r, 1);
        assert_eq!(counter.get(), 2);
        assert_eq!(e.mem.u32(status), 0);
        assert_close(&load_vec(&e, out)[..3], &[0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [7.0, 8.0, 9.0]);
        assert_eq!(e.mem.f32(out + 0x20), 0.0);
    }

    #[test]
    fn a_simplex_of_zero_vectors_falls_back_to_the_origin() {
        let mut e = engine();
        let this = search(&mut e, 4);
        e.register_double(SUPPORT_OF_DIFFERENCE, |_, _| Ret::default());
        let out = e.mem.alloc(0x30);
        let status = e.mem.alloc(4);
        e.mem.set_u32(status, 9);
        let r = e.call(0x00df_9fe0, &args![this, out, status]).u32() & 0xff;
        assert_eq!(r, 1);
        assert_eq!(e.mem.u32(status), 0);
        assert_eq!(e.mem.u32(this + 0x6c), 1);
        assert_eq!(e.mem.f32(out + 0x20), 0.0);
    }
}

#[cfg(test)]
mod completion_tests {
    use super::*;

    type CallLog = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn the_completion_adds_one_and_releases_the_semaphore() {
        let mut e = Engine::new();
        let log = CallLog::default();
        for (address, eax) in [(INTERLOCKED_EXCHANGE_ADD, 5u32), (SEMAPHORE_RELEASE, 8)] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                let mut words = vec![address];
                words.extend_from_slice(a);
                log.borrow_mut().push(words);
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00df_9840, &args![this]).u32(), 0);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(this + 0x18, 0x7001);
        assert_eq!(e.call(0x00df_9840, &args![this]).u32(), 5);
        e.mem.set_u32(this + 0x10, 0x7002);
        assert_eq!(e.call(0x00df_9840, &args![this]).u32(), 8);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![INTERLOCKED_EXCHANGE_ADD, 0x7001, 1],
                vec![INTERLOCKED_EXCHANGE_ADD, 0x7001, 1],
                vec![SEMAPHORE_RELEASE, 0x7002, 1],
            ]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    /// A source block of 0x80 bytes with a pattern, the kind byte, the step (+0x20), the data pointer
    /// (+0x24), the count (+0x28) and a marker in the word at +0x30.
    fn source_block(e: &mut Engine, kind: u8, step: u32, data: u32, count: u32) -> u32 {
        let source = e.mem.alloc(0x80);
        for word in 0..32u32 {
            e.mem.set_u32(source + 4 * word, 0x1000 + word);
        }
        e.mem.set_u8(source, kind);
        e.mem.set_u32(source + 0x20, step);
        e.mem.set_u32(source + 0x24, data);
        e.mem.set_u32(source + 0x28, count);
        e.mem.set_u32(source + 0x30, 0xabcd);
        source
    }

    #[test]
    fn fn_00df9880_splits_a_kind_zero_job_by_eighty_byte_entries() {
        let mut e = Engine::new();
        let source = source_block(&mut e, 0, 3, 0x10000, 10);
        let target = e.mem.alloc(0x80);
        assert_eq!(
            e.call(0x00df_9880, &args![0u32, 0u32, source, target])
                .u32(),
            1
        );
        // The copy is taken before the update, so the target's count word is the step.
        assert_eq!(e.mem.u32(target + 0x28), 3);
        assert_eq!(e.mem.u32(target + 0x24), 0x10000);
        assert_eq!(e.mem.u32(target + 0x30), 0xabcd);
        assert_eq!(e.mem.u32(target + 0x7c), 0x1000 + 31);
        assert_eq!(e.mem.u32(source + 0x28), 7);
        assert_eq!(e.mem.u32(source + 0x24), 0x10000 + 3 * 80);
        assert_eq!(e.mem.u32(source + 0x30), 0xabcd);
    }

    #[test]
    fn fn_00df9880_splits_a_kind_one_job_by_sixty_four_byte_entries() {
        let mut e = Engine::new();
        let source = source_block(&mut e, 1, 3, 0x10000, 10);
        let target = e.mem.alloc(0x80);
        assert_eq!(
            e.call(0x00df_9880, &args![0u32, 0u32, source, target])
                .u32(),
            1
        );
        assert_eq!(e.mem.u32(target + 0x28), 3);
        assert_eq!(e.mem.u32(source + 0x28), 7);
        assert_eq!(e.mem.u32(source + 0x24), 0x10000 + 3 * 64);
        assert_eq!(e.mem.u32(source + 0x30), 0);
    }

    /// A count that does not exceed the step, or another kind, answers 0 after the copy.
    #[test]
    fn fn_00df9880_answers_zero_when_nothing_is_left_to_split() {
        let mut e = Engine::new();
        let source = source_block(&mut e, 0, 10, 0x10000, 10);
        let target = e.mem.alloc(0x80);
        assert_eq!(
            e.call(0x00df_9880, &args![0u32, 0u32, source, target])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(target + 4), 0x1001);
        assert_eq!(e.mem.u32(source + 0x28), 10);
        let other = source_block(&mut e, 2, 3, 0x10000, 10);
        assert_eq!(
            e.call(0x00df_9880, &args![0u32, 0u32, other, target]).u32(),
            0
        );
        assert_eq!(e.mem.u32(other + 0x24), 0x10000);
        // A negative count is below the step in the signed compare.
        let negative = source_block(&mut e, 1, 3, 0x10000, 0xffff_fff0);
        assert_eq!(
            e.call(0x00df_9880, &args![0u32, 0u32, negative, target])
                .u32(),
            0
        );
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(COPY_RESULT_00DF7D00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn take(e: &mut Engine, addresses: &[u32]) -> Calls {
        let log = e.call_log.replace(vec![]).unwrap();
        log.into_iter()
            .filter(|(a, _)| addresses.contains(a))
            .collect()
    }

    #[test]
    fn fn_00df9920_finishes_or_writes_the_result_by_kind() {
        let mut e = rig();
        e.register(FINISH_JOB_00DF9840, |_, _| Ret::default());
        let counter = e.mem.alloc(4);
        let job = e.mem.alloc(0x70);
        e.mem.set_u32(job + 0x14, counter);
        let out = e.mem.alloc(0x60);
        let run = |e: &mut Engine, kind: u8, count: u32| -> (u32, usize) {
            e.mem.set_u8(job, kind);
            e.mem.set_u32(counter, count);
            let start = e.call_log.as_ref().unwrap().len();
            let answer = e.call(0x00df_9920, &args![0u32, 0u32, job, out]).u32();
            let finished = e.call_log.as_ref().unwrap()[start..]
                .iter()
                .filter(|(a, _)| *a == FINISH_JOB_00DF9840)
                .count();
            (answer, finished)
        };
        // 0, 1, 4: drop, finish at 0.
        for kind in [0u8, 1, 4] {
            assert_eq!(run(&mut e, kind, 2), (1, 0));
            assert_eq!(e.mem.u32(counter), 1);
            assert_eq!(run(&mut e, kind, 1), (1, 1));
        }
        assert_eq!(run(&mut e, 2, 9), (1, 1));
        e.mem.set_u32(job + 0x38, 7);
        assert_eq!(run(&mut e, 3, 9), (1, 1));
        e.mem.set_u32(job + 0x38, 8);
        assert_eq!(run(&mut e, 3, 9), (1, 0));
        assert_eq!(run(&mut e, 6, 1), (1, 0));
        assert_eq!(run(&mut e, 6, 0), (1, 1));
        assert_eq!(run(&mut e, 8, 0), (1, 0));
        // 5 with +0x60 clear: drop and finish.
        assert_eq!(run(&mut e, 5, 4), (1, 1));
        assert_eq!(e.mem.u32(counter), 3);
        // 5 with +0x60 set: the counter not at 0 answers 1, at 0 the header and copy.
        e.mem.set_u32(job + 0x60, 9);
        assert_eq!(run(&mut e, 5, 4), (1, 0));
        e.mem.set_u32(out, 0x77);
        assert_eq!(run(&mut e, 5, 1), (0, 0));
        let block = out + 0x10;
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(
            (e.mem.u8(block), e.mem.u8(block + 1), e.mem.u8(block + 2)),
            (3, 3, 1)
        );
        assert_eq!((e.mem.u16(block + 4), e.mem.u16(block + 6)), (0x50, 0xffff));
        assert_eq!(e.mem.u32(block + 0x14), counter);
        assert_eq!(
            take(&mut e, &[COPY_RESULT_00DF7D00]),
            vec![(COPY_RESULT_00DF7D00, vec![job, block])]
        );
        // 7: the kind-1 header from the job's words.
        let target = e.mem.alloc(4);
        for (offset, word) in [
            (0x44u32, 0x44u32),
            (0x48, 0x81),
            (0x4c, 0x4c),
            (0x50, 0x50),
            (0x54, 0x54),
            (0x58, target),
            (0x5c, 0x5c),
            (0x60, 0x60),
            (0x64, 0x64),
            (0x68, 0x68),
            (0x6c, 0x6c),
        ] {
            e.mem.set_u32(job + offset, word);
        }
        assert_eq!(run(&mut e, 7, 0), (0, 0));
        assert_eq!(e.mem.u8(block), 1);
        assert_eq!(e.mem.u32(target), 3);
        let words: Vec<u32> = (0x10..0x44u32)
            .step_by(4)
            .map(|offset| e.mem.u32(block + offset))
            .collect();
        assert_eq!(
            words,
            vec![0x5c, target, 0, 0x54, 0x40, 0x44, 0x81, 0x4c, 0x50, 0x60, 0x64, 0x68, 0x6c]
        );
        assert_eq!((e.mem.u32(out), e.mem.u8(out + 0x12)), (1, 1));
        e.mem.set_u32(job + 0x48, 0);
        run(&mut e, 7, 0);
        assert_eq!(e.mem.u32(target), 1);
        e.mem.set_u32(job + 0x48, 0x40);
        run(&mut e, 7, 0);
        assert_eq!(e.mem.u32(target), 1);
        e.mem.set_u32(job + 0x48, 0xffff_ff00);
        run(&mut e, 7, 0);
        assert_eq!(e.mem.u32(target), 0xffff_fffd);
    }
}
