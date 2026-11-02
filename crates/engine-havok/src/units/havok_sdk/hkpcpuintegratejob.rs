//! `Havok SDK/hkpcpuintegratejob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions work on a "task" (the job's word at `+0x14`): its entity list is the pointer at
//! `+0x48` with the count at `+0x4C`, the constraint count is at `+0x1C` and `+0xC` is non-zero
//! when the constraints have to be solved. They build an "island" object (0x80 bytes, allocated
//! through the memory router `00c85750`, whose `+0x10` is the heap allocator) that the later
//! jobs share. Words that the exe copies out of stack residue (padding) are zero here.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpcpuagentsectorjob::{JOB_DONE, LABEL_JOB_END, LABEL_LT_INTEGRATE};
use crate::units::havok_sdk::hkpcpubroadphasejob::FINISH_ADD_AND_GET_NEXT_JOB;
use crate::units::havok_sdk::hkpcpuintegratemotionjob::{
    APPLY_FORCES_AND_STEP, ENTITY_BATCH_RECALC_AABB,
};
use crate::units::havok_sdk::hkpcpukdtreebuildjob::{
    ADD_JOB_QUEUE_LOCKED, LOCK_QUEUE, UNLOCK_QUEUE,
};
use crate::units::havok_sdk::hkpcpusolveconstraintsjobs::ADD_JOB;
use crate::units::havok_sdk::hkpcpusplitsimulationislandjob::SPLIT_ISLAND;
use crate::units::havok_sdk::hkpcpuworldraycastjob::ARRAY_RESERVE;
use crate::units::havok_sdk::hkpsphereshape::{
    profile_record, PROFILE_END, PROFILE_SLOT, TLS_GET_VALUE,
};

/// The memory router (`00c85750`, no stack words): `+0` is the top of the stack allocator, `+0xC`
/// and `+0x10` are heap allocators and `+0x18` is the allocator that hands out the solver buffer.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The import slot of `TlsSetValue` and the global that holds the thread-local-storage index the
/// job queue keeps its per-thread state in.
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
const TLS_INDEX: u32 = 0x0126_87c8;
/// `hkpConstraintSolverSetup::calcBufferSize` (name per the engine map, derived from the call
/// graph; cdecl, one word: the task; returns the size in bytes).
const CALC_BUFFER_SIZE: u32 = 0x00d8_89f0;
/// `hkpConstraintSolverSetup::calcBufferOffsetsForSolve` (name per the engine map, derived from
/// the call graph; cdecl, four words: the task, the buffer, its size and the island).
const CALC_BUFFER_OFFSETS_FOR_SOLVE: u32 = 0x00d8_83c0;
/// `hkpConstraintSolverSetup::solve` (name per the engine map, derived from the call graph;
/// cdecl, eight words: the step information at `world + 0x1D0`, the one at `world + 0x1E0`, a
/// pointer to a copy of the 0x80 bytes at `owner + 0x10`, the task, the buffer, the buffer
/// size, the entity list and the entity count; returns a count).
const SOLVE: u32 = 0x00d8_8b50;
/// `00d05190` (in `hkjobqueue.cpp`, no name in the engine map): `ECX` = the queue, two stack
/// words: a 0x80-byte block and 1; its result is 0 when the exe gives the job back.
const REQUEST_JOB: u32 = 0x00d0_5190;
/// `hkJobQueue::finishJob` (Xbox PDB name per the engine map, `00d04d80`): `ECX` = the queue, two
/// stack words: a job block and 0.
const FINISH_JOB: u32 = 0x00d0_4d80;
/// `hkMapBase<unsigned long, unsigned long, ...>` functions (`ECX` = the map): the constructor
/// `00cb1830` (name per the engine map, derived from the call graph; stack words: the storage
/// and its size in bytes), `00cb1870` (insert; no name in the engine map; stack words: the
/// allocator, the key and the value), `getWithDefault` `00cb1930` (name per the engine map,
/// derived from the call graph; stack words: the key and the default; returns the value),
/// `clearAndDeallocate` `00cb1a90` (name per the engine map, derived from the call graph; one
/// stack word: the allocator) and `00a29680` (no name in the engine map; no stack words).
const MAP_CONSTRUCT: u32 = 0x00cb_1830;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_RELEASE: u32 = 0x00a2_9680;

/// `createBuildJacobianAndSolveJacobianTaskCollection`, called through the engine so that the
/// tests can stand in for it.
const TASK_COLLECTION: u32 = 0x00d9_5130;

/// Profiling labels: `"StInit"`, `"StActions"`, `"StIsland Splitting"`, `"StSplitIslands"`,
/// `"StSingleObj"`, `"StSolver 1Cpu"`, `"StSetupJobs"`, `"TtBuildJacTask"` and the number
/// labels `"MiNumEntities"`, `"MiNumConstraints"`, `"MiNumJacobians"`.
const LABEL_ST_INIT: u32 = 0x010c_c5bc;
const LABEL_ST_ACTIONS: u32 = 0x010c_c5b0;
const LABEL_ST_ISLAND_SPLITTING: u32 = 0x010d_3f64;
const LABEL_ST_SPLIT_ISLANDS: u32 = 0x010d_3f54;
const LABEL_ST_SINGLE_OBJ: u32 = 0x010d_3f48;
const LABEL_ST_SOLVER_1CPU: u32 = 0x010d_3f38;
const LABEL_ST_SETUP_JOBS: u32 = 0x010d_3f2c;
const LABEL_TT_BUILD_JAC_TASK: u32 = 0x010d_3f1c;
const LABEL_MI_NUM_ENTITIES: u32 = 0x010d_3d60;
const LABEL_MI_NUM_CONSTRAINTS: u32 = 0x010d_3f08;
const LABEL_MI_NUM_JACOBIANS: u32 = 0x010d_3d70;

/// The capacity bits of an array's capacity word, and the bit that says the storage is not
/// owned.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

/// The frame `integrate_job` keeps its blocks in: the job block for `finishJob`, the 0x80-byte
/// block `00d05190` fills, the size cell the solver-buffer allocator works on, the two job
/// blocks that are added or finished, the two result structures of the task collection builder
/// (the second one at `+8`), the copy of the 0x80 bytes `solve` takes and the 380-byte scratch
/// area `lockQueue` takes.
const FINISH_BLOCK: u32 = 0x000;
const REQUEST_BLOCK: u32 = 0x020;
const SIZE_CELL: u32 = 0x0a0;
const ADD_BLOCK: u32 = 0x0c0;
const DONE_BLOCK: u32 = 0x0e0;
const COLLECTION_OUT: u32 = 0x100;
const SOLVE_COPY: u32 = 0x180;
const LOCK_SCRATCH: u32 = 0x200;

/// The solver buffer of a task: the size the task asked for, the size the allocator granted and
/// the address (0 when there is none).
struct SolverBuffer {
    size: u32,
    granted: u32,
    address: u32,
}

/// Asks the allocator at `+0x18` of the memory router for `requested` bytes through the size
/// cell at `cell` (slot `0xC`); returns the size granted and the address.
fn take_buffer(e: &mut Engine, cell: u32, requested: u32) -> (u32, u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x18);
    e.mem.set_u32(cell, requested);
    let address = e.vcall(allocator, 0xc, &args![cell]).u32();
    (e.mem.u32(cell), address)
}

/// Gives the job back to the queue when the solver buffer cannot be had: `addJob` with 1, a
/// finish block (type 0x10), and the 0x80 bytes the queue request filled copied over the job.
fn give_job_back(e: &mut Engine, queue: u32, job: u32, frame: u32) -> u32 {
    e.call(ADD_JOB, &args![queue, job, 1u32]);
    e.mem.set_u8(frame + FINISH_BLOCK, 0x10);
    e.mem.set_u8(frame + FINISH_BLOCK + 1, 0);
    e.call(FINISH_JOB, &args![queue, frame + FINISH_BLOCK, 0u32]);
    for word in 0..0x20 {
        let value = e.mem.u32(frame + REQUEST_BLOCK + 4 * word);
        e.mem.set_u32(job + 4 * word, value);
    }
    0
}

/// Fills the job block that follows the solving: `kind` at `+0`, 0 at `+1`, `flag` at `+2`,
/// 0xFFFF at `+6`.
fn set_block_header(e: &mut Engine, block: u32, kind: u8, flag: u8) {
    e.mem.set_u8(block, kind);
    e.mem.set_u8(block + 1, 0);
    e.mem.set_u8(block + 2, flag);
    e.mem.set_u16(block + 6, 0xffff);
}

/// The ending every path that solved on this thread shares: the island splitting when it is
/// wanted, the closing record, the island's byte at `+0x20` set, the job turned into a type-0xC
/// block and handed to `finishAddAndGetNextJob`.
fn finish_island(e: &mut Engine, queue: u32, job: u32, task: u32, island: u32, split: bool) -> u32 {
    if split {
        e.call(SPLIT_ISLAND, &args![task, island + 0x5c]);
    }
    profile_record(e, LABEL_JOB_END);
    e.mem.set_u8(island + 0x20, 1);
    set_block_header(e, job, 0xc, 2);
    e.mem.set_u16(job + 4, 0x30);
    e.mem.set_u16(job + 0x20, 1);
    e.mem.set_u32(job + 0x18, island);
    e.call(
        FINISH_ADD_AND_GET_NEXT_JOB,
        &args![queue, 0u32, 0u32, job, 0u32],
    );
    0
}

// Translated from 00d95cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `integrateJob` (Xbox PDB name): cdecl, three stack words: the owner (its first word is the
/// world, whose `+0x1D0` and `+0x1E0` are the step information the solver takes), the word that
/// becomes `ECX` of every job queue call, and the job (its task is the word at `+0x14`).
///
/// With a task that has constraints (`+0xC` non-zero) it asks the solver-buffer allocator for
/// `calcBufferSize` bytes; while that fails it lets the queue hand out another job
/// (`00d05190` with the thread-local-storage value temporarily 1) and, when none is left, gives
/// the job back (`addJob`, `finishJob` with a type-0x10 block, the 0x80 bytes of the request
/// copied over the job) and returns 0. Then it writes the start record (`LtIntegrate`,
/// `StInit`), runs the actions of the task (the objects of the list at `task + 0x38`, vtable
/// slot `0xC` with the world's step information) unless the world's byte at `+0xB0` is set, and
/// builds the island: heap object of 0x80 bytes with the entity list (`+0x50`, count at `+0x54`)
/// and the task's other fields. The byte at `task + 0x24` is incremented and, with the mask byte
/// at `task + 0x25` and the world's thresholds (`+0xB4`, byte `+0xC8`), decides whether the
/// island is split afterwards. Then:
///  - without constraints and with the world's byte at `+0xB1` set it queues the island's jobs:
///    the job becomes a type-0xB block, and under the queue lock (`lockQueue`) a type-5 block is
///    added (`addJobQueueLocked`) when the split is wanted; it ends with
///    `finishAddAndGetNextJob` and returns what that returns;
///  - without constraints and without that byte it integrates the entities in one go
///    (`hkRigidMotionUtilApplyForcesAndStep`, then `entityBatchRecalcAabb`); a result of 5 or
///    less clears the island's word at `+8`;
///  - with constraints, when the larger of the entity count and the constraint count is below
///    the threshold at `[owner + 0x110] + 0xB4` (unsigned) it solves on this thread
///    (`hkpConstraintSolverSetup::solve`, a result of 5 or less clears the island's word at
///    `+8`) and frees the buffer; otherwise it sets the buffer offsets up
///    (`calcBufferOffsetsForSolve`), queues the job as a type-1 job, builds the task collection
///    (`createBuildJacobianAndSolveJacobianTaskCollection` with the byte at `world + 0x304`),
///    copies its results into the island and ends with `00d04e80` and a type-2 block, returning
///    what it returns.
///
/// The paths that solved here split the island if wanted, write the closing record and end with
/// `finishAddAndGetNextJob` and a type-0xC job, returning 0. The bytes of the blocks the exe
/// leaves unwritten and the padding word the island copies from a stack word are zero here.
pub fn integrate_job(e: &mut Engine, owner: Ptr, queue: u32, job: Ptr) -> u32 {
    let owner = owner.addr();
    let job = job.addr();
    e.with_stack(0x400, |e, frame| {
        integrate(e, owner, queue, job, frame.addr())
    })
}

fn integrate(e: &mut Engine, owner: u32, queue: u32, job: u32, frame: u32) -> u32 {
    let task = e.mem.u32(job + 0x14);
    let mut buffer = SolverBuffer {
        size: 0,
        granted: 0,
        address: 0,
    };
    if e.mem.u32(task + 0xc) != 0 {
        buffer.size = e.call(CALC_BUFFER_SIZE, &args![task]).u32();
        (buffer.granted, buffer.address) = take_buffer(e, frame + SIZE_CELL, buffer.size);
        while buffer.address == 0 {
            let index: u32 = e.global(TLS_INDEX);
            let saved = e.call(TLS_GET_VALUE, &args![index]).u32();
            let index: u32 = e.global(TLS_INDEX);
            e.call(TLS_SET_VALUE, &args![index, 1u32]);
            let handed_out = e
                .call(REQUEST_JOB, &args![queue, frame + REQUEST_BLOCK, 1u32])
                .u32();
            let index: u32 = e.global(TLS_INDEX);
            e.call(TLS_SET_VALUE, &args![index, saved]);
            if handed_out == 0 {
                return give_job_back(e, queue, job, frame);
            }
            // The exe spins through 4000 empty iterations here.
            (buffer.granted, buffer.address) = take_buffer(e, frame + SIZE_CELL, buffer.granted);
        }
    }
    profile_start_record(e, LABEL_LT_INTEGRATE, LABEL_ST_INIT);
    let world = e.mem.u32(owner);
    let step_info = world.wrapping_add(0x1d0);
    if e.mem.u8(world + 0xb0) == 0 {
        profile_record(e, LABEL_ST_ACTIONS);
        if e.mem.i32(task + 0x3c) > 0 {
            let mut index = 0u32;
            loop {
                let action = e.mem.u32(e.mem.u32(task + 0x38).wrapping_add(4 * index));
                if action != 0 {
                    e.vcall(action, 0xc, &args![step_info]);
                }
                index += 1;
                if index as i32 >= e.mem.i32(task + 0x3c) {
                    break;
                }
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let heap = e.mem.u32(router + 0x10);
    let object = e.vcall(heap, 4, &args![0x80u32]).u32();
    let island = if object != 0 {
        for offset in [0x14, 0x18, 0x1c, 0x5c, 0x60] {
            e.mem.set_u32(object + offset, 0);
        }
        e.mem.set_u32(object + 0xc, 1);
        e.mem.set_u32(object + 0x10, 1);
        e.mem.set_u32(object + 0x64, 0x8000_0000);
        for offset in [0x68, 0x6c, 0x70, 0x74, 0x78] {
            e.mem.set_u32(object + offset, 0);
        }
        e.mem.set_u16(object + 0x7c, 0);
        object
    } else {
        0
    };
    profile_record(e, LABEL_ST_ISLAND_SPLITTING);
    let count = e.mem.u32(task + 0x4c);
    let entities = e.mem.u32(task + 0x48);
    e.mem.set_u16(island + 0x54, count as u16);
    e.mem.set_u32(island + 0x50, entities);
    let flags = e.mem.u16(task + 0x50);
    e.mem.set_u16(island + 0x56, flags);
    e.mem.set_u32(island + 4, 0);
    e.mem.set_u32(island + 8, 1);
    for offset in [0x14, 0x18, 0x24, 0x68, 0x30] {
        e.mem.set_u32(island + offset, 0);
    }
    profile_record(e, LABEL_ST_SPLIT_ISLANDS);
    let counter = e.mem.u8(task + 0x24).wrapping_add(1);
    e.mem.set_u8(task + 0x24, counter);
    let mask = e.mem.u8(task + 0x25);
    let split = if mask & 0xc == 0 {
        mask & 3 != 0 && counter & 7 == 0 && e.mem.u8(e.mem.u32(owner) + 0xc8) != 0
    } else {
        let entity_count = e.mem.i32(task + 0x4c);
        let constraint_count = e.mem.i32(task + 0x1c);
        let larger = if entity_count > constraint_count {
            entity_count
        } else {
            constraint_count
        };
        if larger < e.mem.i32(e.mem.u32(owner) + 0xb4) {
            counter & 7 == 0
        } else {
            e.mem.set_u8(task + 0x25, mask & 0xf3);
            counter & 3 == 0
        }
    };
    if e.mem.u32(task + 0xc) == 0 {
        let world = e.mem.u32(owner);
        if e.mem.u8(world + 0xb1) != 0 {
            queue_island_jobs(
                e,
                owner,
                queue,
                job,
                frame,
                (task, island),
                buffer.granted,
                split,
            );
            return e
                .call(
                    FINISH_ADD_AND_GET_NEXT_JOB,
                    &args![queue, 0u32, 0u32, job, 0u32],
                )
                .u32();
        }
        profile_record(e, LABEL_ST_SINGLE_OBJ);
        let result = e
            .call(
                APPLY_FORCES_AND_STEP,
                &args![
                    step_info.wrapping_add(0x10),
                    step_info,
                    step_info.wrapping_add(0x30),
                    e.mem.u32(task + 0x48),
                    e.mem.u32(task + 0x4c),
                    0xe0u32
                ],
            )
            .i32();
        let world_word = e.mem.u32(e.mem.u32(task + 0x18) + 0x78);
        let (entity_list, entity_count) = (e.mem.u32(task + 0x48), e.mem.u32(task + 0x4c));
        e.call(
            ENTITY_BATCH_RECALC_AABB,
            &args![world_word, entity_list, entity_count],
        );
        if result <= 5 {
            e.mem.set_u32(island + 8, 0);
        }
    } else {
        let larger = {
            let (entity_count, constraint_count) = (e.mem.i32(task + 0x4c), e.mem.i32(task + 0x1c));
            if constraint_count > entity_count {
                constraint_count
            } else {
                entity_count
            }
        };
        let threshold = e.mem.u32(e.mem.u32(owner + 0x110) + 0xb4);
        if larger as u32 >= threshold {
            let cached = (count, entities);
            return solve_with_jobs(
                e,
                owner,
                queue,
                job,
                frame,
                (task, island),
                cached,
                &buffer,
                split,
            );
        }
        profile_record(e, LABEL_ST_SOLVER_1CPU);
        for word in 0..0x20 {
            let value = e.mem.u32(owner + 0x10 + 4 * word);
            e.mem.set_u32(frame + SOLVE_COPY + 4 * word, value);
        }
        let result = e
            .call(
                SOLVE,
                &args![
                    step_info,
                    step_info.wrapping_add(0x10),
                    frame + SOLVE_COPY,
                    task,
                    buffer.address,
                    buffer.size,
                    entities,
                    count
                ],
            )
            .i32();
        if result <= 5 {
            e.mem.set_u32(island + 8, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x18);
        e.vcall(allocator, 0x10, &args![buffer.address, buffer.granted]);
    }
    finish_island(e, queue, job, task, island, split)
}

/// The path of `integrate_job` for a task without constraints in a world whose byte at `+0xB1`
/// is set: the island's fields, the job turned into a type-0xB block and the queue calls under
/// the lock (a type-5 block is added when `split` is set).
fn queue_island_jobs(
    e: &mut Engine,
    owner: u32,
    queue: u32,
    job: u32,
    frame: u32,
    (task, island): (u32, u32),
    granted: u32,
    split: bool,
) {
    for offset in [0x14, 0x1c, 0x30, 0x3c, 0x40, 0x18] {
        e.mem.set_u32(island + offset, 0);
    }
    let (word_10, word_14) = (e.mem.u32(task + 0x10), e.mem.u32(task + 0x14));
    e.mem.set_u32(island + 0x44, word_10);
    e.mem.set_u32(island + 0x48, word_14);
    e.mem.set_u32(island + 0x1c, granted);
    e.mem.set_u8(island + 0x21, u8::from(split));
    e.mem.set_u32(island + 0x78, 0);
    e.mem.set_u32(island + 0x4c, owner.wrapping_add(0x10));
    let count = e.mem.u32(task + 0x4c);
    let blocks = (count.wrapping_sub(1) >> 7).wrapping_add(1);
    e.mem
        .set_u32(island + 4, blocks.wrapping_add(u32::from(split)));
    set_block_header(e, job, 0xb, 2);
    e.mem.set_u16(job + 4, 0x40);
    e.mem.set_u32(job + 0x20, 0);
    e.mem.set_u16(job + 0x36, 0);
    let low = e.mem.u16(island + 0x54);
    e.mem.set_u16(job + 0x34, low);
    let word_14 = e.mem.u32(island + 0x14);
    e.mem.set_u32(job + 0x30, word_14);
    e.mem.set_u8(job + 0x3c, 0);
    e.mem.set_u32(job + 0x18, island);
    e.mem.set_u8(job + 0x3c, 1);
    e.mem.set_u8(job + 2, 1);
    e.mem.set_u8(island + 0x20, 1);
    e.mem.set_u32(island, blocks);
    profile_record(e, LABEL_JOB_END);
    let token = e
        .call(LOCK_QUEUE, &args![queue, frame + LOCK_SCRATCH])
        .u32();
    if split {
        let add = frame + ADD_BLOCK;
        set_block_header(e, add, 5, 2);
        e.mem.set_u16(add + 4, 0x20);
        let (word_10, word_14, word_18, word_1c) = (
            e.mem.u16(job + 0x10),
            e.mem.u32(job + 0x14),
            e.mem.u32(job + 0x18),
            e.mem.u32(job + 0x1c),
        );
        e.mem.set_u16(add + 0x10, word_10);
        e.mem.set_u32(add + 0x14, word_14);
        e.mem.set_u32(add + 0x18, word_18);
        e.mem.set_u32(add + 0x1c, word_1c);
        e.call(ADD_JOB_QUEUE_LOCKED, &args![queue, token, add, 0u32]);
    }
    e.call(UNLOCK_QUEUE, &args![queue, token]);
}

/// The path of `integrate_job` for a task with constraints that is too large to solve on this
/// thread: the buffer offsets, the type-1 job, the task collection and the type-2 block that
/// ends with `00d04e80`.
fn solve_with_jobs(
    e: &mut Engine,
    owner: u32,
    queue: u32,
    job: u32,
    frame: u32,
    (task, island): (u32, u32),
    (count, entities): (u32, u32),
    buffer: &SolverBuffer,
    split: bool,
) -> u32 {
    profile_record(e, LABEL_ST_SETUP_JOBS);
    e.call(
        CALC_BUFFER_OFFSETS_FOR_SOLVE,
        &args![task, buffer.address, buffer.size, island],
    );
    let (word_10, word_14) = (e.mem.u32(task + 0x10), e.mem.u32(task + 0x14));
    e.mem.set_u32(island + 0x44, word_10);
    e.mem.set_u32(island + 0x1c, buffer.granted);
    e.mem.set_u32(island + 0x48, word_14);
    e.mem.set_u8(island + 0x21, u8::from(split));
    e.mem.set_u32(island + 0x4c, owner.wrapping_add(0x10));
    let blocks = count.wrapping_sub(1) >> 7;
    e.mem.set_u32(
        island + 4,
        blocks.wrapping_add(1).wrapping_add(u32::from(split)),
    );
    e.mem.set_u8(island + 0x20, 0);
    e.mem.set_u32(island, blocks.wrapping_add(2));
    e.mem.set_u16(job + 0x24, 0);
    e.mem.set_u8(job + 2, 1);
    e.mem.set_u8(job, 1);
    e.mem.set_u32(job + 0x18, island);
    e.mem.set_u32(job + 0x20, entities);
    e.mem.set_u16(job + 0x26, count as u16);
    e.call(ADD_JOB, &args![queue, job, 0u32]);
    let results = frame + COLLECTION_OUT;
    let groups = results;
    let chunks = results + 8;
    let world = e.mem.u32(owner);
    let flag = u32::from(e.mem.u8(world.wrapping_add(0x1d0).wrapping_add(0x134)));
    e.call(TASK_COLLECTION, &args![task, flag, island, chunks, groups]);
    profile_record(e, LABEL_JOB_END);
    for (offset, from) in [
        (0x68, chunks),
        (0x6c, chunks + 4),
        (0x70, chunks + 8),
        (0x74, chunks + 12),
    ] {
        let value = e.mem.u32(from);
        e.mem.set_u32(island + offset, value);
    }
    let value = e.mem.u32(groups);
    e.mem.set_u32(island + 0x78, value);
    let counter = e.mem.u16(groups + 4);
    e.mem.set_u32(island + 0x7c, u32::from(counter));
    let done = frame + DONE_BLOCK;
    set_block_header(e, done, 2, 1);
    e.mem.set_u16(done + 4, 0x30);
    let (word_10, word_14, word_1c) = (
        e.mem.u16(job + 0x10),
        e.mem.u32(job + 0x14),
        e.mem.u32(job + 0x1c),
    );
    e.mem.set_u16(done + 0x10, word_10);
    e.mem.set_u32(done + 0x14, word_14);
    e.mem.set_u32(done + 0x18, island);
    e.mem.set_u32(done + 0x1c, word_1c);
    e.call(JOB_DONE, &args![queue, done, job, 0u32]).u32()
}

/// The frame the task collection builder keeps its blocks in: the two arrays of constraint
/// pairs (data, size, capacity word; the first one takes the constraints whose type byte at
/// `+0x13` is below 4), the hash map object, the cell that stands in for the table entry of an
/// entity index of 0xFFFF and the pair record (the constraint at `+0`, the two entity indices
/// as 16-bit words at `+4` and `+6`).
const ARRAY_LOW: u32 = 0x00;
const ARRAY_HIGH: u32 = 0x10;
const ENTITY_MAP: u32 = 0x20;
const NO_ENTITY_CELL: u32 = 0x48;
const PAIR_RECORD: u32 = 0x50;

/// Appends a 8-byte profiling record (`label`, the value as a `float`) to the thread's record
/// stream while its cursor (`+4`) is below its limit (`+0xC`).
fn profile_number(e: &mut Engine, label: u32, value: i32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_f32(cursor + 4, value as f32);
        e.mem.set_u32(stream + 4, cursor + 8);
    }
}

/// Makes sure the array at `array` (data, size, capacity word) can take `wanted` pairs of 8
/// bytes: when its capacity is smaller, `hkArrayUtil::_reserve` (`00c908c0`, cdecl, four words:
/// the allocator at `+0xC` of the memory router, the array, the new capacity and the element
/// size) is asked for the larger of `wanted` and twice the capacity.
fn reserve_pairs(e: &mut Engine, array: u32, wanted: i32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let target = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![allocator, array, target, 8u32]);
    }
}

/// The address of the table entry of the entity index `index`, or of `none` for 0xFFFF.
fn table_slot(table: u32, none: u32, index: u16) -> u32 {
    if index == 0xffff {
        none
    } else {
        table.wrapping_add(u32::from(index) * 4)
    }
}

/// What the task collection builder keeps between the chunks it creates.
struct Collection {
    island: u32,
    /// The out structures: `chunks` = the first chunk (`+0`), the number of chunks (`+4`), the
    /// array of records (`+8`) and its length (`+0xC`); `groups` = the first group object
    /// (`+0`) and, as a 16-bit word at `+4`, the number of groups in the current series.
    chunks: u32,
    groups: u32,
    /// The records (16 bytes: the constraint, its pair word, the address of its chunk entry) taken
    /// off the stack allocator, and how many there are.
    records: u32,
    records_used: u32,
    /// The 0x90 pairs (constraint, pair word) waiting to go into the next chunk, and how many.
    pending_base: u32,
    pending: u32,
    /// The running sums of the island's words at `+0x3C` and `+0x40`.
    sum_first: u32,
    sum_second: u32,
    /// Set when every constraint has its byte at `+0x1B` set: the groups are built then.
    make_groups: bool,
    /// The pass the last group was made in (-1 for none) and the current pass.
    last_group_pass: u32,
    pass: u32,
    /// The last chunk (0 for none), the last group, the one before and the one before that, and
    /// the cell that counts the groups of the current series.
    previous_chunk: u32,
    last_group: u32,
    earlier_group: u32,
    earliest_group: u32,
    series_counter: u32,
}

impl Collection {
    /// Turns the first `count` pending pairs into a chunk of 0x1000 bytes (heap allocator, slot
    /// 4): the chunk is linked after the previous one (`+0`, with the running sum at `+0x10`),
    /// counted in the out structure and filled with an entry of 0x1C bytes per pair at `+0x18`;
    /// pairs whose constraint has bits in its byte at `+0x12` (mask `0xB`) also get a record. The
    /// sums grow by the constraints' words at `+0x14` and (times 4) `+0x18`; afterwards the first
    /// one is 0x10 larger and the second one rounded up to 16. When groups are made, a 0x30-byte
    /// group object is allocated, linked after the last group, chained through the series
    /// when the pass has changed and filled with the sums of this chunk.
    fn flush(&mut self, e: &mut Engine, count: u32) {
        let (first_before, second_before) = (self.sum_first, self.sum_second);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let heap = e.mem.u32(router + 0x10);
        let chunk = e.vcall(heap, 4, &args![0x1000u32]).u32();
        if chunk != 0 {
            e.mem.set_u32(chunk + 0x14, 0);
            e.mem.set_u32(chunk + 0x10, 0);
            e.mem.set_u32(chunk, 0);
        }
        if self.previous_chunk == 0 {
            e.mem.set_u32(self.chunks, chunk);
            e.mem.set_u32(self.chunks + 4, 0);
        } else {
            e.mem.set_u32(self.previous_chunk, chunk);
            e.mem.set_u32(self.previous_chunk + 0x10, self.sum_first);
        }
        let chunks = e.mem.u32(self.chunks + 4);
        e.mem.set_u32(self.chunks + 4, chunks.wrapping_add(1));
        e.mem.set_u32(chunk + 4, self.island);
        e.mem.set_u32(chunk + 0x14, count);
        let word_30 = e.mem.u32(self.island + 0x30);
        e.mem.set_u32(chunk + 8, word_30);
        e.mem.set_u32(chunk + 0xc, self.sum_first);
        for index in 0..count {
            let entry = chunk + 0x18 + 0x1c * index;
            let pending = self.pending_base + 8 * index;
            let constraint = e.mem.u32(pending);
            if e.mem.u8(constraint + 0x12) & 0xb != 0 {
                let record = self.records + 16 * self.records_used;
                self.records_used += 1;
                e.mem.set_u32(record, constraint);
                let pair = e.mem.u32(pending + 4);
                e.mem.set_u32(record + 4, pair);
                e.mem.set_u32(record + 8, entry);
            }
            let words = [
                (0x00, e.mem.u32(constraint + 0xc)),
                (0x04, e.mem.u32(constraint)),
                (0x08, e.mem.u32(constraint + 0x1c)),
                (0x0c, e.mem.u32(constraint + 4).wrapping_add(0xf0)),
                (0x10, e.mem.u32(constraint + 8).wrapping_add(0xf0)),
            ];
            for (offset, value) in words {
                e.mem.set_u32(entry + offset, value);
            }
            let halves = [
                (0x14, e.mem.u16(constraint + 0x10)),
                (0x16, e.mem.u16(constraint + 0x20)),
                (0x18, e.mem.u16(pending + 4).wrapping_add(1)),
                (0x1a, e.mem.u16(pending + 6).wrapping_add(1)),
            ];
            for (offset, value) in halves {
                e.mem.set_u16(entry + offset, value);
            }
            self.sum_first = self
                .sum_first
                .wrapping_add(u32::from(e.mem.u16(constraint + 0x14)));
            self.sum_second = self
                .sum_second
                .wrapping_add(4 * u32::from(e.mem.u16(constraint + 0x18)));
        }
        self.sum_second = self.sum_second.wrapping_add(0xf) & 0xffff_fff0;
        self.sum_first = self.sum_first.wrapping_add(0x10);
        self.pending = 0;
        e.mem.set_u32(chunk + 0x10, self.sum_first);
        self.previous_chunk = chunk;
        if self.make_groups {
            self.make_group(e, first_before, second_before);
        }
    }

    /// The group object `flush` makes after a chunk (see there); `first_before` and
    /// `second_before` are the sums before the chunk.
    fn make_group(&mut self, e: &mut Engine, first_before: u32, second_before: u32) {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let heap = e.mem.u32(router + 0x10);
        let group = e.vcall(heap, 4, &args![0x30u32]).u32();
        if group != 0 {
            e.mem.set_u32(group, 0);
            e.mem.set_u8(group + 0x1c, 0);
            e.mem.set_u16(group + 0x1e, 0);
            e.mem.set_u32(group + 0x20, 0);
        }
        if self.last_group != 0 {
            e.mem.set_u32(self.last_group, group);
        }
        if self.pass != self.last_group_pass {
            if self.pass == 0 {
                e.mem.set_u32(self.groups, group);
            }
            self.last_group_pass = self.pass;
            if self.last_group != 0 {
                let counter = e.mem.u16(self.series_counter);
                self.close_series(e, counter);
                e.mem.set_u8(self.last_group + 0x1c, 1);
                self.series_counter = self.last_group + 0x1e;
            }
            self.earliest_group = self.earlier_group;
            self.earlier_group = group;
        }
        let counter = e.mem.u16(self.series_counter);
        e.mem.set_u16(self.series_counter, counter.wrapping_add(1));
        e.mem.set_u32(group + 4, self.island);
        let word_30 = e.mem.u32(self.island + 0x30);
        e.mem.set_u32(group + 8, word_30);
        e.mem.set_u32(group + 0xc, first_before);
        e.mem.set_u32(group + 0x10, second_before);
        e.mem
            .set_u32(group + 0x14, self.sum_first.wrapping_sub(first_before));
        e.mem
            .set_u32(group + 0x18, self.sum_second.wrapping_sub(second_before));
        self.last_group = group;
    }

    /// Writes the series' length `counter` (16 bits at `+0x1E`) and the group that ends the series
    /// (`+0x20`) into the groups from the earliest one on, through their links (`+0`), up to
    /// the earlier group.
    fn close_series(&self, e: &mut Engine, counter: u16) {
        let mut group = self.earliest_group;
        while group != 0 && group != self.earlier_group {
            e.mem.set_u16(group + 0x1e, counter);
            e.mem.set_u32(group + 0x20, self.earlier_group);
            group = e.mem.u32(group);
        }
    }
}

// Translated from 00d95130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `createBuildJacobianAndSolveJacobianTaskCollection` (Xbox PDB name): cdecl, five stack words:
/// the task, a byte flag, the island and the two out structures `chunks` and `groups` (see
/// `Collection`; both are written, the 16-bit counter at `groups + 4` is set to 0 first). No
/// result.
///
/// Writes `TtBuildJacTask` and splits the task's constraints into chunks of at most 0x90
/// pairs. Each entity (the list at `task + 0x48`, count at `+0x4C`) gets `0x80 * (index + 1)` in
/// its word at `+0xA4`; a hash map from entity to index is built (storage from the allocator at
/// `+0xC` of the memory router, `hkMapBase` functions `00cb1830`, `00cb1870`, `00cb1930`,
/// `00cb1a90`, `00a29680`) and released again. The entities' constraint arrays (`+0xAC`, 0x30
/// bytes each, count as a 16-bit word at `+0xB0`) are walked; for each constraint the pair
/// (constraint, the indices of its two entities, 0xFFFF when the other one is not in the list)
/// goes into the array for types below 4 (byte `+0x13`) or into the one for the others; the
/// groups are only made when every constraint has its byte `+0x1B` set (the island's bytes at
/// `+0x58` and `+0x59` say the opposite). Then `MiNumEntities`, `MiNumConstraints` and
/// `MiNumJacobians` are recorded. The two arrays are processed in turn: in each pass a table of
/// one word per entity is cleared, and pairs whose entities are free or already claimed by the
/// current group (table value 0 or the group number, starting at 1) are moved to the pending
/// list and claim their entities, while the others stay in the array for the next pass; with 0x90
/// pending pairs a chunk is made (and the group number goes up when groups are made), and when
/// the byte flag is set and the group number is above 1 the rest of the pass is left for the
/// next one. Records, chunks and groups are described at `Collection`; the records are
/// copied into an array of 8-byte entries (constraint, the entry address) from the heap
/// allocator at `+0x10` into `chunks + 8` and `chunks + 0xC`. With groups the island's
/// words at `+0x34`, `+0x38`/`+0x3A` are set from its 16-bit word at `+0x54`. The stack
/// allocator is given back to where it was, the arrays released, and `Et` (`010c41f8`) is
/// recorded.
///
/// The exe takes 0x30 bytes at a 4-byte-aligned address for the group objects and clears a
/// release it never makes (it tests the not-owned bit of a capacity word it has just set), so
/// the record array is never released. With no constraint at all and groups wanted the exe
/// writes through a null group pointer (a fault); this translation does the same.
pub fn create_build_jacobian_and_solve_jacobian_task_collection(
    e: &mut Engine,
    task: Ptr,
    flag: u32,
    island: Ptr,
    chunks: Ptr,
    groups: Ptr,
) {
    let (task, island, chunks, groups) = (task.addr(), island.addr(), chunks.addr(), groups.addr());
    e.with_stack(0x80, |e, frame| {
        build_task_collection(e, task, flag as u8, island, (chunks, groups), frame.addr());
    });
}

fn build_task_collection(
    e: &mut Engine,
    task: u32,
    flag: u8,
    island: u32,
    (chunks, groups): (u32, u32),
    frame: u32,
) {
    profile_record(e, LABEL_TT_BUILD_JAC_TASK);
    let count = e.mem.u32(task + 0x4c);
    let entities = e.mem.u32(task + 0x48);
    let entities_end = entities.wrapping_add(count.wrapping_mul(4));
    let table_bytes = count.wrapping_mul(4);
    let array_low = frame + ARRAY_LOW;
    let array_high = frame + ARRAY_HIGH;
    for array in [array_low, array_high] {
        let wanted = e.mem.i32(task + 0x1c);
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        reserve_pairs(e, array, wanted);
    }
    let constraint_count = e.mem.u32(task + 0x1c);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let records = e.mem.u32(router);
    let records_size = constraint_count.wrapping_shl(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, records.wrapping_add(records_size));
    // The hash table has 0x40 entries, or the next power of two at or above the entity count.
    let mut entries = 0x40i32;
    if count as i32 > entries {
        loop {
            entries = entries.wrapping_add(entries);
            if entries >= count as i32 {
                break;
            }
        }
    }
    let map_bytes = (entries as u32).wrapping_mul(16);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let map_storage = e.vcall(allocator, 4, &args![map_bytes]).u32();
    let map = frame + ENTITY_MAP;
    e.call(MAP_CONSTRUCT, &args![map, map_storage, map_bytes]);
    if count as i32 > 0 {
        for index in 0..count {
            let entity = e.mem.u32(entities.wrapping_add(4 * index));
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(MAP_INSERT, &args![map, allocator, entity, index]);
        }
    }
    let none = frame + NO_ENTITY_CELL;
    let pair = frame + PAIR_RECORD;
    let mut all_flagged = 1u32;
    let mut offset = 0x80u32;
    let mut entity_index = 0u32;
    let mut cursor = entities;
    if cursor < entities_end {
        loop {
            let entity = e.mem.u32(cursor);
            if offset != e.mem.u32(entity + 0xa4) {
                e.mem.set_u32(entity + 0xa4, offset);
            }
            let mut constraint = e.mem.u32(entity + 0xac);
            offset = offset.wrapping_add(0x80);
            let end = constraint.wrapping_add(u32::from(e.mem.u16(entity + 0xb0)) * 0x30);
            while constraint < end {
                let own_side = u32::from(e.mem.u8(constraint + 0x1a));
                let other_side = 1u32.wrapping_sub(own_side);
                e.mem.set_u32(pair, constraint);
                e.mem
                    .set_u16(pair.wrapping_add(4 + 2 * own_side), entity_index as u16);
                let other = e.mem.u32(constraint.wrapping_add(4 + 4 * other_side));
                let found = e
                    .call(MAP_GET_WITH_DEFAULT, &args![map, other, 0xffffu32])
                    .u32();
                e.mem
                    .set_u16(pair.wrapping_add(4 + 2 * other_side), found as u16);
                all_flagged &= u32::from(e.mem.u8(constraint + 0x1b) != 0);
                let array = if e.mem.u8(constraint + 0x13) < 4 {
                    array_low
                } else {
                    array_high
                };
                let at = e
                    .mem
                    .u32(array)
                    .wrapping_add(e.mem.u32(array + 4).wrapping_mul(8));
                if at != 0 {
                    let (first, second) = (e.mem.u32(pair), e.mem.u32(pair + 4));
                    e.mem.set_u32(at, first);
                    e.mem.set_u32(at + 4, second);
                }
                let size = e.mem.u32(array + 4);
                e.mem.set_u32(array + 4, size.wrapping_add(1));
                constraint = constraint.wrapping_add(0x30);
            }
            entity_index = entity_index.wrapping_add(1);
            cursor = cursor.wrapping_add(4);
            if cursor >= entities_end {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_RELEASE, &args![map]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    e.vcall(allocator, 8, &args![map_storage, map_bytes]);
    profile_number(e, LABEL_MI_NUM_ENTITIES, count as i32);
    let constraints = e.mem.i32(task + 0x1c);
    profile_number(e, LABEL_MI_NUM_CONSTRAINTS, constraints);
    let jacobians = e.mem.i32(task + 0x10);
    profile_number(e, LABEL_MI_NUM_JACOBIANS, jacobians);
    let not_flagged = u8::from(all_flagged == 0);
    e.mem.set_u8(island + 0x59, not_flagged);
    e.mem.set_u8(island + 0x58, u8::from(not_flagged != 0));
    let make_groups = not_flagged == 0;
    e.mem.set_u16(groups + 4, 0);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let pending_base = e.mem.u32(router);
    e.mem.set_u32(router, pending_base.wrapping_add(0x480));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let table = e.vcall(allocator, 4, &args![table_bytes]).u32();
    let mut state = Collection {
        island,
        chunks,
        groups,
        records,
        records_used: 0,
        pending_base,
        pending: 0,
        sum_first: e.mem.u32(island + 0x3c),
        sum_second: e.mem.u32(island + 0x40),
        make_groups,
        last_group_pass: 0xffff_ffff,
        pass: 0xffff_ffff,
        previous_chunk: 0,
        last_group: 0,
        earlier_group: 0,
        earliest_group: 0,
        series_counter: groups + 4,
    };
    for phase in 0..2 {
        let array = if phase == 0 {
            state.previous_chunk = 0;
            array_low
        } else {
            array_high
        };
        if e.mem.u32(array + 4) == 0 {
            continue;
        }
        loop {
            if count as i32 > 0 {
                for index in 0..count {
                    e.mem.set_u32(table + 4 * index, 0);
                }
            }
            let data = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            state.pass = state.pass.wrapping_add(1);
            let mut group_number = 1u32;
            let end = data.wrapping_add(size.wrapping_mul(8));
            let mut read = data;
            let mut write = data;
            if read < end {
                loop {
                    e.mem.set_u32(none, 0);
                    let slot_first = table_slot(table, none, e.mem.u16(read + 4));
                    let slot_second = table_slot(table, none, e.mem.u16(read + 6));
                    if state.pending == 0x90 {
                        state.flush(e, 0x90);
                        group_number += u32::from(state.make_groups);
                    }
                    if flag != 0 && group_number as i32 > 1 {
                        while read < end {
                            let (first, second) = (e.mem.u32(read), e.mem.u32(read + 4));
                            e.mem.set_u32(write, first);
                            e.mem.set_u32(write + 4, second);
                            read = read.wrapping_add(8);
                            write = write.wrapping_add(8);
                        }
                        break;
                    }
                    let value_first = e.mem.u32(slot_first);
                    let free = (value_first == 0 || value_first == group_number) && {
                        let value_second = e.mem.u32(slot_second);
                        value_second == 0 || value_second == group_number
                    };
                    if free {
                        let at = state
                            .pending_base
                            .wrapping_add(state.pending.wrapping_mul(8));
                        if at != 0 {
                            let (first, second) = (e.mem.u32(read), e.mem.u32(read + 4));
                            e.mem.set_u32(at, first);
                            e.mem.set_u32(at + 4, second);
                        }
                        state.pending += 1;
                        e.mem.set_u32(slot_first, group_number);
                        e.mem.set_u32(slot_second, group_number);
                    } else {
                        let (first, second) = (e.mem.u32(read), e.mem.u32(read + 4));
                        e.mem.set_u32(write, first);
                        e.mem.set_u32(write + 4, second);
                        write = write.wrapping_add(8);
                    }
                    read = read.wrapping_add(8);
                    if read >= end {
                        break;
                    }
                }
            }
            if state.pending != 0 {
                let pending = state.pending;
                state.flush(e, pending);
            }
            let remaining = (write.wrapping_sub(data) as i32) >> 3;
            e.mem.set_u32(array + 4, remaining as u32);
            if remaining == 0 {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    e.vcall(allocator, 8, &args![table, table_bytes]);
    if state.make_groups {
        let counter = e.mem.u16(state.series_counter);
        state.close_series(e, counter);
        e.mem.set_u8(state.last_group + 0x1c, 1);
        let used = u32::from(e.mem.u16(island + 0x54));
        let start = used.wrapping_add(1).wrapping_shl(7);
        let base = e.mem.u32(island + 0x30);
        e.mem.set_u32(island + 0x34, start.wrapping_add(base));
        let groups_total = (used.wrapping_sub(1) >> 7).wrapping_add(1) as u16;
        e.mem.set_u16(island + 0x38, groups_total);
        e.mem.set_u16(island + 0x3a, groups_total);
    }
    e.mem.set_u32(chunks + 0xc, 0);
    e.mem.set_u32(chunks + 8, 0);
    if state.records_used != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let copies = e
            .vcall(allocator, 4, &args![state.records_used.wrapping_mul(8)])
            .u32();
        e.mem.set_u32(chunks + 0xc, state.records_used);
        e.mem.set_u32(chunks + 8, copies);
        for index in 0..state.records_used {
            let record = state.records + 16 * index;
            let (constraint, entry) = (e.mem.u32(record), e.mem.u32(record + 8));
            e.mem.set_u32(copies + 8 * index, constraint);
            e.mem.set_u32(copies + 8 * index + 4, entry);
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, pending_base);
    e.call(MEMORY_ROUTER, &args![]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, records);
    // The exe next tests the not-owned bit of the capacity word it set for the record array and
    // finds it set, so nothing is released.
    for array in [array_high, array_low] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if capacity & NOT_OWNED == 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(8)],
            );
        }
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
    }
    profile_record(e, PROFILE_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d95130,
            create_build_jacobian_and_solve_jacobian_task_collection(Ptr, u32, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d95cf0, integrate_job(Ptr, u32, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, Log,
    };
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    const HEAP: u32 = 0x0140_1000;
    const SOLVER: u32 = 0x0140_2000;
    const QUEUE: u32 = 0x5151_0000;
    const BUFFER_SIZE: u32 = 0x200;
    const TLS: u32 = 5;

    /// A double at `address` that logs its argument words followed by `words` words of the block
    /// its argument number `block` points to (the frame blocks are gone after the call), and
    /// answers `eax`.
    fn capture(e: &mut Engine, log: &Log, address: u32, eax: u32, block: usize, words: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let mut logged = a.to_vec();
            logged.extend((0..words).map(|word| e.mem.u32(a[block] + 4 * word)));
            sink.borrow_mut().push((address, logged));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// A heap allocator object whose slot 4 (`base + 4`) hands out zeroed blocks and whose slots
    /// 8 and 0x10 only log.
    fn heap_allocator(e: &mut Engine, log: &Log, base: u32) -> u32 {
        let sink = Rc::clone(log);
        e.register_double(base + 4, move |e, a| {
            sink.borrow_mut().push((base + 4, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        log_double(e, log, base + 8, 0);
        log_double(e, log, base + 0x10, 0);
        let vtable = e.mem.alloc(0x20);
        for slot in [4, 8, 0x10] {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The solver-buffer allocator: slot 0xC takes the size cell, fails (leaving 0x30 in the
    /// cell) for the first `failures` calls and grants the requested size plus 0x40 afterwards;
    /// slot 0x10 logs. The slot 0xC calls log (object, cell, size in the cell).
    fn solver_allocator(e: &mut Engine, log: &Log, base: u32, failures: u32) -> u32 {
        let sink = Rc::clone(log);
        let mut seen = 0u32;
        e.register_double(base + 0xc, move |e, a| {
            let requested = e.mem.u32(a[1]);
            sink.borrow_mut()
                .push((base + 0xc, vec![a[0], a[1], requested]));
            seen += 1;
            if seen <= failures {
                e.mem.set_u32(a[1], 0x30);
                return Ret::default();
            }
            e.mem.set_u32(a[1], requested + 0x40);
            Ret {
                eax: e.mem.alloc(requested + 0x40),
                ..Ret::default()
            }
        });
        log_double(e, log, base + 0x10, 0);
        let vtable = e.mem.alloc(0x20);
        for slot in [0xc, 0x10] {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The memory router object with the allocators at `+0xC`, `+0x10` and `+0x18` (the stack
    /// allocator's top at `+0` is `top`); returns its address.
    fn memory_router(e: &mut Engine, top: u32, allocators: [u32; 3]) -> u32 {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, top);
        e.mem.set_u32(router + 0xc, allocators[0]);
        e.mem.set_u32(router + 0x10, allocators[1]);
        e.mem.set_u32(router + 0x18, allocators[2]);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        router
    }

    /// The pieces of the `integrate_job` tests: a world, an owner, a task with 6 entities and 4
    /// constraints and a job.
    struct Scene {
        e: Engine,
        rig: JobRig,
        owner: u32,
        world: u32,
        task: u32,
        job: u32,
        entities: u32,
    }

    impl Scene {
        fn run(&mut self) -> u32 {
            self.e
                .call(0x00d9_5cf0, &args![self.owner, QUEUE, self.job])
                .u32()
        }

        fn island(&self) -> u32 {
            self.e.mem.u32(self.job + 0x18)
        }

        /// The labels written: the start record's two labels, then the 12-byte records.
        fn labels(&self) -> Vec<u32> {
            self.rig.labels(&self.e, true)
        }
    }

    /// Doubles for every function the job calls (they answer 0 unless said otherwise), a task
    /// with constraints that the solver should not touch (its flag at `+0xC` is 0) and a world
    /// whose actions are skipped (byte `+0xB0`).
    fn scene() -> Scene {
        let (mut e, rig) = job_rig();
        let heap = heap_allocator(&mut e, &rig.log, HEAP);
        let solver = solver_allocator(&mut e, &rig.log, SOLVER, 0);
        memory_router(&mut e, 0, [0, heap, solver]);
        for (address, eax) in [
            (CALC_BUFFER_SIZE, BUFFER_SIZE),
            (CALC_BUFFER_OFFSETS_FOR_SOLVE, 0),
            (SOLVE, 9),
            (REQUEST_JOB, 0),
            (FINISH_JOB, 0),
            (APPLY_FORCES_AND_STEP, 7),
            (ENTITY_BATCH_RECALC_AABB, 0),
            (SPLIT_ISLAND, 0),
            (ADD_JOB, 0),
            (TLS_SET_VALUE, 0),
            (LOCK_QUEUE, 0x4444),
            (UNLOCK_QUEUE, 0),
            (ADD_JOB_QUEUE_LOCKED, 0),
            (FINISH_ADD_AND_GET_NEXT_JOB, 0x77),
            (0x00d9_5130, 0),
        ] {
            log_double(&mut e, &rig.log, address, eax);
        }
        e.set_global(TLS_INDEX, TLS);
        let world = e.mem.alloc(0x400);
        e.mem.set_u8(world + 0xb0, 1);
        e.mem.set_u32(world + 0xb4, 1000);
        let limits = e.mem.alloc(0x100);
        e.mem.set_u32(limits + 0xb4, 100);
        let owner = e.mem.alloc(0x120);
        e.mem.set_u32(owner, world);
        e.mem.set_u32(owner + 0x110, limits);
        for word in 0..0x20 {
            e.mem.set_u32(owner + 0x10 + 4 * word, 0x100 + word);
        }
        let entities = e.mem.alloc(0x40);
        let holder = e.mem.alloc(0x80);
        e.mem.set_u32(holder + 0x78, 0x7777);
        let list = e.mem.alloc(0x20);
        let task = e.mem.alloc(0x60);
        for (offset, value) in [
            (0x10, 0x11),
            (0x14, 0x14),
            (0x18, holder),
            (0x1c, 4),
            (0x38, list),
            (0x48, entities),
            (0x4c, 6),
        ] {
            e.mem.set_u32(task + offset, value);
        }
        e.mem.set_u16(task + 0x50, 0x1234);
        let job = e.mem.alloc(0x80);
        e.mem.set_u32(job + 0x14, task);
        e.mem.set_u16(job + 0x10, 0xabcd);
        e.mem.set_u32(job + 0x1c, 0x1c1c);
        Scene {
            e,
            rig,
            owner,
            world,
            task,
            job,
            entities,
        }
    }

    #[test]
    fn a_task_without_constraints_is_integrated_in_one_go() {
        let mut s = scene();
        let returned = s.run();
        assert_eq!(returned, 0);
        let island = s.island();
        let world = s.world;
        assert_eq!(
            calls(&s.rig.log, APPLY_FORCES_AND_STEP),
            vec![vec![
                world + 0x1e0,
                world + 0x1d0,
                world + 0x200,
                s.entities,
                6,
                0xe0
            ]]
        );
        assert_eq!(
            calls(&s.rig.log, ENTITY_BATCH_RECALC_AABB),
            vec![vec![0x7777, s.entities, 6]]
        );
        assert!(calls(&s.rig.log, CALC_BUFFER_SIZE).is_empty());
        // The island: allocated through the heap allocator, initialised and filled from the task.
        assert_eq!(calls(&s.rig.log, HEAP + 4).len(), 1);
        assert_eq!(s.e.mem.u32(island + 0xc), 1);
        assert_eq!(s.e.mem.u32(island + 0x10), 1);
        assert_eq!(s.e.mem.u32(island + 0x64), 0x8000_0000);
        assert_eq!(s.e.mem.u32(island + 0x50), s.entities);
        assert_eq!(s.e.mem.u16(island + 0x54), 6);
        assert_eq!(s.e.mem.u16(island + 0x56), 0x1234);
        assert_eq!(s.e.mem.u32(island + 4), 0);
        assert_eq!(s.e.mem.u32(island + 8), 1);
        assert_eq!(s.e.mem.u8(island + 0x20), 1);
        // The job becomes the type-0xC block for the final call.
        assert_eq!(s.e.mem.u8(s.job), 0xc);
        assert_eq!(s.e.mem.u8(s.job + 1), 0);
        assert_eq!(s.e.mem.u8(s.job + 2), 2);
        assert_eq!(s.e.mem.u16(s.job + 4), 0x30);
        assert_eq!(s.e.mem.u16(s.job + 6), 0xffff);
        assert_eq!(s.e.mem.u16(s.job + 0x20), 1);
        assert_eq!(
            calls(&s.rig.log, FINISH_ADD_AND_GET_NEXT_JOB),
            vec![vec![QUEUE, 0, 0, s.job, 0]]
        );
        assert_eq!(
            s.labels(),
            vec![
                LABEL_LT_INTEGRATE,
                LABEL_ST_INIT,
                LABEL_ST_ISLAND_SPLITTING,
                LABEL_ST_SPLIT_ISLANDS,
                LABEL_ST_SINGLE_OBJ,
                LABEL_JOB_END
            ]
        );
        // A result of 5 or less clears the island's word at +8.
        log_double(&mut s.e, &s.rig.log, APPLY_FORCES_AND_STEP, 5);
        s.run();
        assert_eq!(s.e.mem.u32(s.island() + 8), 0);
    }

    #[test]
    fn the_island_is_split_by_the_counters_and_the_world_thresholds() {
        // (mask byte, counter byte before, world byte +0xC8, entity count, expected split,
        // mask byte after). The task has 4 constraints and the world's threshold is 1000.
        let cases = [
            (0x01, 7, 1, 6, true, 0x01),
            (0x01, 7, 0, 6, false, 0x01),
            (0x01, 8, 1, 6, false, 0x01),
            (0x00, 7, 1, 6, false, 0x00),
            (0x04, 7, 0, 6, true, 0x04),
            (0x04, 6, 0, 6, false, 0x04),
            (0x07, 3, 0, 2000, true, 0x03),
            (0x07, 4, 0, 2000, false, 0x03),
            (0x0c, 3, 0, 2000, true, 0x00),
        ];
        for (mask, counter, byte_c8, entity_count, split, mask_after) in cases {
            let mut s = scene();
            s.e.mem.set_u8(s.task + 0x24, counter);
            s.e.mem.set_u8(s.task + 0x25, mask);
            s.e.mem.set_u8(s.world + 0xc8, byte_c8);
            s.e.mem.set_u32(s.task + 0x4c, entity_count);
            s.run();
            let splits = calls(&s.rig.log, SPLIT_ISLAND);
            let expected = if split {
                vec![vec![s.task, s.island() + 0x5c]]
            } else {
                vec![]
            };
            assert_eq!(splits, expected, "case {mask:#x} {counter} {byte_c8}");
            assert_eq!(s.e.mem.u8(s.task + 0x24), counter + 1);
            assert_eq!(s.e.mem.u8(s.task + 0x25), mask_after);
            // The island splitting comes before the closing record.
            let labels = s.labels();
            assert_eq!(labels.last(), Some(&LABEL_JOB_END));
        }
    }

    #[test]
    fn the_actions_of_the_task_run_unless_the_world_says_no() {
        let actions = |s: &mut Scene| {
            let first = logging_object(&mut s.e, &s.rig.log, 0x0140_5000);
            let second = logging_object(&mut s.e, &s.rig.log, 0x0140_6000);
            let list = s.e.mem.u32(s.task + 0x38);
            s.e.mem.set_u32(list, first);
            s.e.mem.set_u32(list + 8, second);
            s.e.mem.set_u32(s.task + 0x3c, 3);
            (first, second)
        };
        let mut s = scene();
        actions(&mut s);
        s.run();
        assert!(calls(&s.rig.log, 0x0140_500c).is_empty());
        assert!(!s.labels().contains(&LABEL_ST_ACTIONS));
        // With the world's byte clear, the non-null actions get the world's step information.
        let mut s = scene();
        let (first, second) = actions(&mut s);
        s.e.mem.set_u8(s.world + 0xb0, 0);
        s.run();
        let step = s.world + 0x1d0;
        assert_eq!(calls(&s.rig.log, 0x0140_500c), vec![vec![first, step]]);
        assert_eq!(calls(&s.rig.log, 0x0140_600c), vec![vec![second, step]]);
        assert_eq!(
            s.labels()[..3],
            [LABEL_LT_INTEGRATE, LABEL_ST_INIT, LABEL_ST_ACTIONS]
        );
    }

    #[test]
    fn a_world_with_the_job_flag_queues_the_island_jobs_of_a_task_without_constraints() {
        for split in [true, false] {
            let mut s = scene();
            s.e.mem.set_u8(s.world + 0xb1, 1);
            if split {
                s.e.mem.set_u8(s.task + 0x24, 7);
                s.e.mem.set_u8(s.task + 0x25, 1);
                s.e.mem.set_u8(s.world + 0xc8, 1);
            }
            capture(&mut s.e, &s.rig.log, ADD_JOB_QUEUE_LOCKED, 0, 2, 8);
            let returned = s.run();
            assert_eq!(returned, 0x77);
            let island = s.island();
            assert!(calls(&s.rig.log, APPLY_FORCES_AND_STEP).is_empty());
            // Lock, add (only when splitting), unlock, in that order, then the final call.
            let order: Vec<u32> = s
                .rig
                .log
                .borrow()
                .iter()
                .map(|(address, _)| *address)
                .filter(|address| {
                    [
                        LOCK_QUEUE,
                        ADD_JOB_QUEUE_LOCKED,
                        UNLOCK_QUEUE,
                        FINISH_ADD_AND_GET_NEXT_JOB,
                    ]
                    .contains(address)
                })
                .collect();
            let mut expected = vec![LOCK_QUEUE];
            if split {
                expected.push(ADD_JOB_QUEUE_LOCKED);
            }
            expected.extend([UNLOCK_QUEUE, FINISH_ADD_AND_GET_NEXT_JOB]);
            assert_eq!(order, expected);
            let locks = calls(&s.rig.log, LOCK_QUEUE);
            assert_eq!((locks.len(), locks[0][0]), (1, QUEUE));
            assert_eq!(calls(&s.rig.log, UNLOCK_QUEUE), vec![vec![QUEUE, 0x4444]]);
            let added = calls(&s.rig.log, ADD_JOB_QUEUE_LOCKED);
            if split {
                let block = added[0][2];
                assert_eq!(
                    added,
                    vec![vec![
                        QUEUE,
                        0x4444,
                        block,
                        0,
                        0x0002_0005,
                        0xffff_0020,
                        0,
                        0,
                        0xabcd,
                        s.task,
                        island,
                        0x1c1c
                    ]]
                );
            } else {
                assert!(added.is_empty());
            }
            // The island and the job.
            assert_eq!(s.e.mem.u32(island + 0x44), 0x11);
            assert_eq!(s.e.mem.u32(island + 0x48), 0x14);
            assert_eq!(s.e.mem.u32(island + 0x1c), 0);
            assert_eq!(s.e.mem.u8(island + 0x21), u8::from(split));
            assert_eq!(s.e.mem.u32(island + 0x4c), s.owner + 0x10);
            assert_eq!(s.e.mem.u32(island + 4), 1 + u32::from(split));
            assert_eq!(s.e.mem.u32(island), 1);
            assert_eq!(s.e.mem.u8(island + 0x20), 1);
            assert_eq!(s.e.mem.u8(s.job), 0xb);
            assert_eq!(s.e.mem.u8(s.job + 2), 1);
            assert_eq!(s.e.mem.u16(s.job + 4), 0x40);
            assert_eq!(s.e.mem.u16(s.job + 0x34), 6);
            assert_eq!(s.e.mem.u16(s.job + 0x36), 0);
            assert_eq!(s.e.mem.u8(s.job + 0x3c), 1);
            assert_eq!(
                calls(&s.rig.log, FINISH_ADD_AND_GET_NEXT_JOB),
                vec![vec![QUEUE, 0, 0, s.job, 0]]
            );
            assert_eq!(
                s.labels(),
                vec![
                    LABEL_LT_INTEGRATE,
                    LABEL_ST_INIT,
                    LABEL_ST_ISLAND_SPLITTING,
                    LABEL_ST_SPLIT_ISLANDS,
                    LABEL_JOB_END
                ]
            );
        }
    }

    #[test]
    fn a_small_task_with_constraints_is_solved_on_this_thread() {
        let mut s = scene();
        s.e.mem.set_u32(s.task + 0xc, 1);
        capture(&mut s.e, &s.rig.log, SOLVE, 9, 2, 0x20);
        let returned = s.run();
        assert_eq!(returned, 0);
        let island = s.island();
        let world = s.world;
        assert_eq!(calls(&s.rig.log, CALC_BUFFER_SIZE), vec![vec![s.task]]);
        let solve = calls(&s.rig.log, SOLVE);
        assert_eq!(solve.len(), 1);
        let buffer = solve[0][4];
        assert_eq!(
            solve[0][..8],
            [
                world + 0x1d0,
                world + 0x1e0,
                solve[0][2],
                s.task,
                buffer,
                BUFFER_SIZE,
                s.entities,
                6
            ]
        );
        let copied: Vec<u32> = (0..0x20).map(|word| 0x100 + word).collect();
        assert_eq!(solve[0][8..], copied[..]);
        // The buffer goes back to the allocator with the size it was granted.
        assert_ne!(buffer, 0);
        assert_eq!(
            calls(&s.rig.log, SOLVER + 0x10)[0][1..],
            [buffer, BUFFER_SIZE + 0x40]
        );
        assert_eq!(s.e.mem.u32(island + 8), 1);
        assert!(calls(&s.rig.log, SPLIT_ISLAND).is_empty());
        assert_eq!(
            s.labels(),
            vec![
                LABEL_LT_INTEGRATE,
                LABEL_ST_INIT,
                LABEL_ST_ISLAND_SPLITTING,
                LABEL_ST_SPLIT_ISLANDS,
                LABEL_ST_SOLVER_1CPU,
                LABEL_JOB_END
            ]
        );
        assert_eq!(
            calls(&s.rig.log, FINISH_ADD_AND_GET_NEXT_JOB),
            vec![vec![QUEUE, 0, 0, s.job, 0]]
        );
        // A result of 5 or less clears the island's word at +8.
        capture(&mut s.e, &s.rig.log, SOLVE, 3, 2, 0x20);
        s.run();
        assert_eq!(s.e.mem.u32(s.island() + 8), 0);
    }

    #[test]
    fn a_large_task_with_constraints_is_solved_by_jobs() {
        let mut s = scene();
        s.e.mem.set_u32(s.task + 0xc, 1);
        // The limit is 5 and the larger of the counts is 6.
        let limits = s.e.mem.u32(s.owner + 0x110);
        s.e.mem.set_u32(limits + 0xb4, 5);
        s.e.mem.set_u8(s.world + 0x304, 0x1f);
        let sink = s.rig.log.clone();
        s.e.register_double(0x00d9_5130, move |e, a| {
            sink.borrow_mut().push((0x00d9_5130, a.to_vec()));
            for (index, value) in [0xc1, 0xc2, 0xc3, 0xc4].into_iter().enumerate() {
                e.mem.set_u32(a[3] + 4 * index as u32, value);
            }
            e.mem.set_u32(a[4], 0x91);
            e.mem.set_u16(a[4] + 4, 0x92);
            Ret::default()
        });
        capture(&mut s.e, &s.rig.log, JOB_DONE, 0x55, 1, 8);
        let returned = s.run();
        assert_eq!(returned, 0x55);
        let island = s.island();
        let done = calls(&s.rig.log, JOB_DONE);
        let block = done[0][1];
        assert_eq!(
            done,
            vec![vec![
                QUEUE,
                block,
                s.job,
                0,
                0x0001_0002,
                0xffff_0030,
                0,
                0,
                0xabcd,
                s.task,
                island,
                0x1c1c
            ]]
        );
        let offsets = calls(&s.rig.log, CALC_BUFFER_OFFSETS_FOR_SOLVE);
        assert_eq!(
            offsets,
            vec![vec![s.task, offsets[0][1], BUFFER_SIZE, island]]
        );
        assert_eq!(calls(&s.rig.log, ADD_JOB), vec![vec![QUEUE, s.job, 0]]);
        let collection = calls(&s.rig.log, 0x00d9_5130);
        assert_eq!(collection[0][..3], [s.task, 0x1f, island]);
        assert!(calls(&s.rig.log, SOLVE).is_empty());
        // The island.
        assert_eq!(s.e.mem.u32(island + 0x44), 0x11);
        assert_eq!(s.e.mem.u32(island + 0x48), 0x14);
        assert_eq!(s.e.mem.u32(island + 0x1c), BUFFER_SIZE + 0x40);
        assert_eq!(s.e.mem.u8(island + 0x21), 0);
        assert_eq!(s.e.mem.u32(island + 0x4c), s.owner + 0x10);
        assert_eq!(s.e.mem.u32(island + 4), 1);
        assert_eq!(s.e.mem.u32(island), 2);
        assert_eq!(s.e.mem.u8(island + 0x20), 0);
        for (index, value) in [0xc1, 0xc2, 0xc3, 0xc4].into_iter().enumerate() {
            assert_eq!(s.e.mem.u32(island + 0x68 + 4 * index as u32), value);
        }
        assert_eq!(s.e.mem.u32(island + 0x78), 0x91);
        assert_eq!(s.e.mem.u32(island + 0x7c), 0x92);
        // The job.
        assert_eq!(s.e.mem.u8(s.job), 1);
        assert_eq!(s.e.mem.u8(s.job + 2), 1);
        assert_eq!(s.e.mem.u16(s.job + 0x24), 0);
        assert_eq!(s.e.mem.u32(s.job + 0x20), s.entities);
        assert_eq!(s.e.mem.u16(s.job + 0x26), 6);
        assert_eq!(
            s.labels(),
            vec![
                LABEL_LT_INTEGRATE,
                LABEL_ST_INIT,
                LABEL_ST_ISLAND_SPLITTING,
                LABEL_ST_SPLIT_ISLANDS,
                LABEL_ST_SETUP_JOBS,
                LABEL_JOB_END
            ]
        );
    }

    #[test]
    fn a_task_that_cannot_get_a_buffer_and_finds_no_other_job_goes_back_to_the_queue() {
        let mut s = scene();
        s.e.mem.set_u32(s.task + 0xc, 1);
        solver_allocator(&mut s.e, &s.rig.log, SOLVER, 1);
        // The request fills the 0x80-byte block with a pattern and finds nothing.
        let sink = s.rig.log.clone();
        s.e.register_double(REQUEST_JOB, move |e, a| {
            sink.borrow_mut().push((REQUEST_JOB, a.to_vec()));
            for word in 0..0x20 {
                e.mem.set_u32(a[1] + 4 * word, 0xa0 + word);
            }
            Ret::default()
        });
        capture(&mut s.e, &s.rig.log, FINISH_JOB, 0, 1, 2);
        let returned = s.run();
        assert_eq!(returned, 0);
        let request = calls(&s.rig.log, REQUEST_JOB);
        assert_eq!(request, vec![vec![QUEUE, request[0][1], 1]]);
        let tls = calls(&s.rig.log, TLS_SET_VALUE);
        assert_eq!((tls.len(), tls[0].clone()), (2, vec![TLS, 1]));
        assert_eq!(tls[1][0], TLS);
        assert_ne!(tls[1][1], 1);
        assert_eq!(calls(&s.rig.log, ADD_JOB), vec![vec![QUEUE, s.job, 1]]);
        let finish = calls(&s.rig.log, FINISH_JOB);
        assert_eq!(finish, vec![vec![QUEUE, finish[0][1], 0, 0x10, 0]]);
        for word in 0..0x20 {
            assert_eq!(s.e.mem.u32(s.job + 4 * word), 0xa0 + word);
        }
        assert!(s.rig.labels(&s.e, false).is_empty());
        assert!(calls(&s.rig.log, HEAP + 4).is_empty());
    }

    #[test]
    fn a_failed_buffer_request_is_retried_with_the_size_granted_so_far() {
        let mut s = scene();
        s.e.mem.set_u32(s.task + 0xc, 1);
        solver_allocator(&mut s.e, &s.rig.log, SOLVER, 1);
        log_double(&mut s.e, &s.rig.log, REQUEST_JOB, 1);
        let returned = s.run();
        assert_eq!(returned, 0);
        // The second request starts from the 0x30 the failed one left in the cell.
        let asked = calls(&s.rig.log, SOLVER + 0xc);
        assert_eq!(asked.len(), 2);
        assert_eq!(asked[0][2], BUFFER_SIZE);
        assert_eq!(asked[1][2], 0x30);
        assert_eq!(calls(&s.rig.log, REQUEST_JOB).len(), 1);
        assert_eq!(
            calls(&s.rig.log, TLS_SET_VALUE)
                .iter()
                .map(|call| call[1] == 1)
                .collect::<Vec<_>>(),
            vec![true, false]
        );
        // The job went on with the buffer from the second request: the solve got the size
        // asked for first and the free gets the size granted last.
        let solve = calls(&s.rig.log, SOLVE);
        assert_eq!(solve[0][5], BUFFER_SIZE);
        assert_eq!(
            calls(&s.rig.log, SOLVER + 0x10)[0][1..],
            [solve[0][4], 0x30 + 0x40]
        );
        assert!(calls(&s.rig.log, ADD_JOB).is_empty());
    }

    const HEAP_A: u32 = 0x0140_3000;
    const HEAP_B: u32 = 0x0140_4000;
    const ISLAND_BASE: u32 = 0x9000;
    const CHUNK_SIZE: u32 = 0x1000;

    /// One constraint of an entity: the entities it joins (`own` holds it, `other` is the far
    /// side), the side byte, the type byte, whether its byte at `+0x1B` is set and its flags.
    #[derive(Clone, Copy)]
    struct Spec {
        own: usize,
        other: usize,
        side: u8,
        kind: u8,
        flagged: bool,
        flags: u8,
    }

    fn spec(own: usize, other: usize) -> Spec {
        Spec {
            own,
            other,
            side: 0,
            kind: 0,
            flagged: true,
            flags: 0,
        }
    }

    /// The scene of the task collection builder: entities with constraint arrays, a task, an
    /// island, the out structures, the memory router and doubles for the hash map and the
    /// array reservation.
    struct Build {
        e: Engine,
        log: Log,
        stream: u32,
        records: u32,
        task: u32,
        island: u32,
        chunks: u32,
        groups: u32,
        router: u32,
        top: u32,
        heap_a: u32,
        heap_b: u32,
        entities: Vec<u32>,
        constraints: Vec<u32>,
    }

    /// Every constraint has 3 in its word at `+0x14` and 2 in its word at `+0x18`; its tag
    /// (`0x100 * (index + 1)`) is spread over the words the entries copy.
    fn build_scene(entity_count: usize, specs: &[Spec]) -> Build {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x400);
        let heap_a = heap_allocator(&mut e, &log, HEAP_A);
        let heap_b = heap_allocator(&mut e, &log, HEAP_B);
        let top = e.mem.alloc(0x8000);
        let router = memory_router(&mut e, top, [heap_a, heap_b, 0]);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            let data = e.mem.alloc(a[2] * 8);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            let mut logged = a.to_vec();
            logged.push(data);
            sink.borrow_mut().push((ARRAY_RESERVE, logged));
            Ret::default()
        });
        let table: Rc<RefCell<HashMap<u32, u32>>> = Rc::default();
        let inserted = Rc::clone(&table);
        let sink = Rc::clone(&log);
        e.register_double(MAP_INSERT, move |_, a| {
            sink.borrow_mut().push((MAP_INSERT, a.to_vec()));
            inserted.borrow_mut().insert(a[2], a[3]);
            Ret::default()
        });
        let looked_up = Rc::clone(&table);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: looked_up.borrow().get(&a[1]).copied().unwrap_or(a[2]),
            ..Ret::default()
        });
        for address in [MAP_CONSTRUCT, MAP_CLEAR_AND_DEALLOCATE, MAP_RELEASE] {
            log_double(&mut e, &log, address, 0);
        }
        let entities: Vec<u32> = (0..entity_count)
            .map(|_| {
                let entity = e.mem.alloc(0x100);
                e.mem.set_u32(entity + 0xa4, 0xdead);
                entity
            })
            .collect();
        let mut constraints = vec![0; specs.len()];
        for (index, entity) in entities.iter().enumerate() {
            let mine: Vec<usize> = (0..specs.len())
                .filter(|&number| specs[number].own == index)
                .collect();
            let array = e.mem.alloc(0x30 * mine.len() as u32);
            e.mem.set_u32(entity + 0xac, array);
            e.mem.set_u16(entity + 0xb0, mine.len() as u16);
            for (slot, &number) in mine.iter().enumerate() {
                let constraint = array + 0x30 * slot as u32;
                constraints[number] = constraint;
                let spec = specs[number];
                let tag = 0x100 * (number as u32 + 1);
                let (near, far) = (entities[spec.own], entities[spec.other]);
                let (first, second) = if spec.side == 0 {
                    (near, far)
                } else {
                    (far, near)
                };
                e.mem.set_u32(constraint + 4, first);
                e.mem.set_u32(constraint + 8, second);
                e.mem.set_u32(constraint + 0xc, tag + 1);
                e.mem.set_u32(constraint, tag + 2);
                e.mem.set_u32(constraint + 0x1c, tag + 3);
                e.mem.set_u16(constraint + 0x10, tag as u16 + 4);
                e.mem.set_u16(constraint + 0x20, tag as u16 + 5);
                e.mem.set_u16(constraint + 0x14, 3);
                e.mem.set_u16(constraint + 0x18, 2);
                e.mem.set_u8(constraint + 0x12, spec.flags);
                e.mem.set_u8(constraint + 0x13, spec.kind);
                e.mem.set_u8(constraint + 0x1a, spec.side);
                e.mem.set_u8(constraint + 0x1b, u8::from(spec.flagged));
            }
        }
        let list = e.mem.alloc(4 * entity_count as u32);
        for (index, entity) in entities.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *entity);
        }
        let task = e.mem.alloc(0x60);
        e.mem.set_u32(task + 0x10, 0x55);
        e.mem.set_u32(task + 0x1c, specs.len() as u32);
        e.mem.set_u32(task + 0x48, list);
        e.mem.set_u32(task + 0x4c, entity_count as u32);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + 0x30, ISLAND_BASE);
        e.mem.set_u32(island + 0x3c, 0x100);
        e.mem.set_u32(island + 0x40, 0x200);
        e.mem.set_u16(island + 0x54, 5);
        let chunks = e.mem.alloc(0x10);
        let groups = e.mem.alloc(0x10);
        Build {
            e,
            log,
            stream,
            records,
            task,
            island,
            chunks,
            groups,
            router,
            top,
            heap_a,
            heap_b,
            entities,
            constraints,
        }
    }

    impl Build {
        fn run(&mut self, flag: u32) {
            self.e.call(
                0x00d9_5130,
                &args![self.task, flag, self.island, self.chunks, self.groups],
            );
        }

        /// The chunk numbered `index` (0 for the first) by following the links.
        fn chunk(&self, index: usize) -> u32 {
            let mut chunk = self.e.mem.u32(self.chunks);
            for _ in 0..index {
                chunk = self.e.mem.u32(chunk);
            }
            chunk
        }

        /// The group numbered `index` (0 for the first) by following the links.
        fn group(&self, index: usize) -> u32 {
            let mut group = self.e.mem.u32(self.groups);
            for _ in 0..index {
                group = self.e.mem.u32(group);
            }
            group
        }
    }

    #[test]
    fn two_constraints_of_different_types_make_two_chunks_with_groups() {
        let high = Spec {
            kind: 4,
            ..spec(0, 1)
        };
        let low = Spec {
            flags: 1,
            ..spec(0, 1)
        };
        let mut b = build_scene(2, &[low, high]);
        b.run(0);
        let (e0, e1) = (b.entities[0], b.entities[1]);
        let c0 = b.constraints[0];
        // The entities get 0x80 times their number plus one.
        assert_eq!(b.e.mem.u32(e0 + 0xa4), 0x80);
        assert_eq!(b.e.mem.u32(e1 + 0xa4), 0x100);
        // The map: the allocator at +0x10 of the router, entity and index.
        assert_eq!(
            calls(&b.log, MAP_INSERT)
                .iter()
                .map(|call| call[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![b.heap_b, e0, 0], vec![b.heap_b, e1, 1]]
        );
        assert_eq!(calls(&b.log, MAP_CONSTRUCT).len(), 1);
        assert_eq!(calls(&b.log, MAP_CONSTRUCT)[0][2..], [0x400]);
        assert_eq!(calls(&b.log, MAP_CLEAR_AND_DEALLOCATE).len(), 1);
        assert_eq!(calls(&b.log, MAP_RELEASE).len(), 1);
        // Two chunks, linked, counted, with the running sums.
        assert_eq!(b.e.mem.u32(b.chunks + 4), 2);
        let (first, second) = (b.chunk(0), b.chunk(1));
        assert_eq!(b.e.mem.u32(first), second);
        assert_eq!(b.e.mem.u32(second), 0);
        for (chunk, before, after) in [(first, 0x100, 0x113), (second, 0x113, 0x126)] {
            assert_eq!(b.e.mem.u32(chunk + 4), b.island);
            assert_eq!(b.e.mem.u32(chunk + 8), ISLAND_BASE);
            assert_eq!(b.e.mem.u32(chunk + 0x14), 1);
            assert_eq!(b.e.mem.u32(chunk + 0xc), before);
            assert_eq!(b.e.mem.u32(chunk + 0x10), after);
        }
        // The entry of the first constraint (flags 1: it also gets a record).
        let entry = first + 0x18;
        assert_eq!(b.e.mem.u32(entry), 0x100 + 1);
        assert_eq!(b.e.mem.u32(entry + 4), 0x100 + 2);
        assert_eq!(b.e.mem.u32(entry + 8), 0x100 + 3);
        assert_eq!(b.e.mem.u32(entry + 0xc), e0 + 0xf0);
        assert_eq!(b.e.mem.u32(entry + 0x10), e1 + 0xf0);
        assert_eq!(b.e.mem.u16(entry + 0x14), 0x104);
        assert_eq!(b.e.mem.u16(entry + 0x16), 0x105);
        assert_eq!(b.e.mem.u16(entry + 0x18), 1);
        assert_eq!(b.e.mem.u16(entry + 0x1a), 2);
        assert_eq!(b.e.mem.u32(second + 0x18), 0x200 + 1);
        assert_eq!(b.e.mem.u32(b.island + 0x3c), 0x100);
        // The record on the stack allocator and its copy in the out structure.
        assert_eq!(b.e.mem.u32(b.top), c0);
        assert_eq!(b.e.mem.u32(b.top + 4), 0x0001_0000);
        assert_eq!(b.e.mem.u32(b.top + 8), entry);
        assert_eq!(b.e.mem.u32(b.chunks + 0xc), 1);
        let copies = b.e.mem.u32(b.chunks + 8);
        assert_eq!(b.e.mem.u32(copies), c0);
        assert_eq!(b.e.mem.u32(copies + 4), entry);
        assert_eq!(b.e.mem.u32(b.router), b.top);
        // Chunks, groups and the record copy come from the heap allocator at +0x10.
        let sizes: Vec<u32> = calls(&b.log, HEAP_B + 4).iter().map(|c| c[1]).collect();
        assert_eq!(sizes, vec![CHUNK_SIZE, 0x30, CHUNK_SIZE, 0x30, 8]);
        // The groups: one per chunk, the first pass series closed by the second.
        let (g0, g1) = (b.group(0), b.group(1));
        assert_eq!(b.e.mem.u32(g1), 0);
        assert_eq!(
            (b.e.mem.u8(g0 + 0x1c), b.e.mem.u8(g1 + 0x1c)),
            (1, 1),
            "both series are closed"
        );
        assert_eq!(b.e.mem.u16(g0 + 0x1e), 1);
        assert_eq!(b.e.mem.u32(g0 + 0x20), g1);
        assert_eq!(b.e.mem.u16(b.groups + 4), 1);
        for (group, before, after) in [
            (g0, (0x100, 0x200), (0x13, 0x10)),
            (g1, (0x113, 0x210), (0x13, 0x10)),
        ] {
            assert_eq!(b.e.mem.u32(group + 4), b.island);
            assert_eq!(b.e.mem.u32(group + 8), ISLAND_BASE);
            assert_eq!(b.e.mem.u32(group + 0xc), before.0);
            assert_eq!(b.e.mem.u32(group + 0x10), before.1);
            assert_eq!(b.e.mem.u32(group + 0x14), after.0);
            assert_eq!(b.e.mem.u32(group + 0x18), after.1);
        }
        // The island: the bytes that say not every constraint is flagged are clear, and the
        // block words follow the 16-bit word at +0x54.
        assert_eq!(b.e.mem.u8(b.island + 0x58), 0);
        assert_eq!(b.e.mem.u8(b.island + 0x59), 0);
        assert_eq!(b.e.mem.u32(b.island + 0x34), ISLAND_BASE + 0x300);
        assert_eq!(b.e.mem.u16(b.island + 0x38), 1);
        assert_eq!(b.e.mem.u16(b.island + 0x3a), 1);
        // Two arrays of 2 pairs each were reserved and are released at the end.
        let reserved = calls(&b.log, ARRAY_RESERVE);
        assert_eq!(reserved.len(), 2);
        assert_eq!(
            [reserved[0][0], reserved[0][2], reserved[0][3]],
            [b.heap_a, 2, 8]
        );
        assert_eq!(
            calls(&b.log, HEAP_A + 0x10)
                .iter()
                .map(|c| c[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![reserved[1][4], 16], vec![reserved[0][4], 16]]
        );
        // The table of one word per entity and the map storage come from the allocator at +0xC
        // and the table goes back to it.
        let from_a: Vec<u32> = calls(&b.log, HEAP_A + 4).iter().map(|c| c[1]).collect();
        assert_eq!(from_a, vec![0x400, 8]);
        assert_eq!(calls(&b.log, HEAP_A + 8).len(), 2);
        // The records: start, three numbers, end.
        let at = b.records;
        assert_eq!(b.e.mem.u32(at), LABEL_TT_BUILD_JAC_TASK);
        for (index, (label, value)) in [
            (LABEL_MI_NUM_ENTITIES, 2.0),
            (LABEL_MI_NUM_CONSTRAINTS, 2.0),
            (LABEL_MI_NUM_JACOBIANS, 85.0),
        ]
        .into_iter()
        .enumerate()
        {
            let record = at + 12 + 8 * index as u32;
            assert_eq!(b.e.mem.u32(record), label);
            assert_eq!(b.e.mem.f32(record + 4), value);
        }
        assert_eq!(b.e.mem.u32(at + 36), PROFILE_END);
        assert_eq!(b.e.mem.u32(b.stream + 4), at + 48);
    }

    #[test]
    fn without_the_flag_byte_on_every_constraint_no_groups_are_made() {
        let unflagged = Spec {
            flagged: false,
            ..spec(0, 1)
        };
        let mut b = build_scene(2, &[spec(1, 0), unflagged]);
        b.run(0);
        assert_eq!(b.e.mem.u8(b.island + 0x58), 1);
        assert_eq!(b.e.mem.u8(b.island + 0x59), 1);
        assert_eq!(b.e.mem.u32(b.groups), 0);
        // One chunk with both constraints: they share no entity with a different group.
        assert_eq!(b.e.mem.u32(b.chunks + 4), 1);
        assert_eq!(b.e.mem.u32(b.chunk(0) + 0x14), 2);
        assert_eq!(b.e.mem.u32(b.island + 0x34), 0);
        assert_eq!(b.e.mem.u32(b.chunks + 8), 0);
        assert_eq!(b.e.mem.u32(b.chunks + 0xc), 0);
        let sizes: Vec<u32> = calls(&b.log, HEAP_B + 4).iter().map(|c| c[1]).collect();
        assert_eq!(sizes, vec![CHUNK_SIZE]);
        // Entity 0 is walked first, so its constraint comes first: entity 0 joined to 1, then
        // entity 1 joined to 0.
        let entry = b.chunk(0) + 0x18;
        assert_eq!(b.e.mem.u16(entry + 0x18), 1);
        assert_eq!(b.e.mem.u16(entry + 0x1a), 2);
        assert_eq!(b.e.mem.u16(entry + 0x1c + 0x18), 2);
        assert_eq!(b.e.mem.u16(entry + 0x1c + 0x1a), 1);
    }

    #[test]
    fn the_side_byte_decides_which_index_is_the_entitys_own() {
        let one_sided = Spec {
            side: 1,
            ..spec(1, 0)
        };
        let mut b = build_scene(2, &[one_sided]);
        b.run(0);
        let entry = b.chunk(0) + 0x18;
        // Side 1: the own index (1) goes to the second word, the map lookup (0) to the first;
        // the entry adds one to each, and the near entity is at +8.
        assert_eq!(b.e.mem.u16(entry + 0x18), 1);
        assert_eq!(b.e.mem.u16(entry + 0x1a), 2);
        assert_eq!(b.e.mem.u32(entry + 0xc), b.entities[0] + 0xf0);
        assert_eq!(b.e.mem.u32(entry + 0x10), b.entities[1] + 0xf0);
    }

    /// 0x90 constraints from entity 0 to entity 1 and one more at the end (from `last_own` to
    /// `last_other`).
    fn crowded(last_own: usize, last_other: usize, flagged: bool) -> Build {
        let mut specs = vec![
            Spec {
                flagged,
                ..spec(0, 1)
            };
            0x90
        ];
        specs.push(Spec {
            flagged,
            ..spec(last_own, last_other)
        });
        build_scene(4, &specs)
    }

    #[test]
    fn a_full_chunk_starts_a_new_group_and_a_free_pair_goes_on_in_the_same_pass() {
        let mut b = crowded(2, 3, true);
        b.run(0);
        assert_eq!(b.e.mem.u32(b.chunks + 4), 2);
        assert_eq!(b.e.mem.u32(b.chunk(0) + 0x14), 0x90);
        assert_eq!(b.e.mem.u32(b.chunk(1) + 0x14), 1);
        // The sums: 0x90 constraints of 3 and 2 words.
        assert_eq!(b.e.mem.u32(b.chunk(1) + 0xc), 0x100 + 0x90 * 3 + 0x10);
        assert_eq!(b.e.mem.u32(b.chunk(1) + 0x10), 0x100 + 0x91 * 3 + 0x20);
        // Both groups belong to the same pass: one series of two groups.
        assert_eq!(b.e.mem.u16(b.groups + 4), 2);
        let (g0, g1) = (b.group(0), b.group(1));
        assert_eq!((b.e.mem.u8(g0 + 0x1c), b.e.mem.u8(g1 + 0x1c)), (0, 1));
        // The pending pairs live on the stack allocator: the 0x90 records plus the last one
        // follow the pending buffer.
        assert_eq!(b.e.mem.u32(b.router), b.top);
        // Records: none of the constraints has flags.
        assert_eq!(b.e.mem.u32(b.chunks + 0xc), 0);
    }

    #[test]
    fn a_pair_that_meets_another_group_waits_for_the_next_pass_and_the_flag_leaves_the_pass() {
        // The last constraint joins the same two entities again: after the full chunk the group
        // number is 2 and the entities belong to group 1, so it waits for the second pass.
        // With the byte flag set, the pass is left as soon as the group number is above 1.
        for (flag, last) in [(0, (0, 1)), (1, (2, 3))] {
            let mut b = crowded(last.0, last.1, true);
            b.run(flag);
            assert_eq!(b.e.mem.u32(b.chunks + 4), 2);
            assert_eq!(b.e.mem.u32(b.chunk(0) + 0x14), 0x90);
            assert_eq!(b.e.mem.u32(b.chunk(1) + 0x14), 1);
            let (g0, g1) = (b.group(0), b.group(1));
            // Two passes: the series are closed one after the other, each with one group.
            assert_eq!(b.e.mem.u16(b.groups + 4), 1);
            assert_eq!((b.e.mem.u8(g0 + 0x1c), b.e.mem.u8(g1 + 0x1c)), (1, 1));
            assert_eq!(b.e.mem.u16(g0 + 0x1e), 1);
            assert_eq!(b.e.mem.u32(g0 + 0x20), g1);
        }
    }

    #[test]
    fn without_groups_the_group_number_stays_and_a_full_chunk_does_not_split_the_pass() {
        let mut b = crowded(0, 1, false);
        b.run(0);
        assert_eq!(b.e.mem.u32(b.chunks + 4), 2);
        assert_eq!(b.e.mem.u32(b.chunk(0) + 0x14), 0x90);
        assert_eq!(b.e.mem.u32(b.chunk(1) + 0x14), 1);
        assert_eq!(b.e.mem.u32(b.groups), 0);
        // One pass only: the last pair is accepted into the pending list again.
        let reserved = calls(&b.log, ARRAY_RESERVE);
        assert_eq!(reserved[0][2], 0x91);
    }

    #[test]
    fn more_than_0x40_entities_double_the_hash_table() {
        let mut b = build_scene(0x41, &[spec(0, 1)]);
        b.run(0);
        assert_eq!(calls(&b.log, MAP_CONSTRUCT)[0][2..], [0x800]);
        assert_eq!(calls(&b.log, MAP_INSERT).len(), 0x41);
        let from_a: Vec<u32> = calls(&b.log, HEAP_A + 4).iter().map(|c| c[1]).collect();
        assert_eq!(from_a, vec![0x800, 0x104]);
        assert_eq!(b.e.mem.u32(b.island + 0x34), ISLAND_BASE + 0x300);
    }
}
