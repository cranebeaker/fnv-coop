//! `Havok SDK/run_00e10a00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e10a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: clears the word at `this` and returns `this` (`EAX`). The class is
/// unconfirmed.
pub fn fn_00e10a00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e1_0a00, fn_00e10a00(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The first word is cleared, the next is untouched, `this` comes back.
    #[test]
    fn fn_00e10a00_clears_the_first_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0x77; 0x10]);
        assert_eq!(e.call(0x00e1_0a00, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0x7777_7777);
    }
}
