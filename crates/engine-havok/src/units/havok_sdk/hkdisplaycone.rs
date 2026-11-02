//! `Havok SDK/hkdisplaycone.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplayCone` (Xbox PDB) keeps, as far as these functions read it: the point every side
//! fans out from (16-byte vector at `+0x60`), a direction vector at `+0x70` (the axis the
//! rings of vertices turn about), the number of sides (`int` at `+0x80`), a float at `+0x84`
//! (the angle the first vertex direction is tilted away from the axis by) and a float at
//! `+0x88` (the scale of each vertex offset from the point). The shared base layout is
//! described in `hkdisplaygeometry`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdisplaygeometry::allocate_geometry_block;
use super::hkpprimitivedrawer::{normalised, rotate_by_quaternion};
use super::hkpsphereshape::{copy_vec, load_vec, store_vec, zip_with, Vec4};

/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// Reserves room for one more element (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW_BY_ONE: u32 = 0x00c9_0940;
/// `hkQuaternion::setAxisAngle` (`00cb2450`, `this` = the quaternion, `(axis, angle)`, `RET 8`).
const QUATERNION_SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// The double `2 * pi`.
const FULL_TURN: u32 = 0x0101_ff48;

/// The locals of `generateConeVertices` passed by address: the unit vector perpendicular to
/// the direction and the two quaternions made from it.
const FRAME_SIZE: u32 = 0x30;
const PERPENDICULAR: u32 = 0x00;
const TILT_ROTATION: u32 = 0x10;
const STEP_ROTATION: u32 = 0x20;

/// An x87 `FLD dword` / `FABS`: the magnitude of the float with the given bits, a signalling
/// NaN made quiet.
fn magnitude(bits: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(bits) & 0x7fff_ffff)
}

// Translated from 00de54b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCone::generateConeVertices` (Xbox PDB), `this` in `ECX`, one stack word: the
/// output vertex array. The array is reserved (`00c908c0`, element size 0x10, with the memory
/// router's allocator) to the larger of twice its capacity and the side count when its
/// capacity is below the count, and its size set to the count.
///
/// The perpendicular is built from the direction `d` at `+0x70` (lanes `d0 d1 d2`), whose
/// magnitudes (`FLD`/`FABS`) are compared as the x87 does (a NaN is never "greater"). `m`
/// starts as `|d0|`. If `m > |d1|` then `m = |d1|`, the first lane is 0 and the alternative
/// second lane is 1; otherwise the first lane is 1 and the alternative is 0. The second lane
/// is 2, unless `m > |d2|`, when it is the alternative. A zero vector then gets
/// `perp[first] = d[second]` and `perp[second] = -d[first]` and is normalised.
/// `setAxisAngle` makes the tilt quaternion about it with the angle at `+0x84` and the step
/// quaternion about `d` itself with `2 * pi / sides`. The first direction is `d` rotated by
/// the tilt; for each side the vertex is `apex + scale * direction` (apex `+0x60`, scale
/// `+0x88`) and the direction is then rotated by the step quaternion.
pub fn hk_display_cone_generate_cone_vertices(e: &mut Engine, this: Ptr, out: Ptr) {
    let me = this.addr();
    let array = out.addr();
    let count = e.mem.i32(me + 0x80);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < count {
        let doubled = capacity.wrapping_add(capacity) as i32;
        let new_capacity = count.max(doubled) as u32;
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, 0x10u32],
        );
    }
    e.mem.set_i32(array + 4, count);
    let direction_bits: [u32; 3] = std::array::from_fn(|i| e.mem.u32(me + 0x70 + 4 * i as u32));
    let first_magnitude = magnitude(direction_bits[0]);
    let second_magnitude = magnitude(direction_bits[1]);
    let third_magnitude = magnitude(direction_bits[2]);
    let mut smallest = first_magnitude;
    let mut first_lane = 1usize;
    let mut other_lane = 0usize;
    if smallest > second_magnitude {
        smallest = second_magnitude;
        first_lane = 0;
        other_lane = 1;
    }
    let mut second_lane = 2usize;
    if smallest > third_magnitude {
        second_lane = other_lane;
    }
    let mut perpendicular: Vec4 = [0.0; 4];
    perpendicular[first_lane] = x87_quiet(e.mem.f32(me + 0x70 + 4 * second_lane as u32));
    perpendicular[second_lane] = -x87_quiet(e.mem.f32(me + 0x70 + 4 * first_lane as u32));
    let tilt_angle = f32::from_bits(x87_quiet_bits(e.mem.u32(me + 0x84)));
    let perpendicular = normalised(e, perpendicular);
    let step_angle = (e.global::<f64>(FULL_TURN) / f64::from(count)) as f32;
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        store_vec(e, frame + PERPENDICULAR, perpendicular);
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![frame + TILT_ROTATION, frame + PERPENDICULAR, tilt_angle],
        );
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![frame + STEP_ROTATION, me + 0x70, step_angle],
        );
        let tilt = load_vec(e, frame + TILT_ROTATION);
        let axis = load_vec(e, me + 0x70);
        let mut direction = rotate_by_quaternion(e, tilt, axis);
        if count > 0 {
            let mut side = 0i32;
            loop {
                let data = e.mem.u32(array);
                let slot = data.wrapping_add((side as u32).wrapping_mul(16));
                copy_vec(e, me + 0x60, slot);
                let scale = e.mem.f32(me + 0x88);
                let apex = load_vec(e, slot);
                let vertex = zip_with(direction.map(|lane| scale * lane), apex, |a, b| a + b);
                store_vec(e, slot, vertex);
                let step = load_vec(e, frame + STEP_ROTATION);
                direction = rotate_by_quaternion(e, step, direction);
                side = side.wrapping_add(1);
                if side >= e.mem.i32(me + 0x80) {
                    break;
                }
            }
        }
    });
}

// Translated from 00de57a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCone::buildGeometry` (Xbox PDB), `this` in `ECX`, no stack words. Builds the
/// geometry block (`+8`): the cone vertices (`generateConeVertices`, into the block's vertex
/// array), then the apex (`+0x60`) as one more vertex (index = the side count; the array grows
/// by one through `00c90940` when full), then the index array (`+0xc` of the block) reserved to
/// the larger of twice its capacity and the side count when the capacity is below it, with
/// its size set to the side count and the records `(sides, i + 1, i, -1)` for `i` below
/// `sides - 1` followed by `(sides, 0, sides - 1, -1)` (or `(sides, 0, 0, -1)` when there is no
/// loop). The exe leaves a pointer in `EAX`; that is not a result.
pub fn hk_display_cone_build_geometry(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let block = allocate_geometry_block(e);
    e.mem.set_u32(me + 8, block);
    hk_display_cone_generate_cone_vertices(e, this, Ptr::new(block));
    let vertices = e.mem.u32(me + 8);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if e.mem.u32(vertices + 4) == e.mem.u32(vertices + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW_BY_ONE, &args![allocator, vertices, 0x10u32]);
    }
    let slot = e
        .mem
        .u32(vertices + 4)
        .wrapping_mul(16)
        .wrapping_add(e.mem.u32(vertices));
    copy_vec(e, me + 0x60, slot);
    let counted = e.mem.u32(vertices + 4).wrapping_add(1);
    e.mem.set_u32(vertices + 4, counted);
    let indices = e.mem.u32(me + 8).wrapping_add(0xc);
    let count = e.mem.i32(me + 0x80);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(indices + 8) & 0x3fff_ffff;
    if (capacity as i32) < count {
        let doubled = capacity.wrapping_add(capacity) as i32;
        let new_capacity = count.max(doubled) as u32;
        e.call(
            ARRAY_RESERVE,
            &args![allocator, indices, new_capacity, 0x10u32],
        );
    }
    e.mem.set_i32(indices + 4, count);
    let mut made = 0i32;
    if count.wrapping_sub(1) > 0 {
        let mut offset = 0u32;
        loop {
            let data = e.mem.u32(e.mem.u32(me + 8).wrapping_add(0xc));
            let record = data.wrapping_add(offset);
            e.mem.set_i32(record, e.mem.i32(me + 0x80));
            e.mem.set_i32(record + 8, made);
            e.mem.set_i32(record + 4, made.wrapping_add(1));
            e.mem.set_i32(record + 0xc, -1);
            made = made.wrapping_add(1);
            let limit = e.mem.i32(me + 0x80).wrapping_sub(1);
            offset = offset.wrapping_add(0x10);
            if made >= limit {
                break;
            }
        }
    }
    let data = e.mem.u32(e.mem.u32(me + 8).wrapping_add(0xc));
    let last = (made as u32).wrapping_mul(16).wrapping_add(data);
    e.mem.set_i32(last + 0xc, -1);
    e.mem.set_i32(last, e.mem.i32(me + 0x80));
    e.mem.set_u32(last + 4, 0);
    e.mem.set_i32(last + 8, made);
}

// Translated from 00de58c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCone::getWireframeGeometry` (Xbox PDB), `this` in `ECX`, two stack words: the
/// output vertex array and the allocator it is reserved with. The cone vertices are generated
/// into a temporary `hkArray` whose first 256 elements live in a 0x1000-byte block of this
/// frame (capacity word `0x80000100`, not owned; the exe grows its stack with `00ec8660` for
/// it). The output is reserved (`00c908c0`, element size 0x10, the caller's allocator) to the
/// larger of twice its capacity and four times the side count when its capacity is smaller,
/// the size set to that count, and for each side `i` four vertices are written: the apex,
/// vertex `i`, vertex `i` again and vertex `(i + 1) % sides`. Finally the temporary array's
/// size is cleared and, unless its capacity word has the not-owned bit, its memory is returned
/// through the memory router's allocator (slot `0x10`, `(data, capacity * 16)`).
pub fn hk_display_cone_get_wireframe_geometry(e: &mut Engine, this: Ptr, out: Ptr, allocator: u32) {
    // The temporary array header sits 0x14 bytes into the frame, its inline storage 0x10
    // bytes after it.
    const FRAME: u32 = 0x1024;
    const HEADER: u32 = 0x14;
    const INLINE: u32 = 0x24;
    let me = this.addr();
    e.with_stack(FRAME, |e, frame| {
        let header = frame.addr() + HEADER;
        e.mem.set_u32(header, frame.addr() + INLINE);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x8000_0100);
        hk_display_cone_generate_cone_vertices(e, this, Ptr::new(header));
        let sides = e.mem.i32(me + 0x80);
        let wanted = sides
            .wrapping_add(sides)
            .wrapping_add(sides)
            .wrapping_add(sides);
        let array = out.addr();
        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        if (capacity as i32) < wanted {
            let doubled = capacity.wrapping_add(capacity) as i32;
            let new_capacity = wanted.max(doubled) as u32;
            e.call(
                ARRAY_RESERVE,
                &args![allocator, array, new_capacity, 0x10u32],
            );
        }
        e.mem.set_i32(array + 4, wanted);
        if e.mem.i32(me + 0x80) > 0 {
            let mut side = 0i32;
            loop {
                let slot = e
                    .mem
                    .u32(array)
                    .wrapping_add((side as u32).wrapping_mul(16 * 4));
                let temporary = e.mem.u32(header);
                let here = temporary.wrapping_add((side as u32).wrapping_mul(16));
                copy_vec(e, me + 0x60, slot);
                copy_vec(e, here, slot + 0x10);
                copy_vec(e, here, slot + 0x20);
                side = side.wrapping_add(1);
                let next = side.wrapping_rem(e.mem.i32(me + 0x80));
                let following = e
                    .mem
                    .u32(header)
                    .wrapping_add((next as u32).wrapping_mul(16));
                copy_vec(e, following, slot + 0x30);
                if side >= e.mem.i32(me + 0x80) {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let pool = e.mem.u32(router + 0x10);
        let capacity_word = e.mem.u32(header + 8);
        e.mem.set_u32(header + 4, 0);
        if capacity_word & 0x8000_0000 == 0 {
            let data = e.mem.u32(header);
            let bytes = (capacity_word & 0x3fff_ffff) << 4;
            e.vcall(pool, 0x10, &args![data, bytes]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00de_54b0,
            hk_display_cone_generate_cone_vertices(Ptr, Ptr)
        ),
        entry!(0x00de_57a0, hk_display_cone_build_geometry(Ptr)),
        entry!(
            0x00de_58c0,
            hk_display_cone_get_wireframe_geometry(Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ZERO_F32: u32 = 0x0101_1d78;
    const THREE_F32: u32 = 0x0101_7718;
    const HALF_F32: u32 = 0x0101_6248;
    const ROTATION_BIAS: u32 = 0x010c_71e0;
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0051;
    const FREE_SLOT_DOUBLE: u32 = 0x0900_0052;

    type AxisCalls = Rc<RefCell<Vec<(Vec4, u32)>>>;

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

    /// The constants, a router whose allocator hands out zeroed blocks (slot 4) and takes
    /// them back (slot 0x10), a reserve and a grow-by-one that move the array to a fresh
    /// buffer (clearing the flag bits), and a `setAxisAngle` that records its axis and angle.
    fn rig() -> (Engine, AxisCalls) {
        let mut e = engine();
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(FULL_TURN, std::f64::consts::TAU);
        for lane in 0..4 {
            e.mem.set_f32(ROTATION_BIAS + 4 * lane, -0.5);
        }
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
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
        e.register_double(FREE_SLOT_DOUBLE, |_, _| Ret::default());
        e.register_double(ARRAY_RESERVE, |e, a| {
            move_array(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register_double(ARRAY_GROW_BY_ONE, |e, a| {
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff).max(2) * 2;
            move_array(e, a[1], capacity, a[2]);
            Ret::default()
        });
        let calls: AxisCalls = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&calls);
        e.register_double(QUATERNION_SET_AXIS_ANGLE, move |e, a| {
            let axis = load_vec(e, a[1]);
            record.borrow_mut().push((axis, a[2]));
            let (sin, cos) = (f64::from(f32::from_bits(a[2])) / 2.0).sin_cos();
            let sin = sin as f32;
            store_vec(
                e,
                a[0],
                [axis[0] * sin, axis[1] * sin, axis[2] * sin, cos as f32],
            );
            Ret::default()
        });
        (e, calls)
    }

    /// Moves an array to a fresh buffer of `capacity` elements, keeping its contents.
    fn move_array(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let old = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let fresh = e.mem.alloc(capacity * element);
        if size > 0 {
            let bytes = e.mem.bytes(old, size * element);
            e.mem.write(fresh, &bytes);
        }
        e.mem.set_u32(array, fresh);
        e.mem.set_u32(array + 8, capacity);
    }

    const APEX: Vec4 = [1.0, 2.0, 3.0, 0.0];

    /// A cone object: apex, direction, side count, tilt angle and scale.
    fn cone(e: &mut Engine, direction: Vec4, sides: i32) -> u32 {
        let this = e.mem.alloc(0x90);
        store_vec(e, this + 0x60, APEX);
        store_vec(e, this + 0x70, direction);
        e.mem.set_i32(this + 0x80, sides);
        e.mem.set_f32(this + 0x84, 0.5);
        e.mem.set_f32(this + 0x88, 2.0);
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

    /// Compares the first three lanes: the fourth lane of a rotated direction is whatever the
    /// rotation formula leaves there (it uses the quaternion's w), not a position.
    fn close_xyz(actual: Vec4, expected: Vec4) {
        for lane in 0..3 {
            close(actual[lane], expected[lane]);
        }
    }

    /// The vertices of a cone whose direction is the z axis: the apex plus twice the unit
    /// vector tilted by 0.5 away from z, turned about z by a quarter turn per side step.
    fn ring(sides: i32) -> Vec<Vec4> {
        (0..sides)
            .map(|side| {
                let turn = side as f32 * std::f32::consts::TAU / sides as f32;
                let (tilt_sin, tilt_cos) = 0.5f32.sin_cos();
                [
                    APEX[0] + 2.0 * tilt_sin * turn.cos(),
                    APEX[1] + 2.0 * tilt_sin * turn.sin(),
                    APEX[2] + 2.0 * tilt_cos,
                    0.0,
                ]
            })
            .collect()
    }

    fn vertices(e: &Engine, array: u32) -> Vec<Vec4> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| load_vec(e, data + 0x10 * i))
            .collect()
    }

    #[test]
    fn hk_display_cone_generate_cone_vertices_turns_the_tilted_direction_about_the_axis() {
        let (mut e, axis_calls) = rig();
        let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], 4);
        let out = e.mem.alloc(0x10);
        e.call(0x00de_54b0, &args![this, out]);
        // Reserved to the side count (twice the empty capacity is 0), 0x10 bytes an element.
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][1..], [out, 4, 0x10]);
        let got = vertices(&e, out);
        assert_eq!(got.len(), 4);
        for (actual, expected) in got.into_iter().zip(ring(4)) {
            close_xyz(actual, expected);
        }
        // The tilt is about the perpendicular (0, 1, -0, 0) with the angle at +0x84, the
        // step about the direction itself with a quarter turn.
        let calls = axis_calls.borrow();
        assert_eq!(calls.len(), 2);
        close_vec(calls[0].0, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(calls[0].1, 0.5f32.to_bits());
        assert_eq!(calls[1].0, [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(calls[1].1, ((std::f64::consts::TAU / 4.0) as f32).to_bits());
        // The step quaternion is made about the direction in the object (`+0x70`).
        assert_eq!(calls_to(&e, QUATERNION_SET_AXIS_ANGLE)[1][1], this + 0x70);
    }

    #[test]
    fn hk_display_cone_generate_cone_vertices_quiets_a_signalling_tilt_angle() {
        let (mut e, axis_calls) = rig();
        let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], 3);
        e.mem.set_u32(this + 0x84, 0x7f80_0001);
        let out = e.mem.alloc(0x10);
        e.call(0x00de_54b0, &args![this, out]);
        assert_eq!(axis_calls.borrow()[0].1, 0x7fc0_0001);
    }

    /// The perpendicular for the direction, as the first `setAxisAngle` axis.
    #[allow(clippy::let_and_return)] // the `Ref` must drop before the `Rc` it borrows
    fn perpendicular_for(direction: Vec4) -> Vec4 {
        let (mut e, axis_calls) = rig();
        let this = cone(&mut e, direction, 3);
        let out = e.mem.alloc(0x10);
        e.call(0x00de_54b0, &args![this, out]);
        let first = axis_calls.borrow()[0].0;
        first
    }

    #[test]
    fn hk_display_cone_generate_cone_vertices_picks_the_lane_pair_from_the_magnitudes() {
        let norm = |v: Vec4| {
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / length, v[1] / length, v[2] / length, 0.0]
        };
        // |d0| <= |d1| and |d2| not below |d0|: lanes (1, 2): perp[1] = d2, perp[2] = -d1.
        close_vec(
            perpendicular_for([1.0, 2.0, 3.0, 0.0]),
            norm([0.0, 3.0, -2.0, 0.0]),
        );
        // |d0| <= |d1| but |d2| below |d0|: lanes (1, 0): perp[1] = d0, perp[0] = -d1.
        close_vec(
            perpendicular_for([1.0, 2.0, 0.5, 0.0]),
            norm([-2.0, 1.0, 0.0, 0.0]),
        );
        // |d0| > |d1| and |d2| not below |d1|: lanes (0, 2): perp[0] = d2, perp[2] = -d0.
        close_vec(
            perpendicular_for([3.0, 1.0, 2.0, 0.0]),
            norm([2.0, 0.0, -3.0, 0.0]),
        );
        // |d0| > |d1| and |d2| below |d1|: lanes (0, 1): perp[0] = d1, perp[1] = -d0.
        close_vec(
            perpendicular_for([3.0, 1.0, 0.5, 0.0]),
            norm([1.0, -3.0, 0.0, 0.0]),
        );
        // A NaN magnitude is never "greater": the default lanes (1, 2).
        close_vec(
            perpendicular_for([f32::NAN, 1.0, 1.0, 0.0]),
            norm([0.0, 1.0, -1.0, 0.0]),
        );
    }

    #[test]
    fn hk_display_cone_generate_cone_vertices_reserves_the_larger_of_double_and_the_count() {
        for (capacity_word, sides, expected) in [
            (3u32, 4i32, Some(6u32)),
            (1, 8, Some(8)),
            (0x8000_0000 | 4, 4, None),
            (9, 4, None),
            (0xc000_0000, 0, None),
        ] {
            let (mut e, _) = rig();
            let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], sides);
            let out = e.mem.alloc(0x10);
            let data = e.mem.alloc(0x100);
            e.mem.set_u32(out, data);
            e.mem.set_u32(out + 8, capacity_word);
            e.call(0x00de_54b0, &args![this, out]);
            let reserves = calls_to(&e, ARRAY_RESERVE);
            match expected {
                Some(capacity) => {
                    assert_eq!(reserves.len(), 1, "{capacity_word:#x}");
                    assert_eq!(reserves[0][2], capacity);
                }
                None => assert!(reserves.is_empty(), "{capacity_word:#x}"),
            }
            assert_eq!(e.mem.i32(out + 4), sides);
        }
    }

    #[test]
    fn hk_display_cone_build_geometry_makes_vertices_the_apex_and_the_fan_of_records() {
        let (mut e, _) = rig();
        let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], 4);
        e.call(0x00de_57a0, &args![this]);
        let block = e.mem.u32(this + 8);
        let got = vertices(&e, block);
        assert_eq!(got.len(), 5);
        for (actual, expected) in got.iter().zip(ring(4)) {
            close_xyz(*actual, expected);
        }
        // The apex follows the ring vertices; the array was full, so it grew by one element.
        assert_eq!(got[4], APEX);
        let grows = calls_to(&e, ARRAY_GROW_BY_ONE);
        assert_eq!(grows.len(), 1);
        assert_eq!(grows[0][1..], [block, 0x10]);
        let data = e.mem.u32(block + 0xc);
        assert_eq!(e.mem.u32(block + 0x10), 4);
        let records: Vec<[i32; 4]> = (0..4)
            .map(|i| std::array::from_fn(|lane| e.mem.i32(data + 0x10 * i + 4 * lane as u32)))
            .collect();
        assert_eq!(
            records,
            vec![[4, 1, 0, -1], [4, 2, 1, -1], [4, 3, 2, -1], [4, 0, 3, -1]]
        );
        // The index array was reserved for the four records.
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 2);
        assert_eq!(reserves[1][1..], [block + 0xc, 4, 0x10]);
        let allocations = calls_to(&e, ALLOCATE_SLOT_DOUBLE);
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0][1], 0x18);
    }

    #[test]
    fn hk_display_cone_build_geometry_with_one_side_has_the_single_closing_record() {
        let (mut e, _) = rig();
        let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], 1);
        e.call(0x00de_57a0, &args![this]);
        let block = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(block + 4), 2);
        let data = e.mem.u32(block + 0xc);
        assert_eq!(e.mem.u32(block + 0x10), 1);
        let record: Vec<i32> = (0..4).map(|lane| e.mem.i32(data + 4 * lane)).collect();
        assert_eq!(record, vec![1, 0, 0, -1]);
    }

    #[test]
    fn hk_display_cone_get_wireframe_geometry_writes_four_vertices_per_side() {
        let (mut e, _) = rig();
        let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], 4);
        let out = e.mem.alloc(0x10);
        e.call(0x00de_58c0, &args![this, out, 0x6666u32]);
        let got = vertices(&e, out);
        assert_eq!(got.len(), 16);
        let ring = ring(4);
        for side in 0..4 {
            close_xyz(got[4 * side], APEX);
            close_xyz(got[4 * side + 1], ring[side]);
            close_xyz(got[4 * side + 2], ring[side]);
            close_xyz(got[4 * side + 3], ring[(side + 1) % 4]);
        }
        // The output was reserved for 4 * 4 elements with the allocator the caller passed;
        // the temporary array is the one the cone vertices went into and is not freed.
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0], vec![0x6666, out, 16, 0x10]);
        assert!(calls_to(&e, FREE_SLOT_DOUBLE).is_empty());
    }

    #[test]
    fn hk_display_cone_get_wireframe_geometry_frees_a_temporary_that_outgrew_its_block() {
        let (mut e, _) = rig();
        let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], 300);
        let out = e.mem.alloc(0x10);
        e.call(0x00de_58c0, &args![this, out, 0x6666u32]);
        assert_eq!(e.mem.u32(out + 4), 1200);
        // The temporary was reserved from 256 to 512 elements by the vertex generator and
        // is returned with its data pointer and size in bytes.
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves[0][2], 512);
        assert_eq!(reserves[1][1..], [out, 1200, 0x10]);
        let frees = calls_to(&e, FREE_SLOT_DOUBLE);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][2], 512 * 0x10);
    }

    #[test]
    fn hk_display_cone_get_wireframe_geometry_does_not_reserve_an_array_with_room() {
        let (mut e, _) = rig();
        let this = cone(&mut e, [0.0, 0.0, 1.0, 0.0], 2);
        let out = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10 * 8);
        e.mem.set_u32(out, data);
        e.mem.set_u32(out + 8, 0x8000_0008);
        e.call(0x00de_58c0, &args![this, out, 0x6666u32]);
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        assert_eq!(e.mem.u32(out), data);
        assert_eq!(e.mem.u32(out + 4), 8);
    }
}
