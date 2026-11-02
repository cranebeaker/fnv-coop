//! `fallout/ai/run_00937d50` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00937d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModifierList::DeleteAllModifiers` (Xbox PDB name of `009370b0`) on
/// `this`; the stack word (`RET 4`) is not read.
pub fn fn_00937d50(e: &mut Engine, this: Ptr, _unused_1: u32) {
    e.call(0x0093_70b0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0093_7d50, fn_00937d50(Ptr, u32))]
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_00937d50_deletes_all_modifiers() {
        let mut e = Engine::new();
        e.register(0x0093_70b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0093_7d50, &[0x9000, 3]);
        assert_eq!(e.call_log.as_ref().unwrap()[1], (0x0093_70b0, vec![0x9000]));
    }
}
