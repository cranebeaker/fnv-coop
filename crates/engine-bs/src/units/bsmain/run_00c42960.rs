//! `BSMain/run_00c42960` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The retired-node lists of the lock-free task containers (hazard-pointer
//! reclamation): a per-thread object whose first word is its owner keeps two
//! singly linked lists of retired nodes with their counts. A pass snapshots
//! the owner's hazard pointers, keeps the nodes some thread still points at
//! and releases the others. Also the thread entry point of the task
//! threads. No Xbox PDB names at these addresses.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton (`this` of its allocate and free).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (thiscall(size)).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager::Deallocate` (thiscall(block)).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `_memcpy` (cdecl(destination, source, size)).
const MEMCPY: u32 = 0x00ec_44d0;
/// The owner's release of a retired node of the first list (`00c41fe0`,
/// thiscall(node)).
const OWNER_RELEASE_NODE: u32 = 0x00c4_1fe0;
/// The release of a retired node's object (`0044dd60`, thiscall).
const NODE_OBJECT_RELEASE: u32 = 0x0044_dd60;
/// `DoThreadSetup` (engine map, cdecl(block)).
const DO_THREAD_SETUP: u32 = 0x00c8_0130;
/// The thread's TLS word that selects the allocator context; the passes set
/// it to 6 around their work and restore it.
const TLS_ALLOCATION_CONTEXT: u32 = 0x2b4;
/// The context they set.
const TASK_CONTEXT: u32 = 6;

/// Owner fields (offsets from the PC code): `+0x14` points at the thread
/// count, `+0x04` is the first list's hazard array (three words per thread),
/// `+0x10` the second's (two words per thread).
const OWNER_THREAD_COUNT: u32 = 0x14;
const OWNER_HAZARDS_A: u32 = 0x04;
const OWNER_HAZARDS_B: u32 = 0x10;

layout! {
    /// The per-thread retired-node lists (offsets from the PC code).
    pub struct RetiredLists: 0x24 {
        /// The owner container.
        0x00 owner: u32,
        /// The second list's node count.
        0x0C count_b: u32,
        /// The second list's head (nodes linked through `+4`).
        0x10 head_b: u32,
        /// The first list's head (nodes linked through `+8`).
        0x1C head_a: u32,
        /// The first list's node count.
        0x20 count_a: u32,
    }
}

/// Allocates `words * 4` bytes (all ones when the product overflows) and
/// copies `words << 2` bytes of the hazard array at `hazards` into it.
fn snapshot_hazards(e: &mut Engine, hazards: u32, words: u32) -> u32 {
    let bytes = words as u64 * 4;
    let size = (bytes as u32) | if bytes > u32::MAX as u64 { u32::MAX } else { 0 };
    let copy = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.call(MEMCPY, &args![copy, hazards, words << 2]);
    copy
}

/// Whether `node` is one of the `words` words of the snapshot.
fn is_hazard(e: &Engine, snapshot: u32, words: u32, node: u32) -> bool {
    let mut index = 0u32;
    while index < words {
        if node == e.mem.u32(snapshot.wrapping_add(index.wrapping_mul(4))) {
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00c42960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c42960 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// One pass over the first retired list: with the allocation context set to
/// 6, snapshots the owner's `3 * threads` hazard words, then takes each node
/// off the list (`+8` links it): a node still in the snapshot goes onto the
/// kept list (pushed at its front, counted), any other gets its link cleared
/// and is released by the owner (`00c41fe0`). Frees the snapshot and stores
/// the kept list and its count; restores the context.
pub fn fn_00c42960(e: &mut Engine, this: Ptr<RetiredLists>) {
    let context = e.tls() + TLS_ALLOCATION_CONTEXT;
    let previous = e.mem.u32(context);
    e.mem.set_u32(context, TASK_CONTEXT);
    let owner = e.get(this, RetiredLists::owner);
    let threads = e.mem.u32(e.mem.u32(owner.wrapping_add(OWNER_THREAD_COUNT)));
    let words = threads.wrapping_mul(3);
    let hazards = e.mem.u32(owner.wrapping_add(OWNER_HAZARDS_A));
    let snapshot = snapshot_hazards(e, hazards, words);
    let mut kept = 0u32;
    let mut kept_count = 0u32;
    while e.get(this, RetiredLists::head_a) != 0 {
        let node = e.get(this, RetiredLists::head_a);
        let next = e.mem.u32(node.wrapping_add(8));
        e.set(this, RetiredLists::head_a, next);
        if is_hazard(e, snapshot, words, node) {
            e.mem.set_u32(node.wrapping_add(8), kept);
            kept = node;
            kept_count = kept_count.wrapping_add(1);
        } else {
            e.mem.set_u32(node.wrapping_add(8), 0);
            let owner = e.get(this, RetiredLists::owner);
            e.call(OWNER_RELEASE_NODE, &args![owner, node]);
        }
    }
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, snapshot]);
    e.set(this, RetiredLists::head_a, kept);
    e.set(this, RetiredLists::count_a, kept_count);
    let context = e.tls() + TLS_ALLOCATION_CONTEXT;
    e.mem.set_u32(context, previous);
}

// Translated from 00c42b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Repeats the first list's pass (`00c42960`) until its count is 0.
pub fn fn_00c42b40(e: &mut Engine, this: Ptr<RetiredLists>) {
    while e.get(this, RetiredLists::count_a) != 0 {
        fn_00c42960(e, this);
    }
}

// Translated from 00c42b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Repeats the second list's pass (`00c42b80`) until its count is 0.
pub fn fn_00c42b60(e: &mut Engine, this: Ptr<RetiredLists>) {
    while e.get(this, RetiredLists::count_b) != 0 {
        fn_00c42b80(e, this);
    }
}

// Translated from 00c42b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c42b80 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// One pass over the second retired list: as `00c42960` with the owner's
/// `2 * threads` hazard words at `+0x10` and nodes linked through `+4`; a
/// node that is not kept gets its link cleared, then (when not null) the
/// object its link word holds is released (`0044dd60`, when that word,
/// read again after the clear, is not 0) and the node is freed.
pub fn fn_00c42b80(e: &mut Engine, this: Ptr<RetiredLists>) {
    let context = e.tls() + TLS_ALLOCATION_CONTEXT;
    let previous = e.mem.u32(context);
    e.mem.set_u32(context, TASK_CONTEXT);
    let owner = e.get(this, RetiredLists::owner);
    let threads = e.mem.u32(e.mem.u32(owner.wrapping_add(OWNER_THREAD_COUNT)));
    let words = threads << 1;
    let hazards = e.mem.u32(owner.wrapping_add(OWNER_HAZARDS_B));
    let snapshot = snapshot_hazards(e, hazards, words);
    let mut kept = 0u32;
    let mut kept_count = 0u32;
    while e.get(this, RetiredLists::head_b) != 0 {
        let node = e.get(this, RetiredLists::head_b);
        let next = e.mem.u32(node.wrapping_add(4));
        e.set(this, RetiredLists::head_b, next);
        if is_hazard(e, snapshot, words, node) {
            e.mem.set_u32(node.wrapping_add(4), kept);
            kept = node;
            kept_count = kept_count.wrapping_add(1);
        } else {
            e.mem.set_u32(node.wrapping_add(4), 0);
            if node != 0 {
                // The word was just cleared; the code reads it again.
                let object = e.mem.u32(node.wrapping_add(4));
                if object != 0 {
                    e.call(NODE_OBJECT_RELEASE, &args![object]);
                }
                // `MOV EDX,1; AND EDX,1; JZ`: the free is never skipped.
                e.call(MEMORY_FREE, &args![MEMORY_MANAGER, node]);
            }
        }
    }
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, snapshot]);
    e.set(this, RetiredLists::head_b, kept);
    e.set(this, RetiredLists::count_b, kept_count);
    let context = e.tls() + TLS_ALLOCATION_CONTEXT;
    e.mem.set_u32(context, previous);
}

// Translated from 00c42da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A task thread's entry point (stdcall(thread)): `DoThreadSetup` on
/// `thread + 0x24`, then the thread's virtual `+4`; returns 0.
pub fn fn_00c42da0(e: &mut Engine, thread: Ptr) -> u32 {
    e.call(DO_THREAD_SETUP, &args![thread.addr().wrapping_add(0x24)]);
    e.vcall(thread.addr(), 4, &args![]);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_2960, fn_00c42960(Ptr<RetiredLists>)),
        entry!(0x00c4_2b40, fn_00c42b40(Ptr<RetiredLists>)),
        entry!(0x00c4_2b60, fn_00c42b60(Ptr<RetiredLists>)),
        entry!(0x00c4_2b80, fn_00c42b80(Ptr<RetiredLists>)),
        entry!(0x00c4_2da0, fn_00c42da0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(OWNER_RELEASE_NODE, |_, _| Ret::default());
        e.register(NODE_OBJECT_RELEASE, |_, _| Ret::default());
        e
    }

    /// An owner with `threads` threads and the given hazard words in both
    /// arrays (padded with zeros).
    fn owner(e: &mut Engine, threads: u32, hazards: &[u32]) -> u32 {
        let owner = e.mem.alloc(0x20);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, threads);
        e.mem.set_u32(owner + OWNER_THREAD_COUNT, count);
        for offset in [OWNER_HAZARDS_A, OWNER_HAZARDS_B] {
            let array = e.mem.alloc(12 * threads);
            for (i, word) in hazards.iter().enumerate() {
                e.mem.set_u32(array + 4 * i as u32, *word);
            }
            e.mem.set_u32(owner + offset, array);
        }
        owner
    }

    /// Nodes `0x10` bytes each, linked through `link` in the given order.
    fn list(e: &mut Engine, count: usize, link: u32) -> Vec<u32> {
        let nodes: Vec<u32> = (0..count).map(|_| e.mem.alloc(0x10)).collect();
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + link, pair[1]);
        }
        nodes
    }

    #[test]
    fn first_pass_keeps_hazards_and_releases_the_rest() {
        let mut e = engine();
        let nodes = list(&mut e, 3, 8);
        let owner = owner(&mut e, 1, &[nodes[1]]);
        let this: Ptr<RetiredLists> = e.new_object();
        e.set(this, RetiredLists::owner, owner);
        e.set(this, RetiredLists::head_a, nodes[0]);
        e.set(this, RetiredLists::count_a, 3);
        let context = e.tls() + TLS_ALLOCATION_CONTEXT;
        e.mem.set_u32(context, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c4_2960, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 12]]);
        assert_eq!(
            calls(&log, OWNER_RELEASE_NODE),
            vec![vec![owner, nodes[0]], vec![owner, nodes[2]]]
        );
        assert_eq!(e.get(this, RetiredLists::head_a), nodes[1]);
        assert_eq!(e.get(this, RetiredLists::count_a), 1);
        assert_eq!(e.mem.u32(nodes[1] + 8), 0);
        assert_eq!(e.mem.u32(nodes[0] + 8), 0);
        assert_eq!(e.mem.u32(context), 2);
        // The snapshot is freed.
        let snapshot = calls(&log, MEMCPY)[0][0];
        assert_eq!(
            calls(&log, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, snapshot]]
        );
    }

    #[test]
    fn first_list_repeats_until_empty() {
        let mut e = engine();
        let nodes = list(&mut e, 2, 8);
        let owner = owner(&mut e, 1, &[]);
        let this: Ptr<RetiredLists> = e.new_object();
        e.set(this, RetiredLists::owner, owner);
        e.set(this, RetiredLists::head_a, nodes[0]);
        e.set(this, RetiredLists::count_a, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c4_2b40, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, OWNER_RELEASE_NODE).len(), 2);
        assert_eq!(e.get(this, RetiredLists::count_a), 0);
        assert_eq!(e.get(this, RetiredLists::head_a), 0);
        // A count of 0 does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c4_2b40, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn second_pass_frees_unreferenced_nodes() {
        let mut e = engine();
        let nodes = list(&mut e, 3, 4);
        let owner = owner(&mut e, 1, &[0, nodes[2]]);
        let this: Ptr<RetiredLists> = e.new_object();
        e.set(this, RetiredLists::owner, owner);
        e.set(this, RetiredLists::head_b, nodes[0]);
        e.set(this, RetiredLists::count_b, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c4_2b80, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 8]]);
        let snapshot = calls(&log, MEMCPY)[0][0];
        assert_eq!(
            calls(&log, MEMORY_FREE),
            vec![
                vec![MEMORY_MANAGER, nodes[0]],
                vec![MEMORY_MANAGER, nodes[1]],
                vec![MEMORY_MANAGER, snapshot]
            ]
        );
        // The link word is read after it was cleared: never released.
        assert!(calls(&log, NODE_OBJECT_RELEASE).is_empty());
        assert_eq!(e.get(this, RetiredLists::head_b), nodes[2]);
        assert_eq!(e.get(this, RetiredLists::count_b), 1);
    }

    #[test]
    fn second_list_repeats_until_empty() {
        let mut e = engine();
        let nodes = list(&mut e, 1, 4);
        let owner = owner(&mut e, 2, &[]);
        let this: Ptr<RetiredLists> = e.new_object();
        e.set(this, RetiredLists::owner, owner);
        e.set(this, RetiredLists::head_b, nodes[0]);
        e.set(this, RetiredLists::count_b, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c4_2b60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 16]]);
        assert_eq!(e.get(this, RetiredLists::count_b), 0);
    }

    #[test]
    fn thread_entry_sets_up_and_runs() {
        let mut e = engine();
        e.register(DO_THREAD_SETUP, |_, _| Ret::default());
        e.register(0x0012_3450, |_, _| 5u32.into_ret());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0012_3450);
        let thread = e.mem.alloc(0x40);
        e.mem.set_u32(thread, vtable);
        e.call_log = Some(vec![]);
        let back = e.call(0x00c4_2da0, &args![thread]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0);
        assert_eq!(calls(&log, DO_THREAD_SETUP), vec![vec![thread + 0x24]]);
        assert_eq!(calls(&log, 0x0012_3450), vec![vec![thread]]);
    }
}
