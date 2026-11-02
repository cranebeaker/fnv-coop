//! `fallout shared/run_0048ef40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048ef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vtable `0101d8e8` into `this` (the engine map has no name);
/// `EAX` is left holding `this`, which is answered.
pub fn fn_0048ef40(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_d8e8);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_ef40, fn_0048ef40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x0048_ef40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0101_d8e8);
    }
}
