//! `BSShader/run_00ba3750` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ba3750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3750`): a byte setter, `thiscall`, one word: stores the
/// argument's low byte at +2.
pub fn fn_00ba3750(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 2, value);
}

// Translated from 00ba3760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3750`): a word setter, `thiscall`, one word: stores the
/// argument at +0xc.
pub fn fn_00ba3760(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xc, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_3750, fn_00ba3750(Ptr, u8)),
        entry!(0x00ba_3760, fn_00ba3760(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ba3750_stores_the_low_byte_at_2() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcc; 0x10]);
        e.call(0x00ba_3750, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(this + 2), 1);
        assert_eq!(e.mem.u8(this + 1), 0xcc);
        assert_eq!(e.mem.u8(this + 3), 0xcc);
    }

    #[test]
    fn fn_00ba3760_stores_the_word_at_0xc() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcc; 0x10]);
        e.call(0x00ba_3760, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234_5678);
        assert_eq!(e.mem.u32(this + 8), 0xcccc_cccc);
    }
}
