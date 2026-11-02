//! `fallout/ai/run_008d5830` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d5830 (decompiled, FalloutNV.exe 1.4.0.525)
/// A number for each `kind` (cdecl; the exe's jump table at `008d58d4`,
/// translated as match arms): 1500 for 0 and 1, 500 for 2, 3 and 11,
/// 180000 for 4 and 6, 120000 for 5, 1000 for 7, 8 and 10, 60000 for 9,
/// 20000 for 12, 3000 for 13 and 500 for every other kind.
/// `followerbarks.rs` uses it as a bark's cooldown.
pub fn fn_008d5830(_e: &mut Engine, kind: u32) -> u32 {
    match kind {
        0 | 1 => 1500,
        2 | 3 | 11 => 500,
        4 | 6 => 180_000,
        5 => 120_000,
        7 | 8 | 10 => 1000,
        9 => 60_000,
        12 => 20_000,
        13 => 3000,
        _ => 500,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d_5830, fn_008d5830(u32) -> u32)]
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_008d5830_maps_every_kind() {
        let mut e = Engine::new();
        let expected = [
            1500, 1500, 500, 500, 180_000, 120_000, 180_000, 1000, 1000, 60_000, 1000, 500, 20_000,
            3000, 500, 500,
        ];
        for (kind, want) in expected.iter().enumerate() {
            assert_eq!(e.call(0x008d_5830, &[kind as u32]).u32(), *want);
        }
        assert_eq!(e.call(0x008d_5830, &[0xffff_ffff]).u32(), 500);
    }
}
