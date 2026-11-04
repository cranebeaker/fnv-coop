//! `fallout/ai/combat/combatplanner.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00995870` calls on the member (`00aa6330`).
const CALLEE_00AA6330: u32 = 0x00aa_6330;

/// The function `fn_00995890` calls on the member (`00aa62e0`).
const CALLEE_00AA62E0: u32 = 0x00aa_62e0;

// Translated from 00995870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00aa6330` with `ECX` = `this + 0x14` and passes on its `bool` result (`AL`).
pub fn fn_00995870(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_00AA6330, &args![this.byte_add(0x14)]).bool()
}

// Translated from 00995890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00aa62e0` with `ECX` = `this + 0x14` and passes on nothing (the callee returns no value).
pub fn fn_00995890(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00AA62E0, &args![this.byte_add(0x14)]);
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DC80: u32 = 0x0108_dc80;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanLink@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DCA8: u32 = 0x0108_dca8;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PAVCombatPlanNode@@PAVCombatPlanLink@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108DCBC: u32 = 0x0108_dcbc;

// Translated from 00996540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00996540(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DC80);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 00996640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCombatPlanLink@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00996640(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCA8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009966f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PAVCombatPlanNode@@PAVCombatPlanLink@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009966f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCBC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// ---- Batch b012 --------------------------------------------------------------------------

/// `00719b20` (`thiscall`, `RET 0xc`; engine-map unit `interfacemanager.cpp`): a search
/// answering an index or -1, given the key, 0 and the compare function `009a3830`.
const FIND_INDEX: u32 = 0x0071_9b20;
const COMPARE_009A3830: u32 = 0x009a_3830;
/// `ScrapHeap::Allocate` (`00aa54a0`, `thiscall`, `RET 8`: size, alignment) and
/// `ScrapHeap::Deallocate` (`00aa5610`, `thiscall`, `RET 4`), on the scrap heap a
/// `BSScrapArray` keeps at `+0x10`; the alignment word they pass (`010a2720`).
const SCRAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_DEALLOCATE: u32 = 0x00aa_5610;
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `0044ddc0` (`thiscall`, no stack word): the array's element count; `00401460` (`cdecl`,
/// `(to, from, bytes)`): the copy `_Reallocate` makes.
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const COPY_BYTES: u32 = 0x0040_1460;
/// `00403d30` (`cdecl`, `(block, value, bytes)`; Ghidra names it `_Fiopen`): fills a block.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `BSSimpleArray<TESObjectWEAP *,1024>::SetSize` (engine map, `006f2290`, `thiscall`,
/// `RET 8`) and `00877a30` (`thiscall`, `RET 4`: the address of an element).
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;

// Translated from 009962f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `00719b20(key, 0, 009a3830)` on
/// `this` finds an index (anything but -1).
pub fn fn_009962f0(e: &mut Engine, this: Ptr, key: u32) -> bool {
    e.call(FIND_INDEX, &args![this, key, 0u32, COMPARE_009A3830])
        .u32()
        != 0xffff_ffff
}

// Translated from 006f2870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanNode *,1024>::_Allocate` (Xbox PDB name from the engine map;
/// `thiscall`, `RET 4`): `count * 4` bytes from the array's scrap heap (`+0x10`) with the
/// alignment word at `010a2720`.
pub fn bs_scrap_array_combat_plan_node_allocate(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let alignment = e.mem.u32(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_ALLOCATE, &args![heap, count << 2, alignment])
        .u32()
}

// Translated from 006cf4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanNode *,1024>::_Deallocate` (Xbox PDB name from the engine map;
/// `thiscall`, `RET 4`): gives `block` back to the array's scrap heap (`+0x10`).
pub fn bs_scrap_array_combat_plan_node_deallocate(e: &mut Engine, this: Ptr, block: u32) {
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_DEALLOCATE, &args![heap, block]);
}

// Translated from 006cfc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanNode *,1024>::_Reallocate` (Xbox PDB name from the engine map;
/// `thiscall`, `RET 8`): a block of `count * 4` bytes from the scrap heap, the array's
/// elements (`0044ddc0` of them, 4 bytes each) copied from `old` (`00401460`), `old` given
/// back, and the new block answered.
pub fn bs_scrap_array_combat_plan_node_reallocate(
    e: &mut Engine,
    this: Ptr,
    old: u32,
    count: u32,
) -> u32 {
    let alignment = e.mem.u32(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + 0x10);
    let block = e
        .call(SCRAP_ALLOCATE, &args![heap, count << 2, alignment])
        .u32();
    let elements = e.call(ARRAY_COUNT, &args![this]).u32();
    e.call(COPY_BYTES, &args![block, old, elements << 2]);
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_DEALLOCATE, &args![heap, old]);
    block
}

// Translated from 00996840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, `this` not read): a hash, `key % 0x7f7`.
pub fn fn_00996840(_e: &mut Engine, _this: Ptr, key: u32) -> u32 {
    key % 0x7f7
}

// Translated from 00996800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): when the count word at `+0x1fdc` is not 0,
/// clears the 0x1fdc bytes before it (`00403d30(this, 0, 0x1fdc)`) and the count.
pub fn fn_00996800(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x1fdc) != 0 {
        e.call(FILL_BYTES, &args![this, 0u32, 0x1fdcu32]);
        e.mem.set_u32(this.addr() + 0x1fdc, 0);
    }
}

// Translated from 00996ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`, `this` not read): turns the chain of
/// path nodes from `node` (each linked to the next at `+0x10`) into two arrays, back to
/// front: `nodes` sized to the chain length and `links` to one less (`006f2290` with 1);
/// walking the chain with `i` from the last index down, `nodes[i]` gets the node's `+8`
/// and, while `i > 0`, `links[i - 1]` its `+0xc` (each element through `00877a30`). False
/// for a null `node`, true otherwise.
pub fn fn_00996ae0(e: &mut Engine, _this: Ptr, node: u32, nodes: Ptr, links: Ptr) -> bool {
    if node == 0 {
        return false;
    }
    let mut count = 0u32;
    let mut walk = node;
    while walk != 0 {
        count = count.wrapping_add(1);
        walk = e.mem.u32(walk + 0x10);
    }
    e.call(ARRAY_SET_SIZE, &args![nodes, count, 1u32]);
    e.call(ARRAY_SET_SIZE, &args![links, count.wrapping_sub(1), 1u32]);
    let mut walk = node;
    let mut index = count.wrapping_sub(1) as i32;
    while walk != 0 {
        let slot = e.call(ARRAY_ELEMENT, &args![nodes, index as u32]).u32();
        let value = e.mem.u32(walk + 8);
        e.mem.set_u32(slot, value);
        if index > 0 {
            let slot = e
                .call(ARRAY_ELEMENT, &args![links, index.wrapping_sub(1) as u32])
                .u32();
            let value = e.mem.u32(walk + 0xc);
            e.mem.set_u32(slot, value);
        }
        walk = e.mem.u32(walk + 0x10);
        index = index.wrapping_sub(1);
    }
    true
}

// Translated from 00996ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): takes the first entry of the first non-empty of
/// the 20 buckets at `this + 4`: the bucket gets the entry's next (`+0x18`; the next's back
/// link `+0x14` set to the bucket's address) or 0, the entry's links are cleared, and the
/// entry answered; 0 when every bucket is empty.
pub fn fn_00996ce0(e: &mut Engine, this: Ptr) -> u32 {
    for index in 0..0x14u32 {
        let bucket = this.addr() + 4 + 4 * index;
        let entry = e.mem.u32(bucket);
        if entry == 0 {
            continue;
        }
        let next = e.mem.u32(entry + 0x18);
        if next == 0 {
            e.mem.set_u32(bucket, 0);
        } else {
            e.mem.set_u32(next + 0x14, bucket);
            let next = e.mem.u32(e.mem.u32(bucket) + 0x18);
            e.mem.set_u32(bucket, next);
        }
        e.mem.set_u32(entry + 0x14, 0);
        e.mem.set_u32(entry + 0x18, 0);
        return entry;
    }
    0
}

// ---- Batch n023 ----------------------------------------------------------------------------

/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;
/// `0044edb0` (`thiscall` on a plan node; engine map `BaseProcess::GetCurrentProcedureIndex`,
/// a folded body that answers the word at `+0x10`): the node the search reached this one from
/// (`00996860` sets it).
const NODE_PARENT: u32 = 0x0044_edb0;
/// `006815c0` (`thiscall` on a list node): the address of the node's item; `00726070`
/// (`thiscall`): a node's next, also an item's value; `00559450` (`thiscall`): an item's kind;
/// `0062d990` (`thiscall` on a node): the next node.
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT_OR_VALUE: u32 = 0x0072_6070;
const ITEM_KIND: u32 = 0x0055_9450;
const NODE_NEXT: u32 = 0x0062_d990;
/// `006e6da0` (`cdecl`: size, block): the placement step after a scrap-heap allocation.
const PLACE_BLOCK: u32 = 0x006e_6da0;
/// `CombatWorldStateArray::IsTrue` (engine-map name of `009aadc0`; `thiscall`, `RET 8`).
const WORLD_STATE_IS_TRUE: u32 = 0x009a_adc0;
/// `006b3eb0` (`thiscall`, `RET 8`): sets up a `BSSimpleArray` with two words.
const SIMPLE_ARRAY_INIT: u32 = 0x006b_3eb0;
/// `006a7ad0` (`thiscall`, `RET 4`): the address of an array element.
const ARRAY_AT: u32 = 0x006a_7ad0;
/// `007cb2e0` (`thiscall`, `RET 4`; engine map `BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized`):
/// appends the word at the address.
const ARRAY_APPEND: u32 = 0x007c_b2e0;
/// The vtables of `BSScrapArray<CombatPlanNode*,1024>`, `BSScrapArray<CombatPlanLink*,1024>`
/// and `BSScrapArray<AStarNode<CombatPlanNode*,CombatPlanLink*>*,1024>` (RTTI names).
const SCRAP_ARRAY_VTABLE_0108DC6C: u32 = 0x0108_dc6c;
const SCRAP_ARRAY_VTABLE_0108DC94: u32 = 0x0108_dc94;
const SCRAP_ARRAY_VTABLE_0108DCD0: u32 = 0x0108_dcd0;
/// The table of step lists by goal kind (`009957f0`).
const STEP_LISTS: u32 = 0x011f_13d0;
/// The global `00995830` reads (the object `009611e0` is asked for).
const OBJECT_POINTER_011F1958: u32 = 0x011f_1958;
/// The largest step cost that still connects (`float` at `0108dbf0`).
const COST_LIMIT: u32 = 0x0108_dbf0;
/// `SendMessageA`, `ShowWindow` and `UpdateWindow` (import slots).
const SEND_MESSAGE: u32 = 0x00fd_f28c;
const SHOW_WINDOW: u32 = 0x00fd_f288;
const UPDATE_WINDOW: u32 = 0x00fd_f284;
/// `TVM_INSERTITEMA`, `TVI_ROOT`, `TVI_LAST` and the item mask (`TVIF_TEXT | TVIF_IMAGE |
/// TVIF_PARAM | TVIF_SELECTEDIMAGE`) of the debug window's tree view.
const TREE_INSERT_ITEM: u32 = 0x1100;
const TREE_ROOT: u32 = 0xffff_0000;
const TREE_LAST: u32 = 0xffff_0002;
const TREE_ITEM_MASK: u32 = 0x27;
/// `00464f30` (`cdecl`: text, 0): the text the tree item points at.
const TREE_TEXT: u32 = 0x0046_4f30;
/// `"Initial World State"`, `"Combat Plan"`, `"Planner"`.
const INITIAL_WORLD_STATE: u32 = 0x0108_dc14;
const COMBAT_PLAN_TITLE: u32 = 0x0108_dc00;
const PLANNER_TITLE: u32 = 0x0108_dc0c;
/// `_snprintf`-like `00406d00` (`cdecl`: buffer, size, format, ...) and its formats
/// `"%s [ %i ] (%.2f) %s"`, `"Start [ %i ]"`, `"Node Cost: %.02f Key: %08X"`; the marks
/// `"***"` and `""`.
const PRINT_TEXT: u32 = 0x0040_6d00;
const NODE_FORMAT: u32 = 0x0108_dc54;
const START_FORMAT: u32 = 0x0108_dc44;
const COST_FORMAT: u32 = 0x0108_dc28;
const MARK_KNOWN: u32 = 0x0108_019c;
const MARK_NONE: u32 = 0x0101_1584;

/// A block of `size` bytes from the planner's scrap heap (`+0x20b8`, `ScrapHeap::Allocate`
/// with the alignment word), then `006e6da0(size, block)`.
fn scrap_new(e: &mut Engine, planner: u32, size: u32) -> u32 {
    let alignment = e.mem.u32(SCRAP_ALIGNMENT);
    let block = e
        .call(
            SCRAP_ALLOCATE,
            &args![planner.wrapping_add(0x20b8), size, alignment],
        )
        .u32();
    e.call(PLACE_BLOCK, &args![size, block]).u32()
}

/// The item of a list node, or `None` at a node without one (`006815c0` twice, as the exe
/// reads it).
fn node_item(e: &mut Engine, node: u32) -> Option<u32> {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    if e.mem.u32(slot) == 0 {
        return None;
    }
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    Some(e.mem.u32(slot))
}

/// `SendMessageA(window, TVM_INSERTITEMA, 0, insert)`: the new tree item.
fn insert_tree_item(e: &mut Engine, window: u32, insert: u32) -> u32 {
    e.call(SEND_MESSAGE, &args![window, TREE_INSERT_ITEM, 0u32, insert])
        .u32()
}

// Translated from 009948b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the first word of the element `index` of
/// the array at `*this`.
pub fn fn_009948b0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = e.mem.u32(this.addr());
    let element = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
    e.mem.u32(element)
}

// Translated from 009948d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the word at `+0xc` of the element `index`
/// of the array at `*this`.
pub fn fn_009948d0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = e.mem.u32(this.addr());
    let element = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
    e.mem.u32(element.wrapping_add(0xc))
}

// Translated from 00995760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): stores -1 at `this`; returns `this`.
pub fn fn_00995760(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0xffff_ffff);
    this
}

// Translated from 00995780 (decompiled, FalloutNV.exe 1.4.0.525)
/// A plan link constructor (unnamed in the engine map; `thiscall`, `RET 0xc`): `+0` = `step`,
/// `+4` = `third`, `+8` = `value`, then `00995760` on `+0xc`. Returns `this`.
pub fn fn_00995780(e: &mut Engine, this: Ptr, step: u32, value: u32, third: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, step);
    e.mem.set_u32(me.wrapping_add(4), third);
    e.mem.set_u32(me.wrapping_add(8), value);
    fn_00995760(e, this.byte_add(0xc));
    this
}

// Translated from 009957c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `009aa3b0` on `this + 0x14` with the item's
/// kind (`00559450`) and value (`00726070`, read first); passes on its `EAX`.
pub fn fn_009957c0(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let value = e.call(NODE_NEXT_OR_VALUE, &args![item]).u32();
    let kind = e.call(ITEM_KIND, &args![item]).u32();
    e.call(0x009a_a3b0, &args![this.byte_add(0x14), kind, value])
        .u32()
}

// Translated from 009957f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the step list of goal kind `kind` (the table
/// at `011f13d0`).
pub fn fn_009957f0(e: &mut Engine, kind: u32) -> u32 {
    e.mem.u32(STEP_LISTS.wrapping_add(kind.wrapping_mul(4)))
}

// Translated from 00995800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatPlanner::IsGoalNode` (Xbox PDB; `thiscall`, `RET 4`, `this` not read): whether the
/// count at `+0xdc` of the node's world state (`+8`) is 0.
pub fn combat_planner_is_goal_node(e: &mut Engine, _this: Ptr, node: Ptr) -> bool {
    let state = e.mem.u32(node.addr().wrapping_add(8));
    e.mem.u32(state.wrapping_add(0xdc)) == 0
}

// Translated from 00995830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` not read): while `00995870` answers true for the object
/// `009611e0` gives for the global at `011f1958`, runs `00995890` on it.
pub fn fn_00995830(e: &mut Engine, _this: Ptr) {
    loop {
        let owner = e.mem.u32(OBJECT_POINTER_011F1958);
        let object = e.call(0x0096_11e0, &args![owner]).u32();
        if !fn_00995870(e, Ptr::new(object)) {
            break;
        }
        let owner = e.mem.u32(OBJECT_POINTER_011F1958);
        let object = e.call(0x0096_11e0, &args![owner]).u32();
        fn_00995890(e, Ptr::new(object));
    }
}

// Translated from 009958b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): clears the word at `+0` and the three 0x58-byte
/// tables at `+8`, `+0x60` and `+0xb8` (`00403d30`). Returns `this`.
pub fn fn_009958b0(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0);
    for offset in [8u32, 0x60, 0xb8] {
        e.call(FILL_BYTES, &args![me.wrapping_add(offset), 0u32, 0x58u32]);
    }
    this
}

// Translated from 00995910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): for the 0x16 slots, frees the block at `+8 + 4i`
/// (`00401030`) and runs `006d3370(1)` on a non-null object at `+0x60 + 4i`.
pub fn fn_00995910(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    for i in 0..0x16u32 {
        let block = e.mem.u32(me.wrapping_add(8 + 4 * i));
        e.call(FREE, &args![block]);
        let object = e.mem.u32(me.wrapping_add(0x60 + 4 * i));
        if object != 0 {
            e.call(0x006d_3370, &args![object, 1u32]);
        }
    }
}

// Translated from 00995980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): slot `index` gets `word` at `+8 + 4i` and
/// `value` (a `float`, `FLD`/`FSTP`) at `+0xb8 + 4i`, then `00537e90(index)` on the object at
/// `+4`.
pub fn fn_00995980(e: &mut Engine, this: Ptr, index: u32, word: u32, value: f32) {
    let me = this.addr();
    let offset = index.wrapping_mul(4);
    e.mem.set_u32(me.wrapping_add(offset).wrapping_add(8), word);
    e.mem.set_u32(
        me.wrapping_add(offset).wrapping_add(0xb8),
        x87_quiet_bits(value.to_bits()),
    );
    let object = e.mem.u32(me.wrapping_add(4));
    e.call(0x0053_7e90, &args![object, index]);
}

// Translated from 009959c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): slot `index` gets `word` at `+0x60 + 4i`,
/// then `00537e90(index)` on the object at `+4`.
pub fn fn_009959c0(e: &mut Engine, this: Ptr, index: u32, word: u32) {
    let me = this.addr();
    e.mem.set_u32(
        me.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x60),
        word,
    );
    let object = e.mem.u32(me.wrapping_add(4));
    e.call(0x0053_7e90, &args![object, index]);
}

// Translated from 00995db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): `this + 0x205c` (the planner's world state).
pub fn fn_00995db0(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x205c)
}

// Translated from 009962d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the word at `+0x2050`.
pub fn fn_009962d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x2050))
}

// Translated from 00996320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the first non-null of the 0x7f7 bucket words at
/// `this`, or 0.
pub fn fn_00996320(e: &mut Engine, this: Ptr) -> u32 {
    for i in 0..0x7f7u32 {
        let entry = e.mem.u32(this.addr().wrapping_add(4 * i));
        if entry != 0 {
            return entry;
        }
    }
    0
}

// Translated from 00996370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node map's step (unnamed in the engine map; `thiscall`, `RET 0xc`): false at a null
/// `*position`. Otherwise `*key` gets the entry's key (`+0x1c`) and `*entry` the entry; the
/// position moves to the entry's next (`+0x20`, answering true) or to the first non-null bucket
/// after the key's own (`00996840`), answering true, or to null, answering false.
pub fn fn_00996370(e: &mut Engine, this: Ptr, position: Ptr, key: Ptr, entry: Ptr) -> bool {
    let current = e.mem.u32(position.addr());
    if current == 0 {
        return false;
    }
    let entry_key = e.mem.u32(current.wrapping_add(0x1c));
    e.mem.set_u32(key.addr(), entry_key);
    e.mem.set_u32(entry.addr(), current);
    let next = e.mem.u32(current.wrapping_add(0x20));
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return true;
    }
    let entry_key = e.mem.u32(current.wrapping_add(0x1c));
    let bucket = fn_00996840(e, this, entry_key);
    let mut i = bucket.wrapping_add(1);
    while i < 0x7f7 {
        let head = e.mem.u32(this.addr().wrapping_add(i.wrapping_mul(4)));
        if head != 0 {
            e.mem.set_u32(position.addr(), head);
            return true;
        }
        i = i.wrapping_add(1);
    }
    e.mem.set_u32(position.addr(), 0);
    false
}

// Translated from 00995dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): walks the node map of the planner at
/// `+0x30` (`007d6bb0`, `00996320`, `00996370`) and appends to `out` (`007cb2e0`) every node
/// whose parent (`0044edb0`) is `node`.
pub fn fn_00995dd0(e: &mut Engine, this: Ptr, node: u32, out: Ptr) {
    let planner = e.mem.u32(this.addr().wrapping_add(0x30));
    let map = e.call(0x007d_6bb0, &args![planner]).u32();
    // The frame: position +0, key +4, entry +8.
    e.with_stack(0xc, |e, frame| {
        let f = frame.addr();
        let first = fn_00996320(e, Ptr::new(map));
        e.mem.set_u32(f, first);
        while e.mem.u32(f) != 0 {
            fn_00996370(
                e,
                Ptr::new(map),
                Ptr::new(f),
                Ptr::new(f + 4),
                Ptr::new(f + 8),
            );
            let entry = e.mem.u32(f + 8);
            if e.call(NODE_PARENT, &args![entry]).u32() == node {
                e.call(ARRAY_APPEND, &args![out, f + 8]);
            }
        }
    });
}

// Translated from 009966c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray<AStarNode<CombatPlanNode*,CombatPlanLink*>*,1024>` constructor (by its
/// vtable `0108dcbc`; `thiscall`, `RET 8`): the vtable, then `006b3eb0(first, second)`.
/// Returns `this`.
pub fn fn_009966c0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCBC);
    e.call(SIMPLE_ARRAY_INIT, &args![this, first, second]);
    this
}

// Translated from 00995e30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00995e30 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`): the number of nodes below `node` in the
/// search tree: its children (`00995dd0` into a stack `BSSimpleArray`, `009966c0(0x10, 0)`),
/// plus, recursively, theirs; the array is destroyed (`009966f0`) after.
pub fn fn_00995e30(e: &mut Engine, this: Ptr, node: u32) -> u32 {
    e.with_stack(0x10, |e, children| {
        fn_009966c0(e, children, 0x10, 0);
        fn_00995dd0(e, this, node, children);
        let mut count = e.call(ARRAY_COUNT, &args![children]).u32();
        let mut i = 0u32;
        while i < e.call(ARRAY_COUNT, &args![children]).u32() {
            let slot = e.call(ARRAY_AT, &args![children, i]).u32();
            let child = e.mem.u32(slot);
            count = fn_00995e30(e, this, child).wrapping_add(count);
            i = i.wrapping_add(1);
        }
        fn_009966f0(e, children);
        count
    })
}

// Translated from 00995ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00995ef0 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// Unnamed in the engine map (`thiscall`, `RET 8`): adds `node` and what is below it to the
/// debug window's tree (`SendMessageA(+0xc, TVM_INSERTITEMA, ..)`) under `parent`: a node item
/// (`lParam` the node, images 3) reading `"Start [ %i ]"` with the count below it
/// (`00995e30`) for a node without a parent, otherwise `"%s [ %i ] (%.2f) %s"` with the
/// step's name (slot `+0x10` of the step object `0048d150` points at), the count, the
/// difference of the node's and its parent's costs (`006a7f50`) and `"***"` when the planner
/// at `+0x38` knows the node's key (`00413f40`, `009962f0`) else `""`; under it a
/// `"Node Cost: %.02f Key: %08X"` item (images 5) and one item per non-null goal of the
/// node's world state (`009aa150`, images 6); then each child (`00995dd0`) the same way
/// under the node item.
pub fn fn_00995ef0(e: &mut Engine, this: Ptr, parent: u32, node: Ptr) {
    let me = this.addr();
    // The frame: the tree view insert structure +0 (0x30 bytes), the text +0x30 and the
    // step name +0x134 (0x104 bytes each), the children array +0x238.
    e.with_stack(0x248, |e, frame| {
        let tv = frame.addr();
        let text = tv + 0x30;
        let name = tv + 0x134;
        let children = Ptr::new(tv + 0x238);
        e.mem.set_u32(tv + 4, TREE_LAST);
        e.mem.set_u32(tv + 8, TREE_ITEM_MASK);
        e.mem.set_u32(tv, parent);
        e.mem.set_u32(tv + 0x2c, node.addr());
        let count = fn_00995e30(e, this, node.addr());
        if e.call(NODE_PARENT, &args![node]).u32() != 0 {
            let cost = e.call(0x006a_7f50, &args![node]).f64();
            let up = e.call(NODE_PARENT, &args![node]).u32();
            let up_cost = e.call(0x006a_7f50, &args![up]).f64();
            let step_cost = x87_store32(x87_sub(cost, up_cost));
            let holder = e.call(0x0048_d150, &args![node]).u32();
            let holder = e.mem.u32(holder);
            let object = e.mem.u32(holder);
            let data = e.mem.u32(holder.wrapping_add(8));
            e.vcall(object, 0x10, &args![name, 0x104u32, data]);
            let key = e.call(0x0041_3f40, &args![node]).u32();
            let known_nodes = Ptr::new(e.mem.u32(me.wrapping_add(0x38)));
            let mark = if fn_009962f0(e, known_nodes, key) {
                MARK_KNOWN
            } else {
                MARK_NONE
            };
            e.call(
                PRINT_TEXT,
                &args![
                    text,
                    0x104u32,
                    NODE_FORMAT,
                    name,
                    count,
                    x87_load32(step_cost),
                    mark
                ],
            );
        } else {
            e.call(PRINT_TEXT, &args![text, 0x104u32, START_FORMAT, count]);
        }
        let window = me.wrapping_add(0xc);
        let label = e.call(TREE_TEXT, &args![text, 0u32]).u32();
        e.mem.set_u32(tv + 0x18, label);
        e.mem.set_u32(tv + 0x20, 3);
        e.mem.set_u32(tv + 0x24, 3);
        let hwnd = e.mem.u32(window);
        let item = insert_tree_item(e, hwnd, tv);
        e.mem.set_u32(tv, item);
        e.mem.set_u32(tv + 0x2c, 0);
        let state = e.call(0x0041_3f40, &args![node]).u32();
        let state = e.mem.u32(state);
        let state_key = e.call(0x0099_4a90, &args![state]).u32();
        let cost = e.call(0x006a_7f50, &args![node]).f64();
        e.call(
            PRINT_TEXT,
            &args![text, 0x104u32, COST_FORMAT, cost, state_key],
        );
        let label = e.call(TREE_TEXT, &args![text, 0u32]).u32();
        e.mem.set_u32(tv + 0x18, label);
        e.mem.set_u32(tv + 0x20, 5);
        e.mem.set_u32(tv + 0x24, 5);
        let node_item = e.mem.u32(tv);
        let hwnd = e.mem.u32(window);
        let item = insert_tree_item(e, hwnd, tv);
        e.mem.set_u32(tv, item);
        let state = e.call(0x0041_3f40, &args![node]).u32();
        let state = e.mem.u32(state);
        let goals = e.mem.u32(state.wrapping_add(0xdc));
        let mut i = 0u32;
        while i < goals {
            let goal = e
                .mem
                .u32(state.wrapping_add(i.wrapping_mul(8)).wrapping_add(0x5c));
            if goal != 0 {
                e.call(0x009a_a150, &args![goal, text, 0x104u32, 0xffff_ffffu32]);
                let label = e.call(TREE_TEXT, &args![text, 0u32]).u32();
                e.mem.set_u32(tv + 0x18, label);
                e.mem.set_u32(tv + 0x20, 6);
                e.mem.set_u32(tv + 0x24, 6);
                let hwnd = e.mem.u32(window);
                insert_tree_item(e, hwnd, tv);
            }
            i = i.wrapping_add(1);
        }
        fn_009966c0(e, children, 0x10, 0);
        fn_00995dd0(e, this, node.addr(), children);
        let mut j = 0u32;
        while j < e.call(ARRAY_COUNT, &args![children]).u32() {
            let slot = e.call(ARRAY_AT, &args![children, j]).u32();
            let child = e.mem.u32(slot);
            fn_00995ef0(e, this, node_item, Ptr::new(child));
            j = j.wrapping_add(1);
        }
        fn_009966f0(e, children);
    });
}

// Translated from 00995aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00995aa0 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`ECX` only): fills the planner's debug window (tree view at
/// `+0xc`, window at `+8`): an `"Initial World State"` root with the 0x14 lines of the
/// planner's world state (`009aae70` into a 0xa28-byte buffer); with a plan at `+0x34`, a
/// `"Combat Plan"` root with a line per step (`009948b0`, the step's slot `+0x10` with
/// `009948d0`); and with a search root (the first node, `009962d0` on the planner at `+0x30`,
/// followed up through `0044edb0` to one without a parent), a `"Planner"` root (its `lParam`
/// the root, the item kept at `+0x10`) filled by `00995ef0`, after which the window is shown
/// (`ShowWindow(.., 10)`) and updated. The insert structure's other words are the exe's
/// uninitialised stack; the item mask (0x27) leaves them unread.
pub fn fn_00995aa0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    // The frame: the tree view insert structure +0 (0x30 bytes), the world-state text +0x30
    // (0xa28 bytes), the step text +0xa58 (0x104 bytes).
    e.with_stack(0xb5c, |e, frame| {
        let tv = frame.addr();
        let text = tv + 0x30;
        let step_text = tv + 0xa58;
        e.mem.set_u32(tv + 4, TREE_LAST);
        e.mem.set_u32(tv + 8, TREE_ITEM_MASK);
        let mut root = 0u32;
        let planner = Ptr::new(e.mem.u32(me.wrapping_add(0x30)));
        let mut node = fn_009962d0(e, planner);
        while node != 0 {
            if e.call(NODE_PARENT, &args![node]).u32() == 0 {
                root = node;
                break;
            }
            node = e.call(NODE_PARENT, &args![node]).u32();
        }
        let planner = Ptr::new(e.mem.u32(me.wrapping_add(0x30)));
        let world = fn_00995db0(e, planner);
        let window = me.wrapping_add(0xc);
        let top = |e: &mut Engine, title: u32, param: u32| {
            e.mem.set_u32(tv, TREE_ROOT);
            let label = e.call(TREE_TEXT, &args![title, 0u32]).u32();
            e.mem.set_u32(tv + 0x18, label);
            e.mem.set_u32(tv + 0x20, 0);
            e.mem.set_u32(tv + 0x24, 0);
            e.mem.set_u32(tv + 0x2c, param);
            let hwnd = e.mem.u32(window);
            insert_tree_item(e, hwnd, tv)
        };
        let states = top(e, INITIAL_WORLD_STATE, 0);
        for i in 0..0x14u32 {
            e.mem.set_u32(tv, states);
            e.call(0x009a_ae70, &args![world, text, 0xa28u32, i]);
            let label = e.call(TREE_TEXT, &args![text, 0u32]).u32();
            e.mem.set_u32(tv + 0x18, label);
            let hwnd = e.mem.u32(window);
            insert_tree_item(e, hwnd, tv);
        }
        if e.mem.u32(me.wrapping_add(0x34)) != 0 {
            let plan_item = top(e, COMBAT_PLAN_TITLE, 0);
            let mut i = 0u32;
            loop {
                let plan = e.mem.u32(me.wrapping_add(0x34));
                if i >= e.call(NODE_NEXT_OR_VALUE, &args![plan]).u32() {
                    break;
                }
                let plan = Ptr::new(e.mem.u32(me.wrapping_add(0x34)));
                let step = fn_009948b0(e, plan, i);
                if step != 0 {
                    let plan = Ptr::new(e.mem.u32(me.wrapping_add(0x34)));
                    let data = fn_009948d0(e, plan, i);
                    e.mem.set_u32(tv, plan_item);
                    e.vcall(step, 0x10, &args![step_text, 0x104u32, data]);
                    let label = e.call(TREE_TEXT, &args![step_text, 0u32]).u32();
                    e.mem.set_u32(tv + 0x18, label);
                    let hwnd = e.mem.u32(window);
                    insert_tree_item(e, hwnd, tv);
                }
                i = i.wrapping_add(1);
            }
        }
        if root != 0 {
            let item = top(e, PLANNER_TITLE, root);
            e.mem.set_u32(me.wrapping_add(0x10), item);
            let item = e.mem.u32(me.wrapping_add(0x10));
            fn_00995ef0(e, this, item, Ptr::new(root));
            let hwnd = e.mem.u32(me.wrapping_add(8));
            e.call(SHOW_WINDOW, &args![hwnd, 10u32]);
            let hwnd = e.mem.u32(me.wrapping_add(8));
            e.call(UPDATE_WINDOW, &args![hwnd]);
        }
    });
}

// Translated from 009959f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009959f0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The planner debug window's constructor (unnamed in the engine map; `thiscall`, `RET 0xc`):
/// `004d61b0` with `0084e3a0` and `0044ddc0` of the object at `011dea0c` (that order of calls),
/// 0, `"Combat Plan"`, `0x80000000` twice, 0x320 and 0x258; then `+0x30` = `planner`, `+0x34` =
/// `plan`, `+0x38` = `known`, and `00995aa0` fills it. Returns `this`.
pub fn fn_009959f0(e: &mut Engine, this: Ptr, planner: u32, plan: u32, known: u32) -> Ptr {
    let object = e.mem.u32(0x011d_ea0c);
    let second = e.call(ARRAY_COUNT, &args![object]).u32();
    let object = e.mem.u32(0x011d_ea0c);
    let first = e.call(0x0084_e3a0, &args![object]).u32();
    e.call(
        0x004d_61b0,
        &args![
            this,
            first,
            second,
            0u32,
            COMBAT_PLAN_TITLE,
            0x8000_0000u32,
            0x8000_0000u32,
            0x320u32,
            0x258u32
        ],
    );
    let me = this.addr();
    e.mem.set_u32(me.wrapping_add(0x30), planner);
    e.mem.set_u32(me.wrapping_add(0x34), plan);
    e.mem.set_u32(me.wrapping_add(0x38), known);
    fn_00995aa0(e, this);
    this
}

// Translated from 00996420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): stores at `+0x2054` the byte the tasklet
/// manager (`BSTaskletManager::QInstance`, `00b00a00`) answers through its slot `+0xc` for
/// `0040fc90`'s word, then searches (`00996860`). A found node answers true; otherwise false,
/// with the best node so far (`+0x2048`) used when `use_best` is set. A node in hand is turned
/// into the `nodes` and `links` arrays (`00996ae0`).
pub fn fn_00996420(e: &mut Engine, this: Ptr, nodes: Ptr, links: Ptr, use_best: bool) -> bool {
    let me = this.addr();
    let manager = e.call(0x00b0_0a00, &[]).u32();
    let word = e.call(0x0040_fc90, &[]).u32();
    let flag = e.vcall(manager, 0xc, &args![word]).u8();
    e.mem.set_u8(me.wrapping_add(0x2054), flag);
    let found = fn_00996860(e, this);
    let (answer, node) = if found != 0 {
        (true, found)
    } else if use_best {
        (false, e.mem.u32(me.wrapping_add(0x2048)))
    } else {
        (false, 0)
    };
    if node != 0 {
        fn_00996ae0(e, this, node, nodes, links);
    }
    answer
}

// Translated from 00996ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray<CombatPlanNode*,1024>` constructor (by its vtable `0108dc80`; `ECX` only):
/// the vtable, then `006b3eb0(0, 0)`. Returns `this`.
pub fn fn_00996ba0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DC80);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00996bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray<CombatPlanLink*,1024>` constructor (by its vtable `0108dca8`; `ECX` only):
/// the vtable, then `006b3eb0(0, 0)`. Returns `this`.
pub fn fn_00996bd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCA8);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00996db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray<AStarNode<..>*,1024>` constructor (by its vtable `0108dcbc`; `ECX` only):
/// the vtable, then `006b3eb0(0, 0)`. Returns `this`.
pub fn fn_00996db0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108DCBC);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

/// A `BSScrapArray` constructor body: `base` (the `BSSimpleArray` constructor), the scrap
/// array's vtable, the thread's scrap heap (`MemoryManager::GetThreadScrapHeap`, `00aa42e0`,
/// on `00401020()`) at `+0x10`, then `006b3eb0(0, 0)`.
fn scrap_array_construct(
    e: &mut Engine,
    this: Ptr,
    base: fn(&mut Engine, Ptr) -> Ptr,
    vtable: u32,
) -> Ptr {
    base(e, this);
    e.mem.set_u32(this.addr(), vtable);
    let manager = e.call(0x0040_1020, &[]).u32();
    let heap = e.call(0x00aa_42e0, &args![manager]).u32();
    e.mem.set_u32(this.addr().wrapping_add(0x10), heap);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

/// A `BSScrapArray` destructor body: the vtable, `008454f0(1)`, then `base` (the
/// `BSSimpleArray` destructor).
fn scrap_array_destroy(e: &mut Engine, this: Ptr, vtable: u32, base: fn(&mut Engine, Ptr)) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    base(e, this);
}

// Translated from 009964c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009964c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSScrapArray<CombatPlanNode*,1024>` constructor (by its vtable `0108dc6c`; `ECX`
/// only): `00996ba0`, the vtable, the thread's scrap heap at `+0x10`, `006b3eb0(0, 0)`.
/// Returns `this`.
pub fn fn_009964c0(e: &mut Engine, this: Ptr) -> Ptr {
    scrap_array_construct(e, this, fn_00996ba0, SCRAP_ARRAY_VTABLE_0108DC6C)
}

// Translated from 009965c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009965c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSScrapArray<CombatPlanLink*,1024>` constructor (by its vtable `0108dc94`; `ECX`
/// only): `00996bd0`, the vtable, the thread's scrap heap at `+0x10`, `006b3eb0(0, 0)`.
/// Returns `this`.
pub fn fn_009965c0(e: &mut Engine, this: Ptr) -> Ptr {
    scrap_array_construct(e, this, fn_00996bd0, SCRAP_ARRAY_VTABLE_0108DC94)
}

// Translated from 00996c00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00996c00 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSScrapArray<AStarNode<..>*,1024>` constructor (by its vtable `0108dcd0`; `ECX`
/// only): `00996db0`, the vtable, the thread's scrap heap at `+0x10`, `006b3eb0(0, 0)`.
/// Returns `this`.
pub fn fn_00996c00(e: &mut Engine, this: Ptr) -> Ptr {
    scrap_array_construct(e, this, fn_00996db0, SCRAP_ARRAY_VTABLE_0108DCD0)
}

// Translated from 00996560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00996560 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSScrapArray<CombatPlanNode*,1024>` destructor (by its vtable `0108dc6c`; `ECX`
/// only): the vtable, `008454f0(1)`, then the `BSSimpleArray` destructor `00996540`.
pub fn fn_00996560(e: &mut Engine, this: Ptr) {
    scrap_array_destroy(e, this, SCRAP_ARRAY_VTABLE_0108DC6C, fn_00996540);
}

// Translated from 00996660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00996660 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSScrapArray<CombatPlanLink*,1024>` destructor (by its vtable `0108dc94`; `ECX`
/// only): the vtable, `008454f0(1)`, then the `BSSimpleArray` destructor `00996640`.
pub fn fn_00996660(e: &mut Engine, this: Ptr) {
    scrap_array_destroy(e, this, SCRAP_ARRAY_VTABLE_0108DC94, fn_00996640);
}

// Translated from 00996c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00996c80 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSScrapArray<AStarNode<..>*,1024>` destructor (by its vtable `0108dcd0`; `ECX`
/// only): the vtable, `008454f0(1)`, then the `BSSimpleArray` destructor `009966f0`.
pub fn fn_00996c80(e: &mut Engine, this: Ptr) {
    scrap_array_destroy(e, this, SCRAP_ARRAY_VTABLE_0108DCD0, fn_009966f0);
}

/// A scalar deleting destructor body: `destroy`, then `00401030` frees `this` when bit 0 of
/// `flags` is set; answers `this`.
fn delete_with(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00996710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatPlanNode*,1024>::scalar deleting destructor` (Xbox PDB name in the
/// engine map; `thiscall`, `RET 4`): `00996540`, then frees `this` on bit 0. Returns `this`.
pub fn bs_simple_array_combat_plan_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    delete_with(e, this, flags, fn_00996540)
}

// Translated from 00996740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanNode*,1024>::scalar deleting destructor` (Xbox PDB name in the
/// engine map; `thiscall`, `RET 4`): `00996560`, then frees `this` on bit 0. Returns `this`.
pub fn bs_scrap_array_combat_plan_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    delete_with(e, this, flags, fn_00996560)
}

// Translated from 00996770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatPlanLink*,1024>::scalar deleting destructor` (Xbox PDB name in the
/// engine map; `thiscall`, `RET 4`): `00996640`, then frees `this` on bit 0. Returns `this`.
pub fn bs_simple_array_combat_plan_link_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    delete_with(e, this, flags, fn_00996640)
}

// Translated from 009967a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CombatPlanLink*,1024>::scalar deleting destructor` (Xbox PDB name in the
/// engine map; `thiscall`, `RET 4`): `00996660`, then frees `this` on bit 0. Returns `this`.
pub fn bs_scrap_array_combat_plan_link_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    delete_with(e, this, flags, fn_00996660)
}

// Translated from 009967d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<AStarNode<CombatPlanNode*,CombatPlanLink*>*,1024>::scalar deleting
/// destructor` (Xbox PDB name in the engine map; `thiscall`, `RET 4`): `009966f0`, then frees
/// `this` on bit 0. Returns `this`.
pub fn bs_simple_array_a_star_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    delete_with(e, this, flags, fn_009966f0)
}

// Translated from 00996d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<AStarNode<CombatPlanNode*,CombatPlanLink*>*,1024>::scalar deleting
/// destructor` (Xbox PDB name in the engine map; `thiscall`, `RET 4`): `00996c80`, then frees
/// `this` on bit 0. Returns `this`.
pub fn bs_scrap_array_a_star_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    delete_with(e, this, flags, fn_00996c80)
}

// Translated from 00996860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00996860 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The planner's A* search (unnamed in the engine map; `ECX` only): takes the open node
/// (`00996ce0` on the open list at `+0x1ff4`) into `+0x204c`, keeping the best node (lowest
/// `006b9130`) at `+0x2048`. A node the planner's slot `+8` accepts as a goal ends the search
/// and is answered. Otherwise its connections (slot `+4`, into a stack `BSScrapArray`
/// (`00996c00`, emptied with `008454f0(0)` first)) are taken off and put back on the open list
/// (`006b9540`, `006b94a0`) with the node as their parent (`+0x10`), a connection back to the
/// node's own parent ending the search with null; with the byte at `+0x2054` set, every
/// 0x32nd node runs `0040fca0(0)`. An empty open list answers null (the last node taken stays
/// at `+0x204c`).
pub fn fn_00996860(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    let current = me.wrapping_add(0x204c);
    let best = me.wrapping_add(0x2048);
    let open = me.wrapping_add(0x1ff4);
    e.with_stack(0x14, |e, connections| {
        fn_00996c00(e, connections);
        let mut previous = e.mem.u32(current);
        let taken = fn_00996ce0(e, Ptr::new(open));
        e.mem.set_u32(current, taken);
        if e.mem.u32(current) == 0 {
            fn_00996c80(e, connections);
            return 0;
        }
        let node = e.mem.u32(current);
        e.mem.set_u32(best, node);
        let mut steps = 0u32;
        while e.mem.u32(current) != 0 {
            let node = e.mem.u32(current);
            if e.vcall(me, 8, &args![node]).bool() {
                let node = e.mem.u32(current);
                e.mem.set_u32(best, node);
                let node = e.mem.u32(current);
                fn_00996c80(e, connections);
                return node;
            }
            e.call(ARRAY_RELEASE_008454F0, &args![connections, 0u32]);
            let node = e.mem.u32(current);
            e.vcall(me, 4, &args![node, connections]);
            let mut i = 0u32;
            while i < e.call(ARRAY_COUNT, &args![connections]).u32() {
                let slot = e.call(ARRAY_ELEMENT, &args![connections, i]).u32();
                let next = e.mem.u32(slot);
                e.call(0x006b_9540, &args![open, next]);
                let node = e.mem.u32(current);
                if e.mem.u32(node.wrapping_add(0x10)) == next {
                    fn_00996c80(e, connections);
                    return 0;
                }
                let node = e.mem.u32(current);
                e.mem.set_u32(next.wrapping_add(0x10), node);
                e.call(0x006b_94a0, &args![open, next]);
                let next_cost = e.call(0x006b_9130, &args![next]).f64();
                let leader = e.mem.u32(best);
                let best_cost = e.call(0x006b_9130, &args![leader]).f64();
                if best_cost > next_cost {
                    e.mem.set_u32(best, next);
                }
                i = i.wrapping_add(1);
            }
            if e.mem.u8(me.wrapping_add(0x2054)) != 0 {
                steps = steps.wrapping_add(1);
                if steps == 0x32 {
                    steps = 0;
                    e.call(0x0040_fca0, &args![0u32]);
                }
            }
            previous = e.mem.u32(current);
            let taken = fn_00996ce0(e, Ptr::new(open));
            e.mem.set_u32(current, taken);
        }
        e.mem.set_u32(current, previous);
        fn_00996c80(e, connections);
        0
    })
}

// Translated from 009950c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009950c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatPlanner::GetNodeConnections` (Xbox PDB; `thiscall`, `RET 8`): the nodes reachable
/// from `node`, appended to `out` (reserved 0x10 first when `node` has no parent). For each
/// goal of the node's world state (`+8`: the pairs at `+0x5c`, count `+0xdc`) the steps of
/// its kind (`009957f0`) are tried, a step that `00994250(1)` marks only at the root. A step
/// applies when `009957c0` passes, its slot `+0` accepts the planner's object (`+0x2058`) and
/// world state (`+0x205c`) with the goal's value (mapped through the node state's `00984470`
/// when `00994830` says so), and its cost (slot `+0xc`) is below the `float` at `0108dbf0`.
/// Then a new 0xE4-byte world state (`00994e60`, scrap heap `+0x20b8`) takes the step's
/// effects (the lists `007d6bb0` and `00409140`: `009aae10` with `009947f0`), every goal of
/// the node and every condition of the step (`0048d150`) not true in it is added as a new
/// 8-byte goal (`004b0680`, `00994ea0`), the cost plus the node's own (`+0`) goes with the
/// state's key (`009950a0`) to `006b8fe0`, and for its answers 0 (a new entry: the state at
/// `+8`, cost 0 at `+4`) and 1 a new 16-byte link (`00995780`; `+0xc` the word `006ecd40`
/// filled from the planner's object) goes to the entry's `+0xc` and the entry into `out`.
/// A goal no step applies to empties `out` (`008454f0(0)`) and answers false; true otherwise.
pub fn combat_planner_get_node_connections(e: &mut Engine, this: Ptr, node: Ptr, out: Ptr) -> bool {
    let me = this.addr();
    let n = node.addr();
    let base_cost = x87_quiet_bits(e.mem.u32(n));
    let at_root = e.call(NODE_PARENT, &args![node]).u32() == 0;
    if at_root {
        e.call(0x0084_b5c0, &args![out, 0x10u32]);
    }
    // The frame: the link word +0, the new world state +4, the entry `006b8fe0` answers +8.
    e.with_stack(0xc, |e, frame| {
        let f = frame.addr();
        let mut i = 0u32;
        loop {
            let state = e.mem.u32(n.wrapping_add(8));
            if i >= e.mem.u32(state.wrapping_add(0xdc)) {
                return true;
            }
            let goal = e
                .mem
                .u32(state.wrapping_add(i.wrapping_mul(8)).wrapping_add(0x5c));
            let kind = e.call(ITEM_KIND, &args![goal]).u32();
            let mut list = fn_009957f0(e, kind);
            let mut connected = false;
            while list != 0 {
                let Some(step) = node_item(e, list) else {
                    break;
                };
                if !e.call(0x0099_4250, &args![step, 1u32]).bool() || at_root {
                    'step: {
                        let applies = fn_009957c0(e, Ptr::new(step), Ptr::new(goal)) as u8 != 0;
                        let mut value = e.call(NODE_NEXT_OR_VALUE, &args![goal]).u32();
                        if e.call(0x0099_4830, &args![value]).bool() {
                            let state = e.mem.u32(n.wrapping_add(8));
                            value = e.call(0x0098_4470, &args![state, value]).u32();
                        }
                        fn_00995760(e, Ptr::new(f));
                        let holder = e.mem.u32(me.wrapping_add(0x2058));
                        let object = e.call(0x005e_3fc0, &args![holder]).u32();
                        e.call(0x006e_cd40, &args![object, f]);
                        if !applies {
                            break 'step;
                        }
                        let holder = e.mem.u32(me.wrapping_add(0x2058));
                        if !e
                            .vcall(step, 0, &args![holder, me.wrapping_add(0x205c), value])
                            .bool()
                        {
                            break 'step;
                        }
                        let holder = e.mem.u32(me.wrapping_add(0x2058));
                        let cost = e.vcall(step, 0xc, &args![holder, value]).f64();
                        let mut cost = x87_store32(cost);
                        let limit = x87_load32(e.mem.u32(COST_LIMIT));
                        if !(limit > x87_load32(cost)) {
                            break 'step;
                        }
                        let block = scrap_new(e, me, 0xe4);
                        let world = if block != 0 {
                            e.call(0x0099_4e60, &args![block, me.wrapping_add(0x205c)])
                                .u32()
                        } else {
                            0
                        };
                        e.mem.set_u32(f + 4, world);
                        for list_of in [0x007d_6bb0u32, 0x0040_9140] {
                            let mut effects = e.call(list_of, &args![step]).u32();
                            while effects != 0 {
                                let Some(effect) = node_item(e, effects) else {
                                    break;
                                };
                                let world = e.mem.u32(f + 4);
                                let raw = e.call(NODE_NEXT_OR_VALUE, &args![effect]).u32();
                                let mapped = e.call(0x0099_47f0, &args![world, raw, value]).u32();
                                let effect_kind = e.call(ITEM_KIND, &args![effect]).u32();
                                e.call(0x009a_ae10, &args![world, effect_kind, mapped]);
                                effects = e.call(NODE_NEXT, &args![effects]).u32();
                            }
                        }
                        let mut j = 0u32;
                        loop {
                            let state = e.mem.u32(n.wrapping_add(8));
                            if j >= e.mem.u32(state.wrapping_add(0xdc)) {
                                break;
                            }
                            let pair = state.wrapping_add(j.wrapping_mul(8));
                            let other = e.mem.u32(pair.wrapping_add(0x5c));
                            let world = e.mem.u32(f + 4);
                            let raw = e.call(NODE_NEXT_OR_VALUE, &args![other]).u32();
                            let mapped = e.call(0x0099_47f0, &args![world, raw, value]).u32();
                            let other_kind = e.call(ITEM_KIND, &args![other]).u32();
                            if !e
                                .call(WORLD_STATE_IS_TRUE, &args![world, other_kind, mapped])
                                .bool()
                            {
                                let block = scrap_new(e, me, 8);
                                let condition = if block != 0 {
                                    let other_kind = e.call(ITEM_KIND, &args![other]).u32();
                                    e.call(0x004b_0680, &args![block, other_kind, mapped]).u32()
                                } else {
                                    0
                                };
                                let state = e.mem.u32(n.wrapping_add(8));
                                let pair = state.wrapping_add(j.wrapping_mul(8));
                                let extra = e.mem.u32(pair.wrapping_add(0x60));
                                let world = e.mem.u32(f + 4);
                                e.call(0x0099_4ea0, &args![world, condition, extra]);
                            }
                            j = j.wrapping_add(1);
                        }
                        let mut needs = e.call(0x0048_d150, &args![step]).u32();
                        while needs != 0 {
                            let Some(need) = node_item(e, needs) else {
                                break;
                            };
                            let world = e.mem.u32(f + 4);
                            let raw = e.call(NODE_NEXT_OR_VALUE, &args![need]).u32();
                            let mapped = e.call(0x0099_47f0, &args![world, raw, value]).u32();
                            let need_kind = e.call(ITEM_KIND, &args![need]).u32();
                            if !e
                                .call(WORLD_STATE_IS_TRUE, &args![world, need_kind, mapped])
                                .bool()
                            {
                                let block = scrap_new(e, me, 8);
                                let condition = if block != 0 {
                                    let need_kind = e.call(ITEM_KIND, &args![need]).u32();
                                    e.call(0x004b_0680, &args![block, need_kind, mapped]).u32()
                                } else {
                                    0
                                };
                                let world = e.mem.u32(f + 4);
                                e.call(0x0099_4ea0, &args![world, condition, world]);
                            }
                            needs = e.call(NODE_NEXT, &args![needs]).u32();
                        }
                        cost = x87_store32(x87_add(x87_load32(cost), x87_m32(base_cost)));
                        connected = true;
                        let key = e.call(0x0099_50a0, &args![this, f + 4]).u32();
                        let status = e
                            .call(0x006b_8fe0, &args![this, key, x87_quiet_bits(cost), f + 8])
                            .u32();
                        match status {
                            0 => {
                                let entry = e.mem.u32(f + 8);
                                let world = e.mem.u32(f + 4);
                                e.mem.set_u32(entry.wrapping_add(8), world);
                                let entry = e.mem.u32(f + 8);
                                e.mem.set_u32(entry.wrapping_add(4), x87_store32(0.0));
                            }
                            1 => {}
                            _ => break 'step,
                        }
                        let block = scrap_new(e, me, 0x10);
                        let link = if block != 0 {
                            let state = e.mem.u32(n.wrapping_add(8));
                            let pair = state.wrapping_add(i.wrapping_mul(8));
                            let extra = e.mem.u32(pair.wrapping_add(0x60));
                            fn_00995780(e, Ptr::new(block), step, value, extra).addr()
                        } else {
                            0
                        };
                        let word = e.mem.u32(f);
                        e.mem.set_u32(link.wrapping_add(0xc), word);
                        let entry = e.mem.u32(f + 8);
                        e.mem.set_u32(entry.wrapping_add(0xc), link);
                        e.call(ARRAY_APPEND, &args![out, f + 8]);
                    }
                }
                list = e.call(NODE_NEXT_OR_VALUE, &args![list]).u32();
            }
            if !connected {
                e.call(ARRAY_RELEASE_008454F0, &args![out, 0u32]);
                return false;
            }
            i = i.wrapping_add(1);
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_48b0, fn_009948b0(Ptr, u32) -> u32),
        entry!(0x0099_48d0, fn_009948d0(Ptr, u32) -> u32),
        entry!(0x0099_5760, fn_00995760(Ptr) -> Ptr),
        entry!(0x0099_5780, fn_00995780(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0099_57c0, fn_009957c0(Ptr, Ptr) -> u32),
        entry!(0x0099_57f0, fn_009957f0(u32) -> u32),
        entry!(0x0099_5800, combat_planner_is_goal_node(Ptr, Ptr) -> bool),
        entry!(0x0099_5830, fn_00995830(Ptr)),
        entry!(0x0099_58b0, fn_009958b0(Ptr) -> Ptr),
        entry!(0x0099_5910, fn_00995910(Ptr)),
        entry!(0x0099_5980, fn_00995980(Ptr, u32, u32, f32)),
        entry!(0x0099_59c0, fn_009959c0(Ptr, u32, u32)),
        entry!(0x0099_5db0, fn_00995db0(Ptr) -> u32),
        entry!(0x0099_62d0, fn_009962d0(Ptr) -> u32),
        entry!(0x0099_6320, fn_00996320(Ptr) -> u32),
        entry!(0x0099_6370, fn_00996370(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x0099_5dd0, fn_00995dd0(Ptr, u32, Ptr)),
        entry!(0x0099_66c0, fn_009966c0(Ptr, u32, u32) -> Ptr),
        entry!(0x0099_5e30, fn_00995e30(Ptr, u32) -> u32),
        entry!(0x0099_5ef0, fn_00995ef0(Ptr, u32, Ptr)),
        entry!(0x0099_5aa0, fn_00995aa0(Ptr)),
        entry!(0x0099_59f0, fn_009959f0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0099_6420, fn_00996420(Ptr, Ptr, Ptr, bool) -> bool),
        entry!(0x0099_6ba0, fn_00996ba0(Ptr) -> Ptr),
        entry!(0x0099_6bd0, fn_00996bd0(Ptr) -> Ptr),
        entry!(0x0099_6db0, fn_00996db0(Ptr) -> Ptr),
        entry!(0x0099_64c0, fn_009964c0(Ptr) -> Ptr),
        entry!(0x0099_65c0, fn_009965c0(Ptr) -> Ptr),
        entry!(0x0099_6c00, fn_00996c00(Ptr) -> Ptr),
        entry!(0x0099_6560, fn_00996560(Ptr)),
        entry!(0x0099_6660, fn_00996660(Ptr)),
        entry!(0x0099_6c80, fn_00996c80(Ptr)),
        entry!(
            0x0099_6710,
            bs_simple_array_combat_plan_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0099_6740,
            bs_scrap_array_combat_plan_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0099_6770,
            bs_simple_array_combat_plan_link_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0099_67a0,
            bs_scrap_array_combat_plan_link_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0099_67d0,
            bs_simple_array_a_star_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0099_6d80,
            bs_scrap_array_a_star_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0099_6860, fn_00996860(Ptr) -> u32),
        entry!(
            0x0099_50c0,
            combat_planner_get_node_connections(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x0099_5870, fn_00995870(Ptr) -> bool),
        entry!(0x0099_5890, fn_00995890(Ptr)),
        entry!(0x0099_6540, fn_00996540(Ptr)),
        entry!(0x0099_6640, fn_00996640(Ptr)),
        entry!(0x0099_66f0, fn_009966f0(Ptr)),
        entry!(0x0099_62f0, fn_009962f0(Ptr, u32) -> bool),
        entry!(0x006f_2870, bs_scrap_array_combat_plan_node_allocate(Ptr, u32) -> u32),
        entry!(
            0x006c_f4a0,
            bs_scrap_array_combat_plan_node_deallocate(Ptr, u32)
        ),
        entry!(
            0x006c_fc20,
            bs_scrap_array_combat_plan_node_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(0x0099_6840, fn_00996840(Ptr, u32) -> u32),
        entry!(0x0099_6800, fn_00996800(Ptr)),
        entry!(0x0099_6ae0, fn_00996ae0(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x0099_6ce0, fn_00996ce0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00995870_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00AA6330, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_5870, &args![0x4100u32]).bool());
        assert!(e.call(0x0099_5870, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00AA6330).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0x14]);
    }

    #[test]
    fn l68d_00995890_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00AA62E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0099_5890, &args![0x4200u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00AA62E0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x14]);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_00996540_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_6540, ARRAY_VTABLE_0108DC80, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_00996640_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_6640, ARRAY_VTABLE_0108DCA8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009966f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0099_66f0, ARRAY_VTABLE_0108DCBC, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod b012_tests {
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

    #[test]
    fn finding_answers_whether_an_index_came_back() {
        let mut e = Engine::new();
        e.register(FIND_INDEX, |_, a| {
            if a[1] == 7 { 3u32 } else { 0xffff_ffff }.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_62f0, &args![0x5000u32, 7u32]).bool());
        assert!(!e.call(0x0099_62f0, &args![0x5000u32, 8u32]).bool());
        assert_eq!(
            calls(&e, FIND_INDEX)[0],
            vec![0x5000, 7, 0, COMPARE_009A3830]
        );
    }

    #[test]
    fn the_scrap_array_allocator() {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.mem.set_u32(SCRAP_ALIGNMENT, 4);
        e.register(SCRAP_ALLOCATE, |_, a| (0x7000 + a[1]).into_ret());
        e.register(SCRAP_DEALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_COUNT, |_, _| 3u32.into_ret());
        e.register(COPY_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x6600);
        assert_eq!(e.call(0x006f_2870, &args![this, 5u32]).u32(), 0x7014);
        e.call(0x006c_f4a0, &args![this, 0x9000u32]);
        assert_eq!(
            e.call(0x006c_fc20, &args![this, 0x9100u32, 8u32]).u32(),
            0x7020
        );
        assert_eq!(
            calls(&e, SCRAP_ALLOCATE),
            vec![vec![0x6600, 20, 4], vec![0x6600, 32, 4]]
        );
        assert_eq!(calls(&e, COPY_BYTES), vec![vec![0x7020, 0x9100, 12]]);
        assert_eq!(
            calls(&e, SCRAP_DEALLOCATE),
            vec![vec![0x6600, 0x9000], vec![0x6600, 0x9100]]
        );
    }

    #[test]
    fn the_hash_and_the_clear() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0099_6840, &args![0u32, 0x7f8u32]).u32(), 1);
        e.register(FILL_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x1fe0);
        e.call(0x0099_6800, &args![this]);
        assert!(calls(&e, FILL_BYTES).is_empty());
        e.mem.set_u32(this + 0x1fdc, 4);
        e.call(0x0099_6800, &args![this]);
        assert_eq!(calls(&e, FILL_BYTES), vec![vec![this, 0, 0x1fdc]]);
        assert_eq!(e.mem.u32(this + 0x1fdc), 0);
    }

    #[test]
    fn a_node_chain_becomes_two_arrays_back_to_front() {
        let mut e = Engine::new();
        let nodes_array = e.mem.alloc(0x40);
        let links_array = e.mem.alloc(0x40);
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_ELEMENT, |_, a| (a[0] + 4 * a[1]).into_ret());
        e.call_log = Some(vec![]);
        let chain: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x14)).collect();
        for (index, node) in chain.iter().enumerate() {
            e.mem.set_u32(node + 8, 0x10 + index as u32);
            e.mem.set_u32(node + 0xc, 0x20 + index as u32);
            let next = chain.get(index + 1).copied().unwrap_or(0);
            e.mem.set_u32(node + 0x10, next);
        }
        let answer = e.call(
            0x0099_6ae0,
            &args![0u32, chain[0], nodes_array, links_array],
        );
        assert!(answer.bool());
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE),
            vec![vec![nodes_array, 3, 1], vec![links_array, 2, 1]]
        );
        let nodes: Vec<u32> = (0..3).map(|i| e.mem.u32(nodes_array + 4 * i)).collect();
        let links: Vec<u32> = (0..2).map(|i| e.mem.u32(links_array + 4 * i)).collect();
        assert_eq!(nodes, vec![0x12, 0x11, 0x10]);
        assert_eq!(links, vec![0x21, 0x20]);
        assert!(!e
            .call(0x0099_6ae0, &args![0u32, 0u32, nodes_array, links_array])
            .bool());
    }

    #[test]
    fn the_first_bucket_entry_is_taken() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x54);
        assert_eq!(e.call(0x0099_6ce0, &args![this]).u32(), 0);
        let (first, second) = (e.mem.alloc(0x1c), e.mem.alloc(0x1c));
        e.mem.set_u32(first + 0x18, second);
        e.mem.set_u32(first + 0x14, 0x77);
        e.mem.set_u32(this + 4 + 4 * 3, first);
        assert_eq!(e.call(0x0099_6ce0, &args![this]).u32(), first);
        assert_eq!(e.mem.u32(this + 0x10), second);
        assert_eq!(e.mem.u32(second + 0x14), this + 0x10);
        assert_eq!((e.mem.u32(first + 0x14), e.mem.u32(first + 0x18)), (0, 0));
        assert_eq!(e.call(0x0099_6ce0, &args![this]).u32(), second);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}

#[cfg(test)]
mod n023_tests {
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

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    const VTABLE: u32 = 0x0e00_0000;

    const fn slot(offset: u32) -> u32 {
        0x0f00_0000 + offset
    }

    fn object(e: &mut Engine, words: &[u32]) -> u32 {
        if !e.mem.is_mapped(VTABLE) {
            let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
            e.put_vtable(VTABLE, &slots);
        }
        let o = e.mem.alloc(4 + 4 * words.len() as u32);
        e.mem.set_u32(o, VTABLE);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(o + 4 + 4 * i as u32, *w);
        }
        o
    }

    /// Lists are `{item, next}` nodes and items `{kind, value}` blocks, as the accessors read
    /// them.
    fn list_doubles(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT_OR_VALUE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ITEM_KIND, |e, a| e.mem.u32(a[0]).into_ret());
    }

    fn pair(e: &mut Engine, first: u32, second: u32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_u32(p, first);
        e.mem.set_u32(p + 4, second);
        p
    }

    /// A list of `items` (null-terminated by an empty node).
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut node = pair(e, 0, 0);
        for item in items.iter().rev() {
            node = pair(e, *item, node);
        }
        node
    }

    /// Arrays: `006b3eb0` gives a fresh 64-word buffer (`+4`) and size 0 (`+8`), `007cb2e0`
    /// appends the word at its address, `0044ddc0` answers the size, `006a7ad0` / `00877a30`
    /// the address of an element; releases do nothing.
    fn array_doubles(e: &mut Engine) {
        e.register(SIMPLE_ARRAY_INIT, |e, a| {
            let buffer = e.mem.alloc(0x100);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(ARRAY_APPEND, |e, a| {
            let n = e.mem.u32(a[0] + 8);
            let word = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * n, word);
            e.mem.set_u32(a[0] + 8, n + 1);
            Ret::default()
        });
        e.register(ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_AT, |e, a| (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret());
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_RELEASE_008454F0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
    }

    fn array_words(e: &Engine, array: u32) -> Vec<u32> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    #[test]
    fn element_words_through_the_array() {
        let mut e = Engine::new();
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, 0x77);
        e.mem.set_u32(second + 0xc, 0x88);
        let base = pair(&mut e, first, second);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, base);
        assert_eq!(e.call(0x0099_48b0, &args![array, 1u32]).u32(), 0x77);
        assert_eq!(e.call(0x0099_48d0, &args![array, 1u32]).u32(), 0x88);
    }

    #[test]
    fn the_link_word_starts_at_minus_one() {
        let mut e = Engine::new();
        let w = e.mem.alloc(4);
        assert_eq!(e.call(0x0099_5760, &args![w]).u32(), w);
        assert_eq!(e.mem.u32(w), 0xffff_ffff);
    }

    #[test]
    fn a_link_holds_step_third_and_value() {
        let mut e = Engine::new();
        let link = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x0099_5780, &args![link, 1u32, 2u32, 3u32]).u32(),
            link
        );
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(link + 4 * i)).collect();
        assert_eq!(words, vec![1, 3, 2, 0xffff_ffff]);
    }

    #[test]
    fn a_step_asks_its_check_with_the_goals_kind_and_value() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(0x009a_a3b0, |_, a| (a[1] + a[2]).into_ret());
        let goal = pair(&mut e, 5, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_57c0, &args![0x4000u32, goal]).u32(), 11);
        assert_eq!(calls(&e, 0x009a_a3b0), vec![vec![0x4014, 5, 6]]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![0x0099_57c0, NODE_NEXT_OR_VALUE, ITEM_KIND, 0x009a_a3b0]
        );
    }

    #[test]
    fn step_lists_by_kind() {
        let mut e = Engine::new();
        e.map(STEP_LISTS, 0x40);
        e.mem.set_u32(STEP_LISTS + 8, 0x1234);
        assert_eq!(e.call(0x0099_57f0, &args![2u32]).u32(), 0x1234);
    }

    #[test]
    fn a_goal_node_has_no_goals_left() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0xe0);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, state);
        assert!(e.call(0x0099_5800, &args![0u32, node]).bool());
        e.mem.set_u32(state + 0xdc, 1);
        assert!(!e.call(0x0099_5800, &args![0u32, node]).bool());
    }

    #[test]
    fn the_queue_runs_while_its_check_passes() {
        let mut e = Engine::new();
        e.map(OBJECT_POINTER_011F1958, 4);
        e.set_global(OBJECT_POINTER_011F1958, 0x5000u32);
        e.register(0x0096_11e0, |_, a| (a[0] + 0x100).into_ret());
        let mut left = 2;
        e.register_double(CALLEE_00AA6330, move |_, _| {
            left -= 1;
            u32::from(left >= 0).into_ret()
        });
        noop(&mut e, &[CALLEE_00AA62E0]);
        e.call_log = Some(vec![]);
        e.call(0x0099_5830, &args![0u32]);
        assert_eq!(calls(&e, CALLEE_00AA6330).len(), 3);
        assert_eq!(calls(&e, CALLEE_00AA62E0), vec![vec![0x5114]; 2]);
    }

    #[test]
    fn the_three_tables_are_cleared() {
        let mut e = Engine::new();
        noop(&mut e, &[FILL_BYTES]);
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_58b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(
            calls(&e, FILL_BYTES),
            vec![
                vec![this + 8, 0, 0x58],
                vec![this + 0x60, 0, 0x58],
                vec![this + 0xb8, 0, 0x58]
            ]
        );
    }

    #[test]
    fn every_slot_frees_its_block_and_deletes_its_object() {
        let mut e = Engine::new();
        noop(&mut e, &[FREE, 0x006d_3370]);
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 8, 0x7000);
        e.mem.set_u32(this + 0x60 + 4 * 3, 0x7100);
        e.call_log = Some(vec![]);
        e.call(0x0099_5910, &args![this]);
        let frees = calls(&e, FREE);
        assert_eq!(frees.len(), 0x16);
        assert_eq!(frees[0], vec![0x7000]);
        assert_eq!(calls(&e, 0x006d_3370), vec![vec![0x7100, 1]]);
    }

    #[test]
    fn a_slot_takes_a_word_and_a_float() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0053_7e90]);
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 4, 0x6600);
        e.call_log = Some(vec![]);
        e.call(0x0099_5980, &args![this, 2u32, 0x55u32, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 8 + 8), 0x55);
        assert_eq!(e.mem.u32(this + 0xb8 + 8), 0x7fc0_0001);
        assert_eq!(calls(&e, 0x0053_7e90), vec![vec![0x6600, 2]]);
    }

    #[test]
    fn a_slot_takes_an_object() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0053_7e90]);
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 4, 0x6600);
        e.call_log = Some(vec![]);
        e.call(0x0099_59c0, &args![this, 3u32, 0x66u32]);
        assert_eq!(e.mem.u32(this + 0x60 + 12), 0x66);
        assert_eq!(calls(&e, 0x0053_7e90), vec![vec![0x6600, 3]]);
    }

    #[test]
    fn the_world_state_address_and_the_first_node() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0099_5db0, &args![0x10_0000u32]).u32(), 0x10_205c);
        let planner = e.mem.alloc(0x2060);
        e.mem.set_u32(planner + 0x2050, 0x4242);
        assert_eq!(e.call(0x0099_62d0, &args![planner]).u32(), 0x4242);
    }

    #[test]
    fn the_first_bucket_entry_of_the_node_map() {
        let mut e = Engine::new();
        let map = e.mem.alloc(4 * 0x7f7);
        assert_eq!(e.call(0x0099_6320, &args![map]).u32(), 0);
        e.mem.set_u32(map + 4 * 0x7f6, 9);
        e.mem.set_u32(map + 4 * 0x400, 8);
        assert_eq!(e.call(0x0099_6320, &args![map]).u32(), 8);
    }

    /// A node map entry: key `+0x1c`, next `+0x20`, parent `+0x10`; put into its bucket
    /// (`key % 0x7f7`) at the head.
    fn entry(e: &mut Engine, map: u32, key: u32, parent: u32) -> u32 {
        let n = e.mem.alloc(0x28);
        e.mem.set_u32(n + 0x1c, key);
        e.mem.set_u32(n + 0x10, parent);
        let bucket = map + 4 * (key % 0x7f7);
        let head = e.mem.u32(bucket);
        e.mem.set_u32(n + 0x20, head);
        e.mem.set_u32(bucket, n);
        n
    }

    #[test]
    fn the_node_map_step_follows_chains_and_buckets() {
        let mut e = Engine::new();
        let map = e.mem.alloc(4 * 0x7f7);
        let a = entry(&mut e, map, 3, 0);
        let b = entry(&mut e, map, 3 + 0x7f7, 0);
        let c = entry(&mut e, map, 10, 0);
        let frame = e.mem.alloc(12);
        e.mem.set_u32(frame, b);
        let mut seen = vec![];
        let mut answers = vec![];
        while e.mem.u32(frame) != 0 {
            let r = e
                .call(0x0099_6370, &args![map, frame, frame + 4, frame + 8])
                .bool();
            answers.push(r);
            seen.push((e.mem.u32(frame + 4), e.mem.u32(frame + 8)));
        }
        assert_eq!(seen, vec![(3 + 0x7f7, b), (3, a), (10, c)]);
        assert_eq!(answers, vec![true, true, false]);
        assert!(!e
            .call(0x0099_6370, &args![map, frame, frame + 4, frame + 8])
            .bool());
    }

    /// A planner object (`+0x30` of a window) whose node map `007d6bb0` answers.
    fn window_with_map(e: &mut Engine) -> (u32, u32) {
        let map = e.mem.alloc(4 * 0x7f7);
        let planner = e.mem.alloc(4);
        e.mem.set_u32(planner, map);
        e.register(0x007d_6bb0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NODE_PARENT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let window = e.mem.alloc(0x40);
        e.mem.set_u32(window + 0x30, planner);
        (window, map)
    }

    #[test]
    fn the_children_of_a_node_are_collected() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let (window, map) = window_with_map(&mut e);
        let root = entry(&mut e, map, 1, 0);
        let a = entry(&mut e, map, 2, root);
        let b = entry(&mut e, map, 5, root);
        entry(&mut e, map, 7, a);
        let out = e.mem.alloc(0x10);
        e.call(SIMPLE_ARRAY_INIT, &args![out, 0u32, 0u32]);
        e.call(0x0099_5dd0, &args![window, root, out]);
        assert_eq!(array_words(&e, out), vec![a, b]);
    }

    #[test]
    fn simple_array_constructors_set_their_vtables() {
        let mut e = Engine::new();
        noop(&mut e, &[SIMPLE_ARRAY_INIT]);
        let a = e.mem.alloc(0x10);
        for (address, vtable, words) in [
            (0x0099_66c0u32, ARRAY_VTABLE_0108DCBC, vec![0x10u32, 2]),
            (0x0099_6ba0, ARRAY_VTABLE_0108DC80, vec![0, 0]),
            (0x0099_6bd0, ARRAY_VTABLE_0108DCA8, vec![0, 0]),
            (0x0099_6db0, ARRAY_VTABLE_0108DCBC, vec![0, 0]),
        ] {
            e.call_log = Some(vec![]);
            let mut call = vec![a];
            call.extend(
                words
                    .iter()
                    .take(if address == 0x0099_66c0 { 2 } else { 0 }),
            );
            assert_eq!(e.call(address, &call).u32(), a);
            assert_eq!(e.mem.u32(a), vtable);
            let mut expected = vec![a];
            expected.extend(words);
            assert_eq!(calls(&e, SIMPLE_ARRAY_INIT), vec![expected]);
        }
    }

    #[test]
    fn the_nodes_below_are_counted_recursively() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let (window, map) = window_with_map(&mut e);
        let root = entry(&mut e, map, 1, 0);
        let a = entry(&mut e, map, 2, root);
        entry(&mut e, map, 5, root);
        entry(&mut e, map, 7, a);
        assert_eq!(e.call(0x0099_5e30, &args![window, root]).u32(), 3);
        assert_eq!(e.call(0x0099_5e30, &args![window, a]).u32(), 1);
    }

    /// The window's tree: every insert is recorded as (parent, text, image, param) and
    /// answers the next item number from 0x100.
    fn tree_doubles(e: &mut Engine) -> std::rc::Rc<std::cell::RefCell<Vec<[u32; 4]>>> {
        let inserted = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = inserted.clone();
        e.register_double(SEND_MESSAGE, move |e, a| {
            assert_eq!((a[1], a[2]), (TREE_INSERT_ITEM, 0));
            let tv = a[3];
            assert_eq!(
                (e.mem.u32(tv + 4), e.mem.u32(tv + 8)),
                (TREE_LAST, TREE_ITEM_MASK)
            );
            let item = [
                e.mem.u32(tv),
                e.mem.u32(tv + 0x18),
                e.mem.u32(tv + 0x20),
                e.mem.u32(tv + 0x2c),
            ];
            assert_eq!(e.mem.u32(tv + 0x24), item[2]);
            seen.borrow_mut().push(item);
            (0x100 + seen.borrow().len() as u32).into_ret()
        });
        e.register(TREE_TEXT, |e, a| {
            if a[0] < 0x2000_0000 {
                a[0].into_ret()
            } else {
                e.mem.u32(a[0]).into_ret()
            }
        });
        inserted
    }

    #[test]
    fn the_search_tree_goes_into_the_window() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let inserted = tree_doubles(&mut e);
        let (window, map) = window_with_map(&mut e);
        e.mem.set_u32(window + 0xc, 0x3300);
        // `PRINT_TEXT` leaves its format's address as the text's first word.
        e.register(PRINT_TEXT, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            Ret::default()
        });
        // A node's cost is its float at +0, its state the word at +4 (`00413f40` answers
        // its address), its step holder the word at +8.
        e.register(0x006a_7f50, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(0x0041_3f40, |_, a| (a[0] + 4).into_ret());
        e.register(0x0048_d150, |_, a| (a[0] + 8).into_ret());
        e.register(0x0099_4a90, |_, _| 0xabcu32.into_ret());
        e.register(FIND_INDEX, |_, _| 0xffff_ffffu32.into_ret());
        e.register(slot(0x10), |e, a| {
            e.mem.set_u32(a[1], 0x6e61_6d65);
            Ret::default()
        });
        noop(&mut e, &[0x009a_a150]);
        let known = e.mem.alloc(0x10);
        e.mem.set_u32(window + 0x38, known);
        let root = entry(&mut e, map, 1, 0);
        let child = entry(&mut e, map, 2, root);
        let state = e.mem.alloc(0xe0);
        e.mem.set_u32(state + 0xdc, 2);
        e.mem.set_u32(state + 0x5c + 8, 0x7777);
        e.mem.set_f32(root, 2.0);
        e.mem.set_u32(root + 4, state);
        e.mem.set_f32(child, 5.0);
        let empty = e.mem.alloc(0xe0);
        e.mem.set_u32(child + 4, empty);
        let step = object(&mut e, &[0, 0x99]);
        let holder_block = e.mem.alloc(0xc);
        e.mem.set_u32(holder_block, step);
        e.mem.set_u32(holder_block + 8, 0x99);
        e.mem.set_u32(child + 8, holder_block);
        e.call_log = Some(vec![]);
        e.call(0x0099_5ef0, &args![window, 0x55u32, root]);
        let items = inserted.borrow().clone();
        assert_eq!(
            items,
            vec![
                [0x55, START_FORMAT, 3, root],
                [0x101, COST_FORMAT, 5, 0],
                [0x102, COST_FORMAT, 6, 0],
                [0x101, NODE_FORMAT, 3, child],
                [0x104, COST_FORMAT, 5, 0],
            ]
        );
        let printed = calls(&e, PRINT_TEXT);
        let name_buffer = calls(&e, slot(0x10))[0][1];
        assert_eq!(calls(&e, slot(0x10))[0][2..], [0x104, 0x99]);
        assert_eq!(printed[0][3], 1);
        assert_eq!(printed[1][5], 0xabc);
        assert_eq!(printed[1][..3], [printed[0][0], 0x104, COST_FORMAT]);
        let diff = 3.0f64.to_bits();
        assert_eq!(
            printed[2][2..],
            [
                NODE_FORMAT,
                name_buffer,
                0,
                diff as u32,
                (diff >> 32) as u32,
                MARK_NONE
            ]
        );
        let cost = 2.0f64.to_bits();
        assert_eq!(printed[1][3..5], [cost as u32, (cost >> 32) as u32]);
        assert_eq!(
            calls(&e, 0x009a_a150)[0][1..],
            [printed[0][0], 0x104, 0xffff_ffff]
        );
    }

    #[test]
    fn the_debug_window_lists_states_plan_and_planner() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        list_doubles(&mut e);
        let inserted = tree_doubles(&mut e);
        let (window, map) = window_with_map(&mut e);
        e.mem.set_u32(window + 0xc, 0x3300);
        e.mem.set_u32(window + 8, 0x3400);
        noop(
            &mut e,
            &[
                0x009a_ae70,
                slot(0x10),
                SHOW_WINDOW,
                UPDATE_WINDOW,
                PRINT_TEXT,
            ],
        );
        e.register(0x006a_7f50, |_, _| Ret::default());
        e.register(0x0041_3f40, |_, a| (a[0] + 4).into_ret());
        e.register(0x0099_4a90, |_, _| Ret::default());
        // The planner object's first node (+0x2050) leads up to the root.
        let planner = e.mem.u32(window + 0x30);
        let big = e.mem.alloc(0x2060);
        e.mem.set_u32(big, e.mem.u32(planner));
        e.mem.set_u32(window + 0x30, big);
        let root = entry(&mut e, map, 1, 0);
        let leaf = entry(&mut e, map, 2, root);
        e.mem.set_u32(big + 0x2050, leaf);
        let state = e.mem.alloc(0xe0);
        e.mem.set_u32(root + 4, state);
        e.mem.set_u32(leaf + 4, state);
        e.register(0x0048_d150, |_, a| (a[0] + 8).into_ret());
        e.register(FIND_INDEX, |_, _| 0xffff_ffffu32.into_ret());
        let named = object(&mut e, &[]);
        let holder = e.mem.alloc(0xc);
        e.mem.set_u32(holder, named);
        e.mem.set_u32(leaf + 8, holder);
        // A plan of two steps, one of them null.
        let step = object(&mut e, &[]);
        let element = e.mem.alloc(0x10);
        e.mem.set_u32(element, step);
        e.mem.set_u32(element + 0xc, 0x42);
        let empty = e.mem.alloc(0x10);
        let base = pair(&mut e, empty, element);
        let plan = pair(&mut e, base, 2);
        e.mem.set_u32(window + 0x34, plan);
        e.call_log = Some(vec![]);
        e.call(0x0099_5aa0, &args![window]);
        let items = inserted.borrow().clone();
        assert_eq!(items[0], [TREE_ROOT, INITIAL_WORLD_STATE, 0, 0]);
        assert!(items[1..0x15].iter().all(|i| i[0] == 0x101 && i[3] == 0));
        assert_eq!(calls(&e, 0x009a_ae70).len(), 0x14);
        assert_eq!(calls(&e, 0x009a_ae70)[3][3], 3);
        assert_eq!(calls(&e, 0x009a_ae70)[0][0], big + 0x205c);
        assert_eq!(items[0x15], [TREE_ROOT, COMBAT_PLAN_TITLE, 0, 0]);
        assert_eq!(items[0x16][0], 0x116);
        assert_eq!(calls(&e, slot(0x10))[0][2..], [0x104, 0x42]);
        assert_eq!(items[0x17], [TREE_ROOT, PLANNER_TITLE, 0, root]);
        assert_eq!(e.mem.u32(window + 0x10), 0x118);
        assert_eq!(items[0x18][0], 0x118);
        assert_eq!(items[0x18][3], root);
        assert_eq!(calls(&e, SHOW_WINDOW), vec![vec![0x3400, 10]]);
        assert_eq!(calls(&e, UPDATE_WINDOW), vec![vec![0x3400]]);
    }

    #[test]
    fn the_debug_window_constructor_fills_itself() {
        let mut e = Engine::new();
        let inserted = tree_doubles(&mut e);
        noop(&mut e, &[0x004d_61b0, 0x009a_ae70]);
        e.map(0x011d_ea0c, 4);
        e.set_global(0x011d_ea0c, 0x6600u32);
        e.register(ARRAY_COUNT, |_, _| 0x22u32.into_ret());
        e.register(0x0084_e3a0, |_, _| 0x11u32.into_ret());
        let planner = e.mem.alloc(0x2060);
        let window = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0099_59f0, &args![window, planner, 0u32, 0x77u32])
                .u32(),
            window
        );
        assert_eq!(
            calls(&e, 0x004d_61b0),
            vec![vec![
                window,
                0x11,
                0x22,
                0,
                COMBAT_PLAN_TITLE,
                0x8000_0000,
                0x8000_0000,
                0x320,
                0x258
            ]]
        );
        assert_eq!(e.mem.u32(window + 0x30), planner);
        assert_eq!(e.mem.u32(window + 0x34), 0);
        assert_eq!(e.mem.u32(window + 0x38), 0x77);
        assert_eq!(inserted.borrow().len(), 0x15);
    }

    /// A planner (0x2100 bytes, the test vtable) with an empty open list; slot `+8` (goal)
    /// answers the node's word `+0x28`, slot `+4` appends the node's connections (the words
    /// from `+0x2c`, count `+0x24`... see `node`).
    fn search_planner(e: &mut Engine) -> u32 {
        array_doubles(e);
        let p = e.mem.alloc(0x2100);
        e.mem.set_u32(p, VTABLE);
        object(e, &[]);
        e.register(slot(8), |e, a| e.mem.u32(a[1] + 0x28).into_ret());
        e.register(slot(4), |e, a| {
            let count = e.mem.u32(a[1] + 0x2c);
            for i in 0..count {
                let next = e.mem.u32(a[1] + 0x30 + 4 * i);
                let n = e.mem.u32(a[2] + 8);
                let buffer = e.mem.u32(a[2] + 4);
                e.mem.set_u32(buffer + 4 * n, next);
                e.mem.set_u32(a[2] + 8, n + 1);
            }
            Ret::default()
        });
        // The open list: `006b94a0` pushes onto bucket 0, `006b9540` does nothing.
        e.register(0x006b_94a0, |e, a| {
            let bucket = a[0] + 4;
            let head = e.mem.u32(bucket);
            e.mem.set_u32(a[1] + 0x18, head);
            if head != 0 {
                e.mem.set_u32(head + 0x14, a[1] + 0x18);
            }
            e.mem.set_u32(bucket, a[1]);
            Ret::default()
        });
        noop(e, &[0x006b_9540, 0x0040_fca0]);
        e.register(0x006b_9130, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x20)),
            ..Ret::default()
        });
        e.register(0x0040_1020, |_, _| 0x5500u32.into_ret());
        e.register(0x00aa_42e0, |_, _| 0x5600u32.into_ret());
        p
    }

    /// A search node: cost `+0x20`, goal flag `+0x28`, connections `+0x2c` (count) and `+0x30`.
    fn node(e: &mut Engine, cost: f32, goal: bool, connections: &[u32]) -> u32 {
        let n = e.mem.alloc(0x40);
        e.mem.set_f32(n + 0x20, cost);
        e.mem.set_u32(n + 0x28, u32::from(goal));
        e.mem.set_u32(n + 0x2c, connections.len() as u32);
        for (i, c) in connections.iter().enumerate() {
            e.mem.set_u32(n + 0x30 + 4 * i as u32, *c);
        }
        n
    }

    fn push_open(e: &mut Engine, planner: u32, n: u32) {
        e.call(0x006b_94a0, &args![planner + 0x1ff4, n]);
    }

    #[test]
    fn the_search_finds_a_goal_and_keeps_the_best_node() {
        let mut e = Engine::new();
        let p = search_planner(&mut e);
        // Nothing open: null, and `+0x204c` is the empty take.
        e.mem.set_u32(p + 0x204c, 0x77);
        assert_eq!(e.call(0x0099_6860, &args![p]).u32(), 0);
        assert_eq!(e.mem.u32(p + 0x204c), 0);
        let cheap = node(&mut e, 5.0, false, &[]);
        let goal = node(&mut e, 7.0, true, &[]);
        let start = node(&mut e, 10.0, false, &[cheap, goal]);
        push_open(&mut e, p, start);
        assert_eq!(e.call(0x0099_6860, &args![p]).u32(), goal);
        assert_eq!(e.mem.u32(p + 0x2048), goal);
        assert_eq!(e.mem.u32(cheap + 0x10), start);
        assert_eq!(e.mem.u32(goal + 0x10), start);
        // A dead end: the open list runs out, the best node stays, null comes back.
        let p = search_planner(&mut e);
        let worse = node(&mut e, 12.0, false, &[]);
        let start = node(&mut e, 10.0, false, &[worse]);
        push_open(&mut e, p, start);
        assert_eq!(e.call(0x0099_6860, &args![p]).u32(), 0);
        assert_eq!(e.mem.u32(p + 0x2048), start);
        assert_eq!(e.mem.u32(p + 0x204c), worse);
        // A connection back to the node's own parent ends the search.
        let p = search_planner(&mut e);
        let parent = node(&mut e, 1.0, false, &[]);
        let start = node(&mut e, 10.0, false, &[parent]);
        e.mem.set_u32(start + 0x10, parent);
        push_open(&mut e, p, start);
        assert_eq!(e.call(0x0099_6860, &args![p]).u32(), 0);
    }

    #[test]
    fn a_plan_comes_from_a_found_or_the_best_node() {
        let mut e = Engine::new();
        let p = search_planner(&mut e);
        let manager = object(&mut e, &[]);
        e.register_double(0x00b0_0a00, move |_, _| manager.into_ret());
        e.register(0x0040_fc90, |_, _| 0x31u32.into_ret());
        e.register(slot(0xc), |_, a| (a[1] + 1).into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        let nodes = e.mem.alloc(0x10);
        let links = e.mem.alloc(0x10);
        e.call(SIMPLE_ARRAY_INIT, &args![nodes, 0u32, 0u32]);
        // Nothing found, best node not wanted: false and no plan.
        e.mem.set_u32(p + 0x2048, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_6420, &args![p, nodes, links, 0u32]).bool());
        assert_eq!(e.mem.u8(p + 0x2054), 0x32);
        assert!(calls(&e, ARRAY_SET_SIZE).is_empty());
        // The best node wanted: still false, but the plan is made from it.
        let best = node(&mut e, 1.0, false, &[]);
        e.mem.set_u32(best + 8, 0x4321);
        e.mem.set_u32(p + 0x2048, best);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_6420, &args![p, nodes, links, 1u32]).bool());
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE),
            vec![vec![nodes, 1, 1], vec![links, 0, 1]]
        );
        // A goal found: true.
        let goal = node(&mut e, 1.0, true, &[]);
        push_open(&mut e, p, goal);
        assert!(e.call(0x0099_6420, &args![p, nodes, links, 0u32]).bool());
    }

    #[test]
    fn scrap_array_constructors_take_the_threads_scrap_heap() {
        let mut e = Engine::new();
        noop(&mut e, &[SIMPLE_ARRAY_INIT]);
        e.register(0x0040_1020, |_, _| 0x5500u32.into_ret());
        e.register(0x00aa_42e0, |_, a| (a[0] + 1).into_ret());
        let a = e.mem.alloc(0x14);
        for (address, vtable) in [
            (0x0099_64c0u32, SCRAP_ARRAY_VTABLE_0108DC6C),
            (0x0099_65c0, SCRAP_ARRAY_VTABLE_0108DC94),
            (0x0099_6c00, SCRAP_ARRAY_VTABLE_0108DCD0),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![a]).u32(), a);
            assert_eq!(e.mem.u32(a), vtable);
            assert_eq!(e.mem.u32(a + 0x10), 0x5501);
            assert_eq!(calls(&e, SIMPLE_ARRAY_INIT), vec![vec![a, 0, 0]; 2]);
        }
    }

    #[test]
    fn scrap_array_destructors_release_then_run_the_base() {
        let mut e = Engine::new();
        let released = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let r = released.clone();
        e.register_double(ARRAY_RELEASE_008454F0, move |e, a| {
            r.borrow_mut().push((e.mem.u32(a[0]), a[1]));
            Ret::default()
        });
        let a = e.mem.alloc(0x14);
        for (address, vtable, base) in [
            (
                0x0099_6560u32,
                SCRAP_ARRAY_VTABLE_0108DC6C,
                ARRAY_VTABLE_0108DC80,
            ),
            (
                0x0099_6660,
                SCRAP_ARRAY_VTABLE_0108DC94,
                ARRAY_VTABLE_0108DCA8,
            ),
            (
                0x0099_6c80,
                SCRAP_ARRAY_VTABLE_0108DCD0,
                ARRAY_VTABLE_0108DCBC,
            ),
        ] {
            released.borrow_mut().clear();
            e.call(address, &args![a]);
            assert_eq!(*released.borrow(), vec![(vtable, 1), (base, 1)]);
            assert_eq!(e.mem.u32(a), base);
        }
    }

    #[test]
    fn deleting_destructors_free_on_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[ARRAY_RELEASE_008454F0, FREE]);
        let a = e.mem.alloc(0x14);
        for (address, vtable) in [
            (0x0099_6710u32, ARRAY_VTABLE_0108DC80),
            (0x0099_6740, ARRAY_VTABLE_0108DC80),
            (0x0099_6770, ARRAY_VTABLE_0108DCA8),
            (0x0099_67a0, ARRAY_VTABLE_0108DCA8),
            (0x0099_67d0, ARRAY_VTABLE_0108DCBC),
            (0x0099_6d80, ARRAY_VTABLE_0108DCBC),
        ] {
            for (flags, frees) in [(0u32, 0usize), (1, 1)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![a, flags]).u32(), a, "{address:08x}");
                assert_eq!(e.mem.u32(a), vtable, "{address:08x}");
                assert_eq!(calls(&e, FREE).len(), frees, "{address:08x}");
            }
        }
    }

    /// The fixture of `GetNodeConnections`: a planner with its object (`+0x2058`), scrap heap
    /// doubles, and a node whose state has `goals` (kind, value, extra).
    struct Connections {
        planner: u32,
        node: u32,
        out: u32,
        step: u32,
    }

    /// A step: words +4 root-only (`00994250`), +8 precondition (slot `+0`), +0xc cost (slot
    /// `+0xc`), +0x10 applies (`009aa3b0` on `step + 0x14` reads `+0x14`... the word at +0x14).
    fn connections(e: &mut Engine, root: bool, goals: &[(u32, u32, u32)]) -> Connections {
        list_doubles(e);
        array_doubles(e);
        e.map(SCRAP_ALIGNMENT & !0xfff, 0x1000);
        e.mem.set_u32(SCRAP_ALIGNMENT, 4);
        e.register(SCRAP_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(PLACE_BLOCK, |_, a| a[1].into_ret());
        e.map(COST_LIMIT & !0xfff, 0x1000);
        e.mem.set_f32(COST_LIMIT, 100.0);
        e.register(NODE_PARENT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0099_4250, |e, a| {
            assert_eq!(a[1], 1);
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(0x009a_a3b0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0099_4830, |_, _| Ret::default());
        e.register(0x005e_3fc0, |_, a| (a[0] + 1).into_ret());
        e.register(0x006e_cd40, |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            Ret::default()
        });
        e.register(slot(0), |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(slot(0xc), |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0xc)),
            ..Ret::default()
        });
        e.register(0x0099_4e60, |_, a| a[0].into_ret());
        for list_of in [0x007d_6bb0u32, 0x0040_9140, 0x0048_d150] {
            e.register(list_of, |e, _| {
                let empty = e.mem.alloc(8);
                empty.into_ret()
            });
        }
        e.register(0x0099_50a0, |_, _| 0x5eedu32.into_ret());
        noop(e, &[0x0084_b5c0, 0x009a_ae10, 0x0099_4ea0]);
        e.register(0x0099_47f0, |_, a| a[1].into_ret());
        e.register(WORLD_STATE_IS_TRUE, |_, _| 1u32.into_ret());
        e.register(0x004b_0680, |_, a| a[0].into_ret());
        let planner = e.mem.alloc(0x20c0);
        e.mem.set_u32(planner + 0x2058, 0x6600);
        let state = e.mem.alloc(0xe0);
        e.mem.set_u32(state + 0xdc, goals.len() as u32);
        e.map(STEP_LISTS, 0x100);
        let step = object(e, &[0, 1, 0, 0, 1]);
        e.mem.set_f32(step + 0xc, 3.0);
        let steps = list(e, &[step]);
        for (i, (kind, value, extra)) in goals.iter().enumerate() {
            let goal = pair(e, *kind, *value);
            e.mem.set_u32(state + 0x5c + 8 * i as u32, goal);
            e.mem.set_u32(state + 0x60 + 8 * i as u32, *extra);
            e.mem.set_u32(STEP_LISTS + 4 * kind, steps);
        }
        let node = e.mem.alloc(0x20);
        e.mem.set_f32(node, 2.0);
        e.mem.set_u32(node + 8, state);
        if !root {
            e.mem.set_u32(node + 0x10, 0x9999);
        }
        let out = e.mem.alloc(0x10);
        e.call(SIMPLE_ARRAY_INIT, &args![out, 0u32, 0u32]);
        Connections {
            planner,
            node,
            out,
            step,
        }
    }

    fn entry_answer(e: &mut Engine, status: u32) -> u32 {
        let entry = e.mem.alloc(0x10);
        e.register_double(0x006b_8fe0, move |e, a| {
            e.mem.set_u32(a[3], entry);
            status.into_ret()
        });
        entry
    }

    #[test]
    fn a_step_that_applies_connects_through_a_new_entry() {
        let mut e = Engine::new();
        let c = connections(&mut e, true, &[(3, 0x30, 0x300)]);
        let entry = entry_answer(&mut e, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
        assert_eq!(calls(&e, 0x0084_b5c0), vec![vec![c.out, 0x10]]);
        let asked = calls(&e, 0x006b_8fe0);
        assert_eq!(asked[0][..3], [c.planner, 0x5eed, 5.0f32.to_bits()]);
        let world = e.mem.u32(entry + 8);
        assert_eq!(
            calls(&e, 0x0099_4e60),
            vec![vec![world, c.planner + 0x205c]]
        );
        assert_eq!(e.mem.u32(entry + 4), 0);
        let link = e.mem.u32(entry + 0xc);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(link + 4 * i)).collect();
        assert_eq!(words, vec![c.step, 0x300, 0x30, 0x4444]);
        assert_eq!(array_words(&e, c.out), vec![entry]);
        assert_eq!(
            calls(&e, slot(0)),
            vec![vec![c.step, 0x6600, c.planner + 0x205c, 0x30]]
        );
        assert_eq!(calls(&e, slot(0xc)), vec![vec![c.step, 0x6600, 0x30]]);
        // An existing entry (1) keeps its state but gets the link too.
        let c = connections(&mut e, true, &[(3, 0x30, 0x300)]);
        let entry = entry_answer(&mut e, 1);
        e.mem.set_u32(entry + 8, 0x1111);
        assert!(e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
        assert_eq!(e.mem.u32(entry + 8), 0x1111);
        assert_eq!(array_words(&e, c.out), vec![entry]);
        // Any other answer connects nothing, but the goal counts as reached.
        let c = connections(&mut e, true, &[(3, 0x30, 0x300)]);
        entry_answer(&mut e, 2);
        assert!(e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
        assert!(array_words(&e, c.out).is_empty());
    }

    #[test]
    fn a_goal_without_an_applying_step_empties_the_result() {
        let mut e = Engine::new();
        // The step's check fails.
        let c = connections(&mut e, true, &[(3, 0x30, 0x300)]);
        entry_answer(&mut e, 0);
        e.mem.set_u32(c.step + 0x14, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
        assert_eq!(calls(&e, ARRAY_RELEASE_008454F0), vec![vec![c.out, 0]]);
        // The step's cost is not below the limit.
        let c = connections(&mut e, true, &[(3, 0x30, 0x300)]);
        e.mem.set_f32(c.step + 0xc, 100.0);
        assert!(!e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
        // A root-only step away from the root.
        let c = connections(&mut e, false, &[(3, 0x30, 0x300)]);
        e.mem.set_u32(c.step + 4, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
        assert!(calls(&e, 0x0084_b5c0).is_empty());
        assert!(calls(&e, slot(0)).is_empty());
        // No goals at all: true.
        let c = connections(&mut e, true, &[]);
        assert!(e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
    }

    #[test]
    fn the_new_state_takes_effects_and_unmet_goals() {
        let mut e = Engine::new();
        let c = connections(&mut e, true, &[(3, 0x30, 0x300), (4, 0x40, 0x400)]);
        entry_answer(&mut e, 0);
        // The step's effects: one item (kind 9, value 0x90); its needs: one item (kind 8).
        let effect = pair(&mut e, 9, 0x90);
        let effects = list(&mut e, &[effect]);
        e.register_double(0x007d_6bb0, move |_, _| effects.into_ret());
        let need = pair(&mut e, 8, 0x80);
        let needs = list(&mut e, &[need]);
        e.register_double(0x0048_d150, move |_, _| needs.into_ret());
        e.register(0x0099_47f0, |_, a| (a[1] + a[2]).into_ret());
        noop(&mut e, &[0x009a_ae10, 0x0099_4ea0]);
        // Only the second goal's kind (4) holds in the new state.
        e.register(WORLD_STATE_IS_TRUE, |_, a| u32::from(a[1] == 4).into_ret());
        e.register(0x004b_0680, |_, a| (a[1] << 16 | a[2]).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_50c0, &args![c.planner, c.node, c.out]).bool());
        // Two goals, each connected once through the same step.
        let worlds: Vec<u32> = calls(&e, 0x0099_4e60).iter().map(|w| w[0]).collect();
        assert_eq!(worlds.len(), 2);
        let w = worlds[0];
        let effects = calls(&e, 0x009a_ae10);
        assert_eq!(effects[0], vec![w, 9, 0x90 + 0x30]);
        let added = calls(&e, 0x0099_4ea0);
        assert_eq!(
            added[..2],
            [
                vec![w, 3 << 16 | (0x30 + 0x30), 0x300],
                vec![w, 8 << 16 | (0x80 + 0x30), w]
            ]
        );
        assert_eq!(array_words(&e, c.out).len(), 2);
    }
}
