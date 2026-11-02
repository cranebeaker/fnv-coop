//! `Havok SDK/hkdisplaysemicircle.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplaySemiCircle` (the Xbox PDB has no record of the display classes; names and offsets
//! are read from the code) keeps, as far as these functions read it: the geometry block
//! pointer at `+8` (the layout `hkdisplaygeometry` describes), the axis vector at `+0x60`, the
//! start direction at `+0x70`, the centre at `+0x80`, the start and end angles (`float`s at
//! `+0x90` and `+0x94`), the radius (`float` at `+0x98`) and the number of segments (`int` at
//! `+0x9c`). SSE code computes lane by lane in `f32`; the x87 angle step is computed in `f64`
//! and rounded at its `float` store.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdisplaygeometry::allocate_geometry_block;
use super::hkpsphereshape::{load_vec, store_vec, zip_with};

/// The memory router (`00c85750`): its field `+0x10` is the allocator the arrays use.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`)
/// and the growth by one element (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW_BY_ONE: u32 = 0x00c9_0940;
/// `hkRotation::setAxisAngle` (`00cb2e30`, `this` = the rotation, `(axis, angle)`, `RET 8`) and
/// `hkVector4::setRotatedDir` (`00c7f700`, `this` = the result, `(rotation, vector)`, `RET 8`).
const ROTATION_SET_AXIS_ANGLE: u32 = 0x00cb_2e30;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;

layout! {
    /// `hkDisplaySemiCircle` as far as these functions read it (names unconfirmed).
    pub struct HkDisplaySemiCircle: 0xa0 {
        /// The geometry block `buildGeometry` makes.
        0x08 geometry: Ptr,
        /// The first radius direction (scaled by the radius).
        0x70 startDirection: Inline<Vector>,
        /// The centre.
        0x80 center: Inline<Vector>,
        0x90 startAngle: f32,
        0x94 endAngle: f32,
        0x98 radius: f32,
        0x9c segments: i32,
    }
    /// A 16-byte vector.
    pub struct Vector: 0x10 {}
}

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The frame of `generatePoints`, addressed like the exe's (from the stack pointer after the
/// three registers are pushed): the angle step, then the running point, the scratch point and
/// the rotation block (three rows). The exe keeps the frame 16-byte aligned for `MOVAPS`.
const STEP: u32 = 0x10;
const CURRENT: u32 = 0x20;
const SCRATCH: u32 = 0x30;
const ROTATION: u32 = 0x40;

// Translated from 00de4af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySemiCircle::generatePoints` (Xbox PDB), thiscall, `RET 4`; `out` is an
/// `hkArray` of 16-byte points. The angle step is `(end - start) / segments` (x87 `FSUB`,
/// `FIDIV`, stored as a `float`). The rotation block is made about the axis (the vector at
/// `+0x60`) by the start angle (`hkRotation::setAxisAngle`) and the radius times the start
/// direction is rotated by it (`setRotatedDir`) and added to the centre: the first point. The
/// array is made room for `segments + 2` points (`_reserve` to the larger of that and twice
/// the capacity, signed), its size set to that and the first point stored. With `segments` not
/// negative the rotation is remade for the step and, `segments + 1` times, the point's offset
/// from the centre is rotated by it and added to the centre again, and the result is stored as
/// the next point.
pub fn hk_display_semi_circle_generate_points(
    e: &mut Engine,
    this: Ptr<HkDisplaySemiCircle>,
    out: Ptr,
) {
    let me = this.addr();
    e.with_stack(0x90, |e, block| {
        let b = (block.addr() + 0xf) & !0xf;
        let start = e.get(this, HkDisplaySemiCircle::startAngle);
        let end = e.get(this, HkDisplaySemiCircle::endAngle);
        let segments = e.get(this, HkDisplaySemiCircle::segments);
        let span = f64::from(x87_quiet(end)) - f64::from(x87_quiet(start));
        let step = (span / f64::from(segments)) as f32;
        e.mem.set_f32(b + STEP, step);
        let start = e.get(this, HkDisplaySemiCircle::startAngle);
        e.call(
            ROTATION_SET_AXIS_ANGLE,
            &args![b + ROTATION, me + 0x60, x87_quiet(start)],
        );
        let radius = e.get(this, HkDisplaySemiCircle::radius);
        let direction = load_vec(e, me + 0x70);
        store_vec(e, b + CURRENT, direction.map(|lane| radius * lane));
        e.call(
            SET_ROTATED_DIR,
            &args![b + CURRENT, b + ROTATION, b + CURRENT],
        );
        let rotated = load_vec(e, b + CURRENT);
        let center = load_vec(e, me + 0x80);
        store_vec(e, b + CURRENT, zip_with(center, rotated, |a, b| a + b));
        let step = e.mem.f32(b + STEP);
        e.call(
            ROTATION_SET_AXIS_ANGLE,
            &args![b + ROTATION, me + 0x60, x87_quiet(step)],
        );
        let wanted = e.get(this, HkDisplaySemiCircle::segments).wrapping_add(2);
        let allocator = router_allocator(e);
        let array = out.addr();
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < wanted {
            let new_capacity = wanted.max(capacity.wrapping_add(capacity));
            e.call(
                ARRAY_RESERVE,
                &args![allocator, array, new_capacity as u32, 0x10u32],
            );
        }
        let data = e.mem.u32(array);
        let first = load_vec(e, b + CURRENT);
        e.mem.set_u32(array + 4, wanted as u32);
        store_vec(e, data, first);
        if e.get(this, HkDisplaySemiCircle::segments) < 0 {
            return;
        }
        e.mem.set_u32(b + STEP, 0);
        let mut index = 0i32;
        loop {
            let current = load_vec(e, b + CURRENT);
            let center = load_vec(e, me + 0x80);
            store_vec(e, b + SCRATCH, zip_with(current, center, |a, b| a - b));
            e.call(
                SET_ROTATED_DIR,
                &args![b + SCRATCH, b + ROTATION, b + SCRATCH],
            );
            let rotated = load_vec(e, b + SCRATCH);
            let center = load_vec(e, me + 0x80);
            let offset = e.mem.u32(b + STEP);
            let data = e.mem.u32(array);
            let point = zip_with(center, rotated, |a, b| a + b);
            store_vec(e, b + SCRATCH, point);
            store_vec(e, data.wrapping_add(offset).wrapping_add(0x10), point);
            let point = load_vec(e, b + SCRATCH);
            index = index.wrapping_add(1);
            store_vec(e, b + CURRENT, point);
            e.mem.set_u32(b + STEP, offset.wrapping_add(0x10));
            if index > e.get(this, HkDisplaySemiCircle::segments) {
                break;
            }
        }
    });
}

// Translated from 00de4c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySemiCircle::getWireframeGeometry` (Xbox PDB), thiscall, `RET 8`; `out` is an
/// `hkArray` of 16-byte points and `allocator` the allocator it grows with. Generates the
/// points into a local array, makes `out` room for `2 * segments` points (`_reserve` with
/// `allocator` to the larger of that and twice the capacity, signed) and sets its size to
/// that, then with `segments` above zero copies each pair of neighbouring points into it, so
/// that segment `i` is the points `i` and `i + 1`. The local array's storage is freed through
/// the router's allocator (slot `0x10`, `(data, 16 * capacity)`) unless it does not own it.
pub fn hk_display_semi_circle_get_wireframe_geometry(
    e: &mut Engine,
    this: Ptr<HkDisplaySemiCircle>,
    out: Ptr,
    allocator: u32,
) {
    e.with_stack(0xc, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        e.mem.set_u32(local + 4, 0);
        e.mem.set_u32(local + 8, 0x8000_0000);
        hk_display_semi_circle_generate_points(e, this, Ptr::new(local));
        let array = out.addr();
        let segments = e.get(this, HkDisplaySemiCircle::segments);
        let doubled = segments.wrapping_add(e.get(this, HkDisplaySemiCircle::segments));
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < doubled {
            let new_capacity = doubled.max(capacity.wrapping_add(capacity));
            e.call(
                ARRAY_RESERVE,
                &args![allocator, array, new_capacity as u32, 0x10u32],
            );
        }
        e.mem.set_u32(array + 4, doubled as u32);
        if e.get(this, HkDisplaySemiCircle::segments) > 0 {
            let mut index = 0i32;
            let mut source = 0u32;
            let mut target = 0u32;
            loop {
                let points = e.mem.u32(local);
                let point = load_vec(e, source.wrapping_add(points));
                let data = e.mem.u32(array);
                store_vec(e, data.wrapping_add(target), point);
                let points = e.mem.u32(local);
                let data = e.mem.u32(array);
                let next = load_vec(e, source.wrapping_add(points).wrapping_add(0x10));
                target = target.wrapping_add(0x10);
                store_vec(e, data.wrapping_add(target), next);
                index = index.wrapping_add(1);
                target = target.wrapping_add(0x10);
                source = source.wrapping_add(0x10);
                if index >= e.get(this, HkDisplaySemiCircle::segments) {
                    break;
                }
            }
        }
        let release = router_allocator(e);
        let capacity = e.mem.u32(local + 8);
        e.mem.set_u32(local + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(local);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(16);
            e.vcall(release, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00de4d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySemiCircle::buildGeometry` (Xbox PDB), thiscall: makes the geometry block
/// (`+8`, two empty arrays; none when the allocator answers null), generates the points into
/// its first array, appends the centre as one more vertex (growing the array by one element
/// through `00c90940` when it is full) and, with `segments` above zero, writes for each
/// segment `i` the record `(index of the centre, i + 1, i, -1)` at `16 * i` from the data
/// pointer of the block's second array. As in the exe that array has no storage yet, so the
/// writes go to the address `16 * i + 0` (a fault in the exe).
pub fn hk_display_semi_circle_build_geometry(e: &mut Engine, this: Ptr<HkDisplaySemiCircle>) {
    let block = allocate_geometry_block(e);
    e.set(this, HkDisplaySemiCircle::geometry, Ptr::new(block));
    hk_display_semi_circle_generate_points(e, this, Ptr::new(block));
    let geometry = e.get(this, HkDisplaySemiCircle::geometry).addr();
    let allocator = router_allocator(e);
    if e.mem.u32(geometry + 4) == e.mem.u32(geometry + 8) & 0x3fff_ffff {
        e.call(ARRAY_GROW_BY_ONE, &args![allocator, geometry, 0x10u32]);
    }
    let size = e.mem.u32(geometry + 4);
    let center = load_vec(e, this.addr() + 0x80);
    let slot = size.wrapping_mul(16).wrapping_add(e.mem.u32(geometry));
    store_vec(e, slot, center);
    e.mem.set_u32(geometry + 4, size.wrapping_add(1));
    let block = e.get(this, HkDisplaySemiCircle::geometry).addr();
    let center_index = e.mem.u32(block + 4).wrapping_sub(1);
    if e.get(this, HkDisplaySemiCircle::segments) > 0 {
        let mut segment = 0u32;
        let mut offset = 0u32;
        loop {
            let block = e.get(this, HkDisplaySemiCircle::geometry).addr();
            let record = e.mem.u32(block + 0xc).wrapping_add(offset);
            e.mem.set_u32(record + 8, segment);
            e.mem.set_u32(record, center_index);
            e.mem.set_u32(record + 4, segment.wrapping_add(1));
            e.mem.set_u32(record + 0xc, 0xffff_ffff);
            segment = segment.wrapping_add(1);
            offset = offset.wrapping_add(0x10);
            if (segment as i32) >= e.get(this, HkDisplaySemiCircle::segments) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00de4af0,
            hk_display_semi_circle_generate_points(Ptr<HkDisplaySemiCircle>, Ptr)
        ),
        entry!(
            0x00de4c40,
            hk_display_semi_circle_get_wireframe_geometry(Ptr<HkDisplaySemiCircle>, Ptr, u32)
        ),
        entry!(
            0x00de4d20,
            hk_display_semi_circle_build_geometry(Ptr<HkDisplaySemiCircle>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROTATION_DOUBLE: u32 = ROTATION_SET_AXIS_ANGLE;
    const ROTATE_DOUBLE: u32 = SET_ROTATED_DIR;
    const FREE_DOUBLE: u32 = 0x4800_0010;
    const ALLOCATE_DOUBLE: u32 = 0x4800_0004;
    const RESERVE_DOUBLE: u32 = ARRAY_RESERVE;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A 16-byte aligned block of `size` bytes.
    fn aligned(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    /// The router with an allocator whose slot 4 allocates and slot `0x10` frees (logged).
    fn install_router(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register(MEMORY_ROUTER, |e, _| {
            // The router object is found through the data word the test keeps at 0x4800_0000.
            let router = e.mem.u32(0x4800_0000);
            Ret {
                eax: router,
                ..Ret::default()
            }
        });
        e.map(0x4800_0000, 0x1000);
        e.mem.set_u32(0x4800_0000, router);
        e.register(ALLOCATE_DOUBLE, |e, a| Ret {
            eax: aligned(e, a[1]),
            ..Ret::default()
        });
        e.register(FREE_DOUBLE, |_, _| Ret::default());
        allocator
    }

    /// The semicircle with the given angles, radius and segments; the axis, the start
    /// direction and the centre are fixed.
    fn semicircle(e: &mut Engine, start: f32, end: f32, radius: f32, segments: i32) -> u32 {
        let this = aligned(e, 0xa0);
        for (offset, lanes) in [
            (0x60u32, [0.0f32, 0.0, 1.0, 0.0]),
            (0x70, [1.0, 0.0, 0.0, 0.0]),
            (0x80, [10.0, 20.0, 30.0, 1.0]),
        ] {
            for (i, lane) in lanes.into_iter().enumerate() {
                e.mem.set_f32(this + offset + 4 * i as u32, lane);
            }
        }
        e.mem.set_f32(this + 0x90, start);
        e.mem.set_f32(this + 0x94, end);
        e.mem.set_f32(this + 0x98, radius);
        e.mem.set_i32(this + 0x9c, segments);
        this
    }

    /// Doubles for the rotation: it records the angle it is given in the block and
    /// `setRotatedDir` adds the rotation block's first word (the angle) to the vector's lane 0.
    fn install_rotation(e: &mut Engine) {
        e.register(ROTATION_DOUBLE, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            Ret::default()
        });
        e.register(ROTATE_DOUBLE, |e, a| {
            let angle = e.mem.f32(a[1]);
            let lane = e.mem.f32(a[2]) + angle;
            e.mem.set_f32(a[0], lane);
            for i in 1..4 {
                let value = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, value);
            }
            Ret::default()
        });
    }

    #[test]
    fn generate_points_steps_the_rotation_from_the_start_angle() {
        let mut e = Engine::new();
        install_router(&mut e);
        install_rotation(&mut e);
        // The reserve double gives the array room for what it is asked.
        e.register(RESERVE_DOUBLE, |e, a| {
            let data = aligned(e, a[2] * 16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let this = semicircle(&mut e, 0.25, 2.25, 2.0, 4);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call_log = Some(vec![]);
        e.call(0x00de_4af0, &args![this, out]);
        // The rotation block is made for the start angle and then for the step (0.5).
        let rotations = calls_to(&e, ROTATION_DOUBLE);
        assert_eq!(rotations.len(), 2);
        assert_eq!(rotations[0][2], 0.25f32.to_bits());
        assert_eq!(rotations[1][2], 0.5f32.to_bits());
        assert_eq!(rotations[0][1], this + 0x60);
        // Six points: the first is the centre plus radius * direction rotated by the angle
        // (the double adds the angle to lane 0), the next ones add the step each time.
        assert_eq!(e.mem.u32(out + 4), 6);
        assert_eq!(calls_to(&e, RESERVE_DOUBLE).len(), 1);
        assert_eq!(calls_to(&e, RESERVE_DOUBLE)[0][2..], [6, 0x10]);
        let data = e.mem.u32(out);
        let x: Vec<f32> = (0..6).map(|i| e.mem.f32(data + 16 * i)).collect();
        assert_eq!(x, vec![12.25, 12.75, 13.25, 13.75, 14.25, 14.75]);
        // The other lanes are the centre's.
        assert_eq!(e.mem.f32(data + 16 * 5 + 4), 20.0);
        assert_eq!(e.mem.f32(data + 16 * 5 + 8), 30.0);
    }

    #[test]
    fn generate_points_with_no_segments_stores_only_the_first_point() {
        let mut e = Engine::new();
        install_router(&mut e);
        install_rotation(&mut e);
        let this = semicircle(&mut e, 0.25, 2.25, 2.0, -1);
        let out = e.mem.alloc(0x10);
        let data = aligned(&mut e, 0x40);
        e.mem.set_u32(out, data);
        e.mem.set_u32(out + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00de_4af0, &args![this, out]);
        // Negative segments: the size is segments + 2 and nothing else is stored.
        assert_eq!(e.mem.u32(out + 4), 1);
        assert_eq!(e.mem.f32(data), 12.25);
        assert_eq!(calls_to(&e, ROTATE_DOUBLE).len(), 1);
        assert!(calls_to(&e, RESERVE_DOUBLE).is_empty());
    }

    #[test]
    fn get_wireframe_geometry_pairs_neighbouring_points() {
        let mut e = Engine::new();
        install_router(&mut e);
        install_rotation(&mut e);
        e.register(RESERVE_DOUBLE, |e, a| {
            let data = aligned(e, a[2] * 16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let this = semicircle(&mut e, 0.25, 2.25, 2.0, 4);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call_log = Some(vec![]);
        e.call(0x00de_4c40, &args![this, out, 0x1234u32]);
        // Eight lane-0 values: the segments (0,1), (1,2), (2,3), (3,4).
        assert_eq!(e.mem.u32(out + 4), 8);
        let data = e.mem.u32(out);
        let x: Vec<f32> = (0..8).map(|i| e.mem.f32(data + 16 * i)).collect();
        assert_eq!(
            x,
            vec![12.25, 12.75, 12.75, 13.25, 13.25, 13.75, 13.75, 14.25]
        );
        // The output array was reserved with the allocator argument (after the generation's own).
        let reserves = calls_to(&e, RESERVE_DOUBLE);
        assert_eq!(reserves.len(), 2);
        assert_eq!(reserves[1][..], [0x1234, out, 8, 0x10]);
        // The local array (capacity 6) is freed with 6 * 16 bytes.
        let freed = calls_to(&e, FREE_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 96);
    }

    #[test]
    fn get_wireframe_geometry_without_segments_only_sizes_the_output() {
        let mut e = Engine::new();
        install_router(&mut e);
        install_rotation(&mut e);
        e.register(RESERVE_DOUBLE, |e, a| {
            let data = aligned(e, a[2] * 16 + 32);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2] + 2);
            Ret::default()
        });
        let this = semicircle(&mut e, 0.25, 2.25, 2.0, 0);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.mem.set_u32(out + 4, 7);
        e.call_log = Some(vec![]);
        e.call(0x00de_4c40, &args![this, out, 0x1234u32]);
        assert_eq!(e.mem.u32(out + 4), 0);
        // The generation reserved room for two points, the output needed none.
        assert_eq!(calls_to(&e, RESERVE_DOUBLE).len(), 1);
    }

    #[test]
    fn build_geometry_without_segments_adds_the_centre_vertex() {
        let mut e = Engine::new();
        install_router(&mut e);
        install_rotation(&mut e);
        e.register(RESERVE_DOUBLE, |e, a| {
            let data = aligned(e, a[2] * 16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register(ARRAY_GROW_BY_ONE, |e, a| {
            let data = aligned(e, 0x100);
            let size = e.mem.u32(a[1] + 4);
            for word in 0..size * 4 {
                let value = e.mem.u32(e.mem.u32(a[1]) + 4 * word);
                e.mem.set_u32(data + 4 * word, value);
            }
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 16);
            Ret::default()
        });
        let this = semicircle(&mut e, 0.25, 2.25, 2.0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00de_4d20, &args![this]);
        let block = e.mem.u32(this + 8);
        assert_ne!(block, 0);
        // Two generated points (segments + 2), then the centre appended: its index is 2.
        assert_eq!(e.mem.u32(block + 4), 3);
        let data = e.mem.u32(block);
        assert_eq!(e.mem.f32(data), 12.25);
        assert_eq!(e.mem.f32(data + 32), 10.0);
        assert_eq!(e.mem.f32(data + 36), 20.0);
        // The generation filled the array exactly, so it was grown by one element.
        assert_eq!(calls_to(&e, ARRAY_GROW_BY_ONE).len(), 1);
        assert_eq!(calls_to(&e, ARRAY_GROW_BY_ONE)[0][2..], [0x10]);
    }

    #[test]
    fn build_geometry_writes_a_triangle_per_segment_into_the_second_array() {
        let mut e = Engine::new();
        install_router(&mut e);
        install_rotation(&mut e);
        let this = semicircle(&mut e, 0.25, 2.25, 2.0, 2);
        // The block's second array gets storage by hand (from the reserve of the points, which
        // runs on the first array of the block): the exe writes through the data pointer of the
        // empty array it has just made, which faults.
        let triangles = aligned(&mut e, 0x40);
        e.register_double(RESERVE_DOUBLE, move |e, a| {
            let data = aligned(e, a[2] * 16 + 64);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2] + 4);
            e.mem.set_u32(a[1] + 0xc, triangles);
            Ret::default()
        });
        e.call(0x00de_4d20, &args![this]);
        // The centre vertex is the last: 2 segments + 2 points, then the centre (index 4).
        let block = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(block + 4), 5);
        // Each record is (centre index, i + 1, i, -1).
        let records: Vec<[u32; 4]> = (0..2)
            .map(|i| std::array::from_fn(|w| e.mem.u32(triangles + 16 * i + 4 * w as u32)))
            .collect();
        assert_eq!(
            records,
            vec![[4, 1, 0, 0xffff_ffff], [4, 2, 1, 0xffff_ffff]]
        );
    }
}
