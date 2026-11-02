//! `BSHavok/run_00cd4260` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable stored at the start.
const VTABLE: u32 = 0x010c_b398;

// Translated from 00cd4260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cd4260` (unnamed in the Xbox PDB): a constructor with no argument besides `this`.
/// Stores the 16-bit word 1 at `+6` and the vtable 0x010c_b398 at `+0`, and answers `this`.
/// The class is not named by the exe data read so far.
pub fn fn_00cd4260(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd4260, fn_00cd4260(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_count_and_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00cd4260, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
    }
}
