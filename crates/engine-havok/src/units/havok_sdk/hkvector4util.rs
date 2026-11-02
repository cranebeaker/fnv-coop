//! `Havok SDK/hkvector4util.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d975d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4Util::packQuaternionIntoInt32` (Xbox PDB name from the engine map): one stack word, the
/// address of a quaternion of four floats (cdecl, nothing popped). Each lane becomes
/// `lane * k + b` in single precision, with the four-float constants at `010d4020` (`k`) and
/// `010d4010` (`b`); the integer bits of the lanes `x`, `y`, `z`, `w` are then packed as
/// `(((((w & 0xffffffc0) << 8) | (z & 0x3fc0)) << 8) | (y & 0x3fc0)) << 2 | ((x >> 6) & 0xff)`.
/// Returns the packed word.
pub fn hk_vector4_util_pack_quaternion_into_int32(e: &mut Engine, quaternion: u32) -> u32 {
    let lane: [u32; 4] = std::array::from_fn(|i| {
        let q = e.mem.f32(quaternion + 4 * i as u32);
        let k = e.mem.f32(0x010d_4020 + 4 * i as u32);
        let b = e.mem.f32(0x010d_4010 + 4 * i as u32);
        (q * k + b).to_bits()
    });
    let mut packed = ((lane[3] & 0xffff_ffc0) << 8) | (lane[2] & 0x3fc0);
    packed = (packed << 8) | (lane[1] & 0x3fc0);
    (packed << 2) | ((lane[0] >> 6) & 0xff)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_75d0, hk_vector4_util_pack_quaternion_into_int32(u32) -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    /// A quaternion engine whose constants are `k` and `b`.
    fn packing_engine(k: [f32; 4], b: [f32; 4], quaternion: [f32; 4]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x010d_4000, 0x1000);
        for lane in 0..4usize {
            e.mem.set_f32(0x010d_4020 + 4 * lane as u32, k[lane]);
            e.mem.set_f32(0x010d_4010 + 4 * lane as u32, b[lane]);
        }
        let at = e.mem.alloc(0x10);
        for lane in 0..4usize {
            e.mem.set_f32(at + 4 * lane as u32, quaternion[lane]);
        }
        (e, at)
    }

    /// With k = 0 each lane is the constant b, so the packing of known bit patterns can be checked.
    #[test]
    fn hk_vector4_util_pack_quaternion_into_int32_packs_the_lane_bits() {
        let b = [
            f32::from_bits(0x1234_5678),
            f32::from_bits(0x2345_6789),
            f32::from_bits(0x3456_789a),
            f32::from_bits(0x4567_89ab),
        ];
        let (mut e, at) = packing_engine([0.0; 4], b, [1.0, 2.0, 3.0, 4.0]);
        let w = 0x4567_89ab_u32 & 0xffff_ffc0;
        let z = 0x3456_789a_u32 & 0x3fc0;
        let y = 0x2345_6789_u32 & 0x3fc0;
        let x = (0x1234_5678_u32 >> 6) & 0xff;
        let expected = ((((w << 8) | z) << 8 | y) << 2) | x;
        assert_eq!(e.call(0x00d9_75d0, &args![at]).u32(), expected);
    }

    /// The lanes are `lane * k + b`: a quaternion of 0.5, 0.25, -0.5, 1.0 with k = 4 and b = 4.
    #[test]
    fn hk_vector4_util_pack_quaternion_into_int32_scales_and_offsets_each_lane() {
        let (mut e, at) = packing_engine([4.0; 4], [4.0; 4], [0.5, 0.25, -0.5, 1.0]);
        // 6.0 = 0x40c00000, 5.0 = 0x40a00000, 2.0 = 0x40000000, 8.0 = 0x41000000.
        let x = (0x40c0_0000_u32 >> 6) & 0xff;
        let y = 0x40a0_0000_u32 & 0x3fc0;
        let z = 0x4000_0000_u32 & 0x3fc0;
        let w = 0x4100_0000_u32 & 0xffff_ffc0;
        let expected = ((((w << 8) | z) << 8 | y) << 2) | x;
        assert_eq!(e.call(0x00d9_75d0, &args![at]).u32(), expected);
    }
}
