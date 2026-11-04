//! `(unplaced)/run_0088b030` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0088b030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at +0x114 (through the FPU; the engine map has no name).
pub fn fn_0088b030(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x114, x87_quiet_bits(value.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0088_b030, fn_0088b030(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_at_0x114() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x118);
        e.call(0x0088_b030, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0x114), 2.5);
        e.call(0x0088_b030, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x114), 0x7fc0_0001);
    }
}
