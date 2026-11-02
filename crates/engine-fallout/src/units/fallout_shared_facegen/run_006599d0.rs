//! `fallout shared/facegen/run_006599d0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor body `006599d0` runs first (`00659600`, thiscall, unnamed in the engine map).
const DESTRUCTOR_00659600: u32 = 0x0065_9600;
/// the allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;

// Translated from 006599d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (unnamed in the engine map; its class is
/// not confirmed): runs the destructor `00659600` on `this` and, when bit
/// 0 of the flags is set, frees the object. Answers `this` (`RET 4`: one
/// stack word, the flags).
pub fn fn_006599d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_00659600, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006599d0, fn_006599d0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn destroys_and_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_00659600, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_99d0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls(&e, DESTRUCTOR_00659600), vec![vec![0x5000]]);
        assert_eq!(calls(&e, FREE), vec![vec![0x5000]]);
        e.call_log = Some(vec![]);
        e.call(0x0065_99d0, &args![0x5000u32, 2u32]);
        assert_eq!(calls(&e, DESTRUCTOR_00659600).len(), 1);
        assert!(calls(&e, FREE).is_empty());
    }
}
