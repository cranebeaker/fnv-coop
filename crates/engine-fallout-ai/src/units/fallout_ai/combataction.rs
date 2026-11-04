//! `fallout/ai/combat/combataction.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CombatAction::CreateCombatAction` (the factory of the 27 action kinds),
//! the table of one action per kind at `011f1420` and the 20 lists at
//! `011f13d0` that index the actions by the first words of their +0x04
//! list's entries.

#[allow(unused_imports)]
use crate::prelude::*;

/// The action table (27 pointers, one per kind).
const ACTIONS: u32 = 0x011f_1420;
/// The lists (20 pointers) the actions are filed in.
const ACTION_LISTS: u32 = 0x011f_13d0;
/// Number of action kinds and of lists.
const ACTION_KINDS: u32 = 0x1b;
const LIST_COUNT: u32 = 0x14;
/// `operator new` (`00401000`, `cdecl(size)`) and `operator delete`
/// (`00401030`, `cdecl(block)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The game's `memset` wrapper (`00403d30`, `cdecl(dst, value, count)`).
const MEMSET: u32 = 0x0040_3d30;
/// The debug print `005b5e40` (`cdecl(format, ...)`) and
/// `"COMBAT: Could not create combat action of type %i"` (`0108cacc`).
const DEBUG_PRINT: u32 = 0x005b_5e40;
const CREATE_FAILED_FORMAT: u32 = 0x0108_cacc;
/// The two-word list head constructor (`0096a2d0`, `thiscall`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Appends to a list (`00905820`, `thiscall(element pointer)`).
const LIST_APPEND: u32 = 0x0090_5820;
/// Empties a list (`00470470`, `thiscall`) and its scalar deleting
/// destructor (`004702f0`, `thiscall(flags)`).
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
const LIST_DELETE: u32 = 0x0047_02f0;
/// The first node of an action +0x04 list (`007d6bb0`, `thiscall`), a
/// node item address (`006815c0`), the next node (`0062d990`) and an
/// entry first word (`00559450`).
const FIRST_NODE: u32 = 0x007d_6bb0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0062_d990;
const ENTRY_FIRST: u32 = 0x0055_9450;
/// The `CombatAction` destructor body (`00979c80`, `thiscall`).
const ACTION_DESTRUCT: u32 = 0x0097_9c80;
/// Every action object is 0x2C bytes.
const ACTION_SIZE: u32 = 0x2c;

/// The constructor of each kind (the jump table at `0097a924`, as arms).
fn action_constructor(kind: u32) -> Option<u32> {
    let constructor = match kind {
        0 => 0x0097_b0b0,  // CombatActionAttackRangedExplosive
        1 => 0x0097_b280,  // CombatActionAttackRangedExplosiveFromCover
        2 => 0x0097_ad10,  // CombatActionAttackRanged
        3 => 0x0097_aeb0,  // CombatActionAttackRangedFromCover
        4 => 0x0097_b620,  // CombatActionAttackGrenade
        5 => 0x0097_b7a0,  // CombatActionAttackGrenadeFlushTarget
        6 => 0x0097_b9e0,  // CombatActionAttackGrenadeFromCover
        7 => 0x0097_b350,  // CombatActionAttackMelee
        8 => 0x0097_b4e0,  // CombatActionAttackHandToHand
        9 => 0x0097_bbd0,  // CombatActionMove
        10 => 0x0097_bfe0, // CombatActionMoveAndSwitchToMelee
        11 => 0x0097_c170, // CombatActionMoveAndAttackRanged
        12 => 0x0097_c3a0, // CombatActionMoveAndAttackRangedExplosive
        13 => 0x0097_c560, // CombatActionMoveAndAttackGrenade
        14 => 0x0097_c700, // CombatActionDrawWeapon
        15 => 0x0097_c850, // CombatActionSwitchWeapon
        16 => 0x0097_c9d0, // CombatActionAvoidThreat
        17 => 0x0097_ca60, // CombatActionSearch
        18 => 0x0097_cb70, // CombatActionInvestigate
        19 => 0x0097_cc00, // CombatActionDodge
        20 => 0x0097_cc80, // CombatActionIgnoreBlockedTarget
        21 => 0x0097_cd30, // CombatActionFlee
        22 => 0x0097_ce90, // CombatActionActivateCombatItem
        23 => 0x0097_cfe0, // CombatActionUseCombatItem
        24 => 0x0097_d1c0, // CombatActionAcquireLineOfSight
        25 => 0x0097_d250, // CombatActionHide
        26 => 0x0097_d130, // CombatActionApproachTarget
        _ => return None,
    };
    Some(constructor)
}

// Translated from 00979f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00979f90 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatAction::CreateCombatAction` (Xbox PDB; `cdecl(kind)`): for kinds
/// 0..=26 allocates 0x2C bytes and runs that kind constructor with `kind`
/// (the class names are the RTTI names of the vtables they set); returns the
/// action, or null when the allocation fails. Any other kind prints
/// `"COMBAT: Could not create combat action of type %i"` (`005b5e40`) and
/// returns null.
pub fn combat_action_create_combat_action(e: &mut Engine, kind: u32) -> Ptr {
    let Some(constructor) = action_constructor(kind) else {
        e.call(DEBUG_PRINT, &args![CREATE_FAILED_FORMAT, kind]);
        return Ptr::NULL;
    };
    let block = e.call(OPERATOR_NEW, &args![ACTION_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(constructor, &args![block, kind]).ptr()
}

// Translated from 0097a990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no parameters): clears the 20 lists
/// at `011f13d0`, makes the action of each kind
/// ([`combat_action_create_combat_action`]) into the table at `011f1420`,
/// and files each action in the list of the first word of every entry of its
/// +0x04 list ([`fn_0097ab30`]).
pub fn fn_0097a990(e: &mut Engine) {
    e.call(MEMSET, &args![ACTION_LISTS, 0u32, 0x50u32]);
    for kind in 0..ACTION_KINDS {
        let action = combat_action_create_combat_action(e, kind).addr();
        e.set_global(ACTIONS + 4 * kind, action);
        if action == 0 {
            continue;
        }
        let mut node = e.call(FIRST_NODE, &args![action]).u32();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let entry = e.mem.u32(slot);
            let index = e.call(ENTRY_FIRST, &args![entry]).u32();
            fn_0097ab30(e, index, action);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 0097aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no parameters): deletes every
/// action of the table at `011f1420` ([`fn_0097ab00`] with 1), then empties
/// and deletes every list at `011f13d0` (`00470470`, `004702f0(1)`). The
/// table entries are left as they are.
pub fn fn_0097aa40(e: &mut Engine) {
    for kind in 0..ACTION_KINDS {
        let action: u32 = e.global(ACTIONS + 4 * kind);
        if action != 0 {
            fn_0097ab00(e, Ptr::new(action), 1);
        }
    }
    for index in 0..LIST_COUNT {
        let list: u32 = e.global(ACTION_LISTS + 4 * index);
        if list != 0 {
            e.call(LIST_REMOVE_ALL, &args![list]);
            let list: u32 = e.global(ACTION_LISTS + 4 * index);
            if list != 0 {
                e.call(LIST_DELETE, &args![list, 1u32]);
            }
        }
    }
}

// Translated from 0097ab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction` scalar deleting destructor (unnamed in the engine map; by
/// its body): the destructor body (`00979c80`), then `operator delete` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_0097ab00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ACTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0097ab30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097ab30 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`cdecl(index, action)`): makes list `index`
/// of `011f13d0` when it is null (8 bytes, `0096a2d0`; null when the
/// allocation fails) and appends `action` to it (`00905820`, given the
/// address of the `action` argument).
pub fn fn_0097ab30(e: &mut Engine, index: u32, action: u32) {
    let entry = ACTION_LISTS.wrapping_add(index.wrapping_mul(4));
    if e.mem.u32(entry) == 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(entry, list);
    }
    let list = e.mem.u32(entry);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), action);
        e.call(LIST_APPEND, &args![list, argument]);
    });
}

/// The table of the 27 action kinds at `011a4280`: 12 bytes per kind, its
/// name (a string pointer) at +0 and its cost (`float`) at +4.
const KIND_TABLE: u32 = 0x011a_4280;
const KIND_ENTRY_SIZE: u32 = 0xc;
/// The action's kind (`0059bb30`, `thiscall`: the word at +0x24; the engine
/// map's name for it, `D3DTexture_LockRect`, is a folded body).
const ACTION_KIND: u32 = 0x0059_bb30;
/// `00406d30` (`cdecl(buffer, size, text)`), which the debug text calls with
/// the kind's name.
const COPY_TEXT: u32 = 0x0040_6d30;
/// `00717e50` (`thiscall`: `this + 4`, the action's first list; folded body
/// named `BaseProcess::GetActorPackageThatIsRunning` in the engine map).
const FIRST_LIST: u32 = 0x0071_7e50;
/// An entry's two words (`00559450` and `00726070`, `thiscall`).
const ENTRY_KEY: u32 = 0x0055_9450;
const ENTRY_VALUE: u32 = 0x0072_6070;
/// `CombatWorldStateArray::IsTrue` (engine map; `009aadc0`,
/// `thiscall(key, value)`).
const WORLD_STATE_IS_TRUE: u32 = 0x009a_adc0;
/// `005e3fc0` (`thiscall`) on the controller argument of `CanExecute`: the
/// object [`fn_00979f70`] stores into.
const CONTROLLER_PLANNER: u32 = 0x005e_3fc0;
/// The pair lookup in the action's +0x0C list (`009aa410`,
/// `thiscall(key, out)`: true with the pair's second word in `*out`).
const LIST_FIND: u32 = 0x009a_a410;
/// Stores its word at `this` (`00537e90`, `thiscall(word)`).
const STORE_WORD: u32 = 0x0053_7e90;
/// `00404dc0` (`cdecl(a, b)`): zero when the two strings match.
const COMPARE_TEXT: u32 = 0x0040_4dc0;
/// `00586150` (`thiscall`: the word at +0x9C).
const CONTROLLER_STATE: u32 = 0x0058_6150;
/// `009cdac0` (`thiscall`, `float` in ST0) on that object: the value the
/// attack cost subtracts from.
const ATTACK_BASE: u32 = 0x009c_dac0;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;
/// `FLT_MAX` (`0108ca80`), the attack cost when the per-index value is 0.
const NO_ATTACK_COST: u32 = 0x0108_ca80;
/// `BGSWorldLocation::GetDistanceSquared` (engine map; `00527a40`,
/// `thiscall(other)`, ST0), the `double` it is divided by (`0108cb08`) and
/// the two `float`s passed with the result to `0040ebd0` and `00404010`
/// (both `cdecl(float, float)`, ST0): `0108cb00`, then `01013ea4`.
const DISTANCE_SQUARED: u32 = 0x0052_7a40;
const MOVEMENT_DIVISOR: u32 = 0x0108_cb08;
const MOVEMENT_FIRST_BOUND: u32 = 0x0108_cb00;
const MOVEMENT_SECOND_BOUND: u32 = 0x0101_3ea4;
const FIRST_BOUND_CALL: u32 = 0x0040_ebd0;
const SECOND_BOUND_CALL: u32 = 0x0040_4010;

/// The kind's table entry.
fn kind_entry(e: &mut Engine, this: Ptr) -> u32 {
    let kind = e.call(ACTION_KIND, &args![this]).u32();
    KIND_TABLE.wrapping_add(kind.wrapping_mul(KIND_ENTRY_SIZE))
}

// Translated from 00979e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction::GetName` (Xbox PDB; `thiscall`): the name pointer of the
/// action's kind in the table at `011a4280`.
pub fn combat_action_get_name(e: &mut Engine, this: Ptr) -> u32 {
    let entry = kind_entry(e, this);
    e.mem.u32(entry)
}

// Translated from 00979e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction::GetCost` (Xbox PDB; `thiscall`, two stack words it never
/// reads): the cost of the action's kind in the table at `011a4280` (`FLD`,
/// a signalling NaN comes back quiet).
pub fn combat_action_get_cost(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> f32 {
    let entry = kind_entry(e, this);
    x87_quiet(e.mem.f32(entry + 4))
}

// Translated from 00979e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction::GetDebugText` (Xbox PDB; `thiscall(buffer, size, word)`):
/// `00406d30(buffer, size, name)` with the kind's name; the third word is
/// not read.
pub fn combat_action_get_debug_text(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    _unused_3: u32,
) {
    let name = combat_action_get_name(e, this);
    e.call(COPY_TEXT, &args![buffer, size, name]);
}

// Translated from 00979e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction::CanExecute` (Xbox PDB; `thiscall(controller, world_state,
/// default_value)`): false when any entry of the action's first list
/// (`00717e50`) is not true in `world_state`
/// (`CombatWorldStateArray::IsTrue(key, value)`, the entry's value, or
/// `default_value` where the entry's value is -1). Otherwise, when the
/// action's +0x0C list holds a pair for 1 ([`fn_00979f50`]), stores its
/// second word through [`fn_00979f70`] on the controller's `005e3fc0`, and
/// returns true.
pub fn combat_action_can_execute(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    world_state: u32,
    default_value: u32,
) -> bool {
    let mut node = e.call(FIRST_LIST, &args![this]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            let value = if e.call(ENTRY_VALUE, &args![entry]).i32() == -1 {
                default_value
            } else {
                e.call(ENTRY_VALUE, &args![entry]).u32()
            };
            let key = e.call(ENTRY_KEY, &args![entry]).u32();
            if !e
                .call(WORLD_STATE_IS_TRUE, &args![world_state, key, value])
                .bool()
            {
                return false;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0xffff_ffff);
        if fn_00979f50(e, this, 1, out.addr()) {
            let value = e.mem.u32(out.addr());
            let planner = e.call(CONTROLLER_PLANNER, &args![controller]).u32();
            fn_00979f70(e, Ptr::new(planner), value);
        }
    });
    true
}

// Translated from 00979f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(key, out)`): looks `key` up in the
/// pair list at +0x0C (`009aa410`) and returns whether it was found.
pub fn fn_00979f50(e: &mut Engine, this: Ptr, key: u32, out: u32) -> bool {
    e.call(LIST_FIND, &args![this.addr() + 0xc, key, out])
        .bool()
}

// Translated from 00979f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(word)`): stores `word` through the
/// pointer at +0x04 (`00537e90`).
pub fn fn_00979f70(e: &mut Engine, this: Ptr, word: u32) {
    let target = e.mem.u32(this.addr() + 4);
    e.call(STORE_WORD, &args![target, word]);
}

// Translated from 0097abd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl(name, cost)`): sets the cost of the
/// first kind of the table at `011a4280` whose name matches `name`
/// (`00404dc0`) and returns true; false when none does.
pub fn fn_0097abd0(e: &mut Engine, name: u32, cost: f32) -> bool {
    for kind in 0..ACTION_KINDS {
        let entry = KIND_TABLE + kind * KIND_ENTRY_SIZE;
        let entry_name = e.mem.u32(entry);
        if e.call(COMPARE_TEXT, &args![entry_name, name]).u32() == 0 {
            e.mem.set_f32(entry + 4, x87_quiet(cost));
            return true;
        }
    }
    false
}

// Translated from 0097ac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction::GetAttackCost` (Xbox PDB; `cdecl(controller, index)`):
/// on the controller's +0x9C object (`00586150`), `FLT_MAX` when its value
/// `index` ([`fn_0097ac90`]) is 0, otherwise `009cdac0`'s value (stored as a
/// `float`) minus that value, as a `float`.
pub fn combat_action_get_attack_cost(e: &mut Engine, controller: u32, index: u32) -> f32 {
    let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
    let value = fn_0097ac90(e, Ptr::new(state), index).to_bits();
    let zero: f64 = e.global(ZERO);
    if x87_load32(value) == zero {
        return x87_quiet(e.global::<f32>(NO_ATTACK_COST));
    }
    let base = x87_store32(e.call(ATTACK_BASE, &args![state]).f64());
    let cost = x87_sub(x87_load32(base), x87_m32(value));
    f32::from_bits(x87_store32(cost))
}

// Translated from 0097ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(index)`): the `float` at
/// `+0x3C + 4 * index` (`FLD`).
pub fn fn_0097ac90(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    let address = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(0x3c);
    x87_quiet(e.mem.f32(address))
}

// Translated from 0097acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatAction::GetMovementCost` (Xbox PDB; `cdecl(unused, from, to)`):
/// the squared distance between the two world locations divided by the
/// `double` at `0108cb08` (stored as a `float`), passed with the `float` at
/// `0108cb00` to `0040ebd0`, and that result with the `float` at `01013ea4`
/// to `00404010`; each result is stored as a `float`.
pub fn combat_action_get_movement_cost(e: &mut Engine, _unused_0: u32, from: u32, to: u32) -> f32 {
    let distance = e.call(DISTANCE_SQUARED, &args![from, to]).f64();
    let divisor: f64 = e.global(MOVEMENT_DIVISOR);
    let scaled = x87_store32(x87_div(distance, divisor));
    let first_bound = x87_quiet_bits(e.global::<u32>(MOVEMENT_FIRST_BOUND));
    let bounded = e
        .call(
            FIRST_BOUND_CALL,
            &args![x87_quiet_bits(scaled), first_bound],
        )
        .f64();
    let bounded = x87_store32(bounded);
    let second_bound = x87_quiet_bits(e.global::<u32>(MOVEMENT_SECOND_BOUND));
    let result = e
        .call(
            SECOND_BOUND_CALL,
            &args![x87_quiet_bits(bounded), second_bound],
        )
        .f64();
    f32::from_bits(x87_store32(result))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_9e20, combat_action_get_name(Ptr) -> u32),
        entry!(0x0097_9e40, combat_action_get_cost(Ptr, u32, u32) -> f32),
        entry!(
            0x0097_9e60,
            combat_action_get_debug_text(Ptr, u32, u32, u32)
        ),
        entry!(
            0x0097_9e90,
            combat_action_can_execute(Ptr, u32, u32, u32) -> bool
        ),
        entry!(0x0097_9f50, fn_00979f50(Ptr, u32, u32) -> bool),
        entry!(0x0097_9f70, fn_00979f70(Ptr, u32)),
        entry!(0x0097_abd0, fn_0097abd0(u32, f32) -> bool),
        entry!(
            0x0097_ac30,
            combat_action_get_attack_cost(u32, u32) -> f32
        ),
        entry!(0x0097_ac90, fn_0097ac90(Ptr, u32) -> f32),
        entry!(
            0x0097_acb0,
            combat_action_get_movement_cost(u32, u32, u32) -> f32
        ),
        entry!(
            0x0097_9f90,
            combat_action_create_combat_action(u32) -> Ptr
        ),
        entry!(0x0097_a990, fn_0097a990()),
        entry!(0x0097_aa40, fn_0097aa40()),
        entry!(0x0097_ab00, fn_0097ab00(Ptr, u32) -> Ptr),
        entry!(0x0097_ab30, fn_0097ab30(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_1000, 0x1000);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for kind in 0..ACTION_KINDS {
            let constructor = action_constructor(kind).unwrap();
            e.register(constructor, |e, a| {
                e.mem.set_u32(a[0] + 0x24, a[1]);
                a[0].into_ret()
            });
        }
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_runs_the_kind_constructor() {
        let mut e = engine();
        for kind in 0..ACTION_KINDS {
            let action = e.call(0x0097_9f90, &args![kind]).u32();
            assert_ne!(action, 0);
            assert_eq!(e.mem.u32(action + 0x24), kind);
            let constructor = action_constructor(kind).unwrap();
            assert_eq!(calls(&e, constructor), vec![vec![action, kind]]);
        }
        assert_eq!(action_constructor(2), Some(0x0097_ad10));
        assert_eq!(action_constructor(26), Some(0x0097_d130));
        let sizes: Vec<u32> = calls(&e, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x2c; 27]);
    }

    #[test]
    fn create_reports_an_unknown_kind_and_a_failed_allocation() {
        let mut e = engine();
        assert_eq!(e.call(0x0097_9f90, &args![27u32]).u32(), 0);
        assert_eq!(calls(&e, DEBUG_PRINT), vec![vec![CREATE_FAILED_FORMAT, 27]]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0097_9f90, &args![4u32]).u32(), 0);
        assert!(calls(&e, 0x0097_b620).is_empty());
    }

    #[test]
    fn file_makes_the_list_once_and_appends() {
        let mut e = engine();
        e.register(LIST_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        let appended: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let record = appended.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x0097_ab30, &args![3u32, 0x7000u32]);
        e.call(0x0097_ab30, &args![3u32, 0x7100u32]);
        let list = e.global::<u32>(ACTION_LISTS + 12);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![list - 1]]);
        assert_eq!(*appended.borrow(), vec![(list, 0x7000), (list, 0x7100)]);
    }

    #[test]
    fn set_up_makes_every_action_and_files_it() {
        let mut e = engine();
        e.register(MEMSET, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIST_APPEND, |_, _| Ret::default());
        // Every action has one entry, whose first word is kind % 20.
        e.register(FIRST_NODE, |_, a| a[0].into_ret());
        e.register(NODE_ITEM, |_, a| (a[0] + 0x24).into_ret());
        e.register(ENTRY_FIRST, |_, a| (a[0] % 20).into_ret());
        e.register(NODE_NEXT, |_, _| 0u32.into_ret());
        e.call(0x0097_a990, &args![]);
        assert_eq!(calls(&e, MEMSET), vec![vec![ACTION_LISTS, 0, 0x50]]);
        // Kind 0 stores 0 at +0x24: its node has no entry.
        assert_eq!(calls(&e, ENTRY_FIRST).len(), 26);
        assert_eq!(calls(&e, LIST_APPEND).len(), 26);
        for kind in 0..ACTION_KINDS {
            assert_ne!(e.global::<u32>(ACTIONS + 4 * kind), 0);
        }
    }

    #[test]
    fn tear_down_deletes_actions_and_lists() {
        let mut e = engine();
        for addr in [
            ACTION_DESTRUCT,
            OPERATOR_DELETE,
            LIST_REMOVE_ALL,
            LIST_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.set_global(ACTIONS + 8, 0x7000u32);
        e.set_global(ACTION_LISTS + 4, 0x7100u32);
        e.call(0x0097_aa40, &args![]);
        assert_eq!(calls(&e, ACTION_DESTRUCT), vec![vec![0x7000]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x7000]]);
        assert_eq!(calls(&e, LIST_REMOVE_ALL), vec![vec![0x7100]]);
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![0x7100, 1]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_ab00, &args![0x7000u32, 0u32]).u32(), 0x7000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }
}

#[cfg(test)]
mod kind_tests {
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

    fn ret_f64(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_4000, 0x1000);
        e.map(0x0101_2000, 0x2000);
        e.map(0x0108_c000, 0x1000);
        e.register(ACTION_KIND, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn name_cost_and_debug_text_read_the_kind_table() {
        let mut e = engine();
        let action = e.mem.alloc(0x2c);
        e.mem.set_u32(action + 0x24, 3);
        e.mem.set_u32(KIND_TABLE + 36, 0x0123_4567);
        e.mem.set_u32(KIND_TABLE + 40, 0x7f80_0001);
        assert_eq!(e.call(0x0097_9e20, &args![action]).u32(), 0x0123_4567);
        let cost = e.call(0x0097_9e40, &args![action, 0u32, 0u32]).f32();
        assert_eq!(cost.to_bits(), 0x7fc0_0001);
        e.register(COPY_TEXT, |_, _| Ret::default());
        e.call(0x0097_9e60, &args![action, 0x5000u32, 0x40u32, 9u32]);
        assert_eq!(calls(&e, COPY_TEXT), vec![vec![0x5000, 0x40, 0x0123_4567]]);
    }

    #[test]
    fn set_cost_by_name_finds_the_first_match() {
        let mut e = engine();
        for kind in 0..ACTION_KINDS {
            e.mem.set_u32(KIND_TABLE + kind * 12, 0x100 + kind);
        }
        e.register(COMPARE_TEXT, |_, a| u32::from(a[0] != a[1]).into_ret());
        assert!(e.call(0x0097_abd0, &args![0x105u32, 2.5f32]).bool());
        assert_eq!(e.mem.f32(KIND_TABLE + 5 * 12 + 4), 2.5);
        assert_eq!(calls(&e, COMPARE_TEXT).len(), 6);
        assert!(!e.call(0x0097_abd0, &args![0x999u32, 1.0f32]).bool());
        assert_eq!(calls(&e, COMPARE_TEXT).len(), 6 + 27);
    }

    #[test]
    fn can_execute_checks_every_entry_then_stores_the_pair() {
        let mut e = engine();
        // Two nodes: (key 7, value -1) and (key 8, value 4).
        let entries = [e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(entries[0], 7);
        e.mem.set_u32(entries[0] + 4, 0xffff_ffff);
        e.mem.set_u32(entries[1], 8);
        e.mem.set_u32(entries[1] + 4, 4);
        let nodes = [e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(nodes[0], entries[0]);
        e.mem.set_u32(nodes[0] + 4, nodes[1]);
        e.mem.set_u32(nodes[1], entries[1]);
        let this = e.mem.alloc(0x2c);
        let first = nodes[0];
        e.register_double(FIRST_LIST, move |_, _| first.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ENTRY_KEY, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ENTRY_VALUE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORLD_STATE_IS_TRUE, |_, a| {
            (a[1] != 8 || a[0] == 1).into_ret()
        });
        e.register(LIST_FIND, |e, a| {
            e.mem.set_u32(a[2], 0x55);
            true.into_ret()
        });
        let planner = e.mem.alloc(8);
        e.mem.set_u32(planner + 4, 0x6000);
        e.register_double(CONTROLLER_PLANNER, move |_, _| planner.into_ret());
        e.register(STORE_WORD, |_, _| Ret::default());
        // World state 2 rejects key 8.
        assert!(!e
            .call(0x0097_9e90, &args![this, 0x900u32, 2u32, 0x33u32])
            .bool());
        assert_eq!(
            calls(&e, WORLD_STATE_IS_TRUE),
            vec![vec![2, 7, 0x33], vec![2, 8, 4]]
        );
        assert!(calls(&e, LIST_FIND).is_empty());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0097_9e90, &args![this, 0x900u32, 1u32, 0x33u32])
            .bool());
        let find = calls(&e, LIST_FIND);
        assert_eq!(find.len(), 1);
        assert_eq!(&find[0][..2], &[this + 0xc, 1]);
        assert_eq!(calls(&e, CONTROLLER_PLANNER), vec![vec![0x900]]);
        assert_eq!(calls(&e, STORE_WORD), vec![vec![0x6000, 0x55]]);
        // No pair: nothing is stored.
        e.call_log = Some(vec![]);
        e.register(LIST_FIND, |_, _| false.into_ret());
        assert!(e
            .call(0x0097_9e90, &args![this, 0x900u32, 1u32, 0x33u32])
            .bool());
        assert!(calls(&e, STORE_WORD).is_empty());
    }

    #[test]
    fn attack_cost_is_the_base_minus_the_value() {
        let mut e = engine();
        e.set_global(ZERO, 0.0f64);
        e.set_global(NO_ATTACK_COST, f32::MAX);
        let state = e.mem.alloc(0x60);
        e.mem.set_f32(state + 0x3c + 8, 1.5);
        e.register_double(CONTROLLER_STATE, move |_, _| state.into_ret());
        e.register(ATTACK_BASE, |_, _| ret_f64(4.0));
        assert_eq!(e.call(0x0097_ac30, &args![0x900u32, 2u32]).f32(), 2.5);
        assert_eq!(calls(&e, ATTACK_BASE), vec![vec![state]]);
        assert_eq!(e.call(0x0097_ac30, &args![0x900u32, 1u32]).f32(), f32::MAX);
        assert_eq!(calls(&e, ATTACK_BASE).len(), 1);
        assert_eq!(e.call(0x0097_ac90, &args![state, 2u32]).f32(), 1.5);
    }

    #[test]
    fn movement_cost_scales_and_bounds_the_distance() {
        let mut e = engine();
        e.set_global(MOVEMENT_DIVISOR, 100.0f64);
        e.set_global(MOVEMENT_FIRST_BOUND, 0.5f32);
        e.map(0x0101_3000, 0x1000);
        e.set_global(MOVEMENT_SECOND_BOUND, 3.0f32);
        e.register(DISTANCE_SQUARED, |_, _| ret_f64(250.0));
        e.register(FIRST_BOUND_CALL, |_, a| {
            ret_f64(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(SECOND_BOUND_CALL, |_, a| {
            ret_f64(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        let cost = e.call(0x0097_acb0, &args![0u32, 0x700u32, 0x800u32]).f32();
        assert_eq!(cost, 2.5);
        assert_eq!(calls(&e, DISTANCE_SQUARED), vec![vec![0x700, 0x800]]);
        assert_eq!(
            calls(&e, FIRST_BOUND_CALL),
            vec![vec![2.5f32.to_bits(), 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, SECOND_BOUND_CALL),
            vec![vec![2.5f32.to_bits(), 3.0f32.to_bits()]]
        );
    }
}
