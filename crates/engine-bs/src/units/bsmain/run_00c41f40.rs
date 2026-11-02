//! `BSMain/run_00c41f40` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Fields of the object these two run on (offsets as the code uses them;
/// the class is not named in the code): the owner +0x00 (whose word at
/// +0x10 is the free-list limit), the free-list head +0x1c and the free
/// count +0x20. A node keeps a counted pointer at +0x08 (its count at +8
/// of the object it points to) and, for `00c41fe0`, two words at +0 and +4.
const OWNER: u32 = 0x00;
const OWNER_LIMIT: u32 = 0x10;
const FREE_HEAD: u32 = 0x1c;
const FREE_COUNT: u32 = 0x20;
const NODE_POINTER: u32 = 0x08;

/// `0044dd60` (unit `run_0044d8c0` in `engine-fallout`, `thiscall`): the
/// release of the object a node points to.
const RELEASE_POINTED: u32 = 0x0044_dd60;
/// `00c42960` (`thiscall`): run when the free count reaches the limit.
const FREE_LIST_FULL: u32 = 0x00c4_2960;
/// The memory manager singleton and its `Deallocate` (`00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The owner's virtual slot `0x28` (`00c41fe0`, with the node's two words).
const SLOT_NODE_REMOVED: u32 = 0x28;

// Translated from 00c41f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: puts `node` on the free list. A pointer the
/// node still holds is released (`0044dd60`) and cleared (the inline
/// smart-pointer assignment of null, whose count of the new, null, pointer
/// never runs); the node is linked in front of the free list, the free
/// count goes up and, when it reaches the owner's limit (+0x10), `00c42960`
/// runs.
pub fn fn_00c41f40(e: &mut Engine, this: Ptr, node: Ptr) {
    let this = this.addr();
    let pointer = node.addr().wrapping_add(NODE_POINTER);
    if e.mem.u32(pointer) != 0 {
        let held = e.mem.u32(pointer);
        if held != 0 {
            e.call(RELEASE_POINTED, &args![held]);
        }
        e.mem.set_u32(pointer, 0);
        let held = e.mem.u32(pointer);
        if held != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(8)]);
        }
    }
    let head = e.mem.u32(this.wrapping_add(FREE_HEAD));
    e.mem.set_u32(pointer, head);
    e.mem.set_u32(this.wrapping_add(FREE_HEAD), node.addr());
    let count = e.mem.u32(this.wrapping_add(FREE_COUNT)).wrapping_add(1);
    e.mem.set_u32(this.wrapping_add(FREE_COUNT), count);
    let owner = e.mem.u32(this.wrapping_add(OWNER));
    let limit = e.mem.u32(owner.wrapping_add(OWNER_LIMIT));
    if e.mem.u32(this.wrapping_add(FREE_COUNT)) == limit {
        e.call(FREE_LIST_FULL, &args![this]);
    }
}

// Translated from 00c41fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: frees `node`. The object's slot `0x28` runs
/// with the node's two words; then, for a non-null node, the pointer it
/// holds is released (`0044dd60`) and the node goes back to the memory
/// manager (the compiler's constant "delete" flag is always set).
pub fn fn_00c41fe0(e: &mut Engine, this: Ptr, node: Ptr) {
    let node = node.addr();
    let first = e.mem.u32(node);
    let second = e.mem.u32(node.wrapping_add(4));
    e.vcall(this.addr(), SLOT_NODE_REMOVED, &args![first, second]);
    if node == 0 {
        return;
    }
    let held = e.mem.u32(node.wrapping_add(NODE_POINTER));
    if held != 0 {
        e.call(RELEASE_POINTED, &args![held]);
    }
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, node]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_1f40, fn_00c41f40(Ptr, Ptr)),
        entry!(0x00c4_1fe0, fn_00c41fe0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const NODE_REMOVED: u32 = 0x00ee_0100;

    fn logged(e: &mut Engine, log: &Log, address: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    #[test]
    fn fn_00c41f40_links_the_node_and_reports_a_full_list() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, RELEASE_POINTED);
        logged(&mut e, &log, FREE_LIST_FULL);
        let owner = e.mem.alloc(0x14);
        e.mem.set_u32(owner + OWNER_LIMIT, 2);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + OWNER, owner);
        e.mem.set_u32(this + FREE_HEAD, 0x7000);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node + NODE_POINTER, 0x6600);
        e.call(0x00c4_1f40, &args![this, node]);
        assert_eq!(*log.borrow(), [(RELEASE_POINTED, vec![0x6600])]);
        assert_eq!(e.mem.u32(node + NODE_POINTER), 0x7000);
        assert_eq!(e.mem.u32(this + FREE_HEAD), node);
        assert_eq!(e.mem.u32(this + FREE_COUNT), 1);
        let second = e.mem.alloc(0x10);
        e.call(0x00c4_1f40, &args![this, second]);
        assert_eq!(e.mem.u32(second + NODE_POINTER), node);
        assert_eq!(log.borrow()[1], (FREE_LIST_FULL, vec![this]));
    }

    #[test]
    fn fn_00c41fe0_reports_releases_and_frees_the_node() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, RELEASE_POINTED);
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE);
        logged(&mut e, &log, NODE_REMOVED);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + SLOT_NODE_REMOVED, NODE_REMOVED);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this, vtable);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, 11);
        e.mem.set_u32(node + 4, 22);
        e.mem.set_u32(node + NODE_POINTER, 0x6600);
        e.call(0x00c4_1fe0, &args![this, node]);
        assert_eq!(
            *log.borrow(),
            [
                (NODE_REMOVED, vec![this, 11, 22]),
                (RELEASE_POINTED, vec![0x6600]),
                (MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, node]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(node + NODE_POINTER, 0);
        e.call(0x00c4_1fe0, &args![this, node]);
        assert_eq!(log.borrow().len(), 2);
    }
}
