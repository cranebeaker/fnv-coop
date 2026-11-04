//! `(unplaced)/run_0088b0a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word at this offset of the object (class not identified).
const VALUE_OFFSET: u32 = 0x520;

// Translated from 0088b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall setter (`RET 4`): stores `value` at `this + 0x520`.
pub fn fn_0088b0a0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + VALUE_OFFSET, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0088b0a0, fn_0088b0a0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_at_0x520() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(0x600));
        e.call(0x0088_b0a0, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this.addr() + 0x520), 0xdead_beef);
    }
}
