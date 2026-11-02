//! `BSCore/run_00aa50f0` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`): clears the byte at +0, stores 0.0 in the
/// floats at +4, +8 and +0xc (`FLDZ`/`FSTP`), clears the word at +0x14, stores `value` at
/// +0x18 and clears the bytes at +0x1d and +0x1c (in that order). The word at +0x10 is not
/// written. `EAX` is left holding `value` (a by-product of the last store, not a result).
pub fn fn_00aa50f0(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    e.mem.set_u8(base, 0);
    for offset in [4u32, 8, 0xc] {
        e.mem.set_u32(base + offset, 0.0f32.to_bits());
    }
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, value);
    e.mem.set_u8(base + 0x1d, 0);
    e.mem.set_u8(base + 0x1c, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_50f0, fn_00aa50f0(Ptr, u32))]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn clears_the_fields_and_stores_the_value() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(this + 4 * i, 0xaaaa_aaaa);
        }
        e.call(0x00aa_50f0, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this), 0xaaaa_aa00);
        for offset in [4u32, 8, 0xc, 0x14] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x10), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(this + 0x18), 0x1234);
        assert_eq!(e.mem.u32(this + 0x1c), 0xaaaa_0000);
    }
}
