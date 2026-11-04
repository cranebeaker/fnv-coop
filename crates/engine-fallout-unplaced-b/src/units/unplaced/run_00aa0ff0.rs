//! `(unplaced)/run_00aa0ff0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa0ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at +8 is not null (thiscall; called by `00b9d020`).
pub fn fn_00aa0ff0(e: &mut Engine, this: u32) -> bool {
    e.mem.u32(this + 8) != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_0ff0, fn_00aa0ff0(u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_the_word_at_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        assert!(!e.call(0x00aa_0ff0, &[this]).bool());
        e.mem.set_u32(this + 8, 0x100);
        assert!(e.call(0x00aa_0ff0, &[this]).bool());
    }
}
