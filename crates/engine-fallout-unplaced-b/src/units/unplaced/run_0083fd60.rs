//! `(unplaced)/run_0083fd60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005ae380` (`thiscall`): the list's item count (the engine map's name is
/// `VATS::GetCount`).
const LIST_COUNT: u32 = 0x005a_e380;
/// `007a8790` (`thiscall`, `RET 4`): what the function does instead when
/// the stack block is null.
const SORT_WITHOUT_BUFFER: u32 = 0x007a_8790;
/// `007653f0` (`thiscall`, `RET 0xc`: the items, the word passed in, the
/// count): sorts the copied items.
const SORT_ITEMS: u32 = 0x0076_53f0;
/// `00470470` (`thiscall`): empties the list (nodes after the head freed).
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// `00401000` (`cdecl`): allocate.
const ALLOCATE: u32 = 0x0040_1000;
/// `00470440` (`thiscall`, `RET 4`, the item's address): a list node holding
/// that item.
const NODE_CONSTRUCT: u32 = 0x0047_0440;

// Translated from 0083fd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sorts a `BSSimpleList` (`thiscall`, `RET 4`:
/// a word passed to the sorter). The items (`005ae380` of them) are copied
/// into a stack block of `count * 4 + 4` bytes (`__alloca_probe_16`;
/// were it null, `007a8790` with the word would run instead), sorted by
/// `007653f0`, the list emptied (`00470470`) and rebuilt in the sorted
/// order: the first item in the head node, each other in a new 8-byte node
/// (`00401000`, `00470440`; a failed allocation leaves a null node, which
/// the next link then writes through) linked after the previous one.
// Translation incomplete: 0083fd60 - the SEH frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_0083fd60(e: &mut Engine, this: Ptr, compare: u32) {
    let count = e.call(LIST_COUNT, &args![this]).u32();
    let size = count.wrapping_mul(4).wrapping_add(4);
    e.with_stack(size.wrapping_add(15) & !15, |e, block| {
        let items = block.addr();
        if items == 0 {
            e.call(SORT_WITHOUT_BUFFER, &args![this, compare]);
            return;
        }
        let mut node = this.addr();
        let mut index: i32 = 0;
        while index < count as i32 {
            let item = e.mem.u32(node);
            e.mem
                .set_u32(items.wrapping_add((index as u32).wrapping_mul(4)), item);
            index = index.wrapping_add(1);
            node = e.mem.u32(node.wrapping_add(4));
        }
        e.call(SORT_ITEMS, &args![this, items, compare, count]);
        e.call(LIST_REMOVE_ALL, &args![this]);
        let mut node = this.addr();
        let mut previous = 0u32;
        let mut index: i32 = 0;
        while index < count as i32 {
            let item_address = items.wrapping_add((index as u32).wrapping_mul(4));
            if index > 0 {
                let block = e.call(ALLOCATE, &args![8u32]).u32();
                node = if block != 0 {
                    e.call(NODE_CONSTRUCT, &args![block, item_address]).u32()
                } else {
                    0
                };
            } else {
                let item = e.mem.u32(item_address);
                e.mem.set_u32(node, item);
                e.mem.set_u32(node.wrapping_add(4), 0);
            }
            if previous != 0 {
                e.mem.set_u32(previous.wrapping_add(4), node);
            }
            previous = node;
            index = index.wrapping_add(1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0083_fd60, fn_0083fd60(Ptr, u32))]
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    #[test]
    fn fn_0083fd60_sorts_and_rebuilds_the_list() {
        let mut e = Engine::new();
        e.register(LIST_COUNT, |e, a| {
            let mut n = 0u32;
            let mut node = a[0];
            while node != 0 {
                n += 1;
                node = e.mem.u32(node + 4);
            }
            n.into_ret()
        });
        e.register(SORT_ITEMS, |e, a| {
            let mut v: Vec<u32> = (0..a[3]).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            v.sort_unstable();
            for (i, x) in v.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *x);
            }
            Ret::default()
        });
        e.register(LIST_REMOVE_ALL, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_CONSTRUCT, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        let head = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        for (node, item, next) in [(head, 30u32, second), (second, 10, third), (third, 20, 0)] {
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
        }
        e.call_log = Some(vec![]);
        e.call(0x0083_fd60, &args![head, 0x1234u32]);
        let mut items = vec![];
        let mut node = head;
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        assert_eq!(items, [10, 20, 30]);
        let log = e.call_log.take().unwrap();
        let sort = log.iter().find(|c| c.0 == SORT_ITEMS).unwrap();
        assert_eq!((sort.1[0], sort.1[2], sort.1[3]), (head, 0x1234, 3));
        assert_eq!(log.iter().filter(|c| c.0 == ALLOCATE).count(), 2);
        assert!(log.iter().all(|c| c.0 != SORT_WITHOUT_BUFFER));
    }
}
