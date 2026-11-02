//! `(unplaced)/run_00aa1d10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base virtual table of the binary streams (`010a24b8`).
const BASE_VTABLE: u32 = 0x010a_24b8;

// Translated from 00aa1d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base class constructor of the binary streams (called by `NiMemStream`
/// `00aa2fb0`; `thiscall`, no stack word): stores the base virtual table, clears
/// the position (`+4`) and the read and write function pointers (`+8`, `+0xc`)
/// and answers `this`.
pub fn fn_00aa1d10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, BASE_VTABLE);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa1d10, fn_00aa1d10(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_clears_the_fields_and_stores_the_vtable() {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x24);
        e.mem.write(stream, &[0xcd; 0x24]);
        assert_eq!(e.call(0x00aa1d10, &args![stream]).u32(), stream);
        assert_eq!(e.mem.u32(stream), BASE_VTABLE);
        assert_eq!(e.mem.u32(stream + 4), 0);
        assert_eq!(e.mem.u32(stream + 8), 0);
        assert_eq!(e.mem.u32(stream + 0xc), 0);
        assert_eq!(e.mem.u32(stream + 0x10), 0xcdcd_cdcd);
    }
}
