//! `Havok SDK/run_00ce4e20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SSE code, computed lane by lane in `f32`; `RSQRTPS`/`RSQRTSS` are the host's estimate
//! (`rsqrt_estimate`, which differs between Intel and AMD parts in the last bits).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{addps, mulps, shufps, splat, subps};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, Vec4};
use crate::x87::rsqrt_estimate;

/// The vector constants `3.0` (`010c71c0`) and `0.5` (`010c71d0`) in all four lanes, and the
/// scalars `1.0` (`01012068`), `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`).
const THREE_VECTOR: u32 = 0x010c_71c0;
const HALF_VECTOR: u32 = 0x010c_71d0;
const ONE: u32 = 0x0101_2068;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;

/// The sum of the squares of a vector's four lanes as the `SHUFPS 0x4e`/`ADDPS`/`SHUFPS
/// 0xb1`/`ADDPS` sequence forms it (every lane of the result holds it): `(q0 + q2) + (q1 + q3)`
/// with `q` the lane squares. Returns lane 0.
fn sum_of_squares(row: Vec4) -> f32 {
    let squares = mulps(row, row);
    let pairs = addps(shufps(squares, squares, 0x4e), squares);
    addps(shufps(pairs, pairs, 0xb1), pairs)[0]
}

// Translated from 00ce4e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(blocks, scales, count)`: `count` blocks of three 16-byte rows (`0x30` bytes each,
/// the class is unconfirmed), `scales` a float per block. First, rows 0 and 2 of every block
/// are multiplied by `1.0 / scale` (all four lanes; `DIVSS`). Then the middle rows (row 1,
/// at `+0x10` of a block) are normalised: in groups of four blocks (`count >> 2`) the squared
/// lengths of the four rows (four lanes each, summed as `sum_of_squares`, each copied through
/// the x87 stack so that a signalling NaN is quieted) go through `RSQRTPS` and one Newton step,
/// `(3 - ((r * x) * r)) * (r * 0.5)`, and every row is multiplied by its factor; the
/// remaining `count & 3` blocks one by one with the scalar form `(0.5 * r) * (3 - ((x * r) *
/// r))` from `RSQRTSS`, whose factor is 0 when the squared length equals 0.
pub fn fn_00ce4e20(e: &mut Engine, blocks: Ptr, scales: Ptr, count: u32) {
    let one = e.mem.f32(ONE);
    let mut block = blocks.addr();
    for index in 0..count {
        let scale = e.mem.f32(scales.addr() + 4 * index);
        let inverse = splat(one / scale);
        let first = load_vec(e, block);
        store_vec(e, block, mulps(inverse, first));
        let third = load_vec(e, block + 0x20);
        store_vec(e, block + 0x20, mulps(inverse, third));
        block += 0x30;
    }
    let mut block = blocks.addr();
    for _ in 0..count >> 2 {
        let rows: [Vec4; 4] = [
            load_vec(e, block + 0x10),
            load_vec(e, block + 0x40),
            load_vec(e, block + 0x70),
            load_vec(e, block + 0xa0),
        ];
        let sums: Vec4 = std::array::from_fn(|lane| x87_quiet(sum_of_squares(rows[lane])));
        let estimates: Vec4 = std::array::from_fn(|lane| rsqrt_estimate(sums[lane]));
        let three = load_vec(e, THREE_VECTOR);
        let half = load_vec(e, HALF_VECTOR);
        let refined = subps(three, mulps(mulps(estimates, sums), estimates));
        let factors = mulps(refined, mulps(estimates, half));
        for (lane, offset) in [0x10u32, 0x40, 0x70, 0xa0].into_iter().enumerate() {
            let row = load_vec(e, block + offset);
            store_vec(e, block + offset, mulps(splat(factors[lane]), row));
        }
        block += 0xc0;
    }
    let three = e.mem.f32(THREE);
    let half = e.mem.f32(HALF);
    let zero = e.mem.f32(ZERO);
    block += 0x10;
    for _ in 0..count & 3 {
        let row = load_vec(e, block);
        let length_squared = sum_of_squares(row);
        let estimate = rsqrt_estimate(length_squared);
        let refined = (half * estimate) * (three - ((length_squared * estimate) * estimate));
        let factor = if length_squared == zero { 0.0 } else { refined };
        store_vec(e, block, mulps(splat(factor), row));
        block += 0x30;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ce_4e20, fn_00ce4e20(Ptr, Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ONE, 1.0);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        for lane in 0..4 {
            e.mem.set_f32(THREE_VECTOR + 4 * lane, 3.0);
            e.mem.set_f32(HALF_VECTOR + 4 * lane, 0.5);
        }
        e
    }

    /// `count` blocks whose row 1 is `middle[i]` and whose rows 0 and 2 are numbered.
    fn blocks(e: &mut Engine, middle: &[[f32; 4]]) -> u32 {
        let at = e.mem.alloc(0x30 * middle.len().max(1) as u32);
        for (i, row) in middle.iter().enumerate() {
            let block = at + 0x30 * i as u32;
            for lane in 0..4 {
                e.mem
                    .set_f32(block + 4 * lane, 1.0 + lane as f32 + i as f32);
                e.mem.set_f32(block + 0x10 + 4 * lane, row[lane as usize]);
                e.mem
                    .set_f32(block + 0x20 + 4 * lane, 10.0 * (1.0 + lane as f32));
            }
        }
        at
    }

    fn scales(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len().max(1) as u32);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
        at
    }

    /// Within a few units in the last place: the estimate is the host's.
    fn close(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= 6.0e-7 * expected.abs().max(1e-6)
    }

    const MIDDLE: [[f32; 4]; 6] = [
        [1.0, 2.0, 2.0, 0.0],
        [0.0, 3.0, 4.0, 0.0],
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 3.0, 6.0, 0.0],
        [0.0, 0.0, 12.0, 5.0],
        [8.0, 0.0, 0.0, 6.0],
    ];
    const NORMS: [f32; 6] = [3.0, 5.0, 2.0, 7.0, 13.0, 10.0];

    /// Rows 0 and 2 are scaled by `1 / scale` in all four lanes, and row 1 is normalised with
    /// all four lanes counting, for blocks handled by the groups of four, the tail, and the
    /// two together.
    #[test]
    fn rows_are_scaled_and_the_middle_rows_normalised() {
        for count in [1usize, 2, 4, 5, 6] {
            let mut e = engine();
            let at = blocks(&mut e, &MIDDLE[..count]);
            let values: Vec<f32> = (0..count).map(|i| 2.0 + i as f32).collect();
            let scale_at = scales(&mut e, &values);
            e.call(0x00ce_4e20, &args![at, scale_at, count as u32]);
            for i in 0..count {
                let block = at + 0x30 * i as u32;
                for lane in 0..4 {
                    let first = 1.0 + lane as f32 + i as f32;
                    assert_eq!(
                        e.mem.f32(block + 4 * lane),
                        first * (1.0 / values[i]),
                        "count {count} block {i} row 0 lane {lane}"
                    );
                    let third = 10.0 * (1.0 + lane as f32);
                    assert_eq!(
                        e.mem.f32(block + 0x20 + 4 * lane),
                        third * (1.0 / values[i])
                    );
                    let got = e.mem.f32(block + 0x10 + 4 * lane);
                    let want = MIDDLE[i][lane as usize] / NORMS[i];
                    assert!(
                        close(got, want),
                        "count {count} block {i} lane {lane}: {got} vs {want}"
                    );
                }
            }
        }
    }

    /// Nothing happens for a count of zero.
    #[test]
    fn a_count_of_zero_changes_nothing() {
        let mut e = engine();
        let at = blocks(&mut e, &MIDDLE[..1]);
        let scale_at = scales(&mut e, &[2.0]);
        e.call(0x00ce_4e20, &args![at, scale_at, 0u32]);
        assert_eq!(e.mem.f32(at), 1.0);
        assert_eq!(e.mem.f32(at + 0x10 + 4), 2.0);
    }

    /// A zero row: the scalar form gives a factor of 0 (the row stays 0), the packed form a
    /// NaN (the estimate of the reciprocal root of 0 is infinite and `0 * inf` is NaN).
    #[test]
    fn zero_rows_stay_zero_in_the_tail_and_become_nan_in_groups() {
        let mut e = engine();
        let at = blocks(&mut e, &[[0.0; 4]]);
        let scale_at = scales(&mut e, &[1.0]);
        e.call(0x00ce_4e20, &args![at, scale_at, 1u32]);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(at + 0x10 + 4 * lane), 0.0);
        }
        let mut e = engine();
        let at = blocks(&mut e, &[[0.0; 4]; 4]);
        let scale_at = scales(&mut e, &[1.0; 4]);
        e.call(0x00ce_4e20, &args![at, scale_at, 4u32]);
        for lane in 0..4 {
            assert!(e.mem.f32(at + 0x10 + 4 * lane).is_nan());
        }
    }
}
