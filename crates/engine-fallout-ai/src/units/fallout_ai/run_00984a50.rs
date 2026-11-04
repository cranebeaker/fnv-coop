//! `fallout/ai/run_00984a50` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The table of the 16 `CombatGoal`s at `011f17a0`: made one per goal kind
//! by `CombatGoal::CreateCombatGoal` (`00984490`) and deleted again.

#[allow(unused_imports)]
use crate::prelude::*;

/// The goal table (16 pointers).
const GOALS: u32 = 0x011f_17a0;
/// `CombatGoal::CreateCombatGoal` (`00984490`, Xbox PDB; `cdecl(kind)`).
const CREATE_COMBAT_GOAL: u32 = 0x0098_4490;
/// The `CombatGoal` destructor body (`00984290`, `thiscall`).
const COMBAT_GOAL_DESTRUCT: u32 = 0x0098_4290;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00984a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the goal table: entry `i` (0..16) is `CreateCombatGoal(i)`.
pub fn fn_00984a50(e: &mut Engine) {
    for kind in 0..0x10u32 {
        let goal = e.call(CREATE_COMBAT_GOAL, &args![kind]).u32();
        e.set_global(GOALS + 4 * kind, goal);
    }
}

// Translated from 00984a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every goal in the table ([`fn_00984af0`] with 1); the entries
/// are left as they are.
pub fn fn_00984a90(e: &mut Engine) {
    for kind in 0..0x10u32 {
        let goal: u32 = e.global(GOALS + 4 * kind);
        if goal != 0 {
            fn_00984af0(e, Ptr::new(goal), 1);
        }
    }
}

// Translated from 00984af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGoal` scalar deleting destructor (unnamed in the engine map; by
/// its body): the destructor body (`00984290`), then `operator delete` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_00984af0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(COMBAT_GOAL_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_4a50, fn_00984a50()),
        entry!(0x0098_4a90, fn_00984a90()),
        entry!(0x0098_4af0, fn_00984af0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn the_table_is_filled_and_emptied() {
        let mut e = Engine::new();
        e.map(0x011f_1000, 0x1000);
        // Kind 5 makes no goal.
        e.register(CREATE_COMBAT_GOAL, |_, a| {
            if a[0] == 5 {
                0u32.into_ret()
            } else {
                (0x7000 + 0x10 * a[0]).into_ret()
            }
        });
        e.register(COMBAT_GOAL_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_4a50, &args![]);
        assert_eq!(calls(&e, CREATE_COMBAT_GOAL).len(), 16);
        assert_eq!(e.global::<u32>(GOALS), 0x7000);
        assert_eq!(e.global::<u32>(GOALS + 0x3c), 0x70f0);
        assert_eq!(e.global::<u32>(GOALS + 0x14), 0);
        e.call_log = Some(vec![]);
        e.call(0x0098_4a90, &args![]);
        assert_eq!(calls(&e, COMBAT_GOAL_DESTRUCT).len(), 15);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 15);
        assert_eq!(e.global::<u32>(GOALS), 0x7000);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(COMBAT_GOAL_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0098_4af0, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0098_4af0, &args![0x5000u32, 3u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x5000]]);
        assert_eq!(calls(&e, COMBAT_GOAL_DESTRUCT).len(), 2);
    }
}
