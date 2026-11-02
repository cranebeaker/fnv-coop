//! `BSMain/run_00c3c230` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c3c230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false (`XOR AL,AL`; plain `RET`, nothing in the exe refers to it).
pub fn fn_00c3c230(_e: &mut Engine) -> bool {
    false
}

// Translated from 00c3c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (plain `RET`; nothing in the exe refers to it).
pub fn fn_00c3c240(_e: &mut Engine) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_c230, fn_00c3c230() -> bool),
        entry!(0x00c3_c240, fn_00c3c240()),
    ]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_stubs_answer_false_and_nothing() {
        let mut e = Engine::new();
        assert!(!e.call(0x00c3_c230, &args![]).bool());
        e.call_log = Some(vec![]);
        e.call(0x00c3_c240, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
