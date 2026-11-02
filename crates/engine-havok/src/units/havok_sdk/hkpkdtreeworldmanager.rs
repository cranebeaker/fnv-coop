//! `Havok SDK/hkpkdtreeworldmanager.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! What the code shows about the manager (`this`): `+0x10` holds the `hkKdTree*` it builds
//! (the functions take its address as a "tree slot"), `+0x1c` the world, `+0x20` a byte that
//! the update clears. The world's `+0x58` is the broad phase (slot `0x24` answers a count,
//! slot `0x40` fills two vectors with a bounding box), `+0x78` a word the distributed build
//! copies into its job, `+0x20`/`+0x24` the fixed island and its entity, `+0x28`/`+0x34` two
//! arrays of islands, `+0x110` an array of phantoms. An island has an entity array (`+0x48`
//! data, `+0x4c` count); an entity's collidable is at `+0x10`. An `hkArray` is
//! `{ data, size, capacityAndFlags }` (low 30 bits of the last word are the capacity, bit 31
//! means the array does not own its memory). Of the memory router (`00c85750`) `+0x10` is the
//! heap allocator (vtable slot `4` allocates, `8` frees `(pointer, size)`) and `+0xC` the
//! stack allocator (slot `0xC` takes the byte count by address and answers a block, slot
//! `0x10` frees `(pointer, size)`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::{ARRAY_RESERVE, MEMORY_ROUTER};
use crate::units::havok_sdk::hkpbvagent::{profile_start_record, LABEL_END};
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// `hkpCollidableMediator::hkpCollidableMediator` (`00d9df50`, `this`, `(collidables, world)`).
const MEDIATOR_CONSTRUCT: u32 = 0x00d9_df50;
/// `hkpCollidableMediator::getNumPrimitives` (`00d9e000`, `this`).
const MEDIATOR_NUM_PRIMITIVES: u32 = 0x00d9_e000;
/// `hkKdTreeCinfo::hkKdTreeCinfo` (`00d976d0`, `this`).
const CINFO_CONSTRUCT: u32 = 0x00d9_76d0;
/// `hkKdTree::~hkKdTree` (`00d97850`, `this`).
const TREE_DESTRUCT: u32 = 0x00d9_7850;
/// `hkKdTree::hkKdTree` (`00d97970`, `this`, `cinfo`); answers `this`.
const TREE_CONSTRUCT: u32 = 0x00d9_7970;
/// `hkKdTreeBuildingUtils::allocateBuffers` (`00d9d800`, cdecl `(buffers, count)`).
const ALLOCATE_BUFFERS: u32 = 0x00d9_d800;
/// `hkKdTreeBuildingUtils::deallocateBuffers` (`00d9d840`, cdecl `(buffers, count)`).
const DEALLOCATE_BUFFERS: u32 = 0x00d9_d840;
/// `hkKdTreeBuildingUtils::completeDistributedBuild` (`00d9d950`, cdecl `(tree, count, a, b)`).
const COMPLETE_DISTRIBUTED_BUILD: u32 = 0x00d9_d950;
/// `hkKdTreeBuildingUtils::projectPrimitives` (`00d9dab0`, cdecl
/// `(mediator, first, count, buffer)`).
const PROJECT_PRIMITIVES: u32 = 0x00d9_dab0;
/// `hkKdTreeBuildingUtils::buildTree_ov2` (`00d9db30`, cdecl `(buffers, tree)`).
const BUILD_TREE: u32 = 0x00d9_db30;
/// `hkSemaphore::hkSemaphore` (`00d04350`, `this`, `(initial, maximum)`); answers `this`.
const SEMAPHORE_CONSTRUCT: u32 = 0x00d0_4350;
/// `hkSemaphore::~hkSemaphore` (`00d04370`, `this`).
const SEMAPHORE_DESTRUCT: u32 = 0x00d0_4370;
/// `hkSemaphore::acquire` (`00d04380`, `this`).
const SEMAPHORE_ACQUIRE: u32 = 0x00d0_4380;
/// `hkJobQueue::addJob` (`00d04b50`, `this`, `(job, priority)`).
const JOB_QUEUE_ADD_JOB: u32 = 0x00d0_4b50;
/// `hkJobQueue::processAllJobs` (`00d051b0`, `this`).
const JOB_QUEUE_PROCESS_ALL: u32 = 0x00d0_51b0;

/// Profile labels (strings in the exe): `"TtbuildTreeSingleThreaded"`.
const LABEL_BUILD_SINGLE: u32 = 0x010c_d2a4;
/// `"LtbuildTreeDistributed"` (first label of the 16-byte start record).
const LABEL_BUILD_DISTRIBUTED: u32 = 0x010c_d104;
/// `"Stqueue"` (second label of that record).
const LABEL_QUEUE: u32 = 0x010c_d0fc;
/// `"Ststart"`.
const LABEL_START: u32 = 0x010c_d0f4;
/// `"StwaitForCompletion"`.
const LABEL_WAIT_FOR_COMPLETION: u32 = 0x010c_d0e0;
/// `"StwaitSemaphore"`.
const LABEL_WAIT_SEMAPHORE: u32 = 0x010c_d0d0;
/// `"StfinishPpu"`.
const LABEL_FINISH: u32 = 0x010c_d0c4;
/// `"LtkdTreeUpdate"` (first label of the 16-byte start record of `updateFromWorld`).
const LABEL_UPDATE: u32 = 0x010c_d2d8;
/// `"Stgather"` (second label of that record).
const LABEL_GATHER: u32 = 0x010c_d2cc;
/// `"Strebuild"`.
const LABEL_REBUILD: u32 = 0x010c_d2c0;

/// The capacity bits of an `hkArray`'s last word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// Capacity word of an array that does not own its memory and is empty.
const NOT_OWNED: u32 = 0x8000_0000;

/// A field of the memory router (`00c85750`): `0x10` is the heap allocator, `0xC` the stack
/// allocator.
fn router_field(e: &mut Engine, offset: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + offset)
}

/// Destroys and frees the 0x50-byte tree the slot held (the exe does nothing without one).
fn release_tree(e: &mut Engine, tree: u32) {
    if tree != 0 {
        e.call(TREE_DESTRUCT, &args![tree]);
        let heap = router_field(e, 0x10);
        e.vcall(heap, 8, &args![tree, 0x50u32]);
    }
}

/// Allocates and constructs a 0x50-byte tree from `cinfo`; 0 when the allocation fails.
fn new_tree(e: &mut Engine, cinfo: u32) -> u32 {
    let heap = router_field(e, 0x10);
    let block = e.vcall(heap, 4, &args![0x50u32]).u32();
    if block != 0 {
        e.call(TREE_CONSTRUCT, &args![block, cinfo]).u32()
    } else {
        0
    }
}

// Translated from 00d0a560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeWorldManager::buildTreeSingleThreaded` (Xbox PDB): builds the tree for the
/// `collidables` array in this thread. A mediator over the array and the world is made
/// (`00d9df50`); the tree the slot at `tree_slot` held is destroyed and freed; the mediator's
/// primitive count and the broad phase's bounding box (slot `0x40`, into the two vectors at
/// `+0x10` and `+0x20` of the tree's construction info) fill the info (count at `+0`, the
/// byte at `+5` cleared); the new tree is stored in the slot, the profile label
/// `TtbuildTreeSingleThreaded` is recorded, and the primitives are projected into the
/// buffers, the tree built from them and the buffers released, with the end record last.
pub fn build_tree_single_threaded(e: &mut Engine, this: Ptr, tree_slot: Ptr, collidables: Ptr) {
    let this = this.addr();
    let slot = tree_slot.addr();
    let collidables = collidables.addr();
    e.with_stack(0x70, |e, frame| {
        let base = frame.addr();
        let (buffers, mediator, cinfo) = (base + 0x1c, base + 0x24, base + 0x30);
        let world = e.mem.u32(this + 0x1c);
        e.call(MEDIATOR_CONSTRUCT, &args![mediator, collidables, world]);
        let old = e.mem.u32(slot);
        release_tree(e, old);
        let count = e.call(MEDIATOR_NUM_PRIMITIVES, &args![mediator]).u32();
        e.call(CINFO_CONSTRUCT, &args![cinfo]);
        let world = e.mem.u32(this + 0x1c);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(broad_phase, 0x40, &args![cinfo + 0x10, cinfo + 0x20]);
        e.mem.set_u32(cinfo, count);
        e.mem.set_u8(cinfo + 5, 0);
        let tree = new_tree(e, cinfo);
        e.mem.set_u32(slot, tree);
        profile_record(e, LABEL_BUILD_SINGLE);
        e.call(ALLOCATE_BUFFERS, &args![buffers, count]);
        let projected = e.mem.u32(buffers);
        e.call(PROJECT_PRIMITIVES, &args![mediator, 0u32, count, projected]);
        let tree = e.mem.u32(slot);
        e.call(BUILD_TREE, &args![buffers, tree]);
        e.call(DEALLOCATE_BUFFERS, &args![buffers, count]);
        profile_record(e, PROFILE_END);
    });
}

/// Appends `value` at the end of the `hkArray` at `array` without checking its capacity.
fn push_back(e: &mut Engine, array: u32, value: u32) {
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Appends the collidable (`+0x10`) of each entity of `island` except `skip`; the island's
/// entity count is read again for every entity.
fn push_island_collidables(e: &mut Engine, island: u32, array: u32, skip: Option<u32>) {
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(island + 0x4c) {
        let entities = e.mem.u32(island + 0x48);
        let entity = e.mem.u32(entities.wrapping_add(index.wrapping_mul(4)));
        if Some(entity) != skip {
            push_back(e, array, entity.wrapping_add(0x10));
        }
        index = index.wrapping_add(1);
    }
}

/// Appends the entity collidables of every island of the `hkArray` of islands at `islands`.
fn push_array_collidables(e: &mut Engine, islands: u32, array: u32) {
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(islands + 4) {
        let table = e.mem.u32(islands);
        let island = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        push_island_collidables(e, island, array, None);
        index = index.wrapping_add(1);
    }
}

// Translated from 00d0a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeWorldManager::gatherCollidablesFromWorld` (Xbox PDB): empties the `hkArray` of
/// pointers at `array`, makes room for the count the broad phase answers (slot `0x24`:
/// the larger of that count and twice the capacity, `00c908c0` with element size 4, when the
/// capacity is smaller), then appends the collidable (`entity + 0x10`) of every entity of
/// every island in the world's two island arrays (`+0x28`, `+0x34`), the phantoms'
/// collidable (`+0x10` of each item of the array at `+0x110`) when its first word is set, and
/// the entities of the fixed island (`+0x20`) other than the fixed entity (`+0x24`). Pushes do
/// not check the capacity.
pub fn gather_collidables_from_world(e: &mut Engine, this: Ptr, array: Ptr) {
    let this = this.addr();
    let array = array.addr();
    e.mem.set_u32(array + 4, 0);
    let world = e.mem.u32(this + 0x1c);
    let broad_phase = e.mem.u32(world + 0x58);
    let wanted = e.vcall(broad_phase, 0x24, &args![]).u32() as i32;
    let heap = router_field(e, 0x10);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let target = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![heap, array, target as u32, 4u32]);
    }
    let world = e.mem.u32(this + 0x1c);
    push_array_collidables(e, world + 0x28, array);
    let world = e.mem.u32(this + 0x1c);
    push_array_collidables(e, world + 0x34, array);
    let world = e.mem.u32(this + 0x1c);
    let phantoms = world + 0x110;
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(phantoms + 4) {
        let table = e.mem.u32(phantoms);
        let item = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        let collidable = item.wrapping_add(0x10);
        if e.mem.u32(collidable) != 0 {
            push_back(e, array, collidable);
        }
        index = index.wrapping_add(1);
    }
    let world = e.mem.u32(this + 0x1c);
    let island = e.mem.u32(world + 0x20);
    let fixed = e.mem.u32(world + 0x24);
    push_island_collidables(e, island, array, Some(fixed));
}

/// A block from the stack allocator: `(data, capacity word)`. The byte count is passed by
/// address and the granted count, divided by 4 (rounding toward zero), is the capacity; none
/// leaves the not-owned flag.
fn stack_block(e: &mut Engine, cell: u32, bytes: i32) -> (u32, u32) {
    let allocator = router_field(e, 0xc);
    e.mem.set_i32(cell, bytes);
    let data = e.vcall(allocator, 0xc, &args![cell]).u32();
    let granted = e.mem.i32(cell) / 4;
    (
        data,
        if granted == 0 {
            NOT_OWNED
        } else {
            granted as u32
        },
    )
}

/// Gives a stack block back when its capacity word says it owns its memory.
fn release_stack_block(e: &mut Engine, data: u32, capacity: u32) {
    let allocator = router_field(e, 0xc);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00d0a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the distributed build of the tree behind `tree_slot` for the
/// `collidables` array (`+0` data, `+4` count), run on the job queue `job_queue` and the
/// object `workers` (unconfirmed role: its slot `0xC` is called with `(queue, 3)` and slot
/// `0x10` waits). The old tree is destroyed and freed, the construction info is filled as in
/// `buildTreeSingleThreaded` (count at `+0`, the byte at `+5` set to 1, the bounding box from
/// the broad phase), the new tree stored and its buffers allocated. A 0x10-byte block
/// (first word 4) and two blocks from the stack allocator (16 bytes each, capacity = granted
/// bytes / 4) are taken, and a semaphore (`hkSemaphore(0, 1000)`) is made when the heap
/// gives 4 bytes. The 0x60-byte job is filled (bytes `6`, `3`, `1` at `+0`..`+2`, size
/// `0x60` at `+4`, `0xFFFF` at `+6`, the semaphore at `+0x10`, the block at `+0x14`, the
/// world's word at `+0x78` at `+0x1C`, the two buffer words at `+0x20`/`+0x24`, words of the
/// tree at `+0x28` (`+0`), `+0x2C` (`+0x1C`), `+0x38` (`+0x10`), `+0x3C` (`+0x14 + +0x18`),
/// `+0x40` (`+0x18`), the two stack blocks at `+0x30`/`+0x34`, the collidables' data at
/// `+0x50`; the exe leaves the other words as they were on its stack) and queued with
/// priority 0 between the profile records `LtbuildTreeDistributed`/`Stqueue` and `Ststart`;
/// the workers start (`(queue, 3)`), the queue processes all jobs, `StwaitForCompletion`,
/// the workers' slot `0x10`, `StwaitSemaphore`, the semaphore is acquired, `StfinishPpu`,
/// `completeDistributedBuild(tree, count, a, b)` runs with the job's two stack blocks, the end
/// record is made, and the buffers, the 0x10-byte block, the semaphore (destroyed first) and
/// the stack blocks (second first) are freed.
pub fn fn_00d0a810(
    e: &mut Engine,
    this: Ptr,
    tree_slot: Ptr,
    collidables: Ptr,
    job_queue: Ptr,
    workers: Ptr,
) {
    let this = this.addr();
    let slot = tree_slot.addr();
    let collidables = collidables.addr();
    let queue = job_queue.addr();
    let workers = workers.addr();
    e.with_stack(0xd0, |e, frame| {
        let base = frame.addr();
        let (buffers, cinfo, job) = (base + 0x14, base + 0x40, base + 0x70);
        let old = e.mem.u32(slot);
        release_tree(e, old);
        let count = e.mem.u32(collidables + 4);
        e.call(CINFO_CONSTRUCT, &args![cinfo]);
        let world = e.mem.u32(this + 0x1c);
        e.mem.set_u32(cinfo, count);
        e.mem.set_u8(cinfo + 5, 1);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(broad_phase, 0x40, &args![cinfo + 0x10, cinfo + 0x20]);
        let tree = new_tree(e, cinfo);
        e.mem.set_u32(slot, tree);
        e.call(ALLOCATE_BUFFERS, &args![buffers, count]);
        let heap = router_field(e, 0x10);
        let block = e.vcall(heap, 4, &args![0x10u32]).u32();
        e.mem.set_u32(block, 4);
        let (first_data, first_capacity) = stack_block(e, base + 0x2c, 0x10);
        let (second_data, second_capacity) = stack_block(e, base + 0x28, 0x10);
        let heap = router_field(e, 0x10);
        let semaphore_block = e.vcall(heap, 4, &args![4u32]).u32();
        let semaphore = if semaphore_block != 0 {
            e.call(SEMAPHORE_CONSTRUCT, &args![semaphore_block, 0u32, 0x3e8u32])
                .u32()
        } else {
            0
        };
        let world = e.mem.u32(this + 0x1c);
        e.mem.set_u8(job, 6);
        e.mem.set_u8(job + 1, 3);
        e.mem.set_u8(job + 2, 1);
        e.mem.set_u16(job + 4, 0x60);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x10, semaphore);
        e.mem.set_u32(job + 0x14, block);
        let world_word = e.mem.u32(world + 0x78);
        e.mem.set_u32(job + 0x1c, world_word);
        let buffer_words = (e.mem.u32(buffers), e.mem.u32(buffers + 4));
        e.mem.set_u32(job + 0x20, buffer_words.0);
        e.mem.set_u32(job + 0x24, buffer_words.1);
        e.mem.set_u32(job + 0x30, first_data);
        e.mem.set_u32(job + 0x34, second_data);
        let data = e.mem.u32(collidables);
        e.mem.set_u32(job + 0x50, data);
        let tree = e.mem.u32(slot);
        let tree_words = (
            e.mem.u32(tree),
            e.mem.u32(tree + 0x1c),
            e.mem.u32(tree + 0x10),
            e.mem.u32(tree + 0x14),
            e.mem.u32(tree + 0x18),
        );
        e.mem.set_u32(job + 0x28, tree_words.0);
        e.mem.set_u32(job + 0x2c, tree_words.1);
        e.mem.set_u32(job + 0x38, tree_words.2);
        e.mem
            .set_u32(job + 0x3c, tree_words.4.wrapping_add(tree_words.3));
        e.mem.set_u32(job + 0x40, tree_words.4);
        profile_start_record(e, LABEL_BUILD_DISTRIBUTED, LABEL_QUEUE);
        e.call(JOB_QUEUE_ADD_JOB, &args![queue, job, 0u32]);
        profile_record(e, LABEL_START);
        e.vcall(workers, 0xc, &args![queue, 3u32]);
        e.call(JOB_QUEUE_PROCESS_ALL, &args![queue]);
        profile_record(e, LABEL_WAIT_FOR_COMPLETION);
        e.vcall(workers, 0x10, &args![]);
        profile_record(e, LABEL_WAIT_SEMAPHORE);
        e.call(SEMAPHORE_ACQUIRE, &args![semaphore]);
        profile_record(e, LABEL_FINISH);
        let (first, second) = (e.mem.u32(job + 0x30), e.mem.u32(job + 0x34));
        let tree = e.mem.u32(slot);
        e.call(
            COMPLETE_DISTRIBUTED_BUILD,
            &args![tree, count, first, second],
        );
        profile_record(e, LABEL_END);
        e.call(DEALLOCATE_BUFFERS, &args![buffers, count]);
        let heap = router_field(e, 0x10);
        e.vcall(heap, 8, &args![block, 0x10u32]);
        if semaphore != 0 {
            e.call(SEMAPHORE_DESTRUCT, &args![semaphore]);
            let heap = router_field(e, 0x10);
            e.vcall(heap, 8, &args![semaphore, 4u32]);
        }
        release_stack_block(e, second_data, second_capacity);
        release_stack_block(e, first_data, first_capacity);
    });
}

// Translated from 00d0ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds the tree behind `tree_slot` for the `collidables` array
/// on the job queue and workers when both are given (`00d0a810`), otherwise in this thread
/// (`buildTreeSingleThreaded`).
pub fn fn_00d0ac40(
    e: &mut Engine,
    this: Ptr,
    tree_slot: Ptr,
    collidables: Ptr,
    job_queue: Ptr,
    workers: Ptr,
) {
    if job_queue.addr() != 0 && workers.addr() != 0 {
        fn_00d0a810(e, this, tree_slot, collidables, job_queue, workers);
    } else {
        build_tree_single_threaded(e, this, tree_slot, collidables);
    }
}

// Translated from 00d0ac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeWorldManager::updateFromWorld` (Xbox PDB): between the profile records
/// `LtkdTreeUpdate`/`Stgather`, `Strebuild` and the end record, gathers the world's
/// collidables into a local empty `hkArray` (`gatherCollidablesFromWorld`), rebuilds the tree
/// at `this + 0x10` from it with the two words given (`00d0ac40`: the job queue and the
/// workers), clears the byte at `this + 0x20` and frees the array's memory through the heap
/// allocator when it owns it.
pub fn update_from_world(e: &mut Engine, this: Ptr, job_queue: Ptr, workers: Ptr) {
    profile_start_record(e, LABEL_UPDATE, LABEL_GATHER);
    e.with_stack(0xc, |e, array| {
        e.mem.set_u32(array.addr() + 8, NOT_OWNED);
        gather_collidables_from_world(e, this, array);
        profile_record(e, LABEL_REBUILD);
        let slot = Ptr::new(this.addr() + 0x10);
        fn_00d0ac40(e, this, slot, array, job_queue, workers);
        e.mem.set_u8(this.addr() + 0x20, 0);
        profile_record(e, LABEL_END);
        let heap = router_field(e, 0x10);
        let capacity = e.mem.u32(array.addr() + 8);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array.addr());
            let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
            e.vcall(heap, 0x10, &args![data, bytes]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_a560, build_tree_single_threaded(Ptr, Ptr, Ptr)),
        entry!(0x00d0_a6a0, gather_collidables_from_world(Ptr, Ptr)),
        entry!(0x00d0_a810, fn_00d0a810(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d0_ac40, fn_00d0ac40(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d0_ac70, update_from_world(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Double bases: slot `s` of the broad phase, the heap allocator, the stack allocator and
    /// the workers object is the double at `base + s`.
    const BROAD_PHASE: u32 = 0x0900_0000;
    const HEAP: u32 = 0x0900_1000;
    const STACK: u32 = 0x0900_2000;
    const WORKERS: u32 = 0x0900_3000;
    /// The first word of every phantom collidable that counts.
    const SHAPE: u32 = 0x5555;

    /// Answers `blocks` in turn from the heap allocator's allocate slot (`HEAP + 4`), then 0.
    fn heap_allocator(e: &mut Engine, log: &Log, blocks: Vec<u32>) -> u32 {
        let object = logging_object(e, log, HEAP);
        let sink = Rc::clone(log);
        let mut next = blocks.into_iter();
        e.register_double(HEAP + 4, move |_, a| {
            sink.borrow_mut().push((HEAP + 4, a.to_vec()));
            Ret {
                eax: next.next().unwrap_or(0),
                ..Ret::default()
            }
        });
        object
    }

    /// Grants `(block, bytes)` in turn from the stack allocator's slot `0xC`.
    fn stack_allocator(e: &mut Engine, log: &Log, grants: Vec<(u32, i32)>) -> u32 {
        let object = logging_object(e, log, STACK);
        let sink = Rc::clone(log);
        let mut next = grants.into_iter();
        e.register_double(STACK + 0xc, move |e, a| {
            sink.borrow_mut().push((STACK + 0xc, a.to_vec()));
            let (block, bytes) = next.next().unwrap();
            e.mem.set_i32(a[1], bytes);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        object
    }

    fn install_router(e: &mut Engine, heap: u32, stack: u32) {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, stack);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
    }

    /// An object with logging doubles in the slots 0 to 0x44 at `BROAD_PHASE + slot`.
    fn broad_phase_object(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x48);
        for slot in (0..0x48).step_by(4) {
            log_double(e, log, BROAD_PHASE + slot, 0);
            e.mem.set_u32(vtable + slot, BROAD_PHASE + slot);
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    struct Manager {
        this: u32,
        world: u32,
        slot: u32,
        collidables: u32,
        heap: u32,
        stream: u32,
        records: u32,
    }

    /// A manager over a world with an empty fixed island and a broad phase (doubles at
    /// `BROAD_PHASE + slot`; slot `0x40` fills the two vectors), the heap answering `blocks`
    /// and the stack allocator granting `grants`; every function a build calls is a double
    /// (the number of primitives is 7, the tree and semaphore constructors answer `this`, the
    /// buffers allocation writes `0x5000` and `0x6000`); `old_tree` is the tree in the slot.
    /// The thread's profile stream has room for 0x100 bytes.
    fn manager(
        e: &mut Engine,
        log: &Log,
        old_tree: u32,
        blocks: Vec<u32>,
        grants: Vec<(u32, i32)>,
    ) -> Manager {
        let heap = heap_allocator(e, log, blocks);
        let stack = stack_allocator(e, log, grants);
        install_router(e, heap, stack);
        for address in [
            MEDIATOR_CONSTRUCT,
            CINFO_CONSTRUCT,
            TREE_DESTRUCT,
            PROJECT_PRIMITIVES,
            BUILD_TREE,
            DEALLOCATE_BUFFERS,
            COMPLETE_DISTRIBUTED_BUILD,
            SEMAPHORE_DESTRUCT,
            SEMAPHORE_ACQUIRE,
            JOB_QUEUE_PROCESS_ALL,
        ] {
            log_double(e, log, address, 0);
        }
        log_double(e, log, MEDIATOR_NUM_PRIMITIVES, 7);
        for address in [TREE_CONSTRUCT, SEMAPHORE_CONSTRUCT] {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let sink = Rc::clone(log);
        e.register_double(ALLOCATE_BUFFERS, move |e, a| {
            sink.borrow_mut().push((ALLOCATE_BUFFERS, a.to_vec()));
            e.mem.set_u32(a[0], 0x5000);
            e.mem.set_u32(a[0] + 4, 0x6000);
            Ret::default()
        });
        let broad_phase = broad_phase_object(e, log);
        let sink = Rc::clone(log);
        e.register_double(BROAD_PHASE + 0x40, move |e, a| {
            sink.borrow_mut().push((BROAD_PHASE + 0x40, a.to_vec()));
            store_vec(e, a[1], [1.0, 2.0, 3.0, 0.0]);
            store_vec(e, a[2], [4.0, 5.0, 6.0, 0.0]);
            Ret::default()
        });
        let (stream, records) = monitor(e, 0x100);
        let world = e.mem.alloc(0x200);
        e.mem.set_u32(world + 0x58, broad_phase);
        e.mem.set_u32(world + 0x78, 0x7878);
        let fixed = e.mem.alloc(0x60);
        e.mem.set_u32(world + 0x20, fixed);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x1c, world);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, old_tree);
        let collidables = e.mem.alloc(0x10);
        e.mem.set_u32(collidables, 0x1234_5000);
        e.mem.set_u32(collidables + 4, 5);
        Manager {
            this,
            world,
            slot,
            collidables,
            heap,
            stream,
            records,
        }
    }

    /// The addresses of the logged calls, in order.
    fn order(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(address, _)| *address).collect()
    }

    /// A 0x50-byte tree with the words the distributed job copies.
    fn tree(e: &mut Engine) -> u32 {
        let tree = e.mem.alloc(0x50);
        e.mem.set_u32(tree, 0x1111);
        e.mem.set_u32(tree + 0x10, 0x2222);
        e.mem.set_u32(tree + 0x14, 5);
        e.mem.set_u32(tree + 0x18, 7);
        e.mem.set_u32(tree + 0x1c, 0x3333);
        tree
    }

    #[test]
    fn build_tree_single_threaded_00d0a560_replaces_the_tree() {
        let mut e = engine();
        let log = new_log();
        let new_block = tree(&mut e);
        let m = manager(&mut e, &log, 0x4444, vec![new_block], Vec::new());
        e.call(0x00d0_a560, &args![m.this, m.slot, m.collidables]);
        assert_eq!(e.mem.u32(m.slot), new_block);
        assert_eq!(
            order(&log),
            vec![
                MEDIATOR_CONSTRUCT,
                TREE_DESTRUCT,
                HEAP + 8,
                MEDIATOR_NUM_PRIMITIVES,
                CINFO_CONSTRUCT,
                BROAD_PHASE + 0x40,
                HEAP + 4,
                TREE_CONSTRUCT,
                ALLOCATE_BUFFERS,
                PROJECT_PRIMITIVES,
                BUILD_TREE,
                DEALLOCATE_BUFFERS,
            ]
        );
        let mediator = calls(&log, MEDIATOR_NUM_PRIMITIVES)[0][0];
        let cinfo = calls(&log, CINFO_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&log, MEDIATOR_CONSTRUCT),
            vec![vec![mediator, m.collidables, m.world]]
        );
        assert_eq!(calls(&log, TREE_DESTRUCT), vec![vec![0x4444]]);
        assert_eq!(calls(&log, HEAP + 8), vec![vec![m.heap, 0x4444, 0x50]]);
        assert_eq!(calls(&log, HEAP + 4), vec![vec![m.heap, 0x50]]);
        assert_eq!(calls(&log, TREE_CONSTRUCT), vec![vec![new_block, cinfo]]);
        assert_eq!(
            calls(&log, BROAD_PHASE + 0x40)[0][1..],
            [cinfo + 0x10, cinfo + 0x20]
        );
        assert_eq!(e.mem.u32(cinfo), 7);
        assert_eq!(e.mem.u8(cinfo + 5), 0);
        assert_eq!(load_vec(&e, cinfo + 0x10), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, cinfo + 0x20), [4.0, 5.0, 6.0, 0.0]);
        let buffers = calls(&log, ALLOCATE_BUFFERS)[0][0];
        assert_eq!(calls(&log, ALLOCATE_BUFFERS), vec![vec![buffers, 7]]);
        assert_eq!(
            calls(&log, PROJECT_PRIMITIVES),
            vec![vec![mediator, 0, 7, 0x5000]]
        );
        assert_eq!(calls(&log, BUILD_TREE), vec![vec![buffers, new_block]]);
        assert_eq!(calls(&log, DEALLOCATE_BUFFERS), vec![vec![buffers, 7]]);
        assert_eq!(e.mem.u32(m.stream + 4), m.records + 24);
        assert_eq!(e.mem.u32(m.records), LABEL_BUILD_SINGLE);
        assert_eq!(e.mem.u32(m.records + 12), PROFILE_END);
    }

    #[test]
    fn build_tree_single_threaded_00d0a560_keeps_going_without_memory_or_old_tree() {
        let mut e = engine();
        let log = new_log();
        let m = manager(&mut e, &log, 0, Vec::new(), Vec::new());
        e.call(0x00d0_a560, &args![m.this, m.slot, m.collidables]);
        assert_eq!(e.mem.u32(m.slot), 0);
        assert!(calls(&log, TREE_DESTRUCT).is_empty());
        assert!(calls(&log, TREE_CONSTRUCT).is_empty());
        let buffers = calls(&log, ALLOCATE_BUFFERS)[0][0];
        assert_eq!(calls(&log, BUILD_TREE), vec![vec![buffers, 0]]);
    }

    /// Entities, islands, phantoms and a fixed island in a manager's world; returns the
    /// collidables the gather must produce, in order.
    fn populate_world(e: &mut Engine, world: u32) -> Vec<u32> {
        let list_of = |e: &mut Engine, items: &[u32]| {
            let list = e.mem.alloc(0x20);
            for (n, item) in items.iter().enumerate() {
                e.mem.set_u32(list + 4 * n as u32, *item);
            }
            list
        };
        let island = |e: &mut Engine, entities: &[u32]| {
            let list = list_of(e, entities);
            let island = e.mem.alloc(0x60);
            e.mem.set_u32(island + 0x48, list);
            e.mem.set_u32(island + 0x4c, entities.len() as u32);
            island
        };
        let array_at = |e: &mut Engine, at: u32, items: &[u32]| {
            let list = list_of(e, items);
            e.mem.set_u32(at, list);
            e.mem.set_u32(at + 4, items.len() as u32);
        };
        let entities: Vec<u32> = (0..7).map(|_| e.mem.alloc(0x30)).collect();
        let first = island(e, &entities[0..2]);
        let second = island(e, &entities[2..3]);
        let third = island(e, &entities[3..4]);
        array_at(e, world + 0x28, &[first, second]);
        array_at(e, world + 0x34, &[third]);
        let counted = e.mem.alloc(0x20);
        e.mem.set_u32(counted + 0x10, SHAPE);
        let ignored = e.mem.alloc(0x20);
        array_at(e, world + 0x110, &[counted, ignored]);
        let fixed = island(e, &entities[4..7]);
        e.mem.set_u32(world + 0x20, fixed);
        e.mem.set_u32(world + 0x24, entities[5]);
        vec![
            entities[0] + 0x10,
            entities[1] + 0x10,
            entities[2] + 0x10,
            entities[3] + 0x10,
            counted + 0x10,
            entities[4] + 0x10,
            entities[6] + 0x10,
        ]
    }

    /// An `hkArray` with room for pointers whose size is 9 and capacity word `capacity`.
    fn pointer_array(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 9);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    #[test]
    fn gather_collidables_from_world_00d0a6a0_walks_islands_phantoms_and_the_fixed_island() {
        let mut e = engine();
        let log = new_log();
        let m = manager(&mut e, &log, 0, Vec::new(), Vec::new());
        log_double(&mut e, &log, BROAD_PHASE + 0x24, 4);
        log_double(&mut e, &log, ARRAY_RESERVE, 0);
        let expected = populate_world(&mut e, m.world);
        let array = pointer_array(&mut e, 0x10);
        e.call(0x00d0_a6a0, &args![m.this, array]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 7);
        let got: Vec<u32> = (0..7).map(|n| e.mem.u32(data + 4 * n)).collect();
        assert_eq!(got, expected);
        assert!(calls(&log, ARRAY_RESERVE).is_empty());
    }

    #[test]
    fn gather_collidables_from_world_00d0a6a0_reserves_the_larger_of_count_and_double() {
        for (capacity, count, wanted) in [(2u32, 3u32, 4u32), (2, 9, 9), (0x4000_0003, 5, 6)] {
            let mut e = engine();
            let log = new_log();
            let m = manager(&mut e, &log, 0, Vec::new(), Vec::new());
            log_double(&mut e, &log, BROAD_PHASE + 0x24, count);
            log_double(&mut e, &log, ARRAY_RESERVE, 0);
            let array = pointer_array(&mut e, capacity);
            e.call(0x00d0_a6a0, &args![m.this, array]);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(
                calls(&log, ARRAY_RESERVE),
                vec![vec![m.heap, array, wanted, 4]],
                "capacity {capacity:#x}, count {count}"
            );
        }
    }

    #[test]
    fn gather_collidables_from_world_00d0a6a0_does_not_reserve_when_there_is_room() {
        let mut e = engine();
        let log = new_log();
        let m = manager(&mut e, &log, 0, Vec::new(), Vec::new());
        log_double(&mut e, &log, BROAD_PHASE + 0x24, 9);
        log_double(&mut e, &log, ARRAY_RESERVE, 0);
        let array = pointer_array(&mut e, 0xc000_0009);
        e.call(0x00d0_a6a0, &args![m.this, array]);
        assert!(calls(&log, ARRAY_RESERVE).is_empty());
    }

    /// A manager for the distributed build: the heap answers a tree block, a job block and a
    /// semaphore block (`semaphore`, 0 for none); the stack allocator grants 0x20 then 0
    /// bytes; the queue call takes a snapshot of the job. Returns the workers object too.
    fn distributed(
        e: &mut Engine,
        log: &Log,
        blocks: [u32; 3],
    ) -> (Manager, u32, Rc<RefCell<Vec<u8>>>) {
        let m = manager(
            e,
            log,
            0,
            blocks.to_vec(),
            vec![(0xa000, 0x20), (0xb000, 0)],
        );
        let snapshot = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&snapshot);
        let queue_log = Rc::clone(log);
        e.register_double(JOB_QUEUE_ADD_JOB, move |e, a| {
            queue_log.borrow_mut().push((JOB_QUEUE_ADD_JOB, a.to_vec()));
            *sink.borrow_mut() = e.mem.bytes(a[1], 0x60);
            Ret::default()
        });
        let workers = logging_object(e, log, WORKERS);
        (m, workers, snapshot)
    }

    fn word(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ])
    }

    #[test]
    fn fn_00d0a810_runs_a_job_on_the_queue_and_waits() {
        let mut e = engine();
        let log = new_log();
        let new_block = tree(&mut e);
        let job_block = e.mem.alloc(0x10);
        let semaphore = e.mem.alloc(4);
        let (m, workers, snapshot) = distributed(&mut e, &log, [new_block, job_block, semaphore]);
        e.call(
            0x00d0_a810,
            &args![m.this, m.slot, m.collidables, 0x7100u32, workers],
        );
        assert_eq!(e.mem.u32(m.slot), new_block);
        assert_eq!(
            order(&log),
            vec![
                CINFO_CONSTRUCT,
                BROAD_PHASE + 0x40,
                HEAP + 4,
                TREE_CONSTRUCT,
                ALLOCATE_BUFFERS,
                HEAP + 4,
                STACK + 0xc,
                STACK + 0xc,
                HEAP + 4,
                SEMAPHORE_CONSTRUCT,
                JOB_QUEUE_ADD_JOB,
                WORKERS + 0xc,
                JOB_QUEUE_PROCESS_ALL,
                WORKERS + 0x10,
                SEMAPHORE_ACQUIRE,
                COMPLETE_DISTRIBUTED_BUILD,
                DEALLOCATE_BUFFERS,
                HEAP + 8,
                SEMAPHORE_DESTRUCT,
                HEAP + 8,
                STACK + 0x10,
            ]
        );
        let cinfo = calls(&log, CINFO_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(cinfo), 5);
        assert_eq!(e.mem.u8(cinfo + 5), 1);
        assert_eq!(load_vec(&e, cinfo + 0x10), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(e.mem.u32(job_block), 4);
        assert_eq!(
            calls(&log, HEAP + 4),
            vec![vec![m.heap, 0x50], vec![m.heap, 0x10], vec![m.heap, 4]]
        );
        assert_eq!(
            calls(&log, SEMAPHORE_CONSTRUCT),
            vec![vec![semaphore, 0, 0x3e8]]
        );
        let queue_call = &calls(&log, JOB_QUEUE_ADD_JOB)[0];
        assert_eq!(queue_call[0], 0x7100);
        assert_eq!(queue_call[2], 0);
        let job = snapshot.borrow();
        assert_eq!(&job[0..3], &[6, 3, 1]);
        assert_eq!(u16::from_le_bytes([job[4], job[5]]), 0x60);
        assert_eq!(u16::from_le_bytes([job[6], job[7]]), 0xffff);
        let expected = [
            (0x10, semaphore),
            (0x14, job_block),
            (0x1c, 0x7878),
            (0x20, 0x5000),
            (0x24, 0x6000),
            (0x28, 0x1111),
            (0x2c, 0x3333),
            (0x30, 0xa000),
            (0x34, 0xb000),
            (0x38, 0x2222),
            (0x3c, 12),
            (0x40, 7),
            (0x50, 0x1234_5000),
        ];
        for (offset, value) in expected {
            assert_eq!(word(&job, offset), value, "job word {offset:#x}");
        }
        assert_eq!(calls(&log, WORKERS + 0xc)[0][1..], [0x7100, 3]);
        assert_eq!(calls(&log, SEMAPHORE_ACQUIRE), vec![vec![semaphore]]);
        assert_eq!(
            calls(&log, COMPLETE_DISTRIBUTED_BUILD),
            vec![vec![new_block, 5, 0xa000, 0xb000]]
        );
        let buffers = calls(&log, ALLOCATE_BUFFERS)[0][0];
        assert_eq!(calls(&log, DEALLOCATE_BUFFERS), vec![vec![buffers, 5]]);
        assert_eq!(
            calls(&log, HEAP + 8),
            vec![vec![m.heap, job_block, 0x10], vec![m.heap, semaphore, 4]]
        );
        assert_eq!(calls(&log, SEMAPHORE_DESTRUCT), vec![vec![semaphore]]);
        // The first stack block owns 8 words; the second was granted nothing.
        assert_eq!(calls(&log, STACK + 0x10)[0][1..], [0xa000, 0x20]);
        assert_eq!(e.mem.u32(m.records), LABEL_BUILD_DISTRIBUTED);
        assert_eq!(e.mem.u32(m.records + 0xc), LABEL_QUEUE);
        let labels: Vec<u32> = (0..5)
            .map(|n| e.mem.u32(m.records + 0x10 + 12 * n))
            .collect();
        assert_eq!(
            labels,
            vec![
                LABEL_START,
                LABEL_WAIT_FOR_COMPLETION,
                LABEL_WAIT_SEMAPHORE,
                LABEL_FINISH,
                LABEL_END
            ]
        );
        assert_eq!(e.mem.u32(m.stream + 4), m.records + 0x10 + 60);
    }

    #[test]
    fn fn_00d0a810_without_a_semaphore_block_waits_on_nothing() {
        let mut e = engine();
        let log = new_log();
        let new_block = tree(&mut e);
        let job_block = e.mem.alloc(0x10);
        let (m, workers, snapshot) = distributed(&mut e, &log, [new_block, job_block, 0]);
        e.mem.set_u32(m.slot, 0x4444);
        e.call(
            0x00d0_a810,
            &args![m.this, m.slot, m.collidables, 0x7100u32, workers],
        );
        assert!(calls(&log, SEMAPHORE_CONSTRUCT).is_empty());
        assert!(calls(&log, SEMAPHORE_DESTRUCT).is_empty());
        assert_eq!(calls(&log, SEMAPHORE_ACQUIRE), vec![vec![0]]);
        assert_eq!(word(&snapshot.borrow(), 0x10), 0);
        assert_eq!(calls(&log, TREE_DESTRUCT), vec![vec![0x4444]]);
        // The old tree and the job block.
        assert_eq!(calls(&log, HEAP + 8).len(), 2);
    }

    #[test]
    fn fn_00d0ac40_picks_the_distributed_build_only_with_both_words() {
        for (queue, with_workers, distributed_build) in [
            (0x7100u32, true, true),
            (0, true, false),
            (0x7100, false, false),
        ] {
            let mut e = engine();
            let log = new_log();
            let new_block = tree(&mut e);
            let job_block = e.mem.alloc(0x10);
            let semaphore = e.mem.alloc(4);
            let (m, object, _) = distributed(&mut e, &log, [new_block, job_block, semaphore]);
            let workers = if with_workers { object } else { 0 };
            e.call(
                0x00d0_ac40,
                &args![m.this, m.slot, m.collidables, queue, workers],
            );
            assert_eq!(
                !calls(&log, JOB_QUEUE_ADD_JOB).is_empty(),
                distributed_build,
                "queue {queue:#x}, workers {workers:#x}"
            );
            assert_eq!(
                !calls(&log, PROJECT_PRIMITIVES).is_empty(),
                !distributed_build
            );
        }
    }

    #[test]
    fn update_from_world_00d0ac70_gathers_rebuilds_and_frees_the_array() {
        let mut e = engine();
        let log = new_log();
        let new_block = tree(&mut e);
        let m = manager(&mut e, &log, 0, vec![new_block], Vec::new());
        log_double(&mut e, &log, BROAD_PHASE + 0x24, 3);
        // The reserve gives the local array 3 owned words.
        let data = Rc::new(RefCell::new(0u32));
        let sink = Rc::clone(&log);
        let kept = Rc::clone(&data);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let block = e.mem.alloc(0x10);
            *kept.borrow_mut() = block;
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.mem.set_u8(m.this + 0x20, 1);
        e.call(0x00d0_ac70, &args![m.this, 0u32, 0u32]);
        assert_eq!(e.mem.u8(m.this + 0x20), 0);
        let reserve = &calls(&log, ARRAY_RESERVE)[0];
        let array = reserve[1];
        assert_eq!(reserve[2..], [3, 4]);
        // The rebuild got the tree slot inside the manager and the local array.
        assert_eq!(e.mem.u32(m.this + 0x10), new_block);
        assert_eq!(calls(&log, MEDIATOR_CONSTRUCT)[0][1..], [array, m.world]);
        assert_eq!(
            calls(&log, HEAP + 0x10),
            vec![vec![m.heap, *data.borrow(), 12]]
        );
        assert_eq!(e.mem.u32(m.records), LABEL_UPDATE);
        assert_eq!(e.mem.u32(m.records + 0xc), LABEL_GATHER);
        let labels: Vec<u32> = (0..4)
            .map(|n| e.mem.u32(m.records + 0x10 + 12 * n))
            .collect();
        assert_eq!(
            labels,
            vec![LABEL_REBUILD, LABEL_BUILD_SINGLE, PROFILE_END, LABEL_END]
        );
        assert_eq!(e.mem.u32(m.stream + 4), m.records + 0x10 + 48);
    }

    #[test]
    fn update_from_world_00d0ac70_leaves_an_array_it_does_not_own() {
        let mut e = engine();
        let log = new_log();
        let m = manager(&mut e, &log, 0, Vec::new(), Vec::new());
        log_double(&mut e, &log, BROAD_PHASE + 0x24, 0);
        e.call(0x00d0_ac70, &args![m.this, 0u32, 0u32]);
        assert!(calls(&log, HEAP + 0x10).is_empty());
        assert_eq!(calls(&log, MEDIATOR_CONSTRUCT).len(), 1);
    }
}
