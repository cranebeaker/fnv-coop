//! `(unplaced)/run_00ca9840` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ca9840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the object at `this + 4` is not null, stores the float
/// argument at +0x1c of it (`FLD`/`FSTP`, so a signalling NaN is stored quiet). The float
/// comes as its bit pattern (`RET 4`).
pub fn fn_00ca9840(e: &mut Engine, this: Ptr, value: u32) {
    let target = e.mem.u32(this.addr() + 4);
    if target != 0 {
        e.mem.set_u32(target + 0x1c, x87_quiet_bits(value));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ca_9840, fn_00ca9840(Ptr, u32))]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn stores_the_quieted_float_only_with_a_target() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x00ca_9840, &args![this, 2.5f32.to_bits()]);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, target);
        e.call(0x00ca_9840, &args![this, 2.5f32.to_bits()]);
        assert_eq!(e.mem.f32(target + 0x1c), 2.5);
        e.call(0x00ca_9840, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(target + 0x1c), 0x7fc0_0001);
    }
}
