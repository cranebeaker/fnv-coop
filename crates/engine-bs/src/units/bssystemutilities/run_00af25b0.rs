//! `BSSystemUtilities/run_00af25b0` (the engine map's run of unnamed functions from
//! `00af25b0`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager's allocation (`00aa3e40`, `thiscall` on `011f6238`, one word: the
/// size).
const ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;

// Translated from 00af25b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The add at the tail of a simple list (no label; `thiscall`, one word: the address of the
/// item word, `RET 4`). The list is a chain of nodes of two words (the item, the next node),
/// the first node inside the list object. A null item adds nothing. Otherwise the last node is
/// found (following +4 while it is not null); when its item word is empty the item goes there;
/// otherwise a node of 8 bytes is allocated (`00aa3e40` on the memory manager), given the
/// item and a null next, and linked after the last one (a failed allocation links null).
pub fn simple_list_add_tail(e: &mut Engine, this: Ptr, item: u32) {
    if e.mem.u32(item) == 0 {
        return;
    }
    let mut node = this.addr();
    while e.mem.u32(node.wrapping_add(4)) != 0 {
        node = e.mem.u32(node.wrapping_add(4));
    }
    if e.mem.u32(node) == 0 {
        let value = e.mem.u32(item);
        e.mem.set_u32(node, value);
        return;
    }
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
    let link = if block != 0 {
        let value = e.mem.u32(item);
        e.mem.set_u32(block, value);
        e.mem.set_u32(block.wrapping_add(4), 0);
        block
    } else {
        0
    };
    e.mem.set_u32(node.wrapping_add(4), link);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_25b0, simple_list_add_tail(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: u32 = 0x0800_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(LIST, 0x100);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e
    }

    fn items(e: &Engine) -> Vec<u32> {
        let mut out = Vec::new();
        let mut node = LIST;
        while node != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn add_tail_fills_the_first_node_then_links_new_ones() {
        let mut e = engine();
        let item = e.mem.alloc(4);
        for value in [0u32, 7, 8, 9] {
            e.mem.set_u32(item, value);
            simple_list_add_tail(&mut e, Ptr::new(LIST), item);
        }
        assert_eq!(items(&e), vec![7, 8, 9]);
        // Through the entry.
        e.mem.set_u32(item, 10);
        e.call(0x00af_25b0, &args![LIST, item]);
        assert_eq!(items(&e), vec![7, 8, 9, 10]);
    }

    #[test]
    fn add_tail_fills_an_empty_last_node_and_links_null_without_memory() {
        let mut e = engine();
        let item = e.mem.alloc(4);
        // A second node with an empty item: filled in place.
        let second = e.mem.alloc(8);
        e.mem.set_u32(LIST, 5);
        e.mem.set_u32(LIST + 4, second);
        e.mem.set_u32(item, 6);
        simple_list_add_tail(&mut e, Ptr::new(LIST), item);
        assert_eq!(items(&e), vec![5, 6]);
        // A failed allocation links null.
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.mem.set_u32(item, 7);
        simple_list_add_tail(&mut e, Ptr::new(LIST), item);
        assert_eq!(items(&e), vec![5, 6]);
    }
}
