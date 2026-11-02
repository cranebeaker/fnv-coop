//! `fallout/ai/procedure.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The procedure counts, one signed byte per package type (0x31 entries).
const PROCEDURE_COUNTS: u32 = 0x011a_3fbc;

// Translated from 0096a820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Procedure::GetProcedureCount` (Xbox PDB name in the engine map): the
/// signed byte for `kind` in the table at `011a3fbc`, or -1 outside 0..0x31.
pub fn procedure_get_procedure_count(e: &mut Engine, kind: i32) -> i32 {
    if !(0..0x31).contains(&kind) {
        return -1;
    }
    e.global::<i8>(PROCEDURE_COUNTS.wrapping_add(kind as u32)) as i32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0096_a820, procedure_get_procedure_count(i32) -> i32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_procedure_count_reads_the_signed_table() {
        let mut e = Engine::new();
        e.map(PROCEDURE_COUNTS, 0x31);
        e.set_global(PROCEDURE_COUNTS, 3u8);
        e.set_global(PROCEDURE_COUNTS + 0x30, 0xfeu8);
        assert_eq!(e.call(0x0096_a820, &args![0u32]).i32(), 3);
        assert_eq!(e.call(0x0096_a820, &args![0x30u32]).i32(), -2);
        assert_eq!(e.call(0x0096_a820, &args![0x31u32]).i32(), -1);
        assert_eq!(e.call(0x0096_a820, &args![-1i32]).i32(), -1);
    }
}
