//! `BSHavok/run_00cd3f90` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float stored at `+8`.
const DEFAULT_VALUE: u32 = 0x0101_3970;
/// The vtable stored at the start.
const VTABLE: u32 = 0x010c_b36c;

// Translated from 00cd3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cd3f90` (unnamed in the Xbox PDB): a constructor with no argument besides `this`.
/// Stores the float at 0x01013970 (through the x87 stack) at `+8`, the 16-bit word 1 at `+6`
/// and the vtable 0x010cb36c at `+0`, and answers `this`. The class is not named by the exe
/// data read so far.
pub fn fn_00cd3f90(e: &mut Engine, this: u32) -> u32 {
    let value = x87_quiet_bits(e.mem.u32(DEFAULT_VALUE));
    e.mem.set_u32(this + 8, value);
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd3f90, fn_00cd3f90(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_default_value_the_count_and_the_vtable() {
        let mut e = Engine::new();
        e.map(DEFAULT_VALUE, 4);
        e.mem.set_u32(DEFAULT_VALUE, 0x7f80_0001);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00cd_3f90, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE);
    }
}
