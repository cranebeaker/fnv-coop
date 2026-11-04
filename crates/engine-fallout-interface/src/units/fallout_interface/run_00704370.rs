//! `fallout/interface/run_00704370` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00704370 (decompiled, FalloutNV.exe 1.4.0.525)
/// A getter: the word at +0x174 of `this` (`ECX`). Which class and field
/// it reads is not confirmed (the unit is an unnamed range of the
/// interface code).
pub fn fn_00704370(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x174))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_4370, fn_00704370(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_word_at_0x174() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x174, 0xdead_beef);
        assert_eq!(e.call(0x0070_4370, &args![this]).u32(), 0xdead_beef);
    }
}
