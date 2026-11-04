//! `(unplaced)/run_00507700` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00507700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float, `RET 4`): stores
/// `value` at +0xFC of `this` through the x87 stack (a signalling NaN comes
/// out quiet; the field is unnamed here).
pub fn fn_00507700(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0xfc, x87_quiet_bits(value.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00507700, fn_00507700(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_quieted() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.call(0x0050_7700, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0xfc), 2.5);
        e.call(0x0050_7700, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xfc), 0x7fc0_0001);
    }
}
