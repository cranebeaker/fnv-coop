//! `(unplaced)/run_00cf08c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The absolute-value mask (`01017d10`, four `0x7fffffff` lanes in the exe).
const ABS_MASK: u32 = 0x0101_7d10;
/// The tolerance (`01017d00`, a float).
const TOLERANCE: u32 = 0x0101_7d00;

// Translated from 00cf08c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; called from an `IsEqual`): whether two arrays of
/// 16-byte vectors (data +0, count +4) are equal within a tolerance: the counts must be
/// equal, then for each vector pair (while all before matched) every lane of
/// `(a - b) & mask` (`SUBPS`, `ANDPS` with the four words at `01017d10`) must be `<=` the
/// float at `01017d00` (`CMPLEPS`, false for a NaN). Equal counts of 0 or less are equal.
pub fn fn_00cf08c0(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    use crate::sse::{compare, subps};
    let first = first.addr();
    let second = second.addr();
    let count = e.mem.i32(first + 4);
    let mut equal = count == e.mem.i32(second + 4);
    if !equal || count <= 0 {
        return equal;
    }
    let mask: [u32; 4] = std::array::from_fn(|lane| e.mem.u32(ABS_MASK + 4 * lane as u32));
    let tolerance = e.mem.f32(TOLERANCE);
    let mut index: i32 = 0;
    let mut offset = 0u32;
    loop {
        if !equal {
            break;
        }
        let a_data = e.mem.u32(first);
        let a: [f32; 4] =
            std::array::from_fn(|lane| e.mem.f32(a_data.wrapping_add(offset + 4 * lane as u32)));
        let b_data = e.mem.u32(second);
        let b: [f32; 4] =
            std::array::from_fn(|lane| e.mem.f32(b_data.wrapping_add(offset + 4 * lane as u32)));
        let difference = subps(a, b);
        equal = (0..4).all(|lane| {
            let magnitude = f32::from_bits(difference[lane].to_bits() & mask[lane]);
            compare(magnitude, tolerance, 2)
        });
        index += 1;
        offset = offset.wrapping_add(0x10);
        if index >= count {
            break;
        }
    }
    equal
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf_08c0, fn_00cf08c0(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    fn array(e: &mut Engine, vectors: &[[f32; 4]]) -> u32 {
        let header = e.mem.alloc(8);
        let data = e.mem.alloc(16 * vectors.len().max(1) as u32);
        for (i, v) in vectors.iter().enumerate() {
            for (lane, x) in v.iter().enumerate() {
                e.mem.set_f32(data + 16 * i as u32 + 4 * lane as u32, *x);
            }
        }
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, vectors.len() as u32);
        header
    }

    #[test]
    fn compares_counts_then_each_lane_within_the_tolerance() {
        let mut e = Engine::new();
        e.map(TOLERANCE, 0x20);
        e.mem.set_f32(TOLERANCE, 0.001);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        let base = array(&mut e, &[[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]]);
        for (other, want) in [
            (vec![[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]], true),
            (vec![[1.0005, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 7.9995]], true),
            (vec![[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.1, 8.0]], false),
            (vec![[1.0, 2.0, 3.0, 4.0], [5.0, f32::NAN, 7.0, 8.0]], false),
            (vec![[1.0, 2.0, 3.0, 4.0]], false),
        ] {
            let other = array(&mut e, &other);
            assert_eq!(e.call(0x00cf_08c0, &args![base, other]).bool(), want);
        }
        let empty = array(&mut e, &[]);
        let empty_too = array(&mut e, &[]);
        assert!(e.call(0x00cf_08c0, &args![empty, empty_too]).bool());
    }
}
