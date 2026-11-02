//! `BSCore/zerooverheadheap.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ZeroOverheadHeap`: a bump allocator over one block (`pHeap`, `iSize`)
//! whose allocations each carry a one-word size header and are never
//! freed. The layout is the one `run_00aa7030.rs` declares with the
//! class's constructor.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bscore::run_00aa7030::ZeroOverheadHeap;

/// `ZeroOverheadHeap`'s vtable and `IMemoryHeap`'s (RTTI locators).
const ZERO_OVERHEAD_HEAP_VTABLE: u32 = 0x010a_2884;
const I_MEMORY_HEAP_VTABLE: u32 = 0x010a_28c8;
/// The `MemoryManager` singleton, its block free (`00aa42c0`,
/// `thiscall(manager, block, 1)`) and its deallocation (`00aa4060`,
/// `thiscall(manager, block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_42c0;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The spin-lock acquire (`this` = lock, one word: a name string, here
/// null).
const SPIN_LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00aa74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::~ZeroOverheadHeap` (the class by its vtable): gives
/// `pHeap` back to the `MemoryManager` (`00aa42c0(pHeap, 1)`) between
/// restoring the `ZeroOverheadHeap` and the `IMemoryHeap` vtables.
// Translation incomplete: 00aa74c0 - the FS:[0] exception frame (restores the IMemoryHeap vtable if the free throws) is not translated
pub fn zero_overhead_heap_destruct(e: &mut Engine, this: Ptr<ZeroOverheadHeap>) {
    e.set(this, ZeroOverheadHeap::vtable, ZERO_OVERHEAD_HEAP_VTABLE);
    let heap = e.get(this, ZeroOverheadHeap::pHeap);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, heap, 1u32]);
    e.set(this, ZeroOverheadHeap::vtable, I_MEMORY_HEAP_VTABLE);
}

// Translated from 00aa7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::Allocate` (engine map; two words, the second not
/// read): null without a heap; otherwise, when `pCurrentFree + size + 4`
/// is still below the heap's end, under the spin lock at +0x20 writes the
/// size header at `pCurrentFree`, answers the word after it and moves
/// `pCurrentFree` past the block, counting it in `iAllocations`. A request
/// that does not fit answers null.
pub fn zero_overhead_heap_allocate(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    size: u32,
    _unused_2: u32,
) -> u32 {
    if e.get(this, ZeroOverheadHeap::pHeap) == 0 {
        return 0;
    }
    let mut block = 0;
    let end_of_block = e
        .get(this, ZeroOverheadHeap::pCurrentFree)
        .wrapping_add(size)
        .wrapping_add(4);
    let end_of_heap = e
        .get(this, ZeroOverheadHeap::pHeap)
        .wrapping_add(e.get(this, ZeroOverheadHeap::iSize));
    if end_of_block < end_of_heap {
        let lock = this.addr() + 0x20;
        e.call(SPIN_LOCK_ENTER, &args![lock, 0u32]);
        let header = e.get(this, ZeroOverheadHeap::pCurrentFree);
        e.mem.set_u32(header, size);
        block = e.get(this, ZeroOverheadHeap::pCurrentFree).wrapping_add(4);
        e.set(
            this,
            ZeroOverheadHeap::pCurrentFree,
            block.wrapping_add(size),
        );
        let count = e.get(this, ZeroOverheadHeap::iAllocations).wrapping_add(1);
        e.set(this, ZeroOverheadHeap::iAllocations, count);
        // The inlined spin-lock release.
        let depth = e.get(this, ZeroOverheadHeap::lockCount).wrapping_sub(1);
        e.set(this, ZeroOverheadHeap::lockCount, depth);
        if e.get(this, ZeroOverheadHeap::lockCount) == 0 {
            e.set(this, ZeroOverheadHeap::lockOwner, 0);
        }
    }
    block
}

// Translated from 00aa75e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The heap's free (two words): does nothing (a zero-overhead heap never
/// frees a block).
pub fn fn_00aa75e0(_e: &mut Engine, _this: Ptr<ZeroOverheadHeap>, _unused_1: u32, _unused_2: u32) {}

// Translated from 00aa75f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::PointerInHeap` (engine map): whether `pointer` lies
/// strictly after `pHeap` and before `pHeap + iSize`.
pub fn zero_overhead_heap_pointer_in_heap(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    pointer: u32,
) -> bool {
    if pointer <= e.get(this, ZeroOverheadHeap::pHeap) {
        return false;
    }
    let end = e
        .get(this, ZeroOverheadHeap::pHeap)
        .wrapping_add(e.get(this, ZeroOverheadHeap::iSize));
    pointer < end
}

// Translated from 00aa77e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap`'s scalar deleting destructor (engine map): the
/// destructor, then, with bit 0 of `flags`, the object goes back to the
/// `MemoryManager` (`00aa4060`). Returns `this`.
pub fn zero_overhead_heap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    flags: u32,
) -> Ptr<ZeroOverheadHeap> {
    zero_overhead_heap_destruct(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// Writes one zero word through `stream`'s virtual slot +8 (`(buffer, 4)`) for a non-null
/// stream, answering what it answers; 0 for a null stream (`00aa7700`, `00aa7740`).
fn write_zero_word(e: &mut Engine, stream: u32) -> u32 {
    if stream == 0 {
        return 0;
    }
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.vcall(stream, 8, &args![word, 4u32]).u32()
    })
}

// Translated from 00aa7630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::GetHeapStats` (Xbox PDB; `thiscall`, two words, `RET 8`, the second
/// not read): with a non-null `stats`, fills its fourteen words: the name (the heap's own
/// virtual slot +4), `iSize` twice, `pCurrentFree - pHeap`, `iAllocations`, 0, 0,
/// `pCurrentFree - pHeap` again, 0, 0, 0x40, 0, `iAllocations * 4` and 0.
pub fn zero_overhead_heap_get_heap_stats(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    stats: u32,
    _unused_2: u32,
) {
    if stats == 0 {
        return;
    }
    let name = e.vcall(this.addr(), 4, &args![]).u32();
    e.mem.set_u32(stats, name);
    let size = e.get(this, ZeroOverheadHeap::iSize);
    e.mem.set_u32(stats + 4, size);
    let size = e.get(this, ZeroOverheadHeap::iSize);
    e.mem.set_u32(stats + 8, size);
    let used = e
        .get(this, ZeroOverheadHeap::pCurrentFree)
        .wrapping_sub(e.get(this, ZeroOverheadHeap::pHeap));
    e.mem.set_u32(stats + 0xc, used);
    let allocations = e.get(this, ZeroOverheadHeap::iAllocations);
    e.mem.set_u32(stats + 0x10, allocations);
    e.mem.set_u32(stats + 0x14, 0);
    e.mem.set_u32(stats + 0x18, 0);
    let used = e
        .get(this, ZeroOverheadHeap::pCurrentFree)
        .wrapping_sub(e.get(this, ZeroOverheadHeap::pHeap));
    e.mem.set_u32(stats + 0x1c, used);
    e.mem.set_u32(stats + 0x20, 0);
    e.mem.set_u32(stats + 0x24, 0);
    e.mem.set_u32(stats + 0x28, 0x40);
    e.mem.set_u32(stats + 0x2c, 0);
    let allocations = e.get(this, ZeroOverheadHeap::iAllocations);
    e.mem.set_u32(stats + 0x30, allocations << 2);
    e.mem.set_u32(stats + 0x34, 0);
}

// Translated from 00aa7700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x1c of the `ZeroOverheadHeap` vtable (`OutputBucketInfo`
/// in the Xbox PDB's `IMemoryHeap` order) (`thiscall`, one word: a stream): 0 for a null
/// stream, else one zero word written through the stream's slot +8, answering what that
/// answers.
pub fn fn_00aa7700(e: &mut Engine, _this: Ptr<ZeroOverheadHeap>, stream: u32) -> u32 {
    write_zero_word(e, stream)
}

// Translated from 00aa7740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x20 of the `ZeroOverheadHeap` vtable (`OutputBlockInfo`
/// in the Xbox PDB's order) (`thiscall`, two words, the second not read): as `00aa7700`.
pub fn fn_00aa7740(
    e: &mut Engine,
    _this: Ptr<ZeroOverheadHeap>,
    stream: u32,
    _unused_2: u32,
) -> u32 {
    write_zero_word(e, stream)
}

// Translated from 00aa7780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::Size` (Xbox PDB; `thiscall`, two words, the second not read): the
/// block's size header, the word before it.
pub fn zero_overhead_heap_size(
    e: &mut Engine,
    _this: Ptr<ZeroOverheadHeap>,
    block: u32,
    _unused_2: u32,
) -> u32 {
    e.mem.u32(block.wrapping_sub(4))
}

// Translated from 00aa77b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ZeroOverheadHeap::TotalSize` (Xbox PDB; `thiscall`, one word): the heap's own slot +0x24
/// (`Size`) with the block and 0, plus 4 (the header).
pub fn zero_overhead_heap_total_size(
    e: &mut Engine,
    this: Ptr<ZeroOverheadHeap>,
    block: u32,
) -> u32 {
    e.vcall(this.addr(), 0x24, &args![block, 0u32])
        .u32()
        .wrapping_add(4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_7630,
            zero_overhead_heap_get_heap_stats(Ptr<ZeroOverheadHeap>, u32, u32)
        ),
        entry!(0x00aa_7700, fn_00aa7700(Ptr<ZeroOverheadHeap>, u32) -> u32),
        entry!(
            0x00aa_7740,
            fn_00aa7740(Ptr<ZeroOverheadHeap>, u32, u32) -> u32
        ),
        entry!(
            0x00aa_7780,
            zero_overhead_heap_size(Ptr<ZeroOverheadHeap>, u32, u32) -> u32
        ),
        entry!(
            0x00aa_77b0,
            zero_overhead_heap_total_size(Ptr<ZeroOverheadHeap>, u32) -> u32
        ),
        entry!(
            0x00aa74c0,
            zero_overhead_heap_destruct(Ptr<ZeroOverheadHeap>)
        ),
        entry!(
            0x00aa7530,
            zero_overhead_heap_allocate(Ptr<ZeroOverheadHeap>, u32, u32) -> u32
        ),
        entry!(0x00aa75e0, fn_00aa75e0(Ptr<ZeroOverheadHeap>, u32, u32)),
        entry!(
            0x00aa75f0,
            zero_overhead_heap_pointer_in_heap(Ptr<ZeroOverheadHeap>, u32) -> bool
        ),
        entry!(
            0x00aa77e0,
            zero_overhead_heap_scalar_deleting_destructor(
                Ptr<ZeroOverheadHeap>,
                u32,
            ) -> Ptr<ZeroOverheadHeap>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn heap(e: &mut Engine, size: u32) -> Ptr<ZeroOverheadHeap> {
        let this: Ptr<ZeroOverheadHeap> = e.new_object();
        let block = e.mem.alloc(size);
        e.set(this, ZeroOverheadHeap::pHeap, block);
        e.set(this, ZeroOverheadHeap::iSize, size);
        e.set(this, ZeroOverheadHeap::pCurrentFree, block);
        this
    }

    #[test]
    fn destructor_frees_the_block_between_the_vtables() {
        let mut e = Engine::new();
        let this = heap(&mut e, 0x40);
        // The free runs with the ZeroOverheadHeap vtable in place.
        let object = this.addr();
        e.register_double(MEMORY_MANAGER_FREE, move |e, _| {
            assert_eq!(e.mem.u32(object), ZERO_OVERHEAD_HEAP_VTABLE);
            Ret::default()
        });
        let block = e.get(this, ZeroOverheadHeap::pHeap);
        e.call_log = Some(vec![]);
        e.call(0x00aa_74c0, &args![this]);
        assert_eq!(e.get(this, ZeroOverheadHeap::vtable), I_MEMORY_HEAP_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, block, 1]]
        );
    }

    #[test]
    fn allocate_bumps_with_a_size_header_under_the_lock() {
        let mut e = Engine::new();
        // The acquire takes the lock once.
        e.register(SPIN_LOCK_ENTER, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            let n = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, n);
            Ret::default()
        });
        let this = heap(&mut e, 0x40);
        let base = e.get(this, ZeroOverheadHeap::pHeap);
        e.call_log = Some(vec![]);
        let first = e.call(0x00aa_7530, &args![this, 0x10u32, 0u32]).u32();
        assert_eq!(first, base + 4);
        assert_eq!(e.mem.u32(base), 0x10);
        assert_eq!(e.get(this, ZeroOverheadHeap::pCurrentFree), base + 0x14);
        assert_eq!(e.get(this, ZeroOverheadHeap::iAllocations), 1);
        assert_eq!(e.get(this, ZeroOverheadHeap::lockOwner), 0);
        assert_eq!(e.get(this, ZeroOverheadHeap::lockCount), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SPIN_LOCK_ENTER),
            vec![vec![this.addr() + 0x20, 0]]
        );
        // 0x14 + 0x27 + 4 = 0x3f fits below 0x40; 0x28 would reach the end.
        assert_eq!(e.call(0x00aa_7530, &args![this, 0x28u32, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x00aa_7530, &args![this, 0x27u32, 0u32]).u32(),
            base + 0x18
        );
        assert_eq!(e.get(this, ZeroOverheadHeap::iAllocations), 2);
        // Without a heap: null.
        let empty: Ptr<ZeroOverheadHeap> = e.new_object();
        assert_eq!(e.call(0x00aa_7530, &args![empty, 4u32, 0u32]).u32(), 0);
    }

    #[test]
    fn free_does_nothing() {
        let mut e = Engine::new();
        let this = heap(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00aa_75e0, &args![this, 1u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn pointer_in_heap_excludes_the_start() {
        let mut e = Engine::new();
        let this = heap(&mut e, 0x40);
        let base = e.get(this, ZeroOverheadHeap::pHeap);
        for (pointer, inside) in [
            (base, false),
            (base + 1, true),
            (base + 0x3f, true),
            (base + 0x40, false),
        ] {
            assert_eq!(e.call(0x00aa_75f0, &args![this, pointer]).bool(), inside);
        }
    }

    #[test]
    fn deleting_destructor_deallocates_on_bit_zero() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let this = heap(&mut e, 0x10);
        for (flags, frees) in [(0u32, false), (1, true), (2, false)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00aa_77e0, &args![this, flags]).u32(), this.addr());
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, MEMORY_MANAGER_FREE).len(), 1);
            assert_eq!(
                calls(&log, MEMORY_MANAGER_DEALLOCATE).len(),
                usize::from(frees)
            );
        }
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    /// A heap object whose vtable at 0x7000 has slot `i` at `0x8000 + 4 * i`.
    fn heap(e: &mut Engine) -> Ptr<ZeroOverheadHeap> {
        let slots: Vec<u32> = (0..0x10).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        let heap = e.new_object::<ZeroOverheadHeap>();
        e.mem.set_u32(heap.addr(), 0x7000);
        heap
    }

    #[test]
    fn get_heap_stats_fills_the_record() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(0x8004, |_, _| 0x1234u32.into_ret());
        e.set(this, ZeroOverheadHeap::iSize, 0x1000);
        e.set(this, ZeroOverheadHeap::pHeap, 0x5000);
        e.set(this, ZeroOverheadHeap::pCurrentFree, 0x5300);
        e.set(this, ZeroOverheadHeap::iAllocations, 6);
        let stats = e.mem.alloc(0x38);
        for i in 0..14 {
            e.mem.set_u32(stats + 4 * i, 0xdead_beef);
        }
        e.call(0x00aa_7630, &args![this, stats, 0u32]);
        let words: Vec<u32> = (0..14).map(|i| e.mem.u32(stats + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x1234, 0x1000, 0x1000, 0x300, 6, 0, 0, 0x300, 0, 0, 0x40, 0, 24, 0]
        );
        // A null record: nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x00aa_7630, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn outputs_write_one_zero_word_to_a_stream() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(0x8008, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            (a[2] + 0x10).into_ret()
        });
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_7700, &args![this, stream]).u32(), 0x14);
        assert_eq!(e.call(0x00aa_7740, &args![this, stream, 9u32]).u32(), 0x14);
        assert_eq!(e.call(0x00aa_7700, &args![this, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00aa_7740, &args![this, 0u32, 9u32]).u32(), 0);
        let writes = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x8008)
            .count();
        assert_eq!(writes, 2);
    }

    #[test]
    fn size_and_total_size_read_the_header() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(0x8024, |_, a| {
            assert_eq!(a[2], 0);
            (a[1] & 0xff).into_ret()
        });
        let block = e.mem.alloc(8) + 4;
        e.mem.set_u32(block - 4, 0x30);
        assert_eq!(e.call(0x00aa_7780, &args![this, block, 7u32]).u32(), 0x30);
        assert_eq!(
            e.call(0x00aa_77b0, &args![this, block]).u32(),
            (block & 0xff) + 4
        );
    }
}
