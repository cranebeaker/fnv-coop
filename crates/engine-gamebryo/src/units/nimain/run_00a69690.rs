//! `NiMain/run_00a69690` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a69690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clamps the three floats at `this` to at most 1.0 (the engine map has no
/// name): each one greater than 1.0 becomes 1.0; a NaN is left alone (the
/// x87 comparison is unordered).
pub fn fn_00a69690(e: &mut Engine, this: Ptr) {
    for i in 0..3 {
        let address = this.addr() + 4 * i;
        if 1.0 < e.mem.f32(address) {
            e.mem.set_f32(address, 1.0);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_9690, fn_00a69690(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_floats_are_clamped_to_one() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_f32(this, 1.5);
        e.mem.set_f32(this + 4, 0.25);
        e.mem.set_u32(this + 8, 0x7fc0_0000);
        e.mem.set_f32(this + 12, 7.0);
        e.call(0x00a6_9690, &args![this]);
        assert_eq!(e.mem.f32(this), 1.0);
        assert_eq!(e.mem.f32(this + 4), 0.25);
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0000);
        assert_eq!(e.mem.f32(this + 12), 7.0);
        e.mem.set_f32(this + 8, 3.0);
        e.call(0x00a6_9690, &args![this]);
        assert_eq!(e.mem.f32(this + 8), 1.0);
    }
}
