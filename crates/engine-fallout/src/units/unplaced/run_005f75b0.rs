//! `(unplaced)/run_005f75b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005f7900` (`thiscall`, no stack word; unit `fallout shared/tescreature.cpp`): the
/// destructor.
const DESTRUCTOR: u32 = 0x005f_7900;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 005f75b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a scalar deleting destructor. Runs the
/// destructor `005f7900` and, for bit 0 of `flags`, `operator delete` on `this`. Returns
/// `this`.
pub fn fn_005f75b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_75b0, fn_005f75b0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destroys_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        for flags in [1u32, 0] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x005f_75b0, &args![0x7000u32, flags]).u32(), 0x7000);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(DESTRUCTOR, vec![0x7000])];
            if flags == 1 {
                expected.push((OPERATOR_DELETE, vec![0x7000]));
            }
            assert_eq!(log[1..], expected[..]);
        }
    }
}
