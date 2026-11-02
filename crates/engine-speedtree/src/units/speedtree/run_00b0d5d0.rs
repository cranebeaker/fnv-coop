//! `SpeedTree/run_00b0d5d0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b0d5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; called by `UpdateBSTreeNode`
/// `0066c237`): the word at +0x68.
pub fn fn_00b0d5d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x68))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b0_d5d0, fn_00b0d5d0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00b0d5d0_answers_the_word_at_0x68() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x68, 0x5678);
        assert_eq!(e.call(0x00b0_d5d0, &args![this]).u32(), 0x5678);
    }
}
