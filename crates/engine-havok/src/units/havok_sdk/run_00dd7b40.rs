//! `Havok SDK/run_00dd7b40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two unnamed members of a free-list allocator: an object with a singly linked list of free
//! elements (`+0`, each element's first word the next one), the element size (`+4`), the block
//! being worked on (`+8`) and the blocks still to look at (`+0xc`), the blocks found free
//! (`+0x10`), and a pair of words (`+0x20`, `+0x24`). A block record has its next block at `+0`,
//! the address of its first element at `+8` and its element count at `+0xc`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router getter (`00c85750`): its first word is the cursor of the thread's stack
/// memory, which the first function advances to borrow an array and puts back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The sort (`00dd7d00`, cdecl: the array, 0, the last index, the comparison function) and the
/// comparison function the first function passes (`00dd70f0`).
const QUICK_SORT: u32 = 0x00dd_7d00;
const NODE_COMPARE: u32 = 0x00dd_70f0;

/// Puts the router's stack cursor back to `cursor` (the router is asked again, as in the code).
fn restore_stack(e: &mut Engine, cursor: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, cursor);
}

// Translated from 00dd7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (the free list), `block`). Takes the
/// free elements that lie in `block` (`start <= element < start + count * size`) out of the
/// free list, noting them in an array of `(count * 4 + 0x7f) & !0x7f` bytes borrowed from the
/// router's stack memory. Answers 1 when the block is entirely free: its element count
/// equals the number taken (when the word at `this + 0x20` lies in the block that number is
/// increased by `([this + 0x24] - [this + 0x20]) / size`, and both words are then cleared).
/// Otherwise it puts the taken elements back, sorted (`00dd7d00` with `00dd70f0`) and linked in
/// that order in front of the list, and answers 0. The stack cursor is restored either way.
/// A zero element size faults in the code's `DIV`; it panics here.
pub fn fn_00dd7b40(e: &mut Engine, this: u32, block: u32) -> u32 {
    let count = e.mem.u32(block + 0xc);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let stack = e.mem.u32(router);
    let borrowed = (count.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(stack);
    e.mem.set_u32(router, borrowed);
    let start = e.mem.u32(block + 8);
    let end = e
        .mem
        .u32(block + 0xc)
        .wrapping_mul(e.mem.u32(this + 4))
        .wrapping_add(start);
    let mut taken = 0u32;
    let mut previous = this;
    let mut element = e.mem.u32(this);
    while element != 0 {
        if element >= start && element < end {
            e.mem.set_u32(stack + 4 * taken, element);
            let next = e.mem.u32(element);
            e.mem.set_u32(previous, next);
            taken += 1;
        } else {
            previous = element;
        }
        element = e.mem.u32(element);
    }
    let marker = e.mem.u32(this + 0x20);
    let whole = if marker >= start && marker < end {
        let size = e.mem.u32(this + 4);
        assert!(size != 0, "divide by zero at 00dd7bc0 (element size 0)");
        let covered = e.mem.u32(this + 0x24).wrapping_sub(marker) / size;
        if covered.wrapping_add(taken) == e.mem.u32(block + 0xc) {
            e.mem.set_u32(this + 0x20, 0);
            e.mem.set_u32(this + 0x24, 0);
            true
        } else {
            false
        }
    } else {
        taken == e.mem.u32(block + 0xc)
    };
    if whole {
        restore_stack(e, stack);
        return 1;
    }
    if (taken as i32) > 0 {
        if (taken as i32) > 1 {
            e.call(
                QUICK_SORT,
                &args![stack, 0u32, taken.wrapping_sub(1), NODE_COMPARE],
            );
        }
        let mut chained = e.mem.u32(stack);
        let mut index = 1u32;
        while (index as i32) < (taken as i32) {
            let next = e.mem.u32(stack + 4 * index);
            index += 1;
            e.mem.set_u32(chained, next);
            chained = next;
        }
        let last = e.mem.u32(stack + 4 * taken - 4);
        let head = e.mem.u32(this);
        e.mem.set_u32(last, head);
        let first = e.mem.u32(stack);
        e.mem.set_u32(this, first);
    }
    restore_stack(e, stack);
    0
}

// Translated from 00dd7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `this`, `out_done` (a pointer to a byte),
/// `count`, `out_count` (a pointer to a word)). Looks at up to `count` blocks with `00dd7b40`.
/// With a current block (the word at `+8`) it looks at the block after it (its word at `+0`),
/// otherwise at the first of the list at `+0xc`; when there is none it stores the number looked
/// at in `out_count`, clears the word at `+8`, sets `out_done` to 1 and returns. A block
/// `00dd7b40` answers 1 for is unlinked and pushed on the list at `+0x10`; one it answers 0 for
/// becomes the current block (`+8`). After `count` blocks it stores the number and sets
/// `out_done` to 1 when there is no current block.
/// Answers `out_done` (`EAX` at every `RET`; callers test the flag through it, 00d5ff00).
pub fn fn_00dd7c40(e: &mut Engine, this: u32, out_done: u32, count: i32, out_count: u32) -> u32 {
    let mut looked = 0i32;
    if count > 0 {
        loop {
            let current = e.mem.u32(this + 8);
            if current != 0 {
                let block = e.mem.u32(current);
                if block == 0 {
                    e.mem.set_i32(out_count, looked);
                    e.mem.set_u32(this + 8, 0);
                    e.mem.set_u8(out_done, 1);
                    return out_done;
                }
                if fn_00dd7b40(e, this, block) == 0 {
                    e.mem.set_u32(this + 8, block);
                } else {
                    let next = e.mem.u32(block);
                    let current = e.mem.u32(this + 8);
                    e.mem.set_u32(current, next);
                    let list = e.mem.u32(this + 0x10);
                    e.mem.set_u32(block, list);
                    e.mem.set_u32(this + 0x10, block);
                }
            } else {
                let block = e.mem.u32(this + 0xc);
                if block == 0 {
                    e.mem.set_i32(out_count, looked);
                    e.mem.set_u32(this + 8, 0);
                    e.mem.set_u8(out_done, 1);
                    return out_done;
                }
                if fn_00dd7b40(e, this, block) == 0 {
                    e.mem.set_u32(this + 8, block);
                } else {
                    let next = e.mem.u32(block);
                    let list = e.mem.u32(this + 0x10);
                    e.mem.set_u32(this + 0xc, next);
                    e.mem.set_u32(block, list);
                    e.mem.set_u32(this + 0x10, block);
                }
            }
            looked += 1;
            if looked >= count {
                break;
            }
        }
    }
    e.mem.set_i32(out_count, looked);
    let done = e.mem.u32(this + 8) == 0;
    e.mem.set_u8(out_done, u8::from(done));
    out_done
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd7b40, fn_00dd7b40(u32, u32) -> u32),
        entry!(0x00dd7c40, fn_00dd7c40(u32, u32, i32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A router whose stack cursor points at a 0x100-byte area, and a sort double that
    /// orders the pointer array by value and records the call.
    fn environment() -> (Engine, u32, u32, Rc<RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        let area = e.mem.alloc(0x100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, area);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sorts: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&sorts);
        e.register_double(QUICK_SORT, move |e, a| {
            seen.borrow_mut().push(a[1..].to_vec());
            let mut values: Vec<u32> = (0..=a[2]).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            values.sort_unstable();
            for (i, v) in values.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, v);
            }
            Ret::default()
        });
        (e, router, area, sorts)
    }

    /// The free list object `this` and a block of `count` elements of 0x10 bytes.
    fn free_list(e: &mut Engine, count: u32) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x40);
        let block = e.mem.alloc(0x10);
        let start = e.mem.alloc(0x10 * count.max(1) + 0x40);
        e.mem.set_u32(this + 4, 0x10);
        e.mem.set_u32(block + 8, start);
        e.mem.set_u32(block + 0xc, count);
        (this, block, start)
    }

    #[test]
    fn a_block_with_all_its_elements_free_is_taken_whole() {
        let (mut e, router, area, _) = environment();
        let (this, block, start) = free_list(&mut e, 2);
        // The list: both elements of the block, in reverse order.
        e.mem.set_u32(this, start + 0x10);
        e.mem.set_u32(start + 0x10, start);
        e.mem.set_u32(start, 0);
        assert_eq!(e.call(0x00dd_7b40, &args![this, block]).u32(), 1);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(router), area);
    }

    #[test]
    fn a_partly_free_block_gets_its_free_elements_back_sorted_in_front() {
        let (mut e, router, area, sorts) = environment();
        let (this, block, start) = free_list(&mut e, 4);
        let outside = e.mem.alloc(0x10);
        // The list: block element 2, an element elsewhere, block element 1.
        e.mem.set_u32(this, start + 0x20);
        e.mem.set_u32(start + 0x20, outside);
        e.mem.set_u32(outside, start + 0x10);
        e.mem.set_u32(start + 0x10, 0);
        assert_eq!(e.call(0x00dd_7b40, &args![this, block]).u32(), 0);
        // Sorted: element 1, element 2, then the rest of the list.
        assert_eq!(e.mem.u32(this), start + 0x10);
        assert_eq!(e.mem.u32(start + 0x10), start + 0x20);
        assert_eq!(e.mem.u32(start + 0x20), outside);
        assert_eq!(e.mem.u32(outside), 0);
        assert_eq!(*sorts.borrow(), vec![vec![0, 1, NODE_COMPARE]]);
        assert_eq!(e.mem.u32(router), area);
    }

    #[test]
    fn one_free_element_is_not_sorted_and_none_leaves_the_list_alone() {
        let (mut e, router, area, sorts) = environment();
        let (this, block, start) = free_list(&mut e, 4);
        let outside = e.mem.alloc(0x10);
        e.mem.set_u32(this, outside);
        e.mem.set_u32(outside, start + 0x30);
        e.mem.set_u32(start + 0x30, 0);
        assert_eq!(e.call(0x00dd_7b40, &args![this, block]).u32(), 0);
        assert!(sorts.borrow().is_empty());
        assert_eq!(e.mem.u32(this), start + 0x30);
        assert_eq!(e.mem.u32(start + 0x30), outside);
        assert_eq!(e.mem.u32(outside), 0);
        // No element of the block is free.
        let (this, block, _) = free_list(&mut e, 4);
        e.mem.set_u32(this, outside);
        e.mem.set_u32(outside, 0);
        assert_eq!(e.call(0x00dd_7b40, &args![this, block]).u32(), 0);
        assert_eq!(e.mem.u32(this), outside);
        assert_eq!(e.mem.u32(router), area);
    }

    #[test]
    fn the_marker_pair_counts_the_elements_it_covers() {
        let (mut e, _, _, _) = environment();
        let (this, block, start) = free_list(&mut e, 4);
        // Two free elements in the list and the marker covering two more: the block is whole.
        e.mem.set_u32(this, start);
        e.mem.set_u32(start, start + 0x10);
        e.mem.set_u32(start + 0x10, 0);
        e.mem.set_u32(this + 0x20, start + 0x20);
        e.mem.set_u32(this + 0x24, start + 0x40);
        assert_eq!(e.call(0x00dd_7b40, &args![this, block]).u32(), 1);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        // Covering only one more does not complete it and leaves the pair alone.
        let (this, block, start) = free_list(&mut e, 4);
        e.mem.set_u32(this, start);
        e.mem.set_u32(start, start + 0x10);
        e.mem.set_u32(start + 0x10, 0);
        e.mem.set_u32(this + 0x20, start + 0x20);
        e.mem.set_u32(this + 0x24, start + 0x30);
        assert_eq!(e.call(0x00dd_7b40, &args![this, block]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x20), start + 0x20);
    }

    fn walk(e: &mut Engine, this: u32, count: i32) -> (u8, i32) {
        let flag = e.mem.alloc(8);
        let looked = e.mem.alloc(8);
        e.mem.set_u8(flag, 9);
        e.call(0x00dd_7c40, &args![this, flag, count, looked]);
        (e.mem.u8(flag), e.mem.i32(looked))
    }

    #[test]
    fn blocks_that_are_whole_move_to_the_free_block_list_and_the_walk_ends_with_the_list() {
        let (mut e, _, _, _) = environment();
        let (this, block, start) = free_list(&mut e, 2);
        e.mem.set_u32(this, start);
        e.mem.set_u32(start, start + 0x10);
        e.mem.set_u32(start + 0x10, 0);
        e.mem.set_u32(this + 0xc, block);
        assert_eq!(walk(&mut e, this, 3), (1, 1));
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), block);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn the_walk_answers_its_flag_pointer() {
        let (mut e, _, _, _) = environment();
        let (this, _, _) = free_list(&mut e, 2);
        let flag = e.mem.alloc(8);
        let looked = e.mem.alloc(8);
        // EAX is `out_done` at every RET; the callers test the flag through it (00d5ff00).
        assert_eq!(
            e.call(0x00dd_7c40, &args![this, flag, 0i32, looked]).u32(),
            flag
        );
    }

    #[test]
    fn a_block_that_is_not_whole_becomes_the_current_one_and_the_next_is_after_it() {
        let (mut e, _, _, _) = environment();
        let (this, first, start) = free_list(&mut e, 4);
        let second = e.mem.alloc(0x10);
        let second_start = e.mem.alloc(0x40);
        e.mem.set_u32(second + 8, second_start);
        e.mem.set_u32(second + 0xc, 1);
        e.mem.set_u32(first, second);
        e.mem.set_u32(this + 0xc, first);
        // Block `first` has no free element (list empty): not whole, becomes current.
        assert_eq!(walk(&mut e, this, 1), (0, 1));
        assert_eq!(e.mem.u32(this + 8), first);
        // The next step looks at the block after the current one: `second` is whole when its
        // only element is free; the current block's link skips it.
        e.mem.set_u32(this, second_start);
        e.mem.set_u32(second_start, 0);
        e.mem.set_u32(second, 0);
        assert_eq!(walk(&mut e, this, 1), (0, 1));
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(this + 0x10), second);
        assert_eq!(e.mem.u32(second), 0);
        // No more blocks after the current one: done.
        assert_eq!(walk(&mut e, this, 2), (1, 0));
        assert_eq!(e.mem.u32(this + 8), 0);
        let _ = start;
    }

    #[test]
    fn a_zero_count_looks_at_nothing() {
        let (mut e, _, _, _) = environment();
        let (this, _, _) = free_list(&mut e, 1);
        assert_eq!(walk(&mut e, this, 0), (1, 0));
    }
}
