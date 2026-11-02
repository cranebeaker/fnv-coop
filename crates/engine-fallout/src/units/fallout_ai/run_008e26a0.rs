//! `fallout/ai/run_008e26a0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` 0.0 the float is compared with (`.rdata`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

// Translated from 008e26a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0xE0, or 0.0 when it is below zero (`FCOMP 0.0; TEST AH,5;
/// JP`: a NaN is answered as it is, quiet).
pub fn fn_008e26a0(e: &mut Engine, this: Ptr) -> f32 {
    let bits = e.mem.u32(this.addr() + 0xe0);
    if x87_load32(bits) < e.mem.f64(ZERO_DOUBLE) {
        0.0
    } else {
        f32::from_bits(x87_quiet_bits(bits))
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008e_26a0, fn_008e26a0(Ptr) -> f32)]
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_008e26a0_clamps_below_zero() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE, 8);
        let this = e.mem.alloc(0xe4);
        e.mem.set_f32(this + 0xe0, 2.5);
        assert_eq!(e.call(0x008e_26a0, &[this]).f32(), 2.5);
        e.mem.set_f32(this + 0xe0, -2.5);
        assert_eq!(e.call(0x008e_26a0, &[this]).f32(), 0.0);
        e.mem.set_u32(this + 0xe0, 0xff80_0001);
        assert!(e.call(0x008e_26a0, &[this]).f32().is_nan());
    }
}
