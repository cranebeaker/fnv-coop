//! `Havok SDK/hkgeometryutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SSE code computes lane by lane in `f32`; x87 code works in double precision (the 53-bit
//! control word of the process) and rounds at each `float` store; `RSQRTSS` is the host CPU's
//! estimate (refined once, as the exe does).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdebugdisplay::{add_lanes, cross_rows, mul_lanes, rotate_vector};
use super::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, store_vec, sum3, zip_with, Vec4, HALF_F32,
    ZERO_F32,
};

/// The memory router (`00c85750`): its field `+0x10` is the allocator every block comes from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkQuaternion::setAxisAngle` (`00cb2450`, `this` = the quaternion, `(axis pointer, angle
/// float)`), `hkVector4::setTransformedPos` (`00c7f640`, `this` = the result, `(transform,
/// point)`; the transform's translation row is `+0x30`), `hkArrayUtil::_reserve` (`00c908c0`,
/// cdecl `(allocator, array, count, element size)`) and the array's grow-by-one (`00c90940`,
/// cdecl `(allocator, array, element size)`).
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
const TRANSFORM_POINT: u32 = 0x00c7_f640;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW_BY_ONE: u32 = 0x00c9_0940;
/// The vector the capsule axis is compared with (`010c71a0`: the z axis), the double below which
/// the cosine must be for the frame to be built from it (`010cbf70`), pi and one half as doubles
/// (`0101ff40`, `01011588`), and the floats one (`01012068`) and minus one (`01012054`).
const AXIS_REFERENCE: u32 = 0x010c_71a0;
const PARALLEL_LIMIT: u32 = 0x010c_bf70;
const PI_DOUBLE: u32 = 0x0101_ff40;
const HALF_DOUBLE: u32 = 0x0101_1588;
const ONE_F32: u32 = 0x0101_2068;
const MINUS_ONE_F32: u32 = 0x0101_2054;
/// The `hkGeometry` the capsule builder fills: its vertices (`hkArray<hkVector4>`: data,
/// size, capacity word) at `+0` and its triangles (`hkArray<Triangle>`, 16-byte elements: three
/// vertex indices and a material of `-1`) at `+0xc`; 0x18 bytes.
const GEOMETRY_SIZE: u32 = 0x18;
const GEOMETRY_TRIANGLES: u32 = 0xc;
/// The capacity word of an array that owns no storage.
const NO_STORAGE: u32 = 0x8000_0000;

/// The allocator of the memory router.
fn allocator_of_router(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Appends the four words as one 16-byte element: asks the router first (the exe's order), grows
/// the array by one when its size equals its capacity (the capacity word without its flag
/// bits), then stores the element and counts it.
fn push_element(e: &mut Engine, array: u32, words: [u32; 4]) {
    let allocator = allocator_of_router(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW_BY_ONE, &args![allocator, array, 0x10u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let at = data.wrapping_add(size.wrapping_mul(0x10));
    for (lane, word) in words.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * lane as u32, word);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00da16d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryUtils::createCapsuleGeometry` (Xbox PDB), cdecl, seven stack arguments: the
/// bottom point `a` and top point `b` (pointers to vectors), the `radius`, the number of
/// `rings` of the cylinder, the number of `sides` (the hemispheres have `sides >> 1` rings
/// each), the `transform` every vertex is put through (`00c7f640`) and `out`, where the new
/// `hkGeometry` pointer is written.
///
/// The frame is built from `d = b - a`: with `L` its length (`SQRTSS`), when `L > 0` (ordered)
/// and `|z . d/L|` (the reference vector `010c71a0`, refined reciprocal root) is below the
/// double `010cbf70` the second axis is the normalised `z x d/L`, the first `d/L x` that and
/// the third `d/L`; otherwise the frame is the identity. The centre is `0.5 * (a + b)`. The
/// local points are the poles `(0, 0, +-(radius + L/2))` (the x87 sum in double precision,
/// rounded at the store) and rings of `sides` points each: on the top hemisphere a first
/// quaternion about `(0, -1, 0)` turns the x axis by `(i / half) * pi / 2` (`i` from
/// `half - 1` down to 0), then one about the z axis by `2 pi j / sides` rotates it around
/// (`rotate_vector`: `2 * (qw * (v x q) + ((qw * qw - 0.5) * v + (v . q) * q))`) and the point
/// is `radius * rotated + (0, 0, L/2)`; the cylinder's `rings - 1` inner rings (fraction `k /
/// rings` from the bottom) add `radius * rotated x axis` to `bottom + fraction * (top -
/// bottom)`; the bottom hemisphere turns by `-(i / half) * pi / 2` for `i` from 0 up. Every
/// point goes through the frame (`00c7f640` in place) into an array that `_reserve` sizes
/// first. A new 0x18-byte `hkGeometry` (from the router's allocator, slot `4`; a null block
/// faults at the first read, as it does in the exe) gets a copy of the points (a block from
/// slot `0xc`, which sizes it in place; the old one goes back through slot `0x10`), each
/// transformed by `transform`; its triangles are the top fan `(0, j, j % sides + 1)`, the
/// strips `(i, i + sides, c)` and `(c - sides, i, c)` of the quads between rings and the
/// bottom fan; the temporary points go back to the allocator.
#[allow(clippy::too_many_arguments)]
pub fn hk_geometry_utils_create_capsule_geometry(
    e: &mut Engine,
    a: u32,
    b: u32,
    radius: f32,
    rings: i32,
    sides: i32,
    transform: u32,
    out: u32,
) {
    e.with_stack(0x1b0, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let points = frame + 0x1c;
        e.mem.set_u32(points, 0);
        e.mem.set_u32(points + 4, 0);
        e.mem.set_u32(points + 8, NO_STORAGE);
        let zero: f32 = e.global(ZERO_F32);
        let bottom_point = load_vec(e, a);
        let top_point = load_vec(e, b);
        let direction = zip_with(top_point, bottom_point, |x, y| x - y);
        let squared = mul_lanes(direction, direction);
        let length = sum3(squared).sqrt();
        let half_sides = sides >> 1;
        // The frame: the three axes, then the centre.
        let identity: [Vec4; 3] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        let [mut first, mut second, mut third] = identity;
        if x87_quiet(length) > 0.0 {
            let inverse = inverse_length(e, sum3(squared));
            let unit = direction.map(|v| inverse * v);
            let reference = load_vec(e, AXIS_REFERENCE);
            let cosine = sum3(mul_lanes(reference, unit));
            let limit: f64 = e.global(PARALLEL_LIMIT);
            if f64::from(x87_quiet(cosine).abs()) < limit {
                let across = cross_rows(reference, unit);
                let across_inverse = inverse_length(e, sum3(mul_lanes(across, across)));
                second = across.map(|v| across_inverse * v);
                first = cross_rows(unit, second);
                third = unit;
            }
        }
        let half: f32 = e.global(HALF_F32);
        let centre = mul_lanes([half; 4], add_lanes(top_point, bottom_point));
        for (index, axis) in [first, second, third, centre].into_iter().enumerate() {
            store_vec(e, frame + 0xb0 + 0x10 * index as u32, axis);
        }
        // The x87 scalars: half the length as a double, and its float roundings.
        let length_double = f64::from(x87_quiet(length));
        let half_double: f64 = e.global(HALF_DOUBLE);
        let negative_half_length = ((-length_double) * half_double) as f32;
        let half_length_double = length_double * half_double;
        let half_length = half_length_double as f32;
        let bottom_centre = [zero, zero, negative_half_length, zero];
        let top_centre = [zero, zero, half_length, zero];
        let axis_length = zip_with(top_centre, bottom_centre, |x, y| x - y);
        let one: f32 = e.global(ONE_F32);
        let x_axis = [one, zero, zero, zero];
        let z_axis_at = frame + 0x80;
        store_vec(e, z_axis_at, [zero, zero, one, zero]);
        let y_axis_at = frame + 0xf0;
        let minus_one: f32 = e.global(MINUS_ONE_F32);
        store_vec(e, y_axis_at, [zero, minus_one, zero, zero]);
        // Room for every point.
        let wanted = rings
            .wrapping_add(half_sides.wrapping_mul(2))
            .wrapping_sub(1)
            .wrapping_mul(sides)
            .wrapping_add(2);
        let allocator = allocator_of_router(e);
        if ((e.mem.u32(points + 8) & 0x3fff_ffff) as i32) < wanted {
            e.call(
                ARRAY_RESERVE,
                &args![allocator, points, wanted as u32, 0x10u32],
            );
        }
        let radius_float = x87_quiet(radius);
        let pi: f64 = e.global(PI_DOUBLE);
        let sides_float = sides as f32;
        let half_float = half_sides as f32;
        let scratch_at = frame + 0x30;
        let transform_at = frame + 0xb0;
        // One point: through the frame in place, then appended.
        let emit = |e: &mut Engine, local: Vec4| {
            store_vec(e, scratch_at, local);
            e.call(
                TRANSFORM_POINT,
                &args![scratch_at, transform_at, scratch_at],
            );
            let words = load_bits(e, scratch_at);
            push_element(e, points, words);
        };
        // The quaternion about `axis_at` by an angle, rounded to float as the exe passes it.
        let turn = |e: &mut Engine, result_at: u32, axis_at: u32, angle: f64| -> Vec4 {
            e.call(SET_AXIS_ANGLE, &args![result_at, axis_at, angle as f32]);
            load_vec(e, result_at)
        };
        // The angle of side `j`: `2 pi j / sides`.
        let side_angle = |j: i32| {
            let turn = (f64::from(j) / f64::from(sides_float)) * pi;
            turn + turn
        };
        // The top pole.
        let apex = f64::from(radius_float) + half_length_double;
        emit(e, [zero, zero, apex as f32, zero]);
        // The top hemisphere, from the ring nearest the pole down to the equator.
        let mut ring = half_sides.wrapping_sub(1);
        while ring >= 0 {
            let angle = ((f64::from(ring) / f64::from(half_float)) * pi) * half_double;
            let q = turn(e, frame + 0x150, y_axis_at, angle);
            let ring_vector = rotate_vector(e, q, x_axis);
            if sides > 0 {
                for side in 0..sides {
                    let q = turn(e, frame + 0x140, z_axis_at, side_angle(side));
                    let rotated = rotate_vector(e, q, ring_vector);
                    let local = add_lanes(rotated.map(|v| radius_float * v), top_centre);
                    emit(e, local);
                }
            }
            ring -= 1;
        }
        // The cylinder's inner rings.
        let mut step = rings.wrapping_sub(1);
        while step > 0 {
            if sides > 0 {
                let fraction = (f64::from(step) / f64::from(rings)) as f32;
                for side in 0..sides {
                    let q = turn(e, frame + 0xa0, z_axis_at, side_angle(side));
                    let rotated = rotate_vector(e, q, x_axis);
                    let base = add_lanes(axis_length.map(|v| fraction * v), bottom_centre);
                    let local = add_lanes(base, rotated.map(|v| radius_float * v));
                    emit(e, local);
                }
            }
            step -= 1;
        }
        // The bottom hemisphere, from the equator down.
        let mut ring = 0;
        while ring < half_sides {
            let angle = ((-f64::from(ring) / f64::from(half_float)) * pi) * half_double;
            let q = turn(e, frame + 0x90, y_axis_at, angle);
            let ring_vector = rotate_vector(e, q, x_axis);
            if sides > 0 {
                for side in 0..sides {
                    let q = turn(e, frame + 0xa0, z_axis_at, side_angle(side));
                    let rotated = rotate_vector(e, q, ring_vector);
                    let local = add_lanes(rotated.map(|v| radius_float * v), bottom_centre);
                    emit(e, local);
                }
            }
            ring += 1;
        }
        // The bottom pole.
        emit(e, [zero, zero, (-apex) as f32, zero]);
        // The geometry.
        let allocator = allocator_of_router(e);
        let geometry = e.vcall(allocator, 4, &args![GEOMETRY_SIZE]).u32();
        if geometry != 0 {
            for (offset, word) in [
                (0u32, 0u32),
                (4, 0),
                (8, NO_STORAGE),
                (0xc, 0),
                (0x10, 0),
                (0x14, NO_STORAGE),
            ] {
                e.mem.set_u32(geometry + offset, word);
            }
        }
        let allocator = allocator_of_router(e);
        let count = e.mem.u32(points + 4) as i32;
        let capacity_word = e.mem.u32(geometry + 8);
        if ((capacity_word & 0x3fff_ffff) as i32) < count {
            if (capacity_word as i32) >= 0 {
                let old = e.mem.u32(geometry);
                e.vcall(
                    allocator,
                    0x10,
                    &args![old, (capacity_word & 0x3fff_ffff) << 4],
                );
            }
            let bytes_at = frame + 0x10;
            e.mem.set_u32(bytes_at, (count as u32) << 4);
            let block = e.vcall(allocator, 0xc, &args![bytes_at]).u32();
            e.mem.set_u32(geometry, block);
            let granted = e.mem.u32(bytes_at) as i32;
            let rounded = granted.wrapping_add((granted >> 31) & 0xf);
            e.mem.set_u32(geometry + 8, (rounded >> 4) as u32);
        }
        let vertices = e.mem.u32(geometry);
        e.mem.set_u32(geometry + 4, count as u32);
        let source = e.mem.u32(points);
        for index in 0..count.max(0) as u32 {
            copy_vec(e, source + 0x10 * index, vertices + 0x10 * index);
        }
        let stored = e.mem.u32(geometry + 4) as i32;
        for index in 0..stored.max(0) as u32 {
            let vertex = e.mem.u32(geometry) + 0x10 * index;
            e.call(TRANSFORM_POINT, &args![vertex, transform, vertex]);
        }
        // The triangles.
        let triangles = geometry + GEOMETRY_TRIANGLES;
        let triangle = |e: &mut Engine, x: i32, y: i32, z: i32| {
            push_element(e, triangles, [x as u32, y as u32, z as u32, u32::MAX]);
        };
        if sides > 0 {
            for j in 1..=sides {
                triangle(e, 0, j, j % sides + 1);
            }
        }
        let mut base = 1i32;
        let strip_rows = rings
            .wrapping_add(half_sides.wrapping_mul(2))
            .wrapping_sub(2);
        let mut rows_left = strip_rows;
        while rows_left > 0 {
            if sides > 0 {
                for j in 1..=sides {
                    let low = base + j - 1;
                    let next = base + j % sides;
                    let upper_next = next + sides;
                    triangle(e, low, low + sides, upper_next);
                    triangle(e, next, low, upper_next);
                }
            }
            base = base.wrapping_add(sides);
            rows_left -= 1;
        }
        if sides > 0 {
            for j in 1..=sides {
                let low = j + base - 1;
                triangle(e, base + j % sides, low, base + sides);
            }
        }
        e.mem.set_u32(out, geometry);
        // The temporary points go back (their storage is owned unless the flag bit is set).
        let allocator = allocator_of_router(e);
        let capacity_word = e.mem.u32(points + 8);
        e.mem.set_u32(points + 4, 0);
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(points);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity_word & 0x3fff_ffff) << 4],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00da_16d0,
        hk_geometry_utils_create_capsule_geometry(u32, u32, f32, i32, i32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The allocator method doubles: slot `s` is the function at `ALLOCATOR + s`.
    const ALLOCATOR: u32 = 0x4c00_0000;
    /// The vector of `-0.5` the rotation adds (`010c71e0`).
    const ROTATION_BIAS: u32 = 0x010c_71e0;
    /// The float of three the refined reciprocal root uses (`01017718`).
    const THREE: u32 = 0x0101_7718;

    fn aligned(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with the constants, a router and real doubles for the quaternion, the point
    /// transform and the array routines. `reserve` says whether `_reserve` really reserves.
    fn capsule_engine(reserve: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(ONE_F32, 1.0f32);
        e.set_global(MINUS_ONE_F32, -1.0f32);
        e.set_global(PARALLEL_LIMIT, 0.99999f64);
        e.set_global(PI_DOUBLE, std::f64::consts::PI);
        e.set_global(HALF_DOUBLE, 0.5f64);
        for lane in 0..4 {
            e.set_global(ROTATION_BIAS + 4 * lane, -0.5f32);
        }
        store_vec(&mut e, AXIS_REFERENCE, [0.0, 0.0, 1.0, 0.0]);
        let vtable = e.mem.alloc(0x20);
        for slot in [4u32, 0xc, 0x10] {
            e.mem.set_u32(vtable + slot, ALLOCATOR + slot);
        }
        e.register(ALLOCATOR + 4, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(ALLOCATOR + 0xc, |e, a| {
            let bytes = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(bytes),
                ..Ret::default()
            }
        });
        e.register(ALLOCATOR + 0x10, |_, _| Ret::default());
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // `_reserve` gives the array room for `count` elements; growth doubles the capacity.
        fn make_room(e: &mut Engine, array: u32, capacity: u32) {
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let block = e.mem.alloc(capacity * 0x10);
            for word in 0..size * 4 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(block + 4 * word, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
        }
        e.register_double(ARRAY_RESERVE, move |e, a| {
            if reserve {
                make_room(e, a[1], a[2]);
            }
            Ret::default()
        });
        e.register_double(ARRAY_GROW_BY_ONE, |e, a| {
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff).max(2) * 2;
            make_room(e, a[1], capacity);
            Ret::default()
        });
        // `hkQuaternion::setAxisAngle`.
        e.register(SET_AXIS_ANGLE, |e, a| {
            let axis = load_vec(e, a[1]);
            let half = f32::from_bits(a[2]) * 0.5;
            let (sine, cosine) = half.sin_cos();
            store_vec(
                e,
                a[0],
                [axis[0] * sine, axis[1] * sine, axis[2] * sine, cosine],
            );
            Ret::default()
        });
        // `hkVector4::setTransformedPos`: `x * row0 + y * row1 + z * row2 + row3`.
        e.register(TRANSFORM_POINT, |e, a| {
            let point = load_vec(e, a[2]);
            let rows: [Vec4; 4] = std::array::from_fn(|r| load_vec(e, a[1] + 0x10 * r as u32));
            let moved: Vec4 = std::array::from_fn(|lane| {
                point[0] * rows[0][lane]
                    + point[1] * rows[1][lane]
                    + point[2] * rows[2][lane]
                    + rows[3][lane]
            });
            store_vec(e, a[0], moved);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, allocator)
    }

    fn identity_transform(e: &mut Engine, translation: [f32; 3]) -> u32 {
        let at = aligned(e, 0x40);
        let rows = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [translation[0], translation[1], translation[2], 1.0],
        ];
        for (row, value) in rows.into_iter().enumerate() {
            store_vec(e, at + 0x10 * row as u32, value);
        }
        at
    }

    struct Capsule {
        vertices: Vec<[f32; 4]>,
        triangles: Vec<[i32; 4]>,
    }

    fn read_capsule(e: &Engine, geometry: u32) -> Capsule {
        let count = e.mem.u32(geometry + 4);
        let data = e.mem.u32(geometry);
        let triangle_count = e.mem.u32(geometry + 0x10);
        let triangle_data = e.mem.u32(geometry + 0xc);
        Capsule {
            vertices: (0..count).map(|i| load_vec(e, data + 0x10 * i)).collect(),
            triangles: (0..triangle_count)
                .map(|i| {
                    std::array::from_fn(|lane| {
                        e.mem.u32(triangle_data + 0x10 * i + 4 * lane as u32) as i32
                    })
                })
                .collect(),
        }
    }

    /// Builds a capsule between `a` and `b` and returns the engine, the allocator and the result.
    fn build(
        ends: ([f32; 3], [f32; 3]),
        radius: f32,
        rings: i32,
        sides: i32,
        translation: [f32; 3],
        reserve: bool,
    ) -> (Engine, u32, Capsule) {
        let (a, b) = ends;
        let (mut e, allocator) = capsule_engine(reserve);
        let bottom = aligned(&mut e, 0x10);
        let top = aligned(&mut e, 0x10);
        store_vec(&mut e, bottom, [a[0], a[1], a[2], 0.0]);
        store_vec(&mut e, top, [b[0], b[1], b[2], 0.0]);
        let transform = identity_transform(&mut e, translation);
        let out = e.mem.alloc(4);
        e.call(
            0x00da_16d0,
            &args![bottom, top, radius, rings, sides, transform, out],
        );
        let geometry = e.mem.u32(out);
        let capsule = read_capsule(&e, geometry);
        (e, allocator, capsule)
    }

    /// The distance from `p` to the segment `a`-`b`, in double precision.
    fn segment_distance(p: [f32; 4], a: [f32; 3], b: [f32; 3]) -> f64 {
        let (a, b) = (a.map(f64::from), b.map(f64::from));
        let p = [f64::from(p[0]), f64::from(p[1]), f64::from(p[2])];
        let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let ap = [p[0] - a[0], p[1] - a[1], p[2] - a[2]];
        let length_squared = ab[0] * ab[0] + ab[1] * ab[1] + ab[2] * ab[2];
        let t = if length_squared == 0.0 {
            0.0
        } else {
            ((ap[0] * ab[0] + ap[1] * ab[1] + ap[2] * ab[2]) / length_squared).clamp(0.0, 1.0)
        };
        let nearest = [a[0] + t * ab[0], a[1] + t * ab[1], a[2] + t * ab[2]];
        let offset = [p[0] - nearest[0], p[1] - nearest[1], p[2] - nearest[2]];
        (offset[0] * offset[0] + offset[1] * offset[1] + offset[2] * offset[2]).sqrt()
    }

    fn check_surface(capsule: &Capsule, ends: ([f32; 3], [f32; 3]), radius: f32) {
        for (index, vertex) in capsule.vertices.iter().enumerate() {
            let distance = segment_distance(*vertex, ends.0, ends.1);
            assert!(
                (distance - f64::from(radius)).abs() < 2e-3,
                "vertex {index} {vertex:?} is {distance} from the axis"
            );
        }
    }

    fn check_indices(capsule: &Capsule) {
        for triangle in &capsule.triangles {
            for index in &triangle[..3] {
                assert!(
                    *index >= 0 && (*index as usize) < capsule.vertices.len(),
                    "{triangle:?}"
                );
            }
            assert_eq!(triangle[3], -1);
        }
    }

    fn near(got: [f32; 4], want: [f32; 3]) {
        for lane in 0..3 {
            assert!(
                (got[lane] - want[lane]).abs() < 2e-3,
                "lane {lane}: {got:?} against {want:?}"
            );
        }
    }

    const VERTICAL: ([f32; 3], [f32; 3]) = ([0.0; 3], [0.0, 0.0, 4.0]);

    #[test]
    fn a_vertical_capsule_has_its_poles_rings_and_triangles() {
        let (_, _, capsule) = build(VERTICAL, 1.0, 2, 8, [0.0; 3], true);
        assert_eq!(capsule.vertices.len(), 74);
        assert_eq!(capsule.triangles.len(), 144);
        near(capsule.vertices[0], [0.0, 0.0, 5.0]);
        near(capsule.vertices[73], [0.0, 0.0, -1.0]);
        // The first ring is at 67.5 degrees above the top centre.
        let angle = 3.0f32 / 4.0 * std::f32::consts::FRAC_PI_2;
        near(capsule.vertices[1], [angle.cos(), 0.0, 4.0 + angle.sin()]);
        check_surface(&capsule, VERTICAL, 1.0);
        check_indices(&capsule);
        assert_eq!(capsule.triangles[0], [0, 1, 2, -1]);
        assert_eq!(capsule.triangles[7], [0, 8, 1, -1]);
        assert_eq!(capsule.triangles[143], [65, 72, 73, -1]);
        // The first strip quad: the two triangles between the first and second rings.
        assert_eq!(capsule.triangles[8], [1, 9, 10, -1]);
        assert_eq!(capsule.triangles[9], [2, 1, 10, -1]);
    }

    #[test]
    fn a_tilted_capsule_lies_on_the_surface_between_its_end_points() {
        let ends = ([1.0, 2.0, 3.0], [1.0, 6.0, 3.0]);
        let (_, _, capsule) = build(ends, 0.75, 3, 6, [0.0; 3], true);
        assert_eq!(capsule.vertices.len(), 50);
        near(capsule.vertices[0], [1.0, 6.75, 3.0]);
        near(capsule.vertices[49], [1.0, 1.25, 3.0]);
        check_surface(&capsule, ends, 0.75);
        check_indices(&capsule);
    }

    #[test]
    fn a_general_direction_builds_the_frame_from_the_cross_product() {
        let ends = ([0.0, 0.0, 0.0], [3.0, 0.0, 4.0]);
        let (_, _, capsule) = build(ends, 1.0, 2, 8, [0.0; 3], true);
        check_surface(&capsule, ends, 1.0);
        // The top pole is the end point pushed out along the axis.
        near(capsule.vertices[0], [3.6, 0.0, 4.8]);
        near(
            capsule.vertices[capsule.vertices.len() - 1],
            [-0.6, 0.0, -0.8],
        );
    }

    #[test]
    fn a_zero_length_capsule_is_a_sphere_with_the_identity_frame() {
        let point = [1.0, 1.0, 1.0];
        let (_, _, capsule) = build((point, point), 2.0, 1, 8, [0.0; 3], true);
        near(capsule.vertices[0], [1.0, 1.0, 3.0]);
        for vertex in &capsule.vertices {
            assert!((segment_distance(*vertex, point, point) - 2.0).abs() < 2e-3);
        }
        check_indices(&capsule);
    }

    #[test]
    fn an_axis_nearly_parallel_to_the_reference_keeps_the_identity_frame() {
        // The cosine of an axis 0.01 off the z axis is above 0.99999: the frame stays the
        // identity, so the top pole is straight above the centre rather than along the axis.
        let ends = ([0.0; 3], [0.01, 0.0, 4.0]);
        let (_, _, capsule) = build(ends, 1.0, 2, 8, [0.0; 3], true);
        near(capsule.vertices[0], [0.005, 0.0, 5.000_006]);
    }

    #[test]
    fn the_transform_moves_every_vertex() {
        let (_, _, plain) = build(VERTICAL, 1.0, 2, 8, [0.0; 3], true);
        let (_, _, moved) = build(VERTICAL, 1.0, 2, 8, [10.0, -1.0, 2.0], true);
        assert_eq!(plain.vertices.len(), moved.vertices.len());
        for (a, b) in plain.vertices.iter().zip(&moved.vertices) {
            assert_eq!(*b, [a[0] + 10.0, a[1] - 1.0, a[2] + 2.0, 1.0]);
        }
    }

    #[test]
    fn no_sides_leaves_just_the_two_poles() {
        let (_, _, capsule) = build(VERTICAL, 1.0, 2, 0, [0.0; 3], true);
        assert_eq!(capsule.vertices.len(), 2);
        assert!(capsule.triangles.is_empty());
        near(capsule.vertices[0], [0.0, 0.0, 5.0]);
        near(capsule.vertices[1], [0.0, 0.0, -1.0]);
    }

    #[test]
    fn the_points_array_is_reserved_first_and_freed_last() {
        let (e, allocator, _) = build(VERTICAL, 1.0, 2, 8, [0.0; 3], true);
        let reserve = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(
            [reserve[0][0], reserve[0][2], reserve[0][3]],
            [allocator, 74, 0x10]
        );
        // Room was made, so the points never needed to grow; the triangles did.
        let grows = calls_to(&e, ARRAY_GROW_BY_ONE);
        assert!(grows.iter().all(|call| call[1] != reserve[0][1]));
        assert!(!grows.is_empty());
        // The geometry's vertex block comes from slot 0xc and the temporary points go back.
        assert_eq!(calls_to(&e, ALLOCATOR + 0xc).len(), 1);
        let freed = calls_to(&e, ALLOCATOR + 0x10);
        assert_eq!(freed.last().map(|call| call[2]), Some(74 * 16));
        assert_eq!(calls_to(&e, ALLOCATOR + 4), vec![vec![allocator, 0x18]]);
    }

    #[test]
    fn growing_one_at_a_time_gives_the_same_geometry() {
        let ends = ([0.0; 3], [2.0, 1.0, 4.0]);
        let (_, _, reserved) = build(ends, 1.0, 2, 6, [0.0; 3], true);
        let (e, _, grown) = build(ends, 1.0, 2, 6, [0.0; 3], false);
        assert_eq!(reserved.vertices, grown.vertices);
        assert_eq!(reserved.triangles, grown.triangles);
        assert!(calls_to(&e, ARRAY_GROW_BY_ONE).len() > 4);
    }

    #[test]
    fn the_counts_follow_the_ring_and_side_counts() {
        for (rings, sides) in [(1, 4), (3, 5), (2, 7)] {
            let ends = ([0.0; 3], [0.0, 0.0, 3.0]);
            let (_, _, capsule) = build(ends, 0.5, rings, sides, [0.0; 3], true);
            let half = sides >> 1;
            let strip_rows = rings + 2 * half - 2;
            assert_eq!(
                capsule.vertices.len() as i32,
                (rings + 2 * half - 1) * sides + 2
            );
            assert_eq!(
                capsule.triangles.len() as i32,
                2 * sides + 2 * strip_rows * sides
            );
            check_indices(&capsule);
        }
    }

    #[test]
    fn the_result_pointer_receives_the_geometry() {
        let (mut e, _) = capsule_engine(true);
        let bottom = aligned(&mut e, 0x10);
        let top = aligned(&mut e, 0x10);
        store_vec(&mut e, top, [0.0, 0.0, 1.0, 0.0]);
        let transform = identity_transform(&mut e, [0.0; 3]);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        e.call(
            0x00da_16d0,
            &args![bottom, top, 0.5f32, 1, 4, transform, out],
        );
        let geometry = e.mem.u32(out);
        assert_ne!(geometry, 0x1234);
        assert_eq!(e.mem.u32(geometry + 4), 18);
    }
}
