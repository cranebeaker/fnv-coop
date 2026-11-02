//! `(unplaced)/run_0051f570` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): stores `value` in the
/// word at +0x218.
pub fn fn_0051f570(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x218, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_f570, fn_0051f570(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0051f570_stores_the_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x220);
        e.call(0x0051_f570, &args![object, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(object + 0x218), 0x1234_5678);
        assert_eq!(e.mem.u32(object + 0x214), 0);
        assert_eq!(e.mem.u32(object + 0x21c), 0);
    }
}
