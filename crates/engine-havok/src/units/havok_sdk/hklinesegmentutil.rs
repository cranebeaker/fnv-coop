//! `Havok SDK/hklinesegmentutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vectors are SSE (`f32` lanes, three-component sums added as `z + (y + x)`); the scalar
//! solving in `closestLineSegLineSeg` runs on the x87 stack, approximated here in `f64` with a
//! rounding to `f32` wherever the code stores a `float`.

#[allow(unused_imports)]
use crate::prelude::*;

type Vec4 = [f32; 4];

fn load(e: &Engine, at: Ptr) -> Vec4 {
    let at = at.addr();
    [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ]
}

fn store(e: &mut Engine, at: u32, value: Vec4) {
    for (lane, component) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * lane as u32, component);
    }
}

/// `SUBPS a, b` (the SSE NaN rules).
fn sub(a: Vec4, b: Vec4) -> Vec4 {
    crate::sse::subps(a, b)
}

/// `MULPS a, b`, then the sum of the three lane products added as `z + (y + x)` (`ADDSS`
/// with the higher lane as the destination), with the SSE NaN rules.
fn dot3(a: Vec4, b: Vec4) -> f32 {
    use crate::sse::{add, mulps};
    let p = mulps(a, b);
    add(p[2], add(p[1], p[0]))
}

/// `scale * direction + base` in every lane (`MULPS` with the splatted scale as the
/// destination, then `ADDPS` onto `base`), with the SSE NaN rules.
fn scaled_add(scale: f32, direction: Vec4, base: Vec4) -> Vec4 {
    crate::sse::addps(crate::sse::mulps([scale; 4], direction), base)
}

// Translated from 00d230e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLineSegmentUtil::closestPointLineSeg` (Xbox PDB), cdecl `(point, a, b, out)`: writes the
/// point of segment `a`..`b` closest to `point` (16 bytes) and returns which end it is:
/// 8 for `a` (the projection `(point - a) . (b - a)` is not above 0), 4 for `b` (the squared
/// length does not exceed the projection), 0 for a point inside (NaNs fall into this case).
pub fn hk_line_segment_closest_point(e: &mut Engine, point: Ptr, a: Ptr, b: Ptr, out: Ptr) -> u32 {
    let a = load(e, a);
    let b = load(e, b);
    let p = load(e, point);
    let along = sub(b, a);
    let projection = dot3(sub(p, a), along);
    let length_squared = dot3(along, along);
    if projection <= 0.0 {
        store(e, out.addr(), a);
        return 8;
    }
    if length_squared <= projection {
        store(e, out.addr(), b);
        return 4;
    }
    // `FDIVP` of the two loaded floats (the x87 choice of NaN).
    let fraction = f32::from_bits(x87_store32(x87_div(
        x87_load32(projection.to_bits()),
        x87_load32(length_squared.to_bits()),
    )));
    store(e, out.addr(), scaled_add(fraction, along, a));
    0
}

// Translated from 00d22e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLineSegmentUtil::closestLineSegLineSeg` (Xbox PDB), cdecl
/// `(start_a, direction_a, start_b, direction_b, out)`: the closest points of the segments
/// `start_a + s * direction_a` and `start_b + t * direction_b` (`s`, `t` in 0..1). Writes the
/// point on A (`out[0..4]`), the vector from the point on B to it (`out + 0x10`) and its squared
/// length (`out + 0x20`), and returns which parameters were clamped: bit 0 `s` = 1, bit 1
/// `s` = 0, bit 2 `t` = 1, bit 3 `t` = 0.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn hk_line_segment_closest_line_seg_line_seg(
    e: &mut Engine,
    start_a: Ptr,
    direction_a: Ptr,
    start_b: Ptr,
    direction_b: Ptr,
    out: Ptr,
) -> u32 {
    let a = load(e, start_a);
    let u = load(e, direction_a);
    let c = load(e, start_b);
    let v = load(e, direction_b);
    let r = sub(c, a);
    // `MULPS` with `v` as the destination.
    let uv = dot3(v, u);
    let ur = dot3(u, r);
    let vr = dot3(v, r);
    let uu = dot3(u, u);
    let vv = dot3(v, v);
    let (uv_d, ur_d, vr_d, uu_d, vv_d) = (
        f64::from(uv),
        f64::from(ur),
        f64::from(vr),
        f64::from(uu),
        f64::from(vv),
    );
    // The x87 arithmetic keeps the x87 choice of NaN; `rounded` is a store to a
    // scratch float.
    let (fadd, fsub, fmul, fdiv) = (x87_add, x87_sub, x87_mul, x87_div);
    let rounded = |value: f64| f32::from_bits(x87_store32(value));
    // The determinant is stored as a float, made positive and reloaded.
    let determinant = rounded(fsub(fmul(uu_d, vv_d), fmul(uv_d, uv_d)));
    let denominator = x87_quiet(determinant.abs());
    let numerator = rounded(fsub(fmul(ur_d, vv_d), fmul(vr_d, uv_d)));
    let (den_d, num_d) = (f64::from(denominator), f64::from(numerator));

    let mut status;
    let mut s: f32;
    let t: f32;
    if !(fmul(num_d, den_d) >= fmul(den_d, den_d)) {
        if numerator <= 0.0 {
            s = 0.0;
            status = 2;
        } else {
            // The constant is the double 2^-20 at 0x010cb420.
            let sum = fadd(fmul(uu_d, vv_d), fmul(uv_d, uv_d));
            let scaled = rounded(fmul(sum, f64::from_bits(0x3eb0_0000_0000_0000)));
            if scaled < denominator {
                s = rounded(fdiv(num_d, den_d));
                status = 0;
            } else {
                s = 1.0;
                status = 1;
            }
        }
    } else {
        s = 1.0;
        status = 1;
    }

    let w = rounded(fsub(fmul(uv_d, f64::from(s)), vr_d));
    let mut clamped_t = true;
    if w >= vv {
        status = 4;
        t = 1.0;
    } else if w <= 0.0 {
        status = 8;
        t = 0.0;
    } else {
        t = rounded(fdiv(f64::from(w), vv_d));
        clamped_t = false;
    }
    if clamped_t {
        let projected = rounded(fadd(fmul(uv_d, f64::from(t)), ur_d));
        if projected <= 0.0 {
            status |= 2;
            s = 0.0;
        } else if uu <= projected {
            status |= 1;
            s = 1.0;
        } else {
            s = rounded(fdiv(f64::from(projected), uu_d));
        }
    }

    let on_a = scaled_add(s, u, a);
    let on_b = scaled_add(t, v, c);
    let difference = sub(on_a, on_b);
    store(e, out.addr(), on_a);
    store(e, out.addr() + 0x10, difference);
    let squared = dot3(difference, difference);
    e.mem.set_f32(out.addr() + 0x20, x87_quiet(squared));
    status
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d230e0,
            hk_line_segment_closest_point(Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00d22e40,
            hk_line_segment_closest_line_seg_line_seg(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(e: &mut Engine, v: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(0x40);
        for (i, c) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, c);
        }
        e.mem.set_f32(at + 12, 1.0);
        Ptr::new(at)
    }

    fn read(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn closest_point(p: [f32; 3]) -> (u32, [f32; 3]) {
        let mut e = Engine::new();
        let point = vector(&mut e, p);
        let a = vector(&mut e, [0.0, 0.0, 0.0]);
        let b = vector(&mut e, [4.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0; 3]);
        let status = e.call(0x00d230e0, &args![point, a, b, out]).u32();
        (status, read(&e, out.addr()))
    }

    #[test]
    fn point_before_the_start_gives_the_start() {
        assert_eq!(closest_point([-1.0, 2.0, 0.0]), (8, [0.0, 0.0, 0.0]));
    }

    #[test]
    fn point_beyond_the_end_gives_the_end() {
        assert_eq!(closest_point([5.0, 2.0, 0.0]), (4, [4.0, 0.0, 0.0]));
    }

    #[test]
    fn point_beside_the_segment_projects_onto_it() {
        assert_eq!(closest_point([1.0, 3.0, 0.0]), (0, [1.0, 0.0, 0.0]));
    }

    type Closest = (u32, [f32; 3], [f32; 3], f32);

    fn segments(c: [f32; 3]) -> Closest {
        let mut e = Engine::new();
        let a = vector(&mut e, [0.0, 0.0, 0.0]);
        let u = vector(&mut e, [2.0, 0.0, 0.0]);
        let c = vector(&mut e, c);
        let v = vector(&mut e, [0.0, 2.0, 0.0]);
        let out = e.mem.alloc(0x40);
        let status = e.call(0x00d22e40, &args![a, u, c, v, out]).u32();
        (
            status,
            read(&e, out),
            read(&e, out + 0x10),
            e.mem.f32(out + 0x20),
        )
    }

    #[test]
    fn crossing_segments_meet_in_their_middles() {
        let (status, on_a, difference, squared) = segments([1.0, -1.0, 1.0]);
        assert_eq!(status, 0);
        assert_eq!(on_a, [1.0, 0.0, 0.0]);
        assert_eq!(difference, [0.0, 0.0, -1.0]);
        assert_eq!(squared, 1.0);
    }

    #[test]
    fn second_segment_clamps_at_its_end() {
        let (status, on_a, difference, squared) = segments([1.0, -5.0, 1.0]);
        assert_eq!(status, 4);
        assert_eq!(on_a, [1.0, 0.0, 0.0]);
        assert_eq!(difference, [0.0, 3.0, -1.0]);
        assert_eq!(squared, 10.0);
    }

    #[test]
    fn second_segment_clamps_at_its_start() {
        // The crossing lies before the start of B: t = 0.
        let (status, ..) = segments([1.0, 5.0, 1.0]);
        assert_eq!(status, 8);
    }
}
