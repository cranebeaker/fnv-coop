//! `BSHavok/run_00cf43b0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The floats stored at `+0xC`, `+0x10` and `+0x14`.
const VALUE_AT_0C: u32 = 0x010c_b790;
const VALUE_AT_10: u32 = 0x0108_cb00;
const VALUE_AT_14: u32 = 0x0103_0ff0;
/// The vtable stored at the start.
const VTABLE: u32 = 0x010c_c0ec;

// Translated from 00cf43b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cf43b0` (unnamed in the Xbox PDB): a constructor with no argument besides `this`.
/// Stores (floats through the x87 stack): the float at 0x010cb790 at `+0xC`, the float at
/// 0x0108cb00 at `+0x10`, 1.0 at `+0x18` and the float at 0x01030ff0 at `+0x14`; the 16-bit
/// word 1 at `+6`, the byte 2 at `+8`, the byte 0 at `+0x1c` and the vtable 0x010cc0ec at
/// `+0`. Answers `this`. The class is not named by the exe data read so far.
pub fn fn_00cf43b0(e: &mut Engine, this: u32) -> u32 {
    let first = x87_quiet_bits(e.mem.u32(VALUE_AT_0C));
    e.mem.set_u32(this + 0xc, first);
    e.mem.set_u16(this + 6, 1);
    let second = x87_quiet_bits(e.mem.u32(VALUE_AT_10));
    e.mem.set_u32(this + 0x10, second);
    e.mem.set_u8(this + 8, 2);
    e.mem.set_f32(this + 0x18, 1.0);
    let third = x87_quiet_bits(e.mem.u32(VALUE_AT_14));
    e.mem.set_u32(this + 0x14, third);
    e.mem.set_u8(this + 0x1c, 0);
    e.mem.set_u32(this, VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf43b0, fn_00cf43b0(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_constants_the_flags_and_the_vtable() {
        let mut e = Engine::new();
        for (address, bits) in [
            (VALUE_AT_0C, 0x4000_0000u32),
            (VALUE_AT_10, 0x7f80_0001),
            (VALUE_AT_14, 0x3e80_0000),
        ] {
            e.map(address, 4);
            e.mem.set_u32(address, bits);
        }
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 0x1c, 7);
        assert_eq!(e.call(0x00cf43b0, &args![this]).u32(), this);
        assert_eq!(e.mem.f32(this + 0xc), 2.0);
        assert_eq!(e.mem.u32(this + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x14), 0.25);
        assert_eq!(e.mem.f32(this + 0x18), 1.0);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u8(this + 8), 2);
        assert_eq!(e.mem.u8(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this), VTABLE);
    }
}
