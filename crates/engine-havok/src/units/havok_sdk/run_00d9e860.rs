//! `Havok SDK/run_00d9e860` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d9e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, no stack words): the 16-bit word 1 at `+6`, the vtables `010d40e0` at
/// `+0` and `010d40d0` at `+8` (`010ccf7c` is written first), zero at `+0xc`, the byte 0 at
/// `+0x10`, and three empty arrays (data 0, size 0, capacity word `0x80000000`) at `+0x14`,
/// `+0x20` and `+0x2c`. Returns `this`. The class is unconfirmed (the list of end-of-step
/// callbacks of `hkpEndOfStepCallbackUtil`).
pub fn fn_00d9e860(e: &mut Engine, this: Ptr) -> Ptr {
    let this = this.addr();
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this + 8, 0x010c_cf7c);
    e.mem.set_u32(this, 0x010d_40e0);
    e.mem.set_u32(this + 8, 0x010d_40d0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u8(this + 0x10, 0);
    for array in [0x14u32, 0x20, 0x2c] {
        e.mem.set_u32(this + array, 0);
        e.mem.set_u32(this + array + 4, 0);
        e.mem.set_u32(this + array + 8, 0x8000_0000);
    }
    Ptr::new(this)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_e860, fn_00d9e860(Ptr) -> Ptr)]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_constructor_makes_three_empty_arrays() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00d9_e860, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010d_40e0);
        assert_eq!(e.mem.u32(this + 8), 0x010d_40d0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        for array in [0x14u32, 0x20, 0x2c] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        // The bytes between the fields are untouched.
        assert_eq!(e.mem.u8(this + 0x11), 0xff);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u32(this + 0x38), 0xffff_ffff);
    }
}
