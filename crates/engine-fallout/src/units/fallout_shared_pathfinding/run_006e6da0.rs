//! `fallout shared/pathfinding/run_006e6da0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e6da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a cdecl function of two words that returns the
/// second one; the first is never read.
pub fn fn_006e6da0(_e: &mut Engine, _unused_1: u32, second: u32) -> u32 {
    second
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e6da0, fn_006e6da0(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_second_word() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x006e6da0, &args![4u32, 0x6004u32]).u32(), 0x6004);
    }
}
