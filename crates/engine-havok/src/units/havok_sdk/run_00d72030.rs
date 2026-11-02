//! `Havok SDK/run_00d72030` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d72030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 0x80-byte record (thiscall on the destination `this`, one stack word: the source;
/// the exe's `buildJacobianFromMovingSurfaceModifier` callers use it): the byte at `+0`, the
/// word at `+4`, the float at `+8` (`FLD` / `FSTP`: a signalling NaN comes out quiet) and the
/// seven 16-byte vectors at `+0x10`, `+0x20` .. `+0x70`. The word at `+0xc` is not copied.
pub fn fn_00d72030(e: &mut Engine, this: Ptr, source: Ptr) {
    let to = this.addr();
    let from = source.addr();
    let byte = e.mem.u8(from);
    e.mem.set_u8(to, byte);
    let word = e.mem.u32(from.wrapping_add(4));
    e.mem.set_u32(to.wrapping_add(4), word);
    let float = x87_quiet_bits(e.mem.u32(from.wrapping_add(8)));
    e.mem.set_u32(to.wrapping_add(8), float);
    for offset in (0x10u32..0x80).step_by(0x10) {
        for lane in (0..16).step_by(4) {
            let word = e.mem.u32(from.wrapping_add(offset + lane));
            e.mem.set_u32(to.wrapping_add(offset + lane), word);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d7_2030, fn_00d72030(Ptr, Ptr))]
}

#[cfg(test)]
mod record_copy_tests {
    use super::*;

    #[test]
    fn fn_00d72030_copies_the_record_but_not_the_word_at_0xc() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x80);
        let target = e.mem.alloc(0x80);
        for i in 0..0x20u32 {
            e.mem.set_u32(source + 4 * i, 0x1000 + i);
            e.mem.set_u32(target + 4 * i, 0xdead_0000 + i);
        }
        e.mem.set_u8(source, 0x7f);
        e.mem.set_u32(source + 8, 1.5f32.to_bits());
        e.call(0x00d7_2030, &args![target, source]);
        assert_eq!(e.mem.u8(target), 0x7f);
        // The three bytes after the first one are not touched.
        assert_eq!(e.mem.u32(target) >> 8, 0xdead_0000 >> 8);
        assert_eq!(e.mem.u32(target + 4), 0x1001);
        assert_eq!(e.mem.u32(target + 8), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(target + 0xc), 0xdead_0003);
        for i in 4..0x20u32 {
            assert_eq!(e.mem.u32(target + 4 * i), 0x1000 + i);
        }
    }

    #[test]
    fn fn_00d72030_quiets_a_signalling_nan_in_the_float() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x80);
        let target = e.mem.alloc(0x80);
        e.mem.set_u32(source + 8, 0x7f80_0001);
        e.call(0x00d7_2030, &args![target, source]);
        assert_eq!(e.mem.u32(target + 8), 0x7fc0_0001);
    }
}
