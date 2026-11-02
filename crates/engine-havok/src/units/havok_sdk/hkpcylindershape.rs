//! `Havok SDK/hkpcylindershape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpCylinderShape` is not in the Xbox PDB, so the layout below is read off the code and the
//! field names are descriptive only (unconfirmed): `+0x10` is the `float` every `hkpConvexShape`
//! keeps as `m_radius` (Xbox PDB), `+0x14` the cylinder's own radius, `+0x18` a scale applied to
//! it for the collision spheres, `+0x20` and `+0x30` the two axis end points (`hkVector4`) and
//! `+0x40` and `+0x50` two direction vectors spanning the cross section. The methods are named
//! after the `hkpConvexShape` virtuals the Ghidra project names them by. SSE code computes lane by
//! lane in `f32`; a dot product is `z + (y + x)` the way the `SHUFPS`/`ADDSS` sequences add it.
//! x87 code compares the way its `FNSTSW`/`TEST AH` pairs read the flags (NaN included) and rounds
//! at each `float` store; a float copied through `FLD`/`FSTP` goes through [`x87_quiet`].
//! `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`), as elsewhere in the crate. The
//! profiling records `castRayImpl` writes use the host clock for `RDTSC`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `+0x10`: the `hkpConvexShape` radius (Xbox PDB `m_radius`).
const CONVEX_RADIUS: u32 = 0x10;
/// `+0x14`: the cylinder's radius (unconfirmed name).
const CYLINDER_RADIUS: u32 = 0x14;
/// `+0x18`: the factor the collision spheres scale the radius by (unconfirmed name).
const RADIUS_FACTOR: u32 = 0x18;
/// The axis end points and the two cross section directions.
const VERTEX_A: u32 = 0x20;
const VERTEX_B: u32 = 0x30;
const DIRECTION_U: u32 = 0x40;
const DIRECTION_V: u32 = 0x50;

/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// The closest point on a segment (`00d224c0`, cdecl `(point, a, b, out)`).
const CLOSEST_POINT_ON_SEGMENT: u32 = 0x00d2_24c0;
/// The wrapper of the closest-points helper (`00d22700`, cdecl, nine pointers).
const CLOSEST_POINTS_WRAPPER: u32 = 0x00d2_2700;
/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcCylinder"` and `"Et"`.
const PROFILE_BEGIN_CYLINDER: u32 = 0x010d_0f70;
const PROFILE_END: u32 = 0x010c_41f8;

/// `float` constants: 0.5, 0.0, 3.0 (the `RSQRTSS` refinement), -1.0, the epsilon `2^-23` and the
/// largest value the ray distance starts at.
const HALF: u32 = 0x0101_6248;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const MINUS_ONE: u32 = 0x0101_2054;
const EPSILON: u32 = 0x0104_da4c;
const LARGE_DISTANCE: u32 = 0x0104_da48;
/// `float` constants of `getSupportingVertexImpl`: `2^-46` and the three that map the smaller
/// direction component to a vertex index.
const TINY_LENGTH_SQUARED: u32 = 0x010c_b428;
const INDEX_SLOPE: u32 = 0x011b_5be0;
const ID_ANGLE_SCALE: u32 = 0x011b_5be4;
const INDEX_OFFSET: u32 = 0x011b_5be8;
/// `double` constants: the index bias (0.05f), 0.5, 4.0, `0.70710677f`, 1.01f.
const INDEX_BIAS: u32 = 0x0103_c7c8;
const HALF_DOUBLE: u32 = 0x0101_1588;
const FOUR_DOUBLE: u32 = 0x0101_db80;
const ROOT_HALF_DOUBLE: u32 = 0x010d_0f10;
const EXTENT_LIMIT_DOUBLE: u32 = 0x010d_0f68;
/// Vector constants: four 1.0 and the mask keeping the first three lanes.
const ONES: u32 = 0x010e_c7f0;
const LANE_MASK: u32 = 0x010c_bfc0;

// `hkpShapeRayCastInput` (Xbox PDB): `m_from` +0x00, `m_to` +0x10.
const RAY_FROM: u32 = 0x00;
const RAY_TO: u32 = 0x10;
// `hkpShapeRayCastOutput` (Xbox PDB): `m_normal` +0x00, `m_hitFraction` +0x10, `m_shapeKeys` +0x20,
// `m_shapeKeyIndex` +0x40.
const OUT_NORMAL: u32 = 0x00;
const OUT_HIT_FRACTION: u32 = 0x10;
const OUT_SHAPE_KEYS: u32 = 0x20;
const OUT_SHAPE_KEY_INDEX: u32 = 0x40;

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

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

fn dot3(a: Vec4, b: Vec4) -> f32 {
    sum3(zip_with(a, b, |x, y| x * y))
}

/// `SHUFPS v, v, 0xc9`: lanes `(y, z, x, w)`.
fn shuffle_yzx(v: Vec4) -> Vec4 {
    [v[1], v[2], v[0], v[3]]
}

/// `SHUFPS v, v, 0xd2`: lanes `(z, x, y, w)`.
fn shuffle_zxy(v: Vec4) -> Vec4 {
    [v[2], v[0], v[1], v[3]]
}

/// `MAXPS dst, src` per lane: `dst` when it is greater, else `src`.
fn maxps(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// `MINPS dst, src` per lane: `dst` when it is smaller, else `src`.
fn minps(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `FISTP dword` of a `float` already on the x87 stack: nearest (even), `0x80000000` for a NaN or a
/// value outside the `int` range.
fn fistp(value: f32) -> i32 {
    crate::x87::fistp(f64::from(value))
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero guard:
/// `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == zero`.
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

/// `(1 - fraction) * start + end * fraction` in every lane (`ones` is the vector constant of four
/// 1.0).
fn interpolate(ones: Vec4, start: Vec4, end: Vec4, fraction: f32) -> Vec4 {
    std::array::from_fn(|lane| (ones[lane] - fraction) * start[lane] + end[lane] * fraction)
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record stream (`TlsGetValue`
/// of the index at `0126835c`) while its cursor (`+4`) is below its limit (`+0xC`); a record is 12
/// bytes.
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// The x87 square root helper on a value that is stored back as a `float`.
fn x87_sqrt(e: &mut Engine, value: f32) -> f32 {
    e.call(X87_SQRT, &args![f64::from(value)]).f64() as f32
}

/// The vertex a 16-bit vertex id stands for, shared by `getSupportingVertexImpl` and
/// `convertVertexIdsToVerticesImpl`. Bits 0..3 pick a step along a quarter turn
/// (`first = (low + 0.5) * scale`, `second = sqrt(1 - first^2)`), bit 4 swaps which of the two
/// goes with the first direction, bits 5 and 6 are clear to negate the first and second
/// direction's factor, and bit 7 picks the axis end (`m_vertexB` when clear). The point is
/// `(second_factor * V + first_factor * U) * radius + end`, lane by lane.
fn vertex_from_id(e: &mut Engine, this: u32, id: u32) -> Vec4 {
    let bits = id & 0xffff;
    let half: f64 = e.global(HALF_DOUBLE);
    let scale: f32 = e.global(ID_ANGLE_SCALE);
    let step = ((bits & 0xf) as f64 + half) as f32;
    let first = (f64::from(step) * f64::from(scale)) as f32;
    let remainder = (1.0 - f64::from(first) * f64::from(first)) as f32;
    let second = x87_sqrt(e, remainder);
    let (mut factor_u, mut factor_v) = if bits & 0x10 != 0 {
        (second, first)
    } else {
        (first, second)
    };
    if bits & 0x40 == 0 {
        factor_v = -factor_v;
    }
    if bits & 0x20 == 0 {
        factor_u = -factor_u;
    }
    let direction_u = load_vec(e, this + DIRECTION_U);
    let direction_v = load_vec(e, this + DIRECTION_V);
    let radius = e.mem.f32(this + CYLINDER_RADIUS);
    let end = load_vec(e, this + 0x10 * (3 - ((bits >> 7) & 1)));
    std::array::from_fn(|lane| {
        ((factor_v * direction_v[lane]) + (factor_u * direction_u[lane])) * radius + end[lane]
    })
}

/// The vertex id `getSupportingVertexImpl` picks for `direction`: the direction's components
/// along the two cross section directions give an angle step (`fistp` of a linear map of the
/// smaller of the two normalised components), the signs of the components and of the component
/// along the axis give the high bits.
fn supporting_vertex_id(e: &mut Engine, this: u32, direction: Vec4) -> u32 {
    let a = load_vec(e, this + VERTEX_A);
    let b = load_vec(e, this + VERTEX_B);
    let direction_u = load_vec(e, this + DIRECTION_U);
    let direction_v = load_vec(e, this + DIRECTION_V);
    let span = zip_with(b, a, |p, q| p - q);
    let along_u = dot3(direction, direction_u);
    let along_v = dot3(direction, direction_v);
    // `du^2 + dv^2` is formed in x87 and stored as a `float`.
    let squared =
        (f64::from(along_u) * f64::from(along_u) + f64::from(along_v) * f64::from(along_v)) as f32;
    let tiny: f32 = e.global(TINY_LENGTH_SQUARED);
    // `FCOM` leaves C0 set for "less" and for unordered.
    let (along_v, along_u, v_not_negative) = if squared < tiny || squared.is_nan() {
        (0.0f32, 1.0f32, 1u32)
    } else {
        let length = x87_sqrt(e, squared);
        let inverse = (1.0 / f64::from(length)) as f32;
        let along_v = (f64::from(along_v) * f64::from(inverse)) as f32;
        let along_u = (f64::from(inverse) * f64::from(along_u)) as f32;
        (along_v, along_u, (0.0 <= along_v) as u32)
    };
    let u_not_negative = (along_u >= 0.0) as u32;
    let abs_u = along_u.abs();
    let abs_v = along_v.abs();
    let (u_is_larger, smaller) = if abs_u >= abs_v {
        (1u32, abs_v)
    } else {
        (0u32, abs_u)
    };
    let along_axis = dot3(direction, span);
    let backwards = (along_axis <= 0.0) as u32;
    let slope: f32 = e.global(INDEX_SLOPE);
    let offset: f32 = e.global(INDEX_OFFSET);
    let bias: f64 = e.global(INDEX_BIAS);
    let scaled = ((f64::from(slope) * f64::from(smaller)) - f64::from(offset)) + bias;
    let step = fistp(scaled as f32);
    let high = u_is_larger + 2 * (u_not_negative + 2 * (v_not_negative + 2 * backwards));
    (high << 4).wrapping_add(step as u32)
}

// Translated from 00d571b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::getNumCollisionSpheresImpl` (Ghidra name): 0x12.
pub fn hkp_cylinder_shape_get_num_collision_spheres_impl(_e: &mut Engine, _this: Ptr) -> u32 {
    0x12
}

// Translated from 00d57400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::getSupportingVertexImpl` (Ghidra name): writes the point of the cylinder's
/// rim (or axis end) that lies furthest along `direction` into `out`, its `w` lane holding the
/// vertex id as the bits `0x3f000000 | id`. See [`supporting_vertex_id`] and [`vertex_from_id`]
/// for how the id is chosen and expanded.
pub fn hkp_cylinder_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    out: Ptr,
) {
    let direction = load_vec(e, direction.addr());
    let id = supporting_vertex_id(e, this.addr(), direction);
    let vertex = vertex_from_id(e, this.addr(), id);
    store_vec(e, out.addr(), vertex);
    e.mem.set_u32(out.addr() + 0xc, id | 0x3f00_0000);
}

// Translated from 00d576d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::convertVertexIdsToVerticesImpl` (Ghidra name): for each of `count` 16-bit
/// vertex ids, writes the vertex [`vertex_from_id`] gives to `out` and replaces its `w` lane by
/// the bits `0x3f000000 | id`.
pub fn hkp_cylinder_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    this: Ptr,
    ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let mut id_address = ids.addr();
    let mut out_address = out.addr();
    for _ in 0..count.max(0) {
        let id = u32::from(e.mem.u16(id_address));
        let vertex = vertex_from_id(e, this.addr(), id);
        store_vec(e, out_address, vertex);
        e.mem.set_u32(out_address + 0xc, id | 0x3f00_0000);
        id_address = id_address.wrapping_add(2);
        out_address = out_address.wrapping_add(0x10);
    }
}

// Translated from 00d57830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::getCentreImpl` (Ghidra name): `0.5 * (B + A)` in every lane.
pub fn hkp_cylinder_shape_get_centre_impl(e: &mut Engine, this: Ptr, out: Ptr) {
    let a = load_vec(e, this.addr() + VERTEX_A);
    let b = load_vec(e, this.addr() + VERTEX_B);
    let half: f32 = e.global(HALF);
    let sum = zip_with(b, a, |p, q| p + q);
    store_vec(e, out.addr(), sum.map(|lane| half * lane));
}

// Translated from 00d57870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::getFirstVertex` (Ghidra name): copies the second axis end point.
pub fn hkp_cylinder_shape_get_first_vertex(e: &mut Engine, this: Ptr, out: Ptr) {
    copy_vec(e, this.addr() + VERTEX_B, out.addr());
}

// Translated from 00d57890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::getCollisionSpheresImpl` (Ghidra name): fills `out` with 18 spheres (16-byte
/// vectors, the `w` lane the sphere radius). With `r` the cylinder radius, a long cylinder
/// (`4 r^2 <= |B - A|^2`) uses the offset `c = r * factor` and the end spheres sit `r` along
/// `U x V`, a short one uses `c = r` and no end offset. Each end point gets eight spheres in a
/// ring of offsets `c U`, `h (V + U)`, `c V`, `-h (U - V)`, ... with `h = c * 0.70710677`
/// (lane 3 of each offset cleared, the sphere radius is the `+0x10` radius); the last two are the
/// end points moved by `+-` that end offset along `U x V` with radius `radius + end offset`.
/// Answers `out` (the `const hkSphere*` of the Xbox PDB signature; `EAX` is `out`, loaded
/// at 00d57919, at the RET 00d57a98): hkpConvexTransformShape::getCollisionSpheresImpl reads
/// it through its child shape's slot +0x30 (`MOV EDI,EAX` at 00d5df2f).
pub fn hkp_cylinder_shape_get_collision_spheres_impl(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let me = this.addr();
    let r = x87_quiet(e.mem.f32(me + CYLINDER_RADIUS));
    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let span = zip_with(b, a, |p, q| p - q);
    let span_squared = sum3(zip_with(span, span, |p, q| p * q));
    let four: f64 = e.global(FOUR_DOUBLE);
    let factor = e.mem.f32(me + RADIUS_FACTOR);
    // `4 r * r` against `|B - A|^2`: `FCOMPP` then `JP` falls through for "less" and "equal".
    let (c, end_offset) = if f64::from(r) * four * f64::from(r) <= f64::from(span_squared) {
        ((f64::from(r) * f64::from(factor)) as f32, r)
    } else {
        (r, 0.0f32)
    };
    let root_half: f64 = e.global(ROOT_HALF_DOUBLE);
    let h = (f64::from(c) * root_half) as f32;
    let convex_radius = x87_quiet(e.mem.f32(me + CONVEX_RADIUS));
    let direction_u = load_vec(e, me + DIRECTION_U);
    let direction_v = load_vec(e, me + DIRECTION_V);
    // `ANDPS` with the lane mask clears lane 3 and keeps the other three bit for bit.
    let mask: [u32; 4] = std::array::from_fn(|lane| e.mem.u32(LANE_MASK + 4 * lane as u32));
    let masked = |v: Vec4| -> Vec4 {
        std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() & mask[lane]))
    };
    let offset_1 = masked(direction_u.map(|lane| c * lane));
    let offset_2 = masked(direction_v.map(|lane| c * lane));
    let sum_vu = zip_with(direction_v, direction_u, |p, q| p + q);
    let offset_3 = masked(sum_vu.map(|lane| h * lane));
    let difference_uv = zip_with(direction_u, direction_v, |p, q| p - q);
    let offset_4 = masked(difference_uv.map(|lane| h * lane));
    for (end, base) in [(a, out.addr()), (b, out.addr() + 0x80)] {
        let mut centre = end;
        centre[3] = convex_radius;
        let add = |offset: Vec4| zip_with(centre, offset, |p, q| p + q);
        let sub = |offset: Vec4| zip_with(centre, offset, |p, q| p - q);
        store_vec(e, base, add(offset_1));
        store_vec(e, base + 0x10, add(offset_3));
        store_vec(e, base + 0x20, add(offset_2));
        store_vec(e, base + 0x30, sub(offset_4));
        store_vec(e, base + 0x40, sub(offset_1));
        store_vec(e, base + 0x50, sub(offset_3));
        store_vec(e, base + 0x60, sub(offset_2));
        store_vec(e, base + 0x70, add(offset_4));
    }
    // `U x V` for the two end spheres, `yzx(U) * zxy(V) - zxy(U) * yzx(V)`.
    let cross = zip_with(
        zip_with(
            shuffle_yzx(direction_u),
            shuffle_zxy(direction_v),
            |p, q| p * q,
        ),
        zip_with(
            shuffle_zxy(direction_u),
            shuffle_yzx(direction_v),
            |p, q| p * q,
        ),
        |p, q| p - q,
    );
    let negated = x87_quiet(-end_offset);
    let radius_sum = x87_quiet((f64::from(convex_radius) + f64::from(end_offset)) as f32);
    store_vec(
        e,
        out.addr() + 0x100,
        zip_with(cross, a, |p, q| end_offset * p + q),
    );
    store_vec(
        e,
        out.addr() + 0x110,
        zip_with(cross, b, |p, q| negated * p + q),
    );
    e.mem.set_f32(out.addr() + 0x10c, radius_sum);
    e.mem.set_f32(out.addr() + 0x11c, radius_sum);
    out
}

// Translated from 00d57b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::getAabbImpl` (Ghidra name): transforms both axis end points by `transform`
/// (an `hkTransform`: three rotation columns then the translation, each a four-lane vector;
/// `p = (y*c1 + x*c0) + (z*c2 + t)`), and boxes them with a margin: with `d` the unit direction
/// from the first to the second (the `RSQRTSS` estimate times the difference) the per-axis
/// extent is `sqrt(1.01f - d_i^2)` (lane 3 keeps `d.w`), the margin is `(m_radius + tolerance) +
/// radius * extent`, and the box is the lane-wise `MINPS`/`MAXPS` of the two points widened by
/// that margin.
pub fn hkp_cylinder_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let me = this.addr();
    let columns: [Vec4; 4] =
        std::array::from_fn(|column| load_vec(e, transform.addr() + 0x10 * column as u32));
    let transformed = |point: Vec4| -> Vec4 {
        std::array::from_fn(|lane| {
            let x_and_y = point[1] * columns[1][lane] + point[0] * columns[0][lane];
            let z_and_translation = point[2] * columns[2][lane] + columns[3][lane];
            x_and_y + z_and_translation
        })
    };
    // The loop runs on `m_vertexB` first, then `m_vertexA`.
    let transformed_b = transformed(load_vec(e, me + VERTEX_B));
    let transformed_a = transformed(load_vec(e, me + VERTEX_A));
    let difference = zip_with(transformed_b, transformed_a, |p, q| p - q);
    let inverse = rsqrt_estimate(sum3(zip_with(difference, difference, |p, q| p * q)));
    let direction = difference.map(|lane| inverse * lane);
    let limit: f64 = e.global(EXTENT_LIMIT_DOUBLE);
    let mut extent = direction;
    for (slot, component) in extent.iter_mut().zip(direction).take(3) {
        let rest = (limit - f64::from(component) * f64::from(component)) as f32;
        *slot = x87_sqrt(e, rest);
    }
    let convex_radius = e.mem.f32(me + CONVEX_RADIUS);
    let margin = (f64::from(convex_radius) + f64::from(tolerance)) as f32;
    let radius = e.mem.f32(me + CYLINDER_RADIUS);
    let widen: Vec4 = extent.map(|lane| margin + radius * lane);
    let highest = zip_with(transformed_a, transformed_b, maxps);
    let lowest = zip_with(transformed_a, transformed_b, minps);
    store_vec(e, out.addr(), zip_with(lowest, widen, |p, q| p - q));
    store_vec(e, out.addr() + 0x10, zip_with(highest, widen, |p, q| p + q));
}

// Translated from 00d57d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::castRayImpl` (Ghidra name): casts the ray `input` (`hkpShapeRayCastInput`:
/// `m_from`, `m_to`) against the cylinder and updates `output` (`hkpShapeRayCastOutput`) when it
/// hits closer than `output.m_hitFraction`. Writes the hit flag (`hkBool`) to `result` and returns
/// `result`; a profiling record `"TtrcCylinder"` is written first and `"Et"` last.
///
/// The axis runs between `P0 = A + k` and `P1 = B - k` with `k = (U x V) * m_radius` (the
/// `zxy(U) * yzx(V) - yzx(U) * zxy(V)` form) and the radius is `r + m_radius`. A ray that starts
/// within that radius of the segment and between the planes of its ends misses. Otherwise the
/// closest points between the ray and the axis line (`00d22700`) give the entry fraction on the
/// infinite cylinder; when it lies between the end planes the hit is on the side (the normal
/// points away from the axis), else the ray is tested against the end disc it passes
/// (normal `-+axis`). The shape key slot `m_shapeKeyIndex` is set to -1.
pub fn hkp_cylinder_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CYLINDER);
    let hit = e.with_stack(0xc0, |e, scratch| {
        cast_ray_against_cylinder(e, this, input, output, scratch)
    });
    e.mem.set_u8(result.addr(), hit as u8);
    profile_record(e, PROFILE_END);
    result
}

/// The body of `castRayImpl`; `scratch` is 0xc0 bytes of the function's stack locals: the two
/// segment ends `P0` +0x30 and `P1` +0x40, the helper's three scalar results +0x24, +0x28, +0x2c,
/// the axis direction +0x60, the ray direction +0x70, the closest point +0x80 and the helper's
/// two vector results +0x90 and +0xa0.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the exe's flag tests, NaN included
fn cast_ray_against_cylinder(
    e: &mut Engine,
    this: Ptr,
    input: Ptr,
    output: Ptr,
    scratch: Ptr,
) -> bool {
    let me = this.addr();
    let out = output.addr();
    let from_address = input.addr() + RAY_FROM;
    let to_address = input.addr() + RAY_TO;
    let base = scratch.addr();
    let (p0_address, p1_address) = (base + 0x30, base + 0x40);
    let (distance_address, parameter_address, spare_address) =
        (base + 0x24, base + 0x28, base + 0x2c);
    let (axis_direction_address, ray_direction_address) = (base + 0x60, base + 0x70);
    let closest_address = base + 0x80;
    let (vector_b_address, vector_a_address) = (base + 0x90, base + 0xa0);

    // `FLD [radius]; FADD [m_radius]; FSTP`: rounded to a `float`.
    let reach = (f64::from(e.mem.f32(me + CYLINDER_RADIUS))
        + f64::from(e.mem.f32(me + CONVEX_RADIUS))) as f32;
    let direction_u = load_vec(e, me + DIRECTION_U);
    let direction_v = load_vec(e, me + DIRECTION_V);
    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let scale = e.mem.f32(me + CONVEX_RADIUS);
    let cross = zip_with(
        zip_with(
            shuffle_zxy(direction_u),
            shuffle_yzx(direction_v),
            |p, q| p * q,
        ),
        zip_with(
            shuffle_yzx(direction_u),
            shuffle_zxy(direction_v),
            |p, q| p * q,
        ),
        |p, q| p - q,
    );
    let offset = cross.map(|lane| lane * scale);
    let p0 = zip_with(a, offset, |p, q| p + q);
    let p1 = zip_with(b, offset, |p, q| p - q);
    store_vec(e, p0_address, p0);
    store_vec(e, p1_address, p1);
    e.call(
        CLOSEST_POINT_ON_SEGMENT,
        &args![from_address, p0_address, p1_address, closest_address],
    );

    let from = load_vec(e, from_address);
    let to = load_vec(e, to_address);
    let closest = load_vec(e, closest_address);
    let to_closest = zip_with(from, closest, |p, q| p - q);
    // `SQRTSS` of the squared distance, compared with the reach by `FCOMPP` (falls through when
    // the reach is greater).
    let distance = sum3(zip_with(to_closest, to_closest, |p, q| p * q)).sqrt();
    if reach > distance {
        let segment = zip_with(p1, p0, |p, q| p - q);
        let behind_start = sum3(zip_with(
            zip_with(from, p0, |p, q| p - q),
            segment,
            |p, q| p * q,
        ));
        if 0.0 < behind_start {
            let past_end = sum3(zip_with(
                zip_with(from, p1, |p, q| p - q),
                segment,
                |p, q| p * q,
            ));
            if 0.0 > past_end {
                return false;
            }
        }
    }

    let large_distance = x87_quiet(e.global::<f32>(LARGE_DISTANCE));
    e.mem.set_f32(distance_address, large_distance);
    let ray_direction = zip_with(to, from, |p, q| p - q);
    let axis_direction = zip_with(p1, p0, |p, q| p - q);
    store_vec(e, ray_direction_address, ray_direction);
    store_vec(e, axis_direction_address, axis_direction);
    e.call(
        CLOSEST_POINTS_WRAPPER,
        &args![
            from_address,
            ray_direction_address,
            p0_address,
            axis_direction_address,
            distance_address,
            parameter_address,
            spare_address,
            vector_a_address,
            vector_b_address
        ],
    );
    // `Rr * Rr` stays in x87 (exact) and is stored as a `double` too.
    let reach_squared = f64::from(reach) * f64::from(reach);
    let distance_squared = x87_quiet(e.mem.f32(distance_address));
    if f64::from(distance_squared) > reach_squared {
        return false;
    }

    let epsilon: f32 = e.global(EPSILON);
    let axis_length_squared = sum3(zip_with(axis_direction, axis_direction, |p, q| p * q));
    // `FCOMP` of the epsilon against the squared length: a miss unless the epsilon is smaller
    // (NaN is a miss).
    if !(epsilon < axis_length_squared) {
        return false;
    }
    // The unit axis and the axis length.
    let inverse = inverse_length(e, axis_length_squared);
    let axis = axis_direction.map(|lane| inverse * lane);
    let axis_length = axis_length_squared * inverse;

    // The ray direction without its part along the axis, and the fraction where the ray enters
    // the infinite cylinder (-1.0 when it has none).
    let product = zip_with(ray_direction, axis, |p, q| p * q);
    let along_axis = sum3(product);
    let minus_along = x87_quiet(-along_axis);
    let perpendicular = zip_with(axis, ray_direction, |u, d| minus_along * u + d);
    let perpendicular_squared = sum3(zip_with(perpendicular, perpendicular, |p, q| p * q));
    let entry = if perpendicular_squared == 0.0 {
        x87_quiet(e.global::<f32>(MINUS_ONE))
    } else {
        let parameter = f64::from(x87_quiet(e.mem.f32(parameter_address)));
        let numerator = (reach_squared - f64::from(distance_squared)) as f32;
        let inverse_perpendicular = inverse_length(e, perpendicular_squared);
        let root = x87_sqrt(e, numerator);
        let scaled = (f64::from(root) * f64::from(inverse_perpendicular)) as f32;
        (parameter - f64::from(scaled)) as f32
    };
    // A hit has to be nearer than the one already in the output: the `FCOMP` falls to the miss
    // for "less" and for "equal".
    let nearest = e.mem.f32(out + OUT_HIT_FRACTION);
    if (nearest < entry) != (nearest == entry) {
        return false;
    }

    let ones = load_vec(e, ONES);
    let point = interpolate(ones, from, to, entry);
    let a_along = sum3(zip_with(axis, p0, |p, q| p * q));
    let point_along = sum3(zip_with(point, axis, |p, q| p * q));
    let height = (f64::from(point_along) - f64::from(a_along)) as f32;
    // Entry at or after the ray start (not "less", not unordered), above the first end plane and
    // below the second: the side of the cylinder.
    if entry >= 0.0 && height > 0.0 && height < axis_length {
        let fraction = (f64::from(height) / f64::from(axis_length)) as f32;
        e.mem.set_f32(out + OUT_HIT_FRACTION, x87_quiet(entry));
        let on_axis = interpolate(ones, p0, p1, fraction);
        let normal = zip_with(point, on_axis, |p, q| p - q);
        let normal_inverse = inverse_length(e, sum3(zip_with(normal, normal, |p, q| p * q)));
        let key_index = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
        store_vec(
            e,
            out + OUT_NORMAL,
            normal.map(|lane| normal_inverse * lane),
        );
        e.mem.set_u32(
            out.wrapping_add(key_index.wrapping_mul(4))
                .wrapping_add(OUT_SHAPE_KEYS),
            0xffff_ffff,
        );
        return true;
    }

    // The end disc the ray passes: the first end plane when the start lies below it, else the
    // second one (which the start must lie beyond).
    let from_along = sum3(zip_with(from, axis, |p, q| p * q));
    let (side, end) = if from_along < a_along {
        (x87_quiet(e.global::<f32>(MINUS_ONE)), p0)
    } else {
        let far_plane = (f64::from(axis_length) + f64::from(a_along)) as f32;
        // `FCOMPP` then `JP`: a miss unless the far plane is below the start (NaN is a miss).
        if !(far_plane < from_along) {
            return false;
        }
        (1.0f32, p1)
    };
    let to_end_along = sum3(zip_with(zip_with(end, from, |p, q| p - q), axis, |p, q| {
        p * q
    }));
    // `side * g`, then `* -1.0` (a `double` constant): the distance to the plane along the axis.
    // `black_box` keeps the multiply: LLVM folds `x * -1.0` into a negation in release builds,
    // which flips a NaN's sign where the exe's `FMUL` passes it through.
    let plane_distance =
        ((f64::from(to_end_along) * f64::from(side)) * std::hint::black_box(-1.0)) as f32;
    if plane_distance < 0.0 {
        return false;
    }
    let negated_side = f64::from(side) * std::hint::black_box(-1.0);
    let approach = (negated_side * f64::from(sum3(product))) as f32;
    let scaled_nearest = f64::from(approach) * f64::from(nearest);
    // `FCOMP`: miss for "less" and for "equal".
    let plane_distance_wide = f64::from(plane_distance);
    if (scaled_nearest < plane_distance_wide) != (scaled_nearest == plane_distance_wide) {
        return false;
    }
    let fraction = (plane_distance_wide / f64::from(approach)) as f32;
    let at_plane = interpolate(ones, from, to, fraction);
    let in_plane = zip_with(at_plane, end, |p, q| p - q);
    let radial_squared = sum3(zip_with(in_plane, in_plane, |p, q| p * q));
    if f64::from(radial_squared) > reach_squared {
        return false;
    }
    let key_index = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
    e.mem.set_f32(out + OUT_HIT_FRACTION, x87_quiet(fraction));
    store_vec(e, out + OUT_NORMAL, axis.map(|lane| side * lane));
    e.mem.set_u32(
        out.wrapping_add(key_index.wrapping_mul(4))
            .wrapping_add(OUT_SHAPE_KEYS),
        0xffff_ffff,
    );
    true
}

/// `hkReferencedObject::calcContentStatistics` (`this` in `ECX`, collector, class) and the class
/// word `hkpCylinderShape::calcContentStatistics` passes to it.
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
const CYLINDER_SHAPE_CLASS: u32 = 0x0126_8940;
/// The vtable the constructors install first (`010305a8`) and the cylinder's own (`010d0f1c`).
const BASE_SHAPE_VTABLE: u32 = 0x0103_05a8;
const CYLINDER_VTABLE: u32 = 0x010d_0f1c;
/// The two values `+0xC` takes in the constructors: `0x12` while the first one runs and `2` at
/// the end.
const TYPE_INTERIM: u32 = 0x12;
const TYPE_FINAL: u32 = 2;
/// The step (a `double`, 0.01 as a `float`) and the limit (a `float`, 1.1) of the search in
/// `fn_00d571d0`.
const SEARCH_STEP: u32 = 0x0101_6408;
const SEARCH_LIMIT: u32 = 0x0105_6528;
/// The `float` the full constructor copies into `+0x18`.
const DEFAULT_RADIUS_FACTOR: u32 = 0x0103_0ff0;

// Translated from 00d571a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::getCylinderRadius` (Ghidra name): the `float` at `+0x14`, returned in
/// `ST0`.
pub fn hkp_cylinder_shape_get_cylinder_radius(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + CYLINDER_RADIUS)
}

// Translated from 00d571c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::calcContentStatistics` (Ghidra name): `ECX` = shape, the collector and one
/// more word on the stack that the code overwrites with the class word `01268940` before it
/// jumps to `hkReferencedObject::calcContentStatistics`.
pub fn hkp_cylinder_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused_1: u32,
) {
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, CYLINDER_SHAPE_CLASS],
    );
}

// Translated from 00d571d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function with no name in the engine map and no arguments, result in `ST0`. Starts from
/// 0 and adds the `double` at `01016408` (stored back as a `float`) until the value rounds to a
/// non-zero integer, which it then returns. If the value reaches the `float` limit at `01056528`
/// first (or the compare is unordered) it returns 1.0.
pub fn fn_00d571d0(e: &mut Engine) -> f32 {
    let step: f64 = e.global(SEARCH_STEP);
    let limit: f32 = e.global(SEARCH_LIMIT);
    let mut value = 0.0f32;
    while fistp(value) == 0 {
        value = (f64::from(value) + step) as f32;
        if limit.partial_cmp(&value) != Some(std::cmp::Ordering::Greater) {
            return 1.0;
        }
    }
    value
}

/// The lazy fix-up both constructors share: while the `float` at `INDEX_OFFSET` is negative
/// (it is -1.0 until then) it becomes `1 - fn_00d571d0()`.
fn settle_index_offset(e: &mut Engine) {
    let current: f32 = e.global(INDEX_OFFSET);
    if 0.0 > current {
        let found = fn_00d571d0(e);
        e.set_global(INDEX_OFFSET, (1.0 - f64::from(found)) as f32);
    }
}

// Translated from 00d57220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function with no name in the engine map: `ECX` = shape, nothing on the stack. Builds the
/// two cross section directions from the axis `B - A` (`+0x30` minus `+0x20`): the axis is
/// scaled by its inverse length (the `RSQRTSS` estimate refined once, as elsewhere in this
/// file). Of the magnitudes of its first two components the smaller one (the first when they
/// are equal) is compared with the third; the vector that has the larger of the first two
/// components' slot set to the axis component at the "other" index (the third, unless the
/// smaller magnitude is above the third's, then the smaller of the first two) and that other
/// slot set to the negated axis component of the larger slot, is normalised and stored at
/// `+0x40`; `axis x u` goes to `+0x50`.
pub fn fn_00d57220(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let span = zip_with(b, a, |p, q| p - q);
    let squared = sum3(zip_with(span, span, |p, q| p * q));
    let scale = inverse_length(e, squared);
    let axis = span.map(|lane| scale * lane);
    let magnitude: [f32; 3] = std::array::from_fn(|lane| x87_quiet(axis[lane].abs()));
    let (smaller, larger_index, smaller_index) = if magnitude[0] > magnitude[1] {
        (magnitude[1], 0, 1)
    } else {
        (magnitude[0], 1, 0)
    };
    let other_index = if smaller > magnitude[2] {
        smaller_index
    } else {
        2
    };
    let mut perpendicular = [0.0f32; 4];
    perpendicular[larger_index] = x87_quiet(axis[other_index]);
    perpendicular[other_index] = -x87_quiet(axis[larger_index]);
    let length_squared = sum3(zip_with(perpendicular, perpendicular, |p, q| p * q));
    let unit_scale = inverse_length(e, length_squared);
    let unit = perpendicular.map(|lane| unit_scale * lane);
    store_vec(e, me + DIRECTION_U, unit);
    let forward = zip_with(shuffle_zxy(unit), shuffle_yzx(axis), |p, q| p * q);
    let backward = zip_with(shuffle_yzx(unit), shuffle_zxy(axis), |p, q| p * q);
    store_vec(
        e,
        me + DIRECTION_V,
        zip_with(forward, backward, |p, q| p - q),
    );
}

// Translated from 00d573e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function with no name in the engine map: `ECX` = shape, one `float` on the stack. Stores
/// it as the cylinder radius (`+0x14`) and the sum of the `+0x10` radius and it in the fourth
/// lane of both axis end points (`+0x2c` and `+0x3c`).
pub fn fn_00d573e0(e: &mut Engine, this: Ptr, radius: f32) {
    let me = this.addr();
    let radius = x87_quiet(radius);
    e.mem.set_f32(me + CYLINDER_RADIUS, radius);
    // `FLD [+0x10]`, `FADD ST0,ST1`; then `FADD [+0x10]` (a memory operand) onto the radius:
    // the x87 choice of NaN.
    let convex = e.mem.u32(me + CONVEX_RADIUS);
    let sum = x87_add(x87_load32(convex), f64::from(radius));
    e.mem.set_u32(me + VERTEX_A + 0xc, x87_store32(sum));
    let sum = x87_add(f64::from(radius), x87_m32(convex));
    e.mem.set_u32(me + VERTEX_B + 0xc, x87_store32(sum));
}

// Translated from 00d57aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCylinderShape::hkpCylinderShape` (Ghidra name): `ECX` = shape; the stack holds the
/// addresses of the two axis end points, the cylinder radius and the `+0x10` radius (`float`s).
/// Sets the `+0x10` radius, `1` at `+6`, 0 at `+8`, the type 2 at `+0xC` and the vtable
/// `010d0f1c`; fixes up the constant at `011b5be8` the first time (see `settle_index_offset`);
/// copies the end points to `+0x20` and `+0x30`; runs `fn_00d573e0` and `fn_00d57220`; copies
/// the `float` at `01030ff0` into `+0x18`. Returns `this`.
pub fn hkp_cylinder_shape_constructor(
    e: &mut Engine,
    this: Ptr,
    end_a: u32,
    end_b: u32,
    radius: f32,
    convex_radius: f32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_f32(me + CONVEX_RADIUS, x87_quiet(convex_radius));
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, TYPE_FINAL);
    e.mem.set_u32(me, CYLINDER_VTABLE);
    settle_index_offset(e);
    copy_vec(e, end_a, me + VERTEX_A);
    copy_vec(e, end_b, me + VERTEX_B);
    fn_00d573e0(e, this, radius);
    fn_00d57220(e, this);
    let factor = e.mem.u32(DEFAULT_RADIUS_FACTOR);
    e.mem.set_u32(me + RADIUS_FACTOR, x87_quiet_bits(factor));
    this
}

// Translated from 00d57b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor with no name in the engine map: `ECX` = shape, one stack word (`flag`).
/// Installs the vtable `010305a8`, then (with a non-zero `flag` the code clears `+0xC` first)
/// sets the interim type `0x12` and the cylinder vtable `010d0f1c`; with a non-zero `flag` it
/// also fixes up the constant at `011b5be8` the first time (see `settle_index_offset`). `+0xC`
/// ends as 2. Returns `this`.
pub fn fn_00d57b20(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, BASE_SHAPE_VTABLE);
    if flag != 0 {
        e.mem.set_u32(me + 0xc, 0);
    }
    e.mem.set_u32(me + 0xc, TYPE_INTERIM);
    e.mem.set_u32(me, CYLINDER_VTABLE);
    if flag != 0 {
        settle_index_offset(e);
    }
    e.mem.set_u32(me + 0xc, TYPE_FINAL);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d571b0,
            hkp_cylinder_shape_get_num_collision_spheres_impl(Ptr) -> u32
        ),
        entry!(
            0x00d57400,
            hkp_cylinder_shape_get_supporting_vertex_impl(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d576d0,
            hkp_cylinder_shape_convert_vertex_ids_to_vertices_impl(Ptr, Ptr, i32, Ptr)
        ),
        entry!(0x00d57830, hkp_cylinder_shape_get_centre_impl(Ptr, Ptr)),
        entry!(0x00d57870, hkp_cylinder_shape_get_first_vertex(Ptr, Ptr)),
        entry!(
            0x00d57890,
            hkp_cylinder_shape_get_collision_spheres_impl(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d57b70,
            hkp_cylinder_shape_get_aabb_impl(Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d57d50,
            hkp_cylinder_shape_cast_ray_impl(Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d571a0,
            hkp_cylinder_shape_get_cylinder_radius(Ptr) -> f32
        ),
        entry!(
            0x00d571c0,
            hkp_cylinder_shape_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(0x00d571d0, fn_00d571d0() -> f32),
        entry!(0x00d57220, fn_00d57220(Ptr)),
        entry!(0x00d573e0, fn_00d573e0(Ptr, f32)),
        entry!(
            0x00d57aa0,
            hkp_cylinder_shape_constructor(Ptr, u32, u32, f32, f32) -> Ptr
        ),
        entry!(0x00d57b20, fn_00d57b20(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.set_global(HALF, 0.5f32);
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(EPSILON, f32::from_bits(0x3400_0000));
        e.set_global(LARGE_DISTANCE, f32::from_bits(0x7f7f_ffee));
        e.set_global(TINY_LENGTH_SQUARED, f32::from_bits(0x2880_0000));
        e.set_global(INDEX_SLOPE, f32::from_bits(0x402b_fe8d));
        e.set_global(ID_ANGLE_SCALE, f32::from_bits(0x3ebe_8495));
        e.set_global(INDEX_OFFSET, -1.0f32);
        e.set_global(INDEX_BIAS, f64::from_bits(0x3fa9_9999_a000_0000));
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(FOUR_DOUBLE, 4.0f64);
        e.set_global(ROOT_HALF_DOUBLE, f64::from_bits(0x3fe6_a09e_6000_0000));
        e.set_global(EXTENT_LIMIT_DOUBLE, f64::from_bits(0x3ff0_28f5_c000_0000));
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem
                .set_u32(LANE_MASK + 4 * lane, if lane < 3 { 0xffff_ffff } else { 0 });
        }
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> Ptr {
        let address = e.mem.alloc(16);
        store_vec(e, address, v);
        Ptr::new(address)
    }

    /// A cylinder from `(0,0,0)` to `(0,0,2)` with the cross section spanned by the x and y
    /// axes, radius 1 and `+0x10` radius `convex_radius`.
    fn cylinder(e: &mut Engine, convex_radius: f32, factor: f32) -> Ptr {
        let this = e.mem.alloc(0x60);
        e.mem.set_f32(this + CONVEX_RADIUS, convex_radius);
        e.mem.set_f32(this + CYLINDER_RADIUS, 1.0);
        e.mem.set_f32(this + RADIUS_FACTOR, factor);
        store_vec(e, this + VERTEX_A, [0.0, 0.0, 0.0, 0.0]);
        store_vec(e, this + VERTEX_B, [0.0, 0.0, 2.0, 0.0]);
        store_vec(e, this + DIRECTION_U, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, this + DIRECTION_V, [0.0, 1.0, 0.0, 0.0]);
        Ptr::new(this)
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn assert_near(actual: Vec4, expected: Vec4, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() <= tolerance,
                "{actual:?} != {expected:?}"
            );
        }
    }

    /// `(first, second)` of a vertex id with the given low four bits.
    fn angle_pair(low: u32) -> (f32, f32) {
        let step = (low as f64 + 0.5) as f32;
        let first = (step as f64 * f32::from_bits(0x3ebe_8495) as f64) as f32;
        let second = ((1.0 - first as f64 * first as f64) as f32).sqrt();
        (first, second)
    }

    #[test]
    fn num_collision_spheres_is_eighteen() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.0, 1.0);
        assert_eq!(e.call(0x00d571b0, &args![this]).u32(), 0x12);
    }

    #[test]
    fn centre_is_half_the_sum_of_the_ends() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.0, 1.0);
        let out = e.mem.alloc(16);
        e.call(0x00d57830, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn first_vertex_is_the_second_end() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.0, 1.0);
        let out = e.mem.alloc(16);
        e.call(0x00d57870, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 2.0, 0.0]);
    }

    #[test]
    fn supporting_vertex_expands_the_chosen_id() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.1, 1.0);
        let out = e.mem.alloc(16);
        let (first, second) = angle_pair(1);
        // Along +x: id 0xf1 (first end A, no negation, bit 4 set).
        let along_x = vector_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00d57400, &args![this, along_x, out]);
        assert_near(
            load_vec(&e, out),
            [second, first, 0.0, f32::from_bits(0x3f00_00f1)],
            1e-5,
        );
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_00f1);
        // Along -x and down the axis: id 0xd1 (the first direction's factor is negated).
        let down = vector_at(&mut e, [-1.0, 0.0, -1.0, 0.0]);
        e.call(0x00d57400, &args![this, down, out]);
        assert_near(
            load_vec(&e, out),
            [-second, first, 0.0, f32::from_bits(0x3f00_00d1)],
            1e-5,
        );
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_00d1);
        // Along +z there is no cross section part: id 0x71 on the second end B.
        let up = vector_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00d57400, &args![this, up, out]);
        assert_near(
            load_vec(&e, out),
            [second, first, 2.0, f32::from_bits(0x3f00_0071)],
            1e-5,
        );
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0071);
    }

    #[test]
    fn vertex_ids_expand_to_vertices() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.1, 1.0);
        let ids = e.mem.alloc(8);
        e.mem.set_u16(ids, 0xf1);
        e.mem.set_u16(ids + 2, 0);
        let out = e.mem.alloc(0x20);
        e.call(0x00d576d0, &args![this, ids, 2i32, out]);
        let (first_1, second_1) = angle_pair(1);
        assert_near(
            load_vec(&e, out),
            [second_1, first_1, 0.0, f32::from_bits(0x3f00_00f1)],
            1e-5,
        );
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_00f1);
        // Id 0: bits 5 and 6 clear negate both factors, bit 7 clear takes the end B.
        let (first_0, second_0) = angle_pair(0);
        assert_near(
            load_vec(&e, out + 0x10),
            [-first_0, -second_0, 2.0, 0.5],
            1e-5,
        );
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f00_0000);
        // A count of zero (or less) writes nothing.
        let untouched = e.mem.alloc(0x10);
        e.call(0x00d576d0, &args![this, ids, 0i32, untouched]);
        e.call(0x00d576d0, &args![this, ids, -2i32, untouched]);
        assert_eq!(e.mem.u32(untouched + 0xc), 0);
    }

    #[test]
    fn collision_spheres_answer_the_buffer() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.1, 0.5);
        let out = e.mem.alloc(0x120);
        // EAX is `out` at the RET (00d57a98); the convex transform shape reads it (00d5df2f).
        assert_eq!(e.call(0x00d5_7890, &args![this, out]).u32(), out);
    }

    #[test]
    fn collision_spheres_ring_both_ends() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.1, 0.5);
        let out = e.mem.alloc(0x120);
        e.call(0x00d57890, &args![this, out]);
        let (c, h) = (0.5f32, 0.5f32 * 0.707_106_77);
        let ring = [
            [c, 0.0, 0.0],
            [h, h, 0.0],
            [0.0, c, 0.0],
            [-h, h, 0.0],
            [-c, 0.0, 0.0],
            [-h, -h, 0.0],
            [0.0, -c, 0.0],
            [h, -h, 0.0],
        ];
        for (end, z) in [(0u32, 0.0f32), (0x80, 2.0)] {
            for (index, offset) in ring.iter().enumerate() {
                let sphere = load_vec(&e, out + end + 0x10 * index as u32);
                assert_near(sphere, [offset[0], offset[1], z, 0.1], 1e-5);
            }
        }
        // A long cylinder: the end spheres sit r along U x V = +z from the end points.
        assert_near(load_vec(&e, out + 0x100), [0.0, 0.0, 1.0, 1.1], 1e-5);
        assert_near(load_vec(&e, out + 0x110), [0.0, 0.0, 1.0, 1.1], 1e-5);
    }

    #[test]
    fn collision_spheres_of_a_short_cylinder_use_the_full_radius() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.1, 0.5);
        store_vec(&mut e, this.addr() + VERTEX_B, [0.0, 0.0, 0.5, 0.0]);
        let out = e.mem.alloc(0x120);
        e.call(0x00d57890, &args![this, out]);
        assert_near(load_vec(&e, out), [1.0, 0.0, 0.0, 0.1], 1e-5);
        assert_near(load_vec(&e, out + 0x100), [0.0, 0.0, 0.0, 0.1], 1e-5);
        assert_near(load_vec(&e, out + 0x110), [0.0, 0.0, 0.5, 0.1], 1e-5);
    }

    #[test]
    fn aabb_encloses_the_transformed_ends_and_grows_by_the_margin() {
        let mut e = engine();
        let this = cylinder(&mut e, 0.1, 1.0);
        let transform = e.mem.alloc(0x40);
        store_vec(&mut e, transform, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, transform + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, transform + 0x30, [10.0, 20.0, 30.0, 0.0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00d57b70, &args![this, transform, 0.5f32, out]);
        // Margin 0.6 plus the radius times (1.005, 1.005, about 0.1).
        let low = load_vec(&e, out);
        let high = load_vec(&e, out + 0x10);
        assert_near(low, [8.395, 18.395, 29.3, low[3]], 0.02);
        assert_near(high, [11.605, 21.605, 32.7, high[3]], 0.02);
    }

    // ---- castRayImpl

    /// A profile stream with room for two records; the TLS lookup returns it.
    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 3);
            ret(stream)
        });
        stream
    }

    struct Cast {
        hit: bool,
        output: u32,
        stream: u32,
    }

    /// Casts `from` -> `to` at the cylinder with `hit_fraction` already in the output;
    /// `closest` is what the doubled closest-point helper reports for the start, and
    /// `distance_squared` and `parameter` what the doubled closest-points wrapper reports.
    fn cast(
        e: &mut Engine,
        from: Vec4,
        to: Vec4,
        hit_fraction: f32,
        closest: Vec4,
        distance_squared: f32,
        parameter: f32,
    ) -> Cast {
        let this = cylinder(e, 0.0, 1.0);
        let stream = profile_stream(e);
        e.register_double(CLOSEST_POINT_ON_SEGMENT, move |e, a| {
            store_vec(e, a[3], closest);
            Ret::default()
        });
        e.register_double(CLOSEST_POINTS_WRAPPER, move |e, a| {
            assert_eq!(a.len(), 9);
            e.mem.set_f32(a[4], distance_squared);
            e.mem.set_f32(a[5], parameter);
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, hit_fraction);
        e.mem.set_u32(output + 0x40, 3);
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let returned = e
            .call(0x00d57d50, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        Cast {
            hit: e.mem.u8(result) != 0,
            output,
            stream,
        }
    }

    fn assert_profile_records(e: &Engine, stream: u32) {
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CYLINDER);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
    }

    #[test]
    fn cast_ray_hits_the_side() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [5.0, 0.0, 1.0, 0.0],
            [-5.0, 0.0, 1.0, 0.0],
            1.0,
            [0.0, 0.0, 1.0, 0.0],
            0.0,
            0.5,
        );
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 0.4).abs() < 1e-5);
        assert_near(load_vec(&e, c.output), [1.0, 0.0, 0.0, 0.0], 1e-5);
        assert_eq!(e.mem.u32(c.output + 0x20 + 3 * 4), 0xffff_ffff);
        assert_profile_records(&e, c.stream);
        assert_eq!(calls_to(&e, CLOSEST_POINT_ON_SEGMENT).len(), 1);
    }

    #[test]
    fn cast_ray_ignores_a_hit_farther_than_the_one_in_the_output() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [5.0, 0.0, 1.0, 0.0],
            [-5.0, 0.0, 1.0, 0.0],
            0.3,
            [0.0, 0.0, 1.0, 0.0],
            0.0,
            0.5,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.3);
        assert_eq!(e.mem.u32(c.output + 0x20 + 3 * 4), 0);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_misses_when_it_starts_inside() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [0.5, 0.0, 1.0, 0.0],
            [-5.0, 0.0, 1.0, 0.0],
            1.0,
            [0.0, 0.0, 1.0, 0.0],
            0.0,
            0.5,
        );
        assert!(!c.hit);
        assert!(calls_to(&e, CLOSEST_POINTS_WRAPPER).is_empty());
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_misses_when_the_lines_are_too_far_apart() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [3.0, 0.0, 5.0, 0.0],
            [3.0, 1.0, 4.0, 0.0],
            1.0,
            [0.0, 0.0, 2.0, 0.0],
            9.0,
            0.5,
        );
        assert!(!c.hit);
        assert_eq!(calls_to(&e, CLOSEST_POINTS_WRAPPER).len(), 1);
    }

    #[test]
    fn cast_ray_hits_the_far_end_disc() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [0.2, 0.0, 5.0, 0.0],
            [0.2, 0.0, -1.0, 0.0],
            1.0,
            [0.2, 0.0, 2.0, 0.0],
            0.04,
            0.0,
        );
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 0.5).abs() < 1e-5);
        assert_near(load_vec(&e, c.output), [0.0, 0.0, 1.0, 0.0], 1e-5);
        assert_eq!(e.mem.u32(c.output + 0x20 + 3 * 4), 0xffff_ffff);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_hits_the_near_end_disc() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [0.2, 0.0, -3.0, 0.0],
            [0.2, 0.0, 3.0, 0.0],
            1.0,
            [0.2, 0.0, 0.0, 0.0],
            0.04,
            0.0,
        );
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 0.5).abs() < 1e-5);
        assert_near(load_vec(&e, c.output), [0.0, 0.0, -1.0, 0.0], 1e-5);
    }

    #[test]
    fn cast_ray_moving_away_from_the_end_disc_misses() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [0.2, 0.0, 5.0, 0.0],
            [0.2, 0.0, 8.0, 0.0],
            1.0,
            [0.2, 0.0, 2.0, 0.0],
            0.04,
            0.0,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 1.0);
    }

    #[test]
    fn cast_ray_starting_between_the_end_planes_misses() {
        let mut e = engine();
        // The start lies between the end planes and within the radius of the axis.
        let c = cast(
            &mut e,
            [0.2, 0.0, 1.0, 0.0],
            [0.2, 0.0, 1.5, 0.0],
            1.0,
            [0.2, 0.0, 1.0, 0.0],
            0.04,
            0.0,
        );
        assert!(!c.hit);
    }

    #[test]
    fn profile_record_is_skipped_when_the_stream_is_full() {
        let mut e = engine();
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x20);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| ret(stream));
        profile_record(&mut e, 0x1234);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20);
        assert_eq!(e.mem.u32(stream + 0x20), 0);
    }

    #[test]
    fn fistp_rounds_to_nearest_even_and_flags_out_of_range() {
        assert_eq!(fistp(1.5), 2);
        assert_eq!(fistp(2.5), 2);
        assert_eq!(fistp(-0.5), 0);
        assert_eq!(fistp(f32::NAN), i32::MIN);
        assert_eq!(fistp(3.0e9), i32::MIN);
    }

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() < 1e-3
    }

    #[test]
    fn cylinder_radius_is_returned_in_st0() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        e.mem.set_f32(this + CYLINDER_RADIUS, 0.75);
        assert_eq!(e.call(0x00d5_71a0, &args![this]).f32(), 0.75);
    }

    #[test]
    fn calc_content_statistics_jumps_to_the_base_with_the_class() {
        let mut e = engine();
        e.register_double(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, |_, _| {
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d5_71c0, &args![0x1000u32, 0x2000u32, 0x9999u32]);
        assert_eq!(
            calls_to(&e, REFERENCED_OBJECT_CALC_CONTENT_STATISTICS),
            vec![vec![0x1000, 0x2000, 0x0126_8940]]
        );
    }

    #[test]
    fn the_search_stops_at_the_first_step_that_rounds_to_one() {
        let mut e = engine();
        e.set_global(SEARCH_STEP, f64::from(0.01f32));
        e.set_global(SEARCH_LIMIT, 1.1f32);
        let found = e.call(0x00d5_71d0, &args![]).f32();
        assert!((0.5..0.52).contains(&found), "{found}");
        // A limit at or below the first step ends the search with 1.0.
        e.set_global(SEARCH_LIMIT, 0.01f32);
        assert_eq!(e.call(0x00d5_71d0, &args![]).f32(), 1.0);
    }

    #[test]
    fn directions_follow_the_axis() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        store_vec(&mut e, this + VERTEX_A, [1.0, 2.0, 3.0, 5.0]);
        store_vec(&mut e, this + VERTEX_B, [1.0, 2.0, 7.0, 5.0]);
        e.call(0x00d5_7220, &args![this]);
        let u = load_vec(&e, this + DIRECTION_U);
        let v = load_vec(&e, this + DIRECTION_V);
        // The axis is +z: the first two components tie at zero, the third wins.
        assert!(near(u[0], 0.0) && near(u[1], 1.0) && near(u[2], 0.0) && u[3] == 0.0);
        assert!(near(v[0], -1.0) && near(v[1], 0.0) && near(v[2], 0.0));
    }

    #[test]
    fn directions_avoid_the_smallest_component() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        store_vec(&mut e, this + VERTEX_A, [0.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, this + VERTEX_B, [2.0, 3.0, 1.0, 0.0]);
        e.call(0x00d5_7220, &args![this]);
        let u = load_vec(&e, this + DIRECTION_U);
        let v = load_vec(&e, this + DIRECTION_V);
        // The third component is the smallest, so the first perpendicular has none of it.
        let length = 13.0f32.sqrt();
        assert!(near(u[0], -3.0 / length) && near(u[1], 2.0 / length) && u[2] == 0.0);
        let axis = [
            2.0 / 14.0f32.sqrt(),
            3.0 / 14.0f32.sqrt(),
            1.0 / 14.0f32.sqrt(),
        ];
        let dot = u[0] * axis[0] + u[1] * axis[1] + u[2] * axis[2];
        assert!(near(dot, 0.0));
        let dot = v[0] * axis[0] + v[1] * axis[1] + v[2] * axis[2];
        assert!(near(dot, 0.0));
    }

    #[test]
    fn radius_setter_fills_the_end_point_fourth_lanes() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        e.mem.set_f32(this + CONVEX_RADIUS, 0.5);
        e.call(0x00d5_73e0, &args![this, 0.25f32]);
        assert_eq!(e.mem.f32(this + CYLINDER_RADIUS), 0.25);
        assert_eq!(e.mem.f32(this + VERTEX_A + 0xc), 0.75);
        assert_eq!(e.mem.f32(this + VERTEX_B + 0xc), 0.75);
    }

    #[test]
    fn constructor_builds_the_shape_and_settles_the_constant_once() {
        let mut e = engine();
        e.set_global(SEARCH_STEP, f64::from(0.01f32));
        e.set_global(SEARCH_LIMIT, 1.1f32);
        e.set_global(DEFAULT_RADIUS_FACTOR, 0.7f32);
        let this = e.mem.alloc(0x60);
        let end_a = vector_at(&mut e, [0.0, 0.0, 0.0, 9.0]).addr();
        let end_b = vector_at(&mut e, [0.0, 0.0, 2.0, 9.0]).addr();
        let returned = e
            .call(0x00d5_7aa0, &args![this, end_a, end_b, 0.25f32, 0.5f32])
            .u32();
        assert_eq!(returned, this);
        assert_eq!(e.mem.f32(this + CONVEX_RADIUS), 0.5);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this), 0x010d_0f1c);
        assert_eq!(e.mem.f32(this + CYLINDER_RADIUS), 0.25);
        assert_eq!(load_vec(&e, this + VERTEX_A), [0.0, 0.0, 0.0, 0.75]);
        assert_eq!(load_vec(&e, this + VERTEX_B), [0.0, 0.0, 2.0, 0.75]);
        assert!(near(e.mem.f32(this + DIRECTION_U + 4), 1.0));
        assert_eq!(e.mem.f32(this + RADIUS_FACTOR), 0.7);
        let settled: f32 = e.global(INDEX_OFFSET);
        assert!((0.0..0.5).contains(&settled), "{settled}");
        // Once positive the constant is left alone.
        e.set_global(INDEX_OFFSET, 0.25f32);
        e.call(0x00d5_7aa0, &args![this, end_a, end_b, 0.25f32, 0.5f32]);
        assert_eq!(e.global::<f32>(INDEX_OFFSET), 0.25);
    }

    #[test]
    fn loaded_object_constructor_only_settles_the_constant_when_asked() {
        let mut e = engine();
        e.set_global(SEARCH_STEP, f64::from(0.01f32));
        e.set_global(SEARCH_LIMIT, 1.1f32);
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00d5_7b20, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_0f1c);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.global::<f32>(INDEX_OFFSET), -1.0);
        e.call(0x00d5_7b20, &args![this, 1u32]);
        let settled: f32 = e.global(INDEX_OFFSET);
        assert!((0.0..0.5).contains(&settled), "{settled}");
    }
}
