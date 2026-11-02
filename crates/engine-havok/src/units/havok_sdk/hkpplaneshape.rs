//! `Havok SDK/hkpplaneshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{
    load_bits, load_vec, profile_record, store_vec, zip_with, Vec4, PROFILE_END,
};

// `hkpPlaneShape` (the vtable `010d0e18` is that class by its RTTI): reference count word at
// +6, shape type 0x1C at +0xC, the plane equation (normal in lanes 0 to 2, offset in lane 3) at
// +0x10, the centre of the shape's box at +0x20 and its half extents at +0x30; 0x40 bytes.
const PLANE_SHAPE_VTABLE: u32 = 0x010d_0e18;
const PLANE: u32 = 0x10;
const CENTRE: u32 = 0x20;
const HALF_EXTENTS: u32 = 0x30;
/// The profiling record label `castRayImpl` starts with (the label it ends with is
/// `PROFILE_END`).
const PROFILE_BEGIN_CAST_RAY: u32 = 0x010d_0e4c;
/// The class record `calcContentStatistics` passes on.
const STATISTICS_CLASS: u32 = 0x0126_9b84;
/// `hkReferencedObject::calcContentStatistics` (Xbox PDB, `this`, two words).
const REFERENCED_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// `hkVector4::setMul3` (Xbox PDB): `this`, one vector.
const VECTOR_SET_MUL3: u32 = 0x00c7_f750;
/// `hkAabbUtil::calcAabb` (cdecl): the transform, half extents, centre, extra radius (float
/// bits) and the output box.
const CALC_AABB: u32 = 0x00d2_6230;
/// Four lanes of `1.0`, the lane-wise sign mask (`0x7fffffff`) and four lanes of `0.5`.
const ONES: u32 = 0x010e_c7f0;
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
const HALF_VECTOR: u32 = 0x010c_71d0;

/// The distance sum the code forms from `first * second` lane by lane (the order of the
/// multiplication is the exe's): `(p.z + plane_offset) + (p.y + p.x)`.
fn plane_distance(first: Vec4, second: Vec4, plane_offset: f32) -> f32 {
    let p = zip_with(first, second, |a, b| a * b);
    (p[2] + plane_offset) + (p[1] + p[0])
}

/// Whether `(1 - t) * from + to * t` lies inside the shape's box: `|point - centre| <= half
/// extents` in lanes 0 to 2 (a NaN lane fails).
fn point_inside_box(e: &Engine, shape: u32, from: Vec4, to: Vec4, t: f32) -> bool {
    let ones = load_vec(e, ONES);
    let splat = [t; 4];
    let blended = zip_with(zip_with(ones, splat, |a, b| a - b), from, |a, b| a * b);
    let moved = zip_with(to, splat, |a, b| a * b);
    let point = zip_with(blended, moved, |a, b| a + b);
    let offset = zip_with(point, load_vec(e, shape + CENTRE), |a, b| a - b);
    let mask = load_bits(e, ABSOLUTE_MASK);
    let half = load_vec(e, shape + HALF_EXTENTS);
    (0..3).all(|lane| f32::from_bits(offset[lane].to_bits() & mask[lane]) <= half[lane])
}

// Translated from 00d54f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPlaneShape::calcContentStatistics` (Xbox PDB): tail-jumps to
/// `hkReferencedObject::calcContentStatistics` with `this`, the first word and, in place of
/// the second word, the class record at `01269b84`. Returns that function's result.
pub fn plane_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused_2: u32,
) -> u32 {
    e.call(
        REFERENCED_CALC_CONTENT_STATISTICS,
        &args![this, collector, STATISTICS_CLASS],
    )
    .u32()
}

// Translated from 00d54f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPlaneShape::castRayWithCollectorImpl` (Xbox PDB): builds a ray cast output (hit
/// fraction = the collector's early-out fraction at `collector + 4`, extra info -1, first
/// shape key -1, shape key index 0), calls the shape's virtual at +0x20 (`castRayImpl`) with
/// a result byte, the input and the output; when it reports a hit, transforms the output with
/// the body's motion (`cdBody + 8`, `hkVector4::setMul3`) and hands it to the collector's
/// virtual at +0 with the body.
pub fn plane_shape_cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr,
    input: u32,
    body: u32,
    collector: u32,
) {
    let early_out = x87_quiet(e.mem.f32(collector + 4));
    e.with_stack(0x50, |e, output| {
        let out = output.addr();
        e.mem.set_f32(out + 0x10, early_out);
        e.mem.set_u32(out + 0x14, u32::MAX);
        e.mem.set_u32(out + 0x20, u32::MAX);
        e.mem.set_u32(out + 0x40, 0);
        let hit = e.with_stack(4, |e, flag| {
            e.vcall(this.addr(), 0x20, &args![flag, input, output]);
            e.mem.u8(flag.addr())
        });
        if hit != 0 {
            let motion = e.mem.u32(body + 8);
            // 00d54fe0: `hkVector4::setMul3` (RET 8) on the output's normal (ECX, the same
            // address as the pushed vector): rotated in place by the body's motion.
            e.call(VECTOR_SET_MUL3, &args![output, motion, output]);
            e.vcall(collector, 0, &args![body, output]);
        }
    });
}

// Translated from 00d55000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPlaneShape::castSphere` (Xbox PDB): the sphere runs from `input` (`from` at +0, `to` at
/// +0x10) with the radius at +0x30. With the signed distances `a` of `from` and `b` of `to`
/// from the plane, less the radius (each rounded to a `float`), nothing happens unless `b < 0`
/// (or is a NaN) and the float at +0x34 is not above `a - b`. The hit fraction is `a / (a - b)`
/// (0 when `a <= 0`); when the hit point is inside the shape's box the output is the plane as
/// normal, that fraction, extra info -1, first shape key -1 and shape key index 0, passed to
/// the collector's virtual at +0 with the body.
pub fn plane_shape_cast_sphere(e: &mut Engine, this: Ptr, input: u32, body: u32, collector: u32) {
    let shape = this.addr();
    let plane = load_vec(e, shape + PLANE);
    let from = load_vec(e, input);
    let radius = x87_quiet(e.mem.f32(input + 0x30));
    let dist_from = plane_distance(plane, from, plane[3]);
    let to = load_vec(e, input + 0x10);
    let dist_to = plane_distance(plane, to, plane[3]);
    let a = (f64::from(dist_from) - f64::from(radius)) as f32;
    let b = (f64::from(dist_to) - f64::from(radius)) as f32;
    if b >= 0.0 {
        return;
    }
    let span = f64::from(a) - f64::from(b);
    let limit = f64::from(e.mem.f32(input + 0x34));
    if limit > span {
        return;
    }
    let fraction = if a <= 0.0 {
        0.0f32
    } else {
        (f64::from(a) / span) as f32
    };
    if !point_inside_box(e, shape, from, to, fraction) {
        return;
    }
    e.with_stack(0x50, |e, output| {
        let out = output.addr();
        store_vec(e, out, plane);
        e.mem.set_f32(out + 0x10, x87_quiet(fraction));
        e.mem.set_u32(out + 0x14, u32::MAX);
        e.mem.set_u32(out + 0x20, u32::MAX);
        e.mem.set_u32(out + 0x40, 0);
        e.vcall(collector, 0, &args![body, output]);
    });
}

// Translated from 00d55160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPlaneShape::collideSpheres` (Xbox PDB): `spheres` is `{ data, count }`; for each sphere
/// (16 bytes, centre in lanes 0 to 2, radius in lane 3) writes the plane vector to `output`
/// with lane 3 replaced by the sphere's distance from the plane less its radius (computed in
/// double precision, rounded to a `float`).
pub fn plane_shape_collide_spheres(e: &mut Engine, this: Ptr, spheres: u32, output: u32) {
    let shape = this.addr();
    let mut sphere = e.mem.u32(spheres);
    let mut remaining = e.mem.i32(spheres + 4).wrapping_sub(1);
    let mut out = output;
    if remaining >= 0 {
        loop {
            let plane = load_vec(e, shape + PLANE);
            store_vec(e, out, plane);
            let plane = load_vec(e, shape + PLANE);
            let centre = load_vec(e, sphere);
            let distance = plane_distance(centre, plane, plane[3]);
            let radius = e.mem.f32(sphere + 0xc);
            let result = (f64::from(distance) - f64::from(x87_quiet(radius))) as f32;
            out = out.wrapping_add(0x10);
            sphere = sphere.wrapping_add(0x10);
            remaining = remaining.wrapping_sub(1);
            e.mem.set_f32(out.wrapping_sub(4), result);
            if remaining < 0 {
                break;
            }
        }
    }
}

// Translated from 00d551d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor of `hkpPlaneShape` (the vtable it installs is that class's, by its RTTI),
/// from a plane vector and a box (`min` at +0, `max` at +0x10): clears +8, sets the type word
/// 0x1C at +0xC, the vtable, reference count 1 at +6, the plane vector at +0x10, the centre
/// `(max + min) * 0.5` at +0x20 and the half extents `(max - min) * 0.5` at +0x30. Returns
/// `this`.
pub fn fn_00d551d0(e: &mut Engine, this: Ptr, plane: u32, aabb: u32) -> Ptr {
    let shape = this.addr();
    e.mem.set_u32(shape + 8, 0);
    e.mem.set_u32(shape + 0xc, 0x1c);
    e.mem.set_u32(shape, PLANE_SHAPE_VTABLE);
    e.mem.set_u16(shape + 6, 1);
    let normal = load_vec(e, plane);
    store_vec(e, shape + PLANE, normal);
    let min = load_vec(e, aabb);
    let max = load_vec(e, aabb + 0x10);
    let half = load_vec(e, HALF_VECTOR);
    let sum = zip_with(max, min, |a, b| a + b);
    store_vec(e, shape + CENTRE, zip_with(sum, half, |a, b| a * b));
    let min = load_vec(e, aabb);
    let max = load_vec(e, aabb + 0x10);
    let half = load_vec(e, HALF_VECTOR);
    let difference = zip_with(max, min, |a, b| a - b);
    store_vec(
        e,
        shape + HALF_EXTENTS,
        zip_with(difference, half, |a, b| a * b),
    );
    this
}

// Translated from 00d55240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Another constructor of `hkpPlaneShape` (the vtable is that class's), from a normal, a
/// centre and half extents: the same header as `00d551d0`; the normal is copied to +0x10 and
/// its lane 3 (+0x1C) becomes minus the sum `(p.z + (p.y + p.x))` of `p = normal * centre`
/// (the plane offset); the centre goes to +0x20 and the half extents to +0x30. Returns `this`.
pub fn fn_00d55240(e: &mut Engine, this: Ptr, normal: u32, centre: u32, half_extents: u32) -> Ptr {
    let shape = this.addr();
    e.mem.set_u16(shape + 6, 1);
    e.mem.set_u32(shape + 8, 0);
    e.mem.set_u32(shape + 0xc, 0x1c);
    e.mem.set_u32(shape, PLANE_SHAPE_VTABLE);
    let vector = load_vec(e, normal);
    store_vec(e, shape + PLANE, vector);
    let vector = load_vec(e, normal);
    let centre_vector = load_vec(e, centre);
    let p = zip_with(vector, centre_vector, |a, b| a * b);
    let sum = p[2] + (p[1] + p[0]);
    e.mem.set_f32(shape + 0x1c, -x87_quiet(sum));
    let centre_vector = load_vec(e, centre);
    store_vec(e, shape + CENTRE, centre_vector);
    let half = load_vec(e, half_extents);
    store_vec(e, shape + HALF_EXTENTS, half);
    this
}

// Translated from 00d552c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPlaneShape::getAabbImpl` (Xbox PDB): `hkAabbUtil::calcAabb(transform, half extents
/// (+0x30), centre (+0x20), tolerance, output)`; the tolerance goes through the x87 stack on
/// the way (a signalling NaN is quieted).
pub fn plane_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: u32,
    tolerance: f32,
    output: u32,
) {
    let shape = this.addr();
    e.call(
        CALC_AABB,
        &args![
            transform,
            shape + HALF_EXTENTS,
            shape + CENTRE,
            x87_quiet(tolerance),
            output
        ],
    );
}

// Translated from 00d552f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPlaneShape::castRayImpl` (Xbox PDB): `result` is the `hkBool` the caller passes by
/// pointer (the function returns it). Profiling records bracket the work. With `d1` and `d2`
/// the signed distances of the ray's `from` and `to` from the plane, a hit needs `d1 >= 0` and
/// `d2 < 0`; its fraction is `d1 / (d1 - d2)` (double precision, rounded to a `float`) and
/// the hit point must be inside the shape's box. A hit writes the fraction to the output's
/// +0x10, the plane as normal at +0 and -1 into `shapeKeys[shapeKeyIndex]` (+0x20, index at
/// +0x40) and sets the byte to 1; otherwise it is set to 0.
pub fn plane_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    result: u32,
    input: u32,
    output: u32,
) -> u32 {
    let shape = this.addr();
    profile_record(e, PROFILE_BEGIN_CAST_RAY);
    let from = load_vec(e, input);
    let plane = load_vec(e, shape + PLANE);
    let d1 = plane_distance(from, plane, plane[3]);
    let plane = load_vec(e, shape + PLANE);
    let to = load_vec(e, input + 0x10);
    let d2 = plane_distance(plane, to, plane[3]);
    let mut hit = false;
    if d1 >= 0.0 && d2 < 0.0 {
        let fraction = (f64::from(d1) / (f64::from(d1) - f64::from(d2))) as f32;
        if point_inside_box(e, shape, from, to, fraction) {
            let index = e.mem.u32(output + 0x40);
            e.mem.set_f32(output + 0x10, x87_quiet(fraction));
            let plane = load_vec(e, shape + PLANE);
            store_vec(e, output, plane);
            e.mem.set_u32(
                output
                    .wrapping_add(index.wrapping_mul(4))
                    .wrapping_add(0x20),
                u32::MAX,
            );
            hit = true;
        }
    }
    e.mem.set_u8(result, u8::from(hit));
    profile_record(e, PROFILE_END);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_4f80, plane_shape_calc_content_statistics(Ptr, u32, u32) -> u32),
        entry!(
            0x00d5_4f90,
            plane_shape_cast_ray_with_collector_impl(Ptr, u32, u32, u32)
        ),
        entry!(0x00d5_5000, plane_shape_cast_sphere(Ptr, u32, u32, u32)),
        entry!(0x00d5_5160, plane_shape_collide_spheres(Ptr, u32, u32)),
        entry!(0x00d5_51d0, fn_00d551d0(Ptr, u32, u32) -> Ptr),
        entry!(0x00d5_5240, fn_00d55240(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00d5_52c0, plane_shape_get_aabb_impl(Ptr, u32, f32, u32)),
        entry!(0x00d5_52f0, plane_shape_cast_ray_impl(Ptr, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a05_plane_shape_tests {
    use super::super::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use super::*;

    /// An engine with the constants the plane shape reads mapped: ones, the sign mask and the
    /// halves, and the profiling stream.
    fn world() -> Engine {
        let mut e = Engine::new();
        for page in [0x010e_c000, 0x0101_7000, 0x010c_7000, 0x0126_8000] {
            e.map(page, 0x1000);
        }
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem.set_u32(ABSOLUTE_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_f32(HALF_VECTOR + 4 * lane, 0.5);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn set_vec(e: &mut Engine, address: u32, v: Vec4) {
        store_vec(e, address, v);
    }

    /// A plane shape for the plane `y = 0` (normal `(0, 1, 0)`, offset 0) with the box centred
    /// at the origin with half extents `(10, 10, 10)`.
    fn plane_shape(e: &mut Engine) -> u32 {
        let shape = e.mem.alloc(0x40);
        set_vec(e, shape + PLANE, [0.0, 1.0, 0.0, 0.0]);
        set_vec(e, shape + CENTRE, [0.0; 4]);
        set_vec(e, shape + HALF_EXTENTS, [10.0, 10.0, 10.0, 0.0]);
        shape
    }

    /// A collector object whose virtual at +0 is a double that keeps the output vector, hit
    /// fraction, extra info, first shape key and key index it was called with.
    fn collector_with_early_out(e: &mut Engine, early_out: f32) -> (u32, u32) {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable, 0x0900_0400);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        e.mem.set_f32(collector + 4, early_out);
        let seen = e.mem.alloc(0x50);
        e.register_double(0x0900_0400, move |e, a| {
            for word in 0..0x14 {
                let value = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(seen + 4 * word, value);
            }
            Ret::default()
        });
        (collector, seen)
    }

    #[test]
    fn calc_content_statistics_passes_the_class_record() {
        let mut e = world();
        e.register(REFERENCED_CALC_CONTENT_STATISTICS, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let got = e.call(0x00d5_4f80, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        assert_eq!(got.u32(), 0x55);
        assert_eq!(
            calls_to(&e, REFERENCED_CALC_CONTENT_STATISTICS),
            vec![vec![0x1000, 0x2000, 0x0126_9b84]]
        );
    }

    #[test]
    fn cast_ray_with_collector_reports_a_hit_to_the_collector() {
        for hit in [0u8, 1u8] {
            let mut e = world();
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(vtable + 0x20, 0x0900_0410);
            let shape = e.mem.alloc(0x40);
            e.mem.set_u32(shape, vtable);
            let outputs = e.mem.alloc(0x10);
            e.register_double(0x0900_0410, move |e, a| {
                e.mem.set_u8(a[1], hit);
                e.mem.set_u32(outputs, a[3]);

                Ret::default()
            });
            e.register(VECTOR_SET_MUL3, |_, _| Ret::default());
            let (collector, seen) = collector_with_early_out(&mut e, 0.75);
            let body = e.mem.alloc(0x10);
            e.mem.set_u32(body + 8, 0x7000);
            e.call(0x00d5_4f90, &args![shape, 0x6000u32, body, collector]);
            let setters = calls_to(&e, VECTOR_SET_MUL3);
            let delivered = calls_to(&e, 0x0900_0400);
            if hit == 0 {
                assert!(setters.is_empty());
                assert!(delivered.is_empty());
            } else {
                let output = e.mem.u32(outputs);
                assert_eq!(setters, vec![vec![output, 0x7000, output]]);
                assert_eq!(delivered, vec![vec![collector, body, output]]);
                assert_eq!(e.mem.f32(seen + 0x10), 0.75);
                assert_eq!(e.mem.u32(seen + 0x14), u32::MAX);
                assert_eq!(e.mem.u32(seen + 0x20), u32::MAX);
                assert_eq!(e.mem.u32(seen + 0x40), 0);
            }
            // The cast was given a result byte, the input and the output in that order.
            assert_eq!(calls_to(&e, 0x0900_0410).len(), 1);
            assert_eq!(calls_to(&e, 0x0900_0410)[0][0], shape);
            assert_eq!(calls_to(&e, 0x0900_0410)[0][2], 0x6000);
        }
    }

    /// Runs `castSphere` for a sphere of radius 0.5 from `from` to `to` with the limit float
    /// `limit`; returns the fraction and normal the collector saw, if it was called.
    fn run_sphere(from: Vec4, to: Vec4, limit: f32) -> Option<(f32, Vec4)> {
        let mut e = world();
        let shape = plane_shape(&mut e);
        let input = e.mem.alloc(0x40);
        set_vec(&mut e, input, from);
        set_vec(&mut e, input + 0x10, to);
        e.mem.set_f32(input + 0x30, 0.5);
        e.mem.set_f32(input + 0x34, limit);
        let (collector, seen) = collector_with_early_out(&mut e, 1.0);
        e.call(0x00d5_5000, &args![shape, input, 0x1234u32, collector]);
        let delivered = calls_to(&e, 0x0900_0400);
        if delivered.is_empty() {
            return None;
        }
        assert_eq!(delivered[0][1], 0x1234);
        assert_eq!(e.mem.u32(seen + 0x14), u32::MAX);
        assert_eq!(e.mem.u32(seen + 0x20), u32::MAX);
        assert_eq!(e.mem.u32(seen + 0x40), 0);
        Some((e.mem.f32(seen + 0x10), load_vec(&e, seen)))
    }

    #[test]
    fn cast_sphere_hits_the_plane_at_the_fraction_where_the_surface_touches() {
        // a = 2 - 0.5 = 1.5, b = -1 - 0.5 = -1.5: a / (a - b) = 0.5.
        let got = run_sphere([1.0, 2.0, 0.0, 0.0], [1.0, -1.0, 0.0, 0.0], -1.0).unwrap();
        assert_eq!(got.0, 0.5);
        assert_eq!(got.1, [0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn cast_sphere_starting_inside_hits_at_zero() {
        // a = 0.25 - 0.5 < 0: the fraction is 0.
        let got = run_sphere([0.0, 0.25, 0.0, 0.0], [0.0, -1.0, 0.0, 0.0], -1.0).unwrap();
        assert_eq!(got.0, 0.0);
    }

    #[test]
    fn cast_sphere_misses_when_it_ends_in_front_or_the_limit_is_too_high() {
        // b = 1 - 0.5 >= 0.
        assert!(run_sphere([0.0, 3.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], -1.0).is_none());
        // The limit float is above a - b = 3.
        assert!(run_sphere([0.0, 2.0, 0.0, 0.0], [0.0, -1.0, 0.0, 0.0], 3.5).is_none());
        // Outside the box: the hit point is at x = 20.
        assert!(run_sphere([20.0, 2.0, 0.0, 0.0], [20.0, -1.0, 0.0, 0.0], -1.0).is_none());
        // A NaN limit does not stop the cast.
        assert!(run_sphere([0.0, 2.0, 0.0, 0.0], [0.0, -1.0, 0.0, 0.0], f32::NAN).is_some());
    }

    #[test]
    fn collide_spheres_writes_the_plane_with_the_distance_less_the_radius() {
        let mut e = world();
        let shape = plane_shape(&mut e);
        e.mem.set_f32(shape + 0x1c, 1.0);
        let data = e.mem.alloc(0x20);
        set_vec(&mut e, data, [3.0, 5.0, 7.0, 0.5]);
        set_vec(&mut e, data + 0x10, [0.0, -2.0, 0.0, 1.0]);
        let spheres = e.mem.alloc(8);
        e.mem.set_u32(spheres, data);
        e.mem.set_u32(spheres + 4, 2);
        let output = e.mem.alloc(0x20);
        e.call(0x00d5_5160, &args![shape, spheres, output]);
        // Plane (0, 1, 0, 1): distance = 5 + 1 = 6, less 0.5; then -2 + 1 = -1, less 1.
        assert_eq!(load_vec(&e, output), [0.0, 1.0, 0.0, 5.5]);
        assert_eq!(load_vec(&e, output + 0x10), [0.0, 1.0, 0.0, -2.0]);
        // An empty list writes nothing.
        e.mem.set_u32(spheres + 4, 0);
        let untouched = e.mem.alloc(0x10);
        e.call(0x00d5_5160, &args![shape, spheres, untouched]);
        assert_eq!(e.mem.u32(untouched), 0);
    }

    #[test]
    fn the_box_constructor_sets_the_header_and_the_centre_and_half_extents() {
        let mut e = world();
        let shape = e.mem.alloc(0x40);
        let plane = e.mem.alloc(0x10);
        set_vec(&mut e, plane, [0.0, 0.0, 1.0, -4.0]);
        let aabb = e.mem.alloc(0x20);
        set_vec(&mut e, aabb, [-2.0, -4.0, 0.0, 0.0]);
        set_vec(&mut e, aabb + 0x10, [6.0, 4.0, 8.0, 0.0]);
        let got = e.call(0x00d5_51d0, &args![shape, plane, aabb]).u32();
        assert_eq!(got, shape);
        assert_eq!(e.mem.u32(shape), PLANE_SHAPE_VTABLE);
        assert_eq!(e.mem.u16(shape + 6), 1);
        assert_eq!(e.mem.u32(shape + 8), 0);
        assert_eq!(e.mem.u32(shape + 0xc), 0x1c);
        assert_eq!(load_vec(&e, shape + PLANE), [0.0, 0.0, 1.0, -4.0]);
        assert_eq!(load_vec(&e, shape + CENTRE), [2.0, 0.0, 4.0, 0.0]);
        assert_eq!(load_vec(&e, shape + HALF_EXTENTS), [4.0, 4.0, 4.0, 0.0]);
    }

    #[test]
    fn the_centre_constructor_derives_the_plane_offset() {
        let mut e = world();
        let shape = e.mem.alloc(0x40);
        let normal = e.mem.alloc(0x10);
        set_vec(&mut e, normal, [0.0, 1.0, 0.0, 9.0]);
        let centre = e.mem.alloc(0x10);
        set_vec(&mut e, centre, [1.0, 3.0, 2.0, 0.0]);
        let half = e.mem.alloc(0x10);
        set_vec(&mut e, half, [5.0, 6.0, 7.0, 8.0]);
        let got = e
            .call(0x00d5_5240, &args![shape, normal, centre, half])
            .u32();
        assert_eq!(got, shape);
        assert_eq!(e.mem.u32(shape), PLANE_SHAPE_VTABLE);
        assert_eq!(e.mem.u16(shape + 6), 1);
        assert_eq!(e.mem.u32(shape + 0xc), 0x1c);
        assert_eq!(load_vec(&e, shape + PLANE), [0.0, 1.0, 0.0, -3.0]);
        assert_eq!(load_vec(&e, shape + CENTRE), [1.0, 3.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, shape + HALF_EXTENTS), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn get_aabb_passes_the_quieted_tolerance_to_the_box_helper() {
        let mut e = world();
        e.register(CALC_AABB, |_, _| Ret::default());
        let shape = e.mem.alloc(0x40);
        e.call(
            0x00d5_52c0,
            &args![shape, 0x4000u32, f32::from_bits(0x7f80_0001), 0x5000u32],
        );
        assert_eq!(
            calls_to(&e, CALC_AABB),
            vec![vec![
                0x4000,
                shape + 0x30,
                shape + 0x20,
                0x7fc0_0001,
                0x5000
            ]]
        );
    }

    /// Runs `castRayImpl` on the plane `y = 0` and returns the result byte, the returned
    /// pointer, the output vector, the fraction and the key slot written.
    fn run_ray(from: Vec4, to: Vec4) -> (u8, u32, Vec4, f32, u32) {
        let mut e = world();
        let shape = plane_shape(&mut e);
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x40);
        e.mem.set_u32(PROFILE_SLOT, 3);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let input = e.mem.alloc(0x40);
        set_vec(&mut e, input, from);
        set_vec(&mut e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output + 0x40, 2);
        let flag = e.mem.alloc(8);
        e.mem.set_u8(flag, 0xaa);
        let got = e
            .call(0x00d5_52f0, &args![shape, flag, input, output])
            .u32();
        // Two profiling records: the start label and the end label.
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20 + 24);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        (
            e.mem.u8(flag),
            got - flag,
            load_vec(&e, output),
            e.mem.f32(output + 0x10),
            e.mem.u32(output + 0x28),
        )
    }

    #[test]
    fn cast_ray_hits_when_it_crosses_the_plane_inside_the_box() {
        let (byte, returned, normal, fraction, key) =
            run_ray([1.0, 3.0, 0.0, 0.0], [1.0, -1.0, 0.0, 0.0]);
        assert_eq!(byte, 1);
        assert_eq!(returned, 0);
        assert_eq!(normal, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(fraction, 0.75);
        assert_eq!(key, u32::MAX);
    }

    #[test]
    fn cast_ray_misses_behind_in_front_or_outside_the_box() {
        // Starts behind the plane.
        assert_eq!(run_ray([0.0, -1.0, 0.0, 0.0], [0.0, -2.0, 0.0, 0.0]).0, 0);
        // Ends in front of it.
        assert_eq!(run_ray([0.0, 3.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]).0, 0);
        // Ends exactly on the plane (d2 is not below zero).
        assert_eq!(run_ray([0.0, 3.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]).0, 0);
        // Crosses the plane outside the box.
        assert_eq!(run_ray([30.0, 3.0, 0.0, 0.0], [30.0, -1.0, 0.0, 0.0]).0, 0);
    }
}
