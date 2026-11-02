//! `fallout/ai/run_009942d0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The plan here is the object `run_00993d60` builds: its words match the Xbox PDB's
//! `CombatPlan` (`pPlanSteps` at `+0`, `iLength` at `+4`) and its steps `CombatPlanStep`
//! (`pAction`, `pParentStep`, `iNodeKey`, `iActionValue`). The engine map names none of
//! the functions, so they keep `fn_<addr>` names.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, `cdecl`, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, `cdecl`, one word: the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00401460` (unnamed, `cdecl`: destination, source, size): forwards to `00ec44d0`.
const MOVE_BYTES: u32 = 0x0040_1460;
/// `00994270` (`CombatPlan::GetIndexForStep`, Xbox PDB): `thiscall`, one word (`RET 4`).
const GET_INDEX_FOR_STEP: u32 = 0x0099_4270;
/// `007d6bb0` (unnamed): `thiscall`, returns `this + 0x14` (an action's first list node).
const LIST_AT_0X14: u32 = 0x007d_6bb0;
/// `0048d150` (unnamed): `thiscall`, an action's second list (its first node).
const SECOND_LIST: u32 = 0x0048_d150;
/// `006815c0` (unnamed): `thiscall`, returns `this` (a list node's item slot).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `0062d990` (unnamed): `thiscall`, the next list node.
const NODE_NEXT: u32 = 0x0062_d990;
/// `00559450` (unnamed): `thiscall`, returns the word at `+0`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `00726070` (unnamed, `menumanager.cpp` per the engine map): `thiscall` on a list item,
/// no stack words; a word result.
const ITEM_VALUE: u32 = 0x0072_6070;
/// `00984470` (unnamed, `combatgoal.cpp`): `thiscall`, one word (`RET 4`); a word result.
const RESOLVE_VALUE: u32 = 0x0098_4470;
/// `00994250` (unnamed, `run_00993d60`): `thiscall`, one word (`RET 4`); whether an
/// action's word at `+0x28` has a bit of the mask.
const ACTION_HAS_FLAG: u32 = 0x0099_4250;

/// The plan's step `index` (`pPlanSteps[index]`).
fn step(e: &Engine, this: Ptr, index: u32) -> u32 {
    let steps = e.mem.u32(this.addr());
    e.mem.u32(steps.wrapping_add(index.wrapping_mul(4)))
}

/// The address of the plan's step slot `index` (`&pPlanSteps[index]`).
fn slot(e: &Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(this.addr()).wrapping_add(index.wrapping_mul(4))
}

/// `count * 4` as the code computes an array size: `0xffffffff` when the product
/// overflows (`MUL`, `SETO`, `NEG`, `OR`).
fn array_bytes(count: u32) -> u32 {
    let (bytes, overflow) = count.overflowing_mul(4);
    if overflow {
        u32::MAX
    } else {
        bytes
    }
}

/// Whether a list node exists and holds an item (`node != 0 && *006815c0(node) != 0`).
fn has_item(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let item_slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(item_slot) != 0
}

/// The item of a list node (`*006815c0(node)`).
fn item(e: &mut Engine, node: u32) -> u32 {
    let item_slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(item_slot)
}

// Translated from 009942d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): offers every step to `fn_00994350`
/// (its index, action and value, with `context`), in order; when it returns false the
/// same index is offered again (the steps moved). The length is read on every turn.
pub fn fn_009942d0(e: &mut Engine, this: Ptr, context: u32) {
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + 0x4) {
        let current = step(e, this, index);
        let action = e.mem.u32(current);
        let value = e.mem.u32(step(e, this, index) + 0xc);
        if !fn_00994350(e, this, index, action, value, context) {
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00994350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`): checks step `index` (with `action`
/// and `value`) against every later step. For a later step whose action is `action`
/// itself, or one whose list at `+0x14` (`007d6bb0`) has, for each of `action`'s items,
/// an item with the same first word (`00559450`), the step is moved: when it has no parent
/// or its parent's index (`00994270`) is before the later step, the function returns true;
/// otherwise the steps from `index + 1` up to the parent are moved down one place and the
/// step goes just before its parent (returns false). For any other later step, each item
/// of `action`'s list is matched against the later action's second list (`0048d150`):
/// for a pair with the same first word, both items' values (`00726070`, resolved by
/// `fn_009947f0` on `context` with `value` and the later step's value as defaults) are
/// compared; when the later one is not -1 and differs, either the step is moved after
/// the later step (or before its parent, when it has one; not when that is not after
/// `index`), or, when the later action has flag 1 (`00994250`), the step's parent chain
/// reaches the later step, or the move is not possible, a new step is inserted before the
/// later step (`action`, the later value, key 0, parent the step after it; the array is
/// reallocated one longer). Both return false. Returns true when no later step matters.
pub fn fn_00994350(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    action: u32,
    value: u32,
    context: u32,
) -> bool {
    let p = this.addr();
    let mut later = index.wrapping_add(1);
    while later < e.mem.u32(p + 0x4) {
        let later_action = e.mem.u32(step(e, this, later));
        let later_value = e.mem.u32(step(e, this, later) + 0xc);
        let mut same = true;
        if later_action != action {
            let mut node = e.call(LIST_AT_0X14, &args![action]).u32();
            while has_item(e, node) {
                let mut found = false;
                let wanted = item(e, node);
                let mut other = e.call(LIST_AT_0X14, &args![later_action]).u32();
                while has_item(e, other) {
                    let candidate = item(e, other);
                    let wanted_word = e.call(READ_FIRST_WORD, &args![wanted]).u32();
                    if wanted_word == e.call(READ_FIRST_WORD, &args![candidate]).u32() {
                        found = true;
                        break;
                    }
                    other = e.call(NODE_NEXT, &args![other]).u32();
                }
                if !found {
                    same = false;
                    break;
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }

        if same {
            let parent = e.mem.u32(step(e, this, index) + 0x4);
            if parent == 0 {
                return true;
            }
            let parent_index = e.call(GET_INDEX_FOR_STEP, &args![this, parent]).u32();
            if parent_index < later {
                return true;
            }
            let moved = step(e, this, index);
            let size = parent_index
                .wrapping_sub(1)
                .wrapping_sub(index)
                .wrapping_shl(2);
            let source = e
                .mem
                .u32(p)
                .wrapping_add(index.wrapping_mul(4))
                .wrapping_add(4);
            let destination = slot(e, this, index);
            e.call(MOVE_BYTES, &args![destination, source, size]);
            let target = e
                .mem
                .u32(p)
                .wrapping_add(parent_index.wrapping_mul(4))
                .wrapping_sub(4);
            e.mem.set_u32(target, moved);
            return false;
        }

        let mut node = e.call(LIST_AT_0X14, &args![action]).u32();
        while has_item(e, node) {
            let wanted = item(e, node);
            let wanted_value = e.call(ITEM_VALUE, &args![wanted]).u32();
            let first = fn_009947f0(e, Ptr::new(context), wanted_value as i32, value as i32);
            let mut other = e.call(SECOND_LIST, &args![later_action]).u32();
            while has_item(e, other) {
                let candidate = item(e, other);
                let wanted_word = e.call(READ_FIRST_WORD, &args![wanted]).u32();
                if wanted_word == e.call(READ_FIRST_WORD, &args![candidate]).u32() {
                    let candidate_value = e.call(ITEM_VALUE, &args![candidate]).u32();
                    let second = fn_009947f0(
                        e,
                        Ptr::new(context),
                        candidate_value as i32,
                        later_value as i32,
                    );
                    if second != -1 && first != second {
                        let mut insert = false;
                        if e.call(ACTION_HAS_FLAG, &args![later_action, 1u32]).u8() != 0 {
                            insert = true;
                        } else {
                            let mut ancestor = e.mem.u32(step(e, this, index) + 0x4);
                            while ancestor != 0 {
                                if ancestor == step(e, this, later) {
                                    insert = true;
                                    break;
                                }
                                ancestor = e.mem.u32(ancestor + 0x4);
                            }
                        }
                        if !insert {
                            let moved = step(e, this, index);
                            let mut target = later;
                            let parent = e.mem.u32(step(e, this, index) + 0x4);
                            if parent != 0 {
                                target = e
                                    .call(GET_INDEX_FOR_STEP, &args![this, parent])
                                    .u32()
                                    .wrapping_sub(1);
                            }
                            if target > index {
                                let size = target.wrapping_sub(index).wrapping_shl(2);
                                let source = e
                                    .mem
                                    .u32(p)
                                    .wrapping_add(index.wrapping_mul(4))
                                    .wrapping_add(4);
                                let destination = slot(e, this, index);
                                e.call(MOVE_BYTES, &args![destination, source, size]);
                                let target_slot = slot(e, this, target);
                                e.mem.set_u32(target_slot, moved);
                            } else {
                                insert = true;
                            }
                        }
                        if insert {
                            let old_length = e.mem.u32(p + 0x4);
                            let old_steps = e.mem.u32(p);
                            e.mem.set_u32(p + 0x4, old_length.wrapping_add(1));
                            let size = array_bytes(e.mem.u32(p + 0x4));
                            let steps = e.call(OPERATOR_NEW, &args![size]).u32();
                            e.mem.set_u32(p, steps);
                            let steps = e.mem.u32(p);
                            e.call(MOVE_BYTES, &args![steps, old_steps, later.wrapping_shl(2)]);
                            let size = old_length.wrapping_sub(later).wrapping_shl(2);
                            let source = old_steps.wrapping_add(later.wrapping_mul(4));
                            let destination = e
                                .mem
                                .u32(p)
                                .wrapping_add(later.wrapping_mul(4))
                                .wrapping_add(4);
                            e.call(MOVE_BYTES, &args![destination, source, size]);
                            let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
                            let later_slot = slot(e, this, later);
                            e.mem.set_u32(later_slot, block);
                            let new_step = step(e, this, later);
                            e.mem.set_u32(new_step, action);
                            let new_step = step(e, this, later);
                            e.mem.set_u32(new_step + 0xc, second as u32);
                            let new_step = step(e, this, later);
                            e.mem.set_u32(new_step + 0x8, 0);
                            let following = e.mem.u32(
                                e.mem
                                    .u32(p)
                                    .wrapping_add(later.wrapping_mul(4))
                                    .wrapping_add(4),
                            );
                            let new_step = step(e, this, later);
                            e.mem.set_u32(new_step + 0x4, following);
                            e.call(OPERATOR_DELETE, &args![old_steps]);
                        }
                        return false;
                    }
                }
                other = e.call(NODE_NEXT, &args![other]).u32();
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        later = later.wrapping_add(1);
    }
    true
}

// Translated from 009947f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `value`, or `fallback` when `value`
/// is -1; a result that `fn_00994830` accepts (`0xffff0016..=0xffff0018`) is resolved
/// through `00984470` on `this`.
pub fn fn_009947f0(e: &mut Engine, this: Ptr, value: i32, fallback: i32) -> i32 {
    let mut value = value;
    if value == -1 {
        value = fallback;
    }
    if fn_00994830(e, value) {
        value = e.call(RESOLVE_VALUE, &args![this, value]).i32();
    }
    value
}

// Translated from 00994830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): whether `value` (signed) is in
/// `-0xffea..-0xffe7` (`0xffff0016`, `0xffff0017`, `0xffff0018`).
pub fn fn_00994830(_e: &mut Engine, value: i32) -> bool {
    (-0xffea..-0xffe7).contains(&value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_42d0, fn_009942d0(Ptr, u32)),
        entry!(0x0099_4350, fn_00994350(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x0099_47f0, fn_009947f0(Ptr, i32, i32) -> i32),
        entry!(0x0099_4830, fn_00994830(i32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// A plan with one step per action (value `values[i]`, parent `parents[i]` as a step
    /// index).
    fn plan(e: &mut Engine, actions: &[u32], values: &[u32], parents: &[Option<usize>]) -> u32 {
        let this = e.mem.alloc(0x18);
        let steps = e.mem.alloc(4 * actions.len() as u32);
        let blocks: Vec<u32> = actions.iter().map(|_| e.mem.alloc(0x10)).collect();
        for i in 0..actions.len() {
            e.mem.set_u32(blocks[i], actions[i]);
            e.mem.set_u32(blocks[i] + 0xc, values[i]);
            e.mem
                .set_u32(blocks[i] + 4, parents[i].map_or(0, |j| blocks[j]));
            e.mem.set_u32(steps + 4 * i as u32, blocks[i]);
        }
        e.mem.set_u32(this, steps);
        e.mem.set_u32(this + 4, actions.len() as u32);
        this
    }

    /// Doubles: lists are nodes `[item, next]`, an item is `[first word, value]`; an
    /// action holds its first list at `+0x14`, its second at `+0x18` and flags at `+0x28`.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_AT_0X14, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(SECOND_LIST, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(READ_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(ITEM_VALUE, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(ACTION_HAS_FLAG, |e, a| {
            ret((e.mem.u32(a[0] + 0x28) & a[1] != 0) as u32)
        });
        e.register(GET_INDEX_FOR_STEP, |e, a| {
            let steps = e.mem.u32(a[0]);
            let length = e.mem.u32(a[0] + 4);
            ret((0..length)
                .find(|i| e.mem.u32(steps + 4 * i) == a[1])
                .unwrap_or(0))
        });
        e.register(MOVE_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(RESOLVE_VALUE, |_, a| ret(a[1] + 0x100));
    }

    /// A list of items `(first word, value)`.
    fn list(e: &mut Engine, items: &[(u32, u32)]) -> u32 {
        let mut next = 0;
        for (word, value) in items.iter().rev() {
            let item = e.mem.alloc(8);
            e.mem.set_u32(item, *word);
            e.mem.set_u32(item + 4, *value);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn action(e: &mut Engine, first: &[(u32, u32)], second: &[(u32, u32)], flags: u32) -> u32 {
        let object = e.mem.alloc(0x2c);
        let first = list(e, first);
        let second = list(e, second);
        e.mem.set_u32(object + 0x14, first);
        e.mem.set_u32(object + 0x18, second);
        e.mem.set_u32(object + 0x28, flags);
        object
    }

    fn actions_of(e: &Engine, this: u32) -> Vec<u32> {
        (0..e.mem.u32(this + 4))
            .map(|i| e.mem.u32(step(e, Ptr::new(this), i)))
            .collect()
    }

    #[test]
    fn fn_009942d0_offers_every_step_and_repeats_after_a_move() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // Step 0 (a, parent step 2) is like step 1 (b has a's item 5), so it moves after
        // b; index 0 is offered again (now b, no parent: true), then a and c, which
        // match nothing.
        let a = action(&mut e, &[(5, 0)], &[], 0);
        let b = action(&mut e, &[(5, 0)], &[], 0);
        let c = action(&mut e, &[], &[], 0);
        let this = plan(&mut e, &[a, b, c], &[1, 2, 3], &[Some(2), None, None]);
        e.call_log = Some(vec![]);
        e.call(0x0099_42d0, &args![this, 0u32]);
        assert_eq!(calls(&e, GET_INDEX_FOR_STEP).len(), 1);
        assert_eq!(actions_of(&e, this), vec![b, a, c]);
    }

    #[test]
    fn fn_00994350_moves_a_step_before_its_parent() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // Step 0 (action a, items {5}) is like step 1 (action b with an item 5); its
        // parent is step 3, so steps 1 and 2 move down and step 0 goes to index 2.
        let a = action(&mut e, &[(5, 0)], &[], 0);
        let b = action(&mut e, &[(5, 0)], &[], 0);
        let c = action(&mut e, &[], &[], 0);
        let d = action(&mut e, &[], &[], 0);
        let this = plan(&mut e, &[a, b, c, d], &[0; 4], &[Some(3), None, None, None]);
        let r = e
            .call(0x0099_4350, &args![this, 0u32, a, 0u32, 0u32])
            .bool();
        assert!(!r);
        assert_eq!(actions_of(&e, this), vec![b, c, a, d]);
        // Without a parent nothing moves and the result is true.
        let this = plan(&mut e, &[a, b], &[0; 2], &[None, None]);
        assert!(e
            .call(0x0099_4350, &args![this, 0u32, a, 0u32, 0u32])
            .bool());
    }

    #[test]
    fn fn_00994350_moves_a_conflicting_step_after_the_later_one() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // Step 0 (action a, item 7 with value 1) conflicts with step 1 (action b, whose
        // second list has item 7 with value 2): step 0 moves after step 1.
        let a = action(&mut e, &[(7, 1)], &[], 0);
        let b = action(&mut e, &[(9, 0)], &[(7, 2)], 0);
        let this = plan(&mut e, &[a, b], &[0, 0], &[None, None]);
        let r = e
            .call(0x0099_4350, &args![this, 0u32, a, 0u32, 0u32])
            .bool();
        assert!(!r);
        assert_eq!(actions_of(&e, this), vec![b, a]);
        // Equal values: nothing happens.
        let b_same = action(&mut e, &[(9, 0)], &[(7, 1)], 0);
        let this = plan(&mut e, &[a, b_same], &[0, 0], &[None, None]);
        assert!(e
            .call(0x0099_4350, &args![this, 0u32, a, 0u32, 0u32])
            .bool());
    }

    #[test]
    fn fn_00994350_inserts_a_step_for_a_flagged_later_action() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let a = action(&mut e, &[(7, 1)], &[], 0);
        let b = action(&mut e, &[(9, 0)], &[(7, 2)], 1);
        let this = plan(&mut e, &[a, b], &[0, 0], &[None, None]);
        let old_b = step(&e, Ptr::new(this), 1);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x0099_4350, &args![this, 0u32, a, 0u32, 0u32])
            .bool();
        assert!(!r);
        assert_eq!(actions_of(&e, this), vec![a, a, b]);
        let inserted = step(&e, Ptr::new(this), 1);
        assert_eq!(e.mem.u32(inserted + 0xc), 2);
        assert_eq!(e.mem.u32(inserted + 8), 0);
        assert_eq!(e.mem.u32(inserted + 4), old_b);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![12], vec![0x10]]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn fn_009947f0_falls_back_and_resolves() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        assert_eq!(e.call(0x0099_47f0, &args![0u32, 5i32, 9i32]).i32(), 5);
        assert_eq!(e.call(0x0099_47f0, &args![0u32, -1i32, 9i32]).i32(), 9);
        assert_eq!(
            e.call(0x0099_47f0, &args![0u32, -1i32, 0xffff_0017u32])
                .u32(),
            0xffff_0117
        );
    }

    #[test]
    fn fn_00994830_accepts_three_values() {
        let mut e = Engine::new();
        let accepted: Vec<u32> = (0xffff_0014u32..0xffff_001b)
            .filter(|v| e.call(0x0099_4830, &args![*v]).bool())
            .collect();
        assert_eq!(accepted, vec![0xffff_0016, 0xffff_0017, 0xffff_0018]);
    }
}
