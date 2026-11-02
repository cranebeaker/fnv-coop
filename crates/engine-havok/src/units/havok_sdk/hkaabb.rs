//! `Havok SDK/hkaabb.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00da0b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabb::isValid` (Xbox PDB; `this` is ECX, one stack word `result`): writes 1 to the byte
/// at `result` when for each of the first three lanes the minimum (`this + 4 * i`) and the
/// maximum (`this + 0x10 + 4 * i`) both have an exponent field other than all ones (not
/// infinite, not a NaN; the values are tested after an x87 load and store, which quiets a
/// signalling NaN without changing its exponent) and the minimum is not greater than the
/// maximum, 0 otherwise. The comparison is `FCOMPP` followed by `TEST AH, 5` and `JNP`: the
/// box is invalid when exactly one of C0 (maximum below minimum, or unordered) and C2
/// (unordered) is set; an unordered pair sets both and so would pass, but cannot occur after
/// the exponent tests.
/// Answers `result` (`EAX` at both `RET`s: `MOV EAX,[ESP+0xc]` at 00da0b78 and 00da0b85; its
/// caller reads the answer through it, `CMP byte ptr [EAX],0` at 00d1fc03).
pub fn hk_aabb_is_valid(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let this = this.addr();
    let mut valid = true;
    for lane in 0..3u32 {
        let minimum_bits = x87_quiet_bits(e.mem.u32(this + 4 * lane));
        if minimum_bits & 0x7f80_0000 == 0x7f80_0000 {
            valid = false;
            break;
        }
        let maximum_bits = x87_quiet_bits(e.mem.u32(this + 0x10 + 4 * lane));
        if maximum_bits & 0x7f80_0000 == 0x7f80_0000 {
            valid = false;
            break;
        }
        let minimum = e.mem.f32(this + 4 * lane);
        let maximum = e.mem.f32(this + 0x10 + 4 * lane);
        let unordered = minimum.is_nan() || maximum.is_nan();
        let below = unordered || maximum < minimum;
        let flags = u8::from(below) | (u8::from(unordered) << 2);
        if flags.count_ones() % 2 == 1 {
            valid = false;
            break;
        }
    }
    // 00da0b78 / 00da0b85: EAX = the `result` word, then the byte is written through it.
    e.mem.set_u8(result.addr(), u8::from(valid));
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00da_0b30, hk_aabb_is_valid(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The validity byte for a box with the given lower and upper lanes (the fourth lane of
    /// each is NaN: it is not looked at).
    fn valid(lower: [f32; 3], upper: [f32; 3]) -> u8 {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let result = e.mem.alloc(8);
        e.mem.set_u8(result, 0xee);
        for (lane, (low, high)) in lower.iter().zip(&upper).enumerate() {
            e.mem.set_f32(this + 4 * lane as u32, *low);
            e.mem.set_f32(this + 0x10 + 4 * lane as u32, *high);
        }
        e.mem.set_u32(this + 12, 0x7fc0_0000);
        e.mem.set_u32(this + 0x1c, 0x7f80_0000);
        e.call(0x00da_0b30, &args![this, result]);
        e.mem.u8(result)
    }

    #[test]
    fn an_ordered_finite_box_is_valid() {
        assert_eq!(valid([-1.0, -2.0, -3.0], [1.0, 2.0, 3.0]), 1);
        // Equal bounds are allowed, and so is the largest finite value.
        assert_eq!(valid([0.0; 3], [0.0; 3]), 1);
        assert_eq!(valid([-f32::MAX; 3], [f32::MAX; 3]), 1);
    }

    #[test]
    fn a_minimum_above_the_maximum_in_any_lane_is_invalid() {
        assert_eq!(valid([2.0, 0.0, 0.0], [1.0, 1.0, 1.0]), 0);
        assert_eq!(valid([0.0, 2.0, 0.0], [1.0, 1.0, 1.0]), 0);
        assert_eq!(valid([0.0, 0.0, 2.0], [1.0, 1.0, 1.0]), 0);
    }

    #[test]
    fn infinities_and_nans_are_invalid_in_either_bound() {
        let nan = f32::NAN;
        let signalling = f32::from_bits(0x7fa0_0000);
        assert_eq!(valid([f32::NEG_INFINITY, 0.0, 0.0], [1.0; 3]), 0);
        assert_eq!(valid([0.0; 3], [1.0, f32::INFINITY, 1.0]), 0);
        assert_eq!(valid([0.0, 0.0, nan], [1.0; 3]), 0);
        assert_eq!(valid([0.0; 3], [signalling, 1.0, 1.0]), 0);
    }

    #[test]
    fn is_valid_answers_its_result_pointer() {
        // EAX is `result` at both RETs (00da0b78, 00da0b85); detectPenetration reads the
        // answer through it (00d1fc03).
        let boxes = [([-1.0f32; 3], [1.0f32; 3], 1u8), ([2.0; 3], [1.0; 3], 0)];
        for (lower, upper, expected) in boxes {
            let mut e = Engine::new();
            let this = e.mem.alloc(0x20);
            let result = e.mem.alloc(8);
            e.mem.set_u8(result, 0xee);
            for lane in 0..3u32 {
                e.mem.set_f32(this + 4 * lane, lower[lane as usize]);
                e.mem.set_f32(this + 0x10 + 4 * lane, upper[lane as usize]);
            }
            let answered = e.call(0x00da_0b30, &args![this, result]).u32();
            assert_eq!(answered, result);
            assert_eq!(e.mem.u8(answered), expected);
        }
    }
}
