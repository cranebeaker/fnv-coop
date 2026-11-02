//! `Havok SDK/run_00e3f900` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e3f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `this` in `ECX` and one stack word (`RET 4`). Calls the function
/// `00cb0a70` on `this` with the word (it answers its `this` and ignores the word) and returns
/// `this`. Ghidra made no function here; it is called from 00dfdd6d.
pub fn fn_00e3f900(e: &mut Engine, this: u32, word: u32) -> u32 {
    e.call(0x00cb_0a70, &args![this, word]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_f900, fn_00e3f900(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    #[test]
    fn fn_00e3f900_calls_the_member_function_and_returns_this() {
        let mut e = Engine::new();
        e.register_double(0x00cb_0a70, |_, a| {
            assert_eq!(a, [0x1000, 0x42]);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        assert_eq!(
            e.call(0x00e3_f900, &args![0x1000u32, 0x42u32]).u32(),
            0x1000
        );
    }
}
