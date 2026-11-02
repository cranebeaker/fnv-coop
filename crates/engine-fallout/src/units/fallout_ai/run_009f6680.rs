//! `fallout/ai/run_009f6680` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009f67b0` (unnamed, `searchpackage.cpp` per the engine map): the destructor (`ECX`
/// only).
const DESTRUCT_009F67B0: u32 = 0x009f_67b0;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009f6680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a scalar deleting destructor; the
/// destructor `009f67b0`, then `operator delete` on `this` for bit 0 of `flags`. Returns
/// `this`.
pub fn fn_009f6680(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_009F67B0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009f_6680, fn_009f6680(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009f6680_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCT_009F67B0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        for flags in [0u32, 1, 2, 3] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009f_6680, &args![0x5000u32, flags]).u32(), 0x5000);
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
            let mut expected = vec![DESTRUCT_009F67B0];
            if flags & 1 != 0 {
                expected.push(OPERATOR_DELETE);
            }
            assert_eq!(order, expected);
        }
    }
}
