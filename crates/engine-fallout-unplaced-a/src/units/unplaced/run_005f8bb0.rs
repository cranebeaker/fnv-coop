//! `(unplaced)/run_005f8bb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f8bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a setter (`thiscall`, `RET 4`) that stores its
/// word at `this + 0x14c`. The class and field are unconfirmed.
pub fn fn_005f8bb0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x14c), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_8bb0, fn_005f8bb0(Ptr, u32))]
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    #[test]
    fn fn_005f8bb0_stores_the_word_at_0x14c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        e.call(0x005f_8bb0, &args![this, 0xdead_beef_u32]);
        assert_eq!(e.mem.u32(this + 0x14c), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x148), 0);
        assert_eq!(e.mem.u32(this + 0x150), 0);
    }
}
