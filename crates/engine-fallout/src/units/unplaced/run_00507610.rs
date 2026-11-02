//! `(unplaced)/run_00507610` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00507610 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float setter (unnamed in the map; `thiscall`, `(value)`): `value` stored at
/// `this + 0xB0` (through the x87 stack, so a signalling NaN is stored quiet).
pub fn fn_00507610(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0xb0),
        x87_quiet_bits(value.to_bits()),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00507610, fn_00507610(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00507610_stores_the_float_at_0xb0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb4);
        e.call(0x00507610, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0xb0), 2.5);
        e.call(0x00507610, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xb0), 0x7fc0_0001);
    }
}
