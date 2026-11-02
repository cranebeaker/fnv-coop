//! `fallout shared/screencustomsplatter.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004dead0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011c7820 without reading the receiver. The map names this body `BSCustomSplatterExtraData::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_004dead0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011c_7820
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004d_ead0, fn_004dead0(Ptr) -> u32)]
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_004dead0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004d_ead0, &args![0x1000u32]).u32(), 0x011c_7820);
        assert_eq!(e.call(0x004d_ead0, &args![0u32]).u32(), 0x011c_7820);
    }
}
