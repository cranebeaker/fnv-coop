//! `(unplaced)/run_00aa1d30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base virtual table of the binary streams (`010a24b8`).
const BASE_VTABLE: u32 = 0x010a_24b8;

// Translated from 00aa1d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base class destructor of the binary streams (called by `NiMemStream`
/// `00aa2d30`; `thiscall`, no stack word): stores the base virtual table back.
/// Returns nothing (`EAX` is not set).
pub fn fn_00aa1d30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa1d30, fn_00aa1d30(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_destructor_stores_the_base_vtable() {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x24);
        e.mem.set_u32(stream, 0x010a_256c);
        e.mem.set_u32(stream + 4, 77);
        e.call(0x00aa1d30, &args![stream]);
        assert_eq!(e.mem.u32(stream), BASE_VTABLE);
        assert_eq!(e.mem.u32(stream + 4), 77);
    }
}
