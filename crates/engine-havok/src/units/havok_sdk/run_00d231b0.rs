//! `Havok SDK/run_00d231b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The eight functions are the helpers and the driver of a convex-hull style closest-point
//! search (the character proxy's simplex solver calls `00d24360`). None has a name in the
//! engine map or the Xbox PDB, so all are `fn_<address>` and their roles are read from the code.
//!
//! The search state is a "solver" block that `00d24360` keeps on its stack and the other
//! functions receive (as `context`):
//!
//! - `+0x00..+0x2f`: up to four 12-byte entries of three words each: the plane index, a pointer
//!   to the plane (0x40 bytes: normal and offset at +0x00, a point at +0x10, three tolerance
//!   `float`s at +0x20..+0x28, a blend weight at +0x2c, an `int` sort key at +0x30), and a
//!   pointer to the plane's 16-byte record in the output;
//! - `+0x30`: the number of entries;
//! - `+0x38`: the input block (`+0x10` the query point, `+0x20` the tolerance vector, `+0x30` a
//!   direction vector, `+0x40` the time budget, `+0x44` the time limit, `+0x48` the plane array,
//!   `+0x4c` the plane count);
//! - `+0x3c`: the output block (`+0x00` the position, `+0x10` the result point, `+0x20` the time
//!   used, `+0x24` the array of 16-byte plane records: bytes 0 and 1 used flags, `+4` an
//!   accumulated time, `+8` a margin, `+0xc` the state word).
//!
//! SSE code computes lane by lane in `f32`; a dot product is `z + (y + x)` the way the
//! `SHUFPS`/`ADDSS` sequences add it. x87 code works in double precision (the exe's default
//! control word), rounds at each `float` store, and compares the way its `FNSTSW`/`TEST AH`
//! pairs read the flags, `NaN` included. A `float` copied through `FLD`/`FSTP` goes through
//! [`x87_quiet`]. The functions pass some arguments in registers; the exe's calling convention
//! has no register arguments, so each doc comment lists them first, in the order named, then the
//! stack words.

#[allow(unused_imports)]
use crate::prelude::*;

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// The CRT square root (`00ec6040`, argument in ST0, result in ST0).
const SQUARE_ROOT: u32 = 0x00ec_6040;
/// `float` constants: the epsilon `2^-23` and the half-space tolerance `-0.001`.
const EPSILON: u32 = 0x0104_da4c;
const PLANE_TOLERANCE: u32 = 0x0104_ffb8;
/// `double` constants: `2^-20`, 0.5, 0.0, 1.0, `0.001f`, `0.0001f` and `2^-23`.
const LINE_EPSILON: u32 = 0x010c_b420;
const HALF: u32 = 0x0101_1588;
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
const ONE_THOUSANDTH: u32 = 0x0101_6978;
const MINIMUM_ADVANCE: u32 = 0x0103_2980;
const MARGIN_STEP: u32 = 0x010c_9300;

/// Four `float` lanes.
type Vec4 = [f32; 4];

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

fn vmul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

fn vsub(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

fn vadd(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

/// `broadcast(s) * v`.
fn scale(s: f32, v: Vec4) -> Vec4 {
    v.map(|x| s * x)
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

fn dot3(a: Vec4, b: Vec4) -> f32 {
    sum3(vmul(a, b))
}

/// `SHUFPS v, v, 0xc9`: lanes (y, z, x, w).
fn yzx(v: Vec4) -> Vec4 {
    [v[1], v[2], v[0], v[3]]
}

/// `SHUFPS v, v, 0xd2`: lanes (z, x, y, w).
fn zxy(v: Vec4) -> Vec4 {
    [v[2], v[0], v[1], v[3]]
}

fn ptr(addr: u32) -> Ptr {
    Ptr::new(addr)
}

/// The CRT square root of a `float`, stored back as a `float`.
fn square_root(e: &mut Engine, value: f32) -> f32 {
    e.call(SQUARE_ROOT, &args![f64::from(value)]).f64() as f32
}

/// `1 / value` on the x87 stack, stored as a `float`.
fn reciprocal(value: f32) -> f32 {
    (1.0 / f64::from(value)) as f32
}

/// Stores `value` in the state word (+0xc) of `vertex`'s record: the record is the one in the
/// output's array (`+0x24`) at the index of the plane's 0x40-byte slot in the input's plane array
/// (`+0x48`).
fn mark_vertex(e: &mut Engine, context: u32, vertex: u32, value: u32) {
    let input = e.mem.u32(context + 0x38);
    let output = e.mem.u32(context + 0x3c);
    let index = (vertex.wrapping_sub(e.mem.u32(input + 0x48)) as i32) >> 6;
    let slot = (index as u32)
        .wrapping_shl(4)
        .wrapping_add(e.mem.u32(output + 0x24))
        .wrapping_add(0xc);
    e.mem.set_u32(slot, value);
}

/// Entry `entry` (12 bytes at `context + 12 * entry`), word `word`.
fn entry_word(e: &Engine, context: u32, entry: u32, word: u32) -> u32 {
    e.mem.u32(context + 12 * entry + 4 * word)
}

/// Copies all three words of entry `from` over entry `to`.
fn copy_entry(e: &mut Engine, context: u32, from: u32, to: u32) {
    for word in 0..3 {
        let value = entry_word(e, context, from, word);
        e.mem.set_u32(context + 12 * to + 4 * word, value);
    }
}

/// Runs `fn_00d23480` with a scratch result byte: whether `point` is behind `plane`.
fn behind_plane(e: &mut Engine, plane: u32, point: u32) -> bool {
    let result = e.mem.alloc(8);
    fn_00d23480(e, ptr(result), ptr(plane), ptr(point));
    let behind = e.mem.u8(result) != 0;
    e.mem.free(result);
    behind
}

// Translated from 00d231b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the closest points of two lines (`from + s * direction`, `other_from + t *
/// other_direction`; all four are 16-byte vectors). Writes into the 0x30-byte `out`: the squared
/// distance at +0x00, `s` at +0x04, `t` at +0x08, the point on the first line at +0x10 and the
/// point on the second at +0x20 (the fourth lane of each interpolates the inputs' fourth lanes).
/// Returns whether the lines are (nearly) parallel: then `s` is 0.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the x87 compare treats NaN as "parallel"
pub fn fn_00d231b0(
    e: &mut Engine,
    from: Ptr,
    direction: Ptr,
    other_from: Ptr,
    other_direction: Ptr,
    out: Ptr,
) -> bool {
    // The SSE parts with the SSE NaN rules (the destination operand first), the x87 parts
    // with the x87 choice of NaN; `rounded` is a store to a scratch float.
    use crate::sse::{add, addps, mulps, subps};
    let dot = |a: Vec4, b: Vec4| {
        let products = mulps(a, b);
        add(products[2], add(products[1], products[0]))
    };
    let (fadd, fsub, fmul, fdiv) = (x87_add, x87_sub, x87_mul, x87_div);
    let rounded = |value: f64| f32::from_bits(x87_store32(value));
    let wide = |value: f32| f64::from(value);
    let p1 = load_vec(e, from.addr());
    let d1 = load_vec(e, direction.addr());
    let p2 = load_vec(e, other_from.addr());
    let d2 = load_vec(e, other_direction.addr());
    let d12 = dot(d2, d1);
    let offset = subps(p2, p1);
    let along_first = dot(d1, offset);
    let along_second = dot(d2, offset);
    let first_squared = dot(d1, d1);
    let second_squared = dot(d2, d2);
    let product = fmul(wide(second_squared), wide(first_squared));
    let d12_squared = fmul(wide(d12), wide(d12));
    let denominator = rounded(fsub(product, d12_squared));
    let product_stored = rounded(product);
    let sum = fadd(d12_squared, wide(product_stored.abs()));
    let threshold = rounded(fmul(sum, e.mem.f64(LINE_EPSILON)));
    let parallel = !(threshold < denominator.abs());
    let s = if parallel {
        0.0
    } else {
        let numerator = fsub(
            fmul(wide(second_squared), wide(along_first)),
            fmul(wide(along_second), wide(d12)),
        );
        rounded(fdiv(numerator, wide(denominator)))
    };
    let shifted = fsub(fmul(wide(d12), wide(s)), wide(along_second));
    let t = rounded(fdiv(shifted, wide(second_squared)));
    let point1 = addps(mulps([s; 4], d1), p1);
    let point2 = addps(mulps([t; 4], d2), p2);
    let distance = dot(subps(point1, point2), subps(point1, point2));
    let out = out.addr();
    e.mem.set_f32(out, x87_quiet(distance));
    store_vec(e, out + 0x10, point1);
    store_vec(e, out + 0x20, point2);
    e.mem.set_f32(out + 4, s);
    e.mem.set_f32(out + 8, t);
    parallel
}

/// Whether the entry at `anchor` has to swap places with the one at `other` (both entry
/// addresses): the larger sort key (+0x30 of the plane) goes later; with equal keys the plane
/// whose vector at +0x10 is longer goes first (a tie or a `NaN` swaps).
#[allow(clippy::neg_cmp_op_on_partial_ord)] // a NaN length swaps
fn should_swap(e: &Engine, anchor: u32, other: u32) -> bool {
    let first = e.mem.u32(anchor + 4);
    let second = e.mem.u32(other + 4);
    let first_key = e.mem.i32(first + 0x30);
    let second_key = e.mem.i32(second + 0x30);
    if first_key < second_key {
        return false;
    }
    if first_key != second_key {
        return true;
    }
    let first_vec = load_vec(e, first + 0x10);
    let second_vec = load_vec(e, second + 0x10);
    let first_length = dot3(first_vec, first_vec);
    let second_length = dot3(second_vec, second_vec);
    !(second_length > first_length)
}

// Translated from 00d23380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: sorts the solver's entries (`context`, count at +0x30) by an exchange sort: each
/// entry in turn is swapped with every later one that `should_swap` says belongs before it, the
/// three words of an entry moving together.
pub fn fn_00d23380(e: &mut Engine, context: Ptr) {
    let context = context.addr();
    let count = e.mem.i32(context + 0x30);
    if count <= 1 {
        return;
    }
    let mut i = 1;
    loop {
        let anchor = context + 12 * (i - 1) as u32;
        if i < e.mem.i32(context + 0x30) {
            let mut j = i;
            loop {
                let other = context + 12 * j as u32;
                if should_swap(e, anchor, other) {
                    for word in 0..3 {
                        let kept = e.mem.u32(anchor + 4 * word);
                        let moved = e.mem.u32(other + 4 * word);
                        e.mem.set_u32(anchor + 4 * word, moved);
                        e.mem.set_u32(other + 4 * word, kept);
                    }
                }
                j += 1;
                if j >= e.mem.i32(context + 0x30) {
                    break;
                }
            }
        }
        let go_on = i < e.mem.i32(context + 0x30) - 1;
        i += 1;
        if !go_on {
            break;
        }
    }
}

// Translated from 00d23480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: whether `point` is behind `plane` (normal at +0x00, a point on the plane at +0x10):
/// the signed distance is below the `float` -0.001 at `0104ffb8`, or `NaN`. Writes the answer
/// as a byte to `result` and returns `result`. Register arguments: `result` in ECX, `plane` in
/// EAX; `point` is the stack word.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // a NaN distance is behind
pub fn fn_00d23480(e: &mut Engine, result: Ptr, plane: Ptr, point: Ptr) -> Ptr {
    let normal = load_vec(e, plane.addr());
    let on_plane = load_vec(e, plane.addr() + 0x10);
    let at = load_vec(e, point.addr());
    let distance = dot3(vsub(at, on_plane), normal);
    let tolerance = e.mem.f32(PLANE_TOLERANCE);
    // FCOMP: not behind when the tolerance is below or equal to the distance.
    let behind = !(tolerance <= distance);
    e.mem.set_u8(result.addr(), u8::from(behind));
    result
}

// Translated from 00d234e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: projects `point` onto the shape at `shape` (an axis vector at +0x00, an origin at
/// +0x10, a radius at +0x20, two margins at +0x24 and +0x28, a blend weight at +0x2c) and writes
/// the projected point to `out`: the origin when the point is within the allowed distance, else
/// the origin plus the part of the offset perpendicular to the axis, scaled back when the blend
/// weight is below 1. The direction at `context` + 0x38 + 0x30 picks the margin. Register
/// arguments: `shape` in ECX, `point` in EAX, `out` in ESI; `context` is the stack word.
pub fn fn_00d234e0(e: &mut Engine, shape: Ptr, point: Ptr, out: Ptr, context: Ptr) {
    let shape = shape.addr();
    let out = out.addr();
    let axis = load_vec(e, shape);
    let origin = load_vec(e, shape + 0x10);
    let radius = x87_quiet(e.mem.f32(shape + 0x20));
    let blend = x87_quiet(e.mem.f32(shape + 0x2c));
    let input = e.mem.u32(context.addr() + 0x38);
    let query = load_vec(e, point.addr());
    let offset = vsub(query, origin);
    let along = dot3(axis, offset);
    let offset_squared = dot3(offset, offset);
    let mut perpendicular = vsub(offset, scale(along, axis));
    let along_squared = along * along;
    let direction = load_vec(e, input + 0x30);
    let side = dot3(direction, perpendicular);
    let margin_slot = if 0.0 < side { 0x24 } else { 0x28 };
    let margin = x87_quiet(e.mem.f32(shape + margin_slot));
    let radius_squared = (f64::from(radius) * f64::from(radius)) as f32;
    if 0.0 < margin {
        let cross = vsub(
            vmul(zxy(axis), yzx(direction)),
            vmul(yzx(axis), zxy(direction)),
        );
        let cross_squared = dot3(cross, cross);
        let mut unit = [0.0; 4];
        let mut hit = 0.0_f32;
        if cross_squared > e.mem.f32(EPSILON) {
            let length = square_root(e, cross_squared);
            unit = scale(reciprocal(length), cross);
            hit = dot3(unit, perpendicular);
            let bound = f64::from(radius_squared) * f64::from(along_squared);
            let hit_squared = (f64::from(hit) * f64::from(hit)) as f32;
            if f64::from(hit_squared) <= bound {
                perpendicular = vsub(perpendicular, scale(hit, unit));
                hit = 0.0;
            }
        }
        let reach = f64::from(margin) + f64::from(radius);
        let reach_squared = (reach * reach) as f32;
        let limit = f64::from(reach_squared) * f64::from(along_squared);
        let slack = ((f64::from(offset_squared) - f64::from(hit) * f64::from(hit))
            - f64::from(along_squared)) as f32;
        if f64::from(slack) <= limit {
            if hit == 0.0 {
                copy_vec(e, shape + 0x10, out);
                return;
            }
            perpendicular = scale(hit, unit);
        }
    } else {
        let bound = (f64::from(radius_squared) + e.mem.f64(ONE)) * f64::from(along_squared);
        if f64::from(offset_squared) <= bound {
            copy_vec(e, shape + 0x10, out);
            return;
        }
    }
    if 1.0 > blend {
        let perpendicular_squared = dot3(perpendicular, perpendicular);
        if perpendicular_squared >= e.mem.f32(EPSILON)
            && e.mem.f64(ONE_THOUSANDTH) * f64::from(offset_squared)
                < f64::from(perpendicular_squared)
        {
            let ratio = (f64::from(offset_squared) / f64::from(perpendicular_squared)) as f32;
            let root = square_root(e, ratio);
            let weight = (f64::from(blend) + (1.0 - f64::from(blend)) * f64::from(root)) as f32;
            let scaled = scale(weight, perpendicular);
            let along_scaled = dot3(axis, scaled);
            let result = vadd(origin, vsub(scaled, scale(along_scaled, axis)));
            store_vec(e, out, result);
            return;
        }
    }
    store_vec(e, out, vadd(origin, perpendicular));
}

/// The projection `00d23850` falls back on: marks both planes as visited (state 2) and projects
/// `point` onto each in turn (the one with the smaller sort key first), the second projection
/// starting from the first's result in `out`.
fn project_onto_both(e: &mut Engine, context: Ptr, first: u32, second: u32, point: Ptr, out: Ptr) {
    mark_vertex(e, context.addr(), first, 2);
    mark_vertex(e, context.addr(), second, 2);
    if e.mem.i32(first + 0x30) <= e.mem.i32(second + 0x30) {
        fn_00d234e0(e, ptr(first), point, out, context);
        fn_00d234e0(e, ptr(second), out, out, context);
    } else {
        fn_00d234e0(e, ptr(second), point, out, context);
        fn_00d234e0(e, ptr(first), out, out, context);
    }
}

// Translated from 00d23850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the closest point to `point` on the line where two planes (`first`, `second`) meet.
/// The line's direction is the normalized cross product of the two normals; its point comes from
/// the planes' offsets; the point is written to `out` (the point plus the part of the offset to
/// `point` along the line, shortened by a blend weight when the offset is mostly off the line).
/// When the planes are parallel, or the line's point lies outside the input's tolerance vector
/// (`context` + 0x38 + 0x20), it marks both planes visited and projects onto each in turn
/// instead (`project_onto_both`). `context` is passed in EDI; the four others are stack words.
pub fn fn_00d23850(e: &mut Engine, context: Ptr, first: Ptr, second: Ptr, point: Ptr, out: Ptr) {
    let (a, b) = (first.addr(), second.addr());
    let a0 = load_vec(e, a);
    let b0 = load_vec(e, b);
    let cross = vsub(vmul(zxy(b0), yzx(a0)), vmul(yzx(b0), zxy(a0)));
    let cross_squared = dot3(cross, cross);
    if cross_squared <= e.mem.f32(EPSILON) {
        project_onto_both(e, context, a, b, point, out);
        return;
    }
    let length = square_root(e, cross_squared);
    let line = scale(reciprocal(length), cross);
    let around_first = vsub(vmul(yzx(line), zxy(a0)), vmul(zxy(line), yzx(a0)));
    let around_second = vsub(vmul(zxy(line), yzx(b0)), vmul(yzx(line), zxy(b0)));
    let a1 = load_vec(e, a + 0x10);
    let b1 = load_vec(e, b + 0x10);
    let first_offset = dot3(a1, a0);
    let second_offset = dot3(b1, b0);
    let along = dot3(vadd(a1, b1), line);
    let middle = (e.mem.f64(HALF) * f64::from(along)) as f32;
    let sum = vadd(
        vadd(scale(middle, cross), scale(first_offset, around_second)),
        scale(second_offset, around_first),
    );
    let on_line = scale(reciprocal(length), sum);
    let input = e.mem.u32(context.addr() + 0x38);
    let tolerance = load_vec(e, input + 0x20);
    if (0..3).any(|lane| tolerance[lane] < on_line[lane].abs()) {
        project_onto_both(e, context, a, b, point, out);
        return;
    }
    let target = load_vec(e, point.addr());
    let to_target = vsub(target, on_line);
    let to_target_squared = dot3(to_target, to_target);
    let reference = load_vec(e, input + 0x30);
    let facing = dot3(reference, line);
    let distance = dot3(to_target, line);
    let first_base = (f64::from(e.mem.f32(a + 0x20)) + f64::from(e.mem.f32(b + 0x20))) as f32;
    let distance_squared = f64::from(distance) * f64::from(distance);
    let slot = if f64::from(distance) * f64::from(facing) > e.mem.f64(ZERO) {
        0x24
    } else {
        0x28
    };
    let selected = f64::from(e.mem.f32(a + slot)) + f64::from(e.mem.f32(b + slot));
    let shifted = (f64::from(facing) * selected + f64::from(first_base)) as f32;
    let middle_offset = (f64::from(shifted) * e.mem.f64(HALF)) as f32;
    let weight_sum = f64::from(e.mem.f32(a + 0x2c)) + f64::from(e.mem.f32(b + 0x2c));
    let weight = (e.mem.f64(HALF) * weight_sum) as f32;
    let distance_squared_stored = distance_squared as f32;
    let remainder = f64::from(to_target_squared) - f64::from(distance_squared_stored);
    let middle_squared = (f64::from(middle_offset) * f64::from(middle_offset)) as f32;
    let limit = remainder * f64::from(middle_squared);
    if limit >= f64::from(distance_squared_stored) {
        store_vec(e, out.addr(), on_line);
        return;
    }
    let mut step = distance;
    if 1.0 > weight && e.mem.f64(ONE_THOUSANDTH) * f64::from(to_target_squared) < distance_squared {
        let inverse = reciprocal(distance).abs();
        let root = square_root(e, to_target_squared);
        let ratio = (f64::from(root) * f64::from(inverse)) as f32;
        let blended = (f64::from(weight) + (1.0 - f64::from(weight)) * f64::from(ratio)) as f32;
        step = (f64::from(blended) * f64::from(distance)) as f32;
    }
    store_vec(e, out.addr(), vadd(scale(step, line), on_line));
}

// Translated from 00d23ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the point where three planes (`first`, `second`, `third`) meet, written to `out`. It
/// solves the 3x3 system by cross products: the determinant of the three normals, and for each
/// plane its offset times the cross product of the other two normals. When the determinant is
/// below the epsilon, or the solution lies outside the input's tolerance vector (`context` +
/// 0x38 + 0x20), it marks the three planes with state 1 (sorting the solver's entries first when
/// `sort` is not 0, and taking the planes from the first three entries) and runs `00d23850` on
/// each pair in turn while the solver's entry count does not change. Register arguments:
/// `context` in ECX, `first` in EAX; `second`, `third`, `sort`, `point`, `out` are stack words
/// (`point` is only passed on to `00d23850`).
#[allow(clippy::too_many_arguments)] // The exe ABI has two register and five stack arguments.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the x87 compare treats NaN as "not below"
pub fn fn_00d23ca0(
    e: &mut Engine,
    context: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    sort: u32,
    point: Ptr,
    out: Ptr,
) {
    let ctx = context.addr();
    let (mut v1, mut v2, mut v3) = (first.addr(), second.addr(), third.addr());
    let a0 = load_vec(e, v1);
    let b0 = load_vec(e, v2);
    let c0 = load_vec(e, v3);
    let cross23 = vsub(vmul(zxy(c0), yzx(b0)), vmul(yzx(c0), zxy(b0)));
    let cross12 = vsub(vmul(yzx(a0), zxy(b0)), vmul(zxy(a0), yzx(b0)));
    let cross31 = vsub(vmul(zxy(a0), yzx(c0)), vmul(yzx(a0), zxy(c0)));
    let determinant = dot3(a0, cross23);
    if !(determinant.abs() < e.mem.f32(EPSILON)) {
        let offset3 = dot3(load_vec(e, v3 + 0x10), c0);
        let offset2 = dot3(load_vec(e, v2 + 0x10), b0);
        let offset1 = dot3(load_vec(e, v1 + 0x10), a0);
        let inverse = reciprocal(determinant);
        let sum = vadd(
            vadd(scale(offset2, cross31), scale(offset1, cross23)),
            scale(offset3, cross12),
        );
        let solution = scale(inverse, sum);
        let input = e.mem.u32(ctx + 0x38);
        let tolerance = load_vec(e, input + 0x20);
        if !(0..3).any(|lane| tolerance[lane] < solution[lane].abs()) {
            store_vec(e, out.addr(), solution);
            return;
        }
    }
    if sort != 0 {
        fn_00d23380(e, context);
        v1 = entry_word(e, ctx, 0, 1);
        v2 = entry_word(e, ctx, 1, 1);
        v3 = entry_word(e, ctx, 2, 1);
    }
    for vertex in [v1, v2, v3] {
        mark_vertex(e, ctx, vertex, 1);
    }
    let count = e.mem.u32(ctx + 0x30);
    fn_00d23850(e, context, ptr(v1), ptr(v2), point, out);
    if e.mem.u32(ctx + 0x30) == count {
        fn_00d23850(e, context, ptr(v1), ptr(v3), out, out);
    }
    if e.mem.u32(ctx + 0x30) == count {
        fn_00d23850(e, context, ptr(v2), ptr(v3), out, out);
    }
}

/// The wrapper `00d23f50` uses for `00d23ca0` with the first three entries' planes: no sort,
/// `point` and `out` as given.
fn solve_three(e: &mut Engine, context: u32, planes: [u32; 3], point: u32, out: u32) {
    fn_00d23ca0(
        e,
        ptr(context),
        ptr(planes[0]),
        ptr(planes[1]),
        ptr(planes[2]),
        0,
        ptr(point),
        ptr(out),
    );
}

// Translated from 00d23f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: reduces the solver's simplex (`context` in EAX, entry count at +0x30) towards the
/// point nearest the input's query point (`input + 0x10`), writing it to the output's result
/// point (`output + 0x10`). With one entry it projects onto that plane (`00d234e0`). With two it
/// projects onto the second and keeps the second when the projected point is not behind the
/// first, else solves for both planes (`00d23850`). With three or four it checks the faces, drops
/// the entries the point is not behind, or falls back on `00d23ca0` for the triples. Cases that
/// shrink the entry count loop again with the smaller simplex.
pub fn fn_00d23f50(e: &mut Engine, context: Ptr) {
    let ctx = context.addr();
    loop {
        let index = e.mem.u32(ctx + 0x30).wrapping_sub(1);
        if index > 3 {
            return;
        }
        let input = e.mem.u32(ctx + 0x38);
        let output = e.mem.u32(ctx + 0x3c);
        match index {
            0 => {
                let plane = entry_word(e, ctx, 0, 1);
                fn_00d234e0(
                    e,
                    ptr(plane),
                    ptr(input + 0x10),
                    ptr(output + 0x10),
                    context,
                );
                return;
            }
            1 => {
                let buffer = e.mem.alloc(0x10);
                let second = entry_word(e, ctx, 1, 1);
                fn_00d234e0(e, ptr(second), ptr(input + 0x10), ptr(buffer), context);
                let first = entry_word(e, ctx, 0, 1);
                if behind_plane(e, first, buffer) {
                    fn_00d23850(
                        e,
                        context,
                        ptr(first),
                        ptr(second),
                        ptr(input + 0x10),
                        ptr(output + 0x10),
                    );
                } else {
                    copy_vec(e, buffer, output + 0x10);
                    copy_entry(e, ctx, 1, 0);
                    e.mem.set_u32(ctx + 0x30, 1);
                }
                e.mem.free(buffer);
                return;
            }
            2 => {
                let buffer = e.mem.alloc(0x10);
                let third = entry_word(e, ctx, 2, 1);
                fn_00d234e0(e, ptr(third), ptr(input + 0x10), ptr(buffer), context);
                let first = entry_word(e, ctx, 0, 1);
                let second = entry_word(e, ctx, 1, 1);
                if !behind_plane(e, first, buffer) && !behind_plane(e, second, buffer) {
                    copy_vec(e, buffer, output + 0x10);
                    copy_entry(e, ctx, 2, 0);
                    e.mem.set_u32(ctx + 0x30, 1);
                    e.mem.free(buffer);
                    continue;
                }
                let scratch = e.mem.alloc(0x10);
                let mut kept = None;
                for candidate in 0..2 {
                    let input = e.mem.u32(ctx + 0x38);
                    let vertex = entry_word(e, ctx, candidate, 1);
                    let third = entry_word(e, ctx, 2, 1);
                    fn_00d23850(
                        e,
                        context,
                        ptr(vertex),
                        ptr(third),
                        ptr(input + 0x10),
                        ptr(scratch),
                    );
                    let other = entry_word(e, ctx, 1 - candidate, 1);
                    if !behind_plane(e, other, scratch) {
                        kept = Some(candidate);
                        break;
                    }
                }
                e.mem.free(buffer);
                e.mem.free(scratch);
                let Some(kept) = kept else {
                    let (first, second, third) = (
                        entry_word(e, ctx, 0, 1),
                        entry_word(e, ctx, 1, 1),
                        entry_word(e, ctx, 2, 1),
                    );
                    fn_00d23ca0(
                        e,
                        context,
                        ptr(first),
                        ptr(second),
                        ptr(third),
                        1,
                        ptr(input + 0x10),
                        ptr(output + 0x10),
                    );
                    return;
                };
                copy_entry(e, ctx, kept, 0);
                let count = e.mem.u32(ctx + 0x30).wrapping_sub(1);
                e.mem.set_u32(ctx + 0x30, count);
                copy_entry(e, ctx, 2, 1);
            }
            _ => {
                fn_00d23380(e, context);
                let buffer = e.mem.alloc(0x10);
                let mut accepted = None;
                for face in 1..4_u32 {
                    let input = e.mem.u32(ctx + 0x38);
                    let planes = [
                        entry_word(e, ctx, face % 3, 1),
                        entry_word(e, ctx, (face + 1) % 3, 1),
                        entry_word(e, ctx, 3, 1),
                    ];
                    solve_three(e, ctx, planes, input + 0x10, buffer);
                    let opposite = entry_word(e, ctx, face - 1, 1);
                    if !behind_plane(e, opposite, buffer) {
                        accepted = Some(face - 1);
                        break;
                    }
                }
                if let Some(slot) = accepted {
                    copy_entry(e, ctx, 2, slot);
                    copy_entry(e, ctx, 3, 2);
                    e.mem.set_u32(ctx + 0x30, 3);
                    e.mem.free(buffer);
                    continue;
                }
                copy_vec(e, input + 0x10, buffer);
                let planes: Vec<u32> = (0..4).map(|entry| entry_word(e, ctx, entry, 1)).collect();
                let count = e.mem.u32(ctx + 0x30);
                solve_three(e, ctx, [planes[0], planes[1], planes[2]], buffer, buffer);
                if e.mem.u32(ctx + 0x30) == count {
                    solve_three(e, ctx, [planes[0], planes[1], planes[3]], buffer, buffer);
                }
                if e.mem.u32(ctx + 0x30) == count {
                    solve_three(e, ctx, [planes[0], planes[2], planes[3]], buffer, buffer);
                }
                if e.mem.u32(ctx + 0x30) == count {
                    solve_three(e, ctx, [planes[1], planes[2], planes[3]], buffer, buffer);
                }
                copy_vec(e, buffer, output + 0x10);
                e.mem.free(buffer);
                // The largest state word among the four entries' records (at least 0).
                let mut largest = 0;
                for entry in 0..4 {
                    let state = e.mem.i32(entry_word(e, ctx, entry, 2) + 0xc);
                    if state > largest {
                        largest = state;
                    }
                }
                for entry in 0..4 {
                    if e.mem.i32(entry_word(e, ctx, entry, 2) + 0xc) == largest {
                        copy_entry(e, ctx, 3, entry);
                        break;
                    }
                }
                let count = e.mem.u32(ctx + 0x30).wrapping_sub(1);
                e.mem.set_u32(ctx + 0x30, count);
                for entry in 0..3 {
                    let record = entry_word(e, ctx, entry, 2);
                    e.mem.set_u32(record + 0xc, 0);
                }
            }
        }
    }
}

/// The body of `00d24360`: `solver` is the solver block on its stack.
fn search(e: &mut Engine, input: u32, output: u32, solver: u32) {
    let mut total = 0.0_f32;
    let mut remaining = x87_quiet(e.mem.f32(input + 0x40));
    copy_vec(e, input + 0x10, output + 0x10);
    copy_vec(e, input, output);
    e.mem.set_u32(solver + 0x30, 0);
    let mut plane_index = 0;
    while plane_index < e.mem.i32(input + 0x4c) {
        let record = e.mem.u32(output + 0x24) + 16 * plane_index as u32;
        e.mem.set_f32(record + 4, 0.0);
        e.mem.set_f32(record + 8, 0.0);
        e.mem.set_u8(record + 1, 0);
        e.mem.set_u8(record, 0);
        e.mem.set_u32(record + 0xc, 0);
        plane_index += 1;
    }
    // A negative or NaN budget stops at once.
    if remaining < 0.0 || remaining.is_nan() {
        return;
    }
    loop {
        let count = e.mem.i32(solver + 0x30);
        let mut best_index: i32 = -1;
        let mut best_time = remaining;
        let planes = e.mem.i32(input + 0x4c);
        for index in 0..planes {
            let in_simplex = (count >= 1 && e.mem.i32(solver) == index)
                || (count >= 2 && e.mem.i32(solver + 12) == index)
                || (count >= 3 && e.mem.i32(solver + 24) == index);
            if in_simplex {
                continue;
            }
            let record = e.mem.u32(output + 0x24) + 16 * index as u32;
            if e.mem.u32(record + 0xc) != 0 {
                continue;
            }
            let plane = e.mem.u32(input + 0x48) + 0x40 * index as u32;
            let position = load_vec(e, output + 0x10);
            let on_plane = load_vec(e, plane + 0x10);
            let normal = load_vec(e, plane);
            let approach = -dot3(vsub(position, on_plane), normal);
            if approach <= 0.0 {
                continue;
            }
            let minus_total = -total;
            let moved = vadd(scale(minus_total, on_plane), load_vec(e, output));
            let weighted = vmul(moved, normal);
            let mut distance = (weighted[2] + normal[3]) + (weighted[1] + weighted[0]);
            if distance <= e.mem.f32(EPSILON) {
                distance = 0.0;
            }
            let margin = e.mem.f32(e.mem.u32(output + 0x24) + 16 * index as u32 + 8);
            let allowed = (f64::from(margin) + f64::from(distance)) as f32;
            if f64::from(approach) * f64::from(best_time) > f64::from(allowed) {
                best_index = index;
                best_time = (f64::from(allowed) / f64::from(approach)) as f32;
            }
        }
        if f64::from(best_time) > e.mem.f64(MINIMUM_ADVANCE) {
            total = (f64::from(total) + f64::from(best_time)) as f32;
            let velocity = load_vec(e, output + 0x10);
            let position = load_vec(e, output);
            store_vec(e, output, vadd(scale(best_time, velocity), position));
            remaining = (f64::from(remaining) - f64::from(best_time)) as f32;
            for entry in 0..count {
                let record = entry_word(e, solver, entry as u32, 2);
                let accumulated = f64::from(e.mem.f32(record + 4)) + f64::from(best_time);
                e.mem.set_f32(record + 4, accumulated as f32);
                e.mem.set_u8(record, 1);
            }
            e.mem.set_f32(output + 0x20, total);
            if e.mem.f32(input + 0x44) < total {
                return;
            }
        }
        if best_index < 0 {
            let budget = x87_quiet(e.mem.f32(input + 0x40));
            e.mem.set_f32(output + 0x20, budget);
            return;
        }
        let plane = e.mem.u32(input + 0x48) + 0x40 * best_index as u32;
        let record = e.mem.u32(output + 0x24) + 16 * best_index as u32;
        let slot = solver + 12 * count as u32;
        e.mem.set_u32(solver + 0x30, (count + 1) as u32);
        e.mem.set_u32(slot + 4, plane);
        e.mem.set_u32(slot + 8, record);
        e.mem.set_u32(slot, best_index as u32);
        let margin = f64::from(e.mem.f32(record + 8)) + e.mem.f64(MARGIN_STEP);
        e.mem.set_f32(record + 8, (margin + margin) as f32);
        fn_00d23f50(e, ptr(solver));
        if remaining < 0.0 || remaining.is_nan() {
            return;
        }
    }
}

// Translated from 00d24360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the driver of the search. Copies the input's two vectors (+0x00 and +0x10) to the
/// output, clears the output's plane records, and then, while the time budget (`input + 0x40`)
/// is not negative: finds the plane the position will meet first when it moves by the output's
/// +0x10 vector (the smallest margin-adjusted time over the planes not already in the simplex
/// and not marked), advances the position by that time (once it exceeds 0.0001), adds the time
/// to the used planes' records, stores the total in `output + 0x20` and stops when it passes
/// the limit (`input + 0x44`), adds the hit plane to the solver's simplex and reduces it
/// (`00d23f50`). With no further plane it stores the whole budget in `output + 0x20`.
pub fn fn_00d24360(e: &mut Engine, input: Ptr, output: Ptr) {
    // The exe aligns ESP, then `SUB ESP,0x64` and three pushes: a 0x70-byte frame, the solver
    // state at +0x30 (its input and output words at +0x68 and +0x6c, 00d24394..00d2439c).
    with_aligned_frame(e, 0x70, |e, frame| {
        let solver = frame + 0x30;
        e.mem.set_u32(solver + 0x38, input.addr());
        e.mem.set_u32(solver + 0x3c, output.addr());
        search(e, input.addr(), output.addr(), solver);
    });
}

/// The floats and doubles the three functions read: `1.0` as a float (`01012068`) and as a double
/// (`01012070`), the float `-1.0` (`01012054`), the double `0.5` (`01011588`) and the float
/// `0.0` (`01011d78`).
const HEIGHT_ONE_SINGLE: u32 = 0x0101_2068;
const HEIGHT_ONE_DOUBLE: u32 = 0x0101_2070;
const HEIGHT_MINUS_ONE_SINGLE: u32 = 0x0101_2054;
const HEIGHT_HALF_DOUBLE: u32 = 0x0101_1588;
const HEIGHT_ZERO_SINGLE: u32 = 0x0101_1d78;

/// The height at `(x, z)` as the float the code stores: slot `0x34` of the shape's vtable, a float
/// in `ST0`, rounded by `FSTP dword`.
fn height_bits_00d24670(e: &mut Engine, shape: u32, x: u32, z: u32) -> u32 {
    x87_store32(e.vcall(shape, 0x34, &args![x, z]).f64())
}

// Translated from 00d24660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`__fastcall`, no stack words): writes the base vtable word `0x0102_e388` at `+0`
/// (the body of the destructors that end with that word).
pub fn fn_00d24660(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_e388);
}

// Translated from 00d24670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall on a height-field shape, `RET 0x1c`: seven stack words `x`, `z`, `fx`
/// (float), `fz` (float), `normal`, `height_out`, `triangle_out`; descriptions, not confirmed):
/// the height and normal of the point `(x + fx, z + fz)` inside the cell `(x, z)`. Heights come
/// from slot `0x34` of `this` (`(x, z)`, a float in `ST0`); slot `0x38` (with the address of a
/// byte) decides the diagonal. `normal` is first set to four times the float at `01012068`; then
/// its lanes 0 and 2 are `-dx` and `-dz` and `*height_out` is the height, all rounded to floats
/// with x87 arithmetic, and `*triangle_out` is 1 or 0 for the triangle used. With the byte set
/// (`a`, `d` the heights at `(x, z)` and `(x + 1, z + 1)`): for `fz < fx` the heights at
/// `(x + 1, z)` give `dx = h10 - a`, `dz = d - h10`, else those at `(x, z + 1)` give
/// `dx = d - h01`, `dz = h01 - a`; the height is `(a + dz * fz) + dx * fx`, triangle 1 or 0. With
/// the byte clear (`h10`, `h01` the heights at `(x + 1, z)` and `(x, z + 1)`): for `fx + fz > 1`
/// (double compare) `h11` at `(x + 1, z + 1)` gives `dx = h11 - h01`, `dz = h11 - h10` and the
/// height `(h10 + dz * fz) + (fx - 1) * dx`, triangle 1; otherwise `h00` at `(x, z)` gives
/// `dx = h10 - h00`, `dz = h01 - h00` and `(h00 + dz * fz) + dx * fx`, triangle 0.
pub fn fn_00d24670(
    e: &mut Engine,
    this: Ptr,
    x: u32,
    z: u32,
    fx: f32,
    fz: f32,
    normal: Ptr,
    height_out: Ptr,
    triangle_out: Ptr,
) {
    let shape = this.addr();
    let one = e.mem.u32(HEIGHT_ONE_SINGLE);
    for lane in 0..4 {
        e.mem.set_u32(normal.addr() + lane * 4, one);
    }
    let flag = e.with_stack(4, |e, byte| {
        e.vcall(shape, 0x38, &args![byte]);
        e.mem.u8(byte.addr())
    });
    let (fx_bits, fz_bits) = (fx.to_bits(), fz.to_bits());
    let fx_loaded = x87_load32(fx_bits);
    let fz_loaded = x87_load32(fz_bits);
    let next_x = x.wrapping_add(1);
    let next_z = z.wrapping_add(1);
    // The deltas (as stored floats), the height and the triangle.
    let (dx_bits, dz_bits, height, triangle);
    if flag != 0 {
        let base = height_bits_00d24670(e, shape, x, z);
        let far = height_bits_00d24670(e, shape, next_x, next_z);
        let (dx_value, dz_value, flag_value);
        if fz_loaded < fx_loaded {
            let h10 = height_bits_00d24670(e, shape, next_x, z);
            dx_value = x87_sub(x87_load32(h10), x87_load32(base));
            dz_value = x87_sub(x87_load32(far), x87_load32(h10));
            flag_value = 1u32;
        } else {
            let h01 = height_bits_00d24670(e, shape, x, next_z);
            dx_value = x87_sub(x87_load32(far), x87_load32(h01));
            dz_value = x87_sub(x87_load32(h01), x87_load32(base));
            flag_value = 0;
        }
        dx_bits = x87_store32(dx_value);
        dz_bits = x87_store32(dz_value);
        let dz_loaded = x87_load32(dz_bits);
        let dx_loaded = x87_load32(dx_bits);
        let along_z = x87_add(x87_load32(base), x87_mul(dz_loaded, x87_m32(fz_bits)));
        height = x87_store32(x87_add(along_z, x87_mul(dx_loaded, x87_m32(fx_bits))));
        triangle = flag_value;
    } else {
        let h10 = height_bits_00d24670(e, shape, next_x, z);
        let h01 = height_bits_00d24670(e, shape, x, next_z);
        let sum = x87_add(fx_loaded, x87_m32(fz_bits));
        let one_double = e.mem.f64(HEIGHT_ONE_DOUBLE);
        if sum > one_double {
            let h11 = height_bits_00d24670(e, shape, next_x, next_z);
            dx_bits = x87_store32(x87_sub(x87_load32(h11), x87_m32(h01)));
            dz_bits = x87_store32(x87_sub(x87_load32(h11), x87_load32(h10)));
            let dz_loaded = x87_load32(dz_bits);
            let dx_loaded = x87_load32(dx_bits);
            let along_z = x87_add(x87_load32(h10), x87_mul(dz_loaded, x87_m32(fz_bits)));
            let fx_less_one = x87_sub(fx_loaded, one_double);
            height = x87_store32(x87_add(along_z, x87_mul(fx_less_one, dx_loaded)));
            triangle = 1;
        } else {
            let h00 = height_bits_00d24670(e, shape, x, z);
            dx_bits = x87_store32(x87_sub(x87_load32(h10), x87_load32(h00)));
            dz_bits = x87_store32(x87_sub(x87_load32(h01), x87_load32(h00)));
            let dz_loaded = x87_load32(dz_bits);
            let dx_loaded = x87_load32(dx_bits);
            let along_z = x87_add(x87_load32(h00), x87_mul(dz_loaded, x87_m32(fz_bits)));
            height = x87_store32(x87_add(along_z, x87_mul(dx_loaded, x87_m32(fx_bits))));
            triangle = 0;
        }
    }
    e.mem.set_u32(height_out.addr(), height);
    e.mem
        .set_u32(normal.addr(), x87_store32(-x87_load32(dx_bits)));
    e.mem
        .set_u32(normal.addr() + 8, x87_store32(-x87_load32(dz_bits)));
    e.mem.set_u32(triangle_out.addr(), triangle);
}

// Translated from 00d248a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a sampled height-field shape (class unconfirmed; thiscall, `RET 8`: `info` and a
/// byte `flag`; no map name): writes the vtable word `0x010c_e31c`, the `u16` at `+6` = 1, `+8` =
/// 0, `+0xc` = 0xd, `+0x1d` = `flag`, copies `info + 0x10`, `+0x14` to `+0x10`, `+0x14` and the byte
/// `info + 0x20` to `+0x1c`. With `X`, `Z` the integers at `info + 0x10` and `+0x14`, `Q` and `P`
/// the floats at `info + 0x18` and `+0x1c` and `s` the vector at `info + 0`: `+0x50` is
/// `(X - 1, P - Q, Z - 1, 0)` (each rounded to a float, the last the float at `01011d78`) times
/// `s`; `+0x18` is `-1.0` (and `+0x54`, replacing the second lane above, too) when `P < Q`, else
/// `((P + Q) * 0.5) * s.y`; `+0x20` is `s`; `+0x30` is `(1/s.x, 1/s.y, 1/s.z, 0)`; and `+0x40` is
/// `r * s` for `r` the answer of `00d97440` (no arguments, a float in `ST0`) with lane 1 set to
/// 0.0. Returns `this`.
pub fn fn_00d248a0(e: &mut Engine, this: Ptr, info: Ptr, flag: u8) -> Ptr {
    let me = this.addr();
    let at = info.addr();
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 0xd);
    e.mem.set_u32(me, 0x010c_e31c);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u8(me + 0x1d, flag);
    let width = e.mem.u32(at + 0x10);
    e.mem.set_u32(me + 0x10, width);
    let depth = e.mem.u32(at + 0x14);
    e.mem.set_u32(me + 0x14, depth);
    let corner = e.mem.u8(at + 0x20);
    e.mem.set_u8(me + 0x1c, corner);
    let zero_bits = e.mem.u32(HEIGHT_ZERO_SINGLE);
    let width_less_one = x87_store32(x87_sub(f64::from(e.mem.i32(at + 0x10)), 1.0));
    let q_bits = e.mem.u32(at + 0x18);
    let span = x87_store32(x87_sub(x87_load32(e.mem.u32(at + 0x1c)), x87_m32(q_bits)));
    let depth_less_one = x87_store32(x87_sub(f64::from(e.mem.i32(at + 0x14)), 1.0));
    let scale_lanes = [0u32, 1, 2, 3].map(|lane| e.mem.f32(at + lane * 4));
    let extent = [width_less_one, span, depth_less_one, zero_bits].map(f32::from_bits);
    for lane in 0..4 {
        e.mem.set_f32(
            me + 0x50 + lane * 4,
            // `MULPS` with the extent as the destination (the SSE NaN rules).
            crate::sse::mul(extent[lane as usize], scale_lanes[lane as usize]),
        );
    }
    let p_loaded = x87_load32(e.mem.u32(at + 0x1c));
    let q_loaded = x87_load32(q_bits);
    if p_loaded < q_loaded {
        let minus_one = x87_store32(x87_load32(e.mem.u32(HEIGHT_MINUS_ONE_SINGLE)));
        e.mem.set_u32(me + 0x18, minus_one);
        e.mem.set_u32(me + 0x54, minus_one);
    } else {
        let sum = x87_add(x87_load32(e.mem.u32(at + 0x1c)), x87_m32(q_bits));
        let half = e.mem.f64(HEIGHT_HALF_DOUBLE);
        let scaled = x87_mul(x87_mul(sum, half), x87_m32(e.mem.u32(at + 4)));
        e.mem.set_u32(me + 0x18, x87_store32(scaled));
    }
    for lane in 0..4 {
        let bits = e.mem.u32(at + lane * 4);
        e.mem.set_u32(me + 0x20 + lane * 4, bits);
    }
    let one = 1.0f64;
    let reciprocal_x = x87_store32(x87_div(one, x87_load32(e.mem.u32(at))));
    let reciprocal_y = x87_store32(x87_div(one, x87_load32(e.mem.u32(at + 4))));
    let reciprocal_z = x87_store32(x87_div(one, x87_m32(e.mem.u32(at + 8))));
    for (lane, bits) in [reciprocal_x, reciprocal_y, reciprocal_z, zero_bits]
        .into_iter()
        .enumerate()
    {
        e.mem.set_u32(me + 0x30 + lane as u32 * 4, bits);
    }
    let factor = f32::from_bits(x87_store32(e.call(0x00d9_7440, &args![]).f64()));
    let kept = [0u32, 1, 2, 3].map(|lane| e.mem.f32(me + 0x20 + lane * 4));
    for lane in 0..4 {
        let value = if lane == 1 {
            0.0
        } else {
            crate::sse::mul(factor, kept[lane as usize])
        };
        e.mem.set_f32(me + 0x40 + lane * 4, value);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_31b0, fn_00d231b0(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00d2_3380, fn_00d23380(Ptr)),
        entry!(0x00d2_3480, fn_00d23480(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00d2_34e0, fn_00d234e0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d2_3850, fn_00d23850(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d2_3ca0, fn_00d23ca0(Ptr, Ptr, Ptr, Ptr, u32, Ptr, Ptr)),
        entry!(0x00d2_3f50, fn_00d23f50(Ptr)),
        entry!(0x00d2_4360, fn_00d24360(Ptr, Ptr)),
        entry!(0x00d2_4660, fn_00d24660(Ptr)),
        entry!(
            0x00d2_4670,
            fn_00d24670(Ptr, u32, u32, f32, f32, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d2_48a0, fn_00d248a0(Ptr, Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(EPSILON, f32::from_bits(0x3400_0000));
        e.mem.set_f32(PLANE_TOLERANCE, -0.001);
        e.mem.set_f64(LINE_EPSILON, 2.0_f64.powi(-20));
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(ONE, 1.0);
        e.mem.set_f64(ONE_THOUSANDTH, f64::from(0.001_f32));
        e.mem.set_f64(MINIMUM_ADVANCE, f64::from(0.0001_f32));
        e.mem.set_f64(MARGIN_STEP, 2.0_f64.powi(-23));
        e.register_double(SQUARE_ROOT, |_, a| {
            let bits = u64::from(a[0]) | (u64::from(a[1]) << 32);
            Ret {
                st0: f64::from_bits(bits).sqrt(),
                ..Ret::default()
            }
        });
        e
    }

    fn vec_at(e: &mut Engine, v: Vec4) -> u32 {
        let addr = e.mem.alloc(16);
        store_vec(e, addr, v);
        addr
    }

    #[test]
    fn closest_points_of_two_crossing_lines() {
        let mut e = engine();
        let from = vec_at(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let direction = vec_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let other_from = vec_at(&mut e, [2.0, 1.0, 0.0, 0.0]);
        let other_direction = vec_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let out = e.mem.alloc(0x30);
        let words = args![from, direction, other_from, other_direction, out];
        assert!(!e.call(0x00d2_31b0, &words).bool());
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 0.0);
        assert_eq!(load_vec(&e, out + 0x10), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0, 1.0, 0.0, 0.0]);
        // The same direction twice: parallel, `s` stays 0 and `t` is the offset along it.
        let words = args![from, direction, other_from, direction, out];
        assert!(e.call(0x00d2_31b0, &words).bool());
        assert_eq!(e.mem.f32(out + 4), 0.0);
        assert_eq!(e.mem.f32(out + 8), -2.0);
    }

    /// A solver block with one entry per plane, the planes having the given sort keys and +0x10
    /// vectors. Entry `n` holds the words (10 + n, plane, 20 + n).
    fn sort_fixture(e: &mut Engine, planes: &[(i32, Vec4)]) -> u32 {
        let solver = e.mem.alloc(0x70);
        e.mem.set_u32(solver + 0x30, planes.len() as u32);
        for (index, (key, vector)) in planes.iter().enumerate() {
            let plane = e.mem.alloc(0x40);
            e.mem.set_i32(plane + 0x30, *key);
            store_vec(e, plane + 0x10, *vector);
            let entry = solver + 12 * index as u32;
            e.mem.set_u32(entry, 10 + index as u32);
            e.mem.set_u32(entry + 4, plane);
            e.mem.set_u32(entry + 8, 20 + index as u32);
        }
        solver
    }

    #[test]
    fn entries_are_sorted_by_key_then_by_length() {
        let mut e = engine();
        let solver = sort_fixture(
            &mut e,
            &[
                (2, [1.0, 0.0, 0.0, 0.0]),
                (1, [2.0, 0.0, 0.0, 0.0]),
                (1, [1.0, 0.0, 0.0, 0.0]),
            ],
        );
        let planes: Vec<u32> = (0..3).map(|n| entry_word(&e, solver, n, 1)).collect();
        e.call(0x00d2_3380, &args![solver]);
        let sorted: Vec<[u32; 3]> = (0..3)
            .map(|n| std::array::from_fn(|word| entry_word(&e, solver, n, word as u32)))
            .collect();
        assert_eq!(
            sorted,
            vec![
                [12, planes[2], 22],
                [11, planes[1], 21],
                [10, planes[0], 20]
            ]
        );
        // A single entry is left alone.
        let single = sort_fixture(&mut e, &[(5, [0.0; 4])]);
        e.call(0x00d2_3380, &args![single]);
        assert_eq!(entry_word(&e, single, 0, 0), 10);
    }

    #[test]
    fn point_behind_a_plane_is_flagged() {
        let mut e = engine();
        let plane = e.mem.alloc(0x40);
        store_vec(&mut e, plane, [0.0, 0.0, 1.0, 0.0]);
        let behind = vec_at(&mut e, [0.0, 0.0, -1.0, 0.0]);
        let ahead = vec_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let result = e.mem.alloc(8);
        let returned = e.call(0x00d2_3480, &args![result, plane, behind]).u32();
        assert_eq!(returned, result);
        assert_eq!(e.mem.u8(result), 1);
        e.call(0x00d2_3480, &args![result, plane, ahead]);
        assert_eq!(e.mem.u8(result), 0);
        // Just inside the tolerance of -0.001 counts as in front.
        let close = vec_at(&mut e, [0.0, 0.0, -0.0005, 0.0]);
        e.call(0x00d2_3480, &args![result, plane, close]);
        assert_eq!(e.mem.u8(result), 0);
        let nan = vec_at(&mut e, [0.0, 0.0, f32::NAN, 0.0]);
        e.call(0x00d2_3480, &args![result, plane, nan]);
        assert_eq!(e.mem.u8(result), 1);
    }

    /// A shape at (1, 2, 3) with the axis z, a context whose input has a direction at +0x30, and
    /// the query point (5, 2, 6).
    struct Projection {
        shape: u32,
        context: u32,
        point: u32,
        out: u32,
    }

    fn projection(
        e: &mut Engine,
        radius: f32,
        margins: [f32; 2],
        blend: f32,
        side: Vec4,
    ) -> Projection {
        let shape = e.mem.alloc(0x40);
        store_vec(e, shape, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, shape + 0x10, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_f32(shape + 0x20, radius);
        e.mem.set_f32(shape + 0x24, margins[0]);
        e.mem.set_f32(shape + 0x28, margins[1]);
        e.mem.set_f32(shape + 0x2c, blend);
        let input = e.mem.alloc(0x80);
        store_vec(e, input + 0x30, side);
        let context = e.mem.alloc(0x70);
        e.mem.set_u32(context + 0x38, input);
        let point = vec_at(e, [5.0, 2.0, 6.0, 0.0]);
        let out = e.mem.alloc(16);
        Projection {
            shape,
            context,
            point,
            out,
        }
    }

    fn project(e: &mut Engine, p: &Projection) -> Vec4 {
        e.call(0x00d2_34e0, &args![p.shape, p.point, p.out, p.context]);
        load_vec(e, p.out)
    }

    #[test]
    fn projection_within_reach_is_the_origin() {
        let mut e = engine();
        // Radius 10: (10 * 10 + 1) * 9 exceeds the squared distance 25.
        let p = projection(&mut e, 10.0, [0.0, 0.0], 0.0, [0.0; 4]);
        assert_eq!(project(&mut e, &p), [1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn projection_outside_is_blended_back() {
        let mut e = engine();
        // Offset (4, 0, 3): perpendicular part (4, 0, 0) of squared length 16 against 25 in all,
        // so the weight is 0 + 1 * sqrt(25 / 16) = 1.25.
        let p = projection(&mut e, 0.0, [0.0, 0.0], 0.0, [0.0; 4]);
        assert_eq!(project(&mut e, &p), [6.0, 2.0, 3.0, 0.0]);
        // A blend weight of 1 keeps the perpendicular part as it is.
        let p = projection(&mut e, 0.0, [0.0, 0.0], 1.0, [0.0; 4]);
        assert_eq!(project(&mut e, &p), [5.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn projection_with_a_positive_margin_checks_the_side_direction() {
        let mut e = engine();
        // The direction (1, 0, 0) faces the perpendicular part, so the margin 0.5 at +0x24
        // applies; the direction's cross product with the axis is orthogonal to the offset.
        let p = projection(&mut e, 0.0, [0.5, 0.0], 0.0, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(project(&mut e, &p), [6.0, 2.0, 3.0, 0.0]);
        // A large margin makes the reach wide enough that the origin is the answer.
        let p = projection(&mut e, 0.0, [5.0, 0.0], 0.0, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(project(&mut e, &p), [1.0, 2.0, 3.0, 0.0]);
    }

    /// A solver block over an input with four planes and an output with four records.
    struct World {
        context: u32,
        input: u32,
        output: u32,
        planes: u32,
        records: u32,
    }

    fn world(e: &mut Engine) -> World {
        let context = e.mem.alloc(0x70);
        let input = e.mem.alloc(0x80);
        let output = e.mem.alloc(0x40);
        let planes = e.mem.alloc(0x40 * 4);
        let records = e.mem.alloc(0x10 * 4);
        e.mem.set_u32(context + 0x38, input);
        e.mem.set_u32(context + 0x3c, output);
        e.mem.set_u32(input + 0x48, planes);
        e.mem.set_u32(output + 0x24, records);
        store_vec(e, input + 0x20, [100.0; 4]);
        World {
            context,
            input,
            output,
            planes,
            records,
        }
    }

    fn set_plane(e: &mut Engine, w: &World, index: u32, vectors: [Vec4; 2], scalars: [f32; 4]) {
        let plane = w.planes + 0x40 * index;
        for (n, vector) in vectors.into_iter().enumerate() {
            store_vec(e, plane + 0x10 * n as u32, vector);
        }
        for (n, scalar) in scalars.into_iter().enumerate() {
            e.mem.set_f32(plane + 0x20 + 4 * n as u32, scalar);
        }
    }

    /// Planes x = 2 (index 0) and y = 3 (index 1), whose line runs along z.
    fn two_planes(e: &mut Engine) -> World {
        let w = world(e);
        let scalars = [0.0, 1.0, 0.0, 1.0];
        set_plane(
            e,
            &w,
            0,
            [[1.0, 0.0, 0.0, 0.0], [2.0, 0.0, 0.0, 0.0]],
            scalars,
        );
        set_plane(
            e,
            &w,
            1,
            [[0.0, 1.0, 0.0, 0.0], [0.0, 3.0, 0.0, 0.0]],
            scalars,
        );
        store_vec(e, w.input + 0x30, [0.0, 0.0, 1.0, 0.0]);
        w
    }

    fn state(e: &Engine, w: &World, index: u32) -> u32 {
        e.mem.u32(w.records + 0x10 * index + 0xc)
    }

    #[test]
    fn two_planes_meet_in_a_line_and_the_point_is_moved_along_it() {
        let mut e = engine();
        let w = two_planes(&mut e);
        let point = vec_at(&mut e, [2.0, 3.0, 5.0, 0.0]);
        let out = e.mem.alloc(16);
        let (first, second) = (w.planes, w.planes + 0x40);
        e.call(0x00d2_3850, &args![w.context, first, second, point, out]);
        assert_eq!(load_vec(&e, out), [2.0, 3.0, 5.0, 0.0]);
        // A large offset to the middle makes the result stay on the line's point.
        e.mem.set_f32(first + 0x20, 10.0);
        e.mem.set_f32(second + 0x20, 10.0);
        let point = vec_at(&mut e, [2.0, 5.0, 5.0, 0.0]);
        e.call(0x00d2_3850, &args![w.context, first, second, point, out]);
        assert_eq!(load_vec(&e, out), [2.0, 3.0, 0.0, 0.0]);
        assert_eq!(state(&e, &w, 0), 0);
    }

    #[test]
    fn two_parallel_planes_fall_back_to_projections() {
        let mut e = engine();
        let w = two_planes(&mut e);
        let (first, second) = (w.planes, w.planes + 0x40);
        // Make the second plane parallel to the first.
        store_vec(&mut e, second, [1.0, 0.0, 0.0, 0.0]);
        let point = vec_at(&mut e, [2.0, 3.0, 5.0, 0.0]);
        let out = e.mem.alloc(16);
        e.call(0x00d2_3850, &args![w.context, first, second, point, out]);
        assert_eq!(state(&e, &w, 0), 2);
        assert_eq!(state(&e, &w, 1), 2);
        assert_eq!(state(&e, &w, 2), 0);
    }

    /// Three orthogonal planes with the given offsets along their normals.
    fn three_planes(e: &mut Engine, offsets: [f32; 3]) -> World {
        let w = world(e);
        for (n, offset) in offsets.into_iter().enumerate() {
            let mut normal = [0.0; 4];
            normal[n] = 1.0;
            let mut along = [0.0; 4];
            along[n] = offset;
            set_plane(e, &w, n as u32, [normal, along], [0.0; 4]);
        }
        w
    }

    #[test]
    fn three_planes_meet_in_a_point() {
        let mut e = engine();
        let w = three_planes(&mut e, [7.0, 8.0, 9.0]);
        let out = e.mem.alloc(16);
        let point = vec_at(&mut e, [0.0; 4]);
        let planes = [w.planes, w.planes + 0x40, w.planes + 0x80];
        let words = args![w.context, planes[0], planes[1], planes[2], 0u32, point, out];
        e.call(0x00d2_3ca0, &words);
        assert_eq!(load_vec(&e, out), [7.0, 8.0, 9.0, 0.0]);
        assert_eq!(state(&e, &w, 0), 0);
    }

    #[test]
    fn three_planes_out_of_tolerance_are_marked_and_solved_in_pairs() {
        let mut e = engine();
        let w = three_planes(&mut e, [7.0, 8.0, 9.0]);
        store_vec(&mut e, w.input + 0x20, [1.0; 4]);
        let out = e.mem.alloc(16);
        let point = vec_at(&mut e, [0.0; 4]);
        let planes = [w.planes, w.planes + 0x40, w.planes + 0x80];
        let words = args![w.context, planes[0], planes[1], planes[2], 0u32, point, out];
        e.call(0x00d2_3ca0, &words);
        // Each pair marks its planes as 2 on top of the 1 for the triple.
        let states = [state(&e, &w, 0), state(&e, &w, 1), state(&e, &w, 2)];
        assert_eq!(states, [2, 2, 2]);
        assert_eq!(state(&e, &w, 3), 0);
    }

    #[test]
    fn simplex_of_one_projects_onto_its_plane() {
        let mut e = engine();
        let p = projection(&mut e, 0.0, [0.0, 0.0], 0.0, [0.0; 4]);
        let w = world(&mut e);
        // The solver's input holds the query point; the output gets the result point.
        store_vec(&mut e, w.input + 0x10, [5.0, 2.0, 6.0, 0.0]);
        e.mem.set_u32(w.context + 0x30, 1);
        e.mem.set_u32(w.context + 4, p.shape);
        e.call(0x00d2_3f50, &args![w.context]);
        assert_eq!(load_vec(&e, w.output + 0x10), [6.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn simplex_of_two_keeps_the_second_when_the_point_is_not_behind_the_first() {
        let mut e = engine();
        let p = projection(&mut e, 0.0, [0.0, 0.0], 0.0, [0.0; 4]);
        let w = world(&mut e);
        store_vec(&mut e, w.input + 0x10, [5.0, 2.0, 6.0, 0.0]);
        // The first plane faces +z through the origin: the projected point (6, 2, 3) is in front.
        let first = w.planes;
        store_vec(&mut e, first, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_u32(w.context + 0x30, 2);
        e.mem.set_u32(w.context, 7);
        e.mem.set_u32(w.context + 4, first);
        e.mem.set_u32(w.context + 12, 8);
        e.mem.set_u32(w.context + 16, p.shape);
        e.mem.set_u32(w.context + 20, 9);
        e.call(0x00d2_3f50, &args![w.context]);
        assert_eq!(load_vec(&e, w.output + 0x10), [6.0, 2.0, 3.0, 0.0]);
        assert_eq!(e.mem.u32(w.context + 0x30), 1);
        let kept: [u32; 3] = std::array::from_fn(|word| entry_word(&e, w.context, 0, word as u32));
        assert_eq!(kept, [8, p.shape, 9]);
        // With no entries the call does nothing.
        e.mem.set_u32(w.context + 0x30, 0);
        e.call(0x00d2_3f50, &args![w.context]);
        assert_eq!(e.mem.u32(w.context + 0x30), 0);
    }

    fn search_fixture(e: &mut Engine, budget: f32) -> (u32, u32) {
        let input = e.mem.alloc(0x80);
        store_vec(e, input, [1.0, 2.0, 3.0, 0.0]);
        store_vec(e, input + 0x10, [0.5, 0.0, 0.0, 0.0]);
        e.mem.set_f32(input + 0x40, budget);
        e.mem.set_f32(input + 0x44, 10.0);
        let output = e.mem.alloc(0x40);
        (input, output)
    }

    #[test]
    fn the_search_state_sits_at_0x30_of_the_aligned_frame() {
        let mut e = engine();
        let (input, output) = search_fixture(&mut e, 1.0);
        let raw = e.next_stack_local(0x80);
        e.call(0x00d2_4360, &args![input, output]);
        let frame = aligned_frame_at(raw, 0x70);
        assert_eq!(e.mem.u32(frame + 0x68), input);
        assert_eq!(e.mem.u32(frame + 0x6c), output);
    }

    #[test]
    fn search_without_planes_moves_by_the_whole_budget() {
        let mut e = engine();
        let (input, output) = search_fixture(&mut e, 1.0);
        e.call(0x00d2_4360, &args![input, output]);
        assert_eq!(load_vec(&e, output), [1.5, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, output + 0x10), [0.5, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(output + 0x20), 1.0);
    }

    #[test]
    fn search_with_a_negative_budget_only_copies_and_clears() {
        let mut e = engine();
        let (input, output) = search_fixture(&mut e, -1.0);
        let records = e.mem.alloc(0x20);
        e.mem.set_u32(input + 0x4c, 2);
        e.mem.set_u32(output + 0x24, records);
        for n in 0..8 {
            e.mem.set_u32(records + 4 * n, 0x0101_0101);
        }
        e.call(0x00d2_4360, &args![input, output]);
        assert_eq!(load_vec(&e, output), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, output + 0x10), [0.5, 0.0, 0.0, 0.0]);
        // Per record: the two flag bytes, the time, the margin and the state word are cleared.
        for record in [records, records + 0x10] {
            assert_eq!(e.mem.u8(record), 0);
            assert_eq!(e.mem.u8(record + 1), 0);
            assert_eq!(e.mem.u8(record + 2), 1);
            assert_eq!(e.mem.f32(record + 4), 0.0);
            assert_eq!(e.mem.f32(record + 8), 0.0);
            assert_eq!(e.mem.u32(record + 0xc), 0);
        }
        assert_eq!(e.mem.f32(output + 0x20), 0.0);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d24660_writes_the_base_vtable_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x1234);
        e.call(0x00d2_4660, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 4), 0x1234);
    }

    /// A shape whose height at `(x, z)` is `x * x + 3 * z` (slot `0x34`) and whose slot `0x38`
    /// writes `flag` to the byte it is given.
    fn shape(e: &mut Engine, flag: u8) -> u32 {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(HEIGHT_ONE_SINGLE, 0x3f80_0000);
        e.mem.set_f64(HEIGHT_ONE_DOUBLE, 1.0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x34, 0x0900_00b1);
        e.mem.set_u32(vtable + 0x38, 0x0900_00b2);
        e.register(0x0900_00b1, |_, a| Ret {
            st0: f64::from(a[1] * a[1] + 3 * a[2]),
            ..Ret::default()
        });
        e.register_double(0x0900_00b2, move |e, a| {
            e.mem.set_u8(a[1], flag);
            Ret::default()
        });
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape, vtable);
        shape
    }

    /// `(height, triangle, normal)` for the cell `(2, 3)` and the offsets `(fx, fz)`.
    fn query(flag: u8, fx: f32, fz: f32) -> (f32, u32, [f32; 4]) {
        let mut e = Engine::new();
        let this = shape(&mut e, flag);
        let normal = e.mem.alloc(0x10);
        let outputs = e.mem.alloc(0x10);
        e.call(
            0x00d2_4670,
            &args![this, 2u32, 3u32, fx, fz, normal, outputs, outputs + 4],
        );
        (
            e.mem.f32(outputs),
            e.mem.u32(outputs + 4),
            [0u32, 1, 2, 3].map(|lane| e.mem.f32(normal + lane * 4)),
        )
    }

    #[test]
    fn fn_00d24670_with_the_flag_set_picks_the_triangle_by_the_offsets() {
        // H(2,3) = 13, H(3,3) = 18, H(2,4) = 16, H(3,4) = 21.
        // fz < fx: dx = 18 - 13 = 5, dz = 21 - 18 = 3: 13 + 3 * 0.25 + 5 * 0.75 = 17.5.
        assert_eq!(query(1, 0.75, 0.25), (17.5, 1, [-5.0, 1.0, -3.0, 1.0]));
        // fz >= fx: dx = 21 - 16 = 5, dz = 16 - 13 = 3: 13 + 3 * 0.75 + 5 * 0.25 = 16.5.
        assert_eq!(query(1, 0.25, 0.75), (16.5, 0, [-5.0, 1.0, -3.0, 1.0]));
        // Equal offsets are not `fz < fx`.
        assert_eq!(query(1, 0.5, 0.5).1, 0);
    }

    #[test]
    fn fn_00d24670_with_the_flag_clear_picks_the_triangle_by_the_sum() {
        // fx + fz <= 1: dx = 18 - 13 = 5, dz = 16 - 13 = 3: 13 + 3 * 0.25 + 5 * 0.25 = 15.
        assert_eq!(query(0, 0.25, 0.25), (15.0, 0, [-5.0, 1.0, -3.0, 1.0]));
        // fx + fz > 1: dx = 21 - 16 = 5, dz = 21 - 18 = 3: 18 + 3 * 0.5 + (0.75 - 1) * 5 = 18.25.
        assert_eq!(query(0, 0.75, 0.5), (18.25, 1, [-5.0, 1.0, -3.0, 1.0]));
        // A sum of exactly 1 is not above 1.
        assert_eq!(query(0, 0.5, 0.5).1, 0);
    }

    #[test]
    fn fn_00d24670_a_nan_offset_takes_the_other_branches() {
        // With a NaN `fz` neither `fz < fx` nor `fx + fz > 1` holds.
        assert_eq!(query(1, 0.5, f32::NAN).1, 0);
        assert_eq!(query(0, 0.5, f32::NAN).1, 0);
    }

    #[test]
    fn fn_00d248a0_builds_the_height_field_constants() {
        for (q, p, minus_one) in [(1.0f32, 3.0f32, false), (3.0, 1.0, true)] {
            let mut e = Engine::new();
            e.map(0x0100_0000, 0x0030_0000);
            e.mem.set_u32(HEIGHT_ZERO_SINGLE, 0);
            e.mem.set_u32(HEIGHT_MINUS_ONE_SINGLE, 0xbf80_0000);
            e.mem.set_f64(HEIGHT_HALF_DOUBLE, 0.5);
            e.register(0x00d9_7440, |_, _| Ret {
                st0: 0.5,
                ..Ret::default()
            });
            let info = e.mem.alloc(0x30);
            for (lane, value) in [2.0f32, 4.0, 8.0, 16.0].into_iter().enumerate() {
                e.mem.set_f32(info + lane as u32 * 4, value);
            }
            e.mem.set_i32(info + 0x10, 5);
            e.mem.set_i32(info + 0x14, 4);
            e.mem.set_f32(info + 0x18, q);
            e.mem.set_f32(info + 0x1c, p);
            e.mem.set_u8(info + 0x20, 0x9b);
            let this = e.mem.alloc(0x60);
            let result = e.call(0x00d2_48a0, &args![this, info, 0x5au32]).u32();
            assert_eq!(result, this);
            assert_eq!(e.mem.u32(this), 0x010c_e31c);
            assert_eq!(e.mem.u16(this + 6), 1);
            assert_eq!(e.mem.u32(this + 8), 0);
            assert_eq!(e.mem.u32(this + 0xc), 0xd);
            assert_eq!(e.mem.u32(this + 0x10), 5);
            assert_eq!(e.mem.u32(this + 0x14), 4);
            assert_eq!(e.mem.u8(this + 0x1c), 0x9b);
            assert_eq!(e.mem.u8(this + 0x1d), 0x5a);
            let lanes = |e: &Engine, at: u32| [0u32, 1, 2, 3].map(|lane| e.mem.f32(at + lane * 4));
            // (X - 1, P - Q, Z - 1, 0) * (2, 4, 8, 16).
            let span = (p - q) * 4.0;
            let expected = if minus_one {
                [8.0, -1.0, 24.0, 0.0]
            } else {
                [8.0, span, 24.0, 0.0]
            };
            assert_eq!(lanes(&e, this + 0x50), expected);
            assert_eq!(e.mem.f32(this + 0x18), if minus_one { -1.0 } else { 8.0 });
            assert_eq!(lanes(&e, this + 0x20), [2.0, 4.0, 8.0, 16.0]);
            assert_eq!(lanes(&e, this + 0x30), [0.5, 0.25, 0.125, 0.0]);
            assert_eq!(lanes(&e, this + 0x40), [1.0, 0.0, 4.0, 8.0]);
        }
    }
}
