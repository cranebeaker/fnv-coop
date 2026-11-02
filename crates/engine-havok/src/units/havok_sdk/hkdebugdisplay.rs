//! `Havok SDK/hkdebugdisplay.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::ZERO_VECTOR;
use super::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, maxps, minps, shufps, store_bits, store_vec,
    sum3, zip_with, Vec4, HALF_F32, ZERO_F32,
};
use crate::x87::rsqrt_estimate;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00c99fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c9_9f10`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c99fb0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c9_9f10, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The imported `EnterCriticalSection`, `LeaveCriticalSection` and `DeleteCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// The handler methods the display calls (the Xbox PDB's `hkDebugDisplayHandler` slots, as byte
/// offsets in the handler's vtable): `setGeometryColor` `0xc`, `updateGeometry` `0x10`,
/// `removeGeometry` `0x14`, `updateCamera` `0x18`, `updateBehavior` `0x1c`, `displayPoint` `0x20`,
/// `displayLine` `0x24`, `displayTriangle` `0x28`, `displayText` `0x2c`, `display3dText` `0x30`,
/// `displayGeometry` `0x38` and `0x3c`.
const SLOT_SET_GEOMETRY_COLOR: u32 = 0xc;
const SLOT_UPDATE_GEOMETRY: u32 = 0x10;
const SLOT_REMOVE_GEOMETRY: u32 = 0x14;
const SLOT_UPDATE_CAMERA: u32 = 0x18;
const SLOT_UPDATE_BEHAVIOR: u32 = 0x1c;
const SLOT_DISPLAY_POINT: u32 = 0x20;
const SLOT_DISPLAY_LINE: u32 = 0x24;
const SLOT_DISPLAY_TRIANGLE: u32 = 0x28;
const SLOT_DISPLAY_TEXT: u32 = 0x2c;
const SLOT_DISPLAY_3D_TEXT: u32 = 0x30;
const SLOT_DISPLAY_GEOMETRY: u32 = 0x38;
const SLOT_DISPLAY_GEOMETRY_WITH_TAG: u32 = 0x3c;
/// `hkQsTransform` made from an `hkTransform` (`00ce52e0`, `this` = the 0x30-byte result, one
/// word: the source), `setTransformedPos_ov2` (`00c7f7a0`, `this` = the result vector, `(transform,
/// point)`) and `setTransformedPos` (`00c7f640`, same shape; the transform's translation row
/// is `+0x30`).
const QS_TRANSFORM_FROM_TRANSFORM: u32 = 0x00ce_52e0;
const TRANSFORM_POINT: u32 = 0x00c7_f7a0;
const TRANSFORM_POINT_BY_ROWS: u32 = 0x00c7_f640;
/// The `hkDebugDisplay` functions that draw through each other.
const DISPLAY_LINE: u32 = 0x00c9_86c0;
const DISPLAY_TRANSFORMED_LINE: u32 = 0x00c9_8780;
const DISPLAY_ARROW: u32 = 0x00c9_8880;
const DISPLAY_AXES: u32 = 0x00c9_8b60;
const DISPLAY_TRIANGLE: u32 = 0x00c9_8720;
const DISPLAY_ROTATED_LINE: u32 = 0x00c9_9610;
const DISPLAY_AXIS_ARROWS: u32 = 0x00c9_9720;
const DISPLAY_BONES: u32 = 0x00c9_98b0;
const DISPLAY_OFFSET_LINE: u32 = 0x00c9_8820;
/// The floats the arrow code reads: the squared-length threshold below which nothing is drawn
/// (`0104da4c`), the fraction of the shaft where the head starts (`01019de0`) and the head's
/// half-width factor and its negation (`010c69c4`, `010c69c0`), the constants of the refined
/// reciprocal root and the zero (`01011d78`, `01017718`, `01016248`), and the vector of ones
/// (`010ec7f0`).
const ARROW_MINIMUM_LENGTH_SQUARED: u32 = 0x0104_da4c;
const ARROW_HEAD_START: u32 = 0x0101_9de0;
const ARROW_HEAD_WIDTH: u32 = 0x010c_69c4;
const ARROW_HEAD_WIDTH_NEGATED: u32 = 0x010c_69c0;
const ONES_VECTOR: u32 = 0x010e_c7f0;
/// The vector of `-0.5` the rotation adds to the squared quaternion scalar (`010c71e0`), the
/// absolute-value mask (`01017d10`) and `hkArray::_reserve`'s grow-by-one (`00c90940`, cdecl
/// `(allocator, array, element size)`).
const ROTATION_BIAS: u32 = 0x010c_71e0;
const ABS_MASK: u32 = 0x0101_7d10;
const ARRAY_GROW_BY_ONE: u32 = 0x00c9_0940;

layout! {
    /// `hkDebugDisplay` (Xbox PDB): the handlers it forwards to (a pointer array with its count and
    /// capacity word) and the critical section that guards them.
    pub struct HkDebugDisplay: 0x18 {
        /// The array of `hkDebugDisplayHandler` pointers.
        0x08 handlers: Ptr,
        /// How many handlers there are.
        0x0c handlerCount: i32,
        /// The array's capacity, with its flag bits in the top two bits.
        0x10 handlerCapacity: u32,
        /// The `CRITICAL_SECTION` (0x18 bytes) that guards the handlers.
        0x14 lock: Ptr,
    }
}

/// The locked loop every forwarding method runs: enters the critical section, calls handler
/// slot `slot` of each handler with `words` (the handler is `this` and the count is read again
/// after each call) and leaves the critical section.
fn forward_to_handlers(e: &mut Engine, this: Ptr<HkDebugDisplay>, slot: u32, words: &[u32]) {
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let mut index = 0i32;
    while e.get(this, HkDebugDisplay::handlerCount) > index {
        let handlers = e.get(this, HkDebugDisplay::handlers).addr();
        let handler = e
            .mem
            .u32(handlers.wrapping_add((index as u32).wrapping_mul(4)));
        e.vcall(handler, slot, words);
        index = index.wrapping_add(1);
    }
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
}

// Translated from 00c984a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::setGeometryColor` (Xbox PDB), thiscall, `RET 0xc`: handler slot `0xc` of each
/// handler with the three arguments, under the lock.
pub fn hk_debug_display_set_geometry_color(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
) {
    forward_to_handlers(e, this, SLOT_SET_GEOMETRY_COLOR, &[a, b, c]);
}

// Translated from 00c984f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::updateGeometry` (Xbox PDB), thiscall, `RET 0xc`: handler slot `0x10`.
pub fn hk_debug_display_update_geometry(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
) {
    forward_to_handlers(e, this, SLOT_UPDATE_GEOMETRY, &[a, b, c]);
}

// Translated from 00c98540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::removeGeometry` (Xbox PDB), thiscall, `RET 0xc`: handler slot `0x14`.
pub fn hk_debug_display_remove_geometry(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
) {
    forward_to_handlers(e, this, SLOT_REMOVE_GEOMETRY, &[a, b, c]);
}

// Translated from 00c98590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::updateCamera` (Xbox PDB), thiscall, `RET 0x1c`: handler slot `0x18` with the
/// seven arguments; the fourth to sixth are floats the exe copies through the x87 stack
/// (`FLD`/`FSTP`), so a signalling NaN reaches the handler quiet.
#[allow(clippy::too_many_arguments)]
pub fn hk_debug_display_update_camera(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
    g: u32,
    h: u32,
) {
    let words = [
        a,
        b,
        c,
        x87_quiet_bits(d),
        x87_quiet_bits(f),
        x87_quiet_bits(g),
        h,
    ];
    forward_to_handlers(e, this, SLOT_UPDATE_CAMERA, &words);
}

// Translated from 00c98600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::updateBehavior` (Xbox PDB), thiscall, `RET 0x20`: handler slot `0x1c` with the
/// eight arguments.
#[allow(clippy::too_many_arguments)]
pub fn hk_debug_display_update_behavior(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
    g: u32,
    h: u32,
    i: u32,
) {
    forward_to_handlers(e, this, SLOT_UPDATE_BEHAVIOR, &[a, b, c, d, f, g, h, i]);
}

// Translated from 00c98670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::displayPoint` (Xbox PDB), thiscall, `RET 0x10`: handler slot `0x20`.
pub fn hk_debug_display_display_point(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    forward_to_handlers(e, this, SLOT_DISPLAY_POINT, &[a, b, c, d]);
}

// Translated from 00c986c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::displayLine` (Xbox PDB), thiscall, `RET 0x14`: handler slot `0x24` with the
/// start, the end, the colour and two ids.
pub fn hk_debug_display_display_line(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
) {
    forward_to_handlers(e, this, SLOT_DISPLAY_LINE, &[a, b, c, d, f]);
}

// Translated from 00c98720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::displayTriangle` (Xbox PDB), thiscall, `RET 0x18`: handler slot `0x28`.
pub fn hk_debug_display_display_triangle(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
    g: u32,
) {
    forward_to_handlers(e, this, SLOT_DISPLAY_TRIANGLE, &[a, b, c, d, f, g]);
}

// Translated from 00c990b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::displayText` (Xbox PDB), thiscall, `RET 0x10`: handler slot `0x2c`.
pub fn hk_debug_display_display_text(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    forward_to_handlers(e, this, SLOT_DISPLAY_TEXT, &[a, b, c, d]);
}

// Translated from 00c99100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::display3dText` (Xbox PDB), thiscall, `RET 0x14`: handler slot `0x30`.
pub fn hk_debug_display_display_3d_text(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
) {
    forward_to_handlers(e, this, SLOT_DISPLAY_3D_TEXT, &[a, b, c, d, f]);
}

// Translated from 00c99160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::displayGeometry` (Xbox PDB, the overload with an id), thiscall, `RET 0x14`:
/// handler slot `0x3c`.
pub fn hk_debug_display_display_geometry_with_tag(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
) {
    forward_to_handlers(e, this, SLOT_DISPLAY_GEOMETRY_WITH_TAG, &[a, b, c, d, f]);
}

// Translated from 00c991c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::displayGeometry` (Xbox PDB), thiscall, `RET 0x10`: handler slot `0x38`.
pub fn hk_debug_display_display_geometry(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    forward_to_handlers(e, this, SLOT_DISPLAY_GEOMETRY, &[a, b, c, d]);
}

// Translated from 00c98820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x14`): draws a line from the vector at `start` to
/// the sum of the vectors at `start` and `offset` (`ADDPS`, kept in a 16-byte local), through
/// handler slot `0x24` with `(start, &sum, colour, id, id)`, under no lock.
pub fn fn_00c98820(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    start: u32,
    offset: u32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x40, |e, block| {
        let sum_at = (block.addr() + 0xf) & !0xf;
        let sum = zip_with(load_vec(e, start), load_vec(e, offset), |a, b| a + b);
        store_vec(e, sum_at, sum);
        let mut index = 0i32;
        while e.get(this, HkDebugDisplay::handlerCount) > index {
            let handlers = e.get(this, HkDebugDisplay::handlers).addr();
            let handler = e
                .mem
                .u32(handlers.wrapping_add((index as u32).wrapping_mul(4)));
            e.vcall(
                handler,
                SLOT_DISPLAY_LINE,
                &args![start, sum_at, colour, id_a, id_b],
            );
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c98780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x18`): transforms two points by the transform
/// (`00c7f7a0` into two 16-byte locals) and draws the line between them (`00c986c0`).
pub fn fn_00c98780(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    from: u32,
    to: u32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x40, |e, block| {
        let base = (block.addr() + 0xf) & !0xf;
        let (first, second) = (base + 0x10, base);
        e.call(TRANSFORM_POINT, &args![first, transform, from]);
        e.call(TRANSFORM_POINT, &args![second, transform, to]);
        e.call(
            DISPLAY_LINE,
            &args![this, first, second, colour, id_a, id_b],
        );
    });
}

// Translated from 00c987e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x18`): makes an `hkQsTransform` from the
/// transform (`00ce52e0` into a 0x30-byte local) and draws the transformed line (`00c98780`).
pub fn fn_00c987e0(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    from: u32,
    to: u32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x50, |e, block| {
        let local = (block.addr() + 0xf) & !0xf;
        e.call(QS_TRANSFORM_FROM_TRANSFORM, &args![local, transform]);
        e.call(
            DISPLAY_TRANSFORMED_LINE,
            &args![this, local, from, to, colour, id_a, id_b],
        );
    });
}

// Translated from 00c98880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x14`): the arrow. Draws nothing when the squared
/// length of `direction` is below the threshold `0104da4c` (the compare is `!(threshold >
/// squared)`, so NaN draws). Otherwise it picks the two axes of `direction` other than its
/// smallest, builds a vector perpendicular to `direction` from them (the first axis's
/// component of `direction` in the second axis's slot and the second's negated in the first's)
/// and normalises it (reciprocal root estimate refined once, zero when the squared length is
/// zero). The shaft runs from `start` to `end = start + direction`; the head is four lines from
/// `end` to the points where the head starts (85% of the way: the blend of `start` and `end`),
/// moved by plus and minus 0.15 of `direction`'s length along the perpendicular and along
/// its cross product with `direction`, each drawn with `(colour, id, id)` through
/// `00c986c0`. The x87 stack is left with a stray value in the exe; nothing reads it.
pub fn fn_00c98880(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    start: u32,
    direction: u32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    let d = load_vec(e, direction);
    let squared = zip_with(d, d, |a, b| a * b);
    let length_squared = sum3(squared);
    let threshold: f32 = e.global(ARROW_MINIMUM_LENGTH_SQUARED);
    if threshold > length_squared {
        return;
    }
    let lane_abs = |e: &Engine, lane: u32| x87_quiet(e.mem.f32(direction + 4 * lane)).abs();
    let (abs_x, abs_y, abs_z) = (lane_abs(e, 0), lane_abs(e, 1), lane_abs(e, 2));
    // The exe keeps three indices in EBX, ESI and EDI: the smaller of the x and y axes, the
    // other one of the two, and the z axis; the z axis replaces the smaller one's index when
    // it is smaller still.
    let (mut smaller_xy, mut other_xy, mut third) = (0usize, 1usize, 2usize);
    let mut smallest = abs_x;
    if abs_x > abs_y {
        smallest = abs_y;
        other_xy = 0;
        smaller_xy = third - 1;
    }
    if smallest > abs_z {
        third = smaller_xy;
    }
    let mut perpendicular = [0.0f32; 4];
    perpendicular[other_xy] = x87_quiet(e.mem.f32(direction + 4 * third as u32));
    perpendicular[third] = -x87_quiet(e.mem.f32(direction + 4 * other_xy as u32));
    let inverse = inverse_length(
        e,
        sum3(zip_with(perpendicular, perpendicular, |a, b| a * b)),
    );
    let unit = perpendicular.map(|v| v * inverse);
    let a = load_vec(e, start);
    let end = zip_with(a, d, |x, y| x + y);
    let cross = zip_with(
        zip_with(shufps(d, d, 0xc9), shufps(unit, unit, 0xd2), |x, y| x * y),
        zip_with(shufps(d, d, 0xd2), shufps(unit, unit, 0xc9), |x, y| x * y),
        |x, y| x - y,
    );
    let length = sum3(squared).sqrt();
    let along = unit.map(|v| length * v);
    let head_start: f32 = e.global(ARROW_HEAD_START);
    let width: f32 = e.global(ARROW_HEAD_WIDTH);
    let width_negated: f32 = e.global(ARROW_HEAD_WIDTH_NEGATED);
    let ones = load_vec(e, ONES_VECTOR);
    let base_point = zip_with(
        zip_with(ones.map(|v| v - head_start), a, |x, y| x * y),
        end.map(|v| head_start * v),
        |x, y| x + y,
    );
    let corner =
        |side: f32, offset: Vec4| zip_with(offset.map(|v| side * v), base_point, |x, y| x + y);
    let corners = [
        corner(width, along),
        corner(width_negated, along),
        corner(width, cross),
        corner(width_negated, cross),
    ];
    e.with_stack(0x80, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        store_vec(e, frame + 0x20, end);
        for (index, point) in corners.into_iter().enumerate() {
            store_vec(e, frame + 0x30 + 0x10 * index as u32, point);
        }
        e.call(
            DISPLAY_LINE,
            &args![this, start, frame + 0x20, colour, id_a, id_b],
        );
        for index in 0..4u32 {
            let head = frame + 0x30 + 0x10 * index;
            e.call(
                DISPLAY_LINE,
                &args![this, frame + 0x20, head, colour, id_a, id_b],
            );
        }
    });
}

// Translated from 00c98b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x14`): draws the three coordinate axes through
/// the point at `centre`: for each axis a line from `centre + e_i * size` to `centre - e_i *
/// size` (`00c986c0` with `colour` and the two ids), the unit vector `e_i * size` being built
/// in a 16-byte local by storing the size (a float the exe copies through the x87 stack) into
/// a zeroed vector.
pub fn fn_00c98b60(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    centre: u32,
    size: f32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x80, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let (axis_at, minus_at, plus_at) = (frame + 0x10, frame + 0x30, frame + 0x40);
        for lane in 0..3u32 {
            store_vec(e, axis_at, [0.0; 4]);
            e.mem.set_f32(axis_at + 4 * lane, x87_quiet(size));
            let a = load_vec(e, centre);
            let axis = load_vec(e, axis_at);
            store_vec(e, plus_at, zip_with(a, axis, |x, y| x + y));
            store_vec(e, minus_at, zip_with(a, axis, |x, y| x - y));
            e.call(
                DISPLAY_LINE,
                &args![this, plus_at, minus_at, colour, id_a, id_b],
            );
        }
    });
}

// Translated from 00c98be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x18`): the axes through a transformed point:
/// `setTransformedPos_ov2` (`00c7f7a0`) puts the transform applied to `point` into a 16-byte
/// local and `00c98b60` draws the axes through it.
pub fn fn_00c98be0(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    point: u32,
    size: f32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x40, |e, block| {
        let local = (block.addr() + 0xf) & !0xf;
        e.call(TRANSFORM_POINT, &args![local, transform, point]);
        e.call(
            DISPLAY_AXES,
            &args![this, local, x87_quiet(size), colour, id_a, id_b],
        );
    });
}

// Translated from 00c98c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x18`): as `00c98be0` with `setTransformedPos`
/// (`00c7f640`, the transform's rows with its translation row at `+0x30`).
pub fn fn_00c98c30(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    point: u32,
    size: f32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x40, |e, block| {
        let local = (block.addr() + 0xf) & !0xf;
        e.call(TRANSFORM_POINT_BY_ROWS, &args![local, transform, point]);
        e.call(
            DISPLAY_AXES,
            &args![this, local, x87_quiet(size), colour, id_a, id_b],
        );
    });
}

/// The twelve edges of a box, as the corners (bit 0 selects the maximum x, bit 1 the maximum y,
/// bit 2 the maximum z) they join, in the order the exe draws them.
const BOX_EDGES: [(u32, u32); 12] = [
    (0b000, 0b010),
    (0b000, 0b100),
    (0b000, 0b001),
    (0b111, 0b011),
    (0b111, 0b110),
    (0b111, 0b101),
    (0b010, 0b011),
    (0b010, 0b110),
    (0b011, 0b001),
    (0b110, 0b100),
    (0b100, 0b101),
    (0b101, 0b001),
];

// Translated from 00c98c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x10`): draws the twelve edges of an axis-aligned
/// box (its minimum at `box_at + 0`, its maximum at `box_at + 0x10`, three floats each). The
/// exe builds 24 corner vectors (the box's floats, bit for bit, and the float `01011d78` as the
/// fourth lane) in a 0x180-byte frame and draws consecutive pairs of them with `00c986c0`.
pub fn fn_00c98c80(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    box_at: u32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    let fourth = e.mem.u32(ZERO_F32);
    let minimum: [u32; 3] = std::array::from_fn(|lane| e.mem.u32(box_at + 4 * lane as u32));
    let maximum: [u32; 3] = std::array::from_fn(|lane| e.mem.u32(box_at + 0x10 + 4 * lane as u32));
    e.with_stack(0x1f0, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let corner = |code: u32| -> [u32; 4] {
            let pick = |lane: usize| {
                if code >> lane & 1 != 0 {
                    maximum[lane]
                } else {
                    minimum[lane]
                }
            };
            [pick(0), pick(1), pick(2), fourth]
        };
        for (index, (from, to)) in BOX_EDGES.into_iter().enumerate() {
            let pair = frame + 0x50 + 0x20 * index as u32;
            store_bits(e, pair, corner(from));
            store_bits(e, pair + 0x10, corner(to));
        }
        for index in 0..12u32 {
            let pair = frame + 0x50 + 0x20 * index;
            e.call(
                DISPLAY_LINE,
                &args![this, pair, pair + 0x10, colour, id_a, id_b],
            );
        }
    });
}

pub(crate) fn add_lanes(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

pub(crate) fn sub_lanes(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

pub(crate) fn mul_lanes(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// `first.zxy * second.yzx - first.yzx * second.zxy` per lane: the shuffle-and-multiply cross
/// product the exe builds with `SHUFPS` `0xd2`/`0xc9`, `MULPS` and `SUBPS` (it is the usual
/// `second x first`).
pub(crate) fn cross_rows(first: Vec4, second: Vec4) -> Vec4 {
    sub_lanes(
        mul_lanes(shufps(first, first, 0xd2), shufps(second, second, 0xc9)),
        mul_lanes(shufps(first, first, 0xc9), shufps(second, second, 0xd2)),
    )
}

// Translated from 00c98e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x18`): draws a plane. `plane` is the normal
/// (three floats) with the offset in its fourth; `point` is a point to project: the centre
/// is `point - offset * normal`. The normal's largest axis (by `|x|`, `|y|`, `|z|`, with the
/// ordered `>` tests of the `FCOM`s) picks the axis after it as a helper vector; two edge
/// vectors of length `size` are made from it with the raw `RSQRTSS` estimate (no refinement:
/// the host's estimate stands in for the CPU's, so results can differ in the last bit) and the
/// rectangle of the four corners centre `+/-` first edge `+/-` second edge is drawn as four
/// lines (`00c986c0`, `colour`, the two ids), then an arrow (`00c98880`) from the centre
/// along `size * normal`.
pub fn fn_00c98e60(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    plane: u32,
    point: u32,
    size: f32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    let normal = load_vec(e, plane);
    let negated_offset = -x87_quiet(e.mem.f32(plane + 0xc));
    let centre = add_lanes(normal.map(|v| negated_offset * v), load_vec(e, point));
    let mask = load_bits(e, ABS_MASK);
    let lane_abs = |lane: usize| f32::from_bits(normal[lane].to_bits() & mask[lane]);
    let (abs_x, abs_y, abs_z) = (lane_abs(0), lane_abs(1), lane_abs(2));
    let largest = if abs_y > abs_x {
        if abs_z > abs_y {
            2
        } else {
            1
        }
    } else if abs_z > abs_x {
        2
    } else {
        0
    };
    let mut helper = [0.0f32; 4];
    helper[(largest + 1) % 3] = 1.0;
    let first_row = cross_rows(helper, normal);
    let first_estimate = rsqrt_estimate(sum3(mul_lanes(first_row, first_row)));
    let first_edge = first_row.map(|v| (first_estimate * v) * size);
    let second_row = cross_rows(normal, first_edge);
    let second_estimate = rsqrt_estimate(sum3(mul_lanes(second_row, second_row)));
    let second_edge = second_row.map(|v| (second_estimate * v) * size);
    let scaled_normal = normal.map(|v| size * v);
    e.with_stack(0x80, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let (first_at, second_at, centre_at, normal_at) =
            (frame + 0x40, frame + 0x30, frame + 0x50, frame + 0x60);
        store_vec(e, centre_at, centre);
        for step in 1..=4i32 {
            let mut first = if (step - 1) & 2 != 0 {
                sub_lanes(centre, second_edge)
            } else {
                add_lanes(second_edge, centre)
            };
            let first_sign = (step & 3) >> 1;
            first = if first_sign == 0 {
                add_lanes(first, first_edge)
            } else {
                sub_lanes(first, first_edge)
            };
            store_vec(e, first_at, first);
            let mut second = if first_sign != 0 {
                sub_lanes(centre, second_edge)
            } else {
                add_lanes(second_edge, centre)
            };
            second = if ((step + 1) & 3) >> 1 == 0 {
                add_lanes(second, first_edge)
            } else {
                sub_lanes(second, first_edge)
            };
            store_vec(e, second_at, second);
            e.call(
                DISPLAY_LINE,
                &args![this, first_at, second_at, colour, id_a, id_b],
            );
        }
        store_vec(e, normal_at, scaled_normal);
        e.call(
            DISPLAY_ARROW,
            &args![this, centre_at, normal_at, colour, id_a, id_b],
        );
    });
}

/// The floats `hkDebugDisplay::displayLitTriangle` reads, as global addresses: the two light
/// directions (`(0.5, 2, 1)` from `01016248`, `010162c0` and `01012068`; `(-3, -2, -1)` from
/// `0104fe20`, `0104a494` and `01012054`), the diffuse factor `010177e8`, the ambient colour
/// components (`010181bc` and `010c6850`), the factors `01016264` and `010181bc` of the sum,
/// the double `010231e8` (1/255) and the clamp vectors (`01267e30` for zero, `01012068` for
/// one).
const LIGHT_ONE: [u32; 3] = [0x0101_6248, 0x0101_62c0, 0x0101_2068];
const LIGHT_TWO: [u32; 3] = [0x0104_fe20, 0x0104_a494, 0x0101_2054];
const LIGHT_STRENGTH: u32 = 0x0101_77e8;
const AMBIENT_GREEN: u32 = 0x0101_81bc;
const AMBIENT_BLUE: u32 = 0x010c_6850;
const KEY_SCALE: u32 = 0x0101_6264;
const BYTE_TO_UNIT: u32 = 0x0102_31e8;
const ONE_F32: u32 = 0x0101_2068;
/// `hkColor::rgbFromFloats` (`00cabb30`, cdecl, four floats: red, green, blue, alpha).
const RGB_FROM_FLOATS: u32 = 0x00ca_bb30;
/// The pointer to the debug display the lit triangle draws through.
const DEBUG_DISPLAY_SINGLETON: u32 = 0x0126_81e0;

// Translated from 00c99210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::displayLitTriangle`-style helper (unnamed in the engine map), cdecl, `RET
/// 0x10`: draws the triangle `a`, `b`, `c` (pointers to vectors) with its colour `argb` lit by
/// two fixed lights through the display singleton's `00c98720` (colour, ids 0 and 0). The
/// normal is the normalised `(c - a) x (b - a)` shuffle product (refined root, zero for a
/// zero-length product); each light direction is normalised the same way; the channels are
/// the colour's bytes times `1/255` (the x87 product in double precision rounded to a float
/// at the store). Per lane the colour is `ambient * channel * t2 + 0.75 * channel * t1 +
/// 0.05 * channel`, clamped to `[0, 1]` with `MAXPS`/`MINPS` (the zero vector at `01267e30`
/// and `01012068`), where `t1 = 0.5 * (0.9 * dot(normal, light one)) + 0.5` and `t2` is the
/// same for light two and the ambient is `(0, 0.05, 0.08, 0)`. The alpha is the top byte
/// times `1/255`.
pub fn fn_00c99210(e: &mut Engine, a: u32, b: u32, c: u32, argb: u32) {
    let to_a = |v: Vec4| zip_with(v, load_vec(e, a), |x, y| x - y);
    let first_edge = to_a(load_vec(e, c));
    let second_edge = to_a(load_vec(e, b));
    let product = cross_rows(first_edge, second_edge);
    let normal_scale = inverse_length(e, sum3(mul_lanes(product, product)));
    let normal = product.map(|v| normal_scale * v);
    let zero: f32 = e.global(ZERO_F32);
    let light = |e: &Engine, source: [u32; 3]| -> Vec4 {
        [
            e.global::<f32>(source[0]),
            e.global::<f32>(source[1]),
            e.global::<f32>(source[2]),
            zero,
        ]
    };
    let light_one = light(e, LIGHT_ONE);
    let light_two = light(e, LIGHT_TWO);
    let scale_one = inverse_length(e, sum3(mul_lanes(light_one, light_one)));
    let scale_two = inverse_length(e, sum3(mul_lanes(light_two, light_two)));
    let key: f64 = e.global(BYTE_TO_UNIT);
    let channel = |byte: u32| ((byte as f64) * key) as f32;
    let colour = [
        channel((argb >> 16) & 0xff),
        channel((argb >> 8) & 0xff),
        channel(argb & 0xff),
        zero,
    ];
    let alpha = channel(argb >> 24);
    let strength: f32 = e.global(LIGHT_STRENGTH);
    let half: f32 = e.global(HALF_F32);
    let facing = |scale: f32, direction: Vec4| {
        let lit = mul_lanes(
            mul_lanes(direction.map(|v| scale * v), [strength; 4]),
            normal,
        );
        sum3(lit) * half + half
    };
    let (t_one, t_two) = (facing(scale_one, light_one), facing(scale_two, light_two));
    let ambient = [
        zero,
        e.global::<f32>(AMBIENT_GREEN),
        e.global::<f32>(AMBIENT_BLUE),
        zero,
    ];
    let key_scale: f32 = e.global(KEY_SCALE);
    let key_colour = [key_scale, key_scale, key_scale, zero];
    let fill = [e.global::<f32>(AMBIENT_GREEN); 4];
    let lit = add_lanes(
        add_lanes(
            mul_lanes(mul_lanes(ambient, colour), [t_two; 4]),
            mul_lanes(mul_lanes(key_colour, colour), [t_one; 4]),
        ),
        mul_lanes(fill, colour),
    );
    let floor = load_vec(e, ZERO_VECTOR);
    let ceiling: f32 = e.global(ONE_F32);
    let clamped: Vec4 = std::array::from_fn(|lane| minps(maxps(lit[lane], floor[lane]), ceiling));
    let packed = e
        .call(
            RGB_FROM_FLOATS,
            &args![
                x87_quiet(clamped[0]),
                x87_quiet(clamped[1]),
                x87_quiet(clamped[2]),
                x87_quiet(alpha)
            ],
        )
        .u32();
    let display = e.mem.u32(DEBUG_DISPLAY_SINGLETON);
    e.call(
        DISPLAY_TRIANGLE,
        &args![display, a, b, c, packed, 0u32, 0u32],
    );
}

// Translated from 00c99590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`): removes the handler `handler` from the list
/// under the lock: finds its first index (signed compares, as the exe) and, when found, lowers
/// the count and moves the last handler into the freed slot (nothing moves when the last one
/// was removed). The exe leaves through a tail jump to `LeaveCriticalSection`.
pub fn fn_00c99590(e: &mut Engine, this: Ptr<HkDebugDisplay>, handler: u32) {
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let count = e.get(this, HkDebugDisplay::handlerCount);
    if count > 0 {
        let handlers = e.get(this, HkDebugDisplay::handlers).addr();
        let mut index = 0i32;
        loop {
            if e.mem
                .u32(handlers.wrapping_add((index as u32).wrapping_mul(4)))
                == handler
            {
                let new_count = e.get(this, HkDebugDisplay::handlerCount).wrapping_sub(1);
                e.set(this, HkDebugDisplay::handlerCount, new_count);
                if new_count != index {
                    let handlers = e.get(this, HkDebugDisplay::handlers).addr();
                    let last = e
                        .mem
                        .u32(handlers.wrapping_add((new_count as u32).wrapping_mul(4)));
                    e.mem
                        .set_u32(handlers.wrapping_add((index as u32).wrapping_mul(4)), last);
                }
                break;
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
}

// Translated from 00c995f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no arguments): empties the handler list (count `0`)
/// under the lock.
pub fn fn_00c995f0(e: &mut Engine, this: Ptr<HkDebugDisplay>) {
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.set(this, HkDebugDisplay::handlerCount, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
}

// Translated from 00c99610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x18`): draws a line from the transformed point
/// to that point plus a vector rotated by the transform's rotation. `00c7f7a0` puts the
/// transform applied to `point` into a 16-byte local; the vector at `vector` is rotated by the
/// quaternion at `transform + 0x10` (`2 * (qw * (v x q) + ((qw * qw - 0.5) * v + (v . q) *
/// q))`, the bias being the vector `010c71e0` of `-0.5`) into a second local, and `00c98820`
/// draws from the first to the sum with `colour` and the two ids.
pub fn fn_00c99610(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    point: u32,
    vector: u32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x60, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let (moved_at, rotated_at) = (frame + 0x20, frame + 0x10);
        e.call(TRANSFORM_POINT, &args![moved_at, transform, point]);
        let q = load_vec(e, transform + 0x10);
        let v = load_vec(e, vector);
        let rotated = rotate_vector(e, q, v);
        store_vec(e, rotated_at, rotated);
        e.call(
            DISPLAY_OFFSET_LINE,
            &args![this, moved_at, rotated_at, colour, id_a, id_b],
        );
    });
}

// Translated from 00c996e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x18`): `00c99610` with an `hkQsTransform` made
/// from the transform (`00ce52e0` into a 0x30-byte local).
pub fn fn_00c996e0(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    point: u32,
    vector: u32,
    colour: u32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x50, |e, block| {
        let local = (block.addr() + 0xf) & !0xf;
        e.call(QS_TRANSFORM_FROM_TRANSFORM, &args![local, transform]);
        e.call(
            DISPLAY_ROTATED_LINE,
            &args![this, local, point, vector, colour, id_a, id_b],
        );
    });
}

/// The colours of the x, y and z axes the transform's axes are drawn in.
const AXIS_COLOURS: [u32; 3] = [0xffff_0000, 0xff00_8000, 0xff00_00ff];

// Translated from 00c99720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x10`): draws the axes of a transform as three
/// arrows (`00c98880`) from the transform's origin: the origin and the points `(size, 0, 0)`,
/// `(0, size, 0)` and `(0, 0, size)` (the zero is the float `01011d78`; the fourth lane too)
/// are put through `00c7f7a0` into 16-byte locals, and the arrows run from the origin along
/// each point minus the origin, in the colours `0xffff0000`, `0xff008000` and `0xff0000ff`.
pub fn fn_00c99720(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    size: f32,
    id_a: u32,
    id_b: u32,
) {
    let zero: f32 = e.global(ZERO_F32);
    e.with_stack(0xb0, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let scratch = frame + 0x10;
        let origin_at = frame + 0x20;
        store_vec(e, scratch, [0.0; 4]);
        e.call(TRANSFORM_POINT, &args![origin_at, transform, scratch]);
        for axis in 0..3usize {
            let mut point = [zero; 4];
            point[axis] = size;
            store_vec(e, scratch, point);
            let result_at = origin_at + 0x10 * (axis as u32 + 1);
            e.call(TRANSFORM_POINT, &args![result_at, transform, scratch]);
        }
        let origin = load_vec(e, origin_at);
        for axis in 0..3u32 {
            let point = load_vec(e, origin_at + 0x10 * (axis + 1));
            let offset_at = frame + 0x60 + 0x10 * axis;
            store_vec(e, offset_at, zip_with(point, origin, |a, b| a - b));
        }
        for axis in 0..3u32 {
            let offset_at = frame + 0x60 + 0x10 * axis;
            e.call(
                DISPLAY_ARROW,
                &args![
                    this,
                    origin_at,
                    offset_at,
                    AXIS_COLOURS[axis as usize],
                    id_a,
                    id_b
                ],
            );
        }
    });
}

// Translated from 00c99870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x10`): `00c99720` with an `hkQsTransform` made
/// from the transform (`00ce52e0` into a 0x30-byte local); the size is a float the exe copies
/// through the x87 stack.
pub fn fn_00c99870(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    transform: u32,
    size: f32,
    id_a: u32,
    id_b: u32,
) {
    e.with_stack(0x50, |e, block| {
        let local = (block.addr() + 0xf) & !0xf;
        e.call(QS_TRANSFORM_FROM_TRANSFORM, &args![local, transform]);
        e.call(
            DISPLAY_AXIS_ARROWS,
            &args![this, local, x87_quiet(size), id_a, id_b],
        );
    });
}

/// Rotates `vector` by the quaternion `q` the way the exe's shuffle code does it:
/// `2 * (qw * (v x q) + ((qw * qw - 0.5) * v + (v . q) * q))` per lane, the `-0.5` being the
/// vector `010c71e0` and the dot product the three-lane sum `z + (y + x)`.
pub(crate) fn rotate_vector(e: &Engine, q: Vec4, vector: Vec4) -> Vec4 {
    let bias = load_vec(e, ROTATION_BIAS);
    let dot = sum3(mul_lanes(q, vector));
    let w = [q[3]; 4];
    let turned = mul_lanes(cross_rows(vector, q), w);
    let scaled = mul_lanes(add_lanes(mul_lanes(w, w), bias), vector);
    let along = mul_lanes([dot; 4], q);
    let sum = add_lanes(turned, add_lanes(scaled, along));
    add_lanes(sum, sum)
}

/// `hkQuaternion::setMul` (`00c66320`, `this` = the result, `(left, right)`) and the identity
/// transform (`01268700`) a bone without a parent is composed with.
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const IDENTITY_TRANSFORM: u32 = 0x0126_8700;
/// The floats the bone display clamps a bone's length with: the upper limit (`01017b78`, a
/// float), the lower limit (`0101ffa0`, a double) and the length used below it (`0101e2bc`, a
/// float).
const BONE_LENGTH_LIMIT: u32 = 0x0101_7b78;
const BONE_LENGTH_FLOOR: u32 = 0x0101_ffa0;
const BONE_LENGTH_BELOW_FLOOR: u32 = 0x0101_e2bc;
/// The colours of the bone's x, y and z axis lines.
const BONE_AXIS_COLOURS: [u32; 3] = [0x7fff_0000, 0x7f00_ff00, 0x7f00_00ff];

// Translated from 00c998b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x24`): draws a posed skeleton. `count` (a signed
/// 16-bit value; nothing is drawn for zero or below) bones: `parents` holds a 16-bit parent
/// index each (`0xffff` for none), `bones` their world transforms (0x30 bytes each: translation,
/// rotation quaternion at `+0x10`, scale), `root` the transform every point is put through
/// (`00c7f7a0`). Each bone's origin is joined to its parent's by a line (`00c986c0`, `colour`
/// and the two ids); a bone without a parent is joined to the root's origin when
/// `draw_root` is set. The line's length (the distance to the parent, 1 for a root) is
/// clamped (above the limit `01017b78` it becomes the limit; below the double `0101ffa0` it
/// becomes `0101e2bc`; comparisons ordered, so NaN passes through), scaled by `scale`, and used
/// as the length of three axis lines from the bone's origin, the bone's rotation (the root's
/// rotation times the bone's, by `00c66320`) rotating them: colours `0x7fff0000`,
/// `0x7f00ff00`, `0x7f0000ff`. The exe keeps copies of the origin and of the shuffles of the
/// rotation in its frame; they hold what is recomputed here.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c998b0(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    count: u16,
    parents: u32,
    bones: u32,
    root: u32,
    id_a: u32,
    id_b: u32,
    colour: u32,
    scale: f32,
    draw_root: bool,
) {
    if (count as i16) <= 0 {
        return;
    }
    let zero: f32 = e.global(ZERO_F32);
    e.with_stack(0xe0, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let (origin_at, copy_at, end_at) = (frame + 0x20, frame + 0x40, frame + 0x50);
        let (root_origin_at, rotation_at, parent_origin_at) =
            (frame + 0x60, frame + 0x70, frame + 0x80);
        let (mut bone, mut parent_at) = (bones, parents);
        for _ in 0..count {
            e.call(TRANSFORM_POINT, &args![origin_at, root, bone]);
            let mut length = 1.0f32;
            let parent = e.mem.u16(parent_at);
            if parent == 0xffff {
                if draw_root {
                    store_vec(e, root_origin_at, [zero; 4]);
                    e.call(
                        TRANSFORM_POINT,
                        &args![root_origin_at, root, root_origin_at],
                    );
                    e.call(
                        DISPLAY_LINE,
                        &args![this, origin_at, root_origin_at, colour, id_a, id_b],
                    );
                }
            } else {
                let offset = (i32::from(parent as i16) * 3) << 4;
                let parent_bone = bones.wrapping_add(offset as u32);
                e.call(TRANSFORM_POINT, &args![parent_origin_at, root, parent_bone]);
                e.call(
                    DISPLAY_LINE,
                    &args![this, origin_at, parent_origin_at, colour, id_a, id_b],
                );
                let delta = zip_with(
                    load_vec(e, origin_at),
                    load_vec(e, parent_origin_at),
                    |a, b| a - b,
                );
                length = x87_quiet(sum3(mul_lanes(delta, delta)).sqrt());
                let limit: f32 = e.global(BONE_LENGTH_LIMIT);
                if limit < length {
                    length = x87_quiet(limit);
                }
                let floor: f64 = e.global(BONE_LENGTH_FLOOR);
                if f64::from(length) < floor {
                    length = x87_quiet(e.global::<f32>(BONE_LENGTH_BELOW_FLOOR));
                }
            }
            copy_vec(e, origin_at, copy_at);
            e.call(
                QUATERNION_SET_MUL,
                &args![rotation_at, root + 0x10, bone + 0x10],
            );
            let length = x87_quiet(length * scale);
            let rotation = load_vec(e, rotation_at);
            for (axis, axis_colour) in BONE_AXIS_COLOURS.into_iter().enumerate() {
                let mut direction = [zero; 4];
                direction[axis] = length;
                let rotated = rotate_vector(e, rotation, direction);
                if axis > 0 {
                    copy_vec(e, copy_at, origin_at);
                }
                let added = if axis == 0 {
                    load_vec(e, origin_at)
                } else {
                    load_vec(e, copy_at)
                };
                store_vec(e, end_at, add_lanes(rotated, added));
                e.call(
                    DISPLAY_LINE,
                    &args![this, origin_at, end_at, axis_colour, id_a, id_b],
                );
            }
            parent_at += 2;
            bone += 0x30;
        }
    });
}

// Translated from 00c99ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`): appends the handler to the list under the
/// lock, growing the array by one (`00c90940`, cdecl `(allocator, array, element size 4)`)
/// first when its size equals its capacity (the capacity word without its two flag bits).
pub fn fn_00c99ce0(e: &mut Engine, this: Ptr<HkDebugDisplay>, handler: u32) {
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let array = this.addr() + 8;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.get(this, HkDebugDisplay::handlerCapacity) & 0x3fff_ffff;
    if e.get(this, HkDebugDisplay::handlerCount) as u32 == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW_BY_ONE, &args![allocator, array, 4u32]);
    }
    let count = e.get(this, HkDebugDisplay::handlerCount);
    let data = e.get(this, HkDebugDisplay::handlers).addr();
    e.mem
        .set_u32(data.wrapping_add((count as u32).wrapping_mul(4)), handler);
    e.set(this, HkDebugDisplay::handlerCount, count.wrapping_add(1));
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
}

// Translated from 00c99d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x24`): draws a skeleton from its local pose.
/// It takes a block of `count * 0x30` bytes (rounded up to a multiple of 0x80) from the memory
/// router's stack (the router's first word is the stack top), composes each bone with its
/// parent's world transform (the identity transform `01268700` for a parent index of `-1`):
/// the bone's translation rotated by the parent's rotation (`2 * (...)` as in `00c99610`) plus
/// the parent's translation, the rotation product by `00c66320` and the product of the scales;
/// then draws the world pose with `00c998b0` (the arguments pass through, the world array
/// replacing `locals`) and gives the block back by writing the old top into the router.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c99d40(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
    count: i32,
    parents: u32,
    locals: u32,
    root: u32,
    id_a: u32,
    id_b: u32,
    colour: u32,
    scale: f32,
    draw_root: u32,
) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let base = e.mem.u32(router);
    let size = (count.wrapping_mul(0x30) as u32).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, base.wrapping_add(size));
    for index in 0..count.max(0) as u32 {
        let parent = i32::from(e.mem.u16(parents + 2 * index) as i16);
        let parent_at = if parent == -1 {
            IDENTITY_TRANSFORM
        } else {
            base.wrapping_add(((parent * 3) << 4) as u32)
        };
        let local = locals + 0x30 * index;
        let world = base + 0x30 * index;
        let q = load_vec(e, parent_at + 0x10);
        let v = load_vec(e, local);
        let rotated = rotate_vector(e, q, v);
        let moved = add_lanes(rotated, load_vec(e, parent_at));
        store_vec(e, world, moved);
        e.call(
            QUATERNION_SET_MUL,
            &args![world + 0x10, parent_at + 0x10, local + 0x10],
        );
        let scale_product = mul_lanes(load_vec(e, local + 0x20), load_vec(e, parent_at + 0x20));
        store_vec(e, world + 0x20, scale_product);
    }
    e.call(
        DISPLAY_BONES,
        &args![
            this,
            count as u32,
            parents,
            base,
            root,
            id_a,
            id_b,
            colour,
            x87_quiet(scale),
            draw_root
        ],
    );
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, base);
}

/// The vtable `hkDebugDisplay` sets (`010c69cc`), the base class's (`0102e388`) and the
/// critical-section initialiser the constructor calls (`00cf6da0`, `this` = the block,
/// `(spin count)`).
const DEBUG_DISPLAY_VTABLE: u32 = 0x010c_69cc;
const BASE_VTABLE: u32 = 0x0102_e388;
const INIT_CRITICAL_SECTION: u32 = 0x00cf_6da0;

// Translated from 00c99eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::hkDebugDisplay` (Xbox PDB), thiscall: sets the reference count (the 16-bit
/// word at `+6`) to 1, the vtable and an empty handler array (flag word `0x80000000`: the array
/// owns no storage), takes the 0x18-byte critical section from the router's allocator (slot
/// `4`) and initialises it with `00cf6da0` (spin count `0x3e8`); the pointer it returns
/// is stored at `+0x14` (null when the allocator gave nothing). Returns `this`.
pub fn hk_debug_display_constructor(
    e: &mut Engine,
    this: Ptr<HkDebugDisplay>,
) -> Ptr<HkDebugDisplay> {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), DEBUG_DISPLAY_VTABLE);
    e.set(this, HkDebugDisplay::handlers, Ptr::new(0));
    e.set(this, HkDebugDisplay::handlerCount, 0);
    e.set(this, HkDebugDisplay::handlerCapacity, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x18u32]).u32();
    let lock = if block != 0 {
        e.call(INIT_CRITICAL_SECTION, &args![block, 0x3e8u32]).u32()
    } else {
        0
    };
    e.set(this, HkDebugDisplay::lock, Ptr::new(lock));
    this
}

// Translated from 00c99f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplay::~hkDebugDisplay` (Xbox PDB), thiscall: sets the vtable, deletes the critical
/// section (when there is one: `DeleteCriticalSection`, then the router's allocator frees the
/// 0x18-byte block through slot `8`), empties the handler array (count `0`; when the capacity
/// word's top bit is clear its storage, `4 * (capacity & 0x3fffffff)` bytes, goes back through
/// slot `0x10`), resets the array to null with the flag word `0x80000000` and sets the base
/// class's vtable (`0102e388`).
pub fn hk_debug_display_destructor(e: &mut Engine, this: Ptr<HkDebugDisplay>) {
    let lock = e.get(this, HkDebugDisplay::lock).addr();
    e.mem.set_u32(this.addr(), DEBUG_DISPLAY_VTABLE);
    if lock != 0 {
        e.call(DELETE_CRITICAL_SECTION, &args![lock]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![lock, 0x18u32]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.get(this, HkDebugDisplay::handlerCapacity);
    e.set(this, HkDebugDisplay::handlerCount, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        let data = e.get(this, HkDebugDisplay::handlers).addr();
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.set(this, HkDebugDisplay::handlers, Ptr::new(0));
    e.set(this, HkDebugDisplay::handlerCapacity, 0x8000_0000);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00c99f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 0x18 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10), stores
/// the size as a 16-bit word at +4 and ends with a jump to the constructor `00c99eb0` (`this` in
/// `ECX`, no stack words), whose answer it returns. Ghidra made no function here; the address is stored in the data word at 011b0f04.
pub fn fn_00c99f80(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x18u32]).u32();
    e.mem.set_u16(block + 4, 0x18);
    e.call(0x00c9_9eb0, &args![block]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_9fb0, deleting_destructor_00c99fb0(Ptr, u8) -> Ptr),
        entry!(
            0x00c9_84a0,
            hk_debug_display_set_geometry_color(Ptr<HkDebugDisplay>, u32, u32, u32)
        ),
        entry!(
            0x00c9_84f0,
            hk_debug_display_update_geometry(Ptr<HkDebugDisplay>, u32, u32, u32)
        ),
        entry!(
            0x00c9_8540,
            hk_debug_display_remove_geometry(Ptr<HkDebugDisplay>, u32, u32, u32)
        ),
        entry!(
            0x00c9_8590,
            hk_debug_display_update_camera(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8600,
            hk_debug_display_update_behavior(
                Ptr<HkDebugDisplay>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x00c9_8670,
            hk_debug_display_display_point(Ptr<HkDebugDisplay>, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_86c0,
            hk_debug_display_display_line(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8720,
            hk_debug_display_display_triangle(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8780,
            fn_00c98780(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_87e0,
            fn_00c987e0(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8820,
            fn_00c98820(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8880,
            fn_00c98880(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8b60,
            fn_00c98b60(Ptr<HkDebugDisplay>, u32, f32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8be0,
            fn_00c98be0(Ptr<HkDebugDisplay>, u32, u32, f32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8c30,
            fn_00c98c30(Ptr<HkDebugDisplay>, u32, u32, f32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8c80,
            fn_00c98c80(Ptr<HkDebugDisplay>, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_8e60,
            fn_00c98e60(Ptr<HkDebugDisplay>, u32, u32, f32, u32, u32, u32)
        ),
        entry!(
            0x00c9_90b0,
            hk_debug_display_display_text(Ptr<HkDebugDisplay>, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_9100,
            hk_debug_display_display_3d_text(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_9160,
            hk_debug_display_display_geometry_with_tag(
                Ptr<HkDebugDisplay>,
                u32,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x00c9_91c0,
            hk_debug_display_display_geometry(Ptr<HkDebugDisplay>, u32, u32, u32, u32)
        ),
        entry!(0x00c9_9210, fn_00c99210(u32, u32, u32, u32)),
        entry!(0x00c9_9590, fn_00c99590(Ptr<HkDebugDisplay>, u32)),
        entry!(0x00c9_95f0, fn_00c995f0(Ptr<HkDebugDisplay>)),
        entry!(
            0x00c9_9610,
            fn_00c99610(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_96e0,
            fn_00c996e0(Ptr<HkDebugDisplay>, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_9720,
            fn_00c99720(Ptr<HkDebugDisplay>, u32, f32, u32, u32)
        ),
        entry!(
            0x00c9_9870,
            fn_00c99870(Ptr<HkDebugDisplay>, u32, f32, u32, u32)
        ),
        entry!(
            0x00c9_98b0,
            fn_00c998b0(Ptr<HkDebugDisplay>, u16, u32, u32, u32, u32, u32, u32, f32, bool)
        ),
        entry!(0x00c9_9ce0, fn_00c99ce0(Ptr<HkDebugDisplay>, u32)),
        entry!(
            0x00c9_9d40,
            fn_00c99d40(Ptr<HkDebugDisplay>, i32, u32, u32, u32, u32, u32, u32, f32, u32)
        ),
        entry!(
            0x00c9_9eb0,
            hk_debug_display_constructor(Ptr<HkDebugDisplay>) -> Ptr<HkDebugDisplay>
        ),
        entry!(
            0x00c9_9f10,
            hk_debug_display_destructor(Ptr<HkDebugDisplay>)
        ),
        entry!(0x00c9_9f80, fn_00c99f80() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00c99fb0_frees_only_when_bit_zero_is_set() {
        check(0x00c9_9fb0, 0x00c9_9f10);
    }

    use super::super::hkpsphereshape::THREE_F32;

    /// The handler method doubles: slot `s` of every handler is the function at `SLOTS + s`.
    const SLOTS: u32 = 0x4a00_0000;
    /// The allocator method doubles of `install_allocator`: slot `s` is `ALLOCATOR + s`.
    const ALLOCATOR: u32 = 0x4b00_0000;

    fn aligned(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    fn vector_at(e: &mut Engine, v: [f32; 4]) -> u32 {
        let at = aligned(e, 0x10);
        store_vec(e, at, v);
        at
    }

    fn near(got: [f32; 4], want: [f32; 3]) {
        for lane in 0..3 {
            assert!(
                (got[lane] - want[lane]).abs() < 5e-3,
                "lane {lane}: {got:?} against {want:?}"
            );
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A line the display was asked to draw: the arguments and the two vectors they point at.
    struct Line {
        words: Vec<u32>,
        from: [f32; 4],
        to: [f32; 4],
    }

    type Lines = Rc<RefCell<Vec<Line>>>;

    /// Replaces the function at `address` with a double that records its calls as lines (the
    /// first and second arguments after `this` are pointers to vectors).
    fn record_lines(e: &mut Engine, address: u32) -> Lines {
        let lines: Lines = Rc::default();
        let kept = Rc::clone(&lines);
        e.register_double(address, move |e, a| {
            kept.borrow_mut().push(Line {
                words: a.to_vec(),
                from: load_vec(e, a[1]),
                to: load_vec(e, a[2]),
            });
            Ret::default()
        });
        lines
    }

    /// A display with `count` handlers (whose methods are the `SLOTS` doubles) and the lock
    /// doubles; returns the display, the lock and the handlers.
    fn display_with(e: &mut Engine, count: u32) -> (u32, u32, Vec<u32>) {
        let vtable = e.mem.alloc(0x40);
        for slot in (0xc..=0x3c).step_by(4) {
            e.mem.set_u32(vtable + slot, SLOTS + slot);
            e.register_double(SLOTS + slot, |_, _| Ret::default());
        }
        e.register_double(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register_double(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        let handlers: Vec<u32> = (0..count)
            .map(|_| {
                let handler = e.mem.alloc(0x10);
                e.mem.set_u32(handler, vtable);
                handler
            })
            .collect();
        let array = e.mem.alloc(4 * count.max(1));
        for (index, handler) in handlers.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *handler);
        }
        let lock = e.mem.alloc(0x18);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, array);
        e.mem.set_u32(this + 0xc, count);
        e.mem.set_u32(this + 0x10, count);
        e.mem.set_u32(this + 0x14, lock);
        (this, lock, handlers)
    }

    /// Calls `address` on a display with two handlers and checks that the handlers' slot `slot`
    /// got exactly `words` after themselves, between the lock calls.
    fn check_forward(address: u32, slot: u32, words: &[u32]) {
        let mut e = engine();
        let (this, lock, handlers) = display_with(&mut e, 2);
        e.call_log = Some(vec![]);
        let mut arguments = vec![this];
        arguments.extend_from_slice(words);
        e.call(address, &arguments);
        let log = e.call_log.take().unwrap_or_default();
        let mut expected = vec![(ENTER_CRITICAL_SECTION, vec![lock])];
        for handler in &handlers {
            let mut call = vec![*handler];
            call.extend_from_slice(words);
            expected.push((SLOTS + slot, call));
        }
        expected.push((LEAVE_CRITICAL_SECTION, vec![lock]));
        assert_eq!(log[1..], expected[..]);
    }

    #[test]
    fn set_geometry_color_forwards_three_words() {
        check_forward(0x00c9_84a0, 0xc, &[1, 2, 3]);
    }

    #[test]
    fn update_geometry_forwards_three_words() {
        check_forward(0x00c9_84f0, 0x10, &[4, 5, 6]);
    }

    #[test]
    fn remove_geometry_forwards_three_words() {
        check_forward(0x00c9_8540, 0x14, &[7, 8, 9]);
    }

    #[test]
    fn update_behavior_forwards_eight_words() {
        check_forward(0x00c9_8600, 0x1c, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn display_point_forwards_four_words() {
        check_forward(0x00c9_8670, 0x20, &[1, 2, 3, 4]);
    }

    #[test]
    fn display_line_forwards_five_words() {
        check_forward(0x00c9_86c0, 0x24, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn display_triangle_forwards_six_words() {
        check_forward(0x00c9_8720, 0x28, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn display_text_forwards_four_words() {
        check_forward(0x00c9_90b0, 0x2c, &[1, 2, 3, 4]);
    }

    #[test]
    fn display_3d_text_forwards_five_words() {
        check_forward(0x00c9_9100, 0x30, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn display_geometry_with_tag_forwards_five_words() {
        check_forward(0x00c9_9160, 0x3c, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn display_geometry_forwards_four_words() {
        check_forward(0x00c9_91c0, 0x38, &[1, 2, 3, 4]);
    }

    #[test]
    fn update_camera_forwards_seven_words_and_quiets_the_floats() {
        let mut e = engine();
        let (this, lock, handlers) = display_with(&mut e, 2);
        e.call_log = Some(vec![]);
        let words = [10u32, 11, 12, 0x7f80_0001, 0x3f80_0000, 0xff80_0002, 13];
        let mut arguments = vec![this];
        arguments.extend_from_slice(&words);
        e.call(0x00c9_8590, &arguments);
        let log = e.call_log.take().unwrap_or_default();
        let quiet = [10u32, 11, 12, 0x7fc0_0001, 0x3f80_0000, 0xffc0_0002, 13];
        let mut expected = vec![(ENTER_CRITICAL_SECTION, vec![lock])];
        for handler in &handlers {
            let mut call = vec![*handler];
            call.extend_from_slice(&quiet);
            expected.push((SLOTS + 0x18, call));
        }
        expected.push((LEAVE_CRITICAL_SECTION, vec![lock]));
        assert_eq!(log[1..], expected[..]);
    }

    #[test]
    fn a_display_without_handlers_still_takes_and_releases_the_lock() {
        let mut e = engine();
        let (this, lock, _) = display_with(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c9_84a0, &args![this, 1u32, 2u32, 3u32]);
        let log = e.call_log.take().unwrap_or_default();
        assert_eq!(
            log[1..],
            [
                (ENTER_CRITICAL_SECTION, vec![lock]),
                (LEAVE_CRITICAL_SECTION, vec![lock])
            ]
        );
    }

    #[test]
    fn the_offset_line_draws_from_the_start_to_the_sum_without_the_lock() {
        let mut e = engine();
        let (this, _, handlers) = display_with(&mut e, 2);
        let lines = record_lines(&mut e, SLOTS + 0x24);
        let start = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let offset = vector_at(&mut e, [0.5, 0.25, -1.0, 8.0]);
        e.call_log = Some(vec![]);
        e.call(
            0x00c9_8820,
            &args![this, start, offset, 0xff00_ff00u32, 7u32, 9u32],
        );
        assert!(calls_to(&e, ENTER_CRITICAL_SECTION).is_empty());
        let lines = lines.borrow();
        assert_eq!(lines.len(), 2);
        for (line, handler) in lines.iter().zip(&handlers) {
            assert_eq!(line.words[0], *handler);
            assert_eq!(line.words[1], start);
            assert_eq!(line.words[3..], [0xff00_ff00, 7, 9]);
            assert_eq!(line.from, [1.0, 2.0, 3.0, 4.0]);
            assert_eq!(line.to, [1.5, 2.25, 2.0, 12.0]);
        }
    }

    /// `setTransformedPos` double: the point plus the transform's first float in lane 0.
    fn install_transform(e: &mut Engine, address: u32) {
        e.register(address, |e, a| {
            let marker = e.mem.f32(a[1]);
            let mut v = load_vec(e, a[2]);
            v[0] += marker;
            store_vec(e, a[0], v);
            Ret::default()
        });
    }

    fn transform_with_marker(e: &mut Engine, marker: f32) -> u32 {
        let at = aligned(e, 0x40);
        e.mem.set_f32(at, marker);
        at
    }

    #[test]
    fn the_transformed_line_puts_both_points_through_the_transform() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_transform(&mut e, TRANSFORM_POINT);
        let lines = record_lines(&mut e, DISPLAY_LINE);
        let transform = transform_with_marker(&mut e, 10.0);
        let from = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let to = vector_at(&mut e, [5.0, 6.0, 7.0, 8.0]);
        e.call(
            0x00c9_8780,
            &args![this, transform, from, to, 0xffu32, 3u32, 4u32],
        );
        let lines = lines.borrow();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].words[0], this);
        assert_eq!(lines[0].words[3..], [0xff, 3, 4]);
        assert_eq!(lines[0].from, [11.0, 2.0, 3.0, 4.0]);
        assert_eq!(lines[0].to, [15.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn the_qs_transformed_line_makes_the_transform_first() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        e.register(QS_TRANSFORM_FROM_TRANSFORM, |_, _| Ret::default());
        e.register(DISPLAY_TRANSFORMED_LINE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x00c9_87e0,
            &args![this, 0x111u32, 0x222u32, 0x333u32, 0x444u32, 5u32, 6u32],
        );
        let made = calls_to(&e, QS_TRANSFORM_FROM_TRANSFORM);
        assert_eq!(made.len(), 1);
        let local = made[0][0];
        assert_eq!(made[0][1], 0x111);
        assert_eq!(
            calls_to(&e, DISPLAY_TRANSFORMED_LINE),
            vec![vec![this, local, 0x222, 0x333, 0x444, 5, 6]]
        );
    }

    /// The constants of the refined root and the arrow.
    fn install_arrow_constants(e: &mut Engine) {
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(ARROW_MINIMUM_LENGTH_SQUARED, 0.0001f32);
        e.set_global(ARROW_HEAD_START, 0.85f32);
        e.set_global(ARROW_HEAD_WIDTH, 0.15f32);
        e.set_global(ARROW_HEAD_WIDTH_NEGATED, -0.15f32);
        for lane in 0..4 {
            e.set_global(ONES_VECTOR + 4 * lane, 1.0f32);
        }
    }

    /// Draws an arrow and returns its lines.
    fn arrow(start: [f32; 4], direction: [f32; 4]) -> Vec<Line> {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_arrow_constants(&mut e);
        let lines = record_lines(&mut e, DISPLAY_LINE);
        let start_at = vector_at(&mut e, start);
        let direction_at = vector_at(&mut e, direction);
        e.call(
            0x00c9_8880,
            &args![this, start_at, direction_at, 0xff12_3456u32, 8u32, 9u32],
        );
        let drawn = lines.take();
        for line in &drawn {
            assert_eq!(line.words[0], this);
            assert_eq!(line.words[3..], [0xff12_3456, 8, 9]);
        }
        drawn
    }

    #[test]
    fn the_arrow_of_an_x_direction_has_a_head_in_the_y_and_z_directions() {
        let lines = arrow([1.0, 2.0, 3.0, 0.0], [3.0, 0.0, 0.0, 0.0]);
        assert_eq!(lines.len(), 5);
        near(lines[0].from, [1.0, 2.0, 3.0]);
        near(lines[0].to, [4.0, 2.0, 3.0]);
        let heads = [
            [3.55, 2.0, 2.55],
            [3.55, 2.0, 3.45],
            [3.55, 2.45, 3.0],
            [3.55, 1.55, 3.0],
        ];
        for (line, head) in lines[1..].iter().zip(heads) {
            near(line.from, [4.0, 2.0, 3.0]);
            near(line.to, head);
        }
    }

    #[test]
    fn the_arrow_of_a_z_direction_picks_the_y_axis_for_its_helper() {
        let lines = arrow([0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 2.0, 0.0]);
        assert_eq!(lines.len(), 5);
        near(lines[0].to, [0.0, 0.0, 2.0]);
        let heads = [
            [0.0, 0.3, 1.7],
            [0.0, -0.3, 1.7],
            [-0.3, 0.0, 1.7],
            [0.3, 0.0, 1.7],
        ];
        for (line, head) in lines[1..].iter().zip(heads) {
            near(line.to, head);
        }
    }

    #[test]
    fn the_arrow_replaces_the_smallest_axis_when_the_third_is_smaller_still() {
        let lines = arrow([0.0, 0.0, 0.0, 0.0], [3.0, 2.0, 1.0, 0.0]);
        assert_eq!(lines.len(), 5);
        let heads = [
            [2.86133, 1.23301, 0.85],
            [2.23867, 2.16699, 0.85],
            [2.67481, 1.78321, 0.30917],
            [2.42519, 1.61679, 1.39083],
        ];
        for (line, head) in lines[1..].iter().zip(heads) {
            near(line.from, [3.0, 2.0, 1.0]);
            near(line.to, head);
        }
    }

    #[test]
    fn the_arrow_of_a_tiny_direction_draws_nothing_but_nan_draws() {
        assert!(arrow([0.0; 4], [0.0, 0.0, 0.001, 0.0]).is_empty());
        assert_eq!(arrow([0.0; 4], [f32::NAN, 0.0, 1.0, 0.0]).len(), 5);
    }

    #[test]
    fn the_axes_cross_runs_a_line_through_the_centre_along_each_axis() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        let lines = record_lines(&mut e, DISPLAY_LINE);
        let centre = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.call(
            0x00c9_8b60,
            &args![this, centre, 0.5f32, 0xabu32, 1u32, 2u32],
        );
        let lines = lines.borrow();
        let ends = [
            ([1.5, 2.0, 3.0, 4.0], [0.5, 2.0, 3.0, 4.0]),
            ([1.0, 2.5, 3.0, 4.0], [1.0, 1.5, 3.0, 4.0]),
            ([1.0, 2.0, 3.5, 4.0], [1.0, 2.0, 2.5, 4.0]),
        ];
        assert_eq!(lines.len(), 3);
        for (line, (plus, minus)) in lines.iter().zip(ends) {
            assert_eq!(line.from, plus);
            assert_eq!(line.to, minus);
            assert_eq!(line.words[3..], [0xab, 1, 2]);
        }
    }

    /// Doubles the axes drawing and records what it is given.
    fn record_axes(e: &mut Engine) -> Rc<RefCell<Vec<(Vec<u32>, [f32; 4])>>> {
        let seen: Rc<RefCell<Vec<(Vec<u32>, [f32; 4])>>> = Rc::default();
        let kept = Rc::clone(&seen);
        e.register_double(DISPLAY_AXES, move |e, a| {
            kept.borrow_mut().push((a.to_vec(), load_vec(e, a[1])));
            Ret::default()
        });
        seen
    }

    #[test]
    fn the_transformed_axes_draw_the_axes_through_the_transformed_point() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_transform(&mut e, TRANSFORM_POINT);
        let seen = record_axes(&mut e);
        let transform = transform_with_marker(&mut e, 10.0);
        let point = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.call(
            0x00c9_8be0,
            &args![this, transform, point, 0.25f32, 0xcdu32, 5u32, 6u32],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let words = &seen[0].0;
        assert_eq!(words[0], this);
        assert_eq!(words[2..], [0.25f32.to_bits(), 0xcd, 5, 6]);
        assert_eq!(seen[0].1, [11.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn the_row_transformed_axes_use_the_other_transform_routine() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_transform(&mut e, TRANSFORM_POINT_BY_ROWS);
        e.register(TRANSFORM_POINT, |_, _| {
            panic!("the wrong transform routine")
        });
        let seen = record_axes(&mut e);
        let transform = transform_with_marker(&mut e, 20.0);
        let point = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.call(
            0x00c9_8c30,
            &args![this, transform, point, 0.5f32, 0xefu32, 7u32, 8u32],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[2..], [0.5f32.to_bits(), 0xef, 7, 8]);
        assert_eq!(seen[0].1, [21.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn the_box_draws_its_twelve_edges() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        let lines = record_lines(&mut e, DISPLAY_LINE);
        let bounds = aligned(&mut e, 0x20);
        for (index, value) in [1.0f32, 2.0, 3.0, 9.0, 4.0, 5.0, 6.0, 9.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(bounds + 4 * index as u32, value);
        }
        e.call(0x00c9_8c80, &args![this, bounds, 0x77u32, 1u32, 2u32]);
        let lines = lines.borrow();
        let edges = [
            ([1.0, 2.0, 3.0], [1.0, 5.0, 3.0]),
            ([1.0, 2.0, 3.0], [1.0, 2.0, 6.0]),
            ([1.0, 2.0, 3.0], [4.0, 2.0, 3.0]),
            ([4.0, 5.0, 6.0], [4.0, 5.0, 3.0]),
            ([4.0, 5.0, 6.0], [1.0, 5.0, 6.0]),
            ([4.0, 5.0, 6.0], [4.0, 2.0, 6.0]),
            ([1.0, 5.0, 3.0], [4.0, 5.0, 3.0]),
            ([1.0, 5.0, 3.0], [1.0, 5.0, 6.0]),
            ([4.0, 5.0, 3.0], [4.0, 2.0, 3.0]),
            ([1.0, 5.0, 6.0], [1.0, 2.0, 6.0]),
            ([1.0, 2.0, 6.0], [4.0, 2.0, 6.0]),
            ([4.0, 2.0, 6.0], [4.0, 2.0, 3.0]),
        ];
        assert_eq!(lines.len(), 12);
        for (line, (from, to)) in lines.iter().zip(edges) {
            assert_eq!(line.from[..3], from);
            assert_eq!(line.to[..3], to);
            assert_eq!(line.from[3], 0.0);
            assert_eq!(line.words[3..], [0x77, 1, 2]);
        }
    }

    /// Draws a plane and returns its rectangle lines and the arrow it ends with.
    fn plane(normal: [f32; 3], offset: f32, size: f32) -> (Vec<Line>, Vec<Line>) {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        let lines = record_lines(&mut e, DISPLAY_LINE);
        let arrows = record_lines(&mut e, DISPLAY_ARROW);
        let plane_at = vector_at(&mut e, [normal[0], normal[1], normal[2], offset]);
        let point_at = vector_at(&mut e, [0.0; 4]);
        e.call(
            0x00c9_8e60,
            &args![this, plane_at, point_at, size, 0x55u32, 3u32, 4u32],
        );
        let rectangle = lines.take();
        let head = arrows.take();
        for line in rectangle.iter().chain(&head) {
            assert_eq!(line.words[0], this);
            assert_eq!(line.words[3..], [0x55, 3, 4]);
        }
        (rectangle, head)
    }

    fn check_rectangle(lines: &[Line], corners: [[f32; 3]; 5]) {
        assert_eq!(lines.len(), 4);
        for (index, line) in lines.iter().enumerate() {
            near(line.from, corners[index]);
            near(line.to, corners[index + 1]);
        }
    }

    #[test]
    fn a_plane_with_a_z_normal_is_a_rectangle_around_the_centre() {
        let (lines, arrows) = plane([0.0, 0.0, 1.0], 2.0, 2.0);
        check_rectangle(
            &lines,
            [
                [2.0, 2.0, -2.0],
                [2.0, -2.0, -2.0],
                [-2.0, -2.0, -2.0],
                [-2.0, 2.0, -2.0],
                [2.0, 2.0, -2.0],
            ],
        );
        assert_eq!(arrows.len(), 1);
        near(arrows[0].from, [0.0, 0.0, -2.0]);
        near(arrows[0].to, [0.0, 0.0, 2.0]);
    }

    #[test]
    fn a_plane_with_a_y_normal_uses_the_z_axis_as_its_helper() {
        let (lines, arrows) = plane([0.0, 1.0, 0.0], 1.0, 2.0);
        check_rectangle(
            &lines,
            [
                [2.0, -1.0, 2.0],
                [-2.0, -1.0, 2.0],
                [-2.0, -1.0, -2.0],
                [2.0, -1.0, -2.0],
                [2.0, -1.0, 2.0],
            ],
        );
        near(arrows[0].from, [0.0, -1.0, 0.0]);
        near(arrows[0].to, [0.0, 2.0, 0.0]);
    }

    #[test]
    fn a_plane_with_an_x_normal_uses_the_y_axis_as_its_helper() {
        let (lines, arrows) = plane([1.0, 0.0, 0.0], 1.0, 2.0);
        check_rectangle(
            &lines,
            [
                [-1.0, 2.0, 2.0],
                [-1.0, 2.0, -2.0],
                [-1.0, -2.0, -2.0],
                [-1.0, -2.0, 2.0],
                [-1.0, 2.0, 2.0],
            ],
        );
        near(arrows[0].to, [2.0, 0.0, 0.0]);
    }

    /// The constants of the lit triangle.
    fn install_lighting_constants(e: &mut Engine) {
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        for (address, value) in [
            (LIGHT_ONE[1], 2.0f32),
            (LIGHT_ONE[2], 1.0),
            (LIGHT_TWO[0], -3.0),
            (LIGHT_TWO[1], -2.0),
            (LIGHT_TWO[2], -1.0),
            (LIGHT_STRENGTH, 0.9),
            (AMBIENT_GREEN, 0.05),
            (AMBIENT_BLUE, 0.08),
            (KEY_SCALE, 0.75),
        ] {
            e.set_global(address, value);
        }
        e.set_global(BYTE_TO_UNIT, 1.0f64 / 255.0);
        for lane in 0..4 {
            e.set_global(ZERO_VECTOR + 4 * lane, 0.0f32);
        }
    }

    /// Draws a lit triangle and returns the floats the colour was made from and the triangle
    /// call's arguments.
    fn lit_triangle(argb: u32) -> (Vec<u32>, Vec<u32>, u32) {
        let mut e = engine();
        install_lighting_constants(&mut e);
        let display = e.mem.alloc(0x20);
        e.set_global(DEBUG_DISPLAY_SINGLETON, display);
        e.register(RGB_FROM_FLOATS, |_, _| Ret {
            eax: 0x1234_5678,
            ..Ret::default()
        });
        e.register(DISPLAY_TRIANGLE, |_, _| Ret::default());
        let a = vector_at(&mut e, [0.0; 4]);
        let b = vector_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let c = vector_at(&mut e, [0.0, 1.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c9_9210, &args![a, b, c, argb]);
        let colour = calls_to(&e, RGB_FROM_FLOATS).remove(0);
        let triangle = calls_to(&e, DISPLAY_TRIANGLE).remove(0);
        (colour, triangle, display)
    }

    #[test]
    fn the_lit_triangle_lights_its_colour_and_draws_it_through_the_singleton() {
        let (colour, triangle, display) = lit_triangle(0xff80_4020);
        // Two lights, the unit normal (0, 0, 1): the diffuse terms are 0.5 * 0.9 * dot + 0.5.
        let one = [0.5f64, 2.0, 1.0];
        let two = [-3.0f64, -2.0, -1.0];
        let facing = |light: [f64; 3]| {
            let length = (light[0] * light[0] + light[1] * light[1] + light[2] * light[2]).sqrt();
            0.5 * (0.9 * light[2] / length) + 0.5
        };
        let (t_one, t_two) = (facing(one), facing(two));
        let channels = [128.0 / 255.0, 64.0 / 255.0, 32.0 / 255.0];
        let ambient = [0.0, 0.05, 0.08];
        for lane in 0..3 {
            let want = ambient[lane] * channels[lane] * t_two
                + 0.75 * channels[lane] * t_one
                + 0.05 * channels[lane];
            let got = f64::from(f32::from_bits(colour[lane]));
            assert!(
                (got - want).abs() < 1e-4,
                "channel {lane}: {got} against {want}"
            );
        }
        assert_eq!(f32::from_bits(colour[3]), 1.0);
        assert_eq!(triangle[0], display);
        assert_eq!(triangle[4..], [0x1234_5678, 0, 0]);
    }

    #[test]
    fn the_lit_triangle_takes_the_alpha_from_the_top_byte() {
        let (colour, _, _) = lit_triangle(0x8000_0000);
        let alpha = f64::from(f32::from_bits(colour[3]));
        assert!((alpha - 128.0 / 255.0).abs() < 1e-6);
        // A black triangle is lit to black.
        assert_eq!(f32::from_bits(colour[0]), 0.0);
    }

    #[test]
    fn removing_a_handler_moves_the_last_one_into_its_place() {
        let mut e = engine();
        let (this, lock, handlers) = display_with(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c9_9590, &args![this, handlers[0]]);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let array = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(array), handlers[2]);
        assert_eq!(e.mem.u32(array + 4), handlers[1]);
        let log = e.call_log.take().unwrap_or_default();
        assert_eq!(log[1].0, ENTER_CRITICAL_SECTION);
        assert_eq!(log[2], (LEAVE_CRITICAL_SECTION, vec![lock]));
    }

    #[test]
    fn removing_the_last_handler_just_lowers_the_count() {
        let mut e = engine();
        let (this, _, handlers) = display_with(&mut e, 3);
        e.call(0x00c9_9590, &args![this, handlers[2]]);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let array = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(array), handlers[0]);
        assert_eq!(e.mem.u32(array + 4), handlers[1]);
        assert_eq!(e.mem.u32(array + 8), handlers[2]);
    }

    #[test]
    fn removing_an_unknown_handler_or_from_an_empty_list_changes_nothing() {
        let mut e = engine();
        let (this, _, handlers) = display_with(&mut e, 2);
        e.call(0x00c9_9590, &args![this, 0xdeadu32]);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let array = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(array), handlers[0]);
        let (empty, lock, _) = display_with(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c9_9590, &args![empty, 5u32]);
        assert_eq!(e.mem.u32(empty + 0xc), 0);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION), vec![vec![lock]]);
    }

    #[test]
    fn clearing_empties_the_list_under_the_lock() {
        let mut e = engine();
        let (this, lock, _) = display_with(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c9_95f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let log = e.call_log.take().unwrap_or_default();
        assert_eq!(
            log[1..],
            [
                (ENTER_CRITICAL_SECTION, vec![lock]),
                (LEAVE_CRITICAL_SECTION, vec![lock])
            ]
        );
    }

    /// The quaternion `0, 0, sin(45 degrees), cos(45 degrees)`: a quarter turn about z.
    const QUARTER_TURN: [f32; 4] = [
        0.0,
        0.0,
        std::f32::consts::FRAC_1_SQRT_2,
        std::f32::consts::FRAC_1_SQRT_2,
    ];

    fn install_rotation_bias(e: &mut Engine) {
        for lane in 0..4 {
            e.set_global(ROTATION_BIAS + 4 * lane, -0.5f32);
        }
    }

    /// Runs `00c99610` with a transform whose rotation is `rotation` and returns the offset
    /// line it draws.
    fn rotated_line(rotation: [f32; 4], vector: [f32; 4]) -> Line {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_rotation_bias(&mut e);
        install_transform(&mut e, TRANSFORM_POINT);
        let lines = record_lines(&mut e, DISPLAY_OFFSET_LINE);
        let transform = transform_with_marker(&mut e, 10.0);
        store_vec(&mut e, transform + 0x10, rotation);
        let point = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let vector_at_address = vector_at(&mut e, vector);
        e.call(
            0x00c9_9610,
            &args![
                this,
                transform,
                point,
                vector_at_address,
                0x99u32,
                5u32,
                6u32
            ],
        );
        let mut lines = lines.take();
        assert_eq!(lines.len(), 1);
        let line = lines.remove(0);
        assert_eq!(line.words[0], this);
        assert_eq!(line.words[3..], [0x99, 5, 6]);
        line
    }

    #[test]
    fn the_rotated_line_starts_at_the_transformed_point_and_keeps_an_unrotated_vector() {
        let line = rotated_line([0.0, 0.0, 0.0, 1.0], [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(line.from, [11.0, 2.0, 3.0, 4.0]);
        assert_eq!(line.to, [1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn the_rotated_line_turns_the_vector_by_the_quaternion() {
        let line = rotated_line(QUARTER_TURN, [1.0, 0.0, 0.0, 0.0]);
        near(line.to, [0.0, 1.0, 0.0]);
    }

    #[test]
    fn the_rotated_line_of_a_transform_makes_the_qs_transform_first() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        e.register(QS_TRANSFORM_FROM_TRANSFORM, |_, _| Ret::default());
        e.register(DISPLAY_ROTATED_LINE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x00c9_96e0,
            &args![this, 0x1u32, 0x2u32, 0x3u32, 0x4u32, 5u32, 6u32],
        );
        let made = calls_to(&e, QS_TRANSFORM_FROM_TRANSFORM);
        let local = made[0][0];
        assert_eq!(made[0][1], 1);
        assert_eq!(
            calls_to(&e, DISPLAY_ROTATED_LINE),
            vec![vec![this, local, 2, 3, 4, 5, 6]]
        );
    }

    /// Runs `00c99720` and returns the arrows it draws.
    fn axis_arrows(size: f32) -> Vec<Line> {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_transform(&mut e, TRANSFORM_POINT);
        let arrows = record_lines(&mut e, DISPLAY_ARROW);
        let transform = transform_with_marker(&mut e, 10.0);
        e.call(0x00c9_9720, &args![this, transform, size, 7u32, 8u32]);
        let drawn = arrows.take();
        for arrow in &drawn {
            assert_eq!(arrow.words[0], this);
            assert_eq!(arrow.words[4..], [7, 8]);
        }
        drawn
    }

    #[test]
    fn the_axis_arrows_run_from_the_origin_along_the_three_axes() {
        let arrows = axis_arrows(2.0);
        assert_eq!(arrows.len(), 3);
        let colours = [0xffff_0000u32, 0xff00_8000, 0xff00_00ff];
        let directions = [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]];
        for (index, arrow) in arrows.iter().enumerate() {
            assert_eq!(arrow.from, [10.0, 0.0, 0.0, 0.0]);
            assert_eq!(arrow.to[..3], directions[index]);
            assert_eq!(arrow.words[3], colours[index]);
        }
    }

    #[test]
    fn the_axis_arrows_of_a_transform_make_the_qs_transform_and_quiet_the_size() {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        e.register(QS_TRANSFORM_FROM_TRANSFORM, |_, _| Ret::default());
        e.register(DISPLAY_AXIS_ARROWS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x00c9_9870,
            &args![this, 0x42u32, 0x7f80_0001u32, 5u32, 6u32],
        );
        let made = calls_to(&e, QS_TRANSFORM_FROM_TRANSFORM);
        let local = made[0][0];
        assert_eq!(made[0][1], 0x42);
        assert_eq!(
            calls_to(&e, DISPLAY_AXIS_ARROWS),
            vec![vec![this, local, 0x7fc0_0001, 5, 6]]
        );
    }

    /// The constants and doubles the bone drawing needs: the transform double adds 100 to lane
    /// 0 (through the root's first float), `setMul` stores the identity quaternion, the
    /// length limits are 10 (upper), 0.5 (lower, a double) and 0.25 (the replacement).
    fn bone_engine() -> (Engine, u32, Lines) {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_rotation_bias(&mut e);
        install_transform(&mut e, TRANSFORM_POINT);
        e.register(QUATERNION_SET_MUL, |e, a| {
            store_vec(e, a[0], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        e.set_global(BONE_LENGTH_LIMIT, 10.0f32);
        e.set_global(BONE_LENGTH_FLOOR, 0.5f64);
        e.set_global(BONE_LENGTH_BELOW_FLOOR, 0.25f32);
        let lines = record_lines(&mut e, DISPLAY_LINE);
        (e, this, lines)
    }

    /// Draws two bones (a root at the origin and a child at `child`) and returns the lines.
    fn two_bones(child: [f32; 3], scale: f32, draw_root: bool) -> Vec<Line> {
        let (mut e, this, lines) = bone_engine();
        let root = transform_with_marker(&mut e, 100.0);
        let bones = aligned(&mut e, 0x60);
        store_vec(&mut e, bones + 0x30, [child[0], child[1], child[2], 0.0]);
        let parents = e.mem.alloc(4);
        e.mem.set_u16(parents, 0xffff);
        e.mem.set_u16(parents + 2, 0);
        e.call(
            0x00c9_98b0,
            &args![this, 2u32, parents, bones, root, 4u32, 5u32, 0x66u32, scale, draw_root],
        );
        let drawn = lines.take();
        for line in &drawn {
            assert_eq!(line.words[0], this);
            assert_eq!(line.words[4..], [4, 5]);
        }
        drawn
    }

    #[test]
    fn two_bones_draw_the_parent_lines_and_three_axes_each() {
        let lines = two_bones([3.0, 4.0, 0.0], 2.0, true);
        assert_eq!(lines.len(), 8);
        // The root bone: a line to the root's origin, then its axes of length 1 * 2.
        near(lines[0].from, [100.0, 0.0, 0.0]);
        near(lines[0].to, [100.0, 0.0, 0.0]);
        assert_eq!(lines[0].words[3], 0x66);
        let root_axes = [[102.0, 0.0, 0.0], [100.0, 2.0, 0.0], [100.0, 0.0, 2.0]];
        let colours = [0x7fff_0000u32, 0x7f00_ff00, 0x7f00_00ff];
        for axis in 0..3 {
            near(lines[1 + axis].from, [100.0, 0.0, 0.0]);
            near(lines[1 + axis].to, root_axes[axis]);
            assert_eq!(lines[1 + axis].words[3], colours[axis]);
        }
        // The child: a line to its parent, then axes of length 5 * 2.
        near(lines[4].from, [103.0, 4.0, 0.0]);
        near(lines[4].to, [100.0, 0.0, 0.0]);
        assert_eq!(lines[4].words[3], 0x66);
        let child_axes = [[113.0, 4.0, 0.0], [103.0, 14.0, 0.0], [103.0, 4.0, 10.0]];
        for axis in 0..3 {
            near(lines[5 + axis].from, [103.0, 4.0, 0.0]);
            near(lines[5 + axis].to, child_axes[axis]);
            assert_eq!(lines[5 + axis].words[3], colours[axis]);
        }
    }

    #[test]
    fn a_root_bone_is_not_joined_to_the_root_unless_asked() {
        let lines = two_bones([3.0, 4.0, 0.0], 2.0, false);
        assert_eq!(lines.len(), 7);
        near(lines[0].to, [102.0, 0.0, 0.0]);
    }

    #[test]
    fn a_long_bone_is_clamped_to_the_limit() {
        let lines = two_bones([30.0, 40.0, 0.0], 2.0, true);
        near(lines[5].to, [150.0, 40.0, 0.0]);
    }

    #[test]
    fn a_short_bone_is_given_the_replacement_length() {
        let lines = two_bones([0.1, 0.0, 0.0], 2.0, true);
        near(lines[5].to, [100.6, 0.0, 0.0]);
    }

    #[test]
    fn no_bones_draw_nothing() {
        for count in [0u32, 0x8000] {
            let (mut e, this, lines) = bone_engine();
            let root = transform_with_marker(&mut e, 100.0);
            e.call(
                0x00c9_98b0,
                &args![this, count, 0x10u32, 0x20u32, root, 4u32, 5u32, 0x66u32, 1.0f32, true],
            );
            assert!(lines.borrow().is_empty());
        }
    }

    /// A display whose array holds `count` handlers in room for `capacity`, with a router.
    fn growing_display(count: u32, capacity: u32) -> (Engine, u32, u32, u32) {
        let mut e = engine();
        let (this, lock, _) = display_with(&mut e, count);
        e.mem.set_u32(this + 0x10, capacity | 0x4000_0000);
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, this, lock, allocator)
    }

    #[test]
    fn adding_a_handler_appends_it_when_there_is_room() {
        let (mut e, this, lock, _) = growing_display(2, 4);
        e.register(ARRAY_GROW_BY_ONE, |_, _| panic!("no growth is needed"));
        e.call_log = Some(vec![]);
        e.call(0x00c9_9ce0, &args![this, 0xaaaau32]);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        let array = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(array + 8), 0xaaaa);
        let log = e.call_log.take().unwrap_or_default();
        assert_eq!(log[1], (ENTER_CRITICAL_SECTION, vec![lock]));
        assert_eq!(
            log.last().cloned(),
            Some((LEAVE_CRITICAL_SECTION, vec![lock]))
        );
    }

    #[test]
    fn adding_a_handler_grows_a_full_array_by_one_first() {
        let (mut e, this, _, allocator) = growing_display(2, 2);
        e.register_double(ARRAY_GROW_BY_ONE, |e, a| {
            // The grow routine moves the array to a bigger block and raises the capacity.
            let old = e.mem.u32(a[1]);
            let new = e.mem.alloc(32);
            for word in 0..2 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(new + 4 * word, value);
            }
            e.mem.set_u32(a[1], new);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let before = e.mem.u32(e.mem.u32(this + 8));
        e.call(0x00c9_9ce0, &args![this, 0xbbbbu32]);
        assert_eq!(
            calls_to(&e, ARRAY_GROW_BY_ONE),
            vec![vec![allocator, this + 8, 4]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 3);
        let array = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(array), before);
        assert_eq!(e.mem.u32(array + 8), 0xbbbb);
    }

    /// A router whose first word is the top of a scratch stack, plus the doubles `00c99d40` calls.
    fn pose_engine() -> (Engine, u32, u32, u32) {
        let mut e = engine();
        let (this, _, _) = display_with(&mut e, 0);
        install_rotation_bias(&mut e);
        let stack = aligned(&mut e, 0x400);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, stack);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // The rotation product double copies the bone's rotation.
        e.register(QUATERNION_SET_MUL, |e, a| {
            let q = load_vec(e, a[2]);
            store_vec(e, a[0], q);
            Ret::default()
        });
        // The identity transform.
        store_vec(&mut e, IDENTITY_TRANSFORM, [0.0; 4]);
        store_vec(&mut e, IDENTITY_TRANSFORM + 0x10, [0.0, 0.0, 0.0, 1.0]);
        store_vec(&mut e, IDENTITY_TRANSFORM + 0x20, [1.0; 4]);
        (e, this, router, stack)
    }

    /// Runs `00c99d40` over two bones (a root and a child of it) whose local poses have the
    /// identity rotation; returns the engine, the stack block, and what `00c998b0` was called
    /// with together with the router's stack word at that moment.
    fn compose(count: i32) -> (Engine, u32, Vec<u32>, u32) {
        let (mut e, this, router, stack) = pose_engine();
        let drawn: Rc<RefCell<Vec<(Vec<u32>, u32)>>> = Rc::default();
        let kept = Rc::clone(&drawn);
        e.register_double(DISPLAY_BONES, move |e, a| {
            kept.borrow_mut().push((a.to_vec(), e.mem.u32(router)));
            Ret::default()
        });
        let parents = e.mem.alloc(4);
        e.mem.set_u16(parents, 0xffff);
        e.mem.set_u16(parents + 2, 0);
        let locals = aligned(&mut e, 0x60);
        for (bone, (translation, scale)) in
            [([1.0, 2.0, 3.0, 0.0], 2.0f32), ([4.0, 5.0, 6.0, 0.0], 3.0)]
                .into_iter()
                .enumerate()
        {
            let at = locals + 0x30 * bone as u32;
            store_vec(&mut e, at, translation);
            store_vec(&mut e, at + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store_vec(&mut e, at + 0x20, [scale, scale, scale, 0.0]);
        }
        e.call(
            0x00c9_9d40,
            &args![this, count, parents, locals, 0x1234u32, 4u32, 5u32, 0x66u32, 1.5f32, 1u32],
        );
        let drawn = drawn.take();
        assert_eq!(drawn.len(), 1);
        let (words, during) = drawn.into_iter().next().unwrap_or_default();
        assert_eq!(words[0], this);
        assert_eq!(e.mem.u32(router), stack);
        (e, stack, words, during)
    }

    #[test]
    fn a_local_pose_is_composed_with_the_parents_before_it_is_drawn() {
        let (e, stack, words, during) = compose(2);
        // The block is rounded up to 0x80 bytes while the bones are drawn.
        assert_eq!(during, stack + 0x80);
        assert_eq!(words[1], 2);
        assert_eq!(words[3], stack);
        assert_eq!(words[4..], [0x1234, 4, 5, 0x66, 1.5f32.to_bits(), 1]);
        assert_eq!(load_vec(&e, stack), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, stack + 0x10), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, stack + 0x20), [2.0, 2.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, stack + 0x30), [5.0, 7.0, 9.0, 0.0]);
        assert_eq!(load_vec(&e, stack + 0x50), [6.0, 6.0, 6.0, 0.0]);
    }

    #[test]
    fn a_pose_of_no_bones_is_drawn_empty_and_keeps_the_stack() {
        let (_, stack, words, during) = compose(0);
        assert_eq!(during, stack);
        assert_eq!(words[1], 0);
    }

    /// An engine with a router whose allocator hands out blocks (slot `4`), frees them (slot
    /// `8`) and frees arrays (slot `0x10`); returns it with the router and the allocator.
    fn allocator_engine() -> (Engine, u32, u32) {
        let mut e = engine();
        let vtable = e.mem.alloc(0x20);
        for slot in [4u32, 8, 0x10] {
            e.mem.set_u32(vtable + slot, ALLOCATOR + slot);
        }
        e.register(ALLOCATOR + 4, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(ALLOCATOR + 8, |_, _| Ret::default());
        e.register(ALLOCATOR + 0x10, |_, _| Ret::default());
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, router, allocator)
    }

    #[test]
    fn the_constructor_makes_an_empty_list_and_a_lock() {
        let (mut e, _, allocator) = allocator_engine();
        e.register(INIT_CRITICAL_SECTION, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c9_9eb0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), DEBUG_DISPLAY_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(calls_to(&e, ALLOCATOR + 4), vec![vec![allocator, 0x18]]);
        let init = calls_to(&e, INIT_CRITICAL_SECTION);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1], 0x3e8);
        assert_eq!(e.mem.u32(this + 0x14), init[0][0] + 0x1000);
    }

    #[test]
    fn the_constructor_leaves_the_lock_null_when_the_allocator_gives_nothing() {
        let (mut e, _, _) = allocator_engine();
        e.register(ALLOCATOR + 4, |_, _| Ret::default());
        e.register(INIT_CRITICAL_SECTION, |_, _| {
            panic!("no block to initialise")
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5555);
        assert_eq!(e.call(0x00c9_9eb0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    /// Destroys a display with a three-handler array of the given capacity word and lock and
    /// returns the engine, the display, the array, the lock and the allocator.
    fn destroy(capacity: u32, lock: u32) -> (Engine, u32, u32, u32) {
        let (mut e, _, allocator) = allocator_engine();
        e.register(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x10, capacity);
        e.mem.set_u32(this + 0x14, lock);
        e.call_log = Some(vec![]);
        e.call(0x00c9_9f10, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        (e, data, lock, allocator)
    }

    #[test]
    fn the_destructor_frees_the_lock_and_the_array() {
        let (e, data, lock, allocator) = destroy(4, 0x7000);
        assert_eq!(calls_to(&e, DELETE_CRITICAL_SECTION), vec![vec![lock]]);
        assert_eq!(
            calls_to(&e, ALLOCATOR + 8),
            vec![vec![allocator, lock, 0x18]]
        );
        assert_eq!(
            calls_to(&e, ALLOCATOR + 0x10),
            vec![vec![allocator, data, 16]]
        );
    }

    #[test]
    fn the_destructor_keeps_storage_the_array_does_not_own_and_skips_a_null_lock() {
        let (e, _, _, _) = destroy(0x8000_0004, 0);
        assert!(calls_to(&e, DELETE_CRITICAL_SECTION).is_empty());
        assert!(calls_to(&e, ALLOCATOR + 8).is_empty());
        assert!(calls_to(&e, ALLOCATOR + 0x10).is_empty());
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size stored and its answer is returned.
    #[test]
    fn fn_00c99f80_allocates_and_hands_the_block_to_the_constructor() {
        let (mut e, sizes) = allocator_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00c9_9eb0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00c9_9f80, &args![]).u32(), 0x1234);
        assert_eq!(*sizes.borrow(), vec![0x18]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0.len(), 1);
        assert_eq!(seen[0].1, 0x18);
    }
}
