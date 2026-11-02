//! `Havok SDK/run_00d1bad0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::hkaabbtree::rcp_estimate;

/// The four lanes at `at`.
fn lanes_00d1bad0(e: &Engine, at: u32) -> [f32; 4] {
    [0u32, 1, 2, 3].map(|lane| e.mem.f32(at.wrapping_add(lane * 4)))
}

fn store_lanes_00d1bad0(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (lane, word) in value.into_iter().enumerate() {
        e.mem
            .set_u32(at.wrapping_add(lane as u32 * 4), word.to_bits());
    }
}

/// `SHUFPS dst, src, selector`: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
fn shufps_00d1bad0(dst: [f32; 4], src: [f32; 4], selector: u8) -> [f32; 4] {
    let pick = |from: [f32; 4], shift: u8| from[usize::from((selector >> shift) & 3)];
    [pick(dst, 0), pick(dst, 2), pick(src, 4), pick(src, 6)]
}

/// `MINPS dst, src` on one lane: `dst` when it is below `src`, else `src`.
fn minps_00d1bad0(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` on one lane: `dst` when it is above `src`, else `src`.
fn maxps_00d1bad0(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

// Translated from 00d1bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `aabb`, `point`; descriptions, not confirmed): the squared distance
/// from the point (four lanes at `point`) to the box whose minimum lanes are at `aabb` and maximum
/// lanes at `aabb + 0x10`: 0.0 when the first three lanes are inside (`min <= p` and `p <= max`,
/// `CMPLEPS`), else the sum `z + (y + x)` of the squares of `max(min(p, max), min) - p`. The result
/// is returned in `ST0` as a float.
pub fn fn_00d1bad0(e: &mut Engine, aabb: u32, point: u32) -> f32 {
    let p = lanes_00d1bad0(e, point);
    let minimum = lanes_00d1bad0(e, aabb);
    let maximum = lanes_00d1bad0(e, aabb.wrapping_add(0x10));
    if (0..3).all(|lane| minimum[lane] <= p[lane] && p[lane] <= maximum[lane]) {
        return 0.0;
    }
    let squares = [0usize, 1, 2, 3].map(|lane| {
        let clamped = maxps_00d1bad0(minps_00d1bad0(p[lane], maximum[lane]), minimum[lane]);
        let delta = clamped - p[lane];
        delta * delta
    });
    let low_pair = squares[1] + squares[0];
    x87_quiet(squares[2] + low_pair)
}

// Translated from 00d1bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, one stack word of which the low 16 bits are used): the address of
/// element `index` of the array of 0x30-byte elements whose base pointer is the word at `this`.
pub fn fn_00d1bb40(e: &mut Engine, this: Ptr, index: u16) -> Ptr {
    let base = e.mem.u32(this.addr());
    Ptr::new(base.wrapping_add(u32::from(index) * 0x30))
}

// Translated from 00d1bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, ten words `out`, then `first` ... `eighth` pointers to four-lane vectors
/// at `+0xc`, `+0x10`, ..., `+0x28`, then `extra`; descriptions, not confirmed): transposes the
/// eight vectors into lane-wise vectors with `SHUFPS`: `out + 0x00`, `+0x10`, `+0x20` get lanes
/// 0, 1, 2 of the 1st, 3rd, 5th and 7th vector; the same lanes of the 2nd, 4th, 6th and 8th
/// are the local vectors B. `extra` goes to `out + 0x60`. Then for each of the three lane groups
/// `d = B - A` and `out + 0x30`, `+0x40`, `+0x50` get the reciprocal of `d` (lanes where
/// `d == 0.0` use 1.0 for the estimate, and are replaced by the float `0x7f7fffee` at
/// `010cd670` afterwards): `RCPPS` (the host CPU's estimate, per lane) with one Newton step
/// `(r + r) - (r * x) * r`. The comparison vector is read from `01267e30` (zero in the exe), the
/// replacement from `010cd670` and the one from `010c7170`.
pub fn fn_00d1bb50(
    e: &mut Engine,
    out: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    eighth: u32,
    extra: u32,
) {
    let v0 = lanes_00d1bad0(e, first);
    let v1 = lanes_00d1bad0(e, second);
    let v2 = lanes_00d1bad0(e, third);
    let v3 = lanes_00d1bad0(e, fourth);
    let v4 = lanes_00d1bad0(e, fifth);
    let v5 = lanes_00d1bad0(e, sixth);
    let v6 = lanes_00d1bad0(e, seventh);
    let v7 = lanes_00d1bad0(e, eighth);
    let high_a = shufps_00d1bad0(v0, v2, 0xee);
    let low_a = shufps_00d1bad0(v0, v2, 0x44);
    let low_c = shufps_00d1bad0(v4, v6, 0x44);
    let high_c = shufps_00d1bad0(v4, v6, 0xee);
    let a_z = shufps_00d1bad0(high_a, high_c, 0x88);
    let a_x = shufps_00d1bad0(low_a, low_c, 0x88);
    let a_y = shufps_00d1bad0(low_a, low_c, 0xdd);
    let low_b = shufps_00d1bad0(v1, v3, 0x44);
    let high_b = shufps_00d1bad0(v1, v3, 0xee);
    let low_d = shufps_00d1bad0(v5, v7, 0x44);
    let high_d = shufps_00d1bad0(v5, v7, 0xee);
    let b_z = shufps_00d1bad0(high_b, high_d, 0x88);
    let b_x = shufps_00d1bad0(low_b, low_d, 0x88);
    let b_y = shufps_00d1bad0(low_b, low_d, 0xdd);
    e.mem.set_u32(out.wrapping_add(0x60), extra);
    let compare = lanes_00d1bad0(e, 0x0126_7e30);
    let replacement = lanes_00d1bad0(e, 0x010c_d670);
    let one = lanes_00d1bad0(e, 0x010c_7170);
    store_lanes_00d1bad0(e, out.wrapping_add(0x20), a_z);
    store_lanes_00d1bad0(e, out, a_x);
    store_lanes_00d1bad0(e, out.wrapping_add(0x10), a_y);
    for (group, (a, b)) in [(a_x, b_x), (a_y, b_y), (a_z, b_z)].into_iter().enumerate() {
        let difference = [0usize, 1, 2, 3].map(|lane| b[lane] - a[lane]);
        let result = [0usize, 1, 2, 3].map(|lane| {
            let is_zero = difference[lane] == compare[lane];
            let divisor = if is_zero { one[lane] } else { difference[lane] };
            let estimate = rcp_estimate(divisor);
            let refined = (estimate + estimate) - (estimate * divisor) * estimate;
            if is_zero {
                replacement[lane]
            } else {
                refined
            }
        });
        store_lanes_00d1bad0(e, out.wrapping_add(0x30 + group as u32 * 0x10), result);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_bad0, fn_00d1bad0(u32, u32) -> f32),
        entry!(0x00d1_bb40, fn_00d1bb40(Ptr, u16) -> Ptr),
        entry!(
            0x00d1_bb50,
            fn_00d1bb50(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        store_lanes_00d1bad0(e, at, lanes);
        at
    }

    #[test]
    fn fn_00d1bad0_is_zero_inside_and_the_squared_distance_outside() {
        let mut e = Engine::new();
        let aabb = e.mem.alloc(0x20);
        store_lanes_00d1bad0(&mut e, aabb, [0.0, 1.0, 2.0, 9.0]);
        store_lanes_00d1bad0(&mut e, aabb + 0x10, [4.0, 5.0, 6.0, 9.0]);
        // Inside (the fourth lane is ignored).
        let inside = vector(&mut e, [2.0, 3.0, 4.0, 100.0]);
        assert_eq!(e.call(0x00d1_bad0, &args![aabb, inside]).f32(), 0.0);
        // On the boundary counts as inside.
        let boundary = vector(&mut e, [4.0, 1.0, 6.0, 0.0]);
        assert_eq!(e.call(0x00d1_bad0, &args![aabb, boundary]).f32(), 0.0);
        // Outside: x by 3 below the minimum, y by 2 above the maximum, z inside.
        let outside = vector(&mut e, [-3.0, 7.0, 3.0, 0.0]);
        assert_eq!(e.call(0x00d1_bad0, &args![aabb, outside]).f32(), 13.0);
        // Outside only in z, by 1.5.
        let above = vector(&mut e, [1.0, 2.0, 7.5, 0.0]);
        assert_eq!(e.call(0x00d1_bad0, &args![aabb, above]).f32(), 2.25);
        // A NaN lane is not inside; the squared distance is NaN.
        let nan = vector(&mut e, [f32::NAN, 2.0, 3.0, 0.0]);
        assert!(e.call(0x00d1_bad0, &args![aabb, nan]).f32().is_nan());
    }

    #[test]
    fn fn_00d1bb40_indexes_the_array_of_0x30_byte_elements() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0200_0000);
        assert_eq!(e.call(0x00d1_bb40, &args![this, 0u32]).u32(), 0x0200_0000);
        assert_eq!(e.call(0x00d1_bb40, &args![this, 3u32]).u32(), 0x0200_0090);
        // Only the low 16 bits of the stack word count.
        assert_eq!(
            e.call(0x00d1_bb40, &args![this, 0x0001_0002u32]).u32(),
            0x0200_0060
        );
    }

    #[test]
    fn fn_00d1bb50_transposes_and_takes_reciprocals_of_the_differences() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        store_lanes_00d1bad0(&mut e, 0x0126_7e30, [0.0; 4]);
        store_lanes_00d1bad0(&mut e, 0x010c_d670, [f32::from_bits(0x7f7f_ffee); 4]);
        store_lanes_00d1bad0(&mut e, 0x010c_7170, [1.0; 4]);
        // Vector k holds lanes 10 k + 0 .. 10 k + 3; the odd ones (B) are 2 above the even ones
        // (A) in x, 4 above in y, equal in z (a zero difference).
        let a: Vec<[f32; 4]> = (0..4)
            .map(|k| [1.0 + k as f32, 2.0 + k as f32, 3.0 + k as f32, 0.0])
            .collect();
        let b: Vec<[f32; 4]> = a
            .iter()
            .map(|v| [v[0] + 2.0, v[1] + 4.0, v[2], 7.0])
            .collect();
        let mut pointers = vec![];
        for k in 0..4 {
            pointers.push(vector(&mut e, a[k]));
            pointers.push(vector(&mut e, b[k]));
        }
        let out = e.mem.alloc(0x70);
        e.mem.set_u32(out + 0x60, 0x1111);
        e.call(
            0x00d1_bb50,
            &args![
                out,
                pointers[0],
                pointers[1],
                pointers[2],
                pointers[3],
                pointers[4],
                pointers[5],
                pointers[6],
                pointers[7],
                0xabcdu32
            ],
        );
        let read = |e: &Engine, at: u32| lanes_00d1bad0(e, at);
        for (group, axis) in [(0, 0usize), (1, 1), (2, 2)] {
            let transposed: Vec<f32> = (0..4).map(|k| a[k][axis]).collect();
            assert_eq!(read(&e, out + group * 0x10).to_vec(), transposed);
        }
        assert_eq!(e.mem.u32(out + 0x60), 0xabcd);
        let near = |value: f32, expected: f32| (value - expected).abs() <= expected.abs() * 1e-6;
        // x differences are 2 (reciprocal 0.5), y differences 4 (0.25).
        for lane in read(&e, out + 0x30) {
            assert!(near(lane, 0.5), "{lane}");
        }
        for lane in read(&e, out + 0x40) {
            assert!(near(lane, 0.25), "{lane}");
        }
        // z differences are zero: the replacement constant.
        assert_eq!(read(&e, out + 0x50).map(f32::to_bits), [0x7f7f_ffee_u32; 4]);
    }
}
