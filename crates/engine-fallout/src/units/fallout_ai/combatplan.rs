//! `fallout/ai/combat/combatplan.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The debug print `005b5e40` (`cdecl`: a format and its words) and
/// `"COMBAT: Could not find plan step %08X in CombatPlan::GetIndexForStep()"` (`0108dba8`).
const DEBUG_PRINT: u32 = 0x005b_5e40;
const STEP_NOT_FOUND_FORMAT: u32 = 0x0108_dba8;
/// `0059bb30` (`thiscall`): the action of a plan step (engine map: `D3DTexture_LockRect`, a
/// folded body).
const STEP_ACTION: u32 = 0x0059_bb30;

// Translated from 00994860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether one of the `count` (`+4`) steps of
/// the array at `+0` answers `action` through `0059bb30` (called on the step's first word).
pub fn fn_00994860(e: &mut Engine, this: Ptr, action: u32) -> bool {
    let me = this.addr();
    let mut i = 0u32;
    while i < e.mem.u32(me.wrapping_add(4)) {
        let steps = e.mem.u32(me);
        let step = e.mem.u32(steps.wrapping_add(i.wrapping_mul(4)));
        let first = e.mem.u32(step);
        if e.call(STEP_ACTION, &args![first]).u32() == action {
            return true;
        }
        i = i.wrapping_add(1);
    }
    false
}

// Translated from 00981940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatPlan::ContainsAction` (Xbox PDB; `thiscall`, `RET 4`): false without a step list at
/// `+0xa0`, otherwise `00994860(action)` on it.
pub fn combat_plan_contains_action(e: &mut Engine, this: Ptr, action: u32) -> bool {
    let steps = e.mem.u32(this.addr().wrapping_add(0xa0));
    if steps == 0 {
        return false;
    }
    fn_00994860(e, Ptr::new(steps), action)
}

// Translated from 00994270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatPlan::GetIndexForStep` (Xbox PDB; `thiscall`, `RET 4`): the index of `step` among
/// the `count` (`+4`) words of the array at `+0`; when it is not there, prints
/// `"COMBAT: Could not find plan step %08X in CombatPlan::GetIndexForStep()"` and answers 0.
pub fn combat_plan_get_index_for_step(e: &mut Engine, this: Ptr, step: u32) -> u32 {
    let me = this.addr();
    let mut i = 0u32;
    while i < e.mem.u32(me.wrapping_add(4)) {
        let steps = e.mem.u32(me);
        if e.mem.u32(steps.wrapping_add(i.wrapping_mul(4))) == step {
            return i;
        }
        i = i.wrapping_add(1);
    }
    e.call(DEBUG_PRINT, &args![STEP_NOT_FOUND_FORMAT, step]);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_4860, fn_00994860(Ptr, u32) -> bool),
        entry!(0x0098_1940, combat_plan_contains_action(Ptr, u32) -> bool),
        entry!(0x0099_4270, combat_plan_get_index_for_step(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An array of `steps` (each a block whose first word is its action) at `+0`, its count at
    /// `+4`.
    fn plan_steps(e: &mut Engine, actions: &[u32]) -> (u32, Vec<u32>) {
        let base = e.mem.alloc(4 * actions.len() as u32 + 4);
        let mut steps = vec![];
        for (i, action) in actions.iter().enumerate() {
            let step = e.mem.alloc(4);
            e.mem.set_u32(step, *action);
            e.mem.set_u32(base + 4 * i as u32, step);
            steps.push(step);
        }
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, base);
        e.mem.set_u32(array + 4, actions.len() as u32);
        (array, steps)
    }

    #[test]
    fn a_step_list_finds_an_action_through_each_step() {
        let mut e = Engine::new();
        e.register(STEP_ACTION, |_, a| (a[0] + 0x100).into_ret());
        let (array, _) = plan_steps(&mut e, &[1, 2, 3]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_4860, &args![array, 0x102u32]).bool());
        assert_eq!(calls(&e, STEP_ACTION), vec![vec![1], vec![2]]);
        assert!(!e.call(0x0099_4860, &args![array, 0x104u32]).bool());
    }

    #[test]
    fn contains_action_needs_a_step_list() {
        let mut e = Engine::new();
        e.register(STEP_ACTION, |_, a| a[0].into_ret());
        let plan = e.mem.alloc(0xa4);
        assert!(!e.call(0x0098_1940, &args![plan, 0u32]).bool());
        let (array, _) = plan_steps(&mut e, &[7]);
        e.mem.set_u32(plan + 0xa0, array);
        assert!(e.call(0x0098_1940, &args![plan, 7u32]).bool());
        assert!(!e.call(0x0098_1940, &args![plan, 8u32]).bool());
    }

    #[test]
    fn the_index_of_a_step_or_a_message_and_zero() {
        let mut e = Engine::new();
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        let (array, steps) = plan_steps(&mut e, &[1, 2, 3]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_4270, &args![array, steps[2]]).u32(), 2);
        assert!(calls(&e, DEBUG_PRINT).is_empty());
        assert_eq!(e.call(0x0099_4270, &args![array, 0x1234u32]).u32(), 0);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![STEP_NOT_FOUND_FORMAT, 0x1234]]
        );
    }
}
