//! `Havok SDK/hkpconvexverticesshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpConvexVerticesShape` (Xbox PDB, 0x70 bytes) keeps its vertices as `m_rotatedVertices`
//! (+0x40, an `hkArray` of `FourVectors`: 0x30 bytes holding the x, y and z of four vertices as
//! three four-lane vectors at +0, +0x10 and +0x20), `m_numVertices` (+0x4c), the
//! `m_convexPiece` (+0x50, a `hkpCompressedMeshShape::ConvexPiece`) and `m_planeEquations`
//! (+0x54, an `hkArray` of `hkVector4`). Vertex `i` is lane `i & 3` of the `FourVectors` number
//! `i >> 2`. Copies that go through x87 (`FLD`/`FSTP`) quiet signalling NaNs
//! ([`x87_quiet`]); the SSE ones are bit for bit. x87 code compares the way its `FNSTSW`/`TEST
//! AH` pairs read the flags (NaN included). `RSQRTSS` is the host CPU's estimate
//! (`rsqrt_estimate`), as elsewhere in the crate. The profiling records `castRayImpl` writes use
//! the host clock for `RDTSC`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `m_radius` (+0x10, Xbox PDB), `m_aabbCenter` (+0x30), `m_rotatedVertices` (+0x40: data +0x40,
/// size +0x44), `m_numVertices` (+0x4c), `m_convexPiece` (+0x50) and `m_planeEquations` (+0x54:
/// data +0x54, size +0x58, capacity word +0x5c).
const RADIUS: u32 = 0x10;
const AABB_CENTRE: u32 = 0x30;
const ROTATED_VERTICES: u32 = 0x40;
const ROTATED_VERTICES_SIZE: u32 = 0x44;
const NUM_VERTICES: u32 = 0x4c;
const CONVEX_PIECE: u32 = 0x50;
const PLANE_EQUATIONS: u32 = 0x54;
/// `hkpCompressedMeshShape::ConvexPiece` (Xbox PDB): `m_faceVertices` data at +0x1c and
/// `m_faceOffsets` data and size at +0x28 and +0x2c.
const PIECE_FACE_VERTICES: u32 = 0x1c;
const PIECE_FACE_OFFSETS: u32 = 0x28;
const PIECE_FACE_OFFSETS_SIZE: u32 = 0x2c;
/// The size of `hkpConvexVerticesShape::FourVectors`.
const FOUR_VECTORS: u32 = 0x30;

/// The memory router lookup (`00c85750`, no arguments); its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `_ftol2_sse` (`00ec62c0`): the float in ST0 truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// The call `getAabbImpl` forwards to (`00d26230`, cdecl `(transform, a, b, margin, out)`).
const AABB_OF_SEGMENT: u32 = 0x00d2_6230;
/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcConvexVert"` and `"Et"`.
const PROFILE_BEGIN_CONVEX: u32 = 0x010d_0fb0;
const PROFILE_END: u32 = 0x010c_41f8;

/// `float` constants: 0.5, 0.0, 3.0 (the `RSQRTSS` refinement), -1.0 and the vector constant of
/// four 0.0.
const HALF: u32 = 0x0101_6248;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const MINUS_ONE: u32 = 0x0101_2054;
const ZEROS: u32 = 0x010c_7160;
/// `getSupportingVertexImpl`: the function-local static of four copies of the float at
/// `0104e148` (its guard byte is `01268fb0`, the copies `01268fa0`), the vector `(0.5, 1.5, 2.5,
/// 3.5)` the lane numbers start from and the vector of four 4.0 they advance by.
const BEST_INITIAL_SOURCE: u32 = 0x0104_e148;
const BEST_INITIAL: u32 = 0x0126_8fa0;
const BEST_INITIAL_GUARD: u32 = 0x0126_8fb0;
const LANE_NUMBERS: u32 = 0x010d_9fe0;
const LANE_STEP: u32 = 0x010d_0fa0;

// `hkpShapeRayCastInput` (Xbox PDB): `m_from` +0x00, `m_to` +0x10.
const RAY_FROM: u32 = 0x00;
const RAY_TO: u32 = 0x10;
// `hkpShapeRayCastOutput` (Xbox PDB): `m_normal` +0x00, `m_hitFraction` +0x10, `m_shapeKeys`
// +0x20, `m_shapeKeyIndex` +0x40.
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

/// The address of the `FourVectors` holding vertex `index` and the lane inside it.
fn vertex_slot(e: &Engine, this: u32, index: u32) -> (u32, u32) {
    let data = e.mem.u32(this + ROTATED_VERTICES);
    (
        data.wrapping_add((index >> 2).wrapping_mul(FOUR_VECTORS)),
        index & 3,
    )
}

/// Vertex `index` as `x87` copies read it: `x`, `y` and `z` through `FLD`/`FSTP`.
fn vertex_through_x87(e: &Engine, this: u32, index: u32) -> [f32; 3] {
    let (block, lane) = vertex_slot(e, this, index);
    std::array::from_fn(|axis| x87_quiet(e.mem.f32(block + 0x10 * axis as u32 + 4 * lane)))
}

// Translated from 00d58d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getNumCollisionSpheresImpl` (Xbox PDB): `m_numVertices`.
pub fn hkp_convex_vertices_shape_get_num_collision_spheres_impl(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + NUM_VERTICES)
}

// Translated from 00d58d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getCentreImpl` (Xbox PDB): copies `m_aabbCenter`.
pub fn hkp_convex_vertices_shape_get_centre_impl(e: &mut Engine, this: Ptr, out: Ptr) {
    copy_vec(e, this.addr() + AABB_CENTRE, out.addr());
}

// Translated from 00d58d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getFirstVertex` (Xbox PDB): the first vertex (lane 0 of the first
/// `FourVectors`) with `w` = 0.0, copied through x87.
pub fn hkp_convex_vertices_shape_get_first_vertex(e: &mut Engine, this: Ptr, out: Ptr) {
    let vertex = vertex_through_x87(e, this.addr(), 0);
    store_vec(e, out.addr(), [vertex[0], vertex[1], vertex[2], 0.0]);
}

// Translated from 00d58d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getCollisionSpheresImpl` (Xbox PDB): writes one sphere per vertex to
/// `out`: the vertex's x, y and z with `m_radius` in `w`, all copied bit for bit. Nothing for a
/// count of zero or less.
/// Answers `out` (the `const hkSphere*` of the Xbox PDB signature; `EAX` is `out`,
/// `MOV EAX,EBX` at 00d58ea7, at the RET 00d58ed1): hkpConvexTransformShape::
/// getCollisionSpheresImpl reads it through its child shape's slot +0x30 (`MOV EDI,EAX` at
/// 00d5df2f).
pub fn hkp_convex_vertices_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
) -> Ptr {
    let me = this.addr();
    let count = e.mem.i32(me + NUM_VERTICES);
    let radius_bits = e.mem.u32(me + RADIUS);
    let mut sphere = out.addr();
    for index in 0..count.max(0) as u32 {
        let (block, lane) = vertex_slot(e, me, index);
        for axis in 0..3 {
            let word = e.mem.u32(block + 0x10 * axis + 4 * lane);
            e.mem.set_u32(sphere + 4 * axis, word);
        }
        e.mem.set_u32(sphere + 0xc, radius_bits);
        sphere = sphere.wrapping_add(0x10);
    }
    out
}

// Translated from 00d58ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getSupportingVertexImpl` (Xbox PDB): finds the vertex with the
/// greatest dot product with `direction` and writes it to `out`, its `w` lane holding the vertex
/// index as the bits `0x3f000000 | index`.
///
/// Each `FourVectors` gives four dot products `(y * dy + x * dx) + z * dz`; per lane the greatest
/// so far (strictly greater wins, NaN never does) is kept with its lane number as a float
/// (`0.5, 1.5, ...`, advancing by 4.0 per block), starting from four copies of the function's
/// static initial value (a `float` read from `0104e148` once, guarded by bit 0 of `01268fb0`).
/// The lanes are then reduced pairwise with the `FCOMPP` tests (the second lane of a pair wins
/// ties) and the winner's number is truncated by `_ftol2` (`00ec62c0`).
pub fn hkp_convex_vertices_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    out: Ptr,
) {
    let me = this.addr();
    if e.mem.u8(BEST_INITIAL_GUARD) & 1 == 0 {
        let initial = x87_quiet(e.global::<f32>(BEST_INITIAL_SOURCE));
        let guard = e.mem.u32(BEST_INITIAL_GUARD);
        e.mem.set_u32(BEST_INITIAL_GUARD, guard | 1);
        for lane in 0..4 {
            e.mem.set_f32(BEST_INITIAL + 4 * lane, initial);
        }
    }
    let data = e.mem.u32(me + ROTATED_VERTICES);
    let blocks = e.mem.i32(me + ROTATED_VERTICES_SIZE).wrapping_mul(4);
    let direction = load_vec(e, direction.addr());
    let mut best = load_vec(e, BEST_INITIAL);
    let mut lane_numbers = load_vec(e, LANE_NUMBERS);
    let lane_step = load_vec(e, LANE_STEP);
    let mut best_numbers: Vec4 = [0.0; 4];
    if blocks > 0 {
        let block_count = (((blocks - 1) as u32) >> 2) + 1;
        for block in 0..block_count {
            let base = data.wrapping_add(block.wrapping_mul(FOUR_VECTORS));
            let x = load_vec(e, base);
            let y = load_vec(e, base + 0x10);
            let z = load_vec(e, base + 0x20);
            let dots: Vec4 = std::array::from_fn(|lane| {
                (y[lane] * direction[1] + x[lane] * direction[0]) + z[lane] * direction[2]
            });
            // `CMPLTPS best, dot`: strictly greater wins, NaN never does.
            let greater: [bool; 4] = std::array::from_fn(|lane| best[lane] < dots[lane]);
            best = std::array::from_fn(|lane| {
                if greater[lane] {
                    dots[lane]
                } else {
                    best[lane]
                }
            });
            best_numbers = std::array::from_fn(|lane| {
                if greater[lane] {
                    lane_numbers[lane]
                } else {
                    best_numbers[lane]
                }
            });
            lane_numbers = std::array::from_fn(|lane| lane_numbers[lane] + lane_step[lane]);
        }
    }
    // The first pair: lane 1 unless it is strictly below lane 0 (`FCOMPP`, `JP`).
    let first = if best[1] < best[0] { 0 } else { 1 };
    let second = if best[3] < best[2] { 2 } else { 3 };
    let winner = if best[second] < best[first] {
        first
    } else {
        second
    };
    let number = e.call(FTOL, &args![f64::from(best_numbers[winner])]).i32() as u32;
    let (block, lane) = vertex_slot(e, me, number);
    let vertex = std::array::from_fn::<f32, 3, _>(|axis| {
        x87_quiet(e.mem.f32(block + 0x10 * axis as u32 + 4 * lane))
    });
    store_vec(e, out.addr(), [vertex[0], vertex[1], vertex[2], 0.0]);
    e.mem.set_u32(out.addr() + 0xc, number | 0x3f00_0000);
}

// Translated from 00d59060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::convertVertexIdsToVerticesImpl` (Xbox PDB): for each of `count`
/// 16-bit vertex ids writes the vertex (x, y, z copied through x87) to `out` with the bits
/// `0x3f000000 | id` in `w`. Nothing for a count of zero or less.
pub fn hkp_convex_vertices_shape_convert_vertex_ids_to_vertices_impl(
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
        let vertex = vertex_through_x87(e, this.addr(), id);
        store_vec(
            e,
            out_address,
            [vertex[0], vertex[1], vertex[2], f32::from_bits(0)],
        );
        e.mem.set_u32(out_address + 0xc, id | 0x3f00_0000);
        id_address = id_address.wrapping_add(2);
        out_address = out_address.wrapping_add(0x10);
    }
}

// Translated from 00d591d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getAabbImpl` (Xbox PDB): forwards to `00d26230` with the transform,
/// the addresses `this + 0x20` and `this + 0x30` (`m_aabbHalfExtents` and `m_aabbCenter`), the
/// margin `m_radius + tolerance` (rounded to `float`) and `out`.
pub fn hkp_convex_vertices_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let radius = e.mem.f32(this.addr() + RADIUS);
    let margin = (f64::from(radius) + f64::from(tolerance)) as f32;
    e.call(
        AABB_OF_SEGMENT,
        &args![
            transform,
            this.addr() + 0x20,
            this.addr() + 0x30,
            margin,
            out
        ],
    );
}

/// Builds `m_planeEquations` from the convex piece's faces (`castRayImpl`, first time): for each
/// face but the last, the three vertex indices at `faceVertices + faceOffsets[i]` give the
/// plane through them: the normal `(v2 - v0) x (v1 - v0)` normalised with the refined `RSQRTSS`
/// (`+0.0` for a zero normal) and `w = 0.0 - normal . v0`; planes are appended, growing the
/// array (`00c90940`) when it is full.
fn build_plane_equations(e: &mut Engine, this: u32, piece: u32) {
    let face_vertices = e.mem.u32(piece + PIECE_FACE_VERTICES);
    let mut offsets = e.mem.u32(piece + PIECE_FACE_OFFSETS);
    let face_count = e.mem.i32(piece + PIECE_FACE_OFFSETS_SIZE).wrapping_sub(1);
    let array = this + PLANE_EQUATIONS;
    for _ in 0..face_count.max(0) {
        let face = face_vertices.wrapping_add(u32::from(e.mem.u16(offsets)));
        let corners: [[f32; 3]; 3] = std::array::from_fn(|corner| {
            let index = u32::from(e.mem.u8(face + corner as u32));
            vertex_through_x87(e, this, index)
        });
        let [v0, v1, v2] = corners;
        let a: [f32; 3] = std::array::from_fn(|axis| v1[axis] - v0[axis]);
        let b: [f32; 3] = std::array::from_fn(|axis| v2[axis] - v0[axis]);
        let normal = [
            b[2] * a[1] - b[1] * a[2],
            b[0] * a[2] - b[2] * a[0],
            b[1] * a[0] - b[0] * a[1],
        ];
        let squared = normal[2] * normal[2] + (normal[1] * normal[1] + normal[0] * normal[0]);
        let inverse = inverse_length(e, squared);
        let plane: [f32; 3] = normal.map(|lane| inverse * lane);
        let along = plane[2] * v0[2] + (plane[1] * v0[1] + plane[0] * v0[0]);
        let zero: f32 = e.global(ZEROS);
        let distance = x87_quiet(zero - along);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
        }
        offsets = offsets.wrapping_add(2);
        let size = e.mem.u32(array + 4);
        let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(0x10));
        store_vec(e, slot, [plane[0], plane[1], plane[2], distance]);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
    }
}

// Translated from 00d592a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::castRayImpl` (Xbox PDB): casts the ray `input`
/// (`hkpShapeRayCastInput`: `m_from`, `m_to`) against the convex hull and updates `output`
/// (`hkpShapeRayCastOutput`) when it enters the hull before `output.m_hitFraction`. Writes the
/// hit flag (`hkBool`) to `result` and returns `result`; a profiling record `"TtrcConvexVert"` is
/// written first and `"Et"` last.
///
/// The first time (no plane equations yet, a convex piece present) the planes are built from the
/// piece (`build_plane_equations`). Then, going through the planes from the last to the first, the
/// ray's signed distances `d0` (start) and `d1` (end) to a plane `(n, w)` are `(n_y y + n_x x) +
/// (n_z z + w)`. Both in front (`>= 0`) is a miss; both behind is no constraint; start in front
/// and end behind enters at `t = d0 / (d0 - d1)`, which raises the entry fraction when it is not
/// below it and keeps that plane as the normal; start behind and end in front exits at the same
/// `t`, lowering the exit fraction (initially `m_hitFraction`) when it is not above it. After
/// such a crossing an exit below the entry is a miss. A hit needs an entry fraction that is not
/// below zero (it starts at -1.0, NaN counts as a hit); it is stored as the hit fraction with the
/// plane as the normal (`w` lane included) and the shape key slot `m_shapeKeyIndex` set to -1.
pub fn hkp_convex_vertices_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CONVEX);
    let hit = cast_ray_against_hull(e, this.addr(), input.addr(), output.addr());
    profile_record(e, PROFILE_END);
    e.mem.set_u8(result.addr(), hit as u8);
    result
}

/// The body of `castRayImpl`.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the exe's flag tests, NaN included
fn cast_ray_against_hull(e: &mut Engine, me: u32, input: u32, out: u32) -> bool {
    if e.mem.u32(me + PLANE_EQUATIONS + 4) == 0 {
        let piece = e.mem.u32(me + CONVEX_PIECE);
        if piece != 0 {
            build_plane_equations(e, me, piece);
        }
    }
    let mut entry = x87_quiet(e.global::<f32>(MINUS_ONE));
    let mut exit = x87_quiet(e.mem.f32(out + OUT_HIT_FRACTION));
    let mut normal: Vec4 = [0.0; 4];
    let from = load_vec(e, input + RAY_FROM);
    let to = load_vec(e, input + RAY_TO);
    // The signed distance of `point` to `plane`: `(n_y y + n_x x) + (n_z z + w)`.
    let distance = |plane: Vec4, point: Vec4| -> f32 {
        (plane[1] * point[1] + plane[0] * point[0]) + (plane[2] * point[2] + plane[3])
    };
    let planes_size = e.mem.i32(me + PLANE_EQUATIONS + 4);
    let mut index = planes_size.wrapping_sub(1);
    if index < 0 {
        return false;
    }
    while index >= 0 {
        let data = e.mem.u32(me + PLANE_EQUATIONS);
        let plane = load_vec(e, data.wrapping_add((index as u32) << 4));
        let start = distance(plane, from);
        let end = distance(plane, to);
        // `FCOM` against 0.0 leaves C0 set for "below" and for unordered.
        let start_behind = start < 0.0 || start.is_nan();
        let end_behind = end < 0.0 || end.is_nan();
        if start_behind {
            if !end_behind {
                // Leaving through this plane.
                let crossing = (f64::from(start) / (f64::from(start) - f64::from(end))) as f32;
                // `JZ` skips the update when the crossing is above the exit (ordered).
                if !(crossing > exit) {
                    exit = crossing;
                }
                if exit < entry {
                    return false;
                }
            }
        } else if end_behind {
            // Entering through this plane.
            let crossing = (f64::from(start) / (f64::from(start) - f64::from(end))) as f32;
            if crossing >= entry {
                entry = crossing;
                normal = plane;
            }
            if exit < entry {
                return false;
            }
        } else {
            return false;
        }
        index -= 1;
    }
    // `FCOM` of the entry against 0.0, `JP`: a hit unless the entry is below zero.
    if entry < 0.0 {
        return false;
    }
    let key_index = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
    e.mem.set_f32(out + OUT_HIT_FRACTION, entry);
    for (lane, value) in normal.into_iter().enumerate() {
        e.mem.set_f32(out + OUT_NORMAL + 4 * lane as u32, value);
    }
    e.mem.set_u32(
        out.wrapping_add(key_index.wrapping_mul(4))
            .wrapping_add(OUT_SHAPE_KEYS),
        0xffff_ffff,
    );
    true
}

/// `hkReferencedObject` reference release (`00c905b0`, `this` in `ECX`, nothing on the stack).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkArray::reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkAabbUtil::calcAabb` (Xbox PDB name per the engine map, `00cbeaf0`, cdecl `(vertices,
/// count, stride, out)`): writes the minimum at `out` and the maximum at `out + 0x10`.
const CALC_AABB: u32 = 0x00cb_eaf0;
/// `hkVector4::setRotatedDir` (Xbox PDB name per the engine map, `00c7f700`): `ECX` = result,
/// two stack words (a matrix of three rows, a vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The vector constant of four 0.5, and the one of four sign bits.
const HALF_VECTOR: u32 = 0x010c_71d0;
const SIGN_MASK: u32 = 0x010c_44d0;
/// The vtable the shape constructors and destructors leave in the object, and the one the
/// destructors end with.
const SHAPE_VTABLE: u32 = 0x010d_0fc4;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The `hkArray` capacity word mask, and the empty word (no storage, not owned).
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const EMPTY_CAPACITY: u32 = 0x8000_0000;

/// The allocator behind the memory router (`00c85750`, its `+0x10`).
fn allocator_of_router(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes room for `wanted` elements of `element_size` bytes in the `hkArray` at `array`
/// (`00c908c0` with the larger of `wanted` and twice the current capacity) when its capacity,
/// compared as signed values, is below `wanted`.
fn reserve_array(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let allocator = allocator_of_router(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// Empties the `hkArray` at `array` (size at `+4`, capacity word at `+8`) and frees its data
/// (allocator slot `0x10`: data, capacity times `element_size` bytes) unless bit 31 of the
/// capacity word says it is not owned; the data pointer and capacity word are reset.
fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = allocator_of_router(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, EMPTY_CAPACITY);
}

/// Stores the vertex at `from` (three floats, each through `FLD`/`FSTP`) as vertex `index` of
/// the rotated vertices of the shape at `this`.
fn store_vertex(e: &mut Engine, this: u32, index: u32, from: u32) {
    let (block, lane) = vertex_slot(e, this, index);
    for axis in 0..3 {
        let bits = x87_quiet_bits(e.mem.u32(from + 4 * axis));
        e.mem.set_u32(block + 0x10 * axis + 4 * lane, bits);
    }
}

// Translated from 00d58d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getPlaneEquations` (Xbox PDB): the address of `m_planeEquations`.
pub fn hkp_convex_vertices_shape_get_plane_equations(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + PLANE_EQUATIONS)
}

// Translated from 00d59200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getOriginalVertices` (Xbox PDB): fills the `hkArray` at `array`
/// with one 16-byte vector `(x, y, z, 0.0)` per vertex, growing it first (`00c908c0`, 16-byte
/// elements) when its capacity is below `m_numVertices`. The loop re-reads `m_numVertices` on
/// every pass.
pub fn hkp_convex_vertices_shape_get_original_vertices(e: &mut Engine, this: Ptr, array: Ptr) {
    let me = this.addr();
    let array = array.addr();
    let count = e.mem.i32(me + NUM_VERTICES);
    reserve_array(e, array, count, 0x10);
    e.mem.set_u32(array + 4, count as u32);
    let zero = e.mem.u32(ZERO);
    let mut index = 0i32;
    while index < e.mem.i32(me + NUM_VERTICES) {
        let (block, lane) = vertex_slot(e, me, index as u32);
        let out = e
            .mem
            .u32(array)
            .wrapping_add((index as u32).wrapping_mul(0x10));
        for axis in 0..3 {
            let bits = e.mem.u32(block + 0x10 * axis + 4 * lane);
            e.mem.set_u32(out + 4 * axis, bits);
        }
        e.mem.set_u32(out + 0xc, zero);
        index += 1;
    }
}

// Translated from 00d59710 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = object, one stack word (a float). Stores the float
/// (through `FLD`/`FSTP`) as the radius at `+0x10`, sets the word at `+6` to 1, `+8` to 0, the
/// shape type at `+0xC` to 6, the vtable `010d0fc4`, and empties both arrays (`+0x40`/`+0x44`/
/// `+0x48` and `+0x54`/`+0x58`/`+0x5C`) and the words at `+0x50` and `+0x60`. Returns `this`.
pub fn fn_00d59710(e: &mut Engine, this: Ptr, radius: f32) -> Ptr {
    let me = this.addr();
    e.mem.set_f32(me + RADIUS, x87_quiet(radius));
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 6);
    e.mem.set_u32(me, SHAPE_VTABLE);
    e.mem.set_u32(me + ROTATED_VERTICES, 0);
    e.mem.set_u32(me + ROTATED_VERTICES_SIZE, 0);
    e.mem.set_u32(me + 0x48, EMPTY_CAPACITY);
    e.mem.set_u32(me + CONVEX_PIECE, 0);
    e.mem.set_u32(me + PLANE_EQUATIONS, 0);
    e.mem.set_u32(me + 0x58, 0);
    e.mem.set_u32(me + 0x5c, EMPTY_CAPACITY);
    e.mem.set_u32(me + 0x60, 0);
    this
}

// Translated from 00d59760 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = object, six stack words (the vertex blocks, the vertex
/// count, the plane vectors, the plane count, a box of two vectors, a radius float). The shape
/// borrows both blocks: `+0x40` = the vertex blocks with the size `(count + 3 & !3) / 4` (signed
/// division) and the same size with bit 31 set as capacity word, `+0x4C` = the count, `+0x50` =
/// 0, `+0x54`..`+0x5C` = the planes, their count and the count with bit 31 set, `+0x60` = 0. The
/// radius goes through `FLD`/`FSTP` to `+0x10`, `+6` = 1, `+8` = 0, `+0xC` = 6 and the vtable is
/// `010d0fc4`. `+0x20` = `(max - min) * 0.5` and `+0x30` = `(min + max) * 0.5` of the box (the
/// minimum at +0, the maximum at +0x10; the 0.5 is the float at `01016248`). Returns `this`.
pub fn fn_00d59760(
    e: &mut Engine,
    this: Ptr,
    vertices: Ptr,
    count: i32,
    planes: Ptr,
    plane_count: u32,
    bounds: Ptr,
    radius: f32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_f32(me + RADIUS, x87_quiet(radius));
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 6);
    e.mem.set_u32(me, SHAPE_VTABLE);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + ROTATED_VERTICES, vertices.addr());
    let rounded = count.wrapping_add(3) & !3;
    let fours = rounded.wrapping_add((rounded >> 31) & 3) >> 2;
    e.mem.set_u32(me + ROTATED_VERTICES_SIZE, fours as u32);
    e.mem.set_u32(me + 0x48, fours as u32 | EMPTY_CAPACITY);
    e.mem.set_u32(me + NUM_VERTICES, count as u32);
    e.mem.set_u32(me + CONVEX_PIECE, 0);
    e.mem.set_u32(me + PLANE_EQUATIONS, planes.addr());
    e.mem.set_u32(me + 0x58, plane_count);
    e.mem.set_u32(me + 0x5c, plane_count | EMPTY_CAPACITY);
    e.mem.set_u32(me + 0x60, 0);
    let minimum = load_vec(e, bounds.addr());
    let maximum = load_vec(e, bounds.addr() + 0x10);
    let half: f32 = e.global(HALF);
    let extents = std::array::from_fn(|lane| half * (maximum[lane] - minimum[lane]));
    store_vec(e, me + 0x20, extents);
    let centre = std::array::from_fn(|lane| half * (minimum[lane] + maximum[lane]));
    store_vec(e, me + AABB_CENTRE, centre);
    this
}

// Translated from 00d59830 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = object, one stack word (a flag). Leaves the shape type at
/// `+0xC` at 0x12, or at 6 when the flag is not 0, and the vtable `010d0fc4` (it passes through
/// the vtable `010305a8`, and through type 0 when the flag is set, on the way). Returns `this`.
pub fn fn_00d59830(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(me + 0xc, 0);
    }
    e.mem.set_u32(me + 0xc, 0x12);
    e.mem.set_u32(me, SHAPE_VTABLE);
    if flag != 0 {
        e.mem.set_u32(me + 0xc, 6);
    }
    this
}

// Translated from 00d59870 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = object, nothing on the stack. Sets the vtable
/// `010d0fc4`, releases the reference at `+0x60` when it is not 0 (the word is kept), empties
/// the plane array (`+0x54`/`+0x58`/`+0x5C`, 16-byte elements) and the vertex array
/// (`+0x40`/`+0x44`/`+0x48`, 48-byte elements) releasing their storage unless it is not owned,
/// and ends with the vtable `0102e388`.
pub fn fn_00d59870(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, SHAPE_VTABLE);
    let held = e.mem.u32(me + 0x60);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    release_array(e, me + PLANE_EQUATIONS, 0x10);
    release_array(e, me + ROTATED_VERTICES, FOUR_VECTORS);
    e.mem.set_u32(me, BASE_VTABLE);
}

// Translated from 00d59900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::copyVertexData` (Xbox PDB): `ECX` = object, three stack words
/// (the address of the first vertex, the stride in bytes, the vertex count). Stores the count
/// at `+0x4C`, sizes the vertex array to `(count + 3 & !3) / 4` blocks of 48 bytes (growing it
/// with `00c908c0` when its capacity is smaller), copies each vertex's three floats through
/// `FLD`/`FSTP` into its lane, repeats the last vertex (the one `stride` before the end of the
/// source) in the lanes up to the rounded count, then calls `hkAabbUtil::calcAabb` and sets
/// `+0x20` = `(max - min) * K` and `+0x30` = `(min + max) * K` with the vector `K` at
/// `010c71d0`.
pub fn hkp_convex_vertices_shape_copy_vertex_data(
    e: &mut Engine,
    this: Ptr,
    source: Ptr,
    stride: u32,
    count: i32,
) {
    let me = this.addr();
    e.mem.set_u32(me + NUM_VERTICES, count as u32);
    let rounded = count.wrapping_add(3) & !3;
    let fours = rounded.wrapping_add((rounded >> 31) & 3) >> 2;
    reserve_array(e, me + ROTATED_VERTICES, fours, FOUR_VECTORS);
    e.mem.set_u32(me + ROTATED_VERTICES_SIZE, fours as u32);
    let mut from = source.addr();
    for index in 0..count {
        store_vertex(e, me, index as u32, from);
        from = from.wrapping_add(stride);
    }
    let last = from.wrapping_sub(stride);
    for index in count.max(0)..rounded {
        store_vertex(e, me, index as u32, last);
    }
    let (minimum, maximum) = e.with_stack(0x20, |e, bounds| {
        e.call(CALC_AABB, &args![source, count, stride, bounds]);
        (
            load_vec(e, bounds.addr()),
            load_vec(e, bounds.addr() + 0x10),
        )
    });
    let scale = load_vec(e, HALF_VECTOR);
    let extents = std::array::from_fn(|lane| (maximum[lane] - minimum[lane]) * scale[lane]);
    store_vec(e, me + 0x20, extents);
    let centre = std::array::from_fn(|lane| (minimum[lane] + maximum[lane]) * scale[lane]);
    store_vec(e, me + AABB_CENTRE, centre);
}

// Translated from 00d59bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::hkpConvexVerticesShape` (Xbox PDB): `ECX` = object, three stack
/// words (a block of three words: the address of the first vertex, the vertex count and the
/// stride in bytes; an `hkArray` of 16-byte plane vectors; a radius float). Sets the radius
/// (through `FLD`/`FSTP`), the shape type 6, the vtable `010d0fc4`, empties both arrays, copies
/// the planes into a block the memory router's allocator gives (slot `0xC`, which returns the
/// size it granted in the cell it is given; the old block would be freed through slot `0x10`
/// when the capacity word did not say it is not owned) and calls `copyVertexData` with the
/// vertex block's address, stride and count. Returns `this`.
pub fn hkp_convex_vertices_shape_ctor(
    e: &mut Engine,
    this: Ptr,
    vertices: Ptr,
    planes: Ptr,
    radius: f32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_f32(me + RADIUS, x87_quiet(radius));
    e.mem.set_u32(me + 0xc, 6);
    e.mem.set_u32(me, SHAPE_VTABLE);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + ROTATED_VERTICES, 0);
    e.mem.set_u32(me + ROTATED_VERTICES_SIZE, 0);
    e.mem.set_u32(me + 0x48, EMPTY_CAPACITY);
    e.mem.set_u32(me + CONVEX_PIECE, 0);
    e.mem.set_u32(me + PLANE_EQUATIONS, 0);
    e.mem.set_u32(me + 0x58, 0);
    e.mem.set_u32(me + 0x5c, EMPTY_CAPACITY);
    e.mem.set_u32(me + 0x60, 0);
    let allocator = allocator_of_router(e);
    let capacity_word = e.mem.u32(me + 0x5c);
    let capacity = (capacity_word & CAPACITY_MASK) as i32;
    let wanted = e.mem.i32(planes.addr() + 4);
    if capacity < wanted {
        if capacity_word as i32 >= 0 {
            let data = e.mem.u32(me + PLANE_EQUATIONS);
            e.vcall(allocator, 0x10, &args![data, (capacity as u32) << 4]);
        }
        let (data, granted) = e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), (wanted as u32) << 4);
            let data = e.vcall(allocator, 0xc, &args![cell]).u32();
            (data, e.mem.i32(cell.addr()))
        });
        e.mem.set_u32(me + PLANE_EQUATIONS, data);
        e.mem.set_u32(
            me + 0x5c,
            (granted.wrapping_add((granted >> 31) & 0xf) >> 4) as u32,
        );
    }
    let wanted = e.mem.i32(planes.addr() + 4);
    e.mem.set_u32(me + 0x58, wanted as u32);
    let mut to = e.mem.u32(me + PLANE_EQUATIONS);
    let mut from = e.mem.u32(planes.addr());
    for _ in 0..wanted.max(0) {
        copy_vec(e, from, to);
        from = from.wrapping_add(0x10);
        to = to.wrapping_add(0x10);
    }
    let first = e.mem.u32(vertices.addr());
    let stride = e.mem.u32(vertices.addr() + 8);
    let count = e.mem.i32(vertices.addr() + 4);
    hkp_convex_vertices_shape_copy_vertex_data(e, this, Ptr::new(first), stride, count);
    this
}

// Translated from 00d59cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::transformVerticesAndPlaneEquations` (Xbox PDB): `ECX` = object, one
/// stack word (a matrix of four rows of four floats). Takes a block from the memory router's
/// stack allocator (its word at `+0` is the top, advanced by `16 * count` rounded up to 128 and
/// put back at the end) for the original vertices (`getOriginalVertices`, the count is the
/// result of the virtual function at `+0x2C`), replaces each vertex `v` by `(v.y * row1 + v.x *
/// row0) + (v.z * row2 + row3)`, stores them with `copyVertexData` (stride 16), then moves
/// every plane equation: the normal goes through `hkVector4::setRotatedDir` with the first
/// three rows and the fourth component becomes `(n.z * t.z + w) + (n.y * t.y + n.x * t.x)`
/// where `t` is built from the columns of the rotation and the negated fourth row. A plane
/// count of 0 or less still runs the loop once, as the exe does. Frees the block through the
/// allocator (slot `0x10`) only if it is owned.
pub fn hkp_convex_vertices_shape_transform_vertices_and_plane_equations(
    e: &mut Engine,
    this: Ptr,
    matrix: Ptr,
) {
    let me = this.addr();
    let count = e.vcall(me, 0x2c, &args![]).u32();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let base = e.mem.u32(router);
    e.mem.set_u32(
        router,
        base.wrapping_add((count << 4).wrapping_add(0x7f) & 0xffff_ff80),
    );
    e.with_stack(12, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, base);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, count | EMPTY_CAPACITY);
        hkp_convex_vertices_shape_get_original_vertices(e, this, Ptr::new(array));
        let rows: [Vec4; 4] =
            std::array::from_fn(|row| load_vec(e, matrix.addr() + 0x10 * row as u32));
        let data = e.mem.u32(array);
        let size = e.mem.i32(array + 4);
        let mut index = size - 1;
        while index >= 0 {
            let at = data.wrapping_add((index as u32).wrapping_mul(0x10));
            let vertex = load_vec(e, at);
            let moved = std::array::from_fn(|lane| {
                (vertex[1] * rows[1][lane] + vertex[0] * rows[0][lane])
                    + (vertex[2] * rows[2][lane] + rows[3][lane])
            });
            store_vec(e, at, moved);
            index -= 1;
        }
        hkp_convex_vertices_shape_copy_vertex_data(e, this, Ptr::new(data), 0x10, size);

        let rows: [Vec4; 4] =
            std::array::from_fn(|row| load_vec(e, matrix.addr() + 0x10 * row as u32));
        let planes = e.mem.u32(me + PLANE_EQUATIONS);
        let plane_count = e.mem.i32(me + 0x58);
        let negated: Vec4 = std::array::from_fn(|lane| {
            f32::from_bits(rows[3][lane].to_bits() ^ e.mem.u32(SIGN_MASK + 4 * lane as u32))
        });
        let columns: [Vec4; 3] = [
            [rows[0][0], rows[1][0], rows[2][0], rows[2][1]],
            [rows[0][1], rows[1][1], rows[2][1], rows[2][3]],
            [rows[0][2], rows[1][2], rows[2][2], rows[2][3]],
        ];
        let offset: Vec4 = std::array::from_fn(|lane| {
            (negated[1] * columns[1][lane] + negated[0] * columns[0][lane])
                + columns[2][lane] * negated[2]
        });
        e.with_stack(0x30, |e, rotation| {
            for row in 0..3 {
                copy_vec(e, matrix.addr() + 0x10 * row, rotation.addr() + 0x10 * row);
            }
            let mut plane = planes;
            for _ in 0..plane_count.max(1) {
                let normal = load_vec(e, plane);
                let product: Vec4 = std::array::from_fn(|lane| normal[lane] * offset[lane]);
                let distance = (product[2] + normal[3]) + (product[1] + product[0]);
                e.call(SET_ROTATED_DIR, &args![plane, rotation, plane]);
                let rotated = load_vec(e, plane);
                store_vec(e, plane, [rotated[0], rotated[1], rotated[2], distance]);
                plane = plane.wrapping_add(0x10);
            }
        });

        // The block goes back to the router's stack allocator; the size is cleared even
        // when the block was replaced.
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, base);
        let allocator = allocator_of_router(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if capacity as i32 >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK) << 4],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d58d10,
            hkp_convex_vertices_shape_get_num_collision_spheres_impl(Ptr) -> u32
        ),
        entry!(
            0x00d58d20,
            hkp_convex_vertices_shape_get_centre_impl(Ptr, Ptr)
        ),
        entry!(
            0x00d58d60,
            hkp_convex_vertices_shape_get_first_vertex(Ptr, Ptr)
        ),
        entry!(
            0x00d58d80,
            hkp_convex_vertices_shape_get_collision_spheres_impl(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d58ee0,
            hkp_convex_vertices_shape_get_supporting_vertex_impl(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d59060,
            hkp_convex_vertices_shape_convert_vertex_ids_to_vertices_impl(Ptr, Ptr, i32, Ptr)
        ),
        entry!(
            0x00d591d0,
            hkp_convex_vertices_shape_get_aabb_impl(Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d592a0,
            hkp_convex_vertices_shape_cast_ray_impl(Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d58d50,
            hkp_convex_vertices_shape_get_plane_equations(Ptr) -> Ptr
        ),
        entry!(
            0x00d59200,
            hkp_convex_vertices_shape_get_original_vertices(Ptr, Ptr)
        ),
        entry!(0x00d59710, fn_00d59710(Ptr, f32) -> Ptr),
        entry!(
            0x00d59760,
            fn_00d59760(Ptr, Ptr, i32, Ptr, u32, Ptr, f32) -> Ptr
        ),
        entry!(0x00d59830, fn_00d59830(Ptr, u32) -> Ptr),
        entry!(0x00d59870, fn_00d59870(Ptr)),
        entry!(
            0x00d59900,
            hkp_convex_vertices_shape_copy_vertex_data(Ptr, Ptr, u32, i32)
        ),
        entry!(
            0x00d59bd0,
            hkp_convex_vertices_shape_ctor(Ptr, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(
            0x00d59cc0,
            hkp_convex_vertices_shape_transform_vertices_and_plane_equations(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.set_global(HALF, 0.5f32);
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(BEST_INITIAL_SOURCE, f32::from_bits(0xff7f_ffee));
        store_vec(&mut e, LANE_NUMBERS, [0.5, 1.5, 2.5, 3.5]);
        store_vec(&mut e, LANE_STEP, [4.0; 4]);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A hull of six vertices in two `FourVectors`: (0,0,0), (1,0,0), (0,1,0), (0,0,1) and
    /// (2,3,4), (-1,-2,-3), the unused lanes zero; `m_radius` 0.25.
    fn hull(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x70);
        let data = e.mem.alloc(0x60);
        store_vec(e, data, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, data + 0x10, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, data + 0x20, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, data + 0x30, [2.0, -1.0, 0.0, 0.0]);
        store_vec(e, data + 0x40, [3.0, -2.0, 0.0, 0.0]);
        store_vec(e, data + 0x50, [4.0, -3.0, 0.0, 0.0]);
        e.mem.set_u32(this + ROTATED_VERTICES, data);
        e.mem.set_u32(this + ROTATED_VERTICES_SIZE, 2);
        e.mem.set_u32(this + NUM_VERTICES, 6);
        e.mem.set_f32(this + RADIUS, 0.25);
        store_vec(e, this + AABB_CENTRE, [1.0, 2.0, 3.0, 4.0]);
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

    #[test]
    fn num_collision_spheres_is_the_vertex_count() {
        let mut e = engine();
        let this = hull(&mut e);
        assert_eq!(e.call(0x00d58d10, &args![this]).u32(), 6);
    }

    #[test]
    fn centre_is_the_aabb_centre() {
        let mut e = engine();
        let this = hull(&mut e);
        let out = e.mem.alloc(16);
        e.call(0x00d58d20, &args![this, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn first_vertex_is_quieted_and_has_a_zero_w() {
        let mut e = engine();
        let this = hull(&mut e);
        let data = e.mem.u32(this.addr() + ROTATED_VERTICES);
        e.mem.set_u32(data, 0x7f80_0001);
        e.mem.set_f32(data + 0x10, 5.0);
        e.mem.set_f32(data + 0x20, 6.0);
        let out = e.mem.alloc(16);
        e.mem.set_f32(out + 0xc, 9.0);
        e.call(0x00d58d60, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
        assert_eq!(e.mem.f32(out + 4), 5.0);
        assert_eq!(e.mem.f32(out + 8), 6.0);
        assert_eq!(e.mem.u32(out + 0xc), 0);
    }

    #[test]
    fn collision_spheres_answer_the_buffer() {
        let mut e = engine();
        let this = hull(&mut e);
        let out = e.mem.alloc(0x60);
        // EAX is `out` at the RET (00d58ed1); the convex transform shape reads it (00d5df2f).
        assert_eq!(e.call(0x00d5_8d80, &args![this, out]).u32(), out);
    }

    #[test]
    fn collision_spheres_are_the_vertices_with_the_radius() {
        let mut e = engine();
        let this = hull(&mut e);
        let out = e.mem.alloc(0x60);
        e.call(0x00d58d80, &args![this, out]);
        let expected = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [2.0, 3.0, 4.0],
            [-1.0, -2.0, -3.0],
        ];
        for (index, vertex) in expected.iter().enumerate() {
            let sphere = load_vec(&e, out + 0x10 * index as u32);
            assert_eq!(sphere, [vertex[0], vertex[1], vertex[2], 0.25]);
        }
        // No vertices, nothing written.
        e.mem.set_u32(this.addr() + NUM_VERTICES, 0);
        let untouched = e.mem.alloc(0x10);
        e.call(0x00d58d80, &args![this, untouched]);
        assert_eq!(e.mem.u32(untouched + 0xc), 0);
    }

    #[test]
    fn supporting_vertex_is_the_furthest_along_the_direction() {
        let mut e = engine();
        e.register_double(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            ret(value as i32 as u32)
        });
        let this = hull(&mut e);
        let out = e.mem.alloc(16);
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 1.0, 1.0, 0.0]);
        e.call(0x00d58ee0, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0004);
        // The static of four initial values was filled in once.
        assert_eq!(e.mem.u8(BEST_INITIAL_GUARD) & 1, 1);
        assert_eq!(load_vec(&e, BEST_INITIAL), [f32::from_bits(0xff7f_ffee); 4]);
        store_vec(&mut e, direction, [-1.0, -1.0, -1.0, 0.0]);
        e.call(0x00d58ee0, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [-1.0, -2.0, -3.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0005);
    }

    #[test]
    fn supporting_vertex_without_vertices_is_the_first_one() {
        let mut e = engine();
        e.register_double(FTOL, |_, _| ret(0));
        let this = hull(&mut e);
        e.mem.set_u32(this.addr() + ROTATED_VERTICES_SIZE, 0);
        let out = e.mem.alloc(16);
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00d58ee0, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0000);
    }

    #[test]
    fn vertex_ids_expand_to_vertices() {
        let mut e = engine();
        let this = hull(&mut e);
        let ids = e.mem.alloc(8);
        e.mem.set_u16(ids, 4);
        e.mem.set_u16(ids + 2, 5);
        e.mem.set_u16(ids + 4, 1);
        let out = e.mem.alloc(0x30);
        e.call(0x00d59060, &args![this, ids, 3i32, out]);
        assert_eq!(load_vec(&e, out)[..3], [2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0004);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [-1.0, -2.0, -3.0]);
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f00_0005);
        assert_eq!(load_vec(&e, out + 0x20)[..3], [1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0x2c), 0x3f00_0001);
        let untouched = e.mem.alloc(0x10);
        e.call(0x00d59060, &args![this, ids, 0i32, untouched]);
        e.call(0x00d59060, &args![this, ids, -1i32, untouched]);
        assert_eq!(e.mem.u32(untouched + 0xc), 0);
    }

    #[test]
    fn aabb_forwards_the_margin_and_the_box_vectors() {
        let mut e = engine();
        let this = hull(&mut e);
        e.register_double(AABB_OF_SEGMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d591d0, &args![this, 0x1111u32, 0.5f32, 0x2222u32]);
        assert_eq!(
            calls_to(&e, AABB_OF_SEGMENT),
            vec![vec![
                0x1111,
                this.addr() + 0x20,
                this.addr() + 0x30,
                0.75f32.to_bits(),
                0x2222
            ]]
        );
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

    fn assert_profile_records(e: &Engine, stream: u32) {
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CONVEX);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
    }

    struct Cast {
        hit: bool,
        output: u32,
        stream: u32,
    }

    /// Casts `from` -> `to` at `this` with `hit_fraction` already in the output.
    fn cast(e: &mut Engine, this: Ptr, from: Vec4, to: Vec4, hit_fraction: f32) -> Cast {
        let stream = profile_stream(e);
        let input = e.mem.alloc(0x30);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, hit_fraction);
        e.mem.set_u32(output + 0x40, 2);
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let returned = e
            .call(0x00d592a0, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        Cast {
            hit: e.mem.u8(result) != 0,
            output,
            stream,
        }
    }

    /// The hull with the six planes of the cube of half width 1 around the origin.
    fn cube(e: &mut Engine) -> Ptr {
        let this = hull(e);
        let planes = e.mem.alloc(0x60);
        let normals = [
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ];
        for (index, normal) in normals.iter().enumerate() {
            store_vec(
                e,
                planes + 0x10 * index as u32,
                [normal[0], normal[1], normal[2], -1.0],
            );
        }
        e.mem.set_u32(this.addr() + PLANE_EQUATIONS, planes);
        e.mem.set_u32(this.addr() + PLANE_EQUATIONS + 4, 6);
        e.mem.set_u32(this.addr() + PLANE_EQUATIONS + 8, 6);
        this
    }

    #[test]
    fn cast_ray_enters_through_the_nearest_face() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 0.1, 0.2, 0.0],
            [-3.0, 0.1, 0.2, 0.0],
            1.0,
        );
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 1.0 / 3.0).abs() < 1e-6);
        assert_eq!(load_vec(&e, c.output), [1.0, 0.0, 0.0, -1.0]);
        assert_eq!(e.mem.u32(c.output + 0x20 + 2 * 4), 0xffff_ffff);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_in_front_of_a_face_misses() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 3.0, 0.0, 0.0],
            [3.0, -3.0, 0.0, 0.0],
            1.0,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 1.0);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_behind_a_nearer_hit_misses() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 0.1, 0.2, 0.0],
            [-3.0, 0.1, 0.2, 0.0],
            0.2,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.2);
    }

    #[test]
    fn cast_ray_starting_inside_has_no_entry_and_misses() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [0.0, 0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            1.0,
        );
        assert!(!c.hit);
    }

    #[test]
    fn cast_ray_without_planes_or_piece_misses() {
        let mut e = engine();
        let this = hull(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 0.1, 0.2, 0.0],
            [-3.0, 0.1, 0.2, 0.0],
            1.0,
        );
        assert!(!c.hit);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_builds_the_plane_equations_from_the_convex_piece() {
        let mut e = engine();
        let this = hull(&mut e);
        // Two faces (the offsets array has three entries, the last is not a face): vertices
        // (0,1,2) and (0,2,3) at byte offsets 0 and 3.
        let piece = e.mem.alloc(0x40);
        let faces = e.mem.alloc(8);
        for (index, vertex) in [0u8, 1, 2, 0, 2, 3].into_iter().enumerate() {
            e.mem.set_u8(faces + index as u32, vertex);
        }
        let offsets = e.mem.alloc(8);
        e.mem.set_u16(offsets, 0);
        e.mem.set_u16(offsets + 2, 3);
        e.mem.set_u16(offsets + 4, 6);
        e.mem.set_u32(piece + PIECE_FACE_VERTICES, faces);
        e.mem.set_u32(piece + PIECE_FACE_OFFSETS, offsets);
        e.mem.set_u32(piece + PIECE_FACE_OFFSETS_SIZE, 3);
        e.mem.set_u32(this.addr() + CONVEX_PIECE, piece);
        // The router's allocator and the array growth: capacity 0, so the first plane grows it.
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa110c);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            assert_eq!(a.len(), 3);
            let storage = e.mem.alloc(0x80);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let c = cast(
            &mut e,
            this,
            [-0.5, -0.5, 2.0, 0.0],
            [-0.5, -0.5, -2.0, 0.0],
            1.0,
        );
        let array = this.addr() + PLANE_EQUATIONS;
        assert_eq!(e.mem.u32(array + 4), 2);
        let data = e.mem.u32(array);
        assert_near(load_vec(&e, data), [0.0, 0.0, 1.0, 0.0], 1e-6);
        assert_near(load_vec(&e, data + 0x10), [1.0, 0.0, 0.0, 0.0], 1e-6);
        let reserve_calls = calls_to(&e, ARRAY_RESERVE_MORE);
        assert_eq!(reserve_calls, vec![vec![0xa110c, array, 0x10]]);
        // The ray enters through the first plane (z = 0) halfway.
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 0.5).abs() < 1e-6);
        assert_near(load_vec(&e, c.output), [0.0, 0.0, 1.0, 0.0], 1e-6);
        // A second cast does not build the planes again.
        let again = cast(
            &mut e,
            this,
            [-0.5, -0.5, 2.0, 0.0],
            [-0.5, -0.5, -2.0, 0.0],
            1.0,
        );
        assert!(again.hit);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert!(calls_to(&e, ARRAY_RESERVE_MORE).is_empty());
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

    /// What the doubles saw, in order: a tag and the argument words.
    type Events = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// Doubles for everything the functions call outside this file. The memory router has a
    /// stack allocator top at `+0` (0) and an allocator at `+0x10` whose slot `0xC` grants 16
    /// bytes more than asked and whose slot `0x10` logs the free; `hkArray::reserve` gives the
    /// array new storage of the requested capacity; `calcAabb` writes the box (-1, -2, -3, -4)
    /// to (3, 2, 1, 0); `setRotatedDir` rotates the source by the three rows it is given. The
    /// vector of 0.5 and the vector of sign bits are in memory. Returns the router and the
    /// events.
    fn services(e: &mut Engine) -> (u32, Events) {
        let events = Events::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_000c);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        let sink = Rc::clone(&events);
        e.register_double(0x0900_000c, move |e, a| {
            let size = e.mem.u32(a[1]);
            sink.borrow_mut().push(("alloc", vec![size]));
            e.mem.set_u32(a[1], size + 16);
            ret(e.mem.alloc(size + 16))
        });
        let sink = Rc::clone(&events);
        e.register_double(0x0900_0010, move |_, a| {
            sink.borrow_mut().push(("free", a[1..].to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&events);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            sink.borrow_mut().push(("release", a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&events);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(("reserve", a[1..].to_vec()));
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let sink = Rc::clone(&events);
        e.register_double(CALC_AABB, move |e, a| {
            sink.borrow_mut().push(("aabb", a.to_vec()));
            store_vec(e, a[3], [-1.0, -2.0, -3.0, -4.0]);
            store_vec(e, a[3] + 0x10, [3.0, 2.0, 1.0, 0.0]);
            Ret::default()
        });
        let sink = Rc::clone(&events);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            sink.borrow_mut().push(("rotate", a.to_vec()));
            let rows: Vec<f32> = (0..12).map(|word| e.mem.f32(a[1] + 4 * word)).collect();
            let source = load_vec(e, a[2]);
            let rotated = std::array::from_fn(|lane| {
                source[1] * rows[4 + lane] + source[0] * rows[lane] + source[2] * rows[8 + lane]
            });
            store_vec(e, a[0], rotated);
            Ret::default()
        });
        store_vec(e, HALF_VECTOR, [0.5; 4]);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        (router, events)
    }

    /// The raw words of vertex `index` of the shape at `this`.
    fn vertex_bits(e: &Engine, this: u32, index: u32) -> [u32; 3] {
        let (block, lane) = vertex_slot(e, this, index);
        std::array::from_fn(|axis| e.mem.u32(block + 0x10 * axis as u32 + 4 * lane))
    }

    /// The bits of the floats `x`, `y`, `z`.
    fn bits3(x: f32, y: f32, z: f32) -> [u32; 3] {
        [x.to_bits(), y.to_bits(), z.to_bits()]
    }

    #[test]
    fn plane_equations_is_the_address_of_the_array() {
        let mut e = engine();
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00d5_8d50, &args![this]).u32(), this + 0x54);
    }

    #[test]
    fn original_vertices_grow_the_array_and_fill_it() {
        let mut e = engine();
        let (_, events) = services(&mut e);
        let this = hull(&mut e).addr();
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array + 8, 2);
        e.call(0x00d5_9200, &args![this, array]);
        // The capacity 2 doubled is 4, below 6, so 6 is asked for.
        assert_eq!(*events.borrow(), vec![("reserve", vec![array, 6, 0x10])]);
        assert_eq!(e.mem.u32(array + 4), 6);
        let data = e.mem.u32(array);
        let expected = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [2.0, 3.0, 4.0],
            [-1.0, -2.0, -3.0],
        ];
        for (index, vertex) in expected.into_iter().enumerate() {
            let at = data + 0x10 * index as u32;
            assert_eq!(load_vec(&e, at), [vertex[0], vertex[1], vertex[2], 0.0]);
        }
        // A large enough array is not grown.
        events.borrow_mut().clear();
        e.mem.set_u32(array + 8, 8);
        e.call(0x00d5_9200, &args![this, array]);
        assert!(events.borrow().is_empty());
    }

    #[test]
    fn fn_00d59710_empties_the_shape() {
        let mut e = engine();
        let this = e.mem.alloc(0x70);
        for offset in (0x40..0x64).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let returned = e.call(0x00d5_9710, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(returned.u32(), this);
        assert_eq!(e.mem.u32(this + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        assert_eq!(e.mem.u32(this), 0x010d_0fc4);
        for offset in [0x40, 0x44, 0x50, 0x54, 0x58, 0x60] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x5c), 0x8000_0000);
        // The count word is not touched.
        assert_eq!(e.mem.u32(this + 0x4c), 0xdead_beef);
    }

    #[test]
    fn fn_00d59760_borrows_the_blocks_and_derives_the_box() {
        let mut e = engine();
        let this = e.mem.alloc(0x70);
        let bounds = e.mem.alloc(0x20);
        store_vec(&mut e, bounds, [-2.0, -1.0, 0.0, 0.0]);
        store_vec(&mut e, bounds + 0x10, [2.0, 3.0, 4.0, 0.0]);
        let returned = e.call(
            0x00d5_9760,
            &args![
                this,
                0x1111_0000u32,
                6i32,
                0x2222_0000u32,
                3u32,
                bounds,
                0.25f32
            ],
        );
        assert_eq!(returned.u32(), this);
        assert_eq!(e.mem.f32(this + 0x10), 0.25);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        assert_eq!(e.mem.u32(this), 0x010d_0fc4);
        assert_eq!(e.mem.u32(this + 0x40), 0x1111_0000);
        // 6 vertices round up to 8, which is 2 blocks.
        assert_eq!(e.mem.u32(this + 0x44), 2);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_0002);
        assert_eq!(e.mem.u32(this + 0x4c), 6);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(e.mem.u32(this + 0x54), 0x2222_0000);
        assert_eq!(e.mem.u32(this + 0x58), 3);
        assert_eq!(e.mem.u32(this + 0x5c), 0x8000_0003);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(load_vec(&e, this + 0x20), [2.0, 2.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x30), [0.0, 1.0, 2.0, 0.0]);
    }

    #[test]
    fn fn_00d59830_ends_with_type_6_only_when_the_flag_is_set() {
        let mut e = engine();
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00d5_9830, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        assert_eq!(e.mem.u32(this), 0x010d_0fc4);
        e.call(0x00d5_9830, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x12);
        assert_eq!(e.mem.u32(this), 0x010d_0fc4);
    }

    #[test]
    fn fn_00d59870_releases_the_reference_and_the_owned_storage() {
        let mut e = engine();
        let (_, events) = services(&mut e);
        let this = e.mem.alloc(0x70);
        let planes = e.mem.alloc(0x40);
        let vertices = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x60, 0xa0);
        e.mem.set_u32(this + 0x54, planes);
        e.mem.set_u32(this + 0x58, 2);
        e.mem.set_u32(this + 0x5c, 4);
        e.mem.set_u32(this + 0x40, vertices);
        e.mem.set_u32(this + 0x44, 2);
        e.mem.set_u32(this + 0x48, 2);
        e.call(0x00d5_9870, &args![this]);
        assert_eq!(
            *events.borrow(),
            vec![
                ("release", vec![0xa0]),
                ("free", vec![planes, 0x40]),
                ("free", vec![vertices, 0x60]),
            ]
        );
        for offset in [0x40, 0x44, 0x54, 0x58] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x5c), 0x8000_0000);
        // The reference word is kept and the vtable is the base one.
        assert_eq!(e.mem.u32(this + 0x60), 0xa0);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        // Borrowed storage and no reference: nothing is released or freed.
        events.borrow_mut().clear();
        e.mem.set_u32(this + 0x60, 0);
        e.mem.set_u32(this + 0x54, planes);
        e.mem.set_u32(this + 0x5c, 0x8000_0004);
        e.mem.set_u32(this + 0x40, vertices);
        e.mem.set_u32(this + 0x48, 0x8000_0002);
        e.call(0x00d5_9870, &args![this]);
        assert!(events.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x54), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0);
    }

    /// `count` vertices of 20 bytes each, `(i, i + 0.5, -i)` after a first vertex that starts
    /// with a signalling NaN.
    fn vertex_block(e: &mut Engine, count: u32) -> u32 {
        let source = e.mem.alloc(20 * count + 4);
        for index in 0..count {
            let at = source + 20 * index;
            e.mem.set_f32(at, index as f32);
            e.mem.set_f32(at + 4, index as f32 + 0.5);
            e.mem.set_f32(at + 8, -(index as f32));
        }
        e.mem.set_u32(source, 0x7f80_0001);
        source
    }

    #[test]
    fn copy_vertex_data_fills_lanes_pads_with_the_last_vertex_and_sets_the_box() {
        let mut e = engine();
        let (_, events) = services(&mut e);
        let this = e.mem.alloc(0x70);
        let source = vertex_block(&mut e, 5);
        e.call(0x00d5_9900, &args![this, source, 20u32, 5i32]);
        assert_eq!(e.mem.u32(this + 0x4c), 5);
        assert_eq!(e.mem.u32(this + 0x44), 2);
        let events_seen = events.borrow().clone();
        assert_eq!(events_seen[0], ("reserve", vec![this + 0x40, 2, 0x30]));
        assert_eq!(events_seen[1].0, "aabb");
        assert_eq!(events_seen[1].1[..3], [source, 5, 20]);
        // The signalling NaN is quieted by the x87 copy.
        assert_eq!(
            vertex_bits(&e, this, 0),
            [0x7fc0_0001, 0.5f32.to_bits(), (-0.0f32).to_bits()]
        );
        for index in 1..5 {
            let value = index as f32;
            assert_eq!(
                vertex_bits(&e, this, index),
                bits3(value, value + 0.5, -value)
            );
        }
        // Lanes 5 to 7 repeat vertex 4.
        for index in 5..8 {
            assert_eq!(vertex_bits(&e, this, index), bits3(4.0, 4.5, -4.0));
        }
        assert_eq!(load_vec(&e, this + 0x20), [2.0, 2.0, 2.0, 2.0]);
        assert_eq!(load_vec(&e, this + 0x30), [1.0, 0.0, -1.0, -2.0]);
        // No vertices: nothing is copied and the size is 0, with enough capacity left over.
        events.borrow_mut().clear();
        e.call(0x00d5_9900, &args![this, source, 20u32, 0i32]);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(events.borrow().len(), 1);
        assert_eq!(events.borrow()[0].0, "aabb");
    }

    #[test]
    fn constructor_copies_the_planes_and_the_vertices() {
        let mut e = engine();
        let (_, events) = services(&mut e);
        let this = e.mem.alloc(0x70);
        let source = vertex_block(&mut e, 2);
        let strided = e.mem.alloc(0xc);
        e.mem.set_u32(strided, source);
        e.mem.set_u32(strided + 4, 2);
        e.mem.set_u32(strided + 8, 20);
        let plane_data = e.mem.alloc(0x20);
        store_vec(&mut e, plane_data, [1.0, 0.0, 0.0, 5.0]);
        store_vec(&mut e, plane_data + 0x10, [0.0, 1.0, 0.0, 6.0]);
        let planes = e.mem.alloc(0xc);
        e.mem.set_u32(planes, plane_data);
        e.mem.set_u32(planes + 4, 2);
        let returned = e.call(0x00d5_9bd0, &args![this, strided, planes, 0.5f32]);
        assert_eq!(returned.u32(), this);
        assert_eq!(e.mem.f32(this + 0x10), 0.5);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        assert_eq!(e.mem.u32(this), 0x010d_0fc4);
        assert_eq!(e.mem.u16(this + 6), 1);
        // The allocator grants 48 bytes for the 32 asked: 3 planes of capacity.
        assert_eq!(events.borrow()[0], ("alloc", vec![0x20]));
        assert_eq!(e.mem.u32(this + 0x5c), 3);
        assert_eq!(e.mem.u32(this + 0x58), 2);
        let copied = e.mem.u32(this + 0x54);
        assert_ne!(copied, plane_data);
        assert_eq!(load_vec(&e, copied), [1.0, 0.0, 0.0, 5.0]);
        assert_eq!(load_vec(&e, copied + 0x10), [0.0, 1.0, 0.0, 6.0]);
        // The vertices went through copyVertexData: 2 vertices fill one block.
        assert_eq!(e.mem.u32(this + 0x4c), 2);
        assert_eq!(e.mem.u32(this + 0x44), 1);
        assert_eq!(vertex_bits(&e, this, 1), bits3(1.0, 1.5, -1.0));
        assert_eq!(vertex_bits(&e, this, 3), bits3(1.0, 1.5, -1.0));
        // Without planes nothing is allocated for them.
        events.borrow_mut().clear();
        let other = e.mem.alloc(0x70);
        e.mem.set_u32(planes + 4, 0);
        e.call(0x00d5_9bd0, &args![other, strided, planes, 0.5f32]);
        assert!(events.borrow().iter().all(|(tag, _)| *tag != "alloc"));
        assert_eq!(e.mem.u32(other + 0x54), 0);
        assert_eq!(e.mem.u32(other + 0x58), 0);
    }

    #[test]
    fn transform_moves_the_vertices_and_the_planes() {
        let mut e = engine();
        let (router, events) = services(&mut e);
        let this = hull(&mut e).addr();
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x2c, 0x0900_002c);
        e.mem.set_u32(this, table);
        e.register_double(0x0900_002c, move |_, _| ret(6));
        let base = e.mem.alloc(0x400);
        e.mem.set_u32(router, base);
        let planes = e.mem.alloc(0x20);
        store_vec(&mut e, planes, [1.0, 0.0, 0.0, 5.0]);
        store_vec(&mut e, planes + 0x10, [0.0, 0.0, 1.0, -2.0]);
        e.mem.set_u32(this + 0x54, planes);
        e.mem.set_u32(this + 0x58, 2);
        e.mem.set_u32(this + 0x5c, 0x8000_0002);
        // A quarter turn about z followed by the translation (1, 2, 3).
        let matrix = e.mem.alloc(0x40);
        store_vec(&mut e, matrix, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, matrix + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, matrix + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, matrix + 0x30, [1.0, 2.0, 3.0, 1.0]);
        e.call(0x00d5_9cc0, &args![this, matrix]);
        let expected = [
            (1.0, 2.0, 3.0),
            (1.0, 3.0, 3.0),
            (0.0, 2.0, 3.0),
            (1.0, 2.0, 4.0),
            (-2.0, 4.0, 7.0),
            (3.0, 1.0, 0.0),
            (3.0, 1.0, 0.0),
            (3.0, 1.0, 0.0),
        ];
        for (index, (x, y, z)) in expected.into_iter().enumerate() {
            assert_eq!(vertex_bits(&e, this, index as u32), bits3(x, y, z));
        }
        // The vertices were gathered in the stack block and handed to calcAabb.
        let aabb: Vec<_> = events
            .borrow()
            .iter()
            .filter(|(tag, _)| *tag == "aabb")
            .map(|(_, words)| words[..3].to_vec())
            .collect();
        assert_eq!(aabb, vec![vec![base, 6, 0x10]]);
        assert_eq!(load_vec(&e, planes), [0.0, 1.0, 0.0, 3.0]);
        assert_eq!(load_vec(&e, planes + 0x10), [0.0, 0.0, 1.0, -5.0]);
        // setRotatedDir got the plane twice and a copy of the first three rows.
        let rotations: Vec<_> = events
            .borrow()
            .iter()
            .filter(|(tag, _)| *tag == "rotate")
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(rotations.len(), 2);
        assert_eq!(rotations[0][0], planes);
        assert_eq!(rotations[0][2], planes);
        assert_eq!(rotations[1][0], planes + 0x10);
        assert_ne!(rotations[0][1], matrix);
        // The stack allocator is back where it started and nothing was freed.
        assert_eq!(e.mem.u32(router), base);
        assert!(events.borrow().iter().all(|(tag, _)| *tag != "free"));
    }
}
