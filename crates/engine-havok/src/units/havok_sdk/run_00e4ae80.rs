//! `Havok SDK/run_00e4ae80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e4ae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4` (the stack word is not read): stores the vtable `010ec1ec` and returns `this`.
pub fn fn_00e4ae80(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let this = this.addr();
    e.mem.set_u32(this, 0x010e_c1ec);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e4_ae80, fn_00e4ae80(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_vtable_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00e4_ae80, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_c1ec);
    }
}
