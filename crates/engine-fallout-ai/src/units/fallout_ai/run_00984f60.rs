//! `fallout/ai/run_00984f60` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00984f60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): stores `value` into the word
/// at `this` +0x18 (`CombatGoal::iGroupStrategyAssignment` (Xbox PDB) on a `CombatGoal`;
/// the class of `this` is unconfirmed for its callers).
pub fn fn_00984f60(e: &mut Engine, this: u32, value: u32) {
    e.mem.set_u32(this + 0x18, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_4f60, fn_00984f60(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00984f60_stores_the_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.call(0x0098_4f60, &[this, 0x1234_5678]);
        assert_eq!(e.mem.u32(this + 0x18), 0x1234_5678);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }
}
