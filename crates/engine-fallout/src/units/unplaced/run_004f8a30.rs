//! `(unplaced)/run_004f8a30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004f8a90` (`thiscall`, `RET 8`: a key and the address of the answer word).
const LOOKUP: u32 = 0x004f_8a90;

// Translated from 004f8a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): runs `004f8a90` on `this` with `key`
/// and the address of a local word holding 0, and answers that word.
pub fn fn_004f8a30(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    e.with_stack(4, |e, answer| {
        e.mem.set_u32(answer.addr(), 0);
        e.call(LOOKUP, &args![this, key, answer]);
        e.mem.u32(answer.addr())
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_8a30, fn_004f8a30(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_what_the_lookup_wrote() {
        let mut e = Engine::new();
        e.register(LOOKUP, |e, a| {
            if a[1] == 7 {
                e.mem.set_u32(a[2], 0x77);
            }
            Ret::default()
        });
        assert_eq!(e.call(0x004f_8a30, &args![0x5000u32, 7u32]).u32(), 0x77);
        assert_eq!(e.call(0x004f_8a30, &args![0x5000u32, 8u32]).u32(), 0);
    }
}
