//! `NiMain/nishaderlibrary.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d47a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderLibrary::GetName` (Xbox PDB; `thiscall`, plain `RET`): the word
/// at `this + 0x8`.
pub fn ni_shader_library_get_name(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x8))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_7a30, ni_shader_library_get_name(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_name_reads_the_word_at_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234_5678);
        assert_eq!(e.call(0x00d4_7a30, &[this]).u32(), 0x1234_5678);
    }
}
