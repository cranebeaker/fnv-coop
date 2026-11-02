//! `(unplaced)/run_005076c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005076c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float setter (unnamed in the map; `thiscall`, `(value)`): `value` stored at
/// `this + 0xC0` (through the x87 stack, so a signalling NaN is stored quiet).
pub fn fn_005076c0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0xc0),
        x87_quiet_bits(value.to_bits()),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005076c0, fn_005076c0(Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_005076c0_stores_the_float_at_0xc0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.call(0x005076c0, &args![this, -1.5f32]);
        assert_eq!(e.mem.f32(this + 0xc0), -1.5);
        e.call(0x005076c0, &args![this, 0xff80_0002u32]);
        assert_eq!(e.mem.u32(this + 0xc0), 0xffc0_0002);
    }
}
