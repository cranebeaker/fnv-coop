//! `fallout/interface/run_007e8840` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TraitSelectMenu::SetBoxAsChecked` (`007e86c0`, `RET 8`).
const SET_BOX_AS_CHECKED: u32 = 0x007e_86c0;

// Translated from 007e8840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): `TraitSelectMenu::SetBoxAsChecked(-1, 0)` on `this`.
pub fn fn_007e8840(e: &mut Engine, this: Ptr) {
    e.call(SET_BOX_AS_CHECKED, &args![this, 0xffff_ffffu32, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007e_8840, fn_007e8840(Ptr))]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn unchecks_every_box() {
        let mut e = Engine::new();
        e.register(SET_BOX_AS_CHECKED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007e_8840, &args![0x5000u32]);
        assert!(e
            .call_log
            .unwrap()
            .contains(&(SET_BOX_AS_CHECKED, vec![0x5000, 0xffff_ffff, 0])));
    }
}
