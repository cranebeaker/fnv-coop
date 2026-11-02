//! `Havok SDK/hkpcpubroadphasejob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{
    JOB_DONE, LABEL_JOB_END, LOCK_ISLAND, UNLOCK_ISLAND,
};
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// The memory router (`00c85750`): its first word is the top of the stack allocator, its
/// `+0x10` the allocator (slot 4 allocates, 8 frees an object of a given size, `0x10` frees a
/// buffer, `0x24` asks whether a number of bytes can be had) and its `+0x18` the allocator of
/// the buffer in the job group.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `InterlockedExchangeAdd` import slot (stdcall: the counter, the addend; returns the old value).
const INTERLOCKED_EXCHANGE_ADD: u32 = 0x00fd_f1ec;
/// Unnamed (cdecl, two words: the world and the entity of the job).
const PREPARE_ENTITY: u32 = 0x00cb_5480;
/// `hkpContinuousSimulation::collideEntitiesBroadPhaseContinuousFindPairs` (Ghidra name;
/// `stdcall`, five stack words: two words of the entity, the world and the two arrays that
/// receive the pairs; the word at `+0x110` of the job owner the caller puts in `ECX` is not
/// read).
const FIND_PAIRS: u32 = 0x00d0_c030;
/// `hkpWorldCallbackUtil::fireContactImpulseLimitBreached` (Ghidra name; cdecl: world, the
/// records, their count).
const FIRE_IMPULSE_LIMIT_BREACHED: u32 = 0x00d0_1700;
/// `hkpTypedBroadPhaseDispatcher::removePairs` (Ghidra name; `ECX` = the dispatcher, two stack
/// words: the pairs and their count).
const REMOVE_PAIRS: u32 = 0x00cf_70f0;
/// `hkpTypedBroadPhaseDispatcher::addPairs` (Ghidra name; `ECX` = the dispatcher, three stack
/// words: the pairs, their count and a pointer or 0).
const ADD_PAIRS: u32 = 0x00cf_7080;
/// `hkpAgentSectorHeader::allocate` (Ghidra name; cdecl: count, size in 16-byte units).
const ALLOCATE_SECTORS: u32 = 0x00d9_6530;
/// `hkJobQueue::finishAddAndGetNextJob` (Ghidra name; `ECX` = the second word of the job
/// function, four stack words: 0, 1, the job, 0).
pub(crate) const FINISH_ADD_AND_GET_NEXT_JOB: u32 = 0x00d0_5020;

/// Profiling label `"TtBroadphase"`.
const LABEL_TT_BROADPHASE: u32 = 0x010d_3e68;
/// Profiling label `"TtWaitForExport"`.
const LABEL_TT_WAIT_FOR_EXPORT: u32 = 0x010c_d3d8;
/// Profiling label `"TtWaitForSolverCallbacks"`.
const LABEL_TT_WAIT_FOR_SOLVER_CALLBACKS: u32 = 0x010d_3e4c;
/// Profiling label `"LtAddRemoveAgnts"` (first label of a 16-byte record).
const LABEL_LT_ADD_REMOVE_AGENTS: u32 = 0x010d_3e38;
/// Profiling label `"StInit"` (second label of that record).
const LABEL_ST_INIT: u32 = 0x010c_c5bc;
/// Profiling label `"StRemoveAgt"`.
const LABEL_ST_REMOVE_AGENT: u32 = 0x010c_c5e4;
/// Profiling label `"StAddAgt"`.
const LABEL_ST_ADD_AGENT: u32 = 0x010c_c5d8;

/// The capacity bits of a capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The capacity word of an array without storage.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
/// How many times a wait on a flag that another thread sets is polled before the host gives up:
/// nothing else runs here, so a flag that is not set would be waited on for ever.
const SPIN_LIMIT: u32 = 1 << 20;

/// The allocator behind the memory router (`00c85750`, its `+0x10`).
fn allocator_of_router(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Takes `((count * 8 + 0x7f) & !0x7f)` bytes off the stack allocator (the router's first word
/// moves up) for the array of `count` 8-byte entries at `array` (data at `+0`, size at `+4`,
/// capacity word at `+8`, the old top at `+0xC`): data is the old top, size 0 and the capacity
/// word is the count with bit 31 (not owned) set. The count is the word at `+0x33C` of the
/// world, which is the word at `owner + 0`.
fn take_array_from_stack_allocator(e: &mut Engine, owner: u32, array: u32) {
    let count = e.mem.u32(e.mem.u32(owner) + 0x33c);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, EMPTY_CAPACITY);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let used = (count.wrapping_mul(8).wrapping_add(0x7f)) & 0xffff_ff80;
    e.mem.set_u32(router, used.wrapping_add(top));
    e.mem.set_u32(array, top);
    e.mem.set_u32(array + 8, count | 0x8000_0000);
    e.mem.set_u32(array + 0xc, top);
}

/// Gives back what `take_array_from_stack_allocator` took: the router's first word is put back
/// to the old top, the size becomes 0 and the data is freed (allocator slot `0x10`: data, 8
/// bytes per entry of capacity) unless the capacity word says it is not owned.
fn give_back_array(e: &mut Engine, array: u32) {
    let top = e.mem.u32(array + 0xc);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
    let allocator = allocator_of_router(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(8)],
        );
    }
}

/// Polls the byte at `flag` until it is 2 (or, with `accept_one`, also 1).
fn wait_for_flag(e: &mut Engine, flag: u32, accept_one: bool) {
    for _ in 0..SPIN_LIMIT {
        let value = e.mem.u8(flag);
        if value == 2 || (accept_one && value == 1) {
            return;
        }
    }
    panic!("00d93ba0: the flag at {flag:#x} is waited on for ever");
}

/// Frees every node of the singly linked list whose head word is at `head` (the next pointer is
/// the first word of a node) through the allocator's slot 8 with `size` bytes.
fn free_list(e: &mut Engine, head: u32, size: u32) {
    while e.mem.u32(head) != 0 {
        let node = e.mem.u32(head);
        let next = e.mem.u32(node);
        e.mem.set_u32(head, next);
        let allocator = allocator_of_router(e);
        e.vcall(allocator, 8, &args![node, size]);
    }
}

/// The last thread of a group frees what the group holds: the two lists of nodes (at `+0x68`,
/// 0x1000 bytes each, and at `+0x78`, 0x30 bytes each), the buffer at `+0x14` (size at `+0x1C`;
/// slot `0x10` of the allocator at `router + 0x18`), the 4-byte entries at `+0x50` (16-bit count
/// at `+0x56`) when the thread count at `+0x10` is above 1, the array at `+0x5C` (size at
/// `+0x60`, capacity word at `+0x64`, 4 bytes per entry) and the group itself (0x80 bytes).
fn free_group(e: &mut Engine, group: u32) {
    free_list(e, group + 0x68, 0x1000);
    free_list(e, group + 0x78, 0x30);
    let buffer = e.mem.u32(group + 0x14);
    if buffer != 0 {
        let size = e.mem.u32(group + 0x1c);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let buffer_allocator = e.mem.u32(router + 0x18);
        e.vcall(buffer_allocator, 0x10, &args![buffer, size]);
        e.mem.set_u32(group + 0x14, 0);
    }
    if e.mem.i32(group + 0x10) > 1 {
        let count = u32::from(e.mem.u16(group + 0x56));
        let data = e.mem.u32(group + 0x50);
        let allocator = allocator_of_router(e);
        e.vcall(allocator, 0x10, &args![data, count * 4]);
        e.mem.set_u32(group + 0x50, 0);
    }
    let allocator = allocator_of_router(e);
    let capacity = e.mem.u32(group + 0x64);
    e.mem.set_u32(group + 0x60, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(group + 0x5c);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(group + 0x5c, 0);
    e.mem.set_u32(group + 0x64, EMPTY_CAPACITY);
    let allocator = allocator_of_router(e);
    e.vcall(allocator, 8, &args![group, 0x80u32]);
}

/// The first thread to leave a group has the lists of impulse records that the solver callbacks
/// left gathered: the nodes at `group + 0x24` (a count of 16-byte records at `+0` and the next
/// node at `+4`, the records from `+0x10`) are copied one after the other into a block taken
/// off the stack allocator and `fireContactImpulseLimitBreached` is called with the world, the
/// block and the total count; the block is then given back.
fn fire_impulse_limit_breached(e: &mut Engine, world: u32, group: u32) {
    let mut node = e.mem.u32(group + 0x24);
    if node == 0 {
        return;
    }
    let mut total = 0u32;
    while node != 0 {
        total = total.wrapping_add(e.mem.u32(node));
        node = e.mem.u32(node + 4);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let block = e.mem.u32(router);
    let used = (total.wrapping_shl(4).wrapping_add(0x7f)) & 0xffff_ff80;
    e.mem.set_u32(router, used.wrapping_add(block));
    let mut records = 0u32;
    node = e.mem.u32(group + 0x24);
    while node != 0 {
        let count = e.mem.u32(node);
        let to = block.wrapping_add(records.wrapping_shl(4));
        for word in 0..(count.wrapping_mul(4) as i32).max(0) as u32 {
            let value = e.mem.u32(node + 0x10 + 4 * word);
            e.mem.set_u32(to + 4 * word, value);
        }
        records = records.wrapping_add(count);
        node = e.mem.u32(node + 4);
    }
    e.call(FIRE_IMPULSE_LIMIT_BREACHED, &args![world, block, total]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, block);
}

/// Removes and adds the pairs the search found (`removed` and `added` are the arrays at
/// `scratch` and `scratch + 0x10`) while the island of the entity is locked: `00cf70f0` with the
/// removed pairs, then, when the allocator's slot `0x24` agrees that `added.size << 10` bytes
/// can be had, `00cf7080` with the added pairs and the world's pointer at `+0x7C` plus 8 (0 when
/// that is 0); the island (the word at `+0xCC` of the word at `*[entity + 0x48]`) is unlocked.
fn update_pairs(e: &mut Engine, world: u32, entity: u32, removed: u32, added: u32) {
    profile_start_record(e, LABEL_LT_ADD_REMOVE_AGENTS, LABEL_ST_INIT);
    let hub = e.mem.u32(entity + 0x48);
    if e.mem.i32(removed + 4).wrapping_add(e.mem.i32(added + 4)) > 0 {
        profile_record(e, LABEL_ST_REMOVE_AGENT);
        let island = e.mem.u32(e.mem.u32(hub) + 0xcc);
        e.call(LOCK_ISLAND, &args![world, island]);
        let dispatcher = e.mem.u32(world + 0x64);
        let (pairs, count) = (e.mem.u32(removed), e.mem.u32(removed + 4));
        e.call(REMOVE_PAIRS, &args![dispatcher, pairs, count]);
        let added_count = e.mem.u32(added + 4);
        let allocator = allocator_of_router(e);
        let granted = e
            .vcall(allocator, 0x24, &args![added_count.wrapping_shl(10)])
            .bool();
        if granted {
            profile_record(e, LABEL_ST_ADD_AGENT);
            let extra = match e.mem.u32(world + 0x7c) {
                0 => 0,
                pointer => pointer.wrapping_add(8),
            };
            let (pairs, count) = (e.mem.u32(added), e.mem.u32(added + 4));
            e.call(ADD_PAIRS, &args![dispatcher, pairs, count, extra]);
        }
        let island = e.mem.u32(e.mem.u32(hub) + 0xcc);
        e.call(UNLOCK_ISLAND, &args![world, island]);
    }
    profile_record(e, LABEL_JOB_END);
}

// Translated from 00d93ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSingleThreadedJobsOnIsland::cpuBroadPhaseJob` (Ghidra name; the engine map has none):
/// cdecl, three stack words: the owner (its first word is the world), a word that becomes
/// `ECX` of the closing calls and the job. Writes `TtBroadphase`; when the group (`[job +
/// 0x18]`) has a word at `+8`, the world's byte at `+0xD5` is set, the entity (`[job + 0x14]`)
/// has bits 2 or 3 of its byte at `+0x26` and neither of the same bits of its byte at `+0x25`
/// it calls `00cb5480` with the world and the entity. Takes two arrays of 8-byte entries, each
/// sized for the word at `+0x33C` of the world, off the stack allocator, lets `00d0c030` fill
/// them (the pairs to add and to remove), and takes one off the thread counter at `group +
/// 0xC` (a plain decrement when the thread count at `+0x10` is 1, otherwise
/// `InterlockedExchangeAdd`). The thread whose old value equals the thread count waits for the
/// flag byte at `group + 0x20` to be 1 or 2 (`TtWaitForExport`; the wait polls, for other
/// threads set the flag), fires the impulse records gathered at `group + 0x24`
/// (`fire_impulse_limit_breached`) and sets the flag to 2; any other thread waits for the flag
/// to be 2 (`TtWaitForSolverCallbacks`) and decrements the counter again unless the thread
/// count is 2 or less (when the answer is -1). When the arrays hold pairs, the pairs are removed
/// and added with the island locked (`update_pairs`). When the decrement leaves `1 -` the thread
/// count the group is freed (`free_group`). With a count above 0 at `entity + 0x60` the
/// function then fills the job as a type-0xD job (bytes 0xD, 1, 1 at `+0..+2`, 0x50 and 0xFFFF
/// at `+4`, `+6`, the world's four words at `+0x1D0` to `+0x1DC` at `+0x30` to `+0x3C`, the
/// count, the world's limit at `+0xE0`, the words at `entity + 0x5C` and `+0x58` and, when the
/// count is above the limit, a sector header from `00d96530` for `(count - 1) / limit + 1`
/// sectors) and ends with `hkJobQueue::finishAddAndGetNextJob` (`00d05020`); otherwise it ends
/// with `00d04e80` like the other jobs. The two arrays are given back last. Returns what the
/// last call returns.
pub fn hk_cpu_broad_phase_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    let owner = owner.addr();
    let job = job.addr();
    let entity = e.mem.u32(job + 0x14);
    profile_record(e, LABEL_TT_BROADPHASE);
    let world = e.mem.u32(owner);
    let group = e.mem.u32(job + 0x18);
    if e.mem.u32(group + 8) != 0
        && e.mem.u8(world + 0xd5) != 0
        && e.mem.u8(entity + 0x26) & 0xc != 0
        && e.mem.u8(entity + 0x25) & 0xc == 0
    {
        e.call(PREPARE_ENTITY, &args![world, entity]);
    }
    let flag = e.mem.u32(job + 0x18).wrapping_add(0x20);
    e.with_stack(0x20, |e, arrays| {
        // The array of pairs to remove is first on the stack, the other 0x10 bytes up.
        let removed = arrays.addr();
        let added = removed + 0x10;
        take_array_from_stack_allocator(e, owner, added);
        take_array_from_stack_allocator(e, owner, removed);
        let group = e.mem.u32(job + 0x18);
        let search_this = e.mem.u32(owner + 0x110);
        let (first, second_word) = (e.mem.u32(entity + 0x48), e.mem.u32(entity + 0x4c));
        let world = e.mem.u32(owner);
        // 00d93cbd: `00d0c030` is stdcall (RET 0x14) and does not read ECX.
        let _ = search_this;
        e.call(
            FIND_PAIRS,
            &args![first, second_word, world, added, removed],
        );
        let threads = e.mem.i32(group + 0x10);
        let expected = 1i32.wrapping_sub(threads);
        let mut finish = 0i32;
        let counter = group + 0xc;
        let old = if threads == 1 {
            let old = e.mem.u32(counter);
            e.mem.set_u32(counter, old.wrapping_sub(1));
            old
        } else {
            e.call(INTERLOCKED_EXCHANGE_ADD, &args![counter, -1i32])
                .u32()
        };
        if old == e.mem.u32(group + 0x10) {
            if flag != 0 {
                profile_record(e, LABEL_TT_WAIT_FOR_EXPORT);
                wait_for_flag(e, flag, true);
                profile_record(e, PROFILE_END);
            }
            let world = e.mem.u32(owner);
            fire_impulse_limit_breached(e, world, group);
            e.mem.set_u8(flag, 2);
        } else {
            if e.mem.u8(flag) != 2 {
                profile_record(e, LABEL_TT_WAIT_FOR_SOLVER_CALLBACKS);
                wait_for_flag(e, flag, false);
                profile_record(e, PROFILE_END);
            }
            finish = if e.mem.i32(group + 0x10) <= 2 {
                -1
            } else {
                let old = e
                    .call(INTERLOCKED_EXCHANGE_ADD, &args![counter, -1i32])
                    .u32();
                (old as i32).wrapping_sub(1)
            };
        }
        if e.mem.i32(removed + 4).wrapping_add(e.mem.i32(added + 4)) > 0 {
            let world = e.mem.u32(owner);
            update_pairs(e, world, entity, removed, added);
        }
        if finish == expected {
            free_group(e, group);
        }
        let count = e.mem.i32(entity + 0x60);
        let result = if count <= 0 {
            profile_record(e, PROFILE_END);
            e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
        } else {
            let world = e.mem.u32(owner);
            let limit = e.mem.i32(world + 0xe0);
            let word_0x5c = e.mem.u32(entity + 0x5c);
            let word_0x58 = e.mem.u32(entity + 0x58);
            e.mem.set_u8(job, 0xd);
            e.mem.set_u8(job + 1, 0);
            e.mem.set_u8(job + 2, 1);
            e.mem.set_u16(job + 4, 0x50);
            e.mem.set_u16(job + 6, 0xffff);
            for word in 0..4 {
                let value = e.mem.u32(world + 0x1d0 + 4 * word);
                e.mem.set_u32(job + 0x30 + 4 * word, value);
            }
            e.mem.set_u16(job + 0x24, 0);
            e.mem.set_u32(job + 0x2c, word_0x5c);
            e.mem.set_u16(job + 0x28, count as u16);
            e.mem.set_u16(job + 0x2a, limit as u16);
            e.mem.set_u32(job + 0x20, 0);
            e.mem.set_u8(job + 1, 1);
            e.mem.set_u32(job + 0x40, 0);
            e.mem.set_u16(job + 0x44, word_0x58 as u16);
            if count > limit {
                assert!(limit != 0, "00d93ba0: integer divide error");
                let sectors = (count - 1) / limit + 1;
                let header = e.call(ALLOCATE_SECTORS, &args![sectors, 0x10u32]).u32();
                e.mem.set_u32(job + 0x20, header);
                e.mem.set_u16(job + 0x26, sectors as u16);
            }
            profile_record(e, PROFILE_END);
            e.call(
                FINISH_ADD_AND_GET_NEXT_JOB,
                &args![second, 0u32, 1u32, job, 0u32],
            )
            .u32()
        };
        give_back_array(e, removed);
        e.mem.set_u32(removed + 8, EMPTY_CAPACITY);
        e.mem.set_u32(removed, 0);
        give_back_array(e, added);
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d93ba0,
        hk_cpu_broad_phase_job(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, logging_object};
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};

    const ALLOCATOR: u32 = 0x0900_0000;
    const BUFFER_ALLOCATOR: u32 = 0x0910_0000;
    const SECOND: u32 = 0x99;

    /// A job with its group, entity and world, the doubles of everything the function calls and
    /// the objects the checks need.
    struct Scene {
        e: Engine,
        rig: JobRig,
        owner: u32,
        world: u32,
        entity: u32,
        group: u32,
        job: u32,
        router: u32,
        top: u32,
        allocator: u32,
        buffer_allocator: u32,
    }

    /// `threads` and `counter` are the group's thread count and counter, `flag` the byte the
    /// threads wait on, `found` the sizes of the arrays of pairs to add and to remove that the
    /// search leaves and `grant` what the allocator's slot `0x24` answers. The world has the
    /// four count words 4, 0x7c pointer 0x7000, limit 8 and the four words 0xA1..0xA4 at
    /// `+0x1D0`; the entity wants no job (count 0) and has neither flag bit set.
    fn scene(threads: u32, counter: u32, flag: u8, found: (u32, u32), grant: u32) -> Scene {
        let (mut e, rig) = job_rig();
        let top = e.mem.alloc(0x2000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, top);
        let allocator = logging_object(&mut e, &rig.log, ALLOCATOR);
        let buffer_allocator = logging_object(&mut e, &rig.log, BUFFER_ALLOCATOR);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(router + 0x18, buffer_allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &rig.log, ALLOCATOR + 0x24, grant);
        log_double(&mut e, &rig.log, PREPARE_ENTITY, 0);
        log_double(&mut e, &rig.log, LOCK_ISLAND, 0);
        log_double(&mut e, &rig.log, UNLOCK_ISLAND, 0);
        log_double(&mut e, &rig.log, REMOVE_PAIRS, 0);
        log_double(&mut e, &rig.log, ADD_PAIRS, 0);
        log_double(&mut e, &rig.log, ALLOCATE_SECTORS, 0xabc0);
        log_double(&mut e, &rig.log, FINISH_ADD_AND_GET_NEXT_JOB, 0x66);
        let sink = rig.log.clone();
        e.register_double(FIND_PAIRS, move |e, a| {
            let mut words = a.to_vec();
            words.extend([e.mem.u32(a[3]), e.mem.u32(a[4]), e.mem.u32(a[3] + 8)]);
            sink.borrow_mut().push((FIND_PAIRS, words));
            e.mem.set_u32(a[3] + 4, found.0);
            e.mem.set_u32(a[4] + 4, found.1);
            Ret::default()
        });
        let sink = rig.log.clone();
        e.register_double(FIRE_IMPULSE_LIMIT_BREACHED, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..a[2] * 4).map(|word| e.mem.u32(a[1] + 4 * word)));
            sink.borrow_mut().push((FIRE_IMPULSE_LIMIT_BREACHED, words));
            Ret::default()
        });
        let sink = rig.log.clone();
        e.register_double(INTERLOCKED_EXCHANGE_ADD, move |e, a| {
            sink.borrow_mut()
                .push((INTERLOCKED_EXCHANGE_ADD, a.to_vec()));
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], old.wrapping_add(a[1]));
            Ret {
                eax: old,
                ..Ret::default()
            }
        });
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x33c, 4);
        e.mem.set_u32(world + 0x64, 0x6400);
        e.mem.set_u32(world + 0x7c, 0x7000);
        e.mem.set_u32(world + 0xe0, 8);
        for word in 0..4 {
            e.mem.set_u32(world + 0x1d0 + 4 * word, 0xa1 + word);
        }
        let owner = e.mem.alloc(0x120);
        e.mem.set_u32(owner, world);
        e.mem.set_u32(owner + 0x110, 0x1100);
        let island_holder = e.mem.alloc(0x100);
        e.mem.set_u32(island_holder + 0xcc, 0x1515);
        let hub = e.mem.alloc(8);
        e.mem.set_u32(hub, island_holder);
        let entity = e.mem.alloc(0x100);
        e.mem.set_u32(entity + 0x48, hub);
        e.mem.set_u32(entity + 0x4c, 0x4c4c);
        e.mem.set_u32(entity + 0x58, 0x58585);
        e.mem.set_u32(entity + 0x5c, 0x5c5c);
        let group = e.mem.alloc(0x100);
        e.mem.set_u32(group + 8, 1);
        e.mem.set_u32(group + 0xc, counter);
        e.mem.set_u32(group + 0x10, threads);
        e.mem.set_u8(group + 0x20, flag);
        let job = e.mem.alloc(0x80);
        e.mem.set_u32(job + 0x14, entity);
        e.mem.set_u32(job + 0x18, group);
        Scene {
            e,
            rig,
            owner,
            world,
            entity,
            group,
            job,
            router,
            top,
            allocator,
            buffer_allocator,
        }
    }

    impl Scene {
        fn run(&mut self) -> u32 {
            self.e
                .call(0x00d9_3ba0, &args![self.owner, SECOND, self.job])
                .u32()
        }

        /// The addresses of the calls in `of`, in the order they happened.
        fn order(&self, of: &[u32]) -> Vec<u32> {
            self.rig
                .log
                .borrow()
                .iter()
                .map(|call| call.0)
                .filter(|address| of.contains(address))
                .collect()
        }
    }

    #[test]
    fn the_last_thread_of_a_single_thread_group_runs_everything() {
        let mut s = scene(1, 1, 1, (2, 1), 1);
        // The entity needs preparing and has five bodies for a limit of 2 per sector.
        s.e.mem.set_u8(s.world + 0xd5, 1);
        s.e.mem.set_u8(s.entity + 0x26, 4);
        s.e.mem.set_u32(s.entity + 0x60, 5);
        s.e.mem.set_u32(s.world + 0xe0, 2);
        // Two nodes of impulse records (16 bytes each), the first of one record.
        let second_node = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(second_node, 2);
        for word in 0..8 {
            s.e.mem.set_u32(second_node + 0x10 + 4 * word, 5 + word);
        }
        let first_node = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(first_node, 1);
        s.e.mem.set_u32(first_node + 4, second_node);
        for word in 0..4 {
            s.e.mem.set_u32(first_node + 0x10 + 4 * word, 1 + word);
        }
        s.e.mem.set_u32(s.group + 0x24, first_node);
        // What the group holds that is freed with it.
        let page_two = s.e.mem.alloc(0x10);
        let page_one = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(page_one, page_two);
        s.e.mem.set_u32(s.group + 0x68, page_one);
        let small = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(s.group + 0x78, small);
        s.e.mem.set_u32(s.group + 0x14, 0x1400);
        s.e.mem.set_u32(s.group + 0x1c, 0x1c0);
        s.e.mem.set_u32(s.group + 0x5c, 0x5c00);
        s.e.mem.set_u32(s.group + 0x64, 4);
        let returned = s.run();
        assert_eq!(returned, 0x66);
        assert_eq!(s.e.mem.u8(s.group + 0x20), 2);
        assert_eq!(s.e.mem.u32(s.router), s.top);
        let log = &s.rig.log;
        assert_eq!(calls(log, PREPARE_ENTITY), vec![vec![s.world, s.entity]]);
        // The arrays of 4 entries take 0x80 bytes each off the stack allocator.
        let searched = calls(log, FIND_PAIRS);
        assert_eq!(searched.len(), 1);
        let hub = s.e.mem.u32(s.entity + 0x48);
        assert_eq!(searched[0][..3], [hub, 0x4c4c, s.world]);
        // The array to add to is 0x10 bytes above the other; the data of each starts
        // at the stack allocator's top, then 0x80 bytes up, and the capacity word is the count
        // with bit 31 set.
        assert_eq!(searched[0][3], searched[0][4] + 0x10);
        assert_eq!(searched[0][5..], [s.top, s.top + 0x80, 0x8000_0004]);
        // The counter went down without the import and the old value was the thread count.
        assert!(calls(log, INTERLOCKED_EXCHANGE_ADD).is_empty());
        assert_eq!(s.e.mem.u32(s.group + 0xc), 0);
        assert_eq!(
            calls(log, FIRE_IMPULSE_LIMIT_BREACHED),
            vec![vec![
                s.world,
                s.top + 0x100,
                3,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12
            ]]
        );
        assert_eq!(calls(log, LOCK_ISLAND), vec![vec![s.world, 0x1515]]);
        assert_eq!(
            calls(log, REMOVE_PAIRS),
            vec![vec![0x6400, s.top + 0x80, 1]]
        );
        assert_eq!(
            calls(log, ALLOCATOR + 0x24),
            vec![vec![s.allocator, 2 << 10]]
        );
        assert_eq!(calls(log, ADD_PAIRS), vec![vec![0x6400, s.top, 2, 0x7008]]);
        assert_eq!(calls(log, UNLOCK_ISLAND), vec![vec![s.world, 0x1515]]);
        assert_eq!(
            s.order(&[
                PREPARE_ENTITY,
                FIND_PAIRS,
                FIRE_IMPULSE_LIMIT_BREACHED,
                LOCK_ISLAND,
                REMOVE_PAIRS,
                ADD_PAIRS,
                UNLOCK_ISLAND,
                ALLOCATE_SECTORS,
                FINISH_ADD_AND_GET_NEXT_JOB
            ]),
            vec![
                PREPARE_ENTITY,
                FIND_PAIRS,
                FIRE_IMPULSE_LIMIT_BREACHED,
                LOCK_ISLAND,
                REMOVE_PAIRS,
                ADD_PAIRS,
                UNLOCK_ISLAND,
                ALLOCATE_SECTORS,
                FINISH_ADD_AND_GET_NEXT_JOB
            ]
        );
        // The group is freed: the pages, the small nodes, the buffer, the 4-entry array, itself.
        assert_eq!(
            calls(log, ALLOCATOR + 8),
            vec![
                vec![s.allocator, page_one, 0x1000],
                vec![s.allocator, page_two, 0x1000],
                vec![s.allocator, small, 0x30],
                vec![s.allocator, s.group, 0x80],
            ]
        );
        assert_eq!(
            calls(log, BUFFER_ALLOCATOR + 0x10),
            vec![vec![s.buffer_allocator, 0x1400, 0x1c0]]
        );
        assert_eq!(
            calls(log, ALLOCATOR + 0x10),
            vec![vec![s.allocator, 0x5c00, 16]]
        );
        assert_eq!(s.e.mem.u32(s.group + 0x68), 0);
        assert_eq!(s.e.mem.u32(s.group + 0x14), 0);
        assert_eq!(s.e.mem.u32(s.group + 0x64), 0x8000_0000);
        // The job is a type-0xD job for three sectors.
        assert_eq!(calls(log, ALLOCATE_SECTORS), vec![vec![3, 0x10]]);
        assert_eq!(
            calls(log, FINISH_ADD_AND_GET_NEXT_JOB),
            vec![vec![SECOND, 0, 1, s.job, 0]]
        );
        let job = s.job;
        assert_eq!(
            [s.e.mem.u8(job), s.e.mem.u8(job + 1), s.e.mem.u8(job + 2)],
            [0xd, 1, 1]
        );
        assert_eq!([s.e.mem.u16(job + 4), s.e.mem.u16(job + 6)], [0x50, 0xffff]);
        assert_eq!(
            [
                s.e.mem.u16(job + 0x24),
                s.e.mem.u16(job + 0x26),
                s.e.mem.u16(job + 0x28),
                s.e.mem.u16(job + 0x2a),
                s.e.mem.u16(job + 0x44)
            ],
            [0, 3, 5, 2, 0x8585]
        );
        assert_eq!(
            [
                s.e.mem.u32(job + 0x20),
                s.e.mem.u32(job + 0x2c),
                s.e.mem.u32(job + 0x30),
                s.e.mem.u32(job + 0x3c),
                s.e.mem.u32(job + 0x40)
            ],
            [0xabc0, 0x5c5c, 0xa1, 0xa4, 0]
        );
        assert_eq!(
            s.rig.labels_with_wide(&s.e, LABEL_LT_ADD_REMOVE_AGENTS),
            vec![
                LABEL_TT_BROADPHASE,
                LABEL_TT_WAIT_FOR_EXPORT,
                PROFILE_END,
                LABEL_LT_ADD_REMOVE_AGENTS,
                LABEL_ST_INIT,
                LABEL_ST_REMOVE_AGENT,
                LABEL_ST_ADD_AGENT,
                LABEL_JOB_END,
                PROFILE_END
            ]
        );
    }

    #[test]
    fn a_thread_that_is_not_last_decrements_twice_and_leaves_the_group() {
        let mut s = scene(3, 2, 2, (0, 0), 1);
        let returned = s.run();
        s.rig.assert_finished(returned, SECOND, s.job);
        let counter = s.group + 0xc;
        assert_eq!(
            calls(&s.rig.log, INTERLOCKED_EXCHANGE_ADD),
            vec![vec![counter, 0xffff_ffff], vec![counter, 0xffff_ffff]]
        );
        assert_eq!(s.e.mem.u32(counter), 0);
        // Nothing to prepare, fire, remove or free, and the flag is not touched.
        assert!(calls(&s.rig.log, PREPARE_ENTITY).is_empty());
        assert!(calls(&s.rig.log, FIRE_IMPULSE_LIMIT_BREACHED).is_empty());
        assert!(calls(&s.rig.log, LOCK_ISLAND).is_empty());
        assert!(calls(&s.rig.log, ALLOCATOR + 8).is_empty());
        assert_eq!(s.e.mem.u32(s.router), s.top);
        assert_eq!(
            s.rig.labels_with_wide(&s.e, LABEL_LT_ADD_REMOVE_AGENTS),
            vec![LABEL_TT_BROADPHASE, PROFILE_END]
        );
    }

    #[test]
    fn the_second_of_two_threads_frees_the_group_and_skips_adding_without_a_grant() {
        let mut s = scene(2, 1, 2, (1, 0), 0);
        s.e.mem.set_u32(s.entity + 0x60, 5);
        s.e.mem.set_u32(s.group + 0x50, 0x5000);
        s.e.mem.set_u16(s.group + 0x56, 6);
        s.e.mem.set_u32(s.group + 0x5c, 0x5c00);
        s.e.mem.set_u32(s.group + 0x64, 0x10);
        let returned = s.run();
        assert_eq!(returned, 0x66);
        // Two threads: the counter is decremented once by the import and the answer is -1.
        let counter = s.group + 0xc;
        assert_eq!(
            calls(&s.rig.log, INTERLOCKED_EXCHANGE_ADD),
            vec![vec![counter, 0xffff_ffff]]
        );
        assert_eq!(
            calls(&s.rig.log, REMOVE_PAIRS),
            vec![vec![0x6400, s.top + 0x80, 0]]
        );
        assert_eq!(
            calls(&s.rig.log, ALLOCATOR + 0x24),
            vec![vec![s.allocator, 1 << 10]]
        );
        assert!(calls(&s.rig.log, ADD_PAIRS).is_empty());
        assert_eq!(
            calls(&s.rig.log, UNLOCK_ISLAND),
            vec![vec![s.world, 0x1515]]
        );
        // The entries at +0x50 are freed because there is more than one thread.
        assert_eq!(
            calls(&s.rig.log, ALLOCATOR + 0x10),
            vec![
                vec![s.allocator, 0x5000, 24],
                vec![s.allocator, 0x5c00, 0x40]
            ]
        );
        assert_eq!(s.e.mem.u32(s.group + 0x50), 0);
        // Five bodies are within the limit of 8: no sector header.
        assert!(calls(&s.rig.log, ALLOCATE_SECTORS).is_empty());
        assert_eq!(s.e.mem.u32(s.job + 0x20), 0);
        assert_eq!(
            s.rig.labels_with_wide(&s.e, LABEL_LT_ADD_REMOVE_AGENTS),
            vec![
                LABEL_TT_BROADPHASE,
                LABEL_LT_ADD_REMOVE_AGENTS,
                LABEL_ST_INIT,
                LABEL_ST_REMOVE_AGENT,
                LABEL_JOB_END,
                PROFILE_END
            ]
        );
    }
}
