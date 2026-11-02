//! `(unplaced)/run_00aa0ad0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a81fe0`: the jump target of `00aa0ad0`.
const JUMP_TARGET_00AA0AD0: u32 = 0x00a8_1fe0;

// Translated from 00aa0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00a81fe0`, passing on `this` and its one argument; returns the target result. Return type (bool): its jump chain ends in `00a5d2f0` (`NiObject::RegisterStreamables`, engine map, `bool` in the Xbox PDB), whose result `00a84c70` tests with `TEST AL,AL`.
pub fn fn_00aa0ad0(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00AA0AD0, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_0ad0, fn_00aa0ad0(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00aa0ad0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00aa_0ad0;
        const TARGET: u32 = 0x00a8_1fe0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
