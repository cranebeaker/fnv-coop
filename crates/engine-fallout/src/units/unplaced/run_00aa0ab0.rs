//! `(unplaced)/run_00aa0ab0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1000`: the jump target of `00aa0ab0`.
const JUMP_TARGET_00AA0AB0: u32 = 0x00aa_1000;

// Translated from 00aa0ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00aa1000` (engine map: `NiKeyBasedInterpolator::CopyMembers`), passing on `this` and its two arguments. Return type (void): the exe's only callers of it are `CopyMembers`/`CreateClone` bodies (xrefs), it takes (this, destination, cloning process) and every `CopyMembers` in the Xbox PDB returns `void`; the class is unnamed in the engine map.
pub fn fn_00aa0ab0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00AA0AB0, &args![this, first, second]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_0ab0, fn_00aa0ab0(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00aa0ab0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00aa_0ab0;
        const TARGET: u32 = 0x00aa_1000;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }
}
