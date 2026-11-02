//! `Havok SDK/hkcolor.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMath::hkFloatToInt` (`00cbfef0`, cdecl `(float)`, answers the integer in `EAX`).
const FLOAT_TO_INT: u32 = 0x00cb_fef0;
/// The exe's constant the floats are scaled by (a double, read from memory).
const COLOR_SCALE: u32 = 0x0101_e568;

// Translated from 00cabb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkColor::rgbFromChars` (Xbox PDB; cdecl, four words of which only the low byte counts):
/// packs the four bytes into one word, the fourth in bits 24 to 31, the first in 16 to 23, the
/// second in 8 to 15 and the third in 0 to 7.
pub fn hk_color_rgb_from_chars(
    _e: &mut Engine,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u32 {
    let mut packed = (fourth & 0xff) << 8;
    packed = packed.wrapping_add(first & 0xff) << 8;
    packed = packed.wrapping_add(second & 0xff) << 8;
    packed.wrapping_add(third & 0xff)
}

// Translated from 00cabb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkColor::rgbFromFloats` (Xbox PDB; cdecl, four floats): each float is loaded (quieting a
/// signalling NaN), multiplied by the exe's double at `0101e568`, stored as a float and turned
/// into an integer by `hkMath::hkFloatToInt` (`00cbfef0`); the fourth float goes first, then
/// the first, second and third. The low bytes of the four results go through
/// [`hk_color_rgb_from_chars`] in the order of the floats.
pub fn hk_color_rgb_from_floats(
    e: &mut Engine,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
) -> u32 {
    let mut converted = [0u32; 4];
    for (slot, value) in [(3, fourth), (0, first), (1, second), (2, third)] {
        let scale: f64 = e.global(COLOR_SCALE);
        let scaled = (f64::from(x87_quiet(value)) * scale) as f32;
        converted[slot] = e.call(FLOAT_TO_INT, &args![scaled]).u32();
    }
    hk_color_rgb_from_chars(e, converted[0], converted[1], converted[2], converted[3])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cabb00, hk_color_rgb_from_chars(u32, u32, u32, u32) -> u32),
        entry!(0x00cabb30, hk_color_rgb_from_floats(f32, f32, f32, f32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_from_chars_packs_the_low_bytes() {
        let mut e = Engine::new();
        let packed = e
            .call(0x00cabb00, &args![0x11u32, 0x22u32, 0x33u32, 0x44u32])
            .u32();
        assert_eq!(packed, 0x4411_2233);
        // Only the low byte of each word counts.
        let packed = e
            .call(
                0x00cabb00,
                &args![0xabc1u32, 0x1_ff02u32, 0xdead_be03u32, 0xffff_ff04u32],
            )
            .u32();
        assert_eq!(packed, 0x04c1_0203);
    }

    /// The scale constant mapped and set, `hkFloatToInt` doubled as truncation, and the
    /// arguments it was called with recorded.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_e000, 0x1000);
        e.set_global(COLOR_SCALE, 255.0f64);
        e.register(FLOAT_TO_INT, |_, a| Ret {
            eax: f32::from_bits(a[0]) as i32 as u32,
            ..Ret::default()
        });
        e.call_log = Some(Vec::new());
        e
    }

    #[test]
    fn rgb_from_floats_scales_converts_and_packs() {
        let mut e = engine();
        let packed = e
            .call(0x00cabb30, &args![1.0f32, 0.5f32, 0.25f32, 0.0f32])
            .u32();
        // Truncated 255, 127, 63 and 0 in the order of the floats.
        assert_eq!(packed, 0x00ff_7f3f);
        // The fourth float is converted first, then the first, second and third.
        let converted: Vec<f32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == FLOAT_TO_INT)
            .map(|(_, w)| f32::from_bits(w[0]))
            .collect();
        assert_eq!(converted, vec![0.0, 255.0, 127.5, 63.75]);
    }

    #[test]
    fn rgb_from_floats_keeps_only_the_low_byte_of_each_conversion() {
        let mut e = engine();
        // 2.0 scales to 510 = 0x1fe: its low byte is 0xfe.
        let packed = e
            .call(0x00cabb30, &args![2.0f32, 1.0f32, 0.0f32, 1.0f32])
            .u32();
        assert_eq!(packed, 0xfffe_ff00);
    }

    #[test]
    fn rgb_from_floats_quiets_a_signalling_nan_before_scaling() {
        let mut e = engine();
        e.call(
            0x00cabb30,
            &args![0.0f32, 0.0f32, 0.0f32, f32::from_bits(0x7f80_0001)],
        );
        let first = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == FLOAT_TO_INT)
            .map(|(_, w)| w[0])
            .unwrap();
        assert_eq!(first, 0x7fc0_0001);
    }
}
