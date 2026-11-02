//! `fallout/magic/run_00826260` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `OpenEffect` constructor `008260b0` (`thiscall`, three words).
const CONSTRUCTOR: u32 = 0x0082_60b0;

// Translated from 00826260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00826260 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x48-byte `OpenEffect`
/// (`00401000`) and constructs it with the three words (`008260b0`); returns it, or null
/// when the allocation fails.
pub fn fn_00826260(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(CONSTRUCTOR, &args![block, caster, spell, effect])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_6260, fn_00826260(u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_6260, &[7, 8, 9]).u32(), 0x5001);
        let log = e.call_log.take().unwrap();
        let allocs: Vec<_> = log.iter().filter(|(a, _)| *a == ALLOCATE).collect();
        assert_eq!(allocs[0].1, vec![0x48]);
        let ctors: Vec<_> = log.iter().filter(|(a, _)| *a == CONSTRUCTOR).collect();
        assert_eq!(ctors[0].1, vec![0x5000, 7, 8, 9]);

        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_6260, &[7, 8, 9]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == CONSTRUCTOR));
    }
}
