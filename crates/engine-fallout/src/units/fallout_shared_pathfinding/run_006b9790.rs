//! `fallout shared/pathfinding/run_006b9790` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of geometry helpers the engine map does not name (all `cdecl`
//! except the 2D dot product): barycentric coordinates of a point in a
//! triangle, distances from a point to a line or segment, closest points on
//! a segment, a ray-plane test, the height of a point over a triangle, and
//! 2D segment intersection tests. Points are `NiPoint3`s (three floats) or
//! 2D points (two floats); the vector arithmetic is called by address.
//!
//! x87 note: the code computes in extended precision and stores `float`s;
//! the translations compute in `f64` and round to `f32` at each `float`
//! store. Comparisons follow the exe's flag tests (a NaN takes the branch
//! the exe's unordered result takes).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00452dc0(point, x, y)`: sets a 2D point from two floats, returns it.
const POINT2_SET: u32 = 0x0045_2dc0;
/// The double `0.0`.
const ZERO: u32 = 0x0101_2060;
/// The double `1.0`.
const ONE: u32 = 0x0101_2070;
/// The `float` `1e-7` [`fn_006b9a10`] compares the denominator's magnitude
/// with.
const PLANE_EPSILON: u32 = 0x0106_c288;
/// The double `0.001` [`fn_006b9e90`] compares its sum of areas with.
const AREA_EPSILON: u32 = 0x0101_6978;
/// `00439ef0(point, out, other)`: `out = point - other` (`NiPoint3`),
/// returns `out`.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `00439e90(point, out, other)`: `out = point + other`, returns `out`.
const VECTOR_ADD: u32 = 0x0043_9e90;
/// `004a3760(out, scalar, point)` (cdecl): `out = scalar * point`, returns
/// `out`.
const VECTOR_SCALE: u32 = 0x004a_3760;
/// `NiPoint3::Dot` (`004b6190`, Xbox PDB name in the map; thiscall
/// `(point, other)`), result in ST0.
const DOT: u32 = 0x004b_6190;
/// `00457990(point)`: the length of the vector, in ST0.
const LENGTH: u32 = 0x0045_7990;
/// `004019b0(value)` (cdecl, `float`): forwards to `004019d0`, which
/// returns the CRT function `00ec6054` of the value (the root `00457990`
/// takes of a squared length), in ST0.
const ROOT: u32 = 0x0040_19b0;
/// `00408840(value)` (cdecl, `float`): `fabs`, in ST0.
const FABS: u32 = 0x0040_8840;
/// `004e8880(point, out, other)`: 2D `out = point - other`, returns `out`.
const POINT2_SUBTRACT: u32 = 0x004e_8880;
/// `00532fd0(out, scalar, point)` (cdecl): 2D `out = scalar * point`,
/// returns `out`.
const POINT2_SCALE: u32 = 0x0053_2fd0;
/// `004dfe30(point, out, other)`: 2D `out = point + other`, returns `out`.
const POINT2_ADD: u32 = 0x004d_fe30;

/// The x and y of the point at `p`.
fn xy(e: &Engine, p: u32) -> (f64, f64) {
    (
        f64::from(e.mem.f32(p)),
        f64::from(e.mem.f32(p.wrapping_add(4))),
    )
}

/// `(a - o) x (b - o)` in the plane: `(a.x - o.x) * (b.y - o.y) - (b.x -
/// o.x) * (a.y - o.y)`, in extended precision (here `f64`).
fn cross(e: &Engine, o: u32, a: u32, b: u32) -> f64 {
    let (ox, oy) = xy(e, o);
    let (ax, ay) = xy(e, a);
    let (bx, by) = xy(e, b);
    (ax - ox) * (by - oy) - (bx - ox) * (ay - oy)
}

/// [`cross`] as the x87 runs it: `FLD a.x`, `FSUB o.x`, `FLD b.y`, `FSUB o.y`,
/// `FMULP`, the same for the second product, `FSUBP` (each `FSUB` takes the
/// float in memory, so a signalling NaN there meets the x87 NaN rule as one).
fn cross_x87(e: &Engine, o: u32, a: u32, b: u32) -> f64 {
    let m = |p: u32| x87_m32(e.mem.u32(p));
    let l = |p: u32| x87_load32(e.mem.u32(p));
    let first = x87_mul(
        x87_sub(l(a), m(o)),
        x87_sub(l(b.wrapping_add(4)), m(o.wrapping_add(4))),
    );
    let second = x87_mul(
        x87_sub(l(b), m(o)),
        x87_sub(l(a.wrapping_add(4)), m(o.wrapping_add(4))),
    );
    x87_sub(first, second)
}

/// Copies `words` words from `source` to `dest`.
fn copy_words(e: &mut Engine, dest: u32, source: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

/// `NiPoint3::Dot(a, b)` in ST0.
fn dot(e: &mut Engine, a: u32, b: u32) -> f64 {
    e.call(DOT, &args![a, b]).f64()
}

// Translated from 006b9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the x and y of `p4`, `p3`, `p2` and
/// `p1` (in that order, through x87) into local 2D points (`00452dc0`) and
/// returns [`fn_006b9820`] on them. The exe declares no result; `AL` is
/// left as the callee's.
pub fn fn_006b9790(e: &mut Engine, p1: Ptr, p2: Ptr, p3: Ptr, p4: Ptr, out: Ptr) -> bool {
    e.with_stack(0x20, |e, stack| {
        let mut locals = [0u32; 4];
        for (slot, point) in [(3usize, p4), (2, p3), (1, p2), (0, p1)] {
            let p = point.addr();
            let x = x87_quiet(e.mem.f32(p));
            let y = x87_quiet(e.mem.f32(p.wrapping_add(4)));
            let local = stack.addr() + 8 * slot as u32;
            locals[slot] = e.call(POINT2_SET, &args![local, x, y]).u32();
        }
        fn_006b9820(
            e,
            Ptr::new(locals[0]),
            Ptr::new(locals[1]),
            Ptr::new(locals[2]),
            Ptr::new(locals[3]),
            out,
        )
    })
}

// Translated from 006b9820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the barycentric coordinates of `p4` in the 2D
/// triangle `p1`, `p2`, `p3`. The doubled area `d = (p2 - p1) x (p3 - p1)`
/// is stored as a `float`; when it equals the double `0.0` the function
/// returns false and writes nothing (a NaN area goes on). Otherwise
/// `out[0..3]` are `(p2 - p4) x (p3 - p4)`, `(p3 - p4) x (p1 - p4)` and
/// `(p1 - p4) x (p2 - p4)`, each times the `float` `1 / d`, and it returns
/// true.
pub fn fn_006b9820(e: &mut Engine, p1: Ptr, p2: Ptr, p3: Ptr, p4: Ptr, out: Ptr) -> bool {
    let (a, b, c, p) = (p1.addr(), p2.addr(), p3.addr(), p4.addr());
    let area = x87_store32(cross_x87(e, a, b, c));
    if x87_load32(area) == e.global::<f64>(ZERO) {
        return false;
    }
    let inverse = x87_store32(x87_div(1.0, x87_load32(area)));
    // Each weight is stored before the next is computed, as the exe does.
    for (i, (o, u, v)) in [(p, b, c), (p, c, a), (p, a, b)].into_iter().enumerate() {
        let value = x87_mul(cross_x87(e, o, u, v), x87_m32(inverse));
        e.mem
            .set_u32(out.addr().wrapping_add(4 * i as u32), x87_store32(value));
    }
    true
}

// Translated from 006b9940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the distance from `point` to the line through
/// `start` and `end`: projects `point - start` on `end - start` (the
/// `float` ratio of the two dot products, `004b6190`, scaling `end - start`
/// with `004a3760`), adds the projection to `start` (`00439e90`) and returns
/// `004019b0` of the squared length of `point` minus that foot (ST0, as the
/// callee leaves it).
pub fn fn_006b9940(e: &mut Engine, point: Ptr, start: Ptr, end: Ptr) -> f64 {
    e.with_stack(0x3c, |e, stack| {
        let s = stack.addr();
        let direction = s;
        let offset = s + 0xc;
        let scaled = s + 0x18;
        let foot = s + 0x24;
        let difference = s + 0x30;
        e.call(VECTOR_SUBTRACT, &args![end, direction, start]);
        e.call(VECTOR_SUBTRACT, &args![point, offset, start]);
        let along = dot(e, offset, direction) as f32;
        let squared = dot(e, direction, direction) as f32;
        // 006b9984: `FLD along; FDIV float ptr [squared]; FSTP ratio` (the x87
        // NaN choice and the control word's precision).
        let ratio = f32::from_bits(x87_store32(x87_div(
            x87_load32(along.to_bits()),
            x87_m32(squared.to_bits()),
        )));
        let projection = e.call(VECTOR_SCALE, &args![scaled, ratio, direction]).u32();
        e.call(VECTOR_ADD, &args![start, foot, projection]);
        e.call(VECTOR_SUBTRACT, &args![point, difference, foot]);
        let distance_squared = dot(e, difference, difference) as f32;
        e.call(ROOT, &args![distance_squared]).f64()
    })
}

// Translated from 006b99e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 2D dot product `x * other.x + y *
/// other.y`, stored as a `float` and returned. `RET 4`.
pub fn fn_006b99e0(e: &mut Engine, this: Ptr, other: Ptr) -> f32 {
    // `FLD this.x; FMUL dword ptr other.x; FLD this.y; FMUL dword ptr
    // other.y; FADDP`, with the x87 NaN choice (#199).
    let p = this.addr();
    let q = other.addr();
    let x = f64::from(x87_quiet(e.mem.f32(p)));
    let y = f64::from(x87_quiet(e.mem.f32(p.wrapping_add(4))));
    let ox = x87_m32(e.mem.u32(q));
    let oy = x87_m32(e.mem.u32(q.wrapping_add(4)));
    x87_add(x87_mul(x, ox), x87_mul(y, oy)) as f32
}

// Translated from 006b9a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: where the segment `start`..`end` crosses the
/// plane through `plane_point` with normal `normal`. With `n1 =
/// normal . (end - start)` and `n2 = -(normal . (start - plane_point))`
/// (both stored as `float`s): when `|n1|` (`00408840`) is below the `float`
/// `1e-7` at `0106c288` the segment is parallel and the result is -1 when
/// `n2` equals 0.0 (in the plane), else 0. Otherwise `t = n2 / n1`; outside
/// 0.0..1.0 the result is 0, else `out` is `start + t * (end - start)` and
/// the result 1 (a NaN `t` passes the range test).
pub fn fn_006b9a10(
    e: &mut Engine,
    start: Ptr,
    end: Ptr,
    plane_point: Ptr,
    normal: Ptr,
    out: Ptr,
) -> i32 {
    let epsilon = x87_quiet(e.global::<f32>(PLANE_EPSILON));
    e.with_stack(0x30, |e, stack| {
        let s = stack.addr();
        let direction = s;
        let offset = s + 0xc;
        let scaled = s + 0x18;
        let hit = s + 0x24;
        e.call(VECTOR_SUBTRACT, &args![end, direction, start]);
        e.call(VECTOR_SUBTRACT, &args![start, offset, plane_point]);
        let n1 = dot(e, normal.addr(), direction) as f32;
        let n2 = (-dot(e, normal.addr(), offset)) as f32;
        let magnitude = e.call(FABS, &args![n1]).f64();
        if f64::from(epsilon) > magnitude {
            return if f64::from(n2) == e.global::<f64>(ZERO) {
                -1
            } else {
                0
            };
        }
        // 006b9a97: `FLD n2; FDIV float ptr [n1]; FSTP t` (the x87 NaN choice
        // and the control word's precision).
        let t = f32::from_bits(x87_store32(x87_div(
            x87_load32(n2.to_bits()),
            x87_m32(n1.to_bits()),
        )));
        if f64::from(t) < e.global::<f64>(ZERO) || f64::from(t) > e.global::<f64>(ONE) {
            return 0;
        }
        let along = e
            .call(VECTOR_SCALE, &args![scaled, x87_quiet(t), direction])
            .u32();
        let point = e.call(VECTOR_ADD, &args![start, hit, along]).u32();
        copy_words(e, out.addr(), point, 3);
        1
    })
}

/// The shared head of [`fn_006b9b10`] and [`fn_006b9c20`]: `direction = end
/// - start`, `offset = point - start` and their dot product, kept as a
/// `double`.
fn segment_projection(
    e: &mut Engine,
    point: Ptr,
    start: Ptr,
    end: Ptr,
    direction: u32,
    offset: u32,
) -> f64 {
    e.call(VECTOR_SUBTRACT, &args![end, direction, start]);
    e.call(VECTOR_SUBTRACT, &args![point, offset, start]);
    dot(e, offset, direction)
}

// Translated from 006b9b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the distance from `point` to the segment
/// `start`..`end`, returned in ST0 as `00457990` leaves it. With `along =
/// (point - start) . (end - start)` and `squared = (end - start)^2` (both
/// `double`s): `along <= 0` gives the distance to `start`, `along >=
/// squared` the distance to `end`, otherwise to `start + (along / squared)
/// * (end - start)` (the ratio rounded to a `float`). `inside`, when not
/// null, is set to whether the last case was taken (a NaN goes there).
pub fn fn_006b9b10(e: &mut Engine, point: Ptr, start: Ptr, end: Ptr, inside: Ptr) -> f64 {
    let set_inside = |e: &mut Engine, value: u8| {
        if !inside.is_null() {
            e.mem.set_u8(inside.addr(), value);
        }
    };
    e.with_stack(0x3c, |e, stack| {
        let s = stack.addr();
        let direction = s;
        let offset = s + 0xc;
        let scaled = s + 0x18;
        let foot = s + 0x24;
        let difference = s + 0x30;
        let along = segment_projection(e, point, start, end, direction, offset);
        let target = if along <= 0.0 {
            set_inside(e, 0);
            start.addr()
        } else {
            let squared = dot(e, direction, direction);
            if along >= squared {
                set_inside(e, 0);
                end.addr()
            } else {
                // 006b9bba: `FLD along; FDIV double ptr [squared]; FSTP double;
                // FLD double; FSTP float` (the x87 NaN choice and the control
                // word's precision; the double round trip is exact here).
                let ratio =
                    f32::from_bits(x87_store32(x87_div(x87_load64(along.to_bits()), squared)));
                let projection = e.call(VECTOR_SCALE, &args![scaled, ratio, direction]).u32();
                e.call(VECTOR_ADD, &args![start, foot, projection]);
                set_inside(e, 1);
                foot
            }
        };
        let vector = e
            .call(VECTOR_SUBTRACT, &args![point, difference, target])
            .u32();
        e.call(LENGTH, &args![vector]).f64()
    })
}

// Translated from 006b9c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes to `out` the point of the segment
/// `start`..`end` closest to `point` (the same cases as [`fn_006b9b10`]:
/// `start`, `end`, or `start + ratio * (end - start)` written by
/// `00439e90`) and returns `out`.
pub fn fn_006b9c20(e: &mut Engine, out: Ptr, point: Ptr, start: Ptr, end: Ptr) -> Ptr {
    e.with_stack(0x24, |e, stack| {
        let s = stack.addr();
        let direction = s;
        let offset = s + 0xc;
        let scaled = s + 0x18;
        let along = segment_projection(e, point, start, end, direction, offset);
        if along <= 0.0 {
            copy_words(e, out.addr(), start.addr(), 3);
            return out;
        }
        let squared = dot(e, direction, direction);
        if along >= squared {
            copy_words(e, out.addr(), end.addr(), 3);
            return out;
        }
        // 006b9cb3: `FLD along; FDIV double ptr [squared]; FSTP double; FLD
        // double; FSTP float`, as in 006b9b10.
        let ratio = f32::from_bits(x87_store32(x87_div(x87_load64(along.to_bits()), squared)));
        let projection = e.call(VECTOR_SCALE, &args![scaled, ratio, direction]).u32();
        e.call(VECTOR_ADD, &args![start, out, projection]);
        out
    })
}

// Translated from 006b9cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the height of `point` over the triangle `p1`,
/// `p2`, `p3`. Returns false when the doubled 2D area `(p2 - p1) x (p3 -
/// p1)` (a `float`) equals 0.0, or when any of the three barycentric
/// weights of `point` (each a `float`) is below 0.0. Otherwise the height
/// `w1 * p1.z + w2 * p2.z + w3 * p3.z - point.z` (stored as a `float` twice)
/// must be at most `above` and at least `-below`; then it is written to
/// `out` and the result is true. NaNs pass every test.
#[allow(clippy::too_many_arguments)]
pub fn fn_006b9cf0(
    e: &mut Engine,
    p1: Ptr,
    p2: Ptr,
    p3: Ptr,
    point: Ptr,
    out: Ptr,
    above: f32,
    below: f32,
) -> bool {
    let zero = e.global::<f64>(ZERO);
    let (a, b, c, p) = (p1.addr(), p2.addr(), p3.addr(), point.addr());
    let area = x87_store32(cross_x87(e, a, b, c));
    if x87_load32(area) == zero {
        return false;
    }
    // The x87 stack code: `FLD` a float, `FSUB`/`FMUL` a float in memory, `FSTP`
    // to a scratch float (the scratch floats are the `u32` bit patterns here).
    let m = |q: u32| x87_m32(e.mem.u32(q));
    let l = |q: u32| x87_load32(e.mem.u32(q));
    let difference = |from: u32, to: u32| x87_store32(x87_sub(l(from), m(to)));
    let bx_p = difference(b, p);
    let cx_p = difference(c, p);
    let by_p = difference(b.wrapping_add(4), p.wrapping_add(4));
    let cy_p = difference(c.wrapping_add(4), p.wrapping_add(4));
    let inverse = x87_store32(x87_div(1.0, x87_load32(area)));
    // `FLD s; FMUL t; FLD u; FMUL v; FSUBP; FMUL inverse; FSTP`.
    let weight = |s: u32, t: u32, u: u32, v: u32| {
        let first = x87_mul(x87_load32(s), x87_m32(t));
        let second = x87_mul(x87_load32(u), x87_m32(v));
        x87_store32(x87_mul(x87_sub(first, second), x87_m32(inverse)))
    };
    let w1 = weight(bx_p, cy_p, cx_p, by_p);
    if x87_load32(w1) < zero {
        return false;
    }
    let ax_p = difference(a, p);
    let ay_p = difference(a.wrapping_add(4), p.wrapping_add(4));
    let w2 = weight(cx_p, ay_p, ax_p, cy_p);
    if x87_load32(w2) < zero {
        return false;
    }
    let w3 = weight(ax_p, by_p, bx_p, ay_p);
    if x87_load32(w3) < zero {
        return false;
    }
    let z = |q: u32| m(q.wrapping_add(8));
    let surface = x87_store32(x87_add(
        x87_add(x87_mul(x87_load32(w1), z(a)), x87_mul(x87_load32(w2), z(b))),
        x87_mul(x87_load32(w3), z(c)),
    ));
    let height = x87_store32(x87_sub(x87_load32(surface), z(p)));
    let height_value = f32::from_bits(height);
    if above < height_value {
        return false;
    }
    if height_value < -below {
        return false;
    }
    e.mem.set_u32(out.addr(), height);
    true
}

// Translated from 006b9e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the 2D segments `p1`..`p2` and
/// `p3`..`p4` cross, writing the crossing point (two floats) to `out`.
/// With the `float` areas `a = (p1 - p4) x (p2 - p4)`, `b = (p1 - p3) x
/// (p2 - p3)` and `c = (p3 - p1) x (p4 - p1)`: false unless `a * b < 0`;
/// `d = c + b - a`, recomputed as `(p3 - p2) x (p4 - p2)` when `|d|`
/// (`00408840`) is below the double `0.001`; false unless `c * d < 0`.
/// Then `t = c / (c - d)` and `out = p1 + t * (p2 - p1)` (`004e8880`,
/// `00532fd0`, `004dfe30`), true.
pub fn fn_006b9e90(e: &mut Engine, p1: Ptr, p2: Ptr, p3: Ptr, p4: Ptr, out: Ptr) -> bool {
    let zero = e.global::<f64>(ZERO);
    let (a, b, c, d) = (p1.addr(), p2.addr(), p3.addr(), p4.addr());
    let first = cross(e, d, a, b) as f32;
    let second = cross(e, c, a, b) as f32;
    let opposite_sides = f64::from(first) * f64::from(second) < zero;
    if !opposite_sides {
        return false;
    }
    let third = cross(e, a, c, d) as f32;
    let mut fourth = (f64::from(third) + f64::from(second) - f64::from(first)) as f32;
    let magnitude = e.call(FABS, &args![fourth]).f64();
    if magnitude < e.global::<f64>(AREA_EPSILON) {
        fourth = cross(e, b, c, d) as f32;
    }
    let crosses = f64::from(third) * f64::from(fourth) < zero;
    if !crosses {
        return false;
    }
    let t = (f64::from(third) / (f64::from(third) - f64::from(fourth))) as f32;
    e.with_stack(0x18, |e, stack| {
        let s = stack.addr();
        let direction = e.call(POINT2_SUBTRACT, &args![b, s, a]).u32();
        let along = e
            .call(POINT2_SCALE, &args![s + 8, x87_quiet(t), direction])
            .u32();
        let point = e.call(POINT2_ADD, &args![a, s + 0x10, along]).u32();
        copy_words(e, out.addr(), point, 2);
    });
    true
}

/// The two area tests [`fn_006ba010`] and [`fn_006ba100`] share: `a * b <=
/// 0` and then `c * (c + b - a) < 0` (each area a `float`). Returns `c` and
/// `c + b - a` (their bits) when both pass. Every operation is the x87's
/// (006ba016..006ba0d1 / 006ba106..006ba1c5), rounded at the control word's
/// precision: at 24 bits a product or difference rounds before the next
/// one, which plain `f64` does not do.
fn segments_cross(e: &mut Engine, p1: u32, p2: u32, p3: u32, p4: u32) -> Option<(u32, u32)> {
    let zero = e.global::<f64>(ZERO);
    // 006ba106..006ba16d: `a` (EBP-4) and `b` (EBP-8) stored as floats.
    let first = x87_store32(cross_x87(e, p4, p1, p2));
    let second = x87_store32(cross_x87(e, p3, p1, p2));
    // 006ba170..006ba181: FLD a, FMUL b, FCOMP 0.0, TEST AH,0x41 / JP (`<=`).
    let not_same_side = x87_mul(x87_load32(first), x87_m32(second)) <= zero;
    if !not_same_side {
        return None;
    }
    // 006ba187..006ba1b9: `c` (EBP-0x10).
    let third = x87_store32(cross_x87(e, p1, p3, p4));
    // 006ba1bc..006ba1c5: FLD c, FADD b, FSUB a, FSTP `d` (EBP-0xC).
    let fourth = x87_store32(x87_sub(
        x87_add(x87_load32(third), x87_m32(second)),
        x87_m32(first),
    ));
    // 006ba1c8..006ba1d9: FLD c, FMUL d, FCOMP 0.0, TEST AH,5 / JP (`<`).
    if x87_mul(x87_load32(third), x87_m32(fourth)) < zero {
        Some((third, fourth))
    } else {
        None
    }
}

// Translated from 006ba010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the 2D segments `p1`..`p2` and
/// `p3`..`p4` cross: with the `float` areas `a = (p1 - p4) x (p2 - p4)`,
/// `b = (p1 - p3) x (p2 - p3)` and `c = (p3 - p1) x (p4 - p1)`, true when
/// `a * b <= 0` and `c * (c + b - a) < 0` (NaNs fail both).
pub fn fn_006ba010(e: &mut Engine, p1: Ptr, p2: Ptr, p3: Ptr, p4: Ptr) -> bool {
    segments_cross(e, p1.addr(), p2.addr(), p3.addr(), p4.addr()).is_some()
}

// Translated from 006ba100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: [`fn_006ba010`]'s test, and when the segments
/// cross, `out = p1 + t * (p2 - p1)` as an `NiPoint3` (`00439ef0`,
/// `004a3760`, `00439e90`) with `t = c / (c - d)` (`d = c + b - a`).
pub fn fn_006ba100(e: &mut Engine, p1: Ptr, p2: Ptr, p3: Ptr, p4: Ptr, out: Ptr) -> bool {
    let Some((third, fourth)) = segments_cross(e, p1.addr(), p2.addr(), p3.addr(), p4.addr())
    else {
        return false;
    };
    // 006ba1db..006ba1e4: FLD c, FSUB d, FDIVR c, FSTP `t` (EBP-0x14).
    let t = x87_store32(x87_div(
        x87_m32(third),
        x87_sub(x87_load32(third), x87_m32(fourth)),
    ));
    e.with_stack(0x24, |e, stack| {
        let s = stack.addr();
        let direction = e.call(VECTOR_SUBTRACT, &args![p2, s, p1]).u32();
        // 006ba1f9..006ba1fc: `t` pushed through FLD/FSTP.
        let along = e
            .call(VECTOR_SCALE, &args![s + 0xc, x87_quiet_bits(t), direction])
            .u32();
        let point = e.call(VECTOR_ADD, &args![p1, s + 0x18, along]).u32();
        copy_words(e, out.addr(), point, 3);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b9790, fn_006b9790(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006b9820, fn_006b9820(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006b9940, fn_006b9940(Ptr, Ptr, Ptr) -> f64),
        entry!(0x006b99e0, fn_006b99e0(Ptr, Ptr) -> f32),
        entry!(0x006b9a10, fn_006b9a10(Ptr, Ptr, Ptr, Ptr, Ptr) -> i32),
        entry!(0x006b9b10, fn_006b9b10(Ptr, Ptr, Ptr, Ptr) -> f64),
        entry!(0x006b9c20, fn_006b9c20(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x006b9cf0,
            fn_006b9cf0(Ptr, Ptr, Ptr, Ptr, Ptr, f32, f32) -> bool
        ),
        entry!(0x006b9e90, fn_006b9e90(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006ba010, fn_006ba010(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006ba100, fn_006ba100(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn f(e: &Engine, a: u32) -> f32 {
        e.mem.f32(a)
    }

    /// Doubles computing the vector helpers in `f32` on the game's memory,
    /// plus the constants the code reads.
    fn geometry_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x0101_6000, 0x0106_c000] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(PLANE_EPSILON, 1e-7f32);
        e.set_global(AREA_EPSILON, 0.001f64);
        e.register(POINT2_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = f(e, a[0] + 4 * i) - f(e, a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(VECTOR_ADD, |e, a| {
            for i in 0..3 {
                let v = f(e, a[0] + 4 * i) + f(e, a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(VECTOR_SCALE, |e, a| {
            let s = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = s * f(e, a[2] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(DOT, |e, a| {
            let d: f32 = (0..3)
                .map(|i| f(e, a[0] + 4 * i) * f(e, a[1] + 4 * i))
                .sum();
            f64::from(d).into_ret()
        });
        e.register(LENGTH, |e, a| {
            let d: f32 = (0..3).map(|i| f(e, a[0] + 4 * i).powi(2)).sum();
            f64::from(d.sqrt()).into_ret()
        });
        e.register(ROOT, |_, a| {
            f64::from(f32::from_bits(a[0]).sqrt()).into_ret()
        });
        e.register(FABS, |_, a| {
            f64::from(f32::from_bits(a[0]).abs()).into_ret()
        });
        e.register(POINT2_SUBTRACT, |e, a| {
            for i in 0..2 {
                let v = f(e, a[0] + 4 * i) - f(e, a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT2_SCALE, |e, a| {
            let s = f32::from_bits(a[1]);
            for i in 0..2 {
                let v = s * f(e, a[2] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT2_ADD, |e, a| {
            for i in 0..2 {
                let v = f(e, a[0] + 4 * i) + f(e, a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e
    }

    fn p3(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        p
    }

    fn read3(e: &Engine, p: u32) -> [f32; 3] {
        [f(e, p), f(e, p + 4), f(e, p + 8)]
    }

    #[test]
    fn barycentric_weights() {
        let mut e = geometry_engine();
        let a = p3(&mut e, 0.0, 0.0, 0.0);
        let b = p3(&mut e, 4.0, 0.0, 0.0);
        let c = p3(&mut e, 0.0, 4.0, 0.0);
        let p = p3(&mut e, 1.0, 1.0, 0.0);
        let out = p3(&mut e, 9.0, 9.0, 9.0);
        assert!(e.call(0x006b9820, &args![a, b, c, p, out]).bool());
        assert_eq!(read3(&e, out), [0.5, 0.25, 0.25]);
        // A degenerate triangle: false, nothing written.
        let flat = p3(&mut e, 8.0, 0.0, 0.0);
        let out2 = p3(&mut e, 9.0, 9.0, 9.0);
        assert!(!e.call(0x006b9820, &args![a, b, flat, p, out2]).bool());
        assert_eq!(read3(&e, out2), [9.0, 9.0, 9.0]);
        // A NaN area is not equal to 0.0: it goes on.
        let nan = p3(&mut e, f32::NAN, 0.0, 0.0);
        assert!(e.call(0x006b9820, &args![a, nan, c, p, out2]).bool());
    }

    #[test]
    fn barycentric_through_copied_points() {
        let mut e = geometry_engine();
        let a = p3(&mut e, 0.0, 0.0, 0.0);
        let b = p3(&mut e, 4.0, 0.0, 0.0);
        let c = p3(&mut e, 0.0, 4.0, 0.0);
        let p = p3(&mut e, 1.0, 1.0, 0.0);
        let out = p3(&mut e, 9.0, 9.0, 9.0);
        // A signalling NaN z is not copied; one in p's x is, quiet.
        e.mem.set_u32(a + 8, 0x7f80_0001);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b9790, &args![a, b, c, p, out]).bool());
        let log = e.call_log.take().unwrap();
        let sets = calls(&log, POINT2_SET);
        assert_eq!(sets.len(), 4);
        // p4 first, p1 last.
        assert_eq!(&sets[0][1..], &[1.0f32.to_bits(), 1.0f32.to_bits()]);
        assert_eq!(&sets[3][1..], &[0, 0]);
        assert_eq!(read3(&e, out), [0.5, 0.25, 0.25]);
        e.mem.set_u32(p, 0x7f80_0002);
        e.call_log = Some(vec![]);
        e.call(0x006b9790, &args![a, b, c, p, out]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, POINT2_SET)[0][1], 0x7fc0_0002);
    }

    #[test]
    fn distance_to_line() {
        let mut e = geometry_engine();
        let point = p3(&mut e, 5.0, 4.0, 0.0);
        let start = p3(&mut e, 0.0, 0.0, 0.0);
        let end = p3(&mut e, 10.0, 0.0, 0.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b9940, &args![point, start, end]).f64(), 4.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ROOT), vec![vec![16.0f32.to_bits()]]);
        assert_eq!(calls(&log, VECTOR_SCALE)[0][1], 0.5f32.to_bits());
        // Beyond the end the line still counts.
        let far = p3(&mut e, 20.0, -2.0, 0.0);
        assert_eq!(e.call(0x006b9940, &args![far, start, end]).f64(), 2.0);
    }

    #[test]
    fn dot_2d() {
        let mut e = geometry_engine();
        let a = p3(&mut e, 2.0, 3.0, 100.0);
        let b = p3(&mut e, 4.0, 5.0, 100.0);
        assert_eq!(e.call(0x006b99e0, &args![a, b]).f32(), 23.0);
    }

    #[test]
    fn dot_2d_keeps_the_x87_nan_when_two_nans_meet() {
        // #199: x = -qNaN, other.x = +qNaN; FMUL keeps the positive one.
        let mut e = geometry_engine();
        let a = p3(&mut e, f32::from_bits(0xffff_ffff), 3.0, 100.0);
        let b = p3(&mut e, f32::from_bits(0x7fff_ffff), 5.0, 100.0);
        let result = e.call(0x006b99e0, &args![a, b]).f32();
        assert_eq!(result.to_bits(), 0x7fff_ffff);
    }

    #[test]
    fn segment_plane() {
        let mut e = geometry_engine();
        let start = p3(&mut e, 0.0, 0.0, -1.0);
        let end = p3(&mut e, 2.0, 0.0, 1.0);
        let origin = p3(&mut e, 0.0, 0.0, 0.0);
        let up = p3(&mut e, 0.0, 0.0, 1.0);
        let out = p3(&mut e, 9.0, 9.0, 9.0);
        assert_eq!(
            e.call(0x006b9a10, &args![start, end, origin, up, out])
                .i32(),
            1
        );
        assert_eq!(read3(&e, out), [1.0, 0.0, 0.0]);
        // The plane beyond the end: t = 1.5.
        let high = p3(&mut e, 0.0, 0.0, 2.0);
        assert_eq!(
            e.call(0x006b9a10, &args![start, end, high, up, out]).i32(),
            0
        );
        // Behind the start: t < 0.
        let low = p3(&mut e, 0.0, 0.0, -3.0);
        assert_eq!(
            e.call(0x006b9a10, &args![start, end, low, up, out]).i32(),
            0
        );
        // Parallel: in the plane (-1) or not (0).
        let flat_end = p3(&mut e, 5.0, 0.0, -1.0);
        let in_plane = p3(&mut e, 0.0, 0.0, -1.0);
        assert_eq!(
            e.call(0x006b9a10, &args![start, flat_end, in_plane, up, out])
                .i32(),
            -1
        );
        assert_eq!(
            e.call(0x006b9a10, &args![start, flat_end, origin, up, out])
                .i32(),
            0
        );
    }

    /// 006b9a97: `t = n2 / n1` is an x87 `FDIV`: with two NaNs the one with
    /// the larger significand (here `n1`) comes out, where an `f64` division
    /// would keep the dividend's (`n2`).
    #[test]
    fn segment_plane_divides_with_the_x87_nan_choice() {
        let mut e = geometry_engine();
        // n1 = 0x7fff_0000; n2 = -(0x7fc0_0001) = 0xffc0_0001.
        let mut dots = [0x7fff_0000u32, 0x7fc0_0001].into_iter();
        e.register_double(DOT, move |_, _| {
            f64::from(f32::from_bits(dots.next().unwrap())).into_ret()
        });
        let start = p3(&mut e, 0.0, 0.0, -1.0);
        let end = p3(&mut e, 2.0, 0.0, 1.0);
        let origin = p3(&mut e, 0.0, 0.0, 0.0);
        let up = p3(&mut e, 0.0, 0.0, 1.0);
        let out = p3(&mut e, 9.0, 9.0, 9.0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006b9a10, &args![start, end, origin, up, out])
                .i32(),
            1
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FABS), vec![vec![0x7fff_0000]]);
        assert_eq!(calls(&log, VECTOR_SCALE)[0][1], 0x7fff_0000);
    }

    /// 006b9984: `ratio = along / squared` is an x87 `FDIV` of two floats:
    /// with two quiet NaNs the one with the larger significand (here
    /// `squared`) comes out, where an `f64` division would keep the
    /// dividend's (`along`).
    #[test]
    fn distance_to_line_divides_with_the_x87_nan_choice() {
        let mut e = geometry_engine();
        // along = 0xffc0_0001, squared = 0x7fff_0000, then the squared
        // distance 16.
        let mut dots = [x87_m32(0xffc0_0001), x87_m32(0x7fff_0000), 16.0].into_iter();
        e.register_double(DOT, move |_, _| dots.next().unwrap().into_ret());
        let point = p3(&mut e, 5.0, 4.0, 0.0);
        let start = p3(&mut e, 0.0, 0.0, 0.0);
        let end = p3(&mut e, 10.0, 0.0, 0.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b9940, &args![point, start, end]).f64(), 4.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, VECTOR_SCALE)[0][1], 0x7fff_0000);
        assert_eq!(calls(&log, ROOT), vec![vec![16.0f32.to_bits()]]);
    }

    /// 006b9bba: the ratio is an x87 `FDIV` of two doubles: with two quiet
    /// NaNs the divisor's (the larger significand) comes out, where an `f64`
    /// division would keep the dividend's. A NaN `along` compares unordered,
    /// so `TEST AH,1` at 006b9b4c and 006b9b90 takes both jumps to the ratio
    /// case.
    #[test]
    fn distance_to_segment_divides_with_the_x87_nan_choice() {
        let mut e = geometry_engine();
        let mut dots = [x87_m32(0xffc0_0001), x87_m32(0x7fff_0000)].into_iter();
        e.register_double(DOT, move |_, _| dots.next().unwrap().into_ret());
        let point = p3(&mut e, 4.0, 3.0, 0.0);
        let start = p3(&mut e, 0.0, 0.0, 0.0);
        let end = p3(&mut e, 10.0, 0.0, 0.0);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 7);
        e.call_log = Some(vec![]);
        e.call(0x006b9b10, &args![point, start, end, flag]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, VECTOR_SCALE)[0][1], 0x7fff_0000);
        assert_eq!(e.mem.u8(flag), 1);
    }

    /// 006b9cb3: the same `FDIV` of two doubles as 006b9b10's, reached by a
    /// NaN `along` through the unordered compares at 006b9c5c and 006b9c93.
    #[test]
    fn closest_point_divides_with_the_x87_nan_choice() {
        let mut e = geometry_engine();
        let mut dots = [x87_m32(0xffc0_0001), x87_m32(0x7fff_0000)].into_iter();
        e.register_double(DOT, move |_, _| dots.next().unwrap().into_ret());
        let point = p3(&mut e, 4.0, 3.0, 0.0);
        let start = p3(&mut e, 0.0, 0.0, 0.0);
        let end = p3(&mut e, 10.0, 0.0, 0.0);
        let out = p3(&mut e, 9.0, 9.0, 9.0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006b9c20, &args![out, point, start, end]).u32(),
            out
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, VECTOR_SCALE)[0][1], 0x7fff_0000);
        // `out = start + ratio * (end - start)` by 00439e90.
        assert_eq!(&calls(&log, VECTOR_ADD)[0][..2], &[start, out]);
    }

    #[test]
    fn distance_to_segment_and_closest_point() {
        let mut e = geometry_engine();
        let start = p3(&mut e, 0.0, 0.0, 0.0);
        let end = p3(&mut e, 10.0, 0.0, 0.0);
        let flag = e.mem.alloc(4);
        let out = p3(&mut e, 0.0, 0.0, 0.0);
        // (point, distance, inside, closest)
        let cases: [([f32; 3], f64, u8, [f32; 3]); 3] = [
            ([-3.0, 4.0, 0.0], 5.0, 0, [0.0, 0.0, 0.0]),
            ([13.0, 4.0, 0.0], 5.0, 0, [10.0, 0.0, 0.0]),
            ([4.0, 3.0, 0.0], 3.0, 1, [4.0, 0.0, 0.0]),
        ];
        for (p, distance, inside, closest) in cases {
            let point = p3(&mut e, p[0], p[1], p[2]);
            e.mem.set_u8(flag, 7);
            let got = e.call(0x006b9b10, &args![point, start, end, flag]).f64();
            assert_eq!(got, distance);
            assert_eq!(e.mem.u8(flag), inside);
            // A null flag pointer is skipped.
            let got = e.call(0x006b9b10, &args![point, start, end, 0u32]).f64();
            assert_eq!(got, distance);
            assert_eq!(
                e.call(0x006b9c20, &args![out, point, start, end]).u32(),
                out
            );
            assert_eq!(read3(&e, out), closest);
        }
    }

    #[test]
    fn height_over_triangle() {
        let mut e = geometry_engine();
        let a = p3(&mut e, 0.0, 0.0, 0.0);
        let b = p3(&mut e, 4.0, 0.0, 0.0);
        let c = p3(&mut e, 0.0, 4.0, 8.0);
        let out = e.mem.alloc(4);
        let point = p3(&mut e, 1.0, 1.0, 0.0);
        let call = |e: &mut Engine, p: u32, above: f32, below: f32| {
            e.mem.set_f32(out, -99.0);
            e.call(0x006b9cf0, &args![a, b, c, p, out, above, below])
                .bool()
        };
        assert!(call(&mut e, point, 3.0, 1.0));
        assert_eq!(f(&e, out), 2.0);
        // Too far above, or too far below.
        assert!(!call(&mut e, point, 1.0, 1.0));
        assert_eq!(f(&e, out), -99.0);
        let deep = p3(&mut e, 1.0, 1.0, 5.0);
        assert!(!call(&mut e, deep, 3.0, 1.0));
        assert!(call(&mut e, deep, 3.0, 3.0));
        assert_eq!(f(&e, out), -3.0);
        // Outside the triangle: a negative weight.
        let outside = p3(&mut e, 5.0, 5.0, 0.0);
        assert!(!call(&mut e, outside, 100.0, 100.0));
        let behind = p3(&mut e, -1.0, 1.0, 0.0);
        assert!(!call(&mut e, behind, 100.0, 100.0));
        let below_edge = p3(&mut e, 1.0, -1.0, 0.0);
        assert!(!call(&mut e, below_edge, 100.0, 100.0));
        // A degenerate triangle.
        let line = p3(&mut e, 8.0, 0.0, 0.0);
        assert!(!e
            .call(0x006b9cf0, &args![a, b, line, point, out, 9.0f32, 9.0f32])
            .bool());
    }

    #[test]
    fn segment_intersection_2d() {
        let mut e = geometry_engine();
        let p1 = p3(&mut e, 0.0, 0.0, 0.0);
        let p2 = p3(&mut e, 2.0, 2.0, 4.0);
        let p3_ = p3(&mut e, 0.0, 2.0, 0.0);
        let p4 = p3(&mut e, 2.0, 0.0, 0.0);
        let out = p3(&mut e, 9.0, 9.0, 9.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b9e90, &args![p1, p2, p3_, p4, out]).bool());
        let log = e.call_log.take().unwrap();
        // d = c + b - a = 4: no recomputation needed.
        assert_eq!(calls(&log, FABS), vec![vec![4.0f32.to_bits()]]);
        assert_eq!(read3(&e, out), [1.0, 1.0, 9.0]);
        assert!(e.call(0x006ba010, &args![p1, p2, p3_, p4]).bool());
        assert!(e.call(0x006ba100, &args![p1, p2, p3_, p4, out]).bool());
        assert_eq!(read3(&e, out), [1.0, 1.0, 2.0]);

        // Apart: the second segment is beyond the first's end.
        let q3 = p3(&mut e, 5.0, 0.0, 0.0);
        let q4 = p3(&mut e, 5.0, 9.0, 0.0);
        assert!(!e.call(0x006b9e90, &args![p1, p2, q3, q4, out]).bool());
        assert!(!e.call(0x006ba010, &args![p1, p2, q3, q4]).bool());
        assert!(!e.call(0x006ba100, &args![p1, p2, q3, q4, out]).bool());
        // Both on one side of the first segment's line.
        let r3 = p3(&mut e, 0.0, 5.0, 0.0);
        let r4 = p3(&mut e, 1.0, 9.0, 0.0);
        assert!(!e.call(0x006b9e90, &args![p1, p2, r3, r4, out]).bool());
        assert!(!e.call(0x006ba010, &args![p1, p2, r3, r4]).bool());

        // Touching: p3 on the first segment gives a * b == 0, which only the
        // `<=` test of 006ba010 accepts.
        let t3 = p3(&mut e, 1.0, 1.0, 0.0);
        let t4 = p3(&mut e, 2.0, 0.0, 0.0);
        assert!(!e.call(0x006b9e90, &args![p1, p2, t3, t4, out]).bool());
        assert!(e.call(0x006ba010, &args![p1, p2, t3, t4]).bool());
    }

    /// 006ba100 (and 006ba010, whose test it shares) at the main thread's
    /// 24-bit precision (0x007F): each `FSUB`/`FMUL` of an area rounds to 24
    /// bits before the next. With p4 on the line p1..p2 up to a product's
    /// last bits, `a = p1.x * p2.y - p2.x * p1.y = (-2 + 2^-45) - (-2)`: at
    /// 53 bits 2^-45, so with `b = 9` the test `a * b <= 0` (006ba181) fails;
    /// at 24 bits the product rounds to -2 and `a = 0` passes it. Then
    /// `c = -(3 + 3 eps)` rounds (a tie, to even) to `-(3 + 4 eps)`,
    /// `d = c + b - a = 6 - 4 eps`, and `t = c / (c - d) = (3 + 4 eps) / 9`
    /// goes to the scale (006ba203).
    #[test]
    fn segment_crossing_rounds_each_area_operation_at_24_bit_precision() {
        let mut e = geometry_engine();
        let eps = f32::EPSILON;
        let p1 = p3(&mut e, -(1.0 + eps), -1.0, 0.0);
        let p2 = p3(&mut e, 2.0, 2.0 - 2.0 * eps, 0.0);
        let p3_ = p3(&mut e, 0.0, 3.0, 0.0);
        let p4 = p3(&mut e, 0.0, 0.0, 0.0);
        let out = p3(&mut e, 9.0, 9.0, 9.0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006ba010, &args![p1, p2, p3_, p4]).bool());
        assert!(!e.call(0x006ba100, &args![p1, p2, p3_, p4, out]).bool());
        assert!(calls(e.call_log.as_ref().unwrap(), VECTOR_SCALE).is_empty());
        crate::units::crt::set_x87_control_word(&mut e, 0x007f);
        assert!(e.call(0x006ba010, &args![p1, p2, p3_, p4]).bool());
        assert!(e.call(0x006ba100, &args![p1, p2, p3_, p4, out]).bool());
        crate::units::crt::set_x87_control_word(&mut e, 0x027f);
        let log = e.call_log.take().unwrap();
        let t = ((3.0 + 4.0 * f64::from(eps)) / 9.0) as f32;
        let scaled = calls(&log, VECTOR_SCALE);
        assert_eq!(scaled.len(), 1);
        assert_eq!(scaled[0][1], t.to_bits());
    }
}
