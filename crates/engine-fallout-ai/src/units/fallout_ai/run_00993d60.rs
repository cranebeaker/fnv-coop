//! `fallout/ai/run_00993d60` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here work on an object whose words match the Xbox PDB's `CombatPlan`
//! (0x18 bytes: `pPlanSteps`, `iLength`, `iCurrentAction`, `pGoal`, `iCombatLocation`,
//! `pCombatController`) and its 0x10-byte steps (`CombatPlanStep`: `pAction`,
//! `pParentStep`, `iNodeKey`, `iActionValue`). The engine map names none of them, so they
//! keep `fn_<addr>` names; the class is read at its offsets.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, `cdecl`, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, `cdecl`, one word: the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0044ddc0` (unnamed): `thiscall`, returns the word at `+8` (the array's count).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `006a7ad0` (unnamed, `navmeshsearch.cpp` per the engine map): `thiscall`, one index
/// word (`RET 4`); returns the address of that element (it forwards to `00877a30`).
const ARRAY_ELEMENT: u32 = 0x006a_7ad0;
/// `00559450` (unnamed): `thiscall`, returns the word at `+0` (here a link's
/// `iLocation`).
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `00994a90` (unnamed, `run_009948f0`): `thiscall`, the key of a plan node.
const NODE_KEY: u32 = 0x0099_4a90;
/// `009942d0` (unnamed, `run_009942d0`): `thiscall`, one word (`RET 4`); orders the steps.
const ORDER_STEPS: u32 = 0x0099_42d0;
/// `0097e490` (unnamed): `thiscall`, true when `iCurrentAction >= iLength` (the plan is
/// done).
const PLAN_IS_DONE: u32 = 0x0097_e490;
/// `008cdd90` (unnamed): `thiscall` on the plan's controller argument, one word result.
const CONTROLLER_ACTION: u32 = 0x008c_dd90;
/// `009ee040` (named `DialoguePackage::GetActorStartedConversation` in the engine map;
/// the linker folds such getters): `thiscall` on the controller argument, one word result.
const CONTROLLER_VALUE: u32 = 0x009e_e040;
/// `00980470` (unnamed, `combatcontroller.cpp`): `thiscall`, `bool` result.
const CONTROLLER_TEST_00980470: u32 = 0x0098_0470;
/// `009804d0` (unnamed, `combatcontroller.cpp`): `thiscall`, `bool` result.
const CONTROLLER_TEST_009804D0: u32 = 0x0098_04d0;
/// `00980250` (unnamed, `combatcontroller.cpp`): `thiscall`, no stack words.
const CONTROLLER_RESET_00980250: u32 = 0x0098_0250;
/// `009db930` (unnamed, `actormover.cpp` per the engine map): `thiscall`, one word
/// (`RET 4`).
const CONTROLLER_SET_ACTION: u32 = 0x009d_b930;
/// `005468d0` (unnamed, `tesobjectcell.cpp` per the engine map): `thiscall`, one word
/// (`RET 4`).
const CONTROLLER_SET_VALUE: u32 = 0x0054_68d0;

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

/// The plan's step `index` (`pPlanSteps[index]`).
fn step(e: &Engine, this: Ptr, index: u32) -> u32 {
    let steps = e.mem.u32(this.addr());
    e.mem.u32(steps.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00993d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): clears the steps, length, current
/// step and goal words (`+0` to `+0xc`), sets `+0x10` to -1 and stores `controller` at
/// `+0x14`; returns `this`.
pub fn fn_00993d60(e: &mut Engine, this: Ptr, controller: u32) -> Ptr {
    let p = this.addr();
    e.mem.set_u32(p, 0);
    e.mem.set_u32(p + 0x4, 0);
    e.mem.set_u32(p + 0x8, 0);
    e.mem.set_u32(p + 0xc, 0);
    e.mem.set_u32(p + 0x10, 0xffff_ffff);
    e.mem.set_u32(p + 0x14, controller);
    this
}

// Translated from 00993db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`): builds the plan from a search
/// result. Sets the current step to 0, `+0xc` to `goal`, `+0x10` to -1 and `+0x14` to
/// `controller`; allocates one step per element of `links` (`fn_00993fd0` with
/// `0044ddc0`'s count) and fills step `i` from link `length - 1 - i` (its action, and its
/// desired value as `+0xc`) and node `length - i` of `nodes` (its key, `00994a90`, as
/// `+8`), clearing the parent; a link location (`+0xc`, through `00559450`) other than -1
/// becomes `+0x10`. Then each step's parent is the first step whose key is the key of its
/// link's parent node (`+4`), and `009942d0` orders the steps with `order_argument`.
/// Returns `this`.
pub fn fn_00993db0(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    goal: u32,
    nodes: u32,
    links: u32,
    order_argument: u32,
) -> Ptr {
    let p = this.addr();
    e.mem.set_u32(p + 0x8, 0);
    e.mem.set_u32(p + 0xc, goal);
    e.mem.set_u32(p + 0x10, 0xffff_ffff);
    e.mem.set_u32(p + 0x14, controller);
    let count = e.call(ARRAY_COUNT, &args![links]).u32();
    fn_00993fd0(e, this, count);

    let mut index = 0u32;
    while index < e.mem.u32(p + 0x4) {
        let length = e.mem.u32(p + 0x4);
        let link_slot = e
            .call(
                ARRAY_ELEMENT,
                &args![links, length.wrapping_sub(index).wrapping_sub(1)],
            )
            .u32();
        let link = e.mem.u32(link_slot);
        let length = e.mem.u32(p + 0x4);
        let node_slot = e
            .call(ARRAY_ELEMENT, &args![nodes, length.wrapping_sub(index)])
            .u32();
        let node = e.mem.u32(node_slot);
        let action = e.mem.u32(link);
        let target = step(e, this, index);
        e.mem.set_u32(target, action);
        let desired_value = e.mem.u32(link + 0x8);
        let target = step(e, this, index);
        e.mem.set_u32(target + 0xc, desired_value);
        let key = e.call(NODE_KEY, &args![node]).u32();
        let target = step(e, this, index);
        e.mem.set_u32(target + 0x8, key);
        let target = step(e, this, index);
        e.mem.set_u32(target + 0x4, 0);
        if e.call(READ_FIRST_WORD, &args![link + 0xc]).u32() != 0xffff_ffff {
            let location = e.call(READ_FIRST_WORD, &args![link + 0xc]).u32();
            e.mem.set_u32(p + 0x10, location);
        }
        index = index.wrapping_add(1);
    }

    let mut index = 0u32;
    while index < e.mem.u32(p + 0x4) {
        let length = e.mem.u32(p + 0x4);
        let link_slot = e
            .call(
                ARRAY_ELEMENT,
                &args![links, length.wrapping_sub(index).wrapping_sub(1)],
            )
            .u32();
        let link = e.mem.u32(link_slot);
        let parent_node = e.mem.u32(link + 0x4);
        let key = e.call(NODE_KEY, &args![parent_node]).u32();
        let mut other = 0u32;
        while other < e.mem.u32(p + 0x4) {
            let candidate = step(e, this, other);
            if e.mem.u32(candidate + 0x8) == key {
                let target = step(e, this, index);
                e.mem.set_u32(target + 0x4, candidate);
                break;
            }
            other = other.wrapping_add(1);
        }
        index = index.wrapping_add(1);
    }

    e.call(ORDER_STEPS, &args![this, order_argument]);
    this
}

// Translated from 00993f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): frees every step (`operator delete`), then the
/// step array.
pub fn fn_00993f70(e: &mut Engine, this: Ptr) {
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + 0x4) {
        let block = step(e, this, index);
        e.call(OPERATOR_DELETE, &args![block]);
        index = index.wrapping_add(1);
    }
    let steps = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![steps]);
}

// Translated from 00993fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): sets the length (`+4`) to `count`,
/// allocates the step array (`count * 4` bytes, `0xffffffff` on overflow) into `+0` and
/// one 0x10-byte step for each element.
pub fn fn_00993fd0(e: &mut Engine, this: Ptr, count: u32) {
    let p = this.addr();
    e.mem.set_u32(p + 0x4, count);
    let size = array_bytes(e.mem.u32(p + 0x4));
    let steps = e.call(OPERATOR_NEW, &args![size]).u32();
    e.mem.set_u32(p, steps);
    let mut index = 0u32;
    while index < e.mem.u32(p + 0x4) {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let steps = e.mem.u32(p);
        e.mem
            .set_u32(steps.wrapping_add(index.wrapping_mul(4)), block);
        index = index.wrapping_add(1);
    }
}

// Translated from 00994050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): runs the plan's steps for
/// `controller`. While the plan is not done (`0097e490`) and the current step's action
/// has flag 2 (`fn_00994250`), it calls the action's slot `+4` with `controller` and the
/// step's value and moves to the next step. Then, if the plan is still not done, the
/// current step is started (`00980250` on `controller`, the action's slot `+4`,
/// `009db930` with the action, `005468d0` with the value) unless it is already the one
/// running: its action is `008cdd90`'s, its value `009ee040`'s, the action's slot `+8`
/// returns true and both `00980470` and `009804d0` return false. Either way the plan moves
/// to the next step. Returns whether the plan is done.
pub fn fn_00994050(e: &mut Engine, this: Ptr, controller: u32) -> bool {
    let p = this.addr();
    loop {
        if e.call(PLAN_IS_DONE, &args![this]).u8() != 0 {
            break;
        }
        let current = e.mem.u32(p + 0x8);
        let action = e.mem.u32(step(e, this, current));
        if !fn_00994250(e, Ptr::new(action), 2) {
            break;
        }
        let current = e.mem.u32(p + 0x8);
        let value = e.mem.u32(step(e, this, current) + 0xc);
        let action = e.mem.u32(step(e, this, current));
        e.vcall(action, 0x4, &args![controller, value]);
        let current = e.mem.u32(p + 0x8);
        e.mem.set_u32(p + 0x8, current.wrapping_add(1));
    }

    if e.call(PLAN_IS_DONE, &args![this]).u8() == 0 {
        let mut start = true;
        let current = e.mem.u32(p + 0x8);
        let current_step = step(e, this, current);
        let running_action = e.call(CONTROLLER_ACTION, &args![controller]).u32();
        if e.mem.u32(current_step) == running_action {
            let current = e.mem.u32(p + 0x8);
            let current_step = step(e, this, current);
            let running_value = e.call(CONTROLLER_VALUE, &args![controller]).u32();
            if e.mem.u32(current_step + 0xc) == running_value {
                let current = e.mem.u32(p + 0x8);
                let value = e.mem.u32(step(e, this, current) + 0xc);
                let action = e.mem.u32(step(e, this, current));
                if e.vcall(action, 0x8, &args![controller, value]).u8() != 0
                    && e.call(CONTROLLER_TEST_00980470, &args![controller]).u8() == 0
                    && e.call(CONTROLLER_TEST_009804D0, &args![controller]).u8() == 0
                {
                    start = false;
                }
            }
        }
        if start {
            e.call(CONTROLLER_RESET_00980250, &args![controller]);
            let current = e.mem.u32(p + 0x8);
            let value = e.mem.u32(step(e, this, current) + 0xc);
            let action = e.mem.u32(step(e, this, current));
            e.vcall(action, 0x4, &args![controller, value]);
            let current = e.mem.u32(p + 0x8);
            let action = e.mem.u32(step(e, this, current));
            e.call(CONTROLLER_SET_ACTION, &args![controller, action]);
            let current = e.mem.u32(p + 0x8);
            let value = e.mem.u32(step(e, this, current) + 0xc);
            e.call(CONTROLLER_SET_VALUE, &args![controller, value]);
        }
        // Both branches of the exe end with the same increment.
        let current = e.mem.u32(p + 0x8);
        e.mem.set_u32(p + 0x8, current.wrapping_add(1));
    }
    e.call(PLAN_IS_DONE, &args![this]).u8() != 0
}

// Translated from 00994250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the word at `+0x28` has any
/// bit of `mask` (called on a plan step's action).
pub fn fn_00994250(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x28) & mask != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_3d60, fn_00993d60(Ptr, u32) -> Ptr),
        entry!(0x0099_3db0, fn_00993db0(Ptr, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x0099_3f70, fn_00993f70(Ptr)),
        entry!(0x0099_3fd0, fn_00993fd0(Ptr, u32)),
        entry!(0x0099_4050, fn_00994050(Ptr, u32) -> bool),
        entry!(0x0099_4250, fn_00994250(Ptr, u32) -> bool),
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

    /// Doubles for the allocator: `operator new` allocates, `operator delete` does nothing.
    fn allocator(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0].min(0x1000))));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
    }

    /// A plan whose steps are `actions[i]` with value `values[i]`.
    fn plan(e: &mut Engine, actions: &[u32], values: &[u32]) -> u32 {
        let this = e.mem.alloc(0x18);
        let steps = e.mem.alloc(4 * actions.len() as u32);
        for (i, (action, value)) in actions.iter().zip(values).enumerate() {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, *action);
            e.mem.set_u32(block + 0xc, *value);
            e.mem.set_u32(steps + 4 * i as u32, block);
        }
        e.mem.set_u32(this, steps);
        e.mem.set_u32(this + 4, actions.len() as u32);
        this
    }

    #[test]
    fn fn_00993d60_clears_the_plan() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcd; 0x18]);
        assert_eq!(e.call(0x0099_3d60, &args![this, 0x5000u32]).u32(), this);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0, 0, 0xffff_ffff, 0x5000]);
    }

    #[test]
    fn fn_00993db0_fills_steps_from_links_and_nodes() {
        let mut e = Engine::new();
        allocator(&mut e);
        // Links: an array of two link pointers; nodes: three node pointers. The doubles
        // read the element address as `base + 4 * index` and a node's key at `+0`.
        let links = e.mem.alloc(0x10);
        let nodes = e.mem.alloc(0x10);
        let link_items = e.mem.alloc(8);
        let node_items = e.mem.alloc(12);
        e.mem.set_u32(links + 4, link_items);
        e.mem.set_u32(links + 8, 2);
        e.mem.set_u32(nodes + 4, node_items);
        let mut node_ptrs = vec![];
        for key in [10u32, 11, 12] {
            let node = e.mem.alloc(4);
            e.mem.set_u32(node, key);
            node_ptrs.push(node);
        }
        for (i, node) in node_ptrs.iter().enumerate() {
            e.mem.set_u32(node_items + 4 * i as u32, *node);
        }
        // Link 0 (used by step 1): action 0xa0, parent node 12, location -1.
        // Link 1 (used by step 0): action 0xa1, parent node 11, location 7.
        for (i, (action, value, parent, location)) in [
            (0xa0u32, 100u32, node_ptrs[2], 0xffff_ffffu32),
            (0xa1, 101, node_ptrs[1], 7),
        ]
        .into_iter()
        .enumerate()
        {
            let link = e.mem.alloc(0x10);
            e.mem.set_u32(link, action);
            e.mem.set_u32(link + 4, parent);
            e.mem.set_u32(link + 8, value);
            e.mem.set_u32(link + 0xc, location);
            e.mem.set_u32(link_items + 4 * i as u32, link);
        }
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4).wrapping_add(4 * a[1]))
        });
        e.register(READ_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(NODE_KEY, |e, a| ret(e.mem.u32(a[0])));
        e.register(ORDER_STEPS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x18);
        e.call(
            0x0099_3db0,
            &args![this, 0x5000u32, 0x6000u32, nodes, links, 0x7000u32],
        );
        assert_eq!(e.mem.u32(this + 4), 2);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x6000);
        assert_eq!(e.mem.u32(this + 0x10), 7);
        assert_eq!(e.mem.u32(this + 0x14), 0x5000);
        let step0 = step(&e, Ptr::new(this), 0);
        let step1 = step(&e, Ptr::new(this), 1);
        // Step 0: link 1 and node 2 (key 12); step 1: link 0 and node 1 (key 11).
        assert_eq!(e.mem.u32(step0), 0xa1);
        assert_eq!(e.mem.u32(step0 + 0xc), 101);
        assert_eq!(e.mem.u32(step0 + 8), 12);
        assert_eq!(e.mem.u32(step1), 0xa0);
        assert_eq!(e.mem.u32(step1 + 8), 11);
        // Step 0's link's parent node has key 11: step 1. Step 1's has key 12: step 0.
        assert_eq!(e.mem.u32(step0 + 4), step1);
        assert_eq!(e.mem.u32(step1 + 4), step0);
        assert_eq!(calls(&e, ORDER_STEPS), vec![vec![this, 0x7000]]);
    }

    #[test]
    fn fn_00993f70_frees_steps_then_the_array() {
        let mut e = Engine::new();
        allocator(&mut e);
        let this = plan(&mut e, &[1, 2], &[0, 0]);
        let steps = e.mem.u32(this);
        let first = e.mem.u32(steps);
        let second = e.mem.u32(steps + 4);
        e.call_log = Some(vec![]);
        e.call(0x0099_3f70, &args![this]);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![first], vec![second], vec![steps]]
        );
    }

    #[test]
    fn fn_00993fd0_allocates_the_steps() {
        let mut e = Engine::new();
        allocator(&mut e);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x0099_3fd0, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(
            calls(&e, OPERATOR_NEW),
            vec![vec![12], vec![0x10], vec![0x10], vec![0x10]]
        );
        assert_ne!(step(&e, Ptr::new(this), 2), 0);
        assert_eq!(array_bytes(0x4000_0000), u32::MAX);
        assert_eq!(array_bytes(0x3fff_ffff), 0xffff_fffc);
    }

    /// An action object with a vtable whose slots +4 and +8 are `0x7100` and `0x7200`.
    fn action(e: &mut Engine, flags: u32) -> u32 {
        e.put_vtable(0x7000, &[0, 0x7100, 0x7200]);
        let object = e.mem.alloc(0x2c);
        e.mem.set_u32(object, 0x7000);
        e.mem.set_u32(object + 0x28, flags);
        object
    }

    fn plan_doubles(e: &mut Engine) {
        e.register(PLAN_IS_DONE, |e, a| {
            ret((e.mem.u32(a[0] + 8) >= e.mem.u32(a[0] + 4)) as u32)
        });
        for addr in [
            0x7100,
            CONTROLLER_RESET_00980250,
            CONTROLLER_SET_ACTION,
            CONTROLLER_SET_VALUE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00994050_skips_flagged_steps_and_starts_the_next() {
        let mut e = Engine::new();
        plan_doubles(&mut e);
        let flagged = action(&mut e, 2);
        let plain = action(&mut e, 0);
        let this = plan(&mut e, &[flagged, plain, plain], &[5, 6, 7]);
        e.register(CONTROLLER_ACTION, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_4050, &args![this, 0x9000u32]).bool());
        assert_eq!(
            calls(&e, 0x7100),
            vec![vec![flagged, 0x9000, 5], vec![plain, 0x9000, 6]]
        );
        assert_eq!(calls(&e, CONTROLLER_SET_ACTION), vec![vec![0x9000, plain]]);
        assert_eq!(calls(&e, CONTROLLER_SET_VALUE), vec![vec![0x9000, 6]]);
        assert_eq!(e.mem.u32(this + 8), 2);
    }

    #[test]
    fn fn_00994050_keeps_the_running_step() {
        let mut e = Engine::new();
        plan_doubles(&mut e);
        let plain = action(&mut e, 0);
        let this = plan(&mut e, &[plain], &[6]);
        e.mem.set_u32(0x7f00, plain);
        e.register(CONTROLLER_ACTION, |e, _| ret(e.mem.u32(0x7f00)));
        e.register(CONTROLLER_VALUE, |_, _| ret(6));
        e.register(0x7200, |_, _| ret(1));
        e.register(CONTROLLER_TEST_00980470, |_, _| ret(0));
        e.register(CONTROLLER_TEST_009804D0, |_, _| ret(0x100));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_4050, &args![this, 0x9000u32]).bool());
        assert!(calls(&e, 0x7100).is_empty());
        assert!(calls(&e, CONTROLLER_RESET_00980250).is_empty());
        assert_eq!(calls(&e, 0x7200), vec![vec![plain, 0x9000, 6]]);
        assert_eq!(e.mem.u32(this + 8), 1);
        // When 00980470 says true, the step is started again.
        e.mem.set_u32(this + 8, 0);
        e.register(CONTROLLER_TEST_00980470, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0099_4050, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, CONTROLLER_RESET_00980250).len(), 1);
        assert_eq!(calls(&e, 0x7100), vec![vec![plain, 0x9000, 6]]);
    }

    #[test]
    fn fn_00994250_tests_the_mask() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x2c);
        e.mem.set_u32(object + 0x28, 0b0110);
        assert!(e.call(0x0099_4250, &args![object, 2u32]).bool());
        assert!(!e.call(0x0099_4250, &args![object, 1u32]).bool());
    }
}
