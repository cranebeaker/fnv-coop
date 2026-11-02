//! `(unplaced)/run_00d47a40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d47a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): stores `value` at
/// `this + 8`.
pub fn fn_00d47a40(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(8), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d47a40, fn_00d47a40(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_at_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.call(0x00d47a40, &args![this, 0xabcdu32]);
        assert_eq!(e.mem.u32(this + 8), 0xabcd);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}
