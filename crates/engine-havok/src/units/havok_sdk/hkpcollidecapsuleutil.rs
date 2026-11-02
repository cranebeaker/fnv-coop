//! `Havok SDK/hkpcollidecapsuleutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vectors are SSE (`f32` lanes, three-component sums added as `z + (y + x)`, `RSQRTSS`
//! refined by one Newton step); the scalar solving runs on the x87 stack, approximated here in
//! `f64` with a rounding to `f32` wherever the code stores a `float`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;

/// `FLT_EPSILON`, 2^-23 (the float at `0104da4c` and the double at `010c9300`).
const EPSILON: f32 = 1.192_092_9e-7;
/// The double 2^-20 at `010cb420`.
const TWO_POW_MINUS_20: f64 = 9.536_743_164_062_5e-7;
/// The double at `01018a90`: 0.2 rounded to a `float`.
const FIFTH: f64 = 0.200_000_002_980_232_24;

type Vec4 = [f32; 4];

fn load(e: &Engine, at: u32) -> Vec4 {
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

fn sub(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// The sum of the three lane products added as `z + (y + x)`.
fn dot3(a: Vec4, b: Vec4) -> f32 {
    let p = [a[0] * b[0], a[1] * b[1], a[2] * b[2]];
    p[2] + (p[1] + p[0])
}

/// `scale * direction + base` in every lane (`MULPS` then `ADDPS`).
fn scaled_add(scale: f32, direction: Vec4, base: Vec4) -> Vec4 {
    [
        scale * direction[0] + base[0],
        scale * direction[1] + base[1],
        scale * direction[2] + base[2],
        scale * direction[3] + base[3],
    ]
}

/// XORPS with the sign mask: every lane negated.
fn negate(v: Vec4) -> Vec4 {
    v.map(|lane| f32::from_bits(lane.to_bits() ^ 0x8000_0000))
}

/// The vector scaled to unit length with `RSQRTSS` and one Newton step,
/// `(0.5 * r) * (3 - (sum * r) * r)`; a zero vector gives zeros.
fn normalize_estimate(v: Vec4) -> Vec4 {
    let sum = dot3(v, v);
    let estimate = rsqrt_estimate(sum);
    let factor = if sum == 0.0 {
        0.0
    } else {
        (0.5 * estimate) * (3.0 - (sum * estimate) * estimate)
    };
    v.map(|lane| factor * lane)
}

// Translated from 00cd58b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCollideCapsuleUtilManifoldCapsVsCaps` (Xbox PDB), cdecl
/// `(capsule_a, radius_a, capsule_b, radius_b, out)`. A capsule is two 16-byte points, its
/// axis. `out` receives up to three contacts of 0x20 bytes (position at +0, normal at +0x10 and
/// the signed gap in the normal's fourth lane); the first holds the closest approach of the two
/// axes and the other two the ends of the overlap when the axes are almost parallel. The
/// result is a bit set of the clamped axis parameters: bit 0 `s` = 1, bit 1 `s` = 0 (A's
/// parameter), bit 2 `t` = 1, bit 3 `t` = 0 (B's).
///
/// The gap already in the first contact (`out + 0x1c`, left by the caller) is raised to
/// `FLT_EPSILON`; when the axes are at least that far apart beyond the radii (gap above it)
/// the call only stores the difference scaled by `1 / distance` as the normal and the new gap,
/// leaving the contact position alone.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn hk_collide_capsule_util_manifold_caps_vs_caps(
    e: &mut Engine,
    capsule_a: Ptr,
    radius_a: f32,
    capsule_b: Ptr,
    radius_b: f32,
    out: Ptr,
) -> u32 {
    let a_start = load(e, capsule_a.addr());
    let a_end = load(e, capsule_a.addr() + 0x10);
    let b_start = load(e, capsule_b.addr());
    let b_end = load(e, capsule_b.addr() + 0x10);
    let out = out.addr();

    let mut v = sub(b_end, b_start);
    let mut u = sub(a_end, a_start);
    let mut uv = x87_quiet(dot3(v, u));
    // Axis A is walked so that the two directions do not point apart.
    let (start, end) = if uv < 0.0 {
        uv = x87_quiet(-uv);
        u = negate(u);
        (a_end, a_start)
    } else {
        (a_start, a_end)
    };
    let r = sub(b_start, start);
    let ur = dot3(r, u);
    let vr = dot3(r, v);
    let uu = dot3(u, u);
    let vv = dot3(v, v);
    let (uv_d, ur_d, vr_d, uu_d, vv_d) = (
        f64::from(uv),
        f64::from(ur),
        f64::from(vr),
        f64::from(uu),
        f64::from(vv),
    );
    let uuvv = vv_d * uu_d;
    let determinant = (uuvv - uv_d * uv_d) as f32;
    let den = x87_quiet(determinant.abs());
    let den_d = f64::from(den);
    let num = (vv_d * ur_d - uv_d * vr_d) as f32;
    let num_d = f64::from(num);

    let mut status: u32;
    let mut s: f32;
    if num_d * den_d >= den_d * den_d {
        s = 1.0;
        status = 1;
    } else if num <= 0.0 {
        s = 0.0;
        status = 2;
    } else {
        let magnitude = x87_quiet((uuvv as f32).abs());
        let threshold = ((uv_d * uv_d + f64::from(magnitude)) * TWO_POW_MINUS_20) as f32;
        if threshold < den {
            s = (num_d / den_d) as f32;
            status = 0;
        } else {
            s = 1.0;
            status = 1;
        }
    }

    // The parameter on B: its own clamps replace the status bits when it leaves 0..1.
    let w = (uv_d * f64::from(s) - vr_d) as f32;
    let t: f32;
    let mut clamped_t = true;
    if w >= vv {
        status = 4;
        t = 1.0;
    } else if w <= 0.0 {
        status = 8;
        t = 0.0;
    } else {
        t = (f64::from(w) / vv_d) as f32;
        clamped_t = false;
    }
    if clamped_t {
        let projected = (uv_d * f64::from(t) + ur_d) as f32;
        if projected <= 0.0 {
            status |= 2;
            s = 0.0;
        } else if uu <= projected {
            status |= 1;
            s = 1.0;
        } else {
            s = (f64::from(projected) / uu_d) as f32;
        }
    }

    let radius_sum = (f64::from(radius_a) + f64::from(radius_b)) as f32;
    let on_a = scaled_add(s, u, start);
    let on_b = scaled_add(t, v, b_start);
    let difference = sub(on_a, on_b);
    let previous_gap = x87_quiet(e.mem.f32(out + 0x1c));
    let min_gap = if previous_gap < EPSILON {
        EPSILON
    } else {
        previous_gap
    };
    let distance = x87_quiet(dot3(difference, difference).sqrt());
    store(e, out + 0x10, difference);
    let inverse = (1.0 / f64::from(distance)) as f32;
    let gap = (f64::from(distance) - f64::from(radius_sum)) as f32;

    if !(min_gap > gap) {
        // Far enough apart: the normal is the scaled difference and the gap is stored.
        let scaled = difference.map(|lane| inverse * lane);
        store(e, out + 0x10, scaled);
        e.mem.set_f32(out + 0x1c, gap);
        return status;
    }
    let mut distance = distance;
    if EPSILON > distance {
        // The axes touch: the normal is any perpendicular of the direction of A.
        distance = 0.0;
        let magnitude = [u[0].abs(), u[1].abs(), u[2].abs()];
        // `pair` is the axis index swapped with the smallest-magnitude one (0 or 1), `other`
        // the component the perpendicular is built from (2 unless it is the smallest).
        let (pair, other, smallest) = if magnitude[0] > magnitude[1] {
            (0usize, 1usize, magnitude[1])
        } else {
            (1, 0, magnitude[0])
        };
        let third = if magnitude[2] < smallest { other } else { 2 };
        let mut normal = [0.0f32; 4];
        normal[pair] = x87_quiet(u[third]);
        normal[third] = x87_quiet(-u[pair]);
        store(e, out + 0x10, normalize_estimate(normal));
    } else {
        let scaled = difference.map(|lane| inverse * lane);
        store(e, out + 0x10, scaled);
    }
    let depth = x87_quiet((f64::from(radius_b) - f64::from(distance)) as f32);
    let normal = load(e, out + 0x10);
    e.mem.set_f32(out + 0x1c, gap);
    store(e, out, scaled_add(depth, normal, on_a));
    if FIFTH * uu_d * vv_d < den_d {
        return status;
    }

    // The overlap of almost parallel axes: a contact at each end of the shorter stretch.
    let inverse_product = (1.0 / uuvv) as f32;
    let f3 = (f64::from(inverse_product) * vv_d) as f32;
    let f2 = (f64::from(inverse_product) * uu_d) as f32;
    let mut nvr = x87_quiet(-vr);
    let mut ur_loop = x87_quiet(ur);
    let mut s_point = start;
    let mut b_point = b_start;
    let mut slot = out + 0x20;
    for iteration in 0..2 {
        let mask = if iteration == 0 { 10 } else { 5 };
        if status & mask == 0 {
            'contact: {
                let mut s_side = false;
                if nvr > 0.0 {
                    if vv < nvr {
                        break 'contact;
                    }
                    if ur_loop > 0.0 {
                        if uu < ur_loop {
                            break 'contact;
                        }
                        let (ur_d, nvr_d) = (f64::from(ur_loop), f64::from(nvr));
                        s_side = ur_d * ur_d * f64::from(f3) < nvr_d * nvr_d * f64::from(f2);
                    }
                    if !s_side {
                        let step = (f64::from(nvr) * f64::from(f2)) as f32;
                        b_point = scaled_add(step, v, b_point);
                    }
                } else if ur_loop > 0.0 {
                    if uu < ur_loop {
                        break 'contact;
                    }
                    s_side = true;
                }
                if s_side {
                    let step = (f64::from(ur_loop) * f64::from(f3)) as f32;
                    s_point = scaled_add(step, u, s_point);
                }

                let offset = sub(s_point, b_point);
                let squared = dot3(offset, offset);
                let slot_gap = x87_quiet(e.mem.f32(slot + 0x1c));
                let limit = (f64::from(slot_gap) + f64::from(radius_sum)) as f32;
                let limit_squared = (f64::from(limit) * f64::from(limit)) as f32;
                if !(limit_squared > squared) {
                    break 'contact;
                }
                let root = e.call(X87_SQRT, &args![f64::from(squared)]).f32();
                let normal = if EPSILON < squared {
                    normalize_estimate(offset)
                } else {
                    load(e, out + 0x10)
                };
                store(e, slot + 0x10, normal);
                let depth = (f64::from(radius_b) - f64::from(root)) as f32;
                let contact_gap = (f64::from(root) - f64::from(radius_sum)) as f32;
                e.mem.set_f32(slot + 0x1c, contact_gap);
                store(e, slot, scaled_add(depth, normal, s_point));
            }
        }
        if iteration == 1 {
            break;
        }
        // The second pass walks both axes from their far ends.
        let (uv_d, uu_d, vv_d) = (f64::from(uv), f64::from(uu), f64::from(vv));
        let next_nvr = ((vv_d - f64::from(nvr)) - uv_d) as f32;
        let next_ur = ((uu_d - f64::from(ur_loop)) - uv_d) as f32;
        s_point = end;
        b_point = b_end;
        u = negate(u);
        v = negate(v);
        slot += 0x20;
        nvr = next_nvr;
        ur_loop = next_ur;
    }
    status
}

/// `hkLineSegmentUtil::closestLineSegLineSeg` (Xbox PDB), cdecl
/// `(start_a, direction_a, start_b, direction_b, out)`: the point on A at `out`, the vector from
/// the point on B to it at `out + 0x10`, its squared length at `out + 0x20`; returns the clamp
/// bits.
const CLOSEST_LINE_SEG_LINE_SEG: u32 = 0x00d2_2e40;

/// The float at `0104da48` (`0x7f7fffee`, just below `FLT_MAX`): the gap of an empty contact.
const LARGE_GAP: u32 = 0x0104_da48;
/// The float at `0104e148`, the negative of [`LARGE_GAP`].
const LARGE_NEGATIVE_GAP: u32 = 0x0104_e148;
/// Four floats 1.0 at `010ec7f0`.
const ONES: u32 = 0x010e_c7f0;
/// The zero vector at `01267e30` (`.data`).
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// Four words `0x80000000` at `010c44d0`, the `XORPS` sign mask.
const SIGN_MASK: u32 = 0x010c_44d0;
/// The float 0.0 at `01011d78`.
const ZERO: u32 = 0x0101_1d78;
/// The float 3.0 at `01017718`.
const THREE: u32 = 0x0101_7718;
/// The float 0.5 at `01016248`.
const HALF: u32 = 0x0101_6248;
/// The float 2^-46 at `010cb428`: a squared distance at or below it counts as touching.
const TOUCH_EPSILON: u32 = 0x010c_b428;
/// The float 1.0001 at `010cb42c`, the upper bound of the capsule parameter of an edge contact.
const PARAMETER_ABOVE: u32 = 0x010c_b42c;
/// The double at `010cb430` (-0.0001 rounded to a float), the lower bound of that parameter.
const PARAMETER_BELOW: u32 = 0x010c_b430;
/// The bytes `2, 0, 1, 2, 0` at `010cb440`: edge `i` runs from vertex `[010cb442 + i]` to vertex
/// `[010cb440 + i]` (edge 0: 1 to 2, edge 1: 2 to 0, edge 2: 0 to 1).
const EDGE_VERTICES: u32 = 0x010c_b440;

fn add(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

fn mul(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

fn splat(value: f32) -> Vec4 {
    [value; 4]
}

/// `SHUFPS v,v,0xC9`: `(y, z, x, w)`.
fn yzx(v: Vec4) -> Vec4 {
    [v[1], v[2], v[0], v[3]]
}

/// `SHUFPS v,v,0xD2`: `(z, x, y, w)`.
fn zxy(v: Vec4) -> Vec4 {
    [v[2], v[0], v[1], v[3]]
}

/// `CMPLTPS` then `MOVMSKPS`: bit `i` set where `a[i] < b[i]` (clear for NaN).
fn less_mask(a: Vec4, b: Vec4) -> u32 {
    (0..4).filter(|&i| a[i] < b[i]).map(|i| 1u32 << i).sum()
}

/// `XORPS` with the sign mask read from `010c44d0`.
fn flip(e: &Engine, v: Vec4) -> Vec4 {
    let mask = load(e, SIGN_MASK);
    [0, 1, 2, 3].map(|i| f32::from_bits(v[i].to_bits() ^ mask[i].to_bits()))
}

/// The cross product of two directions as the code forms it,
/// `zxy(b) * yzx(a) - yzx(b) * zxy(a)`: `a x b` in lanes 0..2.
fn cross(a: Vec4, b: Vec4) -> Vec4 {
    sub(mul(zxy(b), yzx(a)), mul(yzx(b), zxy(a)))
}

/// The vector scaled to unit length with `RSQRTSS` and one Newton step,
/// `(0.5 * r) * (3 - (sum * r) * r)`, the constants read from the exe; a squared length equal
/// to the 0.0 at `01011d78` gives zeros.
fn normalize(e: &Engine, v: Vec4) -> Vec4 {
    let sum = dot3(v, v);
    let estimate = rsqrt_estimate(sum);
    let factor = if sum == e.mem.f32(ZERO) {
        0.0
    } else {
        (e.mem.f32(HALF) * estimate) * (e.mem.f32(THREE) - (sum * estimate) * estimate)
    };
    v.map(|lane| factor * lane)
}

/// The address of the triangle's vertex `index` (a sign-extended table byte).
fn vertex(triangle: Ptr, index: i8) -> u32 {
    triangle.addr().wrapping_add((i32::from(index) << 4) as u32)
}

/// A feature entry for an edge: kind 1 (the edge's first vertex) or 2 (both vertices).
fn set_edge_feature(e: &mut Engine, features: u32, slot: usize, kind: u8, first: i8, second: i8) {
    let at = features + 8 * slot as u32;
    e.mem.set_u16(at + 2, i16::from(first) as u16);
    e.mem.set_u8(at, kind);
    if kind == 2 {
        e.mem.set_u16(at + 4, i16::from(second) as u16);
    }
}

/// A feature entry of kind 3: the face, vertices 0, 1 and 2.
fn set_face_feature(e: &mut Engine, features: u32, slot: usize) {
    let at = features + 8 * slot as u32;
    e.mem.set_u8(at, 3);
    e.mem.set_u16(at + 2, 0);
    e.mem.set_u16(at + 4, 1);
    e.mem.set_u16(at + 6, 2);
}

/// What the parts of `hkCollideCapsuleUtilCapsVsTri` share: the arguments (the floats as
/// loaded on the x87 stack), the triangle's normal and edge planes and the capsule's ends in
/// the triangle's frame.
struct CapsuleTriangle {
    capsule: Ptr,
    capsule_radius: f32,
    triangle: Ptr,
    triangle_radius: f32,
    tolerance: f32,
    out: u32,
    features: u32,
    /// The capsule radius plus the triangle radius, as stored.
    radius_sum: f32,
    /// The float at [`LARGE_GAP`] as loaded.
    large: f32,
    /// The scaled cross product of two edges (lane 3 as computed).
    normal: Vec4,
    /// The rows of the frame: lane `i` holds edge plane `i`'s x, y and z, lane 3 the normal's
    /// (the entries moved by the transposition went through the x87 stack).
    rows: [Vec4; 3],
    /// The capsule's ends in the frame: the three edge-plane distances (negative inside) and
    /// the height over the triangle's plane.
    ends: [Vec4; 2],
    /// `MOVMSKPS` of `ends[i] < 0`.
    masks: [u32; 2],
}

// Translated from 00cd6520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCollideCapsuleUtilCapsVsTri` (Xbox PDB), cdecl, 9 words: `capsule` (two 16-byte points),
/// `capsule_radius`, `triangle` (three 16-byte vertices), `triangle_radius`, `cache` (five
/// floats: the scales of the three edges at +0..+8, the normal's scale at +0xC and at +0x10 the
/// value whose product with the first scale places the first edge plane; the Havok name of this
/// block is unconfirmed), `tolerance`, `search_manifold`, `out` (three contacts of 0x20 bytes:
/// position, normal, gap in the normal's fourth lane) and `features` (null, or three entries of
/// 8 bytes: a kind byte, then 16-bit vertex indices at +2, +4, +6).
///
/// The capsule's ends are taken into the triangle's frame (three edge-plane distances and the
/// height over the plane). Every gap starts at the float just below `FLT_MAX`; the call ends
/// there when both ends lie beyond `radius_sum + tolerance` outside one edge plane or above the
/// plane, or both below `-(radius_sum + tolerance)`. Then:
/// - the axis crosses the plane inside the triangle: contact 1 holds the deepest end inside the
///   triangle's prism or the best edge crossing (every feature entry 3, 0, 1, 2);
/// - both ends together have every edge plane behind them: one contact per end, a face contact
///   for an end inside the prism, else the closest of its edges;
/// - else with `search_manifold`: the capsule against each edge outside both ends' planes as a
///   capsule (`hkCollideCapsuleUtilManifoldCapsVsCaps`), keeping the smaller first gap;
/// - else: the closest of those edges in contact 0.
#[allow(clippy::too_many_arguments)]
pub fn hk_collide_capsule_util_caps_vs_tri(
    e: &mut Engine,
    capsule: Ptr,
    capsule_radius: f32,
    triangle: Ptr,
    triangle_radius: f32,
    cache: Ptr,
    tolerance: f32,
    search_manifold: u32,
    out: Ptr,
    features: Ptr,
) {
    let (out, features) = (out.addr(), features.addr());
    if features != 0 {
        for slot in 0..3 {
            e.mem.set_u8(features + 8 * slot, 0);
        }
    }
    let vertices = [0, 0x10, 0x20].map(|offset| load(e, triangle.addr() + offset));
    let scale = |e: &Engine, offset: u32| e.mem.f32(cache.addr() + offset);
    let opposite_0 = sub(vertices[2], vertices[1]);
    let opposite_1 = sub(vertices[0], vertices[2]);
    let opposite_2 = sub(vertices[1], vertices[0]);
    let normal = mul(cross(opposite_0, opposite_1), splat(scale(e, 0xc)));
    // The edge planes: each scaled edge crossed with the normal.
    let planes = [
        mul(splat(scale(e, 0)), opposite_0),
        mul(splat(scale(e, 4)), opposite_1),
        mul(splat(scale(e, 8)), opposite_2),
    ]
    .map(|edge| sub(mul(yzx(edge), zxy(normal)), mul(zxy(edge), yzx(normal))));
    // Transposed in place on the stack; every float it moves goes through FLD/FSTP.
    let rows: [Vec4; 3] = [0, 1, 2].map(|row| {
        [0, 1, 2, 3].map(|column| {
            if column == 3 {
                x87_quiet(normal[row])
            } else if column == row {
                planes[column][row]
            } else {
                x87_quiet(planes[column][row])
            }
        })
    });
    let first_offset = (f64::from(scale(e, 0x10)) * -f64::from(scale(e, 0))) as f32;
    let offset = [first_offset, 0.0, 0.0, 0.0];
    let capsule_start = load(e, capsule.addr());
    let capsule_end = load(e, capsule.addr() + 0x10);
    let in_frame = |point: Vec4| {
        let relative = sub(point, vertices[0]);
        add(
            add(
                mul(splat(relative[1]), rows[1]),
                mul(splat(relative[0]), rows[0]),
            ),
            add(mul(splat(relative[2]), rows[2]), offset),
        )
    };
    let ends = [in_frame(capsule_start), in_frame(capsule_end)];

    let capsule_radius = x87_quiet(capsule_radius);
    let triangle_radius = x87_quiet(triangle_radius);
    let tolerance = x87_quiet(tolerance);
    let radius_sum = (f64::from(capsule_radius) + f64::from(triangle_radius)) as f32;
    let limit = (f64::from(radius_sum) + f64::from(tolerance)) as f32;
    let large = x87_quiet(e.mem.f32(LARGE_GAP));
    for slot in 0..3 {
        e.mem.set_f32(out + 0x20 * slot + 0x1c, large);
    }
    if less_mask(splat(limit), ends[0]) & less_mask(splat(limit), ends[1]) != 0 {
        return;
    }
    let below = -limit;
    let start_height = x87_quiet(ends[0][3]);
    let end_height = x87_quiet(ends[1][3]);
    if below > start_height && end_height < below {
        return;
    }
    let zero = [0.0; 4];
    let masks = [less_mask(ends[0], zero), less_mask(ends[1], zero)];
    let frame = CapsuleTriangle {
        capsule,
        capsule_radius,
        triangle,
        triangle_radius,
        tolerance,
        out,
        features,
        radius_sum,
        large,
        normal,
        rows,
        ends,
        masks,
    };

    if (masks[0] ^ masks[1]) & 8 != 0 {
        // The ends are on both sides of the plane: where the axis crosses it.
        let fraction =
            (f64::from(start_height) / (f64::from(start_height) - f64::from(end_height))) as f32;
        let crossing = add(
            mul(sub(load(e, ONES), splat(fraction)), ends[0]),
            mul(ends[1], splat(fraction)),
        );
        if less_mask(crossing, load(e, ZERO_VECTOR)) & 7 == 7 {
            pierce_contacts(e, &frame, start_height);
            return;
        }
    }

    // The frame's locals from 0xd0: the capsule's direction, at 0xe0 and 0xf0 an edge capsule
    // or (0xe0) an edge's direction, at 0x100 the closest-segments result.
    e.with_stack(0x60, |e, locals| {
        let locals = locals.addr();
        store(e, locals, sub(capsule_end, capsule_start));
        let either = masks[0] | masks[1];
        if either & 7 == 7 {
            end_contacts(e, &frame, locals);
        } else if search_manifold != 0 {
            manifold_contacts(e, &frame, either, locals);
        } else {
            edge_contacts(e, &frame, 0, true, out, locals);
        }
    });
}

/// The axis crosses the plane inside the triangle (`00cd685d`..`00cd6be0`). Contact 1 starts at
/// the negative large gap; each end inside the prism whose depth (its distance from the plane
/// plus the radii, negated) is above the contact's gap replaces it (the end moved along the
/// normal by the triangle's radius less its height, or for an end above the plane by the
/// negated radius less its height; normal `normal` for an end below the plane, else its
/// negation). Then each edge whose line passes closer to the axis than the contact's gap plus
/// the radii, at an axis parameter strictly within -0.0001..1.0001, replaces it: the normal
/// from the edge plane and the plane normal, the point on the axis moved along it.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn pierce_contacts(e: &mut Engine, frame: &CapsuleTriangle, start_height: f32) {
    let radius_sum = frame.radius_sum;
    let triangle_radius = frame.triangle_radius;
    let contact = frame.out + 0x20;
    let deepest = x87_quiet(e.mem.f32(LARGE_NEGATIVE_GAP));
    e.mem.set_f32(contact + 0x1c, deepest);
    for end in 0..2 {
        let mask = frame.masks[end];
        if mask & 7 != 7 {
            continue;
        }
        let height = frame.ends[end][3];
        let size = x87_quiet(x87_quiet(height).abs());
        let depth = (-f64::from(size) - f64::from(radius_sum)) as f32;
        if !(x87_quiet(e.mem.f32(contact + 0x1c)) < depth) {
            continue;
        }
        let (normal, shift) = if mask & 8 != 0 {
            (
                frame.normal,
                (f64::from(triangle_radius) - f64::from(height)) as f32,
            )
        } else {
            (
                flip(e, frame.normal),
                (-f64::from(triangle_radius) - f64::from(height)) as f32,
            )
        };
        store(e, contact + 0x10, normal);
        let point = load(e, frame.capsule.addr() + 0x10 * end as u32);
        e.mem.set_f32(contact + 0x1c, depth);
        store(e, contact, add(mul(splat(shift), frame.normal), point));
    }

    // The axis in the frame: its change in each edge-plane distance and in height.
    let along = sub(frame.ends[1], frame.ends[0]);
    let rise = x87_quiet(along[3]);
    let rise_squared = f64::from(rise) * f64::from(rise);
    let (rise_d, start_d) = (f64::from(rise), f64::from(start_height));
    for edge in 0..3 {
        let component = x87_quiet(along[edge]);
        let component_d = f64::from(component);
        let length = (component_d * component_d + rise_squared) as f32;
        let inverse = (1.0 / f64::from(length)) as f32;
        let start_distance = f64::from(frame.ends[0][edge]);
        let crossing = (component_d * start_d - start_distance * rise_d) as f32;
        let squared = ((f64::from(crossing) * f64::from(crossing)) * f64::from(inverse)) as f32;
        let gap = x87_quiet(e.mem.f32(contact + 0x1c));
        let reach = (f64::from(gap) + f64::from(radius_sum)) as f32;
        if !(f64::from(squared) < f64::from(reach) * f64::from(reach)) {
            continue;
        }
        let projection = (start_distance * component_d + rise_d * start_d) as f32;
        let parameter = (f64::from(projection) * -f64::from(inverse)) as f32;
        if !(f64::from(parameter) > e.mem.f64(PARAMETER_BELOW)) {
            continue;
        }
        if !(parameter < e.mem.f32(PARAMETER_ABOVE)) {
            continue;
        }
        let plane = [
            frame.rows[0][edge],
            frame.rows[1][edge],
            frame.rows[2][edge],
            e.mem.f32(ZERO),
        ];
        let (rise_weight, component) = if 0.0 > rise {
            (-rise, x87_quiet(-component))
        } else {
            (rise, component)
        };
        let normal = add(
            mul(splat(rise_weight), plane),
            mul(splat(-component), frame.normal),
        );
        store(e, contact + 0x10, normalize(e, normal));
        let distance = e.call(X87_SQRT, &args![f64::from(squared)]).f32();
        let negative = -distance;
        let shift = (f64::from(triangle_radius) - f64::from(negative)) as f32;
        let start = load(e, frame.capsule.addr());
        let end = load(e, frame.capsule.addr() + 0x10);
        let on_axis = add(
            mul(sub(load(e, ONES), splat(parameter)), start),
            mul(splat(parameter), end),
        );
        let normal = load(e, contact + 0x10);
        let position = add(on_axis, mul(splat(shift), normal));
        let gap = (f64::from(negative) - f64::from(radius_sum)) as f32;
        e.mem.set_f32(contact + 0x1c, gap);
        store(e, contact, position);
    }

    if frame.features != 0 {
        for slot in 0..3 {
            set_face_feature(e, frame.features, slot);
        }
    }
}

/// Both ends together have every edge plane behind them (`00cd6d5c`..`00cd7140`): one contact
/// per end, in order from `out`. An end inside the prism gets the face contact (the end moved
/// along `normal` as in the crossing case, the normal `normal` or its negation for an end below
/// the plane, the gap its distance from the plane less the radii, feature 3, 0, 1, 2); another
/// end the closest of the edges whose plane it is outside of.
fn end_contacts(e: &mut Engine, frame: &CapsuleTriangle, locals: u32) {
    let mut contact = frame.out;
    for end in 0..2 {
        let mask = frame.masks[end];
        if mask & 7 != 7 {
            edge_contacts(e, frame, end, false, contact, locals);
        } else {
            let point = load(e, frame.capsule.addr() + 0x10 * end as u32);
            let height = frame.ends[end][3];
            let radius = f64::from(frame.triangle_radius);
            let (shift, normal, signed) = if mask & 8 != 0 {
                (
                    (-radius - f64::from(height)) as f32,
                    flip(e, frame.normal),
                    -x87_quiet(height),
                )
            } else {
                (
                    (radius - f64::from(height)) as f32,
                    frame.normal,
                    x87_quiet(height),
                )
            };
            store(e, contact, add(mul(splat(shift), frame.normal), point));
            store(e, contact + 0x10, normal);
            let gap = (f64::from(signed) - f64::from(frame.radius_sum)) as f32;
            e.mem.set_f32(contact + 0x1c, gap);
            if frame.features != 0 {
                set_face_feature(e, frame.features, end);
            }
        }
        contact += 0x20;
    }
}

/// With `search_manifold` (`00cd6c22`..`00cd6d23`): the three gaps start at `tolerance`; each
/// edge whose bit is clear in `either` (both ends' plane masks) goes as a capsule of the
/// triangle's radius against the capsule (`hkCollideCapsuleUtilManifoldCapsVsCaps` into
/// `out`). A copy of the first contact keeps the best: when the copy's gap is below the new
/// first gap the copy is put back, else the new first contact is copied. The copy starts as
/// the ends in the frame with the large gap in its last word (the stack slots it reuses).
fn manifold_contacts(e: &mut Engine, frame: &CapsuleTriangle, either: u32, locals: u32) {
    let out = frame.out;
    for slot in 0..3 {
        e.mem.set_f32(out + 0x20 * slot + 0x1c, frame.tolerance);
    }
    let mut best = [0u32; 8];
    for (word, value) in best.iter_mut().zip(frame.ends.iter().flatten()) {
        *word = value.to_bits();
    }
    best[7] = frame.large.to_bits();
    let pair = locals + 0x10;
    for edge in 0..3u32 {
        if either & (1 << edge) != 0 {
            continue;
        }
        let first = e.mem.i8(EDGE_VERTICES + 2 + edge);
        let second = e.mem.i8(EDGE_VERTICES + edge);
        let from = load(e, vertex(frame.triangle, first));
        store(e, pair, from);
        let to = load(e, vertex(frame.triangle, second));
        store(e, pair + 0x10, to);
        let status = hk_collide_capsule_util_manifold_caps_vs_caps(
            e,
            frame.capsule,
            frame.capsule_radius,
            Ptr::new(pair),
            frame.triangle_radius,
            Ptr::new(out),
        );
        if frame.features != 0 {
            let kind = if status & 0xc != 0 { 1 } else { 2 };
            for slot in 0..3 {
                set_edge_feature(e, frame.features, slot, kind, first, second);
            }
        }
        let gap = x87_quiet(e.mem.f32(out + 0x1c));
        if x87_quiet(f32::from_bits(best[7])) < gap {
            for (i, word) in best.iter().enumerate() {
                e.mem.set_u32(out + 4 * i as u32, *word);
            }
        } else {
            for (i, word) in best.iter_mut().enumerate() {
                *word = e.mem.u32(out + 4 * i as u32);
            }
        }
    }
}

/// The edges for one contact (`00cd6d2d`..`00cd712d`): `single` (no manifold) takes the edges
/// outside both ends' planes into contact 0, otherwise the edges outside end `end`'s planes go
/// into `contact`. Each is `closestLineSegLineSeg` of the capsule's axis and the edge (feature
/// entry `end`: kind 1 with the edge's first vertex when the edge's parameter was clamped, else
/// kind 2 with both). The edge replaces the contact when the square of the contact's gap plus
/// the radii is above its squared distance, unless the axis parameter was clamped at the end's
/// bit (`1 << end`): then a contact for one end skips it, and `single` takes it but counts the
/// later edges for end 1 (bit 2, feature entry 1). The normal is the cross product of the two
/// directions when nothing was clamped, else the vector between the closest points, or when
/// they touch the edge plane (or that cross product if it is not degenerate); it is turned to
/// have a positive distance along it, and the gap is that distance less the radii.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn edge_contacts(
    e: &mut Engine,
    frame: &CapsuleTriangle,
    end: usize,
    single: bool,
    contact: u32,
    locals: u32,
) {
    let mut feature_slot = end;
    let mut end_bit = 1u32 << end;
    let edge_direction_at = locals + 0x10;
    let result = locals + 0x30;
    for edge in 0..3u32 {
        let outside = if single {
            frame.masks[1] & frame.masks[0]
        } else {
            frame.masks[end]
        };
        if outside & (1 << edge) != 0 {
            continue;
        }
        let tip = e.mem.i8(EDGE_VERTICES + edge);
        let base = e.mem.i8(EDGE_VERTICES + 2 + edge);
        let base_at = vertex(frame.triangle, base);
        let edge_direction = sub(load(e, vertex(frame.triangle, tip)), load(e, base_at));
        store(e, edge_direction_at, edge_direction);
        let status = e
            .call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![frame.capsule, locals, base_at, edge_direction_at, result],
            )
            .u32();
        if frame.features != 0 {
            let kind = if status & 0xc != 0 { 1 } else { 2 };
            set_edge_feature(e, frame.features, feature_slot, kind, base, tip);
        }
        let gap = x87_quiet(e.mem.f32(contact + 0x1c));
        let reach = (f64::from(gap) + f64::from(frame.radius_sum)) as f32;
        let reach_squared = (f64::from(reach) * f64::from(reach)) as f32;
        let squared = x87_quiet(e.mem.f32(result + 0x20));
        if !(reach_squared > squared) {
            continue;
        }
        if end_bit & status != 0 {
            if !single {
                continue;
            }
            feature_slot = 1;
            end_bit = 2;
        }
        let capsule_direction = load(e, locals);
        let edge_direction = load(e, edge_direction_at);
        let (direction, projected) = if status == 0 {
            (
                cross(capsule_direction, edge_direction),
                load(e, result + 0x10),
            )
        } else {
            let epsilon = x87_quiet(e.mem.f32(TOUCH_EPSILON));
            if !(epsilon >= squared) {
                let difference = load(e, result + 0x10);
                (difference, difference)
            } else {
                // Touching: the edge plane replaces the vector between the points.
                let plane = [
                    frame.rows[0][edge as usize],
                    frame.rows[1][edge as usize],
                    frame.rows[2][edge as usize],
                    e.mem.f32(ZERO),
                ];
                let across = cross(capsule_direction, edge_direction);
                let across_squared = dot3(across, across);
                store(e, result + 0x10, plane);
                if !(epsilon >= across_squared) {
                    (across, plane)
                } else {
                    (plane, plane)
                }
            }
        };
        let normal = normalize(e, direction);
        let along = x87_quiet(dot3(projected, normal));
        let (normal, along) = if 0.0 > along {
            (flip(e, normal), -along)
        } else {
            (normal, along)
        };
        let shift = (f64::from(frame.triangle_radius) - f64::from(along)) as f32;
        let gap = (f64::from(along) - f64::from(frame.radius_sum)) as f32;
        let point = load(e, result);
        store(e, contact, add(mul(splat(shift), normal), point));
        store(e, contact + 0x10, normal);
        e.mem.set_f32(contact + 0x1c, gap);
    }
}

// Translated from 00cd6120 (decompiled, FalloutNV.exe 1.4.0.525)
/// A helper of the unit with register inputs (the map gives it no name; the registers come
/// first, alphabetically, then the stack word; `RET`): `matrix` is EAX, `normal` EDX,
/// `triangle` ESI and `scales` the first stack word. From the three vertices of `triangle`
/// (at `+0`, `+0x10`, `+0x20`) it takes the edges `v2 - v1`, `v1 - v0` and `v0 - v2` and
/// writes the cross product of the first and the third, times `scales[3]`, to `normal` (16
/// bytes). The first, second and third edge scaled by `scales[0]`, `scales[1]` and `scales[2]`
/// are crossed with that normal into the three rows at `matrix` (`+0`, `+0x10`, `+0x20`); then
/// the words `+4`/`+0x10`, `+8`/`+0x20` and `+0x18`/`+0x24` are exchanged (a transpose; x87
/// loads and stores, so a signalling NaN is quieted), the normal's three lanes go to
/// `+0xC`, `+0x1C` and `+0x2C`, and `matrix + 0x30` gets `(scales[4] * -scales[0], 0, 0, 0)`
/// (x87 product rounded to a float).
pub fn fn_00cd6120(e: &mut Engine, matrix: Ptr, normal: Ptr, triangle: Ptr, scales: Ptr) {
    let (matrix, normal, triangle, scales) =
        (matrix.addr(), normal.addr(), triangle.addr(), scales.addr());
    let first = load(e, triangle);
    let second = load(e, triangle + 0x10);
    let third = load(e, triangle + 0x20);
    let edge_second_third = sub(third, second);
    let edge_first_second = sub(second, first);
    let edge_third_first = sub(first, third);
    let factor = e.mem.f32(scales + 0xc);
    let cross_normal = mul(cross(edge_second_third, edge_third_first), splat(factor));
    store(e, normal, cross_normal);
    let along_first = mul(splat(e.mem.f32(scales)), edge_second_third);
    let along_second = mul(splat(e.mem.f32(scales + 4)), edge_third_first);
    let along_third = mul(splat(e.mem.f32(scales + 8)), edge_first_second);
    store(e, matrix, cross(along_first, cross_normal));
    store(e, matrix + 0x10, cross(along_second, cross_normal));
    store(e, matrix + 0x20, cross(along_third, cross_normal));
    for (high, low) in [(0x10u32, 4u32), (0x20, 8), (0x24, 0x18)] {
        let from_low = x87_quiet_bits(e.mem.u32(matrix + low));
        let from_high = x87_quiet_bits(e.mem.u32(matrix + high));
        e.mem.set_u32(matrix + high, from_low);
        e.mem.set_u32(matrix + low, from_high);
    }
    for (lane, offset) in [0xcu32, 0x1c, 0x2c].into_iter().enumerate() {
        let word = x87_quiet_bits(e.mem.u32(normal + 4 * lane as u32));
        e.mem.set_u32(matrix + offset, word);
    }
    store(e, matrix + 0x30, [0.0; 4]);
    let scale = x87_quiet(e.mem.f32(scales + 0x10));
    let reversed = -x87_quiet(e.mem.f32(scales));
    e.mem.set_f32(
        matrix + 0x30,
        (f64::from(scale) * f64::from(reversed)) as f32,
    );
}

/// The frame of `00cd6260`: the directions of the two segments (`+0x18`, `+0x28`) and the
/// answer of `closestLineSegLineSeg` (`+0x38`: the closest point, a vector, the squared
/// distance at `+0x20`).
const DIRECTION_A: u32 = 0x18;
const DIRECTION_B: u32 = 0x28;
const SEGMENTS: u32 = 0x38;
const CLOSEST_FRAME_SIZE: u32 = 0x68;

// Translated from 00cd6260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCollideCapsuleUtilClostestPointCapsVsCaps` (name from the engine map, unconfirmed;
/// cdecl; `RSQRTSS` is the host CPU's estimate): the closest point query of two capsules
/// (`ends_a`, `ends_b`: two end point vectors each). `closestLineSegLineSeg` (`00d22e40`)
/// finds the closest points of the axes; when the squared distance (`+0x20` of its answer) is
/// above the square of `radius_a + radius_b + tolerance` (each sum rounded to a float; x87)
/// the capsules are too far and the answer is 1, `out` untouched. Else the distance is the
/// CRT square root (`00ec6040`) of it, and the contact normal at `out + 0x10` is: when the
/// segment function answered non-zero and the squared distance is above 0 the
/// vector it made at `+0x10`; when it answered 0 the cross product of the two axis directions
/// if its squared length is above `FLT_EPSILON`, turned around when its dot product with the
/// answer's vector is negative; else a vector perpendicular to the first axis made from its
/// two larger components (the smallest of their absolute values is left at 0: `normal[i] =
/// axis[j]`, `normal[j] = -axis[i]`). The normal is scaled to unit length (the `RSQRTSS`
/// estimate refined by one step; zero stays zero), the position `out` is the segment answer's
/// closest point plus `(radius_b - distance)` times the normal, and the gap
/// `distance - (radius_a + radius_b)` replaces the normal's fourth lane. Answers 0.
pub fn hk_collide_capsule_util_clostest_point_caps_vs_caps(
    e: &mut Engine,
    ends_a: Ptr,
    radius_a: f32,
    ends_b: Ptr,
    radius_b: f32,
    tolerance: f32,
    out: Ptr,
) -> u32 {
    let out = out.addr();
    e.with_stack(CLOSEST_FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let direction_a = sub(load(e, ends_a.addr() + 0x10), load(e, ends_a.addr()));
        store(e, frame + DIRECTION_A, direction_a);
        let direction_b = sub(load(e, ends_b.addr() + 0x10), load(e, ends_b.addr()));
        store(e, frame + DIRECTION_B, direction_b);
        let status = e
            .call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![
                    ends_a,
                    frame + DIRECTION_A,
                    ends_b,
                    frame + DIRECTION_B,
                    frame + SEGMENTS
                ],
            )
            .u32();
        let radius_sum = (f64::from(x87_quiet(radius_a)) + f64::from(x87_quiet(radius_b))) as f32;
        let limit = (f64::from(radius_sum) + f64::from(x87_quiet(tolerance))) as f32;
        let squared = e.mem.f32(frame + SEGMENTS + 0x20);
        if f64::from(squared) > f64::from(limit) * f64::from(limit) {
            return 1;
        }
        let distance = e.call(X87_SQRT, &args![f64::from(squared)]).f32();
        let normal = out + 0x10;
        let found = if status != 0 {
            let positive = 0.0 < squared;
            if positive {
                for word in 0..4 {
                    let value = e.mem.u32(frame + SEGMENTS + 0x10 + 4 * word);
                    e.mem.set_u32(normal + 4 * word, value);
                }
            }
            positive
        } else {
            let product = cross(direction_a, direction_b);
            let usable = EPSILON < dot3(product, product);
            if usable {
                let answer = load(e, frame + SEGMENTS + 0x10);
                let turned = if 0.0 > dot3(product, answer) {
                    flip(e, product)
                } else {
                    product
                };
                store(e, normal, turned);
            }
            usable
        };
        if !found {
            let magnitude = |e: &Engine, lane: u32| {
                f32::from_bits(
                    x87_quiet_bits(e.mem.u32(frame + DIRECTION_A + 4 * lane)) & 0x7fff_ffff,
                )
            };
            let (x, y, z) = (magnitude(e, 0), magnitude(e, 1), magnitude(e, 2));
            let (mut low, mut first, mut second) = (0u32, 1u32, 2u32);
            let mut least = x;
            if x > y {
                least = y;
                first = 0;
                low = second - 1;
            }
            if least > z {
                second = low;
            }
            store(e, normal, [0.0; 4]);
            let kept = x87_quiet_bits(e.mem.u32(frame + DIRECTION_A + 4 * second));
            e.mem.set_u32(normal + 4 * first, kept);
            let turned = x87_quiet_bits(e.mem.u32(frame + DIRECTION_A + 4 * first));
            e.mem.set_u32(normal + 4 * second, turned ^ 0x8000_0000);
        }
        let unit = normalize(e, load(e, normal));
        store(e, normal, unit);
        let depth = (f64::from(x87_quiet(radius_b)) - f64::from(distance)) as f32;
        let position = scaled_add(depth, load(e, normal), load(e, frame + SEGMENTS));
        store(e, out, position);
        let gap = (f64::from(distance) - f64::from(radius_sum)) as f32;
        e.mem.set_f32(out + 0x1c, gap);
        0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cd58b0,
            hk_collide_capsule_util_manifold_caps_vs_caps(Ptr, f32, Ptr, f32, Ptr) -> u32
        ),
        entry!(0x00cd6120, fn_00cd6120(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00cd6260,
            hk_collide_capsule_util_clostest_point_caps_vs_caps(Ptr, f32, Ptr, f32, f32, Ptr) -> u32
        ),
        entry!(
            0x00cd6520,
            hk_collide_capsule_util_caps_vs_tri(Ptr, f32, Ptr, f32, Ptr, f32, u32, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(e: &mut Engine, v: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x10);
        for (i, c) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, c);
        }
        at
    }

    fn capsule(e: &mut Engine, from: [f32; 3], to: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(0x20);
        let first = vector(e, from);
        let second = vector(e, to);
        for lane in 0..4 {
            let low = e.mem.u32(first + 4 * lane);
            let high = e.mem.u32(second + 4 * lane);
            e.mem.set_u32(at + 4 * lane, low);
            e.mem.set_u32(at + 0x10 + 4 * lane, high);
        }
        Ptr::new(at)
    }

    /// An output block of three contacts whose gaps start at `gap`.
    fn contacts(e: &mut Engine, gap: f32) -> Ptr {
        let at = e.mem.alloc(0x60);
        for slot in 0..3 {
            e.mem.set_f32(at + 0x20 * slot + 0x1c, gap);
        }
        Ptr::new(at)
    }

    fn read(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn near(actual: [f32; 3], expected: [f32; 3]) {
        for (a, x) in actual.iter().zip(expected) {
            assert!((a - x).abs() < 2e-3, "{actual:?} vs {expected:?}");
        }
    }

    #[test]
    fn crossing_axes_touch_in_one_contact() {
        let mut e = Engine::new();
        let a = capsule(&mut e, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        let b = capsule(&mut e, [1.0, 0.8, -1.0], [1.0, 0.8, 1.0]);
        let out = contacts(&mut e, 0.1);
        let status = e.call(0x00cd58b0, &args![a, 0.5f32, b, 0.5f32, out]).u32();
        assert_eq!(status, 0);
        let out = out.addr();
        // The axes are 0.8 apart (gap -0.2 against the radii 0.5 + 0.5).
        near(read(&e, out + 0x10), [0.0, -1.0, 0.0]);
        assert!((e.mem.f32(out + 0x1c) + 0.2).abs() < 1e-5);
        near(read(&e, out), [1.0, 0.3, 0.0]);
    }

    #[test]
    fn a_wide_gap_stores_only_the_scaled_difference() {
        let mut e = Engine::new();
        let a = capsule(&mut e, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        let b = capsule(&mut e, [1.0, 0.8, -1.0], [1.0, 0.8, 1.0]);
        let out = contacts(&mut e, 0.5);
        let status = e.call(0x00cd58b0, &args![a, 0.1f32, b, 0.1f32, out]).u32();
        assert_eq!(status, 0);
        let out = out.addr();
        // Gap 0.6 is at least the stored 0.5: the contact point is left untouched.
        near(read(&e, out + 0x10), [0.0, -1.0, 0.0]);
        assert!((e.mem.f32(out + 0x1c) - 0.6).abs() < 1e-5);
        assert_eq!(read(&e, out), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn touching_axes_get_a_perpendicular_normal() {
        let mut e = Engine::new();
        let a = capsule(&mut e, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        let b = capsule(&mut e, [1.0, 0.0, -1.0], [1.0, 0.0, 1.0]);
        let out = contacts(&mut e, 0.1);
        let status = e.call(0x00cd58b0, &args![a, 0.5f32, b, 0.5f32, out]).u32();
        assert_eq!(status, 0);
        let out = out.addr();
        near(read(&e, out + 0x10), [0.0, 0.0, -1.0]);
        near(read(&e, out), [1.0, 0.0, -0.5]);
        assert!((e.mem.f32(out + 0x1c) + 1.0).abs() < 1e-5);
    }

    #[test]
    fn almost_parallel_axes_add_a_second_contact() {
        let mut e = Engine::new();
        let a = capsule(&mut e, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        let b = capsule(&mut e, [0.5, 0.8, 0.0], [2.5, 0.8, 0.25]);
        let out = contacts(&mut e, 1.0e9);
        let status = e.call(0x00cd58b0, &args![a, 0.5f32, b, 0.5f32, out]).u32();
        // Parameter t of B clamps at 0.
        assert_eq!(status, 8);
        let out = out.addr();
        near(read(&e, out), [0.5, 0.3, 0.0]);
        // The far end of A is paired with the point of B at the end of the overlap.
        let second = out + 0x40;
        near(read(&e, second), [1.991, 0.313, 0.0722]);
        assert!(e.mem.f32(second + 0x14) < -0.9);
    }

    // hkCollideCapsuleUtilCapsVsTri (00cd6520). The expectations come from the exe's arithmetic
    // worked by hand on a right triangle (0,0,0), (4,0,0), (0,4,0) whose cache scales every
    // edge by 0.25 and the normal by 1/16 (normal (0,0,1)) with 16 at +0x10, so a point
    // (x, y, z) lands in the frame at (x + y - 4, -x, -y, z). Radii 0.5 and 0, tolerance 0.25:
    // radius sum 0.5, limit 0.75.

    use std::cell::RefCell;
    use std::rc::Rc;

    const CAPS_VS_TRI: u32 = 0x00cd_6520;
    const FLAT: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [0.0, 4.0, 0.0]];
    const FLAT_CACHE: [f32; 5] = [0.25, 0.25, 0.25, 0.0625, 16.0];
    const LARGE_BITS: u32 = 0x7f7f_ffee;

    /// The exe's constants the function reads, with the values the exe holds, and the CRT
    /// square root as a double.
    fn exe_constants(e: &mut Engine) {
        for (at, length) in [
            (LARGE_GAP, 4),
            (LARGE_NEGATIVE_GAP, 4),
            (ONES, 16),
            (ZERO_VECTOR, 16),
            (SIGN_MASK, 16),
            (ZERO, 4),
            (THREE, 4),
            (HALF, 4),
            (TOUCH_EPSILON, 4),
            (PARAMETER_ABOVE, 4),
            (PARAMETER_BELOW, 8),
            (EDGE_VERTICES, 8),
        ] {
            e.map(at, length);
        }
        e.mem.set_u32(LARGE_GAP, LARGE_BITS);
        e.mem.set_u32(LARGE_NEGATIVE_GAP, 0xff7f_ffee);
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem.set_f32(ZERO_VECTOR + 4 * lane, 0.0);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_u32(TOUCH_EPSILON, 0x2880_0000);
        e.mem.set_u32(PARAMETER_ABOVE, 0x3f80_0347);
        e.mem.set_u64(PARAMETER_BELOW, 0xbf1a_36e2_e000_0000);
        for (i, index) in [2u8, 0, 1, 2, 0].into_iter().enumerate() {
            e.mem.set_u8(EDGE_VERTICES + i as u32, index);
        }
        e.register(X87_SQRT, |_, a| {
            let mut i = 0;
            Ret {
                st0: f64::take(a, &mut i).sqrt(),
                ..Ret::default()
            }
        });
    }

    fn put(e: &mut Engine, at: u32, v: [f32; 3]) {
        for (lane, c) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, c);
        }
        e.mem.set_f32(at + 12, 0.0);
    }

    fn triangle(e: &mut Engine, vertices: [[f32; 3]; 3]) -> Ptr {
        let at = e.mem.alloc(0x30);
        for (i, v) in vertices.into_iter().enumerate() {
            put(e, at + 0x10 * i as u32, v);
        }
        Ptr::new(at)
    }

    fn cache(e: &mut Engine, values: [f32; 5]) -> Ptr {
        let at = e.mem.alloc(0x14);
        for (i, v) in values.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, v);
        }
        Ptr::new(at)
    }

    /// Three contacts filled with 7.0 and three feature entries filled with 0xee.
    fn outputs(e: &mut Engine) -> (u32, u32) {
        let out = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_f32(out + 4 * word, 7.0);
        }
        let features = e.mem.alloc(0x18);
        for byte in 0..0x18 {
            e.mem.set_u8(features + byte, 0xee);
        }
        (out, features)
    }

    /// The flat triangle against the capsule `from`..`to` (radius 0.5, tolerance 0.25).
    fn caps_vs_flat(e: &mut Engine, from: [f32; 3], to: [f32; 3], manifold: u32) -> (u32, u32) {
        let tri = triangle(e, FLAT);
        let scales = cache(e, FLAT_CACHE);
        let capsule = capsule(e, from, to);
        let (out, features) = outputs(e);
        e.call(
            CAPS_VS_TRI,
            &args![capsule, 0.5f32, tri, 0.0f32, scales, 0.25f32, manifold, out, features],
        );
        (out, features)
    }

    fn feature(e: &Engine, features: u32, slot: u32) -> (u8, u16, u16, u16) {
        let at = features + 8 * slot;
        (
            e.mem.u8(at),
            e.mem.u16(at + 2),
            e.mem.u16(at + 4),
            e.mem.u16(at + 6),
        )
    }

    fn close(actual: [f32; 3], expected: [f32; 3]) {
        for (a, x) in actual.iter().zip(expected) {
            assert!((a - x).abs() < 1e-5, "{actual:?} vs {expected:?}");
        }
    }

    fn sqrt_arguments(e: &Engine) -> Vec<f64> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(at, _)| *at == X87_SQRT)
            .map(|(_, words)| f64::take(words, &mut 0))
            .collect()
    }

    struct SegmentCall {
        start_a: u32,
        direction_a: [f32; 3],
        start_b: u32,
        direction_b: [f32; 3],
    }

    #[derive(Clone, Copy)]
    struct SegmentReply {
        status: u32,
        point: [f32; 3],
        difference: [f32; 3],
        squared: f32,
    }

    type SegmentCalls = Rc<RefCell<Vec<SegmentCall>>>;

    /// `closestLineSegLineSeg` answering with `replies` in turn and recording its arguments.
    fn segments(e: &mut Engine, replies: Vec<SegmentReply>) -> SegmentCalls {
        let calls = SegmentCalls::default();
        let seen = calls.clone();
        let mut replies = replies.into_iter();
        e.register_double(CLOSEST_LINE_SEG_LINE_SEG, move |e, a| {
            seen.borrow_mut().push(SegmentCall {
                start_a: a[0],
                direction_a: read(e, a[1]),
                start_b: a[2],
                direction_b: read(e, a[3]),
            });
            let reply = replies.next().expect("a reply for every call");
            put(e, a[4], reply.point);
            put(e, a[4] + 0x10, reply.difference);
            e.mem.set_f32(a[4] + 0x20, reply.squared);
            Ret {
                eax: reply.status,
                ..Ret::default()
            }
        });
        calls
    }

    #[test]
    fn separated_capsules_leave_only_the_large_gaps() {
        // Above the plane beyond the limit, below it beyond the limit, outside edge 0.
        let cases = [
            ([1.0, 1.0, 2.0], [1.0, 1.0, 3.0]),
            ([1.0, 1.0, -3.0], [1.0, 1.0, -2.0]),
            ([5.0, 1.0, 0.0], [5.0, 2.0, 0.0]),
        ];
        for (from, to) in cases {
            let mut e = Engine::new();
            exe_constants(&mut e);
            e.call_log = Some(vec![]);
            let (out, features) = caps_vs_flat(&mut e, from, to, 1);
            for slot in 0..3 {
                assert_eq!(e.mem.u32(out + 0x20 * slot + 0x1c), LARGE_BITS);
                assert_eq!(read(&e, out + 0x20 * slot), [7.0; 3]);
                assert_eq!(feature(&e, features, slot), (0, 0xeeee, 0xeeee, 0xeeee));
            }
            assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        }
    }

    #[test]
    fn a_penetrating_end_inside_the_triangle_is_the_contact() {
        let mut e = Engine::new();
        exe_constants(&mut e);
        e.call_log = Some(vec![]);
        // Ends (-2,-1,-1,-1) and (-2,-1,-1,1): the axis crosses the face at (1,1,0).
        let (out, features) = caps_vs_flat(&mut e, [1.0, 1.0, -1.0], [1.0, 1.0, 1.0], 0);
        // The lower end, depth -(1 + 0.5), moved up by 0 - (-1); the upper end ties and the
        // edges (squared distances 4, 1, 1 against (-1.5 + 0.5)^2) stay out.
        let contact = out + 0x20;
        assert_eq!(read(&e, contact), [1.0, 1.0, 0.0]);
        assert_eq!(read(&e, contact + 0x10), [0.0, 0.0, 1.0]);
        assert_eq!(e.mem.f32(contact + 0x1c), -1.5);
        assert_eq!(e.mem.u32(out + 0x1c), LARGE_BITS);
        assert_eq!(e.mem.u32(out + 0x5c), LARGE_BITS);
        assert_eq!(read(&e, out), [7.0; 3]);
        for slot in 0..3 {
            assert_eq!(feature(&e, features, slot), (3, 0, 1, 2));
        }
        assert!(sqrt_arguments(&e).is_empty());
    }

    #[test]
    fn a_penetrating_axis_near_an_edge_takes_the_edge() {
        let mut e = Engine::new();
        exe_constants(&mut e);
        e.call_log = Some(vec![]);
        // Ends (-0.25,-1.5,-2.25,-1) and (-0.25,-1.5,-2.25,1): 0.25 inside edge 0.
        let (out, _) = caps_vs_flat(&mut e, [1.5, 2.25, -1.0], [1.5, 2.25, 1.0], 0);
        // Edge 0: crossing 0 * -1 - (-0.25 * 2) = 0.5, inverse 1 / (0 + 4), squared 0.0625
        // below (-1.5 + 0.5)^2; parameter -(-0.25 * 0 + 2 * -1) * 0.25 = 0.5. Normal
        // 2 * (1, 1, 0) normalized; the axis point (1.5, 2.25, 0) moved by 0 - (-sqrt 0.0625).
        let contact = out + 0x20;
        let half_root = std::f32::consts::FRAC_1_SQRT_2;
        close(read(&e, contact + 0x10), [half_root, half_root, 0.0]);
        close(
            read(&e, contact),
            [1.5 + 0.25 * half_root, 2.25 + 0.25 * half_root, 0.0],
        );
        assert_eq!(e.mem.f32(contact + 0x1c), -0.75);
        assert_eq!(sqrt_arguments(&e), vec![0.0625]);
    }

    #[test]
    fn a_capsule_beside_an_edge_searches_the_manifold() {
        let mut e = Engine::new();
        exe_constants(&mut e);
        e.call_log = Some(vec![]);
        // Ends (-3.25,0.25,-1,0.25) and (-1.25,0.25,-3,0.25): only edge 1 (vertex 2 to 0) is
        // outside both, a capsule (0,4,0)..(0,0,0) parallel to the axis 0.25 away in x and z.
        let (out, features) = caps_vs_flat(&mut e, [-0.25, 1.0, 0.25], [-0.25, 3.0, 0.25], 1);
        // ManifoldCapsVsCaps: s = 1 on the reversed axis (status 1), t = 0.75; the difference
        // (-0.25, 0, 0.25), distance sqrt 0.125, gap below the tolerance 0.25. The axes are
        // parallel: a second contact at the far end, (0,3,0).
        let root = 0.125f32.sqrt();
        let half_root = std::f32::consts::FRAC_1_SQRT_2;
        close(read(&e, out), [0.0, 1.0, 0.0]);
        close(read(&e, out + 0x10), [-half_root, 0.0, half_root]);
        assert!((e.mem.f32(out + 0x1c) - (root - 0.5)).abs() < 1e-6);
        close(read(&e, out + 0x20), [0.0, 3.0, 0.0]);
        assert!((e.mem.f32(out + 0x3c) - (root - 0.5)).abs() < 1e-6);
        assert_eq!(e.mem.f32(out + 0x5c), 0.25);
        for slot in 0..3 {
            assert_eq!(feature(&e, features, slot), (2, 2, 0, 0xeeee));
        }
        assert_eq!(sqrt_arguments(&e).len(), 1);
    }

    #[test]
    fn a_capsule_off_a_corner_takes_its_closest_edge() {
        let mut e = Engine::new();
        exe_constants(&mut e);
        let calls = segments(
            &mut e,
            vec![
                SegmentReply {
                    status: 1,
                    point: [-0.25, -0.5, 0.25],
                    difference: [-0.5, 0.0, 0.0],
                    squared: 0.25,
                },
                SegmentReply {
                    status: 0,
                    point: [-0.25, -0.25, 0.25],
                    difference: [0.0, -0.25, 0.0],
                    squared: 0.0625,
                },
            ],
        );
        // Ends (-4.75,0.5,0.25,0.25) and (-4.75,0.25,0.5,0.25): edges 1 and 2 are outside
        // both; no manifold search.
        let (out, features) = caps_vs_flat(&mut e, [-0.5, -0.25, 0.25], [-0.25, -0.5, 0.25], 0);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 2);
        let tri = calls[0].start_b - 0x20;
        for call in calls.iter() {
            assert_eq!(e.mem.f32(call.start_a), -0.5);
            assert_eq!(call.direction_a, [0.25, -0.25, 0.0]);
        }
        assert_eq!(calls[0].direction_b, [0.0, -4.0, 0.0]);
        assert_eq!(calls[1].start_b, tri);
        assert_eq!(calls[1].direction_b, [4.0, 0.0, 0.0]);
        // Edge 1 clamps the axis at the end bit 1: taken, later edges count for end 1. Its gap
        // is 0.5 - 0.5; edge 2 (nothing clamped) then wins with the cross product of the
        // directions, (0, 0, 1), distance 0 along it: gap -0.5, the point unmoved.
        assert_eq!(read(&e, out), [-0.25, -0.25, 0.25]);
        close(read(&e, out + 0x10), [0.0, 0.0, 1.0]);
        assert_eq!(e.mem.f32(out + 0x1c), -0.5);
        assert_eq!(e.mem.u32(out + 0x3c), LARGE_BITS);
        assert_eq!(feature(&e, features, 0), (2, 2, 0, 0xeeee));
        assert_eq!(feature(&e, features, 1), (2, 0, 1, 0xeeee));
        assert_eq!(feature(&e, features, 2), (0, 0xeeee, 0xeeee, 0xeeee));
    }

    #[test]
    fn a_touching_parallel_edge_uses_its_edge_plane() {
        let mut e = Engine::new();
        exe_constants(&mut e);
        let calls = segments(
            &mut e,
            vec![SegmentReply {
                status: 4,
                point: [-0.25, 2.0, 0.25],
                difference: [0.0, 0.0, 0.0],
                squared: 0.0,
            }],
        );
        let (out, features) = caps_vs_flat(&mut e, [-0.25, 1.0, 0.25], [-0.25, 3.0, 0.25], 0);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].direction_b, [0.0, -4.0, 0.0]);
        // Squared distance 0 and parallel directions: edge 1's plane (-1, 0, 0) is the normal,
        // distance 1 along it; the point moved by 0 - 1.
        close(read(&e, out + 0x10), [-1.0, 0.0, 0.0]);
        close(read(&e, out), [0.75, 2.0, 0.25]);
        assert!((e.mem.f32(out + 0x1c) - 0.5).abs() < 1e-5);
        assert_eq!(feature(&e, features, 0), (1, 2, 0xeeee, 0xeeee));
        assert_eq!(feature(&e, features, 1), (0, 0xeeee, 0xeeee, 0xeeee));
    }

    #[test]
    fn one_end_over_the_face_and_one_beside_an_edge_give_two_contacts() {
        let mut e = Engine::new();
        exe_constants(&mut e);
        let calls = segments(
            &mut e,
            vec![SegmentReply {
                status: 8,
                point: [-0.25, 1.0, 0.25],
                difference: [-0.25, 0.0, 0.0],
                squared: 0.0625,
            }],
        );
        // Ends (-2,-1,-1,0.25) (inside the prism) and (-3.25,0.25,-1,0.25) (outside edge 1).
        let (out, features) = caps_vs_flat(&mut e, [1.0, 1.0, 0.25], [-0.25, 1.0, 0.25], 0);
        // End 0: moved by 0 - 0.25 along the normal, gap 0.25 - 0.5.
        assert_eq!(read(&e, out), [1.0, 1.0, 0.0]);
        assert_eq!(read(&e, out + 0x10), [0.0, 0.0, 1.0]);
        assert_eq!(e.mem.f32(out + 0x1c), -0.25);
        // End 1: edge 1, clamped on the edge only, distance 0.25 along (-1, 0, 0).
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].direction_a, [-1.25, 0.0, 0.0]);
        assert_eq!(calls[0].direction_b, [0.0, -4.0, 0.0]);
        close(read(&e, out + 0x20), [0.0, 1.0, 0.25]);
        close(read(&e, out + 0x30), [-1.0, 0.0, 0.0]);
        assert!((e.mem.f32(out + 0x3c) + 0.25).abs() < 1e-6);
        assert_eq!(e.mem.u32(out + 0x5c), LARGE_BITS);
        assert_eq!(feature(&e, features, 0), (3, 0, 1, 2));
        assert_eq!(feature(&e, features, 1), (1, 2, 0xeeee, 0xeeee));
        assert_eq!(feature(&e, features, 2), (0, 0xeeee, 0xeeee, 0xeeee));
    }

    #[test]
    fn a_degenerate_triangle_tries_all_three_edges() {
        let mut e = Engine::new();
        exe_constants(&mut e);
        let far = SegmentReply {
            status: 0,
            point: [1.0, 1.0, 0.0],
            difference: [0.0, 2.0, 0.0],
            squared: 4.0,
        };
        let calls = segments(
            &mut e,
            vec![
                SegmentReply {
                    status: 0,
                    point: [1.0, 1.0, 0.5],
                    difference: [0.0, 1.0, 0.0],
                    squared: 1.0,
                },
                far,
                far,
            ],
        );
        // Collinear vertices: zero normal and planes, every frame distance 0, no mask bit.
        let tri = triangle(&mut e, [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [4.0, 0.0, 0.0]]);
        let scales = cache(&mut e, [1.0, 1.0, 1.0, 1.0, 0.0]);
        let capsule = capsule(&mut e, [1.0, 1.0, 0.0], [1.0, 1.0, 2.0]);
        let (out, features) = outputs(&mut e);
        e.call(
            CAPS_VS_TRI,
            &args![capsule, 0.5f32, tri, 0.0f32, scales, 0.25f32, 0u32, out, features],
        );
        let calls = calls.borrow();
        let starts: Vec<u32> = calls.iter().map(|call| call.start_b - tri.addr()).collect();
        assert_eq!(starts, vec![0x10, 0x20, 0]);
        let directions: Vec<[f32; 3]> = calls.iter().map(|call| call.direction_b).collect();
        assert_eq!(
            directions,
            vec![[2.0, 0.0, 0.0], [-4.0, 0.0, 0.0], [2.0, 0.0, 0.0]]
        );
        assert!(calls.iter().all(|call| call.direction_a == [0.0, 0.0, 2.0]));
        // Edge 0: cross product (0, 4, 0), distance 1 along it, gap 0.5; edges 1 and 2 are
        // farther than (0.5 + 0.5)^2.
        close(read(&e, out + 0x10), [0.0, 1.0, 0.0]);
        close(read(&e, out), [1.0, 0.0, 0.5]);
        assert!((e.mem.f32(out + 0x1c) - 0.5).abs() < 1e-5);
        assert_eq!(feature(&e, features, 0), (2, 0, 1, 0xeeee));
    }

    #[test]
    fn the_triangle_helper_makes_the_normal_and_the_transposed_matrix_00cd6120() {
        let mut e = Engine::new();
        let triangle = e.mem.alloc(0x30);
        for (n, vertex) in [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 1.0]]
            .into_iter()
            .enumerate()
        {
            put(&mut e, triangle + 0x10 * n as u32, vertex);
        }
        let scales = e.mem.alloc(0x14);
        for (n, value) in [2.0f32, 3.0, 5.0, 7.0, 11.0].into_iter().enumerate() {
            e.mem.set_f32(scales + 4 * n as u32, value);
        }
        let matrix = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(matrix + 4 * word, 0xdead_beef);
        }
        let normal = e.mem.alloc(0x10);
        e.call(0x00cd_6120, &[matrix, normal, triangle, scales]);
        let floats = |at: u32, count: u32| -> Vec<f32> {
            (0..count).map(|n| e.mem.f32(at + 4 * n)).collect()
        };
        assert_eq!(floats(normal, 4), vec![0.0, -7.0, 7.0, 0.0]);
        assert_eq!(
            floats(matrix, 16),
            vec![
                28.0, -42.0, 0.0, 0.0, 14.0, 0.0, -35.0, -7.0, 14.0, 0.0, -35.0, 7.0, -22.0, 0.0,
                0.0, 0.0
            ]
        );
    }

    /// Capsule A from the origin to `end_a`, capsule B from (1, 1, -1) to (1, 1, 1), the
    /// segment function answering `reply`, the radii `(a, b)` and the tolerance; the output
    /// is filled with 7.0.
    struct Closest {
        e: Engine,
        calls: SegmentCalls,
        a: Ptr,
        b: Ptr,
        out: u32,
        status: u32,
    }

    fn closest_case(end_a: [f32; 3], reply: SegmentReply, radii: (f32, f32, f32)) -> Closest {
        let mut e = Engine::new();
        exe_constants(&mut e);
        e.call_log = Some(vec![]);
        let calls = segments(&mut e, vec![reply]);
        let a = capsule(&mut e, [0.0, 0.0, 0.0], end_a);
        let b = capsule(&mut e, [1.0, 1.0, -1.0], [1.0, 1.0, 1.0]);
        let out = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_f32(out + 4 * word, 7.0);
        }
        let status = e
            .call(0x00cd_6260, &args![a, radii.0, b, radii.1, radii.2, out])
            .u32();
        Closest {
            e,
            calls,
            a,
            b,
            out,
            status,
        }
    }

    fn unit(v: [f32; 3]) -> [f32; 3] {
        let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        v.map(|lane| lane / length)
    }

    #[test]
    fn capsules_beyond_the_radii_and_tolerance_answer_1_00cd6260() {
        let reply = SegmentReply {
            status: 0,
            point: [1.0, 0.0, 0.0],
            difference: [0.0, 1.0, 0.0],
            squared: 1.6,
        };
        // (0.5 + 0.5 + 0.25)^2 = 1.5625 < 1.6.
        let s = closest_case([2.0, 0.0, 0.0], reply, (0.5, 0.5, 0.25));
        assert_eq!(s.status, 1);
        assert!(sqrt_arguments(&s.e).is_empty());
        assert_eq!(read(&s.e, s.out), [7.0; 3]);
        let calls = s.calls.borrow();
        assert_eq!(calls[0].start_a, s.a.addr());
        assert_eq!(calls[0].start_b, s.b.addr());
        assert_eq!(calls[0].direction_a, [2.0, 0.0, 0.0]);
        assert_eq!(calls[0].direction_b, [0.0, 0.0, 2.0]);
        // At the limit the capsules still count.
        let reply = SegmentReply {
            squared: 1.5625,
            ..reply
        };
        let s = closest_case([2.0, 0.0, 0.0], reply, (0.5, 0.5, 0.25));
        assert_eq!(s.status, 0);
        // A NaN squared distance is never above the limit.
        let reply = SegmentReply {
            squared: f32::NAN,
            ..reply
        };
        let s = closest_case([2.0, 0.0, 0.0], reply, (0.5, 0.5, 0.25));
        assert_eq!(s.status, 0);
    }

    #[test]
    fn crossing_axes_get_the_cross_product_turned_towards_the_answer_00cd6260() {
        // The axes' cross product is (0, -4, 0); the answer's vector (0, 1, 0) is opposite.
        let reply = SegmentReply {
            status: 0,
            point: [1.0, 0.0, 0.0],
            difference: [0.0, 1.0, 0.0],
            squared: 0.25,
        };
        let s = closest_case([2.0, 0.0, 0.0], reply, (0.5, 1.0, 0.25));
        assert_eq!(s.status, 0);
        assert_eq!(sqrt_arguments(&s.e), vec![0.25]);
        // Distance 0.5, radius sum 1.5: gap -1, depth 0.5.
        close(read(&s.e, s.out + 0x10), [0.0, 1.0, 0.0]);
        close(read(&s.e, s.out), [1.0, 0.5, 0.0]);
        assert!((s.e.mem.f32(s.out + 0x1c) + 1.0).abs() < 1e-5);
        // With the answer's vector on the same side the cross product keeps its direction.
        let reply = SegmentReply {
            difference: [0.0, -1.0, 0.0],
            ..reply
        };
        let s = closest_case([2.0, 0.0, 0.0], reply, (0.5, 1.0, 0.25));
        close(read(&s.e, s.out + 0x10), [0.0, -1.0, 0.0]);
        close(read(&s.e, s.out), [1.0, -0.5, 0.0]);
        // A vector at a right angle (dot product 0) does not turn it either.
        let reply = SegmentReply {
            difference: [1.0, 0.0, 0.0],
            ..reply
        };
        let s = closest_case([2.0, 0.0, 0.0], reply, (0.5, 1.0, 0.25));
        close(read(&s.e, s.out + 0x10), [0.0, -1.0, 0.0]);
    }

    #[test]
    fn a_non_zero_answer_keeps_the_vector_of_the_segment_function_00cd6260() {
        let reply = SegmentReply {
            status: 1,
            point: [1.0, 0.0, 0.0],
            difference: [0.0, 0.0, 3.0],
            squared: 4.0,
        };
        let s = closest_case([2.0, 0.0, 0.0], reply, (1.5, 1.5, 0.0));
        assert_eq!(s.status, 0);
        assert_eq!(sqrt_arguments(&s.e), vec![4.0]);
        // Distance 2, radius sum 3: gap -1; depth 1.5 - 2 = -0.5 along (0, 0, 1).
        close(read(&s.e, s.out + 0x10), [0.0, 0.0, 1.0]);
        close(read(&s.e, s.out), [1.0, 0.0, -0.5]);
        assert!((s.e.mem.f32(s.out + 0x1c) + 1.0).abs() < 1e-5);
    }

    #[test]
    fn without_a_usable_vector_the_normal_is_perpendicular_to_the_first_axis_00cd6260() {
        // (first axis, zeroed component by the order of the absolute values)
        let cases: [([f32; 3], [f32; 3]); 4] = [
            ([3.0, -1.0, 2.0], [2.0, 0.0, -3.0]),
            ([1.0, 2.0, -3.0], [0.0, -3.0, -2.0]),
            ([3.0, 2.0, 1.0], [2.0, -3.0, 0.0]),
            ([2.0, 3.0, 1.0], [-3.0, 2.0, 0.0]),
        ];
        for (axis, normal) in cases {
            // A zero squared distance with a non-zero answer.
            let reply = SegmentReply {
                status: 1,
                point: [1.0, 2.0, 3.0],
                difference: [5.0, 5.0, 5.0],
                squared: 0.0,
            };
            let s = closest_case(axis, reply, (0.5, 0.5, 0.0));
            assert_eq!(s.status, 0, "axis {axis:?}");
            close(read(&s.e, s.out + 0x10), unit(normal));
            // Distance 0: the position moves by the radius of B along the normal.
            let expected = unit(normal).map(|lane| lane * 0.5);
            close(
                read(&s.e, s.out),
                [1.0 + expected[0], 2.0 + expected[1], 3.0 + expected[2]],
            );
            assert!((s.e.mem.f32(s.out + 0x1c) + 1.0).abs() < 1e-5);
        }
        // Parallel axes (zero cross product) with the answer 0 go the same way.
        let reply = SegmentReply {
            status: 0,
            point: [0.0, 0.0, 0.0],
            difference: [1.0, 1.0, 1.0],
            squared: 0.0,
        };
        let s = closest_case([0.0, 0.0, 2.0], reply, (0.5, 0.5, 0.0));
        assert_eq!(s.status, 0);
        // Axis (0, 0, 2): the order of the absolute values leaves the x lane at 0 and turns the
        // y and z lanes: (0, 2, -0).
        close(read(&s.e, s.out + 0x10), [0.0, 1.0, 0.0]);
    }
}
