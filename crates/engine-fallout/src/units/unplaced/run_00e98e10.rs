//! `(unplaced)/run_00e98e10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array clear `00e98cb0` (thiscall: one byte word).
const CLEAR_ARRAY: u32 = 0x00e9_8cb0;

// Translated from 00e98e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a jump to `00e98cb0` on the member at `this + 0x18`, passing
/// on its word.
pub fn fn_00e98e10(e: &mut Engine, this: Ptr, clear: u32) {
    e.call(CLEAR_ARRAY, &args![this.addr().wrapping_add(0x18), clear]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_8e10, fn_00e98e10(Ptr, u32))]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn forwards_to_the_member() {
        let mut e = Engine::new();
        e.register(CLEAR_ARRAY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e9_8e10, &args![0x1000u32, 0x101u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (CLEAR_ARRAY, vec![0x1018, 0x101])
        );
    }
}
