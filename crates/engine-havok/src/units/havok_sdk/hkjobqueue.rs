//! `Havok SDK/hkjobqueue.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no type for `hkJobQueue`, so its layout is described from the code (names
//! are descriptions, not the Havok field names):
//!
//! - `+0x00`: the critical section (`hkCriticalSection`, a Windows `CRITICAL_SECTION`);
//! - `+0x18`: the pointer to the shared block (0x170 bytes) every thread works on under the
//!   lock: `u16` job counts per job type from `+0`, a bit mask at `+0x18`, a flag word at
//!   `+0x1c`, a byte at `+0x20`, `u16` waiting-thread counts per queue from `+0x22`, and from
//!   `+0x2c` sixteen job queues of 0x14 bytes each (a ring buffer of 128-byte jobs: buffer,
//!   capacity, head, tail, count);
//! - `+0x1c`, `+0x20`: two function pointers (cdecl, `(queue, shared block, job, word)`) called
//!   on a job just taken from a ring and on a job that has finished; the constructor sets them
//!   to `00d04550` and `00d04570`;
//! - `+0x24`: the thread count (copied from the set-up object), `+0x28`..`+0x34`, `+0x48`,
//!   `+0x4c`, `+0x50`, `+0x54`: queue ranges and the extra-rule count that
//!   `updateJobQueryRules` computes; `+0x38 + 8 * i`: a copy of extra rule `i` (type word,
//!   subtype byte at `+4`, queue index byte at `+5`);
//! - `+0x58`: a copy of the hardware set-up (`00d05450` constructs it, `00d05b50` assigns it):
//!   words at `+0x58`, `+0x60`, a byte at `+0x5c`, and an array of 12-byte arrays at `+0x64`
//!   (data, size at `+0x68`, capacity word at `+0x6c`);
//! - `+0x74`: byte "semaphores created", `+0x78 + 4 * i`: the semaphore pointers, `+0x8c`: their
//!   count; from `+0x90`, lists of queue indices (18 bytes each, ended by `0xff`); `+0xea + t`:
//!   the queue index of thread `t` (a signed byte); `+0xf0 + 16 * type`: the job-type table
//!   (function table pointer, word, two hook pointers);
//! - `+0x1a0`: an array of 12-byte rules (type word, subtype byte, thread index word), size at
//!   `+0x1a4`, capacity word at `+0x1a8`.
//!
//! Locks, semaphores and thread-local storage are called through their import slots and the
//! semaphore translations exactly as the exe does. The functions write Havok's profiling
//! records (`RDTSC` time stamps appended to the thread's monitor stream); the engine has no
//! model of `RDTSC`, so the host clock stands in for the time stamps. The exe passes
//! `lockQueue` and `unlockQueue` a word it never reads (the address of a stack buffer), which
//! the translations pass as 0.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkadeltacompressedanimation::profile_record;

/// The memory router (`00c85750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkSemaphore::hkSemaphore` (`00d04350`, `this`, `(initial count, maximum count)`).
const SEMAPHORE_CONSTRUCT: u32 = 0x00d0_4350;
/// `hkSemaphore::~hkSemaphore` (`00d04370`).
const SEMAPHORE_DESTRUCT: u32 = 0x00d0_4370;
/// Releases a semaphore (`00d043c0`, cdecl `(semaphore, count)`).
const SEMAPHORE_RELEASE: u32 = 0x00d0_43c0;
/// `hkSemaphore::acquire` (`00d04380`).
const SEMAPHORE_ACQUIRE: u32 = 0x00d0_4380;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, `this`, one word).
const CRITICAL_SECTION_CONSTRUCT: u32 = 0x00cf_6da0;
/// The constructor of one job ring (`00e46e80`, `this` = the ring).
const RING_CONSTRUCT: u32 = 0x00e4_6e80;
/// The destructor of the hardware set-up copy at `+0x58` (`00cbf490`).
const HW_SETUP_DESTRUCT: u32 = 0x00cb_f490;
/// Fills the set-up's thread count (`00cbb960`, cdecl `(address of the result)`).
const THREAD_COUNT_QUERY: u32 = 0x00cb_b960;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The `hkArray` growth routine (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, bytes)`).
const MEM_COPY: u32 = 0x00cb_1260;
/// Import slots: `EnterCriticalSection`, `LeaveCriticalSection`, `DeleteCriticalSection`,
/// `TlsGetValue`, `TlsSetValue`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
/// The exe's global holding the TLS index of the thread's number.
const THREAD_INDEX_SLOT: u32 = 0x0126_87c8;
/// The default hooks the constructor stores at `+0x1c` and `+0x20`, and the default hook words
/// of each job-type table entry (`+8`, `+0xc`).
const DEFAULT_POP_HOOK: u32 = 0x00d0_4550;
const DEFAULT_FINISH_HOOK: u32 = 0x00d0_4570;
const DEFAULT_TYPE_HOOK_A: u32 = 0x00d0_43d0;
const DEFAULT_TYPE_HOOK_B: u32 = 0x00d0_43e0;
/// The monitor labels the functions write (the address of the string) and the one that ends a
/// span (`010c41f8`, the string `"Et"`).
const LABEL_ADD_JOB_BATCH: u32 = 0x010c_cd60;
const LABEL_FINISH_JOB: u32 = 0x010c_cd84;
const LABEL_GET_NEXT_JOB: u32 = 0x010c_cd90;
const LABEL_NO_JOB_AVAILABLE: u32 = 0x010c_cd70;
const LABEL_END: u32 = 0x010c_41f8;
/// `processAllJobs`' label per job type (0..=10; type 8 and anything above 10 take the
/// unknown-jobs label).
const LABEL_UNKNOWN_JOBS: u32 = 0x010c_cda0;
const JOB_TYPE_LABELS: [u32; 11] = [
    0x010c_c568, // "TtPhysics"
    0x010c_c568, // "TtPhysics"
    0x010c_ce10, // "TtCollision Query"
    0x010c_ce00, // "TtRayCast Query"
    0x010c_cdf4, // "TtAnimation"
    0x010c_cde8, // "TtBehavior"
    0x010c_cde0, // "TtCloth"
    0x010c_cdd0, // "TtPathfinding"
    LABEL_UNKNOWN_JOBS,
    0x010c_cdbc, // "TtCharacter Proxy"
    0x010c_cdb0, // "TtUserJob"
];

/// Bytes in a job (the ring entries and the stack copies).
const JOB_BYTES: u32 = 0x80;
/// Bytes of one job ring inside the shared block, and where the rings start.
const RING_BYTES: u32 = 0x14;
const RINGS_OFFSET: u32 = 0x2c;
/// `hkArray` capacity word: the size bits.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// `hkArray` capacity word of an array that owns nothing.
const EMPTY_CAPACITY: u32 = 0x8000_0000;

/// The memory router's heap allocator: its virtual slot `4` allocates `(bytes)`, `8` frees
/// `(block, bytes)`, `0xc` allocates `(address of the byte count)` and `0x10` frees.
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router.wrapping_add(0x10))
}

fn heap_alloc(e: &mut Engine, bytes: u32) -> u32 {
    let allocator = heap_allocator(e);
    e.vcall(allocator, 4, &args![bytes]).u32()
}

fn heap_free(e: &mut Engine, block: u32, bytes: u32) {
    let allocator = heap_allocator(e);
    e.vcall(allocator, 8, &args![block, bytes]);
}

/// The ring of the shared block `data` with index `index`.
fn ring_of(data: u32, index: u32) -> u32 {
    data.wrapping_add(index.wrapping_mul(RING_BYTES))
        .wrapping_add(RINGS_OFFSET)
}

/// `REP MOVSD` of one job (32 words, forward).
fn copy_job(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..0x20 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(destination.wrapping_add(4 * word), value);
    }
}

/// The new capacity a full ring asks for: 8 when it has none, else double.
fn grown_capacity(capacity: i32) -> i32 {
    if capacity == 0 {
        8
    } else {
        capacity.wrapping_add(capacity)
    }
}

/// Releases semaphore `semaphore` once (`00d043c0` with a count of 1).
fn release_semaphore(e: &mut Engine, semaphore: u32) {
    e.call(SEMAPHORE_RELEASE, &args![semaphore, 1u32]);
}

/// `u16` slot `index` (signed) of the waiting-thread counts at `+0x22` of the shared block.
fn waiting_slot(data: u32, index: i32) -> u32 {
    data.wrapping_add((index as u32).wrapping_mul(2))
        .wrapping_add(0x22)
}

/// Decrements the `u16` at `address` (wrapping, as `ADD word ptr [..],0xffff`).
fn decrement_u16(e: &mut Engine, address: u32) {
    let value = e.mem.u16(address).wrapping_add(0xffff);
    e.mem.set_u16(address, value);
}

fn increment_u16(e: &mut Engine, address: u32) {
    let value = e.mem.u16(address).wrapping_add(1);
    e.mem.set_u16(address, value);
}

/// Pushes the job at `job` on the front of the ring at `ring` (the body of `00d057f0`, which
/// several functions have inline).
fn ring_push_front(e: &mut Engine, ring: u32, job: u32) {
    let capacity = e.mem.i32(ring + 4);
    if e.mem.i32(ring + 0x10) >= capacity {
        fn_00d056d0(e, Ptr::new(ring), grown_capacity(capacity));
    }
    if e.mem.u32(ring + 8) == 0 {
        let capacity = e.mem.u32(ring + 4);
        e.mem.set_u32(ring + 8, capacity);
    }
    let head = e.mem.u32(ring + 8).wrapping_sub(1);
    e.mem.set_u32(ring + 8, head);
    let destination = (head << 7).wrapping_add(e.mem.u32(ring));
    copy_job(e, destination, job);
    let count = e.mem.u32(ring + 0x10).wrapping_add(1);
    e.mem.set_u32(ring + 0x10, count);
}

/// Pushes the job at `job` on the back of the ring at `ring` (the body of `00d057a0`).
fn ring_push_back(e: &mut Engine, ring: u32, job: u32) {
    let capacity = e.mem.i32(ring + 4);
    if e.mem.i32(ring + 0x10) >= capacity {
        fn_00d056d0(e, Ptr::new(ring), grown_capacity(capacity));
    }
    if e.mem.u32(ring + 0xc) == e.mem.u32(ring + 4) {
        e.mem.set_u32(ring + 0xc, 0);
    }
    let tail = e.mem.u32(ring + 0xc);
    let destination = (tail << 7).wrapping_add(e.mem.u32(ring));
    copy_job(e, destination, job);
    e.mem.set_u32(ring + 0xc, tail.wrapping_add(1));
    let count = e.mem.u32(ring + 0x10).wrapping_add(1);
    e.mem.set_u32(ring + 0x10, count);
}

/// Pushes `job` on the front (`front` true) or the back of the ring `index` of `data`, the
/// inline form `finishJob` and the get-next functions use after a hook asks for it.
fn ring_push(e: &mut Engine, data: u32, index: u32, front: bool, job: u32) {
    let ring = ring_of(data, index);
    if front {
        ring_push_front(e, ring, job);
    } else {
        ring_push_back(e, ring, job);
    }
}

// Translated from 00d043b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl `(semaphore)`): `hkSemaphore::acquire` (`00d04380`) on the
/// semaphore, which it tail-jumps to; the wait result stays in `EAX`.
pub fn fn_00d043b0(e: &mut Engine, semaphore: Ptr) -> u32 {
    e.call(SEMAPHORE_ACQUIRE, &args![semaphore]).u32()
}

// Translated from 00d04400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::lockQueue` (Xbox PDB), `thiscall`, `RET 4` (the word is never read): enters the
/// queue's critical section and answers the shared block pointer at `+0x18`.
pub fn hk_job_queue_lock_queue(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    e.call(ENTER_CRITICAL_SECTION, &args![this]);
    e.mem.u32(this.addr() + 0x18)
}

// Translated from 00d04420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::unlockQueue` (Xbox PDB), `thiscall`, one stack word that is never read: it
/// overwrites that word with `this` and tail-jumps to `LeaveCriticalSection`.
pub fn hk_job_queue_unlock_queue(e: &mut Engine, this: Ptr, _unused_1: u32) {
    e.call(LEAVE_CRITICAL_SECTION, &args![this]);
}

// Translated from 00d04440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `+0x18` of the shared block (the thread bit mask),
/// read without the lock.
pub fn fn_00d04440(e: &mut Engine, this: Ptr) -> u32 {
    let data = e.mem.u32(this.addr() + 0x18);
    e.mem.u32(data.wrapping_add(0x18))
}

// Translated from 00d04450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): under the lock, stores `mask` as the bit
/// mask at `+0x18` of the shared block; then, when the waiting count of the queue that the
/// byte at `+0xea` names (a signed byte) is not zero, takes one off it and releases that
/// queue's semaphore (`+0x78 + 4 * index`) once.
pub fn fn_00d04450(e: &mut Engine, this: Ptr, mask: u32) {
    let base = this.addr();
    let data = hk_job_queue_lock_queue(e, this, 0);
    let shared = e.mem.u32(base + 0x18);
    e.mem.set_u32(shared.wrapping_add(0x18), mask);
    let index = i32::from(e.mem.i8(base + 0xea));
    let slot = waiting_slot(data, index);
    if e.mem.u16(slot) != 0 {
        decrement_u16(e, slot);
        let semaphore = e
            .mem
            .u32(base.wrapping_add((index as u32).wrapping_mul(4)) + 0x78);
        release_semaphore(e, semaphore);
    }
    hk_job_queue_unlock_queue(e, this, data);
}

// Translated from 00d044c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`): stores four words (`a`..`d`) in the
/// job-type table entry of `job_type` (`this + (job_type + 0xf) * 0x10`).
pub fn fn_00d044c0(e: &mut Engine, this: Ptr, job_type: u32, a: u32, b: u32, c: u32, d: u32) {
    let entry = job_type
        .wrapping_add(0xf)
        .wrapping_shl(4)
        .wrapping_add(this.addr());
    e.mem.set_u32(entry, a);
    e.mem.set_u32(entry.wrapping_add(4), b);
    e.mem.set_u32(entry.wrapping_add(8), c);
    e.mem.set_u32(entry.wrapping_add(12), d);
}

// Translated from 00d044f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): for every queue below the count at
/// `+0x8c`, clears the waiting count in the shared block `data` and releases that queue's
/// semaphore once for each thread that was waiting.
pub fn fn_00d044f0(e: &mut Engine, this: Ptr, data: Ptr) {
    let base = this.addr();
    let mut index = 0i32;
    if e.mem.i32(base + 0x8c) > 0 {
        let mut semaphores = base + 0x78;
        let mut slot = data.addr().wrapping_add(0x22);
        loop {
            let mut waiting = u32::from(e.mem.u16(slot));
            e.mem.set_u16(slot, 0);
            if (waiting as i32) > 0 {
                loop {
                    let semaphore = e.mem.u32(semaphores);
                    release_semaphore(e, semaphore);
                    waiting = waiting.wrapping_sub(1);
                    if (waiting as i32) <= 0 {
                        break;
                    }
                }
            }
            index += 1;
            slot = slot.wrapping_add(2);
            semaphores = semaphores.wrapping_add(4);
            if index >= e.mem.i32(base + 0x8c) {
                break;
            }
        }
    }
}

// Translated from 00d04590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::updateJobQueryRules` (Xbox PDB), `fastcall`: works out which queue each thread
/// takes jobs from and in what order. From the set-up copy (`+0x58`..) and the rule array
/// (`+0x1a0`) it sets the queue ranges (`+0x28`..`+0x34`, `+0x48`..`+0x54`), the thread-to-queue
/// byte table (`+0xea`), the lists of queue indices (`+0x90`, 18 bytes each, ended by `0xff`),
/// the copied rules (`+0x38`), then (re)creates the semaphores (one per queue, `00d04350` with
/// `(0, 1000)`; the old ones are destroyed first) and clears the waiting counts.
pub fn hk_job_queue_update_job_query_rules(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let group_count = e.mem.i32(base + 0x68);
    let queues = if group_count != 0 { group_count } else { 1 };
    let extra = e.mem.u32(base + 0x1a4);
    e.mem.set_u32(base + 0x48, extra);
    if e.mem.u8(base + 0x5c) == 0 {
        let first = e.mem.u32(base + 0x24);
        e.mem.set_u32(base + 0x28, first);
    } else {
        e.mem.set_u32(base + 0x28, 0);
    }
    let first = e.mem.u32(base + 0x28);
    let after_groups = first.wrapping_add(queues as u32);
    e.mem.set_u32(base + 0x2c, after_groups);
    let shared_start = after_groups.wrapping_add(extra);
    let shared_end = e.mem.u32(base + 0x24).wrapping_add(shared_start);
    e.mem.set_u32(base + 0x4c, first);
    e.mem.set_u32(base + 0x30, shared_start);
    e.mem.set_u32(base + 0x34, shared_end);
    e.mem.set_u32(base + 0x50, shared_start);
    // Thread-to-queue table: every thread of a group takes that group's queue.
    if group_count > 0 {
        let mut thread = 0i32;
        if e.mem.i32(base + 0x60) > 0 {
            loop {
                e.mem.set_u8(base.wrapping_add(thread as u32) + 0xea, 0xff);
                thread += 1;
                if thread >= e.mem.i32(base + 0x60) {
                    break;
                }
            }
        }
        let mut group = 0i32;
        if e.mem.i32(base + 0x68) > 0 {
            let mut offset = 0u32;
            loop {
                let groups = e.mem.u32(base + 0x64);
                let mut member = 0i32;
                if e.mem.i32(groups.wrapping_add(offset) + 4) > 0 {
                    loop {
                        let groups = e.mem.u32(base + 0x64);
                        let low = e.mem.u8(base + 0x28);
                        let list = e.mem.u32(groups.wrapping_add(offset));
                        let thread = e
                            .mem
                            .u32(list.wrapping_add((member as u32).wrapping_mul(4)));
                        e.mem.set_u8(
                            thread.wrapping_add(base).wrapping_add(0xea),
                            low.wrapping_add(group as u8),
                        );
                        let groups = e.mem.u32(base + 0x64);
                        member += 1;
                        if member >= e.mem.i32(groups.wrapping_add(offset) + 4) {
                            break;
                        }
                    }
                }
                group += 1;
                offset = offset.wrapping_add(0xc);
                if group >= e.mem.i32(base + 0x68) {
                    break;
                }
            }
        }
    }
    if e.mem.i32(base + 0x68) == 0 {
        let mut thread = 0i32;
        if e.mem.i32(base + 0x60) > 0 {
            loop {
                let low = e.mem.u8(base + 0x28);
                e.mem.set_u8(base.wrapping_add(thread as u32) + 0xea, low);
                thread += 1;
                if thread >= e.mem.i32(base + 0x60) {
                    break;
                }
            }
        }
    }
    // The list of queue indices of every group queue.
    let mut group = 0i32;
    if queues > 0 {
        loop {
            let slot = e
                .mem
                .u32(base + 0x28)
                .wrapping_add(group as u32)
                .wrapping_add(8)
                .wrapping_mul(18)
                .wrapping_add(base);
            let mut cursor = slot;
            let low = e.mem.u8(base + 0x28).wrapping_add(group as u8);
            e.mem.set_u8(cursor, low);
            cursor = cursor.wrapping_add(1);
            let mut other = 0i32;
            if e.mem.i32(base + 0x68) > 0 {
                loop {
                    if group != other {
                        let value = e.mem.u8(base + 0x28).wrapping_add(other as u8);
                        e.mem.set_u8(cursor, value);
                        cursor = cursor.wrapping_add(1);
                    }
                    other += 1;
                    if other >= e.mem.i32(base + 0x68) {
                        break;
                    }
                }
            }
            let mut shared = 0i32;
            if e.mem.i32(base + 0x24) > 0 {
                loop {
                    let value = e.mem.u8(base + 0x30).wrapping_add(shared as u8);
                    e.mem.set_u8(cursor, value);
                    shared += 1;
                    cursor = cursor.wrapping_add(1);
                    if shared >= e.mem.i32(base + 0x24) {
                        break;
                    }
                }
            }
            if e.mem.i32(base + 0x58) == 0 {
                let mut shared = 0i32;
                if e.mem.i32(base + 0x24) > 0 {
                    loop {
                        e.mem.set_u8(cursor, shared as u8);
                        shared += 1;
                        cursor = cursor.wrapping_add(1);
                        if shared >= e.mem.i32(base + 0x24) {
                            break;
                        }
                    }
                }
            }
            let mut rule = 0i32;
            if e.mem.i32(base + 0x48) > 0 {
                loop {
                    let value = e.mem.u8(base + 0x2c).wrapping_add(rule as u8);
                    e.mem.set_u8(cursor, value);
                    rule += 1;
                    cursor = cursor.wrapping_add(1);
                    if rule >= e.mem.i32(base + 0x48) {
                        break;
                    }
                }
            }
            group += 1;
            e.mem.set_u8(cursor, 0xff);
            if group >= queues {
                break;
            }
        }
    }
    // The extra rules: each gets a queue of its own, taking its thread's old queue's list.
    let mut rule = 0i32;
    if e.mem.i32(base + 0x48) > 0 {
        let mut offset = 0u32;
        let mut entry = base + 0x3d;
        loop {
            let rules = e.mem.u32(base + 0x1a0);
            let word = e.mem.u32(rules.wrapping_add(offset));
            e.mem.set_u32(entry.wrapping_sub(5), word);
            let rules = e.mem.u32(base + 0x1a0);
            let subtype = e.mem.u8(rules.wrapping_add(offset).wrapping_add(4));
            e.mem.set_u8(entry.wrapping_sub(1), subtype);
            let queue_index = e.mem.u8(base + 0x2c).wrapping_add(rule as u8);
            e.mem.set_u8(entry, queue_index);
            let rules = e.mem.u32(base + 0x1a0);
            let thread = e.mem.u32(rules.wrapping_add(offset).wrapping_add(8));
            let table_slot = thread.wrapping_add(base).wrapping_add(0xea);
            let old_queue = i32::from(e.mem.i8(table_slot));
            e.mem.set_u8(table_slot, queue_index);
            let slot = e
                .mem
                .u32(base + 0x2c)
                .wrapping_add(rule as u32)
                .wrapping_add(8)
                .wrapping_mul(18)
                .wrapping_add(base);
            let mut cursor = slot;
            let value = e.mem.u8(base + 0x2c).wrapping_add(rule as u8);
            e.mem.set_u8(cursor, value);
            let mut from = base.wrapping_add(
                (old_queue.wrapping_mul(9).wrapping_add(0x48) as u32).wrapping_mul(2),
            );
            cursor = cursor.wrapping_add(1);
            if e.mem.u8(from) != 0xff {
                loop {
                    let byte = e.mem.u8(from);
                    let new_index = e.mem.u8(entry);
                    if i32::from(byte as i8) != i32::from(new_index) {
                        e.mem.set_u8(cursor, byte);
                        cursor = cursor.wrapping_add(1);
                    }
                    from = from.wrapping_add(1);
                    if e.mem.u8(from) == 0xff {
                        break;
                    }
                }
            }
            rule += 1;
            e.mem.set_u8(cursor, 0xff);
            offset = offset.wrapping_add(0xc);
            entry = entry.wrapping_add(8);
            if rule >= e.mem.i32(base + 0x48) {
                break;
            }
        }
    }
    // The main thread's queue, kept apart when another thread shares it.
    let mut semaphore_count = e.mem.u32(base + 0x30);
    let main_queue = e.mem.i8(base + 0xea);
    e.mem.set_u32(base + 0x54, i32::from(main_queue) as u32);
    let mut thread = 1i32;
    if e.mem.i32(base + 0x60) > 1 {
        loop {
            if e.mem.u8(base.wrapping_add(thread as u32) + 0xea) == main_queue as u8 {
                e.mem.set_u8(base + 0xea, semaphore_count as u8);
                semaphore_count = semaphore_count.wrapping_add(1);
                break;
            }
            thread += 1;
            if thread >= e.mem.i32(base + 0x60) {
                break;
            }
        }
    }
    // Destroy the semaphores of an earlier call, then make one per queue.
    if e.mem.u8(base + 0x74) != 0 {
        let mut index = 0i32;
        if e.mem.i32(base + 0x8c) > 0 {
            let mut slot = base + 0x78;
            loop {
                let semaphore = e.mem.u32(slot);
                if semaphore != 0 {
                    e.call(SEMAPHORE_DESTRUCT, &args![semaphore]);
                    heap_free(e, semaphore, 4);
                }
                index += 1;
                slot = slot.wrapping_add(4);
                if index >= e.mem.i32(base + 0x8c) {
                    break;
                }
            }
        }
    }
    e.mem.set_u32(base + 0x8c, semaphore_count);
    let mut index = 0i32;
    if (semaphore_count as i32) > 0 {
        let mut waiting = 0x22u32;
        let mut slot = base + 0x78;
        loop {
            let data = e.mem.u32(base + 0x18);
            e.mem.set_u16(waiting.wrapping_add(data), 0);
            let block = heap_alloc(e, 4);
            let semaphore = if block != 0 {
                e.call(SEMAPHORE_CONSTRUCT, &args![block, 0u32, 1000u32])
                    .u32()
            } else {
                0
            };
            e.mem.set_u32(slot, semaphore);
            index += 1;
            waiting = waiting.wrapping_add(2);
            slot = slot.wrapping_add(4);
            if index >= e.mem.i32(base + 0x8c) {
                break;
            }
        }
    }
    e.mem.set_u8(base + 0x74, 1);
}

// Translated from 00d04890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::getQueueIndexForJob` (Xbox PDB), `thiscall`, `RET 4`: the queue that takes the
/// job at `job` (byte `+0` its subtype, byte `+1` its type): the index of the first extra rule
/// (`+0x38 + 8 * i`) whose type and subtype both match, or the type plus the base at `+0x30`.
pub fn hk_job_queue_get_queue_index_for_job(e: &mut Engine, this: Ptr, job: Ptr) -> u32 {
    let base = this.addr();
    let job = job.addr();
    let count = e.mem.i32(base + 0x48);
    let mut index = 0i32;
    if count > 0 {
        let job_type = e.mem.u8(job + 1);
        let mut entry = base + 0x3c;
        loop {
            if job_type == e.mem.u8(entry.wrapping_sub(4)) && e.mem.u8(job) == e.mem.u8(entry) {
                return u32::from(
                    e.mem
                        .u8(base.wrapping_add((index as u32).wrapping_mul(8)) + 0x3d),
                );
            }
            index += 1;
            entry = entry.wrapping_add(8);
            if index >= count {
                break;
            }
        }
    }
    u32::from(e.mem.u8(job + 1)).wrapping_add(e.mem.u32(base + 0x30))
}

// Translated from 00d048e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): wakes one waiting thread after a queue got
/// work. Looks through the rings from `+0x4c` to `+0x50` for one with jobs and a waiting thread
/// in the slot of the same index (wakes that one); failing that, if any ring from there to
/// `+0x34` has jobs, or one with jobs was seen, wakes the first queue (from `+0x4c`) that has a
/// waiting thread.
pub fn fn_00d048e0(e: &mut Engine, this: Ptr, data: Ptr) {
    let base = this.addr();
    let data = data.addr();
    let mut index = e.mem.i32(base + 0x4c);
    let end = e.mem.i32(base + 0x50);
    let mut seen_jobs = false;
    if index < end {
        let mut count = data.wrapping_add(
            (index as u32)
                .wrapping_mul(5)
                .wrapping_add(0xf)
                .wrapping_mul(4),
        );
        let mut slot = data
            .wrapping_add((index as u32).wrapping_mul(2))
            .wrapping_add(0x22);
        loop {
            if e.mem.u32(count) != 0 {
                seen_jobs = true;
                if e.mem.u16(slot) != 0 {
                    wake_waiting(e, base, data, index);
                    return;
                }
            }
            index += 1;
            count = count.wrapping_add(0x14);
            slot = slot.wrapping_add(2);
            if index >= end {
                break;
            }
        }
    }
    let shared_end = e.mem.i32(base + 0x34);
    let mut found_jobs = false;
    if index < shared_end {
        let mut count = data.wrapping_add(
            (index as u32)
                .wrapping_mul(5)
                .wrapping_add(0xf)
                .wrapping_mul(4),
        );
        loop {
            if e.mem.u32(count) != 0 {
                found_jobs = true;
                break;
            }
            index += 1;
            count = count.wrapping_add(0x14);
            if index >= shared_end {
                break;
            }
        }
    }
    if found_jobs || seen_jobs {
        let mut index = e.mem.i32(base + 0x4c);
        let queues = e.mem.i32(base + 0x8c);
        if index < queues {
            let mut slot = data
                .wrapping_add((index as u32).wrapping_mul(2))
                .wrapping_add(0x22);
            loop {
                if e.mem.u16(slot) != 0 {
                    wake_waiting(e, base, data, index);
                    return;
                }
                index += 1;
                slot = slot.wrapping_add(2);
                if index >= queues {
                    break;
                }
            }
        }
    }
}

/// Takes one off the waiting count of queue `index` and releases its semaphore once.
fn wake_waiting(e: &mut Engine, base: u32, data: u32, index: i32) {
    decrement_u16(e, waiting_slot(data, index));
    let semaphore = e.mem.u32(
        base.wrapping_add((index as u32).wrapping_mul(4))
            .wrapping_add(0x78),
    );
    release_semaphore(e, semaphore);
}

// Translated from 00d04990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::releaseOneWaitingThread` (Xbox PDB), `thiscall`, `RET 8`: when `index` is below
/// the queue count (`+0x8c`) and a thread waits on that queue, takes one off its waiting count
/// in `data` and releases its semaphore once; otherwise tries the general wake-up (`00d048e0`).
pub fn hk_job_queue_release_one_waiting_thread(e: &mut Engine, this: Ptr, index: i32, data: Ptr) {
    let base = this.addr();
    if index < e.mem.i32(base + 0x8c) {
        let slot = waiting_slot(data.addr(), index);
        let waiting = e.mem.u16(slot);
        if waiting != 0 {
            e.mem.set_u16(slot, waiting.wrapping_sub(1));
            let semaphore = e.mem.u32(
                base.wrapping_add((index as u32).wrapping_mul(4))
                    .wrapping_add(0x78),
            );
            release_semaphore(e, semaphore);
            return;
        }
    }
    fn_00d048e0(e, this, data);
}

// Translated from 00d049d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): under the lock, stores `flag` in the word at
/// `+0x1c` of the shared block; when it is 0, wakes every waiting thread (`00d044f0`).
pub fn fn_00d049d0(e: &mut Engine, this: Ptr, flag: u32) {
    let data = hk_job_queue_lock_queue(e, this, 0);
    let shared = e.mem.u32(this.addr() + 0x18);
    e.mem.set_u32(shared.wrapping_add(0x1c), flag);
    if flag == 0 {
        fn_00d044f0(e, this, Ptr::new(data));
    }
    hk_job_queue_unlock_queue(e, this, data);
}

// Translated from 00d04a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): appends a rule (`type_word`, `subtype`,
/// `thread`) to the 12-byte rule array at `+0x1a0` (growing it through `00c90940` when full)
/// and recomputes the queues (`updateJobQueryRules`).
pub fn fn_00d04a20(e: &mut Engine, this: Ptr, type_word: u32, subtype: u8, thread: u32) {
    let base = this.addr();
    let array = base + 0x1a0;
    let allocator = heap_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_MASK {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0xcu32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    let rule = data.wrapping_add(size.wrapping_mul(12));
    e.mem.set_u32(rule, type_word);
    e.mem.set_u8(rule + 4, subtype);
    e.mem.set_u32(rule + 8, thread);
    hk_job_queue_update_job_query_rules(e, this);
}

// Translated from 00d04a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): under the lock, makes room for `wanted`
/// jobs in the ring `+0x30` + `offset` of the shared block and in the ring of every extra rule
/// (the queue index byte at `+0x3d + 8 * i`).
pub fn fn_00d04a80(e: &mut Engine, this: Ptr, offset: u32, wanted: i32) {
    let base = this.addr();
    let data = hk_job_queue_lock_queue(e, this, 0);
    let index = e.mem.u32(base + 0x30).wrapping_add(offset);
    fn_00d056d0(e, Ptr::new(ring_of(data, index)), wanted);
    let mut rule = 0i32;
    if e.mem.i32(base + 0x48) > 0 {
        let mut entry = base + 0x3d;
        loop {
            let index = u32::from(e.mem.u8(entry));
            fn_00d056d0(e, Ptr::new(ring_of(data, index)), wanted);
            rule += 1;
            entry = entry.wrapping_add(8);
            if rule >= e.mem.i32(base + 0x48) {
                break;
            }
        }
    }
    hk_job_queue_unlock_queue(e, this, data);
}

// Translated from 00d04b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::addJobQueueLocked` (Xbox PDB), `thiscall`, `RET 0xc`: with the lock held, puts
/// the job on the back (`flag` not 0) or the front of its queue's ring and wakes a waiting
/// thread of that queue.
pub fn hk_job_queue_add_job_queue_locked(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    job: Ptr,
    flag: u32,
) {
    let index = hk_job_queue_get_queue_index_for_job(e, this, job);
    let ring = ring_of(data.addr(), index);
    if flag == 0 {
        ring_push_front(e, ring, job.addr());
    } else {
        ring_push_back(e, ring, job.addr());
    }
    hk_job_queue_release_one_waiting_thread(e, this, index as i32, data);
}

// Translated from 00d04b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::addJob` (Xbox PDB), `thiscall`, `RET 8`: takes the lock, adds the job
/// (`addJobQueueLocked`), releases the lock.
pub fn hk_job_queue_add_job(e: &mut Engine, this: Ptr, job: Ptr, flag: u32) {
    let data = hk_job_queue_lock_queue(e, this, 0);
    hk_job_queue_add_job_queue_locked(e, this, Ptr::new(data), job, flag);
    hk_job_queue_unlock_queue(e, this, data);
}

// Translated from 00d04b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::addJob_ov2` (Xbox PDB), `thiscall`, `RET 8`: copies the job (its size is the
/// `u16` at `+4`) into a 128-byte stack buffer and adds that copy (`addJob`). The bytes of the
/// buffer beyond the job are stack garbage in the exe and zeros here.
pub fn hk_job_queue_add_job_ov2(e: &mut Engine, this: Ptr, job: Ptr, flag: u32) {
    e.with_stack(JOB_BYTES, |e, copy| {
        let size = u32::from(e.mem.u16(job.addr() + 4));
        e.call(MEM_COPY, &args![copy, job, size]);
        hk_job_queue_add_job(e, this, copy, flag);
    });
}

// Translated from 00d04bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::addJobBatch` (Xbox PDB), `thiscall`, `RET 8`: writes the `"TtAddJobBatch"`
/// monitor record, takes the lock and, for every job pointer of the `hkArray` at `batch`
/// (data, size), copies the job (size from its `u16` at `+4`, `00cb1260`) to a stack buffer,
/// puts it on the back (`flag` not 0) or front of its queue's ring and wakes a waiting thread
/// of that queue; then releases the lock and writes the closing record.
pub fn hk_job_queue_add_job_batch(e: &mut Engine, this: Ptr, batch: Ptr, flag: u32) {
    profile_record(e, LABEL_ADD_JOB_BATCH);
    let data = hk_job_queue_lock_queue(e, this, 0);
    let mut position = 0i32;
    if e.mem.i32(batch.addr() + 4) > 0 {
        loop {
            e.with_stack(JOB_BYTES, |e, copy| {
                let jobs = e.mem.u32(batch.addr());
                let source = e
                    .mem
                    .u32(jobs.wrapping_add((position as u32).wrapping_mul(4)));
                let size = u32::from(e.mem.u16(source + 4));
                e.call(MEM_COPY, &args![copy, source, size]);
                let index = hk_job_queue_get_queue_index_for_job(e, this, copy);
                ring_push(e, data, index, flag == 0, copy.addr());
                hk_job_queue_release_one_waiting_thread(e, this, index as i32, Ptr::new(data));
            });
            position += 1;
            if position >= e.mem.i32(batch.addr() + 4) {
                break;
            }
        }
    }
    hk_job_queue_unlock_queue(e, this, data);
    profile_record(e, LABEL_END);
}

/// What the finish hook (`+0x20`) is given: `(queue, shared block, job, address of a 0x90-byte
/// answer)`. When it returns 0 the answer holds a flag word at `+0` and a job at `+0x10`.
fn call_finish_hook(e: &mut Engine, this: u32, data: u32, job: u32, answer: u32) -> u32 {
    let hook = e.mem.u32(this + 0x20);
    e.call(hook, &args![this, data, job, answer]).u32()
}

// Translated from 00d04d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::finishJob` (Xbox PDB), `thiscall`, `RET 8`: writes the `"TtfinishJob"` record
/// and takes the lock; asks the finish hook (`+0x20`) about the finished job `job`, and when it
/// answers 0 puts the job it left in its answer buffer on the front (answer flag 0) or back of
/// its queue and wakes a waiting thread of that queue; when `keep` is 0 takes one off the
/// in-progress count of the finished job's type (`u16` at `data + type * 2`); releases the
/// lock and writes the closing record.
pub fn hk_job_queue_finish_job(e: &mut Engine, this: Ptr, job: Ptr, keep: u32) {
    profile_record(e, LABEL_FINISH_JOB);
    let data = hk_job_queue_lock_queue(e, this, 0);
    e.with_stack(0x90, |e, answer| {
        let result = call_finish_hook(e, this.addr(), data, job.addr(), answer.addr());
        if result == 0 {
            let next = answer.addr() + 0x10;
            let index = hk_job_queue_get_queue_index_for_job(e, this, Ptr::new(next));
            let front = e.mem.u32(answer.addr()) == 0;
            ring_push(e, data, index, front, next);
            hk_job_queue_release_one_waiting_thread(e, this, index as i32, Ptr::new(data));
        }
    });
    if keep == 0 {
        let job_type = u32::from(e.mem.u8(job.addr() + 1));
        decrement_u16(e, data.wrapping_add(job_type * 2));
    }
    hk_job_queue_unlock_queue(e, this, data);
    profile_record(e, LABEL_END);
}

// Translated from 00d04e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`; the get-next-job loop with an optional
/// finished job): writes the `"TtGetNextJob"` record, then repeats: take the lock; if
/// `finished` is not null, ask the finish hook (`+0x20`) about it, and when that answers 0 put
/// the job it leaves on the front or back of its queue (no wake-up), take one off the
/// in-progress count of the finished job's type and forget the finished job; then look for a
/// job (`findJobInternal` with the queue just fed, or -1, the `no_wait` word and `job_out`)
/// until it answers something other than -1. Writes the closing record and answers it.
pub fn fn_00d04e80(
    e: &mut Engine,
    this: Ptr,
    mut finished: u32,
    job_out: Ptr,
    no_wait: u32,
) -> u32 {
    profile_record(e, LABEL_GET_NEXT_JOB);
    loop {
        let data = hk_job_queue_lock_queue(e, this, 0);
        let mut fed: i32 = -1;
        if finished != 0 {
            e.with_stack(0x90, |e, answer| {
                let result = call_finish_hook(e, this.addr(), data, finished, answer.addr());
                if result == 0 {
                    let next = answer.addr() + 0x10;
                    let index = hk_job_queue_get_queue_index_for_job(e, this, Ptr::new(next));
                    fed = index as i32;
                    let front = e.mem.u32(answer.addr()) == 0;
                    ring_push(e, data, index, front, next);
                }
            });
            let job_type = u32::from(e.mem.u8(finished + 1));
            decrement_u16(e, data.wrapping_add(job_type * 2));
            finished = 0;
        }
        let found = hk_job_queue_find_job_internal(e, this, fed, Ptr::new(data), no_wait, job_out);
        if found != 0xffff_ffff {
            profile_record(e, LABEL_END);
            return found;
        }
    }
}

// Translated from 00d05020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::finishAddAndGetNextJob` (Xbox PDB), `RET 0x10` (not a method: the queue is in
/// `ECX`, set by the caller, so it comes first here): writes the `"TtGetNextJob"` record, then
/// repeats under the lock: the first time, takes one off the `u16` at `data + type_slot * 2`,
/// puts `job` on the front (`flag` 0) or back of its queue (no wake-up); then looks for a job
/// (`findJobInternal` with the queue just fed, or -1, `no_wait` and `job`) until it answers
/// something other than -1. Writes the closing record and answers it.
pub fn hk_job_queue_finish_add_and_get_next_job(
    e: &mut Engine,
    this: Ptr,
    type_slot: u32,
    flag: u32,
    job: Ptr,
    no_wait: u32,
) -> u32 {
    profile_record(e, LABEL_GET_NEXT_JOB);
    let mut first = true;
    loop {
        let data = hk_job_queue_lock_queue(e, this, 0);
        let mut fed: i32 = -1;
        if first {
            decrement_u16(e, data.wrapping_add(type_slot.wrapping_mul(2)));
            let index = hk_job_queue_get_queue_index_for_job(e, this, job);
            fed = index as i32;
            ring_push(e, data, index, flag == 0, job.addr());
            first = false;
        }
        let found = hk_job_queue_find_job_internal(e, this, fed, Ptr::new(data), no_wait, job);
        if found != 0xffff_ffff {
            profile_record(e, LABEL_END);
            return found;
        }
    }
}

// Translated from 00d05190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the get-next-job loop without a finished
/// job (`00d04e80` with 0), into `job_out` with the `no_wait` word.
pub fn fn_00d05190(e: &mut Engine, this: Ptr, job_out: Ptr, no_wait: u32) -> u32 {
    fn_00d04e80(e, this, 0, job_out, no_wait)
}

// Translated from 00d051b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::processAllJobs` (Xbox PDB), `fastcall`: fetches jobs without waiting for the
/// first (`00d05190`) and runs them until a fetch or a job function answers something other
/// than 0, which it returns. Each job is run by function `byte 0` of the table of its type
/// (`byte 1`, at `this + (type + 0xf) * 0x10`), called cdecl with `(queue, job)`, between a
/// monitor record named for the type (`"TtPhysics"` for 0 and 1, `"TtCollision Query"`,
/// `"TtRayCast Query"`, `"TtAnimation"`, `"TtBehavior"`, `"TtCloth"`, `"TtPathfinding"`,
/// `"TtCharacter Proxy"` for 9, `"TtUserJob"` for 10, `"TtUnknownJobs"` otherwise) and the
/// closing record.
pub fn hk_job_queue_process_all_jobs(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(JOB_BYTES, |e, job| {
        let mut result = fn_00d05190(e, this, job, 0);
        loop {
            if result != 0 {
                return result;
            }
            let job_type = u32::from(e.mem.u8(job.addr() + 1));
            let label = JOB_TYPE_LABELS
                .get(job_type as usize)
                .copied()
                .unwrap_or(LABEL_UNKNOWN_JOBS);
            profile_record(e, label);
            let table = e.mem.u32(
                job_type
                    .wrapping_add(0xf)
                    .wrapping_shl(4)
                    .wrapping_add(this.addr()),
            );
            let function = e
                .mem
                .u32(table.wrapping_add(u32::from(e.mem.u8(job.addr())).wrapping_mul(4)));
            result = e.call(function, &args![this, job]).u32();
            profile_record(e, LABEL_END);
        }
    })
}

// Translated from 00d05450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): the default constructor of the hardware set-up
/// (`hkJobQueueHwSetup`, from the neighbouring operator's name): sets the word at `+0` to 1, the
/// byte at `+4` to 1, the word at `+8` to what `00cbb960` stores in a local that held `this`
/// when it was called, and an empty array of arrays at `+0xc` (data 0, size 0, capacity word
/// `0x80000000`). Answers `this`.
pub fn fn_00d05450(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, EMPTY_CAPACITY);
    e.mem.set_u32(base, 1);
    let count = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), base);
        e.call(THREAD_COUNT_QUERY, &args![slot]);
        e.mem.u32(slot.addr())
    });
    e.mem.set_u32(base + 8, count);
    e.mem.set_u8(base + 4, 1);
    this
}

// Translated from 00d05490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::hkJobQueue` (Xbox PDB), `RET 4`: constructs the critical section (`00cf6da0`
/// with 0) and the hardware set-up copy at `+0x58` (`00d05450`), empties the rule array,
/// clears the thread-number TLS slot (`TlsSetValue`), allocates the 0x170-byte shared block
/// and constructs its 16 rings (`00e46e80`), clears the semaphore flag and the shared block's
/// flag word and byte, copies `setup` into the copy (`00d05b50`) and takes the thread count
/// (`+0x18` of `setup`) for `+0x24`, makes room for 128 jobs in every ring (`00d056d0`), clears
/// the mask, the counts and the bits (setting one bit per thread), sets the job-type table's
/// default hooks and the two queue hooks, and computes the queues (`updateJobQueryRules`).
/// Answers `this`.
pub fn hk_job_queue_hk_job_queue(e: &mut Engine, this: Ptr, setup: Ptr) -> Ptr {
    let base = this.addr();
    e.call(CRITICAL_SECTION_CONSTRUCT, &args![this, 0u32]);
    fn_00d05450(e, Ptr::new(base + 0x58));
    e.mem.set_u32(base + 0x1a0, 0);
    e.mem.set_u32(base + 0x1a4, 0);
    e.mem.set_u32(base + 0x1a8, EMPTY_CAPACITY);
    let thread_slot: u32 = e.global(THREAD_INDEX_SLOT);
    e.call(TLS_SET_VALUE, &args![thread_slot, 0u32]);
    let shared = heap_alloc(e, 0x170);
    if shared != 0 {
        let mut ring = shared + RINGS_OFFSET;
        for _ in 0..16 {
            e.call(RING_CONSTRUCT, &args![ring]);
            ring = ring.wrapping_add(RING_BYTES);
        }
    }
    e.mem.set_u32(base + 0x18, shared);
    e.mem.set_u8(base + 0x74, 0);
    let data = e.mem.u32(base + 0x18);
    e.mem.set_u8(data.wrapping_add(0x20), 0);
    let data = e.mem.u32(base + 0x18);
    e.mem.set_u32(data.wrapping_add(0x1c), 0);
    hk_job_queue_hw_setup_operator_(e, Ptr::new(base + 0x58), setup);
    let threads = e.mem.u32(setup.addr() + 0x18);
    e.mem.set_u32(base + 0x24, threads);
    e.mem.set_u32(base + 0x70, 0);
    let mut offset = 0u32;
    while offset < 0x140 {
        let data = e.mem.u32(base + 0x18);
        fn_00d056d0(
            e,
            Ptr::new(data.wrapping_add(offset).wrapping_add(RINGS_OFFSET)),
            0x80,
        );
        offset += RING_BYTES;
    }
    let data = e.mem.u32(base + 0x18);
    e.mem.set_u32(data.wrapping_add(0x18), 0);
    let mut thread = 0u32;
    if e.mem.i32(base + 0x24) > 0 {
        let mut entry = base + 0xf0;
        loop {
            e.mem.set_u32(entry + 4, 0);
            e.mem.set_u32(entry, 0);
            e.mem.set_u32(entry + 8, DEFAULT_TYPE_HOOK_A);
            e.mem.set_u32(entry + 0xc, DEFAULT_TYPE_HOOK_B);
            let data = e.mem.u32(base + 0x18);
            e.mem.set_u16(data.wrapping_add(thread.wrapping_mul(2)), 0);
            let data = e.mem.u32(base + 0x18);
            let bit = 1u32.wrapping_shl(thread & 0x1f);
            thread += 1;
            entry = entry.wrapping_add(0x10);
            let mask = e.mem.u32(data.wrapping_add(0x18)) | bit;
            e.mem.set_u32(data.wrapping_add(0x18), mask);
            if (thread as i32) >= e.mem.i32(base + 0x24) {
                break;
            }
        }
    }
    e.mem.set_u32(base + 0x1c, DEFAULT_POP_HOOK);
    e.mem.set_u32(base + 0x20, DEFAULT_FINISH_HOOK);
    e.mem.set_u32(base + 0x8c, 0);
    hk_job_queue_update_job_query_rules(e, this);
    this
}

// Translated from 00d055c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`; the destructor's body): when the shared block exists
/// destroys its 16 rings (`00d056a0`, last first) and frees the 0x170 bytes; when the
/// semaphores were created destroys (`00d04370`) and frees each; frees the rule array (when it
/// owns its storage), empties it, destroys the hardware set-up copy (`00cbf490`) and deletes
/// the critical section (`DeleteCriticalSection`).
pub fn fn_00d055c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let shared = e.mem.u32(base + 0x18);
    if shared != 0 {
        let mut ring = shared + 0x16c;
        let mut remaining = 0xfi32;
        loop {
            ring = ring.wrapping_sub(RING_BYTES);
            fn_00d056a0(e, Ptr::new(ring));
            remaining -= 1;
            if remaining < 0 {
                break;
            }
        }
        heap_free(e, shared, 0x170);
    }
    if e.mem.u8(base + 0x74) != 0 {
        let mut index = 0i32;
        if e.mem.i32(base + 0x8c) > 0 {
            let mut slot = base + 0x78;
            loop {
                let semaphore = e.mem.u32(slot);
                if semaphore != 0 {
                    e.call(SEMAPHORE_DESTRUCT, &args![semaphore]);
                    heap_free(e, semaphore, 4);
                }
                index += 1;
                slot = slot.wrapping_add(4);
                if index >= e.mem.i32(base + 0x8c) {
                    break;
                }
            }
        }
    }
    let allocator = heap_allocator(e);
    let capacity = e.mem.u32(base + 0x1a8);
    e.mem.set_u32(base + 0x1a4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0x1a0);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(12)],
        );
    }
    e.mem.set_u32(base + 0x1a0, 0);
    e.mem.set_u32(base + 0x1a8, EMPTY_CAPACITY);
    e.call(HW_SETUP_DESTRUCT, &args![base + 0x58]);
    e.call(DELETE_CRITICAL_SECTION, &args![this]);
}

// Translated from 00d056a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): destroys a job ring: when its capacity (`+4`) is not
/// 0 frees the buffer (`+0`) with `capacity * 128` bytes.
pub fn fn_00d056a0(e: &mut Engine, ring: Ptr) {
    let capacity = e.mem.u32(ring.addr() + 4);
    if capacity != 0 {
        let buffer = e.mem.u32(ring.addr());
        heap_free(e, buffer, capacity << 7);
    }
}

// Translated from 00d056d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): makes room in a job ring (buffer `+0`,
/// capacity `+4`, head `+8`, tail `+0xc`, count `+0x10`) for `wanted` jobs: when the capacity is
/// smaller, takes the larger of `wanted` and twice the capacity, allocates that many 128-byte
/// jobs, copies the live jobs to the start of the new buffer (in two pieces when they wrap),
/// frees the old buffer, and stores the new buffer and capacity. A ring that had a buffer has
/// its head reset to 0 and its tail set to its count.
pub fn fn_00d056d0(e: &mut Engine, ring: Ptr, wanted: i32) {
    let ring = ring.addr();
    let capacity = e.mem.i32(ring + 4);
    if capacity >= wanted {
        return;
    }
    let doubled = capacity.wrapping_add(capacity);
    let wanted = if doubled >= wanted { doubled } else { wanted };
    let buffer = heap_alloc(e, (wanted as u32) << 7);
    let old = e.mem.u32(ring);
    if old != 0 {
        let count = e.mem.i32(ring + 0x10);
        if count != 0 {
            let tail = e.mem.i32(ring + 0xc);
            let head = e.mem.i32(ring + 8);
            let source = ((head as u32) << 7).wrapping_add(old);
            if tail > head {
                e.call(MEM_COPY, &args![buffer, source, (count as u32) << 7]);
            } else {
                let first = (e.mem.u32(ring + 4).wrapping_sub(head as u32)) << 7;
                e.call(MEM_COPY, &args![buffer, source, first]);
                let rest = e.mem.u32(ring);
                e.call(
                    MEM_COPY,
                    &args![first.wrapping_add(buffer), rest, (tail as u32) << 7],
                );
            }
        }
        e.mem.set_u32(ring + 8, 0);
        let count = e.mem.u32(ring + 0x10);
        e.mem.set_u32(ring + 0xc, count);
    }
    let old_capacity = e.mem.u32(ring + 4);
    if old_capacity != 0 {
        let old = e.mem.u32(ring);
        heap_free(e, old, old_capacity << 7);
    }
    e.mem.set_u32(ring, buffer);
    e.mem.set_u32(ring + 4, wanted as u32);
}

// Translated from 00d057a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): puts the 128-byte job at `job` on the back
/// of the ring (growing it to 8 jobs, or twice its capacity, when full).
pub fn fn_00d057a0(e: &mut Engine, ring: Ptr, job: Ptr) {
    ring_push_back(e, ring.addr(), job.addr());
}

// Translated from 00d057f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): puts the 128-byte job at `job` on the front
/// of the ring (growing it to 8 jobs, or twice its capacity, when full).
pub fn fn_00d057f0(e: &mut Engine, ring: Ptr, job: Ptr) {
    ring_push_front(e, ring.addr(), job.addr());
}

/// What the pop hook (`+0x1c`) is given: `(queue, shared block, job, word)`.
fn call_pop_hook(e: &mut Engine, this: u32, data: u32, job: u32, word: u32) -> u32 {
    let hook = e.mem.u32(this + 0x1c);
    e.call(hook, &args![this, data, job, word]).u32()
}

// Translated from 00d05840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueue::findJobInternal` (Xbox PDB), `thiscall`, `RET 0x10`: looks for a job for the
/// calling thread with the lock held (`data` the shared block, the lock released before it
/// returns unless it answers -1 after waiting). `fed` is the queue the caller just added to
/// (-1 for none), `no_wait` a word compared with 1, `job_out` the word the pop hook gets.
///
/// The thread's queue list (`+0x90 + 18 * queue`, ended by `0xff`) is walked for a ring with a
/// job; the job is taken from the ring's head, shown to the pop hook (`+0x1c`; when that
/// answers 1 the job goes back on the front and a waiting thread is woken), and the in-progress
/// count of its type goes up; answers 0. A `fed` queue other than the one found gets a waiting
/// thread woken. With no job: if the rings from `+0x4c` to `+0x50` hold any, or the bit mask
/// `data + 0x18` has no bit of a ring or waiting count in use, or the flag word at `+0x1c` is
/// 1 and no ring has a job, the call goes on to `no_wait`: 1 answers 1 at once; otherwise the
/// thread counts itself as waiting, waits on its semaphore (writing the `"TtNoJobAvailable"`
/// record around it) and answers -1. The other outcomes wake every waiting thread and answer 2.
pub fn hk_job_queue_find_job_internal(
    e: &mut Engine,
    this: Ptr,
    fed: i32,
    data: Ptr,
    no_wait: u32,
    job_out: Ptr,
) -> u32 {
    let base = this.addr();
    let data = data.addr();
    let thread_slot: u32 = e.global(THREAD_INDEX_SLOT);
    let number = e.call(TLS_GET_VALUE, &args![thread_slot]).u32();
    let own_queue = if number == 0 {
        e.mem.i32(base + 0x54)
    } else {
        let number = e.call(TLS_GET_VALUE, &args![thread_slot]).u32();
        i32::from(e.mem.i8(number.wrapping_add(base).wrapping_add(0xea)))
    };
    let list_offset = own_queue.wrapping_mul(9).wrapping_add(0x48) as u32;
    let mut list = base.wrapping_add(list_offset.wrapping_mul(2));
    let mut current = i32::from(e.mem.i8(list));
    // The queue a job was taken from, or -1.
    let mut taken: i32 = -1;
    while current >= 0 {
        let ring = ring_of(data, current as u32);
        list = list.wrapping_add(1);
        if e.mem.i32(ring + 0x10) == 0 {
            current = i32::from(e.mem.i8(list));
            continue;
        }
        e.with_stack(JOB_BYTES, |e, job| {
            let head = e.mem.u32(ring + 8);
            let source = (head << 7).wrapping_add(e.mem.u32(ring));
            copy_job(e, job.addr(), source);
            let head = head.wrapping_add(1);
            e.mem.set_u32(ring + 8, head);
            if head == e.mem.u32(ring + 4) {
                e.mem.set_u32(ring + 8, 0);
            }
            let count = e.mem.u32(ring + 0x10).wrapping_sub(1);
            e.mem.set_u32(ring + 0x10, count);
            let answer = call_pop_hook(e, base, data, job.addr(), job_out.addr());
            if answer == 1 {
                ring_push_front(e, ring, job.addr());
                hk_job_queue_release_one_waiting_thread(e, this, current, Ptr::new(data));
            }
            let job_type = u32::from(e.mem.u8(job.addr() + 1));
            increment_u16(e, data.wrapping_add(job_type * 2));
        });
        taken = current;
        break;
    }
    if fed != -1 && fed != taken {
        hk_job_queue_release_one_waiting_thread(e, this, fed, Ptr::new(data));
    }
    if taken >= 0 {
        hk_job_queue_unlock_queue(e, this, data);
        return 0;
    }
    // No job: add up the jobs of the rings from +0x4c to +0x50.
    let mut index = e.mem.i32(base + 0x4c);
    let end = e.mem.i32(base + 0x50);
    let mut sum_even = 0u32;
    let mut sum_odd = 0u32;
    if end.wrapping_sub(index) >= 2 {
        let mut pairs = ((end.wrapping_sub(index).wrapping_sub(2) as u32) >> 1).wrapping_add(1);
        let mut count = data.wrapping_add(
            (index as u32)
                .wrapping_mul(5)
                .wrapping_add(0x14)
                .wrapping_mul(4),
        );
        index = index.wrapping_add((pairs as i32).wrapping_mul(2));
        loop {
            sum_even = sum_even.wrapping_add(e.mem.u32(count.wrapping_sub(0x14)));
            sum_odd = sum_odd.wrapping_add(e.mem.u32(count));
            count = count.wrapping_add(0x28);
            pairs = pairs.wrapping_sub(1);
            if pairs == 0 {
                break;
            }
        }
    }
    let last = if index < e.mem.i32(base + 0x50) {
        e.mem.u32(
            data.wrapping_add(
                (index as u32)
                    .wrapping_mul(5)
                    .wrapping_add(0xf)
                    .wrapping_mul(4),
            ),
        )
    } else {
        0
    };
    let total = sum_even.wrapping_add(sum_odd).wrapping_add(last);
    if total == 0 {
        // Which threads have work: a bit per thread whose in-progress count is positive or
        // whose own ring has jobs.
        let mut mask = 0u32;
        let threads = e.mem.i32(base + 0x24);
        let mut thread = 0i32;
        if threads > 0 {
            loop {
                let in_use = if e
                    .mem
                    .i16(data.wrapping_add((thread as u32).wrapping_mul(2)))
                    > 0
                {
                    1
                } else {
                    let ring_index = e
                        .mem
                        .u32(base + 0x30)
                        .wrapping_add(thread as u32)
                        .wrapping_add(3)
                        .wrapping_mul(5);
                    let jobs = e.mem.u32(data.wrapping_add(ring_index.wrapping_mul(4)));
                    u32::from(jobs != 0)
                };
                mask |= in_use.wrapping_shl(thread as u32 & 0x1f);
                thread += 1;
                if thread >= threads {
                    break;
                }
            }
        }
        let no_overlap = e.mem.u32(data + 0x18) & mask == 0;
        let mask_is_zero = mask == 0;
        if no_overlap {
            let number = e.call(TLS_GET_VALUE, &args![thread_slot]).u32();
            if number == 0 {
                if mask_is_zero {
                    fn_00d044f0(e, this, Ptr::new(data));
                }
                hk_job_queue_unlock_queue(e, this, data);
                return 2;
            }
            let own = i32::from(e.mem.i8(base + 0xea));
            let slot = data
                .wrapping_add((own as u32).wrapping_mul(2))
                .wrapping_add(0x22);
            if e.mem.u16(slot) != 0 {
                decrement_u16(e, slot);
                let own = i32::from(e.mem.i8(base + 0xea));
                let semaphore = e.mem.u32(
                    base.wrapping_add((own as u32).wrapping_mul(4))
                        .wrapping_add(0x78),
                );
                release_semaphore(e, semaphore);
            }
        }
        if mask_is_zero && e.mem.u32(data + 0x1c) != 1 {
            fn_00d044f0(e, this, Ptr::new(data));
            hk_job_queue_unlock_queue(e, this, data);
            return 2;
        }
    }
    // A job may be available: wait for one unless the caller asked not to.
    if no_wait == 1 {
        hk_job_queue_unlock_queue(e, this, data);
        return 1;
    }
    let number = e.call(TLS_GET_VALUE, &args![thread_slot]).u32();
    let own = i32::from(e.mem.i8(number.wrapping_add(base).wrapping_add(0xea)));
    increment_u16(e, waiting_slot(data, own));
    hk_job_queue_unlock_queue(e, this, data);
    profile_record(e, LABEL_NO_JOB_AVAILABLE);
    let semaphore = e.mem.u32(
        base.wrapping_add((own as u32).wrapping_mul(4))
            .wrapping_add(0x78),
    );
    fn_00d043b0(e, Ptr::new(semaphore));
    profile_record(e, LABEL_END);
    0xffff_ffff
}

// Translated from 00d05b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkJobQueueHwSetup::operator_` (the engine map's name; the body is the copy assignment),
/// `thiscall`, `RET 4`: copies the words at `+0` and `+8` and the byte at `+4` from `other`,
/// then makes the array of arrays at `+0xc` (data, size `+0x10`, capacity word `+0x14`) an
/// element-wise copy of `other`'s: the array grows through `_reserve` (`00c908c0`, element size
/// 12) when its capacity is below `other`'s size, extra elements are emptied (their storage
/// freed), shared elements keep their storage when it fits (else it is freed and re-allocated
/// through the allocator's buffer functions, the new capacity being the granted bytes / 4) and
/// get their size and ints copied, and missing elements are constructed as copies. Answers
/// `this`.
pub fn hk_job_queue_hw_setup_operator_(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let target = this.addr();
    let other = other.addr();
    let word = e.mem.u32(other);
    e.mem.set_u32(target, word);
    let byte = e.mem.u8(other + 4);
    e.mem.set_u8(target + 4, byte);
    let word = e.mem.u32(other + 8);
    e.mem.set_u32(target + 8, word);
    let array = target + 0xc;
    let allocator = heap_allocator(e);
    let old_size = e.mem.i32(array + 4);
    let new_size = e.mem.i32(other + 0x10);
    let common = if new_size > old_size {
        old_size
    } else {
        new_size
    };
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let count = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, count as u32, 12u32]);
    }
    // Drop the elements beyond the new size.
    let data = e.mem.u32(array);
    let mut excess = old_size.wrapping_sub(new_size);
    let element = data.wrapping_add((new_size as u32).wrapping_mul(12));
    if excess > 0 {
        let mut slot = element + 8;
        loop {
            let allocator = heap_allocator(e);
            let capacity = e.mem.u32(slot);
            e.mem.set_u32(slot - 4, 0);
            if (capacity as i32) >= 0 {
                let block = e.mem.u32(slot - 8);
                e.vcall(
                    allocator,
                    0x10,
                    &args![block, (capacity & CAPACITY_MASK).wrapping_mul(4)],
                );
            }
            e.mem.set_u32(slot - 8, 0);
            e.mem.set_u32(slot, EMPTY_CAPACITY);
            slot = slot.wrapping_add(12);
            excess -= 1;
            if excess == 0 {
                break;
            }
        }
    }
    // Copy into the elements both arrays have.
    let other_data = e.mem.u32(other + 0xc);
    let data = e.mem.u32(array);
    if common > 0 {
        let mut from = other_data.wrapping_add(4);
        let mut slot = data.wrapping_add(8);
        let mut remaining = common;
        loop {
            let allocator = heap_allocator(e);
            let capacity_word = e.mem.u32(slot);
            let capacity = capacity_word & CAPACITY_MASK;
            if (capacity as i32) < e.mem.i32(from) {
                if (capacity_word as i32) >= 0 {
                    let block = e.mem.u32(slot - 8);
                    e.vcall(allocator, 0x10, &args![block, capacity.wrapping_mul(4)]);
                }
                let wanted = e.mem.i32(from).wrapping_mul(4);
                e.with_stack(4, |e, request| {
                    e.mem.set_i32(request.addr(), wanted);
                    let block = e.vcall(allocator, 0xc, &args![request]).u32();
                    let granted = e.mem.i32(request.addr());
                    e.mem.set_u32(slot - 8, block);
                    e.mem.set_i32(slot, granted / 4);
                });
            }
            let size = e.mem.u32(from);
            let block = e.mem.u32(slot - 8);
            e.mem.set_u32(slot - 4, size);
            let source = e.mem.u32(from - 4);
            copy_ints(e, block, source, size as i32);
            from = from.wrapping_add(12);
            slot = slot.wrapping_add(12);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    // Construct the elements beyond the old size as copies.
    let data = e.mem.u32(array);
    let other_data = e.mem.u32(other + 0xc);
    let skip = (common as u32).wrapping_mul(12);
    let mut element = data.wrapping_add(skip);
    let mut from = other_data.wrapping_add(skip);
    let mut remaining = new_size.wrapping_sub(common);
    if remaining <= 0 {
        e.mem.set_i32(array + 4, new_size);
        return this;
    }
    loop {
        if element != 0 {
            e.mem.set_u32(element, 0);
            e.mem.set_u32(element + 4, 0);
            e.mem.set_u32(element + 8, EMPTY_CAPACITY);
            let wanted = e.mem.u32(from + 4);
            let allocator = heap_allocator(e);
            let (block, count, capacity_word) = if wanted == 0 {
                (0, 0, EMPTY_CAPACITY)
            } else {
                e.with_stack(4, |e, request| {
                    e.mem.set_u32(request.addr(), wanted.wrapping_mul(4));
                    let block = e.vcall(allocator, 0xc, &args![request]).u32();
                    let granted = e.mem.i32(request.addr());
                    let count = granted / 4;
                    let capacity_word = if count == 0 {
                        EMPTY_CAPACITY
                    } else {
                        count as u32
                    };
                    (block, count as u32, capacity_word)
                })
            };
            e.mem.set_u32(element, block);
            e.mem.set_u32(element + 4, count);
            e.mem.set_u32(element + 8, capacity_word);
            let source = e.mem.u32(from);
            copy_ints(e, block, source, count as i32);
        }
        element = element.wrapping_add(12);
        from = from.wrapping_add(12);
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    e.mem.set_i32(array + 4, new_size);
    this
}

/// Copies `count` words (forward) when `count` is positive.
fn copy_ints(e: &mut Engine, destination: u32, source: u32, count: i32) {
    for word in 0..count.max(0) as u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(destination.wrapping_add(4 * word), value);
    }
}

// Translated from 00d04550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `(queue, shared block, job, word)`): the default
/// pop hook the `hkJobQueue` constructor stores at `+0x1c`. A tail jump: it picks the job-type
/// table entry of the job's type (the byte at `job + 1`; entries are 16 bytes from `queue +
/// 0xf0`) and runs its hook A (`+8`, word `queue + 0xf8 + 16 * type`) with the same four words,
/// answering what that answers.
pub fn fn_00d04550(e: &mut Engine, this: Ptr, data: u32, job: Ptr, word: u32) -> u32 {
    let job_type = u32::from(e.mem.u8(job.addr().wrapping_add(1)));
    let hook = e
        .mem
        .u32(this.addr().wrapping_add(job_type << 4).wrapping_add(0xf8));
    e.call(hook, &args![this, data, job, word]).u32()
}

// Translated from 00d04570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `(queue, shared block, job, word)`): the default
/// finish hook the `hkJobQueue` constructor stores at `+0x20`. A tail jump like `00d04550`, to
/// hook B of the job's type (`+0xc` of its table entry, word `queue + 0xfc + 16 * type`) with the
/// same four words, answering what that answers.
pub fn fn_00d04570(e: &mut Engine, this: Ptr, data: u32, job: Ptr, word: u32) -> u32 {
    let job_type = u32::from(e.mem.u8(job.addr().wrapping_add(1)));
    let hook = e
        .mem
        .u32(this.addr().wrapping_add(job_type << 4).wrapping_add(0xfc));
    e.call(hook, &args![this, data, job, word]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d04550, fn_00d04550(Ptr, u32, Ptr, u32) -> u32),
        entry!(0x00d04570, fn_00d04570(Ptr, u32, Ptr, u32) -> u32),
        entry!(0x00d043b0, fn_00d043b0(Ptr) -> u32),
        entry!(0x00d04400, hk_job_queue_lock_queue(Ptr, u32) -> u32),
        entry!(0x00d04420, hk_job_queue_unlock_queue(Ptr, u32)),
        entry!(0x00d04440, fn_00d04440(Ptr) -> u32),
        entry!(0x00d04450, fn_00d04450(Ptr, u32)),
        entry!(0x00d044c0, fn_00d044c0(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x00d044f0, fn_00d044f0(Ptr, Ptr)),
        entry!(0x00d04590, hk_job_queue_update_job_query_rules(Ptr)),
        entry!(
            0x00d04890,
            hk_job_queue_get_queue_index_for_job(Ptr, Ptr) -> u32
        ),
        entry!(0x00d048e0, fn_00d048e0(Ptr, Ptr)),
        entry!(
            0x00d04990,
            hk_job_queue_release_one_waiting_thread(Ptr, i32, Ptr)
        ),
        entry!(0x00d049d0, fn_00d049d0(Ptr, u32)),
        entry!(0x00d04a20, fn_00d04a20(Ptr, u32, u8, u32)),
        entry!(0x00d04a80, fn_00d04a80(Ptr, u32, i32)),
        entry!(
            0x00d04b00,
            hk_job_queue_add_job_queue_locked(Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d04b50, hk_job_queue_add_job(Ptr, Ptr, u32)),
        entry!(0x00d04b90, hk_job_queue_add_job_ov2(Ptr, Ptr, u32)),
        entry!(0x00d04bd0, hk_job_queue_add_job_batch(Ptr, Ptr, u32)),
        entry!(0x00d04d80, hk_job_queue_finish_job(Ptr, Ptr, u32)),
        entry!(0x00d04e80, fn_00d04e80(Ptr, u32, Ptr, u32) -> u32),
        entry!(
            0x00d05020,
            hk_job_queue_finish_add_and_get_next_job(Ptr, u32, u32, Ptr, u32) -> u32
        ),
        entry!(0x00d05190, fn_00d05190(Ptr, Ptr, u32) -> u32),
        entry!(0x00d051b0, hk_job_queue_process_all_jobs(Ptr) -> u32),
        entry!(0x00d05450, fn_00d05450(Ptr) -> Ptr),
        entry!(0x00d05490, hk_job_queue_hk_job_queue(Ptr, Ptr) -> Ptr),
        entry!(0x00d055c0, fn_00d055c0(Ptr)),
        entry!(0x00d056a0, fn_00d056a0(Ptr)),
        entry!(0x00d056d0, fn_00d056d0(Ptr, i32)),
        entry!(0x00d057a0, fn_00d057a0(Ptr, Ptr)),
        entry!(0x00d057f0, fn_00d057f0(Ptr, Ptr)),
        entry!(
            0x00d05840,
            hk_job_queue_find_job_internal(Ptr, i32, Ptr, u32, Ptr) -> u32
        ),
        entry!(0x00d05b50, hk_job_queue_hw_setup_operator_(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A queue whose job-type table entry `job_type` has the hooks `hook_a` and `hook_b`, and a
    /// job of that type; the hooks are doubles answering 42 and 43.
    fn hook_setup_00d04550(job_type: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let queue = e.mem.alloc(0x200);
        let job = e.mem.alloc(0x80);
        e.mem.set_u8(job + 1, job_type as u8);
        e.mem.set_u32(queue + 0xf8 + 16 * job_type, 0x0900_0201);
        e.mem.set_u32(queue + 0xfc + 16 * job_type, 0x0900_0202);
        e.register(0x0900_0201, |_, _| Ret {
            eax: 42,
            ..Ret::default()
        });
        e.register(0x0900_0202, |_, _| Ret {
            eax: 43,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, queue, job)
    }

    #[test]
    fn fn_00d04550_jumps_to_hook_a_of_the_jobs_type() {
        let (mut e, queue, job) = hook_setup_00d04550(3);
        let answer = e
            .call(0x00d0_4550, &args![queue, 0x1111u32, job, 0x2222u32])
            .u32();
        assert_eq!(answer, 42);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0201)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0201, vec![queue, 0x1111, job, 0x2222])]);
    }

    #[test]
    fn fn_00d04570_jumps_to_hook_b_of_the_jobs_type() {
        let (mut e, queue, job) = hook_setup_00d04550(5);
        let answer = e
            .call(0x00d0_4570, &args![queue, 0x1111u32, job, 0x2222u32])
            .u32();
        assert_eq!(answer, 43);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0202)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0202, vec![queue, 0x1111, job, 0x2222])]);
    }

    const TLS_TABLE: u32 = 0x0127_0000;
    const ROUTER_CELL: u32 = 0x0127_0800;
    const ALLOCATOR_VTABLE: u32 = 0x0128_0000;
    const MONITOR_INDEX: u32 = 1;
    const THREAD_INDEX: u32 = 2;
    const POP_HOOK: u32 = 0x0900_0101;
    const FINISH_HOOK: u32 = 0x0900_0102;
    const JOB_FUNCTION: u32 = 0x0900_0103;
    const QUEUE_BYTES: u32 = 0x1b0;
    const SHARED_BYTES: u32 = 0x170;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(TLS_TABLE, 0x1000);
        e.set_global(0x0126_835c, MONITOR_INDEX);
        e.set_global(THREAD_INDEX_SLOT, THREAD_INDEX);
        // The allocator the router answers: slots 4 and 8 allocate and free by size, 0xc and
        // 0x10 take the size by address.
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[0, 0x0900_0001, 0x0900_0002, 0x0900_0003, 0x0900_0004],
        );
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| e.mem.u32(ROUTER_CELL).into_ret());
        e.register(0x0900_0001, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x0900_0002, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(0x0900_0003, |e, a| {
            let bytes = e.mem.u32(a[1]);
            e.mem.alloc(bytes).into_ret()
        });
        e.register(0x0900_0004, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(TLS_GET_VALUE, |e, a| {
            e.mem.u32(TLS_TABLE + 4 * a[0]).into_ret()
        });
        for address in [
            TLS_SET_VALUE,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            SEMAPHORE_RELEASE,
            SEMAPHORE_ACQUIRE,
            SEMAPHORE_DESTRUCT,
            CRITICAL_SECTION_CONSTRUCT,
            RING_CONSTRUCT,
            HW_SETUP_DESTRUCT,
        ] {
            e.register(address, noop);
        }
        e.register(SEMAPHORE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MEM_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        // The monitor stream: cursor at +4, limit at +0xc (room for 8 records).
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x60);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x60);
        e.mem.set_u32(TLS_TABLE + 4 * MONITOR_INDEX, stream);
        e.call_log = Some(vec![]);
        e
    }

    /// The thread number `TlsGetValue` answers for the thread-number index.
    fn set_thread_number(e: &mut Engine, number: u32) {
        e.mem.set_u32(TLS_TABLE + 4 * THREAD_INDEX, number);
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The monitor labels written so far (the first word of each 12-byte record).
    fn labels(e: &Engine) -> Vec<u32> {
        let stream = e.mem.u32(TLS_TABLE + 4 * MONITOR_INDEX);
        let cursor = e.mem.u32(stream + 4);
        let limit = e.mem.u32(stream + 0xc);
        (limit - 0x60..cursor)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }

    /// A queue object and its shared block, linked at `+0x18`.
    fn new_queue(e: &mut Engine) -> (u32, u32) {
        let queue = e.mem.alloc(QUEUE_BYTES);
        let data = e.mem.alloc(SHARED_BYTES);
        e.mem.set_u32(queue + 0x18, data);
        (queue, data)
    }

    fn ring(data: u32, index: u32) -> u32 {
        ring_of(data, index)
    }

    /// Gives the ring a buffer of `capacity` jobs and the state of a ring holding `count` jobs
    /// from `head`.
    fn fill_ring(e: &mut Engine, ring: u32, capacity: u32, head: u32, count: u32) {
        let buffer = e.mem.alloc(capacity * JOB_BYTES);
        e.mem.set_u32(ring, buffer);
        e.mem.set_u32(ring + 4, capacity);
        e.mem.set_u32(ring + 8, head);
        e.mem.set_u32(ring + 0xc, (head + count) % capacity);
        e.mem.set_u32(ring + 0x10, count);
    }

    /// A job: subtype and type bytes, its size word and a marker byte at `+8`.
    fn make_job(e: &mut Engine, subtype: u8, job_type: u8, marker: u8) -> u32 {
        let job = e.mem.alloc(JOB_BYTES);
        e.mem.set_u8(job, subtype);
        e.mem.set_u8(job + 1, job_type);
        e.mem.set_u16(job + 4, 0x20);
        e.mem.set_u8(job + 8, marker);
        job
    }

    fn marker_at(e: &Engine, ring: u32, slot: u32) -> u8 {
        e.mem.u8(e.mem.u32(ring) + slot * JOB_BYTES + 8)
    }

    fn semaphore_table(e: &mut Engine, queue: u32, count: u32) {
        e.mem.set_u32(queue + 0x8c, count);
        for i in 0..count {
            e.mem.set_u32(queue + 0x78 + 4 * i, 0x7000 + i);
        }
    }

    #[test]
    fn fn_00d043b0_acquires_the_semaphore() {
        let mut e = engine();
        e.call(0x00d0_43b0, &args![0x1234u32]);
        assert_eq!(calls(&e, SEMAPHORE_ACQUIRE), vec![vec![0x1234]]);
    }

    #[test]
    fn lock_queue_enters_the_section_and_answers_the_shared_block() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        assert_eq!(e.call(0x00d0_4400, &args![queue, 0u32]).u32(), data);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![queue]]);
    }

    #[test]
    fn unlock_queue_leaves_the_section() {
        let mut e = engine();
        e.call(0x00d0_4420, &args![0x4000u32, 0u32]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![0x4000]]);
    }

    #[test]
    fn fn_00d04440_reads_the_mask_word() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        e.mem.set_u32(data + 0x18, 0xa5);
        assert_eq!(e.call(0x00d0_4440, &args![queue]).u32(), 0xa5);
    }

    #[test]
    fn fn_00d04450_stores_the_mask_and_wakes_its_queue_when_a_thread_waits() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 4);
        e.mem.set_u8(queue + 0xea, 2);
        e.mem.set_u16(data + 0x22 + 4, 3);
        e.call(0x00d0_4450, &args![queue, 0xabu32]);
        assert_eq!(e.mem.u32(data + 0x18), 0xab);
        assert_eq!(e.mem.u16(data + 0x22 + 4), 2);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7002, 1]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![queue]]);
        // Nobody waiting: only the mask changes.
        e.mem.set_u16(data + 0x22 + 4, 0);
        e.call(0x00d0_4450, &args![queue, 0x11u32]);
        assert_eq!(e.mem.u32(data + 0x18), 0x11);
        assert_eq!(e.mem.u16(data + 0x22 + 4), 0);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE).len(), 1);
    }

    #[test]
    fn fn_00d044c0_fills_a_job_type_entry() {
        let mut e = engine();
        let (queue, _) = new_queue(&mut e);
        e.call(0x00d0_44c0, &args![queue, 2u32, 11u32, 22u32, 33u32, 44u32]);
        let entry = queue + (2 + 0xf) * 0x10;
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(entry + 4 * i)).collect();
        assert_eq!(words, vec![11, 22, 33, 44]);
    }

    #[test]
    fn fn_00d044f0_clears_the_waiting_counts_and_releases_each_thread() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 3);
        for (i, waiting) in [2u16, 0, 1].into_iter().enumerate() {
            e.mem.set_u16(data + 0x22 + 2 * i as u32, waiting);
        }
        e.call(0x00d0_44f0, &args![queue, data]);
        assert_eq!(
            calls(&e, SEMAPHORE_RELEASE),
            vec![vec![0x7000, 1], vec![0x7000, 1], vec![0x7002, 1]]
        );
        for i in 0..3 {
            assert_eq!(e.mem.u16(data + 0x22 + 2 * i), 0);
        }
    }

    #[test]
    fn update_job_query_rules_without_groups_uses_one_queue_per_thread_set() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        e.mem.set_u32(queue + 0x24, 2);
        e.mem.set_u32(queue + 0x60, 3);
        e.call(0x00d0_4590, &args![queue]);
        let word = |e: &Engine, offset: u32| e.mem.u32(queue + offset);
        assert_eq!(word(&e, 0x48), 0);
        assert_eq!(word(&e, 0x28), 2);
        assert_eq!(word(&e, 0x2c), 3);
        assert_eq!(word(&e, 0x30), 3);
        assert_eq!(word(&e, 0x34), 5);
        assert_eq!(word(&e, 0x4c), 2);
        assert_eq!(word(&e, 0x50), 3);
        // The main thread's queue is shared by the others, so it moves to its own (3).
        assert_eq!(word(&e, 0x54), 2);
        let table: Vec<u8> = (0..3).map(|i| e.mem.u8(queue + 0xea + i)).collect();
        assert_eq!(table, vec![3, 2, 2]);
        let list: Vec<u8> = (0..6).map(|i| e.mem.u8(queue + 180 + i)).collect();
        assert_eq!(list, vec![2, 3, 4, 0, 1, 0xff]);
        // Four semaphores, made with (0, 1000); the waiting counts are cleared.
        assert_eq!(word(&e, 0x8c), 4);
        assert_eq!(e.mem.u8(queue + 0x74), 1);
        let made = calls(&e, SEMAPHORE_CONSTRUCT);
        assert_eq!(made.len(), 4);
        for (i, call) in made.iter().enumerate() {
            assert_eq!(call[1..], [0, 1000]);
            assert_eq!(e.mem.u32(queue + 0x78 + 4 * i as u32), call[0]);
            assert_eq!(e.mem.u16(data + 0x22 + 2 * i as u32), 0);
        }
        // A second call destroys the four before making new ones.
        e.call(0x00d0_4590, &args![queue]);
        assert_eq!(calls(&e, SEMAPHORE_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, SEMAPHORE_CONSTRUCT).len(), 8);
    }

    #[test]
    fn update_job_query_rules_with_groups_and_a_rule() {
        let mut e = engine();
        let (queue, _) = new_queue(&mut e);
        let word = |e: &Engine, offset: u32| e.mem.u32(queue + offset);
        e.mem.set_u32(queue + 0x24, 1);
        e.mem.set_u8(queue + 0x5c, 1);
        e.mem.set_u32(queue + 0x58, 1);
        e.mem.set_u32(queue + 0x60, 3);
        e.mem.set_u32(queue + 0x68, 2);
        // Two groups: threads [0, 1] and [2].
        let groups = e.mem.alloc(24);
        let members = [e.mem.alloc(8), e.mem.alloc(4)];
        for (value, address) in [(0u32, members[0]), (1, members[0] + 4), (2, members[1])] {
            e.mem.set_u32(address, value);
        }
        e.mem.set_u32(groups, members[0]);
        e.mem.set_u32(groups + 4, 2);
        e.mem.set_u32(groups + 12, members[1]);
        e.mem.set_u32(groups + 16, 1);
        e.mem.set_u32(queue + 0x64, groups);
        // One rule: type word 0x11, subtype 0x22, thread 1.
        let rules = e.mem.alloc(12);
        e.mem.set_u32(rules, 0x11);
        e.mem.set_u8(rules + 4, 0x22);
        e.mem.set_u32(rules + 8, 1);
        e.mem.set_u32(queue + 0x1a0, rules);
        e.mem.set_u32(queue + 0x1a4, 1);
        e.call(0x00d0_4590, &args![queue]);
        assert_eq!(
            [0x48, 0x28, 0x2c, 0x30, 0x34, 0x4c, 0x50, 0x54, 0x8c].map(|o| word(&e, o)),
            [1, 0, 2, 3, 4, 0, 3, 0, 3]
        );
        let bytes = |e: &Engine, offset: u32, count: u32| -> Vec<u8> {
            (0..count).map(|i| e.mem.u8(queue + offset + i)).collect()
        };
        // Threads 0 and 1 are in group 0, thread 2 in group 1; the rule moves thread 1.
        assert_eq!(bytes(&e, 0xea, 3), vec![0, 2, 1]);
        assert_eq!(bytes(&e, 0x90, 5), vec![0, 1, 3, 2, 0xff]);
        assert_eq!(bytes(&e, 0xa2, 5), vec![1, 0, 3, 2, 0xff]);
        // The rule's own list leaves out the rule's queue itself.
        assert_eq!(bytes(&e, 0xb4, 5), vec![2, 0, 1, 3, 0xff]);
        // The rule copy: type word, subtype, queue index.
        assert_eq!(word(&e, 0x38), 0x11);
        assert_eq!(bytes(&e, 0x3c, 2), vec![0x22, 2]);
    }

    #[test]
    fn get_queue_index_for_job_matches_rules_or_falls_back_to_the_type() {
        let mut e = engine();
        let (queue, _) = new_queue(&mut e);
        e.mem.set_u32(queue + 0x48, 2);
        e.mem.set_u32(queue + 0x30, 0x10);
        for (i, (job_type, subtype, index)) in [(5u8, 7u8, 9u8), (6, 8, 4)].into_iter().enumerate()
        {
            let entry = queue + 0x38 + 8 * i as u32;
            e.mem.set_u32(entry, u32::from(job_type));
            e.mem.set_u8(entry + 4, subtype);
            e.mem.set_u8(entry + 5, index);
        }
        let mut index = |subtype, job_type| {
            let job = make_job(&mut e, subtype, job_type, 0);
            e.call(0x00d0_4890, &args![queue, job]).u32()
        };
        assert_eq!(index(8, 6), 4);
        assert_eq!(index(7, 5), 9);
        assert_eq!(index(1, 3), 0x13);
        // The type matches but not the subtype.
        assert_eq!(index(9, 5), 0x15);
    }

    #[test]
    fn fn_00d048e0_wakes_the_queue_with_jobs_and_a_waiting_thread() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 3);
        e.mem.set_u32(queue + 0x4c, 0);
        e.mem.set_u32(queue + 0x50, 2);
        e.mem.set_u32(queue + 0x34, 3);
        // Ring 1 has jobs and a thread waits on queue 1.
        e.mem.set_u32(ring(data, 1) + 0x10, 2);
        e.mem.set_u16(data + 0x22 + 2, 1);
        e.call(0x00d0_48e0, &args![queue, data]);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7001, 1]]);
        assert_eq!(e.mem.u16(data + 0x22 + 2), 0);
        // Jobs in ring 1, nobody waiting there, a thread waiting on queue 2: that one wakes.
        e.mem.set_u16(data + 0x22 + 4, 1);
        e.call(0x00d0_48e0, &args![queue, data]);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE).len(), 2);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE)[1], vec![0x7002, 1]);
        // No jobs anywhere: nobody wakes.
        e.mem.set_u32(ring(data, 1) + 0x10, 0);
        e.mem.set_u16(data + 0x22 + 4, 1);
        e.call(0x00d0_48e0, &args![queue, data]);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE).len(), 2);
        // Jobs only in the shared range (ring 2): the first waiting queue wakes.
        e.mem.set_u32(ring(data, 2) + 0x10, 1);
        e.call(0x00d0_48e0, &args![queue, data]);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE)[2], vec![0x7002, 1]);
    }

    #[test]
    fn release_one_waiting_thread_prefers_the_named_queue() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 3);
        e.mem.set_u16(data + 0x22 + 2, 2);
        e.call(0x00d0_4990, &args![queue, 1u32, data]);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7001, 1]]);
        assert_eq!(e.mem.u16(data + 0x22 + 2), 1);
        // An index at or above the queue count goes to the general wake-up: with jobs in ring 0
        // and a waiting thread on queue 0, that one wakes.
        e.mem.set_u32(queue + 0x4c, 0);
        e.mem.set_u32(queue + 0x50, 1);
        e.mem.set_u32(ring(data, 0) + 0x10, 1);
        e.mem.set_u16(data + 0x22, 1);
        e.call(0x00d0_4990, &args![queue, 5u32, data]);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE)[1], vec![0x7000, 1]);
    }

    #[test]
    fn fn_00d049d0_sets_the_flag_and_wakes_everyone_when_it_is_zero() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 2);
        e.mem.set_u16(data + 0x22, 1);
        e.call(0x00d0_49d0, &args![queue, 1u32]);
        assert_eq!(e.mem.u32(data + 0x1c), 1);
        assert!(calls(&e, SEMAPHORE_RELEASE).is_empty());
        e.call(0x00d0_49d0, &args![queue, 0u32]);
        assert_eq!(e.mem.u32(data + 0x1c), 0);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7000, 1]]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 2);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 2);
    }

    #[test]
    fn fn_00d04a20_appends_a_rule_and_recomputes_the_queues() {
        let mut e = engine();
        let (queue, _) = new_queue(&mut e);
        e.mem.set_u32(queue + 0x1a8, EMPTY_CAPACITY);
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            // Grows the array (a[1]) to 4 rules of 12 bytes.
            let block = e.mem.alloc(48);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.call(0x00d0_4a20, &args![queue, 0x55u32, 9u8, 3u32]);
        let rules = e.mem.u32(queue + 0x1a0);
        assert_eq!(e.mem.u32(queue + 0x1a4), 1);
        assert_eq!(e.mem.u32(rules), 0x55);
        assert_eq!(e.mem.u8(rules + 4), 9);
        assert_eq!(e.mem.u32(rules + 8), 3);
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE)[0][2..], [0xc]);
        // The queues were recomputed with the new rule.
        assert_eq!(e.mem.u32(queue + 0x48), 1);
        assert_eq!(e.mem.u8(queue + 0x74), 1);
    }

    #[test]
    fn fn_00d04a80_makes_room_in_the_shared_and_rule_rings() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        e.mem.set_u32(queue + 0x30, 2);
        e.mem.set_u32(queue + 0x48, 1);
        e.mem.set_u8(queue + 0x3d, 5);
        e.call(0x00d0_4a80, &args![queue, 1u32, 8u32]);
        assert_eq!(e.mem.u32(ring(data, 3) + 4), 8);
        assert_eq!(e.mem.u32(ring(data, 5) + 4), 8);
        assert_ne!(e.mem.u32(ring(data, 3)), 0);
        assert_eq!(e.mem.u32(ring(data, 2) + 4), 0);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn add_job_queue_locked_puts_the_job_on_the_front_or_back_and_wakes_its_queue() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 6);
        e.mem.set_u32(queue + 0x30, 2);
        e.mem.set_u16(data + 0x22 + 2 * 5, 1);
        // Type 3 goes to queue 5.
        let job = make_job(&mut e, 0, 3, 0xaa);
        e.call(0x00d0_4b00, &args![queue, data, job, 1u32]);
        let target = ring(data, 5);
        assert_eq!(e.mem.u32(target + 0x10), 1);
        assert_eq!(marker_at(&e, target, 0), 0xaa);
        assert_eq!(e.mem.u32(target + 4), 8);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7005, 1]]);
        // A zero flag puts it on the front: the head wraps to the last slot.
        let job = make_job(&mut e, 0, 3, 0xbb);
        e.call(0x00d0_4b00, &args![queue, data, job, 0u32]);
        assert_eq!(e.mem.u32(target + 0x10), 2);
        assert_eq!(e.mem.u32(target + 8), 7);
        assert_eq!(marker_at(&e, target, 7), 0xbb);
    }

    #[test]
    fn add_job_takes_the_lock_around_the_add() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 1);
        let job = make_job(&mut e, 0, 0, 0x5a);
        e.call(0x00d0_4b50, &args![queue, job, 1u32]);
        assert_eq!(e.mem.u32(ring(data, 0) + 0x10), 1);
        assert_eq!(marker_at(&e, ring(data, 0), 0), 0x5a);
        let log = e.call_log.as_ref().unwrap();
        let first = log.iter().position(|(a, _)| *a == ENTER_CRITICAL_SECTION);
        let last = log.iter().position(|(a, _)| *a == LEAVE_CRITICAL_SECTION);
        assert!(first.unwrap() < last.unwrap());
    }

    #[test]
    fn add_job_ov2_adds_a_copy_of_the_job() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 1);
        let job = make_job(&mut e, 0, 0, 0x77);
        e.call(0x00d0_4b90, &args![queue, job, 1u32]);
        let copies = calls(&e, MEM_COPY);
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][1..], [job, 0x20]);
        assert_ne!(copies[0][0], job);
        assert_eq!(marker_at(&e, ring(data, 0), 0), 0x77);
    }

    #[test]
    fn add_job_batch_adds_every_job_between_monitor_records() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 2);
        e.mem.set_u16(data + 0x22 + 2, 1);
        let jobs = [make_job(&mut e, 0, 1, 0x11), make_job(&mut e, 0, 0, 0x22)];
        let array = e.mem.alloc(8);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, jobs[0]);
        e.mem.set_u32(list + 4, jobs[1]);
        e.mem.set_u32(array, list);
        e.mem.set_u32(array + 4, 2);
        // Back (flag 1) of the ring of type 1 and then of type 0.
        e.call(0x00d0_4bd0, &args![queue, array, 1u32]);
        assert_eq!(marker_at(&e, ring(data, 1), 0), 0x11);
        assert_eq!(marker_at(&e, ring(data, 0), 0), 0x22);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7001, 1]]);
        assert_eq!(labels(&e), vec![LABEL_ADD_JOB_BATCH, LABEL_END]);
        // The front (flag 0) puts the next job before the first.
        e.call(0x00d0_4bd0, &args![queue, array, 0u32]);
        assert_eq!(e.mem.u32(ring(data, 1) + 0x10), 2);
        assert_eq!(marker_at(&e, ring(data, 1), 7), 0x11);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 2);
    }

    /// Finish hook that leaves a job of type 3 (marker 7) and the flag 1 in its answer.
    fn finish_hook_job(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[3], 1);
        e.mem.set_u8(a[3] + 0x10, 0);
        e.mem.set_u8(a[3] + 0x11, 3);
        e.mem.set_u8(a[3] + 0x18, 7);
        Ret::default()
    }

    #[test]
    fn finish_job_puts_the_hook_job_back_and_counts_the_finished_one_off() {
        let mut e = engine();
        let (queue, data) = new_queue(&mut e);
        semaphore_table(&mut e, queue, 1);
        e.mem.set_u32(queue + 0x20, FINISH_HOOK);
        e.register(FINISH_HOOK, finish_hook_job);
        e.mem.set_u16(data + 2 * 4, 5);
        let finished = make_job(&mut e, 0, 4, 0);
        e.call(0x00d0_4d80, &args![queue, finished, 0u32]);
        // The hook was given (queue, shared block, finished job, answer).
        let hook = calls(&e, FINISH_HOOK);
        assert_eq!(hook[0][..3], [queue, data, finished]);
        // Type 3 goes to ring 3 (back, since the flag is 1).
        assert_eq!(e.mem.u32(ring(data, 3) + 0x10), 1);
        assert_eq!(marker_at(&e, ring(data, 3), 0), 7);
        assert_eq!(e.mem.u16(data + 2 * 4), 4);
        assert_eq!(labels(&e), vec![LABEL_FINISH_JOB, LABEL_END]);
        // `keep` set leaves the count alone.
        e.call(0x00d0_4d80, &args![queue, finished, 1u32]);
        assert_eq!(e.mem.u16(data + 2 * 4), 4);
        // A hook that answers nonzero adds nothing.
        e.register(FINISH_HOOK, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call(0x00d0_4d80, &args![queue, finished, 0u32]);
        assert_eq!(e.mem.u32(ring(data, 3) + 0x10), 2);
        assert_eq!(e.mem.u16(data + 2 * 4), 3);
    }

    /// Pop hook that copies the job shown to it into the buffer of its last word.
    fn pop_hook_copy(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[2], JOB_BYTES);
        e.mem.write(a[3], &bytes);
        Ret::default()
    }

    /// A queue whose thread (queue index byte 0) reads ring 3 and which has one job of type 2
    /// (marker 0x42) there; the thread number is 0, so `+0x54` names its queue.
    fn queue_with_one_job(e: &mut Engine) -> (u32, u32) {
        let (queue, data) = new_queue(e);
        semaphore_table(e, queue, 4);
        set_thread_number(e, 0);
        e.mem.set_u32(queue + 0x54, 0);
        e.mem.set_u32(queue + 0x1c, POP_HOOK);
        e.mem.set_u32(queue + 0x20, FINISH_HOOK);
        e.register(POP_HOOK, pop_hook_copy);
        e.mem.set_u8(queue + 0x90, 3);
        e.mem.set_u8(queue + 0x91, 0xff);
        fill_ring(e, ring(data, 3), 4, 1, 1);
        let slot = e.mem.u32(ring(data, 3)) + JOB_BYTES;
        e.mem.set_u8(slot + 1, 2);
        e.mem.set_u8(slot + 8, 0x42);
        (queue, data)
    }

    #[test]
    fn find_job_internal_takes_a_job_from_the_threads_ring() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        let out = e.mem.alloc(JOB_BYTES);
        let result = e
            .call(0x00d0_5840, &args![queue, 0xffff_ffffu32, data, 0u32, out])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u8(out + 8), 0x42);
        let source = ring(data, 3);
        assert_eq!(e.mem.u32(source + 0x10), 0);
        assert_eq!(e.mem.u32(source + 8), 2);
        assert_eq!(e.mem.u16(data + 2 * 2), 1);
        // The hook got (queue, shared block, local job, out).
        let hook = calls(&e, POP_HOOK);
        assert_eq!((hook[0][0], hook[0][1], hook[0][3]), (queue, data, out));
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn find_job_internal_puts_a_refused_job_back_and_wakes_the_fed_queue() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        e.register(POP_HOOK, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.mem.set_u16(data + 0x22 + 2, 1);
        let out = e.mem.alloc(JOB_BYTES);
        // Queue 1 was just fed; the job was found in queue 3.
        e.call(0x00d0_5840, &args![queue, 1u32, data, 0u32, out]);
        // Back on the front of ring 3 and the count restored.
        assert_eq!(e.mem.u32(ring(data, 3) + 0x10), 1);
        assert_eq!(e.mem.u32(ring(data, 3) + 8), 1);
        assert_eq!(e.mem.u16(data + 2 * 2), 1);
        // The fed queue 1 (a thread waits there) is woken.
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7001, 1]]);
    }

    #[test]
    fn find_job_internal_answers_1_without_waiting_when_told_not_to() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        e.mem.set_u32(ring(data, 3) + 0x10, 0);
        // Ring 1 (in the range +0x4c..+0x50) has jobs.
        e.mem.set_u32(queue + 0x4c, 0);
        e.mem.set_u32(queue + 0x50, 3);
        e.mem.set_u32(ring(data, 1) + 0x10, 2);
        let out = e.mem.alloc(JOB_BYTES);
        let result = e
            .call(0x00d0_5840, &args![queue, 0xffff_ffffu32, data, 1u32, out])
            .u32();
        assert_eq!(result, 1);
        assert!(calls(&e, SEMAPHORE_ACQUIRE).is_empty());
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn find_job_internal_waits_when_jobs_are_expected_and_answers_minus_one() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        e.mem.set_u32(ring(data, 3) + 0x10, 0);
        e.mem.set_u32(queue + 0x4c, 0);
        e.mem.set_u32(queue + 0x50, 3);
        e.mem.set_u32(ring(data, 1) + 0x10, 2);
        // A thread numbered 1 whose queue index byte (+0xeb) is 2; queue 2's list is empty.
        set_thread_number(&mut e, 1);
        e.mem.set_u8(queue + 0xea + 1, 2);
        e.mem.set_u8(queue + 0x90 + 18 * 2, 0xff);
        let out = e.mem.alloc(JOB_BYTES);
        let result = e
            .call(0x00d0_5840, &args![queue, 0xffff_ffffu32, data, 0u32, out])
            .u32();
        assert_eq!(result, 0xffff_ffff);
        assert_eq!(e.mem.u16(data + 0x22 + 4), 1);
        assert_eq!(calls(&e, SEMAPHORE_ACQUIRE), vec![vec![0x7002]]);
        assert_eq!(labels(&e), vec![LABEL_NO_JOB_AVAILABLE, LABEL_END]);
    }

    #[test]
    fn find_job_internal_answers_2_and_wakes_all_when_the_main_thread_finds_nothing() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        e.mem.set_u32(ring(data, 3) + 0x10, 0);
        // The main thread (thread number 0) reads queue +0x54; no threads, no mask bits.
        let out = e.mem.alloc(JOB_BYTES);
        e.mem.set_u16(data + 0x22, 1);
        let result = e
            .call(0x00d0_5840, &args![queue, 0xffff_ffffu32, data, 0u32, out])
            .u32();
        assert_eq!(result, 2);
        assert_eq!(calls(&e, SEMAPHORE_RELEASE), vec![vec![0x7000, 1]]);
        assert_eq!(e.mem.u16(data + 0x22), 0);
    }

    #[test]
    fn get_next_job_with_a_finished_job_feeds_it_and_finds_the_next() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        e.register(FINISH_HOOK, |e, a| {
            // Leaves a type-3 job with the flag 0 (front).
            e.mem.set_u32(a[3], 0);
            e.mem.set_u8(a[3] + 0x11, 3);
            Ret::default()
        });
        e.mem.set_u16(data + 2 * 6, 2);
        let finished = make_job(&mut e, 0, 6, 0);
        let out = e.mem.alloc(JOB_BYTES);
        let result = e
            .call(0x00d0_4e80, &args![queue, finished, out, 0u32])
            .u32();
        assert_eq!(result, 0);
        // The count of the finished type went down; the hook's job went to the front of ring 3
        // and was the one the thread took.
        assert_eq!(e.mem.u16(data + 2 * 6), 1);
        assert_eq!(e.mem.u8(out + 1), 3);
        assert_eq!(e.mem.u32(ring(data, 3) + 0x10), 1);
        assert_eq!(labels(&e), vec![LABEL_GET_NEXT_JOB, LABEL_END]);
    }

    #[test]
    fn get_next_job_without_a_finished_job_just_finds_one() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        let out = e.mem.alloc(JOB_BYTES);
        assert_eq!(e.call(0x00d0_5190, &args![queue, out, 0u32]).u32(), 0);
        assert_eq!(e.mem.u8(out + 8), 0x42);
        assert!(calls(&e, FINISH_HOOK).is_empty());
        assert_eq!(e.mem.u32(ring(data, 3) + 0x10), 0);
    }

    #[test]
    fn finish_add_and_get_next_job_adds_the_job_then_finds_one() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        e.mem.set_u16(data + 2 * 7, 3);
        // The job added is type 3 (queue 3, the thread's own ring); it goes on the back.
        let added = make_job(&mut e, 0, 3, 0x99);
        let result = e
            .call(0x00d0_5020, &args![queue, 7u32, 1u32, added, 0u32])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u16(data + 2 * 7), 2);
        // The thread took the first job (marker 0x42) and wrote it over `added`.
        assert_eq!(e.mem.u8(added + 8), 0x42);
        assert_eq!(e.mem.u32(ring(data, 3) + 0x10), 1);
        assert_eq!(marker_at(&e, ring(data, 3), 2), 0x99);
        assert_eq!(labels(&e), vec![LABEL_GET_NEXT_JOB, LABEL_END]);
    }

    #[test]
    fn process_all_jobs_runs_each_job_by_its_type_table() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        // Type 2's table: function 0 is the job function, which ends the loop with 5.
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, JOB_FUNCTION);
        e.mem.set_u32(queue + (2 + 0xf) * 0x10, table);
        e.register(JOB_FUNCTION, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00d0_51b0, &args![queue]).u32(), 5);
        let run = calls(&e, JOB_FUNCTION);
        assert_eq!(run.len(), 1);
        assert_eq!(run[0][0], queue);
        assert_eq!(e.mem.u32(ring(data, 3) + 0x10), 0);
        assert_eq!(
            labels(&e),
            vec![LABEL_GET_NEXT_JOB, LABEL_END, JOB_TYPE_LABELS[2], LABEL_END]
        );
    }

    #[test]
    fn process_all_jobs_returns_what_the_fetch_answers_when_no_job_is_left() {
        let mut e = engine();
        let (queue, data) = queue_with_one_job(&mut e);
        e.mem.set_u32(ring(data, 3) + 0x10, 0);
        // The main thread finds nothing and nobody works: the fetch answers 2.
        assert_eq!(e.call(0x00d0_51b0, &args![queue]).u32(), 2);
    }

    #[test]
    fn fn_00d05450_sets_up_the_default_hardware_set_up() {
        let mut e = engine();
        e.register(THREAD_COUNT_QUERY, |e, a| {
            e.mem.set_u32(a[0], 6);
            Ret::default()
        });
        let object = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d0_5450, &args![object]).u32(), object);
        let words: Vec<u32> = [0, 8, 0xc, 0x10, 0x14]
            .iter()
            .map(|o| e.mem.u32(object + o))
            .collect();
        assert_eq!(words, vec![1, 6, 0, 0, EMPTY_CAPACITY]);
        assert_eq!(e.mem.u8(object + 4), 1);
        // A query that stores nothing leaves the local as it was: the object's own address.
        e.register(THREAD_COUNT_QUERY, noop);
        e.call(0x00d0_5450, &args![object]);
        assert_eq!(e.mem.u32(object + 8), object);
    }

    #[test]
    fn the_constructor_builds_the_rings_hooks_and_queues() {
        let mut e = engine();
        e.register(THREAD_COUNT_QUERY, |e, a| {
            e.mem.set_u32(a[0], 2);
            Ret::default()
        });
        let queue = e.mem.alloc(QUEUE_BYTES);
        let setup = e.mem.alloc(0x20);
        e.mem.set_u32(setup, 1);
        e.mem.set_u32(setup + 8, 2);
        e.mem.set_u32(setup + 0x18, 2);
        // An empty array of arrays at +0xc.
        e.mem.set_u32(setup + 0x14, EMPTY_CAPACITY);
        assert_eq!(e.call(0x00d0_5490, &args![queue, setup]).u32(), queue);
        let data = e.mem.u32(queue + 0x18);
        assert_ne!(data, 0);
        // Sixteen rings constructed at 0x14 intervals from +0x2c, each then given 128 jobs.
        let constructed = calls(&e, RING_CONSTRUCT);
        assert_eq!(constructed.len(), 16);
        for (i, call) in constructed.iter().enumerate() {
            assert_eq!(call[0], ring(data, i as u32));
            assert_eq!(e.mem.u32(ring(data, i as u32) + 4), 0x80);
        }
        assert_eq!(calls(&e, CRITICAL_SECTION_CONSTRUCT), vec![vec![queue, 0]]);
        assert_eq!(calls(&e, TLS_SET_VALUE), vec![vec![THREAD_INDEX, 0]]);
        assert_eq!(e.mem.u32(queue + 0x24), 2);
        assert_eq!(e.mem.u32(queue + 0x1c), DEFAULT_POP_HOOK);
        assert_eq!(e.mem.u32(queue + 0x20), DEFAULT_FINISH_HOOK);
        // One bit per thread in the mask; the job-type table has the default hook words.
        assert_eq!(e.mem.u32(data + 0x18), 3);
        assert_eq!(e.mem.u32(queue + 0xf0 + 8), DEFAULT_TYPE_HOOK_A);
        assert_eq!(e.mem.u32(queue + 0x100 + 0xc), DEFAULT_TYPE_HOOK_B);
        // The set-up was copied and the queues computed.
        assert_eq!(e.mem.u32(queue + 0x58), 1);
        assert_eq!(e.mem.u32(queue + 0x60), 2);
        assert_eq!(e.mem.u8(queue + 0x74), 1);
        assert_eq!(e.mem.u32(queue + 0x30), 3);
    }

    #[test]
    fn the_destructor_body_frees_the_rings_semaphores_rules_and_section() {
        let mut e = engine();
        e.register(THREAD_COUNT_QUERY, |e, a| {
            e.mem.set_u32(a[0], 1);
            Ret::default()
        });
        let queue = e.mem.alloc(QUEUE_BYTES);
        let setup = e.mem.alloc(0x20);
        e.mem.set_u32(setup + 0x14, EMPTY_CAPACITY);
        e.mem.set_u32(setup + 0x18, 1);
        e.call(0x00d0_5490, &args![queue, setup]);
        let data = e.mem.u32(queue + 0x18);
        let buffers: Vec<u32> = (0..16).map(|i| e.mem.u32(ring(data, i))).collect();
        let semaphore = e.mem.u32(queue + 0x78);
        assert_eq!(e.mem.u32(queue + 0x8c), 2);
        e.call_log = Some(vec![]);
        e.call(0x00d0_55c0, &args![queue]);
        assert_eq!(e.mem.block_size(data), None);
        for buffer in buffers {
            assert_eq!(e.mem.block_size(buffer), None);
        }
        assert_eq!(e.mem.block_size(semaphore), None);
        assert_eq!(calls(&e, SEMAPHORE_DESTRUCT).len(), 2);
        assert_eq!(e.mem.u32(queue + 0x1a8), EMPTY_CAPACITY);
        assert_eq!(calls(&e, HW_SETUP_DESTRUCT), vec![vec![queue + 0x58]]);
        assert_eq!(calls(&e, DELETE_CRITICAL_SECTION), vec![vec![queue]]);
    }

    #[test]
    fn fn_00d056a0_frees_a_ring_buffer_only_when_it_has_capacity() {
        let mut e = engine();
        let ring = e.mem.alloc(0x14);
        e.call(0x00d0_56a0, &args![ring]);
        assert!(calls(&e, MEMORY_ROUTER).is_empty());
        fill_ring(&mut e, ring, 4, 0, 0);
        let buffer = e.mem.u32(ring);
        e.call(0x00d0_56a0, &args![ring]);
        assert_eq!(e.mem.block_size(buffer), None);
    }

    #[test]
    fn fn_00d056d0_grows_a_ring_keeping_its_jobs_in_order() {
        let mut e = engine();
        // Contiguous: jobs at 1 and 2 of a ring of 4.
        let ring_a = e.mem.alloc(0x14);
        fill_ring(&mut e, ring_a, 4, 1, 2);
        for (slot, marker) in [(1u32, 0xa1u8), (2, 0xa2)] {
            let at = e.mem.u32(ring_a) + slot * JOB_BYTES + 8;
            e.mem.set_u8(at, marker);
        }
        e.call(0x00d0_56d0, &args![ring_a, 6u32]);
        // Twice the capacity (8) is larger than the 6 asked for.
        assert_eq!(e.mem.u32(ring_a + 4), 8);
        assert_eq!(
            (marker_at(&e, ring_a, 0), marker_at(&e, ring_a, 1)),
            (0xa1, 0xa2)
        );
        assert_eq!((e.mem.u32(ring_a + 8), e.mem.u32(ring_a + 0xc)), (0, 2));
        // Wrapped: jobs at 3, 0, 1 of a ring of 4.
        let ring_b = e.mem.alloc(0x14);
        fill_ring(&mut e, ring_b, 4, 3, 3);
        for (slot, marker) in [(3u32, 0xb1u8), (0, 0xb2), (1, 0xb3)] {
            let at = e.mem.u32(ring_b) + slot * JOB_BYTES + 8;
            e.mem.set_u8(at, marker);
        }
        e.call(0x00d0_56d0, &args![ring_b, 20u32]);
        assert_eq!(e.mem.u32(ring_b + 4), 20);
        let order: Vec<u8> = (0..3).map(|i| marker_at(&e, ring_b, i)).collect();
        assert_eq!(order, vec![0xb1, 0xb2, 0xb3]);
        assert_eq!((e.mem.u32(ring_b + 8), e.mem.u32(ring_b + 0xc)), (0, 3));
        // No change when the capacity already suffices.
        let before = e.mem.u32(ring_b);
        e.call(0x00d0_56d0, &args![ring_b, 20u32]);
        assert_eq!(e.mem.u32(ring_b), before);
        // A ring with no buffer just gets one.
        let ring_c = e.mem.alloc(0x14);
        e.call(0x00d0_56d0, &args![ring_c, 3u32]);
        assert_eq!(e.mem.u32(ring_c + 4), 3);
        assert_ne!(e.mem.u32(ring_c), 0);
    }

    #[test]
    fn fn_00d057a0_appends_and_wraps_the_tail() {
        let mut e = engine();
        let ring = e.mem.alloc(0x14);
        let first = make_job(&mut e, 0, 0, 0x01);
        e.call(0x00d0_57a0, &args![ring, first]);
        // A full empty ring grows to 8 jobs.
        assert_eq!(e.mem.u32(ring + 4), 8);
        assert_eq!((e.mem.u32(ring + 0xc), e.mem.u32(ring + 0x10)), (1, 1));
        assert_eq!(marker_at(&e, ring, 0), 1);
        // The tail wraps to 0 once it reaches the capacity.
        e.mem.set_u32(ring + 0xc, 8);
        e.mem.set_u32(ring + 0x10, 1);
        let second = make_job(&mut e, 0, 0, 0x02);
        e.call(0x00d0_57a0, &args![ring, second]);
        assert_eq!(marker_at(&e, ring, 0), 2);
        assert_eq!((e.mem.u32(ring + 0xc), e.mem.u32(ring + 0x10)), (1, 2));
    }

    #[test]
    fn fn_00d057f0_prepends_and_wraps_the_head() {
        let mut e = engine();
        let ring = e.mem.alloc(0x14);
        let first = make_job(&mut e, 0, 0, 0x01);
        e.call(0x00d0_57f0, &args![ring, first]);
        // The head starts at the capacity (8) and goes back one.
        assert_eq!(e.mem.u32(ring + 4), 8);
        assert_eq!((e.mem.u32(ring + 8), e.mem.u32(ring + 0x10)), (7, 1));
        assert_eq!(marker_at(&e, ring, 7), 1);
        let second = make_job(&mut e, 0, 0, 0x02);
        e.call(0x00d0_57f0, &args![ring, second]);
        assert_eq!((e.mem.u32(ring + 8), e.mem.u32(ring + 0x10)), (6, 2));
        assert_eq!(marker_at(&e, ring, 6), 2);
    }

    /// The `_reserve` double: a new block of `count * 12` bytes holding the old elements, and
    /// capacity `count`.
    fn reserve_double(e: &mut Engine, a: &[u32]) -> Ret {
        let block = e.mem.alloc(a[2] * 12);
        let (old, size) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
        let bytes = e.mem.bytes(old, size * 12);
        e.mem.write(block, &bytes);
        e.mem.set_u32(a[1], block);
        e.mem.set_u32(a[1] + 8, a[2]);
        Ret::default()
    }

    fn put_array(e: &mut Engine, array: u32, elements: &[&[u32]]) {
        let data = e.mem.alloc(12 * elements.len() as u32 + 4);
        for (i, ints) in elements.iter().enumerate() {
            let block = e.mem.alloc(4 * ints.len() as u32 + 4);
            for (j, value) in ints.iter().enumerate() {
                e.mem.set_u32(block + 4 * j as u32, *value);
            }
            let slot = data + 12 * i as u32;
            e.mem.set_u32(slot, block);
            e.mem.set_u32(slot + 4, ints.len() as u32);
            e.mem.set_u32(slot + 8, ints.len() as u32);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, elements.len() as u32);
        e.mem.set_u32(array + 8, elements.len() as u32);
    }

    fn ints_of(e: &Engine, array: u32, element: u32) -> Vec<u32> {
        let slot = e.mem.u32(array) + 12 * element;
        let (block, size) = (e.mem.u32(slot), e.mem.u32(slot + 4));
        (0..size).map(|i| e.mem.u32(block + 4 * i)).collect()
    }

    #[test]
    fn hw_setup_assignment_copies_the_words_and_the_array_of_arrays() {
        let mut e = engine();
        e.register(ARRAY_RESERVE, reserve_double);
        let source = e.mem.alloc(0x20);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(source, 7);
        e.mem.set_u8(source + 4, 1);
        e.mem.set_u32(source + 8, 9);
        put_array(&mut e, source + 0xc, &[&[1, 2, 3], &[4, 5], &[]]);
        // The target holds two arrays of its own: the first has room for 4 ints, the second
        // (capacity 1) does not have room for 2.
        put_array(&mut e, target + 0xc, &[&[10, 11, 12, 13], &[20]]);
        let roomy = e.mem.u32(target + 0xc);
        let big = e.mem.u32(roomy);
        assert_eq!(e.call(0x00d0_5b50, &args![target, source]).u32(), target);
        assert_eq!(e.mem.u32(target), 7);
        assert_eq!(e.mem.u8(target + 4), 1);
        assert_eq!(e.mem.u32(target + 8), 9);
        assert_eq!(e.mem.u32(target + 0x10), 3);
        assert_eq!(ints_of(&e, target + 0xc, 0), vec![1, 2, 3]);
        assert_eq!(ints_of(&e, target + 0xc, 1), vec![4, 5]);
        assert_eq!(ints_of(&e, target + 0xc, 2), Vec::<u32>::new());
        // The first element kept its storage (capacity 4 was enough).
        assert_eq!(e.mem.u32(e.mem.u32(target + 0xc)), big);
        // The array was grown: twice its capacity (4) is more than the 3 elements.
        assert_eq!(calls(&e, ARRAY_RESERVE).len(), 1);
        assert_eq!(calls(&e, ARRAY_RESERVE)[0][2..], [4, 12]);
    }

    #[test]
    fn hw_setup_assignment_empties_the_extra_elements() {
        let mut e = engine();
        e.register(ARRAY_RESERVE, reserve_double);
        let source = e.mem.alloc(0x20);
        let target = e.mem.alloc(0x20);
        put_array(&mut e, source + 0xc, &[&[5, 6]]);
        put_array(&mut e, target + 0xc, &[&[1], &[2, 3], &[4]]);
        let data = e.mem.u32(target + 0xc);
        let doomed = e.mem.u32(data + 12);
        e.call(0x00d0_5b50, &args![target, source]);
        assert_eq!(e.mem.u32(target + 0x10), 1);
        assert_eq!(ints_of(&e, target + 0xc, 0), vec![5, 6]);
        // The element past the new size has no storage left (freed by the buffer-free slot).
        assert_eq!(e.mem.u32(data + 12), 0);
        assert_eq!(e.mem.u32(data + 12 + 4), 0);
        assert_eq!(e.mem.u32(data + 12 + 8), EMPTY_CAPACITY);
        assert_eq!(e.mem.block_size(doomed), None);
        assert!(calls(&e, ARRAY_RESERVE).is_empty());
    }
}
