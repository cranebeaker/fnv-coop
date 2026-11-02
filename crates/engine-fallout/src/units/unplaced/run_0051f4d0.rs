//! `(unplaced)/run_0051f4d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. `thiscall`, no stack arguments: returns
/// the word at `this + 0x1cc`. `tesobjectweap.rs` calls it as the weapon's
/// critical effect getter (that name is that file's, unconfirmed here).
pub fn fn_0051f4d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1cc)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051f4d0, fn_0051f4d0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_word_at_0x1cc() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object + 0x1cc, 0x0123_4567);
        assert_eq!(e.call(0x0051f4d0, &args![object]).u32(), 0x0123_4567);
    }
}
