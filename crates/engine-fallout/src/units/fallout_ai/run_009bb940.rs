//! `fallout/ai/run_009bb940` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c905b0` (`thiscall`, no stack word; unit `Havok SDK/hkreferencedobject.cpp`).
const HAVOK_RELEASE: u32 = 0x00c9_05b0;

// Translated from 009bb940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): when the word at +0x1c holds an
/// object, calls `00c905b0` on it; then clears the word.
pub fn fn_009bb940(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr() + 0x1c);
    if object != 0 {
        e.call(HAVOK_RELEASE, &args![object]);
    }
    e.mem.set_u32(this.addr() + 0x1c, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009b_b940, fn_009bb940(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn releases_the_object_and_clears_the_word() {
        let mut e = Engine::new();
        e.register(HAVOK_RELEASE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x009b_b940, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 0x1c, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x009b_b940, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(HAVOK_RELEASE, vec![0x7000])]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }
}
