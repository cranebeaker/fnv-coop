//! `fallout/ai/run_00984ba0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `CombatGoal` vtable (RTTI).
const COMBAT_GOAL_VTABLE: u32 = 0x0108_d440;
/// `00680890` (`thiscall`, no stack words, no engine-map name): the constructor run on both
/// `CombatWorldStateList` members.
const WORLD_STATE_LIST_CONSTRUCT: u32 = 0x0068_0890;

// Translated from 00984ba0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; by its vtable `0108d440` the
/// `CombatGoal` constructor (RTTI)): stores the vtable, runs `00680890` on
/// `ActivationConditions` (+0x4) and `GoalStates` (+0xC), sets `eType` (+0x14) to
/// `goal_type` and `iGroupStrategyAssignment` (+0x18) to 0 (field names from the Xbox PDB
/// `CombatGoal`, 0x1c bytes as on PC). Answers `this`.
// Translation incomplete: 00984ba0 - the FS:[0] exception frame (unwinding the first list when the second constructor throws) is not translated
pub fn fn_00984ba0(e: &mut Engine, this: u32, goal_type: u32) -> u32 {
    e.mem.set_u32(this, COMBAT_GOAL_VTABLE);
    e.call(WORLD_STATE_LIST_CONSTRUCT, &args![this + 0x4]);
    e.call(WORLD_STATE_LIST_CONSTRUCT, &args![this + 0xc]);
    // CombatGoal::eType (Xbox PDB) +0x14
    e.mem.set_u32(this + 0x14, goal_type);
    // CombatGoal::iGroupStrategyAssignment (Xbox PDB) +0x18
    e.mem.set_u32(this + 0x18, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_4ba0, fn_00984ba0(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00984ba0_builds_a_combat_goal() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0x55);
        e.register(WORLD_STATE_LIST_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0098_4ba0, &[this, 3]).u32(), this);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == WORLD_STATE_LIST_CONSTRUCT)
            .collect();
        assert_eq!(
            calls,
            vec![
                (WORLD_STATE_LIST_CONSTRUCT, vec![this + 4]),
                (WORLD_STATE_LIST_CONSTRUCT, vec![this + 0xc]),
            ]
        );
        assert_eq!(e.mem.u32(this), COMBAT_GOAL_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), 3);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }
}
