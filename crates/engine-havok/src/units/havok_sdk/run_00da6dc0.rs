//! `Havok SDK/run_00da6dc0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00da6dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: five stack words (cdecl, nothing popped), the first never read
/// (`_unused_0`). Calls `00da42b0` with the second word, the fifth, the third and the fourth, and
/// returns its answer (`EAX`). Ghidra made no function here; the address is an immediate operand at 00d2da0e.
pub fn fn_00da6dc0(e: &mut Engine, _unused_0: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    e.call(0x00da_42b0, &args![a, d, b, c]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00da_6dc0, fn_00da6dc0(u32, u32, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    #[test]
    fn fn_00da6dc0_reorders_its_words() {
        let mut e = Engine::new();
        e.register_double(0x00da_42b0, |_, a| {
            assert_eq!(a, [2, 5, 3, 4]);
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        assert_eq!(
            e.call(0x00da_6dc0, &args![1u32, 2u32, 3u32, 4u32, 5u32])
                .u32(),
            0x77
        );
    }
}
