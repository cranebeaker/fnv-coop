//! `Havok SDK/hkmemoryexceptiontestingutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word `hkSetOutOfMemoryState` writes (a global in `.data`).
const OUT_OF_MEMORY_STATE_01269200: u32 = 0x0126_9200;

// Translated from 00d88040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSetOutOfMemoryState` (map name). cdecl, one word: stores it in the
/// global at `01269200`.
pub fn hk_set_out_of_memory_state(e: &mut Engine, state: u32) {
    e.set_global(OUT_OF_MEMORY_STATE_01269200, state);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d88040, hk_set_out_of_memory_state(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hk_set_out_of_memory_state_stores_the_word_in_the_global() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        e.call(0x00d8_8040, &args![3u32]);
        assert_eq!(e.global::<u32>(OUT_OF_MEMORY_STATE_01269200), 3);
    }
}
