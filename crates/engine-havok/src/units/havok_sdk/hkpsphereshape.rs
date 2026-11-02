//! `Havok SDK/hkpsphereshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpSphereShape` is a sphere of radius `m_radius` (+0x10) around the
//! origin of its own space; it has no vertices of its own, so every vertex
//! getter answers zero. SSE code (`SUBPS`, `MULPS`, `ADDPS`, `RCPPS`,
//! `RSQRTPS`) computes lane by lane in `f32`; x87 code rounds at each
//! `float` store and compares the way its `FNSTSW`/`TEST AH` pairs read the
//! flags (NaN included). `RCPPS` and `RSQRTPS` are the host CPU's estimates.
//!
//! The vector helpers at the top are shared with the other shape units of
//! this group (`hkptriangleshape`, `hkpconvextransformshape`,
//! `hkpconvextranslateshape`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

layout! {
    /// `hkpSphereShape` (Xbox PDB), 0x20 bytes on the Xbox.
    pub struct HkpSphereShape: 0x20 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_radius` (Xbox PDB), inherited from `hkpConvexShape`.
        0x10 m_radius: f32,
    }
}

/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
pub(crate) const TLS_GET_VALUE: u32 = 0x00fd_f044;
pub(crate) const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: the begin records of `castRayImpl` and
/// `castRayBundleImpl`, and the shared end label `"Et"`.
const PROFILE_BEGIN_CAST_RAY: u32 = 0x010c_b5f4;
const PROFILE_BEGIN_CAST_RAY_BUNDLE: u32 = 0x010c_b600;
pub(crate) const PROFILE_END: u32 = 0x010c_41f8;

/// `float` constants: zero, 3.0 and 0.5 (the `RSQRTSS` refinement).
pub(crate) const ZERO_F32: u32 = 0x0101_1d78;
pub(crate) const THREE_F32: u32 = 0x0101_7718;
pub(crate) const HALF_F32: u32 = 0x0101_6248;
/// `100.0` as a `double`: `castRayUtil` scales its threshold by it.
const HUNDRED: u32 = 0x0101_7a40;
/// `100.0` as a `float` (`castRayBundleImpl`'s threshold factor).
const BUNDLE_THRESHOLD_FACTOR: u32 = 0x0101_6410;
/// Vector constants: four 1.0 (two copies, `castRayBundleImpl` and the
/// triangle code use the first, `castRayUtil` the second), four sign bits,
/// four 3.0 and four 0.5.
pub(crate) const ONES: u32 = 0x010c_7170;
const ONES_RAY_UTIL: u32 = 0x010e_c7f0;
pub(crate) const SIGN_MASK: u32 = 0x010c_44d0;
const THREE_VECTOR: u32 = 0x010c_71c0;
const HALF_VECTOR: u32 = 0x010c_71d0;
/// Four dwords `1, 2, 4, 8`: the `MOVMSKPS` bit of each ray of a bundle.
pub(crate) const LANE_BITS: u32 = 0x010c_92e0;

// `hkpShapeRayCastOutput` (Xbox PDB): `m_normal` +0x00, `m_hitFraction`
// +0x10, `m_extraInfo` +0x14, `m_shapeKeys` +0x20, `m_shapeKeyIndex` +0x40;
// 0x50 bytes in all.
pub(crate) const OUT_HIT_FRACTION: u32 = 0x10;
pub(crate) const OUT_SHAPE_KEYS: u32 = 0x20;
pub(crate) const OUT_SHAPE_KEY_INDEX: u32 = 0x40;

pub(crate) type Vec4 = [f32; 4];

pub(crate) fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

pub(crate) fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// The four dwords of a vector, bit for bit.
pub(crate) fn load_bits(e: &Engine, addr: u32) -> [u32; 4] {
    std::array::from_fn(|lane| e.mem.u32(addr + 4 * lane as u32))
}

pub(crate) fn store_bits(e: &mut Engine, addr: u32, v: [u32; 4]) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
pub(crate) fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

pub(crate) fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
pub(crate) fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

pub(crate) fn dot3(a: Vec4, b: Vec4) -> f32 {
    sum3(zip_with(a, b, |x, y| x * y))
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
pub(crate) fn shufps(dst: Vec4, src: Vec4, imm: u8) -> Vec4 {
    let pick = |from: Vec4, shift: u8| from[((imm >> shift) & 3) as usize];
    [pick(dst, 0), pick(dst, 2), pick(src, 4), pick(src, 6)]
}

/// `UNPCKLPS a, b`.
pub(crate) fn unpcklps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0], b[0], a[1], b[1]]
}

/// `UNPCKHPS a, b`.
pub(crate) fn unpckhps(a: Vec4, b: Vec4) -> Vec4 {
    [a[2], b[2], a[3], b[3]]
}

/// `MOVLHPS a, b`: the low half of `b` becomes the high half of `a`.
pub(crate) fn movlhps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0], a[1], b[0], b[1]]
}

/// `MOVHLPS a, b`: the high half of `b` becomes the low half of `a`.
pub(crate) fn movhlps(a: Vec4, b: Vec4) -> Vec4 {
    [b[2], b[3], a[2], a[3]]
}

/// `MAXPS dst, src` per lane: `dst` when it is greater, else `src`.
pub(crate) fn maxps(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// `MINPS dst, src` per lane: `dst` when it is smaller, else `src`.
pub(crate) fn minps(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// A lane compare result as the all-ones or all-zero dword `CMPxxPS` makes.
pub(crate) fn mask_of(test: bool) -> u32 {
    if test {
        u32::MAX
    } else {
        0
    }
}

/// `RCPPS` of one lane (the host CPU's estimate on x86).
#[cfg(target_arch = "x86_64")]
pub(crate) fn rcp_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn rcp_estimate(x: f32) -> f32 {
    1.0 / x
}

/// One Newton step on the reciprocal estimate, the way the code refines it:
/// `(r + r) - ((r * x) * r)`.
pub(crate) fn refined_reciprocal(x: f32) -> f32 {
    let r = rcp_estimate(x);
    (r + r) - (r * x) * r
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero
/// guard: `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == 0`.
pub(crate) fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(ZERO_F32);
    let three: f32 = e.global(THREE_F32);
    let half: f32 = e.global(HALF_F32);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record
/// stream (`TlsGetValue` of the index at `0126835c`) while its cursor (`+4`)
/// is below its limit (`+0xC`); a record is 12 bytes.
pub(crate) fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// `hkReferencedObject::calcContentStatistics` (`00c90360`, `this` in `ECX`, collector, class)
/// and the `hkClass` of the sphere shape that `hkpSphereShape::calcContentStatistics` passes
/// to it.
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
const SPHERE_SHAPE_CLASS: u32 = 0x0126_8940;
/// The vtable of `hkpSphereShape`.
const SPHERE_SHAPE_VTABLE: u32 = 0x010c_b5ac;

// Translated from 00cd8040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::calcContentStatistics` (Xbox PDB): `this` in `ECX`, the collector and one
/// more word on the stack; jumps to `hkReferencedObject::calcContentStatistics` with the
/// second word replaced by the class at `01268940`.
pub fn hkp_sphere_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpSphereShape>,
    collector: u32,
    _unused_1: u32,
) {
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, SPHERE_SHAPE_CLASS],
    );
}

// Translated from 00cd8160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::hkpSphereShape` (Xbox PDB), `this` in `ECX`, the radius on the stack:
/// stores the radius at +0x10 (through the x87 stack: a signalling NaN comes out quiet), `1`
/// in the 16-bit field at +6, `0` at +8, `1` at +0xc and the vtable `010cb5ac`. Returns
/// `this`.
pub fn hkp_sphere_shape_construct(
    e: &mut Engine,
    this: Ptr<HkpSphereShape>,
    radius: f32,
) -> Ptr<HkpSphereShape> {
    let base = this.addr();
    e.mem.set_u32(base + 0x10, x87_quiet_bits(radius.to_bits()));
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 1);
    e.mem.set_u32(base, SPHERE_SHAPE_VTABLE);
    this
}

// Translated from 00cd8050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::getNumCollisionSpheresImpl` (Xbox PDB): 1.
pub fn hkp_sphere_shape_get_num_collision_spheres_impl(
    _e: &mut Engine,
    _this: Ptr<HkpSphereShape>,
) -> u32 {
    1
}

// Translated from 00cd8060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::getSupportingVertexImpl` (Xbox PDB): the sphere is a
/// point, so the supporting vertex written to `out` is zero; `direction` is
/// never read.
pub fn hkp_sphere_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    _this: Ptr<HkpSphereShape>,
    _direction: Ptr,
    out: Ptr,
) {
    store_vec(e, out.addr(), [0.0; 4]);
}

// Translated from 00cd8080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::convertVertexIdsToVerticesImpl` (Xbox PDB): writes
/// `count` zero vectors to `out` (a zero vector, then an overlapping
/// `REP MOVSD` of `((count - 1) & 0x0fffffff) * 4` dwords 16 bytes ahead
/// that propagates it). Nothing is written for `count <= 0`; the ids are
/// never read.
pub fn hkp_sphere_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    _this: Ptr<HkpSphereShape>,
    _ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let last = count.wrapping_sub(1);
    if last < 0 {
        return;
    }
    let out = out.addr();
    store_vec(e, out, [0.0; 4]);
    let dwords = (last as u32 & 0x0fff_ffff) * 4;
    for dword in 0..dwords {
        let word = e.mem.u32(out + 4 * dword);
        e.mem.set_u32(out + 0x10 + 4 * dword, word);
    }
}

// Translated from 00cd80c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::getCentreImpl` (Xbox PDB): the centre is zero.
pub fn hkp_sphere_shape_get_centre_impl(e: &mut Engine, _this: Ptr<HkpSphereShape>, out: Ptr) {
    store_vec(e, out.addr(), [0.0; 4]);
}

// Translated from 00cd80e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::getFirstVertex` (Xbox PDB): zero.
pub fn hkp_sphere_shape_get_first_vertex(e: &mut Engine, _this: Ptr<HkpSphereShape>, out: Ptr) {
    store_vec(e, out.addr(), [0.0; 4]);
}

// Translated from 00cd8100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::getAabbImpl` (Xbox PDB): the box around the
/// translation of `transform` (an `hkTransform`; its translation is the
/// fourth vector, +0x30), widened by `m_radius + tolerance` (rounded to
/// `float`) on every lane.
pub fn hkp_sphere_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpSphereShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    // `FLD m_radius`, `FADD` the tolerance argument (a memory operand): the x87 choice of
    // NaN; then `SUBPS`/`ADDPS` with the SSE NaN rules (the destination first).
    let radius = e.get(this, HkpSphereShape::m_radius);
    let sum = x87_add(x87_load32(radius.to_bits()), x87_m32(tolerance.to_bits()));
    let margin = [f32::from_bits(x87_store32(sum)); 4];
    let centre = load_vec(e, transform.addr() + 0x30);
    store_vec(e, out.addr(), crate::sse::subps(centre, margin));
    let centre = load_vec(e, transform.addr() + 0x30);
    store_vec(e, out.addr() + 0x10, crate::sse::addps(margin, centre));
}

// Translated from 00cd8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::getCollisionSpheresImpl` (Xbox PDB): the one collision
/// sphere is the origin with the radius in its `w` lane (the radius goes
/// through `FLD`/`FSTP`).
/// Answers `out` (the `const hkSphere*` of the Xbox PDB signature; `EAX` is `out` at the
/// RET 00cd8158): hkpConvexTransformShape::getCollisionSpheresImpl reads it through its child
/// shape's slot +0x30 (`MOV EDI,EAX` at 00d5df2f).
pub fn hkp_sphere_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpSphereShape>,
    out: Ptr,
) -> Ptr {
    store_vec(e, out.addr(), [0.0; 4]);
    let radius = e.get(this, HkpSphereShape::m_radius);
    e.mem.set_f32(out.addr() + 0xc, x87_quiet(radius));
    out
}

// Translated from 00cd8190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::castRayImpl` (Xbox PDB): casts the ray `input`
/// (`hkpShapeRayCastInput`: `m_from`, `m_to`) at the sphere with
/// `hkpSphereUtil::castRayUtil`, writing the hit flag (`hkBool`) to `result`
/// and returning `result`. A profiling record is written before and after.
pub fn hkp_sphere_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpSphereShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CAST_RAY);
    let radius = x87_quiet(e.get(this, HkpSphereShape::m_radius));
    hkp_sphere_util_cast_ray_util(e, result, radius, input, output);
    profile_record(e, PROFILE_END);
    result
}

// Translated from 00cd85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereUtil::castRayUtil` (Xbox PDB, cdecl `(result, radius, rays,
/// output)`): casts the ray `rays` (`m_from` +0, `m_to` +0x10) at a sphere
/// of `radius` around the origin; on a hit nearer than
/// `output.m_hitFraction` it writes the fraction, the unit hit normal
/// (`point / radius`) and `-1` to the current shape key slot, and stores
/// 1 in `result` (0 otherwise). Returns `result`.
///
/// With `b = from . (to - from)` and `l = |to - from|^2` (both as `float`),
/// a ray that does not move toward the centre (`b >= 0`) misses. When
/// `l * r^2 * 100 < b^2` the closest point of the ray's line to the centre
/// is the start of the quadratic (a start inside the sphere, `l < r^2`,
/// misses), otherwise the ray's own start is. The discriminant has to be
/// positive; the entry then has to be nearer than the old hit fraction
/// (`l * old > entry`) and not negative.
pub fn hkp_sphere_util_cast_ray_util(
    e: &mut Engine,
    result: Ptr,
    radius: f32,
    rays: Ptr,
    output: Ptr,
) -> Ptr {
    let hit = sphere_cast(e, radius, rays.addr(), output.addr());
    e.mem.set_u8(result.addr(), hit as u8);
    result
}

fn sphere_cast(e: &mut Engine, radius: f32, rays: u32, out: u32) -> bool {
    // `FMUL ST0; FSTP float`.
    let radius_squared = (radius as f64 * radius as f64) as f32;
    let from = load_vec(e, rays);
    let to = load_vec(e, rays + 0x10);
    let direction = zip_with(to, from, |t, f| t - f);
    // `FLD`/`FST`: the dot product goes through x87.
    let along = x87_quiet(dot3(from, direction));
    // `FLDZ; FCOM; TEST AH,0x41; JNP`: the ray continues only when
    // `along < 0` (or is NaN).
    if along >= 0.0 {
        return false;
    }
    let length_squared = dot3(direction, direction);
    let hundred: f64 = e.global(HUNDRED);
    let threshold = ((length_squared as f64 * radius_squared as f64) * hundred) as f32;
    let along_squared = along as f64 * along as f64;
    let ones = load_vec(e, ONES_RAY_UTIL);
    // `start_offset` is the distance already walked to the start point and
    // `start_along` the dot product of the start with the direction.
    let (start_offset, start_along, start): (f32, f32, Vec4) = if (threshold as f64) < along_squared
    {
        if length_squared < radius_squared {
            return false;
        }
        let offset = x87_quiet(-along);
        let fraction = (offset as f64 / length_squared as f64) as f32;
        let point =
            std::array::from_fn(|lane| (ones[lane] - fraction) * from[lane] + to[lane] * fraction);
        (offset, 0.0, point)
    } else {
        (0.0, along, from)
    };
    let gap = (sum3(start.map(|lane| lane * lane)) as f64 - radius_squared as f64) as f32;
    let discriminant =
        (start_along as f64 * start_along as f64 - length_squared as f64 * gap as f64) as f32;
    // `FCOM` against zero: a miss when `discriminant <= 0` (ordered).
    if discriminant <= 0.0 {
        return false;
    }
    let root = e.call(X87_SQRT, &args![discriminant as f64]).f64() as f32;
    let behind = x87_quiet(-start_along);
    let nearer = (behind as f64 - root as f64) as f32;
    let entry = (nearer as f64 + start_offset as f64) as f32;
    let old_fraction = e.mem.f32(out + OUT_HIT_FRACTION);
    let scaled = (length_squared as f64 * old_fraction as f64) as f32;
    // The hit has to be nearer than the old one and not behind the start.
    if scaled <= entry || entry < 0.0 {
        return false;
    }
    let fraction = (entry as f64 / length_squared as f64) as f32;
    let key_index = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
    e.mem.set_f32(out + OUT_HIT_FRACTION, x87_quiet(fraction));
    let inverse_radius = (1.0 / radius as f64) as f32;
    let point: Vec4 =
        std::array::from_fn(|lane| (ones[lane] - fraction) * from[lane] + to[lane] * fraction);
    store_vec(e, out, point.map(|lane| inverse_radius * lane));
    e.mem.set_u32(
        out.wrapping_add(key_index.wrapping_mul(4))
            .wrapping_add(OUT_SHAPE_KEYS),
        0xffff_ffff,
    );
    true
}

// Translated from 00cd8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereShape::castRayBundleImpl` (Xbox PDB): casts four rays at once.
/// `input` is an `hkpShapeRayBundleCastInput` in structure-of-arrays form
/// (`from.x`, `from.y`, `from.z`, `to.x`, `to.y`, `to.z`, each a vector of
/// the four rays), `outputs` four `hkpShapeRayCastOutput`s (0x50 bytes
/// apart) and `in_mask` the rays to cast (four dwords, all ones or zero).
/// The mask of rays that hit is written to `result` (also returned); each
/// hit ray's output gets the hit fraction, its unit normal and `-1` in the
/// current shape key slot.
///
/// Per ray, with `b = from . d`, `l = d . d` (`d = to - from`) and `r2 =
/// radius^2`: the start is moved to the closest point of the line
/// (`t = -b / l`) when `l * r2 * 100 < b^2`; the discriminant is
/// `L^2 - l * (|p|^2 - r2)` (`L` is 0 for a moved start, else `b`); the
/// entry fraction is `((offset - L) - sqrt(disc)) / l` with `offset = -b` for
/// a moved start and 0 otherwise, the reciprocal and the square root
/// refined by one Newton step. A ray hits when
/// `0 < disc`, `entry < old hit fraction` and `0 <= entry`.
pub fn hkp_sphere_shape_cast_ray_bundle_impl(
    e: &mut Engine,
    this: Ptr<HkpSphereShape>,
    result: Ptr,
    input: Ptr,
    outputs: Ptr,
    in_mask: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CAST_RAY_BUNDLE);
    let result = result.addr();
    let outputs = outputs.addr();
    let radius = e.get(this, HkpSphereShape::m_radius);
    let factor = x87_quiet(e.global::<f32>(BUNDLE_THRESHOLD_FACTOR));
    for word in 0..4 {
        let value = e.mem.u32(in_mask.addr() + 4 * word);
        e.mem.set_u32(result + 4 * word, value);
    }
    let input = input.addr();
    let from: [Vec4; 3] = std::array::from_fn(|axis| load_vec(e, input + 0x10 * axis as u32));
    let to: [Vec4; 3] = std::array::from_fn(|axis| load_vec(e, input + 0x30 + 0x10 * axis as u32));
    let lane_ones = load_vec(e, ONES);
    let signs = load_bits(e, SIGN_MASK);
    let three = load_vec(e, THREE_VECTOR);
    let half = load_vec(e, HALF_VECTOR);
    let mask_in = load_bits(e, result);

    let delta: [Vec4; 3] = std::array::from_fn(|axis| zip_with(to[axis], from[axis], |t, f| t - f));
    let rad = [radius; 4];
    let rad_squared = zip_with(rad, rad, |a, b| a * b);
    let mut b = [0.0f32; 4];
    let mut l = [0.0f32; 4];
    for lane in 0..4 {
        let along_x = from[0][lane] * delta[0][lane];
        let along_y = from[1][lane] * delta[1][lane];
        let along_z = from[2][lane] * delta[2][lane];
        b[lane] = (along_y + along_x) + along_z;
        let dx = delta[0][lane] * delta[0][lane];
        let dy = delta[1][lane] * delta[1][lane];
        let dz = delta[2][lane] * delta[2][lane];
        l[lane] = (dy + dx) + dz;
    }
    let mut neg_b_or_zero = [0.0f32; 4];
    let mut a_squared = [0.0f32; 4];
    let mut a_linear = [0.0f32; 4];
    let mut inverse_l = [0.0f32; 4];
    let mut t = [0.0f32; 4];
    let mut point: [Vec4; 3] = [[0.0; 4]; 3];
    for lane in 0..4 {
        let b_squared = b[lane] * b[lane];
        // `CMPLTPS`: the line's closest point is used as the start.
        let moved = (l[lane] * rad_squared[lane]) * factor < b_squared;
        let selected = if moved { b[lane].to_bits() } else { 0 };
        neg_b_or_zero[lane] = f32::from_bits(selected ^ signs[lane]);
        a_squared[lane] = if moved { 0.0 } else { b_squared };
        a_linear[lane] = if moved { 0.0 } else { b[lane] };
        inverse_l[lane] = refined_reciprocal(l[lane]);
        t[lane] = neg_b_or_zero[lane] * inverse_l[lane];
        for axis in 0..3 {
            point[axis][lane] =
                to[axis][lane] * t[lane] + (lane_ones[lane] - t[lane]) * from[axis][lane];
        }
    }
    let mut accumulated = [0.0f32; 4];
    for coordinate in point {
        for lane in 0..4 {
            accumulated[lane] += coordinate[lane] * coordinate[lane];
        }
    }
    let mut entry = [0.0f32; 4];
    let mut mask = [0u32; 4];
    let hit_fractions: Vec4 =
        std::array::from_fn(|ray| e.mem.f32(outputs + 0x50 * ray as u32 + OUT_HIT_FRACTION));
    for lane in 0..4 {
        let negated_l = f32::from_bits(l[lane].to_bits() ^ signs[lane]);
        let discriminant = (accumulated[lane] - rad_squared[lane]) * negated_l + a_squared[lane];
        let positive = 0.0 < discriminant;
        let estimate = rsqrt_estimate(discriminant);
        let root_scale = (three[lane] - (discriminant * estimate) * estimate)
            * (estimate * half[lane])
            * discriminant;
        let root = if positive { root_scale } else { 0.0 };
        entry[lane] = ((neg_b_or_zero[lane] - a_linear[lane]) - root) * inverse_l[lane];
        mask[lane] = mask_in[lane]
            & mask_of(positive)
            & mask_of(entry[lane] < hit_fractions[lane])
            & mask_of(0.0 <= entry[lane]);
    }
    store_bits(e, result, mask);

    // The unit normals: `(to * entry + from * (1 - entry)) / radius`.
    let inverse_radius = refined_reciprocal(radius);
    let sign_bits = (0..4).fold(0u32, |bits, ray| bits | ((mask[ray] >> 31) << ray));
    for lane in 0..4 {
        // `MOVMSKPS` / `TEST [table], bits`.
        if e.mem.u32(LANE_BITS + 4 * lane as u32) & sign_bits == 0 {
            continue;
        }
        let mut normal = [0.0f32; 4];
        for axis in 0..3 {
            let sum =
                to[axis][lane] * entry[lane] + from[axis][lane] * (lane_ones[lane] - entry[lane]);
            normal[axis] = inverse_radius * sum;
        }
        let output = outputs + 0x50 * lane as u32;
        e.mem
            .set_f32(output + OUT_HIT_FRACTION, x87_quiet(entry[lane]));
        store_vec(e, output, normal);
        let key_index = e.mem.u32(output + OUT_SHAPE_KEY_INDEX);
        e.mem.set_u32(
            output
                .wrapping_add(key_index.wrapping_mul(4))
                .wrapping_add(OUT_SHAPE_KEYS),
            0xffff_ffff,
        );
    }
    profile_record(e, PROFILE_END);
    Ptr::new(result)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cd8040,
            hkp_sphere_shape_calc_content_statistics(Ptr<HkpSphereShape>, u32, u32)
        ),
        entry!(
            0x00cd8160,
            hkp_sphere_shape_construct(Ptr<HkpSphereShape>, f32) -> Ptr<HkpSphereShape>
        ),
        entry!(
            0x00cd8050,
            hkp_sphere_shape_get_num_collision_spheres_impl(Ptr<HkpSphereShape>) -> u32
        ),
        entry!(
            0x00cd8060,
            hkp_sphere_shape_get_supporting_vertex_impl(Ptr<HkpSphereShape>, Ptr, Ptr)
        ),
        entry!(
            0x00cd8080,
            hkp_sphere_shape_convert_vertex_ids_to_vertices_impl(
                Ptr<HkpSphereShape>,
                Ptr,
                i32,
                Ptr,
            )
        ),
        entry!(
            0x00cd80c0,
            hkp_sphere_shape_get_centre_impl(Ptr<HkpSphereShape>, Ptr)
        ),
        entry!(
            0x00cd80e0,
            hkp_sphere_shape_get_first_vertex(Ptr<HkpSphereShape>, Ptr)
        ),
        entry!(
            0x00cd8100,
            hkp_sphere_shape_get_aabb_impl(Ptr<HkpSphereShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00cd8140,
            hkp_sphere_shape_get_collision_spheres_impl(Ptr<HkpSphereShape>, Ptr) -> Ptr
        ),
        entry!(
            0x00cd8190,
            hkp_sphere_shape_cast_ray_impl(Ptr<HkpSphereShape>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00cd8220,
            hkp_sphere_shape_cast_ray_bundle_impl(Ptr<HkpSphereShape>, Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00cd85c0,
            hkp_sphere_util_cast_ray_util(Ptr, f32, Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(BUNDLE_THRESHOLD_FACTOR, 100.0f32);
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem.set_f32(ONES_RAY_UTIL + 4 * lane, 1.0);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_f32(THREE_VECTOR + 4 * lane, 3.0);
            e.mem.set_f32(HALF_VECTOR + 4 * lane, 0.5);
            e.mem.set_u32(LANE_BITS + 4 * lane, 1 << lane);
        }
        e.register_double(X87_SQRT, |_, a| {
            let argument = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            Ret {
                st0: argument.sqrt(),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e
    }

    fn sphere(e: &mut Engine, radius: f32) -> Ptr<HkpSphereShape> {
        let this = e.new_object::<HkpSphereShape>();
        e.set(this, HkpSphereShape::m_radius, radius);
        this
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, v);
        address
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

    #[test]
    fn there_is_one_collision_sphere() {
        let mut e = engine();
        let this = sphere(&mut e, 1.0);
        assert_eq!(e.call(0x00cd8050, &args![this]).u32(), 1);
    }

    #[test]
    fn the_supporting_vertex_is_zero() {
        let mut e = engine();
        let this = sphere(&mut e, 1.0);
        let direction = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let out = vector_at(&mut e, [9.0; 4]);
        e.call(0x00cd8060, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out), [0.0; 4]);
    }

    #[test]
    fn vertex_ids_convert_to_count_zero_vectors() {
        let mut e = engine();
        let this = sphere(&mut e, 1.0);
        let ids = e.mem.alloc(8);
        let out = e.mem.alloc(0x50);
        for index in 0..5u32 {
            store_vec(&mut e, out + 0x10 * index, [7.0; 4]);
        }
        e.call(0x00cd8080, &args![this, ids, 3i32, out]);
        for index in 0..3u32 {
            assert_eq!(load_vec(&e, out + 0x10 * index), [0.0; 4]);
        }
        assert_eq!(load_vec(&e, out + 0x30), [7.0; 4]);
        // Nothing is written for a count of zero or less.
        store_vec(&mut e, out, [7.0; 4]);
        e.call(0x00cd8080, &args![this, ids, 0i32, out]);
        e.call(0x00cd8080, &args![this, ids, -2i32, out]);
        assert_eq!(load_vec(&e, out), [7.0; 4]);
    }

    #[test]
    fn the_centre_is_zero() {
        let mut e = engine();
        let this = sphere(&mut e, 1.0);
        let out = vector_at(&mut e, [9.0; 4]);
        e.call(0x00cd80c0, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0; 4]);
    }

    #[test]
    fn the_first_vertex_is_zero() {
        let mut e = engine();
        let this = sphere(&mut e, 1.0);
        let out = vector_at(&mut e, [9.0; 4]);
        e.call(0x00cd80e0, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0; 4]);
    }

    #[test]
    fn the_box_is_the_translation_widened_by_radius_and_tolerance() {
        let mut e = engine();
        let this = sphere(&mut e, 0.5);
        let transform = e.mem.alloc(0x40);
        store_vec(&mut e, transform + 0x30, [1.0, 2.0, 3.0, 4.0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00cd8100, &args![this, transform, 0.25f32, out]);
        assert_eq!(load_vec(&e, out), [0.25, 1.25, 2.25, 3.25]);
        assert_eq!(load_vec(&e, out + 0x10), [1.75, 2.75, 3.75, 4.75]);
    }

    #[test]
    fn collision_sphere_answers_the_buffer() {
        let mut e = engine();
        let this = sphere(&mut e, 0.75);
        let out = e.mem.alloc(0x10);
        // EAX is `out` at the RET (00cd8158); the convex transform shape reads it (00d5df2f).
        assert_eq!(e.call(0x00cd_8140, &args![this, out]).u32(), out);
    }

    #[test]
    fn the_collision_sphere_carries_the_radius() {
        let mut e = engine();
        let this = sphere(&mut e, 0.75);
        let out = vector_at(&mut e, [9.0; 4]);
        e.call(0x00cd8140, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 0.75]);
    }

    struct Cast {
        hit: bool,
        output: u32,
    }

    /// Casts `from` -> `to` through `castRayUtil` at a sphere of `radius`
    /// with `hit_fraction` already in the output (shape key index 2).
    fn util_cast(e: &mut Engine, radius: f32, from: Vec4, to: Vec4, hit_fraction: f32) -> Cast {
        let rays = e.mem.alloc(0x20);
        store_vec(e, rays, from);
        store_vec(e, rays + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, hit_fraction);
        e.mem.set_u32(output + 0x40, 2);
        let result = e.mem.alloc(8);
        let returned = e
            .call(0x00cd85c0, &args![result, radius, rays, output])
            .u32();
        assert_eq!(returned, result);
        Cast {
            hit: e.mem.u8(result) != 0,
            output,
        }
    }

    #[test]
    fn the_util_hits_a_sphere_head_on() {
        let mut e = engine();
        let c = util_cast(
            &mut e,
            1.0,
            [0.0, 0.0, 5.0, 0.0],
            [0.0, 0.0, -5.0, 0.0],
            1.0,
        );
        assert!(c.hit);
        assert_near(
            [e.mem.f32(c.output + 0x10), 0.0, 0.0, 0.0],
            [0.4, 0.0, 0.0, 0.0],
            1e-6,
        );
        assert_near(load_vec(&e, c.output), [0.0, 0.0, 1.0, 0.0], 1e-5);
        assert_eq!(e.mem.u32(c.output + 0x20 + 8), 0xffff_ffff);
        // The sqrt helper got the discriminant as a double.
        assert_eq!(calls_to(&e, X87_SQRT).len(), 1);
    }

    #[test]
    fn the_util_starts_from_the_closest_point_for_long_rays() {
        let mut e = engine();
        let c = util_cast(
            &mut e,
            1.0,
            [0.0, 0.0, 100.0, 0.0],
            [0.0, 0.0, -100.0, 0.0],
            1.0,
        );
        assert!(c.hit);
        assert_near(
            [e.mem.f32(c.output + 0x10), 0.0, 0.0, 0.0],
            [0.495, 0.0, 0.0, 0.0],
            1e-5,
        );
        assert_near(load_vec(&e, c.output), [0.0, 0.0, 1.0, 0.0], 1e-4);
    }

    #[test]
    fn the_util_misses_in_every_way() {
        let mut e = engine();
        // Moving away from the centre.
        let c = util_cast(
            &mut e,
            1.0,
            [0.0, 0.0, 5.0, 0.0],
            [0.0, 0.0, 10.0, 0.0],
            1.0,
        );
        assert!(!c.hit);
        assert!(calls_to(&e, X87_SQRT).is_empty());
        // Passing beside the sphere (the discriminant is not positive).
        let c = util_cast(
            &mut e,
            1.0,
            [3.0, 0.0, 5.0, 0.0],
            [3.0, 0.0, -5.0, 0.0],
            1.0,
        );
        assert!(!c.hit);
        // Hitting, but not nearer than the hit already found.
        let c = util_cast(
            &mut e,
            1.0,
            [0.0, 0.0, 5.0, 0.0],
            [0.0, 0.0, -5.0, 0.0],
            0.3,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.3);
        assert_eq!(e.mem.u32(c.output + 0x28), 0);
        // A long ray that starts inside the sphere.
        let c = util_cast(
            &mut e,
            2.0,
            [0.0, 0.0, 100.0, 0.0],
            [0.0, 0.0, 99.0, 0.0],
            1.0,
        );
        assert!(!c.hit);
    }

    fn cast_ray(e: &mut Engine, from: Vec4, to: Vec4) -> (bool, u32, u32) {
        let this = sphere(e, 1.0);
        let stream = profile_stream(e);
        let input = e.mem.alloc(0x20);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, 1.0);
        let result = e.mem.alloc(8);
        let returned = e
            .call(0x00cd8190, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        (e.mem.u8(result) != 0, output, stream)
    }

    #[test]
    fn cast_ray_uses_the_util_and_records_a_profile() {
        let mut e = engine();
        let (hit, output, _) = cast_ray(&mut e, [0.0, 0.0, 5.0, 0.0], [0.0, 0.0, -5.0, 0.0]);
        assert!(hit);
        assert_near(
            [e.mem.f32(output + 0x10), 0.0, 0.0, 0.0],
            [0.4, 0.0, 0.0, 0.0],
            1e-6,
        );
        let (hit, _, _) = cast_ray(&mut e, [0.0, 0.0, 5.0, 0.0], [0.0, 0.0, 9.0, 0.0]);
        assert!(!hit);
    }

    #[test]
    fn cast_ray_bundle_casts_four_rays() {
        let mut e = engine();
        let this = sphere(&mut e, 1.0);
        let stream = profile_stream(&mut e);
        // Structure of arrays: from.x, from.y, from.z, to.x, to.y, to.z.
        let input = e.mem.alloc(0x60);
        let from_x = [0.0, 0.0, 0.5, 0.0];
        let from_z = [5.0, 5.0, 5.0, 5.0];
        let to_z = [-5.0, 10.0, -5.0, -5.0];
        store_vec(&mut e, input, from_x);
        store_vec(&mut e, input + 0x20, from_z);
        store_vec(&mut e, input + 0x30, from_x);
        store_vec(&mut e, input + 0x50, to_z);
        let outputs = e.mem.alloc(0x140);
        for ray in 0..4u32 {
            e.mem.set_f32(outputs + 0x50 * ray + 0x10, 1.0);
        }
        let in_mask = e.mem.alloc(0x10);
        e.mem.set_u32(in_mask, u32::MAX);
        e.mem.set_u32(in_mask + 4, u32::MAX);
        e.mem.set_u32(in_mask + 8, u32::MAX);
        let result = e.mem.alloc(0x10);
        let returned = e
            .call(0x00cd8220, &args![this, result, input, outputs, in_mask])
            .u32();
        assert_eq!(returned, result);
        assert_eq!(
            [
                e.mem.u32(result),
                e.mem.u32(result + 4),
                e.mem.u32(result + 8),
                e.mem.u32(result + 12)
            ],
            [u32::MAX, 0, u32::MAX, 0]
        );
        // Ray 0 hits the pole; ray 2 (x = 0.5) hits the sphere at z = sqrt(0.75).
        assert_near(
            [e.mem.f32(outputs + 0x10), 0.0, 0.0, 0.0],
            [0.4, 0.0, 0.0, 0.0],
            2e-3,
        );
        assert_near(load_vec(&e, outputs), [0.0, 0.0, 1.0, 0.0], 2e-3);
        let expected = (5.0 - 0.75f32.sqrt()) / 10.0;
        assert_near(
            [e.mem.f32(outputs + 0xa0 + 0x10), 0.0, 0.0, 0.0],
            [expected, 0.0, 0.0, 0.0],
            2e-3,
        );
        assert_near(
            load_vec(&e, outputs + 0xa0),
            [0.5, 0.0, 0.75f32.sqrt(), 0.0],
            2e-3,
        );
        assert_eq!(e.mem.u32(outputs + 0x20), 0xffff_ffff);
        // Rays that missed or were masked off keep their old output.
        assert_eq!(e.mem.f32(outputs + 0x50 + 0x10), 1.0);
        assert_eq!(e.mem.f32(outputs + 0xf0 + 0x10), 1.0);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY_BUNDLE);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
    }

    #[test]
    fn calc_content_statistics_passes_the_sphere_class() {
        let mut e = engine();
        let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let double_log = std::rc::Rc::clone(&log);
        e.register_double(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, move |_, a| {
            double_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00cd8040, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        assert_eq!(
            *log.borrow(),
            vec![vec![0x1000, 0x2000, SPHERE_SHAPE_CLASS]]
        );
    }

    #[test]
    fn the_constructor_sets_the_radius_and_the_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x1234);
        let got = e
            .call(0x00cd8160, &args![this, f32::from_bits(0x7f80_0001)])
            .u32();
        assert_eq!(got, this);
        // The radius went through the x87 stack: the signalling NaN is quiet now.
        assert_eq!(e.mem.u32(this + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this), SPHERE_SHAPE_VTABLE);
        e.call(0x00cd8160, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0x10), 2.5);
    }
}
