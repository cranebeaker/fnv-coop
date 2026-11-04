//! `fallout/ai/run_0087d880` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0087d880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments, `this` unread): answers 0.0 in ST0.
pub fn fn_0087d880(_e: &mut Engine, _this: Ptr) -> f32 {
    0.0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0087d880, fn_0087d880(Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0087d880_answers_zero() {
        let mut e = Engine::new();
        let value = e.call(0x0087d880, &args![0x5000u32]).f64();
        assert_eq!(value.to_bits(), 0);
    }
}
