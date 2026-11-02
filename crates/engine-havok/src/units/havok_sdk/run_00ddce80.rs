//! `Havok SDK/run_00ddce80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{compare_flags, test_parity_even};

/// The `float` two values must differ by to count as different (`01017d00`, 0.001).
const SAME_DISTANCE: u32 = 0x0101_7d00;

// Translated from 00ddce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `out` (a pointer to a byte), `a`, `b` (pointers to records
/// whose `float` at `+8` is compared)). The difference `a.f8 - b.f8` is formed on the x87 stack
/// and stored as a `float`, then its absolute value, also stored as a `float`. Writes 0 at `out`
/// when that is below the `float` at `01017d00` (x87 compare), else 1 (a NaN gives 1), and answers
/// `out` (the hidden result pointer of a returned `hkBool`).
pub fn fn_00ddce80(e: &mut Engine, out: u32, a: u32, b: u32) -> u32 {
    let first = x87_load32(e.mem.u32(a + 8));
    let difference = x87_store32(x87_sub(first, x87_m32(e.mem.u32(b + 8))));
    let magnitude = x87_store32(x87_load32(difference).abs());
    let limit = x87_m32(e.mem.u32(SAME_DISTANCE));
    let flags = compare_flags(x87_load32(magnitude), limit);
    let different = test_parity_even(flags, 5);
    e.mem.set_u8(out, u8::from(different));
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ddce80, fn_00ddce80(u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(e: &mut Engine, value: f32) -> u32 {
        let at = e.mem.alloc(0x10);
        e.mem.set_f32(at + 8, value);
        at
    }

    #[test]
    fn values_closer_than_the_limit_give_zero_and_others_one() {
        let mut e = Engine::new();
        e.map(SAME_DISTANCE, 4);
        e.mem.set_f32(SAME_DISTANCE, 0.001);
        let out = e.mem.alloc(8);
        let one = record(&mut e, 1.0);
        let near = record(&mut e, 1.0005);
        let far = record(&mut e, 1.5);
        let nan = record(&mut e, f32::NAN);
        for (a, b, expected) in [
            (one, near, 0u8),
            (near, one, 0),
            (one, one, 0),
            (one, far, 1),
            (far, one, 1),
            (nan, one, 1),
            (one, nan, 1),
        ] {
            e.mem.set_u8(out, 7);
            assert_eq!(e.call(0x00dd_ce80, &args![out, a, b]).u32(), out);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    #[test]
    fn a_difference_exactly_at_the_limit_counts_as_different() {
        let mut e = Engine::new();
        e.map(SAME_DISTANCE, 4);
        e.mem.set_f32(SAME_DISTANCE, 0.5);
        let out = e.mem.alloc(8);
        let a = record(&mut e, 1.0);
        let b = record(&mut e, 1.5);
        e.call(0x00dd_ce80, &args![out, a, b]);
        assert_eq!(e.mem.u8(out), 1);
    }
}
