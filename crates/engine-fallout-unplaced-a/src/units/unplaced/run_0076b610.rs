//! `(unplaced)/run_0076b610` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0076b610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `+8` of `this` is zero (an "is empty" style getter).
pub fn fn_0076b610(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) == 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0076b610, fn_0076b610(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_only_when_the_word_at_8_is_zero() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        assert!(e.call(0x0076_b610, &args![object]).bool());
        e.mem.set_u32(object.addr() + 8, 3);
        assert!(!e.call(0x0076_b610, &args![object]).bool());
    }
}
