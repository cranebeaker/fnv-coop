//! `NiMain/nibound_xenon.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiBound` (Xbox PDB, 0x10 bytes): `m_kCenter` (`NiPoint3`, `+0x00`) and `m_fRadius`
//! (`+0x0C`). The x87 steps go through the shared helpers one instruction at a time.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00ec6040` (`_CIsqrt`: the value in `ST0`, the root in `ST0`; a leading `f64` word pair).
const SQUARE_ROOT: u32 = 0x00ec_6040;
/// The three floats `ComputeFromData` copies as the centre of an empty bound (`011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;
/// The `double` 0.5 at `01011588` (read as the game reads it).
const HALF: u32 = 0x0101_1588;
/// The float `Merge` compares the distance of the two centres with (`011ab3c0`; which
/// `NiBound` static it is was not confirmed).
const MERGE_THRESHOLD: u32 = 0x011a_b3c0;

/// `FLD dword` of the float at `at`.
fn load(e: &Engine, at: u32) -> f64 {
    x87_load32(e.mem.u32(at))
}

/// A float memory operand (`FADD`/`FSUB`/`FMUL dword ptr`) at `at`.
fn operand(e: &Engine, at: u32) -> f64 {
    x87_m32(e.mem.u32(at))
}

/// `FSTP dword` then `FLD dword` of the same slot: the value rounded to a float, quieted.
fn through_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `FLD a; FLD b; FLD c` then the shared `FLD ST1; FMULP ST2; FLD ST2; FMULP ST3; FXCH;
/// FADDP ST2,ST0; FMUL ST0; FADDP` sequence: `(a*a + b*b) + c*c` with the x87 choice of
/// NaN at each step (`a`, `b`, `c` are the slots in load order).
fn sum_of_squares(a: f64, b: f64, c: f64) -> f64 {
    let bb = x87_mul(b, b);
    let aa = x87_mul(a, a);
    let first_two = x87_add(aa, bb);
    x87_add(first_two, x87_mul(c, c))
}

// Translated from 00a7ee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBound::ComputeFromData` (Xbox PDB; `thiscall`, `RET 8`: the point count and the
/// address of `count` packed `NiPoint3`s). With no points (`count <= 0`) the centre is the
/// point at `011f426c` (copied as words) and the radius 0. Otherwise the centre is the
/// middle of the points' box: the minimum and maximum of each axis (from the first point;
/// a later coordinate replaces the minimum when it is smaller, the maximum when it is
/// larger, both ordered compares), each axis `(max + min)` stored as a float and halved by
/// the `double` 0.5; the radius is the square root (`00ec6040`) of the largest squared
/// distance (each difference and the sum stored as floats; compared with the best so far,
/// which starts at 0) from the centre to a point. The exe walks both loops four points at
/// a time and then the rest; each point goes through the same steps in the same order.
pub fn ni_bound_compute_from_data(e: &mut Engine, this: Ptr, count: i32, points: Ptr) {
    let bound = this.addr();
    if count <= 0 {
        for axis in 0..3 {
            let word = e.mem.u32(ZERO_POINT + 4 * axis);
            e.mem.set_u32(bound + 4 * axis, word);
        }
        e.mem.set_u32(bound + 0xc, x87_store32(0.0));
        return;
    }
    let base = points.addr();
    let first: [u32; 3] = std::array::from_fn(|axis| e.mem.u32(base + 4 * axis as u32));
    let mut low = first;
    let mut high = first;
    for index in 1..count as u32 {
        let point = base.wrapping_add(12 * index);
        for axis in 0..3 {
            let value = e.mem.u32(point + 4 * axis as u32);
            // FLD min; FLD p; FCOMPP; TEST AH,5; JP: replaced when p < min.
            if x87_load32(value) < x87_load32(low[axis]) {
                low[axis] = x87_quiet_bits(value);
            }
        }
        for axis in 0..3 {
            let value = e.mem.u32(point + 4 * axis as u32);
            // FLD max; FLD p; FCOMPP; TEST AH,0x41; JNZ: replaced when p > max.
            if x87_load32(value) > x87_load32(high[axis]) {
                high[axis] = x87_quiet_bits(value);
            }
        }
    }
    let sums: [u32; 3] = std::array::from_fn(|axis| {
        x87_store32(x87_add(x87_load32(high[axis]), x87_m32(low[axis])))
    });
    let half = x87_load64(e.mem.u64(HALF));
    // FLD sx; FLD half; FMUL ST1,ST0; FXCH; FSTP: x = sx * half.
    let x = x87_store32(x87_mul(x87_load32(sums[0]), half));
    e.mem.set_u32(bound, x);
    // FLD sy; FMUL ST0,ST1; FSTP: y = sy * half.
    let y = x87_store32(x87_mul(x87_load32(sums[1]), half));
    e.mem.set_u32(bound + 4, y);
    // FMUL dword [sz]; FSTP: z = half * sz.
    let z = x87_store32(x87_mul(half, x87_m32(sums[2])));
    e.mem.set_u32(bound + 8, z);
    let mut best = x87_store32(0.0);
    for index in 0..count as u32 {
        let point = base.wrapping_add(12 * index);
        let difference: [u32; 3] = std::array::from_fn(|axis| {
            let at = 4 * axis as u32;
            x87_store32(x87_sub(load(e, point + at), operand(e, bound + at)))
        });
        // FLD dy; FLD dx; FLD dz: (dy*dy + dx*dx) + dz*dz.
        let squared = x87_store32(sum_of_squares(
            x87_load32(difference[0]),
            x87_load32(difference[1]),
            x87_load32(difference[2]),
        ));
        // FLD d2; FLD best; FCOMP; TEST AH,5; JP: kept when best < d2.
        if x87_load32(best) < x87_load32(squared) {
            best = x87_store32(x87_load32(squared));
        }
    }
    let root = e.call(SQUARE_ROOT, &args![x87_load32(best)]).f64();
    let radius = x87_store32(through_float(root));
    e.mem.set_u32(bound + 0xc, radius);
}

// Translated from 00a7f3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBound::Merge` (Xbox PDB; `thiscall`, `RET 4`): makes `this` the bound of itself and
/// `other`. With `d = this.center - other.center` (each axis stored as a float), `dist2`
/// its squared length and `dr = other.radius - this.radius`, `dr2 = dr * dr` (floats):
/// when `dr >= 0` (ordered) and `dist2 <= dr2` (ordered) `other` encloses `this` and is
/// copied (centre as words, radius through the x87 stack); when `dr < 0` or NaN and
/// `dist2 <= dr2` or unordered, `this` encloses `other` and nothing changes. Otherwise
/// `dist` is the square root of `dist2` (`00ec6040`, stored as a float); when `dist` is
/// larger than the float at `011ab3c0` the centre moves to `other.center + t * d` with
/// `t = (dist - dr) / (dist + dist)`; the radius becomes
/// `(dist + other.radius + this.radius) * 0.5` (the `double` at `01011588`).
pub fn ni_bound_merge(e: &mut Engine, this: Ptr, other: Ptr) {
    let (bound, with) = (this.addr(), other.addr());
    let d: [u32; 3] = std::array::from_fn(|axis| {
        let at = 4 * axis as u32;
        x87_store32(x87_sub(load(e, bound + at), operand(e, with + at)))
    });
    // FLD dx; FLD dy; FLD dz: (dx*dx + dy*dy) + dz*dz, the same sequence.
    let dist2 = x87_store32(sum_of_squares(
        x87_load32(d[1]),
        x87_load32(d[0]),
        x87_load32(d[2]),
    ));
    let dr = x87_store32(x87_sub(load(e, with + 0xc), operand(e, bound + 0xc)));
    let dr_value = x87_load32(dr);
    let dr2 = x87_store32(x87_mul(dr_value, dr_value));
    let (dist2_value, dr2_value) = (x87_load32(dist2), x87_load32(dr2));
    // FLDZ; FCOMPP (0 against dr); TEST AH,0x41; JP: taken when dr < 0 or unordered.
    let radius_grows = 0.0 <= dr_value;
    if radius_grows {
        // TEST AH,0x41 on FCOM (dist2 against dr2); JP: taken when dist2 > dr2 or unordered.
        if dist2_value <= dr2_value {
            for axis in 0..3 {
                let word = e.mem.u32(with + 4 * axis);
                e.mem.set_u32(bound + 4 * axis, word);
            }
            let radius = x87_store32(load(e, with + 0xc));
            e.mem.set_u32(bound + 0xc, radius);
            return;
        }
    } else if !(dist2_value > dr2_value) {
        // TEST AH,0x41; JNZ: dist2 < dr2, equal or unordered: nothing to do.
        return;
    }
    let root = e.call(SQUARE_ROOT, &args![dist2_value]).f64();
    let dist = through_float(through_float(root));
    let threshold = load(e, MERGE_THRESHOLD);
    // FLD threshold; FCOMP (against dist); TEST AH,5; JP: the centre moves when
    // threshold < dist.
    if threshold < dist {
        let t = x87_store32(x87_div(x87_sub(dist, x87_m32(dr)), x87_add(dist, dist)));
        let t_value = x87_load32(t);
        let sx = x87_store32(x87_mul(x87_load32(d[0]), t_value));
        let sy = x87_store32(x87_mul(x87_load32(d[1]), t_value));
        let sz = x87_store32(x87_mul(t_value, x87_m32(d[2])));
        let x = x87_store32(x87_add(load(e, with), x87_m32(sx)));
        let y = x87_store32(x87_add(x87_load32(sy), operand(e, with + 4)));
        let z = x87_store32(x87_add(load(e, with + 8), x87_m32(sz)));
        e.mem.set_u32(bound, x);
        e.mem.set_u32(bound + 4, y);
        e.mem.set_u32(bound + 8, z);
    }
    let sum = x87_add(
        x87_add(dist, operand(e, with + 0xc)),
        operand(e, bound + 0xc),
    );
    let half = f64::from_bits(e.mem.u64(HALF));
    let radius = x87_store32(x87_mul(sum, half));
    e.mem.set_u32(bound + 0xc, radius);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_ee30, ni_bound_compute_from_data(Ptr, i32, Ptr)),
        entry!(0x00a7_f3f0, ni_bound_merge(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x011a_b000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f32(MERGE_THRESHOLD, 1.0e-4);
        e.register(SQUARE_ROOT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
        at
    }

    fn bound(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|index| e.mem.f32(at + 4 * index as u32))
    }

    #[test]
    fn no_points_give_the_zero_point_and_radius_zero() {
        let mut e = engine();
        e.mem.set_f32(ZERO_POINT, 1.5);
        e.mem.set_f32(ZERO_POINT + 4, 2.5);
        e.mem.set_f32(ZERO_POINT + 8, 3.5);
        let this = floats(&mut e, &[9.0; 4]);
        e.call(0x00a7_ee30, &args![this, 0i32, 0u32]);
        assert_eq!(bound(&e, this), [1.5, 2.5, 3.5, 0.0]);
    }

    #[test]
    fn the_centre_is_the_middle_of_the_box_and_the_radius_the_farthest_point() {
        let mut e = engine();
        // Six points: four through the unrolled loop, two through the rest.
        let points = floats(
            &mut e,
            &[
                1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, 3.0, 0.0,
                0.0, -1.0,
            ],
        );
        let this = floats(&mut e, &[0.0; 4]);
        e.call(0x00a7_ee30, &args![this, 6i32, points]);
        assert_eq!(bound(&e, this), [0.0, 0.0, 1.0, 5.0f64.sqrt() as f32]);
        // One point: centre on it, radius 0.
        let one = floats(&mut e, &[4.0, -4.0, 2.0]);
        e.call(0x00a7_ee30, &args![this, 1i32, one]);
        assert_eq!(bound(&e, this), [4.0, -4.0, 2.0, 0.0]);
    }

    #[test]
    fn merging_an_enclosing_bound_copies_it() {
        let mut e = engine();
        let this = floats(&mut e, &[1.0, 0.0, 0.0, 1.0]);
        let other = floats(&mut e, &[0.0, 0.0, 0.0, 3.0]);
        e.call(0x00a7_f3f0, &args![this, other]);
        assert_eq!(bound(&e, this), [0.0, 0.0, 0.0, 3.0]);
    }

    #[test]
    fn merging_an_enclosed_bound_changes_nothing() {
        let mut e = engine();
        let this = floats(&mut e, &[0.0, 0.0, 0.0, 3.0]);
        let other = floats(&mut e, &[1.0, 0.0, 0.0, 1.0]);
        e.call(0x00a7_f3f0, &args![this, other]);
        assert_eq!(bound(&e, this), [0.0, 0.0, 0.0, 3.0]);
    }

    #[test]
    fn merging_two_apart_moves_the_centre_and_grows_the_radius() {
        let mut e = engine();
        // dist 4, dr 0: t = 0.5, centre midway, radius (4 + 1 + 1) / 2.
        let this = floats(&mut e, &[4.0, 0.0, 0.0, 1.0]);
        let other = floats(&mut e, &[0.0, 0.0, 0.0, 1.0]);
        e.call(0x00a7_f3f0, &args![this, other]);
        assert_eq!(bound(&e, this), [2.0, 0.0, 0.0, 3.0]);
        // dr < 0 and dist2 > dr2: also merged (dist 4, dr -1: t = 5/8).
        let this = floats(&mut e, &[4.0, 0.0, 0.0, 2.0]);
        let other = floats(&mut e, &[0.0, 0.0, 0.0, 1.0]);
        e.call(0x00a7_f3f0, &args![this, other]);
        assert_eq!(bound(&e, this), [2.5, 0.0, 0.0, 3.5]);
    }

    #[test]
    fn coincident_centres_below_the_threshold_keep_the_centre() {
        let mut e = engine();
        e.mem.set_f32(MERGE_THRESHOLD, 10.0);
        // dist 4 is not above 10: only the radius changes.
        let this = floats(&mut e, &[4.0, 0.0, 0.0, 1.0]);
        let other = floats(&mut e, &[0.0, 0.0, 0.0, 1.0]);
        e.call(0x00a7_f3f0, &args![this, other]);
        assert_eq!(bound(&e, this), [4.0, 0.0, 0.0, 3.0]);
    }
}
