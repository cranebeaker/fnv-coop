//! `Havok SDK/run_00d86220` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `float` constants the constructor stores: `010cb790` (at `+0xc`) and `0108cb00` (at
/// `+0x10`).
const FIRST_FLOAT: u32 = 0x010c_b790;
const SECOND_FLOAT: u32 = 0x0108_cb00;
/// The vtable the constructor installs.
const VTABLE: u32 = 0x010d_3838;

// Translated from 00d86220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a motor-like record (the bshavok `run_00cf45d0.rs` calls it as the base): `+6`
/// the 16-bit word 1, `+8` the byte 0 and then 3, the two floats at `010cb790` and `0108cb00` at
/// `+0xc` and `+0x10` (each through `FLD`/`FSTP`, which quiets a signalling NaN), zero floats at
/// `+0x14` and `+0x18`, and the vtable at `010d3838`. Returns `this`. The class is unconfirmed.
pub fn fn_00d86220(e: &mut Engine, this: Ptr) -> Ptr {
    let this = this.addr();
    let first = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u8(this + 8, 0);
    e.mem.set_u32(this + 0xc, first);
    let second = x87_quiet_bits(e.mem.u32(SECOND_FLOAT));
    e.mem.set_u32(this, VTABLE);
    e.mem.set_u32(this + 0x10, second);
    e.mem.set_u8(this + 8, 3);
    e.mem.set_f32(this + 0x14, 0.0);
    e.mem.set_f32(this + 0x18, 0.0);
    Ptr::new(this)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d8_6220, fn_00d86220(Ptr) -> Ptr)]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn constructor_fills_the_record_and_quiets_the_float_constants() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(FIRST_FLOAT, 0x7f80_0001u32);
        e.set_global(SECOND_FLOAT, 0x3f00_0000u32);
        let this = e.mem.alloc(0x20);
        for offset in 0..8 {
            e.mem.set_u32(this + 4 * offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00d8_6220, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u8(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x10), 0x3f00_0000);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        // The bytes around the stored fields are untouched.
        assert_eq!(e.mem.u8(this + 9), 0xff);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u32(this + 0x1c), 0xffff_ffff);
    }
}
