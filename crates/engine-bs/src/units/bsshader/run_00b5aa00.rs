//! `BSShader/run_00b5aa00` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b5aa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no words): packs four flag bytes of the object
/// (+0xe6 to +0xe9) into a 4-bit value in `EAX`, bit 0 set when the byte at +0xe6 is zero, bit
/// 1 for +0xe7, bit 2 for +0xe8 and bit 3 for +0xe9. Its callers (`00b5f170`, `00c48f3d`
/// `BuildForPortal`) pass an occlusion plane.
pub fn fn_00b5aa00(e: &mut Engine, this: Ptr) -> u32 {
    let mut bits = u32::from(e.mem.u8(this.addr().wrapping_add(0xe9)) == 0);
    for offset in [0xe8u32, 0xe7, 0xe6] {
        bits = bits.wrapping_add(bits) | u32::from(e.mem.u8(this.addr().wrapping_add(offset)) == 0);
    }
    bits
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b5_aa00, fn_00b5aa00(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_a_bit_per_zero_byte() {
        let mut e = Engine::new();
        let plane = e.mem.alloc(0xf0);
        for offset in 0xe6..0xea {
            e.mem.set_u8(plane + offset, 1);
        }
        assert_eq!(e.call(0x00b5_aa00, &args![plane]).u32(), 0);
        e.mem.set_u8(plane + 0xe6, 0);
        assert_eq!(e.call(0x00b5_aa00, &args![plane]).u32(), 1);
        e.mem.set_u8(plane + 0xe9, 0);
        assert_eq!(e.call(0x00b5_aa00, &args![plane]).u32(), 9);
        e.mem.set_u8(plane + 0xe7, 0);
        e.mem.set_u8(plane + 0xe8, 0);
        assert_eq!(e.call(0x00b5_aa00, &args![plane]).u32(), 0xf);
    }
}
