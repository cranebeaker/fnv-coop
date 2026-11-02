//! `NiCollision/run_00eae6d0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00eae6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiIntersector` destructor body (shared by the derived
/// intersectors' destructors): sets the `NiIntersector` vtable `010f1a84`.
pub fn fn_00eae6d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010f_1a84);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ea_e6d0, fn_00eae6d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_base_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this, 0x1234);
        e.call(0x00ea_e6d0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010f_1a84);
    }
}
