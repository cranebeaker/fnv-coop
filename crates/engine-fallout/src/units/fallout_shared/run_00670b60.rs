//! `fallout shared/run_00670b60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004797c0` (`thiscall`, no stack word): the destructor.
const DESTRUCTOR_004797C0: u32 = 0x0047_97c0;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE_00401030: u32 = 0x0040_1030;

// Translated from 00670b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a scalar deleting destructor. Runs the
/// destructor `004797c0` and, for bit 0 of `flags`, `operator delete` on `this`. Returns
/// `this`.
pub fn fn_00670b60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_004797C0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_00401030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_0b60, fn_00670b60(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destroys_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_004797C0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE_00401030, |_, _| Ret::default());
        for flags in [3u32, 2] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0067_0b60, &args![0x7000u32, flags]).u32(), 0x7000);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(DESTRUCTOR_004797C0, vec![0x7000])];
            if flags & 1 == 1 {
                expected.push((OPERATOR_DELETE_00401030, vec![0x7000]));
            }
            assert_eq!(log[1..], expected[..]);
        }
    }
}
