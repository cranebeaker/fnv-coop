//! `fallout shared/tescondition.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The evaluation of a condition list with its AND/OR grouping
//! (`TESCondition::IsTrue_ov2`, `00680c60`), and the list's record
//! handling: one `CTDA` chunk loaded into a new item appended to the list
//! (`00680960`), the items' `InitItem` (`00680a20`) and `Save`
//! (`00680910`), the item constructor (`006811e0`) and the append
//! (`0067e420`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The list head of the condition list (`006815c0`, `this`): also, on a
/// node, the address of the node's item word.
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// Whether a list node is the end of the list (`008256d0`, `ECX` = node;
/// the exe has no name for it).
const NODE_IS_END: u32 = 0x0082_56d0;
/// `005693d0(ECX = reference)`: a replacement for the reference argument
/// when it answers nonzero.
const REFERENCE_REPLACEMENT: u32 = 0x0056_93d0;
/// `00413f40(ECX = item)`: the address of the item's 16-bit function
/// number.
const ITEM_FUNCTION_NUMBER: u32 = 0x0041_3f40;
/// `005ec040(ECX = item)`: whether the item is joined to the next by OR.
const ITEM_IS_OR: u32 = 0x005e_c040;
/// `00681600(ECX = item, arg0, reference, &out flag)` (`RET 0xC`): the
/// result of one condition item.
const ITEM_EVALUATE: u32 = 0x0068_1600;
/// The next list node (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the current chunk's
/// tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The chunk tag `CTDA` (one condition item).
const CHUNK_CTDA: u32 = 0x4144_5443;
/// `operator new` (`00401000`, cdecl, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The size of a `TESConditionItem` (Xbox PDB `0x1c`; the PC code allocates
/// the same).
const CONDITION_ITEM_SIZE: u32 = 0x1c;
/// The game's `memset` wrapper (`00403d30`, cdecl: destination, value,
/// count).
const MEMSET: u32 = 0x0040_3d30;
/// `00681220` (`ECX` = item, one stack word; `RET 4`): sets the item up for a
/// function number (unnamed in the map; translated in `run_00681220.rs`);
/// called by the item constructor with 0x1048.
const ITEM_SETUP: u32 = 0x0068_1220;
/// The word the item constructor hands to `00681220`.
const ITEM_SETUP_VALUE: u32 = 0x1048;
/// `TESConditionItem::Load` (`00681950`, `ECX` = item, one stack word: the
/// file).
const ITEM_LOAD: u32 = 0x0068_1950;
/// `TESConditionItem::InitItem` (`006819e0`, `ECX` = item, one stack word).
const ITEM_INIT_ITEM: u32 = 0x0068_19e0;
/// `TESConditionItem::Save` (`00681840`, `ECX` = item, no stack words).
const ITEM_SAVE: u32 = 0x0068_1840;
/// The list append (`00905820`, `ECX` = head node, one stack word: the
/// address of the item word; nothing is added for a null item).
const LIST_APPEND: u32 = 0x0090_5820;

// Translated from 00680c30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `TESCondition::IsTrue` (Xbox PDB; `thiscall`, `RET 8`): [`tes_condition_is_true_ov2`]
/// with its two arguments, the address of a byte of its own frame for the
/// out flag (not read afterwards) and no filter.
pub fn tes_condition_is_true(e: &mut Engine, this: Ptr, arg0: u32, reference: u32) -> bool {
    e.with_stack(4, |e, out_flag| {
        tes_condition_is_true_ov2(e, this, arg0, reference, out_flag, 0)
    })
}

// Translated from 00680c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCondition::IsTrue_ov2` (Xbox PDB): evaluates the condition list at
/// `this` (`ECX`; `RET 0x10`, four stack words) and returns whether it
/// holds. `arg0` and `reference` are handed to each item (`reference` is
/// first replaced by `005693d0(reference)` when that answers nonzero);
/// `out_flag` is a byte that is set to 1 while a failing item reports its
/// own flag (or an OR group fails without an all-false item) and is
/// cleared at the end when the list holds or an item failed outright.
/// With `filter` set, items whose function number is below 0x44, or above
/// 0x48 (except 0x174), are not evaluated and count as true.
///
/// An item joined by OR starts a group that runs to the next item that is
/// not; the group is true when any item is. (The decompiler's output was
/// checked against the disassembly: the byte locals are `all_true` (-9),
/// `current` (-4), `in_or_group` (-0xa), `or_flag` (-1), `or_all_false`
/// (-3) and `failed` (-2).)
pub fn tes_condition_is_true_ov2(
    e: &mut Engine,
    this: Ptr,
    arg0: u32,
    reference: u32,
    out_flag: Ptr,
    filter: u8,
) -> bool {
    let mut node = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
    if e.call(NODE_IS_END, &args![node]).bool() {
        e.mem.set_u8(out_flag.addr(), 0);
        return true;
    }
    let mut all_true = true;
    let mut current = false;
    let mut in_or_group = false;
    let mut or_flag = false;
    let mut or_all_false = false;
    e.mem.set_u8(out_flag.addr(), 0);
    let mut failed = false;
    let mut reference = reference;
    if reference != 0 {
        let replacement = e.call(REFERENCE_REPLACEMENT, &args![reference]).u32();
        if replacement != 0 {
            reference = replacement;
        }
    }
    // The item's own flag byte: a local of the game's stack, passed by address.
    e.with_stack(4, |e, local| {
        while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(slot);
            let mut skip = false;
            if filter != 0 {
                let function = e.call(ITEM_FUNCTION_NUMBER, &args![item]).u32();
                let function = e.mem.u16(function) as i32;
                if function < 0x44 || (function > 0x48 && function != 0x174) {
                    skip = true;
                }
            }
            e.mem.set_u8(local.addr(), 0);
            if !in_or_group {
                if !e.call(ITEM_IS_OR, &args![item]).bool() {
                    current = if skip {
                        true
                    } else {
                        e.call(ITEM_EVALUATE, &args![item, arg0, reference, local])
                            .bool()
                    };
                    all_true = all_true && current;
                    if e.mem.u8(local.addr()) != 0 {
                        e.mem.set_u8(out_flag.addr(), 1);
                    } else if !current {
                        failed = true;
                    }
                } else {
                    current = e
                        .call(ITEM_EVALUATE, &args![item, arg0, reference, local])
                        .bool();
                    in_or_group = true;
                    or_all_false = false;
                    or_flag = e.mem.u8(local.addr()) != 0;
                    if !or_flag && !current {
                        or_all_false = true;
                    }
                }
            } else {
                if !current {
                    current = if skip {
                        skip
                    } else {
                        e.call(ITEM_EVALUATE, &args![item, arg0, reference, local])
                            .bool()
                    };
                    or_flag = or_flag || e.mem.u8(local.addr()) != 0;
                    if e.mem.u8(local.addr()) == 0 && !current {
                        or_all_false = true;
                    }
                }
                if !e.call(ITEM_IS_OR, &args![item]).bool() {
                    in_or_group = false;
                    all_true = all_true && current;
                    if !all_true {
                        if or_flag && !or_all_false {
                            e.mem.set_u8(out_flag.addr(), 1);
                        } else {
                            failed = true;
                        }
                    }
                }
            }
            if !all_true && !in_or_group && e.mem.u8(out_flag.addr()) == 0 {
                break;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    if in_or_group {
        all_true = all_true && current;
        if !all_true {
            if or_flag && !or_all_false {
                e.mem.set_u8(out_flag.addr(), 1);
            } else {
                failed = true;
            }
        }
    }
    if all_true || failed {
        e.mem.set_u8(out_flag.addr(), 0);
    }
    all_true
}

// Translated from 0067e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<TESConditionItem_P>::AddTail` (Xbox PDB): appends `item`
/// to the list whose head node is `this` (`ECX`; `RET 4`). The item word
/// stays in the caller's stack slot and its address is handed to the
/// append `00905820`, on the head that `006815c0(this)` answers.
pub fn bs_simple_list_tes_condition_item_add_tail(e: &mut Engine, this: Ptr, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        let head = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
        e.call(LIST_APPEND, &args![head, slot]);
    });
}

// Translated from 00680910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCondition::Save` (Xbox PDB): runs `TESConditionItem::Save`
/// (`00681840`) on every item of the list at `this` (`ECX`, no stack
/// words), from the head until a null node or the empty end node.
pub fn tes_condition_save(e: &mut Engine, this: Ptr) {
    let mut node = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        e.call(ITEM_SAVE, &args![item]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00680960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00680960 - the exception-unwinding frame (FS:[0]) around the item allocation and the security-cookie bookkeeping are not translated.
/// `TESCondition::Load` (Xbox PDB): with a `file` (`RET 4`) whose current
/// chunk is `CTDA`, allocates a 0x1c-byte item (`operator new`, then the
/// item constructor `006811e0`; null when the allocation fails), has
/// `TESConditionItem::Load` (`00681950`) read it from the file, and appends
/// it to the list at `this` (`0067e420`). Any other chunk, or no file,
/// does nothing. The item's `Load` runs on the allocation result even when
/// it is null, as in the exe.
pub fn tes_condition_load(e: &mut Engine, this: Ptr, file: Ptr) {
    if file.is_null() {
        return;
    }
    if e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_CTDA {
        return;
    }
    let memory = e.call(OPERATOR_NEW, &args![CONDITION_ITEM_SIZE]).u32();
    let item = if memory != 0 {
        fn_006811e0(e, Ptr::new(memory)).addr()
    } else {
        0
    };
    e.call(ITEM_LOAD, &args![item, file]);
    bs_simple_list_tes_condition_item_add_tail(e, this, item);
}

// Translated from 00680a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCondition::InitItem` (Xbox PDB): runs `TESConditionItem::InitItem`
/// (`006819e0`) with `owner` (the one stack word, `RET 4`; the quest when a
/// quest calls it) on every item of the list at `this`, from the head until
/// a null node or the empty end node.
pub fn tes_condition_init_item(e: &mut Engine, this: Ptr, owner: u32) {
    let mut node = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        e.call(ITEM_INIT_ITEM, &args![item, owner]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 006811e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The condition item constructor (unnamed in the map; `ECX` = item, no
/// stack words): clears the item's 0x1c bytes (`memset`), stores 0 at
/// `+0x18`, hands 0x1048 to `00681220` and returns `this`.
pub fn fn_006811e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMSET, &args![this, 0u32, CONDITION_ITEM_SIZE]);
    e.mem.set_u32(this.addr() + 0x18, 0);
    e.call(ITEM_SETUP, &args![this, ITEM_SETUP_VALUE]);
    this
}

/// `006815d0` (`thiscall`, `RET 4`): whether two condition items differ
/// (`AL`), item and other item.
const ITEMS_DIFFER: u32 = 0x0068_15d0;

// Translated from 00680b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two condition lists differ (no name in the engine map): walks
/// both lists (`006815c0` on each list gives its first node) in step; true
/// as soon as `this` ends before `other`, or a pair of items differs
/// (`006815d0`); when `other` ends, true only when `this` still has an
/// item.
pub fn fn_00680b70(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let mut mine = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
    let mut theirs = e.call(LIST_NODE_ITEM_SLOT, &args![other]).u32();
    loop {
        if theirs == 0 || e.call(NODE_IS_END, &args![theirs]).bool() {
            return mine != 0 && !e.call(NODE_IS_END, &args![mine]).bool();
        }
        if mine == 0 || e.call(NODE_IS_END, &args![mine]).bool() {
            return true;
        }
        let mine_slot = e.call(LIST_NODE_ITEM_SLOT, &args![mine]).u32();
        let mine_item = e.mem.u32(mine_slot);
        let theirs_slot = e.call(LIST_NODE_ITEM_SLOT, &args![theirs]).u32();
        let theirs_item = e.mem.u32(theirs_slot);
        if e.call(ITEMS_DIFFER, &args![mine_item, theirs_item]).bool() {
            return true;
        }
        mine = e.call(NODE_NEXT, &args![mine]).u32();
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
    }
}

// Translated from 00680f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCondition::GetConditionItem` (Xbox PDB): the item at `index` of the
/// list (counting nodes until the list ends), or null.
pub fn tes_condition_get_condition_item(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let mut position = 0i32;
    let mut node = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        if position == index {
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            return e.mem.u32(slot);
        }
        position = position.wrapping_add(1);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

/// `00680fe0` (`thiscall`): empties the condition list before a copy.
const CONDITION_CLEAR: u32 = 0x0068_0fe0;
/// The copy `00401460` and `memcmp` (`00ec4835`), both `cdecl (a, b,
/// size)`.
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;

// Translated from 00680a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00680a70 - the C++ exception frame (FS:[0], state for the item allocation) is not translated
/// Unnamed in the engine map (`thiscall`, two words; the copy of a
/// condition list): empties this list (`00680fe0`), then for each item of
/// `other` (while the node is not null and not the end) a new 0x1C-byte
/// item ([`fn_006811e0`], or null when the allocation fails) gets the
/// item's bytes ([`fn_00681590`], given `second` too) and is added after
/// this list's node (`00905820`), which moves on when it has a next node.
pub fn fn_00680a70(e: &mut Engine, this: Ptr, other: u32, second: u32) {
    e.call(CONDITION_CLEAR, &args![this]);
    let mut mine = e.call(LIST_NODE_ITEM_SLOT, &args![this]).u32();
    let mut theirs = e.call(LIST_NODE_ITEM_SLOT, &args![other]).u32();
    while theirs != 0 && e.call(NODE_IS_END, &args![theirs]).u8() == 0 {
        let block = e.call(OPERATOR_NEW, &args![CONDITION_ITEM_SIZE]).u32();
        let item = if block != 0 {
            fn_006811e0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            let source = e.call(LIST_NODE_ITEM_SLOT, &args![theirs]).u32();
            let source = e.mem.u32(source);
            fn_00681590(e, Ptr::new(item), source, second);
            e.call(LIST_APPEND, &args![mine, slot]);
        });
        if e.call(NODE_NEXT, &args![mine]).u32() != 0 {
            mine = e.call(NODE_NEXT, &args![mine]).u32();
        }
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
    }
}

// Translated from 00681590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words; the copy of a
/// condition item): the 0x1C bytes of `source` (through `006815c0`) into
/// this item; the second word is not read.
pub fn fn_00681590(e: &mut Engine, this: Ptr, source: u32, _unused_2: u32) {
    let from = e.call(LIST_NODE_ITEM_SLOT, &args![source]).u32();
    e.call(COPY_BYTES, &args![this, from, CONDITION_ITEM_SIZE]);
}

// Translated from 006815d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; whether two condition
/// items differ): `memcmp` of `other` (through `006815c0`) with this item
/// over 0x1C bytes is non-zero.
pub fn fn_006815d0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let theirs = e.call(LIST_NODE_ITEM_SLOT, &args![other]).u32();
    e.call(MEMCMP, &args![theirs, this, CONDITION_ITEM_SIZE])
        .u32()
        != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067e420,
            bs_simple_list_tes_condition_item_add_tail(Ptr, u32)
        ),
        entry!(0x00680910, tes_condition_save(Ptr)),
        entry!(0x00680960, tes_condition_load(Ptr, Ptr)),
        entry!(0x00680a20, tes_condition_init_item(Ptr, u32)),
        entry!(0x00680c30, tes_condition_is_true(Ptr, u32, u32) -> bool),
        entry!(0x00680c60, tes_condition_is_true_ov2(Ptr, u32, u32, Ptr, u8) -> bool),
        entry!(0x006811e0, fn_006811e0(Ptr) -> Ptr),
        entry!(0x0068_0b70, fn_00680b70(Ptr, u32) -> bool),
        entry!(0x0068_0f70, tes_condition_get_condition_item(Ptr, i32) -> u32),
        entry!(0x0068_0a70, fn_00680a70(Ptr, u32, u32)),
        entry!(0x0068_1590, fn_00681590(Ptr, u32, u32)),
        entry!(0x0068_15d0, fn_006815d0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_is_the_unfiltered_evaluation() {
        let mut e = setup();
        let yes = item(&mut e, false, true, false, 0x10);
        let no = item(&mut e, false, false, false, 0x10);
        let holds = list(&mut e, &[yes]);
        let fails = list(&mut e, &[yes, no]);
        assert_eq!(e.call(0x00680c30, &[holds, 1, 0x300]).u32() & 0xff, 1);
        assert_eq!(e.call(0x00680c30, &[fails, 1, 0x300]).u32() & 0xff, 0);
        // Function 0x10 is below 0x44: a filter would have skipped it, so it
        // was evaluated (no filter).
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, w)| *a == ITEM_EVALUATE && w[0] == no));
    }

    /// An item: +0 or-joined, +1 result, +2 own flag, +4 function number.
    fn item(e: &mut Engine, or: bool, result: bool, flag: bool, function: u16) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_u8(p, or as u8);
        e.mem.set_u8(p + 1, result as u8);
        e.mem.set_u8(p + 2, flag as u8);
        e.mem.set_u16(p + 4, function);
        p
    }

    /// A list of nodes `{item, next}`; the head's address answers `this`.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &it in items.iter().rev() {
            let n = e.mem.alloc(8);
            e.mem.set_u32(n, it);
            e.mem.set_u32(n + 4, next);
            next = n;
        }
        next
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn setup() -> Engine {
        let mut e = Engine::new();
        // A node's item word is at the node itself; a null node is the end.
        e.register(LIST_NODE_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(NODE_IS_END, |_, a| ret((a[0] == 0) as u32));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(REFERENCE_REPLACEMENT, |_, a| {
            ret(if a[0] == 0x100 { 0x200 } else { 0 })
        });
        e.register(ITEM_FUNCTION_NUMBER, |_, a| ret(a[0] + 4));
        e.register(ITEM_IS_OR, |e, a| ret(e.mem.u8(a[0]) as u32));
        e.register(ITEM_EVALUATE, |e, a| {
            e.mem.set_u8(a[3], e.mem.u8(a[0] + 2));
            ret(e.mem.u8(a[0] + 1) as u32)
        });
        e.call_log = Some(vec![]);
        e
    }

    fn run(e: &mut Engine, items: &[u32], filter: u8) -> (bool, u8) {
        let head = list(e, items);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        let ok = e
            .call(
                0x0068_0c60,
                &args![
                    Ptr::<()>::new(head),
                    5u32,
                    0x100u32,
                    Ptr::<()>::new(out),
                    filter
                ],
            )
            .bool();
        (ok, e.mem.u8(out))
    }

    fn evaluations(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ITEM_EVALUATE)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn an_empty_list_is_true_and_clears_the_flag() {
        let mut e = setup();
        assert_eq!(run(&mut e, &[], 0), (true, 0));
        assert!(evaluations(&e).is_empty());
    }

    #[test]
    fn and_items_stop_at_the_first_false_one() {
        let mut e = setup();
        let t = item(&mut e, false, true, false, 0x50);
        let f = item(&mut e, false, false, false, 0x50);
        let never = item(&mut e, false, true, false, 0x50);
        assert_eq!(run(&mut e, &[t, t], 0), (true, 0));
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, &[t, f, never], 0), (false, 0));
        let seen = evaluations(&e);
        assert_eq!(seen.len(), 2);
        // Each item gets the first argument and the replaced reference.
        assert_eq!(seen[0][1..3], [5, 0x200]);
    }

    #[test]
    fn a_failing_item_with_its_own_flag_keeps_the_out_flag_and_goes_on() {
        let mut e = setup();
        let flagged = item(&mut e, false, false, true, 0x50);
        let t = item(&mut e, false, true, false, 0x50);
        assert_eq!(run(&mut e, &[flagged, t], 0), (false, 1));
        assert_eq!(evaluations(&e).len(), 2);
    }

    #[test]
    fn an_or_group_is_true_when_any_item_is() {
        let mut e = setup();
        let a = item(&mut e, true, false, false, 0x50);
        let b = item(&mut e, false, true, false, 0x50);
        let f = item(&mut e, false, false, false, 0x50);
        let t = item(&mut e, false, true, false, 0x50);
        // false OR true, then AND true.
        assert_eq!(run(&mut e, &[a, b, t], 0), (true, 0));
        // true OR (not evaluated), then AND false.
        let a2 = item(&mut e, true, true, false, 0x50);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, &[a2, b, f], 0), (false, 0));
        assert_eq!(evaluations(&e).len(), 2);
    }

    #[test]
    fn a_false_or_group_fails_and_a_flagged_one_reports_it() {
        let mut e = setup();
        let a = item(&mut e, true, false, false, 0x50);
        let b = item(&mut e, false, false, false, 0x50);
        assert_eq!(run(&mut e, &[a, b], 0), (false, 0));
        // A group left open at the end of the list closes after the loop.
        assert_eq!(run(&mut e, &[a], 0), (false, 0));
        // Every item of the group flagged and false: the flag is reported
        // only when no item was false without a flag.
        let af = item(&mut e, true, false, true, 0x50);
        let bf = item(&mut e, false, false, true, 0x50);
        assert_eq!(run(&mut e, &[af, bf], 0), (false, 1));
    }

    /// Doubles for the record functions: a file's current chunk tag is its
    /// first word; `operator new` answers null when `fail_new` is set.
    fn record_setup_with(fail_new: bool) -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NODE_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(NODE_IS_END, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(GET_TES_CHUNK, |e, a| ret(e.mem.u32(a[0])));
        e.register_double(OPERATOR_NEW, move |e, a| {
            ret(if fail_new { 0 } else { e.mem.alloc(a[0]) })
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(ITEM_SETUP, |_, _| ret(0));
        e.register(ITEM_LOAD, |_, _| ret(0));
        e.register(ITEM_INIT_ITEM, |_, _| ret(0));
        e.register(ITEM_SAVE, |_, _| ret(0));
        // The append records the item word it is handed.
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 0x100, item);
            ret(0)
        });
        e.call_log = Some(vec![]);
        e
    }

    fn record_setup() -> Engine {
        record_setup_with(false)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn add_tail_hands_the_address_of_the_item_word_to_the_append() {
        let mut e = record_setup();
        let list = e.mem.alloc(0x200);
        e.call(0x0067_e420, &args![Ptr::<()>::new(list), 0x1234u32]);
        let appends = calls_to(&e, LIST_APPEND);
        assert_eq!(appends.len(), 1);
        assert_eq!(appends[0][0], list);
        assert_eq!(e.mem.u32(list + 0x100), 0x1234);
    }

    #[test]
    fn load_reads_one_ctda_chunk_into_a_new_item() {
        let mut e = record_setup();
        let list = e.mem.alloc(0x200);
        let file = e.mem.alloc(4);
        e.mem.set_u32(file, CHUNK_CTDA);
        e.call(
            0x0068_0960,
            &args![Ptr::<()>::new(list), Ptr::<()>::new(file)],
        );
        let news = calls_to(&e, OPERATOR_NEW);
        assert_eq!(news, vec![vec![0x1c]]);
        let loads = calls_to(&e, ITEM_LOAD);
        assert_eq!(loads.len(), 1);
        let item = loads[0][0];
        assert_ne!(item, 0);
        assert_eq!(loads[0][1], file);
        // The constructor ran on it, and the item went into the list.
        assert_eq!(calls_to(&e, ITEM_SETUP), vec![vec![item, 0x1048]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![item, 0, 0x1c]]);
        assert_eq!(e.mem.u32(list + 0x100), item);
    }

    #[test]
    fn load_ignores_other_chunks_and_a_null_file() {
        let mut e = record_setup();
        let list = e.mem.alloc(0x200);
        let file = e.mem.alloc(4);
        e.mem.set_u32(file, 0x4d414e4e);
        e.call(
            0x0068_0960,
            &args![Ptr::<()>::new(list), Ptr::<()>::new(file)],
        );
        e.call(0x0068_0960, &args![Ptr::<()>::new(list), 0u32]);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        assert!(calls_to(&e, ITEM_LOAD).is_empty());
        assert_eq!(calls_to(&e, GET_TES_CHUNK).len(), 1);
    }

    #[test]
    fn load_with_a_failed_allocation_loads_and_appends_a_null_item() {
        let mut e = record_setup_with(true);
        let list = e.mem.alloc(0x200);
        e.mem.set_u32(list + 0x100, 0xffff);
        let file = e.mem.alloc(4);
        e.mem.set_u32(file, CHUNK_CTDA);
        e.call(
            0x0068_0960,
            &args![Ptr::<()>::new(list), Ptr::<()>::new(file)],
        );
        assert!(calls_to(&e, ITEM_SETUP).is_empty());
        assert_eq!(calls_to(&e, ITEM_LOAD), vec![vec![0, file]]);
        assert_eq!(e.mem.u32(list + 0x100), 0);
    }

    #[test]
    fn init_item_and_save_visit_every_item_until_the_end_node() {
        let mut e = record_setup();
        // Two nodes with items, then an empty end node.
        let end = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x22);
        e.mem.set_u32(second + 4, end);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x11);
        e.mem.set_u32(first + 4, second);
        e.call(0x0068_0a20, &args![Ptr::<()>::new(first), 0x77u32]);
        assert_eq!(
            calls_to(&e, ITEM_INIT_ITEM),
            vec![vec![0x11, 0x77], vec![0x22, 0x77]]
        );
        e.call(0x0068_0910, &args![Ptr::<()>::new(first)]);
        assert_eq!(calls_to(&e, ITEM_SAVE), vec![vec![0x11], vec![0x22]]);
        // An empty list (its head is the end node) visits nothing.
        e.call_log = Some(vec![]);
        e.call(0x0068_0a20, &args![Ptr::<()>::new(end), 0x77u32]);
        e.call(0x0068_0910, &args![Ptr::<()>::new(end)]);
        assert!(calls_to(&e, ITEM_INIT_ITEM).is_empty());
        assert!(calls_to(&e, ITEM_SAVE).is_empty());
    }

    #[test]
    fn the_item_constructor_clears_the_item_and_sets_it_up() {
        let mut e = record_setup();
        let item = e.mem.alloc(0x1c);
        for i in 0..0x1c {
            e.mem.set_u8(item + i, 0xaa);
        }
        let back = e.call(0x0068_11e0, &args![Ptr::<()>::new(item)]).u32();
        assert_eq!(back, item);
        assert_eq!(e.mem.u32(item + 0x18), 0);
        assert_eq!(e.mem.u32(item), 0);
        assert_eq!(calls_to(&e, ITEM_SETUP), vec![vec![item, 0x1048]]);
    }

    #[test]
    fn the_filter_skips_items_outside_the_function_range() {
        let mut e = setup();
        for (function, evaluated) in [
            (0x10u16, false),
            (0x43, false),
            (0x44, true),
            (0x48, true),
            (0x49, false),
            (0x174, true),
            (0x175, false),
        ] {
            let f = item(&mut e, false, false, false, function);
            e.call_log = Some(vec![]);
            let (ok, _) = run(&mut e, &[f], 1);
            assert_eq!(evaluations(&e).len(), evaluated as usize, "{function:#x}");
            assert_eq!(ok, !evaluated, "{function:#x}");
        }
        // Without the filter every item is evaluated.
        let f = item(&mut e, false, false, false, 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, &[f], 0), (false, 0));
        assert_eq!(evaluations(&e).len(), 1);
    }
}

#[cfg(test)]
mod b039_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Lists of nodes [item, next]; a node with item 0 is the end marker
    /// (`008256d0` answers true for it).
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let nodes = e.mem.alloc(8 * items.len() as u32 + 8);
        for (i, item) in items.iter().enumerate() {
            let node = nodes + 8 * i as u32;
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, node + 8);
        }
        let last = nodes + 8 * items.len() as u32;
        e.mem.set_u32(last, 0);
        e.mem.set_u32(last + 4, 0);
        nodes
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NODE_ITEM_SLOT, |_, a| word(a[0]));
        e.register(NODE_IS_END, |e, a| word(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(ITEMS_DIFFER, |_, a| word(u32::from(a[0] != a[1])));
        e
    }

    #[test]
    fn fn_00680b70_compares_the_lists_item_by_item() {
        let mut e = engine();
        let a = list(&mut e, &[1, 2]);
        let b = list(&mut e, &[1, 2]);
        let c = list(&mut e, &[1, 3]);
        let shorter = list(&mut e, &[1]);
        assert!(!e.call(0x0068_0b70, &args![a, b]).bool());
        assert!(e.call(0x0068_0b70, &args![a, c]).bool());
        // `this` has more items: true; `this` has fewer: true.
        assert!(e.call(0x0068_0b70, &args![a, shorter]).bool());
        assert!(e.call(0x0068_0b70, &args![shorter, a]).bool());
        // A null first node.
        let empty = list(&mut e, &[]);
        assert!(!e.call(0x0068_0b70, &args![empty, empty]).bool());
        e.register(LIST_NODE_ITEM_SLOT, |_, a| {
            word(if a[0] == 0x10 { 0 } else { a[0] })
        });
        assert!(e.call(0x0068_0b70, &args![0x10u32, a]).bool());
        assert!(!e.call(0x0068_0b70, &args![0x10u32, 0x10u32]).bool());
    }

    #[test]
    fn get_condition_item_counts_nodes() {
        let mut e = engine();
        let a = list(&mut e, &[0x71, 0x72, 0x73]);
        assert_eq!(e.call(0x0068_0f70, &args![a, 0i32]).u32(), 0x71);
        assert_eq!(e.call(0x0068_0f70, &args![a, 2i32]).u32(), 0x73);
        assert_eq!(e.call(0x0068_0f70, &args![a, 3i32]).u32(), 0);
        assert_eq!(e.call(0x0068_0f70, &args![a, -1i32]).u32(), 0);
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Lists are (item, next) nodes; `006815c0` answers its argument.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(NODE_IS_END, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        for addr in [CONDITION_CLEAR, COPY_BYTES, MEMSET, ITEM_SETUP] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            let n = e.mem.u32(STATE);
            e.mem.set_u32(STATE + 4 + 4 * n, item);
            e.mem.set_u32(STATE, n + 1);
            Ret::default()
        });
        e
    }

    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    #[test]
    fn copy_makes_new_items() {
        let mut e = engine();
        let this = list(&mut e, &[]);
        let other = list(&mut e, &[0x100, 0x200]);
        e.call_log = Some(vec![]);
        e.call(0x0068_0a70, &args![this, other, 9u32]);
        assert_eq!(calls(&e, CONDITION_CLEAR), vec![vec![this]]);
        assert_eq!(e.mem.u32(STATE), 2);
        let first = e.mem.u32(STATE + 4);
        let copies = calls(&e, COPY_BYTES);
        assert_eq!(copies[0], vec![first, 0x100, 0x1c]);
        assert_eq!(copies[1][1], 0x200);
        assert_eq!(calls(&e, ITEM_SETUP)[0], vec![first, ITEM_SETUP_VALUE]);
        assert!(calls(&e, LIST_APPEND).iter().all(|c| c[0] == this));
        // An empty source copies nothing.
        let empty = list(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0068_0a70, &args![this, empty, 9u32]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn item_copy_and_compare() {
        let mut e = engine();
        e.register(MEMCMP, |e, _| e.mem.u32(STATE + 0x20).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0068_1590, &args![0x1000u32, 0x2000u32, 7u32]);
        assert_eq!(calls(&e, COPY_BYTES), vec![vec![0x1000, 0x2000, 0x1c]]);
        assert!(!e.call(0x0068_15d0, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(calls(&e, MEMCMP), vec![vec![0x2000, 0x1000, 0x1c]]);
        e.mem.set_u32(STATE + 0x20, 0x100);
        assert!(e.call(0x0068_15d0, &args![0x1000u32, 0x2000u32]).bool());
    }
}
