//! `Havok SDK/run_00cf7540` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here belong to the world's pending-operation queue (the Xbox PDB types the
//! `hkpWorld` member `m_pendingOperations`, `+0x88`, as `hkpWorldOperationQueue*`; the class
//! itself is not in the PDB, so the field names below are descriptions from the code and the
//! class name is unconfirmed). Layout as read from the code:
//!
//! - `+0x00` an array of 0x14-byte operation records `(kind byte, then four words)`
//!   (`data`, `size +4`, `capacityAndFlags +8`; the low 30 bits of the last are the capacity
//!   and bit 31 means the array does not own its memory), `+0x0C` the world,
//! - `+0x10` an array of 0x14-byte island-merge requests (`size +0x14`, capacity `+0x18`),
//! - `+0x1C` an array of 16-byte `(object, listener)` pairs (`size +0x20`, capacity `+0x24`).
//!
//! The world keeps counters the queue maintains: `+0x8C` pending operations, `+0x90` pending
//! body operations and `+0xA4` the nesting count of `00cf7c10`. Havok objects are reference
//! counted under the recursive lock of `hkreferencedobject.cpp` (`00c90490` / `00c904e0`).

#[allow(unused_imports)]
use crate::prelude::*;

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// Memory router (`00c85750`, no arguments): its `+0x10` is the allocator; the allocator's
/// vtable slots are `+4` allocate `(size)`, `+8` free `(block, size)` and `+0x10` free an
/// array `(block, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90490` / `00c904e0`: take and release the recursive reference-count lock.
const LOCK_ENTER: u32 = 0x00c9_0490;
const LOCK_LEAVE: u32 = 0x00c9_04e0;
/// `00c90510` / `00c905b0`: locked add and remove of a reference (`this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkReferencedObject::removeReferenceLockUnchecked` (Xbox PDB, `this` only).
const REMOVE_REFERENCE_UNCHECKED: u32 = 0x00c9_03b0;
/// `hkString::memCpy` (Xbox PDB, cdecl `(destination, source, bytes)`).
const MEM_COPY: u32 = 0x00cb_1260;
/// `00c90940` (cdecl `(allocator, array, element size)`): grows an `hkArray` by one chunk.
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reserve` (Xbox PDB, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `00d9ec10` (`this`, `other`): swaps the three words (data, size, capacity) of two arrays.
const ARRAY_SWAP: u32 = 0x00d9_ec10;
/// `00c90740` (cdecl `(array, count, 4)`).
const RELEASE_BATCH: u32 = 0x00c9_0740;
/// The comparison functions the sorts are given (no function is defined at these addresses
/// in the disassembly; they are called by address): `00cf74b0` orders the `(object,
/// listener)` pairs, `00cf74e0` the island-merge requests.
const COMPARE_PAIRS: u32 = 0x00cf_74b0;
const COMPARE_MERGES: u32 = 0x00cf_74e0;
/// `hkpWorldOperationUtil::mergeIslands` (Xbox PDB, cdecl `(world, entityA, entityB)`).
const MERGE_ISLANDS: u32 = 0x00cb_5570;
/// `hkpWorldOperationUtil::updateEntityBP` (Xbox PDB, cdecl `(world, entity)`).
const UPDATE_ENTITY_BP: u32 = 0x00cb_5750;
/// `hkpRigidBody::updateBroadphaseAndResetCollisionInformationOfWarpedBody` (Xbox PDB, cdecl
/// `(body)`).
const UPDATE_WARPED_BODY: u32 = 0x00ca_8700;
/// The float constant (0.0) the vector operations use as the fourth lane.
const VECTOR_FOURTH_LANE: u32 = 0x0101_1d78;

/// The `hkArray` capacity mask: the low 30 bits are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// `capacityAndFlags` of an empty array that owns no memory.
const NO_OWNED_MEMORY: u32 = 0x8000_0000;

/// The allocator of the thread's memory router.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Appends nothing, but makes room: `hkArray::pushBack`'s growth step for 0x14-byte records
/// (`size == capacity` calls `00c90940`).
fn grow_if_full(e: &mut Engine, allocator: u32, array: u32) {
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_MASK {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x14u32]);
    }
}

// Translated from 00cf7690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the operation record `op` (five words): appends it to the queue's array (under the
/// reference-count lock), bumps the world's pending-operation count (`+0x8C`) and takes the
/// references its kind needs (kind byte `+0`; the words are `a` = `+4`, `b` = `+8`):
/// kinds 1-4, 8-11, 13, 14, 18, 20, 22, 24-29, 33, 34 and 36 add a reference to `a`; 5, 19
/// and 35 to `a` and `b`; 6, 7, 15, 16 and 23 copy the `b & 0xFFFF` pointers at `a` into a
/// new block and add a reference to each; 12 adds to both, moves the record to the second
/// array and takes it back out of the first; 17 adds to `a` and stores a 0x20-byte copy of
/// the block at `b` in the record; 30 adds to `a` and drops the record again; 31 removes a
/// reference from `a` and drops the record; 21, 32 and the unknown kinds take no reference.
pub fn fn_00cf7690(e: &mut Engine, this: Ptr, op: Ptr) {
    let queue = this.addr();
    e.call(LOCK_ENTER, &args![]);
    let alloc = allocator(e);
    grow_if_full(e, alloc, queue);
    let slot = e
        .mem
        .u32(queue)
        .wrapping_add(e.mem.u32(queue + 4).wrapping_mul(0x14));
    if slot != 0 {
        e.mem.set_u8(slot, 0);
    }
    let size = e.mem.u32(queue + 4);
    let data = e.mem.u32(queue);
    e.mem.set_u32(queue + 4, size.wrapping_add(1));
    let world = e.mem.u32(queue + 0xc);
    let pending = e.mem.u32(world + 0x8c);
    e.mem.set_u32(world + 0x8c, pending.wrapping_add(1));
    let record = data.wrapping_add(size.wrapping_mul(0x14));
    for word in 0..5 {
        let value = e.mem.u32(op.addr() + 4 * word);
        e.mem.set_u32(record + 4 * word, value);
    }
    let a = e.mem.u32(record + 4);
    let b = e.mem.u32(record + 8);
    match e.mem.u8(record) {
        1..=4 | 8..=11 | 13 | 14 | 18 | 20 | 22 | 24..=29 | 33 | 34 | 36 => {
            e.call(ADD_REFERENCE, &args![a]);
        }
        5 | 19 | 35 => {
            e.call(ADD_REFERENCE, &args![a]);
            e.call(ADD_REFERENCE, &args![b]);
        }
        6 | 7 | 15 | 16 | 23 => {
            let count = u32::from(e.mem.u16(op.addr() + 8));
            let alloc = allocator(e);
            let buffer = e.vcall(alloc, 4, &args![count * 4]).u32();
            e.mem.set_u32(record + 4, buffer);
            let source = e.mem.u32(op.addr() + 4);
            e.call(MEM_COPY, &args![buffer, source, count * 4]);
            let count = u32::from(e.mem.u16(record + 8));
            let end = buffer.wrapping_add(count * 4);
            let mut cursor = buffer;
            while cursor < end {
                let object = e.mem.u32(cursor);
                cursor += 4;
                e.call(ADD_REFERENCE, &args![object]);
            }
        }
        12 => {
            e.call(ADD_REFERENCE, &args![a]);
            e.call(ADD_REFERENCE, &args![b]);
            let second = queue + 0x10;
            let alloc = allocator(e);
            grow_if_full(e, alloc, second);
            let target = e
                .mem
                .u32(second)
                .wrapping_add(e.mem.u32(second + 4).wrapping_mul(0x14));
            if target != 0 {
                for word in 0..5 {
                    let value = e.mem.u32(record + 4 * word);
                    e.mem.set_u32(target + 4 * word, value);
                }
            }
            let second_size = e.mem.u32(second + 4);
            e.mem.set_u32(second + 4, second_size.wrapping_add(1));
            let first_size = e.mem.u32(queue + 4);
            e.mem.set_u32(queue + 4, first_size.wrapping_sub(1));
        }
        17 => {
            e.call(ADD_REFERENCE, &args![a]);
            let alloc = allocator(e);
            let copy = e.vcall(alloc, 4, &args![0x20u32]).u32();
            if copy != 0 {
                let source = e.mem.u32(op.addr() + 8);
                for word in 0..8 {
                    let value = e.mem.u32(source + 4 * word);
                    e.mem.set_u32(copy + 4 * word, value);
                }
                e.mem.set_u32(record + 8, copy);
            } else {
                e.mem.set_u32(record + 8, 0);
            }
        }
        30 | 31 => {
            if e.mem.u8(record) == 30 {
                e.call(ADD_REFERENCE, &args![a]);
            } else {
                e.call(REMOVE_REFERENCE, &args![a]);
            }
            let size = e.mem.u32(queue + 4);
            e.mem.set_u32(queue + 4, size.wrapping_sub(1));
            let pending = e.mem.u32(world + 0x8c);
            e.mem.set_u32(world + 0x8c, pending.wrapping_sub(1));
        }
        _ => {}
    }
    e.call(LOCK_LEAVE, &args![]);
}

// Translated from 00cf7ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the queue's `(object, listener)` pairs (array at `+0x1C`, size `+0x20`): while it
/// is not empty, takes its contents into a local array (swap), clears the world's pending
/// body count (`+0x90`), makes sure the emptied array has room for 16 pairs, sorts the pairs
/// (`00cf74b0` through `00cf83c0`), calls each pair's listener (its slot `+0xC` with the
/// object and a zero byte), drops the two references of every pair under the lock
/// (`removeReferenceLockUnchecked`) and frees the local array's memory when it owns it.
pub fn fn_00cf7ae0(e: &mut Engine, this: Ptr) {
    // The exe's frame (`SUB ESP,0x14`, four pushes): 0x24 bytes, the swapped-out array at +0x18
    // (00cf7afc..00cf7b04), the listener's answer byte at +0x13 (00cf7b82).
    e.with_stack(0x24, |e, frame| {
        let frame = frame.addr();
        let queue = this.addr();
        while e.mem.u32(queue + 0x20) != 0 {
            let local = frame + 0x18;
            e.mem.set_u32(local, 0);
            e.mem.set_u32(local + 4, 0);
            e.mem.set_u32(local + 8, NO_OWNED_MEMORY);
            e.call(ARRAY_SWAP, &args![local, queue + 0x1c]);
            let world = e.mem.u32(queue + 0xc);
            e.mem.set_u32(world + 0x90, 0);
            let alloc = allocator(e);
            let capacity = e.mem.u32(queue + 0x24) & CAPACITY_MASK;
            if (capacity as i32) < 0x10 {
                let doubled = (capacity as i32).wrapping_add(capacity as i32);
                let wanted = if doubled > 0x10 { doubled } else { 0x10 };
                e.call(
                    ARRAY_RESERVE,
                    &args![alloc, queue + 0x1c, wanted as u32, 0x10u32],
                );
            }
            let count = e.mem.u32(local + 4) as i32;
            if count > 1 {
                let data = e.mem.u32(local);
                fn_00cf83c0(e, data, 0, count - 1, COMPARE_PAIRS);
            }
            let flag = frame + 0x13;
            for index in 0..count.max(0) as u32 {
                let item = e.mem.u32(local) + index * 16;
                e.mem.set_u8(flag, 0);
                let first = e.mem.u32(item);
                let listener = e.mem.u32(item + 4);
                e.vcall(listener, 0xc, &args![first, flag]);
            }
            e.call(LOCK_ENTER, &args![]);
            for index in 0..count.max(0) as u32 {
                let item = e.mem.u32(local) + index * 16;
                let first = e.mem.u32(item);
                let listener = e.mem.u32(item + 4);
                e.call(REMOVE_REFERENCE_UNCHECKED, &args![first]);
                e.call(REMOVE_REFERENCE_UNCHECKED, &args![listener]);
            }
            e.call(LOCK_LEAVE, &args![]);
            let alloc = allocator(e);
            e.mem.set_u32(local + 4, 0);
            let capacity_flags = e.mem.u32(local + 8);
            if (capacity_flags as i32) >= 0 {
                let bytes = (capacity_flags & CAPACITY_MASK) << 4;
                let data = e.mem.u32(local);
                e.vcall(alloc, 0x10, &args![data, bytes]);
            }
        }
    })
}

// Translated from 00cf83c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Quicksort of the 16-byte elements `base[lo..=hi]` by the comparison function `compare`
/// (cdecl `(a, b)`, a byte result: nonzero when `a` goes before `b`). The pivot is the
/// middle element's copy; the scans and swaps follow Hoare's scheme, recursing into the left
/// part and iterating on the right.
pub fn fn_00cf83c0(e: &mut Engine, base: u32, lo: i32, hi: i32, compare: u32) {
    // The exe aligns ESP, then `SUB ESP,0x24` and three pushes: a 0x30-byte frame, the pivot
    // element copied to +0x10 (00cf83eb..00cf8406).
    with_aligned_frame(e, 0x30, |e, frame| {
        let mut lo = lo;
        let pivot = frame + 0x10;
        loop {
            let middle = (lo.wrapping_add(hi)) >> 1;
            let source = base.wrapping_add((middle as u32) << 4);
            let bytes = e.mem.bytes(source, 16);
            e.mem.write(pivot, &bytes);
            let mut i = lo;
            let mut j = hi;
            loop {
                let mut left = base.wrapping_add((i as u32) << 4);
                while e.call(compare, &args![left, pivot]).u8() != 0 {
                    left = left.wrapping_add(0x10);
                    i = i.wrapping_add(1);
                }
                let mut right = base.wrapping_add((j as u32) << 4);
                while e.call(compare, &args![pivot, right]).u8() != 0 {
                    right = right.wrapping_sub(0x10);
                    j = j.wrapping_sub(1);
                }
                if j < i {
                    break;
                }
                if j != i {
                    swap_elements(e, base, i, j, 16);
                }
                j = j.wrapping_sub(1);
                i = i.wrapping_add(1);
                if i > j {
                    break;
                }
            }
            if lo < j {
                fn_00cf83c0(e, base, lo, j, compare);
            }
            if i < hi {
                lo = i;
            } else {
                break;
            }
        }
    })
}

/// Exchanges elements `i` and `j` of `size` bytes each.
fn swap_elements(e: &mut Engine, base: u32, i: i32, j: i32, size: u32) {
    let first = base.wrapping_add((i as u32).wrapping_mul(size));
    let second = base.wrapping_add((j as u32).wrapping_mul(size));
    let first_bytes = e.mem.bytes(first, size);
    let second_bytes = e.mem.bytes(second, size);
    e.mem.write(first, &second_bytes);
    e.mem.write(second, &first_bytes);
}

// Translated from 00cf8510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same quicksort over 0x14-byte elements `base[lo..=hi]`; here `compare` is cdecl
/// `(result, a, b)`: it stores an `hkBool` in `result` and returns its address, and the sort
/// reads the byte at the returned address.
pub fn fn_00cf8510(e: &mut Engine, base: u32, lo: i32, hi: i32, compare: u32) {
    // The exe aligns ESP, then `SUB ESP,0x54` and three pushes: a 0x60-byte frame, the pivot
    // element copied to +0x40 (00cf853b..00cf8566), the comparison's answer byte at +0x1a
    // (00cf8570).
    with_aligned_frame(e, 0x60, |e, frame| {
        let mut lo = lo;
        let pivot = frame + 0x40;
        let result = frame + 0x1a;
        loop {
            let middle = (lo.wrapping_add(hi)) >> 1;
            let source = base.wrapping_add((middle as u32).wrapping_mul(0x14));
            let bytes = e.mem.bytes(source, 0x14);
            e.mem.write(pivot, &bytes);
            let mut i = lo;
            let mut j = hi;
            loop {
                let mut left = base.wrapping_add((i as u32).wrapping_mul(0x14));
                while compare_flag(e, compare, result, left, pivot) {
                    left = left.wrapping_add(0x14);
                    i = i.wrapping_add(1);
                }
                let mut right = base.wrapping_add((j as u32).wrapping_mul(0x14));
                while compare_flag(e, compare, result, pivot, right) {
                    right = right.wrapping_sub(0x14);
                    j = j.wrapping_sub(1);
                }
                if j < i {
                    break;
                }
                if j != i {
                    swap_elements(e, base, i, j, 0x14);
                }
                j = j.wrapping_sub(1);
                i = i.wrapping_add(1);
                if i > j {
                    break;
                }
            }
            if lo < j {
                fn_00cf8510(e, base, lo, j, compare);
            }
            if i < hi {
                lo = i;
            } else {
                break;
            }
        }
    })
}

/// Calls the hidden-result comparison and reads the byte it points to.
fn compare_flag(e: &mut Engine, compare: u32, result: u32, a: u32, b: u32) -> bool {
    let flag = e.call(compare, &args![result, a, b]).u32();
    e.mem.u8(flag) != 0
}

/// What `00cf7c10` does after an operation: nothing, or release the reference held on the
/// record's first or second object.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Release {
    Nothing,
    First,
    Second,
}

/// A vector `(x, y, z, w)` of raw float bits built from the three floats at `words` and the
/// constant fourth lane, stored at `block` (its slot in `00cf7c10`'s frame).
fn operation_vector(e: &mut Engine, words: u32, block: u32) -> u32 {
    for lane in 0..3 {
        let value = e.mem.u32(words + 4 * lane);
        e.mem.set_u32(block + 4 * lane, value);
    }
    let fourth = e.mem.u32(VECTOR_FOURTH_LANE);
    e.mem.set_u32(block + 12, fourth);
    block
}

/// `00c90740(array, count, 4)` followed by freeing the block of `count` pointers.
fn release_batch(e: &mut Engine, array: u32, count: u32) {
    e.call(RELEASE_BATCH, &args![array, count, 4u32]);
    let alloc = allocator(e);
    e.vcall(alloc, 8, &args![array, count * 4]);
}

// Translated from 00cf7c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Executes the queue's pending operations (the world's `executeAllPending`, name
/// unconfirmed). Under the reference-count lock it takes the queue's record array into a
/// local one (swap), raises the world's nesting count (`+0xA4`), applies the queued
/// island-merge requests (second array, sorted by `00cf74e0` through `00cf8510`, last first:
/// `mergeIslands` when both entities are in this world, neither is of motion type 5 and
/// their islands differ, then both references are released), then runs every record by its
/// kind byte, in order, and releases the references the queueing took (see `00cf7690`);
/// records queued meanwhile are run by a nested call. Kinds: 1 addEntity, 2 removeEntity,
/// 3 updateEntityBP, 4 setMotionType, 5 a call of the object's slot `+0xC`, 6 addEntityBatch,
/// 7 removeEntityBatch, 8 addConstraint, 9 removeConstraint, 10 addAction (when all its
/// entities are in this world), 11 removeAction, 13 addPhantom, 14 removePhantom, 15
/// addPhantomBatch, 16 removePhantomBatch, 17 updateBroadPhase of a phantom, 18 and 20
/// collision-filter updates of an entity and a phantom, 19 re-enabling collision between a
/// pair, 21 the world's collision-filter update, 22 the warped-body update, 23 a batch of
/// activations, 24 setPositionAndRotation, 25 and 26 linear and angular velocity, 27-29
/// activate plus a motion call, 32 activateRegion, 33 activate, 34 deactivate, 35 a call
/// on the object, 36 a call of the object's slot `+0xC`; 12, 30, 31 and the unknown kinds do
/// nothing. The names are those of the Xbox PDB for the called functions.
pub fn fn_00cf7c10(e: &mut Engine, this: Ptr) {
    // The exe aligns ESP, then `SUB ESP,0x84` and three pushes: a 0x90-byte frame with the
    // swapped-out array at +0x34 (00cf7c29), the answer bytes at +0x1a and +0x1b, the action's
    // entity array at +0x28 and the operation vectors at +0x40..+0x70 (see `run_record`).
    with_aligned_frame(e, 0x90, |e, frame| {
        let queue = this.addr();
        e.call(LOCK_ENTER, &args![]);
        let local = frame + 0x34;
        e.mem.set_u32(local, 0);
        e.mem.set_u32(local + 4, 0);
        e.mem.set_u32(local + 8, NO_OWNED_MEMORY);
        e.call(ARRAY_SWAP, &args![local, queue]);
        let world = e.mem.u32(queue + 0xc);
        let nesting = e.mem.u32(world + 0xa4);
        e.mem.set_u32(world + 0xa4, nesting.wrapping_add(1));
        run_merges(e, queue);
        let count = e.mem.u32(local + 4) as i32;
        if count > 0 {
            let mut record = e.mem.u32(local);
            for _ in 0..count {
                run_record(e, queue, record, frame);
                if e.mem.u32(queue + 4) != 0 {
                    fn_00cf7c10(e, this);
                }
                record = record.wrapping_add(0x14);
            }
        }
        e.call(ARRAY_SWAP, &args![local, queue]);
        e.mem.set_u32(queue + 4, 0);
        let world = e.mem.u32(queue + 0xc);
        let nesting = e.mem.u32(world + 0xa4);
        e.mem.set_u32(world + 0xa4, nesting.wrapping_sub(1));
        e.call(LOCK_LEAVE, &args![]);
        let alloc = allocator(e);
        let capacity_flags = e.mem.u32(local + 8);
        if (capacity_flags as i32) >= 0 {
            let bytes = (capacity_flags & CAPACITY_MASK).wrapping_mul(0x14);
            let data = e.mem.u32(local);
            e.vcall(alloc, 0x10, &args![data, bytes]);
        }
    })
}

/// The first part of `00cf7c10`: applies the queued island-merge requests.
fn run_merges(e: &mut Engine, queue: u32) {
    let pending = e.mem.u32(queue + 0x14);
    if pending == 0 {
        return;
    }
    if (pending as i32) > 1 {
        let data = e.mem.u32(queue + 0x10);
        fn_00cf8510(e, data, 0, pending as i32 - 1, COMPARE_MERGES);
    }
    if e.mem.u32(queue + 0x14) == 0 {
        return;
    }
    loop {
        let count = e.mem.u32(queue + 0x14);
        let data = e.mem.u32(queue + 0x10);
        let request = data
            .wrapping_add(count.wrapping_mul(0x14))
            .wrapping_sub(0x14);
        let first = e.mem.u32(request + 4);
        let second = e.mem.u32(request + 8);
        e.mem.set_u32(queue + 0x14, count.wrapping_sub(1));
        let world = e.mem.u32(queue + 0xc);
        let same_world = e.mem.u32(first + 8) == world && e.mem.u32(second + 8) == world;
        // The motion-type byte of both entities is not 5 (fixed bodies) and their islands differ.
        if same_world
            && e.mem.u8(first + 0xe8) != 5
            && e.mem.u8(second + 0xe8) != 5
            && e.mem.u32(first + 0xcc) != e.mem.u32(second + 0xcc)
        {
            let island_world = e.mem.u32(first + 8);
            e.call(MERGE_ISLANDS, &args![island_world, first, second]);
        }
        e.call(REMOVE_REFERENCE, &args![first]);
        e.call(REMOVE_REFERENCE, &args![second]);
        if e.mem.u32(queue + 0x14) == 0 {
            break;
        }
    }
}

/// Runs one operation record and releases what it holds.
fn run_record(e: &mut Engine, queue: u32, record: u32, frame: u32) {
    let world = e.mem.u32(queue + 0xc);
    let a = e.mem.u32(record + 4);
    let b = e.mem.u32(record + 8);
    let in_world = |e: &Engine, object: u32| e.mem.u32(object + 8) == world;
    let release = match e.mem.u8(record) {
        1 => {
            if e.mem.u32(a + 8) == 0 {
                e.call(0x00c9_14d0, &args![world, a, b]);
            }
            Release::First
        }
        2 => {
            if in_world(e, a) {
                // The answer byte at +0x1a of `00cf7c10`'s frame (00cf7d5a).
                e.call(0x00c9_1620, &args![world, frame + 0x1a, a]);
            }
            Release::First
        }
        3 => {
            if in_world(e, a) {
                e.call(UPDATE_ENTITY_BP, &args![world, a]);
            }
            Release::First
        }
        4 => {
            let kind = u32::from(e.mem.u8(record + 8));
            let first = u32::from(e.mem.u8(record + 9));
            let second = u32::from(e.mem.u8(record + 10));
            e.call(0x00ca_8420, &args![a, kind, first, second]);
            Release::First
        }
        5 => {
            e.vcall(a, 0xc, &args![b]);
            e.call(REMOVE_REFERENCE, &args![a]);
            Release::Second
        }
        6 => {
            let count = u32::from(e.mem.u16(record + 8));
            let flag = u32::from(e.mem.u8(record + 10));
            e.call(0x00c9_4bd0, &args![world, a, count, flag]);
            release_batch(e, a, u32::from(e.mem.u16(record + 8)));
            Release::Nothing
        }
        7 => {
            let count = u32::from(e.mem.u16(record + 8));
            e.call(0x00c9_51b0, &args![world, a, count]);
            release_batch(e, a, u32::from(e.mem.u16(record + 8)));
            Release::Nothing
        }
        8 => {
            if e.mem.u32(a + 8) == 0 {
                let first_body = e.mem.u32(a + 0x14);
                if in_world(e, first_body) {
                    let second_body = e.mem.u32(a + 0x18);
                    if in_world(e, second_body) {
                        e.call(0x00c9_7d30, &args![world, a]);
                    }
                }
            }
            Release::First
        }
        9 => {
            if e.mem.u32(a + 8) != 0 {
                // The answer byte at +0x1b of `00cf7c10`'s frame (00cf7e5b).
                e.call(0x00c9_7e80, &args![world, frame + 0x1b, a]);
            }
            Release::First
        }
        10 => {
            if e.mem.u32(a + 8) == 0 {
                add_action_if_all_in_world(e, world, a, frame + 0x28);
            }
            Release::First
        }
        11 => {
            if e.mem.u32(a + 0xc) != 0 {
                e.call(0x00c9_7f50, &args![world, a]);
            }
            Release::First
        }
        13 => {
            if e.mem.u32(a + 8) == 0 {
                e.call(0x00c9_36a0, &args![world, a]);
            }
            Release::First
        }
        14 => {
            if in_world(e, a) {
                e.call(0x00c9_2c70, &args![world, a]);
            }
            Release::First
        }
        15 => {
            let count = u32::from(e.mem.u16(record + 8));
            e.call(0x00c9_54e0, &args![world, a, count]);
            release_batch(e, a, u32::from(e.mem.u16(record + 8)));
            Release::Nothing
        }
        16 => {
            let count = u32::from(e.mem.u16(record + 8));
            e.call(0x00c9_5800, &args![world, a, count]);
            release_batch(e, a, u32::from(e.mem.u16(record + 8)));
            Release::Nothing
        }
        17 => {
            if in_world(e, a) {
                e.call(0x00d0_7ec0, &args![a, b]);
            }
            e.call(REMOVE_REFERENCE, &args![a]);
            if b != 0 {
                let alloc = allocator(e);
                e.vcall(alloc, 8, &args![b, 0x20u32]);
            }
            Release::Nothing
        }
        18 => {
            if in_world(e, a) {
                let first = u32::from(e.mem.u8(record + 8));
                let second = u32::from(e.mem.u8(record + 9));
                e.call(0x00c9_46e0, &args![world, a, first, second]);
            }
            Release::First
        }
        19 => {
            if in_world(e, a) && in_world(e, b) {
                e.call(0x00c9_13b0, &args![world, a, b]);
            }
            e.call(REMOVE_REFERENCE, &args![a]);
            Release::Second
        }
        20 => {
            if in_world(e, a) {
                let flag = u32::from(e.mem.u8(record + 8));
                e.call(0x00c9_44a0, &args![world, a, flag]);
            }
            Release::First
        }
        21 => {
            let first = u32::from(e.mem.u8(record + 1));
            let second = u32::from(e.mem.u8(record + 2));
            e.call(0x00c9_6ff0, &args![world, first, second]);
            Release::Nothing
        }
        22 => {
            e.call(UPDATE_WARPED_BODY, &args![a]);
            Release::First
        }
        23 => {
            activate_batch(e, world, record);
            Release::Nothing
        }
        24 => {
            e.call(0x00ca_8860, &args![a, b, b + 0x10]);
            let alloc = allocator(e);
            e.vcall(alloc, 8, &args![b, 0x20u32]);
            Release::First
        }
        25 => {
            let vector = operation_vector(e, record + 8, frame + 0x40);
            e.call(0x0056_16d0, &args![a, vector]);
            Release::First
        }
        26 => {
            let vector = operation_vector(e, record + 8, frame + 0x50);
            e.call(0x0056_1800, &args![a, vector]);
            Release::First
        }
        27 | 29 => {
            // Kind 27 builds its vector at +0x60, kind 29 at +0x70 (00cf80fe, 00cf8186).
            let (slot, at) = if e.mem.u8(record) == 27 {
                (0x4c, frame + 0x60)
            } else {
                (0x54, frame + 0x70)
            };
            let vector = operation_vector(e, record + 8, at);
            e.call(0x00c9_c1d0, &args![a]);
            e.vcall(a + 0xe0, slot, &args![vector]);
            Release::First
        }
        28 => {
            e.call(0x00c9_c1d0, &args![a]);
            e.vcall(a + 0xe0, 0x50, &args![b + 0x10, b]);
            let alloc = allocator(e);
            e.vcall(alloc, 8, &args![b, 0x20u32]);
            Release::First
        }
        32 => {
            e.call(0x00c9_3c30, &args![world, a]);
            let alloc = allocator(e);
            e.vcall(alloc, 8, &args![a, 0x20u32]);
            Release::Nothing
        }
        33 => {
            e.call(0x00c9_c1d0, &args![a]);
            Release::First
        }
        34 => {
            if in_world(e, a) {
                e.call(0x00c9_c240, &args![a]);
            }
            Release::First
        }
        35 => {
            let flag = u32::from(e.mem.u8(record + 12));
            e.call(0x00d8_7c20, &args![a, b, flag]);
            e.call(REMOVE_REFERENCE, &args![a]);
            Release::Second
        }
        36 => {
            e.vcall(a, 0xc, &args![b]);
            Release::First
        }
        _ => Release::Nothing,
    };
    match release {
        Release::Nothing => {}
        Release::First => {
            e.call(REMOVE_REFERENCE, &args![a]);
        }
        Release::Second => {
            e.call(REMOVE_REFERENCE, &args![b]);
        }
    }
}

/// Kind 10: when none of the action's entities (its slot `+0x10` fills an array with them)
/// lies in another world, adds the action to the world. `entities` is the array's slot in
/// `00cf7c10`'s frame (+0x28, 00cf7e77..00cf7e8c).
fn add_action_if_all_in_world(e: &mut Engine, world: u32, action: u32, entities: u32) {
    e.mem.set_u32(entities, 0);
    e.mem.set_u32(entities + 4, 0);
    e.mem.set_u32(entities + 8, NO_OWNED_MEMORY);
    e.vcall(action, 0x10, &args![entities]);
    let count = e.mem.u32(entities + 4) as i32;
    let mut all_in_world = true;
    let mut index = 0;
    while index < count {
        let entity = e.mem.u32(e.mem.u32(entities) + 4 * index as u32);
        if e.mem.u32(entity + 8) != world {
            all_in_world = false;
            break;
        }
        index += 1;
    }
    if all_in_world {
        e.call(0x00c9_7370, &args![world, action]);
    }
    let alloc = allocator(e);
    e.mem.set_u32(entities + 4, 0);
    let capacity_flags = e.mem.u32(entities + 8);
    if (capacity_flags as i32) >= 0 {
        let bytes = (capacity_flags & CAPACITY_MASK) * 4;
        let data = e.mem.u32(entities);
        e.vcall(alloc, 0x10, &args![data, bytes]);
    }
}

/// Kind 23: moves the entries of the pointer array (`a`, `b & 0xFFFF` entries) that are not
/// in this world to the end and drops them from the count, passes the rest to the
/// simulation island manager (`world + 8`, its slot `+0x30`), then releases every original
/// entry and frees the block.
fn activate_batch(e: &mut Engine, world: u32, record: u32) {
    let array = e.mem.u32(record + 4);
    let total = u32::from(e.mem.u16(record + 8));
    let mut remaining = total;
    let mut last = array.wrapping_add(total * 4).wrapping_sub(4);
    let mut scan = last;
    while scan >= array {
        let object = e.mem.u32(scan);
        if e.mem.u32(object + 8) != world {
            let other = e.mem.u32(last);
            e.mem.set_u32(scan, other);
            e.mem.set_u32(last, object);
            last = last.wrapping_sub(4);
            remaining -= 1;
        }
        scan = scan.wrapping_sub(4);
    }
    if remaining != 0 {
        let flag = u32::from(e.mem.u8(record + 10));
        let manager = e.mem.u32(world + 8);
        e.vcall(manager, 0x30, &args![array, remaining, world, flag]);
    }
    let end = array.wrapping_add(u32::from(e.mem.u16(record + 8)) * 4);
    let mut cursor = array;
    while cursor < end {
        let object = e.mem.u32(cursor);
        cursor += 4;
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    let alloc = allocator(e);
    let count = u32::from(e.mem.u16(record + 8));
    e.vcall(alloc, 8, &args![array, count * 4]);
}

// Translated from 00cf7540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a 16-byte `(a, b, c, d)` record in the queue's array at `+0x1c` (the `(object,
/// listener)` pairs): under the reference-count lock (`00c90490`/`00c904e0`) the world's
/// pending-body-operation counter (`[world + 0x90]`, `world` = the word at `this + 0xc`) is
/// raised by one, the array grows by `00c90940` when it is full, the record is appended with
/// its words in the order `a`, `b`, `c`, `d`, and a reference is added to `a` and then to `b`
/// (`00c90510`).
pub fn fn_00cf7540(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32, d: u32) {
    let queue = this.addr();
    e.call(LOCK_ENTER, &args![]);
    let world = e.mem.u32(queue + 0xc);
    let pending = e.mem.u32(world + 0x90);
    e.mem.set_u32(world + 0x90, pending.wrapping_add(1));
    let array = queue + 0x1c;
    let alloc = allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_MASK {
        e.call(ARRAY_RESERVE_MORE, &args![alloc, array, 0x10u32]);
    }
    let size = e.mem.u32(array + 4);
    let record = size.wrapping_shl(4).wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(record + 8, c);
    e.mem.set_u32(record, a);
    e.mem.set_u32(record + 4, b);
    e.mem.set_u32(record + 0xc, d);
    e.call(ADD_REFERENCE, &args![a]);
    e.call(ADD_REFERENCE, &args![b]);
    e.call(LOCK_LEAVE, &args![]);
}

// Translated from 00cf75c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the queue: three empty arrays (data 0, size 0, capacity word `0x80000000`,
/// not owned) at `+0`, `+0x10` and `+0x1c` (the words at `+0`/`+4`/`+8`, `+0x10`/`+0x14`/`+0x18`,
/// `+0x1c`/`+0x20`/`+0x24`) and the world pointer `world` at `+0xc`. Returns `this`.
pub fn fn_00cf75c0(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let base = this.addr();
    for array in [0x0, 0x10, 0x1c] {
        e.mem.set_u32(base + array, 0);
        e.mem.set_u32(base + array + 4, 0);
        e.mem.set_u32(base + array + 8, NO_OWNED_MEMORY);
    }
    e.mem.set_u32(base + 0xc, world);
    this
}

/// Empties the `hkArray` at `array` (data `+0`, size `+4`, capacity word `+8`) of
/// `element_size`-byte elements: the size becomes 0 and, unless bit 31 of the capacity word
/// says the memory is not owned, the data is freed through the memory router's allocator
/// (slot `0x10`, `(capacity & 0x3fffffff) * element_size` bytes); then the data pointer is
/// cleared and the capacity word set to `0x80000000`.
fn empty_array(e: &mut Engine, array: u32, element_size: u32) {
    let alloc = allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NO_OWNED_MEMORY == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(alloc, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NO_OWNED_MEMORY);
}

// Translated from 00cf75f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the queue: empties the array of 16-byte pairs at `+0x1c`, then the arrays
/// of 20-byte records at `+0x10` and at `+0` (see `empty_array`).
pub fn fn_00cf75f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    empty_array(e, base + 0x1c, 0x10);
    empty_array(e, base + 0x10, 0x14);
    empty_array(e, base, 0x14);
}

// Translated from 00cf8690 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(destination, source)`: copies two 16-byte vectors (eight words, in order) from
/// `source` to `destination`.
pub fn fn_00cf8690(e: &mut Engine, destination: Ptr, source: Ptr) {
    for word in 0..8 {
        let value = e.mem.u32(source.addr() + 4 * word);
        e.mem.set_u32(destination.addr() + 4 * word, value);
    }
}

// Translated from 00cf86d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vtable word `0102e388` at `this`.
pub fn fn_00cf86d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_e388);
}

// Translated from 00cf86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `+0x14` plus `value` at `+0x20` (an x87 `FADD` of the float memory
/// operand, the result stored as a float).
pub fn fn_00cf86e0(e: &mut Engine, this: Ptr, value: f32) {
    let base = this.addr();
    let current = x87_load32(e.mem.u32(base + 0x14));
    let sum = x87_add(current, x87_m32(value.to_bits()));
    e.mem.set_u32(base + 0x20, x87_store32(sum));
}

// Translated from 00cf86f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the floats at `+0x14` and `+0x20` are equal (`FUCOMPP`; a NaN is never equal).
pub fn fn_00cf86f0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let first = x87_load32(e.mem.u32(base + 0x14));
    let second = x87_load32(e.mem.u32(base + 0x20));
    second == first
}

// Translated from 00cf74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `first` and `second`, each the address of a
/// record; descriptions, not confirmed): a "less than" test (code at `00cf7b59` stores its
/// address). Records whose first words are equal compare by the word at `+8`: the answer is 1
/// when `second`'s is below `first`'s (unsigned). Otherwise they compare by the word at `+0xd4`
/// of the object each first word points to: 1 when `first`'s is below `second`'s (unsigned).
/// Answers 1 or 0 (`SBB`/`NEG`).
pub fn fn_00cf74b0(e: &mut Engine, first: Ptr, second: Ptr) -> u32 {
    let first_object = e.mem.u32(first.addr());
    let second_object = e.mem.u32(second.addr());
    if first_object == second_object {
        let first_key = e.mem.u32(first.addr().wrapping_add(8));
        let second_key = e.mem.u32(second.addr().wrapping_add(8));
        return u32::from(second_key < first_key);
    }
    let first_key = e.mem.u32(first_object.wrapping_add(0xd4));
    let second_key = e.mem.u32(second_object.wrapping_add(0xd4));
    u32::from(first_key < second_key)
}

// Translated from 00cf74e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words `out`, `first`, `second`; descriptions, not
/// confirmed): a "less than" test that stores its answer as a byte at `out` (code at `00cf7c5b`
/// stores its address) and answers `out`. For each record the key is the 16-bit word at `+0x20`
/// of the object at `+0xcc` of the object the record's word at `+4` points to; the answer is 1
/// when `first`'s key is below `second`'s (unsigned). Equal keys are decided the same way on the
/// words at `+8` of the records; equal again is 0.
pub fn fn_00cf74e0(e: &mut Engine, out: Ptr, first: Ptr, second: Ptr) -> u32 {
    let key = |e: &Engine, record: u32, offset: u32| -> u32 {
        let object = e.mem.u32(record.wrapping_add(offset));
        let group = e.mem.u32(object.wrapping_add(0xcc));
        u32::from(e.mem.u16(group.wrapping_add(0x20)))
    };
    let first_key = key(e, first.addr(), 4);
    let second_key = key(e, second.addr(), 4);
    let less = if first_key < second_key {
        true
    } else if first_key != second_key {
        false
    } else {
        key(e, first.addr(), 8) < key(e, second.addr(), 8)
    };
    e.mem.set_u8(out.addr(), u8::from(less));
    out.addr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf74b0, fn_00cf74b0(Ptr, Ptr) -> u32),
        entry!(0x00cf74e0, fn_00cf74e0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00cf_7540, fn_00cf7540(Ptr, u32, u32, u32, u32)),
        entry!(0x00cf_75c0, fn_00cf75c0(Ptr, u32) -> Ptr),
        entry!(0x00cf_75f0, fn_00cf75f0(Ptr)),
        entry!(0x00cf_7690, fn_00cf7690(Ptr, Ptr)),
        entry!(0x00cf_7ae0, fn_00cf7ae0(Ptr)),
        entry!(0x00cf_7c10, fn_00cf7c10(Ptr)),
        entry!(0x00cf_83c0, fn_00cf83c0(u32, i32, i32, u32)),
        entry!(0x00cf_8510, fn_00cf8510(u32, i32, i32, u32)),
        entry!(0x00cf_8690, fn_00cf8690(Ptr, Ptr)),
        entry!(0x00cf_86d0, fn_00cf86d0(Ptr)),
        entry!(0x00cf_86e0, fn_00cf86e0(Ptr, f32)),
        entry!(0x00cf_86f0, fn_00cf86f0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A record whose first word is `object` and whose word at `+8` is `key`.
    fn record_00cf74b0(e: &mut Engine, object: u32, key: u32) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, object);
        e.mem.set_u32(record + 8, key);
        record
    }

    #[test]
    fn fn_00cf74b0_compares_the_second_words_when_the_first_words_match() {
        let mut e = Engine::new();
        let shared = e.mem.alloc(0x100);
        let low = record_00cf74b0(&mut e, shared, 3);
        let high = record_00cf74b0(&mut e, shared, 5);
        assert_eq!(e.call(0x00cf_74b0, &args![high, low]).u32(), 1);
        assert_eq!(e.call(0x00cf_74b0, &args![low, high]).u32(), 0);
        assert_eq!(e.call(0x00cf_74b0, &args![low, low]).u32(), 0);
    }

    #[test]
    fn fn_00cf74b0_compares_the_objects_words_when_the_first_words_differ() {
        let mut e = Engine::new();
        let small = e.mem.alloc(0x100);
        let large = e.mem.alloc(0x100);
        e.mem.set_u32(small + 0xd4, 1);
        e.mem.set_u32(large + 0xd4, 0x8000_0000);
        let first = record_00cf74b0(&mut e, small, 9);
        let second = record_00cf74b0(&mut e, large, 0);
        assert_eq!(e.call(0x00cf_74b0, &args![first, second]).u32(), 1);
        assert_eq!(e.call(0x00cf_74b0, &args![second, first]).u32(), 0);
    }

    #[test]
    fn fn_00cf74b0_treats_equal_object_words_as_not_less() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x100);
        let b = e.mem.alloc(0x100);
        e.mem.set_u32(a + 0xd4, 7);
        e.mem.set_u32(b + 0xd4, 7);
        let first = record_00cf74b0(&mut e, a, 0);
        let second = record_00cf74b0(&mut e, b, 0);
        assert_eq!(e.call(0x00cf_74b0, &args![first, second]).u32(), 0);
    }

    /// A record with two objects (at `+4` and `+8`) whose groups hold the keys `first_key` and
    /// `second_key` as the 16-bit word at `+0x20`.
    fn keyed_record_00cf74e0(e: &mut Engine, first_key: u16, second_key: u16) -> u32 {
        let record = e.mem.alloc(0x10);
        for (offset, key) in [(4, first_key), (8, second_key)] {
            let object = e.mem.alloc(0x100);
            let group = e.mem.alloc(0x40);
            e.mem.set_u32(object + 0xcc, group);
            e.mem.set_u16(group + 0x20, key);
            e.mem.set_u32(record + offset, object);
        }
        record
    }

    #[test]
    fn fn_00cf74e0_decides_by_the_first_keys() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let low = keyed_record_00cf74e0(&mut e, 1, 9);
        let high = keyed_record_00cf74e0(&mut e, 2, 0);
        assert_eq!(e.call(0x00cf_74e0, &args![out, low, high]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00cf_74e0, &args![out, high, low]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00cf74e0_breaks_a_tie_with_the_second_keys() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let low = keyed_record_00cf74e0(&mut e, 4, 5);
        let high = keyed_record_00cf74e0(&mut e, 4, 6);
        e.call(0x00cf_74e0, &args![out, low, high]);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00cf_74e0, &args![out, high, low]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00cf_74e0, &args![out, low, low]);
        assert_eq!(e.mem.u8(out), 0);
    }

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOCATE_DOUBLE: u32 = 0x0900_0100;
    const FREE_DOUBLE: u32 = 0x0900_0110;
    const FREE_ARRAY_DOUBLE: u32 = 0x0900_0120;
    const LISTENER_DOUBLE: u32 = 0x0900_0130;
    const ACTION_DOUBLE: u32 = 0x0900_0140;
    const MOTION_DOUBLE: u32 = 0x0900_0150;
    const MANAGER_DOUBLE: u32 = 0x0900_0160;
    const OBJECT_DOUBLE: u32 = 0x0900_0170;

    /// Every address the translations call outside this file.
    const CALLEES: &[u32] = &[
        LOCK_ENTER,
        LOCK_LEAVE,
        ADD_REFERENCE,
        REMOVE_REFERENCE,
        REMOVE_REFERENCE_UNCHECKED,
        ARRAY_RESERVE_MORE,
        ARRAY_RESERVE,
        RELEASE_BATCH,
        MERGE_ISLANDS,
        UPDATE_ENTITY_BP,
        UPDATE_WARPED_BODY,
        0x00c9_14d0,
        0x00c9_1620,
        0x00ca_8420,
        0x00c9_4bd0,
        0x00c9_51b0,
        0x00c9_7d30,
        0x00c9_7e80,
        0x00c9_7370,
        0x00c9_7f50,
        0x00c9_36a0,
        0x00c9_2c70,
        0x00c9_54e0,
        0x00c9_5800,
        0x00d0_7ec0,
        0x00c9_46e0,
        0x00c9_13b0,
        0x00c9_44a0,
        0x00c9_6ff0,
        0x00ca_8860,
        0x0056_16d0,
        0x0056_1800,
        0x00c9_c1d0,
        0x00c9_c240,
        0x00d8_7c20,
        0x00c9_3c30,
        COMPARE_PAIRS,
        COMPARE_MERGES,
    ];

    /// An engine whose callees are logging doubles, with a memory router whose allocator
    /// (slots 4, 8 and 0x10) logs under the pseudo addresses `ALLOCATE_DOUBLE`,
    /// `FREE_DOUBLE` and `FREE_ARRAY_DOUBLE`; the swap and the copy really work.
    fn engine() -> (Engine, Calls) {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in CALLEES.iter().copied() {
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.register_double(ARRAY_SWAP, |e, a| {
            for word in 0..3 {
                let first = e.mem.u32(a[0] + 4 * word);
                let second = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, second);
                e.mem.set_u32(a[1] + 4 * word, first);
            }
            Ret::default()
        });
        e.register_double(MEM_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 8, FREE_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(ALLOCATE_DOUBLE, move |e, a| {
            log.borrow_mut().push((ALLOCATE_DOUBLE, a[1..].to_vec()));
            Ret {
                eax: e.mem.alloc(a[1].max(4)),
                ..Ret::default()
            }
        });
        for address in [FREE_DOUBLE, FREE_ARRAY_DOUBLE] {
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a[1..].to_vec()));
                Ret::default()
            });
        }
        (e, calls)
    }

    fn calls_to(calls: &Calls, address: u32) -> Vec<Vec<u32>> {
        calls
            .borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A queue (0x28 bytes) in a world (0x100 bytes) with room for `capacity` records.
    fn queue(e: &mut Engine, capacity: u32) -> (u32, u32) {
        let world = e.mem.alloc(0x100);
        let queue = e.mem.alloc(0x30);
        let data = e.mem.alloc(capacity * 0x14 + 4);
        e.mem.set_u32(queue, data);
        e.mem.set_u32(queue + 8, capacity);
        e.mem.set_u32(queue + 0xc, world);
        (queue, world)
    }

    /// A Havok object in `world` (0 for none) with island `island` and motion type `motion`.
    fn object(e: &mut Engine, world: u32, island: u32, motion: u8) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 8, world);
        e.mem.set_u32(object + 0xcc, island);
        e.mem.set_u8(object + 0xe8, motion);
        object
    }

    /// An operation record of `kind` with the given words.
    fn record(e: &mut Engine, kind: u8, words: [u32; 4]) -> u32 {
        let record = e.mem.alloc(0x14);
        e.mem.set_u8(record, kind);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(record + 4 + 4 * i as u32, *w);
        }
        record
    }

    fn enqueue(e: &mut Engine, queue: u32, kind: u8, words: [u32; 4]) {
        let op = record(e, kind, words);
        e.call(0x00cf_7690, &args![queue, op]);
    }

    fn stored_record(e: &Engine, queue: u32, index: u32) -> Vec<u32> {
        let data = e.mem.u32(queue);
        (0..5)
            .map(|w| e.mem.u32(data + index * 0x14 + 4 * w))
            .collect()
    }

    #[test]
    fn enqueue_stores_the_record_and_references_the_first_object() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 4);
        enqueue(&mut e, queue, 1, [0x1111, 0x2222, 3, 4]);
        assert_eq!(e.mem.u32(queue + 4), 1);
        assert_eq!(e.mem.u32(world + 0x8c), 1);
        assert_eq!(stored_record(&e, queue, 0), vec![1, 0x1111, 0x2222, 3, 4]);
        assert_eq!(calls_to(&calls, ADD_REFERENCE), vec![vec![0x1111]]);
        let order: Vec<u32> = calls.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order.first(), Some(&LOCK_ENTER));
        assert_eq!(order.last(), Some(&LOCK_LEAVE));
        // A second record goes after the first, and kinds without references add none.
        enqueue(&mut e, queue, 21, [5, 6, 7, 8]);
        assert_eq!(e.mem.u32(queue + 4), 2);
        assert_eq!(stored_record(&e, queue, 1), vec![21, 5, 6, 7, 8]);
        assert_eq!(calls_to(&calls, ADD_REFERENCE).len(), 1);
    }

    #[test]
    fn enqueue_grows_a_full_array_first() {
        let (mut e, calls) = engine();
        let (queue, _) = queue(&mut e, 1);
        enqueue(&mut e, queue, 33, [9, 0, 0, 0]);
        assert!(calls_to(&calls, ARRAY_RESERVE_MORE).is_empty());
        enqueue(&mut e, queue, 33, [10, 0, 0, 0]);
        let reserve = calls_to(&calls, ARRAY_RESERVE_MORE);
        assert_eq!(reserve.len(), 1);
        assert_eq!((reserve[0][1], reserve[0][2]), (queue, 0x14));
    }

    #[test]
    fn enqueue_kinds_with_two_references_and_the_removal_kinds() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 8);
        enqueue(&mut e, queue, 19, [0xa1, 0xb1, 0, 0]);
        assert_eq!(
            calls_to(&calls, ADD_REFERENCE),
            vec![vec![0xa1], vec![0xb1]]
        );
        // Kind 30 references its object and takes its own record back out; kind 31 releases.
        enqueue(&mut e, queue, 30, [0xc1, 0, 0, 0]);
        assert_eq!(e.mem.u32(queue + 4), 1);
        assert_eq!(e.mem.u32(world + 0x8c), 1);
        assert_eq!(calls_to(&calls, ADD_REFERENCE).len(), 3);
        enqueue(&mut e, queue, 31, [0xd1, 0, 0, 0]);
        assert_eq!(e.mem.u32(queue + 4), 1);
        assert_eq!(calls_to(&calls, REMOVE_REFERENCE), vec![vec![0xd1]]);
    }

    #[test]
    fn enqueue_copies_pointer_arrays_and_references_each_entry() {
        let (mut e, calls) = engine();
        let (queue, _) = queue(&mut e, 4);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x7001);
        e.mem.set_u32(source + 4, 0x7002);
        enqueue(&mut e, queue, 6, [source, 2, 0, 0]);
        let stored = stored_record(&e, queue, 0);
        assert_ne!(stored[1], source);
        assert_eq!(e.mem.u32(stored[1]), 0x7001);
        assert_eq!(e.mem.u32(stored[1] + 4), 0x7002);
        assert_eq!(calls_to(&calls, ALLOCATE_DOUBLE), vec![vec![8]]);
        assert_eq!(
            calls_to(&calls, ADD_REFERENCE),
            vec![vec![0x7001], vec![0x7002]]
        );
    }

    #[test]
    fn enqueue_kind_17_keeps_a_copy_of_the_block() {
        let (mut e, calls) = engine();
        let (queue, _) = queue(&mut e, 4);
        let block = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_u32(block + 4 * word, 0x100 + word);
        }
        enqueue(&mut e, queue, 17, [0xe1, block, 0, 0]);
        let stored = stored_record(&e, queue, 0);
        assert_ne!(stored[2], block);
        assert_eq!(e.mem.u32(stored[2] + 0x1c), 0x107);
        assert_eq!(calls_to(&calls, ALLOCATE_DOUBLE), vec![vec![0x20]]);
        assert_eq!(calls_to(&calls, ADD_REFERENCE), vec![vec![0xe1]]);
    }

    #[test]
    fn enqueue_kind_12_moves_the_record_to_the_second_array() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 4);
        let second = e.mem.alloc(0x14 * 4);
        e.mem.set_u32(queue + 0x10, second);
        e.mem.set_u32(queue + 0x18, 4);
        enqueue(&mut e, queue, 12, [0xf1, 0xf2, 0xf3, 0xf4]);
        assert_eq!(e.mem.u32(queue + 4), 0);
        assert_eq!(e.mem.u32(queue + 0x14), 1);
        assert_eq!(e.mem.u32(world + 0x8c), 1);
        assert_eq!(e.mem.u32(second), 12);
        assert_eq!(e.mem.u32(second + 4), 0xf1);
        assert_eq!(e.mem.u32(second + 0x10), 0xf4);
        assert_eq!(
            calls_to(&calls, ADD_REFERENCE),
            vec![vec![0xf1], vec![0xf2]]
        );
    }

    #[test]
    fn flush_pairs_sorts_calls_the_listeners_and_releases() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 1);
        e.mem.set_u32(world + 0x90, 7);
        // The comparison orders the pairs by their first object's address.
        e.register_double(COMPARE_PAIRS, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) < e.mem.u32(a[1])),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, LISTENER_DOUBLE);
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener, vtable);
        let log = Rc::clone(&calls);
        e.register_double(LISTENER_DOUBLE, move |e, a| {
            log.borrow_mut()
                .push((LISTENER_DOUBLE, vec![a[0], a[1], e.mem.u32(a[2])]));
            Ret::default()
        });
        let pairs = e.mem.alloc(3 * 16);
        for (i, first) in [30u32, 10, 20].iter().enumerate() {
            e.mem.set_u32(pairs + 16 * i as u32, *first);
            e.mem.set_u32(pairs + 16 * i as u32 + 4, listener);
        }
        e.mem.set_u32(queue + 0x1c, pairs);
        e.mem.set_u32(queue + 0x20, 3);
        e.mem.set_u32(queue + 0x24, 3);
        e.call(0x00cf_7ae0, &args![queue]);
        assert_eq!(e.mem.u32(world + 0x90), 0);
        let reserve = calls_to(&calls, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][1..], [queue + 0x1c, 0x10, 0x10]);
        let heard: Vec<u32> = calls_to(&calls, LISTENER_DOUBLE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(heard, vec![listener; 3]);
        assert_eq!(calls_to(&calls, LISTENER_DOUBLE)[0][1], 10);
        assert_eq!(calls_to(&calls, LISTENER_DOUBLE)[2][1], 30);
        let released: Vec<u32> = calls_to(&calls, REMOVE_REFERENCE_UNCHECKED)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(released, vec![10, listener, 20, listener, 30, listener]);
        assert_eq!(
            calls_to(&calls, FREE_ARRAY_DOUBLE),
            vec![vec![pairs, 3 * 16]]
        );
        assert_eq!(e.mem.u32(queue + 0x20), 0);
    }

    /// Fills `count` 16-byte (or `size`-byte) elements whose first word is the key.
    fn keyed(e: &mut Engine, keys: &[u32], size: u32) -> u32 {
        let base = e.mem.alloc(keys.len() as u32 * size);
        for (i, key) in keys.iter().enumerate() {
            let element = base + i as u32 * size;
            e.mem.set_u32(element, *key);
            for word in 1..size / 4 {
                e.mem.set_u32(element + 4 * word, key * 10 + word);
            }
        }
        base
    }

    fn keys_of(e: &Engine, base: u32, count: u32, size: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(base + i * size)).collect()
    }

    #[test]
    fn sort_16_orders_ascending_and_moves_whole_elements() {
        let (mut e, _) = engine();
        e.register_double(0x0900_0200, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) < e.mem.u32(a[1])),
            ..Ret::default()
        });
        let keys = [7, 3, 9, 1, 8, 2, 6, 4, 5, 0];
        let base = keyed(&mut e, &keys, 16);
        e.call(0x00cf_83c0, &args![base, 0u32, 9u32, 0x0900_0200u32]);
        assert_eq!(keys_of(&e, base, 10, 16), (0..10).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(base + 16 * 7 + 12), 73);
        // A single-element range and a sub-range are left alone outside it.
        let base = keyed(&mut e, &[5, 4, 3, 2, 1], 16);
        e.call(0x00cf_83c0, &args![base, 1i32, 3i32, 0x0900_0200u32]);
        assert_eq!(keys_of(&e, base, 5, 16), vec![5, 2, 3, 4, 1]);
    }

    #[test]
    fn sort_20_orders_ascending_through_the_hidden_result() {
        let (mut e, _) = engine();
        e.register_double(0x0900_0210, |e, a| {
            let less = e.mem.u32(a[1]) < e.mem.u32(a[2]);
            e.mem.set_u8(a[0], u8::from(less));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let keys = [4, 8, 1, 9, 2, 7, 3, 6, 0, 5];
        let base = keyed(&mut e, &keys, 20);
        e.call(0x00cf_8510, &args![base, 0u32, 9u32, 0x0900_0210u32]);
        assert_eq!(keys_of(&e, base, 10, 20), (0..10).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(base + 20 * 6 + 16), 64);
    }

    #[test]
    fn execute_runs_entity_operations_and_releases_their_objects() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 8);
        let outsider = object(&mut e, 0, 1, 1);
        let member = object(&mut e, world, 1, 1);
        enqueue(&mut e, queue, 1, [outsider, 5, 0, 0]);
        enqueue(&mut e, queue, 1, [member, 5, 0, 0]);
        enqueue(&mut e, queue, 2, [member, 0, 0, 0]);
        enqueue(&mut e, queue, 2, [outsider, 0, 0, 0]);
        enqueue(&mut e, queue, 3, [member, 0, 0, 0]);
        enqueue(&mut e, queue, 4, [member, 0x0302_0100, 0, 0]);
        calls.borrow_mut().clear();
        e.call(0x00cf_7c10, &args![queue]);
        assert_eq!(
            calls_to(&calls, 0x00c9_14d0),
            vec![vec![world, outsider, 5]]
        );
        let removed = calls_to(&calls, 0x00c9_1620);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (world, member));
        assert_eq!(
            calls_to(&calls, UPDATE_ENTITY_BP),
            vec![vec![world, member]]
        );
        assert_eq!(calls_to(&calls, 0x00ca_8420), vec![vec![member, 0, 1, 2]]);
        let released: Vec<u32> = calls_to(&calls, REMOVE_REFERENCE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(
            released,
            vec![outsider, member, member, outsider, member, member]
        );
        assert_eq!(e.mem.u32(queue + 4), 0);
        assert_eq!(e.mem.u32(world + 0xa4), 0);
        let order: Vec<u32> = calls.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order.first(), Some(&LOCK_ENTER));
        assert_eq!(order.last(), Some(&LOCK_LEAVE));
        assert_eq!(calls_to(&calls, FREE_ARRAY_DOUBLE).len(), 0);
    }

    #[test]
    fn execute_merges_islands_before_the_records() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 2);
        e.register_double(COMPARE_MERGES, |e, a| {
            e.mem.set_u8(a[0], 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let first = object(&mut e, world, 1, 1);
        let second = object(&mut e, world, 2, 1);
        let fixed = object(&mut e, world, 3, 5);
        let same_island = object(&mut e, world, 1, 1);
        let requests = e.mem.alloc(3 * 0x14);
        for (i, (a, b)) in [(first, same_island), (first, fixed), (first, second)]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(requests + 0x14 * i as u32 + 4, *a);
            e.mem.set_u32(requests + 0x14 * i as u32 + 8, *b);
        }
        e.mem.set_u32(queue + 0x10, requests);
        e.mem.set_u32(queue + 0x14, 3);
        e.mem.set_u32(queue + 0x18, 3);
        e.call(0x00cf_7c10, &args![queue]);
        // The comparison never says "before", so the sort only swaps the outer requests; the
        // requests run from the end: same island, fixed body, then the real merge.
        assert_eq!(
            calls_to(&calls, MERGE_ISLANDS),
            vec![vec![world, first, second]]
        );
        assert_eq!(e.mem.u32(queue + 0x14), 0);
        let released: Vec<u32> = calls_to(&calls, REMOVE_REFERENCE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(
            released,
            vec![first, same_island, first, fixed, first, second]
        );
        assert_eq!(calls_to(&calls, 0x00c9_14d0).len(), 0);
    }

    #[test]
    fn execute_constraints_actions_and_phantoms() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 12);
        let body_a = object(&mut e, world, 1, 1);
        let body_b = object(&mut e, world, 1, 1);
        let constraint = object(&mut e, 0, 0, 0);
        e.mem.set_u32(constraint + 0x14, body_a);
        e.mem.set_u32(constraint + 0x18, body_b);
        let installed = object(&mut e, world, 0, 0);
        e.mem.set_u32(installed + 0xc, 1);
        // An action with one entity in the world, and one with an outsider.
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, ACTION_DOUBLE);
        let action = |e: &mut Engine, entity: u32| {
            let action = e.mem.alloc(0x20);
            e.mem.set_u32(action, vtable);
            e.mem.set_u32(action + 8, 0);
            e.mem.set_u32(action + 0x1c, entity);
            action
        };
        e.register_double(ACTION_DOUBLE, |e, a| {
            // Fills the array at a[1] with the entity stored at the action's +0x1c.
            let entity = e.mem.u32(a[0] + 0x1c);
            let data = e.mem.alloc(4);
            e.mem.set_u32(data, entity);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 1);
            e.mem.set_u32(a[1] + 8, 1);
            Ret::default()
        });
        let outsider = object(&mut e, 0, 0, 0);
        let good_action = action(&mut e, body_a);
        let bad_action = action(&mut e, outsider);
        let phantom = object(&mut e, 0, 0, 0);
        enqueue(&mut e, queue, 8, [constraint, 0, 0, 0]);
        enqueue(&mut e, queue, 9, [installed, 0, 0, 0]);
        enqueue(&mut e, queue, 10, [good_action, 0, 0, 0]);
        enqueue(&mut e, queue, 10, [bad_action, 0, 0, 0]);
        enqueue(&mut e, queue, 11, [installed, 0, 0, 0]);
        enqueue(&mut e, queue, 13, [phantom, 0, 0, 0]);
        enqueue(&mut e, queue, 14, [body_a, 0, 0, 0]);
        calls.borrow_mut().clear();
        e.call(0x00cf_7c10, &args![queue]);
        assert_eq!(calls_to(&calls, 0x00c9_7d30), vec![vec![world, constraint]]);
        assert_eq!(calls_to(&calls, 0x00c9_7e80).len(), 1);
        assert_eq!(
            calls_to(&calls, 0x00c9_7370),
            vec![vec![world, good_action]]
        );
        assert_eq!(calls_to(&calls, 0x00c9_7f50), vec![vec![world, installed]]);
        assert_eq!(calls_to(&calls, 0x00c9_36a0), vec![vec![world, phantom]]);
        assert_eq!(calls_to(&calls, 0x00c9_2c70), vec![vec![world, body_a]]);
        assert_eq!(calls_to(&calls, FREE_ARRAY_DOUBLE).len(), 2);
    }

    #[test]
    fn execute_batches_free_their_blocks() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 8);
        let in_world = object(&mut e, world, 1, 1);
        let elsewhere = object(&mut e, 0x5555, 1, 1);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, in_world);
        e.mem.set_u32(source + 4, elsewhere);
        let source23 = e.mem.alloc(8);
        e.mem.set_u32(source23, elsewhere);
        e.mem.set_u32(source23 + 4, in_world);
        enqueue(&mut e, queue, 6, [source, 2, 0, 0]);
        enqueue(&mut e, queue, 15, [source, 1, 0, 0]);
        let batch = e.mem.u32(e.mem.u32(queue) + 4);
        let batch2 = e.mem.u32(e.mem.u32(queue) + 0x14 + 4);
        // Kind 23: the entry that is not in the world goes to the end and is not passed on.
        let manager = e.mem.alloc(0x40);
        let manager_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(manager, manager_vtable);
        e.mem.set_u32(manager_vtable + 0x30, MANAGER_DOUBLE);
        e.mem.set_u32(world + 8, manager);
        let log = Rc::clone(&calls);
        e.register_double(MANAGER_DOUBLE, move |e, a| {
            log.borrow_mut()
                .push((MANAGER_DOUBLE, vec![e.mem.u32(a[1]), a[2], a[3], a[4]]));
            Ret::default()
        });
        enqueue(&mut e, queue, 23, [source23, 2, 0, 0]);
        let moved = e.mem.u32(e.mem.u32(queue) + 0x28 + 4);
        calls.borrow_mut().clear();
        e.call(0x00cf_7c10, &args![queue]);
        assert_eq!(
            calls_to(&calls, 0x00c9_4bd0),
            vec![vec![world, batch, 2, 0]]
        );
        assert_eq!(calls_to(&calls, 0x00c9_54e0), vec![vec![world, batch2, 1]]);
        assert_eq!(
            calls_to(&calls, RELEASE_BATCH),
            vec![vec![batch, 2, 4], vec![batch2, 1, 4]]
        );
        assert_eq!(calls_to(&calls, FREE_DOUBLE)[0], vec![batch, 8]);
        assert_eq!(
            calls_to(&calls, MANAGER_DOUBLE),
            vec![vec![in_world, 1, world, 0]]
        );
        assert_eq!(e.mem.u32(moved), in_world);
        assert_eq!(e.mem.u32(moved + 4), elsewhere);
        let released: Vec<u32> = calls_to(&calls, REMOVE_REFERENCE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(released, vec![in_world, elsewhere]);
    }

    #[test]
    fn execute_motion_operations_pass_vectors_and_free_their_blocks() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 8);
        e.mem.set_u32(VECTOR_FOURTH_LANE, 0x3f00_0000);
        let body = object(&mut e, world, 1, 1);
        let motion_vtable = e.mem.alloc(0x60);
        e.mem.set_u32(motion_vtable + 0x4c, MOTION_DOUBLE);
        e.mem.set_u32(motion_vtable + 0x50, MOTION_DOUBLE + 4);
        e.mem.set_u32(motion_vtable + 0x54, MOTION_DOUBLE + 8);
        e.mem.set_u32(body + 0xe0, motion_vtable);
        // The vector consumers read their four lanes while they run, then the block is freed.
        for address in [
            MOTION_DOUBLE,
            MOTION_DOUBLE + 4,
            MOTION_DOUBLE + 8,
            0x0056_16d0,
            0x0056_1800,
        ] {
            let log = Rc::clone(&calls);
            e.register_double(address, move |e, a| {
                let mut words = vec![a[0]];
                words.extend((0..4).map(|i| e.mem.u32(a[1] + 4 * i)));
                log.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        let pose = e.mem.alloc(0x20);
        let pair = e.mem.alloc(0x20);
        // Linear velocity, angular velocity, activate plus impulse (slot 0x4c), activate plus
        // point impulse (slot 0x54), activate plus a pair (slot 0x50), pose, region.
        enqueue(&mut e, queue, 25, [body, 0x11, 0x22, 0x33]);
        enqueue(&mut e, queue, 26, [body, 0x44, 0x55, 0x66]);
        enqueue(&mut e, queue, 27, [body, 0x77, 0x88, 0x99]);
        enqueue(&mut e, queue, 29, [body, 0xaa, 0xbb, 0xcc]);
        enqueue(&mut e, queue, 28, [body, pair, 0, 0]);
        enqueue(&mut e, queue, 24, [body, pose, 0, 0]);
        enqueue(&mut e, queue, 32, [pose, 0, 0, 0]);
        calls.borrow_mut().clear();
        e.call(0x00cf_7c10, &args![queue]);
        assert_eq!(
            calls_to(&calls, 0x0056_16d0),
            vec![vec![body, 0x11, 0x22, 0x33, 0x3f00_0000]]
        );
        assert_eq!(
            calls_to(&calls, 0x0056_1800),
            vec![vec![body, 0x44, 0x55, 0x66, 0x3f00_0000]]
        );
        assert_eq!(calls_to(&calls, 0x00c9_c1d0), vec![vec![body]; 3]);
        assert_eq!(
            calls_to(&calls, MOTION_DOUBLE),
            vec![vec![body + 0xe0, 0x77, 0x88, 0x99, 0x3f00_0000]]
        );
        assert_eq!(
            calls_to(&calls, MOTION_DOUBLE + 8),
            vec![vec![body + 0xe0, 0xaa, 0xbb, 0xcc, 0x3f00_0000]]
        );
        let paired = calls_to(&calls, MOTION_DOUBLE + 4);
        assert_eq!(paired.len(), 1);
        assert_eq!(paired[0][0], body + 0xe0);
        assert_eq!(
            calls_to(&calls, 0x00ca_8860),
            vec![vec![body, pose, pose + 0x10]]
        );
        assert_eq!(calls_to(&calls, 0x00c9_3c30), vec![vec![world, pose]]);
        // Kinds 24, 28 and 32 free their 0x20-byte blocks.
        let freed: Vec<u32> = calls_to(&calls, FREE_DOUBLE)
            .iter()
            .filter(|c| c[1] == 0x20)
            .map(|c| c[0])
            .collect();
        assert_eq!(freed, vec![pair, pose, pose]);
    }

    #[test]
    fn execute_object_calls_collision_updates_and_activation() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 12);
        let body = object(&mut e, world, 1, 1);
        let other = object(&mut e, world, 1, 1);
        let outsider = object(&mut e, 0, 1, 1);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, OBJECT_DOUBLE);
        e.mem.set_u32(body, vtable);
        let log = Rc::clone(&calls);
        e.register_double(OBJECT_DOUBLE, move |_, a| {
            log.borrow_mut().push((OBJECT_DOUBLE, a.to_vec()));
            Ret::default()
        });
        let block = e.mem.alloc(0x20);
        enqueue(&mut e, queue, 5, [body, 0x99, 0, 0]);
        enqueue(&mut e, queue, 36, [body, 0x98, 0, 0]);
        enqueue(&mut e, queue, 17, [body, block, 0, 0]);
        enqueue(&mut e, queue, 18, [body, 0x0201, 0, 0]);
        enqueue(&mut e, queue, 19, [body, other, 0, 0]);
        enqueue(&mut e, queue, 19, [body, outsider, 0, 0]);
        enqueue(&mut e, queue, 20, [body, 0x07, 0, 0]);
        let filter_op = record(&mut e, 21, [0, 0, 0, 0]);
        e.mem.set_u8(filter_op + 1, 2);
        e.mem.set_u8(filter_op + 2, 3);
        e.call(0x00cf_7690, &args![queue, filter_op]);
        enqueue(&mut e, queue, 22, [body, 0, 0, 0]);
        enqueue(&mut e, queue, 33, [body, 0, 0, 0]);
        enqueue(&mut e, queue, 34, [body, 0, 0, 0]);
        enqueue(&mut e, queue, 35, [body, other, 0x0900_0000, 0]);
        calls.borrow_mut().clear();
        e.call(0x00cf_7c10, &args![queue]);
        assert_eq!(
            calls_to(&calls, OBJECT_DOUBLE),
            vec![vec![body, 0x99], vec![body, 0x98]]
        );
        let updated = calls_to(&calls, 0x00d0_7ec0);
        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0][0], body);
        // The record holds a 0x20-byte copy of the block, which is what gets freed.
        let copy = updated[0][1];
        assert_ne!(copy, block);
        assert_eq!(calls_to(&calls, 0x00c9_46e0), vec![vec![world, body, 1, 2]]);
        assert_eq!(
            calls_to(&calls, 0x00c9_13b0),
            vec![vec![world, body, other]]
        );
        assert_eq!(calls_to(&calls, 0x00c9_44a0), vec![vec![world, body, 7]]);
        assert_eq!(calls_to(&calls, 0x00c9_6ff0), vec![vec![world, 2, 3]]);
        assert_eq!(calls_to(&calls, UPDATE_WARPED_BODY), vec![vec![body]]);
        assert_eq!(calls_to(&calls, 0x00c9_c1d0), vec![vec![body]]);
        assert_eq!(calls_to(&calls, 0x00c9_c240), vec![vec![body]]);
        assert_eq!(calls_to(&calls, 0x00d8_7c20), vec![vec![body, other, 0]]);
        assert_eq!(calls_to(&calls, FREE_DOUBLE), vec![vec![copy, 0x20]]);
    }

    #[test]
    fn execute_runs_records_queued_while_executing() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 2);
        let body = object(&mut e, world, 1, 1);
        // The addEntity double queues a kind 33 record (activate) while the queue runs.
        let nested = record(&mut e, 33, [body, 0, 0, 0]);
        e.register_double(0x00c9_14d0, move |e, _| {
            let data = e.mem.alloc(0x14 * 2);
            e.mem.set_u32(queue, data);
            e.mem.set_u32(queue + 8, 2);
            e.mem.set_u32(queue + 4, 1);
            let bytes = e.mem.bytes(nested, 0x14);
            e.mem.write(data, &bytes);
            Ret::default()
        });
        let outsider = object(&mut e, 0, 1, 1);
        enqueue(&mut e, queue, 1, [outsider, 0, 0, 0]);
        calls.borrow_mut().clear();
        e.call(0x00cf_7c10, &args![queue]);
        assert_eq!(calls_to(&calls, 0x00c9_c1d0), vec![vec![body]]);
        assert_eq!(e.mem.u32(queue + 4), 0);
        assert_eq!(e.mem.u32(world + 0xa4), 0);
    }

    #[test]
    fn execute_unknown_kinds_do_nothing() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 2);
        enqueue(&mut e, queue, 99, [1, 2, 3, 4]);
        calls.borrow_mut().clear();
        e.call(0x00cf_7c10, &args![queue]);
        assert!(calls_to(&calls, REMOVE_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(queue + 4), 0);
        assert_eq!(e.mem.u32(world + 0xa4), 0);
    }

    #[test]
    fn the_sorts_keep_their_pivot_and_answer_in_aligned_frames() {
        let (mut e, _) = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(0x0900_0200, move |e, a| {
            sink.borrow_mut().push(a[1]);
            u32::from(e.mem.u32(a[0]) < e.mem.u32(a[1])).into_ret()
        });
        let base = keyed(&mut e, &[3, 1, 2], 16);
        let raw = e.next_stack_local(0x40);
        e.call(0x00cf_83c0, &args![base, 0u32, 2u32, 0x0900_0200u32]);
        assert_eq!(seen.borrow()[0], aligned_frame_at(raw, 0x30) + 0x10);
        let hidden = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&hidden);
        e.register_double(0x0900_0210, move |e, a| {
            sink.borrow_mut().push((a[0], a[2]));
            let less = e.mem.u32(a[1]) < e.mem.u32(a[2]);
            e.mem.set_u8(a[0], u8::from(less));
            a[0].into_ret()
        });
        let base = keyed(&mut e, &[3, 1, 2], 20);
        let raw = e.next_stack_local(0x70);
        e.call(0x00cf_8510, &args![base, 0u32, 2u32, 0x0900_0210u32]);
        let frame = aligned_frame_at(raw, 0x60);
        assert_eq!(hidden.borrow()[0], (frame + 0x1a, frame + 0x40));
    }

    #[test]
    fn flushing_pairs_keeps_the_array_and_answer_byte_in_the_exe_frame() {
        let (mut e, calls) = engine();
        let (queue, _world) = queue(&mut e, 1);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, LISTENER_DOUBLE);
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener, vtable);
        let log = Rc::clone(&calls);
        e.register_double(LISTENER_DOUBLE, move |_, a| {
            log.borrow_mut().push((LISTENER_DOUBLE, a.to_vec()));
            Ret::default()
        });
        let pairs = e.mem.alloc(16);
        e.mem.set_u32(pairs, 10);
        e.mem.set_u32(pairs + 4, listener);
        e.mem.set_u32(queue + 0x1c, pairs);
        e.mem.set_u32(queue + 0x20, 1);
        e.mem.set_u32(queue + 0x24, 1);
        let frame = e.next_stack_local(0x24);
        e.call(0x00cf_7ae0, &args![queue]);
        assert_eq!(calls_to(&calls, LISTENER_DOUBLE)[0][2], frame + 0x13);
        // The pairs were swapped into the array at +0x18.
        assert_eq!(e.mem.u32(frame + 0x18), pairs);
    }

    #[test]
    fn executing_keeps_the_answer_bytes_and_vectors_in_the_aligned_frame() {
        let (mut e, calls) = engine();
        let (queue, world) = queue(&mut e, 8);
        let member = object(&mut e, world, 1, 1);
        let body = object(&mut e, world, 1, 1);
        let motion_vtable = e.mem.alloc(0x60);
        for slot in [0x4cu32, 0x50, 0x54] {
            e.mem.set_u32(motion_vtable + slot, MOTION_DOUBLE);
        }
        e.mem.set_u32(body + 0xe0, motion_vtable);
        for address in [MOTION_DOUBLE, 0x0056_16d0, 0x0056_1800] {
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        enqueue(&mut e, queue, 2, [member, 0, 0, 0]);
        enqueue(&mut e, queue, 25, [body, 1, 2, 3]);
        enqueue(&mut e, queue, 26, [body, 1, 2, 3]);
        enqueue(&mut e, queue, 27, [body, 1, 2, 3]);
        enqueue(&mut e, queue, 29, [body, 1, 2, 3]);
        calls.borrow_mut().clear();
        let raw = e.next_stack_local(0xa0);
        e.call(0x00cf_7c10, &args![queue]);
        let frame = aligned_frame_at(raw, 0x90);
        assert_eq!(calls_to(&calls, 0x00c9_1620)[0][1], frame + 0x1a);
        assert_eq!(calls_to(&calls, 0x0056_16d0)[0][1], frame + 0x40);
        assert_eq!(calls_to(&calls, 0x0056_1800)[0][1], frame + 0x50);
        let motions: Vec<u32> = calls_to(&calls, MOTION_DOUBLE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(motions, vec![frame + 0x60, frame + 0x70]);
    }
}

#[cfg(test)]
mod tests_00cf7540 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_ARRAY_DOUBLE: u32 = 0x0900_0201;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// A router whose allocator frees array data through a double; doubles for the lock, the
    /// reference and the growth functions. Returns the engine, the log and the allocator.
    fn rig() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_DOUBLE);
        let alloc = e.mem.alloc(0x10);
        e.mem.set_u32(alloc, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for (address, name) in [
            (LOCK_ENTER, "lock"),
            (LOCK_LEAVE, "unlock"),
            (ADD_REFERENCE, "reference"),
            (ARRAY_RESERVE_MORE, "grow"),
            (FREE_ARRAY_DOUBLE, "free"),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((name, a.to_vec()));
                if name == "grow" {
                    // The growth makes room for four 16-byte records.
                    let new = e.mem.alloc(0x40);
                    let size = e.mem.u32(a[1] + 4);
                    for i in 0..size * 4 {
                        let word = e.mem.u32(e.mem.u32(a[1]) + 4 * i);
                        e.mem.set_u32(new + 4 * i, word);
                    }
                    e.mem.set_u32(a[1], new);
                    e.mem.set_u32(a[1] + 8, 4);
                }
                Ret::default()
            });
        }
        (e, log, alloc)
    }

    /// The record is appended (growing when the array is full), the counter rises and both
    /// objects get a reference, inside the lock.
    #[test]
    fn a_pair_is_queued_under_the_lock() {
        let (mut e, log, _) = rig();
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x90, 5);
        let queue = e.mem.alloc(0x40);
        e.call(0x00cf_75c0, &args![queue, world]);
        e.call(
            0x00cf_7540,
            &args![queue, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        let calls = log.borrow().clone();
        let names: Vec<&str> = calls.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            names,
            vec!["lock", "grow", "reference", "reference", "unlock"]
        );
        assert_eq!(calls[2].1, vec![0x11]);
        assert_eq!(calls[3].1, vec![0x22]);
        assert_eq!(e.mem.u32(world + 0x90), 6);
        let array = queue + 0x1c;
        assert_eq!(e.mem.u32(array + 4), 1);
        let data = e.mem.u32(array);
        let record: Vec<u32> = (0..4).map(|i| e.mem.u32(data + 4 * i)).collect();
        assert_eq!(record, vec![0x11, 0x22, 0x33, 0x44]);
        // Room is left: the next record needs no growth.
        log.borrow_mut().clear();
        e.call(0x00cf_7540, &args![queue, 1u32, 2u32, 3u32, 4u32]);
        let names: Vec<&str> = log.borrow().iter().map(|(name, _)| *name).collect();
        assert_eq!(names, vec!["lock", "reference", "reference", "unlock"]);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(e.mem.u32(data + 16), 1);
    }

    /// The constructor writes three empty arrays and the world; the destructor frees owned
    /// arrays with their element sizes (16, 20 and 20 bytes) and empties all three.
    #[test]
    fn constructor_and_destructor_of_the_queue() {
        let (mut e, log, alloc) = rig();
        let queue = e.mem.alloc(0x40);
        e.mem.write(queue, &[0xaa; 0x40]);
        assert_eq!(e.call(0x00cf_75c0, &args![queue, 0x777u32]).u32(), queue);
        for array in [0u32, 0x10, 0x1c] {
            assert_eq!(e.mem.u32(queue + array), 0);
            assert_eq!(e.mem.u32(queue + array + 4), 0);
            assert_eq!(e.mem.u32(queue + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(queue + 0xc), 0x777);
        assert_eq!(e.mem.u32(queue + 0x28), 0xaaaa_aaaa);
        for (array, data, capacity) in [
            (0x1cu32, 0x5000u32, 2u32),
            (0x10, 0x6000, 3),
            (0, 0x7000, 0x8000_0004),
        ] {
            e.mem.set_u32(queue + array, data);
            e.mem.set_u32(queue + array + 4, 1);
            e.mem.set_u32(queue + array + 8, capacity);
        }
        log.borrow_mut().clear();
        e.call(0x00cf_75f0, &args![queue]);
        assert_eq!(
            *log.borrow(),
            vec![
                ("free", vec![alloc, 0x5000, 0x20]),
                ("free", vec![alloc, 0x6000, 0x3c])
            ]
        );
        for array in [0u32, 0x10, 0x1c] {
            assert_eq!(e.mem.u32(queue + array), 0);
            assert_eq!(e.mem.u32(queue + array + 4), 0);
            assert_eq!(e.mem.u32(queue + array + 8), 0x8000_0000);
        }
    }

    /// Eight words are copied in order; the vtable word and the float helpers.
    #[test]
    fn copies_and_float_helpers() {
        let mut e = Engine::new();
        let from = e.mem.alloc(0x20);
        let to = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(from + 4 * i, 0x100 + i);
        }
        e.call(0x00cf_8690, &args![to, from]);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(to + 4 * i)).collect();
        assert_eq!(words, (0..8).map(|i| 0x100 + i).collect::<Vec<_>>());
        e.call(0x00cf_86d0, &args![to]);
        assert_eq!(e.mem.u32(to), 0x0102_e388);
        let object = e.mem.alloc(0x40);
        e.mem.set_f32(object + 0x14, 1.5);
        e.call(0x00cf_86e0, &args![object, 2.25f32]);
        assert_eq!(e.mem.f32(object + 0x20), 3.75);
        assert!(!e.call(0x00cf_86f0, &args![object]).bool());
        e.mem.set_f32(object + 0x20, 1.5);
        assert!(e.call(0x00cf_86f0, &args![object]).bool());
        e.mem.set_u32(object + 0x20, 0x7fc0_0000);
        e.mem.set_u32(object + 0x14, 0x7fc0_0000);
        assert!(!e.call(0x00cf_86f0, &args![object]).bool());
        // A signalling NaN float at +0x14 is quieted by the load.
        e.mem.set_u32(object + 0x14, 0x7f80_0001);
        e.call(0x00cf_86e0, &args![object, 1.0f32]);
        assert_eq!(e.mem.u32(object + 0x20) & 0xffc0_0000, 0x7fc0_0000);
    }
}
