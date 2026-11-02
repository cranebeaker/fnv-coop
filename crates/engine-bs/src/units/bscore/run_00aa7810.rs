//! `BSCore/run_00aa7810` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MemoryHeap` (vtable `010a28fc`, RTTI `.?AVMemoryHeap@@`): its
//! constructor and destructor, and the members its vtable holds at `+0x08`
//! (allocate), `+0x0c` (free), `+0x10` (pointer in heap), `+0x24` (block
//! size), `+0x30` (reserve and commit the heap's memory), `+0x34` (release
//! it), `+0x38` (commit pages) and `+0x3c` (decommit pages); plus an empty
//! `IMemoryHeap` member (slot `+0x2c` of `010a28c8`). The base class's
//! fields are read at their offsets (`AbstractHeap`, Xbox PDB names, as in
//! `run_00aa8ca0.rs`); the two bytes after the base at +0x20f0 and +0x20f1
//! are `MemoryHeap`'s own (not in the Xbox PDB, whose class is larger).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryHeap`'s vtable.
const MEMORY_HEAP_VTABLE: u32 = 0x010a_28fc;
/// `AbstractHeap`'s constructor, heap set-up, tear-down and destructor
/// (`run_00aa8ca0.rs`).
const ABSTRACT_HEAP_CONSTRUCT: u32 = 0x00aa_8ca0;
const ABSTRACT_HEAP_SET_UP: u32 = 0x00aa_8dd0;
const ABSTRACT_HEAP_TEAR_DOWN: u32 = 0x00aa_8ea0;
const ABSTRACT_HEAP_DESTRUCT: u32 = 0x00aa_8da0;
/// `AbstractHeap::BaseAllocate` (decompiler name; `thiscall(heap, size)`).
const BASE_ALLOCATE: u32 = 0x00aa_8f50;
/// The heap's block free (`thiscall(heap, block)`).
const BASE_FREE: u32 = 0x00aa_98f0;
/// The `MemoryManager` singleton and its deallocation
/// (`thiscall(manager, block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// Import slots of `VirtualAlloc` and `VirtualFree`.
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
const VIRTUAL_FREE: u32 = 0x00fd_f10c;
/// `VirtualAlloc`/`VirtualFree` flags.
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_DECOMMIT: u32 = 0x4000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_READWRITE: u32 = 4;

/// `AbstractHeap` field offsets (Xbox PDB names).
const PAGE_SIZE: u32 = 0x10; // iPageSize
const MEM_HEAP_SIZE: u32 = 0x14; // MemHeapSize
const MEM_HEAP: u32 = 0x30; // pMemHeap
/// `MemoryHeap`'s own bytes (PC): the constructor's last argument, and a
/// flag the destructor sets.
const OWN_FLAG: u32 = 0x20f0;
const DESTROYED: u32 = 0x20f1;

// Translated from 00aa7830 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty `IMemoryHeap` member (`thiscall`, no arguments; slot `+0x2c` of
/// the `IMemoryHeap` vtable).
pub fn fn_00aa7830(_e: &mut Engine, _this: Ptr) {}

// Translated from 00aa78a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryHeap::MemoryHeap(size, initial_size, name, flag)` (the class by
/// its vtable): the `AbstractHeap` constructor with page size 0x10000, the
/// `MemoryHeap` vtable, `flag` at +0x20f0, 0 at +0x20f1, then the heap
/// set-up `00aa8dd0`. Returns `this`.
// Translation incomplete: 00aa78a0 - the FS:[0] exception frame (runs the AbstractHeap destructor if the set-up throws) is not translated
pub fn memory_heap_construct(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    initial_size: u32,
    name: u32,
    flag: u8,
) -> Ptr {
    e.call(
        ABSTRACT_HEAP_CONSTRUCT,
        &args![this, size, initial_size, name, 0x10000u32],
    );
    e.mem.set_u32(this.addr(), MEMORY_HEAP_VTABLE);
    e.mem.set_u8(this.addr() + OWN_FLAG, flag);
    e.mem.set_u8(this.addr() + DESTROYED, 0);
    e.call(ABSTRACT_HEAP_SET_UP, &args![this]);
    this
}

// Translated from 00aa7930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryHeap::~MemoryHeap` (the class by its vtable): the `MemoryHeap`
/// vtable, the heap tear-down `00aa8ea0`, the byte at +0x20f1 set, then the
/// `AbstractHeap` destructor.
// Translation incomplete: 00aa7930 - the FS:[0] exception frame (runs the AbstractHeap destructor if the tear-down throws) is not translated
pub fn memory_heap_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MEMORY_HEAP_VTABLE);
    e.call(ABSTRACT_HEAP_TEAR_DOWN, &args![this]);
    e.mem.set_u8(this.addr() + DESTROYED, 1);
    e.call(ABSTRACT_HEAP_DESTRUCT, &args![this]);
}

// Translated from 00aa79a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x30`: reserves `reserve` bytes anywhere
/// (`VirtualAlloc(0, reserve, MEM_RESERVE, PAGE_READWRITE)`) and commits
/// the first `commit` bytes of it; when the commit fails the reservation
/// is released and the answer is null. Before that the code reads
/// `iPageSize` (+0x10) and the byte at +0x20f0 into locals it never uses;
/// the reads are kept.
pub fn fn_00aa79a0(e: &mut Engine, this: Ptr, reserve: u32, commit: u32) -> u32 {
    let _page_size = e.mem.u32(this.addr() + PAGE_SIZE);
    let _flag = e.mem.u8(this.addr() + OWN_FLAG);
    let mut memory = e
        .call(
            VIRTUAL_ALLOC,
            &args![0u32, reserve, MEM_RESERVE, PAGE_READWRITE],
        )
        .u32();
    if memory != 0 {
        let committed = e
            .call(
                VIRTUAL_ALLOC,
                &args![memory, commit, MEM_COMMIT, PAGE_READWRITE],
            )
            .u32();
        if committed == 0 {
            e.call(VIRTUAL_FREE, &args![memory, 0u32, MEM_RELEASE]);
            memory = 0;
        }
    }
    memory
}

// Translated from 00aa7a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x34`: releases the heap's memory
/// (`VirtualFree(memory, 0, MEM_RELEASE)`), answering its result.
pub fn fn_00aa7a30(e: &mut Engine, _this: Ptr, memory: u32) -> u32 {
    e.call(VIRTUAL_FREE, &args![memory, 0u32, MEM_RELEASE])
        .u32()
}

// Translated from 00aa7a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x38`: commits `size` bytes at `base + offset`, the size
/// first raised by one page and rounded down to a page
/// (`(size + iPageSize) & !(iPageSize - 1)`); answers that size, or 0 when
/// `VirtualAlloc` fails.
pub fn fn_00aa7a50(e: &mut Engine, this: Ptr, base: u32, offset: u32, size: u32) -> u32 {
    let at = base.wrapping_add(offset);
    let page = e.mem.u32(this.addr() + PAGE_SIZE);
    let mut size = size.wrapping_add(page) & !page.wrapping_sub(1);
    let committed = e
        .call(VIRTUAL_ALLOC, &args![at, size, MEM_COMMIT, PAGE_READWRITE])
        .u32();
    if committed == 0 {
        size = 0;
    }
    size
}

// Translated from 00aa7ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x3c`: rounds `size` down to a page; below one page it
/// answers 0, otherwise decommits that many bytes ending at `base +
/// offset` (`VirtualFree(.., MEM_DECOMMIT)`) and answers the size, or 0
/// when `VirtualFree` does not answer exactly 1.
pub fn fn_00aa7ab0(e: &mut Engine, this: Ptr, base: u32, offset: u32, size: u32) -> u32 {
    let page = e.mem.u32(this.addr() + PAGE_SIZE);
    let mut size = !page.wrapping_sub(1) & size;
    if size < e.mem.u32(this.addr() + PAGE_SIZE) {
        size = 0;
    } else {
        let start = base.wrapping_add(offset).wrapping_sub(size);
        let freed = e
            .call(VIRTUAL_FREE, &args![start, size, MEM_DECOMMIT])
            .u32();
        if freed != 1 {
            size = 0;
        }
    }
    size
}

// Translated from 00aa7b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x08` (allocate; two words, the second not read): the
/// heap's block allocation `00aa8f50(size)`, answering the block.
pub fn fn_00aa7b20(e: &mut Engine, this: Ptr, size: u32, _unused_2: u32) -> u32 {
    e.call(BASE_ALLOCATE, &args![this, size]).u32()
}

// Translated from 00aa7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x0c` (free; two words, the second not read): frees a
/// non-null block through `00aa98f0`.
pub fn fn_00aa7b40(e: &mut Engine, this: Ptr, block: u32, _unused_2: u32) {
    if block != 0 {
        e.call(BASE_FREE, &args![this, block]);
    }
}

// Translated from 00aa7be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x24` (two words, the second not read): a block's size,
/// the low 28 bits of the header word just before it.
pub fn fn_00aa7be0(e: &mut Engine, _this: Ptr, block: u32, _unused_2: u32) -> u32 {
    e.mem.u32(block.wrapping_sub(4)) & 0x0fff_ffff
}

// Translated from 00aa7e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryHeap`'s scalar deleting destructor (vtable slot `+0`): the
/// destructor, then, with bit 0 of `flags`, the block goes back to the
/// `MemoryManager` singleton (`00aa4060`). Returns `this`.
pub fn fn_00aa7e20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    memory_heap_destruct(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00aa7e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x10`: whether `pointer` lies in `[pMemHeap, pMemHeap +
/// MemHeapSize)`.
pub fn fn_00aa7e70(e: &mut Engine, this: Ptr, pointer: u32) -> bool {
    let start = e.mem.u32(this.addr() + MEM_HEAP);
    if pointer < start {
        return false;
    }
    let end = e
        .mem
        .u32(this.addr() + MEM_HEAP)
        .wrapping_add(e.mem.u32(this.addr() + MEM_HEAP_SIZE));
    pointer < end
}

/// `IMemoryHeap`'s vtable (`010a28c8`, RTTI `.?AVIMemoryHeap@@`).
const I_MEMORY_HEAP_VTABLE: u32 = 0x010a_28c8;
/// `AbstractHeap::pName` (Xbox PDB, +4).
const NAME: u32 = 0x04;
/// `AbstractHeap`'s stats, bucket and block writers (`00aaa0e0`, `00aaa310`, `00aaa430`;
/// `thiscall`, two, one and two words), which the `AbstractHeap` vtable `010a2bbc` holds at
/// +0x18, +0x1c and +0x20.
const ABSTRACT_HEAP_STATS: u32 = 0x00aa_a0e0;
const ABSTRACT_HEAP_BUCKETS: u32 = 0x00aa_a310;
const ABSTRACT_HEAP_BLOCKS: u32 = 0x00aa_a430;
/// A block header's flag the decommit sets in its size word (+4).
const DECOMMITTED: u32 = 0x2000_0000;

/// What `00aa7ca0` and `00aa7d00` write for a heap with the byte at +0x20f0 set: one zero
/// word through the stream's virtual slot +8 (`(buffer, 4)`), answering what it answers.
fn write_zero_word(e: &mut Engine, stream: u32) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.vcall(stream, 8, &args![word, 4u32]).u32()
    })
}

// Translated from 00aa7810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it): the `IMemoryHeap` destructor body
/// (`thiscall`, no stack words): stores its vtable (`010a28c8`).
pub fn fn_00aa7810(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), I_MEMORY_HEAP_VTABLE);
}

// Translated from 00aa7840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot 0 of the `IMemoryHeap` vtable, its scalar deleting
/// destructor (`thiscall`, one word of flags): the vtable `010a28c8`, then with flag bit 0
/// the object given back to the `MemoryManager` (`00aa4060`). Answers `this`.
pub fn fn_00aa7840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), I_MEMORY_HEAP_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00aa7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +4 of the `ZeroOverheadHeap` vtable (`010a2884`;
/// `GetName` in the Xbox PDB's `IMemoryHeap` order) (`thiscall`, no stack words): the word at
/// +8 (`ZeroOverheadHeap::pName`).
pub fn fn_00aa7870(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

// Translated from 00aa7890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word it does not read; a shared body in
/// several vtables, among them `BSCoreMessage`'s at +8 and +0x10 and `ZeroOverheadHeap`'s at
/// +0x30 and +0x38): answers 0.
pub fn fn_00aa7890(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

// Translated from 00aa7b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, one word: a block): true
/// when the heap's own slot +0x10 (pointer in heap) answers true (`AL`) for the block and
/// bit 0x4000_0000 of the block header's size word (`block - 8 + 4`) is clear. A further
/// test of the constant 1 the compiler kept always passes; its false arm never runs and is
/// left out.
pub fn fn_00aa7b70(e: &mut Engine, this: Ptr, block: u32) -> bool {
    let header = block.wrapping_sub(8);
    if !e.vcall(this.addr(), 0x10, &args![block]).bool() {
        return false;
    }
    e.mem.u32(header.wrapping_add(4)) & 0x4000_0000 == 0
}

// Translated from 00aa7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x28 of the `MemoryHeap` vtable (`TotalSize` in the Xbox
/// PDB's `IMemoryHeap` order) (`thiscall`, one word): the low 28 bits of the header word
/// before the block, plus 8.
pub fn fn_00aa7c00(e: &mut Engine, _this: Ptr, block: u32) -> u32 {
    (e.mem.u32(block.wrapping_sub(4)) & 0x0fff_ffff).wrapping_add(8)
}

// Translated from 00aa7c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, no stack words): answers 0
/// (through a local).
pub fn fn_00aa7c30(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00aa7c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, no stack words): answers 0.
pub fn fn_00aa7c50(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00aa7c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x18 of the `MemoryHeap` vtable (`GetHeapStats` in the
/// Xbox PDB's order) (`thiscall`, two words): `00aaa0e0(stats, flag)` with the flag's low
/// byte kept only while the byte at +0x20f0 is 0; answers what it answers.
pub fn fn_00aa7c60(e: &mut Engine, this: Ptr, stats: u32, flag: u8) -> u32 {
    let keep = if e.mem.u8(this.addr() + OWN_FLAG) != 0 {
        0
    } else {
        1
    };
    let flag = flag & keep;
    e.call(ABSTRACT_HEAP_STATS, &args![this, stats, u32::from(flag)])
        .u32()
}

// Translated from 00aa7ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x1c of the `MemoryHeap` vtable (`OutputBucketInfo` in
/// the Xbox PDB's order) (`thiscall`, one word: a stream): with the byte at +0x20f0 clear,
/// `00aaa310(stream)`; else one zero word written through the stream's slot +8. Answers
/// what that call answers.
pub fn fn_00aa7ca0(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    if e.mem.u8(this.addr() + OWN_FLAG) == 0 {
        e.call(ABSTRACT_HEAP_BUCKETS, &args![this, stream]).u32()
    } else {
        write_zero_word(e, stream)
    }
}

// Translated from 00aa7d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x20 of the `MemoryHeap` vtable (`OutputBlockInfo` in
/// the Xbox PDB's order) (`thiscall`, two words: a stream and a flag byte): with the byte at
/// +0x20f0 clear, `00aaa430(stream, flag)`; else one zero word written through the stream's
/// slot +8. Answers what that call answers.
pub fn fn_00aa7d00(e: &mut Engine, this: Ptr, stream: u32, flag: u8) -> u32 {
    if e.mem.u8(this.addr() + OWN_FLAG) == 0 {
        e.call(ABSTRACT_HEAP_BLOCKS, &args![this, stream, u32::from(flag)])
            .u32()
    } else {
        write_zero_word(e, stream)
    }
}

// Translated from 00aa7d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x40 of the `MemoryHeap` vtable (`thiscall`, one word: a
/// free block's header). When the block's size (low 28 bits of the word at +4) is at least a
/// page (`iPageSize`, +0x10), the whole pages inside its data (from header + 8, rounded up
/// to a page; the rest of the size rounded down) are decommitted (`VirtualFree(..,
/// MEM_DECOMMIT)`) if they make at least a page, and bit 0x2000_0000 of the size word is set.
/// The compiler kept a test of the constant 1 before that; its clearing arm never runs and
/// is left out.
pub fn fn_00aa7d60(e: &mut Engine, this: Ptr, header: u32) {
    let mut size = e.mem.u32(header.wrapping_add(4)) & 0x0fff_ffff;
    if size < e.mem.u32(this.addr() + PAGE_SIZE) {
        return;
    }
    let data = header.wrapping_add(8);
    let page = e.mem.u32(this.addr() + PAGE_SIZE);
    let mask = !e.mem.u32(this.addr() + PAGE_SIZE).wrapping_sub(1);
    let start = data.wrapping_add(page).wrapping_sub(1) & mask;
    let mask = !e.mem.u32(this.addr() + PAGE_SIZE).wrapping_sub(1);
    size = size.wrapping_sub(start.wrapping_sub(data)) & mask;
    if size < e.mem.u32(this.addr() + PAGE_SIZE) {
        return;
    }
    e.call(VIRTUAL_FREE, &args![start, size, MEM_DECOMMIT]);
    let word = e.mem.u32(header.wrapping_add(4));
    e.mem.set_u32(header.wrapping_add(4), word | DECOMMITTED);
}

// Translated from 00aa7e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +4 of the `MemoryHeap` vtable (`GetName` in the Xbox
/// PDB's order; also in the `AbstractHeap` vtable `010a2bbc`) (`thiscall`, no stack words):
/// `pName` (+4).
pub fn fn_00aa7e50(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + NAME)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_7810, fn_00aa7810(Ptr)),
        entry!(0x00aa_7840, fn_00aa7840(Ptr, u32) -> Ptr),
        entry!(0x00aa_7870, fn_00aa7870(Ptr) -> u32),
        entry!(0x00aa_7890, fn_00aa7890(Ptr, u32) -> u32),
        entry!(0x00aa_7b70, fn_00aa7b70(Ptr, u32) -> bool),
        entry!(0x00aa_7c00, fn_00aa7c00(Ptr, u32) -> u32),
        entry!(0x00aa_7c30, fn_00aa7c30(Ptr) -> u32),
        entry!(0x00aa_7c50, fn_00aa7c50(Ptr) -> u32),
        entry!(0x00aa_7c60, fn_00aa7c60(Ptr, u32, u8) -> u32),
        entry!(0x00aa_7ca0, fn_00aa7ca0(Ptr, u32) -> u32),
        entry!(0x00aa_7d00, fn_00aa7d00(Ptr, u32, u8) -> u32),
        entry!(0x00aa_7d60, fn_00aa7d60(Ptr, u32)),
        entry!(0x00aa_7e50, fn_00aa7e50(Ptr) -> u32),
        entry!(0x00aa7830, fn_00aa7830(Ptr)),
        entry!(
            0x00aa78a0,
            memory_heap_construct(Ptr, u32, u32, u32, u8) -> Ptr
        ),
        entry!(0x00aa7930, memory_heap_destruct(Ptr)),
        entry!(0x00aa79a0, fn_00aa79a0(Ptr, u32, u32) -> u32),
        entry!(0x00aa7a30, fn_00aa7a30(Ptr, u32) -> u32),
        entry!(0x00aa7a50, fn_00aa7a50(Ptr, u32, u32, u32) -> u32),
        entry!(0x00aa7ab0, fn_00aa7ab0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00aa7b20, fn_00aa7b20(Ptr, u32, u32) -> u32),
        entry!(0x00aa7b40, fn_00aa7b40(Ptr, u32, u32)),
        entry!(0x00aa7be0, fn_00aa7be0(Ptr, u32, u32) -> u32),
        entry!(0x00aa7e20, fn_00aa7e20(Ptr, u32) -> Ptr),
        entry!(0x00aa7e70, fn_00aa7e70(Ptr, u32) -> bool),
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

    fn heap(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x20f4);
        e.mem.set_u32(this + PAGE_SIZE, 0x10000);
        this
    }

    #[test]
    fn empty_member_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00aa_7830, &args![0x1000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn constructor_builds_the_base_then_sets_up() {
        let mut e = Engine::new();
        e.register(ABSTRACT_HEAP_CONSTRUCT, no_op);
        e.register(ABSTRACT_HEAP_SET_UP, |e, a| {
            assert_eq!(e.mem.u32(a[0]), MEMORY_HEAP_VTABLE);
            Ret::default()
        });
        let this = heap(&mut e);
        e.mem.set_u8(this + DESTROYED, 7);
        e.call_log = Some(vec![]);
        let back = e.call(
            0x00aa_78a0,
            &args![this, 0x0c80_0000u32, 0x0550_0000u32, 0x0108_24fcu32, 1u32],
        );
        assert_eq!(back.u32(), this);
        assert_eq!(e.mem.u8(this + OWN_FLAG), 1);
        assert_eq!(e.mem.u8(this + DESTROYED), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ABSTRACT_HEAP_CONSTRUCT),
            vec![vec![this, 0x0c80_0000, 0x0550_0000, 0x0108_24fc, 0x10000]]
        );
        assert_eq!(calls(&log, ABSTRACT_HEAP_SET_UP), vec![vec![this]]);
    }

    #[test]
    fn destructor_tears_down_then_destroys_the_base() {
        let mut e = Engine::new();
        e.register(ABSTRACT_HEAP_TEAR_DOWN, no_op);
        e.register(ABSTRACT_HEAP_DESTRUCT, |e, a| {
            assert_eq!(e.mem.u8(a[0] + DESTROYED), 1);
            Ret::default()
        });
        let this = heap(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00aa_7930, &args![this]);
        assert_eq!(e.mem.u32(this), MEMORY_HEAP_VTABLE);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(order, vec![ABSTRACT_HEAP_TEAR_DOWN, ABSTRACT_HEAP_DESTRUCT]);
    }

    #[test]
    fn reserve_and_commit_releases_on_a_failed_commit() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        // Both succeed.
        e.register(VIRTUAL_ALLOC, |_, a| {
            (if a[0] == 0 { 0x5000_0000u32 } else { a[0] }).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_79a0, &args![this, 0x1000_0000u32, 0x10_0000u32])
                .u32(),
            0x5000_0000
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_ALLOC),
            vec![
                vec![0, 0x1000_0000, MEM_RESERVE, PAGE_READWRITE],
                vec![0x5000_0000, 0x10_0000, MEM_COMMIT, PAGE_READWRITE]
            ]
        );
        assert!(calls(&log, VIRTUAL_FREE).is_empty());
        // The commit fails: released, null.
        e.register(VIRTUAL_ALLOC, |_, a| {
            (if a[0] == 0 { 0x5000_0000u32 } else { 0 }).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_79a0, &args![this, 1u32, 1u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_FREE),
            vec![vec![0x5000_0000, 0, MEM_RELEASE]]
        );
        // The reservation fails: null, no commit.
        e.register(VIRTUAL_ALLOC, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_79a0, &args![this, 1u32, 1u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, VIRTUAL_ALLOC).len(), 1);
    }

    #[test]
    fn release_frees_the_memory() {
        let mut e = Engine::new();
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_7a30, &args![0u32, 0x5000_0000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_FREE),
            vec![vec![0x5000_0000, 0, MEM_RELEASE]]
        );
    }

    #[test]
    fn commit_raises_by_a_page_and_rounds() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(VIRTUAL_ALLOC, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let size = e.call(
            0x00aa_7a50,
            &args![this, 0x5000_0000u32, 0x2_0000u32, 0x1234u32],
        );
        assert_eq!(size.u32(), 0x1_0000);
        // An exact page still gains one.
        let size = e.call(0x00aa_7a50, &args![this, 0x5000_0000u32, 0u32, 0x1_0000u32]);
        assert_eq!(size.u32(), 0x2_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_ALLOC)[0],
            vec![0x5002_0000, 0x1_0000, MEM_COMMIT, PAGE_READWRITE]
        );
        e.register(VIRTUAL_ALLOC, |_, _| 0u32.into_ret());
        let size = e.call(0x00aa_7a50, &args![this, 0x5000_0000u32, 0u32, 0x10u32]);
        assert_eq!(size.u32(), 0);
    }

    #[test]
    fn decommit_rounds_down_and_needs_a_whole_page() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let size = e.call(
            0x00aa_7ab0,
            &args![this, 0x5000_0000u32, 0x4_0000u32, 0x2_1234u32],
        );
        assert_eq!(size.u32(), 0x2_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, VIRTUAL_FREE),
            vec![vec![0x5002_0000, 0x2_0000, MEM_DECOMMIT]]
        );
        e.call_log = Some(vec![]);
        let size = e.call(
            0x00aa_7ab0,
            &args![this, 0x5000_0000u32, 0x4_0000u32, 0xffffu32],
        );
        assert_eq!(size.u32(), 0);
        assert!(calls(&e.call_log.take().unwrap(), VIRTUAL_FREE).is_empty());
        // VirtualFree answering anything but 1 gives 0.
        e.register(VIRTUAL_FREE, |_, _| 2u32.into_ret());
        let size = e.call(
            0x00aa_7ab0,
            &args![this, 0x5000_0000u32, 0x4_0000u32, 0x1_0000u32],
        );
        assert_eq!(size.u32(), 0);
    }

    #[test]
    fn allocate_and_free_forward_to_the_base() {
        let mut e = Engine::new();
        e.register(BASE_ALLOCATE, |_, a| (a[1] + 0x100).into_ret());
        e.register(BASE_FREE, no_op);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_7b20, &args![0x10u32, 0x20u32, 9u32]).u32(),
            0x120
        );
        e.call(0x00aa_7b40, &args![0x10u32, 0u32, 9u32]);
        e.call(0x00aa_7b40, &args![0x10u32, 0x3000u32, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BASE_ALLOCATE), vec![vec![0x10, 0x20]]);
        assert_eq!(calls(&log, BASE_FREE), vec![vec![0x10, 0x3000]]);
    }

    #[test]
    fn block_size_is_the_header_low_28_bits() {
        let mut e = Engine::new();
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 0xf000_0123);
        assert_eq!(
            e.call(0x00aa_7be0, &args![0u32, block + 4, 0u32]).u32(),
            0x123
        );
    }

    #[test]
    fn deleting_destructor_returns_the_block_on_bit_zero() {
        let mut e = Engine::new();
        e.register(ABSTRACT_HEAP_TEAR_DOWN, no_op);
        e.register(ABSTRACT_HEAP_DESTRUCT, no_op);
        e.register(MEMORY_MANAGER_DEALLOCATE, no_op);
        let this = heap(&mut e);
        for (flags, frees) in [(0u32, false), (1, true), (2, false)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00aa_7e20, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, ABSTRACT_HEAP_TEAR_DOWN), vec![vec![this]]);
            let freed = calls(&log, MEMORY_MANAGER_DEALLOCATE);
            if frees {
                assert_eq!(freed, vec![vec![MEMORY_MANAGER, this]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }

    #[test]
    fn pointer_in_heap_checks_the_range() {
        let mut e = Engine::new();
        let this = heap(&mut e);
        e.mem.set_u32(this + MEM_HEAP, 0x5000_0000);
        e.mem.set_u32(this + MEM_HEAP_SIZE, 0x1000);
        for (pointer, inside) in [
            (0x4fff_ffffu32, false),
            (0x5000_0000, true),
            (0x5000_0fff, true),
            (0x5000_1000, false),
        ] {
            assert_eq!(e.call(0x00aa_7e70, &args![this, pointer]).bool(), inside);
        }
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn i_memory_heap_destructors_restore_the_vtable() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let heap = e.mem.alloc(8);
        e.call(0x00aa_7810, &args![heap]);
        assert_eq!(e.mem.u32(heap), I_MEMORY_HEAP_VTABLE);
        e.mem.set_u32(heap, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_7840, &args![heap, 0u32]).u32(), heap);
        assert_eq!(e.mem.u32(heap), I_MEMORY_HEAP_VTABLE);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        e.call(0x00aa_7840, &args![heap, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, heap]]
        );
    }

    #[test]
    fn small_getters_and_constants() {
        let mut e = Engine::new();
        let heap = e.mem.alloc(0x10);
        e.mem.set_u32(heap + 4, 0x44);
        e.mem.set_u32(heap + 8, 0x88);
        assert_eq!(e.call(0x00aa_7870, &args![heap]).u32(), 0x88);
        assert_eq!(e.call(0x00aa_7e50, &args![heap]).u32(), 0x44);
        assert_eq!(e.call(0x00aa_7890, &args![heap, 5u32]).u32(), 0);
        assert_eq!(e.call(0x00aa_7c30, &args![heap]).u32(), 0);
        assert_eq!(e.call(0x00aa_7c50, &args![heap]).u32(), 0);
        let block = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(block - 4, 0xf000_0030);
        assert_eq!(e.call(0x00aa_7c00, &args![heap, block]).u32(), 0x38);
    }

    #[test]
    fn fn_00aa7b70_needs_the_heap_and_a_clear_bit_30() {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x10).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        e.register(0x8010, |_, a| {
            if a[1] >= 0x2000_0000 {
                0x101u32.into_ret()
            } else {
                0x100u32.into_ret()
            }
        });
        let heap = e.mem.alloc(8);
        e.mem.set_u32(heap, 0x7000);
        let block = e.mem.alloc(0x10) + 8;
        assert!(e.call(0x00aa_7b70, &args![heap, block]).bool());
        e.mem.set_u32(block - 4, 0x4000_0000);
        assert!(!e.call(0x00aa_7b70, &args![heap, block]).bool());
        // Outside the heap the header is not read.
        assert!(!e.call(0x00aa_7b70, &args![heap, 0x1000u32]).bool());
    }

    #[test]
    fn stats_and_output_go_to_the_base_unless_flagged() {
        let mut e = Engine::new();
        e.register(ABSTRACT_HEAP_STATS, |_, _| 0x51u32.into_ret());
        e.register(ABSTRACT_HEAP_BUCKETS, |_, _| 0x52u32.into_ret());
        e.register(ABSTRACT_HEAP_BLOCKS, |_, _| 0x53u32.into_ret());
        let slots: Vec<u32> = (0..0x10).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        e.register(0x8008, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            0x54u32.into_ret()
        });
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, 0x7000);
        let heap = e.mem.alloc(0x2104);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_7c60, &args![heap, 0x60u32, 0x101u32]).u32(),
            0x51
        );
        assert_eq!(e.call(0x00aa_7ca0, &args![heap, stream]).u32(), 0x52);
        assert_eq!(
            e.call(0x00aa_7d00, &args![heap, stream, 0x101u32]).u32(),
            0x53
        );
        assert_eq!(calls_to(&e, ABSTRACT_HEAP_STATS), vec![vec![heap, 0x60, 1]]);
        assert_eq!(
            calls_to(&e, ABSTRACT_HEAP_BUCKETS),
            vec![vec![heap, stream]]
        );
        assert_eq!(
            calls_to(&e, ABSTRACT_HEAP_BLOCKS),
            vec![vec![heap, stream, 1]]
        );
        // With the byte at +0x20f0 set: the flag is dropped, and the outputs write a zero word.
        e.mem.set_u8(heap + OWN_FLAG, 1);
        e.call_log = Some(vec![]);
        e.call(0x00aa_7c60, &args![heap, 0x60u32, 1u32]);
        assert_eq!(calls_to(&e, ABSTRACT_HEAP_STATS), vec![vec![heap, 0x60, 0]]);
        assert_eq!(e.call(0x00aa_7ca0, &args![heap, stream]).u32(), 0x54);
        assert_eq!(e.call(0x00aa_7d00, &args![heap, stream, 1u32]).u32(), 0x54);
        let writes = calls_to(&e, 0x8008);
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0][0], stream);
        assert_eq!(writes[0][2], 4);
        assert!(calls_to(&e, ABSTRACT_HEAP_BUCKETS).is_empty());
    }

    #[test]
    fn fn_00aa7d60_decommits_whole_pages_inside_a_block() {
        let mut e = Engine::new();
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        let heap = e.mem.alloc(0x40);
        e.mem.set_u32(heap + PAGE_SIZE, 0x1000);
        let header = e.mem.alloc(8);
        // Data at header + 8; pages from the next 0x1000 boundary.
        let data = header + 8;
        let start = (data + 0xfff) & !0xfff;
        let size = (start - data) + 0x2800;
        e.mem.set_u32(header + 4, 0x4000_0000 | size);
        e.call_log = Some(vec![]);
        e.call(0x00aa_7d60, &args![heap, header]);
        assert_eq!(
            calls_to(&e, VIRTUAL_FREE),
            vec![vec![start, 0x2000, MEM_DECOMMIT]]
        );
        assert_eq!(e.mem.u32(header + 4), 0x6000_0000 | size);
        // Less than a page inside, or a small block: nothing.
        for small in [(start - data) + 0xfff, 0xfff] {
            e.mem.set_u32(header + 4, small);
            e.call_log = Some(vec![]);
            e.call(0x00aa_7d60, &args![heap, header]);
            assert!(calls_to(&e, VIRTUAL_FREE).is_empty());
            assert_eq!(e.mem.u32(header + 4), small);
        }
    }
}
