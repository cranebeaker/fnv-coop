//! `fallout/magic/run_0080c490` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor body `00825740` (`thiscall`, no stack words).
const DESTRUCTOR_BODY: u32 = 0x0082_5740;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 0080c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor shared by many effect vtables (`CalmEffect`,
/// `ChameleonEffect`, `CommandEffect`, `FrenzyEffect`, `ShieldEffect`, ...; the class of
/// the body is not confirmed): the destructor body `00825740`, then `operator delete` when
/// bit 0 of `flags` is set (`thiscall`, `RET 4`). Returns `this`.
pub fn fn_0080c490(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_c490, fn_0080c490(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deletes_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_c490, &args![0x500u32, 2u32]).u32(), 0x500);
        assert_eq!(e.call(0x0080_c490, &args![0x500u32, 3u32]).u32(), 0x500);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|(a, _)| *a == OPERATOR_DELETE).collect();
        let bodies = log.iter().filter(|(a, _)| *a == DESTRUCTOR_BODY).count();
        assert_eq!(bodies, 2);
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![0x500]);
    }
}
