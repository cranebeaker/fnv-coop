//! `NiMain/run_00a59b50` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec6040` (`ci_sqrt`: value in `ST0`, answer in `ST0`, so a leading `f64` word pair).
const SQUARE_ROOT: u32 = 0x00ec_6040;

/// The mantissa bits of the square root of the float whose bits are `bits`: `FLD`, the
/// library root, `FSTP` as a float, the low 23 bits.
fn root_mantissa(e: &mut Engine, bits: u32) -> u32 {
    let root = e.call(SQUARE_ROOT, &args![x87_load32(bits)]).f64();
    x87_quiet_bits(x87_store32(root)) & 0x007f_ffff
}

// Translated from 00a59b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00a59b50`; `cdecl`, no arguments): allocates 0x400
/// bytes (`00aa1070`) and fills two tables of 0x80 words with the mantissa bits of square
/// roots: entry `i` of the first (at `+0`) is for the float with bits `(0x3f80 | i) << 16`
/// (1.0 up to just under 2.0), entry `i` of the second (at `+0x200`) for the float with
/// bits `(0x4000 | i) << 16` (2.0 up to just under 4.0). Returns the block.
pub fn fn_00a59b50(e: &mut Engine) -> Ptr {
    let table = e.call(ALLOCATE_BLOCK, &args![0x400u32]).u32();
    for index in 0..0x80u32 {
        let low = root_mantissa(e, (index | 0x3f80) << 16);
        e.mem.set_u32(table + index * 4, low);
        let high = root_mantissa(e, (index | 0x4000) << 16);
        e.mem.set_u32(table + 0x200 + index * 4, high);
    }
    Ptr::new(table)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_9b50, fn_00a59b50() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_both_tables_with_the_square_root_mantissas() {
        let mut e = Engine::new();
        e.register_double(ALLOCATE_BLOCK, |e, a| {
            assert_eq!(a, &[0x400]);
            e.mem.alloc(0x400).into_ret()
        });
        let table = e.call(0x00a5_9b50, &args![]).u32();
        for index in 0..0x80u32 {
            let low = f32::from_bits((index | 0x3f80) << 16);
            let high = f32::from_bits((index | 0x4000) << 16);
            let expected_low = (f64::from(low).sqrt() as f32).to_bits() & 0x007f_ffff;
            let expected_high = (f64::from(high).sqrt() as f32).to_bits() & 0x007f_ffff;
            assert_eq!(e.mem.u32(table + index * 4), expected_low, "{index}");
            assert_eq!(
                e.mem.u32(table + 0x200 + index * 4),
                expected_high,
                "{index}"
            );
        }
        // sqrt(1.0) = 1.0 has no mantissa bits; sqrt(2.0) = 1.4142135 has.
        assert_eq!(e.mem.u32(table), 0);
        assert_eq!(e.mem.u32(table + 0x200), 0x3504f3);
    }
}
