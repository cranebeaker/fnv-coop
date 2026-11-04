//! `(unplaced)/run_00642660` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00642660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: stores `value` (through the FPU) at row `row`, column
/// `column` of the 3x3 float matrix at `this` (`this + row * 0xc +
/// column * 4`).
pub fn fn_00642660(e: &mut Engine, this: Ptr, row: u32, column: u32, value: f32) {
    let address = row
        .wrapping_mul(0xc)
        .wrapping_add(this.addr())
        .wrapping_add(column.wrapping_mul(4));
    e.mem.set_u32(address, x87_quiet_bits(value.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0064_2660, fn_00642660(Ptr, u32, u32, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00642660_sets_one_entry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.call(0x0064_2660, &args![this, 2u32, 1u32, 1.5f32]);
        assert_eq!(e.mem.f32(this + 0x1c), 1.5);
        e.call(0x0064_2660, &args![this, 0u32, 0u32, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
    }
}
