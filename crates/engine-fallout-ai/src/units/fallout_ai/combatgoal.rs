//! `fallout/ai/combat/combatgoal.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CombatGoal::CreateCombatGoal` (the factory of the 16 goal kinds) and the
//! goal subclasses' constructors.

#[allow(unused_imports)]
use crate::prelude::*;

// The `CombatGoal` subclasses' constructors all have one shape (read from
// each one's disassembly): the `CombatGoal` base constructor with the kind
// the factory passes (`00984ba0`), the class's own vtable (each named by its
// RTTI locator), then pairs of words added to the goal's lists through
// `0097ade0` (list +0x04) and `00984390` (+0x0C), and for some a word given to
// `00984f60`. What the words mean is not confirmed from the exe.

/// Every goal object is 0x1C bytes.
const GOAL_SIZE: u32 = 0x1c;
/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The debug print `005b5e40` (`cdecl(format, ...)`) and
/// `"COMBAT: Could not create combat goal of type %i"` (`0108d448`).
const DEBUG_PRINT: u32 = 0x005b_5e40;
const CREATE_FAILED_FORMAT: u32 = 0x0108_d448;

/// A goal constructor of this file (`thiscall(kind)`).
type GoalConstructor = fn(&mut Engine, Ptr, u32) -> Ptr;

/// The constructor of each kind (the jump table at `00984a10`, as arms).
fn goal_constructor(kind: u32) -> Option<GoalConstructor> {
    let constructor: GoalConstructor = match kind {
        0 => fn_00985390,  // CombatGoalStayInCombatArea
        1 => fn_00984e60,  // CombatGoalUseCombatItem
        2 => fn_00984d40,  // CombatGoalFlee
        3 => fn_00984cb0,  // CombatGoalAvoidThreats
        4 => fn_00984dd0,  // CombatGoalActivateCombatItem
        5 => fn_00985080,  // CombatGoalReachDestination
        6 => fn_00985180,  // CombatGoalGuard
        7 => fn_00985200,  // CombatGoalCluster
        8 => fn_00985100,  // CombatGoalAdvance
        9 => fn_00985000,  // CombatGoalAmbush
        10 => fn_00984f80, // CombatGoalFallBack
        11 => fn_00984ee0, // CombatGoalFlank
        12 => fn_00984c20, // CombatGoalFindTarget
        13 => fn_00985300, // CombatGoalFlushTarget
        14 => fn_00984b20, // CombatGoalKillTarget
        15 => fn_00985280, // CombatGoalHideFromTarget
        _ => return None,
    };
    Some(constructor)
}

// Translated from 00984490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984490 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoal::CreateCombatGoal` (Xbox PDB; `cdecl(kind)`): for kinds
/// 0..=15 allocates 0x1C bytes and runs that kind constructor with `kind`
/// (the class names are the RTTI names of the vtables they set); returns the
/// goal, or null when the allocation fails. Any other kind prints
/// `"COMBAT: Could not create combat goal of type %i"` (`005b5e40`) and
/// returns null.
pub fn combat_goal_create_combat_goal(e: &mut Engine, kind: u32) -> Ptr {
    let Some(constructor) = goal_constructor(kind) else {
        e.call(DEBUG_PRINT, &args![CREATE_FAILED_FORMAT, kind]);
        return Ptr::NULL;
    };
    let block = e.call(OPERATOR_NEW, &args![GOAL_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    constructor(e, Ptr::new(block), kind)
}

// Translated from 00984b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984b20 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalKillTarget` constructor (by the vtable it sets, `0108d47c`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984b20(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d47c);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 00984c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984c20 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalFindTarget` constructor (by the vtable it sets, `0108d488`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984c20(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d488);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x10u32]);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0x3u32, 0x0u32]);
    this
}

// Translated from 00984cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984cb0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalAvoidThreats` constructor (by the vtable it sets, `0108d494`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984cb0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d494);
    e.call(0x0097_ade0, &args![this, 0xeu32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xeu32, 0x0u32]);
    this
}

// Translated from 00984d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984d40 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalFlee` constructor (by the vtable it sets, `0108d4a0`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984d40(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4a0);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0x1u32, 0x8u32]);
    e.call(0x0097_ade0, &args![this, 0x1u32, 0x7u32]);
    e.call(0x0098_4390, &args![this, 0xfu32, 0x0u32]);
    this
}

// Translated from 00984dd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984dd0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalActivateCombatItem` constructor (by the vtable it sets, `0108d4ac`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984dd0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4ac);
    e.call(0x0097_ade0, &args![this, 0x16u32, 0xffff_0016u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xdu32, 0xffff_0016u32]);
    this
}

// Translated from 00984e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984e60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalUseCombatItem` constructor (by the vtable it sets, `0108d4b8`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984e60(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4b8);
    e.call(0x0097_ade0, &args![this, 0x16u32, 0xffff_0017u32]);
    e.call(0x0098_4390, &args![this, 0x10u32, 0xffff_0017u32]);
    this
}

// Translated from 00984ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984ee0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalFlank` constructor (by the vtable it sets, `0108d4c4`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984ee0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4c4);
    e.call(0x0097_ade0, &args![this, 0x13u32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x4u32]);
    e.call(0x0098_4f60, &args![this, 0x1u32]);
    this
}

// Translated from 00984f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984f80 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalFallBack` constructor (by the vtable it sets, `0108d4d0`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00984f80(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4d0);
    e.call(0x0097_ade0, &args![this, 0x13u32, 0x2u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x5u32]);
    e.call(0x0098_4f60, &args![this, 0x2u32]);
    this
}

// Translated from 00985000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985000 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalAmbush` constructor (by the vtable it sets, `0108d4dc`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985000(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4dc);
    e.call(0x0097_ade0, &args![this, 0x13u32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0x11u32, 0xfu32]);
    e.call(0x0098_4f60, &args![this, 0x3u32]);
    this
}

// Translated from 00985080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985080 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalReachDestination` constructor (by the vtable it sets, `0108d4e8`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985080(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4e8);
    e.call(0x0097_ade0, &args![this, 0x2u32, 0x7u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x7u32]);
    this
}

// Translated from 00985100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985100 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalAdvance` constructor (by the vtable it sets, `0108d4f4`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985100(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d4f4);
    e.call(0x0097_ade0, &args![this, 0x13u32, 0x4u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x6u32]);
    e.call(0x0098_4f60, &args![this, 0x4u32]);
    this
}

// Translated from 00985180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985180 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalGuard` constructor (by the vtable it sets, `0108d500`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985180(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d500);
    e.call(0x0097_ade0, &args![this, 0x2u32, 0x8u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x8u32]);
    this
}

// Translated from 00985200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985200 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalCluster` constructor (by the vtable it sets, `0108d50c`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985200(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d50c);
    e.call(0x0097_ade0, &args![this, 0x2u32, 0x9u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x9u32]);
    this
}

// Translated from 00985280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985280 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalHideFromTarget` constructor (by the vtable it sets, `0108d518`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985280(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d518);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x11u32, 0x3cu32]);
    this
}

// Translated from 00985300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985300 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalFlushTarget` constructor (by the vtable it sets, `0108d524`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985300(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d524);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x9u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x9u32, 0x1u32]);
    this
}

// Translated from 00985390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00985390 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoalStayInCombatArea` constructor (by the vtable it sets, `0108d530`): the base
/// constructor `00984ba0` with `kind`, the vtable, then the list entries below
/// (see the note above `GOAL_SIZE`). Returns `this`.
pub fn fn_00985390(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0098_4ba0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_d530);
    e.call(0x0097_ade0, &args![this, 0x2u32, 0x10u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x10u32]);
    this
}

// ---- Batch n023 ----------------------------------------------------------------------------

/// `00717e50` (`thiscall` on a goal): the first node of its condition list; `006815c0`
/// (`thiscall` on a node): the address of the node's item; `0062d990` (`thiscall` on a node):
/// the next node.
const LIST_FIRST: u32 = 0x0071_7e50;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0062_d990;
/// `00559450` and `00726070` (`thiscall` on an item): its kind and its value.
const ITEM_KIND: u32 = 0x0055_9450;
const ITEM_VALUE: u32 = 0x0072_6070;
/// `CombatWorldStateArray::IsTrue` (engine-map name of `009aadc0`; `thiscall`, `RET 8`).
const WORLD_STATE_IS_TRUE: u32 = 0x009a_adc0;

// Translated from 00984470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the word at `this + 0x3fff8 + 4 * index`
/// (address arithmetic wraps as the exe's does).
pub fn fn_00984470(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let address = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(0x3fff8);
    e.mem.u32(address)
}

// Translated from 009843b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGoal::IsActive` (Xbox PDB; `thiscall`, `RET 4`): walks the goal's conditions
/// (`00717e50`, `006815c0`, `0062d990`) until a node without an item; each item's kind
/// (`00559450`) and value (`00726070`) are tested on `state`: kind 0x15 fails when
/// `00984470(value)` is non-zero, kind 0x16 when it is zero, any other kind when
/// `CombatWorldStateArray::IsTrue(kind, value)` answers false. True when nothing failed.
pub fn combat_goal_is_active(e: &mut Engine, this: Ptr, state: Ptr) -> bool {
    let mut node = e.call(LIST_FIRST, &args![this]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        let kind = e.call(ITEM_KIND, &args![item]).u32();
        let value = e.call(ITEM_VALUE, &args![item]).u32();
        if kind == 0x15 {
            if fn_00984470(e, state, value) != 0 {
                return false;
            }
        } else if kind == 0x16 {
            if fn_00984470(e, state, value) == 0 {
                return false;
            }
        } else if !e
            .call(WORLD_STATE_IS_TRUE, &args![state, kind, value])
            .bool()
        {
            return false;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_4470, fn_00984470(Ptr, u32) -> u32),
        entry!(0x0098_43b0, combat_goal_is_active(Ptr, Ptr) -> bool),
        entry!(0x0098_4490, combat_goal_create_combat_goal(u32) -> Ptr),
        entry!(0x0098_4b20, fn_00984b20(Ptr, u32) -> Ptr),
        entry!(0x0098_4c20, fn_00984c20(Ptr, u32) -> Ptr),
        entry!(0x0098_4cb0, fn_00984cb0(Ptr, u32) -> Ptr),
        entry!(0x0098_4d40, fn_00984d40(Ptr, u32) -> Ptr),
        entry!(0x0098_4dd0, fn_00984dd0(Ptr, u32) -> Ptr),
        entry!(0x0098_4e60, fn_00984e60(Ptr, u32) -> Ptr),
        entry!(0x0098_4ee0, fn_00984ee0(Ptr, u32) -> Ptr),
        entry!(0x0098_4f80, fn_00984f80(Ptr, u32) -> Ptr),
        entry!(0x0098_5000, fn_00985000(Ptr, u32) -> Ptr),
        entry!(0x0098_5080, fn_00985080(Ptr, u32) -> Ptr),
        entry!(0x0098_5100, fn_00985100(Ptr, u32) -> Ptr),
        entry!(0x0098_5180, fn_00985180(Ptr, u32) -> Ptr),
        entry!(0x0098_5200, fn_00985200(Ptr, u32) -> Ptr),
        entry!(0x0098_5280, fn_00985280(Ptr, u32) -> Ptr),
        entry!(0x0098_5300, fn_00985300(Ptr, u32) -> Ptr),
        entry!(0x0098_5390, fn_00985390(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;
    type Case = (u32, u32, Calls);

    const DOUBLED: [u32; 4] = [0x0098_4ba0, 0x0097_ade0, 0x0098_4390, 0x0098_4f60];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in DOUBLED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        e
    }

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
    fn create_runs_the_kind_constructor() {
        let mut e = engine();
        // Each kind's constructor sets its own vtable.
        let vtables: Vec<u32> = (0..16u32)
            .map(|kind| {
                e.call_log = Some(vec![]);
                let goal = e.call(0x0098_4490, &args![kind]).u32();
                assert_ne!(goal, 0);
                assert_eq!(calls(&e, 0x0098_4ba0), vec![vec![goal, kind]]);
                assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![GOAL_SIZE]]);
                e.mem.u32(goal)
            })
            .collect();
        assert_eq!(vtables[0], 0x0108_d530);
        assert_eq!(vtables[14], 0x0108_d47c);
        assert_eq!(vtables[15], 0x0108_d518);
        let mut unique = vtables.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 16);
    }

    #[test]
    fn create_reports_an_unknown_kind_and_a_failed_allocation() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0098_4490, &args![16u32]).u32(), 0);
        assert_eq!(calls(&e, DEBUG_PRINT), vec![vec![CREATE_FAILED_FORMAT, 16]]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0098_4490, &args![3u32]).u32(), 0);
        assert!(calls(&e, 0x0098_4ba0).is_empty());
    }

    /// Every constructor, its vtable and the calls it makes.
    fn cases(t: u32, k: u32) -> Vec<Case> {
        vec![
            (0x0098_4b20, 0x0108_d47c, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0098_4390, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0098_4c20, 0x0108_d488, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x10]),
                    (0x0097_ade0, vec![t, 0x3, 0x1]),
                    (0x0098_4390, vec![t, 0x3, 0x0]),
                ]
            }),
            (0x0098_4cb0, 0x0108_d494, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0xe, 0x1]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0098_4390, vec![t, 0xe, 0x0]),
                ]
            }),
            (0x0098_4d40, 0x0108_d4a0, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0xf, 0x1]),
                    (0x0097_ade0, vec![t, 0x1, 0x8]),
                    (0x0097_ade0, vec![t, 0x1, 0x7]),
                    (0x0098_4390, vec![t, 0xf, 0x0]),
                ]
            }),
            (0x0098_4dd0, 0x0108_d4ac, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x16, 0xffff_0016]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0098_4390, vec![t, 0xd, 0xffff_0016]),
                ]
            }),
            (0x0098_4e60, 0x0108_d4b8, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x16, 0xffff_0017]),
                    (0x0098_4390, vec![t, 0x10, 0xffff_0017]),
                ]
            }),
            (0x0098_4ee0, 0x0108_d4c4, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x13, 0x1]),
                    (0x0098_4390, vec![t, 0x1, 0x4]),
                    (0x0098_4f60, vec![t, 0x1]),
                ]
            }),
            (0x0098_4f80, 0x0108_d4d0, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x13, 0x2]),
                    (0x0098_4390, vec![t, 0x1, 0x5]),
                    (0x0098_4f60, vec![t, 0x2]),
                ]
            }),
            (0x0098_5000, 0x0108_d4dc, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x13, 0x3]),
                    (0x0098_4390, vec![t, 0x11, 0xf]),
                    (0x0098_4f60, vec![t, 0x3]),
                ]
            }),
            (0x0098_5080, 0x0108_d4e8, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x2, 0x7]),
                    (0x0098_4390, vec![t, 0x1, 0x7]),
                ]
            }),
            (0x0098_5100, 0x0108_d4f4, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x13, 0x4]),
                    (0x0098_4390, vec![t, 0x1, 0x6]),
                    (0x0098_4f60, vec![t, 0x4]),
                ]
            }),
            (0x0098_5180, 0x0108_d500, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x2, 0x8]),
                    (0x0098_4390, vec![t, 0x1, 0x8]),
                ]
            }),
            (0x0098_5200, 0x0108_d50c, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x2, 0x9]),
                    (0x0098_4390, vec![t, 0x1, 0x9]),
                ]
            }),
            (0x0098_5280, 0x0108_d518, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0098_4390, vec![t, 0x11, 0x3c]),
                ]
            }),
            (0x0098_5300, 0x0108_d524, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x9, 0x0]),
                    (0x0098_4390, vec![t, 0x9, 0x1]),
                ]
            }),
            (0x0098_5390, 0x0108_d530, {
                vec![
                    (0x0098_4ba0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x2, 0x10]),
                    (0x0098_4390, vec![t, 0x1, 0x10]),
                ]
            }),
        ]
    }

    #[test]
    fn each_constructor_builds_its_base_sets_its_vtable_and_fills_its_lists() {
        let mut e = engine();
        let this = e.mem.alloc(GOAL_SIZE);
        let kind = 0x0b;
        assert_eq!(cases(this, kind).len(), 16);
        for (address, vtable, expected) in cases(this, kind) {
            e.call_log = Some(vec![]);
            let back = e.call(address, &args![this, kind]).u32();
            assert_eq!(back, this, "{address:08x}");
            assert_eq!(e.mem.u32(this), vtable, "{address:08x}");
            let log: Calls = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != address)
                .collect();
            assert_eq!(log, expected, "{address:08x}");
        }
    }
}

#[cfg(test)]
mod n023_tests {
    use super::*;

    #[test]
    fn the_indexed_word_sits_past_0x3fff8() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0x40010);
        e.mem.set_u32(state + 0x3fff8 + 8, 0xabcd);
        assert_eq!(e.call(0x0098_4470, &args![state, 2u32]).u32(), 0xabcd);
        e.mem.set_u32(state + 0x3fff8 - 4, 0x77);
        assert_eq!(e.call(0x0098_4470, &args![state, u32::MAX]).u32(), 0x77);
    }

    /// A goal with a condition list: each node holds its next (+4) and its item (+8); the
    /// last node holds no item and ends the walk. Items are (kind, value) blocks.
    fn goal_with(e: &mut Engine, items: &[(u32, u32)]) -> u32 {
        let mut node = e.mem.alloc(12);
        for (kind, value) in items.iter().rev() {
            let item = e.mem.alloc(8);
            e.mem.set_u32(item, *kind);
            e.mem.set_u32(item + 4, *value);
            let before = e.mem.alloc(12);
            e.mem.set_u32(before + 4, node);
            e.mem.set_u32(before + 8, item);
            node = before;
        }
        let goal = e.mem.alloc(4);
        e.mem.set_u32(goal, node);
        e.register(LIST_FIRST, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NODE_ITEM, |_, a| (a[0] + 8).into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ITEM_KIND, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ITEM_VALUE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        goal
    }

    #[test]
    fn a_goal_is_active_while_every_condition_holds() {
        let mut e = Engine::new();
        // World-state conditions hold when the value is even.
        e.register(WORLD_STATE_IS_TRUE, |_, a| {
            u32::from(a[2] & 1 == 0).into_ret()
        });
        let state = e.mem.alloc(0x40010);
        e.mem.set_u32(state + 0x3fff8 + 4, 1);
        // Kind 0x16 needs a non-zero word, kind 0x15 a zero word, others the world state.
        let goal = goal_with(&mut e, &[(0x16, 1), (0x15, 0), (3, 4)]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0098_43b0, &args![goal, state]).bool());
        let log = e.call_log.take().unwrap();
        let world: Vec<_> = log.iter().filter(|c| c.0 == WORLD_STATE_IS_TRUE).collect();
        assert_eq!(world.len(), 1);
        assert_eq!(world[0].1, vec![state, 3, 4]);
        for items in [[(0x15, 1)], [(0x16, 0)], [(3, 5)]] {
            let goal = goal_with(&mut e, &items);
            assert!(!e.call(0x0098_43b0, &args![goal, state]).bool());
        }
        let goal = goal_with(&mut e, &[]);
        assert!(e.call(0x0098_43b0, &args![goal, state]).bool());
    }
}
