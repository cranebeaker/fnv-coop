//! `RemoteLog/socket.cpp` (Xbox PDB source unit), subsystem `RemoteLog`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dc1660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `operator_new` (folded code; it sits in the vtables at
/// `010d6ae0`, `010ea3c4` and `010eb9fc`): `thiscall`, `RET 4`, one stack word, which it
/// answers (`this` is not read).
pub fn fn_00dc1660(_e: &mut Engine, _this: Ptr, word: u32) -> u32 {
    word
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dc_1660, fn_00dc1660(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00dc1660_answers_its_word() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00dc_1660, &args![0x1000u32, 0x1234u32]).u32(),
            0x1234
        );
    }
}
