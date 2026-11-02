//! `Havok SDK/hkpprimitivedrawer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkp3axissweep::MEMORY_ROUTER;
use super::hkpsphereshape::{
    copy_vec, inverse_length, load_vec, shufps, store_vec, sum3, zip_with, Vec4,
};

/// Byte offset of the display handler's line-drawing slot: `(from, to, three words)`, the
/// callee pops the five words.
const LINE_SLOT: u32 = 0x24;
/// `hkRotation::setAxisAngle` (`00cb2e30`, `this` = the rotation, `(axis, angle)`, `RET 8`).
const ROTATION_SET_AXIS_ANGLE: u32 = 0x00cb_2e30;
/// `hkVector4::setRotatedDir` (`00c7f700`, `this` = the result, `(rotation, vector)`, `RET 8`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkQuaternion::setAxisAngle` (`00cb2450`, `this` = the quaternion, `(axis, angle)`,
/// `RET 8`).
const QUATERNION_SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// `hkMemoryRouter` heap allocation (`00c85690`, cdecl `(allocator, size)`).
const EASY_ALLOC: u32 = 0x00c8_5690;
/// `hkMemoryRouter` heap release (`00c856c0`, cdecl `(allocator, block)`).
const EASY_FREE: u32 = 0x00c8_56c0;
/// The vector of four `-0.5` the quaternion rotation adds to `w * w`.
const ROTATION_BIAS: u32 = 0x010c_71e0;
/// The float `pi / 4`.
const EIGHTH_TURN: u32 = 0x0106_4f60;
/// The float `-pi / 2`.
const NEGATIVE_QUARTER_TURN: u32 = 0x0101_6b78;
/// The float `pi / 2`.
const QUARTER_TURN: u32 = 0x0101_ff38;
/// The double `2 * pi`.
const FULL_TURN: u32 = 0x0101_ff48;
/// The double the arrow head's length is scaled by (about one third).
const ARROW_HEAD_SCALE: u32 = 0x010c_4ab0;
/// The double the plane's size is scaled by (the square root of one half).
const PLANE_SCALE: u32 = 0x010e_acc0;

/// A frame is addressed like the exe's: an offset from the stack pointer once the frame is
/// set up and the three registers pushed, which is `0xC` below the frame's base.
const fn esp(offset: u32) -> u32 {
    offset - 0xc
}

/// The line-drawing virtual call on the drawer's display handler (the drawer's first word).
fn draw_line(e: &mut Engine, drawer: u32, from: u32, to: u32, words: [u32; 3]) {
    let handler = e.mem.u32(drawer);
    e.vcall(
        handler,
        LINE_SLOT,
        &args![from, to, words[0], words[1], words[2]],
    );
}

/// Copies the float at `from` to `to` through the x87 stack (a quiet copy).
fn copy_quiet(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// An angle constant loaded onto the x87 stack and stored again as an argument.
fn angle_constant(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.global::<f32>(address))
}

/// The vector scaled to unit length by the refined reciprocal square root, all four lanes.
pub(crate) fn normalised(e: &Engine, v: Vec4) -> Vec4 {
    let factor = inverse_length(e, sum3(zip_with(v, v, |a, b| a * b)));
    v.map(|lane| factor * lane)
}

/// [`normalised`] with the SSE NaN rules: `MULPS v, v`, the `ADDSS` sum (`z + (y + x)`, the
/// higher lane the destination) and `MULPS` with the splatted factor as the destination.
pub(crate) fn normalised_sse(e: &Engine, v: Vec4) -> Vec4 {
    use crate::sse::{add, mulps};
    let squares = mulps(v, v);
    let factor = inverse_length(e, add(squares[2], add(squares[1], squares[0])));
    mulps([factor; 4], v)
}

/// Rotates `v` by the quaternion `q`: `2 * (w * (q' x v) + (w * w - 0.5) * v + (q . v) * q)`,
/// where `q'` is the quaternion's vector part, all four lanes.
pub(crate) fn rotate_by_quaternion(e: &Engine, q: Vec4, v: Vec4) -> Vec4 {
    let bias = load_vec(e, ROTATION_BIAS);
    let dot = sum3(zip_with(q, v, |a, b| a * b));
    let forward = zip_with(shufps(q, q, 0xc9), shufps(v, v, 0xd2), |a, b| a * b);
    let backward = zip_with(shufps(q, q, 0xd2), shufps(v, v, 0xc9), |a, b| a * b);
    let w = q[3];
    let cross = zip_with(forward, backward, |a, b| a - b).map(|lane| lane * w);
    let ww = w * w;
    let rest: Vec4 = std::array::from_fn(|lane| (ww + bias[lane]) * v[lane] + dot * q[lane]);
    let sum = zip_with(cross, rest, |a, b| a + b);
    sum.map(|lane| lane + lane)
}

// Translated from 00e10a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrimitiveDrawer::setDisplayHandler`: the drawer's first word becomes the handler.
pub fn hkp_primitive_drawer_set_display_handler(e: &mut Engine, this: Ptr, handler: Ptr) {
    e.mem.set_u32(this.addr(), handler.addr());
}

/// The rotation block, the running point and the scratch vector of the semicircle's frame.
const SEMI_ROTATION: u32 = esp(0x30);
const SEMI_CURRENT: u32 = esp(0x10);
const SEMI_NEXT: u32 = esp(0x20);

// Translated from 00e10a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrimitiveDrawer::drawSemiCircle` (`this` in `ECX`, ten stack words that the callee pops).
/// The start direction and the axis are normalised in place; the start direction scaled by the
/// radius is rotated about the axis by the start angle to give the first point
/// (`center + rotated`); the angle step is `(end - start) / segments`; then, `segments` times,
/// the point's offset from the center is rotated by the step, the next point is
/// `center + rotated`, and the display handler draws the line from the current point to it
/// with the last three words.
pub fn hkp_primitive_drawer_draw_semi_circle(
    e: &mut Engine,
    this: Ptr,
    center: Ptr,
    axis: Ptr,
    start: Ptr,
    start_angle: f32,
    end_angle: f32,
    radius: f32,
    segments: i32,
    line_word_0: u32,
    line_word_1: u32,
    line_word_2: u32,
) {
    let words = [line_word_0, line_word_1, line_word_2];
    e.with_stack(0x54, |e, frame| {
        let f = frame.addr();
        let start_angle = x87_quiet(start_angle);
        let direction = load_vec(e, start.addr());
        store_vec(e, start.addr(), normalised(e, direction));
        let axis_vector = load_vec(e, axis.addr());
        store_vec(e, axis.addr(), normalised(e, axis_vector));
        e.call(
            ROTATION_SET_AXIS_ANGLE,
            &args![f + SEMI_ROTATION, axis, start_angle],
        );
        let direction = load_vec(e, start.addr());
        store_vec(e, f + SEMI_CURRENT, direction.map(|lane| radius * lane));
        e.call(
            SET_ROTATED_DIR,
            &args![f + SEMI_CURRENT, f + SEMI_ROTATION, f + SEMI_CURRENT],
        );
        let span = f64::from(x87_quiet(end_angle)) - f64::from(start_angle);
        let step = (span / f64::from(segments)) as f32;
        let rotated = load_vec(e, f + SEMI_CURRENT);
        let center_vector = load_vec(e, center.addr());
        store_vec(
            e,
            f + SEMI_CURRENT,
            zip_with(center_vector, rotated, |a, b| a + b),
        );
        // The step is also stored in a scratch slot of the frame before it is passed.
        e.mem.set_f32(f + esp(0xc), step);
        e.call(
            ROTATION_SET_AXIS_ANGLE,
            &args![f + SEMI_ROTATION, axis, x87_quiet(step)],
        );
        for _ in 0..segments {
            let current = load_vec(e, f + SEMI_CURRENT);
            let center_vector = load_vec(e, center.addr());
            store_vec(
                e,
                f + SEMI_NEXT,
                zip_with(current, center_vector, |a, b| a - b),
            );
            e.call(
                SET_ROTATED_DIR,
                &args![f + SEMI_NEXT, f + SEMI_ROTATION, f + SEMI_NEXT],
            );
            let rotated = load_vec(e, f + SEMI_NEXT);
            let center_vector = load_vec(e, center.addr());
            store_vec(
                e,
                f + SEMI_NEXT,
                zip_with(center_vector, rotated, |a, b| a + b),
            );
            draw_line(e, this.addr(), f + SEMI_CURRENT, f + SEMI_NEXT, words);
            copy_vec(e, f + SEMI_NEXT, f + SEMI_CURRENT);
        }
    });
}

// Translated from 00e10bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`this` in `ECX`, six stack words that the callee pops): draws three
/// lines through the position, one along each row of the rotation, each from
/// `position - scale * row` to `position - (-scale) * row`, with the last three words.
pub fn fn_00e10bf0(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    rotation: Ptr,
    scale: f32,
    line_word_0: u32,
    line_word_1: u32,
    line_word_2: u32,
) {
    let words = [line_word_0, line_word_1, line_word_2];
    const FROM: u32 = esp(0x30);
    const TO: u32 = esp(0x20);
    const NEGATED: u32 = esp(0x18);
    const NEGATED_COPY: u32 = esp(0x1c);
    const ROW_1: u32 = esp(0x40);
    const ROW_2: u32 = esp(0x50);
    e.with_stack(0x54, |e, frame| {
        let f = frame.addr();
        let negated = -x87_quiet(scale);
        e.mem.set_f32(f + NEGATED, negated);
        copy_quiet(e, f + NEGATED, f + NEGATED_COPY);
        let first_row = load_vec(e, rotation.addr());
        let second_row = load_vec(e, rotation.addr() + 0x10);
        store_vec(e, f + ROW_1, second_row);
        let third_row = load_vec(e, rotation.addr() + 0x20);
        store_vec(e, f + ROW_2, third_row);
        for index in 0..3 {
            let row = match index {
                0 => first_row,
                1 => load_vec(e, f + ROW_1),
                _ => load_vec(e, f + ROW_2),
            };
            if index > 0 {
                copy_quiet(e, f + NEGATED, f + NEGATED_COPY);
            }
            let origin = load_vec(e, position.addr());
            store_vec(e, f + FROM, zip_with(origin, row, |a, b| a - scale * b));
            let negated_copy = e.mem.f32(f + NEGATED_COPY);
            store_vec(
                e,
                f + TO,
                zip_with(origin, row, |a, b| a - negated_copy * b),
            );
            draw_line(e, this.addr(), f + FROM, f + TO, words);
        }
    });
}

// Translated from 00e10d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrimitiveDrawer::displayArrow` (`this` in `ECX`, seven stack words that the callee
/// pops). The arrow's end is `length * direction + from`. Two quaternions about `axis` (by
/// `pi / 4` and `-pi / 2`) turn the direction into the two barbs; each barb runs from the end
/// to `end + m * rotated direction`, with `m = -length * 0.333...` (the second barb rotates
/// the first). All three lines are drawn by the display handler with the last three words.
pub fn hkp_primitive_drawer_display_arrow(
    e: &mut Engine,
    this: Ptr,
    from: Ptr,
    direction: Ptr,
    axis: Ptr,
    line_word_0: u32,
    length: f32,
    line_word_1: u32,
    line_word_2: u32,
) {
    const END: u32 = esp(0x20);
    const BARB: u32 = esp(0x30);
    const QUATERNION_1: u32 = esp(0x40);
    const QUATERNION_2: u32 = esp(0x50);
    const ROTATED: u32 = esp(0x60);
    const SCALE: u32 = esp(0x18);
    const SCALE_COPY: u32 = esp(0x1c);
    let words = [line_word_0, line_word_1, line_word_2];
    e.with_stack(0x64, |e, frame| {
        let f = frame.addr();
        let direction_vector = load_vec(e, direction.addr());
        let start = load_vec(e, from.addr());
        let end: Vec4 = std::array::from_fn(|lane| length * direction_vector[lane] + start[lane]);
        store_vec(e, f + END, end);
        draw_line(e, this.addr(), from.addr(), f + END, words);
        let angle = angle_constant(e, EIGHTH_TURN);
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![f + QUATERNION_1, axis, angle],
        );
        let angle = angle_constant(e, NEGATIVE_QUARTER_TURN);
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![f + QUATERNION_2, axis, angle],
        );
        let negated_length = -f64::from(x87_quiet(length));
        let quaternion = load_vec(e, f + QUATERNION_1);
        let direction_vector = load_vec(e, direction.addr());
        let rotated = rotate_by_quaternion(e, quaternion, direction_vector);
        store_vec(e, f + ROTATED, rotated);
        let head_scale: f64 = e.global(ARROW_HEAD_SCALE);
        e.mem
            .set_f32(f + SCALE, (negated_length * head_scale) as f32);
        copy_quiet(e, f + SCALE, f + SCALE_COPY);
        let scale = e.mem.f32(f + SCALE_COPY);
        let end = load_vec(e, f + END);
        let barb: Vec4 = std::array::from_fn(|lane| scale * rotated[lane] + end[lane]);
        store_vec(e, f + BARB, barb);
        draw_line(e, this.addr(), f + END, f + BARB, words);
        copy_quiet(e, f + SCALE, f + SCALE_COPY);
        let quaternion = load_vec(e, f + QUATERNION_2);
        let rotated = load_vec(e, f + ROTATED);
        let rotated = rotate_by_quaternion(e, quaternion, rotated);
        let scale = e.mem.f32(f + SCALE_COPY);
        let end = load_vec(e, f + END);
        let barb: Vec4 = std::array::from_fn(|lane| scale * rotated[lane] + end[lane]);
        store_vec(e, f + BARB, barb);
        draw_line(e, this.addr(), f + END, f + BARB, words);
    });
}

// Translated from 00e10f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrimitiveDrawer::displayCone` (`this` in `ECX`, nine stack words that the callee
/// pops). Takes `segments * 16` bytes from the heap allocator, tilts the cone axis by `angle`
/// about `tilt_axis`, and turns it about the cone axis in steps of `2 * pi / segments`;
/// point `k` is `apex + (length / angle) * (the k-th turned vector)`. Then, for each point,
/// draws a line from the apex to it and one from it to the next point (the last one wraps to the
/// first), with the three words, and frees the block.
pub fn hkp_primitive_drawer_display_cone(
    e: &mut Engine,
    this: Ptr,
    angle: f32,
    apex: Ptr,
    cone_axis: Ptr,
    tilt_axis: Ptr,
    segments: i32,
    line_word_0: u32,
    length: f32,
    line_word_1: u32,
    line_word_2: u32,
) {
    const TILT: u32 = esp(0x20);
    const STEP: u32 = esp(0x30);
    let words = [line_word_0, line_word_1, line_word_2];
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e
        .call(EASY_ALLOC, &args![allocator, (segments as u32) << 4])
        .u32();
    e.with_stack(0x34, |e, frame| {
        let f = frame.addr();
        let angle = x87_quiet(angle);
        let ratio = (f64::from(x87_quiet(length)) / f64::from(angle)) as f32;
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![f + TILT, tilt_axis, angle],
        );
        let full_turn: f64 = e.global(FULL_TURN);
        let step = (full_turn / f64::from(segments)) as f32;
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![f + STEP, cone_axis, x87_quiet(step)],
        );
        let tilt = load_vec(e, f + TILT);
        let axis_vector = load_vec(e, cone_axis.addr());
        let mut turned = rotate_by_quaternion(e, tilt, axis_vector);
        let mut slot = block;
        for _ in 0..segments {
            let origin = load_vec(e, apex.addr());
            store_vec(e, slot, origin);
            let scaled: Vec4 = turned.map(|lane| ratio * lane);
            let stored = load_vec(e, slot);
            store_vec(e, slot, zip_with(scaled, stored, |a, b| a + b));
            let step_rotation = load_vec(e, f + STEP);
            turned = rotate_by_quaternion(e, step_rotation, turned);
            slot += 0x10;
        }
        for index in 0..segments {
            let point = block + ((index as u32) << 4);
            draw_line(e, this.addr(), apex.addr(), point, words);
            let next = block + (((index + 1) % segments) as u32) * 0x10;
            draw_line(e, this.addr(), point, next, words);
        }
    });
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(EASY_FREE, &args![allocator, block]);
}

// Translated from 00e11180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPrimitiveDrawer::displayPlane` (`this` in `ECX`, seven stack words that the callee
/// pops). The vector is scaled by `size * 0.7071...`, turned by `pi / 4` about the axis, and the
/// four corners are `origin + vector` with the vector turned by `pi / 2` about the axis after
/// each. The display handler draws the four sides and the two diagonals (corners 0-1, 1-2,
/// 2-3, 3-0, 0-2, 1-3) with the last three words; then the last turned vector is normalised and
/// an arrow of that length `size` is drawn from the origin (`displayArrow` with the axis as the
/// direction and the normalised vector as its axis).
pub fn hkp_primitive_drawer_display_plane(
    e: &mut Engine,
    this: Ptr,
    origin: Ptr,
    axis: Ptr,
    vector: Ptr,
    line_word_0: u32,
    size: f32,
    line_word_1: u32,
    line_word_2: u32,
) {
    const SCALED: u32 = esp(0x20);
    const CORNERS: u32 = esp(0x30);
    const QUARTER: u32 = esp(0xa0);
    const EIGHTH: u32 = esp(0x70);
    const ORIGIN_COPY: u32 = esp(0x90);
    let words = [line_word_0, line_word_1, line_word_2];
    e.with_stack(0xa4, |e, frame| {
        let f = frame.addr();
        let plane_scale: f64 = e.global(PLANE_SCALE);
        let scale = (f64::from(x87_quiet(size)) * plane_scale) as f32;
        e.mem.set_f32(f + esp(0x1c), scale);
        let scale = e.mem.f32(f + esp(0x1c));
        let input = load_vec(e, vector.addr());
        let angle = angle_constant(e, QUARTER_TURN);
        e.mem.set_u32(f + esp(0x18), this.addr());
        store_vec(e, f + SCALED, input.map(|lane| scale * lane));
        e.call(QUATERNION_SET_AXIS_ANGLE, &args![f + QUARTER, axis, angle]);
        let angle = angle_constant(e, EIGHTH_TURN);
        e.call(QUATERNION_SET_AXIS_ANGLE, &args![f + EIGHTH, axis, angle]);
        let eighth = load_vec(e, f + EIGHTH);
        let scaled = load_vec(e, f + SCALED);
        let mut turned = rotate_by_quaternion(e, eighth, scaled);
        let start = load_vec(e, origin.addr());
        store_vec(e, f + ORIGIN_COPY, start);
        let quarter = load_vec(e, f + QUARTER);
        for corner in 0..4 {
            let slot = f + CORNERS + 0x10 * corner;
            copy_vec(e, f + ORIGIN_COPY, slot);
            let stored = load_vec(e, slot);
            store_vec(e, slot, zip_with(stored, turned, |a, b| a + b));
            turned = rotate_by_quaternion(e, quarter, turned);
        }
        store_vec(e, f + SCALED, turned);
        let corner = |index: u32| f + CORNERS + 0x10 * index;
        for (from, to) in [(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)] {
            draw_line(e, this.addr(), corner(from), corner(to), words);
        }
        let last = load_vec(e, f + SCALED);
        let direction = normalised(e, last);
        store_vec(e, f + SCALED, direction);
        hkp_primitive_drawer_display_arrow(
            e,
            this,
            origin,
            axis,
            Ptr::new(f + SCALED),
            line_word_0,
            x87_quiet(size),
            line_word_1,
            line_word_2,
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e1_0a10,
            hkp_primitive_drawer_set_display_handler(Ptr, Ptr)
        ),
        entry!(
            0x00e1_0a20,
            hkp_primitive_drawer_draw_semi_circle(
                Ptr, Ptr, Ptr, Ptr, f32, f32, f32, i32, u32, u32, u32
            )
        ),
        entry!(0x00e1_0bf0, fn_00e10bf0(Ptr, Ptr, Ptr, f32, u32, u32, u32)),
        entry!(
            0x00e1_0d50,
            hkp_primitive_drawer_display_arrow(Ptr, Ptr, Ptr, Ptr, u32, f32, u32, u32)
        ),
        entry!(
            0x00e1_0f60,
            hkp_primitive_drawer_display_cone(Ptr, f32, Ptr, Ptr, Ptr, i32, u32, f32, u32, u32)
        ),
        entry!(
            0x00e1_1180,
            hkp_primitive_drawer_display_plane(Ptr, Ptr, Ptr, Ptr, u32, f32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod a05_primitive_drawer_tests {
    use super::super::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::RefCell;
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
    use std::rc::Rc;

    const LINE: u32 = 0x0900_0100;
    const HANDLER_VTABLE: u32 = 0x0900_0200;

    /// One line the handler was asked to draw: both end points and the three words.
    type Line = (Vec4, Vec4, [u32; 3]);
    type LineLog = Rc<RefCell<Vec<Line>>>;

    /// The rotation about `axis` by `angle` as three rows (row `i` is the `i`-th row of the
    /// matrix that maps a column vector).
    fn rotation_rows(axis: Vec4, angle: f32) -> [Vec4; 3] {
        let [x, y, z] = [f64::from(axis[0]), f64::from(axis[1]), f64::from(axis[2])];
        let (s, c) = f64::from(angle).sin_cos();
        let t = 1.0 - c;
        let rows = [
            [c + x * x * t, x * y * t - z * s, x * z * t + y * s],
            [x * y * t + z * s, c + y * y * t, y * z * t - x * s],
            [x * z * t - y * s, y * z * t + x * s, c + z * z * t],
        ];
        rows.map(|row| [row[0] as f32, row[1] as f32, row[2] as f32, 0.0])
    }

    /// An engine with the constants mapped, a handler whose line drawing is logged, and doubles
    /// for the quaternion and rotation setters and for `setRotatedDir` that do real rotations.
    fn world() -> (Engine, u32, LineLog) {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_6000,
            0x0101_7000,
            0x0101_f000,
            0x0106_4000,
            0x010c_4000,
            0x010c_7000,
            0x010e_a000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_u32(EIGHTH_TURN, 0x3f49_0fdb);
        e.mem.set_u32(NEGATIVE_QUARTER_TURN, 0xbfc9_0fdb);
        e.mem.set_u32(QUARTER_TURN, 0x3fc9_0fdb);
        e.mem
            .set_f64(FULL_TURN, f64::from_bits(0x4019_21fb_6000_0000));
        e.mem
            .set_f64(ARROW_HEAD_SCALE, f64::from_bits(0x3fd5_4fdf_4000_0000));
        e.mem
            .set_f64(PLANE_SCALE, f64::from_bits(0x3fe6_a0a5_2000_0000));
        for lane in 0..4 {
            e.mem.set_f32(ROTATION_BIAS + 4 * lane, -0.5);
        }
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
        e.register_double(ROTATION_SET_AXIS_ANGLE, |e, a| {
            let axis = load_vec(e, a[1]);
            for (index, row) in rotation_rows(axis, f32::from_bits(a[2]))
                .into_iter()
                .enumerate()
            {
                store_vec(e, a[0] + 0x10 * index as u32, row);
            }
            Ret::default()
        });
        e.register_double(SET_ROTATED_DIR, |e, a| {
            let vector = load_vec(e, a[2]);
            let out: Vec4 = std::array::from_fn(|lane| {
                if lane < 3 {
                    let row = load_vec(e, a[1] + 0x10 * lane as u32);
                    sum3(zip_with(row, vector, |x, y| x * y))
                } else {
                    0.0
                }
            });
            store_vec(e, a[0], out);
            Ret::default()
        });
        let handler = e.mem.alloc(8);
        e.put_vtable(HANDLER_VTABLE, &[0, 0, 0, 0, 0, 0, 0, 0, 0, LINE]);
        e.mem.set_u32(handler, HANDLER_VTABLE);
        let drawer = e.mem.alloc(8);
        e.mem.set_u32(drawer, handler);
        let log = LineLog::default();
        let sink = log.clone();
        e.register_double(LINE, move |e, a| {
            assert_eq!(a[0], handler);
            sink.borrow_mut()
                .push((load_vec(e, a[1]), load_vec(e, a[2]), [a[3], a[4], a[5]]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, drawer, log)
    }

    fn vector(e: &mut Engine, v: Vec4) -> Ptr {
        let at = e.mem.alloc(16);
        store_vec(e, at, v);
        Ptr::new(at)
    }

    fn assert_near(actual: Vec4, expected: [f32; 3], tolerance: f32) {
        for (lane, (a, x)) in actual.iter().zip(expected).enumerate() {
            assert!(
                (a - x).abs() <= tolerance,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A memory router whose allocator slot is a known token, a heap allocator double that
    /// hands out blocks and a release double; returns the allocator token.
    fn heap(e: &mut Engine) -> u32 {
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(EASY_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[1].max(16)),
            ..Ret::default()
        });
        e.register_double(EASY_FREE, |_, _| Ret::default());
        allocator
    }

    #[test]
    fn set_display_handler_stores_the_handler() {
        let mut e = Engine::new();
        let drawer = e.mem.alloc(8);
        e.call(0x00e1_0a10, &args![drawer, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(drawer), 0x1234_5678);
    }

    #[test]
    fn semi_circle_draws_one_line_per_segment_along_the_arc() {
        let (mut e, drawer, log) = world();
        let center = vector(&mut e, [1.0, 1.0, 1.0, 0.0]);
        let axis = vector(&mut e, [0.0, 0.0, 3.0, 0.0]);
        let start = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00e1_0a20,
            &args![
                drawer,
                center,
                axis,
                start,
                0.5f32,
                0.5f32 + PI,
                2.0f32,
                4i32,
                7u32,
                8u32,
                9u32
            ],
        );
        assert_near(load_vec(&e, start.addr()), [1.0, 0.0, 0.0], 1e-5);
        assert_near(load_vec(&e, axis.addr()), [0.0, 0.0, 1.0], 1e-5);
        let lines = log.borrow();
        assert_eq!(lines.len(), 4);
        let step = f64::from(PI) / 4.0;
        for (index, (from, to, words)) in lines.iter().enumerate() {
            let at = |k: usize| {
                let angle = 0.5 + step * k as f64;
                [
                    1.0 + 2.0 * angle.cos() as f32,
                    1.0 + 2.0 * angle.sin() as f32,
                    1.0,
                ]
            };
            assert_near(*from, at(index), 1e-4);
            assert_near(*to, at(index + 1), 1e-4);
            assert_eq!(*words, [7, 8, 9]);
        }
        let rotations = calls_to(&e, ROTATION_SET_AXIS_ANGLE);
        assert_eq!(rotations.len(), 2);
        assert_eq!(rotations[0][2], 0.5f32.to_bits());
        let span = (f64::from(0.5f32 + PI) - 0.5) / 4.0;
        assert_eq!(rotations[1][2], (span as f32).to_bits());
    }

    #[test]
    fn semi_circle_draws_nothing_without_segments() {
        let (mut e, drawer, log) = world();
        let center = vector(&mut e, [0.0; 4]);
        let axis = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let start = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        for segments in [0i32, -3] {
            e.call(
                0x00e1_0a20,
                &args![
                    drawer, center, axis, start, 0.0f32, 1.0f32, 1.0f32, segments, 0u32, 0u32, 0u32
                ],
            );
        }
        assert!(log.borrow().is_empty());
        assert_eq!(calls_to(&e, SET_ROTATED_DIR).len(), 2);
    }

    #[test]
    fn crosshair_draws_a_line_along_each_row() {
        let (mut e, drawer, log) = world();
        let position = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let rotation = e.mem.alloc(0x30);
        store_vec(&mut e, rotation, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, rotation + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, rotation + 0x20, [0.0, 0.0, 1.0, 0.0]);
        e.call(
            0x00e1_0bf0,
            &args![drawer, position, rotation, 2.0f32, 4u32, 5u32, 6u32],
        );
        let lines = log.borrow();
        assert_eq!(lines.len(), 3);
        let expected = [
            ([-1.0, 2.0, 3.0, 0.0], [3.0, 2.0, 3.0, 0.0]),
            ([1.0, 0.0, 3.0, 0.0], [1.0, 4.0, 3.0, 0.0]),
            ([1.0, 2.0, 1.0, 0.0], [1.0, 2.0, 5.0, 0.0]),
        ];
        for ((from, to, words), (want_from, want_to)) in lines.iter().zip(expected) {
            assert_eq!(*from, want_from);
            assert_eq!(*to, want_to);
            assert_eq!(*words, [4, 5, 6]);
        }
    }

    #[test]
    fn arrow_draws_the_shaft_and_two_barbs() {
        let (mut e, drawer, log) = world();
        let from = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let direction = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let half = std::f32::consts::FRAC_1_SQRT_2;
        let axis = vector(&mut e, [half, half, 0.0, 0.0]);
        e.call(
            0x00e1_0d50,
            &args![drawer, from, direction, axis, 11u32, 2.0f32, 12u32, 13u32],
        );
        let lines = log.borrow();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].0, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lines[0].1, [1.0, 2.0, 5.0, 0.0]);
        let scale = (-2.0f64 * f64::from_bits(0x3fd5_4fdf_4000_0000)) as f32;
        let barbs = [[0.5, -0.5, half], [-0.5, 0.5, half]];
        for (line, barb) in lines[1..].iter().zip(barbs) {
            assert_eq!(line.0, [1.0, 2.0, 5.0, 0.0]);
            assert_near(
                line.1,
                [
                    1.0 + scale * barb[0],
                    2.0 + scale * barb[1],
                    5.0 + scale * barb[2],
                ],
                1e-5,
            );
        }
        assert!(lines.iter().all(|line| line.2 == [11, 12, 13]));
        let quaternions = calls_to(&e, QUATERNION_SET_AXIS_ANGLE);
        assert_eq!(quaternions.len(), 2);
        assert_eq!(quaternions[0][2], FRAC_PI_4.to_bits());
        assert_eq!(quaternions[1][2], (-FRAC_PI_2).to_bits());
    }

    #[test]
    fn cone_draws_a_spoke_and_a_rim_line_for_each_point() {
        let (mut e, drawer, log) = world();
        let allocator = heap(&mut e);
        let apex = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let cone_axis = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let tilt_axis = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00e1_0f60,
            &args![drawer, 0.5f32, apex, cone_axis, tilt_axis, 4i32, 21u32, 2.0f32, 22u32, 23u32],
        );
        let ratio = 4.0f32;
        let (sin, cos) = (0.5f32.sin(), 0.5f32.cos());
        let point = |k: usize| {
            let (a, b) = (std::f32::consts::FRAC_PI_2 * k as f32).sin_cos();
            [
                1.0 + ratio * sin * a,
                2.0 - ratio * sin * b,
                3.0 + ratio * cos,
            ]
        };
        let lines = log.borrow();
        assert_eq!(lines.len(), 8);
        for (index, line) in lines.iter().enumerate() {
            let k = index / 2;
            if index % 2 == 0 {
                assert_eq!(line.0, [1.0, 2.0, 3.0, 0.0]);
                assert_near(line.1, point(k), 1e-4);
            } else {
                assert_near(line.0, point(k), 1e-4);
                assert_near(line.1, point((k + 1) % 4), 1e-4);
            }
            assert_eq!(line.2, [21, 22, 23]);
        }
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![allocator, 64]]);
        let first_point = calls_to(&e, LINE)[0][2];
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![allocator, first_point]]);
        let quaternions = calls_to(&e, QUATERNION_SET_AXIS_ANGLE);
        assert_eq!(quaternions[0][2], 0.5f32.to_bits());
        let step = (f64::from_bits(0x4019_21fb_6000_0000) / 4.0) as f32;
        assert_eq!(quaternions[1][2], step.to_bits());
    }

    #[test]
    fn cone_without_segments_allocates_and_frees_an_empty_block() {
        let (mut e, drawer, log) = world();
        let allocator = heap(&mut e);
        let apex = vector(&mut e, [0.0; 4]);
        let cone_axis = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let tilt_axis = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00e1_0f60,
            &args![drawer, 1.0f32, apex, cone_axis, tilt_axis, 0i32, 0u32, 1.0f32, 0u32, 0u32],
        );
        assert!(log.borrow().is_empty());
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![allocator, 0]]);
        assert_eq!(calls_to(&e, EASY_FREE).len(), 1);
    }

    #[test]
    fn plane_draws_the_square_its_diagonals_and_an_arrow() {
        let (mut e, drawer, log) = world();
        let origin = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let axis = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let side = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00e1_1180,
            &args![drawer, origin, axis, side, 31u32, 2.0f32, 32u32, 33u32],
        );
        let corners = [
            [2.0, 3.0, 3.0],
            [0.0, 3.0, 3.0],
            [0.0, 1.0, 3.0],
            [2.0, 1.0, 3.0],
        ];
        let lines = log.borrow();
        assert_eq!(lines.len(), 9);
        for (line, (from, to)) in lines
            .iter()
            .zip([(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)])
        {
            assert_near(line.0, corners[from], 1e-4);
            assert_near(line.1, corners[to], 1e-4);
        }
        assert_eq!(lines[6].0, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lines[6].1, [1.0, 2.0, 5.0, 0.0]);
        assert!(lines.iter().all(|line| line.2 == [31, 32, 33]));
        let arrow_axis = calls_to(&e, QUATERNION_SET_AXIS_ANGLE);
        assert_eq!(arrow_axis.len(), 4);
        assert_eq!(arrow_axis[0][2], FRAC_PI_2.to_bits());
        assert_eq!(arrow_axis[1][2], FRAC_PI_4.to_bits());
        assert_eq!(arrow_axis[2][1], arrow_axis[3][1]);
        assert_ne!(arrow_axis[2][1], axis.addr());
    }
}
