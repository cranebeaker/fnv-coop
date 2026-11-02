//! `(unplaced)/run_0088b0f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` read (+0x548 of `this`).
const FLOAT_548: u32 = 0x548;

// Translated from 0088b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x548 of `this`, in ST0 (`FLD`; unnamed in the engine
/// map).
pub fn fn_0088b0f0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + FLOAT_548))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0088_b0f0, fn_0088b0f0(Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_float_at_0x548() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x550);
        e.mem.set_f32(this + FLOAT_548, 0.75);
        assert_eq!(e.call(0x0088_b0f0, &args![this]).f64(), 0.75);
        e.mem.set_u32(this + FLOAT_548, 0xff80_0001);
        let nan = e.call(0x0088_b0f0, &args![this]).f64();
        assert_eq!(x87_store32(nan), 0xffc0_0001);
    }
}
