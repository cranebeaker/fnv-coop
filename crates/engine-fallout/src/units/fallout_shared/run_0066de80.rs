//! `fallout shared/run_0066de80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `value` is 7 or 8 (`cdecl`, signed compares; the engine map
/// gives it no name).
pub fn fn_0066de80(_e: &mut Engine, value: i32) -> bool {
    (7..=8).contains(&value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_de80, fn_0066de80(i32) -> bool)]
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn seven_or_eight() {
        let mut e = Engine::new();
        for (value, expected) in [(6i32, false), (7, true), (8, true), (9, false), (-7, false)] {
            assert_eq!(e.call(0x0066_de80, &args![value as u32]).bool(), expected);
        }
    }
}
