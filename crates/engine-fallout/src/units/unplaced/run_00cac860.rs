//! `(unplaced)/run_00cac860` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cac860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers in `ST0` the fourth lane (`SHUFPS ..,0xff`) of the
/// 16-byte vector at `this + 0x1a0`, through a float local (`MOVSS`/`FLD`, so a signalling
/// NaN comes back quiet).
pub fn fn_00cac860(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x1a0 + 0xc))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ca_c860, fn_00cac860(Ptr) -> f32)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn answers_the_fourth_lane_quieted() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1b0);
        e.mem.set_f32(this + 0x1a0, 1.0);
        e.mem.set_f32(this + 0x1ac, -7.5);
        assert_eq!(e.call(0x00ca_c860, &args![this]).f32(), -7.5);
        e.mem.set_u32(this + 0x1ac, 0x7f80_0001);
        assert_eq!(
            e.call(0x00ca_c860, &args![this]).f32().to_bits(),
            0x7fc0_0001
        );
    }
}
