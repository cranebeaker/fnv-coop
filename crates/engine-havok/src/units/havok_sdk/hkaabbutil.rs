//! `Havok SDK/hkaabbutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::x87::rsqrt_estimate;

/// The four-lane `1.0` vector (`010ec7f0`).
const ONES: u32 = 0x010e_c7f0;
/// `0.0`, `3.0` and `0.5` (`float`): the `RSQRTSS` refinement constants.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// `double`s in the exe image: the small angle bound (`010c8150`, pi / 8), the middle bound
/// (`0103b100`, pi / 4), `0.5` (`01011588`), `2.0` (`01011590`) and `0.25` (`010290b0`).
const SMALL_ANGLE: u32 = 0x010c_8150;
const MIDDLE_ANGLE: u32 = 0x0103_b100;
const HALF_DOUBLE: u32 = 0x0101_1588;
const TWO_DOUBLE: u32 = 0x0101_1590;
const QUARTER_DOUBLE: u32 = 0x0102_90b0;
/// The `float` the code stores in the last lane of every step transform (`010c9240`).
const STEP_LAST_LANE: u32 = 0x010c_9240;
/// `00cb21e0(out, source)` (`RET 4`, ECX = out): builds the inverse of the transform at
/// `source` into `out`. `00cb2d90(matrix, quaternion)` (`RET 4`, ECX = matrix): builds a
/// rotation (three 16-byte rows) from a quaternion. Both are outside this unit.
const SET_INVERSE: u32 = 0x00cb_21e0;
const SET_ROTATION: u32 = 0x00cb_2d90;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 come from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: [f32; 4], src: [f32; 4], imm: u32) -> [f32; 4] {
    [
        dst[(imm & 3) as usize],
        dst[((imm >> 2) & 3) as usize],
        src[((imm >> 4) & 3) as usize],
        src[((imm >> 6) & 3) as usize],
    ]
}

/// `(ones - s) * a + b * s` on four lanes: the `SUBPS` / `MULPS` / `ADDPS` blend.
fn blend(ones: [f32; 4], a: [f32; 4], b: [f32; 4], s: [f32; 4]) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| (ones[i] - s[i]) * a[i] + b[i] * s[i])
}

/// Normalises the quaternion `q` with the `MULPS` / `SHUFPS 0x4E, 0xB1` / `ADDPS` sum of
/// squares and the refined `RSQRTSS` (zero for a zero sum).
fn normalise(q: [f32; 4], zero: f32, half: f32, three: f32) -> [f32; 4] {
    let squares = q.map(|lane| lane * lane);
    let swapped = shufps(squares, squares, 0x4e);
    let pairs = [0, 1, 2, 3].map(|i| swapped[i] + squares[i]);
    let crossed = shufps(pairs, pairs, 0xb1);
    let sum = crossed[0] + pairs[0];
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    let scale = if sum == zero { 0.0 } else { value };
    q.map(|lane| scale * lane)
}

/// Subtracts the rotated local offset (`input + 0x80`) from the translation row at
/// `base + 0x30`; the three rotation rows are at `base`, `base + 0x10` and `base + 0x20`.
fn recentre(e: &mut Engine, input: u32, base: u32) {
    let local = lanes(e, input + 0x80);
    let (row0, row1, row2) = (lanes(e, base), lanes(e, base + 0x10), lanes(e, base + 0x20));
    let translation = lanes(e, base + 0x30);
    let y = shufps(local, local, 0x55);
    let x = shufps(local, local, 0x00);
    let z = shufps(local, local, 0xaa);
    let moved = [0, 1, 2, 3].map(|i| (y[i] * row1[i] + x[i] * row0[i]) + z[i] * row2[i]);
    set_lanes(
        e,
        base + 0x30,
        [0, 1, 2, 3].map(|i| translation[i] - moved[i]),
    );
}

// Translated from 00cbec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbUtil::initOffsetAabbInput` (Xbox PDB), cdecl `(input, output)`. `input` is a swept
/// transform: centres of mass at +0x40 and +0x50 (their last lanes are the start time and
/// the inverse duration, +0x4c and +0x5c), quaternions at +0x60 and +0x70, the local offset
/// at +0x80 and an angle at +0x9c; `output` receives the input pointer at +0, up to step
/// transforms of 0x40 bytes from +0x10 (rotation rows then translation), the count at
/// +0x110, a first transform at +0x120 and the inverse of the input (`00cb21e0`) at +0x160.
/// With a zero inverse duration the count is 0. With an angle up to pi / 8 the single step
/// is a copy of the +0x120 transform; up to pi / 4 it is rebuilt from the quaternion at half
/// the interval; larger angles produce one step per `pi / 8 * 2` of angle (count in
/// +0x110). The last lane of the first two forms is the reciprocal `1 / (1 - angle^2 / 2)`
/// or `1 / (1 - angle^2 / 8)`, and of the later steps the constant at `010c9240`.
pub fn hk_aabb_util_init_offset_aabb_input(e: &mut Engine, input: Ptr, output: Ptr) {
    // Stack words (checked against the disassembly of `00cbec90`): the exe reads no stack word
    // it has not written first. The zero lanes in the 16-byte slots at +0x30, +0x40 and +0x50 are
    // written zeros (`XORPS` then `MOVSS` of a constant into a zeroed register), not garbage;
    // the slots at +0x10, +0x14, +0x18 and +0x1c are stored before each read.
    let p = input.addr();
    let o = output.addr();
    e.mem.set_u32(o, p);
    e.call(SET_INVERSE, &args![o + 0x160, p]);
    let ones = lanes(e, ONES);
    let (zero, half, three) = (e.mem.f32(ZERO), e.mem.f32(HALF), e.mem.f32(THREE));
    let start = e.mem.f32(p + 0x4c);
    let inverse_duration = e.mem.f32(p + 0x5c);
    let fraction = ((f64::from(start) - f64::from(start)) * f64::from(inverse_duration)) as f32;
    // The exe's frame: `x` is the stack pointer after the prologue pushes.
    e.with_stack(0x90, |e, frame| {
        let x = frame.addr();
        e.mem.set_f32(x + 0x14, fraction);
        let blend_at = |e: &Engine, s: [f32; 4]| {
            let a = lanes(e, p + 0x60);
            let b = lanes(e, p + 0x70);
            blend(ones, a, b, s)
        };
        let first = blend_at(e, [fraction; 4]);
        set_lanes(e, x + 0x60, [fraction; 4]);
        let unit = normalise(first, zero, half, three);
        set_lanes(e, x + 0x50, [half, 0.0, 0.0, 0.0]);
        set_lanes(e, x + 0x40, [three, 0.0, 0.0, 0.0]);
        set_lanes(e, x + 0x20, unit);
        set_lanes(e, x + 0x30, [0.0; 4]);
        let rows = o + 0x120;
        e.call(SET_ROTATION, &args![rows, x + 0x20]);
        let blend_centre = |e: &Engine, s: [f32; 4]| {
            let a = lanes(e, p + 0x40);
            let b = lanes(e, p + 0x50);
            blend(ones, a, b, s)
        };
        let centre = blend_centre(e, lanes(e, x + 0x60));
        set_lanes(e, rows + 0x30, centre);
        recentre(e, p, rows);
        if 0.0 == inverse_duration {
            e.mem.set_u32(o + 0x110, 0);
            return;
        }
        let step_time = (1.0 / f64::from(inverse_duration)) as f32;
        e.mem.set_f32(x + 0x14, step_time);
        let angle = x87_quiet(e.mem.f32(p + 0x9c));
        e.mem.set_f32(x + 0x10, angle);
        let small = e.mem.f64(SMALL_ANGLE);
        let angle_wide = f64::from(angle);
        let half_double = e.mem.f64(HALF_DOUBLE);
        if angle_wide <= small {
            for row in 0..4 {
                let value = lanes(e, rows + 0x10 * row);
                set_lanes(e, o + 0x10 + 0x10 * row, value);
            }
            let reduced = (1.0 - (angle_wide * angle_wide) * half_double) as f32;
            e.mem.set_f32(x + 0x14, reduced);
            let last = (1.0 / f64::from(reduced)) as f32;
            e.mem.set_f32(o + 0x4c, last);
            e.mem.set_u32(o + 0x110, 1);
            return;
        }
        let middle = e.mem.f64(MIDDLE_ANGLE);
        if angle_wide <= middle {
            let time = (f64::from(step_time) * half_double + f64::from(start)) as f32;
            e.mem.set_f32(x + 0x18, time);
            let shifted =
                ((f64::from(time) - f64::from(start)) * f64::from(inverse_duration)) as f32;
            e.mem.set_f32(x + 0x18, shifted);
            let second = blend_at(e, [shifted; 4]);
            set_lanes(e, x + 0x20, [shifted; 4]);
            let unit = normalise(second, zero, half, three);
            set_lanes(e, x + 0x60, unit);
            let step = o + 0x10;
            e.call(SET_ROTATION, &args![step, x + 0x60]);
            let squared = (angle_wide * angle_wide) * e.mem.f64(QUARTER_DOUBLE);
            let centre = blend_centre(e, lanes(e, x + 0x20));
            set_lanes(e, step + 0x30, centre);
            recentre(e, p, step);
            let reduced = (1.0 - squared * half_double) as f32;
            e.mem.set_f32(x + 0x14, reduced);
            let last = (1.0 / f64::from(reduced)) as f32;
            e.mem.set_f32(o + 0x4c, last);
            e.mem.set_u32(o + 0x110, 1);
            return;
        }
        e.mem.set_u32(o + 0x110, 0);
        let count_wide = (angle_wide + small) / small;
        let count = count_wide as f32;
        e.mem.set_f32(x + 0x10, count);
        let inverse_count = (1.0 / f64::from(count)) as f32;
        e.mem.set_f32(x + 0x18, inverse_count);
        e.mem.set_f32(x + 0x10, 1.0);
        let limit = f64::from(count);
        let mut again = limit > 1.0;
        while again {
            let current = e.mem.f32(x + 0x10);
            let start_copy = x87_quiet(e.mem.f32(p + 0x4c));
            e.mem.set_f32(x + 0x1c, start_copy);
            let index = e.mem.u32(o + 0x110);
            e.mem.set_u32(o + 0x110, index.wrapping_add(1));
            let scaled = (f64::from(current) * f64::from(inverse_count)) * f64::from(step_time);
            let moment = (scaled + f64::from(start_copy)) as f32;
            e.mem.set_f32(x + 0x1c, moment);
            let moment =
                ((f64::from(moment) - f64::from(start)) * f64::from(inverse_duration)) as f32;
            e.mem.set_f32(x + 0x1c, moment);
            let entry = index.wrapping_shl(6).wrapping_add(o).wrapping_add(0x10);
            let blended = blend_at(e, [moment; 4]);
            set_lanes(e, x + 0x70, [moment; 4]);
            let unit = normalise(blended, zero, half, three);
            set_lanes(e, x + 0x60, unit);
            e.call(SET_ROTATION, &args![entry, x + 0x60]);
            let centre = blend_centre(e, lanes(e, x + 0x70));
            set_lanes(e, entry + 0x30, centre);
            recentre(e, p, entry);
            let last = x87_quiet(e.mem.f32(STEP_LAST_LANE));
            e.mem.set_f32(entry + 0x3c, last);
            let next = (f64::from(current) + e.mem.f64(TWO_DOUBLE)) as f32;
            e.mem.set_f32(x + 0x10, next);
            again = f64::from(next) < limit;
        }
    });
}

/// The largest float (`0104da48`) and its negative (`0104e148`): the bounds an empty point set
/// is given (min = largest, max = most negative). The mask that clears the last lane
/// (`010c4c10`: three all-ones words, then zero).
const BOUNDS_START_MIN: u32 = 0x0104_da48;
const BOUNDS_START_MAX: u32 = 0x0104_e148;
const KEEP_THREE_LANES: u32 = 0x010c_4c10;

/// `MINPS dst, src` per lane: `dst` when it is less than `src`, else `src`.
fn min_lanes(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| if dst[i] < src[i] { dst[i] } else { src[i] })
}

/// `MAXPS dst, src` per lane: `dst` when it is greater than `src`, else `src`.
fn max_lanes(dst: [f32; 4], src: [f32; 4]) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| if dst[i] > src[i] { dst[i] } else { src[i] })
}

/// `ANDPS` with the three-lane mask: the last lane becomes zero, the others are kept bit for
/// bit.
fn keep_three_lanes(e: &Engine, value: [f32; 4]) -> [f32; 4] {
    [0, 1, 2, 3]
        .map(|i| f32::from_bits(value[i].to_bits() & e.mem.u32(KEEP_THREE_LANES + 4 * i as u32)))
}

/// Writes the bounds of an empty set: the largest float in every lane of the minimum and the
/// most negative in every lane of the maximum.
fn store_empty_bounds(e: &mut Engine, out: u32) {
    let low = e.mem.f32(BOUNDS_START_MIN);
    let high = e.mem.f32(BOUNDS_START_MAX);
    set_lanes(e, out, [low; 4]);
    set_lanes(e, out + 0x10, [high; 4]);
}

/// The point of three floats at `at`, each copied through the x87 (`FLD` / `FSTP`, which makes
/// a signalling NaN quiet); the last lane is zero here (the exe leaves it as whatever its stack
/// slot held, and every use clears it).
fn point_through_x87(e: &Engine, at: u32) -> [f32; 4] {
    [
        x87_quiet(e.mem.f32(at)),
        x87_quiet(e.mem.f32(at + 4)),
        x87_quiet(e.mem.f32(at + 8)),
        0.0,
    ]
}

// Translated from 00cbeaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbUtil::calcAabb` (Xbox PDB), cdecl `(points, count, stride, out)`: bounds of `count`
/// points of three floats, `stride` bytes apart. Starts from the first point (even for a count
/// below 1) and takes the lane-wise `MINPS` / `MAXPS` of the others in order; writes the
/// minimum to `out` and the maximum to `out + 0x10`, four floats each with the last lane zero.
pub fn hk_aabb_util_calc_aabb(e: &mut Engine, points: Ptr, count: i32, stride: i32, out: Ptr) {
    let first = point_through_x87(e, points.addr());
    let mut low = first;
    let mut high = first;
    if count > 1 {
        // The exe walks a pointer at the third float of the next point.
        let mut at = points.addr().wrapping_add(stride as u32).wrapping_add(8);
        let mut remaining = count - 1;
        loop {
            let next = point_through_x87(e, at.wrapping_sub(8));
            at = at.wrapping_add(stride as u32);
            remaining -= 1;
            low = min_lanes(low, next);
            high = max_lanes(high, next);
            if remaining == 0 {
                break;
            }
        }
    }
    let low = keep_three_lanes(e, low);
    let high = keep_three_lanes(e, high);
    set_lanes(e, out.addr(), low);
    set_lanes(e, out.addr() + 0x10, high);
}

// Translated from 00cbeb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbUtil::calcAabb_ov2` (Xbox PDB), cdecl `(vectors, count, out)`: bounds of `count`
/// 16-byte vectors. A count of 0 gives the empty bounds in all four lanes; any other count
/// starts from the first vector (so a negative count gives that one) and takes the lane-wise
/// `MINPS` / `MAXPS` of the rest; the last lane of both results is cleared.
pub fn hk_aabb_util_calc_aabb_ov2(e: &mut Engine, vectors: Ptr, count: i32, out: Ptr) {
    if count == 0 {
        store_empty_bounds(e, out.addr());
        return;
    }
    let first = lanes(e, vectors.addr());
    let mut low = first;
    let mut high = first;
    if count > 1 {
        let mut at = vectors.addr().wrapping_add(0x10);
        let mut remaining = count - 1;
        loop {
            let next = lanes(e, at);
            at = at.wrapping_add(0x10);
            remaining -= 1;
            low = min_lanes(low, next);
            high = max_lanes(high, next);
            if remaining == 0 {
                break;
            }
        }
    }
    let low = keep_three_lanes(e, low);
    let high = keep_three_lanes(e, high);
    set_lanes(e, out.addr(), low);
    set_lanes(e, out.addr() + 0x10, high);
}

// Translated from 00cbebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bounds of `count` vectors given as an array of pointers to 16-byte vectors (cdecl
/// `(pointers, count, out)`): a count of 0 gives the empty bounds in all four lanes; otherwise
/// `out` and `out + 0x10` start as the first vector and are updated in memory for every other
/// one (`MINPS` / `MAXPS` with the new vector first, so a NaN in the new vector leaves the old
/// bound), and at the end the last lane of both is set to 0.0.
pub fn fn_00cbebf0(e: &mut Engine, pointers: Ptr, count: i32, out: Ptr) {
    let out = out.addr();
    if count == 0 {
        store_empty_bounds(e, out);
        return;
    }
    let first = e.mem.u32(pointers.addr());
    let first_vector = lanes(e, first);
    set_lanes(e, out, first_vector);
    set_lanes(e, out + 0x10, first_vector);
    if count > 1 {
        let mut index = 1i32;
        loop {
            let pointer = e.mem.u32(pointers.addr().wrapping_add(4 * index as u32));
            let new_low = lanes(e, pointer);
            let current_low = lanes(e, out);
            let low = min_lanes(new_low, current_low);
            let current_high = lanes(e, out + 0x10);
            set_lanes(e, out, low);
            let pointer = e.mem.u32(pointers.addr().wrapping_add(4 * index as u32));
            let new_high = lanes(e, pointer);
            index += 1;
            let high = max_lanes(new_high, current_high);
            set_lanes(e, out + 0x10, high);
            if index >= count {
                break;
            }
        }
    }
    e.mem.set_f32(out + 0xc, 0.0);
    e.mem.set_f32(out + 0x1c, 0.0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_ec90, hk_aabb_util_init_offset_aabb_input(Ptr, Ptr)),
        entry!(0x00cb_eaf0, hk_aabb_util_calc_aabb(Ptr, i32, i32, Ptr)),
        entry!(0x00cb_eb70, hk_aabb_util_calc_aabb_ov2(Ptr, i32, Ptr)),
        entry!(0x00cb_ebf0, fn_00cbebf0(Ptr, i32, Ptr)),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, u32, [f32; 4])>>>;

    /// Doubles for the inverse (logs) and the rotation builder (logs the destination and the
    /// quaternion it was given, and writes identity rows), plus the exe constants.
    fn rig(angle: f32, inverse_duration: f32) -> (Engine, Log, u32, u32) {
        let mut e = test_engine();
        set_lanes(&mut e, ONES, [1.0; 4]);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f64(SMALL_ANGLE, 0.392_699_092_626_571_66);
        e.mem.set_f64(MIDDLE_ANGLE, 0.785_398_185_253_143_3);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f64(TWO_DOUBLE, 2.0);
        e.mem.set_f64(QUARTER_DOUBLE, 0.25);
        e.mem.set_f32(STEP_LAST_LANE, 1.0824);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(SET_INVERSE, move |_, a| {
            sink.borrow_mut().push((SET_INVERSE, a[0], [0.0; 4]));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SET_ROTATION, move |e, a| {
            sink.borrow_mut().push((SET_ROTATION, a[0], lanes(e, a[1])));
            for (row, v) in [
                [1.0f32, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ]
            .into_iter()
            .enumerate()
            {
                set_lanes(e, a[0] + 0x10 * row as u32, v);
            }
            Ret::default()
        });
        let p = e.mem.alloc(0xa0);
        set_lanes(&mut e, p + 0x40, [1.0, 2.0, 3.0, 0.0]);
        set_lanes(&mut e, p + 0x50, [3.0, 2.0, 1.0, inverse_duration]);
        set_lanes(&mut e, p + 0x60, [0.0, 0.0, 0.0, 1.0]);
        set_lanes(&mut e, p + 0x70, [0.0, 0.0, 0.0, 1.0]);
        set_lanes(&mut e, p + 0x80, [0.0; 4]);
        e.mem.set_f32(p + 0x9c, angle);
        let o = e.mem.alloc(0x200);
        (e, log, p, o)
    }

    #[test]
    fn a_zero_inverse_duration_leaves_no_steps() {
        let (mut e, log, p, o) = rig(1.0, 0.0);
        e.mem.set_u32(o + 0x110, 7);
        e.call(0x00cb_ec90, &args![p, o]);
        assert_eq!(e.mem.u32(o), p);
        assert_eq!(e.mem.u32(o + 0x110), 0);
        let calls = log.borrow();
        assert_eq!(calls[0].0, SET_INVERSE);
        assert_eq!(calls[0].1, o + 0x160);
        assert_eq!(calls[1].1, o + 0x120);
        assert_eq!(calls.len(), 2);
    }

    #[test]
    fn a_small_angle_copies_the_first_transform() {
        let (mut e, log, p, o) = rig(0.2, 2.0);
        e.call(0x00cb_ec90, &args![p, o]);
        assert_eq!(e.mem.u32(o + 0x110), 1);
        assert_eq!(lanes(&e, o + 0x10), [1.0, 0.0, 0.0, 0.0]);
        // The translation is the blend at fraction 0 (the first centre).
        let translation = lanes(&e, o + 0x40);
        assert_eq!(translation[..3], [1.0, 2.0, 3.0]);
        let reduced = (1.0 - f64::from(0.2f32) * f64::from(0.2f32) * 0.5) as f32;
        assert_eq!(e.mem.f32(o + 0x4c), (1.0 / f64::from(reduced)) as f32);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn a_middle_angle_rebuilds_the_step_at_half_the_interval() {
        let (mut e, log, p, o) = rig(0.5, 2.0);
        e.call(0x00cb_ec90, &args![p, o]);
        assert_eq!(e.mem.u32(o + 0x110), 1);
        let calls = log.borrow();
        assert_eq!(calls[2].1, o + 0x10);
        assert!((calls[2].2[3] - 1.0).abs() < 0.01);
        // Half of the interval (1 / 2) times 0.5 is a quarter, times the inverse duration 2: 0.5.
        let translation = lanes(&e, o + 0x40);
        assert!((translation[0] - 2.0).abs() < 1e-5);
        assert!((translation[2] - 2.0).abs() < 1e-5);
        let reduced = (1.0 - f64::from(0.5f32) * f64::from(0.5f32) * 0.25 * 0.5) as f32;
        assert_eq!(e.mem.f32(o + 0x4c), (1.0 / f64::from(reduced)) as f32);
    }

    #[test]
    fn a_large_angle_makes_a_step_per_two_small_angles() {
        let (mut e, log, p, o) = rig(1.2, 2.0);
        e.call(0x00cb_ec90, &args![p, o]);
        // (1.2 + pi/8) / (pi/8) is about 4.06: steps at 1 and 3.
        assert_eq!(e.mem.u32(o + 0x110), 2);
        let calls = log.borrow();
        assert_eq!(calls[2].1, o + 0x10);
        assert_eq!(calls[3].1, o + 0x50);
        assert_eq!(e.mem.f32(o + 0x10 + 0x3c), 1.0824);
        assert_eq!(e.mem.f32(o + 0x50 + 0x3c), 1.0824);
    }
}

#[cfg(test)]
mod calc_aabb_tests {
    use super::*;

    const HIGHEST: f32 = f32::MAX;

    fn setup() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(BOUNDS_START_MIN, HIGHEST);
        e.mem.set_f32(BOUNDS_START_MAX, -HIGHEST);
        for lane in 0..3u32 {
            e.mem.set_u32(KEEP_THREE_LANES + 4 * lane, 0xffff_ffff);
        }
        e.mem.set_u32(KEEP_THREE_LANES + 12, 0);
        e
    }

    /// Fills `count` records of `stride` bytes with the given points (three floats each) and a
    /// marker in the padding.
    fn points(e: &mut Engine, stride: u32, list: &[[f32; 3]]) -> u32 {
        let at = e.mem.alloc(stride * list.len() as u32 + 16);
        for (index, point) in list.iter().enumerate() {
            for (lane, value) in point.iter().enumerate() {
                e.mem
                    .set_f32(at + stride * index as u32 + 4 * lane as u32, *value);
            }
        }
        at
    }

    fn vectors(e: &mut Engine, list: &[[f32; 4]]) -> u32 {
        let at = e.mem.alloc(16 * list.len() as u32 + 16);
        for (index, vector) in list.iter().enumerate() {
            set_lanes(e, at + 16 * index as u32, *vector);
        }
        at
    }

    fn out_block(e: &mut Engine) -> u32 {
        let at = e.mem.alloc(0x20);
        for word in 0..8u32 {
            e.mem.set_u32(at + 4 * word, 0xaaaa_aaaa);
        }
        at
    }

    #[test]
    fn calc_aabb_takes_the_bounds_of_strided_points() {
        let mut e = setup();
        let at = points(
            &mut e,
            16,
            &[[1.0, 5.0, -2.0], [3.0, -1.0, 0.0], [-4.0, 2.0, 7.0]],
        );
        let out = out_block(&mut e);
        hk_aabb_util_calc_aabb(&mut e, Ptr::new(at), 3, 16, Ptr::new(out));
        assert_eq!(lanes(&e, out), [-4.0, -1.0, -2.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [3.0, 5.0, 7.0, 0.0]);
    }

    #[test]
    fn calc_aabb_starts_from_the_first_point_for_a_count_of_one_or_less() {
        let mut e = setup();
        let at = points(&mut e, 12, &[[1.0, 2.0, 3.0], [9.0, 9.0, 9.0]]);
        for count in [1, 0, -5] {
            let out = out_block(&mut e);
            hk_aabb_util_calc_aabb(&mut e, Ptr::new(at), count, 12, Ptr::new(out));
            assert_eq!(lanes(&e, out), [1.0, 2.0, 3.0, 0.0]);
            assert_eq!(lanes(&e, out + 0x10), [1.0, 2.0, 3.0, 0.0]);
        }
    }

    #[test]
    fn calc_aabb_quiets_a_signalling_nan_and_keeps_a_nan_from_later_points() {
        let mut e = setup();
        let at = points(&mut e, 12, &[[0.0, 1.0, 2.0], [f32::NAN, 0.0, 0.0]]);
        e.mem.set_u32(at, 0x7f80_0001);
        let out = out_block(&mut e);
        hk_aabb_util_calc_aabb(&mut e, Ptr::new(at), 1, 12, Ptr::new(out));
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
        // A NaN in a later point replaces the bound (the source operand wins when unordered).
        hk_aabb_util_calc_aabb(&mut e, Ptr::new(at), 2, 12, Ptr::new(out));
        assert!(e.mem.f32(out + 0x10).is_nan());
        assert!(e.mem.f32(out).is_nan());
    }

    #[test]
    fn calc_aabb_ov2_gives_the_empty_bounds_for_no_vectors() {
        let mut e = setup();
        let out = out_block(&mut e);
        hk_aabb_util_calc_aabb_ov2(&mut e, Ptr::new(0), 0, Ptr::new(out));
        assert_eq!(lanes(&e, out), [HIGHEST; 4]);
        assert_eq!(lanes(&e, out + 0x10), [-HIGHEST; 4]);
    }

    #[test]
    fn calc_aabb_ov2_takes_the_bounds_and_clears_the_last_lane() {
        let mut e = setup();
        let at = vectors(
            &mut e,
            &[
                [1.0, 5.0, -2.0, 8.0],
                [3.0, -1.0, 0.0, 9.0],
                [-4.0, 2.0, 7.0, 6.0],
            ],
        );
        let out = out_block(&mut e);
        hk_aabb_util_calc_aabb_ov2(&mut e, Ptr::new(at), 3, Ptr::new(out));
        assert_eq!(lanes(&e, out), [-4.0, -1.0, -2.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [3.0, 5.0, 7.0, 0.0]);
        // One vector, and a negative count: just the first vector.
        for count in [1, -3] {
            hk_aabb_util_calc_aabb_ov2(&mut e, Ptr::new(at), count, Ptr::new(out));
            assert_eq!(lanes(&e, out), [1.0, 5.0, -2.0, 0.0]);
            assert_eq!(lanes(&e, out + 0x10), [1.0, 5.0, -2.0, 0.0]);
        }
    }

    #[test]
    fn calc_aabb_ov2_lets_a_nan_replace_the_bound() {
        let mut e = setup();
        let at = vectors(&mut e, &[[0.0; 4], [f32::NAN, 0.0, 0.0, 0.0]]);
        let out = out_block(&mut e);
        hk_aabb_util_calc_aabb_ov2(&mut e, Ptr::new(at), 2, Ptr::new(out));
        assert!(e.mem.f32(out).is_nan());
        assert!(e.mem.f32(out + 0x10).is_nan());
    }

    fn pointer_array(e: &mut Engine, list: &[[f32; 4]]) -> u32 {
        let at = e.mem.alloc(4 * list.len() as u32 + 4);
        for (index, vector) in list.iter().enumerate() {
            let vector_at = vectors(e, &[*vector]);
            e.mem.set_u32(at + 4 * index as u32, vector_at);
        }
        at
    }

    #[test]
    fn bounds_of_pointed_to_vectors_for_no_vectors() {
        let mut e = setup();
        let out = out_block(&mut e);
        fn_00cbebf0(&mut e, Ptr::new(0), 0, Ptr::new(out));
        assert_eq!(lanes(&e, out), [HIGHEST; 4]);
        assert_eq!(lanes(&e, out + 0x10), [-HIGHEST; 4]);
    }

    #[test]
    fn bounds_of_pointed_to_vectors_updates_the_output_and_zeroes_the_last_lane() {
        let mut e = setup();
        let at = pointer_array(
            &mut e,
            &[
                [1.0, 5.0, -2.0, 8.0],
                [3.0, -1.0, 0.0, 9.0],
                [-4.0, 2.0, 7.0, 6.0],
            ],
        );
        let out = out_block(&mut e);
        fn_00cbebf0(&mut e, Ptr::new(at), 3, Ptr::new(out));
        assert_eq!(lanes(&e, out), [-4.0, -1.0, -2.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [3.0, 5.0, 7.0, 0.0]);
        // A single vector: copied, last lane cleared.
        fn_00cbebf0(&mut e, Ptr::new(at), 1, Ptr::new(out));
        assert_eq!(lanes(&e, out), [1.0, 5.0, -2.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [1.0, 5.0, -2.0, 0.0]);
    }

    #[test]
    fn bounds_of_pointed_to_vectors_ignores_a_nan_in_a_later_vector() {
        let mut e = setup();
        let at = pointer_array(&mut e, &[[2.0; 4], [f32::NAN, 1.0, 3.0, 0.0]]);
        let out = out_block(&mut e);
        fn_00cbebf0(&mut e, Ptr::new(at), 2, Ptr::new(out));
        // The NaN is the first operand of MINPS / MAXPS, so the old bound (the source) stays.
        assert_eq!(lanes(&e, out), [2.0, 1.0, 2.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [2.0, 2.0, 3.0, 0.0]);
    }
}
