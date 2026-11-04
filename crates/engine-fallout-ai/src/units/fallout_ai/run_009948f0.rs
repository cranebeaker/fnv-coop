//! `fallout/ai/run_009948f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The plan functions here work on the object `run_00993d60` builds (its words match the
//! Xbox PDB's `CombatPlan`: `pPlanSteps`, `iLength`, `iCurrentAction`, `pGoal`; steps
//! `CombatPlanStep`: `pAction`, `pParentStep`, `iNodeKey`, `iActionValue`), the node
//! functions on a 0xe4-byte object whose words match `CombatPlanNode` (a world-state array
//! at `+0`, `UnsatisfiedStates` pairs from `+0x5c`, their count at `+0xdc`, the key at
//! `+0xe0`), and the planner functions on an object whose PC words match
//! `CombatPlanner` 4 bytes lower than the Xbox PDB (`pCombatController` at `+0x2058`,
//! `InitialWorldState` at `+0x205c`, `PlannerHeap` at `+0x20b8`, `PlannerData` at
//! `+0x20c8`). The engine map names none of them, so they keep `fn_<addr>` names.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00994ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): stores `value` in the
/// word at +0xa4.
pub fn fn_00994ef0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xa4, value);
}

/// `00865e50` (unnamed, `bgssavegamebuffer.cpp`): `thiscall` on the save buffer, three
/// words (`RET 0xc`): the data, its size and 0.
const SAVE_DATA: u32 = 0x0086_5e50;
/// `00864980` (unnamed, `bgsloadgamebuffer.cpp`): `thiscall` on the load buffer, two
/// words (`RET 8`): the destination and its size.
const LOAD_DATA: u32 = 0x0086_4980;
/// `0059bb30` (named `D3DTexture_LockRect` in the engine map; a folded body):
/// `thiscall` on a step's action, a byte result (saved as the action's id).
const ACTION_SAVE_ID: u32 = 0x0059_bb30;
/// `00825c00` (unnamed): `thiscall` on the plan's goal, a byte result (saved as the
/// goal's id).
const GOAL_SAVE_ID: u32 = 0x0082_5c00;
/// `00982070` (unnamed, `combatcontroller.cpp`): `cdecl`, one word (an action id); the
/// action.
const ACTION_FOR_ID: u32 = 0x0098_2070;
/// `00993fd0` (unnamed, `run_00993d60`): `thiscall`, one word (`RET 4`); allocates the
/// plan's steps.
const ALLOCATE_STEPS: u32 = 0x0099_3fd0;
/// `0x011f17a0`: the table of goal pointers `fn_00994a80` reads.
const GOALS: u32 = 0x011f_17a0;
/// `005bd5c0` (unnamed, `tesscriptfunctions.cpp` per the engine map): `thiscall` on the
/// node's world-state array, one word (`RET 4`): the value of state `index`.
const WORLD_STATE_VALUE: u32 = 0x005b_d5c0;
/// `00559450` (unnamed): `thiscall`, returns the word at `+0`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `00726070` (unnamed, `menumanager.cpp` per the engine map): `thiscall`, no stack
/// words; a word result.
const ITEM_VALUE: u32 = 0x0072_6070;
/// `009aa490` (unnamed): `thiscall`, one word (`RET 4`); copies a world-state array.
const COPY_WORLD_STATE: u32 = 0x009a_a490;
/// `009aa4e0` (`CombatWorldStateArray::Update`, Xbox PDB): `thiscall`, one word (`RET 4`).
const WORLD_STATE_UPDATE: u32 = 0x009a_a4e0;
/// `009aadc0` (`CombatWorldStateArray::IsTrue`, Xbox PDB): `thiscall`, two words
/// (`RET 8`).
const WORLD_STATE_IS_TRUE: u32 = 0x009a_adc0;
/// `0048d150` (unnamed): `thiscall`, the goal's list (its first node).
const GOAL_LIST: u32 = 0x0048_d150;
/// `006815c0` (unnamed): `thiscall`, returns `this` (a list node's item slot).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `0062d990` (unnamed): `thiscall`, the next list node.
const NODE_NEXT: u32 = 0x0062_d990;
/// `00994830` (unnamed, `run_009942d0`): `cdecl`, one word; whether the value needs
/// resolving.
const NEEDS_RESOLVING: u32 = 0x0099_4830;
/// `00984470` (unnamed, `combatgoal.cpp`): `thiscall`, one word (`RET 4`).
const RESOLVE_VALUE: u32 = 0x0098_4470;
/// `ScrapHeap::Allocate` (`00aa54a0`, Xbox PDB): `thiscall`, size and alignment
/// (`RET 8`).
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
/// The alignment word `fn_00994bd0` passes to `ScrapHeap::Allocate` (read from
/// `0x010a2720`).
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `006e6da0` (unnamed): `cdecl`, size and block; returns the block (placement).
const PLACE_BLOCK: u32 = 0x006e_6da0;
/// `004b0680` (unnamed, `bsanimnotereceiver.cpp` per the engine map): `thiscall`, two
/// words (`RET 8`); constructs the 8-byte unsatisfied state.
const CONSTRUCT_STATE: u32 = 0x004b_0680;
/// `006a7c40` (unnamed, `navmeshsearch.cpp`): `thiscall`, no stack words; resets the
/// search.
const RESET_SEARCH: u32 = 0x006a_7c40;
/// `006b8ec0` (unnamed, `navmeshinfosearch.cpp`): `thiscall`, three words (`RET 0xc`):
/// the start key, the start node pointer's address and a `float`.
const START_SEARCH: u32 = 0x006b_8ec0;
/// `009964c0` / `00996560` (unnamed, `combatplanner.cpp`): constructor and destructor of
/// the 0x14-byte node array (`ECX` only).
const NODE_ARRAY_CONSTRUCT: u32 = 0x0099_64c0;
const NODE_ARRAY_DESTRUCT: u32 = 0x0099_6560;
/// `009965c0` / `00996660` (unnamed, `combatplanner.cpp`): constructor and destructor of
/// the 0x14-byte link array (`ECX` only).
const LINK_ARRAY_CONSTRUCT: u32 = 0x0099_65c0;
const LINK_ARRAY_DESTRUCT: u32 = 0x0099_6660;
/// `00996420` (unnamed, `combatplanner.cpp`): `thiscall`, three words (`RET 0xc`): fills
/// the node and link arrays.
const RUN_SEARCH: u32 = 0x0099_6420;
/// `0044ddc0` (unnamed): `thiscall`, returns the word at `+8` (an array's count).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `operator new` (`00401000`, `cdecl`, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00993db0` (unnamed, `run_00993d60`): the plan constructor (`thiscall`, `RET 0x14`).
const PLAN_CONSTRUCT: u32 = 0x0099_3db0;
/// `009959f0` (unnamed, `combatplanner.cpp`): the planner debug window's constructor
/// (`thiscall`, `RET 0xc`).
const DEBUG_WINDOW_CONSTRUCT: u32 = 0x0099_59f0;

/// Whether a list node exists and holds an item (`node != 0 && *006815c0(node) != 0`).
fn has_item(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let item_slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(item_slot) != 0
}

// Translated from 009948f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): saves the plan into `buffer`
/// (`00865e50`): the length, then for each step its action's id byte (`0059bb30`) and
/// its value (`+0xc`), then the current step and the goal's id byte (`00825c00`).
pub fn fn_009948f0(e: &mut Engine, this: Ptr, buffer: u32) {
    let p = this.addr();
    e.call(SAVE_DATA, &args![buffer, p + 0x4, 4u32, 0u32]);
    let mut index = 0u32;
    while index < e.mem.u32(p + 0x4) {
        let steps = e.mem.u32(p);
        let current = e.mem.u32(steps.wrapping_add(index.wrapping_mul(4)));
        let action = e.mem.u32(current);
        let id = e.call(ACTION_SAVE_ID, &args![action]).u8();
        e.with_stack(1, |e, byte| {
            e.mem.set_u8(byte.addr(), id);
            e.call(SAVE_DATA, &args![buffer, byte, 1u32, 0u32]);
        });
        e.call(SAVE_DATA, &args![buffer, current + 0xc, 4u32, 0u32]);
        index = index.wrapping_add(1);
    }
    e.call(SAVE_DATA, &args![buffer, p + 0x8, 4u32, 0u32]);
    let goal = e.mem.u32(p + 0xc);
    let id = e.call(GOAL_SAVE_ID, &args![goal]).u8();
    e.with_stack(1, |e, byte| {
        e.mem.set_u8(byte.addr(), id);
        e.call(SAVE_DATA, &args![buffer, byte, 1u32, 0u32]);
    });
}

// Translated from 009949a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): loads the plan from `buffer`
/// (`00864980`): the length (and allocates that many steps, `00993fd0`), then for each
/// step an action id byte (its action, `00982070`) and the value (`+0xc`), clearing the
/// parent and key; then the current step and a goal id byte (its goal, `fn_00994a80`).
pub fn fn_009949a0(e: &mut Engine, this: Ptr, buffer: u32) {
    let p = this.addr();
    e.call(LOAD_DATA, &args![buffer, p + 0x4, 4u32]);
    let length = e.mem.u32(p + 0x4);
    e.call(ALLOCATE_STEPS, &args![this, length]);
    let mut index = 0u32;
    while index < e.mem.u32(p + 0x4) {
        let steps = e.mem.u32(p);
        let current = e.mem.u32(steps.wrapping_add(index.wrapping_mul(4)));
        let id = e.with_stack(1, |e, byte| {
            e.mem.set_u8(byte.addr(), 0);
            e.call(LOAD_DATA, &args![buffer, byte, 1u32]);
            e.call(LOAD_DATA, &args![buffer, current + 0xc, 4u32]);
            e.mem.u8(byte.addr())
        });
        let action = e.call(ACTION_FOR_ID, &args![u32::from(id)]).u32();
        e.mem.set_u32(current, action);
        e.mem.set_u32(current + 0x4, 0);
        e.mem.set_u32(current + 0x8, 0);
        index = index.wrapping_add(1);
    }
    e.call(LOAD_DATA, &args![buffer, p + 0x8, 4u32]);
    let id = e.with_stack(1, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        e.call(LOAD_DATA, &args![buffer, byte, 1u32]);
        e.mem.u8(byte.addr())
    });
    let goal = fn_00994a80(e, u32::from(id));
    e.mem.set_u32(p + 0xc, goal);
}

// Translated from 00994a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): goal `index` of the table at
/// `0x011f17a0` (no bounds check).
pub fn fn_00994a80(e: &mut Engine, index: u32) -> u32 {
    e.mem.u32(index.wrapping_mul(4).wrapping_add(GOALS))
}

// Translated from 00994a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the node's key (`+0xe0`), computed once when it
/// is -1: from `0x811c9dc5`, mixed (`fn_00994bb0`) with each of the 20 world-state values
/// (`005bd5c0`), the unsatisfied-state count (`+0xdc`), and each unsatisfied state's first
/// word (`00559450`) and value (`00726070`).
pub fn fn_00994a90(e: &mut Engine, this: Ptr) -> u32 {
    let p = this.addr();
    if e.mem.u32(p + 0xe0) != 0xffff_ffff {
        return e.mem.u32(p + 0xe0);
    }
    e.mem.set_u32(p + 0xe0, 0x811c_9dc5);
    let mut index = 0u32;
    while index < 0x14 {
        let key = e.mem.u32(p + 0xe0);
        let value = e.call(WORLD_STATE_VALUE, &args![this, index]).u32();
        let key = fn_00994bb0(e, value, key);
        e.mem.set_u32(p + 0xe0, key);
        index = index.wrapping_add(1);
    }
    let key = e.mem.u32(p + 0xe0);
    let count = e.mem.u32(p + 0xdc);
    let key = fn_00994bb0(e, count, key);
    e.mem.set_u32(p + 0xe0, key);
    let mut index = 0u32;
    while index < e.mem.u32(p + 0xdc) {
        let state = e.mem.u32(p.wrapping_add(index.wrapping_mul(8)) + 0x5c);
        let key = e.mem.u32(p + 0xe0);
        let word = e.call(READ_FIRST_WORD, &args![state]).u32();
        let key = fn_00994bb0(e, word, key);
        e.mem.set_u32(p + 0xe0, key);
        let key = e.mem.u32(p + 0xe0);
        let value = e.call(ITEM_VALUE, &args![state]).u32();
        let key = fn_00994bb0(e, value, key);
        e.mem.set_u32(p + 0xe0, key);
        index = index.wrapping_add(1);
    }
    e.mem.u32(p + 0xe0)
}

// Translated from 00994bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): `(hash * 0x1000193) ^ value` (one FNV-1
/// step on a whole word).
pub fn fn_00994bb0(_e: &mut Engine, value: u32, hash: u32) -> u32 {
    hash.wrapping_mul(0x0100_0193) ^ value
}

// Translated from 00994bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00994bd0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 8`): plans for `controller`. Stores it at
/// `+0x2058`, gives it `+0x20c8` (`00994ef0`) and updates the initial world state
/// (`+0x205c`, `009aa4e0`). Then for each of the 16 goals (`fn_00994a80`) whose slot `+0`
/// accepts the world state: a start node (a 0xe4-byte stack local, `fn_00994e60`) gets an
/// unsatisfied state (8 bytes from the planner's scrap heap at `+0x20b8`, `006e6da0`,
/// `004b0680`; null when the placement gives null) for each item of the goal's list
/// (`0048d150`) whose state (`00559450`) and value (`00726070`, resolved by `00984470`
/// on the node when `00994830` says so) the node's world state does not hold
/// (`009aadc0`); `fn_00994f10` searches from it with `make_debug_window`. The first plan
/// found is returned; otherwise the goal's slot `+4` is called with `controller` and the
/// world state. Returns 0 when no goal gives a plan.
pub fn fn_00994bd0(e: &mut Engine, this: Ptr, controller: u32, make_debug_window: u8) -> u32 {
    let p = this.addr();
    e.mem.set_u32(p + 0x2058, controller);
    let data = p + 0x20c8;
    let stored = e.mem.u32(p + 0x2058);
    fn_00994ef0(e, Ptr::new(stored), data);
    e.call(WORLD_STATE_UPDATE, &args![p + 0x205c, controller]);
    let mut index = 0u32;
    while index < 0x10 {
        let goal = fn_00994a80(e, index);
        let accepts = e.vcall(goal, 0x0, &args![p + 0x205c]).u8() != 0;
        if accepts {
            let plan = e.with_stack(0xe4, |e, node| {
                fn_00994e60(e, node, p + 0x205c);
                let mut list = e.call(GOAL_LIST, &args![goal]).u32();
                while has_item(e, list) {
                    let item_slot = e.call(NODE_ITEM, &args![list]).u32();
                    let item = e.mem.u32(item_slot);
                    let mut value = e.call(ITEM_VALUE, &args![item]).u32();
                    if e.call(NEEDS_RESOLVING, &args![value]).u8() != 0 {
                        value = e.call(RESOLVE_VALUE, &args![node, value]).u32();
                    }
                    let state = e.call(READ_FIRST_WORD, &args![item]).u32();
                    if e.call(WORLD_STATE_IS_TRUE, &args![node, state, value]).u8() == 0 {
                        let alignment = e.mem.u32(SCRAP_ALIGNMENT);
                        let block = e
                            .call(SCRAP_HEAP_ALLOCATE, &args![p + 0x20b8, 8u32, alignment])
                            .u32();
                        let placed = e.call(PLACE_BLOCK, &args![8u32, block]).u32();
                        let unsatisfied = if placed == 0 {
                            0
                        } else {
                            let state = e.call(READ_FIRST_WORD, &args![item]).u32();
                            e.call(CONSTRUCT_STATE, &args![placed, state, value]).u32()
                        };
                        fn_00994ea0(e, node, unsatisfied, node.addr());
                    }
                    list = e.call(NODE_NEXT, &args![list]).u32();
                }
                fn_00994f10(e, this, controller, goal, node.addr(), make_debug_window)
            });
            if plan != 0 {
                return plan;
            }
            e.vcall(goal, 0x4, &args![controller, p + 0x205c]);
        }
        index = index.wrapping_add(1);
    }
    0
}

// Translated from 00994e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): makes `this` a node of `world_state`
/// (`009aa490` copies it), with no unsatisfied states and key -1; returns `this`.
pub fn fn_00994e60(e: &mut Engine, this: Ptr, world_state: u32) -> Ptr {
    e.call(COPY_WORLD_STATE, &args![this, world_state]);
    e.mem.set_u32(this.addr() + 0xdc, 0);
    e.mem.set_u32(this.addr() + 0xe0, 0xffff_ffff);
    this
}

// Translated from 00994ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): appends the pair (`state`, `parent`)
/// to the node's unsatisfied states (`+0x5c`, 8 bytes each, count at `+0xdc`; no bounds
/// check).
pub fn fn_00994ea0(e: &mut Engine, this: Ptr, state: u32, parent: u32) {
    let p = this.addr();
    let count = e.mem.u32(p + 0xdc);
    e.mem
        .set_u32(p.wrapping_add(count.wrapping_mul(8)) + 0x5c, state);
    let count = e.mem.u32(p + 0xdc);
    e.mem
        .set_u32(p.wrapping_add(count.wrapping_mul(8)) + 0x60, parent);
    let count = e.mem.u32(p + 0xdc);
    e.mem.set_u32(p + 0xdc, count.wrapping_add(1));
}

// Translated from 00994f10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00994f10 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 0x10`): searches a plan for `goal` from the
/// node `start`; 0 at once when the node has no unsatisfied state. Resets the search
/// (`006a7c40`), starts it from the node's key (`fn_009950a0`) with the address of the
/// `start` argument and 0.0 (`006b8ec0`), and runs it (`00996420`) into a node array and
/// a link array (0x14-byte stack locals, `009964c0`/`009965c0`). When links were found a
/// plan is made (`operator new(0x18)`, `00993db0` with `controller`, `goal`, both arrays
/// and the world state at `+0x205c`; null when the allocation fails). With
/// `make_debug_window` a debug window is made too (`operator new(0x3c)`, `009959f0`; the
/// pointer is not kept). The search is reset again and the arrays destroyed
/// (`00996660`, `00996560`). Returns the plan.
pub fn fn_00994f10(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    goal: u32,
    start: u32,
    make_debug_window: u8,
) -> u32 {
    if e.mem.u32(start + 0xdc) == 0 {
        return 0;
    }
    let p = this.addr();
    e.call(RESET_SEARCH, &args![this]);
    // The `start` argument's own stack slot, whose address the search is given.
    e.with_stack(0x2c, |e, frame| {
        let start_slot = frame.addr();
        let nodes = frame.addr() + 0x4;
        let links = frame.addr() + 0x18;
        e.mem.set_u32(start_slot, start);
        let key = fn_009950a0(e, this, start_slot);
        e.call(
            START_SEARCH,
            &args![this, key, start_slot, 0.0f32.to_bits()],
        );
        e.call(NODE_ARRAY_CONSTRUCT, &args![nodes]);
        e.call(LINK_ARRAY_CONSTRUCT, &args![links]);
        e.call(RUN_SEARCH, &args![this, nodes, links, 0u32]);
        let mut plan = 0u32;
        if e.call(ARRAY_COUNT, &args![links]).u32() != 0 {
            let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
            plan = if block == 0 {
                0
            } else {
                e.call(
                    PLAN_CONSTRUCT,
                    &args![block, controller, goal, nodes, links, p + 0x205c],
                )
                .u32()
            };
        }
        if make_debug_window != 0 {
            let block = e.call(OPERATOR_NEW, &args![0x3cu32]).u32();
            if block != 0 {
                e.call(DEBUG_WINDOW_CONSTRUCT, &args![block, this, plan, nodes]);
            }
        }
        e.call(RESET_SEARCH, &args![this]);
        e.call(LINK_ARRAY_DESTRUCT, &args![links]);
        e.call(NODE_ARRAY_DESTRUCT, &args![nodes]);
        plan
    })
}

// Translated from 009950a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` unread): the key
/// (`fn_00994a90`) of the node `*node`.
pub fn fn_009950a0(e: &mut Engine, _this: Ptr, node: u32) -> u32 {
    let node = e.mem.u32(node);
    fn_00994a90(e, Ptr::new(node))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_4ef0, fn_00994ef0(Ptr, u32)),
        entry!(0x0099_48f0, fn_009948f0(Ptr, u32)),
        entry!(0x0099_49a0, fn_009949a0(Ptr, u32)),
        entry!(0x0099_4a80, fn_00994a80(u32) -> u32),
        entry!(0x0099_4a90, fn_00994a90(Ptr) -> u32),
        entry!(0x0099_4bb0, fn_00994bb0(u32, u32) -> u32),
        entry!(0x0099_4bd0, fn_00994bd0(Ptr, u32, u8) -> u32),
        entry!(0x0099_4e60, fn_00994e60(Ptr, u32) -> Ptr),
        entry!(0x0099_4ea0, fn_00994ea0(Ptr, u32, u32)),
        entry!(0x0099_4f10, fn_00994f10(Ptr, u32, u32, u32, u8) -> u32),
        entry!(0x0099_50a0, fn_009950a0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00994ef0_stores_the_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xac);
        e.call(0x0099_4ef0, &args![object, 0xcafe_f00du32]);
        assert_eq!(e.mem.u32(object + 0xa4), 0xcafe_f00d);
        assert_eq!(e.mem.u32(object + 0xa0), 0);
        assert_eq!(e.mem.u32(object + 0xa8), 0);
    }
}

#[cfg(test)]
mod plan_tests {
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

    /// A plan with one step per `(action, value)`.
    fn plan(e: &mut Engine, steps: &[(u32, u32)]) -> u32 {
        let this = e.mem.alloc(0x18);
        let array = e.mem.alloc(4 * steps.len() as u32 + 4);
        for (i, (action, value)) in steps.iter().enumerate() {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, *action);
            e.mem.set_u32(block + 0xc, *value);
            e.mem.set_u32(array + 4 * i as u32, block);
        }
        e.mem.set_u32(this, array);
        e.mem.set_u32(this + 4, steps.len() as u32);
        this
    }

    #[test]
    fn fn_009948f0_saves_length_steps_current_and_goal() {
        let mut e = Engine::new();
        // The double records each saved block (as a word) in order at 0x7f04.
        e.map(0x7f00, 0x100);
        e.register(SAVE_DATA, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            let mut word = 0u32;
            for (i, b) in bytes.iter().enumerate() {
                word |= u32::from(*b) << (8 * i);
            }
            let n = e.mem.u32(0x7f00);
            e.mem.set_u32(0x7f04 + 4 * n, word);
            e.mem.set_u32(0x7f00, n + 1);
            Ret::default()
        });
        e.register(ACTION_SAVE_ID, |e, a| ret(e.mem.u32(a[0]) | 0xff00));
        e.register(GOAL_SAVE_ID, |_, _| ret(0x1234_5609));
        let first = e.mem.alloc(4);
        e.mem.set_u32(first, 3);
        let second = e.mem.alloc(4);
        e.mem.set_u32(second, 4);
        let this = plan(&mut e, &[(first, 30), (second, 40)]);
        e.mem.set_u32(this + 8, 1);
        e.mem.set_u32(this + 0xc, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0099_48f0, &args![this, 0x6000u32]);
        let saved: Vec<u32> = (0..e.mem.u32(0x7f00))
            .map(|i| e.mem.u32(0x7f04 + 4 * i))
            .collect();
        assert_eq!(saved, vec![2, 3, 30, 4, 40, 1, 9]);
        assert!(calls(&e, SAVE_DATA)
            .iter()
            .all(|call| call[0] == 0x6000 && call[3] == 0));
        assert_eq!(calls(&e, GOAL_SAVE_ID), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_009949a0_loads_the_plan() {
        let mut e = Engine::new();
        // Each load reads the next word of the script at 0x7f04 (as many bytes as asked).
        e.map(0x7f00, 0x100);
        for (i, word) in [2u32, 5, 50, 6, 60, 1, 3].iter().enumerate() {
            e.mem.set_u32(0x7f04 + 4 * i as u32, *word);
        }
        e.register(LOAD_DATA, |e, a| {
            let n = e.mem.u32(0x7f00);
            let bytes = e.mem.bytes(0x7f04 + 4 * n, a[2]);
            e.mem.write(a[1], &bytes);
            e.mem.set_u32(0x7f00, n + 1);
            Ret::default()
        });
        e.register(ALLOCATE_STEPS, |e, a| {
            let array = e.mem.alloc(4 * a[1]);
            for i in 0..a[1] {
                let block = e.mem.alloc(0x10);
                e.mem.write(block, &[0xcd; 0x10]);
                e.mem.set_u32(array + 4 * i, block);
            }
            e.mem.set_u32(a[0], array);
            Ret::default()
        });
        e.register(ACTION_FOR_ID, |_, a| ret(0xa000 + a[0]));
        e.map(GOALS, 0x40);
        e.mem.set_u32(GOALS + 12, 0x5003);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x0099_49a0, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u32(this + 4), 2);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0x5003);
        let array = e.mem.u32(this);
        let step = e.mem.u32(array + 4);
        assert_eq!(
            (0..4).map(|i| e.mem.u32(step + 4 * i)).collect::<Vec<_>>(),
            vec![0xa006, 0, 0, 60]
        );
        assert_eq!(calls(&e, ALLOCATE_STEPS), vec![vec![this, 2]]);
        assert_eq!(calls(&e, ACTION_FOR_ID), vec![vec![5], vec![6]]);
    }

    #[test]
    fn fn_00994a80_reads_the_goal_table() {
        let mut e = Engine::new();
        e.map(GOALS, 0x40);
        e.mem.set_u32(GOALS + 8, 0x1234);
        assert_eq!(e.call(0x0099_4a80, &args![2u32]).u32(), 0x1234);
    }

    #[test]
    fn fn_00994a90_hashes_once() {
        let mut e = Engine::new();
        e.register(WORLD_STATE_VALUE, |_, a| ret(a[1] * 3));
        e.register(READ_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(ITEM_VALUE, |e, a| ret(e.mem.u32(a[0] + 4)));
        let node = e.mem.alloc(0xe4);
        let state = e.mem.alloc(8);
        e.mem.set_u32(state, 7);
        e.mem.set_u32(state + 4, 9);
        e.mem.set_u32(node + 0x5c, state);
        e.mem.set_u32(node + 0xdc, 1);
        e.mem.set_u32(node + 0xe0, 0xffff_ffff);
        let mut want = 0x811c_9dc5u32;
        for i in 0..0x14u32 {
            want = want.wrapping_mul(0x0100_0193) ^ (i * 3);
        }
        for value in [1u32, 7, 9] {
            want = want.wrapping_mul(0x0100_0193) ^ value;
        }
        assert_eq!(e.call(0x0099_4a90, &args![node]).u32(), want);
        // Once known, the key is returned without asking again.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_4a90, &args![node]).u32(), want);
        assert!(calls(&e, WORLD_STATE_VALUE).is_empty());
    }

    #[test]
    fn fn_00994bb0_mixes_one_word() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0099_4bb0, &args![5u32, 0x811c_9dc5u32]).u32(),
            0x811c_9dc5u32.wrapping_mul(0x0100_0193) ^ 5
        );
    }

    #[test]
    fn fn_00994e60_and_fn_00994ea0_build_a_node() {
        let mut e = Engine::new();
        e.register(COPY_WORLD_STATE, |_, _| Ret::default());
        let node = e.mem.alloc(0xe4);
        e.mem.set_u32(node + 0xdc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_4e60, &args![node, 0x4000u32]).u32(), node);
        assert_eq!(calls(&e, COPY_WORLD_STATE), vec![vec![node, 0x4000]]);
        assert_eq!(e.mem.u32(node + 0xdc), 0);
        assert_eq!(e.mem.u32(node + 0xe0), 0xffff_ffff);
        e.call(0x0099_4ea0, &args![node, 0x11u32, 0x22u32]);
        e.call(0x0099_4ea0, &args![node, 0x33u32, 0x44u32]);
        assert_eq!(e.mem.u32(node + 0xdc), 2);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(node + 0x5c + 4 * i))
                .collect::<Vec<_>>(),
            vec![0x11, 0x22, 0x33, 0x44]
        );
    }

    fn search_doubles(e: &mut Engine) {
        for addr in [
            RESET_SEARCH,
            START_SEARCH,
            NODE_ARRAY_CONSTRUCT,
            LINK_ARRAY_CONSTRUCT,
            RUN_SEARCH,
            LINK_ARRAY_DESTRUCT,
            NODE_ARRAY_DESTRUCT,
            DEBUG_WINDOW_CONSTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
    }

    #[test]
    fn fn_00994f10_makes_a_plan_when_links_were_found() {
        let mut e = Engine::new();
        search_doubles(&mut e);
        e.register(PLAN_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ARRAY_COUNT, |_, _| ret(2));
        let planner = e.mem.alloc(0x21e0);
        let node = e.mem.alloc(0xe4);
        e.mem.set_u32(node + 0xdc, 1);
        e.mem.set_u32(node + 0xe0, 0x4b);
        e.call_log = Some(vec![]);
        let plan = e
            .call(
                0x0099_4f10,
                &args![planner, 0x9000u32, 0x5000u32, node, 0x101u32],
            )
            .u32();
        assert_ne!(plan, 0);
        let construct = calls(&e, PLAN_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        assert_eq!(&construct[0][..3], &[plan, 0x9000, 0x5000]);
        assert_eq!(construct[0][5], planner + 0x205c);
        let start = calls(&e, START_SEARCH);
        assert_eq!(start[0][0], planner);
        assert_eq!(start[0][1], 0x4b);
        assert_eq!(start[0][3], 0);
        let window = calls(&e, DEBUG_WINDOW_CONSTRUCT);
        assert_eq!(window[0][1..3], [planner, plan]);
        assert_eq!(window[0][3], construct[0][3]);
        assert_eq!(calls(&e, RESET_SEARCH).len(), 2);
        assert_eq!(calls(&e, NODE_ARRAY_DESTRUCT), vec![vec![construct[0][3]]]);
    }

    #[test]
    fn fn_00994f10_without_links_or_states() {
        let mut e = Engine::new();
        search_doubles(&mut e);
        e.register(PLAN_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ARRAY_COUNT, |_, _| ret(0));
        let planner = e.mem.alloc(0x21e0);
        let node = e.mem.alloc(0xe4);
        e.call_log = Some(vec![]);
        let none = e.call(0x0099_4f10, &args![planner, 0u32, 0u32, node, 0u32]);
        assert_eq!(none.u32(), 0);
        assert!(calls(&e, RESET_SEARCH).is_empty());
        e.mem.set_u32(node + 0xdc, 1);
        let none = e.call(0x0099_4f10, &args![planner, 0u32, 0u32, node, 0u32]);
        assert_eq!(none.u32(), 0);
        assert!(calls(&e, PLAN_CONSTRUCT).is_empty());
        assert!(calls(&e, DEBUG_WINDOW_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, RESET_SEARCH).len(), 2);
    }

    #[test]
    fn fn_009950a0_keys_the_pointed_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0xe4);
        e.mem.set_u32(node + 0xe0, 0x77);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, node);
        assert_eq!(e.call(0x0099_50a0, &args![0u32, slot]).u32(), 0x77);
    }

    #[test]
    fn fn_00994bd0_returns_the_first_plan() {
        let mut e = Engine::new();
        search_doubles(&mut e);
        // Goal 0 refuses the world state; goals 1 and 2 accept it and share one item whose
        // state is not true, so each start node gets one unsatisfied state. Goal 1's
        // search gives no plan, so its slot +4 is called; goal 2's gives 0x5555.
        e.map(GOALS, 0x40);
        e.put_vtable(0x7000, &[0x7100, 0x7200]);
        e.put_vtable(0x7010, &[0x7110, 0x7200]);
        let goals: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x10)).collect();
        e.mem.set_u32(goals[0], 0x7000);
        e.mem.set_u32(goals[1], 0x7010);
        e.mem.set_u32(goals[2], 0x7010);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, 0x21);
        e.mem.set_u32(item + 4, 0xffff_0017);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, item);
        for (i, goal) in goals.iter().enumerate() {
            e.mem.set_u32(GOALS + 4 * i as u32, *goal);
            e.mem.set_u32(goal + 8, list);
        }
        e.register(0x7100, |_, _| ret(0));
        e.register(0x7110, |_, _| ret(1));
        e.register(0x7200, |_, _| Ret::default());
        e.register(WORLD_STATE_UPDATE, |_, _| Ret::default());
        e.register(COPY_WORLD_STATE, |_, _| Ret::default());
        e.register(GOAL_LIST, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(ITEM_VALUE, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NEEDS_RESOLVING, |_, a| ret((a[0] == 0xffff_0017) as u32));
        e.register(RESOLVE_VALUE, |_, _| ret(0x31));
        e.register(READ_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(WORLD_STATE_IS_TRUE, |_, _| ret(0));
        e.map(SCRAP_ALIGNMENT, 4);
        e.mem.set_u32(SCRAP_ALIGNMENT, 4);
        e.register(SCRAP_HEAP_ALLOCATE, |e, _| ret(e.mem.alloc(8)));
        e.register(PLACE_BLOCK, |_, a| ret(a[1]));
        e.register(CONSTRUCT_STATE, |_, a| ret(a[0]));
        e.register(ARRAY_COUNT, |_, _| ret(1));
        e.register(WORLD_STATE_VALUE, |_, _| ret(0));
        e.register(PLAN_CONSTRUCT, |e, a| {
            ret(if a[2] == e.mem.u32(GOALS + 8) {
                0x5555
            } else {
                0
            })
        });
        let planner = e.mem.alloc(0x21e0);
        let controller = e.mem.alloc(0xac);
        e.call_log = Some(vec![]);
        let plan = e.call(0x0099_4bd0, &args![planner, controller, 0u32]).u32();
        assert_eq!(plan, 0x5555);
        assert_eq!(e.mem.u32(planner + 0x2058), controller);
        assert_eq!(e.mem.u32(controller + 0xa4), planner + 0x20c8);
        assert_eq!(
            calls(&e, WORLD_STATE_UPDATE),
            vec![vec![planner + 0x205c, controller]]
        );
        // The item's resolved value is 0x31 and its state 0x21, once per accepting goal.
        let constructed = calls(&e, CONSTRUCT_STATE);
        assert_eq!(constructed.len(), 2);
        assert_eq!(constructed[0][1..], [0x21, 0x31]);
        assert_eq!(
            calls(&e, 0x7200),
            vec![vec![goals[1], controller, planner + 0x205c]]
        );
        assert_eq!(
            calls(&e, SCRAP_HEAP_ALLOCATE)[0],
            vec![planner + 0x20b8, 8, 4]
        );
        assert_eq!(calls(&e, PLAN_CONSTRUCT).len(), 2);
    }
}
