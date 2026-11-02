//! `BSShader/run_00bb7160` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Where `fn_00bb7160` jumps (`00ba9380`, unit `bsshaderproperty.cpp`; `thiscall`, one word:
/// the stream).
const SHADER_PROPERTY_SAVE_BINARY_00BA9380: u32 = 0x00ba_9380;

// Translated from 00bb7160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: a jump to `00ba9380` (vtable slot +0x58 of several
/// shader-property vtables, `SaveBinary` in the Xbox PDB's slot list, also called by
/// `00b68edb` and `fn_00b60210`): `thiscall`, one word (the stream), passed on unchanged.
pub fn fn_00bb7160(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(SHADER_PROPERTY_SAVE_BINARY_00BA9380, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bb_7160, fn_00bb7160(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumps_to_the_base_save() {
        let mut e = Engine::new();
        e.register(SHADER_PROPERTY_SAVE_BINARY_00BA9380, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00bb_7160, &args![0x10u32, 0x20u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(SHADER_PROPERTY_SAVE_BINARY_00BA9380, vec![0x10, 0x20])]
        );
    }
}
