//! `fallout shared/run_0048ee50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048ee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0xb8 of `this`, returned in `ST0` (`FLD`, which quiets
/// a signalling NaN; no stack word; the engine map gives it no name).
pub fn fn_0048ee50(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xb8))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_ee50, fn_0048ee50(Ptr) -> f32)]
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_0048ee50_loads_the_float_at_b8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xbc);
        e.mem.set_f32(this + 0xb8, 2.5);
        assert_eq!(e.call(0x0048_ee50, &args![this]).f32(), 2.5);
        e.mem.set_u32(this + 0xb8, 0x7f80_0001);
        let r = e.call(0x0048_ee50, &args![this]).f64();
        assert!(r.is_nan());
        assert_eq!((r as f32).to_bits(), 0x7fc0_0001);
    }
}
