//! `Havok SDK/hkfreelistmemorysystem.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The memory system object is a large block of sub-objects (the Xbox PDB has no
//! entry for `hkFreeListMemorySystem`, so its layout is described by offsets
//! only, each named where the constructor uses it).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkFreeListMemorySystem`'s virtual table.
const VTABLE: u32 = 0x010c_ad74;
/// The virtual table stored at `+0xab8`.
const VTABLE_AT_0AB8: u32 = 0x010c_ad3c;
/// The word `00cd0a60` stores `this + 0x1c` in.
const SYSTEM_ALLOCATOR_GLOBAL: u32 = 0x0126_86cc;

/// The sub-object at `+8` (constructor `00d60500`: two stack words).
const SUB_OBJECT_8: u32 = 0x08;
/// The sub-object at `+0x1c` (constructor `00d60270`: one stack word, the
/// address of the sub-object at `+8`).
const SUB_OBJECT_1C: u32 = 0x1c;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, one stack word) at `+0xad0`
/// and at `+0x336c`.
const CRITICAL_SECTION_A: u32 = 0xad0;
const CRITICAL_SECTION_B: u32 = 0x336c;
/// `hkMemorySystem::FrameInfo` at `+0xb00` (`00cbbfa0`).
const FRAME_INFO: u32 = 0xb00;
/// `hkMemoryRouter::hkMemoryRouter` at `+0xb0c` (`00c85770`).
const MEMORY_ROUTER: u32 = 0xb0c;
/// `hkSolverAllocator::hkSolverAllocator` at `+0xb30` (`00d5e900`).
const SOLVER_ALLOCATOR: u32 = 0xb30;
/// The first of 32 `hkThreadMemory` records (`00d2c720`) and the distance between
/// them.
const THREAD_MEMORIES: u32 = 0xc6c;
const THREAD_MEMORY_SIZE: u32 = 0x138;
const THREAD_MEMORY_COUNT: u32 = 32;

// Translated from 00cd06c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word): stores its argument in
/// the words at `+4` and `+0x18` of the memory system (`+0x18` is the word at
/// `+0x10` of the sub-object at `+8`). The Havok start-up calls it with the
/// address of the allocator it has built (`0126 7b54`) right after the
/// constructor.
pub fn fn_00cd06c0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 4, value);
    e.mem.set_u32(this.addr() + 0x18, value);
}

// Translated from 00cd0a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::hkFreeListMemorySystem` (Xbox PDB): stores the
/// virtual table, constructs the sub-objects (`00d60500` and `00d60270`, two
/// critical sections, the frame info, the memory router, the solver allocator and
/// 32 thread memories, each with their last three words cleared), then points the
/// global at `012686cc` and the word at `+0xb08` at `this + 0x1c`. Returns `this`.
pub fn hk_free_list_memory_system_hk_free_list_memory_system(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.call(0x00d6_0500, &args![base + SUB_OBJECT_8, 0u32, 0u32]);
    e.call(
        0x00d6_0270,
        &args![base + SUB_OBJECT_1C, base + SUB_OBJECT_8],
    );
    e.mem.set_u32(base + 0xab8, VTABLE_AT_0AB8);
    e.mem.set_u32(base + 0xac0, 0);
    e.mem.set_u32(base + 0xac4, 0);
    e.mem.set_u32(base + 0xac8, 0x8000_0000);
    e.mem.set_u32(base + 0xacc, 0);
    e.call(0x00cf_6da0, &args![base + CRITICAL_SECTION_A, 0u32]);
    e.call(0x00cb_bfa0, &args![base + FRAME_INFO]);
    e.call(0x00c8_5770, &args![base + MEMORY_ROUTER]);
    e.call(0x00d5_e900, &args![base + SOLVER_ALLOCATOR]);
    for index in 0..THREAD_MEMORY_COUNT {
        let record = base + THREAD_MEMORIES + index * THREAD_MEMORY_SIZE;
        e.call(0x00d2_c720, &args![record]);
        e.mem.set_u32(record + 0x128, 0);
        e.mem.set_u8(record + 0x12c, 0);
        e.mem.set_u32(record + 0x130, 0);
    }
    e.call(0x00cf_6da0, &args![base + CRITICAL_SECTION_B, 0u32]);
    e.set_global(SYSTEM_ALLOCATOR_GLOBAL, base + SUB_OBJECT_1C);
    e.mem.set_u32(base + 0xb08, base + SUB_OBJECT_1C);
    this
}

// Translated from 00d60500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the sub-object at `+8` (unnamed in the engine map;
/// `thiscall`, `RET 8`: `first`, `second`). Its virtual table (`010d14c8`) is
/// `hkSystemMemoryBlockServer`'s (confirmed by the RTTI record before it), so
/// that is the class. Stores the virtual table, `first` at `+0x10`, clears `+8`
/// and `+0xc`, and stores `second` at `+4` (`0x20000` when it is 0). Returns `this`.
pub fn fn_00d60500(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x10, first);
    e.mem.set_u32(base, 0x010d_14c8);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem
        .set_u32(base + 4, if second == 0 { 0x2_0000 } else { second });
    this
}

/// `EnterCriticalSection` and `LeaveCriticalSection` (`kernel32` import slots,
/// `stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkThreadMemory`'s two-word setter (`00d2c3a0`, `thiscall`) and
/// `hkSolverAllocator::setBuffer` (`00d5e960`, `thiscall`: buffer, size).
const THREAD_MEMORY_SETTER: u32 = 0x00d2_c3a0;
const SOLVER_ALLOCATOR_SET_BUFFER: u32 = 0x00d5_e960;
/// The address `InitSystem` passes as the second word of the router set-up
/// (`01094d90`; the exe's own data, not described here).
const ROUTER_NAME_WORD: u32 = 0x0109_4d90;

// Translated from 00cd0740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::mainInit` (Xbox PDB), `thiscall`, `RET 8`: `info`
/// points at two words (the frame information `00cbbfa0` constructs), copied to
/// `+0xb00` and `+0xb04`. When `flags & 1`, calls the virtual slot `+0xc` of
/// the system (`threadInit`, `00cd0830`) with the router at `+0xb0c`, the word
/// `01094d90` and `flags`. When `flags & 2` and the second word of `info` is not
/// 0, takes that many bytes from the allocator at `+4` (its slot `+4`, called with
/// the count) and hands them to the solver allocator at `+0xb30` (`00d5e960`:
/// buffer, count). Returns the router, `this + 0xb0c`.
pub fn hk_free_list_memory_system_main_init(
    e: &mut Engine,
    this: Ptr,
    info: Ptr,
    flags: u32,
) -> u32 {
    let base = this.addr();
    let first = e.mem.u32(info.addr());
    e.mem.set_u32(base + FRAME_INFO, first);
    let second = e.mem.u32(info.addr() + 4);
    e.mem.set_u32(base + FRAME_INFO + 4, second);
    if flags & 1 != 0 {
        e.vcall(
            base,
            0xc,
            &args![base + MEMORY_ROUTER, ROUTER_NAME_WORD, flags],
        );
    }
    if flags & 2 != 0 {
        let size = e.mem.u32(info.addr() + 4);
        if size != 0 {
            let allocator = e.mem.u32(base + 4);
            let buffer = e.vcall(allocator, 4, &args![size]).u32();
            e.call(
                SOLVER_ALLOCATOR_SET_BUFFER,
                &args![base + SOLVER_ALLOCATOR, buffer, size],
            );
        }
    }
    base + MEMORY_ROUTER
}

// Translated from 00cd0830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::threadInit` (Xbox PDB), `thiscall`, `RET 0xc`: the
/// virtual slot `+0xc`. When `flags & 1`, takes the first free `hkThreadMemory`
/// record (the byte at `+0x12c` of the records at `+0xc6c`, 0x138 apart) inside
/// the critical section at `+0x336c` and marks it used; when all 32 are used the
/// exe goes on with index 32 (the bytes after the table) and so does this. Stores
/// `name` at `+0x128` of the record, calls `00d2c3a0` on it with the word at
/// `+0xb08` and 8, and fills `router`: `+0`, `+4`, `+8` and `+0xc`, `+0x18` cleared,
/// `+0x10` the record, `+0x1c` the word at `+0xb08`, `+0x14` the word at `+4`
/// and `+0x20` the record index. When `flags & 2`, with the router's index (`+0x20`,
/// read from the router, which only the `flags & 1` path sets) and the byte count
/// at `+0xb00` not 0, allocates that many bytes from the allocator at `+0x1c`
/// (its slot `+4`) into `router` `+4`, `+0` and `+8` (end), records the count and the
/// router in the record (`+0x130`, `+0x134`) and sets `router + 0xc` to the record
/// and `router + 0x18` to the solver allocator at `+0xb30`.
pub fn hk_free_list_memory_system_thread_init(
    e: &mut Engine,
    this: Ptr,
    router: u32,
    name: u32,
    flags: u32,
) {
    let base = this.addr();
    let mut index = 0u32;
    if flags & 1 != 0 {
        e.call(ENTER_CRITICAL_SECTION, &args![base + CRITICAL_SECTION_B]);
        let mut flag = base + THREAD_MEMORIES + 0x12c;
        while e.mem.u8(flag) != 0 {
            index += 1;
            flag += THREAD_MEMORY_SIZE;
            if index >= THREAD_MEMORY_COUNT {
                break;
            }
        }
        if index < THREAD_MEMORY_COUNT {
            e.mem.set_u8(flag, 1);
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![base + CRITICAL_SECTION_B]);
        let record = base + THREAD_MEMORIES + index * THREAD_MEMORY_SIZE;
        e.mem.set_u32(record + 0x128, name);
        let word = e.mem.u32(base + 0xb08);
        e.call(THREAD_MEMORY_SETTER, &args![record, word, 8u32]);
        e.mem.set_u32(router + 0xc, 0);
        e.mem.set_u32(router + 0x10, record);
        e.mem.set_u32(router + 0x1c, word);
        let allocator = e.mem.u32(base + 4);
        e.mem.set_u32(router + 0x14, allocator);
        e.mem.set_u32(router + 0x18, 0);
        e.mem.set_u32(router + 0x20, index);
        e.mem.set_u32(router + 4, 0);
        e.mem.set_u32(router, 0);
        e.mem.set_u32(router + 8, 0);
    }
    if flags & 2 != 0 {
        let record_index = e.mem.u32(router + 0x20);
        let size = e.mem.u32(base + 0xb00);
        let record = base + THREAD_MEMORIES + record_index.wrapping_mul(THREAD_MEMORY_SIZE);
        if size != 0 {
            let buffer = e.vcall(base + SUB_OBJECT_1C, 4, &args![size]).u32();
            e.mem.set_u32(router + 4, buffer);
            e.mem.set_u32(router, buffer);
            e.mem.set_u32(router + 8, buffer.wrapping_add(size));
            e.mem.set_u32(record + 0x130, size);
            e.mem.set_u32(record + 0x134, router);
        }
        e.mem.set_u32(router + 0xc, record);
        e.mem.set_u32(router + 0x18, base + SOLVER_ALLOCATOR);
    }
}

/// `hkThreadMemory`'s virtual table (`010cacd0`; slot 0 is `00cd06a0`) and the memory
/// block server's (`010cad0c`).
const THREAD_MEMORY_VTABLE: u32 = 0x010c_acd0;
const BLOCK_SERVER_VTABLE: u32 = 0x010c_ad0c;
/// Callees outside this file: `hkThreadMemory::releaseCachedMemory` (`00d2c7a0`) and
/// `getCachedSizeUnchecked` (`00d2c350`), both `thiscall` with no word; the base
/// allocator destructor (`00cbf1a0`); `DeleteCriticalSection` (`kernel32` import slot,
/// `stdcall`, one word); the solver allocator's destructor (`00d5e940`); the free list
/// allocator's destructor (`00d5ffd0`); `hkMemorySystem`'s destructor (`00cbbfe0`);
/// `hkMemUtil::memSet` (`00d2cab0`, cdecl: destination, value, count).
const THREAD_MEMORY_RELEASE_CACHED: u32 = 0x00d2_c7a0;
const THREAD_MEMORY_CACHED_SIZE: u32 = 0x00d2_c350;
const BASE_ALLOCATOR_DESTRUCTOR: u32 = 0x00cb_f1a0;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const SOLVER_ALLOCATOR_DESTRUCTOR: u32 = 0x00d5_e940;
const FREE_LIST_ALLOCATOR_DESTRUCTOR: u32 = 0x00d5_ffd0;
const MEMORY_SYSTEM_BASE_DESTRUCTOR: u32 = 0x00cb_bfe0;
const MEM_SET: u32 = 0x00d2_cab0;
/// The stream's text writer (`00cc01c0`, `thiscall` on the stream, one word) and the two
/// texts `printStatistics` writes (`010cade0`, `010cadb4`); `hkMemorySystem`'s own
/// `_printStatistics` (`00cbc220`, `thiscall`: stream, input); the memory router getter
/// (`00c85750`, answers the router whose word at `+0x14` is the allocator) and the array
/// growth (`00c90940`, cdecl: allocator, array, element size).
const STREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const STATISTICS_TEXT_FIRST: u32 = 0x010c_ade0;
const STATISTICS_TEXT_SECOND: u32 = 0x010c_adb4;
const MEMORY_SYSTEM_PRINT: u32 = 0x00cb_c220;
const ROUTER_GETTER: u32 = 0x00c8_5750;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The bit an `hkArray` capacity word carries when the array does not own its memory.
const ARRAY_NOT_OWNED: u32 = 0x8000_0000;
const ARRAY_CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00cd06a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::_scalar_deleting_destructor_` (Xbox PDB, from the engine map's vtable
/// entry), `thiscall`, `RET 4`: stores the thread memory's virtual table, releases the
/// cached memory (`00d2c7a0`) and runs the base destructor (`00cbf1a0`). The flags word is
/// never read and nothing is freed here. Returns `this`.
pub fn hk_thread_memory_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), THREAD_MEMORY_VTABLE);
    e.call(THREAD_MEMORY_RELEASE_CACHED, &args![this]);
    e.call(BASE_ALLOCATOR_DESTRUCTOR, &args![this]);
    this
}

// Translated from 00cd06d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::walkMemory` (Xbox PDB), `thiscall`, tail call: the free list
/// allocator at `+0x1c` runs its virtual slot `0x5c` (`walkMemory`, `00d600c0`) with the
/// same two words (`callback`, `user`) and answers what it answers.
pub fn hk_free_list_memory_system_walk_memory(
    e: &mut Engine,
    this: Ptr,
    callback: u32,
    user: u32,
) -> u32 {
    e.vcall(this.addr() + SUB_OBJECT_1C, 0x5c, &args![callback, user])
        .u32()
}

// Translated from 00cd06e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::garbageCollect` (Xbox PDB), `thiscall`, tail call: the free
/// list allocator at `+0x1c` runs its virtual slot `0x40` (`garbageCollect`, `00d5fac0`).
pub fn hk_free_list_memory_system_garbage_collect(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr() + SUB_OBJECT_1C, 0x40, &args![]);
}

// Translated from 00cd06f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::getMemoryStatistics` (Xbox PDB), `thiscall`, `RET 4`: for each
/// of the 32 thread memory records (`+0xc6c`, 0x138 apart) whose used byte (`+0x12c`) is
/// set, adds the record's cached size (`00d2c350`) to the word at `+4` of `stats` and the
/// record's word at `+0x130` to the word at `+0`. Ends by clearing the word at `+8`.
pub fn hk_free_list_memory_system_get_memory_statistics(e: &mut Engine, this: Ptr, stats: Ptr) {
    let base = this.addr();
    let stats = stats.addr();
    for index in 0..THREAD_MEMORY_COUNT {
        let record = base + THREAD_MEMORIES + index * THREAD_MEMORY_SIZE;
        if e.mem.u8(record + 0x12c) != 0 {
            let cached = e.call(THREAD_MEMORY_CACHED_SIZE, &args![record]).u32();
            let second = e.mem.u32(stats + 4).wrapping_add(cached);
            e.mem.set_u32(stats + 4, second);
            let first = e.mem.u32(stats).wrapping_add(e.mem.u32(record + 0x130));
            e.mem.set_u32(stats, first);
        }
    }
    e.mem.set_u32(stats + 8, 0);
}

// Translated from 00cd0730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::getMemorySnapshot` (Xbox PDB), `thiscall`, `RET 4`: answers 1;
/// its argument is never read.
pub fn hk_free_list_memory_system_get_memory_snapshot(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
) -> u32 {
    1
}

// Translated from 00cd07b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::mainQuit` (Xbox PDB), `thiscall`, `RET 4` (`flags`): when
/// `flags & 2` and the solver buffer (`+0xb34` to `+0xb38`) is not empty, gives it back
/// to the allocator at `+4` (its slot 8: buffer, size) and resets the solver allocator
/// at `+0xb30` (`00d5e960`: 0, 0). When `flags & 1`, runs the virtual slot `0x10`
/// (`threadQuit`, `00cd0940`) with the router at `+0xb0c` and `flags`, then the free list
/// allocator's slot `0x40` (`garbageCollect`). Always clears the word at `+4`.
pub fn hk_free_list_memory_system_main_quit(e: &mut Engine, this: Ptr, flags: u32) {
    let base = this.addr();
    if flags & 2 != 0 {
        let size = e
            .mem
            .u32(base + SOLVER_ALLOCATOR + 8)
            .wrapping_sub(e.mem.u32(base + SOLVER_ALLOCATOR + 4));
        if size != 0 {
            let allocator = e.mem.u32(base + 4);
            let buffer = e.mem.u32(base + SOLVER_ALLOCATOR + 4);
            e.vcall(allocator, 8, &args![buffer, size]);
            e.call(
                SOLVER_ALLOCATOR_SET_BUFFER,
                &args![base + SOLVER_ALLOCATOR, 0u32, 0u32],
            );
        }
    }
    if flags & 1 != 0 {
        e.vcall(base, 0x10, &args![base + MEMORY_ROUTER, flags]);
        e.vcall(base + SUB_OBJECT_1C, 0x40, &args![]);
    }
    e.mem.set_u32(base + 4, 0);
}

// Translated from 00cd0940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::threadQuit` (Xbox PDB), `thiscall`, `RET 8` (`router`,
/// `flags`): the thread memory record is the one at index `router + 0x20` of the records
/// at `+0xc6c`. When `flags & 2`: if the record's word at `+0x130` is not 0 and the
/// router's buffer (`+4` to `+8`) is not empty, gives the buffer back to the free list
/// allocator at `+0x1c` (slot 8: buffer, size) and clears the router's `+0`, `+4` and
/// `+8`; then releases the record's cached memory (its slot `0x34`) and clears the
/// router's `+0xc` and `+0x18`. When `flags & 1`: releases the cached memory again,
/// clears the record's `+0x128` and used byte `+0x12c` and zeroes the router's 0x24
/// bytes (`00d2cab0`).
pub fn hk_free_list_memory_system_thread_quit(e: &mut Engine, this: Ptr, router: u32, flags: u32) {
    let base = this.addr();
    let record = e
        .mem
        .u32(router + 0x20)
        .wrapping_mul(THREAD_MEMORY_SIZE)
        .wrapping_add(base)
        .wrapping_add(THREAD_MEMORIES);
    if flags & 2 != 0 {
        if e.mem.u32(record + 0x130) != 0 {
            let start = e.mem.u32(router + 4);
            let size = e.mem.u32(router + 8).wrapping_sub(start);
            if size != 0 {
                e.vcall(base + SUB_OBJECT_1C, 8, &args![start, size]);
                e.mem.set_u32(router + 4, 0);
                e.mem.set_u32(router, 0);
                e.mem.set_u32(router + 8, 0);
            }
        }
        e.vcall(record, 0x34, &args![]);
        e.mem.set_u32(router + 0xc, 0);
        e.mem.set_u32(router + 0x18, 0);
    }
    if flags & 1 != 0 {
        e.vcall(record, 0x34, &args![]);
        e.mem.set_u32(record + 0x128, 0);
        e.mem.set_u8(record + 0x12c, 0);
        e.call(MEM_SET, &args![router, 0u32, 0x24u32]);
    }
}

// Translated from 00cd09d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::~hkFreeListMemorySystem` (Xbox PDB), `thiscall`: stores the
/// virtual table, clears the global at `012686cc`, deletes the critical section at
/// `+0x336c`, destroys the 32 thread memory records from the last to the first (store
/// the thread memory virtual table, `00d2c7a0`, `00cbf1a0`), destroys the solver
/// allocator (`+0xb30`, `00d5e940`), deletes the critical section at `+0xad0` and runs
/// the base destructor on the sub-object at `+0xab8` (`00cbf1a0`), destroys the free
/// list allocator at `+0x1c` (`00d5ffd0`), stores the block server's virtual table at
/// `+8` and tail-calls `hkMemorySystem`'s destructor (`00cbbfe0`).
pub fn hk_free_list_memory_system_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.set_global(SYSTEM_ALLOCATOR_GLOBAL, 0u32);
    e.call(DELETE_CRITICAL_SECTION, &args![base + CRITICAL_SECTION_B]);
    let mut record = base + CRITICAL_SECTION_B;
    let mut remaining = 0x1fi32;
    loop {
        record = record.wrapping_sub(THREAD_MEMORY_SIZE);
        e.mem.set_u32(record, THREAD_MEMORY_VTABLE);
        e.call(THREAD_MEMORY_RELEASE_CACHED, &args![record]);
        e.call(BASE_ALLOCATOR_DESTRUCTOR, &args![record]);
        remaining -= 1;
        if remaining < 0 {
            break;
        }
    }
    e.call(SOLVER_ALLOCATOR_DESTRUCTOR, &args![base + SOLVER_ALLOCATOR]);
    e.call(DELETE_CRITICAL_SECTION, &args![base + 0xab8 + 0x18]);
    e.call(BASE_ALLOCATOR_DESTRUCTOR, &args![base + 0xab8]);
    e.call(FREE_LIST_ALLOCATOR_DESTRUCTOR, &args![base + SUB_OBJECT_1C]);
    e.mem.set_u32(base + SUB_OBJECT_8, BLOCK_SERVER_VTABLE);
    e.call(MEMORY_SYSTEM_BASE_DESTRUCTOR, &args![base]);
}

// Translated from 00cd0b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::printStatistics` (Xbox PDB), `thiscall`, `RET 4` (`stream`):
/// writes the two texts (`010cade0`, `010cadb4`) to the stream (`00cc01c0`), then builds
/// the `hkMemorySystem::_PrintStatisticsInput` in a local of 0x24 bytes: the word at
/// `+0xb08` first, two empty `hkArray`s (at `+4` and `+0x10`; capacity word
/// `0x80000000`) and, in the last word, the free list allocator's soft limit (its slot
/// `0x50`, called on the sub-object at `+0x1c`). For each used thread memory record (32,
/// `+0xc6c`, 0x138 apart; used byte `+0x12c`) the record's address is appended to the
/// first array and the record's word at `+0x134` to the second, growing an array with
/// `00c90940` through the memory router's allocator (`00c85750`, `+0x14`; element size 4)
/// when it is full. Then `hkMemorySystem::_printStatistics` (`00cbc220`) prints the
/// statistics and the input's destructor (`00cd0c80`) frees the arrays.
pub fn hk_free_list_memory_system_print_statistics(e: &mut Engine, this: Ptr, stream: u32) {
    let base = this.addr();
    e.call(STREAM_WRITE_TEXT, &args![stream, STATISTICS_TEXT_FIRST]);
    e.call(STREAM_WRITE_TEXT, &args![stream, STATISTICS_TEXT_SECOND]);
    e.with_stack(0x24, |e, input| {
        let at = input.addr();
        e.mem.set_u32(at + 0xc, ARRAY_NOT_OWNED);
        e.mem.set_u32(at + 0x18, ARRAY_NOT_OWNED);
        for offset in [0x4, 0x8, 0x10, 0x14, 0x1c, 0x20] {
            e.mem.set_u32(at + offset, 0);
        }
        let limit = e.vcall(base + SUB_OBJECT_1C, 0x50, &args![]).u32();
        e.mem.set_u32(at + 0x20, limit);
        let word = e.mem.u32(base + 0xb08);
        e.mem.set_u32(at, word);
        for index in 0..THREAD_MEMORY_COUNT {
            let record = base + THREAD_MEMORIES + index * THREAD_MEMORY_SIZE;
            if e.mem.u8(record + 0x12c) == 0 {
                continue;
            }
            for (array, value) in [(at + 4, record), (at + 0x10, e.mem.u32(record + 0x134))] {
                let router = e.call(ROUTER_GETTER, &args![]).u32();
                let capacity = e.mem.u32(array + 8) & ARRAY_CAPACITY_MASK;
                if e.mem.u32(array + 4) == capacity {
                    let allocator = e.mem.u32(router + 0x14);
                    e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
                }
                let data = e.mem.u32(array);
                let size = e.mem.u32(array + 4);
                e.mem
                    .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
                e.mem.set_u32(array + 4, size.wrapping_add(1));
            }
        }
        // 00cd0c37: `00cbc220` is stdcall (RET 8) and does not read the ECX (this) the
        // caller sets.
        e.call(MEMORY_SYSTEM_PRINT, &args![stream, input]);
        hk_memory_system_print_statistics_input_destructor(e, input);
    });
}

// Translated from 00cd0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::_scalar_deleting_destructor_` (Xbox PDB, the engine map's
/// vtable entry), `thiscall`, `RET 4`: deletes the critical section at `+0x18` and runs
/// the base destructor (`00cbf1a0`). The flags word is never read and nothing is freed
/// here. Returns `this`.
pub fn hk_leak_detect_allocator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    e.call(DELETE_CRITICAL_SECTION, &args![this.addr() + 0x18]);
    e.call(BASE_ALLOCATOR_DESTRUCTOR, &args![this]);
    this
}

// Translated from 00cd0c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListMemorySystem::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`,
/// `RET 4`: runs the destructor (`00cd09d0`) and answers `this`; the flags word is never
/// read and nothing is freed here.
pub fn hk_free_list_memory_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    hk_free_list_memory_system_destructor(e, this);
    this
}

// Translated from 00cd0c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::_PrintStatisticsInput::~_PrintStatisticsInput` (Xbox PDB), `thiscall`:
/// the two `hkArray`s of the input (the one at `+0x10`, then the one at `+4`; data, size,
/// capacity-and-flag) are emptied: the size is cleared and, unless the capacity word has
/// the not-owned bit, the data is freed through the memory router's allocator (`00c85750`,
/// `+0x14`, slot `0x10`: data, capacity times 4 bytes); then the data is cleared and the
/// capacity word set to `0x80000000`.
pub fn hk_memory_system_print_statistics_input_destructor(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for array in [at + 0x10, at + 4] {
        let router = e.call(ROUTER_GETTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x14);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let bytes = (capacity & ARRAY_CAPACITY_MASK).wrapping_mul(4);
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, ARRAY_NOT_OWNED);
    }
}

// Translated from 00d60540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryBlockServer::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4`:
/// stores the block server's virtual table (`010cad0c`) and answers `this`; the flags
/// word is never read and nothing is freed here.
pub fn hk_memory_block_server_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), BLOCK_SERVER_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd06c0, fn_00cd06c0(Ptr, u32)),
        entry!(0x00d60500, fn_00d60500(Ptr, u32, u32) -> Ptr),
        entry!(0x00cd0740, hk_free_list_memory_system_main_init(Ptr, Ptr, u32) -> u32),
        entry!(
            0x00cd0830,
            hk_free_list_memory_system_thread_init(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00cd0a60,
            hk_free_list_memory_system_hk_free_list_memory_system(Ptr) -> Ptr
        ),
        entry!(0x00cd06a0, hk_thread_memory_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cd06d0, hk_free_list_memory_system_walk_memory(Ptr, u32, u32) -> u32),
        entry!(0x00cd06e0, hk_free_list_memory_system_garbage_collect(Ptr)),
        entry!(
            0x00cd06f0,
            hk_free_list_memory_system_get_memory_statistics(Ptr, Ptr)
        ),
        entry!(0x00cd0730, hk_free_list_memory_system_get_memory_snapshot(Ptr, u32) -> u32),
        entry!(0x00cd07b0, hk_free_list_memory_system_main_quit(Ptr, u32)),
        entry!(
            0x00cd0940,
            hk_free_list_memory_system_thread_quit(Ptr, u32, u32)
        ),
        entry!(0x00cd09d0, hk_free_list_memory_system_destructor(Ptr)),
        entry!(
            0x00cd0b30,
            hk_free_list_memory_system_print_statistics(Ptr, u32)
        ),
        entry!(0x00cd0c50, hk_leak_detect_allocator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cd0c70, hk_free_list_memory_system_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00cd0c80,
            hk_memory_system_print_statistics_input_destructor(Ptr)
        ),
        entry!(0x00d60540, hk_memory_block_server_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u32 = 0x3400;

    #[test]
    fn the_setter_writes_both_words() {
        let mut e = Engine::new();
        let system = e.mem.alloc(0x40);
        e.call(0x00cd_06c0, &args![system, 0x0126_7b54u32]);
        assert_eq!(e.mem.u32(system + 4), 0x0126_7b54);
        assert_eq!(e.mem.u32(system + 0x18), 0x0126_7b54);
        assert_eq!(e.mem.u32(system), 0);
    }

    #[test]
    fn the_block_server_constructor_defaults_its_size() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.mem.write(object, &[0xcd; 0x20]);
        assert_eq!(
            e.call(0x00d6_0500, &args![object, 0x55u32, 0u32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), 0x010d_14c8);
        assert_eq!(e.mem.u32(object + 4), 0x2_0000);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x10), 0x55);
        e.call(0x00d6_0500, &args![object, 0x66u32, 0x4000u32]);
        assert_eq!(e.mem.u32(object + 4), 0x4000);
        assert_eq!(e.mem.u32(object + 0x10), 0x66);
        assert_eq!(e.mem.u32(object + 0x14), 0xcdcd_cdcd);
    }

    fn main_init_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(THREAD_MEMORY_SETTER, |_, _| Ret::default());
        e.register(SOLVER_ALLOCATOR_SET_BUFFER, |_, _| Ret::default());
        // The system's vtable: slot 0xc is thread_init (real).
        e.put_vtable(0x4400_0000, &[0, 0, 0, 0x00cd_0830]);
        let system = e.mem.alloc(0x3400);
        e.mem.write(system, &vec![0; 0x3400]);
        e.mem.set_u32(system, 0x4400_0000);
        (e, system)
    }

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn main_init_with_the_router_flag_runs_thread_init_and_answers_the_router() {
        let (mut e, system) = main_init_engine();
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 0x11);
        e.mem.set_u32(info + 4, 0x2000);
        e.mem.set_u32(system + 0xb08, system + 0x1c);
        e.mem.set_u32(system + 4, 0x0126_7b54);
        // Record 0 is in use, so the router gets record 1.
        e.mem.set_u8(system + 0xd98, 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x00cd_0740, &args![system, info, 1u32]);
        assert_eq!(result.u32(), system + 0xb0c);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(system + 0xb00), 0x11);
        assert_eq!(e.mem.u32(system + 0xb04), 0x2000);
        let router = system + 0xb0c;
        assert_eq!(e.mem.u32(router + 0x20), 1);
        let record = system + 0xc6c + 0x138;
        assert_eq!(e.mem.u32(router + 0x10), record);
        assert_eq!(e.mem.u32(router + 0x14), 0x0126_7b54);
        assert_eq!(e.mem.u32(router + 0x1c), system + 0x1c);
        assert_eq!(e.mem.u8(system + 0xd98 + 0x138), 1);
        assert_eq!(e.mem.u32(record + 0x128), 0x0109_4d90);
        assert_eq!(
            calls_to(&log, THREAD_MEMORY_SETTER),
            vec![vec![record, system + 0x1c, 8]]
        );
        assert_eq!(
            calls_to(&log, 0x00cd_0830),
            vec![vec![system, router, 0x0109_4d90, 1]]
        );
        // No buffer without the second flag.
        assert!(calls_to(&log, SOLVER_ALLOCATOR_SET_BUFFER).is_empty());
    }

    #[test]
    fn main_init_with_the_buffer_flag_hands_the_solver_allocator_a_buffer() {
        let (mut e, system) = main_init_engine();
        // The allocator at +4: slot 4 returns a buffer.
        e.put_vtable(0x4410_0000, &[0, 0x4500_0000]);
        e.register(0x4500_0000, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, 0x4410_0000);
        e.mem.set_u32(system + 4, allocator);
        let info = e.mem.alloc(8);
        e.mem.set_u32(info + 4, 0x2000);
        e.call_log = Some(vec![]);
        e.call(0x00cd_0740, &args![system, info, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00cd_0830).is_empty());
        assert_eq!(calls_to(&log, 0x4500_0000), vec![vec![allocator, 0x2000]]);
        assert_eq!(
            calls_to(&log, SOLVER_ALLOCATOR_SET_BUFFER),
            vec![vec![system + 0xb30, 0x7000, 0x2000]]
        );
        // A zero count takes nothing.
        e.mem.set_u32(info + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cd_0740, &args![system, info, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn thread_init_with_both_flags_allocates_the_router_buffer() {
        let (mut e, system) = main_init_engine();
        e.put_vtable(0x4420_0000, &[0, 0x4500_0001]);
        e.register(0x4500_0001, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.mem.set_u32(system + 0x1c, 0x4420_0000);
        e.mem.set_u32(system + 0xb00, 0x100);
        let router = e.mem.alloc(0x24);
        e.call(0x00cd_0830, &args![system, router, 0x55u32, 3u32]);
        let record = system + 0xc6c;
        assert_eq!(e.mem.u32(router), 0x9000);
        assert_eq!(e.mem.u32(router + 4), 0x9000);
        assert_eq!(e.mem.u32(router + 8), 0x9100);
        assert_eq!(e.mem.u32(router + 0xc), record);
        assert_eq!(e.mem.u32(router + 0x18), system + 0xb30);
        assert_eq!(e.mem.u32(record + 0x130), 0x100);
        assert_eq!(e.mem.u32(record + 0x134), router);
        assert_eq!(e.mem.u32(record + 0x128), 0x55);
    }

    #[test]
    fn thread_init_when_every_record_is_used_goes_on_with_index_32() {
        let (mut e, system) = main_init_engine();
        for i in 0..32 {
            e.mem.set_u8(system + 0xd98 + i * 0x138, 1);
        }
        let router = e.mem.alloc(0x24);
        e.call(0x00cd_0830, &args![system, router, 0x55u32, 1u32]);
        assert_eq!(e.mem.u32(router + 0x20), 32);
        assert_eq!(e.mem.u32(router + 0x10), system + 0xc6c + 32 * 0x138);
    }

    fn constructor_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        for address in [
            0x00d6_0500,
            0x00d6_0270,
            0x00cf_6da0,
            0x00cb_bfa0,
            0x00c8_5770,
            0x00d5_e900,
            0x00d2_c720,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn the_constructor_builds_every_sub_object_in_order() {
        let mut e = constructor_engine();
        let system = e.mem.alloc(SIZE);
        e.mem.write(system, &vec![0xcd; SIZE as usize]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cd_0a60, &args![system]).u32(), system);
        let log = e.call_log.take().unwrap();
        let log = &log[1..];
        assert_eq!(log[0], (0x00d6_0500, vec![system + 8, 0, 0]));
        assert_eq!(log[1], (0x00d6_0270, vec![system + 0x1c, system + 8]));
        assert_eq!(log[2], (0x00cf_6da0, vec![system + 0xad0, 0]));
        assert_eq!(log[3], (0x00cb_bfa0, vec![system + 0xb00]));
        assert_eq!(log[4], (0x00c8_5770, vec![system + 0xb0c]));
        assert_eq!(log[5], (0x00d5_e900, vec![system + 0xb30]));
        let records: Vec<u32> = log[6..38].iter().map(|call| call.1[0]).collect();
        assert!(log[6..38].iter().all(|call| call.0 == 0x00d2_c720));
        let want: Vec<u32> = (0..32).map(|i| system + 0xc6c + i * 0x138).collect();
        assert_eq!(records, want);
        assert_eq!(log[38], (0x00cf_6da0, vec![system + 0x336c, 0]));
        assert_eq!(log.len(), 39);
    }

    #[test]
    fn the_constructor_sets_the_fields_and_the_global() {
        let mut e = constructor_engine();
        let system = e.mem.alloc(SIZE);
        e.mem.write(system, &vec![0xcd; SIZE as usize]);
        e.call(0x00cd_0a60, &args![system]);
        assert_eq!(e.mem.u32(system), 0x010c_ad74);
        assert_eq!(e.mem.u32(system + 4), 0);
        assert_eq!(e.mem.u32(system + 0xab8), 0x010c_ad3c);
        assert_eq!(e.mem.u32(system + 0xac0), 0);
        assert_eq!(e.mem.u32(system + 0xac4), 0);
        assert_eq!(e.mem.u32(system + 0xac8), 0x8000_0000);
        assert_eq!(e.mem.u32(system + 0xacc), 0);
        assert_eq!(e.mem.u32(system + 0xb08), system + 0x1c);
        assert_eq!(e.global::<u32>(SYSTEM_ALLOCATOR_GLOBAL), system + 0x1c);
        // Each of the 32 records has its last words cleared; the bytes the
        // constructor does not name stay as they were.
        for i in 0..32 {
            let record = system + 0xc6c + i * 0x138;
            assert_eq!(e.mem.u32(record + 0x128), 0);
            assert_eq!(e.mem.u8(record + 0x12c), 0);
            assert_eq!(e.mem.u8(record + 0x12d), 0xcd);
            assert_eq!(e.mem.u32(record + 0x130), 0);
            assert_eq!(e.mem.u32(record + 0x134), 0xcdcd_cdcd);
        }
    }

    // ---- tests of the second batch (statistics, quit, destructors, printStatistics) ----

    const SYSTEM_VTABLE: u32 = 0x4400_1000;
    const ALLOCATOR_VTABLE: u32 = 0x4400_1100;
    const LIST_VTABLE: u32 = 0x4400_1200;
    const RECORD_VTABLE: u32 = 0x4400_1300;
    const TARGET_A: u32 = 0x4400_0101;
    const TARGET_B: u32 = 0x4400_0102;
    const TARGET_C: u32 = 0x4400_0103;

    fn vtable_with(e: &mut Engine, address: u32, slots: &[(u32, u32)]) {
        e.map(address, 0x80);
        for (offset, target) in slots {
            e.mem.set_u32(address + offset, *target);
        }
    }

    fn zeroed_system(e: &mut Engine) -> u32 {
        e.mem.alloc(0x3400)
    }

    fn logged(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        calls_to(e.call_log.as_ref().unwrap(), address)
    }

    #[test]
    fn the_thread_memory_destructor_releases_the_cache_and_runs_the_base_destructor() {
        let mut e = Engine::new();
        e.register(THREAD_MEMORY_RELEASE_CACHED, |_, _| Ret::default());
        e.register(BASE_ALLOCATOR_DESTRUCTOR, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cd_06a0, &args![object, 1u32]).u32(), object);
        assert_eq!(e.mem.u32(object), THREAD_MEMORY_VTABLE);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x00cd_06a0,
                THREAD_MEMORY_RELEASE_CACHED,
                BASE_ALLOCATOR_DESTRUCTOR
            ]
        );
    }

    #[test]
    fn walk_and_garbage_collect_go_to_the_free_list_allocator() {
        let mut e = Engine::new();
        let system = zeroed_system(&mut e);
        e.mem.set_u32(system + 0x1c, LIST_VTABLE);
        vtable_with(&mut e, LIST_VTABLE, &[(0x5c, TARGET_A), (0x40, TARGET_B)]);
        e.register(TARGET_A, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register(TARGET_B, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00cd_06d0, &args![system, 0x7000u32, 0x1234u32])
                .u32(),
            7
        );
        e.call(0x00cd_06e0, &args![system]);
        assert_eq!(
            logged(&mut e, TARGET_A),
            vec![vec![system + 0x1c, 0x7000, 0x1234]]
        );
        assert_eq!(logged(&mut e, TARGET_B), vec![vec![system + 0x1c]]);
    }

    #[test]
    fn the_statistics_add_the_used_thread_memories() {
        let mut e = Engine::new();
        e.register(THREAD_MEMORY_CACHED_SIZE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x100),
            ..Ret::default()
        });
        let system = zeroed_system(&mut e);
        for (index, cached, held) in [(0u32, 7u32, 100u32), (5, 9, 40), (6, 1000, 1000)] {
            let record = system + 0xc6c + index * 0x138;
            e.mem.set_u32(record + 0x100, cached);
            e.mem.set_u32(record + 0x130, held);
            // Record 6 is not marked used.
            e.mem.set_u8(record + 0x12c, u8::from(index != 6));
        }
        let stats = e.mem.alloc(0x10);
        e.mem.set_u32(stats, 1);
        e.mem.set_u32(stats + 4, 2);
        e.mem.set_u32(stats + 8, 0x55);
        e.call(0x00cd_06f0, &args![system, stats]);
        assert_eq!(e.mem.u32(stats), 141);
        assert_eq!(e.mem.u32(stats + 4), 18);
        assert_eq!(e.mem.u32(stats + 8), 0);
    }

    #[test]
    fn the_memory_snapshot_answers_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cd_0730, &args![0x7000u32, 0u32]).u32(), 1);
    }

    #[test]
    fn main_quit_frees_the_solver_buffer_and_quits_the_threads() {
        let mut e = Engine::new();
        e.register(SOLVER_ALLOCATOR_SET_BUFFER, |_, _| Ret::default());
        let system = zeroed_system(&mut e);
        e.mem.set_u32(system, SYSTEM_VTABLE);
        vtable_with(&mut e, SYSTEM_VTABLE, &[(0x10, TARGET_A)]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        vtable_with(&mut e, ALLOCATOR_VTABLE, &[(8, TARGET_B)]);
        e.mem.set_u32(system + 4, allocator);
        e.mem.set_u32(system + 0x1c, LIST_VTABLE);
        vtable_with(&mut e, LIST_VTABLE, &[(0x40, TARGET_C)]);
        e.mem.set_u32(system + 0xb34, 0x5000);
        e.mem.set_u32(system + 0xb38, 0x5100);
        e.register(TARGET_A, |_, _| Ret::default());
        e.register(TARGET_B, |_, _| Ret::default());
        e.register(TARGET_C, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cd_07b0, &args![system, 3u32]);
        assert_eq!(
            logged(&mut e, TARGET_B),
            vec![vec![allocator, 0x5000, 0x100]]
        );
        assert_eq!(
            logged(&mut e, SOLVER_ALLOCATOR_SET_BUFFER),
            vec![vec![system + 0xb30, 0, 0]]
        );
        assert_eq!(
            logged(&mut e, TARGET_A),
            vec![vec![system, system + 0xb0c, 3]]
        );
        assert_eq!(logged(&mut e, TARGET_C), vec![vec![system + 0x1c]]);
        assert_eq!(e.mem.u32(system + 4), 0);
        // Flag 2 alone with an empty buffer frees nothing and quits no thread.
        e.mem.set_u32(system + 4, allocator);
        e.mem.set_u32(system + 0xb38, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x00cd_07b0, &args![system, 2u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(system + 4), 0);
        // No flags: only the allocator word is cleared.
        e.mem.set_u32(system + 4, allocator);
        e.call(0x00cd_07b0, &args![system, 0u32]);
        assert_eq!(e.mem.u32(system + 4), 0);
    }

    #[test]
    fn thread_quit_frees_the_router_buffer_and_clears_the_router() {
        let mut e = Engine::new();
        e.register(MEM_SET, |_, _| Ret::default());
        let system = zeroed_system(&mut e);
        e.mem.set_u32(system + 0x1c, LIST_VTABLE);
        vtable_with(&mut e, LIST_VTABLE, &[(8, TARGET_A)]);
        let record = system + 0xc6c + 2 * 0x138;
        e.mem.set_u32(record, RECORD_VTABLE);
        vtable_with(&mut e, RECORD_VTABLE, &[(0x34, TARGET_B)]);
        e.register(TARGET_A, |_, _| Ret::default());
        e.register(TARGET_B, |_, _| Ret::default());
        let router = e.mem.alloc(0x40);
        for (offset, value) in [
            (0x20, 2),
            (0, 0x6000),
            (4, 0x6000),
            (8, 0x6040),
            (0xc, 0x11),
            (0x18, 0x22),
        ] {
            e.mem.set_u32(router + offset, value);
        }
        e.mem.set_u32(record + 0x130, 0x40);
        e.mem.set_u32(record + 0x128, 0x33);
        e.mem.set_u8(record + 0x12c, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cd_0940, &args![system, router, 3u32]);
        assert_eq!(
            logged(&mut e, TARGET_A),
            vec![vec![system + 0x1c, 0x6000, 0x40]]
        );
        assert_eq!(logged(&mut e, TARGET_B), vec![vec![record], vec![record]]);
        assert_eq!(logged(&mut e, MEM_SET), vec![vec![router, 0, 0x24]]);
        for offset in [0, 4, 8, 0xc, 0x18] {
            assert_eq!(e.mem.u32(router + offset), 0);
        }
        assert_eq!(e.mem.u32(record + 0x128), 0);
        assert_eq!(e.mem.u8(record + 0x12c), 0);
        // Flag 2 alone with no kept bytes frees nothing but still releases the cache.
        e.mem.set_u32(record + 0x130, 0);
        e.mem.set_u32(router + 8, 0x6040);
        e.mem.set_u32(router + 4, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00cd_0940, &args![system, router, 2u32]);
        assert!(logged(&mut e, TARGET_A).is_empty());
        assert_eq!(logged(&mut e, TARGET_B).len(), 1);
        assert_eq!(e.mem.u32(router + 4), 0x6000);
        assert!(logged(&mut e, MEM_SET).is_empty());
    }

    fn destructor_doubles(e: &mut Engine) {
        e.map(SYSTEM_ALLOCATOR_GLOBAL & !0xfff, 0x1000);
        e.register(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(THREAD_MEMORY_RELEASE_CACHED, |_, _| Ret::default());
        e.register(BASE_ALLOCATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(SOLVER_ALLOCATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE_LIST_ALLOCATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_SYSTEM_BASE_DESTRUCTOR, |_, _| Ret::default());
    }

    #[test]
    fn the_destructor_runs_every_part_in_the_exes_order() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let system = zeroed_system(&mut e);
        e.set_global(SYSTEM_ALLOCATOR_GLOBAL, 0x1234u32);
        e.call_log = Some(vec![]);
        e.call(0x00cd_09d0, &args![system]);
        assert_eq!(e.global::<u32>(SYSTEM_ALLOCATOR_GLOBAL), 0);
        assert_eq!(e.mem.u32(system), VTABLE);
        assert_eq!(e.mem.u32(system + 8), BLOCK_SERVER_VTABLE);
        let released = logged(&mut e, THREAD_MEMORY_RELEASE_CACHED);
        assert_eq!(released.len(), 32);
        assert_eq!(released[0], vec![system + 0x3234]);
        assert_eq!(released[31], vec![system + 0xc6c]);
        assert_eq!(e.mem.u32(system + 0x3234), THREAD_MEMORY_VTABLE);
        assert_eq!(
            logged(&mut e, DELETE_CRITICAL_SECTION),
            vec![vec![system + 0x336c], vec![system + 0xad0]]
        );
        let base_destructions = logged(&mut e, BASE_ALLOCATOR_DESTRUCTOR);
        assert_eq!(base_destructions.len(), 33);
        assert_eq!(base_destructions[32], vec![system + 0xab8]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order[order.len() - 2..],
            [
                FREE_LIST_ALLOCATOR_DESTRUCTOR,
                MEMORY_SYSTEM_BASE_DESTRUCTOR
            ]
        );
        assert_eq!(
            logged(&mut e, SOLVER_ALLOCATOR_DESTRUCTOR),
            vec![vec![system + 0xb30]]
        );
    }

    #[test]
    fn the_scalar_deleting_destructors_answer_this() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let system = zeroed_system(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cd_0c70, &args![system, 1u32]).u32(), system);
        assert_eq!(
            logged(&mut e, MEMORY_SYSTEM_BASE_DESTRUCTOR),
            vec![vec![system]]
        );
        // The leak detector's destructor deletes its section at +0x18.
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cd_0c50, &args![object, 1u32]).u32(), object);
        assert_eq!(
            logged(&mut e, DELETE_CRITICAL_SECTION),
            vec![vec![object + 0x18]]
        );
        assert_eq!(
            logged(&mut e, BASE_ALLOCATOR_DESTRUCTOR),
            vec![vec![object]]
        );
        // The block server's only stores its virtual table.
        assert_eq!(e.call(0x00d6_0540, &args![object, 1u32]).u32(), object);
        assert_eq!(e.mem.u32(object), BLOCK_SERVER_VTABLE);
    }

    fn router_engine(e: &mut Engine) -> u32 {
        // The memory router answers an object whose +0x14 is an allocator; the allocator's
        // slot 0x10 is the free.
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        vtable_with(e, ALLOCATOR_VTABLE, &[(0x10, TARGET_A)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x14, allocator);
        e.register_double(ROUTER_GETTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(TARGET_A, |_, _| Ret::default());
        allocator
    }

    #[test]
    fn the_print_input_destructor_frees_owned_arrays_only() {
        let mut e = Engine::new();
        let allocator = router_engine(&mut e);
        let input = e.mem.alloc(0x24);
        // The first array (at +4) owns 5 words, the second (at +0x10) does not own its memory.
        for (offset, value) in [
            (4, 0x7000),
            (8, 3),
            (0xc, 5),
            (0x10, 0x8000),
            (0x14, 2),
            (0x18, 0x8000_0004),
        ] {
            e.mem.set_u32(input + offset, value);
        }
        e.call_log = Some(vec![]);
        e.call(0x00cd_0c80, &args![input]);
        assert_eq!(logged(&mut e, TARGET_A), vec![vec![allocator, 0x7000, 20]]);
        for offset in [4, 8, 0x10, 0x14] {
            assert_eq!(e.mem.u32(input + offset), 0);
        }
        assert_eq!(e.mem.u32(input + 0xc), 0x8000_0000);
        assert_eq!(e.mem.u32(input + 0x18), 0x8000_0000);
    }

    #[test]
    fn print_statistics_collects_the_used_records_and_prints() {
        let mut e = Engine::new();
        let allocator = router_engine(&mut e);
        e.register(STREAM_WRITE_TEXT, |_, _| Ret::default());
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            // Gives the array room for 4 words.
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let system = zeroed_system(&mut e);
        e.register_double(MEMORY_SYSTEM_PRINT, move |e, a| {
            // `(stream, input)`: the input as printed: the word at +0xb08, 2 and 2 elements,
            // the soft limit.
            assert_eq!((a.len(), a[0]), (2, 0x7000));
            assert_eq!(e.mem.u32(a[1]), 0x5555);
            assert_eq!(e.mem.u32(a[1] + 8), 2);
            assert_eq!(e.mem.u32(a[1] + 0x14), 2);
            assert_eq!(e.mem.u32(a[1] + 0x20), 0x3333);
            let first = e.mem.u32(a[1] + 4);
            assert_eq!(e.mem.u32(first), system + 0xc6c);
            assert_eq!(e.mem.u32(first + 4), system + 0xc6c + 3 * 0x138);
            let second = e.mem.u32(a[1] + 0x10);
            assert_eq!(e.mem.u32(second), 0xaaaa);
            assert_eq!(e.mem.u32(second + 4), 0xbbbb);
            Ret::default()
        });
        e.mem.set_u32(system + 0xb08, 0x5555);
        e.mem.set_u32(system + 0x1c, LIST_VTABLE);
        vtable_with(&mut e, LIST_VTABLE, &[(0x50, TARGET_B)]);
        e.register(TARGET_B, |_, _| Ret {
            eax: 0x3333,
            ..Ret::default()
        });
        for (index, router_word) in [(0u32, 0xaaaau32), (3, 0xbbbb)] {
            let record = system + 0xc6c + index * 0x138;
            e.mem.set_u8(record + 0x12c, 1);
            e.mem.set_u32(record + 0x134, router_word);
        }
        e.call_log = Some(vec![]);
        e.call(0x00cd_0b30, &args![system, 0x7000u32]);
        assert_eq!(
            logged(&mut e, STREAM_WRITE_TEXT),
            vec![
                vec![0x7000, STATISTICS_TEXT_FIRST],
                vec![0x7000, STATISTICS_TEXT_SECOND]
            ]
        );
        // Each array is full (size 0 = capacity 0) on its first element: two growths.
        assert_eq!(logged(&mut e, ARRAY_RESERVE_MORE).len(), 2);
        assert_eq!(allocator, logged(&mut e, ARRAY_RESERVE_MORE)[0][0]);
        assert_eq!(logged(&mut e, MEMORY_SYSTEM_PRINT).len(), 1);
        // The input's destructor ran: it freed both arrays through slot 0x10.
        assert_eq!(logged(&mut e, TARGET_A).len(), 2);
    }
}
