//! `Havok SDK/hkdisplaycylinder.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplayCylinder` (Xbox PDB) keeps its two axis end points as 16-byte vectors at `+0x60`
//! and `+0x70`, the radius (`float`) at `+0x80`, the number of sides (`int`) at `+0x84` and
//! the number of segments along the axis (`int`) at `+0x88`; the shape kind word at `+0x50`
//! is 9. The shared base layout is described in `hkdisplaygeometry`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdisplaygeometry::{allocate_geometry_block, push_back_slot, reset_transform};
use super::hkpprimitivedrawer::{normalised, rotate_by_quaternion};
use super::hkpsphereshape::{
    copy_vec, inverse_length, load_vec, shufps, store_vec, sum3, zip_with, Vec4, HALF_F32,
};

/// The vtable the constructor stores at `+0`.
const CYLINDER_VTABLE: u32 = 0x010d_0f84;
/// `hkQuaternion::setAxisAngle` (`00cb2450`, `this` = the quaternion, `(axis, angle)`, `RET 8`).
const QUATERNION_SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// The quaternion product (`00c66320`, `this` = the result, `(left, right)`, `RET 8`); the
/// callers in the exe use it as `this = left * right`.
const QUATERNION_MULTIPLY: u32 = 0x00c6_6320;
/// The double `2 * pi`.
const FULL_TURN: u32 = 0x0101_ff48;
/// The unit vectors `(1, 0, 0, 0)`, `(0, 1, 0, 0)` and `(0, 0, 1, 0)` the code picks its
/// reference axis from.
const UNIT_X: u32 = 0x010c_7180;
const UNIT_Y: u32 = 0x010c_7190;
const UNIT_Z: u32 = 0x010c_71a0;

/// The locals `buildGeometry` passes by address: the axis (the normalised end-to-end step)
/// and three quaternions, the rotation of one step round the axis, the running rotation of
/// the ring being built and the copy of it the product reads.
const FRAME_SIZE: u32 = 0x40;
const AXIS: u32 = 0x00;
const STEP_ROTATION: u32 = 0x10;
const RING_ROTATION: u32 = 0x20;
const ROTATION_COPY: u32 = 0x30;

// Translated from 00d58500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCylinder::getWireframeGeometry` (Xbox PDB): the exe's body is a bare `RET 8`, so
/// it takes its two stack words and does nothing.
pub fn hk_display_cylinder_get_wireframe_geometry(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) {
}

// Translated from 00d58510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCylinder::hkDisplayCylinder` (Xbox PDB): clears the geometry pointer `+8`, sets
/// the kind word `+0x50` to 9 and the 16-bit count `+6` to 1, writes the identity transform
/// (`reset_transform`), stores the vtable, the two end points (16-byte copies from the vectors
/// the first two arguments point to, to `+0x60` and `+0x70`), the radius (through the x87
/// stack, so a signalling NaN comes out quiet) and the side and segment counts. Returns
/// `this`.
pub fn hk_display_cylinder_constructor(
    e: &mut Engine,
    this: Ptr,
    end_a: Ptr,
    end_b: Ptr,
    radius: f32,
    sides: i32,
    segments: i32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0x50, 9);
    e.mem.set_u16(me + 6, 1);
    reset_transform(e, me);
    e.mem.set_i32(me + 0x84, sides);
    e.mem.set_i32(me + 0x88, segments);
    e.mem.set_u32(me, CYLINDER_VTABLE);
    copy_vec(e, end_a.addr(), me + 0x60);
    e.mem.set_u32(me + 0x80, x87_quiet_bits(radius.to_bits()));
    copy_vec(e, end_b.addr(), me + 0x70);
    this
}

/// Appends the record `(a, b, c, -1)` to the index array at `array`.
fn push_record(e: &mut Engine, array: u32, a: i32, b: i32, c: i32) {
    let record = push_back_slot(e, array);
    e.mem.set_i32(record, a);
    e.mem.set_i32(record + 4, b);
    e.mem.set_i32(record + 8, c);
    e.mem.set_i32(record + 0xc, -1);
}

/// The reference axis the first spoke is built against: `(0, 1, 0)` when the step direction's
/// x lane is above one half in magnitude, else `(1, 0, 0)` when its y lane is not, else
/// `(0, 0, 1)` (the compares are the x87 ones: a NaN lane counts as "not above").
fn reference_axis(e: &Engine, direction: Vec4) -> Vec4 {
    let half: f32 = e.global(HALF_F32);
    let address = if !(half < direction[0].abs()) {
        if !(half < direction[1].abs()) {
            UNIT_X
        } else {
            UNIT_Z
        }
    } else {
        UNIT_Y
    };
    load_vec(e, address)
}

// Translated from 00d58590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCylinder::buildGeometry` (Xbox PDB), `this` in `ECX`, no stack words. Builds the
/// geometry block (`+8`) of vertices and index records.
///
/// The step is `(end A - end B) / segments`; its normalised form is the axis. The first spoke
/// is the cross product of the reference axis and the axis, normalised and scaled by the
/// radius. `setAxisAngle` makes the quaternion of one side step (`2 * pi / sides` about the
/// axis) and the ring rotation (angle 0). The vertices are end B, then end B + segments *
/// step (the other end), then, for each side, `segments + 1` points: the ring rotation applied
/// to the spoke plus end B, then repeatedly plus the step, the ring rotation then multiplied
/// by the step rotation. The index records are two triangles per side and segment (indices
/// wrapping round the sides) and two fans around the end vertices. (The exe also computes the
/// length of the step with `SQRTSS` and stores it in a slot that is overwritten at once; that
/// dead store is not kept.)
pub fn hk_display_cylinder_build_geometry(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let end_a = load_vec(e, me + 0x60);
    let end_b = load_vec(e, me + 0x70);
    let segments = e.mem.i32(me + 0x88);
    let reciprocal = (1.0f64 / f64::from(segments)) as f32;
    let step: Vec4 = zip_with(end_a, end_b, |a, b| a - b).map(|lane| reciprocal * lane);
    let direction = normalised(e, step);
    let seed = reference_axis(e, direction);
    let turned = zip_with(
        shufps(seed, seed, 0xd2),
        shufps(direction, direction, 0xc9),
        |a, b| a * b,
    );
    let crossed = zip_with(
        shufps(seed, seed, 0xc9),
        shufps(direction, direction, 0xd2),
        |a, b| a * b,
    );
    let perpendicular = zip_with(turned, crossed, |a, b| a - b);
    let factor = inverse_length(
        e,
        sum3(zip_with(perpendicular, perpendicular, |a, b| a * b)),
    );
    let radius = e.mem.f32(me + 0x80);
    let spoke: Vec4 = perpendicular.map(|lane| factor * lane * radius);
    let sides = e.mem.i32(me + 0x84);
    let full_turn: f64 = e.global(FULL_TURN);
    let step_angle = (full_turn / f64::from(sides)) as f32;
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        store_vec(e, frame + AXIS, direction);
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![frame + STEP_ROTATION, frame + AXIS, step_angle],
        );
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![frame + RING_ROTATION, frame + AXIS, 0.0f32],
        );
        build_vertices(e, me, frame, step, spoke);
        build_indices(e, me);
    });
}

/// The vertex half of `buildGeometry`: allocates the block, stores it at `this + 8` and fills
/// the vertex array.
fn build_vertices(e: &mut Engine, me: u32, frame: u32, step: Vec4, spoke: Vec4) {
    let block = allocate_geometry_block(e);
    e.mem.set_u32(me + 8, block);
    let slot = push_back_slot(e, block);
    copy_vec(e, me + 0x70, slot);
    let segments = e.mem.i32(me + 0x88);
    let end_b = load_vec(e, me + 0x70);
    let far_end = zip_with(step.map(|lane| (segments as f32) * lane), end_b, |a, b| {
        a + b
    });
    let block = e.mem.u32(me + 8);
    let slot = push_back_slot(e, block);
    store_vec(e, slot, far_end);
    if e.mem.i32(me + 0x84) > 0 {
        let mut ring = 0i32;
        loop {
            let rotation = load_vec(e, frame + RING_ROTATION);
            let rotated = rotate_by_quaternion(e, rotation, spoke);
            let end_b = load_vec(e, me + 0x70);
            let mut point = zip_with(rotated, end_b, |a, b| a + b);
            if e.mem.i32(me + 0x88) > 0 {
                let mut along = 0i32;
                loop {
                    let block = e.mem.u32(me + 8);
                    let slot = push_back_slot(e, block);
                    store_vec(e, slot, point);
                    along = along.wrapping_add(1);
                    let more = along < e.mem.i32(me + 0x88);
                    point = zip_with(point, step, |a, b| a + b);
                    if !more {
                        break;
                    }
                }
            }
            let block = e.mem.u32(me + 8);
            let slot = push_back_slot(e, block);
            store_vec(e, slot, point);
            copy_vec(e, frame + RING_ROTATION, frame + ROTATION_COPY);
            e.call(
                QUATERNION_MULTIPLY,
                &args![
                    frame + RING_ROTATION,
                    frame + ROTATION_COPY,
                    frame + STEP_ROTATION
                ],
            );
            ring = ring.wrapping_add(1);
            if ring >= e.mem.i32(me + 0x84) {
                break;
            }
        }
    }
}

/// The index half of `buildGeometry`. With `stride = segments + 1` points per side and
/// `total = stride * sides + 2` vertices: for each side and segment two records
/// `(u, t, w)` and `(w, t, x)` that start at `(2, segments + 3, 3)` and `x = segments + 4`
/// and all step by one, `t` and `x` stepping back by `total - 2` when `x` reaches `total`
/// at the end of a side; then, for the end vertex 0 and the end vertex 1 in turn, a fan record
/// per side.
fn build_indices(e: &mut Engine, me: u32) {
    let segments = e.mem.i32(me + 0x88);
    let sides = e.mem.i32(me + 0x84);
    let stride = segments.wrapping_add(1);
    let total = stride.wrapping_mul(sides).wrapping_add(2);
    let mut first = 2i32;
    let mut third = 3i32;
    let mut fourth = segments.wrapping_add(4);
    let mut shared = segments.wrapping_add(3);
    if sides > 0 {
        let mut side = 0i32;
        loop {
            if segments > 0 {
                let mut along = 0i32;
                loop {
                    let index_array = e.mem.u32(me + 8).wrapping_add(0xc);
                    push_record(e, index_array, first, shared, third);
                    let index_array = e.mem.u32(me + 8).wrapping_add(0xc);
                    push_record(e, index_array, third, shared, fourth);
                    first = first.wrapping_add(1);
                    third = third.wrapping_add(1);
                    fourth = fourth.wrapping_add(1);
                    shared = shared.wrapping_add(1);
                    along = along.wrapping_add(1);
                    if along >= e.mem.i32(me + 0x88) {
                        break;
                    }
                }
            }
            first = first.wrapping_add(1);
            third = third.wrapping_add(1);
            fourth = fourth.wrapping_add(1);
            shared = shared.wrapping_add(1);
            if fourth >= total {
                shared = shared.wrapping_add(2i32.wrapping_sub(total));
                fourth = fourth.wrapping_add(2i32.wrapping_sub(total));
            }
            side = side.wrapping_add(1);
            if side >= e.mem.i32(me + 0x84) {
                break;
            }
        }
    }
    for cap in 0..2i32 {
        let mut base = if cap != 0 { e.mem.i32(me + 0x88) } else { 0 }.wrapping_add(2);
        if e.mem.i32(me + 0x84) > 0 {
            let mut side = 0i32;
            loop {
                let mut next = base.wrapping_add(stride);
                if next >= total {
                    next = next.wrapping_add(2i32.wrapping_sub(total));
                }
                let index_array = e.mem.u32(me + 8).wrapping_add(0xc);
                if cap != 0 {
                    push_record(e, index_array, cap, base, next);
                } else {
                    push_record(e, index_array, 0, next, base);
                }
                base = next;
                side = side.wrapping_add(1);
                if side >= e.mem.i32(me + 0x84) {
                    break;
                }
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5_8500,
            hk_display_cylinder_get_wireframe_geometry(Ptr, u32, u32)
        ),
        entry!(0x00d5_8510, hk_display_cylinder_constructor(Ptr, Ptr, Ptr, f32, i32, i32) -> Ptr),
        entry!(0x00d5_8590, hk_display_cylinder_build_geometry(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_F32: u32 = 0x0101_1d78;
    const THREE_F32: u32 = 0x0101_7718;
    const ROTATION_BIAS: u32 = 0x010c_71e0;
    const MEMORY_ROUTER: u32 = 0x00c8_5750;
    const ARRAY_RESERVE: u32 = 0x00c9_08c0;
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0031;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// `a * b` of two `(x, y, z, w)` quaternions.
    fn multiply(a: Vec4, b: Vec4) -> Vec4 {
        let cross = [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];
        let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        [
            a[3] * b[0] + b[3] * a[0] + cross[0],
            a[3] * b[1] + b[3] * a[1] + cross[1],
            a[3] * b[2] + b[3] * a[2] + cross[2],
            a[3] * b[3] - dot,
        ]
    }

    /// The constants the code reads, the memory router with an allocator that hands out
    /// zeroed blocks, a reserve that moves an array to a fresh buffer, and quaternion doubles.
    fn rig() -> Engine {
        let mut e = engine();
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(FULL_TURN, std::f64::consts::TAU);
        for lane in 0..4 {
            e.mem.set_f32(ROTATION_BIAS + 4 * lane, -0.5);
        }
        store_vec(&mut e, UNIT_X, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, UNIT_Y, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, UNIT_Z, [0.0, 0.0, 1.0, 0.0]);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_SLOT_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(ARRAY_RESERVE, |e, a| {
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let fresh = e.mem.alloc(a[2] * a[3]);
            if size > 0 {
                let bytes = e.mem.bytes(old, size * a[3]);
                e.mem.write(fresh, &bytes);
            }
            e.mem.set_u32(a[1], fresh);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register_double(QUATERNION_SET_AXIS_ANGLE, |e, a| {
            let axis = load_vec(e, a[1]);
            let (sin, cos) = (f64::from(f32::from_bits(a[2])) / 2.0).sin_cos();
            let sin = sin as f32;
            store_vec(
                e,
                a[0],
                [axis[0] * sin, axis[1] * sin, axis[2] * sin, cos as f32],
            );
            Ret::default()
        });
        e.register_double(QUATERNION_MULTIPLY, |e, a| {
            let product = multiply(load_vec(e, a[1]), load_vec(e, a[2]));
            store_vec(e, a[0], product);
            Ret::default()
        });
        e
    }

    fn cylinder(e: &mut Engine, end_a: Vec4, end_b: Vec4, sides: i32, segments: i32) -> u32 {
        let this = e.mem.alloc(0x90);
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        store_vec(e, a, end_a);
        store_vec(e, b, end_b);
        e.call(0x00d5_8510, &args![this, a, b, 1.0f32, sides, segments]);
        this
    }

    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-4,
            "{actual} is not close to {expected}"
        );
    }

    fn close_vec(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            close(actual[lane], expected[lane]);
        }
    }

    fn vertices(e: &Engine, block: u32) -> Vec<Vec4> {
        let data = e.mem.u32(block);
        (0..e.mem.u32(block + 4))
            .map(|i| load_vec(e, data + 0x10 * i))
            .collect()
    }

    fn records(e: &Engine, block: u32) -> Vec<[i32; 4]> {
        let data = e.mem.u32(block + 0xc);
        (0..e.mem.u32(block + 0x10))
            .map(|i| std::array::from_fn(|lane| e.mem.i32(data + 0x10 * i + 4 * lane as u32)))
            .collect()
    }

    #[test]
    fn hk_display_cylinder_get_wireframe_geometry_does_nothing() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x84, 7);
        e.call(0x00d5_8500, &args![this, 1u32, 2u32]);
        assert_eq!(e.mem.u32(this + 0x84), 7);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|c| c.0 == 0x00d5_8500));
    }

    #[test]
    fn hk_display_cylinder_constructor_stores_its_fields_and_the_identity_transform() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        for offset in (0..0x90).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        store_vec(&mut e, a, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, b, [5.0, 6.0, 7.0, 8.0]);
        let radius = f32::from_bits(0x7f80_0001);
        let got = e.call(0x00d5_8510, &args![this, a, b, radius, 12i32, -3i32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_0f84);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 9);
        assert_eq!(load_vec(&e, this + 0x60), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x70), [5.0, 6.0, 7.0, 8.0]);
        // The radius went through the x87 stack: the signalling NaN is quiet.
        assert_eq!(e.mem.u32(this + 0x80), 0x7fc0_0001);
        assert_eq!(e.mem.i32(this + 0x84), 12);
        assert_eq!(e.mem.i32(this + 0x88), -3);
        for lane in 0..16u32 {
            let expected = if [0, 5, 10].contains(&lane) { 1.0 } else { 0.0 };
            assert_eq!(e.mem.f32(this + 0x10 + 4 * lane), expected);
        }
        // The word between the 16-bit count and the block pointer is left alone.
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
    }

    #[test]
    fn hk_display_cylinder_build_geometry_builds_rings_triangles_and_caps() {
        let mut e = rig();
        let this = cylinder(&mut e, [0.0, 2.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 3, 1);
        e.call(0x00d5_8590, &args![this]);
        let block = e.mem.u32(this + 8);
        let points = vertices(&e, block);
        assert_eq!(points.len(), 8);
        close_vec(points[0], [0.0, 0.0, 0.0, 0.0]);
        close_vec(points[1], [0.0, 2.0, 0.0, 0.0]);
        let sin = 3.0f32.sqrt() / 2.0;
        let ring_points = [
            [1.0, 0.0, 0.0, 0.0],
            [1.0, 2.0, 0.0, 0.0],
            [-0.5, 0.0, -sin, 0.0],
            [-0.5, 2.0, -sin, 0.0],
            [-0.5, 0.0, sin, 0.0],
            [-0.5, 2.0, sin, 0.0],
        ];
        for (index, expected) in ring_points.into_iter().enumerate() {
            close_vec(points[2 + index], expected);
        }
        assert_eq!(
            records(&e, block),
            vec![
                [2, 4, 3, -1],
                [3, 4, 5, -1],
                [4, 6, 5, -1],
                [5, 6, 7, -1],
                [6, 2, 7, -1],
                [7, 2, 3, -1],
                [0, 4, 2, -1],
                [0, 6, 4, -1],
                [0, 2, 6, -1],
                [1, 3, 5, -1],
                [1, 5, 7, -1],
                [1, 7, 3, -1],
            ]
        );
        // The step rotation first (2 * pi / sides), then the identity ring rotation.
        let axis_calls = calls_to(&e, QUATERNION_SET_AXIS_ANGLE);
        assert_eq!(axis_calls.len(), 2);
        assert_eq!(axis_calls[0][1], axis_calls[1][1]);
        assert_eq!(
            axis_calls[0][2],
            ((std::f64::consts::TAU / 3.0) as f32).to_bits()
        );
        assert_eq!(axis_calls[1][2], 0);
        let multiplies = calls_to(&e, QUATERNION_MULTIPLY);
        assert_eq!(multiplies.len(), 3);
        for call in &multiplies {
            assert_eq!(call[0], axis_calls[1][0]);
            assert_eq!(call[2], axis_calls[0][0]);
        }
        // The geometry block is 0x18 bytes from the allocator.
        assert_eq!(calls_to(&e, ALLOCATE_SLOT_DOUBLE).len(), 1);
        assert_eq!(calls_to(&e, ALLOCATE_SLOT_DOUBLE)[0][1], 0x18);
    }

    #[test]
    fn hk_display_cylinder_build_geometry_without_sides_has_only_the_two_end_vertices() {
        let mut e = rig();
        let this = cylinder(&mut e, [0.0, 4.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0, 2);
        e.call(0x00d5_8590, &args![this]);
        let block = e.mem.u32(this + 8);
        let points = vertices(&e, block);
        assert_eq!(points.len(), 2);
        close_vec(points[0], [0.0, 0.0, 0.0, 0.0]);
        close_vec(points[1], [0.0, 4.0, 0.0, 0.0]);
        assert!(records(&e, block).is_empty());
        assert!(calls_to(&e, QUATERNION_MULTIPLY).is_empty());
        // Both quaternions are still made; the step angle is 2 * pi / 0 = +infinity.
        let axis_calls = calls_to(&e, QUATERNION_SET_AXIS_ANGLE);
        assert_eq!(axis_calls[0][2], f32::INFINITY.to_bits());
    }

    #[test]
    fn hk_display_cylinder_build_geometry_picks_the_reference_axis_by_the_direction() {
        // Direction along x (|x| > 1/2): the reference axis is (0, 1, 0), so the first spoke
        // is the direction crossed with it, x cross y = z, scaled by the radius: (0, 0, 1).
        let mut e = rig();
        let this = cylinder(&mut e, [2.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 3, 1);
        e.call(0x00d5_8590, &args![this]);
        let block = e.mem.u32(this + 8);
        let points = vertices(&e, block);
        close_vec(points[2], [0.0, 0.0, 1.0, 0.0]);
        // Direction along z: the x and y lanes are both small, the reference axis is (1, 0, 0)
        // and the spoke is z cross x = y: (0, 1, 0).
        let mut e = rig();
        let this = cylinder(&mut e, [0.0, 0.0, 2.0, 0.0], [0.0, 0.0, 0.0, 0.0], 3, 1);
        e.call(0x00d5_8590, &args![this]);
        let block = e.mem.u32(this + 8);
        close_vec(vertices(&e, block)[2], [0.0, 1.0, 0.0, 0.0]);
        // Direction along y: |x| small, |y| above one half, the reference axis is (0, 0, 1):
        // spoke (1, 0, 0), as the full test shows.
        let mut e = rig();
        let this = cylinder(&mut e, [0.0, 2.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 3, 1);
        e.call(0x00d5_8590, &args![this]);
        let block = e.mem.u32(this + 8);
        close_vec(vertices(&e, block)[2], [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn hk_display_cylinder_build_geometry_takes_the_zero_guard_for_a_zero_step() {
        // Both ends equal: the step is zero, its inverse length is the guarded 0 and every
        // lane of the normalised direction is zero (0 * 0).
        let mut e = rig();
        let this = cylinder(&mut e, [1.0, 1.0, 1.0, 0.0], [1.0, 1.0, 1.0, 0.0], 2, 1);
        e.call(0x00d5_8590, &args![this]);
        let block = e.mem.u32(this + 8);
        let points = vertices(&e, block);
        assert_eq!(points.len(), 2 + 2 * 2);
        close_vec(points[0], [1.0, 1.0, 1.0, 0.0]);
        close_vec(points[1], [1.0, 1.0, 1.0, 0.0]);
    }
}
