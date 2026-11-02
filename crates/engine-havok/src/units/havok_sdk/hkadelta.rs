//! `Havok SDK/hkadelta.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `FADD dword` on the x87 stack: `first` is the value on the stack, `second` the memory
/// operand, both as float bit patterns, with the x87 NaN choice (#199: a quiet NaN beats a
/// signalling memory operand, then the larger significand, then the positive one); otherwise
/// the sum rounded to a float (rounding the f64 sum once gives the same float).
fn x87_add_bits(first: u32, second: u32) -> u32 {
    x87_store32(x87_add(x87_load32(first), x87_m32(second)))
}

// Translated from 00e592a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkInverseDeltaTransform` (Xbox PDB; cdecl `(values, count)`): turns `count` floats from
/// deltas into running sums, in place. The running sum starts as the first float, which stays
/// unchanged, and every later float becomes the sum so far plus itself, rounded to a float at
/// each step. The exe handles the floats from index 1 in groups of four while at least four
/// are left (`count - 1 >= 4`, signed) and the rest one at a time; the order of the additions
/// is the same either way, only the operand order differs (the first float of a group, and
/// every float of the single-step loop, is the stack value, the running sum the memory
/// operand; inside a group it is the other way round), which matters only for NaN results.
pub fn hk_inverse_delta_transform(e: &mut Engine, values: Ptr, count: u32) {
    let base = values.addr();
    let mut running = x87_quiet_bits(e.mem.u32(base));
    let mut at = base.wrapping_add(4);
    let mut next_index = 1u32;
    if (count.wrapping_sub(1) as i32) >= 4 {
        let groups = (count.wrapping_sub(5) >> 2).wrapping_add(1);
        next_index = groups.wrapping_mul(4).wrapping_add(1);
        for _ in 0..groups {
            running = x87_add_bits(e.mem.u32(at), running);
            e.mem.set_u32(at, running);
            for lane in 1..4u32 {
                let slot = at.wrapping_add(4 * lane);
                running = x87_add_bits(running, e.mem.u32(slot));
                e.mem.set_u32(slot, running);
            }
            at = at.wrapping_add(16);
        }
    }
    if next_index < count {
        for _ in 0..count - next_index {
            running = x87_add_bits(e.mem.u32(at), running);
            e.mem.set_u32(at, running);
            at = at.wrapping_add(4);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_92a0, hk_inverse_delta_transform(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The floats at a fresh block, transformed; returns their bit patterns.
    fn transformed(input: &[f32], count: u32) -> Vec<u32> {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let block = e.mem.alloc(4 * input.len() as u32 + 16);
        for (index, value) in input.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, value.to_bits());
        }
        hk_inverse_delta_transform(&mut e, Ptr::new(block), count);
        (0..input.len() as u32)
            .map(|index| e.mem.u32(block + 4 * index))
            .collect()
    }

    /// The sums an obvious left-to-right loop gives.
    fn running_sums(input: &[f32]) -> Vec<u32> {
        let mut sums = Vec::new();
        let mut total = 0.0f32;
        for (index, value) in input.iter().enumerate() {
            total = if index == 0 { *value } else { total + *value };
            sums.push(total.to_bits());
        }
        sums
    }

    #[test]
    fn counts_of_zero_and_one_change_nothing() {
        let input = [1.5, 2.5, 3.5];
        let bits: Vec<u32> = input.iter().map(|value: &f32| value.to_bits()).collect();
        assert_eq!(transformed(&input, 0), bits);
        assert_eq!(transformed(&input, 1), bits);
    }

    #[test]
    fn counts_below_five_use_only_the_single_step_loop() {
        let input = [1.0, 2.0, 4.0, 8.0, 16.0];
        for count in 2..=4usize {
            let result = transformed(&input, count as u32);
            assert_eq!(result[..count], running_sums(&input[..count])[..]);
            // Floats past `count` stay as they were.
            for (after, original) in result[count..].iter().zip(&input[count..]) {
                assert_eq!(*after, original.to_bits());
            }
        }
    }

    #[test]
    fn groups_of_four_and_the_remainder_both_accumulate() {
        let input: Vec<f32> = (0..14).map(|index| 0.1 * (index as f32 + 1.0)).collect();
        for count in [5usize, 6, 8, 9, 10, 13, 14] {
            let result = transformed(&input, count as u32);
            assert_eq!(
                result[..count],
                running_sums(&input[..count])[..],
                "{count}"
            );
        }
    }

    #[test]
    fn the_sum_is_rounded_to_a_float_at_every_step() {
        // 2^24 + 1 is not a float: every added 1.0 is lost, in both loops.
        let big = 16_777_216.0f32;
        let input = [big, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        assert_eq!(transformed(&input, 7), vec![big.to_bits(); 7]);
    }

    #[test]
    fn a_nan_spreads_through_the_rest_and_a_signalling_one_is_quieted() {
        let input = [f32::from_bits(0x7f80_0001), 1.0, 2.0, 3.0, 4.0, 5.0];
        let result = transformed(&input, 6);
        // The first float itself is not written; the running sum starts as its quiet copy.
        assert_eq!(result[0], 0x7f80_0001);
        for bits in &result[1..] {
            assert_eq!(*bits, 0x7fc0_0001);
        }
    }

    #[test]
    fn two_nans_keep_the_one_with_the_larger_significand() {
        let input = [
            f32::from_bits(0x7fc0_0003),
            f32::from_bits(0x7fc0_0007),
            1.0,
        ];
        let result = transformed(&input, 3);
        assert_eq!(result, vec![0x7fc0_0003, 0x7fc0_0007, 0x7fc0_0007]);
    }

    #[test]
    fn opposite_infinities_give_the_default_nan() {
        let input = [f32::INFINITY, f32::NEG_INFINITY, 1.0];
        let result = transformed(&input, 3);
        assert_eq!(result[1], 0xffc0_0000);
        assert_eq!(result[2], 0xffc0_0000);
    }
}
