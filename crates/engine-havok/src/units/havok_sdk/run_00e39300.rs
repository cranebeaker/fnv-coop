//! `Havok SDK/run_00e39300` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e39300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: copies a 0xd8-byte record from `source` into `this` (`ECX`) with
/// gaps, field by field in the exe's order: the vectors at `+0x00`, `+0x10` (`MOVAPS`, bit for
/// bit), the float at `+0x20` (`FLD`/`FSTP`: a signalling NaN is copied quiet), the dwords at
/// `+0x24..+0x44`, the vectors at `+0x50` and `+0x60`, the float at `+0x70`, the vectors at
/// `+0x80`, `+0x90`, `+0xa0` and `+0xb0` and the floats at `+0xc0..+0xd4` (all quiet copies).
/// The bytes `+0x48..+0x4f` and `+0x74..+0x7f` are not copied. The function takes one stack
/// word and pops it (`RET 4`); the result in `EAX` is `this` (the decompiler shows `void`).
pub fn fn_00e39300(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    let copy_bits = |e: &mut Engine, offset: u32| {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    };
    let copy_float = |e: &mut Engine, offset: u32| {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, x87_quiet_bits(word));
    };
    for offset in [0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x18, 0x1c] {
        copy_bits(e, offset);
    }
    copy_float(e, 0x20);
    for offset in [0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44] {
        copy_bits(e, offset);
    }
    for offset in [0x50, 0x54, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c] {
        copy_bits(e, offset);
    }
    copy_float(e, 0x70);
    for offset in (0x80..0xc0u32).step_by(4) {
        copy_bits(e, offset);
    }
    for offset in [0xc0, 0xc4, 0xc8, 0xcc, 0xd0, 0xd4] {
        copy_float(e, offset);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_9300, fn_00e39300(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Every copied field arrives; the gaps keep the old bytes; the four quiet-copied float
    /// positions turn a signalling NaN quiet while the bit-copied ones keep it.
    #[test]
    fn fn_00e39300_copies_the_fields_and_leaves_the_gaps() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0xe0);
        let this = e.mem.alloc(0xe0);
        for word in 0..0x38u32 {
            e.mem.set_u32(source + 4 * word, 0x1000 + word);
            e.mem.set_u32(this + 4 * word, 0xdead_0000 + word);
        }
        // A signalling NaN in a float copy (+0x20), in a plain dword (+0x24) and in one of
        // the late floats (+0xd0).
        e.mem.set_u32(source + 0x20, 0x7f80_0001);
        e.mem.set_u32(source + 0x24, 0x7f80_0002);
        e.mem.set_u32(source + 0xd0, 0xff80_0003);
        assert_eq!(e.call(0x00e3_9300, &args![this, source]).u32(), this);
        for word in 0..0x36u32 {
            let offset = 4 * word;
            let expected = match offset {
                0x20 => 0x7fc0_0001,
                0x24 => 0x7f80_0002,
                0xd0 => 0xffc0_0003,
                0x48 | 0x4c | 0x74 | 0x78 | 0x7c => 0xdead_0000 + word,
                _ => 0x1000 + word,
            };
            assert_eq!(e.mem.u32(this + offset), expected, "offset {offset:#x}");
        }
        // Past the record nothing is written.
        assert_eq!(e.mem.u32(this + 0xd8), 0xdead_0000 + 0x36);
        assert_eq!(e.mem.u32(this + 0xdc), 0xdead_0000 + 0x37);
    }
}
