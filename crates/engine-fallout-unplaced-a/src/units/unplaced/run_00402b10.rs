//! `(unplaced)/run_00402b10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00402d50` (`thiscall`, no stack word): the destructor.
const DESTRUCTOR_00402D50: u32 = 0x0040_2d50;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE_00401030: u32 = 0x0040_1030;

// Translated from 00402b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a scalar deleting destructor. Runs the
/// destructor `00402d50` and, for bit 0 of `flags`, `operator delete` on `this`. Returns
/// `this`.
pub fn fn_00402b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_00402D50, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_00401030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_2b10, fn_00402b10(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destroys_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_00402D50, |_, _| Ret::default());
        e.register(OPERATOR_DELETE_00401030, |_, _| Ret::default());
        for flags in [1u32, 0] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0040_2b10, &args![0x7000u32, flags]).u32(), 0x7000);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(DESTRUCTOR_00402D50, vec![0x7000])];
            if flags == 1 {
                expected.push((OPERATOR_DELETE_00401030, vec![0x7000]));
            }
            assert_eq!(log[1..], expected[..]);
        }
    }
}
