//! `BSCore/run_00aa6680` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MemoryPool`'s constructor and destructor body and the release of its
//! address range (the layout of `memorypool.rs`). Two PC-only words the
//! constructor fills, +0x5c and +0x60, are written at their offsets.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bscore::memorypool::MemoryPool;

/// The `MemoryManager` singleton, its allocation (`00aa3e40`,
/// `thiscall(manager, size)`) and its deallocation (`00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// Reserves the pool's address range (`00aa65b0`, `run_00aa6590.rs`).
const RESERVE_RANGE: u32 = 0x00aa_65b0;
/// The pool steps after its page counts are set (`00aa6980`) and before
/// they are freed (`00aa6a10`), both in `memorypool.rs`.
const AFTER_COUNTS: u32 = 0x00aa_6980;
const BEFORE_FREE: u32 = 0x00aa_6a10;
/// Import slot of `VirtualFree`, and `MEM_RELEASE`.
const VIRTUAL_FREE: u32 = 0x00fd_f10c;
const MEM_RELEASE: u32 = 0x8000;
/// The PC-only words: the blocks of a page split into a sixth (+0x5c) and
/// the rest (+0x60).
const SIXTH_OF_PAGE_BLOCKS: u32 = 0x5c;
const REST_OF_PAGE_BLOCKS: u32 = 0x60;

// Translated from 00aa6680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the pool's reserved range (`VirtualFree(pHeap, 0,
/// MEM_RELEASE)`), answering its result.
pub fn fn_00aa6680(e: &mut Engine, this: Ptr<MemoryPool>) -> u32 {
    let heap = e.get(this, MemoryPool::pHeap);
    e.call(VIRTUAL_FREE, &args![heap, 0u32, MEM_RELEASE]).u32()
}

// Translated from 00aa66a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryPool::MemoryPool(block_size, pool_size, description)` (the
/// class by its fields): clears the heap, free list, lock and counters,
/// keeps `iPoolSize`, `iNumPages` (`pool_size >> 12`) and
/// `iAlignedBlockSize` (rounded up to 4 when not 0), reserves the range
/// (`00aa65b0`); with a range, allocates the `u16` page counts (all ones
/// on overflow) from the `MemoryManager`, splits a page's block count
/// (`0x1000 / iAlignedBlockSize`) into a sixth (+0x5c) and the rest
/// (+0x60), marks every page uncommitted (`0xffff`) and runs `00aa6980`.
/// Returns `this`.
pub fn memory_pool_construct(
    e: &mut Engine,
    this: Ptr<MemoryPool>,
    block_size: u32,
    pool_size: u32,
    description: u32,
) -> Ptr<MemoryPool> {
    e.set(this, MemoryPool::pDescription, description);
    e.set(this, MemoryPool::pHeap, 0);
    e.set(this, MemoryPool::pFreeHead, 0);
    e.set(this, MemoryPool::uiOwningThread, 0);
    e.set(this, MemoryPool::uiLockCount, 0);
    e.set(this, MemoryPool::iHighCommitedPages, 0);
    e.set(this, MemoryPool::pPageRefCount, 0);
    e.set(this, MemoryPool::iPoolSize, pool_size);
    e.set(this, MemoryPool::iNumFreeBlocks, 0);
    e.set(this, MemoryPool::iNumCommittedPages, 0);
    e.set(this, MemoryPool::iNumPages, pool_size >> 12);
    e.set(this, MemoryPool::iAlignedBlockSize, block_size);
    if e.get(this, MemoryPool::iAlignedBlockSize) != 0 {
        let aligned = e.get(this, MemoryPool::iAlignedBlockSize).wrapping_add(3) & 0xffff_fffc;
        e.set(this, MemoryPool::iAlignedBlockSize, aligned);
    }
    e.call(RESERVE_RANGE, &args![this]);
    if e.get(this, MemoryPool::pHeap) == 0 {
        return this;
    }
    // `MUL` by 2; `SETO`/`NEG`/`OR` make an overflowing size all ones.
    let bytes = (pool_size >> 12).checked_mul(2).unwrap_or(u32::MAX);
    let counts = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    e.set(this, MemoryPool::pPageRefCount, counts);
    // `DIV`: a zero block size faults here in the game, as it panics here.
    let per_page = 0x1000 / e.get(this, MemoryPool::iAlignedBlockSize);
    e.mem
        .set_u32(this.addr() + SIXTH_OF_PAGE_BLOCKS, per_page / 6);
    let sixth = e.mem.u32(this.addr() + SIXTH_OF_PAGE_BLOCKS);
    e.mem.set_u32(
        this.addr() + REST_OF_PAGE_BLOCKS,
        per_page.wrapping_sub(sixth),
    );
    let mut page = 0;
    while page < e.get(this, MemoryPool::iNumPages) {
        let counts = e.get(this, MemoryPool::pPageRefCount);
        e.mem
            .set_u16(counts.wrapping_add(page.wrapping_mul(2)), 0xffff);
        page += 1;
    }
    e.call(AFTER_COUNTS, &args![this]);
    this
}

// Translated from 00aa6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MemoryPool` destructor body (`memorypool.rs` calls it by address):
/// with a range, releases it (`00aa6680`) and runs `00aa6a10`; then gives
/// the page counts back to the `MemoryManager`.
pub fn fn_00aa6800(e: &mut Engine, this: Ptr<MemoryPool>) {
    if e.get(this, MemoryPool::pHeap) != 0 {
        fn_00aa6680(e, this);
        e.call(BEFORE_FREE, &args![this]);
    }
    let counts = e.get(this, MemoryPool::pPageRefCount);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, counts]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa6680, fn_00aa6680(Ptr<MemoryPool>) -> u32),
        entry!(
            0x00aa66a0,
            memory_pool_construct(Ptr<MemoryPool>, u32, u32, u32) -> Ptr<MemoryPool>
        ),
        entry!(0x00aa6800, fn_00aa6800(Ptr<MemoryPool>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn release_frees_the_range() {
        let mut e = Engine::new();
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        let pool: Ptr<MemoryPool> = e.new_object();
        e.set(pool, MemoryPool::pHeap, 0x0100_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_6680, &args![pool]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_FREE),
            vec![vec![0x0100_0000, 0, MEM_RELEASE]]
        );
    }

    #[test]
    fn constructor_sets_up_the_page_counts() {
        let mut e = Engine::new();
        e.register(RESERVE_RANGE, |e, a| {
            e.mem.set_u32(a[0] + 4, 0x0100_0000);
            Ret::default()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(AFTER_COUNTS, no_op);
        let pool: Ptr<MemoryPool> = e.new_object();
        e.set(pool, MemoryPool::pFreeHead, 9);
        e.call_log = Some(vec![]);
        let back = e.call(0x00aa_66a0, &args![pool, 0x1du32, 0x3000u32, 0x0abcu32]);
        assert_eq!(back.u32(), pool.addr());
        assert_eq!(e.get(pool, MemoryPool::pDescription), 0x0abc);
        assert_eq!(e.get(pool, MemoryPool::pFreeHead), 0);
        assert_eq!(e.get(pool, MemoryPool::iAlignedBlockSize), 0x20);
        assert_eq!(e.get(pool, MemoryPool::iNumPages), 3);
        assert_eq!(e.get(pool, MemoryPool::iPoolSize), 0x3000);
        // 0x1000 / 0x20 = 128 blocks a page: 21 and 107.
        assert_eq!(e.mem.u32(pool.addr() + SIXTH_OF_PAGE_BLOCKS), 21);
        assert_eq!(e.mem.u32(pool.addr() + REST_OF_PAGE_BLOCKS), 107);
        let counts = e.get(pool, MemoryPool::pPageRefCount);
        for page in 0..3 {
            assert_eq!(e.mem.u16(counts + 2 * page), 0xffff);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 6]]
        );
        assert_eq!(calls(&log, AFTER_COUNTS), vec![vec![pool.addr()]]);
    }

    #[test]
    fn constructor_stops_without_a_range() {
        let mut e = Engine::new();
        e.register(RESERVE_RANGE, no_op);
        let pool: Ptr<MemoryPool> = e.new_object();
        e.call_log = Some(vec![]);
        // A zero block size stays 0 (no rounding) and is never divided by.
        e.call(0x00aa_66a0, &args![pool, 0u32, 0x3000u32, 0u32]);
        assert_eq!(e.get(pool, MemoryPool::iAlignedBlockSize), 0);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert!(calls(&log, AFTER_COUNTS).is_empty());
    }

    #[test]
    fn destructor_body_releases_then_frees_the_counts() {
        let mut e = Engine::new();
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        e.register(BEFORE_FREE, no_op);
        e.register(MEMORY_MANAGER_DEALLOCATE, no_op);
        for heap in [0x0100_0000u32, 0] {
            let pool: Ptr<MemoryPool> = e.new_object();
            e.set(pool, MemoryPool::pHeap, heap);
            e.set(pool, MemoryPool::pPageRefCount, 0x4444);
            e.call_log = Some(vec![]);
            e.call(0x00aa_6800, &args![pool]);
            let log = e.call_log.take().unwrap();
            let ranged = heap != 0;
            assert_eq!(calls(&log, VIRTUAL_FREE).len(), usize::from(ranged));
            assert_eq!(calls(&log, BEFORE_FREE).len(), usize::from(ranged));
            assert_eq!(
                calls(&log, MEMORY_MANAGER_DEALLOCATE),
                vec![vec![MEMORY_MANAGER, 0x4444]]
            );
        }
    }
}
