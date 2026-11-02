//! `fallout shared/run_004a8a90` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (`thiscall`): answers the word at `this`.
const FIRST_WORD: u32 = 0x0055_9450;

// Translated from 004a8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00559450` on `this + 0x30`, answering what
/// it answers (the word there).
pub fn fn_004a8a90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FIRST_WORD, &args![this.addr().wrapping_add(0x30)])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004a8a90, fn_004a8a90(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_this_plus_0x30() {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x004a8a90, &args![0x1000u32]).u32(), 0x1031);
    }
}
