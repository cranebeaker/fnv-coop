//! `fallout shared/magic/run_0040ca10` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0040ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; its caller `0050c1d0` pushes one word
/// and cleans it, the body does not read it): returns -1.
pub fn fn_0040ca10(_e: &mut Engine, _unused_0: u32) -> i32 {
    -1
}

// Translated from 0040ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; its callers push one word, a skill
/// from `EffectItemList::GetAssociatedSkill` in `0050bd60`, and clean it,
/// the body does not read it): returns 6.
pub fn fn_0040ca30(_e: &mut Engine, _unused_0: u32) -> i32 {
    6
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_ca10, fn_0040ca10(u32) -> i32),
        entry!(0x0040_ca30, fn_0040ca30(u32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_minus_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_ca10, &args![5u32]).i32(), -1);
    }

    #[test]
    fn answers_six() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_ca30, &args![5u32]).i32(), 6);
    }
}
