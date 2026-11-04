//! `(unplaced)/run_004b0530` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004b0530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor that only sets the
/// vtable `0101fc80`.
pub fn fn_004b0530(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_fc80);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004b_0530, fn_004b0530(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.call(0x004b_0530, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0101_fc80);
    }
}
