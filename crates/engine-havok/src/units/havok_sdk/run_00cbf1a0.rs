//! `Havok SDK/run_00cbf1a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cbf1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vtable word `010c3b30` at `this` (a destructor body of a class with no members to
/// destroy; the class is unconfirmed). Returns nothing.
pub fn fn_00cbf1a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010c_3b30);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cb_f1a0, fn_00cbf1a0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_the_vtable_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.call(0x00cb_f1a0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010c_3b30);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}
