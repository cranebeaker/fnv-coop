//! `(unplaced)/run_00526ae0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00526ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x28, returned in ST0 (`FLD dword`, which quiets a
/// signalling NaN).
pub fn fn_00526ae0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x28))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0052_6ae0, fn_00526ae0(Ptr) -> f32)]
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    #[test]
    fn b020_00526ae0_loads_the_float() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x30);
        e.mem.set_f32(p + 0x28, 2.5);
        assert_eq!(e.call(0x0052_6ae0, &args![p]).f32(), 2.5);
        e.mem.set_u32(p + 0x28, 0x7f80_0001);
        assert_eq!(fn_00526ae0(&mut e, Ptr::new(p)).to_bits(), 0x7fc0_0001);
    }
}
