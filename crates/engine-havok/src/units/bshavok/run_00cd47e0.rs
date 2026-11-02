//! `BSHavok/run_00cd47e0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable stored at the start.
const VTABLE: u32 = 0x010c_b3f0;

// Translated from 00cd47e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cd47e0` (unnamed in the Xbox PDB): a constructor with no argument besides `this`.
/// Stores the 16-bit word 1 at `+6`, the vtable 0x010cb3f0 at `+0` and the byte 1 at `+8`,
/// and answers `this`. The class is not named by the exe data read so far.
pub fn fn_00cd47e0(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, VTABLE);
    e.mem.set_u8(this + 8, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd47e0, fn_00cd47e0(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_count_the_vtable_and_the_flag_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00cd47e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u8(this + 8), 1);
    }
}
