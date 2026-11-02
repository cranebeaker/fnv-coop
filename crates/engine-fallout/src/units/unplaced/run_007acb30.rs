//! `(unplaced)/run_007acb30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `007fed10`: the destructor body this deleting destructor runs
/// (`thiscall`).
const DESTRUCTOR_007FED10: u32 = 0x007f_ed10;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 007acb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a scalar deleting destructor: `007fed10`, then `operator
/// delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_007acb30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_007FED10, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007a_cb30, fn_007acb30(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_007acb30_destroys_and_deletes_with_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_007FED10, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_cb30, &args![0x3000u32, 3u32]).u32(), 0x3000);
        assert_eq!(e.call(0x007a_cb30, &args![0x3000u32, 2u32]).u32(), 0x3000);
        let log = e.call_log.take().unwrap();
        let deletes = log.iter().filter(|c| c.0 == OPERATOR_DELETE).count();
        let destroys = log.iter().filter(|c| c.0 == DESTRUCTOR_007FED10).count();
        assert_eq!((destroys, deletes), (2, 1));
    }
}
