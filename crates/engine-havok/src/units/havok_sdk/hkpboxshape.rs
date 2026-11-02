//! `Havok SDK/hkpboxshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBoxShape` is a box: `m_radius` (+0x10) and the half extents
//! `m_halfExtents` (+0x20, an `hkVector4`). Its eight corners are the sign
//! pattern table at `010c7030` (eight `hkVector4` of +-1.0) multiplied by the
//! half extents. SSE code (`ADDPS`, `MULPS`, `CMPLEPS`) computes lane by lane
//! in `f32`; the x87 code in `castRayImpl` rounds at each `float` store, the
//! way its `FSTP` instructions do, and compares the way its `FNSTSW` /
//! `TEST AH` pairs read the flags (NaN included). The profiling records
//! `castRayImpl` writes use the host clock for `RDTSC`.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpBoxShape` (Xbox PDB), 0x30 bytes.
    pub struct HkpBoxShape: 0x30 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// `m_userData` (Xbox PDB).
        0x08 m_userData: u32,
        /// `m_type` (Xbox PDB).
        0x0C m_type: u32,
        /// `m_radius` (Xbox PDB).
        0x10 m_radius: f32,
        // +0x20 `m_halfExtents` (Xbox PDB): an `hkVector4`.
    }
}

/// `m_halfExtents` (Xbox PDB).
const HALF_EXTENTS: u32 = 0x20;
/// The eight corner sign vectors (`hkVector4` of +-1.0), 16 bytes each.
const CORNER_SIGNS: u32 = 0x010c_7030;
/// Four sign-bit masks (`0x80000000`): `ANDPS` keeps the sign, `XORPS` flips it.
const SIGN_MASK: u32 = 0x010c_44d0;
/// `-1.0` (`float`): the sign of the second pass of `castRayImpl`.
const MINUS_ONE: u32 = 0x0101_2054;
/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcBox"` and `"Et"`.
const PROFILE_BEGIN_BOX: u32 = 0x010c_70fc;
const PROFILE_END: u32 = 0x010c_41f8;
/// The helper `getAabbImpl` hands the transform, the half extents, the grown
/// radius and the output to (`00c9e120`, cdecl, four words).
const CALC_AABB: u32 = 0x00c9_e120;

// `hkpShapeRayCastOutput` (Xbox PDB): `m_normal` +0x00, `m_hitFraction`
// +0x10, `m_shapeKeys` +0x20, `m_shapeKeyIndex` +0x40.
const OUT_NORMAL: u32 = 0x00;
const OUT_HIT_FRACTION: u32 = 0x10;
const OUT_SHAPE_KEYS: u32 = 0x20;
const OUT_SHAPE_KEY_INDEX: u32 = 0x40;
// `hkpShapeRayCastInput` (Xbox PDB): `m_from` +0x00, `m_to` +0x10.
const RAY_FROM: u32 = 0x00;
const RAY_TO: u32 = 0x10;

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

/// `MOVMSKPS` of a per-lane comparison: bit `lane` is set when `test` holds.
fn move_mask(a: Vec4, b: Vec4, test: impl Fn(f32, f32) -> bool) -> u32 {
    (0..4).fold(0, |mask, lane| {
        mask | (u32::from(test(a[lane], b[lane])) << lane)
    })
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

// Translated from 00c9db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::getNumCollisionSpheresImpl` (Xbox PDB): 8, the corners.
pub fn hkp_box_shape_get_num_collision_spheres_impl(
    _e: &mut Engine,
    _this: Ptr<HkpBoxShape>,
) -> u32 {
    8
}

// Translated from 00c9dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::getFirstVertex` (Xbox PDB): copies the half extents (the
/// corner with all signs positive) to `out`.
pub fn hkp_box_shape_get_first_vertex(e: &mut Engine, this: Ptr<HkpBoxShape>, out: Ptr) {
    copy_vec(e, this.addr() + HALF_EXTENTS, out.addr());
}

// Translated from 00c9dc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::getAabbImpl` (Xbox PDB): hands `transform`, the half
/// extents, `m_radius + tolerance` (rounded to a `float`) and `out` to the
/// box bound helper `00c9e120`.
pub fn hkp_box_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpBoxShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let radius = e.get(this, HkpBoxShape::m_radius);
    // `FLD radius; FADD tolerance; FSTP; FLD; FSTP` into the argument slot.
    let grown = x87_quiet((radius as f64 + tolerance as f64) as f32);
    e.call(
        CALC_AABB,
        &args![transform, this.addr() + HALF_EXTENTS, grown, out],
    );
}

// Translated from 00c9dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::getSupportingVertexImpl` (Xbox PDB): writes the corner of
/// the box furthest along `direction` into `out`: each half extent takes the
/// sign bit of the direction's lane (`ANDPS`, `XORPS`). The `w` lane holds the
/// vertex id as a float with the bits `0x3f000000 | id`, the id being the
/// three-bit mask of the lanes that came out negative (`CMPLTPS`, so NaN does
/// not count).
pub fn hkp_box_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr<HkpBoxShape>,
    direction: Ptr,
    out: Ptr,
) {
    let mut corner = [0u32; 4];
    for (lane, word) in corner.iter_mut().enumerate() {
        let lane = lane as u32;
        let mask = e.mem.u32(SIGN_MASK + 4 * lane);
        let sign = e.mem.u32(direction.addr() + 4 * lane) & mask;
        *word = sign ^ e.mem.u32(this.addr() + HALF_EXTENTS + 4 * lane);
    }
    let lanes = corner.map(f32::from_bits);
    let negative = move_mask(lanes, [0.0; 4], |a, b| a < b) & 7;
    for (lane, word) in corner.iter().take(3).enumerate() {
        e.mem.set_u32(out.addr() + 4 * lane as u32, *word);
    }
    e.mem.set_u32(out.addr() + 0xc, negative | 0x3f00_0000);
}

// Translated from 00c9dcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::getCentreImpl` (Xbox PDB): the centre is the origin; writes
/// four zero lanes to `out`.
pub fn hkp_box_shape_get_centre_impl(e: &mut Engine, _this: Ptr<HkpBoxShape>, out: Ptr) {
    store_vec(e, out.addr(), [0.0; 4]);
}

// Translated from 00c9dcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::convertVertexIdsToVerticesImpl` (Xbox PDB): for each of
/// `count` 16-bit vertex ids, writes the sign vector at `010c7030 + 16 * id`
/// times the half extents to `out`, then replaces the `w` lane by the bits
/// `0x3f000000 | id`.
pub fn hkp_box_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    this: Ptr<HkpBoxShape>,
    ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let mut id_address = ids.addr();
    let mut out_address = out.addr();
    // `ADD EDI,-1; JS`, then `SUB EDI,1; JNS`: `count` passes for a positive count.
    let mut remaining = count.wrapping_sub(1);
    while remaining >= 0 {
        let id = u32::from(e.mem.u16(id_address));
        let half = load_vec(e, this.addr() + HALF_EXTENTS);
        let signs = load_vec(e, CORNER_SIGNS.wrapping_add(id << 4));
        let corner = std::array::from_fn(|lane| signs[lane] * half[lane]);
        store_vec(e, out_address, corner);
        e.mem.set_u32(out_address + 0xc, id | 0x3f00_0000);
        out_address = out_address.wrapping_add(0x10);
        id_address = id_address.wrapping_add(2);
        remaining = remaining.wrapping_sub(1);
    }
}

// Translated from 00c9dd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::getCollisionSpheresImpl` (Xbox PDB): writes the eight
/// corner spheres to `out`: the sign vectors times `(half extent x, y, z,
/// m_radius)`. Answers `out` (the `const hkSphere*` of the Xbox PDB signature; `EAX` is `out`
/// at the RET 00c9dda4): hkpConvexTransformShape::getCollisionSpheresImpl reads it through
/// its child shape's slot +0x30 (`MOV EDI,EAX` at 00d5df2f).
pub fn hkp_box_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpBoxShape>,
    out: Ptr,
) -> Ptr {
    let half = load_vec(e, this.addr() + HALF_EXTENTS);
    let radius = x87_quiet(e.get(this, HkpBoxShape::m_radius));
    // The `w` lane of the multiplier is the radius (it replaces the stored `w`).
    let scale = [half[0], half[1], half[2], radius];
    for corner in 0..8u32 {
        let signs = load_vec(e, CORNER_SIGNS + 0x10 * corner);
        let sphere = std::array::from_fn(|lane| signs[lane] * scale[lane]);
        store_vec(e, out.addr() + 0x10 * corner, sphere);
    }
    out
}

// Translated from 00c9de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::castRayImpl` (Xbox PDB): casts the ray `input`
/// (`hkpShapeRayCastInput`: `m_from`, `m_to`) against the box grown by
/// `m_radius` and updates `output` (`hkpShapeRayCastOutput`) when it hits
/// closer than `output.m_hitFraction`. Writes the hit flag (`hkBool`) to
/// `result` and returns `result`; a profiling record `"TtrcBox"` is written
/// first and `"Et"` last.
///
/// The ray misses when both ends lie beyond the same face, or when the start
/// is inside the box (it lies beyond no face). Otherwise two passes over the
/// three axes (z, y, x) clip the ray's fraction interval: the first against
/// the positive faces with sign +1.0, the second against the negative faces
/// with sign -1.0. A face the start lies beyond raises the entry fraction and
/// stores that axis' sign as the hit normal; any other face lowers the exit
/// fraction. The ray misses as soon as the exit fraction falls below the
/// entry fraction. On a hit the normal and the entry fraction are written
/// and the shape key at `m_shapeKeyIndex` is set to -1. When no face ever
/// raised the entry fraction (a start beyond a face that gives a NaN
/// fraction) the exe stores its 16-byte stack normal (`ESP+0x30`, loaded at
/// `00c9e009` and stored to `output + 0` at `00c9e016`) without ever having
/// written it, so that output normal is stack garbage; zeros stand in.
// Translation incomplete: 00c9de40 - reads uninitialised stack word at ESP+0x30 (the 16-byte hit normal stored to output+0 when no face raised the entry fraction)
pub fn hkp_box_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpBoxShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_BOX);
    let hit = cast_ray_against_box(e, this, input, output);
    profile_record(e, PROFILE_END);
    e.mem.set_u8(result.addr(), u8::from(hit));
    result
}

/// The body of `castRayImpl`: whether the ray hits.
fn cast_ray_against_box(e: &mut Engine, this: Ptr<HkpBoxShape>, input: Ptr, output: Ptr) -> bool {
    let radius = e.get(this, HkpBoxShape::m_radius);
    let half = load_vec(e, this.addr() + HALF_EXTENTS);
    // `ADDPS`: the box grown by the radius in every lane; its negation by `XORPS`.
    let reach: Vec4 = half.map(|extent| radius + extent);
    let negated: Vec4 = std::array::from_fn(|lane| {
        let mask = e.mem.u32(SIGN_MASK + 4 * lane as u32);
        f32::from_bits(reach[lane].to_bits() ^ mask)
    });
    let from = load_vec(e, input.addr() + RAY_FROM);
    let to = load_vec(e, input.addr() + RAY_TO);
    // Lanes where the end lies at or beyond the positive / negative face.
    let from_high = move_mask(reach, from, |a, b| a <= b) & 7;
    let from_low = move_mask(from, negated, |a, b| a <= b) & 7;
    let to_high = move_mask(reach, to, |a, b| a <= b) & 7;
    let to_low = move_mask(to, negated, |a, b| a <= b) & 7;
    if (from_high & to_high) != 0 || (from_low & to_low) != 0 || (from_low | from_high) == 0 {
        return false;
    }

    let mut entry = 0.0f32;
    let mut exit = x87_quiet(e.mem.f32(output.addr() + OUT_HIT_FRACTION));
    let mut normal = [0.0f32; 4];
    let minus_one = x87_quiet(e.global::<f32>(MINUS_ONE));
    let mut sign = 1.0f32;
    // The faces to clip against and the faces the start lies beyond.
    let mut faces = to_high | from_high;
    let mut start_beyond = from_high;
    for pass in 0..2 {
        for axis in (0..3usize).rev() {
            let bit = 1u32 << axis;
            if faces & bit == 0 {
                continue;
            }
            // `FLD [negated lane]; FSTP`: the negated reach, through a float slot.
            let face = x87_quiet(negated[axis]);
            let from_lane = from[axis] as f64;
            let to_lane = to[axis] as f64;
            // x87 sums stay unrounded until the `FSTP float`.
            let start_distance = (from_lane * sign as f64 + face as f64) as f32;
            let end_distance = (face as f64 + to_lane * sign as f64) as f32;
            let fraction =
                (start_distance as f64 / (start_distance as f64 - end_distance as f64)) as f32;
            if start_beyond & bit != 0 {
                // `FCOM`: skipped when the fraction is below the entry or unordered.
                if fraction >= entry {
                    entry = fraction;
                    normal = [0.0; 4];
                    normal[axis] = sign;
                }
            } else {
                // Skipped only when the fraction is above the exit (ordered).
                let above_exit = fraction > exit;
                if !above_exit {
                    exit = fraction;
                }
            }
            if exit < entry {
                return false;
            }
        }
        if pass == 0 {
            faces = to_low | from_low;
            start_beyond = from_low;
            sign = minus_one;
        }
    }

    let key_index = e.mem.u32(output.addr() + OUT_SHAPE_KEY_INDEX);
    e.mem.set_f32(output.addr() + OUT_HIT_FRACTION, entry);
    store_vec(e, output.addr() + OUT_NORMAL, normal);
    e.mem.set_u32(
        output
            .addr()
            .wrapping_add(key_index.wrapping_mul(4))
            .wrapping_add(OUT_SHAPE_KEYS),
        0xffff_ffff,
    );
    true
}

/// `hkReferencedObject::calcContentStatistics` (`this` in `ECX`, collector, class) and the
/// `hkClass` of the box shape that `hkpBoxShape::calcContentStatistics` passes to it.
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
const BOX_SHAPE_CLASS: u32 = 0x0126_8940;
/// The vtable `00c9db80` stores, and the vtable of `hkpBoxShape` the constructor stores.
const BASE_VTABLE: u32 = 0x0102_e388;
const BOX_SHAPE_VTABLE: u32 = 0x010c_70b4;
/// The shape type the constructor stores in `m_type`.
const BOX_SHAPE_TYPE: u32 = 4;

// Translated from 00c9db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::calcContentStatistics` (engine map): `this` in `ECX`, the collector and one
/// more word on the stack; jumps to `hkReferencedObject::calcContentStatistics` with the
/// second word replaced by the class at `01268940`.
pub fn hkp_box_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpBoxShape>,
    collector: u32,
    _unused_1: u32,
) {
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, BOX_SHAPE_CLASS],
    );
}

// Translated from 00c9db80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable at `0102e388` (a scalar deleting destructor table, class unconfirmed)
/// into `this` (`ECX`, `__fastcall`).
pub fn fn_00c9db80(e: &mut Engine, this: Ptr<HkpBoxShape>) {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

/// Stores the smallest of the three half extents in `+0x2c`, the way the x87 code does:
/// `y` unless `y > x`, then that unless `z < it`, each loaded value quieted.
fn store_smallest_half_extent(e: &mut Engine, shape: u32) {
    let x = e.mem.f32(shape + HALF_EXTENTS);
    let y = e.mem.f32(shape + HALF_EXTENTS + 4);
    let z = e.mem.f32(shape + HALF_EXTENTS + 8);
    let first = x87_quiet(if y > x { x } else { y });
    let smallest = if z < first { z } else { first };
    e.mem.set_u32(
        shape + HALF_EXTENTS + 0xc,
        x87_quiet_bits(smallest.to_bits()),
    );
}

// Translated from 00c9db90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the vector at `half_extents` (16 bytes) to `+0x20` and stores the smallest of its
/// first three lanes in `+0x2c` (`this` in `ECX`; no engine-map name, the box shape's
/// half-extents setter by its effect).
pub fn fn_00c9db90(e: &mut Engine, this: Ptr<HkpBoxShape>, half_extents: Ptr) {
    copy_vec(e, half_extents.addr(), this.addr() + HALF_EXTENTS);
    store_smallest_half_extent(e, this.addr());
}

// Translated from 00c9ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxShape::hkpBoxShape` (engine map): `this` in `ECX`, the half extents vector and the
/// radius (a `float`, loaded and stored on the x87 stack, so a signalling NaN is quieted) on
/// the stack. Sets the reference count word `+6` to 1, the radius, user data 0, type 4, the
/// vtable `010c70b4`, the half extents and their smallest in `+0x2c`. Returns `this`.
pub fn hkp_box_shape_construct(
    e: &mut Engine,
    this: Ptr<HkpBoxShape>,
    half_extents: Ptr,
    radius: f32,
) -> Ptr<HkpBoxShape> {
    let shape = this.addr();
    e.mem.set_u16(shape + 6, 1);
    e.mem
        .set_u32(shape + 0x10, x87_quiet_bits(radius.to_bits()));
    e.mem.set_u32(shape + 8, 0);
    e.mem.set_u32(shape + 0xc, BOX_SHAPE_TYPE);
    e.mem.set_u32(shape, BOX_SHAPE_VTABLE);
    copy_vec(e, half_extents.addr(), shape + HALF_EXTENTS);
    store_smallest_half_extent(e, shape);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c9db70,
            hkp_box_shape_get_num_collision_spheres_impl(Ptr<HkpBoxShape>) -> u32
        ),
        entry!(
            0x00c9dbf0,
            hkp_box_shape_get_first_vertex(Ptr<HkpBoxShape>, Ptr)
        ),
        entry!(
            0x00c9dc10,
            hkp_box_shape_get_aabb_impl(Ptr<HkpBoxShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00c9dc40,
            hkp_box_shape_get_supporting_vertex_impl(Ptr<HkpBoxShape>, Ptr, Ptr)
        ),
        entry!(
            0x00c9dcb0,
            hkp_box_shape_get_centre_impl(Ptr<HkpBoxShape>, Ptr)
        ),
        entry!(
            0x00c9dcd0,
            hkp_box_shape_convert_vertex_ids_to_vertices_impl(Ptr<HkpBoxShape>, Ptr, i32, Ptr)
        ),
        entry!(
            0x00c9dd30,
            hkp_box_shape_get_collision_spheres_impl(Ptr<HkpBoxShape>, Ptr) -> Ptr
        ),
        entry!(
            0x00c9de40,
            hkp_box_shape_cast_ray_impl(Ptr<HkpBoxShape>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00c9db60,
            hkp_box_shape_calc_content_statistics(Ptr<HkpBoxShape>, u32, u32)
        ),
        entry!(0x00c9db80, fn_00c9db80(Ptr<HkpBoxShape>)),
        entry!(0x00c9db90, fn_00c9db90(Ptr<HkpBoxShape>, Ptr)),
        entry!(
            0x00c9ddb0,
            hkp_box_shape_construct(Ptr<HkpBoxShape>, Ptr, f32) -> Ptr<HkpBoxShape>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIGNS: [[f32; 4]; 8] = [
        [1.0, 1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0, 1.0],
        [1.0, -1.0, 1.0, 1.0],
        [-1.0, -1.0, 1.0, 1.0],
        [1.0, 1.0, -1.0, 1.0],
        [-1.0, 1.0, -1.0, 1.0],
        [1.0, -1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0, 1.0],
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        for (i, signs) in SIGNS.iter().enumerate() {
            store_vec(&mut e, CORNER_SIGNS + 0x10 * i as u32, *signs);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.set_global(MINUS_ONE, -1.0f32);
        e
    }

    /// A box with half extents (1, 2, 3) and radius 0.5.
    fn the_box(e: &mut Engine) -> Ptr<HkpBoxShape> {
        let this = e.new_object::<HkpBoxShape>();
        store_vec(e, this.addr() + HALF_EXTENTS, [1.0, 2.0, 3.0, 4.0]);
        e.set(this, HkpBoxShape::m_radius, 0.5);
        this
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> Ptr {
        let address = e.mem.alloc(16);
        store_vec(e, address, v);
        Ptr::new(address)
    }

    #[test]
    fn num_collision_spheres_is_eight() {
        let mut e = engine();
        let this = the_box(&mut e);
        assert_eq!(e.call(0x00c9_db70, &args![this]).u32(), 8);
    }

    #[test]
    fn first_vertex_copies_the_half_extents() {
        let mut e = engine();
        let this = the_box(&mut e);
        let out = e.mem.alloc(16);
        e.call(0x00c9_dbf0, &args![this, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn aabb_passes_the_grown_radius_to_the_helper() {
        let mut e = engine();
        let this = the_box(&mut e);
        e.register(CALC_AABB, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c9_dc10, &args![this, 0x1111u32, 0.25f32, 0x2222u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == CALC_AABB).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0].1,
            vec![0x1111, this.addr() + 0x20, 0.75f32.to_bits(), 0x2222]
        );
    }

    #[test]
    fn supporting_vertex_takes_the_sign_of_each_direction_lane() {
        let mut e = engine();
        let this = the_box(&mut e);
        let out = e.mem.alloc(16);
        // Negative x and z, positive y: lanes 0 and 2 negative -> id 5.
        let direction = vector_at(&mut e, [-1.0, 1.0, -1.0, 0.0]);
        e.call(0x00c9_dc40, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [-1.0, 2.0, -3.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0005);
        // -0.0 has its sign bit set, so it picks the negative corner.
        let direction = vector_at(&mut e, [-0.0, 0.0, 0.0, 0.0]);
        e.call(0x00c9_dc40, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [-1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0001);
    }

    #[test]
    fn centre_is_the_origin() {
        let mut e = engine();
        let this = the_box(&mut e);
        let out = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00c9_dcb0, &args![this, out]);
        assert_eq!(load_vec(&e, out.addr()), [0.0; 4]);
    }

    #[test]
    fn vertex_ids_select_corners_and_tag_the_w_lane() {
        let mut e = engine();
        let this = the_box(&mut e);
        let ids = e.mem.alloc(8);
        e.mem.set_u16(ids, 5);
        e.mem.set_u16(ids + 2, 0);
        let out = e.mem.alloc(32);
        e.call(0x00c9_dcd0, &args![this, ids, 2i32, out]);
        assert_eq!(load_vec(&e, out)[..3], [-1.0, 2.0, -3.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0005);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f00_0000);
        // A count of zero (or less) converts nothing.
        let untouched = e.mem.alloc(16);
        e.call(0x00c9_dcd0, &args![this, ids, 0i32, untouched]);
        e.call(0x00c9_dcd0, &args![this, ids, -3i32, untouched]);
        assert_eq!(load_vec(&e, untouched), [0.0; 4]);
    }

    #[test]
    fn collision_spheres_are_the_corners_with_the_radius_in_w() {
        let mut e = engine();
        let this = the_box(&mut e);
        let out = e.mem.alloc(0x80);
        e.call(0x00c9_dd30, &args![this, out]);
        for (corner, signs) in SIGNS.iter().enumerate() {
            let sphere = load_vec(&e, out + 0x10 * corner as u32);
            assert_eq!(
                sphere,
                [signs[0], signs[1] * 2.0, signs[2] * 3.0, 0.5],
                "corner {corner}"
            );
        }
    }

    #[test]
    fn collision_spheres_answer_the_buffer() {
        let mut e = engine();
        let this = the_box(&mut e);
        let out = e.mem.alloc(0x80);
        // EAX is `out` at the RET (00c9dda4); 00d5df2f reads it.
        assert_eq!(e.call(0x00c9_dd30, &args![this, out]).u32(), out);
    }

    /// A profile stream with room for two records; the TLS lookup returns it.
    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 3);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    struct Cast {
        hit: bool,
        output: u32,
        stream: u32,
    }

    /// Casts `from` -> `to` at the box (reach 1.5, 2.5, 3.5) with `hit_fraction`
    /// already in the output.
    fn cast(e: &mut Engine, from: Vec4, to: Vec4, hit_fraction: f32) -> Cast {
        let this = the_box(e);
        let stream = profile_stream(e);
        let input = e.mem.alloc(0x30);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, hit_fraction);
        e.mem.set_u32(output + 0x40, 3);
        let result = e.mem.alloc(8);
        let returned = e
            .call(0x00c9_de40, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        Cast {
            hit: e.mem.u8(result) != 0,
            output,
            stream,
        }
    }

    fn assert_profile_records(e: &Engine, stream: u32) {
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_BOX);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
    }

    #[test]
    fn cast_ray_hits_the_near_face_and_reports_its_normal() {
        let mut e = engine();
        // From x = -5 along +x to x = 5: the face at x = -1.5 is 3.5 away of 10.
        let c = cast(&mut e, [-5.0, 0.0, 0.0, 0.0], [5.0, 0.0, 0.0, 0.0], 1.0);
        assert!(c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.35);
        // The normal is the sign of the pass that found the entry: -1 on x.
        assert_eq!(load_vec(&e, c.output), [-1.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(c.output + 0x20 + 12), 0xffff_ffff);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_from_the_high_side_has_a_positive_normal() {
        let mut e = engine();
        let c = cast(&mut e, [0.0, 5.0, 0.0, 0.0], [0.0, -5.0, 0.0, 0.0], 1.0);
        assert!(c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.25);
        assert_eq!(load_vec(&e, c.output), [0.0, 1.0, 0.0, 0.0]);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_misses_when_both_ends_are_beyond_one_face() {
        let mut e = engine();
        let c = cast(&mut e, [-5.0, 0.0, 0.0, 0.0], [-4.0, 0.0, 0.0, 0.0], 1.0);
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 1.0);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_misses_when_it_starts_inside() {
        let mut e = engine();
        let c = cast(&mut e, [0.0, 0.0, 0.0, 0.0], [9.0, 0.0, 0.0, 0.0], 1.0);
        assert!(!c.hit);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_misses_a_box_it_passes_beside() {
        let mut e = engine();
        // Enters the x slab at 0.35 but leaves the y slab (|y| <= 2.5) at 0.05.
        let c = cast(&mut e, [-5.0, 2.0, 0.0, 0.0], [5.0, 12.0, 0.0, 0.0], 1.0);
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 1.0);
    }

    #[test]
    fn cast_ray_misses_when_the_output_already_holds_a_nearer_hit() {
        let mut e = engine();
        // The hit would be at 0.35 but the output already holds 0.1.
        let c = cast(&mut e, [-5.0, 0.0, 0.0, 0.0], [5.0, 0.0, 0.0, 0.0], 0.1);
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.1);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn extents(e: &mut Engine, v: [u32; 4]) -> u32 {
        let address = e.mem.alloc(0x10);
        for (lane, word) in v.into_iter().enumerate() {
            e.mem.set_u32(address + 4 * lane as u32, word);
        }
        address
    }

    fn bits(values: [f32; 4]) -> [u32; 4] {
        values.map(f32::to_bits)
    }

    #[test]
    fn calc_content_statistics_passes_the_box_class() {
        let mut e = engine();
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.new_object::<HkpBoxShape>();
        e.call(0x00c9_db60, &args![this, 0x1111u32, 0x2222u32]);
        assert_eq!(*log.borrow(), vec![vec![this.addr(), 0x1111, 0x0126_8940]]);
    }

    #[test]
    fn base_vtable_store_changes_only_the_vtable() {
        let mut e = engine();
        let this = e.new_object::<HkpBoxShape>();
        e.mem.set_u32(this.addr() + 8, 0x1234_5678);
        e.call(0x00c9_db80, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x0102_e388);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x1234_5678);
    }

    #[test]
    fn half_extent_setter_stores_the_vector_and_its_smallest_lane() {
        let mut e = engine();
        let this = e.new_object::<HkpBoxShape>();
        for (v, smallest) in [
            ([3.0, 2.0, 5.0, 9.0], 2.0),
            ([1.0, 2.0, 5.0, 9.0], 1.0),
            ([4.0, 6.0, 2.5, 9.0], 2.5),
            ([4.0, 4.0, 4.0, 0.5], 4.0),
        ] {
            let half = extents(&mut e, bits(v));
            e.call(0x00c9_db90, &args![this, half]);
            assert_eq!(load_vec(&e, this.addr() + 0x20)[..3], v[..3]);
            assert_eq!(e.mem.f32(this.addr() + 0x2c), smallest);
        }
    }

    #[test]
    fn half_extent_setter_follows_the_flag_tests_for_nan() {
        let mut e = engine();
        let this = e.new_object::<HkpBoxShape>();
        let nan = 0x7f80_0001;
        let nan_bits = 0x7fc0_0001;
        // y is a NaN: the unordered compare takes y; z is then not below it.
        let half = extents(&mut e, [1.0f32.to_bits(), nan, 0.5f32.to_bits(), 0]);
        e.call(0x00c9_db90, &args![this, half]);
        assert_eq!(e.mem.u32(this.addr() + 0x2c), nan_bits);
        // x is a NaN: y > x is false, so y wins and z below it takes over.
        let half = extents(&mut e, [nan, 3.0f32.to_bits(), 2.0f32.to_bits(), 0]);
        e.call(0x00c9_db90, &args![this, half]);
        assert_eq!(e.mem.f32(this.addr() + 0x2c), 2.0);
        // z is a NaN: never below, so the first result stays.
        let half = extents(&mut e, [5.0f32.to_bits(), 3.0f32.to_bits(), nan, 0]);
        e.call(0x00c9_db90, &args![this, half]);
        assert_eq!(e.mem.f32(this.addr() + 0x2c), 3.0);
        // The copy itself keeps the signalling NaN bits.
        assert_eq!(e.mem.u32(this.addr() + 0x28), nan);
    }

    #[test]
    fn construct_fills_the_box_shape() {
        let mut e = engine();
        let this = e.new_object::<HkpBoxShape>();
        let half = extents(&mut e, bits([3.0, 1.5, 2.0, 7.0]));
        let got = e.call(0x00c9_ddb0, &args![this, half, 0.25f32]).u32();
        assert_eq!(got, this.addr());
        let shape = this.addr();
        assert_eq!(e.mem.u16(shape + 6), 1);
        assert_eq!(e.mem.f32(shape + 0x10), 0.25);
        assert_eq!(e.mem.u32(shape + 8), 0);
        assert_eq!(e.mem.u32(shape + 0xc), 4);
        assert_eq!(e.mem.u32(shape), 0x010c_70b4);
        assert_eq!(load_vec(&e, shape + 0x20)[..3], [3.0, 1.5, 2.0]);
        assert_eq!(e.mem.f32(shape + 0x2c), 1.5);
    }

    #[test]
    fn construct_quiets_a_signalling_nan_radius() {
        let mut e = engine();
        let this = e.new_object::<HkpBoxShape>();
        let half = extents(&mut e, bits([1.0, 1.0, 1.0, 1.0]));
        e.call(0x00c9_ddb0, &args![this, half, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this.addr() + 0x10), 0x7fc0_0001);
    }
}
