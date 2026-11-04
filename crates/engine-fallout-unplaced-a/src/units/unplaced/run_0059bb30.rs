//! `(unplaced)/run_0059bb30` (no source unit: the engine map files `0059bb30` under
//! `d3d9/texture.cpp` by a folded Xbox name, `D3DTexture_LockRect`; the PC body is a plain
//! getter shared by unrelated callers, research/engine-map/unit_overrides.tsv), subsystem
//! `(unplaced)`: its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0059bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A getter (`thiscall`, no stack words): answers the word at `this + 0x24`. Identical-code
/// folding gave it many callers (LowProcess's copy `00907510`, the character controller's
/// landing in `actor_p3.rs`, the cell's reference loading, ...); the map's name belongs to
/// another function.
pub fn fn_0059bb30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x24)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0059bb30, fn_0059bb30(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_word_at_0x24() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x30);
        e.mem.set_u32(0x1024, 0xfeed);
        assert_eq!(e.call(0x0059_bb30, &args![0x1000u32]).u32(), 0xfeed);
    }
}
