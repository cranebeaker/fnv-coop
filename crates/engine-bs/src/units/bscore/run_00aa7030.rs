//! `BSCore/run_00aa7030` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Upkeep of the memory pools in the table at `011f67b8` (256 entries, the
//! table `MemoryPool::FreeUnusedPages` is run over): sorting a pool's free
//! list by address, walking it under the pool's spin lock, and the
//! `ZeroOverheadHeap` constructor. The pool fields are read at their PC
//! offsets (free list head +0x08, spin lock +0x20 with owner +0x20 and
//! count +0x24, a size at +0x40, a count at +0x54); the PC `MemoryPool`
//! differs from the Xbox PDB's, so they are described, not named.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory pool table (256 pointers, null where there is none).
const POOLS: u32 = 0x011f_67b8;
const POOL_COUNT: u32 = 0x100;

/// `00aa7030`'s function-local static critical section and its guard word.
const FREE_PAGES_SECTION: u32 = 0x011f_6bb8;
const FREE_PAGES_GUARD: u32 = 0x011f_6bd0;
/// The `atexit` destructor `00aa7030` registers for its section.
const FREE_PAGES_SECTION_EXIT: u32 = 0x00fd_b2a0;
/// `00aa7370`'s function-local static critical section and its guard word.
const CHECK_SECTION: u32 = 0x011f_6bd4;
const CHECK_GUARD: u32 = 0x011f_6bec;
/// The `atexit` destructor `00aa7370` registers for its section.
const CHECK_SECTION_EXIT: u32 = 0x00fd_b2b0;

/// `MemoryPool::FreeUnusedPages` (engine map).
const MEMORY_POOL_FREE_UNUSED_PAGES: u32 = 0x00aa_6f90;
/// The spin-lock acquire (`this` = lock, one word: a name string).
const SPIN_LOCK_ENTER: u32 = 0x0040_fbf0;
/// `_atexit`.
const ATEXIT: u32 = 0x00ec_658f;
/// `"SortFreeBlocks"`.
const SORT_FREE_BLOCKS_NAME: u32 = 0x010a_2850;
/// `"CheckFreeBlocks"`.
const CHECK_FREE_BLOCKS_NAME: u32 = 0x010a_2860;

/// Import slot of `InitializeCriticalSection`.
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// Import slot of `EnterCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// Import slot of `LeaveCriticalSection`.
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// The `MemoryManager` singleton and the allocation call the
/// `ZeroOverheadHeap` constructor makes on it (`this`, size, 1; `_malloc`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_MALLOC: u32 = 0x00aa_4290;
/// `IMemoryHeap`'s vtable, then `ZeroOverheadHeap`'s (RTTI locators).
const I_MEMORY_HEAP_VTABLE: u32 = 0x010a_28c8;
const ZERO_OVERHEAD_HEAP_VTABLE: u32 = 0x010a_2884;

/// Pool field offsets (PC).
const POOL_FREE_HEAD: u32 = 0x08;
const POOL_LOCK: u32 = 0x20;
const POOL_CHECK_SIZE: u32 = 0x40;
const POOL_FREE_COUNT: u32 = 0x54;

layout! {
    /// `ZeroOverheadHeap` (Xbox PDB), as far as the PC constructor writes it.
    pub struct ZeroOverheadHeap: 0x28 {
        /// The vtable.
        0x00 vtable: u32,
        /// `iSize` (Xbox PDB).
        0x04 iSize: u32,
        /// `pName` (Xbox PDB).
        0x08 pName: u32,
        /// `pHeap` (Xbox PDB).
        0x0C pHeap: u32,
        /// `pCurrentFree` (Xbox PDB).
        0x10 pCurrentFree: u32,
        /// `iAllocations` (Xbox PDB).
        0x14 iAllocations: u32,
        /// The spin lock's owner word (Xbox `lock`, at +0x18 there).
        0x20 lockOwner: u32,
        /// The spin lock's count word.
        0x24 lockCount: u32,
    }
}

fn pool(e: &Engine, index: u32) -> u32 {
    e.mem.u32(POOLS.wrapping_add(index.wrapping_mul(4)))
}

/// Free-list node: +0x00 previous, +0x04 next.
fn next(e: &Engine, node: u32) -> u32 {
    e.mem.u32(node.wrapping_add(4))
}

/// The inlined spin-lock release: decrements the count and clears the
/// owner when it reaches zero.
fn spin_lock_leave(e: &mut Engine, lock: u32) {
    let count = e.mem.u32(lock.wrapping_add(4)).wrapping_sub(1);
    e.mem.set_u32(lock.wrapping_add(4), count);
    if e.mem.u32(lock.wrapping_add(4)) == 0 {
        e.mem.set_u32(lock, 0);
    }
}

/// Runs a function-local static critical section's one-time
/// initialization (`InitializeCriticalSection`, `atexit` destructor).
fn init_static_section(e: &mut Engine, guard: u32, section: u32, exit: u32) {
    if e.global::<u32>(guard) & 1 == 0 {
        let flags: u32 = e.global(guard);
        e.set_global(guard, flags | 1);
        e.call(INITIALIZE_CRITICAL_SECTION, &args![section]);
        e.call(ATEXIT, &args![exit]);
    }
}

// Translated from 00aa7030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under a function-local static critical section (initialized on the
/// first call), runs `MemoryPool::FreeUnusedPages` on every pool of the
/// table.
pub fn fn_00aa7030(e: &mut Engine) {
    init_static_section(
        e,
        FREE_PAGES_GUARD,
        FREE_PAGES_SECTION,
        FREE_PAGES_SECTION_EXIT,
    );
    e.call(ENTER_CRITICAL_SECTION, &args![FREE_PAGES_SECTION]);
    for index in 0..POOL_COUNT {
        if pool(e, index) != 0 {
            let p = pool(e, index);
            e.call(MEMORY_POOL_FREE_UNUSED_PAGES, &args![p]);
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![FREE_PAGES_SECTION]);
}

// Translated from 00aa70c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts a doubly linked free list (node +0x00 previous, +0x04 next) by
/// node address, ascending, with a bottom-up merge sort; returns the new
/// head. Equal runs keep their order (the left node is taken unless the
/// right one is lower). `this` is never read.
pub fn fn_00aa70c0(e: &mut Engine, _this: Ptr, list: u32) -> u32 {
    let mut head = list;
    let mut merges: i32 = 0;
    if head == 0 {
        return 0;
    }
    let mut run: u32 = 1;
    while merges != 1 {
        let mut left = head;
        head = 0;
        let mut tail: u32 = 0;
        merges = 0;
        while left != 0 {
            merges = merges.wrapping_add(1);
            let mut right = left;
            let mut left_size: i32 = 0;
            let mut i: u32 = 0;
            while i < run {
                left_size = left_size.wrapping_add(1);
                right = next(e, right);
                if right == 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
            let mut right_size = run as i32;
            while left_size > 0 || (right_size > 0 && right != 0) {
                // The right run's node when the left run is used up, or
                // when the right run has one and it is the lower address
                // (unsigned); otherwise the left run's node.
                let take_right = left_size == 0 || (right_size != 0 && right != 0 && right < left);
                let node = if take_right {
                    let node = right;
                    right = next(e, right);
                    right_size = right_size.wrapping_sub(1);
                    node
                } else {
                    let node = left;
                    left = next(e, left);
                    left_size = left_size.wrapping_sub(1);
                    node
                };
                if tail == 0 {
                    head = node;
                } else {
                    e.mem.set_u32(tail.wrapping_add(4), node);
                }
                e.mem.set_u32(node, tail);
                tail = node;
            }
            left = right;
        }
        e.mem.set_u32(tail.wrapping_add(4), 0);
        run <<= 1;
    }
    head
}

// Translated from 00aa7260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts the pool's free list (`00aa70c0`) when the count at +0x54 is
/// above 1.
pub fn fn_00aa7260(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr().wrapping_add(POOL_FREE_COUNT)) > 1 {
        let list = e.mem.u32(this.addr().wrapping_add(POOL_FREE_HEAD));
        let sorted = fn_00aa70c0(e, this, list);
        e.mem
            .set_u32(this.addr().wrapping_add(POOL_FREE_HEAD), sorted);
    }
}

// Translated from 00aa7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// For the pool at `index` of the table, if any: takes its spin lock
/// (`"SortFreeBlocks"`), sorts its free list (`00aa7260`) and releases the
/// lock.
pub fn fn_00aa7290(e: &mut Engine, index: u32) {
    if pool(e, index) == 0 {
        return;
    }
    let lock = pool(e, index).wrapping_add(POOL_LOCK);
    e.call(SPIN_LOCK_ENTER, &args![lock, SORT_FREE_BLOCKS_NAME]);
    let p = pool(e, index);
    fn_00aa7260(e, Ptr::new(p));
    let lock = pool(e, index).wrapping_add(POOL_LOCK);
    spin_lock_leave(e, lock);
}

// Translated from 00aa7300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the pool's spin lock (`"CheckFreeBlocks"`), walks the free list to
/// its end when the size at +0x40 is above 8; the walk checks nothing (the
/// check was compiled out) but still reads every node's next pointer.
pub fn fn_00aa7300(e: &mut Engine, this: Ptr) {
    let lock = this.addr().wrapping_add(POOL_LOCK);
    e.call(SPIN_LOCK_ENTER, &args![lock, CHECK_FREE_BLOCKS_NAME]);
    if e.mem.u32(this.addr().wrapping_add(POOL_CHECK_SIZE)) > 8 {
        let mut node = e.mem.u32(this.addr().wrapping_add(POOL_FREE_HEAD));
        while node != 0 {
            node = next(e, node);
        }
    }
    spin_lock_leave(e, lock);
}

// Translated from 00aa7370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under a function-local static critical section (initialized on the
/// first call), runs `00aa7300` on every pool of the table.
pub fn fn_00aa7370(e: &mut Engine) {
    init_static_section(e, CHECK_GUARD, CHECK_SECTION, CHECK_SECTION_EXIT);
    e.call(ENTER_CRITICAL_SECTION, &args![CHECK_SECTION]);
    for index in 0..POOL_COUNT {
        let p = pool(e, index);
        if p != 0 {
            fn_00aa7300(e, Ptr::new(p));
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![CHECK_SECTION]);
}

// Translated from 00aa7400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa7400 - the FS:[0] exception-unwinding frame is not translated
/// The `ZeroOverheadHeap` constructor (the vtables it stores; no name in
/// the engine map): stores size and name, clears the allocation count and
/// the spin lock, takes `size` bytes from the `MemoryManager` (`00aa4290`,
/// `_malloc`) as the heap and makes it the current free pointer.
pub fn fn_00aa7400(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    size: u32,
    name: u32,
) -> Ptr<ZeroOverheadHeap> {
    e.set(this, ZeroOverheadHeap::vtable, I_MEMORY_HEAP_VTABLE);
    e.set(this, ZeroOverheadHeap::vtable, ZERO_OVERHEAD_HEAP_VTABLE);
    e.set(this, ZeroOverheadHeap::iSize, size);
    e.set(this, ZeroOverheadHeap::pName, name);
    e.set(this, ZeroOverheadHeap::iAllocations, 0);
    e.set(this, ZeroOverheadHeap::lockOwner, 0);
    e.set(this, ZeroOverheadHeap::lockCount, 0);
    let heap = e
        .call(MEMORY_MANAGER_MALLOC, &args![MEMORY_MANAGER, size, 1u32])
        .u32();
    e.set(this, ZeroOverheadHeap::pHeap, heap);
    let heap = e.get(this, ZeroOverheadHeap::pHeap);
    e.set(this, ZeroOverheadHeap::pCurrentFree, heap);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa7030, fn_00aa7030()),
        entry!(0x00aa70c0, fn_00aa70c0(Ptr, u32) -> u32),
        entry!(0x00aa7260, fn_00aa7260(Ptr)),
        entry!(0x00aa7290, fn_00aa7290(u32)),
        entry!(0x00aa7300, fn_00aa7300(Ptr)),
        entry!(0x00aa7370, fn_00aa7370()),
        entry!(
            0x00aa7400,
            fn_00aa7400(Ptr<ZeroOverheadHeap>, u32, u32) -> Ptr<ZeroOverheadHeap>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMPORTS: [u32; 3] = [
        INITIALIZE_CRITICAL_SECTION,
        ENTER_CRITICAL_SECTION,
        LEAVE_CRITICAL_SECTION,
    ];

    fn pool_engine() -> Engine {
        let mut e = Engine::new();
        for slot in IMPORTS {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(ATEXIT, |_, _| Ret::default());
        e.register(MEMORY_POOL_FREE_UNUSED_PAGES, |_, _| Ret::default());
        // The double takes the lock like the game: owner 1, count + 1.
        e.register(SPIN_LOCK_ENTER, |e, a| {
            e.mem.set_u32(a[0], 1);
            let count = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
        e.map(0x011f_6000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A list of nodes at the given addresses, in that order; returns the head.
    fn list(e: &mut Engine, nodes: &[u32]) -> u32 {
        e.map(0x0500_0000, 0x1000);
        let mut prev = 0;
        for (i, &n) in nodes.iter().enumerate() {
            e.mem.set_u32(n, prev);
            e.mem.set_u32(n + 4, nodes.get(i + 1).copied().unwrap_or(0));
            prev = n;
        }
        nodes.first().copied().unwrap_or(0)
    }

    /// Follows the list from `head`, checking every back link.
    fn walk(e: &Engine, head: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut prev = 0;
        let mut node = head;
        while node != 0 {
            assert_eq!(e.mem.u32(node), prev);
            out.push(node);
            prev = node;
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn free_unused_pages_of_every_pool_under_a_static_section() {
        let mut e = pool_engine();
        e.set_global(POOLS + 4 * 3, 0x0600_0000u32);
        e.set_global(POOLS + 4 * 255, 0x0600_0100u32);
        e.call(0x00aa7030, &[]);
        assert_eq!(e.global::<u32>(FREE_PAGES_GUARD) & 1, 1);
        assert_eq!(
            calls(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![FREE_PAGES_SECTION]]
        );
        assert_eq!(calls(&e, ATEXIT), vec![vec![FREE_PAGES_SECTION_EXIT]]);
        assert_eq!(
            calls(&e, MEMORY_POOL_FREE_UNUSED_PAGES),
            vec![vec![0x0600_0000], vec![0x0600_0100]]
        );
        assert_eq!(
            calls(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![FREE_PAGES_SECTION]]
        );
        // Second call: no initialization.
        e.call_log = Some(vec![]);
        e.call(0x00aa7030, &[]);
        assert!(calls(&e, INITIALIZE_CRITICAL_SECTION).is_empty());
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION),
            vec![vec![FREE_PAGES_SECTION]]
        );
    }

    #[test]
    fn merge_sort_orders_by_address() {
        let mut e = pool_engine();
        let nodes = [
            0x0500_0050,
            0x0500_0010,
            0x0500_0090,
            0x0500_0030,
            0x0500_0070,
            0x0500_0020,
            0x0500_0080,
        ];
        let head = list(&mut e, &nodes);
        let sorted = e.call(0x00aa70c0, &args![0u32, head]).u32();
        let mut expected = nodes.to_vec();
        expected.sort();
        assert_eq!(walk(&e, sorted), expected);
    }

    #[test]
    fn merge_sort_of_empty_and_single_lists() {
        let mut e = pool_engine();
        assert_eq!(e.call(0x00aa70c0, &args![0u32, 0u32]).u32(), 0);
        let head = list(&mut e, &[0x0500_0040]);
        e.mem.set_u32(head, 0x1234);
        assert_eq!(e.call(0x00aa70c0, &args![0u32, head]).u32(), head);
        assert_eq!(walk(&e, head), vec![head]);
    }

    #[test]
    fn sort_needs_more_than_one_free_block() {
        let mut e = pool_engine();
        let pool = e.mem.alloc(0x60);
        let head = list(&mut e, &[0x0500_0020, 0x0500_0010]);
        e.mem.set_u32(pool + POOL_FREE_HEAD, head);
        e.mem.set_u32(pool + POOL_FREE_COUNT, 1);
        e.call(0x00aa7260, &args![pool]);
        assert_eq!(e.mem.u32(pool + POOL_FREE_HEAD), 0x0500_0020);
        e.mem.set_u32(pool + POOL_FREE_COUNT, 2);
        e.call(0x00aa7260, &args![pool]);
        assert_eq!(e.mem.u32(pool + POOL_FREE_HEAD), 0x0500_0010);
        assert_eq!(walk(&e, 0x0500_0010), vec![0x0500_0010, 0x0500_0020]);
    }

    #[test]
    fn sort_pool_by_index_takes_and_releases_the_lock() {
        let mut e = pool_engine();
        e.call(0x00aa7290, &args![5u32]);
        assert!(calls(&e, SPIN_LOCK_ENTER).is_empty());

        let pool = e.mem.alloc(0x60);
        e.set_global(POOLS + 4 * 5, pool);
        let head = list(&mut e, &[0x0500_0020, 0x0500_0010]);
        e.mem.set_u32(pool + POOL_FREE_HEAD, head);
        e.mem.set_u32(pool + POOL_FREE_COUNT, 2);
        e.call(0x00aa7290, &args![5u32]);
        assert_eq!(
            calls(&e, SPIN_LOCK_ENTER),
            vec![vec![pool + 0x20, SORT_FREE_BLOCKS_NAME]]
        );
        assert_eq!(e.mem.u32(pool + POOL_FREE_HEAD), 0x0500_0010);
        assert_eq!(e.mem.u32(pool + 0x24), 0);
        assert_eq!(e.mem.u32(pool + 0x20), 0);

        // A nested hold stays owned.
        e.mem.set_u32(pool + 0x24, 1);
        e.mem.set_u32(pool + 0x20, 1);
        e.call(0x00aa7290, &args![5u32]);
        assert_eq!(e.mem.u32(pool + 0x24), 1);
        assert_eq!(e.mem.u32(pool + 0x20), 1);
    }

    #[test]
    fn check_free_blocks_walks_under_the_lock() {
        let mut e = pool_engine();
        let pool = e.mem.alloc(0x60);
        e.mem.set_u32(pool + POOL_CHECK_SIZE, 9);
        let head = list(&mut e, &[0x0500_0020, 0x0500_0010]);
        e.mem.set_u32(pool + POOL_FREE_HEAD, head);
        e.call(0x00aa7300, &args![pool]);
        assert_eq!(
            calls(&e, SPIN_LOCK_ENTER),
            vec![vec![pool + 0x20, CHECK_FREE_BLOCKS_NAME]]
        );
        assert_eq!(e.mem.u32(pool + 0x24), 0);
        assert_eq!(e.mem.u32(pool + 0x20), 0);
        // Size 8 or less: the list (here unreadable) is not walked.
        e.mem.set_u32(pool + POOL_CHECK_SIZE, 8);
        e.mem.set_u32(pool + POOL_FREE_HEAD, 0x7000_0000);
        e.call(0x00aa7300, &args![pool]);
    }

    #[test]
    fn check_every_pool_under_a_static_section() {
        let mut e = pool_engine();
        let pool = e.mem.alloc(0x60);
        e.set_global(POOLS + 4 * 7, pool);
        e.call(0x00aa7370, &[]);
        assert_eq!(e.global::<u32>(CHECK_GUARD) & 1, 1);
        assert_eq!(
            calls(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![CHECK_SECTION]]
        );
        assert_eq!(calls(&e, ATEXIT), vec![vec![CHECK_SECTION_EXIT]]);
        assert_eq!(
            calls(&e, SPIN_LOCK_ENTER),
            vec![vec![pool + 0x20, CHECK_FREE_BLOCKS_NAME]]
        );
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![CHECK_SECTION]]);
        e.call_log = Some(vec![]);
        e.call(0x00aa7370, &[]);
        assert!(calls(&e, ATEXIT).is_empty());
    }

    #[test]
    fn zero_overhead_heap_constructor() {
        let mut e = pool_engine();
        e.register(MEMORY_MANAGER_MALLOC, |_, _| 0x0700_0000u32.into_ret());
        let heap: Ptr<ZeroOverheadHeap> = e.new_object();
        for off in (0..0x28).step_by(4) {
            e.mem.set_u32(heap.addr() + off, 0xAAAA_AAAA);
        }
        let r = e
            .call(0x00aa7400, &args![heap, 0x4000u32, 0x0108_0000u32])
            .u32();
        assert_eq!(r, heap.addr());
        assert_eq!(
            e.get(heap, ZeroOverheadHeap::vtable),
            ZERO_OVERHEAD_HEAP_VTABLE
        );
        assert_eq!(e.get(heap, ZeroOverheadHeap::iSize), 0x4000);
        assert_eq!(e.get(heap, ZeroOverheadHeap::pName), 0x0108_0000);
        assert_eq!(e.get(heap, ZeroOverheadHeap::pHeap), 0x0700_0000);
        assert_eq!(e.get(heap, ZeroOverheadHeap::pCurrentFree), 0x0700_0000);
        assert_eq!(e.get(heap, ZeroOverheadHeap::iAllocations), 0);
        assert_eq!(e.get(heap, ZeroOverheadHeap::lockOwner), 0);
        assert_eq!(e.get(heap, ZeroOverheadHeap::lockCount), 0);
        // +0x18 and +0x1C are left alone.
        assert_eq!(e.mem.u32(heap.addr() + 0x18), 0xAAAA_AAAA);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_MALLOC),
            vec![vec![MEMORY_MANAGER, 0x4000, 1]]
        );
    }
}
