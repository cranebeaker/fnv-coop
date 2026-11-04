//! `(unplaced)/run_00aa1d60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa1d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverses the byte order of `count` 8-byte values at `values` in place
/// (cdecl; called by `00aa1de0`'s endian swap for 8-byte items).
pub fn fn_00aa1d60(e: &mut Engine, values: u32, count: u32) {
    let mut at = values;
    for _ in 0..count {
        for (low, high) in [(0u32, 7u32), (1, 6), (2, 5), (3, 4)] {
            let upper = e.mem.u8(at + high);
            let lower = e.mem.u8(at + low);
            e.mem.set_u8(at + low, upper);
            e.mem.set_u8(at + high, lower);
        }
        at = at.wrapping_add(8);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_1d60, fn_00aa1d60(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eight_byte_values_are_reversed() {
        let mut e = Engine::new();
        let values = e.mem.alloc(24);
        e.mem.write(values, &(0u8..24).collect::<Vec<_>>());
        e.call(0x00aa_1d60, &[values, 2]);
        assert_eq!(
            e.mem.bytes(values, 24),
            vec![
                7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 16, 17, 18, 19, 20, 21, 22,
                23
            ]
        );
        e.call(0x00aa_1d60, &[values, 0]);
        assert_eq!(e.mem.u8(values), 7);
    }
}
