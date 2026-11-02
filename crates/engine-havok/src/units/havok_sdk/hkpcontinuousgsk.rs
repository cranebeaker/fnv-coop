//! `Havok SDK/hkpcontinuousgsk.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

/// Four SSE lanes as bits.
type Lanes = [u32; 4];

/// The vector of ones (four floats).
const VECTOR_ONES: u32 = 0x010e_c7f0;
/// The sign mask (`0x80000000` in four lanes).
const SIGN_MASK: u32 = 0x010c_44d0;
/// The double the velocity error is scaled by (`1.2f` widened).
const VELOCITY_DIVISOR: u32 = 0x0102_90f0;
/// The double a quarter.
const QUARTER: u32 = 0x0102_90b0;
/// The doubles one half and two.
const HALF: u32 = 0x0101_1588;
const TWO: u32 = 0x0101_1590;
/// `_CIsqrt` (`00ec6040`): the square root of its argument (a double).
const X87_SQRT: u32 = 0x00ec_6040;
/// The doubles the interpolation fraction is clamped against (`0.1f` and `0.9f` widened).
const LOWER_FRACTION: u32 = 0x0101_ffa0;
const UPPER_FRACTION: u32 = 0x0106_b9e8;
/// The floats the fraction is set to when it is outside (`0.1f` and `0.9f`).
const LOWER_FRACTION_FLOAT: u32 = 0x0101_e2bc;
const UPPER_FRACTION_FLOAT: u32 = 0x0101_77e8;

/// `hkSweptTransformUtil::lerp2Ha` (`00cf1710`, cdecl: the swept transform at `+0x40` of a
/// body, the start time, the time step, the output transform).
const INTERPOLATE_TRANSFORM: u32 = 0x00cf_1710;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, thiscall: the result, two transforms).
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `hkVector4::setRotatedDir` (`00c7f700`, thiscall: the result, a transform, a vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkVector4::setTransformedPos` (`00c7f640`, thiscall: the result, a transform, a vector).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `004b4ab0` (thiscall: the result, a transform, a vector); no name known.
const TRANSFORM_UNNAMED_004B4AB0: u32 = 0x004b_4ab0;
/// `hkpGsk::getClosestFeature` (`00daad40`, thiscall on the `hkpGsk`: two shapes, a
/// transform, the output direction vector whose fourth lane is the distance).
const GET_CLOSEST_FEATURE: u32 = 0x00da_ad40;
/// The memory router (`00c85750`, no arguments): the object whose first word is the cursor
/// of a stack-like allocation area.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The double added to a sum before it is inverted (`1.1920929e-07`).
const EPSILON: u32 = 0x010c_9300;

/// One lane of an SSE arithmetic instruction: a NaN in the destination operand wins
/// (quieted), then a NaN in the source operand, otherwise `op`.
fn sse(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

fn lanes_op(dest: Lanes, source: Lanes, op: fn(f32, f32) -> f32) -> Lanes {
    [0, 1, 2, 3].map(|lane| sse(dest[lane], source[lane], op))
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

/// The lane sum `x2 + (x1 + x0)` the code builds with two `ADDSS` over shuffled copies.
fn lane_sum(lanes: Lanes) -> u32 {
    let low = sse(lanes[1], lanes[0], add);
    sse(lanes[2], low, add)
}

/// `SHUFPS` of a register with itself: lane `n` of the result is lane `order[n]`.
fn shuffle(lanes: Lanes, order: [usize; 4]) -> Lanes {
    order.map(|lane| lanes[lane])
}

/// A `MOVAPS` load of four lanes.
fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    [0, 1, 2, 3].map(|lane| e.mem.u32(addr + 4 * lane))
}

/// A `MOVAPS` store of four lanes.
fn store_lanes(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, value);
    }
}

/// Copies the four words at `from` to `to`.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    let lanes = load_lanes(e, from);
    store_lanes(e, to, lanes);
}

/// The columns of three transform rows, as the unpack and move shuffles build them (the
/// fourth lanes are `row2[1]`, `row2[3]` and `row2[3]`).
fn transpose_rows(rows: [Lanes; 3]) -> [Lanes; 3] {
    let [row0, row1, row2] = rows;
    [
        [row0[0], row1[0], row2[0], row2[1]],
        [row0[1], row1[1], row2[1], row2[3]],
        [row0[2], row1[2], row2[2], row2[3]],
    ]
}

/// `(time - motion[0x4c]) * motion[0x5c]` on the x87 stack, rounded to a float: how far
/// through its step a motion record is at `time`.
fn motion_fraction(e: &Engine, motion: u32, time: f64) -> u32 {
    let start = x87_load32(e.mem.u32(motion + 0x4c));
    let scale = x87_load32(e.mem.u32(motion + 0x5c));
    x87_store32(x87_mul(x87_sub(time, start), scale))
}

/// The motion record's interpolated position: `(1 - f) * record[+0x40] + record[+0x50] * f`
/// in every lane.
fn blended_position(e: &Engine, motion: u32, fraction: u32) -> Lanes {
    let spread = [fraction; 4];
    let ones = load_lanes(e, VECTOR_ONES);
    let from = lanes_op(
        lanes_op(ones, spread, sub),
        load_lanes(e, motion + 0x40),
        mul,
    );
    let to = lanes_op(load_lanes(e, motion + 0x50), spread, mul);
    lanes_op(from, to, add)
}

/// `axis x offset` with the shuffles `0xc9` and `0xd2`.
fn cross(axis: Lanes, offset: Lanes) -> Lanes {
    let forward = lanes_op(
        shuffle(axis, [1, 2, 0, 3]),
        shuffle(offset, [2, 0, 1, 3]),
        mul,
    );
    let backward = lanes_op(
        shuffle(axis, [2, 0, 1, 3]),
        shuffle(offset, [1, 2, 0, 3]),
        mul,
    );
    lanes_op(forward, backward, sub)
}

/// `(axis x offset) . direction`, as the lane sum.
fn cross_dot(axis: Lanes, offset: Lanes, direction: Lanes) -> u32 {
    lane_sum(lanes_op(cross(axis, offset), direction, mul))
}

/// A dot product scaled by the velocity divisor on the x87 stack: divided when `divide`,
/// else multiplied.
fn scaled_by_divisor(e: &Engine, dot: u32, divide: bool) -> f64 {
    let divisor = e.mem.f64(VELOCITY_DIVISOR);
    let value = x87_load32(dot);
    if divide {
        x87_div(value, divisor)
    } else {
        x87_mul(value, divisor)
    }
}

// Translated from 00db49b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCalcPointVelocityAssumingAngularVelError`: the error of the point velocity along
/// `direction`. `axes` arrives in `ECX` (two vectors, at `+0` and `+0x10`) and `second_motion`
/// in `EDX`; `first_motion` and the two motion records hold a start position at `+0x40` with
/// its time in the fourth lane, an end position at `+0x50` with a scale in its fourth lane.
/// `time` is a float. With `d_n = point - position_n(time)` the result is
/// `-(w . direction) + (axes[0] x d_0) . direction / 1.2` (divided when positive, else
/// multiplied) minus the same term with `axes[1]` and the second record (divided when
/// negative, else multiplied); the answer is a float in `ST0`.
pub fn calc_point_velocity_assuming_angular_velocity_error(
    e: &mut Engine,
    axes: Ptr,
    second_motion: Ptr,
    first_motion: Ptr,
    point: Ptr,
    direction: Ptr,
    angular_velocity: Ptr,
    time: u32,
) -> f32 {
    let time = x87_load32(time);
    let direction = load_lanes(e, direction.addr());
    let mask = load_lanes(e, SIGN_MASK);
    let angular = load_lanes(e, angular_velocity.addr());
    let negated = [0, 1, 2, 3].map(|lane| angular[lane] ^ mask[lane]);
    let angular_term = lane_sum(lanes_op(negated, direction, mul));
    let point = load_lanes(e, point.addr());
    let axes = axes.addr();

    let fraction = motion_fraction(e, first_motion.addr(), time);
    let position = blended_position(e, first_motion.addr(), fraction);
    let offset = lanes_op(point, position, sub);
    let dot = cross_dot(load_lanes(e, axes), offset, direction);
    let value = x87_load32(dot);
    let divide = value > 0.0 || value.is_nan();
    let scaled = scaled_by_divisor(e, dot, divide);
    let partial = x87_store32(x87_add(scaled, x87_load32(angular_term)));

    let fraction = motion_fraction(e, second_motion.addr(), time);
    let position = blended_position(e, second_motion.addr(), fraction);
    let offset = lanes_op(point, position, sub);
    let dot = cross_dot(load_lanes(e, axes + 0x10), offset, direction);
    let value = x87_load32(dot);
    let divide = value < 0.0 || value.is_nan();
    let scaled = scaled_by_divisor(e, dot, divide);
    f32::from_bits(x87_store32(x87_sub(x87_load32(partial), scaled)))
}

// Translated from 00db4b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCollideInitMatrices`: fills the `0x130`-byte work area `out` for the interpolation
/// `fraction` (a float) of the time step in the collision `state` (the two bodies at `+0` and
/// `+4`, the time record at `+0x40`): the two interpolated transforms at `out` and `out+0x40`,
/// their relative transform at `out+0x80`, the plane normal at `out+0xc0` (its fourth lane the
/// negated offset at `state+0x14`) and its position at `out+0xd0`, the two transformed rows
/// at `out+0xf0` and `out+0x100`, a body vector copy at `out+0x110`, and two transformed
/// positions at `out+0x120` and `out+0xe0`.
pub fn collide_init_matrices(e: &mut Engine, state: Ptr, fraction: u32, out: Ptr) {
    let state = state.addr();
    let out = out.addr();
    let times = e.mem.u32(state + 0x40);
    let first_body = e.mem.u32(state);
    let span_value = x87_mul(x87_load32(e.mem.u32(times + 8)), x87_load32(fraction));
    let span = x87_quiet_bits(x87_store32(span_value));
    let start = x87_quiet_bits(e.mem.u32(times));
    e.call(
        INTERPOLATE_TRANSFORM,
        &args![first_body + 0x40, start, span, out],
    );
    let second_body = e.mem.u32(state + 4);
    e.call(
        INTERPOLATE_TRANSFORM,
        &args![second_body + 0x40, start, span, out + 0x40],
    );
    e.call(SET_MUL_INVERSE_MUL, &args![out + 0x80, out, out + 0x40]);
    e.call(
        SET_ROTATED_DIR,
        &args![out + 0xc0, out + 0x80, state + 0x20],
    );
    e.call(
        SET_TRANSFORMED_POS,
        &args![out + 0xd0, out + 0x80, state + 0x30],
    );
    let offset = x87_quiet_bits(e.mem.u32(state + 0x14)) ^ 0x8000_0000;
    e.mem.set_u32(out + 0xcc, offset);
    let first_body = e.mem.u32(state);
    copy_vector(e, first_body + 0x80, out + 0x110);

    // The three rows transposed into columns.
    let rows = [0, 0x10, 0x20].map(|row| load_lanes(e, out + row));
    let columns = transpose_rows(rows);
    for row in 0..2 {
        let v = load_lanes(e, state + 0x60 + 0x10 * row);
        let spread = |lane: usize| [v[lane]; 4];
        let from_y = lanes_op(spread(1), columns[1], mul);
        let from_x = lanes_op(spread(0), columns[0], mul);
        let sum = lanes_op(from_y, from_x, add);
        let from_z = lanes_op(spread(2), columns[2], mul);
        store_lanes(e, out + 0xf0 + 0x10 * row, lanes_op(sum, from_z, add));
    }
    let second_body = e.mem.u32(state + 4);
    e.call(
        SET_TRANSFORMED_POS,
        &args![out + 0x120, out + 0x80, second_body + 0x80],
    );
    e.call(
        TRANSFORM_UNNAMED_004B4AB0,
        &args![out + 0xe0, out, state + 0x50],
    );
}

// Translated from 00db4ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCollidePointWithPlaneFinal`: refines, by up to ten interpolation steps between the
/// fractions `a` and `b` (whose signed distances to the plane are `c` and `d`), the fraction
/// where `point` reaches the plane distance in `plane[0]`; the tolerance is a quarter of
/// `plane[4]`. The fraction is clamped to `0.1..0.9` on every even step. If it is below the
/// time of impact in `best[0]`, that and the point (at `best+0x10`) are replaced. Answers
/// `best[0]` as a float. The word after `d` is not used.
pub fn collide_point_with_plane_final(
    e: &mut Engine,
    state: Ptr,
    plane: Ptr,
    point: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    _unused_1: u32,
    best: Ptr,
) -> f32 {
    let plane = plane.addr();
    let best = best.addr();
    let (mut a, mut b, mut c, mut d) = (a, b, c, d);
    let mut fraction = x87_quiet_bits(a);
    let tolerance = x87_store32(x87_mul(
        x87_load32(e.mem.u32(plane + 0x10)),
        e.mem.f64(QUARTER),
    ));
    for step in 0..10u32 {
        let target = x87_load32(e.mem.u32(plane));
        let near = x87_load32(c);
        if target >= near {
            break;
        }
        let span = x87_store32(x87_sub(x87_load32(d), near));
        if -x87_load32(tolerance) < x87_load32(span) {
            break;
        }
        fraction = x87_store32(x87_div(x87_sub(target, near), x87_load32(span)));
        if step % 2 == 0 {
            let above_lower = x87_load32(fraction) > e.mem.f64(LOWER_FRACTION);
            if !above_lower {
                fraction = x87_quiet_bits(e.mem.u32(LOWER_FRACTION_FLOAT));
            }
            let below_upper = x87_load32(fraction) < e.mem.f64(UPPER_FRACTION);
            if !below_upper {
                fraction = x87_quiet_bits(e.mem.u32(UPPER_FRACTION_FLOAT));
            }
        }
        let weight = x87_load32(fraction);
        let from_start = x87_mul(x87_sub(1.0, weight), x87_load32(a));
        let from_end = x87_mul(weight, x87_load32(b));
        fraction = x87_store32(x87_add(from_start, from_end));

        let distance = e.with_stack(0x130, |e, work| {
            let work = work.addr();
            collide_init_matrices(e, state, x87_quiet_bits(fraction), Ptr::new(work));
            let position = load_lanes(e, work + 0xd0);
            let normal = load_lanes(e, work + 0xc0);
            let difference = lanes_op(load_lanes(e, point.addr()), position, sub);
            let product = lanes_op(difference, normal, mul);
            let high = sse(product[2], normal[3], add);
            let low = sse(product[1], product[0], add);
            sse(high, low, add)
        });
        let error = x87_store32(x87_sub(x87_load32(distance), x87_load32(e.mem.u32(plane))));
        if x87_load32(tolerance) > x87_load32(error).abs() {
            break;
        }
        let reached = x87_load32(e.mem.u32(plane));
        let below = reached > x87_load32(distance);
        if below {
            b = x87_quiet_bits(fraction);
            d = x87_quiet_bits(distance);
        } else {
            a = x87_quiet_bits(fraction);
            c = x87_quiet_bits(distance);
        }
    }
    let current = x87_load32(e.mem.u32(best));
    if current > x87_load32(fraction) {
        let replaced = x87_quiet_bits(fraction);
        e.mem.set_u32(best, replaced);
        copy_vector(e, point.addr(), best + 0x10);
        f32::from_bits(replaced)
    } else {
        f32::from_bits(x87_store32(current))
    }
}

// Translated from 00db4e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCollidePointsWithPlane`: sorts the points of the collision `state` (`state[2]` an
/// array of 16-byte points, `state[3]` their number) that can reach the plane `plane` (three
/// floats of limits at `+0`, `+4`, `+8`) to the front and, from the first interpolation
/// fraction `state[0x80]` on, narrows the time of impact stored with its point in `out`
/// (`out[0]`, the point at `out+0x10`): a point whose distance is at most the plane limit
/// plus the margin (`state[0x11]` times the remaining time span) is tested further; one that
/// is on the near side of the plane is handed to
/// [`collide_point_with_plane_final`] (or, in the first round, written to `out` as is), one
/// beyond it is moved to the front part of the array with its distance in the fourth lane and
/// the next fraction is the least time the point can reach the limit. Rounds repeat while
/// points were moved and the next fraction does not pass the best time. The square roots go
/// through the x87 square root (`00ec6040`).
pub fn collide_points_with_plane(e: &mut Engine, state: Ptr, plane: Ptr, out: Ptr) {
    e.with_stack(0x130, |e, work| {
        let work = work.addr();
        let state_ptr = state;
        let state = state.addr();
        let plane = plane.addr();
        let out = out.addr();
        // The slots of the frame: the best time, the current and the previous fraction, the
        // number of points moved this round.
        let mut best = 1.0f32.to_bits();
        let mut start = x87_quiet_bits(e.mem.u32(state + 0x80));
        let mut previous = start;
        let mut kept = e.mem.u32(state + 0xc);
        if kept == 0 {
            return;
        }
        loop {
            collide_init_matrices(e, state_ptr, x87_quiet_bits(start), Ptr::new(work));
            let best_value = x87_load32(best);
            let start_value = x87_load32(start);
            let span = x87_store32(x87_sub(best_value, start_value));
            let span_value = x87_load32(span);
            let mut counter = kept.wrapping_sub(1) as i32;
            kept = 0;
            let points = e.mem.u32(state + 8);
            let mut read = points;
            let mut write = points;
            let margin = x87_store32(x87_mul(x87_load32(e.mem.u32(state + 0x44)), span_value));
            let mut candidate = x87_store32(best_value);
            while counter >= 0 {
                'point: {
                    let point = load_lanes(e, read);
                    let position = load_lanes(e, work + 0xd0);
                    let normal = load_lanes(e, work + 0xc0);
                    let difference = lanes_op(point, position, sub);
                    let product = lanes_op(difference, normal, mul);
                    let high = sse(product[2], normal[3], add);
                    let low = sse(product[1], product[0], add);
                    let distance = sse(high, low, add);
                    let limit = e.mem.u32(plane);
                    let reach = x87_add(x87_load32(limit), x87_load32(margin));
                    let distance_value = x87_load32(distance);
                    if reach < distance_value {
                        break 'point;
                    }
                    let excess = x87_store32(x87_sub(distance_value, x87_load32(limit)));
                    let from_first = lanes_op(point, load_lanes(e, work + 0x110), sub);
                    let from_second = lanes_op(point, load_lanes(e, work + 0x120), sub);
                    let first = lanes_op(
                        cross(load_lanes(e, work + 0xf0), from_first),
                        load_lanes(e, work + 0xe0),
                        sub,
                    );
                    let second = cross(load_lanes(e, work + 0x100), from_second);
                    let turning = lane_sum(lanes_op(lanes_op(first, second, sub), normal, mul));
                    let rate = x87_mul(x87_load32(turning), span_value);
                    let growth = x87_store32(x87_add(x87_load32(excess), rate));
                    let allowed = x87_mul(x87_mul(span_value, e.mem.f64(HALF)), span_value);
                    let allowed =
                        x87_store32(x87_mul(allowed, x87_load32(e.mem.u32(state + 0x18))));
                    if x87_load32(allowed) < x87_load32(growth) {
                        break 'point;
                    }
                    if x87_load32(e.mem.u32(plane)) > distance_value {
                        let lower = x87_load32(start);
                        if x87_load32(e.mem.u32(state + 0x80)) == lower {
                            let first_time = x87_store32(lower);
                            e.mem.set_u32(out, first_time);
                            best = first_time;
                            copy_vector(e, read, out + 0x10);
                        } else {
                            let point_w = x87_quiet_bits(e.mem.u32(read + 0xc));
                            let answer = collide_point_with_plane_final(
                                e,
                                state_ptr,
                                Ptr::new(plane),
                                Ptr::new(read),
                                x87_quiet_bits(previous),
                                x87_store32(lower),
                                point_w,
                                x87_store32(distance_value),
                                x87_quiet_bits(turning),
                                Ptr::new(out),
                            );
                            best = answer.to_bits();
                        }
                        break 'point;
                    }
                    // The point is moved to the front part of the array.
                    let moved = load_lanes(e, read);
                    let front = load_lanes(e, write);
                    store_lanes(e, read, front);
                    store_lanes(e, write, moved);
                    e.mem.set_u32(write + 0xc, x87_store32(distance_value));
                    kept += 1;
                    write += 0x10;
                    let beyond =
                        x87_store32(x87_sub(distance_value, x87_load32(e.mem.u32(plane + 4))));
                    let two = e.mem.f64(TWO);
                    let scaled = x87_store32(x87_mul(
                        x87_mul(x87_load32(beyond), two),
                        x87_load32(e.mem.u32(state + 0x1c)),
                    ));
                    if x87_load32(scaled) > x87_mul(span_value, span_value) {
                        break 'point;
                    }
                    let gap = x87_store32(x87_sub(
                        x87_load32(e.mem.u32(plane)),
                        x87_load32(e.mem.u32(plane + 4)),
                    ));
                    let wide = x87_store32(x87_mul(
                        x87_mul(two, x87_load32(gap)),
                        x87_load32(e.mem.u32(state + 0x1c)),
                    ));
                    let root = e.call(X87_SQRT, &args![x87_load32(scaled)]).f64();
                    let mut reach_time = x87_quiet_bits(x87_store32(root));
                    let turning_value = x87_load32(turning);
                    let rate_factor = x87_load32(e.mem.u32(state + 0x1c));
                    if turning_value < 0.0 || turning_value.is_nan() {
                        let negated = x87_store32(x87_mul(-turning_value, rate_factor));
                        if x87_load32(reach_time) < x87_load32(negated) {
                            reach_time = x87_quiet_bits(negated);
                        }
                    } else {
                        let doubled = x87_mul(turning_value, rate_factor);
                        let doubled = x87_add(doubled, doubled);
                        reach_time = x87_store32(x87_add(doubled, x87_load32(reach_time)));
                    }
                    if span_value < x87_load32(reach_time) {
                        break 'point;
                    }
                    let wide_root = e.call(X87_SQRT, &args![x87_load32(wide)]).f64();
                    let wide_time = x87_store32(wide_root);
                    let total = x87_store32(x87_add(x87_load32(wide_time), x87_load32(reach_time)));
                    let third = x87_quiet_bits(e.mem.u32(plane + 8));
                    reach_time = if x87_load32(third) < x87_load32(total) {
                        total
                    } else {
                        third
                    };
                    let next = x87_store32(x87_add(x87_load32(reach_time), x87_load32(start)));
                    let later = x87_load32(next) > x87_load32(candidate);
                    if !later {
                        candidate = x87_quiet_bits(next);
                    }
                }
                read += 0x10;
                counter = counter.wrapping_sub(1);
            }
            let best_value = x87_load32(best);
            let start_value = x87_load32(start);
            if best_value < start_value {
                return;
            }
            let candidate_value = x87_load32(candidate);
            let continues =
                candidate_value != start_value || best_value > start_value || best_value.is_nan();
            if !continues {
                return;
            }
            previous = x87_store32(start_value);
            start = x87_store32(candidate_value);
            if kept == 0 {
                return;
            }
        }
    });
}

// Translated from 00db52c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCalcSeparatingNormal`: the separating normal and a point for the shape pair.
/// Register convention: `gsk` is `EDI` (the `hkpGsk`), `transform_a` is `ESI` and
/// `transform_b` is `EAX`; `shape_a`, `shape_b`, `out_normal` and `out_second` are the stack
/// words. Takes the closest feature `A` (a direction, its fourth lane a distance) between
/// the shapes in the frame of `transform_a` relative to `transform_b`, writes the direction
/// turned by the rows of `transform_a` to `out_normal` (its fourth lane: the distance minus the
/// two radii at `+0x10` of the shapes), and writes the point (from the `hkpGsk` simplex by its
/// two type words, less the distance times `A` and the relative translation) turned by the
/// transposed relative rotation to `out_second`.
pub fn calc_separating_normal(
    e: &mut Engine,
    gsk: Ptr,
    transform_a: Ptr,
    transform_b: Ptr,
    shape_a: Ptr,
    shape_b: Ptr,
    out_normal: Ptr,
    out_second: Ptr,
) {
    e.with_stack(0x50, |e, frame| {
        let relative = frame.addr();
        let closest = relative + 0x40;
        e.call(
            SET_MUL_INVERSE_MUL,
            &args![relative, transform_a, transform_b],
        );
        e.call(
            GET_CLOSEST_FEATURE,
            &args![gsk, shape_a, shape_b, relative, closest],
        );
        let a = load_lanes(e, closest);
        let spread = |lane: usize| [a[lane]; 4];
        let rows = [0, 0x10, 0x20].map(|row| load_lanes(e, transform_a.addr() + row));
        let toward_y = lanes_op(spread(1), rows[1], mul);
        let toward_x = lanes_op(spread(0), rows[0], mul);
        let partial = lanes_op(toward_y, toward_x, add);
        let toward_z = lanes_op(spread(2), rows[2], mul);
        store_lanes(e, out_normal.addr(), lanes_op(partial, toward_z, add));

        let gsk = gsk.addr();
        let along = lanes_op(spread(3), a, mul);
        let feature = if e.mem.u32(gsk) == 1 {
            load_lanes(e, gsk + 0x20)
        } else if e.mem.u32(gsk + 4) == 1 {
            lanes_op(along, load_lanes(e, gsk + 0xa0), add)
        } else {
            load_lanes(e, gsk + 0x130)
        };
        let distance = x87_load32(a[3]);
        let rotation = [0, 0x10, 0x20].map(|row| load_lanes(e, relative + row));
        let translation = load_lanes(e, relative + 0x30);
        let moved = lanes_op(lanes_op(feature, along, sub), translation, sub);
        let columns = transpose_rows(rotation);
        let spread = |lane: usize| [moved[lane]; 4];
        let toward_y = lanes_op(spread(1), columns[1], mul);
        let toward_x = lanes_op(spread(0), columns[0], mul);
        let toward_z = lanes_op(columns[2], spread(2), mul);
        let partial = lanes_op(toward_y, toward_x, add);
        store_lanes(e, out_second.addr(), lanes_op(partial, toward_z, add));
        let gap = x87_sub(distance, x87_load32(e.mem.u32(shape_a.addr() + 0x10)));
        let gap = x87_sub(gap, x87_load32(e.mem.u32(shape_b.addr() + 0x10)));
        e.mem.set_u32(out_normal.addr() + 0xc, x87_store32(gap));
    });
}

// Translated from 00db53f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the collision `state` (`ESI`) from the description `input` (`EAX`; no name known):
/// `input[0]` and `input[1]` point to two records whose first word is an object (with the
/// virtual slots `0x2c` and `0x30` for the first) and whose third word is a pointer to
/// a body-like block. Stores the two block pointers at `state[0]` and `state[1]`, the sum of
/// the two objects' floats at `+0x10` at `state+0x14`, the vector at `input+0x70` at
/// `state+0x50`, `input[3] + 0x50` at `state+0x40`, the first object's slot `0x2c` answer
/// at `state+0xc` and `state+0x10`, and a `0x80`-aligned allocation of that many 16-byte
/// elements (the allocator's cursor lives in the first word of the object `00c85750`
/// answers) at `state+8`, which the object's slot `0x30` is then given. Finally
/// `state+0x18` is `b[0xa0] * s[0x7c]^2 + a[0xa0] * s[0x6c]^2` (`a`, `b` the two block
/// pointers, `s` the state) and `state+0x1c` its reciprocal plus the epsilon double.
pub fn fn_00db53f0(e: &mut Engine, input: Ptr, state: Ptr) {
    let input = input.addr();
    let state = state.addr();
    let first = e.mem.u32(input);
    let second = e.mem.u32(input + 4);
    let object = e.mem.u32(first);
    let first_block = e.mem.u32(first + 8);
    let other = e.mem.u32(second);
    let second_block = e.mem.u32(second + 8);
    e.mem.set_u32(state, first_block);
    e.mem.set_u32(state + 4, second_block);
    let radii = x87_add(
        x87_load32(e.mem.u32(other + 0x10)),
        x87_load32(e.mem.u32(object + 0x10)),
    );
    e.mem.set_u32(state + 0x14, x87_store32(radii));
    copy_vector(e, input + 0x70, state + 0x50);
    let base = e.mem.u32(input + 0xc).wrapping_add(0x50);
    e.mem.set_u32(state + 0x40, base);
    let count = e.vcall(object, 0x2c, &args![]).u32();
    e.mem.set_u32(state + 0x10, count);
    e.mem.set_u32(state + 0xc, count);
    let allocator = e.call(MEMORY_ROUTER, &args![]).u32();
    let cursor = e.mem.u32(allocator);
    let size = (count.wrapping_mul(16).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(cursor);
    e.mem.set_u32(allocator, size);
    e.mem.set_u32(state + 8, cursor);
    e.vcall(object, 0x30, &args![cursor]);

    let second_term = x87_mul(
        x87_mul(
            x87_load32(e.mem.u32(second_block + 0xa0)),
            x87_load32(e.mem.u32(state + 0x7c)),
        ),
        x87_load32(e.mem.u32(state + 0x7c)),
    );
    let second_term = x87_load32(x87_store32(second_term));
    let first_term = x87_store32(x87_mul(
        x87_mul(
            x87_load32(e.mem.u32(first_block + 0xa0)),
            x87_load32(e.mem.u32(state + 0x6c)),
        ),
        x87_load32(e.mem.u32(state + 0x6c)),
    ));
    let total = x87_store32(x87_add(second_term, x87_load32(first_term)));
    e.mem.set_u32(state + 0x18, total);
    let divisor = x87_add(x87_load32(total), e.mem.f64(EPSILON));
    e.mem
        .set_u32(state + 0x1c, x87_store32(x87_div(1.0, divisor)));
}

/// `hkGskRecalcContactInternal` (`00da8a00`, cdecl, seven stack words): recalculates the
/// contact of an `hkpGsk` from its two point arrays, their counts, a direction and two outputs.
const RECALC_CONTACT: u32 = 0x00da_8a00;
/// `hkpGsk::exitAndExportCacheImpl` (`00da9790`, thiscall on the `hkpGsk`: the output vector).
const EXPORT_CACHE: u32 = 0x00da_9790;
/// The mask that flips the sign of the first three lanes (`0x80000000` x3, then 0).
const NEGATE_XYZ: u32 = 0x010c_4c00;
/// The float `-1.1920929e-07`.
const NEGATIVE_EPSILON: u32 = 0x010c_8440;
/// The float one half, the span when the bounds have crossed.
const HALF_FLOAT: u32 = 0x0101_6248;
/// The double zero.
const ZERO: u32 = 0x0101_2060;
/// The double the best time holds when there is none (the float maximum).
const NO_TIME: u32 = 0x0104_e138;
/// The float stored in the fifth word of the record handed to the callbacks (`320.0`).
const RECORD_FLOAT: u32 = 0x010d_5384;
/// The double `0.2f` widened: a step below it may be improved with the points of the shapes.
const STEP_BOUND: u32 = 0x0101_8a90;
/// The double one: a step that reaches it ends the search.
const TIME_END: u32 = 0x0101_2070;

/// The float at `addr` as an x87 stack value.
fn float_at(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// The bits an `FLD dword` / `FSTP dword` pair leaves for the float at `addr`.
fn quiet_word(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// Writes the feature count byte (`count_at`) and the three 16-bit feature keys (`keys_at`,
/// `+2`, `+4`) the virtual slot `0x40` of a shape reads: the keys are the words at `keys`,
/// `keys+0x10` and `keys+0x20` of the `hkpGsk`, or `0xffff` for the ones beyond `count`.
fn write_feature_keys(e: &mut Engine, count_at: u32, keys_at: u32, count: u8, keys: u32) {
    e.mem.set_u8(count_at, count);
    let first = e.mem.u16(keys);
    e.mem.set_u16(keys_at, first);
    let second = if count < 2 {
        0xffff
    } else {
        e.mem.u16(keys + 0x10)
    };
    e.mem.set_u16(keys_at + 2, second);
    let third = if count < 3 {
        0xffff
    } else {
        e.mem.u16(keys + 0x20)
    };
    e.mem.set_u16(keys_at + 4, third);
}

/// Whether the byte at `+0x3e` of the record the time record (`this[3]`) holds at `+0x60` is
/// set.
fn record_blocks(e: &Engine, this: u32) -> bool {
    let record = e.mem.u32(e.mem.u32(this + 0xc) + 0x60);
    e.mem.u8(record + 0x3e) != 0
}

/// Starts the point-collision state at `state` (the layout `collide_points_with_plane` reads):
/// the two bodies, the time start `time_start` (`T+0x50`), the sum of the radii of the shapes,
/// the own vector, the point count of the first shape, the point array reserved at the memory
/// router's cursor (`+8`; the first shape fills it) and the inertia weight total and its
/// reciprocal, from the fourth lanes of the inertia vectors the caller stored at `+0x60` and
/// `+0x70`.
fn begin_point_state(
    e: &mut Engine,
    state: u32,
    shapes: [u32; 2],
    bodies: [u32; 2],
    time_start: u32,
    own: Lanes,
) {
    e.mem.set_u32(state, bodies[0]);
    e.mem.set_u32(state + 4, bodies[1]);
    e.mem.set_u32(state + 0x40, time_start);
    let radius_sum = x87_store32(x87_add(
        float_at(e, shapes[0] + 0x10),
        float_at(e, shapes[1] + 0x10),
    ));
    e.mem.set_u32(state + 0x14, radius_sum);
    store_lanes(e, state + 0x50, own);
    let count = e.vcall(shapes[0], 0x2c, &args![]).u32();
    e.mem.set_u32(state + 0x10, count);
    e.mem.set_u32(state + 0xc, count);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let cursor = e.mem.u32(router);
    let end = (count.wrapping_mul(16).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(cursor);
    e.mem.set_u32(router, end);
    e.mem.set_u32(state + 8, cursor);
    e.vcall(shapes[0], 0x30, &args![cursor]);
    let squared_a = x87_store32(x87_mul(
        x87_mul(float_at(e, bodies[0] + 0xa0), float_at(e, state + 0x6c)),
        float_at(e, state + 0x6c),
    ));
    let squared_b = x87_store32(x87_mul(
        x87_mul(float_at(e, bodies[1] + 0xa0), float_at(e, state + 0x7c)),
        float_at(e, state + 0x7c),
    ));
    let total = x87_store32(x87_add(x87_load32(squared_a), x87_load32(squared_b)));
    e.mem.set_u32(state + 0x18, total);
    let divisor = x87_add(float_at(e, state + 0x18), e.mem.f64(EPSILON));
    e.mem
        .set_u32(state + 0x1c, x87_store32(x87_div(1.0, divisor)));
}

// Translated from 00db54b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCalcToiFinal`: narrows the time of impact of a collision pair. `this` (`ECX`)
/// holds two records (at `+0` and `+4`, each with a shape at `+0` and a body at `+8`), the
/// time record at `+0xc` (its fields at `+0x50`, `+0x54`, `+0x58`, `+0x5c` are the start,
/// the end, the step scale and a velocity scale), an object at `+0x10` whose virtual slots `0x1c` and `0x20` are called and a vector at
/// `+0x70`. `limits` holds the wanted distance (`+0`), a minimal time (`+0xc`) and the
/// tolerance (`+0x10`). `gsk` is the `hkpGsk` of the pair. `lower_time`/`upper_time` and the
/// distances `lower_distance`/`upper_distance` found there are floats; `closest_query` says
/// whether the closest feature is asked from the `hkpGsk` (else its points are moved to the
/// new relative transform and the contact is recalculated). `rates` holds two angular
/// velocities (`+0`, `+0x10`); `result` is the output record (fields `+0x3010..+0x3050`,
/// the best time at `+0x3030`).
///
/// Up to ten rounds interpolate both bodies to a fraction of the time step (bisecting, with
/// the fraction clamped to `0.1..0.9`), find the distance there and move the bounds
/// until the distance is within the tolerance of the wanted one. Then the time is checked
/// against the best time in `result`; the contact point and normal are built (and for
/// shapes of type 3 refined by their virtual slot `0x40`); the closing speed is estimated
/// (for a result code of 1, also with the points of the shape); the object's slot `0x1c`
/// receives everything and, if it answers 0, the result record is filled (slot `0x20` is first
/// called with `result[1]` and `result+0x3050` unless the best time is the float maximum).
pub fn hk4d_gsk_calc_toi_final(
    e: &mut Engine,
    this: Ptr,
    limits: Ptr,
    gsk: Ptr,
    closest_query: u32,
    direction: Ptr,
    rates: Ptr,
    lower_time: u32,
    upper_time: u32,
    lower_distance: u32,
    upper_distance: u32,
    result: Ptr,
) {
    e.with_stack(0x260, |e, frame| {
        let s = frame.addr();
        let this = this.addr();
        let limits = limits.addr();
        let gsk = gsk.addr();
        let direction = direction.addr();
        let rates = rates.addr();
        let result = result.addr();
        let (mut closest_query, mut lower_time, mut upper_time) =
            (closest_query, lower_time, upper_time);
        let (mut lower_distance, mut upper_distance) = (lower_distance, upper_distance);

        let first = e.mem.u32(this);
        let second = e.mem.u32(this + 4);
        let shape_a = e.mem.u32(first);
        let shape_b = e.mem.u32(second);
        let radii = x87_store32(x87_add(
            float_at(e, shape_a + 0x10),
            float_at(e, shape_b + 0x10),
        ));
        let slope = x87_store32(x87_div(
            x87_sub(x87_load32(upper_distance), x87_load32(lower_distance)),
            x87_sub(x87_load32(upper_time), x87_load32(lower_time)),
        ));
        e.mem.set_u32(s + 0x78, slope);

        let mut counter = 0u32;
        let mut fraction;
        let mut distance;
        loop {
            let first = e.mem.u32(this);
            let second = e.mem.u32(this + 4);
            let mut span = x87_store32(x87_sub(
                x87_load32(upper_distance),
                x87_load32(lower_distance),
            ));
            let negated_tolerance = -float_at(e, limits + 0x10);
            let span_value = x87_load32(span);
            let inside = negated_tolerance < span_value;
            if inside {
                upper_time = x87_quiet_bits(lower_time);
                span = x87_quiet_bits(e.mem.u32(HALF_FLOAT));
            }
            let target = float_at(e, limits);
            let gap = x87_store32(x87_sub(x87_load32(upper_distance), target));
            let gap = x87_store32(x87_load32(gap).abs());
            let close = float_at(e, limits + 0x10) > x87_load32(gap);
            if close {
                e.mem.set_u8(s + 0x13, 1);
                closest_query = 0;
                fraction = x87_store32(x87_load32(upper_time));
            } else {
                e.mem.set_u8(s + 0x13, 0);
                let mut weight = x87_store32(x87_div(
                    x87_sub(target, x87_load32(lower_distance)),
                    x87_load32(span),
                ));
                let above_lower = x87_load32(weight) > e.mem.f64(LOWER_FRACTION);
                if !above_lower {
                    weight = x87_quiet_bits(e.mem.u32(LOWER_FRACTION_FLOAT));
                }
                let below_upper = x87_load32(weight) < e.mem.f64(UPPER_FRACTION);
                if !below_upper {
                    weight = x87_quiet_bits(e.mem.u32(UPPER_FRACTION_FLOAT));
                }
                let weight = x87_load32(weight);
                fraction = x87_store32(x87_add(
                    x87_mul(x87_load32(lower_time), x87_sub(1.0, weight)),
                    x87_mul(weight, x87_load32(upper_time)),
                ));
            }
            let time_record = e.mem.u32(this + 0xc);
            let step = x87_store32(x87_mul(
                float_at(e, time_record + 0x58),
                x87_load32(fraction),
            ));
            let start = quiet_word(e, time_record + 0x50);
            let body_a = e.mem.u32(first + 8);
            e.call(
                INTERPOLATE_TRANSFORM,
                &args![body_a + 0x40, start, step, s + 0xe0],
            );
            let body_b = e.mem.u32(second + 8);
            e.call(
                INTERPOLATE_TRANSFORM,
                &args![body_b + 0x40, start, step, s + 0x220],
            );
            e.call(SET_MUL_INVERSE_MUL, &args![s + 0x1e0, s + 0xe0, s + 0x220]);

            if closest_query != 0 {
                e.call(
                    GET_CLOSEST_FEATURE,
                    &args![gsk, shape_a, shape_b, s + 0x1e0, s + 0x80],
                );
                if e.mem.u32(gsk) == 1 {
                    copy_vector(e, gsk + 0x20, s + 0x90);
                } else if e.mem.u32(gsk + 4) == 1 {
                    let closest = load_lanes(e, s + 0x80);
                    let along = lanes_op([closest[3]; 4], closest, mul);
                    let feature = lanes_op(along, load_lanes(e, gsk + 0xa0), add);
                    store_lanes(e, s + 0x90, feature);
                } else {
                    copy_vector(e, gsk + 0x130, s + 0x90);
                }
            } else {
                let count = e.mem.u32(gsk + 4);
                let columns = [0x1e0, 0x1f0, 0x200, 0x210].map(|at| load_lanes(e, s + at));
                let mut index = count.wrapping_sub(1) as i32;
                loop {
                    let at = (index as u32)
                        .wrapping_add(0xe)
                        .wrapping_shl(4)
                        .wrapping_add(gsk);
                    let point = load_lanes(e, at);
                    let toward_y = lanes_op([point[1]; 4], columns[1], mul);
                    let toward_x = lanes_op([point[0]; 4], columns[0], mul);
                    let partial = lanes_op(toward_y, toward_x, add);
                    let toward_z = lanes_op([point[2]; 4], columns[2], mul);
                    let toward_z = lanes_op(toward_z, columns[3], add);
                    store_lanes(e, at.wrapping_sub(0x40), lanes_op(partial, toward_z, add));
                    index = index.wrapping_sub(1);
                    if index < 0 {
                        break;
                    }
                }
                let rotation = [0xe0, 0xf0, 0x100].map(|at| load_lanes(e, s + at));
                let [row_x, row_y, row_z] = transpose_rows(rotation);
                let vector = load_lanes(e, direction);
                let toward_y = lanes_op(row_y, [vector[1]; 4], mul);
                let toward_x = lanes_op([vector[0]; 4], row_x, mul);
                let partial = lanes_op(toward_y, toward_x, add);
                let toward_z = lanes_op(row_z, [vector[2]; 4], mul);
                store_lanes(e, s + 0x60, lanes_op(partial, toward_z, add));
                let (count_a, count_b) = (e.mem.u32(gsk), e.mem.u32(gsk + 4));
                e.call(
                    RECALC_CONTACT,
                    &args![
                        gsk + 0x20,
                        gsk + 0xa0,
                        count_a,
                        count_b,
                        s + 0x60,
                        s + 0x90,
                        s + 0x80
                    ],
                );
            }

            distance = x87_store32(x87_sub(float_at(e, s + 0x8c), x87_load32(radii)));
            let closing = x87_store32(x87_sub(x87_load32(distance), target));
            let closing = x87_store32(x87_load32(closing).abs());
            let within = float_at(e, limits + 0x10) > x87_load32(closing);
            let flat = x87_load32(lower_time) == x87_load32(upper_time);
            if within || e.mem.u8(s + 0x13) != 0 || flat {
                break;
            }
            let beyond = target > x87_load32(distance);
            if beyond {
                upper_time = x87_quiet_bits(fraction);
                upper_distance = x87_store32(x87_load32(distance));
            } else {
                lower_time = x87_quiet_bits(fraction);
                lower_distance = x87_store32(x87_load32(distance));
            }
            counter += 1;
            if counter >= 10 {
                break;
            }
        }

        let time_record = e.mem.u32(this + 0xc) + 0x50;
        let minimal = quiet_word(e, limits + 0xc);
        let stepped = x87_store32(x87_mul(float_at(e, time_record + 8), x87_load32(fraction)));
        let longer = x87_load32(minimal) > x87_load32(stepped)
            || x87_load32(minimal).is_nan()
            || x87_load32(stepped).is_nan();
        let reach = if longer { minimal } else { stepped };
        let time = x87_store32(x87_add(float_at(e, time_record), x87_load32(reach)));
        e.mem.set_u32(s + 0x2c, time);
        let best = float_at(e, result + 0x3030);
        let time_value = x87_load32(time);
        let improves = best > time_value;
        if !improves {
            return;
        }
        let span_end = x87_sub(float_at(e, time_record + 4), float_at(e, limits + 0xc));
        let fits = span_end > time_value;
        if !fits {
            return;
        }

        let first = e.mem.u32(this);
        let shape_a = e.mem.u32(first);
        e.call(SET_TRANSFORMED_POS, &args![s + 0x1d0, s + 0xe0, s + 0x90]);
        e.call(SET_ROTATED_DIR, &args![s + 0x40, s + 0xe0, s + 0x80]);
        let offset = x87_store32(x87_sub(-float_at(e, shape_a + 0x10), x87_load32(distance)));
        e.mem.set_u32(s + 0x1c, 2);
        let normal = load_lanes(e, s + 0x40);
        let distance_word = x87_store32(x87_load32(distance));
        e.mem.set_u32(s + 0x4c, distance_word);
        store_lanes(
            e,
            s + 0x50,
            [normal[0], normal[1], normal[2], distance_word],
        );
        let scaled = lanes_op([offset; 4], normal, mul);
        let point = lanes_op(scaled, load_lanes(e, s + 0x1d0), add);
        store_lanes(e, s + 0x30, point);

        let second = e.mem.u32(this + 4);
        let shape_b = e.mem.u32(second);
        let speed_direction;
        if e.mem.u32(shape_b + 0xc) == 3 {
            let count = e.mem.u8(gsk + 4);
            write_feature_keys(e, s + 0x13, s + 0x14, count, gsk + 0xec);
            let code = e
                .vcall(
                    shape_b,
                    0x40,
                    &args![
                        s + 0x14,
                        s + 0x13,
                        s + 0x30,
                        s + 0x220,
                        shape_a,
                        s + 0xe0,
                        s + 0x50
                    ],
                )
                .u32();
            speed_direction = load_lanes(e, s + 0x50);
            store_lanes(e, s + 0x40, speed_direction);
            e.mem.set_u32(s + 0x1c, code);
            if code == 0 && record_blocks(e, this) {
                return;
            }
        } else if e.mem.u32(shape_a + 0xc) == 3 {
            let count = e.mem.u8(gsk);
            write_feature_keys(e, s + 0x13, s + 0x14, count, gsk + 0x2c);
            let mask = load_lanes(e, NEGATE_XYZ);
            let normal = load_lanes(e, s + 0x40);
            let along = lanes_op([normal[3]; 4], normal, mul);
            let around = lanes_op(along, load_lanes(e, s + 0x30), add);
            store_lanes(e, s + 0xd0, around);
            let flipped = [0, 1, 2, 3].map(|lane| normal[lane] ^ mask[lane]);
            store_lanes(e, s + 0x50, flipped);
            let code = e
                .vcall(
                    shape_a,
                    0x40,
                    &args![
                        s + 0x14,
                        s + 0x13,
                        s + 0xd0,
                        s + 0xe0,
                        shape_b,
                        s + 0x220,
                        s + 0x50
                    ],
                )
                .u32();
            let back = load_lanes(e, s + 0x50);
            speed_direction = [0, 1, 2, 3].map(|lane| back[lane] ^ mask[lane]);
            store_lanes(e, s + 0x50, speed_direction);
            store_lanes(e, s + 0x40, speed_direction);
            e.mem.set_u32(s + 0x1c, code);
            if code == 0 && record_blocks(e, this) {
                return;
            }
        } else {
            speed_direction = load_lanes(e, s + 0x40);
        }

        // The closing speed along the direction.
        let code = e.mem.u32(s + 0x1c);
        let time_value = float_at(e, s + 0x2c);
        let first = e.mem.u32(this);
        let second = e.mem.u32(this + 4);
        let body_a = e.mem.u32(first + 8);
        let body_b = e.mem.u32(second + 8);
        let fraction_a = motion_fraction(e, body_a, time_value);
        let position_a = blended_position(e, body_a, fraction_a);
        let fraction_b = motion_fraction(e, body_b, time_value);
        let position_b = blended_position(e, body_b, fraction_b);
        let contact = load_lanes(e, s + 0x30);
        let from_a = lanes_op(contact, position_a, sub);
        let from_b = lanes_op(contact, position_b, sub);
        let rate_a = load_lanes(e, rates);
        let rate_b = load_lanes(e, rates + 0x10);
        let own = load_lanes(e, this + 0x70);
        let sign = load_lanes(e, SIGN_MASK);
        let negated = [0, 1, 2, 3].map(|lane| own[lane] ^ sign[lane]);
        let turning_a = lanes_op(cross(rate_a, from_a), negated, add);
        let relative = lanes_op(turning_a, cross(rate_b, from_b), sub);
        let speed = lane_sum(lanes_op(relative, speed_direction, mul));
        let speed = x87_store32(x87_load32(speed));
        e.mem.set_u32(s + 0x24, speed);

        if code == 1 && x87_load32(speed) > float_at(e, NEGATIVE_EPSILON) {
            // The speed is bounded with the shape's points: the bound, the two inertia
            // vectors and the work state go to the frame at `s + 0x120`.
            e.mem.set_u32(s + 0x1a0, x87_quiet_bits(fraction));
            let time_record = e.mem.u32(this + 0xc);
            let time_scale = quiet_word(e, time_record + 0x58);
            let scale_a = x87_store32(x87_mul(float_at(e, body_a + 0x5c), x87_load32(time_scale)));
            let scale_b = x87_store32(x87_mul(x87_load32(time_scale), float_at(e, body_b + 0x5c)));
            let inertia_a = lanes_op([scale_a; 4], load_lanes(e, body_a + 0x90), mul);
            let inertia_b = lanes_op([scale_b; 4], load_lanes(e, body_b + 0x90), mul);
            store_lanes(e, s + 0x180, inertia_a);
            store_lanes(e, s + 0x190, inertia_b);
            let turn_a = cross(inertia_a, speed_direction);
            let turn_b = cross(inertia_b, speed_direction);
            let length_a = f32::from_bits(lane_sum(lanes_op(turn_a, turn_a, mul)))
                .sqrt()
                .to_bits();
            let length_b = f32::from_bits(lane_sum(lanes_op(turn_b, turn_b, mul)))
                .sqrt()
                .to_bits();
            let weighted_a = x87_store32(x87_mul(float_at(e, body_a + 0xa0), x87_load32(length_a)));
            let weighted_b = x87_store32(x87_mul(float_at(e, body_b + 0xa0), x87_load32(length_b)));
            let along_own = lane_sum(lanes_op(own, speed_direction, mul));
            let weighted = x87_store32(x87_add(x87_load32(weighted_a), x87_load32(weighted_b)));
            let bound = x87_store32(x87_add(x87_load32(weighted), x87_load32(along_own)));
            e.mem.set_u32(s + 0x164, bound);
            e.mem.set_u32(s + 0x120, body_a);
            e.mem.set_u32(s + 0x124, body_b);
            e.mem.set_u32(s + 0x160, time_record + 0x50);
            store_lanes(e, s + 0x170, own);
            let radius_sum = x87_store32(x87_add(
                float_at(e, shape_b + 0x10),
                float_at(e, shape_a + 0x10),
            ));
            e.mem.set_u32(s + 0x134, radius_sum);
            let count = e.vcall(shape_a, 0x2c, &args![]).u32();
            e.mem.set_u32(s + 0x130, count);
            e.mem.set_u32(s + 0x12c, count);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let cursor = e.mem.u32(router);
            let end =
                (count.wrapping_mul(16).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(cursor);
            e.mem.set_u32(router, end);
            e.mem.set_u32(s + 0x128, cursor);
            e.vcall(shape_a, 0x30, &args![cursor]);
            let squared_b = x87_store32(x87_mul(
                x87_mul(float_at(e, body_b + 0xa0), float_at(e, s + 0x19c)),
                float_at(e, s + 0x19c),
            ));
            let squared_a = x87_store32(x87_mul(
                x87_mul(float_at(e, body_a + 0xa0), float_at(e, s + 0x18c)),
                float_at(e, s + 0x18c),
            ));
            let total = x87_store32(x87_add(x87_load32(squared_b), x87_load32(squared_a)));
            e.mem.set_u32(s + 0x138, total);
            let divisor = x87_add(float_at(e, s + 0x138), e.mem.f64(EPSILON));
            e.mem.set_u32(s + 0x13c, x87_store32(x87_div(1.0, divisor)));
            e.call(
                TRANSFORM_UNNAMED_004B4AB0,
                &args![s + 0x140, s + 0x220, s + 0x40],
            );
            copy_vector(e, gsk + 0xe0, s + 0x150);
            if e.mem.u32(s + 0x128) != 0 {
                e.mem.set_f32(s + 0x1b0, 1.0);
                collide_points_with_plane(
                    e,
                    Ptr::new(s + 0x120),
                    Ptr::new(limits),
                    Ptr::new(s + 0x1b0),
                );
                let released = e.mem.u32(s + 0x128);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                e.mem.set_u32(router, released);
                let reached = float_at(e, s + 0x1b0) < 1.0;
                if reached {
                    e.call(SET_TRANSFORMED_POS, &args![s + 0xd0, s + 0xe0, s + 0x1c0]);
                    copy_vector(e, s + 0xd0, s + 0x30);
                }
            }
        }

        // The speed is limited by the slope of the distances.
        let slope = x87_load32(e.mem.u32(s + 0x78));
        let speed_value = x87_load32(e.mem.u32(s + 0x24));
        if speed_value > slope {
            let scaled = x87_mul(speed_value, e.mem.f64(VELOCITY_DIVISOR));
            let slower = scaled < slope;
            if slower {
                e.mem.set_u32(s + 0x24, x87_store32(slope));
            } else {
                let time_word = quiet_word(e, s + 0x2c);
                let body_a = e.mem.u32(e.mem.u32(this) + 8);
                let body_b = e.mem.u32(e.mem.u32(this + 4) + 8);
                let error = calc_point_velocity_assuming_angular_velocity_error(
                    e,
                    Ptr::new(rates),
                    Ptr::new(body_b),
                    Ptr::new(body_a),
                    Ptr::new(s + 0x30),
                    Ptr::new(s + 0x40),
                    Ptr::new(this + 0x70),
                    time_word,
                );
                e.mem.set_u32(s + 0x24, error.to_bits());
                if f64::from(error) > e.mem.f64(ZERO) {
                    e.mem.set_u32(s + 0x24, 0);
                }
            }
        }

        let time_record = e.mem.u32(this + 0xc) + 0x50;
        let velocity = x87_store32(x87_mul(
            float_at(e, time_record + 0xc),
            float_at(e, s + 0x24),
        ));
        e.mem.set_u32(s + 0x24, velocity);
        for at in [
            0xa0, 0xa4, 0xa8, 0xac, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xcc,
        ] {
            e.mem.set_u32(s + at, 0);
        }
        let record_float = quiet_word(e, RECORD_FLOAT);
        e.mem.set_u32(s + 0xb0, record_float);
        e.call(EXPORT_CACHE, &args![gsk, s + 0x60]);

        let time_word = quiet_word(e, s + 0x2c);
        let callbacks = e.mem.u32(this + 0x10);
        let time_record_base = e.mem.u32(this + 0xc);
        let first = e.mem.u32(this);
        let second = e.mem.u32(this + 4);
        let accepted = e
            .vcall(
                callbacks,
                0x1c,
                &args![
                    first,
                    second,
                    time_record_base,
                    result,
                    time_word,
                    s + 0x30,
                    s + 0x60,
                    s + 0x24,
                    s + 0xa0
                ],
            )
            .u32();
        if accepted != 0 {
            return;
        }
        let best = float_at(e, result + 0x3030);
        if best != e.mem.f64(NO_TIME) {
            let callbacks = e.mem.u32(this + 0x10);
            let record_key = e.mem.u32(result + 4);
            e.vcall(callbacks, 0x20, &args![record_key, result + 0x3050]);
        }
        let velocity = quiet_word(e, s + 0x24);
        e.mem.set_u32(result + 0x3034, velocity);
        copy_vector(e, s + 0x30, result + 0x3010);
        copy_vector(e, s + 0x40, result + 0x3020);
        let time_word = quiet_word(e, s + 0x2c);
        e.mem.set_u32(result + 0x3030, time_word);
        for word in 0..12 {
            let value = e.mem.u32(s + 0xa0 + 4 * word);
            e.mem.set_u32(result + 0x3050 + 4 * word, value);
        }
        for word in 0..4 {
            let value = e.mem.u32(s + 0x60 + 4 * word);
            e.mem.set_u32(result + 0x3040 + 4 * word, value);
        }
    });
}

// Translated from 00db60c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk4dGskCollideCalcToi`: finds the first time of impact of a collision pair along its time
/// step. The seven stack words are `this` (two records at `+0` and `+4`, each with a shape at
/// `+0` and a body at `+8`; the time record at `+0xc` whose fields `+0x50`, `+0x54`, `+0x58`,
/// `+0x5c` are the start, the end, the step scale and a velocity scale and whose `+0x60` is a
/// record of tolerances; the callback object at `+0x10`; a vector at `+0x70`), three floats
/// (`scale`, the `minimum` distance and the `target` distance), `keys` (the feature counts at
/// `+8`, `+9` and `+10` and the 16-bit feature keys after them), `closest` (a vector whose
/// fourth lane is the distance, updated in place) and `result` (the output record: the best time
/// at `+0x3030`).
///
/// While the distance is above the target plus the tolerance, steps are taken along the time
/// (the step bounded below by the minimal step and, with the points of the first shape, by what
/// `collide_points_with_plane` finds); then up to ten secant rounds interpolate both bodies to a
/// fraction of the step until the distance is within the tolerance of the target. The time found
/// is checked against the best time and the end of the span; the contact point and normal are
/// built (for shapes of type 3 refined by their virtual slot `0x40`), the closing speed is
/// estimated and the callback object's slot `0x1c` receives it all; when it answers 0 the result
/// record is filled (slot `0x20` is first called with `result[1]` and `result+0x3050` unless the
/// best time is the float maximum). The point arrays and the cache of the `hkpGsk` built on the
/// frame are released at the end.
pub fn hk4d_gsk_collide_calc_toi(
    e: &mut Engine,
    this: Ptr,
    scale: u32,
    minimum: u32,
    target: u32,
    keys: Ptr,
    closest: Ptr,
    result: Ptr,
) {
    e.with_stack(0x4b0, |e, frame| {
        let s = frame.addr();
        let this = this.addr();
        let keys = keys.addr();
        let closest = closest.addr();
        let result = result.addr();
        let gsk = s + 0x2f0;
        let state = s + 0x180;
        let plane = s + 0x8c;

        let time_record = e.mem.u32(this + 0xc);
        let limits = e.mem.u32(time_record + 0x60);
        // The plane handed to `collide_points_with_plane`: the target, the minimum, the
        // smallest step, the floor of the time and the tolerance.
        e.mem.set_u32(plane, x87_quiet_bits(target));
        e.mem.set_u32(plane + 4, x87_quiet_bits(minimum));
        let tolerance = x87_store32(x87_mul(float_at(e, limits + 0x30), x87_load32(scale)));
        e.mem.set_u32(plane + 0x10, tolerance);
        let reach = x87_load32(x87_store32(x87_mul(
            x87_load32(scale),
            float_at(e, limits + 0x20),
        )));
        let lowest = float_at(e, limits + 0x24);
        let basis = if lowest < reach { reach } else { lowest };
        let basis = x87_store32(basis);
        let minimum_step = x87_store32(x87_mul(float_at(e, time_record + 0x5c), x87_load32(basis)));
        e.mem.set_u32(plane + 8, minimum_step);
        let floor_word = quiet_word(e, limits + 0x38);
        e.mem.set_u32(plane + 0xc, floor_word);

        let first = e.mem.u32(this);
        let second = e.mem.u32(this + 4);
        let shape_a = e.mem.u32(first);
        let body_a = e.mem.u32(first + 8);
        let shape_b = e.mem.u32(second);
        let body_b = e.mem.u32(second + 8);
        let step_scale = quiet_word(e, time_record + 0x58);
        let scale_a = x87_store32(x87_mul(float_at(e, body_a + 0x5c), x87_load32(step_scale)));
        let scale_b = x87_store32(x87_mul(x87_load32(step_scale), float_at(e, body_b + 0x5c)));
        let inertia_a = lanes_op([scale_a; 4], load_lanes(e, body_a + 0x90), mul);
        let inertia_b = lanes_op([scale_b; 4], load_lanes(e, body_b + 0x90), mul);
        store_lanes(e, s + 0x1e0, inertia_a);
        store_lanes(e, s + 0x1f0, inertia_b);

        // The `hkpGsk` on the frame: the point counts and the key counts of the two shapes.
        let counts = u32::from(e.mem.u8(keys + 10));
        e.mem.set_u32(gsk, u32::from(e.mem.u8(keys + 8)));
        e.mem.set_u32(gsk + 4, u32::from(e.mem.u8(keys + 9)));
        e.mem.set_u32(gsk + 8, counts & 0xf);
        e.mem.set_u32(gsk + 0xc, counts >> 4);
        let count_a = e.mem.u32(gsk);
        e.vcall(shape_a, 0x3c, &args![keys, count_a, gsk + 0x20]);
        let count_b = e.mem.u32(gsk + 4);
        let count_a = e.mem.u32(gsk);
        e.vcall(
            shape_b,
            0x3c,
            &args![keys + 2 * count_a, count_b, gsk + 0xe0],
        );

        let tolerance_value = x87_load32(tolerance);
        let target_value = x87_load32(target);
        let own = load_lanes(e, this + 0x70);
        let sign_mask = load_lanes(e, SIGN_MASK);

        let mut lower_distance = quiet_word(e, closest + 0xc);
        let mut distance_word = lower_distance;
        store_lanes(e, s + 0xa0, [0; 4]);
        let mut time_word = 0u32;
        let mut previous_time = 0u32;
        let limit_word = x87_store32(x87_add(tolerance_value, target_value));
        let limit = x87_load32(limit_word);
        let mut query = 1u32;
        let mut touched = false;
        let mut settled = false;

        'done: {
            let skip_first_loop = x87_load32(distance_word) <= limit;
            if !skip_first_loop {
                loop {
                    let remaining = x87_store32(x87_sub(1.0, x87_load32(time_word)));
                    if x87_load32(remaining) <= 0.0 {
                        break 'done;
                    }
                    // The speed the distance can fall at, from the rotation of the bodies.
                    let vector = load_lanes(e, closest);
                    let turn_a = cross(load_lanes(e, s + 0x1e0), vector);
                    let length_a = f32::from_bits(lane_sum(lanes_op(turn_a, turn_a, mul)))
                        .sqrt()
                        .to_bits();
                    let weighted_a =
                        x87_store32(x87_mul(float_at(e, body_a + 0xa0), x87_load32(length_a)));
                    let turn_b = cross(load_lanes(e, s + 0x1f0), vector);
                    let length_b = f32::from_bits(lane_sum(lanes_op(turn_b, turn_b, mul)))
                        .sqrt()
                        .to_bits();
                    let weighted_b =
                        x87_store32(x87_mul(float_at(e, body_b + 0xa0), x87_load32(length_b)));
                    let along_own = lane_sum(lanes_op(own, vector, mul));
                    let weighted =
                        x87_store32(x87_add(x87_load32(weighted_a), x87_load32(weighted_b)));
                    let bound = x87_store32(x87_add(x87_load32(along_own), x87_load32(weighted)));
                    if x87_load32(bound) <= 0.0 {
                        break 'done;
                    }
                    let lead = x87_store32(x87_sub(
                        float_at(e, closest + 0xc),
                        x87_mul(x87_load32(remaining), x87_load32(bound)),
                    ));
                    if x87_load32(minimum) < x87_load32(lead) {
                        break 'done;
                    }
                    let drop =
                        x87_store32(x87_sub(float_at(e, closest + 0xc), x87_load32(minimum)));
                    let mut ratio = x87_store32(x87_div(x87_load32(drop), x87_load32(bound)));

                    let ratio_value = x87_load32(ratio);
                    let small = ratio_value < e.mem.f64(STEP_BOUND);
                    let own_below = x87_load32(along_own) < x87_load32(weighted);
                    if small && !settled && touched && own_below {
                        if e.mem.u32(state + 8) == 0 {
                            begin_point_state(
                                e,
                                state,
                                [shape_a, shape_b],
                                [body_a, body_b],
                                time_record + 0x50,
                                own,
                            );
                            if e.mem.u32(state + 8) == 0 {
                                settled = true;
                            }
                        }
                        if e.mem.u32(state + 8) != 0 {
                            e.mem.set_u32(state + 0x80, x87_quiet_bits(time_word));
                            e.mem.set_u32(state + 0x44, x87_quiet_bits(bound));
                            e.mem.set_f32(s + 0x160, 1.0);
                            collide_points_with_plane(
                                e,
                                Ptr::new(state),
                                Ptr::new(plane),
                                Ptr::new(s + 0x160),
                            );
                            let reached = float_at(e, s + 0x160);
                            let gain = x87_store32(x87_sub(reached, x87_load32(time_word)));
                            let doubled = x87_add(ratio_value, ratio_value);
                            if doubled > x87_load32(gain) {
                                settled = true;
                            }
                            let better = ratio_value > x87_load32(gain);
                            if !better {
                                ratio = gain;
                            }
                        }
                    }

                    let ahead = x87_add(x87_load32(ratio), x87_load32(time_word));
                    let before_end = ahead < e.mem.f64(TIME_END) || ahead.is_nan();
                    if !before_end {
                        break 'done;
                    }
                    previous_time = x87_quiet_bits(time_word);
                    let sum = if x87_load32(minimum_step) > x87_load32(ratio) {
                        x87_add(x87_load32(minimum_step), x87_load32(time_word))
                    } else {
                        x87_add(x87_load32(ratio), x87_load32(time_word))
                    };
                    let sum = x87_store32(sum);
                    time_word = if 1.0 > x87_load32(sum) {
                        sum
                    } else {
                        1.0f32.to_bits()
                    };

                    // Both bodies move to the new time and the closest feature is found there.
                    lower_distance = quiet_word(e, closest + 0xc);
                    let step = x87_store32(x87_mul(
                        float_at(e, time_record + 0x58),
                        x87_load32(time_word),
                    ));
                    copy_vector(e, closest, s + 0xa0);
                    let start = quiet_word(e, time_record + 0x50);
                    e.call(
                        INTERPOLATE_TRANSFORM,
                        &args![body_a + 0x40, start, step, s + 0x210],
                    );
                    e.call(
                        INTERPOLATE_TRANSFORM,
                        &args![body_b + 0x40, start, step, s + 0xd0],
                    );
                    e.call(SET_MUL_INVERSE_MUL, &args![s + 0x120, s + 0x210, s + 0xd0]);
                    e.call(
                        GET_CLOSEST_FEATURE,
                        &args![gsk, shape_a, shape_b, s + 0x120, s + 0xb0],
                    );

                    let feature = load_lanes(e, s + 0xb0);
                    let columns = [0x210, 0x220, 0x230].map(|at| load_lanes(e, s + at));
                    let toward_y = lanes_op([feature[1]; 4], columns[1], mul);
                    let toward_x = lanes_op([feature[0]; 4], columns[0], mul);
                    let partial = lanes_op(toward_y, toward_x, add);
                    let toward_z = lanes_op([feature[2]; 4], columns[2], mul);
                    store_lanes(e, closest, lanes_op(partial, toward_z, add));

                    let along = lanes_op([feature[3]; 4], feature, mul);
                    let point = if e.mem.u32(gsk) == 1 {
                        load_lanes(e, gsk + 0x20)
                    } else if e.mem.u32(gsk + 4) == 1 {
                        lanes_op(along, load_lanes(e, gsk + 0xa0), add)
                    } else {
                        load_lanes(e, gsk + 0x130)
                    };
                    let offset =
                        lanes_op(lanes_op(point, along, sub), load_lanes(e, s + 0x150), sub);
                    let rotation = [0x120, 0x130, 0x140].map(|at| load_lanes(e, s + at));
                    let [row_x, row_y, row_z] = transpose_rows(rotation);
                    let toward_y = lanes_op(row_y, [offset[1]; 4], mul);
                    let toward_x = lanes_op([offset[0]; 4], row_x, mul);
                    let partial = lanes_op(toward_y, toward_x, add);
                    let toward_z = lanes_op(row_z, [offset[2]; 4], mul);
                    store_lanes(e, s + 0x1b0, lanes_op(partial, toward_z, add));
                    query = e.mem.u32(gsk + 0x14);

                    let new_distance = x87_store32(x87_sub(
                        x87_sub(float_at(e, s + 0xbc), float_at(e, shape_a + 0x10)),
                        float_at(e, shape_b + 0x10),
                    ));
                    e.mem.set_u32(closest + 0xc, new_distance);
                    distance_word = quiet_word(e, closest + 0xc);
                    let vector = load_lanes(e, closest);
                    let rotation = [0xd0, 0xe0, 0xf0].map(|at| load_lanes(e, s + at));
                    let [row_x, row_y, row_z] = transpose_rows(rotation);
                    let toward_y = lanes_op(row_y, [vector[1]; 4], mul);
                    let toward_x = lanes_op([vector[0]; 4], row_x, mul);
                    let partial = lanes_op(toward_y, toward_x, add);
                    let toward_z = lanes_op(row_z, [vector[2]; 4], mul);
                    store_lanes(e, s + 0x1a0, lanes_op(partial, toward_z, add));
                    touched = true;

                    let finished = x87_load32(distance_word) <= limit;
                    if finished {
                        break;
                    }
                }
            }

            // The secant rounds between the last two times.
            let mut lower_time = x87_quiet_bits(previous_time);
            let mut upper_time = x87_quiet_bits(time_word);
            let mut lower_value = x87_quiet_bits(lower_distance);
            let mut upper_value = x87_quiet_bits(distance_word);
            let radii = x87_store32(x87_add(
                float_at(e, shape_a + 0x10),
                float_at(e, shape_b + 0x10),
            ));
            let slope = x87_store32(x87_div(
                x87_sub(x87_load32(upper_value), x87_load32(lower_value)),
                x87_sub(x87_load32(upper_time), x87_load32(lower_time)),
            ));
            let negated_tolerance = -tolerance_value;
            let mut counter = 0u32;
            let mut fraction;
            let mut distance;
            loop {
                let mut span =
                    x87_store32(x87_sub(x87_load32(upper_value), x87_load32(lower_value)));
                let inside = negated_tolerance < x87_load32(span);
                if inside {
                    upper_time = x87_quiet_bits(lower_time);
                    span = x87_quiet_bits(e.mem.u32(HALF_FLOAT));
                }
                let gap = x87_store32(x87_sub(x87_load32(upper_value), target_value));
                let gap = x87_store32(x87_load32(gap).abs());
                let close = tolerance_value > x87_load32(gap);
                if close {
                    touched = true;
                    query = 0;
                    fraction = x87_quiet_bits(upper_time);
                } else {
                    touched = false;
                    let mut weight = x87_store32(x87_div(
                        x87_sub(target_value, x87_load32(lower_value)),
                        x87_load32(span),
                    ));
                    let above_lower = x87_load32(weight) > e.mem.f64(LOWER_FRACTION);
                    if !above_lower {
                        weight = x87_quiet_bits(e.mem.u32(LOWER_FRACTION_FLOAT));
                    }
                    let below_upper = x87_load32(weight) < e.mem.f64(UPPER_FRACTION);
                    if !below_upper {
                        weight = x87_quiet_bits(e.mem.u32(UPPER_FRACTION_FLOAT));
                    }
                    let weight = x87_load32(weight);
                    fraction = x87_store32(x87_add(
                        x87_mul(x87_load32(lower_time), x87_sub(1.0, weight)),
                        x87_mul(weight, x87_load32(upper_time)),
                    ));
                }
                let step = x87_store32(x87_mul(
                    float_at(e, time_record + 0x58),
                    x87_load32(fraction),
                ));
                let start = quiet_word(e, time_record + 0x50);
                e.call(
                    INTERPOLATE_TRANSFORM,
                    &args![body_a + 0x40, start, step, s + 0x120],
                );
                e.call(
                    INTERPOLATE_TRANSFORM,
                    &args![body_b + 0x40, start, step, s + 0x470],
                );
                e.call(SET_MUL_INVERSE_MUL, &args![s + 0x210, s + 0x120, s + 0x470]);

                if query != 0 {
                    e.call(
                        GET_CLOSEST_FEATURE,
                        &args![gsk, shape_a, shape_b, s + 0x210, s + 0xb0],
                    );
                    if e.mem.u32(gsk) == 1 {
                        copy_vector(e, gsk + 0x20, s + 0xc0);
                    } else if e.mem.u32(gsk + 4) == 1 {
                        let found = load_lanes(e, s + 0xb0);
                        let along = lanes_op([found[3]; 4], found, mul);
                        let feature = lanes_op(along, load_lanes(e, gsk + 0xa0), add);
                        store_lanes(e, s + 0xc0, feature);
                    } else {
                        copy_vector(e, gsk + 0x130, s + 0xc0);
                    }
                } else {
                    let count = e.mem.u32(gsk + 4);
                    let columns = [0x210, 0x220, 0x230, 0x240].map(|at| load_lanes(e, s + at));
                    let mut index = count.wrapping_sub(1) as i32;
                    loop {
                        let at = (index as u32)
                            .wrapping_add(0xe)
                            .wrapping_shl(4)
                            .wrapping_add(gsk);
                        let point = load_lanes(e, at);
                        let toward_y = lanes_op([point[1]; 4], columns[1], mul);
                        let toward_x = lanes_op([point[0]; 4], columns[0], mul);
                        let partial = lanes_op(toward_y, toward_x, add);
                        let toward_z = lanes_op([point[2]; 4], columns[2], mul);
                        let toward_z = lanes_op(toward_z, columns[3], add);
                        store_lanes(e, at.wrapping_sub(0x40), lanes_op(partial, toward_z, add));
                        index = index.wrapping_sub(1);
                        if index < 0 {
                            break;
                        }
                    }
                    let rotation = [0x120, 0x130, 0x140].map(|at| load_lanes(e, s + at));
                    let [row_x, row_y, row_z] = transpose_rows(rotation);
                    let vector = load_lanes(e, s + 0xa0);
                    let toward_y = lanes_op(row_y, [vector[1]; 4], mul);
                    let toward_x = lanes_op([vector[0]; 4], row_x, mul);
                    let partial = lanes_op(toward_y, toward_x, add);
                    let toward_z = lanes_op(row_z, [vector[2]; 4], mul);
                    store_lanes(e, s + 0x50, lanes_op(partial, toward_z, add));
                    let (count_a, count_b) = (e.mem.u32(gsk), e.mem.u32(gsk + 4));
                    e.call(
                        RECALC_CONTACT,
                        &args![
                            gsk + 0x20,
                            gsk + 0xa0,
                            count_a,
                            count_b,
                            s + 0x50,
                            s + 0xc0,
                            s + 0xb0
                        ],
                    );
                }

                distance = x87_store32(x87_sub(float_at(e, s + 0xbc), x87_load32(radii)));
                let closing = x87_store32(x87_sub(x87_load32(distance), target_value));
                let closing = x87_store32(x87_load32(closing).abs());
                let within = tolerance_value > x87_load32(closing);
                let flat = x87_load32(lower_time) == x87_load32(upper_time);
                if within || touched || flat {
                    break;
                }
                let beyond = target_value > x87_load32(distance);
                if beyond {
                    upper_time = x87_quiet_bits(fraction);
                    upper_value = x87_store32(x87_load32(distance));
                } else {
                    lower_time = x87_quiet_bits(fraction);
                    lower_value = x87_store32(x87_load32(distance));
                }
                counter += 1;
                if counter >= 10 {
                    break;
                }
            }

            // The time found against the best time and the end of the span.
            let start_at = time_record + 0x50;
            let stepped = x87_store32(x87_mul(float_at(e, start_at + 8), x87_load32(fraction)));
            let floor_value = x87_load32(floor_word);
            let take_stepped = floor_value < x87_load32(stepped);
            let reach = if take_stepped { stepped } else { floor_word };
            let time = x87_store32(x87_add(float_at(e, start_at), x87_load32(reach)));
            let best = float_at(e, result + 0x3030);
            let time_value = x87_load32(time);
            let improves = best > time_value;
            if !improves {
                break 'done;
            }
            let span_end = x87_sub(float_at(e, start_at + 4), floor_value);
            let fits = span_end > time_value;
            if !fits {
                break 'done;
            }

            // The contact point and normal at the relative transform of the last round.
            e.call(SET_TRANSFORMED_POS, &args![s + 0x2e0, s + 0x120, s + 0xc0]);
            e.call(SET_ROTATED_DIR, &args![s + 0x70, s + 0x120, s + 0xb0]);
            let offset = x87_store32(x87_sub(-float_at(e, shape_a + 0x10), x87_load32(distance)));
            let normal = load_lanes(e, s + 0x70);
            let distance_stored = x87_store32(x87_load32(distance));
            store_lanes(
                e,
                s + 0x50,
                [normal[0], normal[1], normal[2], distance_stored],
            );
            e.mem.set_u32(s + 0x7c, distance_stored);
            let scaled = lanes_op([offset; 4], normal, mul);
            let point = lanes_op(scaled, load_lanes(e, s + 0x2e0), add);
            store_lanes(e, s + 0x60, point);

            let mut code = 2u32;
            let speed_direction;
            if e.mem.u32(shape_b + 0xc) == 3 {
                let count = e.mem.u8(gsk + 4);
                write_feature_keys(e, s + 0x17, s + 0x1c, count, gsk + 0xec);
                code = e
                    .vcall(
                        shape_b,
                        0x40,
                        &args![
                            s + 0x1c,
                            s + 0x17,
                            s + 0x60,
                            s + 0x470,
                            shape_a,
                            s + 0x120,
                            s + 0x50
                        ],
                    )
                    .u32();
                speed_direction = load_lanes(e, s + 0x50);
                store_lanes(e, s + 0x70, speed_direction);
                if code == 0 && record_blocks(e, this) {
                    break 'done;
                }
            } else if e.mem.u32(shape_a + 0xc) == 3 {
                let count = e.mem.u8(gsk);
                write_feature_keys(e, s + 0x17, s + 0x1c, count, gsk + 0x2c);
                let mask = load_lanes(e, NEGATE_XYZ);
                let normal = load_lanes(e, s + 0x70);
                let along = lanes_op([normal[3]; 4], normal, mul);
                let around = lanes_op(along, load_lanes(e, s + 0x60), add);
                store_lanes(e, s + 0x110, around);
                let flipped = [0, 1, 2, 3].map(|lane| normal[lane] ^ mask[lane]);
                store_lanes(e, s + 0x50, flipped);
                code = e
                    .vcall(
                        shape_a,
                        0x40,
                        &args![
                            s + 0x1c,
                            s + 0x17,
                            s + 0x110,
                            s + 0x120,
                            shape_b,
                            s + 0x470,
                            s + 0x50
                        ],
                    )
                    .u32();
                let back = load_lanes(e, s + 0x50);
                speed_direction = [0, 1, 2, 3].map(|lane| back[lane] ^ mask[lane]);
                store_lanes(e, s + 0x50, speed_direction);
                store_lanes(e, s + 0x70, speed_direction);
                if code == 0 && record_blocks(e, this) {
                    break 'done;
                }
            } else {
                speed_direction = load_lanes(e, s + 0x70);
            }

            // The closing speed along the normal.
            let time_float = x87_load32(time);
            let fraction_a = motion_fraction(e, body_a, time_float);
            let position_a = blended_position(e, body_a, fraction_a);
            let fraction_b = motion_fraction(e, body_b, time_float);
            let position_b = blended_position(e, body_b, fraction_b);
            let contact = load_lanes(e, s + 0x60);
            let from_a = lanes_op(contact, position_a, sub);
            let from_b = lanes_op(contact, position_b, sub);
            let negated = [0, 1, 2, 3].map(|lane| own[lane] ^ sign_mask[lane]);
            let turning_a = lanes_op(cross(load_lanes(e, s + 0x1e0), from_a), negated, add);
            let relative = lanes_op(turning_a, cross(load_lanes(e, s + 0x1f0), from_b), sub);
            let speed = lane_sum(lanes_op(relative, speed_direction, mul));
            let mut speed_word = x87_store32(x87_load32(speed));

            if code == 1 && x87_load32(speed_word) > float_at(e, NEGATIVE_EPSILON) {
                // The speed is bounded with the points of the first shape: the state at
                // `s + 0x250` is built like the one of the steps.
                let refined = s + 0x250;
                e.mem.set_u32(refined + 0x80, x87_quiet_bits(fraction));
                store_lanes(e, refined + 0x60, inertia_a);
                store_lanes(e, refined + 0x70, inertia_b);
                let turn_a = cross(inertia_a, speed_direction);
                let turn_b = cross(inertia_b, speed_direction);
                let length_a = f32::from_bits(lane_sum(lanes_op(turn_a, turn_a, mul)))
                    .sqrt()
                    .to_bits();
                let length_b = f32::from_bits(lane_sum(lanes_op(turn_b, turn_b, mul)))
                    .sqrt()
                    .to_bits();
                let weighted_a =
                    x87_store32(x87_mul(float_at(e, body_a + 0xa0), x87_load32(length_a)));
                let weighted_b =
                    x87_store32(x87_mul(float_at(e, body_b + 0xa0), x87_load32(length_b)));
                let along_own = lane_sum(lanes_op(own, speed_direction, mul));
                let weighted = x87_store32(x87_add(x87_load32(weighted_a), x87_load32(weighted_b)));
                let bound = x87_store32(x87_add(x87_load32(weighted), x87_load32(along_own)));
                e.mem.set_u32(refined + 0x44, bound);
                begin_point_state(
                    e,
                    refined,
                    [shape_a, shape_b],
                    [body_a, body_b],
                    time_record + 0x50,
                    own,
                );
                e.call(
                    TRANSFORM_UNNAMED_004B4AB0,
                    &args![refined + 0x20, s + 0x470, s + 0x70],
                );
                copy_vector(e, gsk + 0xe0, refined + 0x30);
                let cursor = e.mem.u32(refined + 8);
                if cursor != 0 {
                    e.mem.set_f32(s + 0x160, 1.0);
                    collide_points_with_plane(
                        e,
                        Ptr::new(refined),
                        Ptr::new(plane),
                        Ptr::new(s + 0x160),
                    );
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    e.mem.set_u32(router, cursor);
                    let reached = float_at(e, s + 0x160) < 1.0;
                    if reached {
                        e.call(SET_TRANSFORMED_POS, &args![s + 0x110, s + 0x120, s + 0x170]);
                        copy_vector(e, s + 0x110, s + 0x60);
                    }
                }
            }

            // The speed is limited by the slope of the distances.
            let slope_value = x87_load32(slope);
            let speed_value = x87_load32(speed_word);
            if speed_value > slope_value {
                let scaled = x87_mul(speed_value, e.mem.f64(VELOCITY_DIVISOR));
                let slower = scaled < slope_value;
                if slower {
                    speed_word = x87_store32(slope_value);
                } else {
                    let error = calc_point_velocity_assuming_angular_velocity_error(
                        e,
                        Ptr::new(s + 0x1e0),
                        Ptr::new(body_b),
                        Ptr::new(body_a),
                        Ptr::new(s + 0x60),
                        Ptr::new(s + 0x70),
                        Ptr::new(this + 0x70),
                        x87_quiet_bits(time),
                    );
                    speed_word = error.to_bits();
                    if f64::from(error) > e.mem.f64(ZERO) {
                        speed_word = 0;
                    }
                }
            }

            let velocity =
                x87_store32(x87_mul(float_at(e, start_at + 0xc), x87_load32(speed_word)));
            for at in [
                0xd0, 0xd4, 0xd8, 0xdc, 0xe4, 0xe8, 0xec, 0xf0, 0xf4, 0xf8, 0xfc,
            ] {
                e.mem.set_u32(s + at, 0);
            }
            let record_float = quiet_word(e, RECORD_FLOAT);
            e.mem.set_u32(s + 0xe0, record_float);
            e.call(EXPORT_CACHE, &args![gsk, s + 0xa0]);

            e.mem.set_u32(s + 0x28, velocity);
            let callbacks = e.mem.u32(this + 0x10);
            let accepted = e
                .vcall(
                    callbacks,
                    0x1c,
                    &args![
                        first,
                        second,
                        time_record,
                        result,
                        x87_quiet_bits(time),
                        s + 0x60,
                        s + 0xa0,
                        s + 0x28,
                        s + 0xd0
                    ],
                )
                .u32();
            if accepted != 0 {
                break 'done;
            }
            let best = float_at(e, result + 0x3030);
            if best != e.mem.f64(NO_TIME) {
                let callbacks = e.mem.u32(this + 0x10);
                let record_key = e.mem.u32(result + 4);
                e.vcall(callbacks, 0x20, &args![record_key, result + 0x3050]);
            }
            let velocity_word = quiet_word(e, s + 0x28);
            e.mem.set_u32(result + 0x3034, velocity_word);
            copy_vector(e, s + 0x60, result + 0x3010);
            copy_vector(e, s + 0x70, result + 0x3020);
            e.mem.set_u32(result + 0x3030, x87_quiet_bits(time));
            for word in 0..12 {
                let value = e.mem.u32(s + 0xd0 + 4 * word);
                e.mem.set_u32(result + 0x3050 + 4 * word, value);
            }
            for word in 0..4 {
                let value = e.mem.u32(s + 0xa0 + 4 * word);
                e.mem.set_u32(result + 0x3040 + 4 * word, value);
            }
        }

        // Whatever the point arrays took from the memory router is given back, and the cache
        // of the `hkpGsk` is exported if the closest feature was asked for.
        let reserved = e.mem.u32(state + 8);
        if reserved != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            e.mem.set_u32(router, reserved);
        }
        if e.mem.u32(gsk + 0x14) != 0 {
            e.call(EXPORT_CACHE, &args![gsk, keys]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00db_49b0,
            calc_point_velocity_assuming_angular_velocity_error(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32) -> f32
        ),
        entry!(0x00db_4b50, collide_init_matrices(Ptr, u32, Ptr)),
        entry!(0x00db_4e40, collide_points_with_plane(Ptr, Ptr, Ptr)),
        entry!(
            0x00db_52c0,
            calc_separating_normal(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00db_53f0, fn_00db53f0(Ptr, Ptr)),
        entry!(
            0x00db_4ca0,
            collide_point_with_plane_final(Ptr, Ptr, Ptr, u32, u32, u32, u32, u32, Ptr) -> f32
        ),
        entry!(
            0x00db_54b0,
            hk4d_gsk_calc_toi_final(Ptr, Ptr, Ptr, u32, Ptr, Ptr, u32, u32, u32, u32, Ptr)
        ),
        entry!(
            0x00db_60c0,
            hk4d_gsk_collide_calc_toi(Ptr, u32, u32, u32, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;

    /// An engine with the exe's data range mapped and the constants this unit reads.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_f32(VECTOR_ONES + 4 * lane, 1.0);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.mem.set_f64(VELOCITY_DIVISOR, f64::from(1.2f32));
        e.mem.set_f64(QUARTER, 0.25);
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f64(LOWER_FRACTION, f64::from(0.1f32));
        e.mem.set_f64(UPPER_FRACTION, f64::from(0.9f32));
        e.mem.set_f32(LOWER_FRACTION_FLOAT, 0.1);
        e.mem.set_f32(UPPER_FRACTION_FLOAT, 0.9);
        e
    }

    /// Writes four floats at `at`.
    fn put(e: &mut Engine, at: u32, values: [f32; 4]) {
        for (lane, value) in values.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
    }

    /// A vector of four floats in new memory.
    fn vector(e: &mut Engine, values: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        put(e, at, values);
        at
    }

    /// A motion record: a start position and time, an end position and scale.
    fn motion(e: &mut Engine, start: [f32; 4], end: [f32; 4]) -> u32 {
        let record = e.mem.alloc(0x60);
        put(e, record + 0x40, start);
        put(e, record + 0x50, end);
        record
    }

    /// The point velocity error for the fixed scene: the motions, the axes and the point.
    fn velocity_error(direction: [f32; 4], time: u32) -> f32 {
        let mut e = rig();
        let first = motion(&mut e, [0.0, 0.0, 0.0, 1.0], [4.0, 0.0, 0.0, 0.5]);
        let second = motion(&mut e, [0.0, 0.0, 0.0, 0.0], [0.0, 4.0, 0.0, 0.25]);
        let axes = e.mem.alloc(0x20);
        put(&mut e, axes, [0.0, 1.0, 0.0, 0.0]);
        put(&mut e, axes + 0x10, [1.0, 0.0, 0.0, 0.0]);
        let point = vector(&mut e, [1.0, 1.0, 1.0, 0.0]);
        let direction = vector(&mut e, direction);
        let angular = vector(&mut e, [1.0, 1.0, 1.0, 0.0]);
        e.call(
            0x00db_49b0,
            &args![axes, second, first, point, direction, angular, time],
        )
        .f32()
    }

    #[test]
    fn point_velocity_error_divides_a_positive_term_and_multiplies_a_negative_one() {
        // d_0 = (-1, 1, 1): axes[0] x d_0 = (1, 0, 1); d_1 = (1, -1, 1): axes[1] x d_1 =
        // (0, -1, -1). Direction (1, 2, 3): angular term -6, first dot 4 (divided by 1.2),
        // second dot -5 (divided by 1.2): -6 + 4 / 1.2 + 5 / 1.2 = 1.5.
        let result = velocity_error([1.0, 2.0, 3.0, 0.0], 2.0f32.to_bits());
        assert!((result - 1.5).abs() < 1e-4, "{result}");
        // Direction (-1, -2, -3): angular term 6, first dot -4 (multiplied: -4.8), second
        // dot 5 (multiplied: 6): 6 - 4.8 - 6 = -4.8.
        let result = velocity_error([-1.0, -2.0, -3.0, 0.0], 2.0f32.to_bits());
        assert!((result + 4.8).abs() < 1e-4, "{result}");
    }

    #[test]
    fn point_velocity_error_of_a_nan_time_is_nan() {
        let result = velocity_error([1.0, 2.0, 3.0, 0.0], 0x7fc0_0000);
        assert!(result.is_nan());
    }

    /// A collision state: two bodies (vector at `+0x80`), the times (`+0` start, `+8` span),
    /// the plane data, and two rows at `+0x60`.
    struct State {
        state: u32,
        first_body: u32,
        second_body: u32,
        times: u32,
    }

    fn collision_state(e: &mut Engine) -> State {
        let state = e.mem.alloc(0x80);
        let first_body = e.mem.alloc(0x90);
        let second_body = e.mem.alloc(0x90);
        let times = e.mem.alloc(0x10);
        e.mem.set_u32(state, first_body);
        e.mem.set_u32(state + 4, second_body);
        e.mem.set_u32(state + 0x40, times);
        put(e, times, [0.25, 0.0, 4.0, 0.0]);
        put(e, state + 0x14, [3.5, 0.0, 0.0, 0.0]);
        put(e, state + 0x60, [1.0, 2.0, 3.0, 0.0]);
        put(e, state + 0x70, [0.0, 1.0, 0.0, 0.0]);
        put(e, first_body + 0x80, [9.0, 8.0, 7.0, 6.0]);
        State {
            state,
            first_body,
            second_body,
            times,
        }
    }

    /// Doubles for the five callees outside the unit (they write nothing).
    fn plain_callees(e: &mut Engine) {
        e.call_log = Some(Vec::new());
        for address in [
            INTERPOLATE_TRANSFORM,
            SET_MUL_INVERSE_MUL,
            SET_ROTATED_DIR,
            SET_TRANSFORMED_POS,
            TRANSFORM_UNNAMED_004B4AB0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// The argument lists of the logged calls to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn init_matrices_interpolates_both_bodies_and_transposes_the_rows() {
        let mut e = rig();
        plain_callees(&mut e);
        let s = collision_state(&mut e);
        let out = e.mem.alloc(0x130);
        put(&mut e, out, [1.0, 2.0, 3.0, 4.0]);
        put(&mut e, out + 0x10, [5.0, 6.0, 7.0, 8.0]);
        put(&mut e, out + 0x20, [9.0, 10.0, 11.0, 12.0]);
        e.call(0x00db_4b50, &args![s.state, 0.5f32.to_bits(), out]);

        let span = 2.0f32.to_bits();
        let start = 0.25f32.to_bits();
        assert_eq!(
            calls_to(&e, INTERPOLATE_TRANSFORM),
            vec![
                vec![s.first_body + 0x40, start, span, out],
                vec![s.second_body + 0x40, start, span, out + 0x40],
            ]
        );
        assert_eq!(
            calls_to(&e, SET_MUL_INVERSE_MUL),
            vec![vec![out + 0x80, out, out + 0x40]]
        );
        assert_eq!(
            calls_to(&e, SET_ROTATED_DIR),
            vec![vec![out + 0xc0, out + 0x80, s.state + 0x20]]
        );
        assert_eq!(
            calls_to(&e, SET_TRANSFORMED_POS),
            vec![
                vec![out + 0xd0, out + 0x80, s.state + 0x30],
                vec![out + 0x120, out + 0x80, s.second_body + 0x80],
            ]
        );
        assert_eq!(
            calls_to(&e, TRANSFORM_UNNAMED_004B4AB0),
            vec![vec![out + 0xe0, out, s.state + 0x50]]
        );
        assert_eq!(e.mem.f32(out + 0xcc), -3.5);
        let copied = load_lanes(&e, out + 0x110);
        assert_eq!(copied, [9.0f32, 8.0, 7.0, 6.0].map(f32::to_bits));
        // Columns (1, 5, 9, 10), (2, 6, 10, 12), (3, 7, 11, 12): the first row (1, 2, 3) gives
        // 2 * c1 + 1 * c0 + 3 * c2, the second (0, 1, 0) the column c1.
        let first_row = [14.0f32, 38.0, 62.0, 70.0].map(f32::to_bits);
        assert_eq!(load_lanes(&e, out + 0xf0), first_row);
        let second_row = [2.0f32, 6.0, 10.0, 12.0].map(f32::to_bits);
        assert_eq!(load_lanes(&e, out + 0x100), second_row);
    }

    #[test]
    fn init_matrices_passes_the_time_and_step_quieted() {
        let mut e = rig();
        plain_callees(&mut e);
        let s = collision_state(&mut e);
        let out = e.mem.alloc(0x130);
        e.mem.set_u32(s.times, 0x7f80_0001);
        e.call(0x00db_4b50, &args![s.state, 0x3f80_0000u32, out]);
        let calls = calls_to(&e, INTERPOLATE_TRANSFORM);
        assert_eq!(calls[0][1], 0x7fc0_0001);
        assert_eq!(calls[1][1], 0x7fc0_0001);
        assert_eq!(calls[0][2], 4.0f32.to_bits());
    }

    /// The doubles that make the plane distance of the point at the origin `-g(f)`, with
    /// `g(f) = f` (`squared` false) or `f * f`: the interpolation writes its span (`4 * f`) over 4
    /// into the first word of the work area, the position double stores `g(f)` and the normal is `(1, 0, 0)`.
    fn distance_scene(squared: bool) -> (Engine, State) {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.register(INTERPOLATE_TRANSFORM, |e, a| {
            e.mem.set_f32(a[3], f32::from_bits(a[2]) / 4.0);
            Ret::default()
        });
        e.register(SET_MUL_INVERSE_MUL, |_, _| Ret::default());
        e.register(SET_ROTATED_DIR, |e, a| {
            put(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        if squared {
            e.register(SET_TRANSFORMED_POS, |e, a| {
                let f = e.mem.f32(a[1] - 0x80);
                put(e, a[0], [f * f, 0.0, 0.0, 0.0]);
                Ret::default()
            });
        } else {
            e.register(SET_TRANSFORMED_POS, |e, a| {
                let f = e.mem.f32(a[1] - 0x80);
                put(e, a[0], [f, 0.0, 0.0, 0.0]);
                Ret::default()
            });
        }
        e.register(TRANSFORM_UNNAMED_004B4AB0, |_, _| Ret::default());
        let s = collision_state(&mut e);
        put(&mut e, s.state + 0x14, [0.0, 0.0, 0.0, 0.0]);
        (e, s)
    }

    /// Calls the refinement from fraction 0 (distance 0) to 1 (distance -1) for a plane at
    /// `target`, whose tolerance is a quarter of `size`, with the best time `best` and a point
    /// vector there that is 7 in every lane. Answers the result and the best-time record.
    fn refine(e: &mut Engine, s: &State, size: f32, target: f32, best: f32) -> (f32, u32) {
        let plane = e.mem.alloc(0x20);
        put(e, plane, [target, 0.0, 0.0, 0.0]);
        e.mem.set_f32(plane + 0x10, size);
        let point = vector(e, [0.0, 0.0, 0.0, 1.0]);
        let result = e.mem.alloc(0x20);
        e.mem.set_f32(result, best);
        put(e, result + 0x10, [7.0; 4]);
        let answer = e
            .call(
                0x00db_4ca0,
                &args![
                    s.state,
                    plane,
                    point,
                    0.0f32.to_bits(),
                    1.0f32.to_bits(),
                    0.0f32.to_bits(),
                    (-1.0f32).to_bits(),
                    0xdead_beefu32,
                    result
                ],
            )
            .f32();
        (answer, result)
    }

    #[test]
    fn final_refinement_stops_when_the_fraction_is_inside_the_tolerance() {
        let (mut e, s) = distance_scene(false);
        let (answer, result) = refine(&mut e, &s, 0.04, -0.3, 10.0);
        // Distance -f: the first secant step lands on f = 0.3 exactly.
        assert!((answer - 0.3).abs() < 1e-6, "{answer}");
        assert_eq!(e.mem.f32(result), answer);
        assert_eq!(
            load_lanes(&e, result + 0x10),
            [0.0f32, 0.0, 0.0, 1.0].map(f32::to_bits)
        );
        assert_eq!(calls_to(&e, INTERPOLATE_TRANSFORM).len(), 2);
    }

    #[test]
    fn final_refinement_keeps_a_smaller_time_of_impact() {
        let (mut e, s) = distance_scene(false);
        let (answer, result) = refine(&mut e, &s, 0.04, -0.3, 0.1);
        assert_eq!(answer, 0.1);
        assert_eq!(e.mem.f32(result), 0.1);
        assert_eq!(load_lanes(&e, result + 0x10), [7.0f32; 4].map(f32::to_bits));
    }

    #[test]
    fn final_refinement_without_a_tolerance_takes_ten_steps() {
        let (mut e, s) = distance_scene(true);
        let (answer, result) = refine(&mut e, &s, 0.0, -0.5, 10.0);
        assert_eq!(calls_to(&e, INTERPOLATE_TRANSFORM).len(), 20);
        // The distance is -f * f, so the plane at -0.5 is reached at f = 0.7071.
        assert!(
            (answer - std::f32::consts::FRAC_1_SQRT_2).abs() < 0.05,
            "{answer}"
        );
        assert_eq!(e.mem.f32(result), answer);
    }

    #[test]
    fn final_refinement_returns_at_once_when_the_target_is_not_below_the_start() {
        let (mut e, s) = distance_scene(false);
        // Target 0 is not below the start distance 0.
        let (answer, result) = refine(&mut e, &s, 0.04, 0.0, 10.0);
        assert!(calls_to(&e, INTERPOLATE_TRANSFORM).is_empty());
        // The start fraction 0 is below the best 10: stored.
        assert_eq!(answer, 0.0);
        assert_eq!(e.mem.f32(result), 0.0);
    }

    #[test]
    fn final_refinement_stops_when_the_distances_do_not_fall() {
        let (mut e, s) = distance_scene(false);
        let plane = e.mem.alloc(0x20);
        put(&mut e, plane, [-0.3, 0.0, 0.0, 0.0]);
        e.mem.set_f32(plane + 0x10, 0.04);
        let point = vector(&mut e, [0.0, 0.0, 0.0, 1.0]);
        let result = e.mem.alloc(0x20);
        e.mem.set_f32(result, 10.0);
        // The end distance +1 is above the start distance 0: the step is not negative.
        e.call(
            0x00db_4ca0,
            &args![
                s.state,
                plane,
                point,
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                0u32,
                result
            ],
        );
        assert!(calls_to(&e, INTERPOLATE_TRANSFORM).is_empty());
    }

    /// The relative transform the double for `setMulInverseMul` writes: rows
    /// `(1,2,3,4)`, `(5,6,7,8)`, `(9,10,11,12)` and the translation `(0.5,1,1.5,2)`.
    fn write_relative(e: &mut Engine, a: &[u32]) -> Ret {
        put(e, a[0], [1.0, 2.0, 3.0, 4.0]);
        put(e, a[0] + 0x10, [5.0, 6.0, 7.0, 8.0]);
        put(e, a[0] + 0x20, [9.0, 10.0, 11.0, 12.0]);
        put(e, a[0] + 0x30, [0.5, 1.0, 1.5, 2.0]);
        Ret::default()
    }

    /// The closest feature the double for `getClosestFeature` writes: `(0, 1, 0)`, distance 2.5.
    fn write_feature(e: &mut Engine, a: &[u32]) -> Ret {
        put(e, a[4], [0.0, 1.0, 0.0, 2.5]);
        Ret::default()
    }

    /// Runs the separating normal with the simplex type words `(first, second)`; answers the
    /// two output vectors.
    fn separating(first: u32, second: u32) -> (Lanes, Lanes) {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.register(SET_MUL_INVERSE_MUL, write_relative);
        e.register(GET_CLOSEST_FEATURE, write_feature);
        let gsk = e.mem.alloc(0x140);
        e.mem.set_u32(gsk, first);
        e.mem.set_u32(gsk + 4, second);
        put(&mut e, gsk + 0x20, [4.0, 5.0, 6.0, 7.0]);
        put(&mut e, gsk + 0xa0, [1.0, 1.0, 1.0, 1.0]);
        put(&mut e, gsk + 0x130, [2.0, 2.0, 2.0, 2.0]);
        let transform_a = e.mem.alloc(0x40);
        put(&mut e, transform_a, [1.0, 0.0, 0.0, 0.0]);
        put(&mut e, transform_a + 0x10, [0.0, 2.0, 0.0, 0.0]);
        put(&mut e, transform_a + 0x20, [0.0, 0.0, 3.0, 0.0]);
        let transform_b = e.mem.alloc(0x40);
        let shape_a = e.mem.alloc(0x20);
        let shape_b = e.mem.alloc(0x20);
        e.mem.set_f32(shape_a + 0x10, 0.5);
        e.mem.set_f32(shape_b + 0x10, 0.25);
        let normal = e.mem.alloc(0x10);
        let second_out = e.mem.alloc(0x10);
        e.call(
            0x00db_52c0,
            &args![
                gsk,
                transform_a,
                transform_b,
                shape_a,
                shape_b,
                normal,
                second_out
            ],
        );
        let calls = calls_to(&e, SET_MUL_INVERSE_MUL);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..], [transform_a, transform_b]);
        let relative = calls[0][0];
        let closest = calls_to(&e, GET_CLOSEST_FEATURE);
        assert_eq!(closest.len(), 1);
        assert_eq!(closest[0][..4], [gsk, shape_a, shape_b, relative]);
        (load_lanes(&e, normal), load_lanes(&e, second_out))
    }

    /// The point `v` (three lanes) turned by the transposed rows `(1,2,3,4)`, `(5,6,7,8)`,
    /// `(9,10,11,12)`: columns `(1,5,9,10)`, `(2,6,10,12)`, `(3,7,11,12)`.
    fn turned(v: [f32; 3]) -> [u32; 4] {
        let columns = [
            [1.0f32, 5.0, 9.0, 10.0],
            [2.0, 6.0, 10.0, 12.0],
            [3.0, 7.0, 11.0, 12.0],
        ];
        [0, 1, 2, 3].map(|lane| {
            (v[1] * columns[1][lane] + v[0] * columns[0][lane] + columns[2][lane] * v[2]).to_bits()
        })
    }

    #[test]
    fn separating_normal_uses_the_first_simplex_vector_when_the_first_type_is_one() {
        let (normal, second) = separating(1, 0);
        // The direction (0, 1, 0) through the rows of the first transform: (0, 2, 0), its
        // fourth lane 2.5 - 0.5 - 0.25.
        assert_eq!(normal, [0.0f32, 2.0, 0.0, 1.75].map(f32::to_bits));
        // (4, 5, 6) - 2.5 * (0, 1, 0) - (0.5, 1, 1.5).
        assert_eq!(second, turned([3.5, 1.5, 4.5]));
    }

    #[test]
    fn separating_normal_adds_the_scaled_direction_for_the_second_type() {
        let (normal, second) = separating(0, 1);
        assert_eq!(normal, [0.0f32, 2.0, 0.0, 1.75].map(f32::to_bits));
        // (2.5 * A + (1, 1, 1)) - 2.5 * A - (0.5, 1, 1.5).
        assert_eq!(second, turned([0.5, 0.0, -0.5]));
    }

    #[test]
    fn separating_normal_falls_back_to_the_last_vector() {
        let (_, second) = separating(0, 0);
        assert_eq!(second, turned([1.5, -1.5, 0.5]));
    }

    /// A record whose first word points to an object with the vtable slots the state setup
    /// calls, and whose third word is a block with the float at `+0xa0`.
    fn record(e: &mut Engine, object: u32, block: u32) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, object);
        e.mem.set_u32(record + 8, block);
        record
    }

    #[test]
    fn state_setup_copies_the_blocks_and_builds_the_inertia_sums() {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.mem.set_f64(EPSILON, 1.192_092_9e-7);
        let cursor_cell = e.mem.alloc(8);
        e.mem.set_u32(cursor_cell, 0x2000);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(0x0100_0000),
            ..Ret::default()
        });
        e.mem.set_u32(0x0100_0000, cursor_cell);
        e.register(0x0111_0000, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(0x0111_0004, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x2c, 0x0111_0000);
        e.mem.set_u32(vtable + 0x30, 0x0111_0004);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        e.mem.set_f32(object + 0x10, 0.5);
        let other = e.mem.alloc(0x20);
        e.mem.set_f32(other + 0x10, 0.25);
        let first_block = e.mem.alloc(0xb0);
        e.mem.set_f32(first_block + 0xa0, 5.0);
        let second_block = e.mem.alloc(0xb0);
        e.mem.set_f32(second_block + 0xa0, 3.0);
        let first = record(&mut e, object, first_block);
        let second = record(&mut e, other, second_block);
        let input = e.mem.alloc(0x80);
        e.mem.set_u32(input, first);
        e.mem.set_u32(input + 4, second);
        e.mem.set_u32(input + 0xc, 0x4000);
        put(&mut e, input + 0x70, [1.0, 2.0, 3.0, 4.0]);
        let state = e.mem.alloc(0x90);
        e.mem.set_f32(state + 0x6c, 1.0);
        e.mem.set_f32(state + 0x7c, 2.0);
        e.call(0x00db_53f0, &args![input, state]);

        assert_eq!(e.mem.u32(state), first_block);
        assert_eq!(e.mem.u32(state + 4), second_block);
        assert_eq!(e.mem.f32(state + 0x14), 0.75);
        assert_eq!(
            load_lanes(&e, state + 0x50),
            [1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits)
        );
        assert_eq!(e.mem.u32(state + 0x40), 0x4050);
        assert_eq!(e.mem.u32(state + 0xc), 3);
        assert_eq!(e.mem.u32(state + 0x10), 3);
        // Three 16-byte elements round up to one 0x80 unit.
        assert_eq!(e.mem.u32(state + 8), 0x2000);
        assert_eq!(e.mem.u32(cursor_cell), 0x2080);
        assert_eq!(calls_to(&e, 0x0111_0004), vec![vec![object, 0x2000]]);
        // 3 * 2 * 2 + 5 * 1 * 1.
        assert_eq!(e.mem.f32(state + 0x18), 17.0);
        let reciprocal = e.mem.f32(state + 0x1c);
        assert!((reciprocal - 1.0 / 17.0).abs() < 1e-7, "{reciprocal}");
    }

    /// A collision scene for `collide_points_with_plane`: the interpolated transform is the
    /// identity (its fourth lane of the first row holds the fraction), the plane normal is
    /// `(1, 0, 0)` at a position `2 * fraction` along it, so a point's plane distance is its
    /// `x` minus twice the fraction. The first rotation row of the state is `(0, 0, 1)`, so a
    /// point's turning rate is minus its `y`. The plane limits are `1.0`, `0.9`, `0.0`.
    struct Scene {
        e: Engine,
        state: u32,
        plane: u32,
        out: u32,
        points: u32,
    }

    fn scene(points: &[[f32; 4]]) -> Scene {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.register(INTERPOLATE_TRANSFORM, |e, a| {
            let fraction = f32::from_bits(a[2]);
            put(e, a[3], [1.0, 0.0, 0.0, fraction]);
            put(e, a[3] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            put(e, a[3] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        e.register(SET_MUL_INVERSE_MUL, |_, _| Ret::default());
        e.register(SET_ROTATED_DIR, |e, a| {
            put(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        e.register(SET_TRANSFORMED_POS, |e, a| {
            let fraction = e.mem.f32(a[1] - 0x80 + 0xc);
            put(e, a[0], [2.0 * fraction, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        e.register(TRANSFORM_UNNAMED_004B4AB0, |e, a| {
            put(e, a[0], [0.0; 4]);
            Ret::default()
        });
        let state = e.mem.alloc(0x90);
        let first_body = e.mem.alloc(0x90);
        let second_body = e.mem.alloc(0x90);
        let times = e.mem.alloc(0x10);
        e.mem.set_u32(state, first_body);
        e.mem.set_u32(state + 4, second_body);
        e.mem.set_u32(state + 0x40, times);
        put(&mut e, times, [0.0, 0.0, 1.0, 0.0]);
        let array = e.mem.alloc(16 * points.len() as u32);
        for (index, point) in points.iter().enumerate() {
            put(&mut e, array + 16 * index as u32, *point);
        }
        e.mem.set_u32(state + 8, array);
        e.mem.set_u32(state + 0xc, points.len() as u32);
        put(&mut e, state + 0x14, [0.0; 4]);
        e.mem.set_f32(state + 0x18, 100.0);
        e.mem.set_f32(state + 0x1c, 0.25);
        e.mem.set_f32(state + 0x44, 1.0);
        put(&mut e, state + 0x60, [0.0, 0.0, 1.0, 0.0]);
        put(&mut e, state + 0x70, [0.0; 4]);
        e.mem.set_f32(state + 0x80, 0.0);
        let plane = e.mem.alloc(0x20);
        put(&mut e, plane, [1.0, 0.9, 0.0, 0.0]);
        let out = e.mem.alloc(0x20);
        e.mem.set_f32(out, 10.0);
        put(&mut e, out + 0x10, [7.0; 4]);
        Scene {
            e,
            state,
            plane,
            out,
            points: array,
        }
    }

    impl Scene {
        fn run(&mut self) {
            let (state, plane, out) = (self.state, self.plane, self.out);
            self.e.call(0x00db_4e40, &args![state, plane, out]);
        }

        /// The fraction of the `index`th matrix initialisation.
        fn fraction(&self, index: usize) -> f32 {
            f32::from_bits(calls_to(&self.e, INTERPOLATE_TRANSFORM)[index][2])
        }
    }

    #[test]
    fn points_with_plane_without_points_does_nothing() {
        let mut s = scene(&[[0.5, 0.0, 0.0, 7.0]]);
        s.e.mem.set_u32(s.state + 0xc, 0);
        s.run();
        assert!(calls_to(&s.e, INTERPOLATE_TRANSFORM).is_empty());
        assert_eq!(s.e.mem.f32(s.out), 10.0);
    }

    #[test]
    fn points_with_plane_stores_a_point_in_front_of_the_plane_at_the_first_fraction() {
        // The distance 0.5 is below the limit 1.0 and the start fraction is the first one.
        let mut s = scene(&[[0.5, 0.0, 0.0, 7.0]]);
        s.run();
        assert_eq!(s.e.mem.f32(s.out), 0.0);
        assert_eq!(
            load_lanes(&s.e, s.out + 0x10),
            [0.5f32, 0.0, 0.0, 7.0].map(f32::to_bits)
        );
        // One round: the next fraction is the best time 1.0, but no point was kept.
        assert_eq!(calls_to(&s.e, INTERPOLATE_TRANSFORM).len(), 2);
    }

    #[test]
    fn points_with_plane_moves_a_far_point_to_the_front_with_its_distance() {
        let mut s = scene(&[[3.0, 0.0, 0.0, 7.0], [1.5, 0.0, 0.0, 8.0]]);
        s.run();
        // The point at distance 3.0 is beyond the limit plus the margin 1.0 and stays; the
        // second one (distance 1.5) is moved to the front, its fourth lane the distance.
        let array = load_lanes(&s.e, s.points);
        assert_eq!(array, [1.5f32, 0.0, 0.0, 1.5].map(f32::to_bits));
        let second = load_lanes(&s.e, s.points + 0x10);
        assert_eq!(second, [3.0f32, 0.0, 0.0, 7.0].map(f32::to_bits));
    }

    #[test]
    fn points_with_plane_picks_the_next_fraction_from_the_turning_rate() {
        // (y of the point, the expected second fraction). A point at distance 1.5 with the
        // turning rate -y: the time to the limit is sqrt(0.3) for a rate of 0 (plus
        // sqrt(0.05) for the gap), 2 * rate / 4 added for a positive rate, the larger of the
        // root and rate / 4 for a negative one; too slow a point gives the best time 1.0.
        let expected = [
            (0.0f32, 0.3f64.sqrt() + 0.05f64.sqrt()),
            (-0.1, 0.3f64.sqrt() + 0.05 + 0.05f64.sqrt()),
            (3.0, 0.75 + 0.05f64.sqrt()),
            (-2.0, 1.0),
        ];
        for (y, fraction) in expected {
            let mut s = scene(&[[1.5, y, 0.0, 7.0]]);
            s.run();
            let second = f64::from(s.fraction(2));
            assert!((second - fraction).abs() < 1e-4, "y {y}: {second}");
        }
    }

    #[test]
    fn points_with_plane_refines_the_time_of_impact_in_the_second_round() {
        // The distance 1.5 - 2 * fraction reaches the limit 1.0 at the fraction 0.25.
        let mut s = scene(&[[1.5, 0.0, 0.0, 7.0]]);
        s.run();
        let impact = s.e.mem.f32(s.out);
        assert!((impact - 0.25).abs() < 5e-3, "{impact}");
        assert_eq!(
            load_lanes(&s.e, s.out + 0x10),
            [1.5f32, 0.0, 0.0, 1.5].map(f32::to_bits)
        );
        assert!(calls_to(&s.e, INTERPOLATE_TRANSFORM).len() > 4);
    }

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// The argument words of each call a double received, followed by what it saw in memory.
    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    const CALLBACK_DOUBLE: u32 = 0x0901_0000;
    const RELEASE_DOUBLE: u32 = 0x0901_0004;
    const FEATURES_DOUBLE: u32 = 0x0901_0008;
    const COUNT_DOUBLE: u32 = 0x0901_000c;
    const RECEIVE_DOUBLE: u32 = 0x0901_0010;

    /// A collision pair for `hk4d_gsk_calc_toi_final`. The first body stays at `x = 0`; the second
    /// goes from `x = 10` to `x = 2` over the time step (its position is `10 - 8 * f`, or
    /// `10 - 8 * f * f` for the squared pair, at the fraction `f` of the step), both shapes
    /// have the radius `0.5`, the closest feature is `(1, 0, 0)` at the distance between the
    /// two positions and the wanted distance is `3` within `0.001`.
    struct Pair {
        e: Engine,
        this: u32,
        first: u32,
        second: u32,
        time_record: u32,
        blocking_record: u32,
        limits: u32,
        gsk: u32,
        direction: u32,
        rates: u32,
        result: u32,
        shape_a: u32,
        shape_b: u32,
        body_a: u32,
        body_b: u32,
        callbacks: Calls,
        releases: Calls,
        features: Calls,
        recalcs: Calls,
        state: Rc<RefCell<Vec<u32>>>,
        answer: Rc<Cell<u32>>,
        code: Rc<Cell<u32>>,
    }

    fn pair(squared: bool) -> Pair {
        let mut e = rig();
        e.call_log = Some(Vec::new());
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f64(NO_TIME, f64::from(f32::MAX));
        e.mem.set_f32(NEGATIVE_EPSILON, -1.192_092_9e-7);
        e.mem.set_f32(RECORD_FLOAT, 320.0);
        e.mem.set_f64(EPSILON, 1.192_092_9e-7);
        for lane in 0..3 {
            e.mem.set_u32(NEGATE_XYZ + 4 * lane, 0x8000_0000);
        }
        e.register_double(INTERPOLATE_TRANSFORM, move |e, a| {
            let start = e.mem.f32(a[0]);
            let end = e.mem.f32(a[0] + 0x10);
            let step = f32::from_bits(a[2]);
            let along = if squared { step * step } else { step };
            put(e, a[3], [1.0, 0.0, 0.0, 0.0]);
            put(e, a[3] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            put(e, a[3] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            put(
                e,
                a[3] + 0x30,
                [start + (end - start) * along, 0.0, 0.0, 0.0],
            );
            Ret::default()
        });
        e.register(SET_MUL_INVERSE_MUL, |e, a| {
            put(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            put(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            put(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            let x = e.mem.f32(a[2] + 0x30) - e.mem.f32(a[1] + 0x30);
            put(e, a[0] + 0x30, [x, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        e.register(GET_CLOSEST_FEATURE, |e, a| {
            let x = e.mem.f32(a[3] + 0x30);
            put(e, a[4], [1.0, 0.0, 0.0, x]);
            Ret::default()
        });
        e.register(SET_TRANSFORMED_POS, |e, a| {
            let moved = [0u32, 1, 2, 3].map(|lane| {
                let value = e.mem.f32(a[2] + 4 * lane);
                if lane < 3 {
                    value + e.mem.f32(a[1] + 0x30 + 4 * lane)
                } else {
                    value
                }
            });
            put(e, a[0], moved);
            Ret::default()
        });
        e.register(SET_ROTATED_DIR, |e, a| {
            let copy = [0, 1, 2, 3].map(|lane| e.mem.f32(a[2] + 4 * lane));
            put(e, a[0], copy);
            Ret::default()
        });
        e.register(EXPORT_CACHE, |e, a| {
            put(e, a[1], [9.0, 8.0, 7.0, 6.0]);
            Ret::default()
        });
        let recalcs: Calls = Rc::default();
        let sink = Rc::clone(&recalcs);
        e.register_double(RECALC_CONTACT, move |e, a| {
            let mut words = a.to_vec();
            words.extend(load_lanes(e, a[4]));
            for at in [a[1] - 0x10, a[1], a[1] + 0x10] {
                words.extend(load_lanes(e, at));
            }
            sink.borrow_mut().push(words);
            put(e, a[5], [4.0, 5.0, 6.0, 7.0]);
            put(e, a[6], [1.0, 0.0, 0.0, 4.0]);
            Ret::default()
        });

        let router = e.mem.alloc(8);
        e.mem.set_u32(router, 0x2000);
        let state: Rc<RefCell<Vec<u32>>> = Rc::default();
        let state_at = Rc::new(Cell::new(0u32));
        let (sink, at) = (Rc::clone(&state), Rc::clone(&state_at));
        e.register_double(MEMORY_ROUTER, move |e, _| {
            if at.get() != 0 {
                *sink.borrow_mut() = (0..0x24)
                    .map(|word| e.mem.u32(at.get() + 4 * word))
                    .collect();
            }
            Ret {
                eax: router,
                ..Ret::default()
            }
        });
        let at = Rc::clone(&state_at);
        e.register_double(TRANSFORM_UNNAMED_004B4AB0, move |e, a| {
            at.set(a[0] - 0x20);
            put(e, a[0], [0.0; 4]);
            Ret::default()
        });

        let features: Calls = Rc::default();
        let code = Rc::new(Cell::new(2u32));
        let (sink, reply) = (Rc::clone(&features), Rc::clone(&code));
        e.register_double(FEATURES_DOUBLE, move |e, a| {
            let mut words = a.to_vec();
            words.push(u32::from(e.mem.u16(a[1])));
            words.push(u32::from(e.mem.u16(a[1] + 2)));
            words.push(u32::from(e.mem.u16(a[1] + 4)));
            words.push(u32::from(e.mem.u8(a[2])));
            words.extend(load_lanes(e, a[3]));
            words.extend(load_lanes(e, a[7]));
            sink.borrow_mut().push(words);
            Ret {
                eax: reply.get(),
                ..Ret::default()
            }
        });
        e.register(COUNT_DOUBLE, |_, _| Ret::default());
        e.register(RECEIVE_DOUBLE, |_, _| Ret::default());
        let shape_vtable = e.mem.alloc(0x80);
        e.mem.set_u32(shape_vtable + 0x2c, COUNT_DOUBLE);
        e.mem.set_u32(shape_vtable + 0x30, RECEIVE_DOUBLE);
        e.mem.set_u32(shape_vtable + 0x40, FEATURES_DOUBLE);

        let callbacks: Calls = Rc::default();
        let releases: Calls = Rc::default();
        let answer = Rc::new(Cell::new(0u32));
        let (sink, reply) = (Rc::clone(&callbacks), Rc::clone(&answer));
        e.register_double(CALLBACK_DOUBLE, move |e, a| {
            let mut words = a.to_vec();
            words.extend(load_lanes(e, a[6]));
            words.extend(load_lanes(e, a[7]));
            words.push(e.mem.u32(a[8]));
            words.extend((0..12).map(|word| e.mem.u32(a[9] + 4 * word)));
            sink.borrow_mut().push(words);
            Ret {
                eax: reply.get(),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&releases);
        e.register_double(RELEASE_DOUBLE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let callback_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(callback_vtable + 0x1c, CALLBACK_DOUBLE);
        e.mem.set_u32(callback_vtable + 0x20, RELEASE_DOUBLE);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, callback_vtable);

        let make_shape = |e: &mut Engine| {
            let shape = e.mem.alloc(0x20);
            e.mem.set_u32(shape, shape_vtable);
            e.mem.set_f32(shape + 0x10, 0.5);
            shape
        };
        let shape_a = make_shape(&mut e);
        let shape_b = make_shape(&mut e);
        let make_body = |e: &mut Engine, start: f32, end: f32, factor: f32| {
            let body = e.mem.alloc(0xb0);
            put(e, body + 0x40, [start, 0.0, 0.0, 0.0]);
            put(e, body + 0x50, [end, 0.0, 0.0, 1.0]);
            put(e, body + 0x90, [1.0, 2.0, 3.0, 4.0]);
            e.mem.set_f32(body + 0xa0, factor);
            body
        };
        let body_a = make_body(&mut e, 0.0, 0.0, 2.0);
        let body_b = make_body(&mut e, 10.0, 2.0, 3.0);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, shape_a);
        e.mem.set_u32(first + 8, body_a);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, shape_b);
        e.mem.set_u32(second + 8, body_b);
        let blocking_record = e.mem.alloc(0x40);
        let time_record = e.mem.alloc(0x80);
        put(&mut e, time_record + 0x50, [0.0, 10.0, 1.0, 2.0]);
        e.mem.set_u32(time_record + 0x60, blocking_record);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, first);
        e.mem.set_u32(this + 4, second);
        e.mem.set_u32(this + 0xc, time_record);
        e.mem.set_u32(this + 0x10, object);
        put(&mut e, this + 0x70, [3.0, 0.0, 0.0, 0.0]);
        let limits = e.mem.alloc(0x20);
        e.mem.set_f32(limits, 3.0);
        e.mem.set_f32(limits + 0xc, 0.1);
        e.mem.set_f32(limits + 0x10, 0.001);
        let gsk = e.mem.alloc(0x140);
        e.mem.set_u32(gsk, 1);
        put(&mut e, gsk + 0x20, [4.0, 5.0, 6.0, 7.0]);
        put(&mut e, gsk + 0xe0, [2.0, 4.0, 6.0, 8.0]);
        let direction = vector(&mut e, [2.0, 3.0, 4.0, 0.0]);
        let rates = e.mem.alloc(0x20);
        let result = e.mem.alloc(0x3100);
        e.mem.set_f32(result + 0x3030, f32::MAX);
        e.mem.set_u32(result + 4, 0x7777);
        Pair {
            e,
            this,
            first,
            second,
            time_record,
            blocking_record,
            limits,
            gsk,
            direction,
            rates,
            result,
            shape_a,
            shape_b,
            body_a,
            body_b,
            callbacks,
            releases,
            features,
            recalcs,
            state,
            answer,
            code,
        }
    }

    fn bits4(values: [f32; 4]) -> Lanes {
        values.map(f32::to_bits)
    }

    impl Pair {
        /// Runs from the times `times` with the distances `distances` found there.
        fn run_with(&mut self, query: u32, times: [f32; 2], distances: [f32; 2]) {
            self.e.call(
                0x00db_54b0,
                &args![
                    self.this,
                    self.limits,
                    self.gsk,
                    query,
                    self.direction,
                    self.rates,
                    times[0].to_bits(),
                    times[1].to_bits(),
                    distances[0].to_bits(),
                    distances[1].to_bits(),
                    self.result
                ],
            );
        }

        fn run(&mut self) {
            self.run_with(1, [0.0, 1.0], [9.0, 1.0]);
        }

        /// The steps of the interpolation calls (two per round).
        fn steps(&self) -> Vec<f32> {
            calls_to(&self.e, INTERPOLATE_TRANSFORM)
                .iter()
                .map(|call| f32::from_bits(call[2]))
                .collect()
        }

        /// The one call the callback object received.
        fn callback(&self) -> Vec<u32> {
            let calls = self.callbacks.borrow();
            assert_eq!(calls.len(), 1);
            calls[0].clone()
        }

        fn result_lanes(&self, at: u32) -> Lanes {
            load_lanes(&self.e, self.result + at)
        }
    }

    #[test]
    fn toi_final_finds_the_fraction_where_the_distance_matches_the_target() {
        let mut p = pair(false);
        p.run();
        // The distance is 9 - 8 f, so the first step of the secant lands on f = 0.75.
        assert_eq!(p.steps(), vec![0.75, 0.75]);
        let frames = calls_to(&p.e, INTERPOLATE_TRANSFORM);
        assert_eq!(frames[0][..2], [p.body_a + 0x40, 0.0f32.to_bits()]);
        assert_eq!(frames[1][..2], [p.body_b + 0x40, 0.0f32.to_bits()]);
        let closest = calls_to(&p.e, GET_CLOSEST_FEATURE);
        assert_eq!(closest.len(), 1);
        assert_eq!(closest[0][..3], [p.gsk, p.shape_a, p.shape_b]);
        assert!(calls_to(&p.e, RECALC_CONTACT).is_empty());

        let call = p.callback();
        let time = 0.75f32.to_bits();
        assert_eq!(
            call[..6],
            [
                callback_object(&p),
                p.first,
                p.second,
                p.time_record,
                p.result,
                time
            ]
        );
        // The point (-3.5 * (1, 0, 0, 4)) + (4, 5, 6, 7), the cache vector from the double, the
        // velocity (the speed -3 times the scale 2) and the record with the float 320.
        assert_eq!(call[10..14], bits4([0.5, 5.0, 6.0, -7.0]));
        assert_eq!(call[14..18], bits4([9.0, 8.0, 7.0, 6.0]));
        assert_eq!(call[18], (-6.0f32).to_bits());
        let mut record = [0u32; 12];
        record[4] = 320.0f32.to_bits();
        assert_eq!(call[19..31], record);
        assert!(p.releases.borrow().is_empty());

        assert_eq!(p.result_lanes(0x3010), bits4([0.5, 5.0, 6.0, -7.0]));
        assert_eq!(p.result_lanes(0x3020), bits4([1.0, 0.0, 0.0, 3.0]));
        assert_eq!(p.e.mem.f32(p.result + 0x3030), 0.75);
        assert_eq!(p.e.mem.f32(p.result + 0x3034), -6.0);
        assert_eq!(p.result_lanes(0x3040), bits4([9.0, 8.0, 7.0, 6.0]));
        let copied: Vec<u32> = (0..12)
            .map(|w| p.e.mem.u32(p.result + 0x3050 + 4 * w))
            .collect();
        assert_eq!(copied, record);
    }

    /// The callback object of the pair.
    fn callback_object(p: &Pair) -> u32 {
        p.e.mem.u32(p.this + 0x10)
    }

    #[test]
    fn toi_final_bisects_until_the_distance_is_within_the_tolerance() {
        let mut p = pair(true);
        p.e.mem.set_f32(p.limits, 5.0);
        p.e.mem.set_f32(p.limits + 0x10, 0.01);
        p.run();
        let steps = p.steps();
        assert!(
            (6..=20).contains(&steps.len()) && steps.len() % 2 == 0,
            "{steps:?}"
        );
        // The distance 9 - 8 f * f: the first step is 0.5 (distance 7), the second one
        // interpolates between the distances 7 and 1.
        assert_eq!(steps[0], 0.5);
        assert!((steps[2] - 2.0 / 3.0).abs() < 1e-6, "{steps:?}");
        let found = p.e.mem.f32(p.result + 0x3030);
        assert!((9.0 - 8.0 * found * found - 5.0).abs() < 0.01, "{found}");
    }

    #[test]
    fn toi_final_stops_after_ten_rounds() {
        let mut p = pair(true);
        p.e.mem.set_f32(p.limits, 5.0);
        p.e.mem.set_f32(p.limits + 0x10, 0.0);
        p.run();
        assert_eq!(p.steps().len(), 20);
        assert_eq!(p.callbacks.borrow().len(), 1);
    }

    #[test]
    fn toi_final_uses_half_as_the_span_when_the_distances_do_not_fall() {
        let mut p = pair(false);
        // The distances 9 and 9: the span 0 is not below minus the tolerance, so the upper time
        // becomes the lower one, the weight (3 - 9) / 0.5 is clamped to 0.1 and the fraction is 0.
        p.run_with(1, [0.0, 1.0], [9.0, 9.0]);
        assert_eq!(p.steps(), vec![0.0, 0.0]);
        // The time is the minimal time 0.1 (the step 0 is shorter).
        assert_eq!(p.e.mem.f32(p.result + 0x3030), 0.1);
        assert_eq!(p.callbacks.borrow().len(), 1);
    }

    #[test]
    fn toi_final_asks_for_a_recalculation_when_the_upper_distance_is_close() {
        let mut p = pair(false);
        p.run_with(1, [0.0, 1.0], [9.0, 3.0]);
        // The upper distance is within the tolerance: the fraction is the upper time and the
        // closest feature is not asked for.
        assert_eq!(p.steps(), vec![1.0, 1.0]);
        assert!(calls_to(&p.e, GET_CLOSEST_FEATURE).is_empty());
        let recalcs = p.recalcs.borrow();
        assert_eq!(recalcs.len(), 1);
        assert_eq!(recalcs[0][..4], [p.gsk + 0x20, p.gsk + 0xa0, 1, 0]);
        assert_eq!(p.e.mem.f32(p.result + 0x3030), 1.0);
    }

    #[test]
    fn toi_final_moves_the_points_into_the_relative_frame_when_not_asked_for_a_feature() {
        let mut p = pair(false);
        p.e.register(SET_MUL_INVERSE_MUL, write_relative);
        p.e.mem.set_u32(p.gsk + 4, 2);
        put(&mut p.e, p.gsk + 0xe0, [1.0, 2.0, 3.0, 5.0]);
        put(&mut p.e, p.gsk + 0xf0, [0.0, 1.0, 0.0, 0.0]);
        p.run_with(0, [0.0, 1.0], [9.0, 1.0]);
        // y * c1 + x * c0 + (z * c2 + c3) with the rows (1,2,3,4), (5,6,7,8), (9,10,11,12) and the
        // translation (0.5, 1, 1.5, 2).
        assert_eq!(
            load_lanes(&p.e, p.gsk + 0xa0),
            bits4([38.5, 45.0, 51.5, 58.0])
        );
        assert_eq!(load_lanes(&p.e, p.gsk + 0xb0), bits4([5.5, 7.0, 8.5, 10.0]));
        let recalcs = p.recalcs.borrow();
        assert_eq!(recalcs.len(), 1);
        assert_eq!(recalcs[0][..4], [p.gsk + 0x20, p.gsk + 0xa0, 1, 2]);
        // The direction (2, 3, 4) through the transposed identity rotation.
        assert_eq!(recalcs[0][7..11], bits4([2.0, 3.0, 4.0, 0.0]));
        assert!(calls_to(&p.e, GET_CLOSEST_FEATURE).is_empty());
    }

    #[test]
    fn toi_final_with_no_points_still_moves_the_one_below_the_array() {
        let mut p = pair(false);
        p.e.register(SET_MUL_INVERSE_MUL, write_relative);
        put(&mut p.e, p.gsk + 0xd0, [1.0, 0.0, 0.0, 0.0]);
        p.run_with(0, [0.0, 1.0], [9.0, 1.0]);
        assert_eq!(load_lanes(&p.e, p.gsk + 0x90), bits4([1.5, 3.0, 4.5, 6.0]));
    }

    #[test]
    fn toi_final_returns_when_the_best_time_is_not_later() {
        let mut p = pair(false);
        p.e.mem.set_f32(p.result + 0x3030, 0.5);
        p.run();
        assert!(p.callbacks.borrow().is_empty());
        assert!(calls_to(&p.e, EXPORT_CACHE).is_empty());
        assert_eq!(p.result_lanes(0x3010), [0; 4]);
        assert_eq!(p.e.mem.f32(p.result + 0x3030), 0.5);
    }

    #[test]
    fn toi_final_returns_when_the_time_passes_the_end_of_the_span() {
        let mut p = pair(false);
        // 0.8 - 0.1 is not above the time 0.75.
        p.e.mem.set_f32(p.time_record + 0x54, 0.8);
        p.run();
        assert!(p.callbacks.borrow().is_empty());
        assert!(calls_to(&p.e, SET_ROTATED_DIR).is_empty());
    }

    #[test]
    fn toi_final_leaves_the_record_when_the_callback_refuses() {
        let mut p = pair(false);
        p.answer.set(1);
        p.run();
        assert_eq!(p.callbacks.borrow().len(), 1);
        assert!(p.releases.borrow().is_empty());
        assert_eq!(p.result_lanes(0x3010), [0; 4]);
        assert_eq!(p.e.mem.f32(p.result + 0x3030), f32::MAX);
    }

    #[test]
    fn toi_final_releases_the_old_record_when_there_was_a_best_time() {
        let mut p = pair(false);
        p.e.mem.set_f32(p.result + 0x3030, 5.0);
        p.run();
        let object = callback_object(&p);
        assert_eq!(
            *p.releases.borrow(),
            vec![vec![object, 0x7777, p.result + 0x3050]]
        );
        assert_eq!(p.e.mem.f32(p.result + 0x3030), 0.75);
    }

    #[test]
    fn toi_final_limits_the_speed_by_the_slope_of_the_distances() {
        // (the first lane of the own vector, the velocity in the record): the speed is minus
        // that lane; the slope is -8. A speed below the slope stays (-20 * 2); one above it
        // whose 1.2 multiple is below it becomes the slope (-8 * 2); otherwise the point
        // velocity error is used (-3 * 2), and a positive one gives 0.
        for (own, velocity) in [(20.0f32, -40.0f32), (7.0, -16.0), (3.0, -6.0), (-2.0, 0.0)] {
            let mut p = pair(false);
            put(&mut p.e, p.this + 0x70, [own, 0.0, 0.0, 0.0]);
            p.run();
            assert_eq!(p.e.mem.f32(p.result + 0x3034), velocity, "own {own}");
        }
    }

    #[test]
    fn toi_final_asks_a_type_three_second_shape_for_its_features() {
        let mut p = pair(false);
        p.e.mem.set_u32(p.shape_b + 0xc, 3);
        p.e.mem.set_u32(p.gsk + 4, 2);
        p.e.mem.set_u16(p.gsk + 0xec, 0x1111);
        p.e.mem.set_u16(p.gsk + 0xfc, 0x2222);
        p.e.mem.set_u16(p.gsk + 0x10c, 0x3333);
        p.code.set(3);
        p.run();
        let features = p.features.borrow();
        assert_eq!(features.len(), 1);
        let call = &features[0];
        assert_eq!(call[0], p.shape_b);
        assert_eq!(call[5], p.shape_a);
        // The keys: the first two from the hkpGsk, 0xffff for the missing third; two features.
        assert_eq!(call[8..12], [0x1111, 0x2222, 0xffff, 2]);
        assert_eq!(call[12..16], bits4([0.5, 5.0, 6.0, -7.0]));
        assert_eq!(call[16..20], bits4([1.0, 0.0, 0.0, 3.0]));
        assert_eq!(p.callbacks.borrow().len(), 1);
    }

    #[test]
    fn toi_final_reads_all_three_keys_of_three_features() {
        let mut p = pair(false);
        p.e.mem.set_u32(p.shape_b + 0xc, 3);
        p.e.mem.set_u32(p.gsk + 4, 3);
        p.e.mem.set_u16(p.gsk + 0xec, 0x1111);
        p.e.mem.set_u16(p.gsk + 0xfc, 0x2222);
        p.e.mem.set_u16(p.gsk + 0x10c, 0x3333);
        p.run();
        assert_eq!(p.features.borrow()[0][8..12], [0x1111, 0x2222, 0x3333, 3]);
    }

    #[test]
    fn toi_final_stops_when_a_type_three_shape_finds_nothing_and_the_flag_is_set() {
        for (flag, calls) in [(1u8, 0usize), (0, 1)] {
            let mut p = pair(false);
            p.e.mem.set_u32(p.shape_b + 0xc, 3);
            p.code.set(0);
            p.e.mem.set_u8(p.blocking_record + 0x3e, flag);
            p.run();
            assert_eq!(p.callbacks.borrow().len(), calls, "flag {flag}");
        }
    }

    #[test]
    fn toi_final_asks_a_type_three_first_shape_with_the_flipped_normal() {
        let mut p = pair(false);
        p.e.mem.set_u32(p.shape_a + 0xc, 3);
        p.e.mem.set_u32(p.gsk, 1);
        p.e.mem.set_u16(p.gsk + 0x2c, 0x4444);
        p.code.set(3);
        p.run();
        let features = p.features.borrow();
        assert_eq!(features.len(), 1);
        let call = &features[0];
        assert_eq!(call[0], p.shape_a);
        assert_eq!(call[5], p.shape_b);
        assert_eq!(call[8..12], [0x4444, 0xffff, 0xffff, 1]);
        // distance * (1, 0, 0, 3) + (0.5, 5, 6, -7), and the normal with its first three signs
        // flipped (the fourth lane of the point holds the key 0x4444 in its low half).
        assert_eq!(call[12..15], bits4([3.5, 5.0, 6.0, 0.0])[..3]);
        assert!((f32::from_bits(call[15]) - 2.0).abs() < 0.02);
        assert_eq!(call[16..20], bits4([-1.0, -0.0, -0.0, 3.0]));
        // The normal is flipped back for the result.
        assert_eq!(p.result_lanes(0x3020), bits4([1.0, 0.0, 0.0, 3.0]));
    }

    #[test]
    fn toi_final_bounds_the_speed_with_the_points_of_a_shape_with_result_code_one() {
        let mut p = pair(false);
        p.e.mem.set_u32(p.shape_b + 0xc, 3);
        p.code.set(1);
        put(&mut p.e, p.this + 0x70, [-2.0, 0.0, 0.0, 0.0]);
        p.run();
        assert_eq!(calls_to(&p.e, COUNT_DOUBLE), vec![vec![p.shape_a]]);
        assert_eq!(
            calls_to(&p.e, RECEIVE_DOUBLE),
            vec![vec![p.shape_a, 0x2000]]
        );
        let state = p.state.borrow().clone();
        assert_eq!(state.len(), 0x24);
        assert_eq!(state[0], p.body_a);
        assert_eq!(state[1], p.body_b);
        assert_eq!(state[2], 0x2000);
        assert_eq!(state[3..5], [0, 0]);
        assert_eq!(f32::from_bits(state[5]), 1.0);
        // 3 * 4 * 4 + 2 * 4 * 4.
        assert_eq!(f32::from_bits(state[6]), 80.0);
        let reciprocal = f32::from_bits(state[7]);
        assert!((reciprocal - 1.0 / 80.0).abs() < 1e-7, "{reciprocal}");
        assert_eq!(state[12..16], bits4([2.0, 4.0, 6.0, 8.0]));
        assert_eq!(state[16], p.time_record + 0x50);
        // 5 * sqrt(13) - 2: the inertia vectors (1, 2, 3, 4) crossed with (1, 0, 0) have the
        // length sqrt(13), the factors are 2 and 3 and the own velocity along the normal -2.
        let bound = f32::from_bits(state[17]);
        assert!(
            (bound - (5.0 * 13.0f32.sqrt() - 2.0)).abs() < 1e-3,
            "{bound}"
        );
        assert_eq!(state[20..24], bits4([-2.0, 0.0, 0.0, 0.0]));
        assert_eq!(state[24..28], bits4([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(state[28..32], bits4([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(f32::from_bits(state[32]), 0.75);
        assert_eq!(p.callbacks.borrow().len(), 1);
    }

    const LOADER_DOUBLE: u32 = 0x0901_0014;

    /// Whether the four lanes are within `tolerance` of the wanted floats.
    fn near(actual: &[u32], wanted: [f32; 4], tolerance: f32) -> bool {
        actual
            .iter()
            .zip(wanted)
            .all(|(&bits, want)| (f32::from_bits(bits) - want).abs() <= tolerance)
    }

    /// A setup for `hk4d_gsk_collide_calc_toi` on the pair of the final tests: the closest
    /// vector is `(1, 0, 0)` at the distance 9 and the time step goes from the start 0 to 10; the
    /// distance at the fraction `f` of the step is `9 - 8 f`. The tolerance is `0.001`, the
    /// minimal time and the steps `0`, the first shape has one point and the second none. The
    /// loader of the shapes (slot `0x3c`) fills the points `(4, 5, 6, 7)` and `(2, 4, 6, 8)` and
    /// the feature keys; the cache export copies the state of the first rounds.
    struct Sweep {
        p: Pair,
        keys: u32,
        loads: Calls,
        exports: Calls,
        /// Whether the loader of the first and of the second shape writes feature keys (into the
        /// fourth lanes of the points).
        keyed: Rc<Cell<[bool; 2]>>,
    }

    fn sweep() -> Sweep {
        let mut p = pair(false);
        p.e.mem.set_f64(STEP_BOUND, 0.0);
        p.e.mem.set_f64(TIME_END, 1.0);
        p.e.mem.set_f32(p.blocking_record + 0x30, 0.001);
        put(&mut p.e, p.direction, [1.0, 0.0, 0.0, 9.0]);
        let keys = p.e.mem.alloc(0x20);
        p.e.mem.set_u8(keys + 8, 1);
        p.e.mem.set_u8(keys + 10, 0x21);
        p.e.register(GET_CLOSEST_FEATURE, |e, a| {
            let x = e.mem.f32(a[3] + 0x30);
            put(e, a[4], [1.0, 0.0, 0.0, x]);
            e.mem.set_u32(a[0] + 0x14, 1);
            Ret::default()
        });
        let loads: Calls = Rc::default();
        let sink = Rc::clone(&loads);
        let keyed = Rc::new(Cell::new([false; 2]));
        let writes = Rc::clone(&keyed);
        let first_shape = p.shape_a;
        p.e.register_double(LOADER_DOUBLE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            let (point, keys) = if a[0] == first_shape {
                ([4.0, 5.0, 6.0, 7.0], [0x4444, 0x5555, 0x6666])
            } else {
                ([2.0, 4.0, 6.0, 8.0], [0x1111, 0x2222, 0x3333])
            };
            put(e, a[3], point);
            let wanted = writes.get()[usize::from(a[0] != first_shape)];
            let count = if wanted { a[2].min(3) as usize } else { 0 };
            for (index, key) in keys.into_iter().enumerate().take(count) {
                e.mem.set_u16(a[3] + 0xc + 0x10 * index as u32, key);
            }
            if a[0] != first_shape {
                put(e, a[3] - 0x10, [1.0, 0.0, 0.0, 0.0]);
            }
            Ret::default()
        });
        let vtable = p.e.mem.u32(p.shape_a);
        p.e.mem.set_u32(vtable + 0x3c, LOADER_DOUBLE);
        let exports: Calls = Rc::default();
        let sink = Rc::clone(&exports);
        p.e.register_double(EXPORT_CACHE, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..0x24).map(|word| e.mem.u32(a[0] - 0x170 + 4 * word)));
            sink.borrow_mut().push(words);
            put(e, a[1], [9.0, 8.0, 7.0, 6.0]);
            Ret::default()
        });
        Sweep {
            p,
            keys,
            loads,
            exports,
            keyed,
        }
    }

    impl Sweep {
        fn run_with(&mut self, minimum: f32, target: f32) {
            self.p.e.call(
                0x00db_60c0,
                &args![
                    self.p.this,
                    1.0f32.to_bits(),
                    minimum.to_bits(),
                    target.to_bits(),
                    self.keys,
                    self.p.direction,
                    self.p.result
                ],
            );
        }

        fn run(&mut self) {
            self.run_with(0.0, 3.0);
        }

        /// Makes the minimal step `2 * 0.5 = 1`, so the first round goes to the end of the time
        /// step (the distance there is 1) and the secant gives the exact fraction 0.75.
        fn stepped(&mut self) {
            self.p.e.mem.set_f32(self.p.blocking_record + 0x24, 0.5);
        }

        /// The `hkpGsk` on the frame of the function, from the first closest-feature search.
        fn gsk(&self) -> u32 {
            calls_to(&self.p.e, GET_CLOSEST_FEATURE)[0][0]
        }
    }

    #[test]
    fn collide_toi_steps_to_the_target_distance_then_takes_the_secant_to_the_fraction() {
        let mut s = sweep();
        s.run();
        // Each step adds the distance over the bound 5 * sqrt(13) + 3 = 21.0278 to the time; the
        // distance 9 - 8 t is then 2.14, below the target 3 plus the tolerance, and the secant
        // between the last two times lands on the fraction 0.75.
        let steps = s.p.steps();
        assert_eq!(steps.len(), 8);
        for (round, wanted) in [0.428_006f32, 0.693_177, 0.857_464, 0.75]
            .into_iter()
            .enumerate()
        {
            for body in 0..2 {
                assert!((steps[2 * round + body] - wanted).abs() < 1e-4, "{steps:?}");
            }
        }
        let frames = calls_to(&s.p.e, INTERPOLATE_TRANSFORM);
        assert_eq!(frames[0][..2], [s.p.body_a + 0x40, 0.0f32.to_bits()]);
        assert_eq!(frames[1][..2], [s.p.body_b + 0x40, 0.0f32.to_bits()]);
        let features = calls_to(&s.p.e, GET_CLOSEST_FEATURE);
        assert_eq!(features.len(), 4);
        let gsk = s.gsk();
        for call in &features {
            assert_eq!(call[..3], [gsk, s.p.shape_a, s.p.shape_b]);
        }
        // The relative frame of the steps and the one of the secant differ.
        assert_eq!(features[1][3], features[0][3]);
        assert_eq!(features[2][3], features[0][3]);
        assert_eq!(features[3][3], features[0][3] + 0xf0);
        assert_eq!(
            *s.loads.borrow(),
            vec![
                vec![s.p.shape_a, s.keys, 1, gsk + 0x20],
                vec![s.p.shape_b, s.keys + 2, 0, gsk + 0xe0]
            ]
        );
        // The closest vector is turned by the first body (the identity) and holds the last
        // distance.
        let closest = load_lanes(&s.p.e, s.p.direction);
        assert_eq!(closest[..3], bits4([1.0, 0.0, 0.0, 0.0])[..3]);
        assert!((f32::from_bits(closest[3]) - 2.140_288).abs() < 1e-3);

        let call = s.p.callback();
        assert_eq!(
            call[1..5],
            [s.p.first, s.p.second, s.p.time_record, s.p.result]
        );
        assert!((f32::from_bits(call[5]) - 0.75).abs() < 1e-4);
        // The contact point (-3.5 * (1, 0, 0, 4)) + (4, 5, 6, 7), the cache vector from the
        // double and the speed: minus the point velocity error 4.1 times the scale 2.
        assert!(near(&call[10..14], [0.5, 5.0, 6.0, -7.0], 1e-3));
        assert_eq!(call[14..18], bits4([9.0, 8.0, 7.0, 6.0]));
        assert!((f32::from_bits(call[18]) + 8.2).abs() < 1e-2);
        let mut record = [0u32; 12];
        record[4] = 320.0f32.to_bits();
        assert_eq!(call[19..31], record);
        assert!(s.p.releases.borrow().is_empty());

        assert!(near(&s.p.result_lanes(0x3010), [0.5, 5.0, 6.0, -7.0], 1e-3));
        assert!(near(&s.p.result_lanes(0x3020), [1.0, 0.0, 0.0, 3.0], 1e-3));
        assert!((s.p.e.mem.f32(s.p.result + 0x3030) - 0.75).abs() < 1e-4);
        assert!((s.p.e.mem.f32(s.p.result + 0x3034) + 8.2).abs() < 1e-2);
        assert_eq!(s.p.result_lanes(0x3040), bits4([9.0, 8.0, 7.0, 6.0]));
        let copied: Vec<u32> = (0..12)
            .map(|w| s.p.e.mem.u32(s.p.result + 0x3050 + 4 * w))
            .collect();
        assert_eq!(copied, record);
        // The cache is exported for the callbacks and again, into the key data, at the end.
        let exports = s.exports.borrow();
        assert_eq!(exports.len(), 2);
        assert_eq!(exports[0][..2], [gsk, gsk - 0x250]);
        assert_eq!(exports[1][..2], [gsk, s.keys]);
    }

    #[test]
    fn collide_toi_skips_the_steps_when_the_distance_is_already_close() {
        let mut s = sweep();
        put(&mut s.p.e, s.p.direction, [1.0, 0.0, 0.0, 2.9]);
        s.run();
        // The times are both 0: the span is 0, the weight (3 - 2.9) / 0.5 = 0.2 of the lower time
        // 0, and the lower time equals the upper one after the first search.
        assert_eq!(s.p.steps(), vec![0.0, 0.0]);
        assert_eq!(calls_to(&s.p.e, GET_CLOSEST_FEATURE).len(), 1);
        assert_eq!(s.p.callback()[5], 0.0f32.to_bits());
        assert_eq!(s.p.e.mem.f32(s.p.result + 0x3030), 0.0);
        assert_eq!(s.exports.borrow().len(), 2);
    }

    #[test]
    fn collide_toi_takes_at_least_the_minimal_step() {
        // The larger of the two tolerance fields times the velocity scale 2 is the minimal step.
        for (reach, lowest) in [(0.5f32, 0.0f32), (0.0, 0.5)] {
            let mut s = sweep();
            s.p.e.mem.set_f32(s.p.blocking_record + 0x20, reach);
            s.p.e.mem.set_f32(s.p.blocking_record + 0x24, lowest);
            s.run();
            assert_eq!(s.p.steps(), vec![1.0, 1.0, 0.75, 0.75], "{reach} {lowest}");
            assert_eq!(s.p.callback()[5], 0.75f32.to_bits());
            assert_eq!(s.p.e.mem.f32(s.p.result + 0x3030), 0.75);
            assert_eq!(
                s.p.result_lanes(0x3010),
                bits4([0.5, 5.0, 6.0, -7.0]),
                "{reach} {lowest}"
            );
            assert_eq!(s.p.result_lanes(0x3020), bits4([1.0, 0.0, 0.0, 3.0]));
        }
    }

    #[test]
    fn collide_toi_leaves_at_once_when_the_bound_or_the_gate_stops_it() {
        // (own velocity along the vector, distance, minimum): a negative bound; a minimum
        // below what the remaining time can bring.
        for (own, distance, minimum) in [(-100.0f32, 9.0f32, 0.0f32), (3.0, 9.0, -100.0)] {
            let mut s = sweep();
            put(&mut s.p.e, s.p.this + 0x70, [own, 0.0, 0.0, 0.0]);
            put(&mut s.p.e, s.p.direction, [1.0, 0.0, 0.0, distance]);
            s.run_with(minimum, 3.0);
            assert!(calls_to(&s.p.e, INTERPOLATE_TRANSFORM).is_empty());
            assert!(calls_to(&s.p.e, GET_CLOSEST_FEATURE).is_empty());
            assert!(s.p.callbacks.borrow().is_empty());
            assert!(s.exports.borrow().is_empty());
            assert_eq!(s.loads.borrow().len(), 2);
            assert_eq!(s.p.e.mem.f32(s.p.result + 0x3030), f32::MAX);
        }
    }

    #[test]
    fn collide_toi_leaves_when_no_time_remains() {
        let mut s = sweep();
        s.stepped();
        // The target 0.5: after the minimal step to the time 1 the distance is still 1, so a
        // second round finds no time left.
        s.run_with(0.0, 0.5);
        assert_eq!(s.p.steps(), vec![1.0, 1.0]);
        assert!(s.p.callbacks.borrow().is_empty());
        assert_eq!(s.exports.borrow().len(), 1);
    }

    #[test]
    fn collide_toi_asks_for_a_recalculation_when_the_distance_is_close_to_the_target() {
        let mut s = sweep();
        s.p.e.register(SET_MUL_INVERSE_MUL, write_relative);
        s.p.e.mem.set_u8(s.keys + 9, 2);
        put(&mut s.p.e, s.p.direction, [1.0, 0.0, 0.0, 3.0005]);
        s.run();
        // The span is 0, so the upper time becomes the lower one; the distance is within the
        // tolerance of the target: the fraction is the upper time 0 and the contact is
        // recalculated from the points moved to the relative frame.
        assert_eq!(s.p.steps(), vec![0.0, 0.0]);
        assert!(calls_to(&s.p.e, GET_CLOSEST_FEATURE).is_empty());
        let gsk = calls_to(&s.p.e, RECALC_CONTACT)[0][1] - 0xa0;
        let recalcs = s.p.recalcs.borrow();
        assert_eq!(recalcs.len(), 1);
        assert_eq!(recalcs[0][..4], [gsk + 0x20, gsk + 0xa0, 1, 2]);
        assert_eq!(recalcs[0][4..7], [gsk - 0x2a0, gsk - 0x230, gsk - 0x240]);
        // The saved vector is zero in the first search.
        assert_eq!(recalcs[0][7..11], [0; 4]);
        // The second point is zero, the first is (2, 4, 6, 8) moved with the rows
        // (1,2,3,4), (5,6,7,8), (9,10,11,12) and the translation (0.5, 1, 1.5, 2).
        assert_eq!(recalcs[0][15..19], bits4([76.5, 89.0, 101.5, 114.0]));
        assert_eq!(recalcs[0][19..23], bits4([0.5, 1.0, 1.5, 2.0]));
        // The cache is not exported at the end (the closest feature was never asked for).
        assert_eq!(s.exports.borrow().len(), 1);
        assert_eq!(s.p.callbacks.borrow().len(), 1);
    }

    #[test]
    fn collide_toi_with_no_points_still_moves_the_one_below_the_array() {
        let mut s = sweep();
        s.p.e.register(SET_MUL_INVERSE_MUL, write_relative);
        put(&mut s.p.e, s.p.direction, [1.0, 0.0, 0.0, 3.0005]);
        s.run();
        let recalcs = s.p.recalcs.borrow();
        assert_eq!(recalcs[0][2..4], [1, 0]);
        // (1, 0, 0, 0) moved: the first row plus the translation.
        assert_eq!(recalcs[0][11..15], bits4([1.5, 3.0, 4.5, 6.0]));
    }

    #[test]
    fn collide_toi_ends_the_steps_when_the_points_leave_no_room() {
        let mut s = sweep();
        s.p.e.mem.set_f64(STEP_BOUND, f64::from(0.2f32));
        s.run();
        // The third step (0.164) is below the bound 0.2 after a first search: the point state is
        // built, the plane collision moves nothing and the next time is the end of the step.
        assert!(s.p.callbacks.borrow().is_empty());
        let gsk = s.gsk();
        assert_eq!(calls_to(&s.p.e, COUNT_DOUBLE), vec![vec![s.p.shape_a]]);
        assert_eq!(
            calls_to(&s.p.e, RECEIVE_DOUBLE),
            vec![vec![s.p.shape_a, 0x2000]]
        );
        // The router hands out the cursor and takes it back at the end.
        assert_eq!(calls_to(&s.p.e, MEMORY_ROUTER).len(), 2);
        assert_eq!(calls_to(&s.p.e, INTERPOLATE_TRANSFORM).len(), 4);
        let exports = s.exports.borrow();
        assert_eq!(exports.len(), 1);
        assert_eq!(exports[0][..2], [gsk, s.keys]);
        let state = &exports[0][2..];
        assert_eq!(state[0], s.p.body_a);
        assert_eq!(state[1], s.p.body_b);
        assert_eq!(state[2], 0x2000);
        assert_eq!(state[3..5], [0, 0]);
        assert_eq!(f32::from_bits(state[5]), 1.0);
        assert_eq!(f32::from_bits(state[6]), 80.0);
        assert!((f32::from_bits(state[7]) - 1.0 / 80.0).abs() < 1e-7);
        assert_eq!(state[16], s.p.time_record + 0x50);
        // The bound 21.0278 and the time 0.693177 of the second step.
        assert!((f32::from_bits(state[17]) - 21.027_756).abs() < 1e-3);
        assert_eq!(state[20..24], bits4([3.0, 0.0, 0.0, 0.0]));
        assert_eq!(state[24..28], bits4([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(state[28..32], bits4([1.0, 2.0, 3.0, 4.0]));
        assert!((f32::from_bits(state[32]) - 0.693_177).abs() < 1e-4);
    }

    #[test]
    fn collide_toi_returns_when_the_best_time_is_not_later() {
        let mut s = sweep();
        s.stepped();
        s.p.e.mem.set_f32(s.p.result + 0x3030, 0.5);
        s.run();
        assert!(s.p.callbacks.borrow().is_empty());
        assert!(calls_to(&s.p.e, SET_ROTATED_DIR).is_empty());
        assert_eq!(s.p.result_lanes(0x3010), [0; 4]);
        assert_eq!(s.p.e.mem.f32(s.p.result + 0x3030), 0.5);
        // Only the export at the end.
        assert_eq!(s.exports.borrow().len(), 1);
    }

    #[test]
    fn collide_toi_returns_when_the_time_passes_the_end_of_the_span() {
        let mut s = sweep();
        s.stepped();
        // 0.8 - 0.1 is not above the time 0.75.
        s.p.e.mem.set_f32(s.p.time_record + 0x54, 0.8);
        s.p.e.mem.set_f32(s.p.blocking_record + 0x38, 0.1);
        s.run();
        assert!(s.p.callbacks.borrow().is_empty());
        assert!(calls_to(&s.p.e, SET_ROTATED_DIR).is_empty());
    }

    #[test]
    fn collide_toi_takes_the_floor_of_the_time_when_the_step_is_shorter() {
        let mut s = sweep();
        s.stepped();
        s.p.e.mem.set_f32(s.p.blocking_record + 0x38, 0.8);
        s.run();
        // The fraction 0.75 is below the floor 0.8, which is the time.
        assert_eq!(s.p.callback()[5], 0.8f32.to_bits());
    }

    #[test]
    fn collide_toi_leaves_the_record_when_the_callback_refuses() {
        let mut s = sweep();
        s.stepped();
        s.p.answer.set(1);
        s.run();
        assert_eq!(s.p.callbacks.borrow().len(), 1);
        assert!(s.p.releases.borrow().is_empty());
        assert_eq!(s.p.result_lanes(0x3010), [0; 4]);
        assert_eq!(s.p.e.mem.f32(s.p.result + 0x3030), f32::MAX);
    }

    #[test]
    fn collide_toi_releases_the_old_record_when_there_was_a_best_time() {
        let mut s = sweep();
        s.stepped();
        s.p.e.mem.set_f32(s.p.result + 0x3030, 5.0);
        s.run();
        let object = callback_object(&s.p);
        assert_eq!(
            *s.p.releases.borrow(),
            vec![vec![object, 0x7777, s.p.result + 0x3050]]
        );
        assert_eq!(s.p.e.mem.f32(s.p.result + 0x3030), 0.75);
    }

    #[test]
    fn collide_toi_limits_the_speed_by_the_slope_of_the_distances() {
        // (the first lane of the own vector, the velocity in the record): the speed is minus
        // that lane and the slope is -8. A speed below the slope stays (-20 * 2); one above it
        // whose 1.2 multiple is below it becomes the slope (-8 * 2); otherwise the point
        // velocity error is used (a positive one gives 0).
        for (own, velocity) in [(20.0f32, -40.0f32), (7.0, -16.0), (-2.0, 0.0)] {
            let mut s = sweep();
            s.stepped();
            put(&mut s.p.e, s.p.this + 0x70, [own, 0.0, 0.0, 0.0]);
            s.run();
            assert_eq!(s.p.e.mem.f32(s.p.result + 0x3034), velocity, "own {own}");
        }
        let mut s = sweep();
        s.stepped();
        s.run();
        assert!((s.p.e.mem.f32(s.p.result + 0x3034) + 8.2).abs() < 1e-2);
    }

    #[test]
    fn collide_toi_asks_a_type_three_second_shape_for_its_features() {
        let mut s = sweep();
        s.keyed.set([false, true]);
        s.stepped();
        s.p.e.mem.set_u32(s.p.shape_b + 0xc, 3);
        s.p.e.mem.set_u8(s.keys + 9, 2);
        s.p.code.set(3);
        s.run();
        let features = s.p.features.borrow();
        assert_eq!(features.len(), 1);
        let call = &features[0];
        assert_eq!(call[0], s.p.shape_b);
        assert_eq!(call[5], s.p.shape_a);
        // The keys and their count sit at the frame offsets `0x1c` and `0x17`.
        assert_eq!(call[1], call[2] + 5);
        assert_eq!(call[3], call[2] + 0x49);
        assert_eq!(call[4], call[6] + 0x350);
        // The keys: the first two from the second shape, 0xffff for the missing third.
        assert_eq!(call[8..12], [0x1111, 0x2222, 0xffff, 2]);
        assert_eq!(call[12..16], bits4([0.5, 5.0, 6.0, -7.0]));
        assert_eq!(call[16..20], bits4([1.0, 0.0, 0.0, 3.0]));
        assert_eq!(s.p.callbacks.borrow().len(), 1);
    }

    #[test]
    fn collide_toi_reads_all_three_keys_of_three_features() {
        let mut s = sweep();
        s.keyed.set([false, true]);
        s.stepped();
        s.p.e.mem.set_u32(s.p.shape_b + 0xc, 3);
        s.p.e.mem.set_u8(s.keys + 9, 3);
        s.run();
        assert_eq!(s.p.features.borrow()[0][8..12], [0x1111, 0x2222, 0x3333, 3]);
    }

    #[test]
    fn collide_toi_stops_when_a_type_three_shape_finds_nothing_and_the_flag_is_set() {
        for (flag, calls) in [(1u8, 0usize), (0, 1)] {
            let mut s = sweep();
            s.stepped();
            s.p.e.mem.set_u32(s.p.shape_b + 0xc, 3);
            s.p.code.set(0);
            s.p.e.mem.set_u8(s.p.blocking_record + 0x3e, flag);
            s.run();
            assert_eq!(s.p.callbacks.borrow().len(), calls, "flag {flag}");
        }
    }

    #[test]
    fn collide_toi_asks_a_type_three_first_shape_with_the_flipped_normal() {
        let mut s = sweep();
        s.keyed.set([true, false]);
        s.stepped();
        s.p.e.mem.set_u32(s.p.shape_a + 0xc, 3);
        s.p.code.set(3);
        s.run();
        let features = s.p.features.borrow();
        assert_eq!(features.len(), 1);
        let call = &features[0];
        assert_eq!(call[0], s.p.shape_a);
        assert_eq!(call[5], s.p.shape_b);
        assert_eq!(call[8..12], [0x4444, 0xffff, 0xffff, 1]);
        assert_eq!(call[6], call[4] + 0x350);
        // distance * (1, 0, 0, 3) + (0.5, 5, 6, -7), and the normal with its first three signs
        // flipped (the fourth lane of the point holds the key 0x4444 in its low half).
        assert_eq!(call[12..15], bits4([3.5, 5.0, 6.0, 0.0])[..3]);
        assert!((f32::from_bits(call[15]) - 2.0).abs() < 0.02);
        assert_eq!(call[16..20], bits4([-1.0, -0.0, -0.0, 3.0]));
        // The normal is flipped back for the result.
        assert_eq!(s.p.result_lanes(0x3020), bits4([1.0, 0.0, 0.0, 3.0]));
    }

    #[test]
    fn collide_toi_bounds_the_speed_with_the_points_of_a_shape_with_result_code_one() {
        let mut s = sweep();
        s.stepped();
        s.p.e.mem.set_u32(s.p.shape_b + 0xc, 3);
        s.p.code.set(1);
        put(&mut s.p.e, s.p.this + 0x70, [-2.0, 0.0, 0.0, 0.0]);
        s.run();
        assert_eq!(calls_to(&s.p.e, COUNT_DOUBLE), vec![vec![s.p.shape_a]]);
        assert_eq!(
            calls_to(&s.p.e, RECEIVE_DOUBLE),
            vec![vec![s.p.shape_a, 0x2000]]
        );
        let state = s.p.state.borrow().clone();
        assert_eq!(state.len(), 0x24);
        assert_eq!(state[0], s.p.body_a);
        assert_eq!(state[1], s.p.body_b);
        assert_eq!(state[2], 0x2000);
        assert_eq!(state[3..5], [0, 0]);
        assert_eq!(f32::from_bits(state[5]), 1.0);
        // 3 * 4 * 4 + 2 * 4 * 4.
        assert_eq!(f32::from_bits(state[6]), 80.0);
        let reciprocal = f32::from_bits(state[7]);
        assert!((reciprocal - 1.0 / 80.0).abs() < 1e-7, "{reciprocal}");
        assert_eq!(state[12..16], bits4([2.0, 4.0, 6.0, 8.0]));
        assert_eq!(state[16], s.p.time_record + 0x50);
        // 5 * sqrt(13) - 2: the inertia vectors (1, 2, 3, 4) crossed with (1, 0, 0) have the
        // length sqrt(13), the factors are 2 and 3 and the own velocity along the normal -2.
        let bound = f32::from_bits(state[17]);
        assert!(
            (bound - (5.0 * 13.0f32.sqrt() - 2.0)).abs() < 1e-3,
            "{bound}"
        );
        assert_eq!(state[20..24], bits4([-2.0, 0.0, 0.0, 0.0]));
        assert_eq!(state[24..28], bits4([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(state[28..32], bits4([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(f32::from_bits(state[32]), 0.75);
        assert_eq!(s.p.callbacks.borrow().len(), 1);
    }
}
