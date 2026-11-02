//! `Havok SDK/hkaposematchingutility.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaPoseMatchingUtility` (Xbox PDB name) is read off the code: `+0` an `hkArray` of candidate
//! entries of 0x20 bytes (data, size `+4`, capacity word `+8`; the first 16 bytes of an entry are
//! an orientation quaternion), `+0x10` the reference quaternion the twist is removed around, and
//! `+0x20`, `+0x22`, `+0x24` three 16-bit bone indices that pick three of the 0x30-byte transforms
//! (translation first) of a pose. The orientation of a pose is the quaternion of the frame spanned
//! by those three points. Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkakeyframehierarchyutility::{add4, mul4, rotate_half, splat, sub4};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, shufps, store_bits, store_vec, sum3, Vec4,
};

/// The memory router (`00c85750`): its field `+0x10` is the allocator `_reserveMore` (`00c90940`,
/// cdecl `(allocator, array, element size)`) takes.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkQuaternion::set` from a rotation matrix (`00cb26e0`, `this` = the result, `(matrix)`),
/// `hkQuaternion::setMul` (`00c66320`, `this` = the result, `(a, b)`), the quaternion product
/// `00c74ac0` (`this` = the result, `(a, b)`), `hkQuaternion::decomposeRestAxis` (`00cb2ca0`,
/// `this` = the quaternion, `(axis, twist, angle)`) and `hkQuaternion::setAxisAngle` (`00cb2450`,
/// `this` = the result, `(axis, angle)`).
const QUATERNION_FROM_MATRIX: u32 = 0x00cb_26e0;
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_PRODUCT: u32 = 0x00c7_4ac0;
const DECOMPOSE_REST_AXIS: u32 = 0x00cb_2ca0;
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// This unit's own routines, which the translations call the way the code does:
/// `computeOrientation` (`ECX` = the utility, `(base, out)`) and `hkRemoveTwist` (`EAX` = the
/// reference quaternion, `ESI` = the quaternion to change).
const COMPUTE_ORIENTATION: u32 = 0x00ce_0d30;
const REMOVE_TWIST: u32 = 0x00ce_0ff0;
/// The absolute-value mask (`01017d10`), the sign bits of the three lowest lanes (`010c4c00`), the
/// identity quaternion vector (`010c71b0`), the float that fills the scale vectors (`01012068`)
/// and the float the best distance starts from (`0104da48`).
const ABS_MASK: u32 = 0x0101_7d10;
const SIGN_BITS_XYZ: u32 = 0x010c_4c00;
const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
const SCALE_FILL: u32 = 0x0101_2068;
const WORST_DISTANCE: u32 = 0x0104_da48;

/// `SHUFPS v, v, 0xc9` and `0xd2`: the two lane rotations of a cross product.
fn c9(v: Vec4) -> Vec4 {
    shufps(v, v, 0xc9)
}

fn d2(v: Vec4) -> Vec4 {
    shufps(v, v, 0xd2)
}

/// `c9(x) * d2(a) - d2(x) * c9(a)`, the cross product the code spells out with `SHUFPS`.
fn cross(x: Vec4, a: Vec4) -> Vec4 {
    sub4(mul4(c9(x), d2(a)), mul4(d2(x), c9(a)))
}

/// `d2(x) * c9(a) - c9(x) * d2(a)`: the same with the operands of the subtraction swapped.
fn cross_swapped(x: Vec4, a: Vec4) -> Vec4 {
    sub4(mul4(d2(x), c9(a)), mul4(c9(x), d2(a)))
}

/// `v` times the inverse of the length of its first three lanes (`inverse_length`, `0` for a zero
/// length), in all four lanes.
fn normalised(e: &Engine, v: Vec4) -> Vec4 {
    mul4(splat(inverse_length(e, sum3(mul4(v, v)))), v)
}

/// The transform at the 16-bit index stored at `utility + offset`, in the array at `base`: its
/// first vector.
fn point(e: &Engine, utility: u32, base: u32, offset: u32) -> Vec4 {
    let index = i32::from(e.mem.i16(utility + offset));
    load_vec(e, base.wrapping_add((index as u32).wrapping_mul(0x30)))
}

// Translated from 00ce0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPoseMatchingUtility::computeOrientation` (Xbox PDB), `ECX` = the utility, `(base, out)`:
/// the orientation of the pose at `base` as a quaternion at `out`. From the three points picked
/// by the utility's indices (`+0x20` the origin, `+0x22` and `+0x24`): `a` is the normalised
/// vector from the origin to the second point, `b` the same to the third, `c` the normalised
/// cross product of `b` and `a`, `e` that of `c` and `a`, and `f` a last cross product (`e` and
/// `a`, operands of the subtraction swapped). Each normalisation is `inverse_length` of the first
/// three lanes times the vector (all four lanes). The rows `e`, `f`, `a` are the matrix that
/// `hkQuaternion::set` (`00cb26e0`) turns into the quaternion.
pub fn hka_pose_matching_utility_compute_orientation(
    e: &mut Engine,
    this: Ptr,
    base: Ptr,
    out: Ptr,
) {
    let utility = this.addr();
    let base = base.addr();
    let origin = point(e, utility, base, 0x20);
    let axis_a = normalised(e, sub4(point(e, utility, base, 0x22), origin));
    let axis_b = normalised(e, sub4(point(e, utility, base, 0x24), origin));
    let axis_c = normalised(e, cross(axis_b, axis_a));
    let axis_e = normalised(e, cross(axis_c, axis_a));
    let axis_f = cross_swapped(axis_e, axis_a);
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        store_vec(e, f + 0x10, axis_e);
        store_vec(e, f + 0x20, axis_f);
        store_vec(e, f + 0x30, axis_a);
        e.call(QUATERNION_FROM_MATRIX, &args![f, f + 0x10]);
        copy_vec(e, f, out.addr());
    });
}

/// The `x87` flag test `JNZ` after `FCOM` and `TEST AH, 0x41`: `a <= b`, or unordered.
fn not_greater(a: f32, b: f32) -> bool {
    a <= b || a.is_nan() || b.is_nan()
}

fn xor_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

// Translated from 00ce0ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRemoveTwist` (Xbox PDB), `EAX` = the reference quaternion, `ESI` = the quaternion to
/// change: removes the twist of `ESI` around the reference. Picks the axis of the reference
/// quaternion's largest absolute lane among its first three (ties and unordered compares go to the
/// lower lane; with `x87` flag tests: lane 1 wins only when it is greater than lane 0, then lane 2
/// only when it is greater than the winner), takes the unit vector of the next axis (`(i + 1) %
/// 3`), turns it by `ESI`, crosses the result with the reference, normalises that, and builds the
/// quaternion (`hkQuaternion::set`, `00cb26e0`) of the matrix whose rows are the cross product of
/// that with the reference, the reference and the normalised cross product. `ESI` becomes the
/// product (`setMul`, `00c66320`) of the conjugate of that quaternion (the three lowest sign bits
/// flipped) and the old `ESI`.
pub fn hk_remove_twist(e: &mut Engine, reference: Ptr, quaternion: Ptr) {
    let reference_at = reference.addr();
    let quaternion_at = quaternion.addr();
    let r = load_vec(e, reference_at);
    let mask = load_bits(e, ABS_MASK);
    let abs: Vec4 = std::array::from_fn(|lane| f32::from_bits(r[lane].to_bits() & mask[lane]));
    let largest: usize = if not_greater(abs[1], abs[0]) {
        if not_greater(abs[2], abs[0]) {
            0
        } else {
            2
        }
    } else if not_greater(abs[2], abs[1]) {
        1
    } else {
        2
    };
    let mut axis = [0.0f32; 4];
    axis[(largest + 1) % 3] = 1.0;
    let q = load_vec(e, quaternion_at);
    let half = rotate_half(e, q, axis);
    let turned = add4(half, half);
    let third = normalised(e, cross(turned, r));
    let matrix_third = cross_swapped(third, r);
    e.with_stack(0x60, |e, frame| {
        let f = frame.addr();
        store_vec(e, f + 0x30, matrix_third);
        store_vec(e, f + 0x40, r);
        store_vec(e, f + 0x50, third);
        e.call(QUATERNION_FROM_MATRIX, &args![f + 0x10, f + 0x30]);
        let twist = xor_bits(load_vec(e, f + 0x10), load_bits(e, SIGN_BITS_XYZ));
        store_vec(e, f, twist);
        e.call(QUATERNION_SET_MUL, &args![f + 0x20, f, quaternion_at]);
        copy_vec(e, f + 0x20, quaternion_at);
    });
}

// Translated from 00ce11f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPoseMatchingUtility::findBestCandidatePoseIndex` (Xbox PDB), `ECX` = the utility,
/// `(base, best)`: the index of the candidate entry whose quaternion is closest to the pose at
/// `base` (`-1` for none) and, at `best`, how close: `1 - |dot|` of the four lanes. The pose's
/// orientation (`computeOrientation`) has its twist removed (`hkRemoveTwist` around the utility's
/// reference quaternion) first. `best` starts as the float at `0104da48`; a candidate wins when
/// its distance is below the best so far (ordered, strict, so a tie keeps the first and a
/// distance that is not a number never wins).
pub fn hka_pose_matching_utility_find_best_candidate_pose_index(
    e: &mut Engine,
    this: Ptr,
    base: Ptr,
    best: Ptr,
) -> i32 {
    let utility = this.addr();
    let best = best.addr();
    let orientation = e.with_stack(0x10, |e, q| {
        e.call(COMPUTE_ORIENTATION, &args![utility, base, q]);
        e.call(REMOVE_TWIST, &args![utility + 0x10, q]);
        load_vec(e, q.addr())
    });
    let start = x87_quiet_bits(e.mem.u32(WORST_DISTANCE));
    e.mem.set_u32(best, start);
    let mut found = -1;
    let mut index = 0i32;
    if (e.mem.u32(utility + 4) as i32) > 0 {
        loop {
            let candidate = load_vec(e, e.mem.u32(utility).wrapping_add(0x20 * index as u32));
            let m = mul4(candidate, orientation);
            let dot = (m[3] + m[1]) + (m[2] + m[0]);
            let distance = (1.0 - f64::from(dot.abs())) as f32;
            if x87_quiet(e.mem.f32(best)) > distance {
                e.mem.set_f32(best, distance);
                found = index;
            }
            index += 1;
            if index >= e.mem.u32(utility + 4) as i32 {
                break;
            }
        }
    }
    found
}

// Translated from 00ce12a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPoseMatchingUtility::computeReferenceFrame` (Xbox PDB), `ECX` = the utility,
/// `(first, second, out1, out2)`: the transform that takes the pose at `first` to the pose at
/// `second`, written to `out2` (translation, rotation, scale) and copied to `out1`. The rotation
/// comes from the two orientations (`computeOrientation`) through the quaternion product
/// `00c74ac0`; the translation is the `second` pose's origin translation minus the `first` pose's,
/// turned by the rotation (twice `rotate_half`); the scale is the float at `01012068` in all
/// lanes. Then `decomposeRestAxis` (`00cb2ca0`) of `out1`'s rotation around the utility's
/// reference quaternion gives an angle that `setAxisAngle` (`00cb2450`) turns back into `out1`'s
/// rotation. The vectors the code first writes to `out1` and `out2` (zero translation, the identity
/// quaternion `010c71b0`, the scale) are written here too, in the same order.
pub fn hka_pose_matching_utility_compute_reference_frame(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    out1: Ptr,
    out2: Ptr,
) {
    let utility = this.addr();
    let (first, second) = (first.addr(), second.addr());
    let (out1, out2) = (out1.addr(), out2.addr());
    e.with_stack(0x50, |e, frame| {
        let f = frame.addr();
        let (orientation1, orientation2) = (f, f + 0x10);
        let (zero, fill, angle) = (f + 0x20, f + 0x30, f + 0x40);
        e.call(COMPUTE_ORIENTATION, &args![utility, first, orientation1]);
        e.call(COMPUTE_ORIENTATION, &args![utility, second, orientation2]);
        store_vec(e, out2, [0.0; 4]);
        store_vec(e, zero, [0.0; 4]);
        copy_vec(e, IDENTITY_QUATERNION, out2 + 0x10);
        let filler = e.mem.u32(SCALE_FILL);
        store_bits(e, out2 + 0x20, [filler; 4]);
        store_bits(e, fill, [filler; 4]);
        let offset = (i32::from(e.mem.i16(utility + 0x20)) as u32).wrapping_mul(0x30);
        let second_point = load_vec(e, second.wrapping_add(offset));
        store_vec(e, out2, second_point);
        e.call(
            QUATERNION_PRODUCT,
            &args![out2 + 0x10, orientation2, orientation1],
        );
        let rotation = load_vec(e, out2 + 0x10);
        let first_point = load_vec(e, first.wrapping_add(offset));
        let half = rotate_half(e, rotation, first_point);
        let moved = sub4(load_vec(e, out2), add4(half, half));
        store_vec(e, out2, moved);
        copy_vec(e, zero, out1);
        copy_vec(e, IDENTITY_QUATERNION, out1 + 0x10);
        copy_vec(e, fill, out1 + 0x20);
        for part in [0, 0x10, 0x20] {
            copy_vec(e, out2 + part, out1 + part);
        }
        e.call(
            DECOMPOSE_REST_AXIS,
            &args![out1 + 0x10, utility + 0x10, fill, angle],
        );
        let turn = x87_quiet(e.mem.f32(angle));
        e.call(SET_AXIS_ANGLE, &args![out1 + 0x10, utility + 0x10, turn]);
    });
}

// Translated from 00ce1420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPoseMatchingUtility::addCandidatePose` (Xbox PDB), `ECX` = the utility,
/// `(base, extra_word, extra_float)`: appends a candidate entry for the pose at `base` and answers
/// its index. The entry is the pose's orientation with its twist removed (`computeOrientation`,
/// `hkRemoveTwist`), then `extra_word` and `extra_float` (copied as `FLD` / `FSTP` copies it), then
/// two words that are whatever the stack held (zero here). When the array is full its storage is
/// grown by `_reserveMore`.
pub fn hka_pose_matching_utility_add_candidate_pose(
    e: &mut Engine,
    this: Ptr,
    base: Ptr,
    extra_word: u32,
    extra_float: f32,
) -> i32 {
    let list = this.addr();
    let extra_float = x87_quiet(extra_float);
    e.with_stack(0x20, |e, entry| {
        let q = entry.addr();
        e.mem.set_f32(q + 0x14, extra_float);
        e.mem.set_u32(q + 0x10, extra_word);
        e.call(COMPUTE_ORIENTATION, &args![list, base, q]);
        e.call(REMOVE_TWIST, &args![list + 0x10, q]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
        if e.mem.u32(list + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, list, 0x20u32]);
        }
        let slot = e
            .mem
            .u32(list + 4)
            .wrapping_mul(0x20)
            .wrapping_add(e.mem.u32(list));
        if slot != 0 {
            for k in 0..8 {
                let word = e.mem.u32(q + 4 * k);
                e.mem.set_u32(slot + 4 * k, word);
            }
        }
    });
    let size = e.mem.u32(list + 4).wrapping_add(1);
    e.mem.set_u32(list + 4, size);
    size.wrapping_sub(1) as i32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ce_0d30,
            hka_pose_matching_utility_compute_orientation(Ptr, Ptr, Ptr)
        ),
        entry!(0x00ce_0ff0, hk_remove_twist(Ptr, Ptr)),
        entry!(0x00ce_11f0, hka_pose_matching_utility_find_best_candidate_pose_index(Ptr, Ptr, Ptr) -> i32),
        entry!(
            0x00ce_12a0,
            hka_pose_matching_utility_compute_reference_frame(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00ce_1420, hka_pose_matching_utility_add_candidate_pose(Ptr, Ptr, u32, f32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOCATOR: u32 = 0x00a1_10c0;
    const NEG_HALF_LANES: u32 = 0x010c_71e0;

    fn entry(address: u32, words: &[u32]) -> (u32, Vec<u32>) {
        (address, words.to_vec())
    }

    fn bits(v: Vec4) -> Vec<u32> {
        v.iter().map(|lane| lane.to_bits()).collect()
    }

    fn near(a: Vec4, b: Vec4) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 2.0e-6)
    }

    /// An engine with the constants in memory (0, 3 and 0.5 for the normaliser, -0.5, the
    /// absolute-value mask, the sign bits of the three lowest lanes, the identity quaternion, the
    /// scale fill 0.5 and the starting distance 3e38) and the memory router.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(0x0101_1d78, 0.0);
        e.mem.set_f32(0x0101_7718, 3.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        store_vec(&mut e, NEG_HALF_LANES, [-0.5; 4]);
        store_bits(&mut e, ABS_MASK, [0x7fff_ffff; 4]);
        store_bits(
            &mut e,
            SIGN_BITS_XYZ,
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0],
        );
        store_vec(&mut e, IDENTITY_QUATERNION, [0.0, 0.0, 0.0, 1.0]);
        e.mem.set_f32(SCALE_FILL, 0.5);
        e.mem.set_f32(WORST_DISTANCE, 3.0e38);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, Rc::default())
    }

    /// A utility with the three indices and the reference quaternion.
    fn utility(e: &mut Engine, indices: [i16; 3], reference: Vec4) -> u32 {
        let this = e.mem.alloc(0x40);
        for (k, index) in indices.into_iter().enumerate() {
            e.mem.set_i16(this + 0x20 + 2 * k as u32, index);
        }
        store_vec(e, this + 0x10, reference);
        this
    }

    fn put_points(e: &mut Engine, points: &[Vec4]) -> u32 {
        let base = e.mem.alloc(0x30 * points.len() as u32);
        for (k, point) in points.iter().enumerate() {
            store_vec(e, base + 0x30 * k as u32, *point);
        }
        base
    }

    #[test]
    fn the_orientation_comes_from_three_normalised_axes() {
        let (mut e, _) = engine();
        let this = utility(&mut e, [0, 1, 2], [0.0; 4]);
        let base = put_points(
            &mut e,
            &[[0.0; 4], [2.0, 0.0, 0.0, 0.0], [0.0, 3.0, 0.0, 0.0]],
        );
        let seen: Rc<RefCell<Vec<Vec4>>> = Rc::default();
        let s = Rc::clone(&seen);
        e.register_double(QUATERNION_FROM_MATRIX, move |e, a| {
            let rows = (0..3).map(|k| load_vec(e, a[1] + 0x10 * k));
            s.borrow_mut().extend(rows);
            store_vec(e, a[0], [7.0, 8.0, 9.0, 10.0]);
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        e.call(0x00ce_0d30, &args![this, base, out]);
        assert_eq!(load_vec(&e, out), [7.0, 8.0, 9.0, 10.0]);
        // The rows are the fourth axis (0, -1, 0), the last cross product (0, 0, -1) and the
        // first axis (1, 0, 0).
        let rows = seen.borrow();
        assert!(near(rows[0], [0.0, -1.0, 0.0, 0.0]), "{:?}", rows[0]);
        assert!(near(rows[1], [0.0, 0.0, -1.0, 0.0]), "{:?}", rows[1]);
        assert!(near(rows[2], [1.0, 0.0, 0.0, 0.0]), "{:?}", rows[2]);
    }

    #[test]
    fn coincident_points_give_zero_axes() {
        let (mut e, _) = engine();
        let this = utility(&mut e, [2, 2, 2], [0.0; 4]);
        let base = put_points(&mut e, &[[1.0; 4], [2.0; 4], [3.0, 4.0, 5.0, 6.0]]);
        let seen: Rc<RefCell<Vec<Vec4>>> = Rc::default();
        let s = Rc::clone(&seen);
        e.register_double(QUATERNION_FROM_MATRIX, move |e, a| {
            s.borrow_mut()
                .extend((0..3).map(|k| load_vec(e, a[1] + 0x10 * k)));
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        e.call(0x00ce_0d30, &args![this, base, out]);
        assert_eq!(seen.borrow()[..], [[0.0; 4]; 3]);
    }

    /// Doubles for the quaternion routines of `hkRemoveTwist`: `set` answers (0.1, 0.2, 0.3, 0.9)
    /// and logs the rows of its matrix, `setMul` answers `a + b` and logs `a`.
    fn twist_doubles(e: &mut Engine) -> Rc<RefCell<Vec<Vec4>>> {
        let rows: Rc<RefCell<Vec<Vec4>>> = Rc::default();
        let r = Rc::clone(&rows);
        e.register_double(QUATERNION_FROM_MATRIX, move |e, a| {
            r.borrow_mut()
                .extend((0..3).map(|k| load_vec(e, a[1] + 0x10 * k)));
            store_vec(e, a[0], [0.1, 0.2, 0.3, 0.9]);
            Ret::default()
        });
        let r = Rc::clone(&rows);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            let (p, q) = (load_vec(e, a[1]), load_vec(e, a[2]));
            r.borrow_mut().push(p);
            store_vec(e, a[0], add4(p, q));
            Ret::default()
        });
        rows
    }

    #[test]
    fn removing_the_twist_builds_the_matrix_and_multiplies_by_the_conjugate() {
        let (mut e, _) = engine();
        let rows = twist_doubles(&mut e);
        let reference = e.mem.alloc(0x10);
        store_vec(&mut e, reference, [0.1, 0.2, 0.9, 0.3]);
        let quaternion = e.mem.alloc(0x10);
        store_vec(&mut e, quaternion, [0.0, 0.0, 0.0, 1.0]);
        e.call(0x00ce_0ff0, &args![reference, quaternion]);
        // The largest lane of the reference is 2, so the axis is (1, 0, 0), turned by the
        // identity it stays; crossed with the reference: (0, -0.9, 0.2), normalised.
        let rows = rows.borrow();
        let want_third = [0.0, -0.976_187_06, 0.216_930_46, 0.0];
        let want_matrix = [0.921_954_4, -0.021_693_05, -0.097_618_7, 0.0];
        assert!(near(rows[0], want_matrix), "{:?}", rows[0]);
        assert_eq!(rows[1], [0.1, 0.2, 0.9, 0.3]);
        assert!(near(rows[2], want_third), "{:?}", rows[2]);
        // The conjugate (0.1, 0.2, 0.3, 0.9) with the sign bits of three lanes flipped is
        // multiplied (here: added) onto the old quaternion.
        assert_eq!(rows[3], [-0.1, -0.2, -0.3, 0.9]);
        assert_eq!(load_vec(&e, quaternion), [-0.1, -0.2, -0.3, 1.9]);
    }

    /// The unit vector of `axis x r` (lanes 0 to 2, lane 3 zero), in `f64`.
    fn unit_cross(axis: usize, r: Vec4) -> Vec4 {
        let mut a = [0.0f64; 3];
        a[axis] = 1.0;
        let r = [f64::from(r[0]), f64::from(r[1]), f64::from(r[2])];
        let c = [
            a[1] * r[2] - a[2] * r[1],
            a[2] * r[0] - a[0] * r[2],
            a[0] * r[1] - a[1] * r[0],
        ];
        let length = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
        [
            (c[0] / length) as f32,
            (c[1] / length) as f32,
            (c[2] / length) as f32,
            0.0,
        ]
    }

    #[test]
    fn the_twist_axis_is_the_one_after_the_largest_lane() {
        let (mut e, _) = engine();
        let rows = twist_doubles(&mut e);
        let reference = e.mem.alloc(0x10);
        let quaternion = e.mem.alloc(0x10);
        // The unit axis turned by the identity stays, and the third row is the normalised cross
        // product of that axis with the reference. Lane 0 largest: axis 1; lane 1: axis 2; lane
        // 2: axis 0; equal lanes go to the lower one.
        let cases = [
            ([0.9, 0.2, 0.1, 0.3], 1),
            ([0.2, 0.9, 0.1, 0.3], 2),
            ([0.1, 0.2, 0.9, 0.3], 0),
            ([-0.1, 0.2, -0.9, 0.3], 0),
            ([0.5, 0.5, 0.5, 0.3], 1),
            ([0.5, 0.5, 0.75, 0.3], 0),
            ([0.5, 0.75, 0.75, 0.3], 2),
        ];
        for (r, axis) in cases {
            store_vec(&mut e, reference, r);
            store_vec(&mut e, quaternion, [0.0, 0.0, 0.0, 1.0]);
            rows.borrow_mut().clear();
            e.call(0x00ce_0ff0, &args![reference, quaternion]);
            let third = rows.borrow()[2];
            assert!(near(third, unit_cross(axis, r)), "{r:?}: {third:?}");
        }
    }

    #[test]
    fn the_flag_test_of_the_axis_choice_treats_unordered_as_not_greater() {
        assert!(not_greater(1.0, 2.0));
        assert!(not_greater(2.0, 2.0));
        assert!(!not_greater(2.0, 1.0));
        assert!(not_greater(f32::NAN, 1.0));
        assert!(not_greater(1.0, f32::NAN));
    }

    /// A utility with three candidates (stride 0x20) and the doubles of the orientation (answers
    /// (0.5, 0.5, 0.5, 0.5)) and the twist removal (negates it), which log their registers.
    fn candidates(e: &mut Engine, log: &Log, vectors: &[Vec4]) -> u32 {
        let this = utility(e, [0, 1, 2], [0.25, 0.5, 0.75, 1.0]);
        let data = e.mem.alloc(0x20 * vectors.len() as u32);
        for (k, v) in vectors.iter().enumerate() {
            store_vec(e, data + 0x20 * k as u32, *v);
        }
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 4, vectors.len() as u32);
        let l = Rc::clone(log);
        e.register_double(COMPUTE_ORIENTATION, move |e, a| {
            l.borrow_mut().push((COMPUTE_ORIENTATION, vec![a[0], a[1]]));
            store_vec(e, a[2], [0.5; 4]);
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(REMOVE_TWIST, move |e, a| {
            l.borrow_mut().push((REMOVE_TWIST, vec![a[0]]));
            let q = load_vec(e, a[1]);
            store_vec(e, a[1], q.map(|lane| -lane));
            Ret::default()
        });
        this
    }

    #[test]
    fn the_best_candidate_is_the_first_with_the_smallest_distance() {
        let (mut e, log) = engine();
        let this = candidates(
            &mut e,
            &log,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [-0.5, -0.5, -0.5, -0.5],
            ],
        );
        let best = e.mem.alloc(4);
        let found = e.call(0x00ce_11f0, &args![this, 0x4000u32, best]).i32();
        // The orientation is negated: the dots are -0.5, -1 and 1, so the distances 0.5, 0 and
        // 0; the tie of the last keeps the second.
        assert_eq!(found, 1);
        assert_eq!(e.mem.f32(best), 0.0);
        assert_eq!(
            log.borrow()[..],
            [
                entry(COMPUTE_ORIENTATION, &[this, 0x4000]),
                entry(REMOVE_TWIST, &[this + 0x10]),
            ]
        );
    }

    #[test]
    fn no_candidate_leaves_the_starting_distance_and_minus_one() {
        let (mut e, log) = engine();
        let this = candidates(&mut e, &log, &[]);
        let best = e.mem.alloc(4);
        let found = e.call(0x00ce_11f0, &args![this, 0x4000u32, best]).i32();
        assert_eq!(found, -1);
        assert_eq!(e.mem.f32(best), 3.0e38);
        // A negative count too.
        e.mem.set_u32(this + 4, u32::MAX);
        let found = e.call(0x00ce_11f0, &args![this, 0x4000u32, best]).i32();
        assert_eq!(found, -1);
    }

    #[test]
    fn a_distance_that_is_not_a_number_never_wins() {
        let (mut e, log) = engine();
        let this = candidates(
            &mut e,
            &log,
            &[[f32::NAN, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]],
        );
        let best = e.mem.alloc(4);
        let found = e.call(0x00ce_11f0, &args![this, 0x4000u32, best]).i32();
        assert_eq!(found, 1);
        assert_eq!(e.mem.f32(best), 0.5);
        // A starting distance that is a signalling NaN comes out quiet and nothing is below it.
        e.mem.set_u32(WORST_DISTANCE, 0x7f80_0001);
        let found = e.call(0x00ce_11f0, &args![this, 0x4000u32, best]).i32();
        assert_eq!(found, -1);
        assert_eq!(e.mem.u32(best), 0x7fc0_0001);
    }

    #[test]
    fn the_reference_frame_goes_through_the_quaternion_routines() {
        let (mut e, log) = engine();
        let this = utility(&mut e, [1, 0, 0], [0.1, 0.2, 0.3, 0.9]);
        let first = put_points(&mut e, &[[0.0; 4], [1.0, 2.0, 3.0, 4.0]]);
        let second = put_points(&mut e, &[[0.0; 4], [10.0, 20.0, 30.0, 40.0]]);
        let l = Rc::clone(&log);
        e.register_double(COMPUTE_ORIENTATION, move |e, a| {
            l.borrow_mut().push((COMPUTE_ORIENTATION, vec![a[0], a[1]]));
            let q = if a[1] == first {
                [1.0, 0.0, 0.0, 0.0]
            } else {
                [0.0, 1.0, 0.0, 0.0]
            };
            store_vec(e, a[2], q);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(QUATERNION_PRODUCT, move |e, a| {
            let mut seen = vec![a[0]];
            seen.extend(bits(load_vec(e, a[1])));
            seen.extend(bits(load_vec(e, a[2])));
            l.borrow_mut().push((QUATERNION_PRODUCT, seen));
            store_vec(e, a[0], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(DECOMPOSE_REST_AXIS, move |e, a| {
            let mut seen = vec![a[0], a[1]];
            seen.extend(bits(load_vec(e, a[2])));
            seen.extend(bits(load_vec(e, a[0])));
            l.borrow_mut().push((DECOMPOSE_REST_AXIS, seen));
            e.mem.set_f32(a[3], 0.75);
            store_vec(e, a[2], [9.0; 4]);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(SET_AXIS_ANGLE, move |_, a| {
            l.borrow_mut().push((SET_AXIS_ANGLE, a.to_vec()));
            Ret::default()
        });
        let out1 = e.mem.alloc(0x30);
        let out2 = e.mem.alloc(0x30);
        e.call(0x00ce_12a0, &args![this, first, second, out1, out2]);
        // The translation is the second point minus the first turned by the identity rotation.
        for out in [out1, out2] {
            assert_eq!(load_vec(&e, out), [9.0, 18.0, 27.0, 36.0]);
            assert_eq!(load_vec(&e, out + 0x10), [0.0, 0.0, 0.0, 1.0]);
            assert_eq!(load_vec(&e, out + 0x20), [0.5; 4]);
        }
        let axis = this + 0x10;
        let log = log.borrow();
        let mut product = vec![out2 + 0x10];
        product.extend(bits([0.0, 1.0, 0.0, 0.0]));
        product.extend(bits([1.0, 0.0, 0.0, 0.0]));
        let mut decompose = vec![out1 + 0x10, axis];
        decompose.extend(bits([0.5; 4]));
        decompose.extend(bits([0.0, 0.0, 0.0, 1.0]));
        assert_eq!(
            log[..],
            [
                entry(COMPUTE_ORIENTATION, &[this, first]),
                entry(COMPUTE_ORIENTATION, &[this, second]),
                (QUATERNION_PRODUCT, product),
                (DECOMPOSE_REST_AXIS, decompose),
                entry(SET_AXIS_ANGLE, &[out1 + 0x10, axis, 0.75f32.to_bits()]),
            ]
        );
    }

    /// Doubles for the orientation (answers (1, 2, 3, 4)), the twist removal (adds 10 to every
    /// lane) and `_reserveMore` (doubles the capacity into a new block, copying the entries).
    fn adder(e: &mut Engine, log: &Log) {
        let l = Rc::clone(log);
        e.register_double(COMPUTE_ORIENTATION, move |e, a| {
            l.borrow_mut().push((COMPUTE_ORIENTATION, vec![a[0], a[1]]));
            store_vec(e, a[2], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(REMOVE_TWIST, move |e, a| {
            l.borrow_mut().push((REMOVE_TWIST, vec![a[0]]));
            let q = load_vec(e, a[1]);
            store_vec(e, a[1], q.map(|lane| lane + 10.0));
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff) * 2;
            let data = e.mem.alloc(capacity * a[2]);
            let old = e.mem.bytes(e.mem.u32(a[1]), e.mem.u32(a[1] + 4) * a[2]);
            e.mem.write(data, &old);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
    }

    fn entry_words(e: &Engine, list: u32, index: u32) -> Vec<u32> {
        let data = e.mem.u32(list);
        (0..8)
            .map(|k| e.mem.u32(data + 0x20 * index + 4 * k))
            .collect()
    }

    #[test]
    fn adding_a_candidate_appends_the_corrected_orientation() {
        let (mut e, log) = engine();
        adder(&mut e, &log);
        let list = utility(&mut e, [0, 1, 2], [0.0; 4]);
        let storage = e.mem.alloc(0x80);
        e.mem.set_u32(list, storage);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 4);
        let index = e
            .call(0x00ce_1420, &args![list, 0x4000u32, 0x1234u32, 0.5f32])
            .i32();
        assert_eq!(index, 0);
        assert_eq!(e.mem.u32(list + 4), 1);
        let mut want = bits([11.0, 12.0, 13.0, 14.0]);
        want.extend([0x1234, 0.5f32.to_bits(), 0, 0]);
        assert_eq!(entry_words(&e, list, 0), want);
        assert_eq!(
            log.borrow()[..],
            [
                entry(COMPUTE_ORIENTATION, &[list, 0x4000]),
                entry(REMOVE_TWIST, &[list + 0x10]),
            ]
        );
    }

    #[test]
    fn a_full_candidate_array_grows_first_and_a_signalling_nan_comes_out_quiet() {
        let (mut e, log) = engine();
        adder(&mut e, &log);
        let list = utility(&mut e, [0, 1, 2], [0.0; 4]);
        let data = e.mem.alloc(0x40);
        for k in 0..16 {
            e.mem.set_u32(data + 4 * k, 0x100 + k);
        }
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, 2);
        // The capacity word has the not-owned bit, which does not count.
        e.mem.set_u32(list + 8, 0x8000_0002);
        let index = e
            .call(
                0x00ce_1420,
                &args![list, 0x4000u32, 7u32, f32::from_bits(0x7f80_0001)],
            )
            .i32();
        assert_eq!(index, 2);
        assert_eq!(e.mem.u32(list + 4), 3);
        assert_ne!(e.mem.u32(list), data);
        let old: Vec<u32> = (0..8).map(|k| 0x100 + k).collect();
        assert_eq!(entry_words(&e, list, 0), old);
        let second: Vec<u32> = (0..8).map(|k| 0x108 + k).collect();
        assert_eq!(entry_words(&e, list, 1), second);
        let mut want = bits([11.0, 12.0, 13.0, 14.0]);
        want.extend([7, 0x7fc0_0001, 0, 0]);
        assert_eq!(entry_words(&e, list, 2), want);
        assert_eq!(
            log.borrow()[2],
            entry(ARRAY_RESERVE_MORE, &[ALLOCATOR, list, 0x20])
        );
    }
}
