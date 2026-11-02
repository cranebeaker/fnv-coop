//! `Havok SDK/hkaquantizereal.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e51da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCalcQuantizedSize` (Xbox PDB), cdecl `(count, record)`: the bytes `count` values take under
/// the quantisation record at `record` (byte 0 the bits per value, byte 1 the number of values
/// kept at full size). The packed part is `(count - byte 1) * byte 0 + 7` bits divided by 8
/// (signed, rounding toward zero), plus 4 bytes for each full-size value.
pub fn hk_calc_quantized_size(e: &mut Engine, count: i32, record: Ptr) -> i32 {
    let bits = i32::from(e.mem.u8(record.addr()));
    let full = i32::from(e.mem.u8(record.addr() + 1));
    let total = count.wrapping_sub(full).wrapping_mul(bits).wrapping_add(7);
    let rounded = total.wrapping_add((total >> 31) & 7);
    (rounded >> 3).wrapping_add(full.wrapping_mul(4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_1da0, hk_calc_quantized_size(i32, Ptr) -> i32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn size(e: &mut Engine, record: [u8; 2], count: i32) -> i32 {
        let at = e.mem.alloc(8);
        e.mem.set_u8(at, record[0]);
        e.mem.set_u8(at + 1, record[1]);
        e.call(0x00e5_1da0, &args![count, at]).i32()
    }

    #[test]
    fn the_size_is_the_packed_bits_in_bytes_plus_four_per_full_value() {
        let mut e = Engine::new();
        // (10 - 2) * 5 + 7 = 47 bits -> 5 bytes, plus 2 * 4.
        assert_eq!(size(&mut e, [5, 2], 10), 13);
        // Exactly 8 bits' worth: (9 - 0) * 8 + 7 = 79 -> 9.
        assert_eq!(size(&mut e, [8, 0], 9), 9);
        assert_eq!(size(&mut e, [0, 0], 100), 0);
    }

    #[test]
    fn a_negative_bit_count_rounds_toward_zero() {
        let mut e = Engine::new();
        // (0 - 4) * 3 + 7 = -5 -> 0 bytes (not -1), plus 4 * 4.
        assert_eq!(size(&mut e, [3, 4], 0), 16);
        // (0 - 2) * 10 + 7 = -13 -> -1 byte, plus 2 * 4.
        assert_eq!(size(&mut e, [10, 2], 0), 7);
        // (1 - 2) * 10 + 7 = -3 -> 0 bytes.
        assert_eq!(size(&mut e, [10, 2], 1), 8);
    }
}
